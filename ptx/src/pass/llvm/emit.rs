// We use Raw LLVM-C bindings here because using inkwell is just not worth it.
// Specifically the issue is with builder functions. We maintain the mapping
// between ZLUDA identifiers and LLVM values. When using inkwell, LLVM values
// are kept as instances `AnyValueEnum`. Now look at the signature of
// `Builder::build_int_add(...)`:
//   pub fn build_int_add<T: IntMathValue<'ctx>>(&self, lhs: T, rhs: T, name: &str, ) -> Result<T, BuilderError>
// At this point both lhs and rhs are `AnyValueEnum`. To call
// `build_int_add(...)` we would have to do something like this:
//   if let (Ok(lhs), Ok(rhs)) = (lhs.as_int(), rhs.as_int()) {
//       builder.build_int_add(lhs, rhs, dst)?;
//   } else if let (Ok(lhs), Ok(rhs)) = (lhs.as_pointer(), rhs.as_pointer()) {
//      builder.build_int_add(lhs, rhs, dst)?;
//   } else if let (Ok(lhs), Ok(rhs)) = (lhs.as_vector(), rhs.as_vector()) {
//       builder.build_int_add(lhs, rhs, dst)?;
//   } else {
//       return Err(error_unrachable());
//   }
// while with plain LLVM-C it's just:
//   unsafe { LLVMBuildAdd(builder, lhs, rhs, dst) };

// AMDGPU LLVM backend support for llvm.experimental.constrained.* is incomplete.
// Emitting @llvm.experimental.constrained.fdiv.f32(...) makes LLVm fail with
// "LLVM ERROR: unsupported libcall legalization". Running with "-mllvm -print-before-all"
// shows it fails inside amdgpu-isel. You can get a little bit furthr with "-mllvm -global-isel",
// but it will too fail similarly, but with "unable to legalize instruction"

use std::ffi::{CStr, NulError};
use std::{i8, ptr, u64};

use super::*;
use crate::pass::*;
use llvm_zluda::{core::*, *};
use llvm_zluda::{prelude::*, LLVMZludaBuildAtomicRMW};
use llvm_zluda::{LLVMCallConv, LLVMZludaBuildAlloca};
use ptx_parser::{CpAsyncArgs, CpAsyncDetails, FunnelShiftMode, Mul24Control, ShfArgs};

struct Builder(LLVMBuilderRef);

impl Builder {
    fn new(ctx: &Context) -> Self {
        Self::new_raw(ctx.get())
    }

    fn new_raw(ctx: LLVMContextRef) -> Self {
        Self(unsafe { LLVMCreateBuilderInContext(ctx) })
    }

    fn get(&self) -> LLVMBuilderRef {
        self.0
    }
}

impl Drop for Builder {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.0);
        }
    }
}

pub(crate) fn run<'input>(
    context: &Context,
    id_defs: GlobalStringIdentResolver2<'input>,
    directives: Vec<Directive2<ast::Instruction<SpirvWord>, SpirvWord>>,
) -> Result<llvm::Module, TranslateError> {
    let module = llvm::Module::new(context, LLVM_UNNAMED);
    let mut emit_ctx = ModuleEmitContext::new(context, &module, &id_defs);
    for directive in directives {
        match directive {
            Directive2::Variable(linking, variable) => emit_ctx.emit_global(linking, variable)?,
            Directive2::Method(method) => emit_ctx.emit_method(method)?,
        }
    }
    if cfg!(debug_assertions) {
        if let Err(err) = module.verify() {
            panic!("{:?}", err);
        }
    }
    Ok(module)
}

struct ModuleEmitContext<'a, 'input> {
    context: LLVMContextRef,
    module: LLVMModuleRef,
    builder: Builder,
    id_defs: &'a GlobalStringIdentResolver2<'input>,
    resolver: ResolveIdent,
}

impl<'a, 'input> ModuleEmitContext<'a, 'input> {
    fn new(
        context: &Context,
        module: &llvm::Module,
        id_defs: &'a GlobalStringIdentResolver2<'input>,
    ) -> Self {
        ModuleEmitContext {
            context: context.get(),
            module: module.get(),
            builder: Builder::new(context),
            id_defs,
            resolver: ResolveIdent::new(&id_defs),
        }
    }

    fn kernel_call_convention() -> u32 {
        LLVMCallConv::LLVMAMDGPUKERNELCallConv as u32
    }

    fn func_call_convention() -> u32 {
        LLVMCallConv::LLVMCCallConv as u32
    }

    fn emit_method(
        &mut self,
        method: Function2<ast::Instruction<SpirvWord>, SpirvWord>,
    ) -> Result<(), TranslateError> {
        let name = method
            .import_as
            .as_deref()
            .or_else(|| self.id_defs.ident_map[&method.name].name.as_deref())
            .ok_or_else(|| error_unreachable())?;
        let name = CString::new(name).map_err(|_| error_unreachable())?;
        let mut fn_ = unsafe { LLVMGetNamedFunction(self.module, name.as_ptr()) };
        if fn_ == ptr::null_mut() {
            let fn_type = get_function_type(
                self.context,
                method.return_arguments.iter().map(|v| &v.info.v_type),
                method.input_arguments.iter().map(|v| {
                    get_input_argument_type(self.context, &v.info.v_type, v.info.state_space)
                }),
            )?;
            fn_ = unsafe { LLVMAddFunction(self.module, name.as_ptr(), fn_type) };
            self.emit_fn_attribute(fn_, "amdgpu-unsafe-fp-atomics", "true");
            self.emit_fn_attribute(fn_, "uniform-work-group-size", "true");
            self.emit_fn_attribute(fn_, "no-trapping-math", "true");
        }
        if !method.is_kernel {
            self.resolver.register(method.name, fn_);
            self.emit_fn_attribute(fn_, "denormal-fp-math-f32", "dynamic");
            self.emit_fn_attribute(fn_, "denormal-fp-math", "dynamic");
        } else {
            self.emit_fn_attribute(
                fn_,
                "denormal-fp-math-f32",
                llvm_ftz(method.flush_to_zero_f32),
            );
            self.emit_fn_attribute(
                fn_,
                "denormal-fp-math",
                llvm_ftz(method.flush_to_zero_f16f64),
            );
        }
        self.emit_fn_attribute(fn_, "amdgpu-ieee", "false");
        for (i, param) in method.input_arguments.iter().enumerate() {
            let value = unsafe { LLVMGetParam(fn_, i as u32) };
            let name = self.resolver.get_or_add(param.name);
            if let Some(align) = param.info.align {
                unsafe { LLVMSetParamAlignment(value, align) };
            }
            unsafe { LLVMSetValueName2(value, name.as_ptr().cast(), name.len()) };
            self.resolver.register(param.name, value);
            if method.is_kernel {
                let attr_kind = unsafe {
                    LLVMGetEnumAttributeKindForName(b"byref".as_ptr().cast(), b"byref".len())
                };
                let attr = unsafe {
                    LLVMCreateTypeAttribute(
                        self.context,
                        attr_kind,
                        get_type(self.context, &param.info.v_type)?,
                    )
                };
                unsafe { LLVMAddAttributeAtIndex(fn_, i as u32 + 1, attr) };
            }
        }
        if !method.is_kernel {
            unsafe {
                LLVMSetVisibility(fn_, llvm_zluda::LLVMVisibility::LLVMHiddenVisibility);
            }
        }
        let call_conv = if method.is_kernel {
            Self::kernel_call_convention()
        } else {
            Self::func_call_convention()
        };
        unsafe { LLVMSetFunctionCallConv(fn_, call_conv) };
        if let Some(statements) = method.body {
            let variables_bb =
                unsafe { LLVMAppendBasicBlockInContext(self.context, fn_, LLVM_UNNAMED.as_ptr()) };
            let variables_builder = Builder::new_raw(self.context);
            unsafe { LLVMPositionBuilderAtEnd(variables_builder.get(), variables_bb) };
            let real_bb =
                unsafe { LLVMAppendBasicBlockInContext(self.context, fn_, LLVM_UNNAMED.as_ptr()) };
            unsafe { LLVMPositionBuilderAtEnd(self.builder.get(), real_bb) };
            let mut method_emitter = MethodEmitContext::new(self, fn_, variables_builder);
            for var in method.return_arguments {
                method_emitter.emit_variable(var)?;
            }
            for statement in statements.iter() {
                if let Statement::Label(label) = statement {
                    method_emitter.emit_label_initial(*label);
                }
            }
            let mut statements = statements.into_iter();
            if let Some(Statement::Label(label)) = statements.next() {
                method_emitter.emit_label_delayed(label)?;
            } else {
                return Err(error_unreachable());
            }
            method_emitter.emit_kernel_rounding_prelude(
                method.is_kernel,
                method.rounding_mode_f32,
                method.rounding_mode_f16f64,
            )?;
            for statement in statements {
                method_emitter.emit_statement(statement)?;
            }
            unsafe { LLVMBuildBr(method_emitter.variables_builder.get(), real_bb) };
        }
        Ok(())
    }

    fn emit_global(
        &mut self,
        _linking: ast::LinkingDirective,
        var: ast::Variable<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let name = self
            .id_defs
            .ident_map
            .get(&var.name)
            .map(|entry| {
                entry
                    .name
                    .as_ref()
                    .map(|text| Ok::<_, NulError>(Cow::Owned(CString::new(&**text)?)))
            })
            .flatten()
            .transpose()
            .map_err(|_| error_unreachable())?
            .unwrap_or(Cow::Borrowed(LLVM_UNNAMED));
        let global = unsafe {
            LLVMAddGlobalInAddressSpace(
                self.module,
                get_type(self.context, &var.info.v_type)?,
                name.as_ptr(),
                get_state_space(var.info.state_space)?,
            )
        };
        self.resolver.register(var.name, global);
        if let Some(align) = var.info.align {
            unsafe { LLVMSetAlignment(global, align) };
        }
        if !var.info.array_init.is_empty() {
            let initializer = self.get_array_init(&var.info.v_type, &*var.info.array_init)?;
            unsafe { LLVMSetInitializer(global, initializer) };
        }
        Ok(())
    }

    fn get_array_init(
        &self,
        type_: &ast::Type,
        array_init: &[ast::RegOrImmediate<SpirvWord>],
    ) -> Result<*mut LLVMValue, TranslateError> {
        let initializer = match type_ {
            ast::Type::Array(None, scalar, dimensions) => {
                if dimensions.len() != 1 {
                    todo!()
                }
                if dimensions[0] as usize != array_init.len() {
                    return Err(error_unreachable());
                }
                let type_ = get_scalar_type(self.context, *scalar);
                let mut elements = array_init
                    .iter()
                    .map(|elem| match elem {
                        ast::RegOrImmediate::Reg(reg) => {
                            Ok(unsafe { LLVMConstPtrToInt(self.resolver.value(*reg)?, type_) })
                        }
                        ast::RegOrImmediate::Imm(imm) => {
                            Ok(get_immediate_value(self.context, scalar, imm))
                        }
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                unsafe { LLVMConstArray2(type_, elements.as_mut_ptr(), elements.len() as u64) }
            }
            ast::Type::Scalar(scalar) => {
                let initializer = match array_init {
                    [ast::RegOrImmediate::Imm(init)] => init,
                    _ => return Err(error_mismatched_type()),
                };
                get_immediate_value(self.context, scalar, initializer)
            }
            _ => {
                todo!()
            }
        };
        Ok(initializer)
    }

    fn emit_fn_attribute(&self, llvm_object: LLVMValueRef, key: &str, value: &str) {
        let attribute = unsafe {
            LLVMCreateStringAttribute(
                self.context,
                key.as_ptr() as _,
                key.len() as u32,
                value.as_ptr() as _,
                value.len() as u32,
            )
        };
        unsafe { LLVMAddAttributeAtIndex(llvm_object, LLVMAttributeFunctionIndex, attribute) };
    }
}

fn get_immediate_value(
    context: LLVMContextRef,
    scalar_type: &ast::ScalarType,
    imm: &ast::ImmediateValue,
) -> *mut LLVMValue {
    let type_ = get_scalar_type(context, *scalar_type);
    match imm {
        ast::ImmediateValue::U64(x) => unsafe { LLVMConstInt(type_, *x, 0) },
        ast::ImmediateValue::S64(x) => unsafe { LLVMConstInt(type_, *x as u64, 0) },
        ast::ImmediateValue::F32(x) => unsafe { LLVMConstReal(type_, *x as f64) },
        ast::ImmediateValue::F64(x) => unsafe { LLVMConstReal(type_, *x) },
    }
}

fn llvm_ftz(ftz: bool) -> &'static str {
    if ftz {
        "preserve-sign"
    } else {
        "ieee"
    }
}

fn get_input_argument_type(
    context: LLVMContextRef,
    v_type: &ast::Type,
    state_space: ast::StateSpace,
) -> Result<LLVMTypeRef, TranslateError> {
    match state_space {
        ast::StateSpace::ParamEntry | ast::StateSpace::Shared => {
            Ok(unsafe { LLVMPointerTypeInContext(context, get_state_space(state_space)?) })
        }
        ast::StateSpace::Reg => get_type(context, v_type),
        _ => return Err(error_unreachable()),
    }
}

struct MethodEmitContext<'a> {
    context: LLVMContextRef,
    module: LLVMModuleRef,
    method: LLVMValueRef,
    builder: LLVMBuilderRef,
    variables_builder: Builder,
    resolver: &'a mut ResolveIdent,
}

impl<'a> MethodEmitContext<'a> {
    fn new(
        parent: &'a mut ModuleEmitContext,
        method: LLVMValueRef,
        variables_builder: Builder,
    ) -> MethodEmitContext<'a> {
        MethodEmitContext {
            context: parent.context,
            module: parent.module,
            builder: parent.builder.get(),
            variables_builder,
            resolver: &mut parent.resolver,
            method,
        }
    }

    fn emit_statement(
        &mut self,
        statement: Statement<ast::Instruction<SpirvWord>, SpirvWord>,
    ) -> Result<(), TranslateError> {
        Ok(match statement {
            Statement::Variable(var) => self.emit_variable(var)?,
            Statement::Label(label) => self.emit_label_delayed(label)?,
            Statement::Instruction(inst) => self.emit_instruction(inst)?,
            Statement::Conditional(cond) => self.emit_conditional(cond)?,
            Statement::Conversion(conversion) => self.emit_conversion(conversion)?,
            Statement::Constant(constant) => self.emit_constant(constant)?,
            Statement::RetValue(_, values) => self.emit_ret_value(values)?,
            Statement::PtrAccess(ptr_access) => self.emit_ptr_access(ptr_access)?,
            Statement::RepackVector(repack) => self.emit_vector_repack(repack)?,
            Statement::FunctionPointer(_) => todo!(),
            Statement::VectorRead(vector_read) => self.emit_vector_read(vector_read)?,
            Statement::VectorWrite(vector_write) => self.emit_vector_write(vector_write)?,
            Statement::SetMode(mode_reg) => self.emit_set_mode(mode_reg)?,
            Statement::FpSaturate { dst, src, type_ } => self.emit_fp_saturate(type_, dst, src)?,
            Statement::FpModeRequired { .. } => {}
        })
    }

    // This should be a kernel attribute, but sadly AMDGPU LLVM target does
    // not support attribute for it. So we have to set it as the first
    // instruction in the body of a kernel
    fn emit_kernel_rounding_prelude(
        &mut self,
        is_kernel: bool,
        rounding_mode_f32: ast::RoundingMode,
        rounding_mode_f16f64: ast::RoundingMode,
    ) -> Result<(), TranslateError> {
        if is_kernel {
            if rounding_mode_f32 != ast::RoundingMode::NearestEven
                || rounding_mode_f16f64 != ast::RoundingMode::NearestEven
            {
                self.emit_set_mode(ModeRegister::Rounding {
                    f32: rounding_mode_f32,
                    f16f64: rounding_mode_f16f64,
                })?;
            }
        }
        Ok(())
    }

    fn emit_variable(&mut self, var: ast::Variable<SpirvWord>) -> Result<(), TranslateError> {
        let alloca = unsafe {
            LLVMZludaBuildAlloca(
                self.variables_builder.get(),
                get_type(self.context, &var.info.v_type)?,
                get_state_space(var.info.state_space)?,
                self.resolver.get_or_add_raw(var.name),
            )
        };
        self.resolver.register(var.name, alloca);
        if let Some(align) = var.info.align {
            unsafe { LLVMSetAlignment(alloca, align) };
        }
        if !var.info.array_init.is_empty() {
            return Err(error_unreachable());
        }
        Ok(())
    }

    fn emit_label_initial(&mut self, label: SpirvWord) {
        let block = unsafe {
            LLVMAppendBasicBlockInContext(
                self.context,
                self.method,
                self.resolver.get_or_add_raw(label),
            )
        };
        self.resolver
            .register(label, unsafe { LLVMBasicBlockAsValue(block) });
    }

    fn emit_label_delayed(&mut self, label: SpirvWord) -> Result<(), TranslateError> {
        let block = self.resolver.value(label)?;
        let block = unsafe { LLVMValueAsBasicBlock(block) };
        let last_block = unsafe { LLVMGetInsertBlock(self.builder) };
        if unsafe { LLVMGetBasicBlockTerminator(last_block) } == ptr::null_mut() {
            unsafe { LLVMBuildBr(self.builder, block) };
        }
        unsafe { LLVMPositionBuilderAtEnd(self.builder, block) };
        Ok(())
    }

    fn emit_instruction(
        &mut self,
        inst: ast::Instruction<SpirvWord>,
    ) -> Result<(), TranslateError> {
        match inst {
            ast::Instruction::Mov { data: _, arguments } => self.emit_mov(arguments),
            ast::Instruction::Ld { data, arguments } => self.emit_ld(data, arguments),
            ast::Instruction::Add { data, arguments } => self.emit_add(data, arguments),
            ast::Instruction::Dp4a { data, arguments } => self.emit_dp4a(data, arguments),
            ast::Instruction::St { data, arguments } => self.emit_st(data, arguments),
            ast::Instruction::Mul { data, arguments } => self.emit_mul(data, arguments),
            ast::Instruction::Mul24 { data, arguments } => self.emit_mul24(data, arguments),
            ast::Instruction::Set { data, arguments } => self.emit_set(data, arguments),
            ast::Instruction::SetBool { data, arguments } => self.emit_set_bool(data, arguments),
            ast::Instruction::Setp { data, arguments } => self.emit_setp(data, arguments),
            ast::Instruction::SetpBool { data, arguments } => self.emit_setp_bool(data, arguments),
            ast::Instruction::Not { data, arguments } => self.emit_not(data, arguments),
            ast::Instruction::Or { data, arguments } => self.emit_or(data, arguments),
            ast::Instruction::And { arguments, .. } => self.emit_and(arguments),
            ast::Instruction::Bra { arguments } => self.emit_bra(arguments),
            ast::Instruction::Call { data, arguments } => self.emit_call(data, arguments),
            ast::Instruction::Cvt { data, arguments } => self.emit_cvt(data, arguments),
            ast::Instruction::CvtPack { data, arguments } => self.emit_cvt_pack(data, arguments),
            ast::Instruction::Shf { data, arguments } => self.emit_shf(data, arguments),
            ast::Instruction::Shr { data, arguments } => self.emit_shr(data, arguments),
            ast::Instruction::Shl { data, arguments } => self.emit_shl(data, arguments),
            ast::Instruction::Ret { data } => Ok(self.emit_ret(data)),
            ast::Instruction::Cvta { data, arguments } => self.emit_cvta(data, arguments),
            ast::Instruction::Abs { data, arguments } => self.emit_abs(data, arguments),
            ast::Instruction::Mad { data, arguments } => self.emit_mad(data, arguments),
            ast::Instruction::Fma { data, arguments } => self.emit_fma(data, arguments),
            ast::Instruction::Sub { data, arguments } => self.emit_sub(data, arguments),
            ast::Instruction::Min { data, arguments } => self.emit_min(data, arguments),
            ast::Instruction::Max { data, arguments } => self.emit_max(data, arguments),
            ast::Instruction::Rcp { data, arguments } => self.emit_rcp(data, arguments),
            ast::Instruction::Sqrt { data, arguments } => self.emit_sqrt(data, arguments),
            ast::Instruction::Rsqrt { data, arguments } => self.emit_rsqrt(data, arguments),
            ast::Instruction::Selp { data, arguments } => self.emit_selp(data, arguments),
            ast::Instruction::Atom { data, arguments } => self.emit_atom(data, arguments),
            ast::Instruction::AtomCas { data, arguments } => self.emit_atom_cas(data, arguments),
            ast::Instruction::Div { data, arguments } => self.emit_div(data, arguments),
            ast::Instruction::Neg { data, arguments } => self.emit_neg(data, arguments),
            ast::Instruction::Sin { data, arguments } => self.emit_sin(data, arguments),
            ast::Instruction::Cos { data, arguments } => self.emit_cos(data, arguments),
            ast::Instruction::Lg2 { data, arguments } => self.emit_lg2(data, arguments),
            ast::Instruction::Ex2 { data, arguments } => self.emit_ex2(data, arguments),
            ast::Instruction::Clz { data, arguments } => self.emit_clz(data, arguments),
            ast::Instruction::Brev { data, arguments } => self.emit_brev(data, arguments),
            ast::Instruction::Popc { data, arguments } => self.emit_popc(data, arguments),
            ast::Instruction::Xor { data, arguments } => self.emit_xor(data, arguments),
            ast::Instruction::Rem { data, arguments } => self.emit_rem(data, arguments),
            ast::Instruction::BarWarp { .. } => self.emit_bar_warp(),
            ast::Instruction::Membar { data } => self.emit_membar(data),
            ast::Instruction::Trap {} => self.emit_trap(),
            ast::Instruction::Tanh { data, arguments } => self.emit_tanh(data, arguments),
            ast::Instruction::CpAsync { data, arguments } => self.emit_cp_async(data, arguments),
            ast::Instruction::Copysign { data, arguments } => self.emit_copysign(data, arguments),
            ast::Instruction::CpAsyncCommitGroup {} => Ok(()), // nop
            ast::Instruction::CpAsyncWaitGroup { .. } => Ok(()), // nop
            ast::Instruction::CpAsyncWaitAll { .. } => Ok(()), // nop
            ast::Instruction::GridDepControl { .. } => Ok(()), // nop
            // replaced by a function call
            ast::Instruction::Bfe { .. }
            | ast::Instruction::Bar { .. }
            | ast::Instruction::BarRed { .. }
            | ast::Instruction::Bfi { .. }
            | ast::Instruction::Activemask { .. }
            | ast::Instruction::ShflSync { .. }
            | ast::Instruction::Vote { .. }
            | ast::Instruction::Nanosleep { .. }
            | ast::Instruction::ReduxSync { .. }
            | ast::Instruction::LdMatrix { .. }
            | ast::Instruction::Mma { .. }
            | ast::Instruction::Prmt { .. } => return Err(error_unreachable()),
        }
    }

    fn emit_ld(
        &mut self,
        data: ast::LdDetails,
        arguments: ast::LdArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let builder = self.builder;
        let underlying_type = get_type(self.context, &data.typ)?;
        let needs_cast = not_supported_by_atomics(data.qualifier, underlying_type);
        let op_type = if needs_cast {
            unsafe { LLVMIntTypeInContext(self.context, data.typ.layout().size() as u32 * 8) }
        } else {
            underlying_type
        };
        let src = self.resolver.value(arguments.src)?;
        let load = unsafe { LLVMBuildLoad2(builder, op_type, src, LLVM_UNNAMED.as_ptr()) };
        apply_qualifier(load, data.qualifier)?;
        unsafe { LLVMSetAlignment(load, data.typ.layout().align() as u32) };
        if needs_cast {
            self.resolver.with_result(arguments.dst, |dst| unsafe {
                LLVMBuildBitCast(builder, load, underlying_type, dst)
            });
        } else {
            self.resolver.register(arguments.dst, load);
        }
        Ok(())
    }

    fn emit_conversion(&mut self, conversion: ImplicitConversion) -> Result<(), TranslateError> {
        let builder = self.builder;
        match conversion.kind {
            ConversionKind::Default => self.emit_conversion_default(
                self.resolver.value(conversion.src)?,
                conversion.dst,
                &conversion.from_type,
                conversion.from_space,
                &conversion.to_type,
                conversion.to_space,
            ),
            ConversionKind::SignExtend => {
                let src = self.resolver.value(conversion.src)?;
                let type_ = get_type(self.context, &conversion.to_type)?;
                self.resolver.with_result(conversion.dst, |dst| unsafe {
                    LLVMBuildSExt(builder, src, type_, dst)
                });
                Ok(())
            }
            ConversionKind::BitToPtr => {
                let src = self.resolver.value(conversion.src)?;
                let type_ = get_pointer_type(self.context, conversion.to_space)?;
                self.resolver.with_result(conversion.dst, |dst| unsafe {
                    LLVMBuildIntToPtr(builder, src, type_, dst)
                });
                Ok(())
            }
            ConversionKind::PtrToPtr => {
                let src = self.resolver.value(conversion.src)?;
                let dst_type = get_pointer_type(self.context, conversion.to_space)?;
                self.resolver.with_result(conversion.dst, |dst| unsafe {
                    LLVMBuildAddrSpaceCast(builder, src, dst_type, dst)
                });
                Ok(())
            }
            ConversionKind::AddressOf => {
                let src = self.resolver.value(conversion.src)?;
                let dst_type = get_type(self.context, &conversion.to_type)?;
                self.resolver.with_result(conversion.dst, |dst| unsafe {
                    LLVMBuildPtrToInt(self.builder, src, dst_type, dst)
                });
                Ok(())
            }
        }
    }

    fn emit_conversion_default(
        &mut self,
        src: LLVMValueRef,
        dst: SpirvWord,
        from_type: &ast::Type,
        from_space: ast::StateSpace,
        to_type: &ast::Type,
        to_space: ast::StateSpace,
    ) -> Result<(), TranslateError> {
        match (from_type, to_type) {
            (ast::Type::Scalar(from_type), ast::Type::Scalar(to_type_scalar)) => {
                let from_layout = from_type.layout();
                let to_layout = to_type.layout();
                if from_layout.size() == to_layout.size() {
                    let dst_type = get_type(self.context, &to_type)?;
                    if from_type.kind() != ast::ScalarKind::Float
                        && to_type_scalar.kind() != ast::ScalarKind::Float
                    {
                        // It is noop, but another instruction expects result of this conversion
                        self.resolver.register(dst, src);
                    } else {
                        self.resolver.with_result(dst, |dst| unsafe {
                            LLVMBuildBitCast(self.builder, src, dst_type, dst)
                        });
                    }
                    Ok(())
                } else {
                    // This block is safe because it's illegal to implictly convert between floating point values
                    let same_width_bit_type = unsafe {
                        LLVMIntTypeInContext(self.context, (from_layout.size() * 8) as u32)
                    };
                    let same_width_bit_value = unsafe {
                        LLVMBuildBitCast(
                            self.builder,
                            src,
                            same_width_bit_type,
                            LLVM_UNNAMED.as_ptr(),
                        )
                    };
                    let wide_bit_type = match to_type_scalar.layout().size() {
                        1 => ast::ScalarType::B8,
                        2 => ast::ScalarType::B16,
                        4 => ast::ScalarType::B32,
                        8 => ast::ScalarType::B64,
                        _ => return Err(error_unreachable()),
                    };
                    let wide_bit_type_llvm = unsafe {
                        LLVMIntTypeInContext(self.context, (to_layout.size() * 8) as u32)
                    };
                    if to_type_scalar.kind() == ast::ScalarKind::Unsigned
                        || to_type_scalar.kind() == ast::ScalarKind::Bit
                    {
                        let llvm_fn = if to_type_scalar.size_of() >= from_type.size_of() {
                            LLVMBuildZExtOrBitCast
                        } else {
                            LLVMBuildTrunc
                        };
                        self.resolver.with_result(dst, |dst| unsafe {
                            llvm_fn(self.builder, same_width_bit_value, wide_bit_type_llvm, dst)
                        });
                        Ok(())
                    } else {
                        let conversion_fn = if from_type.kind() == ast::ScalarKind::Signed
                            && to_type_scalar.kind() == ast::ScalarKind::Signed
                        {
                            if to_type_scalar.size_of() >= from_type.size_of() {
                                LLVMBuildSExtOrBitCast
                            } else {
                                LLVMBuildTrunc
                            }
                        } else {
                            if to_type_scalar.size_of() >= from_type.size_of() {
                                LLVMBuildZExtOrBitCast
                            } else {
                                LLVMBuildTrunc
                            }
                        };
                        let wide_bit_value = unsafe {
                            conversion_fn(
                                self.builder,
                                same_width_bit_value,
                                wide_bit_type_llvm,
                                LLVM_UNNAMED.as_ptr(),
                            )
                        };
                        self.emit_conversion_default(
                            wide_bit_value,
                            dst,
                            &wide_bit_type.into(),
                            from_space,
                            to_type,
                            to_space,
                        )
                    }
                }
            }
            (ast::Type::Vector(..), ast::Type::Scalar(..))
            | (ast::Type::Scalar(..), ast::Type::Vector(..))
            | (ast::Type::Scalar(..), ast::Type::Array(..))
            | (ast::Type::Array(..), ast::Type::Scalar(..)) => {
                let dst_type = get_type(self.context, to_type)?;
                self.resolver.with_result(dst, |dst| unsafe {
                    LLVMBuildBitCast(self.builder, src, dst_type, dst)
                });
                Ok(())
            }
            _ => return Err(error_todo()),
        }
    }

    fn emit_constant(&mut self, constant: ConstantDefinition) -> Result<(), TranslateError> {
        let value = get_immediate_value(self.context, &constant.typ, &constant.value);
        self.resolver.register(constant.dst, value);
        Ok(())
    }

    fn emit_add(
        &mut self,
        data: ast::ArithDetails,
        arguments: ast::AddArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let builder = self.builder;
        let fn_ = match data {
            ast::ArithDetails::Integer(ast::ArithInteger {
                saturate: true,
                type_,
            }) => {
                let op = if type_.kind() == ast::ScalarKind::Signed {
                    "sadd"
                } else {
                    "uadd"
                };
                return self.emit_intrinsic_saturate(
                    op,
                    type_,
                    arguments.dst,
                    arguments.src1,
                    arguments.src2,
                );
            }
            ast::ArithDetails::Integer(ast::ArithInteger {
                saturate: false, ..
            }) => LLVMBuildAdd,
            ast::ArithDetails::Float(..) => LLVMBuildFAdd,
        };
        let src1 = self.resolver.value(arguments.src1)?;
        let src2 = self.resolver.value(arguments.src2)?;
        self.resolver.with_result(arguments.dst, |dst| unsafe {
            fn_(builder, src1, src2, dst)
        });
        Ok(())
    }

    fn emit_st(
        &self,
        data: ast::StData,
        arguments: ast::StArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let ptr = self.resolver.value(arguments.src1)?;
        let underlying_type = get_type(self.context, &data.typ)?;
        let needs_cast = not_supported_by_atomics(data.qualifier, underlying_type);
        let mut value = self.resolver.value(arguments.src2)?;
        if needs_cast {
            value = unsafe {
                LLVMBuildBitCast(
                    self.builder,
                    value,
                    LLVMIntTypeInContext(self.context, data.typ.layout().size() as u32 * 8),
                    LLVM_UNNAMED.as_ptr(),
                )
            };
        }
        let store = unsafe { LLVMBuildStore(self.builder, value, ptr) };
        apply_qualifier(store, data.qualifier)?;
        unsafe {
            LLVMSetAlignment(store, data.typ.layout().align() as u32);
        }
        Ok(())
    }

    fn emit_ret(&self, _data: ast::RetData) {
        unsafe { LLVMBuildRetVoid(self.builder) };
    }

    fn emit_call(
        &mut self,
        data: ast::CallDetails,
        arguments: ast::CallArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        if cfg!(debug_assertions) {
            for (_, space) in data.return_arguments.iter() {
                if *space != ast::StateSpace::Reg {
                    panic!()
                }
            }
        }
        let name = match &*arguments.return_arguments {
            [dst] => self.resolver.get_or_add_raw(*dst),
            _ => LLVM_UNNAMED.as_ptr(),
        };
        let type_ = get_function_type(
            self.context,
            data.return_arguments.iter().map(|(type_, ..)| type_),
            data.input_arguments
                .iter()
                .map(|(type_, space)| get_input_argument_type(self.context, &type_, *space)),
        )?;
        let mut input_arguments = arguments
            .input_arguments
            .iter()
            .map(|arg| self.resolver.value(*arg))
            .collect::<Result<Vec<_>, _>>()?;
        let llvm_call = unsafe {
            LLVMBuildCall2(
                self.builder,
                type_,
                self.resolver.value(arguments.func)?,
                input_arguments.as_mut_ptr(),
                input_arguments.len() as u32,
                name,
            )
        };
        match &*arguments.return_arguments {
            [] => {}
            [name] => self.resolver.register(*name, llvm_call),
            args => {
                for (i, arg) in args.iter().copied().enumerate() {
                    self.resolver.with_result(arg, |name| unsafe {
                        LLVMBuildExtractValue(self.builder, llvm_call, i as u32, name)
                    });
                }
            }
        }
        Ok(())
    }

    fn emit_mov(&mut self, arguments: ast::MovArgs<SpirvWord>) -> Result<(), TranslateError> {
        self.resolver
            .register(arguments.dst, self.resolver.value(arguments.src)?);
        Ok(())
    }

    fn emit_ptr_access(&mut self, ptr_access: PtrAccess<SpirvWord>) -> Result<(), TranslateError> {
        let ptr_src = self.resolver.value(ptr_access.ptr_src)?;
        let mut offset_src = self.resolver.value(ptr_access.offset_src)?;
        let pointee_type = get_scalar_type(self.context, ast::ScalarType::B8);
        self.resolver.with_result(ptr_access.dst, |dst| unsafe {
            LLVMBuildInBoundsGEP2(self.builder, pointee_type, ptr_src, &mut offset_src, 1, dst)
        });
        Ok(())
    }

    fn emit_and(&mut self, arguments: ast::AndArgs<SpirvWord>) -> Result<(), TranslateError> {
        let builder = self.builder;
        let src1 = self.resolver.value(arguments.src1)?;
        let src2 = self.resolver.value(arguments.src2)?;
        self.resolver.with_result(arguments.dst, |dst| unsafe {
            LLVMBuildAnd(builder, src1, src2, dst)
        });
        Ok(())
    }

    fn emit_atom(
        &mut self,
        data: ast::AtomDetails,
        arguments: ast::AtomArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let builder = self.builder;
        let src1 = self.resolver.value(arguments.src1)?;
        let src2 = self.resolver.value(arguments.src2)?;
        let op = match data.op {
            ast::AtomicOp::And => LLVMZludaAtomicRMWBinOp::LLVMZludaAtomicRMWBinOpAnd,
            ast::AtomicOp::Or => LLVMZludaAtomicRMWBinOp::LLVMZludaAtomicRMWBinOpOr,
            ast::AtomicOp::Xor => LLVMZludaAtomicRMWBinOp::LLVMZludaAtomicRMWBinOpXor,
            ast::AtomicOp::Exchange => LLVMZludaAtomicRMWBinOp::LLVMZludaAtomicRMWBinOpXchg,
            ast::AtomicOp::Add => LLVMZludaAtomicRMWBinOp::LLVMZludaAtomicRMWBinOpAdd,
            ast::AtomicOp::IncrementWrap => {
                LLVMZludaAtomicRMWBinOp::LLVMZludaAtomicRMWBinOpUIncWrap
            }
            ast::AtomicOp::DecrementWrap => {
                LLVMZludaAtomicRMWBinOp::LLVMZludaAtomicRMWBinOpUDecWrap
            }
            ast::AtomicOp::SignedMin => LLVMZludaAtomicRMWBinOp::LLVMZludaAtomicRMWBinOpMin,
            ast::AtomicOp::UnsignedMin => LLVMZludaAtomicRMWBinOp::LLVMZludaAtomicRMWBinOpUMin,
            ast::AtomicOp::SignedMax => LLVMZludaAtomicRMWBinOp::LLVMZludaAtomicRMWBinOpMax,
            ast::AtomicOp::UnsignedMax => LLVMZludaAtomicRMWBinOp::LLVMZludaAtomicRMWBinOpUMax,
            ast::AtomicOp::FloatAdd => LLVMZludaAtomicRMWBinOp::LLVMZludaAtomicRMWBinOpFAdd,
            ast::AtomicOp::FloatMin => LLVMZludaAtomicRMWBinOp::LLVMZludaAtomicRMWBinOpFMin,
            ast::AtomicOp::FloatMax => LLVMZludaAtomicRMWBinOp::LLVMZludaAtomicRMWBinOpFMax,
        };
        self.resolver.register(arguments.dst, unsafe {
            LLVMZludaBuildAtomicRMW(
                builder,
                op,
                src1,
                src2,
                get_scope(data.scope)?,
                get_ordering(data.semantics),
            )
        });
        Ok(())
    }

    fn emit_atom_cas(
        &mut self,
        data: ast::AtomCasDetails,
        arguments: ast::AtomCasArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let src1 = self.resolver.value(arguments.src1)?;
        let src2 = self.resolver.value(arguments.src2)?;
        let src3 = self.resolver.value(arguments.src3)?;
        let success_ordering = get_ordering(data.semantics);
        let failure_ordering = get_ordering_failure(data.semantics);
        let temp = unsafe {
            LLVMZludaBuildAtomicCmpXchg(
                self.builder,
                src1,
                src2,
                src3,
                get_scope(data.scope)?,
                success_ordering,
                failure_ordering,
            )
        };
        self.resolver.with_result(arguments.dst, |dst| unsafe {
            LLVMBuildExtractValue(self.builder, temp, 0, dst)
        });
        Ok(())
    }

    fn emit_bra(&self, arguments: ast::BraArgs<SpirvWord>) -> Result<(), TranslateError> {
        let src = self.resolver.value(arguments.src)?;
        let src = unsafe { LLVMValueAsBasicBlock(src) };
        unsafe { LLVMBuildBr(self.builder, src) };
        Ok(())
    }

    fn emit_brev(
        &mut self,
        data: ast::ScalarType,
        arguments: ast::BrevArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let llvm_fn = match data.size_of() {
            4 => c"llvm.bitreverse.i32",
            8 => c"llvm.bitreverse.i64",
            _ => return Err(error_unreachable()),
        };
        let mut fn_ = unsafe { LLVMGetNamedFunction(self.module, llvm_fn.as_ptr()) };
        let type_ = get_scalar_type(self.context, data);
        let fn_type = get_function_type(
            self.context,
            iter::once(&data.into()),
            iter::once(Ok(type_)),
        )?;
        if fn_ == ptr::null_mut() {
            fn_ = unsafe { LLVMAddFunction(self.module, llvm_fn.as_ptr(), fn_type) };
        }
        let mut src = self.resolver.value(arguments.src)?;
        self.resolver.with_result(arguments.dst, |dst| unsafe {
            LLVMBuildCall2(self.builder, fn_type, fn_, &mut src, 1, dst)
        });
        Ok(())
    }

    fn emit_ret_value(
        &mut self,
        values: Vec<(SpirvWord, ptx_parser::Type)>,
    ) -> Result<(), TranslateError> {
        let loads = values
            .iter()
            .map(|(value, type_)| {
                let value = self.resolver.value(*value)?;
                let lowered_type = get_type(self.context, type_)?;
                let load = unsafe {
                    LLVMBuildLoad2(self.builder, lowered_type, value, LLVM_UNNAMED.as_ptr())
                };
                unsafe {
                    LLVMSetAlignment(load, type_.layout().align() as u32);
                }
                Ok((load, type_))
            })
            .collect::<Result<Vec<_>, _>>()?;
        match &*loads {
            [] => unsafe { LLVMBuildRetVoid(self.builder) },
            [(value, _)] => unsafe { LLVMBuildRet(self.builder, *value) },
            loads => {
                let struct_type =
                    get_or_create_struct_type(self.context, loads.iter().map(|(_, type_)| *type_))?;
                let mut value = unsafe { LLVMGetUndef(struct_type) };
                for (i, (load, _)) in loads.iter().enumerate() {
                    value = unsafe {
                        LLVMBuildInsertValue(
                            self.builder,
                            value,
                            *load,
                            i as u32,
                            LLVM_UNNAMED.as_ptr(),
                        )
                    };
                }
                unsafe { LLVMBuildRet(self.builder, value) }
            }
        };
        Ok(())
    }

    fn emit_clz(
        &mut self,
        data: ptx_parser::ScalarType,
        arguments: ptx_parser::ClzArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let llvm_fn = match data.size_of() {
            4 => c"llvm.ctlz.i32",
            8 => c"llvm.ctlz.i64",
            _ => return Err(error_unreachable()),
        };
        let type_ = get_scalar_type(self.context, data.into());
        let pred = get_scalar_type(self.context, ast::ScalarType::Pred);
        let fn_type = get_function_type(
            self.context,
            iter::once(&ast::ScalarType::U32.into()),
            [Ok(type_), Ok(pred)].into_iter(),
        )?;
        let mut fn_ = unsafe { LLVMGetNamedFunction(self.module, llvm_fn.as_ptr()) };
        if fn_ == ptr::null_mut() {
            fn_ = unsafe { LLVMAddFunction(self.module, llvm_fn.as_ptr(), fn_type) };
        }
        let src = self.resolver.value(arguments.src)?;
        let false_ = unsafe { LLVMConstInt(pred, 0, 0) };
        let mut args = [src, false_];
        self.resolver.with_result(arguments.dst, |dst| unsafe {
            LLVMBuildCall2(
                self.builder,
                fn_type,
                fn_,
                args.as_mut_ptr(),
                args.len() as u32,
                dst,
            )
        });
        Ok(())
    }

    fn emit_mul(
        &mut self,
        data: ast::MulDetails,
        arguments: ast::MulArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        self.emit_mul_impl(data, Some(arguments.dst), arguments.src1, arguments.src2)?;
        Ok(())
    }

    fn emit_mul_impl(
        &mut self,
        data: ast::MulDetails,
        dst: Option<SpirvWord>,
        src1: SpirvWord,
        src2: SpirvWord,
    ) -> Result<LLVMValueRef, TranslateError> {
        let mul_fn = match data {
            ast::MulDetails::Integer { control, type_ } => match control {
                ast::MulIntControl::Low => LLVMBuildMul,
                ast::MulIntControl::High => return self.emit_mul_high(type_, dst, src1, src2),
                ast::MulIntControl::Wide => {
                    return Ok(self.emit_mul_wide_impl(type_, dst, src1, src2)?.1)
                }
            },
            ast::MulDetails::Float(..) => LLVMBuildFMul,
        };
        let src1 = self.resolver.value(src1)?;
        let src2 = self.resolver.value(src2)?;
        Ok(self
            .resolver
            .with_result_option(dst, |dst| unsafe { mul_fn(self.builder, src1, src2, dst) }))
    }

    fn emit_mul_high(
        &mut self,
        type_: ptx_parser::ScalarType,
        dst: Option<SpirvWord>,
        src1: SpirvWord,
        src2: SpirvWord,
    ) -> Result<LLVMValueRef, TranslateError> {
        let (wide_type, wide_value) = self.emit_mul_wide_impl(type_, None, src1, src2)?;
        let shift_constant =
            unsafe { LLVMConstInt(wide_type, (type_.layout().size() * 8) as u64, 0) };
        let shifted = unsafe {
            LLVMBuildLShr(
                self.builder,
                wide_value,
                shift_constant,
                LLVM_UNNAMED.as_ptr(),
            )
        };
        let narrow_type = get_scalar_type(self.context, type_);
        Ok(self.resolver.with_result_option(dst, |dst| unsafe {
            LLVMBuildTrunc(self.builder, shifted, narrow_type, dst)
        }))
    }

    fn emit_mul_wide_impl(
        &mut self,
        type_: ptx_parser::ScalarType,
        dst: Option<SpirvWord>,
        src1: SpirvWord,
        src2: SpirvWord,
    ) -> Result<(LLVMTypeRef, LLVMValueRef), TranslateError> {
        let src1 = self.resolver.value(src1)?;
        let src2 = self.resolver.value(src2)?;
        let wide_type =
            unsafe { LLVMIntTypeInContext(self.context, (type_.layout().size() * 8 * 2) as u32) };
        let llvm_cast = match type_.kind() {
            ptx_parser::ScalarKind::Signed => LLVMBuildSExt,
            ptx_parser::ScalarKind::Unsigned => LLVMBuildZExt,
            _ => return Err(error_unreachable()),
        };
        let src1 = unsafe { llvm_cast(self.builder, src1, wide_type, LLVM_UNNAMED.as_ptr()) };
        let src2 = unsafe { llvm_cast(self.builder, src2, wide_type, LLVM_UNNAMED.as_ptr()) };
        Ok((
            wide_type,
            self.resolver.with_result_option(dst, |dst| unsafe {
                LLVMBuildMul(self.builder, src1, src2, dst)
            }),
        ))
    }

    fn emit_cos(
        &mut self,
        _data: ast::FlushToZero,
        arguments: ast::CosArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let llvm_f32 = get_scalar_type(self.context, ast::ScalarType::F32);
        let cos = self.emit_intrinsic(
            c"llvm.cos.f32",
            Some(arguments.dst),
            Some(&ast::ScalarType::F32.into()),
            vec![(self.resolver.value(arguments.src)?, llvm_f32)],
        )?;
        unsafe { LLVMZludaSetFastMathFlags(cos, LLVMZludaFastMathApproxFunc) }
        Ok(())
    }

    fn emit_or(
        &mut self,
        _data: ptx_parser::ScalarType,
        arguments: ptx_parser::OrArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let src1 = self.resolver.value(arguments.src1)?;
        let src2 = self.resolver.value(arguments.src2)?;
        self.resolver.with_result(arguments.dst, |dst| unsafe {
            LLVMBuildOr(self.builder, src1, src2, dst)
        });
        Ok(())
    }

    fn emit_xor(
        &mut self,
        _data: ptx_parser::ScalarType,
        arguments: ptx_parser::XorArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let src1 = self.resolver.value(arguments.src1)?;
        let src2 = self.resolver.value(arguments.src2)?;
        self.resolver.with_result(arguments.dst, |dst| unsafe {
            LLVMBuildXor(self.builder, src1, src2, dst)
        });
        Ok(())
    }

    fn emit_vector_read(&mut self, vec_acccess: VectorRead) -> Result<(), TranslateError> {
        let src = self.resolver.value(vec_acccess.vector_src)?;
        let index = unsafe {
            LLVMConstInt(
                get_scalar_type(self.context, ast::ScalarType::B8),
                vec_acccess.member as _,
                0,
            )
        };
        self.resolver
            .with_result(vec_acccess.scalar_dst, |dst| unsafe {
                LLVMBuildExtractElement(self.builder, src, index, dst)
            });
        Ok(())
    }

    fn emit_vector_write(&mut self, vector_write: VectorWrite) -> Result<(), TranslateError> {
        let vector_src = self.resolver.value(vector_write.vector_src)?;
        let scalar_src = self.resolver.value(vector_write.scalar_src)?;
        let index = unsafe {
            LLVMConstInt(
                get_scalar_type(self.context, ast::ScalarType::B8),
                vector_write.member as _,
                0,
            )
        };
        self.resolver
            .with_result(vector_write.vector_dst, |dst| unsafe {
                LLVMBuildInsertElement(self.builder, vector_src, scalar_src, index, dst)
            });
        Ok(())
    }

    fn emit_vector_repack(&mut self, repack: RepackVectorDetails) -> Result<(), TranslateError> {
        let i8_type = get_scalar_type(self.context, ast::ScalarType::B8);
        if repack.is_extract {
            let src = self.resolver.value(repack.packed)?;
            for (index, dst) in repack.unpacked.iter().enumerate() {
                let index: *mut LLVMValue = unsafe { LLVMConstInt(i8_type, index as _, 0) };
                self.resolver.with_result(*dst, |dst| unsafe {
                    LLVMBuildExtractElement(self.builder, src, index, dst)
                });
            }
        } else {
            let vector_type = get_type(
                self.context,
                &ast::Type::Vector(repack.unpacked.len() as u8, repack.typ),
            )?;
            let mut temp_vec = unsafe { LLVMGetUndef(vector_type) };
            for (index, src_id) in repack.unpacked.iter().enumerate() {
                let dst = if index == repack.unpacked.len() - 1 {
                    Some(repack.packed)
                } else {
                    None
                };
                let scalar_src = self.resolver.value(*src_id)?;
                let index = unsafe { LLVMConstInt(i8_type, index as _, 0) };
                temp_vec = self.resolver.with_result_option(dst, |dst| unsafe {
                    LLVMBuildInsertElement(self.builder, temp_vec, scalar_src, index, dst)
                });
            }
        }
        Ok(())
    }

    fn emit_div(
        &mut self,
        data: ptx_parser::DivDetails,
        arguments: ptx_parser::DivArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let integer_div = match data {
            ptx_parser::DivDetails::Unsigned(_) => LLVMBuildUDiv,
            ptx_parser::DivDetails::Signed(_) => LLVMBuildSDiv,
            ptx_parser::DivDetails::Float(float_div) => {
                return self.emit_div_float(float_div, arguments)
            }
        };
        let src1 = self.resolver.value(arguments.src1)?;
        let src2 = self.resolver.value(arguments.src2)?;
        self.resolver.with_result(arguments.dst, |dst| unsafe {
            integer_div(self.builder, src1, src2, dst)
        });
        Ok(())
    }

    fn emit_div_float(
        &mut self,
        float_div: ptx_parser::DivFloatDetails,
        arguments: ptx_parser::DivArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let builder = self.builder;
        let src1 = self.resolver.value(arguments.src1)?;
        let src2 = self.resolver.value(arguments.src2)?;
        let _rnd = match float_div.kind {
            ptx_parser::DivFloatKind::Approx => ast::RoundingMode::NearestEven,
            ptx_parser::DivFloatKind::ApproxFull => ast::RoundingMode::NearestEven,
            ptx_parser::DivFloatKind::Rounding(rounding_mode) => rounding_mode,
        };
        let approx = match float_div.kind {
            ptx_parser::DivFloatKind::Approx => {
                LLVMZludaFastMathAllowReciprocal | LLVMZludaFastMathApproxFunc
            }
            ptx_parser::DivFloatKind::ApproxFull => LLVMZludaFastMathNone,
            ptx_parser::DivFloatKind::Rounding(_) => LLVMZludaFastMathNone,
        };
        let fdiv = self.resolver.with_result(arguments.dst, |dst| unsafe {
            LLVMBuildFDiv(builder, src1, src2, dst)
        });
        unsafe { LLVMZludaSetFastMathFlags(fdiv, approx) };
        if let ptx_parser::DivFloatKind::ApproxFull = float_div.kind {
            // https://docs.nvidia.com/cuda/parallel-thread-execution/#floating-point-instructions-div:
            // div.full.f32 implements a relatively fast, full-range approximation that scales
            // operands to achieve better accuracy, but is not fully IEEE 754 compliant and does not
            // support rounding modifiers. The maximum ulp error is 2 across the full range of
            // inputs.
            // https://llvm.org/docs/LangRef.html#fpmath-metadata
            let fpmath_value =
                unsafe { LLVMConstReal(get_scalar_type(self.context, ast::ScalarType::F32), 2.0) };
            let fpmath_value = unsafe { LLVMValueAsMetadata(fpmath_value) };
            let mut md_node_content = [fpmath_value];
            let md_node = unsafe {
                LLVMMDNodeInContext2(
                    self.context,
                    md_node_content.as_mut_ptr(),
                    md_node_content.len(),
                )
            };
            let md_node = unsafe { LLVMMetadataAsValue(self.context, md_node) };
            let kind = unsafe {
                LLVMGetMDKindIDInContext(
                    self.context,
                    "fpmath".as_ptr().cast(),
                    "fpmath".len() as u32,
                )
            };
            unsafe { LLVMSetMetadata(fdiv, kind, md_node) };
        }
        Ok(())
    }

    fn emit_cvta(
        &mut self,
        data: ptx_parser::CvtaDetails,
        arguments: ptx_parser::CvtaArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let (from_space, to_space) = match data.direction {
            ptx_parser::CvtaDirection::GenericToExplicit => {
                (ast::StateSpace::Generic, data.state_space)
            }
            ptx_parser::CvtaDirection::ExplicitToGeneric => {
                (data.state_space, ast::StateSpace::Generic)
            }
        };
        let from_type = get_pointer_type(self.context, from_space)?;
        let dest_type = get_pointer_type(self.context, to_space)?;
        let src = self.resolver.value(arguments.src)?;
        let temp_ptr =
            unsafe { LLVMBuildIntToPtr(self.builder, src, from_type, LLVM_UNNAMED.as_ptr()) };
        self.resolver.with_result(arguments.dst, |dst| unsafe {
            LLVMBuildAddrSpaceCast(self.builder, temp_ptr, dest_type, dst)
        });
        Ok(())
    }

    fn emit_sub(
        &mut self,
        data: ptx_parser::ArithDetails,
        arguments: ptx_parser::SubArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        match data {
            ptx_parser::ArithDetails::Integer(arith_integer) => {
                self.emit_sub_integer(arith_integer, arguments)
            }
            ptx_parser::ArithDetails::Float(arith_float) => {
                self.emit_sub_float(arith_float, arguments)
            }
        }
    }

    fn emit_sub_integer(
        &mut self,
        arith_integer: ptx_parser::ArithInteger,
        arguments: ptx_parser::SubArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        if arith_integer.saturate {
            let op = if arith_integer.type_.kind() == ast::ScalarKind::Signed {
                "ssub"
            } else {
                "usub"
            };
            return self.emit_intrinsic_saturate(
                op,
                arith_integer.type_,
                arguments.dst,
                arguments.src1,
                arguments.src2,
            );
        }
        let src1 = self.resolver.value(arguments.src1)?;
        let src2 = self.resolver.value(arguments.src2)?;
        self.resolver.with_result(arguments.dst, |dst| unsafe {
            LLVMBuildSub(self.builder, src1, src2, dst)
        });
        Ok(())
    }

    fn emit_sub_float(
        &mut self,
        _arith_float: ptx_parser::ArithFloat,
        arguments: ptx_parser::SubArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let src1 = self.resolver.value(arguments.src1)?;
        let src2 = self.resolver.value(arguments.src2)?;
        self.resolver.with_result(arguments.dst, |dst| unsafe {
            LLVMBuildFSub(self.builder, src1, src2, dst)
        });
        Ok(())
    }

    fn emit_sin(
        &mut self,
        _data: ptx_parser::FlushToZero,
        arguments: ptx_parser::SinArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let llvm_f32 = get_scalar_type(self.context, ast::ScalarType::F32);
        let sin = self.emit_intrinsic(
            c"llvm.sin.f32",
            Some(arguments.dst),
            Some(&ast::ScalarType::F32.into()),
            vec![(self.resolver.value(arguments.src)?, llvm_f32)],
        )?;
        unsafe { LLVMZludaSetFastMathFlags(sin, LLVMZludaFastMathApproxFunc) }
        Ok(())
    }

    fn emit_intrinsic(
        &mut self,
        name: &CStr,
        dst: Option<SpirvWord>,
        return_type: Option<&ast::Type>,
        arguments: Vec<(LLVMValueRef, LLVMTypeRef)>,
    ) -> Result<LLVMValueRef, TranslateError> {
        let fn_type = get_function_type(
            self.context,
            return_type.into_iter(),
            arguments.iter().map(|(_, type_)| Ok(*type_)),
        )?;
        let mut fn_ = unsafe { LLVMGetNamedFunction(self.module, name.as_ptr()) };
        if fn_ == ptr::null_mut() {
            fn_ = unsafe { LLVMAddFunction(self.module, name.as_ptr(), fn_type) };
        }
        let mut arguments = arguments.iter().map(|(arg, _)| *arg).collect::<Vec<_>>();
        Ok(self.resolver.with_result_option(dst, |dst| unsafe {
            LLVMBuildCall2(
                self.builder,
                fn_type,
                fn_,
                arguments.as_mut_ptr(),
                arguments.len() as u32,
                dst,
            )
        }))
    }

    fn emit_neg(
        &mut self,
        data: ptx_parser::TypeFtz,
        arguments: ptx_parser::NegArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let src = self.resolver.value(arguments.src)?;
        let is_floating_point = data.type_.kind() == ptx_parser::ScalarKind::Float;
        let llvm_fn = if is_floating_point {
            LLVMBuildFNeg
        } else {
            LLVMBuildNeg
        };
        if is_floating_point && data.flush_to_zero == Some(true) {
            let negated = unsafe { llvm_fn(self.builder, src, LLVM_UNNAMED.as_ptr()) };
            let intrinsic = format!("llvm.canonicalize.{}\0", LLVMTypeDisplay(data.type_));
            self.emit_intrinsic(
                unsafe { CStr::from_bytes_with_nul_unchecked(intrinsic.as_bytes()) },
                Some(arguments.dst),
                Some(&data.type_.into()),
                vec![(negated, get_scalar_type(self.context, data.type_))],
            )?;
        } else {
            self.resolver.with_result(arguments.dst, |dst| unsafe {
                llvm_fn(self.builder, src, dst)
            });
        }
        Ok(())
    }

    fn emit_not(
        &mut self,
        type_: ptx_parser::ScalarType,
        arguments: ptx_parser::NotArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let src = self.resolver.value(arguments.src)?;
        let type_ = get_scalar_type(self.context, type_);
        let constant = unsafe { LLVMConstInt(type_, u64::MAX, 0) };
        self.resolver.with_result(arguments.dst, |dst| unsafe {
            LLVMBuildXor(self.builder, src, constant, dst)
        });
        Ok(())
    }

    fn emit_setp(
        &mut self,
        data: ptx_parser::SetpData,
        arguments: ptx_parser::SetpArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let dst = self.emit_setp_impl(data, arguments.dst2, arguments.src1, arguments.src2)?;
        self.resolver.register(arguments.dst1, dst);
        Ok(())
    }

    fn emit_setp_impl(
        &mut self,
        data: ptx_parser::SetpData,
        dst2: Option<SpirvWord>,
        src1: SpirvWord,
        src2: SpirvWord,
    ) -> Result<LLVMValueRef, TranslateError> {
        if dst2.is_some() {
            return Err(error_todo_msg(
                "setp with two dst arguments not yet supported",
            ));
        }
        match data.cmp_op {
            ptx_parser::SetpCompareOp::Integer(setp_compare_int) => {
                self.emit_setp_int(setp_compare_int, src1, src2)
            }
            ptx_parser::SetpCompareOp::Float(setp_compare_float) => {
                self.emit_setp_float(setp_compare_float, src1, src2)
            }
        }
    }

    fn emit_setp_int(
        &mut self,
        setp: ptx_parser::SetpCompareInt,
        src1: SpirvWord,
        src2: SpirvWord,
    ) -> Result<LLVMValueRef, TranslateError> {
        let op = match setp {
            ptx_parser::SetpCompareInt::Eq => LLVMIntPredicate::LLVMIntEQ,
            ptx_parser::SetpCompareInt::NotEq => LLVMIntPredicate::LLVMIntNE,
            ptx_parser::SetpCompareInt::UnsignedLess => LLVMIntPredicate::LLVMIntULT,
            ptx_parser::SetpCompareInt::UnsignedLessOrEq => LLVMIntPredicate::LLVMIntULE,
            ptx_parser::SetpCompareInt::UnsignedGreater => LLVMIntPredicate::LLVMIntUGT,
            ptx_parser::SetpCompareInt::UnsignedGreaterOrEq => LLVMIntPredicate::LLVMIntUGE,
            ptx_parser::SetpCompareInt::SignedLess => LLVMIntPredicate::LLVMIntSLT,
            ptx_parser::SetpCompareInt::SignedLessOrEq => LLVMIntPredicate::LLVMIntSLE,
            ptx_parser::SetpCompareInt::SignedGreater => LLVMIntPredicate::LLVMIntSGT,
            ptx_parser::SetpCompareInt::SignedGreaterOrEq => LLVMIntPredicate::LLVMIntSGE,
        };
        let src1 = self.resolver.value(src1)?;
        let src2 = self.resolver.value(src2)?;
        Ok(unsafe { LLVMBuildICmp(self.builder, op, src1, src2, LLVM_UNNAMED.as_ptr()) })
    }

    fn emit_setp_float(
        &mut self,
        setp: ptx_parser::SetpCompareFloat,
        src1: SpirvWord,
        src2: SpirvWord,
    ) -> Result<LLVMValueRef, TranslateError> {
        let op = match setp {
            ptx_parser::SetpCompareFloat::Eq => LLVMRealPredicate::LLVMRealOEQ,
            ptx_parser::SetpCompareFloat::NotEq => LLVMRealPredicate::LLVMRealONE,
            ptx_parser::SetpCompareFloat::Less => LLVMRealPredicate::LLVMRealOLT,
            ptx_parser::SetpCompareFloat::LessOrEq => LLVMRealPredicate::LLVMRealOLE,
            ptx_parser::SetpCompareFloat::Greater => LLVMRealPredicate::LLVMRealOGT,
            ptx_parser::SetpCompareFloat::GreaterOrEq => LLVMRealPredicate::LLVMRealOGE,
            ptx_parser::SetpCompareFloat::NanEq => LLVMRealPredicate::LLVMRealUEQ,
            ptx_parser::SetpCompareFloat::NanNotEq => LLVMRealPredicate::LLVMRealUNE,
            ptx_parser::SetpCompareFloat::NanLess => LLVMRealPredicate::LLVMRealULT,
            ptx_parser::SetpCompareFloat::NanLessOrEq => LLVMRealPredicate::LLVMRealULE,
            ptx_parser::SetpCompareFloat::NanGreater => LLVMRealPredicate::LLVMRealUGT,
            ptx_parser::SetpCompareFloat::NanGreaterOrEq => LLVMRealPredicate::LLVMRealUGE,
            ptx_parser::SetpCompareFloat::IsNotNan => LLVMRealPredicate::LLVMRealORD,
            ptx_parser::SetpCompareFloat::IsAnyNan => LLVMRealPredicate::LLVMRealUNO,
        };
        let src1 = self.resolver.value(src1)?;
        let src2 = self.resolver.value(src2)?;
        Ok(unsafe { LLVMBuildFCmp(self.builder, op, src1, src2, LLVM_UNNAMED.as_ptr()) })
    }

    fn emit_conditional(&mut self, cond: BrachCondition) -> Result<(), TranslateError> {
        let predicate = self.resolver.value(cond.predicate)?;
        let if_true = self.resolver.value(cond.if_true)?;
        let if_false = self.resolver.value(cond.if_false)?;
        unsafe {
            LLVMBuildCondBr(
                self.builder,
                predicate,
                LLVMValueAsBasicBlock(if_true),
                LLVMValueAsBasicBlock(if_false),
            )
        };
        Ok(())
    }

    fn emit_cvt(
        &mut self,
        data: ptx_parser::CvtDetails,
        arguments: ptx_parser::CvtArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        // Truncating conversions to FP8 types should be replaced by a function call.
        match data {
            ptx_parser::CvtDetails {
                to: ast::ScalarType::E4m3x2 | ast::ScalarType::E5m2x2,
                mode: ast::CvtMode::FPTruncate { .. },
                ..
            } => return Err(error_unreachable()),
            _ => {}
        }

        // Extending conversions from FP8 types should be replaced by a function call.
        match data {
            ptx_parser::CvtDetails {
                from: ast::ScalarType::E4m3x2 | ast::ScalarType::E5m2x2,
                mode: ast::CvtMode::FPExtend { .. },
                ..
            } => return Err(error_unreachable()),
            _ => {}
        }

        let dst_type = get_scalar_type(self.context, data.to);
        let llvm_fn = match data.mode {
            ptx_parser::CvtMode::ZeroExtend => LLVMBuildZExt,
            ptx_parser::CvtMode::SignExtend => LLVMBuildSExt,
            ptx_parser::CvtMode::Truncate => LLVMBuildTrunc,
            ptx_parser::CvtMode::Bitcast => LLVMBuildBitCast,
            ptx_parser::CvtMode::IntSaturateToSigned => {
                return self.emit_cvt_unsigned_to_signed_sat(data.from, data.to, arguments)
            }
            ptx_parser::CvtMode::IntSaturateToUnsigned => {
                return self.emit_cvt_signed_to_unsigned_sat(data.from, data.to, arguments)
            }
            ptx_parser::CvtMode::FPExtend { .. } => LLVMBuildFPExt,
            ptx_parser::CvtMode::FPTruncate { .. } => LLVMBuildFPTrunc,
            ptx_parser::CvtMode::FPRound {
                integer_rounding: None,
                flush_to_zero: None | Some(false),
                ..
            } => {
                return self.emit_mov(ast::MovArgs {
                    dst: arguments.dst,
                    src: arguments.src,
                })
            }
            ptx_parser::CvtMode::FPRound {
                integer_rounding: None,
                flush_to_zero: Some(true),
                ..
            } => return self.flush_denormals(data.to, arguments.src, arguments.dst),
            ptx_parser::CvtMode::FPRound {
                integer_rounding: Some(rounding),
                ..
            } => return self.emit_cvt_float_to_int(data.from, data.to, rounding, arguments, None),
            ptx_parser::CvtMode::SignedFromFP { rounding, .. } => {
                return self.emit_cvt_float_to_int(
                    data.from,
                    data.to,
                    rounding,
                    arguments,
                    Some(true),
                )
            }
            ptx_parser::CvtMode::UnsignedFromFP { rounding, .. } => {
                return self.emit_cvt_float_to_int(
                    data.from,
                    data.to,
                    rounding,
                    arguments,
                    Some(false),
                )
            }
            ptx_parser::CvtMode::FPFromSigned { .. } => {
                return self.emit_cvt_int_to_float(data.to, arguments, LLVMBuildSIToFP)
            }
            ptx_parser::CvtMode::FPFromUnsigned { .. } => {
                return self.emit_cvt_int_to_float(data.to, arguments, LLVMBuildUIToFP)
            }
        };
        let src = self.resolver.value(arguments.src)?;
        if let Some(src2) = arguments.src2 {
            let packed_type = get_scalar_type(
                self.context,
                data.to
                    .packed_type()
                    .ok_or_else(|| error_mismatched_type())?,
            );
            let src2 = self.resolver.value(src2)?;
            let vec = unsafe {
                LLVMBuildInsertElement(
                    self.builder,
                    LLVMGetPoison(dst_type),
                    llvm_fn(self.builder, src, packed_type, LLVM_UNNAMED.as_ptr()),
                    LLVMConstInt(LLVMInt32TypeInContext(self.context), 1, false as i32),
                    LLVM_UNNAMED.as_ptr(),
                )
            };
            self.resolver.with_result(arguments.dst, |dst| unsafe {
                LLVMBuildInsertElement(
                    self.builder,
                    vec,
                    llvm_fn(self.builder, src2, packed_type, LLVM_UNNAMED.as_ptr()),
                    LLVMConstInt(LLVMInt32TypeInContext(self.context), 0, false as i32),
                    dst,
                )
            })
        } else {
            self.resolver.with_result(arguments.dst, |dst| unsafe {
                llvm_fn(self.builder, src, dst_type, dst)
            })
        };
        Ok(())
    }

    fn emit_cvt_pack(
        &mut self,
        type_: ast::ScalarType,
        arguments: ast::CvtPackArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let mut src1_saturated =
            self.emit_saturate_integer(ast::ScalarType::S32, type_, arguments.src1)?;
        let mut src2_saturated =
            self.emit_saturate_integer(ast::ScalarType::S32, type_, arguments.src2)?;
        let dest_ty = get_scalar_type(self.context, ast::ScalarType::U32);
        if type_.kind() == ast::ScalarKind::Signed {
            let type_llvm = get_scalar_type(self.context, type_);
            let src1_truncated = unsafe {
                LLVMBuildTrunc(
                    self.builder,
                    src1_saturated,
                    type_llvm,
                    LLVM_UNNAMED.as_ptr(),
                )
            };
            src1_saturated = unsafe {
                LLVMBuildZExt(self.builder, src1_truncated, dest_ty, LLVM_UNNAMED.as_ptr())
            };
            let src2_truncated = unsafe {
                LLVMBuildTrunc(
                    self.builder,
                    src2_saturated,
                    type_llvm,
                    LLVM_UNNAMED.as_ptr(),
                )
            };
            src2_saturated = unsafe {
                LLVMBuildZExt(self.builder, src2_truncated, dest_ty, LLVM_UNNAMED.as_ptr())
            };
        }
        // result is 32 bit integer packed {src3, src1, src2}
        let input_bit_size = type_.bit_size();
        let src1_shifted = unsafe {
            LLVMBuildShl(
                self.builder,
                src1_saturated,
                LLVMConstInt(dest_ty, input_bit_size as u64, 0),
                LLVM_UNNAMED.as_ptr(),
            )
        };
        let src3 = self.resolver.value(arguments.src3)?;
        let src3_shifted = unsafe {
            LLVMBuildShl(
                self.builder,
                src3,
                LLVMConstInt(dest_ty, input_bit_size as u64 * 2, 0),
                LLVM_UNNAMED.as_ptr(),
            )
        };
        let src1_src2 = unsafe {
            LLVMBuildOr(
                self.builder,
                src1_shifted,
                src2_saturated,
                LLVM_UNNAMED.as_ptr(),
            )
        };
        self.resolver.with_result(arguments.dst, |dst| unsafe {
            LLVMBuildOr(self.builder, src1_src2, src3_shifted, dst)
        });
        Ok(())
    }

    fn emit_cvt_unsigned_to_signed_sat(
        &mut self,
        from: ptx_parser::ScalarType,
        to: ptx_parser::ScalarType,
        arguments: ptx_parser::CvtArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let clamped = self.emit_saturate_integer(from, to, arguments.src)?;
        let resize_fn = if to.layout().size() >= from.layout().size() {
            LLVMBuildSExtOrBitCast
        } else {
            LLVMBuildTrunc
        };
        let to_llvm = get_scalar_type(self.context, to);
        self.resolver.with_result(arguments.dst, |dst| unsafe {
            resize_fn(self.builder, clamped, to_llvm, dst)
        });
        Ok(())
    }

    fn emit_saturate_integer(
        &mut self,
        from: ptx_parser::ScalarType,
        to: ptx_parser::ScalarType,
        src: SpirvWord,
    ) -> Result<LLVMValueRef, TranslateError> {
        let from_llvm = get_scalar_type(self.context, from);
        match from.kind() {
            ptx_parser::ScalarKind::Unsigned => {
                let max_value = match to {
                    ptx_parser::ScalarType::U8 => u8::MAX as u64,
                    ptx_parser::ScalarType::S8 => i8::MAX as u64,
                    ptx_parser::ScalarType::U16 => u16::MAX as u64,
                    ptx_parser::ScalarType::S16 => i16::MAX as u64,
                    ptx_parser::ScalarType::U32 => u32::MAX as u64,
                    ptx_parser::ScalarType::S32 => i32::MAX as u64,
                    ptx_parser::ScalarType::U64 => u64::MAX as u64,
                    ptx_parser::ScalarType::S64 => i64::MAX as u64,
                    _ => return Err(error_unreachable()),
                };
                let intrinsic = format!("llvm.umin.{}\0", LLVMTypeDisplay(from));
                let max = unsafe { LLVMConstInt(from_llvm, max_value, 0) };
                let clamped = self.emit_intrinsic(
                    unsafe { CStr::from_bytes_with_nul_unchecked(intrinsic.as_bytes()) },
                    None,
                    Some(&from.into()),
                    vec![(self.resolver.value(src)?, from_llvm), (max, from_llvm)],
                )?;
                Ok(clamped)
            }
            ptx_parser::ScalarKind::Signed => {
                let (min_value_from, max_value_from) = match from {
                    ptx_parser::ScalarType::S8 => (i8::MIN as i128, i8::MAX as i128),
                    ptx_parser::ScalarType::S16 => (i16::MIN as i128, i16::MAX as i128),
                    ptx_parser::ScalarType::S32 => (i32::MIN as i128, i32::MAX as i128),
                    ptx_parser::ScalarType::S64 => (i64::MIN as i128, i64::MAX as i128),
                    _ => return Err(error_unreachable()),
                };
                let (min_value_to, max_value_to) = match to {
                    ptx_parser::ScalarType::U8 => (u8::MIN as i128, u8::MAX as i128),
                    ptx_parser::ScalarType::S8 => (i8::MIN as i128, i8::MAX as i128),
                    ptx_parser::ScalarType::U16 => (u16::MIN as i128, u16::MAX as i128),
                    ptx_parser::ScalarType::S16 => (i16::MIN as i128, i16::MAX as i128),
                    ptx_parser::ScalarType::U32 => (u32::MIN as i128, u32::MAX as i128),
                    ptx_parser::ScalarType::S32 => (i32::MIN as i128, i32::MAX as i128),
                    ptx_parser::ScalarType::U64 => (u64::MIN as i128, u64::MAX as i128),
                    ptx_parser::ScalarType::S64 => (i64::MIN as i128, i64::MAX as i128),
                    _ => return Err(error_unreachable()),
                };
                let min_value = min_value_from.max(min_value_to);
                let max_value = max_value_from.min(max_value_to);
                let max_intrinsic = format!("llvm.smax.{}\0", LLVMTypeDisplay(from));
                let min = unsafe { LLVMConstInt(from_llvm, min_value as u64, 1) };
                let min_intrinsic = format!("llvm.smin.{}\0", LLVMTypeDisplay(from));
                let max = unsafe { LLVMConstInt(from_llvm, max_value as u64, 1) };
                let clamped = self.emit_intrinsic(
                    unsafe { CStr::from_bytes_with_nul_unchecked(max_intrinsic.as_bytes()) },
                    None,
                    Some(&from.into()),
                    vec![(self.resolver.value(src)?, from_llvm), (min, from_llvm)],
                )?;
                let clamped = self.emit_intrinsic(
                    unsafe { CStr::from_bytes_with_nul_unchecked(min_intrinsic.as_bytes()) },
                    None,
                    Some(&from.into()),
                    vec![(clamped, from_llvm), (max, from_llvm)],
                )?;
                Ok(clamped)
            }
            _ => return Err(error_unreachable()),
        }
    }

    fn emit_cvt_signed_to_unsigned_sat(
        &mut self,
        from: ptx_parser::ScalarType,
        to: ptx_parser::ScalarType,
        arguments: ptx_parser::CvtArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let clamped = self.emit_saturate_integer(from, to, arguments.src)?;
        let resize_fn = if to.layout().size() >= from.layout().size() {
            LLVMBuildZExtOrBitCast
        } else {
            LLVMBuildTrunc
        };
        let to_llvm = get_scalar_type(self.context, to);
        self.resolver.with_result(arguments.dst, |dst| unsafe {
            resize_fn(self.builder, clamped, to_llvm, dst)
        });
        Ok(())
    }

    fn emit_cvt_float_to_int(
        &mut self,
        from: ast::ScalarType,
        to: ast::ScalarType,
        rounding: ast::RoundingMode,
        arguments: ptx_parser::CvtArgs<SpirvWord>,
        signed_cast: Option<bool>,
    ) -> Result<(), TranslateError> {
        let dst_int_rounded =
            self.emit_fp_int_rounding(from, rounding, &arguments, signed_cast.is_some())?;
        // In PTX all the int-from-float casts are saturating casts. On the other hand, in LLVM,
        // out-of-range fptoui and fptosi have undefined behavior.
        // We could handle this all with llvm.fptosi.sat and llvm.fptoui.sat intrinsics, but
        // the problem is that, when using *.sat variants AMDGPU target _always_ emits saturation
        // checks. Often they are unnecessary because v_cvt_* instructions saturates anyway.
        // For that reason, all from-to combinations that we know have a direct corresponding
        // v_cvt_* instruction get special treatment
        let is_saturating_cast = match (to, from) {
            (ast::ScalarType::S16, ast::ScalarType::F16)
            | (ast::ScalarType::S32, ast::ScalarType::F32)
            | (ast::ScalarType::S32, ast::ScalarType::F64)
            | (ast::ScalarType::U16, ast::ScalarType::F16)
            | (ast::ScalarType::U32, ast::ScalarType::F32)
            | (ast::ScalarType::U32, ast::ScalarType::F64) => true,
            _ => false,
        };
        let signed_cast = match signed_cast {
            Some(s) => s,
            None => {
                self.resolver.register(
                    arguments.dst,
                    dst_int_rounded.ok_or_else(error_unreachable)?,
                );
                return Ok(());
            }
        };
        if is_saturating_cast {
            let to = get_scalar_type(self.context, to);
            let src =
                dst_int_rounded.unwrap_or_else(|| self.resolver.value(arguments.src).unwrap());
            let llvm_cast = if signed_cast {
                LLVMBuildFPToSI
            } else {
                LLVMBuildFPToUI
            };
            let poisoned_dst = unsafe { llvm_cast(self.builder, src, to, LLVM_UNNAMED.as_ptr()) };
            self.resolver.with_result(arguments.dst, |dst| unsafe {
                LLVMBuildFreeze(self.builder, poisoned_dst, dst)
            });
        } else {
            let cvt_op = if to.kind() == ptx_parser::ScalarKind::Unsigned {
                "fptoui"
            } else {
                "fptosi"
            };
            let cast_intrinsic = format!(
                "llvm.{cvt_op}.sat.{}.{}\0",
                LLVMTypeDisplay(to),
                LLVMTypeDisplay(from)
            );
            let src =
                dst_int_rounded.unwrap_or_else(|| self.resolver.value(arguments.src).unwrap());
            self.emit_intrinsic(
                unsafe { CStr::from_bytes_with_nul_unchecked(cast_intrinsic.as_bytes()) },
                Some(arguments.dst),
                Some(&to.into()),
                vec![(src, get_scalar_type(self.context, from))],
            )?;
        }
        Ok(())
    }

    fn emit_fp_int_rounding(
        &mut self,
        from: ptx_parser::ScalarType,
        rounding: ptx_parser::RoundingMode,
        arguments: &ptx_parser::CvtArgs<SpirvWord>,
        will_saturate_with_cvt: bool,
    ) -> Result<Option<LLVMValueRef>, TranslateError> {
        let prefix = match rounding {
            ptx_parser::RoundingMode::NearestEven => "llvm.roundeven",
            ptx_parser::RoundingMode::Zero => {
                // cvt has round-to-zero semantics
                if will_saturate_with_cvt {
                    return Ok(None);
                } else {
                    "llvm.trunc"
                }
            }
            ptx_parser::RoundingMode::NegativeInf => "llvm.floor",
            ptx_parser::RoundingMode::PositiveInf => "llvm.ceil",
        };
        let intrinsic = format!("{}.{}\0", prefix, LLVMTypeDisplay(from));
        let rounded_float = self.emit_intrinsic(
            unsafe { CStr::from_bytes_with_nul_unchecked(intrinsic.as_bytes()) },
            None,
            Some(&from.into()),
            vec![(
                self.resolver.value(arguments.src)?,
                get_scalar_type(self.context, from),
            )],
        )?;
        Ok(Some(rounded_float))
    }

    fn emit_cvt_int_to_float(
        &mut self,
        to: ptx_parser::ScalarType,
        arguments: ptx_parser::CvtArgs<SpirvWord>,
        llvm_func: unsafe extern "C" fn(
            LLVMBuilderRef,
            LLVMValueRef,
            LLVMTypeRef,
            *const i8,
        ) -> LLVMValueRef,
    ) -> Result<(), TranslateError> {
        let type_ = get_scalar_type(self.context, to);
        let src = self.resolver.value(arguments.src)?;
        self.resolver.with_result(arguments.dst, |dst| unsafe {
            llvm_func(self.builder, src, type_, dst)
        });
        Ok(())
    }

    fn emit_rsqrt(
        &mut self,
        data: ptx_parser::TypeFtz,
        arguments: ptx_parser::RsqrtArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let type_ = get_scalar_type(self.context, data.type_);
        let intrinsic = match data.type_ {
            ast::ScalarType::F32 => c"llvm.amdgcn.rsq.f32",
            ast::ScalarType::F64 => c"llvm.amdgcn.rsq.f64",
            _ => return Err(error_unreachable()),
        };
        self.emit_intrinsic(
            intrinsic,
            Some(arguments.dst),
            Some(&data.type_.into()),
            vec![(self.resolver.value(arguments.src)?, type_)],
        )?;
        Ok(())
    }

    fn emit_sqrt(
        &mut self,
        data: ptx_parser::RcpData,
        arguments: ptx_parser::SqrtArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let type_ = get_scalar_type(self.context, data.type_);
        let intrinsic = match (data.type_, data.kind) {
            (ast::ScalarType::F32, ast::RcpKind::Approx) => c"llvm.amdgcn.sqrt.f32",
            (ast::ScalarType::F32, ast::RcpKind::Compliant(..)) => c"llvm.sqrt.f32",
            (ast::ScalarType::F64, ast::RcpKind::Compliant(..)) => c"llvm.sqrt.f64",
            _ => return Err(error_unreachable()),
        };
        self.emit_intrinsic(
            intrinsic,
            Some(arguments.dst),
            Some(&data.type_.into()),
            vec![(self.resolver.value(arguments.src)?, type_)],
        )?;
        Ok(())
    }

    fn emit_rcp(
        &mut self,
        data: ptx_parser::RcpData,
        arguments: ptx_parser::RcpArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let type_ = get_scalar_type(self.context, data.type_);
        let intrinsic = match (data.type_, data.kind) {
            (ast::ScalarType::F32, ast::RcpKind::Approx) => c"llvm.amdgcn.rcp.f32",
            (_, ast::RcpKind::Compliant(rnd)) => {
                return self.emit_rcp_compliant(data, arguments, rnd)
            }
            _ => return Err(error_unreachable()),
        };
        self.emit_intrinsic(
            intrinsic,
            Some(arguments.dst),
            Some(&data.type_.into()),
            vec![(self.resolver.value(arguments.src)?, type_)],
        )?;
        Ok(())
    }

    fn emit_rcp_compliant(
        &mut self,
        data: ptx_parser::RcpData,
        arguments: ptx_parser::RcpArgs<SpirvWord>,
        _rnd: ast::RoundingMode,
    ) -> Result<(), TranslateError> {
        let type_ = get_scalar_type(self.context, data.type_);
        let one = unsafe { LLVMConstReal(type_, 1.0) };
        let src = self.resolver.value(arguments.src)?;
        let rcp = self.resolver.with_result(arguments.dst, |dst| unsafe {
            LLVMBuildFDiv(self.builder, one, src, dst)
        });
        unsafe { LLVMZludaSetFastMathFlags(rcp, LLVMZludaFastMathAllowReciprocal) };
        Ok(())
    }

    fn emit_shf(
        &mut self,
        data: ptx_parser::ShfDetails,
        arguments: ShfArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let lsb = self.resolver.value(arguments.src_a)?;
        let msb = self.resolver.value(arguments.src_b)?;
        let shift_amount = self.resolver.value(arguments.src_c)?;

        let llvm_i32 = get_scalar_type(self.context, ast::ScalarType::B32);
        let const_32 = unsafe { LLVMConstInt(llvm_i32, 32, 0) };

        let intrinsic = match data.direction {
            ptx_parser::ShiftDirection::R => c"llvm.fshr.i32",
            ptx_parser::ShiftDirection::L => c"llvm.fshl.i32",
        };

        let shifted = self.emit_intrinsic(
            intrinsic,
            None,
            Some(&ast::Type::Scalar(ptx_parser::ScalarType::B32)),
            vec![(msb, llvm_i32), (lsb, llvm_i32), (shift_amount, llvm_i32)],
        )?;

        if data.mode == FunnelShiftMode::Clamp {
            // `llvm.fsh*` acts like `shf.*.wrap`. To implement clamp, we must conditionally return
            // the left or right-most 32 bits if `shift_amount` is greater than or equal to 32.

            let should_clamp = unsafe {
                LLVMBuildICmp(
                    self.builder,
                    LLVMIntPredicate::LLVMIntUGE,
                    shift_amount,
                    const_32,
                    LLVM_UNNAMED.as_ptr(),
                )
            };

            let max_shift = match data.direction {
                ptx_parser::ShiftDirection::R => msb,
                ptx_parser::ShiftDirection::L => lsb,
            };

            self.resolver.with_result(arguments.dst, |dst| unsafe {
                LLVMBuildSelect(self.builder, should_clamp, max_shift, shifted, dst)
            });
        } else {
            let name = self.resolver.get_or_add(arguments.dst);
            unsafe { LLVMSetValueName2(shifted, name.as_ptr().cast(), name.len()) };
            self.resolver.register(arguments.dst, shifted);
        }

        Ok(())
    }

    fn emit_shr(
        &mut self,
        data: ptx_parser::ShrData,
        arguments: ptx_parser::ShrArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let llvm_type = get_scalar_type(self.context, data.type_);
        let (out_of_range, shift_fn): (
            *mut LLVMValue,
            unsafe extern "C" fn(
                LLVMBuilderRef,
                LLVMValueRef,
                LLVMValueRef,
                *const i8,
            ) -> LLVMValueRef,
        ) = match data.kind {
            ptx_parser::RightShiftKind::Logical => {
                (unsafe { LLVMConstNull(llvm_type) }, LLVMBuildLShr)
            }
            ptx_parser::RightShiftKind::Arithmetic => {
                let src1 = self.resolver.value(arguments.src1)?;
                let shift_size =
                    unsafe { LLVMConstInt(llvm_type, (data.type_.size_of() as u64 * 8) - 1, 0) };
                let out_of_range =
                    unsafe { LLVMBuildAShr(self.builder, src1, shift_size, LLVM_UNNAMED.as_ptr()) };
                (out_of_range, LLVMBuildAShr)
            }
        };
        self.emit_shift(
            data.type_,
            arguments.dst,
            arguments.src1,
            arguments.src2,
            out_of_range,
            shift_fn,
        )
    }

    fn emit_shl(
        &mut self,
        type_: ptx_parser::ScalarType,
        arguments: ptx_parser::ShlArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let llvm_type = get_scalar_type(self.context, type_);
        self.emit_shift(
            type_,
            arguments.dst,
            arguments.src1,
            arguments.src2,
            unsafe { LLVMConstNull(llvm_type) },
            LLVMBuildShl,
        )
    }

    fn emit_shift(
        &mut self,
        type_: ast::ScalarType,
        dst: SpirvWord,
        src1: SpirvWord,
        src2: SpirvWord,
        out_of_range_value: LLVMValueRef,
        llvm_fn: unsafe extern "C" fn(
            LLVMBuilderRef,
            LLVMValueRef,
            LLVMValueRef,
            *const i8,
        ) -> LLVMValueRef,
    ) -> Result<(), TranslateError> {
        let src1 = self.resolver.value(src1)?;
        let shift_size = self.resolver.value(src2)?;
        let integer_bits = type_.layout().size() * 8;
        let integer_bits_constant = unsafe {
            LLVMConstInt(
                get_scalar_type(self.context, ast::ScalarType::U32),
                integer_bits as u64,
                0,
            )
        };
        let should_clamp = unsafe {
            LLVMBuildICmp(
                self.builder,
                LLVMIntPredicate::LLVMIntUGE,
                shift_size,
                integer_bits_constant,
                LLVM_UNNAMED.as_ptr(),
            )
        };
        let llvm_type = get_scalar_type(self.context, type_);
        let normalized_shift_size = if type_.layout().size() >= 4 {
            unsafe {
                LLVMBuildZExtOrBitCast(self.builder, shift_size, llvm_type, LLVM_UNNAMED.as_ptr())
            }
        } else {
            unsafe { LLVMBuildTrunc(self.builder, shift_size, llvm_type, LLVM_UNNAMED.as_ptr()) }
        };
        let shifted = unsafe {
            llvm_fn(
                self.builder,
                src1,
                normalized_shift_size,
                LLVM_UNNAMED.as_ptr(),
            )
        };
        self.resolver.with_result(dst, |dst| unsafe {
            LLVMBuildSelect(self.builder, should_clamp, out_of_range_value, shifted, dst)
        });
        Ok(())
    }

    fn emit_ex2(
        &mut self,
        data: ptx_parser::TypeFtz,
        arguments: ptx_parser::Ex2Args<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let intrinsic = match data.type_ {
            ast::ScalarType::F16 => c"llvm.amdgcn.exp2.f16",
            ast::ScalarType::F32 => c"llvm.amdgcn.exp2.f32",
            _ => return Err(error_unreachable()),
        };
        self.emit_intrinsic(
            intrinsic,
            Some(arguments.dst),
            Some(&data.type_.into()),
            vec![(
                self.resolver.value(arguments.src)?,
                get_scalar_type(self.context, data.type_),
            )],
        )?;
        Ok(())
    }

    fn emit_lg2(
        &mut self,
        _data: ptx_parser::FlushToZero,
        arguments: ptx_parser::Lg2Args<SpirvWord>,
    ) -> Result<(), TranslateError> {
        self.emit_intrinsic(
            c"llvm.amdgcn.log.f32",
            Some(arguments.dst),
            Some(&ast::ScalarType::F32.into()),
            vec![(
                self.resolver.value(arguments.src)?,
                get_scalar_type(self.context, ast::ScalarType::F32),
            )],
        )?;
        Ok(())
    }

    fn emit_selp(
        &mut self,
        _data: ptx_parser::ScalarType,
        arguments: ptx_parser::SelpArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let src1 = self.resolver.value(arguments.src1)?;
        let src2 = self.resolver.value(arguments.src2)?;
        let src3 = self.resolver.value(arguments.src3)?;
        self.resolver.with_result(arguments.dst, |dst_name| unsafe {
            LLVMBuildSelect(self.builder, src3, src1, src2, dst_name)
        });
        Ok(())
    }

    fn emit_rem(
        &mut self,
        data: ptx_parser::ScalarType,
        arguments: ptx_parser::RemArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let llvm_fn = match data.kind() {
            ptx_parser::ScalarKind::Unsigned => LLVMBuildURem,
            ptx_parser::ScalarKind::Signed => LLVMBuildSRem,
            _ => return Err(error_unreachable()),
        };
        let src1 = self.resolver.value(arguments.src1)?;
        let src2 = self.resolver.value(arguments.src2)?;
        self.resolver.with_result(arguments.dst, |dst_name| unsafe {
            llvm_fn(self.builder, src1, src2, dst_name)
        });
        Ok(())
    }

    fn emit_bar_warp(&mut self) -> Result<(), TranslateError> {
        self.emit_intrinsic(c"llvm.amdgcn.wave.barrier", None, None, vec![])?;
        Ok(())
    }

    fn emit_popc(
        &mut self,
        type_: ptx_parser::ScalarType,
        arguments: ptx_parser::PopcArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let intrinsic = match type_ {
            ast::ScalarType::B32 => c"llvm.ctpop.i32",
            ast::ScalarType::B64 => c"llvm.ctpop.i64",
            _ => return Err(error_unreachable()),
        };
        let llvm_type = get_scalar_type(self.context, type_);
        self.emit_intrinsic(
            intrinsic,
            Some(arguments.dst),
            Some(&type_.into()),
            vec![(self.resolver.value(arguments.src)?, llvm_type)],
        )?;
        Ok(())
    }

    fn emit_min(
        &mut self,
        data: ptx_parser::MinMaxDetails,
        arguments: ptx_parser::MinArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let llvm_prefix = match data {
            ptx_parser::MinMaxDetails::Signed(..) => "llvm.smin",
            ptx_parser::MinMaxDetails::Unsigned(..) => "llvm.umin",
            ptx_parser::MinMaxDetails::Float(ptx_parser::MinMaxFloat { .. }) => "llvm.minnum",
        };
        let intrinsic = format!("{}.{}\0", llvm_prefix, LLVMTypeDisplay(data.type_()));
        let llvm_type = get_scalar_type(self.context, data.type_());

        let a = self.resolver.value(arguments.src1)?;
        let b = self.resolver.value(arguments.src2)?;

        let min = self.emit_intrinsic(
            unsafe { CStr::from_bytes_with_nul_unchecked(intrinsic.as_bytes()) },
            None,
            Some(&data.type_().into()),
            vec![(a, llvm_type), (b, llvm_type)],
        )?;

        if let ptx_parser::MinMaxDetails::Float(ptx_parser::MinMaxFloat {
            nan: true, type_, ..
        }) = data
        {
            let is_nan = unsafe {
                LLVMBuildFCmp(
                    self.builder,
                    LLVMRealPredicate::LLVMRealUNO,
                    a,
                    b,
                    LLVM_UNNAMED.as_ptr(),
                )
            };
            self.resolver.with_result(arguments.dst, |dst| unsafe {
                LLVMBuildSelect(
                    self.builder,
                    is_nan,
                    LLVMConstReal(get_scalar_type(self.context, type_), f64::NAN),
                    min,
                    dst,
                )
            });
        } else {
            self.resolver.register(arguments.dst, min);
        }
        Ok(())
    }

    fn emit_max(
        &mut self,
        data: ptx_parser::MinMaxDetails,
        arguments: ptx_parser::MaxArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let llvm_prefix = match data {
            ptx_parser::MinMaxDetails::Signed(..) => "llvm.smax",
            ptx_parser::MinMaxDetails::Unsigned(..) => "llvm.umax",
            ptx_parser::MinMaxDetails::Float(ptx_parser::MinMaxFloat { .. }) => "llvm.maxnum",
        };
        let intrinsic = format!("{}.{}\0", llvm_prefix, LLVMTypeDisplay(data.type_()));
        let llvm_type = get_scalar_type(self.context, data.type_());

        let a = self.resolver.value(arguments.src1)?;
        let b = self.resolver.value(arguments.src2)?;

        let max = self.emit_intrinsic(
            unsafe { CStr::from_bytes_with_nul_unchecked(intrinsic.as_bytes()) },
            None,
            Some(&data.type_().into()),
            vec![(a, llvm_type), (b, llvm_type)],
        )?;

        if let ptx_parser::MinMaxDetails::Float(ptx_parser::MinMaxFloat {
            nan: true, type_, ..
        }) = data
        {
            let is_nan = unsafe {
                LLVMBuildFCmp(
                    self.builder,
                    LLVMRealPredicate::LLVMRealUNO,
                    a,
                    b,
                    LLVM_UNNAMED.as_ptr(),
                )
            };
            self.resolver.with_result(arguments.dst, |dst| unsafe {
                LLVMBuildSelect(
                    self.builder,
                    is_nan,
                    LLVMConstReal(get_scalar_type(self.context, type_), f64::NAN),
                    max,
                    dst,
                )
            });
        } else {
            self.resolver.register(arguments.dst, max);
        }
        Ok(())
    }

    fn emit_fma(
        &mut self,
        data: ptx_parser::ArithFloat,
        arguments: ptx_parser::FmaArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let intrinsic = format!("llvm.fma.{}\0", LLVMTypeDisplay(data.type_));
        self.emit_intrinsic(
            unsafe { CStr::from_bytes_with_nul_unchecked(intrinsic.as_bytes()) },
            Some(arguments.dst),
            Some(&data.type_.into()),
            vec![
                (
                    self.resolver.value(arguments.src1)?,
                    get_scalar_type(self.context, data.type_),
                ),
                (
                    self.resolver.value(arguments.src2)?,
                    get_scalar_type(self.context, data.type_),
                ),
                (
                    self.resolver.value(arguments.src3)?,
                    get_scalar_type(self.context, data.type_),
                ),
            ],
        )?;
        Ok(())
    }

    fn emit_mad(
        &mut self,
        data: ptx_parser::MadDetails,
        arguments: ptx_parser::MadArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let mul_control = match data {
            ptx_parser::MadDetails::Float(mad_float) => {
                return self.emit_fma(
                    mad_float,
                    ast::FmaArgs {
                        dst: arguments.dst,
                        src1: arguments.src1,
                        src2: arguments.src2,
                        src3: arguments.src3,
                    },
                )
            }
            ptx_parser::MadDetails::Integer {
                saturate: true,
                control: ast::MulIntControl::High,
                type_: ast::ScalarType::S32,
            } => {
                return self.emit_mad_hi_sat_s32(
                    arguments.dst,
                    (arguments.src1, arguments.src2, arguments.src3),
                );
            }
            ptx_parser::MadDetails::Integer { saturate: true, .. } => {
                return Err(error_unreachable())
            }
            ptx_parser::MadDetails::Integer { type_, control, .. } => {
                ast::MulDetails::Integer { control, type_ }
            }
        };
        let temp = self.emit_mul_impl(mul_control, None, arguments.src1, arguments.src2)?;
        let src3 = self.resolver.value(arguments.src3)?;
        self.resolver.with_result(arguments.dst, |dst| unsafe {
            LLVMBuildAdd(self.builder, temp, src3, dst)
        });
        Ok(())
    }

    fn emit_membar(&self, data: ptx_parser::MemScope) -> Result<(), TranslateError> {
        unsafe {
            LLVMZludaBuildFence(
                self.builder,
                LLVMAtomicOrdering::LLVMAtomicOrderingSequentiallyConsistent,
                get_scope_membar(data)?,
                LLVM_UNNAMED.as_ptr(),
            )
        };
        Ok(())
    }

    fn emit_abs(
        &mut self,
        data: ast::TypeFtz,
        arguments: ptx_parser::AbsArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let llvm_type = get_scalar_type(self.context, data.type_);
        let src = self.resolver.value(arguments.src)?;
        let is_floating_point = data.type_.kind() == ast::ScalarKind::Float;
        let (prefix, intrinsic_arguments) = if is_floating_point {
            ("llvm.fabs", vec![(src, llvm_type)])
        } else {
            let pred = get_scalar_type(self.context, ast::ScalarType::Pred);
            let zero = unsafe { LLVMConstInt(pred, 0, 0) };
            ("llvm.abs", vec![(src, llvm_type), (zero, pred)])
        };
        let llvm_intrinsic = format!("{}.{}\0", prefix, LLVMTypeDisplay(data.type_));
        let abs_result = self.emit_intrinsic(
            unsafe { CStr::from_bytes_with_nul_unchecked(llvm_intrinsic.as_bytes()) },
            None,
            Some(&data.type_.into()),
            intrinsic_arguments,
        )?;
        if is_floating_point && data.flush_to_zero == Some(true) {
            let intrinsic = format!("llvm.canonicalize.{}\0", LLVMTypeDisplay(data.type_));
            self.emit_intrinsic(
                unsafe { CStr::from_bytes_with_nul_unchecked(intrinsic.as_bytes()) },
                Some(arguments.dst),
                Some(&data.type_.into()),
                vec![(abs_result, llvm_type)],
            )?;
        } else {
            self.resolver.register(arguments.dst, abs_result);
        }
        Ok(())
    }

    fn emit_mul24(
        &mut self,
        data: ast::Mul24Details,
        arguments: ast::Mul24Args<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let src1 = self.resolver.value(arguments.src1)?;
        let src2 = self.resolver.value(arguments.src2)?;
        let name_lo = match data.type_ {
            ast::ScalarType::U32 => c"llvm.amdgcn.mul.u24",
            ast::ScalarType::S32 => c"llvm.amdgcn.mul.i24",
            _ => return Err(error_unreachable()),
        };
        let res_lo = self.emit_intrinsic(
            name_lo,
            if data.control == Mul24Control::Lo {
                Some(arguments.dst)
            } else {
                None
            },
            Some(&ast::Type::Scalar(data.type_)),
            vec![
                (src1, get_scalar_type(self.context, data.type_)),
                (src2, get_scalar_type(self.context, data.type_)),
            ],
        )?;
        if data.control == Mul24Control::Hi {
            // There is an important difference between NVIDIA's mul24.hi and AMD's mulhi.[ui]24.
            // NVIDIA: Returns bits 47..16 of the 64-bit result
            // AMD: Returns bits 63..32 of the 64-bit result
            // Hence we need to compute both hi and lo, shift the results and add them together to replicate NVIDIA's mul24
            let name_hi = match data.type_ {
                ast::ScalarType::U32 => c"llvm.amdgcn.mulhi.u24",
                ast::ScalarType::S32 => c"llvm.amdgcn.mulhi.i24",
                _ => return Err(error_unreachable()),
            };
            let res_hi = self.emit_intrinsic(
                name_hi,
                None,
                Some(&ast::Type::Scalar(data.type_)),
                vec![
                    (src1, get_scalar_type(self.context, data.type_)),
                    (src2, get_scalar_type(self.context, data.type_)),
                ],
            )?;
            let shift_number = unsafe { LLVMConstInt(LLVMInt32TypeInContext(self.context), 16, 0) };
            let res_lo_shr =
                unsafe { LLVMBuildLShr(self.builder, res_lo, shift_number, LLVM_UNNAMED.as_ptr()) };
            let res_hi_shl =
                unsafe { LLVMBuildShl(self.builder, res_hi, shift_number, LLVM_UNNAMED.as_ptr()) };

            self.resolver
                .with_result(arguments.dst, |dst: *const i8| unsafe {
                    LLVMBuildOr(self.builder, res_lo_shr, res_hi_shl, dst)
                });
        }
        Ok(())
    }

    fn emit_set_mode(&mut self, mode_reg: ModeRegister) -> Result<(), TranslateError> {
        fn hwreg(reg: u32, offset: u32, size: u32) -> u32 {
            reg | (offset << 6) | ((size - 1) << 11)
        }
        fn denormal_to_value(ftz: bool) -> u32 {
            if ftz {
                0
            } else {
                3
            }
        }
        fn rounding_to_value(ftz: ast::RoundingMode) -> u32 {
            match ftz {
                ptx_parser::RoundingMode::NearestEven => 0,
                ptx_parser::RoundingMode::Zero => 3,
                ptx_parser::RoundingMode::NegativeInf => 2,
                ptx_parser::RoundingMode::PositiveInf => 1,
            }
        }
        fn merge_regs(f32: u32, f16f64: u32) -> u32 {
            f32 | f16f64 << 2
        }
        let intrinsic = c"llvm.amdgcn.s.setreg";
        let (hwreg, value) = match mode_reg {
            ModeRegister::Denormal { f32, f16f64 } => {
                let hwreg = hwreg(1, 4, 4);
                let f32 = denormal_to_value(f32);
                let f16f64 = denormal_to_value(f16f64);
                let value = merge_regs(f32, f16f64);
                (hwreg, value)
            }
            ModeRegister::Rounding { f32, f16f64 } => {
                let hwreg = hwreg(1, 0, 4);
                let f32 = rounding_to_value(f32);
                let f16f64 = rounding_to_value(f16f64);
                let value = merge_regs(f32, f16f64);
                (hwreg, value)
            }
        };
        let llvm_i32 = get_scalar_type(self.context, ast::ScalarType::B32);
        let hwreg_llvm = unsafe { LLVMConstInt(llvm_i32, hwreg as _, 0) };
        let value_llvm = unsafe { LLVMConstInt(llvm_i32, value as _, 0) };
        self.emit_intrinsic(
            intrinsic,
            None,
            None,
            vec![(hwreg_llvm, llvm_i32), (value_llvm, llvm_i32)],
        )?;
        Ok(())
    }

    fn emit_fp_saturate(
        &mut self,
        type_: ast::ScalarType,
        dst: SpirvWord,
        src: SpirvWord,
    ) -> Result<(), TranslateError> {
        let llvm_type = get_scalar_type(self.context, type_);
        let zero = unsafe { LLVMConstReal(llvm_type, 0.0) };
        let one = unsafe { LLVMConstReal(llvm_type, 1.0) };
        let maxnum_intrinsic = format!("llvm.maxnum.{}\0", LLVMTypeDisplay(type_));
        let minnum_intrinsic = format!("llvm.minnum.{}\0", LLVMTypeDisplay(type_));
        let src = self.resolver.value(src)?;
        let maxnum = self.emit_intrinsic(
            unsafe { CStr::from_bytes_with_nul_unchecked(maxnum_intrinsic.as_bytes()) },
            None,
            Some(&type_.into()),
            vec![(src, llvm_type), (zero, llvm_type)],
        )?;
        self.emit_intrinsic(
            unsafe { CStr::from_bytes_with_nul_unchecked(minnum_intrinsic.as_bytes()) },
            Some(dst),
            Some(&type_.into()),
            vec![(maxnum, llvm_type), (one, llvm_type)],
        )?;
        Ok(())
    }

    fn emit_intrinsic_saturate(
        &mut self,
        op: &str,
        type_: ast::ScalarType,
        dst: SpirvWord,
        src1: SpirvWord,
        src2: SpirvWord,
    ) -> Result<(), TranslateError> {
        let llvm_type = get_scalar_type(self.context, type_);
        let src1 = self.resolver.value(src1)?;
        let src2 = self.resolver.value(src2)?;
        let intrinsic = format!("llvm.{}.sat.{}\0", op, LLVMTypeDisplay(type_));
        self.emit_intrinsic(
            unsafe { CStr::from_bytes_with_nul_unchecked(intrinsic.as_bytes()) },
            Some(dst),
            Some(&type_.into()),
            vec![(src1, llvm_type), (src2, llvm_type)],
        )?;
        Ok(())
    }

    fn emit_cp_async(
        &mut self,
        data: CpAsyncDetails,
        arguments: CpAsyncArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        // Asynchronous copies are not supported by all AMD hardware, so we just do a synchronous copy for now
        let to = self.resolver.value(arguments.src_to)?;
        let from = self.resolver.value(arguments.src_from)?;
        let cp_size = data.cp_size;
        let src_size = data.src_size.unwrap_or(cp_size.as_u64());

        let from_type = unsafe { LLVMIntTypeInContext(self.context, (src_size as u32) * 8) };

        let to_type = match cp_size {
            ptx_parser::CpAsyncCpSize::Bytes4 => unsafe { LLVMInt32TypeInContext(self.context) },
            ptx_parser::CpAsyncCpSize::Bytes8 => unsafe { LLVMInt64TypeInContext(self.context) },
            ptx_parser::CpAsyncCpSize::Bytes16 => unsafe { LLVMInt128TypeInContext(self.context) },
        };

        let load = unsafe { LLVMBuildLoad2(self.builder, from_type, from, LLVM_UNNAMED.as_ptr()) };
        unsafe {
            LLVMSetAlignment(load, cp_size.as_u64() as u32);
        }

        let extended = unsafe { LLVMBuildZExt(self.builder, load, to_type, LLVM_UNNAMED.as_ptr()) };

        let store = unsafe { LLVMBuildStore(self.builder, extended, to) };
        unsafe {
            LLVMSetAlignment(store, cp_size.as_u64() as u32);
        }
        Ok(())
    }

    fn flush_denormals(
        &mut self,
        type_: ptx_parser::ScalarType,
        src: SpirvWord,
        dst: SpirvWord,
    ) -> Result<(), TranslateError> {
        let llvm_type = get_scalar_type(self.context, type_);
        let src = self.resolver.value(src)?;
        let intrinsic = format!("llvm.canonicalize.{}\0", LLVMTypeDisplay(type_));
        self.emit_intrinsic(
            unsafe { CStr::from_bytes_with_nul_unchecked(intrinsic.as_bytes()) },
            Some(dst),
            Some(&type_.into()),
            vec![(src, llvm_type)],
        )?;
        Ok(())
    }

    fn emit_mad_hi_sat_s32(
        &mut self,
        dst: SpirvWord,
        (src1, src2, src3): (SpirvWord, SpirvWord, SpirvWord),
    ) -> Result<(), TranslateError> {
        let src1 = self.resolver.value(src1)?;
        let src2 = self.resolver.value(src2)?;
        let src3 = self.resolver.value(src3)?;
        let llvm_type_s32 = get_scalar_type(self.context, ast::ScalarType::S32);
        let llvm_type_s64 = get_scalar_type(self.context, ast::ScalarType::S64);
        let src1_wide =
            unsafe { LLVMBuildSExt(self.builder, src1, llvm_type_s64, LLVM_UNNAMED.as_ptr()) };
        let src2_wide =
            unsafe { LLVMBuildSExt(self.builder, src2, llvm_type_s64, LLVM_UNNAMED.as_ptr()) };
        let mul_wide =
            unsafe { LLVMBuildMul(self.builder, src1_wide, src2_wide, LLVM_UNNAMED.as_ptr()) };
        let const_32 = unsafe { LLVMConstInt(llvm_type_s64, 32, 0) };
        let mul_wide =
            unsafe { LLVMBuildLShr(self.builder, mul_wide, const_32, LLVM_UNNAMED.as_ptr()) };
        let mul_narrow =
            unsafe { LLVMBuildTrunc(self.builder, mul_wide, llvm_type_s32, LLVM_UNNAMED.as_ptr()) };
        self.emit_intrinsic(
            c"llvm.sadd.sat.i32",
            Some(dst),
            Some(&ast::ScalarType::S32.into()),
            vec![(mul_narrow, llvm_type_s32), (src3, llvm_type_s32)],
        )?;
        Ok(())
    }

    fn emit_set(
        &mut self,
        data: ptx_parser::SetData,
        arguments: ptx_parser::SetArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let setp_result = self.emit_setp_impl(data.base, None, arguments.src1, arguments.src2)?;
        self.setp_to_set(arguments.dst, data.dtype, setp_result)?;
        Ok(())
    }

    fn emit_set_bool(
        &mut self,
        data: ptx_parser::SetBoolData,
        arguments: ptx_parser::SetBoolArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let result =
            self.emit_setp_bool_impl(data.base, arguments.src1, arguments.src2, arguments.src3)?;
        self.setp_to_set(arguments.dst, data.dtype, result)?;
        Ok(())
    }

    fn emit_setp_bool(
        &mut self,
        data: ast::SetpBoolData,
        args: ast::SetpBoolArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let dst = self.emit_setp_bool_impl(data, args.src1, args.src2, args.src3)?;
        self.resolver.register(args.dst1, dst);
        Ok(())
    }

    fn emit_setp_bool_impl(
        &mut self,
        data: ptx_parser::SetpBoolData,
        src1: SpirvWord,
        src2: SpirvWord,
        src3: SpirvWord,
    ) -> Result<LLVMValueRef, TranslateError> {
        let bool_result = self.emit_setp_impl(data.base, None, src1, src2)?;
        let bool_result = if data.negate_src3 {
            let constant =
                unsafe { LLVMConstInt(LLVMIntTypeInContext(self.context, 1), u64::MAX, 0) };
            unsafe { LLVMBuildXor(self.builder, bool_result, constant, LLVM_UNNAMED.as_ptr()) }
        } else {
            bool_result
        };
        let post_op = match data.bool_op {
            ptx_parser::SetpBoolPostOp::Xor => LLVMBuildXor,
            ptx_parser::SetpBoolPostOp::And => LLVMBuildAnd,
            ptx_parser::SetpBoolPostOp::Or => LLVMBuildOr,
        };
        let src3 = self.resolver.value(src3)?;
        Ok(unsafe { post_op(self.builder, bool_result, src3, LLVM_UNNAMED.as_ptr()) })
    }

    fn setp_to_set(
        &mut self,
        dst: SpirvWord,
        dtype: ast::ScalarType,
        setp_result: LLVMValueRef,
    ) -> Result<(), TranslateError> {
        let llvm_dtype = get_scalar_type(self.context, dtype);
        let zero = unsafe { LLVMConstNull(llvm_dtype) };
        let one = if dtype.kind() == ast::ScalarKind::Float {
            unsafe { LLVMConstReal(llvm_dtype, 1.0) }
        } else {
            unsafe { LLVMConstInt(llvm_dtype, u64::MAX, 0) }
        };
        self.resolver.with_result(dst, |dst| unsafe {
            LLVMBuildSelect(self.builder, setp_result, one, zero, dst)
        });
        Ok(())
    }

    // TODO: revisit this on gfx1250 which has native tanh support
    fn emit_tanh(
        &mut self,
        data: ast::ScalarType,
        arguments: ast::TanhArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let src = self.resolver.value(arguments.src)?;
        let llvm_type = get_scalar_type(self.context, data);
        let name = format!("__ocml_tanh_{}\0", LLVMTypeDisplay(data));
        let tanh = self.emit_intrinsic(
            unsafe { CStr::from_bytes_with_nul_unchecked(name.as_bytes()) },
            Some(arguments.dst),
            Some(&data.into()),
            vec![(src, llvm_type)],
        )?;
        // Not sure if it ultimately does anything
        unsafe { LLVMZludaSetFastMathFlags(tanh, LLVMZludaFastMathApproxFunc) }
        Ok(())
    }

    fn emit_dp4a(
        &mut self,
        data: ast::Dp4aDetails,
        arguments: ast::Dp4aArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let intrinsic = match (data.atype, data.btype) {
            (ast::ScalarType::U32, ast::ScalarType::U32) => c"llvm.amdgcn.udot4",
            (ast::ScalarType::S32, ast::ScalarType::S32) => c"llvm.amdgcn.sdot4",
            (ast::ScalarType::U32, ast::ScalarType::S32)
            | (ast::ScalarType::S32, ast::ScalarType::U32) => {
                return Err(error_todo_msg("dp4a with mixed types is not yet supported"))
            }
            _ => return Err(error_unreachable()),
        };
        let pred = get_scalar_type(self.context, ast::ScalarType::Pred);
        let zero = unsafe { LLVMConstInt(pred, 0, 0) };
        self.emit_intrinsic(
            intrinsic,
            Some(arguments.dst),
            Some(&data.ctype().into()),
            vec![
                (
                    self.resolver.value(arguments.src1)?,
                    get_scalar_type(self.context, data.ctype()),
                ),
                (
                    self.resolver.value(arguments.src2)?,
                    get_scalar_type(self.context, data.ctype()),
                ),
                (
                    self.resolver.value(arguments.src3)?,
                    get_scalar_type(self.context, data.ctype()),
                ),
                (zero, pred),
            ],
        )?;
        Ok(())
    }

    fn emit_trap(&mut self) -> Result<(), TranslateError> {
        self.emit_intrinsic(c"llvm.trap", None, None, vec![])?;
        unsafe { LLVMBuildUnreachable(self.builder) };
        Ok(())
    }

    fn emit_copysign(
        &mut self,
        type_: ast::ScalarType,
        arguments: ast::CopysignArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let src1 = self.resolver.value(arguments.src1)?;
        let src2 = self.resolver.value(arguments.src2)?;
        let intrinsic = format!("llvm.copysign.{}\0", LLVMTypeDisplay(type_));
        self.emit_intrinsic(
            unsafe { CStr::from_bytes_with_nul_unchecked(intrinsic.as_bytes()) },
            Some(arguments.dst),
            Some(&type_.into()),
            vec![
                (src2, get_scalar_type(self.context, type_)),
                (src1, get_scalar_type(self.context, type_)),
            ],
        )?;
        Ok(())
    }

    /*
    // Currently unused, LLVM 18 (ROCm 6.2) does not support `llvm.set.rounding`
    // Should be available in LLVM 19
    fn with_rounding<T>(&mut self, rnd: ast::RoundingMode, fn_: impl FnOnce(&mut Self) -> T) -> T {
        let mut u32_type = get_scalar_type(self.context, ast::ScalarType::U32);
        let void_type = unsafe { LLVMVoidTypeInContext(self.context) };
        let get_rounding = c"llvm.get.rounding";
        let get_rounding_fn_type = unsafe { LLVMFunctionType(u32_type, ptr::null_mut(), 0, 0) };
        let mut get_rounding_fn =
            unsafe { LLVMGetNamedFunction(self.module, get_rounding.as_ptr()) };
        if get_rounding_fn == ptr::null_mut() {
            get_rounding_fn = unsafe {
                LLVMAddFunction(self.module, get_rounding.as_ptr(), get_rounding_fn_type)
            };
        }
        let set_rounding = c"llvm.set.rounding";
        let set_rounding_fn_type = unsafe { LLVMFunctionType(void_type, &mut u32_type, 1, 0) };
        let mut set_rounding_fn =
            unsafe { LLVMGetNamedFunction(self.module, set_rounding.as_ptr()) };
        if set_rounding_fn == ptr::null_mut() {
            set_rounding_fn = unsafe {
                LLVMAddFunction(self.module, set_rounding.as_ptr(), set_rounding_fn_type)
            };
        }
        let mut preserved_rounding_mode = unsafe {
            LLVMBuildCall2(
                self.builder,
                get_rounding_fn_type,
                get_rounding_fn,
                ptr::null_mut(),
                0,
                LLVM_UNNAMED.as_ptr(),
            )
        };
        let mut requested_rounding = unsafe {
            LLVMConstInt(
                get_scalar_type(self.context, ast::ScalarType::B32),
                rounding_to_llvm(rnd) as u64,
                0,
            )
        };
        unsafe {
            LLVMBuildCall2(
                self.builder,
                set_rounding_fn_type,
                set_rounding_fn,
                &mut requested_rounding,
                1,
                LLVM_UNNAMED.as_ptr(),
            )
        };
        let result = fn_(self);
        unsafe {
            LLVMBuildCall2(
                self.builder,
                set_rounding_fn_type,
                set_rounding_fn,
                &mut preserved_rounding_mode,
                1,
                LLVM_UNNAMED.as_ptr(),
            )
        };
        result
    }
     */
}

fn not_supported_by_atomics(qualifier: ast::LdStQualifier, underlying_type: *mut LLVMType) -> bool {
    // This is not meant to be 100% accurate, just a best-effort guess for atomics
    fn is_non_scalar_type(type_: LLVMTypeRef) -> bool {
        let kind = unsafe { LLVMGetTypeKind(type_) };
        matches!(
            kind,
            LLVMTypeKind::LLVMArrayTypeKind
                | LLVMTypeKind::LLVMVectorTypeKind
                | LLVMTypeKind::LLVMStructTypeKind
        )
    }
    !matches!(qualifier, ast::LdStQualifier::Weak) && is_non_scalar_type(underlying_type)
}

fn apply_qualifier(
    value: LLVMValueRef,
    qualifier: ptx_parser::LdStQualifier,
) -> Result<(), TranslateError> {
    match qualifier {
        ptx_parser::LdStQualifier::Weak => {}
        ptx_parser::LdStQualifier::Volatile => unsafe {
            LLVMSetVolatile(value, 1);
            // The semantics of volatile operations are equivalent to a relaxed memory operation
            // with system-scope but with the following extra implementation-specific constraints...
            LLVMZludaSetAtomic(
                value,
                LLVMAtomicOrdering::LLVMAtomicOrderingMonotonic,
                get_scope(ast::MemScope::Sys)?,
            );
        },
        ptx_parser::LdStQualifier::Relaxed(mem_scope) => unsafe {
            LLVMZludaSetAtomic(
                value,
                LLVMAtomicOrdering::LLVMAtomicOrderingMonotonic,
                get_scope(mem_scope)?,
            );
        },
        ptx_parser::LdStQualifier::Acquire(mem_scope) => unsafe {
            LLVMZludaSetAtomic(
                value,
                LLVMAtomicOrdering::LLVMAtomicOrderingAcquire,
                get_scope(mem_scope)?,
            );
        },
        ptx_parser::LdStQualifier::Release(mem_scope) => unsafe {
            LLVMZludaSetAtomic(
                value,
                LLVMAtomicOrdering::LLVMAtomicOrderingRelease,
                get_scope(mem_scope)?,
            );
        },
    }
    Ok(())
}

fn get_pointer_type<'ctx>(
    context: LLVMContextRef,
    to_space: ast::StateSpace,
) -> Result<LLVMTypeRef, TranslateError> {
    Ok(unsafe { LLVMPointerTypeInContext(context, get_state_space(to_space)?) })
}

// https://llvm.org/docs/AMDGPUUsage.html#memory-scopes
fn get_scope(scope: ast::MemScope) -> Result<*const i8, TranslateError> {
    Ok(match scope {
        ast::MemScope::Cta => c"workgroup-one-as",
        ast::MemScope::Gpu => c"agent-one-as",
        ast::MemScope::Sys => c"one-as",
        ast::MemScope::Cluster => return Err(error_todo()),
    }
    .as_ptr())
}

fn get_scope_membar(scope: ast::MemScope) -> Result<*const i8, TranslateError> {
    Ok(match scope {
        ast::MemScope::Cta => c"workgroup",
        ast::MemScope::Gpu => c"agent",
        // Don't change to "system", this is the same as __threadfence_system,  AMDPGU LLVM expects "" here
        ast::MemScope::Sys => c"",
        ast::MemScope::Cluster => return Err(error_todo()),
    }
    .as_ptr())
}

fn get_ordering(semantics: ast::AtomSemantics) -> LLVMAtomicOrdering {
    match semantics {
        ast::AtomSemantics::Relaxed => LLVMAtomicOrdering::LLVMAtomicOrderingMonotonic,
        ast::AtomSemantics::Acquire => LLVMAtomicOrdering::LLVMAtomicOrderingAcquire,
        ast::AtomSemantics::Release => LLVMAtomicOrdering::LLVMAtomicOrderingRelease,
        ast::AtomSemantics::AcqRel => LLVMAtomicOrdering::LLVMAtomicOrderingAcquireRelease,
    }
}

fn get_ordering_failure(semantics: ast::AtomSemantics) -> LLVMAtomicOrdering {
    match semantics {
        ast::AtomSemantics::Relaxed => LLVMAtomicOrdering::LLVMAtomicOrderingMonotonic,
        ast::AtomSemantics::Acquire => LLVMAtomicOrdering::LLVMAtomicOrderingAcquire,
        ast::AtomSemantics::Release => LLVMAtomicOrdering::LLVMAtomicOrderingAcquire,
        ast::AtomSemantics::AcqRel => LLVMAtomicOrdering::LLVMAtomicOrderingAcquire,
    }
}

fn get_type(context: LLVMContextRef, type_: &ast::Type) -> Result<LLVMTypeRef, TranslateError> {
    Ok(match type_ {
        ast::Type::Scalar(scalar) => get_scalar_type(context, *scalar),
        ast::Type::Vector(size, scalar) => {
            let base_type = get_scalar_type(context, *scalar);
            unsafe { LLVMVectorType(base_type, *size as u32) }
        }
        ast::Type::Array(vec, scalar, dimensions) => {
            let mut underlying_type = get_scalar_type(context, *scalar);
            if let Some(size) = vec {
                underlying_type = unsafe { LLVMVectorType(underlying_type, size.get() as u32) };
            }
            if dimensions.is_empty() {
                return Ok(unsafe { LLVMArrayType2(underlying_type, 0) });
            }
            dimensions
                .iter()
                .rfold(underlying_type, |result, dimension| unsafe {
                    LLVMArrayType2(result, *dimension as u64)
                })
        }
    })
}

fn get_or_create_struct_type<'a>(
    context: LLVMContextRef,
    mut elem_types: impl Iterator<Item = &'a ast::Type>,
) -> Result<LLVMTypeRef, TranslateError> {
    use std::fmt::Write;
    let (mut name, types) = elem_types.try_fold(
        ("struct".to_string(), Vec::new()),
        |(mut name, mut types), t| {
            name.push('.');
            if let ast::Type::Scalar(scalar) = t {
                write!(name, "{}", LLVMTypeDisplay(*scalar)).ok();
            } else {
                return Err(error_unreachable());
            }
            types.push(get_type(context, t)?);
            Ok((name, types))
        },
    )?;
    name.push('\0');
    let mut struct_type = unsafe { LLVMGetTypeByName2(context, name.as_ptr().cast()) };
    if struct_type.is_null() {
        struct_type = create_struct_type(context, name, types);
    }
    Ok(struct_type)
}

fn create_struct_type(
    context: LLVMContextRef,
    name: String,
    mut elem_types: Vec<LLVMTypeRef>,
) -> LLVMTypeRef {
    let llvm_type = unsafe { LLVMStructCreateNamed(context, name.as_ptr().cast()) };
    unsafe {
        LLVMStructSetBody(
            llvm_type,
            elem_types.as_mut_ptr(),
            elem_types.len() as u32,
            0,
        )
    }
    llvm_type
}

fn get_function_type<'a>(
    context: LLVMContextRef,
    mut return_args: impl DoubleEndedIterator<Item = &'a ast::Type>
        + ExactSizeIterator<Item = &'a ast::Type>,
    input_args: impl ExactSizeIterator<Item = Result<LLVMTypeRef, TranslateError>>,
) -> Result<LLVMTypeRef, TranslateError> {
    let mut input_args = input_args.collect::<Result<Vec<_>, _>>()?;
    let return_type = match return_args.len() {
        0 => unsafe { LLVMVoidTypeInContext(context) },
        1 => get_type(context, &return_args.next().unwrap())?,
        _ => get_or_create_struct_type(context, return_args)?,
    };

    Ok(unsafe {
        LLVMFunctionType(
            return_type,
            input_args.as_mut_ptr(),
            input_args.len() as u32,
            0,
        )
    })
}

struct ResolveIdent {
    words: HashMap<SpirvWord, String>,
    values: HashMap<SpirvWord, LLVMValueRef>,
}

impl ResolveIdent {
    fn new<'input>(_id_defs: &GlobalStringIdentResolver2<'input>) -> Self {
        ResolveIdent {
            words: HashMap::new(),
            values: HashMap::new(),
        }
    }

    fn get_or_ad_impl<'a, T>(&'a mut self, word: SpirvWord, fn_: impl FnOnce(&'a str) -> T) -> T {
        let str = match self.words.entry(word) {
            hash_map::Entry::Occupied(entry) => entry.into_mut(),
            hash_map::Entry::Vacant(entry) => {
                let mut text = word.0.to_string();
                text.push('\0');
                entry.insert(text)
            }
        };
        fn_(&str[..str.len() - 1])
    }

    fn get_or_add(&mut self, word: SpirvWord) -> &str {
        self.get_or_ad_impl(word, |x| x)
    }

    fn get_or_add_raw(&mut self, word: SpirvWord) -> *const i8 {
        self.get_or_add(word).as_ptr().cast()
    }

    fn register(&mut self, word: SpirvWord, v: LLVMValueRef) {
        self.values.insert(word, v);
    }

    fn value(&self, word: SpirvWord) -> Result<LLVMValueRef, TranslateError> {
        self.values
            .get(&word)
            .copied()
            .ok_or_else(|| error_unreachable())
    }

    fn with_result(
        &mut self,
        word: SpirvWord,
        fn_: impl FnOnce(*const i8) -> LLVMValueRef,
    ) -> LLVMValueRef {
        let t = self.get_or_ad_impl(word, |dst| fn_(dst.as_ptr().cast()));
        self.register(word, t);
        t
    }

    fn with_result_option(
        &mut self,
        word: Option<SpirvWord>,
        fn_: impl FnOnce(*const i8) -> LLVMValueRef,
    ) -> LLVMValueRef {
        match word {
            Some(word) => self.with_result(word, fn_),
            None => fn_(LLVM_UNNAMED.as_ptr()),
        }
    }
}

struct LLVMTypeDisplay(ast::ScalarType);

impl std::fmt::Display for LLVMTypeDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            ast::ScalarType::Pred => write!(f, "i1"),
            ast::ScalarType::B8 | ast::ScalarType::U8 | ast::ScalarType::S8 => write!(f, "i8"),
            ast::ScalarType::B16
            | ast::ScalarType::U16
            | ast::ScalarType::S16
            | ast::ScalarType::E4m3x2
            | ast::ScalarType::E5m2x2 => write!(f, "i16"),
            ast::ScalarType::B32 | ast::ScalarType::U32 | ast::ScalarType::S32 => write!(f, "i32"),
            ast::ScalarType::B64 | ast::ScalarType::U64 | ast::ScalarType::S64 => write!(f, "i64"),
            ptx_parser::ScalarType::B128 => write!(f, "i128"),
            ast::ScalarType::F16 => write!(f, "f16"),
            ptx_parser::ScalarType::BF16 => write!(f, "bfloat"),
            ast::ScalarType::F32 => write!(f, "f32"),
            ast::ScalarType::F64 => write!(f, "f64"),
            ptx_parser::ScalarType::S16x2 | ptx_parser::ScalarType::U16x2 => write!(f, "v2i16"),
            ast::ScalarType::F16x2 => write!(f, "v2f16"),
            ptx_parser::ScalarType::BF16x2 => write!(f, "v2bf16"),
        }
    }
}

/*
fn rounding_to_llvm(this: ast::RoundingMode) -> u32 {
    match this {
        ptx_parser::RoundingMode::Zero => 0,
        ptx_parser::RoundingMode::NearestEven => 1,
        ptx_parser::RoundingMode::PositiveInf => 2,
        ptx_parser::RoundingMode::NegativeInf => 3,
    }
}
*/
