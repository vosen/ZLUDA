use super::*;

pub(super) fn run<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    directives: Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>,
) -> Result<Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>, TranslateError> {
    let mut fn_declarations = FxHashMap::default();
    let remapped_directives = directives
        .into_iter()
        .map(|directive| run_directive(resolver, &mut fn_declarations, directive))
        .collect::<Result<Vec<_>, _>>()?;
    let mut result = fn_declarations
        .into_iter()
        .map(|(_, (return_arguments, name, input_arguments))| {
            Directive2::Method(Function2 {
                return_arguments,
                name: name,
                input_arguments,
                body: None,
                import_as: None,
                tuning: Vec::new(),
                linkage: ast::LinkingDirective::EXTERN,
                is_kernel: false,
                flush_to_zero_f32: false,
                flush_to_zero_f16f64: false,
                rounding_mode_f32: ptx_parser::RoundingMode::NearestEven,
                rounding_mode_f16f64: ptx_parser::RoundingMode::NearestEven,
            })
        })
        .collect::<Vec<_>>();
    result.extend(remapped_directives);
    Ok(result)
}

fn run_directive<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    fn_declarations: &mut FxHashMap<
        Cow<'input, str>,
        (
            Vec<ast::Variable<SpirvWord>>,
            SpirvWord,
            Vec<ast::Variable<SpirvWord>>,
        ),
    >,
    directive: Directive2<ast::Instruction<SpirvWord>, SpirvWord>,
) -> Result<Directive2<ast::Instruction<SpirvWord>, SpirvWord>, TranslateError> {
    Ok(match directive {
        var @ Directive2::Variable(..) => var,
        Directive2::Method(mut method) => {
            method.body = method
                .body
                .map(|statements| run_statements(resolver, fn_declarations, statements))
                .transpose()?;
            Directive2::Method(method)
        }
    })
}

fn run_statements<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    fn_declarations: &mut FxHashMap<
        Cow<'input, str>,
        (
            Vec<ast::Variable<SpirvWord>>,
            SpirvWord,
            Vec<ast::Variable<SpirvWord>>,
        ),
    >,
    statements: Vec<Statement<ast::Instruction<SpirvWord>, SpirvWord>>,
) -> Result<Vec<Statement<ast::Instruction<SpirvWord>, SpirvWord>>, TranslateError> {
    statements
        .into_iter()
        .map(|statement| {
            Ok(match statement {
                Statement::Instruction(instruction) => {
                    Statement::Instruction(run_instruction(resolver, fn_declarations, instruction)?)
                }
                s => s,
            })
        })
        .collect::<Result<Vec<_>, _>>()
}

fn run_instruction<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    fn_declarations: &mut FxHashMap<
        Cow<'input, str>,
        (
            Vec<ast::Variable<SpirvWord>>,
            SpirvWord,
            Vec<ast::Variable<SpirvWord>>,
        ),
    >,
    instruction: ptx_parser::Instruction<SpirvWord>,
) -> Result<ptx_parser::Instruction<SpirvWord>, TranslateError> {
    Ok(match instruction {
        i @ ptx_parser::Instruction::Activemask { .. } => {
            to_call(resolver, fn_declarations, "activemask".into(), i)?
        }
        i @ ptx_parser::Instruction::Bfe { data, .. } => {
            let name = ["bfe_", scalar_to_ptx_name(data)].concat();
            to_call(resolver, fn_declarations, name.into(), i)?
        }
        i @ ptx_parser::Instruction::Bfi { data, .. } => {
            let name = ["bfi_", scalar_to_ptx_name(data)].concat();
            to_call(resolver, fn_declarations, name.into(), i)?
        }
        i @ ptx_parser::Instruction::Bar { .. } => {
            to_call(resolver, fn_declarations, "bar_sync".into(), i)?
        }
        ptx_parser::Instruction::BarRed { data, arguments } => {
            if arguments.src_threadcount.is_some() {
                return Err(error_todo());
            }
            let name = match data.pred_reduction {
                ptx_parser::Reduction::And => "bar_red_and_pred",
                ptx_parser::Reduction::Or => "bar_red_or_pred",
            };
            to_call(resolver, fn_declarations, name.into(), ptx_parser::Instruction::BarRed { data, arguments })?
        }
        ptx_parser::Instruction::ShflSync { data, arguments } => {
            let mode = match data.mode {
                ptx_parser::ShuffleMode::Up => "up",
                ptx_parser::ShuffleMode::Down => "down",
                ptx_parser::ShuffleMode::BFly => "bfly",
                ptx_parser::ShuffleMode::Idx => "idx",
            };
            let pred = if arguments.dst_pred.is_some() {
                "_pred"
            } else {
                ""
            };
            to_call(
                resolver,
                fn_declarations,
                format!("shfl_sync_{}_b32{}", mode, pred).into(),
                ptx_parser::Instruction::ShflSync { data, arguments },
            )?
        }
        i @ ptx_parser::Instruction::Nanosleep { .. } => {
            to_call(resolver, fn_declarations, "nanosleep_u32".into(), i)?
        }
        i => i,
    })
}

fn to_call<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    fn_declarations: &mut FxHashMap<
        Cow<'input, str>,
        (
            Vec<ast::Variable<SpirvWord>>,
            SpirvWord,
            Vec<ast::Variable<SpirvWord>>,
        ),
    >,
    name: Cow<'input, str>,
    i: ast::Instruction<SpirvWord>,
) -> Result<ptx_parser::Instruction<SpirvWord>, TranslateError> {
    let mut data_return = Vec::new();
    let mut data_input = Vec::new();
    let mut arguments_return = Vec::new();
    let mut arguments_input = Vec::new();
    ast::visit(&i, &mut |name: &SpirvWord,
                         type_space: Option<(
        &ptx_parser::Type,
        ptx_parser::StateSpace,
    )>,
                         is_dst: bool,
                         _: bool| {
        let (type_, space) = type_space.ok_or_else(error_mismatched_type)?;
        if is_dst {
            data_return.push((type_.clone(), space));
            arguments_return.push(*name);
        } else {
            data_input.push((type_.clone(), space));
            arguments_input.push(*name);
        };
        Ok::<_, TranslateError>(())
    })?;
    let fn_name = match fn_declarations.entry(name) {
        hash_map::Entry::Occupied(occupied_entry) => occupied_entry.get().1,
        hash_map::Entry::Vacant(vacant_entry) => {
            let name = vacant_entry.key().clone();
            let full_name = [ZLUDA_PTX_PREFIX, &*name].concat();
            let name = resolver.register_named(Cow::Owned(full_name.clone()), None);
            vacant_entry.insert((
                to_variables(resolver, &data_return),
                name,
                to_variables(resolver, &data_input),
            ));
            name
        }
    };
    Ok(ast::Instruction::Call {
        data: ptx_parser::CallDetails {
            uniform: false,
            return_arguments: data_return,
            input_arguments: data_input,
        },
        arguments: ptx_parser::CallArgs {
            return_arguments: arguments_return,
            func: fn_name,
            input_arguments: arguments_input,
        },
    })
}

fn to_variables<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    arguments: &Vec<(ptx_parser::Type, ptx_parser::StateSpace)>,
) -> Vec<ptx_parser::Variable<SpirvWord>> {
    arguments
        .iter()
        .map(|(type_, space)| ast::Variable {
            align: None,
            v_type: type_.clone(),
            state_space: *space,
            name: resolver.register_unnamed(Some((type_.clone(), *space))),
            array_init: Vec::new(),
        })
        .collect::<Vec<_>>()
}
