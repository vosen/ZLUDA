use super::{Directive2, Function, SpirvWord, Statement};
use ptx_parser as ast;
use rustc_hash::{FxHashMap, FxHashSet};

pub(super) fn direct_callees(
    function: &Function<ast::Instruction<SpirvWord>, SpirvWord>,
) -> FxHashSet<SpirvWord> {
    let Some(body) = &function.body else {
        return FxHashSet::default();
    };

    body.iter()
        .filter_map(|statement| match statement {
            Statement::Instruction(ast::Instruction::Call { arguments, .. }) => {
                Some(arguments.func)
            }
            _ => None,
        })
        .collect()
}

pub(super) fn reachable_callees(
    root: SpirvWord,
    functions: &FxHashMap<SpirvWord, &Function<ast::Instruction<SpirvWord>, SpirvWord>>,
) -> FxHashSet<SpirvWord> {
    let mut reachable = FxHashSet::default();
    let mut visited = FxHashSet::default();
    let mut pending = vec![root];

    visited.insert(root);

    while let Some(function_name) = pending.pop() {
        let Some(function) = functions.get(&function_name) else {
            continue;
        };

        for callee in direct_callees(function) {
            if visited.insert(callee) {
                reachable.insert(callee);
                pending.push(callee);
            }
        }
    }

    reachable
}

pub(super) fn function_index<'a>(
    directives: &'a [Directive2<ast::Instruction<SpirvWord>, SpirvWord>],
) -> FxHashMap<SpirvWord, &'a Function<ast::Instruction<SpirvWord>, SpirvWord>> {
    directives
        .iter()
        .filter_map(|directive| match directive {
            Directive2::Method(function) => Some((function.name, function)),
            Directive2::Variable(..) => None,
        })
        .collect()
}

pub(super) fn kernel_dependencies(
    directives: &[Directive2<ast::Instruction<SpirvWord>, SpirvWord>],
) -> FxHashMap<SpirvWord, FxHashSet<SpirvWord>> {
    let functions = function_index(directives);

    functions
        .values()
        .filter(|function| function.is_kernel())
        .map(|function| (function.name, reachable_callees(function.name, &functions)))
        .collect()
}

pub(super) fn kernel_method_sets(
    directives: &[Directive2<ast::Instruction<SpirvWord>, SpirvWord>],
) -> FxHashMap<SpirvWord, FxHashSet<SpirvWord>> {
    kernel_dependencies(directives)
        .into_iter()
        .map(|(kernel, mut dependencies)| {
            dependencies.insert(kernel);
            (kernel, dependencies)
        })
        .collect()
}
