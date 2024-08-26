use super::*;
use ptx_parser as ast;

/*
    How do we handle arguments:
    - input .params in kernels
        .param .b64 in_arg
      get turned into this SPIR-V:
        %1 = OpFunctionParameter %ulong
        %2 = OpVariable %_ptr_Function_ulong Function
             OpStore %2 %1
      We do this for two reasons. One, common treatment for argument-declared
      .param variables and .param variables inside function (we assume that
      at SPIR-V level every .param is a pointer in Function storage class)
    - input .params in functions
        .param .b64 in_arg
      get turned into this SPIR-V:
        %1 = OpFunctionParameter %_ptr_Function_ulong
    - input .regs
        .reg .b64 in_arg
      get turned into the same SPIR-V as kernel .params:
        %1 = OpFunctionParameter %ulong
        %2 = OpVariable %_ptr_Function_ulong Function
             OpStore %2 %1
    - output .regs
        .reg .b64 out_arg
      get just a variable declaration:
        %2 = OpVariable %%_ptr_Function_ulong Function
    - output .params don't exist, they have been moved to input positions
      by an earlier pass
    Distinguishing betweem kernel .params and function .params is not the
    cleanest solution. Alternatively, we could "deparamize" all kernel .param
    arguments by turning them into .reg arguments like this:
        .param .b64 arg -> .reg ptr<.b64,.param> arg
    This has the massive downside that this transformation would have to run
    very early and would muddy up already difficult code. It's simpler to just
    have an if here
*/
pub(super) fn run<'a, 'b>(
    func: Vec<TypedStatement>,
    id_def: &mut NumericIdResolver,
    fn_decl: &'a mut ast::MethodDeclaration<'b, SpirvWord>,
) -> Result<Vec<TypedStatement>, TranslateError> {
    let mut result = Vec::with_capacity(func.len());
    for arg in fn_decl.input_arguments.iter_mut() {
        insert_mem_ssa_argument(
            id_def,
            &mut result,
            arg,
            matches!(fn_decl.name, ast::MethodName::Kernel(_)),
        );
    }
    for arg in fn_decl.return_arguments.iter() {
        insert_mem_ssa_argument_reg_return(&mut result, arg);
    }
    for s in func {
        match s {
            Statement::Instruction(inst) => match inst {
                ast::Instruction::Ret { data } => {
                    // TODO: handle multiple output args
                    match &fn_decl.return_arguments[..] {
                        [return_reg] => {
                            let new_id = id_def.register_intermediate(Some((
                                return_reg.v_type.clone(),
                                ast::StateSpace::Reg,
                            )));
                            result.push(Statement::LoadVar(LoadVarDetails {
                                arg: ast::LdArgs {
                                    dst: new_id,
                                    src: return_reg.name,
                                },
                                typ: return_reg.v_type.clone(),
                                member_index: None,
                            }));
                            result.push(Statement::RetValue(data, new_id));
                        }
                        [] => result.push(Statement::Instruction(ast::Instruction::Ret { data })),
                        _ => unimplemented!(),
                    }
                }
                inst => insert_mem_ssa_statement_default(
                    id_def,
                    &mut result,
                    Statement::Instruction(inst),
                )?,
            },
            Statement::Conditional(bra) => {
                insert_mem_ssa_statement_default(id_def, &mut result, Statement::Conditional(bra))?
            }
            Statement::Conversion(conv) => {
                insert_mem_ssa_statement_default(id_def, &mut result, Statement::Conversion(conv))?
            }
            Statement::PtrAccess(ptr_access) => insert_mem_ssa_statement_default(
                id_def,
                &mut result,
                Statement::PtrAccess(ptr_access),
            )?,
            Statement::RepackVector(repack) => insert_mem_ssa_statement_default(
                id_def,
                &mut result,
                Statement::RepackVector(repack),
            )?,
            Statement::FunctionPointer(func_ptr) => insert_mem_ssa_statement_default(
                id_def,
                &mut result,
                Statement::FunctionPointer(func_ptr),
            )?,
            s @ Statement::Variable(_) | s @ Statement::Label(_) | s @ Statement::Constant(..) => {
                result.push(s)
            }
            _ => return Err(error_unreachable()),
        }
    }
    Ok(result)
}

fn insert_mem_ssa_argument(
    id_def: &mut NumericIdResolver,
    func: &mut Vec<TypedStatement>,
    arg: &mut ast::Variable<SpirvWord>,
    is_kernel: bool,
) {
    if !is_kernel && arg.state_space == ast::StateSpace::Param {
        return;
    }
    let new_id = id_def.register_intermediate(Some((arg.v_type.clone(), arg.state_space)));
    func.push(Statement::Variable(ast::Variable {
        align: arg.align,
        v_type: arg.v_type.clone(),
        state_space: ast::StateSpace::Reg,
        name: arg.name,
        array_init: Vec::new(),
    }));
    func.push(Statement::StoreVar(StoreVarDetails {
        arg: ast::StArgs {
            src1: arg.name,
            src2: new_id,
        },
        typ: arg.v_type.clone(),
        member_index: None,
    }));
    arg.name = new_id;
}

fn insert_mem_ssa_argument_reg_return(
    func: &mut Vec<TypedStatement>,
    arg: &ast::Variable<SpirvWord>,
) {
    func.push(Statement::Variable(ast::Variable {
        align: arg.align,
        v_type: arg.v_type.clone(),
        state_space: arg.state_space,
        name: arg.name,
        array_init: arg.array_init.clone(),
    }));
}

fn insert_mem_ssa_statement_default<'a, 'input>(
    id_def: &'a mut NumericIdResolver<'input>,
    func: &'a mut Vec<TypedStatement>,
    stmt: TypedStatement,
) -> Result<(), TranslateError> {
    let mut visitor = InsertMemSSAVisitor {
        id_def,
        func,
        post_statements: Vec::new(),
    };
    let new_stmt = stmt.visit_map(&mut visitor)?;
    visitor.func.push(new_stmt);
    visitor.func.extend(visitor.post_statements);
    Ok(())
}

struct InsertMemSSAVisitor<'a, 'input> {
    id_def: &'a mut NumericIdResolver<'input>,
    func: &'a mut Vec<TypedStatement>,
    post_statements: Vec<TypedStatement>,
}

impl<'a, 'input> InsertMemSSAVisitor<'a, 'input> {
    fn symbol(
        &mut self,
        symbol: SpirvWord,
        member_index: Option<u8>,
        expected: Option<(&ast::Type, ast::StateSpace)>,
        is_dst: bool,
    ) -> Result<SpirvWord, TranslateError> {
        if expected.is_none() {
            return Ok(symbol);
        };
        let (mut var_type, var_space, is_variable) = self.id_def.get_typed(symbol)?;
        if !state_is_compatible(var_space, ast::StateSpace::Reg) || !is_variable {
            return Ok(symbol);
        };
        let member_index = match member_index {
            Some(idx) => {
                let vector_width = match var_type {
                    ast::Type::Vector(scalar_t, width) => {
                        var_type = ast::Type::Scalar(scalar_t);
                        width
                    }
                    _ => return Err(TranslateError::MismatchedType),
                };
                Some((
                    idx,
                    if self.id_def.special_registers.get(symbol).is_some() {
                        Some(vector_width)
                    } else {
                        None
                    },
                ))
            }
            None => None,
        };
        let generated_id = self
            .id_def
            .register_intermediate(Some((var_type.clone(), ast::StateSpace::Reg)));
        if !is_dst {
            self.func.push(Statement::LoadVar(LoadVarDetails {
                arg: ast::LdArgs {
                    dst: generated_id,
                    src: symbol,
                },
                typ: var_type,
                member_index,
            }));
        } else {
            self.post_statements
                .push(Statement::StoreVar(StoreVarDetails {
                    arg: ast::StArgs {
                        src1: symbol,
                        src2: generated_id,
                    },
                    typ: var_type,
                    member_index: member_index.map(|(idx, _)| idx),
                }));
        }
        Ok(generated_id)
    }
}

impl<'a, 'input> ast::VisitorMap<TypedOperand, TypedOperand, TranslateError>
    for InsertMemSSAVisitor<'a, 'input>
{
    fn visit(
        &mut self,
        operand: TypedOperand,
        type_space: Option<(&ast::Type, ast::StateSpace)>,
        is_dst: bool,
        _relaxed_type_check: bool,
    ) -> Result<TypedOperand, TranslateError> {
        Ok(match operand {
            TypedOperand::Reg(reg) => {
                TypedOperand::Reg(self.symbol(reg, None, type_space, is_dst)?)
            }
            TypedOperand::RegOffset(reg, offset) => {
                TypedOperand::RegOffset(self.symbol(reg, None, type_space, is_dst)?, offset)
            }
            op @ TypedOperand::Imm(..) => op,
            TypedOperand::VecMember(symbol, index) => TypedOperand::VecMember(
                self.symbol(symbol, Some(index), type_space, is_dst)?,
                index,
            ),
        })
    }

    fn visit_ident(
        &mut self,
        args: SpirvWord,
        type_space: Option<(&ptx_parser::Type, ptx_parser::StateSpace)>,
        is_dst: bool,
        relaxed_type_check: bool,
    ) -> Result<SpirvWord, TranslateError> {
        self.symbol(args, None, type_space, is_dst)
    }
}
