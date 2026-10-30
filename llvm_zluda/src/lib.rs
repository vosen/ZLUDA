#![allow(non_upper_case_globals)]

pub use llvm_sys::*;

#[cfg(feature = "amdgpu")]
pub mod compile;
#[cfg(feature = "agx")]
pub mod compile_agx;
#[cfg(all(feature = "agx", zluda_air))]
mod agx_merge;
#[cfg(all(feature = "agx", zluda_air))]
pub mod compile_air;
mod ffi;
pub mod utils;

pub use ffi::*;

#[cfg(feature = "amdgpu")]
pub use compile::compile;
#[cfg(feature = "agx")]
pub use compile_agx::{compile_agx, compile_agx_linked, AgxOptions};
