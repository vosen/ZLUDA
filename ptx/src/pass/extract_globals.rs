use super::*;

pub(super) fn run<'input, 'b>(
    sorted_statements: Vec<ExpandedStatement>,
    ptx_impl_imports: &mut HashMap<String, Directive>,
    id_def: &mut NumericIdResolver,
) -> Result<(Vec<ExpandedStatement>, Vec<ast::Variable<SpirvWord>>), TranslateError> {
    let mut local = Vec::with_capacity(sorted_statements.len());
    let mut global = Vec::new();
    for statement in sorted_statements {
        match statement {
            Statement::Variable(
                var @ ast::Variable {
                    state_space: ast::StateSpace::Shared,
                    ..
                },
            )
            | Statement::Variable(
                var @ ast::Variable {
                    state_space: ast::StateSpace::Global,
                    ..
                },
            ) => global.push(var),
            Statement::Instruction(ast::Instruction::Bfe { data, arguments }) => {
                let fn_name = [ZLUDA_PTX_PREFIX, "bfe_", scalar_to_ptx_name(data)].concat();
                local.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Bfe { data, arguments },
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::Bfi { data, arguments }) => {
                let fn_name = [ZLUDA_PTX_PREFIX, "bfi_", scalar_to_ptx_name(data)].concat();
                local.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Bfi { data, arguments },
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::Brev { data, arguments }) => {
                let fn_name: String =
                    [ZLUDA_PTX_PREFIX, "brev_", scalar_to_ptx_name(data)].concat();
                local.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Brev { data, arguments },
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::Activemask { arguments }) => {
                let fn_name = [ZLUDA_PTX_PREFIX, "activemask"].concat();
                local.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Activemask { arguments },
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::Atom {
                data:
                    data @ ast::AtomDetails {
                        op: ast::AtomicOp::IncrementWrap,
                        semantics,
                        scope,
                        space,
                        ..
                    },
                arguments,
            }) => {
                let fn_name = [
                    ZLUDA_PTX_PREFIX,
                    "atom_",
                    semantics_to_ptx_name(semantics),
                    "_",
                    scope_to_ptx_name(scope),
                    "_",
                    space_to_ptx_name(space),
                    "_inc",
                ]
                .concat();
                local.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Atom { data, arguments },
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::Atom {
                data:
                    data @ ast::AtomDetails {
                        op: ast::AtomicOp::DecrementWrap,
                        semantics,
                        scope,
                        space,
                        ..
                    },
                arguments,
            }) => {
                let fn_name = [
                    ZLUDA_PTX_PREFIX,
                    "atom_",
                    semantics_to_ptx_name(semantics),
                    "_",
                    scope_to_ptx_name(scope),
                    "_",
                    space_to_ptx_name(space),
                    "_dec",
                ]
                .concat();
                local.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Atom { data, arguments },
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::Atom {
                data:
                    data @ ast::AtomDetails {
                        op: ast::AtomicOp::FloatAdd,
                        semantics,
                        scope,
                        space,
                        ..
                    },
                arguments,
            }) => {
                let scalar_type = match data.type_ {
                    ptx_parser::Type::Scalar(scalar) => scalar,
                    _ => return Err(error_unreachable()),
                };
                let fn_name = [
                    ZLUDA_PTX_PREFIX,
                    "atom_",
                    semantics_to_ptx_name(semantics),
                    "_",
                    scope_to_ptx_name(scope),
                    "_",
                    space_to_ptx_name(space),
                    "_add_",
                    scalar_to_ptx_name(scalar_type),
                ]
                .concat();
                local.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Atom { data, arguments },
                    fn_name,
                )?);
            }
            s => local.push(s),
        }
    }
    Ok((local, global))
}

fn instruction_to_fn_call(
    id_defs: &mut NumericIdResolver,
    ptx_impl_imports: &mut HashMap<String, Directive>,
    inst: ast::Instruction<SpirvWord>,
    fn_name: String,
) -> Result<ExpandedStatement, TranslateError> {
    let mut arguments = Vec::new();
    ast::visit_map(inst, &mut |operand,
                               type_space: Option<(
        &ast::Type,
        ast::StateSpace,
    )>,
                               is_dst,
                               _| {
        let (typ, space) = match type_space {
            Some((typ, space)) => (typ.clone(), space),
            None => return Err(error_unreachable()),
        };
        arguments.push((operand, is_dst, typ, space));
        Ok(SpirvWord(0))
    })?;
    let return_arguments_count = arguments
        .iter()
        .position(|(desc, is_dst, _, _)| !is_dst)
        .unwrap_or(arguments.len());
    let (return_arguments, input_arguments) = arguments.split_at(return_arguments_count);
    let fn_id = register_external_fn_call(
        id_defs,
        ptx_impl_imports,
        fn_name,
        return_arguments
            .iter()
            .map(|(_, _, typ, state)| (typ, *state)),
        input_arguments
            .iter()
            .map(|(_, _, typ, state)| (typ, *state)),
    )?;
    Ok(Statement::Instruction(ast::Instruction::Call {
        data: ast::CallDetails {
            uniform: false,
            return_arguments: return_arguments
                .iter()
                .map(|(_, _, typ, state)| (typ.clone(), *state))
                .collect::<Vec<_>>(),
            input_arguments: input_arguments
                .iter()
                .map(|(_, _, typ, state)| (typ.clone(), *state))
                .collect::<Vec<_>>(),
        },
        arguments: ast::CallArgs {
            return_arguments: return_arguments
                .iter()
                .map(|(name, _, _, _)| *name)
                .collect::<Vec<_>>(),
            func: fn_id,
            input_arguments: input_arguments
                .iter()
                .map(|(name, _, _, _)| *name)
                .collect::<Vec<_>>(),
        },
    }))
}

fn semantics_to_ptx_name(this: ast::AtomSemantics) -> &'static str {
    match this {
        ast::AtomSemantics::Relaxed => "relaxed",
        ast::AtomSemantics::Acquire => "acquire",
        ast::AtomSemantics::Release => "release",
        ast::AtomSemantics::AcqRel => "acq_rel",
    }
}

fn scope_to_ptx_name(this: ast::MemScope) -> &'static str {
    match this {
        ast::MemScope::Cta => "cta",
        ast::MemScope::Gpu => "gpu",
        ast::MemScope::Sys => "sys",
        ast::MemScope::Cluster => "cluster",
    }
}

fn space_to_ptx_name(this: ast::StateSpace) -> &'static str {
    match this {
        ast::StateSpace::Generic => "generic",
        ast::StateSpace::Global => "global",
        ast::StateSpace::Shared => "shared",
        ast::StateSpace::Reg => "reg",
        ast::StateSpace::Const => "const",
        ast::StateSpace::Local => "local",
        ast::StateSpace::Param => "param",
        ast::StateSpace::SharedCluster => "shared_cluster",
        ast::StateSpace::ParamEntry => "param_entry",
        ast::StateSpace::SharedCta => "shared_cta",
        ast::StateSpace::ParamFunc => "param_func",
    }
}
