use cuda_types::cuda::*;
use hip_runtime_sys::*;
use std::{ffi::c_void, ptr};

use crate::r#impl::{driver, hipfix};

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
            let context = get_context(ptr)?;
            unsafe { *(data.cast()) = context };
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

fn get_context(ptr: hipDeviceptr_t) -> Result<CUcontext, CUerror> {
    let globals = driver::global_state()?;
    let allocations = globals.allocations.lock().map_err(|_| CUerror::UNKNOWN)?;
    let (_, alloc) = allocations
        .get_offset_and_info(ptr.0 as usize)
        .ok_or(CUerror::INVALID_VALUE)?;
    Ok(alloc.context)
}

pub(crate) unsafe fn get_attributes(
    num_attributes: ::core::ffi::c_uint,
    attributes: &mut hipPointer_attribute,
    data: &mut *mut ::core::ffi::c_void,
    ptr: hipDeviceptr_t,
) -> CUresult {
    hipDrvPointerGetAttributes(
        num_attributes,
        attributes,
        data,
        hipfix::get_attributes(ptr),
    )?;
    let attributes = std::slice::from_raw_parts_mut(attributes, num_attributes as usize);
    let data = std::slice::from_raw_parts_mut(data, num_attributes as usize);
    for (attr, data_ptr) in attributes.iter().copied().zip(data.iter().copied()) {
        match attr {
            hipPointer_attribute::HIP_POINTER_ATTRIBUTE_HOST_POINTER => {
                if (*(data_ptr.cast::<hipDeviceptr_t>())).0.is_null() {
                    *(data_ptr.cast::<hipDeviceptr_t>()) = ptr;
                }
            }
            hipPointer_attribute::HIP_POINTER_ATTRIBUTE_CONTEXT => {
                *(data_ptr.cast::<CUcontext>()) =
                    get_context(ptr).unwrap_or(CUcontext(ptr::null_mut()));
            }
            hipPointer_attribute::HIP_POINTER_ATTRIBUTE_MEMORY_TYPE => {
                *(data_ptr.cast::<CUmemorytype>()) =
                    to_cu_memory_type(*data_ptr.cast::<hipMemoryType>()).unwrap_or(CUmemorytype(0));
            }
            _ => {}
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::tests::CudaApi;
    use cuda_macros::test_cuda;
    use cuda_types::cuda::*;
    use std::{ffi::c_void, i32, mem, ptr, usize};

    #[test_cuda]
    pub unsafe fn unknown_ptr_attribute(api: impl CudaApi) {
        api.cuInit(0);
        api.cuCtxCreate_v2(&mut mem::zeroed(), 0, 0);
        let mut ctx = mem::zeroed::<CUcontext>();
        assert_eq!(
            CUresult::ERROR_INVALID_VALUE,
            api.cuPointerGetAttribute_unchecked(
                std::ptr::from_mut(&mut ctx).cast(),
                CUpointer_attribute::CU_POINTER_ATTRIBUTE_CONTEXT,
                CUdeviceptr_v2(0xDEAD as _)
            )
        );
    }

    #[test_cuda]
    pub unsafe fn unknown_ptr_attributes(api: impl CudaApi) {
        api.cuInit(0);
        api.cuCtxCreate_v2(&mut mem::zeroed(), 0, 0);
        let mut mem_type = mem::zeroed::<CUmemorytype>();
        let mut host_ptr = mem::zeroed::<*mut c_void>();
        let mut dev_ptr = mem::zeroed::<*mut c_void>();
        let mut is_managed = mem::zeroed::<bool>();
        let mut ordinal = mem::zeroed::<i32>();
        let mut attrs = [
            CUpointer_attribute::CU_POINTER_ATTRIBUTE_MEMORY_TYPE,
            CUpointer_attribute::CU_POINTER_ATTRIBUTE_DEVICE_POINTER,
            CUpointer_attribute::CU_POINTER_ATTRIBUTE_HOST_POINTER,
            CUpointer_attribute::CU_POINTER_ATTRIBUTE_IS_MANAGED,
            CUpointer_attribute::CU_POINTER_ATTRIBUTE_DEVICE_ORDINAL,
        ];
        let mut values = [
            std::ptr::from_mut(&mut mem_type).cast::<c_void>(),
            std::ptr::from_mut(&mut dev_ptr).cast(),
            std::ptr::from_mut(&mut host_ptr).cast(),
            std::ptr::from_mut(&mut is_managed).cast(),
            std::ptr::from_mut(&mut ordinal).cast(),
        ];
        assert_eq!(
            CUresult::SUCCESS,
            api.cuPointerGetAttributes_unchecked(
                attrs.len() as u32,
                attrs.as_mut_ptr(),
                values.as_mut_ptr(),
                CUdeviceptr_v2(0xDEAD as _)
            )
        );
        assert_eq!(mem_type, CUmemorytype(0));
        assert_eq!(host_ptr, 0xDEAD as _);
        assert_eq!(dev_ptr, ptr::null_mut());
        assert_eq!(is_managed, false);
        assert_eq!(ordinal, -2);
    }

    #[test_cuda]
    pub unsafe fn unknown_ptr_attributes_no_context(api: impl CudaApi) {
        api.cuInit(0);
        api.cuCtxCreate_v2(&mut mem::zeroed(), 0, 0);
        let mut context = CUcontext(1 as _);
        let mut attrs = [CUpointer_attribute::CU_POINTER_ATTRIBUTE_CONTEXT];
        let mut values = [std::ptr::from_mut(&mut context).cast::<c_void>()];
        assert_eq!(
            CUresult::SUCCESS,
            api.cuPointerGetAttributes_unchecked(
                attrs.len() as u32,
                attrs.as_mut_ptr(),
                values.as_mut_ptr(),
                CUdeviceptr_v2(0xDEAD as _)
            )
        );
        assert_eq!(context, CUcontext(ptr::null_mut()));
    }

    #[test_cuda]
    pub unsafe fn null_ptr_attributes_success(api: impl CudaApi) {
        api.cuInit(0);
        api.cuCtxCreate_v2(&mut mem::zeroed(), 0, 0);
        let mut context = CUcontext(1 as _);
        let mut mem_type = mem::transmute::<_, CUmemorytype>(u32::MAX);
        let mut dev_ptr = mem::transmute::<_, *mut c_void>(usize::MAX);
        let mut host_ptr = mem::transmute::<_, *mut c_void>(usize::MAX);
        let mut is_managed = true;
        let mut ordinal = i32::MAX;
        let mut attrs = [
            CUpointer_attribute::CU_POINTER_ATTRIBUTE_CONTEXT,
            CUpointer_attribute::CU_POINTER_ATTRIBUTE_MEMORY_TYPE,
            CUpointer_attribute::CU_POINTER_ATTRIBUTE_DEVICE_POINTER,
            CUpointer_attribute::CU_POINTER_ATTRIBUTE_HOST_POINTER,
            CUpointer_attribute::CU_POINTER_ATTRIBUTE_IS_MANAGED,
            CUpointer_attribute::CU_POINTER_ATTRIBUTE_DEVICE_ORDINAL,
        ];
        let mut values = [
            std::ptr::from_mut(&mut context).cast::<c_void>(),
            std::ptr::from_mut(&mut mem_type).cast::<c_void>(),
            std::ptr::from_mut(&mut dev_ptr).cast::<c_void>(),
            std::ptr::from_mut(&mut host_ptr).cast::<c_void>(),
            std::ptr::from_mut(&mut is_managed).cast::<c_void>(),
            std::ptr::from_mut(&mut ordinal).cast::<c_void>(),
        ];
        assert_eq!(
            CUresult::SUCCESS,
            api.cuPointerGetAttributes_unchecked(
                attrs.len() as u32,
                attrs.as_mut_ptr(),
                values.as_mut_ptr(),
                CUdeviceptr_v2(ptr::null_mut())
            )
        );
        assert_eq!(context, CUcontext(ptr::null_mut()));
        assert_eq!(mem_type, CUmemorytype(0));
        assert_eq!(dev_ptr, ptr::null_mut());
        assert_eq!(host_ptr, ptr::null_mut());
        assert_eq!(is_managed, false);
        assert_eq!(ordinal, -2);
    }
}
