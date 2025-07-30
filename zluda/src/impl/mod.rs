use cuda_types::cuda::*;
use zluda_common::from_cuda_object;

pub(super) mod context;
pub(super) mod device;
pub(super) mod driver;
pub(super) mod function;
pub(super) mod library;
pub(super) mod memory;
pub(super) mod module;
pub(super) mod pointer;
pub(super) mod stream;

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> CUresult {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> CUresult {
    CUresult::ERROR_NOT_SUPPORTED
}

from_cuda_object!(module::Module, context::Context, library::Library);
