use crate::ast;
use half::f16;
use rspirv::dr;
use std::{borrow::Cow, collections::BTreeSet, ffi::CString, hash::Hash, iter, mem};
use std::{
    collections::{hash_map, HashMap, HashSet},
    convert::TryInto,
};

use rspirv::binary::Assemble;

static ZLUDA_PTX_IMPL: &'static [u8] = include_bytes!("../lib/zluda_ptx_impl.spv");

quick_error! {
    #[derive(Debug)]
    pub enum TranslateError {
        UnknownSymbol {}
        UntypedSymbol {}
        MismatchedType {}
        Spirv(err: rspirv::dr::Error) {
            from()
            display("{}", err)
            cause(err)
        }
        Unreachable {}
        Todo {}
    }
}

#[cfg(debug_assertions)]
fn error_unreachable() -> TranslateError {
    unreachable!()
}

#[cfg(not(debug_assertions))]
fn error_unreachable() -> TranslateError {
    TranslateError::Unreachable
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum SpirvType {
    Base(SpirvScalarKey),
    Vector(SpirvScalarKey, u8),
    Array(SpirvScalarKey, Vec<u32>),
    Pointer(Box<SpirvType>, spirv::StorageClass),
    Func(Option<Box<SpirvType>>, Vec<SpirvType>),
    Struct(Vec<SpirvScalarKey>),
}

impl SpirvType {
    fn new_pointer(t: ast::Type, sc: spirv::StorageClass) -> Self {
        let key = t.into();
        SpirvType::Pointer(Box::new(key), sc)
    }
}

impl From<ast::Type> for SpirvType {
    fn from(t: ast::Type) -> Self {
        match t {
            ast::Type::Scalar(t) => SpirvType::Base(t.into()),
            ast::Type::Vector(typ, len) => SpirvType::Vector(typ.into(), len),
            ast::Type::Array(t, len) => SpirvType::Array(t.into(), len),
            ast::Type::Pointer(pointer_t, state_space) => SpirvType::Pointer(
                Box::new(SpirvType::from(ast::Type::from(pointer_t))),
                state_space.to_spirv(),
            ),
        }
    }
}

impl From<ast::PointerType> for ast::Type {
    fn from(t: ast::PointerType) -> Self {
        match t {
            ast::PointerType::Scalar(t) => ast::Type::Scalar(t),
            ast::PointerType::Vector(t, len) => ast::Type::Vector(t, len),
            ast::PointerType::Array(t, dims) => ast::Type::Array(t, dims),
            ast::PointerType::Pointer(t, space) => {
                ast::Type::Pointer(ast::PointerType::Scalar(t), space)
            }
        }
    }
}

impl ast::Type {
    fn param_pointer_to(self, space: ast::LdStateSpace) -> Result<Self, TranslateError> {
        Ok(match self {
            ast::Type::Scalar(t) => ast::Type::Pointer(ast::PointerType::Scalar(t), space),
            ast::Type::Vector(t, len) => {
                ast::Type::Pointer(ast::PointerType::Vector(t, len), space)
            }
            ast::Type::Array(t, _) => ast::Type::Pointer(ast::PointerType::Scalar(t), space),
            ast::Type::Pointer(ast::PointerType::Scalar(t), space) => {
                ast::Type::Pointer(ast::PointerType::Pointer(t, space), space)
            }
            ast::Type::Pointer(_, _) => return Err(error_unreachable()),
        })
    }
}

impl Into<spirv::StorageClass> for ast::PointerStateSpace {
    fn into(self) -> spirv::StorageClass {
        match self {
            ast::PointerStateSpace::Const => spirv::StorageClass::UniformConstant,
            ast::PointerStateSpace::Global => spirv::StorageClass::CrossWorkgroup,
            ast::PointerStateSpace::Shared => spirv::StorageClass::Workgroup,
            ast::PointerStateSpace::Param => spirv::StorageClass::Function,
            ast::PointerStateSpace::Generic => spirv::StorageClass::Generic,
        }
    }
}

impl From<ast::ScalarType> for SpirvType {
    fn from(t: ast::ScalarType) -> Self {
        SpirvType::Base(t.into())
    }
}

struct TypeWordMap {
    void: spirv::Word,
    complex: HashMap<SpirvType, spirv::Word>,
    constants: HashMap<(SpirvType, u64), spirv::Word>,
}

// SPIR-V integer type definitions are signless, more below:
// https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_unsignedsigned_a_unsigned_versus_signed_integers
// https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_validation_rules_for_kernel_a_href_capability_capabilities_a
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum SpirvScalarKey {
    B8,
    B16,
    B32,
    B64,
    F16,
    F32,
    F64,
    Pred,
    F16x2,
}

impl From<ast::ScalarType> for SpirvScalarKey {
    fn from(t: ast::ScalarType) -> Self {
        match t {
            ast::ScalarType::B8 | ast::ScalarType::U8 | ast::ScalarType::S8 => SpirvScalarKey::B8,
            ast::ScalarType::B16 | ast::ScalarType::U16 | ast::ScalarType::S16 => {
                SpirvScalarKey::B16
            }
            ast::ScalarType::B32 | ast::ScalarType::U32 | ast::ScalarType::S32 => {
                SpirvScalarKey::B32
            }
            ast::ScalarType::B64 | ast::ScalarType::U64 | ast::ScalarType::S64 => {
                SpirvScalarKey::B64
            }
            ast::ScalarType::F16 => SpirvScalarKey::F16,
            ast::ScalarType::F32 => SpirvScalarKey::F32,
            ast::ScalarType::F64 => SpirvScalarKey::F64,
            ast::ScalarType::F16x2 => SpirvScalarKey::F16x2,
            ast::ScalarType::Pred => SpirvScalarKey::Pred,
        }
    }
}

impl TypeWordMap {
    fn new(b: &mut dr::Builder) -> TypeWordMap {
        let void = b.type_void();
        TypeWordMap {
            void: void,
            complex: HashMap::<SpirvType, spirv::Word>::new(),
            constants: HashMap::new(),
        }
    }

    fn void(&self) -> spirv::Word {
        self.void
    }

    fn get_or_add_scalar(&mut self, b: &mut dr::Builder, t: ast::ScalarType) -> spirv::Word {
        let key: SpirvScalarKey = t.into();
        self.get_or_add_spirv_scalar(b, key)
    }

    fn get_or_add_spirv_scalar(&mut self, b: &mut dr::Builder, key: SpirvScalarKey) -> spirv::Word {
        *self
            .complex
            .entry(SpirvType::Base(key))
            .or_insert_with(|| match key {
                SpirvScalarKey::B8 => b.type_int(8, 0),
                SpirvScalarKey::B16 => b.type_int(16, 0),
                SpirvScalarKey::B32 => b.type_int(32, 0),
                SpirvScalarKey::B64 => b.type_int(64, 0),
                SpirvScalarKey::F16 => b.type_float(16),
                SpirvScalarKey::F32 => b.type_float(32),
                SpirvScalarKey::F64 => b.type_float(64),
                SpirvScalarKey::Pred => b.type_bool(),
                SpirvScalarKey::F16x2 => todo!(),
            })
    }

    fn get_or_add(&mut self, b: &mut dr::Builder, t: SpirvType) -> spirv::Word {
        match t {
            SpirvType::Base(key) => self.get_or_add_spirv_scalar(b, key),
            SpirvType::Pointer(ref typ, storage) => {
                let base = self.get_or_add(b, *typ.clone());
                *self
                    .complex
                    .entry(t)
                    .or_insert_with(|| b.type_pointer(None, storage, base))
            }
            SpirvType::Vector(typ, len) => {
                let base = self.get_or_add_spirv_scalar(b, typ);
                *self
                    .complex
                    .entry(t)
                    .or_insert_with(|| b.type_vector(base, len as u32))
            }
            SpirvType::Array(typ, array_dimensions) => {
                let u32_type = self.get_or_add_scalar(b, ast::ScalarType::U32);
                let (base_type, length) = match &*array_dimensions {
                    &[len] => {
                        let base = self.get_or_add_spirv_scalar(b, typ);
                        let len_const = b.constant_u32(u32_type, None, len);
                        (base, len_const)
                    }
                    array_dimensions => {
                        let base = self
                            .get_or_add(b, SpirvType::Array(typ, array_dimensions[1..].to_vec()));
                        let len_const = b.constant_u32(u32_type, None, array_dimensions[0]);
                        (base, len_const)
                    }
                };
                *self
                    .complex
                    .entry(SpirvType::Array(typ, array_dimensions))
                    .or_insert_with(|| b.type_array(base_type, length))
            }
            SpirvType::Func(ref out_params, ref in_params) => {
                let out_t = match out_params {
                    Some(p) => self.get_or_add(b, *p.clone()),
                    None => self.void(),
                };
                let in_t = in_params
                    .iter()
                    .map(|t| self.get_or_add(b, t.clone()))
                    .collect::<Vec<_>>();
                *self
                    .complex
                    .entry(t)
                    .or_insert_with(|| b.type_function(out_t, in_t))
            }
            SpirvType::Struct(ref underlying) => {
                let underlying_ids = underlying
                    .iter()
                    .map(|t| self.get_or_add_spirv_scalar(b, *t))
                    .collect::<Vec<_>>();
                *self
                    .complex
                    .entry(t)
                    .or_insert_with(|| b.type_struct(underlying_ids))
            }
        }
    }

    fn get_or_add_fn(
        &mut self,
        b: &mut dr::Builder,
        in_params: impl ExactSizeIterator<Item = SpirvType>,
        mut out_params: impl ExactSizeIterator<Item = SpirvType>,
    ) -> (spirv::Word, spirv::Word) {
        let (out_args, out_spirv_type) = if out_params.len() == 0 {
            (None, self.void())
        } else if out_params.len() == 1 {
            let arg_as_key = out_params.next().unwrap();
            (
                Some(Box::new(arg_as_key.clone())),
                self.get_or_add(b, arg_as_key),
            )
        } else {
            todo!()
        };
        (
            out_spirv_type,
            self.get_or_add(b, SpirvType::Func(out_args, in_params.collect::<Vec<_>>())),
        )
    }

    fn get_or_add_constant(
        &mut self,
        b: &mut dr::Builder,
        typ: &ast::Type,
        init: &[u8],
    ) -> Result<spirv::Word, TranslateError> {
        Ok(match typ {
            ast::Type::Scalar(t) => match t {
                ast::ScalarType::B8 | ast::ScalarType::U8 | ast::ScalarType::S8 => self
                    .get_or_add_constant_single::<u8, _, _>(
                        b,
                        *t,
                        init,
                        |v| v as u64,
                        |b, result_type, v| b.constant_u32(result_type, None, v as u32),
                    ),
                ast::ScalarType::B16 | ast::ScalarType::U16 | ast::ScalarType::S16 => self
                    .get_or_add_constant_single::<u16, _, _>(
                        b,
                        *t,
                        init,
                        |v| v as u64,
                        |b, result_type, v| b.constant_u32(result_type, None, v as u32),
                    ),
                ast::ScalarType::B32 | ast::ScalarType::U32 | ast::ScalarType::S32 => self
                    .get_or_add_constant_single::<u32, _, _>(
                        b,
                        *t,
                        init,
                        |v| v as u64,
                        |b, result_type, v| b.constant_u32(result_type, None, v),
                    ),
                ast::ScalarType::B64 | ast::ScalarType::U64 | ast::ScalarType::S64 => self
                    .get_or_add_constant_single::<u64, _, _>(
                        b,
                        *t,
                        init,
                        |v| v,
                        |b, result_type, v| b.constant_u64(result_type, None, v),
                    ),
                ast::ScalarType::F16 => self.get_or_add_constant_single::<f16, _, _>(
                    b,
                    *t,
                    init,
                    |v| unsafe { mem::transmute::<_, u16>(v) } as u64,
                    |b, result_type, v| b.constant_f32(result_type, None, v.to_f32()),
                ),
                ast::ScalarType::F32 => self.get_or_add_constant_single::<f32, _, _>(
                    b,
                    *t,
                    init,
                    |v| unsafe { mem::transmute::<_, u32>(v) } as u64,
                    |b, result_type, v| b.constant_f32(result_type, None, v),
                ),
                ast::ScalarType::F64 => self.get_or_add_constant_single::<f64, _, _>(
                    b,
                    *t,
                    init,
                    |v| unsafe { mem::transmute::<_, u64>(v) },
                    |b, result_type, v| b.constant_f64(result_type, None, v),
                ),
                ast::ScalarType::F16x2 => return Err(TranslateError::Todo),
                ast::ScalarType::Pred => self.get_or_add_constant_single::<u8, _, _>(
                    b,
                    *t,
                    init,
                    |v| v as u64,
                    |b, result_type, v| {
                        if v == 0 {
                            b.constant_false(result_type, None)
                        } else {
                            b.constant_true(result_type, None)
                        }
                    },
                ),
            },
            ast::Type::Vector(typ, len) => {
                let result_type =
                    self.get_or_add(b, SpirvType::Vector(SpirvScalarKey::from(*typ), *len));
                let size_of_t = typ.size_of();
                let components = (0..*len)
                    .map(|x| {
                        self.get_or_add_constant(
                            b,
                            &ast::Type::Scalar(*typ),
                            &init[((size_of_t as usize) * (x as usize))..],
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                b.constant_composite(result_type, None, &components)
            }
            ast::Type::Array(typ, dims) => match dims.as_slice() {
                [] => return Err(error_unreachable()),
                [dim] => {
                    let result_type = self
                        .get_or_add(b, SpirvType::Array(SpirvScalarKey::from(*typ), vec![*dim]));
                    let size_of_t = typ.size_of();
                    let components = (0..*dim)
                        .map(|x| {
                            self.get_or_add_constant(
                                b,
                                &ast::Type::Scalar(*typ),
                                &init[((size_of_t as usize) * (x as usize))..],
                            )
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    b.constant_composite(result_type, None, &components)
                }
                [first_dim, rest @ ..] => {
                    let result_type = self.get_or_add(
                        b,
                        SpirvType::Array(SpirvScalarKey::from(*typ), rest.to_vec()),
                    );
                    let size_of_t = rest
                        .iter()
                        .fold(typ.size_of() as u32, |x, y| (x as u32) * (*y));
                    let components = (0..*first_dim)
                        .map(|x| {
                            self.get_or_add_constant(
                                b,
                                &ast::Type::Array(*typ, rest.to_vec()),
                                &init[((size_of_t as usize) * (x as usize))..],
                            )
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    b.constant_composite(result_type, None, &components)
                }
            },
            ast::Type::Pointer(typ, state_space) => {
                let base_t = typ.clone().into();
                let base = self.get_or_add_constant(b, &base_t, &[])?;
                let result_type = self.get_or_add(
                    b,
                    SpirvType::Pointer(
                        Box::new(SpirvType::from(base_t)),
                        (*state_space).to_spirv(),
                    ),
                );
                b.variable(result_type, None, (*state_space).to_spirv(), Some(base))
            }
        })
    }

    fn get_or_add_constant_single<
        T: Copy,
        CastAsU64: FnOnce(T) -> u64,
        InsertConstant: FnOnce(&mut dr::Builder, spirv::Word, T) -> spirv::Word,
    >(
        &mut self,
        b: &mut dr::Builder,
        key: ast::ScalarType,
        init: &[u8],
        cast: CastAsU64,
        f: InsertConstant,
    ) -> spirv::Word {
        let value = unsafe { *(init.as_ptr() as *const T) };
        let value_64 = cast(value);
        let ht_key = (SpirvType::Base(SpirvScalarKey::from(key)), value_64);
        match self.constants.get(&ht_key) {
            Some(value) => *value,
            None => {
                let spirv_type = self.get_or_add_scalar(b, key);
                let result = f(b, spirv_type, value);
                self.constants.insert(ht_key, result);
                result
            }
        }
    }
}

pub struct Module {
    pub spirv: dr::Module,
    pub kernel_info: HashMap<String, KernelInfo>,
    pub should_link_ptx_impl: Option<&'static [u8]>,
    pub build_options: CString,
}
impl Module {
    pub fn assemble(&self) -> Vec<u32> {
        self.spirv.assemble()
    }
}

pub struct KernelInfo {
    pub arguments_sizes: Vec<usize>,
    pub uses_shared_mem: bool,
}

pub fn to_spirv_module<'a>(ast: ast::Module<'a>) -> Result<Module, TranslateError> {
    let mut id_defs = GlobalStringIdResolver::new(1);
    let mut ptx_impl_imports = HashMap::new();
    let directives = ast
        .directives
        .into_iter()
        .filter_map(|directive| {
            translate_directive(&mut id_defs, &mut ptx_impl_imports, directive).transpose()
        })
        .collect::<Result<Vec<_>, _>>()?;
    let must_link_ptx_impl = ptx_impl_imports.len() > 0;
    let directives = ptx_impl_imports
        .into_iter()
        .map(|(_, v)| v)
        .chain(directives.into_iter())
        .collect::<Vec<_>>();
    let mut builder = dr::Builder::new();
    builder.reserve_ids(id_defs.current_id());
    let call_map = get_call_map(&directives);
    let mut directives = convert_dynamic_shared_memory_usage(directives, &mut || builder.id());
    normalize_variable_decls(&mut directives);
    let denorm_information = compute_denorm_information(&directives);
    // https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_logicallayout_a_logical_layout_of_a_module
    builder.set_version(1, 3);
    emit_capabilities(&mut builder);
    emit_extensions(&mut builder);
    let opencl_id = emit_opencl_import(&mut builder);
    emit_memory_model(&mut builder);
    let mut map = TypeWordMap::new(&mut builder);
    emit_builtins(&mut builder, &mut map, &id_defs);
    let mut kernel_info = HashMap::new();
    let build_options = emit_denorm_build_string(&call_map, &denorm_information);
    emit_directives(
        &mut builder,
        &mut map,
        &id_defs,
        opencl_id,
        &denorm_information,
        &call_map,
        directives,
        &mut kernel_info,
    )?;
    let spirv = builder.module();
    Ok(Module {
        spirv,
        kernel_info,
        should_link_ptx_impl: if must_link_ptx_impl {
            Some(ZLUDA_PTX_IMPL)
        } else {
            None
        },
        build_options,
    })
}

// TODO: remove this once we have perf-function support for denorms
fn emit_denorm_build_string(
    call_map: &HashMap<&str, HashSet<u32>>,
    denorm_information: &HashMap<MethodName, HashMap<u8, (spirv::FPDenormMode, isize)>>,
) -> CString {
    let denorm_counts = denorm_information
        .iter()
        .map(|(method, meth_denorm)| {
            let f16_count = meth_denorm
                .get(&(mem::size_of::<f16>() as u8))
                .unwrap_or(&(spirv::FPDenormMode::FlushToZero, 0))
                .1;
            let f32_count = meth_denorm
                .get(&(mem::size_of::<f32>() as u8))
                .unwrap_or(&(spirv::FPDenormMode::FlushToZero, 0))
                .1;
            (method, (f16_count + f32_count))
        })
        .collect::<HashMap<_, _>>();
    let mut flush_over_preserve = 0;
    for (kernel, children) in call_map {
        flush_over_preserve += *denorm_counts.get(&MethodName::Kernel(kernel)).unwrap_or(&0);
        for child_fn in children {
            flush_over_preserve += *denorm_counts
                .get(&MethodName::Func(*child_fn))
                .unwrap_or(&0);
        }
    }
    if flush_over_preserve > 0 {
        CString::new("-cl-denorms-are-zero").unwrap()
    } else {
        CString::default()
    }
}

fn emit_directives<'input>(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    id_defs: &GlobalStringIdResolver<'input>,
    opencl_id: spirv::Word,
    denorm_information: &HashMap<MethodName<'input>, HashMap<u8, (spirv::FPDenormMode, isize)>>,
    call_map: &HashMap<&'input str, HashSet<spirv::Word>>,
    directives: Vec<Directive>,
    kernel_info: &mut HashMap<String, KernelInfo>,
) -> Result<(), TranslateError> {
    let empty_body = Vec::new();
    for d in directives.iter() {
        match d {
            Directive::Variable(var) => {
                emit_variable(builder, map, &var)?;
            }
            Directive::Method(f) => {
                let f_body = match &f.body {
                    Some(f) => f,
                    None => {
                        if f.import_as.is_some() {
                            &empty_body
                        } else {
                            continue;
                        }
                    }
                };
                for var in f.globals.iter() {
                    emit_variable(builder, map, var)?;
                }
                emit_function_header(
                    builder,
                    map,
                    &id_defs,
                    &f.globals,
                    &f.spirv_decl,
                    &denorm_information,
                    call_map,
                    &directives,
                    kernel_info,
                )?;
                emit_function_body_ops(builder, map, opencl_id, &f_body)?;
                builder.end_function()?;
                if let (ast::MethodDecl::Func(_, fn_id, _), Some(name)) =
                    (&f.func_decl, &f.import_as)
                {
                    builder.decorate(
                        *fn_id,
                        spirv::Decoration::LinkageAttributes,
                        &[
                            dr::Operand::LiteralString(name.clone()),
                            dr::Operand::LinkageType(spirv::LinkageType::Import),
                        ],
                    );
                }
            }
        }
    }
    Ok(())
}

fn get_call_map<'input>(
    module: &[Directive<'input>],
) -> HashMap<&'input str, HashSet<spirv::Word>> {
    let mut directly_called_by = HashMap::new();
    for directive in module {
        match directive {
            Directive::Method(Function {
                func_decl,
                body: Some(statements),
                ..
            }) => {
                let call_key = MethodName::new(&func_decl);
                if let hash_map::Entry::Vacant(entry) = directly_called_by.entry(call_key) {
                    entry.insert(Vec::new());
                }
                for statement in statements {
                    match statement {
                        Statement::Call(call) => {
                            multi_hash_map_append(&mut directly_called_by, call_key, call.func);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    let mut result = HashMap::new();
    for (method_key, children) in directly_called_by.iter() {
        match method_key {
            MethodName::Kernel(name) => {
                let mut visited = HashSet::new();
                for child in children {
                    add_call_map_single(&directly_called_by, &mut visited, *child);
                }
                result.insert(*name, visited);
            }
            MethodName::Func(_) => {}
        }
    }
    result
}

fn add_call_map_single<'input>(
    directly_called_by: &MultiHashMap<MethodName<'input>, spirv::Word>,
    visited: &mut HashSet<spirv::Word>,
    current: spirv::Word,
) {
    if !visited.insert(current) {
        return;
    }
    if let Some(children) = directly_called_by.get(&MethodName::Func(current)) {
        for child in children {
            add_call_map_single(directly_called_by, visited, *child);
        }
    }
}

type MultiHashMap<K, V> = HashMap<K, Vec<V>>;

fn multi_hash_map_append<K: Eq + std::hash::Hash, V>(m: &mut MultiHashMap<K, V>, key: K, value: V) {
    match m.entry(key) {
        hash_map::Entry::Occupied(mut entry) => {
            entry.get_mut().push(value);
        }
        hash_map::Entry::Vacant(entry) => {
            entry.insert(vec![value]);
        }
    }
}

// PTX represents dynamically allocated shared local memory as
//     .extern .shared .align 4 .b8 shared_mem[];
// In SPIRV/OpenCL world this is expressed as an additional argument
// This pass looks for all uses of .extern .shared and converts them to
// an additional method argument
fn convert_dynamic_shared_memory_usage<'input>(
    module: Vec<Directive<'input>>,
    new_id: &mut impl FnMut() -> spirv::Word,
) -> Vec<Directive<'input>> {
    let mut extern_shared_decls = HashMap::new();
    for dir in module.iter() {
        match dir {
            Directive::Variable(var) => {
                if let ast::VariableType::Shared(ast::VariableGlobalType::Pointer(p_type, _)) =
                    var.v_type
                {
                    extern_shared_decls.insert(var.name, p_type);
                }
            }
            _ => {}
        }
    }
    if extern_shared_decls.len() == 0 {
        return module;
    }
    let mut methods_using_extern_shared = HashSet::new();
    let mut directly_called_by = MultiHashMap::new();
    let module = module
        .into_iter()
        .map(|directive| match directive {
            Directive::Method(Function {
                func_decl,
                globals,
                body: Some(statements),
                import_as,
                spirv_decl,
            }) => {
                let call_key = MethodName::new(&func_decl);
                let statements = statements
                    .into_iter()
                    .map(|statement| match statement {
                        Statement::Call(call) => {
                            multi_hash_map_append(&mut directly_called_by, call.func, call_key);
                            Statement::Call(call)
                        }
                        statement => statement.map_id(&mut |id, _| {
                            if extern_shared_decls.contains_key(&id) {
                                methods_using_extern_shared.insert(call_key);
                            }
                            id
                        }),
                    })
                    .collect();
                Directive::Method(Function {
                    func_decl,
                    globals,
                    body: Some(statements),
                    import_as,
                    spirv_decl,
                })
            }
            directive => directive,
        })
        .collect::<Vec<_>>();
    // If there's a chain `kernel` -> `fn1` -> `fn2`, where only `fn2` uses extern shared,
    // make sure it gets propagated to `fn1` and `kernel`
    get_callers_of_extern_shared(&mut methods_using_extern_shared, &directly_called_by);
    // now visit every method declaration and inject those additional arguments
    module
        .into_iter()
        .map(|directive| match directive {
            Directive::Method(Function {
                func_decl,
                globals,
                body: Some(statements),
                import_as,
                mut spirv_decl,
            }) => {
                if !methods_using_extern_shared.contains(&spirv_decl.name) {
                    return Directive::Method(Function {
                        func_decl,
                        globals,
                        body: Some(statements),
                        import_as,
                        spirv_decl,
                    });
                }
                let shared_id_param = new_id();
                spirv_decl.input.push({
                    ast::Variable {
                        align: None,
                        v_type: ast::Type::Pointer(
                            ast::PointerType::Scalar(ast::ScalarType::U8),
                            ast::LdStateSpace::Shared,
                        ),
                        array_init: Vec::new(),
                        name: shared_id_param,
                    }
                });
                spirv_decl.uses_shared_mem = true;
                let shared_var_id = new_id();
                let shared_var = ExpandedStatement::Variable(ast::Variable {
                    align: None,
                    name: shared_var_id,
                    array_init: Vec::new(),
                    v_type: ast::VariableType::Reg(ast::VariableRegType::Pointer(
                        ast::SizedScalarType::B8,
                        ast::PointerStateSpace::Shared,
                    )),
                });
                let shared_var_st = ExpandedStatement::StoreVar(StoreVarDetails {
                    arg: ast::Arg2St {
                        src1: shared_var_id,
                        src2: shared_id_param,
                    },
                    typ: ast::Type::Scalar(ast::ScalarType::B8),
                    member_index: None,
                });
                let mut new_statements = vec![shared_var, shared_var_st];
                replace_uses_of_shared_memory(
                    &mut new_statements,
                    new_id,
                    &extern_shared_decls,
                    &mut methods_using_extern_shared,
                    shared_id_param,
                    shared_var_id,
                    statements,
                );
                Directive::Method(Function {
                    func_decl,
                    globals,
                    body: Some(new_statements),
                    import_as,
                    spirv_decl,
                })
            }
            directive => directive,
        })
        .collect::<Vec<_>>()
}

fn replace_uses_of_shared_memory<'a>(
    result: &mut Vec<ExpandedStatement>,
    new_id: &mut impl FnMut() -> spirv::Word,
    extern_shared_decls: &HashMap<spirv::Word, ast::SizedScalarType>,
    methods_using_extern_shared: &mut HashSet<MethodName<'a>>,
    shared_id_param: spirv::Word,
    shared_var_id: spirv::Word,
    statements: Vec<ExpandedStatement>,
) {
    for statement in statements {
        match statement {
            Statement::Call(mut call) => {
                // We can safely skip checking call arguments,
                // because there's simply no way to pass shared ptr
                // without converting it to .b64 first
                if methods_using_extern_shared.contains(&MethodName::Func(call.func)) {
                    call.param_list
                        .push((shared_id_param, ast::FnArgumentType::Shared));
                }
                result.push(Statement::Call(call))
            }
            statement => {
                let new_statement = statement.map_id(&mut |id, _| {
                    if let Some(typ) = extern_shared_decls.get(&id) {
                        if *typ == ast::SizedScalarType::B8 {
                            return shared_var_id;
                        }
                        let replacement_id = new_id();
                        result.push(Statement::Conversion(ImplicitConversion {
                            src: shared_var_id,
                            dst: replacement_id,
                            from: ast::Type::Pointer(
                                ast::PointerType::Scalar(ast::ScalarType::B8),
                                ast::LdStateSpace::Shared,
                            ),
                            to: ast::Type::Pointer(
                                ast::PointerType::Scalar((*typ).into()),
                                ast::LdStateSpace::Shared,
                            ),
                            kind: ConversionKind::PtrToPtr { spirv_ptr: true },
                            src_sema: ArgumentSemantics::Default,
                            dst_sema: ArgumentSemantics::Default,
                        }));
                        replacement_id
                    } else {
                        id
                    }
                });
                result.push(new_statement);
            }
        }
    }
}

fn get_callers_of_extern_shared<'a>(
    methods_using_extern_shared: &mut HashSet<MethodName<'a>>,
    directly_called_by: &MultiHashMap<spirv::Word, MethodName<'a>>,
) {
    let direct_uses_of_extern_shared = methods_using_extern_shared
        .iter()
        .filter_map(|method| {
            if let MethodName::Func(f_id) = method {
                Some(*f_id)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    for fn_id in direct_uses_of_extern_shared {
        get_callers_of_extern_shared_single(methods_using_extern_shared, directly_called_by, fn_id);
    }
}

fn get_callers_of_extern_shared_single<'a>(
    methods_using_extern_shared: &mut HashSet<MethodName<'a>>,
    directly_called_by: &MultiHashMap<spirv::Word, MethodName<'a>>,
    fn_id: spirv::Word,
) {
    if let Some(callers) = directly_called_by.get(&fn_id) {
        for caller in callers {
            if methods_using_extern_shared.insert(*caller) {
                if let MethodName::Func(caller_fn) = caller {
                    get_callers_of_extern_shared_single(
                        methods_using_extern_shared,
                        directly_called_by,
                        *caller_fn,
                    );
                }
            }
        }
    }
}

type DenormCountMap<T> = HashMap<T, isize>;

fn denorm_count_map_update<T: Eq + Hash>(map: &mut DenormCountMap<T>, key: T, value: bool) {
    let num_value = if value { 1 } else { -1 };
    denorm_count_map_update_impl(map, key, num_value);
}

fn denorm_count_map_update_impl<T: Eq + Hash>(
    map: &mut DenormCountMap<T>,
    key: T,
    num_value: isize,
) {
    match map.entry(key) {
        hash_map::Entry::Occupied(mut counter) => {
            *(counter.get_mut()) += num_value;
        }
        hash_map::Entry::Vacant(entry) => {
            entry.insert(num_value);
        }
    }
}

// HACK ALERT!
// This function is a "good enough" heuristic of whetever to mark f16/f32 operations
// in the kernel as flushing denorms to zero or preserving them
// PTX support per-instruction ftz information. Unfortunately SPIR-V has no
// such capability, so instead we guesstimate which use is more common in the kernel
// and emit suitable execution mode
fn compute_denorm_information<'input>(
    module: &[Directive<'input>],
) -> HashMap<MethodName<'input>, HashMap<u8, (spirv::FPDenormMode, isize)>> {
    let mut denorm_methods = HashMap::new();
    for directive in module {
        match directive {
            Directive::Variable(_) | Directive::Method(Function { body: None, .. }) => {}
            Directive::Method(Function {
                func_decl,
                body: Some(statements),
                ..
            }) => {
                let mut flush_counter = DenormCountMap::new();
                let method_key = MethodName::new(func_decl);
                for statement in statements {
                    match statement {
                        Statement::Instruction(inst) => {
                            if let Some((flush, width)) = inst.flush_to_zero() {
                                denorm_count_map_update(&mut flush_counter, width, flush);
                            }
                        }
                        Statement::LoadVar(..) => {}
                        Statement::StoreVar(..) => {}
                        Statement::Call(_) => {}
                        Statement::Conditional(_) => {}
                        Statement::Conversion(_) => {}
                        Statement::Constant(_) => {}
                        Statement::RetValue(_, _) => {}
                        Statement::Label(_) => {}
                        Statement::Variable(_) => {}
                        Statement::PtrAccess { .. } => {}
                        Statement::RepackVector(_) => {}
                    }
                }
                denorm_methods.insert(method_key, flush_counter);
            }
        }
    }
    denorm_methods
        .into_iter()
        .map(|(name, v)| {
            let width_to_denorm = v
                .into_iter()
                .map(|(k, flush_over_preserve)| {
                    let mode = if flush_over_preserve > 0 {
                        spirv::FPDenormMode::FlushToZero
                    } else {
                        spirv::FPDenormMode::Preserve
                    };
                    (k, (mode, flush_over_preserve))
                })
                .collect();
            (name, width_to_denorm)
        })
        .collect()
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
enum MethodName<'input> {
    Kernel(&'input str),
    Func(spirv::Word),
}

impl<'input> MethodName<'input> {
    fn new(decl: &ast::MethodDecl<'input, spirv::Word>) -> Self {
        match decl {
            ast::MethodDecl::Kernel { name, .. } => MethodName::Kernel(name),
            ast::MethodDecl::Func(_, id, _) => MethodName::Func(*id),
        }
    }
}

fn emit_builtins(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    id_defs: &GlobalStringIdResolver,
) {
    for (reg, id) in id_defs.special_registers.builtins() {
        let result_type = map.get_or_add(
            builder,
            SpirvType::Pointer(
                Box::new(SpirvType::from(reg.get_type())),
                spirv::StorageClass::Input,
            ),
        );
        builder.variable(result_type, Some(id), spirv::StorageClass::Input, None);
        builder.decorate(
            id,
            spirv::Decoration::BuiltIn,
            &[dr::Operand::BuiltIn(reg.get_builtin())],
        );
    }
}

fn emit_function_header<'a>(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    defined_globals: &GlobalStringIdResolver<'a>,
    synthetic_globals: &[ast::Variable<ast::VariableType, spirv::Word>],
    func_decl: &SpirvMethodDecl<'a>,
    _denorm_information: &HashMap<MethodName<'a>, HashMap<u8, (spirv::FPDenormMode, isize)>>,
    call_map: &HashMap<&'a str, HashSet<spirv::Word>>,
    direcitves: &[Directive],
    kernel_info: &mut HashMap<String, KernelInfo>,
) -> Result<(), TranslateError> {
    if let MethodName::Kernel(name) = func_decl.name {
        let input_args = if !func_decl.uses_shared_mem {
            func_decl.input.as_slice()
        } else {
            &func_decl.input[0..func_decl.input.len() - 1]
        };
        let args_lens = input_args
            .iter()
            .map(|param| param.v_type.size_of())
            .collect();
        kernel_info.insert(
            name.to_string(),
            KernelInfo {
                arguments_sizes: args_lens,
                uses_shared_mem: func_decl.uses_shared_mem,
            },
        );
    }
    let (ret_type, func_type) =
        get_function_type(builder, map, &func_decl.input, &func_decl.output);
    let fn_id = match func_decl.name {
        MethodName::Kernel(name) => {
            let fn_id = defined_globals.get_id(name)?;
            let mut global_variables = defined_globals
                .variables_type_check
                .iter()
                .filter_map(|(k, t)| t.as_ref().map(|_| *k))
                .collect::<Vec<_>>();
            let mut interface = defined_globals.special_registers.interface();
            for ast::Variable { name, .. } in synthetic_globals {
                interface.push(*name);
            }
            let empty_hash_set = HashSet::new();
            let child_fns = call_map.get(name).unwrap_or(&empty_hash_set);
            for directive in direcitves {
                match directive {
                    Directive::Method(Function {
                        func_decl: ast::MethodDecl::Func(_, name, _),
                        globals,
                        ..
                    }) => {
                        if child_fns.contains(name) {
                            for var in globals {
                                interface.push(var.name);
                            }
                        }
                    }
                    _ => {}
                }
            }
            global_variables.append(&mut interface);
            builder.entry_point(spirv::ExecutionModel::Kernel, fn_id, name, global_variables);
            fn_id
        }
        MethodName::Func(name) => name,
    };
    builder.begin_function(
        ret_type,
        Some(fn_id),
        spirv::FunctionControl::NONE,
        func_type,
    )?;
    // TODO: re-enable when Intel float control extension works
    /*
    if let Some(denorm_modes) = denorm_information.get(&func_decl.name) {
        for (size_of, denorm_mode) in denorm_modes {
            builder.decorate(
                fn_id,
                spirv::Decoration::FunctionDenormModeINTEL,
                [
                    dr::Operand::LiteralInt32((*size_of as u32) * 8),
                    dr::Operand::FPDenormMode(*denorm_mode),
                ],
            )
        }
    }
    */
    for input in &func_decl.input {
        let result_type = map.get_or_add(builder, SpirvType::from(input.v_type.clone()));
        let inst = dr::Instruction::new(
            spirv::Op::FunctionParameter,
            Some(result_type),
            Some(input.name),
            Vec::new(),
        );
        builder.function.as_mut().unwrap().parameters.push(inst);
    }
    Ok(())
}

fn emit_capabilities(builder: &mut dr::Builder) {
    builder.capability(spirv::Capability::GenericPointer);
    builder.capability(spirv::Capability::Linkage);
    builder.capability(spirv::Capability::Addresses);
    builder.capability(spirv::Capability::Kernel);
    builder.capability(spirv::Capability::Int8);
    builder.capability(spirv::Capability::Int16);
    builder.capability(spirv::Capability::Int64);
    builder.capability(spirv::Capability::Float16);
    builder.capability(spirv::Capability::Float64);
    // TODO: re-enable when Intel float control extension works
    //builder.capability(spirv::Capability::FunctionFloatControlINTEL);
}

// http://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_float_controls.html
fn emit_extensions(_builder: &mut dr::Builder) {
    // TODO: re-enable when Intel float control extension works
    //builder.extension("SPV_INTEL_float_controls2");
}

fn emit_opencl_import(builder: &mut dr::Builder) -> spirv::Word {
    builder.ext_inst_import("OpenCL.std")
}

fn emit_memory_model(builder: &mut dr::Builder) {
    builder.memory_model(
        spirv::AddressingModel::Physical64,
        spirv::MemoryModel::OpenCL,
    );
}

fn translate_directive<'input>(
    id_defs: &mut GlobalStringIdResolver<'input>,
    ptx_impl_imports: &mut HashMap<String, Directive<'input>>,
    d: ast::Directive<'input, ast::ParsedArgParams<'input>>,
) -> Result<Option<Directive<'input>>, TranslateError> {
    Ok(match d {
        ast::Directive::Variable(v) => Some(Directive::Variable(translate_variable(id_defs, v)?)),
        ast::Directive::Method(f) => {
            translate_function(id_defs, ptx_impl_imports, f)?.map(Directive::Method)
        }
    })
}

fn translate_variable<'a>(
    id_defs: &mut GlobalStringIdResolver<'a>,
    var: ast::Variable<ast::VariableType, &'a str>,
) -> Result<ast::Variable<ast::VariableType, spirv::Word>, TranslateError> {
    let (space, var_type) = var.v_type.to_type();
    let mut is_variable = false;
    let var_type = match space {
        ast::StateSpace::Reg => {
            is_variable = true;
            var_type
        }
        ast::StateSpace::Const => var_type.param_pointer_to(ast::LdStateSpace::Const)?,
        ast::StateSpace::Global => var_type.param_pointer_to(ast::LdStateSpace::Global)?,
        ast::StateSpace::Local => var_type.param_pointer_to(ast::LdStateSpace::Local)?,
        ast::StateSpace::Shared => {
            // If it's a pointer it will be translated to a method parameter later
            if let ast::Type::Pointer(..) = var_type {
                is_variable = true;
                var_type
            } else {
                var_type.param_pointer_to(ast::LdStateSpace::Shared)?
            }
        }
        ast::StateSpace::Param => var_type.param_pointer_to(ast::LdStateSpace::Param)?,
    };
    Ok(ast::Variable {
        align: var.align,
        v_type: var.v_type,
        name: id_defs.get_or_add_def_typed(var.name, var_type, is_variable),
        array_init: var.array_init,
    })
}

fn translate_function<'a>(
    id_defs: &mut GlobalStringIdResolver<'a>,
    ptx_impl_imports: &mut HashMap<String, Directive<'a>>,
    f: ast::ParsedFunction<'a>,
) -> Result<Option<Function<'a>>, TranslateError> {
    let import_as = match &f.func_directive {
        ast::MethodDecl::Func(_, "__assertfail", _) => {
            Some("__zluda_ptx_impl____assertfail".to_owned())
        }
        _ => None,
    };
    let (str_resolver, fn_resolver, fn_decl) = id_defs.start_fn(&f.func_directive)?;
    let mut func = to_ssa(ptx_impl_imports, str_resolver, fn_resolver, fn_decl, f.body)?;
    func.import_as = import_as;
    if func.import_as.is_some() {
        ptx_impl_imports.insert(
            func.import_as.as_ref().unwrap().clone(),
            Directive::Method(func),
        );
        Ok(None)
    } else {
        Ok(Some(func))
    }
}

fn expand_kernel_params<'a, 'b>(
    fn_resolver: &mut FnStringIdResolver<'a, 'b>,
    args: impl Iterator<Item = &'b ast::KernelArgument<&'a str>>,
) -> Result<Vec<ast::KernelArgument<spirv::Word>>, TranslateError> {
    args.map(|a| {
        Ok(ast::KernelArgument {
            name: fn_resolver.add_def(
                a.name,
                Some(ast::Type::from(a.v_type.clone()).param_pointer_to(ast::LdStateSpace::Param)?),
                false,
            ),
            v_type: a.v_type.clone(),
            align: a.align,
            array_init: Vec::new(),
        })
    })
    .collect::<Result<_, _>>()
}

fn expand_fn_params<'a, 'b>(
    fn_resolver: &mut FnStringIdResolver<'a, 'b>,
    args: impl Iterator<Item = &'b ast::FnArgument<&'a str>>,
) -> Result<Vec<ast::FnArgument<spirv::Word>>, TranslateError> {
    args.map(|a| {
        let is_variable = match a.v_type {
            ast::FnArgumentType::Reg(_) => true,
            _ => false,
        };
        let var_type = a.v_type.to_func_type();
        Ok(ast::FnArgument {
            name: fn_resolver.add_def(a.name, Some(var_type), is_variable),
            v_type: a.v_type.clone(),
            align: a.align,
            array_init: Vec::new(),
        })
    })
    .collect()
}

fn to_ssa<'input, 'b>(
    ptx_impl_imports: &mut HashMap<String, Directive>,
    mut id_defs: FnStringIdResolver<'input, 'b>,
    fn_defs: GlobalFnDeclResolver<'input, 'b>,
    f_args: ast::MethodDecl<'input, spirv::Word>,
    f_body: Option<Vec<ast::Statement<ast::ParsedArgParams<'input>>>>,
) -> Result<Function<'input>, TranslateError> {
    let mut spirv_decl = SpirvMethodDecl::new(&f_args);
    let f_body = match f_body {
        Some(vec) => vec,
        None => {
            return Ok(Function {
                func_decl: f_args,
                body: None,
                globals: Vec::new(),
                import_as: None,
                spirv_decl,
            })
        }
    };
    let normalized_ids = normalize_identifiers(&mut id_defs, &fn_defs, f_body)?;
    let mut numeric_id_defs = id_defs.finish();
    let unadorned_statements = normalize_predicates(normalized_ids, &mut numeric_id_defs)?;
    let typed_statements =
        convert_to_typed_statements(unadorned_statements, &fn_defs, &mut numeric_id_defs)?;
    let typed_statements =
        convert_to_stateful_memory_access(&mut spirv_decl, typed_statements, &mut numeric_id_defs)?;
    let ssa_statements = insert_mem_ssa_statements(
        typed_statements,
        &mut numeric_id_defs,
        &f_args,
        &mut spirv_decl,
    )?;
    let ssa_statements = fix_builtins(ssa_statements, &mut numeric_id_defs)?;
    let mut numeric_id_defs = numeric_id_defs.finish();
    let expanded_statements = expand_arguments(ssa_statements, &mut numeric_id_defs)?;
    let expanded_statements =
        insert_implicit_conversions(expanded_statements, &mut numeric_id_defs)?;
    let mut numeric_id_defs = numeric_id_defs.unmut();
    let labeled_statements = normalize_labels(expanded_statements, &mut numeric_id_defs);
    let (f_body, globals) =
        extract_globals(labeled_statements, ptx_impl_imports, &mut numeric_id_defs);
    Ok(Function {
        func_decl: f_args,
        globals: globals,
        body: Some(f_body),
        import_as: None,
        spirv_decl,
    })
}

fn fix_builtins(
    typed_statements: Vec<TypedStatement>,
    numeric_id_defs: &mut NumericIdResolver,
) -> Result<Vec<TypedStatement>, TranslateError> {
    let mut result = Vec::with_capacity(typed_statements.len());
    for s in typed_statements {
        match s {
            Statement::LoadVar(
                mut
                details
                @
                LoadVarDetails {
                    member_index: Some((_, Some(_))),
                    ..
                },
            ) => {
                let index = details.member_index.unwrap().0;
                if index == 3 {
                    result.push(Statement::Constant(ConstantDefinition {
                        dst: details.arg.dst,
                        typ: ast::ScalarType::U32,
                        value: ast::ImmediateValue::U64(0),
                    }));
                } else {
                    let sreg_and_type = match numeric_id_defs.special_registers.get(details.arg.src)
                    {
                        Some(reg) => get_sreg_id_scalar_type(numeric_id_defs, reg),
                        None => None,
                    };
                    let (sreg_src, scalar_typ, vector_width) = match sreg_and_type {
                        Some(sreg_and_type) => sreg_and_type,
                        None => {
                            result.push(Statement::LoadVar(details));
                            continue;
                        }
                    };
                    let temp_id = numeric_id_defs.new_non_variable(Some(details.typ.clone()));
                    let real_dst = details.arg.dst;
                    details.arg.dst = temp_id;
                    result.push(Statement::LoadVar(LoadVarDetails {
                        arg: Arg2 {
                            src: sreg_src,
                            dst: temp_id,
                        },
                        typ: ast::Type::Scalar(scalar_typ),
                        member_index: Some((index, Some(vector_width))),
                    }));
                    result.push(Statement::Conversion(ImplicitConversion {
                        src: temp_id,
                        dst: real_dst,
                        from: ast::Type::Scalar(scalar_typ),
                        to: ast::Type::Scalar(ast::ScalarType::U32),
                        kind: ConversionKind::Default,
                        src_sema: ArgumentSemantics::Default,
                        dst_sema: ArgumentSemantics::Default,
                    }));
                }
            }
            s => result.push(s),
        }
    }
    Ok(result)
}

fn get_sreg_id_scalar_type(
    numeric_id_defs: &mut NumericIdResolver,
    sreg: PtxSpecialRegister,
) -> Option<(spirv::Word, ast::ScalarType, u8)> {
    match sreg.normalized_sreg_and_type() {
        Some((normalized_sreg, typ, vec_width)) => Some((
            numeric_id_defs
                .special_registers
                .get_or_add(numeric_id_defs.current_id, normalized_sreg),
            typ,
            vec_width,
        )),
        None => None,
    }
}

fn extract_globals<'input, 'b>(
    sorted_statements: Vec<ExpandedStatement>,
    ptx_impl_imports: &mut HashMap<String, Directive>,
    id_def: &mut NumericIdResolver,
) -> (
    Vec<ExpandedStatement>,
    Vec<ast::Variable<ast::VariableType, spirv::Word>>,
) {
    let mut local = Vec::with_capacity(sorted_statements.len());
    let mut global = Vec::new();
    for statement in sorted_statements {
        match statement {
            Statement::Variable(
                var
                @
                ast::Variable {
                    v_type: ast::VariableType::Shared(_),
                    ..
                },
            )
            | Statement::Variable(
                var
                @
                ast::Variable {
                    v_type: ast::VariableType::Global(_),
                    ..
                },
            ) => global.push(var),
            Statement::Instruction(ast::Instruction::Bfe { typ, arg }) => {
                local.push(to_ptx_impl_bfe_call(id_def, ptx_impl_imports, typ, arg));
            }
            Statement::Instruction(ast::Instruction::Bfi { typ, arg }) => {
                local.push(to_ptx_impl_bfi_call(id_def, ptx_impl_imports, typ, arg));
            }
            Statement::Instruction(ast::Instruction::Atom(
                d
                @
                ast::AtomDetails {
                    inner:
                        ast::AtomInnerDetails::Unsigned {
                            op: ast::AtomUIntOp::Inc,
                            ..
                        },
                    ..
                },
                a,
            )) => {
                local.push(to_ptx_impl_atomic_call(
                    id_def,
                    ptx_impl_imports,
                    d,
                    a,
                    "inc",
                ));
            }
            Statement::Instruction(ast::Instruction::Atom(
                d
                @
                ast::AtomDetails {
                    inner:
                        ast::AtomInnerDetails::Unsigned {
                            op: ast::AtomUIntOp::Dec,
                            ..
                        },
                    ..
                },
                a,
            )) => {
                local.push(to_ptx_impl_atomic_call(
                    id_def,
                    ptx_impl_imports,
                    d,
                    a,
                    "dec",
                ));
            }
            s => local.push(s),
        }
    }
    (local, global)
}

fn normalize_variable_decls(directives: &mut Vec<Directive>) {
    for directive in directives {
        match directive {
            Directive::Method(Function {
                body: Some(func), ..
            }) => {
                func[1..].sort_by_key(|s| match s {
                    Statement::Variable(_) => 0,
                    _ => 1,
                });
            }
            _ => (),
        }
    }
}

fn convert_to_typed_statements(
    func: Vec<UnconditionalStatement>,
    fn_defs: &GlobalFnDeclResolver,
    id_defs: &mut NumericIdResolver,
) -> Result<Vec<TypedStatement>, TranslateError> {
    let mut result = Vec::<TypedStatement>::with_capacity(func.len());
    for s in func {
        match s {
            Statement::Instruction(inst) => match inst {
                ast::Instruction::Call(call) => {
                    // TODO: error out if lengths don't match
                    let fn_def = fn_defs.get_fn_decl(call.func)?;
                    let out_args = to_resolved_fn_args(call.ret_params, &*fn_def.ret_vals);
                    let in_args = to_resolved_fn_args(call.param_list, &*fn_def.params);
                    let (out_params, out_non_params): (Vec<_>, Vec<_>) = out_args
                        .into_iter()
                        .partition(|(_, arg_type)| arg_type.is_param());
                    let normalized_input_args = out_params
                        .into_iter()
                        .map(|(id, typ)| (ast::Operand::Reg(id), typ))
                        .chain(in_args.into_iter())
                        .collect();
                    let resolved_call = ResolvedCall {
                        uniform: call.uniform,
                        ret_params: out_non_params,
                        func: call.func,
                        param_list: normalized_input_args,
                    };
                    let mut visitor = VectorRepackVisitor::new(&mut result, id_defs);
                    let reresolved_call = resolved_call.visit(&mut visitor)?;
                    visitor.func.push(reresolved_call);
                    visitor.func.extend(visitor.post_stmts);
                }
                ast::Instruction::Mov(mut d, ast::Arg2Mov { dst, src }) => {
                    if let Some(src_id) = src.underlying() {
                        let (typ, _) = id_defs.get_typed(*src_id)?;
                        let take_address = match typ {
                            ast::Type::Scalar(_) => false,
                            ast::Type::Vector(_, _) => false,
                            ast::Type::Array(_, _) => true,
                            ast::Type::Pointer(_, _) => true,
                        };
                        d.src_is_address = take_address;
                    }
                    let mut visitor = VectorRepackVisitor::new(&mut result, id_defs);
                    let instruction = Statement::Instruction(
                        ast::Instruction::Mov(d, ast::Arg2Mov { dst, src }).map(&mut visitor)?,
                    );
                    visitor.func.push(instruction);
                    visitor.func.extend(visitor.post_stmts);
                }
                inst => {
                    let mut visitor = VectorRepackVisitor::new(&mut result, id_defs);
                    let instruction = Statement::Instruction(inst.map(&mut visitor)?);
                    visitor.func.push(instruction);
                    visitor.func.extend(visitor.post_stmts);
                }
            },
            Statement::Label(i) => result.push(Statement::Label(i)),
            Statement::Variable(v) => result.push(Statement::Variable(v)),
            Statement::Conditional(c) => result.push(Statement::Conditional(c)),
            _ => return Err(error_unreachable()),
        }
    }
    Ok(result)
}

struct VectorRepackVisitor<'a, 'b> {
    func: &'b mut Vec<TypedStatement>,
    id_def: &'b mut NumericIdResolver<'a>,
    post_stmts: Option<TypedStatement>,
}

impl<'a, 'b> VectorRepackVisitor<'a, 'b> {
    fn new(func: &'b mut Vec<TypedStatement>, id_def: &'b mut NumericIdResolver<'a>) -> Self {
        VectorRepackVisitor {
            func,
            id_def,
            post_stmts: None,
        }
    }

    fn convert_vector(
        &mut self,
        is_dst: bool,
        vector_sema: ArgumentSemantics,
        typ: &ast::Type,
        idx: Vec<spirv::Word>,
    ) -> Result<spirv::Word, TranslateError> {
        // mov.u32 foobar, {a,b};
        let scalar_t = match typ {
            ast::Type::Vector(scalar_t, _) => *scalar_t,
            _ => return Err(TranslateError::MismatchedType),
        };
        let temp_vec = self.id_def.new_non_variable(Some(typ.clone()));
        let statement = Statement::RepackVector(RepackVectorDetails {
            is_extract: is_dst,
            typ: scalar_t,
            packed: temp_vec,
            unpacked: idx,
            vector_sema,
        });
        if is_dst {
            self.post_stmts = Some(statement);
        } else {
            self.func.push(statement);
        }
        Ok(temp_vec)
    }
}

impl<'a, 'b> ArgumentMapVisitor<NormalizedArgParams, TypedArgParams>
    for VectorRepackVisitor<'a, 'b>
{
    fn id(
        &mut self,
        desc: ArgumentDescriptor<spirv::Word>,
        _: Option<&ast::Type>,
    ) -> Result<spirv::Word, TranslateError> {
        Ok(desc.op)
    }

    fn operand(
        &mut self,
        desc: ArgumentDescriptor<ast::Operand<spirv::Word>>,
        typ: &ast::Type,
    ) -> Result<TypedOperand, TranslateError> {
        Ok(match desc.op {
            ast::Operand::Reg(reg) => TypedOperand::Reg(reg),
            ast::Operand::RegOffset(reg, offset) => TypedOperand::RegOffset(reg, offset),
            ast::Operand::Imm(x) => TypedOperand::Imm(x),
            ast::Operand::VecMember(vec, idx) => TypedOperand::VecMember(vec, idx),
            ast::Operand::VecPack(vec) => {
                TypedOperand::Reg(self.convert_vector(desc.is_dst, desc.sema, typ, vec)?)
            }
        })
    }
}

//TODO: share common code between this and to_ptx_impl_bfe_call
fn to_ptx_impl_atomic_call(
    id_defs: &mut NumericIdResolver,
    ptx_impl_imports: &mut HashMap<String, Directive>,
    details: ast::AtomDetails,
    arg: ast::Arg3<ExpandedArgParams>,
    op: &'static str,
) -> ExpandedStatement {
    let semantics = ptx_semantics_name(details.semantics);
    let scope = ptx_scope_name(details.scope);
    let space = ptx_space_name(details.space);
    let fn_name = format!(
        "__zluda_ptx_impl__atom_{}_{}_{}_{}",
        semantics, scope, space, op
    );
    // TODO: extract to a function
    let ptr_space = match details.space {
        ast::AtomSpace::Generic => ast::PointerStateSpace::Generic,
        ast::AtomSpace::Global => ast::PointerStateSpace::Global,
        ast::AtomSpace::Shared => ast::PointerStateSpace::Shared,
    };
    let fn_id = match ptx_impl_imports.entry(fn_name) {
        hash_map::Entry::Vacant(entry) => {
            let fn_id = id_defs.new_non_variable(None);
            let func_decl = ast::MethodDecl::Func::<spirv::Word>(
                vec![ast::FnArgument {
                    align: None,
                    v_type: ast::FnArgumentType::Reg(ast::VariableRegType::Scalar(
                        ast::ScalarType::U32,
                    )),
                    name: id_defs.new_non_variable(None),
                    array_init: Vec::new(),
                }],
                fn_id,
                vec![
                    ast::FnArgument {
                        align: None,
                        v_type: ast::FnArgumentType::Reg(ast::VariableRegType::Pointer(
                            ast::SizedScalarType::U32,
                            ptr_space,
                        )),
                        name: id_defs.new_non_variable(None),
                        array_init: Vec::new(),
                    },
                    ast::FnArgument {
                        align: None,
                        v_type: ast::FnArgumentType::Reg(ast::VariableRegType::Scalar(
                            ast::ScalarType::U32,
                        )),
                        name: id_defs.new_non_variable(None),
                        array_init: Vec::new(),
                    },
                ],
            );
            let spirv_decl = SpirvMethodDecl::new(&func_decl);
            let func = Function {
                func_decl,
                globals: Vec::new(),
                body: None,
                import_as: Some(entry.key().clone()),
                spirv_decl,
            };
            entry.insert(Directive::Method(func));
            fn_id
        }
        hash_map::Entry::Occupied(entry) => match entry.get() {
            Directive::Method(Function {
                func_decl: ast::MethodDecl::Func(_, name, _),
                ..
            }) => *name,
            _ => unreachable!(),
        },
    };
    Statement::Call(ResolvedCall {
        uniform: false,
        func: fn_id,
        ret_params: vec![(
            arg.dst,
            ast::FnArgumentType::Reg(ast::VariableRegType::Scalar(ast::ScalarType::U32)),
        )],
        param_list: vec![
            (
                arg.src1,
                ast::FnArgumentType::Reg(ast::VariableRegType::Pointer(
                    ast::SizedScalarType::U32,
                    ptr_space,
                )),
            ),
            (
                arg.src2,
                ast::FnArgumentType::Reg(ast::VariableRegType::Scalar(ast::ScalarType::U32)),
            ),
        ],
    })
}

fn to_ptx_impl_bfe_call(
    id_defs: &mut NumericIdResolver,
    ptx_impl_imports: &mut HashMap<String, Directive>,
    typ: ast::IntType,
    arg: ast::Arg4<ExpandedArgParams>,
) -> ExpandedStatement {
    let prefix = "__zluda_ptx_impl__";
    let suffix = match typ {
        ast::IntType::U32 => "bfe_u32",
        ast::IntType::U64 => "bfe_u64",
        ast::IntType::S32 => "bfe_s32",
        ast::IntType::S64 => "bfe_s64",
        _ => unreachable!(),
    };
    let fn_name = format!("{}{}", prefix, suffix);
    let fn_id = match ptx_impl_imports.entry(fn_name) {
        hash_map::Entry::Vacant(entry) => {
            let fn_id = id_defs.new_non_variable(None);
            let func_decl = ast::MethodDecl::Func::<spirv::Word>(
                vec![ast::FnArgument {
                    align: None,
                    v_type: ast::FnArgumentType::Reg(ast::VariableRegType::Scalar(typ.into())),
                    name: id_defs.new_non_variable(None),
                    array_init: Vec::new(),
                }],
                fn_id,
                vec![
                    ast::FnArgument {
                        align: None,
                        v_type: ast::FnArgumentType::Reg(ast::VariableRegType::Scalar(typ.into())),
                        name: id_defs.new_non_variable(None),
                        array_init: Vec::new(),
                    },
                    ast::FnArgument {
                        align: None,
                        v_type: ast::FnArgumentType::Reg(ast::VariableRegType::Scalar(
                            ast::ScalarType::U32,
                        )),
                        name: id_defs.new_non_variable(None),
                        array_init: Vec::new(),
                    },
                    ast::FnArgument {
                        align: None,
                        v_type: ast::FnArgumentType::Reg(ast::VariableRegType::Scalar(
                            ast::ScalarType::U32,
                        )),
                        name: id_defs.new_non_variable(None),
                        array_init: Vec::new(),
                    },
                ],
            );
            let spirv_decl = SpirvMethodDecl::new(&func_decl);
            let func = Function {
                func_decl,
                globals: Vec::new(),
                body: None,
                import_as: Some(entry.key().clone()),
                spirv_decl,
            };
            entry.insert(Directive::Method(func));
            fn_id
        }
        hash_map::Entry::Occupied(entry) => match entry.get() {
            Directive::Method(Function {
                func_decl: ast::MethodDecl::Func(_, name, _),
                ..
            }) => *name,
            _ => unreachable!(),
        },
    };
    Statement::Call(ResolvedCall {
        uniform: false,
        func: fn_id,
        ret_params: vec![(
            arg.dst,
            ast::FnArgumentType::Reg(ast::VariableRegType::Scalar(typ.into())),
        )],
        param_list: vec![
            (
                arg.src1,
                ast::FnArgumentType::Reg(ast::VariableRegType::Scalar(typ.into())),
            ),
            (
                arg.src2,
                ast::FnArgumentType::Reg(ast::VariableRegType::Scalar(ast::ScalarType::U32)),
            ),
            (
                arg.src3,
                ast::FnArgumentType::Reg(ast::VariableRegType::Scalar(ast::ScalarType::U32)),
            ),
        ],
    })
}

fn to_ptx_impl_bfi_call(
    id_defs: &mut NumericIdResolver,
    ptx_impl_imports: &mut HashMap<String, Directive>,
    typ: ast::BitType,
    arg: ast::Arg5<ExpandedArgParams>,
) -> ExpandedStatement {
    let prefix = "__zluda_ptx_impl__";
    let suffix = match typ {
        ast::BitType::B32 => "bfi_b32",
        ast::BitType::B64 => "bfi_b64",
        ast::BitType::B8 | ast::BitType::B16 => unreachable!(),
    };
    let fn_name = format!("{}{}", prefix, suffix);
    let fn_id = match ptx_impl_imports.entry(fn_name) {
        hash_map::Entry::Vacant(entry) => {
            let fn_id = id_defs.new_non_variable(None);
            let func_decl = ast::MethodDecl::Func::<spirv::Word>(
                vec![ast::FnArgument {
                    align: None,
                    v_type: ast::FnArgumentType::Reg(ast::VariableRegType::Scalar(typ.into())),
                    name: id_defs.new_non_variable(None),
                    array_init: Vec::new(),
                }],
                fn_id,
                vec![
                    ast::FnArgument {
                        align: None,
                        v_type: ast::FnArgumentType::Reg(ast::VariableRegType::Scalar(typ.into())),
                        name: id_defs.new_non_variable(None),
                        array_init: Vec::new(),
                    },
                    ast::FnArgument {
                        align: None,
                        v_type: ast::FnArgumentType::Reg(ast::VariableRegType::Scalar(typ.into())),
                        name: id_defs.new_non_variable(None),
                        array_init: Vec::new(),
                    },
                    ast::FnArgument {
                        align: None,
                        v_type: ast::FnArgumentType::Reg(ast::VariableRegType::Scalar(
                            ast::ScalarType::U32,
                        )),
                        name: id_defs.new_non_variable(None),
                        array_init: Vec::new(),
                    },
                    ast::FnArgument {
                        align: None,
                        v_type: ast::FnArgumentType::Reg(ast::VariableRegType::Scalar(
                            ast::ScalarType::U32,
                        )),
                        name: id_defs.new_non_variable(None),
                        array_init: Vec::new(),
                    },
                ],
            );
            let spirv_decl = SpirvMethodDecl::new(&func_decl);
            let func = Function {
                func_decl,
                globals: Vec::new(),
                body: None,
                import_as: Some(entry.key().clone()),
                spirv_decl,
            };
            entry.insert(Directive::Method(func));
            fn_id
        }
        hash_map::Entry::Occupied(entry) => match entry.get() {
            Directive::Method(Function {
                func_decl: ast::MethodDecl::Func(_, name, _),
                ..
            }) => *name,
            _ => unreachable!(),
        },
    };
    Statement::Call(ResolvedCall {
        uniform: false,
        func: fn_id,
        ret_params: vec![(
            arg.dst,
            ast::FnArgumentType::Reg(ast::VariableRegType::Scalar(typ.into())),
        )],
        // Note, for some reason PTX and SPIR-V order base&insert arguments differently
        param_list: vec![
            (
                arg.src2,
                ast::FnArgumentType::Reg(ast::VariableRegType::Scalar(typ.into())),
            ),
            (
                arg.src1,
                ast::FnArgumentType::Reg(ast::VariableRegType::Scalar(typ.into())),
            ),
            (
                arg.src3,
                ast::FnArgumentType::Reg(ast::VariableRegType::Scalar(ast::ScalarType::U32)),
            ),
            (
                arg.src4,
                ast::FnArgumentType::Reg(ast::VariableRegType::Scalar(ast::ScalarType::U32)),
            ),
        ],
    })
}

fn to_resolved_fn_args<T>(
    params: Vec<T>,
    params_decl: &[ast::FnArgumentType],
) -> Vec<(T, ast::FnArgumentType)> {
    params
        .into_iter()
        .zip(params_decl.iter())
        .map(|(id, typ)| (id, typ.clone()))
        .collect::<Vec<_>>()
}

fn normalize_labels(
    func: Vec<ExpandedStatement>,
    id_def: &mut NumericIdResolver,
) -> Vec<ExpandedStatement> {
    let mut labels_in_use = HashSet::new();
    for s in func.iter() {
        match s {
            Statement::Instruction(i) => {
                if let Some(target) = i.jump_target() {
                    labels_in_use.insert(target);
                }
            }
            Statement::Conditional(cond) => {
                labels_in_use.insert(cond.if_true);
                labels_in_use.insert(cond.if_false);
            }
            Statement::Call(..)
            | Statement::Variable(..)
            | Statement::LoadVar(..)
            | Statement::StoreVar(..)
            | Statement::RetValue(..)
            | Statement::Conversion(..)
            | Statement::Constant(..)
            | Statement::Label(..)
            | Statement::PtrAccess { .. }
            | Statement::RepackVector(..) => {}
        }
    }
    iter::once(Statement::Label(id_def.new_non_variable(None)))
        .chain(func.into_iter().filter(|s| match s {
            Statement::Label(i) => labels_in_use.contains(i),
            _ => true,
        }))
        .collect::<Vec<_>>()
}

fn normalize_predicates(
    func: Vec<NormalizedStatement>,
    id_def: &mut NumericIdResolver,
) -> Result<Vec<UnconditionalStatement>, TranslateError> {
    let mut result = Vec::with_capacity(func.len());
    for s in func {
        match s {
            Statement::Label(id) => result.push(Statement::Label(id)),
            Statement::Instruction((pred, inst)) => {
                if let Some(pred) = pred {
                    let if_true = id_def.new_non_variable(None);
                    let if_false = id_def.new_non_variable(None);
                    let folded_bra = match &inst {
                        ast::Instruction::Bra(_, arg) => Some(arg.src),
                        _ => None,
                    };
                    let mut branch = BrachCondition {
                        predicate: pred.label,
                        if_true: folded_bra.unwrap_or(if_true),
                        if_false,
                    };
                    if pred.not {
                        std::mem::swap(&mut branch.if_true, &mut branch.if_false);
                    }
                    result.push(Statement::Conditional(branch));
                    if folded_bra.is_none() {
                        result.push(Statement::Label(if_true));
                        result.push(Statement::Instruction(inst));
                    }
                    result.push(Statement::Label(if_false));
                } else {
                    result.push(Statement::Instruction(inst));
                }
            }
            Statement::Variable(var) => result.push(Statement::Variable(var)),
            // Blocks are flattened when resolving ids
            _ => return Err(error_unreachable()),
        }
    }
    Ok(result)
}

fn insert_mem_ssa_statements<'a, 'b>(
    func: Vec<TypedStatement>,
    id_def: &mut NumericIdResolver,
    ast_fn_decl: &'a ast::MethodDecl<'b, spirv::Word>,
    fn_decl: &mut SpirvMethodDecl,
) -> Result<Vec<TypedStatement>, TranslateError> {
    let is_func = match ast_fn_decl {
        ast::MethodDecl::Func(..) => true,
        ast::MethodDecl::Kernel { .. } => false,
    };
    let mut result = Vec::with_capacity(func.len());
    for arg in fn_decl.output.iter() {
        match type_to_variable_type(&arg.v_type, is_func)? {
            Some(var_type) => {
                result.push(Statement::Variable(ast::Variable {
                    align: arg.align,
                    v_type: var_type,
                    name: arg.name,
                    array_init: arg.array_init.clone(),
                }));
            }
            None => return Err(error_unreachable()),
        }
    }
    for spirv_arg in fn_decl.input.iter_mut() {
        match type_to_variable_type(&spirv_arg.v_type, is_func)? {
            Some(var_type) => {
                let typ = spirv_arg.v_type.clone();
                let new_id = id_def.new_non_variable(Some(typ.clone()));
                result.push(Statement::Variable(ast::Variable {
                    align: spirv_arg.align,
                    v_type: var_type,
                    name: spirv_arg.name,
                    array_init: spirv_arg.array_init.clone(),
                }));
                result.push(Statement::StoreVar(StoreVarDetails {
                    arg: ast::Arg2St {
                        src1: spirv_arg.name,
                        src2: new_id,
                    },
                    typ,
                    member_index: None,
                }));
                spirv_arg.name = new_id;
            }
            None => {}
        }
    }
    for s in func {
        match s {
            Statement::Call(call) => {
                insert_mem_ssa_statement_default(id_def, &mut result, call.cast())?
            }
            Statement::Instruction(inst) => match inst {
                ast::Instruction::Ret(d) => {
                    // TODO: handle multiple output args
                    if let &[out_param] = &fn_decl.output.as_slice() {
                        let (typ, _) = id_def.get_typed(out_param.name)?;
                        let new_id = id_def.new_non_variable(Some(typ.clone()));
                        result.push(Statement::LoadVar(LoadVarDetails {
                            arg: ast::Arg2 {
                                dst: new_id,
                                src: out_param.name,
                            },
                            typ: typ.clone(),
                            member_index: None,
                        }));
                        result.push(Statement::RetValue(d, new_id));
                    } else {
                        result.push(Statement::Instruction(ast::Instruction::Ret(d)))
                    }
                }
                inst => insert_mem_ssa_statement_default(id_def, &mut result, inst)?,
            },
            Statement::Conditional(mut bra) => {
                let generated_id =
                    id_def.new_non_variable(Some(ast::Type::Scalar(ast::ScalarType::Pred)));
                result.push(Statement::LoadVar(LoadVarDetails {
                    arg: Arg2 {
                        dst: generated_id,
                        src: bra.predicate,
                    },
                    typ: ast::Type::Scalar(ast::ScalarType::Pred),
                    member_index: None,
                }));
                bra.predicate = generated_id;
                result.push(Statement::Conditional(bra));
            }
            Statement::Conversion(conv) => {
                insert_mem_ssa_statement_default(id_def, &mut result, conv)?
            }
            Statement::PtrAccess(ptr_access) => {
                insert_mem_ssa_statement_default(id_def, &mut result, ptr_access)?
            }
            Statement::RepackVector(repack) => {
                insert_mem_ssa_statement_default(id_def, &mut result, repack)?
            }
            s @ Statement::Variable(_) | s @ Statement::Label(_) => result.push(s),
            _ => return Err(error_unreachable()),
        }
    }
    Ok(result)
}

fn type_to_variable_type(
    t: &ast::Type,
    is_func: bool,
) -> Result<Option<ast::VariableType>, TranslateError> {
    Ok(match t {
        ast::Type::Scalar(typ) => Some(ast::VariableType::Reg(ast::VariableRegType::Scalar(*typ))),
        ast::Type::Vector(typ, len) => Some(ast::VariableType::Reg(ast::VariableRegType::Vector(
            (*typ)
                .try_into()
                .map_err(|_| TranslateError::MismatchedType)?,
            *len,
        ))),
        ast::Type::Array(typ, len) => Some(ast::VariableType::Reg(ast::VariableRegType::Array(
            (*typ)
                .try_into()
                .map_err(|_| TranslateError::MismatchedType)?,
            len.clone(),
        ))),
        ast::Type::Pointer(ast::PointerType::Scalar(scalar_type), space) => {
            if is_func {
                return Ok(None);
            }
            Some(ast::VariableType::Reg(ast::VariableRegType::Pointer(
                scalar_type
                    .clone()
                    .try_into()
                    .map_err(|_| error_unreachable())?,
                (*space).try_into().map_err(|_| error_unreachable())?,
            )))
        }
        ast::Type::Pointer(_, ast::LdStateSpace::Shared) => None,
        _ => return Err(error_unreachable()),
    })
}

trait Visitable<From: ArgParamsEx, To: ArgParamsEx>: Sized {
    fn visit(
        self,
        visitor: &mut impl ArgumentMapVisitor<From, To>,
    ) -> Result<Statement<ast::Instruction<To>, To>, TranslateError>;
}

struct VisitArgumentDescriptor<
    'a,
    Ctor: FnOnce(spirv::Word) -> Statement<ast::Instruction<U>, U>,
    U: ArgParamsEx,
> {
    desc: ArgumentDescriptor<spirv::Word>,
    typ: &'a ast::Type,
    stmt_ctor: Ctor,
}

impl<
        'a,
        Ctor: FnOnce(spirv::Word) -> Statement<ast::Instruction<U>, U>,
        T: ArgParamsEx<Id = spirv::Word>,
        U: ArgParamsEx<Id = spirv::Word>,
    > Visitable<T, U> for VisitArgumentDescriptor<'a, Ctor, U>
{
    fn visit(
        self,
        visitor: &mut impl ArgumentMapVisitor<T, U>,
    ) -> Result<Statement<ast::Instruction<U>, U>, TranslateError> {
        Ok((self.stmt_ctor)(visitor.id(self.desc, Some(self.typ))?))
    }
}

struct InsertMemSSAVisitor<'a, 'input> {
    id_def: &'a mut NumericIdResolver<'input>,
    func: &'a mut Vec<TypedStatement>,
    post_statements: Vec<TypedStatement>,
}

impl<'a, 'input> InsertMemSSAVisitor<'a, 'input> {
    fn symbol(
        &mut self,
        desc: ArgumentDescriptor<(spirv::Word, Option<u8>)>,
        expected_type: Option<&ast::Type>,
    ) -> Result<spirv::Word, TranslateError> {
        let symbol = desc.op.0;
        if expected_type.is_none() {
            return Ok(symbol);
        };
        let (mut var_type, is_variable) = self.id_def.get_typed(symbol)?;
        if !is_variable {
            return Ok(symbol);
        };
        let member_index = match desc.op.1 {
            Some(idx) => {
                let vector_width = match var_type {
                    ast::Type::Vector(scalar_t, width) => {
                        var_type = ast::Type::Scalar(scalar_t);
                        width
                    }
                    _ => return Err(TranslateError::MismatchedType),
                };
                Some((
                    idx,
                    if self.id_def.special_registers.get(symbol).is_some() {
                        Some(vector_width)
                    } else {
                        None
                    },
                ))
            }
            None => None,
        };
        let generated_id = self.id_def.new_non_variable(Some(var_type.clone()));
        if !desc.is_dst {
            self.func.push(Statement::LoadVar(LoadVarDetails {
                arg: Arg2 {
                    dst: generated_id,
                    src: symbol,
                },
                typ: var_type,
                member_index,
            }));
        } else {
            self.post_statements
                .push(Statement::StoreVar(StoreVarDetails {
                    arg: Arg2St {
                        src1: symbol,
                        src2: generated_id,
                    },
                    typ: var_type,
                    member_index: member_index.map(|(idx, _)| idx),
                }));
        }
        Ok(generated_id)
    }
}

impl<'a, 'input> ArgumentMapVisitor<TypedArgParams, TypedArgParams>
    for InsertMemSSAVisitor<'a, 'input>
{
    fn id(
        &mut self,
        desc: ArgumentDescriptor<spirv::Word>,
        typ: Option<&ast::Type>,
    ) -> Result<spirv::Word, TranslateError> {
        self.symbol(desc.new_op((desc.op, None)), typ)
    }

    fn operand(
        &mut self,
        desc: ArgumentDescriptor<TypedOperand>,
        typ: &ast::Type,
    ) -> Result<TypedOperand, TranslateError> {
        Ok(match desc.op {
            TypedOperand::Reg(reg) => {
                TypedOperand::Reg(self.symbol(desc.new_op((reg, None)), Some(typ))?)
            }
            TypedOperand::RegOffset(reg, offset) => {
                TypedOperand::RegOffset(self.symbol(desc.new_op((reg, None)), Some(typ))?, offset)
            }
            op @ TypedOperand::Imm(..) => op,
            TypedOperand::VecMember(symbol, index) => {
                TypedOperand::Reg(self.symbol(desc.new_op((symbol, Some(index))), Some(typ))?)
            }
        })
    }
}

fn insert_mem_ssa_statement_default<'a, 'input, S: Visitable<TypedArgParams, TypedArgParams>>(
    id_def: &'a mut NumericIdResolver<'input>,
    func: &'a mut Vec<TypedStatement>,
    stmt: S,
) -> Result<(), TranslateError> {
    let mut visitor = InsertMemSSAVisitor {
        id_def,
        func,
        post_statements: Vec::new(),
    };
    let new_stmt = stmt.visit(&mut visitor)?;
    visitor.func.push(new_stmt);
    visitor.func.extend(visitor.post_statements);
    Ok(())
}

fn expand_arguments<'a, 'b>(
    func: Vec<TypedStatement>,
    id_def: &'b mut MutableNumericIdResolver<'a>,
) -> Result<Vec<ExpandedStatement>, TranslateError> {
    let mut result = Vec::with_capacity(func.len());
    for s in func {
        match s {
            Statement::Call(call) => {
                let mut visitor = FlattenArguments::new(&mut result, id_def);
                let (new_call, post_stmts) = (call.map(&mut visitor)?, visitor.post_stmts);
                result.push(Statement::Call(new_call));
                result.extend(post_stmts);
            }
            Statement::Instruction(inst) => {
                let mut visitor = FlattenArguments::new(&mut result, id_def);
                let (new_inst, post_stmts) = (inst.map(&mut visitor)?, visitor.post_stmts);
                result.push(Statement::Instruction(new_inst));
                result.extend(post_stmts);
            }
            Statement::Variable(ast::Variable {
                align,
                v_type,
                name,
                array_init,
            }) => result.push(Statement::Variable(ast::Variable {
                align,
                v_type,
                name,
                array_init,
            })),
            Statement::PtrAccess(ptr_access) => {
                let mut visitor = FlattenArguments::new(&mut result, id_def);
                let (new_inst, post_stmts) = (ptr_access.map(&mut visitor)?, visitor.post_stmts);
                result.push(Statement::PtrAccess(new_inst));
                result.extend(post_stmts);
            }
            Statement::RepackVector(repack) => {
                let mut visitor = FlattenArguments::new(&mut result, id_def);
                let (new_inst, post_stmts) = (repack.map(&mut visitor)?, visitor.post_stmts);
                result.push(Statement::RepackVector(new_inst));
                result.extend(post_stmts);
            }
            Statement::Label(id) => result.push(Statement::Label(id)),
            Statement::Conditional(bra) => result.push(Statement::Conditional(bra)),
            Statement::LoadVar(details) => result.push(Statement::LoadVar(details)),
            Statement::StoreVar(details) => result.push(Statement::StoreVar(details)),
            Statement::RetValue(d, id) => result.push(Statement::RetValue(d, id)),
            Statement::Conversion(conv) => result.push(Statement::Conversion(conv)),
            Statement::Constant(_) => return Err(error_unreachable()),
        }
    }
    Ok(result)
}

struct FlattenArguments<'a, 'b> {
    func: &'b mut Vec<ExpandedStatement>,
    id_def: &'b mut MutableNumericIdResolver<'a>,
    post_stmts: Vec<ExpandedStatement>,
}

impl<'a, 'b> FlattenArguments<'a, 'b> {
    fn new(
        func: &'b mut Vec<ExpandedStatement>,
        id_def: &'b mut MutableNumericIdResolver<'a>,
    ) -> Self {
        FlattenArguments {
            func,
            id_def,
            post_stmts: Vec::new(),
        }
    }

    fn reg(
        &mut self,
        desc: ArgumentDescriptor<spirv::Word>,
        _: Option<&ast::Type>,
    ) -> Result<spirv::Word, TranslateError> {
        Ok(desc.op)
    }

    fn reg_offset(
        &mut self,
        desc: ArgumentDescriptor<(spirv::Word, i32)>,
        typ: &ast::Type,
    ) -> Result<spirv::Word, TranslateError> {
        let (reg, offset) = desc.op;
        let add_type;
        match typ {
            ast::Type::Pointer(underlying_type, state_space) => {
                let reg_typ = self.id_def.get_typed(reg)?;
                if let ast::Type::Pointer(_, _) = reg_typ {
                    let id_constant_stmt = self.id_def.new_non_variable(typ.clone());
                    self.func.push(Statement::Constant(ConstantDefinition {
                        dst: id_constant_stmt,
                        typ: ast::ScalarType::S64,
                        value: ast::ImmediateValue::S64(offset as i64),
                    }));
                    let dst = self.id_def.new_non_variable(typ.clone());
                    self.func.push(Statement::PtrAccess(PtrAccess {
                        underlying_type: underlying_type.clone(),
                        state_space: *state_space,
                        dst,
                        ptr_src: reg,
                        offset_src: id_constant_stmt,
                    }));
                    return Ok(dst);
                } else {
                    add_type = self.id_def.get_typed(reg)?;
                }
            }
            _ => {
                add_type = typ.clone();
            }
        };
        let (width, kind) = match add_type {
            ast::Type::Scalar(scalar_t) => {
                let kind = match scalar_t.kind() {
                    kind @ ScalarKind::Bit
                    | kind @ ScalarKind::Unsigned
                    | kind @ ScalarKind::Signed => kind,
                    ScalarKind::Float => return Err(TranslateError::MismatchedType),
                    ScalarKind::Float2 => return Err(TranslateError::MismatchedType),
                    ScalarKind::Pred => return Err(TranslateError::MismatchedType),
                };
                (scalar_t.size_of(), kind)
            }
            _ => return Err(TranslateError::MismatchedType),
        };
        let arith_detail = if kind == ScalarKind::Signed {
            ast::ArithDetails::Signed(ast::ArithSInt {
                typ: ast::SIntType::from_size(width),
                saturate: false,
            })
        } else {
            ast::ArithDetails::Unsigned(ast::UIntType::from_size(width))
        };
        let id_constant_stmt = self.id_def.new_non_variable(add_type.clone());
        let result_id = self.id_def.new_non_variable(add_type);
        // TODO: check for edge cases around min value/max value/wrapping
        if offset < 0 && kind != ScalarKind::Signed {
            self.func.push(Statement::Constant(ConstantDefinition {
                dst: id_constant_stmt,
                typ: ast::ScalarType::from_parts(width, kind),
                value: ast::ImmediateValue::U64(-(offset as i64) as u64),
            }));
            self.func.push(Statement::Instruction(
                ast::Instruction::<ExpandedArgParams>::Sub(
                    arith_detail,
                    ast::Arg3 {
                        dst: result_id,
                        src1: reg,
                        src2: id_constant_stmt,
                    },
                ),
            ));
        } else {
            self.func.push(Statement::Constant(ConstantDefinition {
                dst: id_constant_stmt,
                typ: ast::ScalarType::from_parts(width, kind),
                value: ast::ImmediateValue::S64(offset as i64),
            }));
            self.func.push(Statement::Instruction(
                ast::Instruction::<ExpandedArgParams>::Add(
                    arith_detail,
                    ast::Arg3 {
                        dst: result_id,
                        src1: reg,
                        src2: id_constant_stmt,
                    },
                ),
            ));
        }
        Ok(result_id)
    }

    fn immediate(
        &mut self,
        desc: ArgumentDescriptor<ast::ImmediateValue>,
        typ: &ast::Type,
    ) -> Result<spirv::Word, TranslateError> {
        let scalar_t = if let ast::Type::Scalar(scalar) = typ {
            *scalar
        } else {
            todo!()
        };
        let id = self.id_def.new_non_variable(ast::Type::Scalar(scalar_t));
        self.func.push(Statement::Constant(ConstantDefinition {
            dst: id,
            typ: scalar_t,
            value: desc.op,
        }));
        Ok(id)
    }
}

impl<'a, 'b> ArgumentMapVisitor<TypedArgParams, ExpandedArgParams> for FlattenArguments<'a, 'b> {
    fn id(
        &mut self,
        desc: ArgumentDescriptor<spirv::Word>,
        t: Option<&ast::Type>,
    ) -> Result<spirv::Word, TranslateError> {
        self.reg(desc, t)
    }

    fn operand(
        &mut self,
        desc: ArgumentDescriptor<TypedOperand>,
        typ: &ast::Type,
    ) -> Result<spirv::Word, TranslateError> {
        match desc.op {
            TypedOperand::Reg(r) => self.reg(desc.new_op(r), Some(typ)),
            TypedOperand::Imm(x) => self.immediate(desc.new_op(x), typ),
            TypedOperand::RegOffset(reg, offset) => {
                self.reg_offset(desc.new_op((reg, offset)), typ)
            }
            TypedOperand::VecMember(..) => Err(error_unreachable()),
        }
    }
}

/*
 There are several kinds of implicit conversions in PTX:
 * auto-bitcast: https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#type-information-for-instructions-and-operands
 * special ld/st/cvt conversion rules: https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#operand-size-exceeding-instruction-type-size
   - ld.param: not documented, but for instruction `ld.param.<type> x, [y]`,
     semantics are to first zext/chop/bitcast `y` as needed and then do
     documented special ld/st/cvt conversion rules for destination operands
   - st.param [x] y (used as function return arguments) same rule as above applies
   - generic/global ld: for instruction `ld x, [y]`, y must be of type
     b64/u64/s64, which is bitcast to a pointer, dereferenced and then
     documented special ld/st/cvt conversion rules are applied to dst
   - generic/global st: for instruction `st [x], y`, x must be of type
     b64/u64/s64, which is bitcast to a pointer
*/
fn insert_implicit_conversions(
    func: Vec<ExpandedStatement>,
    id_def: &mut MutableNumericIdResolver,
) -> Result<Vec<ExpandedStatement>, TranslateError> {
    let mut result = Vec::with_capacity(func.len());
    for s in func.into_iter() {
        match s {
            Statement::Call(call) => insert_implicit_conversions_impl(
                &mut result,
                id_def,
                call,
                should_bitcast_wrapper,
                None,
            )?,
            Statement::Instruction(inst) => {
                let mut default_conversion_fn =
                    should_bitcast_wrapper as for<'a> fn(&'a ast::Type, &'a ast::Type, _) -> _;
                let mut state_space = None;
                if let ast::Instruction::Ld(d, _) = &inst {
                    state_space = Some(d.state_space);
                }
                if let ast::Instruction::St(d, _) = &inst {
                    state_space = Some(d.state_space.to_ld_ss());
                }
                if let ast::Instruction::Atom(d, _) = &inst {
                    state_space = Some(d.space.to_ld_ss());
                }
                if let ast::Instruction::AtomCas(d, _) = &inst {
                    state_space = Some(d.space.to_ld_ss());
                }
                if let ast::Instruction::Mov(..) = &inst {
                    default_conversion_fn = should_bitcast_packed;
                }
                insert_implicit_conversions_impl(
                    &mut result,
                    id_def,
                    inst,
                    default_conversion_fn,
                    state_space,
                )?;
            }
            Statement::PtrAccess(PtrAccess {
                underlying_type,
                state_space,
                dst,
                ptr_src,
                offset_src: constant_src,
            }) => {
                let visit_desc = VisitArgumentDescriptor {
                    desc: ArgumentDescriptor {
                        op: ptr_src,
                        is_dst: false,
                        sema: ArgumentSemantics::PhysicalPointer,
                    },
                    typ: &ast::Type::Pointer(underlying_type.clone(), state_space),
                    stmt_ctor: |new_ptr_src| {
                        Statement::PtrAccess(PtrAccess {
                            underlying_type,
                            state_space,
                            dst,
                            ptr_src: new_ptr_src,
                            offset_src: constant_src,
                        })
                    },
                };
                insert_implicit_conversions_impl(
                    &mut result,
                    id_def,
                    visit_desc,
                    bitcast_physical_pointer,
                    Some(state_space),
                )?;
            }
            Statement::RepackVector(repack) => insert_implicit_conversions_impl(
                &mut result,
                id_def,
                repack,
                should_bitcast_wrapper,
                None,
            )?,
            s @ Statement::Conditional(_)
            | s @ Statement::Conversion(_)
            | s @ Statement::Label(_)
            | s @ Statement::Constant(_)
            | s @ Statement::Variable(_)
            | s @ Statement::LoadVar(..)
            | s @ Statement::StoreVar(..)
            | s @ Statement::RetValue(_, _) => result.push(s),
        }
    }
    Ok(result)
}

fn insert_implicit_conversions_impl(
    func: &mut Vec<ExpandedStatement>,
    id_def: &mut MutableNumericIdResolver,
    stmt: impl Visitable<ExpandedArgParams, ExpandedArgParams>,
    default_conversion_fn: for<'a> fn(
        &'a ast::Type,
        &'a ast::Type,
        Option<ast::LdStateSpace>,
    ) -> Result<Option<ConversionKind>, TranslateError>,
    state_space: Option<ast::LdStateSpace>,
) -> Result<(), TranslateError> {
    let mut post_conv = Vec::new();
    let statement = stmt.visit(
        &mut |desc: ArgumentDescriptor<spirv::Word>, typ: Option<&ast::Type>| {
            let instr_type = match typ {
                None => return Ok(desc.op),
                Some(t) => t,
            };
            let operand_type = id_def.get_typed(desc.op)?;
            let mut conversion_fn = default_conversion_fn;
            match desc.sema {
                ArgumentSemantics::Default => {}
                ArgumentSemantics::DefaultRelaxed => {
                    if desc.is_dst {
                        conversion_fn = should_convert_relaxed_dst_wrapper;
                    } else {
                        conversion_fn = should_convert_relaxed_src_wrapper;
                    }
                }
                ArgumentSemantics::PhysicalPointer => {
                    conversion_fn = bitcast_physical_pointer;
                }
                ArgumentSemantics::RegisterPointer => {
                    conversion_fn = bitcast_register_pointer;
                }
                ArgumentSemantics::Address => {
                    conversion_fn = force_bitcast_ptr_to_bit;
                }
            };
            match conversion_fn(&operand_type, instr_type, state_space)? {
                Some(conv_kind) => {
                    let conv_output = if desc.is_dst {
                        &mut post_conv
                    } else {
                        &mut *func
                    };
                    let mut from = instr_type.clone();
                    let mut to = operand_type;
                    let mut src = id_def.new_non_variable(instr_type.clone());
                    let mut dst = desc.op;
                    let result = Ok(src);
                    if !desc.is_dst {
                        mem::swap(&mut src, &mut dst);
                        mem::swap(&mut from, &mut to);
                    }
                    conv_output.push(Statement::Conversion(ImplicitConversion {
                        src,
                        dst,
                        from,
                        to,
                        kind: conv_kind,
                        src_sema: ArgumentSemantics::Default,
                        dst_sema: ArgumentSemantics::Default,
                    }));
                    result
                }
                None => Ok(desc.op),
            }
        },
    )?;
    func.push(statement);
    func.append(&mut post_conv);
    Ok(())
}

fn get_function_type(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    spirv_input: &[ast::Variable<ast::Type, spirv::Word>],
    spirv_output: &[ast::Variable<ast::Type, spirv::Word>],
) -> (spirv::Word, spirv::Word) {
    map.get_or_add_fn(
        builder,
        spirv_input
            .iter()
            .map(|var| SpirvType::from(var.v_type.clone())),
        spirv_output
            .iter()
            .map(|var| SpirvType::from(var.v_type.clone())),
    )
}

fn emit_function_body_ops(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    opencl: spirv::Word,
    func: &[ExpandedStatement],
) -> Result<(), TranslateError> {
    for s in func {
        match s {
            Statement::Label(id) => {
                if builder.block.is_some() {
                    builder.branch(*id)?;
                }
                builder.begin_block(Some(*id))?;
            }
            _ => {
                if builder.block.is_none() && builder.function.is_some() {
                    builder.begin_block(None)?;
                }
            }
        }
        match s {
            Statement::Label(_) => (),
            Statement::Call(call) => {
                let (result_type, result_id) = match &*call.ret_params {
                    [(id, typ)] => (
                        map.get_or_add(builder, SpirvType::from(typ.to_func_type())),
                        Some(*id),
                    ),
                    [] => (map.void(), None),
                    _ => todo!(),
                };
                let arg_list = call
                    .param_list
                    .iter()
                    .map(|(id, _)| *id)
                    .collect::<Vec<_>>();
                builder.function_call(result_type, result_id, call.func, arg_list)?;
            }
            Statement::Variable(var) => {
                emit_variable(builder, map, var)?;
            }
            Statement::Constant(cnst) => {
                let typ_id = map.get_or_add_scalar(builder, cnst.typ);
                match (cnst.typ, cnst.value) {
                    (ast::ScalarType::B8, ast::ImmediateValue::U64(value))
                    | (ast::ScalarType::U8, ast::ImmediateValue::U64(value)) => {
                        builder.constant_u32(typ_id, Some(cnst.dst), value as u8 as u32);
                    }
                    (ast::ScalarType::B16, ast::ImmediateValue::U64(value))
                    | (ast::ScalarType::U16, ast::ImmediateValue::U64(value)) => {
                        builder.constant_u32(typ_id, Some(cnst.dst), value as u16 as u32);
                    }
                    (ast::ScalarType::B32, ast::ImmediateValue::U64(value))
                    | (ast::ScalarType::U32, ast::ImmediateValue::U64(value)) => {
                        builder.constant_u32(typ_id, Some(cnst.dst), value as u32);
                    }
                    (ast::ScalarType::B64, ast::ImmediateValue::U64(value))
                    | (ast::ScalarType::U64, ast::ImmediateValue::U64(value)) => {
                        builder.constant_u64(typ_id, Some(cnst.dst), value);
                    }
                    (ast::ScalarType::S8, ast::ImmediateValue::U64(value)) => {
                        builder.constant_u32(typ_id, Some(cnst.dst), value as i8 as u32);
                    }
                    (ast::ScalarType::S16, ast::ImmediateValue::U64(value)) => {
                        builder.constant_u32(typ_id, Some(cnst.dst), value as i16 as u32);
                    }
                    (ast::ScalarType::S32, ast::ImmediateValue::U64(value)) => {
                        builder.constant_u32(typ_id, Some(cnst.dst), value as i32 as u32);
                    }
                    (ast::ScalarType::S64, ast::ImmediateValue::U64(value)) => {
                        builder.constant_u64(typ_id, Some(cnst.dst), value as i64 as u64);
                    }
                    (ast::ScalarType::B8, ast::ImmediateValue::S64(value))
                    | (ast::ScalarType::U8, ast::ImmediateValue::S64(value)) => {
                        builder.constant_u32(typ_id, Some(cnst.dst), value as u8 as u32);
                    }
                    (ast::ScalarType::B16, ast::ImmediateValue::S64(value))
                    | (ast::ScalarType::U16, ast::ImmediateValue::S64(value)) => {
                        builder.constant_u32(typ_id, Some(cnst.dst), value as u16 as u32);
                    }
                    (ast::ScalarType::B32, ast::ImmediateValue::S64(value))
                    | (ast::ScalarType::U32, ast::ImmediateValue::S64(value)) => {
                        builder.constant_u32(typ_id, Some(cnst.dst), value as u32);
                    }
                    (ast::ScalarType::B64, ast::ImmediateValue::S64(value))
                    | (ast::ScalarType::U64, ast::ImmediateValue::S64(value)) => {
                        builder.constant_u64(typ_id, Some(cnst.dst), value as u64);
                    }
                    (ast::ScalarType::S8, ast::ImmediateValue::S64(value)) => {
                        builder.constant_u32(typ_id, Some(cnst.dst), value as i8 as u32);
                    }
                    (ast::ScalarType::S16, ast::ImmediateValue::S64(value)) => {
                        builder.constant_u32(typ_id, Some(cnst.dst), value as i16 as u32);
                    }
                    (ast::ScalarType::S32, ast::ImmediateValue::S64(value)) => {
                        builder.constant_u32(typ_id, Some(cnst.dst), value as i32 as u32);
                    }
                    (ast::ScalarType::S64, ast::ImmediateValue::S64(value)) => {
                        builder.constant_u64(typ_id, Some(cnst.dst), value as u64);
                    }
                    (ast::ScalarType::F16, ast::ImmediateValue::F32(value)) => {
                        builder.constant_f32(typ_id, Some(cnst.dst), f16::from_f32(value).to_f32());
                    }
                    (ast::ScalarType::F32, ast::ImmediateValue::F32(value)) => {
                        builder.constant_f32(typ_id, Some(cnst.dst), value);
                    }
                    (ast::ScalarType::F64, ast::ImmediateValue::F32(value)) => {
                        builder.constant_f64(typ_id, Some(cnst.dst), value as f64);
                    }
                    (ast::ScalarType::F16, ast::ImmediateValue::F64(value)) => {
                        builder.constant_f32(typ_id, Some(cnst.dst), f16::from_f64(value).to_f32());
                    }
                    (ast::ScalarType::F32, ast::ImmediateValue::F64(value)) => {
                        builder.constant_f32(typ_id, Some(cnst.dst), value as f32);
                    }
                    (ast::ScalarType::F64, ast::ImmediateValue::F64(value)) => {
                        builder.constant_f64(typ_id, Some(cnst.dst), value);
                    }
                    (ast::ScalarType::Pred, ast::ImmediateValue::U64(value)) => {
                        let bool_type = map.get_or_add_scalar(builder, ast::ScalarType::Pred);
                        if value == 0 {
                            builder.constant_false(bool_type, Some(cnst.dst));
                        } else {
                            builder.constant_true(bool_type, Some(cnst.dst));
                        }
                    }
                    (ast::ScalarType::Pred, ast::ImmediateValue::S64(value)) => {
                        let bool_type = map.get_or_add_scalar(builder, ast::ScalarType::Pred);
                        if value == 0 {
                            builder.constant_false(bool_type, Some(cnst.dst));
                        } else {
                            builder.constant_true(bool_type, Some(cnst.dst));
                        }
                    }
                    _ => return Err(TranslateError::MismatchedType),
                }
            }
            Statement::Conversion(cv) => emit_implicit_conversion(builder, map, cv)?,
            Statement::Conditional(bra) => {
                builder.branch_conditional(bra.predicate, bra.if_true, bra.if_false, [])?;
            }
            Statement::Instruction(inst) => match inst {
                ast::Instruction::Abs(d, arg) => emit_abs(builder, map, opencl, d, arg)?,
                ast::Instruction::Call(_) => unreachable!(),
                // SPIR-V does not support marking jumps as guaranteed-converged
                ast::Instruction::Bra(_, arg) => {
                    builder.branch(arg.src)?;
                }
                ast::Instruction::Ld(data, arg) => {
                    if data.qualifier != ast::LdStQualifier::Weak {
                        todo!()
                    }
                    let result_type =
                        map.get_or_add(builder, SpirvType::from(ast::Type::from(data.typ.clone())));
                    builder.load(
                        result_type,
                        Some(arg.dst),
                        arg.src,
                        Some(spirv::MemoryAccess::ALIGNED),
                        [dr::Operand::LiteralInt32(
                            ast::Type::from(data.typ.clone()).size_of() as u32,
                        )],
                    )?;
                }
                ast::Instruction::St(data, arg) => {
                    if data.qualifier != ast::LdStQualifier::Weak {
                        todo!()
                    }
                    builder.store(
                        arg.src1,
                        arg.src2,
                        Some(spirv::MemoryAccess::ALIGNED),
                        [dr::Operand::LiteralInt32(
                            ast::Type::from(data.typ.clone()).size_of() as u32,
                        )],
                    )?;
                }
                // SPIR-V does not support ret as guaranteed-converged
                ast::Instruction::Ret(_) => builder.ret()?,
                ast::Instruction::Mov(d, arg) => {
                    let result_type =
                        map.get_or_add(builder, SpirvType::from(ast::Type::from(d.typ.clone())));
                    builder.copy_object(result_type, Some(arg.dst), arg.src)?;
                }
                ast::Instruction::Mul(mul, arg) => match mul {
                    ast::MulDetails::Signed(ref ctr) => {
                        emit_mul_sint(builder, map, opencl, ctr, arg)?
                    }
                    ast::MulDetails::Unsigned(ref ctr) => {
                        emit_mul_uint(builder, map, opencl, ctr, arg)?
                    }
                    ast::MulDetails::Float(ref ctr) => emit_mul_float(builder, map, ctr, arg)?,
                },
                ast::Instruction::Add(add, arg) => match add {
                    ast::ArithDetails::Signed(ref desc) => {
                        emit_add_int(builder, map, desc.typ.into(), desc.saturate, arg)?
                    }
                    ast::ArithDetails::Unsigned(ref desc) => {
                        emit_add_int(builder, map, (*desc).into(), false, arg)?
                    }
                    ast::ArithDetails::Float(desc) => emit_add_float(builder, map, desc, arg)?,
                },
                ast::Instruction::Setp(setp, arg) => {
                    if arg.dst2.is_some() {
                        todo!()
                    }
                    emit_setp(builder, map, setp, arg)?;
                }
                ast::Instruction::Not(t, a) => {
                    let result_type = map.get_or_add(builder, SpirvType::from(t.to_type()));
                    let result_id = Some(a.dst);
                    let operand = a.src;
                    match t {
                        ast::BooleanType::Pred => {
                            // HACK ALERT
                            // Temporary workaround until IGC gets its shit together
                            // Currently IGC carries two copies of SPIRV-LLVM translator
                            // a new one in /llvm-spirv/ and old one in /IGC/AdaptorOCL/SPIRV/.
                            // Obviously, old and buggy one is used for compiling L0 SPIRV
                            // https://github.com/intel/intel-graphics-compiler/issues/148
                            let type_pred = map.get_or_add_scalar(builder, ast::ScalarType::Pred);
                            let const_true = builder.constant_true(type_pred, None);
                            let const_false = builder.constant_false(type_pred, None);
                            builder.select(result_type, result_id, operand, const_false, const_true)
                        }
                        _ => builder.not(result_type, result_id, operand),
                    }?;
                }
                ast::Instruction::Shl(t, a) => {
                    let full_type = t.to_type();
                    let size_of = full_type.size_of();
                    let result_type = map.get_or_add(builder, SpirvType::from(full_type));
                    let offset_src = insert_shift_hack(builder, map, a.src2, size_of)?;
                    builder.shift_left_logical(result_type, Some(a.dst), a.src1, offset_src)?;
                }
                ast::Instruction::Shr(t, a) => {
                    let full_type = ast::ScalarType::from(*t);
                    let size_of = full_type.size_of();
                    let result_type = map.get_or_add_scalar(builder, full_type);
                    let offset_src = insert_shift_hack(builder, map, a.src2, size_of as usize)?;
                    if t.signed() {
                        builder.shift_right_arithmetic(
                            result_type,
                            Some(a.dst),
                            a.src1,
                            offset_src,
                        )?;
                    } else {
                        builder.shift_right_logical(
                            result_type,
                            Some(a.dst),
                            a.src1,
                            offset_src,
                        )?;
                    }
                }
                ast::Instruction::Cvt(dets, arg) => {
                    emit_cvt(builder, map, opencl, dets, arg)?;
                }
                ast::Instruction::Cvta(_, arg) => {
                    // This would be only meaningful if const/slm/global pointers
                    // had a different format than generic pointers, but they don't pretty much by ptx definition
                    // Honestly, I have no idea why this instruction exists and is emitted by the compiler
                    let result_type = map.get_or_add_scalar(builder, ast::ScalarType::B64);
                    builder.copy_object(result_type, Some(arg.dst), arg.src)?;
                }
                ast::Instruction::SetpBool(_, _) => todo!(),
                ast::Instruction::Mad(mad, arg) => match mad {
                    ast::MulDetails::Signed(ref desc) => {
                        emit_mad_sint(builder, map, opencl, desc, arg)?
                    }
                    ast::MulDetails::Unsigned(ref desc) => {
                        emit_mad_uint(builder, map, opencl, desc, arg)?
                    }
                    ast::MulDetails::Float(desc) => {
                        emit_mad_float(builder, map, opencl, desc, arg)?
                    }
                },
                ast::Instruction::Or(t, a) => {
                    let result_type = map.get_or_add_scalar(builder, ast::ScalarType::from(*t));
                    if *t == ast::BooleanType::Pred {
                        builder.logical_or(result_type, Some(a.dst), a.src1, a.src2)?;
                    } else {
                        builder.bitwise_or(result_type, Some(a.dst), a.src1, a.src2)?;
                    }
                }
                ast::Instruction::Sub(d, arg) => match d {
                    ast::ArithDetails::Signed(desc) => {
                        emit_sub_int(builder, map, desc.typ.into(), desc.saturate, arg)?;
                    }
                    ast::ArithDetails::Unsigned(desc) => {
                        emit_sub_int(builder, map, (*desc).into(), false, arg)?;
                    }
                    ast::ArithDetails::Float(desc) => {
                        emit_sub_float(builder, map, desc, arg)?;
                    }
                },
                ast::Instruction::Min(d, a) => {
                    emit_min(builder, map, opencl, d, a)?;
                }
                ast::Instruction::Max(d, a) => {
                    emit_max(builder, map, opencl, d, a)?;
                }
                ast::Instruction::Rcp(d, a) => {
                    emit_rcp(builder, map, d, a)?;
                }
                ast::Instruction::And(t, a) => {
                    let result_type = map.get_or_add_scalar(builder, ast::ScalarType::from(*t));
                    if *t == ast::BooleanType::Pred {
                        builder.logical_and(result_type, Some(a.dst), a.src1, a.src2)?;
                    } else {
                        builder.bitwise_and(result_type, Some(a.dst), a.src1, a.src2)?;
                    }
                }
                ast::Instruction::Selp(t, a) => {
                    let result_type = map.get_or_add_scalar(builder, ast::ScalarType::from(*t));
                    builder.select(result_type, Some(a.dst), a.src3, a.src1, a.src2)?;
                }
                // TODO: implement named barriers
                ast::Instruction::Bar(d, _) => {
                    let workgroup_scope = map.get_or_add_constant(
                        builder,
                        &ast::Type::Scalar(ast::ScalarType::U32),
                        &vec_repr(spirv::Scope::Workgroup as u32),
                    )?;
                    let barrier_semantics = match d {
                        ast::BarDetails::SyncAligned => map.get_or_add_constant(
                            builder,
                            &ast::Type::Scalar(ast::ScalarType::U32),
                            &vec_repr(
                                spirv::MemorySemantics::CROSS_WORKGROUP_MEMORY
                                    | spirv::MemorySemantics::WORKGROUP_MEMORY
                                    | spirv::MemorySemantics::SEQUENTIALLY_CONSISTENT,
                            ),
                        )?,
                    };
                    builder.control_barrier(workgroup_scope, workgroup_scope, barrier_semantics)?;
                }
                ast::Instruction::Atom(details, arg) => {
                    emit_atom(builder, map, details, arg)?;
                }
                ast::Instruction::AtomCas(details, arg) => {
                    let result_type = map.get_or_add_scalar(builder, details.typ.into());
                    let memory_const = map.get_or_add_constant(
                        builder,
                        &ast::Type::Scalar(ast::ScalarType::U32),
                        &vec_repr(details.scope.to_spirv() as u32),
                    )?;
                    let semantics_const = map.get_or_add_constant(
                        builder,
                        &ast::Type::Scalar(ast::ScalarType::U32),
                        &vec_repr(details.semantics.to_spirv().bits()),
                    )?;
                    builder.atomic_compare_exchange(
                        result_type,
                        Some(arg.dst),
                        arg.src1,
                        memory_const,
                        semantics_const,
                        semantics_const,
                        arg.src3,
                        arg.src2,
                    )?;
                }
                ast::Instruction::Div(details, arg) => match details {
                    ast::DivDetails::Unsigned(t) => {
                        let result_type = map.get_or_add_scalar(builder, (*t).into());
                        builder.u_div(result_type, Some(arg.dst), arg.src1, arg.src2)?;
                    }
                    ast::DivDetails::Signed(t) => {
                        let result_type = map.get_or_add_scalar(builder, (*t).into());
                        builder.s_div(result_type, Some(arg.dst), arg.src1, arg.src2)?;
                    }
                    ast::DivDetails::Float(t) => {
                        let result_type = map.get_or_add_scalar(builder, t.typ.into());
                        builder.f_div(result_type, Some(arg.dst), arg.src1, arg.src2)?;
                        emit_float_div_decoration(builder, arg.dst, t.kind);
                    }
                },
                ast::Instruction::Sqrt(details, a) => {
                    emit_sqrt(builder, map, opencl, details, a)?;
                }
                ast::Instruction::Rsqrt(details, a) => {
                    let result_type = map.get_or_add_scalar(builder, details.typ.into());
                    builder.ext_inst(
                        result_type,
                        Some(a.dst),
                        opencl,
                        spirv::CLOp::native_rsqrt as spirv::Word,
                        &[a.src],
                    )?;
                }
                ast::Instruction::Neg(details, arg) => {
                    let result_type = map.get_or_add_scalar(builder, details.typ);
                    let negate_func = if details.typ.kind() == ScalarKind::Float {
                        dr::Builder::f_negate
                    } else {
                        dr::Builder::s_negate
                    };
                    negate_func(builder, result_type, Some(arg.dst), arg.src)?;
                }
                ast::Instruction::Sin { arg, .. } => {
                    let result_type = map.get_or_add_scalar(builder, ast::ScalarType::F32);
                    builder.ext_inst(
                        result_type,
                        Some(arg.dst),
                        opencl,
                        spirv::CLOp::sin as u32,
                        [arg.src],
                    )?;
                }
                ast::Instruction::Cos { arg, .. } => {
                    let result_type = map.get_or_add_scalar(builder, ast::ScalarType::F32);
                    builder.ext_inst(
                        result_type,
                        Some(arg.dst),
                        opencl,
                        spirv::CLOp::cos as u32,
                        [arg.src],
                    )?;
                }
                ast::Instruction::Lg2 { arg, .. } => {
                    let result_type = map.get_or_add_scalar(builder, ast::ScalarType::F32);
                    builder.ext_inst(
                        result_type,
                        Some(arg.dst),
                        opencl,
                        spirv::CLOp::log2 as u32,
                        [arg.src],
                    )?;
                }
                ast::Instruction::Ex2 { arg, .. } => {
                    let result_type = map.get_or_add_scalar(builder, ast::ScalarType::F32);
                    builder.ext_inst(
                        result_type,
                        Some(arg.dst),
                        opencl,
                        spirv::CLOp::exp2 as u32,
                        [arg.src],
                    )?;
                }
                ast::Instruction::Clz { typ, arg } => {
                    let result_type = map.get_or_add_scalar(builder, (*typ).into());
                    builder.ext_inst(
                        result_type,
                        Some(arg.dst),
                        opencl,
                        spirv::CLOp::clz as u32,
                        [arg.src],
                    )?;
                }
                ast::Instruction::Brev { typ, arg } => {
                    let result_type = map.get_or_add_scalar(builder, (*typ).into());
                    builder.bit_reverse(result_type, Some(arg.dst), arg.src)?;
                }
                ast::Instruction::Popc { typ, arg } => {
                    let result_type = map.get_or_add_scalar(builder, (*typ).into());
                    builder.bit_count(result_type, Some(arg.dst), arg.src)?;
                }
                ast::Instruction::Xor { typ, arg } => {
                    let builder_fn = match typ {
                        ast::BooleanType::Pred => emit_logical_xor_spirv,
                        _ => dr::Builder::bitwise_xor,
                    };
                    let result_type = map.get_or_add_scalar(builder, (*typ).into());
                    builder_fn(builder, result_type, Some(arg.dst), arg.src1, arg.src2)?;
                }
                ast::Instruction::Bfe { .. } => {
                    // Should have beeen replaced with a funciton call earlier
                    return Err(error_unreachable());
                }
                ast::Instruction::Bfi { .. } => {
                    // Should have beeen replaced with a funciton call earlier
                    return Err(error_unreachable());
                }
                ast::Instruction::Rem { typ, arg } => {
                    let builder_fn = if typ.is_signed() {
                        dr::Builder::s_mod
                    } else {
                        dr::Builder::u_mod
                    };
                    let result_type = map.get_or_add_scalar(builder, (*typ).into());
                    builder_fn(builder, result_type, Some(arg.dst), arg.src1, arg.src2)?;
                }
            },
            Statement::LoadVar(details) => {
                emit_load_var(builder, map, details)?;
            }
            Statement::StoreVar(details) => {
                let dst_ptr = match details.member_index {
                    Some(index) => {
                        let result_ptr_type = map.get_or_add(
                            builder,
                            SpirvType::new_pointer(
                                details.typ.clone(),
                                spirv::StorageClass::Function,
                            ),
                        );
                        let index_spirv = map.get_or_add_constant(
                            builder,
                            &ast::Type::Scalar(ast::ScalarType::U32),
                            &vec_repr(index as u32),
                        )?;
                        builder.in_bounds_access_chain(
                            result_ptr_type,
                            None,
                            details.arg.src1,
                            &[index_spirv],
                        )?
                    }
                    None => details.arg.src1,
                };
                builder.store(dst_ptr, details.arg.src2, None, [])?;
            }
            Statement::RetValue(_, id) => {
                builder.ret_value(*id)?;
            }
            Statement::PtrAccess(PtrAccess {
                underlying_type,
                state_space,
                dst,
                ptr_src,
                offset_src,
            }) => {
                let u8_pointer = map.get_or_add(
                    builder,
                    SpirvType::from(ast::Type::Pointer(
                        ast::PointerType::Scalar(ast::ScalarType::U8),
                        *state_space,
                    )),
                );
                let result_type = map.get_or_add(
                    builder,
                    SpirvType::from(ast::Type::Pointer(underlying_type.clone(), *state_space)),
                );
                let ptr_src_u8 = builder.bitcast(u8_pointer, None, *ptr_src)?;
                let temp = builder.in_bounds_ptr_access_chain(
                    u8_pointer,
                    None,
                    ptr_src_u8,
                    *offset_src,
                    &[],
                )?;
                builder.bitcast(result_type, Some(*dst), temp)?;
            }
            Statement::RepackVector(repack) => {
                if repack.is_extract {
                    let scalar_type = map.get_or_add_scalar(builder, repack.typ);
                    for (index, dst_id) in repack.unpacked.iter().enumerate() {
                        builder.composite_extract(
                            scalar_type,
                            Some(*dst_id),
                            repack.packed,
                            &[index as u32],
                        )?;
                    }
                } else {
                    let vector_type = map.get_or_add(
                        builder,
                        SpirvType::Vector(
                            SpirvScalarKey::from(repack.typ),
                            repack.unpacked.len() as u8,
                        ),
                    );
                    let mut temp_vec = builder.undef(vector_type, None);
                    for (index, src_id) in repack.unpacked.iter().enumerate() {
                        temp_vec = builder.composite_insert(
                            vector_type,
                            None,
                            *src_id,
                            temp_vec,
                            &[index as u32],
                        )?;
                    }
                    builder.copy_object(vector_type, Some(repack.packed), temp_vec)?;
                }
            }
        }
    }
    Ok(())
}

// HACK ALERT
// For some reason IGC fails linking if the value and shift size are of different type
fn insert_shift_hack(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    offset_var: spirv::Word,
    size_of: usize,
) -> Result<spirv::Word, TranslateError> {
    let result_type = match size_of {
        2 => map.get_or_add_scalar(builder, ast::ScalarType::B16),
        8 => map.get_or_add_scalar(builder, ast::ScalarType::B64),
        4 => return Ok(offset_var),
        _ => return Err(error_unreachable()),
    };
    Ok(builder.u_convert(result_type, None, offset_var)?)
}

// TODO: check what kind of assembly do we emit
fn emit_logical_xor_spirv(
    builder: &mut dr::Builder,
    result_type: spirv::Word,
    result_id: Option<spirv::Word>,
    op1: spirv::Word,
    op2: spirv::Word,
) -> Result<spirv::Word, dr::Error> {
    let temp_or = builder.logical_or(result_type, None, op1, op2)?;
    let temp_and = builder.logical_and(result_type, None, op1, op2)?;
    let temp_neg = builder.logical_not(result_type, None, temp_and)?;
    builder.logical_and(result_type, result_id, temp_or, temp_neg)
}

fn emit_sqrt(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    opencl: spirv::Word,
    details: &ast::SqrtDetails,
    a: &ast::Arg2<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let result_type = map.get_or_add_scalar(builder, details.typ.into());
    let (ocl_op, rounding) = match details.kind {
        ast::SqrtKind::Approx => (spirv::CLOp::native_sqrt, None),
        ast::SqrtKind::Rounding(rnd) => (spirv::CLOp::sqrt, Some(rnd)),
    };
    builder.ext_inst(
        result_type,
        Some(a.dst),
        opencl,
        ocl_op as spirv::Word,
        &[a.src],
    )?;
    emit_rounding_decoration(builder, a.dst, rounding);
    Ok(())
}

fn emit_float_div_decoration(builder: &mut dr::Builder, dst: spirv::Word, kind: ast::DivFloatKind) {
    match kind {
        ast::DivFloatKind::Approx => {
            builder.decorate(
                dst,
                spirv::Decoration::FPFastMathMode,
                &[dr::Operand::FPFastMathMode(
                    spirv::FPFastMathMode::ALLOW_RECIP,
                )],
            );
        }
        ast::DivFloatKind::Rounding(rnd) => {
            emit_rounding_decoration(builder, dst, Some(rnd));
        }
        ast::DivFloatKind::Full => {}
    }
}

fn emit_atom(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    details: &ast::AtomDetails,
    arg: &ast::Arg3<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let (spirv_op, typ) = match details.inner {
        ast::AtomInnerDetails::Bit { op, typ } => {
            let spirv_op = match op {
                ast::AtomBitOp::And => dr::Builder::atomic_and,
                ast::AtomBitOp::Or => dr::Builder::atomic_or,
                ast::AtomBitOp::Xor => dr::Builder::atomic_xor,
                ast::AtomBitOp::Exchange => dr::Builder::atomic_exchange,
            };
            (spirv_op, ast::ScalarType::from(typ))
        }
        ast::AtomInnerDetails::Unsigned { op, typ } => {
            let spirv_op = match op {
                ast::AtomUIntOp::Add => dr::Builder::atomic_i_add,
                ast::AtomUIntOp::Inc | ast::AtomUIntOp::Dec => {
                    return Err(error_unreachable());
                }
                ast::AtomUIntOp::Min => dr::Builder::atomic_u_min,
                ast::AtomUIntOp::Max => dr::Builder::atomic_u_max,
            };
            (spirv_op, typ.into())
        }
        ast::AtomInnerDetails::Signed { op, typ } => {
            let spirv_op = match op {
                ast::AtomSIntOp::Add => dr::Builder::atomic_i_add,
                ast::AtomSIntOp::Min => dr::Builder::atomic_s_min,
                ast::AtomSIntOp::Max => dr::Builder::atomic_s_max,
            };
            (spirv_op, typ.into())
        }
        // TODO: Hardware is capable of this, implement it through builtin
        ast::AtomInnerDetails::Float { .. } => todo!(),
    };
    let result_type = map.get_or_add_scalar(builder, typ);
    let memory_const = map.get_or_add_constant(
        builder,
        &ast::Type::Scalar(ast::ScalarType::U32),
        &vec_repr(details.scope.to_spirv() as u32),
    )?;
    let semantics_const = map.get_or_add_constant(
        builder,
        &ast::Type::Scalar(ast::ScalarType::U32),
        &vec_repr(details.semantics.to_spirv().bits()),
    )?;
    spirv_op(
        builder,
        result_type,
        Some(arg.dst),
        arg.src1,
        memory_const,
        semantics_const,
        arg.src2,
    )?;
    Ok(())
}

#[derive(Clone)]
struct PtxImplImport {
    out_arg: ast::Type,
    fn_id: u32,
    in_args: Vec<ast::Type>,
}

fn ptx_semantics_name(sema: ast::AtomSemantics) -> &'static str {
    match sema {
        ast::AtomSemantics::Relaxed => "relaxed",
        ast::AtomSemantics::Acquire => "acquire",
        ast::AtomSemantics::Release => "release",
        ast::AtomSemantics::AcquireRelease => "acq_rel",
    }
}

fn ptx_scope_name(scope: ast::MemScope) -> &'static str {
    match scope {
        ast::MemScope::Cta => "cta",
        ast::MemScope::Gpu => "gpu",
        ast::MemScope::Sys => "sys",
    }
}

fn ptx_space_name(space: ast::AtomSpace) -> &'static str {
    match space {
        ast::AtomSpace::Generic => "generic",
        ast::AtomSpace::Global => "global",
        ast::AtomSpace::Shared => "shared",
    }
}

fn emit_mul_float(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    ctr: &ast::ArithFloat,
    arg: &ast::Arg3<ExpandedArgParams>,
) -> Result<(), dr::Error> {
    if ctr.saturate {
        todo!()
    }
    let result_type = map.get_or_add_scalar(builder, ctr.typ.into());
    builder.f_mul(result_type, Some(arg.dst), arg.src1, arg.src2)?;
    emit_rounding_decoration(builder, arg.dst, ctr.rounding);
    Ok(())
}

fn emit_rcp(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    desc: &ast::RcpDetails,
    a: &ast::Arg2<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let (instr_type, constant) = if desc.is_f64 {
        (ast::ScalarType::F64, vec_repr(1.0f64))
    } else {
        (ast::ScalarType::F32, vec_repr(1.0f32))
    };
    let one = map.get_or_add_constant(builder, &ast::Type::Scalar(instr_type), &constant)?;
    let result_type = map.get_or_add_scalar(builder, instr_type);
    builder.f_div(result_type, Some(a.dst), one, a.src)?;
    emit_rounding_decoration(builder, a.dst, desc.rounding);
    builder.decorate(
        a.dst,
        spirv::Decoration::FPFastMathMode,
        &[dr::Operand::FPFastMathMode(
            spirv::FPFastMathMode::ALLOW_RECIP,
        )],
    );
    Ok(())
}

fn vec_repr<T: Copy>(t: T) -> Vec<u8> {
    let mut result = vec![0; mem::size_of::<T>()];
    unsafe { std::ptr::copy_nonoverlapping(&t, result.as_mut_ptr() as *mut _, 1) };
    result
}

fn emit_variable(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    var: &ast::Variable<ast::VariableType, spirv::Word>,
) -> Result<(), TranslateError> {
    let (must_init, st_class) = match var.v_type {
        ast::VariableType::Reg(_) | ast::VariableType::Param(_) | ast::VariableType::Local(_) => {
            (false, spirv::StorageClass::Function)
        }
        ast::VariableType::Global(_) => (true, spirv::StorageClass::CrossWorkgroup),
        ast::VariableType::Shared(_) => (false, spirv::StorageClass::Workgroup),
    };
    let initalizer = if var.array_init.len() > 0 {
        Some(map.get_or_add_constant(
            builder,
            &ast::Type::from(var.v_type.clone()),
            &*var.array_init,
        )?)
    } else if must_init {
        let type_id = map.get_or_add(
            builder,
            SpirvType::from(ast::Type::from(var.v_type.clone())),
        );
        Some(builder.constant_null(type_id, None))
    } else {
        None
    };
    let ptr_type_id = map.get_or_add(
        builder,
        SpirvType::new_pointer(ast::Type::from(var.v_type.clone()), st_class),
    );
    builder.variable(ptr_type_id, Some(var.name), st_class, initalizer);
    if let Some(align) = var.align {
        builder.decorate(
            var.name,
            spirv::Decoration::Alignment,
            &[dr::Operand::LiteralInt32(align)],
        );
    }
    Ok(())
}

fn emit_mad_uint(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    opencl: spirv::Word,
    desc: &ast::MulUInt,
    arg: &ast::Arg4<ExpandedArgParams>,
) -> Result<(), dr::Error> {
    let inst_type = map.get_or_add(builder, SpirvType::from(ast::ScalarType::from(desc.typ)));
    match desc.control {
        ast::MulIntControl::Low => {
            let mul_result = builder.i_mul(inst_type, None, arg.src1, arg.src2)?;
            builder.i_add(inst_type, Some(arg.dst), arg.src3, mul_result)?;
        }
        ast::MulIntControl::High => {
            builder.ext_inst(
                inst_type,
                Some(arg.dst),
                opencl,
                spirv::CLOp::u_mad_hi as spirv::Word,
                [arg.src1, arg.src2, arg.src3],
            )?;
        }
        ast::MulIntControl::Wide => todo!(),
    };
    Ok(())
}

fn emit_mad_sint(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    opencl: spirv::Word,
    desc: &ast::MulSInt,
    arg: &ast::Arg4<ExpandedArgParams>,
) -> Result<(), dr::Error> {
    let inst_type = map.get_or_add(builder, SpirvType::from(ast::ScalarType::from(desc.typ)));
    match desc.control {
        ast::MulIntControl::Low => {
            let mul_result = builder.i_mul(inst_type, None, arg.src1, arg.src2)?;
            builder.i_add(inst_type, Some(arg.dst), arg.src3, mul_result)?;
        }
        ast::MulIntControl::High => {
            builder.ext_inst(
                inst_type,
                Some(arg.dst),
                opencl,
                spirv::CLOp::s_mad_hi as spirv::Word,
                [arg.src1, arg.src2, arg.src3],
            )?;
        }
        ast::MulIntControl::Wide => todo!(),
    };
    Ok(())
}

fn emit_mad_float(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    opencl: spirv::Word,
    desc: &ast::ArithFloat,
    arg: &ast::Arg4<ExpandedArgParams>,
) -> Result<(), dr::Error> {
    let inst_type = map.get_or_add(builder, SpirvType::from(ast::ScalarType::from(desc.typ)));
    builder.ext_inst(
        inst_type,
        Some(arg.dst),
        opencl,
        spirv::CLOp::mad as spirv::Word,
        [arg.src1, arg.src2, arg.src3],
    )?;
    Ok(())
}

fn emit_add_float(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    desc: &ast::ArithFloat,
    arg: &ast::Arg3<ExpandedArgParams>,
) -> Result<(), dr::Error> {
    let inst_type = map.get_or_add(builder, SpirvType::from(ast::ScalarType::from(desc.typ)));
    builder.f_add(inst_type, Some(arg.dst), arg.src1, arg.src2)?;
    emit_rounding_decoration(builder, arg.dst, desc.rounding);
    Ok(())
}

fn emit_sub_float(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    desc: &ast::ArithFloat,
    arg: &ast::Arg3<ExpandedArgParams>,
) -> Result<(), dr::Error> {
    let inst_type = map.get_or_add(builder, SpirvType::from(ast::ScalarType::from(desc.typ)));
    builder.f_sub(inst_type, Some(arg.dst), arg.src1, arg.src2)?;
    emit_rounding_decoration(builder, arg.dst, desc.rounding);
    Ok(())
}

fn emit_min(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    opencl: spirv::Word,
    desc: &ast::MinMaxDetails,
    arg: &ast::Arg3<ExpandedArgParams>,
) -> Result<(), dr::Error> {
    let cl_op = match desc {
        ast::MinMaxDetails::Signed(_) => spirv::CLOp::s_min,
        ast::MinMaxDetails::Unsigned(_) => spirv::CLOp::u_min,
        ast::MinMaxDetails::Float(_) => spirv::CLOp::fmin,
    };
    let inst_type = map.get_or_add(builder, SpirvType::from(desc.get_type()));
    builder.ext_inst(
        inst_type,
        Some(arg.dst),
        opencl,
        cl_op as spirv::Word,
        [arg.src1, arg.src2],
    )?;
    Ok(())
}

fn emit_max(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    opencl: spirv::Word,
    desc: &ast::MinMaxDetails,
    arg: &ast::Arg3<ExpandedArgParams>,
) -> Result<(), dr::Error> {
    let cl_op = match desc {
        ast::MinMaxDetails::Signed(_) => spirv::CLOp::s_max,
        ast::MinMaxDetails::Unsigned(_) => spirv::CLOp::u_max,
        ast::MinMaxDetails::Float(_) => spirv::CLOp::fmax,
    };
    let inst_type = map.get_or_add(builder, SpirvType::from(desc.get_type()));
    builder.ext_inst(
        inst_type,
        Some(arg.dst),
        opencl,
        cl_op as spirv::Word,
        [arg.src1, arg.src2],
    )?;
    Ok(())
}

fn emit_cvt(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    opencl: spirv::Word,
    dets: &ast::CvtDetails,
    arg: &ast::Arg2<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    match dets {
        ast::CvtDetails::FloatFromFloat(desc) => {
            if desc.saturate {
                todo!()
            }
            let dest_t: ast::ScalarType = desc.dst.into();
            let result_type = map.get_or_add(builder, SpirvType::from(dest_t));
            if desc.dst == desc.src {
                match desc.rounding {
                    Some(ast::RoundingMode::NearestEven) => {
                        builder.ext_inst(
                            result_type,
                            Some(arg.dst),
                            opencl,
                            spirv::CLOp::rint as u32,
                            [arg.src],
                        )?;
                    }
                    Some(ast::RoundingMode::Zero) => {
                        builder.ext_inst(
                            result_type,
                            Some(arg.dst),
                            opencl,
                            spirv::CLOp::trunc as u32,
                            [arg.src],
                        )?;
                    }
                    Some(ast::RoundingMode::NegativeInf) => {
                        builder.ext_inst(
                            result_type,
                            Some(arg.dst),
                            opencl,
                            spirv::CLOp::floor as u32,
                            [arg.src],
                        )?;
                    }
                    Some(ast::RoundingMode::PositiveInf) => {
                        builder.ext_inst(
                            result_type,
                            Some(arg.dst),
                            opencl,
                            spirv::CLOp::ceil as u32,
                            [arg.src],
                        )?;
                    }
                    None => {
                        builder.copy_object(result_type, Some(arg.dst), arg.src)?;
                    }
                }
            } else {
                builder.f_convert(result_type, Some(arg.dst), arg.src)?;
                emit_rounding_decoration(builder, arg.dst, desc.rounding);
            }
        }
        ast::CvtDetails::FloatFromInt(desc) => {
            if desc.saturate {
                todo!()
            }
            let dest_t: ast::ScalarType = desc.dst.into();
            let result_type = map.get_or_add(builder, SpirvType::from(dest_t));
            if desc.src.is_signed() {
                builder.convert_s_to_f(result_type, Some(arg.dst), arg.src)?;
            } else {
                builder.convert_u_to_f(result_type, Some(arg.dst), arg.src)?;
            }
            emit_rounding_decoration(builder, arg.dst, desc.rounding);
        }
        ast::CvtDetails::IntFromFloat(desc) => {
            let dest_t: ast::ScalarType = desc.dst.into();
            let result_type = map.get_or_add(builder, SpirvType::from(dest_t));
            if desc.dst.is_signed() {
                builder.convert_f_to_s(result_type, Some(arg.dst), arg.src)?;
            } else {
                builder.convert_f_to_u(result_type, Some(arg.dst), arg.src)?;
            }
            emit_rounding_decoration(builder, arg.dst, desc.rounding);
            emit_saturating_decoration(builder, arg.dst, desc.saturate);
        }
        ast::CvtDetails::IntFromInt(desc) => {
            let dest_t: ast::ScalarType = desc.dst.into();
            let src_t: ast::ScalarType = desc.src.into();
            // first do shortening/widening
            let src = if desc.dst.width() != desc.src.width() {
                let new_dst = if dest_t.kind() == src_t.kind() {
                    arg.dst
                } else {
                    builder.id()
                };
                let cv = ImplicitConversion {
                    src: arg.src,
                    dst: new_dst,
                    from: ast::Type::Scalar(src_t),
                    to: ast::Type::Scalar(ast::ScalarType::from_parts(
                        dest_t.size_of(),
                        src_t.kind(),
                    )),
                    kind: ConversionKind::Default,
                    src_sema: ArgumentSemantics::Default,
                    dst_sema: ArgumentSemantics::Default,
                };
                emit_implicit_conversion(builder, map, &cv)?;
                new_dst
            } else {
                arg.src
            };
            if dest_t.kind() == src_t.kind() {
                return Ok(());
            }
            // now do actual conversion
            let result_type = map.get_or_add(builder, SpirvType::from(dest_t));
            if desc.saturate {
                if desc.dst.is_signed() {
                    builder.sat_convert_u_to_s(result_type, Some(arg.dst), src)?;
                } else {
                    builder.sat_convert_s_to_u(result_type, Some(arg.dst), src)?;
                }
            } else {
                builder.bitcast(result_type, Some(arg.dst), src)?;
            }
        }
    }
    Ok(())
}

fn emit_saturating_decoration(builder: &mut dr::Builder, dst: u32, saturate: bool) {
    if saturate {
        builder.decorate(dst, spirv::Decoration::SaturatedConversion, []);
    }
}

fn emit_rounding_decoration(
    builder: &mut dr::Builder,
    dst: spirv::Word,
    rounding: Option<ast::RoundingMode>,
) {
    if let Some(rounding) = rounding {
        builder.decorate(
            dst,
            spirv::Decoration::FPRoundingMode,
            [rounding.to_spirv()],
        );
    }
}

impl ast::RoundingMode {
    fn to_spirv(self) -> rspirv::dr::Operand {
        let mode = match self {
            ast::RoundingMode::NearestEven => spirv::FPRoundingMode::RTE,
            ast::RoundingMode::Zero => spirv::FPRoundingMode::RTZ,
            ast::RoundingMode::PositiveInf => spirv::FPRoundingMode::RTP,
            ast::RoundingMode::NegativeInf => spirv::FPRoundingMode::RTN,
        };
        rspirv::dr::Operand::FPRoundingMode(mode)
    }
}

fn emit_setp(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    setp: &ast::SetpData,
    arg: &ast::Arg4Setp<ExpandedArgParams>,
) -> Result<(), dr::Error> {
    let result_type = map.get_or_add(builder, SpirvType::Base(SpirvScalarKey::Pred));
    let result_id = Some(arg.dst1);
    let operand_1 = arg.src1;
    let operand_2 = arg.src2;
    match (setp.cmp_op, setp.typ.kind()) {
        (ast::SetpCompareOp::Eq, ScalarKind::Signed)
        | (ast::SetpCompareOp::Eq, ScalarKind::Unsigned)
        | (ast::SetpCompareOp::Eq, ScalarKind::Bit) => {
            builder.i_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::Eq, ScalarKind::Float) => {
            builder.f_ord_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::NotEq, ScalarKind::Signed)
        | (ast::SetpCompareOp::NotEq, ScalarKind::Unsigned)
        | (ast::SetpCompareOp::NotEq, ScalarKind::Bit) => {
            builder.i_not_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::NotEq, ScalarKind::Float) => {
            builder.f_ord_not_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::Less, ScalarKind::Unsigned)
        | (ast::SetpCompareOp::Less, ScalarKind::Bit) => {
            builder.u_less_than(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::Less, ScalarKind::Signed) => {
            builder.s_less_than(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::Less, ScalarKind::Float) => {
            builder.f_ord_less_than(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::LessOrEq, ScalarKind::Unsigned)
        | (ast::SetpCompareOp::LessOrEq, ScalarKind::Bit) => {
            builder.u_less_than_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::LessOrEq, ScalarKind::Signed) => {
            builder.s_less_than_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::LessOrEq, ScalarKind::Float) => {
            builder.f_ord_less_than_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::Greater, ScalarKind::Unsigned)
        | (ast::SetpCompareOp::Greater, ScalarKind::Bit) => {
            builder.u_greater_than(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::Greater, ScalarKind::Signed) => {
            builder.s_greater_than(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::Greater, ScalarKind::Float) => {
            builder.f_ord_greater_than(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::GreaterOrEq, ScalarKind::Unsigned)
        | (ast::SetpCompareOp::GreaterOrEq, ScalarKind::Bit) => {
            builder.u_greater_than_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::GreaterOrEq, ScalarKind::Signed) => {
            builder.s_greater_than_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::GreaterOrEq, ScalarKind::Float) => {
            builder.f_ord_greater_than_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::NanEq, _) => {
            builder.f_unord_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::NanNotEq, _) => {
            builder.f_unord_not_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::NanLess, _) => {
            builder.f_unord_less_than(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::NanLessOrEq, _) => {
            builder.f_unord_less_than_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::NanGreater, _) => {
            builder.f_unord_greater_than(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::NanGreaterOrEq, _) => {
            builder.f_unord_greater_than_equal(result_type, result_id, operand_1, operand_2)
        }
        _ => todo!(),
    }?;
    Ok(())
}

fn emit_mul_sint(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    opencl: spirv::Word,
    desc: &ast::MulSInt,
    arg: &ast::Arg3<ExpandedArgParams>,
) -> Result<(), dr::Error> {
    let instruction_type = ast::ScalarType::from(desc.typ);
    let inst_type = map.get_or_add(builder, SpirvType::from(ast::ScalarType::from(desc.typ)));
    match desc.control {
        ast::MulIntControl::Low => {
            builder.i_mul(inst_type, Some(arg.dst), arg.src1, arg.src2)?;
        }
        ast::MulIntControl::High => {
            builder.ext_inst(
                inst_type,
                Some(arg.dst),
                opencl,
                spirv::CLOp::s_mul_hi as spirv::Word,
                [arg.src1, arg.src2],
            )?;
        }
        ast::MulIntControl::Wide => {
            let mul_ext_type = SpirvType::Struct(vec![
                SpirvScalarKey::from(instruction_type),
                SpirvScalarKey::from(instruction_type),
            ]);
            let mul_ext_type_id = map.get_or_add(builder, mul_ext_type);
            let mul = builder.s_mul_extended(mul_ext_type_id, None, arg.src1, arg.src2)?;
            let instr_width = instruction_type.size_of();
            let instr_kind = instruction_type.kind();
            let dst_type = ast::ScalarType::from_parts(instr_width * 2, instr_kind);
            let dst_type_id = map.get_or_add_scalar(builder, dst_type);
            struct2_bitcast_to_wide(
                builder,
                map,
                SpirvScalarKey::from(instruction_type),
                inst_type,
                arg.dst,
                dst_type_id,
                mul,
            )?;
        }
    }
    Ok(())
}

fn emit_mul_uint(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    opencl: spirv::Word,
    desc: &ast::MulUInt,
    arg: &ast::Arg3<ExpandedArgParams>,
) -> Result<(), dr::Error> {
    let instruction_type = ast::ScalarType::from(desc.typ);
    let inst_type = map.get_or_add(builder, SpirvType::from(ast::ScalarType::from(desc.typ)));
    match desc.control {
        ast::MulIntControl::Low => {
            builder.i_mul(inst_type, Some(arg.dst), arg.src1, arg.src2)?;
        }
        ast::MulIntControl::High => {
            builder.ext_inst(
                inst_type,
                Some(arg.dst),
                opencl,
                spirv::CLOp::u_mul_hi as spirv::Word,
                [arg.src1, arg.src2],
            )?;
        }
        ast::MulIntControl::Wide => {
            let mul_ext_type = SpirvType::Struct(vec![
                SpirvScalarKey::from(instruction_type),
                SpirvScalarKey::from(instruction_type),
            ]);
            let mul_ext_type_id = map.get_or_add(builder, mul_ext_type);
            let mul = builder.u_mul_extended(mul_ext_type_id, None, arg.src1, arg.src2)?;
            let instr_width = instruction_type.size_of();
            let instr_kind = instruction_type.kind();
            let dst_type = ast::ScalarType::from_parts(instr_width * 2, instr_kind);
            let dst_type_id = map.get_or_add_scalar(builder, dst_type);
            struct2_bitcast_to_wide(
                builder,
                map,
                SpirvScalarKey::from(instruction_type),
                inst_type,
                arg.dst,
                dst_type_id,
                mul,
            )?;
        }
    }
    Ok(())
}

// Surprisingly, structs can't be bitcast, so we route everything through a vector
fn struct2_bitcast_to_wide(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    base_type_key: SpirvScalarKey,
    instruction_type: spirv::Word,
    dst: spirv::Word,
    dst_type_id: spirv::Word,
    src: spirv::Word,
) -> Result<(), dr::Error> {
    let low_bits = builder.composite_extract(instruction_type, None, src, [0])?;
    let high_bits = builder.composite_extract(instruction_type, None, src, [1])?;
    let vector_type = map.get_or_add(builder, SpirvType::Vector(base_type_key, 2));
    let vector = builder.composite_construct(vector_type, None, [low_bits, high_bits])?;
    builder.bitcast(dst_type_id, Some(dst), vector)?;
    Ok(())
}

fn emit_abs(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    opencl: spirv::Word,
    d: &ast::AbsDetails,
    arg: &ast::Arg2<ExpandedArgParams>,
) -> Result<(), dr::Error> {
    let scalar_t = ast::ScalarType::from(d.typ);
    let result_type = map.get_or_add(builder, SpirvType::from(scalar_t));
    let cl_abs = if scalar_t.kind() == ScalarKind::Signed {
        spirv::CLOp::s_abs
    } else {
        spirv::CLOp::fabs
    };
    builder.ext_inst(
        result_type,
        Some(arg.dst),
        opencl,
        cl_abs as spirv::Word,
        [arg.src],
    )?;
    Ok(())
}

fn emit_add_int(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    typ: ast::ScalarType,
    saturate: bool,
    arg: &ast::Arg3<ExpandedArgParams>,
) -> Result<(), dr::Error> {
    if saturate {
        todo!()
    }
    let inst_type = map.get_or_add(builder, SpirvType::from(ast::ScalarType::from(typ)));
    builder.i_add(inst_type, Some(arg.dst), arg.src1, arg.src2)?;
    Ok(())
}

fn emit_sub_int(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    typ: ast::ScalarType,
    saturate: bool,
    arg: &ast::Arg3<ExpandedArgParams>,
) -> Result<(), dr::Error> {
    if saturate {
        todo!()
    }
    let inst_type = map.get_or_add(builder, SpirvType::from(ast::ScalarType::from(typ)));
    builder.i_sub(inst_type, Some(arg.dst), arg.src1, arg.src2)?;
    Ok(())
}

fn emit_implicit_conversion(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    cv: &ImplicitConversion,
) -> Result<(), TranslateError> {
    let from_parts = cv.from.to_parts();
    let to_parts = cv.to.to_parts();
    match (from_parts.kind, to_parts.kind, cv.kind) {
        (_, _, ConversionKind::PtrToBit(typ)) => {
            let dst_type = map.get_or_add_scalar(builder, typ.into());
            builder.convert_ptr_to_u(dst_type, Some(cv.dst), cv.src)?;
        }
        (_, _, ConversionKind::BitToPtr(_)) => {
            let dst_type = map.get_or_add(builder, SpirvType::from(cv.to.clone()));
            builder.convert_u_to_ptr(dst_type, Some(cv.dst), cv.src)?;
        }
        (TypeKind::Scalar, TypeKind::Scalar, ConversionKind::Default) => {
            if from_parts.width == to_parts.width {
                let dst_type = map.get_or_add(builder, SpirvType::from(cv.to.clone()));
                if from_parts.scalar_kind != ScalarKind::Float
                    && to_parts.scalar_kind != ScalarKind::Float
                {
                    // It is noop, but another instruction expects result of this conversion
                    builder.copy_object(dst_type, Some(cv.dst), cv.src)?;
                } else {
                    builder.bitcast(dst_type, Some(cv.dst), cv.src)?;
                }
            } else {
                // This block is safe because it's illegal to implictly convert between floating point instructions
                let same_width_bit_type = map.get_or_add(
                    builder,
                    SpirvType::from(ast::Type::from_parts(TypeParts {
                        scalar_kind: ScalarKind::Bit,
                        ..from_parts
                    })),
                );
                let same_width_bit_value = builder.bitcast(same_width_bit_type, None, cv.src)?;
                let wide_bit_type = ast::Type::from_parts(TypeParts {
                    scalar_kind: ScalarKind::Bit,
                    ..to_parts
                });
                let wide_bit_type_spirv =
                    map.get_or_add(builder, SpirvType::from(wide_bit_type.clone()));
                if to_parts.scalar_kind == ScalarKind::Unsigned
                    || to_parts.scalar_kind == ScalarKind::Bit
                {
                    builder.u_convert(wide_bit_type_spirv, Some(cv.dst), same_width_bit_value)?;
                } else {
                    let conversion_fn = if from_parts.scalar_kind == ScalarKind::Signed
                        && to_parts.scalar_kind == ScalarKind::Signed
                    {
                        dr::Builder::s_convert
                    } else {
                        dr::Builder::u_convert
                    };
                    let wide_bit_value =
                        conversion_fn(builder, wide_bit_type_spirv, None, same_width_bit_value)?;
                    emit_implicit_conversion(
                        builder,
                        map,
                        &ImplicitConversion {
                            src: wide_bit_value,
                            dst: cv.dst,
                            from: wide_bit_type,
                            to: cv.to.clone(),
                            kind: ConversionKind::Default,
                            src_sema: cv.src_sema,
                            dst_sema: cv.dst_sema,
                        },
                    )?;
                }
            }
        }
        (TypeKind::Scalar, TypeKind::Scalar, ConversionKind::SignExtend) => todo!(),
        (TypeKind::Vector, TypeKind::Scalar, ConversionKind::Default)
        | (TypeKind::Scalar, TypeKind::Array, ConversionKind::Default)
        | (TypeKind::Array, TypeKind::Scalar, ConversionKind::Default) => {
            let into_type = map.get_or_add(builder, SpirvType::from(cv.to.clone()));
            builder.bitcast(into_type, Some(cv.dst), cv.src)?;
        }
        (_, _, ConversionKind::PtrToPtr { spirv_ptr }) => {
            let result_type = if spirv_ptr {
                map.get_or_add(
                    builder,
                    SpirvType::Pointer(
                        Box::new(SpirvType::from(cv.to.clone())),
                        spirv::StorageClass::Function,
                    ),
                )
            } else {
                map.get_or_add(builder, SpirvType::from(cv.to.clone()))
            };
            builder.bitcast(result_type, Some(cv.dst), cv.src)?;
        }
        _ => unreachable!(),
    }
    Ok(())
}

fn emit_load_var(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    details: &LoadVarDetails,
) -> Result<(), TranslateError> {
    let result_type = map.get_or_add(builder, SpirvType::from(details.typ.clone()));
    match details.member_index {
        Some((index, Some(width))) => {
            let vector_type = match details.typ {
                ast::Type::Scalar(scalar_t) => ast::Type::Vector(scalar_t, width),
                _ => return Err(TranslateError::MismatchedType),
            };
            let vector_type_spirv = map.get_or_add(builder, SpirvType::from(vector_type));
            let vector_temp = builder.load(vector_type_spirv, None, details.arg.src, None, [])?;
            builder.composite_extract(
                result_type,
                Some(details.arg.dst),
                vector_temp,
                &[index as u32],
            )?;
        }
        Some((index, None)) => {
            let result_ptr_type = map.get_or_add(
                builder,
                SpirvType::new_pointer(details.typ.clone(), spirv::StorageClass::Function),
            );
            let index_spirv = map.get_or_add_constant(
                builder,
                &ast::Type::Scalar(ast::ScalarType::U32),
                &vec_repr(index as u32),
            )?;
            let src = builder.in_bounds_access_chain(
                result_ptr_type,
                None,
                details.arg.src,
                &[index_spirv],
            )?;
            builder.load(result_type, Some(details.arg.dst), src, None, [])?;
        }
        None => {
            builder.load(
                result_type,
                Some(details.arg.dst),
                details.arg.src,
                None,
                [],
            )?;
        }
    };
    Ok(())
}

fn normalize_identifiers<'a, 'b>(
    id_defs: &mut FnStringIdResolver<'a, 'b>,
    fn_defs: &GlobalFnDeclResolver<'a, 'b>,
    func: Vec<ast::Statement<ast::ParsedArgParams<'a>>>,
) -> Result<Vec<NormalizedStatement>, TranslateError> {
    for s in func.iter() {
        match s {
            ast::Statement::Label(id) => {
                id_defs.add_def(*id, None, false);
            }
            _ => (),
        }
    }
    let mut result = Vec::new();
    for s in func {
        expand_map_variables(id_defs, fn_defs, &mut result, s)?;
    }
    Ok(result)
}

fn expand_map_variables<'a, 'b>(
    id_defs: &mut FnStringIdResolver<'a, 'b>,
    fn_defs: &GlobalFnDeclResolver<'a, 'b>,
    result: &mut Vec<NormalizedStatement>,
    s: ast::Statement<ast::ParsedArgParams<'a>>,
) -> Result<(), TranslateError> {
    match s {
        ast::Statement::Block(block) => {
            id_defs.start_block();
            for s in block {
                expand_map_variables(id_defs, fn_defs, result, s)?;
            }
            id_defs.end_block();
        }
        ast::Statement::Label(name) => result.push(Statement::Label(id_defs.get_id(name)?)),
        ast::Statement::Instruction(p, i) => result.push(Statement::Instruction((
            p.map(|p| p.map_variable(&mut |id| id_defs.get_id(id)))
                .transpose()?,
            i.map_variable(&mut |id| id_defs.get_id(id))?,
        ))),
        ast::Statement::Variable(var) => {
            let mut var_type = ast::Type::from(var.var.v_type.clone());
            let mut is_variable = false;
            var_type = match var.var.v_type {
                ast::VariableType::Reg(_) => {
                    is_variable = true;
                    var_type
                }
                ast::VariableType::Shared(_) => {
                    // If it's a pointer it will be translated to a method parameter later
                    if let ast::Type::Pointer(..) = var_type {
                        is_variable = true;
                        var_type
                    } else {
                        var_type.param_pointer_to(ast::LdStateSpace::Shared)?
                    }
                }
                ast::VariableType::Global(_) => {
                    var_type.param_pointer_to(ast::LdStateSpace::Global)?
                }
                ast::VariableType::Param(_) => {
                    var_type.param_pointer_to(ast::LdStateSpace::Param)?
                }
                ast::VariableType::Local(_) => {
                    var_type.param_pointer_to(ast::LdStateSpace::Local)?
                }
            };
            match var.count {
                Some(count) => {
                    for new_id in id_defs.add_defs(var.var.name, count, var_type, is_variable) {
                        result.push(Statement::Variable(ast::Variable {
                            align: var.var.align,
                            v_type: var.var.v_type.clone(),
                            name: new_id,
                            array_init: var.var.array_init.clone(),
                        }))
                    }
                }
                None => {
                    let new_id = id_defs.add_def(var.var.name, Some(var_type), is_variable);
                    result.push(Statement::Variable(ast::Variable {
                        align: var.var.align,
                        v_type: var.var.v_type.clone(),
                        name: new_id,
                        array_init: var.var.array_init,
                    }));
                }
            }
        }
    };
    Ok(())
}

// TODO: detect more patterns (mov, call via reg, call via param)
// TODO: don't convert to ptr if the register is not ultimately used for ld/st
// TODO: once insert_mem_ssa_statements is moved to later, move this pass after
//       argument expansion
// TODO: propagate through calls?
fn convert_to_stateful_memory_access<'a>(
    func_args: &mut SpirvMethodDecl,
    func_body: Vec<TypedStatement>,
    id_defs: &mut NumericIdResolver<'a>,
) -> Result<Vec<TypedStatement>, TranslateError> {
    let func_args_64bit = func_args
        .input
        .iter()
        .filter_map(|arg| match arg.v_type {
            ast::Type::Scalar(ast::ScalarType::U64)
            | ast::Type::Scalar(ast::ScalarType::B64)
            | ast::Type::Scalar(ast::ScalarType::S64) => Some(arg.name),
            _ => None,
        })
        .collect::<HashSet<_>>();
    let mut stateful_markers = Vec::new();
    let mut stateful_init_reg = MultiHashMap::new();
    for statement in func_body.iter() {
        match statement {
            Statement::Instruction(ast::Instruction::Cvta(
                ast::CvtaDetails {
                    to: ast::CvtaStateSpace::Global,
                    size: ast::CvtaSize::U64,
                    from: ast::CvtaStateSpace::Generic,
                },
                arg,
            )) => {
                if let (TypedOperand::Reg(dst), Some(src)) =
                    (arg.dst, arg.src.upcast().underlying())
                {
                    if is_64_bit_integer(id_defs, *src) && is_64_bit_integer(id_defs, dst) {
                        stateful_markers.push((dst, *src));
                    }
                }
            }
            Statement::Instruction(ast::Instruction::Ld(
                ast::LdDetails {
                    state_space: ast::LdStateSpace::Param,
                    typ: ast::LdStType::Scalar(ast::LdStScalarType::U64),
                    ..
                },
                arg,
            ))
            | Statement::Instruction(ast::Instruction::Ld(
                ast::LdDetails {
                    state_space: ast::LdStateSpace::Param,
                    typ: ast::LdStType::Scalar(ast::LdStScalarType::S64),
                    ..
                },
                arg,
            ))
            | Statement::Instruction(ast::Instruction::Ld(
                ast::LdDetails {
                    state_space: ast::LdStateSpace::Param,
                    typ: ast::LdStType::Scalar(ast::LdStScalarType::B64),
                    ..
                },
                arg,
            )) => {
                if let (TypedOperand::Reg(dst), Some(src)) =
                    (&arg.dst, arg.src.upcast().underlying())
                {
                    if func_args_64bit.contains(src) {
                        multi_hash_map_append(&mut stateful_init_reg, *dst, *src);
                    }
                }
            }
            _ => {}
        }
    }
    let mut func_args_ptr = HashSet::new();
    let mut regs_ptr_current = HashSet::new();
    for (dst, src) in stateful_markers {
        if let Some(func_args) = stateful_init_reg.get(&src) {
            for a in func_args {
                func_args_ptr.insert(*a);
                regs_ptr_current.insert(src);
                regs_ptr_current.insert(dst);
            }
        }
    }
    // BTreeSet here to have a stable order of iteration,
    // unfortunately our tests rely on it
    let mut regs_ptr_seen = BTreeSet::new();
    while regs_ptr_current.len() > 0 {
        let mut regs_ptr_new = HashSet::new();
        for statement in func_body.iter() {
            match statement {
                Statement::Instruction(ast::Instruction::Add(
                    ast::ArithDetails::Unsigned(ast::UIntType::U64),
                    arg,
                ))
                | Statement::Instruction(ast::Instruction::Add(
                    ast::ArithDetails::Signed(ast::ArithSInt {
                        typ: ast::SIntType::S64,
                        saturate: false,
                    }),
                    arg,
                ))
                | Statement::Instruction(ast::Instruction::Sub(
                    ast::ArithDetails::Unsigned(ast::UIntType::U64),
                    arg,
                ))
                | Statement::Instruction(ast::Instruction::Sub(
                    ast::ArithDetails::Signed(ast::ArithSInt {
                        typ: ast::SIntType::S64,
                        saturate: false,
                    }),
                    arg,
                )) => {
                    if let (TypedOperand::Reg(dst), Some(src1)) =
                        (arg.dst, arg.src1.upcast().underlying())
                    {
                        if regs_ptr_current.contains(src1) && !regs_ptr_seen.contains(src1) {
                            regs_ptr_new.insert(dst);
                        }
                    } else if let (TypedOperand::Reg(dst), Some(src2)) =
                        (arg.dst, arg.src2.upcast().underlying())
                    {
                        if regs_ptr_current.contains(src2) && !regs_ptr_seen.contains(src2) {
                            regs_ptr_new.insert(dst);
                        }
                    }
                }
                _ => {}
            }
        }
        for id in regs_ptr_current {
            regs_ptr_seen.insert(id);
        }
        regs_ptr_current = regs_ptr_new;
    }
    drop(regs_ptr_current);
    let mut remapped_ids = HashMap::new();
    let mut result = Vec::with_capacity(regs_ptr_seen.len() + func_body.len());
    for reg in regs_ptr_seen {
        let new_id = id_defs.new_variable(ast::Type::Pointer(
            ast::PointerType::Scalar(ast::ScalarType::U8),
            ast::LdStateSpace::Global,
        ));
        result.push(Statement::Variable(ast::Variable {
            align: None,
            name: new_id,
            array_init: Vec::new(),
            v_type: ast::VariableType::Reg(ast::VariableRegType::Pointer(
                ast::SizedScalarType::U8,
                ast::PointerStateSpace::Global,
            )),
        }));
        remapped_ids.insert(reg, new_id);
    }
    for statement in func_body {
        match statement {
            l @ Statement::Label(_) => result.push(l),
            c @ Statement::Conditional(_) => result.push(c),
            Statement::Variable(var) => {
                if !remapped_ids.contains_key(&var.name) {
                    result.push(Statement::Variable(var));
                }
            }
            Statement::Instruction(ast::Instruction::Add(
                ast::ArithDetails::Unsigned(ast::UIntType::U64),
                arg,
            ))
            | Statement::Instruction(ast::Instruction::Add(
                ast::ArithDetails::Signed(ast::ArithSInt {
                    typ: ast::SIntType::S64,
                    saturate: false,
                }),
                arg,
            )) if is_add_ptr_direct(&remapped_ids, &arg) => {
                let (ptr, offset) = match arg.src1.upcast().underlying() {
                    Some(src1) if remapped_ids.contains_key(src1) => {
                        (remapped_ids.get(src1).unwrap(), arg.src2)
                    }
                    Some(src2) if remapped_ids.contains_key(src2) => {
                        (remapped_ids.get(src2).unwrap(), arg.src1)
                    }
                    _ => return Err(error_unreachable()),
                };
                let dst = arg.dst.upcast().unwrap_reg()?;
                result.push(Statement::PtrAccess(PtrAccess {
                    underlying_type: ast::PointerType::Scalar(ast::ScalarType::U8),
                    state_space: ast::LdStateSpace::Global,
                    dst: *remapped_ids.get(&dst).unwrap(),
                    ptr_src: *ptr,
                    offset_src: offset,
                }))
            }
            Statement::Instruction(ast::Instruction::Sub(
                ast::ArithDetails::Unsigned(ast::UIntType::U64),
                arg,
            ))
            | Statement::Instruction(ast::Instruction::Sub(
                ast::ArithDetails::Signed(ast::ArithSInt {
                    typ: ast::SIntType::S64,
                    saturate: false,
                }),
                arg,
            )) if is_add_ptr_direct(&remapped_ids, &arg) => {
                let (ptr, offset) = match arg.src1.upcast().underlying() {
                    Some(src1) if remapped_ids.contains_key(src1) => {
                        (remapped_ids.get(src1).unwrap(), arg.src2)
                    }
                    Some(src2) if remapped_ids.contains_key(src2) => {
                        (remapped_ids.get(src2).unwrap(), arg.src1)
                    }
                    _ => return Err(error_unreachable()),
                };
                let offset_neg =
                    id_defs.new_non_variable(Some(ast::Type::Scalar(ast::ScalarType::S64)));
                result.push(Statement::Instruction(ast::Instruction::Neg(
                    ast::NegDetails {
                        typ: ast::ScalarType::S64,
                        flush_to_zero: None,
                    },
                    ast::Arg2 {
                        src: offset,
                        dst: TypedOperand::Reg(offset_neg),
                    },
                )));
                let dst = arg.dst.upcast().unwrap_reg()?;
                result.push(Statement::PtrAccess(PtrAccess {
                    underlying_type: ast::PointerType::Scalar(ast::ScalarType::U8),
                    state_space: ast::LdStateSpace::Global,
                    dst: *remapped_ids.get(&dst).unwrap(),
                    ptr_src: *ptr,
                    offset_src: TypedOperand::Reg(offset_neg),
                }))
            }
            Statement::Instruction(inst) => {
                let mut post_statements = Vec::new();
                let new_statement = inst.visit(
                    &mut |arg_desc: ArgumentDescriptor<spirv::Word>,
                          expected_type: Option<&ast::Type>| {
                        convert_to_stateful_memory_access_postprocess(
                            id_defs,
                            &remapped_ids,
                            &func_args_ptr,
                            &mut result,
                            &mut post_statements,
                            arg_desc,
                            expected_type,
                        )
                    },
                )?;
                result.push(new_statement);
                result.extend(post_statements);
            }
            Statement::Call(call) => {
                let mut post_statements = Vec::new();
                let new_statement = call.visit(
                    &mut |arg_desc: ArgumentDescriptor<spirv::Word>,
                          expected_type: Option<&ast::Type>| {
                        convert_to_stateful_memory_access_postprocess(
                            id_defs,
                            &remapped_ids,
                            &func_args_ptr,
                            &mut result,
                            &mut post_statements,
                            arg_desc,
                            expected_type,
                        )
                    },
                )?;
                result.push(new_statement);
                result.extend(post_statements);
            }
            Statement::RepackVector(pack) => {
                let mut post_statements = Vec::new();
                let new_statement = pack.visit(
                    &mut |arg_desc: ArgumentDescriptor<spirv::Word>,
                          expected_type: Option<&ast::Type>| {
                        convert_to_stateful_memory_access_postprocess(
                            id_defs,
                            &remapped_ids,
                            &func_args_ptr,
                            &mut result,
                            &mut post_statements,
                            arg_desc,
                            expected_type,
                        )
                    },
                )?;
                result.push(new_statement);
                result.extend(post_statements);
            }
            _ => return Err(error_unreachable()),
        }
    }
    for arg in func_args.input.iter_mut() {
        if func_args_ptr.contains(&arg.name) {
            arg.v_type = ast::Type::Pointer(
                ast::PointerType::Scalar(ast::ScalarType::U8),
                ast::LdStateSpace::Global,
            );
        }
    }
    Ok(result)
}

fn convert_to_stateful_memory_access_postprocess(
    id_defs: &mut NumericIdResolver,
    remapped_ids: &HashMap<spirv::Word, spirv::Word>,
    func_args_ptr: &HashSet<spirv::Word>,
    result: &mut Vec<TypedStatement>,
    post_statements: &mut Vec<TypedStatement>,
    arg_desc: ArgumentDescriptor<spirv::Word>,
    expected_type: Option<&ast::Type>,
) -> Result<spirv::Word, TranslateError> {
    Ok(match remapped_ids.get(&arg_desc.op) {
        Some(new_id) => {
            // We skip conversion here to trigger PtrAcces in a later pass
            let old_type = match expected_type {
                Some(ast::Type::Pointer(_, ast::LdStateSpace::Global)) => return Ok(*new_id),
                _ => id_defs.get_typed(arg_desc.op)?.0,
            };
            let old_type_clone = old_type.clone();
            let converting_id = id_defs.new_non_variable(Some(old_type_clone));
            if arg_desc.is_dst {
                post_statements.push(Statement::Conversion(ImplicitConversion {
                    src: converting_id,
                    dst: *new_id,
                    from: old_type,
                    to: ast::Type::Pointer(
                        ast::PointerType::Scalar(ast::ScalarType::U8),
                        ast::LdStateSpace::Global,
                    ),
                    kind: ConversionKind::BitToPtr(ast::LdStateSpace::Global),
                    src_sema: ArgumentSemantics::Default,
                    dst_sema: arg_desc.sema,
                }));
                converting_id
            } else {
                result.push(Statement::Conversion(ImplicitConversion {
                    src: *new_id,
                    dst: converting_id,
                    from: ast::Type::Pointer(
                        ast::PointerType::Scalar(ast::ScalarType::U8),
                        ast::LdStateSpace::Global,
                    ),
                    to: old_type,
                    kind: ConversionKind::PtrToBit(ast::UIntType::U64),
                    src_sema: arg_desc.sema,
                    dst_sema: ArgumentSemantics::Default,
                }));
                converting_id
            }
        }
        None => match func_args_ptr.get(&arg_desc.op) {
            Some(new_id) => {
                if arg_desc.is_dst {
                    return Err(error_unreachable());
                }
                // We skip conversion here to trigger PtrAcces in a later pass
                let old_type = match expected_type {
                    Some(ast::Type::Pointer(_, ast::LdStateSpace::Global)) => return Ok(*new_id),
                    _ => id_defs.get_typed(arg_desc.op)?.0,
                };
                let old_type_clone = old_type.clone();
                let converting_id = id_defs.new_non_variable(Some(old_type));
                result.push(Statement::Conversion(ImplicitConversion {
                    src: *new_id,
                    dst: converting_id,
                    from: ast::Type::Pointer(
                        ast::PointerType::Pointer(ast::ScalarType::U8, ast::LdStateSpace::Global),
                        ast::LdStateSpace::Param,
                    ),
                    to: old_type_clone,
                    kind: ConversionKind::PtrToPtr { spirv_ptr: false },
                    src_sema: arg_desc.sema,
                    dst_sema: ArgumentSemantics::Default,
                }));
                converting_id
            }
            None => arg_desc.op,
        },
    })
}

fn is_add_ptr_direct(remapped_ids: &HashMap<u32, u32>, arg: &ast::Arg3<TypedArgParams>) -> bool {
    match arg.dst {
        TypedOperand::Imm(..) | TypedOperand::RegOffset(..) | TypedOperand::VecMember(..) => {
            return false
        }
        TypedOperand::Reg(dst) => {
            if !remapped_ids.contains_key(&dst) {
                return false;
            }
            match arg.src1.upcast().underlying() {
                Some(src1) if remapped_ids.contains_key(src1) => true,
                Some(src2) if remapped_ids.contains_key(src2) => true,
                _ => false,
            }
        }
    }
}

fn is_64_bit_integer(id_defs: &NumericIdResolver, id: spirv::Word) -> bool {
    match id_defs.get_typed(id) {
        Ok((ast::Type::Scalar(ast::ScalarType::U64), _))
        | Ok((ast::Type::Scalar(ast::ScalarType::S64), _))
        | Ok((ast::Type::Scalar(ast::ScalarType::B64), _)) => true,
        _ => false,
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
enum PtxSpecialRegister {
    Tid,
    Tid64,
    Ntid,
    Ntid64,
    Ctaid,
    Ctaid64,
    Nctaid,
    Nctaid64,
}

impl PtxSpecialRegister {
    fn try_parse(s: &str) -> Option<Self> {
        match s {
            "%tid" => Some(Self::Tid),
            "%ntid" => Some(Self::Ntid),
            "%ctaid" => Some(Self::Ctaid),
            "%nctaid" => Some(Self::Nctaid),
            _ => None,
        }
    }

    fn get_type(self) -> ast::Type {
        match self {
            PtxSpecialRegister::Tid => ast::Type::Vector(ast::ScalarType::U32, 4),
            PtxSpecialRegister::Tid64 => ast::Type::Vector(ast::ScalarType::U64, 3),
            PtxSpecialRegister::Ntid => ast::Type::Vector(ast::ScalarType::U32, 4),
            PtxSpecialRegister::Ntid64 => ast::Type::Vector(ast::ScalarType::U64, 3),
            PtxSpecialRegister::Ctaid => ast::Type::Vector(ast::ScalarType::U32, 4),
            PtxSpecialRegister::Ctaid64 => ast::Type::Vector(ast::ScalarType::U64, 3),
            PtxSpecialRegister::Nctaid => ast::Type::Vector(ast::ScalarType::U32, 4),
            PtxSpecialRegister::Nctaid64 => ast::Type::Vector(ast::ScalarType::U64, 3),
        }
    }

    fn get_builtin(self) -> spirv::BuiltIn {
        match self {
            PtxSpecialRegister::Tid | PtxSpecialRegister::Tid64 => {
                spirv::BuiltIn::LocalInvocationId
            }
            PtxSpecialRegister::Ntid | PtxSpecialRegister::Ntid64 => spirv::BuiltIn::WorkgroupSize,
            PtxSpecialRegister::Ctaid | PtxSpecialRegister::Ctaid64 => spirv::BuiltIn::WorkgroupId,
            PtxSpecialRegister::Nctaid | PtxSpecialRegister::Nctaid64 => {
                spirv::BuiltIn::NumWorkgroups
            }
        }
    }

    fn normalized_sreg_and_type(self) -> Option<(PtxSpecialRegister, ast::ScalarType, u8)> {
        match self {
            PtxSpecialRegister::Tid => Some((PtxSpecialRegister::Tid64, ast::ScalarType::U64, 3)),
            PtxSpecialRegister::Ntid => Some((PtxSpecialRegister::Ntid64, ast::ScalarType::U64, 3)),
            PtxSpecialRegister::Ctaid => {
                Some((PtxSpecialRegister::Ctaid64, ast::ScalarType::U64, 3))
            }
            PtxSpecialRegister::Nctaid => {
                Some((PtxSpecialRegister::Nctaid64, ast::ScalarType::U64, 3))
            }
            PtxSpecialRegister::Tid64
            | PtxSpecialRegister::Ntid64
            | PtxSpecialRegister::Ctaid64
            | PtxSpecialRegister::Nctaid64 => None,
        }
    }
}

struct SpecialRegistersMap {
    reg_to_id: HashMap<PtxSpecialRegister, spirv::Word>,
    id_to_reg: HashMap<spirv::Word, PtxSpecialRegister>,
}

impl SpecialRegistersMap {
    fn new() -> Self {
        SpecialRegistersMap {
            reg_to_id: HashMap::new(),
            id_to_reg: HashMap::new(),
        }
    }

    fn builtins<'a>(&'a self) -> impl Iterator<Item = (PtxSpecialRegister, spirv::Word)> + 'a {
        self.reg_to_id.iter().filter_map(|(sreg, id)| {
            if sreg.normalized_sreg_and_type().is_none() {
                Some((*sreg, *id))
            } else {
                None
            }
        })
    }

    fn interface(&self) -> Vec<spirv::Word> {
        self.reg_to_id
            .iter()
            .filter_map(|(sreg, id)| {
                if sreg.normalized_sreg_and_type().is_none() {
                    Some(*id)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    fn get(&self, id: spirv::Word) -> Option<PtxSpecialRegister> {
        self.id_to_reg.get(&id).copied()
    }

    fn get_or_add(&mut self, current_id: &mut spirv::Word, reg: PtxSpecialRegister) -> spirv::Word {
        match self.reg_to_id.entry(reg) {
            hash_map::Entry::Occupied(e) => *e.get(),
            hash_map::Entry::Vacant(e) => {
                let numeric_id = *current_id;
                *current_id += 1;
                e.insert(numeric_id);
                self.id_to_reg.insert(numeric_id, reg);
                numeric_id
            }
        }
    }
}

struct GlobalStringIdResolver<'input> {
    current_id: spirv::Word,
    variables: HashMap<Cow<'input, str>, spirv::Word>,
    variables_type_check: HashMap<u32, Option<(ast::Type, bool)>>,
    special_registers: SpecialRegistersMap,
    fns: HashMap<spirv::Word, FnDecl>,
}

pub struct FnDecl {
    ret_vals: Vec<ast::FnArgumentType>,
    params: Vec<ast::FnArgumentType>,
}

impl<'a> GlobalStringIdResolver<'a> {
    fn new(start_id: spirv::Word) -> Self {
        Self {
            current_id: start_id,
            variables: HashMap::new(),
            variables_type_check: HashMap::new(),
            special_registers: SpecialRegistersMap::new(),
            fns: HashMap::new(),
        }
    }

    fn get_or_add_def(&mut self, id: &'a str) -> spirv::Word {
        self.get_or_add_impl(id, None)
    }

    fn get_or_add_def_typed(
        &mut self,
        id: &'a str,
        typ: ast::Type,
        is_variable: bool,
    ) -> spirv::Word {
        self.get_or_add_impl(id, Some((typ, is_variable)))
    }

    fn get_or_add_impl(&mut self, id: &'a str, typ: Option<(ast::Type, bool)>) -> spirv::Word {
        let id = match self.variables.entry(Cow::Borrowed(id)) {
            hash_map::Entry::Occupied(e) => *(e.get()),
            hash_map::Entry::Vacant(e) => {
                let numeric_id = self.current_id;
                e.insert(numeric_id);
                self.current_id += 1;
                numeric_id
            }
        };
        self.variables_type_check.insert(id, typ);
        id
    }

    fn get_id(&self, id: &str) -> Result<spirv::Word, TranslateError> {
        self.variables
            .get(id)
            .copied()
            .ok_or(TranslateError::UnknownSymbol)
    }

    fn current_id(&self) -> spirv::Word {
        self.current_id
    }

    fn start_fn<'b>(
        &'b mut self,
        header: &'b ast::MethodDecl<'a, &'a str>,
    ) -> Result<
        (
            FnStringIdResolver<'a, 'b>,
            GlobalFnDeclResolver<'a, 'b>,
            ast::MethodDecl<'a, spirv::Word>,
        ),
        TranslateError,
    > {
        // In case a function decl was inserted earlier we want to use its id
        let name_id = self.get_or_add_def(header.name());
        let mut fn_resolver = FnStringIdResolver {
            current_id: &mut self.current_id,
            global_variables: &self.variables,
            global_type_check: &self.variables_type_check,
            special_registers: &mut self.special_registers,
            variables: vec![HashMap::new(); 1],
            type_check: HashMap::new(),
        };
        let new_fn_decl = match header {
            ast::MethodDecl::Kernel { name, in_args } => ast::MethodDecl::Kernel {
                name,
                in_args: expand_kernel_params(&mut fn_resolver, in_args.iter())?,
            },
            ast::MethodDecl::Func(ret_params, _, params) => {
                let ret_params_ids = expand_fn_params(&mut fn_resolver, ret_params.iter())?;
                let params_ids = expand_fn_params(&mut fn_resolver, params.iter())?;
                self.fns.insert(
                    name_id,
                    FnDecl {
                        ret_vals: ret_params_ids.iter().map(|p| p.v_type.clone()).collect(),
                        params: params_ids.iter().map(|p| p.v_type.clone()).collect(),
                    },
                );
                ast::MethodDecl::Func(ret_params_ids, name_id, params_ids)
            }
        };
        Ok((
            fn_resolver,
            GlobalFnDeclResolver {
                variables: &self.variables,
                fns: &self.fns,
            },
            new_fn_decl,
        ))
    }
}

pub struct GlobalFnDeclResolver<'input, 'a> {
    variables: &'a HashMap<Cow<'input, str>, spirv::Word>,
    fns: &'a HashMap<spirv::Word, FnDecl>,
}

impl<'input, 'a> GlobalFnDeclResolver<'input, 'a> {
    fn get_fn_decl(&self, id: spirv::Word) -> Result<&FnDecl, TranslateError> {
        self.fns.get(&id).ok_or(TranslateError::UnknownSymbol)
    }

    fn get_fn_decl_str(&self, id: &str) -> Result<&'a FnDecl, TranslateError> {
        match self.variables.get(id).map(|var_id| self.fns.get(var_id)) {
            Some(Some(fn_d)) => Ok(fn_d),
            _ => Err(TranslateError::UnknownSymbol),
        }
    }
}

struct FnStringIdResolver<'input, 'b> {
    current_id: &'b mut spirv::Word,
    global_variables: &'b HashMap<Cow<'input, str>, spirv::Word>,
    global_type_check: &'b HashMap<u32, Option<(ast::Type, bool)>>,
    special_registers: &'b mut SpecialRegistersMap,
    variables: Vec<HashMap<Cow<'input, str>, spirv::Word>>,
    type_check: HashMap<u32, Option<(ast::Type, bool)>>,
}

impl<'a, 'b> FnStringIdResolver<'a, 'b> {
    fn finish(self) -> NumericIdResolver<'b> {
        NumericIdResolver {
            current_id: self.current_id,
            global_type_check: self.global_type_check,
            type_check: self.type_check,
            special_registers: self.special_registers,
        }
    }

    fn start_block(&mut self) {
        self.variables.push(HashMap::new())
    }

    fn end_block(&mut self) {
        self.variables.pop();
    }

    fn get_id(&mut self, id: &str) -> Result<spirv::Word, TranslateError> {
        for scope in self.variables.iter().rev() {
            match scope.get(id) {
                Some(id) => return Ok(*id),
                None => continue,
            }
        }
        match self.global_variables.get(id) {
            Some(id) => Ok(*id),
            None => {
                let sreg =
                    PtxSpecialRegister::try_parse(id).ok_or(TranslateError::UnknownSymbol)?;
                Ok(self.special_registers.get_or_add(self.current_id, sreg))
            }
        }
    }

    fn add_def(&mut self, id: &'a str, typ: Option<ast::Type>, is_variable: bool) -> spirv::Word {
        let numeric_id = *self.current_id;
        self.variables
            .last_mut()
            .unwrap()
            .insert(Cow::Borrowed(id), numeric_id);
        self.type_check
            .insert(numeric_id, typ.map(|t| (t, is_variable)));
        *self.current_id += 1;
        numeric_id
    }

    #[must_use]
    fn add_defs(
        &mut self,
        base_id: &'a str,
        count: u32,
        typ: ast::Type,
        is_variable: bool,
    ) -> impl Iterator<Item = spirv::Word> {
        let numeric_id = *self.current_id;
        for i in 0..count {
            self.variables
                .last_mut()
                .unwrap()
                .insert(Cow::Owned(format!("{}{}", base_id, i)), numeric_id + i);
            self.type_check
                .insert(numeric_id + i, Some((typ.clone(), is_variable)));
        }
        *self.current_id += count;
        (0..count).into_iter().map(move |i| i + numeric_id)
    }
}

struct NumericIdResolver<'b> {
    current_id: &'b mut spirv::Word,
    global_type_check: &'b HashMap<u32, Option<(ast::Type, bool)>>,
    type_check: HashMap<u32, Option<(ast::Type, bool)>>,
    special_registers: &'b mut SpecialRegistersMap,
}

impl<'b> NumericIdResolver<'b> {
    fn finish(self) -> MutableNumericIdResolver<'b> {
        MutableNumericIdResolver { base: self }
    }

    fn get_typed(&self, id: spirv::Word) -> Result<(ast::Type, bool), TranslateError> {
        match self.type_check.get(&id) {
            Some(Some(x)) => Ok(x.clone()),
            Some(None) => Err(TranslateError::UntypedSymbol),
            None => match self.special_registers.get(id) {
                Some(x) => Ok((x.get_type(), true)),
                None => match self.global_type_check.get(&id) {
                    Some(Some(result)) => Ok(result.clone()),
                    Some(None) | None => Err(TranslateError::UntypedSymbol),
                },
            },
        }
    }

    // This is for identifiers which will be emitted later as OpVariable
    // They are candidates for insertion of LoadVar/StoreVar
    fn new_variable(&mut self, typ: ast::Type) -> spirv::Word {
        let new_id = *self.current_id;
        self.type_check.insert(new_id, Some((typ, true)));
        *self.current_id += 1;
        new_id
    }

    fn new_non_variable(&mut self, typ: Option<ast::Type>) -> spirv::Word {
        let new_id = *self.current_id;
        self.type_check.insert(new_id, typ.map(|t| (t, false)));
        *self.current_id += 1;
        new_id
    }
}

struct MutableNumericIdResolver<'b> {
    base: NumericIdResolver<'b>,
}

impl<'b> MutableNumericIdResolver<'b> {
    fn unmut(self) -> NumericIdResolver<'b> {
        self.base
    }

    fn get_typed(&self, id: spirv::Word) -> Result<ast::Type, TranslateError> {
        self.base.get_typed(id).map(|(t, _)| t)
    }

    fn new_non_variable(&mut self, typ: ast::Type) -> spirv::Word {
        self.base.new_non_variable(Some(typ))
    }
}

enum Statement<I, P: ast::ArgParams> {
    Label(u32),
    Variable(ast::Variable<ast::VariableType, P::Id>),
    Instruction(I),
    // SPIR-V compatible replacement for PTX predicates
    Conditional(BrachCondition),
    Call(ResolvedCall<P>),
    LoadVar(LoadVarDetails),
    StoreVar(StoreVarDetails),
    Conversion(ImplicitConversion),
    Constant(ConstantDefinition),
    RetValue(ast::RetData, spirv::Word),
    PtrAccess(PtrAccess<P>),
    RepackVector(RepackVectorDetails),
}

impl ExpandedStatement {
    fn map_id(self, f: &mut impl FnMut(spirv::Word, bool) -> spirv::Word) -> ExpandedStatement {
        match self {
            Statement::Label(id) => Statement::Label(f(id, false)),
            Statement::Variable(mut var) => {
                var.name = f(var.name, true);
                Statement::Variable(var)
            }
            Statement::Instruction(inst) => inst
                .visit(&mut |arg: ArgumentDescriptor<_>, _: Option<&ast::Type>| {
                    Ok(f(arg.op, arg.is_dst))
                })
                .unwrap(),
            Statement::LoadVar(mut details) => {
                details.arg.dst = f(details.arg.dst, true);
                details.arg.src = f(details.arg.src, false);
                Statement::LoadVar(details)
            }
            Statement::StoreVar(mut details) => {
                details.arg.src1 = f(details.arg.src1, false);
                details.arg.src2 = f(details.arg.src2, false);
                Statement::StoreVar(details)
            }
            Statement::Call(mut call) => {
                for (id, typ) in call.ret_params.iter_mut() {
                    let is_dst = match typ {
                        ast::FnArgumentType::Reg(_) => true,
                        ast::FnArgumentType::Param(_) => false,
                        ast::FnArgumentType::Shared => false,
                    };
                    *id = f(*id, is_dst);
                }
                call.func = f(call.func, false);
                for (id, _) in call.param_list.iter_mut() {
                    *id = f(*id, false);
                }
                Statement::Call(call)
            }
            Statement::Conditional(mut conditional) => {
                conditional.predicate = f(conditional.predicate, false);
                conditional.if_true = f(conditional.if_true, false);
                conditional.if_false = f(conditional.if_false, false);
                Statement::Conditional(conditional)
            }
            Statement::Conversion(mut conv) => {
                conv.dst = f(conv.dst, true);
                conv.src = f(conv.src, false);
                Statement::Conversion(conv)
            }
            Statement::Constant(mut constant) => {
                constant.dst = f(constant.dst, true);
                Statement::Constant(constant)
            }
            Statement::RetValue(data, id) => {
                let id = f(id, false);
                Statement::RetValue(data, id)
            }
            Statement::PtrAccess(PtrAccess {
                underlying_type,
                state_space,
                dst,
                ptr_src,
                offset_src: constant_src,
            }) => {
                let dst = f(dst, true);
                let ptr_src = f(ptr_src, false);
                let constant_src = f(constant_src, false);
                Statement::PtrAccess(PtrAccess {
                    underlying_type,
                    state_space,
                    dst,
                    ptr_src,
                    offset_src: constant_src,
                })
            }
            Statement::RepackVector(repack) => {
                let packed = f(repack.packed, !repack.is_extract);
                let unpacked = repack
                    .unpacked
                    .iter()
                    .map(|id| f(*id, repack.is_extract))
                    .collect();
                Statement::RepackVector(RepackVectorDetails {
                    packed,
                    unpacked,
                    ..repack
                })
            }
        }
    }
}

struct LoadVarDetails {
    arg: ast::Arg2<ExpandedArgParams>,
    typ: ast::Type,
    // (index, vector_width)
    // HACK ALERT
    // For some reason IGC explodes when you try to load from builtin vectors
    // using OpInBoundsAccessChain, the one true way to do it is to
    // OpLoad+OpCompositeExtract
    member_index: Option<(u8, Option<u8>)>,
}

struct StoreVarDetails {
    arg: ast::Arg2St<ExpandedArgParams>,
    typ: ast::Type,
    member_index: Option<u8>,
}

struct RepackVectorDetails {
    is_extract: bool,
    typ: ast::ScalarType,
    packed: spirv::Word,
    unpacked: Vec<spirv::Word>,
    vector_sema: ArgumentSemantics,
}

impl RepackVectorDetails {
    fn map<
        From: ArgParamsEx<Id = spirv::Word>,
        To: ArgParamsEx<Id = spirv::Word>,
        V: ArgumentMapVisitor<From, To>,
    >(
        self,
        visitor: &mut V,
    ) -> Result<RepackVectorDetails, TranslateError> {
        let scalar = visitor.id(
            ArgumentDescriptor {
                op: self.packed,
                is_dst: !self.is_extract,
                sema: ArgumentSemantics::Default,
            },
            Some(&ast::Type::Vector(self.typ, self.unpacked.len() as u8)),
        )?;
        let scalar_type = self.typ;
        let is_extract = self.is_extract;
        let vector_sema = self.vector_sema;
        let vector = self
            .unpacked
            .into_iter()
            .map(|id| {
                visitor.id(
                    ArgumentDescriptor {
                        op: id,
                        is_dst: is_extract,
                        sema: vector_sema,
                    },
                    Some(&ast::Type::Scalar(scalar_type)),
                )
            })
            .collect::<Result<_, _>>()?;
        Ok(RepackVectorDetails {
            is_extract,
            typ: self.typ,
            packed: scalar,
            unpacked: vector,
            vector_sema,
        })
    }
}

impl<T: ArgParamsEx<Id = spirv::Word>, U: ArgParamsEx<Id = spirv::Word>> Visitable<T, U>
    for RepackVectorDetails
{
    fn visit(
        self,
        visitor: &mut impl ArgumentMapVisitor<T, U>,
    ) -> Result<Statement<ast::Instruction<U>, U>, TranslateError> {
        Ok(Statement::RepackVector(self.map::<_, _, _>(visitor)?))
    }
}

struct ResolvedCall<P: ast::ArgParams> {
    pub uniform: bool,
    pub ret_params: Vec<(P::Id, ast::FnArgumentType)>,
    pub func: P::Id,
    pub param_list: Vec<(P::Operand, ast::FnArgumentType)>,
}

impl<T: ast::ArgParams> ResolvedCall<T> {
    fn cast<U: ast::ArgParams<Id = T::Id, Operand = T::Operand>>(self) -> ResolvedCall<U> {
        ResolvedCall {
            uniform: self.uniform,
            ret_params: self.ret_params,
            func: self.func,
            param_list: self.param_list,
        }
    }
}

impl<From: ArgParamsEx<Id = spirv::Word>> ResolvedCall<From> {
    fn map<To: ArgParamsEx<Id = spirv::Word>, V: ArgumentMapVisitor<From, To>>(
        self,
        visitor: &mut V,
    ) -> Result<ResolvedCall<To>, TranslateError> {
        let ret_params = self
            .ret_params
            .into_iter()
            .map::<Result<_, TranslateError>, _>(|(id, typ)| {
                let new_id = visitor.id(
                    ArgumentDescriptor {
                        op: id,
                        is_dst: !typ.is_param(),
                        sema: typ.semantics(),
                    },
                    Some(&typ.to_func_type()),
                )?;
                Ok((new_id, typ))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let func = visitor.id(
            ArgumentDescriptor {
                op: self.func,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            None,
        )?;
        let param_list = self
            .param_list
            .into_iter()
            .map::<Result<_, TranslateError>, _>(|(id, typ)| {
                let new_id = visitor.operand(
                    ArgumentDescriptor {
                        op: id,
                        is_dst: false,
                        sema: typ.semantics(),
                    },
                    &typ.to_func_type(),
                )?;
                Ok((new_id, typ))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ResolvedCall {
            uniform: self.uniform,
            ret_params,
            func,
            param_list,
        })
    }
}

impl<T: ArgParamsEx<Id = spirv::Word>, U: ArgParamsEx<Id = spirv::Word>> Visitable<T, U>
    for ResolvedCall<T>
{
    fn visit(
        self,
        visitor: &mut impl ArgumentMapVisitor<T, U>,
    ) -> Result<Statement<ast::Instruction<U>, U>, TranslateError> {
        Ok(Statement::Call(self.map(visitor)?))
    }
}

impl<P: ArgParamsEx<Id = spirv::Word>> PtrAccess<P> {
    fn map<To: ArgParamsEx<Id = spirv::Word>, V: ArgumentMapVisitor<P, To>>(
        self,
        visitor: &mut V,
    ) -> Result<PtrAccess<To>, TranslateError> {
        let sema = match self.state_space {
            ast::LdStateSpace::Const
            | ast::LdStateSpace::Global
            | ast::LdStateSpace::Shared
            | ast::LdStateSpace::Generic => ArgumentSemantics::PhysicalPointer,
            ast::LdStateSpace::Local | ast::LdStateSpace::Param => {
                ArgumentSemantics::RegisterPointer
            }
        };
        let ptr_type = ast::Type::Pointer(self.underlying_type.clone(), self.state_space);
        let new_dst = visitor.id(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                sema,
            },
            Some(&ptr_type),
        )?;
        let new_ptr_src = visitor.id(
            ArgumentDescriptor {
                op: self.ptr_src,
                is_dst: false,
                sema,
            },
            Some(&ptr_type),
        )?;
        let new_constant_src = visitor.operand(
            ArgumentDescriptor {
                op: self.offset_src,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            &ast::Type::Scalar(ast::ScalarType::S64),
        )?;
        Ok(PtrAccess {
            underlying_type: self.underlying_type,
            state_space: self.state_space,
            dst: new_dst,
            ptr_src: new_ptr_src,
            offset_src: new_constant_src,
        })
    }
}

impl<T: ArgParamsEx<Id = spirv::Word>, U: ArgParamsEx<Id = spirv::Word>> Visitable<T, U>
    for PtrAccess<T>
{
    fn visit(
        self,
        visitor: &mut impl ArgumentMapVisitor<T, U>,
    ) -> Result<Statement<ast::Instruction<U>, U>, TranslateError> {
        Ok(Statement::PtrAccess(self.map(visitor)?))
    }
}

pub trait ArgParamsEx: ast::ArgParams + Sized {
    fn get_fn_decl<'x, 'b>(
        id: &Self::Id,
        decl: &'b GlobalFnDeclResolver<'x, 'b>,
    ) -> Result<&'b FnDecl, TranslateError>;
}

impl<'input> ArgParamsEx for ast::ParsedArgParams<'input> {
    fn get_fn_decl<'x, 'b>(
        id: &Self::Id,
        decl: &'b GlobalFnDeclResolver<'x, 'b>,
    ) -> Result<&'b FnDecl, TranslateError> {
        decl.get_fn_decl_str(id)
    }
}

enum NormalizedArgParams {}

impl ast::ArgParams for NormalizedArgParams {
    type Id = spirv::Word;
    type Operand = ast::Operand<spirv::Word>;
}

impl ArgParamsEx for NormalizedArgParams {
    fn get_fn_decl<'a, 'b>(
        id: &Self::Id,
        decl: &'b GlobalFnDeclResolver<'a, 'b>,
    ) -> Result<&'b FnDecl, TranslateError> {
        decl.get_fn_decl(*id)
    }
}

type NormalizedStatement = Statement<
    (
        Option<ast::PredAt<spirv::Word>>,
        ast::Instruction<NormalizedArgParams>,
    ),
    NormalizedArgParams,
>;

type UnconditionalStatement = Statement<ast::Instruction<NormalizedArgParams>, NormalizedArgParams>;

enum TypedArgParams {}

impl ast::ArgParams for TypedArgParams {
    type Id = spirv::Word;
    type Operand = TypedOperand;
}

impl ArgParamsEx for TypedArgParams {
    fn get_fn_decl<'a, 'b>(
        id: &Self::Id,
        decl: &'b GlobalFnDeclResolver<'a, 'b>,
    ) -> Result<&'b FnDecl, TranslateError> {
        decl.get_fn_decl(*id)
    }
}

#[derive(Copy, Clone)]
enum TypedOperand {
    Reg(spirv::Word),
    RegOffset(spirv::Word, i32),
    Imm(ast::ImmediateValue),
    VecMember(spirv::Word, u8),
}

impl TypedOperand {
    fn upcast(self) -> ast::Operand<spirv::Word> {
        match self {
            TypedOperand::Reg(reg) => ast::Operand::Reg(reg),
            TypedOperand::RegOffset(reg, idx) => ast::Operand::RegOffset(reg, idx),
            TypedOperand::Imm(x) => ast::Operand::Imm(x),
            TypedOperand::VecMember(vec, idx) => ast::Operand::VecMember(vec, idx),
        }
    }
}

type TypedStatement = Statement<ast::Instruction<TypedArgParams>, TypedArgParams>;

enum ExpandedArgParams {}
type ExpandedStatement = Statement<ast::Instruction<ExpandedArgParams>, ExpandedArgParams>;

impl ast::ArgParams for ExpandedArgParams {
    type Id = spirv::Word;
    type Operand = spirv::Word;
}

impl ArgParamsEx for ExpandedArgParams {
    fn get_fn_decl<'a, 'b>(
        id: &Self::Id,
        decl: &'b GlobalFnDeclResolver<'a, 'b>,
    ) -> Result<&'b FnDecl, TranslateError> {
        decl.get_fn_decl(*id)
    }
}

enum Directive<'input> {
    Variable(ast::Variable<ast::VariableType, spirv::Word>),
    Method(Function<'input>),
}

struct Function<'input> {
    pub func_decl: ast::MethodDecl<'input, spirv::Word>,
    pub spirv_decl: SpirvMethodDecl<'input>,
    pub globals: Vec<ast::Variable<ast::VariableType, spirv::Word>>,
    pub body: Option<Vec<ExpandedStatement>>,
    import_as: Option<String>,
}

pub trait ArgumentMapVisitor<T: ArgParamsEx, U: ArgParamsEx> {
    fn id(
        &mut self,
        desc: ArgumentDescriptor<T::Id>,
        typ: Option<&ast::Type>,
    ) -> Result<U::Id, TranslateError>;
    fn operand(
        &mut self,
        desc: ArgumentDescriptor<T::Operand>,
        typ: &ast::Type,
    ) -> Result<U::Operand, TranslateError>;
}

impl<T> ArgumentMapVisitor<ExpandedArgParams, ExpandedArgParams> for T
where
    T: FnMut(
        ArgumentDescriptor<spirv::Word>,
        Option<&ast::Type>,
    ) -> Result<spirv::Word, TranslateError>,
{
    fn id(
        &mut self,
        desc: ArgumentDescriptor<spirv::Word>,
        t: Option<&ast::Type>,
    ) -> Result<spirv::Word, TranslateError> {
        self(desc, t)
    }

    fn operand(
        &mut self,
        desc: ArgumentDescriptor<spirv::Word>,
        typ: &ast::Type,
    ) -> Result<spirv::Word, TranslateError> {
        self(desc, Some(typ))
    }
}

impl<'a, T> ArgumentMapVisitor<ast::ParsedArgParams<'a>, NormalizedArgParams> for T
where
    T: FnMut(&str) -> Result<spirv::Word, TranslateError>,
{
    fn id(
        &mut self,
        desc: ArgumentDescriptor<&str>,
        _: Option<&ast::Type>,
    ) -> Result<spirv::Word, TranslateError> {
        self(desc.op)
    }

    fn operand(
        &mut self,
        desc: ArgumentDescriptor<ast::Operand<&str>>,
        typ: &ast::Type,
    ) -> Result<ast::Operand<spirv::Word>, TranslateError> {
        Ok(match desc.op {
            ast::Operand::Reg(id) => ast::Operand::Reg(self(id)?),
            ast::Operand::RegOffset(id, imm) => ast::Operand::RegOffset(self(id)?, imm),
            ast::Operand::Imm(imm) => ast::Operand::Imm(imm),
            ast::Operand::VecMember(id, member) => ast::Operand::VecMember(self(id)?, member),
            ast::Operand::VecPack(ref ids) => ast::Operand::VecPack(
                ids.into_iter()
                    .map(|id| self.id(desc.new_op(id), Some(typ)))
                    .collect::<Result<Vec<_>, _>>()?,
            ),
        })
    }
}

pub struct ArgumentDescriptor<Op> {
    op: Op,
    is_dst: bool,
    sema: ArgumentSemantics,
}

pub struct PtrAccess<P: ast::ArgParams> {
    underlying_type: ast::PointerType,
    state_space: ast::LdStateSpace,
    dst: spirv::Word,
    ptr_src: spirv::Word,
    offset_src: P::Operand,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ArgumentSemantics {
    // normal register access
    Default,
    // normal register access with relaxed conversion rules (ld/st)
    DefaultRelaxed,
    // st/ld global
    PhysicalPointer,
    // st/ld .param, .local
    RegisterPointer,
    // mov of .local/.global variables
    Address,
}

impl<T> ArgumentDescriptor<T> {
    fn new_op<U>(&self, u: U) -> ArgumentDescriptor<U> {
        ArgumentDescriptor {
            op: u,
            is_dst: self.is_dst,
            sema: self.sema,
        }
    }
}

impl<T: ArgParamsEx> ast::Instruction<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
    ) -> Result<ast::Instruction<U>, TranslateError> {
        Ok(match self {
            ast::Instruction::Abs(d, arg) => {
                ast::Instruction::Abs(d, arg.map(visitor, &ast::Type::Scalar(d.typ))?)
            }
            // Call instruction is converted to a call statement early on
            ast::Instruction::Call(_) => return Err(error_unreachable()),
            ast::Instruction::Ld(d, a) => {
                let new_args = a.map(visitor, &d)?;
                ast::Instruction::Ld(d, new_args)
            }
            ast::Instruction::Mov(d, a) => {
                let mapped = a.map(visitor, &d)?;
                ast::Instruction::Mov(d, mapped)
            }
            ast::Instruction::Mul(d, a) => {
                let inst_type = d.get_type();
                let is_wide = d.is_wide();
                ast::Instruction::Mul(d, a.map_non_shift(visitor, &inst_type, is_wide)?)
            }
            ast::Instruction::Add(d, a) => {
                let inst_type = d.get_type();
                ast::Instruction::Add(d, a.map_non_shift(visitor, &inst_type, false)?)
            }
            ast::Instruction::Setp(d, a) => {
                let inst_type = d.typ;
                ast::Instruction::Setp(d, a.map(visitor, &ast::Type::Scalar(inst_type))?)
            }
            ast::Instruction::SetpBool(d, a) => {
                let inst_type = d.typ;
                ast::Instruction::SetpBool(d, a.map(visitor, &ast::Type::Scalar(inst_type))?)
            }
            ast::Instruction::Not(t, a) => ast::Instruction::Not(t, a.map(visitor, &t.to_type())?),
            ast::Instruction::Cvt(d, a) => {
                let (dst_t, src_t) = match &d {
                    ast::CvtDetails::FloatFromFloat(desc) => (
                        ast::Type::Scalar(desc.dst.into()),
                        ast::Type::Scalar(desc.src.into()),
                    ),
                    ast::CvtDetails::FloatFromInt(desc) => (
                        ast::Type::Scalar(desc.dst.into()),
                        ast::Type::Scalar(desc.src.into()),
                    ),
                    ast::CvtDetails::IntFromFloat(desc) => (
                        ast::Type::Scalar(desc.dst.into()),
                        ast::Type::Scalar(desc.src.into()),
                    ),
                    ast::CvtDetails::IntFromInt(desc) => (
                        ast::Type::Scalar(desc.dst.into()),
                        ast::Type::Scalar(desc.src.into()),
                    ),
                };
                ast::Instruction::Cvt(d, a.map_different_types(visitor, &dst_t, &src_t)?)
            }
            ast::Instruction::Shl(t, a) => {
                ast::Instruction::Shl(t, a.map_shift(visitor, &t.to_type())?)
            }
            ast::Instruction::Shr(t, a) => {
                ast::Instruction::Shr(t, a.map_shift(visitor, &ast::Type::Scalar(t.into()))?)
            }
            ast::Instruction::St(d, a) => {
                let new_args = a.map(visitor, &d)?;
                ast::Instruction::St(d, new_args)
            }
            ast::Instruction::Bra(d, a) => ast::Instruction::Bra(d, a.map(visitor, None)?),
            ast::Instruction::Ret(d) => ast::Instruction::Ret(d),
            ast::Instruction::Cvta(d, a) => {
                let inst_type = ast::Type::Scalar(ast::ScalarType::B64);
                ast::Instruction::Cvta(d, a.map(visitor, &inst_type)?)
            }
            ast::Instruction::Mad(d, a) => {
                let inst_type = d.get_type();
                let is_wide = d.is_wide();
                ast::Instruction::Mad(d, a.map(visitor, &inst_type, is_wide)?)
            }
            ast::Instruction::Or(t, a) => ast::Instruction::Or(
                t,
                a.map_non_shift(visitor, &ast::Type::Scalar(t.into()), false)?,
            ),
            ast::Instruction::Sub(d, a) => {
                let typ = d.get_type();
                ast::Instruction::Sub(d, a.map_non_shift(visitor, &typ, false)?)
            }
            ast::Instruction::Min(d, a) => {
                let typ = d.get_type();
                ast::Instruction::Min(d, a.map_non_shift(visitor, &typ, false)?)
            }
            ast::Instruction::Max(d, a) => {
                let typ = d.get_type();
                ast::Instruction::Max(d, a.map_non_shift(visitor, &typ, false)?)
            }
            ast::Instruction::Rcp(d, a) => {
                let typ = ast::Type::Scalar(if d.is_f64 {
                    ast::ScalarType::F64
                } else {
                    ast::ScalarType::F32
                });
                ast::Instruction::Rcp(d, a.map(visitor, &typ)?)
            }
            ast::Instruction::And(t, a) => ast::Instruction::And(
                t,
                a.map_non_shift(visitor, &ast::Type::Scalar(t.into()), false)?,
            ),
            ast::Instruction::Selp(t, a) => ast::Instruction::Selp(t, a.map_selp(visitor, t)?),
            ast::Instruction::Bar(d, a) => ast::Instruction::Bar(d, a.map(visitor)?),
            ast::Instruction::Atom(d, a) => {
                ast::Instruction::Atom(d, a.map_atom(visitor, d.inner.get_type(), d.space)?)
            }
            ast::Instruction::AtomCas(d, a) => {
                ast::Instruction::AtomCas(d, a.map_atom(visitor, d.typ, d.space)?)
            }
            ast::Instruction::Div(d, a) => {
                ast::Instruction::Div(d, a.map_non_shift(visitor, &d.get_type(), false)?)
            }
            ast::Instruction::Sqrt(d, a) => {
                ast::Instruction::Sqrt(d, a.map(visitor, &ast::Type::Scalar(d.typ.into()))?)
            }
            ast::Instruction::Rsqrt(d, a) => {
                ast::Instruction::Rsqrt(d, a.map(visitor, &ast::Type::Scalar(d.typ.into()))?)
            }
            ast::Instruction::Neg(d, a) => {
                ast::Instruction::Neg(d, a.map(visitor, &ast::Type::Scalar(d.typ))?)
            }
            ast::Instruction::Sin { flush_to_zero, arg } => {
                let typ = ast::Type::Scalar(ast::ScalarType::F32);
                ast::Instruction::Sin {
                    flush_to_zero,
                    arg: arg.map(visitor, &typ)?,
                }
            }
            ast::Instruction::Cos { flush_to_zero, arg } => {
                let typ = ast::Type::Scalar(ast::ScalarType::F32);
                ast::Instruction::Cos {
                    flush_to_zero,
                    arg: arg.map(visitor, &typ)?,
                }
            }
            ast::Instruction::Lg2 { flush_to_zero, arg } => {
                let typ = ast::Type::Scalar(ast::ScalarType::F32);
                ast::Instruction::Lg2 {
                    flush_to_zero,
                    arg: arg.map(visitor, &typ)?,
                }
            }
            ast::Instruction::Ex2 { flush_to_zero, arg } => {
                let typ = ast::Type::Scalar(ast::ScalarType::F32);
                ast::Instruction::Ex2 {
                    flush_to_zero,
                    arg: arg.map(visitor, &typ)?,
                }
            }
            ast::Instruction::Clz { typ, arg } => {
                let dst_type = ast::Type::Scalar(ast::ScalarType::B32);
                let src_type = ast::Type::Scalar(typ.into());
                ast::Instruction::Clz {
                    typ,
                    arg: arg.map_different_types(visitor, &dst_type, &src_type)?,
                }
            }
            ast::Instruction::Brev { typ, arg } => {
                let full_type = ast::Type::Scalar(typ.into());
                ast::Instruction::Brev {
                    typ,
                    arg: arg.map(visitor, &full_type)?,
                }
            }
            ast::Instruction::Popc { typ, arg } => {
                let dst_type = ast::Type::Scalar(ast::ScalarType::B32);
                let src_type = ast::Type::Scalar(typ.into());
                ast::Instruction::Popc {
                    typ,
                    arg: arg.map_different_types(visitor, &dst_type, &src_type)?,
                }
            }
            ast::Instruction::Xor { typ, arg } => {
                let full_type = ast::Type::Scalar(typ.into());
                ast::Instruction::Xor {
                    typ,
                    arg: arg.map_non_shift(visitor, &full_type, false)?,
                }
            }
            ast::Instruction::Bfe { typ, arg } => {
                let full_type = ast::Type::Scalar(typ.into());
                ast::Instruction::Bfe {
                    typ,
                    arg: arg.map_bfe(visitor, &full_type)?,
                }
            }
            ast::Instruction::Bfi { typ, arg } => {
                let full_type = ast::Type::Scalar(typ.into());
                ast::Instruction::Bfi {
                    typ,
                    arg: arg.map_bfi(visitor, &full_type)?,
                }
            }
            ast::Instruction::Rem { typ, arg } => {
                let full_type = ast::Type::Scalar(typ.into());
                ast::Instruction::Rem {
                    typ,
                    arg: arg.map_non_shift(visitor, &full_type, false)?,
                }
            }
        })
    }
}

impl<T: ArgParamsEx, U: ArgParamsEx> Visitable<T, U> for ast::Instruction<T> {
    fn visit(
        self,
        visitor: &mut impl ArgumentMapVisitor<T, U>,
    ) -> Result<Statement<ast::Instruction<U>, U>, TranslateError> {
        Ok(Statement::Instruction(self.map(visitor)?))
    }
}

impl ImplicitConversion {
    fn map<
        T: ArgParamsEx<Id = spirv::Word>,
        U: ArgParamsEx<Id = spirv::Word>,
        V: ArgumentMapVisitor<T, U>,
    >(
        self,
        visitor: &mut V,
    ) -> Result<Statement<ast::Instruction<U>, U>, TranslateError> {
        let new_dst = visitor.id(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                sema: self.dst_sema,
            },
            Some(&self.to),
        )?;
        let new_src = visitor.id(
            ArgumentDescriptor {
                op: self.src,
                is_dst: false,
                sema: self.src_sema,
            },
            Some(&self.from),
        )?;
        Ok(Statement::Conversion({
            ImplicitConversion {
                src: new_src,
                dst: new_dst,
                ..self
            }
        }))
    }
}

impl<From: ArgParamsEx<Id = spirv::Word>, To: ArgParamsEx<Id = spirv::Word>> Visitable<From, To>
    for ImplicitConversion
{
    fn visit(
        self,
        visitor: &mut impl ArgumentMapVisitor<From, To>,
    ) -> Result<Statement<ast::Instruction<To>, To>, TranslateError> {
        Ok(self.map(visitor)?)
    }
}

impl<T> ArgumentMapVisitor<TypedArgParams, TypedArgParams> for T
where
    T: FnMut(
        ArgumentDescriptor<spirv::Word>,
        Option<&ast::Type>,
    ) -> Result<spirv::Word, TranslateError>,
{
    fn id(
        &mut self,
        desc: ArgumentDescriptor<spirv::Word>,
        t: Option<&ast::Type>,
    ) -> Result<spirv::Word, TranslateError> {
        self(desc, t)
    }

    fn operand(
        &mut self,
        desc: ArgumentDescriptor<TypedOperand>,
        typ: &ast::Type,
    ) -> Result<TypedOperand, TranslateError> {
        Ok(match desc.op {
            TypedOperand::Reg(id) => TypedOperand::Reg(self(desc.new_op(id), Some(typ))?),
            TypedOperand::Imm(imm) => TypedOperand::Imm(imm),
            TypedOperand::RegOffset(id, imm) => {
                TypedOperand::RegOffset(self(desc.new_op(id), Some(typ))?, imm)
            }
            TypedOperand::VecMember(reg, index) => {
                let scalar_type = match typ {
                    ast::Type::Scalar(scalar_t) => *scalar_t,
                    _ => return Err(error_unreachable()),
                };
                let vec_type = ast::Type::Vector(scalar_type, index + 1);
                TypedOperand::VecMember(self(desc.new_op(reg), Some(&vec_type))?, index)
            }
        })
    }
}

impl ast::Type {
    fn widen(self) -> Result<Self, TranslateError> {
        match self {
            ast::Type::Scalar(scalar) => {
                let kind = scalar.kind();
                let width = scalar.size_of();
                if (kind != ScalarKind::Signed
                    && kind != ScalarKind::Unsigned
                    && kind != ScalarKind::Bit)
                    || (width == 8)
                {
                    return Err(TranslateError::MismatchedType);
                }
                Ok(ast::Type::Scalar(ast::ScalarType::from_parts(
                    width * 2,
                    kind,
                )))
            }
            _ => Err(error_unreachable()),
        }
    }

    fn to_parts(&self) -> TypeParts {
        match self {
            ast::Type::Scalar(scalar) => TypeParts {
                kind: TypeKind::Scalar,
                scalar_kind: scalar.kind(),
                width: scalar.size_of(),
                components: Vec::new(),
                state_space: ast::LdStateSpace::Global,
            },
            ast::Type::Vector(scalar, components) => TypeParts {
                kind: TypeKind::Vector,
                scalar_kind: scalar.kind(),
                width: scalar.size_of(),
                components: vec![*components as u32],
                state_space: ast::LdStateSpace::Global,
            },
            ast::Type::Array(scalar, components) => TypeParts {
                kind: TypeKind::Array,
                scalar_kind: scalar.kind(),
                width: scalar.size_of(),
                components: components.clone(),
                state_space: ast::LdStateSpace::Global,
            },
            ast::Type::Pointer(ast::PointerType::Scalar(scalar), state_space) => TypeParts {
                kind: TypeKind::PointerScalar,
                scalar_kind: scalar.kind(),
                width: scalar.size_of(),
                components: Vec::new(),
                state_space: *state_space,
            },
            ast::Type::Pointer(ast::PointerType::Vector(scalar, len), state_space) => TypeParts {
                kind: TypeKind::PointerVector,
                scalar_kind: scalar.kind(),
                width: scalar.size_of(),
                components: vec![*len as u32],
                state_space: *state_space,
            },
            ast::Type::Pointer(ast::PointerType::Array(scalar, components), state_space) => {
                TypeParts {
                    kind: TypeKind::PointerArray,
                    scalar_kind: scalar.kind(),
                    width: scalar.size_of(),
                    components: components.clone(),
                    state_space: *state_space,
                }
            }
            ast::Type::Pointer(ast::PointerType::Pointer(scalar, inner_space), state_space) => {
                TypeParts {
                    kind: TypeKind::PointerPointer,
                    scalar_kind: scalar.kind(),
                    width: scalar.size_of(),
                    components: vec![*inner_space as u32],
                    state_space: *state_space,
                }
            }
        }
    }

    fn from_parts(t: TypeParts) -> Self {
        match t.kind {
            TypeKind::Scalar => {
                ast::Type::Scalar(ast::ScalarType::from_parts(t.width, t.scalar_kind))
            }
            TypeKind::Vector => ast::Type::Vector(
                ast::ScalarType::from_parts(t.width, t.scalar_kind),
                t.components[0] as u8,
            ),
            TypeKind::Array => ast::Type::Array(
                ast::ScalarType::from_parts(t.width, t.scalar_kind),
                t.components,
            ),
            TypeKind::PointerScalar => ast::Type::Pointer(
                ast::PointerType::Scalar(ast::ScalarType::from_parts(t.width, t.scalar_kind)),
                t.state_space,
            ),
            TypeKind::PointerVector => ast::Type::Pointer(
                ast::PointerType::Vector(
                    ast::ScalarType::from_parts(t.width, t.scalar_kind),
                    t.components[0] as u8,
                ),
                t.state_space,
            ),
            TypeKind::PointerArray => ast::Type::Pointer(
                ast::PointerType::Array(
                    ast::ScalarType::from_parts(t.width, t.scalar_kind),
                    t.components,
                ),
                t.state_space,
            ),
            TypeKind::PointerPointer => ast::Type::Pointer(
                ast::PointerType::Pointer(
                    ast::ScalarType::from_parts(t.width, t.scalar_kind),
                    unsafe { mem::transmute::<_, ast::LdStateSpace>(t.components[0] as u8) },
                ),
                t.state_space,
            ),
        }
    }

    pub fn size_of(&self) -> usize {
        match self {
            ast::Type::Scalar(typ) => typ.size_of() as usize,
            ast::Type::Vector(typ, len) => (typ.size_of() as usize) * (*len as usize),
            ast::Type::Array(typ, len) => len
                .iter()
                .fold(typ.size_of() as usize, |x, y| (x as usize) * (*y as usize)),
            ast::Type::Pointer(_, _) => mem::size_of::<usize>(),
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
struct TypeParts {
    kind: TypeKind,
    scalar_kind: ScalarKind,
    width: u8,
    components: Vec<u32>,
    state_space: ast::LdStateSpace,
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum TypeKind {
    Scalar,
    Vector,
    Array,
    PointerScalar,
    PointerVector,
    PointerArray,
    PointerPointer,
}

impl ast::Instruction<ExpandedArgParams> {
    fn jump_target(&self) -> Option<spirv::Word> {
        match self {
            ast::Instruction::Bra(_, a) => Some(a.src),
            _ => None,
        }
    }

    // .wide instructions don't support ftz, so it's enough to just look at the
    // type declared by the instruction
    fn flush_to_zero(&self) -> Option<(bool, u8)> {
        match self {
            ast::Instruction::Ld(_, _) => None,
            ast::Instruction::St(_, _) => None,
            ast::Instruction::Mov(_, _) => None,
            ast::Instruction::Not(_, _) => None,
            ast::Instruction::Bra(_, _) => None,
            ast::Instruction::Shl(_, _) => None,
            ast::Instruction::Shr(_, _) => None,
            ast::Instruction::Ret(_) => None,
            ast::Instruction::Call(_) => None,
            ast::Instruction::Or(_, _) => None,
            ast::Instruction::And(_, _) => None,
            ast::Instruction::Cvta(_, _) => None,
            ast::Instruction::Selp(_, _) => None,
            ast::Instruction::Bar(_, _) => None,
            ast::Instruction::Atom(_, _) => None,
            ast::Instruction::AtomCas(_, _) => None,
            ast::Instruction::Sub(ast::ArithDetails::Signed(_), _) => None,
            ast::Instruction::Sub(ast::ArithDetails::Unsigned(_), _) => None,
            ast::Instruction::Add(ast::ArithDetails::Signed(_), _) => None,
            ast::Instruction::Add(ast::ArithDetails::Unsigned(_), _) => None,
            ast::Instruction::Mul(ast::MulDetails::Unsigned(_), _) => None,
            ast::Instruction::Mul(ast::MulDetails::Signed(_), _) => None,
            ast::Instruction::Mad(ast::MulDetails::Unsigned(_), _) => None,
            ast::Instruction::Mad(ast::MulDetails::Signed(_), _) => None,
            ast::Instruction::Min(ast::MinMaxDetails::Signed(_), _) => None,
            ast::Instruction::Min(ast::MinMaxDetails::Unsigned(_), _) => None,
            ast::Instruction::Max(ast::MinMaxDetails::Signed(_), _) => None,
            ast::Instruction::Max(ast::MinMaxDetails::Unsigned(_), _) => None,
            ast::Instruction::Cvt(ast::CvtDetails::IntFromInt(_), _) => None,
            ast::Instruction::Cvt(ast::CvtDetails::FloatFromInt(_), _) => None,
            ast::Instruction::Div(ast::DivDetails::Unsigned(_), _) => None,
            ast::Instruction::Div(ast::DivDetails::Signed(_), _) => None,
            ast::Instruction::Clz { .. } => None,
            ast::Instruction::Brev { .. } => None,
            ast::Instruction::Popc { .. } => None,
            ast::Instruction::Xor { .. } => None,
            ast::Instruction::Bfe { .. } => None,
            ast::Instruction::Bfi { .. } => None,
            ast::Instruction::Rem { .. } => None,
            ast::Instruction::Sub(ast::ArithDetails::Float(float_control), _)
            | ast::Instruction::Add(ast::ArithDetails::Float(float_control), _)
            | ast::Instruction::Mul(ast::MulDetails::Float(float_control), _)
            | ast::Instruction::Mad(ast::MulDetails::Float(float_control), _) => float_control
                .flush_to_zero
                .map(|ftz| (ftz, ast::ScalarType::from(float_control.typ).size_of())),
            ast::Instruction::Setp(details, _) => details
                .flush_to_zero
                .map(|ftz| (ftz, details.typ.size_of())),
            ast::Instruction::SetpBool(details, _) => details
                .flush_to_zero
                .map(|ftz| (ftz, details.typ.size_of())),
            ast::Instruction::Abs(details, _) => details
                .flush_to_zero
                .map(|ftz| (ftz, details.typ.size_of())),
            ast::Instruction::Min(ast::MinMaxDetails::Float(float_control), _)
            | ast::Instruction::Max(ast::MinMaxDetails::Float(float_control), _) => float_control
                .flush_to_zero
                .map(|ftz| (ftz, ast::ScalarType::from(float_control.typ).size_of())),
            ast::Instruction::Rcp(details, _) => details
                .flush_to_zero
                .map(|ftz| (ftz, if details.is_f64 { 8 } else { 4 })),
            // Modifier .ftz can only be specified when either .dtype or .atype
            // is .f32 and applies only to single precision (.f32) inputs and results.
            ast::Instruction::Cvt(
                ast::CvtDetails::FloatFromFloat(ast::CvtDesc { flush_to_zero, .. }),
                _,
            )
            | ast::Instruction::Cvt(
                ast::CvtDetails::IntFromFloat(ast::CvtDesc { flush_to_zero, .. }),
                _,
            ) => flush_to_zero.map(|ftz| (ftz, 4)),
            ast::Instruction::Div(ast::DivDetails::Float(details), _) => details
                .flush_to_zero
                .map(|ftz| (ftz, ast::ScalarType::from(details.typ).size_of())),
            ast::Instruction::Sqrt(details, _) => details
                .flush_to_zero
                .map(|ftz| (ftz, ast::ScalarType::from(details.typ).size_of())),
            ast::Instruction::Rsqrt(details, _) => Some((
                details.flush_to_zero,
                ast::ScalarType::from(details.typ).size_of(),
            )),
            ast::Instruction::Neg(details, _) => details
                .flush_to_zero
                .map(|ftz| (ftz, details.typ.size_of())),
            ast::Instruction::Sin { flush_to_zero, .. }
            | ast::Instruction::Cos { flush_to_zero, .. }
            | ast::Instruction::Lg2 { flush_to_zero, .. }
            | ast::Instruction::Ex2 { flush_to_zero, .. } => {
                Some((*flush_to_zero, mem::size_of::<f32>() as u8))
            }
        }
    }
}

type Arg2 = ast::Arg2<ExpandedArgParams>;
type Arg2St = ast::Arg2St<ExpandedArgParams>;

struct ConstantDefinition {
    pub dst: spirv::Word,
    pub typ: ast::ScalarType,
    pub value: ast::ImmediateValue,
}

struct BrachCondition {
    predicate: spirv::Word,
    if_true: spirv::Word,
    if_false: spirv::Word,
}

#[derive(Clone)]
struct ImplicitConversion {
    src: spirv::Word,
    dst: spirv::Word,
    from: ast::Type,
    to: ast::Type,
    kind: ConversionKind,
    src_sema: ArgumentSemantics,
    dst_sema: ArgumentSemantics,
}

#[derive(PartialEq, Copy, Clone)]
enum ConversionKind {
    Default,
    // zero-extend/chop/bitcast depending on types
    SignExtend,
    BitToPtr(ast::LdStateSpace),
    PtrToBit(ast::UIntType),
    PtrToPtr { spirv_ptr: bool },
}

impl<T> ast::PredAt<T> {
    fn map_variable<U, F: FnMut(T) -> Result<U, TranslateError>>(
        self,
        f: &mut F,
    ) -> Result<ast::PredAt<U>, TranslateError> {
        let new_label = f(self.label)?;
        Ok(ast::PredAt {
            not: self.not,
            label: new_label,
        })
    }
}

impl<'a> ast::Instruction<ast::ParsedArgParams<'a>> {
    fn map_variable<F: FnMut(&str) -> Result<spirv::Word, TranslateError>>(
        self,
        f: &mut F,
    ) -> Result<ast::Instruction<NormalizedArgParams>, TranslateError> {
        match self {
            ast::Instruction::Call(call) => {
                let call_inst = ast::CallInst {
                    uniform: call.uniform,
                    ret_params: call
                        .ret_params
                        .into_iter()
                        .map(|p| f(p))
                        .collect::<Result<_, _>>()?,
                    func: f(call.func)?,
                    param_list: call
                        .param_list
                        .into_iter()
                        .map(|p| p.map_variable(f))
                        .collect::<Result<_, _>>()?,
                };
                Ok(ast::Instruction::Call(call_inst))
            }
            i => i.map(f),
        }
    }
}

impl<T: ArgParamsEx> ast::Arg1<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: Option<&ast::Type>,
    ) -> Result<ast::Arg1<U>, TranslateError> {
        let new_src = visitor.id(
            ArgumentDescriptor {
                op: self.src,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            t,
        )?;
        Ok(ast::Arg1 { src: new_src })
    }
}

impl<T: ArgParamsEx> ast::Arg1Bar<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
    ) -> Result<ast::Arg1Bar<U>, TranslateError> {
        let new_src = visitor.operand(
            ArgumentDescriptor {
                op: self.src,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            &ast::Type::Scalar(ast::ScalarType::U32),
        )?;
        Ok(ast::Arg1Bar { src: new_src })
    }
}

impl<T: ArgParamsEx> ast::Arg2<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: &ast::Type,
    ) -> Result<ast::Arg2<U>, TranslateError> {
        let new_dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                sema: ArgumentSemantics::Default,
            },
            t,
        )?;
        let new_src = visitor.operand(
            ArgumentDescriptor {
                op: self.src,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            t,
        )?;
        Ok(ast::Arg2 {
            dst: new_dst,
            src: new_src,
        })
    }

    fn map_different_types<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        dst_t: &ast::Type,
        src_t: &ast::Type,
    ) -> Result<ast::Arg2<U>, TranslateError> {
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                sema: ArgumentSemantics::Default,
            },
            dst_t,
        )?;
        let src = visitor.operand(
            ArgumentDescriptor {
                op: self.src,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            src_t,
        )?;
        Ok(ast::Arg2 { dst, src })
    }
}

impl<T: ArgParamsEx> ast::Arg2Ld<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        details: &ast::LdDetails,
    ) -> Result<ast::Arg2Ld<U>, TranslateError> {
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                sema: ArgumentSemantics::DefaultRelaxed,
            },
            &ast::Type::from(details.typ.clone()),
        )?;
        let is_logical_ptr = details.state_space == ast::LdStateSpace::Param
            || details.state_space == ast::LdStateSpace::Local;
        let src = visitor.operand(
            ArgumentDescriptor {
                op: self.src,
                is_dst: false,
                sema: if is_logical_ptr {
                    ArgumentSemantics::RegisterPointer
                } else {
                    ArgumentSemantics::PhysicalPointer
                },
            },
            &ast::Type::Pointer(
                ast::PointerType::from(details.typ.clone()),
                details.state_space,
            ),
        )?;
        Ok(ast::Arg2Ld { dst, src })
    }
}

impl<T: ArgParamsEx> ast::Arg2St<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        details: &ast::StData,
    ) -> Result<ast::Arg2St<U>, TranslateError> {
        let is_logical_ptr = details.state_space == ast::StStateSpace::Param
            || details.state_space == ast::StStateSpace::Local;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                sema: if is_logical_ptr {
                    ArgumentSemantics::RegisterPointer
                } else {
                    ArgumentSemantics::PhysicalPointer
                },
            },
            &ast::Type::Pointer(
                ast::PointerType::from(details.typ.clone()),
                details.state_space.to_ld_ss(),
            ),
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                sema: ArgumentSemantics::DefaultRelaxed,
            },
            &details.typ.clone().into(),
        )?;
        Ok(ast::Arg2St { src1, src2 })
    }
}

impl<T: ArgParamsEx> ast::Arg2Mov<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        details: &ast::MovDetails,
    ) -> Result<ast::Arg2Mov<U>, TranslateError> {
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                sema: ArgumentSemantics::Default,
            },
            &details.typ.clone().into(),
        )?;
        let src = visitor.operand(
            ArgumentDescriptor {
                op: self.src,
                is_dst: false,
                sema: if details.src_is_address {
                    ArgumentSemantics::Address
                } else {
                    ArgumentSemantics::Default
                },
            },
            &details.typ.clone().into(),
        )?;
        Ok(ast::Arg2Mov { dst, src })
    }
}

impl<T: ArgParamsEx> ast::Arg3<T> {
    fn map_non_shift<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        typ: &ast::Type,
        is_wide: bool,
    ) -> Result<ast::Arg3<U>, TranslateError> {
        let wide_type = if is_wide {
            Some(typ.clone().widen()?)
        } else {
            None
        };
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                sema: ArgumentSemantics::Default,
            },
            wide_type.as_ref().unwrap_or(typ),
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            typ,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            typ,
        )?;
        Ok(ast::Arg3 { dst, src1, src2 })
    }

    fn map_shift<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: &ast::Type,
    ) -> Result<ast::Arg3<U>, TranslateError> {
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                sema: ArgumentSemantics::Default,
            },
            t,
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            t,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            &ast::Type::Scalar(ast::ScalarType::U32),
        )?;
        Ok(ast::Arg3 { dst, src1, src2 })
    }

    fn map_atom<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: ast::ScalarType,
        state_space: ast::AtomSpace,
    ) -> Result<ast::Arg3<U>, TranslateError> {
        let scalar_type = ast::ScalarType::from(t);
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                sema: ArgumentSemantics::Default,
            },
            &ast::Type::Scalar(scalar_type),
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                sema: ArgumentSemantics::PhysicalPointer,
            },
            &ast::Type::Pointer(
                ast::PointerType::Scalar(scalar_type),
                state_space.to_ld_ss(),
            ),
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            &ast::Type::Scalar(scalar_type),
        )?;
        Ok(ast::Arg3 { dst, src1, src2 })
    }
}

impl<T: ArgParamsEx> ast::Arg4<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: &ast::Type,
        is_wide: bool,
    ) -> Result<ast::Arg4<U>, TranslateError> {
        let wide_type = if is_wide {
            Some(t.clone().widen()?)
        } else {
            None
        };
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                sema: ArgumentSemantics::Default,
            },
            wide_type.as_ref().unwrap_or(t),
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            t,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            t,
        )?;
        let src3 = visitor.operand(
            ArgumentDescriptor {
                op: self.src3,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            t,
        )?;
        Ok(ast::Arg4 {
            dst,
            src1,
            src2,
            src3,
        })
    }

    fn map_selp<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: ast::SelpType,
    ) -> Result<ast::Arg4<U>, TranslateError> {
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                sema: ArgumentSemantics::Default,
            },
            &ast::Type::Scalar(t.into()),
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            &ast::Type::Scalar(t.into()),
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            &ast::Type::Scalar(t.into()),
        )?;
        let src3 = visitor.operand(
            ArgumentDescriptor {
                op: self.src3,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            &ast::Type::Scalar(ast::ScalarType::Pred),
        )?;
        Ok(ast::Arg4 {
            dst,
            src1,
            src2,
            src3,
        })
    }

    fn map_atom<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: ast::BitType,
        state_space: ast::AtomSpace,
    ) -> Result<ast::Arg4<U>, TranslateError> {
        let scalar_type = ast::ScalarType::from(t);
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                sema: ArgumentSemantics::Default,
            },
            &ast::Type::Scalar(scalar_type),
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                sema: ArgumentSemantics::PhysicalPointer,
            },
            &ast::Type::Pointer(
                ast::PointerType::Scalar(scalar_type),
                state_space.to_ld_ss(),
            ),
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            &ast::Type::Scalar(scalar_type),
        )?;
        let src3 = visitor.operand(
            ArgumentDescriptor {
                op: self.src3,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            &ast::Type::Scalar(scalar_type),
        )?;
        Ok(ast::Arg4 {
            dst,
            src1,
            src2,
            src3,
        })
    }

    fn map_bfe<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        typ: &ast::Type,
    ) -> Result<ast::Arg4<U>, TranslateError> {
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                sema: ArgumentSemantics::Default,
            },
            typ,
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            typ,
        )?;
        let u32_type = ast::Type::Scalar(ast::ScalarType::U32);
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            &u32_type,
        )?;
        let src3 = visitor.operand(
            ArgumentDescriptor {
                op: self.src3,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            &u32_type,
        )?;
        Ok(ast::Arg4 {
            dst,
            src1,
            src2,
            src3,
        })
    }
}

impl<T: ArgParamsEx> ast::Arg4Setp<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: &ast::Type,
    ) -> Result<ast::Arg4Setp<U>, TranslateError> {
        let dst1 = visitor.id(
            ArgumentDescriptor {
                op: self.dst1,
                is_dst: true,
                sema: ArgumentSemantics::Default,
            },
            Some(&ast::Type::Scalar(ast::ScalarType::Pred)),
        )?;
        let dst2 = self
            .dst2
            .map(|dst2| {
                visitor.id(
                    ArgumentDescriptor {
                        op: dst2,
                        is_dst: true,
                        sema: ArgumentSemantics::Default,
                    },
                    Some(&ast::Type::Scalar(ast::ScalarType::Pred)),
                )
            })
            .transpose()?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            t,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            t,
        )?;
        Ok(ast::Arg4Setp {
            dst1,
            dst2,
            src1,
            src2,
        })
    }
}

impl<T: ArgParamsEx> ast::Arg5<T> {
    fn map_bfi<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        base_type: &ast::Type,
    ) -> Result<ast::Arg5<U>, TranslateError> {
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                sema: ArgumentSemantics::Default,
            },
            base_type,
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            base_type,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            base_type,
        )?;
        let src3 = visitor.operand(
            ArgumentDescriptor {
                op: self.src3,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            &ast::Type::Scalar(ast::ScalarType::U32),
        )?;
        let src4 = visitor.operand(
            ArgumentDescriptor {
                op: self.src4,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            &ast::Type::Scalar(ast::ScalarType::U32),
        )?;
        Ok(ast::Arg5 {
            dst,
            src1,
            src2,
            src3,
            src4,
        })
    }
}

impl<T: ArgParamsEx> ast::Arg5Setp<T> {
    fn map<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: &ast::Type,
    ) -> Result<ast::Arg5Setp<U>, TranslateError> {
        let dst1 = visitor.id(
            ArgumentDescriptor {
                op: self.dst1,
                is_dst: true,
                sema: ArgumentSemantics::Default,
            },
            Some(&ast::Type::Scalar(ast::ScalarType::Pred)),
        )?;
        let dst2 = self
            .dst2
            .map(|dst2| {
                visitor.id(
                    ArgumentDescriptor {
                        op: dst2,
                        is_dst: true,
                        sema: ArgumentSemantics::Default,
                    },
                    Some(&ast::Type::Scalar(ast::ScalarType::Pred)),
                )
            })
            .transpose()?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            t,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            t,
        )?;
        let src3 = visitor.operand(
            ArgumentDescriptor {
                op: self.src3,
                is_dst: false,
                sema: ArgumentSemantics::Default,
            },
            &ast::Type::Scalar(ast::ScalarType::Pred),
        )?;
        Ok(ast::Arg5Setp {
            dst1,
            dst2,
            src1,
            src2,
            src3,
        })
    }
}

impl<T> ast::Operand<T> {
    fn map_variable<U, F: FnMut(T) -> Result<U, TranslateError>>(
        self,
        f: &mut F,
    ) -> Result<ast::Operand<U>, TranslateError> {
        Ok(match self {
            ast::Operand::Reg(reg) => ast::Operand::Reg(f(reg)?),
            ast::Operand::RegOffset(reg, offset) => ast::Operand::RegOffset(f(reg)?, offset),
            ast::Operand::Imm(x) => ast::Operand::Imm(x),
            ast::Operand::VecMember(reg, idx) => ast::Operand::VecMember(f(reg)?, idx),
            ast::Operand::VecPack(vec) => {
                ast::Operand::VecPack(vec.into_iter().map(f).collect::<Result<_, _>>()?)
            }
        })
    }
}

impl ast::Operand<spirv::Word> {
    fn unwrap_reg(&self) -> Result<spirv::Word, TranslateError> {
        match self {
            ast::Operand::Reg(reg) => Ok(*reg),
            _ => Err(error_unreachable()),
        }
    }
}

impl ast::StStateSpace {
    fn to_ld_ss(self) -> ast::LdStateSpace {
        match self {
            ast::StStateSpace::Generic => ast::LdStateSpace::Generic,
            ast::StStateSpace::Global => ast::LdStateSpace::Global,
            ast::StStateSpace::Local => ast::LdStateSpace::Local,
            ast::StStateSpace::Param => ast::LdStateSpace::Param,
            ast::StStateSpace::Shared => ast::LdStateSpace::Shared,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ScalarKind {
    Bit,
    Unsigned,
    Signed,
    Float,
    Float2,
    Pred,
}

impl ast::ScalarType {
    fn kind(self) -> ScalarKind {
        match self {
            ast::ScalarType::U8 => ScalarKind::Unsigned,
            ast::ScalarType::U16 => ScalarKind::Unsigned,
            ast::ScalarType::U32 => ScalarKind::Unsigned,
            ast::ScalarType::U64 => ScalarKind::Unsigned,
            ast::ScalarType::S8 => ScalarKind::Signed,
            ast::ScalarType::S16 => ScalarKind::Signed,
            ast::ScalarType::S32 => ScalarKind::Signed,
            ast::ScalarType::S64 => ScalarKind::Signed,
            ast::ScalarType::B8 => ScalarKind::Bit,
            ast::ScalarType::B16 => ScalarKind::Bit,
            ast::ScalarType::B32 => ScalarKind::Bit,
            ast::ScalarType::B64 => ScalarKind::Bit,
            ast::ScalarType::F16 => ScalarKind::Float,
            ast::ScalarType::F32 => ScalarKind::Float,
            ast::ScalarType::F64 => ScalarKind::Float,
            ast::ScalarType::F16x2 => ScalarKind::Float2,
            ast::ScalarType::Pred => ScalarKind::Pred,
        }
    }

    fn from_parts(width: u8, kind: ScalarKind) -> Self {
        match kind {
            ScalarKind::Float => match width {
                2 => ast::ScalarType::F16,
                4 => ast::ScalarType::F32,
                8 => ast::ScalarType::F64,
                _ => unreachable!(),
            },
            ScalarKind::Bit => match width {
                1 => ast::ScalarType::B8,
                2 => ast::ScalarType::B16,
                4 => ast::ScalarType::B32,
                8 => ast::ScalarType::B64,
                _ => unreachable!(),
            },
            ScalarKind::Signed => match width {
                1 => ast::ScalarType::S8,
                2 => ast::ScalarType::S16,
                4 => ast::ScalarType::S32,
                8 => ast::ScalarType::S64,
                _ => unreachable!(),
            },
            ScalarKind::Unsigned => match width {
                1 => ast::ScalarType::U8,
                2 => ast::ScalarType::U16,
                4 => ast::ScalarType::U32,
                8 => ast::ScalarType::U64,
                _ => unreachable!(),
            },
            ScalarKind::Float2 => match width {
                4 => ast::ScalarType::F16x2,
                _ => unreachable!(),
            },
            ScalarKind::Pred => ast::ScalarType::Pred,
        }
    }
}

impl ast::BooleanType {
    fn to_type(self) -> ast::Type {
        match self {
            ast::BooleanType::Pred => ast::Type::Scalar(ast::ScalarType::Pred),
            ast::BooleanType::B16 => ast::Type::Scalar(ast::ScalarType::B16),
            ast::BooleanType::B32 => ast::Type::Scalar(ast::ScalarType::B32),
            ast::BooleanType::B64 => ast::Type::Scalar(ast::ScalarType::B64),
        }
    }
}

impl ast::ShlType {
    fn to_type(self) -> ast::Type {
        match self {
            ast::ShlType::B16 => ast::Type::Scalar(ast::ScalarType::B16),
            ast::ShlType::B32 => ast::Type::Scalar(ast::ScalarType::B32),
            ast::ShlType::B64 => ast::Type::Scalar(ast::ScalarType::B64),
        }
    }
}

impl ast::ShrType {
    fn signed(&self) -> bool {
        match self {
            ast::ShrType::S16 | ast::ShrType::S32 | ast::ShrType::S64 => true,
            _ => false,
        }
    }
}

impl ast::ArithDetails {
    fn get_type(&self) -> ast::Type {
        ast::Type::Scalar(match self {
            ast::ArithDetails::Unsigned(t) => (*t).into(),
            ast::ArithDetails::Signed(d) => d.typ.into(),
            ast::ArithDetails::Float(d) => d.typ.into(),
        })
    }
}

impl ast::MulDetails {
    fn get_type(&self) -> ast::Type {
        ast::Type::Scalar(match self {
            ast::MulDetails::Unsigned(d) => d.typ.into(),
            ast::MulDetails::Signed(d) => d.typ.into(),
            ast::MulDetails::Float(d) => d.typ.into(),
        })
    }
}

impl ast::MinMaxDetails {
    fn get_type(&self) -> ast::Type {
        ast::Type::Scalar(match self {
            ast::MinMaxDetails::Signed(t) => (*t).into(),
            ast::MinMaxDetails::Unsigned(t) => (*t).into(),
            ast::MinMaxDetails::Float(d) => d.typ.into(),
        })
    }
}

impl ast::DivDetails {
    fn get_type(&self) -> ast::Type {
        ast::Type::Scalar(match self {
            ast::DivDetails::Unsigned(t) => (*t).into(),
            ast::DivDetails::Signed(t) => (*t).into(),
            ast::DivDetails::Float(d) => d.typ.into(),
        })
    }
}

impl ast::AtomInnerDetails {
    fn get_type(&self) -> ast::ScalarType {
        match self {
            ast::AtomInnerDetails::Bit { typ, .. } => (*typ).into(),
            ast::AtomInnerDetails::Unsigned { typ, .. } => (*typ).into(),
            ast::AtomInnerDetails::Signed { typ, .. } => (*typ).into(),
            ast::AtomInnerDetails::Float { typ, .. } => (*typ).into(),
        }
    }
}

impl ast::SIntType {
    fn from_size(width: u8) -> Self {
        match width {
            1 => ast::SIntType::S8,
            2 => ast::SIntType::S16,
            4 => ast::SIntType::S32,
            8 => ast::SIntType::S64,
            _ => unreachable!(),
        }
    }
}

impl ast::UIntType {
    fn from_size(width: u8) -> Self {
        match width {
            1 => ast::UIntType::U8,
            2 => ast::UIntType::U16,
            4 => ast::UIntType::U32,
            8 => ast::UIntType::U64,
            _ => unreachable!(),
        }
    }
}

impl ast::LdStateSpace {
    fn to_spirv(self) -> spirv::StorageClass {
        match self {
            ast::LdStateSpace::Const => spirv::StorageClass::UniformConstant,
            ast::LdStateSpace::Generic => spirv::StorageClass::Generic,
            ast::LdStateSpace::Global => spirv::StorageClass::CrossWorkgroup,
            ast::LdStateSpace::Local => spirv::StorageClass::Function,
            ast::LdStateSpace::Shared => spirv::StorageClass::Workgroup,
            ast::LdStateSpace::Param => spirv::StorageClass::Function,
        }
    }
}

impl From<ast::FnArgumentType> for ast::VariableType {
    fn from(t: ast::FnArgumentType) -> Self {
        match t {
            ast::FnArgumentType::Reg(t) => ast::VariableType::Reg(t),
            ast::FnArgumentType::Param(t) => ast::VariableType::Param(t),
            ast::FnArgumentType::Shared => todo!(),
        }
    }
}

impl<T> ast::Operand<T> {
    fn underlying(&self) -> Option<&T> {
        match self {
            ast::Operand::Reg(r) | ast::Operand::RegOffset(r, _) => Some(r),
            ast::Operand::Imm(_) => None,
            ast::Operand::VecMember(reg, _) => Some(reg),
            ast::Operand::VecPack(..) => None,
        }
    }
}

impl ast::MulDetails {
    fn is_wide(&self) -> bool {
        match self {
            ast::MulDetails::Unsigned(d) => d.control == ast::MulIntControl::Wide,
            ast::MulDetails::Signed(d) => d.control == ast::MulIntControl::Wide,
            ast::MulDetails::Float(_) => false,
        }
    }
}

impl ast::AtomSpace {
    fn to_ld_ss(self) -> ast::LdStateSpace {
        match self {
            ast::AtomSpace::Generic => ast::LdStateSpace::Generic,
            ast::AtomSpace::Global => ast::LdStateSpace::Global,
            ast::AtomSpace::Shared => ast::LdStateSpace::Shared,
        }
    }
}

impl ast::MemScope {
    fn to_spirv(self) -> spirv::Scope {
        match self {
            ast::MemScope::Cta => spirv::Scope::Workgroup,
            ast::MemScope::Gpu => spirv::Scope::Device,
            ast::MemScope::Sys => spirv::Scope::CrossDevice,
        }
    }
}

impl ast::AtomSemantics {
    fn to_spirv(self) -> spirv::MemorySemantics {
        match self {
            ast::AtomSemantics::Relaxed => spirv::MemorySemantics::RELAXED,
            ast::AtomSemantics::Acquire => spirv::MemorySemantics::ACQUIRE,
            ast::AtomSemantics::Release => spirv::MemorySemantics::RELEASE,
            ast::AtomSemantics::AcquireRelease => spirv::MemorySemantics::ACQUIRE_RELEASE,
        }
    }
}

impl ast::FnArgumentType {
    fn semantics(&self) -> ArgumentSemantics {
        match self {
            ast::FnArgumentType::Reg(_) => ArgumentSemantics::Default,
            ast::FnArgumentType::Param(_) => ArgumentSemantics::RegisterPointer,
            ast::FnArgumentType::Shared => ArgumentSemantics::PhysicalPointer,
        }
    }
}

fn bitcast_register_pointer(
    operand_type: &ast::Type,
    instr_type: &ast::Type,
    ss: Option<ast::LdStateSpace>,
) -> Result<Option<ConversionKind>, TranslateError> {
    bitcast_physical_pointer(operand_type, instr_type, ss)
}

fn bitcast_physical_pointer(
    operand_type: &ast::Type,
    instr_type: &ast::Type,
    ss: Option<ast::LdStateSpace>,
) -> Result<Option<ConversionKind>, TranslateError> {
    match operand_type {
        // array decays to a pointer
        ast::Type::Array(op_scalar_t, _) => {
            if let ast::Type::Pointer(instr_scalar_t, instr_space) = instr_type {
                if ss == Some(*instr_space) {
                    if ast::Type::Scalar(*op_scalar_t) == ast::Type::from(instr_scalar_t.clone()) {
                        Ok(None)
                    } else {
                        Ok(Some(ConversionKind::PtrToPtr { spirv_ptr: false }))
                    }
                } else {
                    if ss == Some(ast::LdStateSpace::Generic)
                        || *instr_space == ast::LdStateSpace::Generic
                    {
                        Ok(Some(ConversionKind::PtrToPtr { spirv_ptr: false }))
                    } else {
                        Err(TranslateError::MismatchedType)
                    }
                }
            } else {
                Err(TranslateError::MismatchedType)
            }
        }
        ast::Type::Scalar(ast::ScalarType::B64)
        | ast::Type::Scalar(ast::ScalarType::U64)
        | ast::Type::Scalar(ast::ScalarType::S64) => {
            if let Some(space) = ss {
                Ok(Some(ConversionKind::BitToPtr(space)))
            } else {
                Err(error_unreachable())
            }
        }
        ast::Type::Scalar(ast::ScalarType::B32)
        | ast::Type::Scalar(ast::ScalarType::U32)
        | ast::Type::Scalar(ast::ScalarType::S32) => match ss {
            Some(ast::LdStateSpace::Shared)
            | Some(ast::LdStateSpace::Generic)
            | Some(ast::LdStateSpace::Param)
            | Some(ast::LdStateSpace::Local) => {
                Ok(Some(ConversionKind::BitToPtr(ast::LdStateSpace::Shared)))
            }
            _ => Err(TranslateError::MismatchedType),
        },
        ast::Type::Pointer(op_scalar_t, op_space) => {
            if let ast::Type::Pointer(instr_scalar_t, instr_space) = instr_type {
                if op_space == instr_space {
                    if op_scalar_t == instr_scalar_t {
                        Ok(None)
                    } else {
                        Ok(Some(ConversionKind::PtrToPtr { spirv_ptr: false }))
                    }
                } else {
                    if *op_space == ast::LdStateSpace::Generic
                        || *instr_space == ast::LdStateSpace::Generic
                    {
                        Ok(Some(ConversionKind::PtrToPtr { spirv_ptr: false }))
                    } else {
                        Err(TranslateError::MismatchedType)
                    }
                }
            } else {
                Err(TranslateError::MismatchedType)
            }
        }
        _ => Err(TranslateError::MismatchedType),
    }
}

fn force_bitcast_ptr_to_bit(
    _: &ast::Type,
    instr_type: &ast::Type,
    _: Option<ast::LdStateSpace>,
) -> Result<Option<ConversionKind>, TranslateError> {
    // TODO: verify this on f32, u16 and the like
    if let ast::Type::Scalar(scalar_t) = instr_type {
        if let Ok(int_type) = (*scalar_t).try_into() {
            return Ok(Some(ConversionKind::PtrToBit(int_type)));
        }
    }
    Err(TranslateError::MismatchedType)
}

fn should_bitcast(instr: &ast::Type, operand: &ast::Type) -> bool {
    match (instr, operand) {
        (ast::Type::Scalar(inst), ast::Type::Scalar(operand)) => {
            if inst.size_of() != operand.size_of() {
                return false;
            }
            match inst.kind() {
                ScalarKind::Bit => operand.kind() != ScalarKind::Bit,
                ScalarKind::Float => operand.kind() == ScalarKind::Bit,
                ScalarKind::Signed => {
                    operand.kind() == ScalarKind::Bit || operand.kind() == ScalarKind::Unsigned
                }
                ScalarKind::Unsigned => {
                    operand.kind() == ScalarKind::Bit || operand.kind() == ScalarKind::Signed
                }
                ScalarKind::Float2 => false,
                ScalarKind::Pred => false,
            }
        }
        (ast::Type::Vector(inst, _), ast::Type::Vector(operand, _))
        | (ast::Type::Array(inst, _), ast::Type::Array(operand, _)) => {
            should_bitcast(&ast::Type::Scalar(*inst), &ast::Type::Scalar(*operand))
        }
        _ => false,
    }
}

fn should_bitcast_packed(
    operand: &ast::Type,
    instr: &ast::Type,
    ss: Option<ast::LdStateSpace>,
) -> Result<Option<ConversionKind>, TranslateError> {
    if let (ast::Type::Vector(vec_underlying_type, vec_len), ast::Type::Scalar(scalar)) =
        (operand, instr)
    {
        if scalar.kind() == ScalarKind::Bit
            && scalar.size_of() == (vec_underlying_type.size_of() * vec_len)
        {
            return Ok(Some(ConversionKind::Default));
        }
    }
    should_bitcast_wrapper(operand, instr, ss)
}

fn should_bitcast_wrapper(
    operand: &ast::Type,
    instr: &ast::Type,
    _: Option<ast::LdStateSpace>,
) -> Result<Option<ConversionKind>, TranslateError> {
    if instr == operand {
        return Ok(None);
    }
    if should_bitcast(instr, operand) {
        Ok(Some(ConversionKind::Default))
    } else {
        Err(TranslateError::MismatchedType)
    }
}

fn should_convert_relaxed_src_wrapper(
    src_type: &ast::Type,
    instr_type: &ast::Type,
    _: Option<ast::LdStateSpace>,
) -> Result<Option<ConversionKind>, TranslateError> {
    if src_type == instr_type {
        return Ok(None);
    }
    match should_convert_relaxed_src(src_type, instr_type) {
        conv @ Some(_) => Ok(conv),
        None => Err(TranslateError::MismatchedType),
    }
}

// https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#operand-size-exceeding-instruction-type-size__relaxed-type-checking-rules-source-operands
fn should_convert_relaxed_src(
    src_type: &ast::Type,
    instr_type: &ast::Type,
) -> Option<ConversionKind> {
    if src_type == instr_type {
        return None;
    }
    match (src_type, instr_type) {
        (ast::Type::Scalar(src_type), ast::Type::Scalar(instr_type)) => match instr_type.kind() {
            ScalarKind::Bit => {
                if instr_type.size_of() <= src_type.size_of() {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
            ScalarKind::Signed | ScalarKind::Unsigned => {
                if instr_type.size_of() <= src_type.size_of()
                    && src_type.kind() != ScalarKind::Float
                {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
            ScalarKind::Float => {
                if instr_type.size_of() <= src_type.size_of() && src_type.kind() == ScalarKind::Bit
                {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
            ScalarKind::Float2 => todo!(),
            ScalarKind::Pred => None,
        },
        (ast::Type::Vector(dst_type, _), ast::Type::Vector(instr_type, _))
        | (ast::Type::Array(dst_type, _), ast::Type::Array(instr_type, _)) => {
            should_convert_relaxed_src(
                &ast::Type::Scalar(*dst_type),
                &ast::Type::Scalar(*instr_type),
            )
        }
        _ => None,
    }
}

fn should_convert_relaxed_dst_wrapper(
    dst_type: &ast::Type,
    instr_type: &ast::Type,
    _: Option<ast::LdStateSpace>,
) -> Result<Option<ConversionKind>, TranslateError> {
    if dst_type == instr_type {
        return Ok(None);
    }
    match should_convert_relaxed_dst(dst_type, instr_type) {
        conv @ Some(_) => Ok(conv),
        None => Err(TranslateError::MismatchedType),
    }
}

// https://docs.nvidia.com/cuda/parallel-thread-execution/index.html#operand-size-exceeding-instruction-type-size__relaxed-type-checking-rules-destination-operands
fn should_convert_relaxed_dst(
    dst_type: &ast::Type,
    instr_type: &ast::Type,
) -> Option<ConversionKind> {
    if dst_type == instr_type {
        return None;
    }
    match (dst_type, instr_type) {
        (ast::Type::Scalar(dst_type), ast::Type::Scalar(instr_type)) => match instr_type.kind() {
            ScalarKind::Bit => {
                if instr_type.size_of() <= dst_type.size_of() {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
            ScalarKind::Signed => {
                if dst_type.kind() != ScalarKind::Float {
                    if instr_type.size_of() == dst_type.size_of() {
                        Some(ConversionKind::Default)
                    } else if instr_type.size_of() < dst_type.size_of() {
                        Some(ConversionKind::SignExtend)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            ScalarKind::Unsigned => {
                if instr_type.size_of() <= dst_type.size_of()
                    && dst_type.kind() != ScalarKind::Float
                {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
            ScalarKind::Float => {
                if instr_type.size_of() <= dst_type.size_of() && dst_type.kind() == ScalarKind::Bit
                {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
            ScalarKind::Float2 => todo!(),
            ScalarKind::Pred => None,
        },
        (ast::Type::Vector(dst_type, _), ast::Type::Vector(instr_type, _))
        | (ast::Type::Array(dst_type, _), ast::Type::Array(instr_type, _)) => {
            should_convert_relaxed_dst(
                &ast::Type::Scalar(*dst_type),
                &ast::Type::Scalar(*instr_type),
            )
        }
        _ => None,
    }
}

impl<'a> ast::MethodDecl<'a, &'a str> {
    fn name(&self) -> &'a str {
        match self {
            ast::MethodDecl::Kernel { name, .. } => name,
            ast::MethodDecl::Func(_, name, _) => name,
        }
    }
}

struct SpirvMethodDecl<'input> {
    input: Vec<ast::Variable<ast::Type, spirv::Word>>,
    output: Vec<ast::Variable<ast::Type, spirv::Word>>,
    name: MethodName<'input>,
    uses_shared_mem: bool,
}

impl<'input> SpirvMethodDecl<'input> {
    fn new(ast_decl: &ast::MethodDecl<'input, spirv::Word>) -> Self {
        let (input, output) = match ast_decl {
            ast::MethodDecl::Kernel { in_args, .. } => {
                let spirv_input = in_args
                    .iter()
                    .map(|var| {
                        let v_type = match &var.v_type {
                            ast::KernelArgumentType::Normal(t) => {
                                ast::FnArgumentType::Param(t.clone())
                            }
                            ast::KernelArgumentType::Shared => ast::FnArgumentType::Shared,
                        };
                        ast::Variable {
                            name: var.name,
                            align: var.align,
                            v_type: v_type.to_kernel_type(),
                            array_init: var.array_init.clone(),
                        }
                    })
                    .collect();
                (spirv_input, Vec::new())
            }
            ast::MethodDecl::Func(out_args, _, in_args) => {
                let (param_output, non_param_output): (Vec<_>, Vec<_>) =
                    out_args.iter().partition(|var| var.v_type.is_param());
                let spirv_output = non_param_output
                    .into_iter()
                    .cloned()
                    .map(|var| ast::Variable {
                        name: var.name,
                        align: var.align,
                        v_type: var.v_type.to_func_type(),
                        array_init: var.array_init.clone(),
                    })
                    .collect();
                let spirv_input = param_output
                    .into_iter()
                    .cloned()
                    .chain(in_args.iter().cloned())
                    .map(|var| ast::Variable {
                        name: var.name,
                        align: var.align,
                        v_type: var.v_type.to_func_type(),
                        array_init: var.array_init.clone(),
                    })
                    .collect();
                (spirv_input, spirv_output)
            }
        };
        SpirvMethodDecl {
            input,
            output,
            name: MethodName::new(ast_decl),
            uses_shared_mem: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast;

    static SCALAR_TYPES: [ast::ScalarType; 15] = [
        ast::ScalarType::B8,
        ast::ScalarType::B16,
        ast::ScalarType::B32,
        ast::ScalarType::B64,
        ast::ScalarType::S8,
        ast::ScalarType::S16,
        ast::ScalarType::S32,
        ast::ScalarType::S64,
        ast::ScalarType::U8,
        ast::ScalarType::U16,
        ast::ScalarType::U32,
        ast::ScalarType::U64,
        ast::ScalarType::F16,
        ast::ScalarType::F32,
        ast::ScalarType::F64,
    ];

    static RELAXED_SRC_CONVERSION_TABLE: &'static str =
        "b8 	- 	chop 	chop 	chop 	- 	chop 	chop 	chop 	- 	chop 	chop 	chop 	chop 	chop 	chop
        b16 	inv 	- 	chop 	chop 	inv 	- 	chop 	chop 	inv 	- 	chop 	chop 	- 	chop 	chop
        b32 	inv 	inv 	- 	chop 	inv 	inv 	- 	chop 	inv 	inv 	- 	chop 	inv 	- 	chop
        b64 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	-
        s8 	- 	chop 	chop 	chop 	- 	chop 	chop 	chop 	- 	chop 	chop 	chop 	inv 	inv 	inv
        s16 	inv 	- 	chop 	chop 	inv 	- 	chop 	chop 	inv 	- 	chop 	chop 	inv 	inv 	inv
        s32 	inv 	inv 	- 	chop 	inv 	inv 	- 	chop 	inv 	inv 	- 	chop 	inv 	inv 	inv
        s64 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	inv
        u8 	- 	chop 	chop 	chop 	- 	chop 	chop 	chop 	- 	chop 	chop 	chop 	inv 	inv 	inv
        u16 	inv 	- 	chop 	chop 	inv 	- 	chop 	chop 	inv 	- 	chop 	chop 	inv 	inv 	inv
        u32 	inv 	inv 	- 	chop 	inv 	inv 	- 	chop 	inv 	inv 	- 	chop 	inv 	inv 	inv
        u64 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	inv
        f16 	inv 	- 	chop 	chop 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	- 	inv 	inv
        f32 	inv 	inv 	- 	chop 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	- 	inv
        f64 	inv 	inv 	inv 	- 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	-";

    static RELAXED_DST_CONVERSION_TABLE: &'static str =
        "b8 	- 	zext 	zext 	zext 	- 	zext 	zext 	zext 	- 	zext 	zext 	zext 	zext 	zext 	zext
        b16 	inv 	- 	zext 	zext 	inv 	- 	zext 	zext 	inv 	- 	zext 	zext 	- 	zext 	zext
        b32 	inv 	inv 	- 	zext 	inv 	inv 	- 	zext 	inv 	inv 	- 	zext 	inv 	- 	zext
        b64 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	-
        s8 	- 	sext 	sext 	sext 	- 	sext 	sext 	sext 	- 	sext 	sext 	sext 	inv 	inv 	inv
        s16 	inv 	- 	sext 	sext 	inv 	- 	sext 	sext 	inv 	- 	sext 	sext 	inv 	inv 	inv
        s32 	inv 	inv 	- 	sext 	inv 	inv 	- 	sext 	inv 	inv 	- 	sext 	inv 	inv 	inv
        s64 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	inv
        u8 	- 	zext 	zext 	zext 	- 	zext 	zext 	zext 	- 	zext 	zext 	zext 	inv 	inv 	inv
        u16 	inv 	- 	zext 	zext 	inv 	- 	zext 	zext 	inv 	- 	zext 	zext 	inv 	inv 	inv
        u32 	inv 	inv 	- 	zext 	inv 	inv 	- 	zext 	inv 	inv 	- 	zext 	inv 	inv 	inv
        u64 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	inv 	- 	inv 	inv 	inv
        f16 	inv 	- 	zext 	zext 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	- 	inv 	inv
        f32 	inv 	inv 	- 	zext 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	- 	inv
        f64 	inv 	inv 	inv 	- 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	inv 	-";

    fn table_entry_to_conversion(entry: &'static str) -> Option<ConversionKind> {
        match entry {
            "-" => Some(ConversionKind::Default),
            "inv" => None,
            "zext" => Some(ConversionKind::Default),
            "chop" => Some(ConversionKind::Default),
            "sext" => Some(ConversionKind::SignExtend),
            _ => unreachable!(),
        }
    }

    fn parse_conversion_table(table: &'static str) -> Vec<Vec<Option<ConversionKind>>> {
        table
            .lines()
            .map(|line| {
                line.split_ascii_whitespace()
                    .skip(1)
                    .map(table_entry_to_conversion)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    fn assert_conversion_table<F: Fn(&ast::Type, &ast::Type) -> Option<ConversionKind>>(
        table: &'static str,
        f: F,
    ) {
        let conv_table = parse_conversion_table(table);
        for (instr_idx, instr_type) in SCALAR_TYPES.iter().enumerate() {
            for (op_idx, op_type) in SCALAR_TYPES.iter().enumerate() {
                let conversion = f(
                    &ast::Type::Scalar(*op_type),
                    &ast::Type::Scalar(*instr_type),
                );
                if instr_idx == op_idx {
                    assert!(conversion == None);
                } else {
                    assert!(conversion == conv_table[instr_idx][op_idx]);
                }
            }
        }
    }

    #[test]
    fn should_convert_relaxed_src_all_combinations() {
        assert_conversion_table(RELAXED_SRC_CONVERSION_TABLE, should_convert_relaxed_src);
    }

    #[test]
    fn should_convert_relaxed_dst_all_combinations() {
        assert_conversion_table(RELAXED_DST_CONVERSION_TABLE, should_convert_relaxed_dst);
    }
}
