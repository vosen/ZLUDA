use super::*;
use ptx_parser as ast;
use rustc_hash::FxHashSet;

pub(crate) fn run<'input>(
    directives: Vec<UnconditionalDirective>,
) -> Result<Vec<UnconditionalDirective>, TranslateError> {
    let mut functions = FxHashSet::default();
    directives
        .into_iter()
        .map(|directive| run_directive(&mut functions, directive))
        .collect::<Result<Vec<_>, _>>()
}

fn run_directive<'input>(
    functions: &mut FxHashSet<SpirvWord>,
    directive: UnconditionalDirective,
) -> Result<UnconditionalDirective, TranslateError> {
    Ok(match directive {
        var @ Directive2::Variable(..) => var,
        Directive2::Method(method) => {
            if !method.is_kernel {
                functions.insert(method.name);
            }
            Directive2::Method(run_method(functions, method)?)
        }
    })
}

fn run_method<'input>(
    functions: &mut FxHashSet<SpirvWord>,
    method: UnconditionalFunction,
) -> Result<UnconditionalFunction, TranslateError> {
    let body = method
        .body
        .map(|statements| {
            statements
                .into_iter()
                .map(|statement| run_statement(functions, statement))
                .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?;
    Ok(Function2 { body, ..method })
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
