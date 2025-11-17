use cuda_types::{
    cublas::cublasStatus_tConsts,
    cuda::{CUerror, CUresult, CUresultConsts, CUuuid},
    cudnn9::cudnnStatus_tConsts,
    cufft::cufftResultConsts,
    cusparse::cusparseStatus_tConsts,
    nvml::nvmlReturn_tConsts,
};
use dark_api::ByteVecFfi;
use std::{borrow::Cow, ffi::c_void, num::NonZero, ptr, sync::LazyLock};

pub fn get_export_table() -> Option<::dark_api::zluda_trace::ZludaTraceInternal> {
    static CU_GET_EXPORT_TABLE: LazyLock<
        Result<
            unsafe extern "system" fn(*mut *const ::core::ffi::c_void, *const CUuuid) -> CUresult,
            libloading::Error,
        >,
    > = LazyLock::new(|| unsafe { get_export_table_impl() });
    let cu_get_export_table = CU_GET_EXPORT_TABLE.as_ref().ok()?;
    let mut ptr = ptr::null();
    unsafe { (cu_get_export_table)(&mut ptr, &::dark_api::zluda_trace::ZludaTraceInternal::GUID) }
        .ok()?;
    Some(unsafe { ::dark_api::zluda_trace::ZludaTraceInternal::new(ptr) })
}

unsafe fn get_export_table_impl() -> Result<
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

pub fn dlopen_local_noredirect<'a>(
    path: impl Into<Cow<'a, str>>,
) -> Result<libloading::Library, libloading::Error> {
    unsafe { os::dlopen_local_noredirect(path.into()) }
}

#[cfg(unix)]
pub(crate) mod os {
    use libloading::os;
    use std::borrow::Cow;

    pub fn open_driver() -> Result<libloading::Library, libloading::Error> {
        unsafe {
            os::unix::Library::open(
                Some("libcuda.so.1"),
                libc::RTLD_NOLOAD | os::unix::RTLD_LAZY,
            )
            .or_else(|_| {
                os::unix::Library::open(Some("libcuda.so"), libc::RTLD_NOLOAD | os::unix::RTLD_LAZY)
            })
            .map(Into::into)
        }
    }

    pub unsafe fn dlopen_local_noredirect<'a>(
        path: Cow<'a, str>,
    ) -> Result<libloading::Library, libloading::Error> {
        libloading::Library::new(&*path)
    }
}

#[cfg(windows)]
pub(crate) mod os {
    use libloading::os;
    use std::borrow::Cow;

    pub fn open_driver() -> Result<libloading::Library, libloading::Error> {
        os::windows::Library::open_already_loaded("nvcuda").map(Into::into)
    }

    pub unsafe fn dlopen_local_noredirect<'a>(
        path: Cow<'a, str>,
    ) -> Result<libloading::Library, libloading::Error> {
        fn terminate_with_nul(mut path: Vec<u16>) -> Vec<u16> {
            if path.last().copied() == Some(0) {
                path.push(0);
            }
            path
        }
        let redirect_dll = match os::windows::Library::open_already_loaded("zluda_redirect") {
            Ok(lib) => lib,
            Err(_) => return libloading::Library::new(&*path),
        };
        match redirect_dll.get::<unsafe extern "C" fn(*const u16) -> isize>(
            c"ZludaLoadLibraryW_NoRedirect".to_bytes_with_nul(),
        ) {
            Ok(load_library) => {
                let symbol = load_library(
                    terminate_with_nul(path.encode_utf16().collect::<Vec<u16>>()).as_ptr(),
                );
                if symbol == 0 {
                    Err(libloading::Error::LoadLibraryExWUnknown)
                } else {
                    Ok(libloading::os::windows::Library::from_raw(symbol).into())
                }
            }
            Err(_) => libloading::Library::new(&*path),
        }
    }
}

pub trait ReprUsize {
    const INTERNAL_ERROR: usize = usize::MAX;
    fn to_usize(self) -> usize;
    fn from_usize(x: usize) -> Self;
    extern "C" fn format_status(x: usize) -> ByteVecFfi;
}

impl ReprUsize for CUresult {
    const INTERNAL_ERROR: usize = CUerror::UNKNOWN.0.get() as usize;

    fn to_usize(self) -> usize {
        match self {
            CUresult::SUCCESS => 0,
            Err(err) => err.0.get() as usize,
        }
    }

    fn from_usize(x: usize) -> Self {
        match NonZero::new(x as u32) {
            None => Ok(()),
            Some(err) => Err(CUerror(err)),
        }
    }

    extern "C" fn format_status(x: usize) -> ByteVecFfi {
        let mut writer = Vec::new();
        format::CudaDisplay::write(&Self::from_usize(x), "", 0, &mut writer).ok();
        ByteVecFfi::new(writer)
    }
}

impl ReprUsize for usize {
    fn to_usize(self) -> usize {
        self
    }

    fn from_usize(x: usize) -> usize {
        x
    }

    extern "C" fn format_status(x: usize) -> ByteVecFfi {
        let mut writer = Vec::new();
        format::CudaDisplay::write(&Self::from_usize(x), "", 0, &mut writer).ok();
        ByteVecFfi::new(writer)
    }
}

impl<T> ReprUsize for *const T {
    fn to_usize(self) -> usize {
        self as usize
    }

    fn from_usize(x: usize) -> Self {
        x as Self
    }

    extern "C" fn format_status(x: usize) -> ByteVecFfi {
        let mut writer = Vec::new();
        format::CudaDisplay::write(&Self::from_usize(x).cast::<c_void>(), "", 0, &mut writer).ok();
        ByteVecFfi::new(writer)
    }
}

impl ReprUsize for cuda_types::cublas::cublasStatus_t {
    fn to_usize(self) -> usize {
        match self {
            cuda_types::cublas::cublasStatus_t::SUCCESS => 0,
            Err(err) => err.0.get() as usize,
        }
    }

    fn from_usize(x: usize) -> Self {
        match NonZero::new(x as u32) {
            None => Ok(()),
            Some(err) => Err(cuda_types::cublas::cublasError_t(err)),
        }
    }

    const INTERNAL_ERROR: usize =
        cuda_types::cublas::cublasError_t::INTERNAL_ERROR.0.get() as usize;

    extern "C" fn format_status(x: usize) -> ByteVecFfi {
        let mut writer = Vec::new();
        format::CudaDisplay::write(&Self::from_usize(x), "", 0, &mut writer).ok();
        ByteVecFfi::new(writer)
    }
}

impl ReprUsize for cuda_types::cudnn9::cudnnStatus_t {
    fn to_usize(self) -> usize {
        match self {
            cuda_types::cudnn9::cudnnStatus_t::SUCCESS => 0,
            Err(err) => err.0.get() as usize,
        }
    }

    fn from_usize(x: usize) -> Self {
        match NonZero::new(x as u32) {
            None => Ok(()),
            Some(err) => Err(cuda_types::cudnn9::cudnnError_t(err)),
        }
    }

    // TODO: handle this after cudnn fix

    const INTERNAL_ERROR: usize = 14;

    extern "C" fn format_status(x: usize) -> ByteVecFfi {
        let mut writer = Vec::new();
        format::CudaDisplay::write(&Self::from_usize(x), "", 0, &mut writer).ok();
        ByteVecFfi::new(writer)
    }
}

impl ReprUsize for () {
    fn to_usize(self) -> usize {
        0
    }

    fn from_usize(_x: usize) -> Self {
        ()
    }

    extern "C" fn format_status(_: usize) -> ByteVecFfi {
        let mut writer = Vec::new();
        format::CudaDisplay::write(&(), "", 0, &mut writer).ok();
        ByteVecFfi::new(writer)
    }
}

impl ReprUsize for u32 {
    fn to_usize(self) -> usize {
        self as usize
    }

    fn from_usize(x: usize) -> Self {
        x as Self
    }

    extern "C" fn format_status(x: usize) -> ByteVecFfi {
        let mut writer = Vec::new();
        format::CudaDisplay::write(&Self::from_usize(x), "", 0, &mut writer).ok();
        ByteVecFfi::new(writer)
    }
}

impl ReprUsize for i32 {
    fn to_usize(self) -> usize {
        self as usize
    }

    fn from_usize(x: usize) -> Self {
        x as Self
    }

    extern "C" fn format_status(x: usize) -> ByteVecFfi {
        let mut writer = Vec::new();
        format::CudaDisplay::write(&Self::from_usize(x), "", 0, &mut writer).ok();
        ByteVecFfi::new(writer)
    }
}

impl ReprUsize for u64 {
    fn to_usize(self) -> usize {
        self as usize
    }

    fn from_usize(x: usize) -> Self {
        x as Self
    }

    extern "C" fn format_status(x: usize) -> ByteVecFfi {
        let mut writer = Vec::new();
        format::CudaDisplay::write(&Self::from_usize(x), "", 0, &mut writer).ok();
        ByteVecFfi::new(writer)
    }
}

impl ReprUsize for *mut std::ffi::c_void {
    fn to_usize(self) -> usize {
        self as usize
    }

    fn from_usize(x: usize) -> Self {
        x as Self
    }

    extern "C" fn format_status(x: usize) -> ByteVecFfi {
        let mut writer = Vec::new();
        format::CudaDisplay::write(&Self::from_usize(x), "", 0, &mut writer).ok();
        ByteVecFfi::new(writer)
    }
}

impl ReprUsize for cuda_types::cufft::cufftResult {
    const INTERNAL_ERROR: usize = cuda_types::cufft::cufftError_t::INTERNAL_ERROR.0.get() as usize;

    fn to_usize(self) -> usize {
        match self {
            cuda_types::cufft::cufftResult::SUCCESS => 0,
            Err(err) => err.0.get() as usize,
        }
    }

    fn from_usize(x: usize) -> Self {
        match NonZero::new(x as u32) {
            None => Ok(()),
            Some(err) => Err(cuda_types::cufft::cufftError_t(err)),
        }
    }

    extern "C" fn format_status(x: usize) -> ByteVecFfi {
        let mut writer = Vec::new();
        format::CudaDisplay::write(&Self::from_usize(x), "", 0, &mut writer).ok();
        ByteVecFfi::new(writer)
    }
}

impl ReprUsize for cuda_types::cusparse::cusparseStatus_t {
    fn to_usize(self) -> usize {
        match self {
            cuda_types::cusparse::cusparseStatus_t::SUCCESS => 0,
            Err(err) => err.0.get() as usize,
        }
    }

    fn from_usize(x: usize) -> Self {
        match NonZero::new(x as u32) {
            None => Ok(()),
            Some(err) => Err(cuda_types::cusparse::cusparseError_t(err)),
        }
    }

    const INTERNAL_ERROR: usize = cuda_types::cusparse::cusparseError_t::INTERNAL_ERROR
        .0
        .get() as usize;

    extern "C" fn format_status(x: usize) -> ByteVecFfi {
        let mut writer = Vec::new();
        format::CudaDisplay::write(&Self::from_usize(x), "", 0, &mut writer).ok();
        ByteVecFfi::new(writer)
    }
}

impl ReprUsize for cuda_types::cusparse::cusparseFillMode_t {
    fn to_usize(self) -> usize {
        self.0 as usize
    }

    fn from_usize(x: usize) -> Self {
        Self(x as u32)
    }

    const INTERNAL_ERROR: usize = 0;

    extern "C" fn format_status(x: usize) -> ByteVecFfi {
        let mut writer = Vec::new();
        format::CudaDisplay::write(&Self::from_usize(x), "", 0, &mut writer).ok();
        ByteVecFfi::new(writer)
    }
}

impl ReprUsize for cuda_types::cusparse::cusparseIndexBase_t {
    fn to_usize(self) -> usize {
        self.0 as usize
    }

    fn from_usize(x: usize) -> Self {
        Self(x as u32)
    }

    const INTERNAL_ERROR: usize = 0;

    extern "C" fn format_status(x: usize) -> ByteVecFfi {
        let mut writer = Vec::new();
        format::CudaDisplay::write(&Self::from_usize(x), "", 0, &mut writer).ok();
        ByteVecFfi::new(writer)
    }
}

impl ReprUsize for cuda_types::cusparse::cusparseDiagType_t {
    fn to_usize(self) -> usize {
        self.0 as usize
    }

    fn from_usize(x: usize) -> Self {
        Self(x as u32)
    }

    const INTERNAL_ERROR: usize = 0;

    extern "C" fn format_status(x: usize) -> ByteVecFfi {
        let mut writer = Vec::new();
        format::CudaDisplay::write(&Self::from_usize(x), "", 0, &mut writer).ok();
        ByteVecFfi::new(writer)
    }
}

impl ReprUsize for cuda_types::cusparse::cusparseMatrixType_t {
    fn to_usize(self) -> usize {
        self.0 as usize
    }

    fn from_usize(x: usize) -> Self {
        Self(x as u32)
    }

    const INTERNAL_ERROR: usize = 0;

    extern "C" fn format_status(x: usize) -> ByteVecFfi {
        let mut writer = Vec::new();
        format::CudaDisplay::write(&Self::from_usize(x), "", 0, &mut writer).ok();
        ByteVecFfi::new(writer)
    }
}

impl ReprUsize for cuda_types::nvml::nvmlReturn_t {
    fn to_usize(self) -> usize {
        match self {
            cuda_types::nvml::nvmlReturn_t::SUCCESS => 0,
            Err(err) => err.0.get() as usize,
        }
    }

    fn from_usize(x: usize) -> Self {
        match NonZero::new(x as u32) {
            None => Ok(()),
            Some(err) => Err(cuda_types::nvml::nvmlError_t(err)),
        }
    }

    const INTERNAL_ERROR: usize = cuda_types::nvml::nvmlError_t::UNKNOWN.0.get() as usize;

    extern "C" fn format_status(x: usize) -> ByteVecFfi {
        let mut writer = Vec::new();
        format::CudaDisplay::write(&Self::from_usize(x), "", 0, &mut writer).ok();
        ByteVecFfi::new(writer)
    }
}
