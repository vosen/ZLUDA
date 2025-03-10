use super::BrachCondition;
use super::Directive2;
use super::Function2;
use super::ModeRegister;
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
use smallvec::SmallVec;
use std::hash::Hash;
use std::iter;
use std::mem;
use strum::EnumCount;
use strum_macros::{EnumCount, VariantArray};
use unwrap_or::unwrap_some_or;

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

    fn to_ftz(self) -> bool {
        match self {
            DenormalMode::FlushToZero => true,
            DenormalMode::Preserve => false,
        }
    }
}

impl Into<bool> for DenormalMode {
    fn into(self) -> bool {
        self.to_ftz()
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

impl Into<ast::RoundingMode> for RoundingMode {
    fn into(self) -> ast::RoundingMode {
        self.to_ast()
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
        fn set_if_any<T: Copy>(source: &mut Option<T>, value: Option<T>) {
            if let Some(x) = value {
                *source = Some(x);
            }
        }
        set_if_none(&mut entry.denormal_f32, self.denormal_f32);
        set_if_none(&mut entry.denormal_f16f64, self.denormal_f16f64);
        set_if_none(&mut entry.rounding_f32, self.rounding_f32);
        set_if_none(&mut entry.rounding_f16f64, self.rounding_f16f64);
        set_if_any(&mut exit.denormal_f32, self.denormal_f32);
        set_if_any(&mut exit.denormal_f16f64, self.denormal_f16f64);
        set_if_any(&mut exit.rounding_f32, self.rounding_f32);
        set_if_any(&mut exit.rounding_f16f64, self.rounding_f16f64);
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
            // float to int contains rounding field, but it's not a rounding
            // mode but rather round-to-int operation that will be applied
            ast::CvtMode::SignedFromFP { flush_to_zero, .. }
            | ast::CvtMode::UnsignedFromFP { flush_to_zero, .. } => {
                Self::new(cvt.from, flush_to_zero.map(DenormalMode::from_ftz), None)
            }
            ast::CvtMode::FPFromSigned(rnd) | ast::CvtMode::FPFromUnsigned(rnd) => {
                Self::new(cvt.to, None, Some(RoundingMode::from_ast(rnd)))
            }
        }
    }
}

struct ControlFlowGraph {
    entry_points: FxHashMap<SpirvWord, NodeIndex>,
    basic_blocks: FxHashMap<SpirvWord, NodeIndex>,
    // map function -> return label
    call_returns: FxHashMap<SpirvWord, Vec<NodeIndex>>,
    // map function -> return basic blocks
    functions_rets: FxHashMap<SpirvWord, NodeIndex>,
    graph: Graph<Node, ()>,
}

impl ControlFlowGraph {
    fn new() -> Self {
        Self {
            entry_points: FxHashMap::default(),
            basic_blocks: FxHashMap::default(),
            call_returns: FxHashMap::default(),
            functions_rets: FxHashMap::default(),
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

    fn add_jump(&mut self, from: NodeIndex, to: SpirvWord) -> NodeIndex {
        let to = self.get_or_add_basic_block(to);
        self.graph.add_edge(from, to, ());
        to
    }

    fn set_modes(&mut self, node: NodeIndex, entry: InstructionModes, exit: InstructionModes) {
        let node = &mut self.graph[node];
        node.denormal_f32.entry = entry.denormal_f32.map(ExtendedMode::BasicBlock);
        node.denormal_f16f64.entry = entry.denormal_f16f64.map(ExtendedMode::BasicBlock);
        node.rounding_f32.entry = entry.rounding_f32.map(ExtendedMode::BasicBlock);
        node.rounding_f16f64.entry = entry.rounding_f16f64.map(ExtendedMode::BasicBlock);
        node.denormal_f32.exit = exit.denormal_f32.map(ExtendedMode::BasicBlock);
        node.denormal_f16f64.exit = exit.denormal_f16f64.map(ExtendedMode::BasicBlock);
        node.rounding_f32.exit = exit.rounding_f32.map(ExtendedMode::BasicBlock);
        node.rounding_f16f64.exit = exit.rounding_f16f64.map(ExtendedMode::BasicBlock);
    }

    fn fixup_function_calls(&mut self) {
        for (function, source) in self.functions_rets.iter() {
            for target in self
                .call_returns
                .get(function)
                .iter()
                .map(|vec| vec.iter())
                .flatten()
                .copied()
            {
                self.graph.add_edge(*source, target, ());
            }
        }
    }
}

#[derive(Clone, Copy)]
//#[cfg_attr(test, derive(Debug))]
#[derive(Debug)]
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

//#[cfg_attr(test, derive(Debug))]
#[derive(Debug)]
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
                is_kernel,
                ..
            }) => {
                let (mut bb_state, mut body_iter) =
                    BasicBlockState::new(&mut cfg, *name, body, *is_kernel)?;
                while let Some(statement) = body_iter.next() {
                    match statement {
                        Statement::Instruction(ast::Instruction::Bra { arguments }) => {
                            bb_state.end(&[arguments.src]);
                        }
                        Statement::Instruction(ast::Instruction::Call {
                            arguments: ast::CallArgs { func, .. },
                            ..
                        }) => {
                            let after_call_label = match body_iter.next() {
                                Some(Statement::Instruction(ast::Instruction::Bra {
                                    arguments: ast::BraArgs { src },
                                })) => *src,
                                _ => return Err(error_unreachable()),
                            };
                            bb_state.record_call(*func, after_call_label)?;
                            //body_iter.next();
                        }
                        Statement::RetValue(..)
                        | Statement::Instruction(ast::Instruction::Ret { .. }) => {
                            bb_state.record_ret(*name)?;
                        }
                        Statement::Label(label) => {
                            bb_state.start(*label);
                        }
                        Statement::Conditional(BrachCondition {
                            if_true, if_false, ..
                        }) => {
                            bb_state.end(&[*if_true, *if_false]);
                        }
                        Statement::Instruction(instruction) => {
                            let modes = get_modes(instruction);
                            bb_state.append(modes);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    println!(
        "{:?}",
        petgraph::dot::Dot::with_config(&cfg.graph, &[petgraph::dot::Config::EdgeNoLabel])
    );
    cfg.fixup_function_calls();
    println!(
        "{:?}",
        petgraph::dot::Dot::with_config(&cfg.graph, &[petgraph::dot::Config::EdgeNoLabel])
    );
    let denormal_f32 = compute_single_mode(&cfg, |node| node.denormal_f32);
    let denormal_f16f64 = compute_single_mode(&cfg, |node| node.denormal_f16f64);
    let rounding_f32 = compute_single_mode(&cfg, |node| node.rounding_f32);
    let rounding_f16f64 = compute_single_mode(&cfg, |node| node.rounding_f16f64);
    let denormal_f32 = optimize::<DenormalMode, { DenormalMode::COUNT }>(denormal_f32);
    let denormal_f16f64 = optimize::<DenormalMode, { DenormalMode::COUNT }>(denormal_f16f64);
    let rounding_f32 = optimize::<RoundingMode, { RoundingMode::COUNT }>(rounding_f32);
    let rounding_f16f64: MandatoryModeInsertions<RoundingMode> =
        optimize::<RoundingMode, { RoundingMode::COUNT }>(rounding_f16f64);
    let denormal = join_modes(
        flat_resolver,
        &cfg,
        denormal_f32,
        |node| node.denormal_f32.entry,
        |node| node.denormal_f32.exit,
        denormal_f16f64,
        |node| node.denormal_f16f64.entry,
        |node| node.denormal_f16f64.exit,
    )?;
    let rounding = join_modes(
        flat_resolver,
        &cfg,
        rounding_f32,
        |node| node.rounding_f32.entry,
        |node| node.rounding_f32.exit,
        rounding_f16f64,
        |node| node.rounding_f16f64.entry,
        |node| node.rounding_f16f64.exit,
    )?;
    let all_modes = FullModeInsertion::new(flat_resolver, denormal, rounding)?;
    let directives = insert_mode_control(directives, all_modes)?;
    Ok(directives)
}

// For every basic block this pass computes:
// - Name of mode prologue basic block. Mode prologue is a basic block which
//   contains single instruction that sets mode to the desired value. It will
//   be later inserted just before the basic block and all jumps that require
//   mode change will go through this basic block
// - Entry mode: what is the mode for both f32 and f16f64 at the first instruction.
//   This will be used when emiting instructions in the basic block. When we
//   emit an instruction we get its modes, check if they are different and if so
//   decide: do we emit new mode set statement or we fold into previous mode set.
// We don't need to compute exit mode because this will be computed naturally
// when emitting instructions in a basic block. We need exit mode to know if we
// jump directly to the next bb or jump to mode prologue
fn join_modes<'input, T: Eq + PartialEq + Copy + Default>(
    flat_resolver: &mut super::GlobalStringIdentResolver2<'input>,
    cfg: &ControlFlowGraph,
    f32_insertions: MandatoryModeInsertions<T>,
    mut f32_enter_view: impl FnMut(&Node) -> Option<ExtendedMode<T>>,
    mut f32_exit_view: impl FnMut(&Node) -> Option<ExtendedMode<T>>,
    f16f64_insertions: MandatoryModeInsertions<T>,
    mut f16f64_enter_view: impl FnMut(&Node) -> Option<ExtendedMode<T>>,
    mut f16f64_exit_view: impl FnMut(&Node) -> Option<ExtendedMode<T>>,
) -> Result<TwinModeInsertions<T>, TranslateError> {
    // Returns None if there are multiple conflicting modes
    fn get_incoming_mode<T: Eq + PartialEq + Copy + Default>(
        cfg: &ControlFlowGraph,
        kernels: &FxHashMap<SpirvWord, T>,
        node: NodeIndex,
        mut exit_getter: impl FnMut(&Node) -> Option<ExtendedMode<T>>,
    ) -> Result<Option<T>, TranslateError> {
        let mut mode: Option<T> = None;
        let mut visited = iter::once(node).collect::<FxHashSet<_>>();
        let mut to_visit = cfg
            .graph
            .neighbors_directed(node, Direction::Incoming)
            .map(|x| x)
            .collect::<Vec<_>>();
        while let Some(node) = to_visit.pop() {
            if !visited.insert(node) {
                continue;
            }
            let node_data = &cfg.graph[node];
            match (mode, exit_getter(node_data)) {
                (_, None) => {
                    for next in cfg.graph.neighbors_directed(node, Direction::Incoming) {
                        if !visited.contains(&next) {
                            to_visit.push(next);
                        }
                    }
                }
                (existing_mode, Some(new_mode)) => {
                    let new_mode = match new_mode {
                        ExtendedMode::BasicBlock(new_mode) => new_mode,
                        ExtendedMode::Entry(kernel) => {
                            kernels.get(&kernel).copied().unwrap_or_default()
                        }
                    };
                    if let Some(existing_mode) = existing_mode {
                        if existing_mode != new_mode {
                            return Ok(None);
                        }
                    }
                    mode = Some(new_mode);
                }
            }
        }
        mode.map(Some).ok_or_else(error_unreachable)
    }
    let basic_blocks = cfg
        .graph
        .node_references()
        .into_iter()
        .map(|(node, basic_block)| {
            let requires_prologue = f32_insertions.basic_blocks.contains(&basic_block.label)
                || f16f64_insertions.basic_blocks.contains(&basic_block.label);
            let prologue: Option<SpirvWord> = if requires_prologue {
                Some(flat_resolver.register_unnamed(None))
            } else {
                None
            };
            let f32 = match f32_enter_view(&basic_block) {
                Some(ExtendedMode::BasicBlock(mode)) => Some(mode),
                Some(ExtendedMode::Entry(kernel)) => Some(
                    f32_insertions
                        .kernels
                        .get(&kernel)
                        .copied()
                        .unwrap_or_default(),
                ),
                // None means that no instruction in the basic block sets mode, but
                // another basic block might rely on this instruction transitively
                // passing a mode
                None => None,
            };
            let f16f64 = match f16f64_enter_view(&basic_block) {
                Some(ExtendedMode::BasicBlock(mode)) => Some(mode),
                Some(ExtendedMode::Entry(kernel)) => Some(
                    f16f64_insertions
                        .kernels
                        .get(&kernel)
                        .copied()
                        .unwrap_or_default(),
                ),
                None => None,
            };
            let twin_mode = match (f32, f16f64) {
                (Some(f32), Some(f16f64)) => Some(TwinMode { f32, f16f64 }),
                (None, Some(f16f64)) => {
                    let f32 = get_incoming_mode(cfg, &f32_insertions.kernels, node, |node| {
                        f32_exit_view(node)
                    })?;
                    let f32 = f32.unwrap_or_default();
                    Some(TwinMode { f32, f16f64 })
                }
                (Some(f32), None) => {
                    let f16f64 =
                        get_incoming_mode(cfg, &f16f64_insertions.kernels, node, |node| {
                            f16f64_exit_view(node)
                        })?;
                    let f16f64 = f16f64.unwrap_or_default();
                    Some(TwinMode { f32, f16f64 })
                }
                (None, None) => None,
            };
            Ok((
                basic_block.label,
                BasicBlockEntryState {
                    prologue,
                    twin_mode,
                },
            ))
        })
        .collect::<Result<FxHashMap<_, _>, _>>()?;
    Ok(TwinModeInsertions { basic_blocks })
}

struct TwinModeInsertions<T> {
    basic_blocks: FxHashMap<SpirvWord, BasicBlockEntryState<T>>,
}

struct FullModeInsertion {
    basic_blocks: FxHashMap<SpirvWord, FullBasicBlockEntryState>,
}

impl FullModeInsertion {
    fn new(
        flat_resolver: &mut super::GlobalStringIdentResolver2,
        denormal: TwinModeInsertions<DenormalMode>,
        rounding: TwinModeInsertions<RoundingMode>,
    ) -> Result<Self, TranslateError> {
        let denormal = denormal.basic_blocks;
        let rounding = rounding.basic_blocks;
        if denormal.len() != rounding.len() {
            return Err(error_unreachable());
        }
        let basic_blocks = denormal
            .into_iter()
            .map(|(bb, denormal)| {
                let rounding = rounding.get(&bb).copied().ok_or_else(error_unreachable)?;
                let dual_prologue = if denormal.prologue.is_some() && rounding.prologue.is_some() {
                    Some(flat_resolver.register_unnamed(None))
                } else {
                    None
                };
                Ok((
                    bb,
                    FullBasicBlockEntryState {
                        dual_prologue,
                        denormal,
                        rounding,
                    },
                ))
            })
            .collect::<Result<FxHashMap<_, _>, _>>()?;
        Ok(Self { basic_blocks })
    }
}

struct FullBasicBlockEntryState {
    dual_prologue: Option<SpirvWord>,
    denormal: BasicBlockEntryState<DenormalMode>,
    rounding: BasicBlockEntryState<RoundingMode>,
}

#[derive(Clone, Copy)]
struct BasicBlockEntryState<T> {
    prologue: Option<SpirvWord>,
    // It is None in case where no instructions in the basic block uses mode
    twin_mode: Option<TwinMode<T>>,
}

#[derive(Clone, Copy, Default)]
struct TwinMode<T> {
    f32: T,
    f16f64: T,
}

fn insert_mode_control<'input>(
    directives: Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>,
    global_modes: FullModeInsertion,
) -> Result<Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>, TranslateError> {
    let directives_len = directives.len();
    directives
        .into_iter()
        .map(|mut directive| {
            let mut new_directives = SmallVec::<[_; 4]>::new();
            let (fn_name, initial_mode, body_ptr) = match directive {
                Directive2::Variable(..) | Directive2::Method(Function2 { body: None, .. }) => {
                    new_directives.push(directive);
                    return Ok(new_directives);
                }
                Directive2::Method(Function2 {
                    name,
                    body: Some(ref mut body),
                    ref mut flush_to_zero_f32,
                    ref mut flush_to_zero_f16f64,
                    ref mut rounding_mode_f32,
                    ref mut rounding_mode_f16f64,
                    ..
                }) => {
                    let initial_mode = global_modes
                        .basic_blocks
                        .get(&name)
                        .ok_or_else(error_unreachable)?;
                    *flush_to_zero_f32 = initial_mode
                        .denormal
                        .twin_mode
                        .unwrap_or_default()
                        .f32
                        .to_ftz();
                    *flush_to_zero_f16f64 = initial_mode
                        .denormal
                        .twin_mode
                        .unwrap_or_default()
                        .f16f64
                        .to_ftz();
                    *rounding_mode_f32 = initial_mode
                        .rounding
                        .twin_mode
                        .unwrap_or_default()
                        .f32
                        .to_ast();
                    *rounding_mode_f16f64 = initial_mode
                        .rounding
                        .twin_mode
                        .unwrap_or_default()
                        .f16f64
                        .to_ast();
                    (name, initial_mode, body)
                }
            };
            let old_body = mem::replace(body_ptr, Vec::new());
            let mut result = Vec::with_capacity(old_body.len());
            let mut bb_state = BasicBlockControlState::new(&global_modes, fn_name, initial_mode);
            let mut old_body = old_body.into_iter();
            while let Some(mut statement) = old_body.next() {
                let mut call_target = None;
                match &mut statement {
                    Statement::Label(label) => {
                        bb_state.start(*label, &mut result)?;
                    }
                    Statement::Instruction(ast::Instruction::Call {
                        arguments: ast::CallArgs { func, .. },
                        ..
                    }) => {
                        bb_state.redirect_jump(func)?;
                        call_target = Some(*func);
                    }
                    Statement::Conditional(BrachCondition {
                        if_true, if_false, ..
                    }) => {
                        bb_state.redirect_jump(if_true)?;
                        bb_state.redirect_jump(if_false)?;
                    }
                    Statement::Instruction(ast::Instruction::Bra {
                        arguments: ptx_parser::BraArgs { src },
                    }) => {
                        bb_state.redirect_jump(src)?;
                    }
                    Statement::Instruction(instruction) => {
                        let modes = get_modes(&instruction);
                        bb_state.insert(&mut result, modes)?;
                    }
                    _ => {}
                }
                result.push(statement);
                if let Some(call_target) = call_target {
                    if let Some(Statement::Instruction(ast::Instruction::Bra {
                        arguments: ast::BraArgs { src: post_call_label },
                    })) = old_body.next()
                    {
                        // get return block for the function, if there is a mode
                        // change between caller and callee then apply it here
                        todo!()
                    }
                }
            }
            *body_ptr = result;
            new_directives.push(directive);
            Ok(new_directives)
        })
        .try_fold(Vec::with_capacity(directives_len), |mut acc, d| {
            acc.extend(d?);
            Ok(acc)
        })
}

struct BasicBlockControlState<'a> {
    global_modes: &'a FullModeInsertion,
    denormal_f32: RegisterState<bool>,
    denormal_f16f64: RegisterState<bool>,
    rounding_f32: RegisterState<ast::RoundingMode>,
    rounding_f16f64: RegisterState<ast::RoundingMode>,
    current_bb: SpirvWord,
}

#[derive(Clone, Copy)]
struct RegisterState<T> {
    current_value: Option<T>,
    // This is slightly subtle: this value is Some iff there's a SetMode in this
    // basic block setting this mode, but on which no instruciton relies
    last_foldable: Option<usize>,
}

impl<T> RegisterState<T> {
    fn single(t: T) -> Self {
        RegisterState {
            last_foldable: None,
            current_value: Some(t),
        }
    }

    fn empty() -> Self {
        RegisterState {
            last_foldable: None,
            current_value: None,
        }
    }

    fn new<U: Copy>(computed: &BasicBlockEntryState<U>) -> (RegisterState<T>, RegisterState<T>)
    where
        U: Into<T>,
    {
        match computed.twin_mode {
            Some(ref mode) => (
                RegisterState::single(mode.f32.into()),
                RegisterState::single(mode.f16f64.into()),
            ),
            None => (RegisterState::empty(), RegisterState::empty()),
        }
    }
}

impl<'a> BasicBlockControlState<'a> {
    fn new(
        global_modes: &'a FullModeInsertion,
        current_bb: SpirvWord,
        initial_mode: &FullBasicBlockEntryState,
    ) -> Self {
        let (denormal_f32, denormal_f16f64) = RegisterState::new(&initial_mode.denormal);
        let (rounding_f32, rounding_f16f64) = RegisterState::new(&initial_mode.rounding);
        BasicBlockControlState {
            global_modes,
            denormal_f32,
            denormal_f16f64,
            rounding_f32,
            rounding_f16f64,
            current_bb,
        }
    }

    fn start(
        &mut self,
        basic_block: SpirvWord,
        statements: &mut Vec<Statement<ast::Instruction<SpirvWord>, SpirvWord>>,
    ) -> Result<(), TranslateError> {
        let bb_state = self
            .global_modes
            .basic_blocks
            .get(&basic_block)
            .ok_or_else(error_unreachable)?;

        let (denormal_f32, denormal_f16f64) = RegisterState::new(&bb_state.denormal);
        self.denormal_f32 = denormal_f32;
        self.denormal_f16f64 = denormal_f16f64;
        let (rounding_f32, rounding_f16f64) = RegisterState::new(&bb_state.rounding);
        self.rounding_f32 = rounding_f32;
        self.rounding_f16f64 = rounding_f16f64;
        if let Some(prologue) = bb_state.dual_prologue {
            statements.push(Statement::Label(prologue));
            let denormal = bb_state.denormal.twin_mode.ok_or_else(error_unreachable)?;
            statements.push(Statement::SetMode(ModeRegister::Denormal {
                f32: denormal.f32.to_ftz(),
                f16f64: denormal.f16f64.to_ftz(),
            }));
            let rounding = bb_state.rounding.twin_mode.ok_or_else(error_unreachable)?;
            statements.push(Statement::SetMode(ModeRegister::Rounding {
                f32: rounding.f32.to_ast(),
                f16f64: rounding.f16f64.to_ast(),
            }));
            statements.push(Statement::Instruction(ast::Instruction::Bra {
                arguments: ast::BraArgs { src: basic_block },
            }));
        }
        if let Some(prologue) = bb_state.denormal.prologue {
            statements.push(Statement::Label(prologue));
            let denormal = bb_state.denormal.twin_mode.ok_or_else(error_unreachable)?;
            statements.push(Statement::SetMode(ModeRegister::Denormal {
                f32: denormal.f32.to_ftz(),
                f16f64: denormal.f16f64.to_ftz(),
            }));
            statements.push(Statement::Instruction(ast::Instruction::Bra {
                arguments: ast::BraArgs { src: basic_block },
            }));
        }
        if let Some(prologue) = bb_state.rounding.prologue {
            statements.push(Statement::Label(prologue));
            let rounding = bb_state.rounding.twin_mode.ok_or_else(error_unreachable)?;
            statements.push(Statement::SetMode(ModeRegister::Rounding {
                f32: rounding.f32.to_ast(),
                f16f64: rounding.f16f64.to_ast(),
            }));
            statements.push(Statement::Instruction(ast::Instruction::Bra {
                arguments: ast::BraArgs { src: basic_block },
            }));
        }
        Ok(())
    }

    /*
    fn add_or_fold_mode_set(
        &mut self,
        result: &mut Vec<Statement<ast::Instruction<SpirvWord>, SpirvWord>>,
        new_mode: bool,
    ) -> Option<usize> {
        // try and fold into the other mode set
        if let RegisterState::Value(Some(other_index), other_value) = self.denormal_f16f64 {
            if let Some(Statement::SetMode(ModeRegister::DenormalF16F64(_))) =
                result.get_mut(other_index)
            {
                result[other_index] = Statement::SetMode(ModeRegister::DenormalBoth {
                    f32: new_mode,
                    f16f64: other_value,
                });
                return None;
            }
        }
        result.push(Statement::SetMode(ModeRegister::DenormalF32(new_mode)));
        Some(result.len() - 1)
    }
     */

    fn insert(
        &mut self,
        result: &mut Vec<Statement<ast::Instruction<SpirvWord>, SpirvWord>>,
        modes: InstructionModes,
    ) -> Result<(), TranslateError> {
        self.insert_one::<DenormalF32View>(result, modes.denormal_f32.map(DenormalMode::to_ftz))?;
        self.insert_one::<DenormalF16F64View>(
            result,
            modes.denormal_f16f64.map(DenormalMode::to_ftz),
        )?;
        Ok(())
    }

    fn insert_one<View: ModeView>(
        &mut self,
        result: &mut Vec<Statement<ast::Instruction<SpirvWord>, SpirvWord>>,
        mode: Option<View::Value>,
    ) -> Result<(), TranslateError> {
        fn set_fold_index<View: ModeView>(bb: &mut BasicBlockControlState, index: Option<usize>) {
            let mut reg = View::get_register(bb);
            reg.last_foldable = index;
            View::set_register(bb, reg);
        }
        let new_mode = unwrap_some_or!(mode, return Ok(()));
        //        if let Some(new_mode) = mode {
        let register_state = View::get_register(self);
        match register_state.current_value {
            Some(old) if old == new_mode => {
                set_fold_index::<View>(self, None);
            }
            _ => match register_state.last_foldable {
                // fold successful
                Some(index) => {
                    if let Some(Statement::SetMode(mode_set)) = result.get_mut(index) {
                        View::set_single_mode(mode_set, new_mode)?;
                        set_fold_index::<View>(self, None);
                    } else {
                        return Err(error_unreachable());
                    }
                }
                // fold failed, insert new instruction
                None => {
                    result.push(Statement::SetMode(View::new_mode(
                        new_mode,
                        View::TwinView::get_register(self)
                            .current_value
                            .ok_or_else(error_unreachable)?,
                    )));
                    set_fold_index::<View::TwinView>(self, Some(result.len() - 1));
                }
            },
        }
        //}
        Ok(())
    }

    // Return the index of the last insertion of SetMode with this mode
    /*
    fn add_or_fold_mode_set2<View: ModeView>(
        &self,
        result: &mut Vec<Statement<ast::Instruction<SpirvWord>, SpirvWord>>,
        new_mode: View::Value,
    ) -> Result<(), TranslateError> {
        // try and fold into the other mode set instruction
        View::get_register(bb)
        if let RegisterState { last_foldable:   } = View::TwinView::get_register(self) {
            if let Some(Statement::SetMode(register_mode)) = result.get_mut(twin_index) {
                if let Some(twin_mode) = View::TwinView::get_single_mode(register_mode) {
                    *register_mode = View::new_mode(new_mode, Some(twin_mode));
                    return twin_index;
                }
            }
        }
        result.push(Statement::SetMode(View::new_mode(new_mode, None)));
        result.len() - 1
    }
     */

    fn redirect_jump(&self, jump_target: &mut SpirvWord) -> Result<(), TranslateError> {
        let target = self
            .global_modes
            .basic_blocks
            .get(jump_target)
            .ok_or_else(error_unreachable)?;
        let jump_to_denormal = match (
            self.denormal_f32.current_value,
            self.denormal_f16f64.current_value,
        ) {
            (None, None) => false,
            (Some(current_f32), Some(current_f16f64)) => {
                if let Some(target_mode) = target.denormal.twin_mode {
                    target_mode.f32.to_ftz() != current_f32
                        || target_mode.f16f64.to_ftz() != current_f16f64
                } else {
                    false
                }
            }
            _ => return Err(error_unreachable()),
        };
        let jump_to_rounding = match (
            self.rounding_f32.current_value,
            self.rounding_f16f64.current_value,
        ) {
            (None, None) => false,
            (Some(current_f32), Some(current_f16f64)) => {
                if let Some(target_mode) = target.rounding.twin_mode {
                    target_mode.f32.to_ast() != current_f32
                        || target_mode.f16f64.to_ast() != current_f16f64
                } else {
                    false
                }
            }
            _ => return Err(error_unreachable()),
        };
        match (jump_to_denormal, jump_to_rounding) {
            (true, false) => {
                *jump_target = target.denormal.prologue.ok_or_else(error_unreachable)?;
            }
            (false, true) => {
                *jump_target = target.rounding.prologue.ok_or_else(error_unreachable)?;
            }
            (true, true) => {
                *jump_target = target.dual_prologue.ok_or_else(error_unreachable)?;
            }
            (false, false) => {}
        }
        Ok(())
    }
}

trait ModeView {
    type Value: PartialEq + Eq + Copy + Clone;
    type TwinView: ModeView<Value = Self::Value>;

    fn get_register(bb: &BasicBlockControlState) -> RegisterState<Self::Value>;
    fn set_register(bb: &mut BasicBlockControlState, reg: RegisterState<Self::Value>);
    fn new_mode(t: Self::Value, other: Self::Value) -> ModeRegister;
    fn set_single_mode(reg: &mut ModeRegister, x: Self::Value) -> Result<(), TranslateError>;
}

struct DenormalF32View;

impl ModeView for DenormalF32View {
    type Value = bool;
    type TwinView = DenormalF16F64View;

    fn get_register(bb: &BasicBlockControlState) -> RegisterState<Self::Value> {
        bb.denormal_f32
    }

    fn set_register(bb: &mut BasicBlockControlState, reg: RegisterState<Self::Value>) {
        bb.denormal_f32 = reg;
    }

    fn new_mode(f32: Self::Value, f16f64: Self::Value) -> ModeRegister {
        ModeRegister::Denormal { f32, f16f64 }
    }

    fn set_single_mode(reg: &mut ModeRegister, x: Self::Value) -> Result<(), TranslateError> {
        match reg {
            ModeRegister::Denormal { f32, f16f64: _ } => *f32 = x,
            ModeRegister::Rounding { .. } => return Err(error_unreachable()),
        }
        Ok(())
    }
}

struct DenormalF16F64View;

impl ModeView for DenormalF16F64View {
    type Value = bool;
    type TwinView = DenormalF32View;

    fn get_register(bb: &BasicBlockControlState) -> RegisterState<Self::Value> {
        bb.denormal_f16f64
    }

    fn set_register(bb: &mut BasicBlockControlState, reg: RegisterState<Self::Value>) {
        bb.denormal_f16f64 = reg;
    }

    fn new_mode(f16f64: Self::Value, f32: Self::Value) -> ModeRegister {
        ModeRegister::Denormal { f32, f16f64 }
    }

    fn set_single_mode(reg: &mut ModeRegister, x: Self::Value) -> Result<(), TranslateError> {
        match reg {
            ModeRegister::Denormal { f32: _, f16f64 } => *f16f64 = x,
            ModeRegister::Rounding { .. } => return Err(error_unreachable()),
        }
        Ok(())
    }
}

struct BasicBlockState<'a> {
    cfg: &'a mut ControlFlowGraph,
    node_index: Option<NodeIndex>,
    // If it's a kernel basic block then we don't track entry instruction mode
    entry: InstructionModes,
    exit: InstructionModes,
}

impl<'a> BasicBlockState<'a> {
    #[must_use]
    fn new<'x>(
        cfg: &'a mut ControlFlowGraph,
        fn_name: SpirvWord,
        body: &'x Vec<Statement<ast::Instruction<SpirvWord>, SpirvWord>>,
        is_kernel: bool,
    ) -> Result<
        (
            BasicBlockState<'a>,
            std::iter::Peekable<
                impl Iterator<Item = &'x Statement<ast::Instruction<SpirvWord>, SpirvWord>>,
            >,
        ),
        TranslateError,
    > {
        let entry_index = if is_kernel {
            cfg.add_entry_basic_block(fn_name)
        } else {
            cfg.get_or_add_basic_block(fn_name)
        };
        let mut body_iter = body.iter();
        let mut bb_state = Self {
            cfg,
            node_index: None,
            entry: InstructionModes::none(),
            exit: InstructionModes::none(),
        };
        match body_iter.next() {
            Some(Statement::Label(label)) => {
                bb_state.cfg.add_jump(entry_index, *label);
                bb_state.start(*label);
            }
            _ => return Err(error_unreachable()),
        };
        Ok((bb_state, body_iter.peekable()))
    }

    fn start(&mut self, label: SpirvWord) {
        self.end(&[]);
        self.node_index = Some(self.cfg.get_or_add_basic_block(label));
    }

    fn end(&mut self, jumps: &[SpirvWord]) -> Option<NodeIndex> {
        let node_index = self.node_index.take();
        let node_index = match node_index {
            Some(x) => x,
            None => return None,
        };
        for target in jumps {
            self.cfg.add_jump(node_index, *target);
        }
        self.cfg.set_modes(
            node_index,
            mem::replace(&mut self.entry, InstructionModes::none()),
            mem::replace(&mut self.exit, InstructionModes::none()),
        );
        Some(node_index)
    }

    fn record_call(
        &mut self,
        fn_call: SpirvWord,
        after_call_label: SpirvWord,
    ) -> Result<(), TranslateError> {
        self.end(&[fn_call]).ok_or_else(error_unreachable)?;
        let after_call_label = self.cfg.get_or_add_basic_block(after_call_label);
        let call_returns = self
            .cfg
            .call_returns
            .entry(fn_call)
            .or_insert_with(|| Vec::new());
        call_returns.push(after_call_label);
        Ok(())
    }

    fn record_ret(&mut self, fn_name: SpirvWord) -> Result<(), TranslateError> {
        let node_index = self.node_index.ok_or_else(error_unreachable)?;
        let previous_function_ret = self.cfg.functions_rets.insert(fn_name, node_index);
        // This pass relies on there being only a single `ret;` in a function
        if previous_function_ret.is_some() {
            return Err(error_unreachable());
        }
        Ok(())
    }

    fn append(&mut self, modes: InstructionModes) {
        modes.fold_into(&mut self.entry, &mut self.exit);
    }
}

impl<'a> Drop for BasicBlockState<'a> {
    fn drop(&mut self) {
        self.end(&[]);
    }
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
            if !visited.insert(current) {
                continue;
            }
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

#[derive(Debug)]
struct PartialModeInsertion<T> {
    bb_must_insert_mode: FxHashSet<SpirvWord>,
    bb_maybe_insert_mode: FxHashMap<SpirvWord, (T, FxHashSet<SpirvWord>)>,
}

// Only returns kernel mode insertions if a kernel is relevant to the optimization problem
fn optimize<
    T: Copy + Into<usize> + strum::VariantArray + std::fmt::Debug + Default,
    const N: usize,
>(
    partial: PartialModeInsertion<T>,
) -> MandatoryModeInsertions<T> {
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
    'iterate_kernels: for (kernel, modes) in kernel_modes {
        for (mode, var) in modes.into_iter().enumerate() {
            if solution[var] > 0.5 {
                kernels.insert(kernel, T::VARIANTS[mode]);
                continue 'iterate_kernels;
            }
        }
    }
    MandatoryModeInsertions {
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

struct MandatoryModeInsertions<T> {
    basic_blocks: FxHashSet<SpirvWord>,
    kernels: FxHashMap<SpirvWord, T>,
}

#[derive(Eq, PartialEq, Clone, Copy)]
//#[cfg_attr(test, derive(Debug))]
#[derive(Debug)]
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
        | ast::Instruction::Mul24 { .. }
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
mod test;
