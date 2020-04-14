use crate::ast;
use rspirv::dr;
use std::collections::HashMap;

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
        emit_function(&mut builder, &mut map, &mut ids, &f)?;
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
    f: &ast::Function<'a>,
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
    for s in f.body.iter() {
        match s {
            ast::Statement::Label(name) => {
                let id = ids.get_or_add(builder, name);
                builder.begin_block(Some(id))?;
            }
            ast::Statement::Variable(var) => panic!(),
            ast::Statement::Instruction(_, _) => panic!(),
        }
    }
    builder.ret()?;
    builder.end_function()?;
    Ok(())
}

// TODO: support scopes
fn normalize_identifiers<'a>(func: Vec<ast::Statement<'a>>) -> Vec<Statement> {
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

fn ssa_legalize(func: Vec<Statement>) -> Vec<Statement> {
    vec![]
}

enum Statement {
    Label(u32),
    Instruction(Option<PredAt>, Instruction),
    Phi(Vec<spirv::Word>),
}

impl Statement {
    fn from_ast<'a, F: FnMut(&'a str) -> u32>(s: ast::Statement<'a>, f: &mut F) -> Option<Self> {
        match s {
            ast::Statement::Label(name) => Some(Statement::Label(f(name))),
            ast::Statement::Instruction(p, i) => Some(Statement::Instruction(
                p.map(|p| PredAt::from_ast(p, f)),
                Instruction::from_ast(i, f),
            )),
            ast::Statement::Variable(_) => None,
        }
    }
}

struct PredAt {
    pub not: bool,
    pub label: u32,
}

impl PredAt {
    fn from_ast<'a, F: FnMut(&'a str) -> u32>(p: ast::PredAt<'a>, f: &mut F) -> Self {
        PredAt {
            not: p.not,
            label: f(p.label),
        }
    }
}

enum Instruction {
    Ld(ast::LdData, Arg2),
    Mov(ast::MovData, Arg2Mov),
    Mul(ast::MulData, Arg3),
    Add(ast::AddData, Arg3),
    Setp(ast::SetpData, Arg4),
    SetpBool(ast::SetpBoolData, Arg5),
    Not(ast::NotData, Arg2),
    Bra(ast::BraData, Arg1),
    Cvt(ast::CvtData, Arg2),
    Shl(ast::ShlData, Arg3),
    St(ast::StData, Arg2),
    At(ast::AtData, Arg1),
    Ret(ast::RetData),
}

impl Instruction {
    fn from_ast<'a, F: FnMut(&'a str) -> u32>(i: ast::Instruction<'a>, f: &mut F) -> Self {
        match i {
            ast::Instruction::Ld(d, a) => Instruction::Ld(d, Arg2::from_ast(a, f)),
            ast::Instruction::Mov(d, a) => Instruction::Mov(d, Arg2Mov::from_ast(a, f)),
            ast::Instruction::Mul(d, a) => Instruction::Mul(d, Arg3::from_ast(a, f)),
            ast::Instruction::Add(d, a) => Instruction::Add(d, Arg3::from_ast(a, f)),
            ast::Instruction::Setp(d, a) => Instruction::Setp(d, Arg4::from_ast(a, f)),
            ast::Instruction::SetpBool(d, a) => Instruction::SetpBool(d, Arg5::from_ast(a, f)),
            ast::Instruction::Not(d, a) => Instruction::Not(d, Arg2::from_ast(a, f)),
            ast::Instruction::Bra(d, a) => Instruction::Bra(d, Arg1::from_ast(a, f)),
            ast::Instruction::Cvt(d, a) => Instruction::Cvt(d, Arg2::from_ast(a, f)),
            ast::Instruction::Shl(d, a) => Instruction::Shl(d, Arg3::from_ast(a, f)),
            ast::Instruction::St(d, a) => Instruction::St(d, Arg2::from_ast(a, f)),
            ast::Instruction::At(d, a) => Instruction::At(d, Arg1::from_ast(a, f)),
            ast::Instruction::Ret(d) => Instruction::Ret(d),
        }
    }
}

pub struct Arg1 {
    pub dst: u32,
}

impl Arg1 {
    fn from_ast<'a, F: FnMut(&'a str) -> u32>(a: ast::Arg1<'a>, f: &mut F) -> Self {
        Arg1 { dst: f(a.dst) }
    }
}

pub struct Arg2 {
    pub dst: u32,
    pub src: Operand,
}

impl Arg2 {
    fn from_ast<'a, F: FnMut(&'a str) -> u32>(a: ast::Arg2<'a>, f: &mut F) -> Self {
        Arg2 {
            dst: f(a.dst),
            src: Operand::from_ast(a.src, f),
        }
    }
}

pub struct Arg2Mov {
    pub dst: u32,
    pub src: MovOperand,
}

impl Arg2Mov {
    fn from_ast<'a, F: FnMut(&'a str) -> u32>(a: ast::Arg2Mov<'a>, f: &mut F) -> Self {
        Arg2Mov {
            dst: f(a.dst),
            src: MovOperand::from_ast(a.src, f),
        }
    }
}

pub struct Arg3 {
    pub dst: u32,
    pub src1: Operand,
    pub src2: Operand,
}

impl Arg3 {
    fn from_ast<'a, F: FnMut(&'a str) -> u32>(a: ast::Arg3<'a>, f: &mut F) -> Self {
        Arg3 {
            dst: f(a.dst),
            src1: Operand::from_ast(a.src1, f),
            src2: Operand::from_ast(a.src2, f),
        }
    }
}

pub struct Arg4 {
    pub dst1: u32,
    pub dst2: Option<u32>,
    pub src1: Operand,
    pub src2: Operand,
}

impl Arg4 {
    fn from_ast<'a, F: FnMut(&'a str) -> u32>(a: ast::Arg4<'a>, f: &mut F) -> Self {
        Arg4 {
            dst1: f(a.dst1),
            dst2: a.dst2.map(|i| f(i)),
            src1: Operand::from_ast(a.src1, f),
            src2: Operand::from_ast(a.src2, f),
        }
    }
}

pub struct Arg5 {
    pub dst1: u32,
    pub dst2: Option<u32>,
    pub src1: Operand,
    pub src2: Operand,
    pub src3: Operand,
}

impl Arg5 {
    fn from_ast<'a, F: FnMut(&'a str) -> u32>(a: ast::Arg5<'a>, f: &mut F) -> Self {
        Arg5 {
            dst1: f(a.dst1),
            dst2: a.dst2.map(|i| f(i)),
            src1: Operand::from_ast(a.src1, f),
            src2: Operand::from_ast(a.src2, f),
            src3: Operand::from_ast(a.src3, f),
        }
    }
}

pub enum Operand {
    Reg(u32),
    RegOffset(u32, i32),
    Imm(i128),
}

impl Operand {
    fn from_ast<'a, F: FnMut(&'a str) -> u32>(a: ast::Operand<'a>, f: &mut F) -> Self {
        match a {
            ast::Operand::Reg(i) => Operand::Reg(f(i)),
            ast::Operand::RegOffset(i, o) => Operand::RegOffset(f(i), o),
            ast::Operand::Imm(v) => Operand::Imm(v),
        }
    }
}

pub enum MovOperand {
    Op(Operand),
    Vec(String, String),
}

impl MovOperand {
    fn from_ast<'a, F: FnMut(&'a str) -> u32>(a: ast::MovOperand<'a>, f: &mut F) -> Self {
        match a {
            ast::MovOperand::Op(o) => MovOperand::Op(Operand::from_ast(o, f)),
            ast::MovOperand::Vec(var, idx) => {
                MovOperand::Vec(var.to_owned(), idx.to_string())
            }
        }
    }
}
