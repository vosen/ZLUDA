#![allow(non_snake_case)]

//! CUDA arrays, texture objects and surface objects.
//!
//! Textures and surfaces are sampled in software (see
//! ptx/lib/zluda_ptx_impl_agx_tex.c): a texture or surface object is the device
//! address of a 128-byte descriptor in the encoding of PTX-formalization's model
//! (semantics/ptx-exec-tex.k, extended with a row pitch in word 14 for pitch-linear
//! textures), and a CUDA array is a dense device allocation in the layout those
//! descriptors read: levels one after the other, each holding all layers, each
//! layer depth x height x width texels.

use crate::driver::{from_io, out, with, Driver};
use cuda_types::cuda::*;
use std::ffi::c_void;

const TEXTURE_MAGIC: u64 = 0x5458_5854;
const SURFACE_MAGIC: u64 = 0x5354_5446;
const DESCRIPTOR_BYTES: u64 = 128;

/// A CUDA array (a level of a mipmapped array is one too, borrowing its memory).
pub(crate) struct Array {
    pub va: u64,
    pub format: CUarray_format,
    pub channels: u32,
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub layers: usize,
    pub levels: u32,
    pub flags: u32,
    owned: bool,
}

pub(crate) struct MipmappedArray {
    array: Box<Array>,
    level_arrays: Vec<Box<Array>>,
}

fn element_bytes(format: CUarray_format) -> Option<usize> {
    use CUarray_format_enum as F;
    Some(match format {
        F::CU_AD_FORMAT_UNSIGNED_INT8 | F::CU_AD_FORMAT_SIGNED_INT8 => 1,
        F::CU_AD_FORMAT_UNSIGNED_INT16 | F::CU_AD_FORMAT_SIGNED_INT16 | F::CU_AD_FORMAT_HALF => 2,
        F::CU_AD_FORMAT_UNSIGNED_INT32 | F::CU_AD_FORMAT_SIGNED_INT32 | F::CU_AD_FORMAT_FLOAT => 4,
        _ => return None,
    })
}

/// The texel type code of a texture descriptor (tmCode of the model).
fn type_code(format: CUarray_format) -> u64 {
    use CUarray_format_enum as F;
    match format {
        F::CU_AD_FORMAT_UNSIGNED_INT8 => 0,
        F::CU_AD_FORMAT_UNSIGNED_INT16 => 1,
        F::CU_AD_FORMAT_UNSIGNED_INT32 => 2,
        F::CU_AD_FORMAT_SIGNED_INT32 => 3,
        F::CU_AD_FORMAT_HALF => 6,
        F::CU_AD_FORMAT_FLOAT => 7,
        F::CU_AD_FORMAT_SIGNED_INT8 => 20,
        _ => 21, // SIGNED_INT16
    }
}

/// The sample format of a surface descriptor (for sust.p / sured.p).
fn sample_format(format: CUarray_format) -> u64 {
    use CUarray_format_enum as F;
    match format {
        F::CU_AD_FORMAT_UNSIGNED_INT8 => 2,
        F::CU_AD_FORMAT_SIGNED_INT8 => 3,
        F::CU_AD_FORMAT_UNSIGNED_INT16 => 6,
        F::CU_AD_FORMAT_SIGNED_INT16 => 7,
        F::CU_AD_FORMAT_HALF => 8,
        F::CU_AD_FORMAT_UNSIGNED_INT32 => 9,
        F::CU_AD_FORMAT_SIGNED_INT32 => 10,
        _ => 11,
    }
}

impl Array {
    pub fn texel_bytes(&self) -> usize {
        element_bytes(self.format).unwrap_or(1) * self.channels as usize
    }
    fn level_dim(&self, level: u32, d: usize) -> usize {
        (d >> level).max(1)
    }
    fn level_texels(&self, level: u32) -> usize {
        self.layers * self.level_dim(level, self.width) * self.level_dim(level, self.height)
            * self.level_dim(level, self.depth)
    }
    fn level_offset(&self, level: u32) -> usize {
        (0..level).map(|l| self.level_texels(l)).sum::<usize>() * self.texel_bytes()
    }
    fn bytes(&self) -> usize {
        self.level_offset(self.levels)
    }
    /// The byte offset of (x bytes, y, slice z) of level 0, a slice being a depth slice or a layer.
    pub fn offset(&self, x: usize, y: usize, z: usize) -> usize {
        (z * self.height + y) * self.width * self.texel_bytes() + x
    }
    pub fn row_pitch(&self) -> usize {
        self.width * self.texel_bytes()
    }
}

/// Validates a 3D array descriptor; returns (width, height, depth, layers).
fn array_shape(d: &CUDA_ARRAY3D_DESCRIPTOR) -> Result<(usize, usize, usize, usize), CUerror> {
    if element_bytes(d.Format).is_none() || !matches!(d.NumChannels, 1 | 2 | 4) || d.Width == 0 {
        return Err(CUerror::INVALID_VALUE);
    }
    let layered = d.Flags & (CUDA_ARRAY3D_LAYERED | CUDA_ARRAY3D_CUBEMAP) != 0;
    if d.Flags & CUDA_ARRAY3D_CUBEMAP != 0 && (d.Width != d.Height || d.Depth == 0 || d.Depth % 6 != 0) {
        return Err(CUerror::INVALID_VALUE);
    }
    Ok(if layered {
        (d.Width, d.Height.max(1), 1, d.Depth.max(1))
    } else {
        (d.Width, d.Height.max(1), d.Depth.max(1), 1)
    })
}

fn create_array(d: &mut Driver, desc: &CUDA_ARRAY3D_DESCRIPTOR, levels: u32) -> Result<Box<Array>, CUerror> {
    let (width, height, depth, layers) = array_shape(desc)?;
    let mut array = Box::new(Array {
        va: 0,
        format: desc.Format,
        channels: desc.NumChannels,
        width,
        height,
        depth,
        layers,
        levels: levels.max(1),
        flags: desc.Flags,
        owned: true,
    });
    array.va = d.ctx.mem_alloc(array.bytes().max(16) as u64).map_err(from_io)?;
    d.ctx.memset(array.va, &[0], array.bytes() as u64).map_err(from_io)?;
    Ok(array)
}

pub(crate) unsafe fn cuArrayCreate_v2(handle: *mut CUarray, desc: *const CUDA_ARRAY_DESCRIPTOR) -> CUresult {
    let desc = desc.as_ref().ok_or(CUerror::INVALID_VALUE)?;
    let desc3 = CUDA_ARRAY3D_DESCRIPTOR {
        Width: desc.Width,
        Height: desc.Height,
        Depth: 0,
        Format: desc.Format,
        NumChannels: desc.NumChannels,
        Flags: 0,
    };
    cuArray3DCreate_v2(handle, &desc3)
}

pub(crate) unsafe fn cuArray3DCreate_v2(handle: *mut CUarray, desc: *const CUDA_ARRAY3D_DESCRIPTOR) -> CUresult {
    let desc = desc.as_ref().ok_or(CUerror::INVALID_VALUE)?;
    let array = with(|d| create_array(d, desc, 1))?;
    out(handle, Box::into_raw(array) as CUarray)
}

pub(crate) unsafe fn array<'a>(handle: CUarray) -> Result<&'a Array, CUerror> {
    (handle as *const Array).as_ref().ok_or(CUerror::INVALID_HANDLE)
}

pub(crate) unsafe fn cuArrayDestroy(handle: CUarray) -> CUresult {
    if handle.is_null() {
        return Err(CUerror::INVALID_HANDLE);
    }
    let array = Box::from_raw(handle as *mut Array);
    if array.owned {
        with(|d| {
            d.sync_all()?;
            d.ctx.mem_free(array.va).map_err(from_io)
        })?;
    }
    Ok(())
}

pub(crate) unsafe fn cuArray3DGetDescriptor_v2(desc: *mut CUDA_ARRAY3D_DESCRIPTOR, handle: CUarray) -> CUresult {
    let a = array(handle)?;
    let layered = a.flags & (CUDA_ARRAY3D_LAYERED | CUDA_ARRAY3D_CUBEMAP) != 0;
    out(
        desc,
        CUDA_ARRAY3D_DESCRIPTOR {
            Width: a.width,
            Height: if a.height == 1 && a.depth == 1 && !layered { 0 } else { a.height },
            Depth: if layered { a.layers } else if a.depth == 1 { 0 } else { a.depth },
            Format: a.format,
            NumChannels: a.channels,
            Flags: a.flags,
        },
    )
}

pub(crate) unsafe fn cuArrayGetDescriptor_v2(desc: *mut CUDA_ARRAY_DESCRIPTOR, handle: CUarray) -> CUresult {
    let a = array(handle)?;
    out(
        desc,
        CUDA_ARRAY_DESCRIPTOR {
            Width: a.width,
            Height: if a.height == 1 { 0 } else { a.height },
            Format: a.format,
            NumChannels: a.channels,
        },
    )
}

pub(crate) unsafe fn cuMipmappedArrayCreate(
    handle: *mut CUmipmappedArray,
    desc: *const CUDA_ARRAY3D_DESCRIPTOR,
    levels: ::core::ffi::c_uint,
) -> CUresult {
    let desc = desc.as_ref().ok_or(CUerror::INVALID_VALUE)?;
    if levels == 0 {
        return Err(CUerror::INVALID_VALUE);
    }
    let array = with(|d| create_array(d, desc, levels))?;
    let level_arrays = (0..array.levels)
        .map(|l| {
            Box::new(Array {
                va: array.va + array.level_offset(l) as u64,
                width: array.level_dim(l, array.width),
                height: array.level_dim(l, array.height),
                depth: array.level_dim(l, array.depth),
                levels: 1,
                owned: false,
                ..*array
            })
        })
        .collect();
    out(handle, Box::into_raw(Box::new(MipmappedArray { array, level_arrays })) as CUmipmappedArray)
}

pub(crate) unsafe fn cuMipmappedArrayGetLevel(level: *mut CUarray, handle: CUmipmappedArray, n: ::core::ffi::c_uint) -> CUresult {
    let m = (handle as *const MipmappedArray).as_ref().ok_or(CUerror::INVALID_HANDLE)?;
    let a = m.level_arrays.get(n as usize).ok_or(CUerror::INVALID_VALUE)?;
    out(level, &**a as *const Array as CUarray)
}

pub(crate) unsafe fn cuMipmappedArrayDestroy(handle: CUmipmappedArray) -> CUresult {
    if handle.is_null() {
        return Err(CUerror::INVALID_HANDLE);
    }
    let m = Box::from_raw(handle as *mut MipmappedArray);
    with(|d| {
        d.sync_all()?;
        d.ctx.mem_free(m.array.va).map_err(from_io)
    })
}

// ---------------------------------------------------------------- texture and surface objects

unsafe fn write_descriptor(d: &mut Driver, words: &[u64; 16]) -> Result<u64, CUerror> {
    let va = d.ctx.mem_alloc(DESCRIPTOR_BYTES).map_err(from_io)?;
    let p = d.ctx.cpu_ptr(va, DESCRIPTOR_BYTES).map_err(from_io)? as *mut u64;
    std::ptr::copy_nonoverlapping(words.as_ptr(), p, 16);
    Ok(va)
}

/// The memory a texture or surface reads: base address, element format, channels, dims, layers,
/// levels, flags (bit 0 depth, bit 1 cube) and row pitch (0: dense).
struct Image {
    base: u64,
    format: CUarray_format,
    channels: u32,
    dims: [usize; 3],
    layers: usize,
    levels: u32,
    cube: bool,
    pitch: usize,
}

unsafe fn image(r: &CUDA_RESOURCE_DESC) -> Result<Image, CUerror> {
    use CUresourcetype_enum as R;
    let from_array = |a: &Array| Image {
        base: a.va,
        format: a.format,
        channels: a.channels,
        dims: [a.width, a.height, a.depth],
        layers: a.layers,
        levels: a.levels,
        cube: a.flags & CUDA_ARRAY3D_CUBEMAP != 0,
        pitch: 0,
    };
    Ok(match r.resType {
        R::CU_RESOURCE_TYPE_ARRAY => from_array(array(r.res.array.hArray)?),
        R::CU_RESOURCE_TYPE_MIPMAPPED_ARRAY => {
            let m = (r.res.mipmap.hMipmappedArray as *const MipmappedArray).as_ref().ok_or(CUerror::INVALID_HANDLE)?;
            from_array(&m.array)
        }
        R::CU_RESOURCE_TYPE_LINEAR => {
            let l = r.res.linear;
            let texel = element_bytes(l.format).ok_or(CUerror::INVALID_VALUE)? * l.numChannels as usize;
            Image {
                base: l.devPtr.0 as u64,
                format: l.format,
                channels: l.numChannels,
                dims: [l.sizeInBytes / texel.max(1), 1, 1],
                layers: 1,
                levels: 1,
                cube: false,
                pitch: 0,
            }
        }
        R::CU_RESOURCE_TYPE_PITCH2D => {
            let p = r.res.pitch2D;
            Image {
                base: p.devPtr.0 as u64,
                format: p.format,
                channels: p.numChannels,
                dims: [p.width, p.height, 1],
                layers: 1,
                levels: 1,
                cube: false,
                pitch: p.pitchInBytes,
            }
        }
        _ => return Err(CUerror::INVALID_VALUE),
    })
}

/// CUaddress_mode -> the descriptor's address mode (0 wrap, 1 clamp, 2 border, 3 mirror).
fn address_mode(m: CUaddress_mode) -> u64 {
    use CUaddress_mode_enum as A;
    match m {
        A::CU_TR_ADDRESS_MODE_WRAP => 0,
        A::CU_TR_ADDRESS_MODE_MIRROR => 3,
        A::CU_TR_ADDRESS_MODE_BORDER => 2,
        _ => 1,
    }
}

/// The descriptor of a texture object.
fn texture_words(img: &Image, t: &CUDA_TEXTURE_DESC) -> Result<[u64; 16], CUerror> {
    if element_bytes(img.format).is_none() || !matches!(img.channels, 1 | 2 | 4) {
        return Err(CUerror::INVALID_VALUE);
    }
    // 8- and 16-bit integers read as normalized floats unless CU_TRSF_READ_AS_INTEGER
    let small_integer =
        matches!(element_bytes(img.format), Some(1 | 2)) && img.format != CUarray_format::CU_AD_FORMAT_HALF;
    let read_normalized = small_integer && t.flags & CU_TRSF_READ_AS_INTEGER == 0;
    let normalized = t.flags & CU_TRSF_NORMALIZED_COORDINATES != 0;
    let linear = |f: CUfilter_mode| (f == CUfilter_mode::CU_TR_FILTER_MODE_LINEAR) as u64;
    Ok([
        TEXTURE_MAGIC,
        img.base,
        type_code(img.format),
        img.channels as u64,
        linear(t.filterMode),
        address_mode(t.addressMode[0]),
        normalized as u64,
        read_normalized as u64,
        img.layers as u64,
        img.dims[0] as u64,
        img.dims[1] as u64,
        img.dims[2] as u64,
        img.levels as u64,
        (img.cube as u64) << 1,
        img.pitch as u64,
        linear(t.mipmapFilterMode),
    ])
}

pub(crate) unsafe fn cuTexObjectCreate(
    handle: *mut CUtexObject,
    resource: *const CUDA_RESOURCE_DESC,
    texture: *const CUDA_TEXTURE_DESC,
    _view: *const CUDA_RESOURCE_VIEW_DESC,
) -> CUresult {
    let r = resource.as_ref().ok_or(CUerror::INVALID_VALUE)?;
    let t = texture.as_ref().ok_or(CUerror::INVALID_VALUE)?;
    let words = texture_words(&image(r)?, t)?;
    let va = with(|d| write_descriptor(d, &words))?;
    out(handle, va)
}

pub(crate) unsafe fn cuTexObjectDestroy(handle: CUtexObject) -> CUresult {
    destroy_descriptor(handle, TEXTURE_MAGIC)
}

unsafe fn destroy_descriptor(handle: u64, magic: u64) -> CUresult {
    with(|d| {
        let p = d.ctx.cpu_ptr(handle, DESCRIPTOR_BYTES).map_err(|_| CUerror::INVALID_HANDLE)? as *const u64;
        if *p != magic {
            return Err(CUerror::INVALID_HANDLE);
        }
        d.sync_all()?;
        d.ctx.mem_free(handle).map_err(from_io)
    })
}

pub(crate) unsafe fn cuSurfObjectCreate(handle: *mut CUsurfObject, resource: *const CUDA_RESOURCE_DESC) -> CUresult {
    let r = resource.as_ref().ok_or(CUerror::INVALID_VALUE)?;
    if r.resType != CUresourcetype::CU_RESOURCE_TYPE_ARRAY {
        return Err(CUerror::INVALID_VALUE);
    }
    let a = array(r.res.array.hArray)?;
    let layered = a.flags & (CUDA_ARRAY3D_LAYERED | CUDA_ARRAY3D_CUBEMAP) != 0;
    let words = [
        SURFACE_MAGIC,
        a.va,
        a.texel_bytes() as u64,
        a.layers as u64,
        a.width as u64,
        a.height as u64,
        a.depth as u64,
        sample_format(a.format),
        a.channels as u64,
        0,
        0,
        1,
        layered as u64,
        0,
        0,
        0,
    ];
    let va = with(|d| write_descriptor(d, &words))?;
    out(handle, va)
}

pub(crate) unsafe fn cuSurfObjectDestroy(handle: CUsurfObject) -> CUresult {
    destroy_descriptor(handle, SURFACE_MAGIC)
}

// ---------------------------------------------------------------- copies to and from arrays

/// The CPU address of (x bytes, y, z) of a copy side that is an array.
pub(crate) unsafe fn array_side(d: &mut Driver, handle: CUarray, x: usize, y: usize, z: usize) -> Result<(*mut u8, usize), CUerror> {
    let a = array(handle)?;
    let p = d.ctx.cpu_ptr(a.va, a.bytes() as u64).map_err(|_| CUerror::INVALID_VALUE)?;
    Ok((p.add(a.offset(x, y, z)), a.row_pitch()))
}

pub(crate) unsafe fn cuMemcpyHtoA_v2(dst: CUarray, offset: usize, src: *const c_void, n: usize) -> CUresult {
    with(|d| {
        d.sync_all()?;
        let a = array(dst)?;
        if offset + n > a.bytes() {
            return Err(CUerror::INVALID_VALUE);
        }
        let (p, _) = array_side(d, dst, 0, 0, 0)?;
        std::ptr::copy(src as *const u8, p.add(offset), n);
        Ok(())
    })
}

pub(crate) unsafe fn cuMemcpyAtoH_v2(dst: *mut c_void, src: CUarray, offset: usize, n: usize) -> CUresult {
    with(|d| {
        d.sync_all()?;
        let a = array(src)?;
        if offset + n > a.bytes() {
            return Err(CUerror::INVALID_VALUE);
        }
        let (p, _) = array_side(d, src, 0, 0, 0)?;
        std::ptr::copy(p.add(offset), dst as *mut u8, n);
        Ok(())
    })
}

/// One side of a 3D copy: its CPU address and row and slice pitches.
unsafe fn side_3d(
    d: &mut Driver,
    kind: CUmemorytype,
    host: *const c_void,
    dev: CUdeviceptr,
    arr: CUarray,
    (x, y, z): (usize, usize, usize),
    pitch: usize,
    height: usize,
    extent: (usize, usize, usize),
) -> Result<(*mut u8, usize, usize), CUerror> {
    use CUmemorytype_enum as T;
    if kind == T::CU_MEMORYTYPE_ARRAY {
        let a = array(arr)?;
        let (p, row) = array_side(d, arr, x, y, z)?;
        return Ok((p, row, row * a.height));
    }
    let slice = pitch * height.max(1);
    let start = z * slice + y * pitch + x;
    let (w, h, depth) = extent;
    let span = if depth == 0 || h == 0 { 0 } else { (depth - 1) * slice + (h - 1) * pitch + w };
    let device = match kind {
        T::CU_MEMORYTYPE_HOST => false,
        T::CU_MEMORYTYPE_DEVICE => true,
        T::CU_MEMORYTYPE_UNIFIED => d.ctx.address_range(dev.0 as u64).is_some(),
        _ => return Err(CUerror::NOT_SUPPORTED),
    };
    let p = if device {
        d.ctx.cpu_ptr(dev.0 as u64 + start as u64, span as u64).map_err(|_| CUerror::INVALID_VALUE)?
    } else {
        let base = if kind == T::CU_MEMORYTYPE_UNIFIED { dev.0 as *const u8 } else { host as *const u8 };
        base.add(start) as *mut u8
    };
    Ok((p, pitch, slice))
}

pub(crate) unsafe fn memcpy_3d(c: *const CUDA_MEMCPY3D) -> CUresult {
    let c = c.as_ref().ok_or(CUerror::INVALID_VALUE)?;
    if c.srcLOD != 0 || c.dstLOD != 0 {
        return Err(CUerror::INVALID_VALUE);
    }
    let (w, h, depth) = (c.WidthInBytes, c.Height.max(1), c.Depth.max(1));
    with(|d| {
        d.sync_all()?;
        let (src, sp, ss) = side_3d(
            d,
            c.srcMemoryType,
            c.srcHost,
            c.srcDevice,
            c.srcArray,
            (c.srcXInBytes, c.srcY, c.srcZ),
            c.srcPitch,
            c.srcHeight,
            (w, h, depth),
        )?;
        let (dst, dp, ds) = side_3d(
            d,
            c.dstMemoryType,
            c.dstHost as *const c_void,
            c.dstDevice,
            c.dstArray,
            (c.dstXInBytes, c.dstY, c.dstZ),
            c.dstPitch,
            c.dstHeight,
            (w, h, depth),
        )?;
        for z in 0..depth {
            for y in 0..h {
                std::ptr::copy(src.add(z * ss + y * sp), dst.add(z * ds + y * dp), w);
            }
        }
        Ok(())
    })
}

pub(crate) unsafe fn cuMemcpy3D_v2(c: *const CUDA_MEMCPY3D) -> CUresult {
    memcpy_3d(c)
}

// Copies are CPU copies after the earlier work, so the asynchronous form is synchronous.
pub(crate) unsafe fn cuMemcpy3DAsync_v2(c: *const CUDA_MEMCPY3D, _s: CUstream) -> CUresult {
    memcpy_3d(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pitch_linear_texture_descriptor() {
        let mut r: CUDA_RESOURCE_DESC = unsafe { std::mem::zeroed() };
        r.resType = CUresourcetype::CU_RESOURCE_TYPE_PITCH2D;
        r.res.pitch2D = CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_4 {
            devPtr: CUdeviceptr_v2(0x10000 as *mut c_void),
            format: CUarray_format::CU_AD_FORMAT_UNSIGNED_INT8,
            numChannels: 4,
            width: 30,
            height: 20,
            pitchInBytes: 128,
        };
        let mut t: CUDA_TEXTURE_DESC = unsafe { std::mem::zeroed() };
        t.addressMode = [CUaddress_mode::CU_TR_ADDRESS_MODE_MIRROR; 3];
        t.filterMode = CUfilter_mode::CU_TR_FILTER_MODE_LINEAR;
        t.flags = CU_TRSF_NORMALIZED_COORDINATES;
        let w = texture_words(&unsafe { image(&r) }.unwrap(), &t).unwrap();
        // base, u8, 4 channels, linear, mirror, normalized, read normalized, 1 layer, 30 x 20 x 1, pitch 128
        assert_eq!(w[..12], [TEXTURE_MAGIC, 0x10000, 0, 4, 1, 3, 1, 1, 1, 30, 20, 1]);
        assert_eq!((w[12], w[13], w[14], w[15]), (1, 0, 128, 0));
        t.flags |= CU_TRSF_READ_AS_INTEGER;
        assert_eq!(texture_words(&unsafe { image(&r) }.unwrap(), &t).unwrap()[7], 0);
    }

    #[test]
    fn array_layout() {
        // a layered 4 x 3 array of 2 layers and 3 levels, float2 texels
        let a = Array {
            va: 0,
            format: CUarray_format::CU_AD_FORMAT_FLOAT,
            channels: 2,
            width: 4,
            height: 3,
            depth: 1,
            layers: 2,
            levels: 3,
            flags: CUDA_ARRAY3D_LAYERED,
            owned: false,
        };
        assert_eq!(a.texel_bytes(), 8);
        // levels 4x3, 2x1, 1x1, each with both layers
        assert_eq!((a.level_offset(1), a.level_offset(2), a.bytes()), (24 * 8, 28 * 8, 30 * 8));
        // layer 1, row 2, byte 8 of level 0
        assert_eq!(a.offset(8, 2, 1), (3 + 2) * 32 + 8);
    }
}

// ---------------------------------------------------------------- texture references (legacy API)

/// A texture reference (cuModuleGetTexRef): a module's `.texref` variable, which holds the handle of a texture
/// object that the driver makes from the reference's settings again at each change.
pub(crate) struct TexRef {
    module: usize,
    name: String,
    resource: Option<CUDA_RESOURCE_DESC>,
    desc: CUDA_TEXTURE_DESC,
    format: (CUarray_format, u32),
    object: u64,
}

static TEXREFS: std::sync::Mutex<Vec<(usize, String, usize)>> = std::sync::Mutex::new(Vec::new());

unsafe fn texref<'a>(h: CUtexref) -> Result<&'a mut TexRef, CUerror> {
    (h as *mut TexRef).as_mut().ok_or(CUerror::INVALID_HANDLE)
}

pub(crate) unsafe fn cuModuleGetTexRef(p: *mut CUtexref, hmod: CUmodule, name: *const std::ffi::c_char) -> CUresult {
    let name = std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned();
    let module = hmod.0 as usize;
    with(|d| {
        let m = d.modules.get(&module).ok_or(CUerror::INVALID_HANDLE)?;
        if !m.module.global_symbols.contains_key(&name) {
            return Err(CUerror::NOT_FOUND);
        }
        Ok(())
    })?;
    let mut refs = TEXREFS.lock().unwrap_or_else(|e| e.into_inner());
    let h = match refs.iter().find(|(m, n, _)| *m == module && *n == name) {
        Some(&(_, _, h)) => h,
        None => {
            let mut desc: CUDA_TEXTURE_DESC = std::mem::zeroed();
            desc.addressMode = [CUaddress_mode::CU_TR_ADDRESS_MODE_CLAMP; 3];
            let r = Box::new(TexRef {
                module,
                name: name.clone(),
                resource: None,
                desc,
                format: (CUarray_format::CU_AD_FORMAT_FLOAT, 1),
                object: 0,
            });
            let h = Box::into_raw(r) as usize;
            refs.push((module, name, h));
            h
        }
    };
    out(p, h as CUtexref)
}

/// Makes the reference's texture object again and points the module's variable at it.
unsafe fn rebind(r: &mut TexRef) -> CUresult {
    let Some(resource) = r.resource else { return Ok(()) };
    let words = texture_words(&image(&resource)?, &r.desc)?;
    with(|d| {
        let va = write_descriptor(d, &words)?;
        let old = std::mem::replace(&mut r.object, va);
        let m = d.modules.get_mut(&r.module).ok_or(CUerror::INVALID_HANDLE)?;
        if !m.write_global(&r.name, &va.to_le_bytes()) {
            return Err(CUerror::NOT_FOUND);
        }
        if old != 0 {
            d.sync_all()?;
            d.ctx.mem_free(old).map_err(from_io)?;
        }
        Ok(())
    })
}

pub(crate) unsafe fn cuTexRefSetArray(h: CUtexref, a: CUarray, _flags: std::ffi::c_uint) -> CUresult {
    let r = texref(h)?;
    let mut res: CUDA_RESOURCE_DESC = std::mem::zeroed();
    res.resType = CUresourcetype::CU_RESOURCE_TYPE_ARRAY;
    res.res.array.hArray = a;
    r.resource = Some(res);
    rebind(r)
}

pub(crate) unsafe fn cuTexRefSetMipmappedArray(h: CUtexref, a: CUmipmappedArray, _flags: std::ffi::c_uint) -> CUresult {
    let r = texref(h)?;
    let mut res: CUDA_RESOURCE_DESC = std::mem::zeroed();
    res.resType = CUresourcetype::CU_RESOURCE_TYPE_MIPMAPPED_ARRAY;
    res.res.mipmap.hMipmappedArray = a;
    r.resource = Some(res);
    rebind(r)
}

pub(crate) unsafe fn cuTexRefSetAddress_v2(offset: *mut usize, h: CUtexref, dptr: CUdeviceptr, bytes: usize) -> CUresult {
    let r = texref(h)?;
    let mut res: CUDA_RESOURCE_DESC = std::mem::zeroed();
    res.resType = CUresourcetype::CU_RESOURCE_TYPE_LINEAR;
    res.res.linear.devPtr = dptr;
    res.res.linear.format = r.format.0;
    res.res.linear.numChannels = r.format.1;
    res.res.linear.sizeInBytes = bytes;
    r.resource = Some(res);
    if !offset.is_null() {
        *offset = 0;
    }
    rebind(r)
}

pub(crate) unsafe fn cuTexRefSetAddress2D_v3(
    h: CUtexref,
    desc: *const CUDA_ARRAY_DESCRIPTOR,
    dptr: CUdeviceptr,
    pitch: usize,
) -> CUresult {
    let r = texref(h)?;
    let a = desc.as_ref().ok_or(CUerror::INVALID_VALUE)?;
    let mut res: CUDA_RESOURCE_DESC = std::mem::zeroed();
    res.resType = CUresourcetype::CU_RESOURCE_TYPE_PITCH2D;
    res.res.pitch2D.devPtr = dptr;
    res.res.pitch2D.format = a.Format;
    res.res.pitch2D.numChannels = a.NumChannels;
    res.res.pitch2D.width = a.Width;
    res.res.pitch2D.height = a.Height;
    res.res.pitch2D.pitchInBytes = pitch;
    r.format = (a.Format, a.NumChannels);
    r.resource = Some(res);
    rebind(r)
}

pub(crate) unsafe fn cuTexRefSetFormat(h: CUtexref, fmt: CUarray_format, components: std::ffi::c_int) -> CUresult {
    let r = texref(h)?;
    r.format = (fmt, components.max(1) as u32);
    if let Some(res) = r.resource.as_mut() {
        if res.resType == CUresourcetype::CU_RESOURCE_TYPE_LINEAR {
            res.res.linear.format = fmt;
            res.res.linear.numChannels = components.max(1) as u32;
        }
    }
    rebind(r)
}

pub(crate) unsafe fn cuTexRefSetAddressMode(h: CUtexref, dim: std::ffi::c_int, am: CUaddress_mode) -> CUresult {
    let r = texref(h)?;
    *r.desc.addressMode.get_mut(dim as usize).ok_or(CUerror::INVALID_VALUE)? = am;
    rebind(r)
}

pub(crate) unsafe fn cuTexRefSetFilterMode(h: CUtexref, fm: CUfilter_mode) -> CUresult {
    let r = texref(h)?;
    r.desc.filterMode = fm;
    rebind(r)
}

pub(crate) unsafe fn cuTexRefSetMipmapFilterMode(h: CUtexref, fm: CUfilter_mode) -> CUresult {
    let r = texref(h)?;
    r.desc.mipmapFilterMode = fm;
    rebind(r)
}

pub(crate) unsafe fn cuTexRefSetFlags(h: CUtexref, flags: std::ffi::c_uint) -> CUresult {
    let r = texref(h)?;
    r.desc.flags = flags;
    rebind(r)
}

pub(crate) unsafe fn cuTexRefGetAddress_v2(p: *mut CUdeviceptr, h: CUtexref) -> CUresult {
    let r = texref(h)?;
    match r.resource {
        Some(res) if res.resType == CUresourcetype::CU_RESOURCE_TYPE_LINEAR => out(p, res.res.linear.devPtr),
        Some(res) if res.resType == CUresourcetype::CU_RESOURCE_TYPE_PITCH2D => out(p, res.res.pitch2D.devPtr),
        _ => Err(CUerror::INVALID_VALUE),
    }
}
