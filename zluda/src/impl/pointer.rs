use std::{
    ffi::{c_uint, c_ulonglong, c_void},
    mem, ptr,
};

use cuda_types::*;
use hip_runtime_sys::{
    hipDeviceptr_t, hipError_t, hipMemGetAddressRange, hipMemoryType, hipPointerGetAttributes,
    hipPointer_attribute,
};

use crate::{hip_call_cuda, r#impl::IntoCuda};

pub(crate) unsafe fn get_attribute(
    data: *mut c_void,
    attribute: hipPointer_attribute,
    ptr: hipDeviceptr_t,
) -> Result<(), CUresult> {
    if data == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    let mut attribs = mem::zeroed();
    hip_call_cuda! { hipPointerGetAttributes(&mut attribs, ptr.0 as _) };
    // TODO: implement HIP_POINTER_ATTRIBUTE_CONTEXT
    match attribute {
        hipPointer_attribute::HIP_POINTER_ATTRIBUTE_MEMORY_TYPE => {
            *(data as *mut _) =
                memory_type(attribs.__bindgen_anon_1.memoryType).map_err(IntoCuda::into_cuda)?;
            Ok(())
        }
        hipPointer_attribute::HIP_POINTER_ATTRIBUTE_DEVICE_POINTER => {
            *(data as *mut _) = attribs.devicePointer;
            Ok(())
        }
        hipPointer_attribute::HIP_POINTER_ATTRIBUTE_HOST_POINTER => {
            *(data as *mut _) = attribs.hostPointer;
            Ok(())
        }
        hipPointer_attribute::HIP_POINTER_ATTRIBUTE_IS_MANAGED => {
            *(data as *mut _) = attribs.isManaged;
            Ok(())
        }
        hipPointer_attribute::HIP_POINTER_ATTRIBUTE_RANGE_START_ADDR => {
            let mut start = hipDeviceptr_t(ptr::null_mut());
            let mut size = 0usize;
            hip_call_cuda!(hipMemGetAddressRange(&mut start, &mut size, ptr));
            *(data as *mut _) = start;
            Ok(())
        }
        hipPointer_attribute::HIP_POINTER_ATTRIBUTE_RANGE_SIZE => {
            let mut start = hipDeviceptr_t(ptr::null_mut());
            let mut size = 0usize;
            hip_call_cuda!(hipMemGetAddressRange(&mut start, &mut size, ptr));
            *(data as *mut _) = size;
            Ok(())
        }
        hipPointer_attribute::HIP_POINTER_ATTRIBUTE_DEVICE_ORDINAL => {
            *(data as *mut _) = attribs.device;
            Ok(())
        }
        _ => Err(CUresult::CUDA_ERROR_NOT_SUPPORTED),
    }
}

fn memory_type(cu: hipMemoryType) -> Result<CUmemorytype, hipError_t> {
    match cu {
        hipMemoryType::hipMemoryTypeHost => Ok(CUmemorytype::CU_MEMORYTYPE_HOST),
        hipMemoryType::hipMemoryTypeDevice => Ok(CUmemorytype::CU_MEMORYTYPE_DEVICE),
        hipMemoryType::hipMemoryTypeArray => Ok(CUmemorytype::CU_MEMORYTYPE_ARRAY),
        hipMemoryType::hipMemoryTypeUnified => Ok(CUmemorytype::CU_MEMORYTYPE_UNIFIED),
        _ => Err(hipError_t::hipErrorInvalidValue),
    }
}

// "Unlike cuPointerGetAttribute, this function will not return an error when the ptr encountered is not a valid CUDA pointer.
//  Instead, the attributes are assigned default NULL values and CUDA_SUCCESS is returned. "
// TODO: remove once hipDrvPointerGetAttributes works
pub(crate) unsafe fn get_attributes(
    num_attributes: u32,
    attributes: *mut hipPointer_attribute,
    data: *mut *mut c_void,
    ptr: hipDeviceptr_t,
) -> hipError_t {
    if attributes == ptr::null_mut() {
        return hipError_t::hipErrorInvalidValue;
    }
    for i in 0..num_attributes as usize {
        let result = *data.add(i);
        let attrib = *attributes.add(i);
        if get_attribute(result, attrib, ptr).is_err() {
            if let Some(result_size) = result_size(attrib) {
                ptr::write_bytes(result.cast::<u8>(), 0, result_size);
            } else {
                return hipError_t::hipErrorNotSupported;
            }
        };
    }
    hipError_t::hipSuccess
}

#[repr(C)]
#[allow(non_camel_case_types)]
struct CUDA_POINTER_ATTRIBUTE_P2P_TOKENS {
    p2p_token: c_ulonglong,
    va_space_token: c_uint,
}

fn result_size(attrib: hipPointer_attribute) -> Option<usize> {
    Some(match attrib {
        hipPointer_attribute::HIP_POINTER_ATTRIBUTE_CONTEXT => mem::size_of::<CUcontext>(),
        hipPointer_attribute::HIP_POINTER_ATTRIBUTE_MEMORY_TYPE => mem::size_of::<CUmemorytype>(),
        hipPointer_attribute::HIP_POINTER_ATTRIBUTE_DEVICE_POINTER => mem::size_of::<CUdeviceptr>(),
        hipPointer_attribute::HIP_POINTER_ATTRIBUTE_HOST_POINTER => mem::size_of::<*mut c_void>(),
        hipPointer_attribute::HIP_POINTER_ATTRIBUTE_P2P_TOKENS => {
            mem::size_of::<CUDA_POINTER_ATTRIBUTE_P2P_TOKENS>()
        }
        hipPointer_attribute::HIP_POINTER_ATTRIBUTE_SYNC_MEMOPS => mem::size_of::<bool>(),
        hipPointer_attribute::HIP_POINTER_ATTRIBUTE_BUFFER_ID => mem::size_of::<c_ulonglong>(),
        hipPointer_attribute::HIP_POINTER_ATTRIBUTE_IS_MANAGED => mem::size_of::<bool>(),
        hipPointer_attribute::HIP_POINTER_ATTRIBUTE_DEVICE_ORDINAL => mem::size_of::<u32>(),
        hipPointer_attribute::HIP_POINTER_ATTRIBUTE_IS_LEGACY_HIP_IPC_CAPABLE => {
            mem::size_of::<bool>()
        }
        hipPointer_attribute::HIP_POINTER_ATTRIBUTE_RANGE_START_ADDR => {
            mem::size_of::<*mut c_void>()
        }
        hipPointer_attribute::HIP_POINTER_ATTRIBUTE_RANGE_SIZE => mem::size_of::<usize>(),
        hipPointer_attribute::HIP_POINTER_ATTRIBUTE_MAPPED => mem::size_of::<bool>(),
        hipPointer_attribute::HIP_POINTER_ATTRIBUTE_ALLOWED_HANDLE_TYPES => {
            mem::size_of::<CUmemAllocationHandleType>()
        }
        hipPointer_attribute::HIP_POINTER_ATTRIBUTE_IS_GPU_DIRECT_RDMA_CAPABLE => {
            mem::size_of::<bool>()
        }
        // an enum
        hipPointer_attribute::HIP_POINTER_ATTRIBUTE_ACCESS_FLAGS => mem::size_of::<u32>(),
        hipPointer_attribute::HIP_POINTER_ATTRIBUTE_MEMPOOL_HANDLE => {
            mem::size_of::<CUmemoryPool>()
        }
        _ => return None,
    })
}
