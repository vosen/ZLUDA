macro_rules! export_dnn9_unmangled {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[no_mangle]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                zluda_dnn::dnn9::$fn_name( $( $arg_id ),* )
            }
        )*
    };
}

cuda_macros::cudnn9_function_declarations! {
    export_dnn9_unmangled
}

#[cfg(windows)]
mod windows {
    use zluda_windows;
    #[no_mangle]
    static __pfnDliFailureHook2: zluda_windows::PfnDliHook = delaylink_hook;

    unsafe extern "system" fn delaylink_hook(
        dli_notify: u32,
        pdli: *const zluda_windows::DelayLoadInfo,
    ) -> *mut std::ffi::c_void {
        zluda_windows::delay_load_failure_hook("MIOpen.dll", dli_notify, pdli)
            .map(|hm| hm.0 as *mut std::ffi::c_void)
            .unwrap_or(std::ptr::null_mut())
    }
}
