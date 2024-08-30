use super::*;
use ptx_parser as ast;
use std::{
    collections::{BTreeSet, HashSet},
    iter,
    rc::Rc,
};

/*
    Our goal here is to transform
        .visible .entry foobar(.param .u64 input) {
            .reg .b64 in_addr;
            .reg .b64 in_addr2;
            ld.param.u64 in_addr, [input];
            cvta.to.global.u64  in_addr2, in_addr;
        }
    into:
        .visible .entry foobar(.param .u8 input[]) {
            .reg .u8 in_addr[];
            .reg .u8 in_addr2[];
            ld.param.u8[] in_addr, [input];
            mov.u8[] in_addr2, in_addr;
        }
    or:
        .visible .entry foobar(.reg .u8 input[]) {
            .reg .u8 in_addr[];
            .reg .u8 in_addr2[];
            mov.u8[] in_addr, input;
            mov.u8[] in_addr2, in_addr;
        }
    or:
        .visible .entry foobar(.param ptr<u8, global> input) {
            .reg ptr<u8, global> in_addr;
            .reg ptr<u8, global> in_addr2;
            ld.param.ptr<u8, global> in_addr, [input];
            mov.ptr<u8, global> in_addr2, in_addr;
        }
*/
// TODO: detect more patterns (mov, call via reg, call via param)
// TODO: don't convert to ptr if the register is not ultimately used for ld/st
// TODO: once insert_mem_ssa_statements is moved to later, move this pass after
//       argument expansion
// TODO: propagate out of calls and into calls
pub(super) fn run<'a, 'input>(
    func_args: Rc<RefCell<ast::MethodDeclaration<'input, SpirvWord>>>,
    func_body: Vec<TypedStatement>,
    id_defs: &mut NumericIdResolver<'a>,
) -> Result<
    (
        Rc<RefCell<ast::MethodDeclaration<'input, SpirvWord>>>,
        Vec<TypedStatement>,
    ),
    TranslateError,
> {
    let mut method_decl = func_args.borrow_mut();
    if !matches!(method_decl.name, ast::MethodName::Kernel(..)) {
        drop(method_decl);
        return Ok((func_args, func_body));
    }
    if Rc::strong_count(&func_args) != 1 {
        return Err(error_unreachable());
    }
    let func_args_64bit = (*method_decl)
        .input_arguments
        .iter()
        .filter_map(|arg| match arg.v_type {
            ast::Type::Scalar(ast::ScalarType::U64)
            | ast::Type::Scalar(ast::ScalarType::B64)
            | ast::Type::Scalar(ast::ScalarType::S64) => Some(arg.name),
            _ => None,
        })
        .collect::<HashSet<_>>();
    let mut stateful_markers = Vec::new();
    let mut stateful_init_reg = HashMap::<_, Vec<_>>::new();
    for statement in func_body.iter() {
        match statement {
            Statement::Instruction(ast::Instruction::Cvta {
                data:
                    ast::CvtaDetails {
                        state_space: ast::StateSpace::Global,
                        direction: ast::CvtaDirection::GenericToExplicit,
                    },
                arguments,
            }) => {
                if let (TypedOperand::Reg(dst), Some(src)) =
                    (arguments.dst, arguments.src.underlying_register())
                {
                    if is_64_bit_integer(id_defs, src) && is_64_bit_integer(id_defs, dst) {
                        stateful_markers.push((dst, src));
                    }
                }
            }
            Statement::Instruction(ast::Instruction::Ld {
                data:
                    ast::LdDetails {
                        state_space: ast::StateSpace::Param,
                        typ: ast::Type::Scalar(ast::ScalarType::U64),
                        ..
                    },
                arguments,
            })
            | Statement::Instruction(ast::Instruction::Ld {
                data:
                    ast::LdDetails {
                        state_space: ast::StateSpace::Param,
                        typ: ast::Type::Scalar(ast::ScalarType::S64),
                        ..
                    },
                arguments,
            })
            | Statement::Instruction(ast::Instruction::Ld {
                data:
                    ast::LdDetails {
                        state_space: ast::StateSpace::Param,
                        typ: ast::Type::Scalar(ast::ScalarType::B64),
                        ..
                    },
                arguments,
            }) => {
                if let (TypedOperand::Reg(dst), Some(src)) =
                    (arguments.dst, arguments.src.underlying_register())
                {
                    if func_args_64bit.contains(&src) {
                        multi_hash_map_append(&mut stateful_init_reg, dst, src);
                    }
                }
            }
            _ => {}
        }
    }
    if stateful_markers.len() == 0 {
        drop(method_decl);
        return Ok((func_args, func_body));
    }
    let mut func_args_ptr = HashSet::new();
    let mut regs_ptr_current = HashSet::new();
    for (dst, src) in stateful_markers {
        if let Some(func_args) = stateful_init_reg.get(&src) {
            for a in func_args {
                func_args_ptr.insert(*a);
                regs_ptr_current.insert(src);
                regs_ptr_current.insert(dst);
            }
        }
    }
    // BTreeSet here to have a stable order of iteration,
    // unfortunately our tests rely on it
    let mut regs_ptr_seen = BTreeSet::new();
    while regs_ptr_current.len() > 0 {
        let mut regs_ptr_new = HashSet::new();
        for statement in func_body.iter() {
            match statement {
                Statement::Instruction(ast::Instruction::Add {
                    data:
                        ast::ArithDetails::Integer(ast::ArithInteger {
                            type_: ast::ScalarType::U64,
                            saturate: false,
                        }),
                    arguments,
                })
                | Statement::Instruction(ast::Instruction::Add {
                    data:
                        ast::ArithDetails::Integer(ast::ArithInteger {
                            type_: ast::ScalarType::S64,
                            saturate: false,
                        }),
                    arguments,
                }) => {
                    // TODO: don't mark result of double pointer sub or double
                    // pointer add as ptr result
                    if let (TypedOperand::Reg(dst), Some(src1)) =
                        (arguments.dst, arguments.src1.underlying_register())
                    {
                        if regs_ptr_current.contains(&src1) && !regs_ptr_seen.contains(&src1) {
                            regs_ptr_new.insert(dst);
                        }
                    } else if let (TypedOperand::Reg(dst), Some(src2)) =
                        (arguments.dst, arguments.src2.underlying_register())
                    {
                        if regs_ptr_current.contains(&src2) && !regs_ptr_seen.contains(&src2) {
                            regs_ptr_new.insert(dst);
                        }
                    }
                }

                Statement::Instruction(ast::Instruction::Sub {
                    data:
                        ast::ArithDetails::Integer(ast::ArithInteger {
                            type_: ast::ScalarType::U64,
                            saturate: false,
                        }),
                    arguments,
                })
                | Statement::Instruction(ast::Instruction::Sub {
                    data:
                        ast::ArithDetails::Integer(ast::ArithInteger {
                            type_: ast::ScalarType::S64,
                            saturate: false,
                        }),
                    arguments,
                }) => {
                    // TODO: don't mark result of double pointer sub or double
                    // pointer add as ptr result
                    if let (TypedOperand::Reg(dst), Some(src1)) =
                        (arguments.dst, arguments.src1.underlying_register())
                    {
                        if regs_ptr_current.contains(&src1) && !regs_ptr_seen.contains(&src1) {
                            regs_ptr_new.insert(dst);
                        }
                    } else if let (TypedOperand::Reg(dst), Some(src2)) =
                        (arguments.dst, arguments.src2.underlying_register())
                    {
                        if regs_ptr_current.contains(&src2) && !regs_ptr_seen.contains(&src2) {
                            regs_ptr_new.insert(dst);
                        }
                    }
                }
                _ => {}
            }
        }
        for id in regs_ptr_current {
            regs_ptr_seen.insert(id);
        }
        regs_ptr_current = regs_ptr_new;
    }
    drop(regs_ptr_current);
    let mut remapped_ids = HashMap::new();
    let mut result = Vec::with_capacity(regs_ptr_seen.len() + func_body.len());
    for reg in regs_ptr_seen {
        let new_id = id_defs.register_variable(
            ast::Type::Pointer(ast::ScalarType::U8, ast::StateSpace::Global),
            ast::StateSpace::Reg,
        );
        result.push(Statement::Variable(ast::Variable {
            align: None,
            name: new_id,
            array_init: Vec::new(),
            v_type: ast::Type::Pointer(ast::ScalarType::U8, ast::StateSpace::Global),
            state_space: ast::StateSpace::Reg,
        }));
        remapped_ids.insert(reg, new_id);
    }
    for arg in (*method_decl).input_arguments.iter_mut() {
        if !func_args_ptr.contains(&arg.name) {
            continue;
        }
        let new_id = id_defs.register_variable(
            ast::Type::Pointer(ast::ScalarType::U8, ast::StateSpace::Global),
            ast::StateSpace::Param,
        );
        let old_name = arg.name;
        arg.v_type = ast::Type::Pointer(ast::ScalarType::U8, ast::StateSpace::Global);
        arg.name = new_id;
        remapped_ids.insert(old_name, new_id);
    }
    for statement in func_body {
        match statement {
            l @ Statement::Label(_) => result.push(l),
            c @ Statement::Conditional(_) => result.push(c),
            c @ Statement::Constant(..) => result.push(c),
            Statement::Variable(var) => {
                if !remapped_ids.contains_key(&var.name) {
                    result.push(Statement::Variable(var));
                }
            }
            Statement::Instruction(ast::Instruction::Add {
                data:
                    ast::ArithDetails::Integer(ast::ArithInteger {
                        type_: ast::ScalarType::U64,
                        saturate: false,
                    }),
                arguments,
            })
            | Statement::Instruction(ast::Instruction::Add {
                data:
                    ast::ArithDetails::Integer(ast::ArithInteger {
                        type_: ast::ScalarType::S64,
                        saturate: false,
                    }),
                arguments,
            }) if is_add_ptr_direct(&remapped_ids, &arguments) => {
                let (ptr, offset) = match arguments.src1.underlying_register() {
                    Some(src1) if remapped_ids.contains_key(&src1) => {
                        (remapped_ids.get(&src1).unwrap(), arguments.src2)
                    }
                    Some(src2) if remapped_ids.contains_key(&src2) => {
                        (remapped_ids.get(&src2).unwrap(), arguments.src1)
                    }
                    _ => return Err(error_unreachable()),
                };
                let dst = arguments.dst.unwrap_reg()?;
                result.push(Statement::PtrAccess(PtrAccess {
                    underlying_type: ast::Type::Scalar(ast::ScalarType::U8),
                    state_space: ast::StateSpace::Global,
                    dst: *remapped_ids.get(&dst).unwrap(),
                    ptr_src: *ptr,
                    offset_src: offset,
                }))
            }
            Statement::Instruction(ast::Instruction::Sub {
                data:
                    ast::ArithDetails::Integer(ast::ArithInteger {
                        type_: ast::ScalarType::U64,
                        saturate: false,
                    }),
                arguments,
            })
            | Statement::Instruction(ast::Instruction::Sub {
                data:
                    ast::ArithDetails::Integer(ast::ArithInteger {
                        type_: ast::ScalarType::S64,
                        saturate: false,
                    }),
                arguments,
            }) if is_sub_ptr_direct(&remapped_ids, &arguments) => {
                let (ptr, offset) = match arguments.src1.underlying_register() {
                    Some(ref src1) => (remapped_ids.get(src1).unwrap(), arguments.src2),
                    _ => return Err(error_unreachable()),
                };
                let offset_neg = id_defs.register_intermediate(Some((
                    ast::Type::Scalar(ast::ScalarType::S64),
                    ast::StateSpace::Reg,
                )));
                result.push(Statement::Instruction(ast::Instruction::Neg {
                    data: ast::TypeFtz {
                        type_: ast::ScalarType::S64,
                        flush_to_zero: None,
                    },
                    arguments: ast::NegArgs {
                        src: offset,
                        dst: TypedOperand::Reg(offset_neg),
                    },
                }));
                let dst = arguments.dst.unwrap_reg()?;
                result.push(Statement::PtrAccess(PtrAccess {
                    underlying_type: ast::Type::Scalar(ast::ScalarType::U8),
                    state_space: ast::StateSpace::Global,
                    dst: *remapped_ids.get(&dst).unwrap(),
                    ptr_src: *ptr,
                    offset_src: TypedOperand::Reg(offset_neg),
                }))
            }
            inst @ Statement::Instruction(_) => {
                let mut post_statements = Vec::new();
                let new_statement = inst.visit_map(&mut FnVisitor::new(
                    |operand, type_space, is_dst, relaxed_conversion| {
                        convert_to_stateful_memory_access_postprocess(
                            id_defs,
                            &remapped_ids,
                            &mut result,
                            &mut post_statements,
                            operand,
                            type_space,
                            is_dst,
                            relaxed_conversion,
                        )
                    },
                ))?;
                result.push(new_statement);
                result.extend(post_statements);
            }
            repack @ Statement::RepackVector(_) => {
                let mut post_statements = Vec::new();
                let new_statement = repack.visit_map(&mut FnVisitor::new(
                    |operand, type_space, is_dst, relaxed_conversion| {
                        convert_to_stateful_memory_access_postprocess(
                            id_defs,
                            &remapped_ids,
                            &mut result,
                            &mut post_statements,
                            operand,
                            type_space,
                            is_dst,
                            relaxed_conversion,
                        )
                    },
                ))?;
                result.push(new_statement);
                result.extend(post_statements);
            }
            _ => return Err(error_unreachable()),
        }
    }
    drop(method_decl);
    Ok((func_args, result))
}

fn is_64_bit_integer(id_defs: &NumericIdResolver, id: SpirvWord) -> bool {
    match id_defs.get_typed(id) {
        Ok((ast::Type::Scalar(ast::ScalarType::U64), _, _))
        | Ok((ast::Type::Scalar(ast::ScalarType::S64), _, _))
        | Ok((ast::Type::Scalar(ast::ScalarType::B64), _, _)) => true,
        _ => false,
    }
}

fn is_add_ptr_direct(
    remapped_ids: &HashMap<SpirvWord, SpirvWord>,
    arg: &ast::AddArgs<TypedOperand>,
) -> bool {
    match arg.dst {
        TypedOperand::Imm(..) | TypedOperand::RegOffset(..) | TypedOperand::VecMember(..) => {
            return false
        }
        TypedOperand::Reg(dst) => {
            if !remapped_ids.contains_key(&dst) {
                return false;
            }
            if let Some(ref src1_reg) = arg.src1.underlying_register() {
                if remapped_ids.contains_key(src1_reg) {
                    // don't trigger optimization when adding two pointers
                    if let Some(ref src2_reg) = arg.src2.underlying_register() {
                        return !remapped_ids.contains_key(src2_reg);
                    }
                }
            }
            if let Some(ref src2_reg) = arg.src2.underlying_register() {
                remapped_ids.contains_key(src2_reg)
            } else {
                false
            }
        }
    }
}

fn is_sub_ptr_direct(
    remapped_ids: &HashMap<SpirvWord, SpirvWord>,
    arg: &ast::SubArgs<TypedOperand>,
) -> bool {
    match arg.dst {
        TypedOperand::Imm(..) | TypedOperand::RegOffset(..) | TypedOperand::VecMember(..) => {
            return false
        }
        TypedOperand::Reg(dst) => {
            if !remapped_ids.contains_key(&dst) {
                return false;
            }
            match arg.src1.underlying_register() {
                Some(ref src1_reg) => {
                    if remapped_ids.contains_key(src1_reg) {
                        // don't trigger optimization when subtracting two pointers
                        arg.src2
                            .underlying_register()
                            .map_or(true, |ref src2_reg| !remapped_ids.contains_key(src2_reg))
                    } else {
                        false
                    }
                }
                None => false,
            }
        }
    }
}

fn convert_to_stateful_memory_access_postprocess(
    id_defs: &mut NumericIdResolver,
    remapped_ids: &HashMap<SpirvWord, SpirvWord>,
    result: &mut Vec<TypedStatement>,
    post_statements: &mut Vec<TypedStatement>,
    operand: TypedOperand,
    type_space: Option<(&ast::Type, ast::StateSpace)>,
    is_dst: bool,
    relaxed_conversion: bool,
) -> Result<TypedOperand, TranslateError> {
    operand.map(|operand, _| {
        Ok(match remapped_ids.get(&operand) {
            Some(new_id) => {
                let (new_operand_type, new_operand_space, _) = id_defs.get_typed(*new_id)?;
                // TODO: readd if required
                if let Some(..) = type_space {
                    if relaxed_conversion {
                        return Ok(*new_id);
                    }
                }
                let (old_operand_type, old_operand_space, _) = id_defs.get_typed(operand)?;
                let converting_id = id_defs
                    .register_intermediate(Some((old_operand_type.clone(), old_operand_space)));
                let kind = if space_is_compatible(new_operand_space, ast::StateSpace::Reg) {
                    ConversionKind::Default
                } else {
                    ConversionKind::PtrToPtr
                };
                if is_dst {
                    post_statements.push(Statement::Conversion(ImplicitConversion {
                        src: converting_id,
                        dst: *new_id,
                        from_type: old_operand_type,
                        from_space: old_operand_space,
                        to_type: new_operand_type,
                        to_space: new_operand_space,
                        kind,
                    }));
                    converting_id
                } else {
                    result.push(Statement::Conversion(ImplicitConversion {
                        src: *new_id,
                        dst: converting_id,
                        from_type: new_operand_type,
                        from_space: new_operand_space,
                        to_type: old_operand_type,
                        to_space: old_operand_space,
                        kind,
                    }));
                    converting_id
                }
            }
            None => operand,
        })
    })
}
