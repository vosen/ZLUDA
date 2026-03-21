use cuda_types::cuda::{CUfunction, CUfunction_attribute, CUkernel};
use hip_runtime_sys::*;
use std::mem;

pub(crate) struct Function {
    pub(crate) base: hipFunction_t,
    pub(crate) sm_version: u32,
}

impl<'a, E: zluda_common::CudaErrorType> zluda_common::FromCuda<'a, CUfunction, E>
    for &'a Function
{
    fn from_cuda(cu_func: &'a CUfunction) -> Result<Self, E> {
        Ok(unsafe { mem::transmute(*cu_func) })
    }
}

impl<'a, E: zluda_common::CudaErrorType> zluda_common::FromCuda<'a, CUkernel, E> for &'a Function {
    fn from_cuda(cu_func: &'a CUkernel) -> Result<Self, E> {
        Ok(unsafe { mem::transmute(*cu_func) })
    }
}

impl<'a, E: zluda_common::CudaErrorType> zluda_common::FromCuda<'a, *mut CUfunction, E>
    for &'a mut &'a Function
{
    fn from_cuda(cu_func: &'a *mut CUfunction) -> Result<Self, E> {
        let cu_func = unsafe { cu_func.as_mut() }.ok_or_else(|| E::INVALID_VALUE)?;
        Ok(unsafe { mem::transmute(cu_func) })
    }
}

impl<'a, E: zluda_common::CudaErrorType> zluda_common::FromCuda<'a, *mut CUkernel, E>
    for &'a mut &'a Function
{
    fn from_cuda(cu_func: &'a *mut CUkernel) -> Result<Self, E> {
        let cu_func = unsafe { cu_func.as_mut() }.ok_or_else(|| E::INVALID_VALUE)?;
        Ok(unsafe { mem::transmute(cu_func) })
    }
}

pub(crate) fn get_attribute(
    pi: &mut i32,
    cu_attrib: CUfunction_attribute,
    func: &Function,
) -> hipError_t {
    match cu_attrib {
        CUfunction_attribute::CU_FUNC_ATTRIBUTE_PTX_VERSION => {
            *pi = func.sm_version as i32;
            return Ok(());
        }
        CUfunction_attribute::CU_FUNC_ATTRIBUTE_BINARY_VERSION => {
            *pi = 120;
            return Ok(());
        }
        CUfunction_attribute::CU_FUNC_ATTRIBUTE_CLUSTER_SIZE_MUST_BE_SET
        | CUfunction_attribute::CU_FUNC_ATTRIBUTE_REQUIRED_CLUSTER_WIDTH
        | CUfunction_attribute::CU_FUNC_ATTRIBUTE_REQUIRED_CLUSTER_HEIGHT
        | CUfunction_attribute::CU_FUNC_ATTRIBUTE_REQUIRED_CLUSTER_DEPTH
        | CUfunction_attribute::CU_FUNC_ATTRIBUTE_NON_PORTABLE_CLUSTER_SIZE_ALLOWED
        | CUfunction_attribute::CU_FUNC_ATTRIBUTE_CLUSTER_SCHEDULING_POLICY_PREFERENCE => {
            *pi = 0;
            return Ok(());
        }
        _ => {}
    }
    unsafe { hipFuncGetAttribute(pi, mem::transmute(cu_attrib), func.base) }?;
    if cu_attrib == CUfunction_attribute::CU_FUNC_ATTRIBUTE_NUM_REGS {
        *pi = (*pi).max(1);
    }
    Ok(())
}

pub(crate) fn launch_kernel(
    f: &Function,
    grid_dim_x: ::core::ffi::c_uint,
    grid_dim_y: ::core::ffi::c_uint,
    grid_dim_z: ::core::ffi::c_uint,
    block_dim_x: ::core::ffi::c_uint,
    block_dim_y: ::core::ffi::c_uint,
    block_dim_z: ::core::ffi::c_uint,
    shared_mem_bytes: ::core::ffi::c_uint,
    stream: hipStream_t,
    kernel_params: *mut *mut ::core::ffi::c_void,
    extra: *mut *mut ::core::ffi::c_void,
) -> hipError_t {
    // TODO: fix constants in extra
    if !extra.is_null() {
        return hipError_t::ErrorNotSupported;
    }
    unsafe {
        hipModuleLaunchKernel(
            f.base,
            grid_dim_x,
            grid_dim_y,
            grid_dim_z,
            block_dim_x,
            block_dim_y,
            block_dim_z,
            shared_mem_bytes,
            stream,
            kernel_params,
            extra,
        )
    }
}

pub(crate) unsafe fn set_attribute(
    func: &Function,
    attribute: CUfunction_attribute,
    value: i32,
) -> hipError_t {
    match attribute {
        CUfunction_attribute::CU_FUNC_ATTRIBUTE_PTX_VERSION
        | CUfunction_attribute::CU_FUNC_ATTRIBUTE_BINARY_VERSION => {
            return hipError_t::ErrorNotSupported;
        }
        _ => {}
    }
    hipFuncSetAttribute(func.base.0.cast(), hipFuncAttribute(attribute.0), value)
}
