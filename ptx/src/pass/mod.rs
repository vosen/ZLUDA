use ptx_parser as ast;
use quick_error::quick_error;
use rustc_hash::FxHashMap;
use std::hash::Hash;
use std::{
    borrow::Cow,
    collections::{hash_map, HashMap},
    ffi::CString,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

mod agx_matrix;
mod convert_32bit_to_64bit;
mod deparamize_functions;
mod expand_operands;
mod fix_special_registers;
mod hoist_globals;
mod insert_explicit_load_store;
mod insert_implicit_conversions;
mod insert_post_saturation;
mod instruction_mode_to_global_mode;
pub mod llvm;
mod normalize_basic_blocks;
mod normalize_identifiers;
mod normalize_predicates;
mod optimize_function_arguments;
mod rcp_f64_into_div;
mod remove_unreachable_basic_blocks;
mod replace_instructions_with_functions;
mod replace_instructions_with_functions_fp_required;
mod replace_known_functions;
mod resolve_function_pointers;

#[cfg(test)]
mod test;

static ZLUDA_PTX_IMPL: &'static [u8] = include_bytes!("../../lib/zluda_ptx_impl.bc");
static ZLUDA_PTX_IMPL_CONSTRAINED: &'static [u8] =
    include_bytes!("../../lib/zluda_ptx_impl_constrained.bc");
// Textual LLVM IR, see the comment at its top.
static ZLUDA_PTX_IMPL_AGX: &'static [u8] = include_bytes!("../../lib/zluda_ptx_impl_agx.ll");
const ZLUDA_PTX_PREFIX: &'static str = "__zluda_ptx_impl_";

quick_error! {
    #[derive(Debug, strum_macros::AsRefStr)]
    pub enum TranslateError {
        UnknownSymbol(symbol: String) {
            display("Unknown symbol: \"{}\"", symbol)
        }
        UntypedSymbol {}
        MismatchedType {}
        Unreachable {}
        Todo(msg: String) {
            display("TODO: {}", msg)
        }
        /// A feature the target hardware does not have.
        Unsupported(msg: String) {
            display("{}", msg)
        }
    }
}

/// The GPU architecture that code is generated for.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum Target {
    /// AMD GPUs, with the AMDGPU LLVM backend.
    #[default]
    Amdgpu,
    /// Apple GPUs, with the experimental AGX LLVM backend. Kernels use the
    /// agx_kernel calling convention and the helper functions are linked
    /// from lib/zluda_ptx_impl_agx.bc.
    Agx,
}

/// GPU attributes needed at compile time.
#[derive(Copy, Clone)]
pub struct Attributes {
    /// Clock frequency in kHz.
    pub clock_rate: u32,
}

pub fn to_llvm_module<'input>(
    ast: ast::Module<'input>,
    attributes: Attributes,
    on_pass_end: impl FnMut(&str),
) -> Result<Module, TranslateError> {
    to_llvm_module_for_target(ast, attributes, Target::Amdgpu, on_pass_end)
}

/// Options of the translation.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Options {
    pub target: Target,
    /// Follow the floating point semantics of PTX exactly where the AGX
    /// hardware differs: the generated code implements IEEE 754 binary32
    /// arithmetic (denormals, correctly rounded division and square root) and
    /// canonical NaNs in software, and `.sat` keeps `-0.0`. Slower; ignored
    /// for AMDGPU.
    pub strict_fp: bool,
    /// Keep `.loc` as LLVM debug locations (line tables only), for the AGX
    /// backend's checked mode to name the source of a bad access.
    pub debug_locations: bool,
    /// AGX: bytes of stack per thread for `alloca` of a dynamic size (CUDA's
    /// `CU_LIMIT_STACK_SIZE`; 0 for its default, 1024); the frames of recursive
    /// functions get four times as much.
    pub stack_size: u32,
}

/// Drops the `.loc` statements of every function.
fn drop_locs<P: ast::Operand>(statements: &mut Vec<ast::Statement<P>>) {
    statements.retain(|s| !matches!(s, ast::Statement::Loc(_)));
    for s in statements {
        if let ast::Statement::Block(b) = s {
            drop_locs(b);
        }
    }
}

pub fn to_llvm_module_for_target<'input>(
    ast: ast::Module<'input>,
    attributes: Attributes,
    target: Target,
    on_pass_end: impl FnMut(&str),
) -> Result<Module, TranslateError> {
    to_llvm_module_with_options(
        ast,
        attributes,
        Options {
            target,
            ..Options::default()
        },
        on_pass_end,
    )
}

pub fn to_llvm_module_with_options<'input>(
    ast: ast::Module<'input>,
    attributes: Attributes,
    options: Options,
    mut on_pass_end: impl FnMut(&str),
) -> Result<Module, TranslateError> {
    let Options { target, strict_fp, debug_locations, stack_size } = options;
    let strict_fp = strict_fp && target == Target::Agx;
    let mut ast = ast;
    if !debug_locations {
        for d in ast.directives.iter_mut() {
            if let ast::Directive::Method(_, ast::Function { body: Some(body), .. }) = d {
                drop_locs(body);
            }
        }
    }
    let files = if debug_locations { ast.files.iter().map(|&(i, f)| (i, f.to_string())).collect() } else { Vec::new() };
    let sm_version = ast.sm_version;
    let mut flat_resolver = GlobalStringIdentResolver2::<'input>::new(SpirvWord(1));
    let mut scoped_resolver = ScopedResolver::new(&mut flat_resolver);
    let sreg_map = SpecialRegistersMap::new(&mut scoped_resolver)?;
    let directives = normalize_identifiers::run(&mut scoped_resolver, merge_symbols(ast.directives))?;
    on_pass_end("normalize_identifiers");
    let directives = replace_known_functions::run(&mut flat_resolver, directives);
    on_pass_end("replace_known_functions");
    let directives = normalize_predicates::run(&mut flat_resolver, directives)?;
    on_pass_end("normalize_predicates");
    let directives = optimize_function_arguments::run(&mut flat_resolver, directives)?;
    on_pass_end("optimize_function_arguments");
    let directives = resolve_function_pointers::run(directives)?;
    on_pass_end("resolve_function_pointers");
    let directives = fix_special_registers::run(&mut flat_resolver, &sreg_map, target, directives)?;
    on_pass_end("fix_special_registers");
    let directives = expand_operands::run(&mut flat_resolver, directives)?;
    on_pass_end("expand_operands");
    let directives = insert_post_saturation::run(&mut flat_resolver, directives)?;
    on_pass_end("insert_post_saturation");
    let directives = append_implicit_ret(directives);
    let directives = deparamize_functions::run(&mut flat_resolver, directives)?;
    on_pass_end("deparamize_functions");
    let directives = rcp_f64_into_div::run(&mut flat_resolver, directives)?;
    on_pass_end("rcp_f64_into_div");
    let directives =
        replace_instructions_with_functions_fp_required::run(&mut flat_resolver, target, directives)?;
    on_pass_end("replace_instructions_with_functions_fp_required");
    let directives = normalize_basic_blocks::run(&mut flat_resolver, directives)?;
    on_pass_end("normalize_basic_blocks");
    let directives = remove_unreachable_basic_blocks::run(directives, target == Target::Agx)?;
    on_pass_end("remove_unreachable_basic_blocks");
    let directives = instruction_mode_to_global_mode::run(&mut flat_resolver, directives)?;
    on_pass_end("instruction_mode_to_global_mode");
    let mut directives = insert_explicit_load_store::run(&mut flat_resolver, directives)?;
    on_pass_end("insert_explicit_load_store");
    let mut metadata32 = None;
    if ast.address_size == 32 {
        let (new_directives, new_module32) =
            convert_32bit_to_64bit::run(&mut flat_resolver, directives)?;
        directives = new_directives;
        metadata32 = Some(new_module32);
        on_pass_end("convert_32bit_to_64bit");
    }
    let directives =
        insert_implicit_conversions::run(ast.address_size == 32, &mut flat_resolver, directives)?;
    on_pass_end("insert_implicit_conversions");
    let directives =
        replace_instructions_with_functions::run(&mut flat_resolver, target, directives)?;
    on_pass_end("replace_instructions_with_functions");
    let directives = hoist_globals::run(directives)?;
    on_pass_end("hoist_globals");
    let fp_mode = get_fp_mode(target, &directives[..]);
    on_pass_end("get_fp_mode");
    let context = llvm_zluda::utils::Context::new();
    let llvm_ir =
        llvm::emit::run(&context, flat_resolver, directives, fp_mode, target, strict_fp, stack_size, &files)?;
    let attributes_ir = llvm::attributes::run(&context, attributes)?;
    on_pass_end("emit_llvm");
    Ok(Module {
        llvm_ir,
        attributes_ir,
        context,
        metadata: kernel_metadata::ModuleMetadataV1::new(sm_version),
        metadata32,
        constrained_fp: fp_mode == llvm::emit::FloatingPointMode::Constrained,
        target,
        strict_fp,
    })
}

/// Keeps one module-level definition per name, like a PTX linker: the first
/// definition that is not `.weak`, or else the first `.weak` one. Extern
/// declarations of a defined variable and the other definitions are dropped;
/// the kept variable takes the place of the first declaration, so that it
/// still precedes its uses. Function declarations without a body are kept.
fn merge_symbols<'input>(
    directives: Vec<ast::Directive<'input, ast::ParsedOperand<&'input str>>>,
) -> Vec<ast::Directive<'input, ast::ParsedOperand<&'input str>>> {
    fn name<'a>(d: &ast::Directive<'a, ast::ParsedOperand<&'a str>>) -> &'a str {
        match d {
            ast::Directive::Variable(_, v) => v.name,
            ast::Directive::Method(_, f) => f.func_directive.name(),
        }
    }
    fn is_definition(d: &ast::Directive<'_, ast::ParsedOperand<&str>>) -> bool {
        match d {
            ast::Directive::Variable(linking, _) => !linking.contains(ast::LinkingDirective::EXTERN),
            ast::Directive::Method(_, f) => f.body.is_some(),
        }
    }
    fn is_weak(d: &ast::Directive<'_, ast::ParsedOperand<&str>>) -> bool {
        match d {
            ast::Directive::Variable(linking, _) | ast::Directive::Method(linking, _) => {
                linking.contains(ast::LinkingDirective::WEAK)
            }
        }
    }
    let mut winner = FxHashMap::<&str, usize>::default();
    let mut first = FxHashMap::<&str, usize>::default();
    for (i, d) in directives.iter().enumerate() {
        let n = name(d);
        first.entry(n).or_insert(i);
        if !is_definition(d) {
            continue;
        }
        match winner.get(n) {
            Some(&w) if !(is_weak(&directives[w]) && !is_weak(d)) => {}
            _ => {
                winner.insert(n, i);
            }
        }
    }
    let duplicated = |n: &str| directives.iter().filter(|d| name(d) == n).count() > 1;
    let duplicated = winner
        .keys()
        .copied()
        .filter(|n| duplicated(n))
        .collect::<Vec<_>>();
    if duplicated.is_empty() {
        return directives;
    }
    let mut slots = directives.into_iter().map(Some).collect::<Vec<_>>();
    for n in duplicated {
        let w = winner[n];
        let is_variable = matches!(slots[w], Some(ast::Directive::Variable(..)));
        for i in 0..slots.len() {
            let drop = match &slots[i] {
                Some(d) if i != w && name(d) == n => is_variable || is_definition(d),
                _ => false,
            };
            if drop {
                slots[i] = None;
            }
        }
        let f = first[n];
        if is_variable && f != w {
            slots.swap(f, w);
        }
    }
    slots.into_iter().flatten().collect()
}

fn get_fp_mode(
    target: Target,
    directives: &[Directive2<ast::Instruction<SpirvWord>, SpirvWord>],
) -> llvm::emit::FloatingPointMode {
    for d in directives {
        if let Directive2::Method(method) = d {
            // AMDGPU applies a kernel's initial rounding mode to every
            // instruction through the hardware mode register. On AGX only
            // constrained instructions read the emulated mode register.
            if target == Target::Agx {
                if let Some(attrs) = &method.kernel_attributes {
                    if attrs.rounding_mode_f32 != ast::RoundingMode::NearestEven
                        || attrs.rounding_mode_f16f64 != ast::RoundingMode::NearestEven
                    {
                        return llvm::emit::FloatingPointMode::Constrained;
                    }
                }
            }
            for s in method.body.iter().flatten() {
                if let Statement::SetMode(_) = s {
                    return llvm::emit::FloatingPointMode::Constrained;
                }
            }
        }
    }
    llvm::emit::FloatingPointMode::Normal
}

pub struct Module {
    pub llvm_ir: llvm_zluda::utils::Module,
    pub attributes_ir: llvm_zluda::utils::Module,
    pub context: llvm_zluda::utils::Context,
    pub metadata: kernel_metadata::ModuleMetadataV1,
    pub metadata32: Option<kernel_metadata::ModuleMetadata32Bit>,
    pub constrained_fp: bool,
    pub target: Target,
    /// The code must be compiled with the strict floating point features of
    /// the AGX backend (see `Options::strict_fp`).
    pub strict_fp: bool,
}

impl Module {
    /// The module of helper functions to link with: bitcode for AMDGPU,
    /// textual IR for AGX.
    pub fn linked_bitcode(&self) -> &'static [u8] {
        if self.target == Target::Agx {
            ZLUDA_PTX_IMPL_AGX
        } else if self.constrained_fp {
            ZLUDA_PTX_IMPL_CONSTRAINED
        } else {
            ZLUDA_PTX_IMPL
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, EnumIter)]
enum PtxSpecialRegister {
    Tid,
    Ntid,
    Ctaid,
    Nctaid,
    Clock,
    Clock64,
    Globaltimer,
    LanemaskLe,
    LanemaskLt,
    LanemaskGe,
    Laneid,
    Warpid,
    Nwarpid,
    Smid,
    Nsmid,
    Warpsize,
    Clusterid,
    Nclusterid,
    ClusterCtaid,
    ClusterNctaid,
    ClusterCtarank,
    ClusterNctarank,
    IsExplicitCluster,
    #[strum(disabled)]
    Envreg(u8),
    #[strum(disabled)]
    Pm(u8),
}

impl PtxSpecialRegister {
    fn all() -> impl Iterator<Item = Self> {
        Self::iter()
            .chain((0..32).map(Self::Envreg))
            .chain((0..8).map(Self::Pm))
    }

    fn name(self) -> Cow<'static, str> {
        Cow::Borrowed(match self {
            Self::Tid => "%tid",
            Self::Ntid => "%ntid",
            Self::Ctaid => "%ctaid",
            Self::Nctaid => "%nctaid",
            Self::Clock => "%clock",
            Self::Clock64 => "%clock64",
            Self::Globaltimer => "%globaltimer",
            Self::LanemaskLe => "%lanemask_le",
            Self::LanemaskLt => "%lanemask_lt",
            Self::LanemaskGe => "%lanemask_ge",
            Self::Laneid => "%laneid",
            Self::Warpid => "%warpid",
            Self::Nwarpid => "%nwarpid",
            Self::Smid => "%smid",
            Self::Nsmid => "%nsmid",
            Self::Warpsize => "%warpsize",
            Self::Clusterid => "%clusterid",
            Self::Nclusterid => "%nclusterid",
            Self::ClusterCtaid => "%cluster_ctaid",
            Self::ClusterNctaid => "%cluster_nctaid",
            Self::ClusterCtarank => "%cluster_ctarank",
            Self::ClusterNctarank => "%cluster_nctarank",
            Self::IsExplicitCluster => "%is_explicit_cluster",
            Self::Envreg(n) => return Cow::Owned(format!("%envreg{}", n)),
            Self::Pm(n) => return Cow::Owned(format!("%pm{}", n)),
        })
    }

    fn is_vector(self) -> bool {
        matches!(
            self,
            Self::Tid
                | Self::Ntid
                | Self::Ctaid
                | Self::Nctaid
                | Self::Clusterid
                | Self::Nclusterid
                | Self::ClusterCtaid
                | Self::ClusterNctaid
        )
    }

    fn get_type(self) -> ast::Type {
        if self.is_vector() {
            ast::Type::Vector(4, self.get_function_return_type())
        } else {
            ast::Type::Scalar(self.get_function_return_type())
        }
    }

    fn is_cluster(self) -> bool {
        matches!(
            self,
            Self::Clusterid
                | Self::Nclusterid
                | Self::ClusterCtaid
                | Self::ClusterNctaid
                | Self::ClusterCtarank
                | Self::ClusterNctarank
                | Self::IsExplicitCluster
        )
    }

    /// The register that this one reads, if it is the same. On AMD GPUs a
    /// cluster is a single CTA; AGX runs a cluster as one threadgroup and
    /// reads the cluster registers with functions.
    fn canonical(self, target: Target) -> Self {
        match self {
            Self::Clusterid if target != Target::Agx => Self::Ctaid,
            Self::Nclusterid if target != Target::Agx => Self::Nctaid,
            other => other,
        }
    }

    /// The value of registers that are constant (outside of a cluster
    /// launch on AMD GPUs), for the vector member if any.
    fn constant(self, member: Option<u8>, target: Target) -> Option<u64> {
        if target == Target::Agx && self.is_cluster() {
            return None;
        }
        match self {
            Self::Warpsize => Some(32),
            // An upper bound of %smid: the number of cores is not known when
            // compiling.
            Self::Nsmid => Some(128),
            Self::ClusterCtaid | Self::ClusterCtarank | Self::IsExplicitCluster => Some(0),
            Self::ClusterNctaid => Some((member.unwrap_or(0) < 3) as u64),
            Self::ClusterNctarank => Some(1),
            Self::Envreg(_) | Self::Pm(_) => Some(0),
            _ => None,
        }
    }

    fn needs_function(self, target: Target) -> bool {
        self.canonical(target) == self && self.constant(None, target).is_none()
    }

    fn get_function_return_type(self) -> ast::ScalarType {
        match self {
            PtxSpecialRegister::Clock64 | PtxSpecialRegister::Globaltimer => {
                ast::ScalarType::U64
            }
            _ => ast::ScalarType::U32,
        }
    }

    fn get_function_input_type(self) -> Option<ast::ScalarType> {
        if self.is_vector() {
            Some(ast::ScalarType::U8)
        } else {
            None
        }
    }

    fn get_unprefixed_function_name(self) -> &'static str {
        match self {
            PtxSpecialRegister::Tid => "sreg_tid",
            PtxSpecialRegister::Ntid => "sreg_ntid",
            PtxSpecialRegister::Ctaid => "sreg_ctaid",
            PtxSpecialRegister::Nctaid => "sreg_nctaid",
            PtxSpecialRegister::Clock => "sreg_clock",
            PtxSpecialRegister::Clock64 => "sreg_clock64",
            PtxSpecialRegister::Globaltimer => "sreg_globaltimer",
            PtxSpecialRegister::LanemaskLt => "sreg_lanemask_lt",
            PtxSpecialRegister::LanemaskLe => "sreg_lanemask_le",
            PtxSpecialRegister::LanemaskGe => "sreg_lanemask_ge",
            PtxSpecialRegister::Laneid => "sreg_laneid",
            PtxSpecialRegister::Warpid => "sreg_warpid",
            PtxSpecialRegister::Nwarpid => "sreg_nwarpid",
            PtxSpecialRegister::Smid => "sreg_smid",
            PtxSpecialRegister::Clusterid => "sreg_clusterid",
            PtxSpecialRegister::Nclusterid => "sreg_nclusterid",
            PtxSpecialRegister::ClusterCtaid => "sreg_cluster_ctaid",
            PtxSpecialRegister::ClusterNctaid => "sreg_cluster_nctaid",
            PtxSpecialRegister::ClusterCtarank => "sreg_cluster_ctarank",
            PtxSpecialRegister::ClusterNctarank => "sreg_cluster_nctarank",
            PtxSpecialRegister::IsExplicitCluster => "sreg_is_explicit_cluster",
            _ => "",
        }
    }
}

/// Execution that reaches the end of a function body returns, as if the body
/// ended with `ret;`.
fn append_implicit_ret(
    mut directives: Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>,
) -> Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>> {
    for directive in directives.iter_mut() {
        if let Directive2::Method(Function {
            body: Some(body), ..
        }) = directive
        {
            let falls_through = !matches!(
                body.last(),
                Some(Statement::Instruction(
                    ast::Instruction::Ret { .. }
                        | ast::Instruction::Exit { .. }
                        | ast::Instruction::Trap { .. }
                        | ast::Instruction::Bra { .. }
                ))
            );
            if falls_through {
                body.push(Statement::Instruction(ast::Instruction::Ret {
                    data: ast::RetData { uniform: false },
                }));
            }
        }
    }
    directives
}

#[cfg(debug_assertions)]
fn error_unreachable() -> TranslateError {
    unreachable!()
}

#[cfg(not(debug_assertions))]
fn error_unreachable() -> TranslateError {
    TranslateError::Unreachable
}

#[cfg(debug_assertions)]
fn error_todo_msg<T: Into<String>>(msg: T) -> TranslateError {
    unreachable!("{}", msg.into())
}

#[cfg(not(debug_assertions))]
fn error_todo_msg<T: Into<String>>(msg: T) -> TranslateError {
    TranslateError::Todo(msg.into())
}

#[cfg(debug_assertions)]
fn error_todo() -> TranslateError {
    unreachable!()
}

#[cfg(not(debug_assertions))]
fn error_todo() -> TranslateError {
    TranslateError::Todo("".to_string())
}

#[cfg(debug_assertions)]
fn error_unknown_symbol<T: Into<String>>(symbol: T) -> TranslateError {
    panic!("Unknown symbol: \"{}\"", symbol.into())
}

#[cfg(not(debug_assertions))]
fn error_unknown_symbol<T: Into<String>>(symbol: T) -> TranslateError {
    TranslateError::UnknownSymbol(symbol.into())
}

#[cfg(debug_assertions)]
fn error_mismatched_type() -> TranslateError {
    panic!("Mismatched type")
}

#[cfg(not(debug_assertions))]
fn error_mismatched_type() -> TranslateError {
    TranslateError::MismatchedType
}

enum Statement<I, P: ast::Operand> {
    Label(SpirvWord),
    Variable(ast::Variable<P::Ident>),
    Instruction(I),
    // SPIR-V compatible replacement for PTX predicates
    Conditional(BranchCondition),
    Conversion(ImplicitConversion),
    Constant(ConstantDefinition),
    RetValue(ast::RetData, Vec<(SpirvWord, ast::Type)>),
    PtrAccess(PtrAccess<P>),
    RepackVector(RepackVectorDetails),
    FunctionPointer(FunctionPointerDetails),
    VectorRead(VectorRead),
    VectorWrite(VectorWrite),
    SetMode(ModeRegister),
    // This instruction is a nop, it serves as a marker to indicate that the
    // next instruction requires certain floating-point modes to be set.
    // Some transcendentals compile to a sequence of instructions that
    // require certain modes to be set _mid-function_.
    // See replace_instructions_with_functions_fp_required pass for details
    FpModeRequired {
        ftz_f32: Option<bool>,
        rnd_f32: Option<ast::RoundingMode>,
        ftz_f16f64: Option<bool>,
        rnd_f16f64: Option<ast::RoundingMode>,
    },
    FpSaturate {
        dst: SpirvWord,
        src: SpirvWord,
        type_: ast::ScalarType,
    },
    /// Subnormal values become zeros of the same sign (.ftz of the types without a denormal mode, bf16)
    FpFlush {
        dst: SpirvWord,
        src: SpirvWord,
        type_: ast::ScalarType,
    },
    /// `.loc`: the source position of the statements that follow (kept only with `Options::debug_locations`)
    Loc(ast::DebugLoc),
}

#[derive(Eq, PartialEq, Clone, Copy)]
#[cfg_attr(test, derive(Debug))]
enum ModeRegister {
    Denormal {
        f32: bool,
        f16f64: bool,
    },
    Rounding {
        f32: ast::RoundingMode,
        f16f64: ast::RoundingMode,
    },
}

impl std::fmt::Display for ModeRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModeRegister::Denormal { f32, f16f64 } => write!(
                f,
                "zluda.set_mode.denormal.f32::{}.f16f64::{}",
                if *f32 { "on" } else { "off" },
                if *f16f64 { "on" } else { "off" },
            ),
            ModeRegister::Rounding { f32, f16f64 } => {
                write!(f, "zluda.set_mode.rounding.f32{}.f16f64{}", f32, f16f64)
            }
        }
    }
}

impl<T: ast::Operand<Ident = SpirvWord>> Statement<ast::Instruction<T>, T> {
    fn visit_map<To: ast::Operand<Ident = SpirvWord>, Err>(
        self,
        visitor: &mut impl ast::VisitorMap<T, To, Err>,
    ) -> std::result::Result<Statement<ast::Instruction<To>, To>, Err> {
        Ok(match self {
            Statement::Instruction(i) => {
                return ast::visit_map(i, visitor).map(Statement::Instruction)
            }
            Statement::Label(label) => {
                Statement::Label(visitor.visit_ident(label, None, false, false)?)
            }
            Statement::Variable(var) => {
                let name = visitor.visit_ident(
                    var.name,
                    Some((&var.info.v_type, var.info.state_space)),
                    true,
                    false,
                )?;
                Statement::Variable(ast::Variable {
                    info: ast::VariableInfo {
                        align: var.info.align,
                        v_type: var.info.v_type,
                        state_space: var.info.state_space,
                        array_init: var.info.array_init,
                    },
                    name,
                })
            }
            Statement::Conditional(conditional) => {
                let predicate = visitor.visit_ident(
                    conditional.predicate,
                    Some((&ast::ScalarType::Pred.into(), ast::StateSpace::Reg)),
                    false,
                    false,
                )?;
                let if_true = visitor.visit_ident(conditional.if_true, None, false, false)?;
                let if_false = visitor.visit_ident(conditional.if_false, None, false, false)?;
                Statement::Conditional(BranchCondition {
                    predicate,
                    if_true,
                    if_false,
                })
            }
            Statement::Conversion(ImplicitConversion {
                src,
                dst,
                from_type,
                to_type,
                from_space,
                to_space,
                kind,
            }) => {
                let dst = visitor.visit_ident(
                    dst,
                    Some((&to_type, ast::StateSpace::Reg)),
                    true,
                    false,
                )?;
                let src = visitor.visit_ident(
                    src,
                    Some((&from_type, ast::StateSpace::Reg)),
                    false,
                    false,
                )?;
                Statement::Conversion(ImplicitConversion {
                    src,
                    dst,
                    from_type,
                    to_type,
                    from_space,
                    to_space,
                    kind,
                })
            }
            Statement::Constant(ConstantDefinition { dst, typ, value }) => {
                let dst = visitor.visit_ident(
                    dst,
                    Some((&typ.into(), ast::StateSpace::Reg)),
                    true,
                    false,
                )?;
                Statement::Constant(ConstantDefinition { dst, typ, value })
            }
            Statement::RetValue(data, value) => {
                let value = value
                    .into_iter()
                    .map(|(ident, type_)| {
                        Ok((
                            visitor.visit_ident(
                                ident,
                                Some((&type_, ast::StateSpace::Local)),
                                false,
                                false,
                            )?,
                            type_,
                        ))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Statement::RetValue(data, value)
            }
            Statement::PtrAccess(PtrAccess {
                underlying_type,
                state_space,
                dst,
                ptr_src,
                offset_src,
            }) => {
                let dst =
                    visitor.visit_ident(dst, Some((&underlying_type, state_space)), true, false)?;
                let ptr_src = visitor.visit_ident(
                    ptr_src,
                    Some((&underlying_type, state_space)),
                    false,
                    false,
                )?;
                let offset_src = visitor.visit(
                    offset_src,
                    Some((
                        &ast::Type::Scalar(ast::ScalarType::S64),
                        ast::StateSpace::Reg,
                    )),
                    false,
                    false,
                )?;
                Statement::PtrAccess(PtrAccess {
                    underlying_type,
                    state_space,
                    dst,
                    ptr_src,
                    offset_src,
                })
            }
            Statement::VectorRead(VectorRead {
                scalar_type,
                vector_width,
                scalar_dst: dst,
                vector_src,
                member,
            }) => {
                let scalar_t = scalar_type.into();
                let vector_t = ast::Type::Vector(vector_width, scalar_type);
                let dst: SpirvWord = visitor.visit_ident(
                    dst,
                    Some((&scalar_t, ast::StateSpace::Reg)),
                    true,
                    false,
                )?;
                let src = visitor.visit_ident(
                    vector_src,
                    Some((&vector_t, ast::StateSpace::Reg)),
                    false,
                    false,
                )?;
                Statement::VectorRead(VectorRead {
                    scalar_type,
                    vector_width,
                    scalar_dst: dst,
                    vector_src: src,
                    member,
                })
            }
            Statement::VectorWrite(VectorWrite {
                scalar_type,
                vector_width,
                vector_dst,
                vector_src,
                scalar_src,
                member,
            }) => {
                let scalar_t = scalar_type.into();
                let vector_t = ast::Type::Vector(vector_width, scalar_type);
                let vector_dst = visitor.visit_ident(
                    vector_dst,
                    Some((&vector_t, ast::StateSpace::Reg)),
                    true,
                    false,
                )?;
                let vector_src = visitor.visit_ident(
                    vector_src,
                    Some((&vector_t, ast::StateSpace::Reg)),
                    false,
                    false,
                )?;
                let scalar_src = visitor.visit_ident(
                    scalar_src,
                    Some((&scalar_t, ast::StateSpace::Reg)),
                    false,
                    false,
                )?;
                Statement::VectorWrite(VectorWrite {
                    vector_dst,
                    vector_src,
                    scalar_src,
                    scalar_type,
                    vector_width,
                    member,
                })
            }
            Statement::RepackVector(RepackVectorDetails {
                is_extract,
                typ,
                packed,
                mut unpacked,
                relaxed_type_check,
            }) => {
                let (packed, unpacked) = if is_extract {
                    for ident in unpacked.iter_mut().flatten() {
                        *ident = visitor.visit_ident(
                            *ident,
                            Some((&typ.into(), ast::StateSpace::Reg)),
                            true,
                            relaxed_type_check,
                        )?;
                    }
                    let packed = visitor.visit_ident(
                        packed,
                        Some((
                            &ast::Type::Vector(unpacked.len() as u8, typ),
                            ast::StateSpace::Reg,
                        )),
                        false,
                        false,
                    )?;
                    (packed, unpacked)
                } else {
                    let packed = visitor.visit_ident(
                        packed,
                        Some((
                            &ast::Type::Vector(unpacked.len() as u8, typ),
                            ast::StateSpace::Reg,
                        )),
                        true,
                        false,
                    )?;
                    for ident in unpacked.iter_mut().flatten() {
                        *ident = visitor.visit_ident(
                            *ident,
                            Some((&typ.into(), ast::StateSpace::Reg)),
                            false,
                            relaxed_type_check,
                        )?;
                    }
                    (packed, unpacked)
                };
                Statement::RepackVector(RepackVectorDetails {
                    is_extract,
                    typ,
                    packed,
                    unpacked,
                    relaxed_type_check,
                })
            }
            Statement::FunctionPointer(FunctionPointerDetails { dst, src }) => {
                let dst = visitor.visit_ident(
                    dst,
                    Some((
                        &ast::Type::Scalar(ast::ScalarType::U64),
                        ast::StateSpace::Reg,
                    )),
                    true,
                    false,
                )?;
                let src = visitor.visit_ident(src, None, false, false)?;
                Statement::FunctionPointer(FunctionPointerDetails { dst, src })
            }
            Statement::SetMode(mode_register) => Statement::SetMode(mode_register),
            Statement::FpSaturate { dst, src, type_ } => {
                let dst = visitor.visit_ident(
                    dst,
                    Some((&type_.into(), ast::StateSpace::Reg)),
                    true,
                    false,
                )?;
                let src = visitor.visit_ident(
                    src,
                    Some((&type_.into(), ast::StateSpace::Reg)),
                    false,
                    false,
                )?;
                Statement::FpSaturate { dst, src, type_ }
            }
            Statement::FpFlush { dst, src, type_ } => {
                let dst = visitor.visit_ident(
                    dst,
                    Some((&type_.into(), ast::StateSpace::Reg)),
                    true,
                    false,
                )?;
                let src = visitor.visit_ident(
                    src,
                    Some((&type_.into(), ast::StateSpace::Reg)),
                    false,
                    false,
                )?;
                Statement::FpFlush { dst, src, type_ }
            }
            Statement::Loc(loc) => Statement::Loc(loc),
            Statement::FpModeRequired {
                ftz_f32,
                rnd_f32,
                ftz_f16f64,
                rnd_f16f64,
            } => Statement::FpModeRequired {
                ftz_f32,
                rnd_f32,
                ftz_f16f64,
                rnd_f16f64,
            },
        })
    }
}

struct BranchCondition {
    predicate: SpirvWord,
    if_true: SpirvWord,
    if_false: SpirvWord,
}

impl std::fmt::Display for BranchCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "zluda.branch")
    }
}

#[derive(Clone)]
struct ImplicitConversion {
    src: SpirvWord,
    dst: SpirvWord,
    from_type: ast::Type,
    to_type: ast::Type,
    from_space: ast::StateSpace,
    to_space: ast::StateSpace,
    kind: ConversionKind,
}

impl std::fmt::Display for ImplicitConversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "zluda.convert_implicit{}{}{}{}{}",
            self.kind, self.to_space, self.to_type, self.from_space, self.from_type
        )
    }
}

#[derive(PartialEq, Clone, strum_macros::Display)]
#[strum(serialize_all = "snake_case", prefix = ".")]
enum ConversionKind {
    Default,
    // zero-extend/chop/bitcast depending on types
    SignExtend,
    BitToPtr,
    PtrToPtr,
    AddressOf,
}

struct ConstantDefinition {
    pub dst: SpirvWord,
    pub typ: ast::ScalarType,
    pub value: ast::ImmediateValue,
}

impl std::fmt::Display for ConstantDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "zluda.constant{} {}", self.typ, self.value)
    }
}

pub struct PtrAccess<T> {
    underlying_type: ast::Type,
    state_space: ast::StateSpace,
    dst: SpirvWord,
    ptr_src: SpirvWord,
    offset_src: T,
}

struct RepackVectorDetails {
    is_extract: bool,
    typ: ast::ScalarType,
    packed: SpirvWord,
    // None discards an extracted lane without allocating a destination register.
    unpacked: Vec<Option<SpirvWord>>,
    relaxed_type_check: bool,
}

impl std::fmt::Display for RepackVectorDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let extract = if self.is_extract {
            ".extract"
        } else {
            ".composite"
        };
        let relaxed = if self.relaxed_type_check {
            ".relaxed"
        } else {
            ""
        };
        write!(f, "zluda.repack_vector{}{}{}", extract, relaxed, self.typ)
    }
}

struct FunctionPointerDetails {
    dst: SpirvWord,
    src: SpirvWord,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct SpirvWord(u32);

impl std::fmt::Display for SpirvWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "%{}", self.0)
    }
}

impl From<u32> for SpirvWord {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl From<SpirvWord> for u32 {
    fn from(value: SpirvWord) -> Self {
        value.0
    }
}

impl ast::Operand for SpirvWord {
    type Ident = Self;

    fn from_ident(ident: Self::Ident) -> Self {
        ident
    }
}

type ExpandedStatement = Statement<ast::Instruction<SpirvWord>, SpirvWord>;

type NormalizedStatement = Statement<
    (
        Option<ast::PredAt<SpirvWord>>,
        ast::Instruction<ast::ParsedOperand<SpirvWord>>,
    ),
    ast::ParsedOperand<SpirvWord>,
>;

enum Directive2<Instruction, Operand: ast::Operand> {
    Variable(ast::LinkingDirective, ast::Variable<SpirvWord>),
    Method(Function<Instruction, Operand>),
}

struct KernelAttributes {
    flush_to_zero_f32: bool,
    flush_to_zero_f16f64: bool,
    rounding_mode_f32: ast::RoundingMode,
    rounding_mode_f16f64: ast::RoundingMode,
}

struct Function<Instruction, Operand: ast::Operand> {
    pub return_arguments: Vec<ast::Variable<Operand::Ident>>,
    pub name: Operand::Ident,
    pub input_arguments: Vec<ast::Variable<Operand::Ident>>,
    pub body: Option<Vec<Statement<Instruction, Operand>>>,
    kernel_attributes: Option<KernelAttributes>,
    import_as: Option<String>,
    tuning: Vec<ast::TuningDirective>,
    linkage: ast::LinkingDirective,
    kernel_meta32: Option<Function32>,
}

// We transform the method so that it takes an implicit pointer to memory as an
// additional argument, and all explicit memory accesses are done through that
// pointer
struct Function32 {
    implicit_memory_ptr: SpirvWord,
}

impl<I, O: ast::Operand> Function<I, O> {
    fn is_kernel(&self) -> bool {
        self.kernel_attributes.is_some()
    }
}

type NormalizedDirective2 = Directive2<
    (
        Option<ast::PredAt<SpirvWord>>,
        ast::Instruction<ast::ParsedOperand<SpirvWord>>,
    ),
    ast::ParsedOperand<SpirvWord>,
>;

type NormalizedFunction2 = Function<
    (
        Option<ast::PredAt<SpirvWord>>,
        ast::Instruction<ast::ParsedOperand<SpirvWord>>,
    ),
    ast::ParsedOperand<SpirvWord>,
>;

type UnconditionalDirective =
    Directive2<ast::Instruction<ast::ParsedOperand<SpirvWord>>, ast::ParsedOperand<SpirvWord>>;

type UnconditionalFunction =
    Function<ast::Instruction<ast::ParsedOperand<SpirvWord>>, ast::ParsedOperand<SpirvWord>>;

struct GlobalStringIdentResolver2<'input> {
    pub(crate) current_id: SpirvWord,
    pub(crate) ident_map: FxHashMap<SpirvWord, IdentEntry<'input>>,
}

impl<'input> GlobalStringIdentResolver2<'input> {
    fn new(spirv_word: SpirvWord) -> Self {
        Self {
            current_id: spirv_word,
            ident_map: FxHashMap::default(),
        }
    }

    fn register_named(
        &mut self,
        name: Cow<'input, str>,
        type_space: Option<(ast::Type, ast::StateSpace)>,
    ) -> SpirvWord {
        let new_id = self.current_id;
        self.ident_map.insert(
            new_id,
            IdentEntry {
                name: Some(name),
                type_space,
            },
        );
        self.current_id.0 += 1;
        new_id
    }

    fn register_unnamed(&mut self, type_space: Option<(ast::Type, ast::StateSpace)>) -> SpirvWord {
        let new_id = self.current_id;
        self.ident_map.insert(
            new_id,
            IdentEntry {
                name: None,
                type_space,
            },
        );
        self.current_id.0 += 1;
        new_id
    }

    fn get_typed(&self, id: SpirvWord) -> Result<&(ast::Type, ast::StateSpace), TranslateError> {
        match self.ident_map.get(&id) {
            Some(IdentEntry {
                type_space: Some(type_space),
                ..
            }) => Ok(type_space),
            _ => Err(error_unknown_symbol(format!("{:?}", id))),
        }
    }
}

struct IdentEntry<'input> {
    name: Option<Cow<'input, str>>,
    type_space: Option<(ast::Type, ast::StateSpace)>,
}

struct ScopedResolver<'input, 'b> {
    flat_resolver: &'b mut GlobalStringIdentResolver2<'input>,
    scopes: Vec<ScopeMarker<'input>>,
    // The scope of the function being normalized and the registers used in
    // it without a declaration (see get_or_declare)
    implicit_registers: Option<(usize, Vec<ast::Variable<SpirvWord>>)>,
}

impl<'input, 'b> ScopedResolver<'input, 'b> {
    fn new(flat_resolver: &'b mut GlobalStringIdentResolver2<'input>) -> Self {
        Self {
            flat_resolver,
            scopes: vec![ScopeMarker::new()],
            implicit_registers: None,
        }
    }

    // Like get, but a register (%name) that was never declared is declared
    // in the function with the type of its first use. ptxas rejects such
    // code; the K semantics of PTX (PTX-formalization) accept it.
    fn get_or_declare(
        &mut self,
        name: &'input str,
        type_space: Option<(&ast::Type, ast::StateSpace)>,
    ) -> Result<SpirvWord, TranslateError> {
        let error = match self.get(name) {
            Ok(word) => return Ok(word),
            Err(error) => error,
        };
        let (Some((scope, _)), Some((type_ @ ast::Type::Scalar(_), _)), true) =
            (&self.implicit_registers, type_space, name.starts_with('%'))
        else {
            return Err(error);
        };
        let scope = *scope;
        let result = self.flat_resolver.current_id;
        self.flat_resolver.current_id.0 += 1;
        let type_space = (type_.clone(), ast::StateSpace::Reg);
        self.scopes[scope]
            .name_to_ident
            .insert(Cow::Borrowed(name), result);
        self.scopes[scope].ident_map.insert(
            result,
            IdentEntry {
                name: Some(Cow::Borrowed(name)),
                type_space: Some(type_space),
            },
        );
        if let Some((_, registers)) = &mut self.implicit_registers {
            registers.push(ast::Variable {
                info: ast::VariableInfo {
                    align: None,
                    v_type: type_.clone(),
                    state_space: ast::StateSpace::Reg,
                    array_init: Vec::new(),
                },
                name: result,
            });
        }
        Ok(result)
    }

    fn start_scope(&mut self) {
        self.scopes.push(ScopeMarker::new());
    }

    fn end_scope(&mut self) {
        let scope = self.scopes.pop().unwrap();
        scope.flush(self.flat_resolver);
    }

    fn add_or_get_in_current_scope_untyped(
        &mut self,
        name: &'input str,
    ) -> Result<SpirvWord, TranslateError> {
        let current_scope = self.scopes.last_mut().unwrap();
        Ok(
            match current_scope.name_to_ident.entry(Cow::Borrowed(name)) {
                hash_map::Entry::Occupied(occupied_entry) => {
                    let ident = *occupied_entry.get();
                    let entry = current_scope
                        .ident_map
                        .get(&ident)
                        .ok_or_else(|| error_unreachable())?;
                    if entry.type_space.is_some() {
                        return Err(error_unknown_symbol(name));
                    }
                    ident
                }
                hash_map::Entry::Vacant(vacant_entry) => {
                    let new_id = self.flat_resolver.current_id;
                    self.flat_resolver.current_id.0 += 1;
                    vacant_entry.insert(new_id);
                    current_scope.ident_map.insert(
                        new_id,
                        IdentEntry {
                            name: Some(Cow::Borrowed(name)),
                            type_space: None,
                        },
                    );
                    new_id
                }
            },
        )
    }

    fn add(
        &mut self,
        name: Cow<'input, str>,
        type_space: Option<(ast::Type, ast::StateSpace)>,
    ) -> Result<SpirvWord, TranslateError> {
        let result = self.flat_resolver.current_id;
        self.flat_resolver.current_id.0 += 1;
        let current_scope = self.scopes.last_mut().unwrap();
        if current_scope
            .name_to_ident
            .insert(name.clone(), result)
            .is_some()
        {
            return Err(error_unknown_symbol(name));
        }
        current_scope.ident_map.insert(
            result,
            IdentEntry {
                name: Some(name),
                type_space,
            },
        );
        Ok(result)
    }

    fn get(&mut self, name: &str) -> Result<SpirvWord, TranslateError> {
        self.scopes
            .iter()
            .rev()
            .find_map(|resolver| resolver.name_to_ident.get(name).copied())
            // An undeclared identifier is an error in the input, not in the
            // compiler
            .ok_or_else(|| TranslateError::UnknownSymbol(name.to_string()))
    }

    // The innermost variable of this name that is a vector, or else the
    // innermost variable
    fn get_vector(&mut self, name: &str) -> Result<SpirvWord, TranslateError> {
        let vector = self.scopes.iter().rev().find_map(|scope| {
            let ident = scope.name_to_ident.get(name)?;
            match scope.ident_map.get(ident)?.type_space {
                Some((ast::Type::Vector(..), _)) => Some(*ident),
                _ => None,
            }
        });
        match vector {
            Some(ident) => Ok(ident),
            None => self.get(name),
        }
    }

    fn get_type(&self, name: &str) -> Option<&ast::Type> {
        self.scopes.iter().rev().find_map(|scope| {
            let ident = scope.name_to_ident.get(name)?;
            let entry = scope.ident_map.get(ident)?;
            entry.type_space.as_ref().map(|(type_, _)| type_)
        })
    }

    fn get_in_current_scope(&self, label: &'input str) -> Result<SpirvWord, TranslateError> {
        let current_scope = self.scopes.last().unwrap();
        current_scope
            .name_to_ident
            .get(label)
            .copied()
            .ok_or_else(|| error_unreachable())
    }
}

struct ScopeMarker<'input> {
    ident_map: FxHashMap<SpirvWord, IdentEntry<'input>>,
    name_to_ident: FxHashMap<Cow<'input, str>, SpirvWord>,
}

impl<'input> ScopeMarker<'input> {
    fn new() -> Self {
        Self {
            ident_map: FxHashMap::default(),
            name_to_ident: FxHashMap::default(),
        }
    }

    fn flush(self, resolver: &mut GlobalStringIdentResolver2<'input>) {
        resolver.ident_map.extend(self.ident_map);
    }
}

struct SpecialRegistersMap {
    reg_to_id: FxHashMap<PtxSpecialRegister, SpirvWord>,
    id_to_reg: FxHashMap<SpirvWord, PtxSpecialRegister>,
}

impl SpecialRegistersMap {
    fn new(resolver: &mut ScopedResolver) -> Result<Self, TranslateError> {
        let mut result = SpecialRegistersMap {
            reg_to_id: FxHashMap::default(),
            id_to_reg: FxHashMap::default(),
        };
        for sreg in PtxSpecialRegister::all() {
            let id = resolver.add(
                sreg.name(),
                Some((sreg.get_type(), ast::StateSpace::Reg)),
            )?;
            result.reg_to_id.insert(sreg, id);
            result.id_to_reg.insert(id, sreg);
        }
        Ok(result)
    }

    fn get(&self, id: SpirvWord) -> Option<PtxSpecialRegister> {
        self.id_to_reg.get(&id).copied()
    }

    fn len() -> usize {
        PtxSpecialRegister::all().count()
    }

    fn foreach_declaration<'a, 'input>(
        resolver: &'a mut GlobalStringIdentResolver2<'input>,
        target: Target,
        mut fn_: impl FnMut(
            PtxSpecialRegister,
            (
                Vec<ast::Variable<SpirvWord>>,
                SpirvWord,
                Vec<ast::Variable<SpirvWord>>,
            ),
        ),
    ) {
        for sreg in PtxSpecialRegister::all().filter(|sreg| sreg.needs_function(target)) {
            let external_fn_name = [ZLUDA_PTX_PREFIX, sreg.get_unprefixed_function_name()].concat();
            let name = resolver.register_named(Cow::Owned(external_fn_name), None);
            let return_type = sreg.get_function_return_type();
            let input_type = sreg.get_function_input_type();
            let return_arguments = vec![ast::Variable {
                info: ast::VariableInfo {
                    align: None,
                    v_type: return_type.into(),
                    state_space: ast::StateSpace::Reg,
                    array_init: Vec::new(),
                },
                name: resolver.register_unnamed(Some((return_type.into(), ast::StateSpace::Reg))),
            }];
            let input_arguments = input_type
                .into_iter()
                .map(|type_| ast::Variable {
                    info: ast::VariableInfo {
                        align: None,
                        v_type: type_.into(),
                        state_space: ast::StateSpace::Reg,
                        array_init: Vec::new(),
                    },
                    name: resolver.register_unnamed(Some((type_.into(), ast::StateSpace::Reg))),
                })
                .collect::<Vec<_>>();
            fn_(sreg, (return_arguments, name, input_arguments));
        }
    }
}

pub struct VectorRead {
    scalar_type: ast::ScalarType,
    vector_width: u8,
    scalar_dst: SpirvWord,
    vector_src: SpirvWord,
    member: u8,
}

pub struct VectorWrite {
    scalar_type: ast::ScalarType,
    vector_width: u8,
    vector_dst: SpirvWord,
    vector_src: SpirvWord,
    scalar_src: SpirvWord,
    member: u8,
}

fn scalar_to_ptx_name(this: ast::ScalarType) -> &'static str {
    match this {
        ast::ScalarType::B8 => "b8",
        ast::ScalarType::B16 => "b16",
        ast::ScalarType::B32 => "b32",
        ast::ScalarType::B64 => "b64",
        ast::ScalarType::B128 => "b128",
        ast::ScalarType::U8 => "u8",
        ast::ScalarType::U16 => "u16",
        ast::ScalarType::U16x2 => "u16x2",
        ast::ScalarType::U32 => "u32",
        ast::ScalarType::U64 => "u64",
        ast::ScalarType::S8 => "s8",
        ast::ScalarType::S16 => "s16",
        ast::ScalarType::S16x2 => "s16x2",
        ast::ScalarType::S32 => "s32",
        ast::ScalarType::S64 => "s64",
        ast::ScalarType::F16 => "f16",
        ast::ScalarType::F16x2 => "f16x2",
        ast::ScalarType::F32 => "f32",
        ast::ScalarType::F64 => "f64",
        ast::ScalarType::BF16 => "bf16",
        ast::ScalarType::BF16x2 => "bf16x2",
        ast::ScalarType::F32x2 => "f32x2",
        ast::ScalarType::Pred => "pred",
        ast::ScalarType::E4m3x2 => "e4m3x2",
        ast::ScalarType::E5m2x2 => "e5m2x2",
    }
}

type UnconditionalStatement =
    Statement<ast::Instruction<ast::ParsedOperand<SpirvWord>>, ast::ParsedOperand<SpirvWord>>;
