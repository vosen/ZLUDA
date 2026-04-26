use crate::{
    pass::{
        error_todo, Directive2, Function2, GlobalStringIdentResolver2, PtrAccess, SpirvWord,
        Statement,
    },
    TranslateError,
};
use ptx_parser::{self as ast, CvtDetails, LdDetails};

pub(super) fn run<'a, 'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    directives: Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>,
) -> Result<Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>, TranslateError> {
    directives
        .into_iter()
        .map(|directive| run_directive(resolver, directive))
        .collect::<Result<Vec<_>, _>>()
}

fn run_directive<'a, 'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    directive: Directive2<ast::Instruction<SpirvWord>, SpirvWord>,
) -> Result<Directive2<ast::Instruction<SpirvWord>, SpirvWord>, TranslateError> {
    Ok(match directive {
        var @ Directive2::Variable(..) => var,
        Directive2::Method(method) => Directive2::Method(run_method(resolver, method)?),
    })
}

fn run_method<'a, 'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    mut method: Function2<ast::Instruction<SpirvWord>, SpirvWord>,
) -> Result<Function2<ast::Instruction<SpirvWord>, SpirvWord>, TranslateError> {
    let is_kernel = method.is_kernel();
    if let Some(ref mut body) = method.body {
        if !is_kernel {
            return Err(error_todo());
        }
        let hidden_ptr_param = resolver.register_unnamed(Some((
            ast::Type::Scalar(ast::ScalarType::B64),
            ast::StateSpace::ParamEntry,
        )));
        method.input_arguments.push(ast::Variable {
            name: hidden_ptr_param,
            info: ast::VariableInfo {
                align: None,
                v_type: ast::Type::Scalar(ast::ScalarType::B64),
                state_space: ast::StateSpace::ParamEntry,
                array_init: Vec::new(),
            },
        });
        let mut result = Vec::with_capacity(body.len());
        let hidden_ptr = resolver.register_unnamed(Some((
            ast::Type::Scalar(ast::ScalarType::B64),
            ast::StateSpace::Reg,
        )));
        let mut old_body = std::mem::take(body).into_iter().peekable();
        while let Some(Statement::Label(_)) = old_body.peek() {
            result.push(old_body.next().unwrap());
        }
        result.push(Statement::Instruction(ast::Instruction::Ld {
            data: LdDetails {
                qualifier: ast::LdStQualifier::Weak,
                state_space: ast::StateSpace::ParamEntry,
                caching: ast::LdCacheOperator::Cached,
                typ: ast::Type::Scalar(ast::ScalarType::B64),
                non_coherent: false,
            },
            arguments: ast::LdArgs {
                dst: hidden_ptr,
                src: hidden_ptr_param,
            },
        }));
        for statement in old_body {
            match statement {
                Statement::Instruction(ast::Instruction::Ld {
                    data,
                    mut arguments,
                }) => {
                    if is_global_memory(data.state_space) {
                        replace_with_ptr_access(
                            resolver,
                            hidden_ptr,
                            &mut result,
                            (data.state_space, data.typ.clone()),
                            &mut arguments.src,
                        )?;
                    }
                    result.push(Statement::Instruction(ast::Instruction::Ld {
                        data,
                        arguments,
                    }));
                }
                Statement::Instruction(ast::Instruction::St {
                    data,
                    mut arguments,
                }) => {
                    if is_global_memory(data.state_space) {
                        replace_with_ptr_access(
                            resolver,
                            hidden_ptr,
                            &mut result,
                            (data.state_space, data.typ.clone()),
                            &mut arguments.src1,
                        )?;
                    }
                    result.push(Statement::Instruction(ast::Instruction::St {
                        data,
                        arguments,
                    }));
                }
                Statement::PtrAccess(PtrAccess {
                    underlying_type,
                    state_space,
                    dst,
                    mut ptr_src,
                    offset_src,
                }) => {
                    if is_global_memory(state_space) {
                        replace_with_ptr_access(
                            resolver,
                            hidden_ptr,
                            &mut result,
                            (state_space, underlying_type.clone()),
                            &mut ptr_src,
                        )?;
                    }
                    result.push(Statement::PtrAccess(PtrAccess {
                        underlying_type,
                        state_space,
                        dst,
                        ptr_src,
                        offset_src,
                    }));
                }
                s => result.push(s),
            }
        }
        *body = result;
    }
    Ok(method)
}

fn replace_with_ptr_access<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    hidden_ptr: SpirvWord,
    result: &mut Vec<Statement<ptx_parser::Instruction<SpirvWord>, SpirvWord>>,
    (state_space, type_): (ast::StateSpace, ast::Type),
    offset: &mut SpirvWord,
) -> Result<(), TranslateError> {
    // Offset is either a 32 bit scalar or a variable in the right state space
    // If it's a scalar then we bitcast it, if it's a variable it will
    // be resolved later, during implicit conversion
    let (_, offset_space) = resolver.get_typed(*offset)?;
    let converted_src = match offset_space {
        ast::StateSpace::Reg => {
            let zext_dst = resolver.register_unnamed(Some((
                ast::Type::Scalar(ast::ScalarType::B64),
                ast::StateSpace::Reg,
            )));
            let zext = Statement::Instruction(ast::Instruction::Cvt {
                data: CvtDetails {
                    from: ast::ScalarType::B32,
                    to: ast::ScalarType::B64,
                    mode: ast::CvtMode::ZeroExtend,
                },
                arguments: ast::CvtArgs {
                    dst: zext_dst,
                    src: *offset,
                    src2: None,
                },
            });
            result.push(zext);
            zext_dst
        }
        _ => *offset,
    };
    let dst = resolver.register_unnamed(Some((type_.clone(), state_space)));
    let ptr_access = Statement::PtrAccess(PtrAccess {
        underlying_type: type_,
        state_space: state_space,
        dst,
        ptr_src: hidden_ptr,
        offset_src: converted_src,
    });
    result.push(ptr_access);
    *offset = dst;
    Ok(())
}

fn is_global_memory(state_space: ast::StateSpace) -> bool {
    matches!(
        state_space,
        ast::StateSpace::Global | ast::StateSpace::Generic | ast::StateSpace::Const
    )
}
