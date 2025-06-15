use cuda_types::cufft::cufftResult_t;

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> cufftResult_t {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> cufftResult_t {
    cufftResult_t::CUFFT_NOT_SUPPORTED
}
