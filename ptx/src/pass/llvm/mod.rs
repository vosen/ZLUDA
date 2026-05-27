pub(super) mod attributes;
pub(super) mod emit;

use std::ffi::CStr;

use crate::pass::*;
use llvm_zluda::core::*;
use llvm_zluda::prelude::*;

const LLVM_UNNAMED: &CStr = c"";

// https://llvm.org/docs/AMDGPUUsage.html#address-spaces
const GENERIC_ADDRESS_SPACE: u32 = 0;
const GLOBAL_ADDRESS_SPACE: u32 = 1;
const SHARED_ADDRESS_SPACE: u32 = 3;
const CONSTANT_ADDRESS_SPACE: u32 = 4;
const PRIVATE_ADDRESS_SPACE: u32 = 5;

fn get_scalar_type(context: LLVMContextRef, type_: ast::ScalarType) -> LLVMTypeRef {
    match type_ {
        ast::ScalarType::Pred => unsafe { LLVMInt1TypeInContext(context) },
        ast::ScalarType::S8 | ast::ScalarType::B8 | ast::ScalarType::U8 => unsafe {
            LLVMInt8TypeInContext(context)
        },
        ast::ScalarType::B16
        | ast::ScalarType::U16
        | ast::ScalarType::S16
        | ast::ScalarType::E4m3x2
        | ast::ScalarType::E5m2x2 => unsafe { LLVMInt16TypeInContext(context) },
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
        ast::ScalarType::U16x2 | ast::ScalarType::S16x2 => unsafe {
            LLVMVectorType(LLVMInt16TypeInContext(context), 2)
        },
        ast::ScalarType::F16x2 => unsafe { LLVMVectorType(LLVMHalfTypeInContext(context), 2) },
        ast::ScalarType::BF16x2 => unsafe { LLVMVectorType(LLVMBFloatTypeInContext(context), 2) },
    }
}

fn get_state_space(space: ast::StateSpace) -> Result<u32, TranslateError> {
    match space {
        ast::StateSpace::Reg => Ok(PRIVATE_ADDRESS_SPACE),
        ast::StateSpace::Generic => Ok(GENERIC_ADDRESS_SPACE),
        // This is dodgy, we try our best to convert all .param into either
        // .param::entry or .local, but we can't always succeed.
        // In those cases we convert .param into generic address space
        ast::StateSpace::Param => Ok(GENERIC_ADDRESS_SPACE),
        ast::StateSpace::ParamEntry => Ok(CONSTANT_ADDRESS_SPACE),
        ast::StateSpace::ParamFunc => Err(error_todo()),
        ast::StateSpace::Local => Ok(PRIVATE_ADDRESS_SPACE),
        ast::StateSpace::Global => Ok(GLOBAL_ADDRESS_SPACE),
        ast::StateSpace::Const => Ok(CONSTANT_ADDRESS_SPACE),
        ast::StateSpace::Shared => Ok(SHARED_ADDRESS_SPACE),
        ast::StateSpace::SharedCta => Err(error_todo()),
        ast::StateSpace::SharedCluster => Err(error_todo()),
    }
}
