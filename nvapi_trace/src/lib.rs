use libloading::Library;
use std::{ffi::c_void, ptr, sync::LazyLock};
use zluda_trace_common::ReprUsize;

static LIBRARY: LazyLock<Option<Library>> = LazyLock::new(get_library);

fn get_library() -> Option<Library> {
    let cuda_lib = std::env::var("ZLUDA_NVAPI_LIB").ok().unwrap_or_else(|| {
        if cfg!(target_pointer_width = "64") {
            r"C:\Windows\System32\nvapi64.dll".to_string()
        } else {
            r"C:\Windows\System32\nvapi.dll".to_string()
        }
    });
    zluda_trace_common::dlopen_local_noredirect(cuda_lib).ok()
}

#[no_mangle]
pub unsafe extern "C" fn nvapi_QueryInterface(interface: u32) -> *mut c_void {
    let maybe_fn_ptr = (&*LIBRARY).as_ref().and_then(|lib| {
        lib.get::<unsafe extern "C" fn(interface: u32) -> *mut c_void>(b"nvapi_QueryInterface\0")
            .ok()
    });
    let fn_ptr = unwrap_or::unwrap_some_or!(maybe_fn_ptr, return ptr::null_mut());
    let export_table = unwrap_or::unwrap_some_or!(
        ::zluda_trace_common::get_export_table(),
        return ptr::null_mut()
    );
    let format_args = dark_api::FnFfiWrapper(|| {
        use std::io::Write;
        let mut writer = Vec::new();
        write!(&mut writer, "(interface: {:#010x})", interface).ok();
        dark_api::ByteVecFfi::new(writer)
    });
    let underlying_fn = dark_api::FnFfiWrapper(|| {
        let result = fn_ptr(interface);
        result as usize
    });
    ReprUsize::from_usize(export_table.logged_call(
        cglue::slice::CSliceRef::from_str("nvapi_QueryInterface"),
        cglue::trait_obj!(&format_args as dark_api::FnFfi),
        cglue::trait_obj!(&underlying_fn as dark_api::FnFfi),
        ptr::null_mut::<c_void>() as usize,
        <*mut c_void as ReprUsize>::format_status,
    ))
}
