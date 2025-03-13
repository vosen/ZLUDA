use super::*;

// This pass normalizes ptx modules in two ways that makes mode computation pass
// and code emissions passes much simpler:
// * Inserts label at the start of every function
//   This makes control flow graph simpler in mode computation block: we can
//   represent kernels as separate nodes with its own separate entry/exit mode
// * Inserts label at the start of every basic block
// * Insert explicit jumps before labels
// * Non-.entry methods get a single `ret;` exit point - this is because mode computation
//   logic requires it. Control flow graph constructed by mode computation
//   models function calls as jumps into and then from another function.
//   If this cfg allowed multiple return basic blocks then there would be cases
//   where we want to insert mode setting instruction along the edge between
//   `ret;` and bb in the caller. This is only possible if there's a single
//   edge between from function `ret;` and caller
pub(crate) fn run(
    flat_resolver: &mut GlobalStringIdentResolver2<'_>,
    mut directives: Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>,
) -> Result<Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>, TranslateError> {
    for directive in directives.iter_mut() {
        let (body_ref, is_kernel) = match directive {
            Directive2::Method(Function2 {
                body: Some(body), is_kernel, ..
            }) => (body, *is_kernel),
            _ => continue,
        };
        let body = std::mem::replace(body_ref, Vec::new());
        let mut result = Vec::with_capacity(body.len());
        let mut previous_instruction_was_terminator = TerminatorKind::Not;
        let mut body_iterator = body.into_iter();
        let mut return_statements = Vec::new();
        match body_iterator.next() {
            Some(Statement::Label(_)) => {}
            Some(statement) => {
                result.push(Statement::Label(flat_resolver.register_unnamed(None)));
                result.push(statement);
            }
            None => {}
        }
        for statement in body_iterator {
            match previous_instruction_was_terminator {
                TerminatorKind::Not => match statement {
                    Statement::Label(label) => {
                        result.push(Statement::Instruction(ast::Instruction::Bra {
                            arguments: ast::BraArgs { src: label },
                        }))
                    }
                    _ => {}
                },
                TerminatorKind::Real => {
                    if !matches!(statement, Statement::Label(..)) {
                        result.push(Statement::Label(flat_resolver.register_unnamed(None)));
                    }
                }
                TerminatorKind::Fake => match statement {
                    // If there's a label after a call just reuse it
                    Statement::Label(label) => {
                        result.push(Statement::Instruction(ast::Instruction::Bra {
                            arguments: ast::BraArgs { src: label },
                        }))
                    }
                    _ => {
                        let label = flat_resolver.register_unnamed(None);
                        result.push(Statement::Instruction(ast::Instruction::Bra {
                            arguments: ast::BraArgs { src: label },
                        }));
                        result.push(Statement::Label(label));
                    }
                },
            }
            match statement {
                Statement::RetValue(..) => {
                    return Err(error_unreachable());
                }
                Statement::Instruction(ast::Instruction::Ret { .. }) => {
                    if !is_kernel {
                        return_statements.push(result.len());
                    }
                }
                _ => {}
            }
            previous_instruction_was_terminator = is_block_terminator(&statement);
            result.push(statement);
        }
        convert_from_multiple_returns_to_single_return(
            flat_resolver,
            &mut result,
            return_statements,
        )?;
        *body_ref = result;
    }
    Ok(directives)
}

enum TerminatorKind {
    Not,
    Real,
    Fake,
}

fn convert_from_multiple_returns_to_single_return(
    flat_resolver: &mut GlobalStringIdentResolver2<'_>,
    result: &mut Vec<Statement<ptx_parser::Instruction<SpirvWord>, SpirvWord>>,
    return_statements: Vec<usize>,
) -> Result<(), TranslateError> {
    Ok(if return_statements.len() > 1 {
        let ret_bb = flat_resolver.register_unnamed(None);
        result.push(Statement::Label(ret_bb));
        result.push(Statement::Instruction(ast::Instruction::Ret {
            data: ast::RetData { uniform: false },
        }));
        for ret_index in return_statements {
            let statement = result.get_mut(ret_index).ok_or_else(error_unreachable)?;
            *statement = Statement::Instruction(ast::Instruction::Bra {
                arguments: ast::BraArgs { src: ret_bb },
            });
        }
    })
}

fn is_block_terminator(
    statement: &Statement<ast::Instruction<SpirvWord>, SpirvWord>,
) -> TerminatorKind {
    match statement {
        Statement::Conditional(..)
        | Statement::Instruction(ast::Instruction::Bra { .. })
        // Normally call is not a terminator, but we treat it as such because it
        // makes the "instruction modes to global modes" pass possible
        | Statement::Instruction(ast::Instruction::Ret { .. }) => TerminatorKind::Real,
        Statement::Instruction(ast::Instruction::Call { .. }) => TerminatorKind::Fake,
        _ => TerminatorKind::Not,
    }
}
