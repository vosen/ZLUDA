use cuda_types::cuda::{CUresult, CUuuid};
use std::{ptr, sync::LazyLock};

pub fn get_export_table() -> Option<::dark_api::zluda_dump::ZludaDumpInternal> {
    static CU_GET_EXPORT_TABLE: LazyLock<
        Result<
            unsafe extern "system" fn(*mut *const ::core::ffi::c_void, *const CUuuid) -> CUresult,
            libloading::Error,
        >,
    > = LazyLock::new(|| unsafe { get_dump_table_impl() });
    let cu_get_export_table = CU_GET_EXPORT_TABLE.as_ref().ok()?;
    let mut ptr = ptr::null();
    unsafe { (cu_get_export_table)(&mut ptr, &::dark_api::zluda_dump::ZludaDumpInternal::GUID) }
        .ok()?;
    Some(unsafe { ::dark_api::zluda_dump::ZludaDumpInternal::new(ptr) })
}

unsafe fn get_dump_table_impl() -> Result<
    unsafe extern "system" fn(*mut *const ::core::ffi::c_void, *const CUuuid) -> CUresult,
    libloading::Error,
> {
    let driver = open_driver()?;
    return Ok(
        *(driver.get::<unsafe extern "system" fn(
            *mut *const ::core::ffi::c_void,
            *const CUuuid,
        ) -> CUresult>(b"cuGetExportTable\0")?),
    );
}

fn open_driver() -> Result<libloading::Library, libloading::Error> {
    os::open_driver()
}

#[cfg(unix)]
pub(crate) mod os {
    use libloading::os;

    const RTLD_NOLOAD: i32 = 0x4;

    pub fn open_driver() -> Result<libloading::Library, libloading::Error> {
        unsafe {
            os::unix::Library::open(Some("libcuda.so.1"), RTLD_NOLOAD | os::unix::RTLD_LAZY)
                .or_else(|_| {
                    os::unix::Library::open(Some("libcuda.so"), RTLD_NOLOAD | os::unix::RTLD_LAZY)
                })
                .map(Into::into)
        }
    }
}

#[cfg(windows)]
pub(crate) mod os {
    use libloading::os;

    const RTLD_NOLOAD: i32 = 0x4;

    pub fn open_driver() -> Result<libloading::Library, libloading::Error> {
        unsafe { os::windows::Library::open_already_loaded("nvcuda.dll").map(Into::into) }
    }
}

pub trait ReprUsize {
    const INTERNAL_ERROR: usize;
    fn to_usize(self) -> usize;
    fn from_usize(x: usize) -> Self;
    extern "C" fn format_status(x: usize) -> Vec<u8>;
}

impl ReprUsize for usize {
    fn to_usize(self) -> usize {
        self
    }

    fn from_usize(x: usize) -> usize {
        x
    }

    const INTERNAL_ERROR: usize = 0;

    extern "C" fn format_status(x: usize) -> Vec<u8> {
        x.to_string().into_bytes()
    }
}

impl<T> ReprUsize for *const T {
    fn to_usize(self) -> usize {
        self as usize
    }

    fn from_usize(x: usize) -> Self {
        x as Self
    }

    const INTERNAL_ERROR: usize = 0;

    extern "C" fn format_status(x: usize) -> Vec<u8> {
        todo!()
    }
}

impl ReprUsize for cuda_types::cublas::cublasStatus_t {
    fn to_usize(self) -> usize {
        self.0 as usize
    }

    fn from_usize(x: usize) -> Self {
        Self(x as u32)
    }

    const INTERNAL_ERROR: usize = 0;

    extern "C" fn format_status(x: usize) -> Vec<u8> {
        let mut result = Vec::new();
        format::CudaDisplay::write(&Self::from_usize(x), "", 0, &mut result).ok();
        result
    }
}

impl ReprUsize for cuda_types::cudnn9::cudnnStatus_t {
    fn to_usize(self) -> usize {
        self.0 as usize
    }

    fn from_usize(x: usize) -> Self {
        Self(x as u32)
    }

    const INTERNAL_ERROR: usize = 0;

    extern "C" fn format_status(x: usize) -> Vec<u8> {
        let mut result = Vec::new();
        format::CudaDisplay::write(&Self::from_usize(x), "", 0, &mut result).ok();
        result
    }
}

impl ReprUsize for () {
    fn to_usize(self) -> usize {
        0
    }

    fn from_usize(_x: usize) -> Self {
        ()
    }

    const INTERNAL_ERROR: usize = 0;

    extern "C" fn format_status(x: usize) -> Vec<u8> {
        todo!()
    }
}
