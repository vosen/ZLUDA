use cuda_types::cuda::CUerror;
use std::sync::atomic::{AtomicBool, Ordering};

pub(crate) mod r#impl;
#[cfg_attr(windows, path = "os_win.rs")]
#[cfg_attr(not(windows), path = "os_unix.rs")]
mod os;

static INITIALIZED: AtomicBool = AtomicBool::new(true);
pub(crate) fn initialized() -> bool {
    INITIALIZED.load(Ordering::SeqCst)
}

#[cfg_attr(not(windows), dtor::dtor)]
fn deinitialize() {
    INITIALIZED.store(false, Ordering::SeqCst);
}

macro_rules! unimplemented {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes)]
            #[allow(improper_ctypes_definitions)]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                crate::r#impl::unimplemented()
            }
        )*
    };
}

macro_rules! implemented {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes)]
            #[allow(improper_ctypes_definitions)]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                if !initialized() {
                    return Err(CUerror::DEINITIALIZED);
                }
                cuda_macros::cuda_normalize_fn!( crate::r#impl::$fn_name ) ($(zluda_common::FromCuda::<_, CUerror>::from_cuda(&$arg_id)?),*)?;
                Ok(())
            }
        )*
    };
}

macro_rules! implemented_in_function {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes)]
            #[allow(improper_ctypes_definitions)]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                if !initialized() {
                    return Err(CUerror::DEINITIALIZED);
                }
                cuda_macros::cuda_normalize_fn!( crate::r#impl::function::$fn_name ) ($(zluda_common::FromCuda::<_, CUerror>::from_cuda(&$arg_id)?),*)?;
                Ok(())
            }
        )*
    };
}

cuda_macros::cuda_function_declarations!(
    unimplemented,
    implemented
        <= [
            cuCtxCreate_v2,
            cuCtxDestroy_v2,
            cuCtxGetApiVersion,
            cuCtxGetCurrent,
            cuCtxGetDevice,
            cuCtxGetLimit,
            cuCtxGetStreamPriorityRange,
            cuCtxPopCurrent,
            cuCtxPopCurrent_v2,
            cuCtxPushCurrent,
            cuCtxPushCurrent_v2,
            cuCtxSetCurrent,
            cuCtxSetFlags,
            cuCtxSetLimit,
            cuCtxSynchronize,
            cuDeviceComputeCapability,
            cuDeviceGet,
            cuDeviceGetAttribute,
            cuDeviceGetCount,
            cuDeviceGetLuid,
            cuDeviceGetName,
            cuDeviceGetProperties,
            cuDeviceGetUuid,
            cuDeviceGetUuid_v2,
            cuDevicePrimaryCtxGetState,
            cuDevicePrimaryCtxRelease,
            cuDevicePrimaryCtxReset,
            cuDevicePrimaryCtxRetain,
            cuDeviceTotalMem_v2,
            cuDriverGetVersion,
            cuEventCreate,
            cuEventDestroy_v2,
            cuEventQuery,
            cuEventRecord,
            cuEventSynchronize,
            cuFuncGetAttribute,
            // cuFuncSetAttribute,
            cuGetExportTable,
            cuGetProcAddress,
            cuGetProcAddress_v2,
            // cuGraphDestroy,
            // cuGraphExecDestroy,
            // cuGraphGetNodes,
            // cuGraphInstantiateWithFlags,
            // cuGraphLaunch,
            cuInit,
            // cuKernelGetFunction,
            // cuKernelSetAttribute,
            // cuLaunchKernel,
            // cuLaunchKernelEx,
            // cuLibraryGetGlobal,
            // cuLibraryGetKernel,
            cuLibraryGetModule,
            cuLibraryLoadData,
            cuLibraryUnload,
            cuMemAlloc_v2,
            cuMemFreeHost,
            cuMemFree_v2,
            cuMemGetAddressRange_v2,
            cuMemGetInfo_v2,
            cuMemHostAlloc,
            //cuMemRetainAllocationHandle,
            //cuMemcpyAsync,
            //cuMemcpyDtoDAsync_v2,
            //cuMemcpyDtoHAsync_v2,
            cuMemcpyDtoH_v2,
            // cuMemcpyHtoDAsync_v2,
            cuMemcpyHtoD_v2,
            cuMemsetD32_v2,
            // cuMemsetD8Async,
            cuMemsetD8_v2,
            cuModuleGetFunction,
            cuModuleGetGlobal_v2,
            cuModuleGetLoadingMode,
            cuModuleLoadData,
            cuModuleUnload,
            // cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags,
            cuPointerGetAttribute,
            cuPointerGetAttributes,
            cuProfilerStart,
            cuProfilerStop,
            cuStreamBeginCapture_v2,
            cuStreamCreateWithPriority,
            cuStreamDestroy_v2,
            cuStreamEndCapture,
            cuStreamGetCaptureInfo_v2,
            cuStreamIsCapturing,
            cuStreamSynchronize,
            cuStreamWaitEvent,
            cuThreadExchangeStreamCaptureMode,
        ],
    implemented_in_function <= [cuLaunchKernel,]
);
