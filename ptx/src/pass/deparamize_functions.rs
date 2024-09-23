use std::collections::BTreeMap;

use super::*;

pub(super) fn run<'a, 'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    directives: Vec<Directive2<'input, ast::Instruction<SpirvWord>, SpirvWord>>,
) -> Result<Vec<Directive2<'input, ast::Instruction<SpirvWord>, SpirvWord>>, TranslateError> {
    directives
        .into_iter()
        .map(|directive| run_directive(resolver, directive))
        .collect::<Result<Vec<_>, _>>()
}

fn run_directive<'input>(
    resolver: &mut GlobalStringIdentResolver2,
    directive: Directive2<'input, ast::Instruction<SpirvWord>, SpirvWord>,
) -> Result<Directive2<'input, ast::Instruction<SpirvWord>, SpirvWord>, TranslateError> {
    Ok(match directive {
        var @ Directive2::Variable(..) => var,
        Directive2::Method(method) => Directive2::Method(run_method(resolver, method)?),
    })
}

fn run_method<'input>(
    resolver: &mut GlobalStringIdentResolver2,
    mut method: Function2<'input, ast::Instruction<SpirvWord>, SpirvWord>,
) -> Result<Function2<'input, ast::Instruction<SpirvWord>, SpirvWord>, TranslateError> {
    if method.func_decl.name.is_kernel() {
        return Ok(method);
    }
    let is_declaration = method.body.is_none();
    let mut body = Vec::new();
    let mut remap_returns = Vec::new();
    for arg in method.func_decl.return_arguments.iter_mut() {
        match arg.state_space {
            ptx_parser::StateSpace::Param => {
                arg.state_space = ptx_parser::StateSpace::Reg;
                let old_name = arg.name;
                arg.name = resolver.register_unnamed(Some((arg.v_type.clone(), arg.state_space)));
                if is_declaration {
                    continue;
                }
                remap_returns.push((old_name, arg.name, arg.v_type.clone()));
                body.push(Statement::Variable(ast::Variable {
                    align: None,
                    name: old_name,
                    v_type: arg.v_type.clone(),
                    state_space: ptx_parser::StateSpace::Param,
                    array_init: Vec::new(),
                }));
            }
            ptx_parser::StateSpace::Reg => {}
            _ => return Err(error_unreachable()),
        }
    }
    for arg in method.func_decl.input_arguments.iter_mut() {
        match arg.state_space {
            ptx_parser::StateSpace::Param => {
                arg.state_space = ptx_parser::StateSpace::Reg;
                let old_name = arg.name;
                arg.name = resolver.register_unnamed(Some((arg.v_type.clone(), arg.state_space)));
                if is_declaration {
                    continue;
                }
                body.push(Statement::Variable(ast::Variable {
                    align: None,
                    name: old_name,
                    v_type: arg.v_type.clone(),
                    state_space: ptx_parser::StateSpace::Param,
                    array_init: Vec::new(),
                }));
                body.push(Statement::Instruction(ast::Instruction::St {
                    data: ast::StData {
                        qualifier: ast::LdStQualifier::Weak,
                        state_space: ast::StateSpace::Param,
                        caching: ast::StCacheOperator::Writethrough,
                        typ: arg.v_type.clone(),
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
    if remap_returns.is_empty() {
        return Ok(method);
    }
    let body = method
        .body
        .map(|statements| {
            for statement in statements {
                run_statement(&remap_returns, &mut body, statement)?;
            }
            Ok::<_, TranslateError>(body)
        })
        .transpose()?;
    Ok(Function2 {
        func_decl: method.func_decl,
        globals: method.globals,
        body,
        import_as: method.import_as,
        tuning: method.tuning,
        linkage: method.linkage,
    })
}

fn run_statement<'input>(
    remap_returns: &Vec<(SpirvWord, SpirvWord, ast::Type)>,
    result: &mut Vec<Statement<ast::Instruction<SpirvWord>, SpirvWord>>,
    statement: Statement<ast::Instruction<SpirvWord>, SpirvWord>,
) -> Result<(), TranslateError> {
    match statement {
        Statement::Instruction(ast::Instruction::Ret { .. }) => {
            for (old_name, new_name, type_) in remap_returns.iter().cloned() {
                result.push(Statement::Instruction(ast::Instruction::Ld {
                    data: ast::LdDetails {
                        qualifier: ast::LdStQualifier::Weak,
                        state_space: ast::StateSpace::Reg,
                        caching: ast::LdCacheOperator::Cached,
                        typ: type_,
                        non_coherent: false,
                    },
                    arguments: ast::LdArgs {
                        dst: new_name,
                        src: old_name,
                    },
                }));
            }
            result.push(statement);
        }
        statement => {
            result.push(statement);
        }
    }
    Ok(())
}
