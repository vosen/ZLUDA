use std::mem;

use super::*;
use ptx_parser as ast;

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
pub(super) fn run(
    func: Vec<ExpandedStatement>,
    id_def: &mut MutableNumericIdResolver,
) -> Result<Vec<ExpandedStatement>, TranslateError> {
    let mut result = Vec::with_capacity(func.len());
    for s in func.into_iter() {
        match s {
            Statement::Instruction(inst) => {
                insert_implicit_conversions_impl(
                    &mut result,
                    id_def,
                    Statement::Instruction(inst),
                )?;
            }
            Statement::PtrAccess(access) => {
                insert_implicit_conversions_impl(
                    &mut result,
                    id_def,
                    Statement::PtrAccess(access),
                )?;
            }
            Statement::RepackVector(repack) => {
                insert_implicit_conversions_impl(
                    &mut result,
                    id_def,
                    Statement::RepackVector(repack),
                )?;
            }
            s @ Statement::Conditional(_)
            | s @ Statement::Conversion(_)
            | s @ Statement::Label(_)
            | s @ Statement::Constant(_)
            | s @ Statement::Variable(_)
            | s @ Statement::LoadVar(..)
            | s @ Statement::StoreVar(..)
            | s @ Statement::RetValue(..)
            | s @ Statement::FunctionPointer(..) => result.push(s),
        }
    }
    Ok(result)
}

fn insert_implicit_conversions_impl(
    func: &mut Vec<ExpandedStatement>,
    id_def: &mut MutableNumericIdResolver,
    stmt: ExpandedStatement,
) -> Result<(), TranslateError> {
    let mut post_conv = Vec::new();
    let statement = stmt.visit_map::<SpirvWord, TranslateError>(
        &mut |operand,
              type_state: Option<(&ast::Type, ast::StateSpace)>,
              is_dst,
              relaxed_type_check| {
            let (instr_type, instruction_space) = match type_state {
                None => return Ok(operand),
                Some(t) => t,
            };
            let (operand_type, operand_space) = id_def.get_typed(operand)?;
            let conversion_fn = if relaxed_type_check {
                if is_dst {
                    should_convert_relaxed_dst_wrapper
                } else {
                    should_convert_relaxed_src_wrapper
                }
            } else {
                default_implicit_conversion
            };
            match conversion_fn(
                (operand_space, &operand_type),
                (instruction_space, instr_type),
            )? {
                Some(conv_kind) => {
                    let conv_output = if is_dst { &mut post_conv } else { &mut *func };
                    let mut from_type = instr_type.clone();
                    let mut from_space = instruction_space;
                    let mut to_type = operand_type;
                    let mut to_space = operand_space;
                    let mut src =
                        id_def.register_intermediate(instr_type.clone(), instruction_space);
                    let mut dst = operand;
                    let result = Ok::<_, TranslateError>(src);
                    if !is_dst {
                        mem::swap(&mut src, &mut dst);
                        mem::swap(&mut from_type, &mut to_type);
                        mem::swap(&mut from_space, &mut to_space);
                    }
                    conv_output.push(Statement::Conversion(ImplicitConversion {
                        src,
                        dst,
                        from_type,
                        from_space,
                        to_type,
                        to_space,
                        kind: conv_kind,
                    }));
                    result
                }
                None => Ok(operand),
            }
        },
    )?;
    func.push(statement);
    func.append(&mut post_conv);
    Ok(())
}

pub(crate) fn default_implicit_conversion(
    (operand_space, operand_type): (ast::StateSpace, &ast::Type),
    (instruction_space, instruction_type): (ast::StateSpace, &ast::Type),
) -> Result<Option<ConversionKind>, TranslateError> {
    if instruction_space == ast::StateSpace::Reg {
        if space_is_compatible(operand_space, ast::StateSpace::Reg) {
            if let (ast::Type::Vector(vec_len, vec_underlying_type), ast::Type::Scalar(scalar)) =
                (operand_type, instruction_type)
            {
                if scalar.kind() == ast::ScalarKind::Bit
                    && scalar.size_of() == (vec_underlying_type.size_of() * vec_len)
                {
                    return Ok(Some(ConversionKind::Default));
                }
            }
        } else if is_addressable(operand_space) {
            return Ok(Some(ConversionKind::AddressOf));
        }
    }
    if !space_is_compatible(instruction_space, operand_space) {
        default_implicit_conversion_space(
            (operand_space, operand_type),
            (instruction_space, instruction_type),
        )
    } else if instruction_type != operand_type {
        default_implicit_conversion_type(instruction_space, operand_type, instruction_type)
    } else {
        Ok(None)
    }
}

fn is_addressable(this: ast::StateSpace) -> bool {
    match this {
        ast::StateSpace::Const
        | ast::StateSpace::Generic
        | ast::StateSpace::Global
        | ast::StateSpace::Local
        | ast::StateSpace::Shared => true,
        ast::StateSpace::Param | ast::StateSpace::Reg | ast::StateSpace::Sreg => false,
        ast::StateSpace::SharedCluster
        | ast::StateSpace::SharedCta
        | ast::StateSpace::ParamEntry
        | ast::StateSpace::ParamFunc => todo!(),
    }
}

// Space is different
fn default_implicit_conversion_space(
    (operand_space, operand_type): (ast::StateSpace, &ast::Type),
    (instruction_space, instruction_type): (ast::StateSpace, &ast::Type),
) -> Result<Option<ConversionKind>, TranslateError> {
    if (instruction_space == ast::StateSpace::Generic && coerces_to_generic(operand_space))
        || (operand_space == ast::StateSpace::Generic && coerces_to_generic(instruction_space))
    {
        Ok(Some(ConversionKind::PtrToPtr))
    } else if space_is_compatible(operand_space, ast::StateSpace::Reg) {
        match operand_type {
            ast::Type::Pointer(operand_ptr_type, operand_ptr_space)
                if *operand_ptr_space == instruction_space =>
            {
                if instruction_type != &ast::Type::Scalar(*operand_ptr_type) {
                    Ok(Some(ConversionKind::PtrToPtr))
                } else {
                    Ok(None)
                }
            }
            // TODO: 32 bit
            ast::Type::Scalar(ast::ScalarType::B64)
            | ast::Type::Scalar(ast::ScalarType::U64)
            | ast::Type::Scalar(ast::ScalarType::S64) => match instruction_space {
                ast::StateSpace::Global
                | ast::StateSpace::Generic
                | ast::StateSpace::Const
                | ast::StateSpace::Local
                | ast::StateSpace::Shared => Ok(Some(ConversionKind::BitToPtr)),
                _ => Err(error_mismatched_type()),
            },
            ast::Type::Scalar(ast::ScalarType::B32)
            | ast::Type::Scalar(ast::ScalarType::U32)
            | ast::Type::Scalar(ast::ScalarType::S32) => match instruction_space {
                ast::StateSpace::Const | ast::StateSpace::Local | ast::StateSpace::Shared => {
                    Ok(Some(ConversionKind::BitToPtr))
                }
                _ => Err(error_mismatched_type()),
            },
            _ => Err(error_mismatched_type()),
        }
    } else if space_is_compatible(instruction_space, ast::StateSpace::Reg) {
        match instruction_type {
            ast::Type::Pointer(instruction_ptr_type, instruction_ptr_space)
                if operand_space == *instruction_ptr_space =>
            {
                if operand_type != &ast::Type::Scalar(*instruction_ptr_type) {
                    Ok(Some(ConversionKind::PtrToPtr))
                } else {
                    Ok(None)
                }
            }
            _ => Err(error_mismatched_type()),
        }
    } else {
        Err(error_mismatched_type())
    }
}

// Space is same, but type is different
fn default_implicit_conversion_type(
    space: ast::StateSpace,
    operand_type: &ast::Type,
    instruction_type: &ast::Type,
) -> Result<Option<ConversionKind>, TranslateError> {
    if space_is_compatible(space, ast::StateSpace::Reg) {
        if should_bitcast(instruction_type, operand_type) {
            Ok(Some(ConversionKind::Default))
        } else {
            Err(error_mismatched_type())
        }
    } else {
        Ok(Some(ConversionKind::PtrToPtr))
    }
}

fn coerces_to_generic(this: ast::StateSpace) -> bool {
    match this {
        ast::StateSpace::Global
        | ast::StateSpace::Const
        | ast::StateSpace::Local
        | ptx_parser::StateSpace::SharedCta
        | ast::StateSpace::SharedCluster
        | ast::StateSpace::Shared => true,
        ast::StateSpace::Reg
        | ast::StateSpace::Param
        | ast::StateSpace::ParamEntry
        | ast::StateSpace::ParamFunc
        | ast::StateSpace::Generic
        | ast::StateSpace::Sreg => false,
    }
}

fn should_bitcast(instr: &ast::Type, operand: &ast::Type) -> bool {
    match (instr, operand) {
        (ast::Type::Scalar(inst), ast::Type::Scalar(operand)) => {
            if inst.size_of() != operand.size_of() {
                return false;
            }
            match inst.kind() {
                ast::ScalarKind::Bit => operand.kind() != ast::ScalarKind::Bit,
                ast::ScalarKind::Float => operand.kind() == ast::ScalarKind::Bit,
                ast::ScalarKind::Signed => {
                    operand.kind() == ast::ScalarKind::Bit
                        || operand.kind() == ast::ScalarKind::Unsigned
                }
                ast::ScalarKind::Unsigned => {
                    operand.kind() == ast::ScalarKind::Bit
                        || operand.kind() == ast::ScalarKind::Signed
                }
                ast::ScalarKind::Pred => false,
            }
        }
        (ast::Type::Vector(_, inst), ast::Type::Vector(_, operand))
        | (ast::Type::Array(_, inst, _), ast::Type::Array(_, operand, _)) => {
            should_bitcast(&ast::Type::Scalar(*inst), &ast::Type::Scalar(*operand))
        }
        _ => false,
    }
}

pub(crate) fn should_convert_relaxed_dst_wrapper(
    (operand_space, operand_type): (ast::StateSpace, &ast::Type),
    (instruction_space, instruction_type): (ast::StateSpace, &ast::Type),
) -> Result<Option<ConversionKind>, TranslateError> {
    if !space_is_compatible(operand_space, instruction_space) {
        return Err(error_mismatched_type());
    }
    if operand_type == instruction_type {
        return Ok(None);
    }
    match should_convert_relaxed_dst(operand_type, instruction_type) {
        conv @ Some(_) => Ok(conv),
        None => Err(error_mismatched_type()),
    }
}

// https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#operand-size-exceeding-instruction-type-size__relaxed-type-checking-rules-destination-operands
fn should_convert_relaxed_dst(
    dst_type: &ast::Type,
    instr_type: &ast::Type,
) -> Option<ConversionKind> {
    if dst_type == instr_type {
        return None;
    }
    match (dst_type, instr_type) {
        (ast::Type::Scalar(dst_type), ast::Type::Scalar(instr_type)) => match instr_type.kind() {
            ast::ScalarKind::Bit => {
                if instr_type.size_of() <= dst_type.size_of() {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
            ast::ScalarKind::Signed => {
                if dst_type.kind() != ast::ScalarKind::Float {
                    if instr_type.size_of() == dst_type.size_of() {
                        Some(ConversionKind::Default)
                    } else if instr_type.size_of() < dst_type.size_of() {
                        Some(ConversionKind::SignExtend)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            ast::ScalarKind::Unsigned => {
                if instr_type.size_of() <= dst_type.size_of()
                    && dst_type.kind() != ast::ScalarKind::Float
                {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
            ast::ScalarKind::Float => {
                if instr_type.size_of() <= dst_type.size_of()
                    && dst_type.kind() == ast::ScalarKind::Bit
                {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
            ast::ScalarKind::Pred => None,
        },
        (ast::Type::Vector(_, dst_type), ast::Type::Vector(_, instr_type))
        | (ast::Type::Array(_, dst_type, _), ast::Type::Array(_, instr_type, _)) => {
            should_convert_relaxed_dst(
                &ast::Type::Scalar(*dst_type),
                &ast::Type::Scalar(*instr_type),
            )
        }
        _ => None,
    }
}

pub(crate) fn should_convert_relaxed_src_wrapper(
    (operand_space, operand_type): (ast::StateSpace, &ast::Type),
    (instruction_space, instruction_type): (ast::StateSpace, &ast::Type),
) -> Result<Option<ConversionKind>, TranslateError> {
    if !space_is_compatible(operand_space, instruction_space) {
        return Err(error_mismatched_type());
    }
    if operand_type == instruction_type {
        return Ok(None);
    }
    match should_convert_relaxed_src(operand_type, instruction_type) {
        conv @ Some(_) => Ok(conv),
        None => Err(error_mismatched_type()),
    }
}

// https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#operand-size-exceeding-instruction-type-size__relaxed-type-checking-rules-source-operands
fn should_convert_relaxed_src(
    src_type: &ast::Type,
    instr_type: &ast::Type,
) -> Option<ConversionKind> {
    if src_type == instr_type {
        return None;
    }
    match (src_type, instr_type) {
        (ast::Type::Scalar(src_type), ast::Type::Scalar(instr_type)) => match instr_type.kind() {
            ast::ScalarKind::Bit => {
                if instr_type.size_of() <= src_type.size_of() {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
            ast::ScalarKind::Signed | ast::ScalarKind::Unsigned => {
                if instr_type.size_of() <= src_type.size_of()
                    && src_type.kind() != ast::ScalarKind::Float
                {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
            ast::ScalarKind::Float => {
                if instr_type.size_of() <= src_type.size_of()
                    && src_type.kind() == ast::ScalarKind::Bit
                {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
            ast::ScalarKind::Pred => None,
        },
        (ast::Type::Vector(_, dst_type), ast::Type::Vector(_, instr_type))
        | (ast::Type::Array(_, dst_type, _), ast::Type::Array(_, instr_type, _)) => {
            should_convert_relaxed_src(
                &ast::Type::Scalar(*dst_type),
                &ast::Type::Scalar(*instr_type),
            )
        }
        _ => None,
    }
}
