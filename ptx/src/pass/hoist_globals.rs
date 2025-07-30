use super::*;

pub(super) fn run<'input>(
    directives: Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>,
) -> Result<Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>, TranslateError> {
    let mut result = Vec::with_capacity(directives.len());
    for mut directive in directives.into_iter() {
        run_directive(&mut result, &mut directive)?;
        result.push(directive);
    }
    Ok(result)
}

fn run_directive<'input>(
    result: &mut Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>,
    directive: &mut Directive2<ast::Instruction<SpirvWord>, SpirvWord>,
) -> Result<(), TranslateError> {
    match directive {
        Directive2::Variable(..) => {}
        Directive2::Method(function2) => run_function(result, function2),
    }
    Ok(())
}

fn run_function<'input>(
    result: &mut Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>,
    function: &mut Function2<ast::Instruction<SpirvWord>, SpirvWord>,
) {
    function.body = function.body.take().map(|statements| {
        statements
            .into_iter()
            .filter_map(|statement| match statement {
                Statement::Variable(var @ ast::Variable {
                    state_space:
                        ast::StateSpace::Global | ast::StateSpace::Const | ast::StateSpace::Shared,
                    ..
                }) => {
                    result.push(Directive2::Variable(ast::LinkingDirective::NONE, var));
                    None
                }
                s => Some(s),
            })
            .collect()
    });
}
