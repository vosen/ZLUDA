use cuda_types::*;
use hip_runtime_sys::*;
use std::{ffi::c_void, ptr};

pub(crate) unsafe fn get_attribute(
    data: *mut c_void,
    attribute: hipPointer_attribute,
    ptr: hipDeviceptr_t,
) -> CUresult {
    if data == ptr::null_mut() {
        return CUresult::ERROR_INVALID_VALUE;
    }
    // TODO: implement by getting device ordinal & allocation start,
    // then go through every context for that device
    if attribute == hipPointer_attribute::HIP_POINTER_ATTRIBUTE_CONTEXT {
        return CUresult::ERROR_NOT_SUPPORTED;
    }
    if attribute == hipPointer_attribute::HIP_POINTER_ATTRIBUTE_MEMORY_TYPE {
        let mut hip_result = hipMemoryType(0);
        hipPointerGetAttribute(
            (&mut hip_result as *mut hipMemoryType).cast::<c_void>(),
            attribute,
            ptr,
        )?;
        let cuda_result = memory_type(hip_result)?;
        *(data as _) = cuda_result;
    } else {
        hipPointerGetAttribute(data, attribute, ptr)?;
    }
    Ok(())
}

fn memory_type(cu: hipMemoryType) -> Result<CUmemorytype, hipErrorCode_t> {
    match cu {
        hipMemoryType::hipMemoryTypeHost => Ok(CUmemorytype::CU_MEMORYTYPE_HOST),
        hipMemoryType::hipMemoryTypeDevice => Ok(CUmemorytype::CU_MEMORYTYPE_DEVICE),
        hipMemoryType::hipMemoryTypeArray => Ok(CUmemorytype::CU_MEMORYTYPE_ARRAY),
        hipMemoryType::hipMemoryTypeUnified => Ok(CUmemorytype::CU_MEMORYTYPE_UNIFIED),
        _ => Err(hipErrorCode_t::hipErrorInvalidValue),
    }
}
