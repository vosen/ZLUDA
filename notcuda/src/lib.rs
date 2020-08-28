extern crate level_zero as l0;
extern crate level_zero_sys as l0_sys;
#[macro_use]
extern crate lazy_static;
extern crate lz4;

use std::cell::RefCell;
use std::convert::TryFrom;
use std::os::raw::{c_char, c_int, c_uint};
use std::ptr;
use std::{ffi::c_void, sync::Mutex};

mod cu;
mod cuda;
mod export_table;
mod ze;

thread_local! {
    static CONTEXT_STACK: RefCell<Vec<cu::Context>> = RefCell::new(Vec::new());
}

lazy_static! {
    static ref GLOBAL_STATE: Mutex<Option<Context>> = Mutex::new(None);
}

struct Context {
    base: l0::Context,
    devices: Vec<ze::Device>,
}
unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Context {
    fn new() -> l0::Result<Context> {
        let driver = l0::Driver::get()?;
        let ze_devices = driver[0].devices()?;
        let mut ctx = l0::Context::new(&driver[0])?;
        let devices = ze_devices
            .into_iter()
            .map(|d| ze::Device::new(&mut ctx, d))
            .collect::<Result<_, _>>()?;
        Ok(Context {
            base: ctx,
            devices: devices,
        })
    }

    fn call<F: FnOnce(&mut Context) -> l0_sys::ze_result_t>(f: F) -> cu::Result {
        let mut lock = GLOBAL_STATE.try_lock();
        if let Ok(ref mut mutex) = lock {
            match **mutex {
                None => return cu::Result::ERROR_NOT_INITIALIZED,
                Some(ref mut driver) => {
                    return cu::Result::from_l0(f(driver));
                }
            }
        } else {
            return cu::Result::ERROR_UNKNOWN;
        }
    }

    fn call2<F: FnOnce(&mut Context) -> l0::Result<()>>(f: F) -> cu::Result {
        Context::call(|ctx| match f(ctx) {
            Ok(()) => l0::sys::ze_result_t::ZE_RESULT_SUCCESS,
            Err(err) => err.0,
        })
    }

    fn call_device<F: FnOnce(&mut ze::Device) -> l0::Result<()>>(
        cu::Device(dev): cu::Device,
        f: F,
    ) -> cu::Result {
        if dev < 0 {
            return cu::Result::ERROR_INVALID_VALUE;
        }
        let dev = dev as usize;
        Context::call(|driver| {
            if dev >= driver.devices.len() {
                return l0_sys::ze_result_t::ZE_RESULT_ERROR_INVALID_ARGUMENT;
            }
            match f(&mut driver.devices[dev]) {
                Ok(()) => l0::sys::ze_result_t::ZE_RESULT_SUCCESS,
                Err(err) => err.0,
            }
        })
    }

    fn device_get_count(&self, count: *mut i32) -> l0_sys::ze_result_t {
        unsafe { *count = self.devices.len() as i32 };
        l0_sys::ze_result_t::ZE_RESULT_SUCCESS
    }

    fn device_get(&self, device: *mut cu::Device, ordinal: c_int) -> l0_sys::ze_result_t {
        if (ordinal as usize) >= self.devices.len() {
            return l0_sys::ze_result_t::ZE_RESULT_ERROR_INVALID_ARGUMENT;
        }
        unsafe { *device = cu::Device(ordinal) };
        l0_sys::ze_result_t::ZE_RESULT_SUCCESS
    }
}

#[no_mangle]
pub unsafe extern "C" fn cuDriverGetVersion(version: *mut c_int) -> cu::Result {
    if version == ptr::null_mut() {
        return cu::Result::ERROR_INVALID_VALUE;
    }
    *version = i32::max_value();
    return cu::Result::SUCCESS;
}

#[no_mangle]
pub unsafe extern "C" fn cuInit(_: c_uint) -> cu::Result {
    let l0_sys_init = l0_sys::zeInit(l0_sys::ze_init_flags_t::ZE_INIT_FLAG_GPU_ONLY);
    if l0_sys_init != l0_sys::ze_result_t::ZE_RESULT_SUCCESS {
        return cu::Result::from_l0(l0_sys_init);
    }
    let mut lock = GLOBAL_STATE.try_lock();
    if let Ok(ref mut mutex) = lock {
        if let None = **mutex {
            match Context::new() {
                Ok(state) => **mutex = Some(state),
                Err(err) => return cu::Result::from_l0(err.0),
            }
        }
    } else {
        return cu::Result::ERROR_UNKNOWN;
    }
    cu::Result::SUCCESS
}

#[no_mangle]
pub extern "C" fn cuDeviceGetCount(count: *mut c_int) -> cu::Result {
    if count == ptr::null_mut() {
        return cu::Result::ERROR_INVALID_VALUE;
    }
    Context::call(|driver| driver.device_get_count(count))
}

#[no_mangle]
pub extern "C" fn cuDeviceGet(device: *mut cu::Device, ordinal: c_int) -> cu::Result {
    if ordinal < 0 || device == ptr::null_mut() {
        return cu::Result::ERROR_INVALID_VALUE;
    }
    Context::call(|driver| driver.device_get(device, ordinal))
}

#[no_mangle]
pub extern "C" fn cuDeviceGetName(
    name: *mut c_char,
    len: c_int,
    dev_idx: cu::Device,
) -> cu::Result {
    if name == ptr::null_mut() || len <= 0 {
        return cu::Result::ERROR_INVALID_VALUE;
    }
    Context::call_device(dev_idx, |dev| dev.get_name(name, len))
}

#[no_mangle]
pub extern "C" fn cuDeviceTotalMem_v2(bytes: *mut usize, dev_idx: cu::Device) -> cu::Result {
    if bytes == ptr::null_mut() {
        return cu::Result::ERROR_INVALID_VALUE;
    }
    Context::call_device(dev_idx, |dev| dev.total_mem(bytes))
}

#[no_mangle]
pub extern "C" fn cuDeviceGetAttribute(
    pi: *mut c_int,
    attrib: c_int,
    dev_idx: cu::Device,
) -> cu::Result {
    if pi == ptr::null_mut() {
        return cu::Result::ERROR_INVALID_VALUE;
    }
    let attrib = match u8::try_from(attrib) {
        Ok(a) => a,
        Err(_) => return cu::Result::ERROR_INVALID_VALUE,
    };
    match cu::DeviceAttribute::try_new(attrib) {
        Some(cu::DeviceAttribute::Static(a)) => {
            unsafe { *pi = ze::Device::get_attribute_static(a) };
            cu::Result::SUCCESS
        }
        Some(cu::DeviceAttribute::Dynamic(a)) => {
            Context::call_device(dev_idx, |dev| dev.get_attribute(pi, a))
        }
        // TODO: add support for more properties
        None => cu::Result::SUCCESS,
    }
}

#[no_mangle]
pub extern "C" fn cuDeviceGetUuid(uuid: *mut cu::Uuid, dev_idx: cu::Device) -> cu::Result {
    if uuid == ptr::null_mut() {
        return cu::Result::ERROR_INVALID_VALUE;
    }
    Context::call_device(dev_idx, |dev| dev.get_uuid(uuid))
}

#[no_mangle]
pub extern "C" fn cuCtxGetCurrent(pctx: *mut cu::Context) -> cu::Result {
    let ctx = CONTEXT_STACK.with(|stack| match stack.borrow().last() {
        Some(ctx) => ctx.clone(),
        None => cu::Context::null(),
    });
    unsafe { *pctx = ctx };
    cu::Result::SUCCESS
}

#[no_mangle]
pub extern "C" fn cuCtxSetCurrent(ctx: cu::Context) -> cu::Result {
    CONTEXT_STACK.with(|stack| {
        let mut stack = stack.borrow_mut();
        stack.pop();
        if ctx != cu::Context::null() {
            stack.push(ctx);
        }
    });
    cu::Result::SUCCESS
}

#[no_mangle]
pub extern "C" fn cuMemAlloc_v2(dptr: *mut cu::DevicePtr, bytesize: usize) -> cu::Result {
    if dptr == ptr::null_mut() || bytesize == 0 {
        return cu::Result::ERROR_INVALID_VALUE;
    }
    Context::call2(|drv| {
        unsafe {
            *dptr = cu::DevicePtr(drv.devices[0].base.mem_alloc_device(
                &mut drv.base,
                bytesize,
                0,
            )? as usize)
        };
        Ok(())
    })
}

#[no_mangle]
pub extern "C" fn cuCtxDestroy_v2(ctx: cu::Context) -> cu::Result {
    cu::Result::ERROR_NOT_SUPPORTED
}

#[no_mangle]
pub extern "C" fn cuMemcpyDtoH_v2(
    dst_host: *mut c_void,
    src_device: cu::DevicePtr,
    byte_count: usize,
) -> cu::Result {
    if dst_host == ptr::null_mut() || src_device.0 == 0 || byte_count == 0 {
        return cu::Result::ERROR_INVALID_VALUE;
    }
    // TODO: choose the right device from device ptr
    Context::call(|drv| {
        let cu_dev = &mut drv.devices[0];
        // Safe, because there's no Drop impl for device
        let dev = unsafe { l0::Device::from_ffi(cu_dev.base.as_ffi()) };
        let queue = &mut cu_dev.default_queue;
        let result = unsafe {
            memcpy_impl(
                &mut drv.base,
                dst_host,
                src_device.0 as *mut _,
                byte_count,
                &dev,
                queue,
            )
        };
        match result {
            Ok(()) => l0_sys::ze_result_t::ZE_RESULT_SUCCESS,
            Err(e) => e.0,
        }
    })
}

#[no_mangle]
pub extern "C" fn cuMemFree_v2(srcDevice: cu::DevicePtr) -> cu::Result {
    cu::Result::ERROR_NOT_SUPPORTED
}

#[no_mangle]
pub extern "C" fn cuModuleLoad(module: *mut cu::Module, fname: *const c_char) -> cu::Result {
    cu::Result::ERROR_NOT_SUPPORTED
}

#[no_mangle]
pub extern "C" fn cuGetErrorString(error: cu::Result, pStr: *mut *const c_char) -> cu::Result {
    cu::Result::ERROR_NOT_SUPPORTED
}

#[no_mangle]
pub extern "C" fn cuLaunchKernel(
    f: cu::Function,
    gridDimX: c_uint,
    gridDimY: c_uint,
    gridDimZ: c_uint,
    blockDimX: c_uint,
    blockDimY: c_uint,
    blockDimZ: c_uint,
    sharedMemBytes: c_uint,
    hStream: cu::Stream,
    kernelParams: *mut *mut (),
    extra: *mut *mut (),
) -> cu::Result {
    cu::Result::ERROR_NOT_SUPPORTED
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn cuModuleLoadDataEx(
    module: *mut cu::Module,
    image: *const (),
    numOptions: c_uint,
    options: *mut cu::JitOption,
    optionValues: *mut *mut (),
) -> cu::Result {
    cu::Result::ERROR_NOT_SUPPORTED
}

#[no_mangle]
pub extern "C" fn cuMemcpyHtoD_v2(
    dst_device: cu::DevicePtr,
    src_host: *const c_void,
    byte_count: usize,
) -> cu::Result {
    if dst_device.0 == 0 || src_host == ptr::null_mut() || byte_count == 0 {
        return cu::Result::ERROR_INVALID_VALUE;
    }
    // TODO: choose the right device from device ptr
    Context::call(|drv| {
        let cu_dev = &mut drv.devices[0];
        // Safe, because there's no Drop impl for device
        let dev = unsafe { l0::Device::from_ffi(cu_dev.base.as_ffi()) };
        let queue = &mut cu_dev.default_queue;
        let result = unsafe {
            memcpy_impl(
                &mut drv.base,
                dst_device.0 as *mut _,
                src_host,
                byte_count,
                &dev,
                queue,
            )
        };
        match result {
            Ok(()) => l0_sys::ze_result_t::ZE_RESULT_SUCCESS,
            Err(e) => e.0,
        }
    })
}

unsafe fn memcpy_impl(
    ctx: &mut l0::Context,
    dst: *mut c_void,
    src: *const c_void,
    bytes_count: usize,
    dev: &l0::Device,
    queue: &mut l0::CommandQueue,
) -> l0::Result<()> {
    let mut cmd_list = l0::CommandList::new(ctx, &dev)?;
    cmd_list.append_memory_copy_unsafe(dst, src, bytes_count, None, &mut [])?;
    queue.execute(cmd_list)?;
    Ok(())
}

#[no_mangle]
pub extern "C" fn cuCtxCreate_v2(
    pctx: *mut cu::Context,
    flags: c_uint,
    dev: cu::Device,
) -> cu::Result {
    if pctx == ptr::null_mut() {
        return cu::Result::ERROR_INVALID_VALUE;
    }
    cu::Result::ERROR_NOT_SUPPORTED
}

#[no_mangle]
pub extern "C" fn cuModuleGetFunction(
    hfunc: *mut cu::Function,
    hmod: cu::Module,
    name: *const c_char,
) -> cu::Result {
    cu::Result::ERROR_NOT_SUPPORTED
}

#[no_mangle]
pub extern "C" fn cuDevicePrimaryCtxRetain(pctx: *mut cu::Context, dev: cu::Device) -> cu::Result {
    cu::Result::SUCCESS
}

#[no_mangle]
pub extern "C" fn cuCtxGetDevice(dev: *mut cu::Device) -> cu::Result {
    unsafe { *dev = cu::Device(0) };
    cu::Result::SUCCESS
}
