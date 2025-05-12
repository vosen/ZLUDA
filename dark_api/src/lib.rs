use uuid::uuid;

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
            $writer.write_all(b", ")?;
            $writer.write_all(concat!(stringify!($arg_id), ": ").as_bytes())?;
            format::CudaDisplay::write(& $arg_id, "", $arg_idx, $writer)?;
        )*
    };
    ($writer:ident; $arg_idx:ident; ) => {
    };
}

macro_rules! dark_api_format_fn {
    (SIZE_OF) => { };
    (NULL) => { };
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
    ($(
        $guid:expr => $name:ident [$len:literal] {
            $( [$index:literal] = $fn_:ident $( ( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty )? ),*
        }
    ),+) => {
        #[allow(non_snake_case)]
        pub struct CudaDarkApiGlobalTable {
            $(pub $name: [*const std::ffi::c_void; $len],)+
        }

        impl CudaDarkApiGlobalTable {
            $(const $name: cuda_types::cuda::CUuuid = cuda_types::cuda::CUuuid { bytes: *uuid!($guid).as_bytes() };)+
        }

        unsafe impl Sync for CudaDarkApiGlobalTable {}

        impl CudaDarkApiGlobalTable {
            pub fn new<T: CudaDarkApi>() -> Self {
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

            pub fn get(&self, key: &cuda_types::cuda::CUuuid) -> Option<&[*const std::ffi::c_void]> {
                match key {
                    $(&Self::$name => Some(&self.$name[..]),)+
                    _ => None
                }
            }
        }

        pub trait CudaDarkApi {
            $($(
                dark_api_fn!($fn_ $( ( $($arg_id: $arg_type),* ) -> $ret_type )?);
            )*)+
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
                dark_api_format_fn!($fn_ $( ( $($arg_id: $arg_type),* ) -> $ret_type )? );
            )*)+
        }
    };
}

dark_api! {
    "{7330F811-F47F-41BC-A4FF-E792D073F41F}" => CUDART_INTERFACE[10] {
        [0] = SIZE_OF,
        [1] = cudart_interface_fn1(foobar: usize, baz: i32) -> (),
        [2] = get_module_from_cubin() -> (),
        [6] = get_module_from_cubin_ext1(foobar: usize) -> (),
        [7] = cudart_interface_fn6(foobar: usize) -> (),
        [8] = get_module_from_cubin_ext2(foobar: usize) -> ()
    },
    "{7330F811-F47F-41BC-A4FF-E792D073F42F}" => CONTEXT_LOCAL_STORAGE_INTERFACE_V0301[4] {
        [0] = context_local_storage_ctor(foobar: usize) -> (),
        [1] = context_local_storage_dtor(foobar: usize) -> (),
        [2] = context_local_storage_get_state(foobar: usize) -> ()
    }
}
