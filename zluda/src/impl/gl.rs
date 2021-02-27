use super::{hipfix, stream};
use crate::hip_call_cuda;
use cuda_types::CUresult;
use hip_runtime_sys::*;

pub(crate) unsafe fn register_buffer(
    resource: *mut hipGraphicsResource_t,
    buffer: u32,
    flags: ::std::os::raw::c_uint,
) -> hipError_t {
    hipfix::init_opengl();
    hipGraphicsGLRegisterBuffer(resource, buffer, flags)
}

pub(crate) unsafe fn register_image(
    resource: *mut hipGraphicsResource_t,
    image: u32,
    target: u32,
    flags: ::std::os::raw::c_uint,
) -> hipError_t {
    hipfix::init_opengl();
    hipGraphicsGLRegisterImage(resource, image, target, flags)
}

pub(crate) unsafe fn map_resources(
    count: ::std::os::raw::c_uint,
    resources: *mut hipGraphicsResource_t,
    stream: *mut stream::Stream,
) -> Result<(), CUresult> {
    let stream = stream::as_hip_stream(stream)?;
    hip_call_cuda! { hipGraphicsMapResources(count as i32, resources, stream) };
    Ok(())
}

pub(crate) unsafe fn unmap_resources(
    count: ::std::os::raw::c_uint,
    resources: *mut hipGraphicsResource_t,
    stream: *mut stream::Stream,
) -> Result<(), CUresult> {
    let stream = stream::as_hip_stream(stream)?;
    hip_call_cuda! { hipGraphicsUnmapResources(count as i32, resources, stream) };
    Ok(())
}
