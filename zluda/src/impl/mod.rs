use cuda_types::cuda::*;
use zluda_common::from_cuda_object;

pub(super) mod context;
pub(super) mod device;
pub(super) mod driver;
pub(super) mod event;
pub(super) mod function;
pub(super) mod graph;
pub(super) mod hipfix;
pub(super) mod kernel;
pub(super) mod library;
pub(super) mod memory;
pub(super) mod module;
pub(super) mod pointer;
pub(super) mod stream;
pub(super) mod texture;

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> CUerror {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> CUerror {
    CUerror::NOT_SUPPORTED
}

from_cuda_object!(module::Module, context::Context, library::Library);
