use cuda_types::CUresult;
use std::ptr;

#[repr(C)]
pub enum CudaObjectKind {
    Context,
    Stream,
}

#[repr(C)]
pub struct CudaResult<T> {
    pub return_code: CUresult,
    pub value: T,
}

impl<T> From<Result<*const T, CUresult>> for CudaResult<*const T> {
    fn from(result: Result<*const T, CUresult>) -> Self {
        match result {
            Ok(value) => Self {
                return_code: CUresult::CUDA_SUCCESS,
                value,
            },
            Err(return_code) => Self {
                return_code,
                value: ptr::null(),
            },
        }
    }
}

impl<T> From<CudaResult<*const T>> for Result<*const T, CUresult> {
    fn from(result: CudaResult<*const T>) -> Self {
        if result.return_code == CUresult::CUDA_SUCCESS {
            Ok(result.value)
        } else {
            Err(result.return_code)
        }
    }
}

#[cfg(windows)]
type Library = libloading::os::windows::Library;
#[cfg(windows)]
type Symbol<T> = libloading::os::windows::Symbol<T>;

#[cfg(unix)]
type Library = libloading::os::unix::Library;
#[cfg(unix)]
type Symbol<T> = libloading::os::unix::Symbol<T>;

#[cfg(windows)]
pub unsafe fn get_cuda_library() -> Result<Library, libloading::Error> {
    libloading::os::windows::Library::open_already_loaded("nvcuda")
}

#[cfg(unix)]
pub unsafe fn get_cuda_library() -> Result<Library, libloading::Error> {
    const RTLD_NOLOAD: i32 = 0x4;
    libloading::os::unix::Library::open(
        Some("libcuda.so"),
        libloading::os::unix::RTLD_LAZY | RTLD_NOLOAD,
    )
}

pub unsafe fn zluda_get_hip_object(
    lib: &Library,
) -> Result<
    Symbol<
        unsafe extern "C" fn(
            cuda_object: *mut std::os::raw::c_void,
            kind: CudaObjectKind,
        ) -> CudaResult<*const std::os::raw::c_void>,
    >,
    libloading::Error,
> {
    lib.get(b"zluda_get_hip_object\0")
}
