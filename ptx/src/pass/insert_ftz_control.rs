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
use std::hash::Hash;
use std::iter;
use std::mem;
use std::u32;
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

    fn to_ftz(self) -> bool {
        match self {
            DenormalMode::FlushToZero => true,
            DenormalMode::Preserve => false,
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
    mut directives: Vec<super::Directive2<ast::Instruction<SpirvWord>, super::SpirvWord>>,
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
                // TODO: implement for non-kernels
                if !*is_kernel {
                    todo!()
                }
                let entry_index = cfg.add_entry_basic_block(*name);
                let mut bb_state = BasicBlockState::new(&mut cfg);
                let mut body_iter = body.iter();
                match body_iter.next() {
                    Some(Statement::Label(label)) => {
                        bb_state.cfg.add_jump(entry_index, *label);
                        bb_state.start(*label);
                    }
                    _ => return Err(error_unreachable()),
                };
                for statement in body.iter() {
                    match statement {
                        Statement::Instruction(ast::Instruction::Bra { arguments }) => {
                            bb_state.end(&[arguments.src]);
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
    let denormal_f32 = compute_single_mode(&cfg, |node| node.denormal_f32);
    let denormal_f16f64 = compute_single_mode(&cfg, |node| node.denormal_f16f64);
    let rounding_f32 = compute_single_mode(&cfg, |node| node.rounding_f32);
    let rounding_f16f64 = compute_single_mode(&cfg, |node| node.rounding_f16f64);
    let denormal_f32 = optimize::<DenormalMode, { DenormalMode::COUNT }>(denormal_f32);
    let denormal_f16f64 = optimize::<DenormalMode, { DenormalMode::COUNT }>(denormal_f16f64);
    let rounding_f32 = optimize::<RoundingMode, { RoundingMode::COUNT }>(rounding_f32);
    let rounding_f16f64 = optimize::<RoundingMode, { RoundingMode::COUNT }>(rounding_f16f64);
    insert_mode_control(
        flat_resolver,
        &mut directives,
        &cfg,
        denormal_f32,
        denormal_f16f64,
        rounding_f32,
        rounding_f16f64,
    )?;
    Ok(directives)
}

fn insert_mode_control<'input>(
    flat_resolver: &mut super::GlobalStringIdentResolver2<'input>,
    directives: &mut [Directive2<ast::Instruction<SpirvWord>, SpirvWord>],
    cfg: &ControlFlowGraph,
    denormal_f32: ModeInsertions<DenormalMode>,
    denormal_f16f64: ModeInsertions<DenormalMode>,
    rounding_f32: ModeInsertions<RoundingMode>,
    rounding_f16f64: ModeInsertions<RoundingMode>,
) -> Result<(), TranslateError> {
    for directive in directives.iter_mut() {
        let body_ptr = match directive {
            Directive2::Variable(..) | Directive2::Method(Function2 { body: None, .. }) => continue,
            Directive2::Method(Function2 {
                name,
                body: Some(body),
                flush_to_zero_f32,
                flush_to_zero_f16f64,
                roundind_mode_f32: rounding_mode_f32,
                roundind_mode_f16f64: rounding_mode_f16f64,
                ..
            }) => {
                *flush_to_zero_f32 = denormal_f32
                    .kernels
                    .get(name)
                    .copied()
                    .unwrap_or(DenormalMode::default())
                    .to_ftz();
                *flush_to_zero_f16f64 = denormal_f16f64
                    .kernels
                    .get(name)
                    .copied()
                    .unwrap_or(DenormalMode::default())
                    .to_ftz();
                *rounding_mode_f32 = rounding_f32
                    .kernels
                    .get(name)
                    .copied()
                    .unwrap_or(RoundingMode::default())
                    .to_ast();
                *rounding_mode_f16f64 = rounding_f16f64
                    .kernels
                    .get(name)
                    .copied()
                    .unwrap_or(RoundingMode::default())
                    .to_ast();
                body
            }
        };
        let mut old_body = mem::replace(body_ptr, Vec::new());
        let mut result = Vec::with_capacity(old_body.len());
        let mut bb_state = BasicBlockControlState::new(
            &denormal_f32,
            &denormal_f16f64,
            &rounding_f32,
            &rounding_f16f64,
        );
        for statement in old_body.into_iter() {
            match &statement {
                Statement::Label(label) => {
                    bb_state.start(*label);
                }
                Statement::Instruction(instruction) => {
                    let modes = get_modes(&instruction);
                    bb_state.insert(&mut result, modes)?;
                }
                _ => {}
            }
            result.push(statement);
        }
        *body_ptr = result;
    }
    Ok(())
}

struct BasicBlockControlState<'a> {
    global_denormal_f32: &'a ModeInsertions<DenormalMode>,
    global_denormal_f16f64: &'a ModeInsertions<DenormalMode>,
    global_rounding_f32: &'a ModeInsertions<RoundingMode>,
    global_rounding_f16f64: &'a ModeInsertions<RoundingMode>,
    basic_block: SpirvWord,
    denormal_f32: RegisterState<bool>,
    denormal_f16f64: RegisterState<bool>,
    foldable_rounding_f32: Option<usize>,
    foldable_rounding_f16f64: Option<usize>,
}

#[derive(Clone, Copy)]
enum RegisterState<T> {
    Inherited,
    Unknown,
    Value(Option<usize>, T),
}

impl<T> RegisterState<T> {
    fn empty() -> Self {
        Self::Unknown
    }

    fn new(must_insert: bool) -> Self {
        if must_insert {
            Self::Unknown
        } else {
            Self::Inherited
        }
    }
}

impl<'a> BasicBlockControlState<'a> {
    fn new(
        global_denormal_f32: &'a ModeInsertions<DenormalMode>,
        global_denormal_f16f64: &'a ModeInsertions<DenormalMode>,
        global_rounding_f32: &'a ModeInsertions<RoundingMode>,
        global_rounding_f16f64: &'a ModeInsertions<RoundingMode>,
    ) -> Self {
        BasicBlockControlState {
            global_denormal_f32,
            global_denormal_f16f64,
            global_rounding_f32,
            global_rounding_f16f64,
            basic_block: SpirvWord(u32::MAX),
            denormal_f32: RegisterState::empty(),
            denormal_f16f64: RegisterState::empty(),
            foldable_rounding_f32: None,
            foldable_rounding_f16f64: None,
        }
    }

    fn start(&mut self, label: SpirvWord) {
        self.denormal_f32 =
            RegisterState::new(self.global_denormal_f32.basic_blocks.contains(&label));
        self.denormal_f32 =
            RegisterState::new(self.global_denormal_f16f64.basic_blocks.contains(&label));
        self.foldable_rounding_f32 = None;
        self.foldable_rounding_f16f64 = None;
        self.basic_block = label;
    }

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
        if let Some(new_mode) = mode {
            let register_state = View::get_register(self);
            match register_state {
                RegisterState::Inherited => {
                    View::set_register(self, RegisterState::Value(None, new_mode));
                }
                RegisterState::Unknown => {
                    View::set_register(
                        self,
                        RegisterState::Value(
                            Some(self.add_or_fold_mode_set2::<View>(result, new_mode)),
                            new_mode,
                        ),
                    );
                }
                RegisterState::Value(_, old_value) => {
                    if new_mode == old_value {
                        return Ok(());
                    }
                    View::set_register(
                        self,
                        RegisterState::Value(
                            Some(self.add_or_fold_mode_set2::<View>(result, new_mode)),
                            new_mode,
                        ),
                    );
                }
            }
        }
        Ok(())
    }

    // Return the index of the last insertion of SetMode with this mode
    fn add_or_fold_mode_set2<View: ModeView>(
        &self,
        result: &mut Vec<Statement<ast::Instruction<SpirvWord>, SpirvWord>>,
        new_mode: View::Value,
    ) -> usize {
        // try and fold into the other mode set in struction
        if let RegisterState::Value(Some(twin_index), _) = View::TwinView::get_register(self) {
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
}

trait ModeView {
    type Value: PartialEq + Eq + Copy + Clone;
    type TwinView: ModeView<Value = Self::Value>;

    fn get_register(bb: &BasicBlockControlState) -> RegisterState<Self::Value>;
    fn set_register(bb: &mut BasicBlockControlState, reg: RegisterState<Self::Value>);
    fn new_mode(t: Self::Value, other: Option<Self::Value>) -> ModeRegister;
    fn get_single_mode(reg: &ModeRegister) -> Option<Self::Value>;
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

    fn new_mode(f32: Self::Value, f16f64: Option<Self::Value>) -> ModeRegister {
        match f16f64 {
            Some(f16f64) => ModeRegister::DenormalBoth { f32, f16f64 },
            None => ModeRegister::DenormalF32(f32),
        }
    }

    fn get_single_mode(reg: &ModeRegister) -> Option<Self::Value> {
        match reg {
            ModeRegister::DenormalF32(value) => Some(*value),
            _ => None,
        }
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

    fn new_mode(f16f64: Self::Value, f32: Option<Self::Value>) -> ModeRegister {
        match f32 {
            Some(f32) => ModeRegister::DenormalBoth { f16f64, f32 },
            None => ModeRegister::DenormalF16F64(f16f64),
        }
    }

    fn get_single_mode(reg: &ModeRegister) -> Option<Self::Value> {
        match reg {
            ModeRegister::DenormalF16F64(value) => Some(*value),
            _ => None,
        }
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
    fn new(cfg: &'a mut ControlFlowGraph) -> BasicBlockState<'a> {
        Self {
            cfg,
            node_index: None,
            entry: InstructionModes::none(),
            exit: InstructionModes::none(),
        }
    }

    fn start(&mut self, label: SpirvWord) {
        self.end(&[]);
        self.node_index = Some(self.cfg.get_or_add_basic_block(label));
    }

    fn end(&mut self, jumps: &[SpirvWord]) {
        let node_index = self.node_index.take();
        let node_index = match node_index {
            Some(x) => x,
            None => return,
        };
        for target in jumps {
            self.cfg.add_jump(node_index, *target);
        }
        self.cfg.set_modes(
            node_index,
            mem::replace(&mut self.entry, InstructionModes::none()),
            mem::replace(&mut self.exit, InstructionModes::none()),
        );
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
    'iterate_kernels: for (kernel, modes) in kernel_modes {
        for (mode, var) in modes.into_iter().enumerate() {
            if solution[var] > 0.5 {
                kernels.insert(kernel, T::VARIANTS[mode]);
                continue 'iterate_kernels;
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
