use crate::r#impl::context::{self, Context};
use cl3::{
    ext::CL_MEM_READ_WRITE,
    gl::{cl_context, cl_int, cl_mem, cl_mem_flags, cl_uint},
};
use cuda_types::cuda::*;
use hip_runtime_sys::{
    hipArray_t, hipError_t, hipGraphicsMapResources, hipGraphicsResource_t,
    hipGraphicsSubResourceGetMappedArray, hipGraphicsUnmapResources, hipStream_t,
};
use std::{ffi::c_void, mem, ptr};
use windows::{
    core::Interface,
    Win32::Graphics::{
        Direct3D11::{ID3D11Device, ID3D11Resource, ID3D11Texture2D, ID3D11Texture3D},
        Dxgi::IDXGIAdapter,
    },
};
use zluda_common::FromCuda;

pub(crate) struct OpenCLContext {
    _dll: libloading::os::windows::Library,
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
    ctx: opencl3::context::Context,
}

//                        ██
//                      ██  ██
//                    ██      ██
//                  ██          ██
//                  ██          ██
//                ██              ██
//              ██      ██████      ██
//              ██      ██████      ██
//            ██        ██████        ██
//            ██        ██████        ██
//          ██          ██████          ██
//        ██            ██████            ██
//        ██            ██████            ██
//      ██              ██████              ██
//      ██                                  ██
//    ██                ██████                ██
//    ██                ██████                ██
//  ██                  ██████                  ██
//  ██                                          ██
//    ██████████████████████████████████████████
//
// Warning: This code is very dumb. HIP doesn't have native support for D3D11
// interop. Instead we rely on the fact that AMD's HIP runtime and OpenCL
// runtime share the codebase. That means we can create OpenCL memory objects
// from D3D11 resources, pass them as HIP ones and everything more or less works
pub(crate) unsafe fn d3d11_register_resource(
    cuda_resource: &mut cuda_types::cuda::CUgraphicsResource,
    d3d_dresource: &ID3D11Resource,
    _flags: ::core::ffi::c_uint,
) -> CUresult {
    if let Ok(d3d_dresource) = d3d_dresource.cast::<ID3D11Texture3D>() {
        let device = d3d_dresource
            .GetDevice()
            .map_err(|_| CUerror::DEVICE_UNAVAILABLE)?;
        let mut errcode = 0;
        let clmem = with_opencl_context(&device, |opencl| {
            (opencl.cl_create_from_d3d11_texture_3d_khr)(
                opencl.ctx.get(),
                CL_MEM_READ_WRITE,
                d3d_dresource.as_raw(),
                0,
                &mut errcode,
            )
        })?;
        if errcode != cl3::error_codes::CL_SUCCESS {
            return CUresult::ERROR_UNKNOWN;
        }
        *cuda_resource = clmem.cast();
        return Ok(());
    } else if let Ok(d3d_dresource) = d3d_dresource.cast::<ID3D11Texture2D>() {
        let device = d3d_dresource
            .GetDevice()
            .map_err(|_| CUerror::DEVICE_UNAVAILABLE)?;
        let mut errcode = 0;
        let clmem = with_opencl_context(&device, |opencl| {
            (opencl.cl_create_from_d3d11_texture_2d_khr)(
                opencl.ctx.get(),
                CL_MEM_READ_WRITE,
                d3d_dresource.as_raw(),
                0,
                &mut errcode,
            )
        })?;
        if errcode != cl3::error_codes::CL_SUCCESS {
            return CUresult::ERROR_UNKNOWN;
        }
        *cuda_resource = clmem.cast();
        return Ok(());
    }
    CUresult::ERROR_INVALID_VALUE
}

fn with_opencl_context<T>(
    d3d_device: &ID3D11Device,
    fn_: impl FnOnce(&OpenCLContext) -> T,
) -> Result<T, CUerror> {
    let cu_ctx: CUcontext = context::get_current_context()?;
    let ctx: &Context = FromCuda::<_, CUerror>::from_cuda(&cu_ctx)?;
    let global = crate::r#impl::driver::global_state()?;
    let device = global.devices.get(ctx.device as usize);
    let device = device.ok_or(CUerror::INVALID_DEVICE)?;
    let mut opencl_context = device.opencl_context.lock().map_err(|_| CUerror::UNKNOWN)?;
    let opencl_context = match &*opencl_context {
        Some(c) => c,
        None => {
            let ctx = initialize_opencl_context(d3d_device).map_err(|_| CUerror::UNKNOWN)?;
            *opencl_context = Some(ctx);
            opencl_context.as_mut().unwrap()
        }
    };
    Ok(fn_(opencl_context))
}

fn initialize_opencl_context(device: &ID3D11Device) -> opencl3::Result<OpenCLContext> {
    let adapter = device
        .cast::<IDXGIAdapter>()
        .map_err(|_| opencl3::error_codes::ClError(opencl3::error_codes::CL_INVALID_DEVICE))?;
    let adapter_desc = unsafe { adapter.GetDesc() }
        .map_err(|_| opencl3::error_codes::ClError(opencl3::error_codes::CL_INVALID_DEVICE))?;
    let d3d_luid = adapter_desc.AdapterLuid;
    let platforms = opencl3::platform::get_platforms()?;
    for p in platforms {
        let devices = p.get_devices(cl3::device::CL_DEVICE_TYPE_GPU)?;
        let ocl_dev = devices.into_iter().find(|dev| {
            let dev = opencl3::device::Device::new(*dev);
            let ocl_luid = dev.luid_khr();
            ocl_luid == Ok(unsafe { mem::transmute::<_, [u8; 8]>(d3d_luid) })
        });
        let Some(ocl_dev) = ocl_dev else {
            continue;
        };
        let opencl_context = opencl3::context::Context::from_devices(
            &[ocl_dev],
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
            _dll: dll,
            cl_create_from_d3d11_texture_2d_khr: unsafe {
                mem::transmute(cl_create_from_d3d11_texture_2d_khr)
            },
            cl_create_from_d3d11_texture_3d_khr: unsafe {
                mem::transmute(cl_create_from_d3d11_texture_3d_khr)
            },
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

pub(crate) unsafe fn unregister_resource(_resource: hipGraphicsResource_t) -> hipError_t {
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
