use hip_runtime_sys::*;
use std::{
    ffi::{CStr, CString},
    mem,
};

pub mod cache;
pub mod kernel_metadata;
pub mod raytracing;
#[allow(dead_code)]
mod zluda_capnp;
pub mod zluda_ext;
#[allow(dead_code)]
mod zluda_rt6_capnp;

#[macro_export]
macro_rules! hip {
    ($expr:expr) => {
        #[allow(unused_unsafe)]
        {
            let err = unsafe { $expr };
            if err != hip_runtime_sys::hipError_t::hipSuccess {
                return Result::Err(err);
            }
        }
    };
}

#[macro_export]
macro_rules! cuda {
    ($expr:expr) => {
        #[allow(unused_unsafe)]
        {
            let err = unsafe { $expr };
            if err != cuda_types::CUresult::CUDA_SUCCESS {
                return Result::Err(err);
            }
        }
    };
}

#[macro_export]
macro_rules! unwrap_or_return {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return,
        }
    };
    ( $e:expr, $err:expr ) => {
        match $e {
            Some(x) => x,
            None => return $err,
        }
    };
}

#[macro_export]
macro_rules! unwrap_or_continue {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => continue,
        }
    };
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CompilationMode {
    Wave32 = 1,
    Wave32OnWave64 = 2,
    DoubleWave32OnWave64 = 3,
}

impl CompilationMode {
    pub unsafe fn from_device(device: i32) -> Result<CompilationMode, hipError_t> {
        let mut device_props = mem::zeroed();
        hip! { hipGetDeviceProperties(&mut device_props, device) };
        if device_props.warpSize == 32 {
            Ok(CompilationMode::Wave32)
        } else {
            Ok(CompilationMode::Wave32OnWave64)
        }
    }

    pub fn from_u8(value: u8) -> Option<Self> {
        Some(match value {
            1 => CompilationMode::Wave32,
            2 => CompilationMode::Wave32OnWave64,
            3 => CompilationMode::DoubleWave32OnWave64,
            _ => return None,
        })
    }
}

pub unsafe fn comgr_isa(device: i32) -> Result<CString, hipError_t> {
    let mut device_props = mem::zeroed();
    hip! { hipGetDeviceProperties(&mut device_props, device) };
    let gcn_arch = CStr::from_ptr(device_props.gcnArchName.as_ptr() as _);
    let mut arch_name = b"amdgcn-amd-amdhsa--".to_vec();
    arch_name.extend_from_slice(gcn_arch.to_bytes_with_nul());
    Ok(CString::from_vec_with_nul_unchecked(arch_name))
}

pub mod elf {
    use goblin::elf::header::header64::Header;

    pub unsafe fn as_slice(ptr: *const u8) -> &'static [u8] {
        let header = Header::from_bytes(&*(ptr as *mut [u8; 64]));
        let size =
            header.e_shoff as usize + (header.e_shnum as usize * header.e_shentsize as usize);
        // TODO: enumerate sections to check if there's a section beyond
        // sections header table
        std::slice::from_raw_parts(ptr as _, size)
    }
}
