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

pub(super) fn method_declaration(
    function: &Function<ast::Instruction<SpirvWord>, SpirvWord>,
) -> Function<ast::Instruction<SpirvWord>, SpirvWord> {
    debug_assert!(!function.is_kernel());

    Function {
        return_arguments: function.return_arguments.clone(),
        name: function.name,
        input_arguments: function.input_arguments.clone(),
        body: None,
        kernel_attributes: None,
        import_as: function.import_as.clone(),
        tuning: function.tuning.clone(),
        linkage: function.linkage,
        kernel_meta32: None,
    }
}

pub(super) fn kernel_declaration_sets(
    directives: &[Directive2<ast::Instruction<SpirvWord>, SpirvWord>],
) -> FxHashMap<SpirvWord, Vec<Function<ast::Instruction<SpirvWord>, SpirvWord>>> {
    let functions = function_index(directives);

    kernel_method_sets(directives)
        .into_iter()
        .map(|(kernel, methods)| {
            let declarations = methods
                .into_iter()
                .filter(|method| *method != kernel)
                .filter_map(|method| functions.get(&method))
                .map(|function| method_declaration(function))
                .collect();

            (kernel, declarations)
        })
        .collect()
}

pub(super) fn global_declaration(
    linking: ast::LinkingDirective,
    variable: &ast::Variable<SpirvWord>,
) -> (ast::LinkingDirective, ast::Variable<SpirvWord>) {
    let mut declaration = variable.clone();
    declaration.info.array_init.clear();

    (linking | ast::LinkingDirective::EXTERN, declaration)
}

pub(super) struct KernelModulePlan {
    pub(super) kernel: Function<ast::Instruction<SpirvWord>, SpirvWord>,
    pub(super) global_declarations: Vec<(ast::LinkingDirective, ast::Variable<SpirvWord>)>,
    pub(super) declarations: Vec<Function<ast::Instruction<SpirvWord>, SpirvWord>>,
}

pub(super) struct KernelCompilationPlan {
    pub(super) common: Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>,
    pub(super) kernels: Vec<KernelModulePlan>,
}

pub(super) fn build_compilation_plan(
    directives: Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>,
) -> KernelCompilationPlan {
    let mut declaration_sets = kernel_declaration_sets(&directives);
    let global_declarations = directives
        .iter()
        .filter_map(|directive| match directive {
            Directive2::Variable(linking, variable) => Some(global_declaration(*linking, variable)),
            Directive2::Method(..) => None,
        })
        .collect::<Vec<_>>();
    let mut common = Vec::new();
    let mut kernels = Vec::new();

    for directive in directives {
        match directive {
            Directive2::Variable(..) => common.push(directive),
            Directive2::Method(function) if function.is_kernel() => {
                let declarations = declaration_sets.remove(&function.name).unwrap_or_default();

                kernels.push(KernelModulePlan {
                    kernel: function,
                    global_declarations: global_declarations.clone(),
                    declarations,
                });
            }
            Directive2::Method(..) => common.push(directive),
        }
    }

    KernelCompilationPlan { common, kernels }
}

impl KernelCompilationPlan {
    #[cfg(test)]
    pub(super) fn into_monolithic_directives(
        self,
    ) -> Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>> {
        let mut directives = self.common;

        directives.extend(self.kernels.into_iter().map(|kernel_plan| {
            let KernelModulePlan {
                kernel,
                global_declarations,
                declarations,
            } = kernel_plan;

            drop(global_declarations);
            drop(declarations);
            Directive2::Method(kernel)
        }));

        directives
    }
}
