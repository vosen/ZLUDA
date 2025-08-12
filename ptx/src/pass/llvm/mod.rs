pub(super) mod attributes;
pub(super) mod emit;

use std::ffi::CStr;
use std::mem;
use std::ops::Deref;
use std::ptr;

use crate::pass::*;
use llvm_zluda::analysis::{LLVMVerifierFailureAction, LLVMVerifyModule};
use llvm_zluda::bit_reader::LLVMParseBitcodeInContext2;
use llvm_zluda::bit_writer::LLVMWriteBitcodeToMemoryBuffer;
use llvm_zluda::core::*;
use llvm_zluda::prelude::*;

const LLVM_UNNAMED: &CStr = c"";

// https://llvm.org/docs/AMDGPUUsage.html#address-spaces
const GENERIC_ADDRESS_SPACE: u32 = 0;
const GLOBAL_ADDRESS_SPACE: u32 = 1;
const SHARED_ADDRESS_SPACE: u32 = 3;
const CONSTANT_ADDRESS_SPACE: u32 = 4;
const PRIVATE_ADDRESS_SPACE: u32 = 5;

pub(super) struct Context(LLVMContextRef);

impl Context {
    pub fn new() -> Self {
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

pub struct Module(LLVMModuleRef);

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

    pub fn write_bitcode_to_memory(&self) -> MemoryBuffer {
        let memory_buffer = unsafe { LLVMWriteBitcodeToMemoryBuffer(self.get()) };
        MemoryBuffer(memory_buffer)
    }

    pub fn print_module_to_string(&self) -> Message {
        let asm = unsafe { LLVMPrintModuleToString(self.get()) };
        Message(unsafe { CStr::from_ptr(asm) })
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeModule(self.0);
        }
    }
}

pub fn bitcode_to_ir(bitcode: Vec<u8>) -> Vec<u8> {
    let bitcode: Vec<i8> = bitcode.iter().map(|&v| i8::from_ne_bytes([v])).collect();
    let memory_buffer: LLVMMemoryBufferRef = unsafe {
            LLVMCreateMemoryBufferWithMemoryRangeCopy(bitcode.as_ptr(), bitcode.len(), ptr::null())
    };
    let context = Context::new();
    let mut module: LLVMModuleRef = unsafe { mem::zeroed() };
    unsafe {
        LLVMParseBitcodeInContext2(context.get(), memory_buffer, &mut module);
    }
    let module = Module(module);
    module.print_module_to_string().to_bytes().to_vec()
}

pub struct Message(&'static CStr);

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

fn get_state_space(space: ast::StateSpace) -> Result<u32, TranslateError> {
    match space {
        ast::StateSpace::Reg => Ok(PRIVATE_ADDRESS_SPACE),
        ast::StateSpace::Generic => Ok(GENERIC_ADDRESS_SPACE),
        ast::StateSpace::Param => Err(TranslateError::Todo("".to_string())),
        ast::StateSpace::ParamEntry => Ok(CONSTANT_ADDRESS_SPACE),
        ast::StateSpace::ParamFunc => Err(TranslateError::Todo("".to_string())),
        ast::StateSpace::Local => Ok(PRIVATE_ADDRESS_SPACE),
        ast::StateSpace::Global => Ok(GLOBAL_ADDRESS_SPACE),
        ast::StateSpace::Const => Ok(CONSTANT_ADDRESS_SPACE),
        ast::StateSpace::Shared => Ok(SHARED_ADDRESS_SPACE),
        ast::StateSpace::SharedCta => Err(TranslateError::Todo("".to_string())),
        ast::StateSpace::SharedCluster => Err(TranslateError::Todo("".to_string())),
    }
}
