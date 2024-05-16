use super::context::{ContextInnerMutable, ContextVariant, PrimaryContextData};
use super::{
    context, LiveCheck, GLOBAL_STATE
};
use crate::r#impl::context::ContextData;
use crate::{r#impl::IntoCuda, hip_call_cuda};
use crate::hip_call;
use cuda_types::{CUdevice_attribute, CUdevprop, CUuuid_st, CUresult};
use hip_common::CompilationMode;
use hip_runtime_sys::*;
use paste::paste;
use std::{
    mem,
    os::raw::{c_char, c_uint},
    ptr,ffi::CString,
};

const ZLUDA_SUFFIX: &'static [u8] = b" [ZLUDA]\0";
// We report the highest non-existent compute capability mainly to fool Blender.
// Blender will look for known compute sapabilities and give them ELF.
// If the compute capability is unknown it gives them PTX
pub const COMPUTE_CAPABILITY_MAJOR: u32 = 8;
pub const COMPUTE_CAPABILITY_MINOR: u32 = 8;


pub(crate) struct Device {
    pub(crate) compilation_mode: CompilationMode,
    pub(crate) comgr_isa: CString,
    primary_context: context::Context,
}

impl Device {
    pub(crate) fn new(index: usize) -> Result<Self, CUresult> {
        let comgr_isa = unsafe { hip_common::comgr_isa(index as i32) }.map_err(hipError_t::into_cuda)?;
        let mut warp_size = 0i32;
        hip_call_cuda!{ hipDeviceGetAttribute(&mut warp_size, hipDeviceAttribute_t::hipDeviceAttributeWarpSize, index as i32) };
        let compilation_mode = if warp_size == 32 {
            CompilationMode::Wave32
        } else if warp_size == 64 {
            get_wave64_mode()
        } else {
            return Err(CUresult::CUDA_ERROR_ILLEGAL_STATE);
        };
        Ok(Self {
            compilation_mode,
            comgr_isa,
            primary_context: LiveCheck::new(ContextData::new_primary(index as i32)),
        })
    }
}

fn get_wave64_mode() -> CompilationMode {
    match std::env::var("ZLUDA_WAVE64_SLOW_MODE") {
        Ok(value) => {
            if let Ok(value) = str::parse::<u32>(&value) {
                if value != 0 {
                    return CompilationMode::Wave32OnWave64;
                }
            }
        }
        Err(_) => {}
    }
    CompilationMode::DoubleWave32OnWave64
}

#[allow(warnings)]
trait hipDeviceAttribute_t_ext {
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
}

impl hipDeviceAttribute_t_ext for hipDeviceAttribute_t {}

macro_rules! remap_attribute {
    ($attrib:expr => $([ $($word:expr)* ]),*,) => {
        match $attrib {
            $(
                paste! { CUdevice_attribute:: [< CU_DEVICE_ATTRIBUTE $(_ $word:upper)* >] } => {
                    paste! { hipDeviceAttribute_t:: [< hipDeviceAttribute $($word:camel)* >] }
                }
            )*
            _ => return Err(CUresult::CUDA_ERROR_INVALID_VALUE)
        }
    }
}

pub(crate) unsafe fn get_attribute(
    pi: *mut i32,
    attrib: CUdevice_attribute,
    dev: hipDevice_t,
) -> Result<(), CUresult> {
    if pi == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    let hip_attrib = match attrib {
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_WARP_SIZE => {
            *pi = 32;
            return Ok(());
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_ASYNC_ENGINE_COUNT => {
            *pi = 1;
            return Ok(());
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_GPU_OVERLAP
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_UNIFIED_ADDRESSING
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_STREAM_PRIORITIES_SUPPORTED
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_GLOBAL_L1_CACHE_SUPPORTED
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_LOCAL_L1_CACHE_SUPPORTED 
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_COMPUTE_PREEMPTION_SUPPORTED=> {
            *pi = 1;
            return Ok(());
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_TCC_DRIVER
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_GATHER_WIDTH
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_GATHER_HEIGHT
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_WIDTH_ALTERNATE
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_HEIGHT_ALTERNATE
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_DEPTH_ALTERNATE
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURECUBEMAP_WIDTH
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURECUBEMAP_LAYERED_WIDTH
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURECUBEMAP_LAYERED_LAYERS
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACECUBEMAP_WIDTH
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACECUBEMAP_LAYERED_WIDTH
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACECUBEMAP_LAYERED_LAYERS
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_WIDTH
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_HEIGHT
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_PITCH
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_PERSISTING_L2_CACHE_SIZE
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MULTI_GPU_BOARD_GROUP_ID
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_HOST_NATIVE_ATOMIC_SUPPORTED
        // possibly true for integrated GPUs
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_CAN_USE_HOST_POINTER_FOR_REGISTERED_MEM
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_ACCESS_POLICY_WINDOW_SIZE
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_RESERVED_SHARED_MEMORY_PER_BLOCK
        // Possibly true
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_HOST_REGISTER_SUPPORTED
        // Possibly true
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_READ_ONLY_HOST_REGISTER_SUPPORTED
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_SPARSE_CUDA_ARRAY_SUPPORTED
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_TIMELINE_SEMAPHORE_INTEROP_SUPPORTED
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MEMORY_POOLS_SUPPORTED
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_GPU_DIRECT_RDMA_SUPPORTED
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_GPU_DIRECT_RDMA_FLUSH_WRITES_OPTIONS
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_GPU_DIRECT_RDMA_WRITES_ORDERING
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MEMPOOL_SUPPORTED_HANDLE_TYPES
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_DEFERRED_MAPPING_CUDA_ARRAY_SUPPORTED
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_IPC_EVENT_SUPPORTED
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_CLUSTER_LAUNCH
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_UNIFIED_FUNCTION_POINTERS
        // Possibly true, used by llama.cpp
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_VIRTUAL_MEMORY_MANAGEMENT_SUPPORTED => {
            *pi = 0;
            return Ok(());
        }
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_SINGLE_TO_DOUBLE_PRECISION_PERF_RATIO => {
            // true for most navi1 and navi2 cards
            *pi = 16;
            return Ok(());
        }
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_BLOCKS_PER_MULTIPROCESSOR => {
            // in practical terms max group size = max blocks * warp size
            let mut prop = mem::zeroed();
            hip_call_cuda! { hipGetDeviceProperties(&mut prop, dev) };
            *pi = (prop.maxThreadsPerBlock / 2) / prop.warpSize;
            return Ok(());
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR => {
            compute_capability(pi, &mut 0i32, dev);
            return Ok(());
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR => {
            compute_capability(&mut 0i32, pi, dev);
            return Ok(());
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_REGISTERS_PER_MULTIPROCESSOR => {
            // My 1060 returns same for CU_DEVICE_ATTRIBUTE_MAX_REGISTERS_PER_MULTIPROCESSOR and
            // CU_DEVICE_ATTRIBUTE_MAX_REGISTERS_PER_BLOCK, not sure what is the difference
            hipDeviceAttribute_t::hipDeviceAttributeMaxRegistersPerBlock
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_BLOCK_OPTIN => {
            hipDeviceAttribute_t::hipDeviceAttributeMaxSharedMemoryPerBlock
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MULTI_GPU_BOARD => {
            hipDeviceAttribute_t::hipDeviceAttributeIsMultiGpuBoard
        }
        // we assume that arrayed texts have the same limits
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_ARRAY_WIDTH => {
            hipDeviceAttribute_t::hipDeviceAttributeMaxTexture2DWidth
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_ARRAY_HEIGHT => {
            hipDeviceAttribute_t::hipDeviceAttributeMaxTexture2DHeight
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LAYERED_WIDTH => {
            hipDeviceAttribute_t::hipDeviceAttributeMaxTexture1DWidth
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE1D_LAYERED_WIDTH => {
            hipDeviceAttribute_t::hipDeviceAttributeMaxTexture1DWidth
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_LAYERED_WIDTH => {
            hipDeviceAttribute_t::hipDeviceAttributeMaxTexture2DWidth
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_LAYERED_HEIGHT => {
            hipDeviceAttribute_t::hipDeviceAttributeMaxTexture2DHeight
        }
        // we treat surface the same as texture
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_SURFACE_ALIGNMENT => {
            hipDeviceAttribute_t::hipDeviceAttributeTextureAlignment
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE1D_WIDTH => {
            hipDeviceAttribute_t::hipDeviceAttributeMaxTexture1DWidth
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_WIDTH => {
            hipDeviceAttribute_t::hipDeviceAttributeMaxTexture2DWidth
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_HEIGHT => {
            hipDeviceAttribute_t::hipDeviceAttributeMaxTexture2DHeight
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE3D_WIDTH => {
            hipDeviceAttribute_t::hipDeviceAttributeMaxTexture3DWidth
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE3D_HEIGHT => {
            hipDeviceAttribute_t::hipDeviceAttributeMaxTexture3DHeight
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE3D_DEPTH => {
            hipDeviceAttribute_t::hipDeviceAttributeMaxTexture3DDepth
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_MIPMAPPED_WIDTH => {
            hipDeviceAttribute_t::hipDeviceAttributeMaxTexture2DWidth
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_MIPMAPPED_HEIGHT => {
            hipDeviceAttribute_t::hipDeviceAttributeMaxTexture2DHeight
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_MIPMAPPED_WIDTH => {
            hipDeviceAttribute_t::hipDeviceAttributeMaxTexture1DWidth
        }
        // Totally made up
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_ARRAY_NUMSLICES
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LAYERED_LAYERS
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE1D_LAYERED_LAYERS
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_LAYERED_LAYERS => {
            *pi = u16::MAX as i32;
            return Ok(());
        }
        // linear sizes
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LINEAR_WIDTH => {
            let mut prop = mem::zeroed();
            hip_call_cuda! { hipGetDeviceProperties(&mut prop, dev) };
            *pi = prop.maxTexture1DLinear;
            return Ok(());
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_PCI_DOMAIN_ID => {
            let mut prop = mem::zeroed();
            hip_call_cuda! { hipGetDeviceProperties(&mut prop, dev) };
            *pi = prop.pciDomainID;
            return Ok(());
        }
        attrib @
        (CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_BLOCK
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_X
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_Y
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_Z
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_X
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_Y
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_Z) => {
            let attrib = remap_attribute! {
                attrib =>
                [MAX THREADS PER BLOCK],
                [MAX BLOCK DIM X],
                [MAX BLOCK DIM Y],
                [MAX BLOCK DIM Z],
                [MAX GRID DIM X],
                [MAX GRID DIM Y],
                [MAX GRID DIM Z],
            };
            hip_call_cuda! { hipDeviceGetAttribute(pi, attrib, dev) };
            let dev = GLOBAL_STATE.get()?.device(dev)?;
            if dev.compilation_mode == CompilationMode::Wave32OnWave64 {
                *pi /= 2;
            }
            return Ok(())
        }
        attrib => remap_attribute! {
            attrib =>
            [MAX SHARED MEMORY PER BLOCK],
            [TOTAL CONSTANT MEMORY],
            [WARP SIZE],
            [MAX PITCH],
            [MAX REGISTERS PER BLOCK],
            [CLOCK RATE],
            [TEXTURE ALIGNMENT],
            //[GPU OVERLAP],
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
            //[SURFACE ALIGNMENT],
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
            //[UNIFIED ADDRESSING],
            //[MAXIMUM TEXTURE1D LAYERED WIDTH],
            //[MAXIMUM TEXTURE1D LAYERED LAYERS],
            //[CAN TEX2D GATHER],
            //[MAXIMUM TEXTURE2D GATHER WIDTH],
            //[MAXIMUM TEXTURE2D GATHER HEIGHT],
            //[MAXIMUM TEXTURE3D WIDTH ALTERNATE],
            //[MAXIMUM TEXTURE3D HEIGHT ALTERNATE],
            //[MAXIMUM TEXTURE3D DEPTH ALTERNATE],
            //[PCI DOMAIN ID],
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
            //[STREAM PRIORITIES SUPPORTED],
            //[GLOBAL L1 CACHE SUPPORTED],
            //[LOCAL L1 CACHE SUPPORTED],
            [MAX SHARED MEMORY PER MULTIPROCESSOR],
            //[MAX REGISTERS PER MULTIPROCESSOR],
            [MANAGED MEMORY],
            //[MULTI GPU BOARD],
            //[MULTI GPU BOARD GROUP ID],
            //[HOST NATIVE ATOMIC SUPPORTED],
            [SINGLE TO DOUBLE PRECISION PERF RATIO],
            [PAGEABLE MEMORY ACCESS],
            [CONCURRENT MANAGED ACCESS],
            //[COMPUTE PREEMPTION SUPPORTED],
            //[CAN USE HOST POINTER FOR REGISTERED MEM],
            //[CAN USE STREAM MEM OPS],
            //[CAN USE 64 BIT STREAM MEM OPS],
            //[CAN USE STREAM WAIT VALUE NOR],
            [COOPERATIVE LAUNCH],
            [COOPERATIVE MULTI DEVICE LAUNCH],
            //[MAX SHARED MEMORY PER BLOCK OPTIN],
            //[CAN FLUSH REMOTE WRITES],
            //[HOST REGISTER SUPPORTED],
            [PAGEABLE MEMORY ACCESS USES HOST PAGE TABLES],
            [DIRECT MANAGED MEM ACCESS FROM HOST],
            //[VIRTUAL ADDRESS MANAGEMENT SUPPORTED],
            //[VIRTUAL MEMORY MANAGEMENT SUPPORTED],
            //[HANDLE TYPE POSIX FILE DESCRIPTOR SUPPORTED],
            //[HANDLE TYPE WIN32 HANDLE SUPPORTED],
            //[HANDLE TYPE WIN32 KMT HANDLE SUPPORTED],
            //[MAX BLOCKS PER MULTIPROCESSOR],
            //[GENERIC COMPRESSION SUPPORTED],
            //[MAX ACCESS POLICY WINDOW SIZE],
            //[GPU DIRECT RDMA WITH CUDA VMM SUPPORTED],
            //[RESERVED SHARED MEMORY PER BLOCK],
            //[SPARSE CUDA ARRAY SUPPORTED],
            //[READ ONLY HOST REGISTER SUPPORTED],
            //[TIMELINE SEMAPHORE INTEROP SUPPORTED],
            //[MEMORY POOLS SUPPORTED],
        },
    };
    let error = hipDeviceGetAttribute(pi, hip_attrib, dev);
    // For properties:
    // * CU_DEVICE_ATTRIBUTE_TOTAL_CONSTANT_MEMORY
    // * CU_DEVICE_ATTRIBUTE_MAX_PITCH
    // HIP returns negative numbers (overflows)
    if error == hipError_t::hipSuccess {
        if *pi < 0 {
            *pi = i32::MAX;
        }
        Ok(())
    } else {
        Err(error.into_cuda())
    }
    
}

// TODO
pub(crate) fn get_uuid(uuid: *mut CUuuid_st, _dev: hipDevice_t) -> CUresult {
    unsafe {
        *uuid = CUuuid_st {
            bytes: mem::zeroed(),
        }
    };
    CUresult::CUDA_SUCCESS
}

// TODO
pub(crate) fn get_luid(
    luid: *mut c_char,
    dev_node_mask: *mut c_uint,
    _dev: hipDevice_t,
) -> CUresult {
    unsafe { ptr::write_bytes(luid, 0u8, 8) };
    unsafe { *dev_node_mask = 0 };
    CUresult::CUDA_SUCCESS
}

pub(crate) unsafe fn get_properties(
    prop: *mut CUdevprop,
    dev: hipDevice_t,
) -> Result<(), CUresult> {
    if prop == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    let mut hip_props = mem::zeroed();
    hip_call_cuda! { hipGetDeviceProperties(&mut hip_props, dev) };
    (*prop).maxThreadsPerBlock = hip_props.maxThreadsPerBlock;
    (*prop).maxThreadsDim = hip_props.maxThreadsDim;
    (*prop).maxGridSize = hip_props.maxGridSize;
    (*prop).totalConstantMemory = usize::min(hip_props.totalConstMem, i32::MAX as usize) as i32;
    (*prop).SIMDWidth = hip_props.warpSize;
    (*prop).memPitch = usize::min(hip_props.memPitch, i32::MAX as usize) as i32;
    (*prop).regsPerBlock = hip_props.regsPerBlock;
    (*prop).clockRate = hip_props.clockRate;
    (*prop).textureAlign = usize::min(hip_props.textureAlignment, i32::MAX as usize) as i32;
    let dev = GLOBAL_STATE.get()?.device(dev)?;
    if dev.compilation_mode == CompilationMode::Wave32OnWave64 {
        (*prop).maxThreadsPerBlock /= 2;
        (*prop).maxThreadsDim[0] /= 2;
        (*prop).maxThreadsDim[1] /= 2;
        (*prop).maxThreadsDim[2] /= 2;
        (*prop).maxGridSize[0] /= 2;
        (*prop).maxGridSize[1] /= 2;
        (*prop).maxGridSize[2] /= 2;
    }
    Ok(())
}

pub(crate) unsafe fn compute_capability(
    major: *mut ::std::os::raw::c_int,
    minor: *mut ::std::os::raw::c_int,
    _dev: hipDevice_t,
) {
    *major = COMPUTE_CAPABILITY_MAJOR as i32;
    *minor = COMPUTE_CAPABILITY_MINOR as i32;
}

pub(crate) unsafe fn total_mem(bytes: *mut u32, dev: hipDevice_t) -> Result<(), hipError_t> {
    let mut bytes_usize = 0;
    hip_call!(hipDeviceTotalMem(&mut bytes_usize, dev));
    *bytes = usize::min(bytes_usize, u32::MAX as usize) as u32;
    Ok(())
}

pub(crate) unsafe fn primary_ctx_get(
    pctx: *mut *mut context::Context,
    hip_dev: hipDevice_t,
) -> Result<(), CUresult> {
    primary_ctx_get_or_retain(pctx, hip_dev, false)
}

pub(crate) unsafe fn primary_ctx_retain(
    pctx: *mut *mut context::Context,
    hip_dev: hipDevice_t,
) -> Result<(), CUresult> {
    primary_ctx_get_or_retain(pctx, hip_dev, true)
}

unsafe fn primary_ctx_get_or_retain(
    pctx: *mut *mut context::Context,
    hip_dev: hipDevice_t,
    increment_refcount: bool
) -> Result<(), CUresult> {
    if pctx == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    let ctx = primary_ctx(hip_dev, |ctx, raw_ctx| {
        if increment_refcount || ctx.ref_count == 0  {
            ctx.ref_count += 1;
        }
        Ok(raw_ctx.cast_mut())
    })??;
    *pctx = ctx;
    Ok(())
}

pub(crate) unsafe fn primary_ctx_release(hip_dev: hipDevice_t) -> Result<(), CUresult> {
    primary_ctx(hip_dev, |ctx, _| {
        if ctx.ref_count == 0 {
            return Err(CUresult::CUDA_ERROR_INVALID_CONTEXT);
        }
        ctx.ref_count -= 1;
        if ctx.ref_count == 0 {
            // Even if we encounter errors we can't really surface them
            ctx.mutable.drop_with_result().ok();
            ctx.mutable = ContextInnerMutable::new();
            ctx.flags = 0;
        }
        Ok(())
    })?
}

pub(crate) unsafe fn primary_ctx_reset(_hip_dev: hipDevice_t) -> Result<(), CUresult> {
    Ok(())
    //TODO: fix
    /*
    let maybe_ctx = primary_ctx(hip_dev, Option::take)?;
    maybe_ctx
        .map(|mut ctx| ctx.try_drop(false))
        .unwrap_or(Err(CUresult::CUDA_ERROR_INVALID_CONTEXT))
         */
}

pub(crate) unsafe fn primary_ctx_set_flags(
    hip_dev: hipDevice_t,
    flags: ::std::os::raw::c_uint,
) -> Result<(), CUresult> {
    primary_ctx(hip_dev, |ctx, _| {
        ctx.flags = flags;
        // TODO: actually use flags
        Ok(())
    })?
}

pub(crate) unsafe fn primary_ctx_get_state(
    hip_dev: hipDevice_t,
    flags_ptr: *mut u32,
    active_ptr: *mut i32,
) -> Result<(), CUresult> {
    if flags_ptr == ptr::null_mut() || active_ptr == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    let (flags, active) = primary_ctx(hip_dev, |ctx, _| {
        (ctx.flags, (ctx.ref_count > 0) as i32)
    })?;
    *flags_ptr = flags;
    *active_ptr = active;
    Ok(())
}

pub(crate) unsafe fn primary_ctx<T>(
    dev: hipDevice_t,
    fn_: impl FnOnce(&mut PrimaryContextData, *const LiveCheck<ContextData>) -> T,
) -> Result<T, CUresult> {
    let device = GLOBAL_STATE.get()?.device(dev)?;
    let raw_ptr = &device.primary_context as *const _;
    let context = device.primary_context.as_ref_unchecked();
    match context.variant {
        ContextVariant::Primary(ref mutex_over_primary_ctx) => {
            let mut primary_ctx = mutex_over_primary_ctx.lock().map_err(|_| CUresult::CUDA_ERROR_UNKNOWN)?;
            Ok(fn_(&mut primary_ctx, raw_ptr))
        },
        ContextVariant::NonPrimary(..) => Err(CUresult::CUDA_ERROR_UNKNOWN)
    }
}

pub(crate) unsafe fn get_name(name: *mut i8, len: i32, device: i32) -> hipError_t {
    let result= hipDeviceGetName(name, len, device);
    if result != hipError_t::hipSuccess {
        return result;
    }
    append_zluda_suffix(name, len);
    hipError_t::hipSuccess
}

unsafe fn append_zluda_suffix(name: *mut i8, len: i32) {
    let len = len as usize;
    let str_len = (0..len).position(|i| unsafe { *name.add(i) == 0 } ).unwrap();
    if (str_len + ZLUDA_SUFFIX.len()) > len {
        return;
    }
    ptr::copy_nonoverlapping(ZLUDA_SUFFIX.as_ptr() as _,name.add(str_len),  ZLUDA_SUFFIX.len());
}


#[cfg(test)]
mod tests {
    use super::append_zluda_suffix;

    #[test]
    fn append_name_too_short() {
        let mut input = b"gfx-1030\0\n\n\n\n\n\n\n".to_vec();
        unsafe { append_zluda_suffix(input.as_mut_ptr() as _, input.len() as i32) };
        assert_eq!(input, b"gfx-1030\0\n\n\n\n\n\n\n");
    }

    #[test]
    fn append_name_equal() {
        let mut input = b"gfx-1030\0\n\n\n\n\n\n\n\n".to_vec();
        unsafe { append_zluda_suffix(input.as_mut_ptr() as _, input.len() as i32) };
        assert_eq!(input, b"gfx-1030 [ZLUDA]\0");
    }

    #[test]
    fn append_name_long() {
        let mut input = b"gfx-1030\0\n\n\n\n\n\n\n\n\n\n".to_vec();
        unsafe { append_zluda_suffix(input.as_mut_ptr() as _, input.len() as i32) };
        assert_eq!(input, b"gfx-1030 [ZLUDA]\0\n\n");
    }
}
