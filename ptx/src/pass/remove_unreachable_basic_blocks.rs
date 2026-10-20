use super::*;
use petgraph::{
    graph::NodeIndex,
    visit::{Bfs, VisitMap},
    Graph,
};
use rustc_hash::FxHashSet;

/// Removes unreachable basic blocks, and the functions nothing calls or takes the address of. With
/// `keep_visible`, `.visible` functions stay even when uncalled: another module may call them once linked
/// (`cuLink*` on AGX).
pub(crate) fn run(
    mut directives: Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>,
    keep_visible: bool,
) -> Result<Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>, TranslateError> {
    let mut reachable_funcs = FxHashSet::default();
    for directive in directives.iter_mut() {
        match directive {
            // Functions whose addresses initialize variables
            Directive2::Variable(_, var) => {
                for init in var.info.array_init.iter() {
                    if let ast::RegOrImmediate::Reg(name) = init {
                        reachable_funcs.insert(*name);
                    }
                }
            }
            Directive2::Method(Function {
                body: Some(body), ..
            }) => {
                let old_body = std::mem::replace(body, Vec::new());
                let mut cfg = ControlFlowGraph::new();
                let mut old_body_iter = old_body.iter();
                let mut current_bb = match old_body_iter.next() {
                    Some(Statement::Label(label)) => cfg.add_or_get_node(*label),
                    _ => return Err(error_unreachable()),
                };
                let first_bb = current_bb;
                for statement in old_body_iter {
                    match statement {
                        Statement::Label(label) => {
                            current_bb = cfg.add_or_get_node(*label);
                        }
                        Statement::Conditional(branch) => {
                            cfg.add_branch(current_bb, branch.if_true);
                            cfg.add_branch(current_bb, branch.if_false);
                        }
                        Statement::Instruction(ast::Instruction::Bra {
                            arguments: ast::BraArgs { src },
                        }) => {
                            cfg.add_branch(current_bb, *src);
                        }
                        Statement::FunctionPointer(FunctionPointerDetails { src, .. }) => {
                            reachable_funcs.insert(*src);
                        }
                        Statement::Instruction(ast::Instruction::Call {
                            arguments: ast::CallArgs { func, .. },
                            ..
                        }) => {
                            reachable_funcs.insert(*func);
                        }
                        _ => {}
                    }
                }
                let mut bfs = Bfs::new(&cfg.graph, first_bb);
                while let Some(_) = bfs.next(&cfg.graph) {}
                let mut visited = true;
                *body = try_filter_to_vec(old_body.into_iter(), |statement| {
                    match statement {
                        Statement::Label(label) => {
                            visited = bfs
                                .discovered
                                .is_visited(cfg.nodes.get(label).ok_or_else(error_unreachable)?);
                        }
                        _ => {}
                    }
                    Ok(visited)
                })?;
            }
            _ => {}
        }
    }
    Ok(directives
        .into_iter()
        .filter(|directive| match directive {
            Directive2::Variable(..) => true,
            Directive2::Method(Function {
                name,
                kernel_attributes,
                linkage,
                body,
                ..
            }) => {
                kernel_attributes.is_some()
                    || reachable_funcs.contains(name)
                    || (keep_visible && body.is_some() && linkage.contains(ast::LinkingDirective::VISIBLE))
            }
        })
        .collect::<Vec<_>>())
}

fn try_filter_to_vec<T, E>(
    mut iter: impl ExactSizeIterator<Item = T>,
    mut filter: impl FnMut(&T) -> Result<bool, E>,
) -> Result<Vec<T>, E> {
    iter.try_fold(Vec::with_capacity(iter.len()), |mut vec, item| {
        match filter(&item) {
            Ok(true) => vec.push(item),
            Ok(false) => {}
            Err(err) => return Err(err),
        }
        Ok(vec)
    })
}

struct ControlFlowGraph {
    graph: Graph<SpirvWord, ()>,
    nodes: FxHashMap<SpirvWord, NodeIndex>,
}

impl ControlFlowGraph {
    fn new() -> Self {
        Self {
            graph: Graph::new(),
            nodes: FxHashMap::default(),
        }
    }

    fn add_or_get_node(&mut self, id: SpirvWord) -> NodeIndex {
        *self
            .nodes
            .entry(id)
            .or_insert_with(|| self.graph.add_node(id))
    }

    fn add_branch(&mut self, from: NodeIndex, to: SpirvWord) -> NodeIndex {
        let to = self.add_or_get_node(to);
        self.graph.add_edge(from, to, ());
        to
    }
}
