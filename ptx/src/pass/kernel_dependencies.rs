use super::{Directive2, Function, SpirvWord, Statement};
use petgraph::{graph::NodeIndex, visit::Dfs, Graph};
use ptx_parser as ast;
use rustc_hash::{FxHashMap, FxHashSet};
use std::convert::Infallible;

struct GlobalReferenceVisitor<'a> {
    globals: &'a FxHashSet<SpirvWord>,
    references: FxHashSet<SpirvWord>,
}

impl ast::VisitorMap<SpirvWord, SpirvWord, Infallible> for GlobalReferenceVisitor<'_> {
    fn visit(
        &mut self,
        operand: SpirvWord,
        _type_space: Option<(&ast::Type, ast::StateSpace)>,
        _is_dst: bool,
        _relaxed_type_check: bool,
    ) -> Result<SpirvWord, Infallible> {
        self.record(operand);
        Ok(operand)
    }

    fn visit_ident(
        &mut self,
        ident: SpirvWord,
        _type_space: Option<(&ast::Type, ast::StateSpace)>,
        _is_dst: bool,
        _relaxed_type_check: bool,
    ) -> Result<SpirvWord, Infallible> {
        self.record(ident);
        Ok(ident)
    }
}

impl GlobalReferenceVisitor<'_> {
    fn record(&mut self, ident: SpirvWord) {
        if self.globals.contains(&ident) {
            self.references.insert(ident);
        }
    }
}

pub(super) struct DependencyGraph {
    graph: Graph<SpirvWord, ()>,
    nodes: FxHashMap<SpirvWord, NodeIndex>,
}

impl DependencyGraph {
    pub(super) fn from_directives(
        directives: &mut [Directive2<ast::Instruction<SpirvWord>, SpirvWord>],
    ) -> Self {
        let mut result = Self {
            graph: Graph::new(),
            nodes: FxHashMap::default(),
        };

        // Register all module-level symbols before adding edges. A function
        // declaration and its definition share the same SpirvWord node.
        let globals = directives
            .iter()
            .filter_map(|directive| match directive {
                Directive2::Variable(_, variable) => Some(variable.name),
                Directive2::Method(_) => None,
            })
            .collect::<FxHashSet<_>>();

        for directive in directives.iter() {
            match directive {
                Directive2::Variable(_, variable) => {
                    result.add_node(variable.name);
                }
                Directive2::Method(function) => {
                    result.add_node(function.name);
                }
            }
        }

        for directive in directives.iter_mut() {
            match directive {
                Directive2::Variable(_, variable) => {
                    for initializer in &variable.info.array_init {
                        if let ast::RegOrImmediate::Reg(dependency) = initializer {
                            result.add_dependency(variable.name, *dependency);
                        }
                    }
                }
                Directive2::Method(function) => {
                    let Some(body) = function.body.as_mut() else {
                        continue;
                    };

                    let mut global_references = GlobalReferenceVisitor {
                        globals: &globals,
                        references: FxHashSet::default(),
                    };
                    let mut calls = FxHashSet::default();

                    let old_body = std::mem::take(body);
                    *body = old_body
                        .into_iter()
                        .map(|statement| {
                            if let Statement::Instruction(ast::Instruction::Call {
                                arguments,
                                ..
                            }) = &statement
                            {
                                calls.insert(arguments.func);
                            }

                            statement
                                .visit_map(&mut global_references)
                                .expect("infallible global reference visitor")
                        })
                        .collect();

                    for callee in calls {
                        result.add_dependency(function.name, callee);
                    }

                    for global in global_references.references {
                        result.add_dependency(function.name, global);
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
) -> FxHashMap<SpirvWord, Vec<&'a Function<ast::Instruction<SpirvWord>, SpirvWord>>> {
    let mut functions: FxHashMap<
        SpirvWord,
        Vec<&'a Function<ast::Instruction<SpirvWord>, SpirvWord>>,
    > = FxHashMap::default();

    for directive in directives {
        let Directive2::Method(function) = directive else {
            continue;
        };

        functions.entry(function.name).or_default().push(function);
    }

    functions
}

pub(super) fn kernel_dependencies(
    directives: &mut [Directive2<ast::Instruction<SpirvWord>, SpirvWord>],
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
    directives: &mut [Directive2<ast::Instruction<SpirvWord>, SpirvWord>],
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
    directives: &mut [Directive2<ast::Instruction<SpirvWord>, SpirvWord>],
) -> FxHashMap<SpirvWord, Vec<Function<ast::Instruction<SpirvWord>, SpirvWord>>> {
    let method_sets = kernel_method_sets(directives);
    let functions = function_index(directives);

    method_sets
        .into_iter()
        .map(|(kernel, methods)| {
            let declarations = methods
                .into_iter()
                .filter(|method| *method != kernel)
                .filter_map(|method| functions.get(&method))
                .filter_map(|functions| {
                    functions
                        .iter()
                        .copied()
                        .find(|function| function.body.is_some())
                        .or_else(|| functions.first().copied())
                })
                .map(method_declaration)
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
    let mut directives = directives;
    let reachable_symbols = kernel_dependencies(&mut directives);
    let mut declaration_sets = kernel_declaration_sets(&mut directives);
    let globals = directives
        .iter()
        .filter_map(|directive| match directive {
            Directive2::Variable(linking, variable) => {
                Some((variable.name, global_declaration(*linking, variable)))
            }
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
                let global_declarations = reachable_symbols
                    .get(&function.name)
                    .map(|reachable| {
                        globals
                            .iter()
                            .filter(|(name, _)| reachable.contains(name))
                            .map(|(_, declaration)| declaration.clone())
                            .collect()
                    })
                    .unwrap_or_default();

                kernels.push(KernelModulePlan {
                    kernel: function,
                    global_declarations,
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
