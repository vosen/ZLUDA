use cuda_types::cuda::*;
use hip_runtime_sys::*;
use std::{mem, ptr};

use super::context;

const PROJECT_SUFFIX: &[u8] = b" [ZLUDA]\0";
pub const COMPUTE_CAPABILITY_MAJOR: i32 = 8;
pub const COMPUTE_CAPABILITY_MINOR: i32 = 8;

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
    ($attrib:expr => $([ $($word:expr)* ]),*,) => {
        match $attrib {
            $(
                paste::paste! { CUdevice_attribute:: [< CU_DEVICE_ATTRIBUTE $(_ $word:upper)* >] } => {
                    paste::paste! { hipDeviceAttribute_t:: [< hipDeviceAttribute $($word:camel)* >] }
                }
            )*
            _ => return Err(hipErrorCode_t::NotSupported)
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
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_TCC_DRIVER => {
            *pi = 0;
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
        _ => {}
    }
    let attrib = remap_attribute! {
        attrib =>
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
        [SINGLE TO DOUBLE PRECISION PERF RATIO],
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
        //[MAX BLOCKS PER MULTIPROCESSOR],
        //[GENERIC COMPRESSION SUPPORTED],
        //[MAX PERSISTING L2 CACHE SIZE],
        //[MAX ACCESS POLICY WINDOW SIZE],
        //[GPU DIRECT RDMA WITH CUDA VMM SUPPORTED],
        //[RESERVED SHARED MEMORY PER BLOCK],
        //[SPARSE CUDA ARRAY SUPPORTED],
        //[READ ONLY HOST REGISTER SUPPORTED],
        //[TIMELINE SEMAPHORE INTEROP SUPPORTED],
        [MEMORY POOLS SUPPORTED],
        //[GPU DIRECT RDMA SUPPORTED],
        //[GPU DIRECT RDMA FLUSH WRITES OPTIONS],
        //[GPU DIRECT RDMA WRITES ORDERING],
        //[MEMPOOL SUPPORTED HANDLE TYPES],
        //[CLUSTER LAUNCH],
        //[DEFERRED MAPPING CUDA ARRAY SUPPORTED],
        //[CAN USE 64 BIT STREAM MEM OPS],
        //[CAN USE STREAM WAIT VALUE NOR],
        //[DMA BUF SUPPORTED],
        //[IPC EVENT SUPPORTED],
        //[MEM SYNC DOMAIN COUNT],
        //[TENSOR MAP ACCESS SUPPORTED],
        //[HANDLE TYPE FABRIC SUPPORTED],
        //[UNIFIED FUNCTION POINTERS],
        //[NUMA CONFIG],
        //[NUMA ID],
        //[MULTICAST SUPPORTED],
        //[MPS ENABLED],
        //[HOST NUMA ID],
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
    let len = len as usize;
    let buffer = unsafe { std::slice::from_raw_parts(name, len) };
    let first_zero = buffer.iter().position(|c| *c == 0);
    let first_zero = if let Some(x) = first_zero {
        x
    } else {
        return Ok(());
    };
    if (first_zero + PROJECT_SUFFIX.len()) > len {
        return Ok(());
    }
    unsafe {
        ptr::copy_nonoverlapping(
            PROJECT_SUFFIX.as_ptr() as _,
            name.add(first_zero),
            PROJECT_SUFFIX.len(),
        )
    };
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

pub(crate) fn primary_context_retain(
    pctx: &mut CUcontext,
    hip_dev: hipDevice_t,
) -> Result<(), CUerror> {
    let (ctx, raw_ctx) = context::get_primary(hip_dev)?;
    {
        let mut mutable_ctx = ctx.mutable.lock().map_err(|_| CUerror::UNKNOWN)?;
        mutable_ctx.ref_count += 1;
    }
    *pctx = raw_ctx;
    Ok(())
}

pub(crate) fn primary_context_release(hip_dev: hipDevice_t) -> Result<(), CUerror> {
    let (ctx, _) = context::get_primary(hip_dev)?;
    {
        let mut mutable_ctx = ctx.mutable.lock().map_err(|_| CUerror::UNKNOWN)?;
        if mutable_ctx.ref_count == 0 {
            return Err(CUerror::INVALID_CONTEXT);
        }
        mutable_ctx.ref_count -= 1;
        if mutable_ctx.ref_count == 0 {
            // TODO: drop all children
        }
    }
    Ok(())
}
