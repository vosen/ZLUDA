use ptx_parser as ast;
use rspirv::{binary::Assemble, dr};
use rustc_hash::FxHashMap;
use std::hash::Hash;
use std::num::NonZeroU8;
use std::{
    borrow::Cow,
    cell::RefCell,
    collections::{hash_map, HashMap, HashSet},
    ffi::CString,
    iter,
    marker::PhantomData,
    mem,
    rc::Rc,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

mod convert_dynamic_shared_memory_usage;
mod convert_to_stateful_memory_access;
mod convert_to_typed;
mod deparamize_functions;
pub(crate) mod emit_llvm;
mod emit_spirv;
mod expand_arguments;
mod expand_operands;
mod extract_globals;
mod fix_special_registers;
mod fix_special_registers2;
mod insert_explicit_load_store;
mod insert_implicit_conversions;
mod insert_mem_ssa_statements;
mod normalize_identifiers;
mod normalize_identifiers2;
mod normalize_labels;
mod normalize_predicates;
mod normalize_predicates2;
mod resolve_function_pointers;

static ZLUDA_PTX_IMPL_INTEL: &'static [u8] = include_bytes!("../../lib/zluda_ptx_impl.spv");
static ZLUDA_PTX_IMPL_AMD: &'static [u8] = include_bytes!("../../lib/zluda_ptx_impl.bc");
const ZLUDA_PTX_PREFIX: &'static str = "__zluda_ptx_impl__";

pub fn to_llvm_module<'input>(ast: ast::Module<'input>) -> Result<Module, TranslateError> {
    let mut id_defs = GlobalStringIdResolver::<'input>::new(SpirvWord(1));
    let mut ptx_impl_imports = HashMap::new();
    let directives = ast
        .directives
        .into_iter()
        .filter_map(|directive| {
            translate_directive(&mut id_defs, &mut ptx_impl_imports, directive).transpose()
        })
        .collect::<Result<Vec<_>, _>>()?;
    let directives = hoist_function_globals(directives);
    let must_link_ptx_impl = ptx_impl_imports.len() > 0;
    let mut directives = ptx_impl_imports
        .into_iter()
        .map(|(_, v)| v)
        .chain(directives.into_iter())
        .collect::<Vec<_>>();
    let mut builder = dr::Builder::new();
    builder.reserve_ids(id_defs.current_id().0);
    let call_map = MethodsCallMap::new(&directives);
    let mut directives =
        convert_dynamic_shared_memory_usage::run(directives, &call_map, &mut || {
            SpirvWord(builder.id())
        })?;
    normalize_variable_decls(&mut directives);
    let denorm_information = compute_denorm_information(&directives);
    let llvm_ir = emit_llvm::run(&id_defs, call_map, directives)?;
    Ok(Module {
        llvm_ir,
        kernel_info: HashMap::new(),
    })
}

pub fn to_llvm_module2<'input>(ast: ast::Module<'input>) -> Result<Module, TranslateError> {
    let mut flat_resolver = GlobalStringIdentResolver2::<'input>::new(SpirvWord(1));
    let mut scoped_resolver = ScopedResolver::new(&mut flat_resolver);
    let sreg_map = SpecialRegistersMap2::new(&mut scoped_resolver)?;
    let directives = normalize_identifiers2::run(&mut scoped_resolver, ast.directives)?;
    let directives = normalize_predicates2::run(&mut flat_resolver, directives)?;
    let directives = resolve_function_pointers::run(directives)?;
    let directives = fix_special_registers2::run(&mut flat_resolver, &sreg_map, directives)?;
    let directives = expand_operands::run(&mut flat_resolver, directives)?;
    let directives = deparamize_functions::run(&mut flat_resolver, directives)?;
    let directives = insert_explicit_load_store::run(&mut flat_resolver, directives)?;
    todo!()
}

fn translate_directive<'input, 'a>(
    id_defs: &'a mut GlobalStringIdResolver<'input>,
    ptx_impl_imports: &'a mut HashMap<String, Directive<'input>>,
    d: ast::Directive<'input, ast::ParsedOperand<&'input str>>,
) -> Result<Option<Directive<'input>>, TranslateError> {
    Ok(match d {
        ast::Directive::Variable(linking, var) => Some(Directive::Variable(
            linking,
            ast::Variable {
                align: var.align,
                v_type: var.v_type.clone(),
                state_space: var.state_space,
                name: id_defs.get_or_add_def_typed(var.name, var.v_type, var.state_space, true),
                array_init: var.array_init,
            },
        )),
        ast::Directive::Method(linkage, f) => {
            translate_function(id_defs, ptx_impl_imports, linkage, f)?.map(Directive::Method)
        }
    })
}

type ParsedFunction<'a> = ast::Function<'a, &'a str, ast::Statement<ast::ParsedOperand<&'a str>>>;

fn translate_function<'input, 'a>(
    id_defs: &'a mut GlobalStringIdResolver<'input>,
    ptx_impl_imports: &'a mut HashMap<String, Directive<'input>>,
    linkage: ast::LinkingDirective,
    f: ParsedFunction<'input>,
) -> Result<Option<Function<'input>>, TranslateError> {
    let import_as = match &f.func_directive {
        ast::MethodDeclaration {
            name: ast::MethodName::Func(func_name),
            ..
        } if *func_name == "__assertfail" || *func_name == "vprintf" => {
            Some([ZLUDA_PTX_PREFIX, func_name].concat())
        }
        _ => None,
    };
    let (str_resolver, fn_resolver, fn_decl) = id_defs.start_fn(&f.func_directive)?;
    let mut func = to_ssa(
        ptx_impl_imports,
        str_resolver,
        fn_resolver,
        fn_decl,
        f.body,
        f.tuning,
        linkage,
    )?;
    func.import_as = import_as;
    if func.import_as.is_some() {
        ptx_impl_imports.insert(
            func.import_as.as_ref().unwrap().clone(),
            Directive::Method(func),
        );
        Ok(None)
    } else {
        Ok(Some(func))
    }
}

fn to_ssa<'input, 'b>(
    ptx_impl_imports: &'b mut HashMap<String, Directive<'input>>,
    mut id_defs: FnStringIdResolver<'input, 'b>,
    fn_defs: GlobalFnDeclResolver<'input, 'b>,
    func_decl: Rc<RefCell<ast::MethodDeclaration<'input, SpirvWord>>>,
    f_body: Option<Vec<ast::Statement<ast::ParsedOperand<&'input str>>>>,
    tuning: Vec<ast::TuningDirective>,
    linkage: ast::LinkingDirective,
) -> Result<Function<'input>, TranslateError> {
    //deparamize_function_decl(&func_decl)?;
    let f_body = match f_body {
        Some(vec) => vec,
        None => {
            return Ok(Function {
                func_decl: func_decl,
                body: None,
                globals: Vec::new(),
                import_as: None,
                tuning,
                linkage,
            })
        }
    };
    let normalized_ids = normalize_identifiers::run(&mut id_defs, &fn_defs, f_body)?;
    let mut numeric_id_defs = id_defs.finish();
    let unadorned_statements = normalize_predicates::run(normalized_ids, &mut numeric_id_defs)?;
    let typed_statements =
        convert_to_typed::run(unadorned_statements, &fn_defs, &mut numeric_id_defs)?;
    let typed_statements =
        fix_special_registers::run(ptx_impl_imports, typed_statements, &mut numeric_id_defs)?;
    let (func_decl, typed_statements) =
        convert_to_stateful_memory_access::run(func_decl, typed_statements, &mut numeric_id_defs)?;
    let ssa_statements = insert_mem_ssa_statements::run(
        typed_statements,
        &mut numeric_id_defs,
        &mut (*func_decl).borrow_mut(),
    )?;
    let mut numeric_id_defs = numeric_id_defs.finish();
    let expanded_statements = expand_arguments::run(ssa_statements, &mut numeric_id_defs)?;
    let expanded_statements =
        insert_implicit_conversions::run(expanded_statements, &mut numeric_id_defs)?;
    let mut numeric_id_defs = numeric_id_defs.unmut();
    let labeled_statements = normalize_labels::run(expanded_statements, &mut numeric_id_defs);
    let (f_body, globals) =
        extract_globals::run(labeled_statements, ptx_impl_imports, &mut numeric_id_defs)?;
    Ok(Function {
        func_decl: func_decl,
        globals: globals,
        body: Some(f_body),
        import_as: None,
        tuning,
        linkage,
    })
}

pub struct Module {
    pub llvm_ir: emit_llvm::MemoryBuffer,
    pub kernel_info: HashMap<String, KernelInfo>,
}

struct GlobalStringIdResolver<'input> {
    current_id: SpirvWord,
    variables: HashMap<Cow<'input, str>, SpirvWord>,
    pub(crate) reverse_variables: HashMap<SpirvWord, &'input str>,
    variables_type_check: HashMap<SpirvWord, Option<(ast::Type, ast::StateSpace, bool)>>,
    special_registers: SpecialRegistersMap,
    fns: HashMap<SpirvWord, FnSigMapper<'input>>,
}

impl<'input> GlobalStringIdResolver<'input> {
    fn new(start_id: SpirvWord) -> Self {
        Self {
            current_id: start_id,
            variables: HashMap::new(),
            reverse_variables: HashMap::new(),
            variables_type_check: HashMap::new(),
            special_registers: SpecialRegistersMap::new(),
            fns: HashMap::new(),
        }
    }

    fn get_or_add_def(&mut self, id: &'input str) -> SpirvWord {
        self.get_or_add_impl(id, None)
    }

    fn get_or_add_def_typed(
        &mut self,
        id: &'input str,
        typ: ast::Type,
        state_space: ast::StateSpace,
        is_variable: bool,
    ) -> SpirvWord {
        self.get_or_add_impl(id, Some((typ, state_space, is_variable)))
    }

    fn get_or_add_impl(
        &mut self,
        id: &'input str,
        typ: Option<(ast::Type, ast::StateSpace, bool)>,
    ) -> SpirvWord {
        let id = match self.variables.entry(Cow::Borrowed(id)) {
            hash_map::Entry::Occupied(e) => *(e.get()),
            hash_map::Entry::Vacant(e) => {
                let numeric_id = self.current_id;
                e.insert(numeric_id);
                self.reverse_variables.insert(numeric_id, id);
                self.current_id.0 += 1;
                numeric_id
            }
        };
        self.variables_type_check.insert(id, typ);
        id
    }

    fn get_id(&self, id: &str) -> Result<SpirvWord, TranslateError> {
        self.variables
            .get(id)
            .copied()
            .ok_or_else(error_unknown_symbol)
    }

    fn current_id(&self) -> SpirvWord {
        self.current_id
    }

    fn start_fn<'b>(
        &'b mut self,
        header: &'b ast::MethodDeclaration<'input, &'input str>,
    ) -> Result<
        (
            FnStringIdResolver<'input, 'b>,
            GlobalFnDeclResolver<'input, 'b>,
            Rc<RefCell<ast::MethodDeclaration<'input, SpirvWord>>>,
        ),
        TranslateError,
    > {
        // In case a function decl was inserted earlier we want to use its id
        let name_id = self.get_or_add_def(header.name());
        let mut fn_resolver = FnStringIdResolver {
            current_id: &mut self.current_id,
            global_variables: &self.variables,
            global_type_check: &self.variables_type_check,
            special_registers: &mut self.special_registers,
            variables: vec![HashMap::new(); 1],
            type_check: HashMap::new(),
        };
        let return_arguments = rename_fn_params(&mut fn_resolver, &header.return_arguments);
        let input_arguments = rename_fn_params(&mut fn_resolver, &header.input_arguments);
        let name = match header.name {
            ast::MethodName::Kernel(name) => ast::MethodName::Kernel(name),
            ast::MethodName::Func(_) => ast::MethodName::Func(name_id),
        };
        let fn_decl = ast::MethodDeclaration {
            return_arguments,
            name,
            input_arguments,
            shared_mem: None,
        };
        let new_fn_decl = if !matches!(fn_decl.name, ast::MethodName::Kernel(_)) {
            let resolver = FnSigMapper::remap_to_spirv_repr(fn_decl);
            let new_fn_decl = resolver.func_decl.clone();
            self.fns.insert(name_id, resolver);
            new_fn_decl
        } else {
            Rc::new(RefCell::new(fn_decl))
        };
        Ok((
            fn_resolver,
            GlobalFnDeclResolver { fns: &self.fns },
            new_fn_decl,
        ))
    }
}

fn rename_fn_params<'a, 'b>(
    fn_resolver: &mut FnStringIdResolver<'a, 'b>,
    args: &'b [ast::Variable<&'a str>],
) -> Vec<ast::Variable<SpirvWord>> {
    args.iter()
        .map(|a| ast::Variable {
            name: fn_resolver.add_def(a.name, Some((a.v_type.clone(), a.state_space)), true),
            v_type: a.v_type.clone(),
            state_space: a.state_space,
            align: a.align,
            array_init: a.array_init.clone(),
        })
        .collect()
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
    fn try_parse(s: &str) -> Option<Self> {
        match s {
            "%tid" => Some(Self::Tid),
            "%ntid" => Some(Self::Ntid),
            "%ctaid" => Some(Self::Ctaid),
            "%nctaid" => Some(Self::Nctaid),
            "%clock" => Some(Self::Clock),
            "%lanemask_lt" => Some(Self::LanemaskLt),
            _ => None,
        }
    }

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

struct SpecialRegistersMap {
    reg_to_id: HashMap<PtxSpecialRegister, SpirvWord>,
    id_to_reg: HashMap<SpirvWord, PtxSpecialRegister>,
}

impl SpecialRegistersMap {
    fn new() -> Self {
        SpecialRegistersMap {
            reg_to_id: HashMap::new(),
            id_to_reg: HashMap::new(),
        }
    }

    fn get(&self, id: SpirvWord) -> Option<PtxSpecialRegister> {
        self.id_to_reg.get(&id).copied()
    }

    fn get_or_add(&mut self, current_id: &mut SpirvWord, reg: PtxSpecialRegister) -> SpirvWord {
        match self.reg_to_id.entry(reg) {
            hash_map::Entry::Occupied(e) => *e.get(),
            hash_map::Entry::Vacant(e) => {
                let numeric_id = SpirvWord(current_id.0);
                current_id.0 += 1;
                e.insert(numeric_id);
                self.id_to_reg.insert(numeric_id, reg);
                numeric_id
            }
        }
    }
}

struct FnStringIdResolver<'input, 'b> {
    current_id: &'b mut SpirvWord,
    global_variables: &'b HashMap<Cow<'input, str>, SpirvWord>,
    global_type_check: &'b HashMap<SpirvWord, Option<(ast::Type, ast::StateSpace, bool)>>,
    special_registers: &'b mut SpecialRegistersMap,
    variables: Vec<HashMap<Cow<'input, str>, SpirvWord>>,
    type_check: HashMap<SpirvWord, Option<(ast::Type, ast::StateSpace, bool)>>,
}

impl<'a, 'b> FnStringIdResolver<'a, 'b> {
    fn finish(self) -> NumericIdResolver<'b> {
        NumericIdResolver {
            current_id: self.current_id,
            global_type_check: self.global_type_check,
            type_check: self.type_check,
            special_registers: self.special_registers,
        }
    }

    fn start_block(&mut self) {
        self.variables.push(HashMap::new())
    }

    fn end_block(&mut self) {
        self.variables.pop();
    }

    fn get_id(&mut self, id: &str) -> Result<SpirvWord, TranslateError> {
        for scope in self.variables.iter().rev() {
            match scope.get(id) {
                Some(id) => return Ok(*id),
                None => continue,
            }
        }
        match self.global_variables.get(id) {
            Some(id) => Ok(*id),
            None => {
                let sreg = PtxSpecialRegister::try_parse(id).ok_or_else(error_unknown_symbol)?;
                Ok(self.special_registers.get_or_add(self.current_id, sreg))
            }
        }
    }

    fn add_def(
        &mut self,
        id: &'a str,
        typ: Option<(ast::Type, ast::StateSpace)>,
        is_variable: bool,
    ) -> SpirvWord {
        let numeric_id = *self.current_id;
        self.variables
            .last_mut()
            .unwrap()
            .insert(Cow::Borrowed(id), numeric_id);
        self.type_check.insert(
            numeric_id,
            typ.map(|(typ, space)| (typ, space, is_variable)),
        );
        self.current_id.0 += 1;
        numeric_id
    }

    #[must_use]
    fn add_defs(
        &mut self,
        base_id: &'a str,
        count: u32,
        typ: ast::Type,
        state_space: ast::StateSpace,
        is_variable: bool,
    ) -> impl Iterator<Item = SpirvWord> {
        let numeric_id = *self.current_id;
        for i in 0..count {
            self.variables.last_mut().unwrap().insert(
                Cow::Owned(format!("{}{}", base_id, i)),
                SpirvWord(numeric_id.0 + i),
            );
            self.type_check.insert(
                SpirvWord(numeric_id.0 + i),
                Some((typ.clone(), state_space, is_variable)),
            );
        }
        self.current_id.0 += count;
        (0..count)
            .into_iter()
            .map(move |i| SpirvWord(i + numeric_id.0))
    }
}

struct NumericIdResolver<'b> {
    current_id: &'b mut SpirvWord,
    global_type_check: &'b HashMap<SpirvWord, Option<(ast::Type, ast::StateSpace, bool)>>,
    type_check: HashMap<SpirvWord, Option<(ast::Type, ast::StateSpace, bool)>>,
    special_registers: &'b mut SpecialRegistersMap,
}

impl<'b> NumericIdResolver<'b> {
    fn finish(self) -> MutableNumericIdResolver<'b> {
        MutableNumericIdResolver { base: self }
    }

    fn get_typed(
        &self,
        id: SpirvWord,
    ) -> Result<(ast::Type, ast::StateSpace, bool), TranslateError> {
        match self.type_check.get(&id) {
            Some(Some(x)) => Ok(x.clone()),
            Some(None) => Err(TranslateError::UntypedSymbol),
            None => match self.special_registers.get(id) {
                Some(x) => Ok((x.get_type(), ast::StateSpace::Reg, true)),
                None => match self.global_type_check.get(&id) {
                    Some(Some(result)) => Ok(result.clone()),
                    Some(None) | None => Err(TranslateError::UntypedSymbol),
                },
            },
        }
    }

    // This is for identifiers which will be emitted later as OpVariable
    // They are candidates for insertion of LoadVar/StoreVar
    fn register_variable(&mut self, typ: ast::Type, state_space: ast::StateSpace) -> SpirvWord {
        let new_id = *self.current_id;
        self.type_check
            .insert(new_id, Some((typ, state_space, true)));
        self.current_id.0 += 1;
        new_id
    }

    fn register_intermediate(&mut self, typ: Option<(ast::Type, ast::StateSpace)>) -> SpirvWord {
        let new_id = *self.current_id;
        self.type_check
            .insert(new_id, typ.map(|(t, space)| (t, space, false)));
        self.current_id.0 += 1;
        new_id
    }
}

struct MutableNumericIdResolver<'b> {
    base: NumericIdResolver<'b>,
}

impl<'b> MutableNumericIdResolver<'b> {
    fn unmut(self) -> NumericIdResolver<'b> {
        self.base
    }

    fn get_typed(&self, id: SpirvWord) -> Result<(ast::Type, ast::StateSpace), TranslateError> {
        self.base.get_typed(id).map(|(t, space, _)| (t, space))
    }

    fn register_intermediate(&mut self, typ: ast::Type, state_space: ast::StateSpace) -> SpirvWord {
        self.base.register_intermediate(Some((typ, state_space)))
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum TranslateError {
        UnknownSymbol {}
        UntypedSymbol {}
        MismatchedType {}
        Spirv(err: rspirv::dr::Error) {
            from()
            display("{}", err)
            cause(err)
        }
        Unreachable {}
        Todo {}
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
fn error_unknown_symbol() -> TranslateError {
    panic!()
}

#[cfg(not(debug_assertions))]
fn error_unknown_symbol() -> TranslateError {
    TranslateError::UnknownSymbol
}

#[cfg(debug_assertions)]
fn error_mismatched_type() -> TranslateError {
    panic!()
}

#[cfg(not(debug_assertions))]
fn error_mismatched_type() -> TranslateError {
    TranslateError::MismatchedType
}

pub struct GlobalFnDeclResolver<'input, 'a> {
    fns: &'a HashMap<SpirvWord, FnSigMapper<'input>>,
}

impl<'input, 'a> GlobalFnDeclResolver<'input, 'a> {
    fn get_fn_sig_resolver(&self, id: SpirvWord) -> Result<&FnSigMapper<'input>, TranslateError> {
        self.fns.get(&id).ok_or_else(error_unknown_symbol)
    }
}

struct FnSigMapper<'input> {
    // true - stays as return argument
    // false - is moved to input argument
    return_param_args: Vec<bool>,
    func_decl: Rc<RefCell<ast::MethodDeclaration<'input, SpirvWord>>>,
}

impl<'input> FnSigMapper<'input> {
    fn remap_to_spirv_repr(mut method: ast::MethodDeclaration<'input, SpirvWord>) -> Self {
        let return_param_args = method
            .return_arguments
            .iter()
            .map(|a| a.state_space != ast::StateSpace::Param)
            .collect::<Vec<_>>();
        let mut new_return_arguments = Vec::new();
        for arg in method.return_arguments.into_iter() {
            if arg.state_space == ast::StateSpace::Param {
                method.input_arguments.push(arg);
            } else {
                new_return_arguments.push(arg);
            }
        }
        method.return_arguments = new_return_arguments;
        FnSigMapper {
            return_param_args,
            func_decl: Rc::new(RefCell::new(method)),
        }
    }

    fn resolve_in_spirv_repr(
        &self,
        data: ast::CallDetails,
        arguments: ast::CallArgs<ast::ParsedOperand<SpirvWord>>,
    ) -> Result<ast::Instruction<ast::ParsedOperand<SpirvWord>>, TranslateError> {
        let func_decl = (*self.func_decl).borrow();
        let mut data_return = Vec::new();
        let mut arguments_return = Vec::new();
        let mut data_input = data.input_arguments;
        let mut arguments_input = arguments.input_arguments;
        let mut func_decl_return_iter = func_decl.return_arguments.iter();
        let mut func_decl_input_iter = func_decl.input_arguments[arguments_input.len()..].iter();
        for (idx, id) in arguments.return_arguments.iter().enumerate() {
            let stays_as_return = match self.return_param_args.get(idx) {
                Some(x) => *x,
                None => return Err(TranslateError::MismatchedType),
            };
            if stays_as_return {
                if let Some(var) = func_decl_return_iter.next() {
                    data_return.push((var.v_type.clone(), var.state_space));
                    arguments_return.push(*id);
                } else {
                    return Err(TranslateError::MismatchedType);
                }
            } else {
                if let Some(var) = func_decl_input_iter.next() {
                    data_input.push((var.v_type.clone(), var.state_space));
                    arguments_input.push(ast::ParsedOperand::Reg(*id));
                } else {
                    return Err(TranslateError::MismatchedType);
                }
            }
        }
        if arguments_return.len() != func_decl.return_arguments.len()
            || arguments_input.len() != func_decl.input_arguments.len()
        {
            return Err(TranslateError::MismatchedType);
        }
        let data = ast::CallDetails {
            uniform: data.uniform,
            return_arguments: data_return,
            input_arguments: data_input,
        };
        let arguments = ast::CallArgs {
            func: arguments.func,
            return_arguments: arguments_return,
            input_arguments: arguments_input,
        };
        Ok(ast::Instruction::Call { data, arguments })
    }
}

enum Statement<I, P: ast::Operand> {
    Label(SpirvWord),
    Variable(ast::Variable<P::Ident>),
    Instruction(I),
    // SPIR-V compatible replacement for PTX predicates
    Conditional(BrachCondition),
    LoadVar(LoadVarDetails),
    StoreVar(StoreVarDetails),
    Conversion(ImplicitConversion),
    Constant(ConstantDefinition),
    RetValue(ast::RetData, SpirvWord),
    PtrAccess(PtrAccess<P>),
    RepackVector(RepackVectorDetails),
    FunctionPointer(FunctionPointerDetails),
    VectorAccess(VectorAccess),
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
            Statement::LoadVar(LoadVarDetails {
                arg,
                typ,
                member_index,
            }) => {
                let dst = visitor.visit_ident(
                    arg.dst,
                    Some((&typ, ast::StateSpace::Reg)),
                    true,
                    false,
                )?;
                let src = visitor.visit_ident(
                    arg.src,
                    Some((&typ, ast::StateSpace::Local)),
                    false,
                    false,
                )?;
                Statement::LoadVar(LoadVarDetails {
                    arg: ast::LdArgs { dst, src },
                    typ,
                    member_index,
                })
            }
            Statement::StoreVar(StoreVarDetails {
                arg,
                typ,
                member_index,
            }) => {
                let src1 = visitor.visit_ident(
                    arg.src1,
                    Some((&typ, ast::StateSpace::Local)),
                    false,
                    false,
                )?;
                let src2 = visitor.visit_ident(
                    arg.src2,
                    Some((&typ, ast::StateSpace::Reg)),
                    false,
                    false,
                )?;
                Statement::StoreVar(StoreVarDetails {
                    arg: ast::StArgs { src1, src2 },
                    typ,
                    member_index,
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
                // TODO:
                // We should report type here
                let value = visitor.visit_ident(value, None, false, false)?;
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
            Statement::VectorAccess(VectorAccess {
                scalar_type,
                vector_width,
                dst,
                src: vector_src,
                member,
            }) => {
                let dst: SpirvWord = visitor.visit_ident(
                    dst,
                    Some((&scalar_type.into(), ast::StateSpace::Reg)),
                    true,
                    false,
                )?;
                let src = visitor.visit_ident(
                    vector_src,
                    Some((
                        &ast::Type::Vector(vector_width, scalar_type),
                        ast::StateSpace::Reg,
                    )),
                    false,
                    false,
                )?;
                Statement::VectorAccess(VectorAccess {
                    scalar_type,
                    vector_width,
                    dst,
                    src,
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
        })
    }
}

struct BrachCondition {
    predicate: SpirvWord,
    if_true: SpirvWord,
    if_false: SpirvWord,
}
struct LoadVarDetails {
    arg: ast::LdArgs<SpirvWord>,
    typ: ast::Type,
    // (index, vector_width)
    // HACK ALERT
    // For some reason IGC explodes when you try to load from builtin vectors
    // using OpInBoundsAccessChain, the one true way to do it is to
    // OpLoad+OpCompositeExtract
    member_index: Option<(u8, Option<u8>)>,
}

struct StoreVarDetails {
    arg: ast::StArgs<SpirvWord>,
    typ: ast::Type,
    member_index: Option<u8>,
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

#[derive(PartialEq, Clone)]
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

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct SpirvWord(spirv::Word);

impl From<spirv::Word> for SpirvWord {
    fn from(value: spirv::Word) -> Self {
        Self(value)
    }
}
impl From<SpirvWord> for spirv::Word {
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

fn pred_map_variable<U, T, F: FnMut(T) -> Result<U, TranslateError>>(
    this: ast::PredAt<T>,
    f: &mut F,
) -> Result<ast::PredAt<U>, TranslateError> {
    let new_label = f(this.label)?;
    Ok(ast::PredAt {
        not: this.not,
        label: new_label,
    })
}

pub(crate) enum Directive<'input> {
    Variable(ast::LinkingDirective, ast::Variable<SpirvWord>),
    Method(Function<'input>),
}

pub(crate) struct Function<'input> {
    pub func_decl: Rc<RefCell<ast::MethodDeclaration<'input, SpirvWord>>>,
    pub globals: Vec<ast::Variable<SpirvWord>>,
    pub body: Option<Vec<ExpandedStatement>>,
    import_as: Option<String>,
    tuning: Vec<ast::TuningDirective>,
    linkage: ast::LinkingDirective,
}

type ExpandedStatement = Statement<ast::Instruction<SpirvWord>, SpirvWord>;

type NormalizedStatement = Statement<
    (
        Option<ast::PredAt<SpirvWord>>,
        ast::Instruction<ast::ParsedOperand<SpirvWord>>,
    ),
    ast::ParsedOperand<SpirvWord>,
>;

type UnconditionalStatement =
    Statement<ast::Instruction<ast::ParsedOperand<SpirvWord>>, ast::ParsedOperand<SpirvWord>>;

type TypedStatement = Statement<ast::Instruction<TypedOperand>, TypedOperand>;

#[derive(Copy, Clone)]
enum TypedOperand {
    Reg(SpirvWord),
    RegOffset(SpirvWord, i32),
    Imm(ast::ImmediateValue),
    VecMember(SpirvWord, u8),
}

impl TypedOperand {
    fn map<Err>(
        self,
        fn_: impl FnOnce(SpirvWord, Option<u8>) -> Result<SpirvWord, Err>,
    ) -> Result<Self, Err> {
        Ok(match self {
            TypedOperand::Reg(reg) => TypedOperand::Reg(fn_(reg, None)?),
            TypedOperand::RegOffset(reg, off) => TypedOperand::RegOffset(fn_(reg, None)?, off),
            TypedOperand::Imm(imm) => TypedOperand::Imm(imm),
            TypedOperand::VecMember(reg, idx) => TypedOperand::VecMember(fn_(reg, Some(idx))?, idx),
        })
    }

    fn underlying_register(&self) -> Option<SpirvWord> {
        match self {
            Self::Reg(r) | Self::RegOffset(r, _) | Self::VecMember(r, _) => Some(*r),
            Self::Imm(_) => None,
        }
    }

    fn unwrap_reg(&self) -> Result<SpirvWord, TranslateError> {
        match self {
            TypedOperand::Reg(reg) => Ok(*reg),
            _ => Err(error_unreachable()),
        }
    }
}

impl ast::Operand for TypedOperand {
    type Ident = SpirvWord;

    fn from_ident(ident: Self::Ident) -> Self {
        TypedOperand::Reg(ident)
    }
}

impl<Fn> ast::VisitorMap<TypedOperand, TypedOperand, TranslateError>
    for FnVisitor<TypedOperand, TypedOperand, TranslateError, Fn>
where
    Fn: FnMut(
        TypedOperand,
        Option<(&ast::Type, ast::StateSpace)>,
        bool,
        bool,
    ) -> Result<TypedOperand, TranslateError>,
{
    fn visit(
        &mut self,
        args: TypedOperand,
        type_space: Option<(&ast::Type, ast::StateSpace)>,
        is_dst: bool,
        relaxed_type_check: bool,
    ) -> Result<TypedOperand, TranslateError> {
        (self.fn_)(args, type_space, is_dst, relaxed_type_check)
    }

    fn visit_ident(
        &mut self,
        args: SpirvWord,
        type_space: Option<(&ast::Type, ast::StateSpace)>,
        is_dst: bool,
        relaxed_type_check: bool,
    ) -> Result<SpirvWord, TranslateError> {
        match (self.fn_)(
            TypedOperand::Reg(args),
            type_space,
            is_dst,
            relaxed_type_check,
        )? {
            TypedOperand::Reg(reg) => Ok(reg),
            _ => Err(TranslateError::Unreachable),
        }
    }
}

struct FnVisitor<
    T,
    U,
    Err,
    Fn: FnMut(T, Option<(&ast::Type, ast::StateSpace)>, bool, bool) -> Result<U, Err>,
> {
    fn_: Fn,
    _marker: PhantomData<fn(T) -> Result<U, Err>>,
}

impl<
        T,
        U,
        Err,
        Fn: FnMut(T, Option<(&ast::Type, ast::StateSpace)>, bool, bool) -> Result<U, Err>,
    > FnVisitor<T, U, Err, Fn>
{
    fn new(fn_: Fn) -> Self {
        Self {
            fn_,
            _marker: PhantomData,
        }
    }
}

fn register_external_fn_call<'a>(
    id_defs: &mut NumericIdResolver,
    ptx_impl_imports: &mut HashMap<String, Directive>,
    name: String,
    return_arguments: impl Iterator<Item = (&'a ast::Type, ast::StateSpace)>,
    input_arguments: impl Iterator<Item = (&'a ast::Type, ast::StateSpace)>,
) -> Result<SpirvWord, TranslateError> {
    match ptx_impl_imports.entry(name) {
        hash_map::Entry::Vacant(entry) => {
            let fn_id = id_defs.register_intermediate(None);
            let return_arguments = fn_arguments_to_variables(id_defs, return_arguments);
            let input_arguments = fn_arguments_to_variables(id_defs, input_arguments);
            let func_decl = ast::MethodDeclaration::<SpirvWord> {
                return_arguments,
                name: ast::MethodName::Func(fn_id),
                input_arguments,
                shared_mem: None,
            };
            let func = Function {
                func_decl: Rc::new(RefCell::new(func_decl)),
                globals: Vec::new(),
                body: None,
                import_as: Some(entry.key().clone()),
                tuning: Vec::new(),
                linkage: ast::LinkingDirective::EXTERN,
            };
            entry.insert(Directive::Method(func));
            Ok(fn_id)
        }
        hash_map::Entry::Occupied(entry) => match entry.get() {
            Directive::Method(Function { func_decl, .. }) => match (**func_decl).borrow().name {
                ast::MethodName::Func(fn_id) => Ok(fn_id),
                ast::MethodName::Kernel(_) => Err(error_unreachable()),
            },
            _ => Err(error_unreachable()),
        },
    }
}

fn fn_arguments_to_variables<'a>(
    id_defs: &mut NumericIdResolver,
    args: impl Iterator<Item = (&'a ast::Type, ast::StateSpace)>,
) -> Vec<ast::Variable<SpirvWord>> {
    args.map(|(typ, space)| ast::Variable {
        align: None,
        v_type: typ.clone(),
        state_space: space,
        name: id_defs.register_intermediate(None),
        array_init: Vec::new(),
    })
    .collect::<Vec<_>>()
}

fn hoist_function_globals(directives: Vec<Directive>) -> Vec<Directive> {
    let mut result = Vec::with_capacity(directives.len());
    for directive in directives {
        match directive {
            Directive::Method(method) => {
                for variable in method.globals {
                    result.push(Directive::Variable(ast::LinkingDirective::NONE, variable));
                }
                result.push(Directive::Method(Function {
                    globals: Vec::new(),
                    ..method
                }))
            }
            _ => result.push(directive),
        }
    }
    result
}

struct MethodsCallMap<'input> {
    map: HashMap<ast::MethodName<'input, SpirvWord>, HashSet<SpirvWord>>,
}

impl<'input> MethodsCallMap<'input> {
    fn new(module: &[Directive<'input>]) -> Self {
        let mut directly_called_by = HashMap::new();
        for directive in module {
            match directive {
                Directive::Method(Function {
                    func_decl,
                    body: Some(statements),
                    ..
                }) => {
                    let call_key: ast::MethodName<_> = (**func_decl).borrow().name;
                    if let hash_map::Entry::Vacant(entry) = directly_called_by.entry(call_key) {
                        entry.insert(Vec::new());
                    }
                    for statement in statements {
                        match statement {
                            Statement::Instruction(ast::Instruction::Call { data, arguments }) => {
                                multi_hash_map_append(
                                    &mut directly_called_by,
                                    call_key,
                                    arguments.func,
                                );
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        let mut result = HashMap::new();
        for (&method_key, children) in directly_called_by.iter() {
            let mut visited = HashSet::new();
            for child in children {
                Self::add_call_map_single(&directly_called_by, &mut visited, *child);
            }
            result.insert(method_key, visited);
        }
        MethodsCallMap { map: result }
    }

    fn add_call_map_single(
        directly_called_by: &HashMap<ast::MethodName<'input, SpirvWord>, Vec<SpirvWord>>,
        visited: &mut HashSet<SpirvWord>,
        current: SpirvWord,
    ) {
        if !visited.insert(current) {
            return;
        }
        if let Some(children) = directly_called_by.get(&ast::MethodName::Func(current)) {
            for child in children {
                Self::add_call_map_single(directly_called_by, visited, *child);
            }
        }
    }

    fn get_kernel_children(&self, name: &'input str) -> impl Iterator<Item = &SpirvWord> {
        self.map
            .get(&ast::MethodName::Kernel(name))
            .into_iter()
            .flatten()
    }

    fn kernels(&self) -> impl Iterator<Item = (&'input str, &HashSet<SpirvWord>)> {
        self.map
            .iter()
            .filter_map(|(method, children)| match method {
                ast::MethodName::Kernel(kernel) => Some((*kernel, children)),
                ast::MethodName::Func(..) => None,
            })
    }

    fn methods(
        &self,
    ) -> impl Iterator<Item = (ast::MethodName<'input, SpirvWord>, &HashSet<SpirvWord>)> {
        self.map
            .iter()
            .map(|(method, children)| (*method, children))
    }

    fn visit_callees(&self, method: ast::MethodName<'input, SpirvWord>, f: impl FnMut(SpirvWord)) {
        self.map
            .get(&method)
            .into_iter()
            .flatten()
            .copied()
            .for_each(f);
    }
}

fn multi_hash_map_append<
    K: Eq + std::hash::Hash,
    V,
    Collection: std::iter::Extend<V> + std::default::Default,
>(
    m: &mut HashMap<K, Collection>,
    key: K,
    value: V,
) {
    match m.entry(key) {
        hash_map::Entry::Occupied(mut entry) => {
            entry.get_mut().extend(iter::once(value));
        }
        hash_map::Entry::Vacant(entry) => {
            entry.insert(Default::default()).extend(iter::once(value));
        }
    }
}

fn normalize_variable_decls(directives: &mut Vec<Directive>) {
    for directive in directives {
        match directive {
            Directive::Method(Function {
                body: Some(func), ..
            }) => {
                func[1..].sort_by_key(|s| match s {
                    Statement::Variable(_) => 0,
                    _ => 1,
                });
            }
            _ => (),
        }
    }
}

// HACK ALERT!
// This function is a "good enough" heuristic of whetever to mark f16/f32 operations
// in the kernel as flushing denorms to zero or preserving them
// PTX support per-instruction ftz information. Unfortunately SPIR-V has no
// such capability, so instead we guesstimate which use is more common in the kernel
// and emit suitable execution mode
fn compute_denorm_information<'input>(
    module: &[Directive<'input>],
) -> HashMap<ast::MethodName<'input, SpirvWord>, HashMap<u8, (spirv::FPDenormMode, isize)>> {
    let mut denorm_methods = HashMap::new();
    for directive in module {
        match directive {
            Directive::Variable(..) | Directive::Method(Function { body: None, .. }) => {}
            Directive::Method(Function {
                func_decl,
                body: Some(statements),
                ..
            }) => {
                let mut flush_counter = DenormCountMap::new();
                let method_key = (**func_decl).borrow().name;
                for statement in statements {
                    match statement {
                        Statement::Instruction(inst) => {
                            if let Some((flush, width)) = flush_to_zero(inst) {
                                denorm_count_map_update(&mut flush_counter, width, flush);
                            }
                        }
                        Statement::LoadVar(..) => {}
                        Statement::StoreVar(..) => {}
                        Statement::Conditional(_) => {}
                        Statement::Conversion(_) => {}
                        Statement::Constant(_) => {}
                        Statement::RetValue(_, _) => {}
                        Statement::Label(_) => {}
                        Statement::Variable(_) => {}
                        Statement::PtrAccess { .. } => {}
                        Statement::VectorAccess { .. } => {}
                        Statement::RepackVector(_) => {}
                        Statement::FunctionPointer(_) => {}
                    }
                }
                denorm_methods.insert(method_key, flush_counter);
            }
        }
    }
    denorm_methods
        .into_iter()
        .map(|(name, v)| {
            let width_to_denorm = v
                .into_iter()
                .map(|(k, flush_over_preserve)| {
                    let mode = if flush_over_preserve > 0 {
                        spirv::FPDenormMode::FlushToZero
                    } else {
                        spirv::FPDenormMode::Preserve
                    };
                    (k, (mode, flush_over_preserve))
                })
                .collect();
            (name, width_to_denorm)
        })
        .collect()
}

fn flush_to_zero(this: &ast::Instruction<SpirvWord>) -> Option<(bool, u8)> {
    match this {
        ast::Instruction::Ld { .. } => None,
        ast::Instruction::St { .. } => None,
        ast::Instruction::Mov { .. } => None,
        ast::Instruction::Not { .. } => None,
        ast::Instruction::Bra { .. } => None,
        ast::Instruction::Shl { .. } => None,
        ast::Instruction::Shr { .. } => None,
        ast::Instruction::Ret { .. } => None,
        ast::Instruction::Call { .. } => None,
        ast::Instruction::Or { .. } => None,
        ast::Instruction::And { .. } => None,
        ast::Instruction::Cvta { .. } => None,
        ast::Instruction::Selp { .. } => None,
        ast::Instruction::Bar { .. } => None,
        ast::Instruction::Atom { .. } => None,
        ast::Instruction::AtomCas { .. } => None,
        ast::Instruction::Sub {
            data: ast::ArithDetails::Integer(_),
            ..
        } => None,
        ast::Instruction::Add {
            data: ast::ArithDetails::Integer(_),
            ..
        } => None,
        ast::Instruction::Mul {
            data: ast::MulDetails::Integer { .. },
            ..
        } => None,
        ast::Instruction::Mad {
            data: ast::MadDetails::Integer { .. },
            ..
        } => None,
        ast::Instruction::Min {
            data: ast::MinMaxDetails::Signed(_),
            ..
        } => None,
        ast::Instruction::Min {
            data: ast::MinMaxDetails::Unsigned(_),
            ..
        } => None,
        ast::Instruction::Max {
            data: ast::MinMaxDetails::Signed(_),
            ..
        } => None,
        ast::Instruction::Max {
            data: ast::MinMaxDetails::Unsigned(_),
            ..
        } => None,
        ast::Instruction::Cvt {
            data:
                ast::CvtDetails {
                    mode:
                        ast::CvtMode::ZeroExtend
                        | ast::CvtMode::SignExtend
                        | ast::CvtMode::Truncate
                        | ast::CvtMode::Bitcast
                        | ast::CvtMode::SaturateUnsignedToSigned
                        | ast::CvtMode::SaturateSignedToUnsigned
                        | ast::CvtMode::FPFromSigned(_)
                        | ast::CvtMode::FPFromUnsigned(_),
                    ..
                },
            ..
        } => None,
        ast::Instruction::Div {
            data: ast::DivDetails::Unsigned(_),
            ..
        } => None,
        ast::Instruction::Div {
            data: ast::DivDetails::Signed(_),
            ..
        } => None,
        ast::Instruction::Clz { .. } => None,
        ast::Instruction::Brev { .. } => None,
        ast::Instruction::Popc { .. } => None,
        ast::Instruction::Xor { .. } => None,
        ast::Instruction::Bfe { .. } => None,
        ast::Instruction::Bfi { .. } => None,
        ast::Instruction::Rem { .. } => None,
        ast::Instruction::Prmt { .. } => None,
        ast::Instruction::Activemask { .. } => None,
        ast::Instruction::Membar { .. } => None,
        ast::Instruction::Sub {
            data: ast::ArithDetails::Float(float_control),
            ..
        }
        | ast::Instruction::Add {
            data: ast::ArithDetails::Float(float_control),
            ..
        }
        | ast::Instruction::Mul {
            data: ast::MulDetails::Float(float_control),
            ..
        }
        | ast::Instruction::Mad {
            data: ast::MadDetails::Float(float_control),
            ..
        } => float_control
            .flush_to_zero
            .map(|ftz| (ftz, float_control.type_.size_of())),
        ast::Instruction::Fma { data, .. } => {
            data.flush_to_zero.map(|ftz| (ftz, data.type_.size_of()))
        }
        ast::Instruction::Setp { data, .. } => {
            data.flush_to_zero.map(|ftz| (ftz, data.type_.size_of()))
        }
        ast::Instruction::SetpBool { data, .. } => data
            .base
            .flush_to_zero
            .map(|ftz| (ftz, data.base.type_.size_of())),
        ast::Instruction::Abs { data, .. }
        | ast::Instruction::Rsqrt { data, .. }
        | ast::Instruction::Neg { data, .. }
        | ast::Instruction::Ex2 { data, .. } => {
            data.flush_to_zero.map(|ftz| (ftz, data.type_.size_of()))
        }
        ast::Instruction::Min {
            data: ast::MinMaxDetails::Float(float_control),
            ..
        }
        | ast::Instruction::Max {
            data: ast::MinMaxDetails::Float(float_control),
            ..
        } => float_control
            .flush_to_zero
            .map(|ftz| (ftz, ast::ScalarType::from(float_control.type_).size_of())),
        ast::Instruction::Sqrt { data, .. } | ast::Instruction::Rcp { data, .. } => {
            data.flush_to_zero.map(|ftz| (ftz, data.type_.size_of()))
        }
        // Modifier .ftz can only be specified when either .dtype or .atype
        // is .f32 and applies only to single precision (.f32) inputs and results.
        ast::Instruction::Cvt {
            data:
                ast::CvtDetails {
                    mode:
                        ast::CvtMode::FPExtend { flush_to_zero }
                        | ast::CvtMode::FPTruncate { flush_to_zero, .. }
                        | ast::CvtMode::FPRound { flush_to_zero, .. }
                        | ast::CvtMode::SignedFromFP { flush_to_zero, .. }
                        | ast::CvtMode::UnsignedFromFP { flush_to_zero, .. },
                    ..
                },
            ..
        } => flush_to_zero.map(|ftz| (ftz, 4)),
        ast::Instruction::Div {
            data:
                ast::DivDetails::Float(ast::DivFloatDetails {
                    type_,
                    flush_to_zero,
                    ..
                }),
            ..
        } => flush_to_zero.map(|ftz| (ftz, type_.size_of())),
        ast::Instruction::Sin { data, .. }
        | ast::Instruction::Cos { data, .. }
        | ast::Instruction::Lg2 { data, .. } => {
            Some((data.flush_to_zero, mem::size_of::<f32>() as u8))
        }
        ptx_parser::Instruction::PrmtSlow { .. } => None,
        ptx_parser::Instruction::Trap {} => None,
    }
}

type DenormCountMap<T> = HashMap<T, isize>;

fn denorm_count_map_update<T: Eq + Hash>(map: &mut DenormCountMap<T>, key: T, value: bool) {
    let num_value = if value { 1 } else { -1 };
    denorm_count_map_update_impl(map, key, num_value);
}

fn denorm_count_map_update_impl<T: Eq + Hash>(
    map: &mut DenormCountMap<T>,
    key: T,
    num_value: isize,
) {
    match map.entry(key) {
        hash_map::Entry::Occupied(mut counter) => {
            *(counter.get_mut()) += num_value;
        }
        hash_map::Entry::Vacant(entry) => {
            entry.insert(num_value);
        }
    }
}

pub(crate) enum Directive2<'input, Instruction, Operand: ast::Operand> {
    Variable(ast::LinkingDirective, ast::Variable<SpirvWord>),
    Method(Function2<'input, Instruction, Operand>),
}

pub(crate) struct Function2<'input, Instruction, Operand: ast::Operand> {
    pub func_decl: ast::MethodDeclaration<'input, SpirvWord>,
    pub globals: Vec<ast::Variable<SpirvWord>>,
    pub body: Option<Vec<Statement<Instruction, Operand>>>,
    import_as: Option<String>,
    tuning: Vec<ast::TuningDirective>,
    linkage: ast::LinkingDirective,
}

type NormalizedDirective2<'input> = Directive2<
    'input,
    (
        Option<ast::PredAt<SpirvWord>>,
        ast::Instruction<ast::ParsedOperand<SpirvWord>>,
    ),
    ast::ParsedOperand<SpirvWord>,
>;

type NormalizedFunction2<'input> = Function2<
    'input,
    (
        Option<ast::PredAt<SpirvWord>>,
        ast::Instruction<ast::ParsedOperand<SpirvWord>>,
    ),
    ast::ParsedOperand<SpirvWord>,
>;

type UnconditionalDirective<'input> = Directive2<
    'input,
    ast::Instruction<ast::ParsedOperand<SpirvWord>>,
    ast::ParsedOperand<SpirvWord>,
>;

type UnconditionalFunction<'input> = Function2<
    'input,
    ast::Instruction<ast::ParsedOperand<SpirvWord>>,
    ast::ParsedOperand<SpirvWord>,
>;

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
            _ => Err(error_unknown_symbol()),
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
            return Err(error_unknown_symbol());
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
            .ok_or_else(|| error_unreachable())
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

    fn get_or_add(&mut self, current_id: &mut SpirvWord, reg: PtxSpecialRegister) -> SpirvWord {
        match self.reg_to_id.entry(reg) {
            hash_map::Entry::Occupied(e) => *e.get(),
            hash_map::Entry::Vacant(e) => {
                let numeric_id = SpirvWord(current_id.0);
                current_id.0 += 1;
                e.insert(numeric_id);
                self.id_to_reg.insert(numeric_id, reg);
                numeric_id
            }
        }
    }

    fn generate_declarations<'a, 'input>(
        resolver: &'a mut GlobalStringIdentResolver2<'input>,
    ) -> impl ExactSizeIterator<
        Item = (
            PtxSpecialRegister,
            ast::MethodDeclaration<'input, SpirvWord>,
        ),
    > + 'a {
        PtxSpecialRegister::iter().map(|sreg| {
            let external_fn_name = [ZLUDA_PTX_PREFIX, sreg.get_unprefixed_function_name()].concat();
            let name =
                ast::MethodName::Func(resolver.register_named(Cow::Owned(external_fn_name), None));
            let return_type = sreg.get_function_return_type();
            let input_type = sreg.get_function_return_type();
            (
                sreg,
                ast::MethodDeclaration {
                    return_arguments: vec![ast::Variable {
                        align: None,
                        v_type: return_type.into(),
                        state_space: ast::StateSpace::Reg,
                        name: resolver
                            .register_unnamed(Some((return_type.into(), ast::StateSpace::Reg))),
                        array_init: Vec::new(),
                    }],
                    name: name,
                    input_arguments: vec![ast::Variable {
                        align: None,
                        v_type: input_type.into(),
                        state_space: ast::StateSpace::Reg,
                        name: resolver
                            .register_unnamed(Some((input_type.into(), ast::StateSpace::Reg))),
                        array_init: Vec::new(),
                    }],
                    shared_mem: None,
                },
            )
        })
    }
}

pub struct VectorAccess {
    scalar_type: ast::ScalarType,
    vector_width: u8,
    dst: SpirvWord,
    src: SpirvWord,
    member: u8,
}
