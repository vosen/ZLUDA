use super::*;
use ptx_parser as ast;

pub(crate) fn run<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    directives: Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>,
) -> Result<Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>, TranslateError> {
    directives
        .into_iter()
        .map(|directive| run_directive(resolver, directive))
        .collect::<Result<Vec<_>, _>>()
}

fn run_directive<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    directive: Directive2<ast::Instruction<SpirvWord>, SpirvWord>,
) -> Result<Directive2<ast::Instruction<SpirvWord>, SpirvWord>, TranslateError> {
    Ok(match directive {
        Directive2::Variable(linking, var) => Directive2::Variable(linking, var),
        Directive2::Method(method) => Directive2::Method(run_method(resolver, method)?),
    })
}

fn run_method<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    method: Function<ast::Instruction<SpirvWord>, SpirvWord>,
) -> Result<Function<ast::Instruction<SpirvWord>, SpirvWord>, TranslateError> {
    let body = method
        .body
        .map(|statements| {
            let mut result = Vec::with_capacity(statements.len());
            for statement in statements {
                match statement {
                    Statement::Instruction(ast::Instruction::Rcp {
                        data:
                            ast::RcpData {
                                type_,
                                kind: ast::RcpKind::Compliant(rnd),
                                flush_to_zero,
                            },
                        arguments,
                    }) => {
                        let one = resolver.register_unnamed(Some((
                            ast::Type::Scalar(type_),
                            ast::StateSpace::Reg,
                        )));
                        result.push(Statement::Constant(ConstantDefinition {
                            dst: one,
                            typ: type_,
                            value: ast::ImmediateValue::F64(1.0f64),
                        }));
                        result.push(Statement::Instruction(ast::Instruction::Div {
                            data: ast::DivDetails::Float(ast::DivFloatDetails {
                                type_: type_,
                                kind: ast::DivFloatKind::Rounding(rnd),
                                flush_to_zero,
                            }),
                            arguments: ast::DivArgs {
                                dst: arguments.dst,
                                src1: one,
                                src2: arguments.src,
                            },
                        }));
                    }
                    s => result.push(s),
                }
            }
            Ok::<_, TranslateError>(result)
        })
        .transpose()?;
    Ok(Function {
        body,
        return_arguments: method.return_arguments,
        name: method.name,
        input_arguments: method.input_arguments,
        import_as: method.import_as,
        tuning: method.tuning,
        linkage: method.linkage,
        kernel_attributes: method.kernel_attributes,
        kernel_meta32: method.kernel_meta32,
    })
}
