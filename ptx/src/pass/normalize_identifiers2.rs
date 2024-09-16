use super::*;
use ptx_parser as ast;
use rustc_hash::FxHashMap;

pub(crate) fn run<'input>(
    fn_defs: &mut GlobalStringIdentResolver2<'input>,
    directives: Vec<ast::Directive<'input, ast::ParsedOperand<&'input str>>>,
) -> Result<Vec<NormalizedDirective2<'input>>, TranslateError> {
    let mut resolver = NameResolver::new(fn_defs);
    let result = directives
        .into_iter()
        .map(|directive| run_directive(&mut resolver, directive))
        .collect::<Result<Vec<_>, _>>()?;
    resolver.end_scope();
    Ok(result)
}

fn run_directive<'input, 'b>(
    resolver: &mut NameResolver<'input, 'b>,
    directive: ast::Directive<'input, ast::ParsedOperand<&'input str>>,
) -> Result<NormalizedDirective2<'input>, TranslateError> {
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
    resolver: &mut NameResolver<'input, 'b>,
    linkage: ast::LinkingDirective,
    method: ast::Function<'input, &'input str, ast::Statement<ast::ParsedOperand<&'input str>>>,
) -> Result<NormalizedFunction2<'input>, TranslateError> {
    let name = match method.func_directive.name {
        ast::MethodName::Kernel(name) => ast::MethodName::Kernel(name),
        ast::MethodName::Func(text) => {
            ast::MethodName::Func(resolver.add(Cow::Borrowed(text), None)?)
        }
    };
    resolver.start_scope();
    let func_decl = Rc::new(RefCell::new(run_function_decl(
        resolver,
        method.func_directive,
        name,
    )?));
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
        func_decl,
        globals: Vec::new(),
        body,
        import_as: None,
        tuning: method.tuning,
        linkage,
    })
}

fn run_function_decl<'input, 'b>(
    resolver: &mut NameResolver<'input, 'b>,
    func_directive: ast::MethodDeclaration<'input, &'input str>,
    name: ast::MethodName<'input, SpirvWord>,
) -> Result<ast::MethodDeclaration<'input, SpirvWord>, TranslateError> {
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
    Ok(ast::MethodDeclaration {
        return_arguments,
        name,
        input_arguments,
        shared_mem: None,
    })
}

fn run_variable<'input, 'b>(
    resolver: &mut NameResolver<'input, 'b>,
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
    resolver: &mut NameResolver<'input, 'b>,
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
    resolver: &mut NameResolver<'input, 'b>,
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
    resolver: &mut NameResolver<'input, 'b>,
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

struct NameResolver<'input, 'b> {
    flat_resolver: &'b mut GlobalStringIdentResolver2<'input>,
    scopes: Vec<ScopeStringIdentResolver<'input>>,
}

impl<'input, 'b> NameResolver<'input, 'b> {
    fn new(flat_resolver: &'b mut GlobalStringIdentResolver2<'input>) -> Self {
        Self {
            flat_resolver,
            scopes: vec![ScopeStringIdentResolver::new()],
        }
    }

    fn start_scope(&mut self) {
        self.scopes.push(ScopeStringIdentResolver::new());
    }

    fn end_scope(&mut self) {
        let scope = self.scopes.pop().unwrap();
        scope.flush(self.flat_resolver);
    }

    fn add(
        &mut self,
        name: Cow<'input, str>,
        type_space: Option<(ast::Type, ast::StateSpace)>,
    ) -> Result<SpirvWord, TranslateError> {
        let result = self.flat_resolver.current_id;
        self.flat_resolver.current_id.0 += 1;
        let current_scope = self.scopes.last_mut().unwrap();
        if current_scope
            .name_to_ident
            .insert(name.clone(), result)
            .is_some()
        {
            return Err(error_unknown_symbol());
        }
        current_scope.ident_map.insert(
            result,
            IdentEntry {
                name: Some(name),
                type_space,
            },
        );
        Ok(result)
    }

    fn get(&mut self, name: &str) -> Result<SpirvWord, TranslateError> {
        self.scopes
            .iter()
            .rev()
            .find_map(|resolver| resolver.name_to_ident.get(name).copied())
            .ok_or_else(|| error_unreachable())
    }

    fn get_in_current_scope(&self, label: &'input str) -> Result<SpirvWord, TranslateError> {
        let current_scope = self.scopes.last().unwrap();
        current_scope
            .name_to_ident
            .get(label)
            .copied()
            .ok_or_else(|| error_unreachable())
    }
}

struct ScopeStringIdentResolver<'input> {
    ident_map: FxHashMap<SpirvWord, IdentEntry<'input>>,
    name_to_ident: FxHashMap<Cow<'input, str>, SpirvWord>,
}

impl<'input> ScopeStringIdentResolver<'input> {
    fn new() -> Self {
        Self {
            ident_map: FxHashMap::default(),
            name_to_ident: FxHashMap::default(),
        }
    }

    fn flush(self, resolver: &mut GlobalStringIdentResolver2<'input>) {
        resolver.ident_map.extend(self.ident_map);
    }
}
