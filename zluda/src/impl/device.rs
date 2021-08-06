use super::{context, transmute_lifetime, transmute_lifetime_mut, CUresult, GlobalState};
use crate::cuda;
use cuda::{CUdevice_attribute, CUuuid_st};
use ocl_core::{ClDeviceIdPtr, ContextProperties, DeviceType};
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
        Ok(Self {
            index: Index(idx as c_int),
            ocl_base: ocl_dev,
            default_queue: queue,
            ocl_context: ctx,
            primary_context,
            allocations: HashSet::new(),
            is_amd,
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
    let name_string = GlobalState::lock_device(dev_idx, |dev| {
        let props = ocl_core::get_device_info(dev.ocl_base, ocl_core::DeviceInfo::Name)?;
        if let ocl_core::DeviceInfoResult::Name(name) = props {
            Ok(name)
        } else {
            Err(CUresult::CUDA_ERROR_UNKNOWN)
        }
    })??;
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

impl CUdevice_attribute {
    fn get_static_value(self) -> Option<i32> {
        match self {
            CUdevice_attribute::CU_DEVICE_ATTRIBUTE_GPU_OVERLAP => Some(1),
            CUdevice_attribute::CU_DEVICE_ATTRIBUTE_KERNEL_EXEC_TIMEOUT => Some(1),
            // TODO: go back to this once we have more funcitonality implemented
            CUdevice_attribute::CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR => Some(8),
            CUdevice_attribute::CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR => Some(0),
            CUdevice_attribute::CU_DEVICE_ATTRIBUTE_CAN_MAP_HOST_MEMORY => Some(1),
            _ => None,
        }
    }
}

pub fn get_attribute(
    pi: *mut i32,
    attrib: CUdevice_attribute,
    dev_idx: Index,
) -> Result<(), CUresult> {
    if pi == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    if let Some(value) = attrib.get_static_value() {
        unsafe { *pi = value };
        return Ok(());
    }
    let value = match attrib {
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_INTEGRATED => {
            GlobalState::lock_device(dev_idx, |dev| if dev.is_amd { 0i32 } else { 1i32 })?
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_ASYNC_ENGINE_COUNT => 1,
        // Streaming Multiprocessor corresponds roughly to a sub-slice (thread group can't cross either)
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MULTIPROCESSOR_COUNT => {
            GlobalState::lock_device(dev_idx, |dev| {
                let props =
                    ocl_core::get_device_info(dev.ocl_base, ocl_core::DeviceInfo::MaxComputeUnits)?;
                if let ocl_core::DeviceInfoResult::MaxComputeUnits(count) = props {
                    Ok(count as i32)
                } else {
                    Err(CUresult::CUDA_ERROR_UNKNOWN)
                }
            })??
        }
        // I honestly don't know how to answer this query
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_MULTIPROCESSOR => {
            GlobalState::lock_device(dev_idx, |dev| {
                if !dev.is_amd {
                    8i32 * 7 // correct for GEN9
                } else {
                    4i32 * 32 // probably correct for RDNA
                }
            })?
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_BLOCK => {
            GlobalState::lock_device(dev_idx, |dev| {
                let props = ocl_core::get_device_info(
                    dev.ocl_base,
                    ocl_core::DeviceInfo::MaxWorkGroupSize,
                )?;
                if let ocl_core::DeviceInfoResult::MaxWorkGroupSize(size) = props {
                    Ok(size as i32)
                } else {
                    Err(CUresult::CUDA_ERROR_UNKNOWN)
                }
            })??
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_X => {
            GlobalState::lock_device(dev_idx, |dev| {
                let props = ocl_core::get_device_info(
                    dev.ocl_base,
                    ocl_core::DeviceInfo::MaxWorkItemSizes,
                )?;
                if let ocl_core::DeviceInfoResult::MaxWorkItemSizes(sizes) = props {
                    Ok(sizes)
                } else {
                    Err(CUresult::CUDA_ERROR_UNKNOWN)
                }
            })??[0] as i32
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_Y => {
            GlobalState::lock_device(dev_idx, |dev| {
                let props = ocl_core::get_device_info(
                    dev.ocl_base,
                    ocl_core::DeviceInfo::MaxWorkItemSizes,
                )?;
                if let ocl_core::DeviceInfoResult::MaxWorkItemSizes(sizes) = props {
                    Ok(sizes)
                } else {
                    Err(CUresult::CUDA_ERROR_UNKNOWN)
                }
            })??[1] as i32
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_Z => {
            GlobalState::lock_device(dev_idx, |dev| {
                let props = ocl_core::get_device_info(
                    dev.ocl_base,
                    ocl_core::DeviceInfo::MaxWorkItemSizes,
                )?;
                if let ocl_core::DeviceInfoResult::MaxWorkItemSizes(sizes) = props {
                    Ok(sizes)
                } else {
                    Err(CUresult::CUDA_ERROR_UNKNOWN)
                }
            })??[2] as i32
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_BLOCK => {
            GlobalState::lock_device(dev_idx, |dev| {
                let props =
                    ocl_core::get_device_info(dev.ocl_base, ocl_core::DeviceInfo::LocalMemSize)?;
                if let ocl_core::DeviceInfoResult::LocalMemSize(size) = props {
                    Ok(size)
                } else {
                    Err(CUresult::CUDA_ERROR_UNKNOWN)
                }
            })?? as i32
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_WARP_SIZE => 32,
        _ => {
            // TODO: support more attributes for CUDA runtime
            /*
            return Err(l0::Error(
                l0::sys::ze_result_t::ZE_RESULT_ERROR_UNSUPPORTED_FEATURE,
            ))
            */
            0
        }
    };
    unsafe { *pi = value };
    Ok(())
}

pub fn get_uuid(uuid: *mut CUuuid_st, dev_idx: Index) -> Result<(), CUresult> {
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
