use super::*;
use ptx_parser as ast;
use rustc_hash::FxHashSet;

pub(crate) fn run<'input>(
    directives: Vec<UnconditionalDirective<'input>>,
) -> Result<Vec<UnconditionalDirective<'input>>, TranslateError> {
    let mut functions = FxHashSet::default();
    directives
        .into_iter()
        .map(|directive| run_directive(&mut functions, directive))
        .collect::<Result<Vec<_>, _>>()
}

fn run_directive<'input>(
    functions: &mut FxHashSet<SpirvWord>,
    directive: UnconditionalDirective<'input>,
) -> Result<UnconditionalDirective<'input>, TranslateError> {
    Ok(match directive {
        var @ Directive2::Variable(..) => var,
        Directive2::Method(method) => {
            {
                let func_decl = method.func_decl.borrow();
                match func_decl.name {
                    ptx_parser::MethodName::Kernel(_) => {}
                    ptx_parser::MethodName::Func(name) => {
                        functions.insert(name);
                    }
                }
            }
            Directive2::Method(run_method(functions, method)?)
        }
    })
}

fn run_method<'input>(
    functions: &mut FxHashSet<SpirvWord>,
    method: UnconditionalFunction<'input>,
) -> Result<UnconditionalFunction<'input>, TranslateError> {
    let body = method
        .body
        .map(|statements| {
            statements
                .into_iter()
                .map(|statement| run_statement(functions, statement))
                .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?;
    Ok(Function2 {
        func_decl: method.func_decl,
        globals: method.globals,
        body,
        import_as: method.import_as,
        tuning: method.tuning,
        linkage: method.linkage,
    })
}

fn run_statement<'input>(
    functions: &mut FxHashSet<SpirvWord>,
    statement: UnconditionalStatement,
) -> Result<UnconditionalStatement, TranslateError> {
    Ok(match statement {
        Statement::Instruction(ast::Instruction::Mov {
            data,
            arguments:
                ast::MovArgs {
                    dst: ast::ParsedOperand::Reg(dst_reg),
                    src: ast::ParsedOperand::Reg(src_reg),
                },
        }) if functions.contains(&src_reg) => {
            if data.typ != ast::Type::Scalar(ast::ScalarType::U64) {
                return Err(error_mismatched_type());
            }
            UnconditionalStatement::FunctionPointer(FunctionPointerDetails {
                dst: dst_reg,
                src: src_reg,
            })
        }
        s => s,
    })
}
