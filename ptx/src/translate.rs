use crate::ast;
use bit_vec::BitVec;
use rspirv::dr;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::{borrow::Cow, fmt, mem};

use rspirv::binary::Assemble;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum SpirvType {
    Base(ast::ScalarType),
    Extended(ast::ExtendedScalarType),
    Pointer(ast::ScalarType, spirv::StorageClass),
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
            SpirvType::Extended(t) => self.get_or_add_extended(b, t),
            SpirvType::Pointer(scalar, storage) => {
                let base = self.get_or_add_scalar(b, scalar);
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
    emit_extended_instruction_sets(&mut builder);
    emit_memory_model(&mut builder);
    let mut map = TypeWordMap::new(&mut builder);
    for f in ast.functions {
        emit_function(&mut builder, &mut map, f)?;
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

fn emit_extended_instruction_sets(builder: &mut dr::Builder) {
    builder.ext_inst_import("OpenCL.std");
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
    f: ast::Function<'a>,
) -> Result<spirv::Word, rspirv::dr::Error> {
    let func_type = get_function_type(builder, map, &f.args);
    let func_id =
        builder.begin_function(map.void(), None, spirv::FunctionControl::NONE, func_type)?;
    if f.kernel {
        builder.entry_point(spirv::ExecutionModel::Kernel, func_id, f.name, &[]);
    }
    let (mut func_body, bbs, _, unique_ids) = to_ssa(&f.args, f.body);
    let id_offset = builder.reserve_ids(unique_ids);
    emit_function_args(builder, id_offset, map, &f.args);
    apply_id_offset(&mut func_body, id_offset);
    emit_function_body_ops(builder, map, &func_body, &bbs)?;
    builder.end_function()?;
    Ok(func_id)
}

fn apply_id_offset(func_body: &mut Vec<Statement>, id_offset: u32) {
    for s in func_body {
        s.visit_id_mut(&mut |_, id| *id += id_offset);
    }
}

fn to_ssa<'a>(
    f_args: &[ast::Argument],
    f_body: Vec<ast::Statement<&'a str>>,
) -> (
    Vec<Statement>,
    Vec<BasicBlock>,
    Vec<Vec<PhiDef>>,
    spirv::Word,
) {
    let mut contant_ids = HashMap::new();
    let mut type_check = HashMap::new();
    collect_arg_ids(&mut contant_ids, &mut type_check, &f_args);
    collect_label_ids(&mut contant_ids, &f_body);
    let registers = collect_var_definitions(&f_args, &f_body);
    let (normalized_ids, mut unique_ids) =
        normalize_identifiers(f_body, &contant_ids, &mut type_check, registers);
    let type_check = RefCell::new(type_check);
    let new_id = &mut |typ: Option<ast::Type>| {
        let to_insert = unique_ids;
        {
            let mut type_check = type_check.borrow_mut();
            typ.map(|t| (*type_check).insert(to_insert, t));
        }
        unique_ids += 1;
        to_insert
    };
    let normalized_stmts = normalize_statements(normalized_ids, new_id);
    let mut func_body = insert_implicit_conversions(normalized_stmts, new_id, &|x| {
        let type_check = type_check.borrow();
        type_check[&x]
    });
    let bbs = get_basic_blocks(&func_body);
    let rpostorder = to_reverse_postorder(&bbs);
    let doms = immediate_dominators(&bbs, &rpostorder);
    let dom_fronts = dominance_frontiers(&bbs, &doms);
    let (phis, unique_ids) = ssa_legalize(
        &mut func_body,
        contant_ids.len() as u32,
        unique_ids,
        &bbs,
        &doms,
        &dom_fronts,
    );
    (func_body, bbs, phis, unique_ids)
}

fn normalize_statements(
    func: Vec<ast::Statement<spirv::Word>>,
    new_id: &mut impl FnMut(Option<ast::Type>) -> spirv::Word,
) -> Vec<Statement> {
    let mut result = Vec::with_capacity(func.len());
    for s in func {
        match s {
            ast::Statement::Label(id) => result.push(Statement::Label(id)),
            ast::Statement::Instruction(pred, inst) => {
                if let Some(pred) = pred {
                    let mut if_true = new_id(None);
                    let mut if_false = new_id(None);
                    if pred.not {
                        std::mem::swap(&mut if_true, &mut if_false);
                    }
                    let folded_bra = match &inst {
                        ast::Instruction::Bra(_, arg) => Some(arg.src),
                        _ => None,
                    };
                    let branch = BrachCondition {
                        predicate: pred.label,
                        if_true: folded_bra.unwrap_or(if_true),
                        if_false,
                    };
                    result.push(Statement::Conditional(branch));
                    if folded_bra.is_none() {
                        result.push(Statement::Label(if_true));
                        let instr = normalize_insert_instruction(&mut result, new_id, inst);
                        result.push(Statement::Instruction(instr));
                    }
                    result.push(Statement::Label(if_false));
                } else {
                    let instr = normalize_insert_instruction(&mut result, new_id, inst);
                    result.push(Statement::Instruction(instr));
                }
            }
            ast::Statement::Variable(_) => unreachable!(),
        }
    }
    result
}

#[must_use]
fn normalize_insert_instruction(
    func: &mut Vec<Statement>,
    new_id: &mut impl FnMut(Option<ast::Type>) -> spirv::Word,
    instr: ast::Instruction<spirv::Word>,
) -> Instruction {
    match instr {
        ast::Instruction::Ld(d, a) => {
            let arg = normalize_expand_arg2(func, new_id, &|| Some(d.typ), a);
            Instruction::Ld(d, arg)
        }
        ast::Instruction::Mov(d, a) => {
            let arg = normalize_expand_arg2mov(func, new_id, &|| d.typ.try_as_scalar(), a);
            Instruction::Mov(d, arg)
        }
        ast::Instruction::Mul(d, a) => {
            let arg = normalize_expand_arg3(func, new_id, &|| d.typ.try_as_scalar(), a);
            Instruction::Mul(d, arg)
        }
        ast::Instruction::Add(d, a) => {
            let arg = normalize_expand_arg3(func, new_id, &|| Some(d.typ), a);
            Instruction::Add(d, arg)
        }
        ast::Instruction::Setp(d, a) => {
            let arg = normalize_expand_arg4(func, new_id, &|| Some(d.typ), a);
            Instruction::Setp(d, arg)
        }
        ast::Instruction::SetpBool(d, a) => {
            let arg = normalize_expand_arg5(func, new_id, &|| Some(d.typ), a);
            Instruction::SetpBool(d, arg)
        }
        ast::Instruction::Not(d, a) => {
            let arg = normalize_expand_arg2(func, new_id, &|| todo!(), a);
            Instruction::Not(d, arg)
        }
        ast::Instruction::Bra(d, a) => Instruction::Bra(d, Arg1 { src: a.src }),
        ast::Instruction::Cvt(d, a) => {
            let arg = normalize_expand_arg2(func, new_id, &|| todo!(), a);
            Instruction::Cvt(d, arg)
        }
        ast::Instruction::Shl(d, a) => {
            let arg = normalize_expand_arg3(func, new_id, &|| todo!(), a);
            Instruction::Shl(d, arg)
        }
        ast::Instruction::St(d, a) => {
            let arg = normalize_expand_arg2st(func, new_id, &|| todo!(), a);
            Instruction::St(d, arg)
        }
        ast::Instruction::Ret(d) => Instruction::Ret(d),
    }
}

fn normalize_expand_arg2(
    func: &mut Vec<Statement>,
    new_id: &mut impl FnMut(Option<ast::Type>) -> spirv::Word,
    inst_type: &impl Fn() -> Option<ast::ScalarType>,
    a: ast::Arg2<spirv::Word>,
) -> Arg2 {
    Arg2 {
        dst: a.dst,
        src: normalize_expand_operand(func, new_id, inst_type, a.src),
    }
}

fn normalize_expand_arg2mov(
    func: &mut Vec<Statement>,
    new_id: &mut impl FnMut(Option<ast::Type>) -> spirv::Word,
    inst_type: &impl Fn() -> Option<ast::ScalarType>,
    a: ast::Arg2Mov<spirv::Word>,
) -> Arg2 {
    Arg2 {
        dst: a.dst,
        src: normalize_expand_mov_operand(func, new_id, inst_type, a.src),
    }
}

fn normalize_expand_arg2st(
    func: &mut Vec<Statement>,
    new_id: &mut impl FnMut(Option<ast::Type>) -> spirv::Word,
    inst_type: &impl Fn() -> Option<ast::ScalarType>,
    a: ast::Arg2St<spirv::Word>,
) -> Arg2St {
    Arg2St {
        src1: normalize_expand_operand(func, new_id, inst_type, a.src1),
        src2: normalize_expand_operand(func, new_id, inst_type, a.src2),
    }
}

fn normalize_expand_arg3(
    func: &mut Vec<Statement>,
    new_id: &mut impl FnMut(Option<ast::Type>) -> spirv::Word,
    inst_type: &impl Fn() -> Option<ast::ScalarType>,
    a: ast::Arg3<spirv::Word>,
) -> Arg3 {
    Arg3 {
        dst: a.dst,
        src1: normalize_expand_operand(func, new_id, inst_type, a.src1),
        src2: normalize_expand_operand(func, new_id, inst_type, a.src2),
    }
}

fn normalize_expand_arg4(
    func: &mut Vec<Statement>,
    new_id: &mut impl FnMut(Option<ast::Type>) -> spirv::Word,
    inst_type: &impl Fn() -> Option<ast::ScalarType>,
    a: ast::Arg4<spirv::Word>,
) -> Arg4 {
    Arg4 {
        dst1: a.dst1,
        dst2: a.dst2,
        src1: normalize_expand_operand(func, new_id, inst_type, a.src1),
        src2: normalize_expand_operand(func, new_id, inst_type, a.src2),
    }
}

fn normalize_expand_arg5(
    func: &mut Vec<Statement>,
    new_id: &mut impl FnMut(Option<ast::Type>) -> spirv::Word,
    inst_type: &impl Fn() -> Option<ast::ScalarType>,
    a: ast::Arg5<spirv::Word>,
) -> Arg5 {
    Arg5 {
        dst1: a.dst1,
        dst2: a.dst2,
        src1: normalize_expand_operand(func, new_id, inst_type, a.src1),
        src2: normalize_expand_operand(func, new_id, inst_type, a.src2),
        src3: normalize_expand_operand(func, new_id, inst_type, a.src3),
    }
}

fn normalize_expand_operand(
    func: &mut Vec<Statement>,
    new_id: &mut impl FnMut(Option<ast::Type>) -> spirv::Word,
    inst_type: &impl Fn() -> Option<ast::ScalarType>,
    opr: ast::Operand<spirv::Word>,
) -> spirv::Word {
    match opr {
        ast::Operand::Reg(r) => r,
        ast::Operand::Imm(x) => {
            if let Some(typ) = inst_type() {
                let id = new_id(Some(ast::Type::Scalar(typ)));
                func.push(Statement::Constant(ConstantDefinition {
                    dst: id,
                    typ: typ,
                    value: x,
                }));
                id
            } else {
                todo!()
            }
        }
        _ => todo!(),
    }
}

fn normalize_expand_mov_operand(
    func: &mut Vec<Statement>,
    new_id: &mut impl FnMut(Option<ast::Type>) -> spirv::Word,
    inst_type: &impl Fn() -> Option<ast::ScalarType>,
    opr: ast::MovOperand<spirv::Word>,
) -> spirv::Word {
    match opr {
        ast::MovOperand::Op(opr) => normalize_expand_operand(func, new_id, inst_type, opr),
        _ => todo!(),
    }
}

fn collect_var_definitions<'a>(
    args: &[ast::Argument<'a>],
    body: &[ast::Statement<&'a str>],
) -> HashMap<Cow<'a, str>, ast::Type> {
    let mut result = HashMap::new();
    for param in args {
        result.insert(Cow::Borrowed(param.name), ast::Type::Scalar(param.a_type));
    }
    for s in body {
        match s {
            ast::Statement::Variable(var) => match var.count {
                Some(count) => {
                    for i in 0..count {
                        result.insert(Cow::Owned(format!("{}{}", var.name, i)), var.v_type);
                    }
                }
                None => {
                    result.insert(Cow::Borrowed(var.name), var.v_type);
                }
            },
            ast::Statement::Label(_) | ast::Statement::Instruction(_, _) => (),
        }
    }
    result
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
fn insert_implicit_conversions<TypeCheck: Fn(spirv::Word) -> ast::Type>(
    normalized_ids: Vec<Statement>,
    new_id: &mut impl FnMut(Option<ast::Type>) -> spirv::Word,
    type_check: &TypeCheck,
) -> Vec<Statement> {
    let mut result = Vec::with_capacity(normalized_ids.len());
    for s in normalized_ids.into_iter() {
        match s {
            Statement::Instruction(inst) => match inst {
                Instruction::Ld(ld, mut arg) => {
                    arg.src = insert_implicit_conversions_ld_src(
                        &mut result,
                        ast::Type::Scalar(ld.typ),
                        type_check,
                        new_id,
                        ld.state_space,
                        arg.src,
                    );
                    insert_with_implicit_conversion_dst(
                        &mut result,
                        ld.typ,
                        type_check,
                        new_id,
                        should_convert_relaxed_dst,
                        arg,
                        |arg| &mut arg.dst,
                        |arg| Instruction::Ld(ld, arg),
                    );
                }
                Instruction::St(st, mut arg) => {
                    let arg_src2_type = type_check(arg.src2);
                    if let Some(conv) = should_convert_relaxed_src(arg_src2_type, st.typ) {
                        arg.src2 = insert_conversion_src(
                            &mut result,
                            new_id,
                            arg.src2,
                            arg_src2_type,
                            ast::Type::Scalar(st.typ),
                            conv,
                        );
                    }
                    arg.src1 = insert_implicit_conversions_ld_src(
                        &mut result,
                        ast::Type::Scalar(st.typ),
                        type_check,
                        new_id,
                        st.state_space.to_ld_ss(),
                        arg.src1,
                    );
                    result.push(Statement::Instruction(Instruction::St(st, arg)));
                }
                inst @ _ => insert_implicit_bitcasts(&mut result, type_check, new_id, inst),
            },
            s @ Statement::Conditional(_) | s @ Statement::Label(_) => result.push(s),
            Statement::Constant(_) => (),
            Statement::Converison(_) => unreachable!(),
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

fn collect_arg_ids<'a>(
    result: &mut HashMap<&'a str, spirv::Word>,
    type_check: &mut HashMap<spirv::Word, ast::Type>,
    args: &'a [ast::Argument<'a>],
) {
    let mut id = result.len() as u32;
    for arg in args {
        result.insert(arg.name, id);
        type_check.insert(id, ast::Type::Scalar(arg.a_type));
        id += 1;
    }
}

fn collect_label_ids<'a>(
    result: &mut HashMap<&'a str, spirv::Word>,
    fn_body: &[ast::Statement<&'a str>],
) {
    let mut id = result.len() as u32;
    for s in fn_body {
        match s {
            ast::Statement::Label(name) => {
                result.insert(name, id);
                id += 1;
            }
            ast::Statement::Instruction(_, _) => (),
            ast::Statement::Variable(_) => (),
        }
    }
}

fn emit_function_body_ops(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    func: &[Statement],
    cfg: &[BasicBlock],
) -> Result<(), dr::Error> {
    // TODO: entry basic block can't be target of jumps,
    // we need to emit additional BB for this purpose
    for bb_idx in 0..cfg.len() {
        let body = get_bb_body(func, cfg, BBIndex(bb_idx));
        if body.len() == 0 {
            continue;
        }
        let header_id = if let Statement::Label(id) = body[0] {
            Some(id)
        } else {
            None
        };
        builder.begin_block(header_id)?;
        for s in body {
            match s {
                // If block starts with a label it has already been emitted,
                // all other labels in the block are unused
                Statement::Label(_) => (),
                Statement::Constant(_) => todo!(),
                Statement::Converison(cv) => emit_implicit_conversion(builder, map, cv)?,
                Statement::Conditional(bra) => {
                    builder.branch_conditional(bra.predicate, bra.if_true, bra.if_false, [])?;
                }
                Statement::Instruction(inst) => match inst {
                    // SPIR-V does not support marking jumps as guaranteed-converged
                    Instruction::Bra(_, arg) => {
                        builder.branch(arg.src)?;
                    }
                    Instruction::Ld(data, arg) => {
                        if data.qualifier != ast::LdStQualifier::Weak || data.vector.is_some() {
                            todo!()
                        }
                        let result_type = map.get_or_add_scalar(builder, data.typ);
                        match data.state_space {
                            ast::LdStateSpace::Generic => {
                                builder.load(result_type, Some(arg.dst), arg.src, None, [])?;
                            }
                            ast::LdStateSpace::Param => {
                                builder.copy_object(result_type, Some(arg.dst), arg.src)?;
                            }
                            _ => todo!(),
                        }
                    }
                    Instruction::St(data, arg) => {
                        if data.qualifier != ast::LdStQualifier::Weak
                            || data.vector.is_some()
                            || data.state_space != ast::StStateSpace::Generic
                        {
                            todo!()
                        }
                        builder.store(arg.src1, arg.src2, None, &[])?;
                    }
                    // SPIR-V does not support ret as guaranteed-converged
                    Instruction::Ret(_) => builder.ret()?,
                    Instruction::Mov(mov, arg) => {
                        let result_type = map.get_or_add(builder, SpirvType::from(mov.typ));
                        builder.copy_object(result_type, Some(arg.dst), arg.src)?;
                    }
                    Instruction::Mul(mul, arg) => match mul.desc {
                        ast::MulDescriptor::Int(ref ctr) => {
                            emit_mul_int(builder, map, mul.typ, ctr, arg)
                        }
                        ast::MulDescriptor::Float(_) => todo!(),
                    },
                    _ => todo!(),
                },
            }
        }
    }
    Ok(())
}

fn emit_mul_int(
    _builder: &mut dr::Builder,
    _map: &mut TypeWordMap,
    _typ: ast::Type,
    _ctr: &ast::MulIntControl,
    _arg: &Arg3,
) {
    //let inst_type = map.get_or_add(builder, SpirvType::from(typ));
    //builder.i_mul(inst_type, Some(arg.dst), Some(arg.src1), Some(arg.src2));
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
                SpirvType::Pointer(to_type, spirv_headers::StorageClass::Generic),
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
fn normalize_identifiers<'a>(
    func: Vec<ast::Statement<&'a str>>,
    constant_identifiers: &HashMap<&'a str, spirv::Word>, // arguments and labels can't be redefined
    type_map: &mut HashMap<spirv::Word, ast::Type>,
    types: HashMap<Cow<'a, str>, ast::Type>,
) -> (Vec<ast::Statement<spirv::Word>>, spirv::Word) {
    let mut id: u32 = constant_identifiers.len() as u32;
    let mut remapped_ids = HashMap::new();
    let mut get_or_add = |key| {
        constant_identifiers.get(key).map_or_else(
            || {
                *remapped_ids.entry(key).or_insert_with(|| {
                    let to_insert = id;
                    id += 1;
                    to_insert
                })
            },
            |id| *id,
        )
    };
    let result = func
        .into_iter()
        .filter_map(|s| Statement::from_ast(s, &mut get_or_add))
        .collect::<Vec<_>>();
    type_map.extend(
        remapped_ids
            .into_iter()
            .map(|(old_id, new_id)| (new_id, types[old_id])),
    );
    (result, id)
}

fn ssa_legalize(
    func: &mut [Statement],
    constant_ids: spirv::Word,
    unique_ids: spirv::Word,
    bbs: &[BasicBlock],
    doms: &[BBIndex],
    dom_fronts: &[HashSet<BBIndex>],
) -> (Vec<Vec<PhiDef>>, spirv::Word) {
    let phis = gather_phi_sets(&func, constant_ids, unique_ids, &bbs, dom_fronts);
    apply_ssa_renaming(func, &bbs, doms, constant_ids, unique_ids, &phis)
}

/* "Modern Compiler Implementation in Java" - Algorithm 19.7
 * This algorithm modifies passed function body in-place by renumbering ids,
 * result ids can be divided into following categories
 * - if id < constant_ids
 *      it's a non-redefinable id
 * - if id >= constant_ids && id < all_ids
 *      then it's an undefined id (a0, b0, c0)
 * - if id >= all_ids
 *      then it's a normally redefined id
 */
fn apply_ssa_renaming(
    func: &mut [Statement],
    bbs: &[BasicBlock],
    doms: &[BBIndex],
    constant_ids: spirv::Word,
    all_ids: spirv::Word,
    old_phi: &[HashSet<spirv::Word>],
) -> (Vec<Vec<PhiDef>>, spirv::Word) {
    let mut dom_tree = vec![Vec::new(); bbs.len()];
    for (bb, idom) in doms.iter().enumerate().skip(1) {
        dom_tree[idom.0].push(BBIndex(bb));
    }
    let mut old_dst_id = vec![Vec::new(); bbs.len()];
    for bb in 0..bbs.len() {
        for s in get_bb_body(func, bbs, BBIndex(bb)) {
            s.visit_id(&mut |is_dst, id| {
                if is_dst {
                    old_dst_id[bb].push(id)
                }
            });
        }
    }
    let mut new_phi = old_phi
        .iter()
        .map(|ids| {
            ids.iter()
                .map(|id| (*id, (u32::max_value(), HashSet::new())))
                .collect::<HashMap<_, _>>()
        })
        .collect::<Vec<_>>();
    let mut ssa_state = SSARewriteState::new(constant_ids, all_ids);
    // once again, we do explicit stack
    let mut state = Vec::new();
    state.push((BBIndex(0), 0));
    loop {
        if let Some((BBIndex(bb), dom_succ_idx)) = state.last_mut() {
            let bb = *bb;
            if *dom_succ_idx == 0 {
                rename_phi_dst(&mut ssa_state, &mut new_phi[bb]);
                rename_bb_body(&mut ssa_state, func, bbs, BBIndex(bb));
                for BBIndex(succ_idx) in bbs[bb].succ.iter() {
                    rename_succesor_phi_src(&ssa_state, &mut new_phi[*succ_idx]);
                }
            }
            if let Some(s) = dom_tree[bb].get(*dom_succ_idx) {
                *dom_succ_idx += 1;
                state.push((*s, 0));
            } else {
                state.pop();
                pop_stacks(&mut ssa_state, &old_phi[bb], &old_dst_id[bb]);
            }
        } else {
            break;
        }
    }
    let phi = new_phi
        .into_iter()
        .map(|map| {
            map.into_iter()
                .map(|(_, (new_id, defs))| PhiDef {
                    dst: new_id,
                    src: defs,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (phi, ssa_state.next_id())
}

// before ssa-renaming every phi is x <- phi(x,x,x,x)
#[derive(Debug, PartialEq)]
struct PhiDef {
    dst: spirv::Word,
    src: HashSet<spirv::Word>,
}

fn rename_phi_dst(
    rewriter: &mut SSARewriteState,
    phi: &mut HashMap<spirv::Word, (spirv::Word, HashSet<spirv::Word>)>,
) {
    for (old_k, (new_k, _)) in phi.iter_mut() {
        *new_k = rewriter.redefine(*old_k);
    }
}

fn rename_bb_body(
    ssa_state: &mut SSARewriteState,
    func: &mut [Statement],
    all_bb: &[BasicBlock],
    bb: BBIndex,
) {
    for s in get_bb_body_mut(func, all_bb, bb) {
        s.visit_id_mut(&mut |is_dst, id| {
            if is_dst {
                *id = ssa_state.redefine(*id);
            } else {
                *id = ssa_state.get(*id);
            }
        });
    }
}

fn rename_succesor_phi_src(
    ssa_state: &SSARewriteState,
    phi: &mut HashMap<spirv::Word, (spirv::Word, HashSet<spirv::Word>)>,
) {
    for (id, (_, v)) in phi.iter_mut() {
        v.insert(ssa_state.get(*id));
    }
}

fn pop_stacks(
    ssa_state: &mut SSARewriteState,
    old_phi: &HashSet<spirv::Word>,
    old_ids: &[spirv::Word],
) {
    for id in old_phi.iter().chain(old_ids) {
        ssa_state.pop(*id);
    }
}

fn get_bb_body_mut<'a>(
    func: &'a mut [Statement],
    all_bb: &[BasicBlock],
    bb: BBIndex,
) -> &'a mut [Statement] {
    let (start, end) = get_bb_body_idx(func, all_bb, bb);
    &mut func[start..end]
}

fn get_bb_body<'a>(func: &'a [Statement], all_bb: &[BasicBlock], bb: BBIndex) -> &'a [Statement] {
    let (start, end) = get_bb_body_idx(func, all_bb, bb);
    &func[start..end]
}

fn get_bb_body_idx(func: &[Statement], all_bb: &[BasicBlock], bb: BBIndex) -> (usize, usize) {
    let BBIndex(bb_idx) = bb;
    let start = all_bb[bb_idx].start.0;
    let end = if bb_idx == all_bb.len() - 1 {
        func.len()
    } else {
        all_bb[bb_idx + 1].start.0
    };
    (start, end)
}

// We assume here that the variables are defined in the dense sequence 0..max
struct SSARewriteState {
    next: spirv::Word,
    constant_ids: spirv::Word,
    stack: Vec<Vec<spirv::Word>>,
}

impl<'a> SSARewriteState {
    fn new(constant_ids: spirv::Word, all_ids: spirv::Word) -> Self {
        let to_redefine = all_ids - constant_ids;
        let stack = (0..to_redefine)
            .map(|x| vec![x + constant_ids])
            .collect::<Vec<_>>();
        SSARewriteState {
            next: all_ids,
            constant_ids: constant_ids,
            stack,
        }
    }

    fn get(&self, x: spirv::Word) -> spirv::Word {
        if x < self.constant_ids {
            x
        } else {
            *self.stack[(x - self.constant_ids) as usize].last().unwrap()
        }
    }

    fn redefine(&mut self, x: spirv::Word) -> spirv::Word {
        if x < self.constant_ids {
            x
        } else {
            let result = self.next;
            self.next += 1;
            self.stack[(x - self.constant_ids) as usize].push(result);
            result
        }
    }

    fn pop(&mut self, x: spirv::Word) {
        if x >= self.constant_ids {
            self.stack[(x - self.constant_ids) as usize].pop();
        }
    }

    fn next_id(&self) -> spirv::Word {
        self.next
    }
}

// "Engineering a Compiler" - Figure 9.9
// Calculates semi-pruned phis
fn gather_phi_sets(
    func: &[Statement],
    constant_ids: spirv::Word,
    all_ids: spirv::Word,
    cfg: &[BasicBlock],
    dom_fronts: &[HashSet<BBIndex>],
) -> Vec<HashSet<spirv::Word>> {
    let mut result = vec![HashSet::new(); cfg.len()];
    let mut globals = HashSet::new();
    let mut blocks = vec![(Vec::new(), HashSet::new()); (all_ids - constant_ids) as usize];
    for bb in 0..cfg.len() {
        let mut var_kill = HashSet::new();
        let mut visitor = |is_dst, id: spirv::Word| {
            if id >= constant_ids {
                let id = id - constant_ids;
                if is_dst {
                    var_kill.insert(id);
                    let (ref mut stack, ref mut set) = blocks[id as usize];
                    stack.push(BBIndex(bb));
                    set.insert(BBIndex(bb));
                } else {
                    if !var_kill.contains(&id) {
                        globals.insert(id);
                    }
                }
            }
        };
        // We try to avoid adding labels to the global-visbility set.
        // We are not 100% precise (we add jump targets in bra), but it shouldn't be a problem
        for s in get_bb_body(func, cfg, BBIndex(bb)) {
            match s {
                Statement::Instruction(inst) => inst.visit_id(&mut visitor),
                Statement::Conditional(brc) => visitor(false, brc.predicate),
                Statement::Converison(conv) => conv.visit_id(&mut visitor),
                Statement::Constant(cons) => cons.visit_id(&mut visitor),
                // label redefinition is a compile-time error
                Statement::Label(_) => (),
            }
        }
    }
    for id in globals {
        let (ref mut work_stack, ref mut work_set) = &mut blocks[id as usize];
        loop {
            if let Some(bb) = work_stack.pop() {
                work_set.remove(&bb);
                for d_bb in &dom_fronts[bb.0] {
                    if result[d_bb.0].insert(id + constant_ids) {
                        if work_set.insert(*d_bb) {
                            work_stack.push(*d_bb);
                        }
                    }
                }
            } else {
                break;
            }
        }
    }
    result
}

fn get_basic_blocks(fun: &[Statement]) -> Vec<BasicBlock> {
    // edge signify pred/succ relationship between bbs
    let mut unresolved_bb_edge = Vec::new();
    // bb start means that a bb is starting at this statement, but there's no predecessor
    let mut bb_start = Vec::new();
    let mut labels = HashMap::new();
    for (idx, s) in fun.iter().enumerate() {
        match s {
            Statement::Instruction(i) => {
                if let Some(id) = i.jump_target() {
                    unresolved_bb_edge.push((StmtIndex(idx), id));
                    if idx + 1 < fun.len() {
                        bb_start.push(StmtIndex(idx + 1));
                    }
                } else if i.is_terminal() && idx + 1 < fun.len() {
                    bb_start.push(StmtIndex(idx + 1));
                }
            }
            Statement::Label(id) => {
                labels.insert(id, StmtIndex(idx));
            }
            Statement::Conditional(bra) => {
                unresolved_bb_edge.push((StmtIndex(idx), bra.if_false));
                unresolved_bb_edge.push((StmtIndex(idx), bra.if_true));
            }
            Statement::Constant(_) => (),
            Statement::Converison(_) => (),
        };
    }
    let mut bb_edge = HashSet::new();
    // Resolve every <jump into label> into <jump into statement index>
    // TODO: handle jumps into nowhere
    for (idx, id) in unresolved_bb_edge {
        let target = labels[&id];
        bb_edge.insert((idx, target));
        bb_start.push(target);
        // now check if there is an edge target-1 -> target
        if target != StmtIndex(0) {
            match &fun[target.0 - 1] {
                Statement::Instruction(i) => {
                    if !(i.jump_target().is_some() || i.is_terminal()) {
                        bb_edge.insert((StmtIndex(target.0 - 1), target));
                    }
                }
                Statement::Converison(_) | Statement::Constant(_) | Statement::Label(_) => {
                    bb_edge.insert((StmtIndex(target.0 - 1), target));
                }
                // This is already in `unresolved_bb_edge`
                Statement::Conditional(_) => (),
            }
        }
    }
    // Create list of bbs without succ/pred
    let mut bbs_map = BTreeMap::new();
    bbs_map.insert(
        StmtIndex(0),
        BasicBlock {
            start: StmtIndex(0),
            pred: Vec::new(),
            succ: Vec::new(),
        },
    );
    for bb_first_stmt in bb_start {
        bbs_map.entry(bb_first_stmt).or_insert_with(|| BasicBlock {
            start: bb_first_stmt,
            pred: Vec::new(),
            succ: Vec::new(),
        });
    }
    // Populate succ/pred
    let indexed_bbs_map = bbs_map
        .into_iter()
        .enumerate()
        .map(|(idx, (key, val))| (key, (BBIndex(idx), RefCell::new(val))))
        .collect::<BTreeMap<_, _>>();
    for (from, to) in bb_edge {
        let (_, (from_idx, from_ref)) = indexed_bbs_map.range(..=from).next_back().unwrap();
        let (to_idx, to_ref) = indexed_bbs_map.get(&to).unwrap();
        {
            from_ref.borrow_mut().succ.push(*to_idx);
        }
        {
            to_ref.borrow_mut().pred.push(*from_idx);
        }
    }
    indexed_bbs_map
        .into_iter()
        .map(|(_, (_, bb))| bb.into_inner())
        .collect::<Vec<_>>()
}

// "A Simple, Fast Dominance Algorithm" - Keith D. Cooper, Timothy J. Harvey, and Ken Kennedy
// https://www.cs.rice.edu/~keith/EMBED/dom.pdf
fn dominance_frontiers(bbs: &[BasicBlock], doms: &[BBIndex]) -> Vec<HashSet<BBIndex>> {
    let mut result = vec![HashSet::new(); bbs.len()];
    for (bb_idx, b) in bbs.iter().enumerate() {
        if b.pred.len() < 2 {
            continue;
        }
        for p in b.pred.iter() {
            let mut runner = *p;
            while runner != doms[bb_idx] {
                result[runner.0].insert(BBIndex(bb_idx));
                runner = doms[runner.0];
            }
        }
    }
    result
}

fn immediate_dominators(bbs: &Vec<BasicBlock>, order: &Vec<BBIndex>) -> Vec<BBIndex> {
    let undefined = BBIndex(usize::max_value());
    let mut doms = vec![undefined; bbs.len()];
    doms[0] = BBIndex(0);
    let mut changed = true;
    while changed {
        changed = false;
        for BBIndex(bb_idx) in order.iter().skip(1) {
            let bb = &bbs[*bb_idx];
            if let Some(first_pred) = bb.pred.iter().find(|bb| doms[bb.0] != undefined) {
                let mut new_idom = *first_pred;
                for BBIndex(p_idx) in bb.pred.iter().copied().filter(|bb| bb != first_pred) {
                    if doms[p_idx] != BBIndex(usize::max_value()) {
                        new_idom = intersect(&mut doms, BBIndex(p_idx), new_idom);
                    }
                }
                if doms[*bb_idx] != new_idom {
                    doms[*bb_idx] = new_idom;
                    changed = true;
                }
            }
        }
    }
    return doms;
}

// Original paper uses reverse indexing: their entry node has index n,
// that's why the compares are reversed
fn intersect(doms: &mut Vec<BBIndex>, b1: BBIndex, b2: BBIndex) -> BBIndex {
    let mut finger1 = b1;
    let mut finger2 = b2;
    while finger1 != finger2 {
        while finger1 > finger2 {
            finger1 = doms[finger1.0];
        }
        while finger2 > finger1 {
            finger2 = doms[finger2.0];
        }
    }
    finger1
}

// "A Simple Algorithm for Global Data Flow Analysis Problems" - Hecht, M. S., & Ullman, J. D. (1975)
fn to_reverse_postorder(input: &Vec<BasicBlock>) -> Vec<BBIndex> {
    let mut i = input.len();
    let mut old = BitVec::from_elem(input.len(), false);
    let mut result = vec![BBIndex(usize::max_value()); input.len()];
    // original uses recursion and implicit stack, we do it explictly
    let mut state = Vec::new();
    state.push((BBIndex(0), 0usize));
    loop {
        if let Some((BBIndex(bb), succ_iter_idx)) = state.last_mut() {
            let bb = *bb;
            if *succ_iter_idx == 0 {
                old.set(bb, true);
            }
            if let Some(BBIndex(succ)) = &input[bb].succ.get(*succ_iter_idx) {
                *succ_iter_idx += 1;
                if !old.get(*succ).unwrap() {
                    state.push((BBIndex(*succ), 0));
                }
            } else {
                state.pop();
                i = i - 1;
                result[i] = BBIndex(bb);
            }
        } else {
            break;
        }
    }
    result
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct BasicBlock {
    start: StmtIndex,
    pred: Vec<BBIndex>,
    succ: Vec<BBIndex>,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, PartialOrd, Ord, Hash)]
struct StmtIndex(pub usize);

impl fmt::Display for StmtIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, PartialOrd, Ord, Hash)]
struct BBIndex(pub usize);

impl fmt::Display for BBIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

enum Statement {
    Label(u32),
    Instruction(Instruction),
    // SPIR-V compatible replacement for PTX predicates
    Conditional(BrachCondition),
    Converison(ImplicitConversion),
    Constant(ConstantDefinition),
}

enum Instruction {
    Ld(ast::LdData, Arg2),
    Mov(ast::MovData, Arg2),
    Mul(ast::MulData, Arg3),
    Add(ast::AddData, Arg3),
    Setp(ast::SetpData, Arg4),
    SetpBool(ast::SetpBoolData, Arg5),
    Not(ast::NotData, Arg2),
    Bra(ast::BraData, Arg1),
    Cvt(ast::CvtData, Arg2),
    Shl(ast::ShlData, Arg3),
    St(ast::StData, Arg2St),
    Ret(ast::RetData),
}

impl Instruction {
    fn visit_id<F: FnMut(bool, spirv::Word)>(&self, f: &mut F) {
        match self {
            Instruction::Ld(_, a) => a.visit_id(f),
            Instruction::Mov(_, a) => a.visit_id(f),
            Instruction::Mul(_, a) => a.visit_id(f),
            Instruction::Add(_, a) => a.visit_id(f),
            Instruction::Setp(_, a) => a.visit_id(f),
            Instruction::SetpBool(_, a) => a.visit_id(f),
            Instruction::Not(_, a) => a.visit_id(f),
            Instruction::Cvt(_, a) => a.visit_id(f),
            Instruction::Shl(_, a) => a.visit_id(f),
            Instruction::St(_, a) => a.visit_id(f),
            Instruction::Bra(_, a) => a.visit_id(f),
            Instruction::Ret(_) => (),
        }
    }

    fn visit_id_mut<F: FnMut(bool, &mut spirv::Word)>(&mut self, f: &mut F) {
        match self {
            Instruction::Ld(_, a) => a.visit_id_mut(f),
            Instruction::Mov(_, a) => a.visit_id_mut(f),
            Instruction::Mul(_, a) => a.visit_id_mut(f),
            Instruction::Add(_, a) => a.visit_id_mut(f),
            Instruction::Setp(_, a) => a.visit_id_mut(f),
            Instruction::SetpBool(_, a) => a.visit_id_mut(f),
            Instruction::Not(_, a) => a.visit_id_mut(f),
            Instruction::Cvt(_, a) => a.visit_id_mut(f),
            Instruction::Shl(_, a) => a.visit_id_mut(f),
            Instruction::St(_, a) => a.visit_id_mut(f),
            Instruction::Bra(_, a) => a.visit_id_mut(f),
            Instruction::Ret(_) => (),
        }
    }

    fn get_type(&self) -> Option<ast::Type> {
        match self {
            Instruction::Add(add, _) => Some(ast::Type::Scalar(add.typ)),
            Instruction::Ret(_) => None,
            Instruction::Ld(ld, _) => Some(ast::Type::Scalar(ld.typ)),
            Instruction::St(st, _) => Some(ast::Type::Scalar(st.typ)),
            Instruction::Mov(mov, _) => Some(mov.typ),
            Instruction::Mul(mul, _) => Some(mul.typ),
            _ => todo!(),
        }
    }

    fn jump_target(&self) -> Option<spirv::Word> {
        match self {
            Instruction::Bra(_, a) => Some(a.src),
            Instruction::Ld(_, _)
            | Instruction::Mov(_, _)
            | Instruction::Mul(_, _)
            | Instruction::Add(_, _)
            | Instruction::Setp(_, _)
            | Instruction::SetpBool(_, _)
            | Instruction::Not(_, _)
            | Instruction::Cvt(_, _)
            | Instruction::Shl(_, _)
            | Instruction::St(_, _)
            | Instruction::Ret(_) => None,
        }
    }

    fn is_terminal(&self) -> bool {
        match self {
            Instruction::Ret(_) => true,
            Instruction::Ld(_, _)
            | Instruction::Mov(_, _)
            | Instruction::Mul(_, _)
            | Instruction::Add(_, _)
            | Instruction::Setp(_, _)
            | Instruction::SetpBool(_, _)
            | Instruction::Not(_, _)
            | Instruction::Cvt(_, _)
            | Instruction::Shl(_, _)
            | Instruction::St(_, _)
            | Instruction::Bra(_, _) => false,
        }
    }
}

struct Arg1 {
    pub src: spirv::Word,
}

impl Arg1 {
    fn visit_id<F: FnMut(bool, spirv::Word)>(&self, f: &mut F) {
        f(false, self.src);
    }

    fn visit_id_mut<F: FnMut(bool, &mut spirv::Word)>(&mut self, f: &mut F) {
        f(false, &mut self.src);
    }
}

struct Arg2 {
    pub dst: spirv::Word,
    pub src: spirv::Word,
}

impl Arg2 {
    fn visit_id<F: FnMut(bool, spirv::Word)>(&self, f: &mut F) {
        f(true, self.dst);
        f(false, self.src);
    }

    fn visit_id_mut<F: FnMut(bool, &mut spirv::Word)>(&mut self, f: &mut F) {
        f(false, &mut self.src);
        f(true, &mut self.dst);
    }
}

pub struct Arg2St {
    pub src1: spirv::Word,
    pub src2: spirv::Word,
}

impl Arg2St {
    fn visit_id<F: FnMut(bool, spirv::Word)>(&self, f: &mut F) {
        f(false, self.src1);
        f(false, self.src2);
    }

    fn visit_id_mut<F: FnMut(bool, &mut spirv::Word)>(&mut self, f: &mut F) {
        f(false, &mut self.src1);
        f(false, &mut self.src2);
    }
}

struct Arg3 {
    pub dst: spirv::Word,
    pub src1: spirv::Word,
    pub src2: spirv::Word,
}

impl Arg3 {
    fn visit_id<F: FnMut(bool, spirv::Word)>(&self, f: &mut F) {
        f(true, self.dst);
        f(false, self.src1);
        f(false, self.src2);
    }

    fn visit_id_mut<F: FnMut(bool, &mut spirv::Word)>(&mut self, f: &mut F) {
        f(false, &mut self.src1);
        f(false, &mut self.src2);
        f(true, &mut self.dst);
    }
}

struct Arg4 {
    pub dst1: spirv::Word,
    pub dst2: Option<spirv::Word>,
    pub src1: spirv::Word,
    pub src2: spirv::Word,
}

impl Arg4 {
    fn visit_id<F: FnMut(bool, spirv::Word)>(&self, f: &mut F) {
        f(true, self.dst1);
        self.dst2.map(|dst2| f(true, dst2));
        f(false, self.src1);
        f(false, self.src2);
    }

    fn visit_id_mut<F: FnMut(bool, &mut spirv::Word)>(&mut self, f: &mut F) {
        f(false, &mut self.src1);
        f(false, &mut self.src2);
        f(true, &mut self.dst1);
        self.dst2.as_mut().map(|dst2| f(true, dst2));
    }
}

struct Arg5 {
    pub dst1: spirv::Word,
    pub dst2: Option<spirv::Word>,
    pub src1: spirv::Word,
    pub src2: spirv::Word,
    pub src3: spirv::Word,
}

impl Arg5 {
    fn visit_id<F: FnMut(bool, spirv::Word)>(&self, f: &mut F) {
        f(true, self.dst1);
        self.dst2.map(|dst2| f(true, dst2));
        f(false, self.src1);
        f(false, self.src2);
        f(false, self.src3);
    }

    fn visit_id_mut<F: FnMut(bool, &mut spirv::Word)>(&mut self, f: &mut F) {
        f(false, &mut self.src1);
        f(false, &mut self.src2);
        f(false, &mut self.src3);
        f(true, &mut self.dst1);
        self.dst2.as_mut().map(|dst2| f(true, dst2));
    }
}

struct ConstantDefinition {
    pub dst: spirv::Word,
    pub typ: ast::ScalarType,
    pub value: i128,
}

impl ConstantDefinition {
    fn visit_id<F: FnMut(bool, spirv::Word)>(&self, f: &mut F) {
        f(true, self.dst);
    }

    fn visit_id_mut<F: FnMut(bool, &mut spirv::Word)>(&mut self, f: &mut F) {
        f(true, &mut self.dst);
    }
}

struct BrachCondition {
    predicate: spirv::Word,
    if_true: spirv::Word,
    if_false: spirv::Word,
}

impl BrachCondition {
    fn visit_id<F: FnMut(bool, spirv::Word)>(&self, f: &mut F) {
        f(false, self.predicate);
        f(false, self.if_true);
        f(false, self.if_false);
    }

    fn visit_id_mut<F: FnMut(bool, &mut spirv::Word)>(&mut self, f: &mut F) {
        f(false, &mut self.predicate);
        f(false, &mut self.if_true);
        f(false, &mut self.if_false);
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
    fn visit_id<F: FnMut(bool, spirv::Word)>(&self, f: &mut F) {
        f(false, self.src);
        f(true, self.dst);
    }

    fn visit_id_mut<F: FnMut(bool, &mut spirv::Word)>(&mut self, f: &mut F) {
        f(false, &mut self.src);
        f(true, &mut self.dst);
    }
}

impl Statement {
    fn from_ast<'a, F: FnMut(&'a str) -> u32>(
        s: ast::Statement<&'a str>,
        get_id: &mut F,
    ) -> Option<ast::Statement<spirv::Word>> {
        match s {
            ast::Statement::Label(name) => Some(ast::Statement::Label(get_id(name))),
            ast::Statement::Instruction(p, i) => Some(ast::Statement::Instruction(
                p.map(|p| p.map_id(get_id)),
                i.map_id(get_id),
            )),
            ast::Statement::Variable(_) => None,
        }
    }

    fn visit_id<F: FnMut(bool, spirv::Word)>(&self, f: &mut F) {
        match self {
            Statement::Label(id) => f(false, *id),
            Statement::Instruction(inst) => inst.visit_id(f),
            Statement::Conditional(bra) => bra.visit_id(f),
            Statement::Converison(conv) => conv.visit_id(f),
            Statement::Constant(cons) => cons.visit_id(f),
        }
    }

    // WARNING: It is very important to first visit src operands and then dst operands,
    // otherwise SSA renaming will yield weird results
    fn visit_id_mut<F: FnMut(bool, &mut spirv::Word)>(&mut self, f: &mut F) {
        match self {
            Statement::Label(id) => f(false, id),
            Statement::Instruction(inst) => inst.visit_id_mut(f),
            Statement::Conditional(bra) => bra.visit_id_mut(f),
            Statement::Converison(conv) => conv.visit_id_mut(f),
            Statement::Constant(cons) => cons.visit_id_mut(f),
        }
    }
}

impl<T> ast::PredAt<T> {
    fn map_id<U, F: FnMut(T) -> U>(self, f: &mut F) -> ast::PredAt<U> {
        ast::PredAt {
            not: self.not,
            label: f(self.label),
        }
    }
}

impl<T> ast::Instruction<T> {
    fn map_id<U, F: FnMut(T) -> U>(self, f: &mut F) -> ast::Instruction<U> {
        match self {
            ast::Instruction::Ld(d, a) => ast::Instruction::Ld(d, a.map_id(f)),
            ast::Instruction::Mov(d, a) => ast::Instruction::Mov(d, a.map_id(f)),
            ast::Instruction::Mul(d, a) => ast::Instruction::Mul(d, a.map_id(f)),
            ast::Instruction::Add(d, a) => ast::Instruction::Add(d, a.map_id(f)),
            ast::Instruction::Setp(d, a) => ast::Instruction::Setp(d, a.map_id(f)),
            ast::Instruction::SetpBool(d, a) => ast::Instruction::SetpBool(d, a.map_id(f)),
            ast::Instruction::Not(d, a) => ast::Instruction::Not(d, a.map_id(f)),
            ast::Instruction::Bra(d, a) => ast::Instruction::Bra(d, a.map_id(f)),
            ast::Instruction::Cvt(d, a) => ast::Instruction::Cvt(d, a.map_id(f)),
            ast::Instruction::Shl(d, a) => ast::Instruction::Shl(d, a.map_id(f)),
            ast::Instruction::St(d, a) => ast::Instruction::St(d, a.map_id(f)),
            ast::Instruction::Ret(d) => ast::Instruction::Ret(d),
        }
    }

    fn visit_id<F: FnMut(bool, &T)>(&self, f: &mut F) {
        match self {
            ast::Instruction::Ld(_, a) => a.visit_id(f),
            ast::Instruction::Mov(_, a) => a.visit_id(f),
            ast::Instruction::Mul(_, a) => a.visit_id(f),
            ast::Instruction::Add(_, a) => a.visit_id(f),
            ast::Instruction::Setp(_, a) => a.visit_id(f),
            ast::Instruction::SetpBool(_, a) => a.visit_id(f),
            ast::Instruction::Not(_, a) => a.visit_id(f),
            ast::Instruction::Cvt(_, a) => a.visit_id(f),
            ast::Instruction::Shl(_, a) => a.visit_id(f),
            ast::Instruction::St(_, a) => a.visit_id(f),
            ast::Instruction::Bra(_, a) => a.visit_id(f),
            ast::Instruction::Ret(_) => (),
        }
    }

    fn visit_id_mut<F: FnMut(bool, &mut T)>(&mut self, f: &mut F) {
        match self {
            ast::Instruction::Ld(_, a) => a.visit_id_mut(f),
            ast::Instruction::Mov(_, a) => a.visit_id_mut(f),
            ast::Instruction::Mul(_, a) => a.visit_id_mut(f),
            ast::Instruction::Add(_, a) => a.visit_id_mut(f),
            ast::Instruction::Setp(_, a) => a.visit_id_mut(f),
            ast::Instruction::SetpBool(_, a) => a.visit_id_mut(f),
            ast::Instruction::Not(_, a) => a.visit_id_mut(f),
            ast::Instruction::Cvt(_, a) => a.visit_id_mut(f),
            ast::Instruction::Shl(_, a) => a.visit_id_mut(f),
            ast::Instruction::St(_, a) => a.visit_id_mut(f),
            ast::Instruction::Bra(_, a) => a.visit_id_mut(f),
            ast::Instruction::Ret(_) => (),
        }
    }

    fn get_type(&self) -> Option<ast::Type> {
        match self {
            ast::Instruction::Add(add, _) => Some(ast::Type::Scalar(add.typ)),
            ast::Instruction::Ret(_) => None,
            ast::Instruction::Ld(ld, _) => Some(ast::Type::Scalar(ld.typ)),
            ast::Instruction::St(st, _) => Some(ast::Type::Scalar(st.typ)),
            ast::Instruction::Mov(mov, _) => Some(mov.typ),
            ast::Instruction::Mul(mul, _) => Some(mul.typ),
            _ => todo!(),
        }
    }
}

impl<T: Copy> ast::Instruction<T> {
    fn jump_target(&self) -> Option<T> {
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

    fn is_terminal(&self) -> bool {
        match self {
            ast::Instruction::Ret(_) => true,
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
            | ast::Instruction::Bra(_, _) => false,
        }
    }
}

impl<T> ast::Arg1<T> {
    fn map_id<U, F: FnMut(T) -> U>(self, f: &mut F) -> ast::Arg1<U> {
        ast::Arg1 { src: f(self.src) }
    }

    fn visit_id<F: FnMut(bool, &T)>(&self, f: &mut F) {
        f(false, &self.src);
    }

    fn visit_id_mut<F: FnMut(bool, &mut T)>(&mut self, f: &mut F) {
        f(false, &mut self.src);
    }
}

impl<T> ast::Arg2<T> {
    fn map_id<U, F: FnMut(T) -> U>(self, f: &mut F) -> ast::Arg2<U> {
        ast::Arg2 {
            dst: f(self.dst),
            src: self.src.map_id(f),
        }
    }

    fn visit_id<F: FnMut(bool, &T)>(&self, f: &mut F) {
        f(true, &self.dst);
        self.src.visit_id(f);
    }

    fn visit_id_mut<F: FnMut(bool, &mut T)>(&mut self, f: &mut F) {
        self.src.visit_id_mut(f);
        f(true, &mut self.dst);
    }
}

impl<T> ast::Arg2St<T> {
    fn map_id<U, F: FnMut(T) -> U>(self, f: &mut F) -> ast::Arg2St<U> {
        ast::Arg2St {
            src1: self.src1.map_id(f),
            src2: self.src2.map_id(f),
        }
    }

    fn visit_id<F: FnMut(bool, &T)>(&self, f: &mut F) {
        self.src1.visit_id(f);
        self.src2.visit_id(f);
    }

    fn visit_id_mut<F: FnMut(bool, &mut T)>(&mut self, f: &mut F) {
        self.src1.visit_id_mut(f);
        self.src2.visit_id_mut(f);
    }
}

impl<T> ast::Arg2Mov<T> {
    fn map_id<U, F: FnMut(T) -> U>(self, f: &mut F) -> ast::Arg2Mov<U> {
        ast::Arg2Mov {
            dst: f(self.dst),
            src: self.src.map_id(f),
        }
    }

    fn visit_id<F: FnMut(bool, &T)>(&self, f: &mut F) {
        f(true, &self.dst);
        self.src.visit_id(f);
    }

    fn visit_id_mut<F: FnMut(bool, &mut T)>(&mut self, f: &mut F) {
        self.src.visit_id_mut(f);
        f(true, &mut self.dst);
    }
}

impl<T> ast::Arg3<T> {
    fn map_id<U, F: FnMut(T) -> U>(self, f: &mut F) -> ast::Arg3<U> {
        ast::Arg3 {
            dst: f(self.dst),
            src1: self.src1.map_id(f),
            src2: self.src2.map_id(f),
        }
    }

    fn visit_id<F: FnMut(bool, &T)>(&self, f: &mut F) {
        f(true, &self.dst);
        self.src1.visit_id(f);
        self.src2.visit_id(f);
    }

    fn visit_id_mut<F: FnMut(bool, &mut T)>(&mut self, f: &mut F) {
        self.src1.visit_id_mut(f);
        self.src2.visit_id_mut(f);
        f(true, &mut self.dst);
    }
}

impl<T> ast::Arg4<T> {
    fn map_id<U, F: FnMut(T) -> U>(self, f: &mut F) -> ast::Arg4<U> {
        ast::Arg4 {
            dst1: f(self.dst1),
            dst2: self.dst2.map(|i| f(i)),
            src1: self.src1.map_id(f),
            src2: self.src2.map_id(f),
        }
    }

    fn visit_id<F: FnMut(bool, &T)>(&self, f: &mut F) {
        f(true, &self.dst1);
        self.dst2.as_ref().map(|i| f(true, i));
        self.src1.visit_id(f);
        self.src2.visit_id(f);
    }

    fn visit_id_mut<F: FnMut(bool, &mut T)>(&mut self, f: &mut F) {
        self.src1.visit_id_mut(f);
        self.src2.visit_id_mut(f);
        f(true, &mut self.dst1);
        self.dst2.as_mut().map(|i| f(true, i));
    }
}

impl<T> ast::Arg5<T> {
    fn map_id<U, F: FnMut(T) -> U>(self, f: &mut F) -> ast::Arg5<U> {
        ast::Arg5 {
            dst1: f(self.dst1),
            dst2: self.dst2.map(|i| f(i)),
            src1: self.src1.map_id(f),
            src2: self.src2.map_id(f),
            src3: self.src3.map_id(f),
        }
    }

    fn visit_id<F: FnMut(bool, &T)>(&self, f: &mut F) {
        f(true, &self.dst1);
        self.dst2.as_ref().map(|i| f(true, i));
        self.src1.visit_id(f);
        self.src2.visit_id(f);
        self.src3.visit_id(f);
    }

    fn visit_id_mut<F: FnMut(bool, &mut T)>(&mut self, f: &mut F) {
        self.src1.visit_id_mut(f);
        self.src2.visit_id_mut(f);
        self.src3.visit_id_mut(f);
        f(true, &mut self.dst1);
        self.dst2.as_mut().map(|i| f(true, i));
    }
}

impl<T> ast::Operand<T> {
    fn map_id<U, F: FnMut(T) -> U>(self, f: &mut F) -> ast::Operand<U> {
        match self {
            ast::Operand::Reg(i) => ast::Operand::Reg(f(i)),
            ast::Operand::RegOffset(i, o) => ast::Operand::RegOffset(f(i), o),
            ast::Operand::Imm(v) => ast::Operand::Imm(v),
        }
    }

    fn visit_id<F: FnMut(bool, &T)>(&self, f: &mut F) {
        match self {
            ast::Operand::Reg(i) => f(false, i),
            ast::Operand::RegOffset(i, _) => f(false, i),
            ast::Operand::Imm(_) => (),
        }
    }

    fn visit_id_mut<F: FnMut(bool, &mut T)>(&mut self, f: &mut F) {
        match self {
            ast::Operand::Reg(i) => f(false, i),
            ast::Operand::RegOffset(i, _) => f(false, i),
            ast::Operand::Imm(_) => (),
        }
    }
}

impl<T> ast::MovOperand<T> {
    fn map_id<U, F: FnMut(T) -> U>(self, f: &mut F) -> ast::MovOperand<U> {
        match self {
            ast::MovOperand::Op(o) => ast::MovOperand::Op(o.map_id(f)),
            ast::MovOperand::Vec(s1, s2) => ast::MovOperand::Vec(s1, s2),
        }
    }

    fn visit_id<F: FnMut(bool, &T)>(&self, f: &mut F) {
        match self {
            ast::MovOperand::Op(o) => o.visit_id(f),
            ast::MovOperand::Vec(_, _) => (),
        }
    }

    fn visit_id_mut<F: FnMut(bool, &mut T)>(&mut self, f: &mut F) {
        match self {
            ast::MovOperand::Op(o) => o.visit_id_mut(f),
            ast::MovOperand::Vec(_, _) => (),
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

impl ast::Type {
    fn try_as_scalar(self) -> Option<ast::ScalarType> {
        match self {
            ast::Type::Scalar(s) => Some(s),
            ast::Type::ExtendedScalar(_) => None,
        }
    }
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

fn insert_implicit_conversions_ld_src<TypeCheck: Fn(spirv::Word) -> ast::Type>(
    func: &mut Vec<Statement>,
    instr_type: ast::Type,
    type_check: &TypeCheck,
    new_id: &mut impl FnMut(Option<ast::Type>) -> spirv::Word,
    state_space: ast::LdStateSpace,
    src: spirv::Word,
) -> spirv::Word {
    match state_space {
        ast::LdStateSpace::Param => insert_implicit_conversions_ld_src_impl(
            func,
            type_check,
            new_id,
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
                type_check,
                new_id,
                new_src_type,
                src,
                should_convert_ld_generic_src_to_bitcast,
            );
            insert_conversion_src(
                func,
                new_id,
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
    TypeCheck: Fn(spirv::Word) -> ast::Type,
    ShouldConvert: FnOnce(ast::Type, ast::Type) -> Option<ConversionKind>,
>(
    func: &mut Vec<Statement>,
    type_check: &TypeCheck,
    new_id: &mut impl FnMut(Option<ast::Type>) -> spirv::Word,
    instr_type: ast::Type,
    src: spirv::Word,
    should_convert: ShouldConvert,
) -> spirv::Word {
    let src_type = type_check(src);
    if let Some(conv) = should_convert(src_type, instr_type) {
        insert_conversion_src(func, new_id, src, src_type, instr_type, conv)
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
    func: &mut Vec<Statement>,
    new_id: &mut impl FnMut(Option<ast::Type>) -> spirv::Word,
    src: spirv::Word,
    src_type: ast::Type,
    instr_type: ast::Type,
    conv: ConversionKind,
) -> spirv::Word {
    let temp_src = new_id(Some(instr_type));
    func.push(Statement::Converison(ImplicitConversion {
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
    TypeCheck: Fn(spirv::Word) -> ast::Type,
    ShouldConvert: FnOnce(ast::Type, ast::ScalarType) -> Option<ConversionKind>,
    Setter: Fn(&mut T) -> &mut spirv::Word,
    ToInstruction: FnOnce(T) -> Instruction,
>(
    func: &mut Vec<Statement>,
    instr_type: ast::ScalarType,
    type_check: &TypeCheck,
    new_id: &mut impl FnMut(Option<ast::Type>) -> spirv::Word,
    should_convert: ShouldConvert,
    mut t: T,
    setter: Setter,
    to_inst: ToInstruction,
) {
    let dst = setter(&mut t);
    let dst_type = type_check(*dst);
    let dst_coercion = should_convert(dst_type, instr_type)
        .map(|conv| get_conversion_dst(new_id, dst, ast::Type::Scalar(instr_type), dst_type, conv));
    func.push(Statement::Instruction(to_inst(t)));
    if let Some(conv) = dst_coercion {
        func.push(conv);
    }
}

#[must_use]
fn get_conversion_dst(
    new_id: &mut impl FnMut(Option<ast::Type>) -> spirv::Word,
    dst: &mut spirv::Word,
    instr_type: ast::Type,
    dst_type: ast::Type,
    kind: ConversionKind,
) -> Statement {
    let original_dst = *dst;
    let temp_dst = new_id(Some(instr_type));
    *dst = temp_dst;
    Statement::Converison(ImplicitConversion {
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

fn insert_implicit_bitcasts<TypeCheck: Fn(spirv::Word) -> ast::Type>(
    func: &mut Vec<Statement>,
    type_check: &TypeCheck,
    new_id: &mut impl FnMut(Option<ast::Type>) -> spirv::Word,
    mut instr: Instruction,
) {
    let mut dst_coercion = None;
    if let Some(instr_type) = instr.get_type() {
        instr.visit_id_mut(&mut |is_dst, id| {
            let id_type = type_check(*id);
            if should_bitcast(instr_type, type_check(*id)) {
                if is_dst {
                    dst_coercion = Some(get_conversion_dst(
                        new_id,
                        id,
                        instr_type,
                        id_type,
                        ConversionKind::Default,
                    ));
                } else {
                    *id = insert_conversion_src(
                        func,
                        new_id,
                        *id,
                        id_type,
                        instr_type,
                        ConversionKind::Default,
                    );
                }
            }
        });
    }
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
    use crate::ptx;

    // page 411
    #[test]
    fn to_reverse_postorder1() {
        let input = vec![
            BasicBlock {
                // A
                start: StmtIndex(0),
                pred: vec![],
                succ: vec![BBIndex(1), BBIndex(2)],
            },
            BasicBlock {
                // B
                start: StmtIndex(1),
                pred: vec![BBIndex(0), BBIndex(11)],
                succ: vec![BBIndex(3), BBIndex(6)],
            },
            BasicBlock {
                // C
                start: StmtIndex(2),
                pred: vec![BBIndex(0), BBIndex(4)],
                succ: vec![BBIndex(4), BBIndex(7)],
            },
            BasicBlock {
                // D
                start: StmtIndex(3),
                pred: vec![BBIndex(1)],
                succ: vec![BBIndex(5), BBIndex(6)],
            },
            BasicBlock {
                // E
                start: StmtIndex(4),
                pred: vec![BBIndex(2)],
                succ: vec![BBIndex(2), BBIndex(7)],
            },
            BasicBlock {
                // F
                start: StmtIndex(5),
                pred: vec![BBIndex(3)],
                succ: vec![BBIndex(8), BBIndex(10)],
            },
            BasicBlock {
                // G
                start: StmtIndex(6),
                pred: vec![BBIndex(1), BBIndex(3)],
                succ: vec![BBIndex(9)],
            },
            BasicBlock {
                // H
                start: StmtIndex(7),
                pred: vec![BBIndex(2), BBIndex(4)],
                succ: vec![BBIndex(12)],
            },
            BasicBlock {
                // I
                start: StmtIndex(8),
                pred: vec![BBIndex(5), BBIndex(9)],
                succ: vec![BBIndex(11)],
            },
            BasicBlock {
                // J
                start: StmtIndex(9),
                pred: vec![BBIndex(6)],
                succ: vec![BBIndex(8)],
            },
            BasicBlock {
                // K
                start: StmtIndex(10),
                pred: vec![BBIndex(5)],
                succ: vec![BBIndex(11)],
            },
            BasicBlock {
                // L
                start: StmtIndex(11),
                pred: vec![BBIndex(8), BBIndex(10)],
                succ: vec![BBIndex(1), BBIndex(12)],
            },
            BasicBlock {
                // M
                start: StmtIndex(12),
                pred: vec![BBIndex(7), BBIndex(11)],
                succ: vec![],
            },
        ];
        let rpostord = to_reverse_postorder(&input);
        assert_eq!(
            rpostord,
            vec![
                BBIndex(0),  // A
                BBIndex(2),  // C
                BBIndex(4),  // E
                BBIndex(7),  // H
                BBIndex(1),  // B
                BBIndex(3),  // D
                BBIndex(6),  // G
                BBIndex(9),  // J
                BBIndex(5),  // F
                BBIndex(10), // K
                BBIndex(8),  // I
                BBIndex(11), // L
                BBIndex(12), // M
            ]
        );
    }

    #[test]
    fn get_basic_blocks_empty() {
        let func = Vec::new();
        let bbs = get_basic_blocks(&func);
        assert_eq!(
            bbs,
            vec![BasicBlock {
                start: StmtIndex(0),
                pred: vec![],
                succ: vec![],
            }]
        );
    }

    #[test]
    fn get_basic_blocks_miniloop() {
        let func = vec![
            Statement::Label(12),
            Statement::Instruction(Instruction::Bra(
                ast::BraData { uniform: false },
                Arg1 { src: 12 },
            )),
        ];
        let bbs = get_basic_blocks(&func);
        assert_eq!(
            bbs,
            vec![BasicBlock {
                start: StmtIndex(0),
                pred: vec![BBIndex(0)],
                succ: vec![BBIndex(0)],
            }]
        );
    }

    // "A Simple, Fast Dominance Algorithm" - Fig. 4
    fn simple_fast_dom_fig4() -> Vec<BasicBlock> {
        vec![
            BasicBlock {
                start: StmtIndex(6),
                pred: vec![],
                succ: vec![BBIndex(1), BBIndex(2)],
            },
            BasicBlock {
                start: StmtIndex(5),
                pred: vec![BBIndex(0)],
                succ: vec![BBIndex(5)],
            },
            BasicBlock {
                start: StmtIndex(4),
                pred: vec![BBIndex(0)],
                succ: vec![BBIndex(3), BBIndex(4)],
            },
            BasicBlock {
                start: StmtIndex(3),
                pred: vec![BBIndex(2), BBIndex(4)],
                succ: vec![BBIndex(4)],
            },
            BasicBlock {
                start: StmtIndex(2),
                pred: vec![BBIndex(2), BBIndex(3), BBIndex(5)],
                succ: vec![BBIndex(3), BBIndex(5)],
            },
            BasicBlock {
                start: StmtIndex(1),
                pred: vec![BBIndex(1), BBIndex(4)],
                succ: vec![BBIndex(4)],
            },
        ]
    }

    #[test]
    fn immediate_dominators1() {
        let input = simple_fast_dom_fig4();
        let reverse_postorder = vec![
            BBIndex(0),
            BBIndex(1),
            BBIndex(2),
            BBIndex(3),
            BBIndex(4),
            BBIndex(5),
        ];
        let imm_dominators = immediate_dominators(&input, &reverse_postorder);
        assert_eq!(
            imm_dominators,
            vec![
                BBIndex(0),
                BBIndex(0),
                BBIndex(0),
                BBIndex(0),
                BBIndex(0),
                BBIndex(0)
            ]
        );
    }

    // page 411
    #[test]
    fn immediate_dominators2() {
        let input = vec![
            BasicBlock {
                // A
                start: StmtIndex(0),
                pred: vec![],
                succ: vec![BBIndex(1), BBIndex(2)],
            },
            BasicBlock {
                // B
                start: StmtIndex(1),
                pred: vec![BBIndex(0), BBIndex(11)],
                succ: vec![BBIndex(3), BBIndex(6)],
            },
            BasicBlock {
                // C
                start: StmtIndex(2),
                pred: vec![BBIndex(0), BBIndex(4)],
                succ: vec![BBIndex(4), BBIndex(7)],
            },
            BasicBlock {
                // D
                start: StmtIndex(3),
                pred: vec![BBIndex(1)],
                succ: vec![BBIndex(5), BBIndex(6)],
            },
            BasicBlock {
                // E
                start: StmtIndex(4),
                pred: vec![BBIndex(2)],
                succ: vec![BBIndex(2), BBIndex(7)],
            },
            BasicBlock {
                // F
                start: StmtIndex(5),
                pred: vec![BBIndex(3)],
                succ: vec![BBIndex(8), BBIndex(10)],
            },
            BasicBlock {
                // G
                start: StmtIndex(6),
                pred: vec![BBIndex(1), BBIndex(3)],
                succ: vec![BBIndex(9)],
            },
            BasicBlock {
                // H
                start: StmtIndex(7),
                pred: vec![BBIndex(2), BBIndex(4)],
                succ: vec![BBIndex(12)],
            },
            BasicBlock {
                // I
                start: StmtIndex(8),
                pred: vec![BBIndex(5), BBIndex(9)],
                succ: vec![BBIndex(11)],
            },
            BasicBlock {
                // J
                start: StmtIndex(9),
                pred: vec![BBIndex(6)],
                succ: vec![BBIndex(8)],
            },
            BasicBlock {
                // K
                start: StmtIndex(10),
                pred: vec![BBIndex(5)],
                succ: vec![BBIndex(11)],
            },
            BasicBlock {
                // L
                start: StmtIndex(11),
                pred: vec![BBIndex(8), BBIndex(10)],
                succ: vec![BBIndex(1), BBIndex(12)],
            },
            BasicBlock {
                // M
                start: StmtIndex(12),
                pred: vec![BBIndex(7), BBIndex(11)],
                succ: vec![],
            },
        ];
        let reverse_postorder = vec![
            BBIndex(0),  // A
            BBIndex(2),  // C
            BBIndex(4),  // E
            BBIndex(7),  // H
            BBIndex(1),  // B
            BBIndex(3),  // D
            BBIndex(6),  // G
            BBIndex(9),  // J
            BBIndex(5),  // F
            BBIndex(10), // K
            BBIndex(8),  // I
            BBIndex(11), // L
            BBIndex(12), // M
        ];
        let imm_dominators = immediate_dominators(&input, &reverse_postorder);
        assert_eq!(
            imm_dominators,
            vec![
                BBIndex(0),
                BBIndex(0),
                BBIndex(0),
                BBIndex(1),
                BBIndex(2),
                BBIndex(3),
                BBIndex(1),
                BBIndex(2),
                BBIndex(1),
                BBIndex(6),
                BBIndex(5),
                BBIndex(1),
                BBIndex(0)
            ]
        );
    }

    fn sort_pred_succ(bb: &mut BasicBlock) {
        bb.pred.sort();
        bb.succ.sort();
    }

    // page 403
    const FIG_19_4: &'static str = "{
        .reg.u32 i;
        .reg.u32 j;
        .reg.u32 k;
        .reg.pred p;
        .reg.pred q;

        mov.u32     i, 1;
        mov.u32     j, 1;
        mov.u32     k, 0;
    block_2:
        setp.ge.u32 p, k, 100;
        @p bra      block_4; // conditional p block_4 if_false1
                             // if_false1:
        setp.ge.u32 q, j, 20;
        @q bra      block_6; // conditional q block_6 if_false2
                             // if_false2:
        mov.u32     j, i;
        add.u32     k, k, 1;
        bra         block_7;
    block_6:
        mov.u32     j, k;
        add.u32     k, k, 2;
    block_7:
        bra         block_2;
    block_4:
        ret;
    }";

    #[test]
    fn get_basic_blocks_fig_19_4() {
        let func = FIG_19_4;
        let mut errors = Vec::new();
        let ast = ptx::FunctionBodyParser::new()
            .parse(&mut errors, func)
            .unwrap();
        assert_eq!(errors.len(), 0);
        let mut constant_ids = HashMap::new();
        collect_label_ids(&mut constant_ids, &ast);
        let registers = collect_var_definitions(&[], &ast);
        let mut type_check = HashMap::new();
        let (normalized_ids, mut unique_ids) =
            normalize_identifiers(ast, &constant_ids, &mut type_check, registers);
        let type_check = RefCell::new(type_check);
        let new_id = &mut |typ: Option<ast::Type>| {
            let to_insert = unique_ids;
            {
                let mut type_check = type_check.borrow_mut();
                typ.map(|t| (*type_check).insert(to_insert, t));
            }
            unique_ids += 1;
            to_insert
        };
        let normalized_stmts = normalize_statements(normalized_ids, new_id);
        let mut bbs = get_basic_blocks(&normalized_stmts);
        bbs.iter_mut().for_each(sort_pred_succ);
        assert_eq!(
            bbs,
            vec![
                BasicBlock {
                    start: StmtIndex(0),
                    pred: vec![],
                    succ: vec![BBIndex(1)],
                },
                BasicBlock {
                    start: StmtIndex(6),
                    pred: vec![BBIndex(0), BBIndex(5)],
                    succ: vec![BBIndex(2), BBIndex(6)],
                },
                BasicBlock {
                    start: StmtIndex(10),
                    pred: vec![BBIndex(1)],
                    succ: vec![BBIndex(3), BBIndex(4)],
                },
                BasicBlock {
                    start: StmtIndex(14),
                    pred: vec![BBIndex(2)],
                    succ: vec![BBIndex(5)],
                },
                BasicBlock {
                    start: StmtIndex(19),
                    pred: vec![BBIndex(2)],
                    succ: vec![BBIndex(5)],
                },
                BasicBlock {
                    start: StmtIndex(23),
                    pred: vec![BBIndex(3), BBIndex(4)],
                    succ: vec![BBIndex(1)],
                },
                BasicBlock {
                    start: StmtIndex(25),
                    pred: vec![BBIndex(1)],
                    succ: vec![],
                },
            ]
        );
    }

    fn cfg_fig_19_4() -> Vec<BasicBlock> {
        vec![
            BasicBlock {
                start: StmtIndex(0),
                pred: vec![],
                succ: vec![BBIndex(1)],
            },
            BasicBlock {
                start: StmtIndex(3),
                pred: vec![BBIndex(0), BBIndex(5)],
                succ: vec![BBIndex(2), BBIndex(6)],
            },
            BasicBlock {
                start: StmtIndex(6),
                pred: vec![BBIndex(1)],
                succ: vec![BBIndex(3), BBIndex(4)],
            },
            BasicBlock {
                start: StmtIndex(9),
                pred: vec![BBIndex(2)],
                succ: vec![BBIndex(5)],
            },
            BasicBlock {
                start: StmtIndex(13),
                pred: vec![BBIndex(2)],
                succ: vec![BBIndex(5)],
            },
            BasicBlock {
                start: StmtIndex(16),
                pred: vec![BBIndex(3), BBIndex(4)],
                succ: vec![BBIndex(1)],
            },
            BasicBlock {
                start: StmtIndex(18),
                pred: vec![BBIndex(1)],
                succ: vec![],
            },
        ]
    }

    // cfg from 19.4 with slighlty shuffled order of succ/pred
    #[test]
    fn reverse_postorder_fig_19_4() {
        let mut cfg = cfg_fig_19_4();
        cfg[1].pred.swap(0, 1);
        cfg[2].succ.swap(0, 1);
        let rpostorder = vec![
            BBIndex(0),
            BBIndex(1),
            BBIndex(6),
            BBIndex(2),
            BBIndex(3),
            BBIndex(4),
            BBIndex(5),
        ];
        let doms = immediate_dominators(&cfg, &rpostorder);
        assert_eq!(
            doms,
            vec![
                BBIndex(0),
                BBIndex(0),
                BBIndex(1),
                BBIndex(2),
                BBIndex(2),
                BBIndex(2),
                BBIndex(1)
            ]
        );
    }

    #[test]
    fn dominance_frontiers_fig_19_4() {
        let cfg = cfg_fig_19_4();
        let order = to_reverse_postorder(&cfg);
        let doms = immediate_dominators(&cfg, &order);
        let dom_fronts = dominance_frontiers(&cfg, &doms)
            .into_iter()
            .map(|hs| hs.into_iter().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let should = vec![
            vec![],
            vec![BBIndex(1)],
            vec![BBIndex(1)],
            vec![BBIndex(5)],
            vec![BBIndex(5)],
            vec![BBIndex(1)],
            vec![],
        ];
        assert_eq!(dom_fronts, should);
    }

    #[test]
    fn gather_phi_sets_fig_19_4() {
        let func = FIG_19_4;
        let mut errors = Vec::new();
        let fn_ast = ptx::FunctionBodyParser::new()
            .parse(&mut errors, func)
            .unwrap();
        assert_eq!(errors.len(), 0);
        let mut constant_ids = HashMap::new();
        collect_label_ids(&mut constant_ids, &fn_ast);
        assert_eq!(constant_ids.len(), 4);

        let mut type_check = HashMap::new();
        let registers = collect_var_definitions(&[], &fn_ast);
        let (normalized_ids, mut unique_ids) =
            normalize_identifiers(fn_ast, &constant_ids, &mut type_check, registers);
        let type_check = RefCell::new(type_check);
        let new_id = &mut |typ: Option<ast::Type>| {
            let to_insert = unique_ids;
            {
                let mut type_check = type_check.borrow_mut();
                typ.map(|t| (*type_check).insert(to_insert, t));
            }
            unique_ids += 1;
            to_insert
        };
        let normalized_stmts = normalize_statements(normalized_ids, new_id);
        let bbs = get_basic_blocks(&normalized_stmts);
        let rpostorder = to_reverse_postorder(&bbs);
        let doms = immediate_dominators(&bbs, &rpostorder);
        let dom_fronts = dominance_frontiers(&bbs, &doms);
        let phi = gather_phi_sets(
            &normalized_stmts,
            constant_ids.len() as u32,
            unique_ids,
            &bbs,
            &dom_fronts,
        );
        assert_eq!(
            phi,
            vec![
                HashSet::new(),
                to_hashset(vec![5, 6]),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                to_hashset(vec![5, 6]),
                HashSet::new()
            ]
        );
    }

    fn to_hashset<T: std::hash::Hash + Eq>(v: Vec<T>) -> HashSet<T> {
        v.into_iter().collect::<HashSet<T>>()
    }

    #[test]
    fn ssa_rename_19_4() {
        let func = FIG_19_4;
        let mut errors = Vec::new();
        let fn_ast = ptx::FunctionBodyParser::new()
            .parse(&mut errors, func)
            .unwrap();
        assert_eq!(errors.len(), 0);
        let (func, _, mut ssa_phis, unique_ids) = to_ssa(&[], fn_ast);
        assert_phi_dst_id(unique_ids, &ssa_phis);
        assert_dst_unique(&func, &ssa_phis);
        sort_phi(&mut ssa_phis);

        let i1 = unique_ids;
        let j1 = unique_ids + 1;
        let j2 = get_dst_from_src(&ssa_phis[1], j1);
        let j3 = get_dst(&func[10]);
        let j4 = get_dst_from_src(&ssa_phis[5], j3);
        let j5 = get_dst(&func[14]);
        let k1 = unique_ids + 2;
        let k2 = get_dst_from_src(&ssa_phis[1], k1);
        let k3 = get_dst(&func[11]);
        let k4 = get_dst_from_src(&ssa_phis[5], k3);
        let k5 = get_dst(&func[15]);
        let p1 = get_dst(&func[4]);
        let q1 = get_dst(&func[7]);
        let block_2 = get_label(&func[3]);
        let if_false1 = get_label(&func[6]);
        let if_false2 = get_label(&func[9]);
        let block_6 = get_label(&func[13]);
        let block_7 = get_label(&func[16]);
        let block_4 = get_label(&func[18]);

        {
            assert_eq!(get_ids(&func[0]), vec![i1]);
            assert_eq!(get_ids(&func[1]), vec![j1]);
            assert_eq!(get_ids(&func[2]), vec![k1]);

            assert_eq!(
                ssa_phis[1],
                to_phi(vec![(j2, vec![j4, j1]), (k2, vec![k4, k1])])
            );
            assert_eq!(get_ids(&func[3]), vec![block_2]);
            assert_eq!(get_ids(&func[4]), vec![p1, k2]);
            assert_eq!(get_ids(&func[5]), vec![p1, block_4, if_false1]);

            assert_eq!(get_ids(&func[6]), vec![if_false1]);
            assert_eq!(get_ids(&func[7]), vec![q1, j2]);
            assert_eq!(get_ids(&func[8]), vec![q1, block_6, if_false2]);

            assert_eq!(get_ids(&func[9]), vec![if_false2]);
            assert_eq!(get_ids(&func[10]), vec![j3, i1]);
            assert_eq!(get_ids(&func[11]), vec![k3, k2]);
            assert_eq!(get_ids(&func[12]), vec![block_7]);

            assert_eq!(get_ids(&func[13]), vec![block_6]);
            assert_eq!(get_ids(&func[14]), vec![j5, k2]);
            assert_eq!(get_ids(&func[15]), vec![k5, k2]);

            assert_eq!(
                ssa_phis[5],
                to_phi(vec![(j4, vec![j3, j5]), (k4, vec![k3, k5])])
            );
            assert_eq!(get_ids(&func[16]), vec![block_7]);
            assert_eq!(get_ids(&func[17]), vec![block_2]);

            assert_eq!(get_ids(&func[18]), vec![block_4]);
            assert_eq!(get_ids(&func[19]), vec![]);
        }
    }

    fn assert_phi_dst_id(max_id: spirv::Word, phis: &[Vec<PhiDef>]) {
        for phi_set in phis {
            for phi in phi_set {
                assert!(phi.dst > max_id);
            }
        }
    }

    fn assert_dst_unique(func: &[Statement], phis: &[Vec<PhiDef>]) {
        let mut seen = HashSet::new();
        for s in func {
            s.visit_id(&mut |is_dst, id| {
                if is_dst {
                    assert!(seen.insert(id));
                }
            });
        }
        for phi_set in phis {
            for phi in phi_set {
                assert!(seen.insert(phi.dst));
            }
        }
    }

    fn get_ids(s: &Statement) -> Vec<spirv::Word> {
        let mut result = Vec::new();
        s.visit_id(&mut |_, id| {
            result.push(id);
        });
        result
    }

    fn sort_phi(phis: &mut [Vec<PhiDef>]) {
        for phi_set in phis {
            phi_set.sort_by_key(|phi| phi.dst);
        }
    }

    fn to_phi(raw: Vec<(spirv::Word, Vec<spirv::Word>)>) -> Vec<PhiDef> {
        let result = raw
            .into_iter()
            .map(|(dst, src)| PhiDef {
                dst: dst,
                src: src.into_iter().collect::<HashSet<_>>(),
            })
            .collect::<Vec<_>>();
        let mut result = [result];
        sort_phi(&mut result);
        let [result] = result;
        result
    }

    fn get_dst(s: &Statement) -> spirv::Word {
        let mut result = None;
        s.visit_id(&mut |is_dst, id| {
            if is_dst {
                assert_eq!(result.replace(id), None);
            }
        });
        result.unwrap()
    }

    fn get_label(s: &Statement) -> spirv::Word {
        match s {
            Statement::Label(id) => *id,
            _ => panic!(),
        }
    }

    fn get_dst_from_src(phi: &[PhiDef], src: spirv::Word) -> spirv::Word {
        for phi_set in phi {
            if phi_set.src.contains(&src) {
                return phi_set.dst;
            }
        }
        panic!()
    }

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
