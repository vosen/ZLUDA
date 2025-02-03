use std::hash::Hash;

use super::BrachCondition;
use super::Directive2;
use super::Function2;
use super::SpirvWord;
use super::Statement;
use super::TranslateError;
use petgraph::graph::NodeIndex;
use petgraph::visit::IntoNodeReferences;
use petgraph::Direction;
use petgraph::Graph;
use ptx_parser as ast;
use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;

struct ControlFlowGraph<T: Eq + PartialEq> {
    entry_points: FxHashMap<SpirvWord, NodeIndex>,
    basic_blocks: FxHashMap<SpirvWord, NodeIndex>,
    graph: Graph<Node<ExtendedMode<T>>, ()>,
}

impl<T: Eq + PartialEq> ControlFlowGraph<T> {
    fn new() -> Self {
        Self {
            entry_points: FxHashMap::default(),
            basic_blocks: FxHashMap::default(),
            graph: Graph::new(),
        }
    }

    fn add_entry_basic_block(&mut self, label: SpirvWord) -> NodeIndex {
        let idx = self.graph.add_node(Node {
            label,
            entry: Some(ExtendedMode::Entry(label)),
            exit: Some(ExtendedMode::Entry(label)),
        });
        assert_eq!(self.entry_points.insert(label, idx), None);
        idx
    }

    fn get_or_add_basic_block(&mut self, label: SpirvWord) -> NodeIndex {
        self.basic_blocks.get(&label).copied().unwrap_or_else(|| {
            let idx = self.graph.add_node(Node {
                label,
                entry: None,
                exit: None,
            });
            self.basic_blocks.insert(label, idx);
            idx
        })
    }

    fn add_jump(&mut self, from: NodeIndex, to: SpirvWord) {
        let to = self.get_or_add_basic_block(to);
        self.graph.add_edge(from, to, ());
    }

    fn set_modes(&mut self, node: NodeIndex, entry: T, exit: T) {
        self.graph[node].entry = Some(ExtendedMode::BasicBlock(entry));
        self.graph[node].exit = Some(ExtendedMode::BasicBlock(exit));
    }
}

#[derive(Debug)]
struct Node<T> {
    label: SpirvWord,
    entry: Option<T>,
    exit: Option<T>,
}

pub(crate) fn run<'input>(
    flat_resolver: &mut super::GlobalStringIdentResolver2<'input>,
    directives: Vec<super::Directive2<'input, ast::Instruction<SpirvWord>, super::SpirvWord>>,
) -> Result<Vec<Directive2<'input, ast::Instruction<SpirvWord>, SpirvWord>>, TranslateError> {
    let mut cfg = ControlFlowGraph::<bool>::new();
    let mut node_idx_to_name = FxHashMap::<NodeIndex<u32>, SpirvWord>::default();
    for directive in directives.iter() {
        match directive {
            super::Directive2::Method(Function2 {
                func_decl: ast::MethodDeclaration { name, .. },
                body,
                ..
            }) => {
                for statement in body.iter() {
                    todo!()
                }
            }
            _ => continue,
        }
    }
    todo!()
}

fn compute<T: Copy + Eq>(g: ControlFlowGraph<T>) -> FxHashSet<SpirvWord> {
    let mut must_insert_mode = FxHashSet::<SpirvWord>::default();
    let mut remaining = g
        .graph
        .node_references()
        .rev()
        .filter_map(|(index, node)| node.entry.as_ref().map(|mode| (index, node.label, *mode)))
        .collect::<Vec<_>>();
    'next_basic_block: while let Some((index, node_id, expected_mode)) = remaining.pop() {
        let mut to_visit = UniqueVec::new(g.graph.neighbors_directed(index, Direction::Incoming));
        let mut visited = FxHashSet::default();
        while let Some(current) = to_visit.pop() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);
            let exit_mode = g.graph.node_weight(current).unwrap().exit;
            match exit_mode {
                None => {
                    for predecessor in g.graph.neighbors_directed(current, Direction::Incoming) {
                        if !visited.contains(&predecessor) {
                            to_visit.push(predecessor);
                        }
                    }
                }
                Some(mode) => {
                    if mode != expected_mode {
                        must_insert_mode.insert(node_id);
                        continue 'next_basic_block;
                    }
                }
            }
        }
    }
    must_insert_mode
}

#[derive(Eq, PartialEq, Clone, Copy)]
enum ExtendedMode<T: Eq + PartialEq> {
    BasicBlock(T),
    Entry(SpirvWord),
}

struct UniqueVec<T: Copy + Eq + Hash> {
    set: FxHashSet<T>,
    vec: Vec<T>,
}

impl<T: Copy + Eq + Hash> UniqueVec<T> {
    fn new(iter: impl Iterator<Item = T>) -> Self {
        let mut set = FxHashSet::default();
        let mut vec = Vec::new();
        for item in iter {
            if set.contains(&item) {
                continue;
            }
            set.insert(item);
            vec.push(item);
        }
        Self { set, vec }
    }

    fn pop(&mut self) -> Option<T> {
        if let Some(t) = self.vec.pop() {
            assert!(self.set.remove(&t));
            Some(t)
        } else {
            None
        }
    }

    fn push(&mut self, t: T) -> bool {
        if self.set.insert(t) {
            self.vec.push(t);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transitive_change() {
        let mut graph = ControlFlowGraph::<bool>::new();
        let entry_id = SpirvWord(1);
        let empty_id = SpirvWord(2);
        let false_id = SpirvWord(3);
        let entry = graph.add_entry_basic_block(entry_id);
        graph.add_jump(entry, empty_id);
        let empty = graph.get_or_add_basic_block(empty_id);
        graph.add_jump(empty, false_id);
        let false_ = graph.get_or_add_basic_block(false_id);
        graph.set_modes(false_, false, false);
        let result = super::compute(graph);
        assert_eq!(result.len(), 1);
        assert!(result.contains(&false_id));
    }

    #[test]
    fn codependency() {
        let mut graph = ControlFlowGraph::<bool>::new();
        let entry_id = SpirvWord(1);
        let left_f_id = SpirvWord(2);
        let right_f_id = SpirvWord(3);
        let left_none_id = SpirvWord(4);
        let mid_none_id = SpirvWord(5);
        let right_none_id = SpirvWord(6);
        let entry = graph.add_entry_basic_block(entry_id);
        graph.add_jump(entry, left_f_id);
        graph.add_jump(entry, right_f_id);
        let left_f = graph.get_or_add_basic_block(left_f_id);
        graph.set_modes(left_f, false, false);
        let right_f = graph.get_or_add_basic_block(right_f_id);
        graph.set_modes(right_f, false, false);
        graph.add_jump(left_f, left_none_id);
        let left_none = graph.get_or_add_basic_block(left_none_id);
        graph.add_jump(right_f, right_none_id);
        let right_none = graph.get_or_add_basic_block(right_none_id);
        graph.add_jump(left_none, mid_none_id);
        graph.add_jump(right_none, mid_none_id);
        let mid_none = graph.get_or_add_basic_block(mid_none_id);
        graph.add_jump(mid_none, left_none_id);
        graph.add_jump(mid_none, right_none_id);
        //println!(
        //    "{:?}",
        //    petgraph::dot::Dot::with_config(&graph.graph, &[petgraph::dot::Config::EdgeNoLabel])
        //);
        let result = super::compute(graph);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&left_f_id));
        assert!(result.contains(&right_f_id));
    }
}
