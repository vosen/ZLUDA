macro_rules! dark_api_init {
    (SIZE_OF, $table_len:literal, $type_:ty) => {
        (std::mem::size_of::<usize>() * $table_len) as *const std::ffi::c_void
    };
    (NULL, $table_len:literal, $type_:ty) => {
        std::ptr::null()
    };
    ($fn_:ident, $table_len:literal, $type_:ty) => {
        <$type_>::$fn_ as *const std::ffi::c_void
    };
}

macro_rules! dark_api_fn {
    (SIZE_OF) => { };
    (NULL) => { };
    ($fn_:ident ( $($arg_id:ident: $arg_type:ty),* ) -> $ret_type:ty) => {
        unsafe extern "system" fn $fn_(
            $($arg_id : $arg_type,)*
        ) -> $ret_type;
    }
}

macro_rules! dark_api_entry {
    ($idx:literal, SIZE_OF) => { };
    ($idx:literal, NULL) => { };
    ($idx:literal, $fn_:ident ( $($arg_id:ident: $arg_type:ty),* ) -> $ret_type:ty) => {
        #[allow(non_snake_case)]
        pub unsafe fn $fn_(
            &self,
            $($arg_id : $arg_type,)*
        ) -> $ret_type {
            let ptr = self.ptr as *const *const std::ffi::c_void;
            let ptr = ptr.add($idx);
            let fn_ = std::mem::transmute::<_, unsafe extern "system" fn( $($arg_type,)* ) -> $ret_type >(*ptr);
            (fn_)( $($arg_id,)* )
        }
    }
}

macro_rules! dark_api_format_args {
    ($writer:ident; $arg_idx:ident; $first_arg:ident $(, $arg_id:ident)*) => {
        $writer.write_all(concat!(stringify!($first_arg), ": ").as_bytes())?;
        format::CudaDisplay::write(& $first_arg, "", $arg_idx, $writer)?;
        $(
            $arg_idx += 1;
            $writer.write_all(concat!(", ", stringify!($arg_id), ": ").as_bytes())?;
            format::CudaDisplay::write(& $arg_id, "", $arg_idx, $writer)?;
        )*
    };
    ($writer:ident; $arg_idx:ident; ) => {
    };
}

macro_rules! dark_api_is_fn {
    (SIZE_OF) => {
        false
    };
    (NULL) => {
        false
    };
    ($fn_:ident) => {
        true
    };
}

macro_rules! dark_api_format_fn {
    (SIZE_OF) => { };
    (NULL) => { };
    (#[noformat] $fn_:ident ( $($arg_id:ident: $arg_type:ty),* ) -> $ret_type:ty) => { };
    ($fn_:ident ( $($arg_id:ident: $arg_type:ty),* ) -> $ret_type:ty) => {
        pub fn $fn_ (
            writer: &mut (impl std::io::Write + ?Sized),
            $($arg_id: $arg_type,)*
        ) -> std::io::Result<()> {
            #[allow(unused)]
            let mut arg_idx = 0usize;
            writer.write_all(b"(")?;
            dark_api_format_args!(writer; arg_idx; $($arg_id),*);
            writer.write_all(b")")
        }
    }
}

macro_rules! dark_api {
    (
        $mod_name: ident;
        $(
            $guid:expr => $name:ident [$len:literal] {
                $(
                    $(#[$attr:ident])?
                    [$index:literal] = $fn_:ident $( ( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty )?
                ),*
            }
        ),+
    ) => {
        pub mod $mod_name {
        #[allow(non_snake_case)]
        pub struct CudaDarkApiGlobalTable {
            $(pub $name: [*const std::ffi::c_void; $len],)+
        }

        impl CudaDarkApiGlobalTable {
            $(const $name: cuda_types::cuda::CUuuid = cuda_types::cuda::CUuuid { bytes: *uuid::uuid!($guid).as_bytes() };)+
        }

        unsafe impl Sync for CudaDarkApiGlobalTable {}

        impl CudaDarkApiGlobalTable {
            pub const fn new<T: CudaDarkApi>() -> Self {
                let mut result = Self {
                    $(
                        $name: [std::ptr::null(); $len],
                    )+
                };
                $(
                    $( result.$name[$index] =  dark_api_init!($fn_, $len, T); )*
                )+
                result
            }

            pub fn get(&self, key: &cuda_types::cuda::CUuuid) -> Option<crate::DarkApiTable> {
                match key {
                    $(
                        &Self::$name => {
                            let fns = &self.$name[..];
                            let mut valid_fns = bit_vec::BitVec::from_elem($len, false);
                            $(
                                valid_fns.set($index, dark_api_is_fn!($fn_) );
                            )*
                            Some(crate::DarkApiTable {
                                fns,
                                valid_fns
                            })
                        }
                    )+
                    _ => None
                }
            }
        }

        pub trait CudaDarkApi {
            $($(
                dark_api_fn!($fn_ $( ( $($arg_id: $arg_type),* ) -> $ret_type )?);
            )*)+
        }

        pub fn guid_to_name(guid: &cuda_types::cuda::CUuuid, index: usize) -> Option<(&'static str, Option<&'static str>)> {
            let guid = uuid::Uuid::from_bytes(guid.bytes);
            $(
                if guid == uuid::uuid!($guid) {
                    let guid = stringify!($name);
                    $(
                        if index == $index {
                            return Some((guid, Some(stringify!($fn_))));
                        }
                    )*
                    return Some((guid, None));
                }
            )+
            None
        }

        $(
            paste::paste! {
                pub struct [<$name:camel>] {
                    #[allow(dead_code)]
                    ptr: *const std::ffi::c_void
                }

                impl [<$name:camel>] {
                    pub const GUID: cuda_types::cuda::CUuuid = CudaDarkApiGlobalTable::$name;

                    pub unsafe fn new(ptr: *const std::ffi::c_void) -> Self {
                        Self {
                            ptr
                        }
                    }
                    $(
                        dark_api_entry!($index, $fn_ $( ( $($arg_id: $arg_type),* ) -> $ret_type )?);
                    )*
                }
            }
        )+

        pub mod format {
            $($(
                dark_api_format_fn!($(#[$attr])? $fn_ $( ( $($arg_id: $arg_type),* ) -> $ret_type )? );
            )*)+
        }
        }
    };
}

pub struct DarkApiTable<'a> {
    fns: &'a [*const std::ffi::c_void],
    valid_fns: bit_vec::BitVec,
}

impl<'a> DarkApiTable<'a> {
    pub fn len(&self) -> usize {
        self.fns.len()
    }

    pub fn get_fn(&self, idx: usize) -> Option<*const std::ffi::c_void> {
        if self.valid_fns.get(idx).unwrap_or(false) {
            Some(self.fns[idx])
        } else {
            None
        }
    }

    pub fn start(&self) -> *const std::ffi::c_void {
        self.fns.as_ptr().cast()
    }
}

dark_api! {
    cuda;
    "{6BD5FB6C-5BF4-E74A-8987-D93912FD9DF9}" => CUDART_INTERFACE[10] {
        [0] = SIZE_OF,
        [1] = get_module_from_cubin(
            module: *mut cuda_types::cuda::CUmodule,
            fatbinc_wrapper: *const std::ffi::c_void // FatbincWrapper
        ) -> (),
        [2] = cudart_interface_fn2(
            pctx: *mut cuda_types::cuda::CUcontext,
            dev: cuda_types::cuda::CUdevice
        ) -> cuda_types::cuda::CUresult,
        [6] = get_module_from_cubin_ext1(
            result: *mut cuda_types::cuda::CUmodule,
            fatbinc_wrapper: *const std::ffi::c_void, // FatbincWrapper
            arg3: *mut std::ffi::c_void,
            arg4: *mut std::ffi::c_void,
            arg5: u32
        ) -> cuda_types::cuda::CUresult,
        [7] = cudart_interface_fn7(arg1: usize) -> cuda_types::cuda::CUresult,
        [8] = get_module_from_cubin_ext2(
            fatbinc_wrapper: *const std::ffi::c_void, // FatbinHeader
            result: *mut cuda_types::cuda::CUmodule,
            arg3: *mut std::ffi::c_void,
            arg4: *mut std::ffi::c_void,
            arg5: u32
        ) -> cuda_types::cuda::CUresult
    },
    "{42D85A81-23F6-CB47-8298-F6E78A3AECDC}" => TOOLS_TLS[4] {
        [0] = SIZE_OF
    },
    "{A094798C-2E74-2E74-93F2-0800200C0A66}" => TOOLS_RUNTIME_CALLBACK_HOOKS[7] {
        [0] = SIZE_OF,
        [2] = runtime_callback_hooks_fn2(ptr: *mut *mut std::ffi::c_void, size: *mut usize) -> (),
        [6] = runtime_callback_hooks_fn6(ptr: *mut *mut std::ffi::c_void, size: *mut usize) -> ()
    },
    "{C693336E-1121-DF11-A8C3-68F355D89593}" => CONTEXT_LOCAL_STORAGE_INTERFACE_V0301[4] {
        [0] = context_local_storage_ctor(
            context: cuda_types::cuda::CUcontext,
            manager: *mut std::ffi::c_void, // ContextStateManager
            ctx_state: *mut std::ffi::c_void, // ContextState
            // clsContextDestroyCallback, have to be called on cuDevicePrimaryCtxReset
            dtor_cb: Option<extern "system" fn(
                cuda_types::cuda::CUcontext,
                *mut std::ffi::c_void, // ContextStateManager
                *mut std::ffi::c_void, // ContextState
            )>
        ) -> cuda_types::cuda::CUresult,
        [1] = context_local_storage_dtor(
            arg1: *mut std::ffi::c_void,
            arg2: *mut std::ffi::c_void
        ) -> cuda_types::cuda::CUresult,
        [2] = context_local_storage_get_state(
            ctx_state: *mut std::ffi::c_void, // ContextState
            cu_ctx: cuda_types::cuda::CUcontext,
            manager: *mut std::ffi::c_void // ContextStateManager
        ) -> cuda_types::cuda::CUresult
    },
    "{0CA50B8C-1004-929A-89A7-D0DF10E77286}" => CTX_CREATE_BYPASS[2] {
        [0] = SIZE_OF,
        [1] = ctx_create_v2_bypass(
            pctx: *mut cuda_types::cuda::CUcontext,
            flags: ::std::os::raw::c_uint,
            dev: cuda_types::cuda::CUdevice
        ) -> cuda_types::cuda::CUresult
    },
    "{195BCBF4-D67D-024A-ACC5-1D29CEA631AE}" => HEAP_ACCESS[3] {
        [0] = SIZE_OF,
        [1] = heap_alloc(
            heap_alloc_record_ptr: *mut *const std::ffi::c_void, // HeapAllocRecord
            arg2: usize,
            arg3: usize
        ) -> cuda_types::cuda::CUresult,
        [2] = heap_free(
            heap_alloc_record_ptr: *const std::ffi::c_void, // HeapAllocRecord
            arg2: *mut usize
        ) -> cuda_types::cuda::CUresult
    },
    "{B10541E1-F7C7-C74A-9F64-F223BE99F1E2}" => DEVICE_EXTENDED_RT[26] {
        [0] = SIZE_OF,
        [5] = device_get_attribute_ext(
            dev: cuda_types::cuda::CUdevice,
            attribute: std::ffi::c_uint,
            unknown: std::ffi::c_int,
            result: *mut [usize; 2]
        ) -> cuda_types::cuda::CUresult,
        // I don't know is this function return, but on my GTX 1060 it returns 0
        [13] = device_get_something(
            result: *mut std::ffi::c_uchar,
            dev: cuda_types::cuda::CUdevice
        ) -> cuda_types::cuda::CUresult
    }
}

// Purely for internal use by ZLUDA dump
dark_api! {
    zluda_dump;
    "{0B7A5827-AF98-46AB-A951-22D19BDF5C08}" => ZLUDA_DUMP_INTERNAL[1] {
        #[noformat]
        [0] = logged_call(
            fn_name: &'static str,
            args: &dyn Fn() -> Vec<u8>,
            fn_: &dyn Fn() -> usize,
            internal_error: usize,
            format_status: extern "C" fn(usize) -> Vec<u8>
        ) -> usize
    }
}
