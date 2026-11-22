use hip_runtime_sys::*;

pub(crate) fn synchronize(stream: hipStream_t) -> hipError_t {
    unsafe { hipStreamSynchronize(stream) }
}

pub(crate) fn create(stream: *mut hipStream_t, flags: ::core::ffi::c_uint) -> hipError_t {
    unsafe { hipStreamCreateWithFlags(stream, flags) }
}

pub(crate) fn create_with_priority(
    stream: *mut hipStream_t,
    flags: ::core::ffi::c_uint,
    priority: ::core::ffi::c_int,
) -> hipError_t {
    unsafe { hipStreamCreateWithPriority(stream, flags, priority) }
}

pub(crate) fn destroy_v2(stream: hipStream_t) -> hipError_t {
    unsafe { hipStreamDestroy(stream) }
}

pub(crate) fn begin_capture_v2(stream: hipStream_t, mode: hipStreamCaptureMode) -> hipError_t {
    unsafe { hipStreamBeginCapture(stream, mode) }
}

pub(crate) fn end_capture(stream: hipStream_t, graph: *mut hipGraph_t) -> hipError_t {
    unsafe { hipStreamEndCapture(stream, graph) }
}

pub(crate) fn is_capturing(
    stream: hipStream_t,
    capture_status: *mut hipStreamCaptureStatus,
) -> hipError_t {
    unsafe { hipStreamIsCapturing(stream, capture_status) }
}

pub(crate) fn get_capture_info_v2(
    stream: hipStream_t,
    capture_status: *mut hipStreamCaptureStatus,
    id: *mut ::core::ffi::c_ulonglong,
    graph_out: *mut hipGraph_t,
    dependencies_out: *mut *const hipGraphNode_t,
    num_dependencies_out: *mut usize,
) -> hipError_t {
    unsafe {
        hipStreamGetCaptureInfo_v2(
            stream,
            capture_status,
            id,
            graph_out,
            dependencies_out,
            num_dependencies_out,
        )
    }
}

pub(crate) fn get_capture_info_v3(
    stream: hipStream_t,
    capture_status_out: *mut hipStreamCaptureStatus,
    id_out: *mut ::core::ffi::c_ulonglong,
    graph_out: *mut hipGraph_t,
    dependencies_out: *mut *const hipGraphNode_t,
    edge_data_out: *mut *const cuda_types::cuda::CUgraphEdgeData,
    num_dependencies_out: *mut usize,
) -> hipError_t {
    if !edge_data_out.is_null() {
        return hipError_t::ErrorNotSupported;
    }
    unsafe {
        hipStreamGetCaptureInfo_v2(
            stream,
            capture_status_out,
            id_out,
            graph_out,
            dependencies_out,
            num_dependencies_out,
        )
    }
}

pub(crate) fn wait_event(
    stream: hipStream_t,
    event: hipEvent_t,
    flags: ::core::ffi::c_uint,
) -> hipError_t {
    unsafe { hipStreamWaitEvent(stream, event, flags) }
}

pub(crate) fn query(stream: hipStream_t) -> hipError_t {
    unsafe { hipStreamQuery(stream) }
}

// Keep CUDA's callback ABI and original CUDA stream handle across the HIP callback.
struct CudaStreamCallbackPayload {
    callback: unsafe extern "system" fn(
        cuda_types::cuda::CUstream,
        cuda_types::cuda::CUresult,
        *mut ::core::ffi::c_void,
    ),
    cuda_stream: cuda_types::cuda::CUstream,
    user_data: *mut ::core::ffi::c_void,
}

unsafe extern "C" fn hip_stream_callback_trampoline(
    _hip_stream: hipStream_t,
    status: hipError_t,
    user_data: *mut ::core::ffi::c_void,
) {
    let payload = unsafe { Box::from_raw(user_data.cast::<CudaStreamCallbackPayload>()) };
    let cuda_status: cuda_types::cuda::CUresult =
        status.map_err(|_| cuda_types::cuda::CUerror::UNKNOWN);
    unsafe { (payload.callback)(payload.cuda_stream, cuda_status, payload.user_data) };
}

pub(crate) unsafe fn add_callback(
    cuda_stream: cuda_types::cuda::CUstream,
    callback: cuda_types::cuda::CUstreamCallback,
    user_data: *mut ::core::ffi::c_void,
    flags: ::core::ffi::c_uint,
) -> cuda_types::cuda::CUresult {
    use cuda_types::cuda::CUerror;
    if flags != 0 {
        return Err(CUerror::INVALID_VALUE);
    }
    let callback = callback.ok_or(CUerror::INVALID_VALUE)?;
    let stream: hipStream_t =
        zluda_common::FromCuda::<_, CUerror>::from_cuda(&cuda_stream)?;

    let payload = Box::new(CudaStreamCallbackPayload {
        callback,
        cuda_stream,
        user_data,
    });
    let raw = Box::into_raw(payload);
    match unsafe {
        hipStreamAddCallback(
            stream,
            Some(hip_stream_callback_trampoline),
            raw.cast::<::core::ffi::c_void>(),
            flags,
        )
    } {
        Ok(()) => Ok(()),
        Err(error) => {
            unsafe { drop(Box::from_raw(raw)) };
            Err(CUerror::from(error))
        }
    }
}
