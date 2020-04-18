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

fn ssa_legalize(func: Vec<Statement>) -> Vec<Statement> {
    vec![]
}

enum Statement {
    Label(u32),
    Instruction(Option<ast::PredAt<u32>>, ast::Instruction<u32>),
    Phi(Vec<spirv::Word>),
}

impl Statement {
    fn from_ast<'a, F: FnMut(&'a str) -> u32>(s: ast::Statement<&'a str>, f: &mut F) -> Option<Self> {
        match s {
            ast::Statement::Label(name) => Some(Statement::Label(f(name))),
            ast::Statement::Instruction(p, i) => Some(Statement::Instruction(
                p.map(|p| p.map_id(f)),
                i.map_id(f),
            )),
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

impl<T>  ast::Instruction<T> {
    fn map_id<U, F: FnMut(T) -> U>(self, f: &mut F) ->  ast::Instruction<U> {
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
            ast::MovOperand::Vec(s1, s2) => ast::MovOperand::Vec(s1, s2)
        }
    }
}