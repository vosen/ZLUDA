use std::{collections::HashSet, iter};

use super::*;

pub(super) fn run(
    func: Vec<ExpandedStatement>,
    id_def: &mut NumericIdResolver,
) -> Vec<ExpandedStatement> {
    let mut labels_in_use = HashSet::new();
    for s in func.iter() {
        match s {
            Statement::Instruction(i) => {
                if let Some(target) = jump_target(i) {
                    labels_in_use.insert(target);
                }
            }
            Statement::Conditional(cond) => {
                labels_in_use.insert(cond.if_true);
                labels_in_use.insert(cond.if_false);
            }
            Statement::Variable(..)
            | Statement::LoadVar(..)
            | Statement::StoreVar(..)
            | Statement::RetValue(..)
            | Statement::Conversion(..)
            | Statement::Constant(..)
            | Statement::Label(..)
            | Statement::PtrAccess { .. }
            | Statement::RepackVector(..)
            | Statement::FunctionPointer(..) => {}
        }
    }
    iter::once(Statement::Label(id_def.register_intermediate(None)))
        .chain(func.into_iter().filter(|s| match s {
            Statement::Label(i) => labels_in_use.contains(i),
            _ => true,
        }))
        .collect::<Vec<_>>()
}

fn jump_target<T: ast::Operand<Ident = SpirvWord>>(
    this: &ast::Instruction<T>,
) -> Option<SpirvWord> {
    match this {
        ast::Instruction::Bra { arguments } => Some(arguments.src),
        _ => None,
    }
}
