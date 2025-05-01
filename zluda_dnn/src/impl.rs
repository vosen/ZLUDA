use cuda_types::cudnn9::cudnnStatus_t;

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> cudnnStatus_t {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> cudnnStatus_t {
    cudnnStatus_t::CUDNN_STATUS_NOT_SUPPORTED
}

#[allow(non_snake_case)]
pub(crate) fn cudnnGetVersion() -> usize {
    todo!()
}
#[allow(non_snake_case)]
pub(crate) fn cudnnGetMaxDeviceVersion() -> usize {
    todo!()
}
#[allow(non_snake_case)]
pub(crate) fn cudnnGetCudartVersion() -> usize {
    todo!()
}
#[allow(non_snake_case)]
pub(crate) fn cudnnGetErrorString(
    _status: cuda_types::cudnn9::cudnnStatus_t,
) -> *const ::core::ffi::c_char {
    todo!()
}
#[allow(non_snake_case)]
pub(crate) fn cudnnGetLastErrorString(_message: *mut ::core::ffi::c_char, _max_size: usize) -> () {
    todo!()
}
