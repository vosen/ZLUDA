#![allow(warnings)]
pub mod hip_runtime_api;
pub mod hip_runtime_api_ext {
    use crate::hipStream_t;

    pub const hipStreamNull: hipStream_t = 0 as _;
    pub const hipStreamPerThread: hipStream_t = 2 as _;
    pub const HIP_TRSA_OVERRIDE_FORMAT: u32 = 1;
}
pub use hip_runtime_api::*;
pub use hip_runtime_api_ext::*;
