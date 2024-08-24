use ptx_parser as ast;
use rspirv::{binary::Assemble, dr};
use std::{
    borrow::Cow,
    cell::RefCell,
    collections::{hash_map, HashMap},
    ffi::CString,
    rc::Rc,
};

mod convert_to_typed;
mod normalize_identifiers;
mod normalize_predicates;

static ZLUDA_PTX_IMPL_INTEL: &'static [u8] = include_bytes!("../../lib/zluda_ptx_impl.spv");
static ZLUDA_PTX_IMPL_AMD: &'static [u8] = include_bytes!("../../lib/zluda_ptx_impl.bc");
const ZLUDA_PTX_PREFIX: &'static str = "__zluda_ptx_impl__";

pub fn to_spirv_module<'input>(ast: ast::Module<'input>) -> Result<Module, TranslateError> {
    let mut id_defs = GlobalStringIdResolver::<'input>::new(SpirvWord(1));
    let mut ptx_impl_imports = HashMap::new();
    let directives = ast
        .directives
        .into_iter()
        .filter_map(|directive| {
            translate_directive(&mut id_defs, &mut ptx_impl_imports, directive).transpose()
        })
        .collect::<Result<Vec<_>, _>>()?;
    /*
    let directives = hoist_function_globals(directives);
    let must_link_ptx_impl = ptx_impl_imports.len() > 0;
    let mut directives = ptx_impl_imports
        .into_iter()
        .map(|(_, v)| v)
        .chain(directives.into_iter())
        .collect::<Vec<_>>();
    let mut builder = dr::Builder::new();
    builder.reserve_ids(id_defs.current_id());
    let call_map = MethodsCallMap::new(&directives);
    let mut directives =
        convert_dynamic_shared_memory_usage(directives, &call_map, &mut || builder.id());
    normalize_variable_decls(&mut directives);
    let denorm_information = compute_denorm_information(&directives);
    // https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_logicallayout_a_logical_layout_of_a_module
    builder.set_version(1, 3);
    emit_capabilities(&mut builder);
    emit_extensions(&mut builder);
    let opencl_id = emit_opencl_import(&mut builder);
    emit_memory_model(&mut builder);
    let mut map = TypeWordMap::new(&mut builder);
    //emit_builtins(&mut builder, &mut map, &id_defs);
    let mut kernel_info = HashMap::new();
    let (build_options, should_flush_denorms) =
        emit_denorm_build_string(&call_map, &denorm_information);
    let (directives, globals_use_map) = get_globals_use_map(directives);
    emit_directives(
        &mut builder,
        &mut map,
        &id_defs,
        opencl_id,
        should_flush_denorms,
        &call_map,
        globals_use_map,
        directives,
        &mut kernel_info,
    )?;
    let spirv = builder.module();
    Ok(Module {
        spirv,
        kernel_info,
        should_link_ptx_impl: if must_link_ptx_impl {
            Some((ZLUDA_PTX_IMPL_INTEL, ZLUDA_PTX_IMPL_AMD))
        } else {
            None
        },
        build_options,
    })
     */
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
    todo!()
    /*
    let typed_statements =
        fix_special_registers2(ptx_impl_imports, typed_statements, &mut numeric_id_defs)?;
    let (func_decl, typed_statements) =
        convert_to_stateful_memory_access(func_decl, typed_statements, &mut numeric_id_defs)?;
    let ssa_statements = insert_mem_ssa_statements(
        typed_statements,
        &mut numeric_id_defs,
        &mut (*func_decl).borrow_mut(),
    )?;
    let mut numeric_id_defs = numeric_id_defs.finish();
    let expanded_statements = expand_arguments(ssa_statements, &mut numeric_id_defs)?;
    let expanded_statements =
        insert_implicit_conversions(expanded_statements, &mut numeric_id_defs)?;
    let mut numeric_id_defs = numeric_id_defs.unmut();
    let labeled_statements = normalize_labels(expanded_statements, &mut numeric_id_defs);
    let (f_body, globals) =
        extract_globals(labeled_statements, ptx_impl_imports, &mut numeric_id_defs)?;
    Ok(Function {
        func_decl: func_decl,
        globals: globals,
        body: Some(f_body),
        import_as: None,
        tuning,
        linkage,
    })
     */
}

pub struct Module {
    pub spirv: dr::Module,
    pub kernel_info: HashMap<String, KernelInfo>,
    pub should_link_ptx_impl: Option<(&'static [u8], &'static [u8])>,
    pub build_options: CString,
}

impl Module {
    pub fn assemble(&self) -> Vec<u32> {
        self.spirv.assemble()
    }
}

struct GlobalStringIdResolver<'input> {
    current_id: SpirvWord,
    variables: HashMap<Cow<'input, str>, SpirvWord>,
    reverse_variables: HashMap<SpirvWord, &'input str>,
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

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
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

    fn get_type(self) -> ast::Type {
        match self {
            PtxSpecialRegister::Tid
            | PtxSpecialRegister::Ntid
            | PtxSpecialRegister::Ctaid
            | PtxSpecialRegister::Nctaid => ast::Type::Vector(self.get_function_return_type(), 4),
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
                Some(x) => Ok((x.get_type(), ast::StateSpace::Sreg, true)),
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

fn error_unknown_symbol() -> TranslateError {
    TranslateError::UnknownSymbol
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

    /*
    fn resolve_in_spirv_repr(
        &self,
        call_inst: ast::CallInst<NormalizedArgParams>,
    ) -> Result<ResolvedCall<NormalizedArgParams>, TranslateError> {
        let func_decl = (*self.func_decl).borrow();
        let mut return_arguments = Vec::new();
        let mut input_arguments = call_inst
            .param_list
            .into_iter()
            .zip(func_decl.input_arguments.iter())
            .map(|(id, var)| (id, var.v_type.clone(), var.state_space))
            .collect::<Vec<_>>();
        let mut func_decl_return_iter = func_decl.return_arguments.iter();
        let mut func_decl_input_iter = func_decl.input_arguments[input_arguments.len()..].iter();
        for (idx, id) in call_inst.ret_params.iter().enumerate() {
            let stays_as_return = match self.return_param_args.get(idx) {
                Some(x) => *x,
                None => return Err(TranslateError::MismatchedType),
            };
            if stays_as_return {
                if let Some(var) = func_decl_return_iter.next() {
                    return_arguments.push((*id, var.v_type.clone(), var.state_space));
                } else {
                    return Err(TranslateError::MismatchedType);
                }
            } else {
                if let Some(var) = func_decl_input_iter.next() {
                    input_arguments.push((
                        ast::Operand::Reg(*id),
                        var.v_type.clone(),
                        var.state_space,
                    ));
                } else {
                    return Err(TranslateError::MismatchedType);
                }
            }
        }
        if return_arguments.len() != func_decl.return_arguments.len()
            || input_arguments.len() != func_decl.input_arguments.len()
        {
            return Err(TranslateError::MismatchedType);
        }
        Ok(ResolvedCall {
            return_arguments,
            input_arguments,
            uniform: call_inst.uniform,
            name: call_inst.func,
        })
    }
     */
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
}

struct BrachCondition {
    predicate: SpirvWord,
    if_true: SpirvWord,
    if_false: SpirvWord,
}
struct LoadVarDetails {
    arg: ast::LdArgs<SpirvWord>,
    typ: ast::Type,
    state_space: ast::StateSpace,
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
    relaxed_type_check: bool
}

struct FunctionPointerDetails {
    dst: SpirvWord,
    src: SpirvWord,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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

impl ast::Operand for TypedOperand {
    type Ident = SpirvWord;

    fn from_ident(ident: Self::Ident) -> Self {
        TypedOperand::Reg(ident)
    }
}
