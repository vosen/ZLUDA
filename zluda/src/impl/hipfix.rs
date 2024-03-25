// This module is the central place for HIP workarounds
use cuda_types::*;
use hip_runtime_sys::*;
use std::{env, ptr};

use super::{function::FunctionData, stream, LiveCheck};

// For some reason HIP does not tolerate hipArraySurfaceLoadStore, even though
// it works just fine
pub(crate) unsafe fn array_3d_create(descriptor: &mut HIP_ARRAY3D_DESCRIPTOR) {
    descriptor.Flags &= !hipArraySurfaceLoadStore;
}

#[must_use]
pub(crate) fn get_non_broken_format(format: hipArray_Format) -> (u32, hipArray_Format) {
    match format {
        hipArray_Format::HIP_AD_FORMAT_HALF => (2, hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT16),
        hipArray_Format::HIP_AD_FORMAT_SIGNED_INT16 => {
            (1, hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT16)
        }
        hipArray_Format::HIP_AD_FORMAT_SIGNED_INT8 => {
            (1, hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT8)
        }
        f => (0, f),
    }
}

#[must_use]
pub(crate) fn get_broken_format(array: &hipArray) -> Option<hipArray_Format> {
    Some(match (array.textureType, array.Format) {
        (2, hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT16) => hipArray_Format::HIP_AD_FORMAT_HALF,
        (1, hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT16) => {
            hipArray_Format::HIP_AD_FORMAT_SIGNED_INT16
        }
        (1, hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT8) => {
            hipArray_Format::HIP_AD_FORMAT_SIGNED_INT8
        }
        (_, _) => return None,
    })
}

// memcpy3d fails when copying array1d arrays, so we mark all layered arrays by
// settings LSB
pub(crate) mod array {
    use super::get_broken_format;
    use crate::{
        hip_call_cuda,
        r#impl::{memcpy3d_from_cuda, memory_type_from_cuda, FromCuda},
    };
    use cuda_types::*;
    use hip_runtime_sys::*;
    use std::{mem, ptr};

    pub(crate) unsafe fn with_resource_desc<T>(
        res_desc: *const CUDA_RESOURCE_DESC,
        res_desc_view: *const HIP_RESOURCE_VIEW_DESC,
        fn_: impl FnOnce(*const HIP_RESOURCE_DESC, *const HIP_RESOURCE_VIEW_DESC) -> T,
    ) -> Result<T, CUresult> {
        let cuda = &*res_desc;
        if cuda.resType == CUresourcetype::CU_RESOURCE_TYPE_ARRAY {
            let mut cuda = *cuda;
            let hip_array = get(cuda.res.array.hArray);
            cuda.res.array.hArray = mem::transmute(hip_array);
            if let Some(hip_array) = hip_array.as_ref() {
                if let Some(format_) = get_broken_format(hip_array) {
                    return if res_desc_view == ptr::null() {
                        let res_desc_view = HIP_RESOURCE_VIEW_DESC {
                            format: resource_view_format(format_, hip_array.NumChannels)?,
                            width: hip_array.width as usize,
                            height: hip_array.height as usize,
                            depth: hip_array.depth as usize,
                            firstMipmapLevel: 0,
                            lastMipmapLevel: 0,
                            firstLayer: 0,
                            lastLayer: 0,
                            reserved: mem::zeroed(),
                        };
                        Ok(fn_(
                            (&cuda as *const CUDA_RESOURCE_DESC).cast::<HIP_RESOURCE_DESC>(),
                            &res_desc_view,
                        ))
                    } else {
                        Err(CUresult::CUDA_ERROR_NOT_SUPPORTED)
                    };
                }
            }
            Ok(fn_(
                (&cuda as *const CUDA_RESOURCE_DESC).cast::<HIP_RESOURCE_DESC>(),
                res_desc_view,
            ))
        } else {
            Ok(fn_(
                (cuda as *const CUDA_RESOURCE_DESC).cast::<HIP_RESOURCE_DESC>(),
                res_desc_view,
            ))
        }
    }

    fn resource_view_format(
        format: hipArray_Format,
        num_channels: u32,
    ) -> Result<HIPresourceViewFormat, CUresult> {
        Ok(match (format, num_channels) {
            (hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT8, 1) => {
                HIPresourceViewFormat::HIP_RES_VIEW_FORMAT_UINT_1X8
            }
            (hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT8, 2) => {
                HIPresourceViewFormat::HIP_RES_VIEW_FORMAT_UINT_2X8
            }
            (hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT8, 4) => {
                HIPresourceViewFormat::HIP_RES_VIEW_FORMAT_UINT_4X8
            }
            (hipArray_Format::HIP_AD_FORMAT_SIGNED_INT8, 1) => {
                HIPresourceViewFormat::HIP_RES_VIEW_FORMAT_SINT_1X8
            }
            (hipArray_Format::HIP_AD_FORMAT_SIGNED_INT8, 2) => {
                HIPresourceViewFormat::HIP_RES_VIEW_FORMAT_SINT_2X8
            }
            (hipArray_Format::HIP_AD_FORMAT_SIGNED_INT8, 4) => {
                HIPresourceViewFormat::HIP_RES_VIEW_FORMAT_SINT_4X8
            }
            (hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT16, 1) => {
                HIPresourceViewFormat::HIP_RES_VIEW_FORMAT_UINT_1X16
            }
            (hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT16, 2) => {
                HIPresourceViewFormat::HIP_RES_VIEW_FORMAT_UINT_2X16
            }
            (hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT16, 4) => {
                HIPresourceViewFormat::HIP_RES_VIEW_FORMAT_UINT_4X16
            }
            (hipArray_Format::HIP_AD_FORMAT_SIGNED_INT16, 1) => {
                HIPresourceViewFormat::HIP_RES_VIEW_FORMAT_SINT_1X16
            }
            (hipArray_Format::HIP_AD_FORMAT_SIGNED_INT16, 2) => {
                HIPresourceViewFormat::HIP_RES_VIEW_FORMAT_SINT_2X16
            }
            (hipArray_Format::HIP_AD_FORMAT_SIGNED_INT16, 4) => {
                HIPresourceViewFormat::HIP_RES_VIEW_FORMAT_SINT_4X16
            }
            (hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT32, 1) => {
                HIPresourceViewFormat::HIP_RES_VIEW_FORMAT_UINT_1X32
            }
            (hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT32, 2) => {
                HIPresourceViewFormat::HIP_RES_VIEW_FORMAT_UINT_2X32
            }
            (hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT32, 4) => {
                HIPresourceViewFormat::HIP_RES_VIEW_FORMAT_UINT_4X32
            }
            (hipArray_Format::HIP_AD_FORMAT_SIGNED_INT32, 1) => {
                HIPresourceViewFormat::HIP_RES_VIEW_FORMAT_SINT_1X32
            }
            (hipArray_Format::HIP_AD_FORMAT_SIGNED_INT32, 2) => {
                HIPresourceViewFormat::HIP_RES_VIEW_FORMAT_SINT_2X32
            }
            (hipArray_Format::HIP_AD_FORMAT_SIGNED_INT32, 4) => {
                HIPresourceViewFormat::HIP_RES_VIEW_FORMAT_SINT_4X32
            }
            (hipArray_Format::HIP_AD_FORMAT_HALF, 1) => {
                HIPresourceViewFormat::HIP_RES_VIEW_FORMAT_FLOAT_1X16
            }
            (hipArray_Format::HIP_AD_FORMAT_HALF, 2) => {
                HIPresourceViewFormat::HIP_RES_VIEW_FORMAT_FLOAT_2X16
            }
            (hipArray_Format::HIP_AD_FORMAT_HALF, 4) => {
                HIPresourceViewFormat::HIP_RES_VIEW_FORMAT_FLOAT_4X16
            }
            (hipArray_Format::HIP_AD_FORMAT_FLOAT, 1) => {
                HIPresourceViewFormat::HIP_RES_VIEW_FORMAT_FLOAT_1X32
            }
            (hipArray_Format::HIP_AD_FORMAT_FLOAT, 2) => {
                HIPresourceViewFormat::HIP_RES_VIEW_FORMAT_FLOAT_2X32
            }
            (hipArray_Format::HIP_AD_FORMAT_FLOAT, 4) => {
                HIPresourceViewFormat::HIP_RES_VIEW_FORMAT_FLOAT_4X32
            }
            _ => return Err(CUresult::CUDA_ERROR_NOT_SUPPORTED),
        })
    }

    pub(crate) fn get(cuda: CUarray) -> hipArray_t {
        (cuda as usize & !3usize) as hipArray_t
    }

    pub(crate) fn get_mipmapped(cuda: CUmipmappedArray) -> (hipMipmappedArray_t, u32) {
        let array = (cuda as usize & !3usize) as hipMipmappedArray_t;
        let broken_flag = (cuda as usize & 3usize) as u32;
        (array, broken_flag)
    }

    pub(crate) fn to_cuda(array: hipArray_t, layered_dims: usize) -> CUarray {
        let a1d_layered = layered_dims as usize;
        ((array as usize) | a1d_layered) as CUarray
    }

    pub(crate) fn get_layered_dimensions(cuda: CUarray) -> usize {
        cuda as usize & 3usize
    }

    pub(crate) fn copy3d_async(
        stream: hipStream_t,
        copy_desc: &CUDA_MEMCPY3D,
    ) -> Result<(), CUresult> {
        let src = get_array(copy_desc.srcMemoryType, copy_desc.srcArray);
        let dst = get_array(copy_desc.dstMemoryType, copy_desc.dstArray);
        match (src, dst) {
            (Some((_, 1)), Some((_, 2))) | (Some((_, 2)), Some((_, 1))) => {
                Err(CUresult::CUDA_ERROR_NOT_SUPPORTED)
            }
            (Some((_, 1)), _) | (_, Some((_, 1))) => {
                hip_call_cuda!(hipMemcpyParam2DAsync(
                    &memcpy3d_to_2d_layered(copy_desc),
                    stream
                ));
                Ok(())
            }
            _ => {
                // hipDrvMemcpy3D does not respect pitch parameter if src or target is an array
                let hip_copy_desc = memcpy3d_from_cuda(copy_desc)?;
                if (hip_copy_desc.srcMemoryType == hipMemoryType::hipMemoryTypeArray
                    || hip_copy_desc.dstMemoryType == hipMemoryType::hipMemoryTypeArray)
                    && (hip_copy_desc.dstPitch > hip_copy_desc.WidthInBytes
                        || hip_copy_desc.srcPitch > hip_copy_desc.WidthInBytes)
                {
                    if hip_copy_desc.srcPitch > hip_copy_desc.WidthInBytes
                        && (hip_copy_desc.srcMemoryType == hipMemoryType::hipMemoryTypeDevice
                            || hip_copy_desc.srcMemoryType == hipMemoryType::hipMemoryTypeHost)
                        && hip_copy_desc.dstMemoryType == hipMemoryType::hipMemoryTypeArray
                    {
                        if hip_copy_desc.srcXInBytes != 0
                            || hip_copy_desc.srcY != 0
                            || hip_copy_desc.srcZ != 0
                        {
                            return Err(CUresult::CUDA_ERROR_NOT_SUPPORTED);
                        }
                        if hip_copy_desc.dstXInBytes != 0 || hip_copy_desc.dstY != 0 {
                            return Err(CUresult::CUDA_ERROR_NOT_SUPPORTED);
                        }
                        let mut temporary_buffer = ptr::null_mut();
                        hip_call_cuda!(hipMalloc(
                            &mut temporary_buffer,
                            hip_copy_desc.WidthInBytes as usize
                                * hip_copy_desc.Height as usize
                                * hip_copy_desc.Depth as usize
                        ));
                        let mut reduce_pitch = hip_copy_desc.clone();
                        reduce_pitch.dstMemoryType = hipMemoryType::hipMemoryTypeDevice;
                        reduce_pitch.dstDevice = hipDeviceptr_t(temporary_buffer);
                        reduce_pitch.dstArray = ptr::null_mut();
                        reduce_pitch.dstZ = 0;
                        hip_call_cuda!(hipDrvMemcpy3DAsync(&reduce_pitch, stream));
                        let mut final_copy = hip_copy_desc.clone();
                        final_copy.srcMemoryType = hipMemoryType::hipMemoryTypeDevice;
                        final_copy.srcDevice = hipDeviceptr_t(temporary_buffer);
                        final_copy.srcPitch = final_copy.WidthInBytes;
                        hip_call_cuda!(hipDrvMemcpy3DAsync(&final_copy, stream));
                        Ok(())
                        /*
                        hip_call_cuda!(hipStreamAddCallback(
                            stream,
                            Some(free_device_allocation),
                            temporary_buffer,
                            0
                        ));
                         */
                    } else {
                        Err(CUresult::CUDA_ERROR_NOT_SUPPORTED)
                    }
                } else {
                    hip_call_cuda!(hipDrvMemcpy3DAsync(&hip_copy_desc, stream));
                    Ok(())
                }
            }
        }
    }

    pub(crate) fn copy3d(copy_desc: &CUDA_MEMCPY3D) -> Result<(), CUresult> {
        let src = get_array(copy_desc.srcMemoryType, copy_desc.srcArray);
        let dst = get_array(copy_desc.dstMemoryType, copy_desc.dstArray);
        match (src, dst) {
            (Some((_, 1)), Some((_, 2))) | (Some((_, 2)), Some((_, 1))) => {
                Err(CUresult::CUDA_ERROR_NOT_SUPPORTED)
            }
            (Some((_, 1)), _) | (_, Some((_, 1))) => {
                hip_call_cuda!(hipMemcpyParam2D(&memcpy3d_to_2d_layered(copy_desc)));
                Ok(())
            }
            _ => {
                // hipDrvMemcpy3D does not respect pitch parameter if src or target is an array
                let hip_copy_desc = memcpy3d_from_cuda(copy_desc)?;
                if (hip_copy_desc.srcMemoryType == hipMemoryType::hipMemoryTypeArray
                    || hip_copy_desc.dstMemoryType == hipMemoryType::hipMemoryTypeArray)
                    && (hip_copy_desc.dstPitch > hip_copy_desc.WidthInBytes
                        || hip_copy_desc.srcPitch > hip_copy_desc.WidthInBytes)
                {
                    if hip_copy_desc.srcPitch > hip_copy_desc.WidthInBytes
                        && (hip_copy_desc.srcMemoryType == hipMemoryType::hipMemoryTypeDevice
                            || hip_copy_desc.srcMemoryType == hipMemoryType::hipMemoryTypeHost)
                        && hip_copy_desc.dstMemoryType == hipMemoryType::hipMemoryTypeArray
                    {
                        if hip_copy_desc.srcXInBytes != 0
                            || hip_copy_desc.srcY != 0
                            || hip_copy_desc.srcZ != 0
                        {
                            return Err(CUresult::CUDA_ERROR_NOT_SUPPORTED);
                        }
                        if hip_copy_desc.dstXInBytes != 0 || hip_copy_desc.dstY != 0 {
                            return Err(CUresult::CUDA_ERROR_NOT_SUPPORTED);
                        }
                        let mut temporary_buffer = ptr::null_mut();
                        hip_call_cuda!(hipMalloc(
                            &mut temporary_buffer,
                            hip_copy_desc.WidthInBytes as usize
                                * hip_copy_desc.Height as usize
                                * hip_copy_desc.Depth as usize
                        ));
                        let mut reduce_pitch = hip_copy_desc.clone();
                        reduce_pitch.dstMemoryType = hipMemoryType::hipMemoryTypeDevice;
                        reduce_pitch.dstDevice = hipDeviceptr_t(temporary_buffer);
                        reduce_pitch.dstArray = ptr::null_mut();
                        reduce_pitch.dstZ = 0;
                        hip_call_cuda!(hipDrvMemcpy3D(&reduce_pitch));
                        let mut final_copy = hip_copy_desc.clone();
                        final_copy.srcMemoryType = hipMemoryType::hipMemoryTypeDevice;
                        final_copy.srcDevice = hipDeviceptr_t(temporary_buffer);
                        final_copy.srcPitch = final_copy.WidthInBytes;
                        hip_call_cuda!(hipDrvMemcpy3D(&final_copy));
                        hip_call_cuda!(hipFree(temporary_buffer));
                        Ok(())
                    } else {
                        Err(CUresult::CUDA_ERROR_NOT_SUPPORTED)
                    }
                } else {
                    hip_call_cuda!(hipDrvMemcpy3D(&hip_copy_desc));
                    Ok(())
                }
            }
        }
    }

    fn memcpy3d_to_2d_layered(desc_3d: &CUDA_MEMCPY3D) -> hip_Memcpy2D {
        hip_Memcpy2D {
            srcXInBytes: desc_3d.srcXInBytes,
            srcY: desc_3d.srcY,
            srcMemoryType: memory_type_from_cuda(desc_3d.srcMemoryType),
            srcHost: desc_3d.srcHost,
            srcDevice: FromCuda::from_cuda(desc_3d.srcDevice),
            srcArray: get(desc_3d.srcArray),
            srcPitch: desc_3d.srcPitch,
            dstXInBytes: desc_3d.dstXInBytes,
            dstY: desc_3d.dstY,
            dstMemoryType: memory_type_from_cuda(desc_3d.dstMemoryType),
            dstHost: desc_3d.dstHost,
            dstDevice: FromCuda::from_cuda(desc_3d.dstDevice),
            dstArray: get(desc_3d.dstArray),
            dstPitch: desc_3d.dstPitch,
            WidthInBytes: desc_3d.WidthInBytes,
            Height: desc_3d.Depth,
        }
    }

    fn get_array(type_: CUmemorytype, array: CUarray) -> Option<(hipArray_t, usize)> {
        if type_ == CUmemorytype::CU_MEMORYTYPE_ARRAY {
            let dims = get_layered_dimensions(array);
            Some((get(array), dims))
        } else {
            None
        }
    }
}

// Somehow if we get a global with hipModuleGetGlobal and pass NULL as bytes,
// then this global is later unusable (e.g. copying to it returns
// CUDA_ERROR_INVALID_VALUE)
pub(crate) unsafe fn module_get_global(
    dptr: *mut hipDeviceptr_t,
    mut bytes: *mut usize,
    hip_module: *mut ihipModule_t,
    name: *const i8,
) -> hipError_t {
    let mut unused = 0usize;
    if bytes == ptr::null_mut() {
        bytes = &mut unused;
    }
    hipModuleGetGlobal(dptr, bytes, hip_module, name)
}

pub(crate) unsafe fn override_occupancy(
    function: &FunctionData,
    min_grid_size: *mut i32,
    block_size: *mut i32,
) {
    let block_size_override = if let Some((min_block_size, max_block_size)) = function.group_size {
        if (*block_size as u32) < min_block_size {
            Some(min_block_size as f64)
        } else if (*block_size as u32) > max_block_size {
            Some(max_block_size as f64)
        } else {
            None
        }
    } else {
        None
    };
    if let Some(new_block_size) = block_size_override {
        let threads = (*min_grid_size as f64) * (*block_size as f64);
        let grid_size = (threads / new_block_size).round();
        *min_grid_size = grid_size as i32;
        *block_size = new_block_size as i32;
    }
}

pub(crate) fn validate_block_size(
    function: &FunctionData,
    block_dim_x: u32,
    block_dim_y: u32,
    block_dim_z: u32,
) -> Result<(), CUresult> {
    if let Some((min_size, max_size)) = function.group_size {
        let requested_size = block_dim_x * block_dim_y * block_dim_z;
        if requested_size < min_size || requested_size > max_size {
            return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
        }
    }
    Ok(())
}

// HACK ALERT
// GeekBench expects device memory allocations to be zeroed out
// We would prefer to zero-out every buffer on allocation, but
// there is no way to zero-out device memory synchronously.
// cuMemset*/hipMemset* are not synchronous:
// (https://docs.nvidia.com/cuda/cuda-driver-api/api-sync-behavior.html#api-sync-behavior__memset)
pub(crate) fn should_zero_buffers() -> Option<bool> {
    let path = env::current_exe().ok()?;
    let name = path.file_name()?;
    let s_name = name.to_str()?.to_ascii_lowercase();
    Some(s_name.contains("geekbench"))
}

// As of ROCm ~5.6, if you call some OpenGL interop functions (hipGraphicsGLRegisterBuffer and such) without
// calling OpenGL interop functions first, you get failures due to OpenGL interop being uninitialized.
// Calling hipGLGetDevices(...) internally calls setupGLInteropOnce which sets up required interop:
// https://github.com/ROCm-Developer-Tools/clr/blob/5a0085e5166640b1a93822454aa6652335740de4/hipamd/src/hip_gl.cpp#L92C36-L92C54
#[allow(unused_must_use)]
pub(crate) fn init_opengl() {
    unsafe { hipGLGetDevices(ptr::null_mut(), ptr::null_mut(), 0, hipGLDeviceList(0)) };
}

// We round up all allocations to be multiple of 4.
// This helps with implementing cuMemsetD8_v2_ptds:
// right now in HIP there's no  _spt for single byte memset,
// there's only one four byte one
pub(crate) fn alloc_round_up(bytesize: usize) -> usize {
    ((bytesize + 3) / 4) * 4
}

//              ┌────────────┬─────────────┐
//              │   Normal   │ _ptds/_ptsz │
// ┌────────────┼────────────┼─────────────┤
// │       NULL │   legacy   │ per-thread  │
// ├────────────┼────────────┼─────────────┤
// │     legacy │   legacy   │   legacy    │
// ├────────────┼────────────┼─────────────┤
// │ per-thread │ per-thread │ per-thread  │
// └────────────┴────────────┴─────────────┘
// Unfortunately, explicit legacy stream does not exist in HIP
// We need to call non-ptds functions if the legacy stream has been explicitly requested
pub(crate) fn as_default_stream_per_thread(
    stream: *mut stream::Stream,
    default_stream_per_thread: bool,
) -> Option<hipStream_t> {
    match (stream, default_stream_per_thread) {
        (stream::CU_STREAM_NULL, false) => Some(hipStreamNull),
        (stream::CU_STREAM_NULL, true) => Some(hipStreamPerThread),
        (stream::CU_STREAM_LEGACY, _) => Some(hipStreamNull),
        (stream::CU_STREAM_PER_THREAD, _) => Some(hipStreamPerThread),
        _ => None,
    }
}

pub(crate) unsafe fn as_hip_stream_per_thread(
    stream: *mut stream::Stream,
    default_stream_per_thread: bool,
) -> Result<hipStream_t, CUresult> {
    Ok(
        match as_default_stream_per_thread(stream, default_stream_per_thread) {
            Some(s) => s,
            None => LiveCheck::as_result(stream)?.base,
        },
    )
}

// I don't know why, but hipModuleOccupancyMaxActiveBlocksPerMultiprocessorWithFlags
// sometimes returns 0, which is clearly wrong
pub(crate) unsafe fn occupancy_max_potential_blocks_per_multiprocessor(num_blocks: *mut i32) {
    *num_blocks = i32::max(*num_blocks, 1);
}
