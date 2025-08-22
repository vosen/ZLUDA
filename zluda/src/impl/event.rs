use cuda_types::cuda::CUresult;
use hip_runtime_sys::*;

pub(crate) unsafe fn create(event: *mut hipEvent_t, flags: ::core::ffi::c_uint) -> CUresult {
    // Flag values are compatible between CUDA and HIP for 0,1,2,4
    hipEventCreateWithFlags(event, flags)?;
    Ok(())
}

pub(crate) unsafe fn query(event: hipEvent_t) -> CUresult {
    hipEventQuery(event)?;
    Ok(())
}

pub(crate) unsafe fn destroy_v2(event: hipEvent_t) -> CUresult {
    hipEventDestroy(event)?;
    Ok(())
}

pub(crate) unsafe fn record(event: hipEvent_t, stream: hipStream_t) -> CUresult {
    hipEventRecord(event, stream)?;
    Ok(())
}

pub(crate) unsafe fn synchronize(event: hipEvent_t) -> CUresult {
    hipEventSynchronize(event)?;
    Ok(())
}
