use cuda_types::{
    cublas::*,
    cublaslt::cublasLtHandle_t,
    cuda::*,
    dark_api::{FatbinHeader, FatbincWrapper},
    nvml::*,
};
use dark_api::fatbin::{Fatbin, FatbinError, FatbinFile, FatbinSubmodule};
use hip_runtime_sys::*;
use rocblas_sys::*;
use std::{
    ffi::{c_void, CStr},
    mem::{self, ManuallyDrop, MaybeUninit},
    ptr,
    str::Utf8Error,
};

pub trait CudaErrorType {
    const INVALID_VALUE: Self;
    const NOT_SUPPORTED: Self;
}

impl CudaErrorType for CUerror {
    const INVALID_VALUE: Self = Self::INVALID_VALUE;
    const NOT_SUPPORTED: Self = Self::NOT_SUPPORTED;
}

impl CudaErrorType for cublasError_t {
    const INVALID_VALUE: Self = Self::INVALID_VALUE;
    const NOT_SUPPORTED: Self = Self::NOT_SUPPORTED;
}

impl CudaErrorType for rocblas_error {
    const INVALID_VALUE: Self = Self::invalid_value;
    const NOT_SUPPORTED: Self = Self::not_implemented;
}

impl CudaErrorType for nvmlError_t {
    const INVALID_VALUE: Self = Self::INVALID_ARGUMENT;
    const NOT_SUPPORTED: Self = Self::NOT_SUPPORTED;
}

/// Used to try to convert CUDA API values into our internal representation.
///
/// Similar to [`TryFrom`], but we can implement this for primitive types. We also provide conversions from pointers to references.
pub trait FromCuda<'a, T, E: CudaErrorType>: Sized {
    /// Tries to convert to this type.
    fn from_cuda(t: &'a T) -> Result<Self, E>;
}

macro_rules! from_cuda_nop {
    ($($type_:ty),*) => {
        $(
            impl<'a, E: CudaErrorType> FromCuda<'a, $type_, E> for $type_ {
                fn from_cuda(x: &'a $type_) -> Result<Self, E> {
                    Ok(*x)
                }
            }

            impl<'a, E: CudaErrorType> FromCuda<'a, *mut $type_, E> for &'a mut $type_ {
                fn from_cuda(x: &'a *mut $type_) -> Result<Self, E> {
                    match unsafe { x.as_mut() } {
                        Some(x) => Ok(x),
                        None => Err(E::INVALID_VALUE),
                    }
                }
            }

            impl<'a, E: CudaErrorType> FromCuda<'a, *const $type_, E> for &'a $type_ {
                fn from_cuda(x: &'a *const $type_) -> Result<Self, E> {
                    match unsafe { x.as_ref() } {
                        Some(x) => Ok(x),
                        None => Err(E::INVALID_VALUE),
                    }
                }
            }

            impl<'a, E: CudaErrorType> FromCuda<'a, *mut $type_, E> for Option<&'a mut $type_> {
                fn from_cuda(x: &'a *mut $type_) -> Result<Self, E> {
                    Ok(unsafe { x.as_mut() })
                }
            }
        )*
    };
}

macro_rules! from_cuda_transmute {
    ($($from:ty => $to:ty),*) => {
        $(
            impl<'a, E: CudaErrorType> FromCuda<'a, $from, E> for $to {
                fn from_cuda(x: &'a $from) -> Result<Self, E> {
                    Ok(unsafe { std::mem::transmute(*x) })
                }
            }

            impl<'a, E: CudaErrorType> FromCuda<'a, *mut $from, E> for &'a mut $to {
                fn from_cuda(x: &'a *mut $from) -> Result<Self, E> {
                    match unsafe { x.cast::<$to>().as_mut() } {
                        Some(x) => Ok(x),
                        None => Err(E::INVALID_VALUE),
                    }
                }
            }

            impl<'a, E: CudaErrorType> FromCuda<'a, *mut $from, E> for * mut $to {
                fn from_cuda(x: &'a *mut $from) -> Result<Self, E> {
                    Ok(x.cast::<$to>())
                }
            }

            impl<'a, E: CudaErrorType> FromCuda<'a, *mut *const $from, E> for *mut *const $to {
                fn from_cuda(x: &'a *mut *const $from) -> Result<Self, E> {
                    Ok(x.cast::<*const $to>())
                }
            }
        )*
    };
}

/// Implement the [`FromCuda`] trait for a [`ZludaObject`].
#[macro_export]
macro_rules! from_cuda_object {
    ($($type_:ty),*) => {
        $(
            impl<'a> zluda_common::FromCuda<'a, <$type_ as zluda_common::ZludaObject>::CudaHandle, <$type_ as zluda_common::ZludaObject>::Error> for &'a $type_ {
                fn from_cuda(handle: &'a <$type_ as zluda_common::ZludaObject>::CudaHandle) -> Result<&'a $type_, <$type_ as zluda_common::ZludaObject>::Error> {
                    Ok(zluda_common::as_ref(handle).as_result()?)
                }
            }
        )*
    };
}

from_cuda_nop!(
    *mut i8,
    *mut i32,
    *mut u64,
    *mut usize,
    *const f32,
    *mut f32,
    *const ::core::ffi::c_void,
    *const ::core::ffi::c_char,
    *mut ::core::ffi::c_void,
    *mut *mut ::core::ffi::c_void,
    u8,
    i32,
    u32,
    u64,
    i64,
    usize,
    cuda_types::cuda::CUdevprop,
    CUdevice_attribute,
    CUdriverProcAddressQueryResult,
    CUjit_option,
    CUlibraryOption,
    CUmoduleLoadingMode,
    CUuuid,
    CUlibrary,
    CUmodule,
    CUcontext,
    cublasHandle_t,
    cublasStatus_t,
    CUlaunchConfig,
    cublasMath_t,
    nvmlDevice_t,
    nvmlFieldValue_t,
    nvmlGpuFabricInfo_t,
    cublasLtHandle_t,
    CUmemAllocationGranularity_flags,
    CUmemAllocationProp,
    CUresult
);
from_cuda_transmute!(
    CUuuid => hipUUID,
    CUfunction => hipFunction_t,
    CUfunction_attribute => hipFunction_attribute,
    CUstream => hipStream_t,
    CUpointer_attribute => hipPointer_attribute,
    CUdeviceptr_v2 => hipDeviceptr_t,
    CUevent => hipEvent_t,
    // This is safe because HIP's enum is the subset of CUDA's enum and
    // this type is used purely as a function result
    CUstreamCaptureStatus => hipStreamCaptureStatus,
    CUgraph => hipGraph_t,
    CUstreamCaptureMode => hipStreamCaptureMode,
    CUgraphNode => hipGraphNode_t,
    CUgraphExec => hipGraphExec_t,
    CUkernel => hipFunction_t
);

impl<'a, E: CudaErrorType> FromCuda<'a, CUlimit, E> for hipLimit_t {
    fn from_cuda(limit: &'a CUlimit) -> Result<Self, E> {
        Ok(match *limit {
            CUlimit::CU_LIMIT_STACK_SIZE => hipLimit_t::hipLimitStackSize,
            CUlimit::CU_LIMIT_PRINTF_FIFO_SIZE => hipLimit_t::hipLimitPrintfFifoSize,
            CUlimit::CU_LIMIT_MALLOC_HEAP_SIZE => hipLimit_t::hipLimitMallocHeapSize,
            _ => return Err(E::NOT_SUPPORTED),
        })
    }
}

impl<'a, E: CudaErrorType> FromCuda<'a, *const ::core::ffi::c_char, E> for &CStr {
    fn from_cuda(s: &'a *const ::core::ffi::c_char) -> Result<Self, E> {
        if *s != ptr::null() {
            Ok(unsafe { CStr::from_ptr(*s) })
        } else {
            Err(E::INVALID_VALUE)
        }
    }
}

impl<'a, E: CudaErrorType> FromCuda<'a, *const ::core::ffi::c_void, E> for &'a ::core::ffi::c_void {
    fn from_cuda(x: &'a *const ::core::ffi::c_void) -> Result<Self, E> {
        match unsafe { x.as_ref() } {
            Some(x) => Ok(x),
            None => Err(E::INVALID_VALUE),
        }
    }
}

impl<'a, E: CudaErrorType> FromCuda<'a, cublasOperation_t, E> for rocblas_operation {
    fn from_cuda(t: &'a cublasOperation_t) -> Result<Self, E> {
        Ok(match *t {
            cublasOperation_t::CUBLAS_OP_N => rocblas_operation::rocblas_operation_none,
            cublasOperation_t::CUBLAS_OP_T => rocblas_operation::rocblas_operation_transpose,
            cublasOperation_t::CUBLAS_OP_C => {
                rocblas_operation::rocblas_operation_conjugate_transpose
            }
            _ => return Err(E::NOT_SUPPORTED),
        })
    }
}

impl<'a, E: CudaErrorType> FromCuda<'a, cublasMath_t, E> for rocblas_math_mode {
    fn from_cuda(mode: &'a cublasMath_t) -> Result<Self, E> {
        Ok(match *mode {
            cublasMath_t::CUBLAS_DEFAULT_MATH => rocblas_math_mode_::rocblas_default_math,
            cublasMath_t::CUBLAS_TF32_TENSOR_OP_MATH => rocblas_math_mode::rocblas_xf32_xdl_math_op,
            _ => return Err(E::NOT_SUPPORTED),
        })
    }
}

impl<'a, E: CudaErrorType> FromCuda<'a, rocblas_math_mode, E> for cublasMath_t {
    fn from_cuda(mode: &'a rocblas_math_mode) -> Result<Self, E> {
        Ok(match *mode {
            rocblas_math_mode_::rocblas_default_math => cublasMath_t::CUBLAS_DEFAULT_MATH,
            rocblas_math_mode::rocblas_xf32_xdl_math_op => cublasMath_t::CUBLAS_TF32_TENSOR_OP_MATH,
            _ => return Err(E::NOT_SUPPORTED),
        })
    }
}

impl<'a, E: CudaErrorType> FromCuda<'a, cuda_types::cublas::cudaDataType, E> for rocblas_datatype {
    fn from_cuda(mode: &'a cuda_types::cublas::cudaDataType) -> Result<Self, E> {
        Ok(match *mode {
            cudaDataType_t::CUDA_R_16F => rocblas_datatype::rocblas_datatype_f16_r,
            cudaDataType_t::CUDA_R_32F => rocblas_datatype::rocblas_datatype_f32_r,
            cudaDataType_t::CUDA_R_64F => rocblas_datatype::rocblas_datatype_f64_r,
            cudaDataType_t::CUDA_C_16F => rocblas_datatype::rocblas_datatype_f16_c,
            cudaDataType_t::CUDA_C_32F => rocblas_datatype::rocblas_datatype_f32_c,
            cudaDataType_t::CUDA_C_64F => rocblas_datatype::rocblas_datatype_f64_c,
            cudaDataType_t::CUDA_R_8I => rocblas_datatype::rocblas_datatype_i8_r,
            cudaDataType_t::CUDA_R_8U => rocblas_datatype::rocblas_datatype_u8_r,
            cudaDataType_t::CUDA_R_32I => rocblas_datatype::rocblas_datatype_i32_r,
            cudaDataType_t::CUDA_R_32U => rocblas_datatype::rocblas_datatype_u32_r,
            cudaDataType_t::CUDA_C_8I => rocblas_datatype::rocblas_datatype_i8_c,
            cudaDataType_t::CUDA_C_8U => rocblas_datatype::rocblas_datatype_u8_c,
            cudaDataType_t::CUDA_C_32I => rocblas_datatype::rocblas_datatype_i32_c,
            cudaDataType_t::CUDA_C_32U => rocblas_datatype::rocblas_datatype_u32_c,
            cudaDataType_t::CUDA_R_16BF => rocblas_datatype::rocblas_datatype_bf16_r,
            cudaDataType_t::CUDA_C_16BF => rocblas_datatype::rocblas_datatype_bf16_c,
            cudaDataType_t::CUDA_R_8F_UE4M3 => rocblas_datatype::rocblas_datatype_f8_r,
            cudaDataType_t::CUDA_R_8F_E5M2 => rocblas_datatype::rocblas_datatype_bf8_r,
            _ => return Err(E::NOT_SUPPORTED),
        })
    }
}

impl<'a, E: CudaErrorType> FromCuda<'a, cuda_types::cublas::cublasComputeType_t, E>
    for rocblas_computetype
{
    fn from_cuda(mode: &'a cuda_types::cublas::cublasComputeType_t) -> Result<Self, E> {
        Ok(match *mode {
            cublasComputeType_t::CUBLAS_COMPUTE_32F => {
                rocblas_computetype::rocblas_compute_type_f32
            }
            _ => return Err(E::NOT_SUPPORTED),
        })
    }
}

impl<'a, E: CudaErrorType> FromCuda<'a, cuda_types::cublas::cublasComputeType_t, E>
    for rocblas_datatype
{
    fn from_cuda(mode: &'a cuda_types::cublas::cublasComputeType_t) -> Result<Self, E> {
        Ok(match *mode {
            cublasComputeType_t::CUBLAS_COMPUTE_16F => rocblas_datatype::rocblas_datatype_f16_r,
            cublasComputeType_t::CUBLAS_COMPUTE_32F => rocblas_datatype::rocblas_datatype_f32_r,
            cublasComputeType_t::CUBLAS_COMPUTE_64F => rocblas_datatype::rocblas_datatype_f64_r,
            _ => return Err(E::NOT_SUPPORTED),
        })
    }
}

impl<'a, E: CudaErrorType> FromCuda<'a, cuda_types::cublas::cublasGemmAlgo_t, E>
    for rocblas_gemm_algo
{
    fn from_cuda(_: &'a cuda_types::cublas::cublasGemmAlgo_t) -> Result<Self, E> {
        Ok(rocblas_gemm_algo::rocblas_gemm_algo_standard)
    }
}

/// Represents an object that can be sent across the API boundary.
///
/// Some CUDA calls operate on an opaque handle. For example, `cuModuleLoadData` will load a
/// module's data and set the `module` output argument to a new `CUmodule`. Then, other functions
/// like `cuModuleGetFunction` can take that `CUmodule` as an argument.
pub trait ZludaObject: Sized + Send + Sync {
    /// This is a unique identifier used by [`LiveCheck`] for runtime type and lifetime checking.
    ///
    /// You can generate a new one with Python:
    ///
    /// ```python
    /// import random
    /// hex(random.getrandbits(64))
    /// ```
    const COOKIE: usize;

    /// The value of [`Self::Error`] used to represent a type check failure or use after free.
    const LIVENESS_FAIL: Self::Error = Self::Error::INVALID_VALUE;

    /// The error type that should be used. This is generally specific to the library this trait
    /// is being implemented in – for example, a [`ZludaObject`] in `zluda` should use the
    /// [`CUerror`] type, and a [`ZludaObject`] in `zluda_blas` should use the [`cublasStatus_t`]
    /// type.
    type Error: CudaErrorType;
    /// The handle type that an object of this trait should should be wrapped as.
    type CudaHandle: Sized;

    /// Executes the destructor for this type.
    fn drop_checked(&mut self) -> Result<(), Self::Error>;

    /// Wraps an object of this trait in a [`LiveCheck`] that can be used for runtime type and
    /// lifetime checking, and returns it as an opaque [`Self::CudaHandle`] that can be passed to
    /// the API caller.
    fn wrap(self) -> Self::CudaHandle {
        unsafe { mem::transmute_copy(&LiveCheck::wrap(self)) }
    }
}

/// Wraps a [`ZludaObject`] and provides runtime type and lifetime checking.
///
/// Arbitrary memory can be cast to a value of this type, and then [`LiveCheck::as_result`] can be
/// used to get the wrapped [`ZludaObject`] value, if it is valid.
#[repr(C)]
pub struct LiveCheck<T: ZludaObject> {
    cookie: usize,
    /// The wrapped [`ZludaObject`].
    pub data: MaybeUninit<T>,
}

impl<T: ZludaObject> LiveCheck<T> {
    /// Wraps `data` as a valid, initialized `LiveCheck`.
    pub fn new(data: T) -> Self {
        LiveCheck {
            cookie: T::COOKIE,
            data: MaybeUninit::new(data),
        }
    }

    /// Returns this value as an opaque `T::CudaHandle`.
    pub fn as_handle(&self) -> T::CudaHandle {
        unsafe { mem::transmute_copy(&self) }
    }

    fn wrap(data: T) -> *mut Self {
        Box::into_raw(Box::new(Self::new(data)))
    }

    /// Checks if this value represents a valid and initialized value of `T` and returns it.
    /// Returns `T::LIVENESS_FAIL` if it does not.
    pub fn as_result(&self) -> Result<&T, T::Error> {
        if self.cookie == T::COOKIE {
            Ok(unsafe { self.data.assume_init_ref() })
        } else {
            Err(T::LIVENESS_FAIL)
        }
    }

    // This looks like nonsense, but it's not. There are two cases:
    // Err(CUerror) -> meaning that the object is invalid, this pointer does not point into valid memory
    // Ok(maybe_error) -> meaning that the object is valid, we dropped everything, but there *might*
    //                    an error in the underlying runtime that we want to propagate
    #[must_use]
    fn drop_checked(&mut self) -> Result<Result<(), T::Error>, T::Error> {
        if self.cookie == T::COOKIE {
            self.cookie = 0;
            let result = unsafe { self.data.assume_init_mut().drop_checked() };
            unsafe { MaybeUninit::assume_init_drop(&mut self.data) };
            Ok(result)
        } else {
            Err(T::LIVENESS_FAIL)
        }
    }
}

/// Cast a `T::CudaHandle` reference to a [`LiveCheck`] reference, preserving the lifetime.
pub fn as_ref<'a, T: ZludaObject>(
    handle: &'a T::CudaHandle,
) -> &'a ManuallyDrop<Box<LiveCheck<T>>> {
    unsafe { mem::transmute(handle) }
}

/// Try to drop `handle`.
///
/// Returns an error if `handle` is not initialized, not a value of `T`, or if `T::drop_checked`
/// returns an error.
pub fn drop_checked<T: ZludaObject>(handle: T::CudaHandle) -> Result<(), T::Error> {
    let mut wrapped_object: ManuallyDrop<Box<LiveCheck<T>>> =
        unsafe { mem::transmute_copy(&handle) };
    let underlying_error = LiveCheck::drop_checked(&mut wrapped_object)?;
    unsafe { ManuallyDrop::drop(&mut wrapped_object) };
    underlying_error
}

/*
pub struct CodeModuleRef<'a> {
    pub kind: CodeModuleKind,
    pub data: &'a [u8],
}

impl<'a> CodeModule<'a> {
    /// Interprets `data` as a code module of some kind.
    ///
    /// This does not validate the contents of `data`, it only looks at the headers to determine
    /// what kind of data it is.
    pub fn parse(data: *mut c_void) -> Result<Self, CUerror> {
        if data.len() >= 4 {
            let kind = match &data[0..4] {
                FatbincWrapper::MAGIC => CodeModuleKind::FatbincWrapper,
                FatbinHeader::MAGIC => CodeModuleKind::FatbinHeader,
                elf64::header::ELFMAG => CodeModuleKind::Elf,
                _ => {
                    if data.ends_with(&[0]) && data.iter().all(|&c| c != 0) {
                        CodeModuleKind::Ptx
                    } else {
                        CodeModuleKind::ForeignElf
                    }
                }
            };
            Ok(CodeModule { kind, data })
        } else {
            Err(CUerror::INVALID_VALUE)
        }
    }
}
     */

// We receive module as an opaque pointer. We want to handle three different
// lifetime-related scenarios:
// * The module has a 'static lifetime, but we don't want to use it just yet
//   (`cuLibraryLoadData` with CU_LIBRARY_BINARY_IS_PRESERVED = 1), we might
//   never use it.
//       In this case we just keep the void pointer, we can pass it to
//       the consuming function later
// * The module has a non-'static lifetime, and we will use it in the future
//   (`cuLibraryLoadData` with CU_LIBRARY_BINARY_IS_PRESERVED = 0)
//       In this case we need to copy the data into its own buffers
// * The module lifetime is scoped to the current function. E.g. zluda_trace
//   might to parse a module to inspect and save it or it's cuModuleLoadData
//        In this case we need to return either the compatible ELF or the
//        iterator over `Cow` with decompressed PTX strings
//            Even here there are two cases:
//            * The consumer is cuModuleLoadData, if it's our ELF then it wants
//              to load it directly from the pointer
//            * The consumer is zluda_trace, it wants to compute the length of
//              the ELF and save it to a file
#[derive(Clone, Copy)]
pub enum CodeLibraryRef<'a> {
    FatbincWrapper(Fatbin<'a>),
    FatbinHeader(FatbinSubmodule<'a>),
    Text(&'a str),
    Elf(&'a c_void),
    Archive(&'a c_void),
}

impl<'a> CodeLibraryRef<'a> {
    const ELFMAG: [u8; 4] = *b"\x7FELF";
    const AR_MAGIC: [u8; 8] = *b"!<arch>\x0A";

    pub unsafe fn try_load(ptr: *const c_void) -> Result<Self, Utf8Error> {
        Ok(match *ptr.cast::<[u8; 4]>() {
            FatbincWrapper::MAGIC => Self::FatbincWrapper(Fatbin {
                wrapper: &*(ptr.cast()),
            }),
            FatbinHeader::MAGIC => Self::FatbinHeader(FatbinSubmodule {
                header: &*(ptr.cast()),
            }),
            Self::ELFMAG => Self::Elf(&*ptr),
            _ => match *ptr.cast::<[u8; 8]>() {
                Self::AR_MAGIC => Self::Archive(&*ptr),
                _ => CStr::from_ptr(ptr.cast()).to_str().map(Self::Text)?,
            },
        })
    }

    pub unsafe fn iterate_modules(
        self,
        mut fn_: impl FnMut(Option<(usize, Option<usize>)>, Result<CodeModuleRef<'a>, FatbinError>),
    ) {
        match self {
            CodeLibraryRef::FatbincWrapper(fatbin) => {
                let module_iter = fatbin.get_submodules();
                match module_iter {
                    Ok(mut iter) => {
                        let mut module_index = if iter.multi_module() {
                            None
                        } else {
                            Some(0usize)
                        };
                        while let Some(maybe_submodule) = iter.next() {
                            match maybe_submodule {
                                Ok(submodule) => iterate_modules_fatbin_header(
                                    |subindex, module| {
                                        let index = match module_index {
                                            Some(index) => (index, Some(subindex)),
                                            None => (subindex, None),
                                        };
                                        fn_(Some(index), module)
                                    },
                                    submodule,
                                ),
                                Err(err) => fn_(
                                    module_index.map(|module_index| (module_index, None)),
                                    Err(FatbinError::ParseFailure(err)),
                                ),
                            }
                            module_index = module_index.map(|index| index + 1);
                        }
                    }
                    Err(err) => fn_(None, Err(err)),
                }
            }
            CodeLibraryRef::FatbinHeader(submodule) => iterate_modules_fatbin_header(
                |index, module| fn_(Some((index, None)), module),
                submodule,
            ),
            CodeLibraryRef::Text(text) => fn_(None, Ok(CodeModuleRef::Text(text))),
            CodeLibraryRef::Elf(elf) => fn_(None, Ok(CodeModuleRef::Elf(elf))),
            CodeLibraryRef::Archive(ar) => fn_(None, Ok(CodeModuleRef::Archive(ar))),
        }
    }
}

unsafe fn iterate_modules_fatbin_header<'x>(
    mut fn_: impl FnMut(usize, Result<CodeModuleRef<'x>, FatbinError>),
    submodule: FatbinSubmodule<'x>,
) {
    let mut iter = submodule.get_files();
    let mut index = 0;
    while let Some(file) = iter.next() {
        fn_(
            index,
            file.map(CodeModuleRef::File)
                .map_err(FatbinError::ParseFailure),
        );
        index += 1;
    }
}

#[derive(Clone, Copy)]
pub enum CodeModuleRef<'a> {
    File(FatbinFile<'a>),
    Text(&'a str),
    Elf(*const c_void),
    Archive(*const c_void),
}
