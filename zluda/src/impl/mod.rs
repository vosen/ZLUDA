use cuda_types::cuda::*;
use zluda_common::from_cuda_object;

pub(super) mod array;
pub(super) mod context;
#[cfg(windows)]
pub(super) mod d3d11;
pub(super) mod device;
pub(super) mod driver;
pub(super) mod event;
pub(super) mod function;
pub(super) mod graph;
#[cfg(windows)]
pub(super) mod graphics;
pub(super) mod hipfix;
pub(super) mod kernel;
pub(super) mod library;
pub(super) mod memory;
pub(super) mod module;
pub(super) mod pointer;
pub(super) mod stream;
pub(super) mod surf;
pub(super) mod tex;

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> CUerror {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> CUerror {
    CUerror::NOT_SUPPORTED
}

from_cuda_object!(module::Module, context::Context, library::Library);
