use hip_common::CompilationMode;
use rustc_hash::FxHashMap;
use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::TryInto;
use std::ffi::CStr;
use std::fmt::Display;
use std::io::Write;
use std::ptr::null_mut;
use std::{convert, iter, mem, ptr};
use zluda_llvm::core::*;
use zluda_llvm::prelude::*;
use zluda_llvm::zluda::*;
use zluda_llvm::*;

use crate::translate::{
    self, Arg4CarryOut, ConstType, ConversionKind, DenormSummary, ExpandedArgParams, FPDenormMode,
    MadCCDetails, MadCDetails, TranslationModule, TypeKind, TypeParts,
};
use crate::translate::{Id, IdGenerator};
use crate::{
    ast::{self, LinkingDirective},
    llvm,
    translate::Directive,
    TranslateError,
};

const LLVM_UNNAMED: *const i8 = b"\0".as_ptr() as *const i8;
const TEXREF_UNDERLYING: *const i8 = b"struct.textureReference\0".as_ptr() as *const i8;

struct EmitContext<'a> {
    context: &'a llvm::Context,
    module: &'a llvm::Module,
    builder: llvm::Builder,
    texref_underlying_type: LLVMTypeRef,
    constants: Constants,
    names: NamedIdGenerator,
    denorm_statistics: FxHashMap<Id, DenormSummary>,
    compilation_mode: CompilationMode,
}

impl<'a> EmitContext<'a> {
    fn new<'input>(
        context: &'a llvm::Context,
        module: &'a llvm::Module,
        id_gen: IdGenerator,
        id_defs: &FxHashMap<Id, Cow<'input, str>>,
        directive: &[Directive<'input>],
        denorm_statistics: FxHashMap<Id, DenormSummary>,
        compilation_mode: CompilationMode,
    ) -> Self {
        let builder = unsafe { llvm::Builder::create(context.get()) };
        let texref_underlying_type =
            unsafe { LLVMStructCreateNamed(context.get(), TEXREF_UNDERLYING) };
        EmitContext {
            context,
            module,
            builder,
            texref_underlying_type,
            constants: Constants::amdgpu(),
            names: NamedIdGenerator::new(id_gen, id_defs, directive),
            denorm_statistics,
            compilation_mode,
        }
    }
}

struct NamedIdGenerator {
    id_gen: IdGenerator,
    names: Vec<u8>,
    name_override: HashMap<Id, Vec<i8>>,
    value_refs: Vec<LLVMValueRef>,
}

impl NamedIdGenerator {
    fn new<'input>(
        id_gen: IdGenerator,
        id_defs: &FxHashMap<Id, Cow<'input, str>>,
        directives: &[Directive<'input>],
    ) -> Self {
        let mut names = Vec::new();
        for id in 1..(id_gen.next.get()) {
            write!(names, "{}\0", id).unwrap();
        }
        let name_override =
            id_defs
                .iter()
                .map(|(func_id, func_name)| (*func_id, clone_and_append_trailing_nul(&func_name)))
                .chain(directives.iter().filter_map(|directive| match directive {
                    Directive::Method(method) => method.source_name.as_ref().map(|source_name| {
                        (method.name, clone_and_append_trailing_nul(&*source_name))
                    }),
                    translate::TranslationDirective::Variable(_, Some(compiled_name), var) => {
                        Some((var.name, clone_and_append_trailing_nul(compiled_name)))
                    }
                    _ => None,
                }))
                .collect();
        let value_refs = vec![ptr::null_mut(); (id_gen.next.get() - 1) as usize];
        Self {
            names,
            id_gen,
            name_override,
            value_refs,
        }
    }

    fn name_ptr(&self, id: Id) -> *const i8 {
        self.name(id).as_ptr()
    }

    fn name<'a>(&'a self, id: Id) -> &'a [i8] {
        if let Some(name) = self.name_override.get(&id) {
            // Skip trailing NUL
            return &name[..name.len() - 1];
        }
        let id = id.get();
        let digits_lower = (id as f64).log10() as usize;
        let length_of_digits_below = 10usize.pow(digits_lower as u32) * digits_lower
            - ((10usize.pow(digits_lower as u32) + 1) / 9);
        let length_of_digits_above =
            (digits_lower + 1) * (id as usize - 10usize.pow(digits_lower as u32));
        let offset = length_of_digits_below + length_of_digits_above + id as usize - 1;
        unsafe {
            std::slice::from_raw_parts(self.names.as_ptr().add(offset) as _, digits_lower + 1)
        }
    }

    fn next(&mut self) -> Id {
        let result = self.id_gen.next();
        write!(self.names, "{}\0", result.get()).unwrap();
        self.value_refs.push(ptr::null_mut());
        result
    }

    fn register(&mut self, id: Id, val: LLVMValueRef) {
        self.value_refs[(id.get() - 1) as usize] = val;
    }

    fn register_result(
        &mut self,
        id: Id,
        func: impl FnOnce(*const i8) -> LLVMValueRef,
    ) -> LLVMValueRef {
        let name = self.name_ptr(id);
        let result = func(name);
        self.register(id, result);
        result
    }

    fn register_result_option(
        &mut self,
        id: Option<Id>,
        func: impl FnOnce(*const i8) -> LLVMValueRef,
    ) -> LLVMValueRef {
        if let Some(id) = id {
            self.register_result(id, func)
        } else {
            func(b"\0".as_ptr() as _)
        }
    }

    fn value(&self, id: Id) -> Result<LLVMValueRef, TranslateError> {
        self.try_value(id).ok_or_else(TranslateError::unreachable)
    }

    fn try_value(&self, id: Id) -> Option<LLVMValueRef> {
        let ptr = self.value_refs[(id.get() - 1) as usize];
        if ptr == null_mut() {
            None
        } else {
            Some(ptr)
        }
    }
}

fn clone_and_append_trailing_nul(s: &str) -> Vec<i8> {
    let mut string = s.as_bytes().to_owned();
    string.push(b'\0');
    let ptr = string.as_mut_ptr() as _;
    let len = string.len();
    let capacity = string.capacity();
    mem::forget(string);
    unsafe { Vec::from_raw_parts(ptr, len, capacity) }
}

struct Constants {
    data_layout: &'static [u8],
    target_triple: &'static [u8],
    generic_space: u32,
    global_space: u32,
    shared_space: u32,
    constant_space: u32,
    private_space: u32,
    kernel_callconv: LLVMCallConv,
    function_callconv: LLVMCallConv,
}

impl Constants {
    // https://llvm.org/docs/AMDGPUUsage.html
    fn amdgpu() -> Self {
        Self {
            data_layout: b"e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7\0",
            target_triple: b"amdgcn-amd-amdhsa\0",
            generic_space: 0,
            global_space: 1,
            shared_space: 3,
            constant_space: 4,
            private_space: 5,
            kernel_callconv: LLVMCallConv::LLVMAMDGPUKERNELCallConv,
            function_callconv: LLVMCallConv::LLVMCCallConv
        }
    }
}

pub(crate) unsafe fn emit_llvm_bitcode_and_linker_module<'input>(
    module: TranslationModule<'input, ExpandedArgParams>,
    denorm_statistics: FxHashMap<Id, DenormSummary>,
) -> Result<(llvm::Context, llvm::Module), TranslateError> {
    let context = llvm::Context::create();
    LLVMContextSetOpaquePointers(context.get(), 1);
    let llvm_module = llvm::Module::create(b"\0".as_ptr() as _, context.get());
    {
        let mut emit_ctx = EmitContext::new(
            &context,
            &llvm_module,
            module.id_defs.id_gen,
            &module.id_defs.globals.reverse_variables,
            &module.directives,
            denorm_statistics,
            module.compilation_mode,
        );
        emit_prelude(&mut emit_ctx);
        for d in module.directives {
            emit_directive(&mut emit_ctx, d)?;
        }
    }
    if cfg!(debug_assertions) {
        if let Some(error_msg) = llvm_module.verify() {
            LLVMDumpModule(llvm_module.get());
            return Err(TranslateError::l_l_v_m(error_msg));
        }
    }
    Ok((context, llvm_module))
}

pub(crate) fn emit_section(section_name: &str, metadata: &[u8], text_buffer: &mut Vec<u8>) {
    writeln!(text_buffer, ".section {}", section_name).ok();
    writeln!(text_buffer, ".p2align 3").ok();
    let mut chunks = metadata.chunks_exact(std::mem::size_of::<u64>());
    {
        if let Some(qword) = chunks.next() {
            let qword = u64::from_le_bytes(qword.try_into().unwrap());
            write!(text_buffer, ".quad {:#x}", qword).ok();
            for qword in &mut chunks {
                let qword = u64::from_le_bytes(qword.try_into().unwrap());
                write!(text_buffer, ",{:#x}", qword).ok();
            }
            writeln!(text_buffer, "").ok();
        }
    }
    let mut remainder = chunks.remainder().iter().copied();
    if let Some(byte) = remainder.next() {
        write!(text_buffer, ".byte {:#x}", byte).ok();
        for byte in remainder {
            write!(text_buffer, ",{:#x}", byte).ok();
        }
        writeln!(text_buffer, "").ok();
    }
}

unsafe fn emit_prelude(ctx: &mut EmitContext) {
    LLVMSetDataLayout(ctx.module.get(), ctx.constants.data_layout.as_ptr() as _);
    LLVMSetTarget(ctx.module.get(), ctx.constants.target_triple.as_ptr() as _);
}

unsafe fn emit_directive<'a, 'input>(
    ctx: &mut EmitContext<'a>,
    d: Directive<'input>,
) -> Result<(), TranslateError> {
    Ok(match d {
        Directive::Variable(linking, compiled_name, variable) => {
            emit_global_variable(ctx, linking, variable, compiled_name.is_some())?
        }
        Directive::Method(m) => emit_method(ctx, m)?,
    })
}

unsafe fn emit_global_variable(
    ctx: &mut EmitContext,
    linking: LinkingDirective,
    variable: translate::Variable,
    globally_visible: bool,
) -> Result<(), TranslateError> {
    let module = ctx.module.get();
    let llvm_type = get_llvm_type(ctx, &variable.type_)?;
    let address_space = get_llvm_address_space(&ctx.constants, variable.state_space)?;
    let value = match ctx.names.try_value(variable.name) {
        Some(value) => {
            if linking == ast::LinkingDirective::Extern {
                return Ok(());
            }
            value
        }
        None => ctx.names.register_result(variable.name, |mut name| {
            // HACK ALERT
            // For whatever reason if there's a global variable named __unnamed_1 AMDGPU falls on its face
            if CStr::from_ptr(name)
                .to_string_lossy()
                .starts_with("__unnamed_")
            {
                name = LLVM_UNNAMED;
            }
            // HACK ALERT
            // Autogenerated globals that hold string content, done to avoid name clash during linking
            if CStr::from_ptr(name).to_string_lossy().starts_with("$str") {
                name = LLVM_UNNAMED;
            }
            LLVMAddGlobalInAddressSpace(module, llvm_type, name, address_space)
        }),
    };
    emit_linkage_for_variable(variable.state_space, value, globally_visible)?;
    emit_init(
        ctx,
        value,
        linking,
        &variable.type_,
        variable.state_space,
        variable.initializer,
    )?;
    emit_alignment(value, variable.align);
    match variable.state_space {
        ast::StateSpace::Const | ast::StateSpace::Global | ast::StateSpace::Generic => {
            LLVMSetExternallyInitialized(value, 1);
        }
        ast::StateSpace::Reg
        | ast::StateSpace::Local
        | ast::StateSpace::Shared
        | ast::StateSpace::Param
        | ast::StateSpace::Sreg => {}
    }
    if variable.type_ == ast::Type::Texref || variable.type_ == ast::Type::Surfref {
        LLVMSetExternallyInitialized(value, 1);
        LLVMSetAlignment(value, mem::size_of::<usize>() as u32);
    }
    Ok(())
}

fn emit_alignment(value: LLVMValueRef, align: Option<u32>) {
    if let Some(align) = align {
        unsafe { LLVMSetAlignment(value, align) };
    }
}

unsafe fn emit_init(
    ctx: &mut EmitContext,
    value_ref: LLVMValueRef,
    linking: ast::LinkingDirective,
    type_: &ast::Type,
    space: ast::StateSpace,
    initializer: Option<ast::Initializer<Id>>,
) -> Result<(), TranslateError> {
    // .shared can't be const-initialized
    let const_value = if space == ast::StateSpace::Shared {
        if linking == ast::LinkingDirective::Extern {
            return Ok(());
        } else {
            LLVMGetUndef(get_llvm_type(ctx, type_)?)
        }
    } else {
        get_llvm_const(ctx, ConstType::Type(type_), initializer)?
    };
    LLVMSetInitializer(value_ref, const_value);
    Ok(())
}

unsafe fn get_llvm_const(
    ctx: &mut EmitContext,
    type_: ConstType,
    initalizer: Option<ast::Initializer<Id>>,
) -> Result<LLVMValueRef, TranslateError> {
    let const_value = match (type_, initalizer) {
        (ConstType::Type(type_), None) => LLVMConstNull(get_llvm_type(ctx, type_)?),
        (ConstType::ArraySubtype(type_, dimensions), None) => LLVMConstNull(get_llvm_array_type(
            get_llvm_type(ctx, &ast::Type::Scalar(type_))?,
            dimensions,
        )),
        (ConstType::Type(ast::Type::Scalar(scalar_type)), Some(ast::Initializer::Constant(x))) => {
            get_llvm_const_scalar(ctx, *scalar_type, x)?
        }
        (
            ConstType::Type(ast::Type::Array(scalar_type, dimensions)),
            Some(ast::Initializer::Array(array)),
        ) => get_llvm_const_array(ctx, *scalar_type, &dimensions, array)?,
        (
            ConstType::ArraySubtype(scalar_type, dimensions),
            Some(ast::Initializer::Array(array)),
        ) => get_llvm_const_array(ctx, scalar_type, dimensions, array)?,
        (type_, Some(ast::Initializer::Add(add))) => {
            let (init1, init2) = *add;
            let const1 = get_llvm_const(ctx, type_, Some(init1))?;
            let const2 = get_llvm_const(ctx, type_, Some(init2))?;
            LLVMConstAdd(const1, const2)
        }
        (_, Some(ast::Initializer::Global(id, type_))) => {
            let name = ctx.names.value(id)?;
            let b64 = get_llvm_type(ctx, &ast::Type::Scalar(ast::ScalarType::B64))?;
            let mut zero = LLVMConstInt(b64, 0, 0);
            let src_type = get_initializer_llvm_type(ctx, type_)?;
            let global_ptr = LLVMConstInBoundsGEP2(src_type, name, &mut zero, 1);
            LLVMConstPtrToInt(global_ptr, b64)
        }
        (_, Some(ast::Initializer::GenericGlobal(id, type_))) => {
            let name = ctx.names.value(id)?;
            let b64 = get_llvm_type(ctx, &ast::Type::Scalar(ast::ScalarType::B64))?;
            let mut zero = LLVMConstInt(b64, 0, 0);
            let src_type = get_initializer_llvm_type(ctx, type_)?;
            let global_ptr = LLVMConstInBoundsGEP2(src_type, name, &mut zero, 1);
            // void pointers are illegal in LLVM IR
            let b8 = get_llvm_type(ctx, &ast::Type::Scalar(ast::ScalarType::B8))?;
            let b8_generic_ptr = LLVMPointerType(
                b8,
                get_llvm_address_space(&ctx.constants, ast::StateSpace::Generic)?,
            );
            let generic_ptr = LLVMConstAddrSpaceCast(global_ptr, b8_generic_ptr);
            LLVMConstPtrToInt(generic_ptr, b64)
        }
        _ => return Err(TranslateError::todo()),
    };
    Ok(const_value)
}

fn get_initializer_llvm_type(
    ctx: &mut EmitContext,
    type_: ast::InitializerType,
) -> Result<LLVMTypeRef, TranslateError> {
    Ok(match type_ {
        ast::InitializerType::Unknown => return Err(TranslateError::unreachable()),
        ast::InitializerType::Value(type_) => get_llvm_type(ctx, &type_)?,
        ast::InitializerType::Function(return_args, input_args) => {
            let return_type = match &*return_args {
                [] => llvm::void_type(&ctx.context),
                [type_] => get_llvm_type(ctx, type_)?,
                [..] => get_llvm_type_struct(ctx, return_args.into_iter().map(Cow::Owned))?,
            };
            get_llvm_function_type(
                ctx,
                return_type,
                input_args.iter().map(|type_| (type_, ast::StateSpace::Reg)),
            )?
        }
    })
}

unsafe fn get_llvm_const_scalar(
    ctx: &mut EmitContext,
    scalar_type: ast::ScalarType,
    constant: ast::ImmediateValue,
) -> Result<LLVMValueRef, TranslateError> {
    let llvm_type = get_llvm_type(ctx, &ast::Type::Scalar(scalar_type))?;
    Ok(match scalar_type.kind() {
        ast::ScalarKind::Pred
        | ast::ScalarKind::Bit
        | ast::ScalarKind::Unsigned
        | ast::ScalarKind::Signed => LLVMConstInt(
            llvm_type,
            constant
                .as_u64()
                .ok_or_else(TranslateError::mismatched_type)?,
            0,
        ),
        ast::ScalarKind::Float => LLVMConstReal(
            llvm_type,
            constant
                .as_f64()
                .ok_or_else(TranslateError::mismatched_type)?,
        ),
        ast::ScalarKind::Float2 => return Err(TranslateError::todo()),
    })
}

unsafe fn get_llvm_const_array(
    ctx: &mut EmitContext,
    scalar_type: ast::ScalarType,
    dimensions: &[u32],
    initializer: Vec<ast::Initializer<Id>>,
) -> Result<LLVMValueRef, TranslateError> {
    let llvm_type: *mut LLVMType = get_llvm_type(ctx, &ast::Type::Scalar(scalar_type))?;
    Ok(match dimensions {
        [] => return Err(TranslateError::unreachable()),
        [dim, rest @ ..] => {
            let inner_array_type = if rest.len() == 0 {
                llvm_type
            } else {
                get_llvm_array_type(llvm_type, rest)
            };
            if initializer.len() != *dim as usize {
                return Err(TranslateError::unreachable());
            }
            let mut subinits = initializer
                .into_iter()
                .map(|inner_initalizer| {
                    if rest.len() == 0 {
                        get_llvm_const(
                            ctx,
                            ConstType::Type(&ast::Type::Scalar(scalar_type)),
                            Some(inner_initalizer),
                        )
                    } else {
                        get_llvm_const(
                            ctx,
                            ConstType::ArraySubtype(scalar_type, rest),
                            Some(inner_initalizer),
                        )
                    }
                })
                .collect::<Result<Vec<_>, _>>()?;
            LLVMConstArray(inner_array_type, subinits.as_mut_ptr(), *dim)
        }
    })
}

fn emit_function_variable(
    ctx: &mut EmitContext,
    variable: translate::Variable,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let llvm_type = get_llvm_type(ctx, &variable.type_)?;
    let addr_space = get_llvm_address_space(&ctx.constants, variable.state_space)?;
    let value = ctx.names.register_result(variable.name, |name| unsafe {
        LLVMZludaBuildAlloca(builder, llvm_type, addr_space, name)
    });
    match variable.initializer {
        None => {}
        Some(init) => {
            let constant =
                unsafe { get_llvm_const(ctx, ConstType::Type(&variable.type_), Some(init))? };
            unsafe { LLVMBuildStore(builder, constant, value) };
        }
    }
    // TODO: it shuld be possible to skip alignment of .param/.reg variables and get
    // minimally better assembly. See if this doesn't crash anything and measure impact
    emit_alignment(value, variable.align);
    Ok(())
}

fn emit_method<'a, 'input>(
    ctx: &mut EmitContext<'a>,
    method: crate::translate::Function<'input>,
) -> Result<(), TranslateError> {
    let is_kernel = method.is_kernel;
    let llvm_method = emit_method_declaration(ctx, &method)?;
    emit_linkage_for_method(&method, is_kernel, llvm_method);
    emit_tuning(ctx, llvm_method, &method.tuning);
    for statement in method.body.iter().flat_map(convert::identity) {
        register_basic_blocks(ctx, llvm_method, statement);
    }
    for statement in method.body.into_iter().flatten() {
        emit_statement(ctx, is_kernel, statement)?;
    }
    Ok(())
}

fn emit_tuning<'a>(
    ctx: &mut EmitContext<'a>,
    llvm_method: *mut LLVMValue,
    tunings: &[ast::TuningDirective],
) {
    for tuning in tunings {
        emit_tuning_single(ctx, llvm_method, *tuning);
    }
}

fn emit_tuning_single<'a>(
    ctx: &mut EmitContext<'a>,
    llvm_method: *mut LLVMValue,
    tuning: ast::TuningDirective,
) {
    match tuning {
        // TODO: measure
        ast::TuningDirective::MaxNReg(_) | ast::TuningDirective::MinNCtaPerSm(_) => {}
        ast::TuningDirective::MaxNtid(x, y, z) => {
            let size = x as u64 * y as u64 * z as u64;
            emit_llvm_string_attribute(
                ctx,
                llvm_method,
                b"amdgpu-flat-work-group-size",
                format!("1,{0}", size).as_bytes(),
            );
        }
        ast::TuningDirective::ReqNtid(x, y, z) => {
            let size = x as u64 * y as u64 * z as u64;
            emit_llvm_string_attribute(
                ctx,
                llvm_method,
                b"amdgpu-flat-work-group-size",
                format!("{0},{0}", size).as_bytes(),
            );
        }
        ast::TuningDirective::Noreturn => {
            let noreturn = b"noreturn";
            let attr_kind = unsafe {
                LLVMGetEnumAttributeKindForName(noreturn.as_ptr().cast(), noreturn.len())
            };
            if attr_kind == 0 {
                panic!();
            }
            let noreturn = unsafe { LLVMCreateEnumAttribute(ctx.context.get(), attr_kind, 0) };
            unsafe { LLVMAddAttributeAtIndex(llvm_method, LLVMAttributeFunctionIndex, noreturn) };
        }
    }
}

fn register_basic_blocks(
    ctx: &mut EmitContext,
    llvm_method: LLVMValueRef,
    statement: &crate::translate::ExpandedStatement,
) {
    match statement {
        crate::translate::Statement::Label(label) => {
            let context = ctx.context.get();
            ctx.names.register_result(*label, |label_name| unsafe {
                LLVMBasicBlockAsValue(LLVMAppendBasicBlockInContext(
                    context,
                    llvm_method,
                    label_name,
                ))
            });
        }
        _ => {}
    }
}

fn emit_statement(
    ctx: &mut EmitContext,
    is_kernel: bool,
    statement: crate::translate::ExpandedStatement,
) -> Result<(), TranslateError> {
    start_synthetic_basic_block_if_needed(ctx, &statement);
    Ok(match statement {
        crate::translate::Statement::Label(label) => emit_label(ctx, label)?,
        crate::translate::Statement::Variable(var) => emit_function_variable(ctx, var)?,
        crate::translate::Statement::Instruction(inst) => emit_instruction(ctx, is_kernel, &inst)?,
        crate::translate::Statement::Conditional(cond) => emit_conditional(ctx, &cond)?,
        crate::translate::Statement::Call(call) => emit_call(ctx, &call)?,
        crate::translate::Statement::LoadVar(load) => emit_load_var(ctx, &load)?,
        crate::translate::Statement::StoreVar(store) => emit_store_var(ctx, &store)?,
        crate::translate::Statement::Conversion(conv) => emit_implicit_conversion(ctx, &conv)?,
        crate::translate::Statement::Constant(constant) => emit_constant(ctx, &constant)?,
        crate::translate::Statement::RetValue(ret, ids) => emit_ret_value(ctx, &ret, &ids)?,
        crate::translate::Statement::PtrAccess(ptr_access) => emit_ptr_access(ctx, &ptr_access)?,
        crate::translate::Statement::RepackVector(repack) => emit_repack_vector(ctx, &repack)?,
        crate::translate::Statement::FunctionPointer(fnptr) => emit_function_pointer(ctx, &fnptr)?,
        crate::translate::Statement::MadC(MadCDetails { type_, is_hi, arg }) => {
            emit_inst_madc(ctx, type_, is_hi, &arg)?
        }
        crate::translate::Statement::MadCC(MadCCDetails { type_, arg }) => {
            emit_inst_madcc(ctx, type_, &arg)?
        }
        crate::translate::Statement::AddC(type_, arg) => emit_inst_add_c(ctx, type_, &arg)?,
        crate::translate::Statement::AddCC(type_, arg) => {
            emit_inst_addsub_cc(ctx, "add", type_, &arg)?
        }
        crate::translate::Statement::SubC(type_, arg) => emit_inst_sub_c(ctx, type_, &arg)?,
        crate::translate::Statement::SubCC(type_, arg) => {
            emit_inst_addsub_cc(ctx, "sub", type_, &arg)?
        }
        crate::translate::Statement::AsmVolatile { asm, constraints } => unsafe {
            emit_asm_volatile(ctx, asm, constraints)?
        },
    })
}

unsafe fn emit_asm_volatile(
    ctx: &mut EmitContext,
    asm: &str,
    constraints: &str,
) -> Result<(), TranslateError> {
    let func_type = get_llvm_function_type(ctx, llvm::void_type(ctx.context), iter::empty())?;
    let asm_call = LLVMGetInlineAsm(
        func_type,
        asm.as_ptr() as *mut _,
        asm.len(),
        constraints.as_ptr().cast::<i8>() as _,
        constraints.len(),
        1,
        0,
        LLVMInlineAsmDialect::LLVMInlineAsmDialectATT,
        0,
    );
    LLVMBuildCall2(
        ctx.builder.get(),
        func_type,
        asm_call,
        ptr::null_mut(),
        0,
        LLVM_UNNAMED,
    );
    Ok(())
}

fn emit_conditional(
    ctx: &mut EmitContext,
    cond: &crate::translate::BrachCondition,
) -> Result<(), TranslateError> {
    let predicate = ctx.names.value(cond.predicate)?;
    let if_true = unsafe { LLVMValueAsBasicBlock(ctx.names.value(cond.if_true)?) };
    let if_false = unsafe { LLVMValueAsBasicBlock(ctx.names.value(cond.if_false)?) };
    unsafe { LLVMBuildCondBr(ctx.builder.get(), predicate, if_true, if_false) };
    Ok(())
}

fn emit_repack_vector(
    ctx: &mut EmitContext,
    repack: &crate::translate::RepackVectorDetails,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let llvm_i32 = get_llvm_type(ctx, &ast::Type::Scalar(ast::ScalarType::U32))?;
    if repack.is_extract {
        let src = ctx.names.value(repack.packed)?;
        for (index, dst_id) in repack.unpacked.iter().enumerate() {
            let index_llvm = unsafe { LLVMConstInt(llvm_i32, index as _, 0) };
            ctx.names.register_result(*dst_id, |dst_name| unsafe {
                LLVMBuildExtractElement(builder, src, index_llvm, dst_name)
            });
        }
    } else {
        let vector_type = get_llvm_type(
            ctx,
            &ast::Type::Vector(repack.typ, repack.unpacked.len() as u8),
        )?;
        let mut temp_vec = unsafe { LLVMGetUndef(vector_type) };
        for (index, src_id) in repack.unpacked.iter().enumerate() {
            let dst = if index == repack.unpacked.len() - 1 {
                Some(repack.packed)
            } else {
                None
            };
            let src = ctx.names.value(*src_id)?;
            let index_llvm = unsafe { LLVMConstInt(llvm_i32, index as _, 0) };
            temp_vec = ctx.names.register_result_option(dst, |dst_name| unsafe {
                LLVMBuildInsertElement(builder, temp_vec, src, index_llvm, dst_name)
            });
        }
    }
    Ok(())
}

fn emit_function_pointer(
    ctx: &mut EmitContext,
    fnptr: &crate::translate::FunctionPointerDetails,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let src = ctx.names.value(fnptr.src)?;
    let type_ = get_llvm_type(ctx, &ast::Type::Scalar(ast::ScalarType::B64))?;
    ctx.names.register_result(fnptr.dst, |dst| unsafe {
        LLVMBuildPtrToInt(builder, src, type_, dst)
    });
    Ok(())
}

fn emit_ret_value(
    ctx: &mut EmitContext,
    _ret: &ast::RetData,
    ids: &[(Id, ast::Type)],
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    if ids.len() == 1 {
        let ret_value = ctx.names.value(ids[0].0)?;
        unsafe { LLVMBuildRet(builder, ret_value) };
        return Ok(());
    }
    let ret_type = get_llvm_type_struct(ctx, ids.iter().map(|(_, type_)| Cow::Borrowed(type_)))?;
    let mut ret_value = unsafe { LLVMGetUndef(ret_type) };
    for (idx, (id, _)) in ids.iter().enumerate() {
        let id = ctx.names.value(*id)?;
        ret_value =
            unsafe { LLVMBuildInsertValue(builder, ret_value, id, idx as u32, LLVM_UNNAMED) };
    }
    unsafe { LLVMBuildRet(builder, ret_value) };
    Ok(())
}

fn start_synthetic_basic_block_if_needed(
    ctx: &mut EmitContext,
    statement: &crate::translate::ExpandedStatement,
) {
    let current_block = unsafe { LLVMGetInsertBlock(ctx.builder.get()) };
    if current_block == ptr::null_mut() {
        return;
    }
    let terminator = unsafe { LLVMGetBasicBlockTerminator(current_block) };
    if terminator == ptr::null_mut() {
        return;
    }
    if let crate::translate::Statement::Label(..) = statement {
        return;
    }
    let new_block =
        unsafe { LLVMCreateBasicBlockInContext(ctx.context.get(), b"\0".as_ptr() as _) };
    unsafe { LLVMInsertExistingBasicBlockAfterInsertBlock(ctx.builder.get(), new_block) };
    unsafe { LLVMPositionBuilderAtEnd(ctx.builder.get(), new_block) };
}

fn emit_ptr_access(
    ctx: &mut EmitContext,
    ptr_access: &crate::translate::PtrAccess<crate::translate::ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let ptr_id = ctx.names.value(ptr_access.ptr_src)?;
    let temp1_id = ctx.names.next();
    let llvm_ptr_u8_type = get_llvm_pointer_type(
        ctx,
        &ast::Type::Scalar(ast::ScalarType::B8),
        ptr_access.state_space,
    )?;
    let llvm_u8_type = get_llvm_type(ctx, &ast::Type::Scalar(ast::ScalarType::B8))?;
    let underlying_ptr_llvm_type =
        get_llvm_pointer_type(ctx, &ptr_access.underlying_type, ptr_access.state_space)?;
    let temp1_value = ctx.names.register_result(temp1_id, |temp1_name| unsafe {
        LLVMBuildPointerCast(builder, ptr_id, llvm_ptr_u8_type, temp1_name)
    });
    let mut ptr_offset_id = ctx.names.value(ptr_access.offset_src)?;
    let temp2_id = ctx.names.next();
    let temp2_value = ctx.names.register_result(temp2_id, |temp2_name| unsafe {
        LLVMBuildInBoundsGEP2(
            builder,
            llvm_u8_type,
            temp1_value,
            &mut ptr_offset_id,
            1,
            temp2_name,
        )
    });
    ctx.names
        .register_result(ptr_access.dst, |dst_name| unsafe {
            LLVMBuildPointerCast(builder, temp2_value, underlying_ptr_llvm_type, dst_name)
        });
    Ok(())
}

fn emit_call(
    ctx: &mut EmitContext,
    call: &crate::translate::ResolvedCall<crate::translate::ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let mut args = call
        .input_arguments
        .iter()
        .map(|(id, _, _)| ctx.names.value(*id))
        .collect::<Result<Vec<_>, _>>()?;
    let (ret_value_name, return_type) = match &*call.return_arguments {
        [] => (LLVM_UNNAMED, llvm::void_type(&ctx.context)),
        [(single_return_id, type_, _)] => (
            ctx.names.name_ptr(*single_return_id),
            get_llvm_type(ctx, type_)?,
        ),
        [..] => (
            LLVM_UNNAMED,
            get_llvm_type_struct(
                ctx,
                call.return_arguments
                    .iter()
                    .map(|(_, type_, _)| Cow::Borrowed(type_)),
            )?,
        ),
    };
    let function_type = get_llvm_function_type(
        ctx,
        return_type,
        call.input_arguments
            .iter()
            .map(|(_, type_, state_space)| (type_, *state_space)),
    )?;
    let mut llvm_fn = ctx.names.value(call.name)?;
    if call.is_indirect {
        llvm_fn = unsafe {
            LLVMBuildIntToPtr(
                builder,
                llvm_fn,
                LLVMPointerType(function_type, ctx.constants.generic_space),
                LLVM_UNNAMED,
            )
        };
    }
    let call_result = unsafe {
        LLVMBuildCall2(
            builder,
            function_type,
            llvm_fn,
            args.as_mut_ptr(),
            args.len() as u32,
            ret_value_name,
        )
    };
    match &*call.return_arguments {
        [] => {}
        [(single_return_id, _, _)] => ctx.names.register(*single_return_id, call_result),
        many_return_args => {
            for (idx, (id, _, _)) in many_return_args.iter().enumerate() {
                ctx.names.register_result(*id, |id| unsafe {
                    LLVMBuildExtractValue(builder, call_result, idx as u32, id)
                });
            }
        }
    }
    Ok(())
}

fn emit_constant(
    ctx: &mut EmitContext,
    constant: &crate::translate::ConstantDefinition,
) -> Result<(), TranslateError> {
    let llvm_type = get_llvm_type(ctx, &ast::Type::Scalar(constant.typ))?;
    let dst_value = unsafe { emit_constant_value(llvm_type, constant.value) };
    ctx.names.register(constant.dst, dst_value);
    Ok(())
}

unsafe fn emit_constant_value(llvm_type: LLVMTypeRef, value: ast::ImmediateValue) -> LLVMValueRef {
    match value {
        ast::ImmediateValue::U64(x) => LLVMConstInt(llvm_type, x, 0),
        ast::ImmediateValue::S64(x) => LLVMConstInt(llvm_type, x as _, 0),
        ast::ImmediateValue::F32(x) => LLVMConstReal(llvm_type, x as f64),
        ast::ImmediateValue::F64(x) => LLVMConstReal(llvm_type, x),
    }
}

fn emit_implicit_conversion(
    ctx: &mut EmitContext,
    cv: &crate::translate::ImplicitConversion,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let from_parts = cv.from_type.to_parts();
    let to_parts = cv.to_type.to_parts();
    Ok(match (from_parts.kind, to_parts.kind, &cv.kind) {
        (_, _, &ConversionKind::BitToPtr) => {
            let dst_type = get_llvm_pointer_type(ctx, &cv.to_type, cv.to_space)?;
            let src = ctx.names.value(cv.src)?;
            ctx.names.register_result(cv.dst, |dst| unsafe {
                LLVMBuildIntToPtr(builder, src, dst_type, dst)
            });
        }
        (TypeKind::Scalar, TypeKind::Scalar, &ConversionKind::Default) => {
            if from_parts.width == to_parts.width {
                if from_parts.scalar_kind == to_parts.scalar_kind {
                    // It is noop, but another instruction expects result of this conversion
                    emit_value_copy(ctx, &cv.to_type, ctx.names.value(cv.src)?, cv.dst)?;
                } else {
                    let dst_type = get_llvm_type(ctx, &cv.to_type)?;
                    let src = ctx.names.value(cv.src)?;
                    ctx.names.register_result(cv.dst, |dst| unsafe {
                        LLVMBuildBitCast(builder, src, dst_type, dst)
                    });
                }
            } else {
                // This block is safe because it's illegal to implictly convert between floating point values
                let same_width_bit_type = get_llvm_type(
                    ctx,
                    &ast::Type::from_parts(TypeParts {
                        scalar_kind: ast::ScalarKind::Bit,
                        ..from_parts
                    }),
                )?;
                let src = ctx.names.value(cv.src)?;
                let temp_id = ctx.names.next();
                let same_width_bit_value =
                    ctx.names.register_result(temp_id, |temp_value| unsafe {
                        LLVMBuildBitCast(builder, src, same_width_bit_type, temp_value)
                    });
                let wide_bit_type = ast::Type::from_parts(TypeParts {
                    scalar_kind: ast::ScalarKind::Bit,
                    ..to_parts
                });
                let wide_bit_type_llvm = get_llvm_type(ctx, &wide_bit_type)?;
                if to_parts.scalar_kind == ast::ScalarKind::Unsigned
                    || to_parts.scalar_kind == ast::ScalarKind::Bit
                {
                    let conversion_fn = if to_parts.width > from_parts.width {
                        LLVMBuildZExt
                    } else {
                        LLVMBuildTruncOrBitCast
                    };
                    ctx.names.register_result(cv.dst, |dst| unsafe {
                        conversion_fn(builder, same_width_bit_value, wide_bit_type_llvm, dst)
                    });
                } else {
                    let conversion_fn = if to_parts.width > from_parts.width {
                        if from_parts.scalar_kind == ast::ScalarKind::Signed
                            && to_parts.scalar_kind == ast::ScalarKind::Signed
                        {
                            LLVMBuildSExt
                        } else {
                            LLVMBuildZExt
                        }
                    } else {
                        LLVMBuildTruncOrBitCast
                    };
                    let wide_bit_id = ctx.names.next();
                    ctx.names.register_result(wide_bit_id, |dst| unsafe {
                        conversion_fn(builder, same_width_bit_value, wide_bit_type_llvm, dst)
                    });
                    emit_implicit_conversion(
                        ctx,
                        &crate::translate::ImplicitConversion {
                            src: wide_bit_id,
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
            let result_type = get_llvm_type(ctx, &cv.to_type)?;
            let src = ctx.names.value(cv.src)?;
            ctx.names.register_result(cv.dst, |dst| unsafe {
                LLVMBuildSExtOrBitCast(builder, src, result_type, dst)
            });
        }
        (TypeKind::Vector, TypeKind::Scalar, &ConversionKind::Default)
        | (TypeKind::Scalar, TypeKind::Vector, &ConversionKind::Default)
        | (TypeKind::Scalar, TypeKind::Array, &ConversionKind::Default)
        | (TypeKind::Array, TypeKind::Scalar, &ConversionKind::Default) => {
            let result_type = get_llvm_type(ctx, &cv.to_type)?;
            let src = ctx.names.value(cv.src)?;
            ctx.names.register_result(cv.dst, |dst| unsafe {
                LLVMBuildBitCast(builder, src, result_type, dst)
            });
        }
        (_, _, &ConversionKind::PtrToPtr) => {
            let result_type = get_llvm_pointer_type(ctx, &cv.to_type, cv.to_space)?;
            if cv.to_space == ast::StateSpace::Generic && cv.from_space != ast::StateSpace::Generic
            {
                let src = if cv.from_type != cv.to_type {
                    let pointer_from_type = get_llvm_pointer_type(ctx, &cv.to_type, cv.from_space)?;
                    let src = ctx.names.value(cv.src)?;
                    let temp_id = ctx.names.next();
                    ctx.names.register_result(temp_id, |dst| unsafe {
                        LLVMBuildBitCast(builder, src, pointer_from_type, dst)
                    })
                } else {
                    ctx.names.value(cv.src)?
                };
                ctx.names.register_result(cv.dst, |dst| unsafe {
                    LLVMBuildAddrSpaceCast(builder, src, result_type, dst)
                });
            } else if cv.from_space == ast::StateSpace::Generic
                && cv.to_space != ast::StateSpace::Generic
            {
                let src = if cv.from_type != cv.to_type {
                    let temp_type = get_llvm_pointer_type(ctx, &cv.to_type, cv.from_space)?;
                    let src = ctx.names.value(cv.src)?;
                    let temp_id = ctx.names.next();
                    ctx.names.register_result(temp_id, |dst| unsafe {
                        LLVMBuildBitCast(builder, src, temp_type, dst)
                    })
                } else {
                    ctx.names.value(cv.src)?
                };
                ctx.names.register_result(cv.dst, |dst| unsafe {
                    LLVMBuildAddrSpaceCast(builder, src, result_type, dst)
                });
            } else {
                let src = ctx.names.value(cv.src)?;
                ctx.names.register_result(cv.dst, |dst| unsafe {
                    LLVMBuildBitCast(builder, src, result_type, dst)
                });
            }
        }
        (_, _, &ConversionKind::AddressOf) => {
            let dst_type = get_llvm_type(ctx, &cv.to_type)?;
            let src = ctx.names.value(cv.src)?;
            ctx.names.register_result(cv.dst, |dst| unsafe {
                LLVMBuildPtrToInt(builder, src, dst_type, dst)
            });
        }
        (TypeKind::Pointer, TypeKind::Scalar, &ConversionKind::Default) => {
            let result_type = get_llvm_type(ctx, &cv.to_type)?;
            let src = ctx.names.value(cv.src)?;
            ctx.names.register_result(cv.dst, |dst| unsafe {
                LLVMBuildPtrToInt(builder, src, result_type, dst)
            });
        }
        (TypeKind::Scalar, TypeKind::Pointer, &ConversionKind::Default) => {
            let result_type = get_llvm_type(ctx, &cv.to_type)?;
            let src = ctx.names.value(cv.src)?;
            ctx.names.register_result(cv.dst, |dst| unsafe {
                LLVMBuildIntToPtr(builder, src, result_type, dst)
            });
        }
        _ => unreachable!(),
    })
}

fn emit_value_copy(
    ctx: &mut EmitContext,
    type_: &ast::Type,
    src: LLVMValueRef,
    dst: Id,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let type_ = get_llvm_type(ctx, type_)?;
    let temp_value = unsafe { LLVMBuildAlloca(builder, type_, LLVM_UNNAMED) };
    unsafe { LLVMBuildStore(builder, src, temp_value) };
    ctx.names.register_result(dst, |dst| unsafe {
        LLVMBuildLoad2(builder, type_, temp_value, dst)
    });
    Ok(())
}

fn emit_instruction(
    ctx: &mut EmitContext,
    is_kernel: bool,
    inst: &ast::Instruction<crate::translate::ExpandedArgParams>,
) -> Result<(), TranslateError> {
    Ok(match inst {
        ast::Instruction::Ld(details, args) => emit_inst_ld(ctx, details, args)?,
        ast::Instruction::Mov(details, args) => emit_inst_mov(ctx, details, args)?,
        ast::Instruction::Mul(details, args) => emit_inst_mul(ctx, details, args)?,
        ast::Instruction::Add(details, args) => emit_inst_add(ctx, details, args)?,
        ast::Instruction::Setp(details, args) => emit_inst_setp(ctx, details, args, None)?,
        ast::Instruction::SetpBool(details, args) => emit_inst_setp_bool(ctx, details, args)?,
        ast::Instruction::Not(type_, args) => emit_inst_not(ctx, *type_, args)?,
        ast::Instruction::Bra(_, args) => emit_inst_bra(ctx, args)?,
        ast::Instruction::Cvt(details, args) => emit_inst_cvt(ctx, details, args)?,
        ast::Instruction::Cvta(details, args) => emit_inst_cvta(ctx, details, args)?,
        ast::Instruction::Shl(type_, args) => emit_inst_shl(ctx, *type_, args)?,
        ast::Instruction::Shr(type_, args) => emit_inst_shr(ctx, *type_, args)?,
        ast::Instruction::St(details, args) => emit_inst_st(ctx, details, args)?,
        ast::Instruction::Exit => emit_inst_exit(ctx, is_kernel)?,
        ast::Instruction::Ret(_) => emit_inst_ret(ctx),
        ast::Instruction::Abs(details, args) => emit_inst_abs(ctx, details, args)?,
        ast::Instruction::Mad(details, args) => emit_inst_mad(ctx, details, args)?,
        ast::Instruction::Fma(details, args) => emit_inst_fma(ctx, details, args)?,
        ast::Instruction::Or(_, args) => emit_inst_or(ctx, args)?,
        ast::Instruction::Sub(details, args) => emit_inst_sub(ctx, details, args)?,
        ast::Instruction::Min(details, args) => emit_inst_min(ctx, details, args)?,
        ast::Instruction::Max(details, args) => emit_inst_max(ctx, details, args)?,
        ast::Instruction::Rcp(details, args) => emit_inst_rcp(ctx, details, args)?,
        ast::Instruction::And(_, args) => emit_inst_and(ctx, args)?,
        ast::Instruction::Selp(_, args) => emit_inst_selp(ctx, args)?,
        ast::Instruction::Atom(details, args) => emit_inst_atom(ctx, details, args)?,
        ast::Instruction::AtomCas(details, args) => emit_inst_atom_cas(ctx, details, args)?,
        ast::Instruction::Div(details, args) => emit_inst_div(ctx, details, args)?,
        ast::Instruction::Sqrt(details, args) => emit_inst_sqrt(ctx, details, args)?,
        ast::Instruction::Rsqrt(details, args) => emit_inst_rsqrt(ctx, details, args)?,
        ast::Instruction::Neg(details, args) => emit_inst_neg(ctx, details, args)?,
        ast::Instruction::Sin { arg, .. } => emit_inst_sin(ctx, arg)?,
        ast::Instruction::Cos { arg, .. } => emit_inst_cos(ctx, arg)?,
        ast::Instruction::Lg2 { arg, .. } => emit_inst_lg2(ctx, arg)?,
        ast::Instruction::Ex2 { arg, .. } => emit_inst_ex2(ctx, arg)?,
        ast::Instruction::Clz { typ, arg } => emit_inst_clz(ctx, *typ, arg)?,
        ast::Instruction::Bfind(details, arg) => emit_inst_bfind(ctx, details, arg)?,
        ast::Instruction::Brev { typ, arg } => emit_inst_brev(ctx, *typ, arg)?,
        ast::Instruction::Popc { typ, arg } => emit_inst_popc(ctx, *typ, arg)?,
        ast::Instruction::Xor { arg, .. } => emit_inst_xor(ctx, arg)?,
        ast::Instruction::Rem { typ, arg } => emit_inst_rem(ctx, *typ, arg)?,
        ast::Instruction::Prmt { control, arg } => emit_inst_prmt(ctx, *control, arg)?,
        ast::Instruction::PrmtSlow { .. } => return Err(TranslateError::unexpected_pattern()),
        ast::Instruction::Membar { level } => emit_inst_membar(ctx, *level),
        ast::Instruction::Shf(details, args) => emit_inst_shf(ctx, details, args)?,
        ast::Instruction::Trap => emit_int_trap(ctx)?,
        ast::Instruction::Brkpt => emit_int_brkpt(ctx)?,
        ast::Instruction::BarWarp(..) => emit_inst_bar_warp(ctx)?,
        ast::Instruction::Vshr(arg) => emit_inst_vshr(ctx, arg)?,
        ast::Instruction::Set(details, arg) => emit_inst_set(ctx, details, arg)?,
        ast::Instruction::Red(details, arg) => emit_inst_red(ctx, details, arg)?,
        ast::Instruction::Isspacep(space, arg) => emit_inst_isspacep(ctx, *space, arg)?,
        // replaced by function calls or Statement variants
        ast::Instruction::Activemask { .. }
        | ast::Instruction::Bar(..)
        | ast::Instruction::BarRed(..)
        | ast::Instruction::Bfe { .. }
        | ast::Instruction::Bfi { .. }
        | ast::Instruction::MadC { .. }
        | ast::Instruction::MadCC { .. }
        | ast::Instruction::AddC { .. }
        | ast::Instruction::AddCC { .. }
        | ast::Instruction::SubC { .. }
        | ast::Instruction::SubCC { .. }
        | ast::Instruction::Tex(..)
        | ast::Instruction::Suld(..)
        | ast::Instruction::Sust(..)
        | ast::Instruction::Call(_)
        | ast::Instruction::Vote { .. }
        | ast::Instruction::Shfl(..)
        | ast::Instruction::Dp4a(..)
        | ast::Instruction::Nanosleep(..)
        | ast::Instruction::MatchAny(..) => return Err(TranslateError::unreachable()),
    })
}

fn emit_inst_isspacep(
    ctx: &mut EmitContext,
    space: ast::StateSpace,
    arg: &ast::Arg2<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    match space {
        ast::StateSpace::Local => {
            emit_inst_isspacep_impl(ctx, Some(arg.dst), arg.src, b"llvm.amdgcn.is.private\0")?;
            Ok(())
        }
        ast::StateSpace::Shared => {
            emit_inst_isspacep_impl(ctx, Some(arg.dst), arg.src, b"llvm.amdgcn.is.shared\0")?;
            Ok(())
        }
        ast::StateSpace::Global => {
            let builder = ctx.builder.get();
            let is_private =
                emit_inst_isspacep_impl(ctx, None, arg.src, b"llvm.amdgcn.is.private\0")?;
            let is_shared =
                emit_inst_isspacep_impl(ctx, None, arg.src, b"llvm.amdgcn.is.shared\0")?;
            let private_or_shared =
                unsafe { LLVMBuildOr(builder, is_private, is_shared, LLVM_UNNAMED) };
            let i1_true = unsafe {
                LLVMConstInt(
                    get_llvm_type(ctx, &ast::Type::Scalar(ast::ScalarType::Pred))?,
                    1,
                    0,
                )
            };
            ctx.names.register_result(arg.dst, |dst| unsafe {
                // I'd rathr user LLVMBuildNeg(...), but when using LLVMBuildNeg(...) in LLVM 15,
                // LLVM emits this broken IR:
                //      %"14" = sub i1 false, %4
                LLVMBuildSub(builder, i1_true, private_or_shared, dst)
            });
            Ok(())
        }
        _ => Err(TranslateError::unreachable()),
    }
}

fn emit_inst_isspacep_impl(
    ctx: &mut EmitContext,
    dst: Option<Id>,
    src: Id,
    intrinsic: &[u8],
) -> Result<LLVMValueRef, TranslateError> {
    let src = ctx.names.value(src)?;
    let b8 = get_llvm_type(ctx, &ast::Type::Scalar(ast::ScalarType::B8))?;
    let b8_generic_ptr = unsafe {
        LLVMPointerType(
            b8,
            get_llvm_address_space(&ctx.constants, ast::StateSpace::Generic)?,
        )
    };
    let src = unsafe { LLVMBuildIntToPtr(ctx.builder.get(), src, b8_generic_ptr, LLVM_UNNAMED) };
    emit_intrinsic_arg2(
        ctx,
        (ast::ScalarType::Pred, dst),
        (ast::ScalarType::B8, ast::StateSpace::Generic, src),
        intrinsic,
    )
}

fn emit_inst_red(
    ctx: &mut EmitContext,
    details: &ast::AtomDetails,
    arg: &ast::Arg2St<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    emit_inst_atom_impl(ctx, details, None, arg.src1, arg.src2)
}

fn emit_inst_set(
    ctx: &mut EmitContext,
    details: &ast::SetData,
    arg: &ast::Arg3<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let temp_result = emit_inst_setp_float(ctx, details.cmp_op, None, arg.src1, arg.src2)?;
    if details.src_type != ast::ScalarType::F16x2 {
        return Err(TranslateError::todo());
    }
    if details.dst_type.is_integer() && details.dst_type.size_of() == mem::size_of::<u32>() as u8 {
        let b16vec2_type = get_llvm_type(ctx, &ast::Type::Vector(ast::ScalarType::B16, 2))?;
        let b16vec2_result =
            unsafe { LLVMBuildSExt(builder, temp_result, b16vec2_type, LLVM_UNNAMED) };

        let u32_type = get_llvm_type(ctx, &ast::Type::Scalar(ast::ScalarType::U32))?;
        ctx.names.register_result(arg.dst, |dst_name| unsafe {
            LLVMBuildBitCast(builder, b16vec2_result, u32_type, dst_name)
        });
    } else if matches!(details.dst_type, ast::ScalarType::F16x2) {
        let f16x2_type = get_llvm_type(ctx, &ast::Type::Scalar(ast::ScalarType::F16x2))?;
        ctx.names.register_result(arg.dst, |dst_name| unsafe {
            LLVMBuildUIToFP(builder, temp_result, f16x2_type, dst_name)
        });
    } else {
        return Err(TranslateError::todo());
    }
    Ok(())
}

fn emit_inst_bfind(
    ctx: &mut EmitContext,
    details: &ast::BfindDetails,
    arg: &ast::Arg2<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let src = arg.src.get_llvm_value(&mut ctx.names)?;
    let llvm_dst_type = get_llvm_type(ctx, &ast::Type::Scalar(ast::ScalarType::U32))?;
    let const_0 = unsafe { LLVMConstInt(llvm_dst_type, 0, 0) };
    let const_int_max = unsafe { LLVMConstInt(llvm_dst_type, u64::MAX, 0) };
    let is_zero = unsafe {
        LLVMBuildICmp(
            builder,
            LLVMIntPredicate::LLVMIntEQ,
            src,
            const_0,
            LLVM_UNNAMED,
        )
    };
    let mut clz_result = emit_inst_clz_impl(ctx, ast::ScalarType::U32, None, arg.src, true)?;
    if !details.shift {
        let bits = unsafe {
            LLVMConstInt(
                llvm_dst_type,
                (details.type_.size_of() as u32 * u8::BITS) as u64 - 1,
                0,
            )
        };
        clz_result = unsafe { LLVMBuildSub(builder, bits, clz_result, LLVM_UNNAMED) };
    }
    ctx.names.register_result(arg.dst, |dst_name| unsafe {
        LLVMBuildSelect(builder, is_zero, const_int_max, clz_result, dst_name)
    });
    Ok(())
}

fn emit_inst_vshr(
    ctx: &mut EmitContext,
    arg: &ast::Arg4<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let shift_result = emit_inst_shift_impl(
        ctx,
        ast::ScalarType::U32,
        None,
        arg.src1,
        arg.src2,
        LLVMBuildLShr,
        0,
    )?;
    let src3 = arg.src3.get_llvm_value(&mut ctx.names)?;
    ctx.names.register_result(arg.dst, |name| unsafe {
        LLVMBuildAdd(builder, shift_result, src3, name)
    });
    Ok(())
}

fn emit_int_brkpt(ctx: &mut EmitContext) -> Result<(), TranslateError> {
    emit_intrinsic_arg0(ctx, b"llvm.debugtrap\0")?;
    Ok(())
}

fn emit_inst_bar_warp(ctx: &mut EmitContext) -> Result<(), TranslateError> {
    emit_intrinsic_arg0(ctx, b"llvm.amdgcn.wave.barrier\0")?;
    Ok(())
}

fn emit_int_trap(ctx: &mut EmitContext) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let function_type = get_llvm_function_type(ctx, llvm::void_type(&ctx.context), iter::empty())?;
    let mut function_value =
        unsafe { LLVMGetNamedFunction(ctx.module.get(), "llvm.trap\0".as_ptr().cast()) };
    if function_value == ptr::null_mut() {
        function_value = unsafe {
            LLVMAddFunction(
                ctx.module.get(),
                "llvm.trap\0".as_ptr().cast(),
                function_type,
            )
        };
    }
    unsafe {
        LLVMBuildCall2(
            builder,
            function_type,
            function_value,
            ptr::null_mut(),
            0,
            LLVM_UNNAMED,
        );
        // llvm.trap is not a terminator,
        // LLVM might fail with an unterminated basic block if we don't insert unreachable
        LLVMBuildUnreachable(builder);
    }
    Ok(())
}

fn emit_inst_exit(ctx: &mut EmitContext, is_kernel: bool) -> Result<(), TranslateError> {
    if !is_kernel {
        return Err(TranslateError::todo());
    }
    Ok(emit_inst_ret(ctx))
}

fn emit_inst_shf(
    ctx: &mut EmitContext,
    details: &ast::FunnelShift,
    args: &ast::Arg4<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    match details.mode {
        ast::ShiftNormalization::Clamp => return Err(TranslateError::unreachable()),
        ast::ShiftNormalization::Wrap => {}
    }
    let op_name = match details.direction {
        ast::FunnelDirection::Left => "fshl",
        ast::FunnelDirection::Right => "fshr",
    };
    let intrinsic_name = format!("llvm.{}.i32\0", op_name);
    let builder = ctx.builder.get();
    let llvm_type = get_llvm_type(ctx, &ast::Type::Scalar(ast::ScalarType::B32))?;
    let function_type = get_llvm_function_type(
        ctx,
        llvm_type,
        [
            (
                &ast::Type::Scalar(ast::ScalarType::B32),
                ast::StateSpace::Reg,
            ),
            (
                &ast::Type::Scalar(ast::ScalarType::B32),
                ast::StateSpace::Reg,
            ),
            (
                &ast::Type::Scalar(ast::ScalarType::B32),
                ast::StateSpace::Reg,
            ),
        ]
        .iter()
        .copied(),
    )?;
    let mut function_value =
        unsafe { LLVMGetNamedFunction(ctx.module.get(), intrinsic_name.as_ptr() as _) };
    if function_value == ptr::null_mut() {
        function_value = unsafe {
            LLVMAddFunction(
                ctx.module.get(),
                intrinsic_name.as_ptr() as _,
                function_type,
            )
        };
    }
    let src1 = args.src1.get_llvm_value(&mut ctx.names)?;
    let src2 = args.src2.get_llvm_value(&mut ctx.names)?;
    let src3 = args.src3.get_llvm_value(&mut ctx.names)?;
    let mut llvm_args = [src2, src1, src3];
    ctx.names
        .register_result_option(Some(args.dst), |dst_name| unsafe {
            LLVMBuildCall2(
                builder,
                function_type,
                function_value,
                llvm_args.as_mut_ptr(),
                3,
                dst_name,
            )
        });
    Ok(())
}

fn emit_inst_setp_bool(
    ctx: &mut EmitContext,
    details: &ast::SetpBoolData,
    args: &ast::Arg5Setp<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let base_args = ast::Arg4Setp {
        dst1: args.dst1,
        dst2: args.dst2,
        src1: args.src1,
        src2: args.src2,
    };
    emit_inst_setp(
        ctx,
        &details.base,
        &base_args,
        Some((details.negate_src3, args.src3, details.bool_op)),
    )
}

fn emit_inst_abs(
    ctx: &mut EmitContext,
    details: &ast::AbsDetails,
    args: &ast::Arg2<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    if details.typ.kind() == ast::ScalarKind::Float {
        let intrinsic_name = format!("llvm.fabs.{}\0", details.typ.llvm_display());
        emit_intrinsic_arg2(
            ctx,
            (details.typ, Some(args.dst)),
            (details.typ, ast::StateSpace::Reg, args.src),
            intrinsic_name.as_bytes(),
        )?;
    } else {
        let intrinsic_name = format!("llvm.abs.{}\0", details.typ.llvm_display());
        let const_0 = unsafe {
            LLVMConstInt(
                get_llvm_type(ctx, &ast::Type::Scalar(ast::ScalarType::Pred))?,
                0,
                0,
            )
        };
        emit_intrinsic_arg3(
            ctx,
            (
                get_llvm_type(ctx, &ast::Type::Scalar(details.typ))?,
                Some(args.dst),
            ),
            (details.typ, args.src),
            (ast::ScalarType::Pred, const_0),
            intrinsic_name.as_bytes(),
        )?;
    }
    Ok(())
}

fn emit_inst_xor(
    ctx: &mut EmitContext,
    args: &ast::Arg3<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let src1 = ctx.names.value(args.src1)?;
    let src2 = ctx.names.value(args.src2)?;
    ctx.names.register_result(args.dst, |dst_name| unsafe {
        LLVMBuildXor(builder, src1, src2, dst_name)
    });
    Ok(())
}

fn emit_inst_shl(
    ctx: &mut EmitContext,
    type_: ast::ScalarType,
    args: &ast::Arg3<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    emit_inst_shift_impl(
        ctx,
        type_,
        Some(args.dst),
        args.src1,
        args.src2,
        LLVMBuildShl,
        0u64,
    )?;
    Ok(())
}

fn emit_inst_shr(
    ctx: &mut EmitContext,
    type_: ast::ScalarType,
    args: &ast::Arg3<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let (shift_fn, zero) = if type_.kind() == ast::ScalarKind::Signed {
        (
            LLVMBuildAShr
                as unsafe extern "C" fn(
                    LLVMBuilderRef,
                    LLVMValueRef,
                    LLVMValueRef,
                    *const i8,
                ) -> LLVMValueRef,
            u64::MAX,
        )
    } else {
        (
            LLVMBuildLShr
                as unsafe extern "C" fn(
                    LLVMBuilderRef,
                    LLVMValueRef,
                    LLVMValueRef,
                    *const i8,
                ) -> LLVMValueRef,
            0u64,
        )
    };
    emit_inst_shift_impl(
        ctx,
        type_,
        Some(args.dst),
        args.src1,
        args.src2,
        shift_fn,
        zero,
    )?;
    Ok(())
}

// I would do it as as function in zluda_ptx_impl, but C++ hates 16 bit shifts
fn emit_inst_shift_impl(
    ctx: &mut EmitContext,
    type_: ast::ScalarType,
    dst: Option<Id>,
    src1: Id,
    src2: Id,
    shift_fn: unsafe extern "C" fn(
        LLVMBuilderRef,
        LLVMValueRef,
        LLVMValueRef,
        *const i8,
    ) -> LLVMValueRef,
    zero: u64,
) -> Result<LLVMValueRef, TranslateError> {
    let builder = ctx.builder.get();
    let llvm_type = get_llvm_type(ctx, &ast::Type::Scalar(type_))?;
    let src = ctx.names.value(src1)?;
    let mut shift_amount = ctx.names.value(src2)?;
    if type_.size_of() == 8 {
        shift_amount = unsafe { LLVMBuildZExt(builder, shift_amount, llvm_type, LLVM_UNNAMED) };
    } else if type_.size_of() == 2 {
        shift_amount = unsafe { LLVMBuildTrunc(builder, shift_amount, llvm_type, LLVM_UNNAMED) };
    };
    let max_shift_const: *mut LLVMValue =
        unsafe { LLVMConstInt(llvm_type, (type_.size_of() as u64 * 8) - 1, 0) };
    let is_over_max_shift = unsafe {
        LLVMBuildICmp(
            builder,
            LLVMIntPredicate::LLVMIntUGT,
            shift_amount,
            max_shift_const,
            LLVM_UNNAMED,
        )
    };
    let const_0 = unsafe { LLVMConstInt(llvm_type, zero, 0) };
    let shift_result = unsafe { shift_fn(builder, src, shift_amount, LLVM_UNNAMED) };
    Ok(ctx.names.register_result_option(dst, |dst_name| unsafe {
        LLVMBuildSelect(builder, is_over_max_shift, const_0, shift_result, dst_name)
    }))
}

fn emit_inst_selp(
    ctx: &mut EmitContext,
    args: &ast::Arg4<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let src1 = ctx.names.value(args.src1)?;
    let src2 = ctx.names.value(args.src2)?;
    let src3 = ctx.names.value(args.src3)?;
    ctx.names.register_result(args.dst, |dst_name| unsafe {
        LLVMBuildSelect(builder, src3, src1, src2, dst_name)
    });
    Ok(())
}

fn emit_inst_rsqrt(
    ctx: &mut EmitContext,
    details: &ast::RsqrtDetails,
    args: &ast::Arg2<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let type_llvm = get_llvm_type(ctx, &ast::Type::Scalar(details.typ))?;
    let const_1 = unsafe { LLVMConstReal(type_llvm, 1.0) };
    let intrinsic_fn = match details.typ {
        ast::ScalarType::F32 => &b"llvm.sqrt.f32\0"[..],
        ast::ScalarType::F64 => b"llvm.sqrt.f64\0",
        _ => return Err(TranslateError::unreachable()),
    };
    let sqrt_result = emit_intrinsic_arg2(
        ctx,
        (details.typ, None),
        (details.typ, ast::StateSpace::Reg, args.src),
        intrinsic_fn,
    )?;
    unsafe { LLVMZludaSetFastMathFlags(sqrt_result, FastMathFlags::ApproxFunc) };
    let result = ctx.names.register_result(args.dst, |dst_name| unsafe {
        LLVMBuildFDiv(builder, const_1, sqrt_result, dst_name)
    });
    unsafe {
        LLVMZludaSetFastMathFlags(
            result,
            FastMathFlags::ApproxFunc | FastMathFlags::AllowReciprocal,
        )
    };
    Ok(())
}

fn emit_inst_rem(
    ctx: &mut EmitContext,
    type_: ast::ScalarType,
    args: &ast::Arg3<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let build_fn = if type_.kind() == ast::ScalarKind::Signed {
        LLVMBuildSRem
    } else {
        LLVMBuildURem
    };
    let src1 = ctx.names.value(args.src1)?;
    let src2 = ctx.names.value(args.src2)?;
    ctx.names.register_result(args.dst, |dst_name| unsafe {
        build_fn(builder, src1, src2, dst_name)
    });
    Ok(())
}

fn emit_inst_sqrt(
    ctx: &mut EmitContext,
    details: &ast::RcpSqrtDetails,
    args: &ast::Arg2<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let (intrinsic_fn, fast_math) = match (details.type_, details.kind) {
        (ast::ScalarType::F32, ast::RcpSqrtKind::Approx) => {
            (&b"llvm.sqrt.f32\0"[..], FastMathFlags::ApproxFunc)
        }
        (ast::ScalarType::F64, ast::RcpSqrtKind::Approx) => {
            (&b"llvm.sqrt.f64\0"[..], FastMathFlags::ApproxFunc)
        }
        // TODO: Go back to llvm.sqrt.f64 when this commit lands:
        //       https://github.com/RadeonOpenCompute/llvm-project/commit/e3fd8f83a801b1918508c7c0a71cc31bc95ad4d2
        //       It's not yet present as of ROCm 5.7.1
        // TODO: support correct rounding
        (ast::ScalarType::F32, _) => (&b"__ocml_sqrt_f32\0"[..], FastMathFlags::empty()),
        (ast::ScalarType::F64, _) => (&b"__ocml_sqrt_f64\0"[..], FastMathFlags::empty()),
        _ => return Err(TranslateError::unreachable()),
    };
    let sqrt_result = emit_intrinsic_arg2(
        ctx,
        (details.type_, Some(args.dst)),
        (details.type_, ast::StateSpace::Reg, args.src),
        intrinsic_fn,
    )?;
    unsafe { LLVMZludaSetFastMathFlags(sqrt_result, fast_math) };
    Ok(())
}

fn emit_inst_rcp(
    ctx: &mut EmitContext,
    details: &ast::RcpSqrtDetails,
    args: &ast::Arg2<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let type_llvm = get_llvm_type(ctx, &ast::Type::Scalar(details.type_))?;
    let const_1 = unsafe { LLVMConstReal(type_llvm, 1.0) };
    let src = ctx.names.value(args.src)?;
    let value = ctx.names.register_result(args.dst, |dst_name| unsafe {
        LLVMBuildFDiv(builder, const_1, src, dst_name)
    });
    let fast_math = match details.kind {
        ast::RcpSqrtKind::Approx => FastMathFlags::AllowReciprocal | FastMathFlags::ApproxFunc,
        ast::RcpSqrtKind::Rounding(ast::RoundingMode::NearestEven) => {
            FastMathFlags::AllowReciprocal
        }
        // TODO: implement this correctly
        ast::RcpSqrtKind::Rounding(ast::RoundingMode::PositiveInf) => {
            FastMathFlags::AllowReciprocal
        }
        // TODO: fix this
        ast::RcpSqrtKind::Rounding(_) => FastMathFlags::AllowReciprocal,
    };
    unsafe { LLVMZludaSetFastMathFlags(value, fast_math) };
    Ok(())
}

fn emit_inst_prmt(
    ctx: &mut EmitContext,
    control: u16,
    arg: &ast::Arg3<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let components = [
        ((control >> 0) & 0b0111) as u32,
        ((control >> 4) & 0b0111) as u32,
        ((control >> 8) & 0b0111) as u32,
        ((control >> 12) & 0b0111) as u32,
    ];
    let sext_components = [
        ((control >> 0) & 0b1000) != 0,
        ((control >> 4) & 0b1000) != 0,
        ((control >> 8) & 0b1000) != 0,
        ((control >> 12) & 0b1000) != 0,
    ];
    let llvm_i32 = get_llvm_type(ctx, &ast::Type::Scalar(ast::ScalarType::U32))?;
    let llvm_vec4_i8 = get_llvm_type(ctx, &ast::Type::Vector(ast::ScalarType::U8, 4))?;
    let src1 = ctx.names.value(arg.src1)?;
    let src2 = ctx.names.value(arg.src2)?;
    let src1_vector = unsafe { LLVMBuildBitCast(builder, src1, llvm_vec4_i8, LLVM_UNNAMED) };
    let src2_vector = unsafe { LLVMBuildBitCast(builder, src2, llvm_vec4_i8, LLVM_UNNAMED) };
    let mut components_llvm = [
        unsafe { LLVMConstInt(llvm_i32, components[0] as _, 0) },
        unsafe { LLVMConstInt(llvm_i32, components[1] as _, 0) },
        unsafe { LLVMConstInt(llvm_i32, components[2] as _, 0) },
        unsafe { LLVMConstInt(llvm_i32, components[3] as _, 0) },
    ];
    let mask =
        unsafe { LLVMConstVector(components_llvm.as_mut_ptr(), components_llvm.len() as u32) };
    let mut shuffle_result =
        unsafe { LLVMBuildShuffleVector(builder, src1_vector, src2_vector, mask, LLVM_UNNAMED) };
    // In sext case I'd prefer to just emit V_PERM_B32 directly and be done with it,
    // but V_PERM_B32 can sext only odd-indexed bytes.
    let llvm_i8 = get_llvm_type(ctx, &ast::Type::Scalar(ast::ScalarType::U8))?;
    let const_7 = unsafe { LLVMConstInt(llvm_i8, 7, 0) };
    for (idx, requires_sext) in sext_components.iter().copied().enumerate() {
        if !requires_sext {
            continue;
        }
        let idx = unsafe { LLVMConstInt(llvm_i32, idx as u64, 0) };
        let scalar = unsafe { LLVMBuildExtractElement(builder, shuffle_result, idx, LLVM_UNNAMED) };
        let shift = unsafe { LLVMBuildAShr(builder, scalar, const_7, LLVM_UNNAMED) };
        shuffle_result =
            unsafe { LLVMBuildInsertElement(builder, shuffle_result, shift, idx, LLVM_UNNAMED) };
    }
    ctx.names.register_result(arg.dst, |dst_name| unsafe {
        LLVMBuildBitCast(builder, shuffle_result, llvm_i32, dst_name)
    });
    Ok(())
}

fn emit_inst_setp(
    ctx: &mut EmitContext,
    setp: &ast::SetpData,
    args: &ast::Arg4Setp<ExpandedArgParams>,
    bool_postprocess: Option<(bool, Id, ast::SetpBoolPostOp)>,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let cmp_op_dst1 = if bool_postprocess.is_none() {
        Some(args.dst1)
    } else {
        None
    };
    let cmp_result1 = match setp.typ.kind() {
        ast::ScalarKind::Signed | ast::ScalarKind::Unsigned | ast::ScalarKind::Bit => {
            emit_inst_setp_int(ctx, setp, cmp_op_dst1, args.src1, args.src2)?
        }
        ast::ScalarKind::Float => {
            emit_inst_setp_float(ctx, setp.cmp_op, cmp_op_dst1, args.src1, args.src2)?
        }
        _ => return Err(TranslateError::unreachable()),
    };
    let cmp_result2 = match args.dst2 {
        Some(dst2) => {
            let cmp_op_dst2 = if bool_postprocess.is_none() {
                Some(dst2)
            } else {
                None
            };
            Some((
                dst2,
                emit_inst_not_impl(ctx, ast::ScalarType::Pred, cmp_op_dst2, cmp_result1)?,
            ))
        }
        None => None,
    };
    if let Some((negate_src3, src3, bool_op)) = bool_postprocess {
        let mut src3 = ctx.names.value(src3)?;
        if negate_src3 {
            src3 = emit_inst_not_impl(ctx, ast::ScalarType::Pred, None, src3)?;
        }
        let llvm_fn = match bool_op {
            ast::SetpBoolPostOp::And => LLVMBuildAnd,
            ast::SetpBoolPostOp::Or => LLVMBuildOr,
            ast::SetpBoolPostOp::Xor => LLVMBuildXor,
        };
        ctx.names.register_result(args.dst1, |dst_name| unsafe {
            llvm_fn(builder, cmp_result1, src3, dst_name)
        });
        if let Some((dst2, llvm_result2)) = cmp_result2 {
            ctx.names.register_result(dst2, |dst_name| unsafe {
                llvm_fn(builder, llvm_result2, src3, dst_name)
            });
        }
    }
    Ok(())
}

fn emit_inst_setp_int(
    ctx: &mut EmitContext,
    setp: &ast::SetpData,
    dst: Option<Id>,
    src1: Id,
    src2: Id,
) -> Result<LLVMValueRef, TranslateError> {
    let builder = ctx.builder.get();
    let is_signed = setp.typ.kind() == ast::ScalarKind::Signed;
    let comparer_op = match (setp.cmp_op, is_signed) {
        (ast::SetpCompareOp::Eq, _) => LLVMIntPredicate::LLVMIntEQ,
        (ast::SetpCompareOp::NotEq, _) => LLVMIntPredicate::LLVMIntNE,
        (ast::SetpCompareOp::Less, false) => LLVMIntPredicate::LLVMIntULT,
        (ast::SetpCompareOp::Less, true) => LLVMIntPredicate::LLVMIntSLT,
        (ast::SetpCompareOp::LessOrEq, false) => LLVMIntPredicate::LLVMIntULE,
        (ast::SetpCompareOp::LessOrEq, true) => LLVMIntPredicate::LLVMIntSLE,
        (ast::SetpCompareOp::Greater, false) => LLVMIntPredicate::LLVMIntUGT,
        (ast::SetpCompareOp::Greater, true) => LLVMIntPredicate::LLVMIntSGT,
        (ast::SetpCompareOp::GreaterOrEq, false) => LLVMIntPredicate::LLVMIntUGE,
        (ast::SetpCompareOp::GreaterOrEq, true) => LLVMIntPredicate::LLVMIntSGE,
        (ast::SetpCompareOp::NanEq, _)
        | (ast::SetpCompareOp::NanNotEq, _)
        | (ast::SetpCompareOp::NanLess, _)
        | (ast::SetpCompareOp::NanLessOrEq, _)
        | (ast::SetpCompareOp::NanGreater, _)
        | (ast::SetpCompareOp::NanGreaterOrEq, _)
        | (ast::SetpCompareOp::IsNotNan, _)
        | (ast::SetpCompareOp::IsAnyNan, _) => return Err(TranslateError::unreachable()),
    };
    let src1 = ctx.names.value(src1)?;
    let src2 = ctx.names.value(src2)?;
    Ok(ctx.names.register_result_option(dst, |dst_name| unsafe {
        LLVMBuildICmp(builder, comparer_op, src1, src2, dst_name)
    }))
}

fn emit_inst_setp_float(
    ctx: &mut EmitContext,
    cmp_op: ast::SetpCompareOp,
    dst: Option<Id>,
    src1: Id,
    src2: Id,
) -> Result<LLVMValueRef, TranslateError> {
    let builder = ctx.builder.get();
    let comparer_op = match cmp_op {
        ast::SetpCompareOp::Eq => LLVMRealPredicate::LLVMRealOEQ,
        ast::SetpCompareOp::NotEq => LLVMRealPredicate::LLVMRealONE,
        ast::SetpCompareOp::Less => LLVMRealPredicate::LLVMRealOLT,
        ast::SetpCompareOp::LessOrEq => LLVMRealPredicate::LLVMRealOLE,
        ast::SetpCompareOp::Greater => LLVMRealPredicate::LLVMRealOGT,
        ast::SetpCompareOp::GreaterOrEq => LLVMRealPredicate::LLVMRealOGE,
        ast::SetpCompareOp::NanEq => LLVMRealPredicate::LLVMRealUEQ,
        ast::SetpCompareOp::NanNotEq => LLVMRealPredicate::LLVMRealUNE,
        ast::SetpCompareOp::NanLess => LLVMRealPredicate::LLVMRealULT,
        ast::SetpCompareOp::NanLessOrEq => LLVMRealPredicate::LLVMRealULE,
        ast::SetpCompareOp::NanGreater => LLVMRealPredicate::LLVMRealUGT,
        ast::SetpCompareOp::NanGreaterOrEq => LLVMRealPredicate::LLVMRealUGE,
        ast::SetpCompareOp::IsNotNan => LLVMRealPredicate::LLVMRealORD,
        ast::SetpCompareOp::IsAnyNan => LLVMRealPredicate::LLVMRealUNO,
    };
    let src1 = ctx.names.value(src1)?;
    let src2 = ctx.names.value(src2)?;
    Ok(ctx.names.register_result_option(dst, |dst_name| unsafe {
        LLVMBuildFCmp(builder, comparer_op, src1, src2, dst_name)
    }))
}

fn emit_inst_sub(
    ctx: &mut EmitContext,
    details: &ast::ArithDetails,
    args: &ast::Arg3<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    emit_inst_sub_impl(ctx, details.get_type(), args.dst, args.src1, args.src2)
}

fn emit_inst_sub_impl(
    ctx: &mut EmitContext,
    details: ast::ScalarType,
    dst: Id,
    src1: impl GetLLVMValue,
    src2: impl GetLLVMValue,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let function = match details.kind() {
        ast::ScalarKind::Float | ast::ScalarKind::Float2 => LLVMBuildFSub,
        ast::ScalarKind::Unsigned | ast::ScalarKind::Signed => LLVMBuildSub,
        _ => return Err(TranslateError::unreachable()),
    };
    let src1 = src1.get_llvm_value(&mut ctx.names)?;
    let src2 = src2.get_llvm_value(&mut ctx.names)?;
    ctx.names.register_result(dst, |dst_name| unsafe {
        function(builder, src1, src2, dst_name)
    });
    Ok(())
}

fn emit_inst_neg(
    ctx: &mut EmitContext,
    details: &ast::NegDetails,
    args: &ast::Arg2<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let zero = if details.typ.kind() == ast::ScalarKind::Float {
        ast::ImmediateValue::F64(0.0)
    } else {
        ast::ImmediateValue::S64(0)
    };
    let zero = unsafe { get_llvm_const_scalar(ctx, details.typ, zero)? };
    emit_inst_sub_impl(ctx, details.typ, args.dst, zero, args.src)
}

fn emit_inst_not(
    ctx: &mut EmitContext,
    type_: ast::ScalarType,
    args: &ast::Arg2<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    emit_inst_not_impl(ctx, type_, Some(args.dst), args.src)?;
    Ok(())
}

fn emit_inst_not_impl(
    ctx: &mut EmitContext,
    type_: ast::ScalarType,
    dst: Option<Id>,
    src: impl GetLLVMValue,
) -> Result<LLVMValueRef, TranslateError> {
    let builder = ctx.builder.get();
    let llvm_type = get_llvm_type(ctx, &ast::Type::Scalar(type_))?;
    let constant = unsafe { LLVMConstInt(llvm_type, u64::MAX, 0) };
    let src = src.get_llvm_value(&mut ctx.names)?;
    Ok(ctx.names.register_result_option(dst, |dst_name| unsafe {
        LLVMBuildXor(builder, src, constant, dst_name)
    }))
}

// Looking at the docs (https://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html#memory-fence-functions):
// * __threadfence_block() compiles to membar.cta
// * __threadfence_system() compiles to membar.sys
// Additionally, they are dfined as:
//      "also ensures that no writes to *all* memory made by the calling thread after the call to ..."
fn emit_inst_membar(ctx: &mut EmitContext, level: ast::MemScope) {
    let scope = get_llvm_scope_for_membar(level);
    let ordering = LLVMAtomicOrdering::LLVMAtomicOrderingSequentiallyConsistent;
    unsafe {
        LLVMZludaBuildFence(
            ctx.builder.get(),
            ordering,
            scope.as_ptr() as _,
            b"\0".as_ptr() as _,
        )
    };
}

fn emit_inst_max(
    ctx: &mut EmitContext,
    details: &ast::MinMaxDetails,
    args: &ast::Arg3<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let function_name = match details {
        ast::MinMaxDetails::Signed(ast::ScalarType::S16) => &b"llvm.smax.i16\0"[..],
        ast::MinMaxDetails::Signed(ast::ScalarType::S32) => b"llvm.smax.i32\0",
        ast::MinMaxDetails::Signed(ast::ScalarType::S64) => b"llvm.smax.i64\0",
        ast::MinMaxDetails::Unsigned(ast::ScalarType::U16) => b"llvm.umax.i16\0",
        ast::MinMaxDetails::Unsigned(ast::ScalarType::U32) => b"llvm.umax.i32\0",
        ast::MinMaxDetails::Unsigned(ast::ScalarType::U64) => b"llvm.umax.i64\0",
        ast::MinMaxDetails::Float(desc) => {
            if desc.nan {
                return Err(TranslateError::todo());
            }
            match desc.typ {
                ast::ScalarType::F16 => b"llvm.maxnum.f16\0",
                ast::ScalarType::F32 => b"llvm.maxnum.f32\0",
                ast::ScalarType::F64 => b"llvm.maxnum.f64\0",
                _ => return Err(TranslateError::unreachable()),
            }
        }
        _ => return Err(TranslateError::unreachable()),
    };
    let type_ = details.get_type();
    emit_intrinsic_arg3(
        ctx,
        (
            get_llvm_type(ctx, &ast::Type::Scalar(type_))?,
            Some(args.dst),
        ),
        (type_, args.src1),
        (type_, args.src2),
        function_name,
    )?;
    Ok(())
}

fn emit_inst_min(
    ctx: &mut EmitContext,
    details: &ast::MinMaxDetails,
    args: &ast::Arg3<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let function_name = match details {
        ast::MinMaxDetails::Signed(ast::ScalarType::S16) => &b"llvm.smin.i16\0"[..],
        ast::MinMaxDetails::Signed(ast::ScalarType::S32) => b"llvm.smin.i32\0",
        ast::MinMaxDetails::Signed(ast::ScalarType::S64) => b"llvm.smin.i64\0",
        ast::MinMaxDetails::Unsigned(ast::ScalarType::U16) => b"llvm.umin.i16\0",
        ast::MinMaxDetails::Unsigned(ast::ScalarType::U32) => b"llvm.umin.i32\0",
        ast::MinMaxDetails::Unsigned(ast::ScalarType::U64) => b"llvm.umin.i64\0",
        ast::MinMaxDetails::Float(desc) => {
            if desc.nan {
                return Err(TranslateError::todo());
            }
            match desc.typ {
                ast::ScalarType::F16 => b"llvm.minnum.f16\0",
                ast::ScalarType::F32 => b"llvm.minnum.f32\0",
                ast::ScalarType::F64 => b"llvm.minnum.f64\0",
                _ => return Err(TranslateError::unreachable()),
            }
        }
        _ => return Err(TranslateError::unreachable()),
    };
    let type_ = details.get_type();
    emit_intrinsic_arg3(
        ctx,
        (
            get_llvm_type(ctx, &ast::Type::Scalar(type_))?,
            Some(args.dst),
        ),
        (type_, args.src1),
        (type_, args.src2),
        function_name,
    )?;
    Ok(())
}

fn emit_inst_mad(
    ctx: &mut EmitContext,
    details: &ast::MulDetails,
    args: &ast::Arg4<crate::translate::ExpandedArgParams>,
) -> Result<(), TranslateError> {
    match details {
        ast::MulDetails::Unsigned(ast::MulInt {
            control: ast::MulIntControl::Low,
            typ,
        })
        | ast::MulDetails::Signed(ast::MulInt {
            control: ast::MulIntControl::Low,
            typ,
        }) => emit_inst_mad_lo(ctx, *typ, args),
        ast::MulDetails::Unsigned(ast::MulInt {
            control: ast::MulIntControl::High,
            typ,
        })
        | ast::MulDetails::Signed(ast::MulInt {
            control: ast::MulIntControl::High,
            typ,
        }) => emit_inst_mad_hi(ctx, *typ, args),
        ast::MulDetails::Unsigned(ast::MulInt {
            control: ast::MulIntControl::Wide,
            typ,
        })
        | ast::MulDetails::Signed(ast::MulInt {
            control: ast::MulIntControl::Wide,
            typ,
        }) => emit_inst_mad_wide(ctx, *typ, args),
        ast::MulDetails::Float(arith) => return emit_inst_fma(ctx, arith, args),
    }
}

fn emit_inst_mad_wide(
    ctx: &mut EmitContext,
    type_: ast::ScalarType,
    args: &ast::Arg4<crate::translate::ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let result = emit_inst_mul_wide(ctx, type_, None, args.src1, args.src2)?;
    let wide_type = type_.widen()?;
    emit_inst_add_impl(
        ctx,
        &ast::ArithDetails::Unsigned(wide_type),
        args.dst,
        result,
        args.src3,
    )
}

fn emit_inst_mad_hi(
    ctx: &mut EmitContext,
    typ: ast::ScalarType,
    args: &ast::Arg4<crate::translate::ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let temp_dst = emit_inst_mul_hi_impl(ctx, typ, None, args.src1, args.src2)?;
    emit_inst_add_impl(
        ctx,
        &ast::ArithDetails::Unsigned(typ),
        args.dst,
        temp_dst,
        args.src3,
    )
}

fn emit_inst_mad_lo(
    ctx: &mut EmitContext,
    typ: ast::ScalarType,
    args: &ast::Arg4<crate::translate::ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let mul_result = emit_inst_mul_low_impl(ctx, None, args.src1, args.src2, LLVMBuildMul)?;
    emit_inst_add_impl(
        ctx,
        &ast::ArithDetails::Unsigned(typ),
        args.dst,
        mul_result,
        args.src3,
    )
}

// TODO: support mad.hi.cc
fn emit_inst_madcc(
    ctx: &mut EmitContext,
    type_: ast::ScalarType,
    arg: &Arg4CarryOut<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let src1 = ctx.names.value(arg.src1)?;
    let src2 = ctx.names.value(arg.src2)?;
    let mul_result = unsafe { LLVMBuildMul(builder, src1, src2, LLVM_UNNAMED) };
    emit_inst_addsub_cc_impl(
        ctx,
        "add",
        type_,
        arg.dst,
        arg.carry_out,
        mul_result,
        arg.src3,
    )
}

fn get_llvm_type_struct<'a>(
    ctx: &EmitContext,
    types: impl ExactSizeIterator<Item = Cow<'a, ast::Type>>,
) -> Result<LLVMTypeRef, TranslateError> {
    use std::fmt::Write;
    let element_count = types.len() as u32;
    let mut name_suffix = "struct.".to_string();
    let mut llvm_types = Vec::new();
    let mut is_anonymous = false;
    for type_ in types {
        llvm_types.push(get_llvm_type(ctx, &type_)?);
        match &*type_ {
            ast::Type::Scalar(type_) => {
                write!(name_suffix, "{}", type_.llvm_display())
                    .map_err(|_| TranslateError::unreachable())?;
            }
            _ => is_anonymous = true,
        }
    }
    name_suffix.push('\0');
    if is_anonymous {
        unsafe {
            Ok(LLVMStructTypeInContext(
                ctx.context.get(),
                llvm_types.as_mut_ptr(),
                element_count,
                0,
            ))
        }
    } else {
        // We emit named type for structs, because unnamed LLVM structs are not subject
        // to unification during linking, e.g. if we have a func decl:
        //   declare protected { float, float } @foobar(i32, i32)
        // that is used like this:
        //   %0 = call { float, float } @foobar(i32 %"arg1", i32 %"arg2")
        // and definition in another module, like this:
        //   %struct.float2_struct = type { float, float }
        //   define linkonce_odr hidden %struct.float2_struct @foobar(i32 %0, i32 %1) {
        //     ...
        //   }
        // then llvm-link emits our original call as a cast:
        //   %0 = call { float, float } bitcast (%struct.float2_struct (i32, i32)* @foobar to { float, float } (i32, i32)*)(i32 %"arg1", i32 %"arg2")
        // which makes certain amount of sense, but prevents inlining
        let mut struct_type =
            unsafe { LLVMGetTypeByName2(ctx.context.get(), name_suffix.as_ptr() as _) };
        if struct_type == ptr::null_mut() {
            struct_type =
                unsafe { LLVMStructCreateNamed(ctx.context.get(), name_suffix.as_ptr() as _) };
            unsafe { LLVMStructSetBody(struct_type, llvm_types.as_mut_ptr(), element_count, 0) };
        }
        Ok(struct_type)
    }
}

fn emit_inst_madc(
    ctx: &mut EmitContext,
    type_: ast::ScalarType,
    is_hi: bool,
    args: &translate::Arg4CarryIn<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let mul_result = if is_hi {
        emit_inst_mul_hi_impl(ctx, type_, None, args.src1, args.src2)?
    } else {
        emit_inst_mul_low_impl(ctx, None, args.src1, args.src2, LLVMBuildMul)?
    };
    emit_inst_addsub_c_impl(
        ctx,
        "add",
        LLVMBuildAdd,
        type_,
        args.dst,
        args.carry_out,
        args.carry_in,
        mul_result,
        args.src3,
    )
    /*
       let src3 = ctx.names.value(args.src3)?;
       let add_no_carry = unsafe { LLVMBuildAdd(builder, mul_result, src3, LLVM_UNNAMED) };
       let carry_flag = ctx.names.value(args.carry_in)?;
       let llvm_type = get_llvm_type(ctx, &ast::Type::Scalar(type_))?;
       let carry_flag = unsafe { LLVMBuildZExt(builder, carry_flag, llvm_type, LLVM_UNNAMED) };
       if let Some(carry_out) = args.carry_out {
           emit_inst_addsub_cc_impl(
               ctx,
               "add",
               type_,
               args.dst,
               carry_out,
               add_no_carry,
               carry_flag,
           )?;
       } else {
           ctx.names.register_result(args.dst, |dst| unsafe {
               LLVMBuildAdd(builder, add_no_carry, carry_flag, dst)
           });
       }
       Ok(())
    */
}

fn emit_inst_add_c(
    ctx: &mut EmitContext,
    type_: ast::ScalarType,
    arg: &translate::Arg3CarryIn<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    emit_inst_addsub_c_impl(
        ctx,
        "add",
        LLVMBuildAdd,
        type_,
        arg.dst,
        arg.carry_out,
        arg.carry_in,
        arg.src1,
        arg.src2,
    )
}

fn emit_inst_sub_c(
    ctx: &mut EmitContext<'_>,
    type_: ast::ScalarType,
    arg: &translate::Arg3CarryIn<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    emit_inst_addsub_c_impl(
        ctx,
        "sub",
        LLVMBuildSub,
        type_,
        arg.dst,
        arg.carry_out,
        arg.carry_in,
        arg.src1,
        arg.src2,
    )
}

fn emit_inst_addsub_c_impl(
    ctx: &mut EmitContext,
    op: &str,
    llvm_fn: unsafe extern "C" fn(
        LLVMBuilderRef,
        LLVMValueRef,
        LLVMValueRef,
        *const i8,
    ) -> LLVMValueRef,
    type_: ast::ScalarType,
    dst: Id,
    carry_out: Option<Id>,
    carry_in: Id,
    src1: impl GetLLVMValue,
    src2: Id,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let carry_in = ctx.names.value(carry_in)?;
    let carry_in = unsafe {
        LLVMBuildZExt(
            builder,
            carry_in,
            get_llvm_type(ctx, &ast::Type::Scalar(type_))?,
            LLVM_UNNAMED,
        )
    };
    if let Some(carry_out) = carry_out {
        let fn_name = format!("llvm.u{}.with.overflow.{}\0", op, type_.llvm_display());
        let dst_type = get_anonymous_struct_type(ctx, [type_, ast::ScalarType::Pred])?;
        let temp_result1 = emit_intrinsic_arg3(
            ctx,
            (dst_type, None),
            (type_, src1),
            (type_, src2),
            fn_name.as_bytes(),
        )?;
        let temp_value = unsafe { LLVMBuildExtractValue(builder, temp_result1, 0, LLVM_UNNAMED) };
        let intermediate_flag1 =
            unsafe { LLVMBuildExtractValue(builder, temp_result1, 1, LLVM_UNNAMED) };
        let temp_result2 = emit_intrinsic_arg3(
            ctx,
            (dst_type, None),
            (type_, temp_value),
            (type_, carry_in),
            fn_name.as_bytes(),
        )?;
        ctx.names.register_result(dst, |dst: *const i8| unsafe {
            LLVMBuildExtractValue(builder, temp_result2, 0, dst)
        });
        let intermediate_flag2 =
            unsafe { LLVMBuildExtractValue(builder, temp_result2, 1, LLVM_UNNAMED) };
        ctx.names
            .register_result(carry_out, |dst: *const i8| unsafe {
                LLVMBuildXor(builder, intermediate_flag1, intermediate_flag2, dst)
            });
    } else {
        let src2 = ctx.names.value(src2)?;
        let temp = unsafe {
            llvm_fn(
                builder,
                src1.get_llvm_value(&mut ctx.names)?,
                src2,
                LLVM_UNNAMED,
            )
        };
        ctx.names
            .register_result(dst, |dst| unsafe { llvm_fn(builder, temp, carry_in, dst) });
    }
    Ok(())
}

fn emit_inst_addsub_cc(
    ctx: &mut EmitContext,
    op: &str,
    type_: ast::ScalarType,
    arg: &translate::Arg3CarryOut<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    emit_inst_addsub_cc_impl(ctx, op, type_, arg.dst, arg.carry_flag, arg.src1, arg.src2)
}

fn emit_inst_addsub_cc_impl(
    ctx: &mut EmitContext,
    op: &str,
    type_: ast::ScalarType,
    dst: Id,
    carry_flag: Id,
    src1: impl GetLLVMValue,
    src2: impl GetLLVMValue,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let dst_type = get_anonymous_struct_type(ctx, [type_, ast::ScalarType::Pred])?;
    let fn_name = format!("llvm.u{}.with.overflow.{}\0", op, type_.llvm_display());
    let result = emit_intrinsic_arg3(
        ctx,
        (dst_type, None),
        (type_, src1),
        (type_, src2),
        fn_name.as_bytes(),
    )?;
    ctx.names.register_result(dst, |dst: *const i8| unsafe {
        LLVMBuildExtractValue(builder, result, 0, dst)
    });
    ctx.names.register_result(carry_flag, |dst| unsafe {
        LLVMBuildExtractValue(builder, result, 1, dst)
    });
    Ok(())
}

fn emit_inst_fma(
    ctx: &mut EmitContext,
    details: &ast::ArithFloat,
    args: &ast::Arg4<translate::ExpandedArgParams>,
) -> Result<(), TranslateError> {
    if details.saturate
    //|| !(details.rounding == None || details.rounding == Some(ast::RoundingMode::NearestEven))
    {
        return Err(TranslateError::todo());
    }
    let builder = ctx.builder.get();
    let src1 = ctx.names.value(args.src1)?;
    let src2 = ctx.names.value(args.src2)?;
    let src3 = ctx.names.value(args.src3)?;
    let intrinsic_name = match details.typ {
        ast::ScalarType::F16 => &b"llvm.fma.f16\0"[..],
        ast::ScalarType::F16x2 => b"llvm.fma.v2f16\0",
        ast::ScalarType::F32 => b"llvm.fma.f32\0",
        ast::ScalarType::F64 => b"llvm.fma.f64\0",
        _ => return Err(TranslateError::unreachable()),
    };
    let llvm_type = get_llvm_type(ctx, &ast::Type::Scalar(details.typ))?;
    let function_type = get_llvm_function_type(
        ctx,
        llvm_type,
        iter::repeat((&ast::Type::Scalar(details.typ), ast::StateSpace::Reg)).take(3),
    )?;
    let mut function_value =
        unsafe { LLVMGetNamedFunction(ctx.module.get(), intrinsic_name.as_ptr() as _) };
    if function_value == ptr::null_mut() {
        function_value = unsafe {
            LLVMAddFunction(
                ctx.module.get(),
                intrinsic_name.as_ptr() as _,
                function_type,
            )
        };
    }
    let mut fn_args = [src1, src2, src3];
    ctx.names.register_result(args.dst, |dst| unsafe {
        LLVMBuildCall2(
            builder,
            function_type,
            function_value,
            fn_args.as_mut_ptr(),
            3,
            dst,
        )
    });
    Ok(())
}

fn emit_inst_div(
    ctx: &mut EmitContext,
    details: &ast::DivDetails,
    args: &ast::Arg3<crate::translate::ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let src1 = ctx.names.value(args.src1)?;
    let src2 = ctx.names.value(args.src2)?;
    let (llvm_fn, approx) = match details {
        ast::DivDetails::Unsigned(_) => (
            LLVMBuildUDiv as unsafe extern "C" fn(_, _, _, _) -> _,
            false,
        ),
        ast::DivDetails::Signed(_) => (
            LLVMBuildSDiv as unsafe extern "C" fn(_, _, _, _) -> _,
            false,
        ),
        ast::DivDetails::Float(ast::DivFloatDetails { kind, .. }) => match kind {
            ast::DivFloatKind::Approx => {
                (LLVMBuildFDiv as unsafe extern "C" fn(_, _, _, _) -> _, true)
            }
            ast::DivFloatKind::Full
            | ast::DivFloatKind::Rounding(ast::RoundingMode::NearestEven) => (
                LLVMBuildFDiv as unsafe extern "C" fn(_, _, _, _) -> _,
                false,
            ),
            // TODO: Fix this
            ast::DivFloatKind::Rounding(_) => (
                LLVMBuildFDiv as unsafe extern "C" fn(_, _, _, _) -> _,
                false,
            ),
        },
    };
    let value = ctx.names.register_result(args.dst, |dst_name| unsafe {
        llvm_fn(builder, src1, src2, dst_name)
    });
    if approx {
        unsafe {
            LLVMZludaSetFastMathFlags(
                value,
                FastMathFlags::AllowReciprocal | FastMathFlags::ApproxFunc,
            )
        };
    }
    Ok(())
}

fn emit_inst_cvt(
    ctx: &mut EmitContext,
    details: &ast::CvtDetails,
    args: &ast::Arg2<crate::translate::ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    Ok(match details {
        ast::CvtDetails::FloatFromFloat(desc) => {
            if desc.saturate {
                return Err(TranslateError::todo());
            }
            let type_ = desc.dst;
            if desc.dst == desc.src {
                match desc.rounding {
                    Some(ast::RoundingMode::NearestEven) => {
                        let intrinsic_fn = match type_ {
                            ast::ScalarType::F16 => &b"llvm.rint.f16\0"[..],
                            ast::ScalarType::F32 => &b"llvm.rint.f32\0"[..],
                            ast::ScalarType::F64 => &b"llvm.rint.f64\0"[..],
                            _ => return Err(TranslateError::unreachable()),
                        };
                        emit_intrinsic_arg2(
                            ctx,
                            (type_, Some(args.dst)),
                            (type_, ast::StateSpace::Reg, args.src),
                            intrinsic_fn,
                        )?;
                    }
                    Some(ast::RoundingMode::Zero) => {
                        let intrinsic_fn = match type_ {
                            ast::ScalarType::F16 => &b"llvm.trunc.f16\0"[..],
                            ast::ScalarType::F32 => &b"llvm.trunc.f32\0"[..],
                            ast::ScalarType::F64 => &b"llvm.trunc.f64\0"[..],
                            _ => return Err(TranslateError::unreachable()),
                        };
                        emit_intrinsic_arg2(
                            ctx,
                            (type_, Some(args.dst)),
                            (type_, ast::StateSpace::Reg, args.src),
                            intrinsic_fn,
                        )?;
                    }
                    Some(ast::RoundingMode::NegativeInf) => {
                        let intrinsic_fn = match type_ {
                            ast::ScalarType::F16 => &b"llvm.floor.f16\0"[..],
                            ast::ScalarType::F32 => &b"llvm.floor.f32\0"[..],
                            ast::ScalarType::F64 => &b"llvm.floor.f64\0"[..],
                            _ => return Err(TranslateError::unreachable()),
                        };
                        emit_intrinsic_arg2(
                            ctx,
                            (type_, Some(args.dst)),
                            (type_, ast::StateSpace::Reg, args.src),
                            intrinsic_fn,
                        )?;
                    }
                    Some(ast::RoundingMode::PositiveInf) => {
                        let intrinsic_fn = match type_ {
                            ast::ScalarType::F16 => &b"llvm.ceil.f16\0"[..],
                            ast::ScalarType::F32 => &b"llvm.ceil.f32\0"[..],
                            ast::ScalarType::F64 => &b"llvm.ceil.f64\0"[..],
                            _ => return Err(TranslateError::unreachable()),
                        };
                        emit_intrinsic_arg2(
                            ctx,
                            (type_, Some(args.dst)),
                            (type_, ast::StateSpace::Reg, args.src),
                            intrinsic_fn,
                        )?;
                    }
                    None => {
                        let src = ctx.names.value(args.src)?;
                        emit_value_copy(ctx, &ast::Type::Scalar(type_), src, args.dst)?;
                    }
                }
            } else if desc.dst.size_of() > desc.src.size_of() {
                let src = ctx.names.value(args.src)?;
                let type_ = get_llvm_type(ctx, &ast::Type::Scalar(desc.dst))?;
                ctx.names.register_result(args.dst, |dst| unsafe {
                    LLVMBuildFPExt(builder, src, type_, dst)
                });
            } else {
                // Replaced by a function call
                return Err(TranslateError::unreachable());
            }
        }
        ast::CvtDetails::FloatFromInt(_) => {
            // Replaced by a function call
            return Err(TranslateError::unreachable());
        }
        ast::CvtDetails::IntFromFloat(_) => {
            // Replaced by a function call
            return Err(TranslateError::unreachable());
        }
        ast::CvtDetails::IntFromInt(desc) => emit_inst_cvt_int_from_int(ctx, desc, args)?,
    })
}

fn emit_inst_cvt_int_from_int(
    ctx: &mut EmitContext,
    desc: &ast::CvtIntToIntDesc,
    args: &ast::Arg2<crate::translate::ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let src = if desc.saturate {
        match emit_int_saturate_if_needed(ctx, desc, args) {
            Ok(Some(src)) => src,
            Ok(None) => ctx.names.value(args.src)?,
            Err(err) => return Err(err),
        }
    } else {
        ctx.names.value(args.src)?
    };
    // The value is now clamped, do bit conversions
    let llvm_op = if desc.dst.size_of() > desc.src.size_of() {
        if desc.src.kind() == ast::ScalarKind::Signed {
            LLVMBuildSExt
        } else {
            LLVMBuildZExt
        }
    } else if desc.dst.size_of() < desc.src.size_of() {
        LLVMBuildTrunc
    } else {
        return emit_value_copy(ctx, &ast::Type::Scalar(desc.dst), src, args.dst);
    };
    let type_ = get_llvm_type(ctx, &ast::Type::Scalar(desc.dst))?;
    ctx.names
        .register_result(args.dst, |dst| unsafe { llvm_op(builder, src, type_, dst) });
    Ok(())
}

fn emit_int_saturate_if_needed(
    ctx: &mut EmitContext,
    desc: &ast::CvtIntToIntDesc,
    args: &ast::Arg2<crate::translate::ExpandedArgParams>,
) -> Result<Option<LLVMValueRef>, TranslateError> {
    let (src_min, src_max) = get_min_max(desc.src)?;
    let (dst_min, dst_max) = get_min_max(desc.dst)?;
    let src_type = get_llvm_type(ctx, &ast::Type::Scalar(desc.src))?;
    let mut result = None;
    if src_max > dst_max {
        let intrinsic_name = match desc.src.kind() {
            ast::ScalarKind::Signed => format!("llvm.smin.{}\0", desc.src.llvm_display()),
            ast::ScalarKind::Bit | ast::ScalarKind::Unsigned => {
                format!("llvm.umin.{}\0", desc.src.llvm_display())
            }
            ast::ScalarKind::Float | ast::ScalarKind::Float2 | ast::ScalarKind::Pred => {
                return Err(TranslateError::unreachable())
            }
        };
        let dst_max_llvm = unsafe { LLVMConstInt(src_type, dst_max as _, 0) };
        result = Some(emit_intrinsic_arg3(
            ctx,
            (get_llvm_type(ctx, &ast::Type::Scalar(desc.src))?, None),
            (desc.src, args.src),
            (desc.src, dst_max_llvm),
            intrinsic_name.as_bytes(),
        )?);
    }
    if src_min < dst_min {
        let intrinsic_name = match desc.src.kind() {
            ast::ScalarKind::Signed => format!("llvm.smax.{}\0", desc.src.llvm_display()),
            ast::ScalarKind::Bit | ast::ScalarKind::Unsigned => {
                format!("llvm.umax.{}\0", desc.src.llvm_display())
            }
            ast::ScalarKind::Float | ast::ScalarKind::Float2 | ast::ScalarKind::Pred => {
                return Err(TranslateError::unreachable())
            }
        };
        let dst_min_llvm = unsafe { LLVMConstInt(src_type, dst_min as _, 0) };
        result = Some(emit_intrinsic_arg3(
            ctx,
            (get_llvm_type(ctx, &ast::Type::Scalar(desc.src))?, None),
            (desc.src, args.src),
            (desc.src, dst_min_llvm),
            intrinsic_name.as_bytes(),
        )?);
    }
    Ok(result)
}

fn get_min_max(type_: ast::ScalarType) -> Result<(i128, i128), TranslateError> {
    Ok(match type_ {
        ast::ScalarType::B8 | ast::ScalarType::U8 => (u8::MIN as _, u8::MAX as _),
        ast::ScalarType::B16 | ast::ScalarType::U16 => (u16::MIN as _, u16::MAX as _),
        ast::ScalarType::B32 | ast::ScalarType::U32 => (u32::MIN as _, u32::MAX as _),
        ast::ScalarType::B64 | ast::ScalarType::U64 => (u64::MIN as _, u64::MAX as _),
        ast::ScalarType::S8 => (i8::MIN as _, i8::MAX as _),
        ast::ScalarType::S16 => (i16::MIN as _, i16::MAX as _),
        ast::ScalarType::S32 => (i32::MIN as _, i32::MAX as _),
        ast::ScalarType::S64 => (i64::MIN as _, i64::MAX as _),
        ast::ScalarType::F16
        | ast::ScalarType::F32
        | ast::ScalarType::F64
        | ast::ScalarType::F16x2
        | ast::ScalarType::Pred => return Err(TranslateError::unreachable()),
    })
}

fn emit_inst_cvta(
    ctx: &mut EmitContext,
    details: &ast::CvtaDetails,
    args: &ast::Arg2<crate::translate::ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let src = ctx.names.value(args.src)?;
    let from_ptr_type =
        get_llvm_pointer_type(ctx, &ast::Type::Scalar(ast::ScalarType::B8), details.from)?;
    let src_ptr = unsafe { LLVMBuildIntToPtr(builder, src, from_ptr_type, b"\0".as_ptr() as _) };
    let to_ptr_type =
        get_llvm_pointer_type(ctx, &ast::Type::Scalar(ast::ScalarType::B8), details.to)?;
    let cast_result =
        unsafe { LLVMBuildAddrSpaceCast(builder, src_ptr, to_ptr_type, b"\0".as_ptr() as _) };
    let scalar_type = match details.size {
        ast::CvtaSize::U32 => ast::ScalarType::U32,
        ast::CvtaSize::U64 => ast::ScalarType::U64,
    };
    let type_ = get_llvm_type(ctx, &ast::Type::Scalar(scalar_type))?;
    ctx.names.register_result(args.dst, |dst_name| unsafe {
        LLVMBuildPtrToInt(builder, cast_result, type_, dst_name)
    });
    Ok(())
}

fn emit_inst_cos(
    ctx: &mut EmitContext,
    args: &ast::Arg2<crate::translate::ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let function_name = b"llvm.cos.f32\0";
    let cos_value = emit_intrinsic_arg2(
        ctx,
        (ast::ScalarType::F32, Some(args.dst)),
        (ast::ScalarType::F32, ast::StateSpace::Reg, args.src),
        function_name,
    )?;
    unsafe { LLVMZludaSetFastMathFlags(cos_value, FastMathFlags::ApproxFunc) };
    Ok(())
}

fn emit_inst_sin(
    ctx: &mut EmitContext,
    args: &ast::Arg2<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let function_name = b"llvm.sin.f32\0";
    let cos_value = emit_intrinsic_arg2(
        ctx,
        (ast::ScalarType::F32, Some(args.dst)),
        (ast::ScalarType::F32, ast::StateSpace::Reg, args.src),
        function_name,
    )?;
    unsafe { LLVMZludaSetFastMathFlags(cos_value, FastMathFlags::ApproxFunc) };
    Ok(())
}

fn emit_inst_mul(
    ctx: &mut EmitContext,
    details: &ast::MulDetails,
    args: &ast::Arg3<crate::translate::ExpandedArgParams>,
) -> Result<(), TranslateError> {
    match details {
        ast::MulDetails::Unsigned(ast::MulInt {
            control: ast::MulIntControl::Wide,
            typ,
        })
        | ast::MulDetails::Signed(ast::MulInt {
            control: ast::MulIntControl::Wide,
            typ,
        }) => {
            emit_inst_mul_wide(ctx, *typ, Some(args.dst), args.src1, args.src2)?;
            Ok(())
        }
        ast::MulDetails::Unsigned(ast::MulInt {
            control: ast::MulIntControl::Low,
            ..
        })
        | ast::MulDetails::Signed(ast::MulInt {
            control: ast::MulIntControl::Low,
            ..
        }) => emit_inst_mul_lo(ctx, args, LLVMBuildMul),
        ast::MulDetails::Float(ast::ArithFloat { .. }) => {
            emit_inst_mul_lo(ctx, args, LLVMBuildFMul)
        }
        ast::MulDetails::Unsigned(ast::MulInt {
            control: ast::MulIntControl::High,
            typ,
        })
        | ast::MulDetails::Signed(ast::MulInt {
            control: ast::MulIntControl::High,
            typ,
        }) => {
            emit_inst_mul_hi(ctx, *typ, args)?;
            Ok(())
        }
    }
}

fn emit_inst_mul_hi(
    ctx: &mut EmitContext,
    type_: ast::ScalarType,
    args: &ast::Arg3<crate::translate::ExpandedArgParams>,
) -> Result<(), TranslateError> {
    emit_inst_mul_hi_impl(ctx, type_, Some(args.dst), args.src1, args.src2)?;
    Ok(())
}

fn emit_inst_mul_hi_impl(
    ctx: &mut EmitContext,
    type_: ast::ScalarType,
    dst: Option<Id>,
    src1: impl GetLLVMValue,
    src2: impl GetLLVMValue,
) -> Result<LLVMValueRef, TranslateError> {
    let temp_dst = emit_inst_mul_wide(ctx, type_, None, src1, src2)?;
    emit_get_high_bits(ctx, type_, temp_dst, dst)
}

fn emit_get_high_bits(
    ctx: &mut EmitContext,
    narrow_type: ast::ScalarType,
    input: LLVMValueRef,
    dst: Option<Id>,
) -> Result<LLVMValueRef, TranslateError> {
    let builder: *mut LLVMBuilder = ctx.builder.get();
    let llvm_narrow_type = get_llvm_type(ctx, &ast::Type::Scalar(narrow_type))?;
    let llvm_wide_type = get_llvm_type(ctx, &ast::Type::Scalar(narrow_type.widen()?))?;
    let shift_constant =
        unsafe { LLVMConstInt(llvm_wide_type, narrow_type.size_of() as u64 * 8u64, 0) };
    let shift_result = unsafe { LLVMBuildLShr(builder, input, shift_constant, LLVM_UNNAMED) };
    Ok(ctx.names.register_result_option(dst, |dst_name| unsafe {
        LLVMBuildTrunc(builder, shift_result, llvm_narrow_type, dst_name)
    }))
}

fn emit_inst_mul_wide(
    ctx: &mut EmitContext,
    type_: ast::ScalarType,
    dst: Option<Id>,
    src1: impl GetLLVMValue,
    src2: impl GetLLVMValue,
) -> Result<LLVMValueRef, TranslateError> {
    let builder = ctx.builder.get();
    let (extend_fn, build_fn) = if type_.kind() == ast::ScalarKind::Signed {
        (
            LLVMBuildSExt as unsafe extern "C" fn(_, _, _, _) -> _,
            LLVMBuildNSWMul as unsafe extern "C" fn(_, _, _, _) -> _,
        )
    } else {
        (
            LLVMBuildZExt as unsafe extern "C" fn(_, _, _, _) -> _,
            LLVMBuildNUWMul as unsafe extern "C" fn(_, _, _, _) -> _,
        )
    };
    let widened_type = get_llvm_type(ctx, &ast::Type::Scalar(type_.widen()?))?;
    let src1 = src1.get_llvm_value(&mut ctx.names)?;
    let src2 = src2.get_llvm_value(&mut ctx.names)?;
    let src1_temp = unsafe { extend_fn(builder, src1, widened_type, b"\0".as_ptr() as _) };
    let src2_temp = unsafe { extend_fn(builder, src2, widened_type, b"\0".as_ptr() as _) };
    Ok(ctx.names.register_result_option(dst, |dst_name| unsafe {
        build_fn(builder, src1_temp, src2_temp, dst_name)
    }))
}

fn emit_inst_mul_lo(
    ctx: &mut EmitContext,
    args: &ast::Arg3<crate::translate::ExpandedArgParams>,
    llvm_fn: unsafe extern "C" fn(
        LLVMBuilderRef,
        LLVMValueRef,
        LLVMValueRef,
        *const i8,
    ) -> LLVMValueRef,
) -> Result<(), TranslateError> {
    emit_inst_mul_low_impl(ctx, Some(args.dst), args.src1, args.src2, llvm_fn)?;
    Ok(())
}

fn emit_inst_mul_low_impl(
    ctx: &mut EmitContext,
    dst: Option<Id>,
    src1: impl GetLLVMValue,
    src2: impl GetLLVMValue,
    llvm_fn: unsafe extern "C" fn(
        LLVMBuilderRef,
        LLVMValueRef,
        LLVMValueRef,
        *const i8,
    ) -> LLVMValueRef,
) -> Result<LLVMValueRef, TranslateError> {
    let builder = ctx.builder.get();
    let src1 = src1.get_llvm_value(&mut ctx.names)?;
    let src2 = src2.get_llvm_value(&mut ctx.names)?;
    Ok(ctx
        .names
        .register_result_option(dst, |dst| unsafe { llvm_fn(builder, src1, src2, dst) }))
}

fn emit_inst_clz(
    ctx: &mut EmitContext,
    type_: ast::ScalarType,
    args: &ast::Arg2<crate::translate::ExpandedArgParams>,
) -> Result<(), TranslateError> {
    emit_inst_clz_impl(ctx, type_, Some(args.dst), args.src, false)?;
    Ok(())
}

fn emit_inst_clz_impl(
    ctx: &mut EmitContext,
    type_: ast::ScalarType,
    dst: Option<Id>,
    src: Id,
    is_zero_poison: bool,
) -> Result<LLVMValueRef, TranslateError> {
    let builder = ctx.builder.get();
    let intrinsic_name = match type_.size_of() {
        4 => &b"llvm.ctlz.i32\0"[..],
        8 => b"llvm.ctlz.i64\0",
        _ => return Err(TranslateError::unreachable()),
    };
    let const_0 = unsafe {
        LLVMConstInt(
            get_llvm_type(ctx, &ast::Type::Scalar(ast::ScalarType::Pred))?,
            is_zero_poison as _,
            0,
        )
    };
    let temp_result = emit_intrinsic_arg3(
        ctx,
        (get_llvm_type(ctx, &ast::Type::Scalar(type_))?, None),
        (type_, src),
        (ast::ScalarType::Pred, const_0),
        intrinsic_name,
    )?;
    let b32_type = get_llvm_type(ctx, &ast::Type::Scalar(ast::ScalarType::B32))?;
    Ok(ctx.names.register_result_option(dst, |dst_name| unsafe {
        LLVMBuildTrunc(builder, temp_result, b32_type, dst_name)
    }))
}

fn emit_inst_brev(
    ctx: &mut EmitContext,
    type_: ast::ScalarType,
    args: &ast::Arg2<crate::translate::ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let function_name = match type_.size_of() {
        4 => &b"llvm.bitreverse.i32\0"[..],
        8 => b"llvm.bitreverse.i64\0",
        _ => return Err(TranslateError::unreachable()),
    };
    emit_intrinsic_arg2(
        ctx,
        (type_, Some(args.dst)),
        (type_, ast::StateSpace::Reg, args.src),
        function_name,
    )?;
    Ok(())
}

fn emit_inst_popc(
    ctx: &mut EmitContext,
    type_: ast::ScalarType,
    args: &ast::Arg2<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let (function_name, shorten) = match type_.size_of() {
        4 => (&b"llvm.ctpop.i32\0"[..], false),
        8 => (&b"llvm.ctpop.i64\0"[..], true),
        _ => return Err(TranslateError::unreachable()),
    };
    let popc_dst = if shorten { None } else { Some(args.dst) };
    let popc_result = emit_intrinsic_arg2(
        ctx,
        (type_, popc_dst),
        (type_, ast::StateSpace::Reg, args.src),
        function_name,
    )?;
    if shorten {
        let llvm_i32 = get_llvm_type(ctx, &ast::Type::Scalar(ast::ScalarType::U32))?;
        ctx.names.register_result(args.dst, |dst_name| unsafe {
            LLVMBuildTrunc(builder, popc_result, llvm_i32, dst_name)
        });
    }
    Ok(())
}

fn emit_inst_ex2(
    ctx: &mut EmitContext,
    args: &ast::Arg2<crate::translate::ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let function_name = b"llvm.exp2.f32\0";
    let llvm_value = emit_intrinsic_arg2(
        ctx,
        (ast::ScalarType::F32, Some(args.dst)),
        (ast::ScalarType::F32, ast::StateSpace::Reg, args.src),
        function_name,
    )?;
    unsafe { LLVMZludaSetFastMathFlags(llvm_value, FastMathFlags::ApproxFunc) };
    Ok(())
}

fn emit_inst_lg2(
    ctx: &mut EmitContext,
    args: &ast::Arg2<crate::translate::ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let function_name = b"llvm.log2.f32\0";
    let llvm_value = emit_intrinsic_arg2(
        ctx,
        (ast::ScalarType::F32, Some(args.dst)),
        (ast::ScalarType::F32, ast::StateSpace::Reg, args.src),
        function_name,
    )?;
    unsafe { LLVMZludaSetFastMathFlags(llvm_value, FastMathFlags::ApproxFunc) };
    Ok(())
}

fn emit_intrinsic_arg0(
    ctx: &mut EmitContext,
    intrinsic_name: &[u8],
) -> Result<LLVMValueRef, TranslateError> {
    let builder = ctx.builder.get();
    let function_type = get_llvm_function_type(ctx, llvm::void_type(ctx.context), iter::empty())?;
    let mut function_value =
        unsafe { LLVMGetNamedFunction(ctx.module.get(), intrinsic_name.as_ptr() as _) };
    if function_value == ptr::null_mut() {
        function_value = unsafe {
            LLVMAddFunction(
                ctx.module.get(),
                intrinsic_name.as_ptr() as _,
                function_type,
            )
        };
    }
    Ok(unsafe {
        LLVMBuildCall2(
            builder,
            function_type,
            function_value,
            ptr::null_mut(),
            0,
            LLVM_UNNAMED,
        )
    })
}

fn emit_intrinsic_arg2(
    ctx: &mut EmitContext,
    (dst_type, dst): (ast::ScalarType, Option<Id>),
    (src_type, src_space, src): (ast::ScalarType, ast::StateSpace, impl GetLLVMValue),
    intrinsic_name: &[u8],
) -> Result<LLVMValueRef, TranslateError> {
    let builder = ctx.builder.get();
    let mut llvm_src = src.get_llvm_value(&mut ctx.names)?;
    let dst_type = get_llvm_type(ctx, &ast::Type::Scalar(dst_type))?;
    let function_type = get_llvm_function_type(
        ctx,
        dst_type,
        iter::once((&ast::Type::Scalar(src_type), src_space)),
    )?;
    let mut function_value =
        unsafe { LLVMGetNamedFunction(ctx.module.get(), intrinsic_name.as_ptr() as _) };
    if function_value == ptr::null_mut() {
        function_value = unsafe {
            LLVMAddFunction(
                ctx.module.get(),
                intrinsic_name.as_ptr() as _,
                function_type,
            )
        };
    }
    Ok(ctx.names.register_result_option(dst, |dst| unsafe {
        LLVMBuildCall2(
            builder,
            function_type,
            function_value,
            &mut llvm_src,
            1,
            dst,
        )
    }))
}

fn emit_intrinsic_arg3(
    ctx: &mut EmitContext,
    (dst_type, dst): (LLVMTypeRef, Option<Id>),
    (src1_type, src1): (ast::ScalarType, impl GetLLVMValue),
    (src2_type, src2): (ast::ScalarType, impl GetLLVMValue),
    intrinsic_name: &[u8],
) -> Result<LLVMValueRef, TranslateError> {
    let builder = ctx.builder.get();
    let function_type = get_llvm_function_type(
        ctx,
        dst_type,
        [
            (&ast::Type::Scalar(src1_type), ast::StateSpace::Reg),
            (&ast::Type::Scalar(src2_type), ast::StateSpace::Reg),
        ]
        .iter()
        .copied(),
    )?;
    let mut function_value =
        unsafe { LLVMGetNamedFunction(ctx.module.get(), intrinsic_name.as_ptr() as _) };
    if function_value == ptr::null_mut() {
        function_value = unsafe {
            LLVMAddFunction(
                ctx.module.get(),
                intrinsic_name.as_ptr() as _,
                function_type,
            )
        };
    }
    let src1 = src1.get_llvm_value(&mut ctx.names)?;
    let src2 = src2.get_llvm_value(&mut ctx.names)?;
    let mut args = [src1, src2];
    Ok(ctx.names.register_result_option(dst, |dst_name| unsafe {
        LLVMBuildCall2(
            builder,
            function_type,
            function_value,
            args.as_mut_ptr(),
            2,
            dst_name,
        )
    }))
}

fn emit_inst_bra(
    ctx: &mut EmitContext,
    args: &ast::Arg1<crate::translate::ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let label = ctx.names.value(args.src)?;
    unsafe { LLVMBuildBr(ctx.builder.get(), LLVMValueAsBasicBlock(label)) };
    Ok(())
}

fn emit_inst_atom_cas(
    ctx: &mut EmitContext,
    details: &ast::AtomCasDetails,
    args: &ast::Arg4<crate::translate::ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let ptr = ctx.names.value(args.src1)?;
    let cmp = ctx.names.value(args.src2)?;
    let new = ctx.names.value(args.src3)?;
    let success_ordering = get_llvm_ordering(details.semantics);
    // https://llvm.org/docs/LangRef.html#cmpxchg-instruction
    // "the failure ordering cannot be either release or acq_rel"
    let failure_ordering = match success_ordering {
        LLVMAtomicOrdering::LLVMAtomicOrderingRelease
        | LLVMAtomicOrdering::LLVMAtomicOrderingAcquireRelease => {
            LLVMAtomicOrdering::LLVMAtomicOrderingAcquire
        }
        o => o,
    };
    let scope = get_llvm_scope(details.scope);
    let aggregate_value = unsafe {
        LLVMZludaBuildAtomicCmpXchg(
            builder,
            ptr,
            cmp,
            new,
            scope.as_ptr() as _,
            success_ordering,
            failure_ordering,
            details.typ.size_of() as u32,
        )
    };
    // cmpxchg yields a struct { ty, i1 }, we still need to extract the result
    ctx.names.register_result(args.dst, |dst_name| unsafe {
        LLVMBuildExtractValue(builder, aggregate_value, 0, dst_name)
    });
    Ok(())
}

fn emit_inst_atom(
    ctx: &mut EmitContext,
    details: &ast::AtomDetails,
    args: &ast::Arg3<crate::translate::ExpandedArgParams>,
) -> Result<(), TranslateError> {
    emit_inst_atom_impl(ctx, details, Some(args.dst), args.src1, args.src2)
}

fn emit_inst_atom_impl(
    ctx: &mut EmitContext,
    details: &ast::AtomDetails,
    dst: Option<Id>,
    src1: Id,
    src2: Id,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let (atomic_op, type_) = match details.inner {
        ast::AtomInnerDetails::Bit {
            op: ast::AtomBitOp::And,
            typ,
        } => (LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpAnd, typ),
        ast::AtomInnerDetails::Bit {
            op: ast::AtomBitOp::Exchange,
            typ,
        } => (LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpXchg, typ),
        ast::AtomInnerDetails::Bit {
            op: ast::AtomBitOp::Or,
            typ,
        } => (LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpOr, typ),
        ast::AtomInnerDetails::Bit {
            op: ast::AtomBitOp::Xor,
            typ,
        } => (LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpXor, typ),
        ast::AtomInnerDetails::Unsigned {
            op: ast::AtomUIntOp::Add,
            typ,
        }
        | ast::AtomInnerDetails::Signed {
            op: ast::AtomSIntOp::Add,
            typ,
        } => (LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpAdd, typ),
        ast::AtomInnerDetails::Unsigned {
            op: ast::AtomUIntOp::Max,
            typ,
        } => (LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpUMax, typ),
        ast::AtomInnerDetails::Signed {
            op: ast::AtomSIntOp::Max,
            typ,
        } => (LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpMax, typ),
        ast::AtomInnerDetails::Unsigned {
            op: ast::AtomUIntOp::Min,
            typ,
        } => (LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpUMin, typ),
        ast::AtomInnerDetails::Signed {
            op: ast::AtomSIntOp::Min,
            typ,
        } => (LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpMin, typ),
        ast::AtomInnerDetails::Float {
            op: ast::AtomFloatOp::Add,
            typ,
        } => (LLVMAtomicRMWBinOp::LLVMAtomicRMWBinOpFAdd, typ),
        // Converted to a function call in a compiler pass
        ast::AtomInnerDetails::Unsigned {
            op: ast::AtomUIntOp::Dec,
            ..
        }
        | ast::AtomInnerDetails::Unsigned {
            op: ast::AtomUIntOp::Inc,
            ..
        } => return Err(TranslateError::unreachable()),
    };
    let ordering = get_llvm_ordering(details.semantics);
    let scope = get_llvm_scope(details.scope);
    let ptr = ctx.names.value(src1)?;
    let val = ctx.names.value(src2)?;
    let value = unsafe {
        LLVMZludaBuildAtomicRMW(
            builder,
            atomic_op,
            ptr,
            val,
            scope.as_ptr() as _,
            ordering,
            type_.size_of() as u32,
        )
    };
    if let Some(dst) = dst {
        ctx.names.register(dst, value);
        let name = ctx.names.name(dst);
        unsafe { LLVMSetValueName2(value, name.as_ptr(), name.len()) };
    }
    Ok(())
}

// https://llvm.org/docs/AMDGPUUsage.html#memory-scopes
fn get_llvm_scope(scope: ast::MemScope) -> &'static [u8] {
    match scope {
        ast::MemScope::Cta => &b"workgroup-one-as\0"[..],
        ast::MemScope::Gpu => b"agent-one-as\0",
        ast::MemScope::Sys => b"one-as\0",
    }
}
// https://llvm.org/docs/AMDGPUUsage.html#memory-scopes
fn get_llvm_scope_for_membar(scope: ast::MemScope) -> &'static [u8] {
    match scope {
        // HACK ALERT: for reasons that I don't understand emitting workgroup fence for membar.cta
        // lead to HIP hand or X11 crashes when running XGBoost tests as of ROCm 5.7.1
        // Probably related to those two:
        // https://gitlab.freedesktop.org/drm/amd/-/issues/2145
        // https://projects.blender.org/blender/blender/issues/100353
        // TODO: change it back to "workgroup" when HIP, amdgpu or whoever is responsible for
        // the underlying issue fixes it
        ast::MemScope::Cta => &b"agent\0"[..],
        ast::MemScope::Gpu => b"agent\0",
        ast::MemScope::Sys => b"\0",
    }
}

fn get_llvm_ordering(semantics: ast::AtomSemantics) -> LLVMAtomicOrdering {
    match semantics {
        ast::AtomSemantics::Relaxed => LLVMAtomicOrdering::LLVMAtomicOrderingMonotonic,
        ast::AtomSemantics::Acquire => LLVMAtomicOrdering::LLVMAtomicOrderingAcquire,
        ast::AtomSemantics::Release => LLVMAtomicOrdering::LLVMAtomicOrderingRelease,
        ast::AtomSemantics::AcquireRelease => LLVMAtomicOrdering::LLVMAtomicOrderingAcquireRelease,
    }
}

fn emit_inst_mov(
    ctx: &mut EmitContext,
    details: &ast::MovDetails,
    args: &ast::Arg2Mov<crate::translate::ExpandedArgParams>,
) -> Result<(), TranslateError> {
    emit_value_copy(ctx, &details.typ, ctx.names.value(args.src)?, args.dst)
}

fn emit_inst_ret(ctx: &mut EmitContext) {
    unsafe { LLVMBuildRetVoid(ctx.builder.get()) };
}

fn emit_inst_st(
    ctx: &mut EmitContext,
    details: &ast::StData,
    args: &ast::Arg2St<crate::translate::ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let val = ctx.names.value(args.src2)?;
    let ptr = ctx.names.value(args.src1)?;
    if !matches!(
        details.qualifier,
        ast::LdStQualifier::Weak | ast::LdStQualifier::Volatile
    ) {
        if let ast::Type::Vector(..) = details.typ {
            let integer_layout = details.typ.layout();
            let integer_type = unsafe {
                LLVMIntTypeInContext(ctx.context.get(), integer_layout.size() as u32 * u8::BITS)
            };
            let pointer_type = unsafe {
                LLVMPointerType(
                    integer_type,
                    get_llvm_address_space(&ctx.constants, details.state_space)?,
                )
            };
            let integer_value =
                unsafe { LLVMBuildBitCast(builder, val, integer_type, LLVM_UNNAMED) };
            let pointer_cast_value =
                unsafe { LLVMBuildPointerCast(builder, ptr, pointer_type, LLVM_UNNAMED) };
            let result = unsafe { LLVMBuildStore(builder, integer_value, pointer_cast_value) };
            unsafe { apply_qualifier(ctx.context.get(), result, details.qualifier) };
            // Weirdly, for i128 we get default alignment 8
            unsafe { LLVMSetAlignment(result, integer_layout.align() as u32) };
            return Ok(());
        }
    }
    let result = unsafe { LLVMBuildStore(builder, val, ptr) };
    unsafe { apply_qualifier(ctx.context.get(), result, details.qualifier) };
    Ok(())
}

unsafe fn apply_qualifier(ctx: LLVMContextRef, value: LLVMValueRef, qualifier: ast::LdStQualifier) {
    unsafe fn apply_qualifier_atomic(
        ctx: LLVMContextRef,
        value: LLVMValueRef,
        ordering: LLVMAtomicOrdering,
        scope: ast::MemScope,
    ) {
        let scope = get_llvm_scope(scope);
        LLVMZludaSetAtomic(ctx, value, ordering, scope.as_ptr().cast());
    }
    match qualifier {
        ast::LdStQualifier::Weak => {}
        ast::LdStQualifier::Volatile => LLVMSetVolatile(value, 1),
        ast::LdStQualifier::Relaxed(scope) => apply_qualifier_atomic(
            ctx,
            value,
            LLVMAtomicOrdering::LLVMAtomicOrderingMonotonic,
            scope,
        ),
        ast::LdStQualifier::Acquire(scope) => apply_qualifier_atomic(
            ctx,
            value,
            LLVMAtomicOrdering::LLVMAtomicOrderingAcquire,
            scope,
        ),
        ast::LdStQualifier::Release(scope) => apply_qualifier_atomic(
            ctx,
            value,
            LLVMAtomicOrdering::LLVMAtomicOrderingRelease,
            scope,
        ),
    }
}

fn emit_inst_add(
    ctx: &mut EmitContext,
    details: &ast::ArithDetails,
    args: &ast::Arg3<ExpandedArgParams>,
) -> Result<(), TranslateError> {
    emit_inst_add_impl(ctx, details, args.dst, args.src1, args.src2)
}

fn emit_inst_add_impl(
    ctx: &mut EmitContext,
    details: &ast::ArithDetails,
    dst: Id,
    src1: impl GetLLVMValue,
    src2: impl GetLLVMValue,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let function = match details {
        ast::ArithDetails::Unsigned(..) | ast::ArithDetails::Signed(..) => LLVMBuildAdd,
        ast::ArithDetails::Float(..) => LLVMBuildFAdd,
    };
    let src1 = src1.get_llvm_value(&mut ctx.names)?;
    let src2 = src2.get_llvm_value(&mut ctx.names)?;
    ctx.names.register_result(dst, |dst_name| unsafe {
        function(builder, src1, src2, dst_name)
    });
    Ok(())
}

fn emit_inst_or(
    ctx: &mut EmitContext,
    args: &ast::Arg3<crate::translate::ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let src1 = ctx.names.value(args.src1)?;
    let src2 = ctx.names.value(args.src2)?;
    ctx.names.register_result(args.dst, |dst| unsafe {
        LLVMBuildOr(builder, src1, src2, dst)
    });
    Ok(())
}

fn emit_inst_and(
    ctx: &mut EmitContext,
    args: &ast::Arg3<crate::translate::ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let src1 = ctx.names.value(args.src1)?;
    let src2 = ctx.names.value(args.src2)?;
    ctx.names.register_result(args.dst, |dst| unsafe {
        LLVMBuildAnd(builder, src1, src2, dst)
    });
    Ok(())
}

fn emit_inst_ld(
    ctx: &mut EmitContext,
    details: &ast::LdDetails,
    args: &ast::Arg2Ld<crate::translate::ExpandedArgParams>,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let ptr = ctx.names.value(args.src)?;
    let vector_type = get_llvm_type(ctx, &details.typ)?;
    if !matches!(
        details.qualifier,
        ast::LdStQualifier::Weak | ast::LdStQualifier::Volatile
    ) {
        if let ast::Type::Vector(..) = details.typ {
            let integer_layout = details.typ.layout();
            let integer_type = unsafe {
                LLVMIntTypeInContext(ctx.context.get(), integer_layout.size() as u32 * u8::BITS)
            };
            let pointer_type = unsafe {
                LLVMPointerType(
                    integer_type,
                    get_llvm_address_space(&ctx.constants, details.state_space)?,
                )
            };
            let pointer_cast_value =
                unsafe { LLVMBuildPointerCast(builder, ptr, pointer_type, LLVM_UNNAMED) };
            let integer_result =
                unsafe { LLVMBuildLoad2(builder, integer_type, pointer_cast_value, LLVM_UNNAMED) };
            unsafe { apply_qualifier(ctx.context.get(), integer_result, details.qualifier) };
            // Weirdly, for i128 we get default alignment 8
            unsafe { LLVMSetAlignment(integer_result, integer_layout.align() as u32) };
            ctx.names.register_result(args.dst, |dst| unsafe {
                LLVMBuildBitCast(builder, integer_result, vector_type, dst)
            });
            return Ok(());
        }
    }
    let result = ctx.names.register_result(args.dst, |dst| unsafe {
        LLVMBuildLoad2(builder, vector_type, ptr, dst)
    });
    unsafe { apply_qualifier(ctx.context.get(), result, details.qualifier) };
    Ok(())
}

fn emit_load_var(
    ctx: &mut EmitContext,
    load: &crate::translate::LoadVarDetails,
) -> Result<(), TranslateError> {
    let builder = ctx.builder.get();
    let mut src = ctx.names.value(load.arg.src)?;
    if let Some((idx, width)) = load.member_index {
        // TODO: improve
        let vector_type = match load.typ {
            ast::Type::Scalar(scalar_t) => ast::Type::Vector(scalar_t, width),
            _ => return Err(TranslateError::mismatched_type()),
        };
        let llvm_type = get_llvm_type(ctx, &vector_type)?;
        let llvm_i32 = get_llvm_type(ctx, &ast::Type::Scalar(ast::ScalarType::U32))?;
        let zero_llvm = unsafe { LLVMConstInt(llvm_i32, 0, 0) };
        let index_llvm = unsafe { LLVMConstInt(llvm_i32, idx as _, 0) };
        let indices = [zero_llvm, index_llvm];
        src = unsafe {
            LLVMBuildInBoundsGEP2(
                builder,
                llvm_type,
                src,
                indices.as_ptr() as _,
                2,
                LLVM_UNNAMED,
            )
        };
    }
    let llvm_type = get_llvm_type(ctx, &load.typ)?;
    ctx.names.register_result(load.arg.dst, |dst| unsafe {
        LLVMBuildLoad2(builder, llvm_type, src, dst)
    });
    Ok(())
}

fn emit_store_var(
    ctx: &mut EmitContext,
    store: &crate::translate::StoreVarDetails,
) -> Result<(), TranslateError> {
    let mut ptr = ctx.names.value(store.arg.src1)?;
    if let Some(index) = store.member_index {
        let llvm_i32 = get_llvm_type(ctx, &ast::Type::Scalar(ast::ScalarType::U32))?;
        let zero_llvm = unsafe { LLVMConstInt(llvm_i32, 0, 0) };
        let index_llvm = unsafe { LLVMConstInt(llvm_i32, index as _, 0) };
        let indices = [zero_llvm, index_llvm];
        let gep_type = get_llvm_type(ctx, &store.type_)?;
        ptr = unsafe {
            LLVMBuildInBoundsGEP2(
                ctx.builder.get(),
                gep_type,
                ptr,
                indices.as_ptr() as _,
                2,
                LLVM_UNNAMED,
            )
        };
    };
    let val = ctx.names.value(store.arg.src2)?;
    unsafe { LLVMBuildStore(ctx.builder.get(), val, ptr) };
    Ok(())
}

fn emit_label(ctx: &mut EmitContext, label: Id) -> Result<(), TranslateError> {
    let new_block = unsafe { LLVMValueAsBasicBlock(ctx.names.value(label)?) };
    terminate_current_block_if_needed(ctx, new_block);
    unsafe { LLVMPositionBuilderAtEnd(ctx.builder.get(), new_block) };
    Ok(())
}

fn terminate_current_block_if_needed(ctx: &mut EmitContext, new_block: LLVMBasicBlockRef) {
    let current_block = unsafe { LLVMGetInsertBlock(ctx.builder.get()) };
    if current_block == ptr::null_mut() {
        return;
    }
    let terminator = unsafe { LLVMGetBasicBlockTerminator(current_block) };
    if terminator != ptr::null_mut() {
        return;
    }
    unsafe { LLVMBuildBr(ctx.builder.get(), new_block) };
}

fn emit_method_declaration<'input>(
    ctx: &mut EmitContext,
    method: &crate::translate::Function<'input>,
) -> Result<LLVMValueRef, TranslateError> {
    if let Some(llvm_decl) = ctx.names.try_value(method.name) {
        // We rename function args if the function was already declared, because fn
        // declaration and definition have different ids in arguments
        if method.body.is_some() {
            for (idx, input_arg) in method.input_arguments.iter().enumerate() {
                let llvm_param = unsafe { LLVMGetParam(llvm_decl, idx as u32) };
                ctx.names.register(input_arg.name, llvm_param);
                let name = ctx.names.name(input_arg.name);
                unsafe {
                    LLVMSetValueName2(llvm_param, name.as_ptr(), name.len());
                }
            }
        }
        return Ok(llvm_decl);
    }
    let name_ptr = ctx.names.name_ptr(method.name);
    let call_conv = if method.is_kernel {
        ctx.constants.kernel_callconv
    } else {
        ctx.constants.function_callconv
    };
    let input_types = method
        .input_arguments
        .iter()
        .map(|var| (&var.type_, var.state_space));
    let return_type = match &*method.return_arguments {
        [] => llvm::void_type(&ctx.context),
        [var] => get_llvm_type(ctx, &var.type_)?,
        [..] => get_llvm_type_struct(
            ctx,
            method
                .return_arguments
                .iter()
                .map(|var| Cow::Borrowed(&var.type_)),
        )?,
    };
    let fn_type = get_llvm_function_type(ctx, return_type, input_types)?;
    let llvm_func = unsafe { LLVMAddFunction(ctx.module.get(), name_ptr, fn_type) };
    ctx.names.register(method.name, llvm_func);
    unsafe { LLVMSetFunctionCallConv(llvm_func, call_conv as u32) };
    emit_denorm(ctx, method.name, llvm_func);
    emit_llvm_string_attribute(ctx, llvm_func, b"amdgpu-unsafe-fp-atomics", b"true");
    emit_llvm_string_attribute(ctx, llvm_func, b"no-trapping-math", b"true");
    emit_llvm_string_attribute(ctx, llvm_func, b"uniform-work-group-size", b"true");
    // TODO: split this into a kernel/app-profile
    if ctx.compilation_mode != CompilationMode::Wave32 {
        if let Some(ref name) = method.source_name {
            if name.contains("IN9LAMMPS_NS") {
                emit_llvm_string_attribute(
                    ctx,
                    llvm_func,
                    b"amdgpu-flat-work-group-size",
                    b"1,512",
                );
            }
        }
    }
    for (idx, input_arg) in method.input_arguments.iter().enumerate() {
        let llvm_param = unsafe { LLVMGetParam(llvm_func, idx as u32) };
        ctx.names.register(input_arg.name, llvm_param);
        let name = ctx.names.name(input_arg.name);
        unsafe {
            LLVMSetValueName2(llvm_param, name.as_ptr(), name.len());
        }
        let llvm_param = unsafe { LLVMGetParam(llvm_func, idx as u32) };
        if method.is_kernel {
            unsafe {
                if let Some(align) = input_arg.align {
                    LLVMSetParamAlignment(llvm_param, align);
                }
                let attr_kind =
                    LLVMGetEnumAttributeKindForName(b"byref".as_ptr().cast(), b"byref".len());
                if attr_kind == 0 {
                    panic!();
                }
                let attr = LLVMCreateTypeAttribute(
                    ctx.context.get(),
                    attr_kind,
                    get_llvm_type(ctx, &input_arg.type_)?,
                );
                LLVMAddAttributeAtIndex(llvm_func, idx as u32 + 1, attr);
            }
        }
    }
    Ok(llvm_func)
}

fn emit_denorm(ctx: &mut EmitContext, method_name: Id, llvm_func: *mut LLVMValue) {
    let maybe_denorm_summary = ctx.denorm_statistics.get(&method_name).copied();
    if let Some(denorm_summary) = maybe_denorm_summary {
        emit_denorm_attribute(ctx, llvm_func, b"denormal-fp-math", denorm_summary.f16f64);
        emit_denorm_attribute(ctx, llvm_func, b"denormal-fp-math-f32", denorm_summary.f32);
    }
}

fn emit_denorm_attribute(
    ctx: &mut EmitContext,
    new_llvm_func: LLVMValueRef,
    denorm_key: &'static [u8],
    denorm_value: FPDenormMode,
) {
    let denorm_mode_value = match denorm_value {
        FPDenormMode::FlushToZero => &b"preserve-sign,preserve-sign"[..],
        FPDenormMode::Preserve => b"ieee,ieee",
    };
    emit_llvm_string_attribute(ctx, new_llvm_func, denorm_key, denorm_mode_value);
}

fn emit_llvm_string_attribute(
    ctx: &mut EmitContext,
    llvm_func: LLVMValueRef,
    key: &[u8],
    value: &[u8],
) {
    let denorm_attr = unsafe {
        LLVMCreateStringAttribute(
            ctx.context.get(),
            key.as_ptr() as _,
            key.len() as u32,
            value.as_ptr() as _,
            value.len() as u32,
        )
    };
    unsafe { LLVMAddAttributeAtIndex(llvm_func, LLVMAttributeFunctionIndex, denorm_attr) };
}

fn get_llvm_type(ctx: &EmitContext, type_: &ast::Type) -> Result<LLVMTypeRef, TranslateError> {
    unsafe {
        Ok(match type_ {
            ast::Type::Scalar(scalar) => match scalar {
                ast::ScalarType::B8 | ast::ScalarType::U8 | ast::ScalarType::S8 => {
                    LLVMInt8TypeInContext(ctx.context.get())
                }
                ast::ScalarType::B16 | ast::ScalarType::U16 | ast::ScalarType::S16 => {
                    LLVMInt16TypeInContext(ctx.context.get())
                }
                ast::ScalarType::B32 | ast::ScalarType::U32 | ast::ScalarType::S32 => {
                    LLVMInt32TypeInContext(ctx.context.get())
                }
                ast::ScalarType::B64 | ast::ScalarType::U64 | ast::ScalarType::S64 => {
                    LLVMInt64TypeInContext(ctx.context.get())
                }
                ast::ScalarType::F16 => LLVMHalfTypeInContext(ctx.context.get()),
                ast::ScalarType::F32 => LLVMFloatTypeInContext(ctx.context.get()),
                ast::ScalarType::F64 => LLVMDoubleTypeInContext(ctx.context.get()),
                ast::ScalarType::F16x2 => LLVMVectorType(
                    get_llvm_type(ctx, &ast::Type::Scalar(ast::ScalarType::F16))?,
                    2,
                ),
                ast::ScalarType::Pred => LLVMInt1TypeInContext(ctx.context.get()),
            },
            ast::Type::Vector(scalar_type, count) => LLVMVectorType(
                get_llvm_type(ctx, &ast::Type::Scalar(*scalar_type))?,
                *count as u32,
            ),
            ast::Type::Array(scalar_type, dims) => {
                let llvm_type_scalar = get_llvm_type(ctx, &ast::Type::Scalar(*scalar_type))?;
                get_llvm_array_type(llvm_type_scalar, &*dims)
            }
            ast::Type::Pointer(scalar_type, state_space) => {
                get_llvm_pointer_type(ctx, &ast::Type::Scalar(*scalar_type), *state_space)?
            }
            ast::Type::Texref | ast::Type::Surfref => ctx.texref_underlying_type,
            ast::Type::Struct(fields) => get_llvm_type_struct(
                ctx,
                fields
                    .iter()
                    .map(|struct_field| Cow::Owned(struct_field.to_type())),
            )?,
        })
    }
}

fn get_anonymous_struct_type<const N: usize>(
    ctx: &EmitContext,
    fields: [ast::ScalarType; N],
) -> Result<LLVMTypeRef, TranslateError> {
    let mut fields = IntoIterator::into_iter(fields)
        .map(|scalar| get_llvm_type(ctx, &ast::Type::Scalar(scalar)))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(unsafe {
        LLVMStructTypeInContext(
            ctx.context.get(),
            fields.as_mut_ptr(),
            fields.len() as u32,
            0,
        )
    })
}

fn get_llvm_pointer_type(
    ctx: &EmitContext,
    type_: &ast::Type,
    state_space: ast::StateSpace,
) -> Result<LLVMTypeRef, TranslateError> {
    let underlying_dst_type = get_llvm_type(ctx, type_)?;
    let address_space = get_llvm_address_space(&ctx.constants, state_space)?;
    Ok(unsafe { LLVMPointerType(underlying_dst_type, address_space) })
}

fn get_llvm_function_type<'a>(
    ctx: &mut EmitContext,
    return_type: LLVMTypeRef,
    param_types: impl Iterator<Item = (&'a ast::Type, ast::StateSpace)>,
) -> Result<LLVMTypeRef, TranslateError> {
    let mut llvm_types = param_types
        .map(|(type_, space)| get_method_input_type(ctx, type_, space))
        .collect::<Result<Vec<_>, _>>()?;
    unsafe {
        Ok(LLVMFunctionType(
            return_type,
            llvm_types.as_mut_ptr(),
            llvm_types.len() as u32,
            0,
        ))
    }
}

fn get_method_input_type(
    ctx: &mut EmitContext,
    type_: &ast::Type,
    state_space: ast::StateSpace,
) -> Result<LLVMTypeRef, TranslateError> {
    if state_space != ast::StateSpace::Reg {
        get_llvm_pointer_type(ctx, type_, state_space)
    } else {
        get_llvm_type(ctx, type_)
    }
}

fn get_llvm_address_space(
    constants: &Constants,
    state_space: ast::StateSpace,
) -> Result<u32, TranslateError> {
    Ok(match state_space {
        ast::StateSpace::Reg => constants.private_space,
        ast::StateSpace::Const => constants.constant_space,
        ast::StateSpace::Global => constants.global_space,
        ast::StateSpace::Local => constants.private_space,
        ast::StateSpace::Shared => constants.shared_space,
        // Should be removed by deparamize passes during translation
        ast::StateSpace::Param => return Err(TranslateError::unreachable()),
        ast::StateSpace::Generic => constants.generic_space,
        ast::StateSpace::Sreg => constants.private_space,
    })
}

unsafe fn get_llvm_array_type(inner_type: LLVMTypeRef, dims: &[u32]) -> LLVMTypeRef {
    match dims.split_last() {
        Some((dim, dims)) => {
            let current_array_type = LLVMArrayType(inner_type, *dim);
            if dims.len() > 0 {
                get_llvm_array_type(current_array_type, dims)
            } else {
                current_array_type
            }
        }
        None => LLVMArrayType(inner_type, 0),
    }
}

// That is not spelled explicitly in LLVM LangRef, but the difference between linkage and visibility:
// * Linkage applies during LLVM linking step, is visible in the single LLVM IR module
// * Visibility appplies post-LLVM linking, is visible in the resulting ELF binary
// So e.g. difference between private linkage and hidden visibility is:
// * With private linkage, symbol is removed during LLVM linking phase and just not present from that point on
// * With hidden visibility, symbol survives LLVM linking, but is emitted into ELF with HIDDEN visibility
fn emit_linkage_for_variable(
    space: ast::StateSpace,
    value: LLVMValueRef,
    globally_visible: bool,
) -> Result<(), TranslateError> {
    let (visibility, linking) = if globally_visible {
        (
            if space == ast::StateSpace::Shared {
                LLVMVisibility::LLVMHiddenVisibility
            } else {
                LLVMVisibility::LLVMProtectedVisibility
            },
            LLVMLinkage::LLVMExternalLinkage,
        )
    } else {
        (
            // Local (private or internal linkage) requires default visibility
            LLVMVisibility::LLVMDefaultVisibility,
            LLVMLinkage::LLVMPrivateLinkage,
        )
    };
    unsafe { LLVMSetLinkage(value, linking) };
    unsafe { LLVMSetVisibility(value, visibility) };
    Ok(())
}

fn emit_linkage_for_method<'input>(
    method: &crate::translate::Function<'input>,
    is_kernel: bool,
    value: LLVMValueRef,
) {
    // There's a little bit of mismatch between ZLUDA and LLVM view of the world.
    // ZLUDA can have a function declared multiple times (and defined once) whereas
    // in LLVM a function can be either defined or declared (and only once). Additionally,
    // LLVM declarations are always global and external. Which are defaults when creating
    // LLVM function objects
    // For this reason we don't emit linkage&visibility for pure declarations
    if method.body.is_none() {
        return;
    }
    let (visibility, linking) =
        if method.special_raytracing_linking || (is_kernel && method.source_name.is_some()) {
            (
                LLVMVisibility::LLVMProtectedVisibility,
                LLVMLinkage::LLVMExternalLinkage,
            )
        } else {
            (
                // Local (private or internal linkage) requires default visibility
                LLVMVisibility::LLVMDefaultVisibility,
                LLVMLinkage::LLVMPrivateLinkage,
            )
        };
    unsafe { LLVMSetLinkage(value, linking) };
    unsafe { LLVMSetVisibility(value, visibility) };
}

trait GetLLVMValue {
    fn get_llvm_value(self, names: &mut NamedIdGenerator) -> Result<LLVMValueRef, TranslateError>;
}

impl GetLLVMValue for LLVMValueRef {
    fn get_llvm_value(self, _names: &mut NamedIdGenerator) -> Result<LLVMValueRef, TranslateError> {
        Ok(self)
    }
}

impl GetLLVMValue for Id {
    fn get_llvm_value(self, names: &mut NamedIdGenerator) -> Result<LLVMValueRef, TranslateError> {
        names.value(self)
    }
}

impl ast::ScalarType {
    fn llvm_display(self) -> ScalarTypeLLVMDisplay {
        ScalarTypeLLVMDisplay(self)
    }
}

struct ScalarTypeLLVMDisplay(ast::ScalarType);

impl Display for ScalarTypeLLVMDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            ast::ScalarType::B8 | ast::ScalarType::U8 | ast::ScalarType::S8 => write!(f, "i8"),
            ast::ScalarType::B16 | ast::ScalarType::U16 | ast::ScalarType::S16 => write!(f, "i16"),
            ast::ScalarType::B32 | ast::ScalarType::U32 | ast::ScalarType::S32 => write!(f, "i32"),
            ast::ScalarType::B64 | ast::ScalarType::U64 | ast::ScalarType::S64 => write!(f, "i64"),
            ast::ScalarType::F16 => write!(f, "f16"),
            ast::ScalarType::F32 => write!(f, "f32"),
            ast::ScalarType::F64 => write!(f, "f64"),
            ast::ScalarType::F16x2 => write!(f, "v2f16"),
            ast::ScalarType::Pred => write!(f, "i1"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NamedIdGenerator;
    use crate::translate::IdGenerator;
    use rustc_hash::FxHashMap;
    use std::{mem, slice};

    #[test]
    fn name_cache_returns_correct_result_for_first_10002_ids() {
        let mut id_gen = IdGenerator::new();
        let ids = id_gen.reserve(10002);
        let names = NamedIdGenerator::new(id_gen, &FxHashMap::default(), &[]);
        for id in ids {
            let mut reference = id.get().to_string();
            reference.push('\0');
            let ptr = names.name_ptr(id) as *const u8;
            let computed = unsafe { slice::from_raw_parts(ptr as _, reference.len()) };
            assert_eq!(reference.as_bytes(), computed);
            let computed_slice = names.name(id);
            assert_eq!(reference.as_bytes().split_last().unwrap().1, unsafe {
                mem::transmute::<_, &[u8]>(computed_slice)
            });
        }
    }
}
