use super::{context, transmute_lifetime, CUresult, Error};
use crate::cuda;
use cuda::{CUdevice_attribute, CUuuid_st};
use std::{
    cmp, mem,
    os::raw::{c_char, c_int},
    ptr,
    sync::{
        atomic::{AtomicU32, Ordering},
        Mutex, MutexGuard,
    },
};

const PROJECT_URL_SUFFIX: &'static str = " [github.com/vosen/notCUDA]";
static mut DEVICES: Option<Vec<Mutex<Device>>> = None;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Index(pub c_int);

pub struct Device {
    pub base: l0::Device,
    pub default_queue: l0::CommandQueue,
    pub l0_context: l0::Context,
    pub primary_context: context::Context,
    properties: Option<Box<l0::sys::ze_device_properties_t>>,
    image_properties: Option<Box<l0::sys::ze_device_image_properties_t>>,
    memory_properties: Option<Vec<l0::sys::ze_device_memory_properties_t>>,
    compute_properties: Option<Box<l0::sys::ze_device_compute_properties_t>>,
}

unsafe impl Send for Device {}

impl Device {
    // Unsafe because it does not fully initalize primary_context
    unsafe fn new(drv: &l0::Driver, d: l0::Device, idx: usize) -> l0::Result<Self> {
        let mut ctx = l0::Context::new(drv)?;
        let queue = l0::CommandQueue::new(&mut ctx, &d)?;
        let primary_context = context::Context::new(context::ContextData::new(
            0,
            true,
            Index(idx as c_int),
            ptr::null(),
        ));
        Ok(Self {
            base: d,
            default_queue: queue,
            l0_context: ctx,
            primary_context: primary_context,
            properties: None,
            image_properties: None,
            memory_properties: None,
            compute_properties: None,
        })
    }

    fn get_properties<'a>(&'a mut self) -> l0::Result<&'a l0::sys::ze_device_properties_t> {
        if let Some(ref prop) = self.properties {
            return Ok(prop);
        }
        match self.base.get_properties() {
            Ok(prop) => Ok(self.properties.get_or_insert(prop)),
            Err(e) => Err(e),
        }
    }

    fn get_image_properties(&mut self) -> l0::Result<&l0::sys::ze_device_image_properties_t> {
        if let Some(ref prop) = self.image_properties {
            return Ok(prop);
        }
        match self.base.get_image_properties() {
            Ok(prop) => Ok(self.image_properties.get_or_insert(prop)),
            Err(e) => Err(e),
        }
    }

    fn get_memory_properties(&mut self) -> l0::Result<&[l0::sys::ze_device_memory_properties_t]> {
        if let Some(ref prop) = self.memory_properties {
            return Ok(prop);
        }
        match self.base.get_memory_properties() {
            Ok(prop) => Ok(self.memory_properties.get_or_insert(prop)),
            Err(e) => Err(e),
        }
    }

    fn get_compute_properties(&mut self) -> l0::Result<&l0::sys::ze_device_compute_properties_t> {
        if let Some(ref prop) = self.compute_properties {
            return Ok(prop);
        }
        match self.base.get_compute_properties() {
            Ok(prop) => Ok(self.compute_properties.get_or_insert(prop)),
            Err(e) => Err(e),
        }
    }
}

pub fn init(driver: &l0::Driver) -> l0::Result<()> {
    let ze_devices = driver.devices()?;
    let mut devices = ze_devices
        .into_iter()
        .enumerate()
        .map(|(idx, d)| unsafe { Device::new(driver, d, idx) }.map(Mutex::new))
        .collect::<Result<Vec<_>, _>>()?;
    for d in devices.iter_mut() {
        d.get_mut()
            .unwrap()
            .primary_context
            .as_mut()
            .unwrap()
            .device = d;
    }
    unsafe { DEVICES = Some(devices) };
    Ok(())
}

fn devices() -> Result<&'static Vec<Mutex<Device>>, CUresult> {
    match unsafe { &DEVICES } {
        Some(devs) => Ok(devs),
        None => Err(CUresult::CUDA_ERROR_NOT_INITIALIZED),
    }
}

pub fn get_device_ref(Index(dev_idx): Index) -> Result<&'static Mutex<Device>, CUresult> {
    let devs = devices()?;
    if dev_idx < 0 || dev_idx >= devs.len() as c_int {
        return Err(CUresult::CUDA_ERROR_INVALID_DEVICE);
    }
    Ok(&devs[dev_idx as usize])
}

pub fn get_device(dev_idx: Index) -> Result<MutexGuard<'static, Device>, CUresult> {
    let dev = get_device_ref(dev_idx)?;
    dev.lock().map_err(|_| CUresult::CUDA_ERROR_ILLEGAL_STATE)
}

pub fn get_count(count: *mut c_int) -> CUresult {
    let len = devices().map(|d| d.len());
    match len {
        Ok(len) => {
            unsafe { *count = len as c_int };
            CUresult::CUDA_SUCCESS
        }
        Err(e) => e,
    }
}

pub fn get(device: *mut Index, ordinal: c_int) -> CUresult {
    if device == ptr::null_mut() || ordinal < 0 {
        return CUresult::CUDA_ERROR_INVALID_VALUE;
    }
    let len = devices().map(|d| d.len());
    match len {
        Ok(len) if ordinal < (len as i32) => {
            unsafe { *device = Index(ordinal) };
            CUresult::CUDA_SUCCESS
        }
        Ok(_) => CUresult::CUDA_ERROR_INVALID_VALUE,
        Err(e) => e,
    }
}

pub fn get_name(name: *mut c_char, len: i32, dev: Index) -> Result<(), CUresult> {
    if name == ptr::null_mut() || len < 0 {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    // This is safe because devices are 'static
    let name_ptr = {
        let mut dev = get_device(dev)?;
        let props = dev.get_properties().map_err(Into::<CUresult>::into)?;
        props.name.as_ptr()
    };
    let name_len = (0..256)
        .position(|i| unsafe { *name_ptr.add(i) } == 0)
        .unwrap_or(256);
    let mut dst_null_pos = cmp::min((len - 1) as usize, name_len);
    unsafe { std::ptr::copy_nonoverlapping(name_ptr, name, dst_null_pos) };
    if name_len + PROJECT_URL_SUFFIX.len() < (len as usize) {
        unsafe {
            std::ptr::copy_nonoverlapping(
                PROJECT_URL_SUFFIX.as_ptr(),
                name.add(name_len) as *mut _,
                PROJECT_URL_SUFFIX.len(),
            )
        };
        dst_null_pos += PROJECT_URL_SUFFIX.len();
    }
    unsafe { *(name.add(dst_null_pos)) = 0 };
    Ok(())
}

pub fn total_mem_v2(bytes: *mut usize, dev: Index) -> Result<(), CUresult> {
    if bytes == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    // This is safe because devices are 'static
    let mem_props = {
        let mut dev = get_device(dev)?;
        unsafe {
            transmute_lifetime(
                dev.get_memory_properties()
                    .map_err(Into::<CUresult>::into)?,
            )
        }
    };
    let max_mem = mem_props
        .iter()
        .map(|p| p.totalSize)
        .max()
        .ok_or(CUresult::CUDA_ERROR_ILLEGAL_STATE)?;
    unsafe { *bytes = max_mem as usize };
    Ok(())
}

impl CUdevice_attribute {
    fn get_static_value(self) -> Option<i32> {
        match self {
            CUdevice_attribute::CU_DEVICE_ATTRIBUTE_GPU_OVERLAP => Some(1),
            CUdevice_attribute::CU_DEVICE_ATTRIBUTE_KERNEL_EXEC_TIMEOUT => Some(1),
            // TODO: fix this for DG1
            CUdevice_attribute::CU_DEVICE_ATTRIBUTE_INTEGRATED => Some(1),
            // TODO: go back to this once we have more funcitonality implemented
            CUdevice_attribute::CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR => Some(8),
            CUdevice_attribute::CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR => Some(0),
            CUdevice_attribute::CU_DEVICE_ATTRIBUTE_CAN_MAP_HOST_MEMORY => Some(1),
            _ => None,
        }
    }
}

pub fn get_attribute(pi: *mut i32, attrib: CUdevice_attribute, dev: Index) -> Result<(), Error> {
    if pi == ptr::null_mut() {
        return Err(Error::Cuda(CUresult::CUDA_ERROR_INVALID_VALUE));
    }
    if let Some(value) = attrib.get_static_value() {
        unsafe { *pi = value };
        return Ok(());
    }
    let mut dev = get_device(dev).map_err(Error::Cuda)?;
    let value = match attrib {
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_ASYNC_ENGINE_COUNT => {
            dev.get_properties().map_err(Error::L0)?.maxHardwareContexts as i32
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MULTIPROCESSOR_COUNT => {
            let props = dev.get_properties().map_err(Error::L0)?;
            (props.numSlices * props.numSubslicesPerSlice * props.numEUsPerSubslice) as i32
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_WIDTH => cmp::min(
            dev.get_image_properties()
                .map_err(Error::L0)?
                .maxImageDims1D,
            c_int::max_value() as u32,
        ) as c_int,
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_X => {
            let props = dev.get_compute_properties().map_err(Error::L0)?;
            cmp::max(i32::max_value() as u32, props.maxGroupCountX) as i32
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_Y => {
            let props = dev.get_compute_properties().map_err(Error::L0)?;
            cmp::max(i32::max_value() as u32, props.maxGroupCountY) as i32
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_Z => {
            let props = dev.get_compute_properties().map_err(Error::L0)?;
            cmp::max(i32::max_value() as u32, props.maxGroupCountZ) as i32
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_X => {
            let props = dev.get_compute_properties().map_err(Error::L0)?;
            cmp::max(i32::max_value() as u32, props.maxGroupSizeX) as i32
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_Y => {
            let props = dev.get_compute_properties().map_err(Error::L0)?;
            cmp::max(i32::max_value() as u32, props.maxGroupSizeY) as i32
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_Z => {
            let props = dev.get_compute_properties().map_err(Error::L0)?;
            cmp::max(i32::max_value() as u32, props.maxGroupSizeZ) as i32
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_BLOCK => {
            let props = dev.get_compute_properties().map_err(Error::L0)?;
            cmp::max(i32::max_value() as u32, props.maxTotalGroupSize) as i32
        }
        _ => {
            // TODO: support more attributes for CUDA runtime
            /*
            return Err(l0::Error(
                l0::sys::ze_result_t::ZE_RESULT_ERROR_UNSUPPORTED_FEATURE,
            ))
            */
            return Ok(());
        }
    };
    unsafe { *pi = value };
    Ok(())
}

pub fn get_uuid(uuid: *mut CUuuid_st, dev: Index) -> Result<(), Error> {
    let ze_uuid = {
        get_device(dev)
            .map_err(Error::Cuda)?
            .get_properties()
            .map_err(Error::L0)?
            .uuid
    };
    unsafe {
        *uuid = CUuuid_st {
            bytes: mem::transmute(ze_uuid.id),
        }
    };
    Ok(())
}

pub fn with_current_exclusive<F: FnOnce(&mut Device) -> R, R>(f: F) -> Result<R, CUresult> {
    let dev = super::context::with_current(|ctx| ctx.device);
    dev.and_then(|dev| {
        unsafe { &*dev }
            .try_lock()
            .map(|mut dev| f(&mut dev))
            .map_err(|_| CUresult::CUDA_ERROR_ILLEGAL_STATE)
    })
}

pub fn with_exclusive<F: FnOnce(&mut Device) -> R, R>(dev: Index, f: F) -> Result<R, CUresult> {
    let dev = get_device_ref(dev)?;
    dev.try_lock()
        .map(|mut dev| f(&mut dev))
        .map_err(|_| CUresult::CUDA_ERROR_ILLEGAL_STATE)
}

pub fn primary_ctx_get_state(
    idx: Index,
    flags: *mut u32,
    active: *mut i32,
) -> Result<(), CUresult> {
    let (ctx_ptr, flags_ptr) = with_exclusive(idx, |dev| {
        // This is safe because primary context can't be dropped
        let ctx_ptr = &dev.primary_context as *const _;
        let flags_ptr =
            (&unsafe { dev.primary_context.as_ref_unchecked() }.flags) as *const AtomicU32;
        (ctx_ptr, flags_ptr)
    })?;
    let is_active = context::CONTEXT_STACK
        .with(|stack| stack.borrow().last().map(|x| *x))
        .map(|current| current == ctx_ptr)
        .unwrap_or(false);
    let flags_value = unsafe { &*flags_ptr }.load(Ordering::Relaxed);
    unsafe { *flags = flags_value };
    unsafe { *active = if is_active { 1 } else { 0 } };
    Ok(())
}

pub fn primary_ctx_retain(pctx: *mut *mut context::Context, dev: Index) -> Result<(), CUresult> {
    let ctx_ptr = with_exclusive(dev, |dev| &mut dev.primary_context as *mut _)?;
    unsafe { *pctx = ctx_ptr };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::test::CudaDriverFns;
    use super::super::CUresult;

    cuda_driver_test!(primary_ctx_default_inactive);

    fn primary_ctx_default_inactive<T: CudaDriverFns>() {
        assert_eq!(T::cuInit(0), CUresult::CUDA_SUCCESS);
        let mut flags = u32::max_value();
        let mut active = i32::max_value();
        assert_eq!(
            T::cuDevicePrimaryCtxGetState(0, &mut flags, &mut active),
            CUresult::CUDA_SUCCESS
        );
        assert_eq!(flags, 0);
        assert_eq!(active, 0);
    }
}
