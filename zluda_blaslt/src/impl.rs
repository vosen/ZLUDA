use cuda_types::{cublas::*, cublaslt::*};
use hip_runtime_sys::hipStream_t;
use hipblaslt_sys::*;
use std::mem;
use zluda_common::{from_cuda_object, FromCuda, ZludaObject};

pub struct Handle {
    handle: hipblasLtHandle_t,
}

impl Handle {
    fn new() -> Self {
        Self {
            handle: unsafe { mem::zeroed() },
        }
    }
}

impl ZludaObject for Handle {
    const COOKIE: usize = 0x49dec801578301ee;

    type Error = cublasError_t;
    type CudaHandle = cublasLtHandle_t;

    fn drop_checked(&mut self) -> cublasStatus_t {
        Ok(())
    }
}

from_cuda_object!(Handle);

pub struct MatmulDesc {
    desc: hipblasLtMatmulDesc_t,
}

impl MatmulDesc {
    fn new() -> Self {
        Self {
            desc: unsafe { mem::zeroed() },
        }
    }
}

impl ZludaObject for MatmulDesc {
    const COOKIE: usize = 0x4406a5f4b814f52b;

    type Error = cublasError_t;
    type CudaHandle = cublasLtMatmulDesc_t;

    fn drop_checked(&mut self) -> cublasStatus_t {
        unsafe { hipblasLtMatmulDescDestroy(self.desc) }?;
        Ok(())
    }
}

from_cuda_object!(MatmulDesc);

pub struct MatmulPreference {
    pref: hipblasLtMatmulPreference_t,
}

impl MatmulPreference {
    fn new() -> Self {
        Self {
            pref: unsafe { mem::zeroed() },
        }
    }
}

impl ZludaObject for MatmulPreference {
    const COOKIE: usize = 0x6a6d1c41958baa9b;

    type Error = cublasError_t;
    type CudaHandle = cublasLtMatmulPreference_t;

    fn drop_checked(&mut self) -> cublasStatus_t {
        unsafe { hipblasLtMatmulPreferenceDestroy(self.pref) }?;
        Ok(())
    }
}

from_cuda_object!(MatmulPreference);

pub struct MatrixLayout {
    layout: hipblasLtMatrixLayout_t,
}

impl MatrixLayout {
    fn new() -> Self {
        Self {
            layout: unsafe { mem::zeroed() },
        }
    }
}

impl ZludaObject for MatrixLayout {
    const COOKIE: usize = 0xcf566e9656cec9b8;

    type Error = cublasError_t;
    type CudaHandle = cublasLtMatrixLayout_t;

    fn drop_checked(&mut self) -> cublasStatus_t {
        unsafe { hipblasLtMatrixLayoutDestroy(self.layout) }?;
        Ok(())
    }
}

from_cuda_object!(MatrixLayout);

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
    let mut zluda_blaslt_handle = Handle::new();
    unsafe { hipblasLtCreate(&mut zluda_blaslt_handle.handle) }?;
    *handle = Handle::wrap(zluda_blaslt_handle);
    Ok(())
}

pub(crate) fn destroy(handle: cublasLtHandle_t) -> cublasStatus_t {
    zluda_common::drop_checked::<Handle>(handle)
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
    compute_desc: &MatmulDesc,
    alpha: *const ::core::ffi::c_void,
    a: *const ::core::ffi::c_void,
    a_desc: &MatrixLayout,
    b: *const ::core::ffi::c_void,
    b_desc: &MatrixLayout,
    beta: *const ::core::ffi::c_void,
    c: *const ::core::ffi::c_void,
    c_desc: &MatrixLayout,
    d: *mut ::core::ffi::c_void,
    d_desc: &MatrixLayout,
    algo: hipblasLtMatmulAlgo_t,
    workspace: *mut ::core::ffi::c_void,
    workspace_size_in_bytes: usize,
    stream: hipStream_t,
) -> cublasStatus_t {
    unsafe {
        hipblasLtMatmul(
            light_handle.handle,
            compute_desc.desc,
            alpha,
            a,
            a_desc.layout,
            b,
            b_desc.layout,
            beta,
            c,
            c_desc.layout,
            d,
            d_desc.layout,
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
    operation_desc: &MatmulDesc,
    a_desc: &MatrixLayout,
    b_desc: &MatrixLayout,
    c_desc: &MatrixLayout,
    d_desc: &MatrixLayout,
    preference: &MatmulPreference,
    requested_algo_count: ::core::ffi::c_int,
    heuristic_results_array: &mut cublasLtMatmulHeuristicResult_t,
    return_algo_count: &mut ::core::ffi::c_int,
) -> cublasStatus_t {
    let mut hip_algos = vec![unsafe { mem::zeroed() }; requested_algo_count as usize];
    unsafe {
        hipblasLtMatmulAlgoGetHeuristic(
            light_handle.handle,
            operation_desc.desc,
            a_desc.layout,
            b_desc.layout,
            c_desc.layout,
            d_desc.layout,
            preference.pref,
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
    matmul_desc: &mut cublasLtMatmulDesc_t,
    compute_type: hipblasComputeType_t,
    scale_type: hipDataType,
) -> cublasStatus_t {
    let mut zluda_blaslt_desc = MatmulDesc::new();
    unsafe { hipblasLtMatmulDescCreate(&mut zluda_blaslt_desc.desc, compute_type, scale_type) }?;
    *matmul_desc = MatmulDesc::wrap(zluda_blaslt_desc);
    Ok(())
}

pub(crate) fn matmul_desc_destroy(matmul_desc: cublasLtMatmulDesc_t) -> cublasStatus_t {
    zluda_common::drop_checked::<MatmulDesc>(matmul_desc)
}

pub(crate) fn matmul_desc_set_attribute(
    matmul_desc: &MatmulDesc,
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
        hipblasLtMatmulDescAttributes_t::HIPBLASLT_MATMUL_DESC_BIAS_POINTER => unsafe {
            hipblasLtMatmulDescSetAttribute(matmul_desc.desc, hip_attr, buf, size_in_bytes)
        }?,
        _ => return cublasStatus_t::ERROR_NOT_SUPPORTED,
    }
    Ok(())
}

fn convert_and_set_attribute<'a, CudaType: 'a, HipType>(
    matmul_desc: &MatmulDesc,
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
            matmul_desc.desc,
            hip_attr,
            hip_buf.cast(),
            mem::size_of::<HipType>(),
        )
    }?;
    Ok(())
}

pub(crate) fn matmul_preference_create(pref: &mut cublasLtMatmulPreference_t) -> cublasStatus_t {
    let mut zluda_matmul_pref = MatmulPreference::new();
    unsafe { hipblasLtMatmulPreferenceCreate(&mut zluda_matmul_pref.pref) }?;
    *pref = MatmulPreference::wrap(zluda_matmul_pref);
    Ok(())
}

pub(crate) fn matmul_preference_destroy(pref: cublasLtMatmulPreference_t) -> cublasStatus_t {
    zluda_common::drop_checked::<MatmulPreference>(pref)
}

pub(crate) fn matmul_preference_set_attribute(
    pref: &MatmulPreference,
    attr: hipblasLtMatmulPreferenceAttributes_t,
    buf: *const ::core::ffi::c_void,
    size_in_bytes: usize,
) -> cublasStatus_t {
    unsafe { hipblasLtMatmulPreferenceSetAttribute(pref.pref, attr, buf, size_in_bytes) }?;
    Ok(())
}

pub(crate) fn matrix_layout_create(
    mat_layout: &mut cublasLtMatrixLayout_t,
    type_: hipDataType,
    rows: u64,
    cols: u64,
    ld: i64,
) -> cublasStatus_t {
    let mut zluda_matrix_layout = MatrixLayout::new();
    unsafe { hipblasLtMatrixLayoutCreate(&mut zluda_matrix_layout.layout, type_, rows, cols, ld) }?;
    *mat_layout = MatrixLayout::wrap(zluda_matrix_layout);
    Ok(())
}

pub(crate) fn matrix_layout_destroy(mat_layout: cublasLtMatrixLayout_t) -> cublasStatus_t {
    zluda_common::drop_checked::<MatrixLayout>(mat_layout)
}

pub(crate) fn matrix_layout_set_attribute(
    mat_layout: &MatrixLayout,
    attr: hipblasLtMatrixLayoutAttribute_t,
    buf: *const ::core::ffi::c_void,
    size_in_bytes: usize,
) -> cublasStatus_t {
    unsafe { hipblasLtMatrixLayoutSetAttribute(mat_layout.layout, attr, buf, size_in_bytes) }?;
    Ok(())
}
