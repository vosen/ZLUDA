use cuda_types::{cublas::*, cublaslt::*};
use hip_runtime_sys::hipStream_t;
use hipblaslt_sys::*;
use std::mem;
use zluda_common::{constants, FromCuda, ZludaObject};

pub struct Handle {
    handle: hipblasLtHandle_t,
}

impl Handle {
    fn new(handle: hipblasLtHandle_t) -> Self {
        Self { handle }
    }
}

impl ZludaObject for Handle {
    const COOKIE: usize = 0xd1d9cb43416c9620;
    type Error = cublasError_t;
    type CudaHandle = cublasLtHandle_t;
    fn drop_checked(&mut self) -> cublasStatus_t {
        unsafe { hipblaslt_sys::hipblasLtDestroy(self.handle) }?;
        Ok(())
    }
}

#[repr(transparent)]
struct WrappedCublasLtHandle(cublasLtHandle_t);

unsafe impl Send for WrappedCublasLtHandle {}
unsafe impl Sync for WrappedCublasLtHandle {}

// This is repr(c) because it needs to be compatible with Handle in zluda_blas.
#[repr(C)]
struct BlasHandle {
    blas_lt: WrappedCublasLtHandle,
}

impl ZludaObject for BlasHandle {
    const COOKIE: usize = constants::BLAS_HANDLE_COOKIE;
    type Error = cublasError_t;
    type CudaHandle = cublasLtHandle_t;
    fn drop_checked(&mut self) -> cublasStatus_t {
        // This is managed by zluda_blas
        Ok(())
    }
}

#[allow(unused_imports)]
impl<'a> FromCuda<'a, cublasLtHandle_t, cublasError_t> for &'a Handle {
    fn from_cuda(handle: &'a cublasLtHandle_t) -> Result<&'a Handle, cublasError_t> {
        // Try interpreting as a direct BlasLtHandle
        let live_check = zluda_common::as_ref(handle).ok_or(cublasError_t::INVALID_VALUE)?;
        if let Ok(blas_lt) = live_check.as_result() {
            return Ok(blas_lt);
        }

        // Fallback: handle may be a BlasHandle with an embedded BlasLtHandle
        let live_check =
            zluda_common::as_ref::<BlasHandle>(handle).ok_or(cublasError_t::INVALID_VALUE)?;
        let blas_handle = live_check.as_result()?;
        Ok(zluda_common::as_ref::<Handle>(&blas_handle.blas_lt.0)
            .ok_or(cublasError_t::INVALID_VALUE)?
            .as_result()?)
    }
}
const _: fn() = || {
    let _ = std::mem::transmute::<<Handle as zluda_common::ZludaObject>::CudaHandle, usize>;
};

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> cublasStatus_t {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> cublasStatus_t {
    cublasStatus_t::ERROR_NOT_SUPPORTED
}

pub(crate) fn get_status_name(_status: cublasStatus_t) -> *const ::core::ffi::c_char {
    todo!()
}

pub(crate) fn get_status_string(_status: cublasStatus_t) -> *const ::core::ffi::c_char {
    todo!()
}

pub(crate) fn get_version() -> usize {
    todo!()
}

pub(crate) fn get_cudart_version() -> usize {
    todo!()
}

pub(crate) fn disable_cpu_instructions_set_mask(_mask: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    todo!()
}

pub(crate) fn create(handle: &mut cublasLtHandle_t) -> cublasStatus_t {
    let mut hipblas_lt = unsafe { std::mem::zeroed() };
    unsafe { hipblasLtCreate(&mut hipblas_lt) }?;

    let zluda_handle = Handle::new(hipblas_lt);

    *handle = Handle::wrap(zluda_handle);
    Ok(())
}

pub(crate) fn destroy(handle: cublasLtHandle_t) -> cublasStatus_t {
    // Try interpreting as a direct BlasLtHandle
    let result = zluda_common::drop_checked::<Handle>(handle);
    if result != cublasStatus_t::ERROR_INVALID_VALUE {
        return result;
    }

    // Fallback: handle may be a BlasHandle with an embedded blas_lt
    let blas_handle = zluda_common::as_ref::<BlasHandle>(&handle)
        .ok_or(cublasError_t::INVALID_VALUE)?
        .as_result()?;
    zluda_common::drop_checked::<Handle>(blas_handle.blas_lt.0)
}

fn cuda_algo_from_hip(hip: hipblasLtMatmulAlgo_t) -> cublasLtMatmulAlgo_t {
    let mut cuda = cublasLtMatmulAlgo_t { data: [0; 8] };
    let (chunks, _) = hip.data.as_chunks::<8>();
    cuda.data[0] = u64::from_ne_bytes(chunks[0]);
    cuda.data[1] = u64::from_ne_bytes(chunks[1]);
    cuda.data[2] = hip.max_workspace_bytes as u64;
    cuda
}

pub(crate) fn matmul(
    light_handle: &Handle,
    compute_desc: hipblasLtMatmulDesc_t,
    alpha: *const ::core::ffi::c_void,
    a: *const ::core::ffi::c_void,
    a_desc: hipblasLtMatrixLayout_t,
    b: *const ::core::ffi::c_void,
    b_desc: hipblasLtMatrixLayout_t,
    beta: *const ::core::ffi::c_void,
    c: *const ::core::ffi::c_void,
    c_desc: hipblasLtMatrixLayout_t,
    d: *mut ::core::ffi::c_void,
    d_desc: hipblasLtMatrixLayout_t,
    algo: hipblasLtMatmulAlgo_t,
    workspace: *mut ::core::ffi::c_void,
    workspace_size_in_bytes: usize,
    stream: hipStream_t,
) -> cublasStatus_t {
    unsafe {
        hipblasLtMatmul(
            light_handle.handle,
            compute_desc,
            alpha,
            a,
            a_desc,
            b,
            b_desc,
            beta,
            c,
            c_desc,
            d,
            d_desc,
            &algo,
            workspace,
            workspace_size_in_bytes,
            stream,
        )
    }?;
    Ok(())
}

pub(crate) fn matmul_algo_get_heuristic(
    light_handle: &Handle,
    operation_desc: hipblasLtMatmulDesc_t,
    a_desc: hipblasLtMatrixLayout_t,
    b_desc: hipblasLtMatrixLayout_t,
    c_desc: hipblasLtMatrixLayout_t,
    d_desc: hipblasLtMatrixLayout_t,
    preference: hipblasLtMatmulPreference_t,
    requested_algo_count: ::core::ffi::c_int,
    heuristic_results_array: &mut cublasLtMatmulHeuristicResult_t,
    return_algo_count: &mut ::core::ffi::c_int,
) -> cublasStatus_t {
    let mut hip_algos = vec![unsafe { mem::zeroed() }; requested_algo_count as usize];
    unsafe {
        hipblasLtMatmulAlgoGetHeuristic(
            light_handle.handle,
            operation_desc,
            a_desc,
            b_desc,
            c_desc,
            d_desc,
            preference,
            requested_algo_count,
            hip_algos.as_mut_ptr(),
            return_algo_count,
        )
    }?;
    if *return_algo_count as usize > hip_algos.len() {
        return cublasStatus_t::ERROR_INTERNAL_ERROR;
    }
    for (idx, hip_algo) in hip_algos
        .into_iter()
        .take(*return_algo_count as usize)
        .enumerate()
    {
        let heuristic_results_array: *mut cublasLtMatmulHeuristicResult_t = heuristic_results_array;
        let result = unsafe { &mut *heuristic_results_array.add(idx) };
        result.algo = cuda_algo_from_hip(hip_algo.algo);
        result.workspaceSize = hip_algo.workspaceSize;
        result.state = hip_algo.state.map_err(|e| cublasError_t::from(e));
        result.wavesCount = hip_algo.wavesCount;
    }
    Ok(())
}

pub(crate) fn matmul_desc_create(
    matmul_desc: &mut hipblasLtMatmulDesc_t,
    compute_type: hipblasComputeType_t,
    scale_type: hipDataType,
) -> cublasStatus_t {
    unsafe { hipblasLtMatmulDescCreate(matmul_desc, compute_type, scale_type) }?;
    Ok(())
}

pub(crate) fn matmul_desc_destroy(matmul_desc: hipblasLtMatmulDesc_t) -> cublasStatus_t {
    unsafe { hipblasLtMatmulDescDestroy(matmul_desc) }?;
    Ok(())
}

pub(crate) fn matmul_desc_set_attribute(
    matmul_desc: hipblasLtMatmulDesc_t,
    attr: cublasLtMatmulDescAttributes_t,
    buf: *const ::core::ffi::c_void,
    size_in_bytes: usize,
) -> cublasStatus_t {
    if attr == cublasLtMatmulDescAttributes_t::CUBLASLT_MATMUL_DESC_SCALE_TYPE {
        if size_in_bytes != 4 {
            return cublasStatus_t::ERROR_INVALID_VALUE;
        }
        let scale_type = cudaDataType_t(unsafe { *buf.cast() });
        if scale_type != cudaDataType_t::CUDA_R_32F {
            return cublasStatus_t::ERROR_NOT_SUPPORTED;
        }
        return Ok(());
    }
    let hip_attr = FromCuda::<_, cublasError_t>::from_cuda(&attr)?;
    match hip_attr {
        hipblasLtMatmulDescAttributes_t::HIPBLASLT_MATMUL_DESC_TRANSA => {
            convert_and_set_attribute::<cublasOperation_t, hipblasOperation_t>(
                matmul_desc,
                buf,
                hip_attr,
            )?
        }
        hipblasLtMatmulDescAttributes_t::HIPBLASLT_MATMUL_DESC_TRANSB => {
            convert_and_set_attribute::<cublasOperation_t, hipblasOperation_t>(
                matmul_desc,
                buf,
                hip_attr,
            )?
        }
        hipblasLtMatmulDescAttributes_t::HIPBLASLT_MATMUL_DESC_EPILOGUE => {
            convert_and_set_attribute::<cublasLtEpilogue_t, hipblasLtEpilogue_t>(
                matmul_desc,
                buf,
                hip_attr,
            )?
        }
        hipblasLtMatmulDescAttributes_t::HIPBLASLT_MATMUL_DESC_BIAS_DATA_TYPE => {
            convert_and_set_attribute::<cudaDataType, hipDataType>(matmul_desc, buf, hip_attr)?
        }
        hipblasLtMatmulDescAttributes_t::HIPBLASLT_MATMUL_DESC_BIAS_POINTER => {
            unsafe { hipblasLtMatmulDescSetAttribute(matmul_desc, hip_attr, buf, size_in_bytes) }?
        }
        _ => return cublasStatus_t::ERROR_NOT_SUPPORTED,
    }
    Ok(())
}

fn convert_and_set_attribute<'a, CudaType: 'a, HipType>(
    matmul_desc: hipblasLtMatmulDesc_t,
    buf: *const std::ffi::c_void,
    hip_attr: hipblasLtMatmulDescAttributes_t,
) -> Result<(), cublasError_t>
where
    HipType: FromCuda<'a, CudaType, cuda_types::cublas::cublasError_t>,
{
    let cublas_operation: &CudaType =
        unsafe { buf.cast::<CudaType>().as_ref() }.ok_or(cublasError_t::INVALID_VALUE)?;
    let hip_operation: HipType = FromCuda::<_, cublasError_t>::from_cuda(cublas_operation)?;
    let hip_buf: *const HipType = &hip_operation;
    unsafe {
        hipblasLtMatmulDescSetAttribute(
            matmul_desc,
            hip_attr,
            hip_buf.cast(),
            mem::size_of::<HipType>(),
        )
    }?;
    Ok(())
}

pub(crate) fn matmul_preference_create(pref: &mut hipblasLtMatmulPreference_t) -> cublasStatus_t {
    unsafe { hipblasLtMatmulPreferenceCreate(pref) }?;
    Ok(())
}

pub(crate) fn matmul_preference_destroy(pref: hipblasLtMatmulPreference_t) -> cublasStatus_t {
    unsafe { hipblasLtMatmulPreferenceDestroy(pref) }?;
    Ok(())
}

pub(crate) fn matmul_preference_set_attribute(
    pref: hipblasLtMatmulPreference_t,
    attr: hipblasLtMatmulPreferenceAttributes_t,
    buf: *const ::core::ffi::c_void,
    size_in_bytes: usize,
) -> cublasStatus_t {
    unsafe { hipblasLtMatmulPreferenceSetAttribute(pref, attr, buf, size_in_bytes) }?;
    Ok(())
}

pub(crate) fn matrix_layout_create(
    mat_layout: &mut hipblasLtMatrixLayout_t,
    type_: hipDataType,
    rows: u64,
    cols: u64,
    ld: i64,
) -> cublasStatus_t {
    unsafe { hipblasLtMatrixLayoutCreate(mat_layout, type_, rows, cols, ld) }?;
    Ok(())
}

pub(crate) fn matrix_layout_destroy(mat_layout: hipblasLtMatrixLayout_t) -> cublasStatus_t {
    unsafe { hipblasLtMatrixLayoutDestroy(mat_layout) }?;
    Ok(())
}

pub(crate) fn matrix_layout_set_attribute(
    mat_layout: hipblasLtMatrixLayout_t,
    attr: hipblasLtMatrixLayoutAttribute_t,
    buf: *const ::core::ffi::c_void,
    size_in_bytes: usize,
) -> cublasStatus_t {
    unsafe { hipblasLtMatrixLayoutSetAttribute(mat_layout, attr, buf, size_in_bytes) }?;
    Ok(())
}
