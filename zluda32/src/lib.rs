use std::sync::{Mutex, OnceLock};
use compio::fs::named_pipe::NamedPipeServer;
use cuda_macros::cuda_function_declarations;
use cuda_types::cuda::*;
use paste::paste;

mod ipc;

macro_rules! unimplemented {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes)]
            #[allow(improper_ctypes_definitions)]
            #[allow(unused_variables)]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                Err(cuda_types::cuda::CUerror::NOT_SUPPORTED)
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
                use crate::*;
                paste! { [< $fn_name:snake:lower >]($($arg_id),*)? };
                Ok(())
            }
        )*
    };
}

cuda_function_declarations! {
    unimplemented,
    implemented <= [
        cuCtxCreate_v2,
        cuCtxDetach,
        cuCtxGetApiVersion,
        cuCtxGetCurrent,
        cuCtxGetDevice,
        cuCtxSynchronize,
        cuDeviceComputeCapability,
        cuDeviceGet,
        cuDeviceGetAttribute,
        cuDeviceGetCount,
        cuDeviceGetName,
        cuDeviceGetProperties,
        cuDeviceTotalMem_v2,
        cuDriverGetVersion,
        cuEventCreate,
        cuEventDestroy_v2,
        cuGetExportTable,
        cuInit,
        cuLaunchKernel,
        cuMemAlloc_v2,
        cuMemFreeHost,
        cuMemFree_v2,
        cuMemGetAddressRange_v2,
        cuMemHostAlloc,
        cuMemcpyDtoDAsync_v2,
        cuMemcpyDtoHAsync_v2,
        cuMemcpyHtoDAsync_v2,
        cuMemsetD8_v2,
        cuModuleGetFunction,
        cuModuleGetGlobal_v2,
        cuModuleGetTexRef,
        cuStreamCreate,
        cuStreamDestroy_v2,
        cuTexRefSetAddressMode,
        cuTexRefSetAddress_v2,
        cuTexRefSetFilterMode,
        cuTexRefSetFlags,
        cuTexRefSetFormat,
        cuTexRefSetMaxAnisotropy,
        cuTexRefSetMipmapFilterMode,
        cuTexRefSetMipmapLevelBias,
        cuTexRefSetMipmapLevelClamp,
    ]
}

pub fn cu_init(flags: u32) -> Result<(), CUerror> {
    let server = ipc::Server::get()?;
    server.cu_init(flags)
}

pub fn cu_ctx_create_v2(
    pctx: *mut CUcontext,
    flags: ::core::ffi::c_uint,
    dev: CUdevice,
) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_ctx_detach(ctx: CUcontext) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_ctx_get_api_version(
    ctx: CUcontext,
    version: *mut ::core::ffi::c_uint,
) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_ctx_get_current(pctx: *mut CUcontext) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_ctx_get_device(device: *mut CUdevice) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_ctx_synchronize() -> Result<(), CUerror> {
    todo!()
}

pub fn cu_device_compute_capability(
    major: *mut ::core::ffi::c_int,
    minor: *mut ::core::ffi::c_int,
    dev: CUdevice,
) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_device_get(device: *mut CUdevice, ordinal: ::core::ffi::c_int) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_device_get_attribute(
    pi: *mut ::core::ffi::c_int,
    attrib: CUdevice_attribute,
    dev: CUdevice,
) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_device_get_count(count: *mut ::core::ffi::c_int) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_device_get_name(
    name: *mut ::core::ffi::c_char,
    len: ::core::ffi::c_int,
    dev: CUdevice,
) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_device_get_properties(prop: *mut CUdevprop, dev: CUdevice) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_device_total_mem_v2(bytes: *mut usize, dev: CUdevice) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_driver_get_version(driverVersion: *mut ::core::ffi::c_int) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_event_create(phEvent: *mut CUevent, Flags: ::core::ffi::c_uint) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_event_destroy_v2(hEvent: CUevent) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_get_export_table(
    ppExportTable: *mut *const ::core::ffi::c_void,
    pExportTableId: *const CUuuid,
) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_launch_kernel(
    f: CUfunction,
    gridDimX: ::core::ffi::c_uint,
    gridDimY: ::core::ffi::c_uint,
    gridDimZ: ::core::ffi::c_uint,
    blockDimX: ::core::ffi::c_uint,
    blockDimY: ::core::ffi::c_uint,
    blockDimZ: ::core::ffi::c_uint,
    sharedMemBytes: ::core::ffi::c_uint,
    hStream: CUstream,
    kernelParams: *mut *mut ::core::ffi::c_void,
    extra: *mut *mut ::core::ffi::c_void,
) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_mem_alloc_v2(dptr: *mut CUdeviceptr, bytesize: usize) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_mem_free_host(p: *mut ::core::ffi::c_void) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_mem_free_v2(dptr: CUdeviceptr) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_mem_get_address_range_v2(
    pbase: *mut CUdeviceptr,
    psize: *mut usize,
    dptr: CUdeviceptr,
) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_mem_host_alloc(
    pp: *mut *mut ::core::ffi::c_void,
    bytesize: usize,
    Flags: ::core::ffi::c_uint,
) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_memcpy_dto_d_async_v2(
    dstDevice: CUdeviceptr,
    srcDevice: CUdeviceptr,
    ByteCount: usize,
    hStream: CUstream,
) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_memcpy_dto_h_async_v2(
    dstHost: *mut ::core::ffi::c_void,
    srcDevice: CUdeviceptr,
    ByteCount: usize,
    hStream: CUstream,
) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_memcpy_hto_d_async_v2(
    dstDevice: CUdeviceptr,
    srcHost: *const ::core::ffi::c_void,
    ByteCount: usize,
    hStream: CUstream,
) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_memset_d8_v2(
    dstDevice: CUdeviceptr,
    uc: ::core::ffi::c_uchar,
    N: usize,
) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_module_get_function(
    hfunc: *mut CUfunction,
    hmod: CUmodule,
    name: *const ::core::ffi::c_char,
) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_module_get_global_v2(
    dptr: *mut CUdeviceptr,
    bytes: *mut usize,
    hmod: CUmodule,
    name: *const ::core::ffi::c_char,
) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_module_get_tex_ref(
    pTexRef: *mut CUtexref,
    hmod: CUmodule,
    name: *const ::core::ffi::c_char,
) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_stream_create(
    phStream: *mut CUstream,
    Flags: ::core::ffi::c_uint,
) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_stream_destroy_v2(hStream: CUstream) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_tex_ref_set_address_mode(
    hTexRef: CUtexref,
    dim: ::core::ffi::c_int,
    am: CUaddress_mode,
) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_tex_ref_set_address_v2(
    ByteOffset: *mut usize,
    hTexRef: CUtexref,
    dptr: CUdeviceptr,
    bytes: usize,
) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_tex_ref_set_filter_mode(hTexRef: CUtexref, fm: CUfilter_mode) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_tex_ref_set_flags(hTexRef: CUtexref, Flags: ::core::ffi::c_uint) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_tex_ref_set_format(
    hTexRef: CUtexref,
    fmt: CUarray_format,
    NumPackedComponents: ::core::ffi::c_int,
) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_tex_ref_set_max_anisotropy(
    hTexRef: CUtexref,
    maxAniso: ::core::ffi::c_uint,
) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_tex_ref_set_mipmap_filter_mode(
    hTexRef: CUtexref,
    fm: CUfilter_mode,
) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_tex_ref_set_mipmap_level_bias(hTexRef: CUtexref, bias: f32) -> Result<(), CUerror> {
    todo!()
}

pub fn cu_tex_ref_set_mipmap_level_clamp(
    hTexRef: CUtexref,
    minMipmapLevelClamp: f32,
    maxMipmapLevelClamp: f32,
) -> Result<(), CUerror> {
    todo!()
}
