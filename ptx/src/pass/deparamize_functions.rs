use super::*;

pub(super) fn run<'a, 'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    directives: Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>,
) -> Result<Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>, TranslateError> {
    directives
        .into_iter()
        .map(|directive| run_directive(resolver, directive))
        .collect::<Result<Vec<_>, _>>()
}

fn run_directive<'input>(
    resolver: &mut GlobalStringIdentResolver2,
    directive: Directive2<ast::Instruction<SpirvWord>, SpirvWord>,
) -> Result<Directive2<ast::Instruction<SpirvWord>, SpirvWord>, TranslateError> {
    Ok(match directive {
        var @ Directive2::Variable(..) => var,
        Directive2::Method(method) => Directive2::Method(run_method(resolver, method)?),
    })
}

fn run_method<'input>(
    resolver: &mut GlobalStringIdentResolver2,
    mut method: Function2<ast::Instruction<SpirvWord>, SpirvWord>,
) -> Result<Function2<ast::Instruction<SpirvWord>, SpirvWord>, TranslateError> {
    let is_declaration = method.body.is_none();
    let mut body = Vec::new();
    let mut remap_returns = Vec::new();
    if !method.is_kernel {
        for arg in method.return_arguments.iter_mut() {
            match arg.info.state_space {
                ptx_parser::StateSpace::Param => {
                    arg.info.state_space = ptx_parser::StateSpace::Reg;
                    let old_name = arg.name;
                    arg.name = resolver
                        .register_unnamed(Some((arg.info.v_type.clone(), arg.info.state_space)));
                    if is_declaration {
                        continue;
                    }
                    remap_returns.push((old_name, arg.name, arg.info.v_type.clone()));
                    body.push(Statement::Variable(ast::Variable {
                        info: ast::VariableInfo {
                            align: None,
                            v_type: arg.info.v_type.clone(),
                            state_space: ptx_parser::StateSpace::Param,
                            array_init: Vec::new(),
                        },
                        name: old_name,
                    }));
                }
                ptx_parser::StateSpace::Reg => {}
                _ => return Err(error_unreachable()),
            }
        }
        for arg in method.input_arguments.iter_mut() {
            match arg.info.state_space {
                ptx_parser::StateSpace::Param => {
                    arg.info.state_space = ptx_parser::StateSpace::Reg;
                    let old_name = arg.name;
                    arg.name = resolver
                        .register_unnamed(Some((arg.info.v_type.clone(), arg.info.state_space)));
                    if is_declaration {
                        continue;
                    }
                    body.push(Statement::Variable(ast::Variable {
                        info: ast::VariableInfo {
                            align: None,
                            v_type: arg.info.v_type.clone(),
                            state_space: ptx_parser::StateSpace::Param,
                            array_init: Vec::new(),
                        },
                        name: old_name,
                    }));
                    body.push(Statement::Instruction(ast::Instruction::St {
                        data: ast::StData {
                            qualifier: ast::LdStQualifier::Weak,
                            state_space: ast::StateSpace::Param,
                            caching: ast::StCacheOperator::Writethrough,
                            typ: arg.info.v_type.clone(),
                        },
                        arguments: ast::StArgs {
                            src1: old_name,
                            src2: arg.name,
                        },
                    }));
                }
                ptx_parser::StateSpace::Reg => {}
                _ => return Err(error_unreachable()),
            }
        }
    }
    let body = method
        .body
        .map(|statements| {
            for statement in statements {
                run_statement(resolver, &remap_returns, &mut body, statement)?;
            }
            Ok::<_, TranslateError>(body)
        })
        .transpose()?;
    Ok(Function2 { body, ..method })
}

fn run_statement<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    remap_returns: &Vec<(SpirvWord, SpirvWord, ast::Type)>,
    result: &mut Vec<Statement<ast::Instruction<SpirvWord>, SpirvWord>>,
    statement: Statement<ast::Instruction<SpirvWord>, SpirvWord>,
) -> Result<(), TranslateError> {
    match statement {
        Statement::Instruction(ast::Instruction::Call {
            mut data,
            mut arguments,
        }) => {
            let mut post_st = Vec::new();
            for ((type_, space), ident) in data
                .input_arguments
                .iter_mut()
                .zip(arguments.input_arguments.iter_mut())
            {
                if *space == ptx_parser::StateSpace::Param {
                    *space = ptx_parser::StateSpace::Reg;
                    let old_name = *ident;
                    *ident = resolver
                        .register_unnamed(Some((type_.clone(), ptx_parser::StateSpace::Reg)));
                    result.push(Statement::Instruction(ast::Instruction::Ld {
                        data: ast::LdDetails {
                            qualifier: ast::LdStQualifier::Weak,
                            state_space: ast::StateSpace::Param,
                            caching: ast::LdCacheOperator::Cached,
                            typ: type_.clone(),
                            non_coherent: false,
                        },
                        arguments: ast::LdArgs {
                            dst: *ident,
                            src: old_name,
                        },
                    }));
                }
            }
            for ((type_, space), ident) in data
                .return_arguments
                .iter_mut()
                .zip(arguments.return_arguments.iter_mut())
            {
                if *space == ptx_parser::StateSpace::Param {
                    *space = ptx_parser::StateSpace::Reg;
                    let old_name = *ident;
                    *ident = resolver
                        .register_unnamed(Some((type_.clone(), ptx_parser::StateSpace::Reg)));
                    post_st.push(Statement::Instruction(ast::Instruction::St {
                        data: ast::StData {
                            qualifier: ast::LdStQualifier::Weak,
                            state_space: ast::StateSpace::Param,
                            caching: ast::StCacheOperator::Writethrough,
                            typ: type_.clone(),
                        },
                        arguments: ast::StArgs {
                            src1: old_name,
                            src2: *ident,
                        },
                    }));
                }
            }
            result.push(Statement::Instruction(ast::Instruction::Call {
                data,
                arguments,
            }));
            result.extend(post_st.into_iter());
        }
        Statement::Instruction(ast::Instruction::Ret { data }) => {
            for (old_name, new_name, type_) in remap_returns.iter() {
                result.push(Statement::Instruction(ast::Instruction::Ld {
                    data: ast::LdDetails {
                        qualifier: ast::LdStQualifier::Weak,
                        state_space: ast::StateSpace::Param,
                        caching: ast::LdCacheOperator::Cached,
                        typ: type_.clone(),
                        non_coherent: false,
                    },
                    arguments: ast::LdArgs {
                        dst: *new_name,
                        src: *old_name,
                    },
                }));
            }
            result.push(Statement::Instruction(ast::Instruction::Ret { data }));
        }
        statement => {
            result.push(statement);
        }
    }
    Ok(())
}
