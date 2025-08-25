use cuda_types::cuda::*;
use hip_runtime_sys::*;
use std::{ffi::c_void, ptr};

use crate::r#impl::driver;

// TODO: handlehipMemoryTypeUnregistered
fn to_cu_memory_type(cu: hipMemoryType) -> Result<CUmemorytype, hipErrorCode_t> {
    match cu {
        hipMemoryType::hipMemoryTypeHost => Ok(CUmemorytype::CU_MEMORYTYPE_HOST),
        hipMemoryType::hipMemoryTypeDevice => Ok(CUmemorytype::CU_MEMORYTYPE_DEVICE),
        hipMemoryType::hipMemoryTypeArray => Ok(CUmemorytype::CU_MEMORYTYPE_ARRAY),
        // TODO: check if this is correct
        hipMemoryType::hipMemoryTypeManaged => Ok(CUmemorytype::CU_MEMORYTYPE_UNIFIED),
        hipMemoryType::hipMemoryTypeUnified => Ok(CUmemorytype::CU_MEMORYTYPE_UNIFIED),
        _ => Err(hipErrorCode_t::InvalidValue),
    }
}

pub(crate) unsafe fn get_attribute(
    data: *mut c_void,
    attribute: hipPointer_attribute,
    ptr: hipDeviceptr_t,
) -> CUresult {
    if data == ptr::null_mut() {
        return CUresult::ERROR_INVALID_VALUE;
    }
    match attribute {
        hipPointer_attribute::HIP_POINTER_ATTRIBUTE_CONTEXT => {
            let globals = driver::global_state()?;
            let allocations = globals.allocations.lock().map_err(|_| CUerror::UNKNOWN)?;
            let (_, alloc) = allocations
                .get_offset_and_info(ptr.0 as usize)
                .ok_or(CUerror::INVALID_VALUE)?;
            unsafe { *(data.cast()) = alloc.context };
            Ok(())
        }
        hipPointer_attribute::HIP_POINTER_ATTRIBUTE_MEMORY_TYPE => {
            let mut memory_type = hipMemoryType(0);
            hipPointerGetAttribute(ptr::from_mut(&mut memory_type).cast(), attribute, ptr)?;
            unsafe { *(data.cast()) = to_cu_memory_type(memory_type)? };
            Ok(())
        }
        _ => {
            hipPointerGetAttribute(data, attribute, ptr)?;
            Ok(())
        }
    }
}

pub(crate) unsafe fn get_attributes(
    num_attributes: ::core::ffi::c_uint,
    attributes: &mut hipPointer_attribute,
    data: &mut *mut ::core::ffi::c_void,
    ptr: hipDeviceptr_t,
) -> CUresult {
    hipDrvPointerGetAttributes(num_attributes, attributes, data, ptr)?;
    let attributes = std::slice::from_raw_parts_mut(attributes, num_attributes as usize);
    let data = std::slice::from_raw_parts_mut(data, num_attributes as usize);
    for (attr, data_ptr) in attributes.iter().copied().zip(data.iter().copied()) {
        match attr {
            hipPointer_attribute::HIP_POINTER_ATTRIBUTE_CONTEXT => {
                get_attribute(data_ptr, attr, ptr).ok();
            }
            hipPointer_attribute::HIP_POINTER_ATTRIBUTE_MEMORY_TYPE => {
                *(data_ptr.cast::<CUmemorytype>()) =
                    to_cu_memory_type(*data_ptr.cast::<hipMemoryType>())?;
            }
            _ => {}
        }
    }
    Ok(())
}
