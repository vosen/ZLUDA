use cuda_base::cuda_function_declarations;

use crate::r#impl::{FromCuda, IntoCuda};

macro_rules! unimplemented_cuda_fn {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:path);*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                crate::r#impl::unimplemented()
            }
        )*
    };
}

macro_rules! implemented_cuda_fn {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:path);*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                definitions::$fn_name($(FromCuda::from_cuda($arg_id)),*).into_cuda()
            }
        )*
    };
}

cuda_function_declarations!(
    cuda_types,
    unimplemented_cuda_fn,
    implemented_cuda_fn,
    [
        cuGetErrorString,
        cuInit,
        cuGetProcAddress,
        cuGetProcAddress_v2,
        cuGetExportTable,
        cuDriverGetVersion,
        cuDeviceCanAccessPeer,
        cuDeviceGet,
        cuDeviceGetCount,
        cuDeviceGetMemPool,
        cuDeviceGetName,
        cuDeviceGetUuid,
        cuDeviceGetUuid_v2,
        cuDeviceGetLuid,
        cuDeviceTotalMem,
        cuDeviceTotalMem_v2,
        cuDeviceGetAttribute,
        cuDeviceGetProperties,
        cuDeviceComputeCapability,
        cuDevicePrimaryCtxRetain,
        cuDevicePrimaryCtxRelease,
        cuDevicePrimaryCtxRelease_v2,
        cuDevicePrimaryCtxReset,
        cuDevicePrimaryCtxReset_v2,
        cuDevicePrimaryCtxSetFlags,
        cuDevicePrimaryCtxSetFlags_v2,
        cuDevicePrimaryCtxGetState,
        cuCtxCreate,
        cuCtxCreate_v2,
        cuCtxDestroy,
        cuCtxDestroy_v2,
        cuCtxPushCurrent,
        cuCtxPushCurrent_v2,
        cuCtxPopCurrent,
        cuCtxPopCurrent_v2,
        cuCtxSetCurrent,
        cuCtxGetCurrent,
        cuCtxGetDevice,
        cuCtxGetLimit,
        cuCtxSetLimit,
        cuCtxSetFlags,
        cuCtxGetStreamPriorityRange,
        cuCtxSynchronize,
        cuCtxSetCacheConfig,
        cuCtxGetApiVersion,
        cuFuncSetCacheConfig,
        cuLibraryLoadData,
        cuLibraryGetModule,
        cuLibraryUnload,
        cuModuleLoad,
        cuModuleLoadData,
        cuModuleLoadDataEx,
        cuModuleUnload,
        cuModuleGetFunction,
        cuModuleGetGlobal_v2,
        cuModuleGetLoadingMode,
        cuModuleGetSurfRef,
        cuModuleGetTexRef,
        cuMemGetInfo_v2,
        cuMemAlloc_v2,
        cuMemAllocManaged,
        cuMemAllocPitch_v2,
        cuMemFree_v2,
        cuMemFreeAsync,
        cuMemFreeHost,
        cuMemHostAlloc,
        cuMemHostRegister,
        cuMemHostRegister_v2,
        cuMemHostUnregister,
        cuMemGetAddressRange_v2,
        cuMemPoolSetAttribute,
        cuMemPrefetchAsync,
        cuDeviceGetPCIBusId,
        cuMemcpy,
        cuMemcpy_ptds,
        cuMemcpyAsync,
        cuMemcpyAsync_ptsz,
        cuMemcpyHtoD_v2,
        cuMemcpyHtoD_v2_ptds,
        cuMemcpyDtoH_v2,
        cuMemcpyDtoH_v2_ptds,
        cuMemcpyDtoD_v2,
        cuMemcpyDtoDAsync_v2,
        cuMemcpyDtoDAsync_v2_ptsz,
        cuMemcpyHtoDAsync_v2,
        cuMemcpyHtoDAsync_v2_ptsz,
        cuMemcpyDtoHAsync_v2,
        cuMemcpyDtoHAsync_v2_ptsz,
        cuMemcpy2D_v2,
        cuMemcpy2DAsync_v2,
        cuMemcpy2DUnaligned_v2,
        cuMemcpy3D_v2,
        cuMemcpy3DAsync_v2,
        cuMemsetD8_v2,
        cuMemsetD8_v2_ptds,
        cuMemsetD8Async,
        cuMemsetD8Async_ptsz,
        cuMemsetD16_v2,
        cuMemsetD32Async,
        cuMemsetD32_v2,
        cuMemsetD32_v2_ptds,
        cuMemsetD2D8_v2,
        cuOccupancyMaxPotentialBlockSize,
        cuArrayCreate_v2,
        cuArrayDestroy,
        cuArray3DCreate_v2,
        cuArray3DGetDescriptor_v2,
        cuPointerGetAttribute,
        cuPointerGetAttributes,
        cuStreamCreate,
        cuStreamCreateWithPriority,
        cuStreamGetCaptureInfo,
        cuStreamGetCtx,
        cuStreamGetCtx_ptsz,
        cuStreamGetFlags,
        cuStreamIsCapturing,
        cuStreamQuery,
        cuStreamSynchronize,
        cuStreamSynchronize_ptsz,
        cuStreamDestroy,
        cuStreamDestroy_v2,
        cuStreamWaitEvent,
        cuStreamWaitEvent_ptsz,
        cuFuncGetAttribute,
        cuFuncSetAttribute,
        cuLaunchHostFunc,
        cuLaunchKernel,
        cuLaunchKernel_ptsz,
        cuMemHostGetDevicePointer_v2,
        cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags,
        cuSurfObjectCreate,
        cuSurfObjectDestroy,
        cuTexObjectCreate,
        cuTexObjectDestroy,
        cuTexRefGetAddress_v2,
        cuTexRefGetAddressMode,
        cuTexRefGetFilterMode,
        cuTexRefGetFlags,
        cuTexRefGetMipmapFilterMode,
        cuTexRefGetMipmapLevelBias,
        cuTexRefGetMipmapLevelClamp,
        cuTexRefGetMaxAnisotropy,
        cuTexRefSetAddress2D_v3,
        cuTexRefSetAddressMode,
        cuTexRefSetAddress_v2,
        cuTexRefSetArray,
        cuTexRefSetFilterMode,
        cuTexRefSetFlags,
        cuTexRefSetFormat,
        cuTexRefGetFormat,
        cuTexRefSetMaxAnisotropy,
        cuTexRefSetMipmapFilterMode,
        cuTexRefSetMipmapLevelBias,
        cuTexRefSetMipmapLevelClamp,
        cuSurfRefSetArray,
        cuCtxDetach,
        cuFuncSetBlockShape,
        cuEventCreate,
        cuEventDestroy,
        cuEventDestroy_v2,
        cuEventQuery,
        cuEventElapsedTime,
        cuEventRecord,
        cuEventRecord_ptsz,
        cuEventSynchronize,
        cuGraphAddDependencies,
        cuGraphAddEmptyNode,
        cuGraphAddKernelNode,
        cuGraphCreate,
        cuGraphDestroy,
        cuGraphExecDestroy,
        cuGraphInstantiate,
        cuGraphInstantiate_v2,
        cuGraphLaunch,
        cuGraphicsSubResourceGetMappedArray,
        cuGraphicsGLRegisterBuffer,
        cuGraphicsGLRegisterImage,
        cuGraphicsMapResources,
        cuGraphicsResourceGetMappedPointer_v2,
        cuGraphicsUnmapResources,
        cuGraphicsUnregisterResource,
        cuLinkAddData_v2,
        cuLinkComplete,
        cuLinkDestroy,
        cuLinkCreate_v2,
    ]
);

mod definitions {
    use std::ptr;

    use cuda_types::*;
    use hip_runtime_sys::*;

    use crate::hip_call_cuda;
    use crate::r#impl;
    use crate::r#impl::array;
    use crate::r#impl::context;
    use crate::r#impl::dark_api;
    use crate::r#impl::device;
    use crate::r#impl::function;
    use crate::r#impl::gl;
    use crate::r#impl::graph;
    use crate::r#impl::hipfix;
    use crate::r#impl::library;
    use crate::r#impl::link;
    use crate::r#impl::memcpy2d_from_cuda;
    use crate::r#impl::memory;
    use crate::r#impl::module;
    use crate::r#impl::pointer;
    use crate::r#impl::stream;
    use crate::r#impl::surface;
    use crate::r#impl::surfref;
    use crate::r#impl::texobj;
    use crate::r#impl::texref;

    pub(crate) unsafe fn cuGetErrorString(
        error: hipError_t,
        pStr: *mut *const ::std::os::raw::c_char,
    ) -> CUresult {
        *pStr = hipGetErrorString(error);
        CUresult::CUDA_SUCCESS
    }

    pub(crate) unsafe fn cuInit(Flags: ::std::os::raw::c_uint) -> Result<(), CUresult> {
        r#impl::init(Flags)
    }

    pub(crate) unsafe fn cuGetProcAddress(
        symbol: *const ::std::os::raw::c_char,
        pfn: *mut *mut ::std::os::raw::c_void,
        cudaVersion: ::std::os::raw::c_int,
        flags: cuuint64_t,
    ) -> CUresult {
        cuGetProcAddress_v2(symbol, pfn, cudaVersion, flags, ptr::null_mut())
    }

    pub(crate) fn cuGetProcAddress_v2(
        symbol: *const ::std::os::raw::c_char,
        pfn: *mut *mut ::std::os::raw::c_void,
        cudaVersion: ::std::os::raw::c_int,
        flags: cuuint64_t,
        symbolStatus: *mut CUdriverProcAddressQueryResult,
    ) -> CUresult {
        unsafe { r#impl::get_proc_address_v2(symbol, pfn, cudaVersion, flags, symbolStatus) }
    }

    pub(crate) unsafe fn cuGetExportTable(
        ppExportTable: *mut *const ::std::os::raw::c_void,
        pExportTableId: *const CUuuid,
    ) -> CUresult {
        dark_api::get_table(ppExportTable, pExportTableId)
    }

    pub(crate) unsafe fn cuDriverGetVersion(driverVersion: *mut ::std::os::raw::c_int) -> CUresult {
        *driverVersion = crate::DRIVER_VERSION;
        CUresult::CUDA_SUCCESS
    }

    pub(crate) unsafe fn cuDeviceCanAccessPeer(
        canAccessPeer: *mut ::std::os::raw::c_int,
        dev: hipDevice_t,
        peerDev: hipDevice_t,
    ) -> hipError_t {
        hipDeviceCanAccessPeer(canAccessPeer, dev, peerDev)
    }

    pub(crate) unsafe fn cuDeviceGet(
        device: *mut hipDevice_t,
        ordinal: ::std::os::raw::c_int,
    ) -> hipError_t {
        hipDeviceGet(device as _, ordinal)
    }

    pub(crate) unsafe fn cuDeviceGetCount(count: *mut ::std::os::raw::c_int) -> hipError_t {
        hipGetDeviceCount(count)
    }

    pub(crate) unsafe fn cuDeviceGetMemPool(
        pool: *mut hipMemPool_t,
        dev: hipDevice_t,
    ) -> hipError_t {
        hipDeviceGetMemPool(pool, dev)
    }

    pub(crate) unsafe fn cuDeviceGetName(
        name: *mut ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
        dev: hipDevice_t,
    ) -> hipError_t {
        device::get_name(name, len, dev)
    }

    pub(crate) unsafe fn cuDeviceGetUuid(uuid: *mut CUuuid, dev: hipDevice_t) -> CUresult {
        device::get_uuid(uuid, dev)
    }

    pub(crate) unsafe fn cuDeviceGetUuid_v2(uuid: *mut CUuuid, dev: hipDevice_t) -> CUresult {
        device::get_uuid(uuid, dev)
    }

    pub(crate) unsafe fn cuDeviceGetLuid(
        luid: *mut ::std::os::raw::c_char,
        deviceNodeMask: *mut ::std::os::raw::c_uint,
        dev: hipDevice_t,
    ) -> CUresult {
        device::get_luid(luid, deviceNodeMask, dev)
    }

    pub(crate) unsafe fn cuDeviceTotalMem(
        bytes: *mut u32,
        dev: hipDevice_t,
    ) -> Result<(), hipError_t> {
        device::total_mem(bytes, dev)
    }

    pub(crate) unsafe fn cuDeviceTotalMem_v2(bytes: *mut usize, dev: hipDevice_t) -> hipError_t {
        hipDeviceTotalMem(bytes, dev)
    }

    pub(crate) unsafe fn cuDeviceGetAttribute(
        pi: *mut ::std::os::raw::c_int,
        attrib: CUdevice_attribute,
        dev: hipDevice_t,
    ) -> Result<(), CUresult> {
        device::get_attribute(pi, attrib, dev)
    }

    pub(crate) unsafe fn cuDeviceGetProperties(
        prop: *mut CUdevprop,
        dev: hipDevice_t,
    ) -> Result<(), CUresult> {
        device::get_properties(prop, dev)
    }

    pub(crate) unsafe fn cuDeviceComputeCapability(
        major: *mut ::std::os::raw::c_int,
        minor: *mut ::std::os::raw::c_int,
        dev: hipDevice_t,
    ) {
        device::compute_capability(major, minor, dev)
    }

    pub(crate) unsafe fn cuDevicePrimaryCtxRetain(
        pctx: *mut *mut context::Context,
        dev: hipDevice_t,
    ) -> Result<(), CUresult> {
        device::primary_ctx_retain(pctx, dev)
    }

    pub(crate) unsafe fn cuDevicePrimaryCtxRelease(dev: hipDevice_t) -> Result<(), CUresult> {
        device::primary_ctx_release(dev)
    }

    pub(crate) unsafe fn cuDevicePrimaryCtxRelease_v2(dev: hipDevice_t) -> Result<(), CUresult> {
        device::primary_ctx_release(dev)
    }

    pub(crate) unsafe fn cuDevicePrimaryCtxReset(dev: hipDevice_t) -> Result<(), CUresult> {
        device::primary_ctx_reset(dev)
    }

    pub(crate) unsafe fn cuDevicePrimaryCtxReset_v2(dev: hipDevice_t) -> Result<(), CUresult> {
        device::primary_ctx_reset(dev)
    }

    pub(crate) unsafe fn cuDevicePrimaryCtxSetFlags(
        dev: hipDevice_t,
        flags: ::std::os::raw::c_uint,
    ) -> Result<(), CUresult> {
        device::primary_ctx_set_flags(dev, flags)
    }

    pub(crate) unsafe fn cuDevicePrimaryCtxSetFlags_v2(
        dev: hipDevice_t,
        flags: ::std::os::raw::c_uint,
    ) -> Result<(), CUresult> {
        device::primary_ctx_set_flags(dev, flags)
    }

    pub(crate) unsafe fn cuDevicePrimaryCtxGetState(
        dev: hipDevice_t,
        flags: *mut ::std::os::raw::c_uint,
        active: *mut ::std::os::raw::c_int,
    ) -> Result<(), CUresult> {
        device::primary_ctx_get_state(dev, flags, active)
    }

    pub(crate) unsafe fn cuCtxCreate(
        pctx: *mut *mut context::Context,
        flags: ::std::os::raw::c_uint,
        dev: hipDevice_t,
    ) -> Result<(), CUresult> {
        context::create(pctx, flags, dev)
    }

    pub(crate) unsafe fn cuCtxCreate_v2(
        pctx: *mut *mut context::Context,
        flags: ::std::os::raw::c_uint,
        dev: hipDevice_t,
    ) -> Result<(), CUresult> {
        context::create(pctx, flags, dev)
    }

    pub(crate) unsafe fn cuCtxDestroy(ctx: *mut context::Context) -> Result<(), CUresult> {
        context::destroy(ctx)
    }

    pub(crate) unsafe fn cuCtxDestroy_v2(ctx: *mut context::Context) -> Result<(), CUresult> {
        context::destroy(ctx)
    }

    pub(crate) unsafe fn cuCtxDetach(ctx: *mut context::Context) -> Result<(), CUresult> {
        Ok(())
    }

    pub(crate) unsafe fn cuCtxPushCurrent(ctx: *mut context::Context) -> Result<(), CUresult> {
        context::push_current(ctx)
    }

    pub(crate) unsafe fn cuCtxPushCurrent_v2(ctx: *mut context::Context) -> Result<(), CUresult> {
        context::push_current(ctx)
    }

    pub(crate) unsafe fn cuCtxPopCurrent(pctx: *mut *mut context::Context) -> Result<(), CUresult> {
        context::pop_current(pctx)
    }

    pub(crate) unsafe fn cuCtxPopCurrent_v2(
        pctx: *mut *mut context::Context,
    ) -> Result<(), CUresult> {
        context::pop_current(pctx)
    }

    pub(crate) unsafe fn cuCtxSetCurrent(ctx: *mut context::Context) -> Result<(), CUresult> {
        context::set_current(ctx)
    }

    pub(crate) unsafe fn cuCtxGetCurrent(pctx: *mut *mut context::Context) -> CUresult {
        context::get_current(pctx)
    }

    pub(crate) unsafe fn cuCtxGetDevice(device: *mut hipDevice_t) -> Result<(), CUresult> {
        context::get_device(device)
    }

    pub(crate) unsafe fn cuCtxGetLimit(
        pvalue: *mut usize,
        limit: hipLimit_t,
    ) -> Result<(), CUresult> {
        context::get_limit(pvalue, limit)
    }

    pub(crate) unsafe fn cuCtxSetLimit(limit: hipLimit_t, value: usize) -> Result<(), CUresult> {
        context::set_limit(limit, value)
    }

    pub(crate) unsafe fn cuCtxSetFlags(flags: u32) -> Result<(), CUresult> {
        context::set_flags(flags)
    }

    pub(crate) unsafe fn cuCtxGetStreamPriorityRange(
        leastPriority: *mut ::std::os::raw::c_int,
        greatestPriority: *mut ::std::os::raw::c_int,
    ) -> Result<(), CUresult> {
        context::get_stream_priority_range(leastPriority, greatestPriority)
    }

    pub(crate) unsafe fn cuCtxSynchronize() -> Result<(), CUresult> {
        context::synchronize()
    }

    // TODO
    pub(crate) unsafe fn cuCtxSetCacheConfig(config: CUfunc_cache) -> CUresult {
        CUresult::CUDA_SUCCESS
    }

    pub(crate) unsafe fn cuCtxGetApiVersion(
        ctx: *mut context::Context,
        version: *mut ::std::os::raw::c_uint,
    ) -> Result<(), CUresult> {
        context::get_api_version(ctx, version)
    }

    pub(crate) unsafe fn cuFuncSetCacheConfig(
        hfunc: *mut function::Function,
        config: hipFuncCache_t,
    ) -> CUresult {
        CUresult::CUDA_SUCCESS
    }

    pub(crate) unsafe fn cuLibraryLoadData(
        library: *mut *mut library::Library,
        code: *const ::std::os::raw::c_void,
        jitOptions: *mut CUjit_option,
        jitOptionsValues: *mut *mut ::std::os::raw::c_void,
        numJitOptions: ::std::os::raw::c_uint,
        libraryOptions: *mut CUlibraryOption,
        libraryOptionValues: *mut *mut ::std::os::raw::c_void,
        numLibraryOptions: ::std::os::raw::c_uint,
    ) -> Result<(), CUresult> {
        library::load_data(
            library,
            code,
            jitOptions,
            jitOptionsValues,
            numJitOptions,
            libraryOptions,
            libraryOptionValues,
            numLibraryOptions,
        )
    }

    pub(crate) unsafe fn cuLibraryGetModule(
        pMod: *mut *mut module::Module,
        library: *mut library::Library,
    ) -> Result<(), CUresult> {
        library::get_module(pMod, library)
    }

    pub(crate) unsafe fn cuLibraryUnload(library: *mut library::Library) -> Result<(), CUresult> {
        library::unload(library)
    }

    pub(crate) unsafe fn cuModuleLoad(
        module: *mut *mut module::Module,
        fname: *const ::std::os::raw::c_char,
    ) -> Result<(), CUresult> {
        module::load(module, fname)
    }

    pub(crate) unsafe fn cuModuleLoadData(
        module: *mut *mut module::Module,
        image: *const ::std::os::raw::c_void,
    ) -> Result<(), CUresult> {
        module::load_data(module, image)
    }

    // TODO: parse jit options
    pub(crate) unsafe fn cuModuleLoadDataEx(
        module: *mut *mut module::Module,
        image: *const ::std::os::raw::c_void,
        numOptions: ::std::os::raw::c_uint,
        options: *mut CUjit_option,
        optionValues: *mut *mut ::std::os::raw::c_void,
    ) -> Result<(), CUresult> {
        module::load_data(module, image)
    }

    pub(crate) unsafe fn cuModuleUnload(hmod: *mut module::Module) -> Result<(), CUresult> {
        module::unload(hmod)
    }

    pub(crate) unsafe fn cuModuleGetFunction(
        hfunc: *mut *mut function::Function,
        hmod: *mut module::Module,
        name: *const ::std::os::raw::c_char,
    ) -> Result<(), CUresult> {
        module::get_function(hfunc, hmod, name)
    }

    pub(crate) unsafe fn cuModuleGetGlobal_v2(
        dptr: *mut hipDeviceptr_t,
        bytes: *mut usize,
        hmod: *mut module::Module,
        name: *const ::std::os::raw::c_char,
    ) -> Result<(), CUresult> {
        module::get_global(dptr, bytes, hmod, name)
    }

    pub(crate) unsafe fn cuModuleGetLoadingMode(mode: *mut CUmoduleLoadingMode) -> CUresult {
        module::get_loading_mode(mode)
    }

    pub(crate) unsafe fn cuModuleGetSurfRef(
        pTexRef: *mut *mut textureReference,
        hmod: *mut module::Module,
        name: *const ::std::os::raw::c_char,
    ) -> Result<(), CUresult> {
        module::get_surf_ref(pTexRef, hmod, name)
    }

    pub(crate) unsafe fn cuModuleGetTexRef(
        pTexRef: *mut *mut textureReference,
        hmod: *mut module::Module,
        name: *const ::std::os::raw::c_char,
    ) -> Result<(), CUresult> {
        module::get_tex_ref(pTexRef, hmod, name)
    }

    pub(crate) unsafe fn cuMemGetInfo_v2(free: *mut usize, total: *mut usize) -> hipError_t {
        hipMemGetInfo(free, total)
    }

    pub(crate) unsafe fn cuMemAlloc_v2(
        dptr: *mut hipDeviceptr_t,
        bytesize: usize,
    ) -> Result<(), CUresult> {
        memory::alloc(dptr, bytesize)
    }

    pub(crate) unsafe fn cuMemAllocManaged(
        dev_ptr: *mut hipDeviceptr_t,
        size: usize,
        flags: ::std::os::raw::c_uint,
    ) -> hipError_t {
        hipMallocManaged(dev_ptr.cast(), size, flags)
    }

    pub(crate) unsafe fn cuMemAllocPitch_v2(
        dptr: *mut hipDeviceptr_t,
        ptr_pitch: *mut usize,
        width_in_bytes: usize,
        height: usize,
        _element_size_bytes: ::std::os::raw::c_uint,
    ) -> hipError_t {
        hipMallocPitch(dptr as _, ptr_pitch, width_in_bytes, height)
    }

    pub(crate) unsafe fn cuMemFree_v2(dptr: hipDeviceptr_t) -> hipError_t {
        hipFree(dptr.0)
    }

    pub(crate) unsafe fn cuMemFreeAsync(
        dptr: hipDeviceptr_t,
        hStream: *mut stream::Stream,
    ) -> Result<(), CUresult> {
        memory::free_async(dptr, hStream)
    }

    pub(crate) unsafe fn cuMemFreeHost(p: *mut ::std::os::raw::c_void) -> hipError_t {
        hipFreeHost(p)
    }

    pub(crate) unsafe fn cuMemHostAlloc(
        pp: *mut *mut ::std::os::raw::c_void,
        bytesize: usize,
        flags: ::std::os::raw::c_uint,
    ) -> hipError_t {
        hipHostMalloc(pp, bytesize, flags)
    }

    pub(crate) unsafe fn cuMemHostRegister(
        p: *mut ::std::os::raw::c_void,
        bytesize: usize,
        Flags: ::std::os::raw::c_uint,
    ) -> hipError_t {
        hipHostRegister(p, bytesize, Flags)
    }

    pub(crate) unsafe fn cuMemHostRegister_v2(
        p: *mut ::std::os::raw::c_void,
        bytesize: usize,
        Flags: ::std::os::raw::c_uint,
    ) -> hipError_t {
        hipHostRegister(p, bytesize, Flags)
    }

    pub(crate) unsafe fn cuMemHostUnregister(p: *mut ::std::os::raw::c_void) -> hipError_t {
        hipHostUnregister(p)
    }

    pub(crate) unsafe fn cuMemGetAddressRange_v2(
        pbase: *mut hipDeviceptr_t,
        psize: *mut usize,
        dptr: hipDeviceptr_t,
    ) -> hipError_t {
        memory::get_address_range(pbase, psize, dptr)
    }

    pub(crate) unsafe fn cuMemPoolSetAttribute(
        pool: hipMemPool_t,
        attr: hipMemPoolAttr,
        value: *mut ::std::os::raw::c_void,
    ) -> hipError_t {
        hipMemPoolGetAttribute(pool, attr, value)
    }

    pub(crate) unsafe fn cuMemPrefetchAsync(
        devPtr: hipDeviceptr_t,
        count: usize,
        dev: hipDevice_t,
        hStream: *mut stream::Stream,
    ) -> Result<(), CUresult> {
        memory::prefetch_async(devPtr, count, dev, hStream)
    }

    pub(crate) unsafe fn cuDeviceGetPCIBusId(
        pciBusId: *mut ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
        dev: hipDevice_t,
    ) -> hipError_t {
        hipDeviceGetPCIBusId(pciBusId, len, dev)
    }

    pub(crate) unsafe fn cuMemcpy(
        dst: hipDeviceptr_t,
        src: hipDeviceptr_t,
        ByteCount: usize,
    ) -> hipError_t {
        hipMemcpy(dst.0, src.0, ByteCount, hipMemcpyKind::hipMemcpyDefault)
    }

    pub(crate) unsafe fn cuMemcpy_ptds(
        dst: hipDeviceptr_t,
        src: hipDeviceptr_t,
        ByteCount: usize,
    ) -> hipError_t {
        hipMemcpy_spt(dst.0, src.0, ByteCount, hipMemcpyKind::hipMemcpyDefault)
    }

    pub(crate) unsafe fn cuMemcpyAsync(
        dst: hipDeviceptr_t,
        src: hipDeviceptr_t,
        ByteCount: usize,
        hStream: *mut stream::Stream,
    ) -> Result<(), CUresult> {
        memory::copy_async(dst, src, ByteCount, hStream, false)
    }

    pub(crate) unsafe fn cuMemcpyAsync_ptsz(
        dst: hipDeviceptr_t,
        src: hipDeviceptr_t,
        ByteCount: usize,
        hStream: *mut stream::Stream,
    ) -> Result<(), CUresult> {
        memory::copy_async(dst, src, ByteCount, hStream, true)
    }

    pub(crate) unsafe fn cuMemcpyHtoD_v2(
        dstDevice: hipDeviceptr_t,
        srcHost: *const ::std::os::raw::c_void,
        ByteCount: usize,
    ) -> hipError_t {
        hipMemcpyHtoD(dstDevice, srcHost as _, ByteCount)
    }

    pub(crate) unsafe fn cuMemcpyHtoD_v2_ptds(
        dstDevice: hipDeviceptr_t,
        srcHost: *const ::std::os::raw::c_void,
        ByteCount: usize,
    ) -> hipError_t {
        hipMemcpy_spt(
            dstDevice.0,
            srcHost,
            ByteCount,
            hipMemcpyKind::hipMemcpyHostToDevice,
        )
    }

    pub(crate) unsafe fn cuMemcpyDtoH_v2(
        dstHost: *mut ::std::os::raw::c_void,
        srcDevice: hipDeviceptr_t,
        ByteCount: usize,
    ) -> hipError_t {
        hipMemcpyDtoH(dstHost, srcDevice, ByteCount)
    }

    pub(crate) unsafe fn cuMemcpyDtoH_v2_ptds(
        dstHost: *mut ::std::os::raw::c_void,
        srcDevice: hipDeviceptr_t,
        ByteCount: usize,
    ) -> hipError_t {
        hipMemcpy_spt(
            dstHost,
            srcDevice.0,
            ByteCount,
            hipMemcpyKind::hipMemcpyDeviceToHost,
        )
    }

    pub(crate) unsafe fn cuMemcpyDtoD_v2(
        dstDevice: hipDeviceptr_t,
        srcDevice: hipDeviceptr_t,
        ByteCount: usize,
    ) -> hipError_t {
        hipMemcpyDtoD(dstDevice, srcDevice, ByteCount)
    }

    pub(crate) unsafe fn cuMemcpyDtoDAsync_v2(
        dstDevice: hipDeviceptr_t,
        srcDevice: hipDeviceptr_t,
        ByteCount: usize,
        hStream: *mut stream::Stream,
    ) -> Result<(), CUresult> {
        memory::copy_dtd_async(dstDevice, srcDevice, ByteCount, hStream, false)
    }

    pub(crate) unsafe fn cuMemcpyDtoDAsync_v2_ptsz(
        dstDevice: hipDeviceptr_t,
        srcDevice: hipDeviceptr_t,
        ByteCount: usize,
        hStream: *mut stream::Stream,
    ) -> Result<(), CUresult> {
        memory::copy_dtd_async(dstDevice, srcDevice, ByteCount, hStream, true)
    }

    pub(crate) unsafe fn cuMemcpyHtoDAsync_v2(
        dstDevice: hipDeviceptr_t,
        srcHost: *const ::std::os::raw::c_void,
        ByteCount: usize,
        hStream: *mut stream::Stream,
    ) -> Result<(), CUresult> {
        memory::copy_h_to_d_async(dstDevice, srcHost, ByteCount, hStream, false)
    }

    pub(crate) unsafe fn cuMemcpyHtoDAsync_v2_ptsz(
        dstDevice: hipDeviceptr_t,
        srcHost: *const ::std::os::raw::c_void,
        ByteCount: usize,
        hStream: *mut stream::Stream,
    ) -> Result<(), CUresult> {
        memory::copy_h_to_d_async(dstDevice, srcHost, ByteCount, hStream, true)
    }

    pub(crate) unsafe fn cuMemcpyDtoHAsync_v2(
        dstHost: *mut ::std::os::raw::c_void,
        srcDevice: hipDeviceptr_t,
        ByteCount: usize,
        hStream: *mut stream::Stream,
    ) -> Result<(), CUresult> {
        memory::copy_d_to_h_async(dstHost, srcDevice, ByteCount, hStream, false)
    }

    pub(crate) unsafe fn cuMemcpyDtoHAsync_v2_ptsz(
        dstHost: *mut ::std::os::raw::c_void,
        srcDevice: hipDeviceptr_t,
        ByteCount: usize,
        hStream: *mut stream::Stream,
    ) -> Result<(), CUresult> {
        memory::copy_d_to_h_async(dstHost, srcDevice, ByteCount, hStream, true)
    }

    pub(crate) unsafe fn cuMemcpy2D_v2(copy: *const CUDA_MEMCPY2D) -> hipError_t {
        memory::copy2d(copy)
    }

    pub(crate) unsafe fn cuMemcpy2DAsync_v2(
        copy: *const CUDA_MEMCPY2D,
        hStream: *mut stream::Stream,
    ) -> Result<(), CUresult> {
        memory::copy2d_async(copy, hStream)
    }

    pub(crate) unsafe fn cuMemcpy2DUnaligned_v2(copy: *const CUDA_MEMCPY2D) -> hipError_t {
        memory::copy2d_unaligned(copy)
    }

    pub(crate) unsafe fn cuMemcpy3D_v2(copy: *const CUDA_MEMCPY3D) -> Result<(), CUresult> {
        memory::copy3d(copy)
    }

    pub(crate) unsafe fn cuMemcpy3DAsync_v2(
        copy: *const CUDA_MEMCPY3D,
        hStream: *mut stream::Stream,
    ) -> Result<(), CUresult> {
        memory::copy3d_async(copy, hStream)
    }

    pub(crate) unsafe fn cuMemsetD8_v2(
        dstDevice: hipDeviceptr_t,
        uc: ::std::os::raw::c_uchar,
        N: usize,
    ) -> hipError_t {
        hipMemsetD8(dstDevice, uc, N)
    }

    pub(crate) unsafe fn cuMemsetD8_v2_ptds(
        dstDevice: hipDeviceptr_t,
        uc: ::std::os::raw::c_uchar,
        N: usize,
    ) -> hipError_t {
        memory::set_d8_ptds(dstDevice, uc, N)
    }

    pub(crate) unsafe fn cuMemsetD8Async(
        dstDevice: hipDeviceptr_t,
        uc: ::std::os::raw::c_uchar,
        N: usize,
        hStream: *mut stream::Stream,
    ) -> Result<(), CUresult> {
        memory::set_d8_async(dstDevice, uc, N, hStream, false)
    }

    pub(crate) unsafe fn cuMemsetD8Async_ptsz(
        dstDevice: hipDeviceptr_t,
        uc: ::std::os::raw::c_uchar,
        N: usize,
        hStream: *mut stream::Stream,
    ) -> Result<(), CUresult> {
        memory::set_d8_async(dstDevice, uc, N, hStream, true)
    }

    pub(crate) unsafe fn cuMemsetD16_v2(
        dstDevice: hipDeviceptr_t,
        us: ::std::os::raw::c_ushort,
        N: usize,
    ) -> hipError_t {
        hipMemsetD16(dstDevice, us, N)
    }

    pub(crate) unsafe fn cuMemsetD32Async(
        dstDevice: hipDeviceptr_t,
        ui: ::std::os::raw::c_uint,
        N: usize,
        hStream: *mut stream::Stream,
    ) -> Result<(), CUresult> {
        memory::set_d32_async(dstDevice, ui, N, hStream)
    }

    pub(crate) unsafe fn cuMemsetD16_v2_ptds(
        dstDevice: hipDeviceptr_t,
        us: ::std::os::raw::c_ushort,
        N: usize,
    ) -> hipError_t {
        hipMemsetD16(dstDevice, us, N)
    }

    pub(crate) unsafe fn cuMemsetD32_v2(
        dstDevice: hipDeviceptr_t,
        ui: ::std::os::raw::c_uint,
        N: usize,
    ) -> hipError_t {
        hipMemsetD32(dstDevice, ui as i32, N)
    }

    pub(crate) unsafe fn cuMemsetD32_v2_ptds(
        dstDevice: hipDeviceptr_t,
        ui: ::std::os::raw::c_uint,
        N: usize,
    ) -> hipError_t {
        hipMemset_spt(dstDevice.0, ui as i32, N)
    }

    pub(crate) unsafe fn cuMemsetD2D8_v2(
        dst_device: hipDeviceptr_t,
        dst_pitch: usize,
        uc: ::std::os::raw::c_uchar,
        width: usize,
        height: usize,
    ) -> hipError_t {
        hipMemset2D(
            dst_device.0,
            dst_pitch,
            i32::from_ne_bytes([uc, uc, uc, uc]),
            width,
            height,
        )
    }

    pub(crate) unsafe fn cuOccupancyMaxPotentialBlockSize(
        minGridSize: *mut ::std::os::raw::c_int,
        blockSize: *mut ::std::os::raw::c_int,
        func: *mut function::Function,
        blockSizeToDynamicSMemSize: CUoccupancyB2DSize,
        dynamicSMemSize: usize,
        blockSizeLimit: ::std::os::raw::c_int,
    ) -> Result<(), CUresult> {
        function::occupancy_max_potential_block_size(
            minGridSize,
            blockSize,
            func,
            blockSizeToDynamicSMemSize,
            dynamicSMemSize,
            blockSizeLimit,
        )
    }

    pub(crate) unsafe fn cuArrayCreate_v2(
        pHandle: *mut CUarray,
        pAllocateArray: *const HIP_ARRAY_DESCRIPTOR,
    ) -> Result<(), CUresult> {
        array::create(pHandle, pAllocateArray)
    }

    pub(crate) unsafe fn cuArrayDestroy(hArray: CUarray) -> hipError_t {
        let cu_array = hipfix::array::get(hArray);
        hipArrayDestroy(cu_array)
    }

    pub(crate) unsafe fn cuArray3DCreate_v2(
        pHandle: *mut CUarray,
        pAllocateArray: *const HIP_ARRAY3D_DESCRIPTOR,
    ) -> Result<(), CUresult> {
        array::create_3d(pHandle, pAllocateArray)
    }

    pub(crate) unsafe fn cuArray3DGetDescriptor_v2(
        pArrayDescriptor: *mut CUDA_ARRAY3D_DESCRIPTOR,
        hArray: CUarray,
    ) -> hipError_t {
        array::get_descriptor_3d(pArrayDescriptor, hArray)
    }

    pub(crate) unsafe fn cuPointerGetAttribute(
        data: *mut ::std::os::raw::c_void,
        attribute: hipPointer_attribute,
        ptr: hipDeviceptr_t,
    ) -> Result<(), CUresult> {
        pointer::get_attribute(data, attribute, ptr)
    }

    pub(crate) unsafe fn cuPointerGetAttributes(
        numAttributes: ::std::os::raw::c_uint,
        attributes: *mut hipPointer_attribute,
        data: *mut *mut ::std::os::raw::c_void,
        ptr: hipDeviceptr_t,
    ) -> hipError_t {
        pointer::get_attributes(numAttributes, attributes, data, ptr)
    }

    pub(crate) unsafe fn cuStreamCreate(
        phStream: *mut *mut stream::Stream,
        Flags: ::std::os::raw::c_uint,
    ) -> Result<(), CUresult> {
        stream::create_with_priority(phStream, Flags, 0)
    }

    pub(crate) unsafe fn cuStreamCreateWithPriority(
        phStream: *mut *mut stream::Stream,
        flags: ::std::os::raw::c_uint,
        priority: ::std::os::raw::c_int,
    ) -> Result<(), CUresult> {
        stream::create_with_priority(phStream, flags, priority)
    }

    pub(crate) unsafe fn cuStreamGetCaptureInfo(
        stream: *mut stream::Stream,
        captureStatus_out: *mut hipStreamCaptureStatus,
        id_out: *mut cuuint64_t,
    ) -> Result<(), CUresult> {
        stream::get_capture_info(stream, captureStatus_out, id_out)
    }

    pub(crate) unsafe fn cuStreamGetCtx(
        hStream: *mut stream::Stream,
        pctx: *mut *mut context::Context,
    ) -> Result<(), CUresult> {
        stream::get_ctx(hStream, pctx)
    }

    pub(crate) unsafe fn cuStreamGetCtx_ptsz(
        hStream: *mut stream::Stream,
        pctx: *mut *mut context::Context,
    ) -> Result<(), CUresult> {
        stream::get_ctx(hStream, pctx)
    }

    pub(crate) unsafe fn cuStreamGetFlags(
        hStream: *mut stream::Stream,
        flags: *mut ::std::os::raw::c_uint,
    ) -> Result<(), CUresult> {
        stream::get_flags(hStream, flags)
    }

    pub(crate) unsafe fn cuStreamIsCapturing(
        hStream: *mut stream::Stream,
        captureStatus: *mut hipStreamCaptureStatus,
    ) -> Result<(), CUresult> {
        stream::is_capturing(hStream, captureStatus)
    }

    pub(crate) unsafe fn cuStreamQuery(hStream: *mut stream::Stream) -> Result<(), CUresult> {
        stream::query(hStream)
    }

    pub(crate) unsafe fn cuStreamSynchronize(hStream: *mut stream::Stream) -> Result<(), CUresult> {
        stream::synchronize(hStream, false)
    }

    pub(crate) unsafe fn cuStreamSynchronize_ptsz(
        hStream: *mut stream::Stream,
    ) -> Result<(), CUresult> {
        stream::synchronize(hStream, true)
    }

    pub(crate) unsafe fn cuStreamDestroy(hStream: *mut stream::Stream) -> Result<(), CUresult> {
        stream::destroy(hStream)
    }

    pub(crate) unsafe fn cuStreamDestroy_v2(hStream: *mut stream::Stream) -> Result<(), CUresult> {
        stream::destroy(hStream)
    }

    pub(crate) unsafe fn cuStreamWaitEvent(
        hStream: *mut stream::Stream,
        hEvent: hipEvent_t,
        Flags: ::std::os::raw::c_uint,
    ) -> Result<(), CUresult> {
        stream::wait_event(hStream, hEvent, Flags, false)
    }

    pub(crate) unsafe fn cuStreamWaitEvent_ptsz(
        hStream: *mut stream::Stream,
        hEvent: hipEvent_t,
        Flags: ::std::os::raw::c_uint,
    ) -> Result<(), CUresult> {
        stream::wait_event(hStream, hEvent, Flags, true)
    }

    pub(crate) unsafe fn cuFuncGetAttribute(
        pi: *mut ::std::os::raw::c_int,
        attrib: hipFunction_attribute,
        func: *mut function::Function,
    ) -> Result<(), CUresult> {
        function::get_attribute(pi, attrib, func)
    }

    pub(crate) unsafe fn cuFuncSetAttribute(
        func: *mut function::Function,
        attrib: hipFunction_attribute,
        value: ::std::os::raw::c_int,
    ) -> Result<(), CUresult> {
        function::set_attribute(func, attrib, value)
    }

    pub(crate) unsafe fn cuLaunchHostFunc(
        stream: *mut stream::Stream,
        fn_: CUhostFn,
        userData: *mut ::std::os::raw::c_void,
    ) -> Result<(), CUresult> {
        stream::launch_host_func(stream, fn_, userData)
    }

    pub(crate) unsafe fn cuLaunchKernel(
        f: *mut function::Function,
        gridDimX: ::std::os::raw::c_uint,
        gridDimY: ::std::os::raw::c_uint,
        gridDimZ: ::std::os::raw::c_uint,
        blockDimX: ::std::os::raw::c_uint,
        blockDimY: ::std::os::raw::c_uint,
        blockDimZ: ::std::os::raw::c_uint,
        sharedMemBytes: ::std::os::raw::c_uint,
        hStream: *mut stream::Stream,
        kernelParams: *mut *mut ::std::os::raw::c_void,
        extra: *mut *mut ::std::os::raw::c_void,
    ) -> Result<(), CUresult> {
        function::launch_kernel(
            f,
            gridDimX,
            gridDimY,
            gridDimZ,
            blockDimX,
            blockDimY,
            blockDimZ,
            sharedMemBytes,
            hStream,
            kernelParams,
            extra,
            false,
        )
    }

    pub(crate) unsafe fn cuLaunchKernel_ptsz(
        f: *mut function::Function,
        gridDimX: ::std::os::raw::c_uint,
        gridDimY: ::std::os::raw::c_uint,
        gridDimZ: ::std::os::raw::c_uint,
        blockDimX: ::std::os::raw::c_uint,
        blockDimY: ::std::os::raw::c_uint,
        blockDimZ: ::std::os::raw::c_uint,
        sharedMemBytes: ::std::os::raw::c_uint,
        hStream: *mut stream::Stream,
        kernelParams: *mut *mut ::std::os::raw::c_void,
        extra: *mut *mut ::std::os::raw::c_void,
    ) -> Result<(), CUresult> {
        function::launch_kernel(
            f,
            gridDimX,
            gridDimY,
            gridDimZ,
            blockDimX,
            blockDimY,
            blockDimZ,
            sharedMemBytes,
            hStream,
            kernelParams,
            extra,
            true,
        )
    }

    pub(crate) unsafe fn cuMemHostGetDevicePointer_v2(
        pdptr: *mut hipDeviceptr_t,
        p: *mut ::std::os::raw::c_void,
        Flags: ::std::os::raw::c_uint,
    ) -> hipError_t {
        memory::host_get_device_pointer(pdptr, p, Flags)
    }

    pub(crate) unsafe fn cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags(
        num_blocks: *mut ::std::os::raw::c_int,
        func: *mut function::Function,
        block_size: ::std::os::raw::c_int,
        dynamic_smem_size: usize,
        flags: ::std::os::raw::c_uint,
    ) -> Result<(), CUresult> {
        function::occupancy_max_potential_blocks_per_multiprocessor(
            num_blocks,
            func,
            block_size,
            dynamic_smem_size,
            flags,
        )
    }

    pub(crate) unsafe fn cuSurfObjectCreate(
        pSurfObject: *mut hipSurfaceObject_t,
        pResDesc: *const CUDA_RESOURCE_DESC,
    ) -> Result<(), CUresult> {
        surface::create(pSurfObject, pResDesc)
    }

    pub(crate) unsafe fn cuSurfObjectDestroy(
        surfObject: hipSurfaceObject_t,
    ) -> hipError_t {
        hipDestroySurfaceObject(surfObject)
    }

    pub(crate) unsafe fn cuTexObjectCreate(
        pTexObject: *mut hipTextureObject_t,
        pResDesc: *const CUDA_RESOURCE_DESC,
        pTexDesc: *const HIP_TEXTURE_DESC,
        pResViewDesc: *const HIP_RESOURCE_VIEW_DESC,
    ) -> hipError_t {
        texobj::create(pTexObject, pResDesc, pTexDesc, pResViewDesc)
    }

    pub(crate) unsafe fn cuTexObjectDestroy(texObject: hipTextureObject_t) -> hipError_t {
        hipTexObjectDestroy(texObject)
    }

    pub(crate) unsafe fn cuTexRefGetAddress_v2(
        pdptr: *mut hipDeviceptr_t,
        tex_ref: *mut textureReference,
    ) -> hipError_t {
        hipTexRefGetAddress(pdptr, tex_ref)
    }

    pub(crate) unsafe fn cuTexRefGetAddressMode(
        pam: *mut hipTextureAddressMode,
        tex_ref: *mut textureReference,
        dim: ::std::os::raw::c_int,
    ) -> hipError_t {
        hipTexRefGetAddressMode(pam, tex_ref, dim)
    }

    pub(crate) unsafe fn cuTexRefGetFilterMode(
        pfm: *mut hipTextureFilterMode,
        tex_ref: *mut textureReference,
    ) -> hipError_t {
        hipTexRefGetFilterMode(pfm, tex_ref)
    }

    pub(crate) unsafe fn cuTexRefGetFlags(
        flags: *mut ::std::os::raw::c_uint,
        tex_ref: *mut textureReference,
    ) -> hipError_t {
        hipTexRefGetFlags(flags, tex_ref)
    }

    pub(crate) unsafe fn cuTexRefGetMipmapFilterMode(
        pfm: *mut hipTextureFilterMode,
        tex_ref: *mut textureReference,
    ) -> hipError_t {
        texref::get_mipmap_filter_mode(pfm, tex_ref)
    }

    pub(crate) unsafe fn cuTexRefGetMipmapLevelBias(
        pbias: *mut f32,
        tex_ref: *mut textureReference,
    ) -> hipError_t {
        texref::get_mipmap_level_bias(pbias, tex_ref)
    }

    pub(crate) unsafe fn cuTexRefGetMipmapLevelClamp(
        min_mipmap_level_clamp: *mut f32,
        max_mipmap_level_clamp: *mut f32,
        tex_ref: *mut textureReference,
    ) -> hipError_t {
        texref::get_mipmap_level_clamp(min_mipmap_level_clamp, max_mipmap_level_clamp, tex_ref)
    }

    pub(crate) unsafe fn cuTexRefGetMaxAnisotropy(
        pmaxAniso: *mut ::std::os::raw::c_int,
        tex_ref: *mut textureReference,
    ) -> hipError_t {
        texref::get_max_anisotropy(pmaxAniso, tex_ref)
    }

    pub(crate) unsafe fn cuTexRefSetAddress2D_v3(
        tex_ref: *mut textureReference,
        desc: *const HIP_ARRAY_DESCRIPTOR,
        dptr: hipDeviceptr_t,
        pitch: usize,
    ) -> hipError_t {
        hipTexRefSetAddress2D(tex_ref, desc, dptr, pitch)
    }

    pub(crate) unsafe fn cuTexRefSetAddressMode(
        tex_ref: *mut textureReference,
        dim: ::std::os::raw::c_int,
        am: hipTextureAddressMode,
    ) -> Result<(), CUresult> {
        texref::set_address_mode(tex_ref, dim, am)
    }

    pub(crate) unsafe fn cuTexRefSetAddress_v2(
        byte_offset: *mut usize,
        tex_ref: *mut textureReference,
        dptr: hipDeviceptr_t,
        bytes: usize,
    ) -> hipError_t {
        texref::set_address(byte_offset, tex_ref, dptr, bytes)
    }

    pub(crate) unsafe fn cuTexRefSetArray(
        hTexRef: *mut textureReference,
        hArray: CUarray,
        Flags: ::std::os::raw::c_uint,
    ) -> Result<(), CUresult> {
        texref::set_array(hTexRef, hArray, Flags)
    }

    pub(crate) unsafe fn cuTexRefSetFilterMode(
        tex_ref: *mut textureReference,
        fm: hipTextureFilterMode,
    ) -> Result<(), CUresult> {
        texref::set_filter_mode(tex_ref, fm)
    }

    pub(crate) unsafe fn cuTexRefSetFlags(
        tex_ref: *mut textureReference,
        flags: ::std::os::raw::c_uint,
    ) -> Result<(), CUresult> {
        texref::set_flags(tex_ref, flags)
    }

    pub(crate) unsafe fn cuTexRefSetFormat(
        tex_ref: *mut textureReference,
        fmt: hipArray_Format,
        num_packed_components: ::std::os::raw::c_int,
    ) -> Result<(), CUresult> {
        texref::set_format(tex_ref, fmt, num_packed_components)
    }

    pub(crate) unsafe fn cuTexRefGetFormat(
        pFormat: *mut hipArray_Format,
        pNumChannels: *mut ::std::os::raw::c_int,
        hTexRef: *mut textureReference,
    ) -> hipError_t {
        hipTexRefGetFormat(pFormat, pNumChannels, hTexRef)
    }

    pub(crate) unsafe fn cuTexRefSetMaxAnisotropy(
        tex_ref: *mut textureReference,
        max_aniso: ::std::os::raw::c_uint,
    ) -> Result<(), CUresult> {
        texref::set_max_anisotropy(tex_ref, max_aniso)
    }

    pub(crate) unsafe fn cuTexRefSetMipmapFilterMode(
        tex_ref: *mut textureReference,
        fm: hipTextureFilterMode,
    ) -> Result<(), CUresult> {
        texref::set_mipmap_filter_mode(tex_ref, fm)
    }

    pub(crate) unsafe fn cuTexRefSetMipmapLevelBias(
        tex_ref: *mut textureReference,
        bias: f32,
    ) -> Result<(), CUresult> {
        texref::set_mipmap_level_bias(tex_ref, bias)
    }

    pub(crate) unsafe fn cuTexRefSetMipmapLevelClamp(
        tex_ref: *mut textureReference,
        min_mipmap_level_clamp: f32,
        max_mipmap_level_clamp: f32,
    ) -> Result<(), CUresult> {
        texref::set_mipmap_level_clamp(tex_ref, min_mipmap_level_clamp, max_mipmap_level_clamp)
    }

    pub(crate) unsafe fn cuSurfRefSetArray(
        hSurfRef: *mut textureReference,
        hArray: CUarray,
        Flags: ::std::os::raw::c_uint,
    ) -> Result<(), CUresult> {
        surfref::set_array(hSurfRef, hArray, Flags)
    }

    pub(crate) unsafe fn cuFuncSetBlockShape(
        hfunc: *mut function::Function,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        z: ::std::os::raw::c_int,
    ) -> Result<(), CUresult> {
        Ok(())
    }

    pub(crate) unsafe fn cuEventCreate(
        phEvent: *mut hipEvent_t,
        Flags: ::std::os::raw::c_uint,
    ) -> hipError_t {
        hipEventCreate(phEvent)
    }

    pub(crate) unsafe fn cuEventDestroy(event: hipEvent_t) -> hipError_t {
        cuEventDestroy_v2(event)
    }

    pub(crate) unsafe fn cuEventDestroy_v2(event: hipEvent_t) -> hipError_t {
        hipEventDestroy(event)
    }

    pub(crate) unsafe fn cuEventQuery(event: hipEvent_t) -> hipError_t {
        hipEventQuery(event)
    }

    pub(crate) unsafe fn cuEventElapsedTime(
        ms: *mut f32,
        start: hipEvent_t,
        stop: hipEvent_t,
    ) -> hipError_t {
        hipEventElapsedTime(ms, start, stop)
    }

    pub(crate) unsafe fn cuEventRecord(
        event: hipEvent_t,
        stream: *mut stream::Stream,
    ) -> Result<(), CUresult> {
        let stream = stream::as_hip_stream(stream)?;
        hip_call_cuda!(hipEventRecord(event, stream));
        Ok(())
    }

    pub(crate) unsafe fn cuEventRecord_ptsz(
        event: hipEvent_t,
        stream: *mut stream::Stream,
    ) -> Result<(), CUresult> {
        let stream = hipfix::as_hip_stream_per_thread(stream, true)?;
        hip_call_cuda!(hipEventRecord(event, stream));
        Ok(())
    }

    pub(crate) unsafe fn cuEventSynchronize(event: hipEvent_t) -> hipError_t {
        hipEventSynchronize(event)
    }

    pub(crate) unsafe fn cuGraphAddDependencies(
        graph: hipGraph_t,
        from: *const hipGraphNode_t,
        to: *const hipGraphNode_t,
        numDependencies: usize,
    ) -> hipError_t {
        hipGraphAddDependencies(graph, from, to, numDependencies)
    }

    pub(crate) unsafe fn cuGraphAddEmptyNode(
        pGraphNode: *mut hipGraphNode_t,
        graph: hipGraph_t,
        pDependencies: *const hipGraphNode_t,
        numDependencies: usize,
    ) -> hipError_t {
        hipGraphAddEmptyNode(pGraphNode, graph, pDependencies, numDependencies)
    }

    pub(crate) unsafe fn cuGraphAddKernelNode(
        phGraphNode: *mut hipGraphNode_t,
        hGraph: hipGraph_t,
        dependencies: *const hipGraphNode_t,
        numDependencies: usize,
        nodeParams: *const CUDA_KERNEL_NODE_PARAMS_v1,
    ) -> Result<(), CUresult> {
        graph::add_kernel_node(
            phGraphNode,
            hGraph,
            dependencies,
            numDependencies,
            nodeParams,
        )
    }

    pub(crate) unsafe fn cuGraphCreate(
        phGraph: *mut hipGraph_t,
        flags: ::std::os::raw::c_uint,
    ) -> hipError_t {
        hipGraphCreate(phGraph, flags)
    }

    pub(crate) unsafe fn cuGraphDestroy(graph: hipGraph_t) -> hipError_t {
        hipGraphDestroy(graph)
    }

    pub(crate) unsafe fn cuGraphExecDestroy(graphExec: hipGraphExec_t) -> hipError_t {
        hipGraphExecDestroy(graphExec)
    }

    pub(crate) unsafe fn cuGraphInstantiate(
        phGraphExec: *mut hipGraphExec_t,
        hGraph: hipGraph_t,
        phErrorNode: *mut hipGraphNode_t,
        logBuffer: *mut ::std::os::raw::c_char,
        bufferSize: usize,
    ) -> hipError_t {
        hipGraphInstantiate(phGraphExec, hGraph, phErrorNode, logBuffer, bufferSize)
    }

    pub(crate) unsafe fn cuGraphInstantiate_v2(
        phGraphExec: *mut hipGraphExec_t,
        hGraph: hipGraph_t,
        phErrorNode: *mut hipGraphNode_t,
        logBuffer: *mut ::std::os::raw::c_char,
        bufferSize: usize,
    ) -> hipError_t {
        cuGraphInstantiate(phGraphExec, hGraph, phErrorNode, logBuffer, bufferSize)
    }

    pub(crate) unsafe fn cuGraphLaunch(
        hGraph: hipGraphExec_t,
        hStream: *mut stream::Stream,
    ) -> Result<(), CUresult> {
        graph::launch(hGraph, hStream)
    }

    pub(crate) unsafe fn cuGraphicsSubResourceGetMappedArray(
        pArray: *mut CUarray,
        resource: hipGraphicsResource_t,
        arrayIndex: ::std::os::raw::c_uint,
        mipLevel: ::std::os::raw::c_uint,
    ) -> hipError_t {
        hipGraphicsSubResourceGetMappedArray(pArray.cast(), resource, arrayIndex, mipLevel)
    }

    pub(crate) unsafe fn cuGraphicsGLRegisterBuffer(
        resource: *mut hipGraphicsResource_t,
        buffer: u32,
        flags: ::std::os::raw::c_uint,
    ) -> hipError_t {
        gl::register_buffer(resource, buffer, flags)
    }

    pub(crate) unsafe fn cuGraphicsGLRegisterImage(
        resource: *mut hipGraphicsResource_t,
        image: u32,
        target: u32,
        flags: ::std::os::raw::c_uint,
    ) -> hipError_t {
        gl::register_image(resource, image, target, flags)
    }

    pub(crate) unsafe fn cuGraphicsMapResources(
        count: ::std::os::raw::c_uint,
        resources: *mut hipGraphicsResource_t,
        hStream: *mut stream::Stream,
    ) -> Result<(), CUresult> {
        gl::map_resources(count, resources, hStream)
    }

    pub(crate) unsafe fn cuGraphicsResourceGetMappedPointer_v2(
        pDevPtr: *mut hipDeviceptr_t,
        pSize: *mut usize,
        resource: hipGraphicsResource_t,
    ) -> hipError_t {
        hipGraphicsResourceGetMappedPointer(pDevPtr.cast(), pSize, resource)
    }

    pub(crate) unsafe fn cuGraphicsUnmapResources(
        count: ::std::os::raw::c_uint,
        resources: *mut hipGraphicsResource_t,
        hStream: *mut stream::Stream,
    ) -> Result<(), CUresult> {
        gl::unmap_resources(count, resources, hStream)
    }

    pub(crate) unsafe fn cuGraphicsUnregisterResource(
        resource: hipGraphicsResource_t,
    ) -> hipError_t {
        hipGraphicsUnregisterResource(resource)
    }

    pub(crate) unsafe fn cuLinkAddData_v2(
        state: *mut link::LinkState,
        type_: CUjitInputType,
        data: *mut ::std::os::raw::c_void,
        size: usize,
        name: *const ::std::os::raw::c_char,
        numOptions: ::std::os::raw::c_uint,
        options: *mut CUjit_option,
        optionValues: *mut *mut ::std::os::raw::c_void,
    ) -> Result<(), CUresult> {
        link::add_data(
            state,
            type_,
            data,
            size,
            name,
            numOptions,
            options,
            optionValues,
        )
    }

    pub(crate) unsafe fn cuLinkComplete(
        state: *mut link::LinkState,
        cubinOut: *mut *mut ::std::os::raw::c_void,
        sizeOut: *mut usize,
    ) -> Result<(), CUresult> {
        link::complete(state, cubinOut, sizeOut)
    }

    pub(crate) unsafe fn cuLinkDestroy(state: *mut link::LinkState) -> Result<(), CUresult> {
        link::destroy(state)
    }

    pub(crate) unsafe fn cuLinkCreate_v2(
        numOptions: ::std::os::raw::c_uint,
        options: *mut CUjit_option,
        optionValues: *mut *mut ::std::os::raw::c_void,
        stateOut: *mut *mut link::LinkState,
    ) -> Result<(), CUresult> {
        link::create(numOptions, options, optionValues, stateOut)
    }
}
