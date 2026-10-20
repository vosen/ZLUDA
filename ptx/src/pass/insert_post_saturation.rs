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
    method: Function<ast::Instruction<SpirvWord>, SpirvWord>,
) -> Result<Function<ast::Instruction<SpirvWord>, SpirvWord>, TranslateError> {
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
    Ok(Function { body, ..method })
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
        ast::Instruction::MixedFloat { data, arguments } => {
            return expand_mixed_float(resolver, result, data, arguments);
        }
        ast::Instruction::Sqrt {
            data: ast::RcpData { saturate: true, type_, .. },
            arguments: ast::SqrtArgs { ref mut dst, .. },
        } => {
            let sat = get_post_saturation(resolver, type_, dst)?;
            result.push(Statement::Instruction(instruction));
            result.push(sat);
            return Ok(());
        }
        ast::Instruction::Div {
            data: ast::DivDetails::Float(ast::DivFloatDetails { saturate: true, type_, .. }),
            arguments: ast::DivArgs { ref mut dst, .. },
        } => {
            let sat = get_post_saturation(resolver, type_, dst)?;
            result.push(Statement::Instruction(instruction));
            result.push(sat);
            return Ok(());
        }
        ast::Instruction::Add { data: ast::ArithDetails::Float(ref mut f), .. }
        | ast::Instruction::Sub { data: ast::ArithDetails::Float(ref mut f), .. }
        | ast::Instruction::Mul { data: ast::MulDetails::Float(ref mut f), .. }
        | ast::Instruction::Fma { data: ref mut f, .. }
            if f.flush_to_zero == Some(true)
                && matches!(f.type_, ast::ScalarType::BF16 | ast::ScalarType::BF16x2) =>
        {
            let type_ = f.type_;
            f.flush_to_zero = None;
            return expand_bf16_ftz(resolver, result, type_, instruction);
        }
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
        | ast::Instruction::AddExtended { .. }
        | ast::Instruction::SubExtended { .. }
        | ast::Instruction::MadExtended { .. }
        | ast::Instruction::And { .. }
        | ast::Instruction::Atom { .. }
        | ast::Instruction::Red { .. }
        | ast::Instruction::AtomCas { .. }
        | ast::Instruction::BarWarp { .. }
        | ast::Instruction::Bar { .. }
        | ast::Instruction::BarRed { .. }
        | ast::Instruction::Bfe { .. }
        | ast::Instruction::Bfi { .. }
        | ast::Instruction::Bmsk { .. }
        | ast::Instruction::Bra { .. }
        | ast::Instruction::Brev { .. }
        | ast::Instruction::Call { .. }
        | ast::Instruction::Clz { .. }
        | ast::Instruction::Bfind { .. }
        | ast::Instruction::Cos { .. }
        | ast::Instruction::Copysign { .. }
        | ast::Instruction::CpAsync { .. }
        | ast::Instruction::CpAsyncCommitGroup { .. }
        | ast::Instruction::CpAsyncWaitGroup { .. }
        | ast::Instruction::CpAsyncWaitAll { .. }
        | ast::Instruction::CpAsyncMbarrierArrive { .. }
        | ast::Instruction::CpAsyncBulk { .. }
        | ast::Instruction::CpAsyncBulkTensor { .. }
        | ast::Instruction::TensormapReplace { .. }
        | ast::Instruction::TexAgx { .. }
        | ast::Instruction::TexQuery { .. }
        | ast::Instruction::SuldAgx { .. }
        | ast::Instruction::SustAgx { .. }
        | ast::Instruction::SuredAgx { .. }
        | ast::Instruction::Istypep { .. }
        | ast::Instruction::CpReduceAsyncBulk { .. }
        | ast::Instruction::MbarrierInit { .. }
        | ast::Instruction::MbarrierInval { .. }
        | ast::Instruction::MbarrierArrive { .. }
        | ast::Instruction::MbarrierTx { .. }
        | ast::Instruction::MbarrierWait { .. }
        | ast::Instruction::MbarrierPendingCount { .. }
        | ast::Instruction::CreatePolicyFractional { .. }
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
        | ast::Instruction::CvtPack { .. }
        | ast::Instruction::CvtTf32 { .. }
        | ast::Instruction::CvtMini { .. }
        | ast::Instruction::CvtNarrow { .. }
        | ast::Instruction::Spcompress { .. }
        | ast::Instruction::Spdecompress { .. }
        | ast::Instruction::Tcgen05Sync { .. }
        | ast::Instruction::Tcgen05Alloc { .. }
        | ast::Instruction::Tcgen05Dealloc { .. }
        | ast::Instruction::Tcgen05Ld { .. }
        | ast::Instruction::Tcgen05St { .. }
        | ast::Instruction::Tcgen05Mma { .. }
        | ast::Instruction::Tcgen05Cp { .. }
        | ast::Instruction::Tcgen05Shift { .. }
        | ast::Instruction::Tcgen05Commit { .. }
        | ast::Instruction::Tcgen05LdRed { .. }
        | ast::Instruction::Isspacep { .. }
        | ast::Instruction::Packed8 { .. }
        | ast::Instruction::Clmad { .. }
        | ast::Instruction::VideoScalar { .. }
        | ast::Instruction::VideoSimd { .. }
        | ast::Instruction::Vmad { .. }
        | ast::Instruction::Alloca { .. }
        | ast::Instruction::StackSave { .. }
        | ast::Instruction::StackRestore { .. }
        | ast::Instruction::Cvta { .. }
        | ast::Instruction::Div {
            data: ast::DivDetails::Unsigned(..) | ast::DivDetails::Signed(..),
            ..
        }
        | ast::Instruction::Div {
            data: ast::DivDetails::Float(ast::DivFloatDetails { saturate: false, .. }),
            ..
        }
        | ast::Instruction::Dp4a { .. }
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
        | ast::Instruction::Nanosleep { .. }
        | ast::Instruction::Neg { .. }
        | ast::Instruction::Not { .. }
        | ast::Instruction::Or { .. }
        | ast::Instruction::Popc { .. }
        | ast::Instruction::Prmt { .. }
        | ast::Instruction::Szext { .. }
        | ast::Instruction::Lop3 { .. }
        | ast::Instruction::Mad24 { .. }
        | ast::Instruction::Mapa { .. }
        | ast::Instruction::Getctarank { .. }
        | ast::Instruction::BarrierCluster { .. }
        | ast::Instruction::StAsync { .. }
        | ast::Instruction::StBulk { .. }
        | ast::Instruction::SetMaxNReg { .. }
        | ast::Instruction::ClusterLaunchTryCancel { .. }
        | ast::Instruction::ClusterLaunchQuery { .. }
        | ast::Instruction::Slct { .. }
        | ast::Instruction::Testp { .. }
        | ast::Instruction::MatchAllSync { .. }
        | ast::Instruction::ElectSync { .. }
        | ast::Instruction::Fns { .. }
        | ast::Instruction::Rcp { .. }
        | ast::Instruction::Rem { .. }
        | ast::Instruction::Ret { .. }
        | ast::Instruction::Rsqrt { .. }
        | ast::Instruction::Selp { .. }
        | ast::Instruction::Set { .. }
        | ast::Instruction::SetBool { .. }
        | ast::Instruction::Setp { .. }
        | ast::Instruction::SetpBool { .. }
        | ast::Instruction::Shfl { .. }
        | ast::Instruction::MatchSync { .. }
        | ast::Instruction::Shf { .. }
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
        | ast::Instruction::Exit {}
        | ast::Instruction::Xor { .. }
        | ast::Instruction::Vote { .. }
        | ast::Instruction::ReduxSync { .. }
        | ast::Instruction::GridDepControl { .. }
        | ast::Instruction::LdMatrix { .. }
        | ast::Instruction::StMatrix { .. }
        | ast::Instruction::MmaSparse { .. }
        | ast::Instruction::MmaBlockScale { .. }
        | ast::Instruction::MovMatrix { .. }
        | ast::Instruction::WmmaLoad { .. }
        | ast::Instruction::WmmaStore { .. }
        | ast::Instruction::WmmaMma { .. }
        | ast::Instruction::Prefetch { .. }
        | ast::Instruction::Sad { .. }
        | ast::Instruction::Dp2a { .. }
        | ast::Instruction::Tex { .. }
        | ast::Instruction::Mma { .. }
        | ast::Instruction::Wgmma { .. }
        | ast::Instruction::WgmmaFence { .. }
        | ast::Instruction::Vshr { .. } => result.push(Statement::Instruction(instruction)),
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

// The mixed-precision forms: the 16-bit operands are converted to f32 (exactly) and the operation is the f32 one,
// whose .sat is then handled like any other. A 16-bit pair result of f32x2 operands is computed with rz and rounded
// to the pair type with rz; mul of two different 16-bit pair types converts c to the type of a.
fn expand_mixed_float<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    result: &mut Vec<Statement<ast::Instruction<SpirvWord>, SpirvWord>>,
    data: ast::MixedFloatDetails,
    arguments: ast::MixedFloatArgs<SpirvWord>,
) -> Result<(), TranslateError> {
    let narrow_pair = |t| matches!(t, ast::ScalarType::F16x2 | ast::ScalarType::BF16x2);
    let narrowed = narrow_pair(data.dtype) && data.atype == ast::ScalarType::F32x2;
    let (compute, rounding) = if narrowed {
        (ast::ScalarType::F32x2, ast::RoundingMode::Zero)
    } else {
        (data.dtype, data.rounding)
    };
    let mut convert = |src: SpirvWord, from: ast::ScalarType, to: ast::ScalarType, mode: ast::CvtMode| {
        if from == to {
            return src;
        }
        let dst = resolver.register_unnamed(Some((to.into(), ast::StateSpace::Reg)));
        result.push(Statement::Instruction(ast::Instruction::Cvt {
            data: ast::CvtDetails { from, to, mode },
            arguments: ast::CvtArgs { dst, src, src2: None },
        }));
        dst
    };
    let extend = ast::CvtMode::FPExtend {
        flush_to_zero: None,
        saturate: false,
        relu: false,
        satfinite: false,
    };
    let truncate = |rounding| ast::CvtMode::FPTruncate {
        rounding,
        is_integer_rounding: false,
        flush_to_zero: None,
        saturate: false,
        relu: false,
        satfinite: false,
    };
    let src1 = convert(arguments.src1, data.atype, compute, extend);
    let src2 = if narrow_pair(compute) {
        // f16x2 <-> bf16x2: neither is wider
        convert(arguments.src2, data.btype, compute, truncate(ast::RoundingMode::NearestEven))
    } else {
        convert(arguments.src2, data.btype, compute, extend)
    };
    let float = ast::ArithFloat {
        type_: compute,
        rounding,
        flush_to_zero: Some(data.flush_to_zero),
        saturate: data.saturate,
        is_fusable: false,
        relu: false,
    };
    let dst = if narrowed {
        resolver.register_unnamed(Some((compute.into(), ast::StateSpace::Reg)))
    } else {
        arguments.dst
    };
    let instruction = match (data.op, arguments.src3) {
        (ast::MixedFloatOp::Add, None) => ast::Instruction::Add {
            data: ast::ArithDetails::Float(float),
            arguments: ast::AddArgs { dst, src1, src2 },
        },
        (ast::MixedFloatOp::Sub, None) => ast::Instruction::Sub {
            data: ast::ArithDetails::Float(float),
            arguments: ast::SubArgs { dst, src1, src2 },
        },
        (ast::MixedFloatOp::Mul, None) => ast::Instruction::Mul {
            data: ast::MulDetails::Float(float),
            arguments: ast::MulArgs { dst, src1, src2 },
        },
        (ast::MixedFloatOp::Fma, Some(src3)) => ast::Instruction::Fma {
            data: float,
            arguments: ast::FmaArgs { dst, src1, src2, src3 },
        },
        _ => return Err(error_unreachable()),
    };
    run_instruction(resolver, result, instruction)?;
    if narrowed {
        result.push(Statement::Instruction(ast::Instruction::Cvt {
            data: ast::CvtDetails {
                from: compute,
                to: data.dtype,
                mode: truncate(ast::RoundingMode::Zero),
            },
            arguments: ast::CvtArgs { dst: arguments.dst, src: dst, src2: None },
        }));
    }
    Ok(())
}

// bf16 has no denormal mode: .ftz flushes the operands and the result (after .sat) explicitly
fn expand_bf16_ftz<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    result: &mut Vec<Statement<ast::Instruction<SpirvWord>, SpirvWord>>,
    type_: ast::ScalarType,
    mut instruction: ast::Instruction<SpirvWord>,
) -> Result<(), TranslateError> {
    let mut flush = |result: &mut Vec<_>, src: &mut SpirvWord| {
        let dst = resolver.register_unnamed(Some((type_.into(), ast::StateSpace::Reg)));
        result.push(Statement::FpFlush { dst, src: *src, type_ });
        *src = dst;
    };
    let (dst, sources) = match &mut instruction {
        ast::Instruction::Add { arguments: ast::AddArgs { dst, src1, src2 }, .. }
        | ast::Instruction::Sub { arguments: ast::SubArgs { dst, src1, src2 }, .. }
        | ast::Instruction::Mul { arguments: ast::MulArgs { dst, src1, src2 }, .. } => (dst, vec![src1, src2]),
        ast::Instruction::Fma { arguments: ast::FmaArgs { dst, src1, src2, src3 }, .. } => {
            (dst, vec![src1, src2, src3])
        }
        _ => return Err(error_unreachable()),
    };
    for src in sources {
        flush(result, src);
    }
    let final_dst = *dst;
    let unflushed = resolver.register_unnamed(Some((type_.into(), ast::StateSpace::Reg)));
    *dst = unflushed;
    run_instruction(resolver, result, instruction)?;
    result.push(Statement::FpFlush { dst: final_dst, src: unflushed, type_ });
    Ok(())
}
