use super::{context, driver};
use cuda_types::cuda::*;
use hip_runtime_sys::*;
use std::mem;
use zluda_common::constants::{COMPUTE_CAPABILITY_MAJOR, COMPUTE_CAPABILITY_MINOR};

pub(crate) fn compute_capability(major: &mut i32, minor: &mut i32, _dev: hipDevice_t) -> CUresult {
    *major = COMPUTE_CAPABILITY_MAJOR;
    *minor = COMPUTE_CAPABILITY_MINOR;
    Ok(())
}

pub(crate) fn get(device: *mut hipDevice_t, ordinal: i32) -> hipError_t {
    unsafe { hipDeviceGet(device, ordinal) }
}

#[allow(warnings)]
trait DeviceAttributeNames {
    const hipDeviceAttributeGpuOverlap: hipDeviceAttribute_t =
        hipDeviceAttribute_t::hipDeviceAttributeDeviceOverlap;
    const hipDeviceAttributeMaximumTexture1DWidth: hipDeviceAttribute_t =
        hipDeviceAttribute_t::hipDeviceAttributeMaxTexture1DWidth;
    const hipDeviceAttributeMaximumTexture2DWidth: hipDeviceAttribute_t =
        hipDeviceAttribute_t::hipDeviceAttributeMaxTexture2DWidth;
    const hipDeviceAttributeMaximumTexture2DHeight: hipDeviceAttribute_t =
        hipDeviceAttribute_t::hipDeviceAttributeMaxTexture2DHeight;
    const hipDeviceAttributeMaximumTexture3DWidth: hipDeviceAttribute_t =
        hipDeviceAttribute_t::hipDeviceAttributeMaxTexture3DWidth;
    const hipDeviceAttributeMaximumTexture3DHeight: hipDeviceAttribute_t =
        hipDeviceAttribute_t::hipDeviceAttributeMaxTexture3DHeight;
    const hipDeviceAttributeMaximumTexture3DDepth: hipDeviceAttribute_t =
        hipDeviceAttribute_t::hipDeviceAttributeMaxTexture3DDepth;
    const hipDeviceAttributeGlobalMemoryBusWidth: hipDeviceAttribute_t =
        hipDeviceAttribute_t::hipDeviceAttributeMemoryBusWidth;
    const hipDeviceAttributeMaxThreadsPerMultiprocessor: hipDeviceAttribute_t =
        hipDeviceAttribute_t::hipDeviceAttributeMaxThreadsPerMultiProcessor;
    const hipDeviceAttributeAsyncEngineCount: hipDeviceAttribute_t =
        hipDeviceAttribute_t::hipDeviceAttributeConcurrentKernels;
    const hipDeviceAttributePciDomainId: hipDeviceAttribute_t =
        hipDeviceAttribute_t::hipDeviceAttributePciDomainID;
    const hipDeviceAttributeMultiGpuBoard: hipDeviceAttribute_t =
        hipDeviceAttribute_t::hipDeviceAttributeIsMultiGpuBoard;
    const hipDeviceAttributeMultiGpuBoardGroupId: hipDeviceAttribute_t =
        hipDeviceAttribute_t::hipDeviceAttributeMultiGpuBoardGroupID;
    const hipDeviceAttributeMaxSharedMemoryPerBlockOptin: hipDeviceAttribute_t =
        hipDeviceAttribute_t::hipDeviceAttributeSharedMemPerBlockOptin;
}

impl DeviceAttributeNames for hipDeviceAttribute_t {}

macro_rules! remap_attribute {
    ($attrib:expr => { $([ $($word:expr)* ]),*, }, { $( $exactWord:expr => $hipWord:expr ),*, }) => {
        match $attrib {
            $(
                paste::paste! { CUdevice_attribute:: [< CU_DEVICE_ATTRIBUTE $(_ $word:upper)* >] } => {
                    paste::paste! { hipDeviceAttribute_t:: [< hipDeviceAttribute $($word:camel)* >] }
                }
            )*
            $(
                paste::paste! { CUdevice_attribute:: [< CU_DEVICE_ATTRIBUTE_ $exactWord >] } => {
                    paste::paste! { hipDeviceAttribute_t:: [< hipDeviceAttribute $hipWord >] }
                }
            )*
            _ => return Err(hipErrorCode_t::InvalidValue)
        }
    }
}

pub(crate) fn get_attribute(
    pi: &mut i32,
    attrib: CUdevice_attribute,
    dev_idx: hipDevice_t,
) -> hipError_t {
    fn get_device_prop(
        pi: &mut i32,
        dev_idx: hipDevice_t,
        f: impl FnOnce(&hipDeviceProp_tR0600) -> i32,
    ) -> hipError_t {
        let mut props = unsafe { mem::zeroed() };
        unsafe { hipGetDevicePropertiesR0600(&mut props, dev_idx)? };
        *pi = f(&props);
        Ok(())
    }
    match attrib {
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_WARP_SIZE => {
            *pi = 32;
            return Ok(());
        }
        // TODO: maintain a table, certain RDNAs are 1/16, some are 1/32
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_SINGLE_TO_DOUBLE_PRECISION_PERF_RATIO => {
            *pi = 32;
            return Ok(());
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_TCC_DRIVER
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MEMORY_POOLS_SUPPORTED
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MEMPOOL_SUPPORTED_HANDLE_TYPES
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_DMA_BUF_SUPPORTED
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_NUMA_CONFIG
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MPS_ENABLED
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_HOST_NUMA_ID
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_GPU_PCI_DEVICE_ID // todo: check
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_GPU_PCI_SUBSYSTEM_ID // todo: check
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_HOST_NUMA_MULTINODE_IPC_SUPPORTED => {
            *pi = 0;
            return Ok(());
        }
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_NUMA_ID => {
            *pi = -1;
            return Ok(());
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LAYERED_WIDTH => {
            return get_device_prop(pi, dev_idx, |props| props.maxTexture2DLayered[0])
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LAYERED_HEIGHT => {
            return get_device_prop(pi, dev_idx, |props| props.maxTexture2DLayered[1])
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LAYERED_LAYERS => {
            return get_device_prop(pi, dev_idx, |props| props.maxTexture2DLayered[2])
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LAYERED_WIDTH => {
            return get_device_prop(pi, dev_idx, |props| props.maxTexture1DLayered[0])
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LAYERED_LAYERS => {
            return get_device_prop(pi, dev_idx, |props| props.maxTexture1DLayered[1])
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_CAN_TEX2D_GATHER => {
            return get_device_prop(pi, dev_idx, |props| {
                (props.maxTexture2DGather[0] > 0 && props.maxTexture2DGather[1] > 0) as i32
            })
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_GATHER_WIDTH => {
            return get_device_prop(pi, dev_idx, |props| props.maxTexture2DGather[0])
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_GATHER_HEIGHT => {
            return get_device_prop(pi, dev_idx, |props| props.maxTexture2DGather[1])
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_WIDTH_ALTERNATE => {
            return get_device_prop(pi, dev_idx, |props| props.maxTexture3DAlt[0])
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_HEIGHT_ALTERNATE => {
            return get_device_prop(pi, dev_idx, |props| props.maxTexture3DAlt[1])
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_DEPTH_ALTERNATE => {
            return get_device_prop(pi, dev_idx, |props| props.maxTexture3DAlt[2])
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURECUBEMAP_WIDTH => {
            return get_device_prop(pi, dev_idx, |props| props.maxTextureCubemap)
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURECUBEMAP_LAYERED_WIDTH => {
            return get_device_prop(pi, dev_idx, |props| props.maxTextureCubemapLayered[0])
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURECUBEMAP_LAYERED_LAYERS => {
            return get_device_prop(pi, dev_idx, |props| props.maxTextureCubemapLayered[1])
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE1D_WIDTH => {
            return get_device_prop(pi, dev_idx, |props| props.maxSurface1D)
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_WIDTH => {
            return get_device_prop(pi, dev_idx, |props| props.maxSurface2D[0])
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_HEIGHT => {
            return get_device_prop(pi, dev_idx, |props| props.maxSurface2D[1])
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE3D_WIDTH => {
            return get_device_prop(pi, dev_idx, |props| props.maxSurface3D[0])
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE3D_HEIGHT => {
            return get_device_prop(pi, dev_idx, |props| props.maxSurface3D[1])
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE3D_DEPTH => {
            return get_device_prop(pi, dev_idx, |props| props.maxSurface3D[2])
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE1D_LAYERED_WIDTH => {
            return get_device_prop(pi, dev_idx, |props| props.maxSurface1DLayered[0])
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE1D_LAYERED_LAYERS => {
            return get_device_prop(pi, dev_idx, |props| props.maxSurface1DLayered[1])
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_LAYERED_WIDTH => {
            return get_device_prop(pi, dev_idx, |props| props.maxSurface2DLayered[0])
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_LAYERED_HEIGHT => {
            return get_device_prop(pi, dev_idx, |props| props.maxSurface2DLayered[1])
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_LAYERED_LAYERS => {
            return get_device_prop(pi, dev_idx, |props| props.maxSurface2DLayered[2])
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACECUBEMAP_WIDTH => {
            return get_device_prop(pi, dev_idx, |props| props.maxSurfaceCubemap)
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACECUBEMAP_LAYERED_WIDTH => {
            return get_device_prop(pi, dev_idx, |props| props.maxSurfaceCubemapLayered[0])
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACECUBEMAP_LAYERED_LAYERS => {
            return get_device_prop(pi, dev_idx, |props| props.maxSurfaceCubemapLayered[1])
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LINEAR_WIDTH => {
            return get_device_prop(pi, dev_idx, |props| props.maxTexture1DLinear)
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_WIDTH => {
            return get_device_prop(pi, dev_idx, |props| props.maxTexture2DLinear[0])
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_HEIGHT => {
            return get_device_prop(pi, dev_idx, |props| props.maxTexture2DLinear[1])
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_PITCH => {
            return get_device_prop(pi, dev_idx, |props| props.maxTexture2DLinear[2])
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_MIPMAPPED_WIDTH => {
            return get_device_prop(pi, dev_idx, |props| props.maxTexture2DMipmap[0])
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_MIPMAPPED_HEIGHT => {
            return get_device_prop(pi, dev_idx, |props| props.maxTexture2DMipmap[1])
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR => {
            *pi = COMPUTE_CAPABILITY_MAJOR;
            return Ok(());
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR => {
            *pi = COMPUTE_CAPABILITY_MINOR;
            return Ok(());
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_MIPMAPPED_WIDTH => {
            return get_device_prop(pi, dev_idx, |props| props.maxTexture1DMipmap)
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_PERSISTING_L2_CACHE_SIZE => {
            return get_device_prop(pi, dev_idx, |props| props.persistingL2CacheMaxSize)
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_ACCESS_POLICY_WINDOW_SIZE => {
            return get_device_prop(pi, dev_idx, |props| props.accessPolicyMaxWindowSize)
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_SPARSE_CUDA_ARRAY_SUPPORTED => {
            return get_device_prop(pi, dev_idx, |props| props.sparseHipArraySupported)
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_READ_ONLY_HOST_REGISTER_SUPPORTED => {
            return get_device_prop(pi, dev_idx, |props| props.hostRegisterReadOnlySupported)
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_TIMELINE_SEMAPHORE_INTEROP_SUPPORTED => {
            return get_device_prop(pi, dev_idx, |props| props.timelineSemaphoreInteropSupported)
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_GPU_DIRECT_RDMA_SUPPORTED => {
            return get_device_prop(pi, dev_idx, |props| props.gpuDirectRDMASupported)
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_GPU_DIRECT_RDMA_FLUSH_WRITES_OPTIONS => {
            return get_device_prop(pi, dev_idx, |props| {
                props.gpuDirectRDMAFlushWritesOptions as i32
            })
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_GPU_DIRECT_RDMA_WRITES_ORDERING => {
            return get_device_prop(pi, dev_idx, |props| props.gpuDirectRDMAWritesOrdering)
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_CLUSTER_LAUNCH => {
            return get_device_prop(pi, dev_idx, |props| props.clusterLaunch)
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_DEFERRED_MAPPING_CUDA_ARRAY_SUPPORTED => {
            return get_device_prop(pi, dev_idx, |props| props.deferredMappingHipArraySupported)
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_IPC_EVENT_SUPPORTED => {
            return get_device_prop(pi, dev_idx, |props| props.ipcEventSupported)
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_UNIFIED_FUNCTION_POINTERS => {
            return get_device_prop(pi, dev_idx, |props| props.unifiedFunctionPointers)
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_VIRTUAL_MEMORY_MANAGEMENT_SUPPORTED => {
            *pi = 0;
            return Ok(());
        }
        _ => {}
    }
    let attrib = remap_attribute! {
        attrib => {
            [MAX THREADS PER BLOCK],
            [MAX BLOCK DIM X],
            [MAX BLOCK DIM Y],
            [MAX BLOCK DIM Z],
            [MAX GRID DIM X],
            [MAX GRID DIM Y],
            [MAX GRID DIM Z],
            [MAX SHARED MEMORY PER BLOCK],
            [TOTAL CONSTANT MEMORY],
            //[WARP SIZE],
            [MAX PITCH],
            [MAX REGISTERS PER BLOCK],
            [CLOCK RATE],
            [TEXTURE ALIGNMENT],
            [GPU OVERLAP],
            [MULTIPROCESSOR COUNT],
            [KERNEL EXEC TIMEOUT],
            [INTEGRATED],
            [CAN MAP HOST MEMORY],
            [COMPUTE MODE],
            [MAXIMUM TEXTURE1D WIDTH],
            [MAXIMUM TEXTURE2D WIDTH],
            [MAXIMUM TEXTURE2D HEIGHT],
            [MAXIMUM TEXTURE3D WIDTH],
            [MAXIMUM TEXTURE3D HEIGHT],
            [MAXIMUM TEXTURE3D DEPTH],
            //[MAXIMUM TEXTURE2D LAYERED WIDTH],
            //[MAXIMUM TEXTURE2D LAYERED HEIGHT],
            //[MAXIMUM TEXTURE2D LAYERED LAYERS],
            //[MAXIMUM TEXTURE2D ARRAY WIDTH],
            //[MAXIMUM TEXTURE2D ARRAY HEIGHT],
            //[MAXIMUM TEXTURE2D ARRAY NUMSLICES],
            [SURFACE ALIGNMENT],
            [CONCURRENT KERNELS],
            [ECC ENABLED],
            [PCI BUS ID],
            [PCI DEVICE ID],
            //[TCC DRIVER],
            [MEMORY CLOCK RATE],
            [GLOBAL MEMORY BUS WIDTH],
            [L2 CACHE SIZE],
            [MAX THREADS PER MULTIPROCESSOR],
            [ASYNC ENGINE COUNT],
            [UNIFIED ADDRESSING],
            //[MAXIMUM TEXTURE1D LAYERED WIDTH],
            //[MAXIMUM TEXTURE1D LAYERED LAYERS],
            //[CAN TEX2D GATHER],
            //[MAXIMUM TEXTURE2D GATHER WIDTH],
            //[MAXIMUM TEXTURE2D GATHER HEIGHT],
            //[MAXIMUM TEXTURE3D WIDTH ALTERNATE],
            //[MAXIMUM TEXTURE3D HEIGHT ALTERNATE],
            //[MAXIMUM TEXTURE3D DEPTH ALTERNATE],
            [PCI DOMAIN ID],
            [TEXTURE PITCH ALIGNMENT],
            //[MAXIMUM TEXTURECUBEMAP WIDTH],
            //[MAXIMUM TEXTURECUBEMAP LAYERED WIDTH],
            //[MAXIMUM TEXTURECUBEMAP LAYERED LAYERS],
            //[MAXIMUM SURFACE1D WIDTH],
            //[MAXIMUM SURFACE2D WIDTH],
            //[MAXIMUM SURFACE2D HEIGHT],
            //[MAXIMUM SURFACE3D WIDTH],
            //[MAXIMUM SURFACE3D HEIGHT],
            //[MAXIMUM SURFACE3D DEPTH],
            //[MAXIMUM SURFACE1D LAYERED WIDTH],
            //[MAXIMUM SURFACE1D LAYERED LAYERS],
            //[MAXIMUM SURFACE2D LAYERED WIDTH],
            //[MAXIMUM SURFACE2D LAYERED HEIGHT],
            //[MAXIMUM SURFACE2D LAYERED LAYERS],
            //[MAXIMUM SURFACECUBEMAP WIDTH],
            //[MAXIMUM SURFACECUBEMAP LAYERED WIDTH],
            //[MAXIMUM SURFACECUBEMAP LAYERED LAYERS],
            //[MAXIMUM TEXTURE1D LINEAR WIDTH],
            //[MAXIMUM TEXTURE2D LINEAR WIDTH],
            //[MAXIMUM TEXTURE2D LINEAR HEIGHT],
            //[MAXIMUM TEXTURE2D LINEAR PITCH],
            //[MAXIMUM TEXTURE2D MIPMAPPED WIDTH],
            //[MAXIMUM TEXTURE2D MIPMAPPED HEIGHT],
            //[COMPUTE CAPABILITY MAJOR],
            //[COMPUTE CAPABILITY MINOR],
            //[MAXIMUM TEXTURE1D MIPMAPPED WIDTH],
            [STREAM PRIORITIES SUPPORTED],
            [GLOBAL L1 CACHE SUPPORTED],
            [LOCAL L1 CACHE SUPPORTED],
            [MAX SHARED MEMORY PER MULTIPROCESSOR],
            [MAX REGISTERS PER MULTIPROCESSOR],
            [MANAGED MEMORY],
            [MULTI GPU BOARD],
            [MULTI GPU BOARD GROUP ID],
            [HOST NATIVE ATOMIC SUPPORTED],
            // [SINGLE TO DOUBLE PRECISION PERF RATIO], // not supported by hipDeviceGetAttribute
            [PAGEABLE MEMORY ACCESS],
            [CONCURRENT MANAGED ACCESS],
            [COMPUTE PREEMPTION SUPPORTED],
            [CAN USE HOST POINTER FOR REGISTERED MEM],
            //[CAN USE STREAM MEM OPS],
            [COOPERATIVE LAUNCH],
            [COOPERATIVE MULTI DEVICE LAUNCH],
            [MAX SHARED MEMORY PER BLOCK OPTIN],
            //[CAN FLUSH REMOTE WRITES],
            [HOST REGISTER SUPPORTED],
            [PAGEABLE MEMORY ACCESS USES HOST PAGE TABLES],
            [DIRECT MANAGED MEM ACCESS FROM HOST],
            //[VIRTUAL ADDRESS MANAGEMENT SUPPORTED],
            [VIRTUAL MEMORY MANAGEMENT SUPPORTED],
            //[HANDLE TYPE POSIX FILE DESCRIPTOR SUPPORTED],
            //[HANDLE TYPE WIN32 HANDLE SUPPORTED],
            //[HANDLE TYPE WIN32 KMT HANDLE SUPPORTED],
            //[GENERIC COMPRESSION SUPPORTED],
            //[GPU DIRECT RDMA WITH CUDA VMM SUPPORTED],
            [MEMORY POOLS SUPPORTED],
            //[CAN USE 64 BIT STREAM MEM OPS],
            //[CAN USE STREAM WAIT VALUE NOR],
            //[DMA BUF SUPPORTED],
            //[MEM SYNC DOMAIN COUNT],
            //[TENSOR MAP ACCESS SUPPORTED],
            //[HANDLE TYPE FABRIC SUPPORTED],
            //[NUMA CONFIG],
            //[NUMA ID],
            //[MULTICAST SUPPORTED],
            //[MPS ENABLED],
            //[HOST NUMA ID],
        }, {
            MAX_BLOCKS_PER_MULTIPROCESSOR => MaxBlocksPerMultiProcessor,
            RESERVED_SHARED_MEMORY_PER_BLOCK => ReservedSharedMemPerBlock,
            MEMPOOL_SUPPORTED_HANDLE_TYPES => MemoryPoolSupportedHandleTypes,
        }
    };
    unsafe { hipDeviceGetAttribute(pi, attrib, dev_idx) }
}

pub(crate) fn get_uuid(uuid: *mut hipUUID, device: hipDevice_t) -> hipError_t {
    unsafe { hipDeviceGetUuid(uuid, device) }
}

pub(crate) fn get_uuid_v2(uuid: *mut hipUUID, device: hipDevice_t) -> hipError_t {
    get_uuid(uuid, device)
}

pub(crate) fn get_luid(
    luid: *mut ::core::ffi::c_char,
    device_node_mask: &mut ::core::ffi::c_uint,
    dev: hipDevice_t,
) -> hipError_t {
    let luid = unsafe {
        luid.cast::<[i8; 8]>()
            .as_mut()
            .ok_or(hipErrorCode_t::InvalidValue)
    }?;
    let mut properties = unsafe { mem::zeroed() };
    unsafe { hipGetDevicePropertiesR0600(&mut properties, dev) }?;
    *luid = properties.luid;
    *device_node_mask = properties.luidDeviceNodeMask;
    Ok(())
}

pub(crate) fn get_name(
    name: *mut ::core::ffi::c_char,
    len: ::core::ffi::c_int,
    dev: hipDevice_t,
) -> CUresult {
    unsafe { hipDeviceGetName(name, len, dev) }?;
    zluda_common::append_suffix(name, len as usize);
    Ok(())
}

pub(crate) fn total_mem_v2(bytes: *mut usize, dev: hipDevice_t) -> hipError_t {
    unsafe { hipDeviceTotalMem(bytes, dev) }
}

pub(crate) fn get_properties(prop: &mut CUdevprop, dev: hipDevice_t) -> hipError_t {
    let mut hip_props = unsafe { mem::zeroed() };
    unsafe { hipGetDevicePropertiesR0600(&mut hip_props, dev) }?;
    prop.maxThreadsPerBlock = hip_props.maxThreadsPerBlock;
    prop.maxThreadsDim = hip_props.maxThreadsDim;
    prop.maxGridSize = hip_props.maxGridSize;
    prop.totalConstantMemory = clamp_usize(hip_props.totalConstMem);
    prop.SIMDWidth = 32;
    prop.memPitch = clamp_usize(hip_props.memPitch);
    prop.regsPerBlock = hip_props.regsPerBlock;
    prop.clockRate = hip_props.clockRate;
    prop.textureAlign = clamp_usize(hip_props.textureAlignment);
    Ok(())
}

pub(crate) fn get_count(count: &mut ::core::ffi::c_int) -> hipError_t {
    unsafe { hipGetDeviceCount(count) }
}

fn clamp_usize(x: usize) -> i32 {
    usize::min(x, i32::MAX as usize) as i32
}

pub(crate) fn get_primary_context(
    hip_dev: hipDevice_t,
) -> Result<(&'static context::Context, CUcontext), CUerror> {
    let dev: &'static driver::Device = driver::device(hip_dev)?;
    Ok(dev.primary_context())
}

pub(crate) fn primary_context_set_flags(
    hip_dev: hipDevice_t,
    _flags: ::core::ffi::c_uint,
) -> CUresult {
    let (ctx, _) = get_primary_context(hip_dev)?;
    ctx.with_state_mut(|_state| {
        // Do nothing
        Ok(())
    })?;
    Ok(())
}

pub(crate) fn primary_context_set_flags_v2(
    hip_dev: hipDevice_t,
    flags: ::core::ffi::c_uint,
) -> CUresult {
    primary_context_set_flags(hip_dev, flags)
}

pub(crate) fn primary_context_retain(pctx: &mut CUcontext, hip_dev: hipDevice_t) -> CUresult {
    let (ctx, cu_ctx) = get_primary_context(hip_dev)?;

    ctx.with_state_mut(|state: &mut context::ContextState| {
        state.ref_count += 1;
        Ok(())
    })?;

    *pctx = cu_ctx;
    Ok(())
}

pub(crate) fn primary_context_release(hip_dev: hipDevice_t) -> CUresult {
    let (ctx, _) = get_primary_context(hip_dev)?;

    ctx.with_state_mut(|state| {
        state.ref_count -= 1;
        if state.ref_count == 0 {
            return state.reset();
        }
        Ok(())
    })?;
    Ok(())
}

pub(crate) fn primary_context_release_v2(hip_dev: hipDevice_t) -> CUresult {
    primary_context_release(hip_dev)
}

pub(crate) fn primary_context_reset(hip_dev: hipDevice_t) -> CUresult {
    let (ctx, _) = get_primary_context(hip_dev)?;
    ctx.with_state_mut(|state| state.reset())?;
    Ok(())
}

pub(crate) unsafe fn primary_context_get_state(
    dev: hipDevice_t,
    flags_out: &mut ::core::ffi::c_uint,
    active_out: &mut ::core::ffi::c_int,
) -> CUresult {
    let (ctx, _) = get_primary_context(dev)?;
    let mut flags = 0u32;
    let mut active = 0i32;
    ctx.with_state_mut(|state| {
        flags = state.flags;
        active = (state.ref_count > 0) as i32;
        Ok(())
    })?;
    *flags_out = flags;
    *active_out = active;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::tests::CudaApi;
    use cuda_macros::test_cuda;
    use std::{mem, ptr};

    #[test_cuda]
    unsafe fn primary_ctx_retain_does_not_make_it_active(api: impl CudaApi) {
        api.cuInit(0);
        let mut current_ctx = mem::zeroed();
        api.cuCtxGetCurrent(&mut current_ctx);
        assert_eq!(current_ctx.0, ptr::null_mut());
        let mut primary_ctx = mem::zeroed();
        api.cuDevicePrimaryCtxRetain(&mut primary_ctx, 0);
        assert_ne!(primary_ctx.0, ptr::null_mut());
        api.cuCtxGetCurrent(&mut current_ctx);
        assert_eq!(current_ctx.0, ptr::null_mut());
    }
}
