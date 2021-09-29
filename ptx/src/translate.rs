use crate::ast;
use half::f16;
use rspirv::dr;
use std::cell::RefCell;
use std::collections::{hash_map, BTreeMap, HashMap, HashSet};
use std::{borrow::Cow, collections::BTreeSet, ffi::CString, hash::Hash, iter, mem, rc::Rc};

use rspirv::binary::{Assemble, Disassemble};

static ZLUDA_PTX_IMPL_INTEL: &'static [u8] = include_bytes!("../lib/zluda_ptx_impl.spv");
static ZLUDA_PTX_IMPL_AMD: &'static [u8] = include_bytes!("../lib/zluda_ptx_impl.bc");
const ZLUDA_PTX_PREFIX: &'static str = "__zluda_ptx_impl__";

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

fn error_unknown_symbol() -> TranslateError {
    TranslateError::UnknownSymbol
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
    fn new(t: ast::Type) -> Self {
        match t {
            ast::Type::Scalar(t) => SpirvType::Base(t.into()),
            ast::Type::Vector(typ, len) => SpirvType::Vector(typ.into(), len),
            ast::Type::Array(t, len) => SpirvType::Array(t.into(), len),
            ast::Type::Pointer(pointer_t, space) => SpirvType::Pointer(
                Box::new(SpirvType::Base(pointer_t.into())),
                space.to_spirv(),
            ),
        }
    }

    fn pointer_to(t: ast::Type, outer_space: spirv::StorageClass) -> Self {
        let key = Self::new(t);
        SpirvType::Pointer(Box::new(key), outer_space)
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
        let void = b.type_void(None);
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
                SpirvScalarKey::B8 => b.type_int(None, 8, 0),
                SpirvScalarKey::B16 => b.type_int(None, 16, 0),
                SpirvScalarKey::B32 => b.type_int(None, 32, 0),
                SpirvScalarKey::B64 => b.type_int(None, 64, 0),
                SpirvScalarKey::F16 => b.type_float(None, 16),
                SpirvScalarKey::F32 => b.type_float(None, 32),
                SpirvScalarKey::F64 => b.type_float(None, 64),
                SpirvScalarKey::Pred => b.type_bool(None),
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
                    .or_insert_with(|| b.type_vector(None, base, len as u32))
            }
            SpirvType::Array(typ, array_dimensions) => {
                let (base_type, length) = match &*array_dimensions {
                    &[] => {
                        return self.get_or_add(b, SpirvType::Base(typ));
                    }
                    &[len] => {
                        let u32_type = self.get_or_add_scalar(b, ast::ScalarType::U32);
                        let base = self.get_or_add_spirv_scalar(b, typ);
                        let len_const = b.constant_u32(u32_type, None, len);
                        (base, len_const)
                    }
                    array_dimensions => {
                        let u32_type = self.get_or_add_scalar(b, ast::ScalarType::U32);
                        let base = self
                            .get_or_add(b, SpirvType::Array(typ, array_dimensions[1..].to_vec()));
                        let len_const = b.constant_u32(u32_type, None, array_dimensions[0]);
                        (base, len_const)
                    }
                };
                *self
                    .complex
                    .entry(SpirvType::Array(typ, array_dimensions))
                    .or_insert_with(|| b.type_array(None, base_type, length))
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
                    .or_insert_with(|| b.type_function(None, out_t, in_t))
            }
            SpirvType::Struct(ref underlying) => {
                let underlying_ids = underlying
                    .iter()
                    .map(|t| self.get_or_add_spirv_scalar(b, *t))
                    .collect::<Vec<_>>();
                *self
                    .complex
                    .entry(t)
                    .or_insert_with(|| b.type_struct(None, underlying_ids))
            }
        }
    }

    fn get_or_add_fn(
        &mut self,
        b: &mut dr::Builder,
        in_params: impl Iterator<Item = SpirvType>,
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
            // TODO: support multiple return values
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
                b.constant_composite(result_type, None, components.into_iter())
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
                    b.constant_composite(result_type, None, components.into_iter())
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
                    b.constant_composite(result_type, None, components.into_iter())
                }
            },
            ast::Type::Pointer(..) => return Err(error_unreachable()),
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
    pub should_link_ptx_impl: Option<(&'static [u8], &'static [u8])>,
    pub build_options: CString,
}
impl Module {
    pub fn assemble(&self) -> Vec<u32> {
        self.spirv.assemble()
    }
}

pub struct KernelInfo {
    pub arguments_sizes: Vec<(usize, bool)>,
    pub uses_shared_mem: bool,
}

pub fn to_spirv_module<'input>(ast: ast::Module<'input>) -> Result<Module, TranslateError> {
    let mut id_defs = GlobalStringIdResolver::<'input>::new(1);
    let mut ptx_impl_imports = HashMap::new();
    let directives = ast
        .directives
        .into_iter()
        .filter_map(|directive| {
            translate_directive(&mut id_defs, &mut ptx_impl_imports, directive).transpose()
        })
        .collect::<Result<Vec<_>, _>>()?;
    let directives = hoist_function_globals(directives);
    let must_link_ptx_impl = ptx_impl_imports.len() > 0;
    let mut directives = ptx_impl_imports
        .into_iter()
        .map(|(_, v)| v)
        .chain(directives.into_iter())
        .collect::<Vec<_>>();
    let mut builder = dr::Builder::new();
    builder.reserve_ids(id_defs.current_id());
    let call_map = MethodsCallMap::new(&directives);
    let mut directives =
        convert_dynamic_shared_memory_usage(directives, &call_map, &mut || builder.id());
    normalize_variable_decls(&mut directives);
    let denorm_information = compute_denorm_information(&directives);
    // https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_logicallayout_a_logical_layout_of_a_module
    builder.set_version(1, 3);
    emit_capabilities(&mut builder);
    emit_extensions(&mut builder);
    let opencl_id = emit_opencl_import(&mut builder);
    emit_memory_model(&mut builder);
    let mut map = TypeWordMap::new(&mut builder);
    //emit_builtins(&mut builder, &mut map, &id_defs);
    let mut kernel_info = HashMap::new();
    let (build_options, should_flush_denorms) =
        emit_denorm_build_string(&call_map, &denorm_information);
    let (directives, globals_use_map) = get_globals_use_map(directives);
    emit_directives(
        &mut builder,
        &mut map,
        &id_defs,
        opencl_id,
        should_flush_denorms,
        &call_map,
        globals_use_map,
        directives,
        &mut kernel_info,
    )?;
    let spirv = builder.module();
    Ok(Module {
        spirv,
        kernel_info,
        should_link_ptx_impl: if must_link_ptx_impl {
            Some((ZLUDA_PTX_IMPL_INTEL, ZLUDA_PTX_IMPL_AMD))
        } else {
            None
        },
        build_options,
    })
}

fn get_globals_use_map<'input>(
    directives: Vec<Directive<'input>>,
) -> (
    Vec<Directive<'input>>,
    HashMap<ast::MethodName<'input, spirv::Word>, HashSet<spirv::Word>>,
) {
    let mut known_globals = HashSet::new();
    for directive in directives.iter() {
        match directive {
            Directive::Variable(_, ast::Variable { name, .. }) => {
                known_globals.insert(*name);
            }
            Directive::Method(..) => {}
        }
    }
    let mut symbol_uses_map = HashMap::new();
    let directives = directives
        .into_iter()
        .map(|directive| match directive {
            Directive::Variable(..) | Directive::Method(Function { body: None, .. }) => directive,
            Directive::Method(Function {
                func_decl,
                body: Some(mut statements),
                globals,
                import_as,
                tuning,
                linkage,
            }) => {
                let method_name = func_decl.borrow().name;
                statements = statements
                    .into_iter()
                    .map(|statement| {
                        statement.map_id(&mut |symbol, _| {
                            if known_globals.contains(&symbol) {
                                multi_hash_map_append(&mut symbol_uses_map, method_name, symbol);
                            }
                            symbol
                        })
                    })
                    .collect::<Vec<_>>();
                Directive::Method(Function {
                    func_decl,
                    body: Some(statements),
                    globals,
                    import_as,
                    tuning,
                    linkage,
                })
            }
        })
        .collect::<Vec<_>>();
    (directives, symbol_uses_map)
}

fn hoist_function_globals(directives: Vec<Directive>) -> Vec<Directive> {
    let mut result = Vec::with_capacity(directives.len());
    for directive in directives {
        match directive {
            Directive::Method(method) => {
                for variable in method.globals {
                    result.push(Directive::Variable(ast::LinkingDirective::NONE, variable));
                }
                result.push(Directive::Method(Function {
                    globals: Vec::new(),
                    ..method
                }))
            }
            _ => result.push(directive),
        }
    }
    result
}

// TODO: remove this once we have pef-function support for denorms
fn emit_denorm_build_string<'input>(
    call_map: &MethodsCallMap,
    denorm_information: &HashMap<
        ast::MethodName<'input, spirv::Word>,
        HashMap<u8, (spirv::FPDenormMode, isize)>,
    >,
) -> (CString, bool) {
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
    for (kernel, children) in call_map.kernels() {
        flush_over_preserve += *denorm_counts
            .get(&ast::MethodName::Kernel(kernel))
            .unwrap_or(&0);
        for child_fn in children {
            flush_over_preserve += *denorm_counts
                .get(&ast::MethodName::Func(*child_fn))
                .unwrap_or(&0);
        }
    }
    if flush_over_preserve > 0 {
        (
            CString::new("-ze-take-global-address -ze-denorms-are-zero").unwrap(),
            true,
        )
    } else {
        (CString::new("-ze-take-global-address").unwrap(), false)
    }
}

fn emit_directives<'input>(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    id_defs: &GlobalStringIdResolver<'input>,
    opencl_id: spirv::Word,
    should_flush_denorms: bool,
    call_map: &MethodsCallMap<'input>,
    globals_use_map: HashMap<ast::MethodName<'input, spirv::Word>, HashSet<spirv::Word>>,
    directives: Vec<Directive<'input>>,
    kernel_info: &mut HashMap<String, KernelInfo>,
) -> Result<(), TranslateError> {
    let empty_body = Vec::new();
    for d in directives.iter() {
        match d {
            Directive::Variable(linking, var) => {
                emit_variable(builder, map, id_defs, *linking, &var)?;
            }
            Directive::Method(f) => {
                let f_body = match &f.body {
                    Some(f) => f,
                    None => {
                        if f.linkage.contains(ast::LinkingDirective::EXTERN) {
                            &empty_body
                        } else {
                            continue;
                        }
                    }
                };
                for var in f.globals.iter() {
                    emit_variable(builder, map, id_defs, ast::LinkingDirective::NONE, var)?;
                }
                let func_decl = (*f.func_decl).borrow();
                let fn_id = emit_function_header(
                    builder,
                    map,
                    &id_defs,
                    &*func_decl,
                    call_map,
                    &globals_use_map,
                    kernel_info,
                )?;
                if func_decl.name.is_kernel() {
                    if should_flush_denorms {
                        builder.execution_mode(
                            fn_id,
                            spirv_headers::ExecutionMode::DenormFlushToZero,
                            [16],
                        );
                        builder.execution_mode(
                            fn_id,
                            spirv_headers::ExecutionMode::DenormFlushToZero,
                            [32],
                        );
                        builder.execution_mode(
                            fn_id,
                            spirv_headers::ExecutionMode::DenormFlushToZero,
                            [64],
                        );
                    }
                    // FP contraction happens when compiling source -> PTX and is illegal at this stage (unless you force it in cuModuleLoadDataEx)
                    builder.execution_mode(fn_id, spirv_headers::ExecutionMode::ContractionOff, []);
                    for t in f.tuning.iter() {
                        match *t {
                            ast::TuningDirective::MaxNtid(nx, ny, nz) => {
                                builder.execution_mode(
                                    fn_id,
                                    spirv_headers::ExecutionMode::MaxWorkgroupSizeINTEL,
                                    [nx, ny, nz],
                                );
                            }
                            ast::TuningDirective::ReqNtid(nx, ny, nz) => {
                                builder.execution_mode(
                                    fn_id,
                                    spirv_headers::ExecutionMode::LocalSize,
                                    [nx, ny, nz],
                                );
                            }
                            // Too architecture specific
                            ast::TuningDirective::MaxNReg(..)
                            | ast::TuningDirective::MinNCtaPerSm(..) => {}
                        }
                    }
                }
                emit_function_body_ops(builder, map, id_defs, opencl_id, &f_body)?;
                emit_function_linkage(builder, id_defs, f, fn_id)?;
                builder.select_block(None)?;
                builder.end_function()?;
            }
        }
    }
    Ok(())
}

fn emit_function_linkage<'input>(
    builder: &mut dr::Builder,
    id_defs: &GlobalStringIdResolver<'input>,
    f: &Function,
    fn_name: spirv::Word,
) -> Result<(), TranslateError> {
    if f.linkage == ast::LinkingDirective::NONE {
        return Ok(());
    };
    let linking_name = match f.func_decl.borrow().name {
        // According to SPIR-V rules linkage attributes are invalid on kernels
        ast::MethodName::Kernel(..) => return Ok(()),
        ast::MethodName::Func(fn_id) => f.import_as.as_deref().map_or_else(
            || match id_defs.reverse_variables.get(&fn_id) {
                Some(fn_name) => Ok(fn_name),
                None => Err(error_unknown_symbol()),
            },
            Result::Ok,
        )?,
    };
    emit_linking_decoration(builder, id_defs, Some(linking_name), fn_name, f.linkage);
    Ok(())
}

struct MethodsCallMap<'input> {
    map: HashMap<ast::MethodName<'input, spirv::Word>, HashSet<spirv::Word>>,
}

impl<'input> MethodsCallMap<'input> {
    fn new(module: &[Directive<'input>]) -> Self {
        let mut directly_called_by = HashMap::new();
        for directive in module {
            match directive {
                Directive::Method(Function {
                    func_decl,
                    body: Some(statements),
                    ..
                }) => {
                    let call_key: ast::MethodName<_> = (**func_decl).borrow().name;
                    if let hash_map::Entry::Vacant(entry) = directly_called_by.entry(call_key) {
                        entry.insert(Vec::new());
                    }
                    for statement in statements {
                        match statement {
                            Statement::Call(call) => {
                                multi_hash_map_append(&mut directly_called_by, call_key, call.name);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        let mut result = HashMap::new();
        for (&method_key, children) in directly_called_by.iter() {
            let mut visited = HashSet::new();
            for child in children {
                Self::add_call_map_single(&directly_called_by, &mut visited, *child);
            }
            result.insert(method_key, visited);
        }
        MethodsCallMap { map: result }
    }

    fn add_call_map_single(
        directly_called_by: &HashMap<ast::MethodName<'input, spirv::Word>, Vec<spirv::Word>>,
        visited: &mut HashSet<spirv::Word>,
        current: spirv::Word,
    ) {
        if !visited.insert(current) {
            return;
        }
        if let Some(children) = directly_called_by.get(&ast::MethodName::Func(current)) {
            for child in children {
                Self::add_call_map_single(directly_called_by, visited, *child);
            }
        }
    }

    fn get_kernel_children(&self, name: &'input str) -> impl Iterator<Item = &spirv::Word> {
        self.map
            .get(&ast::MethodName::Kernel(name))
            .into_iter()
            .flatten()
    }

    fn kernels(&self) -> impl Iterator<Item = (&'input str, &HashSet<spirv::Word>)> {
        self.map
            .iter()
            .filter_map(|(method, children)| match method {
                ast::MethodName::Kernel(kernel) => Some((*kernel, children)),
                ast::MethodName::Func(..) => None,
            })
    }

    fn visit_callees(
        &self,
        method: ast::MethodName<'input, spirv::Word>,
        f: impl FnMut(spirv::Word),
    ) {
        self.map
            .get(&method)
            .into_iter()
            .flatten()
            .copied()
            .for_each(f);
    }
}

fn multi_hash_map_append<
    K: Eq + std::hash::Hash,
    V,
    Collection: std::iter::Extend<V> + std::default::Default,
>(
    m: &mut HashMap<K, Collection>,
    key: K,
    value: V,
) {
    match m.entry(key) {
        hash_map::Entry::Occupied(mut entry) => {
            entry.get_mut().extend(iter::once(value));
        }
        hash_map::Entry::Vacant(entry) => {
            entry.insert(Default::default()).extend(iter::once(value));
        }
    }
}

/*
    PTX represents dynamically allocated shared local memory as
        .extern .shared .b32 shared_mem[];
    In SPIRV/OpenCL world this is expressed as an additional argument to the kernel
    And in AMD compilation
    This pass looks for all uses of .extern .shared and converts them to
    an additional method argument
    The question is how this artificial argument should be expressed. There are
    several options:
    * Straight conversion:
        .shared .b32 shared_mem[]
    * Introduce .param_shared statespace:
        .param_shared .b32 shared_mem
        or
        .param_shared .b32 shared_mem[]
    * Introduce .shared_ptr <SCALAR> type:
        .param .shared_ptr .b32 shared_mem
    * Reuse .ptr hint:
        .param .u64 .ptr shared_mem
      This is the most tempting, but also the most nonsensical, .ptr is just a
      hint, which has no semantical meaning (and the output of our
      transformation has a semantical meaning - we emit additional
      "OpFunctionParameter ..." with type "OpTypePointer Workgroup ...")
*/
fn convert_dynamic_shared_memory_usage<'input>(
    module: Vec<Directive<'input>>,
    kernels_methods_call_map: &MethodsCallMap<'input>,
    new_id: &mut impl FnMut() -> spirv::Word,
) -> Vec<Directive<'input>> {
    let mut globals_shared = HashMap::new();
    for dir in module.iter() {
        match dir {
            Directive::Variable(
                _,
                ast::Variable {
                    state_space: ast::StateSpace::Shared,
                    name,
                    v_type,
                    ..
                },
            ) => {
                globals_shared.insert(*name, v_type.clone());
            }
            _ => {}
        }
    }
    if globals_shared.len() == 0 {
        return module;
    }
    let mut methods_to_directly_used_shared_globals = HashMap::<_, HashSet<spirv::Word>>::new();
    let module = module
        .into_iter()
        .map(|directive| match directive {
            Directive::Method(Function {
                func_decl,
                globals,
                body: Some(statements),
                import_as,
                tuning,
                linkage,
            }) => {
                let call_key = (*func_decl).borrow().name;
                let statements = statements
                    .into_iter()
                    .map(|statement| {
                        statement.map_id(&mut |id, _| {
                            if let Some(type_) = globals_shared.get(&id) {
                                methods_to_directly_used_shared_globals
                                    .entry(call_key)
                                    .or_insert_with(HashSet::new)
                                    .insert(id);
                            }
                            id
                        })
                    })
                    .collect();
                Directive::Method(Function {
                    func_decl,
                    globals,
                    body: Some(statements),
                    import_as,
                    tuning,
                    linkage,
                })
            }
            directive => directive,
        })
        .collect::<Vec<_>>();
    // If there's a chain `kernel` -> `fn1` -> `fn2`, where only `fn2` uses extern shared,
    // make sure it gets propagated to `fn1` and `kernel`
    let methods_to_indirectly_used_shared_globals = resolve_indirect_uses_of_globals_shared(
        methods_to_directly_used_shared_globals,
        kernels_methods_call_map,
    );
    // now visit every method declaration and inject those additional arguments
    let mut directives = Vec::with_capacity(module.len());
    for directive in module.into_iter() {
        match directive {
            Directive::Method(Function {
                func_decl,
                globals,
                body: Some(statements),
                import_as,
                tuning,
                linkage,
            }) => {
                let statements = {
                    let func_decl_ref = &mut (*func_decl).borrow_mut();
                    let method_name = func_decl_ref.name;
                    insert_arguments_remap_statements(
                        new_id,
                        kernels_methods_call_map,
                        &globals_shared,
                        &methods_to_indirectly_used_shared_globals,
                        method_name,
                        &mut directives,
                        func_decl_ref,
                        statements,
                    )
                };
                directives.push(Directive::Method(Function {
                    func_decl,
                    globals,
                    body: Some(statements),
                    import_as,
                    tuning,
                    linkage,
                }));
            }
            directive => directives.push(directive),
        }
    }
    directives
}

fn insert_arguments_remap_statements<'input>(
    new_id: &mut impl FnMut() -> u32,
    kernels_methods_call_map: &MethodsCallMap<'input>,
    globals_shared: &HashMap<u32, ast::Type>,
    methods_to_indirectly_used_shared_globals: &HashMap<
        ast::MethodName<'input, spirv::Word>,
        BTreeSet<spirv::Word>,
    >,
    method_name: ast::MethodName<u32>,
    result: &mut Vec<Directive>,
    func_decl_ref: &mut std::cell::RefMut<ast::MethodDeclaration<u32>>,
    statements: Vec<Statement<ast::Instruction<ExpandedArgParams>, ExpandedArgParams>>,
) -> Vec<Statement<ast::Instruction<ExpandedArgParams>, ExpandedArgParams>> {
    let remapped_globals_in_method =
        if let Some(method_globals) = methods_to_indirectly_used_shared_globals.get(&method_name) {
            match method_name {
                ast::MethodName::Func(..) => {
                    let remapped_globals = method_globals
                        .iter()
                        .map(|global| {
                            (
                                *global,
                                (
                                    new_id(),
                                    globals_shared
                                        .get(&global)
                                        .unwrap_or_else(|| todo!())
                                        .clone(),
                                ),
                            )
                        })
                        .collect::<BTreeMap<_, _>>();
                    for (_, (new_shared_global_id, shared_global_type)) in remapped_globals.iter() {
                        func_decl_ref.input_arguments.push(ast::Variable {
                            align: None,
                            v_type: shared_global_type.clone(),
                            state_space: ast::StateSpace::Shared,
                            name: *new_shared_global_id,
                            array_init: Vec::new(),
                        });
                    }
                    remapped_globals
                }
                ast::MethodName::Kernel(..) => method_globals
                    .iter()
                    .map(|global| {
                        (
                            *global,
                            (
                                *global,
                                globals_shared
                                    .get(&global)
                                    .unwrap_or_else(|| todo!())
                                    .clone(),
                            ),
                        )
                    })
                    .collect::<BTreeMap<_, _>>(),
            }
        } else {
            return statements;
        };
    replace_uses_of_shared_memory(
        new_id,
        methods_to_indirectly_used_shared_globals,
        statements,
        remapped_globals_in_method,
    )
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum GlobalSharedSize {
    ExternUnsized,
    Sized(usize),
}

impl GlobalSharedSize {
    fn fold(self, other: GlobalSharedSize) -> GlobalSharedSize {
        match (self, other) {
            (GlobalSharedSize::Sized(s1), GlobalSharedSize::Sized(s2)) => {
                GlobalSharedSize::Sized(usize::max(s1, s2))
            }
            _ => GlobalSharedSize::ExternUnsized,
        }
    }
}

fn replace_uses_of_shared_memory<'input>(
    new_id: &mut impl FnMut() -> spirv::Word,
    methods_to_indirectly_used_shared_globals: &HashMap<
        ast::MethodName<'input, spirv::Word>,
        BTreeSet<spirv::Word>,
    >,
    statements: Vec<ExpandedStatement>,
    remapped_globals_in_method: BTreeMap<spirv::Word, (spirv::Word, ast::Type)>,
) -> Vec<ExpandedStatement> {
    let mut result = Vec::with_capacity(statements.len());
    for statement in statements {
        match statement {
            Statement::Call(mut call) => {
                // We can safely skip checking call arguments,
                // because there's simply no way to pass shared ptr
                // without converting it to .b64 first
                if let Some(shared_globals_used_by_callee) =
                    methods_to_indirectly_used_shared_globals.get(&ast::MethodName::Func(call.name))
                {
                    for &shared_global_used_by_callee in shared_globals_used_by_callee {
                        let (remapped_shared_id, type_) = remapped_globals_in_method
                            .get(&shared_global_used_by_callee)
                            .unwrap_or_else(|| todo!());
                        call.input_arguments.push((
                            *remapped_shared_id,
                            type_.clone(),
                            ast::StateSpace::Shared,
                        ));
                    }
                }
                result.push(Statement::Call(call))
            }
            statement => {
                let new_statement = statement.map_id(&mut |id, _| {
                    if let Some((remapped_shared_id, _)) = remapped_globals_in_method.get(&id) {
                        *remapped_shared_id
                    } else {
                        id
                    }
                });
                result.push(new_statement);
            }
        }
    }
    result
}

// We need to compute two kinds of information:
// * If it's a kernel -> size of .shared globals in use (direct or indirect)
// * If it's a function -> does it use .shared global (directly or indirectly)
fn resolve_indirect_uses_of_globals_shared<'input>(
    methods_use_of_globals_shared: HashMap<
        ast::MethodName<'input, spirv::Word>,
        HashSet<spirv::Word>,
    >,
    kernels_methods_call_map: &MethodsCallMap<'input>,
) -> HashMap<ast::MethodName<'input, spirv::Word>, BTreeSet<spirv::Word>> {
    let mut result = HashMap::new();
    for (method, direct_globals) in methods_use_of_globals_shared.iter() {
        let mut indirect_globals = direct_globals.iter().copied().collect::<BTreeSet<_>>();
        kernels_methods_call_map.visit_callees(*method, |func| {
            indirect_globals.extend(
                methods_use_of_globals_shared
                    .get(&ast::MethodName::Func(func))
                    .into_iter()
                    .flatten()
                    .copied(),
            );
        });
        result.insert(*method, indirect_globals);
    }
    result
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
) -> HashMap<ast::MethodName<'input, spirv::Word>, HashMap<u8, (spirv::FPDenormMode, isize)>> {
    let mut denorm_methods = HashMap::new();
    for directive in module {
        match directive {
            Directive::Variable(..) | Directive::Method(Function { body: None, .. }) => {}
            Directive::Method(Function {
                func_decl,
                body: Some(statements),
                ..
            }) => {
                let mut flush_counter = DenormCountMap::new();
                let method_key = (**func_decl).borrow().name;
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
                        Statement::FunctionPointer(_) => {}
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

fn emit_function_header<'input>(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    defined_globals: &GlobalStringIdResolver<'input>,
    func_decl: &ast::MethodDeclaration<'input, spirv::Word>,
    call_map: &MethodsCallMap<'input>,
    globals_use_map: &HashMap<ast::MethodName<'input, spirv::Word>, HashSet<spirv::Word>>,
    kernel_info: &mut HashMap<String, KernelInfo>,
) -> Result<spirv::Word, TranslateError> {
    if let ast::MethodName::Kernel(name) = func_decl.name {
        let args_lens = func_decl
            .input_arguments
            .iter()
            .map(|param| {
                (
                    param.v_type.size_of(),
                    matches!(param.v_type, ast::Type::Pointer(..)),
                )
            })
            .collect();
        kernel_info.insert(
            name.to_string(),
            KernelInfo {
                arguments_sizes: args_lens,
                uses_shared_mem: func_decl.shared_mem.is_some(),
            },
        );
    }
    let (ret_type, func_type) = get_function_type(
        builder,
        map,
        func_decl.effective_input_arguments().map(|(_, typ)| typ),
        &func_decl.return_arguments,
    );
    let fn_id = match func_decl.name {
        ast::MethodName::Kernel(name) => {
            let fn_id = defined_globals.get_id(name)?;
            let interface = globals_use_map
                .get(&ast::MethodName::Kernel(name))
                .into_iter()
                .flatten()
                .copied()
                .chain({
                    call_map
                        .get_kernel_children(name)
                        .copied()
                        .flat_map(|subfunction| {
                            globals_use_map
                                .get(&ast::MethodName::Func(subfunction))
                                .into_iter()
                                .flatten()
                                .copied()
                        })
                        .into_iter()
                })
                .collect::<Vec<spirv::Word>>();
            builder.entry_point(spirv::ExecutionModel::Kernel, fn_id, name, interface);
            fn_id
        }
        ast::MethodName::Func(name) => name,
    };
    builder.begin_function(
        ret_type,
        Some(fn_id),
        spirv::FunctionControl::NONE,
        func_type,
    )?;
    for (name, typ) in func_decl.effective_input_arguments() {
        let result_type = map.get_or_add(builder, typ);
        builder.function_parameter(Some(name), result_type)?;
    }
    Ok(fn_id)
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
    builder.capability(spirv::Capability::DenormFlushToZero);
    // TODO: re-enable when Intel float control extension works
    //builder.capability(spirv::Capability::FunctionFloatControlINTEL);
}

// http://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_float_controls.html
fn emit_extensions(builder: &mut dr::Builder) {
    // TODO: re-enable when Intel float control extension works
    //builder.extension("SPV_INTEL_float_controls2");
    builder.extension("SPV_KHR_float_controls");
    builder.extension("SPV_KHR_no_integer_wrap_decoration");
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

fn translate_directive<'input, 'a>(
    id_defs: &'a mut GlobalStringIdResolver<'input>,
    ptx_impl_imports: &'a mut HashMap<String, Directive<'input>>,
    d: ast::Directive<'input, ast::ParsedArgParams<'input>>,
) -> Result<Option<Directive<'input>>, TranslateError> {
    Ok(match d {
        ast::Directive::Variable(linking, var) => Some(Directive::Variable(
            linking,
            ast::Variable {
                align: var.align,
                v_type: var.v_type.clone(),
                state_space: var.state_space,
                name: id_defs.get_or_add_def_typed(var.name, var.v_type, var.state_space, true),
                array_init: var.array_init,
            },
        )),
        ast::Directive::Method(linkage, f) => {
            translate_function(id_defs, ptx_impl_imports, linkage, f)?.map(Directive::Method)
        }
    })
}

fn translate_function<'input, 'a>(
    id_defs: &'a mut GlobalStringIdResolver<'input>,
    ptx_impl_imports: &'a mut HashMap<String, Directive<'input>>,
    linkage: ast::LinkingDirective,
    f: ast::ParsedFunction<'input>,
) -> Result<Option<Function<'input>>, TranslateError> {
    let import_as = match &f.func_directive {
        ast::MethodDeclaration {
            name: ast::MethodName::Func(func_name),
            ..
        } if *func_name == "__assertfail" || *func_name == "vprintf" => {
            Some([ZLUDA_PTX_PREFIX, func_name].concat())
        }
        _ => None,
    };
    let (str_resolver, fn_resolver, fn_decl) = id_defs.start_fn(&f.func_directive)?;
    let mut func = to_ssa(
        ptx_impl_imports,
        str_resolver,
        fn_resolver,
        fn_decl,
        f.body,
        f.tuning,
        linkage,
    )?;
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

fn rename_fn_params<'a, 'b>(
    fn_resolver: &mut FnStringIdResolver<'a, 'b>,
    args: &'b [ast::Variable<&'a str>],
) -> Vec<ast::Variable<spirv::Word>> {
    args.iter()
        .map(|a| ast::Variable {
            name: fn_resolver.add_def(a.name, Some((a.v_type.clone(), a.state_space)), true),
            v_type: a.v_type.clone(),
            state_space: a.state_space,
            align: a.align,
            array_init: a.array_init.clone(),
        })
        .collect()
}

fn to_ssa<'input, 'b>(
    ptx_impl_imports: &'b mut HashMap<String, Directive<'input>>,
    mut id_defs: FnStringIdResolver<'input, 'b>,
    fn_defs: GlobalFnDeclResolver<'input, 'b>,
    func_decl: Rc<RefCell<ast::MethodDeclaration<'input, spirv::Word>>>,
    f_body: Option<Vec<ast::Statement<ast::ParsedArgParams<'input>>>>,
    tuning: Vec<ast::TuningDirective>,
    linkage: ast::LinkingDirective,
) -> Result<Function<'input>, TranslateError> {
    //deparamize_function_decl(&func_decl)?;
    let f_body = match f_body {
        Some(vec) => vec,
        None => {
            return Ok(Function {
                func_decl: func_decl,
                body: None,
                globals: Vec::new(),
                import_as: None,
                tuning,
                linkage,
            })
        }
    };
    let normalized_ids = normalize_identifiers(&mut id_defs, &fn_defs, f_body)?;
    let mut numeric_id_defs = id_defs.finish();
    let unadorned_statements = normalize_predicates(normalized_ids, &mut numeric_id_defs)?;
    let typed_statements =
        convert_to_typed_statements(unadorned_statements, &fn_defs, &mut numeric_id_defs)?;
    let typed_statements =
        fix_special_registers2(ptx_impl_imports, typed_statements, &mut numeric_id_defs)?;
    let (func_decl, typed_statements) =
        convert_to_stateful_memory_access(func_decl, typed_statements, &mut numeric_id_defs)?;
    let ssa_statements = insert_mem_ssa_statements(
        typed_statements,
        &mut numeric_id_defs,
        &mut (*func_decl).borrow_mut(),
    )?;
    let mut numeric_id_defs = numeric_id_defs.finish();
    let expanded_statements = expand_arguments(ssa_statements, &mut numeric_id_defs)?;
    let expanded_statements =
        insert_implicit_conversions(expanded_statements, &mut numeric_id_defs)?;
    let mut numeric_id_defs = numeric_id_defs.unmut();
    let labeled_statements = normalize_labels(expanded_statements, &mut numeric_id_defs);
    let (f_body, globals) =
        extract_globals(labeled_statements, ptx_impl_imports, &mut numeric_id_defs)?;
    Ok(Function {
        func_decl: func_decl,
        globals: globals,
        body: Some(f_body),
        import_as: None,
        tuning,
        linkage,
    })
}

fn fix_special_registers2<'a, 'b, 'input>(
    ptx_impl_imports: &'a mut HashMap<String, Directive<'input>>,
    typed_statements: Vec<TypedStatement>,
    numeric_id_defs: &'a mut NumericIdResolver<'b>,
) -> Result<Vec<TypedStatement>, TranslateError> {
    let result = Vec::with_capacity(typed_statements.len());
    let mut sreg_sresolver = SpecialRegisterResolver {
        ptx_impl_imports,
        numeric_id_defs,
        result,
    };
    for s in typed_statements {
        match s {
            Statement::Call(details) => {
                let new_statement = details.visit(&mut sreg_sresolver)?;
                sreg_sresolver.result.push(new_statement);
            }
            Statement::Instruction(details) => {
                let new_statement = details.visit(&mut sreg_sresolver)?;
                sreg_sresolver.result.push(new_statement);
            }
            Statement::Conditional(details) => {
                let new_statement = details.visit(&mut sreg_sresolver)?;
                sreg_sresolver.result.push(new_statement);
            }
            Statement::Conversion(details) => {
                let new_statement = details.visit(&mut sreg_sresolver)?;
                sreg_sresolver.result.push(new_statement);
            }
            Statement::PtrAccess(details) => {
                let new_statement = details.visit(&mut sreg_sresolver)?;
                sreg_sresolver.result.push(new_statement);
            }
            Statement::RepackVector(details) => {
                let new_statement = details.visit(&mut sreg_sresolver)?;
                sreg_sresolver.result.push(new_statement);
            }
            s @ Statement::Variable(_)
            | s @ Statement::Label(_)
            | s @ Statement::FunctionPointer(_) => sreg_sresolver.result.push(s),
            _ => return Err(error_unreachable()),
        }
    }
    Ok(sreg_sresolver.result)
}

struct SpecialRegisterResolver<'a, 'b, 'input> {
    ptx_impl_imports: &'a mut HashMap<String, Directive<'input>>,
    numeric_id_defs: &'a mut NumericIdResolver<'b>,
    result: Vec<TypedStatement>,
}

impl<'a, 'b, 'input> SpecialRegisterResolver<'a, 'b, 'input> {
    fn replace_sreg(
        &mut self,
        desc: ArgumentDescriptor<spirv::Word>,
        vector_index: Option<u8>,
    ) -> Result<spirv::Word, TranslateError> {
        if let Some(sreg) = self.numeric_id_defs.special_registers.get(desc.op) {
            if desc.is_dst {
                return Err(TranslateError::MismatchedType);
            }
            let input_arguments = match (vector_index, sreg.get_function_input_type()) {
                (Some(idx), Some(inp_type)) => {
                    if inp_type != ast::ScalarType::U8 {
                        return Err(TranslateError::Unreachable);
                    }
                    let constant = self.numeric_id_defs.register_intermediate(Some((
                        ast::Type::Scalar(inp_type),
                        ast::StateSpace::Reg,
                    )));
                    self.result.push(Statement::Constant(ConstantDefinition {
                        dst: constant,
                        typ: inp_type,
                        value: ast::ImmediateValue::U64(idx as u64),
                    }));
                    vec![(
                        TypedOperand::Reg(constant),
                        ast::Type::Scalar(inp_type),
                        ast::StateSpace::Reg,
                    )]
                }
                (None, None) => Vec::new(),
                _ => return Err(TranslateError::MismatchedType),
            };
            let ocl_fn_name = [ZLUDA_PTX_PREFIX, sreg.get_unprefixed_function_name()].concat();
            let return_type = sreg.get_function_return_type();
            let fn_result = self.numeric_id_defs.register_intermediate(Some((
                ast::Type::Scalar(return_type),
                ast::StateSpace::Reg,
            )));
            let return_arguments = vec![(
                fn_result,
                ast::Type::Scalar(return_type),
                ast::StateSpace::Reg,
            )];
            let fn_call = register_external_fn_call(
                self.numeric_id_defs,
                self.ptx_impl_imports,
                ocl_fn_name.to_string(),
                return_arguments.iter().map(|(_, typ, space)| (typ, *space)),
                input_arguments.iter().map(|(_, typ, space)| (typ, *space)),
            )?;
            self.result.push(Statement::Call(ResolvedCall {
                uniform: false,
                return_arguments,
                name: fn_call,
                input_arguments,
            }));
            Ok(fn_result)
        } else {
            Ok(desc.op)
        }
    }
}

impl<'a, 'b, 'input> ArgumentMapVisitor<TypedArgParams, TypedArgParams>
    for SpecialRegisterResolver<'a, 'b, 'input>
{
    fn id(
        &mut self,
        desc: ArgumentDescriptor<spirv::Word>,
        _: Option<(&ast::Type, ast::StateSpace)>,
    ) -> Result<spirv::Word, TranslateError> {
        self.replace_sreg(desc, None)
    }

    fn operand(
        &mut self,
        desc: ArgumentDescriptor<TypedOperand>,
        typ: &ast::Type,
        state_space: ast::StateSpace,
    ) -> Result<TypedOperand, TranslateError> {
        Ok(match desc.op {
            TypedOperand::Reg(reg) => TypedOperand::Reg(self.replace_sreg(desc.new_op(reg), None)?),
            op @ TypedOperand::RegOffset(_, _) => op,
            op @ TypedOperand::Imm(_) => op,
            TypedOperand::VecMember(reg, idx) => {
                TypedOperand::VecMember(self.replace_sreg(desc.new_op(reg), Some(idx))?, idx)
            }
        })
    }
}

fn extract_globals<'input, 'b>(
    sorted_statements: Vec<ExpandedStatement>,
    ptx_impl_imports: &mut HashMap<String, Directive>,
    id_def: &mut NumericIdResolver,
) -> Result<(Vec<ExpandedStatement>, Vec<ast::Variable<spirv::Word>>), TranslateError> {
    let mut local = Vec::with_capacity(sorted_statements.len());
    let mut global = Vec::new();
    for statement in sorted_statements {
        match statement {
            Statement::Variable(
                var
                @
                ast::Variable {
                    state_space: ast::StateSpace::Shared,
                    ..
                },
            )
            | Statement::Variable(
                var
                @
                ast::Variable {
                    state_space: ast::StateSpace::Global,
                    ..
                },
            ) => global.push(var),
            Statement::Instruction(ast::Instruction::Bfe { typ, arg }) => {
                let fn_name = [ZLUDA_PTX_PREFIX, "bfe_", typ.to_ptx_name()].concat();
                local.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Bfe { typ, arg },
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::Bfi { typ, arg }) => {
                let fn_name = [ZLUDA_PTX_PREFIX, "bfi_", typ.to_ptx_name()].concat();
                local.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Bfi { typ, arg },
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::Brev { typ, arg }) => {
                let fn_name = [ZLUDA_PTX_PREFIX, "brev_", typ.to_ptx_name()].concat();
                local.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Brev { typ, arg },
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::Activemask { arg }) => {
                let fn_name = [ZLUDA_PTX_PREFIX, "activemask"].concat();
                local.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Activemask { arg },
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::Atom(
                details
                @
                ast::AtomDetails {
                    inner:
                        ast::AtomInnerDetails::Unsigned {
                            op: ast::AtomUIntOp::Inc,
                            ..
                        },
                    ..
                },
                args,
            )) => {
                let fn_name = [
                    ZLUDA_PTX_PREFIX,
                    "atom_",
                    details.semantics.to_ptx_name(),
                    "_",
                    details.scope.to_ptx_name(),
                    "_",
                    details.space.to_ptx_name(),
                    "_inc",
                ]
                .concat();
                local.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Atom(details, args),
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::Atom(
                details
                @
                ast::AtomDetails {
                    inner:
                        ast::AtomInnerDetails::Unsigned {
                            op: ast::AtomUIntOp::Dec,
                            ..
                        },
                    ..
                },
                args,
            )) => {
                let fn_name = [
                    ZLUDA_PTX_PREFIX,
                    "atom_",
                    details.semantics.to_ptx_name(),
                    "_",
                    details.scope.to_ptx_name(),
                    "_",
                    details.space.to_ptx_name(),
                    "_dec",
                ]
                .concat();
                local.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Atom(details, args),
                    fn_name,
                )?);
            }
            Statement::Instruction(ast::Instruction::Atom(
                details
                @
                ast::AtomDetails {
                    inner:
                        ast::AtomInnerDetails::Float {
                            op: ast::AtomFloatOp::Add,
                            ..
                        },
                    ..
                },
                args,
            )) => {
                let fn_name = [
                    ZLUDA_PTX_PREFIX,
                    "atom_",
                    details.semantics.to_ptx_name(),
                    "_",
                    details.scope.to_ptx_name(),
                    "_",
                    details.space.to_ptx_name(),
                    "_add_",
                    details.inner.get_type().to_ptx_name(),
                ]
                .concat();
                local.push(instruction_to_fn_call(
                    id_def,
                    ptx_impl_imports,
                    ast::Instruction::Atom(details, args),
                    fn_name,
                )?);
            }
            s => local.push(s),
        }
    }
    Ok((local, global))
}

impl ast::ScalarType {
    fn to_ptx_name(self) -> &'static str {
        match self {
            ast::ScalarType::B8 => "b8",
            ast::ScalarType::B16 => "b16",
            ast::ScalarType::B32 => "b32",
            ast::ScalarType::B64 => "b64",
            ast::ScalarType::U8 => "u8",
            ast::ScalarType::U16 => "u16",
            ast::ScalarType::U32 => "u32",
            ast::ScalarType::U64 => "u64",
            ast::ScalarType::S8 => "s8",
            ast::ScalarType::S16 => "s16",
            ast::ScalarType::S32 => "s32",
            ast::ScalarType::S64 => "s64",
            ast::ScalarType::F16 => "f16",
            ast::ScalarType::F32 => "f32",
            ast::ScalarType::F64 => "f64",
            ast::ScalarType::F16x2 => "f16x2",
            ast::ScalarType::Pred => "pred",
        }
    }
}

impl ast::AtomSemantics {
    fn to_ptx_name(self) -> &'static str {
        match self {
            ast::AtomSemantics::Relaxed => "relaxed",
            ast::AtomSemantics::Acquire => "acquire",
            ast::AtomSemantics::Release => "release",
            ast::AtomSemantics::AcquireRelease => "acq_rel",
        }
    }
}

impl ast::MemScope {
    fn to_ptx_name(self) -> &'static str {
        match self {
            ast::MemScope::Cta => "cta",
            ast::MemScope::Gpu => "gpu",
            ast::MemScope::Sys => "sys",
        }
    }
}

impl ast::StateSpace {
    fn to_ptx_name(self) -> &'static str {
        match self {
            ast::StateSpace::Generic => "generic",
            ast::StateSpace::Global => "global",
            ast::StateSpace::Shared => "shared",
            ast::StateSpace::Reg => "reg",
            ast::StateSpace::Const => "const",
            ast::StateSpace::Local => "local",
            ast::StateSpace::Param => "param",
            ast::StateSpace::Sreg => "sreg",
        }
    }
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
                ast::Instruction::Mov(
                    mov,
                    ast::Arg2Mov {
                        dst: ast::Operand::Reg(dst_reg),
                        src: ast::Operand::Reg(src_reg),
                    },
                ) if fn_defs.fns.contains_key(&src_reg) => {
                    if mov.typ != ast::Type::Scalar(ast::ScalarType::U64) {
                        return Err(TranslateError::MismatchedType);
                    }
                    result.push(TypedStatement::FunctionPointer(FunctionPointerDetails {
                        dst: dst_reg,
                        src: src_reg,
                    }));
                }
                ast::Instruction::Call(call) => {
                    let resolver = fn_defs.get_fn_sig_resolver(call.func)?;
                    let resolved_call = resolver.resolve_in_spirv_repr(call)?;
                    let mut visitor = VectorRepackVisitor::new(&mut result, id_defs);
                    let reresolved_call = resolved_call.visit(&mut visitor)?;
                    visitor.func.push(reresolved_call);
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
        non_default_implicit_conversion: Option<
            fn(
                (ast::StateSpace, &ast::Type),
                (ast::StateSpace, &ast::Type),
            ) -> Result<Option<ConversionKind>, TranslateError>,
        >,
        typ: &ast::Type,
        state_space: ast::StateSpace,
        idx: Vec<spirv::Word>,
    ) -> Result<spirv::Word, TranslateError> {
        // mov.u32 foobar, {a,b};
        let scalar_t = match typ {
            ast::Type::Vector(scalar_t, _) => *scalar_t,
            _ => return Err(TranslateError::MismatchedType),
        };
        let temp_vec = self
            .id_def
            .register_intermediate(Some((typ.clone(), state_space)));
        let statement = Statement::RepackVector(RepackVectorDetails {
            is_extract: is_dst,
            typ: scalar_t,
            packed: temp_vec,
            unpacked: idx,
            non_default_implicit_conversion,
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
        _: Option<(&ast::Type, ast::StateSpace)>,
    ) -> Result<spirv::Word, TranslateError> {
        Ok(desc.op)
    }

    fn operand(
        &mut self,
        desc: ArgumentDescriptor<ast::Operand<spirv::Word>>,
        typ: &ast::Type,
        state_space: ast::StateSpace,
    ) -> Result<TypedOperand, TranslateError> {
        Ok(match desc.op {
            ast::Operand::Reg(reg) => TypedOperand::Reg(reg),
            ast::Operand::RegOffset(reg, offset) => TypedOperand::RegOffset(reg, offset),
            ast::Operand::Imm(x) => TypedOperand::Imm(x),
            ast::Operand::VecMember(vec, idx) => TypedOperand::VecMember(vec, idx),
            ast::Operand::VecPack(vec) => TypedOperand::Reg(self.convert_vector(
                desc.is_dst,
                desc.non_default_implicit_conversion,
                typ,
                state_space,
                vec,
            )?),
        })
    }
}

fn instruction_to_fn_call(
    id_defs: &mut NumericIdResolver,
    ptx_impl_imports: &mut HashMap<String, Directive>,
    inst: ast::Instruction<ExpandedArgParams>,
    fn_name: String,
) -> Result<ExpandedStatement, TranslateError> {
    let mut arguments = Vec::new();
    inst.visit(&mut |desc: ArgumentDescriptor<spirv::Word>,
                     typ: Option<(&ast::Type, ast::StateSpace)>| {
        let (typ, space) = match typ {
            Some((typ, space)) => (typ.clone(), space),
            None => return Err(error_unreachable()),
        };
        arguments.push((desc, typ, space));
        Ok(0)
    })?;
    let return_arguments_count = arguments
        .iter()
        .position(|(desc, _, _)| !desc.is_dst)
        .unwrap_or(arguments.len());
    let (return_arguments, input_arguments) = arguments.split_at(return_arguments_count);
    let fn_id = register_external_fn_call(
        id_defs,
        ptx_impl_imports,
        fn_name,
        return_arguments.iter().map(|(_, typ, state)| (typ, *state)),
        input_arguments.iter().map(|(_, typ, state)| (typ, *state)),
    )?;
    Ok(Statement::Call(ResolvedCall {
        uniform: false,
        name: fn_id,
        return_arguments: arguments_to_resolved_arguments(return_arguments),
        input_arguments: arguments_to_resolved_arguments(input_arguments),
    }))
}

fn register_external_fn_call<'a>(
    id_defs: &mut NumericIdResolver,
    ptx_impl_imports: &mut HashMap<String, Directive>,
    name: String,
    return_arguments: impl Iterator<Item = (&'a ast::Type, ast::StateSpace)>,
    input_arguments: impl Iterator<Item = (&'a ast::Type, ast::StateSpace)>,
) -> Result<spirv::Word, TranslateError> {
    match ptx_impl_imports.entry(name) {
        hash_map::Entry::Vacant(entry) => {
            let fn_id = id_defs.register_intermediate(None);
            let return_arguments = fn_arguments_to_variables(id_defs, return_arguments);
            let input_arguments = fn_arguments_to_variables(id_defs, input_arguments);
            let func_decl = ast::MethodDeclaration::<spirv::Word> {
                return_arguments,
                name: ast::MethodName::Func(fn_id),
                input_arguments,
                shared_mem: None,
            };
            let func = Function {
                func_decl: Rc::new(RefCell::new(func_decl)),
                globals: Vec::new(),
                body: None,
                import_as: Some(entry.key().clone()),
                tuning: Vec::new(),
                linkage: ast::LinkingDirective::EXTERN,
            };
            entry.insert(Directive::Method(func));
            Ok(fn_id)
        }
        hash_map::Entry::Occupied(entry) => match entry.get() {
            Directive::Method(Function { func_decl, .. }) => match (**func_decl).borrow().name {
                ast::MethodName::Func(fn_id) => Ok(fn_id),
                ast::MethodName::Kernel(_) => Err(error_unreachable()),
            },
            _ => Err(error_unreachable()),
        },
    }
}

fn fn_arguments_to_variables<'a>(
    id_defs: &mut NumericIdResolver,
    args: impl Iterator<Item = (&'a ast::Type, ast::StateSpace)>,
) -> Vec<ast::Variable<spirv::Word>> {
    args.map(|(typ, space)| ast::Variable {
        align: None,
        v_type: typ.clone(),
        state_space: space,
        name: id_defs.register_intermediate(None),
        array_init: Vec::new(),
    })
    .collect::<Vec<_>>()
}

fn arguments_to_resolved_arguments(
    args: &[(ArgumentDescriptor<spirv::Word>, ast::Type, ast::StateSpace)],
) -> Vec<(spirv::Word, ast::Type, ast::StateSpace)> {
    args.iter()
        .map(|(desc, typ, space)| (desc.op, typ.clone(), *space))
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
            | Statement::RepackVector(..)
            | Statement::FunctionPointer(..) => {}
        }
    }
    iter::once(Statement::Label(id_def.register_intermediate(None)))
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
                    let if_true = id_def.register_intermediate(None);
                    let if_false = id_def.register_intermediate(None);
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

/*
    How do we handle arguments:
    - input .params in kernels
        .param .b64 in_arg
      get turned into this SPIR-V:
        %1 = OpFunctionParameter %ulong
        %2 = OpVariable %_ptr_Function_ulong Function
             OpStore %2 %1
      We do this for two reasons. One, common treatment for argument-declared
      .param variables and .param variables inside function (we assume that
      at SPIR-V level every .param is a pointer in Function storage class)
    - input .params in functions
        .param .b64 in_arg
      get turned into this SPIR-V:
        %1 = OpFunctionParameter %_ptr_Function_ulong
    - input .regs
        .reg .b64 in_arg
      get turned into the same SPIR-V as kernel .params:
        %1 = OpFunctionParameter %ulong
        %2 = OpVariable %_ptr_Function_ulong Function
             OpStore %2 %1
    - output .regs
        .reg .b64 out_arg
      get just a variable declaration:
        %2 = OpVariable %%_ptr_Function_ulong Function
    - output .params don't exist, they have been moved to input positions
      by an earlier pass
    Distinguishing betweem kernel .params and function .params is not the
    cleanest solution. Alternatively, we could "deparamize" all kernel .param
    arguments by turning them into .reg arguments like this:
        .param .b64 arg -> .reg ptr<.b64,.param> arg
    This has the massive downside that this transformation would have to run
    very early and would muddy up already difficult code. It's simpler to just
    have an if here
*/
fn insert_mem_ssa_statements<'a, 'b>(
    func: Vec<TypedStatement>,
    id_def: &mut NumericIdResolver,
    fn_decl: &'a mut ast::MethodDeclaration<'b, spirv::Word>,
) -> Result<Vec<TypedStatement>, TranslateError> {
    let mut result = Vec::with_capacity(func.len());
    for arg in fn_decl.input_arguments.iter_mut() {
        insert_mem_ssa_argument(id_def, &mut result, arg, fn_decl.name.is_kernel());
    }
    for arg in fn_decl.return_arguments.iter() {
        insert_mem_ssa_argument_reg_return(&mut result, arg);
    }
    for s in func {
        match s {
            Statement::Call(call) => {
                insert_mem_ssa_statement_default(id_def, &mut result, call.cast())?
            }
            Statement::Instruction(inst) => match inst {
                ast::Instruction::Ret(d) => {
                    // TODO: handle multiple output args
                    match &fn_decl.return_arguments[..] {
                        [return_reg] => {
                            let new_id = id_def.register_intermediate(Some((
                                return_reg.v_type.clone(),
                                ast::StateSpace::Reg,
                            )));
                            result.push(Statement::LoadVar(LoadVarDetails {
                                arg: ast::Arg2 {
                                    dst: new_id,
                                    src: return_reg.name,
                                },
                                // TODO: ret with stateful conversion
                                state_space: ast::StateSpace::Reg,
                                typ: return_reg.v_type.clone(),
                                member_index: None,
                            }));
                            result.push(Statement::RetValue(d, new_id));
                        }
                        [] => result.push(Statement::Instruction(ast::Instruction::Ret(d))),
                        _ => unimplemented!(),
                    }
                }
                inst => insert_mem_ssa_statement_default(id_def, &mut result, inst)?,
            },
            Statement::Conditional(bra) => {
                insert_mem_ssa_statement_default(id_def, &mut result, bra)?
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
            Statement::FunctionPointer(func_ptr) => {
                insert_mem_ssa_statement_default(id_def, &mut result, func_ptr)?
            }
            s @ Statement::Variable(_) | s @ Statement::Label(_) | s @ Statement::Constant(..) => {
                result.push(s)
            }
            _ => return Err(error_unreachable()),
        }
    }
    Ok(result)
}

fn insert_mem_ssa_argument(
    id_def: &mut NumericIdResolver,
    func: &mut Vec<TypedStatement>,
    arg: &mut ast::Variable<spirv::Word>,
    is_kernel: bool,
) {
    if !is_kernel && arg.state_space == ast::StateSpace::Param {
        return;
    }
    let new_id = id_def.register_intermediate(Some((arg.v_type.clone(), arg.state_space)));
    func.push(Statement::Variable(ast::Variable {
        align: arg.align,
        v_type: arg.v_type.clone(),
        state_space: ast::StateSpace::Reg,
        name: arg.name,
        array_init: Vec::new(),
    }));
    func.push(Statement::StoreVar(StoreVarDetails {
        arg: ast::Arg2St {
            src1: arg.name,
            src2: new_id,
        },
        typ: arg.v_type.clone(),
        member_index: None,
    }));
    arg.name = new_id;
}

fn insert_mem_ssa_argument_reg_return(
    func: &mut Vec<TypedStatement>,
    arg: &ast::Variable<spirv::Word>,
) {
    func.push(Statement::Variable(ast::Variable {
        align: arg.align,
        v_type: arg.v_type.clone(),
        state_space: arg.state_space,
        name: arg.name,
        array_init: arg.array_init.clone(),
    }));
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
    state_space: ast::StateSpace,
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
        Ok((self.stmt_ctor)(
            visitor.id(self.desc, Some((self.typ, self.state_space)))?,
        ))
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
        expected: Option<(&ast::Type, ast::StateSpace)>,
    ) -> Result<spirv::Word, TranslateError> {
        let symbol = desc.op.0;
        if expected.is_none() {
            return Ok(symbol);
        };
        let (mut var_type, var_space, is_variable) = self.id_def.get_typed(symbol)?;
        if !var_space.is_compatible(ast::StateSpace::Reg) || !is_variable {
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
        let generated_id = self
            .id_def
            .register_intermediate(Some((var_type.clone(), ast::StateSpace::Reg)));
        if !desc.is_dst {
            self.func.push(Statement::LoadVar(LoadVarDetails {
                arg: Arg2 {
                    dst: generated_id,
                    src: symbol,
                },
                state_space: ast::StateSpace::Reg,
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
        typ: Option<(&ast::Type, ast::StateSpace)>,
    ) -> Result<spirv::Word, TranslateError> {
        self.symbol(desc.new_op((desc.op, None)), typ)
    }

    fn operand(
        &mut self,
        desc: ArgumentDescriptor<TypedOperand>,
        typ: &ast::Type,
        state_space: ast::StateSpace,
    ) -> Result<TypedOperand, TranslateError> {
        Ok(match desc.op {
            TypedOperand::Reg(reg) => {
                TypedOperand::Reg(self.symbol(desc.new_op((reg, None)), Some((typ, state_space)))?)
            }
            TypedOperand::RegOffset(reg, offset) => TypedOperand::RegOffset(
                self.symbol(desc.new_op((reg, None)), Some((typ, state_space)))?,
                offset,
            ),
            op @ TypedOperand::Imm(..) => op,
            TypedOperand::VecMember(symbol, index) => TypedOperand::Reg(
                self.symbol(desc.new_op((symbol, Some(index))), Some((typ, state_space)))?,
            ),
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
                state_space,
                name,
                array_init,
            }) => result.push(Statement::Variable(ast::Variable {
                align,
                v_type,
                state_space,
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
            Statement::Constant(c) => result.push(Statement::Constant(c)),
            Statement::FunctionPointer(d) => result.push(Statement::FunctionPointer(d)),
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
        _: Option<(&ast::Type, ast::StateSpace)>,
    ) -> Result<spirv::Word, TranslateError> {
        Ok(desc.op)
    }

    fn reg_offset(
        &mut self,
        desc: ArgumentDescriptor<(spirv::Word, i32)>,
        typ: &ast::Type,
        state_space: ast::StateSpace,
    ) -> Result<spirv::Word, TranslateError> {
        let (reg, offset) = desc.op;
        if !desc.is_memory_access {
            let (reg_type, reg_space) = self.id_def.get_typed(reg)?;
            if !reg_space.is_compatible(ast::StateSpace::Reg) {
                return Err(TranslateError::MismatchedType);
            }
            let reg_scalar_type = match reg_type {
                ast::Type::Scalar(underlying_type) => underlying_type,
                _ => return Err(TranslateError::MismatchedType),
            };
            let id_constant_stmt = self
                .id_def
                .register_intermediate(reg_type.clone(), ast::StateSpace::Reg);
            self.func.push(Statement::Constant(ConstantDefinition {
                dst: id_constant_stmt,
                typ: reg_scalar_type,
                value: ast::ImmediateValue::S64(offset as i64),
            }));
            let arith_details = match reg_scalar_type.kind() {
                ast::ScalarKind::Signed => ast::ArithDetails::Signed(ast::ArithSInt {
                    typ: reg_scalar_type,
                    saturate: false,
                }),
                ast::ScalarKind::Unsigned | ast::ScalarKind::Bit => {
                    ast::ArithDetails::Unsigned(reg_scalar_type)
                }
                _ => return Err(error_unreachable()),
            };
            let id_add_result = self.id_def.register_intermediate(reg_type, state_space);
            self.func.push(Statement::Instruction(ast::Instruction::Add(
                arith_details,
                ast::Arg3 {
                    dst: id_add_result,
                    src1: reg,
                    src2: id_constant_stmt,
                },
            )));
            Ok(id_add_result)
        } else {
            let id_constant_stmt = self.id_def.register_intermediate(
                ast::Type::Scalar(ast::ScalarType::S64),
                ast::StateSpace::Reg,
            );
            self.func.push(Statement::Constant(ConstantDefinition {
                dst: id_constant_stmt,
                typ: ast::ScalarType::S64,
                value: ast::ImmediateValue::S64(offset as i64),
            }));
            let dst = self.id_def.register_intermediate(typ.clone(), state_space);
            self.func.push(Statement::PtrAccess(PtrAccess {
                underlying_type: typ.clone(),
                state_space: state_space,
                dst,
                ptr_src: reg,
                offset_src: id_constant_stmt,
            }));
            Ok(dst)
        }
    }

    fn immediate(
        &mut self,
        desc: ArgumentDescriptor<ast::ImmediateValue>,
        typ: &ast::Type,
        state_space: ast::StateSpace,
    ) -> Result<spirv::Word, TranslateError> {
        let scalar_t = if let ast::Type::Scalar(scalar) = typ {
            *scalar
        } else {
            todo!()
        };
        let id = self
            .id_def
            .register_intermediate(ast::Type::Scalar(scalar_t), state_space);
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
        t: Option<(&ast::Type, ast::StateSpace)>,
    ) -> Result<spirv::Word, TranslateError> {
        self.reg(desc, t)
    }

    fn operand(
        &mut self,
        desc: ArgumentDescriptor<TypedOperand>,
        typ: &ast::Type,
        state_space: ast::StateSpace,
    ) -> Result<spirv::Word, TranslateError> {
        match desc.op {
            TypedOperand::Reg(r) => self.reg(desc.new_op(r), Some((typ, state_space))),
            TypedOperand::Imm(x) => self.immediate(desc.new_op(x), typ, state_space),
            TypedOperand::RegOffset(reg, offset) => {
                self.reg_offset(desc.new_op((reg, offset)), typ, state_space)
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
            Statement::Call(call) => {
                insert_implicit_conversions_impl(&mut result, id_def, call)?;
            }
            Statement::Instruction(inst) => {
                insert_implicit_conversions_impl(&mut result, id_def, inst)?;
            }
            Statement::PtrAccess(access) => {
                insert_implicit_conversions_impl(&mut result, id_def, access)?;
            }
            Statement::RepackVector(repack) => {
                insert_implicit_conversions_impl(&mut result, id_def, repack)?;
            }
            s @ Statement::Conditional(_)
            | s @ Statement::Conversion(_)
            | s @ Statement::Label(_)
            | s @ Statement::Constant(_)
            | s @ Statement::Variable(_)
            | s @ Statement::LoadVar(..)
            | s @ Statement::StoreVar(..)
            | s @ Statement::RetValue(..)
            | s @ Statement::FunctionPointer(..) => result.push(s),
        }
    }
    Ok(result)
}

fn insert_implicit_conversions_impl(
    func: &mut Vec<ExpandedStatement>,
    id_def: &mut MutableNumericIdResolver,
    stmt: impl Visitable<ExpandedArgParams, ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let mut post_conv = Vec::new();
    let statement =
        stmt.visit(&mut |desc: ArgumentDescriptor<spirv::Word>,
                         typ: Option<(&ast::Type, ast::StateSpace)>| {
            let (instr_type, instruction_space) = match typ {
                None => return Ok(desc.op),
                Some(t) => t,
            };
            let (operand_type, operand_space) = id_def.get_typed(desc.op)?;
            let conversion_fn = desc
                .non_default_implicit_conversion
                .unwrap_or(default_implicit_conversion);
            match conversion_fn(
                (operand_space, &operand_type),
                (instruction_space, instr_type),
            )? {
                Some(conv_kind) => {
                    let conv_output = if desc.is_dst {
                        &mut post_conv
                    } else {
                        &mut *func
                    };
                    let mut from_type = instr_type.clone();
                    let mut from_space = instruction_space;
                    let mut to_type = operand_type;
                    let mut to_space = operand_space;
                    let mut src =
                        id_def.register_intermediate(instr_type.clone(), instruction_space);
                    let mut dst = desc.op;
                    let result = Ok(src);
                    if !desc.is_dst {
                        mem::swap(&mut src, &mut dst);
                        mem::swap(&mut from_type, &mut to_type);
                        mem::swap(&mut from_space, &mut to_space);
                    }
                    conv_output.push(Statement::Conversion(ImplicitConversion {
                        src,
                        dst,
                        from_type,
                        from_space,
                        to_type,
                        to_space,
                        kind: conv_kind,
                    }));
                    result
                }
                None => Ok(desc.op),
            }
        })?;
    func.push(statement);
    func.append(&mut post_conv);
    Ok(())
}

fn get_function_type(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    spirv_input: impl Iterator<Item = SpirvType>,
    spirv_output: &[ast::Variable<spirv::Word>],
) -> (spirv::Word, spirv::Word) {
    map.get_or_add_fn(
        builder,
        spirv_input,
        spirv_output
            .iter()
            .map(|var| SpirvType::new(var.v_type.clone())),
    )
}

fn emit_function_body_ops<'input>(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    id_defs: &GlobalStringIdResolver<'input>,
    opencl: spirv::Word,
    func: &[ExpandedStatement],
) -> Result<(), TranslateError> {
    for s in func {
        match s {
            Statement::Label(id) => {
                if builder.selected_block().is_some() {
                    builder.branch(*id)?;
                }
                builder.begin_block(Some(*id))?;
            }
            _ => {
                if builder.selected_block().is_none() && builder.selected_function().is_some() {
                    builder.begin_block(None)?;
                }
            }
        }
        match s {
            Statement::Label(_) => (),
            Statement::Call(call) => {
                let (result_type, result_id) = match &*call.return_arguments {
                    [(id, typ, space)] => {
                        if *space != ast::StateSpace::Reg {
                            return Err(error_unreachable());
                        }
                        (
                            map.get_or_add(builder, SpirvType::new(typ.clone())),
                            Some(*id),
                        )
                    }
                    [] => (map.void(), None),
                    _ => todo!(),
                };
                let arg_list = call
                    .input_arguments
                    .iter()
                    .map(|(id, _, _)| *id)
                    .collect::<Vec<_>>();
                builder.function_call(result_type, result_id, call.name, arg_list)?;
            }
            Statement::Variable(var) => {
                emit_variable(builder, map, id_defs, ast::LinkingDirective::NONE, var)?;
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
                builder.branch_conditional(
                    bra.predicate,
                    bra.if_true,
                    bra.if_false,
                    iter::empty(),
                )?;
            }
            Statement::FunctionPointer(FunctionPointerDetails { dst, src }) => {
                // TODO: implement properly
                let zero = map.get_or_add_constant(
                    builder,
                    &ast::Type::Scalar(ast::ScalarType::U64),
                    &vec_repr(0u64),
                )?;
                let result_type = map.get_or_add_scalar(builder, ast::ScalarType::U64);
                builder.copy_object(result_type, Some(*dst), zero)?;
            }
            Statement::Instruction(inst) => match inst {
                ast::Instruction::Abs(d, arg) => emit_abs(builder, map, opencl, d, arg)?,
                ast::Instruction::Call(_) => unreachable!(),
                // SPIR-V does not support marking jumps as guaranteed-converged
                ast::Instruction::Bra(_, arg) => {
                    builder.branch(arg.src)?;
                }
                ast::Instruction::Ld(data, arg) => {
                    let mem_access = match data.qualifier {
                        ast::LdStQualifier::Weak => spirv::MemoryAccess::NONE,
                        // ld.volatile does not match Volatile OpLoad nor Relaxed OpAtomicLoad
                        ast::LdStQualifier::Volatile => spirv::MemoryAccess::VOLATILE,
                        _ => return Err(TranslateError::Todo),
                    };
                    let result_type =
                        map.get_or_add(builder, SpirvType::new(ast::Type::from(data.typ.clone())));
                    builder.load(
                        result_type,
                        Some(arg.dst),
                        arg.src,
                        Some(mem_access | spirv::MemoryAccess::ALIGNED),
                        [dr::Operand::LiteralInt32(
                            ast::Type::from(data.typ.clone()).size_of() as u32,
                        )]
                        .iter()
                        .cloned(),
                    )?;
                }
                ast::Instruction::St(data, arg) => {
                    let mem_access = match data.qualifier {
                        ast::LdStQualifier::Weak => spirv::MemoryAccess::NONE,
                        // st.volatile does not match Volatile OpStore nor Relaxed OpAtomicStore
                        ast::LdStQualifier::Volatile => spirv::MemoryAccess::VOLATILE,
                        _ => return Err(TranslateError::Todo),
                    };
                    builder.store(
                        arg.src1,
                        arg.src2,
                        Some(mem_access | spirv::MemoryAccess::ALIGNED),
                        [dr::Operand::LiteralInt32(
                            ast::Type::from(data.typ.clone()).size_of() as u32,
                        )]
                        .iter()
                        .cloned(),
                    )?;
                }
                // SPIR-V does not support ret as guaranteed-converged
                ast::Instruction::Ret(_) => builder.ret()?,
                ast::Instruction::Mov(d, arg) => {
                    let result_type =
                        map.get_or_add(builder, SpirvType::new(ast::Type::from(d.typ.clone())));
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
                    let result_type = map.get_or_add(builder, SpirvType::from(*t));
                    let result_id = Some(a.dst);
                    let operand = a.src;
                    match t {
                        ast::ScalarType::Pred => {
                            logical_not(builder, result_type, result_id, operand)
                        }
                        _ => builder.not(result_type, result_id, operand),
                    }?;
                }
                ast::Instruction::Shl(t, a) => {
                    let full_type = ast::Type::Scalar(*t);
                    let size_of = full_type.size_of();
                    let result_type = map.get_or_add(builder, SpirvType::new(full_type));
                    let offset_src = insert_shift_hack(builder, map, a.src2, size_of)?;
                    builder.shift_left_logical(result_type, Some(a.dst), a.src1, offset_src)?;
                }
                ast::Instruction::Shr(t, a) => {
                    let full_type = ast::ScalarType::from(*t);
                    let size_of = full_type.size_of();
                    let result_type = map.get_or_add_scalar(builder, full_type);
                    let offset_src = insert_shift_hack(builder, map, a.src2, size_of as usize)?;
                    if t.kind() == ast::ScalarKind::Signed {
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
                ast::Instruction::Fma(fma, arg) => emit_fma_float(builder, map, opencl, fma, arg)?,
                ast::Instruction::Or(t, a) => {
                    let result_type = map.get_or_add_scalar(builder, ast::ScalarType::from(*t));
                    if *t == ast::ScalarType::Pred {
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
                    emit_rcp(builder, map, opencl, d, a)?;
                }
                ast::Instruction::And(t, a) => {
                    let result_type = map.get_or_add_scalar(builder, ast::ScalarType::from(*t));
                    if *t == ast::ScalarType::Pred {
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
                        spirv::CLOp::rsqrt as spirv::Word,
                        [dr::Operand::IdRef(a.src)].iter().cloned(),
                    )?;
                }
                ast::Instruction::Neg(details, arg) => {
                    let result_type = map.get_or_add_scalar(builder, details.typ);
                    let negate_func = if details.typ.kind() == ast::ScalarKind::Float {
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
                        [dr::Operand::IdRef(arg.src)].iter().cloned(),
                    )?;
                }
                ast::Instruction::Cos { arg, .. } => {
                    let result_type = map.get_or_add_scalar(builder, ast::ScalarType::F32);
                    builder.ext_inst(
                        result_type,
                        Some(arg.dst),
                        opencl,
                        spirv::CLOp::cos as u32,
                        [dr::Operand::IdRef(arg.src)].iter().cloned(),
                    )?;
                }
                ast::Instruction::Lg2 { arg, .. } => {
                    let result_type = map.get_or_add_scalar(builder, ast::ScalarType::F32);
                    builder.ext_inst(
                        result_type,
                        Some(arg.dst),
                        opencl,
                        spirv::CLOp::log2 as u32,
                        [dr::Operand::IdRef(arg.src)].iter().cloned(),
                    )?;
                }
                ast::Instruction::Ex2 { arg, .. } => {
                    let result_type = map.get_or_add_scalar(builder, ast::ScalarType::F32);
                    builder.ext_inst(
                        result_type,
                        Some(arg.dst),
                        opencl,
                        spirv::CLOp::exp2 as u32,
                        [dr::Operand::IdRef(arg.src)].iter().cloned(),
                    )?;
                }
                ast::Instruction::Clz { typ, arg } => {
                    let result_type = map.get_or_add_scalar(builder, (*typ).into());
                    builder.ext_inst(
                        result_type,
                        Some(arg.dst),
                        opencl,
                        spirv::CLOp::clz as u32,
                        [dr::Operand::IdRef(arg.src)].iter().cloned(),
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
                        ast::ScalarType::Pred => emit_logical_xor_spirv,
                        _ => dr::Builder::bitwise_xor,
                    };
                    let result_type = map.get_or_add_scalar(builder, (*typ).into());
                    builder_fn(builder, result_type, Some(arg.dst), arg.src1, arg.src2)?;
                }
                ast::Instruction::Bfe { .. }
                | ast::Instruction::Bfi { .. }
                | ast::Instruction::Activemask { .. } => {
                    // Should have beeen replaced with a funciton call earlier
                    return Err(error_unreachable());
                }

                ast::Instruction::Rem { typ, arg } => {
                    let builder_fn = if typ.kind() == ast::ScalarKind::Signed {
                        dr::Builder::s_mod
                    } else {
                        dr::Builder::u_mod
                    };
                    let result_type = map.get_or_add_scalar(builder, (*typ).into());
                    builder_fn(builder, result_type, Some(arg.dst), arg.src1, arg.src2)?;
                }
                ast::Instruction::Prmt { control, arg } => {
                    let control = *control as u32;
                    let components = [
                        (control >> 0) & 0b1111,
                        (control >> 4) & 0b1111,
                        (control >> 8) & 0b1111,
                        (control >> 12) & 0b1111,
                    ];
                    if components.iter().any(|&c| c > 7) {
                        return Err(TranslateError::Todo);
                    }
                    let vec4_b8_type =
                        map.get_or_add(builder, SpirvType::Vector(SpirvScalarKey::B8, 4));
                    let b32_type = map.get_or_add_scalar(builder, ast::ScalarType::B32);
                    let src1_vector = builder.bitcast(vec4_b8_type, None, arg.src1)?;
                    let src2_vector = builder.bitcast(vec4_b8_type, None, arg.src2)?;
                    let dst_vector = builder.vector_shuffle(
                        vec4_b8_type,
                        None,
                        src1_vector,
                        src2_vector,
                        components,
                    )?;
                    builder.bitcast(b32_type, Some(arg.dst), dst_vector)?;
                }
                ast::Instruction::Membar { level } => {
                    let (scope, semantics) = match level {
                        ast::MemScope::Cta => (
                            spirv::Scope::Workgroup,
                            spirv::MemorySemantics::CROSS_WORKGROUP_MEMORY
                                | spirv::MemorySemantics::WORKGROUP_MEMORY
                                | spirv::MemorySemantics::SEQUENTIALLY_CONSISTENT,
                        ),
                        ast::MemScope::Gpu => (
                            spirv::Scope::Device,
                            spirv::MemorySemantics::CROSS_WORKGROUP_MEMORY
                                | spirv::MemorySemantics::WORKGROUP_MEMORY
                                | spirv::MemorySemantics::SEQUENTIALLY_CONSISTENT,
                        ),
                        ast::MemScope::Sys => (
                            spirv::Scope::CrossDevice,
                            spirv::MemorySemantics::CROSS_WORKGROUP_MEMORY
                                | spirv::MemorySemantics::WORKGROUP_MEMORY
                                | spirv::MemorySemantics::SEQUENTIALLY_CONSISTENT,
                        ),
                    };
                    let spirv_scope = map.get_or_add_constant(
                        builder,
                        &ast::Type::Scalar(ast::ScalarType::U32),
                        &vec_repr(scope as u32),
                    )?;
                    let spirv_semantics = map.get_or_add_constant(
                        builder,
                        &ast::Type::Scalar(ast::ScalarType::U32),
                        &vec_repr(semantics),
                    )?;
                    builder.memory_barrier(spirv_scope, spirv_semantics)?;
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
                            SpirvType::pointer_to(
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
                            [index_spirv].iter().copied(),
                        )?
                    }
                    None => details.arg.src1,
                };
                builder.store(dst_ptr, details.arg.src2, None, iter::empty())?;
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
                    SpirvType::new(ast::Type::Pointer(ast::ScalarType::U8, *state_space)),
                );
                let result_type = map.get_or_add(
                    builder,
                    SpirvType::pointer_to(underlying_type.clone(), state_space.to_spirv()),
                );
                let ptr_src_u8 = builder.bitcast(u8_pointer, None, *ptr_src)?;
                let temp = builder.in_bounds_ptr_access_chain(
                    u8_pointer,
                    None,
                    ptr_src_u8,
                    *offset_src,
                    iter::empty(),
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
                            [index as u32].iter().copied(),
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
                            [index as u32].iter().copied(),
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
    let temp_neg = logical_not(builder, result_type, None, temp_and)?;
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
        ast::SqrtKind::Approx => (spirv::CLOp::sqrt, None),
        ast::SqrtKind::Rounding(rnd) => (spirv::CLOp::sqrt, Some(rnd)),
    };
    builder.ext_inst(
        result_type,
        Some(a.dst),
        opencl,
        ocl_op as spirv::Word,
        [dr::Operand::IdRef(a.src)].iter().cloned(),
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
                [dr::Operand::FPFastMathMode(
                    spirv::FPFastMathMode::ALLOW_RECIP,
                )]
                .iter()
                .cloned(),
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
        ast::AtomInnerDetails::Float { op, typ } => {
            let spirv_op: fn(&mut dr::Builder, _, _, _, _, _, _) -> _ = match op {
                ast::AtomFloatOp::Add => dr::Builder::atomic_f_add_ext,
            };
            (spirv_op, typ.into())
        }
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
    opencl: spirv::Word,
    desc: &ast::RcpDetails,
    arg: &ast::Arg2<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let (instr_type, constant) = if desc.is_f64 {
        (ast::ScalarType::F64, vec_repr(1.0f64))
    } else {
        (ast::ScalarType::F32, vec_repr(1.0f32))
    };
    let result_type = map.get_or_add_scalar(builder, instr_type);
    if !desc.is_f64 && desc.rounding.is_none() {
        builder.ext_inst(
            result_type,
            Some(arg.dst),
            opencl,
            spirv::CLOp::native_recip as u32,
            [dr::Operand::IdRef(arg.src)].iter().cloned(),
        )?;
        return Ok(());
    }
    let one = map.get_or_add_constant(builder, &ast::Type::Scalar(instr_type), &constant)?;
    builder.f_div(result_type, Some(arg.dst), one, arg.src)?;
    emit_rounding_decoration(builder, arg.dst, desc.rounding);
    builder.decorate(
        arg.dst,
        spirv::Decoration::FPFastMathMode,
        [dr::Operand::FPFastMathMode(
            spirv::FPFastMathMode::ALLOW_RECIP,
        )]
        .iter()
        .cloned(),
    );
    Ok(())
}

fn vec_repr<T: Copy>(t: T) -> Vec<u8> {
    let mut result = vec![0; mem::size_of::<T>()];
    unsafe { std::ptr::copy_nonoverlapping(&t, result.as_mut_ptr() as *mut _, 1) };
    result
}

fn emit_variable<'input>(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    id_defs: &GlobalStringIdResolver<'input>,
    linking: ast::LinkingDirective,
    var: &ast::Variable<spirv::Word>,
) -> Result<(), TranslateError> {
    let (must_init, st_class) = match var.state_space {
        ast::StateSpace::Reg | ast::StateSpace::Param | ast::StateSpace::Local => {
            (false, spirv::StorageClass::Function)
        }
        ast::StateSpace::Global => (true, spirv::StorageClass::CrossWorkgroup),
        ast::StateSpace::Shared => (false, spirv::StorageClass::Workgroup),
        ast::StateSpace::Const => (false, spirv::StorageClass::UniformConstant),
        ast::StateSpace::Generic => todo!(),
        ast::StateSpace::Sreg => todo!(),
    };
    let initalizer = if var.array_init.len() > 0 {
        Some(map.get_or_add_constant(
            builder,
            &ast::Type::from(var.v_type.clone()),
            &*var.array_init,
        )?)
    } else if must_init {
        let type_id = map.get_or_add(builder, SpirvType::new(var.v_type.clone()));
        Some(builder.constant_null(type_id, None))
    } else {
        None
    };
    let ptr_type_id = map.get_or_add(builder, SpirvType::pointer_to(var.v_type.clone(), st_class));
    builder.variable(ptr_type_id, Some(var.name), st_class, initalizer);
    if let Some(align) = var.align {
        builder.decorate(
            var.name,
            spirv::Decoration::Alignment,
            [dr::Operand::LiteralInt32(align)].iter().cloned(),
        );
    }
    if var.state_space != ast::StateSpace::Shared
        || !linking.contains(ast::LinkingDirective::EXTERN)
    {
        emit_linking_decoration(builder, id_defs, None, var.name, linking);
    }
    Ok(())
}

fn emit_linking_decoration<'input>(
    builder: &mut dr::Builder,
    id_defs: &GlobalStringIdResolver<'input>,
    name_override: Option<&str>,
    name: spirv::Word,
    linking: ast::LinkingDirective,
) {
    if linking == ast::LinkingDirective::NONE {
        return;
    }
    if linking.contains(ast::LinkingDirective::VISIBLE) {
        let string_name =
            name_override.unwrap_or_else(|| id_defs.reverse_variables.get(&name).unwrap());
        builder.decorate(
            name,
            spirv::Decoration::LinkageAttributes,
            [
                dr::Operand::LiteralString(string_name.to_string()),
                dr::Operand::LinkageType(spirv::LinkageType::Export),
            ]
            .iter()
            .cloned(),
        );
    } else if linking.contains(ast::LinkingDirective::EXTERN) {
        let string_name =
            name_override.unwrap_or_else(|| id_defs.reverse_variables.get(&name).unwrap());
        builder.decorate(
            name,
            spirv::Decoration::LinkageAttributes,
            [
                dr::Operand::LiteralString(string_name.to_string()),
                dr::Operand::LinkageType(spirv::LinkageType::Import),
            ]
            .iter()
            .cloned(),
        );
    }
    // TODO: handle LinkingDirective::WEAK
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
                [
                    dr::Operand::IdRef(arg.src1),
                    dr::Operand::IdRef(arg.src2),
                    dr::Operand::IdRef(arg.src3),
                ]
                .iter()
                .cloned(),
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
                [
                    dr::Operand::IdRef(arg.src1),
                    dr::Operand::IdRef(arg.src2),
                    dr::Operand::IdRef(arg.src3),
                ]
                .iter()
                .cloned(),
            )?;
        }
        ast::MulIntControl::Wide => todo!(),
    };
    Ok(())
}

fn emit_fma_float(
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
        spirv::CLOp::fma as spirv::Word,
        [
            dr::Operand::IdRef(arg.src1),
            dr::Operand::IdRef(arg.src2),
            dr::Operand::IdRef(arg.src3),
        ]
        .iter()
        .cloned(),
    )?;
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
        [
            dr::Operand::IdRef(arg.src1),
            dr::Operand::IdRef(arg.src2),
            dr::Operand::IdRef(arg.src3),
        ]
        .iter()
        .cloned(),
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
    let inst_type = map.get_or_add(builder, SpirvType::new(desc.get_type()));
    builder.ext_inst(
        inst_type,
        Some(arg.dst),
        opencl,
        cl_op as spirv::Word,
        [dr::Operand::IdRef(arg.src1), dr::Operand::IdRef(arg.src2)]
            .iter()
            .cloned(),
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
    let inst_type = map.get_or_add(builder, SpirvType::new(desc.get_type()));
    builder.ext_inst(
        inst_type,
        Some(arg.dst),
        opencl,
        cl_op as spirv::Word,
        [dr::Operand::IdRef(arg.src1), dr::Operand::IdRef(arg.src2)]
            .iter()
            .cloned(),
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
                            [dr::Operand::IdRef(arg.src)].iter().cloned(),
                        )?;
                    }
                    Some(ast::RoundingMode::Zero) => {
                        builder.ext_inst(
                            result_type,
                            Some(arg.dst),
                            opencl,
                            spirv::CLOp::trunc as u32,
                            [dr::Operand::IdRef(arg.src)].iter().cloned(),
                        )?;
                    }
                    Some(ast::RoundingMode::NegativeInf) => {
                        builder.ext_inst(
                            result_type,
                            Some(arg.dst),
                            opencl,
                            spirv::CLOp::floor as u32,
                            [dr::Operand::IdRef(arg.src)].iter().cloned(),
                        )?;
                    }
                    Some(ast::RoundingMode::PositiveInf) => {
                        builder.ext_inst(
                            result_type,
                            Some(arg.dst),
                            opencl,
                            spirv::CLOp::ceil as u32,
                            [dr::Operand::IdRef(arg.src)].iter().cloned(),
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
            if desc.src.kind() == ast::ScalarKind::Signed {
                builder.convert_s_to_f(result_type, Some(arg.dst), arg.src)?;
            } else {
                builder.convert_u_to_f(result_type, Some(arg.dst), arg.src)?;
            }
            emit_rounding_decoration(builder, arg.dst, desc.rounding);
        }
        ast::CvtDetails::IntFromFloat(desc) => {
            let dest_t: ast::ScalarType = desc.dst.into();
            let result_type = map.get_or_add(builder, SpirvType::from(dest_t));
            if desc.dst.kind() == ast::ScalarKind::Signed {
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
            let src = if desc.dst.size_of() != desc.src.size_of() {
                let new_dst = if dest_t.kind() == src_t.kind() {
                    arg.dst
                } else {
                    builder.id()
                };
                let cv = ImplicitConversion {
                    src: arg.src,
                    dst: new_dst,
                    from_type: ast::Type::Scalar(src_t),
                    from_space: ast::StateSpace::Reg,
                    to_type: ast::Type::Scalar(ast::ScalarType::from_parts(
                        dest_t.size_of(),
                        src_t.kind(),
                    )),
                    to_space: ast::StateSpace::Reg,
                    kind: ConversionKind::Default,
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
                if desc.dst.kind() == ast::ScalarKind::Signed {
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
        builder.decorate(dst, spirv::Decoration::SaturatedConversion, iter::empty());
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
            [rounding.to_spirv()].iter().cloned(),
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
        (ast::SetpCompareOp::Eq, ast::ScalarKind::Signed)
        | (ast::SetpCompareOp::Eq, ast::ScalarKind::Unsigned)
        | (ast::SetpCompareOp::Eq, ast::ScalarKind::Bit) => {
            builder.i_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::Eq, ast::ScalarKind::Float) => {
            builder.f_ord_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::NotEq, ast::ScalarKind::Signed)
        | (ast::SetpCompareOp::NotEq, ast::ScalarKind::Unsigned)
        | (ast::SetpCompareOp::NotEq, ast::ScalarKind::Bit) => {
            builder.i_not_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::NotEq, ast::ScalarKind::Float) => {
            builder.f_ord_not_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::Less, ast::ScalarKind::Unsigned)
        | (ast::SetpCompareOp::Less, ast::ScalarKind::Bit) => {
            builder.u_less_than(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::Less, ast::ScalarKind::Signed) => {
            builder.s_less_than(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::Less, ast::ScalarKind::Float) => {
            builder.f_ord_less_than(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::LessOrEq, ast::ScalarKind::Unsigned)
        | (ast::SetpCompareOp::LessOrEq, ast::ScalarKind::Bit) => {
            builder.u_less_than_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::LessOrEq, ast::ScalarKind::Signed) => {
            builder.s_less_than_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::LessOrEq, ast::ScalarKind::Float) => {
            builder.f_ord_less_than_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::Greater, ast::ScalarKind::Unsigned)
        | (ast::SetpCompareOp::Greater, ast::ScalarKind::Bit) => {
            builder.u_greater_than(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::Greater, ast::ScalarKind::Signed) => {
            builder.s_greater_than(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::Greater, ast::ScalarKind::Float) => {
            builder.f_ord_greater_than(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::GreaterOrEq, ast::ScalarKind::Unsigned)
        | (ast::SetpCompareOp::GreaterOrEq, ast::ScalarKind::Bit) => {
            builder.u_greater_than_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::GreaterOrEq, ast::ScalarKind::Signed) => {
            builder.s_greater_than_equal(result_type, result_id, operand_1, operand_2)
        }
        (ast::SetpCompareOp::GreaterOrEq, ast::ScalarKind::Float) => {
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
        (ast::SetpCompareOp::IsAnyNan, _) => {
            let temp1 = builder.is_nan(result_type, None, operand_1)?;
            let temp2 = builder.is_nan(result_type, None, operand_2)?;
            builder.logical_or(result_type, result_id, temp1, temp2)
        }
        (ast::SetpCompareOp::IsNotNan, _) => {
            let temp1 = builder.is_nan(result_type, None, operand_1)?;
            let temp2 = builder.is_nan(result_type, None, operand_2)?;
            let any_nan = builder.logical_or(result_type, None, temp1, temp2)?;
            logical_not(builder, result_type, result_id, any_nan)
        }
        _ => todo!(),
    }?;
    Ok(())
}

// HACK ALERT
// Temporary workaround until IGC gets its shit together
// Currently IGC carries two copies of SPIRV-LLVM translator
// a new one in /llvm-spirv/ and old one in /IGC/AdaptorOCL/SPIRV/.
// Obviously, old and buggy one is used for compiling L0 SPIRV
// https://github.com/intel/intel-graphics-compiler/issues/148
fn logical_not(
    builder: &mut dr::Builder,
    result_type: spirv::Word,
    result_id: Option<spirv::Word>,
    operand: spirv::Word,
) -> Result<spirv::Word, dr::Error> {
    let const_true = builder.constant_true(result_type, None);
    let const_false = builder.constant_false(result_type, None);
    builder.select(result_type, result_id, operand, const_false, const_true)
}

fn emit_mul_sint(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    opencl: spirv::Word,
    desc: &ast::MulSInt,
    arg: &ast::Arg3<ExpandedArgParams>,
) -> Result<(), dr::Error> {
    let instruction_type = desc.typ;
    let inst_type = map.get_or_add(builder, SpirvType::from(desc.typ));
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
                [dr::Operand::IdRef(arg.src1), dr::Operand::IdRef(arg.src2)]
                    .iter()
                    .cloned(),
            )?;
        }
        ast::MulIntControl::Wide => {
            let instr_width = instruction_type.size_of();
            let instr_kind = instruction_type.kind();
            let dst_type = ast::ScalarType::from_parts(instr_width * 2, instr_kind);
            let dst_type_id = map.get_or_add_scalar(builder, dst_type);
            let src1 = builder.s_convert(dst_type_id, None, arg.src1)?;
            let src2 = builder.s_convert(dst_type_id, None, arg.src2)?;
            builder.i_mul(dst_type_id, Some(arg.dst), src1, src2)?;
            builder.decorate(arg.dst, spirv::Decoration::NoSignedWrap, iter::empty());
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
                [dr::Operand::IdRef(arg.src1), dr::Operand::IdRef(arg.src2)]
                    .iter()
                    .cloned(),
            )?;
        }
        ast::MulIntControl::Wide => {
            let instr_width = instruction_type.size_of();
            let instr_kind = instruction_type.kind();
            let dst_type = ast::ScalarType::from_parts(instr_width * 2, instr_kind);
            let dst_type_id = map.get_or_add_scalar(builder, dst_type);
            let src1 = builder.u_convert(dst_type_id, None, arg.src1)?;
            let src2 = builder.u_convert(dst_type_id, None, arg.src2)?;
            builder.i_mul(dst_type_id, Some(arg.dst), src1, src2)?;
            builder.decorate(arg.dst, spirv::Decoration::NoUnsignedWrap, iter::empty());
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
    let low_bits = builder.composite_extract(instruction_type, None, src, [0].iter().copied())?;
    let high_bits = builder.composite_extract(instruction_type, None, src, [1].iter().copied())?;
    let vector_type = map.get_or_add(builder, SpirvType::Vector(base_type_key, 2));
    let vector =
        builder.composite_construct(vector_type, None, [low_bits, high_bits].iter().copied())?;
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
    let cl_abs = if scalar_t.kind() == ast::ScalarKind::Signed {
        spirv::CLOp::s_abs
    } else {
        spirv::CLOp::fabs
    };
    builder.ext_inst(
        result_type,
        Some(arg.dst),
        opencl,
        cl_abs as spirv::Word,
        [dr::Operand::IdRef(arg.src)].iter().cloned(),
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
    let from_parts = cv.from_type.to_parts();
    let to_parts = cv.to_type.to_parts();
    match (from_parts.kind, to_parts.kind, &cv.kind) {
        (_, _, &ConversionKind::BitToPtr) => {
            let dst_type = map.get_or_add(
                builder,
                SpirvType::pointer_to(cv.to_type.clone(), cv.to_space.to_spirv()),
            );
            builder.convert_u_to_ptr(dst_type, Some(cv.dst), cv.src)?;
        }
        (TypeKind::Scalar, TypeKind::Scalar, &ConversionKind::Default) => {
            if from_parts.width == to_parts.width {
                let dst_type = map.get_or_add(builder, SpirvType::new(cv.to_type.clone()));
                if from_parts.scalar_kind != ast::ScalarKind::Float
                    && to_parts.scalar_kind != ast::ScalarKind::Float
                {
                    // It is noop, but another instruction expects result of this conversion
                    builder.copy_object(dst_type, Some(cv.dst), cv.src)?;
                } else {
                    builder.bitcast(dst_type, Some(cv.dst), cv.src)?;
                }
            } else {
                // This block is safe because it's illegal to implictly convert between floating point values
                let same_width_bit_type = map.get_or_add(
                    builder,
                    SpirvType::new(ast::Type::from_parts(TypeParts {
                        scalar_kind: ast::ScalarKind::Bit,
                        ..from_parts
                    })),
                );
                let same_width_bit_value = builder.bitcast(same_width_bit_type, None, cv.src)?;
                let wide_bit_type = ast::Type::from_parts(TypeParts {
                    scalar_kind: ast::ScalarKind::Bit,
                    ..to_parts
                });
                let wide_bit_type_spirv =
                    map.get_or_add(builder, SpirvType::new(wide_bit_type.clone()));
                if to_parts.scalar_kind == ast::ScalarKind::Unsigned
                    || to_parts.scalar_kind == ast::ScalarKind::Bit
                {
                    builder.u_convert(wide_bit_type_spirv, Some(cv.dst), same_width_bit_value)?;
                } else {
                    let conversion_fn = if from_parts.scalar_kind == ast::ScalarKind::Signed
                        && to_parts.scalar_kind == ast::ScalarKind::Signed
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
                            from_type: wide_bit_type,
                            from_space: cv.from_space,
                            to_type: cv.to_type.clone(),
                            to_space: cv.to_space,
                            kind: ConversionKind::Default,
                        },
                    )?;
                }
            }
        }
        (TypeKind::Scalar, TypeKind::Scalar, &ConversionKind::SignExtend) => {
            let result_type = map.get_or_add(builder, SpirvType::new(cv.to_type.clone()));
            builder.s_convert(result_type, Some(cv.dst), cv.src)?;
        }
        (TypeKind::Vector, TypeKind::Scalar, &ConversionKind::Default)
        | (TypeKind::Scalar, TypeKind::Array, &ConversionKind::Default)
        | (TypeKind::Array, TypeKind::Scalar, &ConversionKind::Default) => {
            let into_type = map.get_or_add(builder, SpirvType::new(cv.to_type.clone()));
            builder.bitcast(into_type, Some(cv.dst), cv.src)?;
        }
        (_, _, &ConversionKind::PtrToPtr) => {
            let result_type = map.get_or_add(
                builder,
                SpirvType::Pointer(
                    Box::new(SpirvType::new(cv.to_type.clone())),
                    cv.to_space.to_spirv(),
                ),
            );
            if cv.to_space == ast::StateSpace::Generic && cv.from_space != ast::StateSpace::Generic
            {
                let src = if cv.from_type != cv.to_type {
                    let temp_type = map.get_or_add(
                        builder,
                        SpirvType::Pointer(
                            Box::new(SpirvType::new(cv.to_type.clone())),
                            cv.from_space.to_spirv(),
                        ),
                    );
                    builder.bitcast(temp_type, None, cv.src)?
                } else {
                    cv.src
                };
                builder.ptr_cast_to_generic(result_type, Some(cv.dst), src)?;
            } else if cv.from_space == ast::StateSpace::Generic
                && cv.to_space != ast::StateSpace::Generic
            {
                let src = if cv.from_type != cv.to_type {
                    let temp_type = map.get_or_add(
                        builder,
                        SpirvType::Pointer(
                            Box::new(SpirvType::new(cv.to_type.clone())),
                            cv.from_space.to_spirv(),
                        ),
                    );
                    builder.bitcast(temp_type, None, cv.src)?
                } else {
                    cv.src
                };
                builder.generic_cast_to_ptr(result_type, Some(cv.dst), src)?;
            } else {
                builder.bitcast(result_type, Some(cv.dst), cv.src)?;
            }
        }
        (_, _, &ConversionKind::AddressOf) => {
            let dst_type = map.get_or_add(builder, SpirvType::new(cv.to_type.clone()));
            builder.convert_ptr_to_u(dst_type, Some(cv.dst), cv.src)?;
        }
        (TypeKind::Pointer, TypeKind::Scalar, &ConversionKind::Default) => {
            let result_type = map.get_or_add(builder, SpirvType::new(cv.to_type.clone()));
            builder.convert_ptr_to_u(result_type, Some(cv.dst), cv.src)?;
        }
        (TypeKind::Scalar, TypeKind::Pointer, &ConversionKind::Default) => {
            let result_type = map.get_or_add(builder, SpirvType::new(cv.to_type.clone()));
            builder.convert_u_to_ptr(result_type, Some(cv.dst), cv.src)?;
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
    let result_type = map.get_or_add(builder, SpirvType::new(details.typ.clone()));
    match details.member_index {
        Some((index, Some(width))) => {
            let vector_type = match details.typ {
                ast::Type::Scalar(scalar_t) => ast::Type::Vector(scalar_t, width),
                _ => return Err(TranslateError::MismatchedType),
            };
            let vector_type_spirv = map.get_or_add(builder, SpirvType::new(vector_type));
            let vector_temp = builder.load(
                vector_type_spirv,
                None,
                details.arg.src,
                None,
                iter::empty(),
            )?;
            builder.composite_extract(
                result_type,
                Some(details.arg.dst),
                vector_temp,
                [index as u32].iter().copied(),
            )?;
        }
        Some((index, None)) => {
            let result_ptr_type = map.get_or_add(
                builder,
                SpirvType::pointer_to(details.typ.clone(), spirv::StorageClass::Function),
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
                [index_spirv].iter().copied(),
            )?;
            builder.load(result_type, Some(details.arg.dst), src, None, iter::empty())?;
        }
        None => {
            builder.load(
                result_type,
                Some(details.arg.dst),
                details.arg.src,
                None,
                iter::empty(),
            )?;
        }
    };
    Ok(())
}

fn normalize_identifiers<'input, 'b>(
    id_defs: &mut FnStringIdResolver<'input, 'b>,
    fn_defs: &GlobalFnDeclResolver<'input, 'b>,
    func: Vec<ast::Statement<ast::ParsedArgParams<'input>>>,
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
            let var_type = var.var.v_type.clone();
            match var.count {
                Some(count) => {
                    for new_id in
                        id_defs.add_defs(var.var.name, count, var_type, var.var.state_space, true)
                    {
                        result.push(Statement::Variable(ast::Variable {
                            align: var.var.align,
                            v_type: var.var.v_type.clone(),
                            state_space: var.var.state_space,
                            name: new_id,
                            array_init: var.var.array_init.clone(),
                        }))
                    }
                }
                None => {
                    let new_id =
                        id_defs.add_def(var.var.name, Some((var_type, var.var.state_space)), true);
                    result.push(Statement::Variable(ast::Variable {
                        align: var.var.align,
                        v_type: var.var.v_type.clone(),
                        state_space: var.var.state_space,
                        name: new_id,
                        array_init: var.var.array_init,
                    }));
                }
            }
        }
    };
    Ok(())
}

/*
    Our goal here is to transform
        .visible .entry foobar(.param .u64 input) {
            .reg .b64 in_addr;
            .reg .b64 in_addr2;
            ld.param.u64 in_addr, [input];
            cvta.to.global.u64  in_addr2, in_addr;
        }
    into:
        .visible .entry foobar(.param .u8 input[]) {
            .reg .u8 in_addr[];
            .reg .u8 in_addr2[];
            ld.param.u8[] in_addr, [input];
            mov.u8[] in_addr2, in_addr;
        }
    or:
        .visible .entry foobar(.reg .u8 input[]) {
            .reg .u8 in_addr[];
            .reg .u8 in_addr2[];
            mov.u8[] in_addr, input;
            mov.u8[] in_addr2, in_addr;
        }
    or:
        .visible .entry foobar(.param ptr<u8, global> input) {
            .reg ptr<u8, global> in_addr;
            .reg ptr<u8, global> in_addr2;
            ld.param.ptr<u8, global> in_addr, [input];
            mov.ptr<u8, global> in_addr2, in_addr;
        }
*/
// TODO: detect more patterns (mov, call via reg, call via param)
// TODO: don't convert to ptr if the register is not ultimately used for ld/st
// TODO: once insert_mem_ssa_statements is moved to later, move this pass after
//       argument expansion
// TODO: propagate out of calls and into calls
fn convert_to_stateful_memory_access<'a, 'input>(
    func_args: Rc<RefCell<ast::MethodDeclaration<'input, spirv::Word>>>,
    func_body: Vec<TypedStatement>,
    id_defs: &mut NumericIdResolver<'a>,
) -> Result<
    (
        Rc<RefCell<ast::MethodDeclaration<'input, spirv::Word>>>,
        Vec<TypedStatement>,
    ),
    TranslateError,
> {
    let mut method_decl = func_args.borrow_mut();
    if !method_decl.name.is_kernel() {
        drop(method_decl);
        return Ok((func_args, func_body));
    }
    if Rc::strong_count(&func_args) != 1 {
        return Err(error_unreachable());
    }
    let func_args_64bit = (*method_decl)
        .input_arguments
        .iter()
        .filter_map(|arg| match arg.v_type {
            ast::Type::Scalar(ast::ScalarType::U64)
            | ast::Type::Scalar(ast::ScalarType::B64)
            | ast::Type::Scalar(ast::ScalarType::S64) => Some(arg.name),
            _ => None,
        })
        .collect::<HashSet<_>>();
    let mut stateful_markers = Vec::new();
    let mut stateful_init_reg = HashMap::<_, Vec<_>>::new();
    for statement in func_body.iter() {
        match statement {
            Statement::Instruction(ast::Instruction::Cvta(
                ast::CvtaDetails {
                    to: ast::StateSpace::Global,
                    size: ast::CvtaSize::U64,
                    from: ast::StateSpace::Generic,
                },
                arg,
            )) => {
                if let (TypedOperand::Reg(dst), Some(src)) =
                    (arg.dst, arg.src.upcast().underlying_register())
                {
                    if is_64_bit_integer(id_defs, *src) && is_64_bit_integer(id_defs, dst) {
                        stateful_markers.push((dst, *src));
                    }
                }
            }
            Statement::Instruction(ast::Instruction::Ld(
                ast::LdDetails {
                    state_space: ast::StateSpace::Param,
                    typ: ast::Type::Scalar(ast::ScalarType::U64),
                    ..
                },
                arg,
            ))
            | Statement::Instruction(ast::Instruction::Ld(
                ast::LdDetails {
                    state_space: ast::StateSpace::Param,
                    typ: ast::Type::Scalar(ast::ScalarType::S64),
                    ..
                },
                arg,
            ))
            | Statement::Instruction(ast::Instruction::Ld(
                ast::LdDetails {
                    state_space: ast::StateSpace::Param,
                    typ: ast::Type::Scalar(ast::ScalarType::B64),
                    ..
                },
                arg,
            )) => {
                if let (TypedOperand::Reg(dst), Some(src)) =
                    (&arg.dst, arg.src.upcast().underlying_register())
                {
                    if func_args_64bit.contains(src) {
                        multi_hash_map_append(&mut stateful_init_reg, *dst, *src);
                    }
                }
            }
            _ => {}
        }
    }
    if stateful_markers.len() == 0 {
        drop(method_decl);
        return Ok((func_args, func_body));
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
                    ast::ArithDetails::Unsigned(ast::ScalarType::U64),
                    arg,
                ))
                | Statement::Instruction(ast::Instruction::Add(
                    ast::ArithDetails::Signed(ast::ArithSInt {
                        typ: ast::ScalarType::S64,
                        saturate: false,
                    }),
                    arg,
                ))
                | Statement::Instruction(ast::Instruction::Sub(
                    ast::ArithDetails::Unsigned(ast::ScalarType::U64),
                    arg,
                ))
                | Statement::Instruction(ast::Instruction::Sub(
                    ast::ArithDetails::Signed(ast::ArithSInt {
                        typ: ast::ScalarType::S64,
                        saturate: false,
                    }),
                    arg,
                )) => {
                    // TODO: don't mark result of double pointer sub or double
                    // pointer add as ptr result
                    if let (TypedOperand::Reg(dst), Some(src1)) =
                        (arg.dst, arg.src1.upcast().underlying_register())
                    {
                        if regs_ptr_current.contains(src1) && !regs_ptr_seen.contains(src1) {
                            regs_ptr_new.insert(dst);
                        }
                    } else if let (TypedOperand::Reg(dst), Some(src2)) =
                        (arg.dst, arg.src2.upcast().underlying_register())
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
        let new_id = id_defs.register_variable(
            ast::Type::Pointer(ast::ScalarType::U8, ast::StateSpace::Global),
            ast::StateSpace::Reg,
        );
        result.push(Statement::Variable(ast::Variable {
            align: None,
            name: new_id,
            array_init: Vec::new(),
            v_type: ast::Type::Pointer(ast::ScalarType::U8, ast::StateSpace::Global),
            state_space: ast::StateSpace::Reg,
        }));
        remapped_ids.insert(reg, new_id);
    }
    for arg in (*method_decl).input_arguments.iter_mut() {
        if !func_args_ptr.contains(&arg.name) {
            continue;
        }
        let new_id = id_defs.register_variable(
            ast::Type::Pointer(ast::ScalarType::U8, ast::StateSpace::Global),
            ast::StateSpace::Param,
        );
        let old_name = arg.name;
        arg.v_type = ast::Type::Pointer(ast::ScalarType::U8, ast::StateSpace::Global);
        arg.name = new_id;
        remapped_ids.insert(old_name, new_id);
    }
    for statement in func_body {
        match statement {
            l @ Statement::Label(_) => result.push(l),
            c @ Statement::Conditional(_) => result.push(c),
            c @ Statement::Constant(..) => result.push(c),
            Statement::Variable(var) => {
                if !remapped_ids.contains_key(&var.name) {
                    result.push(Statement::Variable(var));
                }
            }
            Statement::Instruction(ast::Instruction::Add(
                ast::ArithDetails::Unsigned(ast::ScalarType::U64),
                arg,
            ))
            | Statement::Instruction(ast::Instruction::Add(
                ast::ArithDetails::Signed(ast::ArithSInt {
                    typ: ast::ScalarType::S64,
                    saturate: false,
                }),
                arg,
            )) if is_add_ptr_direct(&remapped_ids, &arg) => {
                let (ptr, offset) = match arg.src1.upcast().underlying_register() {
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
                    underlying_type: ast::Type::Scalar(ast::ScalarType::U8),
                    state_space: ast::StateSpace::Global,
                    dst: *remapped_ids.get(&dst).unwrap(),
                    ptr_src: *ptr,
                    offset_src: offset,
                }))
            }
            Statement::Instruction(ast::Instruction::Sub(
                ast::ArithDetails::Unsigned(ast::ScalarType::U64),
                arg,
            ))
            | Statement::Instruction(ast::Instruction::Sub(
                ast::ArithDetails::Signed(ast::ArithSInt {
                    typ: ast::ScalarType::S64,
                    saturate: false,
                }),
                arg,
            )) if is_sub_ptr_direct(&remapped_ids, &arg) => {
                let (ptr, offset) = match arg.src1.upcast().underlying_register() {
                    Some(src1) => (remapped_ids.get(src1).unwrap(), arg.src2),
                    _ => return Err(error_unreachable()),
                };
                let offset_neg = id_defs.register_intermediate(Some((
                    ast::Type::Scalar(ast::ScalarType::S64),
                    ast::StateSpace::Reg,
                )));
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
                    underlying_type: ast::Type::Scalar(ast::ScalarType::U8),
                    state_space: ast::StateSpace::Global,
                    dst: *remapped_ids.get(&dst).unwrap(),
                    ptr_src: *ptr,
                    offset_src: TypedOperand::Reg(offset_neg),
                }))
            }
            Statement::Instruction(inst) => {
                let mut post_statements = Vec::new();
                let new_statement =
                    inst.visit(&mut |arg_desc, expected_type: Option<(&ast::Type, _)>| {
                        convert_to_stateful_memory_access_postprocess(
                            id_defs,
                            &remapped_ids,
                            &mut result,
                            &mut post_statements,
                            arg_desc,
                            expected_type,
                        )
                    })?;
                result.push(new_statement);
                result.extend(post_statements);
            }
            Statement::Call(call) => {
                let mut post_statements = Vec::new();
                let new_statement =
                    call.visit(&mut |arg_desc, expected_type: Option<(&ast::Type, _)>| {
                        convert_to_stateful_memory_access_postprocess(
                            id_defs,
                            &remapped_ids,
                            &mut result,
                            &mut post_statements,
                            arg_desc,
                            expected_type,
                        )
                    })?;
                result.push(new_statement);
                result.extend(post_statements);
            }
            Statement::RepackVector(pack) => {
                let mut post_statements = Vec::new();
                let new_statement =
                    pack.visit(&mut |arg_desc, expected_type: Option<(&ast::Type, _)>| {
                        convert_to_stateful_memory_access_postprocess(
                            id_defs,
                            &remapped_ids,
                            &mut result,
                            &mut post_statements,
                            arg_desc,
                            expected_type,
                        )
                    })?;
                result.push(new_statement);
                result.extend(post_statements);
            }
            _ => return Err(error_unreachable()),
        }
    }
    drop(method_decl);
    Ok((func_args, result))
}

fn convert_to_stateful_memory_access_postprocess(
    id_defs: &mut NumericIdResolver,
    remapped_ids: &HashMap<spirv::Word, spirv::Word>,
    result: &mut Vec<TypedStatement>,
    post_statements: &mut Vec<TypedStatement>,
    arg_desc: ArgumentDescriptor<spirv::Word>,
    expected_type: Option<(&ast::Type, ast::StateSpace)>,
) -> Result<spirv::Word, TranslateError> {
    Ok(match remapped_ids.get(&arg_desc.op) {
        Some(new_id) => {
            let (new_operand_type, new_operand_space, _) = id_defs.get_typed(*new_id)?;
            if let Some((expected_type, expected_space)) = expected_type {
                let implicit_conversion = arg_desc
                    .non_default_implicit_conversion
                    .unwrap_or(default_implicit_conversion);
                if implicit_conversion(
                    (new_operand_space, &new_operand_type),
                    (expected_space, expected_type),
                )
                .is_ok()
                {
                    return Ok(*new_id);
                }
            }
            let (old_operand_type, old_operand_space, _) = id_defs.get_typed(arg_desc.op)?;
            let converting_id =
                id_defs.register_intermediate(Some((old_operand_type.clone(), old_operand_space)));
            let kind = if new_operand_space.is_compatible(ast::StateSpace::Reg) {
                ConversionKind::Default
            } else {
                ConversionKind::PtrToPtr
            };
            if arg_desc.is_dst {
                post_statements.push(Statement::Conversion(ImplicitConversion {
                    src: converting_id,
                    dst: *new_id,
                    from_type: old_operand_type,
                    from_space: old_operand_space,
                    to_type: new_operand_type,
                    to_space: new_operand_space,
                    kind,
                }));
                converting_id
            } else {
                result.push(Statement::Conversion(ImplicitConversion {
                    src: *new_id,
                    dst: converting_id,
                    from_type: new_operand_type,
                    from_space: new_operand_space,
                    to_type: old_operand_type,
                    to_space: old_operand_space,
                    kind,
                }));
                converting_id
            }
        }
        None => arg_desc.op,
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
            if let Some(src1_reg) = arg.src1.upcast().underlying_register() {
                if remapped_ids.contains_key(src1_reg) {
                    // don't trigger optimization when adding two pointers
                    if let Some(src2_reg) = arg.src2.upcast().underlying_register() {
                        return !remapped_ids.contains_key(src2_reg);
                    }
                }
            }
            if let Some(src2_reg) = arg.src2.upcast().underlying_register() {
                remapped_ids.contains_key(src2_reg)
            } else {
                false
            }
        }
    }
}

fn is_sub_ptr_direct(remapped_ids: &HashMap<u32, u32>, arg: &ast::Arg3<TypedArgParams>) -> bool {
    match arg.dst {
        TypedOperand::Imm(..) | TypedOperand::RegOffset(..) | TypedOperand::VecMember(..) => {
            return false
        }
        TypedOperand::Reg(dst) => {
            if !remapped_ids.contains_key(&dst) {
                return false;
            }
            match arg.src1.upcast().underlying_register() {
                Some(src1_reg) => {
                    if remapped_ids.contains_key(src1_reg) {
                        // don't trigger optimization when subtracting two pointers
                        arg.src2
                            .upcast()
                            .underlying_register()
                            .map_or(true, |src2_reg| !remapped_ids.contains_key(src2_reg))
                    } else {
                        false
                    }
                }
                None => false,
            }
        }
    }
}

fn is_64_bit_integer(id_defs: &NumericIdResolver, id: spirv::Word) -> bool {
    match id_defs.get_typed(id) {
        Ok((ast::Type::Scalar(ast::ScalarType::U64), _, _))
        | Ok((ast::Type::Scalar(ast::ScalarType::S64), _, _))
        | Ok((ast::Type::Scalar(ast::ScalarType::B64), _, _)) => true,
        _ => false,
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
enum PtxSpecialRegister {
    Tid,
    Ntid,
    Ctaid,
    Nctaid,
    Clock,
    LanemaskLt,
}

impl PtxSpecialRegister {
    fn try_parse(s: &str) -> Option<Self> {
        match s {
            "%tid" => Some(Self::Tid),
            "%ntid" => Some(Self::Ntid),
            "%ctaid" => Some(Self::Ctaid),
            "%nctaid" => Some(Self::Nctaid),
            "%clock" => Some(Self::Clock),
            "%lanemask_lt" => Some(Self::LanemaskLt),
            _ => None,
        }
    }

    fn get_type(self) -> ast::Type {
        match self {
            PtxSpecialRegister::Tid
            | PtxSpecialRegister::Ntid
            | PtxSpecialRegister::Ctaid
            | PtxSpecialRegister::Nctaid => ast::Type::Vector(self.get_function_return_type(), 4),
            _ => ast::Type::Scalar(self.get_function_return_type()),
        }
    }

    fn get_function_return_type(self) -> ast::ScalarType {
        match self {
            PtxSpecialRegister::Tid => ast::ScalarType::U32,
            PtxSpecialRegister::Ntid => ast::ScalarType::U32,
            PtxSpecialRegister::Ctaid => ast::ScalarType::U32,
            PtxSpecialRegister::Nctaid => ast::ScalarType::U32,
            PtxSpecialRegister::Clock => ast::ScalarType::U32,
            PtxSpecialRegister::LanemaskLt => ast::ScalarType::U32,
        }
    }

    fn get_function_input_type(self) -> Option<ast::ScalarType> {
        match self {
            PtxSpecialRegister::Tid
            | PtxSpecialRegister::Ntid
            | PtxSpecialRegister::Ctaid
            | PtxSpecialRegister::Nctaid => Some(ast::ScalarType::U8),
            PtxSpecialRegister::Clock | PtxSpecialRegister::LanemaskLt => None,
        }
    }

    fn get_unprefixed_function_name(self) -> &'static str {
        match self {
            PtxSpecialRegister::Tid => "sreg_tid",
            PtxSpecialRegister::Ntid => "sreg_ntid",
            PtxSpecialRegister::Ctaid => "sreg_ctaid",
            PtxSpecialRegister::Nctaid => "sreg_nctaid",
            PtxSpecialRegister::Clock => "sreg_clock",
            PtxSpecialRegister::LanemaskLt => "sreg_lanemask_lt",
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

struct FnSigMapper<'input> {
    // true - stays as return argument
    // false - is moved to input argument
    return_param_args: Vec<bool>,
    func_decl: Rc<RefCell<ast::MethodDeclaration<'input, spirv::Word>>>,
}

impl<'input> FnSigMapper<'input> {
    fn remap_to_spirv_repr(mut method: ast::MethodDeclaration<'input, spirv::Word>) -> Self {
        let return_param_args = method
            .return_arguments
            .iter()
            .map(|a| a.state_space != ast::StateSpace::Param)
            .collect::<Vec<_>>();
        let mut new_return_arguments = Vec::new();
        for arg in method.return_arguments.into_iter() {
            if arg.state_space == ast::StateSpace::Param {
                method.input_arguments.push(arg);
            } else {
                new_return_arguments.push(arg);
            }
        }
        method.return_arguments = new_return_arguments;
        FnSigMapper {
            return_param_args,
            func_decl: Rc::new(RefCell::new(method)),
        }
    }

    fn resolve_in_spirv_repr(
        &self,
        call_inst: ast::CallInst<NormalizedArgParams>,
    ) -> Result<ResolvedCall<NormalizedArgParams>, TranslateError> {
        let func_decl = (*self.func_decl).borrow();
        let mut return_arguments = Vec::new();
        let mut input_arguments = call_inst
            .param_list
            .into_iter()
            .zip(func_decl.input_arguments.iter())
            .map(|(id, var)| (id, var.v_type.clone(), var.state_space))
            .collect::<Vec<_>>();
        let mut func_decl_return_iter = func_decl.return_arguments.iter();
        let mut func_decl_input_iter = func_decl.input_arguments[input_arguments.len()..].iter();
        for (idx, id) in call_inst.ret_params.iter().enumerate() {
            let stays_as_return = match self.return_param_args.get(idx) {
                Some(x) => *x,
                None => return Err(TranslateError::MismatchedType),
            };
            if stays_as_return {
                if let Some(var) = func_decl_return_iter.next() {
                    return_arguments.push((*id, var.v_type.clone(), var.state_space));
                } else {
                    return Err(TranslateError::MismatchedType);
                }
            } else {
                if let Some(var) = func_decl_input_iter.next() {
                    input_arguments.push((
                        ast::Operand::Reg(*id),
                        var.v_type.clone(),
                        var.state_space,
                    ));
                } else {
                    return Err(TranslateError::MismatchedType);
                }
            }
        }
        if return_arguments.len() != func_decl.return_arguments.len()
            || input_arguments.len() != func_decl.input_arguments.len()
        {
            return Err(TranslateError::MismatchedType);
        }
        Ok(ResolvedCall {
            return_arguments,
            input_arguments,
            uniform: call_inst.uniform,
            name: call_inst.func,
        })
    }
}

struct GlobalStringIdResolver<'input> {
    current_id: spirv::Word,
    variables: HashMap<Cow<'input, str>, spirv::Word>,
    reverse_variables: HashMap<spirv::Word, &'input str>,
    variables_type_check: HashMap<u32, Option<(ast::Type, ast::StateSpace, bool)>>,
    special_registers: SpecialRegistersMap,
    fns: HashMap<spirv::Word, FnSigMapper<'input>>,
}

impl<'input> GlobalStringIdResolver<'input> {
    fn new(start_id: spirv::Word) -> Self {
        Self {
            current_id: start_id,
            variables: HashMap::new(),
            reverse_variables: HashMap::new(),
            variables_type_check: HashMap::new(),
            special_registers: SpecialRegistersMap::new(),
            fns: HashMap::new(),
        }
    }

    fn get_or_add_def(&mut self, id: &'input str) -> spirv::Word {
        self.get_or_add_impl(id, None)
    }

    fn get_or_add_def_typed(
        &mut self,
        id: &'input str,
        typ: ast::Type,
        state_space: ast::StateSpace,
        is_variable: bool,
    ) -> spirv::Word {
        self.get_or_add_impl(id, Some((typ, state_space, is_variable)))
    }

    fn get_or_add_impl(
        &mut self,
        id: &'input str,
        typ: Option<(ast::Type, ast::StateSpace, bool)>,
    ) -> spirv::Word {
        let id = match self.variables.entry(Cow::Borrowed(id)) {
            hash_map::Entry::Occupied(e) => *(e.get()),
            hash_map::Entry::Vacant(e) => {
                let numeric_id = self.current_id;
                e.insert(numeric_id);
                self.reverse_variables.insert(numeric_id, id);
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
            .ok_or_else(error_unknown_symbol)
    }

    fn current_id(&self) -> spirv::Word {
        self.current_id
    }

    fn start_fn<'b>(
        &'b mut self,
        header: &'b ast::MethodDeclaration<'input, &'input str>,
    ) -> Result<
        (
            FnStringIdResolver<'input, 'b>,
            GlobalFnDeclResolver<'input, 'b>,
            Rc<RefCell<ast::MethodDeclaration<'input, spirv::Word>>>,
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
        let return_arguments = rename_fn_params(&mut fn_resolver, &header.return_arguments);
        let input_arguments = rename_fn_params(&mut fn_resolver, &header.input_arguments);
        let name = match header.name {
            ast::MethodName::Kernel(name) => ast::MethodName::Kernel(name),
            ast::MethodName::Func(_) => ast::MethodName::Func(name_id),
        };
        let fn_decl = ast::MethodDeclaration {
            return_arguments,
            name,
            input_arguments,
            shared_mem: None,
        };
        let new_fn_decl = if !fn_decl.name.is_kernel() {
            let resolver = FnSigMapper::remap_to_spirv_repr(fn_decl);
            let new_fn_decl = resolver.func_decl.clone();
            self.fns.insert(name_id, resolver);
            new_fn_decl
        } else {
            Rc::new(RefCell::new(fn_decl))
        };
        Ok((
            fn_resolver,
            GlobalFnDeclResolver { fns: &self.fns },
            new_fn_decl,
        ))
    }
}

pub struct GlobalFnDeclResolver<'input, 'a> {
    fns: &'a HashMap<spirv::Word, FnSigMapper<'input>>,
}

impl<'input, 'a> GlobalFnDeclResolver<'input, 'a> {
    fn get_fn_sig_resolver(&self, id: spirv::Word) -> Result<&FnSigMapper<'input>, TranslateError> {
        self.fns.get(&id).ok_or_else(error_unknown_symbol)
    }
}

struct FnStringIdResolver<'input, 'b> {
    current_id: &'b mut spirv::Word,
    global_variables: &'b HashMap<Cow<'input, str>, spirv::Word>,
    global_type_check: &'b HashMap<u32, Option<(ast::Type, ast::StateSpace, bool)>>,
    special_registers: &'b mut SpecialRegistersMap,
    variables: Vec<HashMap<Cow<'input, str>, spirv::Word>>,
    type_check: HashMap<u32, Option<(ast::Type, ast::StateSpace, bool)>>,
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
                let sreg = PtxSpecialRegister::try_parse(id).ok_or_else(error_unknown_symbol)?;
                Ok(self.special_registers.get_or_add(self.current_id, sreg))
            }
        }
    }

    fn add_def(
        &mut self,
        id: &'a str,
        typ: Option<(ast::Type, ast::StateSpace)>,
        is_variable: bool,
    ) -> spirv::Word {
        let numeric_id = *self.current_id;
        self.variables
            .last_mut()
            .unwrap()
            .insert(Cow::Borrowed(id), numeric_id);
        self.type_check.insert(
            numeric_id,
            typ.map(|(typ, space)| (typ, space, is_variable)),
        );
        *self.current_id += 1;
        numeric_id
    }

    #[must_use]
    fn add_defs(
        &mut self,
        base_id: &'a str,
        count: u32,
        typ: ast::Type,
        state_space: ast::StateSpace,
        is_variable: bool,
    ) -> impl Iterator<Item = spirv::Word> {
        let numeric_id = *self.current_id;
        for i in 0..count {
            self.variables
                .last_mut()
                .unwrap()
                .insert(Cow::Owned(format!("{}{}", base_id, i)), numeric_id + i);
            self.type_check.insert(
                numeric_id + i,
                Some((typ.clone(), state_space, is_variable)),
            );
        }
        *self.current_id += count;
        (0..count).into_iter().map(move |i| i + numeric_id)
    }
}

struct NumericIdResolver<'b> {
    current_id: &'b mut spirv::Word,
    global_type_check: &'b HashMap<u32, Option<(ast::Type, ast::StateSpace, bool)>>,
    type_check: HashMap<u32, Option<(ast::Type, ast::StateSpace, bool)>>,
    special_registers: &'b mut SpecialRegistersMap,
}

impl<'b> NumericIdResolver<'b> {
    fn finish(self) -> MutableNumericIdResolver<'b> {
        MutableNumericIdResolver { base: self }
    }

    fn get_typed(
        &self,
        id: spirv::Word,
    ) -> Result<(ast::Type, ast::StateSpace, bool), TranslateError> {
        match self.type_check.get(&id) {
            Some(Some(x)) => Ok(x.clone()),
            Some(None) => Err(TranslateError::UntypedSymbol),
            None => match self.special_registers.get(id) {
                Some(x) => Ok((x.get_type(), ast::StateSpace::Sreg, true)),
                None => match self.global_type_check.get(&id) {
                    Some(Some(result)) => Ok(result.clone()),
                    Some(None) | None => Err(TranslateError::UntypedSymbol),
                },
            },
        }
    }

    // This is for identifiers which will be emitted later as OpVariable
    // They are candidates for insertion of LoadVar/StoreVar
    fn register_variable(&mut self, typ: ast::Type, state_space: ast::StateSpace) -> spirv::Word {
        let new_id = *self.current_id;
        self.type_check
            .insert(new_id, Some((typ, state_space, true)));
        *self.current_id += 1;
        new_id
    }

    fn register_intermediate(&mut self, typ: Option<(ast::Type, ast::StateSpace)>) -> spirv::Word {
        let new_id = *self.current_id;
        self.type_check
            .insert(new_id, typ.map(|(t, space)| (t, space, false)));
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

    fn get_typed(&self, id: spirv::Word) -> Result<(ast::Type, ast::StateSpace), TranslateError> {
        self.base.get_typed(id).map(|(t, space, _)| (t, space))
    }

    fn register_intermediate(
        &mut self,
        typ: ast::Type,
        state_space: ast::StateSpace,
    ) -> spirv::Word {
        self.base.register_intermediate(Some((typ, state_space)))
    }
}

struct FunctionPointerDetails {
    dst: spirv::Word,
    src: spirv::Word,
}

impl<T: ArgParamsEx<Id = spirv::Word>, U: ArgParamsEx<Id = spirv::Word>> Visitable<T, U>
    for FunctionPointerDetails
{
    fn visit(
        self,
        visitor: &mut impl ArgumentMapVisitor<T, U>,
    ) -> Result<Statement<ast::Instruction<U>, U>, TranslateError> {
        Ok(Statement::FunctionPointer(FunctionPointerDetails {
            dst: visitor.id(
                ArgumentDescriptor {
                    op: self.dst,
                    is_dst: true,
                    is_memory_access: false,
                    non_default_implicit_conversion: None,
                },
                Some((
                    &ast::Type::Scalar(ast::ScalarType::U64),
                    ast::StateSpace::Reg,
                )),
            )?,
            src: visitor.id(
                ArgumentDescriptor {
                    op: self.src,
                    is_dst: false,
                    is_memory_access: false,
                    non_default_implicit_conversion: None,
                },
                None,
            )?,
        }))
    }
}

enum Statement<I, P: ast::ArgParams> {
    Label(u32),
    Variable(ast::Variable<P::Id>),
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
    FunctionPointer(FunctionPointerDetails),
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
                .visit(&mut |arg: ArgumentDescriptor<_>,
                             _: Option<(&ast::Type, ast::StateSpace)>| {
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
                for (id, _, space) in call.return_arguments.iter_mut() {
                    let is_dst = match space {
                        ast::StateSpace::Reg => true,
                        ast::StateSpace::Param => false,
                        ast::StateSpace::Shared => false,
                        _ => todo!(),
                    };
                    *id = f(*id, is_dst);
                }
                call.name = f(call.name, false);
                for (id, _, _) in call.input_arguments.iter_mut() {
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
            Statement::FunctionPointer(FunctionPointerDetails { dst, src }) => {
                Statement::FunctionPointer(FunctionPointerDetails {
                    dst: f(dst, true),
                    src: f(src, false),
                })
            }
        }
    }
}

struct LoadVarDetails {
    arg: ast::Arg2<ExpandedArgParams>,
    typ: ast::Type,
    state_space: ast::StateSpace,
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
    non_default_implicit_conversion: Option<
        fn(
            (ast::StateSpace, &ast::Type),
            (ast::StateSpace, &ast::Type),
        ) -> Result<Option<ConversionKind>, TranslateError>,
    >,
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
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            Some((
                &ast::Type::Vector(self.typ, self.unpacked.len() as u8),
                ast::StateSpace::Reg,
            )),
        )?;
        let scalar_type = self.typ;
        let is_extract = self.is_extract;
        let non_default_implicit_conversion = self.non_default_implicit_conversion;
        let vector = self
            .unpacked
            .into_iter()
            .map(|id| {
                visitor.id(
                    ArgumentDescriptor {
                        op: id,
                        is_dst: is_extract,
                        is_memory_access: false,
                        non_default_implicit_conversion,
                    },
                    Some((&ast::Type::Scalar(scalar_type), ast::StateSpace::Reg)),
                )
            })
            .collect::<Result<_, _>>()?;
        Ok(RepackVectorDetails {
            is_extract,
            typ: self.typ,
            packed: scalar,
            unpacked: vector,
            non_default_implicit_conversion,
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
    pub return_arguments: Vec<(P::Id, ast::Type, ast::StateSpace)>,
    pub name: P::Id,
    pub input_arguments: Vec<(P::Operand, ast::Type, ast::StateSpace)>,
}

impl<T: ast::ArgParams> ResolvedCall<T> {
    fn cast<U: ast::ArgParams<Id = T::Id, Operand = T::Operand>>(self) -> ResolvedCall<U> {
        ResolvedCall {
            uniform: self.uniform,
            return_arguments: self.return_arguments,
            name: self.name,
            input_arguments: self.input_arguments,
        }
    }
}

impl<From: ArgParamsEx<Id = spirv::Word>> ResolvedCall<From> {
    fn map<To: ArgParamsEx<Id = spirv::Word>, V: ArgumentMapVisitor<From, To>>(
        self,
        visitor: &mut V,
    ) -> Result<ResolvedCall<To>, TranslateError> {
        let return_arguments = self
            .return_arguments
            .into_iter()
            .map::<Result<_, TranslateError>, _>(|(id, typ, space)| {
                let new_id = visitor.id(
                    ArgumentDescriptor {
                        op: id,
                        is_dst: space != ast::StateSpace::Param,
                        is_memory_access: false,
                        non_default_implicit_conversion: None,
                    },
                    Some((&typ, space)),
                )?;
                Ok((new_id, typ, space))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let func = visitor.id(
            ArgumentDescriptor {
                op: self.name,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            None,
        )?;
        let input_arguments = self
            .input_arguments
            .into_iter()
            .map::<Result<_, TranslateError>, _>(|(id, typ, space)| {
                let new_id = visitor.operand(
                    ArgumentDescriptor {
                        op: id,
                        is_dst: false,
                        is_memory_access: false,
                        non_default_implicit_conversion: None,
                    },
                    &typ,
                    space,
                )?;
                Ok((new_id, typ, space))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ResolvedCall {
            uniform: self.uniform,
            return_arguments,
            name: func,
            input_arguments,
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
        let new_dst = visitor.id(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            Some((&self.underlying_type, self.state_space)),
        )?;
        let new_ptr_src = visitor.id(
            ArgumentDescriptor {
                op: self.ptr_src,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            Some((&self.underlying_type, self.state_space)),
        )?;
        let new_constant_src = visitor.operand(
            ArgumentDescriptor {
                op: self.offset_src,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::S64),
            ast::StateSpace::Reg,
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

pub trait ArgParamsEx: ast::ArgParams + Sized {}

impl<'input> ArgParamsEx for ast::ParsedArgParams<'input> {}

enum NormalizedArgParams {}

impl ast::ArgParams for NormalizedArgParams {
    type Id = spirv::Word;
    type Operand = ast::Operand<spirv::Word>;
}

impl ArgParamsEx for NormalizedArgParams {}

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

impl ArgParamsEx for TypedArgParams {}

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

impl ArgParamsEx for ExpandedArgParams {}

enum Directive<'input> {
    Variable(ast::LinkingDirective, ast::Variable<spirv::Word>),
    Method(Function<'input>),
}

struct Function<'input> {
    pub func_decl: Rc<RefCell<ast::MethodDeclaration<'input, spirv::Word>>>,
    pub globals: Vec<ast::Variable<spirv::Word>>,
    pub body: Option<Vec<ExpandedStatement>>,
    import_as: Option<String>,
    tuning: Vec<ast::TuningDirective>,
    linkage: ast::LinkingDirective,
}

pub trait ArgumentMapVisitor<T: ArgParamsEx, U: ArgParamsEx> {
    fn id(
        &mut self,
        desc: ArgumentDescriptor<T::Id>,
        typ: Option<(&ast::Type, ast::StateSpace)>,
    ) -> Result<U::Id, TranslateError>;
    fn operand(
        &mut self,
        desc: ArgumentDescriptor<T::Operand>,
        typ: &ast::Type,
        state_space: ast::StateSpace,
    ) -> Result<U::Operand, TranslateError>;
}

impl<T> ArgumentMapVisitor<ExpandedArgParams, ExpandedArgParams> for T
where
    T: FnMut(
        ArgumentDescriptor<spirv::Word>,
        Option<(&ast::Type, ast::StateSpace)>,
    ) -> Result<spirv::Word, TranslateError>,
{
    fn id(
        &mut self,
        desc: ArgumentDescriptor<spirv::Word>,
        t: Option<(&ast::Type, ast::StateSpace)>,
    ) -> Result<spirv::Word, TranslateError> {
        self(desc, t)
    }

    fn operand(
        &mut self,
        desc: ArgumentDescriptor<spirv::Word>,
        typ: &ast::Type,
        state_space: ast::StateSpace,
    ) -> Result<spirv::Word, TranslateError> {
        self(desc, Some((typ, state_space)))
    }
}

impl<'a, T> ArgumentMapVisitor<ast::ParsedArgParams<'a>, NormalizedArgParams> for T
where
    T: FnMut(&str) -> Result<spirv::Word, TranslateError>,
{
    fn id(
        &mut self,
        desc: ArgumentDescriptor<&str>,
        _: Option<(&ast::Type, ast::StateSpace)>,
    ) -> Result<spirv::Word, TranslateError> {
        self(desc.op)
    }

    fn operand(
        &mut self,
        desc: ArgumentDescriptor<ast::Operand<&str>>,
        typ: &ast::Type,
        state_space: ast::StateSpace,
    ) -> Result<ast::Operand<spirv::Word>, TranslateError> {
        Ok(match desc.op {
            ast::Operand::Reg(id) => ast::Operand::Reg(self(id)?),
            ast::Operand::RegOffset(id, imm) => ast::Operand::RegOffset(self(id)?, imm),
            ast::Operand::Imm(imm) => ast::Operand::Imm(imm),
            ast::Operand::VecMember(id, member) => ast::Operand::VecMember(self(id)?, member),
            ast::Operand::VecPack(ref ids) => ast::Operand::VecPack(
                ids.into_iter()
                    .map(|id| self.id(desc.new_op(id), Some((typ, state_space))))
                    .collect::<Result<Vec<_>, _>>()?,
            ),
        })
    }
}

pub struct ArgumentDescriptor<Op> {
    op: Op,
    is_dst: bool,
    is_memory_access: bool,
    non_default_implicit_conversion: Option<
        fn(
            (ast::StateSpace, &ast::Type),
            (ast::StateSpace, &ast::Type),
        ) -> Result<Option<ConversionKind>, TranslateError>,
    >,
}

pub struct PtrAccess<P: ast::ArgParams> {
    underlying_type: ast::Type,
    state_space: ast::StateSpace,
    dst: spirv::Word,
    ptr_src: spirv::Word,
    offset_src: P::Operand,
}

impl<T> ArgumentDescriptor<T> {
    fn new_op<U>(&self, u: U) -> ArgumentDescriptor<U> {
        ArgumentDescriptor {
            op: u,
            is_dst: self.is_dst,
            is_memory_access: self.is_memory_access,
            non_default_implicit_conversion: self.non_default_implicit_conversion,
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
            ast::Instruction::Not(t, a) => {
                ast::Instruction::Not(t, a.map(visitor, &ast::Type::Scalar(t))?)
            }
            ast::Instruction::Cvt(d, a) => {
                let (dst_t, src_t, int_to_int) = match &d {
                    ast::CvtDetails::FloatFromFloat(desc) => ((desc.dst, desc.src, false)),
                    ast::CvtDetails::FloatFromInt(desc) => ((desc.dst, desc.src, false)),
                    ast::CvtDetails::IntFromFloat(desc) => ((desc.dst, desc.src, false)),
                    ast::CvtDetails::IntFromInt(desc) => ((desc.dst, desc.src, true)),
                };
                ast::Instruction::Cvt(d, a.map_cvt(visitor, dst_t, src_t, int_to_int)?)
            }
            ast::Instruction::Shl(t, a) => {
                ast::Instruction::Shl(t, a.map_shift(visitor, &ast::Type::Scalar(t))?)
            }
            ast::Instruction::Shr(t, a) => {
                ast::Instruction::Shr(t, a.map_shift(visitor, &ast::Type::Scalar(t.into()))?)
            }
            ast::Instruction::St(d, a) => {
                let new_args = a.map(visitor, &d)?;
                ast::Instruction::St(d, new_args)
            }
            ast::Instruction::Bra(d, a) => ast::Instruction::Bra(d, a.map(visitor, false, None)?),
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
            ast::Instruction::Fma(d, a) => {
                let inst_type = ast::Type::Scalar(d.typ);
                ast::Instruction::Fma(d, a.map(visitor, &inst_type, false)?)
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
            ast::Instruction::Prmt { control, arg } => ast::Instruction::Prmt {
                control,
                arg: arg.map_prmt(visitor)?,
            },
            ast::Instruction::Activemask { arg } => ast::Instruction::Activemask {
                arg: arg.map(
                    visitor,
                    true,
                    Some((
                        &ast::Type::Scalar(ast::ScalarType::B32),
                        ast::StateSpace::Reg,
                    )),
                )?,
            },
            ast::Instruction::Membar { level } => ast::Instruction::Membar { level },
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
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            Some((&self.to_type, self.to_space)),
        )?;
        let new_src = visitor.id(
            ArgumentDescriptor {
                op: self.src,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            Some((&self.from_type, self.from_space)),
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
        Option<(&ast::Type, ast::StateSpace)>,
    ) -> Result<spirv::Word, TranslateError>,
{
    fn id(
        &mut self,
        desc: ArgumentDescriptor<spirv::Word>,
        t: Option<(&ast::Type, ast::StateSpace)>,
    ) -> Result<spirv::Word, TranslateError> {
        self(desc, t)
    }

    fn operand(
        &mut self,
        desc: ArgumentDescriptor<TypedOperand>,
        typ: &ast::Type,
        state_space: ast::StateSpace,
    ) -> Result<TypedOperand, TranslateError> {
        Ok(match desc.op {
            TypedOperand::Reg(id) => {
                TypedOperand::Reg(self(desc.new_op(id), Some((typ, state_space)))?)
            }
            TypedOperand::Imm(imm) => TypedOperand::Imm(imm),
            TypedOperand::RegOffset(id, imm) => {
                TypedOperand::RegOffset(self(desc.new_op(id), Some((typ, state_space)))?, imm)
            }
            TypedOperand::VecMember(reg, index) => {
                let scalar_type = match typ {
                    ast::Type::Scalar(scalar_t) => *scalar_t,
                    _ => return Err(error_unreachable()),
                };
                let vec_type = ast::Type::Vector(scalar_type, index + 1);
                TypedOperand::VecMember(
                    self(desc.new_op(reg), Some((&vec_type, state_space)))?,
                    index,
                )
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
                if (kind != ast::ScalarKind::Signed
                    && kind != ast::ScalarKind::Unsigned
                    && kind != ast::ScalarKind::Bit)
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
                state_space: ast::StateSpace::Reg,
                scalar_kind: scalar.kind(),
                width: scalar.size_of(),
                components: Vec::new(),
            },
            ast::Type::Vector(scalar, components) => TypeParts {
                kind: TypeKind::Vector,
                state_space: ast::StateSpace::Reg,
                scalar_kind: scalar.kind(),
                width: scalar.size_of(),
                components: vec![*components as u32],
            },
            ast::Type::Array(scalar, components) => TypeParts {
                kind: TypeKind::Array,
                state_space: ast::StateSpace::Reg,
                scalar_kind: scalar.kind(),
                width: scalar.size_of(),
                components: components.clone(),
            },
            ast::Type::Pointer(scalar, space) => TypeParts {
                kind: TypeKind::Pointer,
                state_space: *space,
                scalar_kind: scalar.kind(),
                width: scalar.size_of(),
                components: Vec::new(),
            },
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
            TypeKind::Pointer => ast::Type::Pointer(
                ast::ScalarType::from_parts(t.width, t.scalar_kind),
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
            ast::Type::Pointer(..) => mem::size_of::<usize>(),
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
struct TypeParts {
    kind: TypeKind,
    scalar_kind: ast::ScalarKind,
    width: u8,
    state_space: ast::StateSpace,
    components: Vec<u32>,
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum TypeKind {
    Scalar,
    Vector,
    Array,
    Pointer,
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
            ast::Instruction::Prmt { .. } => None,
            ast::Instruction::Activemask { .. } => None,
            ast::Instruction::Membar { .. } => None,
            ast::Instruction::Sub(ast::ArithDetails::Float(float_control), _)
            | ast::Instruction::Add(ast::ArithDetails::Float(float_control), _)
            | ast::Instruction::Mul(ast::MulDetails::Float(float_control), _)
            | ast::Instruction::Mad(ast::MulDetails::Float(float_control), _) => float_control
                .flush_to_zero
                .map(|ftz| (ftz, ast::ScalarType::from(float_control.typ).size_of())),
            ast::Instruction::Fma(d, _) => d.flush_to_zero.map(|ftz| (ftz, d.typ.size_of())),
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

impl<From: ArgParamsEx<Id = spirv::Word>, To: ArgParamsEx<Id = spirv::Word>> Visitable<From, To>
    for BrachCondition
{
    fn visit(
        self,
        visitor: &mut impl ArgumentMapVisitor<From, To>,
    ) -> Result<Statement<ast::Instruction<To>, To>, TranslateError> {
        let predicate = visitor.id(
            ArgumentDescriptor {
                op: self.predicate,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            Some((
                &ast::Type::Scalar(ast::ScalarType::Pred),
                ast::StateSpace::Reg,
            )),
        )?;
        let if_true = self.if_true;
        let if_false = self.if_false;
        Ok(Statement::Conditional(BrachCondition {
            predicate,
            if_true,
            if_false,
        }))
    }
}

#[derive(Clone)]
struct ImplicitConversion {
    src: spirv::Word,
    dst: spirv::Word,
    from_type: ast::Type,
    to_type: ast::Type,
    from_space: ast::StateSpace,
    to_space: ast::StateSpace,
    kind: ConversionKind,
}

#[derive(PartialEq, Clone)]
enum ConversionKind {
    Default,
    // zero-extend/chop/bitcast depending on types
    SignExtend,
    BitToPtr,
    PtrToPtr,
    AddressOf,
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
        is_dst: bool,
        t: Option<(&ast::Type, ast::StateSpace)>,
    ) -> Result<ast::Arg1<U>, TranslateError> {
        let new_src = visitor.id(
            ArgumentDescriptor {
                op: self.src,
                is_dst,
                is_memory_access: false,
                non_default_implicit_conversion: None,
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
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::U32),
            ast::StateSpace::Reg,
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
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            t,
            ast::StateSpace::Reg,
        )?;
        let new_src = visitor.operand(
            ArgumentDescriptor {
                op: self.src,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            t,
            ast::StateSpace::Reg,
        )?;
        Ok(ast::Arg2 {
            dst: new_dst,
            src: new_src,
        })
    }

    fn map_cvt<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        dst_t: ast::ScalarType,
        src_t: ast::ScalarType,
        is_int_to_int: bool,
    ) -> Result<ast::Arg2<U>, TranslateError> {
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: if is_int_to_int {
                    Some(should_convert_relaxed_dst_wrapper)
                } else {
                    None
                },
            },
            &ast::Type::Scalar(dst_t),
            ast::StateSpace::Reg,
        )?;
        let src = visitor.operand(
            ArgumentDescriptor {
                op: self.src,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: if is_int_to_int {
                    Some(should_convert_relaxed_src_wrapper)
                } else {
                    None
                },
            },
            &ast::Type::Scalar(src_t),
            ast::StateSpace::Reg,
        )?;
        Ok(ast::Arg2 { dst, src })
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
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            dst_t,
            ast::StateSpace::Reg,
        )?;
        let src = visitor.operand(
            ArgumentDescriptor {
                op: self.src,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            src_t,
            ast::StateSpace::Reg,
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
                is_memory_access: false,
                non_default_implicit_conversion: Some(should_convert_relaxed_dst_wrapper),
            },
            &ast::Type::from(details.typ.clone()),
            ast::StateSpace::Reg,
        )?;
        let src = visitor.operand(
            ArgumentDescriptor {
                op: self.src,
                is_dst: false,
                is_memory_access: true,
                non_default_implicit_conversion: None,
            },
            &details.typ,
            details.state_space,
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
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: true,
                non_default_implicit_conversion: None,
            },
            &details.typ,
            details.state_space,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: Some(should_convert_relaxed_src_wrapper),
            },
            &details.typ.clone().into(),
            ast::StateSpace::Reg,
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
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &details.typ.clone().into(),
            ast::StateSpace::Reg,
        )?;
        let src = visitor.operand(
            ArgumentDescriptor {
                op: self.src,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: Some(implicit_conversion_mov),
            },
            &details.typ.clone().into(),
            ast::StateSpace::Reg,
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
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            wide_type.as_ref().unwrap_or(typ),
            ast::StateSpace::Reg,
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            typ,
            ast::StateSpace::Reg,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            typ,
            ast::StateSpace::Reg,
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
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            t,
            ast::StateSpace::Reg,
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            t,
            ast::StateSpace::Reg,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::U32),
            ast::StateSpace::Reg,
        )?;
        Ok(ast::Arg3 { dst, src1, src2 })
    }

    fn map_atom<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
        t: ast::ScalarType,
        state_space: ast::StateSpace,
    ) -> Result<ast::Arg3<U>, TranslateError> {
        let scalar_type = ast::ScalarType::from(t);
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(scalar_type),
            ast::StateSpace::Reg,
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: true,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(scalar_type),
            state_space,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(scalar_type),
            ast::StateSpace::Reg,
        )?;
        Ok(ast::Arg3 { dst, src1, src2 })
    }

    fn map_prmt<U: ArgParamsEx, V: ArgumentMapVisitor<T, U>>(
        self,
        visitor: &mut V,
    ) -> Result<ast::Arg3<U>, TranslateError> {
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::B32),
            ast::StateSpace::Reg,
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::B32),
            ast::StateSpace::Reg,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::B32),
            ast::StateSpace::Reg,
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
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            wide_type.as_ref().unwrap_or(t),
            ast::StateSpace::Reg,
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            t,
            ast::StateSpace::Reg,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            t,
            ast::StateSpace::Reg,
        )?;
        let src3 = visitor.operand(
            ArgumentDescriptor {
                op: self.src3,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            t,
            ast::StateSpace::Reg,
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
        t: ast::ScalarType,
    ) -> Result<ast::Arg4<U>, TranslateError> {
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(t.into()),
            ast::StateSpace::Reg,
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(t.into()),
            ast::StateSpace::Reg,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(t.into()),
            ast::StateSpace::Reg,
        )?;
        let src3 = visitor.operand(
            ArgumentDescriptor {
                op: self.src3,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::Pred),
            ast::StateSpace::Reg,
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
        t: ast::ScalarType,
        state_space: ast::StateSpace,
    ) -> Result<ast::Arg4<U>, TranslateError> {
        let scalar_type = ast::ScalarType::from(t);
        let dst = visitor.operand(
            ArgumentDescriptor {
                op: self.dst,
                is_dst: true,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(scalar_type),
            ast::StateSpace::Reg,
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: true,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(scalar_type),
            state_space,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(scalar_type),
            ast::StateSpace::Reg,
        )?;
        let src3 = visitor.operand(
            ArgumentDescriptor {
                op: self.src3,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(scalar_type),
            ast::StateSpace::Reg,
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
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            typ,
            ast::StateSpace::Reg,
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            typ,
            ast::StateSpace::Reg,
        )?;
        let u32_type = ast::Type::Scalar(ast::ScalarType::U32);
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &u32_type,
            ast::StateSpace::Reg,
        )?;
        let src3 = visitor.operand(
            ArgumentDescriptor {
                op: self.src3,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &u32_type,
            ast::StateSpace::Reg,
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
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            Some((
                &ast::Type::Scalar(ast::ScalarType::Pred),
                ast::StateSpace::Reg,
            )),
        )?;
        let dst2 = self
            .dst2
            .map(|dst2| {
                visitor.id(
                    ArgumentDescriptor {
                        op: dst2,
                        is_dst: true,
                        is_memory_access: false,
                        non_default_implicit_conversion: None,
                    },
                    Some((
                        &ast::Type::Scalar(ast::ScalarType::Pred),
                        ast::StateSpace::Reg,
                    )),
                )
            })
            .transpose()?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            t,
            ast::StateSpace::Reg,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            t,
            ast::StateSpace::Reg,
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
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            base_type,
            ast::StateSpace::Reg,
        )?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            base_type,
            ast::StateSpace::Reg,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            base_type,
            ast::StateSpace::Reg,
        )?;
        let src3 = visitor.operand(
            ArgumentDescriptor {
                op: self.src3,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::U32),
            ast::StateSpace::Reg,
        )?;
        let src4 = visitor.operand(
            ArgumentDescriptor {
                op: self.src4,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::U32),
            ast::StateSpace::Reg,
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
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            Some((
                &ast::Type::Scalar(ast::ScalarType::Pred),
                ast::StateSpace::Reg,
            )),
        )?;
        let dst2 = self
            .dst2
            .map(|dst2| {
                visitor.id(
                    ArgumentDescriptor {
                        op: dst2,
                        is_dst: true,
                        is_memory_access: false,
                        non_default_implicit_conversion: None,
                    },
                    Some((
                        &ast::Type::Scalar(ast::ScalarType::Pred),
                        ast::StateSpace::Reg,
                    )),
                )
            })
            .transpose()?;
        let src1 = visitor.operand(
            ArgumentDescriptor {
                op: self.src1,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            t,
            ast::StateSpace::Reg,
        )?;
        let src2 = visitor.operand(
            ArgumentDescriptor {
                op: self.src2,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            t,
            ast::StateSpace::Reg,
        )?;
        let src3 = visitor.operand(
            ArgumentDescriptor {
                op: self.src3,
                is_dst: false,
                is_memory_access: false,
                non_default_implicit_conversion: None,
            },
            &ast::Type::Scalar(ast::ScalarType::Pred),
            ast::StateSpace::Reg,
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

impl ast::ScalarType {
    fn from_parts(width: u8, kind: ast::ScalarKind) -> Self {
        match kind {
            ast::ScalarKind::Float => match width {
                2 => ast::ScalarType::F16,
                4 => ast::ScalarType::F32,
                8 => ast::ScalarType::F64,
                _ => unreachable!(),
            },
            ast::ScalarKind::Bit => match width {
                1 => ast::ScalarType::B8,
                2 => ast::ScalarType::B16,
                4 => ast::ScalarType::B32,
                8 => ast::ScalarType::B64,
                _ => unreachable!(),
            },
            ast::ScalarKind::Signed => match width {
                1 => ast::ScalarType::S8,
                2 => ast::ScalarType::S16,
                4 => ast::ScalarType::S32,
                8 => ast::ScalarType::S64,
                _ => unreachable!(),
            },
            ast::ScalarKind::Unsigned => match width {
                1 => ast::ScalarType::U8,
                2 => ast::ScalarType::U16,
                4 => ast::ScalarType::U32,
                8 => ast::ScalarType::U64,
                _ => unreachable!(),
            },
            ast::ScalarKind::Float2 => match width {
                4 => ast::ScalarType::F16x2,
                _ => unreachable!(),
            },
            ast::ScalarKind::Pred => ast::ScalarType::Pred,
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

impl ast::StateSpace {
    fn to_spirv(self) -> spirv::StorageClass {
        match self {
            ast::StateSpace::Const => spirv::StorageClass::UniformConstant,
            ast::StateSpace::Generic => spirv::StorageClass::Generic,
            ast::StateSpace::Global => spirv::StorageClass::CrossWorkgroup,
            ast::StateSpace::Local => spirv::StorageClass::Function,
            ast::StateSpace::Shared => spirv::StorageClass::Workgroup,
            ast::StateSpace::Param => spirv::StorageClass::Function,
            ast::StateSpace::Reg => spirv::StorageClass::Function,
            ast::StateSpace::Sreg => spirv::StorageClass::Input,
        }
    }

    fn is_compatible(self, other: ast::StateSpace) -> bool {
        self == other
            || self == ast::StateSpace::Reg && other == ast::StateSpace::Sreg
            || self == ast::StateSpace::Sreg && other == ast::StateSpace::Reg
    }

    fn coerces_to_generic(self) -> bool {
        match self {
            ast::StateSpace::Global
            | ast::StateSpace::Const
            | ast::StateSpace::Local
            | ast::StateSpace::Shared => true,
            ast::StateSpace::Reg
            | ast::StateSpace::Param
            | ast::StateSpace::Generic
            | ast::StateSpace::Sreg => false,
        }
    }

    fn is_addressable(self) -> bool {
        match self {
            ast::StateSpace::Const
            | ast::StateSpace::Generic
            | ast::StateSpace::Global
            | ast::StateSpace::Local
            | ast::StateSpace::Shared => true,
            ast::StateSpace::Param | ast::StateSpace::Reg | ast::StateSpace::Sreg => false,
        }
    }
}

impl<T> ast::Operand<T> {
    fn underlying_register(&self) -> Option<&T> {
        match self {
            ast::Operand::Reg(r)
            | ast::Operand::RegOffset(r, _)
            | ast::Operand::VecMember(r, _) => Some(r),
            ast::Operand::Imm(_) | ast::Operand::VecPack(..) => None,
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

fn default_implicit_conversion(
    (operand_space, operand_type): (ast::StateSpace, &ast::Type),
    (instruction_space, instruction_type): (ast::StateSpace, &ast::Type),
) -> Result<Option<ConversionKind>, TranslateError> {
    if !instruction_space.is_compatible(operand_space) {
        default_implicit_conversion_space(
            (operand_space, operand_type),
            (instruction_space, instruction_type),
        )
    } else if instruction_type != operand_type {
        default_implicit_conversion_type(instruction_space, operand_type, instruction_type)
    } else {
        Ok(None)
    }
}

// Space is different
fn default_implicit_conversion_space(
    (operand_space, operand_type): (ast::StateSpace, &ast::Type),
    (instruction_space, instruction_type): (ast::StateSpace, &ast::Type),
) -> Result<Option<ConversionKind>, TranslateError> {
    if (instruction_space == ast::StateSpace::Generic && operand_space.coerces_to_generic())
        || (operand_space == ast::StateSpace::Generic && instruction_space.coerces_to_generic())
    {
        Ok(Some(ConversionKind::PtrToPtr))
    } else if operand_space.is_compatible(ast::StateSpace::Reg) {
        match operand_type {
            ast::Type::Pointer(operand_ptr_type, operand_ptr_space)
                if *operand_ptr_space == instruction_space =>
            {
                if instruction_type != &ast::Type::Scalar(*operand_ptr_type) {
                    Ok(Some(ConversionKind::PtrToPtr))
                } else {
                    Ok(None)
                }
            }
            // TODO: 32 bit
            ast::Type::Scalar(ast::ScalarType::B64)
            | ast::Type::Scalar(ast::ScalarType::U64)
            | ast::Type::Scalar(ast::ScalarType::S64) => match instruction_space {
                ast::StateSpace::Global
                | ast::StateSpace::Generic
                | ast::StateSpace::Const
                | ast::StateSpace::Local
                | ast::StateSpace::Shared => Ok(Some(ConversionKind::BitToPtr)),
                _ => Err(TranslateError::MismatchedType),
            },
            ast::Type::Scalar(ast::ScalarType::B32)
            | ast::Type::Scalar(ast::ScalarType::U32)
            | ast::Type::Scalar(ast::ScalarType::S32) => match instruction_space {
                ast::StateSpace::Const | ast::StateSpace::Local | ast::StateSpace::Shared => {
                    Ok(Some(ConversionKind::BitToPtr))
                }
                _ => Err(TranslateError::MismatchedType),
            },
            _ => Err(TranslateError::MismatchedType),
        }
    } else if instruction_space.is_compatible(ast::StateSpace::Reg) {
        match instruction_type {
            ast::Type::Pointer(instruction_ptr_type, instruction_ptr_space)
                if operand_space == *instruction_ptr_space =>
            {
                if operand_type != &ast::Type::Scalar(*instruction_ptr_type) {
                    Ok(Some(ConversionKind::PtrToPtr))
                } else {
                    Ok(None)
                }
            }
            _ => Err(TranslateError::MismatchedType),
        }
    } else {
        Err(TranslateError::MismatchedType)
    }
}

// Space is same, but type is different
fn default_implicit_conversion_type(
    space: ast::StateSpace,
    operand_type: &ast::Type,
    instruction_type: &ast::Type,
) -> Result<Option<ConversionKind>, TranslateError> {
    if space.is_compatible(ast::StateSpace::Reg) {
        if should_bitcast(instruction_type, operand_type) {
            Ok(Some(ConversionKind::Default))
        } else {
            Err(TranslateError::MismatchedType)
        }
    } else {
        Ok(Some(ConversionKind::PtrToPtr))
    }
}

fn should_bitcast(instr: &ast::Type, operand: &ast::Type) -> bool {
    match (instr, operand) {
        (ast::Type::Scalar(inst), ast::Type::Scalar(operand)) => {
            if inst.size_of() != operand.size_of() {
                return false;
            }
            match inst.kind() {
                ast::ScalarKind::Bit => operand.kind() != ast::ScalarKind::Bit,
                ast::ScalarKind::Float => operand.kind() == ast::ScalarKind::Bit,
                ast::ScalarKind::Signed => {
                    operand.kind() == ast::ScalarKind::Bit
                        || operand.kind() == ast::ScalarKind::Unsigned
                }
                ast::ScalarKind::Unsigned => {
                    operand.kind() == ast::ScalarKind::Bit
                        || operand.kind() == ast::ScalarKind::Signed
                }
                ast::ScalarKind::Float2 => false,
                ast::ScalarKind::Pred => false,
            }
        }
        (ast::Type::Vector(inst, _), ast::Type::Vector(operand, _))
        | (ast::Type::Array(inst, _), ast::Type::Array(operand, _)) => {
            should_bitcast(&ast::Type::Scalar(*inst), &ast::Type::Scalar(*operand))
        }
        _ => false,
    }
}

fn implicit_conversion_mov(
    (operand_space, operand_type): (ast::StateSpace, &ast::Type),
    (instruction_space, instruction_type): (ast::StateSpace, &ast::Type),
) -> Result<Option<ConversionKind>, TranslateError> {
    // instruction_space is always reg
    if operand_space.is_compatible(ast::StateSpace::Reg) {
        if let (ast::Type::Vector(vec_underlying_type, vec_len), ast::Type::Scalar(scalar)) =
            (operand_type, instruction_type)
        {
            if scalar.kind() == ast::ScalarKind::Bit
                && scalar.size_of() == (vec_underlying_type.size_of() * vec_len)
            {
                return Ok(Some(ConversionKind::Default));
            }
        }
    // TODO: verify .params addressability:
    // * kernel arg
    // * func arg
    // * variable
    } else if operand_space.is_addressable() {
        return Ok(Some(ConversionKind::AddressOf));
    }
    default_implicit_conversion(
        (operand_space, operand_type),
        (instruction_space, instruction_type),
    )
}

fn should_convert_relaxed_src_wrapper(
    (operand_space, operand_type): (ast::StateSpace, &ast::Type),
    (instruction_space, instruction_type): (ast::StateSpace, &ast::Type),
) -> Result<Option<ConversionKind>, TranslateError> {
    if !operand_space.is_compatible(instruction_space) {
        return Err(TranslateError::MismatchedType);
    }
    if operand_type == instruction_type {
        return Ok(None);
    }
    match should_convert_relaxed_src(operand_type, instruction_type) {
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
            ast::ScalarKind::Bit => {
                if instr_type.size_of() <= src_type.size_of() {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
            ast::ScalarKind::Signed | ast::ScalarKind::Unsigned => {
                if instr_type.size_of() <= src_type.size_of()
                    && src_type.kind() != ast::ScalarKind::Float
                {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
            ast::ScalarKind::Float => {
                if instr_type.size_of() <= src_type.size_of()
                    && src_type.kind() == ast::ScalarKind::Bit
                {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
            ast::ScalarKind::Float2 => todo!(),
            ast::ScalarKind::Pred => None,
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
    (operand_space, operand_type): (ast::StateSpace, &ast::Type),
    (instruction_space, instruction_type): (ast::StateSpace, &ast::Type),
) -> Result<Option<ConversionKind>, TranslateError> {
    if !operand_space.is_compatible(instruction_space) {
        return Err(TranslateError::MismatchedType);
    }
    if operand_type == instruction_type {
        return Ok(None);
    }
    match should_convert_relaxed_dst(operand_type, instruction_type) {
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
            ast::ScalarKind::Bit => {
                if instr_type.size_of() <= dst_type.size_of() {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
            ast::ScalarKind::Signed => {
                if dst_type.kind() != ast::ScalarKind::Float {
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
            ast::ScalarKind::Unsigned => {
                if instr_type.size_of() <= dst_type.size_of()
                    && dst_type.kind() != ast::ScalarKind::Float
                {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
            ast::ScalarKind::Float => {
                if instr_type.size_of() <= dst_type.size_of()
                    && dst_type.kind() == ast::ScalarKind::Bit
                {
                    Some(ConversionKind::Default)
                } else {
                    None
                }
            }
            ast::ScalarKind::Float2 => todo!(),
            ast::ScalarKind::Pred => None,
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

impl<'a> ast::MethodDeclaration<'a, &'a str> {
    fn name(&self) -> &'a str {
        match self.name {
            ast::MethodName::Kernel(name) => name,
            ast::MethodName::Func(name) => name,
        }
    }
}

impl<'a> ast::MethodDeclaration<'a, spirv::Word> {
    fn effective_input_arguments(&self) -> impl Iterator<Item = (spirv::Word, SpirvType)> + '_ {
        let is_kernel = self.name.is_kernel();
        self.input_arguments.iter().map(move |arg| {
            if !is_kernel && arg.state_space != ast::StateSpace::Reg {
                let spirv_type =
                    SpirvType::pointer_to(arg.v_type.clone(), arg.state_space.to_spirv());
                (arg.name, spirv_type)
            } else {
                (arg.name, SpirvType::new(arg.v_type.clone()))
            }
        })
    }
}

impl<'input, ID> ast::MethodName<'input, ID> {
    fn is_kernel(&self) -> bool {
        match self {
            ast::MethodName::Kernel(..) => true,
            ast::MethodName::Func(..) => false,
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
