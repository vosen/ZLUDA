use super::BrachCondition;
use super::Directive2;
use super::Function2;
use super::SpirvWord;
use super::Statement;
use super::TranslateError;
use microlp::OptimizationDirection;
use microlp::Problem;
use microlp::Variable;
use petgraph::graph::NodeIndex;
use petgraph::visit::IntoNodeReferences;
use petgraph::Direction;
use petgraph::Graph;
use ptx_parser as ast;
use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;
use std::hash::Hash;
use std::iter;

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

fn compute<T: Copy + Eq>(graph: ControlFlowGraph<T>) -> PartialModeInsertion<T> {
    let mut must_insert_mode = FxHashSet::<SpirvWord>::default();
    let mut maybe_insert_mode = FxHashMap::default();
    let mut remaining = graph
        .graph
        .node_references()
        .rev()
        .filter_map(|(index, node)| {
            node.entry
                .as_ref()
                .map(|mode| match mode {
                    ExtendedMode::BasicBlock(mode) => Some((index, node.label, *mode)),
                    ExtendedMode::Entry(_) => None,
                })
                .flatten()
        })
        .collect::<Vec<_>>();
    'next_basic_block: while let Some((index, node_id, expected_mode)) = remaining.pop() {
        let mut to_visit =
            UniqueVec::new(graph.graph.neighbors_directed(index, Direction::Incoming));
        let mut visited = FxHashSet::default();
        while let Some(current) = to_visit.pop() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);
            let exit_mode = graph.graph.node_weight(current).unwrap().exit;
            match exit_mode {
                None => {
                    for predecessor in graph.graph.neighbors_directed(current, Direction::Incoming)
                    {
                        if !visited.contains(&predecessor) {
                            to_visit.push(predecessor);
                        }
                    }
                }
                Some(ExtendedMode::BasicBlock(mode)) => {
                    if mode != expected_mode {
                        maybe_insert_mode.remove(&node_id);
                        must_insert_mode.insert(node_id);
                        continue 'next_basic_block;
                    }
                }
                Some(ExtendedMode::Entry(kernel)) => match maybe_insert_mode.entry(node_id) {
                    std::collections::hash_map::Entry::Vacant(entry) => {
                        entry.insert((expected_mode, iter::once(kernel).collect::<FxHashSet<_>>()));
                    }
                    std::collections::hash_map::Entry::Occupied(mut entry) => {
                        entry.get_mut().1.insert(kernel);
                    }
                },
            }
        }
    }
    PartialModeInsertion {
        bb_must_insert_mode: must_insert_mode,
        bb_maybe_insert_mode: maybe_insert_mode,
    }
}

struct PartialModeInsertion<T> {
    bb_must_insert_mode: FxHashSet<SpirvWord>,
    bb_maybe_insert_mode: FxHashMap<SpirvWord, (T, FxHashSet<SpirvWord>)>,
}

fn optimize<T: Copy + Into<usize> + TryFrom<usize> + std::fmt::Debug, const N: usize>(
    partial: PartialModeInsertion<T>,
) -> ModeInsertions<T> {
    let mut problem = Problem::new(OptimizationDirection::Maximize);
    let mut kernel_modes = FxHashMap::default();
    let basic_block_variables = partial
        .bb_maybe_insert_mode
        .into_iter()
        .map(|(basic_block, (value, entry_points))| {
            let modes = entry_points
                .iter()
                .map(|entry_point| {
                    let kernel_modes = kernel_modes
                        .entry(*entry_point)
                        .or_insert_with(|| one_of::<N>(&mut problem));
                    kernel_modes[value.into()]
                })
                .collect::<Vec<Variable>>();
            let bb = and(&mut problem, &*modes);
            (basic_block, bb)
        })
        .collect::<Vec<_>>();
    // TODO: add fallback on Error
    let solution = problem.solve().unwrap();
    let mut basic_blocks = partial.bb_must_insert_mode;
    for (basic_block, variable) in basic_block_variables {
        if solution[variable] < 0.5 {
            basic_blocks.insert(basic_block);
        }
    }
    let mut kernels = FxHashMap::default();
    for (kernel, modes) in kernel_modes {
        for (mode, var) in modes.into_iter().enumerate() {
            if solution[var] > 0.5 {
                kernels.insert(kernel, T::try_from(mode).unwrap_or_else(|_| todo!()));
            }
        }
    }
    ModeInsertions {
        basic_blocks,
        kernels,
    }
}

fn and(problem: &mut Problem, variables: &[Variable]) -> Variable {
    let result = problem.add_binary_var(1.0);
    for var in variables {
        problem.add_constraint(
            &[(result, 1.0), (*var, -1.0)],
            microlp::ComparisonOp::Le,
            0.0,
        );
    }
    problem.add_constraint(
        iter::once((result, 1.0)).chain(variables.iter().map(|var| (*var, -1.0))),
        microlp::ComparisonOp::Ge,
        -((variables.len() - 1) as f64),
    );
    result
}

fn one_of<const N: usize>(problem: &mut Problem) -> [Variable; N] {
    let result = std::array::from_fn(|_| problem.add_binary_var(0.0));
    problem.add_constraint(
        result.into_iter().map(|var| (var, 1.0)),
        microlp::ComparisonOp::Eq,
        1.0,
    );
    result
}

struct ModeInsertions<T> {
    basic_blocks: FxHashSet<SpirvWord>,
    kernels: FxHashMap<SpirvWord, T>,
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
    use int_enum::IntEnum;

    #[repr(usize)]
    #[derive(IntEnum, Eq, PartialEq, Copy, Clone, Debug)]
    enum Bool {
        False = 0,
        True = 1,
    }

    #[test]
    fn transitive_mixed() {
        let mut graph = ControlFlowGraph::<Bool>::new();
        let entry_id = SpirvWord(1);
        let false_id = SpirvWord(2);
        let empty_id = SpirvWord(3);
        let false2_id = SpirvWord(4);
        let entry = graph.add_entry_basic_block(entry_id);
        graph.add_jump(entry, false_id);
        let false_ = graph.get_or_add_basic_block(false_id);
        graph.set_modes(false_, Bool::False, Bool::False);
        graph.add_jump(false_, empty_id);
        let empty = graph.get_or_add_basic_block(empty_id);
        graph.add_jump(empty, false2_id);
        let false2_ = graph.get_or_add_basic_block(false2_id);
        graph.set_modes(false2_, Bool::False, Bool::False);
        let partial_result = super::compute(graph);
        assert_eq!(partial_result.bb_must_insert_mode.len(), 0);
        assert_eq!(partial_result.bb_maybe_insert_mode.len(), 1);
        assert_eq!(
            partial_result.bb_maybe_insert_mode[&false_id],
            (Bool::False, iter::once(entry_id).collect())
        );

        let result = optimize::<Bool, 2>(partial_result);
        assert_eq!(result.basic_blocks.len(), 0);
        assert_eq!(result.kernels.len(), 1);
        assert_eq!(result.kernels[&entry_id], Bool::False);
    }

    #[test]
    fn transitive_change_twice() {
        let mut graph = ControlFlowGraph::<Bool>::new();
        let entry_id = SpirvWord(1);
        let false_id = SpirvWord(2);
        let empty_id = SpirvWord(3);
        let true_id = SpirvWord(4);
        let entry = graph.add_entry_basic_block(entry_id);
        graph.add_jump(entry, false_id);
        let false_ = graph.get_or_add_basic_block(false_id);
        graph.set_modes(false_, Bool::False, Bool::False);
        graph.add_jump(false_, empty_id);
        let empty = graph.get_or_add_basic_block(empty_id);
        graph.add_jump(empty, true_id);
        let true_ = graph.get_or_add_basic_block(true_id);
        graph.set_modes(true_, Bool::True, Bool::True);
        let partial_result = super::compute(graph);
        assert_eq!(partial_result.bb_must_insert_mode.len(), 1);
        assert!(partial_result.bb_must_insert_mode.contains(&true_id));
        assert_eq!(partial_result.bb_maybe_insert_mode.len(), 1);
        assert_eq!(
            partial_result.bb_maybe_insert_mode[&false_id],
            (Bool::False, iter::once(entry_id).collect())
        );

        let result = optimize::<Bool, 2>(partial_result);
        assert_eq!(result.basic_blocks, iter::once(true_id).collect());
        assert_eq!(result.kernels.len(), 1);
        assert_eq!(result.kernels[&entry_id], Bool::False);
    }

    #[test]
    fn transitive_change() {
        let mut graph = ControlFlowGraph::<Bool>::new();
        let entry_id = SpirvWord(1);
        let empty_id = SpirvWord(2);
        let true_id = SpirvWord(3);
        let entry = graph.add_entry_basic_block(entry_id);
        graph.add_jump(entry, empty_id);
        let empty = graph.get_or_add_basic_block(empty_id);
        graph.add_jump(empty, true_id);
        let true_ = graph.get_or_add_basic_block(true_id);
        graph.set_modes(true_, Bool::True, Bool::True);
        let partial_result = super::compute(graph);
        assert_eq!(partial_result.bb_must_insert_mode.len(), 0);
        assert_eq!(partial_result.bb_maybe_insert_mode.len(), 1);
        assert_eq!(
            partial_result.bb_maybe_insert_mode[&true_id],
            (Bool::True, iter::once(entry_id).collect())
        );

        let result = optimize::<Bool, 2>(partial_result);
        assert_eq!(result.basic_blocks.len(), 0);
        assert_eq!(result.kernels.len(), 1);
        assert_eq!(result.kernels[&entry_id], Bool::True);
    }

    #[test]
    fn codependency() {
        let mut graph = ControlFlowGraph::<Bool>::new();
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
        graph.set_modes(left_f, Bool::False, Bool::False);
        let right_f = graph.get_or_add_basic_block(right_f_id);
        graph.set_modes(right_f, Bool::False, Bool::False);
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
        let partial_result = super::compute(graph);
        assert_eq!(partial_result.bb_must_insert_mode.len(), 0);
        assert_eq!(partial_result.bb_maybe_insert_mode.len(), 2);
        assert_eq!(
            partial_result.bb_maybe_insert_mode[&left_f_id],
            (Bool::False, iter::once(entry_id).collect())
        );
        assert_eq!(
            partial_result.bb_maybe_insert_mode[&right_f_id],
            (Bool::False, iter::once(entry_id).collect())
        );

        let result = optimize::<Bool, 2>(partial_result);
        assert_eq!(result.basic_blocks.len(), 0);
        assert_eq!(result.kernels.len(), 1);
        assert_eq!(result.kernels[&entry_id], Bool::False);
    }
}
