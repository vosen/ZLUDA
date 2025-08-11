use super::*;
use smallvec::*;

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
            Ok::<SmallVec<[_; 3]>, _>(match statement {
                Statement::Instruction(ast::Instruction::ShflSync {
                    data,
                    arguments:
                        ast::ShflSyncArgs {
                            dst_pred: Some(dst_pred),
                            dst,
                            src,
                            src_lane,
                            src_opts,
                            src_membermask,
                        },
                }) => {
                    let mode = match data.mode {
                        ptx_parser::ShuffleMode::Up => "up",
                        ptx_parser::ShuffleMode::Down => "down",
                        ptx_parser::ShuffleMode::BFly => "bfly",
                        ptx_parser::ShuffleMode::Idx => "idx",
                    };
                    let packed_var = resolver.register_unnamed(Some((
                        ast::Type::Vector(2, ast::ScalarType::U32),
                        ptx_parser::StateSpace::Reg,
                    )));
                    let dst_pred_wide = resolver.register_unnamed(Some((
                        ast::Type::Scalar(ast::ScalarType::U32),
                        ptx_parser::StateSpace::Reg,
                    )));
                    let full_name = [ZLUDA_PTX_PREFIX, "shfl_sync_", mode, "_b32_pred"].concat();
                    let return_arguments = vec![(
                        ast::Type::Vector(2, ast::ScalarType::U32),
                        ptx_parser::StateSpace::Reg,
                    )];
                    let input_arguments = vec![
                        (
                            ast::Type::Scalar(ast::ScalarType::U32),
                            ptx_parser::StateSpace::Reg,
                        ),
                        (
                            ast::Type::Scalar(ast::ScalarType::U32),
                            ptx_parser::StateSpace::Reg,
                        ),
                        (
                            ast::Type::Scalar(ast::ScalarType::U32),
                            ptx_parser::StateSpace::Reg,
                        ),
                        (
                            ast::Type::Scalar(ast::ScalarType::U32),
                            ptx_parser::StateSpace::Reg,
                        ),
                    ];
                    let func = match fn_declarations.entry(full_name.into()) {
                        hash_map::Entry::Occupied(occupied_entry) => occupied_entry.get().1,
                        hash_map::Entry::Vacant(vacant_entry) => {
                            let name = vacant_entry.key().clone();
                            let name = resolver.register_named(name, None);
                            vacant_entry.insert((
                                to_variables(resolver, &return_arguments),
                                name,
                                to_variables(resolver, &input_arguments),
                            ));
                            name
                        }
                    };
                    smallvec![
                        Statement::Instruction::<_, SpirvWord>(ast::Instruction::Call {
                            data: ptx_parser::CallDetails {
                                uniform: false,
                                return_arguments,
                                input_arguments
                            },
                            arguments: ptx_parser::CallArgs {
                                return_arguments: vec![packed_var],
                                func,
                                input_arguments: vec![src, src_lane, src_opts, src_membermask],
                            },
                        }),
                        Statement::RepackVector(RepackVectorDetails {
                            is_extract: true,
                            typ: ast::ScalarType::U32,
                            packed: packed_var,
                            unpacked: vec![dst, dst_pred_wide],
                            relaxed_type_check: false,
                        }),
                        Statement::Instruction(ast::Instruction::Cvt {
                            data: ast::CvtDetails {
                                from: ast::ScalarType::U32,
                                to: ast::ScalarType::Pred,
                                mode: ast::CvtMode::Truncate
                            },
                            arguments: ast::CvtArgs {
                                dst: dst_pred,
                                src: dst_pred_wide,
                                src2: None,
                            },
                        })
                    ]
                }
                Statement::Instruction(ast::Instruction::Cvt {
                    data:
                        ast::CvtDetails {
                            from: from @ (ast::ScalarType::E4m3x2 | ast::ScalarType::E5m2x2),
                            to: ast::ScalarType::F16x2,
                            mode: _,
                        },
                    arguments:
                        ast::CvtArgs {
                            dst,
                            src,
                            src2: None,
                        },
                }) => {
                    let from_str = match from {
                        ast::ScalarType::E4m3x2 => "e4m3x2",
                        ast::ScalarType::E5m2x2 => "e5m2x2",
                        _ => unreachable!(),
                    };
                    let packed_output = resolver.register_unnamed(Some((
                        ast::Type::Scalar(ast::ScalarType::B32),
                        ast::StateSpace::Reg,
                    )));
                    let name = format!("cvt_rn_f16x2_{}", from_str).into();
                    let return_arguments = vec![(
                        ast::Type::Scalar(ast::ScalarType::B32),
                        ast::StateSpace::Reg,
                    )];
                    let input_arguments = vec![(
                        ast::Type::Scalar(ast::ScalarType::B16),
                        ast::StateSpace::Reg,
                    )];
                    // TODO: fill out above
                    // TODO: refactor common bits
                    let func = match fn_declarations.entry(name) {
                        hash_map::Entry::Occupied(occupied_entry) => occupied_entry.get().1,
                        hash_map::Entry::Vacant(vacant_entry) => {
                            let name = vacant_entry.key().clone();
                            let full_name = [ZLUDA_PTX_PREFIX, &*name].concat();
                            let name = resolver.register_named(Cow::Owned(full_name.clone()), None);
                            vacant_entry.insert((
                                to_variables(resolver, &return_arguments),
                                name,
                                to_variables(resolver, &input_arguments),
                            ));
                            name
                        }
                    };
                    smallvec![
                        Statement::Instruction::<_, SpirvWord>(ast::Instruction::Call {
                            data: ptx_parser::CallDetails {
                                uniform: false,
                                return_arguments,
                                input_arguments,
                            },
                            arguments: ptx_parser::CallArgs {
                                return_arguments: vec![packed_output],
                                func,
                                input_arguments: vec![src],
                            },
                        }),
                        Statement::Instruction(ast::Instruction::Cvt {
                            data: ast::CvtDetails {
                                from: ast::ScalarType::B32,
                                to: ast::ScalarType::F16x2,
                                mode: ast::CvtMode::Bitcast
                            },
                            arguments: ast::CvtArgs {
                                dst,
                                src: packed_output,
                                src2: None,
                            },
                        })
                    ]
                }
                Statement::<ast::Instruction<SpirvWord>, SpirvWord>::Instruction(instruction) => {
                    smallvec![
                        Statement::<ast::Instruction<SpirvWord>, SpirvWord>::Instruction(
                            run_instruction(resolver, fn_declarations, instruction)?
                        )
                    ]
                }
                s => smallvec![s],
            })
        })
        .flat_map(|result| match result {
            Ok(vec) => vec.into_iter().map(|item| Ok(item)).collect(),
            Err(er) => vec![Err(er)],
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
        i @ ptx_parser::Instruction::Sqrt {
            data:
                ast::RcpData {
                    kind: ast::RcpKind::Approx,
                    type_: ast::ScalarType::F32,
                    flush_to_zero: None | Some(false),
                },
            ..
        } => to_call(resolver, fn_declarations, "sqrt_approx_f32".into(), i)?,
        i @ ptx_parser::Instruction::Rsqrt {
            data:
                ast::TypeFtz {
                    type_: ast::ScalarType::F32,
                    flush_to_zero: None | Some(false),
                },
            ..
        } => to_call(resolver, fn_declarations, "rsqrt_approx_f32".into(), i)?,
        i @ ptx_parser::Instruction::Rcp {
            data:
                ast::RcpData {
                    kind: ast::RcpKind::Approx,
                    type_: ast::ScalarType::F32,
                    flush_to_zero: None | Some(false),
                },
            ..
        } => to_call(resolver, fn_declarations, "rcp_approx_f32".into(), i)?,
        i @ ptx_parser::Instruction::Ex2 {
            data:
                ast::TypeFtz {
                    type_: ast::ScalarType::F32,
                    flush_to_zero: None | Some(false),
                },
            ..
        } => to_call(resolver, fn_declarations, "ex2_approx_f32".into(), i)?,
        i @ ptx_parser::Instruction::Lg2 {
            data: ast::FlushToZero {
                flush_to_zero: false,
            },
            ..
        } => to_call(resolver, fn_declarations, "lg2_approx_f32".into(), i)?,
        i @ ptx_parser::Instruction::Activemask { .. } => {
            to_call(resolver, fn_declarations, "activemask".into(), i)?
        }
        i @ ptx_parser::Instruction::Bfe { data, .. } => {
            let name = ["bfe_", scalar_to_ptx_name(data)].concat();
            to_call(resolver, fn_declarations, name.into(), i)?
        }
        i @ ptx_parser::Instruction::Sqrt {
            data:
                ast::RcpData {
                    kind: ast::RcpKind::Compliant(ast::RoundingMode::NearestEven),
                    type_: ast::ScalarType::F32,
                    flush_to_zero: Some(true),
                    ..
                },
            ..
        } => {
            let name = "sqrt_rn_ftz_f32";
            to_call(resolver, fn_declarations, name.into(), i)?
        }
        i @ ptx_parser::Instruction::Sqrt {
            data:
                ast::RcpData {
                    kind: ast::RcpKind::Compliant(ast::RoundingMode::NearestEven),
                    type_: ast::ScalarType::F32,
                    ..
                },
            ..
        } => {
            let name = "sqrt_rn_f32";
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
            to_call(
                resolver,
                fn_declarations,
                name.into(),
                ptx_parser::Instruction::BarRed { data, arguments },
            )?
        }
        ptx_parser::Instruction::ShflSync {
            data,
            arguments: orig_arguments @ ast::ShflSyncArgs { dst_pred: None, .. },
        } => {
            let mode = match data.mode {
                ptx_parser::ShuffleMode::Up => "up",
                ptx_parser::ShuffleMode::Down => "down",
                ptx_parser::ShuffleMode::BFly => "bfly",
                ptx_parser::ShuffleMode::Idx => "idx",
            };
            to_call(
                resolver,
                fn_declarations,
                format!("shfl_sync_{}_b32", mode).into(),
                ptx_parser::Instruction::ShflSync {
                    data,
                    arguments: orig_arguments,
                },
            )?
        }
        i @ ptx_parser::Instruction::Nanosleep { .. } => {
            to_call(resolver, fn_declarations, "nanosleep_u32".into(), i)?
        }
        i @ ptx_parser::Instruction::Cvt {
            data:
                ptx_parser::CvtDetails {
                    from: ast::ScalarType::F32,
                    to: to @ (ast::ScalarType::E4m3x2 | ast::ScalarType::E5m2x2),
                    mode: _,
                },
            arguments: _,
        } => {
            let to = match to {
                ptx_parser::ScalarType::E4m3x2 => "e4m3x2",
                ptx_parser::ScalarType::E5m2x2 => "e5m2x2",
                _ => unreachable!(),
            };
            // Conversions from f32 to f8 must have two source arguments.
            // satfinite is mandatory for conversions to e4m3x2.
            to_call(
                resolver,
                fn_declarations,
                format!("cvt_rn_satfinite_{}_f32", to).into(),
                i,
            )?
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
