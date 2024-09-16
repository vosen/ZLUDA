use super::*;
use ptx_parser as ast;

pub(crate) fn run<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    directives: Vec<NormalizedDirective2<'input>>,
) -> Result<Vec<UnconditionalDirective<'input>>, TranslateError> {
    directives
        .into_iter()
        .map(|directive| run_directive(resolver, directive))
        .collect::<Result<Vec<_>, _>>()
}

fn run_directive<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    directive: NormalizedDirective2<'input>,
) -> Result<UnconditionalDirective<'input>, TranslateError> {
    Ok(match directive {
        Directive2::Variable(linking, var) => Directive2::Variable(linking, var),
        Directive2::Method(method) => Directive2::Method(run_method(resolver, method)?),
    })
}

fn run_method<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    method: NormalizedFunction2<'input>,
) -> Result<UnconditionalFunction<'input>, TranslateError> {
    let body = method
        .body
        .map(|statements| {
            let mut result = Vec::with_capacity(statements.len());
            for statement in statements {
                run_statement(resolver, &mut result, statement)?;
            }
            Ok::<_, TranslateError>(result)
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
    resolver: &mut GlobalStringIdentResolver2<'input>,
    result: &mut Vec<UnconditionalStatement>,
    statement: NormalizedStatement,
) -> Result<(), TranslateError> {
    Ok(match statement {
        Statement::Label(label) => result.push(Statement::Label(label)),
        Statement::Variable(var) => result.push(Statement::Variable(var)),
        Statement::Instruction((predicate, instruction)) => {
            if let Some(pred) = predicate {
                let if_true = resolver.register_intermediate(None);
                let if_false = resolver.register_intermediate(None);
                let folded_bra = match &instruction {
                    ast::Instruction::Bra { arguments, .. } => Some(arguments.src),
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
                    result.push(Statement::Instruction(instruction));
                }
                result.push(Statement::Label(if_false));
            } else {
                result.push(Statement::Instruction(instruction));
            }
        }
        _ => return Err(error_unreachable()),
    })
}
