use crate::ast;
use rspirv::dr;
use std::collections::{HashMap, HashSet};
use std::{borrow::Cow, iter, mem};

use rspirv::binary::Assemble;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum SpirvType {
    Base(ast::ScalarType),
    Extended(ast::ExtendedScalarType),
    Pointer(ast::Type, spirv::StorageClass),
}

impl From<ast::Type> for SpirvType {
    fn from(t: ast::Type) -> Self {
        match t {
            ast::Type::Scalar(t) => SpirvType::Base(t),
            ast::Type::ExtendedScalar(t) => SpirvType::Extended(t),
        }
    }
}

struct TypeWordMap {
    void: spirv::Word,
    complex: HashMap<SpirvType, spirv::Word>,
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
        *self
            .complex
            .entry(SpirvType::Base(t))
            .or_insert_with(|| match t {
                ast::ScalarType::B8 | ast::ScalarType::U8 => b.type_int(8, 0),
                ast::ScalarType::B16 | ast::ScalarType::U16 => b.type_int(16, 0),
                ast::ScalarType::B32 | ast::ScalarType::U32 => b.type_int(32, 0),
                ast::ScalarType::B64 | ast::ScalarType::U64 => b.type_int(64, 0),
                ast::ScalarType::S8 => b.type_int(8, 1),
                ast::ScalarType::S16 => b.type_int(16, 1),
                ast::ScalarType::S32 => b.type_int(32, 1),
                ast::ScalarType::S64 => b.type_int(64, 1),
                ast::ScalarType::F16 => b.type_float(16),
                ast::ScalarType::F32 => b.type_float(32),
                ast::ScalarType::F64 => b.type_float(64),
            })
    }

    fn get_or_add_extended(
        &mut self,
        b: &mut dr::Builder,
        t: ast::ExtendedScalarType,
    ) -> spirv::Word {
        *self
            .complex
            .entry(SpirvType::Extended(t))
            .or_insert_with(|| match t {
                ast::ExtendedScalarType::Pred => b.type_bool(),
                ast::ExtendedScalarType::F16x2 => todo!(),
            })
    }

    fn get_or_add(&mut self, b: &mut dr::Builder, t: SpirvType) -> spirv::Word {
        match t {
            SpirvType::Base(scalar) => self.get_or_add_scalar(b, scalar),
            SpirvType::Extended(ext) => self.get_or_add_extended(b, ext),
            SpirvType::Pointer(typ, storage) => {
                let base = match typ {
                    ast::Type::Scalar(scalar) => self.get_or_add_scalar(b, scalar),
                    ast::Type::ExtendedScalar(ext) => self.get_or_add_extended(b, ext),
                };
                *self
                    .complex
                    .entry(t)
                    .or_insert_with(|| b.type_pointer(None, storage, base))
            }
        }
    }

    fn get_or_add_fn<Args: Iterator<Item = SpirvType>>(
        &mut self,
        b: &mut dr::Builder,
        args: Args,
    ) -> spirv::Word {
        let params = args.map(|a| self.get_or_add(b, a)).collect::<Vec<_>>();
        b.type_function(self.void(), params)
    }
}

pub fn to_spirv_module(ast: ast::Module) -> Result<dr::Module, dr::Error> {
    let mut builder = dr::Builder::new();
    // https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_logicallayout_a_logical_layout_of_a_module
    builder.set_version(1, 0);
    emit_capabilities(&mut builder);
    emit_extensions(&mut builder);
    let opencl_id = emit_opencl_import(&mut builder);
    emit_memory_model(&mut builder);
    let mut map = TypeWordMap::new(&mut builder);
    for f in ast.functions {
        emit_function(&mut builder, &mut map, opencl_id, f)?;
    }
    Ok(builder.module())
}

pub fn to_spirv(ast: ast::Module) -> Result<Vec<u32>, dr::Error> {
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

fn emit_function<'a>(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    opencl_id: spirv::Word,
    f: ast::Function<'a>,
) -> Result<spirv::Word, rspirv::dr::Error> {
    let func_type = get_function_type(builder, map, &f.args);
    let func_id =
        builder.begin_function(map.void(), None, spirv::FunctionControl::NONE, func_type)?;
    if f.kernel {
        builder.entry_point(spirv::ExecutionModel::Kernel, func_id, f.name, &[]);
    }
    let (mut func_body, unique_ids) = to_ssa(&f.args, f.body);
    let id_offset = builder.reserve_ids(unique_ids);
    emit_function_args(builder, id_offset, map, &f.args);
    func_body = apply_id_offset(func_body, id_offset);
    emit_function_body_ops(builder, map, opencl_id, &func_body)?;
    builder.end_function()?;
    Ok(func_id)
}

fn apply_id_offset(func_body: Vec<ExpandedStatement>, id_offset: u32) -> Vec<ExpandedStatement> {
    func_body
        .into_iter()
        .map(|s| s.visit_variable(&mut |id| id + id_offset))
        .collect()
}

fn to_ssa<'a, 'b>(
    f_args: &'b [ast::Argument<'a>],
    f_body: Vec<ast::Statement<ast::ParsedArgParams<'a>>>,
) -> (Vec<ExpandedStatement>, spirv::Word) {
    let (normalized_ids, mut id_def) = normalize_identifiers(&f_args, f_body);
    let normalized_statements = normalize_predicates(normalized_ids, &mut id_def);
    let ssa_statements = insert_mem_ssa_statements(normalized_statements, &mut id_def);
    let expanded_statements = expand_arguments(ssa_statements, &mut id_def);
    let expanded_statements = insert_implicit_conversions(expanded_statements, &mut id_def);
    let labeled_statements = normalize_labels(expanded_statements, &mut id_def);
    (labeled_statements, id_def.ids_count())
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
            Statement::Variable(_, _, _)
            | Statement::LoadVar(_, _)
            | Statement::StoreVar(_, _)
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
    func: Vec<ast::Statement<NormalizedArgParams>>,
    id_def: &mut NumericIdResolver,
) -> Vec<NormalizedStatement> {
    let mut result = Vec::with_capacity(func.len());
    for s in func {
        match s {
            ast::Statement::Label(id) => result.push(Statement::Label(id)),
            ast::Statement::Instruction(pred, inst) => {
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
            ast::Statement::Variable(var) => {
                result.push(Statement::Variable(var.name, var.v_type, var.space))
            }
        }
    }
    result
}

fn insert_mem_ssa_statements(
    func: Vec<NormalizedStatement>,
    id_def: &mut NumericIdResolver,
) -> Vec<NormalizedStatement> {
    let mut result = Vec::with_capacity(func.len());
    for s in func {
        match s {
            Statement::Instruction(inst) => match inst {
                ast::Instruction::Ld(
                    ld
                    @
                    ast::LdData {
                        state_space: ast::LdStateSpace::Param,
                        ..
                    },
                    arg,
                ) => {
                    result.push(Statement::Instruction(ast::Instruction::Ld(ld, arg)));
                }
                inst => {
                    let mut post_statements = Vec::new();
                    let inst = inst.visit_variable(&mut |id, is_dst, id_type| {
                        let id_type = match id_type {
                            Some(t) => t,
                            None => return id,
                        };
                        let generated_id = id_def.new_id(Some(id_type));
                        if !is_dst {
                            result.push(Statement::LoadVar(
                                Arg2 {
                                    dst: generated_id,
                                    src: id,
                                },
                                id_type,
                            ));
                        } else {
                            post_statements.push(Statement::StoreVar(
                                Arg2St {
                                    src1: id,
                                    src2: generated_id,
                                },
                                id_type,
                            ));
                        }
                        generated_id
                    });
                    result.push(Statement::Instruction(inst));
                    result.append(&mut post_statements);
                }
            },
            Statement::Conditional(mut bra) => {
                let generated_id = id_def.new_id(Some(ast::Type::ExtendedScalar(
                    ast::ExtendedScalarType::Pred,
                )));
                result.push(Statement::LoadVar(
                    Arg2 {
                        dst: generated_id,
                        src: bra.predicate,
                    },
                    ast::Type::ExtendedScalar(ast::ExtendedScalarType::Pred),
                ));
                bra.predicate = generated_id;
                result.push(Statement::Conditional(bra));
            }
            s @ Statement::Variable(_, _, _) | s @ Statement::Label(_) => result.push(s),
            Statement::LoadVar(_, _)
            | Statement::StoreVar(_, _)
            | Statement::Conversion(_)
            | Statement::Constant(_) => unreachable!(),
        }
    }
    result
}

fn expand_arguments(
    func: Vec<NormalizedStatement>,
    id_def: &mut NumericIdResolver,
) -> Vec<ExpandedStatement> {
    let mut result = Vec::with_capacity(func.len());
    for s in func {
        match s {
            Statement::Instruction(inst) => {
                let mut visitor = FlattenArguments::new(&mut result, id_def);
                let new_inst = inst.map(&mut visitor);
                result.push(Statement::Instruction(new_inst));
            }
            Statement::Variable(id, typ, ss) => result.push(Statement::Variable(id, typ, ss)),
            Statement::Label(id) => result.push(Statement::Label(id)),
            Statement::Conditional(bra) => result.push(Statement::Conditional(bra)),
            Statement::LoadVar(arg, typ) => result.push(Statement::LoadVar(arg, typ)),
            Statement::StoreVar(arg, typ) => result.push(Statement::StoreVar(arg, typ)),
            Statement::Conversion(_) | Statement::Constant(_) => unreachable!(),
        }
    }
    result
}

struct FlattenArguments<'a> {
    func: &'a mut Vec<ExpandedStatement>,
    id_def: &'a mut NumericIdResolver,
}

impl<'a> FlattenArguments<'a> {
    fn new(func: &'a mut Vec<ExpandedStatement>, id_def: &'a mut NumericIdResolver) -> Self {
        FlattenArguments { func, id_def }
    }
}

impl<'a> ArgumentMapVisitor<NormalizedArgParams, ExpandedArgParams> for FlattenArguments<'a> {
    fn dst_variable(&mut self, x: spirv::Word, _: Option<ast::Type>) -> spirv::Word {
        x
    }

    fn src_operand(&mut self, op: ast::Operand<spirv::Word>, t: Option<ast::Type>) -> spirv::Word {
        match op {
            ast::Operand::Reg(r) => r,
            ast::Operand::Imm(x) => {
                if let Some(typ) = t {
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
                } else {
                    todo!()
                }
            }
            ast::Operand::RegOffset(reg, offset) => {
                if let Some(typ) = t {
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
                    let result_id = self.id_def.new_id(t);
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
                } else {
                    todo!()
                }
            }
        }
    }

    fn src_mov_operand(
        &mut self,
        op: ast::MovOperand<spirv::Word>,
        t: Option<ast::Type>,
    ) -> spirv::Word {
        match op {
            ast::MovOperand::Op(opr) => self.src_operand(opr, t),
            ast::MovOperand::Vec(_, _) => todo!(),
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
   - generic ld: for instruction `ld x, [y]`, y must be of type b64/u64/s64,
     which is bitcast to a pointer, dereferenced and then documented special
     ld/st/cvt conversion rules are applied to dst
   - generic st: for instruction `st [x], y`, x must be of type b64/u64/s64,
     which is bitcast to a pointer
*/
fn insert_implicit_conversions(
    func: Vec<ExpandedStatement>,
    id_def: &mut NumericIdResolver,
) -> Vec<ExpandedStatement> {
    let mut result = Vec::with_capacity(func.len());
    for s in func.into_iter() {
        match s {
            Statement::Instruction(inst) => match inst {
                ast::Instruction::Ld(ld, mut arg) => {
                    arg.src = insert_implicit_conversions_ld_src(
                        &mut result,
                        ast::Type::Scalar(ld.typ),
                        id_def,
                        ld.state_space,
                        arg.src,
                    );
                    insert_with_implicit_conversion_dst(
                        &mut result,
                        ld.typ,
                        id_def,
                        should_convert_relaxed_dst,
                        arg,
                        |arg| &mut arg.dst,
                        |arg| ast::Instruction::Ld(ld, arg),
                    );
                }
                ast::Instruction::St(st, mut arg) => {
                    let arg_src2_type = id_def.get_type(arg.src2);
                    if let Some(conv) = should_convert_relaxed_src(arg_src2_type, st.typ) {
                        arg.src2 = insert_conversion_src(
                            &mut result,
                            id_def,
                            arg.src2,
                            arg_src2_type,
                            ast::Type::Scalar(st.typ),
                            conv,
                        );
                    }
                    arg.src1 = insert_implicit_conversions_ld_src(
                        &mut result,
                        ast::Type::Scalar(st.typ),
                        id_def,
                        st.state_space.to_ld_ss(),
                        arg.src1,
                    );
                    result.push(Statement::Instruction(ast::Instruction::St(st, arg)));
                }
                inst @ _ => insert_implicit_bitcasts(&mut result, id_def, inst),
            },
            s @ Statement::Conditional(_)
            | s @ Statement::Label(_)
            | s @ Statement::Constant(_)
            | s @ Statement::Variable(_, _, _)
            | s @ Statement::LoadVar(_, _)
            | s @ Statement::StoreVar(_, _) => result.push(s),
            Statement::Conversion(_) => unreachable!(),
        }
    }
    result
}

fn get_function_type(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    args: &[ast::Argument],
) -> spirv::Word {
    map.get_or_add_fn(builder, args.iter().map(|arg| SpirvType::Base(arg.a_type)))
}

fn emit_function_args(
    builder: &mut dr::Builder,
    id_offset: spirv::Word,
    map: &mut TypeWordMap,
    args: &[ast::Argument],
) {
    let mut id = id_offset;
    for arg in args {
        let result_type = map.get_or_add_scalar(builder, arg.a_type);
        let inst = dr::Instruction::new(
            spirv::Op::FunctionParameter,
            Some(result_type),
            Some(id),
            Vec::new(),
        );
        builder.function.as_mut().unwrap().parameters.push(inst);
        id += 1;
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
            Statement::Variable(id, typ, ss) => {
                let type_id = map.get_or_add(
                    builder,
                    SpirvType::Pointer(*typ, spirv::StorageClass::Function),
                );
                if *ss != ast::StateSpace::Reg {
                    todo!()
                }
                builder.variable(type_id, Some(*id), spirv::StorageClass::Function, None);
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
                // SPIR-V does not support marking jumps as guaranteed-converged
                ast::Instruction::Bra(_, arg) => {
                    builder.branch(arg.src)?;
                }
                ast::Instruction::Ld(data, arg) => {
                    if data.qualifier != ast::LdStQualifier::Weak || data.vector.is_some() {
                        todo!()
                    }
                    let result_type = map.get_or_add_scalar(builder, data.typ);
                    match data.state_space {
                        ast::LdStateSpace::Generic => {
                            builder.load(result_type, Some(arg.dst), arg.src, None, [])?;
                        }
                        ast::LdStateSpace::Param => {
                            builder.store(arg.dst, arg.src, None, [])?;
                        }
                        _ => todo!(),
                    }
                }
                ast::Instruction::St(data, arg) => {
                    if data.qualifier != ast::LdStQualifier::Weak
                        || data.vector.is_some()
                        || data.state_space != ast::StStateSpace::Generic
                    {
                        todo!()
                    }
                    builder.store(arg.src1, arg.src2, None, &[])?;
                }
                // SPIR-V does not support ret as guaranteed-converged
                ast::Instruction::Ret(_) => builder.ret()?,
                ast::Instruction::Mov(mov, arg) => {
                    let result_type = map.get_or_add(builder, SpirvType::from(mov.typ));
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
                _ => todo!(),
            },
            Statement::LoadVar(arg, typ) => {
                let type_id = map.get_or_add(builder, SpirvType::from(*typ));
                builder.load(type_id, Some(arg.dst), arg.src, None, [])?;
            }
            Statement::StoreVar(arg, _) => {
                builder.store(arg.src1, arg.src2, None, [])?;
            }
        }
    }
    Ok(())
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
    let result_type = map.get_or_add(builder, SpirvType::Extended(ast::ExtendedScalarType::Pred));
    let result_id = Some(arg.dst1);
    let operand_1 = arg.src1;
    let operand_2 = arg.src2;
    match (setp.cmp_op, setp.typ.kind()) {
        (ast::SetpCompareOp::Eq, ScalarKind::Signed)
        | (ast::SetpCompareOp::Eq, ScalarKind::Unsigned)
        | (ast::SetpCompareOp::Eq, ScalarKind::Byte) => {
            builder.i_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::Eq, ScalarKind::Float) => {
            builder.f_ord_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::NotEq, ScalarKind::Signed)
        | (ast::SetpCompareOp::NotEq, ScalarKind::Unsigned)
        | (ast::SetpCompareOp::NotEq, ScalarKind::Byte) => {
            builder.i_not_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::NotEq, ScalarKind::Float) => {
            builder.f_ord_not_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::Less, ScalarKind::Unsigned)
        | (ast::SetpCompareOp::Less, ScalarKind::Byte) => {
            builder.u_less_than(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::Less, ScalarKind::Signed) => {
            builder.s_less_than(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::Less, ScalarKind::Float) => {
            builder.f_ord_less_than(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::LessOrEq, ScalarKind::Unsigned)
        | (ast::SetpCompareOp::LessOrEq, ScalarKind::Byte) => {
            builder.u_less_than_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::LessOrEq, ScalarKind::Signed) => {
            builder.s_less_than_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::LessOrEq, ScalarKind::Float) => {
            builder.f_ord_less_than_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::Greater, ScalarKind::Unsigned)
        | (ast::SetpCompareOp::Greater, ScalarKind::Byte) => {
            builder.u_greater_than(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::Greater, ScalarKind::Signed) => {
            builder.s_greater_than(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::Greater, ScalarKind::Float) => {
            builder.f_ord_greater_than(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::GreaterOrEq, ScalarKind::Unsigned)
        | (ast::SetpCompareOp::GreaterOrEq, ScalarKind::Byte) => {
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
    let inst_type = map.get_or_add(builder, SpirvType::Base(desc.typ.into()));
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
    let inst_type = map.get_or_add(builder, SpirvType::Base(ctr.typ.into()));
    builder.i_add(inst_type, Some(arg.dst), arg.src1, arg.src2)?;
    Ok(())
}

fn emit_implicit_conversion(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    cv: &ImplicitConversion,
) -> Result<(), dr::Error> {
    let (from_type, to_type) = match (cv.from, cv.to) {
        (ast::Type::Scalar(from), ast::Type::Scalar(to)) => (from, to),
        _ => todo!(),
    };
    match cv.kind {
        ConversionKind::Ptr => {
            let dst_type = map.get_or_add(
                builder,
                SpirvType::Pointer(
                    ast::Type::Scalar(to_type),
                    spirv_headers::StorageClass::Generic,
                ),
            );
            builder.convert_u_to_ptr(dst_type, Some(cv.dst), cv.src)?;
        }
        ConversionKind::Default => {
            if from_type.width() == to_type.width() {
                let dst_type = map.get_or_add_scalar(builder, to_type);
                if from_type.kind() == ScalarKind::Unsigned && to_type.kind() == ScalarKind::Byte
                    || from_type.kind() == ScalarKind::Byte
                        && to_type.kind() == ScalarKind::Unsigned
                {
                    // It is noop, but another instruction expects result of this conversion
                    builder.copy_object(dst_type, Some(cv.dst), cv.src)?;
                }
                builder.bitcast(dst_type, Some(cv.dst), cv.src)?;
            } else {
                let as_unsigned_type = map.get_or_add_scalar(
                    builder,
                    ast::ScalarType::from_parts(from_type.width(), ScalarKind::Unsigned),
                );
                let as_unsigned = builder.bitcast(as_unsigned_type, None, cv.src)?;
                let as_unsigned_wide_type =
                    ast::ScalarType::from_parts(to_type.width(), ScalarKind::Unsigned);
                let as_unsigned_wide_spirv = map.get_or_add_scalar(
                    builder,
                    ast::ScalarType::from_parts(to_type.width(), ScalarKind::Unsigned),
                );
                if to_type.kind() == ScalarKind::Unsigned || to_type.kind() == ScalarKind::Byte {
                    builder.u_convert(as_unsigned_wide_spirv, Some(cv.dst), as_unsigned)?;
                } else {
                    let as_unsigned_wide =
                        builder.u_convert(as_unsigned_wide_spirv, None, as_unsigned)?;
                    emit_implicit_conversion(
                        builder,
                        map,
                        &ImplicitConversion {
                            src: as_unsigned_wide,
                            dst: cv.dst,
                            from: ast::Type::Scalar(as_unsigned_wide_type),
                            to: cv.to,
                            kind: ConversionKind::Default,
                        },
                    )?;
                }
            }
        }
        ConversionKind::SignExtend => todo!(),
    }
    Ok(())
}

// TODO: support scopes
fn normalize_identifiers<'a, 'b>(
    args: &'b [ast::Argument<'a>],
    func: Vec<ast::Statement<ast::ParsedArgParams<'a>>>,
) -> (Vec<ast::Statement<NormalizedArgParams>>, NumericIdResolver) {
    let mut id_defs = StringIdResolver::new();
    for arg in args {
        id_defs.add_def(arg.name, Some(ast::Type::Scalar(arg.a_type)));
    }
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
        expand_map_variables(&mut id_defs, &mut result, s);
    }
    (result, id_defs.finish())
}

fn expand_map_variables<'a>(
    id_defs: &mut StringIdResolver<'a>,
    result: &mut Vec<ast::Statement<NormalizedArgParams>>,
    s: ast::Statement<ast::ParsedArgParams<'a>>,
) {
    match s {
        ast::Statement::Label(name) => {
            result.push(ast::Statement::Label(id_defs.get_id(name)))
        }
        ast::Statement::Instruction(p, i) => result.push(ast::Statement::Instruction(
            p.map(|p| p.map_variable(&mut |id| id_defs.get_id(id))),
            i.map_variable(&mut |id| id_defs.get_id(id)),
        )),
        ast::Statement::Variable(var) => match var.count {
            Some(count) => {
                for new_id in id_defs.add_defs(var.name, count, var.v_type) {
                    result.push(ast::Statement::Variable(ast::Variable {
                        space: var.space,
                        v_type: var.v_type,
                        name: new_id,
                        count: None,
                    }))
                }
            }
            None => {
                let new_id = id_defs.add_def(var.name, Some(var.v_type));
                result.push(ast::Statement::Variable(ast::Variable {
                    space: var.space,
                    v_type: var.v_type,
                    name: new_id,
                    count: None,
                }));
            }
        },
    }
}

struct StringIdResolver<'a> {
    current_id: spirv::Word,
    variables: HashMap<Cow<'a, str>, spirv::Word>,
    type_check: HashMap<u32, ast::Type>,
}

impl<'a> StringIdResolver<'a> {
    fn new() -> Self {
        StringIdResolver {
            current_id: 0u32,
            variables: HashMap::new(),
            type_check: HashMap::new(),
        }
    }

    fn finish(self) -> NumericIdResolver {
        NumericIdResolver {
            current_id: self.current_id,
            type_check: self.type_check,
        }
    }

    fn get_id(&self, id: &'a str) -> spirv::Word {
        self.variables[id]
    }

    fn add_def(&mut self, id: &'a str, typ: Option<ast::Type>) -> spirv::Word {
        let numeric_id = self.current_id;
        self.variables.insert(Cow::Borrowed(id), numeric_id);
        if let Some(typ) = typ {
            self.type_check.insert(numeric_id, typ);
        }
        self.current_id += 1;
        numeric_id
    }

    #[must_use]
    fn add_defs(
        &mut self,
        base_id: &'a str,
        count: u32,
        typ: ast::Type,
    ) -> impl Iterator<Item = spirv::Word> {
        let numeric_id = self.current_id;
        for i in 0..count {
            self.variables
                .insert(Cow::Owned(format!("{}{}", base_id, i)), numeric_id + i);
            self.type_check.insert(numeric_id + i, typ);
        }
        self.current_id += count;
        (0..count).into_iter().map(move |i| i + numeric_id)
    }
}

struct NumericIdResolver {
    current_id: spirv::Word,
    type_check: HashMap<u32, ast::Type>,
}

impl NumericIdResolver {
    fn get_type(&self, id: spirv::Word) -> ast::Type {
        self.type_check[&id]
    }

    fn new_id(&mut self, typ: Option<ast::Type>) -> spirv::Word {
        let new_id = self.current_id;
        if let Some(typ) = typ {
            self.type_check.insert(new_id, typ);
        }
        self.current_id += 1;
        new_id
    }

    fn ids_count(&self) -> spirv::Word {
        self.current_id
    }
}

enum Statement<I> {
    Variable(spirv::Word, ast::Type, ast::StateSpace),
    LoadVar(ast::Arg2<ExpandedArgParams>, ast::Type),
    StoreVar(ast::Arg2St<ExpandedArgParams>, ast::Type),
    Label(u32),
    Instruction(I),
    // SPIR-V compatible replacement for PTX predicates
    Conditional(BrachCondition),
    Conversion(ImplicitConversion),
    Constant(ConstantDefinition),
}

impl Statement<ast::Instruction<ExpandedArgParams>> {
    fn visit_variable<F: FnMut(spirv::Word) -> spirv::Word>(self, f: &mut F) -> Self {
        match self {
            Statement::Variable(id, t, ss) => Statement::Variable(f(id), t, ss),
            Statement::LoadVar(a, t) => {
                Statement::LoadVar(a.map(&mut reduced_visitor(f), Some(t)), t)
            }
            Statement::StoreVar(a, t) => {
                Statement::StoreVar(a.map(&mut reduced_visitor(f), Some(t)), t)
            }
            Statement::Label(id) => Statement::Label(f(id)),
            Statement::Instruction(inst) => Statement::Instruction(inst.visit_variable(f)),
            Statement::Conditional(bra) => Statement::Conditional(bra.map(f)),
            Statement::Conversion(conv) => Statement::Conversion(conv.map(f)),
            Statement::Constant(cons) => Statement::Constant(cons.map(f)),
        }
    }
}

enum NormalizedArgParams {}
type NormalizedStatement = Statement<ast::Instruction<NormalizedArgParams>>;

impl ast::ArgParams for NormalizedArgParams {
    type ID = spirv::Word;
    type Operand = ast::Operand<spirv::Word>;
    type MovOperand = ast::MovOperand<spirv::Word>;
}

enum ExpandedArgParams {}
type ExpandedStatement = Statement<ast::Instruction<ExpandedArgParams>>;

impl ast::ArgParams for ExpandedArgParams {
    type ID = spirv::Word;
    type Operand = spirv::Word;
    type MovOperand = spirv::Word;
}

trait ArgumentMapVisitor<T: ast::ArgParams, U: ast::ArgParams> {
    fn dst_variable(&mut self, v: T::ID, typ: Option<ast::Type>) -> U::ID;
    fn src_operand(&mut self, o: T::Operand, typ: Option<ast::Type>) -> U::Operand;
    fn src_mov_operand(&mut self, o: T::MovOperand, typ: Option<ast::Type>) -> U::MovOperand;
}

impl<T> ArgumentMapVisitor<ExpandedArgParams, ExpandedArgParams> for T
where
    T: FnMut(spirv::Word, bool, Option<ast::Type>) -> spirv::Word,
{
    fn dst_variable(&mut self, x: spirv::Word, t: Option<ast::Type>) -> spirv::Word {
        self(x, t.is_some(), t)
    }
    fn src_operand(&mut self, x: spirv::Word, t: Option<ast::Type>) -> spirv::Word {
        self(x, false, t)
    }
    fn src_mov_operand(&mut self, x: spirv::Word, t: Option<ast::Type>) -> spirv::Word {
        self(x, false, t)
    }
}

impl<'a, T> ArgumentMapVisitor<ast::ParsedArgParams<'a>, NormalizedArgParams> for T
where
    T: FnMut(&str) -> spirv::Word,
{
    fn dst_variable(&mut self, x: &str, _: Option<ast::Type>) -> spirv::Word {
        self(x)
    }

    fn src_operand(
        &mut self,
        x: ast::Operand<&str>,
        _: Option<ast::Type>,
    ) -> ast::Operand<spirv::Word> {
        match x {
            ast::Operand::Reg(id) => ast::Operand::Reg(self(id)),
            ast::Operand::Imm(imm) => ast::Operand::Imm(imm),
            ast::Operand::RegOffset(id, imm) => ast::Operand::RegOffset(self(id), imm),
        }
    }

    fn src_mov_operand(
        &mut self,
        x: ast::MovOperand<&str>,
        t: Option<ast::Type>,
    ) -> ast::MovOperand<spirv::Word> {
        match x {
            ast::MovOperand::Op(op) => ast::MovOperand::Op(self.src_operand(op, t)),
            ast::MovOperand::Vec(x1, x2) => ast::MovOperand::Vec(x1, x2),
        }
    }
}

impl<T: ast::ArgParams> ast::Instruction<T> {
    fn map<U: ast::ArgParams, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
    ) -> ast::Instruction<U> {
        match self {
            ast::Instruction::Ld(d, a) => {
                let inst_type = d.typ;
                ast::Instruction::Ld(d, a.map(visitor, Some(ast::Type::Scalar(inst_type))))
            }
            ast::Instruction::Mov(d, a) => {
                let inst_type = d.typ;
                ast::Instruction::Mov(d, a.map(visitor, Some(inst_type)))
            }
            ast::Instruction::Mul(d, a) => {
                let inst_type = d.get_type();
                ast::Instruction::Mul(d, a.map(visitor, Some(inst_type)))
            }
            ast::Instruction::Add(d, a) => {
                let inst_type = d.get_type();
                ast::Instruction::Add(d, a.map(visitor, Some(inst_type)))
            }
            ast::Instruction::Setp(d, a) => {
                let inst_type = d.typ;
                ast::Instruction::Setp(d, a.map(visitor, Some(ast::Type::Scalar(inst_type))))
            }
            ast::Instruction::SetpBool(d, a) => {
                let inst_type = d.typ;
                ast::Instruction::SetpBool(d, a.map(visitor, Some(ast::Type::Scalar(inst_type))))
            }
            ast::Instruction::Not(_, _) => todo!(),
            ast::Instruction::Cvt(_, _) => todo!(),
            ast::Instruction::Shl(_, _) => todo!(),
            ast::Instruction::St(d, a) => {
                let inst_type = d.typ;
                ast::Instruction::St(d, a.map(visitor, Some(ast::Type::Scalar(inst_type))))
            }
            ast::Instruction::Bra(d, a) => ast::Instruction::Bra(d, a.map(visitor, None)),
            ast::Instruction::Ret(d) => ast::Instruction::Ret(d),
        }
    }
}

impl ast::Instruction<NormalizedArgParams> {
    fn visit_variable<F: FnMut(spirv::Word, bool, Option<ast::Type>) -> spirv::Word>(
        self,
        f: &mut F,
    ) -> ast::Instruction<NormalizedArgParams> {
        self.map(f)
    }
}

impl<T> ArgumentMapVisitor<NormalizedArgParams, NormalizedArgParams> for T
where
    T: FnMut(spirv::Word, bool, Option<ast::Type>) -> spirv::Word,
{
    fn dst_variable(&mut self, x: spirv::Word, t: Option<ast::Type>) -> spirv::Word {
        self(x, t.is_some(), t)
    }

    fn src_operand(
        &mut self,
        x: ast::Operand<spirv::Word>,
        t: Option<ast::Type>,
    ) -> ast::Operand<spirv::Word> {
        match x {
            ast::Operand::Reg(id) => ast::Operand::Reg(self(id, false, t)),
            ast::Operand::Imm(imm) => ast::Operand::Imm(imm),
            ast::Operand::RegOffset(id, imm) => ast::Operand::RegOffset(self(id, false, t), imm),
        }
    }

    fn src_mov_operand(
        &mut self,
        x: ast::MovOperand<spirv::Word>,
        t: Option<ast::Type>,
    ) -> ast::MovOperand<spirv::Word> {
        match x {
            ast::MovOperand::Op(op) => ast::MovOperand::Op(ArgumentMapVisitor::<
                NormalizedArgParams,
                NormalizedArgParams,
            >::src_operand(self, op, t)),
            ast::MovOperand::Vec(x1, x2) => ast::MovOperand::Vec(x1, x2),
        }
    }
}

fn reduced_visitor<'a>(
    f: &'a mut impl FnMut(spirv::Word) -> spirv::Word,
) -> impl FnMut(spirv::Word, bool, Option<ast::Type>) -> spirv::Word + 'a {
    move |id, _, _| f(id)
}

impl ast::Instruction<ExpandedArgParams> {
    fn visit_variable<F: FnMut(spirv::Word) -> spirv::Word>(self, f: &mut F) -> Self {
        let mut visitor = reduced_visitor(f);
        self.map(&mut visitor)
    }

    fn visit_variable_extended<F: FnMut(spirv::Word, bool, Option<ast::Type>) -> spirv::Word>(
        self,
        f: &mut F,
    ) -> Self {
        self.map(f)
    }

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
            | ast::Instruction::Shl(_, _)
            | ast::Instruction::St(_, _)
            | ast::Instruction::Ret(_) => None,
        }
    }
}

type Arg2 = ast::Arg2<ExpandedArgParams>;
type Arg2St = ast::Arg2St<ExpandedArgParams>;

struct ConstantDefinition {
    pub dst: spirv::Word,
    pub typ: ast::ScalarType,
    pub value: i128,
}

impl ConstantDefinition {
    fn map<F: FnMut(spirv::Word) -> spirv::Word>(self, f: &mut F) -> Self {
        Self {
            dst: f(self.dst),
            typ: self.typ,
            value: self.value,
        }
    }
}

struct BrachCondition {
    predicate: spirv::Word,
    if_true: spirv::Word,
    if_false: spirv::Word,
}

impl BrachCondition {
    fn map<F: FnMut(spirv::Word) -> spirv::Word>(self, f: &mut F) -> Self {
        Self {
            predicate: f(self.predicate),
            if_true: f(self.if_true),
            if_false: f(self.if_false),
        }
    }
}

struct ImplicitConversion {
    src: spirv::Word,
    dst: spirv::Word,
    from: ast::Type,
    to: ast::Type,
    kind: ConversionKind,
}

#[derive(Debug, PartialEq)]
enum ConversionKind {
    Default,
    // zero-extend/chop/bitcast depending on types
    SignExtend,
    Ptr,
}

impl ImplicitConversion {
    fn map<F: FnMut(spirv::Word) -> spirv::Word>(self, f: &mut F) -> Self {
        Self {
            src: f(self.src),
            dst: f(self.dst),
            from: self.from,
            to: self.to,
            kind: self.kind,
        }
    }
}

impl<T> ast::PredAt<T> {
    fn map_variable<U, F: FnMut(T) -> U>(self, f: &mut F) -> ast::PredAt<U> {
        ast::PredAt {
            not: self.not,
            label: f(self.label),
        }
    }
}

// REMOVE
impl<'a> ast::Instruction<ast::ParsedArgParams<'a>> {
    fn map_variable<F: FnMut(&str) -> spirv::Word>(
        self,
        f: &mut F,
    ) -> ast::Instruction<NormalizedArgParams> {
        self.map(f)
    }
}

impl<T: ast::ArgParams> ast::Arg1<T> {
    fn map<U: ast::ArgParams, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: Option<ast::Type>,
    ) -> ast::Arg1<U> {
        ast::Arg1 {
            src: visitor.dst_variable(self.src, t),
        }
    }
}

impl<T: ast::ArgParams> ast::Arg2<T> {
    fn map<U: ast::ArgParams, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: Option<ast::Type>,
    ) -> ast::Arg2<U> {
        ast::Arg2 {
            dst: visitor.dst_variable(self.dst, t),
            src: visitor.src_operand(self.src, t),
        }
    }
}

impl<T: ast::ArgParams> ast::Arg2St<T> {
    fn map<U: ast::ArgParams, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: Option<ast::Type>,
    ) -> ast::Arg2St<U> {
        ast::Arg2St {
            src1: visitor.src_operand(self.src1, t),
            src2: visitor.src_operand(self.src2, t),
        }
    }
}

impl<T: ast::ArgParams> ast::Arg2Mov<T> {
    fn map<U: ast::ArgParams, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: Option<ast::Type>,
    ) -> ast::Arg2Mov<U> {
        ast::Arg2Mov {
            dst: visitor.dst_variable(self.dst, t),
            src: visitor.src_mov_operand(self.src, t),
        }
    }
}

impl<T: ast::ArgParams> ast::Arg3<T> {
    fn map<U: ast::ArgParams, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: Option<ast::Type>,
    ) -> ast::Arg3<U> {
        ast::Arg3 {
            dst: visitor.dst_variable(self.dst, t),
            src1: visitor.src_operand(self.src1, t),
            src2: visitor.src_operand(self.src2, t),
        }
    }
}

impl<T: ast::ArgParams> ast::Arg4<T> {
    fn map<U: ast::ArgParams, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: Option<ast::Type>,
    ) -> ast::Arg4<U> {
        ast::Arg4 {
            dst1: visitor.dst_variable(
                self.dst1,
                Some(ast::Type::ExtendedScalar(ast::ExtendedScalarType::Pred)),
            ),
            dst2: self.dst2.map(|dst2| {
                visitor.dst_variable(
                    dst2,
                    Some(ast::Type::ExtendedScalar(ast::ExtendedScalarType::Pred)),
                )
            }),
            src1: visitor.src_operand(self.src1, t),
            src2: visitor.src_operand(self.src2, t),
        }
    }
}

impl<T: ast::ArgParams> ast::Arg5<T> {
    fn map<U: ast::ArgParams, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: Option<ast::Type>,
    ) -> ast::Arg5<U> {
        ast::Arg5 {
            dst1: visitor.dst_variable(
                self.dst1,
                Some(ast::Type::ExtendedScalar(ast::ExtendedScalarType::Pred)),
            ),
            dst2: self.dst2.map(|dst2| {
                visitor.dst_variable(
                    dst2,
                    Some(ast::Type::ExtendedScalar(ast::ExtendedScalarType::Pred)),
                )
            }),
            src1: visitor.src_operand(self.src1, t),
            src2: visitor.src_operand(self.src2, t),
            src3: visitor.src_operand(
                self.src3,
                Some(ast::Type::ExtendedScalar(ast::ExtendedScalarType::Pred)),
            ),
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

#[derive(Clone, Copy, PartialEq)]
enum ScalarKind {
    Byte,
    Unsigned,
    Signed,
    Float,
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
            ast::ScalarType::B8 => ScalarKind::Byte,
            ast::ScalarType::B16 => ScalarKind::Byte,
            ast::ScalarType::B32 => ScalarKind::Byte,
            ast::ScalarType::B64 => ScalarKind::Byte,
            ast::ScalarType::F16 => ScalarKind::Float,
            ast::ScalarType::F32 => ScalarKind::Float,
            ast::ScalarType::F64 => ScalarKind::Float,
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
            ScalarKind::Byte => match width {
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
        }
    }
}

impl ast::AddDetails {
    fn get_type(&self) -> ast::Type {
        match self {
            ast::AddDetails::Int(ast::AddIntDesc { typ, .. }) => ast::Type::Scalar((*typ).into()),
            ast::AddDetails::Float(ast::AddFloatDesc { typ, .. }) => (*typ).into(),
        }
    }
}

impl ast::MulDetails {
    fn get_type(&self) -> ast::Type {
        match self {
            ast::MulDetails::Int(ast::MulIntDesc { typ, .. }) => ast::Type::Scalar((*typ).into()),
            ast::MulDetails::Float(ast::MulFloatDesc { typ, .. }) => (*typ).into(),
        }
    }
}

impl ast::IntType {
    fn is_signed(self) -> bool {
        match self {
            ast::IntType::S16 | ast::IntType::S32 | ast::IntType::S64 => true,
            ast::IntType::U16 | ast::IntType::U32 | ast::IntType::U64 => false,
        }
    }

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

fn should_bitcast(instr: ast::Type, operand: ast::Type) -> bool {
    match (instr, operand) {
        (ast::Type::Scalar(inst), ast::Type::Scalar(operand)) => {
            if inst.width() != operand.width() {
                return false;
            }
            match inst.kind() {
                ScalarKind::Byte => operand.kind() != ScalarKind::Byte,
                ScalarKind::Float => operand.kind() == ScalarKind::Byte,
                ScalarKind::Signed => {
                    operand.kind() == ScalarKind::Byte || operand.kind() == ScalarKind::Unsigned
                }
                ScalarKind::Unsigned => {
                    operand.kind() == ScalarKind::Byte || operand.kind() == ScalarKind::Signed
                }
            }
        }
        _ => false,
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
        ast::LdStateSpace::Generic => {
            let new_src_type = ast::Type::Scalar(ast::ScalarType::from_parts(
                mem::size_of::<usize>() as u8,
                ScalarKind::Byte,
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
                ConversionKind::Ptr,
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
    if let Some(conv) = should_convert(src_type, instr_type) {
        insert_conversion_src(func, id_def, src, src_type, instr_type, conv)
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

fn insert_with_implicit_conversion_dst<
    T,
    ShouldConvert: FnOnce(ast::Type, ast::ScalarType) -> Option<ConversionKind>,
    Setter: Fn(&mut T) -> &mut spirv::Word,
    ToInstruction: FnOnce(T) -> ast::Instruction<ExpandedArgParams>,
>(
    func: &mut Vec<ExpandedStatement>,
    instr_type: ast::ScalarType,
    id_def: &mut NumericIdResolver,
    should_convert: ShouldConvert,
    mut t: T,
    setter: Setter,
    to_inst: ToInstruction,
) {
    let dst = setter(&mut t);
    let dst_type = id_def.get_type(*dst);
    let dst_coercion = should_convert(dst_type, instr_type)
        .map(|conv| get_conversion_dst(id_def, dst, ast::Type::Scalar(instr_type), dst_type, conv));
    func.push(Statement::Instruction(to_inst(t)));
    if let Some(conv) = dst_coercion {
        func.push(conv);
    }
}

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
    instr_type: ast::ScalarType,
) -> Option<ConversionKind> {
    if src_type == ast::Type::Scalar(instr_type) {
        return None;
    }
    match src_type {
        ast::Type::Scalar(src_type) => match instr_type.kind() {
            ScalarKind::Byte => {
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
                if instr_type.width() <= src_type.width() && src_type.kind() == ScalarKind::Byte {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
        },
        _ => None,
    }
}

// https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#operand-size-exceeding-instruction-type-size__relaxed-type-checking-rules-destination-operands
fn should_convert_relaxed_dst(
    dst_type: ast::Type,
    instr_type: ast::ScalarType,
) -> Option<ConversionKind> {
    if dst_type == ast::Type::Scalar(instr_type) {
        return None;
    }
    match dst_type {
        ast::Type::Scalar(dst_type) => match instr_type.kind() {
            ScalarKind::Byte => {
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
                if instr_type.width() <= dst_type.width() && dst_type.kind() == ScalarKind::Byte {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
        },
        _ => None,
    }
}

fn insert_implicit_bitcasts(
    func: &mut Vec<ExpandedStatement>,
    id_def: &mut NumericIdResolver,
    instr: ast::Instruction<ExpandedArgParams>,
) {
    let mut dst_coercion = None;
    let instr = instr.visit_variable_extended(&mut |mut id, is_dst, id_type| {
        let id_type_from_instr = match id_type {
            Some(t) => t,
            None => return id,
        };
        let id_actual_type = id_def.get_type(id);
        if should_bitcast(id_type_from_instr, id_def.get_type(id)) {
            if is_dst {
                dst_coercion = Some(get_conversion_dst(
                    id_def,
                    &mut id,
                    id_type_from_instr,
                    id_actual_type,
                    ConversionKind::Default,
                ));
                id
            } else {
                insert_conversion_src(
                    func,
                    id_def,
                    id,
                    id_actual_type,
                    id_type_from_instr,
                    ConversionKind::Default,
                )
            }
        } else {
            id
        }
    });
    func.push(Statement::Instruction(instr));
    if let Some(cond) = dst_coercion {
        func.push(cond);
    }
}

// CFGs below taken from "Modern Compiler Implementation in Java"
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

    fn assert_conversion_table<F: Fn(ast::Type, ast::ScalarType) -> Option<ConversionKind>>(
        table: &'static str,
        f: F,
    ) {
        let conv_table = parse_conversion_table(table);
        for (instr_idx, instr_type) in SCALAR_TYPES.iter().enumerate() {
            for (op_idx, op_type) in SCALAR_TYPES.iter().enumerate() {
                let conversion = f(ast::Type::Scalar(*op_type), *instr_type);
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
