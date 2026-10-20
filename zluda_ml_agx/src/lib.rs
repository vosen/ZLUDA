//! NVML for Apple GPUs on Asahi Linux: the device queries monitoring tools and frameworks make (count, name, UUID,
//! memory, compute capability, driver versions), answered from the Asahi DRM parameters. Everything else returns
//! `NVML_ERROR_NOT_SUPPORTED`; in particular utilisation, clocks, power and temperature, which the Asahi driver does
//! not expose to userspace.

#![allow(non_snake_case)]

use agx_runtime::context::DeviceProps;
use cuda_types::nvml::*;
use std::ffi::{c_char, c_int, c_uint, CStr};
use std::sync::Mutex;

macro_rules! unimplemented_fn {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes_definitions, unused_variables)]
            pub extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                Err(nvmlError_t::NOT_SUPPORTED)
            }
        )*
    };
}

macro_rules! implemented_fn {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes_definitions)]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                imp::$fn_name($($arg_id),*)
            }
        )*
    };
}

cuda_macros::nvml_function_declarations!(
    unimplemented_fn,
    implemented_fn
        <= [
            nvmlDeviceGetCount_v2,
            nvmlDeviceGetCudaComputeCapability,
            nvmlDeviceGetHandleByIndex_v2,
            nvmlDeviceGetMemoryInfo,
            nvmlDeviceGetName,
            nvmlDeviceGetUUID,
            nvmlErrorString,
            nvmlInitWithFlags,
            nvmlInit_v2,
            nvmlShutdown,
            nvmlSystemGetCudaDriverVersion,
            nvmlSystemGetDriverVersion,
        ]
);

mod imp {
    use super::*;

    static DEVICE: Mutex<Option<DeviceProps>> = Mutex::new(None);
    const HANDLE: usize = 1;

    fn props() -> Result<DeviceProps, nvmlError_t> {
        DEVICE.lock().unwrap().clone().ok_or(nvmlError_t::UNINITIALIZED)
    }

    fn total_memory() -> u64 {
        let mut s: libc_sysinfo = unsafe { std::mem::zeroed() };
        if unsafe { sysinfo(&mut s) } != 0 {
            return 0;
        }
        s.totalram * s.mem_unit as u64
    }

    fn free_memory() -> u64 {
        let mut s: libc_sysinfo = unsafe { std::mem::zeroed() };
        if unsafe { sysinfo(&mut s) } != 0 {
            return 0;
        }
        s.freeram * s.mem_unit as u64
    }

    // struct sysinfo of 64-bit Linux
    #[repr(C)]
    pub struct libc_sysinfo {
        uptime: i64,
        loads: [u64; 3],
        totalram: u64,
        freeram: u64,
        sharedram: u64,
        bufferram: u64,
        totalswap: u64,
        freeswap: u64,
        procs: u16,
        pad: u16,
        totalhigh: u64,
        freehigh: u64,
        mem_unit: u32,
        _f: [u8; 0],
    }
    extern "C" {
        fn sysinfo(info: *mut libc_sysinfo) -> c_int;
    }

    unsafe fn copy_str(s: &str, out: *mut c_char, len: c_uint) -> nvmlReturn_t {
        if out.is_null() {
            return Err(nvmlError_t::INVALID_ARGUMENT);
        }
        if s.len() + 1 > len as usize {
            return Err(nvmlError_t::INSUFFICIENT_SIZE);
        }
        std::ptr::copy_nonoverlapping(s.as_ptr() as *const c_char, out, s.len());
        *out.add(s.len()) = 0;
        Ok(())
    }

    fn check(device: nvmlDevice_t) -> nvmlReturn_t {
        props()?;
        if device as usize == HANDLE {
            Ok(())
        } else {
            Err(nvmlError_t::INVALID_ARGUMENT)
        }
    }

    pub unsafe fn nvmlInitWithFlags(_flags: c_uint) -> nvmlReturn_t {
        let mut d = DEVICE.lock().unwrap();
        if d.is_some() {
            return Ok(());
        }
        let node = std::env::var("ZLUDA_AGX_NODE").ok();
        let nodes: Vec<String> = match node {
            Some(n) => vec![n],
            None => (128..192).map(|n| format!("/dev/dri/renderD{n}")).collect(),
        };
        for n in nodes {
            if let Ok(dev) = agx_runtime::Device::open(&n) {
                *d = Some(DeviceProps::from_params(&dev.params, total_memory()));
                return Ok(());
            }
        }
        Err(nvmlError_t::DRIVER_NOT_LOADED)
    }

    pub unsafe fn nvmlInit_v2() -> nvmlReturn_t {
        nvmlInitWithFlags(0)
    }

    pub unsafe fn nvmlShutdown() -> nvmlReturn_t {
        *DEVICE.lock().unwrap() = None;
        Ok(())
    }

    pub unsafe fn nvmlErrorString(result: nvmlReturn_t) -> *const c_char {
        let s: &'static CStr = match result {
            Ok(()) => c"Success",
            Err(e) if e == nvmlError_t::UNINITIALIZED => c"Uninitialized",
            Err(e) if e == nvmlError_t::INVALID_ARGUMENT => c"Invalid Argument",
            Err(e) if e == nvmlError_t::NOT_SUPPORTED => c"Not Supported",
            Err(e) if e == nvmlError_t::INSUFFICIENT_SIZE => c"Insufficient Size",
            Err(e) if e == nvmlError_t::DRIVER_NOT_LOADED => c"Driver Not Loaded",
            Err(_) => c"Unknown Error",
        };
        s.as_ptr()
    }

    pub unsafe fn nvmlSystemGetDriverVersion(version: *mut c_char, length: c_uint) -> nvmlReturn_t {
        copy_str("580.00 (ZLUDA AGX)", version, length)
    }

    pub unsafe fn nvmlSystemGetCudaDriverVersion(v: *mut c_int) -> nvmlReturn_t {
        if v.is_null() {
            return Err(nvmlError_t::INVALID_ARGUMENT);
        }
        *v = 12040;
        Ok(())
    }

    pub unsafe fn nvmlDeviceGetCount_v2(count: *mut c_uint) -> nvmlReturn_t {
        props()?;
        if count.is_null() {
            return Err(nvmlError_t::INVALID_ARGUMENT);
        }
        *count = 1;
        Ok(())
    }

    pub unsafe fn nvmlDeviceGetHandleByIndex_v2(index: c_uint, device: *mut nvmlDevice_t) -> nvmlReturn_t {
        props()?;
        if index != 0 || device.is_null() {
            return Err(nvmlError_t::INVALID_ARGUMENT);
        }
        *device = HANDLE as nvmlDevice_t;
        Ok(())
    }

    pub unsafe fn nvmlDeviceGetName(device: nvmlDevice_t, name: *mut c_char, length: c_uint) -> nvmlReturn_t {
        check(device)?;
        copy_str(&props()?.name, name, length)
    }

    pub unsafe fn nvmlDeviceGetUUID(device: nvmlDevice_t, uuid: *mut c_char, length: c_uint) -> nvmlReturn_t {
        check(device)?;
        // stable per chip model: GPU-<llvm cpu>-<core count>
        let p = props()?;
        copy_str(&format!("GPU-agx-{}-{:08x}", p.llvm_cpu, p.cores), uuid, length)
    }

    pub unsafe fn nvmlDeviceGetMemoryInfo(device: nvmlDevice_t, memory: *mut nvmlMemory_t) -> nvmlReturn_t {
        check(device)?;
        if memory.is_null() {
            return Err(nvmlError_t::INVALID_ARGUMENT);
        }
        // The GPU shares system memory.
        let (total, free) = (total_memory(), free_memory());
        *memory = nvmlMemory_t { total, free, used: total.saturating_sub(free) };
        Ok(())
    }

    pub unsafe fn nvmlDeviceGetCudaComputeCapability(device: nvmlDevice_t, major: *mut c_int, minor: *mut c_int) -> nvmlReturn_t {
        check(device)?;
        if major.is_null() || minor.is_null() {
            return Err(nvmlError_t::INVALID_ARGUMENT);
        }
        // the same as zluda_agx's cuDeviceComputeCapability
        *major = 8;
        *minor = 6;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queries_before_init_fail_and_errors_have_names() {
        unsafe {
            let mut n = 0;
            assert_eq!(nvmlDeviceGetCount_v2(&mut n), Err(nvmlError_t::UNINITIALIZED));
            let s = CStr::from_ptr(nvmlErrorString(Err(nvmlError_t::NOT_SUPPORTED)));
            assert_eq!(s.to_str().unwrap(), "Not Supported");
            let mut v = [0 as c_char; 64];
            nvmlSystemGetDriverVersion(v.as_mut_ptr(), 64).unwrap();
            assert!(nvmlSystemGetDriverVersion(v.as_mut_ptr(), 4).is_err());
        }
    }
}
