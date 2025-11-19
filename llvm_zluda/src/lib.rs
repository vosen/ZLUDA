#![allow(non_upper_case_globals)]

pub use llvm_sys::*;

pub mod compile;
mod ffi;
pub mod utils;

pub use ffi::*;

pub use compile::compile;
