use std::collections::{BTreeMap, BTreeSet};

use super::*;

/*
    PTX represents dynamically allocated shared local memory as
        .extern .shared .b32 shared_mem[];
    In SPIRV/OpenCL world this is expressed as an additional argument to the kernel
    And in AMD compilation
    This pass looks for all uses of .extern .shared and converts them to
    an additional method argument
    The question is how this artificial argument should be expressed. There are
    several options:
    * Straight conversion:
        .shared .b32 shared_mem[]
    * Introduce .param_shared statespace:
        .param_shared .b32 shared_mem
        or
        .param_shared .b32 shared_mem[]
    * Introduce .shared_ptr <SCALAR> type:
        .param .shared_ptr .b32 shared_mem
    * Reuse .ptr hint:
        .param .u64 .ptr shared_mem
      This is the most tempting, but also the most nonsensical, .ptr is just a
      hint, which has no semantical meaning (and the output of our
      transformation has a semantical meaning - we emit additional
      "OpFunctionParameter ..." with type "OpTypePointer Workgroup ...")
*/
pub(super) fn run<'input>(
    module: Vec<Directive<'input>>,
    kernels_methods_call_map: &MethodsCallMap<'input>,
    new_id: &mut impl FnMut() -> SpirvWord,
) -> Result<Vec<Directive<'input>>, TranslateError> {
    let mut globals_shared = HashMap::new();
    for dir in module.iter() {
        match dir {
            Directive::Variable(
                _,
                ast::Variable {
                    state_space: ast::StateSpace::Shared,
                    name,
                    v_type,
                    ..
                },
            ) => {
                globals_shared.insert(*name, v_type.clone());
            }
            _ => {}
        }
    }
    if globals_shared.len() == 0 {
        return Ok(module);
    }
    let mut methods_to_directly_used_shared_globals = HashMap::<_, HashSet<SpirvWord>>::new();
    let module = module
        .into_iter()
        .map(|directive| match directive {
            Directive::Method(Function {
                func_decl,
                globals,
                body: Some(statements),
                import_as,
                tuning,
                linkage,
            }) => {
                let call_key = (*func_decl).borrow().name;
                let statements = statements
                    .into_iter()
                    .map(|statement| {
                        statement.visit_map(
                            &mut |id, _: Option<(&ast::Type, ast::StateSpace)>, _, _| {
                                if let Some(_) = globals_shared.get(&id) {
                                    methods_to_directly_used_shared_globals
                                        .entry(call_key)
                                        .or_insert_with(HashSet::new)
                                        .insert(id);
                                }
                                Ok::<_, TranslateError>(id)
                            },
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok::<_, TranslateError>(Directive::Method(Function {
                    func_decl,
                    globals,
                    body: Some(statements),
                    import_as,
                    tuning,
                    linkage,
                }))
            }
            directive => Ok(directive),
        })
        .collect::<Result<Vec<_>, _>>()?;
    // If there's a chain `kernel` -> `fn1` -> `fn2`, where only `fn2` uses extern shared,
    // make sure it gets propagated to `fn1` and `kernel`
    let methods_to_indirectly_used_shared_globals = resolve_indirect_uses_of_globals_shared(
        methods_to_directly_used_shared_globals,
        kernels_methods_call_map,
    );
    // now visit every method declaration and inject those additional arguments
    let mut directives = Vec::with_capacity(module.len());
    for directive in module.into_iter() {
        match directive {
            Directive::Method(Function {
                func_decl,
                globals,
                body: Some(statements),
                import_as,
                tuning,
                linkage,
            }) => {
                let statements = {
                    let func_decl_ref = &mut (*func_decl).borrow_mut();
                    let method_name = func_decl_ref.name;
                    insert_arguments_remap_statements(
                        new_id,
                        kernels_methods_call_map,
                        &globals_shared,
                        &methods_to_indirectly_used_shared_globals,
                        method_name,
                        &mut directives,
                        func_decl_ref,
                        statements,
                    )?
                };
                directives.push(Directive::Method(Function {
                    func_decl,
                    globals,
                    body: Some(statements),
                    import_as,
                    tuning,
                    linkage,
                }));
            }
            directive => directives.push(directive),
        }
    }
    Ok(directives)
}

// We need to compute two kinds of information:
// * If it's a kernel -> size of .shared globals in use (direct or indirect)
// * If it's a function -> does it use .shared global (directly or indirectly)
fn resolve_indirect_uses_of_globals_shared<'input>(
    methods_use_of_globals_shared: HashMap<ast::MethodName<'input, SpirvWord>, HashSet<SpirvWord>>,
    kernels_methods_call_map: &MethodsCallMap<'input>,
) -> HashMap<ast::MethodName<'input, SpirvWord>, BTreeSet<SpirvWord>> {
    let mut result = HashMap::new();
    for (method, callees) in kernels_methods_call_map.methods() {
        let mut indirect_globals = methods_use_of_globals_shared
            .get(&method)
            .into_iter()
            .flatten()
            .copied()
            .collect::<BTreeSet<_>>();
        for &callee in callees {
            indirect_globals.extend(
                methods_use_of_globals_shared
                    .get(&ast::MethodName::Func(callee))
                    .into_iter()
                    .flatten()
                    .copied(),
            );
        }
        result.insert(method, indirect_globals);
    }
    result
}

fn insert_arguments_remap_statements<'input>(
    new_id: &mut impl FnMut() -> SpirvWord,
    kernels_methods_call_map: &MethodsCallMap<'input>,
    globals_shared: &HashMap<SpirvWord, ast::Type>,
    methods_to_indirectly_used_shared_globals: &HashMap<
        ast::MethodName<'input, SpirvWord>,
        BTreeSet<SpirvWord>,
    >,
    method_name: ast::MethodName<SpirvWord>,
    result: &mut Vec<Directive>,
    func_decl_ref: &mut std::cell::RefMut<ast::MethodDeclaration<SpirvWord>>,
    statements: Vec<Statement<ast::Instruction<SpirvWord>, SpirvWord>>,
) -> Result<Vec<Statement<ast::Instruction<SpirvWord>, SpirvWord>>, TranslateError> {
    let remapped_globals_in_method =
        if let Some(method_globals) = methods_to_indirectly_used_shared_globals.get(&method_name) {
            match method_name {
                ast::MethodName::Func(..) => {
                    let remapped_globals = method_globals
                        .iter()
                        .map(|global| {
                            (
                                *global,
                                (
                                    new_id(),
                                    globals_shared
                                        .get(&global)
                                        .unwrap_or_else(|| todo!())
                                        .clone(),
                                ),
                            )
                        })
                        .collect::<BTreeMap<_, _>>();
                    for (_, (new_shared_global_id, shared_global_type)) in remapped_globals.iter() {
                        func_decl_ref.input_arguments.push(ast::Variable {
                            align: None,
                            v_type: shared_global_type.clone(),
                            state_space: ast::StateSpace::Shared,
                            name: *new_shared_global_id,
                            array_init: Vec::new(),
                        });
                    }
                    remapped_globals
                }
                ast::MethodName::Kernel(..) => method_globals
                    .iter()
                    .map(|global| {
                        (
                            *global,
                            (
                                *global,
                                globals_shared
                                    .get(&global)
                                    .unwrap_or_else(|| todo!())
                                    .clone(),
                            ),
                        )
                    })
                    .collect::<BTreeMap<_, _>>(),
            }
        } else {
            return Ok(statements);
        };
    replace_uses_of_shared_memory(
        new_id,
        methods_to_indirectly_used_shared_globals,
        statements,
        remapped_globals_in_method,
    )
}

fn replace_uses_of_shared_memory<'input>(
    new_id: &mut impl FnMut() -> SpirvWord,
    methods_to_indirectly_used_shared_globals: &HashMap<
        ast::MethodName<'input, SpirvWord>,
        BTreeSet<SpirvWord>,
    >,
    statements: Vec<ExpandedStatement>,
    remapped_globals_in_method: BTreeMap<SpirvWord, (SpirvWord, ast::Type)>,
) -> Result<Vec<ExpandedStatement>, TranslateError> {
    let mut result = Vec::with_capacity(statements.len());
    for statement in statements {
        match statement {
            Statement::Instruction(ast::Instruction::Call {
                mut data,
                mut arguments,
            }) => {
                // We can safely skip checking call arguments,
                // because there's simply no way to pass shared ptr
                // without converting it to .b64 first
                if let Some(shared_globals_used_by_callee) =
                    methods_to_indirectly_used_shared_globals
                        .get(&ast::MethodName::Func(arguments.func))
                {
                    for &shared_global_used_by_callee in shared_globals_used_by_callee {
                        let (remapped_shared_id, type_) = remapped_globals_in_method
                            .get(&shared_global_used_by_callee)
                            .unwrap_or_else(|| todo!());
                        data.input_arguments
                            .push((type_.clone(), ast::StateSpace::Shared));
                        arguments.input_arguments.push(*remapped_shared_id);
                    }
                }
                result.push(Statement::Instruction(ast::Instruction::Call {
                    data,
                    arguments,
                }))
            }
            statement => {
                let new_statement =
                    statement.visit_map(&mut |id,
                                               _: Option<(&ast::Type, ast::StateSpace)>,
                                               _,
                                               _| {
                        Ok::<_, TranslateError>(
                            if let Some((remapped_shared_id, _)) =
                                remapped_globals_in_method.get(&id)
                            {
                                *remapped_shared_id
                            } else {
                                id
                            },
                        )
                    })?;
                result.push(new_statement);
            }
        }
    }
    Ok(result)
}
