use super::BrachCondition;
use super::Directive2;
use super::Function2;
use super::SpirvWord;
use super::Statement;
use super::TranslateError;
use crate::pass::error_unreachable;
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
use std::mem;
use strum::EnumCount;
use strum_macros::{EnumCount, VariantArray};

#[derive(Default, PartialEq, Eq, Clone, Copy, Debug, VariantArray, EnumCount)]
enum DenormalMode {
    #[default]
    FlushToZero,
    Preserve,
}

impl DenormalMode {
    fn from_ftz(ftz: bool) -> Self {
        if ftz {
            DenormalMode::FlushToZero
        } else {
            DenormalMode::Preserve
        }
    }
}

impl Into<usize> for DenormalMode {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Default, PartialEq, Eq, Clone, Copy, Debug, VariantArray, EnumCount)]
enum RoundingMode {
    #[default]
    NearestEven,
    Zero,
    NegativeInf,
    PositiveInf,
}

impl RoundingMode {
    fn to_ast(self) -> ast::RoundingMode {
        match self {
            RoundingMode::NearestEven => ast::RoundingMode::NearestEven,
            RoundingMode::Zero => ast::RoundingMode::Zero,
            RoundingMode::NegativeInf => ast::RoundingMode::NegativeInf,
            RoundingMode::PositiveInf => ast::RoundingMode::PositiveInf,
        }
    }

    fn from_ast(rnd: ast::RoundingMode) -> Self {
        match rnd {
            ast::RoundingMode::NearestEven => RoundingMode::NearestEven,
            ast::RoundingMode::Zero => RoundingMode::Zero,
            ast::RoundingMode::NegativeInf => RoundingMode::NegativeInf,
            ast::RoundingMode::PositiveInf => RoundingMode::PositiveInf,
        }
    }
}

impl Into<usize> for RoundingMode {
    fn into(self) -> usize {
        self as usize
    }
}

struct InstructionModes {
    denormal_f32: Option<DenormalMode>,
    denormal_f16f64: Option<DenormalMode>,
    rounding_f32: Option<RoundingMode>,
    rounding_f16f64: Option<RoundingMode>,
}

impl InstructionModes {
    fn fold_into(self, entry: &mut Self, exit: &mut Self) {
        fn set_if_none<T: Copy>(source: &mut Option<T>, value: Option<T>) {
            match (*source, value) {
                (None, Some(x)) => *source = Some(x),
                _ => {}
            }
        }
        fn set_if_some<T: Copy>(source: &mut Option<T>, value: Option<T>) {
            match (source, value) {
                (Some(ref mut x), Some(y)) => *x = y,
                _ => {}
            }
        }
        set_if_none(&mut entry.denormal_f32, self.denormal_f32);
        set_if_none(&mut entry.denormal_f16f64, self.denormal_f16f64);
        set_if_none(&mut entry.rounding_f32, self.rounding_f32);
        set_if_none(&mut entry.rounding_f16f64, self.rounding_f16f64);
        set_if_some(&mut exit.denormal_f32, self.denormal_f32);
        set_if_some(&mut exit.denormal_f16f64, self.denormal_f16f64);
        set_if_some(&mut exit.rounding_f32, self.rounding_f32);
        set_if_some(&mut exit.rounding_f16f64, self.rounding_f16f64);
    }

    fn none() -> Self {
        Self {
            denormal_f32: None,
            denormal_f16f64: None,
            rounding_f32: None,
            rounding_f16f64: None,
        }
    }

    fn new(
        type_: ast::ScalarType,
        denormal: Option<DenormalMode>,
        rounding: Option<RoundingMode>,
    ) -> Self {
        if type_ != ast::ScalarType::F32 {
            Self {
                denormal_f16f64: denormal,
                rounding_f16f64: rounding,
                ..Self::none()
            }
        } else {
            Self {
                denormal_f32: denormal,
                rounding_f32: rounding,
                ..Self::none()
            }
        }
    }

    fn mixed_ftz_f32(
        type_: ast::ScalarType,
        denormal: Option<DenormalMode>,
        rounding: Option<RoundingMode>,
    ) -> Self {
        if type_ != ast::ScalarType::F32 {
            Self {
                denormal_f16f64: denormal,
                rounding_f32: rounding,
                ..Self::none()
            }
        } else {
            Self {
                denormal_f32: denormal,
                rounding_f32: rounding,
                ..Self::none()
            }
        }
    }

    fn from_arith_float(arith: &ast::ArithFloat) -> InstructionModes {
        let denormal = arith.flush_to_zero.map(DenormalMode::from_ftz);
        let rounding = Some(RoundingMode::from_ast(arith.rounding));
        InstructionModes::new(arith.type_, denormal, rounding)
    }

    fn from_ftz(type_: ast::ScalarType, ftz: Option<bool>) -> Self {
        Self::new(type_, ftz.map(DenormalMode::from_ftz), None)
    }

    fn from_ftz_f32(ftz: bool) -> Self {
        Self::new(
            ast::ScalarType::F32,
            Some(DenormalMode::from_ftz(ftz)),
            None,
        )
    }

    fn from_rcp(data: ast::RcpData) -> InstructionModes {
        let rounding = match data.kind {
            ast::RcpKind::Approx => None,
            ast::RcpKind::Compliant(rnd) => Some(RoundingMode::from_ast(rnd)),
        };
        let denormal = data.flush_to_zero.map(DenormalMode::from_ftz);
        InstructionModes::new(data.type_, denormal, rounding)
    }

    fn from_cvt(cvt: &ast::CvtDetails) -> InstructionModes {
        match cvt.mode {
            ast::CvtMode::ZeroExtend
            | ast::CvtMode::SignExtend
            | ast::CvtMode::Truncate
            | ast::CvtMode::Bitcast
            | ast::CvtMode::SaturateUnsignedToSigned
            | ast::CvtMode::SaturateSignedToUnsigned => Self::none(),
            ast::CvtMode::FPExtend { flush_to_zero } => {
                Self::from_ftz(ast::ScalarType::F32, flush_to_zero)
            }
            ast::CvtMode::FPTruncate {
                rounding,
                flush_to_zero,
            }
            | ast::CvtMode::FPRound {
                integer_rounding: rounding,
                flush_to_zero,
            } => Self::mixed_ftz_f32(
                cvt.to,
                flush_to_zero.map(DenormalMode::from_ftz),
                Some(RoundingMode::from_ast(rounding)),
            ),
            ast::CvtMode::SignedFromFP {
                flush_to_zero,
                rounding,
            }
            | ast::CvtMode::UnsignedFromFP {
                flush_to_zero,
                rounding,
            } => Self::new(
                cvt.from,
                flush_to_zero.map(DenormalMode::from_ftz),
                Some(RoundingMode::from_ast(rounding)),
            ),
            ast::CvtMode::FPFromSigned(rnd) | ast::CvtMode::FPFromUnsigned(rnd) => {
                Self::new(cvt.to, None, Some(RoundingMode::from_ast(rnd)))
            }
        }
    }
}

struct ControlFlowGraph {
    entry_points: FxHashMap<SpirvWord, NodeIndex>,
    basic_blocks: FxHashMap<SpirvWord, NodeIndex>,
    graph: Graph<Node, ()>,
}

impl ControlFlowGraph {
    fn new() -> Self {
        Self {
            entry_points: FxHashMap::default(),
            basic_blocks: FxHashMap::default(),
            graph: Graph::new(),
        }
    }

    fn add_entry_basic_block(&mut self, label: SpirvWord) -> NodeIndex {
        let idx = self.graph.add_node(Node::entry(label));
        assert_eq!(self.entry_points.insert(label, idx), None);
        idx
    }

    fn get_or_add_basic_block(&mut self, label: SpirvWord) -> NodeIndex {
        self.basic_blocks.get(&label).copied().unwrap_or_else(|| {
            let idx = self.graph.add_node(Node::new(label));
            self.basic_blocks.insert(label, idx);
            idx
        })
    }

    fn add_jump(&mut self, from: NodeIndex, to: SpirvWord) {
        let to = self.get_or_add_basic_block(to);
        self.graph.add_edge(from, to, ());
    }

    fn set_modes(&mut self, node: NodeIndex, entry: InstructionModes, exit: InstructionModes) {
        self.graph[node].denormal_f32 = Mode {
            entry: entry.denormal_f32.map(ExtendedMode::BasicBlock),
            exit: exit.denormal_f32.map(ExtendedMode::BasicBlock),
        };
        self.graph[node].denormal_f16f64 = Mode {
            entry: entry.denormal_f16f64.map(ExtendedMode::BasicBlock),
            exit: exit.denormal_f16f64.map(ExtendedMode::BasicBlock),
        };
        self.graph[node].rounding_f32 = Mode {
            entry: entry.rounding_f32.map(ExtendedMode::BasicBlock),
            exit: exit.rounding_f32.map(ExtendedMode::BasicBlock),
        };
        self.graph[node].rounding_f16f64 = Mode {
            entry: entry.rounding_f16f64.map(ExtendedMode::BasicBlock),
            exit: exit.rounding_f16f64.map(ExtendedMode::BasicBlock),
        };
    }
}

#[derive(Clone, Copy)]
struct Mode<T: Eq + PartialEq> {
    entry: Option<ExtendedMode<T>>,
    exit: Option<ExtendedMode<T>>,
}

impl<T: Eq + PartialEq> Mode<T> {
    fn new() -> Self {
        Self {
            entry: None,
            exit: None,
        }
    }

    fn entry(label: SpirvWord) -> Self {
        Self {
            entry: Some(ExtendedMode::Entry(label)),
            exit: Some(ExtendedMode::Entry(label)),
        }
    }
}

struct Node {
    label: SpirvWord,
    denormal_f32: Mode<DenormalMode>,
    denormal_f16f64: Mode<DenormalMode>,
    rounding_f32: Mode<RoundingMode>,
    rounding_f16f64: Mode<RoundingMode>,
}

impl Node {
    fn entry(label: SpirvWord) -> Self {
        Self {
            label,
            denormal_f32: Mode::entry(label),
            denormal_f16f64: Mode::entry(label),
            rounding_f32: Mode::entry(label),
            rounding_f16f64: Mode::entry(label),
        }
    }

    fn new(label: SpirvWord) -> Self {
        Self {
            label,
            denormal_f32: Mode::new(),
            denormal_f16f64: Mode::new(),
            rounding_f32: Mode::new(),
            rounding_f16f64: Mode::new(),
        }
    }
}

trait EnumTuple {
    const LENGTH: usize;

    fn get(&self, x: usize) -> u8;
    fn get_mut(&mut self, x: usize) -> &mut u8;
}

pub(crate) fn run<'input>(
    flat_resolver: &mut super::GlobalStringIdentResolver2<'input>,
    directives: Vec<super::Directive2<ast::Instruction<SpirvWord>, super::SpirvWord>>,
) -> Result<Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>, TranslateError> {
    let mut cfg = ControlFlowGraph::new();
    for directive in directives.iter() {
        match directive {
            super::Directive2::Method(Function2 {
                name,
                body: Some(body),
                ..
            }) => {
                let mut basic_block = Some(cfg.add_entry_basic_block(*name));
                let mut entry = InstructionModes::none();
                let mut exit = InstructionModes::none();
                for statement in body.iter() {
                    match statement {
                        Statement::Instruction(ast::Instruction::Bra { arguments }) => {
                            let bb_index = basic_block.ok_or_else(error_unreachable)?;
                            cfg.add_jump(bb_index, arguments.src);
                            cfg.set_modes(
                                bb_index,
                                mem::replace(&mut entry, InstructionModes::none()),
                                mem::replace(&mut exit, InstructionModes::none()),
                            );
                            basic_block = None;
                        }
                        Statement::Label(label) => {
                            basic_block = Some(cfg.get_or_add_basic_block(*label));
                        }
                        Statement::Conditional(BrachCondition {
                            if_true, if_false, ..
                        }) => {
                            let bb_index = basic_block.ok_or_else(error_unreachable)?;
                            cfg.add_jump(bb_index, *if_true);
                            cfg.add_jump(bb_index, *if_false);
                            cfg.set_modes(
                                bb_index,
                                mem::replace(&mut entry, InstructionModes::none()),
                                mem::replace(&mut exit, InstructionModes::none()),
                            );
                            basic_block = None;
                        }
                        Statement::Instruction(instruction) => {
                            let modes = get_modes(instruction);
                            modes.fold_into(&mut entry, &mut exit);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    todo!()
}

fn compute_single_mode<T: Copy + Eq>(
    graph: &ControlFlowGraph,
    mut getter: impl FnMut(&Node) -> Mode<T>,
) -> PartialModeInsertion<T> {
    let mut must_insert_mode = FxHashSet::<SpirvWord>::default();
    let mut maybe_insert_mode = FxHashMap::default();
    let mut remaining = graph
        .graph
        .node_references()
        .rev()
        .filter_map(|(index, node)| {
            getter(node)
                .entry
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
            let exit_mode = getter(graph.graph.node_weight(current).unwrap()).exit;
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

fn optimize<T: Copy + Into<usize> + strum::VariantArray + std::fmt::Debug, const N: usize>(
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
                kernels.insert(kernel, T::VARIANTS[mode]);
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

fn get_modes<T: ast::Operand>(inst: &ast::Instruction<T>) -> InstructionModes {
    match inst {
        // TODO: review it when implementing virtual calls
        ast::Instruction::Call { .. }
        | ast::Instruction::Mov { .. }
        | ast::Instruction::Ld { .. }
        | ast::Instruction::St { .. }
        | ast::Instruction::PrmtSlow { .. }
        | ast::Instruction::Prmt { .. }
        | ast::Instruction::Activemask { .. }
        | ast::Instruction::Membar { .. }
        | ast::Instruction::Trap {}
        | ast::Instruction::Not { .. }
        | ast::Instruction::Or { .. }
        | ast::Instruction::And { .. }
        | ast::Instruction::Bra { .. }
        | ast::Instruction::Clz { .. }
        | ast::Instruction::Brev { .. }
        | ast::Instruction::Popc { .. }
        | ast::Instruction::Xor { .. }
        | ast::Instruction::Rem { .. }
        | ast::Instruction::Bfe { .. }
        | ast::Instruction::Bfi { .. }
        | ast::Instruction::Shr { .. }
        | ast::Instruction::Shl { .. }
        | ast::Instruction::Selp { .. }
        | ast::Instruction::Ret { .. }
        | ast::Instruction::Bar { .. }
        | ast::Instruction::Cvta { .. }
        | ast::Instruction::Atom { .. }
        | ast::Instruction::AtomCas { .. } => InstructionModes::none(),
        ast::Instruction::Add {
            data: ast::ArithDetails::Integer(_),
            ..
        }
        | ast::Instruction::Sub {
            data: ast::ArithDetails::Integer(..),
            ..
        }
        | ast::Instruction::Mul {
            data: ast::MulDetails::Integer { .. },
            ..
        }
        | ast::Instruction::Mad {
            data: ast::MadDetails::Integer { .. },
            ..
        }
        | ast::Instruction::Min {
            data: ast::MinMaxDetails::Signed(..) | ast::MinMaxDetails::Unsigned(..),
            ..
        }
        | ast::Instruction::Max {
            data: ast::MinMaxDetails::Signed(..) | ast::MinMaxDetails::Unsigned(..),
            ..
        }
        | ast::Instruction::Div {
            data: ast::DivDetails::Signed(..) | ast::DivDetails::Unsigned(..),
            ..
        } => InstructionModes::none(),
        ast::Instruction::Fma { data, .. }
        | ast::Instruction::Sub {
            data: ast::ArithDetails::Float(data),
            ..
        }
        | ast::Instruction::Mul {
            data: ast::MulDetails::Float(data),
            ..
        }
        | ast::Instruction::Mad {
            data: ast::MadDetails::Float(data),
            ..
        }
        | ast::Instruction::Add {
            data: ast::ArithDetails::Float(data),
            ..
        } => InstructionModes::from_arith_float(data),
        ast::Instruction::Setp {
            data:
                ast::SetpData {
                    type_,
                    flush_to_zero,
                    ..
                },
            ..
        }
        | ast::Instruction::SetpBool {
            data:
                ast::SetpBoolData {
                    base:
                        ast::SetpData {
                            type_,
                            flush_to_zero,
                            ..
                        },
                    ..
                },
            ..
        }
        | ast::Instruction::Neg {
            data: ast::TypeFtz {
                type_,
                flush_to_zero,
            },
            ..
        }
        | ast::Instruction::Ex2 {
            data: ast::TypeFtz {
                type_,
                flush_to_zero,
            },
            ..
        }
        | ast::Instruction::Rsqrt {
            data: ast::TypeFtz {
                type_,
                flush_to_zero,
            },
            ..
        }
        | ast::Instruction::Abs {
            data: ast::TypeFtz {
                type_,
                flush_to_zero,
            },
            ..
        }
        | ast::Instruction::Min {
            data:
                ast::MinMaxDetails::Float(ast::MinMaxFloat {
                    type_,
                    flush_to_zero,
                    ..
                }),
            ..
        }
        | ast::Instruction::Max {
            data:
                ast::MinMaxDetails::Float(ast::MinMaxFloat {
                    type_,
                    flush_to_zero,
                    ..
                }),
            ..
        }
        | ast::Instruction::Div {
            data:
                ast::DivDetails::Float(ast::DivFloatDetails {
                    type_,
                    flush_to_zero,
                    ..
                }),
            ..
        } => InstructionModes::from_ftz(*type_, *flush_to_zero),
        ast::Instruction::Sin { data, .. }
        | ast::Instruction::Cos { data, .. }
        | ast::Instruction::Lg2 { data, .. } => InstructionModes::from_ftz_f32(data.flush_to_zero),
        ast::Instruction::Rcp { data, .. } | ast::Instruction::Sqrt { data, .. } => {
            InstructionModes::from_rcp(*data)
        }
        ast::Instruction::Cvt { data, .. } => InstructionModes::from_cvt(data),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use int_enum::IntEnum;
    use strum::EnumCount;

    #[repr(usize)]
    #[derive(IntEnum, Eq, PartialEq, Copy, Clone, Debug)]
    enum Bool {
        False = 0,
        True = 1,
    }

    fn ftz() -> InstructionModes {
        InstructionModes {
            denormal_f32: Some(DenormalMode::FlushToZero),
            denormal_f16f64: None,
            rounding_f32: None,
            rounding_f16f64: None,
        }
    }

    fn preserve() -> InstructionModes {
        InstructionModes {
            denormal_f32: Some(DenormalMode::Preserve),
            denormal_f16f64: None,
            rounding_f32: None,
            rounding_f16f64: None,
        }
    }

    #[test]
    fn transitive_mixed() {
        let mut graph = ControlFlowGraph::new();
        let entry_id = SpirvWord(1);
        let false_id = SpirvWord(2);
        let empty_id = SpirvWord(3);
        let false2_id = SpirvWord(4);
        let entry = graph.add_entry_basic_block(entry_id);
        graph.add_jump(entry, false_id);
        let false_ = graph.get_or_add_basic_block(false_id);
        graph.set_modes(false_, ftz(), ftz());
        graph.add_jump(false_, empty_id);
        let empty = graph.get_or_add_basic_block(empty_id);
        graph.add_jump(empty, false2_id);
        let false2_ = graph.get_or_add_basic_block(false2_id);
        graph.set_modes(false2_, ftz(), ftz());
        let partial_result = super::compute_single_mode(&graph, |node| node.denormal_f32);
        assert_eq!(partial_result.bb_must_insert_mode.len(), 0);
        assert_eq!(partial_result.bb_maybe_insert_mode.len(), 1);
        assert_eq!(
            partial_result.bb_maybe_insert_mode[&false_id],
            (DenormalMode::FlushToZero, iter::once(entry_id).collect())
        );

        let result = optimize::<DenormalMode, { DenormalMode::COUNT }>(partial_result);
        assert_eq!(result.basic_blocks.len(), 0);
        assert_eq!(result.kernels.len(), 1);
        assert_eq!(result.kernels[&entry_id], DenormalMode::FlushToZero);
    }

    #[test]
    fn transitive_change_twice() {
        let mut graph = ControlFlowGraph::new();
        let entry_id = SpirvWord(1);
        let false_id = SpirvWord(2);
        let empty_id = SpirvWord(3);
        let true_id = SpirvWord(4);
        let entry = graph.add_entry_basic_block(entry_id);
        graph.add_jump(entry, false_id);
        let false_ = graph.get_or_add_basic_block(false_id);
        graph.set_modes(false_, ftz(), ftz());
        graph.add_jump(false_, empty_id);
        let empty = graph.get_or_add_basic_block(empty_id);
        graph.add_jump(empty, true_id);
        let true_ = graph.get_or_add_basic_block(true_id);
        graph.set_modes(true_, preserve(), preserve());
        let partial_result = super::compute_single_mode(&graph, |node| node.denormal_f32);
        assert_eq!(partial_result.bb_must_insert_mode.len(), 1);
        assert!(partial_result.bb_must_insert_mode.contains(&true_id));
        assert_eq!(partial_result.bb_maybe_insert_mode.len(), 1);
        assert_eq!(
            partial_result.bb_maybe_insert_mode[&false_id],
            (DenormalMode::FlushToZero, iter::once(entry_id).collect())
        );

        let result = optimize::<DenormalMode, { DenormalMode::COUNT }>(partial_result);
        assert_eq!(result.basic_blocks, iter::once(true_id).collect());
        assert_eq!(result.kernels.len(), 1);
        assert_eq!(result.kernels[&entry_id], DenormalMode::FlushToZero);
    }

    #[test]
    fn transitive_change() {
        let mut graph = ControlFlowGraph::new();
        let entry_id = SpirvWord(1);
        let empty_id = SpirvWord(2);
        let true_id = SpirvWord(3);
        let entry = graph.add_entry_basic_block(entry_id);
        graph.add_jump(entry, empty_id);
        let empty = graph.get_or_add_basic_block(empty_id);
        graph.add_jump(empty, true_id);
        let true_ = graph.get_or_add_basic_block(true_id);
        graph.set_modes(true_, preserve(), preserve());
        let partial_result = super::compute_single_mode(&graph, |node| node.denormal_f32);
        assert_eq!(partial_result.bb_must_insert_mode.len(), 0);
        assert_eq!(partial_result.bb_maybe_insert_mode.len(), 1);
        assert_eq!(
            partial_result.bb_maybe_insert_mode[&true_id],
            (DenormalMode::Preserve, iter::once(entry_id).collect())
        );

        let result = optimize::<DenormalMode, { DenormalMode::COUNT }>(partial_result);
        assert_eq!(result.basic_blocks.len(), 0);
        assert_eq!(result.kernels.len(), 1);
        assert_eq!(result.kernels[&entry_id], DenormalMode::Preserve);
    }

    #[test]
    fn codependency() {
        let mut graph = ControlFlowGraph::new();
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
        graph.set_modes(left_f, ftz(), ftz());
        let right_f = graph.get_or_add_basic_block(right_f_id);
        graph.set_modes(right_f, ftz(), ftz());
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
        let partial_result = super::compute_single_mode(&graph, |node| node.denormal_f32);
        assert_eq!(partial_result.bb_must_insert_mode.len(), 0);
        assert_eq!(partial_result.bb_maybe_insert_mode.len(), 2);
        assert_eq!(
            partial_result.bb_maybe_insert_mode[&left_f_id],
            (DenormalMode::FlushToZero, iter::once(entry_id).collect())
        );
        assert_eq!(
            partial_result.bb_maybe_insert_mode[&right_f_id],
            (DenormalMode::FlushToZero, iter::once(entry_id).collect())
        );

        let result = optimize::<DenormalMode, { DenormalMode::COUNT }>(partial_result);
        assert_eq!(result.basic_blocks.len(), 0);
        assert_eq!(result.kernels.len(), 1);
        assert_eq!(result.kernels[&entry_id], DenormalMode::FlushToZero);
    }
}
