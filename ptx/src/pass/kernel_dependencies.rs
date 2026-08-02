use super::{Directive2, Function, SpirvWord, Statement};
use petgraph::{graph::NodeIndex, visit::Dfs, Graph};
use ptx_parser as ast;
use rustc_hash::{FxHashMap, FxHashSet};

pub(super) struct DependencyGraph {
    graph: Graph<SpirvWord, ()>,
    nodes: FxHashMap<SpirvWord, NodeIndex>,
}

impl DependencyGraph {
    pub(super) fn from_directives(
        directives: &[Directive2<ast::Instruction<SpirvWord>, SpirvWord>],
    ) -> Self {
        let mut result = Self {
            graph: Graph::new(),
            nodes: FxHashMap::default(),
        };

        // Register all module-level symbols before adding edges. A function
        // declaration and its definition share the same SpirvWord node.
        for directive in directives {
            match directive {
                Directive2::Variable(_, variable) => {
                    result.add_node(variable.name);
                }
                Directive2::Method(function) => {
                    result.add_node(function.name);
                }
            }
        }

        for directive in directives {
            match directive {
                Directive2::Variable(_, variable) => {
                    for initializer in &variable.info.array_init {
                        if let ast::RegOrImmediate::Reg(dependency) = initializer {
                            result.add_dependency(variable.name, *dependency);
                        }
                    }
                }
                Directive2::Method(function) => {
                    let Some(body) = &function.body else {
                        continue;
                    };

                    for statement in body {
                        if let Statement::Instruction(ast::Instruction::Call {
                            arguments, ..
                        }) = statement
                        {
                            result.add_dependency(function.name, arguments.func);
                        }
                    }
                }
            }
        }

        result
    }

    fn add_node(&mut self, symbol: SpirvWord) -> NodeIndex {
        if let Some(index) = self.nodes.get(&symbol) {
            return *index;
        }

        let index = self.graph.add_node(symbol);
        self.nodes.insert(symbol, index);
        index
    }

    fn add_dependency(&mut self, from: SpirvWord, to: SpirvWord) {
        let from = self.add_node(from);
        let to = self.add_node(to);

        if self.graph.find_edge(from, to).is_none() {
            self.graph.add_edge(from, to, ());
        }
    }

    pub(super) fn reachable_from(&self, root: SpirvWord) -> FxHashSet<SpirvWord> {
        let Some(root_index) = self.nodes.get(&root).copied() else {
            return FxHashSet::default();
        };

        let mut reachable = FxHashSet::default();
        let mut traversal = Dfs::new(&self.graph, root_index);

        while let Some(index) = traversal.next(&self.graph) {
            let symbol = self.graph[index];

            if symbol != root {
                reachable.insert(symbol);
            }
        }

        reachable
    }
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
    let graph = DependencyGraph::from_directives(directives);

    directives
        .iter()
        .filter_map(|directive| match directive {
            Directive2::Method(function) if function.is_kernel() => {
                Some((function.name, graph.reachable_from(function.name)))
            }
            _ => None,
        })
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
