use cuda_types::cuda::*;
use hip_runtime_sys::*;
use std::{ffi::c_void, ptr};

fn memory_type(cu: hipMemoryType) -> Result<CUmemorytype, hipErrorCode_t> {
    match cu {
        hipMemoryType::hipMemoryTypeHost => Ok(CUmemorytype::CU_MEMORYTYPE_HOST),
        hipMemoryType::hipMemoryTypeDevice => Ok(CUmemorytype::CU_MEMORYTYPE_DEVICE),
        hipMemoryType::hipMemoryTypeArray => Ok(CUmemorytype::CU_MEMORYTYPE_ARRAY),
        hipMemoryType::hipMemoryTypeUnified => Ok(CUmemorytype::CU_MEMORYTYPE_UNIFIED),
        _ => Err(hipErrorCode_t::InvalidValue),
    }
}

pub(crate) unsafe fn get_attribute(
    data: *mut c_void,
    attribute: hipPointer_attribute,
    ptr: hipDeviceptr_t,
) -> hipError_t {
    if data == ptr::null_mut() {
        return hipError_t::ErrorInvalidValue;
    }
    match attribute {
        // TODO: implement by getting device ordinal & allocation start,
        // then go through every context for that device
        hipPointer_attribute::HIP_POINTER_ATTRIBUTE_CONTEXT => hipError_t::ErrorNotSupported,
        hipPointer_attribute::HIP_POINTER_ATTRIBUTE_MEMORY_TYPE => {
            let mut hip_result = hipMemoryType(0);
            hipPointerGetAttribute(
                (&mut hip_result as *mut hipMemoryType).cast::<c_void>(),
                attribute,
                ptr,
            )?;
            let cuda_result = memory_type(hip_result)?;
            unsafe { *(data.cast()) = cuda_result };
            Ok(())
        }
        _ => unsafe { hipPointerGetAttribute(data, attribute, ptr) },
    }
}

pub(crate) unsafe fn get_attributes(
    num_attributes: ::core::ffi::c_uint,
    attributes: &mut hipPointer_attribute,
    data: &mut *mut ::core::ffi::c_void,
    ptr: hipDeviceptr_t,
) -> hipError_t {
    todo!("Implement CU_POINTER_ATTRIBUTE_SYNC_MEMOPS");
    //hipDrvPointerGetAttributes(num_attributes, attributes, data, ptr)
}
