use crate::ast;
use bit_vec::BitVec;
use rspirv::dr;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum SpirvType {
    Base(ast::ScalarType),
}

struct TypeWordMap {
    void: spirv::Word,
    fn_void: spirv::Word,
    complex: HashMap<SpirvType, spirv::Word>,
}

impl TypeWordMap {
    fn new(b: &mut dr::Builder) -> TypeWordMap {
        let void = b.type_void();
        TypeWordMap {
            void: void,
            fn_void: b.type_function(void, vec![]),
            complex: HashMap::<SpirvType, spirv::Word>::new(),
        }
    }

    fn void(&self) -> spirv::Word {
        self.void
    }
    fn fn_void(&self) -> spirv::Word {
        self.fn_void
    }

    fn get_or_add(&mut self, b: &mut dr::Builder, t: SpirvType) -> spirv::Word {
        *self.complex.entry(t).or_insert_with(|| match t {
            SpirvType::Base(ast::ScalarType::B8) | SpirvType::Base(ast::ScalarType::U8) => {
                b.type_int(8, 0)
            }
            SpirvType::Base(ast::ScalarType::B16) | SpirvType::Base(ast::ScalarType::U16) => {
                b.type_int(16, 0)
            }
            SpirvType::Base(ast::ScalarType::B32) | SpirvType::Base(ast::ScalarType::U32) => {
                b.type_int(32, 0)
            }
            SpirvType::Base(ast::ScalarType::B64) | SpirvType::Base(ast::ScalarType::U64) => {
                b.type_int(64, 0)
            }
            SpirvType::Base(ast::ScalarType::S8) => b.type_int(8, 1),
            SpirvType::Base(ast::ScalarType::S16) => b.type_int(16, 1),
            SpirvType::Base(ast::ScalarType::S32) => b.type_int(32, 1),
            SpirvType::Base(ast::ScalarType::S64) => b.type_int(64, 1),
            SpirvType::Base(ast::ScalarType::F16) => b.type_float(16),
            SpirvType::Base(ast::ScalarType::F32) => b.type_float(32),
            SpirvType::Base(ast::ScalarType::F64) => b.type_float(64),
        })
    }
}

pub fn to_spirv(ast: ast::Module) -> Result<Vec<u32>, rspirv::dr::Error> {
    let mut builder = dr::Builder::new();
    // https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_logicallayout_a_logical_layout_of_a_module
    builder.set_version(1, 0);
    emit_capabilities(&mut builder);
    emit_extensions(&mut builder);
    emit_extended_instruction_sets(&mut builder);
    emit_memory_model(&mut builder);
    let mut map = TypeWordMap::new(&mut builder);
    for f in ast.functions {
        emit_function(&mut builder, &mut map, f)?;
    }
    Ok(vec![])
}

fn emit_capabilities(builder: &mut dr::Builder) {
    builder.capability(spirv::Capability::Linkage);
    builder.capability(spirv::Capability::Addresses);
    builder.capability(spirv::Capability::Kernel);
    builder.capability(spirv::Capability::Int64);
    builder.capability(spirv::Capability::Int8);
}

fn emit_extensions(_: &mut dr::Builder) {}

fn emit_extended_instruction_sets(builder: &mut dr::Builder) {
    builder.ext_inst_import("OpenCL.std");
}

fn emit_memory_model(builder: &mut dr::Builder) {
    builder.memory_model(
        spirv::AddressingModel::Physical64,
        spirv::MemoryModel::OpenCL,
    );
}

fn emit_function<'a>(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    f: ast::Function<'a>,
) -> Result<spirv::Word, rspirv::dr::Error> {
    let func_id = builder.begin_function(
        map.void(),
        None,
        spirv::FunctionControl::NONE,
        map.fn_void(),
    )?;
    for arg in f.args.iter() {
        let arg_type = map.get_or_add(builder, SpirvType::Base(arg.a_type));
        builder.function_parameter(arg_type)?;
    }
    let (mut normalized_ids, max_id) = normalize_identifiers(f.body);
    let bbs = get_basic_blocks(&normalized_ids);
    let rpostorder = to_reverse_postorder(&bbs);
    let doms = immediate_dominators(&bbs, &rpostorder);
    let dom_fronts = dominance_frontiers(&bbs, &rpostorder, &doms);
    ssa_legalize(&mut normalized_ids, max_id, bbs, &doms, &dom_fronts);
    emit_function_body_ops(builder);
    builder.ret()?;
    builder.end_function()?;
    Ok(func_id)
}

fn emit_function_body_ops(builder: &mut dr::Builder) {
    todo!()
}

// TODO: support scopes
fn normalize_identifiers<'a>(func: Vec<ast::Statement<&'a str>>) -> (Vec<Statement>, spirv::Word) {
    let mut result = Vec::with_capacity(func.len());
    let mut id: u32 = 0;
    let mut known_ids = HashMap::new();
    let mut get_or_add = |key| {
        *known_ids.entry(key).or_insert_with(|| {
            let to_insert = id;
            id += 1;
            to_insert
        })
    };
    for s in func {
        if let Some(s) = Statement::from_ast(s, &mut get_or_add) {
            result.push(s);
        }
    }
    (result, id - 1)
}

fn ssa_legalize(
    func: &mut [Statement],
    max_id: spirv::Word,
    bbs: Vec<BasicBlock>,
    doms: &Vec<BBIndex>,
    dom_fronts: &Vec<HashSet<BBIndex>>,
) {
    let phis = gather_phi_sets(&func, max_id, &bbs, dom_fronts);
    apply_ssa_renaming(func, &bbs, doms, max_id, &phis);
}

// "Modern Compiler Implementation in Java" - Algorithm 19.7
fn apply_ssa_renaming(
    func: &mut [Statement],
    bbs: &[BasicBlock],
    doms: &[BBIndex],
    max_id: spirv::Word,
    old_phi: &[HashSet<spirv::Word>],
) {
    let mut dom_tree = vec![Vec::new(); bbs.len()];
    for (bb, idom) in doms.iter().enumerate() {
        dom_tree[idom.0].push(BBIndex(bb));
    }
    let mut old_dst_id = vec![Vec::new(); bbs.len()];
    for bb in 0..bbs.len() {
        for s in get_bb_body(func, bbs, BBIndex(bb)) {
            s.for_dst_id(&mut |id| old_dst_id[bb].push(id));
        }
    }
    let mut new_phi = old_phi
        .iter()
        .map(|ids| {
            ids.iter()
                .map(|id| (*id, Vec::new()))
                .collect::<HashMap<_, _>>()
        })
        .collect::<Vec<_>>();
    let mut ssa_state = SSARewriteState::new(max_id);
    // once again, we do explicit stack
    let mut state = Vec::new();
    state.push((BBIndex(0), 0));
    loop {
        if let Some((BBIndex(bb), dom_succ_idx)) = state.last_mut() {
            let bb = *bb;
            if *dom_succ_idx == 0 {
                rename_phi_dst(max_id, &mut ssa_state, &mut new_phi[bb]);
                rename_bb_body(&mut ssa_state, func, bbs, BBIndex(bb));
                for BBIndex(succ_idx) in bbs[bb].succ.iter() {
                    rename_succesor_phi_src(&ssa_state, &mut new_phi[*succ_idx]);
                }
            }
            if let Some(s) = dom_tree[bb].get(*dom_succ_idx) {
                *dom_succ_idx += 1;
                state.push((*s, 0));
            } else {
                state.pop();
                pop_stacks(&mut ssa_state, &old_phi[bb], &old_dst_id[bb]);
            }
        } else {
            break;
        }
    }
}

fn rename_phi_dst(
    max_old_id: spirv::Word,
    rewriter: &mut SSARewriteState,
    phi: &mut HashMap<spirv::Word, Vec<spirv::Word>>,
) {
    let old_keys = phi
        .keys()
        .copied()
        .filter(|id| *id <= max_old_id)
        .collect::<Vec<_>>();
    for k in old_keys.into_iter() {
        let remapped_id = rewriter.redefine(k);
        let values = phi.remove(&k).unwrap();
        phi.insert(remapped_id, values);
    }
}

fn rename_bb_body(
    ssa_state: &mut SSARewriteState,
    func: &mut [Statement],
    all_bb: &[BasicBlock],
    bb: BBIndex,
) {
    for s in get_bb_body_mut(func, all_bb, bb) {
        s.visit_id_mut(&mut |is_dst, id| {
            if is_dst {
                *id = ssa_state.redefine(*id);
            } else {
                *id = ssa_state.get(*id);
            }
        });
    }
}

fn rename_succesor_phi_src(
    ssa_state: &SSARewriteState,
    phi: &mut HashMap<spirv::Word, Vec<spirv::Word>>,
) {
    for (id, v) in phi.iter_mut() {
        v.push(ssa_state.get(*id));
    }
}

fn pop_stacks(
    ssa_state: &mut SSARewriteState,
    old_phi: &HashSet<spirv::Word>,
    old_ids: &[spirv::Word],
) {
    for id in old_phi.iter().chain(old_ids) {
        ssa_state.pop(*id);
    }
}

fn get_bb_body_mut<'a>(
    func: &'a mut [Statement],
    all_bb: &[BasicBlock],
    bb: BBIndex,
) -> &'a mut [Statement] {
    let (start, end) = get_bb_body_idx(all_bb, bb);
    &mut func[start..end]
}

fn get_bb_body<'a>(func: &'a [Statement], all_bb: &[BasicBlock], bb: BBIndex) -> &'a [Statement] {
    let (start, end) = get_bb_body_idx(all_bb, bb);
    &func[start..end]
}

fn get_bb_body_idx(all_bb: &[BasicBlock], bb: BBIndex) -> (usize, usize) {
    let BBIndex(bb_idx) = bb;
    let start = all_bb[bb_idx].start.0;
    let end = if bb_idx == all_bb.len() - 1 {
        all_bb.len()
    } else {
        all_bb[bb_idx + 1].start.0
    };
    (start, end)
}

// We assume here that the variables are defined in the dense sequence 0..max
struct SSARewriteState {
    next: spirv::Word,
    stack: Vec<Vec<spirv::Word>>,
}

impl SSARewriteState {
    fn new(max: spirv::Word) -> Self {
        let stack = vec![Vec::new(); max as usize];
        SSARewriteState {
            next: max + 1,
            stack,
        }
    }

    fn get(&self, x: spirv::Word) -> spirv::Word {
        *self.stack[x as usize].last().unwrap()
    }

    fn redefine(&mut self, x: spirv::Word) -> spirv::Word {
        let result = self.next;
        self.next += 1;
        self.stack[x as usize].push(result);
        return result;
    }

    fn pop(&mut self, x: spirv::Word) {
        self.stack[x as usize].pop();
    }
}

// "Modern Compiler Implementation in Java" - Algorithm 19.6
fn gather_phi_sets(
    func: &[Statement],
    max_id: spirv::Word,
    bbs: &[BasicBlock],
    dom_fronts: &[HashSet<BBIndex>],
) -> Vec<HashSet<spirv::Word>> {
    let mut result = vec![HashSet::new(); bbs.len()];
    let mut bb_dst_definitions = vec![HashSet::new(); bbs.len()];
    let mut def_sites = vec![(HashSet::new(), Vec::new()); (max_id as usize) + 1];
    for bb in 0..bbs.len() {
        let bb = BBIndex(bb);
        for s in get_bb_body(func, bbs, bb) {
            s.for_dst_id(&mut |id| {
                bb_dst_definitions[bb.0].insert(id);
                let (ref mut set, ref mut stack) = def_sites[id as usize];
                if set.insert(bb) {
                    stack.push(bb);
                }
            });
        }
    }
    for (id, to_work) in def_sites.iter_mut().enumerate() {
        let id = id as spirv::Word;
        let (ref mut set, ref mut stack) = to_work;
        loop {
            if let Some(bb) = stack.pop() {
                set.remove(&bb);
                for y_bb in &dom_fronts[bb.0] {
                    if result[y_bb.0].insert(id) {
                        if !bb_dst_definitions[y_bb.0].contains(&id) {
                            if set.insert(*y_bb) {
                                stack.push(*y_bb);
                            }
                        }
                    }
                }
            } else {
                break;
            }
        }
    }
    result
}

fn get_basic_blocks(fun: &[Statement]) -> Vec<BasicBlock> {
    // edge signify pred/succ relationship between bbs
    let mut bb_edge = HashSet::new();
    let mut unresolved_bb_edge = Vec::new();
    // bb start means that a bb is starting at this statement, but there's no predecessor
    let mut bb_start = Vec::new();
    let mut labels = HashMap::new();
    for (idx, s) in fun.iter().enumerate() {
        match s {
            Statement::Instruction(pred, i) => {
                if let Some(id) = i.jump_target() {
                    unresolved_bb_edge.push((StmtIndex(idx), id));
                    if idx + 1 < fun.len() {
                        if pred.is_some() {
                            bb_edge.insert((StmtIndex(idx), StmtIndex(idx + 1)));
                        }
                        bb_start.push(StmtIndex(idx + 1));
                    }
                } else if i.is_terminal() && idx + 1 < fun.len() {
                    bb_start.push(StmtIndex(idx + 1));
                }
            }
            Statement::Label(id) => {
                labels.insert(id, StmtIndex(idx));
            }
        };
    }
    // Resolve every <jump into label> into <jump into statement index>
    // TODO: handle jumps into nowhere
    for (idx, id) in unresolved_bb_edge {
        let target = labels[&id];
        bb_edge.insert((idx, target));
        bb_start.push(target);
        // now check if the preceding statement forms an edge
        if target != StmtIndex(0) {
            match &fun[target.0 - 1] {
                Statement::Instruction(pred, i) => {
                    if !((pred.is_none() && i.jump_target().is_some()) || i.is_terminal()) {
                        bb_edge.insert((StmtIndex(target.0 - 1), target));
                    }
                }
                Statement::Label(_) => (),
            }
        }
    }
    // Create list of bbs without succ/pred
    let mut bbs_map = BTreeMap::new();
    bbs_map.insert(
        StmtIndex(0),
        BasicBlock {
            start: StmtIndex(0),
            pred: Vec::new(),
            succ: Vec::new(),
        },
    );
    for bb_first_stmt in bb_start {
        bbs_map.entry(bb_first_stmt).or_insert_with(|| BasicBlock {
            start: bb_first_stmt,
            pred: Vec::new(),
            succ: Vec::new(),
        });
    }
    // Populate succ/pred
    let indexed_bbs_map = bbs_map
        .into_iter()
        .enumerate()
        .map(|(idx, (key, val))| (key, (BBIndex(idx), RefCell::new(val))))
        .collect::<BTreeMap<_, _>>();
    for (from, to) in bb_edge {
        let (_, (from_idx, from_ref)) = indexed_bbs_map.range(..=from).next_back().unwrap();
        let (to_idx, to_ref) = indexed_bbs_map.get(&to).unwrap();
        {
            from_ref.borrow_mut().succ.push(*to_idx);
        }
        {
            to_ref.borrow_mut().pred.push(*from_idx);
        }
    }
    indexed_bbs_map
        .into_iter()
        .map(|(_, (_, bb))| bb.into_inner())
        .collect::<Vec<_>>()
}

// "A Simple, Fast Dominance Algorithm" - Keith D. Cooper, Timothy J. Harvey, and Ken Kennedy
// https://www.cs.rice.edu/~keith/EMBED/dom.pdf
fn dominance_frontiers(
    bbs: &Vec<BasicBlock>,
    order: &Vec<BBIndex>,
    doms: &Vec<BBIndex>,
) -> Vec<HashSet<BBIndex>> {
    let mut result = vec![HashSet::new(); bbs.len()];
    for (bb_idx, b) in bbs.iter().enumerate() {
        if b.pred.len() < 2 {
            continue;
        }
        for p in b.pred.iter() {
            let mut runner = *p;
            while runner != doms[bb_idx] {
                result[runner.0].insert(BBIndex(bb_idx));
                runner = doms[runner.0];
            }
        }
    }
    result
}

fn immediate_dominators(bbs: &Vec<BasicBlock>, order: &Vec<BBIndex>) -> Vec<BBIndex> {
    let mut doms = vec![BBIndex(usize::max_value()); bbs.len()];
    doms[0] = BBIndex(0);
    let mut changed = true;
    while changed {
        changed = false;
        for BBIndex(bb_idx) in order.iter().skip(1) {
            let bb = &bbs[*bb_idx];
            if let Some(first_pred) = bb.pred.get(0) {
                let mut new_idom = *first_pred;
                for BBIndex(p_idx) in bb.pred.iter().copied().skip(1) {
                    if doms[p_idx] != BBIndex(usize::max_value()) {
                        new_idom = intersect(&mut doms, BBIndex(p_idx), new_idom);
                    }
                }
                if doms[*bb_idx] != new_idom {
                    doms[*bb_idx] = new_idom;
                    changed = true;
                }
            }
        }
    }
    return doms;
}

// Original paper uses reverse indexing: their entry node has index n,
// that's why the compares are reversed
fn intersect(doms: &mut Vec<BBIndex>, b1: BBIndex, b2: BBIndex) -> BBIndex {
    let mut finger1 = b1;
    let mut finger2 = b2;
    while finger1 != finger2 {
        while finger1 > finger2 {
            finger1 = doms[finger1.0];
        }
        while finger2 > finger1 {
            finger2 = doms[finger2.0];
        }
    }
    finger1
}

// "A Simple Algorithm for Global Data Flow Analysis Problems" - Hecht, M. S., & Ullman, J. D. (1975)
fn to_reverse_postorder(input: &Vec<BasicBlock>) -> Vec<BBIndex> {
    let mut i = input.len();
    let mut old = BitVec::from_elem(input.len(), false);
    let mut result = vec![BBIndex(usize::max_value()); input.len()];
    // original uses recursion and implicit stack, we do it explictly
    let mut state = Vec::new();
    state.push((BBIndex(0), 0usize));
    loop {
        if let Some((BBIndex(bb), succ_iter_idx)) = state.last_mut() {
            let bb = *bb;
            if *succ_iter_idx == 0 {
                old.set(bb, true);
            }
            if let Some(BBIndex(succ)) = &input[bb].succ.get(*succ_iter_idx) {
                *succ_iter_idx += 1;
                if !old.get(*succ).unwrap() {
                    state.push((BBIndex(*succ), 0));
                }
            } else {
                state.pop();
                i = i - 1;
                result[i] = BBIndex(bb);
            }
        } else {
            break;
        }
    }
    result
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct BasicBlock {
    start: StmtIndex,
    pred: Vec<BBIndex>,
    succ: Vec<BBIndex>,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, PartialOrd, Ord, Hash)]
struct StmtIndex(pub usize);
#[derive(Eq, PartialEq, Debug, Copy, Clone, PartialOrd, Ord, Hash)]
struct BBIndex(pub usize);

enum Statement {
    Label(u32),
    Instruction(
        Option<ast::PredAt<spirv::Word>>,
        ast::Instruction<spirv::Word>,
    ),
}

impl Statement {
    fn from_ast<'a, F: FnMut(&'a str) -> u32>(
        s: ast::Statement<&'a str>,
        f: &mut F,
    ) -> Option<Self> {
        match s {
            ast::Statement::Label(name) => Some(Statement::Label(f(name))),
            ast::Statement::Instruction(p, i) => {
                Some(Statement::Instruction(p.map(|p| p.map_id(f)), i.map_id(f)))
            }
            ast::Statement::Variable(_) => None,
        }
    }

    fn for_dst_id<F: FnMut(spirv::Word)>(&self, f: &mut F) {
        match self {
            Statement::Label(id) => f(*id),
            Statement::Instruction(pred, inst) => {
                pred.as_ref().map(|p| p.for_dst_id(f));
                inst.for_dst_id(f);
            }
        }
    }

    fn visit_id_mut<F: FnMut(bool, &mut spirv::Word)>(&mut self, f: &mut F) {
        match self {
            Statement::Label(id) => f(true, id),
            Statement::Instruction(pred, inst) => {
                pred.as_mut().map(|p| p.visit_id_mut(f));
                inst.visit_id_mut(f);
            }
        }
    }
}

impl<T> ast::PredAt<T> {
    fn map_id<U, F: FnMut(T) -> U>(self, f: &mut F) -> ast::PredAt<U> {
        ast::PredAt {
            not: self.not,
            label: f(self.label),
        }
    }

    fn visit_id_mut<F: FnMut(bool, &mut T)>(&mut self, f: &mut F) {
        f(false, &mut self.label)
    }
}

impl<T: Copy> ast::PredAt<T> {
    fn for_dst_id<F: FnMut(T)>(&self, _: &mut F) {}
}

impl<T> ast::Instruction<T> {
    fn map_id<U, F: FnMut(T) -> U>(self, f: &mut F) -> ast::Instruction<U> {
        match self {
            ast::Instruction::Ld(d, a) => ast::Instruction::Ld(d, a.map_id(f)),
            ast::Instruction::Mov(d, a) => ast::Instruction::Mov(d, a.map_id(f)),
            ast::Instruction::Mul(d, a) => ast::Instruction::Mul(d, a.map_id(f)),
            ast::Instruction::Add(d, a) => ast::Instruction::Add(d, a.map_id(f)),
            ast::Instruction::Setp(d, a) => ast::Instruction::Setp(d, a.map_id(f)),
            ast::Instruction::SetpBool(d, a) => ast::Instruction::SetpBool(d, a.map_id(f)),
            ast::Instruction::Not(d, a) => ast::Instruction::Not(d, a.map_id(f)),
            ast::Instruction::Bra(d, a) => ast::Instruction::Bra(d, a.map_id(f)),
            ast::Instruction::Cvt(d, a) => ast::Instruction::Cvt(d, a.map_id(f)),
            ast::Instruction::Shl(d, a) => ast::Instruction::Shl(d, a.map_id(f)),
            ast::Instruction::St(d, a) => ast::Instruction::St(d, a.map_id(f)),
            ast::Instruction::Ret(d) => ast::Instruction::Ret(d),
        }
    }

    fn visit_id_mut<F: FnMut(bool, &mut T)>(&mut self, f: &mut F) {
        match self {
            ast::Instruction::Ld(_, a) => a.visit_id_mut(f),
            ast::Instruction::Mov(_, a) => a.visit_id_mut(f),
            ast::Instruction::Mul(_, a) => a.visit_id_mut(f),
            ast::Instruction::Add(_, a) => a.visit_id_mut(f),
            ast::Instruction::Setp(_, a) => a.visit_id_mut(f),
            ast::Instruction::SetpBool(_, a) => a.visit_id_mut(f),
            ast::Instruction::Not(_, a) => a.visit_id_mut(f),
            ast::Instruction::Cvt(_, a) => a.visit_id_mut(f),
            ast::Instruction::Shl(_, a) => a.visit_id_mut(f),
            ast::Instruction::St(_, a) => a.visit_id_mut(f),
            ast::Instruction::Bra(_, a) => a.visit_id_mut(f),
            ast::Instruction::Ret(_) => (),
        }
    }
}

impl<T: Copy> ast::Instruction<T> {
    fn jump_target(&self) -> Option<T> {
        match self {
            ast::Instruction::Bra(_, a) => Some(a.src),
            ast::Instruction::Ld(_, _)
            | ast::Instruction::Mov(_, _)
            | ast::Instruction::Mul(_, _)
            | ast::Instruction::Add(_, _)
            | ast::Instruction::Setp(_, _)
            | ast::Instruction::SetpBool(_, _)
            | ast::Instruction::Not(_, _)
            | ast::Instruction::Cvt(_, _)
            | ast::Instruction::Shl(_, _)
            | ast::Instruction::St(_, _)
            | ast::Instruction::Ret(_) => None,
        }
    }

    fn is_terminal(&self) -> bool {
        match self {
            ast::Instruction::Ret(_) => true,
            ast::Instruction::Ld(_, _)
            | ast::Instruction::Mov(_, _)
            | ast::Instruction::Mul(_, _)
            | ast::Instruction::Add(_, _)
            | ast::Instruction::Setp(_, _)
            | ast::Instruction::SetpBool(_, _)
            | ast::Instruction::Not(_, _)
            | ast::Instruction::Cvt(_, _)
            | ast::Instruction::Shl(_, _)
            | ast::Instruction::St(_, _)
            | ast::Instruction::Bra(_, _) => false,
        }
    }

    fn for_dst_id<F: FnMut(T)>(&self, f: &mut F) {
        match self {
            ast::Instruction::Ld(_, a) => a.for_dst_id(f),
            ast::Instruction::Mov(_, a) => a.for_dst_id(f),
            ast::Instruction::Mul(_, a) => a.for_dst_id(f),
            ast::Instruction::Add(_, a) => a.for_dst_id(f),
            ast::Instruction::Setp(_, a) => a.for_dst_id(f),
            ast::Instruction::SetpBool(_, a) => a.for_dst_id(f),
            ast::Instruction::Not(_, a) => a.for_dst_id(f),
            ast::Instruction::Cvt(_, a) => a.for_dst_id(f),
            ast::Instruction::Shl(_, a) => a.for_dst_id(f),
            ast::Instruction::St(_, a) => a.for_dst_id(f),
            ast::Instruction::Bra(_, _) => (),
            ast::Instruction::Ret(_) => (),
        }
    }
}

impl<T> ast::Arg1<T> {
    fn map_id<U, F: FnMut(T) -> U>(self, f: &mut F) -> ast::Arg1<U> {
        ast::Arg1 { src: f(self.src) }
    }

    fn visit_id_mut<F: FnMut(bool, &mut T)>(&mut self, f: &mut F) {
        f(false, &mut self.src);
    }
}

impl<T> ast::Arg2<T> {
    fn map_id<U, F: FnMut(T) -> U>(self, f: &mut F) -> ast::Arg2<U> {
        ast::Arg2 {
            dst: f(self.dst),
            src: self.src.map_id(f),
        }
    }

    fn visit_id_mut<F: FnMut(bool, &mut T)>(&mut self, f: &mut F) {
        f(true, &mut self.dst);
        self.src.visit_id_mut(f);
    }
}

impl<T: Copy> ast::Arg2<T> {
    fn for_dst_id<F: FnMut(T)>(&self, f: &mut F) {
        f(self.dst);
    }
}

impl<T> ast::Arg2Mov<T> {
    fn map_id<U, F: FnMut(T) -> U>(self, f: &mut F) -> ast::Arg2Mov<U> {
        ast::Arg2Mov {
            dst: f(self.dst),
            src: self.src.map_id(f),
        }
    }

    fn visit_id_mut<F: FnMut(bool, &mut T)>(&mut self, f: &mut F) {
        f(true, &mut self.dst);
        self.src.visit_id_mut(f);
    }
}

impl<T: Copy> ast::Arg2Mov<T> {
    fn for_dst_id<F: FnMut(T)>(&self, f: &mut F) {
        f(self.dst);
    }
}

impl<T> ast::Arg3<T> {
    fn map_id<U, F: FnMut(T) -> U>(self, f: &mut F) -> ast::Arg3<U> {
        ast::Arg3 {
            dst: f(self.dst),
            src1: self.src1.map_id(f),
            src2: self.src2.map_id(f),
        }
    }

    fn visit_id_mut<F: FnMut(bool, &mut T)>(&mut self, f: &mut F) {
        f(true, &mut self.dst);
        self.src1.visit_id_mut(f);
        self.src2.visit_id_mut(f);
    }
}

impl<T: Copy> ast::Arg3<T> {
    fn for_dst_id<F: FnMut(T)>(&self, f: &mut F) {
        f(self.dst);
    }
}

impl<T> ast::Arg4<T> {
    fn map_id<U, F: FnMut(T) -> U>(self, f: &mut F) -> ast::Arg4<U> {
        ast::Arg4 {
            dst1: f(self.dst1),
            dst2: self.dst2.map(|i| f(i)),
            src1: self.src1.map_id(f),
            src2: self.src2.map_id(f),
        }
    }

    fn visit_id_mut<F: FnMut(bool, &mut T)>(&mut self, f: &mut F) {
        f(true, &mut self.dst1);
        self.dst2.as_mut().map(|i| f(true, i));
        self.src1.visit_id_mut(f);
        self.src2.visit_id_mut(f);
    }
}

impl<T: Copy> ast::Arg4<T> {
    fn for_dst_id<F: FnMut(T)>(&self, f: &mut F) {
        f(self.dst1);
        self.dst2.map(|t| f(t));
    }
}

impl<T> ast::Arg5<T> {
    fn map_id<U, F: FnMut(T) -> U>(self, f: &mut F) -> ast::Arg5<U> {
        ast::Arg5 {
            dst1: f(self.dst1),
            dst2: self.dst2.map(|i| f(i)),
            src1: self.src1.map_id(f),
            src2: self.src2.map_id(f),
            src3: self.src3.map_id(f),
        }
    }

    fn visit_id_mut<F: FnMut(bool, &mut T)>(&mut self, f: &mut F) {
        f(true, &mut self.dst1);
        self.dst2.as_mut().map(|i| f(true, i));
        self.src1.visit_id_mut(f);
        self.src2.visit_id_mut(f);
        self.src3.visit_id_mut(f);
    }
}

impl<T: Copy> ast::Arg5<T> {
    fn for_dst_id<F: FnMut(T)>(&self, f: &mut F) {
        f(self.dst1);
        self.dst2.map(|t| f(t));
    }
}

impl<T> ast::Operand<T> {
    fn map_id<U, F: FnMut(T) -> U>(self, f: &mut F) -> ast::Operand<U> {
        match self {
            ast::Operand::Reg(i) => ast::Operand::Reg(f(i)),
            ast::Operand::RegOffset(i, o) => ast::Operand::RegOffset(f(i), o),
            ast::Operand::Imm(v) => ast::Operand::Imm(v),
        }
    }

    fn visit_id_mut<F: FnMut(bool, &mut T)>(&mut self, f: &mut F) {
        match self {
            ast::Operand::Reg(i) => f(false, i),
            ast::Operand::RegOffset(i, _) => f(false, i),
            ast::Operand::Imm(_) => (),
        }
    }
}

impl<T> ast::MovOperand<T> {
    fn map_id<U, F: FnMut(T) -> U>(self, f: &mut F) -> ast::MovOperand<U> {
        match self {
            ast::MovOperand::Op(o) => ast::MovOperand::Op(o.map_id(f)),
            ast::MovOperand::Vec(s1, s2) => ast::MovOperand::Vec(s1, s2),
        }
    }

    fn visit_id_mut<F: FnMut(bool, &mut T)>(&mut self, f: &mut F) {
        match self {
            ast::MovOperand::Op(o) => o.visit_id_mut(f),
            ast::MovOperand::Vec(_, _) => (),
        }
    }
}

// CFGs below taken from "Modern Compiler Implementation in Java"
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast;
    use crate::ptx;

    // page 411
    #[test]
    fn to_reverse_postorder1() {
        let input = vec![
            BasicBlock {
                // A
                start: StmtIndex(0),
                pred: vec![],
                succ: vec![BBIndex(1), BBIndex(2)],
            },
            BasicBlock {
                // B
                start: StmtIndex(1),
                pred: vec![BBIndex(0), BBIndex(11)],
                succ: vec![BBIndex(3), BBIndex(6)],
            },
            BasicBlock {
                // C
                start: StmtIndex(2),
                pred: vec![BBIndex(0), BBIndex(4)],
                succ: vec![BBIndex(4), BBIndex(7)],
            },
            BasicBlock {
                // D
                start: StmtIndex(3),
                pred: vec![BBIndex(1)],
                succ: vec![BBIndex(5), BBIndex(6)],
            },
            BasicBlock {
                // E
                start: StmtIndex(4),
                pred: vec![BBIndex(2)],
                succ: vec![BBIndex(2), BBIndex(7)],
            },
            BasicBlock {
                // F
                start: StmtIndex(5),
                pred: vec![BBIndex(3)],
                succ: vec![BBIndex(8), BBIndex(10)],
            },
            BasicBlock {
                // G
                start: StmtIndex(6),
                pred: vec![BBIndex(1), BBIndex(3)],
                succ: vec![BBIndex(9)],
            },
            BasicBlock {
                // H
                start: StmtIndex(7),
                pred: vec![BBIndex(2), BBIndex(4)],
                succ: vec![BBIndex(12)],
            },
            BasicBlock {
                // I
                start: StmtIndex(8),
                pred: vec![BBIndex(5), BBIndex(9)],
                succ: vec![BBIndex(11)],
            },
            BasicBlock {
                // J
                start: StmtIndex(9),
                pred: vec![BBIndex(6)],
                succ: vec![BBIndex(8)],
            },
            BasicBlock {
                // K
                start: StmtIndex(10),
                pred: vec![BBIndex(5)],
                succ: vec![BBIndex(11)],
            },
            BasicBlock {
                // L
                start: StmtIndex(11),
                pred: vec![BBIndex(8), BBIndex(10)],
                succ: vec![BBIndex(1), BBIndex(12)],
            },
            BasicBlock {
                // M
                start: StmtIndex(12),
                pred: vec![BBIndex(7), BBIndex(11)],
                succ: vec![],
            },
        ];
        let rpostord = to_reverse_postorder(&input);
        assert_eq!(
            rpostord,
            vec![
                BBIndex(0),  // A
                BBIndex(2),  // C
                BBIndex(4),  // E
                BBIndex(7),  // H
                BBIndex(1),  // B
                BBIndex(3),  // D
                BBIndex(6),  // G
                BBIndex(9),  // J
                BBIndex(5),  // F
                BBIndex(10), // K
                BBIndex(8),  // I
                BBIndex(11), // L
                BBIndex(12), // M
            ]
        );
    }

    #[test]
    fn get_basic_blocks_empty() {
        let func = Vec::new();
        let bbs = get_basic_blocks(&func);
        assert_eq!(
            bbs,
            vec![BasicBlock {
                start: StmtIndex(0),
                pred: vec![],
                succ: vec![]
            }]
        );
    }

    #[test]
    fn get_basic_blocks_miniloop() {
        let func = vec![
            Statement::Label(12),
            Statement::Instruction(
                None,
                ast::Instruction::Bra(ast::BraData {}, ast::Arg1 { src: 12 }),
            ),
        ];
        let bbs = get_basic_blocks(&func);
        assert_eq!(
            bbs,
            vec![BasicBlock {
                start: StmtIndex(0),
                pred: vec![BBIndex(0)],
                succ: vec![BBIndex(0)]
            }]
        );
    }

    // "A Simple, Fast Dominance Algorithm" - Fig. 4
    #[test]
    fn immediate_dominators1() {
        let input = vec![
            BasicBlock {
                start: StmtIndex(6),
                pred: vec![],
                succ: vec![BBIndex(1), BBIndex(2)],
            },
            BasicBlock {
                start: StmtIndex(5),
                pred: vec![BBIndex(0)],
                succ: vec![BBIndex(5)],
            },
            BasicBlock {
                start: StmtIndex(4),
                pred: vec![BBIndex(0)],
                succ: vec![BBIndex(3), BBIndex(4)],
            },
            BasicBlock {
                start: StmtIndex(3),
                pred: vec![BBIndex(2), BBIndex(4)],
                succ: vec![BBIndex(4)],
            },
            BasicBlock {
                start: StmtIndex(2),
                pred: vec![BBIndex(2), BBIndex(3), BBIndex(5)],
                succ: vec![BBIndex(3), BBIndex(5)],
            },
            BasicBlock {
                start: StmtIndex(1),
                pred: vec![BBIndex(1), BBIndex(4)],
                succ: vec![BBIndex(4)],
            },
        ];
        let reverse_postorder = vec![
            BBIndex(0),
            BBIndex(1),
            BBIndex(2),
            BBIndex(3),
            BBIndex(4),
            BBIndex(5),
        ];
        let imm_dominators = immediate_dominators(&input, &reverse_postorder);
        assert_eq!(
            imm_dominators,
            vec![
                BBIndex(0),
                BBIndex(0),
                BBIndex(0),
                BBIndex(0),
                BBIndex(0),
                BBIndex(0)
            ]
        );
    }

    // page 411
    #[test]
    fn immediate_dominators2() {
        let input = vec![
            BasicBlock {
                // A
                start: StmtIndex(0),
                pred: vec![],
                succ: vec![BBIndex(1), BBIndex(2)],
            },
            BasicBlock {
                // B
                start: StmtIndex(1),
                pred: vec![BBIndex(0), BBIndex(11)],
                succ: vec![BBIndex(3), BBIndex(6)],
            },
            BasicBlock {
                // C
                start: StmtIndex(2),
                pred: vec![BBIndex(0), BBIndex(4)],
                succ: vec![BBIndex(4), BBIndex(7)],
            },
            BasicBlock {
                // D
                start: StmtIndex(3),
                pred: vec![BBIndex(1)],
                succ: vec![BBIndex(5), BBIndex(6)],
            },
            BasicBlock {
                // E
                start: StmtIndex(4),
                pred: vec![BBIndex(2)],
                succ: vec![BBIndex(2), BBIndex(7)],
            },
            BasicBlock {
                // F
                start: StmtIndex(5),
                pred: vec![BBIndex(3)],
                succ: vec![BBIndex(8), BBIndex(10)],
            },
            BasicBlock {
                // G
                start: StmtIndex(6),
                pred: vec![BBIndex(1), BBIndex(3)],
                succ: vec![BBIndex(9)],
            },
            BasicBlock {
                // H
                start: StmtIndex(7),
                pred: vec![BBIndex(2), BBIndex(4)],
                succ: vec![BBIndex(12)],
            },
            BasicBlock {
                // I
                start: StmtIndex(8),
                pred: vec![BBIndex(5), BBIndex(9)],
                succ: vec![BBIndex(11)],
            },
            BasicBlock {
                // J
                start: StmtIndex(9),
                pred: vec![BBIndex(6)],
                succ: vec![BBIndex(8)],
            },
            BasicBlock {
                // K
                start: StmtIndex(10),
                pred: vec![BBIndex(5)],
                succ: vec![BBIndex(11)],
            },
            BasicBlock {
                // L
                start: StmtIndex(11),
                pred: vec![BBIndex(8), BBIndex(10)],
                succ: vec![BBIndex(1), BBIndex(12)],
            },
            BasicBlock {
                // M
                start: StmtIndex(12),
                pred: vec![BBIndex(7), BBIndex(11)],
                succ: vec![],
            },
        ];
        let reverse_postorder = vec![
            BBIndex(0),  // A
            BBIndex(2),  // C
            BBIndex(4),  // E
            BBIndex(7),  // H
            BBIndex(1),  // B
            BBIndex(3),  // D
            BBIndex(6),  // G
            BBIndex(9),  // J
            BBIndex(5),  // F
            BBIndex(10), // K
            BBIndex(8),  // I
            BBIndex(11), // L
            BBIndex(12), // M
        ];
        let imm_dominators = immediate_dominators(&input, &reverse_postorder);
        assert_eq!(
            imm_dominators,
            vec![
                BBIndex(0),
                BBIndex(0),
                BBIndex(0),
                BBIndex(1),
                BBIndex(2),
                BBIndex(3),
                BBIndex(1),
                BBIndex(2),
                BBIndex(1),
                BBIndex(6),
                BBIndex(5),
                BBIndex(1),
                BBIndex(0)
            ]
        );
    }

    fn sort_pred_succ(bb: &mut BasicBlock) {
        bb.pred.sort();
        bb.succ.sort();
    }

    // page 403
    #[test]
    fn gather_phi_sets_19_4() {
        let func = "{
            mov.u32     i, 1;
            mov.u32     j, 1;
            mov.u32     k, 0;
        block_2:
            setp.ge.u32 p, k, 100;
            @p bra      block_4;
        block_3:
            setp.ge.u32 q, j, 20;
            @q bra      block_6;
        block_5:
            mov.u32     j, i;
            add.u32     k, k, 1;
            bra         block_7;
        block_6:
            mov.u32     j, k;
            add.u32     k, k, 2;
        block_7:
            bra         block_2;
        block_4:
            ret;
        }";
        let mut errors = Vec::new();
        let ast = ptx::FunctionBodyParser::new()
            .parse(&mut errors, func)
            .unwrap();
        assert_eq!(errors.len(), 0);
        let (normalized_ids, _) = normalize_identifiers(ast);
        let mut bbs = get_basic_blocks(&normalized_ids);
        bbs.iter_mut().for_each(sort_pred_succ);
        assert_eq!(
            bbs,
            vec![
                BasicBlock {
                    start: StmtIndex(0),
                    pred: vec![],
                    succ: vec![BBIndex(1)]
                },
                BasicBlock {
                    start: StmtIndex(3),
                    pred: vec![BBIndex(0), BBIndex(5)],
                    succ: vec![BBIndex(2), BBIndex(6)]
                },
                BasicBlock {
                    start: StmtIndex(6),
                    pred: vec![BBIndex(1)],
                    succ: vec![BBIndex(3), BBIndex(4)]
                },
                BasicBlock {
                    start: StmtIndex(9),
                    pred: vec![BBIndex(2)],
                    succ: vec![BBIndex(5)]
                },
                BasicBlock {
                    start: StmtIndex(13),
                    pred: vec![BBIndex(2)],
                    succ: vec![BBIndex(5)]
                },
                BasicBlock {
                    start: StmtIndex(16),
                    pred: vec![BBIndex(3), BBIndex(4)],
                    succ: vec![BBIndex(1)]
                },
                BasicBlock {
                    start: StmtIndex(18),
                    pred: vec![BBIndex(1)],
                    succ: vec![]
                },
            ]
        );
    }
}
