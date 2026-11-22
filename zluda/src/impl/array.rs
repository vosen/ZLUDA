use hip_runtime_sys::*;
use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

static MAPPED_GRAPHICS_ARRAYS: OnceLock<Mutex<HashSet<usize>>> =
    OnceLock::new();

pub(crate) fn mark_mapped_graphics_array(array: hipArray_t) {
    let set = MAPPED_GRAPHICS_ARRAYS
        .get_or_init(|| Mutex::new(HashSet::new()));
    if let Ok(mut guard) = set.lock() {
        guard.insert(array as usize);
    }
}

fn is_mapped_graphics_array(array: hipArray_t) -> bool {
    MAPPED_GRAPHICS_ARRAYS.get()
        .and_then(|set| {
            set.lock().ok().map(|guard| {
                guard.contains(&(array as usize))
            })
        })
        .unwrap_or(false)
}


pub(crate) unsafe fn create_v2(
    handle: *mut hipArray_t,
    desc: *const HIP_ARRAY_DESCRIPTOR,
) -> hipError_t {
    hipArrayCreate(handle, desc)
}

pub(crate) unsafe fn destroy(array: hipArray_t) -> hipError_t {
    hipArrayDestroy(array)
}

pub(crate) unsafe fn _3d_create_v2(
    handle: *mut hipArray_t,
    desc: *const HIP_ARRAY3D_DESCRIPTOR,
) -> hipError_t {
    hipArray3DCreate(handle, desc)
}

// Resolve 21: query metadata for CUDA arrays created via cuArray3DCreate_v2.
// HIP provides the corresponding API; do not fabricate a success result.
pub(crate) unsafe fn _3d_get_descriptor_v2(
    desc: *mut HIP_ARRAY3D_DESCRIPTOR,
    array: hipArray_t,
) -> hipError_t {
    hipArray3DGetDescriptor(desc, array)?;
    if !desc.is_null() && crate::r#impl::graphics::zluda_gl16f_array_info(array).is_some() {
        let native = (*desc).Format;
        (*desc).Format = hipArray_Format::HIP_AD_FORMAT_HALF;
        crate::r#impl::graphics::zluda_gl16f_log(&format!("CUDA ARRAY_FORMAT {:?} -> HALF; {}/{}", native, (*desc).Width, (*desc).Height));
    }

    if std::env::var("ZLUDA_GL_2D_DEPTH_ZERO")
        .map_or(false, |v| v == "1")
        && !desc.is_null()
        && is_mapped_graphics_array(array)
    {
        let d = &mut *desc;
        if d.Depth == 1 && d.Height > 0 {
            d.Depth = 0;
        }
    }
    Ok(())
}
