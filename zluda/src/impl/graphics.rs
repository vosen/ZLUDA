use crate::r#impl::context::{self, Context};
use cl3::{
    d3d11::{clCreateFromD3D11Texture2DKHR_fn, clCreateFromD3D11Texture3DKHR_fn},
    ext::{cl_icd::clGetExtensionFunctionAddressForPlatform_t, CL_MEM_READ_WRITE},
    gl::{cl_context, cl_int, cl_mem, cl_mem_flags, cl_uint},
};
use cuda_types::cuda::*;
use hip_runtime_sys::{
    hipArray_t, hipError_t, hipGraphicsMapResources, hipGraphicsResource_t,
    hipGraphicsSubResourceGetMappedArray, hipGraphicsUnmapResources, hipGraphicsUnregisterResource,
    hipStream_t,
};
use std::{ffi::c_void, mem, ptr};
use windows::{
    core::Interface,
    Win32::Graphics::Direct3D11::{ID3D11Device, ID3D11Resource, ID3D11Texture2D, ID3D11Texture3D},
};
use zluda_common::FromCuda;

pub(crate) struct OpenCLContext {
    dll: libloading::os::windows::Library,
    cl_create_from_d3d11_texture_2d_khr: unsafe extern "C" fn(
        context: cl_context,
        flags: cl_mem_flags,
        resource: *mut c_void,
        subresource: cl_uint,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,
    cl_create_from_d3d11_texture_3d_khr: unsafe extern "C" fn(
        context: cl_context,
        flags: cl_mem_flags,
        resource: *mut c_void,
        subresource: cl_uint,
        errcode_ret: *mut cl_int,
    ) -> cl_mem,
    platform: opencl3::platform::Platform,
    ctx: opencl3::context::Context,
}

pub(crate) unsafe fn d3d11_register_resource(
    cuda_resource: &mut cuda_types::cuda::CUgraphicsResource,
    d3d_dresource: &ID3D11Resource,
    _flags: ::core::ffi::c_uint,
) -> CUresult {
    let cu_ctx: CUcontext = context::get_current_context()?;
    let ctx: &Context = FromCuda::<_, CUerror>::from_cuda(&cu_ctx)?;
    let mut state = ctx.state.lock().map_err(|_| CUerror::UNKNOWN)?;
    if let Ok(d3d_dresource) = d3d_dresource.cast::<ID3D11Texture3D>() {
        let device = d3d_dresource
            .GetDevice()
            .map_err(|_| CUerror::DEVICE_UNAVAILABLE)?;
        let ctx = initialize_opencl(&mut state.opencl_context, &device)?;
        let mut errcode = 0;
        let clmem = (ctx.cl_create_from_d3d11_texture_3d_khr)(
            ctx.ctx.get(),
            CL_MEM_READ_WRITE,
            d3d_dresource.as_raw(),
            0,
            &mut errcode,
        );
        *cuda_resource = clmem.cast();
        return mem::transmute(errcode);
    } else if let Ok(d3d_dresource) = d3d_dresource.cast::<ID3D11Texture2D>() {
        let device = d3d_dresource
            .GetDevice()
            .map_err(|_| CUerror::DEVICE_UNAVAILABLE)?;
        let ctx = initialize_opencl(&mut state.opencl_context, &device)?;
        let mut errcode = 0;
        let clmem = (ctx.cl_create_from_d3d11_texture_2d_khr)(
            ctx.ctx.get(),
            CL_MEM_READ_WRITE,
            d3d_dresource.as_raw(),
            0,
            &mut errcode,
        );
        *cuda_resource = clmem.cast();
        return mem::transmute(errcode);
    }
    CUresult::ERROR_INVALID_VALUE
}

fn initialize_opencl<'a>(
    opencl_context: &'a mut Option<OpenCLContext>,
    device: &ID3D11Device,
) -> Result<&'a OpenCLContext, CUerror> {
    match opencl_context {
        Some(c) => Ok(&*c),
        None => {
            let ctx = initialize_context(device).map_err(|_| CUerror::UNKNOWN)?;
            *opencl_context = Some(ctx);
            Ok(opencl_context.as_ref().unwrap())
        }
    }
}

fn initialize_context(device: &ID3D11Device) -> opencl3::Result<OpenCLContext> {
    let platforms = opencl3::platform::get_platforms()?;
    for p in platforms {
        let devices = p.get_devices(cl3::device::CL_DEVICE_TYPE_GPU)?;
        if devices.iter().any(|dev| {
            let dev = opencl3::device::Device::new(*dev);
            dev.vendor_id() != Ok(0x1002)
        }) {
            continue;
        }
        let opencl_context = opencl3::context::Context::from_devices(
            &devices,
            &[
                cl3::ext::CL_CONTEXT_PLATFORM,
                p.id() as _,
                cl3::d3d11::CL_CONTEXT_D3D11_DEVICE_KHR as _,
                device.as_raw() as _,
                0,
            ],
            None,
            ptr::null_mut(),
        )?;
        let dll =
            libloading::os::windows::Library::open_already_loaded("OpenCL").map_err(|_| {
                opencl3::error_codes::ClError(opencl3::error_codes::DLOPEN_RUNTIME_LOAD_FAILED)
            })?;
        let extensions_fn_for_platform = unsafe {
            dll.get::<cl3::ext::clIcdGetFunctionAddressForPlatformKHR_fn>(
                c"clGetExtensionFunctionAddressForPlatform".to_bytes_with_nul(),
            )
        }
        .map_err(|_| {
            opencl3::error_codes::ClError(opencl3::error_codes::DLOPEN_RUNTIME_LOAD_FAILED)
        })?
        .ok_or(opencl3::error_codes::ClError(
            opencl3::error_codes::DLOPEN_RUNTIME_LOAD_FAILED,
        ))?;
        let cl_create_from_d3d11_texture_2d_khr = unsafe {
            extensions_fn_for_platform(p.id(), c"clCreateFromD3D11Texture2DKHR".as_ptr())
        };
        if cl_create_from_d3d11_texture_2d_khr.is_null() {
            return Err(opencl3::error_codes::ClError(
                opencl3::error_codes::DLOPEN_RUNTIME_LOAD_FAILED,
            ));
        }
        let cl_create_from_d3d11_texture_3d_khr = unsafe {
            extensions_fn_for_platform(p.id(), c"clCreateFromD3D11Texture3DKHR".as_ptr())
        };
        if cl_create_from_d3d11_texture_3d_khr.is_null() {
            return Err(opencl3::error_codes::ClError(
                opencl3::error_codes::DLOPEN_RUNTIME_LOAD_FAILED,
            ));
        }
        return Ok(OpenCLContext {
            dll,
            cl_create_from_d3d11_texture_2d_khr: unsafe {
                mem::transmute(cl_create_from_d3d11_texture_2d_khr)
            },
            cl_create_from_d3d11_texture_3d_khr: unsafe {
                mem::transmute(cl_create_from_d3d11_texture_3d_khr)
            },
            platform: p,
            ctx: opencl_context,
        });
    }
    Err(opencl3::error_codes::ClError(
        opencl3::error_codes::CL_DEVICE_NOT_FOUND,
    ))
}

pub(crate) unsafe fn map_resources(
    count: ::core::ffi::c_uint,
    resources: *mut hipGraphicsResource_t,
    stream: hipStream_t,
) -> hipError_t {
    hipGraphicsMapResources(count as i32, resources, stream)
}

pub(crate) unsafe fn unregister_resource(resource: hipGraphicsResource_t) -> hipError_t {
    // hipGraphicsUnregisterResource(resource)
    Ok(())
}

pub(crate) unsafe fn resource_set_map_flags_v2(
    _resource: hipGraphicsResource_t,
    _flags: ::core::ffi::c_uint,
) -> CUresult {
    Ok(())
}

pub(crate) unsafe fn unmap_resources(
    count: ::core::ffi::c_uint,
    resources: *mut hipGraphicsResource_t,
    stream: hipStream_t,
) -> hipError_t {
    hipGraphicsUnmapResources(count as i32, resources, stream)
}

pub(crate) unsafe fn sub_resource_get_mapped_array(
    array: *mut hipArray_t,
    resource: hipGraphicsResource_t,
    array_index: ::core::ffi::c_uint,
    mip_level: ::core::ffi::c_uint,
) -> hipError_t {
    hipGraphicsSubResourceGetMappedArray(array, resource, array_index, mip_level)
}
