use super::BrachCondition;
use super::Directive2;
use super::Function2;
use super::GlobalStringIdentResolver2;
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

struct ResolvedInstructionModes {
    denormal_f32: Resolved<bool>,
    denormal_f16f64: Resolved<bool>,
    rounding_f32: Resolved<ast::RoundingMode>,
    rounding_f16f64: Resolved<ast::RoundingMode>,
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

    fn from_typed_denormal_rounding(
        from_type: ast::ScalarType,
        to_type: ast::ScalarType,
        denormal: DenormalMode,
        rounding: RoundingMode,
    ) -> Self {
        Self {
            rounding_f32: Some(rounding),
            rounding_f16f64: Some(rounding),
            ..Self::from_typed_denormal(from_type, to_type, denormal)
        }
    }

    // This function accepts DenormalMode and not Option<DenormalMode> because
    // the semantics are slightly different.
    // * In instructions `None` means: flush-to-zero has not been explicitly requested
    // * In this pass `None` means: neither flush-to-zero, nor preserve is applicable
    fn from_typed_denormal(
        from_type: ast::ScalarType,
        to_type: ast::ScalarType,
        denormal: DenormalMode,
    ) -> Self {
        let mut result = Self::none();
        if from_type == ast::ScalarType::F32 || to_type == ast::ScalarType::F32 {
            result.denormal_f32 = if denormal == DenormalMode::FlushToZero {
                Some(DenormalMode::FlushToZero)
            } else {
                Some(DenormalMode::Preserve)
            };
        }
        if !(from_type == ast::ScalarType::F32 && to_type == ast::ScalarType::F32) {
            result.denormal_f16f64 = Some(DenormalMode::Preserve);
        }
        result
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

    fn from_rtz_special(data: ast::RcpData) -> InstructionModes {
        let rounding = match data.kind {
            ast::RcpKind::Approx => None,
            ast::RcpKind::Compliant(rnd) => Some(RoundingMode::from_ast(rnd)),
        };
        let denormal = match (
            data.kind == ast::RcpKind::Approx,
            data.flush_to_zero == Some(true),
        ) {
            // If we are approximate and flushing then we compile to V_RSQ_F32
            // or V_SQRT_F32 which ignores prevailing denormal mode and flushes anyway
            (true, true) => None,
            // If we are approximate and flushing the V_RSQ_F32/V_SQRT_F32
            // ignores ftz mode, but we implement instruction in terms of fmuls
            // which must preserve denormals
            (true, false) => Some(DenormalMode::Preserve),
            // For full precision we set denormal mode accordingly
            (false, ftz) => Some(DenormalMode::from_ftz(ftz)),
        };
        InstructionModes::new(data.type_, denormal, rounding)
    }

    fn from_cvt(cvt: &ast::CvtDetails) -> InstructionModes {
        match cvt.mode {
            ast::CvtMode::ZeroExtend
            | ast::CvtMode::SignExtend
            | ast::CvtMode::Truncate
            | ast::CvtMode::Bitcast
            | ast::CvtMode::IntSaturateToSigned
            | ast::CvtMode::IntSaturateToUnsigned => Self::none(),
            ast::CvtMode::FPExtend { flush_to_zero, .. } => Self::from_typed_denormal(
                cvt.from,
                cvt.to,
                flush_to_zero
                    .map(DenormalMode::from_ftz)
                    .unwrap_or(DenormalMode::Preserve),
            ),
            ast::CvtMode::FPTruncate {
                rounding,
                flush_to_zero,
                is_integer_rounding,
                ..
            } => {
                let denormal_mode = match (is_integer_rounding, flush_to_zero) {
                    (true, Some(true)) => DenormalMode::FlushToZero,
                    _ => DenormalMode::Preserve,
                };
                Self::from_typed_denormal_rounding(
                    cvt.from,
                    cvt.to,
                    denormal_mode,
                    RoundingMode::from_ast(rounding),
                )
            }
            ast::CvtMode::FPRound { flush_to_zero, .. } => Self::from_typed_denormal(
                cvt.from,
                cvt.to,
                flush_to_zero
                    .map(DenormalMode::from_ftz)
                    .unwrap_or(DenormalMode::Preserve),
            ),
            // float to int contains rounding field, but it's not a rounding
            // mode but rather round-to-int operation that will be applied
            ast::CvtMode::SignedFromFP { flush_to_zero, .. }
            | ast::CvtMode::UnsignedFromFP { flush_to_zero, .. } => Self::from_typed_denormal(
                cvt.from,
                cvt.from,
                flush_to_zero
                    .map(DenormalMode::from_ftz)
                    .unwrap_or(DenormalMode::Preserve),
            ),
            ast::CvtMode::FPFromSigned { rounding, .. }
            | ast::CvtMode::FPFromUnsigned { rounding, .. } => {
                Self::new(cvt.to, None, Some(RoundingMode::from_ast(rounding)))
            }
        }
    }
}

struct ControlFlowGraph {
    entry_points: FxHashMap<SpirvWord, NodeIndex>,
    basic_blocks: FxHashMap<SpirvWord, NodeIndex>,
    // map function -> return label
    call_returns: FxHashMap<SpirvWord, Vec<NodeIndex>>,
    // map function -> return basic block
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

    // Our control flow graph expresses function calls as edges in the graph.
    // While building the graph it's always possible to create the edge from
    // caller basic block to a function, but it's impossible to construct an
    // edge from the function return basic block to after-call basic block in
    // caller (the function might have been just a declaration for now).
    // That's why we collect:
    // * Which basic blocks does a function return to
    // * What is thew functin's return basic blocks
    // and then, after visiting all functions, we add the missing edges here
    fn fixup_function_calls(&mut self) -> Result<(), TranslateError> {
        for (fn_, follow_on_labels) in self.call_returns.iter() {
            let connecting_bb = match self.functions_rets.get(fn_) {
                Some(return_bb) => *return_bb,
                // function is just a declaration
                None => *self.basic_blocks.get(fn_).ok_or_else(error_unreachable)?,
            };
            for follow_on_label in follow_on_labels {
                self.graph.add_edge(connecting_bb, *follow_on_label, ());
            }
        }
        Ok(())
    }
}

struct ResolvedControlFlowGraph {
    basic_blocks: FxHashMap<SpirvWord, NodeIndex>,
    // map function -> return basic block
    functions_rets: FxHashMap<SpirvWord, NodeIndex>,
    graph: Graph<ResolvedNode, ()>,
}

impl ResolvedControlFlowGraph {
    // This function takes the initial control flow graph. Initial control flow
    // graph only has mode values for basic blocks if any instruction in the
    // given basic block requires a mode. All the other basic blocks have no
    // value. This pass resolved the values for all basic blocks. If a basic
    // block sets no value then and there are multiple incoming edges from
    // basic block with different values then the value is set to a special
    // value "Conflict".
    // After this pass every basic block either has a concrete value or "Conflict"
    fn new(
        cfg: ControlFlowGraph,
        f32_denormal_kernels: &FxHashMap<SpirvWord, DenormalMode>,
        f16f64_denormal_kernels: &FxHashMap<SpirvWord, DenormalMode>,
        f32_rounding_kernels: &FxHashMap<SpirvWord, RoundingMode>,
        f16f64_rounding_kernels: &FxHashMap<SpirvWord, RoundingMode>,
    ) -> Result<Self, TranslateError> {
        fn get_incoming_mode<T: Eq + PartialEq + Copy + Default>(
            cfg: &ControlFlowGraph,
            kernels: &FxHashMap<SpirvWord, T>,
            node: NodeIndex,
            mut exit_getter: impl FnMut(&Node) -> Option<ExtendedMode<T>>,
        ) -> Result<Resolved<T>, TranslateError> {
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
                                return Ok(Resolved::Conflict);
                            }
                        }
                        mode = Some(new_mode);
                    }
                }
            }
            // This should happen only for orphaned basic blocks
            mode.map(Resolved::Value).ok_or_else(error_unreachable)
        }
        fn resolve_mode<T: Eq + PartialEq + Copy + Default>(
            cfg: &ControlFlowGraph,
            kernels: &FxHashMap<SpirvWord, T>,
            node: NodeIndex,
            exit_getter: impl FnMut(&Node) -> Option<ExtendedMode<T>>,
            mode: &Mode<T>,
        ) -> Result<ResolvedMode<T>, TranslateError> {
            let entry = match mode.entry {
                Some(ExtendedMode::Entry(kernel)) => {
                    Resolved::Value(kernels.get(&kernel).copied().unwrap_or_default())
                }
                Some(ExtendedMode::BasicBlock(bb)) => Resolved::Value(bb),
                None => get_incoming_mode(cfg, kernels, node, exit_getter)?,
            };
            let exit = match mode.entry {
                Some(ExtendedMode::BasicBlock(bb)) => Resolved::Value(bb),
                Some(ExtendedMode::Entry(_)) | None => entry,
            };
            Ok(ResolvedMode { entry, exit })
        }
        fn resolve_node_impl(
            cfg: &ControlFlowGraph,
            f32_denormal_kernels: &FxHashMap<SpirvWord, DenormalMode>,
            f16f64_denormal_kernels: &FxHashMap<SpirvWord, DenormalMode>,
            f32_rounding_kernels: &FxHashMap<SpirvWord, RoundingMode>,
            f16f64_rounding_kernels: &FxHashMap<SpirvWord, RoundingMode>,
            index: NodeIndex,
            node: &Node,
        ) -> Result<ResolvedNode, TranslateError> {
            let denormal_f32 = resolve_mode(
                cfg,
                f32_denormal_kernels,
                index,
                |node| node.denormal_f32.exit,
                &node.denormal_f32,
            )?;
            let denormal_f16f64 = resolve_mode(
                cfg,
                f16f64_denormal_kernels,
                index,
                |node| node.denormal_f16f64.exit,
                &node.denormal_f16f64,
            )?;
            let rounding_f32 = resolve_mode(
                cfg,
                f32_rounding_kernels,
                index,
                |node| node.rounding_f32.exit,
                &node.rounding_f32,
            )?;
            let rounding_f16f64 = resolve_mode(
                cfg,
                f16f64_rounding_kernels,
                index,
                |node| node.rounding_f16f64.exit,
                &node.rounding_f16f64,
            )?;
            Ok(ResolvedNode {
                label: node.label,
                denormal_f32,
                denormal_f16f64,
                rounding_f32,
                rounding_f16f64,
            })
        }
        fn resolve_node(
            cfg: &ControlFlowGraph,
            f32_denormal_kernels: &FxHashMap<SpirvWord, DenormalMode>,
            f16f64_denormal_kernels: &FxHashMap<SpirvWord, DenormalMode>,
            f32_rounding_kernels: &FxHashMap<SpirvWord, RoundingMode>,
            f16f64_rounding_kernels: &FxHashMap<SpirvWord, RoundingMode>,
            index: NodeIndex,
            node: &Node,
            error: &mut bool,
        ) -> ResolvedNode {
            match resolve_node_impl(
                cfg,
                f32_denormal_kernels,
                f16f64_denormal_kernels,
                f32_rounding_kernels,
                f16f64_rounding_kernels,
                index,
                node,
            ) {
                Ok(node) => node,
                Err(_) => {
                    *error = true;
                    ResolvedNode {
                        label: SpirvWord(u32::MAX),
                        denormal_f32: ResolvedMode {
                            entry: Resolved::Conflict,
                            exit: Resolved::Conflict,
                        },
                        denormal_f16f64: ResolvedMode {
                            entry: Resolved::Conflict,
                            exit: Resolved::Conflict,
                        },
                        rounding_f32: ResolvedMode {
                            entry: Resolved::Conflict,
                            exit: Resolved::Conflict,
                        },
                        rounding_f16f64: ResolvedMode {
                            entry: Resolved::Conflict,
                            exit: Resolved::Conflict,
                        },
                    }
                }
            }
        }
        let mut error = false;
        let graph = cfg.graph.map(
            |index, node| {
                resolve_node(
                    &cfg,
                    f32_denormal_kernels,
                    f16f64_denormal_kernels,
                    f32_rounding_kernels,
                    f16f64_rounding_kernels,
                    index,
                    node,
                    &mut error,
                )
            },
            |_, ()| (),
        );
        if error {
            Err(error_unreachable())
        } else {
            Ok(Self {
                basic_blocks: cfg.basic_blocks,
                functions_rets: cfg.functions_rets,
                graph,
            })
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

#[derive(Copy, Clone)]
struct ResolvedMode<T> {
    entry: Resolved<T>,
    exit: Resolved<T>,
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

struct ResolvedNode {
    label: SpirvWord,
    denormal_f32: ResolvedMode<DenormalMode>,
    denormal_f16f64: ResolvedMode<DenormalMode>,
    rounding_f32: ResolvedMode<RoundingMode>,
    rounding_f16f64: ResolvedMode<RoundingMode>,
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

// This instruction convert instruction-scoped modes (denormal, rounding) in PTX
// to globally-scoped modes as expected by AMD GPUs.
// As a simplified example this pass converts this instruction:
//      add.ftz.rn.f32 %r1, %r2, %r3;
// to:
//      set_ftz_mode true;
//      set_rnd_mode rn;
//      add.ftz.rn.f32 %r1, %r2, %r3;
pub(crate) fn run<'input>(
    flat_resolver: &mut GlobalStringIdentResolver2<'input>,
    directives: Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>,
) -> Result<Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>, TranslateError> {
    let cfg = create_control_flow_graph(&directives)?;
    let (denormal_f32, denormal_f16f64, rounding_f32, rounding_f16f64) =
        compute_minimal_mode_insertions(&cfg);
    let temp = compute_full_mode_insertions(
        flat_resolver,
        &directives,
        cfg,
        denormal_f32,
        denormal_f16f64,
        rounding_f32,
        rounding_f16f64,
    )?;
    apply_global_mode_controls(directives, temp)
}

// For every basic block this pass computes:
// - Name of mode prologue basic blocks. Mode prologue is a basic block which
//   contains single instruction that sets mode to the desired value. It will
//   be later inserted just before the basic block and all jumps that require
//   mode change will go through this basic block
// - Entry mode: what is the mode for both f32 and f16f64 at the first instruction.
//   This will be used when emiting instructions in the basic block. When we
//   emit an instruction we get its modes, check if they are different and if so
//   decide: do we emit new mode set statement or we fold into previous mode set.
// We don't need to compute exit mode for every basic block because this will be
// computed naturally when emitting instructions in a basic block.
// Only exception is exit mode for returning (containing instruction `ret;`)
// basic blocks for functions.
// We need this information to handle call instructions correctly.
fn compute_full_mode_insertions(
    flat_resolver: &mut GlobalStringIdentResolver2,
    directives: &Vec<Directive2<ptx_parser::Instruction<SpirvWord>, SpirvWord>>,
    cfg: ControlFlowGraph,
    denormal_f32: MandatoryModeInsertions<DenormalMode>,
    denormal_f16f64: MandatoryModeInsertions<DenormalMode>,
    rounding_f32: MandatoryModeInsertions<RoundingMode>,
    rounding_f16f64: MandatoryModeInsertions<RoundingMode>,
) -> Result<FullModeInsertion, TranslateError> {
    let cfg = ResolvedControlFlowGraph::new(
        cfg,
        &denormal_f32.kernels,
        &denormal_f16f64.kernels,
        &rounding_f32.kernels,
        &rounding_f16f64.kernels,
    )?;
    join_modes(
        flat_resolver,
        directives,
        cfg,
        denormal_f32,
        denormal_f16f64,
        rounding_f32,
        rounding_f16f64,
    )
}

// This function takes the control flow graph and for each global mode computes:
// * Which basic blocks have an incoming edge from at least one basic block with
//   different mode. That means that we will later need to insert a mode
//   "prologue": an artifical basic block which sets the mode to the desired
//   value. All mode-changing edges will be redirected to than basic block
// * What is the initial value for the mode in a kernel. Note, that only
//   computes the initial value if the value is observed by a basic block.
//   For some kernels the initial value does not matter and in that case a later
//   pass should use default value
fn compute_minimal_mode_insertions(
    cfg: &ControlFlowGraph,
) -> (
    MandatoryModeInsertions<DenormalMode>,
    MandatoryModeInsertions<DenormalMode>,
    MandatoryModeInsertions<RoundingMode>,
    MandatoryModeInsertions<RoundingMode>,
) {
    let rounding_f32 = compute_single_mode_insertions(cfg, |node| node.rounding_f32);
    let denormal_f32 = compute_single_mode_insertions(cfg, |node| node.denormal_f32);
    let denormal_f16f64 = compute_single_mode_insertions(cfg, |node| node.denormal_f16f64);
    let rounding_f16f64 = compute_single_mode_insertions(cfg, |node| node.rounding_f16f64);
    let denormal_f32 =
        optimize_mode_insertions::<DenormalMode, { DenormalMode::COUNT }>(denormal_f32);
    let denormal_f16f64 =
        optimize_mode_insertions::<DenormalMode, { DenormalMode::COUNT }>(denormal_f16f64);
    let rounding_f32 =
        optimize_mode_insertions::<RoundingMode, { RoundingMode::COUNT }>(rounding_f32);
    let rounding_f16f64: MandatoryModeInsertions<RoundingMode> =
        optimize_mode_insertions::<RoundingMode, { RoundingMode::COUNT }>(rounding_f16f64);
    (denormal_f32, denormal_f16f64, rounding_f32, rounding_f16f64)
}

// This function creates control flow graph for the whole module. This control
// flow graph expresses function calls as edges in the control flow graph
fn create_control_flow_graph(
    directives: &Vec<Directive2<ptx_parser::Instruction<SpirvWord>, SpirvWord>>,
) -> Result<ControlFlowGraph, TranslateError> {
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
                        }
                        Statement::RetValue(..)
                        | Statement::Instruction(ast::Instruction::Ret { .. }) => {
                            if !is_kernel {
                                bb_state.record_ret(*name)?;
                            }
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
    cfg.fixup_function_calls()?;
    Ok(cfg)
}

fn join_modes(
    flat_resolver: &mut super::GlobalStringIdentResolver2,
    directives: &Vec<super::Directive2<ast::Instruction<SpirvWord>, super::SpirvWord>>,
    cfg: ResolvedControlFlowGraph,
    mandatory_denormal_f32: MandatoryModeInsertions<DenormalMode>,
    mandatory_denormal_f16f64: MandatoryModeInsertions<DenormalMode>,
    mandatory_rounding_f32: MandatoryModeInsertions<RoundingMode>,
    mandatory_rounding_f16f64: MandatoryModeInsertions<RoundingMode>,
) -> Result<FullModeInsertion, TranslateError> {
    let basic_blocks = cfg
        .graph
        .node_weights()
        .map(|basic_block| {
            let denormal_prologue = if mandatory_denormal_f32
                .basic_blocks
                .contains(&basic_block.label)
                || mandatory_denormal_f16f64
                    .basic_blocks
                    .contains(&basic_block.label)
            {
                Some(flat_resolver.register_unnamed(None))
            } else {
                None
            };
            let rounding_prologue = if mandatory_rounding_f32
                .basic_blocks
                .contains(&basic_block.label)
                || mandatory_rounding_f16f64
                    .basic_blocks
                    .contains(&basic_block.label)
            {
                Some(flat_resolver.register_unnamed(None))
            } else {
                None
            };
            let dual_prologue = if denormal_prologue.is_some() && rounding_prologue.is_some() {
                Some(flat_resolver.register_unnamed(None))
            } else {
                None
            };
            let denormal = BasicBlockEntryState {
                prologue: denormal_prologue,
                twin_mode: TwinMode {
                    f32: basic_block.denormal_f32.entry,
                    f16f64: basic_block.denormal_f16f64.entry,
                },
            };
            let rounding = BasicBlockEntryState {
                prologue: rounding_prologue,
                twin_mode: TwinMode {
                    f32: basic_block.rounding_f32.entry,
                    f16f64: basic_block.rounding_f16f64.entry,
                },
            };
            Ok((
                basic_block.label,
                FullBasicBlockEntryState {
                    dual_prologue,
                    denormal,
                    rounding,
                },
            ))
        })
        .collect::<Result<FxHashMap<_, _>, _>>()?;
    let functions_exit_modes = directives
        .iter()
        .filter_map(|directive| match directive {
            Directive2::Method(Function2 {
                name,
                body: None,
                is_kernel: false,
                ..
            }) => {
                let fn_bb = match cfg.basic_blocks.get(name) {
                    Some(bb) => bb,
                    None => return None,
                };
                let weights = cfg.graph.node_weight(*fn_bb).unwrap();
                let modes = ResolvedInstructionModes {
                    denormal_f32: weights.denormal_f32.exit.map(DenormalMode::to_ftz),
                    denormal_f16f64: weights.denormal_f16f64.exit.map(DenormalMode::to_ftz),
                    rounding_f32: weights.rounding_f32.exit.map(RoundingMode::to_ast),
                    rounding_f16f64: weights.rounding_f16f64.exit.map(RoundingMode::to_ast),
                };
                Some(Ok((*name, modes)))
            }
            Directive2::Method(Function2 {
                name,
                body: Some(_),
                is_kernel: false,
                ..
            }) => {
                let ret_bb = cfg.functions_rets.get(name).unwrap();
                let weights = cfg.graph.node_weight(*ret_bb).unwrap();
                let modes = ResolvedInstructionModes {
                    denormal_f32: weights.denormal_f32.exit.map(DenormalMode::to_ftz),
                    denormal_f16f64: weights.denormal_f16f64.exit.map(DenormalMode::to_ftz),
                    rounding_f32: weights.rounding_f32.exit.map(RoundingMode::to_ast),
                    rounding_f16f64: weights.rounding_f16f64.exit.map(RoundingMode::to_ast),
                };
                Some(Ok((*name, modes)))
            }
            _ => None,
        })
        .collect::<Result<FxHashMap<_, _>, _>>()?;
    Ok(FullModeInsertion {
        basic_blocks,
        functions_exit_modes,
    })
}

struct FullModeInsertion {
    basic_blocks: FxHashMap<SpirvWord, FullBasicBlockEntryState>,
    functions_exit_modes: FxHashMap<SpirvWord, ResolvedInstructionModes>,
}

struct FullBasicBlockEntryState {
    dual_prologue: Option<SpirvWord>,
    denormal: BasicBlockEntryState<DenormalMode>,
    rounding: BasicBlockEntryState<RoundingMode>,
}

#[derive(Clone, Copy)]
struct BasicBlockEntryState<T> {
    prologue: Option<SpirvWord>,
    twin_mode: TwinMode<Resolved<T>>,
}

#[derive(Clone, Copy)]
struct TwinMode<T> {
    f32: T,
    f16f64: T,
}

// This function goes through every method, every basic block, every instruction
// and based on computed information inserts:
// * Instructions that change global mode
// * Insert additional "prelude" basic blocks that sets mode
// * Redirect some jumps to "prelude" basic blocks
fn apply_global_mode_controls(
    directives: Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>,
    global_modes: FullModeInsertion,
) -> Result<Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>, TranslateError> {
    directives
        .into_iter()
        .map(|directive| {
            let (mut method, initial_mode) = match directive {
                Directive2::Variable(..) | Directive2::Method(Function2 { body: None, .. }) => {
                    return Ok(directive);
                }
                Directive2::Method(
                    mut method @ Function2 {
                        name,
                        body: Some(_),
                        ..
                    },
                ) => {
                    let initial_mode = global_modes
                        .basic_blocks
                        .get(&name)
                        .ok_or_else(error_unreachable)?;
                    let denormal_mode = initial_mode.denormal.twin_mode;
                    let rounding_mode = initial_mode.rounding.twin_mode;
                    method.flush_to_zero_f32 =
                        denormal_mode.f32.ok_or_else(error_unreachable)?.to_ftz();
                    method.flush_to_zero_f16f64 =
                        denormal_mode.f16f64.ok_or_else(error_unreachable)?.to_ftz();
                    method.rounding_mode_f32 =
                        rounding_mode.f32.ok_or_else(error_unreachable)?.to_ast();
                    method.rounding_mode_f16f64 =
                        rounding_mode.f16f64.ok_or_else(error_unreachable)?.to_ast();
                    (method, initial_mode)
                }
            };
            check_function_prelude(&method, &global_modes)?;
            let old_body = method.body.take().unwrap();
            let mut result = Vec::with_capacity(old_body.len());
            let mut bb_state = BasicBlockControlState::new(&global_modes, initial_mode);
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
                    let mut post_call_bra = old_body.next().ok_or_else(error_unreachable)?;
                    if let Statement::Instruction(ast::Instruction::Bra {
                        arguments:
                            ast::BraArgs {
                                src: ref mut post_call_label,
                            },
                    }) = post_call_bra
                    {
                        let node_exit_mode = global_modes
                            .functions_exit_modes
                            .get(&call_target)
                            .ok_or_else(error_unreachable)?;
                        redirect_jump_impl(
                            &bb_state.global_modes,
                            node_exit_mode,
                            post_call_label,
                        )?;
                        result.push(post_call_bra);
                    } else {
                        return Err(error_unreachable());
                    }
                }
            }
            method.body = Some(result);
            Ok(Directive2::Method(method))
        })
        .collect::<Result<Vec<_>, _>>()
}

fn check_function_prelude(
    method: &Function2<ast::Instruction<SpirvWord>, SpirvWord>,
    global_modes: &FullModeInsertion,
) -> Result<(), TranslateError> {
    let fn_mode_state = global_modes
        .basic_blocks
        .get(&method.name)
        .ok_or_else(error_unreachable)?;
    // A function should never have a prelude. Preludes happen only if there
    // is an edge in the control flow graph that requires a mode change.
    // Since functions never have a mode setting instructions that means they
    // only pass the mode from incoming edges to outgoing edges
    if fn_mode_state.dual_prologue.is_some()
        || fn_mode_state.denormal.prologue.is_some()
        || fn_mode_state.rounding.prologue.is_some()
    {
        return Err(error_unreachable());
    }
    Ok(())
}

struct BasicBlockControlState<'a> {
    global_modes: &'a FullModeInsertion,
    denormal_f32: RegisterState<bool>,
    denormal_f16f64: RegisterState<bool>,
    rounding_f32: RegisterState<ast::RoundingMode>,
    rounding_f16f64: RegisterState<ast::RoundingMode>,
}

#[derive(Clone, Copy)]
struct RegisterState<T> {
    current_value: Resolved<T>,
    // This is slightly subtle: this value is Some iff there's a SetMode in this
    // basic block setting this mode, but on which no instruciton relies
    last_foldable: Option<usize>,
}

impl<T> RegisterState<T> {
    fn new<U>(value: Resolved<U>) -> RegisterState<T>
    where
        U: Into<T>,
    {
        RegisterState {
            current_value: value.map(Into::into),
            last_foldable: None,
        }
    }
}

impl<'a> BasicBlockControlState<'a> {
    fn new(global_modes: &'a FullModeInsertion, initial_mode: &FullBasicBlockEntryState) -> Self {
        let denormal_f32 = RegisterState::new(initial_mode.denormal.twin_mode.f32);
        let denormal_f16f64 = RegisterState::new(initial_mode.denormal.twin_mode.f16f64);
        let rounding_f32 = RegisterState::new(initial_mode.rounding.twin_mode.f32);
        let rounding_f16f64 = RegisterState::new(initial_mode.rounding.twin_mode.f16f64);
        BasicBlockControlState {
            global_modes,
            denormal_f32,
            denormal_f16f64,
            rounding_f32,
            rounding_f16f64,
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

        let denormal_f32 = RegisterState::new(bb_state.denormal.twin_mode.f32);
        let denormal_f16f64 = RegisterState::new(bb_state.denormal.twin_mode.f16f64);
        self.denormal_f32 = denormal_f32;
        self.denormal_f16f64 = denormal_f16f64;
        let rounding_f32 = RegisterState::new(bb_state.rounding.twin_mode.f32);
        let rounding_f16f64 = RegisterState::new(bb_state.rounding.twin_mode.f16f64);
        self.rounding_f32 = rounding_f32;
        self.rounding_f16f64 = rounding_f16f64;
        if let Some(prologue) = bb_state.dual_prologue {
            statements.push(Statement::Label(prologue));
            statements.push(Statement::SetMode(ModeRegister::Denormal {
                f32: bb_state.denormal.twin_mode.f32.unwrap_or_default().to_ftz(),
                f16f64: bb_state
                    .denormal
                    .twin_mode
                    .f16f64
                    .unwrap_or_default()
                    .to_ftz(),
            }));
            statements.push(Statement::SetMode(ModeRegister::Rounding {
                f32: bb_state.rounding.twin_mode.f32.unwrap_or_default().to_ast(),
                f16f64: bb_state
                    .rounding
                    .twin_mode
                    .f16f64
                    .unwrap_or_default()
                    .to_ast(),
            }));
            statements.push(Statement::Instruction(ast::Instruction::Bra {
                arguments: ast::BraArgs { src: basic_block },
            }));
        }
        if let Some(prologue) = bb_state.denormal.prologue {
            statements.push(Statement::Label(prologue));
            statements.push(Statement::SetMode(ModeRegister::Denormal {
                f32: bb_state.denormal.twin_mode.f32.unwrap_or_default().to_ftz(),
                f16f64: bb_state
                    .denormal
                    .twin_mode
                    .f16f64
                    .unwrap_or_default()
                    .to_ftz(),
            }));
            statements.push(Statement::Instruction(ast::Instruction::Bra {
                arguments: ast::BraArgs { src: basic_block },
            }));
        }
        if let Some(prologue) = bb_state.rounding.prologue {
            statements.push(Statement::Label(prologue));
            statements.push(Statement::SetMode(ModeRegister::Rounding {
                f32: bb_state.rounding.twin_mode.f32.unwrap_or_default().to_ast(),
                f16f64: bb_state
                    .rounding
                    .twin_mode
                    .f16f64
                    .unwrap_or_default()
                    .to_ast(),
            }));
            statements.push(Statement::Instruction(ast::Instruction::Bra {
                arguments: ast::BraArgs { src: basic_block },
            }));
        }
        Ok(())
    }

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
        self.insert_one::<RoundingF32View>(result, modes.rounding_f32.map(RoundingMode::to_ast))?;
        self.insert_one::<RoundingF16F64View>(
            result,
            modes.rounding_f16f64.map(RoundingMode::to_ast),
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
        let register_state = View::get_register(self);
        match register_state.current_value {
            Resolved::Conflict => {
                return Err(error_unreachable());
            }
            Resolved::Value(old) if old == new_mode => {
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
                            .unwrap_or(View::ComputeValue::default().into()),
                    )));
                    View::set_register(
                        self,
                        RegisterState {
                            current_value: Resolved::Value(new_mode),
                            last_foldable: None,
                        },
                    );
                    set_fold_index::<View::TwinView>(self, Some(result.len() - 1));
                }
            },
        }
        Ok(())
    }

    fn redirect_jump(&self, jump_target: &mut SpirvWord) -> Result<(), TranslateError> {
        let current_mode = ResolvedInstructionModes {
            denormal_f32: self.denormal_f32.current_value,
            denormal_f16f64: self.denormal_f16f64.current_value,
            rounding_f32: self.rounding_f32.current_value,
            rounding_f16f64: self.rounding_f16f64.current_value,
        };
        redirect_jump_impl(self.global_modes, &current_mode, jump_target)
    }
}

fn redirect_jump_impl(
    global_modes: &FullModeInsertion,
    current_mode: &ResolvedInstructionModes,
    jump_target: &mut SpirvWord,
) -> Result<(), TranslateError> {
    let target = global_modes
        .basic_blocks
        .get(jump_target)
        .ok_or_else(error_unreachable)?;
    let jump_to_denormal_prelude = current_mode
        .denormal_f32
        .mode_change(target.denormal.twin_mode.f32.map(DenormalMode::to_ftz))
        || current_mode
            .denormal_f16f64
            .mode_change(target.denormal.twin_mode.f16f64.map(DenormalMode::to_ftz));
    let jump_to_rounding_prelude = current_mode
        .rounding_f32
        .mode_change(target.rounding.twin_mode.f32.map(RoundingMode::to_ast))
        || current_mode
            .rounding_f16f64
            .mode_change(target.rounding.twin_mode.f16f64.map(RoundingMode::to_ast));
    match (jump_to_denormal_prelude, jump_to_rounding_prelude) {
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

#[derive(Copy, Clone)]
enum Resolved<T> {
    Conflict,
    Value(T),
}

impl<T: Default> Resolved<T> {
    fn unwrap_or_default(self) -> T {
        match self {
            Resolved::Conflict => T::default(),
            Resolved::Value(t) => t,
        }
    }
}

impl<T: Eq + PartialEq> Resolved<T> {
    fn mode_change(self, target: Self) -> bool {
        match (self, target) {
            (Resolved::Conflict, Resolved::Conflict) => false,
            (Resolved::Conflict, Resolved::Value(_)) => true,
            (Resolved::Value(_), Resolved::Conflict) => false,
            (Resolved::Value(x), Resolved::Value(y)) => x != y,
        }
    }
}

impl<T> Resolved<T> {
    fn unwrap_or(self, if_fail: T) -> T {
        match self {
            Resolved::Conflict => if_fail,
            Resolved::Value(t) => t,
        }
    }

    fn map<U, F>(self, f: F) -> Resolved<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Resolved::Value(x) => Resolved::Value(f(x)),
            Resolved::Conflict => Resolved::Conflict,
        }
    }

    fn ok_or_else<E, F>(self, err: F) -> Result<T, E>
    where
        F: FnOnce() -> E,
    {
        match self {
            Resolved::Value(v) => Ok(v),
            Resolved::Conflict => Err(err()),
        }
    }
}

trait ModeView {
    type ComputeValue: Default + Into<Self::Value>;
    type Value: PartialEq + Eq + Copy + Clone;
    type TwinView: ModeView<Value = Self::Value>;

    fn get_register(bb: &BasicBlockControlState) -> RegisterState<Self::Value>;
    fn set_register(bb: &mut BasicBlockControlState, reg: RegisterState<Self::Value>);
    fn new_mode(t: Self::Value, other: Self::Value) -> ModeRegister;
    fn set_single_mode(reg: &mut ModeRegister, x: Self::Value) -> Result<(), TranslateError>;
}

struct DenormalF32View;

impl ModeView for DenormalF32View {
    type ComputeValue = DenormalMode;
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
    type ComputeValue = DenormalMode;
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

struct RoundingF32View;

impl ModeView for RoundingF32View {
    type ComputeValue = RoundingMode;
    type Value = ast::RoundingMode;
    type TwinView = RoundingF16F64View;

    fn get_register(bb: &BasicBlockControlState) -> RegisterState<Self::Value> {
        bb.rounding_f32
    }

    fn set_register(bb: &mut BasicBlockControlState, reg: RegisterState<Self::Value>) {
        bb.rounding_f32 = reg;
    }

    fn new_mode(f32: Self::Value, f16f64: Self::Value) -> ModeRegister {
        ModeRegister::Rounding { f32, f16f64 }
    }

    fn set_single_mode(reg: &mut ModeRegister, x: Self::Value) -> Result<(), TranslateError> {
        match reg {
            ModeRegister::Rounding { f32, f16f64: _ } => *f32 = x,
            ModeRegister::Denormal { .. } => return Err(error_unreachable()),
        }
        Ok(())
    }
}

struct RoundingF16F64View;

impl ModeView for RoundingF16F64View {
    type ComputeValue = RoundingMode;
    type Value = ast::RoundingMode;
    type TwinView = RoundingF32View;

    fn get_register(bb: &BasicBlockControlState) -> RegisterState<Self::Value> {
        bb.rounding_f16f64
    }

    fn set_register(bb: &mut BasicBlockControlState, reg: RegisterState<Self::Value>) {
        bb.rounding_f16f64 = reg;
    }

    fn new_mode(f16f64: Self::Value, f32: Self::Value) -> ModeRegister {
        ModeRegister::Rounding { f32, f16f64 }
    }

    fn set_single_mode(reg: &mut ModeRegister, x: Self::Value) -> Result<(), TranslateError> {
        match reg {
            ModeRegister::Rounding { f32: _, f16f64 } => *f16f64 = x,
            ModeRegister::Denormal { .. } => return Err(error_unreachable()),
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

fn compute_single_mode_insertions<T: Copy + Eq>(
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
fn optimize_mode_insertions<
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
        // abs and neg have special handling in AMD GPU ISA. They get compiled
        // down to instruction argument modifiers, floating point flags have no
        // effect on it. We handle it during LLVM bitcode emission
        | ast::Instruction::Abs { .. }
        | ast::Instruction::Neg {.. }
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
        | ast::Instruction::ShflSync { .. }
        | ast::Instruction::CpAsync { .. }
        | ast::Instruction::CpAsyncCommitGroup { .. }
        | ast::Instruction::CpAsyncWaitGroup { .. }
        | ast::Instruction::CpAsyncWaitAll { .. }
        | ast::Instruction::Shf { .. }
        | ast::Instruction::Shl { .. }
        | ast::Instruction::Selp { .. }
        | ast::Instruction::Ret { .. }
        | ast::Instruction::Bar { .. }
        | ast::Instruction::BarRed { .. }
        | ast::Instruction::Cvta { .. }
        | ast::Instruction::Atom { .. }
        | ast::Instruction::Mul24 { .. }
        | ast::Instruction::Nanosleep { .. }
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
        ast::Instruction::Set {
            data: ast::SetData{
                base: ast::SetpData {
                    type_,
                    flush_to_zero,
                    ..
                },
                ..
            },
            ..
        }
        | ast::Instruction::SetBool {
            data: ast::SetBoolData {
                base: ast::SetpBoolData {
                    base: ast::SetpData {
                        type_,
                        flush_to_zero,
                            ..
                        },
                    ..
                },
            ..
            }, ..
        }
        | ast::Instruction::Setp {
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
        } => InstructionModes::from_ftz(*type_, *flush_to_zero),
        ast::Instruction::Div {
            data:
                ast::DivDetails::Float(ast::DivFloatDetails {
                    type_,
                    flush_to_zero,
                    kind,
                }),
            ..
        } => {
            let rounding = match kind {
                ast::DivFloatKind::Rounding(rnd) => RoundingMode::from_ast(*rnd),
                ast::DivFloatKind::Approx => RoundingMode::NearestEven,
                ast::DivFloatKind::ApproxFull => RoundingMode::NearestEven,
            };
            InstructionModes::new(
                *type_,
                flush_to_zero.map(DenormalMode::from_ftz),
                Some(rounding),
            )
        }
        ast::Instruction::Sin { data, .. }
        | ast::Instruction::Cos { data, .. } => InstructionModes::from_ftz_f32(data.flush_to_zero),
        ast::Instruction::Rcp { data, .. } | ast::Instruction::Sqrt { data, .. } => {
            InstructionModes::from_rtz_special(*data)
        }
        ast::Instruction::Rsqrt { data, .. }
        | ast::Instruction::Ex2 { data, .. } => {
            let data = ast::RcpData {
                type_: data.type_,
                flush_to_zero: data.flush_to_zero,
                kind: ast::RcpKind::Approx,
            };
            InstructionModes::from_rtz_special(data)
        },
        ast::Instruction::Lg2 { data, .. } => {
            let data = ast::RcpData {
                type_: ast::ScalarType::F32,
                flush_to_zero: Some(data.flush_to_zero),
                kind: ast::RcpKind::Approx,
            };
            InstructionModes::from_rtz_special(data)
        },
        ast::Instruction::Cvt { data, .. } => InstructionModes::from_cvt(data),
        ast::Instruction::Tanh { data, .. } => InstructionModes::from_ftz(*data, Some(false)),
    }
}

#[cfg(test)]
mod test;
