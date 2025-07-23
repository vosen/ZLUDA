use super::*;
use ptx_parser as ast;

pub(super) fn run<'a, 'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    directives: Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>,
) -> Result<Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>, TranslateError> {
    directives
        .into_iter()
        .map(|directive| run_directive(resolver, directive))
        .collect::<Result<Vec<_>, _>>()
}

fn run_directive<'input>(
    resolver: &mut GlobalStringIdentResolver2,
    directive: Directive2<ast::Instruction<SpirvWord>, SpirvWord>,
) -> Result<Directive2<ast::Instruction<SpirvWord>, SpirvWord>, TranslateError> {
    Ok(match directive {
        var @ Directive2::Variable(..) => var,
        Directive2::Method(method) => Directive2::Method(run_method(resolver, method)?),
    })
}

fn run_method<'input>(
    resolver: &mut GlobalStringIdentResolver2,
    method: Function2<ast::Instruction<SpirvWord>, SpirvWord>,
) -> Result<Function2<ast::Instruction<SpirvWord>, SpirvWord>, TranslateError> {
    let mut new_statements = Vec::new();
    let body = method
        .body
        .map(|statements| {
            for statement in statements {
                run_statement(resolver, &mut new_statements, statement)?;
            }
            Ok::<_, TranslateError>(new_statements)
        })
        .transpose()?;
    Ok(Function2 { body, ..method })
}

fn run_statement<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    result: &mut Vec<Statement<ast::Instruction<SpirvWord>, SpirvWord>>,
    statement: Statement<ast::Instruction<SpirvWord>, SpirvWord>,
) -> Result<(), TranslateError> {
    match statement {
        Statement::Instruction(inst) => run_instruction(resolver, result, inst)?,
        statement => {
            result.push(statement);
        }
    }
    Ok(())
}

fn run_instruction<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    result: &mut Vec<Statement<ast::Instruction<SpirvWord>, SpirvWord>>,
    mut instruction: ast::Instruction<SpirvWord>,
) -> Result<(), TranslateError> {
    match instruction {
        ast::Instruction::Abs { .. }
        | ast::Instruction::Activemask { .. }
        | ast::Instruction::Add {
            data:
                ast::ArithDetails::Float(ast::ArithFloat {
                    saturate: false, ..
                }),
            ..
        }
        | ast::Instruction::Add {
            data: ast::ArithDetails::Integer(..),
            ..
        }
        | ast::Instruction::And { .. }
        | ast::Instruction::Atom { .. }
        | ast::Instruction::AtomCas { .. }
        | ast::Instruction::Bar { .. }
        | ast::Instruction::BarRed { .. }
        | ast::Instruction::Bfe { .. }
        | ast::Instruction::Bfi { .. }
        | ast::Instruction::Bra { .. }
        | ast::Instruction::Brev { .. }
        | ast::Instruction::Call { .. }
        | ast::Instruction::Clz { .. }
        | ast::Instruction::Cos { .. }
        | ast::Instruction::Cvt {
            data:
                ast::CvtDetails {
                    mode:
                        ast::CvtMode::ZeroExtend
                        | ast::CvtMode::SignExtend
                        | ast::CvtMode::Truncate
                        | ast::CvtMode::Bitcast
                        | ast::CvtMode::IntSaturateToSigned
                        | ast::CvtMode::IntSaturateToUnsigned
                        | ast::CvtMode::SignedFromFP { .. }
                        | ast::CvtMode::UnsignedFromFP { .. }
                        | ast::CvtMode::FPFromSigned {
                            saturate: false, ..
                        }
                        | ast::CvtMode::FPFromUnsigned {
                            saturate: false, ..
                        }
                        | ast::CvtMode::FPExtend {
                            saturate: false, ..
                        }
                        | ast::CvtMode::FPTruncate {
                            saturate: false, ..
                        }
                        | ast::CvtMode::FPRound {
                            saturate: false, ..
                        },
                    ..
                },
            ..
        }
        | ast::Instruction::Cvta { .. }
        | ast::Instruction::Div { .. }
        | ast::Instruction::Ex2 { .. }
        | ast::Instruction::Fma {
            data: ast::ArithFloat {
                saturate: false, ..
            },
            ..
        }
        | ast::Instruction::Ld { .. }
        | ast::Instruction::Lg2 { .. }
        | ast::Instruction::Mad {
            data:
                ast::MadDetails::Float(ast::ArithFloat {
                    saturate: false, ..
                }),
            ..
        }
        | ast::Instruction::Mad {
            data: ast::MadDetails::Integer { .. },
            ..
        }
        | ast::Instruction::Max { .. }
        | ast::Instruction::Membar { .. }
        | ast::Instruction::Min { .. }
        | ast::Instruction::Mov { .. }
        | ast::Instruction::Mul {
            data:
                ast::MulDetails::Float(ast::ArithFloat {
                    saturate: false, ..
                }),
            ..
        }
        | ast::Instruction::Mul {
            data: ast::MulDetails::Integer { .. },
            ..
        }
        | ast::Instruction::Mul24 { .. }
        | ast::Instruction::Neg { .. }
        | ast::Instruction::Not { .. }
        | ast::Instruction::Or { .. }
        | ast::Instruction::Popc { .. }
        | ast::Instruction::Prmt { .. }
        | ast::Instruction::PrmtSlow { .. }
        | ast::Instruction::Rcp { .. }
        | ast::Instruction::Rem { .. }
        | ast::Instruction::Ret { .. }
        | ast::Instruction::Rsqrt { .. }
        | ast::Instruction::Selp { .. }
        | ast::Instruction::Set { .. }
        | ast::Instruction::SetBool { .. }
        | ast::Instruction::Setp { .. }
        | ast::Instruction::SetpBool { .. }
        | ast::Instruction::ShflSync { .. }
        | ast::Instruction::Shl { .. }
        | ast::Instruction::Shr { .. }
        | ast::Instruction::Sin { .. }
        | ast::Instruction::Sqrt { .. }
        | ast::Instruction::St { .. }
        | ast::Instruction::Sub {
            data:
                ast::ArithDetails::Float(ast::ArithFloat {
                    saturate: false, ..
                }),
            ..
        }
        | ast::Instruction::Sub {
            data: ast::ArithDetails::Integer(..),
            ..
        }
        | ast::Instruction::Tanh { .. }
        | ast::Instruction::Trap {}
        | ast::Instruction::Xor { .. } => result.push(Statement::Instruction(instruction)),
        ast::Instruction::Add {
            data:
                ast::ArithDetails::Float(ast::ArithFloat {
                    saturate: true,
                    type_,
                    ..
                }),
            arguments: ast::AddArgs { ref mut dst, .. },
        }
        | ast::Instruction::Fma {
            data:
                ast::ArithFloat {
                    saturate: true,
                    type_,
                    ..
                },
            arguments: ast::FmaArgs { ref mut dst, .. },
        }
        | ast::Instruction::Mad {
            data:
                ast::MadDetails::Float(ast::ArithFloat {
                    saturate: true,
                    type_,
                    ..
                }),
            arguments: ast::MadArgs { ref mut dst, .. },
        }
        | ast::Instruction::Mul {
            data:
                ast::MulDetails::Float(ast::ArithFloat {
                    saturate: true,
                    type_,
                    ..
                }),
            arguments: ast::MulArgs { ref mut dst, .. },
        }
        | ast::Instruction::Sub {
            data:
                ast::ArithDetails::Float(ast::ArithFloat {
                    saturate: true,
                    type_,
                    ..
                }),
            arguments: ast::SubArgs { ref mut dst, .. },
        }
        | ast::Instruction::Cvt {
            data:
                ast::CvtDetails {
                    to: type_,
                    mode: ast::CvtMode::FPExtend { saturate: true, .. },
                    ..
                },
            arguments: ast::CvtArgs { ref mut dst, .. },
        }
        | ast::Instruction::Cvt {
            data:
                ast::CvtDetails {
                    to: type_,
                    mode: ast::CvtMode::FPTruncate { saturate: true, .. },
                    ..
                },
            arguments: ast::CvtArgs { ref mut dst, .. },
        }
        | ast::Instruction::Cvt {
            data:
                ast::CvtDetails {
                    to: type_,
                    mode: ast::CvtMode::FPRound { saturate: true, .. },
                    ..
                },
            arguments: ast::CvtArgs { ref mut dst, .. },
        }
        | ast::Instruction::Cvt {
            data:
                ast::CvtDetails {
                    to: type_,
                    mode: ast::CvtMode::FPFromSigned { saturate: true, .. },
                    ..
                },
            arguments: ast::CvtArgs { ref mut dst, .. },
        }
        | ast::Instruction::Cvt {
            data:
                ast::CvtDetails {
                    to: type_,
                    mode: ast::CvtMode::FPFromUnsigned { saturate: true, .. },
                    ..
                },
            arguments: ast::CvtArgs { ref mut dst, .. },
        } => {
            let sat = get_post_saturation(resolver, type_, dst)?;
            result.push(Statement::Instruction(instruction));
            result.push(sat);
        }
    }
    Ok(())
}

fn get_post_saturation<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    type_: ast::ScalarType,
    old_dst: &mut SpirvWord,
) -> Result<Statement<ast::Instruction<SpirvWord>, SpirvWord>, TranslateError> {
    let post_sat = resolver.register_unnamed(Some((type_.into(), ast::StateSpace::Reg)));
    let dst = *old_dst;
    *old_dst = post_sat;
    Ok(Statement::FpSaturate {
        dst,
        src: post_sat,
        type_,
    })
}
