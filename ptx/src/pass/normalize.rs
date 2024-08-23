use super::*;
use ptx_parser as ast;

type NormalizedStatement = Statement<
    (
        Option<ast::PredAt<SpirvWord>>,
        ast::Instruction<ast::ParsedOperand<SpirvWord>>,
    ),
    ast::ParsedOperand<SpirvWord>,
>;

fn run<'input, 'b>(
    id_defs: &mut FnStringIdResolver<'input, 'b>,
    fn_defs: &GlobalFnDeclResolver<'input, 'b>,
    func: Vec<ast::Statement<ast::ParsedOperand<&'input str>>>,
) -> Result<Vec<NormalizedStatement>, TranslateError> {
    for s in func.iter() {
        match s {
            ast::Statement::Label(id) => {
                id_defs.add_def(*id, None, false);
            }
            _ => (),
        }
    }
    let mut result = Vec::new();
    for s in func {
        expand_map_variables(id_defs, fn_defs, &mut result, s)?;
    }
    Ok(result)
}

fn expand_map_variables<'a, 'b>(
    id_defs: &mut FnStringIdResolver<'a, 'b>,
    fn_defs: &GlobalFnDeclResolver<'a, 'b>,
    result: &mut Vec<NormalizedStatement>,
    s: ast::Statement<ast::ParsedOperand<&'a str>>,
) -> Result<(), TranslateError> {
    match s {
        ast::Statement::Block(block) => {
            id_defs.start_block();
            for s in block {
                expand_map_variables(id_defs, fn_defs, result, s)?;
            }
            id_defs.end_block();
        }
        ast::Statement::Label(name) => result.push(Statement::Label(id_defs.get_id(name)?)),
        ast::Statement::Instruction(p, i) => result.push(Statement::Instruction((
            p.map(|p| pred_map_variable(p, &mut |id| id_defs.get_id(id)))
                .transpose()?,
            op_map_variable(i, &mut |id| id_defs.get_id(id))?,
        ))),
        ast::Statement::Variable(var) => {
            let var_type = var.var.v_type.clone();
            match var.count {
                Some(count) => {
                    for new_id in
                        id_defs.add_defs(var.var.name, count, var_type, var.var.state_space, true)
                    {
                        result.push(Statement::Variable(ast::Variable {
                            align: var.var.align,
                            v_type: var.var.v_type.clone(),
                            state_space: var.var.state_space,
                            name: new_id,
                            array_init: var.var.array_init.clone(),
                        }))
                    }
                }
                None => {
                    let new_id =
                        id_defs.add_def(var.var.name, Some((var_type, var.var.state_space)), true);
                    result.push(Statement::Variable(ast::Variable {
                        align: var.var.align,
                        v_type: var.var.v_type.clone(),
                        state_space: var.var.state_space,
                        name: new_id,
                        array_init: var.var.array_init,
                    }));
                }
            }
        }
    };
    Ok(())
}
