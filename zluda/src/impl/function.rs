use super::{stream, LiveCheck, ZludaObject};
use crate::{hip_call_cuda, r#impl::hipfix};
use cuda_types::*;
use hip_common::CompilationMode;
use hip_runtime_sys::*;
use std::{ffi::c_void, ptr};

const CU_LAUNCH_PARAM_BUFFER_POINTER: *mut c_void = 1 as *mut _;
const CU_LAUNCH_PARAM_BUFFER_SIZE: *mut c_void = 2 as *mut _;
const CU_LAUNCH_PARAM_END: *mut c_void = 0 as *mut _;
const HIP_LAUNCH_PARAM_END: *mut c_void = 3 as *mut _;

pub(crate) type Function = LiveCheck<FunctionData>;

impl ZludaObject for FunctionData {
    #[cfg(target_pointer_width = "64")]
    const LIVENESS_COOKIE: usize = 0x86b7301e5869d145;
    #[cfg(target_pointer_width = "32")]
    const LIVENESS_COOKIE: usize = 0x5cebb802;
    const LIVENESS_FAIL: CUresult = CUresult::CUDA_ERROR_INVALID_HANDLE;

    fn drop_with_result(&mut self, _by_owner: bool) -> Result<(), CUresult> {
        Ok(())
    }
}

pub(crate) struct FunctionData {
    pub(crate) base: hipFunction_t,
    pub(crate) ptx_version: u32,
    pub(crate) binary_version: u32,
    pub(crate) group_size: Option<(u32, u32)>,
    pub(crate) compilation_mode: CompilationMode,
}

pub(crate) unsafe fn launch_kernel(
    f: *mut Function,
    grid_dim_x: ::std::os::raw::c_uint,
    grid_dim_y: ::std::os::raw::c_uint,
    grid_dim_z: ::std::os::raw::c_uint,
    block_dim_x: ::std::os::raw::c_uint,
    block_dim_y: ::std::os::raw::c_uint,
    mut block_dim_z: ::std::os::raw::c_uint,
    shared_mem_bytes: ::std::os::raw::c_uint,
    stream: *mut stream::Stream,
    kernel_params: *mut *mut ::std::os::raw::c_void,
    extra: *mut *mut ::std::os::raw::c_void,
    default_stream_per_thread: bool,
) -> Result<(), CUresult> {
    let hip_stream = hipfix::as_hip_stream_per_thread(stream, default_stream_per_thread)?;
    let function = LiveCheck::as_result(f)?;
    hipfix::validate_block_size(function, block_dim_x, block_dim_y, block_dim_z)?;
    if function.compilation_mode == CompilationMode::Wave32OnWave64 {
        block_dim_z *= 2;
    }
    if extra != ptr::null_mut() {
        if kernel_params != ptr::null_mut() {
            return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
        }
        let mut extra_params = *(extra as *mut [*mut c_void; 5]);
        if extra_params[0] != CU_LAUNCH_PARAM_BUFFER_POINTER
            || extra_params[2] != CU_LAUNCH_PARAM_BUFFER_SIZE
            || extra_params[4] != CU_LAUNCH_PARAM_END
        {
            return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
        }
        // CU_LAUNCH_PARAM_END is 0, while HIP_LAUNCH_PARAM_END is 3
        extra_params[4] = HIP_LAUNCH_PARAM_END;
        hip_call_cuda!(hipModuleLaunchKernel(
            function.base,
            grid_dim_x,
            grid_dim_y,
            grid_dim_z,
            block_dim_x,
            block_dim_y,
            block_dim_z,
            shared_mem_bytes,
            hip_stream,
            ptr::null_mut(),
            extra_params.as_mut_ptr(),
        ));
    } else {
        hip_call_cuda!(hipModuleLaunchKernel(
            function.base,
            grid_dim_x,
            grid_dim_y,
            grid_dim_z,
            block_dim_x,
            block_dim_y,
            block_dim_z,
            shared_mem_bytes,
            hip_stream,
            kernel_params,
            extra,
        ));
    }

    Ok(())
}

pub(crate) unsafe fn occupancy_max_potential_block_size(
    min_grid_size: *mut i32,
    block_size: *mut i32,
    func: *mut Function,
    _block_size_to_dynamic_smem_size: CUoccupancyB2DSize,
    dynamic_smem_size: usize,
    block_size_limit: i32,
) -> Result<(), CUresult> {
    if min_grid_size == ptr::null_mut() || block_size == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    let function = LiveCheck::as_result(func)?;
    hip_call_cuda!(hipModuleOccupancyMaxPotentialBlockSize(
        min_grid_size,
        block_size,
        function.base,
        dynamic_smem_size,
        block_size_limit
    ));
    hipfix::override_occupancy(function, min_grid_size, block_size);
    if function.compilation_mode == CompilationMode::Wave32OnWave64 {
        *block_size /= 2;
    }
    Ok(())
}

pub(crate) unsafe fn occupancy_max_potential_blocks_per_multiprocessor(
    num_blocks: *mut i32,
    func: *mut LiveCheck<FunctionData>,
    mut block_size: i32,
    dynamic_smem_size: usize,
    flags: u32,
) -> Result<(), CUresult> {
    let function = LiveCheck::as_result(func)?;
    if function.compilation_mode == CompilationMode::Wave32OnWave64 {
        block_size *= 2;
    }
    hip_call_cuda!(hipModuleOccupancyMaxActiveBlocksPerMultiprocessorWithFlags(
        num_blocks,
        function.base,
        block_size,
        dynamic_smem_size,
        flags,
    ));
    hipfix::occupancy_max_potential_blocks_per_multiprocessor(num_blocks);
    Ok(())
}

pub(crate) unsafe fn get_attribute(
    pi: *mut i32,
    attrib: hipFunction_attribute,
    func: *mut LiveCheck<FunctionData>,
) -> Result<(), CUresult> {
    let function = LiveCheck::as_result(func)?;

    match CUfunction_attribute(attrib.0) {
        CUfunction_attribute::CU_FUNC_ATTRIBUTE_PTX_VERSION => {
            *pi = function.ptx_version as i32;
            return Ok(());
        }
        CUfunction_attribute::CU_FUNC_ATTRIBUTE_BINARY_VERSION => {
            *pi = function.binary_version as i32;
            return Ok(());
        }
        CUfunction_attribute::CU_FUNC_ATTRIBUTE_PREFERRED_SHARED_MEMORY_CARVEOUT => {
            *pi = -1;
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
    hip_call_cuda!(hipFuncGetAttribute(pi, attrib, function.base));
    if attrib == hipFunction_attribute::HIP_FUNC_ATTRIBUTE_NUM_REGS {
        // For a completely empty kernel CUDA 11.8 returns 2 regs
        // HIP returns zero
        // Kokkos relies on this property being non-zero
        *pi = i32::max(*pi, 1);
    }
    if attrib == hipFunction_attribute::HIP_FUNC_ATTRIBUTE_MAX_THREADS_PER_BLOCK {
        if function.compilation_mode == CompilationMode::Wave32OnWave64 {
            *pi /= 2;
        }
    }
    Ok(())
}

pub(crate) unsafe fn set_attribute(
    func: *mut LiveCheck<FunctionData>,
    attrib: hipFunction_attribute,
    requested_value: i32,
) -> Result<(), CUresult> {
    let function = LiveCheck::as_result(func)?;
    match attrib {
        // Required by xgboost
        hipFunction_attribute::HIP_FUNC_ATTRIBUTE_MAX_DYNAMIC_SHARED_SIZE_BYTES => {
            let mut current_value = 0;
            hip_call_cuda! { hipFuncGetAttribute(&mut current_value, hipFunction_attribute::HIP_FUNC_ATTRIBUTE_MAX_DYNAMIC_SHARED_SIZE_BYTES, function.base) };
            if requested_value > current_value {
                Err(CUresult::CUDA_ERROR_NOT_SUPPORTED)
            } else {
                Ok(())
            }
        }
        // Can't set attributes in HIP
        _ => Err(CUresult::CUDA_ERROR_NOT_SUPPORTED),
    }
}
