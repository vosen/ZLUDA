use hip_runtime_sys::{
    hipDrvMemcpy3D, hipError_t, hipMemcpy3D, hipMemcpy3DParms, hipMemoryType, hipPitchedPtr,
    hipPos, HIP_MEMCPY3D,
};
use std::ptr;

use crate::{
    cuda::{CUDA_MEMCPY3D_st, CUdeviceptr, CUmemorytype, CUresult},
    hip_call,
};

// TODO change HIP impl to 64 bits
pub(crate) unsafe fn copy_3d(cu_copy: *const CUDA_MEMCPY3D_st) -> Result<(), hipError_t> {
    if cu_copy == ptr::null() {
        return Err(hipError_t::hipErrorInvalidValue);
    }
    let cu_copy = *cu_copy;
    let hip_copy = HIP_MEMCPY3D {
        srcXInBytes: cu_copy.srcXInBytes as u32,
        srcY: cu_copy.srcY as u32,
        srcZ: cu_copy.srcZ as u32,
        srcLOD: cu_copy.srcLOD as u32,
        srcMemoryType: memory_type(cu_copy.srcMemoryType)?,
        srcHost: cu_copy.srcHost,
        srcDevice: cu_copy.srcDevice.0 as _,
        srcArray: cu_copy.srcArray as _,
        srcPitch: cu_copy.srcPitch as u32,
        srcHeight: cu_copy.srcHeight as u32,
        dstXInBytes: cu_copy.dstXInBytes as u32,
        dstY: cu_copy.dstY as u32,
        dstZ: cu_copy.dstZ as u32,
        dstLOD: cu_copy.dstLOD as u32,
        dstMemoryType: memory_type(cu_copy.dstMemoryType)?,
        dstHost: cu_copy.dstHost,
        dstDevice: cu_copy.dstDevice.0 as _,
        dstArray: cu_copy.dstArray as _,
        dstPitch: cu_copy.dstPitch as u32,
        dstHeight: cu_copy.dstHeight as u32,
        WidthInBytes: cu_copy.WidthInBytes as u32,
        Height: cu_copy.Height as u32,
        Depth: cu_copy.Depth as u32,
    };
    hip_call! { hipDrvMemcpy3D(&hip_copy) };
    Ok(())
}

pub(crate) fn memory_type(cu: CUmemorytype) -> Result<hipMemoryType, hipError_t> {
    match cu {
        CUmemorytype::CU_MEMORYTYPE_HOST => Ok(hipMemoryType::hipMemoryTypeHost),
        CUmemorytype::CU_MEMORYTYPE_DEVICE => Ok(hipMemoryType::hipMemoryTypeDevice),
        CUmemorytype::CU_MEMORYTYPE_ARRAY => Ok(hipMemoryType::hipMemoryTypeArray),
        CUmemorytype::CU_MEMORYTYPE_UNIFIED => Ok(hipMemoryType::hipMemoryTypeUnified),
        _ => Err(hipError_t::hipErrorInvalidValue),
    }
}
