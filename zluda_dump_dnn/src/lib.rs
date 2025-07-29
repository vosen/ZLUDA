use libloading::Library;
use std::sync::LazyLock;

static LIBRARY: LazyLock<Option<Library>> = LazyLock::new(get_library);

fn get_library() -> Option<Library> {
    let cuda_lib = std::env::var("ZLUDA_DNN_LIB")
        .ok()
        .unwrap_or_else(|| "/usr/lib/x86_64-linux-gnu/libcudnn.so.9".to_string());
    zluda_dump_common::dlopen_local_noredirect(cuda_lib).ok()
}

macro_rules! unimplemented {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes)]
            #[allow(improper_ctypes_definitions)]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                use ::zluda_dump_common::ReprUsize;
                let internal_error_untyped: usize = <$ret_type as ReprUsize>::INTERNAL_ERROR;
                let internal_error: $ret_type = ReprUsize::from_usize(internal_error_untyped);
                let maybe_fn_ptr = (&*LIBRARY).as_ref().and_then(|lib| lib.get::<unsafe extern $abi fn ( $($arg_type),* ) -> $ret_type>(concat!( stringify!($fn_name), "\0").as_bytes()).ok());
                let fn_ptr = unwrap_or::unwrap_some_or!(maybe_fn_ptr, return internal_error);
                let export_table = unwrap_or::unwrap_some_or!(::zluda_dump_common::get_export_table(), return internal_error);
                let format_args = dark_api::FnFfiWrapper(|| {
                    let mut writer = Vec::new();
                    let formatter = paste::paste! { ::format:: [< write_  $fn_name>] };
                    formatter(&mut writer $(, $arg_id)* ).ok();
                    dark_api::ByteVecFfi::new(writer)
                });
                let underlying_fn = dark_api::FnFfiWrapper(|| {
                    let result = fn_ptr(  $( $arg_id),* );
                    ReprUsize::to_usize(result)
                });
                ReprUsize::from_usize(export_table.logged_call(
                    cglue::slice::CSliceRef::from_str(stringify!($fn_name)),
                    cglue::trait_obj!(&format_args as dark_api::FnFfi),
                    cglue::trait_obj!(&underlying_fn as dark_api::FnFfi),
                    internal_error_untyped,
                    <$ret_type as ReprUsize>::format_status)
                )
            }
        )*
    };
}

cuda_macros::cudnn9_function_declarations!(unimplemented);
