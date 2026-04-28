use crate::{
    pass::{
        error_todo, error_unreachable, Directive2, Function, GlobalStringIdentResolver2, Module32,
        PtrAccess, SpirvWord, Statement,
    },
    TranslateError,
};
use ptx_parser::{self as ast, CvtDetails, LdDetails, RegOrImmediate};
use rustc_hash::FxHashMap;
use std::alloc::Layout;

pub(super) fn run<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    directives: Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>,
) -> Result<
    (
        Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>,
        Module32,
    ),
    TranslateError,
> {
    let globals = collect_globals(resolver, &directives)?;
    let directives = directives
        .into_iter()
        .map(|directive| run_directive(resolver, directive, &globals))
        .collect::<Result<Vec<_>, _>>()?;
    let globals = globals
        .into_iter()
        .map(|(_, name, layout)| (name, layout))
        .collect();
    Ok((directives, Module32 { globals }))
}

pub(super) fn collect_globals<'a, 'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    directives: &[Directive2<ast::Instruction<SpirvWord>, SpirvWord>],
) -> Result<Vec<(SpirvWord, String, Layout)>, TranslateError> {
    directives
        .iter()
        .filter_map(|directive| match directive {
            Directive2::Variable(
                _,
                ast::Variable {
                    info:
                        ast::VariableInfo {
                            align,
                            v_type:
                                ast::Type::Array(
                                    None,
                                    ast::ScalarType::U8 | ast::ScalarType::B8 | ast::ScalarType::S8,
                                    array_dims,
                                ),
                            state_space: ast::StateSpace::Const | ast::StateSpace::Global,
                            array_init,
                        },
                    name,
                },
            ) if array_dims.len() == 1
                && array_init.iter().all(|x| {
                    matches!(
                        x,
                        RegOrImmediate::Imm(ptx_parser::ImmediateValue::U64(0))
                            | RegOrImmediate::Imm(ptx_parser::ImmediateValue::S64(0))
                    )
                }) =>
            {
                Some(get_name_layout(resolver, align, array_dims, name))
            }
            Directive2::Variable(
                _,
                ast::Variable {
                    info:
                        ast::VariableInfo {
                            state_space: ast::StateSpace::Shared,
                            ..
                        },
                    ..
                },
            )
            | Directive2::Variable(
                _,
                ast::Variable {
                    info:
                        ast::VariableInfo {
                            v_type: ast::Type::Texref,
                            ..
                        },
                    ..
                },
            )
            | Directive2::Method(..) => None,
            _ => Some(Err(error_todo())),
        })
        .collect::<Result<Vec<_>, _>>()
}

fn get_name_layout<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    align: &Option<u32>,
    array_dims: &Vec<u32>,
    name: &SpirvWord,
) -> Result<(SpirvWord, String, Layout), TranslateError> {
    let entry = resolver.ident_map.get(name).ok_or_else(error_unreachable)?;
    let text_name = entry
        .name
        .as_ref()
        .ok_or_else(error_unreachable)?
        .to_string();
    Ok((
        *name,
        text_name,
        Layout::from_size_align(array_dims[0] as usize, align.unwrap_or(1) as usize)
            .map_err(|_| error_unreachable())?,
    ))
}

fn run_directive<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    directive: Directive2<ast::Instruction<SpirvWord>, SpirvWord>,
    globals: &[(SpirvWord, String, Layout)],
) -> Result<Directive2<ast::Instruction<SpirvWord>, SpirvWord>, TranslateError> {
    Ok(match directive {
        var @ Directive2::Variable(..) => var,
        Directive2::Method(method) => Directive2::Method(run_method(resolver, method, globals)?),
    })
}

fn run_method<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    mut method: Function<ast::Instruction<SpirvWord>, SpirvWord>,
    globals: &[(SpirvWord, String, Layout)],
) -> Result<Function<ast::Instruction<SpirvWord>, SpirvWord>, TranslateError> {
    let is_kernel = method.is_kernel();
    if let Some(ref mut body) = method.body {
        if !is_kernel {
            return Err(error_todo());
        }
        let mut result = Vec::with_capacity(body.len());
        let mut old_body = std::mem::take(body).into_iter().peekable();
        while let Some(Statement::Label(_)) = old_body.peek() {
            result.push(old_body.next().unwrap());
        }
        let hidden_ptr = add_hidden_argument(
            resolver,
            ast::ScalarType::B64,
            &mut method.input_arguments,
            &mut result,
        );
        let globals_remapping = globals
            .iter()
            .map(|(name, _, _)| {
                (
                    *name,
                    add_hidden_argument(
                        resolver,
                        ast::ScalarType::B32,
                        &mut method.input_arguments,
                        &mut result,
                    ),
                )
            })
            .collect::<FxHashMap<_, _>>();
        for statement in old_body {
            run_statement(
                resolver,
                &mut result,
                hidden_ptr,
                &globals_remapping,
                statement,
            )?;
        }
        *body = result;
    }
    Ok(method)
}

fn run_statement<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    result: &mut Vec<Statement<ptx_parser::Instruction<SpirvWord>, SpirvWord>>,
    hidden_ptr: SpirvWord,
    globals_remapping: &FxHashMap<SpirvWord, SpirvWord>,
    statement: Statement<ptx_parser::Instruction<SpirvWord>, SpirvWord>,
) -> Result<(), TranslateError> {
    let statement =
        statement.visit_map(&mut |arg: SpirvWord,
                                   _type_space: Option<(&ast::Type, ast::StateSpace)>,
                                   _is_dst: bool,
                                   _relaxed_type_check: bool| {
            Ok(globals_remapping.get(&arg).copied().unwrap_or(arg))
        })?;
    Ok(match statement {
        Statement::Instruction(ast::Instruction::Ld {
            data,
            mut arguments,
        }) => {
            if is_global_memory(data.state_space) {
                replace_with_ptr_access(
                    resolver,
                    hidden_ptr,
                    result,
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
                    result,
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
                    result,
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
        Statement::Instruction(ast::Instruction::Atom {
            data,
            mut arguments,
        }) => {
            if is_global_memory(data.space) {
                replace_with_ptr_access(
                    resolver,
                    hidden_ptr,
                    result,
                    (data.space, data.type_.clone()),
                    &mut arguments.src1,
                )?;
            }
            result.push(Statement::Instruction(ast::Instruction::Atom {
                data,
                arguments,
            }));
        }
        s => result.push(s),
    })
}

fn add_hidden_argument<'input>(
    resolver: &mut GlobalStringIdentResolver2<'input>,
    type_: ast::ScalarType,
    input_arguments: &mut Vec<ast::Variable<SpirvWord>>,
    body: &mut Vec<Statement<ast::Instruction<SpirvWord>, SpirvWord>>,
) -> SpirvWord {
    let implicit_argument_param = resolver.register_unnamed(Some((
        ast::Type::Scalar(type_),
        ast::StateSpace::ParamEntry,
    )));
    input_arguments.push(ast::Variable {
        name: implicit_argument_param,
        info: ast::VariableInfo {
            align: None,
            v_type: ast::Type::Scalar(type_),
            state_space: ast::StateSpace::ParamEntry,
            array_init: Vec::new(),
        },
    });
    let implicit_argument =
        resolver.register_unnamed(Some((ast::Type::Scalar(type_), ast::StateSpace::Reg)));
    body.push(Statement::Instruction(ast::Instruction::Ld {
        data: LdDetails {
            qualifier: ast::LdStQualifier::Weak,
            state_space: ast::StateSpace::ParamEntry,
            caching: ast::LdCacheOperator::Cached,
            typ: ast::Type::Scalar(type_),
            non_coherent: false,
        },
        arguments: ast::LdArgs {
            dst: implicit_argument,
            src: implicit_argument_param,
        },
    }));
    implicit_argument
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
