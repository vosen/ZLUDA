use std::{ffi::c_void, mem, ptr};

use hip_runtime_sys::{hipError_t, hipMemoryType, hipPointerGetAttributes};

use crate::{
    cuda::{CUdeviceptr, CUmemorytype, CUpointer_attribute},
    hip_call,
};

pub(crate) unsafe fn get_attribute(
    data: *mut c_void,
    attribute: CUpointer_attribute,
    ptr: CUdeviceptr,
) -> Result<(), hipError_t> {
    if data == ptr::null_mut() {
        return Err(hipError_t::hipErrorInvalidValue);
    }
    let mut attribs = mem::zeroed();
    hip_call! { hipPointerGetAttributes(&mut attribs, ptr.0 as _) };
    match attribute {
        CUpointer_attribute::CU_POINTER_ATTRIBUTE_CONTEXT => {
            *(data as *mut _) = attribs.device;
            Ok(())
        }
        CUpointer_attribute::CU_POINTER_ATTRIBUTE_MEMORY_TYPE => {
            *(data as *mut _) = memory_type(attribs.memoryType)?;
            Ok(())
        }
        CUpointer_attribute::CU_POINTER_ATTRIBUTE_DEVICE_POINTER => {
            *(data as *mut _) = attribs.devicePointer;
            Ok(())
        }
        CUpointer_attribute::CU_POINTER_ATTRIBUTE_HOST_POINTER => {
            *(data as *mut _) = attribs.hostPointer;
            Ok(())
        }
        CUpointer_attribute::CU_POINTER_ATTRIBUTE_IS_MANAGED => {
            *(data as *mut _) = attribs.isManaged;
            Ok(())
        }
        _ => Err(hipError_t::hipErrorNotSupported),
    }
}

pub(crate) fn memory_type(cu: hipMemoryType) -> Result<CUmemorytype, hipError_t> {
    match cu {
        hipMemoryType::hipMemoryTypeHost => Ok(CUmemorytype::CU_MEMORYTYPE_HOST),
        hipMemoryType::hipMemoryTypeDevice => Ok(CUmemorytype::CU_MEMORYTYPE_DEVICE),
        hipMemoryType::hipMemoryTypeArray => Ok(CUmemorytype::CU_MEMORYTYPE_ARRAY),
        hipMemoryType::hipMemoryTypeUnified => Ok(CUmemorytype::CU_MEMORYTYPE_UNIFIED),
        _ => Err(hipError_t::hipErrorInvalidValue),
    }
}
