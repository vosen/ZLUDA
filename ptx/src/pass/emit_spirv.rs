use super::*;
use half::f16;
use ptx_parser as ast;
use rspirv::{binary::Assemble, dr};
use std::{
    collections::{HashMap, HashSet},
    ffi::CString,
    mem,
};

pub(super) fn run<'input>(
    mut builder: dr::Builder,
    id_defs: &GlobalStringIdResolver<'input>,
    call_map: MethodsCallMap<'input>,
    denorm_information: HashMap<
        ptx_parser::MethodName<SpirvWord>,
        HashMap<u8, (spirv::FPDenormMode, isize)>,
    >,
    directives: Vec<Directive<'input>>,
) -> Result<(dr::Module, HashMap<String, KernelInfo>, CString), TranslateError> {
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
    Ok((builder.module(), kernel_info, build_options))
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

struct TypeWordMap {
    void: spirv::Word,
    complex: HashMap<SpirvType, SpirvWord>,
    constants: HashMap<(SpirvType, u64), SpirvWord>,
}

impl TypeWordMap {
    fn new(b: &mut dr::Builder) -> TypeWordMap {
        let void = b.type_void(None);
        TypeWordMap {
            void: void,
            complex: HashMap::<SpirvType, SpirvWord>::new(),
            constants: HashMap::new(),
        }
    }

    fn void(&self) -> spirv::Word {
        self.void
    }

    fn get_or_add_scalar(&mut self, b: &mut dr::Builder, t: ast::ScalarType) -> SpirvWord {
        let key: SpirvScalarKey = t.into();
        self.get_or_add_spirv_scalar(b, key)
    }

    fn get_or_add_spirv_scalar(&mut self, b: &mut dr::Builder, key: SpirvScalarKey) -> SpirvWord {
        *self.complex.entry(SpirvType::Base(key)).or_insert_with(|| {
            SpirvWord(match key {
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
        })
    }

    fn get_or_add(&mut self, b: &mut dr::Builder, t: SpirvType) -> SpirvWord {
        match t {
            SpirvType::Base(key) => self.get_or_add_spirv_scalar(b, key),
            SpirvType::Pointer(ref typ, storage) => {
                let base = self.get_or_add(b, *typ.clone());
                *self
                    .complex
                    .entry(t)
                    .or_insert_with(|| SpirvWord(b.type_pointer(None, storage, base.0)))
            }
            SpirvType::Vector(typ, len) => {
                let base = self.get_or_add_spirv_scalar(b, typ);
                *self
                    .complex
                    .entry(t)
                    .or_insert_with(|| SpirvWord(b.type_vector(None, base.0, len as u32)))
            }
            SpirvType::Array(typ, array_dimensions) => {
                let (base_type, length) = match &*array_dimensions {
                    &[] => {
                        return self.get_or_add(b, SpirvType::Base(typ));
                    }
                    &[len] => {
                        let u32_type = self.get_or_add_scalar(b, ast::ScalarType::U32);
                        let base = self.get_or_add_spirv_scalar(b, typ);
                        let len_const = b.constant_u32(u32_type.0, None, len);
                        (base, len_const)
                    }
                    array_dimensions => {
                        let u32_type = self.get_or_add_scalar(b, ast::ScalarType::U32);
                        let base = self
                            .get_or_add(b, SpirvType::Array(typ, array_dimensions[1..].to_vec()));
                        let len_const = b.constant_u32(u32_type.0, None, array_dimensions[0]);
                        (base, len_const)
                    }
                };
                *self
                    .complex
                    .entry(SpirvType::Array(typ, array_dimensions))
                    .or_insert_with(|| SpirvWord(b.type_array(None, base_type.0, length)))
            }
            SpirvType::Func(ref out_params, ref in_params) => {
                let out_t = match out_params {
                    Some(p) => self.get_or_add(b, *p.clone()),
                    None => SpirvWord(self.void()),
                };
                let in_t = in_params
                    .iter()
                    .map(|t| self.get_or_add(b, t.clone()).0)
                    .collect::<Vec<_>>();
                *self
                    .complex
                    .entry(t)
                    .or_insert_with(|| SpirvWord(b.type_function(None, out_t.0, in_t)))
            }
            SpirvType::Struct(ref underlying) => {
                let underlying_ids = underlying
                    .iter()
                    .map(|t| self.get_or_add_spirv_scalar(b, *t).0)
                    .collect::<Vec<_>>();
                *self
                    .complex
                    .entry(t)
                    .or_insert_with(|| SpirvWord(b.type_struct(None, underlying_ids)))
            }
        }
    }

    fn get_or_add_fn(
        &mut self,
        b: &mut dr::Builder,
        in_params: impl Iterator<Item = SpirvType>,
        mut out_params: impl ExactSizeIterator<Item = SpirvType>,
    ) -> (SpirvWord, SpirvWord) {
        let (out_args, out_spirv_type) = if out_params.len() == 0 {
            (None, SpirvWord(self.void()))
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
    ) -> Result<SpirvWord, TranslateError> {
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
                ast::ScalarType::S16x2
                | ast::ScalarType::U16x2
                | ast::ScalarType::BF16
                | ast::ScalarType::BF16x2
                | ast::ScalarType::B128 => todo!(),
            },
            ast::Type::Vector(typ, len) => {
                let result_type =
                    self.get_or_add(b, SpirvType::Vector(SpirvScalarKey::from(*typ), *len));
                let size_of_t = typ.size_of();
                let components = (0..*len)
                    .map(|x| {
                        Ok::<_, TranslateError>(
                            self.get_or_add_constant(
                                b,
                                &ast::Type::Scalar(*typ),
                                &init[((size_of_t as usize) * (x as usize))..],
                            )?
                            .0,
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                SpirvWord(b.constant_composite(result_type.0, None, components.into_iter()))
            }
            ast::Type::Array(typ, dims) => match dims.as_slice() {
                [] => return Err(error_unreachable()),
                [dim] => {
                    let result_type = self
                        .get_or_add(b, SpirvType::Array(SpirvScalarKey::from(*typ), vec![*dim]));
                    let size_of_t = typ.size_of();
                    let components = (0..*dim)
                        .map(|x| {
                            Ok::<_, TranslateError>(
                                self.get_or_add_constant(
                                    b,
                                    &ast::Type::Scalar(*typ),
                                    &init[((size_of_t as usize) * (x as usize))..],
                                )?
                                .0,
                            )
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    SpirvWord(b.constant_composite(result_type.0, None, components.into_iter()))
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
                            Ok::<_, TranslateError>(
                                self.get_or_add_constant(
                                    b,
                                    &ast::Type::Array(*typ, rest.to_vec()),
                                    &init[((size_of_t as usize) * (x as usize))..],
                                )?
                                .0,
                            )
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    SpirvWord(b.constant_composite(result_type.0, None, components.into_iter()))
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
    ) -> SpirvWord {
        let value = unsafe { *(init.as_ptr() as *const T) };
        let value_64 = cast(value);
        let ht_key = (SpirvType::Base(SpirvScalarKey::from(key)), value_64);
        match self.constants.get(&ht_key) {
            Some(value) => *value,
            None => {
                let spirv_type = self.get_or_add_scalar(b, key);
                let result = SpirvWord(f(b, spirv_type.0, value));
                self.constants.insert(ht_key, result);
                result
            }
        }
    }
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
                space_to_spirv(space),
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
            ast::ScalarType::S16x2
            | ast::ScalarType::U16x2
            | ast::ScalarType::BF16
            | ast::ScalarType::BF16x2
            | ast::ScalarType::B128 => todo!(),
        }
    }
}

fn space_to_spirv(this: ast::StateSpace) -> spirv::StorageClass {
    match this {
        ast::StateSpace::Const => spirv::StorageClass::UniformConstant,
        ast::StateSpace::Generic => spirv::StorageClass::Generic,
        ast::StateSpace::Global => spirv::StorageClass::CrossWorkgroup,
        ast::StateSpace::Local => spirv::StorageClass::Function,
        ast::StateSpace::Shared => spirv::StorageClass::Workgroup,
        ast::StateSpace::Param => spirv::StorageClass::Function,
        ast::StateSpace::Reg => spirv::StorageClass::Function,
        ast::StateSpace::Sreg => spirv::StorageClass::Input,
        ast::StateSpace::ParamEntry
        | ast::StateSpace::ParamFunc
        | ast::StateSpace::SharedCluster
        | ast::StateSpace::SharedCta => todo!(),
    }
}

// TODO: remove this once we have pef-function support for denorms
fn emit_denorm_build_string<'input>(
    call_map: &MethodsCallMap,
    denorm_information: &HashMap<
        ast::MethodName<'input, SpirvWord>,
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

fn get_globals_use_map<'input>(
    directives: Vec<Directive<'input>>,
) -> (
    Vec<Directive<'input>>,
    HashMap<ast::MethodName<'input, SpirvWord>, HashSet<SpirvWord>>,
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
                        statement.visit_map(
                            &mut |symbol, _: Option<(&ast::Type, ast::StateSpace)>, _, _| {
                                if known_globals.contains(&symbol) {
                                    multi_hash_map_append(
                                        &mut symbol_uses_map,
                                        method_name,
                                        symbol,
                                    );
                                }
                                Ok::<_, TranslateError>(symbol)
                            },
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();
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

fn emit_directives<'input>(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    id_defs: &GlobalStringIdResolver<'input>,
    opencl_id: spirv::Word,
    should_flush_denorms: bool,
    call_map: &MethodsCallMap<'input>,
    globals_use_map: HashMap<ast::MethodName<'input, SpirvWord>, HashSet<SpirvWord>>,
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
                if matches!(func_decl.name, ast::MethodName::Kernel(_)) {
                    if should_flush_denorms {
                        builder.execution_mode(
                            fn_id.0,
                            spirv_headers::ExecutionMode::DenormFlushToZero,
                            [16],
                        );
                        builder.execution_mode(
                            fn_id.0,
                            spirv_headers::ExecutionMode::DenormFlushToZero,
                            [32],
                        );
                        builder.execution_mode(
                            fn_id.0,
                            spirv_headers::ExecutionMode::DenormFlushToZero,
                            [64],
                        );
                    }
                    // FP contraction happens when compiling source -> PTX and is illegal at this stage (unless you force it in cuModuleLoadDataEx)
                    builder.execution_mode(
                        fn_id.0,
                        spirv_headers::ExecutionMode::ContractionOff,
                        [],
                    );
                    for t in f.tuning.iter() {
                        match *t {
                            ast::TuningDirective::MaxNtid(nx, ny, nz) => {
                                builder.execution_mode(
                                    fn_id.0,
                                    spirv_headers::ExecutionMode::MaxWorkgroupSizeINTEL,
                                    [nx, ny, nz],
                                );
                            }
                            ast::TuningDirective::ReqNtid(nx, ny, nz) => {
                                builder.execution_mode(
                                    fn_id.0,
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

fn emit_variable<'input>(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    id_defs: &GlobalStringIdResolver<'input>,
    linking: ast::LinkingDirective,
    var: &ast::Variable<SpirvWord>,
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
        ast::StateSpace::ParamEntry
        | ast::StateSpace::ParamFunc
        | ast::StateSpace::SharedCluster
        | ast::StateSpace::SharedCta => todo!(),
    };
    let initalizer = if var.array_init.len() > 0 {
        Some(
            map.get_or_add_constant(
                builder,
                &ast::Type::from(var.v_type.clone()),
                &*var.array_init,
            )?
            .0,
        )
    } else if must_init {
        let type_id = map.get_or_add(builder, SpirvType::new(var.v_type.clone()));
        Some(builder.constant_null(type_id.0, None))
    } else {
        None
    };
    let ptr_type_id = map.get_or_add(builder, SpirvType::pointer_to(var.v_type.clone(), st_class));
    builder.variable(ptr_type_id.0, Some(var.name.0), st_class, initalizer);
    if let Some(align) = var.align {
        builder.decorate(
            var.name.0,
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

fn emit_function_header<'input>(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    defined_globals: &GlobalStringIdResolver<'input>,
    func_decl: &ast::MethodDeclaration<'input, SpirvWord>,
    call_map: &MethodsCallMap<'input>,
    globals_use_map: &HashMap<ast::MethodName<'input, SpirvWord>, HashSet<SpirvWord>>,
    kernel_info: &mut HashMap<String, KernelInfo>,
) -> Result<SpirvWord, TranslateError> {
    if let ast::MethodName::Kernel(name) = func_decl.name {
        let args_lens = func_decl
            .input_arguments
            .iter()
            .map(|param| {
                (
                    type_size_of(&param.v_type),
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
        effective_input_arguments(func_decl).map(|(_, typ)| typ),
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
                .map(|word| word.0)
                .collect::<Vec<spirv::Word>>();
            builder.entry_point(spirv::ExecutionModel::Kernel, fn_id.0, name, interface);
            fn_id
        }
        ast::MethodName::Func(name) => name,
    };
    builder.begin_function(
        ret_type.0,
        Some(fn_id.0),
        spirv::FunctionControl::NONE,
        func_type.0,
    )?;
    for (name, typ) in effective_input_arguments(func_decl) {
        let result_type = map.get_or_add(builder, typ);
        builder.function_parameter(Some(name.0), result_type.0)?;
    }
    Ok(fn_id)
}

pub fn type_size_of(this: &ast::Type) -> usize {
    match this {
        ast::Type::Scalar(typ) => typ.size_of() as usize,
        ast::Type::Vector(typ, len) => (typ.size_of() as usize) * (*len as usize),
        ast::Type::Array(typ, len) => len
            .iter()
            .fold(typ.size_of() as usize, |x, y| (x as usize) * (*y as usize)),
        ast::Type::Pointer(..) => mem::size_of::<usize>(),
    }
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
                    builder.branch(id.0)?;
                }
                builder.begin_block(Some(id.0))?;
            }
            _ => {
                if builder.selected_block().is_none() && builder.selected_function().is_some() {
                    builder.begin_block(None)?;
                }
            }
        }
        match s {
            Statement::Label(_) => (),
            Statement::Variable(var) => {
                emit_variable(builder, map, id_defs, ast::LinkingDirective::NONE, var)?;
            }
            Statement::Constant(cnst) => {
                let typ_id = map.get_or_add_scalar(builder, cnst.typ);
                match (cnst.typ, cnst.value) {
                    (ast::ScalarType::B8, ast::ImmediateValue::U64(value))
                    | (ast::ScalarType::U8, ast::ImmediateValue::U64(value)) => {
                        builder.constant_u32(typ_id.0, Some(cnst.dst.0), value as u8 as u32);
                    }
                    (ast::ScalarType::B16, ast::ImmediateValue::U64(value))
                    | (ast::ScalarType::U16, ast::ImmediateValue::U64(value)) => {
                        builder.constant_u32(typ_id.0, Some(cnst.dst.0), value as u16 as u32);
                    }
                    (ast::ScalarType::B32, ast::ImmediateValue::U64(value))
                    | (ast::ScalarType::U32, ast::ImmediateValue::U64(value)) => {
                        builder.constant_u32(typ_id.0, Some(cnst.dst.0), value as u32);
                    }
                    (ast::ScalarType::B64, ast::ImmediateValue::U64(value))
                    | (ast::ScalarType::U64, ast::ImmediateValue::U64(value)) => {
                        builder.constant_u64(typ_id.0, Some(cnst.dst.0), value);
                    }
                    (ast::ScalarType::S8, ast::ImmediateValue::U64(value)) => {
                        builder.constant_u32(typ_id.0, Some(cnst.dst.0), value as i8 as u32);
                    }
                    (ast::ScalarType::S16, ast::ImmediateValue::U64(value)) => {
                        builder.constant_u32(typ_id.0, Some(cnst.dst.0), value as i16 as u32);
                    }
                    (ast::ScalarType::S32, ast::ImmediateValue::U64(value)) => {
                        builder.constant_u32(typ_id.0, Some(cnst.dst.0), value as i32 as u32);
                    }
                    (ast::ScalarType::S64, ast::ImmediateValue::U64(value)) => {
                        builder.constant_u64(typ_id.0, Some(cnst.dst.0), value as i64 as u64);
                    }
                    (ast::ScalarType::B8, ast::ImmediateValue::S64(value))
                    | (ast::ScalarType::U8, ast::ImmediateValue::S64(value)) => {
                        builder.constant_u32(typ_id.0, Some(cnst.dst.0), value as u8 as u32);
                    }
                    (ast::ScalarType::B16, ast::ImmediateValue::S64(value))
                    | (ast::ScalarType::U16, ast::ImmediateValue::S64(value)) => {
                        builder.constant_u32(typ_id.0, Some(cnst.dst.0), value as u16 as u32);
                    }
                    (ast::ScalarType::B32, ast::ImmediateValue::S64(value))
                    | (ast::ScalarType::U32, ast::ImmediateValue::S64(value)) => {
                        builder.constant_u32(typ_id.0, Some(cnst.dst.0), value as u32);
                    }
                    (ast::ScalarType::B64, ast::ImmediateValue::S64(value))
                    | (ast::ScalarType::U64, ast::ImmediateValue::S64(value)) => {
                        builder.constant_u64(typ_id.0, Some(cnst.dst.0), value as u64);
                    }
                    (ast::ScalarType::S8, ast::ImmediateValue::S64(value)) => {
                        builder.constant_u32(typ_id.0, Some(cnst.dst.0), value as i8 as u32);
                    }
                    (ast::ScalarType::S16, ast::ImmediateValue::S64(value)) => {
                        builder.constant_u32(typ_id.0, Some(cnst.dst.0), value as i16 as u32);
                    }
                    (ast::ScalarType::S32, ast::ImmediateValue::S64(value)) => {
                        builder.constant_u32(typ_id.0, Some(cnst.dst.0), value as i32 as u32);
                    }
                    (ast::ScalarType::S64, ast::ImmediateValue::S64(value)) => {
                        builder.constant_u64(typ_id.0, Some(cnst.dst.0), value as u64);
                    }
                    (ast::ScalarType::F16, ast::ImmediateValue::F32(value)) => {
                        builder.constant_f32(
                            typ_id.0,
                            Some(cnst.dst.0),
                            f16::from_f32(value).to_f32(),
                        );
                    }
                    (ast::ScalarType::F32, ast::ImmediateValue::F32(value)) => {
                        builder.constant_f32(typ_id.0, Some(cnst.dst.0), value);
                    }
                    (ast::ScalarType::F64, ast::ImmediateValue::F32(value)) => {
                        builder.constant_f64(typ_id.0, Some(cnst.dst.0), value as f64);
                    }
                    (ast::ScalarType::F16, ast::ImmediateValue::F64(value)) => {
                        builder.constant_f32(
                            typ_id.0,
                            Some(cnst.dst.0),
                            f16::from_f64(value).to_f32(),
                        );
                    }
                    (ast::ScalarType::F32, ast::ImmediateValue::F64(value)) => {
                        builder.constant_f32(typ_id.0, Some(cnst.dst.0), value as f32);
                    }
                    (ast::ScalarType::F64, ast::ImmediateValue::F64(value)) => {
                        builder.constant_f64(typ_id.0, Some(cnst.dst.0), value);
                    }
                    (ast::ScalarType::Pred, ast::ImmediateValue::U64(value)) => {
                        let bool_type = map.get_or_add_scalar(builder, ast::ScalarType::Pred).0;
                        if value == 0 {
                            builder.constant_false(bool_type, Some(cnst.dst.0));
                        } else {
                            builder.constant_true(bool_type, Some(cnst.dst.0));
                        }
                    }
                    (ast::ScalarType::Pred, ast::ImmediateValue::S64(value)) => {
                        let bool_type = map.get_or_add_scalar(builder, ast::ScalarType::Pred).0;
                        if value == 0 {
                            builder.constant_false(bool_type, Some(cnst.dst.0));
                        } else {
                            builder.constant_true(bool_type, Some(cnst.dst.0));
                        }
                    }
                    _ => return Err(error_mismatched_type()),
                }
            }
            Statement::Conversion(cv) => emit_implicit_conversion(builder, map, cv)?,
            Statement::Conditional(bra) => {
                builder.branch_conditional(
                    bra.predicate.0,
                    bra.if_true.0,
                    bra.if_false.0,
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
                builder.copy_object(result_type.0, Some(dst.0), zero.0)?;
            }
            Statement::Instruction(inst) => match inst {
                ast::Instruction::PrmtSlow { .. } | ast::Instruction::Trap { .. } => todo!(),
                ast::Instruction::Call { data, arguments } => {
                    let (result_type, result_id) =
                        match (&*data.return_arguments, &*arguments.return_arguments) {
                            ([(type_, space)], [id]) => {
                                if *space != ast::StateSpace::Reg {
                                    return Err(error_unreachable());
                                }
                                (
                                    map.get_or_add(builder, SpirvType::new(type_.clone())).0,
                                    Some(id.0),
                                )
                            }
                            ([], []) => (map.void(), None),
                            _ => todo!(),
                        };
                    let arg_list = arguments
                        .input_arguments
                        .iter()
                        .map(|id| id.0)
                        .collect::<Vec<_>>();
                    builder.function_call(result_type, result_id, arguments.func.0, arg_list)?;
                }
                ast::Instruction::Abs { data, arguments } => {
                    emit_abs(builder, map, opencl, data, arguments)?
                }
                // SPIR-V does not support marking jumps as guaranteed-converged
                ast::Instruction::Bra { arguments, .. } => {
                    builder.branch(arguments.src.0)?;
                }
                ast::Instruction::Ld { data, arguments } => {
                    let mem_access = match data.qualifier {
                        ast::LdStQualifier::Weak => spirv::MemoryAccess::NONE,
                        // ld.volatile does not match Volatile OpLoad nor Relaxed OpAtomicLoad
                        ast::LdStQualifier::Volatile => spirv::MemoryAccess::VOLATILE,
                        _ => return Err(TranslateError::Todo),
                    };
                    let result_type =
                        map.get_or_add(builder, SpirvType::new(ast::Type::from(data.typ.clone())));
                    builder.load(
                        result_type.0,
                        Some(arguments.dst.0),
                        arguments.src.0,
                        Some(mem_access | spirv::MemoryAccess::ALIGNED),
                        [dr::Operand::LiteralInt32(
                            type_size_of(&ast::Type::from(data.typ.clone())) as u32,
                        )]
                        .iter()
                        .cloned(),
                    )?;
                }
                ast::Instruction::St { data, arguments } => {
                    let mem_access = match data.qualifier {
                        ast::LdStQualifier::Weak => spirv::MemoryAccess::NONE,
                        // st.volatile does not match Volatile OpStore nor Relaxed OpAtomicStore
                        ast::LdStQualifier::Volatile => spirv::MemoryAccess::VOLATILE,
                        _ => return Err(TranslateError::Todo),
                    };
                    builder.store(
                        arguments.src1.0,
                        arguments.src2.0,
                        Some(mem_access | spirv::MemoryAccess::ALIGNED),
                        [dr::Operand::LiteralInt32(
                            type_size_of(&ast::Type::from(data.typ.clone())) as u32,
                        )]
                        .iter()
                        .cloned(),
                    )?;
                }
                // SPIR-V does not support ret as guaranteed-converged
                ast::Instruction::Ret { .. } => builder.ret()?,
                ast::Instruction::Mov { data, arguments } => {
                    let result_type =
                        map.get_or_add(builder, SpirvType::new(ast::Type::from(data.typ.clone())));
                    builder.copy_object(result_type.0, Some(arguments.dst.0), arguments.src.0)?;
                }
                ast::Instruction::Mul { data, arguments } => match data {
                    ast::MulDetails::Integer { type_, control } => {
                        emit_mul_int(builder, map, opencl, *type_, *control, arguments)?
                    }
                    ast::MulDetails::Float(ref ctr) => {
                        emit_mul_float(builder, map, ctr, arguments)?
                    }
                },
                ast::Instruction::Add { data, arguments } => match data {
                    ast::ArithDetails::Integer(desc) => {
                        emit_add_int(builder, map, desc.type_.into(), desc.saturate, arguments)?
                    }
                    ast::ArithDetails::Float(desc) => {
                        emit_add_float(builder, map, desc, arguments)?
                    }
                },
                ast::Instruction::Setp { data, arguments } => {
                    if arguments.dst2.is_some() {
                        todo!()
                    }
                    emit_setp(builder, map, data, arguments)?;
                }
                ast::Instruction::Not { data, arguments } => {
                    let result_type = map.get_or_add(builder, SpirvType::from(*data));
                    let result_id = Some(arguments.dst.0);
                    let operand = arguments.src;
                    match data {
                        ast::ScalarType::Pred => {
                            logical_not(builder, result_type.0, result_id, operand.0)
                        }
                        _ => builder.not(result_type.0, result_id, operand.0),
                    }?;
                }
                ast::Instruction::Shl { data, arguments } => {
                    let full_type = ast::Type::Scalar(*data);
                    let size_of = type_size_of(&full_type);
                    let result_type = map.get_or_add(builder, SpirvType::new(full_type));
                    let offset_src = insert_shift_hack(builder, map, arguments.src2.0, size_of)?;
                    builder.shift_left_logical(
                        result_type.0,
                        Some(arguments.dst.0),
                        arguments.src1.0,
                        offset_src,
                    )?;
                }
                ast::Instruction::Shr { data, arguments } => {
                    let full_type = ast::ScalarType::from(data.type_);
                    let size_of = full_type.size_of();
                    let result_type = map.get_or_add_scalar(builder, full_type).0;
                    let offset_src =
                        insert_shift_hack(builder, map, arguments.src2.0, size_of as usize)?;
                    match data.kind {
                        ptx_parser::RightShiftKind::Arithmetic => {
                            builder.shift_right_arithmetic(
                                result_type,
                                Some(arguments.dst.0),
                                arguments.src1.0,
                                offset_src,
                            )?;
                        }
                        ptx_parser::RightShiftKind::Logical => {
                            builder.shift_right_logical(
                                result_type,
                                Some(arguments.dst.0),
                                arguments.src1.0,
                                offset_src,
                            )?;
                        }
                    }
                }
                ast::Instruction::Cvt { data, arguments } => {
                    emit_cvt(builder, map, opencl, data, arguments)?;
                }
                ast::Instruction::Cvta { data, arguments } => {
                    // This would be only meaningful if const/slm/global pointers
                    // had a different format than generic pointers, but they don't pretty much by ptx definition
                    // Honestly, I have no idea why this instruction exists and is emitted by the compiler
                    let result_type = map.get_or_add_scalar(builder, ast::ScalarType::B64);
                    builder.copy_object(result_type.0, Some(arguments.dst.0), arguments.src.0)?;
                }
                ast::Instruction::SetpBool { .. } => todo!(),
                ast::Instruction::Mad { data, arguments } => match data {
                    ast::MadDetails::Integer {
                        type_,
                        control,
                        saturate,
                    } => {
                        if *saturate {
                            todo!()
                        }
                        if type_.kind() == ast::ScalarKind::Signed {
                            emit_mad_sint(builder, map, opencl, *type_, *control, arguments)?
                        } else {
                            emit_mad_uint(builder, map, opencl, *type_, *control, arguments)?
                        }
                    }
                    ast::MadDetails::Float(desc) => {
                        emit_mad_float(builder, map, opencl, desc, arguments)?
                    }
                },
                ast::Instruction::Fma { data, arguments } => {
                    emit_fma_float(builder, map, opencl, data, arguments)?
                }
                ast::Instruction::Or { data, arguments } => {
                    let result_type = map.get_or_add_scalar(builder, *data).0;
                    if *data == ast::ScalarType::Pred {
                        builder.logical_or(
                            result_type,
                            Some(arguments.dst.0),
                            arguments.src1.0,
                            arguments.src2.0,
                        )?;
                    } else {
                        builder.bitwise_or(
                            result_type,
                            Some(arguments.dst.0),
                            arguments.src1.0,
                            arguments.src2.0,
                        )?;
                    }
                }
                ast::Instruction::Sub { data, arguments } => match data {
                    ast::ArithDetails::Integer(desc) => {
                        emit_sub_int(builder, map, desc.type_.into(), desc.saturate, arguments)?;
                    }
                    ast::ArithDetails::Float(desc) => {
                        emit_sub_float(builder, map, desc, arguments)?;
                    }
                },
                ast::Instruction::Min { data, arguments } => {
                    emit_min(builder, map, opencl, data, arguments)?;
                }
                ast::Instruction::Max { data, arguments } => {
                    emit_max(builder, map, opencl, data, arguments)?;
                }
                ast::Instruction::Rcp { data, arguments } => {
                    emit_rcp(builder, map, opencl, data, arguments)?;
                }
                ast::Instruction::And { data, arguments } => {
                    let result_type = map.get_or_add_scalar(builder, *data);
                    if *data == ast::ScalarType::Pred {
                        builder.logical_and(
                            result_type.0,
                            Some(arguments.dst.0),
                            arguments.src1.0,
                            arguments.src2.0,
                        )?;
                    } else {
                        builder.bitwise_and(
                            result_type.0,
                            Some(arguments.dst.0),
                            arguments.src1.0,
                            arguments.src2.0,
                        )?;
                    }
                }
                ast::Instruction::Selp { data, arguments } => {
                    let result_type = map.get_or_add_scalar(builder, *data);
                    builder.select(
                        result_type.0,
                        Some(arguments.dst.0),
                        arguments.src3.0,
                        arguments.src1.0,
                        arguments.src2.0,
                    )?;
                }
                // TODO: implement named barriers
                ast::Instruction::Bar { data, arguments } => {
                    let workgroup_scope = map.get_or_add_constant(
                        builder,
                        &ast::Type::Scalar(ast::ScalarType::U32),
                        &vec_repr(spirv::Scope::Workgroup as u32),
                    )?;
                    let barrier_semantics = map.get_or_add_constant(
                        builder,
                        &ast::Type::Scalar(ast::ScalarType::U32),
                        &vec_repr(
                            spirv::MemorySemantics::CROSS_WORKGROUP_MEMORY
                                | spirv::MemorySemantics::WORKGROUP_MEMORY
                                | spirv::MemorySemantics::SEQUENTIALLY_CONSISTENT,
                        ),
                    )?;
                    builder.control_barrier(
                        workgroup_scope.0,
                        workgroup_scope.0,
                        barrier_semantics.0,
                    )?;
                }
                ast::Instruction::Atom { data, arguments } => {
                    emit_atom(builder, map, data, arguments)?;
                }
                ast::Instruction::AtomCas { data, arguments } => {
                    let result_type = map.get_or_add_scalar(builder, data.type_);
                    let memory_const = map.get_or_add_constant(
                        builder,
                        &ast::Type::Scalar(ast::ScalarType::U32),
                        &vec_repr(scope_to_spirv(data.scope) as u32),
                    )?;
                    let semantics_const = map.get_or_add_constant(
                        builder,
                        &ast::Type::Scalar(ast::ScalarType::U32),
                        &vec_repr(semantics_to_spirv(data.semantics).bits()),
                    )?;
                    builder.atomic_compare_exchange(
                        result_type.0,
                        Some(arguments.dst.0),
                        arguments.src1.0,
                        memory_const.0,
                        semantics_const.0,
                        semantics_const.0,
                        arguments.src3.0,
                        arguments.src2.0,
                    )?;
                }
                ast::Instruction::Div { data, arguments } => match data {
                    ast::DivDetails::Unsigned(t) => {
                        let result_type = map.get_or_add_scalar(builder, (*t).into());
                        builder.u_div(
                            result_type.0,
                            Some(arguments.dst.0),
                            arguments.src1.0,
                            arguments.src2.0,
                        )?;
                    }
                    ast::DivDetails::Signed(t) => {
                        let result_type = map.get_or_add_scalar(builder, (*t).into());
                        builder.s_div(
                            result_type.0,
                            Some(arguments.dst.0),
                            arguments.src1.0,
                            arguments.src2.0,
                        )?;
                    }
                    ast::DivDetails::Float(t) => {
                        let result_type = map.get_or_add_scalar(builder, t.type_.into());
                        builder.f_div(
                            result_type.0,
                            Some(arguments.dst.0),
                            arguments.src1.0,
                            arguments.src2.0,
                        )?;
                        emit_float_div_decoration(builder, arguments.dst, t.kind);
                    }
                },
                ast::Instruction::Sqrt { data, arguments } => {
                    emit_sqrt(builder, map, opencl, data, arguments)?;
                }
                ast::Instruction::Rsqrt { data, arguments } => {
                    let result_type = map.get_or_add_scalar(builder, data.type_.into());
                    builder.ext_inst(
                        result_type.0,
                        Some(arguments.dst.0),
                        opencl,
                        spirv::CLOp::rsqrt as spirv::Word,
                        [dr::Operand::IdRef(arguments.src.0)].iter().cloned(),
                    )?;
                }
                ast::Instruction::Neg { data, arguments } => {
                    let result_type = map.get_or_add_scalar(builder, data.type_);
                    let negate_func = if data.type_.kind() == ast::ScalarKind::Float {
                        dr::Builder::f_negate
                    } else {
                        dr::Builder::s_negate
                    };
                    negate_func(
                        builder,
                        result_type.0,
                        Some(arguments.dst.0),
                        arguments.src.0,
                    )?;
                }
                ast::Instruction::Sin { arguments, .. } => {
                    let result_type = map.get_or_add_scalar(builder, ast::ScalarType::F32);
                    builder.ext_inst(
                        result_type.0,
                        Some(arguments.dst.0),
                        opencl,
                        spirv::CLOp::sin as u32,
                        [dr::Operand::IdRef(arguments.src.0)].iter().cloned(),
                    )?;
                }
                ast::Instruction::Cos { arguments, .. } => {
                    let result_type = map.get_or_add_scalar(builder, ast::ScalarType::F32);
                    builder.ext_inst(
                        result_type.0,
                        Some(arguments.dst.0),
                        opencl,
                        spirv::CLOp::cos as u32,
                        [dr::Operand::IdRef(arguments.src.0)].iter().cloned(),
                    )?;
                }
                ast::Instruction::Lg2 { arguments, .. } => {
                    let result_type = map.get_or_add_scalar(builder, ast::ScalarType::F32);
                    builder.ext_inst(
                        result_type.0,
                        Some(arguments.dst.0),
                        opencl,
                        spirv::CLOp::log2 as u32,
                        [dr::Operand::IdRef(arguments.src.0)].iter().cloned(),
                    )?;
                }
                ast::Instruction::Ex2 { arguments, .. } => {
                    let result_type = map.get_or_add_scalar(builder, ast::ScalarType::F32);
                    builder.ext_inst(
                        result_type.0,
                        Some(arguments.dst.0),
                        opencl,
                        spirv::CLOp::exp2 as u32,
                        [dr::Operand::IdRef(arguments.src.0)].iter().cloned(),
                    )?;
                }
                ast::Instruction::Clz { data, arguments } => {
                    let result_type = map.get_or_add_scalar(builder, (*data).into());
                    builder.ext_inst(
                        result_type.0,
                        Some(arguments.dst.0),
                        opencl,
                        spirv::CLOp::clz as u32,
                        [dr::Operand::IdRef(arguments.src.0)].iter().cloned(),
                    )?;
                }
                ast::Instruction::Brev { data, arguments } => {
                    let result_type = map.get_or_add_scalar(builder, (*data).into());
                    builder.bit_reverse(result_type.0, Some(arguments.dst.0), arguments.src.0)?;
                }
                ast::Instruction::Popc { data, arguments } => {
                    let result_type = map.get_or_add_scalar(builder, (*data).into());
                    builder.bit_count(result_type.0, Some(arguments.dst.0), arguments.src.0)?;
                }
                ast::Instruction::Xor { data, arguments } => {
                    let builder_fn: fn(
                        &mut dr::Builder,
                        u32,
                        Option<u32>,
                        u32,
                        u32,
                    ) -> Result<u32, dr::Error> = match data {
                        ast::ScalarType::Pred => emit_logical_xor_spirv,
                        _ => dr::Builder::bitwise_xor,
                    };
                    let result_type = map.get_or_add_scalar(builder, (*data).into());
                    builder_fn(
                        builder,
                        result_type.0,
                        Some(arguments.dst.0),
                        arguments.src1.0,
                        arguments.src2.0,
                    )?;
                }
                ast::Instruction::Bfe { .. }
                | ast::Instruction::Bfi { .. }
                | ast::Instruction::Activemask { .. } => {
                    // Should have beeen replaced with a funciton call earlier
                    return Err(error_unreachable());
                }

                ast::Instruction::Rem { data, arguments } => {
                    let builder_fn = if data.kind() == ast::ScalarKind::Signed {
                        dr::Builder::s_mod
                    } else {
                        dr::Builder::u_mod
                    };
                    let result_type = map.get_or_add_scalar(builder, (*data).into());
                    builder_fn(
                        builder,
                        result_type.0,
                        Some(arguments.dst.0),
                        arguments.src1.0,
                        arguments.src2.0,
                    )?;
                }
                ast::Instruction::Prmt { data, arguments } => {
                    let control = *data as u32;
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
                    let src1_vector = builder.bitcast(vec4_b8_type.0, None, arguments.src1.0)?;
                    let src2_vector = builder.bitcast(vec4_b8_type.0, None, arguments.src2.0)?;
                    let dst_vector = builder.vector_shuffle(
                        vec4_b8_type.0,
                        None,
                        src1_vector,
                        src2_vector,
                        components,
                    )?;
                    builder.bitcast(b32_type.0, Some(arguments.dst.0), dst_vector)?;
                }
                ast::Instruction::Membar { data } => {
                    let (scope, semantics) = match data {
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

                        ast::MemScope::Cluster => todo!(),
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
                    builder.memory_barrier(spirv_scope.0, spirv_semantics.0)?;
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
                            result_ptr_type.0,
                            None,
                            details.arg.src1.0,
                            [index_spirv.0].iter().copied(),
                        )?
                    }
                    None => details.arg.src1.0,
                };
                builder.store(dst_ptr, details.arg.src2.0, None, iter::empty())?;
            }
            Statement::RetValue(_, id) => {
                builder.ret_value(id.0)?;
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
                    SpirvType::pointer_to(underlying_type.clone(), space_to_spirv(*state_space)),
                );
                let ptr_src_u8 = builder.bitcast(u8_pointer.0, None, ptr_src.0)?;
                let temp = builder.in_bounds_ptr_access_chain(
                    u8_pointer.0,
                    None,
                    ptr_src_u8,
                    offset_src.0,
                    iter::empty(),
                )?;
                builder.bitcast(result_type.0, Some(dst.0), temp)?;
            }
            Statement::RepackVector(repack) => {
                if repack.is_extract {
                    let scalar_type = map.get_or_add_scalar(builder, repack.typ);
                    for (index, dst_id) in repack.unpacked.iter().enumerate() {
                        builder.composite_extract(
                            scalar_type.0,
                            Some(dst_id.0),
                            repack.packed.0,
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
                    let mut temp_vec = builder.undef(vector_type.0, None);
                    for (index, src_id) in repack.unpacked.iter().enumerate() {
                        temp_vec = builder.composite_insert(
                            vector_type.0,
                            None,
                            src_id.0,
                            temp_vec,
                            [index as u32].iter().copied(),
                        )?;
                    }
                    builder.copy_object(vector_type.0, Some(repack.packed.0), temp_vec)?;
                }
            }
        }
    }
    Ok(())
}

fn emit_function_linkage<'input>(
    builder: &mut dr::Builder,
    id_defs: &GlobalStringIdResolver<'input>,
    f: &Function,
    fn_name: SpirvWord,
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

fn get_function_type(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    spirv_input: impl Iterator<Item = SpirvType>,
    spirv_output: &[ast::Variable<SpirvWord>],
) -> (SpirvWord, SpirvWord) {
    map.get_or_add_fn(
        builder,
        spirv_input,
        spirv_output
            .iter()
            .map(|var| SpirvType::new(var.v_type.clone())),
    )
}

fn emit_linking_decoration<'input>(
    builder: &mut dr::Builder,
    id_defs: &GlobalStringIdResolver<'input>,
    name_override: Option<&str>,
    name: SpirvWord,
    linking: ast::LinkingDirective,
) {
    if linking == ast::LinkingDirective::NONE {
        return;
    }
    if linking.contains(ast::LinkingDirective::VISIBLE) {
        let string_name =
            name_override.unwrap_or_else(|| id_defs.reverse_variables.get(&name).unwrap());
        builder.decorate(
            name.0,
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
            name.0,
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

fn effective_input_arguments<'a>(
    this: &'a ast::MethodDeclaration<'a, SpirvWord>,
) -> impl Iterator<Item = (SpirvWord, SpirvType)> + 'a {
    let is_kernel = matches!(this.name, ast::MethodName::Kernel(_));
    this.input_arguments.iter().map(move |arg| {
        if !is_kernel && arg.state_space != ast::StateSpace::Reg {
            let spirv_type =
                SpirvType::pointer_to(arg.v_type.clone(), space_to_spirv(arg.state_space));
            (arg.name, spirv_type)
        } else {
            (arg.name, SpirvType::new(arg.v_type.clone()))
        }
    })
}

fn emit_implicit_conversion(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    cv: &ImplicitConversion,
) -> Result<(), TranslateError> {
    let from_parts = to_parts(&cv.from_type);
    let to_parts = to_parts(&cv.to_type);
    match (from_parts.kind, to_parts.kind, &cv.kind) {
        (_, _, &ConversionKind::BitToPtr) => {
            let dst_type = map.get_or_add(
                builder,
                SpirvType::pointer_to(cv.to_type.clone(), space_to_spirv(cv.to_space)),
            );
            builder.convert_u_to_ptr(dst_type.0, Some(cv.dst.0), cv.src.0)?;
        }
        (TypeKind::Scalar, TypeKind::Scalar, &ConversionKind::Default) => {
            if from_parts.width == to_parts.width {
                let dst_type = map.get_or_add(builder, SpirvType::new(cv.to_type.clone()));
                if from_parts.scalar_kind != ast::ScalarKind::Float
                    && to_parts.scalar_kind != ast::ScalarKind::Float
                {
                    // It is noop, but another instruction expects result of this conversion
                    builder.copy_object(dst_type.0, Some(cv.dst.0), cv.src.0)?;
                } else {
                    builder.bitcast(dst_type.0, Some(cv.dst.0), cv.src.0)?;
                }
            } else {
                // This block is safe because it's illegal to implictly convert between floating point values
                let same_width_bit_type = map.get_or_add(
                    builder,
                    SpirvType::new(type_from_parts(TypeParts {
                        scalar_kind: ast::ScalarKind::Bit,
                        ..from_parts
                    })),
                );
                let same_width_bit_value =
                    builder.bitcast(same_width_bit_type.0, None, cv.src.0)?;
                let wide_bit_type = type_from_parts(TypeParts {
                    scalar_kind: ast::ScalarKind::Bit,
                    ..to_parts
                });
                let wide_bit_type_spirv =
                    map.get_or_add(builder, SpirvType::new(wide_bit_type.clone()));
                if to_parts.scalar_kind == ast::ScalarKind::Unsigned
                    || to_parts.scalar_kind == ast::ScalarKind::Bit
                {
                    builder.u_convert(
                        wide_bit_type_spirv.0,
                        Some(cv.dst.0),
                        same_width_bit_value,
                    )?;
                } else {
                    let conversion_fn = if from_parts.scalar_kind == ast::ScalarKind::Signed
                        && to_parts.scalar_kind == ast::ScalarKind::Signed
                    {
                        dr::Builder::s_convert
                    } else {
                        dr::Builder::u_convert
                    };
                    let wide_bit_value =
                        conversion_fn(builder, wide_bit_type_spirv.0, None, same_width_bit_value)?;
                    emit_implicit_conversion(
                        builder,
                        map,
                        &ImplicitConversion {
                            src: SpirvWord(wide_bit_value),
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
            builder.s_convert(result_type.0, Some(cv.dst.0), cv.src.0)?;
        }
        (TypeKind::Vector, TypeKind::Scalar, &ConversionKind::Default)
        | (TypeKind::Scalar, TypeKind::Array, &ConversionKind::Default)
        | (TypeKind::Array, TypeKind::Scalar, &ConversionKind::Default) => {
            let into_type = map.get_or_add(builder, SpirvType::new(cv.to_type.clone()));
            builder.bitcast(into_type.0, Some(cv.dst.0), cv.src.0)?;
        }
        (_, _, &ConversionKind::PtrToPtr) => {
            let result_type = map.get_or_add(
                builder,
                SpirvType::Pointer(
                    Box::new(SpirvType::new(cv.to_type.clone())),
                    space_to_spirv(cv.to_space),
                ),
            );
            if cv.to_space == ast::StateSpace::Generic && cv.from_space != ast::StateSpace::Generic
            {
                let src = if cv.from_type != cv.to_type {
                    let temp_type = map.get_or_add(
                        builder,
                        SpirvType::Pointer(
                            Box::new(SpirvType::new(cv.to_type.clone())),
                            space_to_spirv(cv.from_space),
                        ),
                    );
                    builder.bitcast(temp_type.0, None, cv.src.0)?
                } else {
                    cv.src.0
                };
                builder.ptr_cast_to_generic(result_type.0, Some(cv.dst.0), src)?;
            } else if cv.from_space == ast::StateSpace::Generic
                && cv.to_space != ast::StateSpace::Generic
            {
                let src = if cv.from_type != cv.to_type {
                    let temp_type = map.get_or_add(
                        builder,
                        SpirvType::Pointer(
                            Box::new(SpirvType::new(cv.to_type.clone())),
                            space_to_spirv(cv.from_space),
                        ),
                    );
                    builder.bitcast(temp_type.0, None, cv.src.0)?
                } else {
                    cv.src.0
                };
                builder.generic_cast_to_ptr(result_type.0, Some(cv.dst.0), src)?;
            } else {
                builder.bitcast(result_type.0, Some(cv.dst.0), cv.src.0)?;
            }
        }
        (_, _, &ConversionKind::AddressOf) => {
            let dst_type = map.get_or_add(builder, SpirvType::new(cv.to_type.clone()));
            builder.convert_ptr_to_u(dst_type.0, Some(cv.dst.0), cv.src.0)?;
        }
        (TypeKind::Pointer, TypeKind::Scalar, &ConversionKind::Default) => {
            let result_type = map.get_or_add(builder, SpirvType::new(cv.to_type.clone()));
            builder.convert_ptr_to_u(result_type.0, Some(cv.dst.0), cv.src.0)?;
        }
        (TypeKind::Scalar, TypeKind::Pointer, &ConversionKind::Default) => {
            let result_type = map.get_or_add(builder, SpirvType::new(cv.to_type.clone()));
            builder.convert_u_to_ptr(result_type.0, Some(cv.dst.0), cv.src.0)?;
        }
        _ => unreachable!(),
    }
    Ok(())
}

fn vec_repr<T: Copy>(t: T) -> Vec<u8> {
    let mut result = vec![0; mem::size_of::<T>()];
    unsafe { std::ptr::copy_nonoverlapping(&t, result.as_mut_ptr() as *mut _, 1) };
    result
}

fn emit_abs(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    opencl: spirv::Word,
    d: &ast::TypeFtz,
    arg: &ast::AbsArgs<SpirvWord>,
) -> Result<(), dr::Error> {
    let scalar_t = ast::ScalarType::from(d.type_);
    let result_type = map.get_or_add(builder, SpirvType::from(scalar_t));
    let cl_abs = if scalar_t.kind() == ast::ScalarKind::Signed {
        spirv::CLOp::s_abs
    } else {
        spirv::CLOp::fabs
    };
    builder.ext_inst(
        result_type.0,
        Some(arg.dst.0),
        opencl,
        cl_abs as spirv::Word,
        [dr::Operand::IdRef(arg.src.0)].iter().cloned(),
    )?;
    Ok(())
}

fn emit_mul_int(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    opencl: spirv::Word,
    type_: ast::ScalarType,
    control: ast::MulIntControl,
    arg: &ast::MulArgs<SpirvWord>,
) -> Result<(), dr::Error> {
    let inst_type = map.get_or_add(builder, SpirvType::from(type_));
    match control {
        ast::MulIntControl::Low => {
            builder.i_mul(inst_type.0, Some(arg.dst.0), arg.src1.0, arg.src2.0)?;
        }
        ast::MulIntControl::High => {
            builder.ext_inst(
                inst_type.0,
                Some(arg.dst.0),
                opencl,
                spirv::CLOp::s_mul_hi as spirv::Word,
                [
                    dr::Operand::IdRef(arg.src1.0),
                    dr::Operand::IdRef(arg.src2.0),
                ]
                .iter()
                .cloned(),
            )?;
        }
        ast::MulIntControl::Wide => {
            let instr_width = type_.size_of();
            let instr_kind = type_.kind();
            let dst_type = scalar_from_parts(instr_width * 2, instr_kind);
            let dst_type_id = map.get_or_add_scalar(builder, dst_type);
            let (src1, src2) = if type_.kind() == ast::ScalarKind::Signed {
                let src1 = builder.s_convert(dst_type_id.0, None, arg.src1.0)?;
                let src2 = builder.s_convert(dst_type_id.0, None, arg.src2.0)?;
                (src1, src2)
            } else {
                let src1 = builder.u_convert(dst_type_id.0, None, arg.src1.0)?;
                let src2 = builder.u_convert(dst_type_id.0, None, arg.src2.0)?;
                (src1, src2)
            };
            builder.i_mul(dst_type_id.0, Some(arg.dst.0), src1, src2)?;
            builder.decorate(arg.dst.0, spirv::Decoration::NoSignedWrap, iter::empty());
        }
    }
    Ok(())
}

fn emit_mul_float(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    ctr: &ast::ArithFloat,
    arg: &ast::MulArgs<SpirvWord>,
) -> Result<(), dr::Error> {
    if ctr.saturate {
        todo!()
    }
    let result_type = map.get_or_add_scalar(builder, ctr.type_.into());
    builder.f_mul(result_type.0, Some(arg.dst.0), arg.src1.0, arg.src2.0)?;
    emit_rounding_decoration(builder, arg.dst, ctr.rounding);
    Ok(())
}

fn scalar_from_parts(width: u8, kind: ast::ScalarKind) -> ast::ScalarType {
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
        ast::ScalarKind::Pred => ast::ScalarType::Pred,
    }
}

fn emit_rounding_decoration(
    builder: &mut dr::Builder,
    dst: SpirvWord,
    rounding: Option<ast::RoundingMode>,
) {
    if let Some(rounding) = rounding {
        builder.decorate(
            dst.0,
            spirv::Decoration::FPRoundingMode,
            [rounding_to_spirv(rounding)].iter().cloned(),
        );
    }
}

fn rounding_to_spirv(this: ast::RoundingMode) -> rspirv::dr::Operand {
    let mode = match this {
        ast::RoundingMode::NearestEven => spirv::FPRoundingMode::RTE,
        ast::RoundingMode::Zero => spirv::FPRoundingMode::RTZ,
        ast::RoundingMode::PositiveInf => spirv::FPRoundingMode::RTP,
        ast::RoundingMode::NegativeInf => spirv::FPRoundingMode::RTN,
    };
    rspirv::dr::Operand::FPRoundingMode(mode)
}

fn emit_add_int(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    typ: ast::ScalarType,
    saturate: bool,
    arg: &ast::AddArgs<SpirvWord>,
) -> Result<(), dr::Error> {
    if saturate {
        todo!()
    }
    let inst_type = map.get_or_add(builder, SpirvType::from(ast::ScalarType::from(typ)));
    builder.i_add(inst_type.0, Some(arg.dst.0), arg.src1.0, arg.src2.0)?;
    Ok(())
}

fn emit_add_float(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    desc: &ast::ArithFloat,
    arg: &ast::AddArgs<SpirvWord>,
) -> Result<(), dr::Error> {
    let inst_type = map.get_or_add(builder, SpirvType::from(ast::ScalarType::from(desc.type_)));
    builder.f_add(inst_type.0, Some(arg.dst.0), arg.src1.0, arg.src2.0)?;
    emit_rounding_decoration(builder, arg.dst, desc.rounding);
    Ok(())
}

fn emit_setp(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    setp: &ast::SetpData,
    arg: &ast::SetpArgs<SpirvWord>,
) -> Result<(), dr::Error> {
    let result_type = map
        .get_or_add(builder, SpirvType::Base(SpirvScalarKey::Pred))
        .0;
    let result_id = Some(arg.dst1.0);
    let operand_1 = arg.src1.0;
    let operand_2 = arg.src2.0;
    match setp.cmp_op {
        ast::SetpCompareOp::Integer(ast::SetpCompareInt::Eq) => {
            builder.i_equal(result_type, result_id, operand_1, operand_2)
        }
        ast::SetpCompareOp::Float(ast::SetpCompareFloat::Eq) => {
            builder.f_ord_equal(result_type, result_id, operand_1, operand_2)
        }
        ast::SetpCompareOp::Integer(ast::SetpCompareInt::NotEq) => {
            builder.i_not_equal(result_type, result_id, operand_1, operand_2)
        }
        ast::SetpCompareOp::Float(ast::SetpCompareFloat::NotEq) => {
            builder.f_ord_not_equal(result_type, result_id, operand_1, operand_2)
        }
        ast::SetpCompareOp::Integer(ast::SetpCompareInt::UnsignedLess) => {
            builder.u_less_than(result_type, result_id, operand_1, operand_2)
        }
        ast::SetpCompareOp::Integer(ast::SetpCompareInt::SignedLess) => {
            builder.s_less_than(result_type, result_id, operand_1, operand_2)
        }
        ast::SetpCompareOp::Float(ast::SetpCompareFloat::Less) => {
            builder.f_ord_less_than(result_type, result_id, operand_1, operand_2)
        }
        ast::SetpCompareOp::Integer(ast::SetpCompareInt::UnsignedLessOrEq) => {
            builder.u_less_than_equal(result_type, result_id, operand_1, operand_2)
        }
        ast::SetpCompareOp::Integer(ast::SetpCompareInt::SignedLessOrEq) => {
            builder.s_less_than_equal(result_type, result_id, operand_1, operand_2)
        }
        ast::SetpCompareOp::Float(ast::SetpCompareFloat::LessOrEq) => {
            builder.f_ord_less_than_equal(result_type, result_id, operand_1, operand_2)
        }
        ast::SetpCompareOp::Integer(ast::SetpCompareInt::UnsignedGreater) => {
            builder.u_greater_than(result_type, result_id, operand_1, operand_2)
        }
        ast::SetpCompareOp::Integer(ast::SetpCompareInt::SignedGreater) => {
            builder.s_greater_than(result_type, result_id, operand_1, operand_2)
        }
        ast::SetpCompareOp::Float(ast::SetpCompareFloat::Greater) => {
            builder.f_ord_greater_than(result_type, result_id, operand_1, operand_2)
        }
        ast::SetpCompareOp::Integer(ast::SetpCompareInt::UnsignedGreaterOrEq) => {
            builder.u_greater_than_equal(result_type, result_id, operand_1, operand_2)
        }
        ast::SetpCompareOp::Integer(ast::SetpCompareInt::SignedGreaterOrEq) => {
            builder.s_greater_than_equal(result_type, result_id, operand_1, operand_2)
        }
        ast::SetpCompareOp::Float(ast::SetpCompareFloat::GreaterOrEq) => {
            builder.f_ord_greater_than_equal(result_type, result_id, operand_1, operand_2)
        }
        ast::SetpCompareOp::Float(ast::SetpCompareFloat::NanEq) => {
            builder.f_unord_equal(result_type, result_id, operand_1, operand_2)
        }
        ast::SetpCompareOp::Float(ast::SetpCompareFloat::NanNotEq) => {
            builder.f_unord_not_equal(result_type, result_id, operand_1, operand_2)
        }
        ast::SetpCompareOp::Float(ast::SetpCompareFloat::NanLess) => {
            builder.f_unord_less_than(result_type, result_id, operand_1, operand_2)
        }
        ast::SetpCompareOp::Float(ast::SetpCompareFloat::NanLessOrEq) => {
            builder.f_unord_less_than_equal(result_type, result_id, operand_1, operand_2)
        }
        ast::SetpCompareOp::Float(ast::SetpCompareFloat::NanGreater) => {
            builder.f_unord_greater_than(result_type, result_id, operand_1, operand_2)
        }
        ast::SetpCompareOp::Float(ast::SetpCompareFloat::NanGreaterOrEq) => {
            builder.f_unord_greater_than_equal(result_type, result_id, operand_1, operand_2)
        }
        ast::SetpCompareOp::Float(ast::SetpCompareFloat::IsAnyNan) => {
            let temp1 = builder.is_nan(result_type, None, operand_1)?;
            let temp2 = builder.is_nan(result_type, None, operand_2)?;
            builder.logical_or(result_type, result_id, temp1, temp2)
        }
        ast::SetpCompareOp::Float(ast::SetpCompareFloat::IsNotNan) => {
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
    Ok(builder.u_convert(result_type.0, None, offset_var)?)
}

fn emit_cvt(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    opencl: spirv::Word,
    dets: &ast::CvtDetails,
    arg: &ast::CvtArgs<SpirvWord>,
) -> Result<(), TranslateError> {
    match dets.mode {
        ptx_parser::CvtMode::SignExtend => {
            let cv = ImplicitConversion {
                src: arg.src,
                dst: arg.dst,
                from_type: dets.from.into(),
                from_space: ast::StateSpace::Reg,
                to_type: dets.to.into(),
                to_space: ast::StateSpace::Reg,
                kind: ConversionKind::SignExtend,
            };
            emit_implicit_conversion(builder, map, &cv)?;
        }
        ptx_parser::CvtMode::ZeroExtend
        | ptx_parser::CvtMode::Truncate
        | ptx_parser::CvtMode::Bitcast => {
            let cv = ImplicitConversion {
                src: arg.src,
                dst: arg.dst,
                from_type: dets.from.into(),
                from_space: ast::StateSpace::Reg,
                to_type: dets.to.into(),
                to_space: ast::StateSpace::Reg,
                kind: ConversionKind::Default,
            };
            emit_implicit_conversion(builder, map, &cv)?;
        }
        ptx_parser::CvtMode::SaturateUnsignedToSigned => {
            let result_type = map.get_or_add(builder, SpirvType::from(dets.to));
            builder.sat_convert_u_to_s(result_type.0, Some(arg.dst.0), arg.src.0)?;
        }
        ptx_parser::CvtMode::SaturateSignedToUnsigned => {
            let result_type = map.get_or_add(builder, SpirvType::from(dets.to));
            builder.sat_convert_s_to_u(result_type.0, Some(arg.dst.0), arg.src.0)?;
        }
        ptx_parser::CvtMode::FPExtend { flush_to_zero } => {
            if flush_to_zero == Some(true) {
                todo!()
            }
            let result_type = map.get_or_add(builder, SpirvType::from(dets.to));
            builder.f_convert(result_type.0, Some(arg.dst.0), arg.src.0)?;
        }
        ptx_parser::CvtMode::FPTruncate {
            rounding,
            flush_to_zero,
        } => {
            if flush_to_zero == Some(true) {
                todo!()
            }
            let result_type = map.get_or_add(builder, SpirvType::from(dets.to));
            builder.f_convert(result_type.0, Some(arg.dst.0), arg.src.0)?;
            emit_rounding_decoration(builder, arg.dst, Some(rounding));
        }
        ptx_parser::CvtMode::FPRound {
            integer_rounding,
            flush_to_zero,
        } => {
            if flush_to_zero == Some(true) {
                todo!()
            }
            let result_type = map.get_or_add(builder, SpirvType::from(dets.to));
            match integer_rounding {
                Some(ast::RoundingMode::NearestEven) => {
                    builder.ext_inst(
                        result_type.0,
                        Some(arg.dst.0),
                        opencl,
                        spirv::CLOp::rint as u32,
                        [dr::Operand::IdRef(arg.src.0)].iter().cloned(),
                    )?;
                }
                Some(ast::RoundingMode::Zero) => {
                    builder.ext_inst(
                        result_type.0,
                        Some(arg.dst.0),
                        opencl,
                        spirv::CLOp::trunc as u32,
                        [dr::Operand::IdRef(arg.src.0)].iter().cloned(),
                    )?;
                }
                Some(ast::RoundingMode::NegativeInf) => {
                    builder.ext_inst(
                        result_type.0,
                        Some(arg.dst.0),
                        opencl,
                        spirv::CLOp::floor as u32,
                        [dr::Operand::IdRef(arg.src.0)].iter().cloned(),
                    )?;
                }
                Some(ast::RoundingMode::PositiveInf) => {
                    builder.ext_inst(
                        result_type.0,
                        Some(arg.dst.0),
                        opencl,
                        spirv::CLOp::ceil as u32,
                        [dr::Operand::IdRef(arg.src.0)].iter().cloned(),
                    )?;
                }
                None => {
                    builder.copy_object(result_type.0, Some(arg.dst.0), arg.src.0)?;
                }
            }
        }
        ptx_parser::CvtMode::SignedFromFP {
            rounding,
            flush_to_zero,
        } => {
            if flush_to_zero == Some(true) {
                todo!()
            }
            let dest_t: ast::ScalarType = dets.to.into();
            let result_type = map.get_or_add(builder, SpirvType::from(dest_t));
            builder.convert_f_to_s(result_type.0, Some(arg.dst.0), arg.src.0)?;
            emit_rounding_decoration(builder, arg.dst, Some(rounding));
        }
        ptx_parser::CvtMode::UnsignedFromFP {
            rounding,
            flush_to_zero,
        } => {
            if flush_to_zero == Some(true) {
                todo!()
            }
            let dest_t: ast::ScalarType = dets.to.into();
            let result_type = map.get_or_add(builder, SpirvType::from(dest_t));
            builder.convert_f_to_u(result_type.0, Some(arg.dst.0), arg.src.0)?;
            emit_rounding_decoration(builder, arg.dst, Some(rounding));
        }
        ptx_parser::CvtMode::FPFromSigned(rounding) => {
            let dest_t: ast::ScalarType = dets.to.into();
            let result_type = map.get_or_add(builder, SpirvType::from(dest_t));
            builder.convert_s_to_f(result_type.0, Some(arg.dst.0), arg.src.0)?;
            emit_rounding_decoration(builder, arg.dst, Some(rounding));
        }
        ptx_parser::CvtMode::FPFromUnsigned(rounding) => {
            let dest_t: ast::ScalarType = dets.to.into();
            let result_type = map.get_or_add(builder, SpirvType::from(dest_t));
            builder.convert_u_to_f(result_type.0, Some(arg.dst.0), arg.src.0)?;
            emit_rounding_decoration(builder, arg.dst, Some(rounding));
        }
    }
    Ok(())
}

fn emit_mad_uint(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    opencl: spirv::Word,
    type_: ast::ScalarType,
    control: ast::MulIntControl,
    arg: &ast::MadArgs<SpirvWord>,
) -> Result<(), dr::Error> {
    let inst_type = map
        .get_or_add(builder, SpirvType::from(ast::ScalarType::from(type_)))
        .0;
    match control {
        ast::MulIntControl::Low => {
            let mul_result = builder.i_mul(inst_type, None, arg.src1.0, arg.src2.0)?;
            builder.i_add(inst_type, Some(arg.dst.0), arg.src3.0, mul_result)?;
        }
        ast::MulIntControl::High => {
            builder.ext_inst(
                inst_type,
                Some(arg.dst.0),
                opencl,
                spirv::CLOp::u_mad_hi as spirv::Word,
                [
                    dr::Operand::IdRef(arg.src1.0),
                    dr::Operand::IdRef(arg.src2.0),
                    dr::Operand::IdRef(arg.src3.0),
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
    type_: ast::ScalarType,
    control: ast::MulIntControl,
    arg: &ast::MadArgs<SpirvWord>,
) -> Result<(), dr::Error> {
    let inst_type = map.get_or_add(builder, SpirvType::from(type_)).0;
    match control {
        ast::MulIntControl::Low => {
            let mul_result = builder.i_mul(inst_type, None, arg.src1.0, arg.src2.0)?;
            builder.i_add(inst_type, Some(arg.dst.0), arg.src3.0, mul_result)?;
        }
        ast::MulIntControl::High => {
            builder.ext_inst(
                inst_type,
                Some(arg.dst.0),
                opencl,
                spirv::CLOp::s_mad_hi as spirv::Word,
                [
                    dr::Operand::IdRef(arg.src1.0),
                    dr::Operand::IdRef(arg.src2.0),
                    dr::Operand::IdRef(arg.src3.0),
                ]
                .iter()
                .cloned(),
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
    arg: &ast::MadArgs<SpirvWord>,
) -> Result<(), dr::Error> {
    let inst_type = map
        .get_or_add(builder, SpirvType::from(ast::ScalarType::from(desc.type_)))
        .0;
    builder.ext_inst(
        inst_type,
        Some(arg.dst.0),
        opencl,
        spirv::CLOp::mad as spirv::Word,
        [
            dr::Operand::IdRef(arg.src1.0),
            dr::Operand::IdRef(arg.src2.0),
            dr::Operand::IdRef(arg.src3.0),
        ]
        .iter()
        .cloned(),
    )?;
    Ok(())
}

fn emit_fma_float(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    opencl: spirv::Word,
    desc: &ast::ArithFloat,
    arg: &ast::FmaArgs<SpirvWord>,
) -> Result<(), dr::Error> {
    let inst_type = map
        .get_or_add(builder, SpirvType::from(ast::ScalarType::from(desc.type_)))
        .0;
    builder.ext_inst(
        inst_type,
        Some(arg.dst.0),
        opencl,
        spirv::CLOp::fma as spirv::Word,
        [
            dr::Operand::IdRef(arg.src1.0),
            dr::Operand::IdRef(arg.src2.0),
            dr::Operand::IdRef(arg.src3.0),
        ]
        .iter()
        .cloned(),
    )?;
    Ok(())
}

fn emit_sub_int(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    typ: ast::ScalarType,
    saturate: bool,
    arg: &ast::SubArgs<SpirvWord>,
) -> Result<(), dr::Error> {
    if saturate {
        todo!()
    }
    let inst_type = map
        .get_or_add(builder, SpirvType::from(ast::ScalarType::from(typ)))
        .0;
    builder.i_sub(inst_type, Some(arg.dst.0), arg.src1.0, arg.src2.0)?;
    Ok(())
}

fn emit_sub_float(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    desc: &ast::ArithFloat,
    arg: &ast::SubArgs<SpirvWord>,
) -> Result<(), dr::Error> {
    let inst_type = map
        .get_or_add(builder, SpirvType::from(ast::ScalarType::from(desc.type_)))
        .0;
    builder.f_sub(inst_type, Some(arg.dst.0), arg.src1.0, arg.src2.0)?;
    emit_rounding_decoration(builder, arg.dst, desc.rounding);
    Ok(())
}

fn emit_min(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    opencl: spirv::Word,
    desc: &ast::MinMaxDetails,
    arg: &ast::MinArgs<SpirvWord>,
) -> Result<(), dr::Error> {
    let cl_op = match desc {
        ast::MinMaxDetails::Signed(_) => spirv::CLOp::s_min,
        ast::MinMaxDetails::Unsigned(_) => spirv::CLOp::u_min,
        ast::MinMaxDetails::Float(_) => spirv::CLOp::fmin,
    };
    let inst_type = map.get_or_add(builder, SpirvType::from(desc.type_()));
    builder.ext_inst(
        inst_type.0,
        Some(arg.dst.0),
        opencl,
        cl_op as spirv::Word,
        [
            dr::Operand::IdRef(arg.src1.0),
            dr::Operand::IdRef(arg.src2.0),
        ]
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
    arg: &ast::MaxArgs<SpirvWord>,
) -> Result<(), dr::Error> {
    let cl_op = match desc {
        ast::MinMaxDetails::Signed(_) => spirv::CLOp::s_max,
        ast::MinMaxDetails::Unsigned(_) => spirv::CLOp::u_max,
        ast::MinMaxDetails::Float(_) => spirv::CLOp::fmax,
    };
    let inst_type = map.get_or_add(builder, SpirvType::from(desc.type_()));
    builder.ext_inst(
        inst_type.0,
        Some(arg.dst.0),
        opencl,
        cl_op as spirv::Word,
        [
            dr::Operand::IdRef(arg.src1.0),
            dr::Operand::IdRef(arg.src2.0),
        ]
        .iter()
        .cloned(),
    )?;
    Ok(())
}

fn emit_rcp(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    opencl: spirv::Word,
    desc: &ast::RcpData,
    arg: &ast::RcpArgs<SpirvWord>,
) -> Result<(), TranslateError> {
    let is_f64 = desc.type_ == ast::ScalarType::F64;
    let (instr_type, constant) = if is_f64 {
        (ast::ScalarType::F64, vec_repr(1.0f64))
    } else {
        (ast::ScalarType::F32, vec_repr(1.0f32))
    };
    let result_type = map.get_or_add_scalar(builder, instr_type);
    let rounding = match desc.kind {
        ptx_parser::RcpKind::Approx => {
            builder.ext_inst(
                result_type.0,
                Some(arg.dst.0),
                opencl,
                spirv::CLOp::native_recip as u32,
                [dr::Operand::IdRef(arg.src.0)].iter().cloned(),
            )?;
            return Ok(());
        }
        ptx_parser::RcpKind::Compliant(rounding) => rounding,
    };
    let one = map.get_or_add_constant(builder, &ast::Type::Scalar(instr_type), &constant)?;
    builder.f_div(result_type.0, Some(arg.dst.0), one.0, arg.src.0)?;
    emit_rounding_decoration(builder, arg.dst, Some(rounding));
    builder.decorate(
        arg.dst.0,
        spirv::Decoration::FPFastMathMode,
        [dr::Operand::FPFastMathMode(
            spirv::FPFastMathMode::ALLOW_RECIP,
        )]
        .iter()
        .cloned(),
    );
    Ok(())
}

fn emit_atom(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    details: &ast::AtomDetails,
    arg: &ast::AtomArgs<SpirvWord>,
) -> Result<(), TranslateError> {
    let spirv_op = match details.op {
        ptx_parser::AtomicOp::And => dr::Builder::atomic_and,
        ptx_parser::AtomicOp::Or => dr::Builder::atomic_or,
        ptx_parser::AtomicOp::Xor => dr::Builder::atomic_xor,
        ptx_parser::AtomicOp::Exchange => dr::Builder::atomic_exchange,
        ptx_parser::AtomicOp::Add => dr::Builder::atomic_i_add,
        ptx_parser::AtomicOp::IncrementWrap | ptx_parser::AtomicOp::DecrementWrap => {
            return Err(error_unreachable())
        }
        ptx_parser::AtomicOp::SignedMin => dr::Builder::atomic_s_min,
        ptx_parser::AtomicOp::UnsignedMin => dr::Builder::atomic_u_min,
        ptx_parser::AtomicOp::SignedMax => dr::Builder::atomic_s_max,
        ptx_parser::AtomicOp::UnsignedMax => dr::Builder::atomic_u_max,
        ptx_parser::AtomicOp::FloatAdd => dr::Builder::atomic_f_add_ext,
        ptx_parser::AtomicOp::FloatMin => todo!(),
        ptx_parser::AtomicOp::FloatMax => todo!(),
    };
    let result_type = map.get_or_add(builder, SpirvType::new(details.type_.clone()));
    let memory_const = map.get_or_add_constant(
        builder,
        &ast::Type::Scalar(ast::ScalarType::U32),
        &vec_repr(scope_to_spirv(details.scope) as u32),
    )?;
    let semantics_const = map.get_or_add_constant(
        builder,
        &ast::Type::Scalar(ast::ScalarType::U32),
        &vec_repr(semantics_to_spirv(details.semantics).bits()),
    )?;
    spirv_op(
        builder,
        result_type.0,
        Some(arg.dst.0),
        arg.src1.0,
        memory_const.0,
        semantics_const.0,
        arg.src2.0,
    )?;
    Ok(())
}

fn scope_to_spirv(this: ast::MemScope) -> spirv::Scope {
    match this {
        ast::MemScope::Cta => spirv::Scope::Workgroup,
        ast::MemScope::Gpu => spirv::Scope::Device,
        ast::MemScope::Sys => spirv::Scope::CrossDevice,
        ptx_parser::MemScope::Cluster => todo!(),
    }
}

fn semantics_to_spirv(this: ast::AtomSemantics) -> spirv::MemorySemantics {
    match this {
        ast::AtomSemantics::Relaxed => spirv::MemorySemantics::RELAXED,
        ast::AtomSemantics::Acquire => spirv::MemorySemantics::ACQUIRE,
        ast::AtomSemantics::Release => spirv::MemorySemantics::RELEASE,
        ast::AtomSemantics::AcqRel => spirv::MemorySemantics::ACQUIRE_RELEASE,
    }
}

fn emit_float_div_decoration(builder: &mut dr::Builder, dst: SpirvWord, kind: ast::DivFloatKind) {
    match kind {
        ast::DivFloatKind::Approx => {
            builder.decorate(
                dst.0,
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
        ast::DivFloatKind::ApproxFull => {}
    }
}

fn emit_sqrt(
    builder: &mut dr::Builder,
    map: &mut TypeWordMap,
    opencl: spirv::Word,
    details: &ast::RcpData,
    a: &ast::SqrtArgs<SpirvWord>,
) -> Result<(), TranslateError> {
    let result_type = map.get_or_add_scalar(builder, details.type_.into());
    let (ocl_op, rounding) = match details.kind {
        ast::RcpKind::Approx => (spirv::CLOp::sqrt, None),
        ast::RcpKind::Compliant(rnd) => (spirv::CLOp::sqrt, Some(rnd)),
    };
    builder.ext_inst(
        result_type.0,
        Some(a.dst.0),
        opencl,
        ocl_op as spirv::Word,
        [dr::Operand::IdRef(a.src.0)].iter().cloned(),
    )?;
    emit_rounding_decoration(builder, a.dst, rounding);
    Ok(())
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
                _ => return Err(error_mismatched_type()),
            };
            let vector_type_spirv = map.get_or_add(builder, SpirvType::new(vector_type));
            let vector_temp = builder.load(
                vector_type_spirv.0,
                None,
                details.arg.src.0,
                None,
                iter::empty(),
            )?;
            builder.composite_extract(
                result_type.0,
                Some(details.arg.dst.0),
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
                result_ptr_type.0,
                None,
                details.arg.src.0,
                [index_spirv.0].iter().copied(),
            )?;
            builder.load(
                result_type.0,
                Some(details.arg.dst.0),
                src,
                None,
                iter::empty(),
            )?;
        }
        None => {
            builder.load(
                result_type.0,
                Some(details.arg.dst.0),
                details.arg.src.0,
                None,
                iter::empty(),
            )?;
        }
    };
    Ok(())
}

fn to_parts(this: &ast::Type) -> TypeParts {
    match this {
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

fn type_from_parts(t: TypeParts) -> ast::Type {
    match t.kind {
        TypeKind::Scalar => ast::Type::Scalar(scalar_from_parts(t.width, t.scalar_kind)),
        TypeKind::Vector => ast::Type::Vector(
            scalar_from_parts(t.width, t.scalar_kind),
            t.components[0] as u8,
        ),
        TypeKind::Array => {
            ast::Type::Array(scalar_from_parts(t.width, t.scalar_kind), t.components)
        }
        TypeKind::Pointer => {
            ast::Type::Pointer(scalar_from_parts(t.width, t.scalar_kind), t.state_space)
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
