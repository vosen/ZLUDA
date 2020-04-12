use crate::ast;
use rspirv::dr;
use std::collections::HashMap;

pub struct TranslationError {

}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum SpirvType {
    Base(BaseType),
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum BaseType {
    Int8,
    Int16,
    Int32,
    Int64,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Float16,
    Float32,
    Float64,
}

struct TypeWordMap {
    void: spirv::Word,
    fn_void: spirv::Word,
    complex: HashMap<SpirvType, spirv::Word>
}

impl TypeWordMap {
    fn new(b: &mut dr::Builder) -> TypeWordMap {
        let void = b.type_void();
        TypeWordMap {
            void: void,
            fn_void: b.type_function(void, vec![]),
            complex: HashMap::<SpirvType, spirv::Word>::new()
        }
    }

    fn void(&self) -> spirv::Word { self.void }
    fn fn_void(&self) -> spirv::Word { self.fn_void }

    fn get_or_add(&mut self, b: &mut dr::Builder, t: SpirvType) -> spirv::Word {
        *self.complex.entry(t).or_insert_with(|| {
            match t {
                SpirvType::Base(BaseType::Int8) => b.type_int(8, 1),
                SpirvType::Base(BaseType::Int16) => b.type_int(16, 1),
                SpirvType::Base(BaseType::Int32) => b.type_int(32, 1),
                SpirvType::Base(BaseType::Int64) => b.type_int(64, 1),
                SpirvType::Base(BaseType::Uint8) => b.type_int(8, 0),
                SpirvType::Base(BaseType::Uint16) => b.type_int(16, 0),
                SpirvType::Base(BaseType::Uint32) => b.type_int(32, 0),
                SpirvType::Base(BaseType::Uint64) => b.type_int(64, 0),
                SpirvType::Base(BaseType::Float16) => b.type_float(16),
                SpirvType::Base(BaseType::Float32) => b.type_float(32),
                SpirvType::Base(BaseType::Float64) => b.type_float(64),
            }
        })
    }
}

pub fn to_spirv(ast: ast::Module) -> Result<Vec<u32>, TranslationError> {
    let mut builder = dr::Builder::new();
    // https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_logicallayout_a_logical_layout_of_a_module
    builder.set_version(1, 0);
    emit_capabilities(&mut builder);
    emit_extensions(&mut builder);
    emit_extended_instruction_sets(&mut builder);
    emit_memory_model(&mut builder);
    let mut map = TypeWordMap::new(&mut builder);
    for f in ast.functions {
        emit_function(&mut builder, &mut map, &f);
    }
    Ok(vec!())
}

fn emit_capabilities(builder: &mut dr::Builder) {
    builder.capability(spirv::Capability::Linkage);
    builder.capability(spirv::Capability::Addresses);
    builder.capability(spirv::Capability::Kernel);
    builder.capability(spirv::Capability::Int64);
    builder.capability(spirv::Capability::Int8);
}

fn emit_extensions(_: &mut dr::Builder) {

}

fn emit_extended_instruction_sets(builder: &mut dr::Builder) {
    builder.ext_inst_import("OpenCL.std");
}

fn emit_memory_model(builder: &mut dr::Builder) {
    builder.memory_model(spirv::AddressingModel::Physical64, spirv::MemoryModel::OpenCL);
}

fn emit_function(builder: &mut dr::Builder, map: &TypeWordMap, f: &ast::Function) {
    let func_id = builder.begin_function(map.void(), None, spirv::FunctionControl::NONE, map.fn_void());

    builder.ret();
    builder.end_function();
}