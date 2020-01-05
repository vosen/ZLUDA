mod cu;

#[no_mangle]
pub extern "stdcall" fn cuDriverGetVersion(version: &mut std::os::raw::c_int) -> cu::Result {
    *version = 0;
    return cu::Result::SUCCESS;
}

#[no_mangle]
pub extern "stdcall" fn cuInit(_: *const std::os::raw::c_uint) -> cu::Result {
    return cu::Result::SUCCESS;
}

#[no_mangle]
pub extern "stdcall" fn cuGetExportTable(_: *const *const std::os::raw::c_void, _: cu::Uuid) -> cu::Result {
    return cu::Result::ERROR_NOT_SUPPORTED;
}

#[no_mangle]
pub extern "stdcall" fn cuDeviceGetCount(count: &mut std::os::raw::c_int) -> cu::Result {
    *count = 1;
    return cu::Result::SUCCESS;
}
