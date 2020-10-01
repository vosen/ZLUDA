use crate::ast;
use rspirv::{binary::Disassemble, dr};
use std::collections::{hash_map, HashMap, HashSet};
use std::convert::TryInto;
use std::{borrow::Cow, iter, mem};

use rspirv::binary::Assemble;

quick_error! {
    #[derive(Debug)]
    pub enum TranslateError {
        UnknownSymbol {}
        UntypedSymbol {}
        MismatchedType {}
        Spirv (err: rspirv::dr::Error) {
            from()
            display("{}", err)
            cause(err)
        }
        Unreachable {}
        Todo {}
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum SpirvType {
    Base(SpirvScalarKey),
    Vector(SpirvScalarKey, u8),
    Array(SpirvScalarKey, u32),
    Pointer(Box<SpirvType>, spirv::StorageClass),
    Func(Option<Box<SpirvType>>, Vec<SpirvType>),
    Struct(Vec<SpirvScalarKey>),
}

impl SpirvType {
    fn new_pointer(t: ast::Type, sc: spirv::StorageClass) -> Self {
        let key = match t {
            ast::Type::Scalar(typ) => SpirvType::Base(SpirvScalarKey::from(typ)),
            ast::Type::Vector(typ, len) => SpirvType::Vector(SpirvScalarKey::from(typ), len),
            ast::Type::Array(typ, len) => SpirvType::Array(SpirvScalarKey::from(typ), len),
        };
        SpirvType::Pointer(Box::new(key), sc)
    }
}

impl From<ast::Type> for SpirvType {
    fn from(t: ast::Type) -> Self {
        match t {
            ast::Type::Scalar(t) => SpirvType::Base(t.into()),
            ast::Type::Vector(typ, len) => SpirvType::Vector(typ.into(), len),
            ast::Type::Array(t, len) => SpirvType::Array(t.into(), len),
        }
    }
}

impl From<ast::ScalarType> for SpirvType {
    fn from(t: ast::ScalarType) -> Self {
        SpirvType::Base(t.into())
    }
}

struct TypeWordMap {
    void: spirv::Word,
    complex: HashMap<SpirvType, spirv::Word>,
}

// SPIR-V integer type definitions are signless, more below:
// https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_unsignedsigned_a_unsigned_versus_signed_integers
// https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_validation_rules_for_kernel_a_href_capability_capabilities_a
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum SpirvScalarKey {
    B8,
    B16,
    B32,
    B64,
    F16,
    F32,
    F64,
    Pred,
    F16x2,
}

impl From<ast::ScalarType> for SpirvScalarKey {
    fn from(t: ast::ScalarType) -> Self {
        match t {
            ast::ScalarType::B8 | ast::ScalarType::U8 | ast::ScalarType::S8 => SpirvScalarKey::B8,
            ast::ScalarType::B16 | ast::ScalarType::U16 | ast::ScalarType::S16 => {
                SpirvScalarKey::B16
            }
            ast::ScalarType::B32 | ast::ScalarType::U32 | ast::ScalarType::S32 => {
                SpirvScalarKey::B32
            }
            ast::ScalarType::B64 | ast::ScalarType::U64 | ast::ScalarType::S64 => {
                SpirvScalarKey::B64
            }
            ast::ScalarType::F16 => SpirvScalarKey::F16,
            ast::ScalarType::F32 => SpirvScalarKey::F32,
            ast::ScalarType::F64 => SpirvScalarKey::F64,
            ast::ScalarType::F16x2 => SpirvScalarKey::F16x2,
            ast::ScalarType::Pred => SpirvScalarKey::Pred,
        }
    }
}

impl TypeWordMap {
    fn new(b: &mut dr::Builder) -> TypeWordMap {
        let void = b.type_void();
        TypeWordMap {
            void: void,
            complex: HashMap::<SpirvType, spirv::Word>::new(),
        }
    }

    fn void(&self) -> spirv::Word {
        self.void
    }

    fn get_or_add_scalar(&mut self, b: &mut dr::Builder, t: ast::ScalarType) -> spirv::Word {
        let key: SpirvScalarKey = t.into();
        self.get_or_add_spirv_scalar(b, key)
    }

    fn get_or_add_spirv_scalar(&mut self, b: &mut dr::Builder, key: SpirvScalarKey) -> spirv::Word {
        *self
            .complex
            .entry(SpirvType::Base(key))
            .or_insert_with(|| match key {
                SpirvScalarKey::B8 => b.type_int(8, 0),
                SpirvScalarKey::B16 => b.type_int(16, 0),
                SpirvScalarKey::B32 => b.type_int(32, 0),
                SpirvScalarKey::B64 => b.type_int(64, 0),
                SpirvScalarKey::F16 => b.type_float(16),
                SpirvScalarKey::F32 => b.type_float(32),
                SpirvScalarKey::F64 => b.type_float(64),
                SpirvScalarKey::Pred => b.type_bool(),
                SpirvScalarKey::F16x2 => todo!(),
            })
    }

    fn get_or_add(&mut self, b: &mut dr::Builder, t: SpirvType) -> spirv::Word {
        match t {
            SpirvType::Base(key) => self.get_or_add_spirv_scalar(b, key),
            SpirvType::Pointer(ref typ, storage) => {
                let base = self.get_or_add(b, *typ.clone());
                *self
                    .complex
                    .entry(t)
                    .or_insert_with(|| b.type_pointer(None, storage, base))
            }
            SpirvType::Vector(typ, len) => {
                let base = self.get_or_add_spirv_scalar(b, typ);
                *self
                    .complex
                    .entry(t)
                    .or_insert_with(|| b.type_vector(base, len as u32))
            }
            SpirvType::Array(typ, len) => {
                let base = self.get_or_add_spirv_scalar(b, typ);
                let u32_type = self.get_or_add_scalar(b, ast::ScalarType::U32);
                *self.complex.entry(t).or_insert_with(|| {
                    let len_word = b.constant_u32(u32_type, None, len);
                    b.type_array(base, len_word)
                })
            }
            SpirvType::Func(ref out_params, ref in_params) => {
                let out_t = match out_params {
                    Some(p) => self.get_or_add(b, *p.clone()),
                    None => self.void(),
                };
                let in_t = in_params
                    .iter()
                    .map(|t| self.get_or_add(b, t.clone()))
                    .collect::<Vec<_>>();
                *self
                    .complex
                    .entry(t)
                    .or_insert_with(|| b.type_function(out_t, in_t))
            }
            SpirvType::Struct(ref underlying) => {
                let underlying_ids = underlying
                    .iter()
                    .map(|t| self.get_or_add_spirv_scalar(b, *t))
                    .collect::<Vec<_>>();
                *self
                    .complex
                    .entry(t)
                    .or_insert_with(|| b.type_struct(underlying_ids))
            }
        }
    }

    fn get_or_add_fn(
        &mut self,
        b: &mut dr::Builder,
        mut out_params: impl ExactSizeIterator<Item = SpirvType>,
        in_params: impl ExactSizeIterator<Item = SpirvType>,
    ) -> (spirv::Word, spirv::Word) {
        let (out_args, out_spirv_type) = if out_params.len() == 0 {
            (None, self.void())
        } else if out_params.len() == 1 {
            let arg_as_key = out_params.next().unwrap();
            (
                Some(Box::new(arg_as_key.clone())),
                self.get_or_add(b, arg_as_key),
            )
        } else {
            todo!()
        };
        (
            out_spirv_type,
            self.get_or_add(b, SpirvType::Func(out_args, in_params.collect::<Vec<_>>())),
        )
    }
}

pub fn to_spirv_module<'a>(
    ast: ast::Module<'a>,
) -> Result<(dr::Module, HashMap<String, Vec<usize>>), TranslateError> {
    let mut id_defs = GlobalStringIdResolver::new(1);
    let ssa_functions = ast
        .functions
        .into_iter()
        .map(|f| to_ssa_function(&mut id_defs, f))
        .collect::<Result<Vec<_>, _>>()?;
    let mut builder = dr::Builder::new();
    builder.reserve_ids(id_defs.current_id());
    // https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_logicallayout_a_logical_layout_of_a_module
    builder.set_version(1, 3);
    emit_capabilities(&mut builder);
    emit_extensions(&mut builder);
    let opencl_id = emit_opencl_import(&mut builder);
    emit_memory_model(&mut builder);
    let mut map = TypeWordMap::new(&mut builder);
    emit_builtins(&mut builder, &mut map, &id_defs);
    let mut args_len = HashMap::new();
    for f in ssa_functions {
        let f_body = match f.body {
            Some(f) => f,
            None => continue,
        };
        emit_function_body_ops(&mut builder, &mut map, opencl_id, &f.globals)?;
        emit_function_header(
            &mut builder,
            &mut map,
            &id_defs,
            f.func_directive,
            &mut args_len,
        )?;
        emit_function_body_ops(&mut builder, &mut map, opencl_id, &f_body)?;
        builder.end_function()?;
    }
    Ok((builder.module(), args_len))
}

fn emit_builtins(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    id_defs: &GlobalStringIdResolver,
) {
    for (reg, id) in id_defs.special_registers.iter() {
        let result_type = map.get_or_add(
            builder,
            SpirvType::Pointer(
                Box::new(SpirvType::from(reg.get_type())),
                spirv::StorageClass::UniformConstant,
            ),
        );
        builder.variable(
            result_type,
            Some(*id),
            spirv::StorageClass::UniformConstant,
            None,
        );
        builder.decorate(
            *id,
            spirv::Decoration::BuiltIn,
            &[dr::Operand::BuiltIn(reg.get_builtin())],
        );
    }
}

fn emit_function_header<'a>(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    global: &GlobalStringIdResolver<'a>,
    func_directive: ast::MethodDecl<spirv::Word>,
    all_args_lens: &mut HashMap<String, Vec<usize>>,
) -> Result<(), TranslateError> {
    if let ast::MethodDecl::Kernel(name, args) = &func_directive {
        let args_lens = args.iter().map(|param| param.v_type.width()).collect();
        all_args_lens.insert(name.to_string(), args_lens);
    }
    let (ret_type, func_type) = get_function_type(builder, map, &func_directive);
    let fn_id = match func_directive {
        ast::MethodDecl::Kernel(name, _) => {
            let fn_id = global.get_id(name)?;
            let interface = global
                .special_registers
                .iter()
                .map(|(_, id)| *id)
                .collect::<Vec<_>>();
            builder.entry_point(spirv::ExecutionModel::Kernel, fn_id, name, interface);
            fn_id
        }
        ast::MethodDecl::Func(_, name, _) => name,
    };
    builder.begin_function(
        ret_type,
        Some(fn_id),
        spirv::FunctionControl::NONE,
        func_type,
    )?;
    func_directive.visit_args(&mut |arg| {
        let result_type = map.get_or_add(builder, ast::Type::from(arg.v_type).into());
        let inst = dr::Instruction::new(
            spirv::Op::FunctionParameter,
            Some(result_type),
            Some(arg.name),
            Vec::new(),
        );
        builder.function.as_mut().unwrap().parameters.push(inst);
    });
    Ok(())
}

pub fn to_spirv<'a>(
    ast: ast::Module<'a>,
) -> Result<(Vec<u32>, HashMap<String, Vec<usize>>), TranslateError> {
    let (module, all_args_lens) = to_spirv_module(ast)?;
    Ok((module.assemble(), all_args_lens))
}

fn emit_capabilities(builder: &mut dr::Builder) {
    builder.capability(spirv::Capability::GenericPointer);
    builder.capability(spirv::Capability::Linkage);
    builder.capability(spirv::Capability::Addresses);
    builder.capability(spirv::Capability::Kernel);
    builder.capability(spirv::Capability::Int8);
    builder.capability(spirv::Capability::Int16);
    builder.capability(spirv::Capability::Int64);
    builder.capability(spirv::Capability::Float16);
    builder.capability(spirv::Capability::Float64);
}

fn emit_extensions(_: &mut dr::Builder) {}

fn emit_opencl_import(builder: &mut dr::Builder) -> spirv::Word {
    builder.ext_inst_import("OpenCL.std")
}

fn emit_memory_model(builder: &mut dr::Builder) {
    builder.memory_model(
        spirv::AddressingModel::Physical64,
        spirv::MemoryModel::OpenCL,
    );
}

fn to_ssa_function<'a>(
    id_defs: &mut GlobalStringIdResolver<'a>,
    f: ast::ParsedFunction<'a>,
) -> Result<Function<'a>, TranslateError> {
    let (str_resolver, fn_resolver, fn_decl) = id_defs.start_fn(&f.func_directive);
    to_ssa(str_resolver, fn_resolver, fn_decl, f.body)
}

fn expand_kernel_params<'a, 'b>(
    fn_resolver: &mut FnStringIdResolver<'a, 'b>,
    args: impl Iterator<Item = &'b ast::KernelArgument<&'a str>>,
) -> Vec<ast::KernelArgument<spirv::Word>> {
    args.map(|a| ast::KernelArgument {
        name: fn_resolver.add_def(a.name, Some((StateSpace::Param, ast::Type::from(a.v_type)))),
        v_type: a.v_type,
        align: a.align,
    })
    .collect()
}

fn expand_fn_params<'a, 'b>(
    fn_resolver: &mut FnStringIdResolver<'a, 'b>,
    args: impl Iterator<Item = &'b ast::FnArgument<&'a str>>,
) -> Vec<ast::FnArgument<spirv::Word>> {
    args.map(|a| {
        let ss = match a.v_type {
            ast::FnArgumentType::Reg(_) => StateSpace::Reg,
            ast::FnArgumentType::Param(_) => StateSpace::Param,
        };
        ast::FnArgument {
            name: fn_resolver.add_def(a.name, Some((ss, ast::Type::from(a.v_type)))),
            v_type: a.v_type,
            align: a.align,
        }
    })
    .collect()
}

fn to_ssa<'input, 'b>(
    mut id_defs: FnStringIdResolver<'input, 'b>,
    fn_defs: GlobalFnDeclResolver<'input, 'b>,
    f_args: ast::MethodDecl<'input, spirv::Word>,
    f_body: Option<Vec<ast::Statement<ast::ParsedArgParams<'input>>>>,
) -> Result<Function<'input>, TranslateError> {
    let f_body = match f_body {
        Some(vec) => vec,
        None => {
            return Ok(Function {
                func_directive: f_args,
                body: None,
                globals: Vec::new(),
            })
        }
    };
    let normalized_ids = normalize_identifiers(&mut id_defs, &fn_defs, f_body)?;
    let mut numeric_id_defs = id_defs.finish();
    let unadorned_statements = normalize_predicates(normalized_ids, &mut numeric_id_defs);
    let typed_statements =
        convert_to_typed_statements(unadorned_statements, &fn_defs, &numeric_id_defs)?;
    let mut numeric_id_defs = numeric_id_defs.finish();
    let (f_args, ssa_statements) =
        insert_mem_ssa_statements(typed_statements, &mut numeric_id_defs, f_args)?;
    let expanded_statements = expand_arguments(ssa_statements, &mut numeric_id_defs)?;
    let expanded_statements =
        insert_implicit_conversions(expanded_statements, &mut numeric_id_defs)?;
    let mut numeric_id_defs = numeric_id_defs.unmut();
    let labeled_statements = normalize_labels(expanded_statements, &mut numeric_id_defs);
    let sorted_statements = normalize_variable_decls(labeled_statements);
    let (f_body, globals) = extract_globals(sorted_statements);
    Ok(Function {
        func_directive: f_args,
        globals: globals,
        body: Some(f_body),
    })
}

fn extract_globals(
    sorted_statements: Vec<ExpandedStatement>,
) -> (Vec<ExpandedStatement>, Vec<ExpandedStatement>) {
    // This fn will be used for SLM
    (sorted_statements, Vec::new())
}

fn normalize_variable_decls(mut func: Vec<ExpandedStatement>) -> Vec<ExpandedStatement> {
    func[1..].sort_by_key(|s| match s {
        Statement::Variable(_) => 0,
        _ => 1,
    });
    func
}

fn convert_to_typed_statements(
    func: Vec<UnconditionalStatement>,
    fn_defs: &GlobalFnDeclResolver,
    id_defs: &NumericIdResolver,
) -> Result<Vec<TypedStatement>, TranslateError> {
    let mut result = Vec::<TypedStatement>::with_capacity(func.len());
    for s in func {
        match s {
            Statement::Instruction(inst) => match inst {
                ast::Instruction::Call(call) => {
                    // TODO: error out if lengths don't match
                    let fn_def = fn_defs.get_fn_decl(call.func)?;
                    let ret_params = to_resolved_fn_args(call.ret_params, &*fn_def.ret_vals);
                    let param_list = to_resolved_fn_args(call.param_list, &*fn_def.params);
                    let resolved_call = ResolvedCall {
                        uniform: call.uniform,
                        ret_params,
                        func: call.func,
                        param_list,
                    };
                    result.push(Statement::Call(resolved_call));
                }
                // Supported ld/st:
                // global: only compatible with reg b64/u64/s64 source/dest
                // generic: compatible with global/local sources
                // param: compiled as mov
                // local compiled as mov
                // We would like to convert ld/st local/param to movs here,
                // but they have different semantics for implicit conversions
                // For now, we convert generic ld from local params to ld.local.
                // This way, we can rely on further stages of the compilation on
                // ld.generic & ld.global having bytes address source
                // One complication: immediate address is only allowed in local,
                // It is not supported in generic ld
                //     ld.local foo, [1];
                ast::Instruction::Ld(mut d, arg) => {
                    match arg.src.underlying() {
                        None => {}
                        Some(u) => {
                            let (ss, _) = id_defs.get_typed(*u)?;
                            match (d.state_space, ss) {
                                (ast::LdStateSpace::Generic, StateSpace::Local) => {
                                    d.state_space = ast::LdStateSpace::Local;
                                }
                                _ => {}
                            };
                        }
                    };
                    result.push(Statement::Instruction(ast::Instruction::Ld(d, arg.cast())));
                }
                ast::Instruction::St(mut d, arg) => {
                    match arg.src1.underlying() {
                        None => {}
                        Some(u) => {
                            let (ss, _) = id_defs.get_typed(*u)?;
                            match (d.state_space, ss) {
                                (ast::StStateSpace::Generic, StateSpace::Local) => {
                                    d.state_space = ast::StStateSpace::Local;
                                }
                                _ => (),
                            };
                        }
                    };
                    result.push(Statement::Instruction(ast::Instruction::St(d, arg.cast())));
                }
                ast::Instruction::Mov(mut d, args) => match args {
                    ast::Arg2Mov::Normal(arg) => {
                        if let Some(src_id) = arg.src.single_underlying() {
                            let (scope, _) = id_defs.get_typed(*src_id)?;
                            d.src_is_address = match scope {
                                StateSpace::Reg => false,
                                StateSpace::Const
                                | StateSpace::Global
                                | StateSpace::Local
                                | StateSpace::Shared
                                | StateSpace::Param
                                | StateSpace::ParamReg => true,
                            };
                        }
                        result.push(Statement::Instruction(ast::Instruction::Mov(
                            d,
                            ast::Arg2Mov::Normal(arg.cast()),
                        )));
                    }
                    ast::Arg2Mov::Member(args) => {
                        if let Some(dst_typ) = args.vector_dst() {
                            match id_defs.get_typed(*dst_typ)? {
                                (_, ast::Type::Vector(_, len)) => {
                                    d.dst_width = len;
                                }
                                _ => return Err(TranslateError::MismatchedType),
                            }
                        };
                        if let Some((src_typ, _)) = args.vector_src() {
                            match id_defs.get_typed(*src_typ)? {
                                (_, ast::Type::Vector(_, len)) => {
                                    d.src_width = len;
                                }
                                _ => return Err(TranslateError::MismatchedType),
                            }
                        };
                        result.push(Statement::Instruction(ast::Instruction::Mov(
                            d,
                            ast::Arg2Mov::Member(args.cast()),
                        )));
                    }
                },
                ast::Instruction::Mul(d, a) => {
                    result.push(Statement::Instruction(ast::Instruction::Mul(d, a.cast())))
                }
                ast::Instruction::Add(d, a) => {
                    result.push(Statement::Instruction(ast::Instruction::Add(d, a.cast())))
                }
                ast::Instruction::Setp(d, a) => {
                    result.push(Statement::Instruction(ast::Instruction::Setp(d, a.cast())))
                }
                ast::Instruction::SetpBool(d, a) => result.push(Statement::Instruction(
                    ast::Instruction::SetpBool(d, a.cast()),
                )),
                ast::Instruction::Not(d, a) => {
                    result.push(Statement::Instruction(ast::Instruction::Not(d, a.cast())))
                }
                ast::Instruction::Bra(d, a) => {
                    result.push(Statement::Instruction(ast::Instruction::Bra(d, a.cast())))
                }
                ast::Instruction::Cvt(d, a) => {
                    result.push(Statement::Instruction(ast::Instruction::Cvt(d, a.cast())))
                }
                ast::Instruction::Cvta(d, a) => {
                    result.push(Statement::Instruction(ast::Instruction::Cvta(d, a.cast())))
                }
                ast::Instruction::Shl(d, a) => {
                    result.push(Statement::Instruction(ast::Instruction::Shl(d, a.cast())))
                }
                ast::Instruction::Ret(d) => {
                    result.push(Statement::Instruction(ast::Instruction::Ret(d)))
                }
                ast::Instruction::Abs(d, a) => {
                    result.push(Statement::Instruction(ast::Instruction::Abs(d, a.cast())))
                }
                ast::Instruction::Mad(d, a) => {
                    result.push(Statement::Instruction(ast::Instruction::Mad(d, a.cast())))
                }
                ast::Instruction::Shr(d, a) => {
                    result.push(Statement::Instruction(ast::Instruction::Shr(d, a.cast())))
                }
            },
            Statement::Label(i) => result.push(Statement::Label(i)),
            Statement::Variable(v) => result.push(Statement::Variable(v)),
            Statement::LoadVar(a, t) => result.push(Statement::LoadVar(a, t)),
            Statement::StoreVar(a, t) => result.push(Statement::StoreVar(a, t)),
            Statement::Call(c) => result.push(Statement::Call(c.cast())),
            Statement::Composite(c) => result.push(Statement::Composite(c)),
            Statement::Conditional(c) => result.push(Statement::Conditional(c)),
            Statement::Conversion(c) => result.push(Statement::Conversion(c)),
            Statement::Constant(c) => result.push(Statement::Constant(c)),
            Statement::RetValue(d, id) => result.push(Statement::RetValue(d, id)),
            Statement::Undef(_, _) => return Err(TranslateError::Unreachable),
        }
    }
    Ok(result)
}

fn to_resolved_fn_args<T>(
    params: Vec<T>,
    params_decl: &[ast::FnArgumentType],
) -> Vec<(T, ast::FnArgumentType)> {
    params
        .into_iter()
        .zip(params_decl.iter())
        .map(|(id, typ)| (id, *typ))
        .collect::<Vec<_>>()
}

fn normalize_labels(
    func: Vec<ExpandedStatement>,
    id_def: &mut NumericIdResolver,
) -> Vec<ExpandedStatement> {
    let mut labels_in_use = HashSet::new();
    for s in func.iter() {
        match s {
            Statement::Instruction(i) => {
                if let Some(target) = i.jump_target() {
                    labels_in_use.insert(target);
                }
            }
            Statement::Conditional(cond) => {
                labels_in_use.insert(cond.if_true);
                labels_in_use.insert(cond.if_false);
            }
            Statement::Composite(_)
            | Statement::Call(_)
            | Statement::Variable(_)
            | Statement::LoadVar(_, _)
            | Statement::StoreVar(_, _)
            | Statement::RetValue(_, _)
            | Statement::Conversion(_)
            | Statement::Constant(_)
            | Statement::Label(_)
            | Statement::Undef(_, _) => (),
        }
    }
    iter::once(Statement::Label(id_def.new_id(None)))
        .chain(func.into_iter().filter(|s| match s {
            Statement::Label(i) => labels_in_use.contains(i),
            _ => true,
        }))
        .collect::<Vec<_>>()
}

fn normalize_predicates(
    func: Vec<NormalizedStatement>,
    id_def: &mut NumericIdResolver,
) -> Vec<UnconditionalStatement> {
    let mut result = Vec::with_capacity(func.len());
    for s in func {
        match s {
            Statement::Label(id) => result.push(Statement::Label(id)),
            Statement::Instruction((pred, inst)) => {
                if let Some(pred) = pred {
                    let if_true = id_def.new_id(None);
                    let if_false = id_def.new_id(None);
                    let folded_bra = match &inst {
                        ast::Instruction::Bra(_, arg) => Some(arg.src),
                        _ => None,
                    };
                    let mut branch = BrachCondition {
                        predicate: pred.label,
                        if_true: folded_bra.unwrap_or(if_true),
                        if_false,
                    };
                    if pred.not {
                        std::mem::swap(&mut branch.if_true, &mut branch.if_false);
                    }
                    result.push(Statement::Conditional(branch));
                    if folded_bra.is_none() {
                        result.push(Statement::Label(if_true));
                        result.push(Statement::Instruction(inst));
                    }
                    result.push(Statement::Label(if_false));
                } else {
                    result.push(Statement::Instruction(inst));
                }
            }
            Statement::Variable(var) => result.push(Statement::Variable(var)),
            // Blocks are flattened when resolving ids
            _ => unreachable!(),
        }
    }
    result
}

fn insert_mem_ssa_statements<'a, 'b>(
    func: Vec<TypedStatement>,
    id_def: &mut MutableNumericIdResolver,
    mut f_args: ast::MethodDecl<'a, spirv::Word>,
) -> Result<(ast::MethodDecl<'a, spirv::Word>, Vec<TypedStatement>), TranslateError> {
    let mut result = Vec::with_capacity(func.len());
    let out_param = match &mut f_args {
        ast::MethodDecl::Kernel(_, in_params) => {
            for p in in_params.iter_mut() {
                let typ = ast::Type::from(p.v_type);
                let new_id = id_def.new_id(typ);
                result.push(Statement::Variable(ast::Variable {
                    align: p.align,
                    v_type: ast::VariableType::Param(p.v_type),
                    name: p.name,
                }));
                result.push(Statement::StoreVar(
                    ast::Arg2St {
                        src1: p.name,
                        src2: new_id,
                    },
                    typ,
                ));
                p.name = new_id;
            }
            None
        }
        ast::MethodDecl::Func(out_params, _, in_params) => {
            for p in in_params.iter_mut() {
                let typ = ast::Type::from(p.v_type);
                let new_id = id_def.new_id(typ);
                let var_typ = ast::VariableType::from(p.v_type);
                result.push(Statement::Variable(ast::Variable {
                    align: p.align,
                    v_type: var_typ,
                    name: p.name,
                }));
                result.push(Statement::StoreVar(
                    ast::Arg2St {
                        src1: p.name,
                        src2: new_id,
                    },
                    typ,
                ));
                p.name = new_id;
            }
            match &mut **out_params {
                [p] => {
                    result.push(Statement::Variable(ast::Variable {
                        align: p.align,
                        v_type: ast::VariableType::from(p.v_type),
                        name: p.name,
                    }));
                    Some(p.name)
                }
                [] => None,
                _ => todo!(),
            }
        }
    };
    for s in func {
        match s {
            Statement::Call(call) => {
                insert_mem_ssa_statement_default(id_def, &mut result, call.cast())?
            }
            Statement::Instruction(inst) => match inst {
                ast::Instruction::Ret(d) => {
                    if let Some(out_param) = out_param {
                        let typ = id_def.get_typed(out_param)?;
                        let new_id = id_def.new_id(typ);
                        result.push(Statement::LoadVar(
                            ast::Arg2 {
                                dst: new_id,
                                src: out_param,
                            },
                            typ,
                        ));
                        result.push(Statement::RetValue(d, new_id));
                    } else {
                        result.push(Statement::Instruction(ast::Instruction::Ret(d)))
                    }
                }
                inst => insert_mem_ssa_statement_default(id_def, &mut result, inst)?,
            },
            Statement::Conditional(mut bra) => {
                let generated_id = id_def.new_id(ast::Type::Scalar(ast::ScalarType::Pred));
                result.push(Statement::LoadVar(
                    Arg2 {
                        dst: generated_id,
                        src: bra.predicate,
                    },
                    ast::Type::Scalar(ast::ScalarType::Pred),
                ));
                bra.predicate = generated_id;
                result.push(Statement::Conditional(bra));
            }
            s @ Statement::Variable(_) | s @ Statement::Label(_) => result.push(s),
            Statement::LoadVar(_, _)
            | Statement::StoreVar(_, _)
            | Statement::Conversion(_)
            | Statement::RetValue(_, _)
            | Statement::Constant(_)
            | Statement::Undef(_, _) => {}
            Statement::Composite(_) => todo!(),
        }
    }
    Ok((f_args, result))
}

trait VisitVariable: Sized {
    fn visit_variable<
        'a,
        F: FnMut(
            ArgumentDescriptor<spirv::Word>,
            Option<ast::Type>,
        ) -> Result<spirv::Word, TranslateError>,
    >(
        self,
        f: &mut F,
    ) -> Result<TypedStatement, TranslateError>;
}
trait VisitVariableExpanded {
    fn visit_variable_extended<
        F: FnMut(
            ArgumentDescriptor<spirv::Word>,
            Option<ast::Type>,
        ) -> Result<spirv::Word, TranslateError>,
    >(
        self,
        f: &mut F,
    ) -> Result<ExpandedStatement, TranslateError>;
}

fn insert_mem_ssa_statement_default<'a, F: VisitVariable>(
    id_def: &mut MutableNumericIdResolver,
    result: &mut Vec<TypedStatement>,
    stmt: F,
) -> Result<(), TranslateError> {
    let mut post_statements = Vec::new();
    let new_statement =
        stmt.visit_variable(&mut |desc: ArgumentDescriptor<spirv::Word>, instr_type| {
            if instr_type.is_none() {
                return Ok(desc.op);
            }
            let id_type = match (id_def.get_typed(desc.op)?, desc.sema) {
                (_, ArgumentSemantics::Address) => return Ok(desc.op),
                (t, ArgumentSemantics::RegisterPointer)
                | (t, ArgumentSemantics::Default)
                | (t, ArgumentSemantics::DefaultRelaxed)
                | (t, ArgumentSemantics::PhysicalPointer) => t,
            };
            let generated_id = id_def.new_id(id_type);
            if !desc.is_dst {
                result.push(Statement::LoadVar(
                    Arg2 {
                        dst: generated_id,
                        src: desc.op,
                    },
                    id_type,
                ));
            } else {
                post_statements.push(Statement::StoreVar(
                    Arg2St {
                        src1: desc.op,
                        src2: generated_id,
                    },
                    id_type,
                ));
            }
            Ok(generated_id)
        })?;
    result.push(new_statement);
    result.append(&mut post_statements);
    Ok(())
}

fn expand_arguments<'a, 'b>(
    func: Vec<TypedStatement>,
    id_def: &'b mut MutableNumericIdResolver<'a>,
) -> Result<Vec<ExpandedStatement>, TranslateError> {
    let mut result = Vec::with_capacity(func.len());
    for s in func {
        match s {
            Statement::Call(call) => {
                let mut visitor = FlattenArguments::new(&mut result, id_def);
                let (new_call, post_stmts) = (call.map(&mut visitor)?, visitor.post_stmts);
                result.push(Statement::Call(new_call));
                result.extend(post_stmts);
            }
            Statement::Instruction(inst) => {
                let mut visitor = FlattenArguments::new(&mut result, id_def);
                let (new_inst, post_stmts) = (inst.map(&mut visitor)?, visitor.post_stmts);
                result.push(Statement::Instruction(new_inst));
                result.extend(post_stmts);
            }
            Statement::Variable(ast::Variable {
                align,
                v_type,
                name,
            }) => result.push(Statement::Variable(ast::Variable {
                align,
                v_type,
                name,
            })),
            Statement::Label(id) => result.push(Statement::Label(id)),
            Statement::Conditional(bra) => result.push(Statement::Conditional(bra)),
            Statement::LoadVar(arg, typ) => result.push(Statement::LoadVar(arg, typ)),
            Statement::StoreVar(arg, typ) => result.push(Statement::StoreVar(arg, typ)),
            Statement::RetValue(d, id) => result.push(Statement::RetValue(d, id)),
            Statement::Composite(_)
            | Statement::Conversion(_)
            | Statement::Constant(_)
            | Statement::Undef(_, _) => unreachable!(),
        }
    }
    Ok(result)
}

struct FlattenArguments<'a, 'b> {
    func: &'b mut Vec<ExpandedStatement>,
    id_def: &'b mut MutableNumericIdResolver<'a>,
    post_stmts: Vec<ExpandedStatement>,
}

impl<'a, 'b> FlattenArguments<'a, 'b> {
    fn new(
        func: &'b mut Vec<ExpandedStatement>,
        id_def: &'b mut MutableNumericIdResolver<'a>,
    ) -> Self {
        FlattenArguments {
            func,
            id_def,
            post_stmts: Vec::new(),
        }
    }

    fn insert_composite_read(
        func: &mut Vec<ExpandedStatement>,
        id_def: &mut MutableNumericIdResolver<'a>,
        typ: (ast::ScalarType, u8),
        scalar_dst: Option<spirv::Word>,
        scalar_sema_override: Option<ArgumentSemantics>,
        composite_src: (spirv::Word, u8),
    ) -> spirv::Word {
        let new_id = scalar_dst.unwrap_or_else(|| id_def.new_id(ast::Type::Scalar(typ.0)));
        func.push(Statement::Composite(CompositeRead {
            typ: typ.0,
            dst: new_id,
            dst_semantics_override: scalar_sema_override,
            src_composite: composite_src.0,
            src_index: composite_src.1 as u32,
            src_len: typ.1 as u32,
        }));
        new_id
    }

    fn reg(
        &mut self,
        desc: ArgumentDescriptor<spirv::Word>,
        _: Option<ast::Type>,
    ) -> Result<spirv::Word, TranslateError> {
        Ok(desc.op)
    }

    fn reg_offset(
        &mut self,
        desc: ArgumentDescriptor<(spirv::Word, i32)>,
        typ: ast::Type,
    ) -> Result<spirv::Word, TranslateError> {
        let (reg, offset) = desc.op;
        match desc.sema {
            ArgumentSemantics::Default | ArgumentSemantics::DefaultRelaxed => {
                let scalar_t = if let ast::Type::Scalar(scalar) = typ {
                    scalar
                } else {
                    todo!()
                };
                let id_constant_stmt = self.id_def.new_id(ast::Type::Scalar(scalar_t));
                let result_id = self.id_def.new_id(typ);
                self.func.push(Statement::Constant(ConstantDefinition {
                    dst: id_constant_stmt,
                    typ: scalar_t,
                    value: offset as i64,
                }));
                let int_type = ast::IntType::try_new(scalar_t).unwrap_or_else(|| todo!());
                self.func.push(Statement::Instruction(
                    ast::Instruction::<ExpandedArgParams>::Add(
                        ast::AddDetails::Int(ast::AddIntDesc {
                            typ: int_type,
                            saturate: false,
                        }),
                        ast::Arg3 {
                            dst: result_id,
                            src1: reg,
                            src2: id_constant_stmt,
                        },
                    ),
                ));
                Ok(result_id)
            }
            ArgumentSemantics::PhysicalPointer => {
                let scalar_t = ast::ScalarType::U64;
                let id_constant_stmt = self.id_def.new_id(ast::Type::Scalar(scalar_t));
                let result_id = self.id_def.new_id(ast::Type::Scalar(scalar_t));
                self.func.push(Statement::Constant(ConstantDefinition {
                    dst: id_constant_stmt,
                    typ: scalar_t,
                    value: offset as i64,
                }));
                let int_type = ast::IntType::U64;
                self.func.push(Statement::Instruction(
                    ast::Instruction::<ExpandedArgParams>::Add(
                        ast::AddDetails::Int(ast::AddIntDesc {
                            typ: int_type,
                            saturate: false,
                        }),
                        ast::Arg3 {
                            dst: result_id,
                            src1: reg,
                            src2: id_constant_stmt,
                        },
                    ),
                ));
                Ok(result_id)
            }
            ArgumentSemantics::RegisterPointer => {
                if offset == 0 {
                    return Ok(reg);
                }
                todo!()
            }
            ArgumentSemantics::Address => todo!(),
        }
    }

    fn immediate(
        &mut self,
        desc: ArgumentDescriptor<u32>,
        typ: ast::Type,
    ) -> Result<spirv::Word, TranslateError> {
        let scalar_t = if let ast::Type::Scalar(scalar) = typ {
            scalar
        } else {
            todo!()
        };
        let id = self.id_def.new_id(ast::Type::Scalar(scalar_t));
        self.func.push(Statement::Constant(ConstantDefinition {
            dst: id,
            typ: scalar_t,
            value: desc.op as i64,
        }));
        Ok(id)
    }

    fn member_src(
        &mut self,
        desc: ArgumentDescriptor<(spirv::Word, u8)>,
        typ: (ast::ScalarType, u8),
    ) -> Result<spirv::Word, TranslateError> {
        if desc.is_dst {
            return Err(TranslateError::Unreachable);
        }
        let new_id = Self::insert_composite_read(
            self.func,
            self.id_def,
            typ,
            None,
            Some(desc.sema),
            desc.op,
        );
        Ok(new_id)
    }

    fn vector(
        &mut self,
        desc: ArgumentDescriptor<&Vec<spirv::Word>>,
        typ: ast::Type,
    ) -> Result<spirv::Word, TranslateError> {
        let (scalar_type, vec_len) = typ.get_vector()?;
        if !desc.is_dst {
            let mut new_id = self.id_def.new_id(typ);
            self.func.push(Statement::Undef(typ, new_id));
            for (idx, id) in desc.op.iter().enumerate() {
                let newer_id = self.id_def.new_id(typ);
                self.func.push(Statement::Instruction(ast::Instruction::Mov(
                    ast::MovDetails {
                        typ: ast::Type::Scalar(scalar_type),
                        src_is_address: false,
                        dst_width: vec_len,
                        src_width: 0,
                        relaxed_src2_conv: desc.sema == ArgumentSemantics::DefaultRelaxed,
                    },
                    ast::Arg2Mov::Member(ast::Arg2MovMember::Dst(
                        (newer_id, idx as u8),
                        new_id,
                        *id,
                    )),
                )));
                new_id = newer_id;
            }
            Ok(new_id)
        } else {
            let new_id = self.id_def.new_id(typ);
            for (idx, id) in desc.op.iter().enumerate() {
                Self::insert_composite_read(
                    &mut self.post_stmts,
                    self.id_def,
                    (scalar_type, vec_len),
                    Some(*id),
                    Some(desc.sema),
                    (new_id, idx as u8),
                );
            }
            Ok(new_id)
        }
    }
}

impl<'a, 'b> ArgumentMapVisitor<TypedArgParams, ExpandedArgParams> for FlattenArguments<'a, 'b> {
    fn id(
        &mut self,
        desc: ArgumentDescriptor<spirv::Word>,
        t: Option<ast::Type>,
    ) -> Result<spirv::Word, TranslateError> {
        self.reg(desc, t)
    }

    fn operand(
        &mut self,
        desc: ArgumentDescriptor<ast::Operand<spirv::Word>>,
        typ: ast::Type,
    ) -> Result<spirv::Word, TranslateError> {
        match desc.op {
            ast::Operand::Reg(r) => self.reg(desc.new_op(r), Some(typ)),
            ast::Operand::Imm(x) => self.immediate(desc.new_op(x), typ),
            ast::Operand::RegOffset(reg, offset) => {
                self.reg_offset(desc.new_op((reg, offset)), typ)
            }
        }
    }

    fn src_call_operand(
        &mut self,
        desc: ArgumentDescriptor<ast::CallOperand<spirv::Word>>,
        typ: ast::Type,
    ) -> Result<spirv::Word, TranslateError> {
        match desc.op {
            ast::CallOperand::Reg(reg) => self.reg(desc.new_op(reg), Some(typ)),
            ast::CallOperand::Imm(x) => self.immediate(desc.new_op(x), typ),
        }
    }

    fn src_member_operand(
        &mut self,
        desc: ArgumentDescriptor<(spirv::Word, u8)>,
        typ: (ast::ScalarType, u8),
    ) -> Result<spirv::Word, TranslateError> {
        self.member_src(desc, typ)
    }

    fn id_or_vector(
        &mut self,
        desc: ArgumentDescriptor<ast::IdOrVector<spirv::Word>>,
        typ: ast::Type,
    ) -> Result<spirv::Word, TranslateError> {
        match desc.op {
            ast::IdOrVector::Reg(r) => self.reg(desc.new_op(r), Some(typ)),
            ast::IdOrVector::Vec(ref v) => self.vector(desc.new_op(v), typ),
        }
    }

    fn operand_or_vector(
        &mut self,
        desc: ArgumentDescriptor<ast::OperandOrVector<spirv::Word>>,
        typ: ast::Type,
    ) -> Result<spirv::Word, TranslateError> {
        match desc.op {
            ast::OperandOrVector::Reg(r) => self.reg(desc.new_op(r), Some(typ)),
            ast::OperandOrVector::RegOffset(r, imm) => self.reg_offset(desc.new_op((r, imm)), typ),
            ast::OperandOrVector::Imm(imm) => self.immediate(desc.new_op(imm), typ),
            ast::OperandOrVector::Vec(ref v) => self.vector(desc.new_op(v), typ),
        }
    }
}

/*
 There are several kinds of implicit conversions in PTX:
 * auto-bitcast: https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#type-information-for-instructions-and-operands
 * special ld/st/cvt conversion rules: https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#operand-size-exceeding-instruction-type-size
   - ld.param: not documented, but for instruction `ld.param.<type> x, [y]`,
     semantics are to first zext/chop/bitcast `y` as needed and then do
     documented special ld/st/cvt conversion rules for destination operands
   - st.param [x] y (used as function return arguments) same rule as above applies
   - generic/global ld: for instruction `ld x, [y]`, y must be of type
     b64/u64/s64, which is bitcast to a pointer, dereferenced and then
     documented special ld/st/cvt conversion rules are applied to dst
   - generic/global st: for instruction `st [x], y`, x must be of type
     b64/u64/s64, which is bitcast to a pointer
*/
fn insert_implicit_conversions(
    func: Vec<ExpandedStatement>,
    id_def: &mut MutableNumericIdResolver,
) -> Result<Vec<ExpandedStatement>, TranslateError> {
    let mut result = Vec::with_capacity(func.len());
    for s in func.into_iter() {
        match s {
            Statement::Call(call) => insert_implicit_conversions_impl(
                &mut result,
                id_def,
                call,
                should_bitcast_wrapper,
                None,
            )?,
            Statement::Instruction(inst) => {
                let mut default_conversion_fn = should_bitcast_wrapper
                    as fn(_, _, _) -> Result<Option<ConversionKind>, TranslateError>;
                let mut state_space = None;
                if let ast::Instruction::Ld(d, _) = &inst {
                    state_space = Some(d.state_space);
                }
                if let ast::Instruction::St(d, _) = &inst {
                    state_space = Some(d.state_space.to_ld_ss());
                }
                if let ast::Instruction::Mov(_, ast::Arg2Mov::Normal(_)) = &inst {
                    default_conversion_fn = should_bitcast_packed;
                }
                insert_implicit_conversions_impl(
                    &mut result,
                    id_def,
                    inst,
                    default_conversion_fn,
                    state_space,
                )?;
            }
            Statement::Composite(composite) => insert_implicit_conversions_impl(
                &mut result,
                id_def,
                composite,
                should_bitcast_wrapper,
                None,
            )?,
            s @ Statement::Conditional(_)
            | s @ Statement::Label(_)
            | s @ Statement::Constant(_)
            | s @ Statement::Variable(_)
            | s @ Statement::LoadVar(_, _)
            | s @ Statement::StoreVar(_, _)
            | s @ Statement::Undef(_, _)
            | s @ Statement::RetValue(_, _) => result.push(s),
            Statement::Conversion(_) => unreachable!(),
        }
    }
    Ok(result)
}

fn insert_implicit_conversions_impl(
    func: &mut Vec<ExpandedStatement>,
    id_def: &mut MutableNumericIdResolver,
    stmt: impl VisitVariableExpanded,
    default_conversion_fn: fn(
        ast::Type,
        ast::Type,
        Option<ast::LdStateSpace>,
    ) -> Result<Option<ConversionKind>, TranslateError>,
    state_space: Option<ast::LdStateSpace>,
) -> Result<(), TranslateError> {
    let mut post_conv = Vec::new();
    let statement = stmt.visit_variable_extended(&mut |desc, typ| {
        let instr_type = match typ {
            None => return Ok(desc.op),
            Some(t) => t,
        };
        let operand_type = id_def.get_typed(desc.op)?;
        let mut conversion_fn = default_conversion_fn;
        match desc.sema {
            ArgumentSemantics::Default => {}
            ArgumentSemantics::DefaultRelaxed => {
                if desc.is_dst {
                    conversion_fn = should_convert_relaxed_dst_wrapper;
                } else {
                    conversion_fn = should_convert_relaxed_src_wrapper;
                }
            }
            ArgumentSemantics::PhysicalPointer => {
                conversion_fn = bitcast_physical_pointer;
            }
            ArgumentSemantics::RegisterPointer => {
                conversion_fn = force_bitcast;
            }
            ArgumentSemantics::Address => {
                conversion_fn = force_bitcast_ptr_to_bit;
            }
        };
        match conversion_fn(operand_type, instr_type, state_space)? {
            Some(conv_kind) => {
                let conv_output = if desc.is_dst {
                    &mut post_conv
                } else {
                    &mut *func
                };
                let mut from = instr_type;
                let mut to = operand_type;
                let mut src = id_def.new_id(instr_type);
                let mut dst = desc.op;
                let result = Ok(src);
                if !desc.is_dst {
                    mem::swap(&mut src, &mut dst);
                    mem::swap(&mut from, &mut to);
                }
                conv_output.push(Statement::Conversion(ImplicitConversion {
                    src,
                    dst,
                    from,
                    to,
                    kind: conv_kind,
                }));
                result
            }
            None => Ok(desc.op),
        }
    })?;
    func.push(statement);
    func.append(&mut post_conv);
    Ok(())
}

fn get_function_type(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    method_decl: &ast::MethodDecl<spirv::Word>,
) -> (spirv::Word, spirv::Word) {
    match method_decl {
        ast::MethodDecl::Func(out_params, _, in_params) => map.get_or_add_fn(
            builder,
            out_params
                .iter()
                .map(|p| SpirvType::from(ast::Type::from(p.v_type))),
            in_params
                .iter()
                .map(|p| SpirvType::from(ast::Type::from(p.v_type))),
        ),
        ast::MethodDecl::Kernel(_, params) => map.get_or_add_fn(
            builder,
            iter::empty(),
            params
                .iter()
                .map(|p| SpirvType::from(ast::Type::from(p.v_type))),
        ),
    }
}

fn emit_function_body_ops(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    opencl: spirv::Word,
    func: &[ExpandedStatement],
) -> Result<(), TranslateError> {
    for s in func {
        match s {
            Statement::Label(id) => {
                if builder.block.is_some() {
                    builder.branch(*id)?;
                }
                builder.begin_block(Some(*id))?;
            }
            _ => {
                if builder.block.is_none() && builder.function.is_some() {
                    builder.begin_block(None)?;
                }
            }
        }
        match s {
            Statement::Label(_) => (),
            Statement::Call(call) => {
                let (result_type, result_id) = match &*call.ret_params {
                    [(id, typ)] => (
                        map.get_or_add(builder, SpirvType::from(ast::Type::from(*typ))),
                        Some(*id),
                    ),
                    [] => (map.void(), None),
                    _ => todo!(),
                };
                let arg_list = call
                    .param_list
                    .iter()
                    .map(|(id, _)| *id)
                    .collect::<Vec<_>>();
                builder.function_call(result_type, result_id, call.func, arg_list)?;
            }
            Statement::Variable(ast::Variable {
                align,
                v_type,
                name,
            }) => {
                let st_class = match v_type {
                    ast::VariableType::Reg(_)
                    | ast::VariableType::Param(_)
                    | ast::VariableType::Local(_) => spirv::StorageClass::Function,
                };
                let type_id = map.get_or_add(
                    builder,
                    SpirvType::new_pointer(ast::Type::from(*v_type), st_class),
                );
                builder.variable(type_id, Some(*name), st_class, None);
                if let Some(align) = align {
                    builder.decorate(
                        *name,
                        spirv::Decoration::Alignment,
                        &[dr::Operand::LiteralInt32(*align)],
                    );
                }
            }
            Statement::Constant(cnst) => {
                let typ_id = map.get_or_add_scalar(builder, cnst.typ);
                match cnst.typ {
                    ast::ScalarType::B8 | ast::ScalarType::U8 => {
                        builder.constant_u32(typ_id, Some(cnst.dst), cnst.value as u8 as u32);
                    }
                    ast::ScalarType::B16 | ast::ScalarType::U16 => {
                        builder.constant_u32(typ_id, Some(cnst.dst), cnst.value as u16 as u32);
                    }
                    ast::ScalarType::B32 | ast::ScalarType::U32 => {
                        builder.constant_u32(typ_id, Some(cnst.dst), cnst.value as u32);
                    }
                    ast::ScalarType::B64 | ast::ScalarType::U64 => {
                        builder.constant_u64(typ_id, Some(cnst.dst), cnst.value as u64);
                    }
                    ast::ScalarType::S8 => {
                        builder.constant_u32(typ_id, Some(cnst.dst), cnst.value as i8 as u32);
                    }
                    ast::ScalarType::S16 => {
                        builder.constant_u32(typ_id, Some(cnst.dst), cnst.value as i16 as u32);
                    }
                    ast::ScalarType::S32 => {
                        builder.constant_u32(typ_id, Some(cnst.dst), cnst.value as i32 as u32);
                    }
                    ast::ScalarType::S64 => {
                        builder.constant_u64(typ_id, Some(cnst.dst), cnst.value as i64 as u64);
                    }
                    _ => unreachable!(),
                }
            }
            Statement::Conversion(cv) => emit_implicit_conversion(builder, map, cv)?,
            Statement::Conditional(bra) => {
                builder.branch_conditional(bra.predicate, bra.if_true, bra.if_false, [])?;
            }
            Statement::Instruction(inst) => match inst {
                ast::Instruction::Abs(d, arg) => emit_abs(builder, map, opencl, d, arg)?,
                ast::Instruction::Call(_) => unreachable!(),
                // SPIR-V does not support marking jumps as guaranteed-converged
                ast::Instruction::Bra(_, arg) => {
                    builder.branch(arg.src)?;
                }
                ast::Instruction::Ld(data, arg) => {
                    if data.qualifier != ast::LdStQualifier::Weak {
                        todo!()
                    }
                    let result_type = map.get_or_add(builder, SpirvType::from(data.typ));
                    match data.state_space {
                        ast::LdStateSpace::Generic | ast::LdStateSpace::Global => {
                            builder.load(result_type, Some(arg.dst), arg.src, None, [])?;
                        }
                        ast::LdStateSpace::Param | ast::LdStateSpace::Local => {
                            let result_type = map.get_or_add(builder, SpirvType::from(data.typ));
                            builder.copy_object(result_type, Some(arg.dst), arg.src)?;
                        }
                        _ => todo!(),
                    }
                }
                ast::Instruction::St(data, arg) => {
                    if data.qualifier != ast::LdStQualifier::Weak {
                        todo!()
                    }
                    if data.state_space == ast::StStateSpace::Param
                        || data.state_space == ast::StStateSpace::Local
                    {
                        let result_type = map.get_or_add(builder, SpirvType::from(data.typ));
                        builder.copy_object(result_type, Some(arg.src1), arg.src2)?;
                    } else if data.state_space == ast::StStateSpace::Generic
                        || data.state_space == ast::StStateSpace::Global
                    {
                        builder.store(arg.src1, arg.src2, None, &[])?;
                    } else {
                        todo!()
                    }
                }
                // SPIR-V does not support ret as guaranteed-converged
                ast::Instruction::Ret(_) => builder.ret()?,
                ast::Instruction::Mov(d, arg) => match arg {
                    ast::Arg2Mov::Normal(ast::Arg2MovNormal { dst, src })
                    | ast::Arg2Mov::Member(ast::Arg2MovMember::Src(dst, src)) => {
                        let result_type =
                            map.get_or_add(builder, SpirvType::from(ast::Type::from(d.typ)));
                        builder.copy_object(result_type, Some(*dst), *src)?;
                    }
                    ast::Arg2Mov::Member(ast::Arg2MovMember::Dst(
                        dst,
                        composite_src,
                        scalar_src,
                    ))
                    | ast::Arg2Mov::Member(ast::Arg2MovMember::Both(
                        dst,
                        composite_src,
                        scalar_src,
                    )) => {
                        let scalar_type = d.typ.get_scalar()?;
                        let result_type = map.get_or_add(
                            builder,
                            SpirvType::from(ast::Type::Vector(scalar_type, d.dst_width)),
                        );
                        let result_id = Some(dst.0);
                        builder.composite_insert(
                            result_type,
                            result_id,
                            *scalar_src,
                            *composite_src,
                            [dst.1 as u32],
                        )?;
                    }
                },
                ast::Instruction::Mul(mul, arg) => match mul {
                    ast::MulDetails::Int(ref ctr) => {
                        emit_mul_int(builder, map, opencl, ctr, arg)?;
                    }
                    ast::MulDetails::Float(_) => todo!(),
                },
                ast::Instruction::Add(add, arg) => match add {
                    ast::AddDetails::Int(ref desc) => emit_add_int(builder, map, desc, arg)?,
                    ast::AddDetails::Float(desc) => emit_add_float(builder, map, desc, arg)?,
                },
                ast::Instruction::Setp(setp, arg) => {
                    if arg.dst2.is_some() {
                        todo!()
                    }
                    emit_setp(builder, map, setp, arg)?;
                }
                ast::Instruction::Not(t, a) => {
                    let result_type = map.get_or_add(builder, SpirvType::from(t.to_type()));
                    let result_id = Some(a.dst);
                    let operand = a.src;
                    match t {
                        ast::NotType::Pred => {
                            // HACK ALERT
                            // Temporary workaround until IGC gets its shit together
                            // Currently IGC carries two copies of SPIRV-LLVM translator
                            // a new one in /llvm-spirv/ and old one in /IGC/AdaptorOCL/SPIRV/.
                            // Obviously, old and buggy one is used for compiling L0 SPIRV
                            // https://github.com/intel/intel-graphics-compiler/issues/148
                            let type_pred = map.get_or_add_scalar(builder, ast::ScalarType::Pred);
                            let const_true = builder.constant_true(type_pred, None);
                            let const_false = builder.constant_false(type_pred, None);
                            builder.select(result_type, result_id, operand, const_false, const_true)
                        }
                        _ => builder.not(result_type, result_id, operand),
                    }?;
                }
                ast::Instruction::Shl(t, a) => {
                    let result_type = map.get_or_add(builder, SpirvType::from(t.to_type()));
                    builder.shift_left_logical(result_type, Some(a.dst), a.src1, a.src2)?;
                }
                ast::Instruction::Shr(t, a) => {
                    let result_type = map.get_or_add_scalar(builder, ast::ScalarType::from(*t));
                    if t.signed() {
                        builder.shift_right_arithmetic(result_type, Some(a.dst), a.src1, a.src2)?;
                    } else {
                        builder.shift_right_logical(result_type, Some(a.dst), a.src1, a.src2)?;
                    }
                }
                ast::Instruction::Cvt(dets, arg) => {
                    emit_cvt(builder, map, dets, arg)?;
                }
                ast::Instruction::Cvta(_, arg) => {
                    // This would be only meaningful if const/slm/global pointers
                    // had a different format than generic pointers, but they don't pretty much by ptx definition
                    // Honestly, I have no idea why this instruction exists and is emitted by the compiler
                    let result_type = map.get_or_add_scalar(builder, ast::ScalarType::B64);
                    builder.copy_object(result_type, Some(arg.dst), arg.src)?;
                }
                ast::Instruction::SetpBool(_, _) => todo!(),
                ast::Instruction::Mad(mad, arg) => match mad {
                    ast::MulDetails::Int(ref desc) => {
                        emit_mad_int(builder, map, opencl, desc, arg)?
                    }
                    ast::MulDetails::Float(desc) => emit_mad_float(builder, map, desc, arg)?,
                },
            },
            Statement::LoadVar(arg, typ) => {
                let type_id = map.get_or_add(builder, SpirvType::from(*typ));
                builder.load(type_id, Some(arg.dst), arg.src, None, [])?;
            }
            Statement::StoreVar(arg, _) => {
                builder.store(arg.src1, arg.src2, None, [])?;
            }
            Statement::RetValue(_, id) => {
                builder.ret_value(*id)?;
            }
            Statement::Composite(c) => {
                let result_type = map.get_or_add_scalar(builder, c.typ.into());
                let result_id = Some(c.dst);
                builder.composite_extract(
                    result_type,
                    result_id,
                    c.src_composite,
                    [c.src_index],
                )?;
            }
            Statement::Undef(t, id) => {
                let result_type = map.get_or_add(builder, SpirvType::from(*t));
                builder.undef(result_type, Some(*id));
            }
        }
    }
    Ok(())
}

fn emit_mad_int(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    opencl: spirv::Word,
    desc: &ast::MulIntDesc,
    arg: &ast::Arg4<ExpandedArgParams>,
) -> Result<(), dr::Error> {
    let inst_type = map.get_or_add(builder, SpirvType::from(ast::ScalarType::from(desc.typ)));
    match desc.control {
        ast::MulIntControl::Low => {
            let mul_result = builder.i_mul(inst_type, None, arg.src1, arg.src2)?;
            builder.i_add(inst_type, Some(arg.dst), arg.src3, mul_result)?;
        }
        ast::MulIntControl::High => {
            let cl_op = if desc.typ.is_signed() {
                spirv::CLOp::s_mad_hi
            } else {
                spirv::CLOp::u_mad_hi
            };
            builder.ext_inst(
                inst_type,
                Some(arg.dst),
                opencl,
                cl_op as spirv::Word,
                [arg.src1, arg.src2, arg.src3],
            )?;
        }
        ast::MulIntControl::Wide => todo!(),
    };
    Ok(())
}

fn emit_mad_float(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    desc: &ast::MulFloatDesc,
    arg: &ast::Arg4<ExpandedArgParams>,
) -> Result<(), dr::Error> {
    todo!()
}

fn emit_add_float(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    desc: &ast::AddFloatDesc,
    arg: &ast::Arg3<ExpandedArgParams>,
) -> Result<(), dr::Error> {
    if desc.flush_to_zero {
        todo!()
    }
    let inst_type = map.get_or_add(builder, SpirvType::from(ast::ScalarType::from(desc.typ)));
    builder.f_add(inst_type, Some(arg.dst), arg.src1, arg.src2)?;
    emit_rounding_decoration(builder, arg.dst, desc.rounding);
    Ok(())
}

fn emit_cvt(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    dets: &ast::CvtDetails,
    arg: &ast::Arg2<ExpandedArgParams>,
) -> Result<(), dr::Error> {
    match dets {
        ast::CvtDetails::FloatFromFloat(desc) => {
            if desc.dst == desc.src {
                return Ok(());
            }
            if desc.saturate || desc.flush_to_zero {
                todo!()
            }
            let dest_t: ast::ScalarType = desc.dst.into();
            let result_type = map.get_or_add(builder, SpirvType::from(dest_t));
            builder.f_convert(result_type, Some(arg.dst), arg.src)?;
            emit_rounding_decoration(builder, arg.dst, desc.rounding);
        }
        ast::CvtDetails::FloatFromInt(desc) => {
            if desc.saturate || desc.flush_to_zero {
                todo!()
            }
            let dest_t: ast::ScalarType = desc.dst.into();
            let result_type = map.get_or_add(builder, SpirvType::from(dest_t));
            if desc.src.is_signed() {
                builder.convert_s_to_f(result_type, Some(arg.dst), arg.src)?;
            } else {
                builder.convert_u_to_f(result_type, Some(arg.dst), arg.src)?;
            }
            emit_rounding_decoration(builder, arg.dst, desc.rounding);
        }
        ast::CvtDetails::IntFromFloat(desc) => {
            if desc.flush_to_zero {
                todo!()
            }
            let dest_t: ast::ScalarType = desc.dst.into();
            let result_type = map.get_or_add(builder, SpirvType::from(dest_t));
            if desc.dst.is_signed() {
                builder.convert_f_to_s(result_type, Some(arg.dst), arg.src)?;
            } else {
                builder.convert_f_to_u(result_type, Some(arg.dst), arg.src)?;
            }
            emit_rounding_decoration(builder, arg.dst, desc.rounding);
            emit_saturating_decoration(builder, arg.dst, desc.saturate);
        }
        ast::CvtDetails::IntFromInt(desc) => {
            if desc.dst == desc.src {
                return Ok(());
            }
            let dest_t: ast::ScalarType = desc.dst.into();
            let src_t: ast::ScalarType = desc.src.into();
            // first do shortening/widening
            let src = if desc.dst.width() != desc.src.width() {
                let new_dst = if dest_t.kind() == src_t.kind() {
                    arg.dst
                } else {
                    builder.id()
                };
                let cv = ImplicitConversion {
                    src: arg.src,
                    dst: new_dst,
                    from: ast::Type::Scalar(src_t),
                    to: ast::Type::Scalar(ast::ScalarType::from_parts(
                        dest_t.width(),
                        src_t.kind(),
                    )),
                    kind: ConversionKind::Default,
                };
                emit_implicit_conversion(builder, map, &cv)?;
                new_dst
            } else {
                arg.src
            };
            if dest_t.kind() == src_t.kind() {
                return Ok(());
            }
            // now do actual conversion
            let result_type = map.get_or_add(builder, SpirvType::from(dest_t));
            if desc.saturate {
                if desc.dst.is_signed() {
                    builder.sat_convert_u_to_s(result_type, Some(arg.dst), src)?;
                } else {
                    builder.sat_convert_s_to_u(result_type, Some(arg.dst), src)?;
                }
            } else {
                builder.bitcast(result_type, Some(arg.dst), src)?;
            }
        }
    }
    Ok(())
}

fn emit_saturating_decoration(builder: &mut dr::Builder, dst: u32, saturate: bool) {
    if saturate {
        builder.decorate(dst, spirv::Decoration::SaturatedConversion, []);
    }
}

fn emit_rounding_decoration(
    builder: &mut dr::Builder,
    dst: spirv::Word,
    rounding: Option<ast::RoundingMode>,
) {
    if let Some(rounding) = rounding {
        builder.decorate(
            dst,
            spirv::Decoration::FPRoundingMode,
            [rounding.to_spirv()],
        );
    }
}

impl ast::RoundingMode {
    fn to_spirv(self) -> rspirv::dr::Operand {
        let mode = match self {
            ast::RoundingMode::NearestEven => spirv::FPRoundingMode::RTE,
            ast::RoundingMode::Zero => spirv::FPRoundingMode::RTZ,
            ast::RoundingMode::PositiveInf => spirv::FPRoundingMode::RTP,
            ast::RoundingMode::NegativeInf => spirv::FPRoundingMode::RTN,
        };
        rspirv::dr::Operand::FPRoundingMode(mode)
    }
}

fn emit_setp(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    setp: &ast::SetpData,
    arg: &ast::Arg4Setp<ExpandedArgParams>,
) -> Result<(), dr::Error> {
    if setp.flush_to_zero {
        todo!()
    }
    let result_type = map.get_or_add(builder, SpirvType::Base(SpirvScalarKey::Pred));
    let result_id = Some(arg.dst1);
    let operand_1 = arg.src1;
    let operand_2 = arg.src2;
    match (setp.cmp_op, setp.typ.kind()) {
        (ast::SetpCompareOp::Eq, ScalarKind::Signed)
        | (ast::SetpCompareOp::Eq, ScalarKind::Unsigned)
        | (ast::SetpCompareOp::Eq, ScalarKind::Bit) => {
            builder.i_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::Eq, ScalarKind::Float) => {
            builder.f_ord_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::NotEq, ScalarKind::Signed)
        | (ast::SetpCompareOp::NotEq, ScalarKind::Unsigned)
        | (ast::SetpCompareOp::NotEq, ScalarKind::Bit) => {
            builder.i_not_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::NotEq, ScalarKind::Float) => {
            builder.f_ord_not_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::Less, ScalarKind::Unsigned)
        | (ast::SetpCompareOp::Less, ScalarKind::Bit) => {
            builder.u_less_than(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::Less, ScalarKind::Signed) => {
            builder.s_less_than(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::Less, ScalarKind::Float) => {
            builder.f_ord_less_than(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::LessOrEq, ScalarKind::Unsigned)
        | (ast::SetpCompareOp::LessOrEq, ScalarKind::Bit) => {
            builder.u_less_than_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::LessOrEq, ScalarKind::Signed) => {
            builder.s_less_than_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::LessOrEq, ScalarKind::Float) => {
            builder.f_ord_less_than_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::Greater, ScalarKind::Unsigned)
        | (ast::SetpCompareOp::Greater, ScalarKind::Bit) => {
            builder.u_greater_than(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::Greater, ScalarKind::Signed) => {
            builder.s_greater_than(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::Greater, ScalarKind::Float) => {
            builder.f_ord_greater_than(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::GreaterOrEq, ScalarKind::Unsigned)
        | (ast::SetpCompareOp::GreaterOrEq, ScalarKind::Bit) => {
            builder.u_greater_than_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::GreaterOrEq, ScalarKind::Signed) => {
            builder.s_greater_than_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::GreaterOrEq, ScalarKind::Float) => {
            builder.f_ord_greater_than_equal(result_type, result_id, operand_1, operand_2)
        }
        _ => todo!(),
    }?;
    Ok(())
}

fn emit_mul_int(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    opencl: spirv::Word,
    desc: &ast::MulIntDesc,
    arg: &ast::Arg3<ExpandedArgParams>,
) -> Result<(), dr::Error> {
    let instruction_type = ast::ScalarType::from(desc.typ);
    let inst_type = map.get_or_add(builder, SpirvType::from(ast::ScalarType::from(desc.typ)));
    match desc.control {
        ast::MulIntControl::Low => {
            builder.i_mul(inst_type, Some(arg.dst), arg.src1, arg.src2)?;
        }
        ast::MulIntControl::High => {
            let ocl_mul_hi = if desc.typ.is_signed() {
                spirv::CLOp::s_mul_hi
            } else {
                spirv::CLOp::u_mul_hi
            };
            builder.ext_inst(
                inst_type,
                Some(arg.dst),
                opencl,
                ocl_mul_hi as spirv::Word,
                [arg.src1, arg.src2],
            )?;
        }
        ast::MulIntControl::Wide => {
            let mul_ext_type = SpirvType::Struct(vec![
                SpirvScalarKey::from(instruction_type),
                SpirvScalarKey::from(instruction_type),
            ]);
            let mul_ext_type_id = map.get_or_add(builder, mul_ext_type);
            let mul = if desc.typ.is_signed() {
                builder.s_mul_extended(mul_ext_type_id, None, arg.src1, arg.src2)?
            } else {
                builder.u_mul_extended(mul_ext_type_id, None, arg.src1, arg.src2)?
            };
            let instr_width = instruction_type.width();
            let instr_kind = instruction_type.kind();
            let dst_type = ast::ScalarType::from_parts(instr_width * 2, instr_kind);
            let dst_type_id = map.get_or_add_scalar(builder, dst_type);
            struct2_bitcast_to_wide(
                builder,
                map,
                SpirvScalarKey::from(instruction_type),
                inst_type,
                arg.dst,
                dst_type_id,
                mul,
            )?;
        }
    }
    Ok(())
}

// Surprisingly, structs can't be bitcast, so we route everything through a vector
fn struct2_bitcast_to_wide(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    base_type_key: SpirvScalarKey,
    instruction_type: spirv::Word,
    dst: spirv::Word,
    dst_type_id: spirv::Word,
    src: spirv::Word,
) -> Result<(), dr::Error> {
    let low_bits = builder.composite_extract(instruction_type, None, src, [0])?;
    let high_bits = builder.composite_extract(instruction_type, None, src, [1])?;
    let vector_type = map.get_or_add(builder, SpirvType::Vector(base_type_key, 2));
    let vector = builder.composite_construct(vector_type, None, [low_bits, high_bits])?;
    builder.bitcast(dst_type_id, Some(dst), vector)?;
    Ok(())
}

fn emit_abs(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    opencl: spirv::Word,
    d: &ast::AbsDetails,
    arg: &ast::Arg2<ExpandedArgParams>,
) -> Result<(), dr::Error> {
    let scalar_t = ast::ScalarType::from(d.typ);
    let result_type = map.get_or_add(builder, SpirvType::from(scalar_t));
    let cl_abs = if scalar_t.kind() == ScalarKind::Signed {
        spirv::CLOp::s_abs
    } else {
        spirv::CLOp::fabs
    };
    builder.ext_inst(
        result_type,
        Some(arg.dst),
        opencl,
        cl_abs as spirv::Word,
        [arg.src],
    )?;
    Ok(())
}

fn emit_add_int(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    ctr: &ast::AddIntDesc,
    arg: &ast::Arg3<ExpandedArgParams>,
) -> Result<(), dr::Error> {
    let inst_type = map.get_or_add(builder, SpirvType::from(ast::ScalarType::from(ctr.typ)));
    builder.i_add(inst_type, Some(arg.dst), arg.src1, arg.src2)?;
    Ok(())
}

fn emit_implicit_conversion(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    cv: &ImplicitConversion,
) -> Result<(), dr::Error> {
    let from_parts = cv.from.to_parts();
    let to_parts = cv.to.to_parts();
    match (from_parts.kind, to_parts.kind, cv.kind) {
        (_, _, ConversionKind::PtrToBit) => {
            let dst_type = map.get_or_add_scalar(builder, ast::ScalarType::B64);
            builder.convert_ptr_to_u(dst_type, Some(cv.dst), cv.src)?;
        }
        (_, _, ConversionKind::BitToPtr(space)) => {
            let dst_type = map.get_or_add(
                builder,
                SpirvType::Pointer(Box::new(SpirvType::from(cv.to)), space.to_spirv()),
            );
            builder.convert_u_to_ptr(dst_type, Some(cv.dst), cv.src)?;
        }
        (TypeKind::Scalar, TypeKind::Scalar, ConversionKind::Default) => {
            if from_parts.width == to_parts.width {
                let dst_type = map.get_or_add(builder, SpirvType::from(cv.to));
                if from_parts.scalar_kind != ScalarKind::Float
                    && to_parts.scalar_kind != ScalarKind::Float
                {
                    // It is noop, but another instruction expects result of this conversion
                    builder.copy_object(dst_type, Some(cv.dst), cv.src)?;
                } else {
                    builder.bitcast(dst_type, Some(cv.dst), cv.src)?;
                }
            } else {
                // This block is safe because it's illegal to implictly convert between floating point instructions
                let same_width_bit_type = map.get_or_add(
                    builder,
                    SpirvType::from(ast::Type::from_parts(TypeParts {
                        scalar_kind: ScalarKind::Bit,
                        ..from_parts
                    })),
                );
                let same_width_bit_value = builder.bitcast(same_width_bit_type, None, cv.src)?;
                let wide_bit_type = ast::Type::from_parts(TypeParts {
                    scalar_kind: ScalarKind::Bit,
                    ..to_parts
                });
                let wide_bit_type_spirv = map.get_or_add(builder, SpirvType::from(wide_bit_type));
                if to_parts.scalar_kind == ScalarKind::Unsigned
                    || to_parts.scalar_kind == ScalarKind::Bit
                {
                    builder.u_convert(wide_bit_type_spirv, Some(cv.dst), same_width_bit_value)?;
                } else {
                    let wide_bit_value =
                        builder.u_convert(wide_bit_type_spirv, None, same_width_bit_value)?;
                    emit_implicit_conversion(
                        builder,
                        map,
                        &ImplicitConversion {
                            src: wide_bit_value,
                            dst: cv.dst,
                            from: wide_bit_type,
                            to: cv.to,
                            kind: ConversionKind::Default,
                        },
                    )?;
                }
            }
        }
        (TypeKind::Scalar, TypeKind::Scalar, ConversionKind::SignExtend) => todo!(),
        (TypeKind::Vector, TypeKind::Scalar, ConversionKind::Default)
        | (TypeKind::Scalar, TypeKind::Array, ConversionKind::Default)
        | (TypeKind::Array, TypeKind::Scalar, ConversionKind::Default) => {
            let into_type = map.get_or_add(builder, SpirvType::from(cv.to));
            builder.bitcast(into_type, Some(cv.dst), cv.src)?;
        }
        _ => unreachable!(),
    }
    Ok(())
}

fn normalize_identifiers<'a, 'b>(
    id_defs: &mut FnStringIdResolver<'a, 'b>,
    fn_defs: &GlobalFnDeclResolver<'a, 'b>,
    func: Vec<ast::Statement<ast::ParsedArgParams<'a>>>,
) -> Result<Vec<NormalizedStatement>, TranslateError> {
    for s in func.iter() {
        match s {
            ast::Statement::Label(id) => {
                id_defs.add_def(*id, None);
            }
            _ => (),
        }
    }
    let mut result = Vec::new();
    for s in func {
        expand_map_variables(id_defs, fn_defs, &mut result, s)?;
    }
    Ok(result)
}

fn expand_map_variables<'a, 'b>(
    id_defs: &mut FnStringIdResolver<'a, 'b>,
    fn_defs: &GlobalFnDeclResolver<'a, 'b>,
    result: &mut Vec<NormalizedStatement>,
    s: ast::Statement<ast::ParsedArgParams<'a>>,
) -> Result<(), TranslateError> {
    match s {
        ast::Statement::Block(block) => {
            id_defs.start_block();
            for s in block {
                expand_map_variables(id_defs, fn_defs, result, s)?;
            }
            id_defs.end_block();
        }
        ast::Statement::Label(name) => result.push(Statement::Label(id_defs.get_id(name)?)),
        ast::Statement::Instruction(p, i) => result.push(Statement::Instruction((
            p.map(|p| p.map_variable(&mut |id| id_defs.get_id(id)))
                .transpose()?,
            i.map_variable(&mut |id| id_defs.get_id(id))?,
        ))),
        ast::Statement::Variable(var) => {
            let ss = match var.var.v_type {
                ast::VariableType::Reg(_) => StateSpace::Reg,
                ast::VariableType::Local(_) => StateSpace::Local,
                ast::VariableType::Param(_) => StateSpace::ParamReg,
            };
            match var.count {
                Some(count) => {
                    for new_id in id_defs.add_defs(var.var.name, count, ss, var.var.v_type.into()) {
                        result.push(Statement::Variable(ast::Variable {
                            align: var.var.align,
                            v_type: var.var.v_type,
                            name: new_id,
                        }))
                    }
                }
                None => {
                    let new_id = id_defs.add_def(var.var.name, Some((ss, var.var.v_type.into())));
                    result.push(Statement::Variable(ast::Variable {
                        align: var.var.align,
                        v_type: var.var.v_type,
                        name: new_id,
                    }));
                }
            }
        }
    };
    Ok(())
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
enum PtxSpecialRegister {
    Tid,
    Ntid,
    Ctaid,
    Nctaid,
}

impl PtxSpecialRegister {
    fn try_parse(s: &str) -> Option<Self> {
        match s {
            "%tid" => Some(Self::Tid),
            "%ntid" => Some(Self::Ntid),
            "%ctaid" => Some(Self::Ctaid),
            "%nctaid" => Some(Self::Nctaid),
            _ => None,
        }
    }

    fn get_type(self) -> ast::Type {
        match self {
            PtxSpecialRegister::Tid => ast::Type::Vector(ast::ScalarType::U32, 4),
            PtxSpecialRegister::Ntid => ast::Type::Vector(ast::ScalarType::U32, 4),
            PtxSpecialRegister::Ctaid => ast::Type::Vector(ast::ScalarType::U32, 4),
            PtxSpecialRegister::Nctaid => ast::Type::Vector(ast::ScalarType::U32, 4),
        }
    }

    fn get_builtin(self) -> spirv::BuiltIn {
        match self {
            PtxSpecialRegister::Tid => spirv::BuiltIn::LocalInvocationId,
            PtxSpecialRegister::Ntid => spirv::BuiltIn::WorkgroupSize,
            PtxSpecialRegister::Ctaid => spirv::BuiltIn::WorkgroupId,
            PtxSpecialRegister::Nctaid => spirv::BuiltIn::NumWorkgroups,
        }
    }
}

struct GlobalStringIdResolver<'input> {
    current_id: spirv::Word,
    variables: HashMap<Cow<'input, str>, spirv::Word>,
    special_registers: HashMap<PtxSpecialRegister, spirv::Word>,
    fns: HashMap<spirv::Word, FnDecl>,
}

pub struct FnDecl {
    ret_vals: Vec<ast::FnArgumentType>,
    params: Vec<ast::FnArgumentType>,
}

impl<'a> GlobalStringIdResolver<'a> {
    fn new(start_id: spirv::Word) -> Self {
        Self {
            current_id: start_id,
            variables: HashMap::new(),
            special_registers: HashMap::new(),
            fns: HashMap::new(),
        }
    }

    fn get_or_add_def(&mut self, id: &'a str) -> spirv::Word {
        match self.variables.entry(Cow::Borrowed(id)) {
            hash_map::Entry::Occupied(e) => *(e.get()),
            hash_map::Entry::Vacant(e) => {
                let numeric_id = self.current_id;
                e.insert(numeric_id);
                self.current_id += 1;
                numeric_id
            }
        }
    }

    fn get_id(&self, id: &str) -> Result<spirv::Word, TranslateError> {
        self.variables
            .get(id)
            .copied()
            .ok_or(TranslateError::UnknownSymbol)
    }

    fn current_id(&self) -> spirv::Word {
        self.current_id
    }

    fn start_fn<'b>(
        &'b mut self,
        header: &'b ast::MethodDecl<'a, &'a str>,
    ) -> (
        FnStringIdResolver<'a, 'b>,
        GlobalFnDeclResolver<'a, 'b>,
        ast::MethodDecl<'a, spirv::Word>,
    ) {
        // In case a function decl was inserted earlier we want to use its id
        let name_id = self.get_or_add_def(header.name());
        let mut fn_resolver = FnStringIdResolver {
            current_id: &mut self.current_id,
            global_variables: &self.variables,
            special_registers: &mut self.special_registers,
            variables: vec![HashMap::new(); 1],
            type_check: HashMap::new(),
        };
        let new_fn_decl = match header {
            ast::MethodDecl::Kernel(name, params) => {
                ast::MethodDecl::Kernel(name, expand_kernel_params(&mut fn_resolver, params.iter()))
            }
            ast::MethodDecl::Func(ret_params, _, params) => {
                let ret_params_ids = expand_fn_params(&mut fn_resolver, ret_params.iter());
                let params_ids = expand_fn_params(&mut fn_resolver, params.iter());
                self.fns.insert(
                    name_id,
                    FnDecl {
                        ret_vals: ret_params_ids.iter().map(|p| p.v_type).collect(),
                        params: params_ids.iter().map(|p| p.v_type).collect(),
                    },
                );
                ast::MethodDecl::Func(ret_params_ids, name_id, params_ids)
            }
        };
        (
            fn_resolver,
            GlobalFnDeclResolver {
                variables: &self.variables,
                fns: &self.fns,
            },
            new_fn_decl,
        )
    }
}

pub struct GlobalFnDeclResolver<'input, 'a> {
    variables: &'a HashMap<Cow<'input, str>, spirv::Word>,
    fns: &'a HashMap<spirv::Word, FnDecl>,
}

impl<'input, 'a> GlobalFnDeclResolver<'input, 'a> {
    fn get_fn_decl(&self, id: spirv::Word) -> Result<&FnDecl, TranslateError> {
        self.fns.get(&id).ok_or(TranslateError::UnknownSymbol)
    }

    fn get_fn_decl_str(&self, id: &str) -> Result<&'a FnDecl, TranslateError> {
        match self.variables.get(id).map(|var_id| self.fns.get(var_id)) {
            Some(Some(fn_d)) => Ok(fn_d),
            _ => Err(TranslateError::UnknownSymbol),
        }
    }
}

struct FnStringIdResolver<'input, 'b> {
    current_id: &'b mut spirv::Word,
    global_variables: &'b HashMap<Cow<'input, str>, spirv::Word>,
    special_registers: &'b mut HashMap<PtxSpecialRegister, spirv::Word>,
    variables: Vec<HashMap<Cow<'input, str>, spirv::Word>>,
    type_check: HashMap<u32, Option<(StateSpace, ast::Type)>>,
}

impl<'a, 'b> FnStringIdResolver<'a, 'b> {
    fn finish(self) -> NumericIdResolver<'b> {
        NumericIdResolver {
            current_id: self.current_id,
            type_check: self.type_check,
            special_registers: self
                .special_registers
                .iter()
                .map(|(reg, id)| (*id, *reg))
                .collect(),
        }
    }

    fn start_block(&mut self) {
        self.variables.push(HashMap::new())
    }

    fn end_block(&mut self) {
        self.variables.pop();
    }

    fn get_id(&mut self, id: &str) -> Result<spirv::Word, TranslateError> {
        for scope in self.variables.iter().rev() {
            match scope.get(id) {
                Some(id) => return Ok(*id),
                None => continue,
            }
        }
        match self.global_variables.get(id) {
            Some(id) => Ok(*id),
            None => {
                let sreg =
                    PtxSpecialRegister::try_parse(id).ok_or(TranslateError::UnknownSymbol)?;
                match self.special_registers.entry(sreg) {
                    hash_map::Entry::Occupied(e) => Ok(*e.get()),
                    hash_map::Entry::Vacant(e) => {
                        let numeric_id = *self.current_id;
                        *self.current_id += 1;
                        e.insert(numeric_id);
                        Ok(numeric_id)
                    }
                }
            }
        }
    }

    fn add_def(&mut self, id: &'a str, typ: Option<(StateSpace, ast::Type)>) -> spirv::Word {
        let numeric_id = *self.current_id;
        self.variables
            .last_mut()
            .unwrap()
            .insert(Cow::Borrowed(id), numeric_id);
        self.type_check.insert(numeric_id, typ);
        *self.current_id += 1;
        numeric_id
    }

    #[must_use]
    fn add_defs(
        &mut self,
        base_id: &'a str,
        count: u32,
        ss: StateSpace,
        typ: ast::Type,
    ) -> impl Iterator<Item = spirv::Word> {
        let numeric_id = *self.current_id;
        for i in 0..count {
            self.variables
                .last_mut()
                .unwrap()
                .insert(Cow::Owned(format!("{}{}", base_id, i)), numeric_id + i);
            self.type_check.insert(numeric_id + i, Some((ss, typ)));
        }
        *self.current_id += count;
        (0..count).into_iter().map(move |i| i + numeric_id)
    }
}

struct NumericIdResolver<'b> {
    current_id: &'b mut spirv::Word,
    type_check: HashMap<u32, Option<(StateSpace, ast::Type)>>,
    special_registers: HashMap<spirv::Word, PtxSpecialRegister>,
}

impl<'b> NumericIdResolver<'b> {
    fn finish(self) -> MutableNumericIdResolver<'b> {
        MutableNumericIdResolver { base: self }
    }

    fn get_typed(&self, id: spirv::Word) -> Result<(StateSpace, ast::Type), TranslateError> {
        match self.type_check.get(&id) {
            Some(Some(x)) => Ok(*x),
            Some(None) => Err(TranslateError::UntypedSymbol),
            None => match self.special_registers.get(&id) {
                Some(x) => Ok((StateSpace::Reg, x.get_type())),
                None => Err(TranslateError::UntypedSymbol),
            },
        }
    }

    fn new_id(&mut self, typ: Option<(StateSpace, ast::Type)>) -> spirv::Word {
        let new_id = *self.current_id;
        self.type_check.insert(new_id, typ);
        *self.current_id += 1;
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

    fn get_typed(&self, id: spirv::Word) -> Result<ast::Type, TranslateError> {
        self.base.get_typed(id).map(|(_, t)| t)
    }

    fn new_id(&mut self, typ: ast::Type) -> spirv::Word {
        self.base.new_id(Some((StateSpace::Reg, typ)))
    }
}

enum Statement<I, P: ast::ArgParams> {
    Label(u32),
    Variable(ast::Variable<ast::VariableType, P::Id>),
    Instruction(I),
    LoadVar(ast::Arg2<ExpandedArgParams>, ast::Type),
    StoreVar(ast::Arg2St<ExpandedArgParams>, ast::Type),
    Call(ResolvedCall<P>),
    Composite(CompositeRead),
    // SPIR-V compatible replacement for PTX predicates
    Conditional(BrachCondition),
    Conversion(ImplicitConversion),
    Constant(ConstantDefinition),
    RetValue(ast::RetData, spirv::Word),
    Undef(ast::Type, spirv::Word),
}

struct ResolvedCall<P: ast::ArgParams> {
    pub uniform: bool,
    pub ret_params: Vec<(spirv::Word, ast::FnArgumentType)>,
    pub func: spirv::Word,
    pub param_list: Vec<(P::CallOperand, ast::FnArgumentType)>,
}

impl<T: ast::ArgParams> ResolvedCall<T> {
    fn cast<U: ast::ArgParams<CallOperand = T::CallOperand>>(self) -> ResolvedCall<U> {
        ResolvedCall {
            uniform: self.uniform,
            ret_params: self.ret_params,
            func: self.func,
            param_list: self.param_list,
        }
    }
}

impl<From: ArgParamsEx<Id = spirv::Word>> ResolvedCall<From> {
    fn map<To: ArgParamsEx<Id = spirv::Word>, V: ArgumentMapVisitor<From, To>>(
        self,
        visitor: &mut V,
    ) -> Result<ResolvedCall<To>, TranslateError> {
        let ret_params = self
            .ret_params
            .into_iter()
            .map::<Result<_, TranslateError>, _>(|(id, typ)| {
                let new_id = visitor.id(
                    ArgumentDescriptor {
                        op: id,
                        is_dst: true,
                        sema: ArgumentSemantics::Default,
                    },
                    Some(typ.into()),
                )?;
                Ok((new_id, typ))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let func = visitor.id(
            ArgumentDescriptor {
                op: self.func,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            None,
        )?;
        let param_list = self
            .param_list
            .into_iter()
            .map::<Result<_, TranslateError>, _>(|(id, typ)| {
                let new_id = visitor.src_call_operand(
                    ArgumentDescriptor {
                        op: id,
                        is_dst: false,
                        sema: ArgumentSemantics::Default,
                    },
                    typ.into(),
                )?;
                Ok((new_id, typ))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ResolvedCall {
            uniform: self.uniform,
            ret_params,
            func,
            param_list,
        })
    }
}

impl VisitVariable for ResolvedCall<TypedArgParams> {
    fn visit_variable<
        'a,
        F: FnMut(
            ArgumentDescriptor<spirv::Word>,
            Option<ast::Type>,
        ) -> Result<spirv::Word, TranslateError>,
    >(
        self,
        f: &mut F,
    ) -> Result<TypedStatement, TranslateError> {
        Ok(Statement::Call(self.map(f)?))
    }
}

impl VisitVariableExpanded for ResolvedCall<ExpandedArgParams> {
    fn visit_variable_extended<
        F: FnMut(
            ArgumentDescriptor<spirv::Word>,
            Option<ast::Type>,
        ) -> Result<spirv::Word, TranslateError>,
    >(
        self,
        f: &mut F,
    ) -> Result<ExpandedStatement, TranslateError> {
        Ok(Statement::Call(self.map(f)?))
    }
}

pub trait ArgParamsEx: ast::ArgParams + Sized {
    fn get_fn_decl<'x, 'b>(
        id: &Self::Id,
        decl: &'b GlobalFnDeclResolver<'x, 'b>,
    ) -> Result<&'b FnDecl, TranslateError>;
}

impl<'input> ArgParamsEx for ast::ParsedArgParams<'input> {
    fn get_fn_decl<'x, 'b>(
        id: &Self::Id,
        decl: &'b GlobalFnDeclResolver<'x, 'b>,
    ) -> Result<&'b FnDecl, TranslateError> {
        decl.get_fn_decl_str(id)
    }
}

enum NormalizedArgParams {}

impl ast::ArgParams for NormalizedArgParams {
    type Id = spirv::Word;
    type Operand = ast::Operand<spirv::Word>;
    type CallOperand = ast::CallOperand<spirv::Word>;
    type IdOrVector = ast::IdOrVector<spirv::Word>;
    type OperandOrVector = ast::OperandOrVector<spirv::Word>;
    type SrcMemberOperand = (spirv::Word, u8);
}

impl ArgParamsEx for NormalizedArgParams {
    fn get_fn_decl<'a, 'b>(
        id: &Self::Id,
        decl: &'b GlobalFnDeclResolver<'a, 'b>,
    ) -> Result<&'b FnDecl, TranslateError> {
        decl.get_fn_decl(*id)
    }
}

type NormalizedStatement = Statement<
    (
        Option<ast::PredAt<spirv::Word>>,
        ast::Instruction<NormalizedArgParams>,
    ),
    NormalizedArgParams,
>;

type UnconditionalStatement = Statement<ast::Instruction<NormalizedArgParams>, NormalizedArgParams>;

enum TypedArgParams {}

impl ast::ArgParams for TypedArgParams {
    type Id = spirv::Word;
    type Operand = ast::Operand<spirv::Word>;
    type CallOperand = ast::CallOperand<spirv::Word>;
    type IdOrVector = ast::IdOrVector<spirv::Word>;
    type OperandOrVector = ast::OperandOrVector<spirv::Word>;
    type SrcMemberOperand = (spirv::Word, u8);
}

impl ArgParamsEx for TypedArgParams {
    fn get_fn_decl<'a, 'b>(
        id: &Self::Id,
        decl: &'b GlobalFnDeclResolver<'a, 'b>,
    ) -> Result<&'b FnDecl, TranslateError> {
        decl.get_fn_decl(*id)
    }
}

type TypedStatement = Statement<ast::Instruction<TypedArgParams>, TypedArgParams>;

enum ExpandedArgParams {}
type ExpandedStatement = Statement<ast::Instruction<ExpandedArgParams>, ExpandedArgParams>;

impl ast::ArgParams for ExpandedArgParams {
    type Id = spirv::Word;
    type Operand = spirv::Word;
    type CallOperand = spirv::Word;
    type IdOrVector = spirv::Word;
    type OperandOrVector = spirv::Word;
    type SrcMemberOperand = spirv::Word;
}

impl ArgParamsEx for ExpandedArgParams {
    fn get_fn_decl<'a, 'b>(
        id: &Self::Id,
        decl: &'b GlobalFnDeclResolver<'a, 'b>,
    ) -> Result<&'b FnDecl, TranslateError> {
        decl.get_fn_decl(*id)
    }
}

#[derive(Copy, Clone)]
pub enum StateSpace {
    Reg,
    Const,
    Global,
    Local,
    Shared,
    Param,
    ParamReg,
}

struct Function<'input> {
    pub func_directive: ast::MethodDecl<'input, spirv::Word>,
    pub globals: Vec<ExpandedStatement>,
    pub body: Option<Vec<ExpandedStatement>>,
}

pub trait ArgumentMapVisitor<T: ArgParamsEx, U: ArgParamsEx> {
    fn id(
        &mut self,
        desc: ArgumentDescriptor<T::Id>,
        typ: Option<ast::Type>,
    ) -> Result<U::Id, TranslateError>;
    fn operand(
        &mut self,
        desc: ArgumentDescriptor<T::Operand>,
        typ: ast::Type,
    ) -> Result<U::Operand, TranslateError>;
    fn id_or_vector(
        &mut self,
        desc: ArgumentDescriptor<T::IdOrVector>,
        typ: ast::Type,
    ) -> Result<U::IdOrVector, TranslateError>;
    fn operand_or_vector(
        &mut self,
        desc: ArgumentDescriptor<T::OperandOrVector>,
        typ: ast::Type,
    ) -> Result<U::OperandOrVector, TranslateError>;
    fn src_call_operand(
        &mut self,
        desc: ArgumentDescriptor<T::CallOperand>,
        typ: ast::Type,
    ) -> Result<U::CallOperand, TranslateError>;
    fn src_member_operand(
        &mut self,
        desc: ArgumentDescriptor<T::SrcMemberOperand>,
        typ: (ast::ScalarType, u8),
    ) -> Result<U::SrcMemberOperand, TranslateError>;
}

impl<T> ArgumentMapVisitor<ExpandedArgParams, ExpandedArgParams> for T
where
    T: FnMut(
        ArgumentDescriptor<spirv::Word>,
        Option<ast::Type>,
    ) -> Result<spirv::Word, TranslateError>,
{
    fn id(
        &mut self,
        desc: ArgumentDescriptor<spirv::Word>,
        t: Option<ast::Type>,
    ) -> Result<spirv::Word, TranslateError> {
        self(desc, t)
    }

    fn operand(
        &mut self,
        desc: ArgumentDescriptor<spirv::Word>,
        t: ast::Type,
    ) -> Result<spirv::Word, TranslateError> {
        self(desc, Some(t))
    }

    fn id_or_vector(
        &mut self,
        desc: ArgumentDescriptor<spirv::Word>,
        typ: ast::Type,
    ) -> Result<spirv::Word, TranslateError> {
        self(desc, Some(typ))
    }

    fn operand_or_vector(
        &mut self,
        desc: ArgumentDescriptor<spirv::Word>,
        typ: ast::Type,
    ) -> Result<spirv::Word, TranslateError> {
        self(desc, Some(typ))
    }

    fn src_call_operand(
        &mut self,
        desc: ArgumentDescriptor<spirv::Word>,
        t: ast::Type,
    ) -> Result<spirv::Word, TranslateError> {
        self(desc, Some(t))
    }

    fn src_member_operand(
        &mut self,
        desc: ArgumentDescriptor<spirv::Word>,
        (scalar_type, _): (ast::ScalarType, u8),
    ) -> Result<spirv::Word, TranslateError> {
        self(desc.new_op(desc.op), Some(ast::Type::Scalar(scalar_type)))
    }
}

impl<'a, T> ArgumentMapVisitor<ast::ParsedArgParams<'a>, NormalizedArgParams> for T
where
    T: FnMut(&str) -> Result<spirv::Word, TranslateError>,
{
    fn id(
        &mut self,
        desc: ArgumentDescriptor<&str>,
        _: Option<ast::Type>,
    ) -> Result<spirv::Word, TranslateError> {
        self(desc.op)
    }

    fn operand(
        &mut self,
        desc: ArgumentDescriptor<ast::Operand<&str>>,
        _: ast::Type,
    ) -> Result<ast::Operand<spirv::Word>, TranslateError> {
        match desc.op {
            ast::Operand::Reg(id) => Ok(ast::Operand::Reg(self(id)?)),
            ast::Operand::RegOffset(id, imm) => Ok(ast::Operand::RegOffset(self(id)?, imm)),
            ast::Operand::Imm(imm) => Ok(ast::Operand::Imm(imm)),
        }
    }

    fn id_or_vector(
        &mut self,
        desc: ArgumentDescriptor<ast::IdOrVector<&'a str>>,
        _: ast::Type,
    ) -> Result<ast::IdOrVector<spirv::Word>, TranslateError> {
        match desc.op {
            ast::IdOrVector::Reg(id) => Ok(ast::IdOrVector::Reg(self(id)?)),
            ast::IdOrVector::Vec(ids) => Ok(ast::IdOrVector::Vec(
                ids.into_iter().map(self).collect::<Result<_, _>>()?,
            )),
        }
    }

    fn operand_or_vector(
        &mut self,
        desc: ArgumentDescriptor<ast::OperandOrVector<&'a str>>,
        _: ast::Type,
    ) -> Result<ast::OperandOrVector<spirv::Word>, TranslateError> {
        match desc.op {
            ast::OperandOrVector::Reg(id) => Ok(ast::OperandOrVector::Reg(self(id)?)),
            ast::OperandOrVector::RegOffset(id, imm) => {
                Ok(ast::OperandOrVector::RegOffset(self(id)?, imm))
            }
            ast::OperandOrVector::Imm(imm) => Ok(ast::OperandOrVector::Imm(imm)),
            ast::OperandOrVector::Vec(ids) => Ok(ast::OperandOrVector::Vec(
                ids.into_iter().map(self).collect::<Result<_, _>>()?,
            )),
        }
    }

    fn src_call_operand(
        &mut self,
        desc: ArgumentDescriptor<ast::CallOperand<&str>>,
        _: ast::Type,
    ) -> Result<ast::CallOperand<spirv::Word>, TranslateError> {
        match desc.op {
            ast::CallOperand::Reg(id) => Ok(ast::CallOperand::Reg(self(id)?)),
            ast::CallOperand::Imm(imm) => Ok(ast::CallOperand::Imm(imm)),
        }
    }

    fn src_member_operand(
        &mut self,
        desc: ArgumentDescriptor<(&str, u8)>,
        _: (ast::ScalarType, u8),
    ) -> Result<(spirv::Word, u8), TranslateError> {
        Ok((self(desc.op.0)?, desc.op.1))
    }
}

pub struct ArgumentDescriptor<Op> {
    op: Op,
    is_dst: bool,
    sema: ArgumentSemantics,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ArgumentSemantics {
    // normal register access
    Default,
    // normal register access with relaxed conversion rules (ld/st)
    DefaultRelaxed,
    // st/ld global
    PhysicalPointer,
    // st/ld .param, .local
    RegisterPointer,
    // mov of .local/.global variables
    Address,
}

impl<T> ArgumentDescriptor<T> {
    fn new_op<U>(&self, u: U) -> ArgumentDescriptor<U> {
        ArgumentDescriptor {
            op: u,
            is_dst: self.is_dst,
            sema: self.sema,
        }
    }
}

impl<T: ArgParamsEx> ast::Instruction<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
    ) -> Result<ast::Instruction<U>, TranslateError> {
        Ok(match self {
            ast::Instruction::Abs(d, arg) => {
                ast::Instruction::Abs(d, arg.map(visitor, false, ast::Type::Scalar(d.typ))?)
            }
            // Call instruction is converted to a call statement early on
            ast::Instruction::Call(_) => return Err(TranslateError::Unreachable),
            ast::Instruction::Ld(d, a) => {
                let inst_type = d.typ;
                let is_param = d.state_space == ast::LdStateSpace::Param
                    || d.state_space == ast::LdStateSpace::Local;
                ast::Instruction::Ld(d, a.map(visitor, inst_type, is_param)?)
            }
            ast::Instruction::Mov(d, a) => {
                let mapped = a.map(visitor, d)?;
                ast::Instruction::Mov(d, mapped)
            }
            ast::Instruction::Mul(d, a) => {
                let inst_type = d.get_type();
                let is_wide = d.is_wide();
                ast::Instruction::Mul(d, a.map_non_shift(visitor, inst_type, is_wide)?)
            }
            ast::Instruction::Add(d, a) => {
                let inst_type = d.get_type();
                ast::Instruction::Add(d, a.map_non_shift(visitor, inst_type, false)?)
            }
            ast::Instruction::Setp(d, a) => {
                let inst_type = d.typ;
                ast::Instruction::Setp(d, a.map(visitor, ast::Type::Scalar(inst_type))?)
            }
            ast::Instruction::SetpBool(d, a) => {
                let inst_type = d.typ;
                ast::Instruction::SetpBool(d, a.map(visitor, ast::Type::Scalar(inst_type))?)
            }
            ast::Instruction::Not(t, a) => {
                ast::Instruction::Not(t, a.map(visitor, false, t.to_type())?)
            }
            ast::Instruction::Cvt(d, a) => {
                let (dst_t, src_t) = match &d {
                    ast::CvtDetails::FloatFromFloat(desc) => (
                        ast::Type::Scalar(desc.dst.into()),
                        ast::Type::Scalar(desc.src.into()),
                    ),
                    ast::CvtDetails::FloatFromInt(desc) => (
                        ast::Type::Scalar(desc.dst.into()),
                        ast::Type::Scalar(desc.src.into()),
                    ),
                    ast::CvtDetails::IntFromFloat(desc) => (
                        ast::Type::Scalar(desc.dst.into()),
                        ast::Type::Scalar(desc.src.into()),
                    ),
                    ast::CvtDetails::IntFromInt(desc) => (
                        ast::Type::Scalar(desc.dst.into()),
                        ast::Type::Scalar(desc.src.into()),
                    ),
                };
                ast::Instruction::Cvt(d, a.map_cvt(visitor, dst_t, src_t)?)
            }
            ast::Instruction::Shl(t, a) => {
                ast::Instruction::Shl(t, a.map_shift(visitor, t.to_type())?)
            }
            ast::Instruction::Shr(t, a) => {
                ast::Instruction::Shr(t, a.map_shift(visitor, ast::Type::Scalar(t.into()))?)
            }
            ast::Instruction::St(d, a) => {
                let inst_type = d.typ;
                let is_param = d.state_space == ast::StStateSpace::Param
                    || d.state_space == ast::StStateSpace::Local;
                ast::Instruction::St(d, a.map(visitor, inst_type, is_param)?)
            }
            ast::Instruction::Bra(d, a) => ast::Instruction::Bra(d, a.map(visitor, None)?),
            ast::Instruction::Ret(d) => ast::Instruction::Ret(d),
            ast::Instruction::Cvta(d, a) => {
                let inst_type = ast::Type::Scalar(ast::ScalarType::B64);
                ast::Instruction::Cvta(d, a.map(visitor, false, inst_type)?)
            }
            ast::Instruction::Mad(d, a) => {
                let inst_type = d.get_type();
                let is_wide = d.is_wide();
                ast::Instruction::Mad(d, a.map(visitor, inst_type, is_wide)?)
            }
        })
    }
}

impl VisitVariable for ast::Instruction<TypedArgParams> {
    fn visit_variable<
        'a,
        F: FnMut(
            ArgumentDescriptor<spirv_headers::Word>,
            Option<ast::Type>,
        ) -> Result<spirv_headers::Word, TranslateError>,
    >(
        self,
        f: &mut F,
    ) -> Result<TypedStatement, TranslateError> {
        Ok(Statement::Instruction(self.map(f)?))
    }
}

impl<T> ArgumentMapVisitor<TypedArgParams, TypedArgParams> for T
where
    T: FnMut(
        ArgumentDescriptor<spirv::Word>,
        Option<ast::Type>,
    ) -> Result<spirv::Word, TranslateError>,
{
    fn id(
        &mut self,
        desc: ArgumentDescriptor<spirv::Word>,
        t: Option<ast::Type>,
    ) -> Result<spirv::Word, TranslateError> {
        self(desc, t)
    }

    fn operand(
        &mut self,
        desc: ArgumentDescriptor<ast::Operand<spirv::Word>>,
        t: ast::Type,
    ) -> Result<ast::Operand<spirv::Word>, TranslateError> {
        match desc.op {
            ast::Operand::Reg(id) => Ok(ast::Operand::Reg(self(desc.new_op(id), Some(t))?)),
            ast::Operand::Imm(imm) => Ok(ast::Operand::Imm(imm)),
            ast::Operand::RegOffset(id, imm) => Ok(ast::Operand::RegOffset(
                self(desc.new_op(id), Some(t))?,
                imm,
            )),
        }
    }

    fn src_call_operand(
        &mut self,
        desc: ArgumentDescriptor<ast::CallOperand<spirv::Word>>,
        t: ast::Type,
    ) -> Result<ast::CallOperand<spirv::Word>, TranslateError> {
        match desc.op {
            ast::CallOperand::Reg(id) => Ok(ast::CallOperand::Reg(self(desc.new_op(id), Some(t))?)),
            ast::CallOperand::Imm(imm) => Ok(ast::CallOperand::Imm(imm)),
        }
    }

    fn id_or_vector(
        &mut self,
        desc: ArgumentDescriptor<ast::IdOrVector<spirv::Word>>,
        typ: ast::Type,
    ) -> Result<ast::IdOrVector<spirv::Word>, TranslateError> {
        match desc.op {
            ast::IdOrVector::Reg(id) => Ok(ast::IdOrVector::Reg(self(desc.new_op(id), Some(typ))?)),
            ast::IdOrVector::Vec(ref ids) => Ok(ast::IdOrVector::Vec(
                ids.iter()
                    .map(|id| self(desc.new_op(*id), Some(typ)))
                    .collect::<Result<_, _>>()?,
            )),
        }
    }

    fn operand_or_vector(
        &mut self,
        desc: ArgumentDescriptor<ast::OperandOrVector<spirv::Word>>,
        typ: ast::Type,
    ) -> Result<ast::OperandOrVector<spirv::Word>, TranslateError> {
        match desc.op {
            ast::OperandOrVector::Reg(id) => {
                Ok(ast::OperandOrVector::Reg(self(desc.new_op(id), Some(typ))?))
            }
            ast::OperandOrVector::RegOffset(id, imm) => Ok(ast::OperandOrVector::RegOffset(
                self(desc.new_op(id), Some(typ))?,
                imm,
            )),
            ast::OperandOrVector::Imm(imm) => Ok(ast::OperandOrVector::Imm(imm)),
            ast::OperandOrVector::Vec(ref ids) => Ok(ast::OperandOrVector::Vec(
                ids.iter()
                    .map(|id| self(desc.new_op(*id), Some(typ)))
                    .collect::<Result<_, _>>()?,
            )),
        }
    }

    fn src_member_operand(
        &mut self,
        desc: ArgumentDescriptor<(spirv::Word, u8)>,
        (scalar_type, vector_len): (ast::ScalarType, u8),
    ) -> Result<(spirv::Word, u8), TranslateError> {
        Ok((
            self(
                desc.new_op(desc.op.0),
                Some(ast::Type::Vector(scalar_type.into(), vector_len)),
            )?,
            desc.op.1,
        ))
    }
}

impl ast::Type {
    fn widen(self) -> Result<Self, TranslateError> {
        match self {
            ast::Type::Scalar(scalar) => {
                let kind = scalar.kind();
                let width = scalar.width();
                if (kind != ScalarKind::Signed
                    && kind != ScalarKind::Unsigned
                    && kind != ScalarKind::Bit)
                    || (width == 8)
                {
                    return Err(TranslateError::MismatchedType);
                }
                Ok(ast::Type::Scalar(ast::ScalarType::from_parts(
                    width * 2,
                    kind,
                )))
            }
            _ => Err(TranslateError::Unreachable),
        }
    }

    fn to_parts(self) -> TypeParts {
        match self {
            ast::Type::Scalar(scalar) => TypeParts {
                kind: TypeKind::Scalar,
                scalar_kind: scalar.kind(),
                width: scalar.width(),
                components: 0,
            },
            ast::Type::Vector(scalar, components) => TypeParts {
                kind: TypeKind::Vector,
                scalar_kind: scalar.kind(),
                width: scalar.width(),
                components: components as u32,
            },
            ast::Type::Array(scalar, components) => TypeParts {
                kind: TypeKind::Array,
                scalar_kind: scalar.kind(),
                width: scalar.width(),
                components: components,
            },
        }
    }

    fn from_parts(t: TypeParts) -> Self {
        match t.kind {
            TypeKind::Scalar => {
                ast::Type::Scalar(ast::ScalarType::from_parts(t.width, t.scalar_kind))
            }
            TypeKind::Vector => ast::Type::Vector(
                ast::ScalarType::from_parts(t.width, t.scalar_kind),
                t.components as u8,
            ),
            TypeKind::Array => ast::Type::Array(
                ast::ScalarType::from_parts(t.width, t.scalar_kind),
                t.components,
            ),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct TypeParts {
    kind: TypeKind,
    scalar_kind: ScalarKind,
    width: u8,
    components: u32,
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum TypeKind {
    Scalar,
    Vector,
    Array,
}

impl ast::Instruction<ExpandedArgParams> {
    fn jump_target(&self) -> Option<spirv::Word> {
        match self {
            ast::Instruction::Bra(_, a) => Some(a.src),
            ast::Instruction::Ld(_, _)
            | ast::Instruction::Mov(_, _)
            | ast::Instruction::Mul(_, _)
            | ast::Instruction::Add(_, _)
            | ast::Instruction::Setp(_, _)
            | ast::Instruction::SetpBool(_, _)
            | ast::Instruction::Not(_, _)
            | ast::Instruction::Cvt(_, _)
            | ast::Instruction::Cvta(_, _)
            | ast::Instruction::Shl(_, _)
            | ast::Instruction::Shr(_, _)
            | ast::Instruction::St(_, _)
            | ast::Instruction::Ret(_)
            | ast::Instruction::Abs(_, _)
            | ast::Instruction::Call(_)
            | ast::Instruction::Mad(_, _) => None,
        }
    }
}

impl VisitVariableExpanded for ast::Instruction<ExpandedArgParams> {
    fn visit_variable_extended<
        F: FnMut(
            ArgumentDescriptor<spirv_headers::Word>,
            Option<ast::Type>,
        ) -> Result<spirv_headers::Word, TranslateError>,
    >(
        self,
        f: &mut F,
    ) -> Result<ExpandedStatement, TranslateError> {
        Ok(Statement::Instruction(self.map(f)?))
    }
}

type Arg2 = ast::Arg2<ExpandedArgParams>;
type Arg2St = ast::Arg2St<ExpandedArgParams>;

struct CompositeRead {
    pub typ: ast::ScalarType,
    pub dst: spirv::Word,
    pub dst_semantics_override: Option<ArgumentSemantics>,
    pub src_composite: spirv::Word,
    pub src_index: u32,
    pub src_len: u32,
}

impl VisitVariableExpanded for CompositeRead {
    fn visit_variable_extended<
        F: FnMut(
            ArgumentDescriptor<spirv_headers::Word>,
            Option<ast::Type>,
        ) -> Result<spirv_headers::Word, TranslateError>,
    >(
        self,
        f: &mut F,
    ) -> Result<ExpandedStatement, TranslateError> {
        let dst_sema = self
            .dst_semantics_override
            .unwrap_or(ArgumentSemantics::Default);
        Ok(Statement::Composite(CompositeRead {
            dst: f(
                ArgumentDescriptor {
                    op: self.dst,
                    is_dst: true,
                    sema: dst_sema,
                },
                Some(ast::Type::Scalar(self.typ)),
            )?,
            src_composite: f(
                ArgumentDescriptor {
                    op: self.src_composite,
                    is_dst: false,
                    sema: ArgumentSemantics::Default,
                },
                Some(ast::Type::Vector(self.typ, self.src_len as u8)),
            )?,
            ..self
        }))
    }
}

struct ConstantDefinition {
    pub dst: spirv::Word,
    pub typ: ast::ScalarType,
    pub value: i64,
}

struct BrachCondition {
    predicate: spirv::Word,
    if_true: spirv::Word,
    if_false: spirv::Word,
}

#[derive(Copy, Clone)]
struct ImplicitConversion {
    src: spirv::Word,
    dst: spirv::Word,
    from: ast::Type,
    to: ast::Type,
    kind: ConversionKind,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ConversionKind {
    Default,
    // zero-extend/chop/bitcast depending on types
    SignExtend,
    BitToPtr(ast::LdStateSpace),
    PtrToBit,
}

impl<T> ast::PredAt<T> {
    fn map_variable<U, F: FnMut(T) -> Result<U, TranslateError>>(
        self,
        f: &mut F,
    ) -> Result<ast::PredAt<U>, TranslateError> {
        let new_label = f(self.label)?;
        Ok(ast::PredAt {
            not: self.not,
            label: new_label,
        })
    }
}

impl<'a> ast::Instruction<ast::ParsedArgParams<'a>> {
    fn map_variable<F: FnMut(&str) -> Result<spirv::Word, TranslateError>>(
        self,
        f: &mut F,
    ) -> Result<ast::Instruction<NormalizedArgParams>, TranslateError> {
        match self {
            ast::Instruction::Call(call) => {
                let call_inst = ast::CallInst {
                    uniform: call.uniform,
                    ret_params: call
                        .ret_params
                        .into_iter()
                        .map(|p| f(p))
                        .collect::<Result<_, _>>()?,
                    func: f(call.func)?,
                    param_list: call
                        .param_list
                        .into_iter()
                        .map(|p| p.map_variable(f))
                        .collect::<Result<_, _>>()?,
                };
                Ok(ast::Instruction::Call(call_inst))
            }
            i => i.map(f),
        }
    }
}

impl ast::VariableParamType {
    fn width(self) -> usize {
        match self {
            ast::VariableParamType::Scalar(t) => ast::ScalarType::from(t).width() as usize,
            ast::VariableParamType::Array(t, len) => {
                (ast::ScalarType::from(t).width() as usize) * (len as usize)
            }
        }
    }
}

impl<T: ArgParamsEx> ast::Arg1<T> {
    fn cast<U: ArgParamsEx<Id = T::Id>>(self) -> ast::Arg1<U> {
        ast::Arg1 { src: self.src }
    }

    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: Option<ast::Type>,
    ) -> Result<ast::Arg1<U>, TranslateError> {
        let new_src = visitor.id(
            ArgumentDescriptor {
                op: self.src,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            t,
        )?;
        Ok(ast::Arg1 { src: new_src })
    }
}

impl<T: ArgParamsEx> ast::Arg2<T> {
    fn cast<U: ArgParamsEx<Id = T::Id, Operand = T::Operand>>(self) -> ast::Arg2<U> {
        ast::Arg2 {
            src: self.src,
            dst: self.dst,
        }
    }

    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        src_is_addr: bool,
        t: ast::Type,
    ) -> Result<ast::Arg2<U>, TranslateError> {
        let new_dst = visitor.id(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                sema: ArgumentSemantics::Default,
            },
            Some(t),
        )?;
        let new_src = visitor.operand(
            ArgumentDescriptor {
                op: self.src,
                is_dst: false,
                sema: if src_is_addr {
                    ArgumentSemantics::Address
                } else {
                    ArgumentSemantics::Default
                },
            },
            t,
        )?;
        Ok(ast::Arg2 {
            dst: new_dst,
            src: new_src,
        })
    }

    fn map_cvt<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        dst_t: ast::Type,
        src_t: ast::Type,
    ) -> Result<ast::Arg2<U>, TranslateError> {
        let dst = visitor.id(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                sema: ArgumentSemantics::Default,
            },
            Some(dst_t),
        )?;
        let src = visitor.operand(
            ArgumentDescriptor {
                op: self.src,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            src_t,
        )?;
        Ok(ast::Arg2 { dst, src })
    }
}

impl<T: ArgParamsEx> ast::Arg2Ld<T> {
    fn cast<U: ArgParamsEx<Operand = T::Operand, IdOrVector = T::IdOrVector>>(
        self,
    ) -> ast::Arg2Ld<U> {
        ast::Arg2Ld {
            dst: self.dst,
            src: self.src,
        }
    }

    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: ast::Type,
        is_param: bool,
    ) -> Result<ast::Arg2Ld<U>, TranslateError> {
        let dst = visitor.id_or_vector(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                sema: ArgumentSemantics::DefaultRelaxed,
            },
            t.into(),
        )?;
        let src = visitor.operand(
            ArgumentDescriptor {
                op: self.src,
                is_dst: false,
                sema: if is_param {
                    ArgumentSemantics::RegisterPointer
                } else {
                    ArgumentSemantics::PhysicalPointer
                },
            },
            t,
        )?;
        Ok(ast::Arg2Ld { dst, src })
    }
}

impl<T: ArgParamsEx> ast::Arg2St<T> {
    fn cast<U: ArgParamsEx<Operand = T::Operand, OperandOrVector = T::OperandOrVector>>(
        self,
    ) -> ast::Arg2St<U> {
        ast::Arg2St {
            src1: self.src1,
            src2: self.src2,
        }
    }

    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: ast::Type,
        is_param: bool,
    ) -> Result<ast::Arg2St<U>, TranslateError> {
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: is_param,
                sema: if is_param {
                    ArgumentSemantics::RegisterPointer
                } else {
                    ArgumentSemantics::PhysicalPointer
                },
            },
            t,
        )?;
        let src2 = visitor.operand_or_vector(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                sema: ArgumentSemantics::DefaultRelaxed,
            },
            t,
        )?;
        Ok(ast::Arg2St { src1, src2 })
    }
}

impl<T: ArgParamsEx> ast::Arg2Mov<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        details: ast::MovDetails,
    ) -> Result<ast::Arg2Mov<U>, TranslateError> {
        Ok(match self {
            ast::Arg2Mov::Normal(arg) => ast::Arg2Mov::Normal(arg.map(visitor, details)?),
            ast::Arg2Mov::Member(arg) => ast::Arg2Mov::Member(arg.map(visitor, details)?),
        })
    }
}

impl<P: ArgParamsEx> ast::Arg2MovNormal<P> {
    fn cast<U: ArgParamsEx<IdOrVector = P::IdOrVector, OperandOrVector = P::OperandOrVector>>(
        self,
    ) -> ast::Arg2MovNormal<U> {
        ast::Arg2MovNormal {
            dst: self.dst,
            src: self.src,
        }
    }

    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<P, U>>(
        self,
        visitor: &mut V,
        details: ast::MovDetails,
    ) -> Result<ast::Arg2MovNormal<U>, TranslateError> {
        let dst = visitor.id_or_vector(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                sema: ArgumentSemantics::Default,
            },
            details.typ.into(),
        )?;
        let src = visitor.operand_or_vector(
            ArgumentDescriptor {
                op: self.src,
                is_dst: false,
                sema: if details.src_is_address {
                    ArgumentSemantics::Address
                } else {
                    ArgumentSemantics::Default
                },
            },
            details.typ.into(),
        )?;
        Ok(ast::Arg2MovNormal { dst, src })
    }
}

impl<T: ArgParamsEx> ast::Arg2MovMember<T> {
    fn cast<U: ArgParamsEx<Id = T::Id, SrcMemberOperand = T::SrcMemberOperand>>(
        self,
    ) -> ast::Arg2MovMember<U> {
        match self {
            ast::Arg2MovMember::Dst(dst, src1, src2) => ast::Arg2MovMember::Dst(dst, src1, src2),
            ast::Arg2MovMember::Src(dst, src) => ast::Arg2MovMember::Src(dst, src),
            ast::Arg2MovMember::Both(dst, src1, src2) => ast::Arg2MovMember::Both(dst, src1, src2),
        }
    }

    fn vector_dst(&self) -> Option<&T::Id> {
        match self {
            ast::Arg2MovMember::Src(_, _) => None,
            ast::Arg2MovMember::Dst((d, _), _, _) | ast::Arg2MovMember::Both((d, _), _, _) => {
                Some(d)
            }
        }
    }

    fn vector_src(&self) -> Option<&T::SrcMemberOperand> {
        match self {
            ast::Arg2MovMember::Src(_, d) | ast::Arg2MovMember::Both(_, _, d) => Some(d),
            ast::Arg2MovMember::Dst(_, _, _) => None,
        }
    }
}

impl<T: ArgParamsEx> ast::Arg2MovMember<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        details: ast::MovDetails,
    ) -> Result<ast::Arg2MovMember<U>, TranslateError> {
        match self {
            ast::Arg2MovMember::Dst((dst, len), composite_src, scalar_src) => {
                let scalar_type = details.typ.get_scalar()?;
                let dst = visitor.id(
                    ArgumentDescriptor {
                        op: dst,
                        is_dst: true,
                        sema: ArgumentSemantics::Default,
                    },
                    Some(ast::Type::Vector(scalar_type, details.dst_width)),
                )?;
                let src1 = visitor.id(
                    ArgumentDescriptor {
                        op: composite_src,
                        is_dst: false,
                        sema: ArgumentSemantics::Default,
                    },
                    Some(ast::Type::Vector(scalar_type, details.dst_width)),
                )?;
                let src2 = visitor.id(
                    ArgumentDescriptor {
                        op: scalar_src,
                        is_dst: false,
                        sema: if details.src_is_address {
                            ArgumentSemantics::Address
                        } else if details.relaxed_src2_conv {
                            ArgumentSemantics::DefaultRelaxed
                        } else {
                            ArgumentSemantics::Default
                        },
                    },
                    Some(details.typ.into()),
                )?;
                Ok(ast::Arg2MovMember::Dst((dst, len), src1, src2))
            }
            ast::Arg2MovMember::Src(dst, src) => {
                let dst = visitor.id(
                    ArgumentDescriptor {
                        op: dst,
                        is_dst: true,
                        sema: ArgumentSemantics::Default,
                    },
                    Some(details.typ.into()),
                )?;
                let scalar_typ = details.typ.get_scalar()?;
                let src = visitor.src_member_operand(
                    ArgumentDescriptor {
                        op: src,
                        is_dst: false,
                        sema: ArgumentSemantics::Default,
                    },
                    (scalar_typ.into(), details.src_width),
                )?;
                Ok(ast::Arg2MovMember::Src(dst, src))
            }
            ast::Arg2MovMember::Both((dst, len), composite_src, src) => {
                let scalar_type = details.typ.get_scalar()?;
                let dst = visitor.id(
                    ArgumentDescriptor {
                        op: dst,
                        is_dst: true,
                        sema: ArgumentSemantics::Default,
                    },
                    Some(ast::Type::Vector(scalar_type, details.dst_width)),
                )?;
                let composite_src = visitor.id(
                    ArgumentDescriptor {
                        op: composite_src,
                        is_dst: false,
                        sema: ArgumentSemantics::Default,
                    },
                    Some(ast::Type::Vector(scalar_type, details.dst_width)),
                )?;
                let src = visitor.src_member_operand(
                    ArgumentDescriptor {
                        op: src,
                        is_dst: false,
                        sema: if details.relaxed_src2_conv {
                            ArgumentSemantics::DefaultRelaxed
                        } else {
                            ArgumentSemantics::Default
                        },
                    },
                    (scalar_type.into(), details.src_width),
                )?;
                Ok(ast::Arg2MovMember::Both((dst, len), composite_src, src))
            }
        }
    }
}

impl<T: ArgParamsEx> ast::Arg3<T> {
    fn cast<U: ArgParamsEx<Id = T::Id, Operand = T::Operand>>(self) -> ast::Arg3<U> {
        ast::Arg3 {
            dst: self.dst,
            src1: self.src1,
            src2: self.src2,
        }
    }

    fn map_non_shift<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        typ: ast::Type,
        is_wide: bool,
    ) -> Result<ast::Arg3<U>, TranslateError> {
        let dst = visitor.id(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                sema: ArgumentSemantics::Default,
            },
            Some(if is_wide { typ.widen()? } else { typ }),
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            typ,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            typ,
        )?;
        Ok(ast::Arg3 { dst, src1, src2 })
    }

    fn map_shift<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: ast::Type,
    ) -> Result<ast::Arg3<U>, TranslateError> {
        let dst = visitor.id(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                sema: ArgumentSemantics::Default,
            },
            Some(t),
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            t,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            ast::Type::Scalar(ast::ScalarType::U32),
        )?;
        Ok(ast::Arg3 { dst, src1, src2 })
    }
}

impl<T: ArgParamsEx> ast::Arg4<T> {
    fn cast<U: ArgParamsEx<Id = T::Id, Operand = T::Operand>>(self) -> ast::Arg4<U> {
        ast::Arg4 {
            dst: self.dst,
            src1: self.src1,
            src2: self.src2,
            src3: self.src3,
        }
    }

    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: ast::Type,
        is_wide: bool,
    ) -> Result<ast::Arg4<U>, TranslateError> {
        let dst = visitor.id(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                sema: ArgumentSemantics::Default,
            },
            Some(if is_wide { t.widen()? } else { t }),
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            t,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            t,
        )?;
        let src3 = visitor.operand(
            ArgumentDescriptor {
                op: self.src3,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            t,
        )?;
        Ok(ast::Arg4 {
            dst,
            src1,
            src2,
            src3,
        })
    }
}

impl<T: ArgParamsEx> ast::Arg4Setp<T> {
    fn cast<U: ArgParamsEx<Id = T::Id, Operand = T::Operand>>(self) -> ast::Arg4Setp<U> {
        ast::Arg4Setp {
            dst1: self.dst1,
            dst2: self.dst2,
            src1: self.src1,
            src2: self.src2,
        }
    }

    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: ast::Type,
    ) -> Result<ast::Arg4Setp<U>, TranslateError> {
        let dst1 = visitor.id(
            ArgumentDescriptor {
                op: self.dst1,
                is_dst: true,
                sema: ArgumentSemantics::Default,
            },
            Some(ast::Type::Scalar(ast::ScalarType::Pred)),
        )?;
        let dst2 = self
            .dst2
            .map(|dst2| {
                visitor.id(
                    ArgumentDescriptor {
                        op: dst2,
                        is_dst: true,
                        sema: ArgumentSemantics::Default,
                    },
                    Some(ast::Type::Scalar(ast::ScalarType::Pred)),
                )
            })
            .transpose()?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            t,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            t,
        )?;
        Ok(ast::Arg4Setp {
            dst1,
            dst2,
            src1,
            src2,
        })
    }
}

impl<T: ArgParamsEx> ast::Arg5<T> {
    fn cast<U: ArgParamsEx<Id = T::Id, Operand = T::Operand>>(self) -> ast::Arg5<U> {
        ast::Arg5 {
            dst1: self.dst1,
            dst2: self.dst2,
            src1: self.src1,
            src2: self.src2,
            src3: self.src3,
        }
    }

    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: ast::Type,
    ) -> Result<ast::Arg5<U>, TranslateError> {
        let dst1 = visitor.id(
            ArgumentDescriptor {
                op: self.dst1,
                is_dst: true,
                sema: ArgumentSemantics::Default,
            },
            Some(ast::Type::Scalar(ast::ScalarType::Pred)),
        )?;
        let dst2 = self
            .dst2
            .map(|dst2| {
                visitor.id(
                    ArgumentDescriptor {
                        op: dst2,
                        is_dst: true,
                        sema: ArgumentSemantics::Default,
                    },
                    Some(ast::Type::Scalar(ast::ScalarType::Pred)),
                )
            })
            .transpose()?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            t,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            t,
        )?;
        let src3 = visitor.operand(
            ArgumentDescriptor {
                op: self.src3,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            ast::Type::Scalar(ast::ScalarType::Pred),
        )?;
        Ok(ast::Arg5 {
            dst1,
            dst2,
            src1,
            src2,
            src3,
        })
    }
}

impl ast::Type {
    fn get_vector(self) -> Result<(ast::ScalarType, u8), TranslateError> {
        match self {
            ast::Type::Vector(t, len) => Ok((t, len)),
            _ => Err(TranslateError::MismatchedType),
        }
    }

    fn get_scalar(self) -> Result<ast::ScalarType, TranslateError> {
        match self {
            ast::Type::Scalar(t) => Ok(t),
            _ => Err(TranslateError::MismatchedType),
        }
    }
}

impl<T> ast::CallOperand<T> {
    fn map_variable<U, F: FnMut(T) -> Result<U, TranslateError>>(
        self,
        f: &mut F,
    ) -> Result<ast::CallOperand<U>, TranslateError> {
        match self {
            ast::CallOperand::Reg(id) => Ok(ast::CallOperand::Reg(f(id)?)),
            ast::CallOperand::Imm(x) => Ok(ast::CallOperand::Imm(x)),
        }
    }
}

impl ast::StStateSpace {
    fn to_ld_ss(self) -> ast::LdStateSpace {
        match self {
            ast::StStateSpace::Generic => ast::LdStateSpace::Generic,
            ast::StStateSpace::Global => ast::LdStateSpace::Global,
            ast::StStateSpace::Local => ast::LdStateSpace::Local,
            ast::StStateSpace::Param => ast::LdStateSpace::Param,
            ast::StStateSpace::Shared => ast::LdStateSpace::Shared,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ScalarKind {
    Bit,
    Unsigned,
    Signed,
    Float,
    Float2,
    Pred,
}

impl ast::ScalarType {
    fn width(self) -> u8 {
        match self {
            ast::ScalarType::U8 => 1,
            ast::ScalarType::S8 => 1,
            ast::ScalarType::B8 => 1,
            ast::ScalarType::U16 => 2,
            ast::ScalarType::S16 => 2,
            ast::ScalarType::B16 => 2,
            ast::ScalarType::F16 => 2,
            ast::ScalarType::U32 => 4,
            ast::ScalarType::S32 => 4,
            ast::ScalarType::B32 => 4,
            ast::ScalarType::F32 => 4,
            ast::ScalarType::U64 => 8,
            ast::ScalarType::S64 => 8,
            ast::ScalarType::B64 => 8,
            ast::ScalarType::F64 => 8,
            ast::ScalarType::F16x2 => 4,
            ast::ScalarType::Pred => 1,
        }
    }

    fn kind(self) -> ScalarKind {
        match self {
            ast::ScalarType::U8 => ScalarKind::Unsigned,
            ast::ScalarType::U16 => ScalarKind::Unsigned,
            ast::ScalarType::U32 => ScalarKind::Unsigned,
            ast::ScalarType::U64 => ScalarKind::Unsigned,
            ast::ScalarType::S8 => ScalarKind::Signed,
            ast::ScalarType::S16 => ScalarKind::Signed,
            ast::ScalarType::S32 => ScalarKind::Signed,
            ast::ScalarType::S64 => ScalarKind::Signed,
            ast::ScalarType::B8 => ScalarKind::Bit,
            ast::ScalarType::B16 => ScalarKind::Bit,
            ast::ScalarType::B32 => ScalarKind::Bit,
            ast::ScalarType::B64 => ScalarKind::Bit,
            ast::ScalarType::F16 => ScalarKind::Float,
            ast::ScalarType::F32 => ScalarKind::Float,
            ast::ScalarType::F64 => ScalarKind::Float,
            ast::ScalarType::F16x2 => ScalarKind::Float,
            ast::ScalarType::Pred => ScalarKind::Pred,
        }
    }

    fn from_parts(width: u8, kind: ScalarKind) -> Self {
        match kind {
            ScalarKind::Float => match width {
                2 => ast::ScalarType::F16,
                4 => ast::ScalarType::F32,
                8 => ast::ScalarType::F64,
                _ => unreachable!(),
            },
            ScalarKind::Bit => match width {
                1 => ast::ScalarType::B8,
                2 => ast::ScalarType::B16,
                4 => ast::ScalarType::B32,
                8 => ast::ScalarType::B64,
                _ => unreachable!(),
            },
            ScalarKind::Signed => match width {
                1 => ast::ScalarType::S8,
                2 => ast::ScalarType::S16,
                4 => ast::ScalarType::S32,
                8 => ast::ScalarType::S64,
                _ => unreachable!(),
            },
            ScalarKind::Unsigned => match width {
                1 => ast::ScalarType::U8,
                2 => ast::ScalarType::U16,
                4 => ast::ScalarType::U32,
                8 => ast::ScalarType::U64,
                _ => unreachable!(),
            },
            ScalarKind::Float2 => match width {
                4 => ast::ScalarType::F16x2,
                _ => unreachable!(),
            },
            ScalarKind::Pred => ast::ScalarType::Pred,
        }
    }
}

impl ast::NotType {
    fn to_type(self) -> ast::Type {
        match self {
            ast::NotType::Pred => ast::Type::Scalar(ast::ScalarType::Pred),
            ast::NotType::B16 => ast::Type::Scalar(ast::ScalarType::B16),
            ast::NotType::B32 => ast::Type::Scalar(ast::ScalarType::B32),
            ast::NotType::B64 => ast::Type::Scalar(ast::ScalarType::B64),
        }
    }
}

impl ast::ShlType {
    fn to_type(self) -> ast::Type {
        match self {
            ast::ShlType::B16 => ast::Type::Scalar(ast::ScalarType::B16),
            ast::ShlType::B32 => ast::Type::Scalar(ast::ScalarType::B32),
            ast::ShlType::B64 => ast::Type::Scalar(ast::ScalarType::B64),
        }
    }
}

impl ast::ShrType {
    fn signed(&self) -> bool {
        match self {
            ast::ShrType::S16 | ast::ShrType::S32 | ast::ShrType::S64 => true,
            _ => false,
        }
    }
}

impl ast::AddDetails {
    fn get_type(&self) -> ast::Type {
        match self {
            ast::AddDetails::Int(ast::AddIntDesc { typ, .. }) => ast::Type::Scalar((*typ).into()),
            ast::AddDetails::Float(ast::AddFloatDesc { typ, .. }) => {
                ast::Type::Scalar((*typ).into())
            }
        }
    }
}

impl ast::MulDetails {
    fn get_type(&self) -> ast::Type {
        match self {
            ast::MulDetails::Int(ast::MulIntDesc { typ, .. }) => ast::Type::Scalar((*typ).into()),
            ast::MulDetails::Float(ast::MulFloatDesc { typ, .. }) => {
                ast::Type::Scalar((*typ).into())
            }
        }
    }
}

impl ast::IntType {
    fn try_new(t: ast::ScalarType) -> Option<Self> {
        match t {
            ast::ScalarType::U16 => Some(ast::IntType::U16),
            ast::ScalarType::U32 => Some(ast::IntType::U32),
            ast::ScalarType::U64 => Some(ast::IntType::U64),
            ast::ScalarType::S16 => Some(ast::IntType::S16),
            ast::ScalarType::S32 => Some(ast::IntType::S32),
            ast::ScalarType::S64 => Some(ast::IntType::S64),
            _ => None,
        }
    }
}

impl ast::LdStateSpace {
    fn to_spirv(self) -> spirv::StorageClass {
        match self {
            ast::LdStateSpace::Const => spirv::StorageClass::UniformConstant,
            ast::LdStateSpace::Generic => spirv::StorageClass::Generic,
            ast::LdStateSpace::Global => spirv::StorageClass::CrossWorkgroup,
            ast::LdStateSpace::Local => spirv::StorageClass::Function,
            ast::LdStateSpace::Shared => spirv::StorageClass::Workgroup,
            ast::LdStateSpace::Param => unreachable!(),
        }
    }
}

impl From<ast::FnArgumentType> for ast::VariableType {
    fn from(t: ast::FnArgumentType) -> Self {
        match t {
            ast::FnArgumentType::Reg(t) => ast::VariableType::Reg(t),
            ast::FnArgumentType::Param(t) => ast::VariableType::Param(t),
        }
    }
}

impl<T> ast::Operand<T> {
    fn underlying(&self) -> Option<&T> {
        match self {
            ast::Operand::Reg(r) | ast::Operand::RegOffset(r, _) => Some(r),
            ast::Operand::Imm(_) => None,
        }
    }
}

impl<T> ast::OperandOrVector<T> {
    fn single_underlying(&self) -> Option<&T> {
        match self {
            ast::OperandOrVector::Reg(r) | ast::OperandOrVector::RegOffset(r, _) => Some(r),
            ast::OperandOrVector::Imm(_) | ast::OperandOrVector::Vec(_) => None,
        }
    }
}

impl ast::MulDetails {
    fn is_wide(&self) -> bool {
        match self {
            ast::MulDetails::Int(desc) => desc.control == ast::MulIntControl::Wide,
            ast::MulDetails::Float(_) => false,
        }
    }
}

fn force_bitcast(
    operand: ast::Type,
    instr: ast::Type,
    _: Option<ast::LdStateSpace>,
) -> Result<Option<ConversionKind>, TranslateError> {
    if instr != operand {
        Ok(Some(ConversionKind::Default))
    } else {
        Ok(None)
    }
}

fn bitcast_physical_pointer(
    operand_type: ast::Type,
    _: ast::Type,
    ss: Option<ast::LdStateSpace>,
) -> Result<Option<ConversionKind>, TranslateError> {
    match operand_type {
        ast::Type::Scalar(ast::ScalarType::B64)
        | ast::Type::Scalar(ast::ScalarType::U64)
        | ast::Type::Scalar(ast::ScalarType::S64) => {
            if let Some(space) = ss {
                Ok(Some(ConversionKind::BitToPtr(space)))
            } else {
                Err(TranslateError::Unreachable)
            }
        }
        _ => Err(TranslateError::MismatchedType),
    }
}

fn force_bitcast_ptr_to_bit(
    _: ast::Type,
    _: ast::Type,
    _: Option<ast::LdStateSpace>,
) -> Result<Option<ConversionKind>, TranslateError> {
    Ok(Some(ConversionKind::PtrToBit))
}

fn should_bitcast(instr: ast::Type, operand: ast::Type) -> bool {
    match (instr, operand) {
        (ast::Type::Scalar(inst), ast::Type::Scalar(operand)) => {
            if inst.width() != operand.width() {
                return false;
            }
            match inst.kind() {
                ScalarKind::Bit => operand.kind() != ScalarKind::Bit,
                ScalarKind::Float => operand.kind() == ScalarKind::Bit,
                ScalarKind::Signed => {
                    operand.kind() == ScalarKind::Bit || operand.kind() == ScalarKind::Unsigned
                }
                ScalarKind::Unsigned => {
                    operand.kind() == ScalarKind::Bit || operand.kind() == ScalarKind::Signed
                }
                ScalarKind::Float2 => false,
                ScalarKind::Pred => false,
            }
        }
        (ast::Type::Vector(inst, _), ast::Type::Vector(operand, _))
        | (ast::Type::Array(inst, _), ast::Type::Array(operand, _)) => {
            should_bitcast(ast::Type::Scalar(inst), ast::Type::Scalar(operand))
        }
        _ => false,
    }
}

fn should_bitcast_packed(
    operand: ast::Type,
    instr: ast::Type,
    ss: Option<ast::LdStateSpace>,
) -> Result<Option<ConversionKind>, TranslateError> {
    if let (ast::Type::Vector(vec_underlying_type, vec_len), ast::Type::Scalar(scalar)) =
        (operand, instr)
    {
        if scalar.kind() == ScalarKind::Bit
            && scalar.width() == (vec_underlying_type.width() * vec_len)
        {
            return Ok(Some(ConversionKind::Default));
        }
    }
    should_bitcast_wrapper(operand, instr, ss)
}

fn should_bitcast_wrapper(
    operand: ast::Type,
    instr: ast::Type,
    _: Option<ast::LdStateSpace>,
) -> Result<Option<ConversionKind>, TranslateError> {
    if instr == operand {
        return Ok(None);
    }
    if should_bitcast(instr, operand) {
        Ok(Some(ConversionKind::Default))
    } else {
        Err(TranslateError::MismatchedType)
    }
}

fn should_convert_relaxed_src_wrapper(
    src_type: ast::Type,
    instr_type: ast::Type,
    _: Option<ast::LdStateSpace>,
) -> Result<Option<ConversionKind>, TranslateError> {
    if src_type == instr_type {
        return Ok(None);
    }
    match should_convert_relaxed_src(src_type, instr_type) {
        conv @ Some(_) => Ok(conv),
        None => Err(TranslateError::MismatchedType),
    }
}

// https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#operand-size-exceeding-instruction-type-size__relaxed-type-checking-rules-source-operands
fn should_convert_relaxed_src(
    src_type: ast::Type,
    instr_type: ast::Type,
) -> Option<ConversionKind> {
    if src_type == instr_type {
        return None;
    }
    match (src_type, instr_type) {
        (ast::Type::Scalar(src_type), ast::Type::Scalar(instr_type)) => match instr_type.kind() {
            ScalarKind::Bit => {
                if instr_type.width() <= src_type.width() {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
            ScalarKind::Signed | ScalarKind::Unsigned => {
                if instr_type.width() <= src_type.width() && src_type.kind() != ScalarKind::Float {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
            ScalarKind::Float => {
                if instr_type.width() <= src_type.width() && src_type.kind() == ScalarKind::Bit {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
            ScalarKind::Float2 => todo!(),
            ScalarKind::Pred => None,
        },
        (ast::Type::Vector(dst_type, _), ast::Type::Vector(instr_type, _))
        | (ast::Type::Array(dst_type, _), ast::Type::Array(instr_type, _)) => {
            should_convert_relaxed_src(ast::Type::Scalar(dst_type), ast::Type::Scalar(instr_type))
        }
        _ => None,
    }
}

fn should_convert_relaxed_dst_wrapper(
    dst_type: ast::Type,
    instr_type: ast::Type,
    _: Option<ast::LdStateSpace>,
) -> Result<Option<ConversionKind>, TranslateError> {
    if dst_type == instr_type {
        return Ok(None);
    }
    match should_convert_relaxed_dst(dst_type, instr_type) {
        conv @ Some(_) => Ok(conv),
        None => Err(TranslateError::MismatchedType),
    }
}

// https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#operand-size-exceeding-instruction-type-size__relaxed-type-checking-rules-destination-operands
fn should_convert_relaxed_dst(
    dst_type: ast::Type,
    instr_type: ast::Type,
) -> Option<ConversionKind> {
    if dst_type == instr_type {
        return None;
    }
    match (dst_type, instr_type) {
        (ast::Type::Scalar(dst_type), ast::Type::Scalar(instr_type)) => match instr_type.kind() {
            ScalarKind::Bit => {
                if instr_type.width() <= dst_type.width() {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
            ScalarKind::Signed => {
                if dst_type.kind() != ScalarKind::Float {
                    if instr_type.width() == dst_type.width() {
                        Some(ConversionKind::Default)
                    } else if instr_type.width() < dst_type.width() {
                        Some(ConversionKind::SignExtend)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            ScalarKind::Unsigned => {
                if instr_type.width() <= dst_type.width() && dst_type.kind() != ScalarKind::Float {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
            ScalarKind::Float => {
                if instr_type.width() <= dst_type.width() && dst_type.kind() == ScalarKind::Bit {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
            ScalarKind::Float2 => todo!(),
            ScalarKind::Pred => None,
        },
        (ast::Type::Vector(dst_type, _), ast::Type::Vector(instr_type, _))
        | (ast::Type::Array(dst_type, _), ast::Type::Array(instr_type, _)) => {
            should_convert_relaxed_dst(ast::Type::Scalar(dst_type), ast::Type::Scalar(instr_type))
        }
        _ => None,
    }
}

impl<'a> ast::MethodDecl<'a, &'a str> {
    fn name(&self) -> &'a str {
        match self {
            ast::MethodDecl::Kernel(name, _) => name,
            ast::MethodDecl::Func(_, name, _) => name,
        }
    }
}

impl<'a> ast::MethodDecl<'a, spirv::Word> {
    fn visit_args(&self, f: &mut impl FnMut(&ast::FnArgument<spirv::Word>)) {
        match self {
            ast::MethodDecl::Func(_, _, params) => params.iter().for_each(f),
            ast::MethodDecl::Kernel(_, params) => params.iter().for_each(|arg| {
                f(&ast::FnArgument {
                    align: arg.align,
                    name: arg.name,
                    v_type: ast::FnArgumentType::Param(arg.v_type),
                })
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast;

    static SCALAR_TYPES: [ast::ScalarType; 15] = [
        ast::ScalarType::B8,
        ast::ScalarType::B16,
        ast::ScalarType::B32,
        ast::ScalarType::B64,
        ast::ScalarType::S8,
        ast::ScalarType::S16,
        ast::ScalarType::S32,
        ast::ScalarType::S64,
        ast::ScalarType::U8,
        ast::ScalarType::U16,
        ast::ScalarType::U32,
        ast::ScalarType::U64,
        ast::ScalarType::F16,
        ast::ScalarType::F32,
        ast::ScalarType::F64,
    ];

    static RELAXED_SRC_CONVERSION_TABLE: &'static str =
        "b8 	- 	chop 	chop 	chop 	- 	chop 	chop 	chop 	- 	chop 	chop 	chop 	chop 	chop 	chop
        b16 	inv 	- 	chop 	chop 	inv 	- 	chop 	chop 	inv 	- 	chop 	chop 	- 	chop 	chop
        b32 	inv 	inv 	- 	chop 	inv 	inv 	- 	chop 	inv 	inv 	- 	chop 	inv 	- 	chop
        b64 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	-
        s8 	- 	chop 	chop 	chop 	- 	chop 	chop 	chop 	- 	chop 	chop 	chop 	inv 	inv 	inv
        s16 	inv 	- 	chop 	chop 	inv 	- 	chop 	chop 	inv 	- 	chop 	chop 	inv 	inv 	inv
        s32 	inv 	inv 	- 	chop 	inv 	inv 	- 	chop 	inv 	inv 	- 	chop 	inv 	inv 	inv
        s64 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	inv
        u8 	- 	chop 	chop 	chop 	- 	chop 	chop 	chop 	- 	chop 	chop 	chop 	inv 	inv 	inv
        u16 	inv 	- 	chop 	chop 	inv 	- 	chop 	chop 	inv 	- 	chop 	chop 	inv 	inv 	inv
        u32 	inv 	inv 	- 	chop 	inv 	inv 	- 	chop 	inv 	inv 	- 	chop 	inv 	inv 	inv
        u64 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	inv
        f16 	inv 	- 	chop 	chop 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	- 	inv 	inv
        f32 	inv 	inv 	- 	chop 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	- 	inv
        f64 	inv 	inv 	inv 	- 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	-";

    static RELAXED_DST_CONVERSION_TABLE: &'static str =
        "b8 	- 	zext 	zext 	zext 	- 	zext 	zext 	zext 	- 	zext 	zext 	zext 	zext 	zext 	zext
        b16 	inv 	- 	zext 	zext 	inv 	- 	zext 	zext 	inv 	- 	zext 	zext 	- 	zext 	zext
        b32 	inv 	inv 	- 	zext 	inv 	inv 	- 	zext 	inv 	inv 	- 	zext 	inv 	- 	zext
        b64 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	-
        s8 	- 	sext 	sext 	sext 	- 	sext 	sext 	sext 	- 	sext 	sext 	sext 	inv 	inv 	inv
        s16 	inv 	- 	sext 	sext 	inv 	- 	sext 	sext 	inv 	- 	sext 	sext 	inv 	inv 	inv
        s32 	inv 	inv 	- 	sext 	inv 	inv 	- 	sext 	inv 	inv 	- 	sext 	inv 	inv 	inv
        s64 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	inv
        u8 	- 	zext 	zext 	zext 	- 	zext 	zext 	zext 	- 	zext 	zext 	zext 	inv 	inv 	inv
        u16 	inv 	- 	zext 	zext 	inv 	- 	zext 	zext 	inv 	- 	zext 	zext 	inv 	inv 	inv
        u32 	inv 	inv 	- 	zext 	inv 	inv 	- 	zext 	inv 	inv 	- 	zext 	inv 	inv 	inv
        u64 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	inv
        f16 	inv 	- 	zext 	zext 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	- 	inv 	inv
        f32 	inv 	inv 	- 	zext 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	- 	inv
        f64 	inv 	inv 	inv 	- 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	-";

    fn table_entry_to_conversion(entry: &'static str) -> Option<ConversionKind> {
        match entry {
            "-" => Some(ConversionKind::Default),
            "inv" => None,
            "zext" => Some(ConversionKind::Default),
            "chop" => Some(ConversionKind::Default),
            "sext" => Some(ConversionKind::SignExtend),
            _ => unreachable!(),
        }
    }

    fn parse_conversion_table(table: &'static str) -> Vec<Vec<Option<ConversionKind>>> {
        table
            .lines()
            .map(|line| {
                line.split_ascii_whitespace()
                    .skip(1)
                    .map(table_entry_to_conversion)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    fn assert_conversion_table<F: Fn(ast::Type, ast::Type) -> Option<ConversionKind>>(
        table: &'static str,
        f: F,
    ) {
        let conv_table = parse_conversion_table(table);
        for (instr_idx, instr_type) in SCALAR_TYPES.iter().enumerate() {
            for (op_idx, op_type) in SCALAR_TYPES.iter().enumerate() {
                let conversion = f(ast::Type::Scalar(*op_type), ast::Type::Scalar(*instr_type));
                if instr_idx == op_idx {
                    assert_eq!(conversion, None);
                } else {
                    assert_eq!(conversion, conv_table[instr_idx][op_idx]);
                }
            }
        }
    }

    #[test]
    fn should_convert_relaxed_src_all_combinations() {
        assert_conversion_table(RELAXED_SRC_CONVERSION_TABLE, should_convert_relaxed_src);
    }

    #[test]
    fn should_convert_relaxed_dst_all_combinations() {
        assert_conversion_table(RELAXED_DST_CONVERSION_TABLE, should_convert_relaxed_dst);
    }
}
