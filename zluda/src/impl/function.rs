use crate::r#impl::driver;
use cuda_types::cuda::{CUerror, CUfunction, CUfunction_attribute, CUkernel};
use dark_api::FunctionArgInfo;
use hip_runtime_sys::*;
use std::mem;

pub(crate) struct Function {
    pub(crate) base: hipFunction_t,
    pub(crate) name: std::ffi::CString,
    pub(crate) sm_version: u32,
    pub(crate) explicit_args_size_align: Option<Vec<FunctionArgInfo>>,
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
) -> Result<(), CUerror> {
    match cu_attrib {
        CUfunction_attribute::CU_FUNC_ATTRIBUTE_PTX_VERSION => {
            *pi = func.sm_version as i32;
            return Ok(());
        }
        CUfunction_attribute::CU_FUNC_ATTRIBUTE_BINARY_VERSION => {
            let (major_cap, minor_cap) = driver::global_state()?.compute_capability;
            *pi = major_cap * 10 + minor_cap;
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

const CU_LAUNCH_PARAM_BUFFER_POINTER: usize = 0x01;
const CU_LAUNCH_PARAM_BUFFER_SIZE: usize = 0x02;
const CU_LAUNCH_PARAM_END: usize = 0x00;
const HIP_LAUNCH_PARAM_END: usize = 0x03;

pub(crate) unsafe fn launch_kernel(
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
    // ZLUDA_PREVIEW_RGBA8_AB_V1: A/B-only correction of a single kernel argument.
    // Original Resolve format=10 stores RGBA16F (8 bytes/px), while the mapped
    // 720x480 GL array reports UNSIGNED_INT8 x 4 (RGBA8, 4 bytes/px).
    // Target the exact kernel; never modify caller memory, PTX or other kernels.
    let mut zluda_rgba8_surface_arg = [0u8; 16];
    let mut zluda_rgba8_arg_ptrs = [std::ptr::null_mut::<std::ffi::c_void>(); 16];
    let mut zluda_rgba8_applied = false;
    if std::env::var("ZLUDA_PREVIEW_RGBA8_AB").ok().as_deref() == Some("1")
        && (f.name.as_bytes() == b"_ZN4DVIP35c1_cdb6b90342834e03c0c36cd122abfaa2IfEEviiiijifNS_14BlankingParamsENS_15TransformMatrixES2_PKT_S5_S5_S5_11CudaSurfaceb"
        || f.name.as_bytes() == b"_ZN4DVIP35c1_cdb6b90342834e03c0c36cd122abfaa2ItEEviiiijifNS_14BlankingParamsENS_15TransformMatrixES2_PKT_S5_S5_S5_11CudaSurfaceb")
        && !kernel_params.is_null()
        && extra.is_null()
    {
        let original_surface_arg = *kernel_params.add(14);
        if !original_surface_arg.is_null() {
            std::ptr::copy_nonoverlapping(
                original_surface_arg.cast::<u8>(),
                zluda_rgba8_surface_arg.as_mut_ptr(),
                16,
            );
            let old_format = u32::from_le_bytes(
                zluda_rgba8_surface_arg[8..12].try_into().unwrap(),
            );
            let handle = u64::from_le_bytes(zluda_rgba8_surface_arg[0..8].try_into().unwrap());
            if old_format == 10 && crate::r#impl::surf::zluda_gl16f_is_surface(handle) {
                crate::r#impl::graphics::zluda_gl16f_log(&format!("KERNEL_KEEP_10: genuine RGBA16F surface={handle:x} grid={grid_dim_x}x{grid_dim_y}"));
            } else if old_format == 10 {
                zluda_rgba8_surface_arg[8..12].copy_from_slice(&8u32.to_le_bytes());
                std::ptr::copy_nonoverlapping(
                    kernel_params,
                    zluda_rgba8_arg_ptrs.as_mut_ptr(),
                    16,
                );
                zluda_rgba8_arg_ptrs[14] = zluda_rgba8_surface_arg.as_mut_ptr().cast();
                zluda_rgba8_applied = true;
                use std::sync::atomic::{AtomicUsize, Ordering};
                static LOG_COUNT: AtomicUsize = AtomicUsize::new(0);
                if LOG_COUNT.fetch_add(1, Ordering::Relaxed) < 2000 {
                    if let Some(temp) = std::env::var_os("TEMP") {
                        let path = std::path::PathBuf::from(temp).join("ZLUDA-RGBA8-AB.log");
                        if let Ok(mut file) = std::fs::OpenOptions::new()
                            .create(true).append(true).open(path)
                        {
                            use std::io::Write;
                            let _ = writeln!(file, "Resolve preview format override: 10 -> 8; fp16={} grid={}x{} block={}x{}", f.name.as_bytes() == b"_ZN4DVIP35c1_cdb6b90342834e03c0c36cd122abfaa2ItEEviiiijifNS_14BlankingParamsENS_15TransformMatrixES2_PKT_S5_S5_S5_11CudaSurfaceb", grid_dim_x, grid_dim_y, block_dim_x, block_dim_y);
                        }
                    }
                }
            }
        }
    }
    let kernel_params = if zluda_rgba8_applied {
        zluda_rgba8_arg_ptrs.as_mut_ptr()
    } else {
        kernel_params
    };
    let zluda_launch_result = if let Some(extra) = extra.cast::<[usize; 5]>().as_ref() {
        match extra {
            [CU_LAUNCH_PARAM_BUFFER_POINTER, pointer, CU_LAUNCH_PARAM_BUFFER_SIZE, size, CU_LAUNCH_PARAM_END] =>
            {
                let mut hip_extra: [_; 5] = [
                    CU_LAUNCH_PARAM_BUFFER_POINTER,
                    *pointer,
                    CU_LAUNCH_PARAM_BUFFER_SIZE,
                    *size,
                    HIP_LAUNCH_PARAM_END,
                ];
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
                    hip_extra.as_mut_ptr().cast(),
                )
            }
            _ => hipError_t::ErrorInvalidValue,
        }
    } else {
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
    };

    // ZLUDA_EXPORT_SYNC_AB_V1: test whether the preview surface is destroyed
    // while its last HIP kernel is still in flight. Disabled by default.
    if zluda_rgba8_applied
        && std::env::var("ZLUDA_PREVIEW_KERNEL_SYNC").ok().as_deref() == Some("1")
    {
        return zluda_launch_result.and_then(|_| hipStreamSynchronize(stream));
    }
    zluda_launch_result

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

pub(crate) unsafe fn set_cache_config(
    _func: &Function,
    _config: cuda_types::cuda::CUfunc_cache,
) -> hipError_t {
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::tests::CudaApi;
    use cuda_macros::test_cuda;
    use cuda_types::cuda::{CUdevice_attribute, CUfunction_attribute};
    use std::mem;

    #[test_cuda]
    fn check_versions(api: impl CudaApi) {
        api.cuInit(0);
        let mut ctx = unsafe { mem::zeroed() };
        api.cuCtxCreate_v2(&mut ctx, 0, 0);
        let mut major = 0;
        api.cuDeviceGetAttribute(
            &mut major,
            CUdevice_attribute::CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR,
            0,
        );
        let mut minor = 0;
        api.cuDeviceGetAttribute(
            &mut minor,
            CUdevice_attribute::CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR,
            0,
        );
        let ptx = c".version 8.7
        .target sm_60
        .address_size 64

        .entry foobar() { ret; }
        ";

        let mut module = unsafe { mem::zeroed() };
        api.cuModuleLoadData(&mut module, ptx.as_ptr().cast());
        let mut func = unsafe { mem::zeroed() };
        api.cuModuleGetFunction(&mut func, module, c"foobar".as_ptr().cast());
        let mut binary_version = 0;
        api.cuFuncGetAttribute(
            &mut binary_version,
            CUfunction_attribute::CU_FUNC_ATTRIBUTE_BINARY_VERSION,
            func,
        );
        let mut ptx_version = 0;
        api.cuFuncGetAttribute(
            &mut ptx_version,
            CUfunction_attribute::CU_FUNC_ATTRIBUTE_PTX_VERSION,
            func,
        );
        assert_eq!(binary_version, major * 10 + minor);
        assert_eq!(ptx_version, 60);
    }
}
