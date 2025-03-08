use super::*;

// This pass normalized ptx modules in two ways that makes mode computation pass
// and code emissions passes much simpler:
// * Inserts label at the start of every function
//   This makes control flow graph simpler in mode computation block: we can
//   represent kernels as separate nodes with its own separate entry/exit mode
// * Inserts label at the start of every basic block

pub(crate) fn run(
    flat_resolver: &mut GlobalStringIdentResolver2<'_>,
    mut directives: Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>,
) -> Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>> {
    for directive in directives.iter_mut() {
        let body_ref = match directive {
            Directive2::Method(Function2 {
                body: Some(body), ..
            }) => body,
            _ => continue,
        };
        let body = std::mem::replace(body_ref, Vec::new());
        let mut result = Vec::with_capacity(body.len());
        let mut needs_label = false;
        let mut body_iterator = body.into_iter();
        match body_iterator.next() {
            Some(Statement::Label(_)) => {}
            Some(statement) => {
                result.push(Statement::Label(flat_resolver.register_unnamed(None)));
                result.push(statement);
            }
            None => {}
        }
        for statement in body_iterator {
            if needs_label && !matches!(statement, Statement::Label(..)) {
                result.push(Statement::Label(flat_resolver.register_unnamed(None)));
            }
            needs_label = is_block_terminator(&statement);
            result.push(statement);
        }
        *body_ref = result;
    }
    directives
}

fn is_block_terminator(instruction: &Statement<ast::Instruction<SpirvWord>, SpirvWord>) -> bool {
    match instruction {
        Statement::Conditional(..)
        | Statement::Instruction(ast::Instruction::Bra { .. })
        // Normally call is not a terminator, but we treat it as such because it
        // makes the instruction modes to global modes pass possible
        | Statement::Instruction(ast::Instruction::Call { .. })
        | Statement::Instruction(ast::Instruction::Ret { .. }) => true,
        _ => false,
    }
}
