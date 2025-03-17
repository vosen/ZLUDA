use super::*;
use ptx_parser as ast;

pub(crate) fn run<'input, 'b>(
    resolver: &mut ScopedResolver<'input, 'b>,
    directives: Vec<ast::Directive<'input, ast::ParsedOperand<&'input str>>>,
) -> Result<Vec<NormalizedDirective2>, TranslateError> {
    resolver.start_scope();
    let result = directives
        .into_iter()
        .map(|directive| run_directive(resolver, directive))
        .collect::<Result<Vec<_>, _>>()?;
    resolver.end_scope();
    Ok(result)
}

fn run_directive<'input, 'b>(
    resolver: &mut ScopedResolver<'input, 'b>,
    directive: ast::Directive<'input, ast::ParsedOperand<&'input str>>,
) -> Result<NormalizedDirective2, TranslateError> {
    Ok(match directive {
        ast::Directive::Variable(linking, var) => {
            NormalizedDirective2::Variable(linking, run_variable(resolver, var)?)
        }
        ast::Directive::Method(linking, directive) => {
            NormalizedDirective2::Method(run_method(resolver, linking, directive)?)
        }
    })
}

fn run_method<'input, 'b>(
    resolver: &mut ScopedResolver<'input, 'b>,
    linkage: ast::LinkingDirective,
    method: ast::Function<'input, &'input str, ast::Statement<ast::ParsedOperand<&'input str>>>,
) -> Result<NormalizedFunction2, TranslateError> {
    let is_kernel = method.func_directive.name.is_kernel();
    let name = resolver.add_or_get_in_current_scope_untyped(method.func_directive.name.text())?;
    resolver.start_scope();
    let (return_arguments, input_arguments) = run_function_decl(resolver, method.func_directive)?;
    let body = method
        .body
        .map(|statements| {
            let mut result = Vec::with_capacity(statements.len());
            run_statements(resolver, &mut result, statements)?;
            Ok::<_, TranslateError>(result)
        })
        .transpose()?;
    resolver.end_scope();
    Ok(Function2 {
        return_arguments,
        name,
        input_arguments,
        body,
        import_as: None,
        linkage,
        is_kernel,
        tuning: method.tuning,
        flush_to_zero_f32: false,
        flush_to_zero_f16f64: false,
        rounding_mode_f32: ptx_parser::RoundingMode::NearestEven,
        rounding_mode_f16f64: ptx_parser::RoundingMode::NearestEven,
    })
}

fn run_function_decl<'input, 'b>(
    resolver: &mut ScopedResolver<'input, 'b>,
    func_directive: ast::MethodDeclaration<'input, &'input str>,
) -> Result<(Vec<ast::Variable<SpirvWord>>, Vec<ast::Variable<SpirvWord>>), TranslateError> {
    assert!(func_directive.shared_mem.is_none());
    let return_arguments = func_directive
        .return_arguments
        .into_iter()
        .map(|var| run_variable(resolver, var))
        .collect::<Result<Vec<_>, _>>()?;
    let input_arguments = func_directive
        .input_arguments
        .into_iter()
        .map(|var| run_variable(resolver, var))
        .collect::<Result<Vec<_>, _>>()?;
    Ok((return_arguments, input_arguments))
}

fn run_variable<'input, 'b>(
    resolver: &mut ScopedResolver<'input, 'b>,
    variable: ast::Variable<&'input str>,
) -> Result<ast::Variable<SpirvWord>, TranslateError> {
    Ok(ast::Variable {
        name: resolver.add(
            Cow::Borrowed(variable.name),
            Some((variable.v_type.clone(), variable.state_space)),
        )?,
        align: variable.align,
        v_type: variable.v_type,
        state_space: variable.state_space,
        array_init: variable.array_init,
    })
}

fn run_statements<'input, 'b>(
    resolver: &mut ScopedResolver<'input, 'b>,
    result: &mut Vec<NormalizedStatement>,
    statements: Vec<ast::Statement<ast::ParsedOperand<&'input str>>>,
) -> Result<(), TranslateError> {
    for statement in statements.iter() {
        match statement {
            ast::Statement::Label(label) => {
                resolver.add(Cow::Borrowed(*label), None)?;
            }
            _ => {}
        }
    }
    for statement in statements {
        match statement {
            ast::Statement::Label(label) => {
                result.push(Statement::Label(resolver.get_in_current_scope(label)?))
            }
            ast::Statement::Variable(variable) => run_multivariable(resolver, result, variable)?,
            ast::Statement::Instruction(predicate, instruction) => {
                result.push(Statement::Instruction((
                    predicate
                        .map(|pred| {
                            Ok::<_, TranslateError>(ast::PredAt {
                                not: pred.not,
                                label: resolver.get(pred.label)?,
                            })
                        })
                        .transpose()?,
                    run_instruction(resolver, instruction)?,
                )))
            }
            ast::Statement::Block(block) => {
                resolver.start_scope();
                run_statements(resolver, result, block)?;
                resolver.end_scope();
            }
        }
    }
    Ok(())
}

fn run_instruction<'input, 'b>(
    resolver: &mut ScopedResolver<'input, 'b>,
    instruction: ast::Instruction<ast::ParsedOperand<&'input str>>,
) -> Result<ast::Instruction<ast::ParsedOperand<SpirvWord>>, TranslateError> {
    ast::visit_map(instruction, &mut |name: &'input str,
                                      _: Option<(
        &ast::Type,
        ast::StateSpace,
    )>,
                                      _,
                                      _| {
        resolver.get(&name)
    })
}

fn run_multivariable<'input, 'b>(
    resolver: &mut ScopedResolver<'input, 'b>,
    result: &mut Vec<NormalizedStatement>,
    variable: ast::MultiVariable<&'input str>,
) -> Result<(), TranslateError> {
    match variable.count {
        Some(count) => {
            for i in 0..count {
                let name = Cow::Owned(format!("{}{}", variable.var.name, i));
                let ident = resolver.add(
                    name,
                    Some((variable.var.v_type.clone(), variable.var.state_space)),
                )?;
                result.push(Statement::Variable(ast::Variable {
                    align: variable.var.align,
                    v_type: variable.var.v_type.clone(),
                    state_space: variable.var.state_space,
                    name: ident,
                    array_init: variable.var.array_init.clone(),
                }));
            }
        }
        None => {
            let name = Cow::Borrowed(variable.var.name);
            let ident = resolver.add(
                name,
                Some((variable.var.v_type.clone(), variable.var.state_space)),
            )?;
            result.push(Statement::Variable(ast::Variable {
                align: variable.var.align,
                v_type: variable.var.v_type.clone(),
                state_space: variable.var.state_space,
                name: ident,
                array_init: variable.var.array_init.clone(),
            }));
        }
    }
    Ok(())
}
