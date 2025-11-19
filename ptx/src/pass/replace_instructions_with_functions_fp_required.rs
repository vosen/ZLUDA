// This pass exists specifically to replace the `div.rn.ftz.f32` instruction
// with a function call. One inherent weirdness of the replacement function is
// that it requires different rounding mode for the first part of the
// division and the second part. The first part is executed with FTZ disabled
// and the second part with FTZ enabled.
// For this reason we can't handle this past FTZ mode insertion without making
// the function read and restore the FTZ mode. For this reason we split the
// replacement function in two functions and prefix them with a noop
// (FpModeRequired) that carries the FTZ mode information.

use super::*;
use ptx_parser as ast;
use smallvec::smallvec;
use smallvec::SmallVec;

pub(crate) fn run<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    directives: Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>,
) -> Result<Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>, TranslateError> {
    let mut imports = None;
    let directives = directives
        .into_iter()
        .map(|directive| run_directive(resolver, directive, &mut imports))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(match imports {
        Some(imports) => {
            let mut result = Vec::with_capacity(directives.len() + 2);
            result.extend([
                Directive2::Method(Function2 {
                    return_arguments: vec![
                        ast::Variable {
                            name: resolver.register_unnamed(Some((
                                ast::Type::Scalar(ast::ScalarType::F32),
                                ast::StateSpace::Reg,
                            ))),
                            info: ast::VariableInfo {
                                align: None,
                                v_type: ast::Type::Scalar(ast::ScalarType::F32),
                                state_space: ast::StateSpace::Reg,
                                array_init: Vec::new(),
                            },
                        },
                        ast::Variable {
                            name: resolver.register_unnamed(Some((
                                ast::Type::Scalar(ast::ScalarType::F32),
                                ast::StateSpace::Reg,
                            ))),
                            info: ast::VariableInfo {
                                align: None,
                                v_type: ast::Type::Scalar(ast::ScalarType::F32),
                                state_space: ast::StateSpace::Reg,
                                array_init: Vec::new(),
                            },
                        },
                        ast::Variable {
                            name: resolver.register_unnamed(Some((
                                ast::Type::Scalar(ast::ScalarType::F32),
                                ast::StateSpace::Reg,
                            ))),
                            info: ast::VariableInfo {
                                align: None,
                                v_type: ast::Type::Scalar(ast::ScalarType::F32),
                                state_space: ast::StateSpace::Reg,
                                array_init: Vec::new(),
                            },
                        },
                        ast::Variable {
                            name: resolver.register_unnamed(Some((
                                ast::Type::Scalar(ast::ScalarType::U8),
                                ast::StateSpace::Reg,
                            ))),
                            info: ast::VariableInfo {
                                align: None,
                                v_type: ast::Type::Scalar(ast::ScalarType::U8),
                                state_space: ast::StateSpace::Reg,
                                array_init: Vec::new(),
                            },
                        },
                    ],
                    name: imports.part1,
                    input_arguments: vec![
                        ast::Variable {
                            name: resolver.register_unnamed(Some((
                                ast::Type::Scalar(ast::ScalarType::F32),
                                ast::StateSpace::Reg,
                            ))),
                            info: ast::VariableInfo {
                                align: None,
                                v_type: ast::Type::Scalar(ast::ScalarType::F32),
                                state_space: ast::StateSpace::Reg,
                                array_init: Vec::new(),
                            },
                        },
                        ast::Variable {
                            name: resolver.register_unnamed(Some((
                                ast::Type::Scalar(ast::ScalarType::F32),
                                ast::StateSpace::Reg,
                            ))),
                            info: ast::VariableInfo {
                                align: None,
                                v_type: ast::Type::Scalar(ast::ScalarType::F32),
                                state_space: ast::StateSpace::Reg,
                                array_init: Vec::new(),
                            },
                        },
                    ],
                    body: None,
                    import_as: None,
                    tuning: Vec::new(),
                    linkage: ast::LinkingDirective::EXTERN,
                    is_kernel: false,
                    flush_to_zero_f32: false,
                    flush_to_zero_f16f64: false,
                    rounding_mode_f32: ptx_parser::RoundingMode::NearestEven,
                    rounding_mode_f16f64: ptx_parser::RoundingMode::NearestEven,
                }),
                Directive2::Method(Function2 {
                    return_arguments: vec![ast::Variable {
                        name: resolver.register_unnamed(Some((
                            ast::Type::Scalar(ast::ScalarType::F32),
                            ast::StateSpace::Reg,
                        ))),
                        info: ast::VariableInfo {
                            align: None,
                            v_type: ast::Type::Scalar(ast::ScalarType::F32),
                            state_space: ast::StateSpace::Reg,
                            array_init: Vec::new(),
                        },
                    }],
                    name: imports.part2,
                    input_arguments: vec![
                        ast::Variable {
                            name: resolver.register_unnamed(Some((
                                ast::Type::Scalar(ast::ScalarType::F32),
                                ast::StateSpace::Reg,
                            ))),
                            info: ast::VariableInfo {
                                align: None,
                                v_type: ast::Type::Scalar(ast::ScalarType::F32),
                                state_space: ast::StateSpace::Reg,
                                array_init: Vec::new(),
                            },
                        },
                        ast::Variable {
                            name: resolver.register_unnamed(Some((
                                ast::Type::Scalar(ast::ScalarType::F32),
                                ast::StateSpace::Reg,
                            ))),
                            info: ast::VariableInfo {
                                align: None,
                                v_type: ast::Type::Scalar(ast::ScalarType::F32),
                                state_space: ast::StateSpace::Reg,
                                array_init: Vec::new(),
                            },
                        },
                        ast::Variable {
                            name: resolver.register_unnamed(Some((
                                ast::Type::Scalar(ast::ScalarType::F32),
                                ast::StateSpace::Reg,
                            ))),
                            info: ast::VariableInfo {
                                align: None,
                                v_type: ast::Type::Scalar(ast::ScalarType::F32),
                                state_space: ast::StateSpace::Reg,
                                array_init: Vec::new(),
                            },
                        },
                        ast::Variable {
                            name: resolver.register_unnamed(Some((
                                ast::Type::Scalar(ast::ScalarType::F32),
                                ast::StateSpace::Reg,
                            ))),
                            info: ast::VariableInfo {
                                align: None,
                                v_type: ast::Type::Scalar(ast::ScalarType::F32),
                                state_space: ast::StateSpace::Reg,
                                array_init: Vec::new(),
                            },
                        },
                        ast::Variable {
                            name: resolver.register_unnamed(Some((
                                ast::Type::Scalar(ast::ScalarType::F32),
                                ast::StateSpace::Reg,
                            ))),
                            info: ast::VariableInfo {
                                align: None,
                                v_type: ast::Type::Scalar(ast::ScalarType::F32),
                                state_space: ast::StateSpace::Reg,
                                array_init: Vec::new(),
                            },
                        },
                        ast::Variable {
                            name: resolver.register_unnamed(Some((
                                ast::Type::Scalar(ast::ScalarType::U8),
                                ast::StateSpace::Reg,
                            ))),
                            info: ast::VariableInfo {
                                align: None,
                                v_type: ast::Type::Scalar(ast::ScalarType::U8),
                                state_space: ast::StateSpace::Reg,
                                array_init: Vec::new(),
                            },
                        },
                    ],
                    body: None,
                    import_as: None,
                    tuning: Vec::new(),
                    linkage: ast::LinkingDirective::EXTERN,
                    is_kernel: false,
                    flush_to_zero_f32: false,
                    flush_to_zero_f16f64: false,
                    rounding_mode_f32: ptx_parser::RoundingMode::NearestEven,
                    rounding_mode_f16f64: ptx_parser::RoundingMode::NearestEven,
                }),
            ]);
            result.extend(directives);
            result
        }
        None => directives,
    })
}

fn run_directive<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    directive: Directive2<ast::Instruction<SpirvWord>, SpirvWord>,
    imports: &mut Option<FunctionImports>,
) -> Result<Directive2<ast::Instruction<SpirvWord>, SpirvWord>, TranslateError> {
    Ok(match directive {
        Directive2::Variable(linking, var) => Directive2::Variable(linking, var),
        Directive2::Method(method) => Directive2::Method(run_method(resolver, method, imports)?),
    })
}

fn run_method<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    mut method: Function2<ast::Instruction<SpirvWord>, SpirvWord>,
    imports: &mut Option<FunctionImports>,
) -> Result<Function2<ast::Instruction<SpirvWord>, SpirvWord>, TranslateError> {
    method.body = method.body.map(|body| {
        body.into_iter()
            .flat_map(|stmt| run_statement(resolver, stmt, imports))
            .collect()
    });
    Ok(method)
}

fn run_statement<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    stmt: Statement<ast::Instruction<SpirvWord>, SpirvWord>,
    imports: &mut Option<FunctionImports>,
) -> SmallVec<[Statement<ast::Instruction<SpirvWord>, SpirvWord>; 4]> {
    match stmt {
        Statement::Instruction(ast::Instruction::Div {
            data:
                ast::DivDetails::Float(ast::DivFloatDetails {
                    flush_to_zero,
                    kind: ast::DivFloatKind::Rounding(rnd),
                    type_: ast::ScalarType::F32,
                }),
            arguments,
        }) => {
            let ftz = flush_to_zero.unwrap_or(false);
            let FunctionImports { part1, part2, .. } = FunctionImports::init(imports, resolver);
            let fma_4 = resolver.register_unnamed(Some((
                ast::Type::Scalar(ast::ScalarType::F32),
                ast::StateSpace::Reg,
            )));
            let fma_1 = resolver.register_unnamed(Some((
                ast::Type::Scalar(ast::ScalarType::F32),
                ast::StateSpace::Reg,
            )));
            let fma3_ = resolver.register_unnamed(Some((
                ast::Type::Scalar(ast::ScalarType::F32),
                ast::StateSpace::Reg,
            )));
            let numerator_scaled_flag = resolver.register_unnamed(Some((
                ast::Type::Scalar(ast::ScalarType::U8),
                ast::StateSpace::Reg,
            )));
            smallvec![
                Statement::FpModeRequired {
                    ftz_f32: Some(false),
                    rnd_f32: Some(ast::RoundingMode::NearestEven),
                },
                Statement::Instruction(ast::Instruction::Call {
                    arguments: ast::CallArgs {
                        return_arguments: vec![fma_4, fma_1, fma3_, numerator_scaled_flag],
                        func: *part1,
                        input_arguments: vec![arguments.src1, arguments.src2],
                        is_external: true,
                    },
                    data: ast::CallDetails {
                        uniform: false,
                        return_arguments: vec![
                            (
                                ast::Type::Scalar(ast::ScalarType::F32),
                                ast::StateSpace::Reg,
                            ),
                            (
                                ast::Type::Scalar(ast::ScalarType::F32),
                                ast::StateSpace::Reg,
                            ),
                            (
                                ast::Type::Scalar(ast::ScalarType::F32),
                                ast::StateSpace::Reg,
                            ),
                            (ast::Type::Scalar(ast::ScalarType::U8), ast::StateSpace::Reg,)
                        ],
                        input_arguments: vec![
                            (
                                ast::Type::Scalar(ast::ScalarType::F32),
                                ast::StateSpace::Reg,
                            ),
                            (
                                ast::Type::Scalar(ast::ScalarType::F32),
                                ast::StateSpace::Reg,
                            )
                        ]
                    }
                }),
                Statement::FpModeRequired {
                    ftz_f32: Some(ftz),
                    rnd_f32: Some(rnd),
                },
                Statement::Instruction(ast::Instruction::Call {
                    arguments: ast::CallArgs {
                        return_arguments: vec![arguments.dst],
                        func: *part2,
                        input_arguments: vec![
                            arguments.src1,
                            arguments.src2,
                            fma_4,
                            fma_1,
                            fma3_,
                            numerator_scaled_flag
                        ],
                        is_external: true,
                    },
                    data: ast::CallDetails {
                        uniform: false,
                        return_arguments: vec![(
                            ast::Type::Scalar(ast::ScalarType::F32),
                            ast::StateSpace::Reg,
                        )],
                        input_arguments: vec![
                            (
                                ast::Type::Scalar(ast::ScalarType::F32),
                                ast::StateSpace::Reg,
                            ),
                            (
                                ast::Type::Scalar(ast::ScalarType::F32),
                                ast::StateSpace::Reg,
                            ),
                            (
                                ast::Type::Scalar(ast::ScalarType::F32),
                                ast::StateSpace::Reg,
                            ),
                            (
                                ast::Type::Scalar(ast::ScalarType::F32),
                                ast::StateSpace::Reg,
                            ),
                            (
                                ast::Type::Scalar(ast::ScalarType::F32),
                                ast::StateSpace::Reg,
                            ),
                            (ast::Type::Scalar(ast::ScalarType::U8), ast::StateSpace::Reg,)
                        ]
                    }
                })
            ]
        }
        _ => smallvec![stmt],
    }
}

#[derive(Clone)]
struct FunctionImports {
    part1: SpirvWord,
    part2: SpirvWord,
}

impl FunctionImports {
    fn init<'a>(
        this: &'a mut Option<FunctionImports>,
        resolver: &mut GlobalStringIdentResolver2,
    ) -> &'a FunctionImports {
        this.get_or_insert_with(|| {
            let part1_name = [ZLUDA_PTX_PREFIX, "div_f32_part1"].concat();
            let part1 = resolver.register_named(part1_name.into(), None);
            let part2_name = [ZLUDA_PTX_PREFIX, "div_f32_part2"].concat();
            let part2 = resolver.register_named(part2_name.into(), None);
            FunctionImports { part1, part2 }
        })
    }
}
