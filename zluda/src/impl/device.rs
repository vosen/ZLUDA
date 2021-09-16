use super::{transmute_lifetime, transmute_lifetime_mut, CUresult};
use crate::{
    cuda::{self, CUdevice, CUdevprop},
    hip_call,
};
use cuda::{CUdevice_attribute, CUuuid_st};
use hip_runtime_sys::{
    hipDeviceAttribute_t, hipDeviceGetAttribute, hipError_t, hipGetDeviceProperties,
};
use ocl_core::{ClDeviceIdPtr, ContextProperties, DeviceType};
use paste::paste;
use std::{
    cmp,
    collections::HashSet,
    ffi::c_void,
    mem,
    os::raw::{c_char, c_int, c_uint},
    ptr,
    sync::atomic::{AtomicU32, Ordering},
};

const PROJECT_URL_SUFFIX_SHORT: &'static str = " [ZLUDA]";
const PROJECT_URL_SUFFIX_LONG: &'static str = " [github.com/vosen/ZLUDA]";

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
            _ => return hipError_t::hipErrorInvalidValue
        }
    }
}

pub fn get_attribute(pi: *mut i32, attrib: CUdevice_attribute, dev_idx: c_int) -> hipError_t {
    if pi == ptr::null_mut() {
        return hipError_t::hipErrorInvalidValue;
    }
    //let mut props = unsafe { mem::zeroed() };
    let hip_attrib = match attrib {
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_ASYNC_ENGINE_COUNT => {
            unsafe { *pi = 1 };
            return hipError_t::hipSuccess;
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_GPU_OVERLAP
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_UNIFIED_ADDRESSING
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_STREAM_PRIORITIES_SUPPORTED
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_GLOBAL_L1_CACHE_SUPPORTED
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_LOCAL_L1_CACHE_SUPPORTED => {
            unsafe { *pi = 1 };
            return hipError_t::hipSuccess;
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
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_REGISTERS_PER_MULTIPROCESSOR
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MULTI_GPU_BOARD
        | CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MULTI_GPU_BOARD_GROUP_ID => {
            unsafe { *pi = 0 };
            return hipError_t::hipSuccess;
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR => {
            unsafe { *pi = 8 };
            return hipError_t::hipSuccess;
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR => {
            unsafe { *pi = 0 };
            return hipError_t::hipSuccess;
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
            unsafe { *pi = u16::MAX as i32 };
            return hipError_t::hipSuccess;
        }
        // linear sizes
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LINEAR_WIDTH => {
            let mut prop = unsafe { mem::zeroed() };
            let err = unsafe { hipGetDeviceProperties(&mut prop, dev_idx) };
            if err != hipError_t::hipSuccess {
                return err;
            }
            unsafe { *pi = prop.maxTexture1DLinear };
            return hipError_t::hipSuccess;
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_PCI_DOMAIN_ID => {
            let mut prop = unsafe { mem::zeroed() };
            let err = unsafe { hipGetDeviceProperties(&mut prop, dev_idx) };
            if err != hipError_t::hipSuccess {
                return err;
            }
            unsafe { *pi = prop.pciDomainID };
            return hipError_t::hipSuccess;
        }
        attrib => remap_attribute! {
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
            //[SINGLE TO DOUBLE PRECISION PERF RATIO],
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
            //[MAX PERSISTING L2 CACHE SIZE],
            //[MAX ACCESS POLICY WINDOW SIZE],
            //[GPU DIRECT RDMA WITH CUDA VMM SUPPORTED],
            //[RESERVED SHARED MEMORY PER BLOCK],
            //[SPARSE CUDA ARRAY SUPPORTED],
            //[READ ONLY HOST REGISTER SUPPORTED],
            //[TIMELINE SEMAPHORE INTEROP SUPPORTED],
            //[MEMORY POOLS SUPPORTED],
        },
    };
    unsafe { hipDeviceGetAttribute(pi, hip_attrib, dev_idx) }
}

pub fn get_uuid(uuid: *mut CUuuid_st, _dev_idx: c_int) -> Result<(), CUresult> {
    unsafe {
        *uuid = CUuuid_st {
            bytes: mem::zeroed(),
        }
    };
    Ok(())
}

// TODO: add support if Level 0 exposes it
pub fn get_luid(
    luid: *mut c_char,
    dev_node_mask: *mut c_uint,
    _dev_idx: c_int,
) -> Result<(), CUresult> {
    unsafe { ptr::write_bytes(luid, 0u8, 8) };
    unsafe { *dev_node_mask = 0 };
    Ok(())
}

pub(crate) unsafe fn get_properties(prop: *mut CUdevprop, dev: CUdevice) -> Result<(), hipError_t> {
    if prop == ptr::null_mut() {
        return Err(hipError_t::hipErrorInvalidValue);
    }
    let mut hip_props = mem::zeroed();
    hip_call! { hipGetDeviceProperties(&mut hip_props, dev.0) };
    (*prop).maxThreadsPerBlock = hip_props.maxThreadsPerBlock;
    (*prop).maxThreadsDim = hip_props.maxThreadsDim;
    (*prop).maxGridSize = hip_props.maxGridSize;
    (*prop).totalConstantMemory = usize::min(hip_props.totalConstMem, i32::MAX as usize) as i32;
    (*prop).SIMDWidth = hip_props.warpSize;
    (*prop).memPitch = usize::min(hip_props.memPitch, i32::MAX as usize) as i32;
    (*prop).regsPerBlock = hip_props.regsPerBlock;
    (*prop).clockRate = hip_props.clockRate;
    (*prop).textureAlign = usize::min(hip_props.textureAlignment, i32::MAX as usize) as i32;
    Ok(())
}
