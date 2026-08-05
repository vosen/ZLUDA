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
            result.extend(get_functions(resolver, ast::ScalarType::F32, imports.f32));
            result.extend(get_functions(resolver, ast::ScalarType::F64, imports.f64));
            result.extend(directives);
            result
        }
        None => directives,
    })
}

fn get_functions<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    type_: ast::ScalarType,
    (part1, part2): (SpirvWord, SpirvWord),
) -> [Directive2<ptx_parser::Instruction<SpirvWord>, SpirvWord>; 2] {
    [
        Directive2::Method(Function {
            return_arguments: vec![
                ast::Variable {
                    name: resolver
                        .register_unnamed(Some((ast::Type::Scalar(type_), ast::StateSpace::Reg))),
                    info: ast::VariableInfo {
                        align: None,
                        v_type: ast::Type::Scalar(type_),
                        state_space: ast::StateSpace::Reg,
                        array_init: Vec::new(),
                    },
                },
                ast::Variable {
                    name: resolver
                        .register_unnamed(Some((ast::Type::Scalar(type_), ast::StateSpace::Reg))),
                    info: ast::VariableInfo {
                        align: None,
                        v_type: ast::Type::Scalar(type_),
                        state_space: ast::StateSpace::Reg,
                        array_init: Vec::new(),
                    },
                },
                ast::Variable {
                    name: resolver
                        .register_unnamed(Some((ast::Type::Scalar(type_), ast::StateSpace::Reg))),
                    info: ast::VariableInfo {
                        align: None,
                        v_type: ast::Type::Scalar(type_),
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
            name: part1,
            input_arguments: vec![
                ast::Variable {
                    name: resolver
                        .register_unnamed(Some((ast::Type::Scalar(type_), ast::StateSpace::Reg))),
                    info: ast::VariableInfo {
                        align: None,
                        v_type: ast::Type::Scalar(type_),
                        state_space: ast::StateSpace::Reg,
                        array_init: Vec::new(),
                    },
                },
                ast::Variable {
                    name: resolver
                        .register_unnamed(Some((ast::Type::Scalar(type_), ast::StateSpace::Reg))),
                    info: ast::VariableInfo {
                        align: None,
                        v_type: ast::Type::Scalar(type_),
                        state_space: ast::StateSpace::Reg,
                        array_init: Vec::new(),
                    },
                },
            ],
            body: None,
            import_as: None,
            tuning: Vec::new(),
            linkage: ast::LinkingDirective::EXTERN,
            kernel_attributes: None,
            kernel_meta32: None,
        }),
        Directive2::Method(Function {
            return_arguments: vec![ast::Variable {
                name: resolver
                    .register_unnamed(Some((ast::Type::Scalar(type_), ast::StateSpace::Reg))),
                info: ast::VariableInfo {
                    align: None,
                    v_type: ast::Type::Scalar(type_),
                    state_space: ast::StateSpace::Reg,
                    array_init: Vec::new(),
                },
            }],
            name: part2,
            input_arguments: vec![
                ast::Variable {
                    name: resolver
                        .register_unnamed(Some((ast::Type::Scalar(type_), ast::StateSpace::Reg))),
                    info: ast::VariableInfo {
                        align: None,
                        v_type: ast::Type::Scalar(type_),
                        state_space: ast::StateSpace::Reg,
                        array_init: Vec::new(),
                    },
                },
                ast::Variable {
                    name: resolver
                        .register_unnamed(Some((ast::Type::Scalar(type_), ast::StateSpace::Reg))),
                    info: ast::VariableInfo {
                        align: None,
                        v_type: ast::Type::Scalar(type_),
                        state_space: ast::StateSpace::Reg,
                        array_init: Vec::new(),
                    },
                },
                ast::Variable {
                    name: resolver
                        .register_unnamed(Some((ast::Type::Scalar(type_), ast::StateSpace::Reg))),
                    info: ast::VariableInfo {
                        align: None,
                        v_type: ast::Type::Scalar(type_),
                        state_space: ast::StateSpace::Reg,
                        array_init: Vec::new(),
                    },
                },
                ast::Variable {
                    name: resolver
                        .register_unnamed(Some((ast::Type::Scalar(type_), ast::StateSpace::Reg))),
                    info: ast::VariableInfo {
                        align: None,
                        v_type: ast::Type::Scalar(type_),
                        state_space: ast::StateSpace::Reg,
                        array_init: Vec::new(),
                    },
                },
                ast::Variable {
                    name: resolver
                        .register_unnamed(Some((ast::Type::Scalar(type_), ast::StateSpace::Reg))),
                    info: ast::VariableInfo {
                        align: None,
                        v_type: ast::Type::Scalar(type_),
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
            kernel_attributes: None,
            kernel_meta32: None,
        }),
    ]
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
    mut method: Function<ast::Instruction<SpirvWord>, SpirvWord>,
    imports: &mut Option<FunctionImports>,
) -> Result<Function<ast::Instruction<SpirvWord>, SpirvWord>, TranslateError> {
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
                    type_,
                }),
            arguments,
        }) => {
            let ftz = flush_to_zero.unwrap_or(false);
            let (part1, part2) = if type_ == ast::ScalarType::F64 {
                FunctionImports::init(imports, resolver).f64
            } else {
                FunctionImports::init(imports, resolver).f32
            };
            let fma_4 =
                resolver.register_unnamed(Some((ast::Type::Scalar(type_), ast::StateSpace::Reg)));
            let fma_1 =
                resolver.register_unnamed(Some((ast::Type::Scalar(type_), ast::StateSpace::Reg)));
            let fma3_ =
                resolver.register_unnamed(Some((ast::Type::Scalar(type_), ast::StateSpace::Reg)));
            let numerator_scaled_flag = resolver.register_unnamed(Some((
                ast::Type::Scalar(ast::ScalarType::U8),
                ast::StateSpace::Reg,
            )));
            smallvec![
                if type_ == ast::ScalarType::F64 {
                    Statement::FpModeRequired {
                        ftz_f32: None,
                        rnd_f32: None,
                        ftz_f16f64: Some(false),
                        rnd_f16f64: Some(ast::RoundingMode::NearestEven),
                    }
                } else {
                    Statement::FpModeRequired {
                        ftz_f32: Some(false),
                        rnd_f32: Some(ast::RoundingMode::NearestEven),
                        ftz_f16f64: None,
                        rnd_f16f64: None,
                    }
                },
                Statement::Instruction(ast::Instruction::Call {
                    arguments: ast::CallArgs {
                        return_arguments: vec![fma_4, fma_1, fma3_, numerator_scaled_flag],
                        func: part1,
                        input_arguments: vec![arguments.src1, arguments.src2],
                        is_external: true,
                    },
                    data: ast::CallDetails {
                        uniform: false,
                        return_arguments: vec![
                            (ast::Type::Scalar(type_), ast::StateSpace::Reg,),
                            (ast::Type::Scalar(type_), ast::StateSpace::Reg,),
                            (ast::Type::Scalar(type_), ast::StateSpace::Reg,),
                            (ast::Type::Scalar(ast::ScalarType::U8), ast::StateSpace::Reg,)
                        ],
                        input_arguments: vec![
                            (ast::Type::Scalar(type_), ast::StateSpace::Reg,),
                            (ast::Type::Scalar(type_), ast::StateSpace::Reg,)
                        ]
                    }
                }),
                if type_ == ast::ScalarType::F64 {
                    Statement::FpModeRequired {
                        ftz_f32: None,
                        rnd_f32: None,
                        ftz_f16f64: Some(ftz),
                        rnd_f16f64: Some(rnd),
                    }
                } else {
                    Statement::FpModeRequired {
                        ftz_f32: Some(ftz),
                        rnd_f32: Some(rnd),
                        ftz_f16f64: None,
                        rnd_f16f64: None,
                    }
                },
                Statement::Instruction(ast::Instruction::Call {
                    arguments: ast::CallArgs {
                        return_arguments: vec![arguments.dst],
                        func: part2,
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
                        return_arguments: vec![(ast::Type::Scalar(type_), ast::StateSpace::Reg,)],
                        input_arguments: vec![
                            (ast::Type::Scalar(type_), ast::StateSpace::Reg,),
                            (ast::Type::Scalar(type_), ast::StateSpace::Reg,),
                            (ast::Type::Scalar(type_), ast::StateSpace::Reg,),
                            (ast::Type::Scalar(type_), ast::StateSpace::Reg,),
                            (ast::Type::Scalar(type_), ast::StateSpace::Reg,),
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
    f32: (SpirvWord, SpirvWord),
    f64: (SpirvWord, SpirvWord),
}

impl FunctionImports {
    fn init<'a>(
        this: &'a mut Option<FunctionImports>,
        resolver: &mut GlobalStringIdentResolver2,
    ) -> &'a FunctionImports {
        this.get_or_insert_with(|| {
            let f32_part1_name = [ZLUDA_PTX_PREFIX, "div_f32_part1"].concat();
            let f32_part1 = resolver.register_named(f32_part1_name.into(), None);
            let f32_part2_name = [ZLUDA_PTX_PREFIX, "div_f32_part2"].concat();
            let f32_part2 = resolver.register_named(f32_part2_name.into(), None);
            let f64_part1_name = [ZLUDA_PTX_PREFIX, "div_f64_part1"].concat();
            let f64_part1 = resolver.register_named(f64_part1_name.into(), None);
            let f64_part2_name = [ZLUDA_PTX_PREFIX, "div_f64_part2"].concat();
            let f64_part2 = resolver.register_named(f64_part2_name.into(), None);
            FunctionImports {
                f32: (f32_part1, f32_part2),
                f64: (f64_part1, f64_part2),
            }
        })
    }
}
