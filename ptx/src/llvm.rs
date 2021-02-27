/*
 * This is a minimal home-made LLVM wrapper, we don't use:
 * Inkwell: it's too strongly-typed (splitting Value into variants) and does not
 * expose underlying raw LLVM value
 * LLVM-ir: can't emit code
 * rustc_codegen_llvm: the best of the bunch, but too much work to split it out
 */

use std::error::Error;
use std::ffi::CStr;
use std::fmt::{self, Debug, Display};
use std::mem;
use std::ptr;

use zluda_llvm::analysis::*;
use zluda_llvm::core::*;
use zluda_llvm::prelude::*;

macro_rules! llvm_wrapper {
    ($([
        $rust_name:ident,
        $llvm_name:path,
        $ctor:ident( $($arg_name:ident : $arg_type: ty),* ),
        $dtor:path
    ]),+) => {
        $(
            pub struct $rust_name($llvm_name);

            impl Drop for $rust_name {
                fn drop(&mut self) {
                    unsafe { $dtor(self.0) }
                }
            }

            impl $rust_name {
                #[allow(non_snake_case, dead_code)]
                pub(crate) unsafe fn create( $($arg_name : $arg_type),* ) -> $rust_name {
                    $rust_name($ctor( $($arg_name),* ))
                }

                #[allow(non_snake_case, dead_code)]
                pub(crate) unsafe fn from_ffi(x: $llvm_name) -> $rust_name {
                    $rust_name(x)
                }

                #[allow(dead_code)]
                pub(crate) fn get(&self) -> $llvm_name {
                    self.0
                }
            }
        )+
    };
}

llvm_wrapper!(
    [
        Context,
        LLVMContextRef,
        LLVMContextCreate(),
        LLVMContextDispose
    ],
    [
        Module,
        LLVMModuleRef,
        LLVMModuleCreateWithNameInContext(ModuleID: *const i8, C: LLVMContextRef),
        LLVMDisposeModule
    ],
    [
        Builder,
        LLVMBuilderRef,
        LLVMCreateBuilderInContext(C: LLVMContextRef),
        LLVMDisposeBuilder
    ],
    [
        MemoryBuffer,
        LLVMMemoryBufferRef,
        LLVMCreateMemoryBufferWithMemoryRangeCopy(
            InputData: *const std::os::raw::c_char,
            InputDataLength: usize,
            BufferName: *const std::os::raw::c_char
        ),
        LLVMDisposeMemoryBuffer
    ]
);

impl Module {
    pub(crate) fn verify(&self) -> Option<Message> {
        let mut msg = ptr::null_mut();
        let is_broken = unsafe {
            LLVMVerifyModule(
                self.get(),
                LLVMVerifierFailureAction::LLVMReturnStatusAction,
                &mut msg,
            )
        };
        if is_broken != 0 {
            Some(Message::from_ffi(msg))
        } else {
            None
        }
    }
}

impl MemoryBuffer {
    pub unsafe fn create_no_copy<T>(input: &[T], requires_null_terminator: bool) -> Self {
        let mut len = input.len();
        if requires_null_terminator {
            len -= 1;
        }
        MemoryBuffer(LLVMCreateMemoryBufferWithMemoryRange(
            input.as_ptr() as _,
            len * mem::size_of::<T>(),
            ptr::null_mut(),
            if requires_null_terminator { 1 } else { 0 },
        ))
    }

    pub fn as_slice<'a>(&'a self) -> &'a [u8] {
        let len = unsafe { LLVMGetBufferSize(self.get()) };
        let start = unsafe { LLVMGetBufferStart(self.get()) };
        unsafe { std::slice::from_raw_parts(start as _, len) }
    }
}

impl AsRef<[u8]> for MemoryBuffer {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

#[cfg(test)]
pub(crate) unsafe fn parse_ir_in_context(
    context: &Context,
    mem_buf: MemoryBuffer,
) -> Result<Module, Message> {
    use zluda_llvm::ir_reader::*;
    let mut module = ptr::null_mut();
    let mut message = ptr::null_mut();
    let result = LLVMParseIRInContext(context.get(), mem_buf.get(), &mut module, &mut message);
    mem::forget(mem_buf);
    if result == 0 {
        Ok(Module::from_ffi(module))
    } else {
        Err(Message(message))
    }
}
pub struct Message(*mut i8);

impl Error for Message {}

impl Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_cstr().to_str().map_err(|_| fmt::Error)?)
    }
}

impl Message {
    pub(crate) fn as_cstr<'a>(&'a self) -> &'a CStr {
        unsafe { CStr::from_ptr(self.0) }
    }

    pub(crate) fn from_ffi(msg: *mut i8) -> Self {
        Self(msg)
    }
}

impl Drop for Message {
    fn drop(&mut self) {
        unsafe { LLVMDisposeMessage(self.0) };
    }
}

pub(crate) fn void_type(llvm_ctx: &Context) -> LLVMTypeRef {
    unsafe { LLVMVoidTypeInContext(llvm_ctx.get()) }
}
