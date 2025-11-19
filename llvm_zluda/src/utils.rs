use std::ffi::CStr;
use std::ops::Deref;
use std::{mem, ptr};

use llvm_sys::analysis::{LLVMVerifierFailureAction, LLVMVerifyModule};
use llvm_sys::bit_reader::LLVMParseBitcodeInContext2;
use llvm_sys::bit_writer::LLVMWriteBitcodeToMemoryBuffer;
use llvm_sys::core::*;
use llvm_sys::linker::LLVMLinkModules2;
use llvm_sys::prelude::*;
use llvm_sys::target_machine::{
    LLVMCodeGenFileType, LLVMCodeGenOptLevel, LLVMCodeModel, LLVMCreateTargetMachine,
    LLVMDisposeTargetMachine, LLVMRelocMode, LLVMTargetMachineEmitToMemoryBuffer,
    LLVMTargetMachineRef, LLVMTargetRef,
};
use llvm_sys::transforms::pass_builder::{
    LLVMCreatePassBuilderOptions, LLVMDisposePassBuilderOptions, LLVMPassBuilderOptionsRef,
};

pub struct Context(LLVMContextRef);

impl Context {
    pub fn new() -> Self {
        Self(unsafe { LLVMContextCreate() })
    }

    pub fn get(&self) -> LLVMContextRef {
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

pub struct Module(LLVMModuleRef);

impl Module {
    pub fn new(ctx: &Context, name: &CStr) -> Self {
        Self(unsafe { LLVMModuleCreateWithNameInContext(name.as_ptr(), ctx.get()) })
    }

    pub fn try_from_bitcode(ctx: &Context, bitcode: &[u8], name: Option<&CStr>) -> Option<Self> {
        let bitcode: Vec<i8> = bitcode.iter().map(|&v| i8::from_ne_bytes([v])).collect();
        let memory_buffer: LLVMMemoryBufferRef = unsafe {
            LLVMCreateMemoryBufferWithMemoryRangeCopy(
                bitcode.as_ptr(),
                bitcode.len(),
                name.map(|n| n.as_ptr()).unwrap_or(ptr::null()),
            )
        };
        let mut module: LLVMModuleRef = unsafe { mem::zeroed() };
        let status = unsafe { LLVMParseBitcodeInContext2(ctx.get(), memory_buffer, &mut module) };
        if status != 0 {
            None
        } else {
            Some(Self(module))
        }
    }

    pub fn clone(&self) -> Self {
        Self(unsafe { LLVMCloneModule(self.0) })
    }

    pub fn get(&self) -> LLVMModuleRef {
        self.0
    }

    pub fn verify(&self) -> Result<(), String> {
        let mut err = ptr::null_mut();
        let error = unsafe {
            LLVMVerifyModule(
                self.get(),
                LLVMVerifierFailureAction::LLVMReturnStatusAction,
                &mut err,
            )
        };
        if error == 1 && err != ptr::null_mut() {
            let message = Message(unsafe { CStr::from_ptr(err) });
            Err(message.to_str().to_owned())
        } else {
            Ok(())
        }
    }

    pub fn write_bitcode_to_memory(&self) -> MemoryBuffer {
        let memory_buffer = unsafe { LLVMWriteBitcodeToMemoryBuffer(self.get()) };
        MemoryBuffer(memory_buffer)
    }

    pub fn print_module_to_string(&self) -> Message {
        let asm = unsafe { LLVMPrintModuleToString(self.get()) };
        Message(unsafe { CStr::from_ptr(asm) })
    }

    pub fn link(&self, other: Module) -> Result<(), &'static str> {
        let status = unsafe { LLVMLinkModules2(self.get(), other.into_raw()) };
        if status != 0 {
            Err("could not link file")
        } else {
            Ok(())
        }
    }

    fn into_raw(self) -> LLVMModuleRef {
        let ptr = self.0;
        mem::forget(self);
        ptr
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeModule(self.0);
        }
    }
}

pub fn bitcode_to_ir(bitcode: Vec<u8>) -> Option<Vec<u8>> {
    let context = Context::new();
    let module = Module::try_from_bitcode(&context, &bitcode, None)?;
    Some(module.print_module_to_string().to_bytes().to_vec())
}

pub struct Message(&'static CStr);

impl Message {
    pub fn new(cstr: &'static CStr) -> Self {
        Self(cstr)
    }
}

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

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string_lossy())
    }
}

impl Message {
    pub fn to_bytes(&self) -> &[u8] {
        self.0.to_bytes()
    }

    pub fn to_str(&self) -> &str {
        self.0.to_str().unwrap().trim()
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

pub struct TargetMachine(LLVMTargetMachineRef);

impl TargetMachine {
    pub fn new(
        target: LLVMTargetRef,
        triple: &CStr,
        cpu: &CStr,
        features: &CStr,
        opt_level: LLVMCodeGenOptLevel,
        reloc_mode: LLVMRelocMode,
        code_model: LLVMCodeModel,
    ) -> Self {
        Self(unsafe {
            LLVMCreateTargetMachine(
                target,
                triple.as_ptr(),
                cpu.as_ptr(),
                features.as_ptr(),
                opt_level,
                reloc_mode,
                code_model,
            )
        })
    }

    pub fn get(&self) -> LLVMTargetMachineRef {
        self.0
    }

    pub fn emit_to_memory_buffer(
        &self,
        module: &Module,
        file_type: LLVMCodeGenFileType,
    ) -> Result<MemoryBuffer, String> {
        let mut buffer = unsafe { mem::zeroed() };
        let mut err = ptr::null_mut();
        let status = unsafe {
            LLVMTargetMachineEmitToMemoryBuffer(
                self.0,
                module.get(),
                file_type,
                &mut err,
                &mut buffer,
            )
        };
        if status != 0 {
            let message = Message(unsafe { CStr::from_ptr(err) });
            Err(message.to_str().to_owned())
        } else {
            Ok(MemoryBuffer(buffer))
        }
    }
}

impl Drop for TargetMachine {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeTargetMachine(self.0);
        }
    }
}

pub struct PassBuilderOptions(LLVMPassBuilderOptionsRef);

impl PassBuilderOptions {
    pub fn new() -> Self {
        Self(unsafe { LLVMCreatePassBuilderOptions() })
    }

    pub fn get(&self) -> LLVMPassBuilderOptionsRef {
        self.0
    }
}

impl Drop for PassBuilderOptions {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposePassBuilderOptions(self.0);
        }
    }
}
