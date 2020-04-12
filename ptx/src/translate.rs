use crate::ast;
use rspirv::dr;
use std::collections::hash_map::Entry;
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
    fn new() -> Self { IdWordMap(HashMap::new()) }
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
            ast::Statement::Instruction(i) => panic!(),
        }
    }
    builder.ret()?;
    builder.end_function()?;
    Ok(())
}
