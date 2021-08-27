use super::{context, transmute_lifetime, transmute_lifetime_mut, CUresult, GlobalState};
use crate::cuda;
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

#[repr(transparent)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Index(pub c_int);

pub struct Device {
    pub index: Index,
    pub ocl_base: ocl_core::DeviceId,
    pub default_queue: ocl_core::CommandQueue,
    pub ocl_context: ocl_core::Context,
    pub primary_context: context::Context,
    pub allocations: HashSet<*mut c_void>,
    pub is_amd: bool,
    pub name: String,
}

unsafe impl Send for Device {}

impl Device {
    pub fn new(
        platform: ocl_core::PlatformId,
        ocl_dev: ocl_core::DeviceId,
        idx: usize,
        is_amd: bool,
    ) -> Result<Self, CUresult> {
        let mut props = ocl_core::ContextProperties::new();
        props.set_platform(platform);
        let ctx = ocl_core::create_context(Some(&props), &[ocl_dev], None, None)?;
        let queue = ocl_core::create_command_queue(&ctx, ocl_dev, None)?;
        let primary_context =
            context::Context::new(context::ContextData::new(0, true, ptr::null_mut())?);
        let props = ocl_core::get_device_info(ocl_dev, ocl_core::DeviceInfo::Name)?;
        let name = if let ocl_core::DeviceInfoResult::Name(name) = props {
            Ok(name)
        } else {
            Err(CUresult::CUDA_ERROR_UNKNOWN)
        }?;
        Ok(Self {
            index: Index(idx as c_int),
            ocl_base: ocl_dev,
            default_queue: queue,
            ocl_context: ctx,
            primary_context,
            allocations: HashSet::new(),
            is_amd,
            name,
        })
    }

    pub fn late_init(&mut self) {
        self.primary_context.as_option_mut().unwrap().device = self as *mut _;
    }
}

pub fn get_count(count: *mut c_int) -> Result<(), CUresult> {
    let len = GlobalState::lock(|state| state.devices.len())?;
    unsafe { *count = len as c_int };
    Ok(())
}

pub fn get(device: *mut Index, ordinal: c_int) -> Result<(), CUresult> {
    if device == ptr::null_mut() || ordinal < 0 {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    let len = GlobalState::lock(|state| state.devices.len())?;
    if ordinal < (len as i32) {
        unsafe { *device = Index(ordinal) };
        Ok(())
    } else {
        Err(CUresult::CUDA_ERROR_INVALID_VALUE)
    }
}

pub fn get_name(name: *mut c_char, len: i32, dev_idx: Index) -> Result<(), CUresult> {
    if name == ptr::null_mut() || len < 0 {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    let name_string = GlobalState::lock_device(dev_idx, |dev| dev.name.clone())?;
    let mut dst_null_pos = cmp::min((len - 1) as usize, name_string.len());
    unsafe { std::ptr::copy_nonoverlapping(name_string.as_ptr() as *const _, name, dst_null_pos) };
    if name_string.len() + PROJECT_URL_SUFFIX_LONG.len() < (len as usize) {
        unsafe {
            std::ptr::copy_nonoverlapping(
                PROJECT_URL_SUFFIX_LONG.as_ptr(),
                name.add(name_string.len()) as *mut _,
                PROJECT_URL_SUFFIX_LONG.len(),
            )
        };
        dst_null_pos += PROJECT_URL_SUFFIX_LONG.len();
    } else if name_string.len() + PROJECT_URL_SUFFIX_SHORT.len() < (len as usize) {
        unsafe {
            std::ptr::copy_nonoverlapping(
                PROJECT_URL_SUFFIX_SHORT.as_ptr(),
                name.add(name_string.len()) as *mut _,
                PROJECT_URL_SUFFIX_SHORT.len(),
            )
        };
        dst_null_pos += PROJECT_URL_SUFFIX_SHORT.len();
    }
    unsafe { *(name.add(dst_null_pos)) = 0 };
    Ok(())
}

pub fn total_mem_v2(bytes: *mut usize, dev_idx: Index) -> Result<(), CUresult> {
    if bytes == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    let mem_size = GlobalState::lock_device(dev_idx, |dev| {
        let props = ocl_core::get_device_info(dev.ocl_base, ocl_core::DeviceInfo::GlobalMemSize)?;
        if let ocl_core::DeviceInfoResult::GlobalMemSize(mem_size) = props {
            Ok(mem_size)
        } else {
            Err(CUresult::CUDA_ERROR_UNKNOWN)
        }
    })??;
    unsafe { *bytes = mem_size as usize };
    Ok(())
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

pub fn get_uuid(uuid: *mut CUuuid_st, _: Index) -> Result<(), CUresult> {
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
    _dev_idx: Index,
) -> Result<(), CUresult> {
    unsafe { ptr::write_bytes(luid, 0u8, 8) };
    unsafe { *dev_node_mask = 0 };
    Ok(())
}

pub fn primary_ctx_get_state(
    dev_idx: Index,
    flags: *mut u32,
    active: *mut i32,
) -> Result<(), CUresult> {
    let (is_active, flags_value) = GlobalState::lock_device(dev_idx, |dev| {
        // This is safe because primary context can't be dropped
        let ctx_ptr = &mut dev.primary_context as *mut _;
        let flags_ptr =
            (&unsafe { dev.primary_context.as_ref_unchecked() }.flags) as *const AtomicU32;
        let is_active = context::CONTEXT_STACK
            .with(|stack| stack.borrow().last().map(|x| *x))
            .map(|current| current == ctx_ptr)
            .unwrap_or(false);
        let flags_value = unsafe { &*flags_ptr }.load(Ordering::Relaxed);
        Ok::<_, CUresult>((is_active, flags_value))
    })??;
    unsafe { *active = if is_active { 1 } else { 0 } };
    unsafe { *flags = flags_value };
    Ok(())
}

pub fn primary_ctx_retain(
    pctx: *mut *mut context::Context,
    dev_idx: Index,
) -> Result<(), CUresult> {
    let ctx_ptr = GlobalState::lock_device(dev_idx, |dev| &mut dev.primary_context as *mut _)?;
    unsafe { *pctx = ctx_ptr };
    Ok(())
}

// TODO: allow for retain/reset/release of primary context
pub(crate) fn primary_ctx_release_v2(_dev_idx: Index) -> CUresult {
    CUresult::CUDA_SUCCESS
}
