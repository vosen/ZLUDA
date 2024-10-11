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

use std::array::TryFromSliceError;
use std::convert::TryInto;
use std::ffi::{CStr, NulError};
use std::ops::Deref;
use std::{i8, ptr};

use super::*;
use llvm_zluda::analysis::{LLVMVerifierFailureAction, LLVMVerifyModule};
use llvm_zluda::bit_writer::LLVMWriteBitcodeToMemoryBuffer;
use llvm_zluda::{core::*, *};
use llvm_zluda::{prelude::*, LLVMZludaBuildAtomicRMW};
use llvm_zluda::{LLVMCallConv, LLVMZludaBuildAlloca};

const LLVM_UNNAMED: &CStr = c"";
// https://llvm.org/docs/AMDGPUUsage.html#address-spaces
const GENERIC_ADDRESS_SPACE: u32 = 0;
const GLOBAL_ADDRESS_SPACE: u32 = 1;
const SHARED_ADDRESS_SPACE: u32 = 3;
const CONSTANT_ADDRESS_SPACE: u32 = 4;
const PRIVATE_ADDRESS_SPACE: u32 = 5;

struct Context(LLVMContextRef);

impl Context {
    fn new() -> Self {
        Self(unsafe { LLVMContextCreate() })
    }

    fn get(&self) -> LLVMContextRef {
        self.0
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            LLVMContextDispose(self.0);
        }
    }
}

struct Module(LLVMModuleRef);

impl Module {
    fn new(ctx: &Context, name: &CStr) -> Self {
        Self(unsafe { LLVMModuleCreateWithNameInContext(name.as_ptr(), ctx.get()) })
    }

    fn get(&self) -> LLVMModuleRef {
        self.0
    }

    fn verify(&self) -> Result<(), Message> {
        let mut err = ptr::null_mut();
        let error = unsafe {
            LLVMVerifyModule(
                self.get(),
                LLVMVerifierFailureAction::LLVMReturnStatusAction,
                &mut err,
            )
        };
        if error == 1 && err != ptr::null_mut() {
            Err(Message(unsafe { CStr::from_ptr(err) }))
        } else {
            Ok(())
        }
    }

    fn write_bitcode_to_memory(&self) -> MemoryBuffer {
        let memory_buffer = unsafe { LLVMWriteBitcodeToMemoryBuffer(self.get()) };
        MemoryBuffer(memory_buffer)
    }

    fn write_to_stderr(&self) {
        unsafe { LLVMDumpModule(self.get()) };
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeModule(self.0);
        }
    }
}

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

struct Message(&'static CStr);

impl Drop for Message {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeMessage(self.0.as_ptr().cast_mut());
        }
    }
}

impl std::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

pub struct MemoryBuffer(LLVMMemoryBufferRef);

impl Drop for MemoryBuffer {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeMemoryBuffer(self.0);
        }
    }
}

impl Deref for MemoryBuffer {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        let data = unsafe { LLVMGetBufferStart(self.0) };
        let len = unsafe { LLVMGetBufferSize(self.0) };
        unsafe { std::slice::from_raw_parts(data.cast(), len) }
    }
}

pub(super) fn run<'input>(
    id_defs: GlobalStringIdentResolver2<'input>,
    directives: Vec<Directive2<'input, ast::Instruction<SpirvWord>, SpirvWord>>,
) -> Result<MemoryBuffer, TranslateError> {
    let context = Context::new();
    let module = Module::new(&context, LLVM_UNNAMED);
    let mut emit_ctx = ModuleEmitContext::new(&context, &module, &id_defs);
    for directive in directives {
        match directive {
            Directive2::Variable(linking, variable) => emit_ctx.emit_global(linking, variable)?,
            Directive2::Method(method) => emit_ctx.emit_method(method)?,
        }
    }
    module.write_to_stderr();
    if let Err(err) = module.verify() {
        panic!("{:?}", err);
    }
    Ok(module.write_bitcode_to_memory())
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
        module: &Module,
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
        method: Function2<'input, ast::Instruction<SpirvWord>, SpirvWord>,
    ) -> Result<(), TranslateError> {
        let func_decl = method.func_decl;
        let name = method
            .import_as
            .as_deref()
            .or_else(|| match func_decl.name {
                ast::MethodName::Kernel(name) => Some(name),
                ast::MethodName::Func(id) => self.id_defs.ident_map[&id].name.as_deref(),
            })
            .ok_or_else(|| error_unreachable())?;
        let name = CString::new(name).map_err(|_| error_unreachable())?;
        let mut fn_ = unsafe { LLVMGetNamedFunction(self.module, name.as_ptr()) };
        if fn_ == ptr::null_mut() {
            let fn_type = get_function_type(
                self.context,
                func_decl.return_arguments.iter().map(|v| &v.v_type),
                func_decl
                    .input_arguments
                    .iter()
                    .map(|v| get_input_argument_type(self.context, &v.v_type, v.state_space)),
            )?;
            fn_ = unsafe { LLVMAddFunction(self.module, name.as_ptr(), fn_type) };
        }
        if let ast::MethodName::Func(name) = func_decl.name {
            self.resolver.register(name, fn_);
        }
        for (i, param) in func_decl.input_arguments.iter().enumerate() {
            let value = unsafe { LLVMGetParam(fn_, i as u32) };
            let name = self.resolver.get_or_add(param.name);
            unsafe { LLVMSetValueName2(value, name.as_ptr().cast(), name.len()) };
            self.resolver.register(param.name, value);
            if func_decl.name.is_kernel() {
                let attr_kind = unsafe {
                    LLVMGetEnumAttributeKindForName(b"byref".as_ptr().cast(), b"byref".len())
                };
                let attr = unsafe {
                    LLVMCreateTypeAttribute(
                        self.context,
                        attr_kind,
                        get_type(self.context, &param.v_type)?,
                    )
                };
                unsafe { LLVMAddAttributeAtIndex(fn_, i as u32 + 1, attr) };
            }
        }
        let call_conv = if func_decl.name.is_kernel() {
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
            for var in func_decl.return_arguments {
                method_emitter.emit_variable(var)?;
            }
            for statement in statements.iter() {
                if let Statement::Label(label) = statement {
                    method_emitter.emit_label_initial(*label);
                }
            }
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
                get_type(self.context, &var.v_type)?,
                name.as_ptr(),
                get_state_space(var.state_space)?,
            )
        };
        self.resolver.register(var.name, global);
        if let Some(align) = var.align {
            unsafe { LLVMSetAlignment(global, align) };
        }
        if !var.array_init.is_empty() {
            self.emit_array_init(&var.v_type, &*var.array_init, global)?;
        }
        Ok(())
    }

    // TODO: instead of Vec<u8> we should emit a typed initializer
    fn emit_array_init(
        &mut self,
        type_: &ast::Type,
        array_init: &[u8],
        global: *mut llvm_zluda::LLVMValue,
    ) -> Result<(), TranslateError> {
        match type_ {
            ast::Type::Array(None, scalar, dimensions) => {
                if dimensions.len() != 1 {
                    todo!()
                }
                if dimensions[0] as usize * scalar.size_of() as usize != array_init.len() {
                    return Err(error_unreachable());
                }
                let type_ = get_scalar_type(self.context, *scalar);
                let mut elements = array_init
                    .chunks(scalar.size_of() as usize)
                    .map(|chunk| self.constant_from_bytes(*scalar, chunk, type_))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|_| error_unreachable())?;
                let initializer =
                    unsafe { LLVMConstArray2(type_, elements.as_mut_ptr(), elements.len() as u64) };
                unsafe { LLVMSetInitializer(global, initializer) };
            }
            _ => todo!(),
        }
        Ok(())
    }

    fn constant_from_bytes(
        &self,
        scalar: ast::ScalarType,
        bytes: &[u8],
        llvm_type: LLVMTypeRef,
    ) -> Result<LLVMValueRef, TryFromSliceError> {
        Ok(match scalar {
            ptx_parser::ScalarType::Pred
            | ptx_parser::ScalarType::S8
            | ptx_parser::ScalarType::B8
            | ptx_parser::ScalarType::U8 => unsafe {
                LLVMConstInt(llvm_type, u8::from_le_bytes(bytes.try_into()?) as u64, 0)
            },
            ptx_parser::ScalarType::S16
            | ptx_parser::ScalarType::B16
            | ptx_parser::ScalarType::U16 => unsafe {
                LLVMConstInt(llvm_type, u16::from_le_bytes(bytes.try_into()?) as u64, 0)
            },
            ptx_parser::ScalarType::F16 => todo!(),
            ptx_parser::ScalarType::BF16 => todo!(),
            ptx_parser::ScalarType::S32 => todo!(),
            ptx_parser::ScalarType::U64 => todo!(),
            ptx_parser::ScalarType::S64 => todo!(),
            ptx_parser::ScalarType::S16x2 => todo!(),
            ptx_parser::ScalarType::B32 => todo!(),
            ptx_parser::ScalarType::F32 => todo!(),
            ptx_parser::ScalarType::B64 => todo!(),
            ptx_parser::ScalarType::F64 => todo!(),
            ptx_parser::ScalarType::B128 => todo!(),
            ptx_parser::ScalarType::U16x2 => todo!(),
            ptx_parser::ScalarType::F16x2 => todo!(),
            ptx_parser::ScalarType::U32 => todo!(),
            ptx_parser::ScalarType::BF16x2 => todo!(),
        })
    }
}

fn get_input_argument_type(
    context: LLVMContextRef,
    v_type: &ast::Type,
    state_space: ast::StateSpace,
) -> Result<LLVMTypeRef, TranslateError> {
    match state_space {
        ast::StateSpace::ParamEntry => {
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
        })
    }

    fn emit_variable(&mut self, var: ast::Variable<SpirvWord>) -> Result<(), TranslateError> {
        let alloca = unsafe {
            LLVMZludaBuildAlloca(
                self.variables_builder.get(),
                get_type(self.context, &var.v_type)?,
                get_state_space(var.state_space)?,
                self.resolver.get_or_add_raw(var.name),
            )
        };
        self.resolver.register(var.name, alloca);
        if let Some(align) = var.align {
            unsafe { LLVMSetAlignment(alloca, align) };
        }
        if !var.array_init.is_empty() {
            todo!()
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
            ast::Instruction::Mov { data, arguments } => self.emit_mov(data, arguments),
            ast::Instruction::Ld { data, arguments } => self.emit_ld(data, arguments),
            ast::Instruction::Add { data, arguments } => self.emit_add(data, arguments),
            ast::Instruction::St { data, arguments } => self.emit_st(data, arguments),
            ast::Instruction::Mul { data, arguments } => self.emit_mul(data, arguments),
            ast::Instruction::Setp { data, arguments } => self.emit_setp(data, arguments),
            ast::Instruction::SetpBool { .. } => todo!(),
            ast::Instruction::Not { data, arguments } => self.emit_not(data, arguments),
            ast::Instruction::Or { .. } => todo!(),
            ast::Instruction::And { arguments, .. } => self.emit_and(arguments),
            ast::Instruction::Bra { arguments } => self.emit_bra(arguments),
            ast::Instruction::Call { data, arguments } => self.emit_call(data, arguments),
            ast::Instruction::Cvt { .. } => todo!(),
            ast::Instruction::Shr { .. } => todo!(),
            ast::Instruction::Shl { .. } => todo!(),
            ast::Instruction::Ret { data } => Ok(self.emit_ret(data)),
            ast::Instruction::Cvta { data, arguments } => self.emit_cvta(data, arguments),
            ast::Instruction::Abs { .. } => todo!(),
            ast::Instruction::Mad { .. } => todo!(),
            ast::Instruction::Fma { .. } => todo!(),
            ast::Instruction::Sub { data, arguments } => self.emit_sub(data, arguments),
            ast::Instruction::Min { .. } => todo!(),
            ast::Instruction::Max { .. } => todo!(),
            ast::Instruction::Rcp { .. } => todo!(),
            ast::Instruction::Sqrt { .. } => todo!(),
            ast::Instruction::Rsqrt { .. } => todo!(),
            ast::Instruction::Selp { .. } => todo!(),
            ast::Instruction::Bar { .. } => todo!(),
            ast::Instruction::Atom { data, arguments } => self.emit_atom(data, arguments),
            ast::Instruction::AtomCas { data, arguments } => self.emit_atom_cas(data, arguments),
            ast::Instruction::Div { data, arguments } => self.emit_div(data, arguments),
            ast::Instruction::Neg { data, arguments } => self.emit_neg(data, arguments),
            ast::Instruction::Sin { data, arguments } => self.emit_sin(data, arguments),
            ast::Instruction::Cos { data, arguments } => self.emit_cos(data, arguments),
            ast::Instruction::Lg2 { .. } => todo!(),
            ast::Instruction::Ex2 { .. } => todo!(),
            ast::Instruction::Clz { data, arguments } => self.emit_clz(data, arguments),
            ast::Instruction::Brev { data, arguments } => self.emit_brev(data, arguments),
            ast::Instruction::Popc { .. } => todo!(),
            ast::Instruction::Xor { data, arguments } => self.emit_xor(data, arguments),
            ast::Instruction::Rem { .. } => todo!(),
            ast::Instruction::PrmtSlow { .. } => todo!(),
            ast::Instruction::Prmt { .. } => todo!(),
            ast::Instruction::Membar { .. } => todo!(),
            ast::Instruction::Trap {} => todo!(),
            // replaced by a function call
            ast::Instruction::Bfe { .. }
            | ast::Instruction::Bfi { .. }
            | ast::Instruction::Activemask { .. } => return Err(error_unreachable()),
        }
    }

    fn emit_ld(
        &mut self,
        data: ast::LdDetails,
        arguments: ast::LdArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        if data.qualifier != ast::LdStQualifier::Weak {
            todo!()
        }
        let builder = self.builder;
        let type_ = get_type(self.context, &data.typ)?;
        let ptr = self.resolver.value(arguments.src)?;
        self.resolver.with_result(arguments.dst, |dst| unsafe {
            LLVMBuildLoad2(builder, type_, ptr, dst)
        });
        Ok(())
    }

    fn emit_conversion(&mut self, conversion: ImplicitConversion) -> Result<(), TranslateError> {
        let builder = self.builder;
        match conversion.kind {
            ConversionKind::Default => {
                match (&conversion.from_type, &conversion.to_type) {
                    (ast::Type::Scalar(from_type), ast::Type::Scalar(to_type)) => {
                        let from_layout = conversion.from_type.layout();
                        let to_layout = conversion.to_type.layout();
                        if from_layout.size() == to_layout.size() {
                            let dst_type = get_type(self.context, &conversion.to_type)?;
                            if from_type.kind() != ast::ScalarKind::Float
                                && to_type.kind() != ast::ScalarKind::Float
                            {
                                // It is noop, but another instruction expects result of this conversion
                                self.resolver
                                    .register(conversion.dst, self.resolver.value(conversion.src)?);
                            } else {
                                let src = self.resolver.value(conversion.src)?;
                                self.resolver.with_result(conversion.dst, |dst| unsafe {
                                    LLVMBuildBitCast(builder, src, dst_type, dst)
                                });
                            }
                            Ok(())
                        } else {
                            let src = self.resolver.value(conversion.src)?;
                            // This block is safe because it's illegal to implictly convert between floating point values
                            let same_width_bit_type = unsafe {
                                LLVMIntTypeInContext(self.context, (from_layout.size() * 8) as u32)
                            };
                            let same_width_bit_value = unsafe {
                                LLVMBuildBitCast(
                                    builder,
                                    src,
                                    same_width_bit_type,
                                    LLVM_UNNAMED.as_ptr(),
                                )
                            };
                            let wide_bit_type = unsafe {
                                LLVMIntTypeInContext(self.context, (to_layout.size() * 8) as u32)
                            };
                            if to_type.kind() == ast::ScalarKind::Unsigned
                                || to_type.kind() == ast::ScalarKind::Bit
                            {
                                let llvm_fn = if to_type.size_of() >= from_type.size_of() {
                                    LLVMBuildZExtOrBitCast
                                } else {
                                    LLVMBuildTrunc
                                };
                                self.resolver.with_result(conversion.dst, |dst| unsafe {
                                    llvm_fn(builder, same_width_bit_value, wide_bit_type, dst)
                                });
                                Ok(())
                            } else {
                                let _conversion_fn = if from_type.kind() == ast::ScalarKind::Signed
                                    && to_type.kind() == ast::ScalarKind::Signed
                                {
                                    if to_type.size_of() >= from_type.size_of() {
                                        LLVMBuildSExtOrBitCast
                                    } else {
                                        LLVMBuildTrunc
                                    }
                                } else {
                                    if to_type.size_of() >= from_type.size_of() {
                                        LLVMBuildZExtOrBitCast
                                    } else {
                                        LLVMBuildTrunc
                                    }
                                };
                                todo!()
                            }
                        }
                    }
                    (ast::Type::Vector(..), ast::Type::Scalar(..))
                    | (ast::Type::Scalar(..), ast::Type::Array(..))
                    | (ast::Type::Array(..), ast::Type::Scalar(..)) => {
                        let src = self.resolver.value(conversion.src)?;
                        let dst_type = get_type(self.context, &conversion.to_type)?;
                        self.resolver.with_result(conversion.dst, |dst| unsafe {
                            LLVMBuildBitCast(builder, src, dst_type, dst)
                        });
                        Ok(())
                    }
                    _ => todo!(),
                }
            }
            ConversionKind::SignExtend => todo!(),
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

    fn emit_constant(&mut self, constant: ConstantDefinition) -> Result<(), TranslateError> {
        let type_ = get_scalar_type(self.context, constant.typ);
        let value = match constant.value {
            ast::ImmediateValue::U64(x) => unsafe { LLVMConstInt(type_, x, 0) },
            ast::ImmediateValue::S64(x) => unsafe { LLVMConstInt(type_, x as u64, 0) },
            ast::ImmediateValue::F32(x) => unsafe { LLVMConstReal(type_, x as f64) },
            ast::ImmediateValue::F64(x) => unsafe { LLVMConstReal(type_, x) },
        };
        self.resolver.register(constant.dst, value);
        Ok(())
    }

    fn emit_add(
        &mut self,
        data: ast::ArithDetails,
        arguments: ast::AddArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let builder = self.builder;
        let src1 = self.resolver.value(arguments.src1)?;
        let src2 = self.resolver.value(arguments.src2)?;
        let fn_ = match data {
            ast::ArithDetails::Integer(..) => LLVMBuildAdd,
            ast::ArithDetails::Float(..) => LLVMBuildFAdd,
        };
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
        let value = self.resolver.value(arguments.src2)?;
        if data.qualifier != ast::LdStQualifier::Weak {
            todo!()
        }
        unsafe { LLVMBuildStore(self.builder, value, ptr) };
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
            for (_, space) in data.input_arguments.iter() {
                if *space != ast::StateSpace::Reg {
                    panic!()
                }
            }
        }
        let name = match &*arguments.return_arguments {
            [] => LLVM_UNNAMED.as_ptr(),
            [dst] => self.resolver.get_or_add_raw(*dst),
            _ => todo!(),
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
        let llvm_fn = unsafe {
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
            [name] => {
                self.resolver.register(*name, llvm_fn);
            }
            _ => todo!(),
        }
        Ok(())
    }

    fn emit_mov(
        &mut self,
        _data: ast::MovDetails,
        arguments: ast::MovArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
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
        match &*values {
            [] => unsafe { LLVMBuildRetVoid(self.builder) },
            [(value, type_)] => {
                let value = self.resolver.value(*value)?;
                let type_ = get_type(self.context, type_)?;
                let value =
                    unsafe { LLVMBuildLoad2(self.builder, type_, value, LLVM_UNNAMED.as_ptr()) };
                unsafe { LLVMBuildRet(self.builder, value) }
            }
            _ => todo!(),
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
        let mul_fn = match data {
            ast::MulDetails::Integer { control, .. } => match control {
                ast::MulIntControl::Low => LLVMBuildMul,
                ast::MulIntControl::High => todo!(),
                ast::MulIntControl::Wide => todo!(),
            },
            ast::MulDetails::Float(..) => LLVMBuildFMul,
        };
        let src1 = self.resolver.value(arguments.src1)?;
        let src2 = self.resolver.value(arguments.src2)?;
        self.resolver.with_result(arguments.dst, |dst| unsafe {
            mul_fn(self.builder, src1, src2, dst)
        });
        Ok(())
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
            &ast::ScalarType::F32.into(),
            vec![(arguments.src, llvm_f32)],
        )?;
        unsafe { LLVMZludaSetFastMathFlags(cos, LLVMZludaFastMathApproxFunc) }
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
            todo!()
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
        arith_float: ptx_parser::ArithFloat,
        arguments: ptx_parser::SubArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        if arith_float.saturate {
            todo!()
        }
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
            &ast::ScalarType::F32.into(),
            vec![(arguments.src, llvm_f32)],
        )?;
        unsafe { LLVMZludaSetFastMathFlags(sin, LLVMZludaFastMathApproxFunc) }
        Ok(())
    }

    fn emit_intrinsic(
        &mut self,
        name: &CStr,
        dst: Option<SpirvWord>,
        return_type: &ast::Type,
        arguments: Vec<(SpirvWord, LLVMTypeRef)>,
    ) -> Result<LLVMValueRef, TranslateError> {
        let fn_type = get_function_type(
            self.context,
            iter::once(return_type),
            arguments.iter().map(|(_, type_)| Ok(*type_)),
        )?;
        let mut fn_ = unsafe { LLVMGetNamedFunction(self.module, name.as_ptr()) };
        if fn_ == ptr::null_mut() {
            fn_ = unsafe { LLVMAddFunction(self.module, name.as_ptr(), fn_type) };
        }
        let mut arguments = arguments
            .iter()
            .map(|(arg, _)| self.resolver.value(*arg))
            .collect::<Result<Vec<_>, _>>()?;
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
        let llvm_fn = if data.type_.kind() == ptx_parser::ScalarKind::Float {
            LLVMBuildFNeg
        } else {
            LLVMBuildNeg
        };
        self.resolver.with_result(arguments.dst, |dst| unsafe {
            llvm_fn(self.builder, src, dst)
        });
        Ok(())
    }

    fn emit_not(
        &mut self,
        _data: ptx_parser::ScalarType,
        arguments: ptx_parser::NotArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let src = self.resolver.value(arguments.src)?;
        self.resolver.with_result(arguments.dst, |dst| unsafe {
            LLVMBuildNot(self.builder, src, dst)
        });
        Ok(())
    }

    fn emit_setp(
        &mut self,
        data: ptx_parser::SetpData,
        arguments: ptx_parser::SetpArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        if arguments.dst2.is_some() {
            todo!()
        }
        match data.cmp_op {
            ptx_parser::SetpCompareOp::Integer(setp_compare_int) => {
                self.emit_setp_int(setp_compare_int, arguments)
            }
            ptx_parser::SetpCompareOp::Float(setp_compare_float) => {
                self.emit_setp_float(setp_compare_float, arguments)
            }
        }
    }

    fn emit_setp_int(
        &mut self,
        setp: ptx_parser::SetpCompareInt,
        arguments: ptx_parser::SetpArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
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
        let src1 = self.resolver.value(arguments.src1)?;
        let src2 = self.resolver.value(arguments.src2)?;
        self.resolver.with_result(arguments.dst1, |dst1| unsafe {
            LLVMBuildICmp(self.builder, op, src1, src2, dst1)
        });
        Ok(())
    }

    fn emit_setp_float(
        &mut self,
        setp: ptx_parser::SetpCompareFloat,
        arguments: ptx_parser::SetpArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
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
        let src1 = self.resolver.value(arguments.src1)?;
        let src2 = self.resolver.value(arguments.src2)?;
        self.resolver.with_result(arguments.dst1, |dst1| unsafe {
            LLVMBuildFCmp(self.builder, op, src1, src2, dst1)
        });
        Ok(())
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
        ast::MemScope::Cluster => todo!(),
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
        ast::Type::Pointer(_, space) => get_pointer_type(context, *space)?,
    })
}

fn get_scalar_type(context: LLVMContextRef, type_: ast::ScalarType) -> LLVMTypeRef {
    match type_ {
        ast::ScalarType::Pred => unsafe { LLVMInt1TypeInContext(context) },
        ast::ScalarType::S8 | ast::ScalarType::B8 | ast::ScalarType::U8 => unsafe {
            LLVMInt8TypeInContext(context)
        },
        ast::ScalarType::B16 | ast::ScalarType::U16 | ast::ScalarType::S16 => unsafe {
            LLVMInt16TypeInContext(context)
        },
        ast::ScalarType::S32 | ast::ScalarType::B32 | ast::ScalarType::U32 => unsafe {
            LLVMInt32TypeInContext(context)
        },
        ast::ScalarType::U64 | ast::ScalarType::S64 | ast::ScalarType::B64 => unsafe {
            LLVMInt64TypeInContext(context)
        },
        ast::ScalarType::B128 => unsafe { LLVMInt128TypeInContext(context) },
        ast::ScalarType::F16 => unsafe { LLVMHalfTypeInContext(context) },
        ast::ScalarType::F32 => unsafe { LLVMFloatTypeInContext(context) },
        ast::ScalarType::F64 => unsafe { LLVMDoubleTypeInContext(context) },
        ast::ScalarType::BF16 => unsafe { LLVMBFloatTypeInContext(context) },
        ast::ScalarType::U16x2 => todo!(),
        ast::ScalarType::S16x2 => todo!(),
        ast::ScalarType::F16x2 => todo!(),
        ast::ScalarType::BF16x2 => todo!(),
    }
}

fn get_function_type<'a>(
    context: LLVMContextRef,
    mut return_args: impl ExactSizeIterator<Item = &'a ast::Type>,
    input_args: impl ExactSizeIterator<Item = Result<LLVMTypeRef, TranslateError>>,
) -> Result<LLVMTypeRef, TranslateError> {
    let mut input_args = input_args.collect::<Result<Vec<_>, _>>()?;
    let return_type = match return_args.len() {
        0 => unsafe { LLVMVoidTypeInContext(context) },
        1 => get_type(context, return_args.next().unwrap())?,
        _ => todo!(),
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

fn get_state_space(space: ast::StateSpace) -> Result<u32, TranslateError> {
    match space {
        ast::StateSpace::Reg => Ok(PRIVATE_ADDRESS_SPACE),
        ast::StateSpace::Generic => Ok(GENERIC_ADDRESS_SPACE),
        ast::StateSpace::Param => Err(TranslateError::Todo),
        ast::StateSpace::ParamEntry => Ok(CONSTANT_ADDRESS_SPACE),
        ast::StateSpace::ParamFunc => Err(TranslateError::Todo),
        ast::StateSpace::Local => Ok(PRIVATE_ADDRESS_SPACE),
        ast::StateSpace::Global => Ok(GLOBAL_ADDRESS_SPACE),
        ast::StateSpace::Const => Ok(CONSTANT_ADDRESS_SPACE),
        ast::StateSpace::Shared => Ok(SHARED_ADDRESS_SPACE),
        ast::StateSpace::SharedCta => Err(TranslateError::Todo),
        ast::StateSpace::SharedCluster => Err(TranslateError::Todo),
    }
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

/*
struct ScalarTypeInLLVM(ast::ScalarType);

impl std::fmt::Display for ScalarTypeInLLVM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            ast::ScalarType::Pred => write!(f, "i1"),
            ast::ScalarType::B8 | ast::ScalarType::U8 | ast::ScalarType::S8 => write!(f, "i8"),
            ast::ScalarType::B16 | ast::ScalarType::U16 | ast::ScalarType::S16 => write!(f, "i16"),
            ast::ScalarType::B32 | ast::ScalarType::U32 | ast::ScalarType::S32 => write!(f, "i32"),
            ast::ScalarType::B64 | ast::ScalarType::U64 | ast::ScalarType::S64 => write!(f, "i64"),
            ptx_parser::ScalarType::B128 => write!(f, "i128"),
            ast::ScalarType::F16 => write!(f, "f16"),
            ptx_parser::ScalarType::BF16 => write!(f, "bfloat"),
            ast::ScalarType::F32 => write!(f, "f32"),
            ast::ScalarType::F64 => write!(f, "f64"),
            ptx_parser::ScalarType::S16x2 | ptx_parser::ScalarType::U16x2 => write!(f, "v2i16"),
            ast::ScalarType::F16x2 => write!(f, "v2f16"),
            ptx_parser::ScalarType::BF16x2 => write!(f, "v2bfloat"),
        }
    }
}

fn rounding_to_llvm(this: ast::RoundingMode) -> u32 {
    match this {
        ptx_parser::RoundingMode::Zero => 0,
        ptx_parser::RoundingMode::NearestEven => 1,
        ptx_parser::RoundingMode::PositiveInf => 2,
        ptx_parser::RoundingMode::NegativeInf => 3,
    }
}
*/
