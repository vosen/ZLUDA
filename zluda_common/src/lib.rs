use cuda_types::{
    cublas::*,
    cublaslt::*,
    cuda::*,
    cudnn9,
    dark_api::{FatbinHeader, FatbincWrapper},
    nvml::*,
};
use dark_api::fatbin::{Fatbin, FatbinError, FatbinFile, FatbinSubmodule};
use hip_runtime_sys::*;
use hipblaslt_sys::*;
use miopen_sys::*;
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

impl CudaErrorType for cudnn9::cudnnError_t {
    const INVALID_VALUE: Self = Self::INVALID_VALUE;
    const NOT_SUPPORTED: Self = Self::NOT_SUPPORTED;
}

impl CudaErrorType for miopen_sys::miopenError_t {
    const INVALID_VALUE: Self = Self::InvalidValue;
    const NOT_SUPPORTED: Self = Self::NotImplemented;
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
                    use zluda_common::CudaErrorType;
                    Ok(zluda_common::as_ref(handle).ok_or(<$type_ as zluda_common::ZludaObject>::Error::INVALID_VALUE)?.as_result()?)
                }
            }

            // If the CUDA handle is not pointer sized it will break assumptions in `as_ref`
            const _: fn() = || {
                let _ = std::mem::transmute::<<$type_ as zluda_common::ZludaObject>::CudaHandle, usize>;
            };
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
    nvmlProcessInfo_v1_t,
    nvmlFieldValue_t,
    nvmlGpuFabricInfo_t,
    cublasLtHandle_t,
    cublasLtMatmulDesc_t,
    cublasLtMatmulPreference_t,
    cublasLtMatrixLayout_t,
    cublasLtMatmulDescAttributes_t,
    CUmemAllocationGranularity_flags,
    CUmemAllocationProp,
    CUresult,
    CUfunction_attribute,
    CUgraphExecUpdateResultInfo,
    *mut cudnn9::cudnnHandle_t,
    cudnn9::cudnnHandle_t,
    cudnn9::cudnnMathType_t,
    cudnn9::cudnnConvolutionFwdAlgoPerfStruct
);
from_cuda_transmute!(
    CUuuid => hipUUID,
    CUfunction => hipFunction_t,
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
    CUkernel => hipFunction_t,
    cublasLtMatmulDesc_t => hipblasLtMatmulDesc_t,
    cublasLtMatmulPreference_t => hipblasLtMatmulPreference_t,
    cublasLtMatrixLayout_t => hipblasLtMatrixLayout_t,
    cudnn9::cudnnTensorDescriptor_t => miopenTensorDescriptor_t,
    cudnn9::cudnnFilterDescriptor_t => miopenTensorDescriptor_t,
    cudnn9::cudnnConvolutionDescriptor_t => miopenConvolutionDescriptor_t
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

// These have the same values, so it might be okay to use from_cuda_transmute
impl<'a, E: CudaErrorType> FromCuda<'a, cudaDataType, E> for hipDataType {
    fn from_cuda(t: &'a cudaDataType) -> Result<Self, E> {
        Ok(match *t {
            cudaDataType::CUDA_R_16F => hipDataType::HIP_R_16F,
            cudaDataType::CUDA_C_16F => hipDataType::HIP_C_16F,
            cudaDataType::CUDA_R_16BF => hipDataType::HIP_R_16BF,
            cudaDataType::CUDA_C_16BF => hipDataType::HIP_C_16BF,
            cudaDataType::CUDA_R_32F => hipDataType::HIP_R_32F,
            cudaDataType::CUDA_C_32F => hipDataType::HIP_C_32F,
            cudaDataType::CUDA_R_64F => hipDataType::HIP_R_64F,
            cudaDataType::CUDA_C_64F => hipDataType::HIP_C_64F,
            cudaDataType::CUDA_R_8I => hipDataType::HIP_R_8I,
            cudaDataType::CUDA_C_8I => hipDataType::HIP_C_8I,
            cudaDataType::CUDA_R_8U => hipDataType::HIP_R_8U,
            cudaDataType::CUDA_C_8U => hipDataType::HIP_C_8U,
            cudaDataType::CUDA_R_32I => hipDataType::HIP_R_32I,
            cudaDataType::CUDA_C_32I => hipDataType::HIP_C_32I,
            cudaDataType::CUDA_R_8F_E4M3 => hipDataType::HIP_R_8F_E4M3,
            cudaDataType::CUDA_R_8F_E5M2 => hipDataType::HIP_R_8F_E5M2,
            _ => return Err(E::NOT_SUPPORTED),
        })
    }
}

impl<'a, E: CudaErrorType> FromCuda<'a, cublasComputeType_t, E> for hipblasComputeType_t {
    fn from_cuda(t: &'a cublasComputeType_t) -> Result<Self, E> {
        Ok(match *t {
            cublasComputeType_t::CUBLAS_COMPUTE_16F => hipblasComputeType_t::HIPBLAS_COMPUTE_16F,
            cublasComputeType_t::CUBLAS_COMPUTE_16F_PEDANTIC => {
                hipblasComputeType_t::HIPBLAS_COMPUTE_16F_PEDANTIC
            }
            cublasComputeType_t::CUBLAS_COMPUTE_32F => hipblasComputeType_t::HIPBLAS_COMPUTE_32F,
            cublasComputeType_t::CUBLAS_COMPUTE_32F_PEDANTIC => {
                hipblasComputeType_t::HIPBLAS_COMPUTE_32F_PEDANTIC
            }
            cublasComputeType_t::CUBLAS_COMPUTE_32F_FAST_16F => {
                hipblasComputeType_t::HIPBLAS_COMPUTE_32F_FAST_16F
            }
            cublasComputeType_t::CUBLAS_COMPUTE_32F_FAST_16BF => {
                hipblasComputeType_t::HIPBLAS_COMPUTE_32F_FAST_16BF
            }
            cublasComputeType_t::CUBLAS_COMPUTE_32F_FAST_TF32 => {
                hipblasComputeType_t::HIPBLAS_COMPUTE_32F_FAST_TF32
            }
            cublasComputeType_t::CUBLAS_COMPUTE_64F => hipblasComputeType_t::HIPBLAS_COMPUTE_64F,
            cublasComputeType_t::CUBLAS_COMPUTE_64F_PEDANTIC => {
                hipblasComputeType_t::HIPBLAS_COMPUTE_64F_PEDANTIC
            }
            cublasComputeType_t::CUBLAS_COMPUTE_32I => hipblasComputeType_t::HIPBLAS_COMPUTE_32I,
            cublasComputeType_t::CUBLAS_COMPUTE_32I_PEDANTIC => {
                hipblasComputeType_t::HIPBLAS_COMPUTE_32I_PEDANTIC
            }
            _ => return Err(E::NOT_SUPPORTED),
        })
    }
}

impl<'a, E: CudaErrorType> FromCuda<'a, cublasLtMatmulDescAttributes_t, E>
    for hipblasLtMatmulDescAttributes_t
{
    fn from_cuda(t: &'a cuda_types::cublaslt::cublasLtMatmulDescAttributes_t) -> Result<Self, E> {
        Ok(match *t {
            cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_TRANSA => {
                hipblasLtMatmulDescAttributes_t::HIPBLASLT_MATMUL_DESC_TRANSA
            }
            cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_TRANSB => {
                hipblasLtMatmulDescAttributes_t::HIPBLASLT_MATMUL_DESC_TRANSB
            }
            cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_EPILOGUE => {
                hipblasLtMatmulDescAttributes_t::HIPBLASLT_MATMUL_DESC_EPILOGUE
            }
            cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_BIAS_POINTER => {
                hipblasLtMatmulDescAttributes_t::HIPBLASLT_MATMUL_DESC_BIAS_POINTER
            }
            cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_BIAS_DATA_TYPE => {
                hipblasLtMatmulDescAttributes_t::HIPBLASLT_MATMUL_DESC_BIAS_DATA_TYPE
            }
            cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_A_SCALE_POINTER => {
                hipblasLtMatmulDescAttributes_t::HIPBLASLT_MATMUL_DESC_A_SCALE_POINTER
            }
            cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_B_SCALE_POINTER => {
                hipblasLtMatmulDescAttributes_t::HIPBLASLT_MATMUL_DESC_B_SCALE_POINTER
            }
            cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_C_SCALE_POINTER => {
                hipblasLtMatmulDescAttributes_t::HIPBLASLT_MATMUL_DESC_C_SCALE_POINTER
            }
            cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_D_SCALE_POINTER => {
                hipblasLtMatmulDescAttributes_t::HIPBLASLT_MATMUL_DESC_D_SCALE_POINTER
            }
            cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_SCALE_POINTER => {
                hipblasLtMatmulDescAttributes_t::HIPBLASLT_MATMUL_DESC_EPILOGUE_AUX_SCALE_POINTER
            }
            cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_POINTER => {
                hipblasLtMatmulDescAttributes_t::HIPBLASLT_MATMUL_DESC_EPILOGUE_AUX_POINTER
            }
            cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_LD => {
                hipblasLtMatmulDescAttributes_t::HIPBLASLT_MATMUL_DESC_EPILOGUE_AUX_LD
            }
            cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_EPILOGUE_AUX_BATCH_STRIDE => {
                hipblasLtMatmulDescAttributes_t::HIPBLASLT_MATMUL_DESC_EPILOGUE_AUX_BATCH_STRIDE
            }
            cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_POINTER_MODE => {
                hipblasLtMatmulDescAttributes_t::HIPBLASLT_MATMUL_DESC_POINTER_MODE
            }
            cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_AMAX_D_POINTER => {
                hipblasLtMatmulDescAttributes_t::HIPBLASLT_MATMUL_DESC_AMAX_D_POINTER
            }
            _ => return Err(E::NOT_SUPPORTED),
        })
    }
}

impl<'a, E: CudaErrorType> FromCuda<'a, cublasLtMatrixLayoutAttribute_t, E>
    for hipblasLtMatrixLayoutAttribute_t
{
    fn from_cuda(t: &'a cublasLtMatrixLayoutAttribute_t) -> Result<Self, E> {
        Ok(match *t {
            cublasLtMatrixLayoutAttribute_t::CUBLASLT_MATRIX_LAYOUT_TYPE => {
                hipblasLtMatrixLayoutAttribute_t::HIPBLASLT_MATRIX_LAYOUT_TYPE
            }
            cublasLtMatrixLayoutAttribute_t::CUBLASLT_MATRIX_LAYOUT_ORDER => {
                hipblasLtMatrixLayoutAttribute_t::HIPBLASLT_MATRIX_LAYOUT_ORDER
            }
            cublasLtMatrixLayoutAttribute_t::CUBLASLT_MATRIX_LAYOUT_ROWS => {
                hipblasLtMatrixLayoutAttribute_t::HIPBLASLT_MATRIX_LAYOUT_ROWS
            }
            cublasLtMatrixLayoutAttribute_t::CUBLASLT_MATRIX_LAYOUT_COLS => {
                hipblasLtMatrixLayoutAttribute_t::HIPBLASLT_MATRIX_LAYOUT_COLS
            }
            cublasLtMatrixLayoutAttribute_t::CUBLASLT_MATRIX_LAYOUT_LD => {
                hipblasLtMatrixLayoutAttribute_t::HIPBLASLT_MATRIX_LAYOUT_LD
            }
            cublasLtMatrixLayoutAttribute_t::CUBLASLT_MATRIX_LAYOUT_BATCH_COUNT => {
                hipblasLtMatrixLayoutAttribute_t::HIPBLASLT_MATRIX_LAYOUT_BATCH_COUNT
            }
            cublasLtMatrixLayoutAttribute_t::CUBLASLT_MATRIX_LAYOUT_STRIDED_BATCH_OFFSET => {
                hipblasLtMatrixLayoutAttribute_t::HIPBLASLT_MATRIX_LAYOUT_STRIDED_BATCH_OFFSET
            }
            _ => return Err(E::NOT_SUPPORTED),
        })
    }
}

impl<'a, E: CudaErrorType> FromCuda<'a, cublasLtMatmulPreferenceAttributes_t, E>
    for hipblasLtMatmulPreferenceAttributes_t
{
    fn from_cuda(t: &'a cublasLtMatmulPreferenceAttributes_t) -> Result<Self, E> {
        Ok(match *t {
            cublasLtMatmulPreferenceAttributes_t::CUBLASLT_MATMUL_PREF_SEARCH_MODE => {
                hipblasLtMatmulPreferenceAttributes_t::HIPBLASLT_MATMUL_PREF_SEARCH_MODE
            }
            cublasLtMatmulPreferenceAttributes_t::CUBLASLT_MATMUL_PREF_MAX_WORKSPACE_BYTES => {
                hipblasLtMatmulPreferenceAttributes_t::HIPBLASLT_MATMUL_PREF_MAX_WORKSPACE_BYTES
            }
            _ => return Err(E::NOT_SUPPORTED),
        })
    }
}

impl<'a, E: CudaErrorType> FromCuda<'a, *const cublasLtMatmulAlgo_t, E> for hipblasLtMatmulAlgo_t {
    fn from_cuda(t: &'a *const cublasLtMatmulAlgo_t) -> Result<Self, E> {
        // We assume the algo came from hip_algo_to_cuda so we can discard the last six bytes
        let cuda_algo = match unsafe { t.as_ref() } {
            Some(algo) => algo,
            None => return Err(E::INVALID_VALUE),
        };
        let mut hip = hipblasLtMatmulAlgo_t {
            data: [0; 16],
            max_workspace_bytes: cuda_algo.data[2] as usize,
        };
        hip.data[..8].copy_from_slice(&cuda_algo.data[0].to_ne_bytes());
        hip.data[8..].copy_from_slice(&cuda_algo.data[1].to_ne_bytes());
        Ok(hip)
    }
}

impl<'a, E: CudaErrorType> FromCuda<'a, cublasOperation_t, E> for hipblasOperation_t {
    fn from_cuda(t: &'a cublasOperation_t) -> Result<Self, E> {
        Ok(match *t {
            cublasOperation_t::CUBLAS_OP_N => hipblasOperation_t::HIPBLAS_OP_N,
            cublasOperation_t::CUBLAS_OP_T => hipblasOperation_t::HIPBLAS_OP_T,
            cublasOperation_t::CUBLAS_OP_C => hipblasOperation_t::HIPBLAS_OP_C,
            _ => return Err(E::NOT_SUPPORTED),
        })
    }
}

impl<'a, E: CudaErrorType> FromCuda<'a, cublasLtEpilogue_t, E> for hipblasLtEpilogue_t {
    fn from_cuda(t: &'a cublasLtEpilogue_t) -> Result<Self, E> {
        Ok(match *t {
            cublasLtEpilogue_t::CUBLASLT_EPILOGUE_DEFAULT => {
                hipblasLtEpilogue_t::HIPBLASLT_EPILOGUE_DEFAULT
            }
            cublasLtEpilogue_t::CUBLASLT_EPILOGUE_RELU => {
                hipblasLtEpilogue_t::HIPBLASLT_EPILOGUE_RELU
            }
            cublasLtEpilogue_t::CUBLASLT_EPILOGUE_BIAS => {
                hipblasLtEpilogue_t::HIPBLASLT_EPILOGUE_BIAS
            }
            cublasLtEpilogue_t::CUBLASLT_EPILOGUE_RELU_BIAS => {
                hipblasLtEpilogue_t::HIPBLASLT_EPILOGUE_RELU_BIAS
            }
            cublasLtEpilogue_t::CUBLASLT_EPILOGUE_GELU => {
                hipblasLtEpilogue_t::HIPBLASLT_EPILOGUE_GELU
            }
            cublasLtEpilogue_t::CUBLASLT_EPILOGUE_GELU_AUX => {
                hipblasLtEpilogue_t::HIPBLASLT_EPILOGUE_GELU_AUX
            }
            cublasLtEpilogue_t::CUBLASLT_EPILOGUE_GELU_BIAS => {
                hipblasLtEpilogue_t::HIPBLASLT_EPILOGUE_GELU_BIAS
            }
            cublasLtEpilogue_t::CUBLASLT_EPILOGUE_GELU_AUX_BIAS => {
                hipblasLtEpilogue_t::HIPBLASLT_EPILOGUE_GELU_AUX_BIAS
            }
            cublasLtEpilogue_t::CUBLASLT_EPILOGUE_DGELU => {
                hipblasLtEpilogue_t::HIPBLASLT_EPILOGUE_DGELU
            }
            cublasLtEpilogue_t::CUBLASLT_EPILOGUE_DGELU_BGRAD => {
                hipblasLtEpilogue_t::HIPBLASLT_EPILOGUE_DGELU_BGRAD
            }
            cublasLtEpilogue_t::CUBLASLT_EPILOGUE_BGRADA => {
                hipblasLtEpilogue_t::HIPBLASLT_EPILOGUE_BGRADA
            }
            cublasLtEpilogue_t::CUBLASLT_EPILOGUE_BGRADB => {
                hipblasLtEpilogue_t::HIPBLASLT_EPILOGUE_BGRADB
            }
            _ => return Err(E::NOT_SUPPORTED),
        })
    }
}

impl<'a, E: CudaErrorType> FromCuda<'a, *mut cublasLtMatmulHeuristicResult_t, E>
    for &'a mut cublasLtMatmulHeuristicResult_t
{
    fn from_cuda(x: &'a *mut cublasLtMatmulHeuristicResult_t) -> Result<Self, E> {
        match unsafe { x.as_mut() } {
            Some(x) => Ok(x),
            None => Err(E::INVALID_VALUE),
        }
    }
}

impl<'a, E: CudaErrorType> FromCuda<'a, cudnn9::cudnnTensorFormat_t, E> for miopenTensorLayout_t {
    fn from_cuda(format: &'a cudnn9::cudnnTensorFormat_t) -> Result<Self, E> {
        Ok(match *format {
            cudnn9::cudnnTensorFormat_t::CUDNN_TENSOR_NCHW => {
                miopenTensorLayout_t::miopenTensorNCHW
            }
            cudnn9::cudnnTensorFormat_t::CUDNN_TENSOR_NHWC => {
                miopenTensorLayout_t::miopenTensorNHWC
            }
            cudnn9::cudnnTensorFormat_t::CUDNN_TENSOR_NCHW_VECT_C => {
                miopenTensorLayout_t::miopenTensorNCHWc4
            }
            _ => return Err(E::NOT_SUPPORTED),
        })
    }
}

impl<'a, E: CudaErrorType> FromCuda<'a, cudnn9::cudnnDataType_t, E> for miopenDataType_t {
    fn from_cuda(format: &'a cudnn9::cudnnDataType_t) -> Result<Self, E> {
        Ok(match *format {
            cudnn9::cudnnDataType_t::CUDNN_DATA_HALF => miopenDataType_t::miopenHalf,
            cudnn9::cudnnDataType_t::CUDNN_DATA_FLOAT => miopenDataType_t::miopenFloat,
            cudnn9::cudnnDataType_t::CUDNN_DATA_INT32 => miopenDataType_t::miopenInt32,
            cudnn9::cudnnDataType_t::CUDNN_DATA_INT8 => miopenDataType_t::miopenInt8,
            cudnn9::cudnnDataType_t::CUDNN_DATA_BFLOAT16 => miopenDataType_t::miopenBFloat16,
            cudnn9::cudnnDataType_t::CUDNN_DATA_DOUBLE => miopenDataType_t::miopenDouble,
            cudnn9::cudnnDataType_t::CUDNN_DATA_FP8_E4M3 => miopenDataType_t::miopenFloat8,
            cudnn9::cudnnDataType_t::CUDNN_DATA_FP8_E5M2 => miopenDataType_t::miopenBFloat8,
            cudnn9::cudnnDataType_t::CUDNN_DATA_INT64 => miopenDataType_t::miopenInt64,
            _ => return Err(E::NOT_SUPPORTED),
        })
    }
}

impl<'a, E: CudaErrorType> FromCuda<'a, cudnn9::cudnnConvolutionMode_t, E>
    for miopenConvolutionMode_t
{
    fn from_cuda(format: &'a cudnn9::cudnnConvolutionMode_t) -> Result<Self, E> {
        Ok(match *format {
            cudnn9::cudnnConvolutionMode_t::CUDNN_CROSS_CORRELATION => {
                miopenConvolutionMode_t::miopenConvolution
            }
            _ => return Err(E::NOT_SUPPORTED),
        })
    }
}

impl<'a, E: CudaErrorType> FromCuda<'a, cudnn9::cudnnConvolutionFwdAlgo_t, E>
    for miopenConvFwdAlgorithm_t
{
    fn from_cuda(format: &'a cudnn9::cudnnConvolutionFwdAlgo_t) -> Result<Self, E> {
        Ok(match *format {
            cudnn9::cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_IMPLICIT_GEMM => {
                miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoImplicitGEMM
            }
            cudnn9::cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_IMPLICIT_PRECOMP_GEMM => {
                // No direct MIOpen equivalent; map to closest available
                miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoImplicitGEMM
            }
            cudnn9::cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_GEMM => {
                miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoGEMM
            }
            cudnn9::cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_DIRECT => {
                miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoDirect
            }
            cudnn9::cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_FFT => {
                miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoFFT
            }
            cudnn9::cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_FFT_TILING => {
                // No direct MIOpen equivalent; map to FFT variant
                miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoFFT
            }
            cudnn9::cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_WINOGRAD => {
                miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoWinograd
            }
            cudnn9::cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_WINOGRAD_NONFUSED => {
                // No direct MIOpen equivalent; map to Winograd variant
                miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoWinograd
            }
            _ => return Err(E::NOT_SUPPORTED),
        })
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
pub fn as_ref<'a, T: ZludaObject>(handle: &'a T::CudaHandle) -> Option<&'a LiveCheck<T>> {
    if unsafe { mem::transmute_copy::<_, usize>(handle) } == 0 {
        return None;
    }
    let option_box = unsafe { mem::transmute::<_, &'a ManuallyDrop<Box<LiveCheck<T>>>>(handle) };
    Some(option_box)
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
