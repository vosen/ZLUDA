//! NVML for Apple GPUs on Asahi Linux: the device queries monitoring tools and frameworks make (count, name, UUID,
//! memory, compute capability, driver versions), answered from the Asahi DRM parameters, the maximum clock, and the
//! GPU utilisation of this process's kernels. Everything else returns `NVML_ERROR_NOT_SUPPORTED`; in particular the
//! current clock, power and temperature, which the Asahi driver does not expose without root (ptx-re-blockers B25).

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
            nvmlDeviceGetClockInfo,
            nvmlDeviceGetHandleByIndex_v2,
            nvmlDeviceGetMaxClockInfo,
            nvmlDeviceGetMemoryInfo,
            nvmlDeviceGetName,
            nvmlDeviceGetUUID,
            nvmlDeviceGetUtilizationRates,
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
                // the first utilisation sample covers the time since initialisation
                *SAMPLE.lock().unwrap() = Some((std::time::Instant::now(), busy_ns()));
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

    /// The shader clock is the device's maximum frequency (the DRM's `max_frequency_khz`); Asahi gives no other.
    pub unsafe fn nvmlDeviceGetMaxClockInfo(device: nvmlDevice_t, type_: nvmlClockType_t, clock: *mut c_uint) -> nvmlReturn_t {
        check(device)?;
        if clock.is_null() {
            return Err(nvmlError_t::INVALID_ARGUMENT);
        }
        match type_ {
            nvmlClockType_t::NVML_CLOCK_GRAPHICS | nvmlClockType_t::NVML_CLOCK_SM => {
                *clock = props()?.clock_khz / 1000;
                Ok(())
            }
            _ => Err(nvmlError_t::NOT_SUPPORTED),
        }
    }

    /// The current clock is not exposed (B25: no devfreq or clock readout without root).
    pub unsafe fn nvmlDeviceGetClockInfo(device: nvmlDevice_t, _type: nvmlClockType_t, clock: *mut c_uint) -> nvmlReturn_t {
        check(device)?;
        if clock.is_null() {
            return Err(nvmlError_t::INVALID_ARGUMENT);
        }
        Err(nvmlError_t::NOT_SUPPORTED)
    }

    /// GPU time of this process's finished kernels (ZLUDA's `zluda_agx_gpu_busy_ns`, from the firmware's command
    /// timestamps), or 0 when the process does not use zluda_agx.
    fn busy_ns() -> u64 {
        type Busy = extern "C" fn() -> u64;
        static F: std::sync::OnceLock<Option<Busy>> = std::sync::OnceLock::new();
        let f = F.get_or_init(|| {
            let p = unsafe { libc::dlsym(libc::RTLD_DEFAULT, c"zluda_agx_gpu_busy_ns".as_ptr()) };
            (!p.is_null()).then(|| unsafe { std::mem::transmute::<*mut libc::c_void, Busy>(p) })
        });
        f.map_or(0, |f| f())
    }

    static SAMPLE: Mutex<Option<(std::time::Instant, u64)>> = Mutex::new(None);

    /// Percent of the time since the last call (or since NVML was initialised) the GPU spent on this process's
    /// kernels, capped at 100 (kernels of several streams overlap). Kernels of other processes are not seen.
    pub(super) fn utilization(now: std::time::Instant, busy: u64, last: &mut Option<(std::time::Instant, u64)>) -> u32 {
        let pct = match *last {
            Some((t, b)) => {
                let wall = now.duration_since(t).as_nanos().max(1);
                ((busy.saturating_sub(b) as u128 * 100) / wall).min(100) as u32
            }
            None => 0,
        };
        *last = Some((now, busy));
        pct
    }

    pub unsafe fn nvmlDeviceGetUtilizationRates(device: nvmlDevice_t, u: *mut nvmlUtilization_t) -> nvmlReturn_t {
        check(device)?;
        if u.is_null() {
            return Err(nvmlError_t::INVALID_ARGUMENT);
        }
        let gpu = utilization(std::time::Instant::now(), busy_ns(), &mut SAMPLE.lock().unwrap());
        // memory activity is not measured
        *u = nvmlUtilization_t { gpu, memory: 0 };
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
    fn utilization_is_the_busy_share_of_the_interval() {
        let t0 = std::time::Instant::now();
        let mut last = None;
        assert_eq!(imp::utilization(t0, 5, &mut last), 0, "no interval yet");
        let t1 = t0 + std::time::Duration::from_millis(100);
        assert_eq!(imp::utilization(t1, 5 + 25_000_000, &mut last), 25);
        let t2 = t1 + std::time::Duration::from_millis(10);
        assert_eq!(imp::utilization(t2, 5 + 25_000_000 + 30_000_000, &mut last), 100, "overlapping streams are capped");
        let t3 = t2 + std::time::Duration::from_millis(10);
        assert_eq!(imp::utilization(t3, 5 + 55_000_000, &mut last), 0, "idle");
    }

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
