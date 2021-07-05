use super::{context, transmute_lifetime, transmute_lifetime_mut, CUresult, GlobalState};
use crate::cuda;
use cuda::{CUdevice_attribute, CUuuid_st};
use std::{
    cmp, mem,
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
    pub base: l0::Device,
    pub default_queue: l0::CommandQueue<'static>,
    pub l0_context: l0::Context,
    pub primary_context: context::Context,
    pub device_event_pool: DynamicEventPool,
    pub host_event_pool: DynamicEventPool,
    properties: Option<Box<l0::sys::ze_device_properties_t>>,
    image_properties: Option<Box<l0::sys::ze_device_image_properties_t>>,
    memory_properties: Option<Vec<l0::sys::ze_device_memory_properties_t>>,
    compute_properties: Option<Box<l0::sys::ze_device_compute_properties_t>>,
}

unsafe impl Send for Device {}

impl Device {
    // Unsafe because it does not fully initalize primary_context
    // and we transmute lifetimes left and right
    unsafe fn new(drv: &l0::Driver, l0_dev: l0::Device, idx: usize) -> Result<Self, CUresult> {
        let ctx = l0::Context::new(*drv, Some(&[l0_dev]))?;
        let queue = l0::CommandQueue::new(mem::transmute(&ctx), l0_dev)?;
        let mut host_event_pool = DynamicEventPool::new(
            l0_dev,
            transmute_lifetime(&ctx),
            l0::sys::ze_event_pool_flags_t::ZE_EVENT_POOL_FLAG_HOST_VISIBLE,
            l0::sys::ze_event_scope_flags_t::ZE_EVENT_SCOPE_FLAG_HOST,
        )?;
        let host_event =
            transmute_lifetime_mut(&mut host_event_pool).get(l0_dev, transmute_lifetime(&ctx))?;
        let primary_context = context::Context::new(context::ContextData::new(
            transmute_lifetime(&ctx),
            l0_dev,
            0,
            true,
            host_event,
            ptr::null_mut(),
        )?);
        let device_event_pool = DynamicEventPool::new(
            l0_dev,
            transmute_lifetime(&ctx),
            l0::sys::ze_event_pool_flags_t(0),
            l0::sys::ze_event_scope_flags_t(0),
        )?;
        Ok(Self {
            index: Index(idx as c_int),
            base: l0_dev,
            default_queue: queue,
            l0_context: ctx,
            primary_context: primary_context,
            device_event_pool,
            host_event_pool,
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
        let mut props = Default::default();
        self.base.get_properties(&mut props)?;
        Ok(self.properties.get_or_insert(Box::new(props)))
    }

    fn get_image_properties(&mut self) -> l0::Result<&l0::sys::ze_device_image_properties_t> {
        if let Some(ref prop) = self.image_properties {
            return Ok(prop);
        }
        let mut props = Default::default();
        self.base.get_image_properties(&mut props)?;
        Ok(self.image_properties.get_or_insert(Box::new(props)))
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
        let mut props = Default::default();
        self.base.get_compute_properties(&mut props)?;
        Ok(self.compute_properties.get_or_insert(Box::new(props)))
    }

    pub fn late_init(&mut self) {
        self.primary_context.as_option_mut().unwrap().device = self as *mut _;
    }

    fn get_max_simd(&mut self) -> l0::Result<u32> {
        let props = self.get_compute_properties()?;
        Ok(*props.subGroupSizes[0..props.numSubGroupSizes as usize]
            .iter()
            .max()
            .unwrap())
    }
}

pub fn init(driver: &l0::Driver) -> Result<Vec<Device>, CUresult> {
    let ze_devices = driver.devices()?;
    let mut devices = ze_devices
        .into_iter()
        .enumerate()
        .map(|(idx, d)| unsafe { Device::new(driver, d, idx) })
        .collect::<Result<Vec<_>, _>>()?;
    for dev in devices.iter_mut() {
        dev.late_init();
        dev.primary_context.late_init();
    }
    Ok(devices)
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
    let name_ptr = GlobalState::lock_device(dev_idx, |dev| {
        let props = dev.get_properties()?;
        Ok::<_, l0::sys::ze_result_t>(props.name.as_ptr())
    })??;
    let name_len = (0..256)
        .position(|i| unsafe { *name_ptr.add(i) } == 0)
        .unwrap_or(256);
    let mut dst_null_pos = cmp::min((len - 1) as usize, name_len);
    unsafe { std::ptr::copy_nonoverlapping(name_ptr, name, dst_null_pos) };
    if name_len + PROJECT_URL_SUFFIX_LONG.len() < (len as usize) {
        unsafe {
            std::ptr::copy_nonoverlapping(
                PROJECT_URL_SUFFIX_LONG.as_ptr(),
                name.add(name_len) as *mut _,
                PROJECT_URL_SUFFIX_LONG.len(),
            )
        };
        dst_null_pos += PROJECT_URL_SUFFIX_LONG.len();
    } else if name_len + PROJECT_URL_SUFFIX_SHORT.len() < (len as usize) {
        unsafe {
            std::ptr::copy_nonoverlapping(
                PROJECT_URL_SUFFIX_SHORT.as_ptr(),
                name.add(name_len) as *mut _,
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
    let mem_props = GlobalState::lock_device(dev_idx, |dev| {
        let mem_props = dev.get_memory_properties()?;
        Ok::<_, l0::sys::ze_result_t>(mem_props)
    })??;
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
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_ASYNC_ENGINE_COUNT => {
            GlobalState::lock_device(dev_idx, |dev| {
                let props = dev.get_properties()?;
                Ok::<_, l0::sys::ze_result_t>(props.maxHardwareContexts as i32)
            })??
        }
        // Streaming Multiprocessor corresponds roughly to a sub-slice (thread group can't cross either)
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MULTIPROCESSOR_COUNT => {
            GlobalState::lock_device(dev_idx, |dev| {
                let props = dev.get_properties()?;
                Ok::<_, l0::sys::ze_result_t>((props.numSlices * props.numSubslicesPerSlice) as i32)
            })??
        }
        // I honestly don't know how to answer this query
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_MULTIPROCESSOR => {
            GlobalState::lock_device(dev_idx, |dev| {
                let max_simd = dev.get_max_simd()?;
                let props = dev.get_properties()?;
                Ok::<_, l0::sys::ze_result_t>(
                    (props.numEUsPerSubslice * props.numThreadsPerEU * max_simd) as i32,
                )
            })??
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_BLOCK => {
            GlobalState::lock_device(dev_idx, |dev| {
                let props = dev.get_compute_properties()?;
                Ok::<_, l0::sys::ze_result_t>(cmp::min(
                    i32::max_value() as u32,
                    props.maxTotalGroupSize,
                ) as i32)
            })??
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_WIDTH => {
            GlobalState::lock_device(dev_idx, |dev| {
                let props = dev.get_image_properties()?;
                Ok::<_, l0::sys::ze_result_t>(cmp::min(
                    props.maxImageDims1D,
                    c_int::max_value() as u32,
                ) as c_int)
            })??
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_X => {
            GlobalState::lock_device(dev_idx, |dev| {
                let props = dev.get_compute_properties()?;
                Ok::<_, l0::sys::ze_result_t>(cmp::min(
                    i32::max_value() as u32,
                    props.maxGroupCountX,
                ) as i32)
            })??
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_Y => {
            GlobalState::lock_device(dev_idx, |dev| {
                let props = dev.get_compute_properties()?;
                Ok::<_, l0::sys::ze_result_t>(cmp::min(
                    i32::max_value() as u32,
                    props.maxGroupCountY,
                ) as i32)
            })??
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_Z => {
            GlobalState::lock_device(dev_idx, |dev| {
                let props = dev.get_compute_properties()?;
                Ok::<_, l0::sys::ze_result_t>(cmp::min(
                    i32::max_value() as u32,
                    props.maxGroupCountZ,
                ) as i32)
            })??
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_X => {
            GlobalState::lock_device(dev_idx, |dev| {
                let props = dev.get_compute_properties()?;
                Ok::<_, l0::sys::ze_result_t>(
                    cmp::min(i32::max_value() as u32, props.maxGroupSizeX) as i32,
                )
            })??
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_Y => {
            GlobalState::lock_device(dev_idx, |dev| {
                let props = dev.get_compute_properties()?;
                Ok::<_, l0::sys::ze_result_t>(
                    cmp::min(i32::max_value() as u32, props.maxGroupSizeY) as i32,
                )
            })??
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_Z => {
            GlobalState::lock_device(dev_idx, |dev| {
                let props = dev.get_compute_properties()?;
                Ok::<_, l0::sys::ze_result_t>(
                    cmp::min(i32::max_value() as u32, props.maxGroupSizeZ) as i32,
                )
            })??
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_BLOCK => {
            GlobalState::lock_device(dev_idx, |dev| {
                let props = dev.get_compute_properties()?;
                Ok::<_, l0::sys::ze_result_t>(props.maxSharedLocalMemory as i32)
            })??
        }
        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_WARP_SIZE => {
            GlobalState::lock_device(dev_idx, |dev| Ok::<_, CUresult>(dev.get_max_simd()? as i32))??
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

pub fn get_uuid(uuid: *mut CUuuid_st, dev_idx: Index) -> Result<(), CUresult> {
    let ze_uuid = GlobalState::lock_device(dev_idx, |dev| {
        let props = dev.get_properties()?;
        Ok::<_, l0::sys::ze_result_t>(props.uuid)
    })??;
    unsafe {
        *uuid = CUuuid_st {
            bytes: mem::transmute(ze_uuid.id),
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
        Ok::<_, l0::sys::ze_result_t>((is_active, flags_value))
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

pub struct DynamicEventPool {
    count: usize,
    pool_flags: l0::sys::ze_event_pool_flags_t,
    signal_flags: l0::sys::ze_event_scope_flags_t,
    events: Vec<DynamicEventPoolEntry>,
}

impl DynamicEventPool {
    fn new(
        dev: l0::Device,
        ctx: &'static l0::Context,
        pool_flags: l0::sys::ze_event_pool_flags_t,
        signal_flags: l0::sys::ze_event_scope_flags_t,
    ) -> l0::Result<Self> {
        Ok(DynamicEventPool {
            count: 0,
            pool_flags,
            signal_flags,
            events: vec![DynamicEventPoolEntry::new(dev, ctx, pool_flags)?],
        })
    }

    pub fn get(
        &'static mut self,
        dev: l0::Device,
        ctx: &'static l0::Context,
    ) -> l0::Result<(l0::Event<'static>, u64)> {
        self.count += 1;
        let events = unsafe { transmute_lifetime_mut(&mut self.events) };
        let (global_idx, (ev, local_idx)) = {
            for (idx, entry) in self.events.iter_mut().enumerate() {
                if let Some((ev, local_idx)) = entry.get(self.signal_flags)? {
                    let marker = (idx << 32) as u64 | local_idx as u64;
                    return Ok((ev, marker));
                }
            }
            events.push(DynamicEventPoolEntry::new(dev, ctx, self.pool_flags)?);
            let global_idx = (events.len() - 1) as u64;
            (
                global_idx,
                events.last_mut().unwrap().get(self.signal_flags)?.unwrap(),
            )
        };
        let marker = (global_idx << 32) | local_idx as u64;
        Ok((ev, marker))
    }

    pub fn mark_as_free(&mut self, marker: u64) {
        let global_idx = (marker >> 32) as u32;
        self.events[global_idx as usize].mark_as_free(marker as u32);
        self.count -= 1;
        // TODO: clean up empty entries
    }
}

const DYNAMIC_EVENT_POOL_ENTRY_SIZE: usize = 448;
const DYNAMIC_EVENT_POOL_ENTRY_BITMAP_SIZE: usize =
    DYNAMIC_EVENT_POOL_ENTRY_SIZE / (mem::size_of::<u64>() * 8);
#[repr(C)]
#[repr(align(64))]
struct DynamicEventPoolEntry {
    event_pool: l0::EventPool<'static>,
    bit_map: [u64; DYNAMIC_EVENT_POOL_ENTRY_BITMAP_SIZE],
}

impl DynamicEventPoolEntry {
    fn new(
        dev: l0::Device,
        ctx: &'static l0::Context,
        flags: l0::sys::ze_event_pool_flags_t,
    ) -> l0::Result<Self> {
        Ok(DynamicEventPoolEntry {
            event_pool: l0::EventPool::new(
                ctx,
                flags,
                DYNAMIC_EVENT_POOL_ENTRY_SIZE as u32,
                Some(&[dev]),
            )?,
            bit_map: [0; DYNAMIC_EVENT_POOL_ENTRY_BITMAP_SIZE],
        })
    }

    fn get(
        &'static mut self,
        signal: l0::sys::ze_event_scope_flags_t,
    ) -> l0::Result<Option<(l0::Event<'static>, u32)>> {
        for (idx, value) in self.bit_map.iter_mut().enumerate() {
            let shift = first_index_of_zero_u64(*value);
            if shift == 64 {
                continue;
            }
            *value = *value | (1u64 << shift);
            let entry_index = (idx as u32 * 64u32) + shift;
            let event = l0::Event::new(
                &self.event_pool,
                entry_index,
                signal,
                l0::sys::ze_event_scope_flags_t(0),
            )?;
            return Ok(Some((event, entry_index)));
        }
        Ok(None)
    }

    fn mark_as_free(&mut self, idx: u32) {
        let value = &mut self.bit_map[idx as usize / 64];
        let shift = idx % 64;
        *value = *value & !(1 << shift);
    }
}

fn first_index_of_zero_u64(x: u64) -> u32 {
    let x = !x;
    (x & x.wrapping_neg()).trailing_zeros()
}

#[cfg(test)]
mod test {
    use std::mem;

    use super::DynamicEventPoolEntry;

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

    #[test]
    pub fn dynamic_event_pool_page_is_64b() {
        assert_eq!(mem::size_of::<DynamicEventPoolEntry>(), 64);
        assert_eq!(mem::align_of::<DynamicEventPoolEntry>(), 64);
    }
}
