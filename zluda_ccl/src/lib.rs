#[allow(warnings)]
mod nccl;
pub use nccl::*;

#[cfg(debug_assertions)]
pub(crate) fn unsupported() -> ncclResult_t {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unsupported() -> ncclResult_t {
    ncclResult_t::ncclInternalError
}
