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
use llvm_zluda::utils as llvm;
use llvm_zluda::utils::Context;
use llvm_zluda::{core::*, *};
use llvm_zluda::{prelude::*, LLVMZludaBuildAtomicRMW};
use llvm_zluda::{LLVMCallConv, LLVMZludaBuildAlloca, LLVMZludaBuildArrayAlloca};
use ptx_parser::{
    CpAsyncArgs, CpAsyncDetails, FunnelShiftMode, Mul24Control, ShfArgs, VariableInfo,
};

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
    fp_mode: FloatingPointMode,
    target: Target,
    strict_fp: bool,
    stack_size: u32,
    files: &[(u32, String)],
) -> Result<llvm::Module, TranslateError> {
    let module = llvm::Module::new(context, LLVM_UNNAMED);
    let mut emit_ctx =
        ModuleEmitContext::new(context, &module, &id_defs, fp_mode, target, strict_fp);
    emit_ctx.debug = DebugInfo::new(context.get(), module.get(), files);
    if stack_size != 0 {
        emit_ctx.stack_size = stack_size as u64;
    }
    emit_ctx.threadgroup_bytes_argument = target == Target::Agx && reads_threadgroup_bytes(&id_defs, &directives);
    for directive in directives {
        match directive {
            Directive2::Variable(linking, variable) => emit_ctx.emit_global(linking, variable)?,
            Directive2::Method(method) => emit_ctx.emit_method(method)?,
        }
    }
    emit_ctx.emit_masked_inits()?;
    if let Some(d) = emit_ctx.debug.take() {
        d.finish();
    }
    if target == Target::Agx {
        super::super::agx_matrix::define_helpers(context, &module)?;
        // the backend sizes the stack of recursive functions from it
        unsafe {
            let i32_type = LLVMInt32TypeInContext(context.get());
            let value = LLVMValueAsMetadata(LLVMConstInt(i32_type, emit_ctx.stack_size, 0));
            let key = "agx-stack-size";
            LLVMAddModuleFlag(module.get(), LLVMModuleFlagBehavior::LLVMModuleFlagBehaviorWarning, key.as_ptr().cast(), key.len(), value);
        }
    }
    if cfg!(debug_assertions) {
        if let Err(err) = module.verify() {
            panic!("{}\n{:?}", module.print_module_to_string(), err);
        }
    }
    Ok(module)
}

/// Whether the module reads the shared memory size of the launch (%dynamic_smem_size, %total_smem_size, ...): its
/// kernels then take it in a hidden argument (see `emit_method`)
fn reads_threadgroup_bytes(
    id_defs: &GlobalStringIdentResolver2,
    directives: &[Directive2<ast::Instruction<SpirvWord>, SpirvWord>],
) -> bool {
    directives.iter().any(|directive| match directive {
        Directive2::Method(Function { body: Some(body), .. }) => body.iter().any(|statement| match statement {
            Statement::Instruction(ast::Instruction::Call { arguments, .. }) => id_defs
                .ident_map
                .get(&arguments.func)
                .and_then(|entry| entry.name.as_deref())
                .is_some_and(|name| name.starts_with("__zluda_ptx_impl_sreg_") && name.contains("smem")),
            _ => false,
        }),
        _ => false,
    })
}

/// Line tables from `.file` and `.loc`: a compile unit, a subprogram per function with a body, and the location of
/// the last `.loc` on every instruction after it.
struct DebugInfo {
    context: LLVMContextRef,
    module: LLVMModuleRef,
    builder: LLVMDIBuilderRef,
    unit_file: LLVMMetadataRef,
    files: std::collections::HashMap<u32, LLVMMetadataRef>,
    subroutine: LLVMMetadataRef,
}

impl DebugInfo {
    fn new(context: LLVMContextRef, module: LLVMModuleRef, files: &[(u32, String)]) -> Option<DebugInfo> {
        use llvm_zluda::debuginfo::*;
        let (_, first) = files.first()?;
        let builder = unsafe { LLVMCreateDIBuilder(module) };
        let file = |name: &str| {
            let (dir, base) = match name.rsplit_once('/') {
                Some((d, b)) => (d, b),
                None => ("", name),
            };
            unsafe { LLVMDIBuilderCreateFile(builder, base.as_ptr().cast(), base.len(), dir.as_ptr().cast(), dir.len()) }
        };
        let files: std::collections::HashMap<u32, LLVMMetadataRef> = files.iter().map(|(i, f)| (*i, file(f))).collect();
        let unit_file = file(first);
        let producer = "ZLUDA";
        unsafe {
            LLVMDIBuilderCreateCompileUnit(
                builder,
                LLVMDWARFSourceLanguage::LLVMDWARFSourceLanguageC_plus_plus,
                unit_file,
                producer.as_ptr().cast(),
                producer.len(),
                1,
                ptr::null(),
                0,
                0,
                ptr::null(),
                0,
                LLVMDWARFEmissionKind::LLVMDWARFEmissionKindLineTablesOnly,
                0,
                0,
                0,
                ptr::null(),
                0,
                ptr::null(),
                0,
            )
        };
        let subroutine = unsafe { LLVMDIBuilderCreateSubroutineType(builder, unit_file, ptr::null_mut(), 0, 0) };
        Some(DebugInfo { context, module, builder, unit_file, files, subroutine })
    }

    /// Gives `function` a subprogram and returns it.
    fn subprogram(&self, function: LLVMValueRef, name: &str) -> LLVMMetadataRef {
        use llvm_zluda::debuginfo::*;
        unsafe {
            let sp = LLVMDIBuilderCreateFunction(
                self.builder,
                self.unit_file,
                name.as_ptr().cast(),
                name.len(),
                name.as_ptr().cast(),
                name.len(),
                self.unit_file,
                0,
                self.subroutine,
                0,
                1,
                0,
                0,
                1,
            );
            LLVMSetSubprogram(function, sp);
            sp
        }
    }

    /// The location of `.loc` in `subprogram`.
    fn location(&self, subprogram: LLVMMetadataRef, loc: ast::DebugLoc) -> LLVMMetadataRef {
        use llvm_zluda::debuginfo::*;
        let scope = match self.files.get(&loc.file) {
            Some(&f) if f != self.unit_file => unsafe { LLVMDIBuilderCreateLexicalBlockFile(self.builder, subprogram, f, 0) },
            _ => subprogram,
        };
        unsafe { LLVMDIBuilderCreateDebugLocation(self.context, loc.line, loc.column, scope, ptr::null_mut()) }
    }

    fn finish(self) {
        use llvm_zluda::debuginfo::*;
        unsafe {
            LLVMDIBuilderFinalize(self.builder);
            LLVMDisposeDIBuilder(self.builder);
            let version = LLVMValueAsMetadata(LLVMConstInt(LLVMInt32TypeInContext(self.context), LLVMDebugMetadataVersion() as u64, 0));
            let key = "Debug Info Version";
            LLVMAddModuleFlag(self.module, LLVMModuleFlagBehavior::LLVMModuleFlagBehaviorWarning, key.as_ptr().cast(), key.len(), version);
        }
    }
}

struct ModuleEmitContext<'a, 'input> {
    debug: Option<DebugInfo>,
    context: LLVMContextRef,
    module: LLVMModuleRef,
    builder: Builder,
    id_defs: &'a GlobalStringIdentResolver2<'input>,
    resolver: ResolveIdent,
    fp_mode: FloatingPointMode,
    target: Target,
    strict_fp: bool,
    stack_size: u64,
    // AGX: kernels take the shared memory size of the launch as a hidden last argument
    threadgroup_bytes_argument: bool,
    // AGX: the mask(name + offset) initializer elements, for __zluda_agx_masked_init: (the element's address, the
    // masked address, shift | element bytes << 8)
    masked_inits: Vec<[LLVMValueRef; 3]>,
}

impl<'a, 'input> ModuleEmitContext<'a, 'input> {
    fn new(
        context: &Context,
        module: &llvm::Module,
        id_defs: &'a GlobalStringIdentResolver2<'input>,
        fp_mode: FloatingPointMode,
        target: Target,
        strict_fp: bool,
    ) -> Self {
        ModuleEmitContext {
            debug: None,
            context: context.get(),
            module: module.get(),
            builder: Builder::new(context),
            id_defs,
            resolver: ResolveIdent::new(&id_defs),
            fp_mode,
            target,
            strict_fp,
            stack_size: DYNAMIC_STACK_SIZE,
            threadgroup_bytes_argument: false,
            masked_inits: Vec::new(),
        }
    }

    fn kernel_call_convention(&self) -> u32 {
        match self.target {
            Target::Amdgpu => LLVMCallConv::LLVMAMDGPUKERNELCallConv as u32,
            // CallingConv::AGX_KERNEL, which llvm-sys does not know.
            Target::Agx => 128,
        }
    }

    fn func_call_convention() -> u32 {
        LLVMCallConv::LLVMCCallConv as u32
    }

    fn emit_method(
        &mut self,
        method: Function<ast::Instruction<SpirvWord>, SpirvWord>,
    ) -> Result<(), TranslateError> {
        let name = method
            .import_as
            .as_deref()
            .or_else(|| self.id_defs.ident_map[&method.name].name.as_deref())
            .ok_or_else(|| error_unreachable())?;
        let name = CString::new(name).map_err(|_| error_unreachable())?;
        let mut fn_ = unsafe { LLVMGetNamedFunction(self.module, name.as_ptr()) };
        let threadgroup_bytes = method.is_kernel() && self.threadgroup_bytes_argument;
        if fn_ == ptr::null_mut() {
            let i32_type = unsafe { LLVMInt32TypeInContext(self.context) };
            let fn_type = get_function_type(
                self.context,
                method.return_arguments.iter().map(|v| &v.info.v_type),
                method
                    .input_arguments
                    .iter()
                    .map(|v| get_input_argument_type(self.context, &v.info.v_type, v.info.state_space))
                    .chain(threadgroup_bytes.then_some(Ok(i32_type))),
            )?;
            fn_ = unsafe { LLVMAddFunction(self.module, name.as_ptr(), fn_type) };
            self.emit_fn_attribute_string(fn_, "amdgpu-unsafe-fp-atomics", "true");
            self.emit_fn_attribute_string(fn_, "uniform-work-group-size", "true");
            self.emit_fn_attribute_string(fn_, "no-trapping-math", "true");
            if self.fp_mode == FloatingPointMode::Constrained {
                self.emit_fn_attribute_enum(fn_, "strictfp");
            }
        }
        // kernels too, whose address `mov.u64 %rd, kernel` takes
        self.resolver.register(method.name, fn_);
        self.emit_target_features(fn_);
        self.emit_tuning(fn_, &method.tuning);
        if let Some(kernel_attrs) = &method.kernel_attributes {
            self.emit_fn_attribute_string(
                fn_,
                "denormal-fp-math-f32",
                llvm_ftz(kernel_attrs.flush_to_zero_f32),
            );
            self.emit_fn_attribute_string(
                fn_,
                "denormal-fp-math",
                llvm_ftz(kernel_attrs.flush_to_zero_f16f64),
            );
        } else {
            self.emit_fn_attribute_string(fn_, "denormal-fp-math-f32", "dynamic");
            self.emit_fn_attribute_string(fn_, "denormal-fp-math", "dynamic");
        }
        self.emit_fn_attribute_string(fn_, "amdgpu-ieee", "false");
        for (i, param) in method.input_arguments.iter().enumerate() {
            let value = unsafe { LLVMGetParam(fn_, i as u32) };
            let name = self.resolver.get_or_add(param.name);
            // LLVM complains if alignment is set on function parameters for non-kernels
            if method.is_kernel() {
                if let Some(align) = param.info.align {
                    unsafe { LLVMSetParamAlignment(value, align) };
                }
            }
            unsafe { LLVMSetValueName2(value, name.as_ptr().cast(), name.len()) };
            self.resolver.register(param.name, value);
            if method.is_kernel() {
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
        let is_kernel = method.is_kernel();
        if !is_kernel {
            unsafe {
                LLVMSetVisibility(fn_, llvm_zluda::LLVMVisibility::LLVMHiddenVisibility);
            }
        }
        let call_conv = if is_kernel {
            self.kernel_call_convention()
        } else {
            Self::func_call_convention()
        };
        unsafe { LLVMSetFunctionCallConv(fn_, call_conv) };
        for directive in method.tuning {
            match directive {
                ptx_parser::TuningDirective::NoReturn => {
                    self.emit_fn_attribute_string(fn_, "noreturn", "true")
                }
                _ => {
                    // Ignore other tuning directives for now
                }
            };
        }
        if let Some(statements) = method.body {
            let variables_bb =
                unsafe { LLVMAppendBasicBlockInContext(self.context, fn_, LLVM_UNNAMED.as_ptr()) };
            let variables_builder = Builder::new_raw(self.context);
            unsafe { LLVMPositionBuilderAtEnd(variables_builder.get(), variables_bb) };
            let real_bb =
                unsafe { LLVMAppendBasicBlockInContext(self.context, fn_, LLVM_UNNAMED.as_ptr()) };
            unsafe { LLVMPositionBuilderAtEnd(self.builder.get(), real_bb) };
            if threadgroup_bytes && unsafe { LLVMCountParams(fn_) } as usize > method.input_arguments.len() {
                self.emit_threadgroup_bytes_argument(fn_, method.input_arguments.len() as u32);
            }
            let base_32bit_memory = method.kernel_meta32.as_ref().map(|m| m.implicit_memory_ptr);
            // instructions before the first .loc are at line 0
            let subprogram = self.debug.as_ref().map(|d| {
                let sp = d.subprogram(fn_, name.to_str().unwrap_or(""));
                let at = d.location(sp, ast::DebugLoc { file: u32::MAX, line: 0, column: 0 });
                unsafe { LLVMSetCurrentDebugLocation2(self.builder.get(), at) };
                sp
            });
            let mut method_emitter =
                MethodEmitContext::new(self, fn_, variables_builder, base_32bit_memory, is_kernel, subprogram);
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
            method_emitter.emit_kernel_prelude(method.kernel_attributes)?;
            for statement in statements {
                method_emitter.emit_statement(statement)?;
            }
            method_emitter.finish();
            unsafe { LLVMBuildBr(method_emitter.variables_builder.get(), real_bb) };
            if subprogram.is_some() {
                unsafe { LLVMSetCurrentDebugLocation2(self.builder.get(), ptr::null_mut()) };
            }
        }
        Ok(())
    }

    // The hidden argument of the shared memory size of the launch, static and dynamic bytes (the backend's
    // "agx-hidden"="threadgroup-bytes", filled by the runtime), stored at the kernel's start for the
    // %..._smem_... registers of zluda_ptx_impl_agx.ll to read, in the kernel and the functions it calls
    fn emit_threadgroup_bytes_argument(&mut self, fn_: LLVMValueRef, index: u32) {
        unsafe {
            let value = LLVMGetParam(fn_, index);
            let (key, role) = ("agx-hidden", "threadgroup-bytes");
            let attr = LLVMCreateStringAttribute(
                self.context,
                key.as_ptr().cast(),
                key.len() as u32,
                role.as_ptr().cast(),
                role.len() as u32,
            );
            LLVMAddAttributeAtIndex(fn_, index + 1, attr);
            let i32_type = LLVMInt32TypeInContext(self.context);
            let name = c"__zluda_agx_smem_bytes";
            let mut global = LLVMGetNamedGlobal(self.module, name.as_ptr());
            if global.is_null() {
                global = LLVMAddGlobalInAddressSpace(self.module, i32_type, name.as_ptr(), get_state_space(ast::StateSpace::Shared).unwrap_or(3));
                LLVMSetAlignment(global, 4);
            }
            LLVMBuildStore(self.builder.get(), value, global);
        }
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
        let llvm_type = get_type(self.context, &var.info.v_type)?;
        let global = unsafe {
            LLVMAddGlobalInAddressSpace(
                self.module,
                llvm_type,
                name.as_ptr(),
                get_state_space(var.info.state_space)?,
            )
        };
        if matches!(var.info.v_type, ast::Type::Texref) {
            unsafe { LLVMSetInitializer(global, LLVMGetUndef(llvm_type)) };
            unsafe {
                LLVMSetAlignment(global, 8);
            }
        }
        self.emit_linkage(global, &var.info)?;
        self.resolver.register(var.name, global);
        if let Some(align) = var.info.align {
            unsafe { LLVMSetAlignment(global, align) };
        }
        if !var.info.array_init.is_empty() {
            let initializer = self.get_array_init(&var.info.v_type, &*var.info.array_init)?;
            unsafe { LLVMSetInitializer(global, initializer) };
            self.record_masked_inits(global, &var.info)?;
        }
        Ok(())
    }

    // The loader stores the byte of each masked address (agx_runtime Module::relocated_globals): LLVM has no
    // constant expression for it
    fn record_masked_inits(&mut self, global: LLVMValueRef, info: &ast::VariableInfo<SpirvWord>) -> Result<(), TranslateError> {
        if !info.array_init.iter().any(|e| matches!(e, ast::RegOrImmediate::MaskedAddress(..))) {
            return Ok(());
        }
        let scalar = match &info.v_type {
            ast::Type::Scalar(s) | ast::Type::Array(None, s, _) => *s,
            _ => return Err(error_todo_msg("mask() in an initializer of vectors")),
        };
        if self.target != Target::Agx {
            return Err(error_todo_msg("mask() of an address in an initializer"));
        }
        // its initial value is not that of the initializer
        unsafe { LLVMSetExternallyInitialized(global, 1) };
        let bytes = scalar.size_of() as u64;
        let i64_type = unsafe { LLVMInt64TypeInContext(self.context) };
        for (i, elem) in info.array_init.iter().enumerate() {
            if let ast::RegOrImmediate::MaskedAddress(reg, offset, shift) = elem {
                let slot = self.address_init_of(global, i as i64 * bytes as i64, i64_type);
                let address = self.address_init(*reg, *offset, i64_type)?;
                let desc = unsafe { LLVMConstInt(i64_type, *shift as u64 | bytes << 8, 0) };
                self.masked_inits.push([slot, address, desc]);
            }
        }
        Ok(())
    }

    fn emit_masked_inits(&mut self) -> Result<(), TranslateError> {
        if self.masked_inits.is_empty() {
            return Ok(());
        }
        unsafe {
            let i64_type = LLVMInt64TypeInContext(self.context);
            let entry_type = LLVMArrayType2(i64_type, 3);
            let mut entries = self
                .masked_inits
                .iter_mut()
                .map(|e| LLVMConstArray2(i64_type, e.as_mut_ptr(), 3))
                .collect::<Vec<_>>();
            let table = LLVMConstArray2(entry_type, entries.as_mut_ptr(), entries.len() as u64);
            let global = LLVMAddGlobalInAddressSpace(
                self.module,
                LLVMTypeOf(table),
                c"__zluda_agx_masked_init".as_ptr(),
                get_state_space(ast::StateSpace::Global)?,
            );
            LLVMSetInitializer(global, table);
            LLVMSetAlignment(global, 8);
            let ptr_type = LLVMPointerTypeInContext(self.context, 0);
            let mut used = [LLVMConstAddrSpaceCast(global, ptr_type)];
            let used_init = LLVMConstArray2(ptr_type, used.as_mut_ptr(), 1);
            let used_global = LLVMAddGlobal(self.module, LLVMTypeOf(used_init), c"llvm.used".as_ptr());
            LLVMSetInitializer(used_global, used_init);
            LLVMSetLinkage(used_global, LLVMLinkage::LLVMAppendingLinkage);
            LLVMSetSection(used_global, c"llvm.metadata".as_ptr());
        }
        Ok(())
    }

    fn get_array_init(
        &self,
        type_: &ast::Type,
        array_init: &[ast::RegOrImmediate<SpirvWord>],
    ) -> Result<*mut LLVMValue, TranslateError> {
        let initializer = match type_ {
            ast::Type::Array(vector, scalar, dimensions) if dimensions.len() > 1 || vector.is_some() => {
                // the elements are in row-major order: one constant array (or vector) per row
                let rows = dimensions[0] as usize;
                if rows == 0 || array_init.len() % rows != 0 {
                    return Err(error_unreachable());
                }
                let row_type = if dimensions.len() > 1 {
                    ast::Type::Array(*vector, *scalar, dimensions[1..].to_vec())
                } else {
                    ast::Type::Vector(vector.unwrap().get(), *scalar)
                };
                let mut items = array_init
                    .chunks(array_init.len() / rows)
                    .map(|row| self.get_array_init(&row_type, row))
                    .collect::<Result<Vec<_>, _>>()?;
                let llvm_row = get_type(self.context, &row_type)?;
                unsafe { LLVMConstArray2(llvm_row, items.as_mut_ptr(), items.len() as u64) }
            }
            ast::Type::Array(None, scalar, dimensions) => {
                if dimensions.len() != 1 {
                    return Err(error_todo_msg("an initializer of an array without dimensions"));
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
                        ast::RegOrImmediate::RegOffset(reg, offset) => self.address_init(*reg, *offset, type_),
                        ast::RegOrImmediate::Imm(imm) => {
                            Ok(get_immediate_value(self.context, scalar, imm))
                        }
                        // stored by the loader (record_masked_inits)
                        ast::RegOrImmediate::MaskedAddress(..) => Ok(unsafe { LLVMConstInt(type_, 0, 0) }),
                        ast::RegOrImmediate::VecMember(..) => Err(error_mismatched_type()),
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                unsafe { LLVMConstArray2(type_, elements.as_mut_ptr(), elements.len() as u64) }
            }
            ast::Type::Vector(width, scalar) => {
                if *width as usize != array_init.len() {
                    return Err(error_unreachable());
                }
                let mut elements = array_init
                    .iter()
                    .map(|elem| self.get_array_init(&ast::Type::Scalar(*scalar), std::slice::from_ref(elem)))
                    .collect::<Result<Vec<_>, _>>()?;
                unsafe { LLVMConstVector(elements.as_mut_ptr(), elements.len() as u32) }
            }
            ast::Type::Scalar(scalar) => {
                let type_ = get_scalar_type(self.context, *scalar);
                match array_init {
                    [ast::RegOrImmediate::Imm(init)] => get_immediate_value(self.context, scalar, init),
                    // the address of a variable (or function), plus an offset
                    [ast::RegOrImmediate::Reg(reg)] => self.address_init(*reg, 0, type_)?,
                    [ast::RegOrImmediate::RegOffset(reg, offset)] => self.address_init(*reg, *offset, type_)?,
                    [ast::RegOrImmediate::MaskedAddress(..)] => unsafe { LLVMConstInt(type_, 0, 0) },
                    _ => return Err(error_mismatched_type()),
                }
            }
            _ => {
                todo!()
            }
        };
        Ok(initializer)
    }

    fn address_init(&self, reg: SpirvWord, offset: i64, type_: LLVMTypeRef) -> Result<LLVMValueRef, TranslateError> {
        Ok(self.address_init_of(self.resolver.value(reg)?, offset, type_))
    }

    fn address_init_of(&self, value: LLVMValueRef, offset: i64, type_: LLVMTypeRef) -> LLVMValueRef {
        let address = unsafe { LLVMConstPtrToInt(value, type_) };
        if offset == 0 {
            address
        } else {
            unsafe { LLVMConstAdd(address, LLVMConstInt(type_, offset as u64, 1)) }
        }
    }

    fn emit_fn_attribute_string(&self, llvm_object: LLVMValueRef, key: &str, value: &str) {
        // AMDGPU tuning attributes mean nothing to other backends.
        if self.target != Target::Amdgpu && key.starts_with("amdgpu-") {
            return;
        }
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

    fn emit_fn_attribute_enum(&self, llvm_object: LLVMValueRef, key: &str) {
        let attr_kind = unsafe { LLVMGetEnumAttributeKindForName(key.as_ptr().cast(), key.len()) };
        let attribute = unsafe { LLVMCreateEnumAttribute(self.context, attr_kind, 0) };
        unsafe { LLVMAddAttributeAtIndex(llvm_object, LLVMAttributeFunctionIndex, attribute) };
    }

    fn emit_tuning(&self, fn_: LLVMValueRef, tuning_directives: &[ast::TuningDirective]) {
        if self.target == Target::Agx {
            // The register budget follows the threads a core must hold: the threadgroup size, times the
            // threadgroups .minnctapersm asks for.
            let mut threads = None;
            let mut ctas = 1;
            let mut cluster = 1;
            for tuning in tuning_directives {
                match *tuning {
                    ptx_parser::TuningDirective::MaxNReg(n) => {
                        self.emit_fn_attribute_string(fn_, "agx-max-gprs", &n.to_string())
                    }
                    ptx_parser::TuningDirective::MaxNtid(x, y, z) | ptx_parser::TuningDirective::ReqNtid(x, y, z) => {
                        threads = Some(x * y * z)
                    }
                    ptx_parser::TuningDirective::MinNCtaPerSm(n) => ctas = n.max(1),
                    // A cluster runs as one threadgroup of its CTAs (see llvm_zluda's
                    // LLVMZludaAgxLowerClusters)
                    ptx_parser::TuningDirective::ReqNctaPerCluster(x, y, z) => {
                        cluster = x.max(1) * y.max(1) * z.max(1);
                        self.emit_fn_attribute_string(fn_, "agx-cluster-dims", &format!("{x},{y},{z}"));
                    }
                    ptx_parser::TuningDirective::NoReturn
                    | ptx_parser::TuningDirective::ExplicitCluster
                    | ptx_parser::TuningDirective::MaxClusterRank(_) => {}
                }
            }
            if let Some(t) = threads {
                let t = t.saturating_mul(ctas).saturating_mul(cluster).min(1024);
                self.emit_fn_attribute_string(fn_, "agx-max-threads-per-threadgroup", &t.to_string());
            }
            return;
        }
        for tuning in tuning_directives {
            match tuning {
                // We could implement it for the completeness sake, but it's
                // not particularly important. The functions with .noreturn in
                // PTX are noreturn in LLVM already
                ptx_parser::TuningDirective::NoReturn => {}
                // Not really applicable
                ptx_parser::TuningDirective::MaxNReg(_) => {}
                ptx_parser::TuningDirective::MaxNtid(x, y, z) => {
                    let size = x * y * z;
                    let value = format!("1,{size}");
                    self.emit_fn_attribute_string(fn_, "amdgpu-flat-work-group-size", &value);
                }
                ptx_parser::TuningDirective::ReqNtid(x, y, z) => {
                    let size = x * y * z;
                    let value = format!("{size},{size}");
                    self.emit_fn_attribute_string(fn_, "amdgpu-flat-work-group-size", &value);
                }
                ptx_parser::TuningDirective::MinNCtaPerSm(ctas) => {
                    let value = format!("{ctas},1024");
                    self.emit_fn_attribute_string(fn_, "amdgpu-waves-per-eu", &value);
                }
                // clusters of several CTAs are only implemented for AGX: their instructions are rejected here
                ptx_parser::TuningDirective::ReqNctaPerCluster(..)
                | ptx_parser::TuningDirective::ExplicitCluster
                | ptx_parser::TuningDirective::MaxClusterRank(_) => {}
            }
        }
    }

    fn emit_target_features(&mut self, fn_: LLVMValueRef) {
        // The AGX processor and its features are chosen when compiling.
        if self.target == Target::Agx {
            return;
        }
        let value = format!(
            "+wavefrontsize32,-wavefrontsize64,+cumode{}",
            if cfg!(debug_assertions) {
                ",+precise-memory"
            } else {
                ""
            }
        );
        self.emit_fn_attribute_string(fn_, "target-features", &*value)
    }

    fn emit_linkage(
        &self,
        global: LLVMValueRef,
        var_info: &VariableInfo<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let (linkage, visibility, extern_init) = match var_info.state_space {
            ast::StateSpace::Const | ast::StateSpace::Global => (
                LLVMLinkage::LLVMExternalLinkage,
                LLVMVisibility::LLVMDefaultVisibility,
                true,
            ),
            ast::StateSpace::Shared
            | ast::StateSpace::SharedCluster
            | ast::StateSpace::SharedCta
                if var_info.array_init.is_empty() =>
            {
                (
                    LLVMLinkage::LLVMExternalLinkage,
                    LLVMVisibility::LLVMDefaultVisibility,
                    false,
                )
            }
            ast::StateSpace::Shared
            | ast::StateSpace::SharedCluster
            | ast::StateSpace::SharedCta => (
                LLVMLinkage::LLVMPrivateLinkage,
                LLVMVisibility::LLVMHiddenVisibility,
                false,
            ),
            _ => return Err(error_unreachable()),
        };
        unsafe {
            LLVMSetLinkage(global, linkage);
            LLVMSetVisibility(global, visibility);
            if extern_init {
                LLVMSetExternallyInitialized(global, 1);
            }
        }
        Ok(())
    }
}

fn get_immediate_value(
    context: LLVMContextRef,
    scalar_type: &ast::ScalarType,
    imm: &ast::ImmediateValue,
) -> *mut LLVMValue {
    let type_ = get_scalar_type(context, *scalar_type);
    // packed types (.u16x2, .f16x2, ...): the bits of the immediate as an integer of the same size
    if unsafe { LLVMGetTypeKind(type_) } == LLVMTypeKind::LLVMVectorTypeKind {
        let bits = scalar_type.size_of() as u32 * 8;
        let int_type = unsafe { LLVMIntTypeInContext(context, bits) };
        let value = match imm {
            ast::ImmediateValue::U64(x) => *x,
            ast::ImmediateValue::S64(x) => *x as u64,
            ast::ImmediateValue::F32(x) => x.to_bits() as u64,
            ast::ImmediateValue::F64(x) => x.to_bits(),
        };
        let mask = if bits < 64 { (1u64 << bits) - 1 } else { u64::MAX };
        return unsafe { LLVMConstBitCast(LLVMConstInt(int_type, value & mask, 0), type_) };
    }
    // PTX immediates are 64-bit; LLVM requires them to fit the type. An integer used as a predicate is true when it
    // is not zero (PTX ISA 4.5.3).
    let truncate = |x: u64| {
        let width = unsafe { LLVMGetIntTypeWidth(type_) };
        if width == 1 {
            (x != 0) as u64
        } else if width < 64 {
            x & ((1u64 << width) - 1)
        } else {
            x
        }
    };
    match imm {
        ast::ImmediateValue::U64(x) => unsafe { LLVMConstInt(type_, truncate(*x), 0) },
        ast::ImmediateValue::S64(x) => unsafe { LLVMConstInt(type_, truncate(*x as u64), 0) },
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
        ast::StateSpace::Global | ast::StateSpace::Const if matches!(v_type, ast::Type::Texref) => {
            Ok(unsafe { LLVMPointerTypeInContext(context, get_state_space(state_space)?) })
        }
        // memory operands of the AGX matrix helpers (wmma.load, wmma.store)
        ast::StateSpace::Global | ast::StateSpace::Generic => {
            Ok(unsafe { LLVMPointerTypeInContext(context, get_state_space(state_space)?) })
        }
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
    carry_flag: Option<LLVMValueRef>,
    base_32bit_memory: Option<SpirvWord>,
    is_kernel: bool,
    fp: FloatingPoint,
    target: Target,
    strict_fp: bool,
    stack: Option<PtxStack>,
    dynamic_stack_size: u64,
    /// The function's subprogram and the module's debug info, if `.loc` is kept.
    debug: Option<(LLVMMetadataRef, &'a DebugInfo)>,
}

// The stack of alloca, stacksave and stackrestore: a private array (sized
// when the function is complete) and the offset of its top. As in PTX it grows
// down from the end of the array, an allocation's address is the new top, and
// restoring the offset frees the allocations made since it was saved, so later
// allocations reuse their addresses. Allocations of a dynamic size share
// the stack size of the options (DYNAMIC_STACK_SIZE bytes, CUDA's default,
// unless set); on AGX a thread whose
// allocation does not fit sets __agx_stack_overflow (see the backend's
// AGXUsage.rst, Recursion) and ends.
struct PtxStack {
    base: LLVMValueRef,
    top: LLVMValueRef,
    // the store of the initial top, the size of the array
    init: LLVMValueRef,
    size: u64,
    align: u32,
    dynamic: bool,
}

const DYNAMIC_STACK_SIZE: u64 = 1024;

impl<'a> MethodEmitContext<'a> {
    fn new(
        parent: &'a mut ModuleEmitContext,
        method: LLVMValueRef,
        variables_builder: Builder,
        base_32bit_memory: Option<SpirvWord>,
        is_kernel: bool,
        subprogram: Option<LLVMMetadataRef>,
    ) -> MethodEmitContext<'a> {
        let fp = FloatingPoint {
            context: parent.context,
            module: parent.module,
            builder: parent.builder.get(),
            mode: parent.fp_mode,
        };
        MethodEmitContext {
            context: parent.context,
            module: parent.module,
            builder: parent.builder.get(),
            variables_builder,
            resolver: &mut parent.resolver,
            method,
            carry_flag: None,
            base_32bit_memory,
            is_kernel,
            fp,
            target: parent.target,
            strict_fp: parent.strict_fp,
            stack: None,
            dynamic_stack_size: parent.stack_size,
            debug: subprogram.zip(parent.debug.as_ref()),
        }
    }

    fn finish(&mut self) {
        if let Some(stack) = self.stack.take() {
            let i32_type = get_scalar_type(self.context, ast::ScalarType::B32);
            let size = stack.size + if stack.dynamic { self.dynamic_stack_size } else { 0 };
            let size = size.next_multiple_of(stack.align as u64).max(stack.align as u64);
            unsafe {
                LLVMSetOperand(stack.base, 0, LLVMConstInt(i32_type, size, 0));
                LLVMSetAlignment(stack.base, stack.align);
                LLVMSetOperand(stack.init, 0, LLVMConstInt(i32_type, size, 0));
            }
        }
    }

    fn get_or_create_stack(&mut self) -> Result<(LLVMValueRef, LLVMValueRef), TranslateError> {
        if let Some(stack) = &self.stack {
            return Ok((stack.base, stack.top));
        }
        let i8_type = get_scalar_type(self.context, ast::ScalarType::B8);
        let i32_type = get_scalar_type(self.context, ast::ScalarType::B32);
        let space = get_state_space(ast::StateSpace::Local)?;
        let base = unsafe {
            LLVMZludaBuildArrayAlloca(
                self.variables_builder.get(),
                i8_type,
                space,
                LLVMConstInt(i32_type, 1, 0),
                16,
                LLVM_UNNAMED.as_ptr(),
            )
        };
        let top = unsafe {
            LLVMZludaBuildAlloca(self.variables_builder.get(), i32_type, space, LLVM_UNNAMED.as_ptr())
        };
        let init = unsafe { LLVMBuildStore(self.variables_builder.get(), LLVMConstInt(i32_type, 0, 0), top) };
        self.stack = Some(PtxStack {
            base,
            top,
            init,
            size: 0,
            align: 16,
            dynamic: false,
        });
        Ok((base, top))
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
            Statement::FunctionPointer(FunctionPointerDetails { dst, src }) => {
                let function = self.resolver.value(src)?;
                let i64_type = get_scalar_type(self.context, ast::ScalarType::B64);
                self.resolver.with_result(dst, |dst| unsafe {
                    LLVMBuildPtrToInt(self.builder, function, i64_type, dst)
                });
            }
            Statement::VectorRead(vector_read) => self.emit_vector_read(vector_read)?,
            Statement::VectorWrite(vector_write) => self.emit_vector_write(vector_write)?,
            Statement::SetMode(mode_reg) => self.emit_set_mode(mode_reg)?,
            Statement::FpSaturate { dst, src, type_ } => self.emit_fp_saturate(type_, dst, src)?,
            Statement::FpFlush { dst, src, type_ } => self.emit_fp_flush(type_, dst, src)?,
            Statement::FpModeRequired { .. } => {}
            Statement::Loc(loc) => {
                if let Some((sp, d)) = self.debug {
                    unsafe { LLVMSetCurrentDebugLocation2(self.builder, d.location(sp, loc)) };
                }
            }
        })
    }

    // This should be a kernel attribute, but sadly AMDGPU LLVM target does
    // not support attribute for it. So we have to set it as the first
    // instruction in the body of a kernel
    fn emit_kernel_prelude(
        &mut self,
        kernel_attrs: Option<KernelAttributes>,
    ) -> Result<(), TranslateError> {
        // TODO
        // On Windows, with 32 bit ZLUDA on kernel _Z29KernelDeformableUberCollisionPi in
        // SampleCloth.exe scalar memory accesses read stale memory. This might also affect
        // Linux. Until I have a repro that's the best I can do
        if cfg!(windows) && self.target == Target::Amdgpu {
            self.emit_intrinsic(c"llvm.amdgcn.s.dcache.inv", None, Vec::new(), Vec::new())?;
        }
        if let Some(KernelAttributes {
            rounding_mode_f32,
            rounding_mode_f16f64,
            ..
        }) = kernel_attrs
        {
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
        // On AGX registers (scalar variables) start as zero, so that reading
        // one that was never written is deterministic (stores that are
        // always overwritten are removed by the optimizer)
        if self.target == Target::Agx
            && matches!(
                var.info.state_space,
                ast::StateSpace::Reg | ast::StateSpace::Local
            )
            && matches!(var.info.v_type, ast::Type::Scalar(_) | ast::Type::Vector(..))
        {
            unsafe {
                LLVMBuildStore(
                    self.variables_builder.get(),
                    LLVMConstNull(get_type(self.context, &var.info.v_type)?),
                    alloca,
                )
            };
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
            ast::Instruction::AddExtended { data, arguments } => {
                self.emit_add_extended(data, arguments)
            }
            ast::Instruction::SubExtended { data, arguments } => {
                self.emit_sub_extended(data, arguments)
            }
            ast::Instruction::MadExtended { data, arguments } => {
                self.emit_mad_extended(data, arguments)
            }
            ast::Instruction::Dp4a { data, arguments } => self.emit_dp4a(data, arguments),
            ast::Instruction::St { data, arguments } => self.emit_st(data, arguments),
            ast::Instruction::Mul { data, arguments } => self.emit_mul(data, arguments),
            ast::Instruction::Mul24 { data, arguments } => self.emit_mul24(data, arguments),
            ast::Instruction::Set { data, arguments } => self.emit_set(data, arguments),
            ast::Instruction::SetBool { data, arguments } => self.emit_set_bool(data, arguments),
            ast::Instruction::Setp { data, arguments } => self.emit_setp(data, arguments),
            ast::Instruction::SetpBool { data, arguments } => self.emit_setp_bool(data, arguments),
            ast::Instruction::Not { data, arguments } => self.emit_not(data, arguments),
            ast::Instruction::Cnot { data, arguments } => self.emit_cnot(data, arguments),
            ast::Instruction::Or { data, arguments } => self.emit_or(data, arguments),
            ast::Instruction::And { arguments, .. } => self.emit_and(arguments),
            ast::Instruction::Bra { arguments } => self.emit_bra(arguments),
            ast::Instruction::Call { data, arguments } => self.emit_call(data, arguments),
            ast::Instruction::Cvt { data, arguments } => self.emit_cvt(data, arguments),
            ast::Instruction::CvtPack { data, arguments } => self.emit_cvt_pack(data, arguments),
            ast::Instruction::CvtTf32 { data, arguments } => self.emit_cvt_tf32(data, arguments),
            ast::Instruction::CvtMini { data, arguments } => {
                let dst_type = data.dst_type();
                let ast::Type::Scalar(dst_type) = dst_type else {
                    return Err(error_unreachable());
                };
                self.emit_cvt_mini(data, dst_type, arguments.dst, arguments.src1, arguments.src2)
            }
            ast::Instruction::CvtNarrow { data, arguments } => self.emit_cvt_narrow(data, arguments),
            ast::Instruction::Spcompress { data, arguments } => self.emit_spcompress(data, arguments),
            ast::Instruction::Tcgen05Sync {} => Ok(()),
            ast::Instruction::Tcgen05Cp { data: (shape, nb, group), arguments } => {
                let taddr = self.i32_value(Some(arguments.src_taddr))?;
                let desc = self.i64_value(arguments.src_desc)?;
                let shape = self.const_u32(shape as u64);
                let nb = self.const_u32(nb as u64);
                let pair = self.const_u32((group == 2) as u64);
                self.emit_agx_helper(c"__zluda_ptx_impl_agx_tcgen05_cp", None, None, &[taddr, desc, shape, nb, pair])?;
                Ok(())
            }
            ast::Instruction::Tcgen05Shift { data: group, arguments } => {
                let taddr = self.i32_value(Some(arguments.src_taddr))?;
                let pair = self.const_u32((group == 2) as u64);
                self.emit_agx_helper(c"__zluda_ptx_impl_agx_tcgen05_shift", None, None, &[taddr, pair])?;
                Ok(())
            }
            // the emulated operations are complete: an arrive (count 1) on the mbarrier right away
            ast::Instruction::Tcgen05Commit { data, arguments } => {
                let mbar = self.mbarrier_pointer(data, arguments.src_mbar)?;
                let one = self.const_u32(1);
                let zero = self.const_u32(0);
                self.emit_agx_helper(
                    c"__zluda_ptx_impl_agx_mbarrier_arrive",
                    None,
                    Some(ast::ScalarType::B64),
                    &[mbar, one, zero],
                )?;
                Ok(())
            }
            // replaced by a call to its agx_matrix helper
            ast::Instruction::Tcgen05Mma { .. } => Err(error_unreachable()),
            ast::Instruction::Tcgen05Alloc { data, arguments } => self.emit_tcgen05_alloc(data, arguments),
            ast::Instruction::Tcgen05Dealloc { data, arguments } => self.emit_tcgen05_dealloc(data, arguments),
            ast::Instruction::Tcgen05Ld { data, arguments } => self.emit_tcgen05_ld(data, arguments),
            ast::Instruction::Tcgen05LdRed { data, arguments } => self.emit_tcgen05_ld_red(data, arguments),
            ast::Instruction::Tcgen05St { data, arguments } => self.emit_tcgen05_st(data, arguments),
            ast::Instruction::Spdecompress { data, arguments } => self.emit_spdecompress(data, arguments),
            ast::Instruction::Isspacep { data, arguments } => self.emit_isspacep(data, arguments),
            ast::Instruction::Packed8 { data, arguments } => self.emit_packed8(data, arguments),
            ast::Instruction::Clmad { data, arguments } => self.emit_clmad(data, arguments),
            ast::Instruction::VideoScalar { data, arguments } => {
                self.emit_video_scalar(data, arguments)
            }
            ast::Instruction::VideoSimd { data, arguments } => self.emit_video_simd(data, arguments),
            ast::Instruction::Vmad { data, arguments } => self.emit_vmad(data, arguments),
            ast::Instruction::Alloca { data, arguments } => self.emit_alloca(data, arguments),
            ast::Instruction::StackSave { data, arguments } => self.emit_stacksave(data, arguments),
            ast::Instruction::StackRestore { data, arguments } => {
                self.emit_stackrestore(data, arguments)
            }
            ast::Instruction::Shf { data, arguments } => self.emit_shf(data, arguments),
            ast::Instruction::Shr { data, arguments } => self.emit_shr(data, arguments),
            ast::Instruction::Shl { data, arguments } => self.emit_shl(data, arguments),
            ast::Instruction::Ret { data } => Ok(self.emit_ret(data)),
            ast::Instruction::Exit {} => self.emit_exit(),
            ast::Instruction::Cvta { data, arguments } => self.emit_cvta(data, arguments),
            ast::Instruction::Abs { data, arguments } => self.emit_abs(data, arguments),
            ast::Instruction::Mad { data, arguments } => self.emit_mad(data, arguments),
            ast::Instruction::Fma { data, arguments } => self.emit_fma(data, arguments),
            ast::Instruction::FmaOob { data, arguments } => self.emit_fma_oob(data, arguments),
            ast::Instruction::Sub { data, arguments } => self.emit_sub(data, arguments),
            ast::Instruction::Min { data, arguments } => self.emit_min(data, arguments),
            ast::Instruction::Max { data, arguments } => self.emit_max(data, arguments),
            ast::Instruction::Rcp { data, arguments } => self.emit_rcp(data, arguments),
            ast::Instruction::Sqrt { data, arguments } => self.emit_sqrt(data, arguments),
            ast::Instruction::Rsqrt { data, arguments } => self.emit_rsqrt(data, arguments),
            ast::Instruction::Selp { data, arguments } => self.emit_selp(data, arguments),
            ast::Instruction::Atom { data, arguments } => self.emit_atom(data, arguments),
            ast::Instruction::Red { data, arguments } => {
                self.emit_atom_impl(data, arguments.src1, arguments.src2)?;
                Ok(())
            }
            ast::Instruction::AtomCas { data, arguments } => self.emit_atom_cas(data, arguments),
            ast::Instruction::Div { data, arguments } => self.emit_div(data, arguments),
            ast::Instruction::Neg { data, arguments } => self.emit_neg(data, arguments),
            ast::Instruction::Sin { data, arguments } => self.emit_sin(data, arguments),
            ast::Instruction::Cos { data, arguments } => self.emit_cos(data, arguments),
            ast::Instruction::Lg2 { data, arguments } => self.emit_lg2(data, arguments),
            ast::Instruction::Ex2 { data, arguments } => self.emit_ex2(data, arguments),
            ast::Instruction::Clz { data, arguments } => self.emit_clz(data, arguments),
            ast::Instruction::Bfind { data, arguments } => self.emit_bfind(data, arguments),
            ast::Instruction::Lop3 { data, arguments } => self.emit_lop3(data, arguments),
            ast::Instruction::Mad24 { data, arguments } => self.emit_mad24(data, arguments),
            ast::Instruction::Slct { data, arguments } => self.emit_slct(data, arguments),
            // expanded by insert_post_saturation
            ast::Instruction::MixedFloat { .. } => Err(error_unreachable()),
            ast::Instruction::AltFpX4 { data, arguments } => self.emit_alt_fp_x4(data, arguments),
            ast::Instruction::Multimem { data, arguments } => self.emit_multimem(data, arguments),
            ast::Instruction::Fabric { data, arguments } => self.emit_fabric(data, arguments),
            ast::Instruction::CvtRs { data, arguments } => self.emit_cvt_rs(data, arguments),
            ast::Instruction::CvtPzo { data, arguments } => {
                let kind = self.const_u32(data.kind as u64);
                let flags = self.const_u32(data.flags as u64);
                let a = self.i32_value(Some(arguments.src_a))?;
                let b = self.i32_value(arguments.src_b)?;
                let x = self.emit_agx_helper(c"__zluda_ptx_impl_agx_cvt_pzo", None, Some(ast::ScalarType::B32), &[kind, flags, a, b])?;
                let x = if data.kind <= 1 {
                    unsafe { LLVMBuildTrunc(self.builder, x, LLVMInt16TypeInContext(self.context), LLVM_UNNAMED.as_ptr()) }
                } else {
                    x
                };
                self.resolver.register(arguments.dst, x);
                Ok(())
            }
            ast::Instruction::StAsync { data, arguments } => self.emit_st_async(data, arguments),
            ast::Instruction::StBulk { arguments, .. } => {
                let to = self.resolver.value(arguments.src_addr)?;
                let size = self.resolver.value(arguments.src_size)?;
                let init = self.resolver.value(arguments.src_init)?;
                let init = unsafe {
                    LLVMBuildTrunc(self.builder, init, LLVMInt8TypeInContext(self.context), LLVM_UNNAMED.as_ptr())
                };
                unsafe { LLVMBuildMemSet(self.builder, to, init, size, 8) };
                Ok(())
            }
            // a register budget hint: registers are allocated per kernel
            ast::Instruction::SetMaxNReg { .. } => Ok(()),
            // the emulated wgmma completes when it is executed: fences and waits have nothing to order
            ast::Instruction::WgmmaFence { .. } => Ok(()),
            ast::Instruction::ClusterLaunchTryCancel { data, arguments } => {
                self.emit_cluster_launch_try_cancel(data, arguments)
            }
            ast::Instruction::ClusterLaunchQuery { data, arguments } => {
                // no cluster is ever canceled
                let dst_type = get_type(self.context, &data.dst_type())?;
                self.resolver.register(arguments.dst, unsafe { LLVMConstNull(dst_type) });
                Ok(())
            }
            // calls (replace_instructions_with_functions)
            ast::Instruction::Mapa { .. }
            | ast::Instruction::Getctarank { .. }
            | ast::Instruction::BarrierCluster { .. } => Err(error_unreachable()),
            ast::Instruction::Testp { data, arguments } => self.emit_testp(data, arguments),
            ast::Instruction::MatchAllSync { data, arguments } => self.emit_match_all_sync(data, arguments),
            ast::Instruction::ElectSync { arguments } => self.emit_elect_sync(arguments),
            ast::Instruction::Bmsk {
                data: ast::BmskMode::Wrap,
                arguments,
            } => self.emit_bmsk_wrap(arguments),
            ast::Instruction::Brev { data, arguments } => self.emit_brev(data, arguments),
            ast::Instruction::Popc { data, arguments } => self.emit_popc(data, arguments),
            ast::Instruction::Xor { data, arguments } => self.emit_xor(data, arguments),
            ast::Instruction::Rem { data, arguments } => self.emit_rem(data, arguments),
            ast::Instruction::BarWarp { .. } => self.emit_bar_warp(),
            ast::Instruction::Membar { data } => self.emit_membar(data),
            ast::Instruction::Trap {} => self.emit_trap(),
            ast::Instruction::CpAsync { data, arguments } => self.emit_cp_async(data, arguments),
            ast::Instruction::CpAsyncMbarrierArrive { data, arguments } => {
                self.emit_cp_async_mbarrier_arrive(data, arguments)
            }
            ast::Instruction::CpAsyncBulkTensor { data, arguments } => {
                self.emit_cp_async_bulk_tensor(data, arguments)
            }
            ast::Instruction::TexAgx { data, arguments } => self.emit_tex_agx(data, arguments),
            ast::Instruction::TexQuery { data, arguments } => self.emit_tex_query(data, arguments),
            ast::Instruction::SuldAgx { data, arguments } => self.emit_suld(data, arguments),
            ast::Instruction::SustAgx { data, arguments } => self.emit_sust(data, arguments),
            ast::Instruction::SuredAgx { data, arguments } => self.emit_sured(data, arguments),
            ast::Instruction::Istypep { data, arguments } => self.emit_istypep(data, arguments),
            ast::Instruction::TensormapReplace { data, arguments } => {
                self.emit_tensormap_replace(data, arguments)
            }
            ast::Instruction::CpAsyncBulk { data, arguments } => {
                self.emit_cp_async_bulk(data, arguments)
            }
            ast::Instruction::CpReduceAsyncBulk { data, arguments } => {
                self.emit_cp_reduce_async_bulk(data, arguments)
            }
            ast::Instruction::MbarrierInit { data, arguments } => {
                self.emit_mbarrier_init(data, arguments)
            }
            ast::Instruction::MbarrierInval { data, arguments } => {
                self.emit_mbarrier_inval(data, arguments)
            }
            ast::Instruction::MbarrierArrive { data, arguments } => {
                self.emit_mbarrier_arrive(data, arguments)
            }
            ast::Instruction::MbarrierTx { data, arguments } => {
                self.emit_mbarrier_tx(data, arguments)
            }
            ast::Instruction::MbarrierWait { data, arguments } => {
                self.emit_mbarrier_wait(data, arguments)
            }
            ast::Instruction::MbarrierPendingCount { arguments } => {
                self.emit_mbarrier_pending_count(arguments)
            }
            ast::Instruction::Copysign { data, arguments } => self.emit_copysign(data, arguments),
            ast::Instruction::CreatePolicyFractional { data, arguments } => {
                self.emit_createpolicy_fractional(data, arguments)
            }
            ast::Instruction::Sad { data, arguments } => self.emit_sad(data, arguments),
            ast::Instruction::Vshr { data, arguments } => self.emit_vshr(data, arguments),
            ast::Instruction::CpAsyncCommitGroup {} => Ok(()), // nop
            ast::Instruction::CpAsyncWaitGroup { .. } => Ok(()), // nop
            ast::Instruction::CpAsyncWaitAll { .. } => Ok(()), // nop
            ast::Instruction::GridDepControl { .. } => Ok(()), // nop
            ast::Instruction::Prefetch { .. } => Ok(()),       // nop
            // replaced by a function call
            ast::Instruction::Bfe { .. }
            | ast::Instruction::Bar { .. }
            | ast::Instruction::BarRed { .. }
            | ast::Instruction::Bfi { .. }
            | ast::Instruction::Bmsk {
                data: ast::BmskMode::Clamp,
                ..
            }
            | ast::Instruction::Activemask { .. }
            | ast::Instruction::Shfl { .. }
            | ast::Instruction::MatchSync { .. }
            | ast::Instruction::Vote { .. }
            | ast::Instruction::Nanosleep { .. }
            | ast::Instruction::ReduxSync { .. }
            | ast::Instruction::LdMatrix { .. }
            | ast::Instruction::StMatrix { .. }
            | ast::Instruction::MmaSparse { .. }
            | ast::Instruction::MmaBlockScale { .. }
            | ast::Instruction::MovMatrix { .. }
            | ast::Instruction::WmmaLoad { .. }
            | ast::Instruction::WmmaStore { .. }
            | ast::Instruction::WmmaMma { .. }
            | ast::Instruction::Prmt { .. }
        | ast::Instruction::Szext { .. }
        | ast::Instruction::Fns { .. }
            | ast::Instruction::Mma { .. }
            | ast::Instruction::Wgmma { .. }
            | ast::Instruction::Dp2a { .. }
            | ast::Instruction::Tanh { .. }
            | ast::Instruction::Tex { .. } => return Err(error_unreachable()),
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
            unsafe {
                LLVMIntTypeInContext(
                    self.context,
                    data.typ.layout().ok_or_else(error_unreachable)?.size() as u32 * 8,
                )
            }
        } else {
            underlying_type
        };
        let src = self.resolver.value(arguments.src)?;
        let load = unsafe { LLVMBuildLoad2(builder, op_type, src, LLVM_UNNAMED.as_ptr()) };
        apply_qualifier(load, data.qualifier)?;
        unsafe {
            LLVMSetAlignment(
                load,
                data.typ.layout().ok_or_else(error_unreachable)?.align() as u32,
            )
        };
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
                let ptr_type = get_pointer_type(self.context, conversion.to_space)?;
                if let (
                    Some(base32),
                    ast::StateSpace::Global | ast::StateSpace::Generic | ast::StateSpace::Const,
                ) = (self.base_32bit_memory, conversion.to_space)
                {
                    let i8_type = get_scalar_type(self.context, ast::ScalarType::B8);
                    let i64_type = get_scalar_type(self.context, ast::ScalarType::B64);
                    let src_64 =
                        unsafe { LLVMBuildZExt(builder, src, i64_type, LLVM_UNNAMED.as_ptr()) };
                    let base32 = self.resolver.value(base32)?;
                    let base32_typed = unsafe {
                        LLVMBuildIntToPtr(builder, base32, ptr_type, LLVM_UNNAMED.as_ptr())
                    };
                    self.resolver.with_result(conversion.dst, |dst| unsafe {
                        LLVMBuildInBoundsGEP2(
                            builder,
                            i8_type,
                            base32_typed,
                            [src_64].as_mut_ptr(),
                            1,
                            dst,
                        )
                    });
                } else {
                    self.resolver.with_result(conversion.dst, |dst| unsafe {
                        LLVMBuildIntToPtr(builder, src, ptr_type, dst)
                    });
                };
                Ok(())
            }
            ConversionKind::PtrToPtr => {
                if conversion.from_space == conversion.to_space {
                    // With opaque pointers it's meaningless
                    return self.emit_mov(ast::MovArgs {
                        dst: conversion.dst,
                        src: conversion.src,
                    });
                }
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
                if let (
                    Some(_),
                    ast::StateSpace::Global | ast::StateSpace::Generic | ast::StateSpace::Const,
                ) = (self.base_32bit_memory, conversion.to_space)
                {
                    // Should never happen because globals are removed
                    return Err(error_unreachable());
                } else {
                    self.resolver.with_result(conversion.dst, |dst| unsafe {
                        LLVMBuildPtrToInt(builder, src, dst_type, dst)
                    });
                }
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
                let to_layout = to_type.layout().ok_or_else(error_unreachable)?;
                if from_layout.size() == to_layout.size() {
                    let dst_type = get_type(self.context, &to_type)?;
                    // integers of one width are the same LLVM type, but the packed integer types (u16x2, s16x2)
                    // are vectors and need a real bitcast
                    if from_type.kind() != ast::ScalarKind::Float
                        && to_type_scalar.kind() != ast::ScalarKind::Float
                        && unsafe { LLVMTypeOf(src) } == dst_type
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
        let src1 = self.resolver.value(arguments.src1)?;
        let src2 = self.resolver.value(arguments.src2)?;
        match data {
            ast::ArithDetails::Integer(ast::ArithInteger {
                saturate: true,
                type_,
            }) => {
                let op = if type_.kind() == ast::ScalarKind::Signed {
                    "sadd"
                } else {
                    "uadd"
                };
                return self.emit_intrinsic_saturate(op, type_, arguments.dst, src1, src2);
            }
            ast::ArithDetails::Integer(ast::ArithInteger {
                saturate: false, ..
            }) => {
                self.resolver.with_result(arguments.dst, |dst| unsafe {
                    LLVMBuildAdd(builder, src1, src2, dst)
                });
            }
            ast::ArithDetails::Float(arith) => {
                if self.is_half_directed_rounding(&arith) {
                    return self.emit_half_directed_rounding(
                        "llvm.experimental.constrained.fadd",
                        arith,
                        arguments.dst,
                        vec![src1, src2],
                    );
                }
                self.fp
                    .fadd(self.resolver, data.type_(), arguments.dst, src1, src2)?;
            }
        };
        Ok(())
    }

    fn emit_binop_with_overflow(
        &mut self,
        op: &str,
        type_: ptx_parser::ScalarType,
        lhs: LLVMValueRef,
        rhs: LLVMValueRef,
    ) -> Result<(LLVMValueRef, LLVMValueRef), TranslateError> {
        let builder = self.builder;
        let llvm_type = get_scalar_type(self.context, type_);
        let intrinsic = format!("llvm.{}.with.overflow.{}\0", op, LLVMTypeDisplay(type_));
        let combined = self.emit_intrinsic(
            unsafe { CStr::from_bytes_with_nul_unchecked(intrinsic.as_bytes()) },
            None,
            vec![&type_.into(), &ast::ScalarType::Pred.into()],
            vec![(lhs, llvm_type), (rhs, llvm_type)],
        )?;

        Ok((
            unsafe { LLVMBuildExtractValue(builder, combined, 0, LLVM_UNNAMED.as_ptr()) },
            unsafe { LLVMBuildExtractValue(builder, combined, 1, LLVM_UNNAMED.as_ptr()) },
        ))
    }

    fn get_or_emit_carry_flag(&mut self) -> Result<LLVMValueRef, TranslateError> {
        if let Some(carry_flag) = self.carry_flag {
            return Ok(carry_flag);
        }

        let carry_flag = unsafe {
            LLVMZludaBuildAlloca(
                self.variables_builder.get(),
                LLVMIntTypeInContext(self.context, 1),
                get_state_space(ast::StateSpace::Reg)?,
                c"__ZLUDA_REG_CC_CF".as_ptr(),
            )
        };
        self.carry_flag = Some(carry_flag);
        Ok(carry_flag)
    }

    fn emit_load_carry_flag(
        &mut self,
        reverse_carry: bool,
        type_: ast::ScalarType,
    ) -> Result<LLVMValueRef, TranslateError> {
        let carry_flag = self.get_or_emit_carry_flag()?;
        let builder = self.builder;
        let pred_type = unsafe { LLVMIntTypeInContext(self.context, 1) };
        let mut load =
            unsafe { LLVMBuildLoad2(builder, pred_type, carry_flag, LLVM_UNNAMED.as_ptr()) };
        if reverse_carry {
            load = self.emit_not_impl(pred_type, load);
        }
        let zext = unsafe {
            LLVMBuildZExt(
                builder,
                load,
                get_scalar_type(self.context, type_),
                LLVM_UNNAMED.as_ptr(),
            )
        };
        Ok(zext)
    }

    fn emit_store_carry_flag(
        &mut self,
        reverse_carry: bool,
        mut value: LLVMValueRef,
    ) -> Result<(), TranslateError> {
        let carry_flag = self.get_or_emit_carry_flag()?;
        if reverse_carry {
            value = self.emit_not_impl(unsafe { LLVMIntTypeInContext(self.context, 1) }, value);
        }
        unsafe {
            LLVMBuildStore(self.builder, value, carry_flag);
        };
        Ok(())
    }

    fn emit_extended_arithmetic(
        &mut self,
        reverse_carry: bool,
        op: LLVMOpcode,
        op_name: &str,
        data: ast::CarryDetails,
        dst: SpirvWord,
        lhs: LLVMValueRef,
        rhs: LLVMValueRef,
    ) -> Result<(), TranslateError> {
        let sum = match data.kind {
            ast::CarryKind::CarryIn => {
                // sum = lhs + rhs + carry
                let sum =
                    unsafe { LLVMBuildBinOp(self.builder, op, lhs, rhs, LLVM_UNNAMED.as_ptr()) };
                unsafe {
                    LLVMBuildBinOp(
                        self.builder,
                        op,
                        sum,
                        self.emit_load_carry_flag(reverse_carry, data.type_)?,
                        LLVM_UNNAMED.as_ptr(),
                    )
                }
            }
            ast::CarryKind::CarryOut => {
                // { sum, overflow } = lhs + rhs
                let (sum, overflow) =
                    self.emit_binop_with_overflow(&op_name, data.type_, lhs, rhs)?;
                self.emit_store_carry_flag(reverse_carry, overflow)?;
                sum
            }
            ast::CarryKind::CarryInCarryOut => {
                // { final_sum, final_overflow } = lhs + rhs + carry
                let carry_in = self.emit_load_carry_flag(reverse_carry, data.type_)?;

                let (sum, overflow1) =
                    self.emit_binop_with_overflow(&op_name, data.type_, lhs, rhs)?;
                let (final_sum, overflow2) =
                    self.emit_binop_with_overflow(&op_name, data.type_, sum, carry_in)?;
                let final_overflow = unsafe {
                    LLVMBuildOr(self.builder, overflow1, overflow2, LLVM_UNNAMED.as_ptr())
                };
                self.emit_store_carry_flag(reverse_carry, final_overflow)?;
                final_sum
            }
        };

        self.resolver.register(dst, sum);
        Ok(())
    }

    fn emit_add_extended(
        &mut self,
        data: ast::CarryDetails,
        arguments: ast::AddExtendedArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        self.emit_extended_arithmetic(
            false,
            LLVMOpcode::LLVMAdd,
            "uadd",
            data,
            arguments.dst,
            self.resolver.value(arguments.src1)?,
            self.resolver.value(arguments.src2)?,
        )
    }

    fn emit_sub_extended(
        &mut self,
        data: ast::CarryDetails,
        arguments: ast::SubExtendedArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        self.emit_extended_arithmetic(
            true,
            LLVMOpcode::LLVMSub,
            "usub",
            data,
            arguments.dst,
            self.resolver.value(arguments.src1)?,
            self.resolver.value(arguments.src2)?,
        )
    }

    fn emit_mad_extended(
        &mut self,
        data: ast::MadCarryDetails,
        arguments: ast::MadExtendedArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let mul_control = ast::MulDetails::Integer {
            control: data.control,
            type_: data.type_,
        };
        let mul_result = self.emit_mul_impl(mul_control, None, arguments.src1, arguments.src2)?;

        let src3 = self.resolver.value(arguments.src3)?;

        self.emit_extended_arithmetic(
            false,
            LLVMOpcode::LLVMAdd,
            "uadd",
            ast::CarryDetails {
                kind: data.kind,
                type_: data.type_,
            },
            arguments.dst,
            mul_result,
            src3,
        )
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
                    LLVMIntTypeInContext(
                        self.context,
                        data.typ.layout().ok_or_else(error_unreachable)?.size() as u32 * 8,
                    ),
                    LLVM_UNNAMED.as_ptr(),
                )
            };
        }
        let store = unsafe { LLVMBuildStore(self.builder, value, ptr) };
        apply_qualifier(store, data.qualifier)?;
        unsafe {
            LLVMSetAlignment(
                store,
                data.typ.layout().ok_or_else(error_unreachable)?.align() as u32,
            );
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
        let mut callee = self.resolver.value(arguments.func)?;
        // An indirect call, through a function address in a register
        if unsafe { LLVMIsAFunction(callee) }.is_null() {
            let generic = get_pointer_type(self.context, ast::StateSpace::Generic)?;
            callee =
                unsafe { LLVMBuildIntToPtr(self.builder, callee, generic, LLVM_UNNAMED.as_ptr()) };
        }
        let llvm_call = unsafe {
            LLVMBuildCall2(
                self.builder,
                type_,
                callee,
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
        let result = self.emit_atom_impl(data, arguments.src1, arguments.src2)?;
        self.resolver.register(arguments.dst, result);
        Ok(())
    }

    fn emit_atom_impl(
        &mut self,
        data: ast::AtomDetails,
        src1: SpirvWord,
        src2: SpirvWord,
    ) -> Result<LLVMValueRef, TranslateError> {
        let builder = self.builder;
        let src1 = self.resolver.value(src1)?;
        let src2 = self.resolver.value(src2)?;
        // f32 add keeping subnormals: on shared memory, and with .noftz (PTX ISA 9.4 9.7.15.5)
        if let (ast::AtomicOp::FloatAdd, true) =
            (data.op, data.noftz || data.space == ast::StateSpace::Shared)
        {
            let element = match &data.type_ {
                ast::Type::Scalar(t) => Some((*t, None)),
                ast::Type::Vector(n, t) => Some((*t, Some(*n))),
                _ => None,
            };
            if let (Some((ast::ScalarType::F32, count)), Target::Agx) = (element, self.target) {
                return self.emit_atom_fadd_ieee(data.space, src1, src2, count);
            }
        }
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
        Ok(unsafe {
            LLVMZludaBuildAtomicRMW(
                builder,
                op,
                src1,
                src2,
                get_scope(data.scope)?,
                get_ordering(data.semantics),
            )
        })
    }

    // An f32 atomic add whose sum keeps subnormals (a compare-and-swap loop
    // around an IEEE addition), element by element for a vector
    fn emit_atom_fadd_ieee(
        &mut self,
        space: ast::StateSpace,
        address: LLVMValueRef,
        value: LLVMValueRef,
        count: Option<u8>,
    ) -> Result<LLVMValueRef, TranslateError> {
        let name = match space {
            ast::StateSpace::Shared => c"__zluda_ptx_impl_agx_atom_fadd_ieee_shared",
            ast::StateSpace::Global => c"__zluda_ptx_impl_agx_atom_fadd_ieee_global",
            _ => c"__zluda_ptx_impl_agx_atom_fadd_ieee_generic",
        };
        let Some(count) = count else {
            return self.emit_agx_helper(name, None, Some(ast::ScalarType::F32), &[address, value]);
        };
        let i8_type = unsafe { LLVMInt8TypeInContext(self.context) };
        let mut result = unsafe { LLVMGetUndef(LLVMTypeOf(value)) };
        for i in 0..count as u64 {
            let index = self.const_u32(i);
            let offset = self.const_u32(4 * i);
            let element = unsafe { LLVMBuildExtractElement(self.builder, value, index, LLVM_UNNAMED.as_ptr()) };
            let mut indices = [offset];
            let pointer = unsafe {
                LLVMBuildInBoundsGEP2(self.builder, i8_type, address, indices.as_mut_ptr(), 1, LLVM_UNNAMED.as_ptr())
            };
            let old = self.emit_agx_helper(name, None, Some(ast::ScalarType::F32), &[pointer, element])?;
            result = unsafe { LLVMBuildInsertElement(self.builder, result, old, index, LLVM_UNNAMED.as_ptr()) };
        }
        Ok(result)
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
        let type_ = get_scalar_type(self.context, data);
        let src = self.resolver.value(arguments.src)?;
        self.emit_intrinsic(
            llvm_fn,
            Some(arguments.dst),
            vec![&data.into()],
            vec![(src, type_)],
        )?;
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
                    LLVMSetAlignment(
                        load,
                        type_.layout().ok_or_else(error_unreachable)?.align() as u32,
                    );
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
        let clz_result = self.emit_clz_impl(data, arguments.src)?;
        self.resolver.register(arguments.dst, clz_result);
        Ok(())
    }

    fn emit_clz_impl(
        &mut self,
        type_: ptx_parser::ScalarType,
        src: SpirvWord,
    ) -> Result<LLVMValueRef, TranslateError> {
        let llvm_fn = match type_.size_of() {
            4 => c"llvm.ctlz.i32",
            8 => c"llvm.ctlz.i64",
            _ => return Err(error_unreachable()),
        };
        let llvm_type = get_scalar_type(self.context, type_);
        let pred = get_scalar_type(self.context, ast::ScalarType::Pred);
        let src = self.resolver.value(src)?;
        let false_ = unsafe { LLVMConstInt(pred, 0, 0) };
        let result = self.emit_intrinsic(
            llvm_fn,
            None,
            vec![&type_.into()],
            vec![(src, llvm_type), (false_, pred)],
        )?;
        let i32_llvm = get_scalar_type(self.context, ast::ScalarType::B32);
        Ok(unsafe {
            LLVMBuildTruncOrBitCast(self.builder, result, i32_llvm, LLVM_UNNAMED.as_ptr())
        })
    }

    fn emit_mul(
        &mut self,
        data: ast::MulDetails,
        arguments: ast::MulArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        if let ast::MulDetails::Float(arith) = data {
            if self.is_half_directed_rounding(&arith) {
                let src1 = self.resolver.value(arguments.src1)?;
                let src2 = self.resolver.value(arguments.src2)?;
                return self.emit_half_directed_rounding(
                    "llvm.experimental.constrained.fmul",
                    arith,
                    arguments.dst,
                    vec![src1, src2],
                );
            }
        }
        self.emit_mul_impl(data, Some(arguments.dst), arguments.src1, arguments.src2)?;
        Ok(())
    }

    // AGX rounds 16-bit float arithmetic to nearest only. With a directed
    // rounding mode the operation is computed in f32 (exactly for mul and
    // fma, correctly rounded in the same direction otherwise) and then
    // rounded to the 16-bit type, which gives the same result as rounding
    // once. Both rounding modes are set by instruction_mode_to_global_mode.
    fn is_half_directed_rounding(&self, arith: &ast::ArithFloat) -> bool {
        self.target == Target::Agx
            && self.fp.mode == FloatingPointMode::Constrained
            && arith.rounding != ast::RoundingMode::NearestEven
            && matches!(
                arith.type_,
                ast::ScalarType::F16
                    | ast::ScalarType::F16x2
                    | ast::ScalarType::BF16
                    | ast::ScalarType::BF16x2
            )
    }

    fn emit_half_directed_rounding(
        &mut self,
        intrinsic: &'static str,
        arith: ast::ArithFloat,
        dst: SpirvWord,
        srcs: Vec<LLVMValueRef>,
    ) -> Result<(), TranslateError> {
        let element = arith.type_.packed_type().unwrap_or(arith.type_);
        let lanes = if element == arith.type_ { 1 } else { 2 };
        let ftz = arith.flush_to_zero == Some(true) && element == ast::ScalarType::F16;
        let i32_type = get_scalar_type(self.context, ast::ScalarType::B32);
        let mut result = unsafe { LLVMGetPoison(get_scalar_type(self.context, arith.type_)) };
        for lane in 0..lanes {
            let index = unsafe { LLVMConstInt(i32_type, lane, 0) };
            let mut wide = Vec::with_capacity(srcs.len());
            for &src in srcs.iter() {
                let mut value = if lanes == 1 {
                    src
                } else {
                    unsafe { LLVMBuildExtractElement(self.builder, src, index, LLVM_UNNAMED.as_ptr()) }
                };
                if ftz {
                    value = self.emit_flush_f16(value);
                }
                wide.push(self.fp.fpext(
                    self.resolver,
                    ast::ScalarType::F32,
                    None,
                    element,
                    value,
                )?);
            }
            let wide_result = match wide.len() {
                2 => self.fp.emit_constrained(
                    self.resolver,
                    intrinsic,
                    ast::ScalarType::F32,
                    None,
                    [wide[0], wide[1]],
                    ["round.dynamic", "fpexcept.ignore"],
                )?,
                3 => self.fp.emit_constrained(
                    self.resolver,
                    intrinsic,
                    ast::ScalarType::F32,
                    None,
                    [wide[0], wide[1], wide[2]],
                    ["round.dynamic", "fpexcept.ignore"],
                )?,
                _ => return Err(error_unreachable()),
            };
            let mut narrow = if element == ast::ScalarType::BF16 {
                self.emit_f32_to_bf16_directed(arith.rounding, wide_result)
            } else {
                self.fp
                    .fptrunc(self.resolver, element, None, ast::ScalarType::F32, wide_result)?
            };
            if ftz {
                narrow = self.emit_flush_f16(narrow);
            }
            result = if lanes == 1 {
                narrow
            } else {
                unsafe {
                    LLVMBuildInsertElement(
                        self.builder,
                        result,
                        narrow,
                        index,
                        LLVM_UNNAMED.as_ptr(),
                    )
                }
            };
        }
        self.resolver.register(dst, result);
        Ok(())
    }

    // The AGX backend rounds conversions to bf16 to nearest only. Directed
    // rounding converts to f32 under the same rounding mode first, which
    // instruction_mode_to_global_mode sets for both f32 and f16/f64
    fn is_bf16_directed_rounding(&self, to: ast::ScalarType, rounding: ast::RoundingMode) -> bool {
        self.target == Target::Agx
            && self.fp.mode == FloatingPointMode::Constrained
            && rounding != ast::RoundingMode::NearestEven
            && to == ast::ScalarType::BF16
    }

    // Rounds an f32 to bf16 towards zero or an infinity on the bit pattern:
    // bf16 is the upper half of f32. NaNs become the canonical NaN
    fn emit_f32_to_bf16_directed(
        &mut self,
        rounding: ast::RoundingMode,
        value: LLVMValueRef,
    ) -> LLVMValueRef {
        let i32_type = get_scalar_type(self.context, ast::ScalarType::B32);
        let i16_type = get_scalar_type(self.context, ast::ScalarType::B16);
        let bf16_type = get_scalar_type(self.context, ast::ScalarType::BF16);
        let b = self.builder;
        unsafe {
            let bits = LLVMBuildBitCast(b, value, i32_type, LLVM_UNNAMED.as_ptr());
            let truncated = LLVMBuildLShr(b, bits, LLVMConstInt(i32_type, 16, 0), LLVM_UNNAMED.as_ptr());
            let low = LLVMBuildAnd(b, bits, LLVMConstInt(i32_type, 0xffff, 0), LLVM_UNNAMED.as_ptr());
            let inexact = LLVMBuildICmp(
                b,
                LLVMIntPredicate::LLVMIntNE,
                low,
                LLVMConstInt(i32_type, 0, 0),
                LLVM_UNNAMED.as_ptr(),
            );
            let negative = LLVMBuildICmp(
                b,
                LLVMIntPredicate::LLVMIntSLT,
                bits,
                LLVMConstInt(i32_type, 0, 0),
                LLVM_UNNAMED.as_ptr(),
            );
            // Rounding away from zero increments the magnitude
            let away = match rounding {
                ast::RoundingMode::NegativeInf => Some(negative),
                ast::RoundingMode::PositiveInf => {
                    Some(LLVMBuildNot(b, negative, LLVM_UNNAMED.as_ptr()))
                }
                _ => None,
            };
            let rounded = match away {
                Some(away) => {
                    let up = LLVMBuildAnd(b, away, inexact, LLVM_UNNAMED.as_ptr());
                    let up = LLVMBuildZExt(b, up, i32_type, LLVM_UNNAMED.as_ptr());
                    LLVMBuildAdd(b, truncated, up, LLVM_UNNAMED.as_ptr())
                }
                None => truncated,
            };
            let is_nan = LLVMBuildFCmp(
                b,
                LLVMRealPredicate::LLVMRealUNO,
                value,
                value,
                LLVM_UNNAMED.as_ptr(),
            );
            let rounded = LLVMBuildSelect(
                b,
                is_nan,
                LLVMConstInt(i32_type, 0x7fff, 0),
                rounded,
                LLVM_UNNAMED.as_ptr(),
            );
            let rounded = LLVMBuildTrunc(b, rounded, i16_type, LLVM_UNNAMED.as_ptr());
            LLVMBuildBitCast(b, rounded, bf16_type, LLVM_UNNAMED.as_ptr())
        }
    }

    // Replaces an f32 subnormal with a zero of the same sign
    fn emit_flush_f32(&mut self, value: LLVMValueRef) -> LLVMValueRef {
        let i32_type = get_scalar_type(self.context, ast::ScalarType::B32);
        let f32_type = get_scalar_type(self.context, ast::ScalarType::F32);
        let b = self.builder;
        let c = |v: u64| unsafe { LLVMConstInt(i32_type, v, 0) };
        unsafe {
            let bits = LLVMBuildBitCast(b, value, i32_type, LLVM_UNNAMED.as_ptr());
            let exponent = LLVMBuildAnd(b, bits, c(0x7f80_0000), LLVM_UNNAMED.as_ptr());
            let is_subnormal = LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntEQ, exponent, c(0), LLVM_UNNAMED.as_ptr());
            let sign = LLVMBuildAnd(b, bits, c(0x8000_0000), LLVM_UNNAMED.as_ptr());
            let bits = LLVMBuildSelect(b, is_subnormal, sign, bits, LLVM_UNNAMED.as_ptr());
            LLVMBuildBitCast(b, bits, f32_type, LLVM_UNNAMED.as_ptr())
        }
    }

    // Replaces an f16 subnormal with a zero of the same sign
    fn emit_flush_f16(&mut self, value: LLVMValueRef) -> LLVMValueRef {
        let i16_type = get_scalar_type(self.context, ast::ScalarType::B16);
        let f16_type = get_scalar_type(self.context, ast::ScalarType::F16);
        unsafe {
            let bits = LLVMBuildBitCast(self.builder, value, i16_type, LLVM_UNNAMED.as_ptr());
            let exponent = LLVMBuildAnd(
                self.builder,
                bits,
                LLVMConstInt(i16_type, 0x7c00, 0),
                LLVM_UNNAMED.as_ptr(),
            );
            let is_subnormal = LLVMBuildICmp(
                self.builder,
                LLVMIntPredicate::LLVMIntEQ,
                exponent,
                LLVMConstInt(i16_type, 0, 0),
                LLVM_UNNAMED.as_ptr(),
            );
            let sign = LLVMBuildAnd(
                self.builder,
                bits,
                LLVMConstInt(i16_type, 0x8000, 0),
                LLVM_UNNAMED.as_ptr(),
            );
            let bits = LLVMBuildSelect(self.builder, is_subnormal, sign, bits, LLVM_UNNAMED.as_ptr());
            LLVMBuildBitCast(self.builder, bits, f16_type, LLVM_UNNAMED.as_ptr())
        }
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
            ast::MulDetails::Float(..) => {
                return self.fp.fmul(self.resolver, data.type_(), dst, src1, src2)
            }
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
        let src = self.resolver.value(arguments.src)?;
        let cos = self.fp.cos_f32(self.resolver, arguments.dst, src)?;
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

    fn emit_vector_read(&mut self, vector_read: VectorRead) -> Result<(), TranslateError> {
        let src = self.resolver.value(vector_read.vector_src)?;
        let index = unsafe {
            LLVMConstInt(
                get_scalar_type(self.context, ast::ScalarType::B8),
                vector_read.member as _,
                0,
            )
        };
        self.resolver
            .with_result(vector_read.scalar_dst, |dst| unsafe {
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
                let Some(dst) = dst else { continue };
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
                let scalar_src = self
                    .resolver
                    .value(src_id.ok_or_else(error_mismatched_type)?)?;
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
        if self.target == Target::Agx {
            let signed = matches!(data, ptx_parser::DivDetails::Signed(_));
            return self.emit_defined_int_div(arguments.dst, signed, false, src1, src2);
        }
        self.resolver.with_result(arguments.dst, |dst| unsafe {
            integer_div(self.builder, src1, src2, dst)
        });
        Ok(())
    }

    /// Integer division and remainder with the results NVIDIA GPUs produce
    /// where PTX leaves them unspecified (and LLVM's instructions are
    /// undefined): division by zero gives all ones, the remainder of a
    /// division by zero is the dividend, and signed division of the minimum
    /// value by -1 wraps. The divisor is replaced by 1 in those cases.
    fn emit_defined_int_div(
        &mut self,
        dst: SpirvWord,
        signed: bool,
        rem: bool,
        src1: LLVMValueRef,
        src2: LLVMValueRef,
    ) -> Result<(), TranslateError> {
        unsafe {
            let ty = LLVMTypeOf(src2);
            let zero = LLVMConstNull(ty);
            let one = LLVMConstInt(ty, 1, 0);
            let by_zero = LLVMBuildICmp(self.builder, LLVMIntPredicate::LLVMIntEQ, src2, zero, LLVM_UNNAMED.as_ptr());
            let mut unsafe_divisor = by_zero;
            if signed {
                let width = LLVMGetIntTypeWidth(ty);
                let min = LLVMConstInt(ty, 1u64 << (width - 1), 0);
                let is_min = LLVMBuildICmp(self.builder, LLVMIntPredicate::LLVMIntEQ, src1, min, LLVM_UNNAMED.as_ptr());
                let is_minus_one = LLVMBuildICmp(self.builder, LLVMIntPredicate::LLVMIntEQ, src2, LLVMConstAllOnes(ty), LLVM_UNNAMED.as_ptr());
                let overflow = LLVMBuildAnd(self.builder, is_min, is_minus_one, LLVM_UNNAMED.as_ptr());
                unsafe_divisor = LLVMBuildOr(self.builder, by_zero, overflow, LLVM_UNNAMED.as_ptr());
            }
            let divisor = LLVMBuildSelect(self.builder, unsafe_divisor, one, src2, LLVM_UNNAMED.as_ptr());
            let result = match (signed, rem) {
                (false, false) => LLVMBuildUDiv(self.builder, src1, divisor, LLVM_UNNAMED.as_ptr()),
                (true, false) => LLVMBuildSDiv(self.builder, src1, divisor, LLVM_UNNAMED.as_ptr()),
                (false, true) => LLVMBuildURem(self.builder, src1, divisor, LLVM_UNNAMED.as_ptr()),
                (true, true) => LLVMBuildSRem(self.builder, src1, divisor, LLVM_UNNAMED.as_ptr()),
            };
            let by_zero_result = if rem { src1 } else { LLVMConstAllOnes(ty) };
            self.resolver.with_result(dst, |dst| {
                LLVMBuildSelect(self.builder, by_zero, by_zero_result, result, dst)
            });
        }
        Ok(())
    }

    fn emit_div_float(
        &mut self,
        float_div: ptx_parser::DivFloatDetails,
        arguments: ptx_parser::DivArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let src1 = self.resolver.value(arguments.src1)?;
        let src2 = self.resolver.value(arguments.src2)?;
        if self.target == Target::Agx
            && matches!(float_div.kind, ptx_parser::DivFloatKind::Rounding(_))
        {
            self.fp
                .fdiv(self.resolver, float_div.type_, arguments.dst, src1, src2)?;
            return Ok(());
        }
        if matches!(
            float_div.kind,
            ptx_parser::DivFloatKind::ApproxFull | ptx_parser::DivFloatKind::Rounding(_)
        ) {
            return Err(error_unreachable());
        }
        if self.target == Target::Agx && float_div.type_ == ast::ScalarType::F32 {
            // d = a * (1/b), where 1/b of 2^126 < |b| is a zero of the sign of b (PTX ISA 9.7.3.8): infinite a
            // gives NaN, any other a a zero. 1/b is the hardware's reciprocal (as rcp.approx), which LLVM does not
            // fold: a constant b (from launch-time specialization) gives the same result as a variable one.
            let f32_type = get_scalar_type(self.context, ast::ScalarType::F32);
            let b = self.builder;
            unsafe {
                let recip = self.emit_intrinsic(c"llvm.agx.rcp.f32", None, vec![&ast::Type::Scalar(ast::ScalarType::F32)], vec![(src2, f32_type)])?;
                let magnitude = self.emit_intrinsic(c"llvm.fabs.f32", None, vec![&ast::Type::Scalar(ast::ScalarType::F32)], vec![(src2, f32_type)])?;
                let big = LLVMBuildFCmp(b, LLVMRealPredicate::LLVMRealOGT, magnitude, LLVMConstReal(f32_type, 2f64.powi(126)), LLVM_UNNAMED.as_ptr());
                let zero = self.emit_intrinsic(
                    c"llvm.copysign.f32",
                    None,
                    vec![&ast::Type::Scalar(ast::ScalarType::F32)],
                    vec![(LLVMConstReal(f32_type, 0.0), f32_type), (src2, f32_type)],
                )?;
                let recip = LLVMBuildSelect(b, big, zero, recip, LLVM_UNNAMED.as_ptr());
                self.resolver.with_result(arguments.dst, |dst| LLVMBuildFMul(b, src1, recip, dst));
            }
            return Ok(());
        }
        let fdiv = self
            .fp
            .fdiv(self.resolver, float_div.type_, arguments.dst, src1, src2)?;
        unsafe {
            LLVMZludaSetFastMathFlags(
                fdiv,
                LLVMZludaFastMathAllowReciprocal | LLVMZludaFastMathApproxFunc,
            )
        };
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
        if self.base_32bit_memory.is_some()
            && matches!(
                data.state_space,
                ast::StateSpace::Global | ast::StateSpace::Generic | ast::StateSpace::Const
            )
        {
            return self.emit_mov(ast::MovArgs {
                dst: arguments.dst,
                src: arguments.src,
            });
        }
        let from_type = get_pointer_type(self.context, from_space)?;
        let dest_type = get_pointer_type(self.context, to_space)?;
        let src = self.resolver.value(arguments.src)?;
        let temp_ptr =
            unsafe { LLVMBuildIntToPtr(self.builder, src, from_type, LLVM_UNNAMED.as_ptr()) };
        let temp_converted = unsafe {
            LLVMBuildAddrSpaceCast(self.builder, temp_ptr, dest_type, LLVM_UNNAMED.as_ptr())
        };
        let scalar_type = unsafe { LLVMIntTypeInContext(self.context, data.size as u32) };
        self.resolver.with_result(arguments.dst, |dst| unsafe {
            LLVMBuildPtrToInt(self.builder, temp_converted, scalar_type, dst)
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
        let src1 = self.resolver.value(arguments.src1)?;
        let src2 = self.resolver.value(arguments.src2)?;
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
                src1,
                src2,
            );
        }
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
        let src1 = self.resolver.value(arguments.src1)?;
        let src2 = self.resolver.value(arguments.src2)?;
        if self.is_half_directed_rounding(&arith_float) {
            return self.emit_half_directed_rounding(
                "llvm.experimental.constrained.fsub",
                arith_float,
                arguments.dst,
                vec![src1, src2],
            );
        }
        self.fp
            .fsub(self.resolver, arith_float.type_, arguments.dst, src1, src2)?;
        Ok(())
    }

    fn emit_sin(
        &mut self,
        _data: ptx_parser::FlushToZero,
        arguments: ptx_parser::SinArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let src = self.resolver.value(arguments.src)?;
        let sin = self.fp.sin_f32(self.resolver, arguments.dst, src)?;
        unsafe { LLVMZludaSetFastMathFlags(sin, LLVMZludaFastMathApproxFunc) }
        Ok(())
    }

    fn emit_intrinsic_with_metadata(
        &mut self,
        name: &CStr,
        dst: Option<SpirvWord>,
        return_types: Vec<&ast::Type>,
        mut arguments: Vec<(LLVMValueRef, LLVMTypeRef)>,
        metadata: &[&str],
    ) -> Result<LLVMValueRef, TranslateError> {
        let has_metadata_args = !metadata.is_empty();
        let metadata_type = unsafe { LLVMMetadataTypeInContext(self.context) };
        let metadata = metadata.iter().map(|m| {
            let m_value =
                unsafe { LLVMMDStringInContext2(self.context, m.as_ptr().cast(), m.len()) };
            (
                unsafe { LLVMMetadataAsValue(self.context, m_value) },
                metadata_type,
            )
        });
        if has_metadata_args {
            arguments = arguments.into_iter().chain(metadata).collect::<Vec<_>>();
        }
        let fn_type = get_intrinsic_type(
            self.context,
            return_types.iter().copied(),
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

    fn emit_intrinsic(
        &mut self,
        name: &CStr,
        dst: Option<SpirvWord>,
        return_types: Vec<&ast::Type>,
        arguments: Vec<(LLVMValueRef, LLVMTypeRef)>,
    ) -> Result<LLVMValueRef, TranslateError> {
        self.emit_intrinsic_with_metadata(name, dst, return_types, arguments, &[])
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
        if is_floating_point && data.flush_to_zero == Some(true) && self.target == Target::Agx {
            let flushed = self.emit_flush_subnormal(data.type_, src);
            let result = self.emit_sign_bit_op(data.type_, flushed, true);
            self.resolver.register(arguments.dst, result);
        } else if is_floating_point && data.flush_to_zero == Some(true) {
            let negated = unsafe { llvm_fn(self.builder, src, LLVM_UNNAMED.as_ptr()) };
            let intrinsic = format!("llvm.canonicalize.{}\0", LLVMTypeDisplay(data.type_));
            self.emit_intrinsic(
                unsafe { CStr::from_bytes_with_nul_unchecked(intrinsic.as_bytes()) },
                Some(arguments.dst),
                vec![&data.type_.into()],
                vec![(negated, get_scalar_type(self.context, data.type_))],
            )?;
        } else if is_floating_point && self.target == Target::Agx {
            // A bit operation: the backend may canonicalize the NaNs of
            // floating point negations
            let result = self.emit_sign_bit_op(data.type_, src, true);
            self.resolver.register(arguments.dst, result);
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
        let dst = self.emit_not_impl(type_, src);
        self.resolver.register(arguments.dst, dst);
        Ok(())
    }

    // cnot: 1 if the source is zero, else 0
    fn emit_cnot(
        &mut self,
        type_: ptx_parser::ScalarType,
        arguments: ptx_parser::CnotArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let src = self.resolver.value(arguments.src)?;
        let type_ = get_scalar_type(self.context, type_);
        let zero = unsafe { LLVMConstInt(type_, 0, 0) };
        let is_zero = unsafe {
            LLVMBuildICmp(self.builder, LLVMIntPredicate::LLVMIntEQ, src, zero, LLVM_UNNAMED.as_ptr())
        };
        self.resolver.with_result(arguments.dst, |dst| unsafe {
            LLVMBuildZExt(self.builder, is_zero, type_, dst)
        });
        Ok(())
    }

    fn emit_not_impl(&mut self, type_: LLVMTypeRef, src: LLVMValueRef) -> LLVMValueRef {
        let constant = unsafe { LLVMConstInt(type_, u64::MAX, 1) };
        unsafe { LLVMBuildXor(self.builder, src, constant, LLVM_UNNAMED.as_ptr()) }
    }

    fn emit_setp(
        &mut self,
        data: ptx_parser::SetpData,
        arguments: ptx_parser::SetpArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let src1 = self.resolver.value(arguments.src1)?;
        let src2 = self.resolver.value(arguments.src2)?;
        if let Some(scalar_type) = data.type_.packed_type() {
            // setp.f16x2 p|q: p compares the low halves, q the high halves
            let u8_llvm = get_scalar_type(self.context, ast::ScalarType::B8);
            let lane = |i| unsafe { LLVMConstInt(u8_llvm, i, 0) };
            let b = self.builder;
            let (a0, a1, b0, b1) = unsafe {
                (
                    LLVMBuildExtractElement(b, src1, lane(0), LLVM_UNNAMED.as_ptr()),
                    LLVMBuildExtractElement(b, src1, lane(1), LLVM_UNNAMED.as_ptr()),
                    LLVMBuildExtractElement(b, src2, lane(0), LLVM_UNNAMED.as_ptr()),
                    LLVMBuildExtractElement(b, src2, lane(1), LLVM_UNNAMED.as_ptr()),
                )
            };
            let data = ptx_parser::SetpData {
                type_: scalar_type,
                ..data
            };
            let lo = self.emit_setp_impl(data, a0, b0)?;
            let hi = self.emit_setp_impl(data, a1, b1)?;
            self.resolver.register(arguments.dst1, lo);
            match arguments.dst2 {
                Some(dst2) => self.resolver.register(dst2, hi),
                None => return Err(error_unreachable()),
            }
            return Ok(());
        }
        let dst = self.emit_setp_impl(data, src1, src2)?;
        self.resolver.register(arguments.dst1, dst);
        if let Some(dst2) = arguments.dst2 {
            let not = unsafe { LLVMBuildNot(self.builder, dst, LLVM_UNNAMED.as_ptr()) };
            self.resolver.register(dst2, not);
        }
        Ok(())
    }

    fn emit_setp_impl(
        &mut self,
        data: ptx_parser::SetpData,
        src1: LLVMValueRef,
        src2: LLVMValueRef,
    ) -> Result<LLVMValueRef, TranslateError> {
        match data.cmp_op {
            ptx_parser::SetpCompareOp::Integer(setp_compare_int) => {
                self.emit_setp_int(setp_compare_int, src1, src2)
            }
            ptx_parser::SetpCompareOp::Float(setp_compare_float) => {
                // .ftz in a function of several modes, whose comparisons follow its attribute
                let (src1, src2) = if self.explicit_ftz(data.flush_to_zero) {
                    (self.emit_flush_subnormal(data.type_, src1), self.emit_flush_subnormal(data.type_, src2))
                } else {
                    (src1, src2)
                };
                self.emit_setp_float(setp_compare_float, src1, src2)
            }
        }
    }

    // .ftz done with bit operations: on AGX, in a function whose instructions take several denormal modes
    // (FloatingPointMode::Constrained), for the operations that don't follow the mode set at run time
    fn explicit_ftz(&self, flush_to_zero: Option<bool>) -> bool {
        self.target == Target::Agx
            && self.fp.mode == FloatingPointMode::Constrained
            && flush_to_zero == Some(true)
    }

    fn emit_setp_int(
        &mut self,
        setp: ptx_parser::SetpCompareInt,
        src1: LLVMValueRef,
        src2: LLVMValueRef,
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
        Ok(unsafe { LLVMBuildICmp(self.builder, op, src1, src2, LLVM_UNNAMED.as_ptr()) })
    }

    fn emit_setp_float(
        &mut self,
        setp: ptx_parser::SetpCompareFloat,
        src1: LLVMValueRef,
        src2: LLVMValueRef,
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
        Ok(unsafe { LLVMBuildFCmp(self.builder, op, src1, src2, LLVM_UNNAMED.as_ptr()) })
    }

    fn emit_conditional(&mut self, cond: BranchCondition) -> Result<(), TranslateError> {
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
        // On AGX the conversions to and from fp8 are computed here, see
        // emit_cvt_mini
        if self.target == Target::Agx {
            let mini = |t| match t {
                ast::ScalarType::E4m3x2 => Some(ast::MiniFormat::E4m3x2),
                ast::ScalarType::E5m2x2 => Some(ast::MiniFormat::E5m2x2),
                _ => None,
            };
            let mini_data = match (mini(data.from), mini(data.to), data.to) {
                (None, Some(format), _) => Some((format, ast::MiniDirection::FromF32)),
                (Some(format), None, ast::ScalarType::F16x2) => {
                    Some((format, ast::MiniDirection::ToF16x2))
                }
                (Some(format), None, ast::ScalarType::BF16x2) => {
                    Some((format, ast::MiniDirection::ToBf16x2))
                }
                _ => None,
            };
            if let Some((format, direction)) = mini_data {
                return self.emit_cvt_mini(
                    ast::CvtMiniDetails {
                        format,
                        direction,
                        rounding: ast::RoundingMode::NearestEven,
                        relu: false,
                    },
                    data.to,
                    arguments.dst,
                    arguments.src,
                    arguments.src2,
                );
            }
        }
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
            // .ftz flushes subnormal .f32 results (of bf16 / f64 sources; PTX ISA cvt: .ftz
            // "applies only to single precision (.f32) inputs and results")
            ptx_parser::CvtMode::FPExtend {
                flush_to_zero: Some(true),
                relu: false,
                satfinite: false,
                ..
            } if self.target == Target::Agx
                && data.to == ast::ScalarType::F32
                && data.from != ast::ScalarType::F32
                && arguments.src2.is_none() =>
            {
                let src = self.resolver.value(arguments.src)?;
                let result = self.emit_cvt_float_to_float_impl(None, data.to, None, data.from, src)?;
                let flushed = self.emit_flush_f32(result);
                self.resolver.register(arguments.dst, flushed);
                return Ok(());
            }
            ptx_parser::CvtMode::FPTruncate {
                rounding,
                flush_to_zero: Some(true),
                is_integer_rounding: false,
                relu: false,
                satfinite: false,
                ..
            } if self.target == Target::Agx
                && data.to == ast::ScalarType::F32
                && data.from == ast::ScalarType::F64
                && arguments.src2.is_none() =>
            {
                let src = self.resolver.value(arguments.src)?;
                let result =
                    self.emit_cvt_float_to_float_impl(Some(rounding), data.to, None, data.from, src)?;
                let flushed = self.emit_flush_f32(result);
                self.resolver.register(arguments.dst, flushed);
                return Ok(());
            }
            ptx_parser::CvtMode::FPExtend {
                flush_to_zero: Some(true),
                relu: false,
                satfinite: false,
                ..
            } if self.target == Target::Agx && data.from == ast::ScalarType::F32 => {
                // .ftz flushes subnormal f32 sources, which f64 represents
                let src = self.resolver.value(arguments.src)?;
                let flushed = self.emit_flush_f32(src);
                self.fp
                    .fpext(self.resolver, data.to, Some(arguments.dst), data.from, flushed)?;
                return Ok(());
            }
            ptx_parser::CvtMode::FPTruncate {
                rounding,
                flush_to_zero: Some(true),
                is_integer_rounding: false,
                relu,
                satfinite,
                ..
            } if self.target == Target::Agx
                && data.from == ast::ScalarType::F32
                && arguments.src2.is_none() =>
            {
                // .ftz flushes subnormal f32 sources
                let src = self.resolver.value(arguments.src)?;
                let flushed = self.emit_flush_f32(src);
                let mut result =
                    self.emit_cvt_float_to_float_impl(Some(rounding), data.to, None, data.from, flushed)?;
                if relu {
                    result = self.emit_relu(data.to, result)?;
                }
                if satfinite {
                    result = self.emit_satfinite(data.to, result)?;
                }
                self.resolver.register(arguments.dst, result);
                return Ok(());
            }
            ptx_parser::CvtMode::FPExtend {
                relu, satfinite, ..
            } => {
                return self.emit_cvt_float_to_float(
                    data.from,
                    data.to,
                    None,
                    relu,
                    satfinite,
                    arguments,
                )
            }
            ptx_parser::CvtMode::FPTruncate {
                rounding,
                relu,
                satfinite,
                ..
            } => {
                return self.emit_cvt_float_to_float(
                    data.from,
                    data.to,
                    Some(rounding),
                    relu,
                    satfinite,
                    arguments,
                )
            }
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
            ptx_parser::CvtMode::FPFromSigned { rounding, .. } => {
                return self.emit_cvt_int_to_float(data.from, data.to, rounding, arguments, true)
            }
            ptx_parser::CvtMode::FPFromUnsigned { rounding, .. } => {
                return self.emit_cvt_int_to_float(data.from, data.to, rounding, arguments, false)
            }
        };
        if arguments.src2.is_some() {
            return Err(error_unreachable());
        }
        let src = self.resolver.value(arguments.src)?;
        self.resolver.with_result(arguments.dst, |dst| unsafe {
            llvm_fn(self.builder, src, dst_type, dst)
        });
        Ok(())
    }

    fn emit_cvt_float_to_float(
        &mut self,
        from: ast::ScalarType,
        to: ast::ScalarType,
        // Some for truncations
        truncate: Option<ast::RoundingMode>,
        relu: bool,
        satfinite: bool,
        arguments: ptx_parser::CvtArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let src = self.resolver.value(arguments.src)?;
        // between pair types (the mixed-precision forms): lane by lane
        let lane = |t: ast::ScalarType| match t {
            ast::ScalarType::F32x2 => Some(ast::ScalarType::F32),
            ast::ScalarType::F16x2 => Some(ast::ScalarType::F16),
            ast::ScalarType::BF16x2 => Some(ast::ScalarType::BF16),
            _ => None,
        };
        if let (Some(from_lane), Some(to_lane), None) = (lane(from), lane(to), arguments.src2) {
            let i32_type = unsafe { LLVMInt32TypeInContext(self.context) };
            let mut vector = unsafe { LLVMGetUndef(get_scalar_type(self.context, to)) };
            for i in 0..2 {
                let index = unsafe { LLVMConstInt(i32_type, i, 0) };
                let element = unsafe { LLVMBuildExtractElement(self.builder, src, index, LLVM_UNNAMED.as_ptr()) };
                let mut converted = self.emit_cvt_float_to_float_impl(truncate, to_lane, None, from_lane, element)?;
                if relu {
                    converted = self.emit_relu(to_lane, converted)?;
                }
                if satfinite {
                    converted = self.emit_satfinite(to_lane, converted)?;
                }
                vector = unsafe { LLVMBuildInsertElement(self.builder, vector, converted, index, LLVM_UNNAMED.as_ptr()) };
            }
            self.resolver.register(arguments.dst, vector);
            return Ok(());
        }
        // cvt.frnd2.f16x2.f32 d, a, b;
        if let Some(src2) = arguments.src2 {
            let src2 = self.resolver.value(src2)?;
            return self.emit_cvt_float_to_packed_float(
                from,
                to,
                truncate,
                relu,
                satfinite,
                arguments.dst,
                src,
                src2,
            );
        }
        if relu || satfinite {
            let mut result = self.emit_cvt_float_to_float_impl(truncate, to, None, from, src)?;
            if relu {
                result = self.emit_relu(to, result)?;
            }
            if satfinite {
                result = self.emit_satfinite(to, result)?;
            }
            self.resolver.register(arguments.dst, result);
        } else {
            self.emit_cvt_float_to_float_impl(truncate, to, Some(arguments.dst), from, src)?;
        }
        Ok(())
    }

    // .satfinite: infinities become the largest finite value of the same
    // sign, NaN stays NaN
    fn emit_satfinite(
        &mut self,
        type_: ast::ScalarType,
        value: LLVMValueRef,
    ) -> Result<LLVMValueRef, TranslateError> {
        let llvm_type = get_scalar_type(self.context, type_);
        let max = match type_ {
            ast::ScalarType::F16 => 65504.0,
            ast::ScalarType::BF16 => f32::from_bits(0x7f7f0000) as f64,
            ast::ScalarType::F32 => f32::MAX as f64,
            ast::ScalarType::F64 => f64::MAX,
            _ => return Err(error_unreachable()),
        };
        let fabs = format!("llvm.fabs.{}\0", LLVMTypeDisplay(type_));
        let magnitude = self.emit_intrinsic(
            unsafe { CStr::from_bytes_with_nul_unchecked(fabs.as_bytes()) },
            None,
            vec![&type_.into()],
            vec![(value, llvm_type)],
        )?;
        let is_inf = unsafe {
            LLVMBuildFCmp(
                self.builder,
                LLVMRealPredicate::LLVMRealOEQ,
                magnitude,
                LLVMConstReal(llvm_type, f64::INFINITY),
                LLVM_UNNAMED.as_ptr(),
            )
        };
        let copysign = format!("llvm.copysign.{}\0", LLVMTypeDisplay(type_));
        let max = self.emit_intrinsic(
            unsafe { CStr::from_bytes_with_nul_unchecked(copysign.as_bytes()) },
            None,
            vec![&type_.into()],
            vec![
                (unsafe { LLVMConstReal(llvm_type, max) }, llvm_type),
                (value, llvm_type),
            ],
        )?;
        Ok(unsafe { LLVMBuildSelect(self.builder, is_inf, max, value, LLVM_UNNAMED.as_ptr()) })
    }

    fn emit_cvt_float_to_packed_float(
        &mut self,
        from: ast::ScalarType,
        to: ast::ScalarType,
        truncate: Option<ast::RoundingMode>,
        relu: bool,
        satfinite: bool,
        dst: SpirvWord,
        src: LLVMValueRef,
        src2: LLVMValueRef,
    ) -> Result<(), TranslateError> {
        let element_type = to.packed_type().ok_or_else(|| error_mismatched_type())?;
        let mut src_result =
            self.emit_cvt_float_to_float_impl(truncate, element_type, None, from, src)?;
        let mut src2_result =
            self.emit_cvt_float_to_float_impl(truncate, element_type, None, from, src2)?;
        if relu {
            src_result = self.emit_relu(element_type, src_result)?;
            src2_result = self.emit_relu(element_type, src2_result)?;
        }
        if satfinite {
            src_result = self.emit_satfinite(element_type, src_result)?;
            src2_result = self.emit_satfinite(element_type, src2_result)?;
        }
        let dst_type = get_scalar_type(self.context, to);
        let vec = unsafe {
            LLVMBuildInsertElement(
                self.builder,
                LLVMGetPoison(dst_type),
                src_result,
                LLVMConstInt(LLVMInt32TypeInContext(self.context), 1, false as i32),
                LLVM_UNNAMED.as_ptr(),
            )
        };
        self.resolver.with_result(dst, |dst| unsafe {
            LLVMBuildInsertElement(
                self.builder,
                vec,
                src2_result,
                LLVMConstInt(LLVMInt32TypeInContext(self.context), 0, false as i32),
                dst,
            )
        });
        Ok(())
    }

    fn emit_cvt_float_to_float_impl(
        &mut self,
        truncate: Option<ast::RoundingMode>,
        dst_type: ast::ScalarType,
        dst: Option<SpirvWord>,
        src_type: ast::ScalarType,
        src: LLVMValueRef,
    ) -> Result<LLVMValueRef, TranslateError> {
        if let Some(rounding) = truncate {
            if self.is_bf16_directed_rounding(dst_type, rounding) {
                let wide = if src_type == ast::ScalarType::F32 {
                    src
                } else if src_type.size_of() < 4 {
                    self.fp
                        .fpext(self.resolver, ast::ScalarType::F32, None, src_type, src)?
                } else {
                    self.fp
                        .fptrunc(self.resolver, ast::ScalarType::F32, None, src_type, src)?
                };
                let result = self.emit_f32_to_bf16_directed(rounding, wide);
                if let Some(dst) = dst {
                    self.resolver.register(dst, result);
                }
                return Ok(result);
            }
        }
        let truncate = truncate.is_some();
        if truncate && dst_type.size_of() == src_type.size_of() {
            // f16 <-> bf16: through f32, which holds both exactly
            let wide = self
                .fp
                .fpext(self.resolver, ast::ScalarType::F32, None, src_type, src)?;
            self.fp
                .fptrunc(self.resolver, dst_type, dst, ast::ScalarType::F32, wide)
        } else if truncate {
            self.fp.fptrunc(self.resolver, dst_type, dst, src_type, src)
        } else {
            self.fp.fpext(self.resolver, dst_type, dst, src_type, src)
        }
    }

    fn emit_relu(
        &mut self,
        type_: ast::ScalarType,
        src: LLVMValueRef,
    ) -> Result<LLVMValueRef, TranslateError> {
        let llvm_type = get_scalar_type(self.context, type_);
        let zero = unsafe { LLVMConstNull(llvm_type) };
        let intrinsic = format!("llvm.maximum.{}\0", LLVMTypeDisplay(type_));
        let result = self.emit_intrinsic(
            unsafe { CStr::from_bytes_with_nul_unchecked(intrinsic.as_bytes()) },
            None,
            vec![&type_.into()],
            vec![(src, llvm_type), (zero, llvm_type)],
        )?;
        Ok(self.emit_canonical_nan(type_, result))
    }

    // A NaN value (or element) replaced by the canonical NaN of its type, on AGX: the results of min, max and .relu
    // other than f64 (whose NaN payloads are kept)
    fn emit_canonical_nan(&mut self, type_: ast::ScalarType, value: LLVMValueRef) -> LLVMValueRef {
        let element = type_.packed_type().unwrap_or(type_);
        let canonical: u64 = match element {
            ast::ScalarType::F16 | ast::ScalarType::BF16 => 0x7fff,
            ast::ScalarType::F32 => 0x7fff_ffff,
            _ => return value,
        };
        if self.target != Target::Agx {
            return value;
        }
        unsafe {
            let b = self.builder;
            let element_bits = element.size_of() as u32 * 8;
            let int_element = LLVMIntTypeInContext(self.context, element_bits);
            let mut nan = LLVMConstBitCast(LLVMConstInt(int_element, canonical, 0), get_scalar_type(self.context, element));
            if element != type_ {
                let mut ns = [nan, nan];
                nan = LLVMConstVector(ns.as_mut_ptr(), 2);
            }
            let is_nan = LLVMBuildFCmp(b, LLVMRealPredicate::LLVMRealUNO, value, value, LLVM_UNNAMED.as_ptr());
            LLVMBuildSelect(b, is_nan, nan, value, LLVM_UNNAMED.as_ptr())
        }
    }

    // Conversions to and from the narrow formats (fp8 e4m3 and e5m2, fp6
    // e2m3 and e3m2, fp4 e2m1, and the exponent-only ue8m0) with integer
    // arithmetic on the bit patterns. Narrowing rounds to nearest even (or
    // toward zero, or up for ue8m0) and saturates at the largest finite
    // value; NaN becomes the format's NaN (the largest finite value for the
    // formats without NaN); .relu turns negative values into +0. Widening
    // is exact. The first source of a pair goes to the high element.
    fn emit_cvt_mini(
        &mut self,
        data: ast::CvtMiniDetails,
        dst_type: ast::ScalarType,
        dst: SpirvWord,
        src1: SpirvWord,
        src2: Option<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let i32_type = get_scalar_type(self.context, ast::ScalarType::B32);
        let builder = self.builder;
        let c = |v: u64| unsafe { LLVMConstInt(i32_type, v, 0) };
        let element_bits = mini_element_bits(data.format);
        // container width of an element in the packed pair
        let lane = if element_bits <= 4 { 4 } else { 8 };
        let src1_value = self.resolver.value(src1)?;
        // sources are f32, i32 or i16
        let to_i32 = |this: &mut Self, value: LLVMValueRef| unsafe {
            let type_ = LLVMTypeOf(value);
            if LLVMGetTypeKind(type_) == LLVMTypeKind::LLVMFloatTypeKind {
                LLVMBuildBitCast(this.builder, value, i32_type, LLVM_UNNAMED.as_ptr())
            } else {
                LLVMBuildZExtOrBitCast(this.builder, value, i32_type, LLVM_UNNAMED.as_ptr())
            }
        };
        let result = match data.direction {
            ast::MiniDirection::FromF32 | ast::MiniDirection::FromF16x2 | ast::MiniDirection::FromBf16x2 => {
                // f32 bit patterns of the high and low elements
                let (high, low) = match data.direction {
                    ast::MiniDirection::FromF32 => {
                        let src2 = self.resolver.value(src2.ok_or_else(error_unreachable)?)?;
                        (to_i32(self, src1_value), to_i32(self, src2))
                    }
                    ast::MiniDirection::FromBf16x2 => {
                        let packed = to_i32(self, src1_value);
                        unsafe {
                            let high = LLVMBuildAnd(builder, packed, c(0xffff_0000), LLVM_UNNAMED.as_ptr());
                            let low = LLVMBuildShl(builder, packed, c(16), LLVM_UNNAMED.as_ptr());
                            (high, low)
                        }
                    }
                    _ => {
                        let packed = to_i32(self, src1_value);
                        let half = |this: &mut Self, shift: u64| unsafe {
                            let bits = LLVMBuildLShr(this.builder, packed, c(shift), LLVM_UNNAMED.as_ptr());
                            let bits = LLVMBuildAnd(this.builder, bits, c(0xffff), LLVM_UNNAMED.as_ptr());
                            this.emit_f16_bits_to_f32_bits(bits)
                        };
                        (half(self, 16), half(self, 0))
                    }
                };
                let high = self.emit_mini_encode(data, high);
                let low = self.emit_mini_encode(data, low);
                unsafe {
                    let high = LLVMBuildShl(builder, high, c(lane), LLVM_UNNAMED.as_ptr());
                    let packed = LLVMBuildOr(builder, high, low, LLVM_UNNAMED.as_ptr());
                    LLVMBuildTrunc(builder, packed, LLVMInt16TypeInContext(self.context), LLVM_UNNAMED.as_ptr())
                }
            }
            ast::MiniDirection::ToF16x2 | ast::MiniDirection::ToBf16x2 => {
                let packed = to_i32(self, src1_value);
                let mut halves = [ptr::null_mut(); 2];
                for (i, half) in halves.iter_mut().enumerate() {
                    let element = unsafe {
                        let element = LLVMBuildLShr(builder, packed, c(lane * i as u64), LLVM_UNNAMED.as_ptr());
                        LLVMBuildAnd(builder, element, c((1 << lane) - 1), LLVM_UNNAMED.as_ptr())
                    };
                    *half = if data.format == ast::MiniFormat::Ue8m0x2 {
                        self.emit_ue8m0_decode(element)
                    } else {
                        let f16 = self.emit_mini_decode(data.format, element);
                        if data.direction == ast::MiniDirection::ToBf16x2 {
                            // exact: the narrow formats have few mantissa bits
                            let wide = self.emit_f16_bits_to_f32_bits(f16);
                            unsafe { LLVMBuildLShr(builder, wide, c(16), LLVM_UNNAMED.as_ptr()) }
                        } else {
                            f16
                        }
                    };
                }
                unsafe {
                    let high = LLVMBuildShl(builder, halves[1], c(16), LLVM_UNNAMED.as_ptr());
                    LLVMBuildOr(builder, high, halves[0], LLVM_UNNAMED.as_ptr())
                }
            }
        };
        // i16 or i32, as the destination type (an integer or packed halves)
        let result = unsafe {
            LLVMBuildBitCast(builder, result, get_scalar_type(self.context, dst_type), LLVM_UNNAMED.as_ptr())
        };
        self.resolver.register(dst, result);
        Ok(())
    }

    // Exact conversion of f16 bits to f32 bits (both in i32), with integer
    // arithmetic so that subnormals are kept in any floating point mode
    fn emit_f16_bits_to_f32_bits(&mut self, bits: LLVMValueRef) -> LLVMValueRef {
        let i32_type = get_scalar_type(self.context, ast::ScalarType::B32);
        let b = self.builder;
        let c = |v: u64| unsafe { LLVMConstInt(i32_type, v, 0) };
        unsafe {
            let sign = LLVMBuildShl(b, LLVMBuildAnd(b, bits, c(0x8000), LLVM_UNNAMED.as_ptr()), c(16), LLVM_UNNAMED.as_ptr());
            let exponent = LLVMBuildAnd(b, LLVMBuildLShr(b, bits, c(10), LLVM_UNNAMED.as_ptr()), c(0x1f), LLVM_UNNAMED.as_ptr());
            let mantissa = LLVMBuildAnd(b, bits, c(0x3ff), LLVM_UNNAMED.as_ptr());
            let normal = LLVMBuildOr(
                b,
                LLVMBuildShl(b, LLVMBuildAdd(b, exponent, c(127 - 15), LLVM_UNNAMED.as_ptr()), c(23), LLVM_UNNAMED.as_ptr()),
                LLVMBuildShl(b, mantissa, c(13), LLVM_UNNAMED.as_ptr()),
                LLVM_UNNAMED.as_ptr(),
            );
            let special = LLVMBuildOr(b, c(0x7f80_0000), LLVMBuildShl(b, mantissa, c(13), LLVM_UNNAMED.as_ptr()), LLVM_UNNAMED.as_ptr());
            // subnormal: mantissa * 2^-24, top bit at position p
            let false_ = LLVMConstInt(LLVMInt1TypeInContext(self.context), 0, 0);
            let leading = self.emit_int_intrinsic("llvm.ctlz", i32_type, &[mantissa, false_]);
            let position = LLVMBuildSub(b, c(31), leading, LLVM_UNNAMED.as_ptr());
            let sub_exponent = LLVMBuildAdd(b, position, c(127 - 24), LLVM_UNNAMED.as_ptr());
            let sub_mantissa = LLVMBuildShl(b, mantissa, LLVMBuildSub(b, c(23), position, LLVM_UNNAMED.as_ptr()), LLVM_UNNAMED.as_ptr());
            let sub_mantissa = LLVMBuildAnd(b, sub_mantissa, c(0x7f_ffff), LLVM_UNNAMED.as_ptr());
            let subnormal = LLVMBuildOr(b, LLVMBuildShl(b, sub_exponent, c(23), LLVM_UNNAMED.as_ptr()), sub_mantissa, LLVM_UNNAMED.as_ptr());
            let is_zero = LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntEQ, mantissa, c(0), LLVM_UNNAMED.as_ptr());
            let subnormal = LLVMBuildSelect(b, is_zero, c(0), subnormal, LLVM_UNNAMED.as_ptr());
            let is_subnormal = LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntEQ, exponent, c(0), LLVM_UNNAMED.as_ptr());
            let is_special = LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntEQ, exponent, c(0x1f), LLVM_UNNAMED.as_ptr());
            let value = LLVMBuildSelect(b, is_subnormal, subnormal, normal, LLVM_UNNAMED.as_ptr());
            let value = LLVMBuildSelect(b, is_special, special, value, LLVM_UNNAMED.as_ptr());
            LLVMBuildOr(b, value, sign, LLVM_UNNAMED.as_ptr())
        }
    }

    // f32 bits -> element code (in the low bits of an i32)
    fn emit_mini_encode(&mut self, data: ast::CvtMiniDetails, bits: LLVMValueRef) -> LLVMValueRef {
        if data.format == ast::MiniFormat::Ue8m0x2 {
            return self.emit_ue8m0_encode(data.rounding == ast::RoundingMode::PositiveInf, bits);
        }
        let (exponent_bits, mantissa_bits, top, nan) = mini_format(data.format);
        let bias = (1u64 << (exponent_bits - 1)) - 1;
        let element_bits = mini_element_bits(data.format) as u64;
        let i32_type = get_scalar_type(self.context, ast::ScalarType::B32);
        let b = self.builder;
        let c = |v: u64| unsafe { LLVMConstInt(i32_type, v, 0) };
        let toward_zero = data.rounding == ast::RoundingMode::Zero;
        unsafe {
            // round value >> shift to nearest even (or toward zero)
            let round = |this: &mut Self, value: LLVMValueRef, shift: LLVMValueRef| {
                let q = LLVMBuildLShr(b, value, shift, LLVM_UNNAMED.as_ptr());
                if toward_zero {
                    return q;
                }
                let one = c(1);
                let unit = LLVMBuildShl(b, one, shift, LLVM_UNNAMED.as_ptr());
                let mask = LLVMBuildSub(b, unit, one, LLVM_UNNAMED.as_ptr());
                let rest = LLVMBuildAnd(b, value, mask, LLVM_UNNAMED.as_ptr());
                let half = LLVMBuildLShr(b, unit, one, LLVM_UNNAMED.as_ptr());
                let above = LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntUGT, rest, half, LLVM_UNNAMED.as_ptr());
                let tie = LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntEQ, rest, half, LLVM_UNNAMED.as_ptr());
                let odd = LLVMBuildTrunc(b, q, LLVMInt1TypeInContext(this.context), LLVM_UNNAMED.as_ptr());
                let tie_up = LLVMBuildAnd(b, tie, odd, LLVM_UNNAMED.as_ptr());
                let up = LLVMBuildOr(b, above, tie_up, LLVM_UNNAMED.as_ptr());
                let up = LLVMBuildZExt(b, up, i32_type, LLVM_UNNAMED.as_ptr());
                LLVMBuildAdd(b, q, up, LLVM_UNNAMED.as_ptr())
            };
            let magnitude = LLVMBuildAnd(b, bits, c(0x7fff_ffff), LLVM_UNNAMED.as_ptr());
            let negative = LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntSLT, bits, c(0), LLVM_UNNAMED.as_ptr());
            let exponent = LLVMBuildLShr(b, magnitude, c(23), LLVM_UNNAMED.as_ptr());
            // normal in the narrow format: rebias the exponent and round
            let min_exponent_field = 127 + 1 - bias;
            let is_normal = LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntUGE, exponent, c(min_exponent_field), LLVM_UNNAMED.as_ptr());
            let rebiased = LLVMBuildSub(b, magnitude, c((127 - bias) << 23), LLVM_UNNAMED.as_ptr());
            let normal = round(self, rebiased, c(23 - mantissa_bits));
            // subnormal: the significand shifted by the missing exponent
            let significand = LLVMBuildOr(
                b,
                LLVMBuildAnd(b, magnitude, c(0x7f_ffff), LLVM_UNNAMED.as_ptr()),
                c(0x80_0000),
                LLVM_UNNAMED.as_ptr(),
            );
            let shift = LLVMBuildSub(b, c(min_exponent_field + 23 - mantissa_bits), exponent, LLVM_UNNAMED.as_ptr());
            let shift = self.emit_int_intrinsic("llvm.umin", i32_type, &[shift, c(31)]);
            let subnormal = round(self, significand, shift);
            let code = LLVMBuildSelect(b, is_normal, normal, subnormal, LLVM_UNNAMED.as_ptr());
            // saturation (also of infinities)
            let code = self.emit_int_intrinsic("llvm.umin", i32_type, &[code, c(top)]);
            let signed = LLVMBuildOr(b, code, c(1 << (element_bits - 1)), LLVM_UNNAMED.as_ptr());
            let code = LLVMBuildSelect(b, negative, signed, code, LLVM_UNNAMED.as_ptr());
            let code = if data.relu {
                LLVMBuildSelect(b, negative, c(0), code, LLVM_UNNAMED.as_ptr())
            } else {
                code
            };
            let is_nan = LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntUGT, magnitude, c(0x7f80_0000), LLVM_UNNAMED.as_ptr());
            LLVMBuildSelect(b, is_nan, c(nan), code, LLVM_UNNAMED.as_ptr())
        }
    }

    // ue8m0: the exponent of a positive value, rounded down (or up), biased
    // by 127 and clamped to 0..254; zero and negative finite values give 0,
    // infinities 254 and NaN 255
    fn emit_ue8m0_encode(&mut self, round_up: bool, bits: LLVMValueRef) -> LLVMValueRef {
        let i32_type = get_scalar_type(self.context, ast::ScalarType::B32);
        let b = self.builder;
        let c = |v: i64| unsafe { LLVMConstInt(i32_type, v as u64, 1) };
        unsafe {
            let magnitude = LLVMBuildAnd(b, bits, c(0x7fff_ffff), LLVM_UNNAMED.as_ptr());
            let exponent = LLVMBuildLShr(b, magnitude, c(23), LLVM_UNNAMED.as_ptr());
            let mantissa = LLVMBuildAnd(b, magnitude, c(0x7f_ffff), LLVM_UNNAMED.as_ptr());
            // log2 of a subnormal: -149 + the position of its top bit
            let false_ = LLVMConstInt(LLVMInt1TypeInContext(self.context), 0, 0);
            let leading = self.emit_int_intrinsic("llvm.ctlz", i32_type, &[magnitude, false_]);
            let subnormal_log = LLVMBuildSub(b, c(-149 + 31), leading, LLVM_UNNAMED.as_ptr());
            let is_normal = LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntNE, exponent, c(0), LLVM_UNNAMED.as_ptr());
            let normal_log = LLVMBuildSub(b, exponent, c(127), LLVM_UNNAMED.as_ptr());
            let log = LLVMBuildSelect(b, is_normal, normal_log, subnormal_log, LLVM_UNNAMED.as_ptr());
            let log = if round_up {
                let power_normal = LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntEQ, mantissa, c(0), LLVM_UNNAMED.as_ptr());
                let below = LLVMBuildSub(b, magnitude, c(1), LLVM_UNNAMED.as_ptr());
                let power_subnormal = LLVMBuildICmp(
                    b,
                    LLVMIntPredicate::LLVMIntEQ,
                    LLVMBuildAnd(b, magnitude, below, LLVM_UNNAMED.as_ptr()),
                    c(0),
                    LLVM_UNNAMED.as_ptr(),
                );
                let power = LLVMBuildSelect(b, is_normal, power_normal, power_subnormal, LLVM_UNNAMED.as_ptr());
                let up = LLVMBuildZExt(b, LLVMBuildNot(b, power, LLVM_UNNAMED.as_ptr()), i32_type, LLVM_UNNAMED.as_ptr());
                LLVMBuildAdd(b, log, up, LLVM_UNNAMED.as_ptr())
            } else {
                log
            };
            let code = LLVMBuildAdd(b, log, c(127), LLVM_UNNAMED.as_ptr());
            let code = self.emit_int_intrinsic("llvm.smax", i32_type, &[code, c(0)]);
            let code = self.emit_int_intrinsic("llvm.smin", i32_type, &[code, c(254)]);
            let not_positive = LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntSLE, bits, c(0), LLVM_UNNAMED.as_ptr());
            let code = LLVMBuildSelect(b, not_positive, c(0), code, LLVM_UNNAMED.as_ptr());
            // .satfinite (which ue8m0x2 requires): both infinities give the largest finite code
            let is_inf = LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntEQ, magnitude, c(0x7f80_0000), LLVM_UNNAMED.as_ptr());
            let code = LLVMBuildSelect(b, is_inf, c(254), code, LLVM_UNNAMED.as_ptr());
            let is_nan = LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntUGT, magnitude, c(0x7f80_0000), LLVM_UNNAMED.as_ptr());
            LLVMBuildSelect(b, is_nan, c(255), code, LLVM_UNNAMED.as_ptr())
        }
    }

    // ue8m0 code -> bf16 bits: 2^(code - 127), 255 is NaN
    fn emit_ue8m0_decode(&mut self, code: LLVMValueRef) -> LLVMValueRef {
        let i32_type = get_scalar_type(self.context, ast::ScalarType::B32);
        let b = self.builder;
        let c = |v: u64| unsafe { LLVMConstInt(i32_type, v, 0) };
        unsafe {
            let value = LLVMBuildShl(b, code, c(7), LLVM_UNNAMED.as_ptr());
            let is_zero = LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntEQ, code, c(0), LLVM_UNNAMED.as_ptr());
            let value = LLVMBuildSelect(b, is_zero, c(0x40), value, LLVM_UNNAMED.as_ptr());
            let is_nan = LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntEQ, code, c(255), LLVM_UNNAMED.as_ptr());
            LLVMBuildSelect(b, is_nan, c(0x7fff), value, LLVM_UNNAMED.as_ptr())
        }
    }

    // element code -> f16 bits (exact)
    fn emit_mini_decode(&mut self, format: ast::MiniFormat, code: LLVMValueRef) -> LLVMValueRef {
        let (exponent_bits, mantissa_bits, _, _) = mini_format(format);
        let bias = (1u64 << (exponent_bits - 1)) - 1;
        let element_bits = mini_element_bits(format) as u64;
        let i32_type = get_scalar_type(self.context, ast::ScalarType::B32);
        let b = self.builder;
        let c = |v: u64| unsafe { LLVMConstInt(i32_type, v, 0) };
        unsafe {
            let magnitude = LLVMBuildAnd(b, code, c((1 << (element_bits - 1)) - 1), LLVM_UNNAMED.as_ptr());
            let sign = LLVMBuildLShr(b, code, c(element_bits - 1), LLVM_UNNAMED.as_ptr());
            let sign = LLVMBuildShl(b, sign, c(15), LLVM_UNNAMED.as_ptr());
            let value = if format == ast::MiniFormat::E5m2x2 {
                // the upper byte of an f16
                LLVMBuildShl(b, magnitude, c(8), LLVM_UNNAMED.as_ptr())
            } else {
                let exponent = LLVMBuildLShr(b, magnitude, c(mantissa_bits), LLVM_UNNAMED.as_ptr());
                let mantissa = LLVMBuildAnd(b, magnitude, c((1 << mantissa_bits) - 1), LLVM_UNNAMED.as_ptr());
                let normal = LLVMBuildOr(
                    b,
                    LLVMBuildShl(b, LLVMBuildAdd(b, exponent, c(15 - bias), LLVM_UNNAMED.as_ptr()), c(10), LLVM_UNNAMED.as_ptr()),
                    LLVMBuildShl(b, mantissa, c(10 - mantissa_bits), LLVM_UNNAMED.as_ptr()),
                    LLVM_UNNAMED.as_ptr(),
                );
                // subnormal: normalized in f16, top bit at position p
                let false_ = LLVMConstInt(LLVMInt1TypeInContext(self.context), 0, 0);
                let leading = self.emit_int_intrinsic("llvm.ctlz", i32_type, &[mantissa, false_]);
                let position = LLVMBuildSub(b, c(31), leading, LLVM_UNNAMED.as_ptr());
                let f16_exponent = LLVMBuildAdd(b, position, c(1 + 15 - bias - mantissa_bits), LLVM_UNNAMED.as_ptr());
                let f16_mantissa = LLVMBuildShl(b, mantissa, LLVMBuildSub(b, c(10), position, LLVM_UNNAMED.as_ptr()), LLVM_UNNAMED.as_ptr());
                let f16_mantissa = LLVMBuildAnd(b, f16_mantissa, c(0x3ff), LLVM_UNNAMED.as_ptr());
                let subnormal = LLVMBuildOr(b, LLVMBuildShl(b, f16_exponent, c(10), LLVM_UNNAMED.as_ptr()), f16_mantissa, LLVM_UNNAMED.as_ptr());
                let is_zero = LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntEQ, mantissa, c(0), LLVM_UNNAMED.as_ptr());
                let subnormal = LLVMBuildSelect(b, is_zero, c(0), subnormal, LLVM_UNNAMED.as_ptr());
                let is_normal = LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntNE, exponent, c(0), LLVM_UNNAMED.as_ptr());
                LLVMBuildSelect(b, is_normal, normal, subnormal, LLVM_UNNAMED.as_ptr())
            };
            let value = LLVMBuildOr(b, value, sign, LLVM_UNNAMED.as_ptr());
            let is_nan = match format {
                ast::MiniFormat::E4m3x2 => {
                    Some(LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntEQ, magnitude, c(0x7f), LLVM_UNNAMED.as_ptr()))
                }
                ast::MiniFormat::E5m2x2 => {
                    Some(LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntUGT, magnitude, c(0x7c), LLVM_UNNAMED.as_ptr()))
                }
                _ => None,
            };
            match is_nan {
                Some(is_nan) => LLVMBuildSelect(b, is_nan, c(0x7fff), value, LLVM_UNNAMED.as_ptr()),
                None => value,
            }
        }
    }

    // Rounding on the bit pattern: tf32 is the f32 layout with 13 fraction
    // bits fewer. Rounding up the magnitude carries into the exponent, which
    // also gives the infinities on overflow
    fn emit_cvt_tf32(
        &mut self,
        data: ast::CvtTf32Details,
        arguments: ast::CvtTf32Args<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let b = self.builder;
        let i32_type = get_scalar_type(self.context, ast::ScalarType::B32);
        let c = |v: u64| unsafe { LLVMConstInt(i32_type, v, 0) };
        let src = self.resolver.value(arguments.src)?;
        unsafe {
            let bits = LLVMBuildBitCast(b, src, i32_type, LLVM_UNNAMED.as_ptr());
            let magnitude = LLVMBuildAnd(b, bits, c(0x7fff_ffff), LLVM_UNNAMED.as_ptr());
            let sign = LLVMBuildAnd(b, bits, c(0x8000_0000), LLVM_UNNAMED.as_ptr());
            let negative = LLVMBuildICmp(
                b,
                LLVMIntPredicate::LLVMIntNE,
                sign,
                c(0),
                LLVM_UNNAMED.as_ptr(),
            );
            let inexact = LLVMBuildICmp(
                b,
                LLVMIntPredicate::LLVMIntNE,
                LLVMBuildAnd(b, bits, c(0x1fff), LLVM_UNNAMED.as_ptr()),
                c(0),
                LLVM_UNNAMED.as_ptr(),
            );
            let increment = match data.rounding {
                ast::Tf32Rounding::Rz => c(0),
                ast::Tf32Rounding::Rna => c(0x1000),
                ast::Tf32Rounding::Rn => {
                    let odd = LLVMBuildAnd(
                        b,
                        LLVMBuildLShr(b, bits, c(13), LLVM_UNNAMED.as_ptr()),
                        c(1),
                        LLVM_UNNAMED.as_ptr(),
                    );
                    LLVMBuildAdd(b, odd, c(0xfff), LLVM_UNNAMED.as_ptr())
                }
                ast::Tf32Rounding::Rm | ast::Tf32Rounding::Rp => {
                    let away = if data.rounding == ast::Tf32Rounding::Rm {
                        negative
                    } else {
                        LLVMBuildNot(b, negative, LLVM_UNNAMED.as_ptr())
                    };
                    let up = LLVMBuildAnd(b, away, inexact, LLVM_UNNAMED.as_ptr());
                    LLVMBuildSelect(b, up, c(0x1fff), c(0), LLVM_UNNAMED.as_ptr())
                }
            };
            let rounded = LLVMBuildAdd(b, bits, increment, LLVM_UNNAMED.as_ptr());
            let mut result = LLVMBuildAnd(b, rounded, c(0xffff_e000), LLVM_UNNAMED.as_ptr());
            if data.satfinite {
                let result_magnitude =
                    LLVMBuildAnd(b, result, c(0x7fff_ffff), LLVM_UNNAMED.as_ptr());
                let is_inf = LLVMBuildICmp(
                    b,
                    LLVMIntPredicate::LLVMIntEQ,
                    result_magnitude,
                    c(0x7f80_0000),
                    LLVM_UNNAMED.as_ptr(),
                );
                let max = LLVMBuildOr(b, sign, c(0x7f7f_e000), LLVM_UNNAMED.as_ptr());
                result = LLVMBuildSelect(b, is_inf, max, result, LLVM_UNNAMED.as_ptr());
            }
            if data.relu {
                result = LLVMBuildSelect(b, negative, c(0), result, LLVM_UNNAMED.as_ptr());
            }
            let is_nan = LLVMBuildICmp(
                b,
                LLVMIntPredicate::LLVMIntUGT,
                magnitude,
                c(0x7f80_0000),
                LLVM_UNNAMED.as_ptr(),
            );
            self.resolver.with_result(arguments.dst, |dst| {
                LLVMBuildSelect(b, is_nan, c(0x7fff_ffff), result, dst)
            });
        }
        Ok(())
    }

    // alloca, stacksave and stackrestore work on the stack of private memory
    // (see PtxStack); their results are generic addresses
    fn emit_alloca(
        &mut self,
        data: ast::AllocaDetails,
        arguments: ast::AllocaArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let size = self.resolver.value(arguments.src)?;
        let i8_type = get_scalar_type(self.context, ast::ScalarType::B8);
        let i32_type = get_scalar_type(self.context, ast::ScalarType::B32);
        let i64_type = get_scalar_type(self.context, ast::ScalarType::B64);
        let constant_size = unsafe {
            if LLVMIsAConstantInt(size) != ptr::null_mut() {
                Some(LLVMConstIntGetZExtValue(size))
            } else {
                None
            }
        };
        let constant_size = constant_size.filter(|s| *s < (1 << 24));
        if constant_size.is_none() && self.target != Target::Agx {
            // Allocations of a dynamic size are not freed by stackrestore
            let alloca = unsafe {
                LLVMZludaBuildArrayAlloca(
                    self.builder,
                    i8_type,
                    get_state_space(ast::StateSpace::Local)?,
                    size,
                    data.align,
                    LLVM_UNNAMED.as_ptr(),
                )
            };
            let address = self.emit_private_to_generic_int(alloca, data.type_)?;
            self.resolver.register(arguments.dst, address);
            return Ok(());
        }
        let (base, top) = self.get_or_create_stack()?;
        let stack = self.stack.as_mut().ok_or_else(error_unreachable)?;
        match constant_size {
            Some(constant_size) => stack.size += constant_size + data.align as u64 - 1,
            None => stack.dynamic = true,
        }
        stack.align = stack.align.max(data.align);
        let align_mask = data.align as u64 - 1;
        let offset = unsafe {
            let old_top = LLVMBuildLoad2(self.builder, i32_type, top, LLVM_UNNAMED.as_ptr());
            let size = LLVMBuildZExtOrBitCast(self.builder, size, i64_type, LLVM_UNNAMED.as_ptr());
            let old_top = LLVMBuildZExt(self.builder, old_top, i64_type, LLVM_UNNAMED.as_ptr());
            if self.target == Target::Agx {
                let overflow = LLVMBuildICmp(
                    self.builder,
                    LLVMIntPredicate::LLVMIntUGT,
                    size,
                    old_top,
                    LLVM_UNNAMED.as_ptr(),
                );
                self.emit_stack_overflow_check(overflow)?;
            }
            let offset = LLVMBuildSub(self.builder, old_top, size, LLVM_UNNAMED.as_ptr());
            let offset = LLVMBuildAnd(
                self.builder,
                offset,
                LLVMConstInt(i64_type, !align_mask, 0),
                LLVM_UNNAMED.as_ptr(),
            );
            let offset = LLVMBuildTrunc(self.builder, offset, i32_type, LLVM_UNNAMED.as_ptr());
            LLVMBuildStore(self.builder, offset, top);
            offset
        };
        let address = unsafe {
            let mut indices = [offset];
            LLVMBuildInBoundsGEP2(
                self.builder,
                i8_type,
                base,
                indices.as_mut_ptr(),
                1,
                LLVM_UNNAMED.as_ptr(),
            )
        };
        let address = self.emit_private_to_generic_int(address, data.type_)?;
        self.resolver.register(arguments.dst, address);
        Ok(())
    }

    // A thread for which `overflow` is true sets __agx_stack_overflow and ends.
    fn emit_stack_overflow_check(&mut self, overflow: LLVMValueRef) -> Result<(), TranslateError> {
        let i32_type = get_scalar_type(self.context, ast::ScalarType::B32);
        unsafe {
            let flag = match LLVMGetNamedGlobal(self.module, c"__agx_stack_overflow".as_ptr()) {
                flag if !flag.is_null() => flag,
                _ => {
                    let flag = LLVMAddGlobalInAddressSpace(
                        self.module,
                        i32_type,
                        c"__agx_stack_overflow".as_ptr(),
                        get_state_space(ast::StateSpace::Global)?,
                    );
                    LLVMSetInitializer(flag, LLVMConstInt(i32_type, 0, 0));
                    LLVMSetAlignment(flag, 4);
                    // one word for the modules linked together
                    LLVMSetLinkage(flag, LLVMLinkage::LLVMLinkOnceODRLinkage);
                    flag
                }
            };
            let fail = LLVMAppendBasicBlockInContext(self.context, self.method, LLVM_UNNAMED.as_ptr());
            let ok = LLVMAppendBasicBlockInContext(self.context, self.method, LLVM_UNNAMED.as_ptr());
            LLVMBuildCondBr(self.builder, overflow, fail, ok);
            LLVMPositionBuilderAtEnd(self.builder, fail);
            let store = LLVMBuildStore(self.builder, LLVMConstInt(i32_type, 1, 0), flag);
            LLVMSetVolatile(store, 1);
            self.emit_trap()?;
            LLVMPositionBuilderAtEnd(self.builder, ok);
        }
        Ok(())
    }

    fn emit_private_to_generic_int(
        &mut self,
        ptr: LLVMValueRef,
        type_: ast::ScalarType,
    ) -> Result<LLVMValueRef, TranslateError> {
        let generic = get_pointer_type(self.context, ast::StateSpace::Generic)?;
        unsafe {
            let generic_ptr =
                LLVMBuildAddrSpaceCast(self.builder, ptr, generic, LLVM_UNNAMED.as_ptr());
            Ok(LLVMBuildPtrToInt(
                self.builder,
                generic_ptr,
                get_scalar_type(self.context, type_),
                LLVM_UNNAMED.as_ptr(),
            ))
        }
    }

    fn emit_stacksave(
        &mut self,
        type_: ast::ScalarType,
        arguments: ast::StackSaveArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let (base, top) = self.get_or_create_stack()?;
        let i8_type = get_scalar_type(self.context, ast::ScalarType::B8);
        let i32_type = get_scalar_type(self.context, ast::ScalarType::B32);
        let address = unsafe {
            let offset = LLVMBuildLoad2(self.builder, i32_type, top, LLVM_UNNAMED.as_ptr());
            let mut indices = [offset];
            LLVMBuildInBoundsGEP2(
                self.builder,
                i8_type,
                base,
                indices.as_mut_ptr(),
                1,
                LLVM_UNNAMED.as_ptr(),
            )
        };
        let address = self.emit_private_to_generic_int(address, type_)?;
        self.resolver.register(arguments.dst, address);
        Ok(())
    }

    fn emit_stackrestore(
        &mut self,
        type_: ast::ScalarType,
        arguments: ast::StackRestoreArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let src = self.resolver.value(arguments.src)?;
        let (base, top) = self.get_or_create_stack()?;
        let i32_type = get_scalar_type(self.context, ast::ScalarType::B32);
        let base_address = self.emit_private_to_generic_int(base, type_)?;
        unsafe {
            let offset = LLVMBuildSub(self.builder, src, base_address, LLVM_UNNAMED.as_ptr());
            let offset = LLVMBuildTruncOrBitCast(self.builder, offset, i32_type, LLVM_UNNAMED.as_ptr());
            LLVMBuildStore(self.builder, offset, top);
        }
        Ok(())
    }

    // Video instructions follow the semantics of the PTX ISA: the operand
    // parts are sign- or zero-extended and computed in a wider type

    // Bits offset..offset+width of an i32, extended to `to` by `type_`
    fn emit_video_extract(
        &mut self,
        value: LLVMValueRef,
        type_: ast::ScalarType,
        offset: u32,
        width: u32,
        to: LLVMTypeRef,
    ) -> LLVMValueRef {
        let b = self.builder;
        unsafe {
            let i32_type = LLVMInt32TypeInContext(self.context);
            let shifted = if offset == 0 {
                value
            } else {
                LLVMBuildLShr(b, value, LLVMConstInt(i32_type, offset as u64, 0), LLVM_UNNAMED.as_ptr())
            };
            let part = if width == 32 {
                shifted
            } else {
                LLVMBuildTrunc(
                    b,
                    shifted,
                    LLVMIntTypeInContext(self.context, width),
                    LLVM_UNNAMED.as_ptr(),
                )
            };
            if LLVMGetIntTypeWidth(to) == width {
                part
            } else if type_.kind() == ast::ScalarKind::Signed {
                // Extended in two steps, separated by a freeze: the AGX
                // backend cannot select the sign extension of a loaded byte
                // to 64 bits that instruction selection combines them into
                let part = if width < 32 && LLVMGetIntTypeWidth(to) > 32 {
                    let part = LLVMBuildSExt(b, part, i32_type, LLVM_UNNAMED.as_ptr());
                    LLVMBuildFreeze(b, part, LLVM_UNNAMED.as_ptr())
                } else {
                    part
                };
                LLVMBuildSExt(b, part, to, LLVM_UNNAMED.as_ptr())
            } else {
                LLVMBuildZExt(b, part, to, LLVM_UNNAMED.as_ptr())
            }
        }
    }

    fn emit_video_scalar_operand(
        &mut self,
        value: LLVMValueRef,
        type_: ast::ScalarType,
        selector: Option<ast::VideoSelector>,
        to: LLVMTypeRef,
    ) -> LLVMValueRef {
        match selector {
            Some(sel) => {
                let width = sel.width();
                self.emit_video_extract(value, type_, sel.digits[0] as u32 * width, width, to)
            }
            None => self.emit_video_extract(value, type_, 0, 32, to),
        }
    }

    fn emit_int_intrinsic(
        &mut self,
        name: &str,
        type_: LLVMTypeRef,
        args: &[LLVMValueRef],
    ) -> LLVMValueRef {
        let bits = unsafe { LLVMGetIntTypeWidth(type_) };
        let name = format!("{name}.i{bits}\0");
        let mut params = vec![type_; args.len()];
        if name.starts_with("llvm.abs.") || name.starts_with("llvm.ctlz.") || name.starts_with("llvm.cttz.") {
            params[1] = unsafe { LLVMInt1TypeInContext(self.context) };
        }
        unsafe {
            let fn_type = LLVMFunctionType(type_, params.as_mut_ptr(), params.len() as u32, 0);
            let mut fn_ = LLVMGetNamedFunction(self.module, name.as_ptr().cast());
            if fn_ == ptr::null_mut() {
                fn_ = LLVMAddFunction(self.module, name.as_ptr().cast(), fn_type);
            }
            let mut args = args.to_vec();
            LLVMBuildCall2(
                self.builder,
                fn_type,
                fn_,
                args.as_mut_ptr(),
                args.len() as u32,
                LLVM_UNNAMED.as_ptr(),
            )
        }
    }

    // Clamps to the range of a `width`-bit integer, signed or unsigned
    fn emit_video_saturate(
        &mut self,
        value: LLVMValueRef,
        width: u32,
        signed: bool,
    ) -> LLVMValueRef {
        let type_ = unsafe { LLVMTypeOf(value) };
        let (low, high) = if signed {
            (-(1i64 << (width - 1)), (1i64 << (width - 1)) - 1)
        } else {
            (0, ((1u64 << width) - 1) as i64)
        };
        let low = unsafe { LLVMConstInt(type_, low as u64, 1) };
        let high = unsafe { LLVMConstInt(type_, high as u64, 1) };
        let value = self.emit_int_intrinsic("llvm.smax", type_, &[value, low]);
        self.emit_int_intrinsic("llvm.smin", type_, &[value, high])
    }

    // The primary operation on extended operands
    fn emit_video_op(
        &mut self,
        op: ast::VideoOp,
        clamp: bool,
        a: LLVMValueRef,
        b: LLVMValueRef,
    ) -> LLVMValueRef {
        let builder = self.builder;
        let type_ = unsafe { LLVMTypeOf(a) };
        let k = |v: i64| unsafe { LLVMConstInt(type_, v as u64, 1) };
        unsafe {
            match op {
                ast::VideoOp::Add => LLVMBuildAdd(builder, a, b, LLVM_UNNAMED.as_ptr()),
                ast::VideoOp::Sub => LLVMBuildSub(builder, a, b, LLVM_UNNAMED.as_ptr()),
                ast::VideoOp::Absdiff => {
                    let diff = LLVMBuildSub(builder, a, b, LLVM_UNNAMED.as_ptr());
                    let false_ = LLVMConstInt(LLVMInt1TypeInContext(self.context), 0, 0);
                    self.emit_int_intrinsic("llvm.abs", type_, &[diff, false_])
                }
                ast::VideoOp::Min => self.emit_int_intrinsic("llvm.smin", type_, &[a, b]),
                ast::VideoOp::Max => self.emit_int_intrinsic("llvm.smax", type_, &[a, b]),
                ast::VideoOp::Shl | ast::VideoOp::Shr => {
                    // The shift amount is the low 32 bits of b, unsigned:
                    // clamped to 32 or taken modulo 32
                    let i32_type = LLVMInt32TypeInContext(self.context);
                    let amount = LLVMBuildTrunc(builder, b, i32_type, LLVM_UNNAMED.as_ptr());
                    let amount = if clamp {
                        self.emit_int_intrinsic(
                            "llvm.umin",
                            i32_type,
                            &[amount, LLVMConstInt(i32_type, 32, 0)],
                        )
                    } else {
                        LLVMBuildAnd(builder, amount, LLVMConstInt(i32_type, 31, 0), LLVM_UNNAMED.as_ptr())
                    };
                    let amount = LLVMBuildZExt(builder, amount, type_, LLVM_UNNAMED.as_ptr());
                    if op == ast::VideoOp::Shl {
                        LLVMBuildShl(builder, a, amount, LLVM_UNNAMED.as_ptr())
                    } else {
                        LLVMBuildAShr(builder, a, amount, LLVM_UNNAMED.as_ptr())
                    }
                }
                ast::VideoOp::Avrg => {
                    // (a + b + 1) >> 1 for non-negative sums, (a + b) >> 1 otherwise
                    let sum = LLVMBuildAdd(builder, a, b, LLVM_UNNAMED.as_ptr());
                    let non_negative = LLVMBuildICmp(
                        builder,
                        LLVMIntPredicate::LLVMIntSGE,
                        sum,
                        k(0),
                        LLVM_UNNAMED.as_ptr(),
                    );
                    let round = LLVMBuildZExt(builder, non_negative, type_, LLVM_UNNAMED.as_ptr());
                    let sum = LLVMBuildAdd(builder, sum, round, LLVM_UNNAMED.as_ptr());
                    LLVMBuildAShr(builder, sum, k(1), LLVM_UNNAMED.as_ptr())
                }
                ast::VideoOp::Set(cmp) => {
                    let predicate = match cmp {
                        ast::VideoCompare::Eq => LLVMIntPredicate::LLVMIntEQ,
                        ast::VideoCompare::Ne => LLVMIntPredicate::LLVMIntNE,
                        ast::VideoCompare::Lt => LLVMIntPredicate::LLVMIntSLT,
                        ast::VideoCompare::Le => LLVMIntPredicate::LLVMIntSLE,
                        ast::VideoCompare::Gt => LLVMIntPredicate::LLVMIntSGT,
                        ast::VideoCompare::Ge => LLVMIntPredicate::LLVMIntSGE,
                    };
                    let result = LLVMBuildICmp(builder, predicate, a, b, LLVM_UNNAMED.as_ptr());
                    LLVMBuildZExt(builder, result, type_, LLVM_UNNAMED.as_ptr())
                }
            }
        }
    }

    // Replaces bits position..position+width of `into` with the low bits of
    // `value` (both i32)
    fn emit_video_merge(
        &mut self,
        into: LLVMValueRef,
        value: LLVMValueRef,
        position: u32,
        width: u32,
    ) -> LLVMValueRef {
        let b = self.builder;
        unsafe {
            let i32_type = LLVMInt32TypeInContext(self.context);
            let mask = (((1u64 << width) - 1) << position) & 0xffff_ffff;
            let kept = LLVMBuildAnd(b, into, LLVMConstInt(i32_type, !mask & 0xffff_ffff, 0), LLVM_UNNAMED.as_ptr());
            let part = LLVMBuildShl(b, value, LLVMConstInt(i32_type, position as u64, 0), LLVM_UNNAMED.as_ptr());
            let part = LLVMBuildAnd(b, part, LLVMConstInt(i32_type, mask, 0), LLVM_UNNAMED.as_ptr());
            LLVMBuildOr(b, kept, part, LLVM_UNNAMED.as_ptr())
        }
    }

    fn emit_video_scalar(
        &mut self,
        data: ast::VideoScalarDetails,
        arguments: ast::VideoScalarArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let i32_type = get_scalar_type(self.context, ast::ScalarType::B32);
        let i64_type = get_scalar_type(self.context, ast::ScalarType::B64);
        let a = self.resolver.value(arguments.src1)?;
        let b = self.resolver.value(arguments.src2)?;
        let c = match arguments.src3 {
            Some(src3) => Some(self.resolver.value(src3)?),
            None => None,
        };
        let a = self.emit_video_scalar_operand(a, data.atype, data.asel, i64_type);
        let b = self.emit_video_scalar_operand(b, data.btype, data.bsel, i64_type);
        let mut result = self.emit_video_op(data.op, data.clamp, a, b);
        let dtype_signed = data.dtype.kind() == ast::ScalarKind::Signed;
        let result = if let Some(dsel) = data.dsel {
            // Merge into a byte or half-word of c
            let width = dsel.width();
            if data.saturate {
                result = self.emit_video_saturate(result, width, dtype_signed);
            }
            let result = unsafe { LLVMBuildTrunc(self.builder, result, i32_type, LLVM_UNNAMED.as_ptr()) };
            let c = c.ok_or_else(error_unreachable)?;
            self.emit_video_merge(c, result, dsel.digits[0] as u32 * width, width)
        } else {
            if data.saturate {
                result = self.emit_video_saturate(result, 32, dtype_signed);
            }
            if let (Some(op), Some(c)) = (data.secondary, c) {
                let c = self.emit_video_extract(c, data.dtype, 0, 32, i64_type);
                result = match op {
                    ast::VideoSecondaryOp::Add => unsafe {
                        LLVMBuildAdd(self.builder, result, c, LLVM_UNNAMED.as_ptr())
                    },
                    ast::VideoSecondaryOp::Min => {
                        self.emit_int_intrinsic("llvm.smin", i64_type, &[result, c])
                    }
                    ast::VideoSecondaryOp::Max => {
                        self.emit_int_intrinsic("llvm.smax", i64_type, &[result, c])
                    }
                };
            }
            unsafe { LLVMBuildTrunc(self.builder, result, i32_type, LLVM_UNNAMED.as_ptr()) }
        };
        self.resolver.register(arguments.dst, result);
        Ok(())
    }

    fn emit_video_simd(
        &mut self,
        data: ast::VideoSimdDetails,
        arguments: ast::VideoSimdArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let i32_type = get_scalar_type(self.context, ast::ScalarType::B32);
        let a = self.resolver.value(arguments.src1)?;
        let b = self.resolver.value(arguments.src2)?;
        let c = self.resolver.value(arguments.src3)?;
        let lanes = data.lanes as u32;
        let width = 32 / lanes;
        // Selector digits are written from the most significant lane down
        let element = |sel: Option<ast::VideoSelector>, lane: u32, default_base: u32| match sel {
            Some(sel) => sel.digits[(lanes - 1 - lane) as usize] as u32,
            None => default_base + lane,
        };
        let in_mask = |lane: u32| match data.mask {
            Some(mask) => mask.digits().contains(&(lane as u8)),
            None => true,
        };
        let mut result = if data.add {
            c
        } else {
            unsafe { LLVMConstInt(i32_type, 0, 0) }
        };
        for lane in 0..lanes {
            let operand = |this: &mut Self, sel, base, type_| {
                let index = element(sel, lane, base);
                let (source, index) = if index < lanes { (a, index) } else { (b, index - lanes) };
                this.emit_video_extract(source, type_, index * width, width, i32_type)
            };
            let va = operand(self, data.asel, 0, data.atype);
            let vb = operand(self, data.bsel, lanes, data.btype);
            if in_mask(lane) {
                let mut t = self.emit_video_op(data.op, true, va, vb);
                if data.saturate {
                    t = self.emit_video_saturate(
                        t,
                        width,
                        data.dtype.kind() == ast::ScalarKind::Signed,
                    );
                }
                result = if data.add {
                    unsafe { LLVMBuildAdd(self.builder, result, t, LLVM_UNNAMED.as_ptr()) }
                } else {
                    self.emit_video_merge(result, t, lane * width, width)
                };
            } else if !data.add {
                let kept = unsafe {
                    LLVMBuildLShr(
                        self.builder,
                        c,
                        LLVMConstInt(i32_type, (lane * width) as u64, 0),
                        LLVM_UNNAMED.as_ptr(),
                    )
                };
                result = self.emit_video_merge(result, kept, lane * width, width);
            }
        }
        self.resolver.register(arguments.dst, result);
        Ok(())
    }

    // vmad: (a * b + c) >> shift, where negated operands are complemented
    // and one is added (PTX ISA, also .po)
    fn emit_vmad(
        &mut self,
        data: ast::VmadDetails,
        arguments: ast::VmadArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let i32_type = get_scalar_type(self.context, ast::ScalarType::B32);
        let i64_type = get_scalar_type(self.context, ast::ScalarType::B64);
        let builder = self.builder;
        let k = |v: i64| unsafe { LLVMConstInt(i64_type, v as u64, 1) };
        let a = self.resolver.value(arguments.src1)?;
        let b = self.resolver.value(arguments.src2)?;
        let c = self.resolver.value(arguments.src3)?;
        let a = self.emit_video_scalar_operand(a, data.atype, data.asel, i64_type);
        let b = self.emit_video_scalar_operand(b, data.btype, data.bsel, i64_type);
        let product_negated = !data.plus_one && data.negate_a != data.negate_b;
        let signed = data.atype.kind() == ast::ScalarKind::Signed
            || data.btype.kind() == ast::ScalarKind::Signed
            || data.negate_a != data.negate_b
            || data.negate_c;
        let complement_c = !data.plus_one && data.negate_a == data.negate_b && data.negate_c;
        let plus_one = data.plus_one || data.negate_a != data.negate_b || data.negate_c;
        unsafe {
            // Exact in 64 bits, except for the product of two .u32: it can
            // reach 2^64, which only matters when it is negated and saturated
            let product = LLVMBuildMul(builder, a, b, LLVM_UNNAMED.as_ptr());
            let product_too_big = LLVMBuildICmp(
                builder,
                LLVMIntPredicate::LLVMIntSLT,
                product,
                k(0),
                LLVM_UNNAMED.as_ptr(),
            );
            let product = if product_negated {
                LLVMBuildXor(builder, product, k(-1), LLVM_UNNAMED.as_ptr())
            } else {
                product
            };
            let c = if complement_c {
                LLVMBuildXor(builder, c, LLVMConstInt(i32_type, u32::MAX as u64, 0), LLVM_UNNAMED.as_ptr())
            } else {
                c
            };
            let c = if signed {
                LLVMBuildSExt(builder, c, i64_type, LLVM_UNNAMED.as_ptr())
            } else {
                LLVMBuildZExt(builder, c, i64_type, LLVM_UNNAMED.as_ptr())
            };
            let one = k(plus_one as i64);
            let sum = if data.saturate {
                let add = if signed { "llvm.sadd.sat" } else { "llvm.uadd.sat" };
                let sum = self.emit_int_intrinsic(add, i64_type, &[product, c]);
                self.emit_int_intrinsic(add, i64_type, &[sum, one])
            } else {
                let sum = LLVMBuildAdd(builder, product, c, LLVM_UNNAMED.as_ptr());
                LLVMBuildAdd(builder, sum, one, LLVM_UNNAMED.as_ptr())
            };
            let sum = if data.shift == 0 {
                sum
            } else if signed {
                LLVMBuildAShr(builder, sum, k(data.shift as i64), LLVM_UNNAMED.as_ptr())
            } else {
                LLVMBuildLShr(builder, sum, k(data.shift as i64), LLVM_UNNAMED.as_ptr())
            };
            let result = if data.saturate {
                let mut result = if signed {
                    self.emit_video_saturate(sum, 32, true)
                } else {
                    self.emit_int_intrinsic("llvm.umin", i64_type, &[sum, k(u32::MAX as i64)])
                };
                let both_unsigned = data.atype.kind() != ast::ScalarKind::Signed
                    && data.btype.kind() != ast::ScalarKind::Signed;
                if product_negated && both_unsigned {
                    result = LLVMBuildSelect(builder, product_too_big, k(i32::MIN as i64), result, LLVM_UNNAMED.as_ptr());
                }
                result
            } else {
                sum
            };
            self.resolver.with_result(arguments.dst, |dst| {
                LLVMBuildTrunc(builder, result, i32_type, dst)
            });
        }
        Ok(())
    }

    // The product is the xor of b << i for each bit i set in a; the high
    // half gets b >> (64 - i)
    fn emit_clmad(
        &mut self,
        high: bool,
        arguments: ast::ClmadArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let a = self.resolver.value(arguments.src1)?;
        let b = self.resolver.value(arguments.src2)?;
        let c = self.resolver.value(arguments.src3)?;
        let i64_type = get_scalar_type(self.context, ast::ScalarType::B64);
        let k = |v: u64| unsafe { LLVMConstInt(i64_type, v, 0) };
        let mut result = c;
        for i in 0..64u64 {
            if high && i == 0 {
                continue;
            }
            unsafe {
                let bit = LLVMBuildAnd(
                    self.builder,
                    LLVMBuildLShr(self.builder, a, k(i), LLVM_UNNAMED.as_ptr()),
                    k(1),
                    LLVM_UNNAMED.as_ptr(),
                );
                // all ones when the bit is set
                let mask = LLVMBuildSub(self.builder, k(0), bit, LLVM_UNNAMED.as_ptr());
                let shifted = if high {
                    LLVMBuildLShr(self.builder, b, k(64 - i), LLVM_UNNAMED.as_ptr())
                } else {
                    LLVMBuildShl(self.builder, b, k(i), LLVM_UNNAMED.as_ptr())
                };
                let term = LLVMBuildAnd(self.builder, shifted, mask, LLVM_UNNAMED.as_ptr());
                result = LLVMBuildXor(self.builder, result, term, LLVM_UNNAMED.as_ptr());
            }
        }
        self.resolver.register(arguments.dst, result);
        Ok(())
    }

    fn emit_packed8(
        &mut self,
        data: ast::Packed8Details,
        arguments: ast::Packed8Args<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let i32_type = get_scalar_type(self.context, ast::ScalarType::B32);
        if let ast::Packed8Op::Set { cmp, lanes16 } = data.op {
            // lane-wise compare, all ones where true
            let (bits, lanes) = if lanes16 { (16, 2) } else { (8, 4) };
            let vec_type = unsafe { LLVMVectorType(LLVMIntTypeInContext(self.context, bits), lanes) };
            let a = self.resolver.value(arguments.src1)?;
            let b = self.resolver.value(arguments.src2.ok_or_else(error_unreachable)?)?;
            use LLVMIntPredicate::*;
            let predicate = match (cmp, data.signed) {
                (ast::PackedCmp::Eq, _) => LLVMIntEQ,
                (ast::PackedCmp::Ne, _) => LLVMIntNE,
                (ast::PackedCmp::Lt, true) => LLVMIntSLT,
                (ast::PackedCmp::Lt, false) => LLVMIntULT,
                (ast::PackedCmp::Le, true) => LLVMIntSLE,
                (ast::PackedCmp::Le, false) => LLVMIntULE,
                (ast::PackedCmp::Gt, true) => LLVMIntSGT,
                (ast::PackedCmp::Gt, false) => LLVMIntUGT,
                (ast::PackedCmp::Ge, true) => LLVMIntSGE,
                (ast::PackedCmp::Ge, false) => LLVMIntUGE,
            };
            unsafe {
                let a = LLVMBuildBitCast(self.builder, a, vec_type, LLVM_UNNAMED.as_ptr());
                let b = LLVMBuildBitCast(self.builder, b, vec_type, LLVM_UNNAMED.as_ptr());
                let mask = LLVMBuildICmp(self.builder, predicate, a, b, LLVM_UNNAMED.as_ptr());
                let lanes = LLVMBuildSExt(self.builder, mask, vec_type, LLVM_UNNAMED.as_ptr());
                self.resolver.with_result(arguments.dst, |dst| LLVMBuildBitCast(self.builder, lanes, i32_type, dst));
            }
            return Ok(());
        }
        let vec_type = unsafe { LLVMVectorType(LLVMInt8TypeInContext(self.context), 4) };
        let to_vec = |this: &mut Self, word: SpirvWord| -> Result<LLVMValueRef, TranslateError> {
            let value = this.resolver.value(word)?;
            Ok(unsafe { LLVMBuildBitCast(this.builder, value, vec_type, LLVM_UNNAMED.as_ptr()) })
        };
        let a = to_vec(self, arguments.src1)?;
        let b = match arguments.src2 {
            Some(src2) => Some(to_vec(self, src2)?),
            None => None,
        };
        let zero = unsafe { LLVMConstNull(vec_type) };
        let intrinsic = |this: &mut Self, name: &str, args: Vec<LLVMValueRef>| {
            let name = format!("{name}.v4i8\0");
            let fn_type = unsafe {
                let mut params = vec![vec_type; args.len()];
                LLVMFunctionType(vec_type, params.as_mut_ptr(), params.len() as u32, 0)
            };
            let mut fn_ = unsafe { LLVMGetNamedFunction(this.module, name.as_ptr().cast()) };
            if fn_ == ptr::null_mut() {
                fn_ = unsafe { LLVMAddFunction(this.module, name.as_ptr().cast(), fn_type) };
            }
            let mut args = args;
            unsafe {
                LLVMBuildCall2(
                    this.builder,
                    fn_type,
                    fn_,
                    args.as_mut_ptr(),
                    args.len() as u32,
                    LLVM_UNNAMED.as_ptr(),
                )
            }
        };
        let signed = data.signed;
        let b_or = |b: Option<LLVMValueRef>| b.ok_or_else(error_unreachable);
        let result = match data.op {
            ast::Packed8Op::Add if data.saturate => {
                let name = if signed { "llvm.sadd.sat" } else { "llvm.uadd.sat" };
                intrinsic(self, name, vec![a, b_or(b)?])
            }
            ast::Packed8Op::Sub if data.saturate => {
                let name = if signed { "llvm.ssub.sat" } else { "llvm.usub.sat" };
                intrinsic(self, name, vec![a, b_or(b)?])
            }
            ast::Packed8Op::Add => unsafe {
                LLVMBuildAdd(self.builder, a, b_or(b)?, LLVM_UNNAMED.as_ptr())
            },
            ast::Packed8Op::Sub => unsafe {
                LLVMBuildSub(self.builder, a, b_or(b)?, LLVM_UNNAMED.as_ptr())
            },
            ast::Packed8Op::Min => {
                let name = if signed { "llvm.smin" } else { "llvm.umin" };
                intrinsic(self, name, vec![a, b_or(b)?])
            }
            ast::Packed8Op::Max => {
                let name = if signed { "llvm.smax" } else { "llvm.umax" };
                intrinsic(self, name, vec![a, b_or(b)?])
            }
            ast::Packed8Op::Neg => unsafe { LLVMBuildSub(self.builder, zero, a, LLVM_UNNAMED.as_ptr()) },
            ast::Packed8Op::Set { .. } => return Err(error_unreachable()),
            ast::Packed8Op::Abs => unsafe {
                // |-128| wraps to -128
                let negated = LLVMBuildSub(self.builder, zero, a, LLVM_UNNAMED.as_ptr());
                let negative = LLVMBuildICmp(
                    self.builder,
                    LLVMIntPredicate::LLVMIntSLT,
                    a,
                    zero,
                    LLVM_UNNAMED.as_ptr(),
                );
                LLVMBuildSelect(self.builder, negative, negated, a, LLVM_UNNAMED.as_ptr())
            },
        };
        let result = if data.relu && signed {
            intrinsic(self, "llvm.smax", vec![result, zero])
        } else {
            result
        };
        self.resolver.with_result(arguments.dst, |dst| unsafe {
            LLVMBuildBitCast(self.builder, result, i32_type, dst)
        });
        Ok(())
    }

    fn emit_isspacep(
        &mut self,
        space: ast::StateSpace,
        arguments: ast::IsspacepArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let src = self.resolver.value(arguments.src)?;
        let i1_type = get_scalar_type(self.context, ast::ScalarType::Pred);
        let i64_type = get_scalar_type(self.context, ast::ScalarType::B64);
        let name = match space {
            ast::StateSpace::Global => "global",
            ast::StateSpace::Shared | ast::StateSpace::SharedCta | ast::StateSpace::SharedCluster => "shared",
            ast::StateSpace::Local => "local",
            ast::StateSpace::Const => "const",
            ast::StateSpace::Param => "param",
            _ => return Err(error_unreachable()),
        };
        if self.target == Target::Agx {
            // Decided after optimization by llvm_zluda, from the address
            // space the address comes from (AGX has no generic addressing
            // of threadgroup and private memory)
            let fn_name = CString::new(format!("__zluda_agx_isspacep_{name}"))
                .map_err(|_| error_unreachable())?;
            let mut fn_ = unsafe { LLVMGetNamedFunction(self.module, fn_name.as_ptr()) };
            let mut arg_type = i64_type;
            let fn_type = unsafe { LLVMFunctionType(i1_type, &mut arg_type, 1, 0) };
            if fn_ == ptr::null_mut() {
                fn_ = unsafe { LLVMAddFunction(self.module, fn_name.as_ptr(), fn_type) };
                for (attr, value) in [("nounwind", 0), ("willreturn", 0), ("memory", 0)] {
                    unsafe {
                        let kind = LLVMGetEnumAttributeKindForName(attr.as_ptr().cast(), attr.len());
                        let attr = LLVMCreateEnumAttribute(self.context, kind, value);
                        LLVMAddAttributeAtIndex(fn_, LLVMAttributeFunctionIndex, attr);
                    }
                }
            }
            let mut args = [src];
            if !matches!(space, ast::StateSpace::Shared | ast::StateSpace::SharedCta | ast::StateSpace::SharedCluster) {
                self.resolver.with_result(arguments.dst, |dst| unsafe {
                    LLVMBuildCall2(self.builder, fn_type, fn_, args.as_mut_ptr(), 1, dst)
                });
                return Ok(());
            }
            // .shared is .shared::cta: also in this CTA's window of a cluster; .shared::cluster in the window of
            // one of the cluster's CTAs
            let in_shared =
                unsafe { LLVMBuildCall2(self.builder, fn_type, fn_, args.as_mut_ptr(), 1, LLVM_UNNAMED.as_ptr()) };
            let helper = if space == ast::StateSpace::SharedCluster {
                c"__zluda_ptx_impl_agx_in_cluster"
            } else {
                c"__zluda_ptx_impl_agx_in_own_cta"
            };
            let own = self.emit_agx_helper(helper, None, Some(ast::ScalarType::Pred), &[src])?;
            let false_ = unsafe { LLVMConstInt(i1_type, 0, 0) };
            self.resolver.with_result(arguments.dst, |dst| unsafe {
                LLVMBuildSelect(self.builder, in_shared, own, false_, dst)
            });
            return Ok(());
        }
        let generic = get_pointer_type(self.context, ast::StateSpace::Generic)?;
        let ptr = unsafe { LLVMBuildIntToPtr(self.builder, src, generic, LLVM_UNNAMED.as_ptr()) };
        let test = |this: &mut Self, intrinsic: &CStr| {
            this.emit_intrinsic(
                intrinsic,
                None,
                vec![&ast::ScalarType::Pred.into()],
                vec![(ptr, generic)],
            )
        };
        let result = match space {
            ast::StateSpace::Shared => test(self, c"llvm.amdgcn.is.shared")?,
            ast::StateSpace::Local => test(self, c"llvm.amdgcn.is.private")?,
            ast::StateSpace::Global => {
                let shared = test(self, c"llvm.amdgcn.is.shared")?;
                let private = test(self, c"llvm.amdgcn.is.private")?;
                unsafe {
                    let either = LLVMBuildOr(self.builder, shared, private, LLVM_UNNAMED.as_ptr());
                    LLVMBuildNot(self.builder, either, LLVM_UNNAMED.as_ptr())
                }
            }
            _ => unsafe { LLVMConstInt(i1_type, 0, 0) },
        };
        self.resolver.register(arguments.dst, result);
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
        // (the 16-bit form has no c; the shift would be out of range)
        let src3_shifted = if input_bit_size == 16 { src3 } else { unsafe {
            LLVMBuildShl(
                self.builder,
                src3,
                LLVMConstInt(dest_ty, input_bit_size as u64 * 2, 0),
                LLVM_UNNAMED.as_ptr(),
            )
        } };
        let src1_src2 = unsafe {
            LLVMBuildOr(
                self.builder,
                src1_shifted,
                src2_saturated,
                LLVM_UNNAMED.as_ptr(),
            )
        };
        if input_bit_size == 16 {
            self.resolver.register(arguments.dst, src1_src2);
            return Ok(());
        }
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
        let resize_fn = if to.layout().size() < from.layout().size() {
            LLVMBuildTrunc
        } else if from.kind() == ptx_parser::ScalarKind::Signed {
            LLVMBuildSExtOrBitCast
        } else {
            LLVMBuildZExtOrBitCast
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
                // a destination at least as wide as the source range needs no clamp (the
                // constant would not even fit the source type)
                let from_bits = from.layout().size() as u32 * 8;
                if from_bits < 64 && max_value >= (1u64 << from_bits) - 1 {
                    return self.resolver.value(src);
                }
                let intrinsic = format!("llvm.umin.{}\0", LLVMTypeDisplay(from));
                let max = unsafe { LLVMConstInt(from_llvm, max_value, 0) };
                let clamped = self.emit_intrinsic(
                    unsafe { CStr::from_bytes_with_nul_unchecked(intrinsic.as_bytes()) },
                    None,
                    vec![&from.into()],
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
                    vec![&from.into()],
                    vec![(self.resolver.value(src)?, from_llvm), (min, from_llvm)],
                )?;
                let clamped = self.emit_intrinsic(
                    unsafe { CStr::from_bytes_with_nul_unchecked(min_intrinsic.as_bytes()) },
                    None,
                    vec![&from.into()],
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
        // v_cvt_* instruction get special treatment. AGX has no such guarantee (the out-of-range
        // fptosi folds to anything), so it always takes the .sat intrinsics
        let is_saturating_cast = self.target != Target::Agx && match (to, from) {
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
            let src =
                dst_int_rounded.unwrap_or_else(|| self.resolver.value(arguments.src).unwrap());
            let poisoned_dst = if signed_cast {
                self.fp.fptosi(self.resolver, to, None, from, src)?
            } else {
                self.fp.fptoui(self.resolver, to, None, from, src)?
            };
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
            // A NaN gives 0 (the .sat intrinsics) unless the source is .f64 or the destination 64-bit: then
            // 1 << (BitWidth(dst) - 1) (PTX ISA 9.7.9.22, Integer Notes)
            let nan_top_bit = self.target == Target::Agx && (from == ast::ScalarType::F64 || to.size_of() == 8);
            let converted = self.emit_intrinsic(
                unsafe { CStr::from_bytes_with_nul_unchecked(cast_intrinsic.as_bytes()) },
                (!nan_top_bit).then_some(arguments.dst),
                vec![&to.into()],
                vec![(src, get_scalar_type(self.context, from))],
            )?;
            if nan_top_bit {
                let to_type = get_scalar_type(self.context, to);
                let top = unsafe { LLVMConstInt(to_type, 1u64 << (to.size_of() as u32 * 8 - 1), 0) };
                let is_nan = unsafe {
                    LLVMBuildFCmp(self.builder, LLVMRealPredicate::LLVMRealUNO, src, src, LLVM_UNNAMED.as_ptr())
                };
                self.resolver.with_result(arguments.dst, |dst| unsafe {
                    LLVMBuildSelect(self.builder, is_nan, top, converted, dst)
                });
            }
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
        let name = match rounding {
            ptx_parser::RoundingMode::NearestEven => "roundeven",
            ptx_parser::RoundingMode::Zero => {
                // cvt has round-to-zero semantics
                if will_saturate_with_cvt {
                    return Ok(None);
                } else {
                    "trunc"
                }
            }
            ptx_parser::RoundingMode::NegativeInf => "floor",
            ptx_parser::RoundingMode::PositiveInf => "ceil",
        };
        let (prefix, metadata): (_, &[&str]) = match self.fp.mode {
            FloatingPointMode::Normal => ("llvm", &[]),
            FloatingPointMode::Constrained => {
                ("llvm.experimental.constrained", &["fpexcept.ignore"])
            }
        };
        let intrinsic = format!("{}.{}.{}\0", prefix, name, LLVMTypeDisplay(from));
        let rounded_float = self.emit_intrinsic_with_metadata(
            unsafe { CStr::from_bytes_with_nul_unchecked(intrinsic.as_bytes()) },
            None,
            vec![&from.into()],
            vec![(
                self.resolver.value(arguments.src)?,
                get_scalar_type(self.context, from),
            )],
            metadata,
        )?;
        Ok(Some(rounded_float))
    }

    fn emit_cvt_int_to_float(
        &mut self,
        from: ptx_parser::ScalarType,
        to: ptx_parser::ScalarType,
        rounding: ast::RoundingMode,
        arguments: ptx_parser::CvtArgs<SpirvWord>,
        signed: bool,
    ) -> Result<(), TranslateError> {
        let src = self.resolver.value(arguments.src)?;
        if self.is_bf16_directed_rounding(to, rounding) {
            let wide = if signed {
                self.fp
                    .sitofp(self.resolver, ast::ScalarType::F32, None, from, src)?
            } else {
                self.fp
                    .uitofp(self.resolver, ast::ScalarType::F32, None, from, src)?
            };
            let result = self.emit_f32_to_bf16_directed(rounding, wide);
            self.resolver.register(arguments.dst, result);
            return Ok(());
        }
        let dst = Some(arguments.dst);
        if signed {
            self.fp.sitofp(self.resolver, to, dst, from, src)?;
        } else {
            self.fp.uitofp(self.resolver, to, dst, from, src)?;
        }
        Ok(())
    }

    fn emit_rsqrt(
        &mut self,
        data: ptx_parser::TypeFtz,
        arguments: ptx_parser::RsqrtArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let type_ = get_scalar_type(self.context, data.type_);
        if self.target == Target::Agx && data.type_ == ast::ScalarType::F64 {
            // No f64 hardware: 1 / sqrt(x) with the software f64 square root and division, which is more
            // accurate than PTX's approximation requires.
            let src = self.resolver.value(arguments.src)?;
            let root = self.emit_intrinsic(c"llvm.sqrt.f64", None, vec![&data.type_.into()], vec![(src, type_)])?;
            let one = unsafe { LLVMConstReal(type_, 1.0) };
            self.fp.fdiv(self.resolver, data.type_, arguments.dst, one, root)?;
            return Ok(());
        }
        let intrinsic = match (self.target, data.type_, self.fp.mode) {
            (Target::Agx, ast::ScalarType::F32, _) => c"llvm.agx.rsqrt.f32",
            (Target::Agx, _, _) => return Err(error_unreachable()),
            (_, ast::ScalarType::F32, FloatingPointMode::Normal) => c"llvm.amdgcn.rsq.f32",
            (_, ast::ScalarType::F32, FloatingPointMode::Constrained) => {
                c"llvm.amdgcn.constrained.rsq.f32"
            }
            (_, ast::ScalarType::F64, FloatingPointMode::Normal) => c"llvm.amdgcn.rsq.f64",
            (_, ast::ScalarType::F64, FloatingPointMode::Constrained) => {
                c"llvm.amdgcn.constrained.rsq.f64"
            }
            _ => return Err(error_unreachable()),
        };
        self.emit_intrinsic(
            intrinsic,
            Some(arguments.dst),
            vec![&data.type_.into()],
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
        let mode = self.fp.mode;
        let (intrinsic, metadata): (_, &[&str]) = match (data.type_, data.kind, mode) {
            (ast::ScalarType::F32, ast::RcpKind::Approx, _) if self.target == Target::Agx => {
                (c"llvm.sqrt.f32", &[])
            }
            (ast::ScalarType::F32, ast::RcpKind::Approx, FloatingPointMode::Normal) => {
                (c"llvm.amdgcn.sqrt.f32", &[])
            }
            (ast::ScalarType::F32, ast::RcpKind::Approx, FloatingPointMode::Constrained) => {
                (c"llvm.amdgcn.constrained.sqrt.f32", &[])
            }
            (ast::ScalarType::F32, ast::RcpKind::Compliant(..), FloatingPointMode::Normal) => {
                (c"llvm.sqrt.f32", &[])
            }
            (ast::ScalarType::F32, ast::RcpKind::Compliant(..), FloatingPointMode::Constrained) => {
                (
                    c"llvm.experimental.constrained.sqrt.f32",
                    &["round.dynamic", "fpexcept.ignore"],
                )
            }
            (ast::ScalarType::F64, ast::RcpKind::Compliant(..), FloatingPointMode::Normal) => {
                (c"llvm.sqrt.f64", &[])
            }
            (ast::ScalarType::F64, ast::RcpKind::Compliant(..), FloatingPointMode::Constrained) => {
                (
                    c"llvm.experimental.constrained.sqrt.f64",
                    &["round.dynamic", "fpexcept.ignore"],
                )
            }
            _ => return Err(error_unreachable()),
        };
        self.emit_intrinsic_with_metadata(
            intrinsic,
            Some(arguments.dst),
            vec![&data.type_.into()],
            vec![(self.resolver.value(arguments.src)?, type_)],
            metadata,
        )?;
        Ok(())
    }

    fn emit_rcp(
        &mut self,
        data: ptx_parser::RcpData,
        arguments: ptx_parser::RcpArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let type_ = get_scalar_type(self.context, data.type_);
        let intrinsic = match (data.type_, data.kind, self.fp.mode) {
            (ast::ScalarType::F32, ast::RcpKind::Approx, _) if self.target == Target::Agx => {
                c"llvm.agx.rcp.f32"
            }
            (ast::ScalarType::F64, _, _) if self.target == Target::Agx => {
                // 1 / x with the software f64 division (exact, so also rcp.rn.f64)
                let src = self.resolver.value(arguments.src)?;
                let one = unsafe { LLVMConstReal(type_, 1.0) };
                self.fp.fdiv(self.resolver, data.type_, arguments.dst, one, src)?;
                return Ok(());
            }
            (_, _, _) if self.target == Target::Agx => {
                return Err(TranslateError::Unsupported("rcp of this type is not supported on AGX".into()))
            }
            (ast::ScalarType::F32, ast::RcpKind::Approx, FloatingPointMode::Normal) => {
                c"llvm.amdgcn.rcp.f32"
            }
            (ast::ScalarType::F32, ast::RcpKind::Approx, FloatingPointMode::Constrained) => {
                c"llvm.amdgcn.constrained.rcp.f32"
            }
            (ast::ScalarType::F64, ast::RcpKind::Approx, FloatingPointMode::Normal) => {
                c"llvm.amdgcn.rcp.f64"
            }
            (ast::ScalarType::F64, ast::RcpKind::Approx, FloatingPointMode::Constrained) => {
                c"llvm.amdgcn.constrained.rcp.f64"
            }
            _ => return Err(error_unreachable()),
        };
        self.emit_intrinsic(
            intrinsic,
            Some(arguments.dst),
            vec![&data.type_.into()],
            vec![(self.resolver.value(arguments.src)?, type_)],
        )?;
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
            vec![&ast::Type::Scalar(ptx_parser::ScalarType::B32)],
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
                *const std::ffi::c_char,
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
            *const std::ffi::c_char,
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
        let agx_name;
        let intrinsic = match (data.type_, self.fp.mode) {
            (_, _) if self.target == Target::Agx => {
                let suffix = match data.type_ {
                    ast::ScalarType::F16 => "f16",
                    ast::ScalarType::F32 => "f32",
                    ast::ScalarType::F16x2 => "v2f16",
                    ast::ScalarType::BF16 => "bf16",
                    ast::ScalarType::BF16x2 => "v2bf16",
                    _ => return Err(error_unreachable()),
                };
                agx_name = CString::new(format!("llvm.exp2.{suffix}")).map_err(|_| error_unreachable())?;
                agx_name.as_c_str()
            }
            (ast::ScalarType::F16, FloatingPointMode::Normal) => c"llvm.amdgcn.exp2.f16",
            (ast::ScalarType::F16, FloatingPointMode::Constrained) => {
                c"llvm.amdgcn.constrained.exp2.f16"
            }
            (ast::ScalarType::F32, FloatingPointMode::Normal) => c"llvm.amdgcn.exp2.f32",
            (ast::ScalarType::F32, FloatingPointMode::Constrained) => {
                c"llvm.amdgcn.constrained.exp2.f32"
            }
            _ => return Err(error_unreachable()),
        };
        let result = self.emit_intrinsic(
            intrinsic,
            None,
            vec![&data.type_.into()],
            vec![(
                self.resolver.value(arguments.src)?,
                get_scalar_type(self.context, data.type_),
            )],
        )?;
        // the half formats keep subnormal results
        let result = if self.target == Target::Agx && data.flush_to_zero == Some(true) {
            self.emit_flush_subnormal(data.type_, result)
        } else {
            result
        };
        self.resolver.register(arguments.dst, result);
        Ok(())
    }

    fn emit_lg2(
        &mut self,
        _data: ptx_parser::FlushToZero,
        arguments: ptx_parser::Lg2Args<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let intrinsic = match self.fp.mode {
            _ if self.target == Target::Agx => c"llvm.log2.f32",
            FloatingPointMode::Normal => c"llvm.amdgcn.log.f32",
            FloatingPointMode::Constrained => c"llvm.amdgcn.constrained.log.f32",
        };
        self.emit_intrinsic(
            intrinsic,
            Some(arguments.dst),
            vec![&ast::ScalarType::F32.into()],
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
        if self.target == Target::Agx {
            let signed = data.kind() == ptx_parser::ScalarKind::Signed;
            return self.emit_defined_int_div(arguments.dst, signed, true, src1, src2);
        }
        self.resolver.with_result(arguments.dst, |dst_name| unsafe {
            llvm_fn(self.builder, src1, src2, dst_name)
        });
        Ok(())
    }

    fn emit_bar_warp(&mut self) -> Result<(), TranslateError> {
        // SIMD-groups execute in lockstep on AGX.
        if self.target == Target::Agx {
            return Ok(());
        }
        self.emit_intrinsic(c"llvm.amdgcn.wave.barrier", None, vec![], vec![])?;
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
        let population = self.emit_intrinsic(
            intrinsic,
            None,
            vec![&type_.into()],
            vec![(self.resolver.value(arguments.src)?, llvm_type)],
        )?;
        let i32_llvm = get_scalar_type(self.context, ast::ScalarType::B32);
        self.resolver.with_result(arguments.dst, |dst_name| unsafe {
            LLVMBuildTruncOrBitCast(self.builder, population, i32_llvm, dst_name)
        });
        Ok(())
    }

    fn emit_min(
        &mut self,
        data: ptx_parser::MinMaxDetails,
        arguments: ptx_parser::MinArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        self.emit_min_max(true, data, arguments.dst, arguments.src1, arguments.src2, arguments.src3)
    }

    fn emit_max(
        &mut self,
        data: ptx_parser::MinMaxDetails,
        arguments: ptx_parser::MaxArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        self.emit_min_max(false, data, arguments.dst, arguments.src1, arguments.src2, arguments.src3)
    }

    fn emit_min_max(
        &mut self,
        is_min: bool,
        data: ptx_parser::MinMaxDetails,
        dst: SpirvWord,
        src1: SpirvWord,
        src2: SpirvWord,
        src3: Option<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let llvm_prefix = match (data, is_min) {
            (ptx_parser::MinMaxDetails::Signed(..) | ptx_parser::MinMaxDetails::SignedRelu(..), true) => "llvm.smin",
            (ptx_parser::MinMaxDetails::Signed(..) | ptx_parser::MinMaxDetails::SignedRelu(..), false) => "llvm.smax",
            (ptx_parser::MinMaxDetails::Unsigned(..), true) => "llvm.umin",
            (ptx_parser::MinMaxDetails::Unsigned(..), false) => "llvm.umax",
            (ptx_parser::MinMaxDetails::Float(ptx_parser::MinMaxFloat { nan: false, .. }), true) => {
                "llvm.minimumnum"
            }
            (ptx_parser::MinMaxDetails::Float(ptx_parser::MinMaxFloat { nan: false, .. }), false) => {
                "llvm.maximumnum"
            }
            (ptx_parser::MinMaxDetails::Float(ptx_parser::MinMaxFloat { nan: true, .. }), true) => {
                "llvm.minimum"
            }
            (ptx_parser::MinMaxDetails::Float(ptx_parser::MinMaxFloat { nan: true, .. }), false) => {
                "llvm.maximum"
            }
        };
        let type_ = data.type_();
        let intrinsic = format!("{}.{}\0", llvm_prefix, LLVMTypeDisplay(type_));
        let llvm_type = get_scalar_type(self.context, type_);
        let mut a = self.resolver.value(src1)?;
        let mut b = self.resolver.value(src2)?;
        let xorsign_abs = matches!(
            data,
            ptx_parser::MinMaxDetails::Float(ptx_parser::MinMaxFloat {
                xorsign_abs: true,
                ..
            })
        );
        let abs = matches!(data, ptx_parser::MinMaxDetails::Float(ptx_parser::MinMaxFloat { abs: true, .. }));
        let (original_a, original_b) = (a, b);
        if xorsign_abs || abs {
            let fabs = format!("llvm.fabs.{}\0", LLVMTypeDisplay(type_));
            let fabs = unsafe { CStr::from_bytes_with_nul_unchecked(fabs.as_bytes()) };
            a = self.emit_intrinsic(fabs, None, vec![&type_.into()], vec![(a, llvm_type)])?;
            b = self.emit_intrinsic(fabs, None, vec![&type_.into()], vec![(b, llvm_type)])?;
        }
        let result = self.emit_intrinsic(
            unsafe { CStr::from_bytes_with_nul_unchecked(intrinsic.as_bytes()) },
            None,
            vec![&type_.into()],
            vec![(a, llvm_type), (b, llvm_type)],
        )?;
        // A NaN result of f64 is the first NaN operand, quieted ("NaN payloads are supported for double-precision
        // instructions", PTX ISA 9.7.3): explicit, as the intrinsics are commutative and their operands may swap
        let result = match data {
            ptx_parser::MinMaxDetails::Float(ptx_parser::MinMaxFloat { nan, .. })
                if type_ == ast::ScalarType::F64 && !xorsign_abs && self.target == Target::Agx =>
            unsafe {
                let src_b = b;
                let b = self.builder;
                let i64_type = get_scalar_type(self.context, ast::ScalarType::B64);
                let quiet = |x| {
                    let bits = LLVMBuildBitCast(b, x, i64_type, LLVM_UNNAMED.as_ptr());
                    let bits = LLVMBuildOr(b, bits, LLVMConstInt(i64_type, 1 << 51, 0), LLVM_UNNAMED.as_ptr());
                    LLVMBuildBitCast(b, bits, llvm_type, LLVM_UNNAMED.as_ptr())
                };
                let a_nan = LLVMBuildFCmp(b, LLVMRealPredicate::LLVMRealUNO, a, a, LLVM_UNNAMED.as_ptr());
                let b_nan = LLVMBuildFCmp(b, LLVMRealPredicate::LLVMRealUNO, src_b, src_b, LLVM_UNNAMED.as_ptr());
                let first = LLVMBuildSelect(b, a_nan, quiet(a), quiet(src_b), LLVM_UNNAMED.as_ptr());
                let any = if nan {
                    LLVMBuildOr(b, a_nan, b_nan, LLVM_UNNAMED.as_ptr())
                } else {
                    LLVMBuildAnd(b, a_nan, b_nan, LLVM_UNNAMED.as_ptr())
                };
                LLVMBuildSelect(b, any, first, result, LLVM_UNNAMED.as_ptr())
            },
            ptx_parser::MinMaxDetails::Float(ptx_parser::MinMaxFloat { flush_to_zero, .. }) => {
                // min and max of the flushed inputs are the flushed min and max
                let result = if self.explicit_ftz(flush_to_zero) {
                    self.emit_flush_subnormal(type_, result)
                } else {
                    result
                };
                self.emit_canonical_nan(type_, result)
            }
            _ => result,
        };
        let result = if xorsign_abs {
            self.emit_xor_sign(type_, result, original_a, original_b)?
        } else {
            result
        };
        // .relu of the signed integer forms
        let result = if let ptx_parser::MinMaxDetails::SignedRelu(t) = data {
            let smax = format!("llvm.smax.{}\0", LLVMTypeDisplay(t));
            let zero = unsafe { LLVMConstNull(llvm_type) };
            self.emit_intrinsic(
                unsafe { CStr::from_bytes_with_nul_unchecked(smax.as_bytes()) },
                None,
                vec![&type_.into()],
                vec![(result, llvm_type), (zero, llvm_type)],
            )?
        } else {
            result
        };
        // the three-input form: op(op(a, b), c)
        let result = match src3 {
            Some(c) => {
                let mut c = self.resolver.value(c)?;
                if abs {
                    let fabs = format!("llvm.fabs.{}\0", LLVMTypeDisplay(type_));
                    let fabs = unsafe { CStr::from_bytes_with_nul_unchecked(fabs.as_bytes()) };
                    c = self.emit_intrinsic(fabs, None, vec![&type_.into()], vec![(c, llvm_type)])?;
                }
                let result = self.emit_intrinsic(
                    unsafe { CStr::from_bytes_with_nul_unchecked(intrinsic.as_bytes()) },
                    None,
                    vec![&type_.into()],
                    vec![(result, llvm_type), (c, llvm_type)],
                )?;
                let result = match data {
                    ptx_parser::MinMaxDetails::Float(ptx_parser::MinMaxFloat { flush_to_zero, .. })
                        if self.explicit_ftz(flush_to_zero) =>
                    {
                        self.emit_flush_subnormal(type_, result)
                    }
                    _ => result,
                };
                self.emit_canonical_nan(type_, result)
            }
            None => result,
        };
        self.resolver.register(dst, result);
        Ok(())
    }

    // Gives a non-NaN `value` the sign sign(a) xor sign(b)
    fn emit_xor_sign(
        &mut self,
        type_: ast::ScalarType,
        value: LLVMValueRef,
        a: LLVMValueRef,
        b: LLVMValueRef,
    ) -> Result<LLVMValueRef, TranslateError> {
        let element = type_.packed_type().unwrap_or(type_);
        let element_bits = element.size_of() as u32 * 8;
        let int_element = unsafe { LLVMIntTypeInContext(self.context, element_bits) };
        let (int_type, sign_mask) = unsafe {
            let sign = LLVMConstInt(int_element, 1u64 << (element_bits - 1), 0);
            if element == type_ {
                (int_element, sign)
            } else {
                let mut signs = [sign, sign];
                (
                    LLVMVectorType(int_element, 2),
                    LLVMConstVector(signs.as_mut_ptr(), 2),
                )
            }
        };
        let b_ = self.builder;
        unsafe {
            let a_bits = LLVMBuildBitCast(b_, a, int_type, LLVM_UNNAMED.as_ptr());
            let b_bits = LLVMBuildBitCast(b_, b, int_type, LLVM_UNNAMED.as_ptr());
            let sign = LLVMBuildXor(b_, a_bits, b_bits, LLVM_UNNAMED.as_ptr());
            let sign = LLVMBuildAnd(b_, sign, sign_mask, LLVM_UNNAMED.as_ptr());
            let bits = LLVMBuildBitCast(b_, value, int_type, LLVM_UNNAMED.as_ptr());
            let magnitude = LLVMBuildAnd(
                b_,
                bits,
                LLVMBuildNot(b_, sign_mask, LLVM_UNNAMED.as_ptr()),
                LLVM_UNNAMED.as_ptr(),
            );
            let signed = LLVMBuildOr(b_, magnitude, sign, LLVM_UNNAMED.as_ptr());
            let signed = LLVMBuildBitCast(b_, signed, LLVMTypeOf(value), LLVM_UNNAMED.as_ptr());
            let is_nan = LLVMBuildFCmp(
                b_,
                LLVMRealPredicate::LLVMRealUNO,
                value,
                value,
                LLVM_UNNAMED.as_ptr(),
            );
            Ok(LLVMBuildSelect(b_, is_nan, value, signed, LLVM_UNNAMED.as_ptr()))
        }
    }

    fn emit_fma_oob(&mut self, data: ptx_parser::ArithFloat, arguments: ast::FmaOobArgs<SpirvWord>) -> Result<(), TranslateError> {
        self.emit_fma(
            data,
            ast::FmaArgs { dst: arguments.dst, src1: arguments.src1, src2: arguments.src2, src3: arguments.src3 },
        )?;
        let (lanes, nan) = match data.type_ {
            ast::ScalarType::F16 => (1, 0x7e00),
            ast::ScalarType::BF16 => (1, 0x7fc0),
            ast::ScalarType::F16x2 => (2, 0x7e00),
            _ => (2, 0x7fc0),
        };
        let b = self.builder;
        unsafe {
            let int = LLVMIntTypeInContext(self.context, 16 * lanes);
            let vector = LLVMVectorType(LLVMInt16TypeInContext(self.context), lanes);
            let as_lanes = |v: LLVMValueRef| {
                let x = LLVMBuildBitCast(b, v, int, LLVM_UNNAMED.as_ptr());
                LLVMBuildBitCast(b, x, vector, LLVM_UNNAMED.as_ptr())
            };
            let pattern = LLVMConstVector(
                vec![LLVMConstInt(LLVMInt16TypeInContext(self.context), nan, 0); lanes as usize].as_mut_ptr(),
                lanes,
            );
            let mut oob = None;
            for src in [arguments.src1, arguments.src2, arguments.src3] {
                let x = as_lanes(self.resolver.value(src)?);
                let is = LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntEQ, x, pattern, LLVM_UNNAMED.as_ptr());
                oob = Some(match oob {
                    Some(o) => LLVMBuildOr(b, o, is, LLVM_UNNAMED.as_ptr()),
                    None => is,
                });
            }
            let result = self.resolver.value(arguments.dst)?;
            let r = as_lanes(result);
            let zero = LLVMConstNull(vector);
            let r = LLVMBuildSelect(b, oob.unwrap(), zero, r, LLVM_UNNAMED.as_ptr());
            let r = LLVMBuildBitCast(b, r, int, LLVM_UNNAMED.as_ptr());
            let r = LLVMBuildBitCast(b, r, LLVMTypeOf(result), LLVM_UNNAMED.as_ptr());
            self.resolver.register(arguments.dst, r);
        }
        Ok(())
    }

    fn emit_fma(
        &mut self,
        data: ptx_parser::ArithFloat,
        arguments: ptx_parser::FmaArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        if self.is_half_directed_rounding(&data) {
            if data.relu {
                // PTX's half precision fma (.relu included) has only .rn
                return Err(TranslateError::Unsupported("fma.relu takes only the .rn rounding mode".to_string()));
            }
            let srcs = vec![
                self.resolver.value(arguments.src1)?,
                self.resolver.value(arguments.src2)?,
                self.resolver.value(arguments.src3)?,
            ];
            return self.emit_half_directed_rounding(
                "llvm.experimental.constrained.fma",
                data,
                arguments.dst,
                srcs,
            );
        }
        let (prefix, metadata): (_, &[&str]) = match self.fp.mode {
            FloatingPointMode::Normal => ("llvm.fma", &[]),
            FloatingPointMode::Constrained => (
                "llvm.experimental.constrained.fma",
                &["round.dynamic", "fpexcept.ignore"],
            ),
        };
        let intrinsic = format!("{}.{}\0", prefix, LLVMTypeDisplay(data.type_));
        let result = self.emit_intrinsic_with_metadata(
            unsafe { CStr::from_bytes_with_nul_unchecked(intrinsic.as_bytes()) },
            if data.relu { None } else { Some(arguments.dst) },
            vec![&data.type_.into()],
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
            metadata,
        )?;
        if data.relu {
            // .relu: max(result, +0), keeping NaN
            let result = self.emit_relu(data.type_, result)?;
            self.resolver.register(arguments.dst, result);
        }
        Ok(())
    }

    // mad.f32 of sm_1x: the product with its mantissa truncated to 24 bits (rounded toward zero), then added with
    // rounding to nearest. The product rounded to nearest is one unit too far from zero when the exact error
    // (fma(a, b, -p)) has the opposite sign.
    fn emit_mad_sm_1x(&mut self, arguments: ptx_parser::MadArgs<SpirvWord>) -> Result<(), TranslateError> {
        let a = self.resolver.value(arguments.src1)?;
        let b = self.resolver.value(arguments.src2)?;
        let c = self.resolver.value(arguments.src3)?;
        let f32_type = get_scalar_type(self.context, ast::ScalarType::F32);
        let i32_type = get_scalar_type(self.context, ast::ScalarType::B32);
        let builder = self.builder;
        let p = unsafe { LLVMBuildFMul(builder, a, b, LLVM_UNNAMED.as_ptr()) };
        let neg_p = unsafe { LLVMBuildFNeg(builder, p, LLVM_UNNAMED.as_ptr()) };
        let err = self.emit_intrinsic(
            c"llvm.fma.f32",
            None,
            vec![&ast::ScalarType::F32.into()],
            vec![(a, f32_type), (b, f32_type), (neg_p, f32_type)],
        )?;
        unsafe {
            let zero = LLVMConstReal(f32_type, 0.0);
            let p_bits = LLVMBuildBitCast(builder, p, i32_type, LLVM_UNNAMED.as_ptr());
            let err_bits = LLVMBuildBitCast(builder, err, i32_type, LLVM_UNNAMED.as_ptr());
            let signs = LLVMBuildXor(builder, p_bits, err_bits, LLVM_UNNAMED.as_ptr());
            let opposite = LLVMBuildICmp(builder, LLVMIntPredicate::LLVMIntSLT, signs, LLVMConstInt(i32_type, 0, 0), LLVM_UNNAMED.as_ptr());
            let inexact = LLVMBuildFCmp(builder, LLVMRealPredicate::LLVMRealONE, err, zero, LLVM_UNNAMED.as_ptr());
            let finite = {
                let abs = LLVMBuildAnd(builder, p_bits, LLVMConstInt(i32_type, 0x7fff_ffff, 0), LLVM_UNNAMED.as_ptr());
                LLVMBuildICmp(builder, LLVMIntPredicate::LLVMIntULT, abs, LLVMConstInt(i32_type, 0x7f80_0000, 0), LLVM_UNNAMED.as_ptr())
            };
            let back = LLVMBuildAnd(builder, opposite, inexact, LLVM_UNNAMED.as_ptr());
            let back = LLVMBuildAnd(builder, back, finite, LLVM_UNNAMED.as_ptr());
            let one_less = LLVMBuildSub(builder, p_bits, LLVMConstInt(i32_type, 1, 0), LLVM_UNNAMED.as_ptr());
            let t_bits = LLVMBuildSelect(builder, back, one_less, p_bits, LLVM_UNNAMED.as_ptr());
            let t = LLVMBuildBitCast(builder, t_bits, f32_type, LLVM_UNNAMED.as_ptr());
            self.resolver.with_result(arguments.dst, |dst| LLVMBuildFAdd(builder, t, c, dst));
        }
        Ok(())
    }

    fn emit_mad(
        &mut self,
        data: ptx_parser::MadDetails,
        arguments: ptx_parser::MadArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let mul_control = match data {
            ptx_parser::MadDetails::Float(mad_float)
                if mad_float.is_fusable
                    && mad_float.type_ == ast::ScalarType::F32
                    && !mad_float.saturate
                    && super::super::instruction_mode_to_global_mode::sm_1x() =>
            {
                return self.emit_mad_sm_1x(arguments);
            }
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
        if is_floating_point && data.flush_to_zero == Some(true) && self.target == Target::Agx {
            let flushed = self.emit_flush_subnormal(data.type_, src);
            let result = self.emit_sign_bit_op(data.type_, flushed, false);
            self.resolver.register(arguments.dst, result);
            return Ok(());
        }
        if is_floating_point && data.flush_to_zero != Some(true) && self.target == Target::Agx {
            // A bit operation, which keeps NaN payloads
            let mut result = self.emit_sign_bit_op(data.type_, src, false);
            if data.type_ == ast::ScalarType::F64 {
                // "For abs.f64, NaN input is passed through unchanged" (PTX ISA 9.7.3.9), sign included
                unsafe {
                    let is_nan = LLVMBuildFCmp(self.builder, LLVMRealPredicate::LLVMRealUNO, src, src, LLVM_UNNAMED.as_ptr());
                    result = LLVMBuildSelect(self.builder, is_nan, src, result, LLVM_UNNAMED.as_ptr());
                }
            }
            self.resolver.register(arguments.dst, result);
            return Ok(());
        }
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
            vec![&data.type_.into()],
            intrinsic_arguments,
        )?;
        if is_floating_point && data.flush_to_zero == Some(true) {
            let intrinsic = format!("llvm.canonicalize.{}\0", LLVMTypeDisplay(data.type_));
            self.emit_intrinsic(
                unsafe { CStr::from_bytes_with_nul_unchecked(intrinsic.as_bytes()) },
                Some(arguments.dst),
                vec![&data.type_.into()],
                vec![(abs_result, llvm_type)],
            )?;
        } else {
            self.resolver.register(arguments.dst, abs_result);
        }
        Ok(())
    }

    // A subnormal value (or element) replaced by the zero of its sign, with bit operations: .ftz, independent of
    // the denormal mode of the function
    fn emit_flush_subnormal(&mut self, type_: ast::ScalarType, src: LLVMValueRef) -> LLVMValueRef {
        let element = type_.packed_type().unwrap_or(type_);
        let element_bits = element.size_of() as u32 * 8;
        let exponent_mask: u64 = match element {
            ast::ScalarType::F16 => 0x7c00,
            ast::ScalarType::BF16 => 0x7f80,
            ast::ScalarType::F32 => 0x7f80_0000,
            ast::ScalarType::F64 => 0x7ff0_0000_0000_0000,
            _ => return src,
        };
        unsafe {
            let int_element = LLVMIntTypeInContext(self.context, element_bits);
            let constant = |v: u64| {
                let c = LLVMConstInt(int_element, v, 0);
                if element == type_ {
                    c
                } else {
                    let mut cs = [c, c];
                    LLVMConstVector(cs.as_mut_ptr(), 2)
                }
            };
            let int_type = if element == type_ { int_element } else { LLVMVectorType(int_element, 2) };
            let b = self.builder;
            let bits = LLVMBuildBitCast(b, src, int_type, LLVM_UNNAMED.as_ptr());
            let exponent = LLVMBuildAnd(b, bits, constant(exponent_mask), LLVM_UNNAMED.as_ptr());
            let small = LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntEQ, exponent, constant(0), LLVM_UNNAMED.as_ptr());
            let sign = LLVMBuildAnd(b, bits, constant(1u64 << (element_bits - 1)), LLVM_UNNAMED.as_ptr());
            let result = LLVMBuildSelect(b, small, sign, bits, LLVM_UNNAMED.as_ptr());
            LLVMBuildBitCast(b, result, LLVMTypeOf(src), LLVM_UNNAMED.as_ptr())
        }
    }

    // Flips (negate) or clears the sign bit of a float or packed floats
    fn emit_sign_bit_op(
        &mut self,
        type_: ast::ScalarType,
        src: LLVMValueRef,
        negate: bool,
    ) -> LLVMValueRef {
        let element = type_.packed_type().unwrap_or(type_);
        let element_bits = element.size_of() as u32 * 8;
        unsafe {
            let int_element = LLVMIntTypeInContext(self.context, element_bits);
            let mask = if negate {
                1u64 << (element_bits - 1)
            } else {
                (1u64 << (element_bits - 1)) - 1
            };
            let mask = LLVMConstInt(int_element, mask, 0);
            let (int_type, mask) = if element == type_ {
                (int_element, mask)
            } else {
                let mut masks = [mask, mask];
                (LLVMVectorType(int_element, 2), LLVMConstVector(masks.as_mut_ptr(), 2))
            };
            let bits = LLVMBuildBitCast(self.builder, src, int_type, LLVM_UNNAMED.as_ptr());
            let result = if negate {
                LLVMBuildXor(self.builder, bits, mask, LLVM_UNNAMED.as_ptr())
            } else {
                LLVMBuildAnd(self.builder, bits, mask, LLVM_UNNAMED.as_ptr())
            };
            LLVMBuildBitCast(self.builder, result, LLVMTypeOf(src), LLVM_UNNAMED.as_ptr())
        }
    }

    fn emit_mul24(
        &mut self,
        data: ast::Mul24Details,
        arguments: ast::Mul24Args<SpirvWord>,
    ) -> Result<(), TranslateError> {
        if self.target == Target::Agx {
            return self.emit_mul24_expanded(data, arguments);
        }
        let src1 = self.resolver.value(arguments.src1)?;
        let src2 = self.resolver.value(arguments.src2)?;
        let name_lo = match data.type_ {
            ast::ScalarType::U32 => c"llvm.amdgcn.mul.u24.i32",
            ast::ScalarType::S32 => c"llvm.amdgcn.mul.i24.i32",
            _ => return Err(error_unreachable()),
        };
        let res_lo = self.emit_intrinsic(
            name_lo,
            if data.control == Mul24Control::Lo {
                Some(arguments.dst)
            } else {
                None
            },
            vec![&ast::Type::Scalar(data.type_)],
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
                vec![&ast::Type::Scalar(data.type_)],
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
                .with_result(arguments.dst, |dst: *const std::ffi::c_char| unsafe {
                    LLVMBuildOr(self.builder, res_lo_shr, res_hi_shl, dst)
                });
        }
        Ok(())
    }

    /// mul24 without 24-bit multiply instructions: a 64-bit product of the
    /// extended low 24 bits, of which .lo returns bits 31..0 and .hi bits
    /// 47..16.
    fn emit_mul24_expanded(
        &mut self,
        data: ast::Mul24Details,
        arguments: ast::Mul24Args<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let signed = match data.type_ {
            ast::ScalarType::U32 => false,
            ast::ScalarType::S32 => true,
            _ => return Err(error_unreachable()),
        };
        let i32_type = unsafe { LLVMInt32TypeInContext(self.context) };
        let i64_type = unsafe { LLVMInt64TypeInContext(self.context) };
        let builder = self.builder;
        let extend = |value: LLVMValueRef| unsafe {
            let low24 = if signed {
                let eight = LLVMConstInt(i32_type, 8, 0);
                let shifted = LLVMBuildShl(builder, value, eight, LLVM_UNNAMED.as_ptr());
                LLVMBuildAShr(builder, shifted, eight, LLVM_UNNAMED.as_ptr())
            } else {
                let mask = LLVMConstInt(i32_type, 0xff_ffff, 0);
                LLVMBuildAnd(builder, value, mask, LLVM_UNNAMED.as_ptr())
            };
            if signed {
                LLVMBuildSExt(builder, low24, i64_type, LLVM_UNNAMED.as_ptr())
            } else {
                LLVMBuildZExt(builder, low24, i64_type, LLVM_UNNAMED.as_ptr())
            }
        };
        let src1 = extend(self.resolver.value(arguments.src1)?);
        let src2 = extend(self.resolver.value(arguments.src2)?);
        let product = unsafe { LLVMBuildMul(builder, src1, src2, LLVM_UNNAMED.as_ptr()) };
        let product = if data.control == Mul24Control::Hi {
            let sixteen = unsafe { LLVMConstInt(i64_type, 16, 0) };
            unsafe { LLVMBuildLShr(builder, product, sixteen, LLVM_UNNAMED.as_ptr()) }
        } else {
            product
        };
        self.resolver.with_result(arguments.dst, |dst| unsafe {
            LLVMBuildTrunc(builder, product, i32_type, dst)
        });
        Ok(())
    }

    /// AGX has no floating point mode register: the backend models one with
    /// llvm.agx.get.fpmode / llvm.agx.set.fpmode (bits 0-1 f32 rounding and
    /// 2-3 f16/f64 rounding, numbered like llvm.get.rounding; bit 4 f32 and
    /// bit 5 f16/f64 flush denormals) and implements the directed rounding
    /// modes in software.
    fn emit_set_mode_agx(&mut self, mode_reg: ModeRegister) -> Result<(), TranslateError> {
        fn rounding(mode: ast::RoundingMode) -> u64 {
            match mode {
                ast::RoundingMode::Zero => 0,
                ast::RoundingMode::NearestEven => 1,
                ast::RoundingMode::PositiveInf => 2,
                ast::RoundingMode::NegativeInf => 3,
            }
        }
        let (mask, value) = match mode_reg {
            ModeRegister::Denormal { f32, f16f64 } => {
                (0x30, (f32 as u64) << 4 | (f16f64 as u64) << 5)
            }
            ModeRegister::Rounding { f32, f16f64 } => {
                (0xf, rounding(f32) | rounding(f16f64) << 2)
            }
        };
        let i32_type = get_scalar_type(self.context, ast::ScalarType::B32);
        let current = self.emit_intrinsic(
            c"llvm.agx.get.fpmode",
            None,
            vec![&ast::ScalarType::B32.into()],
            vec![],
        )?;
        let mode = unsafe {
            let kept = LLVMBuildAnd(
                self.builder,
                current,
                LLVMConstInt(i32_type, !mask & 0xffff_ffff, 0),
                LLVM_UNNAMED.as_ptr(),
            );
            LLVMBuildOr(
                self.builder,
                kept,
                LLVMConstInt(i32_type, value, 0),
                LLVM_UNNAMED.as_ptr(),
            )
        };
        self.emit_intrinsic(
            c"llvm.agx.set.fpmode",
            None,
            vec![],
            vec![(mode, i32_type)],
        )?;
        Ok(())
    }

    fn emit_set_mode(&mut self, mode_reg: ModeRegister) -> Result<(), TranslateError> {
        if self.target == Target::Agx {
            return self.emit_set_mode_agx(mode_reg);
        }
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
            vec![],
            vec![(hwreg_llvm, llvm_i32), (value_llvm, llvm_i32)],
        )?;
        Ok(())
    }

    // Subnormals (exponent field 0) keep only their sign bit
    fn emit_fp_flush(
        &mut self,
        type_: ast::ScalarType,
        dst: SpirvWord,
        src: SpirvWord,
    ) -> Result<(), TranslateError> {
        let (bits, lanes, exponent): (u32, u32, u64) = match type_ {
            ast::ScalarType::BF16 => (16, 1, 0x7f80),
            ast::ScalarType::BF16x2 => (16, 2, 0x7f80),
            ast::ScalarType::F16 => (16, 1, 0x7c00),
            ast::ScalarType::F16x2 => (16, 2, 0x7c00),
            ast::ScalarType::F32 => (32, 1, 0x7f80_0000),
            _ => return Err(error_unreachable()),
        };
        let float_type = get_scalar_type(self.context, type_);
        let src = self.resolver.value(src)?;
        unsafe {
            let element = LLVMIntTypeInContext(self.context, bits);
            let int_type = if lanes == 1 { element } else { LLVMVectorType(element, lanes) };
            let splat = |v: u64| {
                let c = LLVMConstInt(element, v, 0);
                if lanes == 1 {
                    c
                } else {
                    let mut elems = vec![c; lanes as usize];
                    LLVMConstVector(elems.as_mut_ptr(), lanes)
                }
            };
            let x = LLVMBuildBitCast(self.builder, src, int_type, LLVM_UNNAMED.as_ptr());
            let exp = LLVMBuildAnd(self.builder, x, splat(exponent), LLVM_UNNAMED.as_ptr());
            let subnormal = LLVMBuildICmp(
                self.builder,
                LLVMIntPredicate::LLVMIntEQ,
                exp,
                splat(0),
                LLVM_UNNAMED.as_ptr(),
            );
            let sign = LLVMBuildAnd(self.builder, x, splat(1 << (bits - 1)), LLVM_UNNAMED.as_ptr());
            let flushed = LLVMBuildSelect(self.builder, subnormal, sign, x, LLVM_UNNAMED.as_ptr());
            self.resolver.with_result(dst, |dst| {
                LLVMBuildBitCast(self.builder, flushed, float_type, dst)
            });
        }
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
        let maxnum_intrinsic = format!("llvm.maximumnum.{}\0", LLVMTypeDisplay(type_));
        let minnum_intrinsic = format!("llvm.minimumnum.{}\0", LLVMTypeDisplay(type_));
        let src = self.resolver.value(src)?;
        if self.strict_fp {
            // PTX: NaN and values below 0 become +0.0, values above 1 become
            // 1.0, and -0.0 stays (maximumnum(-0.0, 0.0) would be +0.0).
            unsafe {
                let below = LLVMBuildFCmp(
                    self.builder,
                    LLVMRealPredicate::LLVMRealULT,
                    src,
                    zero,
                    LLVM_UNNAMED.as_ptr(),
                );
                let above = LLVMBuildFCmp(
                    self.builder,
                    LLVMRealPredicate::LLVMRealOGT,
                    src,
                    one,
                    LLVM_UNNAMED.as_ptr(),
                );
                let clamped = LLVMBuildSelect(self.builder, above, one, src, LLVM_UNNAMED.as_ptr());
                self.resolver.with_result(dst, |dst| {
                    LLVMBuildSelect(self.builder, below, zero, clamped, dst)
                });
            }
            return Ok(());
        }
        let maxnum = self.emit_intrinsic(
            unsafe { CStr::from_bytes_with_nul_unchecked(maxnum_intrinsic.as_bytes()) },
            None,
            vec![&type_.into()],
            vec![(src, llvm_type), (zero, llvm_type)],
        )?;
        self.emit_intrinsic(
            unsafe { CStr::from_bytes_with_nul_unchecked(minnum_intrinsic.as_bytes()) },
            Some(dst),
            vec![&type_.into()],
            vec![(maxnum, llvm_type), (one, llvm_type)],
        )?;
        Ok(())
    }

    fn emit_intrinsic_saturate(
        &mut self,
        op: &str,
        type_: ast::ScalarType,
        dst: SpirvWord,
        src1: LLVMValueRef,
        src2: LLVMValueRef,
    ) -> Result<(), TranslateError> {
        let llvm_type = get_scalar_type(self.context, type_);
        let intrinsic = format!("llvm.{}.sat.{}\0", op, LLVMTypeDisplay(type_));
        self.emit_intrinsic(
            unsafe { CStr::from_bytes_with_nul_unchecked(intrinsic.as_bytes()) },
            Some(dst),
            vec![&type_.into()],
            vec![(src1, llvm_type), (src2, llvm_type)],
        )?;
        Ok(())
    }

    fn emit_cp_async(
        &mut self,
        data: CpAsyncDetails,
        arguments: CpAsyncArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let to = self.resolver.value(arguments.src_to)?;
        let from = self.resolver.value(arguments.src_from)?;
        let cp_size = data.cp_size;
        if self.target == Target::Agx || arguments.src_size.is_some() {
            // Copy the first src-size bytes (none with a true ignore-src) and
            // zero the rest
            let i32_type = unsafe { LLVMInt32TypeInContext(self.context) };
            let cp_size_value = unsafe { LLVMConstInt(i32_type, cp_size.as_u64(), 0) };
            let src_size = match (arguments.src_size, data.src_size) {
                (Some(src_size), _) => {
                    let src_size = self.resolver.value(src_size)?;
                    if data.ignore_src {
                        unsafe {
                            LLVMBuildSelect(
                                self.builder,
                                src_size,
                                LLVMConstInt(i32_type, 0, 0),
                                cp_size_value,
                                LLVM_UNNAMED.as_ptr(),
                            )
                        }
                    } else {
                        self.emit_intrinsic(
                            c"llvm.umin.i32",
                            None,
                            vec![&ast::ScalarType::U32.into()],
                            vec![(src_size, i32_type), (cp_size_value, i32_type)],
                        )?
                    }
                }
                (None, Some(src_size)) => unsafe {
                    LLVMConstInt(i32_type, src_size.min(cp_size.as_u64()), 0)
                },
                (None, None) => cp_size_value,
            };
            let align = cp_size.as_u64() as u32;
            unsafe { LLVMBuildMemCpy(self.builder, to, align, from, align, src_size) };
            let i8_type = unsafe { LLVMInt8TypeInContext(self.context) };
            let mut offset = [src_size];
            let rest = unsafe {
                LLVMBuildInBoundsGEP2(
                    self.builder,
                    i8_type,
                    to,
                    offset.as_mut_ptr(),
                    1,
                    LLVM_UNNAMED.as_ptr(),
                )
            };
            let rest_size = unsafe {
                LLVMBuildSub(self.builder, cp_size_value, src_size, LLVM_UNNAMED.as_ptr())
            };
            unsafe {
                LLVMBuildMemSet(
                    self.builder,
                    rest,
                    LLVMConstInt(i8_type, 0, 0),
                    rest_size,
                    1,
                )
            };
            return Ok(());
        }
        // Asynchronous copies are not supported by all AMD hardware, so we just do a synchronous copy for now
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

    // mbarrier objects and the copies completed on them are emulated by the
    // helper functions of zluda_ptx_impl_agx.ll, which also describes the
    // layout of the objects
    fn emit_agx_helper(
        &mut self,
        name: &CStr,
        dst: Option<SpirvWord>,
        return_type: Option<ast::ScalarType>,
        arguments: &[LLVMValueRef],
    ) -> Result<LLVMValueRef, TranslateError> {
        if self.target != Target::Agx {
            return Err(error_todo_msg(
                "mbarrier objects are only supported on AGX",
            ));
        }
        let return_type = return_type.map(ast::Type::Scalar);
        self.emit_intrinsic(
            name,
            dst,
            return_type.iter().collect(),
            arguments
                .iter()
                .map(|arg| (*arg, unsafe { LLVMTypeOf(*arg) }))
                .collect(),
        )
    }

    // A pointer to an mbarrier object in shared memory
    fn mbarrier_pointer(
        &mut self,
        space: ast::StateSpace,
        mbarrier: SpirvWord,
    ) -> Result<LLVMValueRef, TranslateError> {
        let mbarrier = self.resolver.value(mbarrier)?;
        Ok(match space {
            ast::StateSpace::Shared => mbarrier,
            _ => unsafe {
                LLVMBuildAddrSpaceCast(
                    self.builder,
                    mbarrier,
                    get_pointer_type(self.context, ast::StateSpace::Shared)?,
                    LLVM_UNNAMED.as_ptr(),
                )
            },
        })
    }

    fn const_u32(&self, value: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(LLVMInt32TypeInContext(self.context), value, 0) }
    }

    fn emit_cp_async_mbarrier_arrive(
        &mut self,
        data: ast::CpAsyncMbarrierArriveDetails,
        arguments: ast::CpAsyncMbarrierArriveArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        // The copies are complete: without .noinc, the arrive-on increment of
        // the pending count and the arrival cancel out
        if !data.noinc {
            return Ok(());
        }
        let mbarrier = self.mbarrier_pointer(data.space, arguments.src_mbarrier)?;
        let (one, zero) = (self.const_u32(1), self.const_u32(0));
        self.emit_agx_helper(
            c"__zluda_ptx_impl_agx_mbarrier_arrive",
            None,
            Some(ast::ScalarType::B64),
            &[mbarrier, one, zero],
        )?;
        Ok(())
    }

    // st.async / red.async: the store or the atomic operation, then its bytes complete on the mbarrier
    fn emit_st_async(
        &mut self,
        data: ast::StAsyncDetails,
        arguments: ast::StAsyncArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let bytes = match &data.type_ {
            ast::Type::Vector(n, t) => *n as u64 * t.size_of() as u64,
            _ => data.scalar.size_of() as u64,
        };
        match data.red {
            None => {
                let to = self.resolver.value(arguments.src_addr)?;
                let value = self.resolver.value(arguments.src_value)?;
                unsafe { LLVMBuildStore(self.builder, value, to) };
            }
            Some(op) => {
                self.emit_atom_impl(
                    ast::AtomDetails {
                        noftz: false,
                        semantics: ast::AtomSemantics::Relaxed,
                        scope: ast::MemScope::Gpu,
                        space: ast::StateSpace::Shared,
                        op,
                        type_: ast::Type::Scalar(data.scalar),
                    },
                    arguments.src_addr,
                    arguments.src_value,
                )?;
            }
        }
        let mbarrier = self.mbarrier_pointer(ast::StateSpace::Shared, arguments.src_mbarrier)?;
        let bytes = self.const_u32(bytes);
        self.emit_agx_helper(c"__zluda_ptx_impl_agx_mbarrier_complete_tx", None, None, &[mbarrier, bytes])?;
        Ok(())
    }

    // clusterlaunchcontrol.try_cancel never cancels a cluster: the response (16 bytes) is 0, and completes;
    // .multicast::cluster::all writes it and completes the transaction in each CTA of the cluster
    fn emit_cluster_launch_try_cancel(
        &mut self,
        multicast: bool,
        arguments: ast::ClusterLaunchTryCancelArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let to = self.resolver.value(arguments.src_addr)?;
        if multicast {
            let mbarrier = self.mbarrier_pointer(ast::StateSpace::Shared, arguments.src_mbarrier)?;
            self.emit_agx_helper(c"__zluda_ptx_impl_agx_clc_try_cancel_multicast", None, None, &[to, mbarrier])?;
            return Ok(());
        }
        unsafe {
            let zero = LLVMConstInt(LLVMInt8TypeInContext(self.context), 0, 0);
            let size = LLVMConstInt(LLVMInt32TypeInContext(self.context), 16, 0);
            LLVMBuildMemSet(self.builder, to, zero, size, 16);
        }
        let mbarrier = self.mbarrier_pointer(ast::StateSpace::Shared, arguments.src_mbarrier)?;
        let bytes = self.const_u32(16);
        self.emit_agx_helper(c"__zluda_ptx_impl_agx_mbarrier_complete_tx", None, None, &[mbarrier, bytes])?;
        Ok(())
    }

    fn emit_cp_async_bulk(
        &mut self,
        data: ast::CpAsyncBulkDetails,
        arguments: ast::CpAsyncBulkArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let _ = arguments.src_cache_policy;
        let to = self.resolver.value(arguments.src_to)?;
        let from = self.resolver.value(arguments.src_from)?;
        let size = self.resolver.value(arguments.src_size)?;
        // to the CTAs of the cluster in the mask (a cluster is one threadgroup on AGX)
        if let (true, Some(mask), Some(mbarrier)) = (data.multicast, arguments.src_cta_mask, arguments.src_mbarrier) {
            let mask = self.resolver.value(mask)?;
            let mask = unsafe {
                LLVMBuildZExt(self.builder, mask, LLVMInt32TypeInContext(self.context), LLVM_UNNAMED.as_ptr())
            };
            let mbarrier = self.mbarrier_pointer(data.dst_space, mbarrier)?;
            self.emit_agx_helper(
                c"__zluda_ptx_impl_agx_cp_async_bulk_multicast",
                None,
                None,
                &[to, from, size, mbarrier, mask],
            )?;
            return Ok(());
        }
        match arguments.src_byte_mask {
            Some(byte_mask) => {
                let byte_mask = self.resolver.value(byte_mask)?;
                let byte_mask = unsafe {
                    LLVMBuildZExt(
                        self.builder,
                        byte_mask,
                        LLVMInt32TypeInContext(self.context),
                        LLVM_UNNAMED.as_ptr(),
                    )
                };
                self.emit_agx_helper(
                    c"__zluda_ptx_impl_agx_cp_async_bulk_masked",
                    None,
                    None,
                    &[to, from, size, byte_mask],
                )?;
            }
            None => unsafe {
                LLVMBuildMemCpy(self.builder, to, 16, from, 16, size);
            },
        }
        if let Some(mbarrier) = arguments.src_mbarrier {
            let mbarrier = self.mbarrier_pointer(data.dst_space, mbarrier)?;
            // the data inspection of .report_mechanism precedes the complete-tx
            if data.report != 0 && data.src_space == ast::StateSpace::Global {
                let code = self.const_u32(data.report as u64);
                self.emit_agx_helper(
                    c"__zluda_ptx_impl_agx_mbarrier_inspect",
                    None,
                    None,
                    &[mbarrier, from, size, code],
                )?;
            }
            self.emit_agx_helper(
                c"__zluda_ptx_impl_agx_mbarrier_complete_tx",
                None,
                None,
                &[mbarrier, size],
            )?;
        }
        Ok(())
    }

    fn emit_cp_async_bulk_tensor(
        &mut self,
        data: ast::CpAsyncBulkTensorDetails,
        arguments: ast::CpAsyncBulkTensorArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let _ = arguments.src_cache_policy;
        let i32_type = unsafe { LLVMInt32TypeInContext(self.context) };
        let smem = self.resolver.value(arguments.src_smem)?;
        let tmap = self.resolver.value(arguments.src_tmap)?;
        let tmap = unsafe {
            LLVMBuildIntToPtr(
                self.builder,
                tmap,
                get_pointer_type(self.context, ast::StateSpace::Generic)?,
                LLVM_UNNAMED.as_ptr(),
            )
        };
        // .override: a patched copy of the tensor map, in private memory
        let tmap = match arguments.src_ov_addr {
            Some(address) => {
                let i64_type = unsafe { LLVMInt64TypeInContext(self.context) };
                let i1_type = unsafe { LLVMInt1TypeInContext(self.context) };
                let copy = unsafe {
                    LLVMZludaBuildAlloca(
                        self.variables_builder.get(),
                        LLVMArrayType2(i64_type, 16),
                        get_state_space(ast::StateSpace::Local)?,
                        LLVM_UNNAMED.as_ptr(),
                    )
                };
                let copy = unsafe {
                    LLVMBuildAddrSpaceCast(
                        self.builder,
                        copy,
                        get_pointer_type(self.context, ast::StateSpace::Generic)?,
                        LLVM_UNNAMED.as_ptr(),
                    )
                };
                let address = self.resolver.value(address)?;
                let values = |ids: &[Option<SpirvWord>]| -> Result<(LLVMValueRef, Vec<LLVMValueRef>), TranslateError> {
                    let count = ids.iter().filter(|i| i.is_some()).count();
                    let mut values = Vec::new();
                    for id in ids {
                        values.push(match id {
                            Some(id) => {
                                let v = self.resolver.value(*id)?;
                                unsafe { LLVMBuildZExtOrBitCast(self.builder, v, i32_type, LLVM_UNNAMED.as_ptr()) }
                            }
                            None => self.const_u32(0),
                        });
                    }
                    Ok((self.const_u32(count as u64), values))
                };
                // without .override::global_address the address operand is the map's own
                let (nd, dims) = values(&[
                    arguments.src_ov_d0,
                    arguments.src_ov_d1,
                    arguments.src_ov_d2,
                    arguments.src_ov_d3,
                    arguments.src_ov_d4,
                ])?;
                let (ns, strides) = values(&[
                    arguments.src_ov_s0,
                    arguments.src_ov_s1,
                    arguments.src_ov_s2,
                    arguments.src_ov_s3,
                ])?;
                let (_, up) = values(&[arguments.src_ov_up])?;
                let has_address = unsafe { LLVMConstInt(i1_type, data.override_address as u64, 0) };
                let mut args = vec![copy, tmap, has_address, address, nd];
                args.extend(dims);
                args.push(ns);
                args.extend(strides);
                args.extend(up);
                self.emit_agx_helper(c"__zluda_ptx_impl_agx_tma_override", None, None, &args)?;
                copy
            }
            None => tmap,
        };
        let mut coords = vec![self.resolver.value(arguments.src_c0)?];
        for c in [arguments.src_c1, arguments.src_c2, arguments.src_c3, arguments.src_c4] {
            coords.push(match c {
                Some(c) => self.resolver.value(c)?,
                None => self.const_u32(0),
            });
        }
        let mut info = Vec::new();
        for i in [arguments.src_i0, arguments.src_i1, arguments.src_i2] {
            info.push(match i {
                Some(i) => {
                    let i = self.resolver.value(i)?;
                    unsafe { LLVMBuildSExtOrBitCast(self.builder, i, i32_type, LLVM_UNNAMED.as_ptr()) }
                }
                None => self.const_u32(0),
            });
        }
        let imode = self.const_u32(data.im2col.map_or(u32::MAX, u32::from) as u64);
        let mut args = Vec::new();
        match data.kind {
            ast::TensorCopyKind::Load => {
                let mbarrier = arguments.src_mbarrier.ok_or_else(error_unreachable)?;
                let mbarrier = self.mbarrier_pointer(data.smem_space, mbarrier)?;
                let mask = match arguments.src_cta_mask {
                    Some(mask) => {
                        let mask = self.resolver.value(mask)?;
                        unsafe { LLVMBuildZExtOrBitCast(self.builder, mask, i32_type, LLVM_UNNAMED.as_ptr()) }
                    }
                    None => self.const_u32(0),
                };
                let flags = data.multicast as u64 | (data.cta_group2 as u64) << 1 | (data.report as u64) << 8;
                args.push(smem);
                args.push(tmap);
                args.extend(coords);
                args.extend([mbarrier, mask, self.const_u32(flags), imode]);
                args.extend(info);
                self.emit_agx_helper(c"__zluda_ptx_impl_agx_tma_load", None, None, &args)?;
            }
            ast::TensorCopyKind::Store | ast::TensorCopyKind::Reduce(_) => {
                // tma_red's operations; signedness and float come from the tensor map
                let op: i64 = match data.kind {
                    ast::TensorCopyKind::Reduce(op) => match op {
                        ast::AtomicOp::Add | ast::AtomicOp::FloatAdd => 0,
                        ast::AtomicOp::And => 1,
                        ast::AtomicOp::Or => 2,
                        ast::AtomicOp::Xor => 3,
                        ast::AtomicOp::IncrementWrap => 4,
                        ast::AtomicOp::DecrementWrap => 5,
                        ast::AtomicOp::SignedMin | ast::AtomicOp::UnsignedMin | ast::AtomicOp::FloatMin => 6,
                        ast::AtomicOp::SignedMax | ast::AtomicOp::UnsignedMax | ast::AtomicOp::FloatMax => 8,
                        ast::AtomicOp::Exchange => return Err(error_unreachable()),
                    },
                    _ => -2,
                };
                args.push(tmap);
                args.extend(coords);
                args.push(smem);
                args.push(self.const_u32(op as u32 as u64));
                args.push(imode);
                self.emit_agx_helper(c"__zluda_ptx_impl_agx_tma_store", None, None, &args)?;
            }
        }
        Ok(())
    }

    // ---------------------------------------------------------------- textures and surfaces (in software)

    fn i32_value(&mut self, id: Option<SpirvWord>) -> Result<LLVMValueRef, TranslateError> {
        let i32_type = unsafe { LLVMInt32TypeInContext(self.context) };
        Ok(match id {
            Some(id) => {
                let v = self.resolver.value(id)?;
                let ty = unsafe { LLVMTypeOf(v) };
                unsafe {
                    if LLVMGetTypeKind(ty) == LLVMTypeKind::LLVMFloatTypeKind {
                        LLVMBuildBitCast(self.builder, v, i32_type, LLVM_UNNAMED.as_ptr())
                    } else {
                        LLVMBuildZExtOrBitCast(self.builder, v, i32_type, LLVM_UNNAMED.as_ptr())
                    }
                }
            }
            None => self.const_u32(0),
        })
    }

    fn f32_value(&mut self, id: Option<SpirvWord>) -> Result<LLVMValueRef, TranslateError> {
        let v = self.i32_value(id)?;
        Ok(unsafe { LLVMBuildBitCast(self.builder, v, LLVMFloatTypeInContext(self.context), LLVM_UNNAMED.as_ptr()) })
    }

    // a texture, sampler or surface handle as a device pointer to its descriptor (null for None)
    fn descriptor(&mut self, id: Option<SpirvWord>) -> Result<LLVMValueRef, TranslateError> {
        let ptr = get_pointer_type(self.context, ast::StateSpace::Global)?;
        Ok(match id {
            Some(id) => {
                let mut v = self.resolver.value(id)?;
                let i64_type = unsafe { LLVMInt64TypeInContext(self.context) };
                // a .texref / .surfref / .samplerref variable: on AGX it holds the handle
                if unsafe { LLVMGetTypeKind(LLVMTypeOf(v)) } == LLVMTypeKind::LLVMPointerTypeKind {
                    v = unsafe { LLVMBuildLoad2(self.builder, i64_type, v, LLVM_UNNAMED.as_ptr()) };
                }
                let v = unsafe { LLVMBuildZExtOrBitCast(self.builder, v, i64_type, LLVM_UNNAMED.as_ptr()) };
                unsafe { LLVMBuildIntToPtr(self.builder, v, ptr, LLVM_UNNAMED.as_ptr()) }
            }
            None => unsafe { LLVMConstNull(ptr) },
        })
    }

    // a helper returning a vector of 4 `element`s
    fn emit_vector_helper(
        &mut self,
        name: &CStr,
        element: ast::ScalarType,
        arguments: &[LLVMValueRef],
    ) -> Result<LLVMValueRef, TranslateError> {
        let return_type = ast::Type::Vector(4, element);
        self.emit_intrinsic(
            name,
            None,
            vec![&return_type],
            arguments.iter().map(|arg| (*arg, unsafe { LLVMTypeOf(*arg) })).collect(),
        )
    }

    // element i of a vector, as the type of the destination
    fn vector_element(
        &mut self,
        vector: LLVMValueRef,
        i: u32,
        dst: SpirvWord,
        dst_type: ast::ScalarType,
    ) -> Result<(), TranslateError> {
        let builder = self.builder;
        let index = self.const_u32(i as u64);
        let v = unsafe { LLVMBuildExtractElement(builder, vector, index, LLVM_UNNAMED.as_ptr()) };
        let target = get_scalar_type(self.context, dst_type);
        let v = unsafe {
            if dst_type.kind() == ast::ScalarKind::Float {
                LLVMBuildBitCast(builder, v, target, LLVM_UNNAMED.as_ptr())
            } else {
                LLVMBuildTruncOrBitCast(builder, v, target, LLVM_UNNAMED.as_ptr())
            }
        };
        self.resolver.register(dst, v);
        Ok(())
    }

    fn geometry_code(geometry: ast::TexGeometry) -> u64 {
        geometry as u64
    }

    fn emit_tex_agx(
        &mut self,
        data: ast::TexAgxDetails,
        arguments: ast::TexAgxArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let i32_type = unsafe { LLVMInt32TypeInContext(self.context) };
        let tex = self.descriptor(Some(arguments.src_tex))?;
        let sampler = self.descriptor(arguments.src_sampler)?;
        let geometry = self.const_u32(Self::geometry_code(data.geometry));
        // the f16 forms sample as f32 and round
        let dtype = self.const_u32(match data.dtype {
            ast::ScalarType::F32 | ast::ScalarType::F16 | ast::ScalarType::F16x2 => 0,
            ast::ScalarType::S32 => 1,
            _ => 2,
        });
        let mut coords = Vec::new();
        for c in [arguments.src_c0, arguments.src_c1, arguments.src_c2, arguments.src_c3] {
            coords.push(self.i32_value(c)?);
        }
        let mut offsets = Vec::new();
        for o in [arguments.src_o0, arguments.src_o1, arguments.src_o2] {
            offsets.push(self.i32_value(o)?);
        }
        let has_ref = self.const_u32(arguments.src_ref.is_some() as u64);
        let reference = self.i32_value(arguments.src_ref)?;
        let result = match data.op {
            ast::TexOp::Tex => {
                let mut args = vec![
                    tex,
                    sampler,
                    geometry,
                    self.const_u32(data.lod_mode as u64),
                    self.const_u32((data.ctype == ast::ScalarType::F32) as u64),
                    dtype,
                ];
                args.extend(coords);
                // the lod has the type of the coordinates
                let lod = if data.ctype == ast::ScalarType::S32 && arguments.src_lod.is_some() {
                    let i = self.i32_value(arguments.src_lod)?;
                    unsafe { LLVMBuildSIToFP(self.builder, i, LLVMFloatTypeInContext(self.context), LLVM_UNNAMED.as_ptr()) }
                } else {
                    self.f32_value(arguments.src_lod)?
                };
                args.push(lod);
                for g in [
                    arguments.src_dx0,
                    arguments.src_dx1,
                    arguments.src_dx2,
                    arguments.src_dy0,
                    arguments.src_dy1,
                    arguments.src_dy2,
                ] {
                    args.push(self.f32_value(g)?);
                }
                args.extend(offsets);
                args.extend([has_ref, reference]);
                // the texture unit may sample it (on a texture with hardware descriptors): not bit exact for
                // linear filtering, so not with --strict-fp
                let unit = !self.strict_fp
                    && matches!(data.geometry, ast::TexGeometry::D1 | ast::TexGeometry::D2)
                    && data.lod_mode == 0
                    && data.ctype == ast::ScalarType::F32
                    && arguments.src_sampler.is_none()
                    && arguments.src_ref.is_none()
                    && [arguments.src_o0, arguments.src_o1, arguments.src_o2].iter().all(|o| o.is_none());
                args.push(self.const_u32(unit as u64));
                self.emit_vector_helper(c"__zluda_ptx_impl_agx_tex", ast::ScalarType::B32, &args)?
            }
            ast::TexOp::Tld4(comp) => {
                let mut args = vec![tex, sampler, geometry, self.const_u32(comp as u64), dtype];
                args.extend(coords);
                args.extend(&offsets[..2]);
                args.extend([has_ref, reference]);
                self.emit_vector_helper(c"__zluda_ptx_impl_agx_tld4", ast::ScalarType::B32, &args)?
            }
        };
        let _ = i32_type;
        let dsts = [arguments.dst0, arguments.dst1, arguments.dst2, arguments.dst3];
        if matches!(data.dtype, ast::ScalarType::F16 | ast::ScalarType::F16x2) {
            // the f32 results rounded to f16: one per register, or pairs of them (.v2.f16x2: dst2 and dst3 unused)
            let half = get_scalar_type(self.context, ast::ScalarType::F16);
            let f32_type = get_scalar_type(self.context, ast::ScalarType::F32);
            let mut halves = Vec::new();
            for i in 0..4u64 {
                let b = self.builder;
                let h = unsafe {
                    let e = LLVMBuildExtractElement(b, result, LLVMConstInt(i32_type, i, 0), LLVM_UNNAMED.as_ptr());
                    let f = LLVMBuildBitCast(b, e, f32_type, LLVM_UNNAMED.as_ptr());
                    LLVMBuildFPTrunc(b, f, half, LLVM_UNNAMED.as_ptr())
                };
                halves.push(h);
            }
            if data.dtype == ast::ScalarType::F16 {
                for (dst, h) in dsts.into_iter().zip(halves) {
                    self.resolver.register(dst, h);
                }
            } else {
                let pair_type = get_scalar_type(self.context, ast::ScalarType::F16x2);
                for (k, dst) in dsts.into_iter().enumerate() {
                    let v = if k < 2 {
                        unsafe {
                            let v = LLVMBuildInsertElement(self.builder, LLVMGetPoison(pair_type), halves[2 * k], LLVMConstInt(i32_type, 0, 0), LLVM_UNNAMED.as_ptr());
                            LLVMBuildInsertElement(self.builder, v, halves[2 * k + 1], LLVMConstInt(i32_type, 1, 0), LLVM_UNNAMED.as_ptr())
                        }
                    } else {
                        unsafe { LLVMGetPoison(pair_type) }
                    };
                    self.resolver.register(dst, v);
                }
            }
        } else {
            for (i, dst) in dsts.into_iter().enumerate() {
                self.vector_element(result, i as u32, dst, data.dtype)?;
            }
        }
        // every level is resident
        if let Some(pred) = arguments.dst_pred {
            let one = unsafe { LLVMConstInt(LLVMInt1TypeInContext(self.context), 1, 0) };
            self.resolver.register(pred, one);
        }
        Ok(())
    }

    fn emit_tex_query(
        &mut self,
        data: ast::TexQueryDetails,
        arguments: ast::TexQueryArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let handle = self.descriptor(Some(arguments.src))?;
        let field = self.const_u32(data.field as u64);
        let name = if data.surface { c"__zluda_ptx_impl_agx_suq" } else { c"__zluda_ptx_impl_agx_txq" };
        let Some(level) = arguments.src_level else {
            self.emit_agx_helper(name, Some(arguments.dst), Some(ast::ScalarType::B32), &[handle, field])?;
            return Ok(());
        };
        // the size of a mipmap level: max(1, size >> level)
        let size = self.emit_agx_helper(name, None, Some(ast::ScalarType::B32), &[handle, field])?;
        let level = self.i32_value(Some(level))?;
        let i32_type = unsafe { LLVMInt32TypeInContext(self.context) };
        let shifted = unsafe {
            let shift = LLVMBuildAnd(self.builder, level, LLVMConstInt(i32_type, 31, 0), LLVM_UNNAMED.as_ptr());
            LLVMBuildLShr(self.builder, size, shift, LLVM_UNNAMED.as_ptr())
        };
        let one = unsafe { LLVMConstInt(i32_type, 1, 0) };
        self.emit_intrinsic(c"llvm.umax.i32", Some(arguments.dst), vec![&ast::ScalarType::U32.into()], vec![(shifted, i32_type), (one, i32_type)])?;
        Ok(())
    }

    fn surface_coords(&mut self, cs: [Option<SpirvWord>; 4]) -> Result<Vec<LLVMValueRef>, TranslateError> {
        cs.into_iter().map(|c| self.i32_value(c)).collect()
    }

    fn i64_value(&mut self, id: SpirvWord) -> Result<LLVMValueRef, TranslateError> {
        let i64_type = unsafe { LLVMInt64TypeInContext(self.context) };
        let v = self.resolver.value(id)?;
        let ty = unsafe { LLVMTypeOf(v) };
        Ok(unsafe {
            match LLVMGetTypeKind(ty) {
                LLVMTypeKind::LLVMFloatTypeKind => {
                    let i = LLVMBuildBitCast(self.builder, v, LLVMInt32TypeInContext(self.context), LLVM_UNNAMED.as_ptr());
                    LLVMBuildZExt(self.builder, i, i64_type, LLVM_UNNAMED.as_ptr())
                }
                LLVMTypeKind::LLVMDoubleTypeKind => LLVMBuildBitCast(self.builder, v, i64_type, LLVM_UNNAMED.as_ptr()),
                LLVMTypeKind::LLVMHalfTypeKind | LLVMTypeKind::LLVMBFloatTypeKind => {
                    let i = LLVMBuildBitCast(self.builder, v, LLVMInt16TypeInContext(self.context), LLVM_UNNAMED.as_ptr());
                    LLVMBuildZExt(self.builder, i, i64_type, LLVM_UNNAMED.as_ptr())
                }
                _ => LLVMBuildZExtOrBitCast(self.builder, v, i64_type, LLVM_UNNAMED.as_ptr()),
            }
        })
    }

    fn emit_suld(
        &mut self,
        data: ast::SurfaceDetails,
        arguments: ast::SuldAgxArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let surface = self.descriptor(Some(arguments.src_surf))?;
        let mut args = vec![
            surface,
            self.const_u32(Self::geometry_code(data.geometry)),
            self.const_u32(data.mode as u64),
            self.const_u32(data.vector as u64),
            self.const_u32(data.type_.size_of() as u64),
        ];
        args.extend(self.surface_coords([arguments.src_c0, arguments.src_c1, arguments.src_c2, arguments.src_c3])?);
        let result = self.emit_vector_helper(c"__zluda_ptx_impl_agx_suld_b", ast::ScalarType::B64, &args)?;
        let dsts = [Some(arguments.dst0), arguments.dst1, arguments.dst2, arguments.dst3];
        for (i, dst) in dsts.into_iter().enumerate() {
            if let Some(dst) = dst {
                self.vector_element(result, i as u32, dst, data.type_)?;
            }
        }
        Ok(())
    }

    fn emit_sust(
        &mut self,
        data: ast::SurfaceDetails,
        arguments: ast::SustAgxArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let surface = self.descriptor(Some(arguments.src_surf))?;
        let mut args = vec![surface, self.const_u32(Self::geometry_code(data.geometry)), self.const_u32(data.mode as u64)];
        args.push(self.const_u32(data.vector as u64));
        let values = [Some(arguments.src_v0), arguments.src_v1, arguments.src_v2, arguments.src_v3];
        if data.formatted {
            args.extend(self.surface_coords([arguments.src_c0, arguments.src_c1, arguments.src_c2, arguments.src_c3])?);
            for v in values {
                args.push(self.i32_value(v)?);
            }
            self.emit_agx_helper(c"__zluda_ptx_impl_agx_sust_p", None, None, &args)?;
        } else {
            args.push(self.const_u32(data.type_.size_of() as u64));
            args.extend(self.surface_coords([arguments.src_c0, arguments.src_c1, arguments.src_c2, arguments.src_c3])?);
            for v in values {
                args.push(match v {
                    Some(v) => self.i64_value(v)?,
                    None => unsafe { LLVMConstInt(LLVMInt64TypeInContext(self.context), 0, 0) },
                });
            }
            self.emit_agx_helper(c"__zluda_ptx_impl_agx_sust_b", None, None, &args)?;
        }
        Ok(())
    }

    fn emit_sured(
        &mut self,
        data: ast::SurfaceDetails,
        arguments: ast::SuredAgxArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let surface = self.descriptor(Some(arguments.src_surf))?;
        let bytes = data.type_.size_of() as u64;
        let mut args = vec![
            surface,
            self.const_u32(Self::geometry_code(data.geometry)),
            self.const_u32(data.mode as u64),
            self.const_u32(data.formatted as u64),
            self.const_u32(bytes),
        ];
        args.extend(self.surface_coords([arguments.src_c0, arguments.src_c1, arguments.src_c2, arguments.src_c3])?);
        let value = self.i64_value(arguments.src_v)?;
        // signed: .s32 / .s64, or (.p) a signed integer surface format
        let signed = if data.formatted {
            let s = self.emit_agx_helper(c"__zluda_ptx_impl_agx_surface_signed", None, Some(ast::ScalarType::B32), &[surface])?;
            s
        } else {
            self.const_u32(matches!(data.type_, ast::ScalarType::S32 | ast::ScalarType::S64) as u64)
        };
        let op = self.const_u32(match data.op {
            Some(ast::AtomicOp::Add) => 0,
            Some(ast::AtomicOp::And) => 1,
            Some(ast::AtomicOp::Or) => 2,
            Some(ast::AtomicOp::UnsignedMin) => 3,
            Some(ast::AtomicOp::UnsignedMax) => 4,
            _ => return Err(error_unreachable()),
        });
        args.extend([op, signed, value]);
        self.emit_agx_helper(c"__zluda_ptx_impl_agx_sured", None, None, &args)?;
        Ok(())
    }

    // The lowest active lane of the warp performs the warp-collective tcgen05 operations
    fn emit_is_leader(&mut self) -> Result<(LLVMValueRef, LLVMValueRef), TranslateError> {
        let i32_type = unsafe { LLVMInt32TypeInContext(self.context) };
        let active = self.emit_active_lanes()?;
        let false_ = unsafe { LLVMConstInt(LLVMInt1TypeInContext(self.context), 0, 0) };
        let leader = self.emit_int_intrinsic("llvm.cttz", i32_type, &[active, false_]);
        let lane = self.emit_lane_id()?;
        let is = unsafe { LLVMBuildICmp(self.builder, LLVMIntPredicate::LLVMIntEQ, lane, leader, LLVM_UNNAMED.as_ptr()) };
        let is = unsafe { LLVMBuildZExt(self.builder, is, i32_type, LLVM_UNNAMED.as_ptr()) };
        Ok((leader, is))
    }

    fn emit_tcgen05_alloc(
        &mut self,
        data: ast::Tcgen05AllocDetails,
        arguments: ast::Tcgen05AllocArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let ncols = self.i32_value(Some(arguments.src_ncols))?;
        let (leader, is_leader) = self.emit_is_leader()?;
        let exclusive = self.const_u32(data.exclusive as u64);
        let column = self.emit_agx_helper(
            c"__zluda_ptx_impl_agx_tmem_alloc",
            None,
            Some(ast::ScalarType::B32),
            &[ncols, is_leader, exclusive],
        )?;
        let i32_type = unsafe { LLVMInt32TypeInContext(self.context) };
        let column = self.emit_intrinsic(
            c"llvm.agx.simd.shuffle.i32",
            None,
            vec![&ast::Type::Scalar(ast::ScalarType::B32)],
            vec![(column, i32_type), (leader, i32_type)],
        )?;
        let lane = self.emit_lane_id()?;
        self.emit_agx_helper(c"__zluda_ptx_impl_agx_tmem_zero", None, None, &[column, ncols, lane])?;
        let address = self.mbarrier_pointer(data.space, arguments.src_addr)?;
        unsafe { LLVMBuildStore(self.builder, column, address) };
        Ok(())
    }

    fn emit_tcgen05_dealloc(
        &mut self,
        data: ast::Tcgen05AllocDetails,
        arguments: ast::Tcgen05DeallocArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let taddr = self.i32_value(Some(arguments.src_taddr))?;
        let ncols = self.i32_value(Some(arguments.src_ncols))?;
        let (_, is_leader) = self.emit_is_leader()?;
        let exclusive = self.const_u32(data.exclusive as u64);
        self.emit_agx_helper(c"__zluda_ptx_impl_agx_tmem_dealloc", None, None, &[taddr, ncols, is_leader, exclusive])?;
        Ok(())
    }

    // The tensor memory word (lane, column) of register i of this thread (ISA 9.7.17.2.3, Figures 183-187: in the
    // 16-lane shapes thread t covers lanes t / 4 and t / 4 + 8), as a pointer. With `half`, the 16-bit half of a
    // .pack::16b / .unpack::16b register (Figure 193): the 32-bit cell's column relative to taddr doubles in the
    // 16xNb shapes, and is that of element 2i + half in .32x32b and .16x32bx2.
    fn tmem_cell(
        &mut self,
        base: LLVMValueRef,
        shape: ast::Tcgen05Shape,
        taddr: LLVMValueRef,
        thread: LLVMValueRef,
        split: LLVMValueRef,
        i: u32,
        half: Option<u32>,
    ) -> LLVMValueRef {
        let b = self.builder;
        let i32_type = unsafe { LLVMInt32TypeInContext(self.context) };
        let c = |v: u32| unsafe { LLVMConstInt(i32_type, v as u64, 0) };
        let wide = !matches!(shape, ast::Tcgen05Shape::S32x32b | ast::Tcgen05Shape::S16x32bx2);
        let i = match half {
            Some(h) if !wide => 2 * i + h,
            _ => i,
        };
        unsafe {
            let lane0 = LLVMBuildLShr(b, taddr, c(16), LLVM_UNNAMED.as_ptr());
            let column0 = LLVMBuildAnd(b, taddr, c(0xffff), LLVM_UNNAMED.as_ptr());
            let t16 = LLVMBuildAnd(b, thread, c(15), LLVM_UNNAMED.as_ptr());
            let half16 = LLVMBuildLShr(b, thread, c(4), LLVM_UNNAMED.as_ptr());
            let g = LLVMBuildLShr(b, thread, c(2), LLVM_UNNAMED.as_ptr());
            let t4 = LLVMBuildAnd(b, thread, c(3), LLVM_UNNAMED.as_ptr());
            let (lane, column) = match shape {
                ast::Tcgen05Shape::S32x32b => (thread, c(i)),
                ast::Tcgen05Shape::S16x64b => {
                    let odd = LLVMBuildAnd(b, thread, c(1), LLVM_UNNAMED.as_ptr());
                    let odd = LLVMBuildShl(b, odd, c(3), LLVM_UNNAMED.as_ptr());
                    let col = LLVMBuildLShr(b, t4, c(1), LLVM_UNNAMED.as_ptr());
                    (LLVMBuildAdd(b, g, odd, LLVM_UNNAMED.as_ptr()), LLVMBuildAdd(b, c(2 * i), col, LLVM_UNNAMED.as_ptr()))
                }
                ast::Tcgen05Shape::S16x128b => (
                    LLVMBuildAdd(b, g, c(8 * (i % 2)), LLVM_UNNAMED.as_ptr()),
                    LLVMBuildAdd(b, c(4 * (i / 2)), t4, LLVM_UNNAMED.as_ptr()),
                ),
                ast::Tcgen05Shape::S16x256b => {
                    let col = LLVMBuildShl(b, t4, c(1), LLVM_UNNAMED.as_ptr());
                    (
                        LLVMBuildAdd(b, g, c(8 * ((i % 4) / 2)), LLVM_UNNAMED.as_ptr()),
                        LLVMBuildAdd(b, c(8 * (i / 4) + i % 2), col, LLVM_UNNAMED.as_ptr()),
                    )
                }
                ast::Tcgen05Shape::S16x32bx2 => {
                    let h = LLVMBuildMul(b, half16, split, LLVM_UNNAMED.as_ptr());
                    (t16, LLVMBuildAdd(b, c(i), h, LLVM_UNNAMED.as_ptr()))
                }
            };
            let column = match half {
                Some(h) if wide => {
                    let x = LLVMBuildShl(b, column, c(1), LLVM_UNNAMED.as_ptr());
                    LLVMBuildAdd(b, x, c(h), LLVM_UNNAMED.as_ptr())
                }
                _ => column,
            };
            let lane = LLVMBuildAdd(b, lane0, lane, LLVM_UNNAMED.as_ptr());
            let column = LLVMBuildAdd(b, column0, column, LLVM_UNNAMED.as_ptr());
            let lane = LLVMBuildAnd(b, lane, c(127), LLVM_UNNAMED.as_ptr());
            let column = LLVMBuildAnd(b, column, c(crate::pass::agx_matrix::TMEM_COLS - 1), LLVM_UNNAMED.as_ptr());
            let word = LLVMBuildMul(b, lane, c(crate::pass::agx_matrix::TMEM_COLS), LLVM_UNNAMED.as_ptr());
            let word = LLVMBuildAdd(b, word, column, LLVM_UNNAMED.as_ptr());
            let i64_type = LLVMInt64TypeInContext(self.context);
            let offset = LLVMBuildZExt(b, word, i64_type, LLVM_UNNAMED.as_ptr());
            let offset = LLVMBuildShl(b, offset, LLVMConstInt(i64_type, 2, 0), LLVM_UNNAMED.as_ptr());
            let address = LLVMBuildAdd(b, base, offset, LLVM_UNNAMED.as_ptr());
            let ptr = get_pointer_type(self.context, ast::StateSpace::Global).unwrap();
            LLVMBuildIntToPtr(b, address, ptr, LLVM_UNNAMED.as_ptr())
        }
    }

    fn tmem_access(
        &mut self,
        taddr: SpirvWord,
        offset: Option<SpirvWord>,
    ) -> Result<(LLVMValueRef, LLVMValueRef, LLVMValueRef, LLVMValueRef), TranslateError> {
        let base = self.emit_agx_helper(c"__zluda_ptx_impl_agx_tmem_base", None, Some(ast::ScalarType::B64), &[])?;
        let taddr = self.i32_value(Some(taddr))?;
        let split = self.i32_value(offset)?;
        let thread = self.emit_lane_id()?;
        Ok((base, taddr, thread, split))
    }

    fn emit_tcgen05_ld(
        &mut self,
        data: ast::Tcgen05LdStDetails,
        arguments: ast::Tcgen05LdArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let (base, taddr, thread, split) = self.tmem_access(arguments.src_taddr, arguments.src_offset)?;
        let i32_type = unsafe { LLVMInt32TypeInContext(self.context) };
        let mut words = Vec::new();
        for i in 0..data.registers as u32 {
            let word = if data.packed {
                let lo = self.tmem_cell(base, data.shape, taddr, thread, split, i, Some(0));
                let hi = self.tmem_cell(base, data.shape, taddr, thread, split, i, Some(1));
                unsafe {
                    let lo = LLVMBuildLoad2(self.builder, i32_type, lo, LLVM_UNNAMED.as_ptr());
                    let hi = LLVMBuildLoad2(self.builder, i32_type, hi, LLVM_UNNAMED.as_ptr());
                    let lo = LLVMBuildAnd(self.builder, lo, LLVMConstInt(i32_type, 0xffff, 0), LLVM_UNNAMED.as_ptr());
                    let hi = LLVMBuildShl(self.builder, hi, LLVMConstInt(i32_type, 16, 0), LLVM_UNNAMED.as_ptr());
                    LLVMBuildOr(self.builder, lo, hi, LLVM_UNNAMED.as_ptr())
                }
            } else {
                let p = self.tmem_cell(base, data.shape, taddr, thread, split, i, None);
                unsafe { LLVMBuildLoad2(self.builder, i32_type, p, LLVM_UNNAMED.as_ptr()) }
            };
            words.push(word);
        }
        self.register_value(arguments.dst, words);
        Ok(())
    }

    fn emit_tcgen05_ld_red(
        &mut self,
        data: ast::Tcgen05LdRedDetails,
        arguments: ast::Tcgen05LdRedArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let (base, taddr, thread, split) = self.tmem_access(arguments.src_taddr, None)?;
        let i32_type = unsafe { LLVMInt32TypeInContext(self.context) };
        let mut values = Vec::new();
        for i in 0..data.loaded as u32 {
            let p = self.tmem_cell(base, data.shape, taddr, thread, split, i, None);
            values.push(unsafe { LLVMBuildLoad2(self.builder, i32_type, p, LLVM_UNNAMED.as_ptr()) });
        }
        let c = |this: &Self, v: u64| this.const_u32(v);
        if let Some(dst_red) = arguments.dst_red {
            let mode = c(self, data.min as u64 | (data.type_ as u64) << 1 | (data.nan as u64) << 3);
            let abs = |this: &mut Self, v: LLVMValueRef| {
                if data.abs {
                    unsafe { LLVMBuildAnd(this.builder, v, this.const_u32(0x7fff_ffff), LLVM_UNNAMED.as_ptr()) }
                } else {
                    v
                }
            };
            let mut acc = abs(self, values[0]);
            for v in values.iter().skip(1) {
                let x = abs(self, *v);
                acc = self.emit_agx_helper(c"__zluda_ptx_impl_agx_tcgen05_red", None, Some(ast::ScalarType::B32), &[mode, acc, x])?;
            }
            self.resolver.register(dst_red, acc);
        }
        if data.spcompress {
            // ptx-exec-spc.k spcFin: the kept values in order, their 2-bit indices packed from bit 0
            let op = c(self, 2 * data.min as u64 + data.abs as u64);
            let mut kept = Vec::new();
            let mut meta = c(self, 0);
            for (g, group) in values.chunks(4).enumerate() {
                if group.len() < 4 {
                    break;
                }
                let indices = self.emit_agx_helper(
                    c"__zluda_ptx_impl_agx_tcgen05_spc_f32",
                    None,
                    Some(ast::ScalarType::B32),
                    &[op, group[0], group[1], group[2], group[3]],
                )?;
                for slot in 0..2 {
                    let index = unsafe {
                        let x = LLVMBuildLShr(self.builder, indices, c(self, 2 * slot), LLVM_UNNAMED.as_ptr());
                        LLVMBuildAnd(self.builder, x, c(self, 3), LLVM_UNNAMED.as_ptr())
                    };
                    let mut value = group[0];
                    for j in 1..4 {
                        let is = unsafe {
                            LLVMBuildICmp(self.builder, LLVMIntPredicate::LLVMIntEQ, index, c(self, j), LLVM_UNNAMED.as_ptr())
                        };
                        value = unsafe { LLVMBuildSelect(self.builder, is, group[j as usize], value, LLVM_UNNAMED.as_ptr()) };
                    }
                    kept.push(value);
                    let bit = 2 * (2 * g as u64 + slot);
                    if bit < 32 {
                        let shifted = unsafe { LLVMBuildShl(self.builder, index, c(self, bit), LLVM_UNNAMED.as_ptr()) };
                        meta = unsafe { LLVMBuildOr(self.builder, meta, shifted, LLVM_UNNAMED.as_ptr()) };
                    }
                }
            }
            let mut m_words = vec![meta];
            m_words.resize(data.dst_registers as usize, c(self, 0));
            m_words.truncate(data.dst_registers as usize);
            self.register_value(arguments.dst, m_words);
            if let Some(dst_c) = arguments.dst_c {
                kept.resize(data.c_registers as usize, c(self, 0));
                self.register_value(dst_c, kept);
            }
        } else {
            values.resize(data.dst_registers as usize, c(self, 0));
            self.register_value(arguments.dst, values);
        }
        Ok(())
    }

    fn emit_tcgen05_st(
        &mut self,
        data: ast::Tcgen05LdStDetails,
        arguments: ast::Tcgen05StArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let values = self.register_words(arguments.src_values, data.registers)?;
        let (base, taddr, thread, split) = self.tmem_access(arguments.src_taddr, arguments.src_offset)?;
        let i32_type = unsafe { LLVMInt32TypeInContext(self.context) };
        for (i, value) in values.into_iter().enumerate() {
            let i = i as u32;
            if data.packed {
                let lo = self.tmem_cell(base, data.shape, taddr, thread, split, i, Some(0));
                let hi = self.tmem_cell(base, data.shape, taddr, thread, split, i, Some(1));
                unsafe {
                    let low = LLVMBuildAnd(self.builder, value, LLVMConstInt(i32_type, 0xffff, 0), LLVM_UNNAMED.as_ptr());
                    let high = LLVMBuildLShr(self.builder, value, LLVMConstInt(i32_type, 16, 0), LLVM_UNNAMED.as_ptr());
                    LLVMBuildStore(self.builder, low, lo);
                    LLVMBuildStore(self.builder, high, hi);
                }
            } else {
                let p = self.tmem_cell(base, data.shape, taddr, thread, split, i, None);
                unsafe { LLVMBuildStore(self.builder, value, p) };
            }
        }
        Ok(())
    }

    // the 32-bit registers of a scalar or vector value
    fn register_words(&mut self, id: SpirvWord, count: u8) -> Result<Vec<LLVMValueRef>, TranslateError> {
        let value = self.resolver.value(id)?;
        let i32_type = unsafe { LLVMInt32TypeInContext(self.context) };
        if count == 1 {
            return Ok(vec![self.i32_value(Some(id))?]);
        }
        Ok((0..count as u64)
            .map(|i| unsafe {
                let element = LLVMBuildExtractElement(self.builder, value, LLVMConstInt(i32_type, i, 0), LLVM_UNNAMED.as_ptr());
                LLVMBuildZExtOrBitCast(self.builder, element, i32_type, LLVM_UNNAMED.as_ptr())
            })
            .collect())
    }

    fn register_value(&mut self, dst: SpirvWord, words: Vec<LLVMValueRef>) {
        let i32_type = unsafe { LLVMInt32TypeInContext(self.context) };
        let value = if words.len() == 1 {
            words[0]
        } else {
            let mut vector = unsafe { LLVMGetUndef(LLVMVectorType(i32_type, words.len() as u32)) };
            for (i, word) in words.into_iter().enumerate() {
                vector = unsafe {
                    LLVMBuildInsertElement(self.builder, vector, word, LLVMConstInt(i32_type, i as u64, 0), LLVM_UNNAMED.as_ptr())
                };
            }
            vector
        };
        self.resolver.register(dst, value);
    }

    // bits [offset, offset + width) of a little-endian vector of 32-bit words (width <= 64; missing words are zero)
    fn extract_bits(&mut self, words: &[LLVMValueRef], offset: u32, width: u32) -> LLVMValueRef {
        let i64_type = unsafe { LLVMInt64TypeInContext(self.context) };
        let b = self.builder;
        let mut result = unsafe { LLVMConstInt(i64_type, 0, 0) };
        for (k, word) in words.iter().enumerate() {
            let start = k as i64 * 32;
            if start + 32 <= offset as i64 || start >= (offset + width) as i64 {
                continue;
            }
            let shift = start - offset as i64;
            unsafe {
                let wide = LLVMBuildZExt(b, *word, i64_type, LLVM_UNNAMED.as_ptr());
                let piece = if shift >= 0 {
                    LLVMBuildShl(b, wide, LLVMConstInt(i64_type, shift as u64, 0), LLVM_UNNAMED.as_ptr())
                } else {
                    LLVMBuildLShr(b, wide, LLVMConstInt(i64_type, (-shift) as u64, 0), LLVM_UNNAMED.as_ptr())
                };
                result = LLVMBuildOr(b, result, piece, LLVM_UNNAMED.as_ptr());
            }
        }
        if width < 64 {
            result = unsafe { LLVMBuildAnd(b, result, LLVMConstInt(i64_type, (1u64 << width) - 1, 0), LLVM_UNNAMED.as_ptr()) };
        }
        result
    }

    // ors the low `width` bits of value (i64) into the words at offset
    fn deposit_bits(&mut self, words: &mut [LLVMValueRef], offset: u32, width: u32, value: LLVMValueRef) {
        let i64_type = unsafe { LLVMInt64TypeInContext(self.context) };
        let i32_type = unsafe { LLVMInt32TypeInContext(self.context) };
        let b = self.builder;
        let value = if width < 64 {
            unsafe { LLVMBuildAnd(b, value, LLVMConstInt(i64_type, (1u64 << width) - 1, 0), LLVM_UNNAMED.as_ptr()) }
        } else {
            value
        };
        for (k, word) in words.iter_mut().enumerate() {
            let start = k as i64 * 32;
            if start + 32 <= offset as i64 || start >= (offset + width) as i64 {
                continue;
            }
            let shift = start - offset as i64;
            unsafe {
                let piece = if shift >= 0 {
                    LLVMBuildLShr(b, value, LLVMConstInt(i64_type, shift as u64, 0), LLVM_UNNAMED.as_ptr())
                } else {
                    LLVMBuildShl(b, value, LLVMConstInt(i64_type, (-shift) as u64, 0), LLVM_UNNAMED.as_ptr())
                };
                let piece = LLVMBuildTrunc(b, piece, i32_type, LLVM_UNNAMED.as_ptr());
                *word = LLVMBuildOr(b, *word, piece, LLVM_UNNAMED.as_ptr());
            }
        }
    }

    // spcompress: one helper call per 64-bit iteration (zluda_ptx_impl_agx_cvt.c)
    fn emit_spcompress(
        &mut self,
        data: ast::SparseDetails,
        arguments: ast::SpcompressArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let source = self.register_words(arguments.src_data, data.data)?;
        let desc = self.i32_value(Some(arguments.src_desc))?;
        let es = data.element_bits as u32;
        let ib = data.index_bits as u32;
        // indices per iteration: two of every four elements
        let index_width = 64 / es / 2 * ib;
        let mut m = vec![self.const_u32(0); data.mdata as usize];
        let mut c = vec![self.const_u32(0); data.cdata as usize];
        let i64_type = unsafe { LLVMInt64TypeInContext(self.context) };
        let i32_type = unsafe { LLVMInt32TypeInContext(self.context) };
        for n in 0..data.iterations as u32 {
            let bits = self.extract_bits(&source, 64 * n, 64);
            let (lo, hi) = unsafe {
                (
                    LLVMBuildTrunc(self.builder, bits, i32_type, LLVM_UNNAMED.as_ptr()),
                    LLVMBuildTrunc(
                        self.builder,
                        LLVMBuildLShr(self.builder, bits, LLVMConstInt(i64_type, 32, 0), LLVM_UNNAMED.as_ptr()),
                        i32_type,
                        LLVM_UNNAMED.as_ptr(),
                    ),
                )
            };
            let es_value = self.const_u32(es as u64);
            let ib_value = self.const_u32(ib as u64);
            let result = self.emit_agx_helper(
                c"__zluda_ptx_impl_agx_spc_iter",
                None,
                Some(ast::ScalarType::B64),
                &[desc, es_value, ib_value, lo, hi],
            )?;
            self.deposit_bits(&mut c, 32 * n, 32, result);
            let indices = unsafe { LLVMBuildLShr(self.builder, result, LLVMConstInt(i64_type, 32, 0), LLVM_UNNAMED.as_ptr()) };
            self.deposit_bits(&mut m, index_width * n, index_width, indices);
        }
        self.register_value(arguments.dst_m, m);
        self.register_value(arguments.dst_c, c);
        Ok(())
    }

    fn emit_spdecompress(
        &mut self,
        data: ast::SparseDetails,
        arguments: ast::SpdecompressArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let m_words = self.register_words(arguments.src_m, data.mdata)?;
        let c_words = self.register_words(arguments.src_c, data.cdata)?;
        let es = data.element_bits as u32;
        let ib = data.index_bits as u32;
        let (ns, nt) = (data.kept as u32, data.group as u32);
        let mut d = vec![self.const_u32(0); data.data as usize];
        let i32_type = unsafe { LLVMInt32TypeInContext(self.context) };
        for n in 0..data.iterations as u32 {
            let c = self.extract_bits(&c_words, n * ns * es, ns * es);
            let m = self.extract_bits(&m_words, n * ns * ib, ns * ib);
            let m = unsafe { LLVMBuildTrunc(self.builder, m, i32_type, LLVM_UNNAMED.as_ptr()) };
            let args = [es, ib, ns, nt].map(|v| self.const_u32(v as u64));
            let result = self.emit_agx_helper(
                c"__zluda_ptx_impl_agx_spd_iter",
                None,
                Some(ast::ScalarType::B64),
                &[args[0], args[1], args[2], args[3], c, m],
            )?;
            self.deposit_bits(&mut d, n * nt * es, nt * es, result);
        }
        self.register_value(arguments.dst, d);
        Ok(())
    }

    // in software (zluda_ptx_impl_agx_cvt.c); the scale defaults to 2^0 for each element
    fn emit_cvt_narrow(
        &mut self,
        data: ast::CvtNarrowDetails,
        arguments: ast::CvtNarrowArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let kind = self.const_u32(data.kind as u64);
        let flags = self.const_u32(data.flags as u64);
        let other = self.const_u32(data.other as u64);
        let a = self.i32_value(Some(arguments.src1))?;
        let scale = match arguments.src_scale {
            Some(scale) => self.i32_value(Some(scale))?,
            None => self.const_u32(0x7f7f),
        };
        if data.widen {
            self.emit_agx_helper(
                c"__zluda_ptx_impl_agx_cvt_widen",
                Some(arguments.dst),
                Some(ast::ScalarType::B32),
                &[kind, other, flags, a, scale],
            )?;
        } else {
            let b = self.i32_value(arguments.src2)?;
            if data.kind == 4 {
                let x = self.emit_agx_helper(
                    c"__zluda_ptx_impl_agx_cvt_narrow",
                    None,
                    Some(ast::ScalarType::B16),
                    &[kind, flags, other, a, b, scale],
                )?;
                let i8_type = unsafe { LLVMInt8TypeInContext(self.context) };
                let x = unsafe { LLVMBuildTrunc(self.builder, x, i8_type, LLVM_UNNAMED.as_ptr()) };
                self.resolver.register(arguments.dst, x);
            } else {
                self.emit_agx_helper(
                    c"__zluda_ptx_impl_agx_cvt_narrow",
                    Some(arguments.dst),
                    Some(ast::ScalarType::B16),
                    &[kind, flags, other, a, b, scale],
                )?;
            }
        }
        Ok(())
    }

    fn emit_cvt_rs(&mut self, data: ast::CvtRsDetails, arguments: ast::CvtRsArgs<SpirvWord>) -> Result<(), TranslateError> {
        let kind = self.const_u32(data.kind as u64);
        let flags = self.const_u32(data.flags as u64);
        let a = self.i32_value(Some(arguments.src_a))?;
        let b = self.i32_value(Some(arguments.src_b))?;
        let e = self.i32_value(arguments.src_e)?;
        let f = self.i32_value(arguments.src_f)?;
        let rbits = self.i32_value(Some(arguments.src_rbits))?;
        let x = self.emit_agx_helper(c"__zluda_ptx_impl_agx_cvt_rs", None, Some(ast::ScalarType::B32), &[kind, flags, a, b, e, f, rbits])?;
        let x = if data.kind == 4 {
            let i16_type = unsafe { LLVMInt16TypeInContext(self.context) };
            unsafe { LLVMBuildTrunc(self.builder, x, i16_type, LLVM_UNNAMED.as_ptr()) }
        } else {
            x
        };
        self.resolver.register(arguments.dst, x);
        Ok(())
    }

    // the endpoint table of zluda_ptx_impl_agx_cvt.c; the helpers complete the transfers on the mbarrier
    fn emit_fabric(&mut self, data: ast::FabricDetails, arguments: ast::FabricArgs<SpirvWord>) -> Result<(), TranslateError> {
        if data.kind == 0 {
            return Ok(());
        }
        let le = self.i32_value(arguments.src_le)?;
        let off = self.i64_value(arguments.src_off.ok_or_else(error_unreachable)?)?;
        if data.kind >= 6 {
            let tm = self.i64_value(arguments.src_tm.ok_or_else(error_unreachable)?)?;
            let tm = unsafe {
                LLVMBuildIntToPtr(self.builder, tm, get_pointer_type(self.context, ast::StateSpace::Generic)?, LLVM_UNNAMED.as_ptr())
            };
            let dims = [arguments.src_d0, arguments.src_d1, arguments.src_d2, arguments.src_d3, arguments.src_d4];
            let nd = self.const_u32(dims.iter().filter(|d| d.is_some()).count() as u64);
            let mut args = vec![tm, le, off, nd];
            for x in dims.into_iter().chain([arguments.src_c0, arguments.src_c1, arguments.src_c2, arguments.src_c3, arguments.src_c4]) {
                args.push(self.i32_value(x)?);
            }
            args.push(self.resolver.value(arguments.src_mem.ok_or_else(error_unreachable)?)?);
            args.push(self.i32_value(arguments.src_size)?);
            args.push(self.const_u32((data.flags & 1) as u64 | if data.kind == 7 { 2 } else { 0 }));
            args.push(self.mbarrier_pointer(ast::StateSpace::Shared, arguments.src_mbar.ok_or_else(error_unreachable)?)?);
            self.emit_agx_helper(c"__zluda_ptx_impl_agx_fabric_tensor", None, None, &args)?;
            return Ok(());
        }
        let counter = match arguments.src_counter {
            Some(c) => self.i64_value(c)?,
            None => unsafe { LLVMConstInt(LLVMInt64TypeInContext(self.context), 0, 0) },
        };
        let mem = self.resolver.value(arguments.src_mem.ok_or_else(error_unreachable)?)?;
        let mbar = self.mbarrier_pointer(ast::StateSpace::Shared, arguments.src_mbar.ok_or_else(error_unreachable)?)?;
        let size = self.i32_value(arguments.src_size)?;
        let desc = self.const_u32(data.op as u64 | (data.type_ as u64) << 4);
        let flags = self.const_u32(data.flags as u64);
        match data.kind {
            1 => self.emit_agx_helper(c"__zluda_ptx_impl_agx_fabric_get", None, None, &[mem, le, off, size, mbar])?,
            2 => {
                let mask = self.i32_value(arguments.src_mask)?;
                self.emit_agx_helper(c"__zluda_ptx_impl_agx_fabric_put", None, None, &[le, off, counter, mem, size, mask, flags, mbar])?
            }
            3 => self.emit_agx_helper(c"__zluda_ptx_impl_agx_fabric_red", None, None, &[le, off, counter, mem, size, desc, flags, mbar])?,
            4 => {
                let lane = self.emit_lane_id()?;
                self.emit_agx_helper(c"__zluda_ptx_impl_agx_fabric_pullred", None, None, &[mem, le, off, size, desc, lane, mbar])?
            }
            _ => {
                let src = self.resolver.value(arguments.src_mem2.ok_or_else(error_unreachable)?)?;
                self.emit_agx_helper(c"__zluda_ptx_impl_agx_fabric_atom", None, None, &[le, off, mem, src, desc, mbar])?
            }
        };
        Ok(())
    }

    // each element of the vector at its own address, through the window table (zluda_ptx_impl_agx_cvt.c)
    fn emit_multimem(&mut self, data: ast::MultimemDetails, arguments: ast::MultimemArgs<SpirvWord>) -> Result<(), TranslateError> {
        let bytes = data.element_bytes();
        let desc = self.const_u32((data.op as u64) | (data.type_ as u64) << 3 | (data.acc as u64) << 8);
        let address = self.i64_value(arguments.src_addr)?;
        let b = self.builder;
        unsafe {
            let i64_type = LLVMInt64TypeInContext(self.context);
            let i32_type = LLVMInt32TypeInContext(self.context);
            let element_type = LLVMIntTypeInContext(self.context, 8 * bytes);
            let at = |i: u32| LLVMBuildAdd(b, address, LLVMConstInt(i64_type, (i * bytes) as u64, 0), LLVM_UNNAMED.as_ptr());
            if data.kind == 0 {
                let dst = arguments.dst.ok_or_else(error_unreachable)?;
                let mut elements = Vec::new();
                for i in 0..data.vector as u32 {
                    let a = at(i);
                    let v = self.emit_agx_helper(c"__zluda_ptx_impl_agx_mm_ld_reduce", None, Some(ast::ScalarType::B64), &[a, desc])?;
                    elements.push(LLVMBuildTrunc(self.builder, v, element_type, LLVM_UNNAMED.as_ptr()));
                }
                let value = if data.vector == 1 {
                    elements[0]
                } else {
                    let mut v = LLVMGetUndef(LLVMVectorType(element_type, data.vector as u32));
                    for (i, e) in elements.into_iter().enumerate() {
                        v = LLVMBuildInsertElement(self.builder, v, e, LLVMConstInt(i32_type, i as u64, 0), LLVM_UNNAMED.as_ptr());
                    }
                    v
                };
                self.resolver.register(dst, value);
            } else {
                let source = self.resolver.value(arguments.src_value.ok_or_else(error_unreachable)?)?;
                for i in 0..data.vector as u32 {
                    let e = if data.vector == 1 {
                        source
                    } else {
                        LLVMBuildExtractElement(self.builder, source, LLVMConstInt(i32_type, i as u64, 0), LLVM_UNNAMED.as_ptr())
                    };
                    let e = LLVMBuildBitCast(self.builder, e, element_type, LLVM_UNNAMED.as_ptr());
                    let e = LLVMBuildZExt(self.builder, e, i64_type, LLVM_UNNAMED.as_ptr());
                    let a = at(i);
                    if data.kind == 1 {
                        let n = self.const_u32(bytes as u64);
                        self.emit_agx_helper(c"__zluda_ptx_impl_agx_mm_st", None, None, &[a, e, n])?;
                    } else {
                        self.emit_agx_helper(c"__zluda_ptx_impl_agx_mm_red", None, None, &[a, e, desc])?;
                    }
                }
            }
        }
        Ok(())
    }

    // the four elements, exactly and rounded once (zluda_ptx_impl_agx_cvt.c)
    fn emit_alt_fp_x4(&mut self, data: ast::AltFpX4Details, arguments: ast::AltFpX4Args<SpirvWord>) -> Result<(), TranslateError> {
        let op = self.const_u32(data.op as u64);
        let kinds = data.d as u64 | (data.a as u64) << 4 | (data.b as u64) << 8 | (data.c as u64) << 12;
        let kinds = self.const_u32(kinds);
        let satfinite = self.const_u32(data.satfinite as u64);
        let a = self.i32_value(Some(arguments.src_a))?;
        let b = self.i32_value(arguments.src_b)?;
        let c = self.i32_value(Some(arguments.src_c))?;
        self.emit_agx_helper(
            c"__zluda_ptx_impl_agx_altfp_x4",
            Some(arguments.dst),
            Some(ast::ScalarType::B32),
            &[op, kinds, satfinite, a, b, c],
        )?;
        Ok(())
    }

    fn emit_istypep(&mut self, kind: u8, arguments: ast::IstypepArgs<SpirvWord>) -> Result<(), TranslateError> {
        let handle = self.i64_value(arguments.src)?;
        let kind = self.const_u32(kind as u64);
        let is = self.emit_agx_helper(c"__zluda_ptx_impl_agx_istypep", None, Some(ast::ScalarType::B32), &[handle, kind])?;
        let zero = self.const_u32(0);
        let p = unsafe { LLVMBuildICmp(self.builder, LLVMIntPredicate::LLVMIntNE, is, zero, LLVM_UNNAMED.as_ptr()) };
        self.resolver.register(arguments.dst, p);
        Ok(())
    }

    fn emit_tensormap_replace(
        &mut self,
        data: ast::TensormapReplaceDetails,
        arguments: ast::TensormapReplaceArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let tmap = self.resolver.value(arguments.src_tmap)?;
        let tmap = unsafe {
            LLVMBuildAddrSpaceCast(
                self.builder,
                tmap,
                get_pointer_type(self.context, ast::StateSpace::Generic)?,
                LLVM_UNNAMED.as_ptr(),
            )
        };
        let ord = match arguments.src_ord {
            Some(ord) => self.resolver.value(ord)?,
            None => self.const_u32(0),
        };
        let value = self.resolver.value(arguments.src_value)?;
        let value = unsafe {
            LLVMBuildZExtOrBitCast(
                self.builder,
                value,
                LLVMInt64TypeInContext(self.context),
                LLVM_UNNAMED.as_ptr(),
            )
        };
        let field = self.const_u32(data.field as u64);
        self.emit_agx_helper(
            c"__zluda_ptx_impl_agx_tensormap_replace",
            None,
            None,
            &[tmap, field, ord, value],
        )?;
        Ok(())
    }

    fn emit_cp_reduce_async_bulk(
        &mut self,
        data: ast::CpReduceAsyncBulkDetails,
        arguments: ast::CpReduceAsyncBulkArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let _ = arguments.src_cache_policy;
        let to = self.resolver.value(arguments.src_to)?;
        let from = self.resolver.value(arguments.src_from)?;
        let size = self.resolver.value(arguments.src_size)?;
        let op = match data.op {
            ast::AtomicOp::Add => 0,
            ast::AtomicOp::And => 1,
            ast::AtomicOp::Or => 2,
            ast::AtomicOp::Xor => 3,
            ast::AtomicOp::IncrementWrap => 4,
            ast::AtomicOp::DecrementWrap => 5,
            ast::AtomicOp::SignedMin => 6,
            ast::AtomicOp::UnsignedMin => 7,
            ast::AtomicOp::SignedMax => 8,
            ast::AtomicOp::UnsignedMax => 9,
            ast::AtomicOp::FloatAdd => 10,
            ast::AtomicOp::FloatMin => 11,
            ast::AtomicOp::FloatMax => 12,
            ast::AtomicOp::Exchange => return Err(error_unreachable()),
        };
        let name = match (data.type_, data.dst_space) {
            (ast::ScalarType::F16 | ast::ScalarType::BF16, space) => {
                let op = self.const_u32(op | ((data.type_ == ast::ScalarType::BF16) as u64) << 8);
                let name = if space == ast::StateSpace::Global {
                    c"__zluda_ptx_impl_agx_cp_reduce_bulk_h16_global"
                } else {
                    c"__zluda_ptx_impl_agx_cp_reduce_bulk_h16_shared"
                };
                self.emit_agx_helper(name, None, None, &[to, from, size, op])?;
                if let Some(mbarrier) = arguments.src_mbarrier {
                    let mbarrier = self.mbarrier_pointer(data.dst_space, mbarrier)?;
                    self.emit_agx_helper(c"__zluda_ptx_impl_agx_mbarrier_complete_tx", None, None, &[mbarrier, size])?;
                }
                return Ok(());
            }
            (ast::ScalarType::F32, ast::StateSpace::Global) => {
                c"__zluda_ptx_impl_agx_cp_reduce_bulk_f32_global"
            }
            (ast::ScalarType::F32, _) => c"__zluda_ptx_impl_agx_cp_reduce_bulk_f32_shared",
            (ast::ScalarType::F64, ast::StateSpace::Global) => {
                c"__zluda_ptx_impl_agx_cp_reduce_bulk_f64_global"
            }
            (ast::ScalarType::F64, _) => c"__zluda_ptx_impl_agx_cp_reduce_bulk_f64_shared",
            (type_, ast::StateSpace::Global) if type_.size_of() == 8 => {
                c"__zluda_ptx_impl_agx_cp_reduce_bulk_b64_global"
            }
            (type_, _) if type_.size_of() == 8 => {
                c"__zluda_ptx_impl_agx_cp_reduce_bulk_b64_shared"
            }
            (_, ast::StateSpace::Global) => c"__zluda_ptx_impl_agx_cp_reduce_bulk_b32_global",
            (_, _) => c"__zluda_ptx_impl_agx_cp_reduce_bulk_b32_shared",
        };
        let op = self.const_u32(op);
        self.emit_agx_helper(name, None, None, &[to, from, size, op])?;
        if let Some(mbarrier) = arguments.src_mbarrier {
            let mbarrier = self.mbarrier_pointer(data.dst_space, mbarrier)?;
            self.emit_agx_helper(
                c"__zluda_ptx_impl_agx_mbarrier_complete_tx",
                None,
                None,
                &[mbarrier, size],
            )?;
        }
        Ok(())
    }

    fn emit_mbarrier_init(
        &mut self,
        data: ast::MbarrierInitDetails,
        arguments: ast::MbarrierInitArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let mbarrier = self.mbarrier_pointer(data.space, arguments.src_mbarrier)?;
        let count = self.resolver.value(arguments.src_count)?;
        let v1 = self.const_u32(data.v1 as u64);
        self.emit_agx_helper(
            c"__zluda_ptx_impl_agx_mbarrier_init",
            None,
            None,
            &[mbarrier, count, v1],
        )?;
        Ok(())
    }

    fn emit_mbarrier_inval(
        &mut self,
        space: ast::StateSpace,
        arguments: ast::MbarrierInvalArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let mbarrier = self.mbarrier_pointer(space, arguments.src_mbarrier)?;
        self.emit_agx_helper(
            c"__zluda_ptx_impl_agx_mbarrier_inval",
            None,
            None,
            &[mbarrier],
        )?;
        Ok(())
    }

    fn emit_mbarrier_arrive(
        &mut self,
        data: ast::MbarrierArriveDetails,
        arguments: ast::MbarrierArriveArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let mbarrier = self.mbarrier_pointer(data.space, arguments.src_mbarrier)?;
        // .expect_tx arrives once
        let count = match arguments.src_count {
            Some(count) => self.resolver.value(count)?,
            None => self.const_u32(1),
        };
        let tx_count = match arguments.src_tx_count {
            Some(tx_count) => self.resolver.value(tx_count)?,
            None => self.const_u32(0),
        };
        if let Some(mask) = arguments.src_cta_mask {
            if arguments.dst_state.is_some() {
                return Err(error_todo_msg("multicast mbarrier.arrive with a state"));
            }
            let mask = self.resolver.value(mask)?;
            // 2 arrive, 3 arrive_drop
            let op = self.const_u32(2 + data.drop as u64);
            self.emit_agx_helper(
                c"__zluda_ptx_impl_agx_mbarrier_multicast",
                None,
                None,
                &[mbarrier, op, count, tx_count, mask],
            )?;
            return Ok(());
        }
        let name = if data.drop {
            c"__zluda_ptx_impl_agx_mbarrier_arrive_drop"
        } else {
            c"__zluda_ptx_impl_agx_mbarrier_arrive"
        };
        self.emit_agx_helper(
            name,
            arguments.dst_state,
            Some(ast::ScalarType::B64),
            &[mbarrier, count, tx_count],
        )?;
        Ok(())
    }

    fn emit_mbarrier_tx(
        &mut self,
        data: ast::MbarrierTxDetails,
        arguments: ast::MbarrierTxArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let mbarrier = self.mbarrier_pointer(data.space, arguments.src_mbarrier)?;
        let tx_count = self.resolver.value(arguments.src_tx_count)?;
        if let Some(mask) = arguments.src_cta_mask {
            let mask = self.resolver.value(mask)?;
            let (op, zero) = (self.const_u32(data.complete as u64), self.const_u32(0));
            self.emit_agx_helper(
                c"__zluda_ptx_impl_agx_mbarrier_multicast",
                None,
                None,
                &[mbarrier, op, zero, tx_count, mask],
            )?;
            return Ok(());
        }
        let name = if data.complete {
            c"__zluda_ptx_impl_agx_mbarrier_complete_tx"
        } else {
            c"__zluda_ptx_impl_agx_mbarrier_expect_tx"
        };
        self.emit_agx_helper(name, None, None, &[mbarrier, tx_count])?;
        Ok(())
    }

    fn emit_mbarrier_wait(
        &mut self,
        data: ast::MbarrierWaitDetails,
        arguments: ast::MbarrierWaitArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let _ = arguments.src_time_hint;
        let mbarrier = self.mbarrier_pointer(data.space, arguments.src_mbarrier)?;
        let state = self.resolver.value(arguments.src_state)?;
        // The state of an arrive holds the parity of its phase in bit 0
        let parity = if data.parity {
            state
        } else {
            unsafe {
                LLVMBuildTrunc(
                    self.builder,
                    state,
                    LLVMInt32TypeInContext(self.context),
                    LLVM_UNNAMED.as_ptr(),
                )
            }
        };
        let name = match (data.try_wait, data.conditional) {
            (true, false) => c"__zluda_ptx_impl_agx_mbarrier_try_wait",
            (false, false) => c"__zluda_ptx_impl_agx_mbarrier_test_wait",
            (true, true) => c"__zluda_ptx_impl_agx_mbarrier_try_wait_conditional",
            (false, true) => c"__zluda_ptx_impl_agx_mbarrier_test_wait_conditional",
        };
        let done = self.emit_agx_helper(
            name,
            Some(arguments.dst),
            Some(ast::ScalarType::Pred),
            &[mbarrier, parity],
        )?;
        // the payload report of the completed phase (always zero for .layout::v0)
        if let Some(report) = arguments.dst_report_pred {
            self.emit_agx_helper(
                c"__zluda_ptx_impl_agx_mbarrier_report",
                Some(report),
                Some(ast::ScalarType::Pred),
                &[mbarrier, done],
            )?;
        }
        // the report value is only OR'ed with 0 (PTX ISA 9.7.10.28.1.3)
        if let Some(value) = arguments.dst_report_value {
            let zero = self.const_u32(0);
            self.resolver.register(value, zero);
        }
        Ok(())
    }

    fn emit_mbarrier_pending_count(
        &mut self,
        arguments: ast::MbarrierPendingCountArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        // bits 32-51 of the state of an arrive
        let state = self.resolver.value(arguments.src_state)?;
        let i64_type = unsafe { LLVMInt64TypeInContext(self.context) };
        let shifted = unsafe {
            LLVMBuildLShr(
                self.builder,
                state,
                LLVMConstInt(i64_type, 32, 0),
                LLVM_UNNAMED.as_ptr(),
            )
        };
        let count = unsafe {
            LLVMBuildTrunc(
                self.builder,
                shifted,
                LLVMInt32TypeInContext(self.context),
                LLVM_UNNAMED.as_ptr(),
            )
        };
        let mask = self.const_u32(0xfffff);
        self.resolver.with_result(arguments.dst, |dst| unsafe {
            LLVMBuildAnd(self.builder, count, mask, dst)
        });
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
            vec![&type_.into()],
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
            vec![&ast::ScalarType::S32.into()],
            vec![(mul_narrow, llvm_type_s32), (src3, llvm_type_s32)],
        )?;
        Ok(())
    }

    fn emit_set(
        &mut self,
        data: ptx_parser::SetData,
        arguments: ptx_parser::SetArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let src1 = self.resolver.value(arguments.src1)?;
        let src2 = self.resolver.value(arguments.src2)?;
        let result = if data.base.type_.packed_type().is_some() {
            self.emit_set_packed(data.base, data.dtype, src1, src2, None)?
        } else {
            let setp_result = self.emit_setp_impl(data.base, src1, src2)?;
            self.setp_to_set(data.dtype, setp_result)?
        };
        self.resolver.register(arguments.dst, result);
        Ok(())
    }

    // set on f16x2 and bf16x2: the result of each lane in the lane of dtype (the halves of .u32 and .s32), with the
    // lane's comparison combined with the predicate of the BoolOp form
    fn emit_set_packed(
        &mut self,
        base: ptx_parser::SetpData,
        dtype: ast::ScalarType,
        src1: LLVMValueRef,
        src2: LLVMValueRef,
        bool_op: Option<(ptx_parser::SetpBoolPostOp, LLVMValueRef)>,
    ) -> Result<LLVMValueRef, TranslateError> {
        let lane_type = match dtype {
            ast::ScalarType::U32 | ast::ScalarType::S32 => ast::ScalarType::U16,
            ast::ScalarType::F16x2 => ast::ScalarType::F16,
            ast::ScalarType::BF16x2 => ast::ScalarType::BF16,
            _ => return Err(error_unreachable()),
        };
        let data = ptx_parser::SetpData {
            type_: base.type_.packed_type().ok_or_else(error_unreachable)?,
            ..base
        };
        let u8_llvm = get_scalar_type(self.context, ast::ScalarType::B8);
        let mut vector = unsafe { LLVMGetPoison(get_type(self.context, &ast::Type::Vector(2, lane_type))?) };
        for lane in 0..2 {
            let index = unsafe { LLVMConstInt(u8_llvm, lane, 0) };
            let (a, b) = unsafe {
                (
                    LLVMBuildExtractElement(self.builder, src1, index, LLVM_UNNAMED.as_ptr()),
                    LLVMBuildExtractElement(self.builder, src2, index, LLVM_UNNAMED.as_ptr()),
                )
            };
            let mut lane_result = self.emit_setp_impl(data, a, b)?;
            if let Some((op, c)) = bool_op {
                let op = match op {
                    ptx_parser::SetpBoolPostOp::Xor => LLVMBuildXor,
                    ptx_parser::SetpBoolPostOp::And => LLVMBuildAnd,
                    ptx_parser::SetpBoolPostOp::Or => LLVMBuildOr,
                };
                lane_result = unsafe { op(self.builder, lane_result, c, LLVM_UNNAMED.as_ptr()) };
            }
            let lane_result = self.setp_to_set(lane_type, lane_result)?;
            vector = unsafe { LLVMBuildInsertElement(self.builder, vector, lane_result, index, LLVM_UNNAMED.as_ptr()) };
        }
        Ok(unsafe {
            LLVMBuildBitCast(self.builder, vector, get_scalar_type(self.context, dtype), LLVM_UNNAMED.as_ptr())
        })
    }

    fn emit_set_bool(
        &mut self,
        data: ptx_parser::SetBoolData,
        arguments: ptx_parser::SetBoolArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let src3 = self.resolver.value(arguments.src3)?;
        let set_result = if data.base.base.type_.packed_type().is_some() {
            let src3 = if data.base.negate_src3 {
                unsafe { LLVMBuildNot(self.builder, src3, LLVM_UNNAMED.as_ptr()) }
            } else {
                src3
            };
            let src1 = self.resolver.value(arguments.src1)?;
            let src2 = self.resolver.value(arguments.src2)?;
            self.emit_set_packed(data.base.base, data.dtype, src1, src2, Some((data.base.bool_op, src3)))?
        } else {
            let (result_bool, _) =
                self.emit_setp_bool_impl(data.base, arguments.src1, arguments.src2, src3, false)?;
            self.setp_to_set(data.dtype, result_bool)?
        };
        self.resolver.register(arguments.dst, set_result);
        Ok(())
    }

    fn emit_setp_bool(
        &mut self,
        data: ast::SetpBoolData,
        args: ast::SetpBoolArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let src3 = self.resolver.value(args.src3)?;
        if let Some(scalar_type) = data.base.type_.packed_type() {
            // as setp.f16x2 p|q, each lane combined with c
            let src3 = if data.negate_src3 {
                unsafe { LLVMBuildNot(self.builder, src3, LLVM_UNNAMED.as_ptr()) }
            } else {
                src3
            };
            let op = match data.bool_op {
                ptx_parser::SetpBoolPostOp::Xor => LLVMBuildXor,
                ptx_parser::SetpBoolPostOp::And => LLVMBuildAnd,
                ptx_parser::SetpBoolPostOp::Or => LLVMBuildOr,
            };
            let src1 = self.resolver.value(args.src1)?;
            let src2 = self.resolver.value(args.src2)?;
            let base = ptx_parser::SetpData { type_: scalar_type, ..data.base };
            let u8_llvm = get_scalar_type(self.context, ast::ScalarType::B8);
            let mut lanes = [ptr::null_mut(); 2];
            for (i, lane) in lanes.iter_mut().enumerate() {
                let index = unsafe { LLVMConstInt(u8_llvm, i as u64, 0) };
                let (a, b) = unsafe {
                    (
                        LLVMBuildExtractElement(self.builder, src1, index, LLVM_UNNAMED.as_ptr()),
                        LLVMBuildExtractElement(self.builder, src2, index, LLVM_UNNAMED.as_ptr()),
                    )
                };
                let cmp = self.emit_setp_impl(base, a, b)?;
                *lane = unsafe { op(self.builder, cmp, src3, LLVM_UNNAMED.as_ptr()) };
            }
            self.resolver.register(args.dst1, lanes[0]);
            return match args.dst2 {
                Some(dst2) => {
                    self.resolver.register(dst2, lanes[1]);
                    Ok(())
                }
                None => Err(error_unreachable()),
            };
        }
        let (dst, dst2_value) =
            self.emit_setp_bool_impl(data, args.src1, args.src2, src3, args.dst2.is_some())?;
        self.resolver.register(args.dst1, dst);
        if let (Some(dst2), Some(dst2_value)) = (args.dst2, dst2_value) {
            self.resolver.register(dst2, dst2_value);
        }
        Ok(())
    }

    fn emit_setp_bool_impl(
        &mut self,
        data: ptx_parser::SetpBoolData,
        src1: SpirvWord,
        src2: SpirvWord,
        src3: LLVMValueRef,
        with_dst2: bool,
    ) -> Result<(LLVMValueRef, Option<LLVMValueRef>), TranslateError> {
        let src1 = self.resolver.value(src1)?;
        let src2 = self.resolver.value(src2)?;
        let bool_result = self.emit_setp_impl(data.base, src1, src2)?;
        let post_op = match data.bool_op {
            ptx_parser::SetpBoolPostOp::Xor => LLVMBuildXor,
            ptx_parser::SetpBoolPostOp::And => LLVMBuildAnd,
            ptx_parser::SetpBoolPostOp::Or => LLVMBuildOr,
        };
        let src3 = if data.negate_src3 {
            let constant =
                unsafe { LLVMConstInt(LLVMIntTypeInContext(self.context, 1), u64::MAX, 1) };
            unsafe { LLVMBuildXor(self.builder, src3, constant, LLVM_UNNAMED.as_ptr()) }
        } else {
            src3
        };
        let result = unsafe { post_op(self.builder, bool_result, src3, LLVM_UNNAMED.as_ptr()) };
        let dst2_value = with_dst2.then(|| {
            let not = unsafe { LLVMBuildNot(self.builder, bool_result, LLVM_UNNAMED.as_ptr()) };
            unsafe { post_op(self.builder, not, src3, LLVM_UNNAMED.as_ptr()) }
        });
        Ok((result, dst2_value))
    }

    #[must_use]
    fn setp_to_set(
        &mut self,
        dtype: ast::ScalarType,
        setp_result: LLVMValueRef,
    ) -> Result<LLVMValueRef, TranslateError> {
        let llvm_dtype = get_scalar_type(self.context, dtype);
        let zero = unsafe { LLVMConstNull(llvm_dtype) };
        let one = if dtype.kind() == ast::ScalarKind::Float {
            unsafe { LLVMConstReal(llvm_dtype, 1.0) }
        } else {
            unsafe { LLVMConstInt(llvm_dtype, u64::MAX, 1) }
        };
        Ok(unsafe { LLVMBuildSelect(self.builder, setp_result, one, zero, LLVM_UNNAMED.as_ptr()) })
    }

    fn emit_dp4a(
        &mut self,
        data: ast::Dp4aDetails,
        arguments: ast::Dp4aArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        if self.target == Target::Agx {
            return self.emit_dp4a_expanded(data, arguments);
        }
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
            vec![&data.ctype().into()],
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

    /// dp4a as four byte multiplies: c + sum(a[i] * b[i]), each byte extended
    /// according to the signedness of its operand.
    fn emit_dp4a_expanded(
        &mut self,
        data: ast::Dp4aDetails,
        arguments: ast::Dp4aArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let signed = |t: ast::ScalarType| match t {
            ast::ScalarType::U32 => Ok(false),
            ast::ScalarType::S32 => Ok(true),
            _ => Err(error_unreachable()),
        };
        let a_signed = signed(data.atype)?;
        let b_signed = signed(data.btype)?;
        let i32_type = unsafe { LLVMInt32TypeInContext(self.context) };
        let builder = self.builder;
        let byte = |value: LLVMValueRef, index: u64, is_signed: bool| unsafe {
            // Move byte `index` to the top, then shift it back down.
            let up = LLVMConstInt(i32_type, 24 - 8 * index, 0);
            let down = LLVMConstInt(i32_type, 24, 0);
            let top = LLVMBuildShl(builder, value, up, LLVM_UNNAMED.as_ptr());
            if is_signed {
                LLVMBuildAShr(builder, top, down, LLVM_UNNAMED.as_ptr())
            } else {
                LLVMBuildLShr(builder, top, down, LLVM_UNNAMED.as_ptr())
            }
        };
        let a = self.resolver.value(arguments.src1)?;
        let b = self.resolver.value(arguments.src2)?;
        let mut sum = self.resolver.value(arguments.src3)?;
        for i in 0..4 {
            let product = unsafe {
                LLVMBuildMul(
                    builder,
                    byte(a, i, a_signed),
                    byte(b, i, b_signed),
                    LLVM_UNNAMED.as_ptr(),
                )
            };
            sum = unsafe { LLVMBuildAdd(builder, sum, product, LLVM_UNNAMED.as_ptr()) };
        }
        self.resolver.register(arguments.dst, sum);
        Ok(())
    }

    fn emit_trap(&mut self) -> Result<(), TranslateError> {
        self.emit_intrinsic(c"llvm.trap", None, vec![], vec![])?;
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
            vec![&type_.into()],
            vec![
                (src2, get_scalar_type(self.context, type_)),
                (src1, get_scalar_type(self.context, type_)),
            ],
        )?;
        Ok(())
    }

    fn emit_createpolicy_fractional(
        &mut self,
        _data: ast::CreatePolicyFractionalDetails,
        arguments: ast::CreatePolicyFractionalArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        // Implement this as a nop for now.
        self.resolver.register(arguments.dst_policy, unsafe {
            LLVMConstInt(get_scalar_type(self.context, ast::ScalarType::B64), 0, 0)
        });
        Ok(())
    }

    fn emit_sad(
        &mut self,
        data: ast::ScalarType,
        arguments: ast::SadArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let cmp_op = if data.kind() == ast::ScalarKind::Signed {
            LLVMIntPredicate::LLVMIntSLT
        } else {
            LLVMIntPredicate::LLVMIntULT
        };
        let src1 = self.resolver.value(arguments.src1)?;
        let src2 = self.resolver.value(arguments.src2)?;
        let src3 = self.resolver.value(arguments.src3)?;
        let less_than =
            unsafe { LLVMBuildICmp(self.builder, cmp_op, src1, src2, LLVM_UNNAMED.as_ptr()) };
        let sub1 = unsafe { LLVMBuildSub(self.builder, src1, src2, LLVM_UNNAMED.as_ptr()) };
        let sub2 = unsafe { LLVMBuildSub(self.builder, src2, src1, LLVM_UNNAMED.as_ptr()) };
        let subtraction =
            unsafe { LLVMBuildSelect(self.builder, less_than, sub2, sub1, LLVM_UNNAMED.as_ptr()) };
        self.resolver.with_result(arguments.dst, |dst| unsafe {
            LLVMBuildAdd(self.builder, subtraction, src3, dst)
        });
        Ok(())
    }

    fn emit_vshr(
        &mut self,
        data: ast::VshData,
        arguments: ast::VshrArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let src1 = self.resolver.value(arguments.src1)?;
        let shift_amount = self.resolver.value(arguments.src2)?;
        let shifted = match data.mode {
            ast::FunnelShiftMode::Clamp => {
                let pre_shifted = unsafe {
                    LLVMBuildLShr(self.builder, src1, shift_amount, LLVM_UNNAMED.as_ptr())
                };
                let llvm_i32 = get_scalar_type(self.context, ast::ScalarType::B32);
                let const_32 = unsafe { LLVMConstInt(llvm_i32, 32, 0) };
                let const_0 = unsafe { LLVMConstInt(llvm_i32, 0, 0) };
                let should_clamp = unsafe {
                    LLVMBuildICmp(
                        self.builder,
                        LLVMIntPredicate::LLVMIntUGE,
                        shift_amount,
                        const_32,
                        LLVM_UNNAMED.as_ptr(),
                    )
                };
                unsafe {
                    LLVMBuildSelect(
                        self.builder,
                        should_clamp,
                        const_0,
                        pre_shifted,
                        LLVM_UNNAMED.as_ptr(),
                    )
                }
            }
            ast::FunnelShiftMode::Wrap => {
                return Err(error_todo());
            }
        };
        match data.op2 {
            ast::VshOp::Add => {
                let src3 = self.resolver.value(arguments.src3)?;
                self.resolver.with_result(arguments.dst, |dst| unsafe {
                    LLVMBuildAdd(self.builder, shifted, src3, dst)
                });
            }
        }
        Ok(())
    }

    fn emit_bfind(
        &mut self,
        data: ast::BfindDetails,
        arguments: ast::BfindArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let src = self.resolver.value(arguments.src)?;
        let llvm_type = get_scalar_type(self.context, data.type_);
        let width = data.type_.size_of() as u64 * 8;
        // For signed types look for the most significant bit that differs
        // from the sign bit
        let value = if data.type_.kind() == ast::ScalarKind::Signed {
            let sign = unsafe {
                LLVMBuildAShr(
                    self.builder,
                    src,
                    LLVMConstInt(llvm_type, width - 1, 0),
                    LLVM_UNNAMED.as_ptr(),
                )
            };
            unsafe { LLVMBuildXor(self.builder, src, sign, LLVM_UNNAMED.as_ptr()) }
        } else {
            src
        };
        let llvm_fn = if width == 64 {
            c"llvm.ctlz.i64"
        } else {
            c"llvm.ctlz.i32"
        };
        let pred = get_scalar_type(self.context, ast::ScalarType::Pred);
        let leading_zeros = self.emit_intrinsic(
            llvm_fn,
            None,
            vec![&data.type_.into()],
            vec![(value, llvm_type), (unsafe { LLVMConstInt(pred, 0, 0) }, pred)],
        )?;
        let llvm_i32 = get_scalar_type(self.context, ast::ScalarType::B32);
        let leading_zeros = unsafe {
            LLVMBuildTruncOrBitCast(self.builder, leading_zeros, llvm_i32, LLVM_UNNAMED.as_ptr())
        };
        let result = if data.shift_amount {
            leading_zeros
        } else {
            unsafe {
                LLVMBuildSub(
                    self.builder,
                    LLVMConstInt(llvm_i32, width - 1, 0),
                    leading_zeros,
                    LLVM_UNNAMED.as_ptr(),
                )
            }
        };
        let not_found = unsafe {
            LLVMBuildICmp(
                self.builder,
                LLVMIntPredicate::LLVMIntEQ,
                value,
                LLVMConstInt(llvm_type, 0, 0),
                LLVM_UNNAMED.as_ptr(),
            )
        };
        let const_max = unsafe { LLVMConstInt(llvm_i32, u64::MAX, 1) };
        self.resolver.with_result(arguments.dst, |dst_name| unsafe {
            LLVMBuildSelect(self.builder, not_found, const_max, result, dst_name)
        });
        Ok(())
    }

    // bmsk.wrap.b32 d, a, b: b one bits starting at bit a, both taken modulo 32
    // lop3: bit i of the lookup table is the result for the inputs (a, b, c) = (i >> 2, i >> 1, i) & 1, the
    // minterms of PTX's ta = 0xF0, tb = 0xCC, tc = 0xAA. The table may be a register, so every minterm is masked by
    // its bit; a constant table folds away.
    fn emit_lop3(
        &mut self,
        data: Option<ast::SetpBoolPostOp>,
        arguments: ast::Lop3Args<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let i32_type = get_scalar_type(self.context, ast::ScalarType::B32);
        let b = self.builder;
        let (x, y, z) = (
            self.resolver.value(arguments.src1)?,
            self.resolver.value(arguments.src2)?,
            self.resolver.value(arguments.src3)?,
        );
        let lut = self.resolver.value(arguments.src_lut)?;
        let result = unsafe {
            let c = |v: u64| LLVMConstInt(i32_type, v, 0);
            let not = |v: LLVMValueRef| LLVMBuildNot(b, v, LLVM_UNNAMED.as_ptr());
            let mut acc = c(0);
            for i in 0..8u64 {
                let a = if i & 4 != 0 { x } else { not(x) };
                let bb = if i & 2 != 0 { y } else { not(y) };
                let cc = if i & 1 != 0 { z } else { not(z) };
                let term = LLVMBuildAnd(b, LLVMBuildAnd(b, a, bb, LLVM_UNNAMED.as_ptr()), cc, LLVM_UNNAMED.as_ptr());
                // all ones when bit i of the table is set
                let bit = LLVMBuildAnd(b, LLVMBuildLShr(b, lut, c(i), LLVM_UNNAMED.as_ptr()), c(1), LLVM_UNNAMED.as_ptr());
                let mask = LLVMBuildSub(b, c(0), bit, LLVM_UNNAMED.as_ptr());
                acc = LLVMBuildOr(b, acc, LLVMBuildAnd(b, term, mask, LLVM_UNNAMED.as_ptr()), LLVM_UNNAMED.as_ptr());
            }
            acc
        };
        self.resolver.register(arguments.dst, result);
        if let (Some(op), Some(p), Some(q)) = (data, arguments.dst_pred, arguments.src_pred) {
            let q = self.resolver.value(q)?;
            unsafe {
                let nonzero = LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntNE, result, LLVMConstInt(i32_type, 0, 0), LLVM_UNNAMED.as_ptr());
                self.resolver.with_result(p, |name| match op {
                    ast::SetpBoolPostOp::And => LLVMBuildAnd(b, nonzero, q, name),
                    ast::SetpBoolPostOp::Or => LLVMBuildOr(b, nonzero, q, name),
                    ast::SetpBoolPostOp::Xor => LLVMBuildXor(b, nonzero, q, name),
                });
            }
        }
        Ok(())
    }

    // The member mask is ignored, as in the other warp operations: all active threads take part
    fn emit_active_lanes(&mut self) -> Result<LLVMValueRef, TranslateError> {
        let i1 = unsafe { LLVMInt1TypeInContext(self.context) };
        let true_ = unsafe { LLVMConstInt(i1, 1, 0) };
        self.emit_intrinsic(c"llvm.agx.simd.ballot", None, vec![&ast::Type::Scalar(ast::ScalarType::U32)], vec![(true_, i1)])
    }

    fn emit_lane_id(&mut self) -> Result<LLVMValueRef, TranslateError> {
        self.emit_intrinsic(c"llvm.agx.thread.index.in.simdgroup", None, vec![&ast::Type::Scalar(ast::ScalarType::U32)], vec![])
    }

    // elect.sync: the leader is the lowest active lane
    fn emit_elect_sync(&mut self, arguments: ast::ElectSyncArgs<SpirvWord>) -> Result<(), TranslateError> {
        let i32_type = get_scalar_type(self.context, ast::ScalarType::B32);
        let active = self.emit_active_lanes()?;
        let i1 = unsafe { LLVMInt1TypeInContext(self.context) };
        let false_ = unsafe { LLVMConstInt(i1, 0, 0) };
        let leader = self.emit_int_intrinsic("llvm.cttz", i32_type, &[active, false_]);
        self.resolver.register(arguments.dst, leader);
        if let Some(p) = arguments.dst_pred {
            let lane = self.emit_lane_id()?;
            self.resolver.with_result(p, |name| unsafe {
                LLVMBuildICmp(self.builder, LLVMIntPredicate::LLVMIntEQ, lane, leader, name)
            });
        }
        Ok(())
    }

    // match.all.sync: compare with the value of the lowest active lane
    fn emit_match_all_sync(
        &mut self,
        type_: ast::ScalarType,
        arguments: ast::MatchAllSyncArgs<SpirvWord>,
    ) -> Result<(), TranslateError> {
        let i32_type = get_scalar_type(self.context, ast::ScalarType::B32);
        let i1 = unsafe { LLVMInt1TypeInContext(self.context) };
        let b = self.builder;
        let value = self.resolver.value(arguments.src)?;
        let active = self.emit_active_lanes()?;
        let false_ = unsafe { LLVMConstInt(i1, 0, 0) };
        let leader = self.emit_int_intrinsic("llvm.cttz", i32_type, &[active, false_]);
        let u32_ = ast::Type::Scalar(ast::ScalarType::U32);
        let shuffle = |this: &mut Self, v: LLVMValueRef| {
            this.emit_intrinsic(c"llvm.agx.simd.shuffle.i32", None, vec![&u32_], vec![(v, i32_type), (leader, i32_type)])
        };
        let same = unsafe {
            if type_.size_of() == 8 {
                let i64_type = get_scalar_type(self.context, ast::ScalarType::B64);
                let v = LLVMBuildBitCast(b, value, i64_type, LLVM_UNNAMED.as_ptr());
                let lo = LLVMBuildTrunc(b, v, i32_type, LLVM_UNNAMED.as_ptr());
                let hi = LLVMBuildTrunc(b, LLVMBuildLShr(b, v, LLVMConstInt(i64_type, 32, 0), LLVM_UNNAMED.as_ptr()), i32_type, LLVM_UNNAMED.as_ptr());
                let (slo, shi) = (shuffle(self, lo)?, shuffle(self, hi)?);
                let elo = LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntEQ, lo, slo, LLVM_UNNAMED.as_ptr());
                let ehi = LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntEQ, hi, shi, LLVM_UNNAMED.as_ptr());
                LLVMBuildAnd(b, elo, ehi, LLVM_UNNAMED.as_ptr())
            } else {
                let v = LLVMBuildBitCast(b, value, i32_type, LLVM_UNNAMED.as_ptr());
                let sv = shuffle(self, v)?;
                LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntEQ, v, sv, LLVM_UNNAMED.as_ptr())
            }
        };
        let different = unsafe { LLVMBuildNot(b, same, LLVM_UNNAMED.as_ptr()) };
        let differ_mask = self.emit_intrinsic(c"llvm.agx.simd.ballot", None, vec![&u32_], vec![(different, i1)])?;
        let all = unsafe { LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntEQ, differ_mask, LLVMConstInt(i32_type, 0, 0), LLVM_UNNAMED.as_ptr()) };
        let mask = unsafe { LLVMBuildSelect(b, all, active, LLVMConstInt(i32_type, 0, 0), LLVM_UNNAMED.as_ptr()) };
        self.resolver.register(arguments.dst, mask);
        if let Some(p) = arguments.dst_pred {
            self.resolver.register(p, all);
        }
        Ok(())
    }

    fn emit_mad24(&mut self, data: ast::Mad24Details, arguments: ast::Mad24Args<SpirvWord>) -> Result<(), TranslateError> {
        let i32_type = get_scalar_type(self.context, ast::ScalarType::B32);
        let i64_type = get_scalar_type(self.context, ast::ScalarType::B64);
        let b = self.builder;
        let signed = data.type_ == ast::ScalarType::S32;
        let (x, y, z) = (
            self.resolver.value(arguments.src1)?,
            self.resolver.value(arguments.src2)?,
            self.resolver.value(arguments.src3)?,
        );
        let product = unsafe {
            let c32 = |v: u64| LLVMConstInt(i32_type, v, 0);
            // the low 24 bits, sign- or zero-extended
            let ext = |v: LLVMValueRef| {
                let up = LLVMBuildShl(b, v, c32(8), LLVM_UNNAMED.as_ptr());
                if signed {
                    LLVMBuildSExt(b, LLVMBuildAShr(b, up, c32(8), LLVM_UNNAMED.as_ptr()), i64_type, LLVM_UNNAMED.as_ptr())
                } else {
                    LLVMBuildZExt(b, LLVMBuildLShr(b, up, c32(8), LLVM_UNNAMED.as_ptr()), i64_type, LLVM_UNNAMED.as_ptr())
                }
            };
            let p = LLVMBuildMul(b, ext(x), ext(y), LLVM_UNNAMED.as_ptr());
            let p = if data.control == Mul24Control::Hi {
                LLVMBuildLShr(b, p, LLVMConstInt(i64_type, 16, 0), LLVM_UNNAMED.as_ptr())
            } else {
                p
            };
            LLVMBuildTrunc(b, p, i32_type, LLVM_UNNAMED.as_ptr())
        };
        let result = if data.saturate {
            self.emit_int_intrinsic("llvm.sadd.sat", i32_type, &[product, z])
        } else {
            unsafe { LLVMBuildAdd(b, product, z, LLVM_UNNAMED.as_ptr()) }
        };
        self.resolver.register(arguments.dst, result);
        Ok(())
    }

    fn emit_slct(&mut self, data: ast::SlctDetails, arguments: ast::SlctArgs<SpirvWord>) -> Result<(), TranslateError> {
        let i32_type = get_scalar_type(self.context, ast::ScalarType::B32);
        let b = self.builder;
        let (x, y) = (self.resolver.value(arguments.src1)?, self.resolver.value(arguments.src2)?);
        let c = self.resolver.value(arguments.src3)?;
        let take_a = unsafe {
            let k = |v: u64| LLVMConstInt(i32_type, v, 0);
            if data.ctype == ast::ScalarType::S32 {
                LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntSGE, c, k(0), LLVM_UNNAMED.as_ptr())
            } else {
                // on the bits, so that the floating point modes do not matter: positive (not NaN) or -0, and with
                // .ftz any subnormal
                let bits = LLVMBuildBitCast(b, c, i32_type, LLVM_UNNAMED.as_ptr());
                let magnitude = LLVMBuildAnd(b, bits, k(0x7fff_ffff), LLVM_UNNAMED.as_ptr());
                let positive = LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntSGE, bits, k(0), LLVM_UNNAMED.as_ptr());
                let not_nan = LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntULE, magnitude, k(0x7f80_0000), LLVM_UNNAMED.as_ptr());
                let small = if data.flush_to_zero { 0x80_0000 } else { 1 };
                let zero = LLVMBuildICmp(b, LLVMIntPredicate::LLVMIntULT, magnitude, k(small), LLVM_UNNAMED.as_ptr());
                LLVMBuildOr(b, LLVMBuildAnd(b, positive, not_nan, LLVM_UNNAMED.as_ptr()), zero, LLVM_UNNAMED.as_ptr())
            }
        };
        self.resolver.with_result(arguments.dst, |name| unsafe { LLVMBuildSelect(b, take_a, x, y, name) });
        Ok(())
    }

    fn emit_testp(&mut self, data: ast::TestpDetails, arguments: ast::TestpArgs<SpirvWord>) -> Result<(), TranslateError> {
        let (bits_type, exp_mask, mant_mask, mag_mask) = if data.type_ == ast::ScalarType::F64 {
            (get_scalar_type(self.context, ast::ScalarType::B64), 0x7ff0_0000_0000_0000u64, 0x000f_ffff_ffff_ffffu64, 0x7fff_ffff_ffff_ffffu64)
        } else {
            (get_scalar_type(self.context, ast::ScalarType::B32), 0x7f80_0000, 0x007f_ffff, 0x7fff_ffff)
        };
        let b = self.builder;
        let src = self.resolver.value(arguments.src)?;
        unsafe {
            let k = |v: u64| LLVMConstInt(bits_type, v, 0);
            let bits = LLVMBuildBitCast(b, src, bits_type, LLVM_UNNAMED.as_ptr());
            let magnitude = LLVMBuildAnd(b, bits, k(mag_mask), LLVM_UNNAMED.as_ptr());
            let exponent = LLVMBuildAnd(b, bits, k(exp_mask), LLVM_UNNAMED.as_ptr());
            let cmp = |p, l, r| LLVMBuildICmp(b, p, l, r, LLVM_UNNAMED.as_ptr());
            use LLVMIntPredicate::*;
            let result = match data.op {
                ast::TestpOp::Finite => cmp(LLVMIntNE, exponent, k(exp_mask)),
                ast::TestpOp::Infinite => cmp(LLVMIntEQ, magnitude, k(exp_mask)),
                ast::TestpOp::Number => cmp(LLVMIntULE, magnitude, k(exp_mask)),
                ast::TestpOp::NotANumber => cmp(LLVMIntUGT, magnitude, k(exp_mask)),
                // "positive and negative zero are considered normal numbers" (PTX ISA 9.7.3.1)
                ast::TestpOp::Normal => {
                    let not_special = cmp(LLVMIntNE, exponent, k(exp_mask));
                    let not_subnormal = LLVMBuildOr(
                        b,
                        cmp(LLVMIntNE, exponent, k(0)),
                        cmp(LLVMIntEQ, magnitude, k(0)),
                        LLVM_UNNAMED.as_ptr(),
                    );
                    LLVMBuildAnd(b, not_special, not_subnormal, LLVM_UNNAMED.as_ptr())
                }
                ast::TestpOp::Subnormal => {
                    let small = cmp(LLVMIntEQ, exponent, k(0));
                    let nonzero = cmp(LLVMIntNE, magnitude, k(0));
                    let _ = mant_mask;
                    LLVMBuildAnd(b, small, nonzero, LLVM_UNNAMED.as_ptr())
                }
            };
            self.resolver.register(arguments.dst, result);
        }
        Ok(())
    }

    fn emit_bmsk_wrap(&mut self, arguments: ast::BmskArgs<SpirvWord>) -> Result<(), TranslateError> {
        let llvm_i32 = get_scalar_type(self.context, ast::ScalarType::B32);
        let position = self.resolver.value(arguments.src_a)?;
        let width = self.resolver.value(arguments.src_b)?;
        let const_31 = unsafe { LLVMConstInt(llvm_i32, 31, 0) };
        let const_1 = unsafe { LLVMConstInt(llvm_i32, 1, 0) };
        unsafe {
            let position = LLVMBuildAnd(self.builder, position, const_31, LLVM_UNNAMED.as_ptr());
            let width = LLVMBuildAnd(self.builder, width, const_31, LLVM_UNNAMED.as_ptr());
            let bits = LLVMBuildShl(self.builder, const_1, width, LLVM_UNNAMED.as_ptr());
            let bits = LLVMBuildSub(self.builder, bits, const_1, LLVM_UNNAMED.as_ptr());
            self.resolver.with_result(arguments.dst, |dst| {
                LLVMBuildShl(self.builder, bits, position, dst)
            });
        }
        Ok(())
    }

    fn emit_exit(&mut self) -> Result<(), TranslateError> {
        if !self.is_kernel {
            return Err(error_todo());
        }
        unsafe {
            LLVMBuildRetVoid(self.builder);
        }
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

// (exponent bits, mantissa bits, largest finite magnitude code, NaN code)
fn mini_format(format: ast::MiniFormat) -> (u64, u64, u64, u64) {
    match format {
        ast::MiniFormat::E4m3x2 => (4, 3, 0x7e, 0x7f),
        ast::MiniFormat::E5m2x2 => (5, 2, 0x7b, 0x7f),
        ast::MiniFormat::E2m3x2 => (2, 3, 31, 31),
        ast::MiniFormat::E3m2x2 => (3, 2, 31, 31),
        ast::MiniFormat::E2m1x2 => (2, 1, 7, 7),
        ast::MiniFormat::Ue8m0x2 => (8, 0, 254, 255),
    }
}

fn mini_element_bits(format: ast::MiniFormat) -> u32 {
    match format {
        ast::MiniFormat::E4m3x2 | ast::MiniFormat::E5m2x2 | ast::MiniFormat::Ue8m0x2 => 8,
        ast::MiniFormat::E2m3x2 | ast::MiniFormat::E3m2x2 => 6,
        ast::MiniFormat::E2m1x2 => 4,
    }
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
fn get_scope(scope: ast::MemScope) -> Result<*const std::ffi::c_char, TranslateError> {
    Ok(match scope {
        ast::MemScope::Cta => c"workgroup-one-as",
        ast::MemScope::Gpu => c"agent-one-as",
        ast::MemScope::Sys => c"one-as",
        ast::MemScope::Cluster => return Err(error_todo()),
    }
    .as_ptr())
}

fn get_scope_membar(scope: ast::MemScope) -> Result<*const std::ffi::c_char, TranslateError> {
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
        ast::Type::Texref => {
            // Definitions taken from ROCm LLVM output
            let format_desc =
                unsafe { LLVMStructCreateNamed(context, c"struct.hipChannelFormatDesc".as_ptr()) };
            let llvm_i32 = get_scalar_type(context, ast::ScalarType::B32);
            let mut format_desc_fields = [llvm_i32, llvm_i32, llvm_i32, llvm_i32, llvm_i32];
            unsafe {
                LLVMStructSetBody(
                    format_desc,
                    format_desc_fields.as_mut_ptr(),
                    format_desc_fields.len() as u32,
                    0,
                )
            };
            let texture_reference =
                unsafe { LLVMStructCreateNamed(context, c"struct.textureReference".as_ptr()) };
            let llvm_a3i32 = unsafe { LLVMArrayType2(llvm_i32, 3) };
            let llvm_float = get_scalar_type(context, ast::ScalarType::F32);
            let llvm_ptr = get_pointer_type(context, ast::StateSpace::Generic)?;
            let mut texture_reference_fields = [
                llvm_i32,
                llvm_i32,
                llvm_i32,
                llvm_a3i32,
                format_desc,
                llvm_i32,
                llvm_i32,
                llvm_i32,
                llvm_float,
                llvm_float,
                llvm_float,
                llvm_ptr,
                llvm_i32,
                llvm_i32,
            ];
            unsafe {
                LLVMStructSetBody(
                    texture_reference,
                    texture_reference_fields.as_mut_ptr(),
                    texture_reference_fields.len() as u32,
                    0,
                )
            };
            let texture = unsafe { LLVMStructCreateNamed(context, c"struct.texture".as_ptr()) };
            let mut texture_fields = [texture_reference];
            unsafe {
                LLVMStructSetBody(
                    texture,
                    texture_fields.as_mut_ptr(),
                    texture_fields.len() as u32,
                    0,
                )
            };
            texture
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
            match t {
                ast::Type::Scalar(scalar) => {
                    write!(name, "{}", LLVMTypeDisplay(*scalar)).ok();
                }
                ast::Type::Vector(len, scalar) => {
                    write!(name, "v{}{}", len, LLVMTypeDisplay(*scalar)).ok();
                }
                _ => return Err(error_unreachable()),
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

fn get_anonymous_struct_type<'a>(
    context: LLVMContextRef,
    elem_types: impl Iterator<Item = &'a ast::Type>,
) -> Result<LLVMTypeRef, TranslateError> {
    let mut elem_types = elem_types
        .map(|t| get_type(context, t))
        .collect::<Result<Vec<LLVMTypeRef>, _>>()?;
    Ok(unsafe {
        LLVMStructTypeInContext(context, elem_types.as_mut_ptr(), elem_types.len() as u32, 0)
    })
}

enum StructKind {
    Named,
    Anonymous,
}

fn get_function_type_impl<'a>(
    context: LLVMContextRef,
    mut return_args: impl DoubleEndedIterator<Item = &'a ast::Type>
        + ExactSizeIterator<Item = &'a ast::Type>,
    input_args: impl Iterator<Item = Result<LLVMTypeRef, TranslateError>>,
    struct_kind: StructKind,
) -> Result<LLVMTypeRef, TranslateError> {
    let mut input_args = input_args.collect::<Result<Vec<_>, _>>()?;
    let return_type = match return_args.len() {
        0 => unsafe { LLVMVoidTypeInContext(context) },
        1 => get_type(context, &return_args.next().unwrap())?,
        _ => match struct_kind {
            StructKind::Named => get_or_create_struct_type(context, return_args)?,
            StructKind::Anonymous => get_anonymous_struct_type(context, return_args)?,
        },
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

fn get_function_type<'a>(
    context: LLVMContextRef,
    return_args: impl DoubleEndedIterator<Item = &'a ast::Type>
        + ExactSizeIterator<Item = &'a ast::Type>,
    input_args: impl Iterator<Item = Result<LLVMTypeRef, TranslateError>>,
) -> Result<LLVMTypeRef, TranslateError> {
    get_function_type_impl(context, return_args, input_args, StructKind::Named)
}

fn get_intrinsic_type<'a>(
    context: LLVMContextRef,
    return_args: impl DoubleEndedIterator<Item = &'a ast::Type>
        + ExactSizeIterator<Item = &'a ast::Type>,
    input_args: impl Iterator<Item = Result<LLVMTypeRef, TranslateError>>,
) -> Result<LLVMTypeRef, TranslateError> {
    get_function_type_impl(context, return_args, input_args, StructKind::Anonymous)
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

    fn get_or_add_raw(&mut self, word: SpirvWord) -> *const std::ffi::c_char {
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
        fn_: impl FnOnce(*const std::ffi::c_char) -> LLVMValueRef,
    ) -> LLVMValueRef {
        let t = self.get_or_ad_impl(word, |dst| fn_(dst.as_ptr().cast()));
        self.register(word, t);
        t
    }

    fn with_result_option(
        &mut self,
        word: Option<SpirvWord>,
        fn_: impl FnOnce(*const std::ffi::c_char) -> LLVMValueRef,
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
            ptx_parser::ScalarType::BF16 => write!(f, "bf16"),
            ast::ScalarType::F32 => write!(f, "f32"),
            ast::ScalarType::F64 => write!(f, "f64"),
            ptx_parser::ScalarType::S16x2 | ptx_parser::ScalarType::U16x2 => write!(f, "v2i16"),
            ast::ScalarType::F16x2 => write!(f, "v2f16"),
            ptx_parser::ScalarType::BF16x2 => write!(f, "v2bf16"),
            ptx_parser::ScalarType::F32x2 => write!(f, "v2f32"),
        }
    }
}

struct FloatingPoint {
    context: LLVMContextRef,
    module: LLVMModuleRef,
    builder: LLVMBuilderRef,
    mode: FloatingPointMode,
}

impl FloatingPoint {
    fn emit_constrained<const N: usize, const M: usize>(
        &self,
        resolver: &mut ResolveIdent,
        name: &'static str,
        type_: ast::ScalarType,
        dst: Option<SpirvWord>,
        srcs: [LLVMValueRef; N],
        metadata: [&str; M],
    ) -> Result<LLVMValueRef, TranslateError> {
        let name = format!("{}.{}\0", name, LLVMTypeDisplay(type_));
        self.emit_constrained_impl(resolver, &name, type_, dst, type_, srcs, metadata)
    }

    // Constrained conversions are overloaded on both the destination and the
    // source type, e.g. llvm.experimental.constrained.fptosi.i32.f32
    fn emit_constrained_convert<const M: usize>(
        &self,
        resolver: &mut ResolveIdent,
        name: &'static str,
        dst_type: ast::ScalarType,
        dst: Option<SpirvWord>,
        src_type: ast::ScalarType,
        src: LLVMValueRef,
        metadata: [&str; M],
    ) -> Result<LLVMValueRef, TranslateError> {
        let name = format!(
            "{}.{}.{}\0",
            name,
            LLVMTypeDisplay(dst_type),
            LLVMTypeDisplay(src_type)
        );
        self.emit_constrained_impl(resolver, &name, dst_type, dst, src_type, [src], metadata)
    }

    fn emit_constrained_impl<const N: usize, const M: usize>(
        &self,
        resolver: &mut ResolveIdent,
        name: &str,
        dst_type: ast::ScalarType,
        dst: Option<SpirvWord>,
        src_type: ast::ScalarType,
        srcs: [LLVMValueRef; N],
        metadata: [&str; M],
    ) -> Result<LLVMValueRef, TranslateError> {
        let metadata_type = unsafe { LLVMMetadataTypeInContext(self.context) };
        let metadata = metadata.map(|m| {
            let m_value =
                unsafe { LLVMMDStringInContext2(self.context, m.as_ptr().cast(), m.len()) };
            (
                unsafe { LLVMMetadataAsValue(self.context, m_value) },
                metadata_type,
            )
        });
        let name = unsafe { CString::from_vec_unchecked(name.as_bytes().to_vec()) };
        emit_intrinsic(
            self.context,
            self.module,
            self.builder,
            resolver,
            &name,
            dst,
            vec![&dst_type.into()],
            srcs.iter()
                .map(|&src| (src, get_scalar_type(self.context, src_type)))
                .chain(metadata.into_iter())
                .collect(),
        )
    }

    fn fadd(
        &self,
        resolver: &mut ResolveIdent,
        type_: ast::ScalarType,
        dst: SpirvWord,
        src1: LLVMValueRef,
        src2: LLVMValueRef,
    ) -> Result<LLVMValueRef, TranslateError> {
        match self.mode {
            FloatingPointMode::Normal => Ok(resolver.with_result(dst, |dst| unsafe {
                LLVMBuildFAdd(self.builder, src1, src2, dst)
            })),
            FloatingPointMode::Constrained => self.emit_constrained(
                resolver,
                "llvm.experimental.constrained.fadd",
                type_,
                Some(dst),
                [src1, src2],
                ["round.dynamic", "fpexcept.ignore"],
            ),
        }
    }

    fn fmul(
        &self,
        resolver: &mut ResolveIdent,
        type_: ast::ScalarType,
        dst: Option<SpirvWord>,
        src1: SpirvWord,
        src2: SpirvWord,
    ) -> Result<LLVMValueRef, TranslateError> {
        let src1 = resolver.value(src1)?;
        let src2 = resolver.value(src2)?;
        match self.mode {
            FloatingPointMode::Normal => Ok(resolver.with_result_option(dst, |dst| unsafe {
                LLVMBuildFMul(self.builder, src1, src2, dst)
            })),
            FloatingPointMode::Constrained => self.emit_constrained(
                resolver,
                "llvm.experimental.constrained.fmul",
                type_,
                dst,
                [src1, src2],
                ["round.dynamic", "fpexcept.ignore"],
            ),
        }
    }

    fn fdiv(
        &self,
        resolver: &mut ResolveIdent,
        type_: ast::ScalarType,
        dst: SpirvWord,
        src1: LLVMValueRef,
        src2: LLVMValueRef,
    ) -> Result<LLVMValueRef, TranslateError> {
        match self.mode {
            FloatingPointMode::Normal => Ok(resolver.with_result(dst, |dst| unsafe {
                LLVMBuildFDiv(self.builder, src1, src2, dst)
            })),
            FloatingPointMode::Constrained => self.emit_constrained(
                resolver,
                "llvm.experimental.constrained.fdiv",
                type_,
                Some(dst),
                [src1, src2],
                ["round.dynamic", "fpexcept.ignore"],
            ),
        }
    }

    fn fsub(
        &self,
        resolver: &mut ResolveIdent,
        type_: ast::ScalarType,
        dst: SpirvWord,
        src1: LLVMValueRef,
        src2: LLVMValueRef,
    ) -> Result<LLVMValueRef, TranslateError> {
        match self.mode {
            FloatingPointMode::Normal => Ok(resolver.with_result(dst, |dst| unsafe {
                LLVMBuildFSub(self.builder, src1, src2, dst)
            })),
            FloatingPointMode::Constrained => self.emit_constrained(
                resolver,
                "llvm.experimental.constrained.fsub",
                type_,
                Some(dst),
                [src1, src2],
                ["round.dynamic", "fpexcept.ignore"],
            ),
        }
    }

    fn fpext(
        &self,
        resolver: &mut ResolveIdent,
        dst_type: ast::ScalarType,
        dst: Option<SpirvWord>,
        src_type: ast::ScalarType,
        src: LLVMValueRef,
    ) -> Result<LLVMValueRef, TranslateError> {
        match self.mode {
            FloatingPointMode::Normal => Ok(resolver.with_result_option(dst, |dst| unsafe {
                LLVMBuildFPExt(
                    self.builder,
                    src,
                    get_scalar_type(self.context, dst_type),
                    dst,
                )
            })),
            FloatingPointMode::Constrained => self.emit_constrained_convert(
                resolver,
                "llvm.experimental.constrained.fpext",
                dst_type,
                dst,
                src_type,
                src,
                ["fpexcept.ignore"],
            ),
        }
    }

    fn fptrunc(
        &self,
        resolver: &mut ResolveIdent,
        dst_type: ast::ScalarType,
        dst: Option<SpirvWord>,
        src_type: ast::ScalarType,
        src: LLVMValueRef,
    ) -> Result<LLVMValueRef, TranslateError> {
        match self.mode {
            FloatingPointMode::Normal => Ok(resolver.with_result_option(dst, |dst| unsafe {
                LLVMBuildFPTrunc(
                    self.builder,
                    src,
                    get_scalar_type(self.context, dst_type),
                    dst,
                )
            })),
            FloatingPointMode::Constrained => self.emit_constrained_convert(
                resolver,
                "llvm.experimental.constrained.fptrunc",
                dst_type,
                dst,
                src_type,
                src,
                ["round.dynamic", "fpexcept.ignore"],
            ),
        }
    }

    fn fptosi(
        &self,
        resolver: &mut ResolveIdent,
        dst_type: ast::ScalarType,
        dst: Option<SpirvWord>,
        src_type: ast::ScalarType,
        src: LLVMValueRef,
    ) -> Result<LLVMValueRef, TranslateError> {
        match self.mode {
            FloatingPointMode::Normal => Ok(resolver.with_result_option(dst, |dst| unsafe {
                LLVMBuildFPToSI(
                    self.builder,
                    src,
                    get_scalar_type(self.context, dst_type),
                    dst,
                )
            })),
            FloatingPointMode::Constrained => self.emit_constrained_convert(
                resolver,
                "llvm.experimental.constrained.fptosi",
                dst_type,
                dst,
                src_type,
                src,
                ["fpexcept.ignore"],
            ),
        }
    }

    fn fptoui(
        &self,
        resolver: &mut ResolveIdent,
        dst_type: ast::ScalarType,
        dst: Option<SpirvWord>,
        src_type: ast::ScalarType,
        src: LLVMValueRef,
    ) -> Result<LLVMValueRef, TranslateError> {
        match self.mode {
            FloatingPointMode::Normal => Ok(resolver.with_result_option(dst, |dst| unsafe {
                LLVMBuildFPToUI(
                    self.builder,
                    src,
                    get_scalar_type(self.context, dst_type),
                    dst,
                )
            })),
            FloatingPointMode::Constrained => self.emit_constrained_convert(
                resolver,
                "llvm.experimental.constrained.fptoui",
                dst_type,
                dst,
                src_type,
                src,
                ["fpexcept.ignore"],
            ),
        }
    }

    fn sitofp(
        &self,
        resolver: &mut ResolveIdent,
        dst_type: ast::ScalarType,
        dst: Option<SpirvWord>,
        src_type: ast::ScalarType,
        src: LLVMValueRef,
    ) -> Result<LLVMValueRef, TranslateError> {
        match self.mode {
            FloatingPointMode::Normal => Ok(resolver.with_result_option(dst, |dst| unsafe {
                LLVMBuildSIToFP(
                    self.builder,
                    src,
                    get_scalar_type(self.context, dst_type),
                    dst,
                )
            })),
            FloatingPointMode::Constrained => self.emit_constrained_convert(
                resolver,
                "llvm.experimental.constrained.sitofp",
                dst_type,
                dst,
                src_type,
                src,
                ["round.dynamic", "fpexcept.ignore"],
            ),
        }
    }

    fn uitofp(
        &self,
        resolver: &mut ResolveIdent,
        dst_type: ast::ScalarType,
        dst: Option<SpirvWord>,
        src_type: ast::ScalarType,
        src: LLVMValueRef,
    ) -> Result<LLVMValueRef, TranslateError> {
        match self.mode {
            FloatingPointMode::Normal => Ok(resolver.with_result_option(dst, |dst| unsafe {
                LLVMBuildUIToFP(
                    self.builder,
                    src,
                    get_scalar_type(self.context, dst_type),
                    dst,
                )
            })),
            FloatingPointMode::Constrained => self.emit_constrained_convert(
                resolver,
                "llvm.experimental.constrained.uitofp",
                dst_type,
                dst,
                src_type,
                src,
                ["round.dynamic", "fpexcept.ignore"],
            ),
        }
    }

    fn sin_f32(
        &self,
        resolver: &mut ResolveIdent,
        dst: SpirvWord,
        src: LLVMValueRef,
    ) -> Result<LLVMValueRef, TranslateError> {
        match self.mode {
            FloatingPointMode::Normal => emit_intrinsic(
                self.context,
                self.module,
                self.builder,
                resolver,
                c"llvm.sin.f32",
                Some(dst),
                vec![&ast::ScalarType::F32.into()],
                vec![(src, get_scalar_type(self.context, ast::ScalarType::F32))],
            ),
            FloatingPointMode::Constrained => self.emit_constrained(
                resolver,
                "llvm.experimental.constrained.sin",
                ast::ScalarType::F32,
                Some(dst),
                [src],
                ["round.dynamic", "fpexcept.ignore"],
            ),
        }
    }

    fn cos_f32(
        &self,
        resolver: &mut ResolveIdent,
        dst: SpirvWord,
        src: LLVMValueRef,
    ) -> Result<LLVMValueRef, TranslateError> {
        match self.mode {
            FloatingPointMode::Normal => emit_intrinsic(
                self.context,
                self.module,
                self.builder,
                resolver,
                c"llvm.cos.f32",
                Some(dst),
                vec![&ast::ScalarType::F32.into()],
                vec![(src, get_scalar_type(self.context, ast::ScalarType::F32))],
            ),
            FloatingPointMode::Constrained => self.emit_constrained(
                resolver,
                "llvm.experimental.constrained.cos",
                ast::ScalarType::F32,
                Some(dst),
                [src],
                ["round.dynamic", "fpexcept.ignore"],
            ),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FloatingPointMode {
    Normal,
    Constrained,
}

fn emit_intrinsic(
    context: LLVMContextRef,
    module: LLVMModuleRef,
    builder: LLVMBuilderRef,
    resolver: &mut ResolveIdent,
    name: &CStr,
    dst: Option<SpirvWord>,
    return_types: Vec<&ast::Type>,
    arguments: Vec<(LLVMValueRef, LLVMTypeRef)>,
) -> Result<LLVMValueRef, TranslateError> {
    let fn_type = get_intrinsic_type(
        context,
        return_types.iter().copied(),
        arguments.iter().map(|(_, type_)| Ok(*type_)),
    )?;
    let mut fn_ = unsafe { LLVMGetNamedFunction(module, name.as_ptr()) };
    if fn_ == ptr::null_mut() {
        fn_ = unsafe { LLVMAddFunction(module, name.as_ptr(), fn_type) };
    }
    let mut arguments = arguments.iter().map(|(arg, _)| *arg).collect::<Vec<_>>();
    Ok(resolver.with_result_option(dst, |dst| unsafe {
        LLVMBuildCall2(
            builder,
            fn_type,
            fn_,
            arguments.as_mut_ptr(),
            arguments.len() as u32,
            dst,
        )
    }))
}
