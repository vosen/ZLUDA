use super::*;
use ptx_parser as ast;
use std::mem;

// CUDA compiler generates function parameters as [b8], this is inefficient
// on AMDGPU, we instead relace all the parameters with [b32]
pub(crate) fn run<'a, 'input>(
    resolver: &'a mut GlobalStringIdentResolver2<'input>,
    mut directives: Vec<UnconditionalDirective>,
) -> Result<Vec<UnconditionalDirective>, TranslateError> {
    for d in directives.iter_mut() {
        run_directive(resolver, d)?;
    }
    Ok(directives)
}

fn run_directive<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    directive: &mut UnconditionalDirective,
) -> Result<(), TranslateError> {
    match directive {
        Directive2::Variable(..) => (),
        Directive2::Method(ref mut method) => run_method(resolver, method)?,
    }
    Ok(())
}

fn run_method<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    method: &mut UnconditionalFunction,
) -> Result<(), TranslateError> {
    if !method.is_kernel() {
        for argument in method
            .input_arguments
            .iter_mut()
            .chain(method.return_arguments.iter_mut())
        {
            run_on_param_variable(resolver, argument)?;
        }
    }
    for statement in method.body.iter_mut().flatten() {
        run_statement(resolver, statement)?;
    }
    Ok(())
}

fn run_statement<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    statement: &mut UnconditionalStatement,
) -> Result<(), TranslateError> {
    Ok(match statement {
        Statement::Variable(var) => {
            run_on_param_variable(resolver, var)?;
        }
        Statement::Instruction(ast::Instruction::Call { data, .. }) => {
            for argument in data
                .input_arguments
                .iter_mut()
                .chain(data.return_arguments.iter_mut())
            {
                if let (ast::Type::Array(None, ast::ScalarType::B8, dims), ast::StateSpace::Param) =
                    argument
                {
                    let dims = match &dims[..] {
                        [dim] => {
                            vec![dim.div_ceil(mem::size_of::<u32>() as u32)]
                        }
                        _ => {
                            continue;
                        }
                    };
                    let new_type = ast::Type::Array(None, ast::ScalarType::B32, dims);
                    argument.0 = new_type;
                }
            }
        }
        _ => {}
    })
}

fn run_on_param_variable<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    var: &mut ptx_parser::Variable<SpirvWord>,
) -> Result<(), TranslateError> {
    if var.info.state_space != ast::StateSpace::Param {
        return Ok(());
    }
    Ok(
        if let ast::Type::Array(None, ast::ScalarType::B8, ref dims) = var.info.v_type {
            let dims = match &dims[..] {
                [dim] => {
                    vec![dim.div_ceil(mem::size_of::<u32>() as u32)]
                }
                _ => {
                    return Ok(());
                }
            };
            let new_type = ast::Type::Array(None, ast::ScalarType::B32, dims);
            if let Some(entry) = resolver.ident_map.get_mut(&var.name) {
                if let Some((ref mut type_, _)) = entry.type_space {
                    *type_ = new_type.clone();
                }
            }
            var.info.v_type = new_type;
        },
    )
}
