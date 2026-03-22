mod r#impl;

macro_rules! unimplemented {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes)]
            #[allow(improper_ctypes_definitions)]
            #[allow(unused_variables)]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                crate::r#impl::unimplemented()
            }
        )*
    };
}

macro_rules! implemented {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes)]
            #[allow(improper_ctypes_definitions)]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                cuda_macros::cusparse_normalize_fn!( crate::r#impl::$fn_name ) ($(zluda_common::FromCuda::<_, cuda_types::cusparse::cusparseError_t>::from_cuda(&$arg_id)?),*)?;
                Ok(())
            }
        )*
    };
}

macro_rules! implemented_unmapped {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes)]
            #[allow(improper_ctypes_definitions)]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                cuda_macros::cusparse_normalize_fn!( crate::r#impl::$fn_name ) ( $( $arg_id ),* )
            }
        )*
    };
}

macro_rules! implemented_no_error {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes)]
            #[allow(improper_ctypes_definitions)]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                use unwrap_or::unwrap_ok_or;
                let result = cuda_macros::cusparse_normalize_fn!( crate::r#impl::$fn_name ) ($(
                    unwrap_ok_or!(
                        zluda_common::FromCuda::<_, cuda_types::cusparse::cusparseError_t>::from_cuda(&$arg_id),
                        _,
                        return std::mem::zeroed()
                    )
                ),*);
                std::mem::transmute(result)
            }
        )*
    };
}

cuda_macros::cusparse_function_declarations!(
    unimplemented,
    implemented_unmapped <= [cusparseGetErrorName, cusparseGetErrorString,],
    implemented_no_error
        <= [
            cusparseGetMatDiagType,
            cusparseGetMatFillMode,
            cusparseGetMatIndexBase,
            cusparseGetMatType,
        ],
    implemented
        <= [
            cusparseCreate,
            cusparseCreateConstCoo,
            cusparseCreateConstDnMat,
            cusparseCreateDnMat,
            cusparseDestroy,
            cusparseDestroyDnMat,
            cusparseDestroySpMat,
            cusparseSpMM_bufferSize,
            cusparseSpMM_preprocess,
            cusparseSpMM,
        ]
);

macro_rules! noop {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {};
}

#[cfg(windows)]
mod os_macro {
    macro_rules! vtable_impl {
        ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
            use rocsparse_sys::*;
            struct RocsparseVtable {
                _lib: libloading::os::windows::Library,
                $($fn_name: unsafe extern "C" fn($($arg_id: $arg_type),*) -> $ret_type,)*
            }

            impl RocsparseVtable {
                pub unsafe fn new() -> Result<Self, rocsparse_error> {
                    let hmodule = zluda_windows::try_load_from_self_or_hip_with_message(&["rocsparse.dll"]).ok_or(rocsparse_error::internal_error)?;
                    let lib = libloading::os::windows::Library::from_raw(hmodule.0 as _);
                    $(
                        let $fn_name = *lib.get::<unsafe extern "C" fn($($arg_id: $arg_type),*) -> $ret_type>(concat!(stringify!($fn_name), "\0").as_bytes()).map_err(|_| rocsparse_error::internal_error)?;
                    )*
                    Ok(Self {
                        _lib: lib,
                        $($fn_name,)*
                    })
                }

                $(
                    pub unsafe fn $fn_name(&self, $($arg_id: $arg_type),*) -> $ret_type {
                        (self.$fn_name)($($arg_id),*)
                    }
                )*
            }
        };
    }
    pub(crate) use vtable_impl;
}

#[cfg(not(windows))]
mod os_macro {
    macro_rules! vtable_impl {
        ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
            use rocsparse_sys::*;

            struct RocsparseVtable {}

            impl RocsparseVtable {
                pub unsafe fn new() -> Result<Self, rocsparse_error> {
                    Ok(Self {})
                }
            }

            impl RocsparseVtable {
                $(
                    pub unsafe fn $fn_name(&self, $($arg_id: $arg_type),*) -> $ret_type {
                        (rocsparse_sys::$fn_name)($($arg_id),*)
                    }
                )*
            }
        };
    }
    pub(crate) use vtable_impl;
}

cuda_macros::rocsparse_function_declarations!(
    noop,
    os_macro::vtable_impl
        <= [
            rocsparse_create_const_coo_descr,
            rocsparse_create_const_dnmat_descr,
            rocsparse_create_dnmat_descr,
            rocsparse_create_handle,
            rocsparse_destroy_dnmat_descr,
            rocsparse_destroy_handle,
            rocsparse_destroy_spmat_descr,
            rocsparse_get_mat_diag_type,
            rocsparse_get_mat_fill_mode,
            rocsparse_get_mat_index_base,
            rocsparse_get_mat_type,
            rocsparse_spmm_buffer_size,
            rocsparse_spmm_preprocess,
            rocsparse_spmm,
        ]
);
