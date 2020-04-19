use crate::ast;
use bit_vec::BitVec;
use rspirv::dr;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::{cell::RefCell, ptr};

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

struct IdWordMap<'a>(HashMap<&'a str, spirv::Word>);

impl<'a> IdWordMap<'a> {
    fn new() -> Self {
        IdWordMap(HashMap::new())
    }
}

impl<'a> IdWordMap<'a> {
    fn get_or_add(&mut self, b: &mut dr::Builder, id: &'a str) -> spirv::Word {
        *self.0.entry(id).or_insert_with(|| b.id())
    }
}

pub fn to_spirv(ast: ast::Module) -> Result<Vec<u32>, rspirv::dr::Error> {
    let mut builder = dr::Builder::new();
    let mut ids = IdWordMap::new();
    // https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_logicallayout_a_logical_layout_of_a_module
    builder.set_version(1, 0);
    emit_capabilities(&mut builder);
    emit_extensions(&mut builder);
    emit_extended_instruction_sets(&mut builder);
    emit_memory_model(&mut builder);
    let mut map = TypeWordMap::new(&mut builder);
    for f in ast.functions {
        emit_function(&mut builder, &mut map, &mut ids, f)?;
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
    ids: &mut IdWordMap<'a>,
    f: ast::Function<'a>,
) -> Result<(), rspirv::dr::Error> {
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
    let normalized_ids = normalize_identifiers(f.body);
    let bbs = get_basic_blocks(&normalized_ids);
    let rpostorder = to_reverse_postorder(&bbs);
    let dom_fronts = dominance_frontiers(&bbs, &rpostorder);
    let ssa = ssa_legalize(normalized_ids, dom_fronts);
    emit_function_body_ops(ssa, builder);
    builder.ret()?;
    builder.end_function()?;
    Ok(())
}

fn emit_function_body_ops(ssa: Vec<Statement>, builder: &mut dr::Builder) {
    unimplemented!()
}

// TODO: support scopes
fn normalize_identifiers<'a>(func: Vec<ast::Statement<&'a str>>) -> Vec<Statement> {
    let mut result = Vec::with_capacity(func.len());
    let mut id: u32 = 0;
    let mut known_ids = HashMap::new();
    let mut get_or_add = |key| {
        *known_ids.entry(key).or_insert_with(|| {
            id += 1;
            id
        })
    };
    for s in func {
        if let Some(s) = Statement::from_ast(s, &mut get_or_add) {
            result.push(s);
        }
    }
    result
}

fn ssa_legalize(func: Vec<Statement>, dom_fronts: Vec<HashSet<BBIndex>>) -> Vec<Statement> {
    unimplemented!()
}

fn get_basic_blocks(fun: &Vec<Statement>) -> Vec<BasicBlock> {
    let mut direct_bb_start = Vec::new();
    let mut indirect_bb_start = Vec::new();
    let mut labels = HashMap::new();
    for (idx, s) in fun.iter().enumerate() {
        match s {
            Statement::Instruction(_, i) => {
                if let Some(id) = i.jump_target() {
                    indirect_bb_start.push((StmtIndex(idx), id));
                    if idx + 1 < fun.len() {
                        direct_bb_start.push((StmtIndex(idx), StmtIndex(idx + 1)));
                    }
                }
            }
            Statement::Label(id) => {
                labels.insert(id, StmtIndex(idx));
            }
            Statement::Phi(_) => (),
        };
    }
    let mut bbs_map = BTreeMap::new();
    bbs_map.insert(
        StmtIndex(0),
        BasicBlock {
            start: StmtIndex(0),
            pred: Vec::new(),
            succ: Vec::new(),
        },
    );
    // TODO: handle jumps into nowhere
    let resolved_indirect_bb_start = indirect_bb_start
        .into_iter()
        .map(|(idx, id)| (idx, labels[&id]))
        .collect::<Vec<_>>();
    for (_, to) in direct_bb_start
        .iter()
        .chain(resolved_indirect_bb_start.iter())
    {
        bbs_map.entry(*to).or_insert_with(|| BasicBlock {
            start: *to,
            pred: Vec::new(),
            succ: Vec::new(),
        });
    }
    let indexed_bbs_map = bbs_map
        .into_iter()
        .enumerate()
        .map(|(idx, (key, val))| (key, (BBIndex(idx), RefCell::new(val))))
        .collect::<BTreeMap<_, _>>();
    for (from, to) in direct_bb_start
        .iter()
        .chain(resolved_indirect_bb_start.iter())
    {
        let (_, (from_idx, from_ref)) = indexed_bbs_map.range(..=*from).next_back().unwrap();
        let (to_idx, to_ref) = indexed_bbs_map.get(to).unwrap();
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
fn dominance_frontiers(bbs: &Vec<BasicBlock>, order: &Vec<BBIndex>) -> Vec<HashSet<BBIndex>> {
    let doms = immediate_dominators(bbs, order);
    let mut result = vec![HashSet::new(); bbs.len()];
    for (bb_idx, b) in bbs.iter().enumerate() {
        if b.pred.len() < 2 { continue; }
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
    let mut doms = vec![BBIndex(usize::max_value()); bbs.len() - 1];
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

fn intersect(doms: &mut Vec<BBIndex>, b1: BBIndex, b2: BBIndex) -> BBIndex {
    let mut finger1 = b1;
    let mut finger2 = b2;
    while finger1 != finger2 {
        while finger1 < finger2 {
            finger1 = doms[finger1.0];
        }
        while finger2 < finger1 {
            finger2 = doms[finger2.0];
        }
    }
    finger1
}

// "A Simple Algorithm for Global Data Flow Analysis Problems" - Hecht, M. S., & Ullman, J. D. (1975)
fn to_reverse_postorder(input: &Vec<BasicBlock>) -> Vec<BBIndex> {
    let mut i = input.len();
    let mut old = BitVec::from_elem(input.len(), false);
    // I would do just vec![BasicBlock::empty(), input.len()], but Vec<T> is not Copy
    let mut result = Vec::with_capacity(input.len());
    unsafe { result.set_len(input.len()) };
    // original uses recursion and implicit stack, we do it explictly
    let mut state = Vec::new();
    state.push((BBIndex(0), 0usize));
    loop {
        if let Some((BBIndex(bb), succ_iter_idx)) = state.last_mut() {
            let bb = *bb;
            old.set(bb, true);
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

#[derive(Eq, PartialEq, Debug, Copy, Clone, Ord, PartialOrd)]
struct StmtIndex(pub usize);
#[derive(Eq, PartialEq, Debug, Copy, Clone, PartialOrd, Hash)]
struct BBIndex(pub usize);

#[derive(Eq, PartialEq, Debug, Clone)]
struct BasicBlock {
    start: StmtIndex,
    pred: Vec<BBIndex>,
    succ: Vec<BBIndex>,
}

enum Statement {
    Label(u32),
    Instruction(Option<ast::PredAt<u32>>, ast::Instruction<u32>),
    Phi(Vec<spirv::Word>),
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
}

impl<T> ast::PredAt<T> {
    fn map_id<U, F: FnMut(T) -> U>(self, f: &mut F) -> ast::PredAt<U> {
        ast::PredAt {
            not: self.not,
            label: f(self.label),
        }
    }
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
            ast::Instruction::At(d, a) => ast::Instruction::At(d, a.map_id(f)),
            ast::Instruction::Ret(d) => ast::Instruction::Ret(d),
        }
    }
}

impl<T: Copy> ast::Instruction<T> {
    fn jump_target(&self) -> Option<T> {
        match self {
            ast::Instruction::Bra(d, a) => Some(a.dst),
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
            | ast::Instruction::At(_, _)
            | ast::Instruction::Ret(_) => None,
        }
    }
}

impl<T> ast::Arg1<T> {
    fn map_id<U, F: FnMut(T) -> U>(self, f: &mut F) -> ast::Arg1<U> {
        ast::Arg1 { dst: f(self.dst) }
    }
}

impl<T> ast::Arg2<T> {
    fn map_id<U, F: FnMut(T) -> U>(self, f: &mut F) -> ast::Arg2<U> {
        ast::Arg2 {
            dst: f(self.dst),
            src: self.src.map_id(f),
        }
    }
}

impl<T> ast::Arg2Mov<T> {
    fn map_id<U, F: FnMut(T) -> U>(self, f: &mut F) -> ast::Arg2Mov<U> {
        ast::Arg2Mov {
            dst: f(self.dst),
            src: self.src.map_id(f),
        }
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
}

impl<T> ast::Operand<T> {
    fn map_id<U, F: FnMut(T) -> U>(self, f: &mut F) -> ast::Operand<U> {
        match self {
            ast::Operand::Reg(i) => ast::Operand::Reg(f(i)),
            ast::Operand::RegOffset(i, o) => ast::Operand::RegOffset(f(i), o),
            ast::Operand::Imm(v) => ast::Operand::Imm(v),
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
}

// CFGs below taken from "Modern Compiler Implementation in Java"
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // page 411
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
                ast::Instruction::Bra(ast::BraData {}, ast::Arg1 { dst: 12 }),
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
}
