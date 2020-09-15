use crate::ast;
use rspirv::{binary::Disassemble, dr};
use std::collections::{hash_map, HashMap, HashSet};
use std::{borrow::Cow, iter, mem};

use rspirv::binary::Assemble;

#[derive(PartialEq, Eq, Hash, Clone)]
enum SpirvType {
    Base(SpirvScalarKey),
    Vector(SpirvScalarKey, u8),
    Array(SpirvScalarKey, u32),
    Pointer(Box<SpirvType>, spirv::StorageClass),
    Func(Option<Box<SpirvType>>, Vec<SpirvType>),
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
                *self
                    .complex
                    .entry(t)
                    .or_insert_with(|| b.type_array(base, len))
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

pub fn to_spirv_module<'a>(ast: ast::Module<'a>) -> Result<dr::Module, dr::Error> {
    let mut id_defs = GlobalStringIdResolver::new(1);
    let ssa_functions = ast
        .functions
        .into_iter()
        .map(|f| to_ssa_function(&mut id_defs, f))
        .collect::<Vec<_>>();
    let mut builder = dr::Builder::new();
    builder.reserve_ids(id_defs.current_id());
    // https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_logicallayout_a_logical_layout_of_a_module
    builder.set_version(1, 3);
    emit_capabilities(&mut builder);
    emit_extensions(&mut builder);
    let opencl_id = emit_opencl_import(&mut builder);
    emit_memory_model(&mut builder);
    let mut map = TypeWordMap::new(&mut builder);
    for f in ssa_functions {
        let f_body = match f.body {
            Some(f) => f,
            None => continue,
        };
        emit_function_header(&mut builder, &mut map, &id_defs, f.func_directive)?;
        emit_function_body_ops(&mut builder, &mut map, opencl_id, &f_body)?;
        builder.end_function()?;
    }
    Ok(builder.module())
}

fn emit_function_header<'a>(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    global: &GlobalStringIdResolver<'a>,
    func_directive: ast::MethodDecl<ExpandedArgParams>,
) -> Result<(), dr::Error> {
    let (ret_type, func_type) = get_function_type(builder, map, &func_directive);
    let fn_id = match func_directive {
        ast::MethodDecl::Kernel(name, _) => {
            let fn_id = global.get_id(name);
            builder.entry_point(spirv::ExecutionModel::Kernel, fn_id, name, &[]);
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

pub fn to_spirv<'a>(ast: ast::Module<'a>) -> Result<Vec<u32>, dr::Error> {
    let module = to_spirv_module(ast)?;
    Ok(module.assemble())
}

fn emit_capabilities(builder: &mut dr::Builder) {
    builder.capability(spirv::Capability::GenericPointer);
    builder.capability(spirv::Capability::Linkage);
    builder.capability(spirv::Capability::Addresses);
    builder.capability(spirv::Capability::Kernel);
    builder.capability(spirv::Capability::Int64);
    builder.capability(spirv::Capability::Int8);
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
) -> ExpandedFunction<'a> {
    let (str_resolver, fn_resolver, fn_decl) = id_defs.start_fn(&f.func_directive);
    to_ssa(str_resolver, fn_resolver, fn_decl, f.body)
}

fn expand_kernel_params<'a, 'b>(
    fn_resolver: &mut FnStringIdResolver<'a, 'b>,
    args: impl Iterator<Item = &'b ast::KernelArgument<ast::ParsedArgParams<'a>>>,
) -> Vec<ast::KernelArgument<ExpandedArgParams>> {
    args.map(|a| ast::KernelArgument {
        name: fn_resolver.add_def(a.name, Some(ast::Type::from(a.v_type))),
        v_type: a.v_type,
        align: a.align,
    })
    .collect()
}

fn expand_fn_params<'a, 'b>(
    fn_resolver: &mut FnStringIdResolver<'a, 'b>,
    args: impl Iterator<Item = &'b ast::FnArgument<ast::ParsedArgParams<'a>>>,
) -> Vec<ast::FnArgument<ExpandedArgParams>> {
    args.map(|a| ast::FnArgument {
        name: fn_resolver.add_def(a.name, Some(ast::Type::from(a.v_type))),
        v_type: a.v_type,
        align: a.align,
    })
    .collect()
}

fn to_ssa<'input, 'b>(
    mut id_defs: FnStringIdResolver<'input, 'b>,
    fn_defs: GlobalFnDeclResolver<'input, 'b>,
    f_args: ast::MethodDecl<'input, ExpandedArgParams>,
    f_body: Option<Vec<ast::Statement<ast::ParsedArgParams<'input>>>>,
) -> ExpandedFunction<'input> {
    let f_body = match f_body {
        Some(vec) => vec,
        None => {
            return ExpandedFunction {
                func_directive: f_args,
                body: None,
            }
        }
    };
    let normalized_ids = normalize_identifiers(&mut id_defs, &fn_defs, f_body);
    let mut numeric_id_defs = id_defs.finish();
    let unadorned_statements = normalize_predicates(normalized_ids, &mut numeric_id_defs);
    let unadorned_statements =
        add_types_to_statements(unadorned_statements, &fn_defs, &numeric_id_defs);
    let (f_args, ssa_statements) =
        insert_mem_ssa_statements(unadorned_statements, &mut numeric_id_defs, f_args);
    let expanded_statements = expand_arguments(ssa_statements, &mut numeric_id_defs);
    let expanded_statements =
        insert_implicit_conversions(expanded_statements, &mut numeric_id_defs);
    let labeled_statements = normalize_labels(expanded_statements, &mut numeric_id_defs);
    let sorted_statements = normalize_variable_decls(labeled_statements);
    ExpandedFunction {
        func_directive: f_args,
        body: Some(sorted_statements),
    }
}

fn normalize_variable_decls(mut func: Vec<ExpandedStatement>) -> Vec<ExpandedStatement> {
    func[1..].sort_by_key(|s| match s {
        Statement::Variable(_) => 0,
        _ => 1,
    });
    func
}

fn add_types_to_statements(
    func: Vec<UnadornedStatement>,
    fn_defs: &GlobalFnDeclResolver,
    id_defs: &NumericIdResolver,
) -> Vec<UnadornedStatement> {
    func.into_iter()
        .map(|s| {
            match s {
                Statement::Instruction(ast::Instruction::Call(call)) => {
                    // TODO: error out if lengths don't match
                    let fn_def = fn_defs.get_fn_decl(call.func);
                    let ret_params = to_resolved_fn_args(call.ret_params, &*fn_def.ret_vals);
                    let param_list = to_resolved_fn_args(call.param_list, &*fn_def.params);
                    let resolved_call = ResolvedCall {
                        uniform: call.uniform,
                        ret_params,
                        func: call.func,
                        param_list,
                    };
                    Statement::Call(resolved_call)
                }
                Statement::Instruction(ast::Instruction::MovVector(dets, args)) => {
                    // TODO fail on type mismatch
                    let new_dets = match id_defs.get_type(*args.dst()) {
                        Some(ast::Type::Vector(_, len)) => ast::MovVectorDetails {
                            length: len,
                            ..dets
                        },
                        _ => dets,
                    };
                    Statement::Instruction(ast::Instruction::MovVector(new_dets, args))
                }
                s => s,
            }
        })
        .collect()
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
            | Statement::Label(_) => (),
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
) -> Vec<UnadornedStatement> {
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
    func: Vec<UnadornedStatement>,
    id_def: &mut NumericIdResolver,
    mut f_args: ast::MethodDecl<'a, ExpandedArgParams>,
) -> (
    ast::MethodDecl<'a, ExpandedArgParams>,
    Vec<UnadornedStatement>,
) {
    let mut result = Vec::with_capacity(func.len());
    let out_param = match &mut f_args {
        ast::MethodDecl::Kernel(_, in_params) => {
            for p in in_params.iter_mut() {
                let typ = ast::Type::from(p.v_type);
                let new_id = id_def.new_id(Some(typ));
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
                let new_id = id_def.new_id(Some(typ));
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
            Statement::Call(call) => insert_mem_ssa_statement_default(id_def, &mut result, call),
            Statement::Instruction(inst) => match inst {
                ast::Instruction::Ret(d) => {
                    if let Some(out_param) = out_param {
                        let typ = id_def.get_type(out_param);
                        let new_id = id_def.new_id(typ);
                        result.push(Statement::LoadVar(
                            ast::Arg2 {
                                dst: new_id,
                                src: out_param,
                            },
                            typ.unwrap(),
                        ));
                        result.push(Statement::RetValue(d, new_id));
                    } else {
                        result.push(Statement::Instruction(ast::Instruction::Ret(d)))
                    }
                }
                inst => insert_mem_ssa_statement_default(id_def, &mut result, inst),
            },
            Statement::Conditional(mut bra) => {
                let generated_id = id_def.new_id(Some(ast::Type::Scalar(ast::ScalarType::Pred)));
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
            | Statement::Constant(_) => unreachable!(),
            Statement::Composite(_) => todo!(),
        }
    }
    (f_args, result)
}

trait VisitVariable: Sized {
    fn visit_variable<
        'a,
        F: FnMut(ArgumentDescriptor<spirv::Word>, Option<ast::Type>) -> spirv::Word,
    >(
        self,
        f: &mut F,
    ) -> UnadornedStatement;
}
trait VisitVariableExpanded {
    fn visit_variable_extended<
        F: FnMut(ArgumentDescriptor<spirv::Word>, Option<ast::Type>) -> spirv::Word,
    >(
        self,
        f: &mut F,
    ) -> ExpandedStatement;
}

fn insert_mem_ssa_statement_default<'a, F: VisitVariable>(
    id_def: &mut NumericIdResolver,
    result: &mut Vec<UnadornedStatement>,
    stmt: F,
) {
    let mut post_statements = Vec::new();
    let new_statement = stmt.visit_variable(&mut |desc: ArgumentDescriptor<spirv::Word>, _| {
        let id_type = match (id_def.get_type(desc.op), desc.is_pointer) {
            (Some(t), false) => t,
            (Some(_), true) => ast::Type::Scalar(ast::ScalarType::B64),
            (None, _) => return desc.op,
        };
        let generated_id = id_def.new_id(Some(id_type));
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
        generated_id
    });
    result.push(new_statement);
    result.append(&mut post_statements);
}

fn expand_arguments<'a, 'b>(
    func: Vec<UnadornedStatement>,
    id_def: &'b mut NumericIdResolver<'a>,
) -> Vec<ExpandedStatement> {
    let mut result = Vec::with_capacity(func.len());
    for s in func {
        match s {
            Statement::Call(call) => {
                let mut visitor = FlattenArguments::new(&mut result, id_def);
                let (new_call, post_stmts) = (call.map(&mut visitor), visitor.post_stmts);
                result.push(Statement::Call(new_call));
                result.extend(post_stmts);
            }
            Statement::Instruction(inst) => {
                let mut visitor = FlattenArguments::new(&mut result, id_def);
                let (new_inst, post_stmts) = (inst.map(&mut visitor), visitor.post_stmts);
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
            Statement::Composite(_) | Statement::Conversion(_) | Statement::Constant(_) => {
                unreachable!()
            }
        }
    }
    result
}

struct FlattenArguments<'a, 'b> {
    func: &'b mut Vec<ExpandedStatement>,
    id_def: &'b mut NumericIdResolver<'a>,
    post_stmts: Vec<ExpandedStatement>,
}

impl<'a, 'b> FlattenArguments<'a, 'b> {
    fn new(func: &'b mut Vec<ExpandedStatement>, id_def: &'b mut NumericIdResolver<'a>) -> Self {
        FlattenArguments {
            func,
            id_def,
            post_stmts: Vec::new(),
        }
    }
}

impl<'a, 'b> ArgumentMapVisitor<NormalizedArgParams, ExpandedArgParams>
    for FlattenArguments<'a, 'b>
{
    fn variable(
        &mut self,
        desc: ArgumentDescriptor<spirv::Word>,
        _: Option<ast::Type>,
    ) -> spirv::Word {
        desc.op
    }

    fn operand(
        &mut self,
        desc: ArgumentDescriptor<ast::Operand<spirv::Word>>,
        typ: ast::Type,
    ) -> spirv::Word {
        match desc.op {
            ast::Operand::Reg(r) => self.variable(desc.new_op(r), Some(typ)),
            ast::Operand::Imm(x) => {
                let scalar_t = if let ast::Type::Scalar(scalar) = typ {
                    scalar
                } else {
                    todo!()
                };
                let id = self.id_def.new_id(Some(ast::Type::Scalar(scalar_t)));
                self.func.push(Statement::Constant(ConstantDefinition {
                    dst: id,
                    typ: scalar_t,
                    value: x,
                }));
                id
            }
            ast::Operand::RegOffset(reg, offset) => {
                let scalar_t = if let ast::Type::Scalar(scalar) = typ {
                    scalar
                } else {
                    todo!()
                };
                let id_constant_stmt = self.id_def.new_id(Some(ast::Type::Scalar(scalar_t)));
                self.func.push(Statement::Constant(ConstantDefinition {
                    dst: id_constant_stmt,
                    typ: scalar_t,
                    value: offset as i128,
                }));
                let result_id = self.id_def.new_id(Some(typ));
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
                result_id
            }
        }
    }

    fn src_call_operand(
        &mut self,
        desc: ArgumentDescriptor<ast::CallOperand<spirv::Word>>,
        typ: ast::Type,
    ) -> spirv::Word {
        match desc.op {
            ast::CallOperand::Reg(reg) => self.variable(desc.new_op(reg), Some(typ)),
            ast::CallOperand::Imm(x) => self.operand(desc.new_op(ast::Operand::Imm(x)), typ),
        }
    }

    fn src_vec_operand(
        &mut self,
        desc: ArgumentDescriptor<(spirv::Word, u8)>,
        (scalar_type, vec_len): (ast::MovVectorType, u8),
    ) -> spirv::Word {
        let new_id = self
            .id_def
            .new_id(Some(ast::Type::Vector(scalar_type.into(), vec_len)));
        self.func.push(Statement::Composite(CompositeRead {
            typ: scalar_type,
            dst: new_id,
            src_composite: desc.op.0,
            src_index: desc.op.1 as u32,
        }));
        new_id
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
    id_def: &mut NumericIdResolver,
) -> Vec<ExpandedStatement> {
    let mut result = Vec::with_capacity(func.len());
    for s in func.into_iter() {
        match s {
            Statement::Call(call) => insert_implicit_bitcasts(&mut result, id_def, call),
            Statement::Instruction(inst) => match inst {
                ast::Instruction::Ld(ld, arg) => {
                    let pre_conv =
                        get_implicit_conversions_ld_src(id_def, ld.typ, ld.state_space, arg.src);
                    let post_conv = get_implicit_conversions_ld_dst(
                        id_def,
                        ld.typ,
                        arg.dst,
                        should_convert_relaxed_dst,
                        false,
                    );
                    insert_with_conversions(
                        &mut result,
                        id_def,
                        arg,
                        pre_conv.into_iter(),
                        iter::empty(),
                        post_conv.into_iter().collect(),
                        |arg| &mut arg.src,
                        |arg| &mut arg.dst,
                        |arg| ast::Instruction::Ld(ld, arg),
                    )
                }
                ast::Instruction::St(st, arg) => {
                    let pre_conv = get_implicit_conversions_ld_dst(
                        id_def,
                        st.typ,
                        arg.src2,
                        should_convert_relaxed_src,
                        true,
                    );
                    let post_conv = get_implicit_conversions_ld_src(
                        id_def,
                        st.typ,
                        st.state_space.to_ld_ss(),
                        arg.src1,
                    );
                    let (pre_conv_dest, post_conv) = if st.state_space == ast::StStateSpace::Param {
                        (Vec::new(), post_conv)
                    } else {
                        (post_conv, Vec::new())
                    };
                    insert_with_conversions(
                        &mut result,
                        id_def,
                        arg,
                        pre_conv.into_iter(),
                        pre_conv_dest.into_iter(),
                        post_conv,
                        |arg| &mut arg.src2,
                        |arg| &mut arg.src1,
                        |arg| ast::Instruction::St(st, arg),
                    )
                }
                ast::Instruction::Mov(d, mut arg) => {
                    // https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#data-movement-and-conversion-instructions-mov-2
                    // TODO: handle the case of mixed vector/scalar implicit conversions
                    let inst_typ_is_bit = match d {
                        ast::MovType::Scalar(t) => {
                            ast::ScalarType::from(t).kind() == ScalarKind::Bit
                        }
                        ast::MovType::Vector(_, _) => false,
                    };
                    let mut did_vector_implicit = false;
                    let mut post_conv = None;
                    if inst_typ_is_bit {
                        let src_type = id_def.get_type(arg.src).unwrap_or_else(|| todo!());
                        if let ast::Type::Vector(_, _) = src_type {
                            arg.src = insert_conversion_src(
                                &mut result,
                                id_def,
                                arg.src,
                                src_type,
                                d.into(),
                                ConversionKind::Default,
                            );
                            did_vector_implicit = true;
                        }
                        let dst_type = id_def.get_type(arg.dst).unwrap_or_else(|| todo!());
                        if let ast::Type::Vector(_, _) = src_type {
                            post_conv = Some(get_conversion_dst(
                                id_def,
                                &mut arg.dst,
                                d.into(),
                                dst_type,
                                ConversionKind::Default,
                            ));
                            did_vector_implicit = true;
                        }
                    }
                    if did_vector_implicit {
                        result.push(Statement::Instruction(ast::Instruction::Mov(d, arg)));
                    } else {
                        insert_implicit_bitcasts(
                            &mut result,
                            id_def,
                            ast::Instruction::Mov(d, arg),
                        );
                    }
                    if let Some(post_conv) = post_conv {
                        result.push(post_conv);
                    }
                }
                inst @ _ => insert_implicit_bitcasts(&mut result, id_def, inst),
            },
            s @ Statement::Composite(_)
            | s @ Statement::Conditional(_)
            | s @ Statement::Label(_)
            | s @ Statement::Constant(_)
            | s @ Statement::Variable(_)
            | s @ Statement::LoadVar(_, _)
            | s @ Statement::StoreVar(_, _)
            | s @ Statement::RetValue(_, _) => result.push(s),
            Statement::Conversion(_) => unreachable!(),
        }
    }
    result
}

fn get_function_type(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    method_decl: &ast::MethodDecl<ExpandedArgParams>,
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
) -> Result<(), dr::Error> {
    for s in func {
        match s {
            Statement::Label(id) => {
                if builder.block.is_some() {
                    builder.branch(*id)?;
                }
                builder.begin_block(Some(*id))?;
            }
            _ => {
                if builder.block.is_none() {
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
                        *id,
                    ),
                    _ => todo!(),
                };
                let arg_list = call
                    .param_list
                    .iter()
                    .map(|(id, _)| *id)
                    .collect::<Vec<_>>();
                builder.function_call(result_type, Some(result_id), call.func, arg_list)?;
            }
            Statement::Variable(ast::Variable {
                align,
                v_type,
                name,
            }) => {
                let type_id = map.get_or_add(
                    builder,
                    SpirvType::new_pointer(ast::Type::from(*v_type), spirv::StorageClass::Function),
                );
                let st_class = match v_type {
                    ast::VariableType::Reg(_) | ast::VariableType::Param(_) => {
                        spirv::StorageClass::Function
                    }
                    ast::VariableType::Local(_) => spirv::StorageClass::Workgroup,
                };
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
                ast::Instruction::Abs(_, _) => todo!(),
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
                        ast::LdStateSpace::Param => {
                            let result_type = map.get_or_add(builder, SpirvType::from(data.typ));
                            builder.copy_object(result_type, Some(arg.dst), arg.src)?;
                        }
                        _ => todo!(),
                    }
                }
                ast::Instruction::St(data, arg) => {
                    if data.qualifier != ast::LdStQualifier::Weak
                        || (data.state_space != ast::StStateSpace::Generic
                            && data.state_space != ast::StStateSpace::Param
                            && data.state_space != ast::StStateSpace::Global)
                    {
                        todo!()
                    }
                    if data.state_space == ast::StStateSpace::Param {
                        let result_type = map.get_or_add(builder, SpirvType::from(data.typ));
                        builder.copy_object(result_type, Some(arg.src1), arg.src2)?;
                    } else {
                        builder.store(arg.src1, arg.src2, None, &[])?;
                    }
                }
                // SPIR-V does not support ret as guaranteed-converged
                ast::Instruction::Ret(_) => builder.ret()?,
                ast::Instruction::Mov(mov_type, arg) => {
                    let result_type =
                        map.get_or_add(builder, SpirvType::from(ast::Type::from(*mov_type)));
                    builder.copy_object(result_type, Some(arg.dst), arg.src)?;
                }
                ast::Instruction::Mul(mul, arg) => match mul {
                    ast::MulDetails::Int(ref ctr) => {
                        emit_mul_int(builder, map, opencl, ctr, arg)?;
                    }
                    ast::MulDetails::Float(_) => todo!(),
                },
                ast::Instruction::Add(add, arg) => match add {
                    ast::AddDetails::Int(ref desc) => {
                        emit_add_int(builder, map, desc, arg)?;
                    }
                    ast::AddDetails::Float(_) => todo!(),
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
                        ast::NotType::Pred => builder.logical_not(result_type, result_id, operand),
                        _ => builder.not(result_type, result_id, operand),
                    }?;
                }
                ast::Instruction::Shl(t, a) => {
                    let result_type = map.get_or_add(builder, SpirvType::from(t.to_type()));
                    builder.shift_left_logical(result_type, Some(a.dst), a.src1, a.src2)?;
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
                ast::Instruction::MovVector(typ, arg) => match arg {
                    ast::Arg2Vec::Dst((dst, dst_index), composite_src, src)
                    | ast::Arg2Vec::Both((dst, dst_index), composite_src, src) => {
                        let result_type = map.get_or_add(
                            builder,
                            SpirvType::Vector(
                                SpirvScalarKey::from(ast::ScalarType::from(typ.typ)),
                                typ.length,
                            ),
                        );
                        let result_id = Some(*dst);
                        builder.composite_insert(
                            result_type,
                            result_id,
                            *src,
                            *composite_src,
                            [*dst_index as u32],
                        )?;
                    }
                    ast::Arg2Vec::Src(dst, src) => {
                        let result_type =
                            map.get_or_add_scalar(builder, ast::ScalarType::from(typ.typ));
                        builder.copy_object(result_type, Some(*dst), *src)?;
                    }
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
        }
    }
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
    arg: &ast::Arg4<ExpandedArgParams>,
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
        ast::MulIntControl::Wide => todo!(),
    }
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
        (_, _, ConversionKind::Ptr(space)) => {
            let dst_type = map.get_or_add(
                builder,
                SpirvType::Pointer(Box::new(SpirvType::from(cv.to)), space.to_spirv()),
            );
            builder.convert_u_to_ptr(dst_type, Some(cv.dst), cv.src)?;
        }
        (TypeKind::Scalar, TypeKind::Scalar, ConversionKind::Default) => {
            if from_parts.width == to_parts.width {
                let dst_type = map.get_or_add(builder, SpirvType::from(cv.from));
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
        | (TypeKind::Scalar, TypeKind::Vector, ConversionKind::Default) => {
            let into_type = map.get_or_add(builder, SpirvType::from(cv.to));
            builder.bitcast(into_type, Some(cv.dst), cv.src)?;
        }
        _ => unreachable!(),
    }
    Ok(())
}

// TODO: support scopes
fn normalize_identifiers<'a, 'b>(
    id_defs: &mut FnStringIdResolver<'a, 'b>,
    fn_defs: &GlobalFnDeclResolver<'a, 'b>,
    func: Vec<ast::Statement<ast::ParsedArgParams<'a>>>,
) -> Vec<NormalizedStatement> {
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
        expand_map_variables(id_defs, fn_defs, &mut result, s);
    }
    result
}

fn expand_map_variables<'a, 'b>(
    id_defs: &mut FnStringIdResolver<'a, 'b>,
    fn_defs: &GlobalFnDeclResolver<'a, 'b>,
    result: &mut Vec<NormalizedStatement>,
    s: ast::Statement<ast::ParsedArgParams<'a>>,
) {
    match s {
        ast::Statement::Block(block) => {
            id_defs.start_block();
            for s in block {
                expand_map_variables(id_defs, fn_defs, result, s);
            }
            id_defs.end_block();
        }
        ast::Statement::Label(name) => result.push(Statement::Label(id_defs.get_id(name))),
        ast::Statement::Instruction(p, i) => result.push(Statement::Instruction((
            p.map(|p| p.map_variable(&mut |id| id_defs.get_id(id))),
            i.map_variable(&mut |id| id_defs.get_id(id)),
        ))),
        ast::Statement::Variable(var) => match var.count {
            Some(count) => {
                for new_id in id_defs.add_defs(var.var.name, count, var.var.v_type.into()) {
                    result.push(Statement::Variable(ast::Variable {
                        align: var.var.align,
                        v_type: var.var.v_type,
                        name: new_id,
                    }))
                }
            }
            None => {
                let new_id = id_defs.add_def(var.var.name, Some(var.var.v_type.into()));
                result.push(Statement::Variable(ast::Variable {
                    align: var.var.align,
                    v_type: var.var.v_type,
                    name: new_id,
                }));
            }
        },
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash)]
enum PtxSpecialRegister {
    Tid,
    Ntid,
    Ctaid,
    Nctaid,
    Gridid,
}

impl PtxSpecialRegister {
    fn try_parse(s: &str) -> Option<Self> {
        match s {
            "%tid" => Some(Self::Tid),
            "%ntid" => Some(Self::Ntid),
            "%ctaid" => Some(Self::Ctaid),
            "%nctaid" => Some(Self::Nctaid),
            "%gridid" => Some(Self::Gridid),
            _ => None,
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

    fn get_id(&self, id: &str) -> spirv::Word {
        self.variables[id]
    }

    fn current_id(&self) -> spirv::Word {
        self.current_id
    }

    fn start_fn<'b>(
        &'b mut self,
        header: &'b ast::MethodDecl<'a, ast::ParsedArgParams<'a>>,
    ) -> (
        FnStringIdResolver<'a, 'b>,
        GlobalFnDeclResolver<'a, 'b>,
        ast::MethodDecl<'a, ExpandedArgParams>,
    ) {
        // In case a function decl was inserted eearlier we want to use its id
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
    fn get_fn_decl(&self, id: spirv::Word) -> &FnDecl {
        &self.fns[&id]
    }

    fn get_fn_decl_str(&self, id: &str) -> &'a FnDecl {
        &self.fns[&self.variables[id]]
    }
}

struct FnStringIdResolver<'input, 'b> {
    current_id: &'b mut spirv::Word,
    global_variables: &'b HashMap<Cow<'input, str>, spirv::Word>,
    special_registers: &'b mut HashMap<PtxSpecialRegister, spirv::Word>,
    variables: Vec<HashMap<Cow<'input, str>, spirv::Word>>,
    type_check: HashMap<u32, ast::Type>,
}

impl<'a, 'b> FnStringIdResolver<'a, 'b> {
    fn finish(self) -> NumericIdResolver<'b> {
        NumericIdResolver {
            current_id: self.current_id,
            type_check: self.type_check,
        }
    }

    fn start_block(&mut self) {
        self.variables.push(HashMap::new())
    }

    fn end_block(&mut self) {
        self.variables.pop();
    }

    fn get_id(&mut self, id: &str) -> spirv::Word {
        for scope in self.variables.iter().rev() {
            match scope.get(id) {
                Some(id) => return *id,
                None => continue,
            }
        }
        match self.global_variables.get(id) {
            Some(id) => *id,
            None => {
                let sreg = PtxSpecialRegister::try_parse(id).unwrap_or_else(|| todo!());
                match self.special_registers.entry(sreg) {
                    hash_map::Entry::Occupied(e) => *e.get(),
                    hash_map::Entry::Vacant(e) => {
                        let numeric_id = *self.current_id;
                        *self.current_id += 1;
                        e.insert(numeric_id);
                        numeric_id
                    }
                }
            }
        }
    }

    fn add_def(&mut self, id: &'a str, typ: Option<ast::Type>) -> spirv::Word {
        let numeric_id = *self.current_id;
        self.variables
            .last_mut()
            .unwrap()
            .insert(Cow::Borrowed(id), numeric_id);
        if let Some(typ) = typ {
            self.type_check.insert(numeric_id, typ);
        }
        *self.current_id += 1;
        numeric_id
    }

    #[must_use]
    fn add_defs(
        &mut self,
        base_id: &'a str,
        count: u32,
        typ: ast::Type,
    ) -> impl Iterator<Item = spirv::Word> {
        let numeric_id = *self.current_id;
        for i in 0..count {
            self.variables
                .last_mut()
                .unwrap()
                .insert(Cow::Owned(format!("{}{}", base_id, i)), numeric_id + i);
            self.type_check.insert(numeric_id + i, typ);
        }
        *self.current_id += count;
        (0..count).into_iter().map(move |i| i + numeric_id)
    }
}

struct NumericIdResolver<'b> {
    current_id: &'b mut spirv::Word,
    type_check: HashMap<u32, ast::Type>,
}

impl<'b> NumericIdResolver<'b> {
    fn get_type(&self, id: spirv::Word) -> Option<ast::Type> {
        self.type_check.get(&id).map(|x| *x)
    }

    fn new_id(&mut self, typ: Option<ast::Type>) -> spirv::Word {
        let new_id = *self.current_id;
        if let Some(typ) = typ {
            self.type_check.insert(new_id, typ);
        }
        *self.current_id += 1;
        new_id
    }
}

enum Statement<I, P: ast::ArgParams> {
    Label(u32),
    Variable(ast::Variable<ast::VariableType, P>),
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
}

struct ResolvedCall<P: ast::ArgParams> {
    pub uniform: bool,
    pub ret_params: Vec<(spirv::Word, ast::FnArgumentType)>,
    pub func: spirv::Word,
    pub param_list: Vec<(P::CallOperand, ast::FnArgumentType)>,
}

impl<From: ArgParamsEx<ID = spirv::Word>> ResolvedCall<From> {
    fn map<To: ArgParamsEx<ID = spirv::Word>, V: ArgumentMapVisitor<From, To>>(
        self,
        visitor: &mut V,
    ) -> ResolvedCall<To> {
        let ret_params = self
            .ret_params
            .into_iter()
            .map(|(id, typ)| {
                let new_id = visitor.variable(
                    ArgumentDescriptor {
                        op: id,
                        is_dst: true,
                        is_pointer: false,
                    },
                    Some(typ.into()),
                );
                (new_id, typ)
            })
            .collect();
        let func = visitor.variable(
            ArgumentDescriptor {
                op: self.func,
                is_dst: false,
                is_pointer: false,
            },
            None,
        );
        let param_list = self
            .param_list
            .into_iter()
            .map(|(id, typ)| {
                let new_id = visitor.src_call_operand(
                    ArgumentDescriptor {
                        op: id,
                        is_dst: false,
                        is_pointer: false,
                    },
                    typ.into(),
                );
                (new_id, typ)
            })
            .collect();
        ResolvedCall {
            uniform: self.uniform,
            ret_params,
            func,
            param_list,
        }
    }
}

impl VisitVariable for ResolvedCall<NormalizedArgParams> {
    fn visit_variable<
        'a,
        F: FnMut(ArgumentDescriptor<spirv::Word>, Option<ast::Type>) -> spirv::Word,
    >(
        self,
        f: &mut F,
    ) -> UnadornedStatement {
        Statement::Call(self.map(f))
    }
}

impl VisitVariableExpanded for ResolvedCall<ExpandedArgParams> {
    fn visit_variable_extended<
        F: FnMut(ArgumentDescriptor<spirv::Word>, Option<ast::Type>) -> spirv::Word,
    >(
        self,
        f: &mut F,
    ) -> ExpandedStatement {
        Statement::Call(self.map(f))
    }
}

pub trait ArgParamsEx: ast::ArgParams {
    fn get_fn_decl<'x, 'b>(id: &Self::ID, decl: &'b GlobalFnDeclResolver<'x, 'b>) -> &'b FnDecl;
}

impl<'input> ArgParamsEx for ast::ParsedArgParams<'input> {
    fn get_fn_decl<'x, 'b>(id: &Self::ID, decl: &'b GlobalFnDeclResolver<'x, 'b>) -> &'b FnDecl {
        decl.get_fn_decl_str(id)
    }
}

enum NormalizedArgParams {}
type NormalizedStatement = Statement<
    (
        Option<ast::PredAt<spirv::Word>>,
        ast::Instruction<NormalizedArgParams>,
    ),
    NormalizedArgParams,
>;
type UnadornedStatement = Statement<ast::Instruction<NormalizedArgParams>, NormalizedArgParams>;

impl ast::ArgParams for NormalizedArgParams {
    type ID = spirv::Word;
    type Operand = ast::Operand<spirv::Word>;
    type CallOperand = ast::CallOperand<spirv::Word>;
    type VecOperand = (spirv::Word, u8);
}

impl ArgParamsEx for NormalizedArgParams {
    fn get_fn_decl<'a, 'b>(id: &Self::ID, decl: &'b GlobalFnDeclResolver<'a, 'b>) -> &'b FnDecl {
        decl.get_fn_decl(*id)
    }
}

enum ExpandedArgParams {}
type ExpandedStatement = Statement<ast::Instruction<ExpandedArgParams>, ExpandedArgParams>;
type ExpandedFunction<'a> = ast::Function<'a, ExpandedArgParams, ExpandedStatement>;

impl ast::ArgParams for ExpandedArgParams {
    type ID = spirv::Word;
    type Operand = spirv::Word;
    type CallOperand = spirv::Word;
    type VecOperand = spirv::Word;
}

impl ArgParamsEx for ExpandedArgParams {
    fn get_fn_decl<'a, 'b>(id: &Self::ID, decl: &'b GlobalFnDeclResolver<'a, 'b>) -> &'b FnDecl {
        decl.get_fn_decl(*id)
    }
}

trait ArgumentMapVisitor<T: ArgParamsEx, U: ArgParamsEx> {
    fn variable(&mut self, desc: ArgumentDescriptor<T::ID>, typ: Option<ast::Type>) -> U::ID;
    fn operand(&mut self, desc: ArgumentDescriptor<T::Operand>, typ: ast::Type) -> U::Operand;
    fn src_call_operand(
        &mut self,
        desc: ArgumentDescriptor<T::CallOperand>,
        typ: ast::Type,
    ) -> U::CallOperand;
    fn src_vec_operand(
        &mut self,
        desc: ArgumentDescriptor<T::VecOperand>,
        typ: (ast::MovVectorType, u8),
    ) -> U::VecOperand;
}

impl<T> ArgumentMapVisitor<ExpandedArgParams, ExpandedArgParams> for T
where
    T: FnMut(ArgumentDescriptor<spirv::Word>, Option<ast::Type>) -> spirv::Word,
{
    fn variable(
        &mut self,
        desc: ArgumentDescriptor<spirv::Word>,
        t: Option<ast::Type>,
    ) -> spirv::Word {
        self(desc, t)
    }
    fn operand(&mut self, desc: ArgumentDescriptor<spirv::Word>, t: ast::Type) -> spirv::Word {
        self(desc, Some(t))
    }
    fn src_call_operand(
        &mut self,
        desc: ArgumentDescriptor<spirv::Word>,
        t: ast::Type,
    ) -> spirv::Word {
        self(desc, Some(t))
    }
    fn src_vec_operand(
        &mut self,
        desc: ArgumentDescriptor<spirv::Word>,
        (scalar_type, vec_len): (ast::MovVectorType, u8),
    ) -> spirv::Word {
        self(
            desc.new_op(desc.op),
            Some(ast::Type::Vector(scalar_type.into(), vec_len)),
        )
    }
}

impl<'a, T> ArgumentMapVisitor<ast::ParsedArgParams<'a>, NormalizedArgParams> for T
where
    T: FnMut(&str) -> spirv::Word,
{
    fn variable(&mut self, desc: ArgumentDescriptor<&str>, _: Option<ast::Type>) -> spirv::Word {
        self(desc.op)
    }

    fn operand(
        &mut self,
        desc: ArgumentDescriptor<ast::Operand<&str>>,
        _: ast::Type,
    ) -> ast::Operand<spirv::Word> {
        match desc.op {
            ast::Operand::Reg(id) => ast::Operand::Reg(self(id)),
            ast::Operand::Imm(imm) => ast::Operand::Imm(imm),
            ast::Operand::RegOffset(id, imm) => ast::Operand::RegOffset(self(id), imm),
        }
    }

    fn src_call_operand(
        &mut self,
        desc: ArgumentDescriptor<ast::CallOperand<&str>>,
        _: ast::Type,
    ) -> ast::CallOperand<spirv::Word> {
        match desc.op {
            ast::CallOperand::Reg(id) => ast::CallOperand::Reg(self(id)),
            ast::CallOperand::Imm(imm) => ast::CallOperand::Imm(imm),
        }
    }

    fn src_vec_operand(
        &mut self,
        desc: ArgumentDescriptor<(&str, u8)>,
        _: (ast::MovVectorType, u8),
    ) -> (spirv::Word, u8) {
        (self(desc.op.0), desc.op.1)
    }
}

struct ArgumentDescriptor<Op> {
    op: Op,
    is_dst: bool,
    is_pointer: bool,
}

impl<T> ArgumentDescriptor<T> {
    fn new_op<U>(&self, u: U) -> ArgumentDescriptor<U> {
        ArgumentDescriptor {
            op: u,
            is_dst: self.is_dst,
            is_pointer: self.is_pointer,
        }
    }
}

impl<T: ArgParamsEx> ast::Instruction<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
    ) -> ast::Instruction<U> {
        match self {
            ast::Instruction::MovVector(t, a) => {
                ast::Instruction::MovVector(t, a.map(visitor, (t.typ, t.length)))
            }
            ast::Instruction::Abs(_, _) => todo!(),
            // Call instruction is converted to a call statement early on
            ast::Instruction::Call(_) => unreachable!(),
            ast::Instruction::Ld(d, a) => {
                let inst_type = d.typ;
                let src_is_pointer = d.state_space != ast::LdStateSpace::Param;
                ast::Instruction::Ld(d, a.map_ld(visitor, inst_type, src_is_pointer))
            }
            ast::Instruction::Mov(mov_type, a) => {
                ast::Instruction::Mov(mov_type, a.map(visitor, mov_type.into()))
            }
            ast::Instruction::Mul(d, a) => {
                let inst_type = d.get_type();
                ast::Instruction::Mul(d, a.map_non_shift(visitor, inst_type))
            }
            ast::Instruction::Add(d, a) => {
                let inst_type = d.get_type();
                ast::Instruction::Add(d, a.map_non_shift(visitor, inst_type))
            }
            ast::Instruction::Setp(d, a) => {
                let inst_type = d.typ;
                ast::Instruction::Setp(d, a.map(visitor, ast::Type::Scalar(inst_type)))
            }
            ast::Instruction::SetpBool(d, a) => {
                let inst_type = d.typ;
                ast::Instruction::SetpBool(d, a.map(visitor, ast::Type::Scalar(inst_type)))
            }
            ast::Instruction::Not(t, a) => ast::Instruction::Not(t, a.map(visitor, t.to_type())),
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
                ast::Instruction::Cvt(d, a.map_cvt(visitor, dst_t, src_t))
            }
            ast::Instruction::Shl(t, a) => {
                ast::Instruction::Shl(t, a.map_shift(visitor, t.to_type()))
            }
            ast::Instruction::St(d, a) => {
                let inst_type = d.typ;
                let param_space = d.state_space == ast::StStateSpace::Param;
                ast::Instruction::St(d, a.map(visitor, inst_type, param_space))
            }
            ast::Instruction::Bra(d, a) => ast::Instruction::Bra(d, a.map(visitor, None)),
            ast::Instruction::Ret(d) => ast::Instruction::Ret(d),
            ast::Instruction::Cvta(d, a) => {
                let inst_type = ast::Type::Scalar(ast::ScalarType::B64);
                ast::Instruction::Cvta(d, a.map(visitor, inst_type))
            }
        }
    }
}

impl VisitVariable for ast::Instruction<NormalizedArgParams> {
    fn visit_variable<
        'a,
        F: FnMut(ArgumentDescriptor<spirv::Word>, Option<ast::Type>) -> spirv::Word,
    >(
        self,
        f: &mut F,
    ) -> UnadornedStatement {
        Statement::Instruction(self.map(f))
    }
}

impl<T> ArgumentMapVisitor<NormalizedArgParams, NormalizedArgParams> for T
where
    T: FnMut(ArgumentDescriptor<spirv::Word>, Option<ast::Type>) -> spirv::Word,
{
    fn variable(
        &mut self,
        desc: ArgumentDescriptor<spirv::Word>,
        t: Option<ast::Type>,
    ) -> spirv::Word {
        self(desc, t)
    }

    fn operand(
        &mut self,
        desc: ArgumentDescriptor<ast::Operand<spirv::Word>>,
        t: ast::Type,
    ) -> ast::Operand<spirv::Word> {
        match desc.op {
            ast::Operand::Reg(id) => ast::Operand::Reg(self(desc.new_op(id), Some(t))),
            ast::Operand::Imm(imm) => ast::Operand::Imm(imm),
            ast::Operand::RegOffset(id, imm) => {
                ast::Operand::RegOffset(self(desc.new_op(id), Some(t)), imm)
            }
        }
    }

    fn src_call_operand(
        &mut self,
        desc: ArgumentDescriptor<ast::CallOperand<spirv::Word>>,
        t: ast::Type,
    ) -> ast::CallOperand<spirv::Word> {
        match desc.op {
            ast::CallOperand::Reg(id) => ast::CallOperand::Reg(self(desc.new_op(id), Some(t))),
            ast::CallOperand::Imm(imm) => ast::CallOperand::Imm(imm),
        }
    }

    fn src_vec_operand(
        &mut self,
        desc: ArgumentDescriptor<(spirv::Word, u8)>,
        (scalar_type, vector_len): (ast::MovVectorType, u8),
    ) -> (spirv::Word, u8) {
        (
            self(
                desc.new_op(desc.op.0),
                Some(ast::Type::Vector(scalar_type.into(), vector_len)),
            ),
            desc.op.1,
        )
    }
}

impl ast::Type {
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
            | ast::Instruction::MovVector(_, _)
            | ast::Instruction::Mul(_, _)
            | ast::Instruction::Add(_, _)
            | ast::Instruction::Setp(_, _)
            | ast::Instruction::SetpBool(_, _)
            | ast::Instruction::Not(_, _)
            | ast::Instruction::Cvt(_, _)
            | ast::Instruction::Cvta(_, _)
            | ast::Instruction::Shl(_, _)
            | ast::Instruction::St(_, _)
            | ast::Instruction::Ret(_)
            | ast::Instruction::Abs(_, _)
            | ast::Instruction::Call(_) => None,
        }
    }
}

impl VisitVariableExpanded for ast::Instruction<ExpandedArgParams> {
    fn visit_variable_extended<
        F: FnMut(ArgumentDescriptor<spirv::Word>, Option<ast::Type>) -> spirv::Word,
    >(
        self,
        f: &mut F,
    ) -> ExpandedStatement {
        Statement::Instruction(self.map(f))
    }
}

type Arg2 = ast::Arg2<ExpandedArgParams>;
type Arg2St = ast::Arg2St<ExpandedArgParams>;

struct CompositeRead {
    pub typ: ast::MovVectorType,
    pub dst: spirv::Word,
    pub src_composite: spirv::Word,
    pub src_index: u32,
}

struct ConstantDefinition {
    pub dst: spirv::Word,
    pub typ: ast::ScalarType,
    pub value: i128,
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
    Ptr(ast::LdStateSpace),
}

impl<T> ast::PredAt<T> {
    fn map_variable<U, F: FnMut(T) -> U>(self, f: &mut F) -> ast::PredAt<U> {
        ast::PredAt {
            not: self.not,
            label: f(self.label),
        }
    }
}

impl<'a> ast::Instruction<ast::ParsedArgParams<'a>> {
    fn map_variable<F: FnMut(&str) -> spirv::Word>(
        self,
        f: &mut F,
    ) -> ast::Instruction<NormalizedArgParams> {
        match self {
            ast::Instruction::Call(call) => {
                let call_inst = ast::CallInst {
                    uniform: call.uniform,
                    ret_params: call.ret_params.into_iter().map(|p| f(p)).collect(),
                    func: f(call.func),
                    param_list: call
                        .param_list
                        .into_iter()
                        .map(|p| p.map_variable(f))
                        .collect(),
                };
                ast::Instruction::Call(call_inst)
            }
            i => i.map(f),
        }
    }
}

impl<T: ArgParamsEx> ast::Arg1<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: Option<ast::Type>,
    ) -> ast::Arg1<U> {
        ast::Arg1 {
            src: visitor.variable(
                ArgumentDescriptor {
                    op: self.src,
                    is_dst: false,
                    is_pointer: false,
                },
                t,
            ),
        }
    }
}

impl<T: ArgParamsEx> ast::Arg2<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: ast::Type,
    ) -> ast::Arg2<U> {
        ast::Arg2 {
            dst: visitor.variable(
                ArgumentDescriptor {
                    op: self.dst,
                    is_dst: true,
                    is_pointer: false,
                },
                Some(t),
            ),
            src: visitor.operand(
                ArgumentDescriptor {
                    op: self.src,
                    is_dst: false,
                    is_pointer: false,
                },
                t,
            ),
        }
    }

    fn map_ld<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: ast::Type,
        is_src_pointer: bool,
    ) -> ast::Arg2<U> {
        ast::Arg2 {
            dst: visitor.variable(
                ArgumentDescriptor {
                    op: self.dst,
                    is_dst: true,
                    is_pointer: false,
                },
                Some(t),
            ),
            src: visitor.operand(
                ArgumentDescriptor {
                    op: self.src,
                    is_dst: false,
                    is_pointer: is_src_pointer,
                },
                t,
            ),
        }
    }

    fn map_cvt<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        dst_t: ast::Type,
        src_t: ast::Type,
    ) -> ast::Arg2<U> {
        ast::Arg2 {
            dst: visitor.variable(
                ArgumentDescriptor {
                    op: self.dst,
                    is_dst: true,
                    is_pointer: false,
                },
                Some(dst_t),
            ),
            src: visitor.operand(
                ArgumentDescriptor {
                    op: self.src,
                    is_dst: false,
                    is_pointer: false,
                },
                src_t,
            ),
        }
    }
}

impl<T: ArgParamsEx> ast::Arg2St<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: ast::Type,
        param_space: bool,
    ) -> ast::Arg2St<U> {
        ast::Arg2St {
            src1: visitor.operand(
                ArgumentDescriptor {
                    op: self.src1,
                    is_dst: param_space,
                    is_pointer: !param_space,
                },
                t,
            ),
            src2: visitor.operand(
                ArgumentDescriptor {
                    op: self.src2,
                    is_dst: false,
                    is_pointer: false,
                },
                t,
            ),
        }
    }
}

impl<T: ArgParamsEx> ast::Arg2Vec<T> {
    fn dst(&self) -> &T::ID {
        match self {
            ast::Arg2Vec::Dst((d, _), _, _)
            | ast::Arg2Vec::Src(d, _)
            | ast::Arg2Vec::Both((d, _), _, _) => d,
        }
    }

    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        (scalar_type, vec_len): (ast::MovVectorType, u8),
    ) -> ast::Arg2Vec<U> {
        match self {
            ast::Arg2Vec::Dst((dst, len), composite_src, scalar_src) => ast::Arg2Vec::Dst(
                (
                    visitor.variable(
                        ArgumentDescriptor {
                            op: dst,
                            is_dst: true,
                            is_pointer: false,
                        },
                        Some(ast::Type::Scalar(scalar_type.into())),
                    ),
                    len,
                ),
                visitor.variable(
                    ArgumentDescriptor {
                        op: composite_src,
                        is_dst: false,
                        is_pointer: false,
                    },
                    Some(ast::Type::Scalar(scalar_type.into())),
                ),
                visitor.variable(
                    ArgumentDescriptor {
                        op: scalar_src,
                        is_dst: false,
                        is_pointer: false,
                    },
                    Some(ast::Type::Scalar(scalar_type.into())),
                ),
            ),
            ast::Arg2Vec::Src(dst, src) => ast::Arg2Vec::Src(
                visitor.variable(
                    ArgumentDescriptor {
                        op: dst,
                        is_dst: true,
                        is_pointer: false,
                    },
                    Some(ast::Type::Scalar(scalar_type.into())),
                ),
                visitor.src_vec_operand(
                    ArgumentDescriptor {
                        op: src,
                        is_dst: false,
                        is_pointer: false,
                    },
                    (scalar_type, vec_len),
                ),
            ),
            ast::Arg2Vec::Both((dst, len), composite_src, src) => ast::Arg2Vec::Both(
                (
                    visitor.variable(
                        ArgumentDescriptor {
                            op: dst,
                            is_dst: true,
                            is_pointer: false,
                        },
                        Some(ast::Type::Scalar(scalar_type.into())),
                    ),
                    len,
                ),
                visitor.variable(
                    ArgumentDescriptor {
                        op: composite_src,
                        is_dst: false,
                        is_pointer: false,
                    },
                    Some(ast::Type::Scalar(scalar_type.into())),
                ),
                visitor.src_vec_operand(
                    ArgumentDescriptor {
                        op: src,
                        is_dst: false,
                        is_pointer: false,
                    },
                    (scalar_type, vec_len),
                ),
            ),
        }
    }
}

impl<T: ArgParamsEx> ast::Arg3<T> {
    fn map_non_shift<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: ast::Type,
    ) -> ast::Arg3<U> {
        ast::Arg3 {
            dst: visitor.variable(
                ArgumentDescriptor {
                    op: self.dst,
                    is_dst: true,
                    is_pointer: false,
                },
                Some(t),
            ),
            src1: visitor.operand(
                ArgumentDescriptor {
                    op: self.src1,
                    is_dst: false,
                    is_pointer: false,
                },
                t,
            ),
            src2: visitor.operand(
                ArgumentDescriptor {
                    op: self.src2,
                    is_dst: false,
                    is_pointer: false,
                },
                t,
            ),
        }
    }

    fn map_shift<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: ast::Type,
    ) -> ast::Arg3<U> {
        ast::Arg3 {
            dst: visitor.variable(
                ArgumentDescriptor {
                    op: self.dst,
                    is_dst: true,
                    is_pointer: false,
                },
                Some(t),
            ),
            src1: visitor.operand(
                ArgumentDescriptor {
                    op: self.src1,
                    is_dst: false,
                    is_pointer: false,
                },
                t,
            ),
            src2: visitor.operand(
                ArgumentDescriptor {
                    op: self.src2,
                    is_dst: false,
                    is_pointer: false,
                },
                ast::Type::Scalar(ast::ScalarType::U32),
            ),
        }
    }
}

impl<T: ArgParamsEx> ast::Arg4<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: ast::Type,
    ) -> ast::Arg4<U> {
        ast::Arg4 {
            dst1: visitor.variable(
                ArgumentDescriptor {
                    op: self.dst1,
                    is_dst: true,
                    is_pointer: false,
                },
                Some(ast::Type::Scalar(ast::ScalarType::Pred)),
            ),
            dst2: self.dst2.map(|dst2| {
                visitor.variable(
                    ArgumentDescriptor {
                        op: dst2,
                        is_dst: true,
                        is_pointer: false,
                    },
                    Some(ast::Type::Scalar(ast::ScalarType::Pred)),
                )
            }),
            src1: visitor.operand(
                ArgumentDescriptor {
                    op: self.src1,
                    is_dst: false,
                    is_pointer: false,
                },
                t,
            ),
            src2: visitor.operand(
                ArgumentDescriptor {
                    op: self.src2,
                    is_dst: false,
                    is_pointer: false,
                },
                t,
            ),
        }
    }
}

impl<T: ArgParamsEx> ast::Arg5<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: ast::Type,
    ) -> ast::Arg5<U> {
        ast::Arg5 {
            dst1: visitor.variable(
                ArgumentDescriptor {
                    op: self.dst1,
                    is_dst: true,
                    is_pointer: false,
                },
                Some(ast::Type::Scalar(ast::ScalarType::Pred)),
            ),
            dst2: self.dst2.map(|dst2| {
                visitor.variable(
                    ArgumentDescriptor {
                        op: dst2,
                        is_dst: true,
                        is_pointer: false,
                    },
                    Some(ast::Type::Scalar(ast::ScalarType::Pred)),
                )
            }),
            src1: visitor.operand(
                ArgumentDescriptor {
                    op: self.src1,
                    is_dst: false,
                    is_pointer: false,
                },
                t,
            ),
            src2: visitor.operand(
                ArgumentDescriptor {
                    op: self.src2,
                    is_dst: false,
                    is_pointer: false,
                },
                t,
            ),
            src3: visitor.operand(
                ArgumentDescriptor {
                    op: self.src3,
                    is_dst: false,
                    is_pointer: false,
                },
                ast::Type::Scalar(ast::ScalarType::Pred),
            ),
        }
    }
}

impl<T> ast::CallOperand<T> {
    fn map_variable<U, F: FnMut(T) -> U>(self, f: &mut F) -> ast::CallOperand<U> {
        match self {
            ast::CallOperand::Reg(id) => ast::CallOperand::Reg(f(id)),
            ast::CallOperand::Imm(x) => ast::CallOperand::Imm(x),
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

fn insert_with_conversions<T, ToInstruction: FnOnce(T) -> ast::Instruction<ExpandedArgParams>>(
    func: &mut Vec<ExpandedStatement>,
    id_def: &mut NumericIdResolver,
    mut instr: T,
    pre_conv_src: impl ExactSizeIterator<Item = ImplicitConversion>,
    pre_conv_dst: impl ExactSizeIterator<Item = ImplicitConversion>,
    mut post_conv: Vec<ImplicitConversion>,
    mut src: impl FnMut(&mut T) -> &mut spirv::Word,
    mut dst: impl FnMut(&mut T) -> &mut spirv::Word,
    to_inst: ToInstruction,
) {
    insert_with_conversions_pre_conv(func, id_def, &mut instr, pre_conv_src, &mut src);
    insert_with_conversions_pre_conv(func, id_def, &mut instr, pre_conv_dst, &mut dst);
    if post_conv.len() > 0 {
        let new_id = id_def.new_id(Some(post_conv[0].from));
        post_conv[0].src = new_id;
        post_conv.last_mut().unwrap().dst = *dst(&mut instr);
        *dst(&mut instr) = new_id;
    }
    func.push(Statement::Instruction(to_inst(instr)));
    for conv in post_conv {
        func.push(Statement::Conversion(conv));
    }
}

fn insert_with_conversions_pre_conv<T>(
    func: &mut Vec<ExpandedStatement>,
    id_def: &mut NumericIdResolver,
    mut instr: &mut T,
    pre_conv: impl ExactSizeIterator<Item = ImplicitConversion>,
    src: &mut impl FnMut(&mut T) -> &mut spirv::Word,
) {
    let pre_conv_len = pre_conv.len();
    for (i, mut conv) in pre_conv.enumerate() {
        let original_src = src(&mut instr);
        if i == 0 {
            conv.src = *original_src;
        }
        if i == pre_conv_len - 1 {
            let new_id = id_def.new_id(Some(conv.to));
            conv.dst = new_id;
            *original_src = new_id;
        }
        func.push(Statement::Conversion(conv));
    }
}

fn get_implicit_conversions_ld_dst<
    ShouldConvert: FnOnce(ast::Type, ast::Type) -> Option<ConversionKind>,
>(
    id_def: &mut NumericIdResolver,
    instr_type: ast::Type,
    dst: spirv::Word,
    should_convert: ShouldConvert,
    in_reverse: bool,
) -> Option<ImplicitConversion> {
    let dst_type = id_def.get_type(dst).unwrap_or_else(|| todo!());
    if let Some(conv) = should_convert(dst_type, instr_type) {
        Some(ImplicitConversion {
            src: u32::max_value(),
            dst: u32::max_value(),
            from: if !in_reverse { dst_type } else { instr_type },
            to: if !in_reverse { instr_type } else { dst_type },
            kind: conv,
        })
    } else {
        None
    }
}

fn get_implicit_conversions_ld_src(
    id_def: &mut NumericIdResolver,
    instr_type: ast::Type,
    state_space: ast::LdStateSpace,
    src: spirv::Word,
) -> Vec<ImplicitConversion> {
    let src_type = id_def.get_type(src).unwrap_or_else(|| todo!());
    match state_space {
        ast::LdStateSpace::Param => {
            if src_type != instr_type {
                vec![
                    ImplicitConversion {
                        src: u32::max_value(),
                        dst: u32::max_value(),
                        from: src_type,
                        to: instr_type,
                        kind: ConversionKind::Default,
                    };
                    1
                ]
            } else {
                Vec::new()
            }
        }
        ast::LdStateSpace::Generic | ast::LdStateSpace::Global => {
            let new_src_type = ast::Type::Scalar(ast::ScalarType::from_parts(
                mem::size_of::<usize>() as u8,
                ScalarKind::Bit,
            ));
            let mut result = Vec::new();
            // HACK ALERT
            // IGC currently segfaults if you bitcast integer -> ptr, that's why we emit an
            // additional S64/U64 -> B64 conversion here, so the SPIR-V emission is easier
            // TODO: error out if the src is not B64/U64/S64
            if let ast::Type::Scalar(scalar_src_type) = src_type {
                if scalar_src_type.kind() == ScalarKind::Signed {
                    result.push(ImplicitConversion {
                        src: u32::max_value(),
                        dst: u32::max_value(),
                        from: src_type,
                        to: new_src_type,
                        kind: ConversionKind::Default,
                    });
                }
            }
            result.push(ImplicitConversion {
                src: u32::max_value(),
                dst: u32::max_value(),
                from: src_type,
                to: instr_type,
                kind: ConversionKind::Ptr(state_space),
            });
            if result.len() == 2 {
                let new_id = id_def.new_id(Some(new_src_type));
                result[0].dst = new_id;
                result[1].src = new_id;
                result[1].from = new_src_type;
            }
            result
        }
        _ => todo!(),
    }
}
fn insert_implicit_conversions_ld_src(
    func: &mut Vec<ExpandedStatement>,
    instr_type: ast::Type,
    id_def: &mut NumericIdResolver,
    state_space: ast::LdStateSpace,
    src: spirv::Word,
) -> spirv::Word {
    match state_space {
        ast::LdStateSpace::Param => insert_implicit_conversions_ld_src_impl(
            func,
            id_def,
            instr_type,
            src,
            should_convert_ld_param_src,
        ),
        ast::LdStateSpace::Generic | ast::LdStateSpace::Global => {
            let new_src_type = ast::Type::Scalar(ast::ScalarType::from_parts(
                mem::size_of::<usize>() as u8,
                ScalarKind::Bit,
            ));
            let new_src = insert_implicit_conversions_ld_src_impl(
                func,
                id_def,
                new_src_type,
                src,
                should_convert_ld_generic_src_to_bitcast,
            );
            insert_conversion_src(
                func,
                id_def,
                new_src,
                new_src_type,
                instr_type,
                ConversionKind::Ptr(state_space),
            )
        }
        _ => todo!(),
    }
}

fn insert_implicit_conversions_ld_src_impl<
    ShouldConvert: FnOnce(ast::Type, ast::Type) -> Option<ConversionKind>,
>(
    func: &mut Vec<ExpandedStatement>,
    id_def: &mut NumericIdResolver,
    instr_type: ast::Type,
    src: spirv::Word,
    should_convert: ShouldConvert,
) -> spirv::Word {
    let src_type = id_def.get_type(src);
    if let Some(conv) = should_convert(src_type.unwrap(), instr_type) {
        insert_conversion_src(func, id_def, src, src_type.unwrap(), instr_type, conv)
    } else {
        src
    }
}

fn should_convert_ld_param_src(
    src_type: ast::Type,
    instr_type: ast::Type,
) -> Option<ConversionKind> {
    if src_type != instr_type {
        return Some(ConversionKind::Default);
    }
    None
}

// HACK ALERT
// IGC currently segfaults if you bitcast integer -> ptr, that's why we emit an
// additional S64/U64 -> B64 conversion here, so the SPIR-V emission is easier
fn should_convert_ld_generic_src_to_bitcast(
    src_type: ast::Type,
    _instr_type: ast::Type,
) -> Option<ConversionKind> {
    if let ast::Type::Scalar(src_type) = src_type {
        if src_type.kind() == ScalarKind::Signed {
            return Some(ConversionKind::Default);
        }
    }
    None
}

#[must_use]
fn insert_conversion_src(
    func: &mut Vec<ExpandedStatement>,
    id_def: &mut NumericIdResolver,
    src: spirv::Word,
    src_type: ast::Type,
    instr_type: ast::Type,
    conv: ConversionKind,
) -> spirv::Word {
    let temp_src = id_def.new_id(Some(instr_type));
    func.push(Statement::Conversion(ImplicitConversion {
        src: src,
        dst: temp_src,
        from: src_type,
        to: instr_type,
        kind: conv,
    }));
    temp_src
}

/*
fn insert_with_implicit_conversion_dst<
    T,
    ShouldConvert: FnOnce(ast::StateSpace, ast::Type, ast::Type) -> Option<ConversionKind>,
    Setter: Fn(&mut T) -> &mut spirv::Word,
    ToInstruction: FnOnce(T) -> ast::Instruction<ExpandedArgParams>,
>(
    func: &mut Vec<ExpandedStatement>,
    instr_type: ast::Type,
    id_def: &mut NumericIdResolver,
    should_convert: ShouldConvert,
    mut t: T,
    setter: Setter,
    to_inst: ToInstruction,
) {
    let dst = setter(&mut t);
    let dst_type = id_def.get_type(*dst);
    let dst_coercion = should_convert(dst_type.unwrap(), instr_type)
        .map(|conv| get_conversion_dst(id_def, dst, instr_type, dst_type.unwrap(), conv));
    func.push(Statement::Instruction(to_inst(t)));
    if let Some(conv) = dst_coercion {
        func.push(conv);
    }
}
*/

#[must_use]
fn get_conversion_dst(
    id_def: &mut NumericIdResolver,
    dst: &mut spirv::Word,
    instr_type: ast::Type,
    dst_type: ast::Type,
    kind: ConversionKind,
) -> ExpandedStatement {
    let original_dst = *dst;
    let temp_dst = id_def.new_id(Some(instr_type));
    *dst = temp_dst;
    Statement::Conversion(ImplicitConversion {
        src: temp_dst,
        dst: original_dst,
        from: instr_type,
        to: dst_type,
        kind: kind,
    })
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

fn insert_implicit_bitcasts(
    func: &mut Vec<ExpandedStatement>,
    id_def: &mut NumericIdResolver,
    stmt: impl VisitVariableExpanded,
) {
    let mut dst_coercion = None;
    let instr = stmt.visit_variable_extended(&mut |mut desc, typ| {
        let id_type_from_instr = match typ {
            Some(t) => t,
            None => return desc.op,
        };
        let id_actual_type = id_def.get_type(desc.op).unwrap();
        if should_bitcast(id_type_from_instr, id_def.get_type(desc.op).unwrap()) {
            if desc.is_dst {
                dst_coercion = Some(get_conversion_dst(
                    id_def,
                    &mut desc.op,
                    id_type_from_instr,
                    id_actual_type,
                    ConversionKind::Default,
                ));
                desc.op
            } else {
                insert_conversion_src(
                    func,
                    id_def,
                    desc.op,
                    id_actual_type,
                    id_type_from_instr,
                    ConversionKind::Default,
                )
            }
        } else {
            desc.op
        }
    });
    func.push(instr);
    if let Some(cond) = dst_coercion {
        func.push(cond);
    }
}
impl<'a> ast::MethodDecl<'a, ast::ParsedArgParams<'a>> {
    fn name(&self) -> &'a str {
        match self {
            ast::MethodDecl::Kernel(name, _) => name,
            ast::MethodDecl::Func(_, name, _) => name,
        }
    }
}

impl<'a, P: ArgParamsEx<ID = spirv::Word>> ast::MethodDecl<'a, P> {
    fn visit_args(&self, f: &mut impl FnMut(&ast::FnArgument<P>)) {
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
