use ptx_parser as ast;
use quick_error::quick_error;
use rustc_hash::FxHashMap;
use std::hash::Hash;
use std::{
    borrow::Cow,
    collections::{hash_map, HashMap},
    ffi::CString,
    iter,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

mod deparamize_functions;
mod expand_operands;
mod fix_special_registers2;
mod hoist_globals;
mod insert_explicit_load_store;
mod insert_implicit_conversions2;
mod insert_post_saturation;
mod instruction_mode_to_global_mode;
pub mod llvm;
mod normalize_basic_blocks;
mod normalize_identifiers2;
mod normalize_predicates2;
mod remove_unreachable_basic_blocks;
mod replace_instructions_with_functions;
mod replace_instructions_with_functions_fp_required;
mod replace_known_functions;
mod resolve_function_pointers;

#[cfg(test)]
mod test;

static ZLUDA_PTX_IMPL: &'static [u8] = include_bytes!("../../lib/zluda_ptx_impl.bc");
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
    }
}

/// GPU attributes needed at compile time.
#[derive(serde::Serialize)]
pub struct Attributes {
    /// Clock frequency in kHz.
    pub clock_rate: u32,
}

pub fn to_llvm_module<'input>(
    ast: ast::Module<'input>,
    attributes: Attributes,
) -> Result<Module, TranslateError> {
    let mut flat_resolver = GlobalStringIdentResolver2::<'input>::new(SpirvWord(1));
    let mut scoped_resolver = ScopedResolver::new(&mut flat_resolver);
    let sreg_map = SpecialRegistersMap2::new(&mut scoped_resolver)?;
    let directives = normalize_identifiers2::run(&mut scoped_resolver, ast.directives)?;
    let directives = replace_known_functions::run(&mut flat_resolver, directives);
    let directives = normalize_predicates2::run(&mut flat_resolver, directives)?;
    let directives = resolve_function_pointers::run(directives)?;
    let directives = fix_special_registers2::run(&mut flat_resolver, &sreg_map, directives)?;
    let directives = expand_operands::run(&mut flat_resolver, directives)?;
    let directives = insert_post_saturation::run(&mut flat_resolver, directives)?;
    let directives = deparamize_functions::run(&mut flat_resolver, directives)?;
    let directives =
        replace_instructions_with_functions_fp_required::run(&mut flat_resolver, directives)?;
    let directives = normalize_basic_blocks::run(&mut flat_resolver, directives)?;
    let directives = remove_unreachable_basic_blocks::run(directives)?;
    let directives = instruction_mode_to_global_mode::run(&mut flat_resolver, directives)?;
    let directives = insert_explicit_load_store::run(&mut flat_resolver, directives)?;
    let directives = insert_implicit_conversions2::run(&mut flat_resolver, directives)?;
    let directives = replace_instructions_with_functions::run(&mut flat_resolver, directives)?;
    let directives = hoist_globals::run(directives)?;

    let context = llvm::Context::new();
    let llvm_ir = llvm::emit::run(&context, flat_resolver, directives)?;
    let attributes_ir = llvm::attributes::run(&context, attributes)?;
    Ok(Module {
        llvm_ir,
        attributes_ir,
        kernel_info: HashMap::new(),
        _context: context,
    })
}

pub struct Module {
    pub llvm_ir: llvm::Module,
    pub attributes_ir: llvm::Module,
    pub kernel_info: HashMap<String, KernelInfo>,
    _context: llvm::Context,
}

impl Module {
    pub fn linked_bitcode(&self) -> &[u8] {
        ZLUDA_PTX_IMPL
    }
}

pub struct KernelInfo {
    pub arguments_sizes: Vec<(usize, bool)>,
    pub uses_shared_mem: bool,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, EnumIter)]
enum PtxSpecialRegister {
    Tid,
    Ntid,
    Ctaid,
    Nctaid,
    Clock,
    LanemaskLt,
}

impl PtxSpecialRegister {
    fn as_str(self) -> &'static str {
        match self {
            Self::Tid => "%tid",
            Self::Ntid => "%ntid",
            Self::Ctaid => "%ctaid",
            Self::Nctaid => "%nctaid",
            Self::Clock => "%clock",
            Self::LanemaskLt => "%lanemask_lt",
        }
    }

    fn get_type(self) -> ast::Type {
        match self {
            PtxSpecialRegister::Tid
            | PtxSpecialRegister::Ntid
            | PtxSpecialRegister::Ctaid
            | PtxSpecialRegister::Nctaid => ast::Type::Vector(4, self.get_function_return_type()),
            _ => ast::Type::Scalar(self.get_function_return_type()),
        }
    }

    fn get_function_return_type(self) -> ast::ScalarType {
        match self {
            PtxSpecialRegister::Tid => ast::ScalarType::U32,
            PtxSpecialRegister::Ntid => ast::ScalarType::U32,
            PtxSpecialRegister::Ctaid => ast::ScalarType::U32,
            PtxSpecialRegister::Nctaid => ast::ScalarType::U32,
            PtxSpecialRegister::Clock => ast::ScalarType::U32,
            PtxSpecialRegister::LanemaskLt => ast::ScalarType::U32,
        }
    }

    fn get_function_input_type(self) -> Option<ast::ScalarType> {
        match self {
            PtxSpecialRegister::Tid
            | PtxSpecialRegister::Ntid
            | PtxSpecialRegister::Ctaid
            | PtxSpecialRegister::Nctaid => Some(ast::ScalarType::U8),
            PtxSpecialRegister::Clock | PtxSpecialRegister::LanemaskLt => None,
        }
    }

    fn get_unprefixed_function_name(self) -> &'static str {
        match self {
            PtxSpecialRegister::Tid => "sreg_tid",
            PtxSpecialRegister::Ntid => "sreg_ntid",
            PtxSpecialRegister::Ctaid => "sreg_ctaid",
            PtxSpecialRegister::Nctaid => "sreg_nctaid",
            PtxSpecialRegister::Clock => "sreg_clock",
            PtxSpecialRegister::LanemaskLt => "sreg_lanemask_lt",
        }
    }
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
    panic!()
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
    Conditional(BrachCondition),
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
    },
    FpSaturate {
        dst: SpirvWord,
        src: SpirvWord,
        type_: ast::ScalarType,
    },
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
                    Some((&var.v_type, var.state_space)),
                    true,
                    false,
                )?;
                Statement::Variable(ast::Variable {
                    align: var.align,
                    v_type: var.v_type,
                    state_space: var.state_space,
                    name,
                    array_init: var.array_init,
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
                Statement::Conditional(BrachCondition {
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
                unpacked,
                relaxed_type_check,
            }) => {
                let (packed, unpacked) = if is_extract {
                    let unpacked = unpacked
                        .into_iter()
                        .map(|ident| {
                            visitor.visit_ident(
                                ident,
                                Some((&typ.into(), ast::StateSpace::Reg)),
                                true,
                                relaxed_type_check,
                            )
                        })
                        .collect::<Result<Vec<_>, _>>()?;
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
                    let unpacked = unpacked
                        .into_iter()
                        .map(|ident| {
                            visitor.visit_ident(
                                ident,
                                Some((&typ.into(), ast::StateSpace::Reg)),
                                false,
                                relaxed_type_check,
                            )
                        })
                        .collect::<Result<Vec<_>, _>>()?;
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
            Statement::FpModeRequired { ftz_f32, rnd_f32 } => {
                Statement::FpModeRequired { ftz_f32, rnd_f32 }
            }
        })
    }
}

struct BrachCondition {
    predicate: SpirvWord,
    if_true: SpirvWord,
    if_false: SpirvWord,
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

#[derive(PartialEq, Clone)]
enum ConversionKind {
    Default,
    // zero-extend/chop/bitcast depending on types
    SignExtend,
    BitToPtr,
    PtrToPtr,
    AddressOf,
}

impl std::fmt::Display for ConversionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ConversionKind::Default => ".default",
            ConversionKind::SignExtend => ".sign_extend",
            ConversionKind::BitToPtr => ".bit_to_ptr",
            ConversionKind::PtrToPtr => ".ptr_to_ptr",
            ConversionKind::AddressOf => ".address_of",
        };
        write!(f, "{}", s)
    }
}

struct ConstantDefinition {
    pub dst: SpirvWord,
    pub typ: ast::ScalarType,
    pub value: ast::ImmediateValue,
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
    unpacked: Vec<SpirvWord>,
    relaxed_type_check: bool,
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
    Method(Function2<Instruction, Operand>),
}

struct Function2<Instruction, Operand: ast::Operand> {
    pub return_arguments: Vec<ast::Variable<Operand::Ident>>,
    pub name: Operand::Ident,
    pub input_arguments: Vec<ast::Variable<Operand::Ident>>,
    pub body: Option<Vec<Statement<Instruction, Operand>>>,
    is_kernel: bool,
    import_as: Option<String>,
    tuning: Vec<ast::TuningDirective>,
    linkage: ast::LinkingDirective,
    flush_to_zero_f32: bool,
    flush_to_zero_f16f64: bool,
    rounding_mode_f32: ast::RoundingMode,
    rounding_mode_f16f64: ast::RoundingMode,
}

type NormalizedDirective2 = Directive2<
    (
        Option<ast::PredAt<SpirvWord>>,
        ast::Instruction<ast::ParsedOperand<SpirvWord>>,
    ),
    ast::ParsedOperand<SpirvWord>,
>;

type NormalizedFunction2 = Function2<
    (
        Option<ast::PredAt<SpirvWord>>,
        ast::Instruction<ast::ParsedOperand<SpirvWord>>,
    ),
    ast::ParsedOperand<SpirvWord>,
>;

type UnconditionalDirective =
    Directive2<ast::Instruction<ast::ParsedOperand<SpirvWord>>, ast::ParsedOperand<SpirvWord>>;

type UnconditionalFunction =
    Function2<ast::Instruction<ast::ParsedOperand<SpirvWord>>, ast::ParsedOperand<SpirvWord>>;

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
}

impl<'input, 'b> ScopedResolver<'input, 'b> {
    fn new(flat_resolver: &'b mut GlobalStringIdentResolver2<'input>) -> Self {
        Self {
            flat_resolver,
            scopes: vec![ScopeMarker::new()],
        }
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
            .ok_or_else(|| error_unknown_symbol(name))
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

struct SpecialRegistersMap2 {
    reg_to_id: FxHashMap<PtxSpecialRegister, SpirvWord>,
    id_to_reg: FxHashMap<SpirvWord, PtxSpecialRegister>,
}

impl SpecialRegistersMap2 {
    fn new(resolver: &mut ScopedResolver) -> Result<Self, TranslateError> {
        let mut result = SpecialRegistersMap2 {
            reg_to_id: FxHashMap::default(),
            id_to_reg: FxHashMap::default(),
        };
        for sreg in PtxSpecialRegister::iter() {
            let text = sreg.as_str();
            let id = resolver.add(
                Cow::Borrowed(text),
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
        PtxSpecialRegister::iter().len()
    }

    fn foreach_declaration<'a, 'input>(
        resolver: &'a mut GlobalStringIdentResolver2<'input>,
        mut fn_: impl FnMut(
            PtxSpecialRegister,
            (
                Vec<ast::Variable<SpirvWord>>,
                SpirvWord,
                Vec<ast::Variable<SpirvWord>>,
            ),
        ),
    ) {
        for sreg in PtxSpecialRegister::iter() {
            let external_fn_name = [ZLUDA_PTX_PREFIX, sreg.get_unprefixed_function_name()].concat();
            let name = resolver.register_named(Cow::Owned(external_fn_name), None);
            let return_type = sreg.get_function_return_type();
            let input_type = sreg.get_function_input_type();
            let return_arguments = vec![ast::Variable {
                align: None,
                v_type: return_type.into(),
                state_space: ast::StateSpace::Reg,
                name: resolver.register_unnamed(Some((return_type.into(), ast::StateSpace::Reg))),
                array_init: Vec::new(),
            }];
            let input_arguments = input_type
                .into_iter()
                .map(|type_| ast::Variable {
                    align: None,
                    v_type: type_.into(),
                    state_space: ast::StateSpace::Reg,
                    name: resolver.register_unnamed(Some((type_.into(), ast::StateSpace::Reg))),
                    array_init: Vec::new(),
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
        ast::ScalarType::Pred => "pred",
    }
}

type UnconditionalStatement =
    Statement<ast::Instruction<ast::ParsedOperand<SpirvWord>>, ast::ParsedOperand<SpirvWord>>;
