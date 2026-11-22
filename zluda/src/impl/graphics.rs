use hip_runtime_sys::{hipMemcpy3D, hipMemcpy3DParms, hipPitchedPtr, hipExtent};
use hip_runtime_sys::{hipArrayGetDescriptor, hipArray_Format, hipMemcpyKind, hipStreamSynchronize, HIP_ARRAY_DESCRIPTOR};
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

// ZLUDA GL synchronization diagnostic: 0=original, 1=HIP stream, 2=HIP+GL.
// Disabled by default; opt-in only for controlled DaVinci Resolve preview tests.
fn zluda_gl_preview_sync_mode() -> u8 {
    std::env::var("ZLUDA_GL_PREVIEW_SYNC")
        .ok()
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap_or(0)
}

#[link(name = "opengl32")]
unsafe extern "system" {
    fn wglGetCurrentContext() -> *mut std::ffi::c_void;
    fn glFinish();
}
pub(crate) unsafe fn map_resources(
    count: ::core::ffi::c_uint,
    resources: *mut hipGraphicsResource_t,
    stream: hipStream_t,
) -> hipError_t {
    let mode = zluda_gl_preview_sync_mode();
    // Only finish GL work on a thread with a current GL context.
    if mode >= 2 && !wglGetCurrentContext().is_null() {
        glFinish();
    }
    hipGraphicsMapResources(count as i32, resources, stream)
}

pub(crate) unsafe fn unregister_resource(resource: hipGraphicsResource_t) -> hipError_t {
    if zluda_gl16f_enabled() {
        if let Ok(mut m) = zluda_gl16f_resources().lock() { m.remove(&(resource as usize)); }
    }
    if let Ok(mut images) = zluda_gl_capture_textures().lock() {
        images.remove(&(resource as usize));
    }
    if let Ok(mut tracked) = registered_gl_resources().lock() {
        if tracked.remove(&(resource as usize)) {
            return hip_runtime_sys::hipGraphicsUnregisterResource(resource);
        }
    }
    Ok(())
}

pub(crate) unsafe fn resource_set_map_flags_v2(
    _resource: hipGraphicsResource_t,
    _flags: ::core::ffi::c_uint,
) -> CUresult {
    Ok(())
}

// ZLUDA_EXPORT_PREVIEW_READBACK_V1 - diagnostic, disabled unless trigger file is present.
// Captures a few mapped RGBA8 frames before returning them from HIP to OpenGL.
fn zluda_capture_export_preview_if_requested(
    count: ::core::ffi::c_uint,
    resources: *mut hipGraphicsResource_t,
    stream: hipStream_t,
) {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static REMAINING: AtomicUsize = AtomicUsize::new(0);
    static SEQUENCE: AtomicUsize = AtomicUsize::new(0);
    let trigger = std::env::temp_dir().join("ZLUDA-CaptureExportPreview.flag");
    if trigger.exists() && std::fs::remove_file(&trigger).is_ok() {
        REMAINING.store(3, Ordering::SeqCst);
    }
    if count != 1 || resources.is_null() {
        return;
    }
    if REMAINING
        .try_update(Ordering::SeqCst, Ordering::SeqCst, |n| n.checked_sub(1))
        .is_err()
    {
        return;
    }
    let index = SEQUENCE.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join("ZLUDA-Export-Readback");
    let _ = std::fs::create_dir_all(&dir);
    let result = (|| -> Result<String, String> {
        let resource = unsafe { *resources };
        if resource.is_null() {
            return Err("resource is null".into());
        }
        let mut array: hipArray_t = std::ptr::null_mut();
        unsafe { hipGraphicsSubResourceGetMappedArray(&mut array, resource, 0, 0) }
            .map_err(|e| format!("GetMappedArray: {:?}", e))?;
        if array.is_null() {
            return Err("mapped array is null".into());
        }
        let mut desc: HIP_ARRAY_DESCRIPTOR = unsafe { std::mem::zeroed() };
        unsafe { hipArrayGetDescriptor(&mut desc, array) }
            .map_err(|e| format!("ArrayGetDescriptor: {:?}", e))?;
        let width = desc.Width;
        let height = desc.Height;
        if width == 0 || width > 1920 || height == 0 || height > 1080 || desc.NumChannels != 4
            || desc.Format != hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT8
        {
            return Err(format!("Unexpected array: {}x{}, channels={}, format={:?}",
                width, height, desc.NumChannels, desc.Format));
        }
        let stride = width.checked_mul(4).ok_or("pitch overflow")?;
        let len = stride.checked_mul(height).ok_or("buffer overflow")?;
        let mut rgba = vec![0u8; len];
        unsafe { hipStreamSynchronize(stream) }
            .map_err(|e| format!("StreamSynchronize: {:?}", e))?;
        let mut copy: hipMemcpy3DParms = unsafe { std::mem::zeroed() };

        copy.srcArray = array;

        copy.dstPtr = hipPitchedPtr {
            ptr: rgba.as_mut_ptr().cast(),
            pitch: stride,
            xsize: stride,
            ysize: height,
        };

        copy.extent = hipExtent {
            width,
            height,
            depth: 1,
        };

        copy.kind = hipMemcpyKind::hipMemcpyDeviceToHost;

        unsafe { hipMemcpy3D(&copy) }
            .map_err(|e| format!(
                "Memcpy3D: {:?}; array={:?}, {}x{}, channels={}, format={:?}, stride={}",
                e, array, width, height, desc.NumChannels, desc.Format, stride
            ))?;
        // 24-bit BMP; Windows Photos opens it without auxiliary conversion.
        let bmp_stride = (width * 3 + 3) & !3;
        let byte_count = 54usize + bmp_stride * height;
        let mut bmp = vec![0u8; byte_count];
        bmp[0..2].copy_from_slice(b"BM");
        bmp[2..6].copy_from_slice(&(byte_count as u32).to_le_bytes());
        bmp[10..14].copy_from_slice(&54u32.to_le_bytes());
        bmp[14..18].copy_from_slice(&40u32.to_le_bytes());
        bmp[18..22].copy_from_slice(&(width as i32).to_le_bytes());
        bmp[22..26].copy_from_slice(&(height as i32).to_le_bytes());
        bmp[26..28].copy_from_slice(&1u16.to_le_bytes());
        bmp[28..30].copy_from_slice(&24u16.to_le_bytes());
        bmp[34..38].copy_from_slice(&(bmp_stride as u32 * height as u32).to_le_bytes());
        for y in 0..height {
            for x in 0..width {
                let src = y * stride + x * 4;
                let dst = 54 + (height - 1 - y) * bmp_stride + x * 3;
                bmp[dst] = rgba[src + 2];
                bmp[dst + 1] = rgba[src + 1];
                bmp[dst + 2] = rgba[src];
            }
        }
        let dst = dir.join(format!("mapped_{:03}_{}x{}.bmp", index, width, height));
        std::fs::write(&dst, bmp).map_err(|e| format!("Write BMP: {e}"))?;
        Ok(format!("SUCCESS {} {}x{}", dst.display(), width, height))
    })();
    use std::io::Write;
    if let Ok(mut log) = std::fs::OpenOptions::new().append(true).create(true)
        .open(dir.join("readback.log"))
    {
        let _ = writeln!(log, "capture #{index}: {}", result.unwrap_or_else(|e| format!("ERROR: {e}")));
    }
}

// ZLUDA_EXPORT_GL_READBACK_V1
// Opt-in diagnostic: capture GL texture after HIP hands it back to OpenGL.
// Does not change CUDA kernels, surface formatting or release timing normally.
static ZLUDA_GL_CAPTURE_TEXTURES: std::sync::OnceLock<
    std::sync::Mutex<std::collections::HashMap<usize, (u32, u32)>>,
> = std::sync::OnceLock::new();

fn zluda_gl_capture_textures() -> &'static std::sync::Mutex<std::collections::HashMap<usize, (u32, u32)>> {
    ZLUDA_GL_CAPTURE_TEXTURES.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

#[link(name = "opengl32")]
unsafe extern "system" {
    fn glIsTexture(texture: u32) -> u8;
    fn glGetIntegerv(pname: u32, data: *mut i32);
    fn glBindTexture(target: u32, texture: u32);
    fn glGetTexLevelParameteriv(target: u32, level: i32, pname: u32, params: *mut i32);
    fn glPixelStorei(pname: u32, param: i32);
    fn glGetTexImage(target: u32, level: i32, format: u32, pixel_type: u32, pixels: *mut std::ffi::c_void);
    fn glGetError() -> u32;
}

fn zluda_gl_capture_log(message: &str) {
    use std::io::Write;
    let dir = std::env::temp_dir().join("ZLUDA-Export-GLCapture");
    let _ = std::fs::create_dir_all(&dir);
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(dir.join("capture.log")) {
        let _ = writeln!(f, "{message}");
    }
}

unsafe fn zluda_gl_capture_after_unmap(
    count: ::core::ffi::c_uint,
    resources: *mut hipGraphicsResource_t,
) {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static LEFT: AtomicUsize = AtomicUsize::new(0);
    static CAPTURE_SMALL: AtomicUsize = AtomicUsize::new(0);
    static CAPTURE_LARGE: AtomicUsize = AtomicUsize::new(0);
    let trigger = std::env::temp_dir().join("ZLUDA-Export-GLCapture.flag");
    if trigger.exists() && std::fs::remove_file(&trigger).is_ok() {
        LEFT.store(120, Ordering::Relaxed);
        CAPTURE_SMALL.store(0, Ordering::Relaxed);
        CAPTURE_LARGE.store(0, Ordering::Relaxed);
        zluda_gl_capture_log("TRIGGER: 120 GL resource unmap opportunities; up to 3 BMP images per resolution");
    }
    if LEFT.load(Ordering::Relaxed) == 0 || resources.is_null() || count == 0 { return; }
    let Ok(map) = zluda_gl_capture_textures().lock() else { return; };
    let selected: Vec<(usize, u32, u32)> = (0..count.min(16))
        .filter_map(|i| {
            let res = unsafe { *resources.add(i as usize) } as usize;
            map.get(&res).map(|(texture, target)| (res, *texture, *target))
        }).collect();
    drop(map);
    if selected.is_empty() { return; }
    let left = LEFT.try_update(Ordering::Relaxed, Ordering::Relaxed, |v| v.checked_sub(1));
    let Ok(remaining_before) = left else { return; };
    let attempt = 120usize.saturating_sub(remaining_before);
    for (_resource, texture, target) in selected {
        if unsafe { wglGetCurrentContext() }.is_null() {
            if attempt <= 3 || attempt % 20 == 0 { zluda_gl_capture_log(&format!("attempt {attempt}: NO_CURRENT_GL_CONTEXT")); }
            continue;
        }
        const GL_TEXTURE_2D: u32 = 0x0DE1;
        const GL_TEXTURE_BINDING_2D: u32 = 0x8069;
        const GL_TEXTURE_WIDTH: u32 = 0x1000;
        const GL_TEXTURE_HEIGHT: u32 = 0x1001;
        const GL_PIXEL_PACK_BUFFER_BINDING: u32 = 0x88ED;
        const GL_PACK_ALIGNMENT: u32 = 0x0D05;
        const GL_PACK_ROW_LENGTH: u32 = 0x0D02;
        const GL_PACK_SKIP_ROWS: u32 = 0x0D03;
        const GL_PACK_SKIP_PIXELS: u32 = 0x0D04;
        const GL_RGBA: u32 = 0x1908;
        const GL_UNSIGNED_BYTE: u32 = 0x1401;
        if target != GL_TEXTURE_2D {
            if attempt <= 3 { zluda_gl_capture_log(&format!("attempt {attempt}: SKIP_GL_TARGET target=0x{target:x}")); }
            continue;
        }
        if unsafe { glIsTexture(texture) } == 0 {
            if attempt <= 3 || attempt % 20 == 0 { zluda_gl_capture_log(&format!("attempt {attempt}: GL_TEXTURE_NOT_VISIBLE id={texture}")); }
            continue;
        }
        let mut pbo = 0i32;
        unsafe { glGetIntegerv(GL_PIXEL_PACK_BUFFER_BINDING, &mut pbo); }
        if pbo != 0 {
            if attempt <= 3 || attempt % 20 == 0 { zluda_gl_capture_log(&format!("attempt {attempt}: PIXEL_PACK_BUFFER_BOUND={pbo}; skipping to avoid corrupting PBO")); }
            continue;
        }
        let mut previous_texture = 0i32;
        unsafe { glGetIntegerv(GL_TEXTURE_BINDING_2D, &mut previous_texture); glBindTexture(GL_TEXTURE_2D, texture); }
        let mut width = 0i32;
        let mut height = 0i32;
        unsafe {
            glGetTexLevelParameteriv(GL_TEXTURE_2D, 0, GL_TEXTURE_WIDTH, &mut width);
            glGetTexLevelParameteriv(GL_TEXTURE_2D, 0, GL_TEXTURE_HEIGHT, &mut height);
        }
        let (counter, label) = match (width, height) {
            (720, 480) => (&CAPTURE_SMALL, "720x480"),
            (1920, 1080) => (&CAPTURE_LARGE, "1920x1080"),
            _ => {
                if attempt <= 3 || attempt % 20 == 0 { zluda_gl_capture_log(&format!("attempt {attempt}: unexpected GL size {width}x{height} texture={texture}")); }
                unsafe { glBindTexture(GL_TEXTURE_2D, previous_texture as u32); }
                continue;
            }
        };
        if counter.load(Ordering::Relaxed) >= 3 {
            unsafe { glBindTexture(GL_TEXTURE_2D, previous_texture as u32); }
            continue;
        }
        let Some(stride) = (width as usize).checked_mul(4) else {
            unsafe { glBindTexture(GL_TEXTURE_2D, previous_texture as u32); }
            continue;
        };
        let Some(len) = stride.checked_mul(height as usize) else {
            unsafe { glBindTexture(GL_TEXTURE_2D, previous_texture as u32); }
            continue;
        };
        let mut rgba = vec![0u8; len];
        let mut pack_align = 0i32;
        let mut pack_row_length = 0i32;
        let mut pack_skip_rows = 0i32;
        let mut pack_skip_pixels = 0i32;
        unsafe {
            glGetIntegerv(GL_PACK_ALIGNMENT, &mut pack_align);
            glGetIntegerv(GL_PACK_ROW_LENGTH, &mut pack_row_length);
            glGetIntegerv(GL_PACK_SKIP_ROWS, &mut pack_skip_rows);
            glGetIntegerv(GL_PACK_SKIP_PIXELS, &mut pack_skip_pixels);
            glPixelStorei(GL_PACK_ALIGNMENT, 1);
            glPixelStorei(GL_PACK_ROW_LENGTH, 0);
            glPixelStorei(GL_PACK_SKIP_ROWS, 0);
            glPixelStorei(GL_PACK_SKIP_PIXELS, 0);
            glGetTexImage(GL_TEXTURE_2D, 0, GL_RGBA, GL_UNSIGNED_BYTE, rgba.as_mut_ptr().cast());
            glPixelStorei(GL_PACK_ALIGNMENT, pack_align);
            glPixelStorei(GL_PACK_ROW_LENGTH, pack_row_length);
            glPixelStorei(GL_PACK_SKIP_ROWS, pack_skip_rows);
            glPixelStorei(GL_PACK_SKIP_PIXELS, pack_skip_pixels);
            glBindTexture(GL_TEXTURE_2D, previous_texture as u32);
        }
        let error = unsafe { glGetError() };
        if error != 0 {
            zluda_gl_capture_log(&format!("attempt {attempt}: GL_ERROR=0x{error:x} texture={texture} size={label}"));
            continue;
        }
        let idx = counter.fetch_add(1, Ordering::Relaxed);
        if idx >= 3 { continue; }
        let width = width as usize;
        let height = height as usize;
        let bmp_stride = (width * 3 + 3) & !3;
        let Some(bmp_len) = bmp_stride.checked_mul(height).and_then(|size| size.checked_add(54)) else { continue; };
        let mut bmp = vec![0u8; bmp_len];
        bmp[..2].copy_from_slice(b"BM");
        bmp[2..6].copy_from_slice(&(bmp_len as u32).to_le_bytes());
        bmp[10..14].copy_from_slice(&54u32.to_le_bytes());
        bmp[14..18].copy_from_slice(&40u32.to_le_bytes());
        bmp[18..22].copy_from_slice(&(width as i32).to_le_bytes());
        bmp[22..26].copy_from_slice(&(height as i32).to_le_bytes());
        bmp[26..28].copy_from_slice(&1u16.to_le_bytes());
        bmp[28..30].copy_from_slice(&24u16.to_le_bytes());
        bmp[34..38].copy_from_slice(&((bmp_stride * height) as u32).to_le_bytes());
        for y in 0..height {
            for x in 0..width {
                let src = y * stride + x * 4;
                let dst = 54 + y * bmp_stride + x * 3;
                bmp[dst] = rgba[src + 2];
                bmp[dst + 1] = rgba[src + 1];
                bmp[dst + 2] = rgba[src];
            }
        }
        let dir = std::env::temp_dir().join("ZLUDA-Export-GLCapture");
        let path = dir.join(format!("gl_{label}_{idx:03}.bmp"));
        match std::fs::write(&path, bmp) {
            Ok(()) => zluda_gl_capture_log(&format!("SUCCESS: {} texture={texture}", path.display())),
            Err(e) => zluda_gl_capture_log(&format!("SAVE_ERROR: {} {e}", path.display())),
        }
    }
}

// ZLUDA_GL_CPU_ROUNDTRIP_V2
// EXPERIMENTAL, opt-in, limited to RGBA16F / RGBA8 textures at 720x480 or 1920x1080.
// Checks whether making a CPU-side GL roundtrip after CUDA/HIP unmap
// repairs the distorted Deliver viewer on AMD. Not a production fix.
#[link(name = "opengl32")]
unsafe extern "system" {
    fn glTexSubImage2D(
        target: u32, level: i32, xoffset: i32, yoffset: i32,
        width: i32, height: i32, format: u32, pixel_type: u32,
        pixels: *const std::ffi::c_void,
    );
}

fn zluda_reupload_log(message: &str) {
    use std::io::Write;
    let dir = std::env::temp_dir().join("ZLUDA-GL-Reupload");
    let _ = std::fs::create_dir_all(&dir);
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(dir.join("result.log")) {
        let _ = writeln!(f, "{message}");
    }
}

unsafe fn zluda_gl_reupload_after_unmap(
    count: ::core::ffi::c_uint,
    resources: *mut hipGraphicsResource_t,
) {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static PROBES: AtomicUsize = AtomicUsize::new(0);
    static UPLOADS: AtomicUsize = AtomicUsize::new(0);
    if std::env::var_os("ZLUDA_GL_CPU_ROUNDTRIP").as_deref() != Some(std::ffi::OsStr::new("1")) {
        return;
    }
    if resources.is_null() || count == 0 { return; }
    let probe = PROBES.fetch_add(1, Ordering::Relaxed);
    if probe > 500 { return; }
    if unsafe { wglGetCurrentContext() }.is_null() {
        if probe < 3 { zluda_reupload_log("SKIP: no current GL context"); }
        return;
    }
    let Ok(registered) = zluda_gl_capture_textures().lock() else { return; };
    let selected: Vec<(u32, u32)> = (0..count.min(16))
        .filter_map(|i| {
            let key = unsafe { *resources.add(i as usize) } as usize;
            registered.get(&key).copied()
        })
        .collect();
    drop(registered);
    if selected.is_empty() {
        if probe < 3 { zluda_reupload_log("SKIP: no tracked OpenGL images"); }
        return;
    }

    const GL_TEXTURE_2D: u32 = 0x0DE1;
    const GL_TEXTURE_BINDING_2D: u32 = 0x8069;
    const GL_TEXTURE_WIDTH: u32 = 0x1000;
    const GL_TEXTURE_HEIGHT: u32 = 0x1001;
    const GL_TEXTURE_INTERNAL_FORMAT: u32 = 0x1003;
    const GL_RGBA8: i32 = 0x8058;
    const GL_RGBA16F: i32 = 0x881A;
    const GL_PIXEL_PACK_BUFFER_BINDING: u32 = 0x88ED;
    const GL_PIXEL_UNPACK_BUFFER_BINDING: u32 = 0x88EF;
    const GL_PACK_ALIGNMENT: u32 = 0x0D05;
    const GL_PACK_ROW_LENGTH: u32 = 0x0D02;
    const GL_PACK_SKIP_ROWS: u32 = 0x0D03;
    const GL_PACK_SKIP_PIXELS: u32 = 0x0D04;
    const GL_UNPACK_ALIGNMENT: u32 = 0x0CF5;
    const GL_UNPACK_ROW_LENGTH: u32 = 0x0CF2;
    const GL_UNPACK_SKIP_ROWS: u32 = 0x0CF3;
    const GL_UNPACK_SKIP_PIXELS: u32 = 0x0CF4;
    const GL_RGBA: u32 = 0x1908;
    const GL_UNSIGNED_BYTE: u32 = 0x1401;
    const GL_HALF_FLOAT: u32 = 0x140B;

    for (texture, target) in selected {
        if target != GL_TEXTURE_2D || unsafe { glIsTexture(texture) } == 0 { continue; }
        if UPLOADS.load(Ordering::Relaxed) >= 90 { return; }
        let mut pbo_pack = 0i32;
        let mut pbo_unpack = 0i32;
        unsafe {
            glGetIntegerv(GL_PIXEL_PACK_BUFFER_BINDING, &mut pbo_pack);
            glGetIntegerv(GL_PIXEL_UNPACK_BUFFER_BINDING, &mut pbo_unpack);
        }
        if pbo_pack != 0 || pbo_unpack != 0 {
            if probe < 3 { zluda_reupload_log("SKIP: PBO binding active"); }
            continue;
        }
        let mut old_texture = 0i32;
        unsafe {
            glGetIntegerv(GL_TEXTURE_BINDING_2D, &mut old_texture);
            glBindTexture(GL_TEXTURE_2D, texture);
        }
        let mut width = 0i32;
        let mut height = 0i32;
        let mut internal_format = 0i32;
        unsafe {
            glGetTexLevelParameteriv(GL_TEXTURE_2D, 0, GL_TEXTURE_WIDTH, &mut width);
            glGetTexLevelParameteriv(GL_TEXTURE_2D, 0, GL_TEXTURE_HEIGHT, &mut height);
            glGetTexLevelParameteriv(GL_TEXTURE_2D, 0, GL_TEXTURE_INTERNAL_FORMAT, &mut internal_format);
        }
        // Preserve exact pixel type: RGBA16F must NOT pass through an 8-bit conversion.
        if !matches!((width, height), (720, 480) | (1920, 1080))
            || (internal_format != GL_RGBA8 && internal_format != GL_RGBA16F)
        {
            if probe < 3 {
                zluda_reupload_log(&format!(
                    "SKIP: texture={texture}, {width}x{height}, format=0x{internal_format:x}"
                ));
            }
            unsafe { glBindTexture(GL_TEXTURE_2D, old_texture as u32); }
            continue;
        }
        let (pixel_type, bytes_per_pixel, fmt) = if internal_format == GL_RGBA16F {
            (GL_HALF_FLOAT, 8usize, "RGBA16F")
        } else {
            (GL_UNSIGNED_BYTE, 4usize, "RGBA8")
        };
        let Some(len) = (width as usize)
            .checked_mul(height as usize)
            .and_then(|n| n.checked_mul(bytes_per_pixel))
        else {
            unsafe { glBindTexture(GL_TEXTURE_2D, old_texture as u32); }
            continue;
        };
        // A u16 buffer has sufficient alignment for both GL_HALF_FLOAT and GL_UNSIGNED_BYTE.
        let mut pixels = vec![0u16; len / 2];
        let mut old_pixel_store = [0i32; 8];
        let names = [
            GL_PACK_ALIGNMENT, GL_PACK_ROW_LENGTH, GL_PACK_SKIP_ROWS, GL_PACK_SKIP_PIXELS,
            GL_UNPACK_ALIGNMENT, GL_UNPACK_ROW_LENGTH, GL_UNPACK_SKIP_ROWS, GL_UNPACK_SKIP_PIXELS,
        ];
        for (idx, pname) in names.iter().enumerate() {
            unsafe { glGetIntegerv(*pname, &mut old_pixel_store[idx]); }
        }
        let read_error: u32;
        unsafe {
            for pname in names { glPixelStorei(pname, if pname == GL_PACK_ALIGNMENT || pname == GL_UNPACK_ALIGNMENT { 1 } else { 0 }); }
            glGetTexImage(GL_TEXTURE_2D, 0, GL_RGBA, pixel_type, pixels.as_mut_ptr().cast());
            read_error = glGetError();
            if read_error == 0 {
                glTexSubImage2D(GL_TEXTURE_2D, 0, 0, 0, width, height, GL_RGBA, pixel_type, pixels.as_ptr().cast());
            } else {
                zluda_reupload_log(&format!("READ_ERROR texture={texture} {width}x{height} {fmt} GL_ERROR=0x{read_error:x}"));
            }
            glFinish();
            for (idx, pname) in names.iter().enumerate() { glPixelStorei(*pname, old_pixel_store[idx]); }
            glBindTexture(GL_TEXTURE_2D, old_texture as u32);
        }
        let error = unsafe { glGetError() };
        if read_error != 0 { continue; }
        let done = UPLOADS.fetch_add(1, Ordering::Relaxed) + 1;
        if done <= 3 || done % 30 == 0 || error != 0 {
            zluda_reupload_log(&format!("ROUNDTRIP_V2 #{done} texture={texture} {width}x{height} {fmt} bytes={len} GL_ERROR=0x{error:x}"));
        }
    }
}

pub(crate) unsafe fn unmap_resources(
    count: ::core::ffi::c_uint,
    resources: *mut hipGraphicsResource_t,
    stream: hipStream_t,
) -> hipError_t {
    zluda_capture_export_preview_if_requested(count, resources, stream);
    let mode = zluda_gl_preview_sync_mode();
    if mode >= 1 {
        // Ensure preceding surface-store kernels completed before GL owns image.
        hip_runtime_sys::hipStreamSynchronize(stream)?;
    }
    hipGraphicsUnmapResources(count as i32, resources, stream)?;
    if mode >= 2 {
        hip_runtime_sys::hipStreamSynchronize(stream)?;
    }
    zluda_gl_capture_after_unmap(count, resources);
    zluda_gl_reupload_after_unmap(count, resources);
    Ok(())
}

// ZLUDA_GL16F_PIXEL_PITCH_AB_V1
// A/B-only: correlate the REAL GL texture internal format with the HIP array
// descriptor. Do not alter GL textures or HIP allocations.
static ZLUDA_GL16F_RESOURCES: std::sync::OnceLock<
    std::sync::Mutex<std::collections::HashMap<usize, (i32, i32)>>,
> = std::sync::OnceLock::new();
static ZLUDA_GL16F_ARRAYS: std::sync::OnceLock<
    std::sync::Mutex<std::collections::HashMap<usize, (i32, i32)>>,
> = std::sync::OnceLock::new();

fn zluda_gl16f_resources() -> &'static std::sync::Mutex<std::collections::HashMap<usize, (i32, i32)>> {
    ZLUDA_GL16F_RESOURCES.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}
fn zluda_gl16f_arrays() -> &'static std::sync::Mutex<std::collections::HashMap<usize, (i32, i32)>> {
    ZLUDA_GL16F_ARRAYS.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}
pub(crate) fn zluda_gl16f_enabled() -> bool {
    std::env::var_os("ZLUDA_GL16F_PIXEL_PITCH_AB").as_deref()
        == Some(std::ffi::OsStr::new("1"))
}
pub(crate) fn zluda_gl16f_array_info(array: hipArray_t) -> Option<(i32, i32)> {
    if !zluda_gl16f_enabled() { return None; }
    zluda_gl16f_arrays().lock().ok()?.get(&(array as usize)).copied()
}
pub(crate) fn zluda_gl16f_log(message: &str) {
    use std::io::Write;
    use std::sync::atomic::{AtomicUsize, Ordering};
    static LINES: AtomicUsize = AtomicUsize::new(0);
    if LINES.fetch_add(1, Ordering::Relaxed) > 300 { return; }
    let dir = std::env::temp_dir().join("ZLUDA-GL16F-Pitch");
    let _ = std::fs::create_dir_all(&dir);
    if let Ok(mut log) = std::fs::OpenOptions::new().create(true).append(true)
        .open(dir.join("result.log")) {
        let _ = writeln!(log, "{message}");
    }
}

unsafe fn zluda_gl16f_probe_texture(image: u32, target: u32) -> Option<(i32, i32)> {
    const GL_TEXTURE_2D: u32 = 0x0DE1;
    const GL_TEXTURE_BINDING_2D: u32 = 0x8069;
    const GL_TEXTURE_WIDTH: u32 = 0x1000;
    const GL_TEXTURE_HEIGHT: u32 = 0x1001;
    const GL_TEXTURE_INTERNAL_FORMAT: u32 = 0x1003;
    const GL_RGBA16F: i32 = 0x881A;
    if target != GL_TEXTURE_2D || wglGetCurrentContext().is_null() || glIsTexture(image) == 0 {
        return None;
    }
    let mut prior = 0i32;
    glGetIntegerv(GL_TEXTURE_BINDING_2D, &mut prior);
    glBindTexture(GL_TEXTURE_2D, image);
    let mut w = 0i32;
    let mut h = 0i32;
    let mut internal = 0i32;
    glGetTexLevelParameteriv(GL_TEXTURE_2D, 0, GL_TEXTURE_WIDTH, &mut w);
    glGetTexLevelParameteriv(GL_TEXTURE_2D, 0, GL_TEXTURE_HEIGHT, &mut h);
    glGetTexLevelParameteriv(GL_TEXTURE_2D, 0, GL_TEXTURE_INTERNAL_FORMAT, &mut internal);
    glBindTexture(GL_TEXTURE_2D, prior as u32);
    if internal == GL_RGBA16F && w > 0 && h > 0 {
        Some((w,h))
    } else {
        None
    }
}

pub(crate) unsafe fn sub_resource_get_mapped_array(
    array: *mut hipArray_t,
    resource: hipGraphicsResource_t,
    array_index: ::core::ffi::c_uint,
    mip_level: ::core::ffi::c_uint,
) -> hipError_t {
    hipGraphicsSubResourceGetMappedArray(array, resource, array_index, mip_level)?;
    if zluda_gl16f_enabled() && !array.is_null() && !(*array).is_null() {
        let info = zluda_gl16f_resources().lock().ok()
            .and_then(|m| m.get(&(resource as usize)).copied());
        if let Some((w,h)) = info {
            let mut native: hip_runtime_sys::HIP_ARRAY3D_DESCRIPTOR = std::mem::zeroed();
            if hip_runtime_sys::hipArray3DGetDescriptor(&mut native, *array).is_ok() {
                if native.Width as i32 == w && native.Height as i32 == h && native.NumChannels == 4 {
                    if let Ok(mut m) = zluda_gl16f_arrays().lock() {
                        m.insert(*array as usize, (w, h));
                    }
                    zluda_gl16f_log(&format!("MAP GL_RGBA16F {w}x{h}: HIP_Format={:?} Channels={} Array={:x}", native.Format, native.NumChannels, *array as usize));
                } else {
                    zluda_gl16f_log(&format!("SKIP mapped array: HIP={}x{} channels={}", native.Width, native.Height, native.NumChannels));
                }
            } else {
                zluda_gl16f_log("SKIP: hipArray3DGetDescriptor failed for mapped GL texture");
            }
        }
    }
    if !array.is_null() && !(*array).is_null() {
        crate::r#impl::array::mark_mapped_graphics_array(*array);
    }
    Ok(())
}

// CUDA / OpenGL interop is OPTIONAL in Windows HIP. Resolve 21 uses this
// for preview textures. Dynamically query HIP, never return fake success.
static REGISTERED_GL_RESOURCES: std::sync::OnceLock<
    std::sync::Mutex<std::collections::HashSet<usize>>,
> = std::sync::OnceLock::new();

fn registered_gl_resources() -> &'static std::sync::Mutex<std::collections::HashSet<usize>> {
    REGISTERED_GL_RESOURCES.get_or_init(|| std::sync::Mutex::new(std::collections::HashSet::new()))
}

pub(crate) unsafe fn g_l_register_image(
    resource: &mut CUgraphicsResource,
    image: ::core::ffi::c_uint,
    target: ::core::ffi::c_uint,
    flags: ::core::ffi::c_uint,
) -> CUresult {
    type RegisterFn = unsafe extern "C" fn(
        *mut hipGraphicsResource_t,
        ::core::ffi::c_uint,
        ::core::ffi::c_uint,
        ::core::ffi::c_uint,
    ) -> hipError_t;
    let lib = libloading::os::windows::Library::open_already_loaded("amdhip64_7.dll")
        .map_err(|_| CUerror::NOT_SUPPORTED)?;
    let register = unsafe { lib.get::<RegisterFn>(b"hipGraphicsGLRegisterImage\0") }
        .map_err(|_| CUerror::NOT_SUPPORTED)?;
    let gl16f_info = if zluda_gl16f_enabled() {
        let info = zluda_gl16f_probe_texture(image, target);
        if info.is_none() { zluda_gl16f_log(&format!("REGISTER_SKIP texture={image} target=0x{target:x}: no compatible RGBA16F or GL context")); }
        info
    } else { None };
    let mut hip_resource: hipGraphicsResource_t = std::ptr::null_mut();
    // Experimental Resolve preview fix. The application's GL images are
    // registered with flags=0, then written via CUDA surface objects.
    // HIP's SurfaceLoadStore flag is bit 4; apply ONLY to otherwise unflagged
    // GL images, never to buffers or explicitly flagged resources.
    // Set ZLUDA_GL_KEEP_ORIGINAL_FLAGS=1 to run an A/B test without rebuilding.
    let hip_flags = if flags == 0
        && std::env::var_os("ZLUDA_GL_KEEP_ORIGINAL_FLAGS").is_none()
    {
        4_u32 // hipGraphicsRegisterFlagsSurfaceLoadStore
    } else {
        flags
    };
    if std::env::var_os("ZLUDA_GL_DEBUG_SURFACE").is_some() {
        eprintln!(
            "[ZLUDA GL] image={} target={:#x} CUDA flags={} HIP flags={}",
            image, target, flags, hip_flags
        );
    }
    unsafe { register(&mut hip_resource, image, target, hip_flags) }
        .map_err(CUerror::from)?;
    let mut tracked = registered_gl_resources().lock().map_err(|_| CUerror::UNKNOWN)?;
    tracked.insert(hip_resource as usize);
    if let Some((w,h)) = gl16f_info {
        if let Ok(mut m) = zluda_gl16f_resources().lock() {
            m.insert(hip_resource as usize, (w,h));
        }
        zluda_gl16f_log(&format!("REGISTER GL_RGBA16F {w}x{h} texture={image} resource={:x}", hip_resource as usize));
    }
    if let Ok(mut images) = zluda_gl_capture_textures().lock() {
        images.insert(hip_resource as usize, (image, target));
    }
    *resource = hip_resource.cast();
    Ok(())
}

pub(crate) unsafe fn g_l_register_buffer(
    resource: &mut CUgraphicsResource,
    buffer: ::core::ffi::c_uint,
    flags: ::core::ffi::c_uint,
) -> CUresult {
    type RegisterFn = unsafe extern "C" fn(
        *mut hipGraphicsResource_t,
        ::core::ffi::c_uint,
        ::core::ffi::c_uint,
    ) -> hipError_t;
    let lib = libloading::os::windows::Library::open_already_loaded("amdhip64_7.dll")
        .map_err(|_| CUerror::NOT_SUPPORTED)?;
    let register = unsafe { lib.get::<RegisterFn>(b"hipGraphicsGLRegisterBuffer\0") }
        .map_err(|_| CUerror::NOT_SUPPORTED)?;
    let mut hip_resource: hipGraphicsResource_t = std::ptr::null_mut();
    unsafe { register(&mut hip_resource, buffer, flags) }
        .map_err(CUerror::from)?;
    let mut tracked = registered_gl_resources().lock().map_err(|_| CUerror::UNKNOWN)?;
    tracked.insert(hip_resource as usize);
    *resource = hip_resource.cast();
    Ok(())
}
