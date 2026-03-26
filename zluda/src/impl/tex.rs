use crate::r#impl::hipfix;
use hip_runtime_sys::*;

pub(crate) unsafe fn object_create(
    p_tex_object: *mut hipTextureObject_t,
    p_res_desc: *const HIP_RESOURCE_DESC,
    p_tex_desc: *const HIP_TEXTURE_DESC,
    p_res_view_desc: *const HIP_RESOURCE_VIEW_DESC,
) -> hipError_t {
    hipTexObjectCreate(p_tex_object, p_res_desc, p_tex_desc, p_res_view_desc)
}

pub(crate) unsafe fn object_destroy(tex_object: hipTextureObject_t) -> hipError_t {
    hipDestroyTextureObject(tex_object)
}

pub(crate) unsafe fn ref_set_array(
    texref: *mut textureReference,
    array: hipArray_t,
    flags: ::core::ffi::c_uint,
) -> hipError_t {
    hipTexRefSetArray(texref, array, flags)
}

pub(crate) unsafe fn ref_set_flags(
    raw_texref: *mut textureReference,
    flags: ::core::ffi::c_uint,
) -> hipError_t {
    fn get_flags(texref: &textureReference) -> (u32, i32, i32) {
        (texref.readMode.0, texref.normalized, texref.sRGB)
    }
    let texref = raw_texref.as_ref().ok_or(hipErrorCode_t::InvalidValue)?;
    let pre_flags = get_flags(texref);
    hipTexRefSetFlags(raw_texref, flags)?;
    let post_flags = get_flags(texref);
    if pre_flags != post_flags {
        hipfix::refresh_texref(raw_texref)?;
    }
    Ok(())
}

pub(crate) unsafe fn ref_set_filter_mode(
    raw_texref: *mut textureReference,
    filter_mode: hipTextureFilterMode,
) -> hipError_t {
    fn get_flags(texref: &textureReference) -> u32 {
        texref.filterMode.0
    }
    let texref = raw_texref.as_ref().ok_or(hipErrorCode_t::InvalidValue)?;
    let pre_flags = get_flags(texref);
    hipTexRefSetFilterMode(raw_texref, filter_mode)?;
    let post_flags = get_flags(texref);
    if pre_flags != post_flags {
        hipfix::refresh_texref(raw_texref)?;
    }
    Ok(())
}

pub(crate) unsafe fn ref_set_address_mode(
    raw_texref: *mut textureReference,
    dim: i32,
    address_mode: hipTextureAddressMode,
) -> hipError_t {
    fn get_flags(texref: &textureReference, dim: i32) -> u32 {
        texref.addressMode[dim as usize].0
    }
    if dim < 0 || dim > 2 {
        return Err(hipErrorCode_t::InvalidValue);
    }
    let texref = raw_texref.as_ref().ok_or(hipErrorCode_t::InvalidValue)?;
    let pre_flags = get_flags(texref, dim);
    hipTexRefSetAddressMode(raw_texref, dim, address_mode)?;
    let post_flags = get_flags(texref, dim);
    if pre_flags != post_flags {
        hipfix::refresh_texref(raw_texref)?;
    }
    Ok(())
}

pub(crate) unsafe fn ref_set_format(
    raw_texref: *mut textureReference,
    format: hipArray_Format,
    num_components: ::core::ffi::c_int,
) -> hipError_t {
    fn get_flags(texref: &textureReference) -> (u32, u32) {
        (texref.format.0, texref.numChannels as u32)
    }
    let texref = raw_texref.as_ref().ok_or(hipErrorCode_t::InvalidValue)?;
    let pre_flags = get_flags(texref);
    hipTexRefSetFormat(raw_texref, format, num_components)?;
    let post_flags = get_flags(texref);
    if pre_flags != post_flags {
        hipfix::refresh_texref(raw_texref)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::tests::CudaApi;
    use cuda_macros::test_cuda;
    use cuda_types::cuda::*;
    use std::ffi::c_void;

    const TEX_READ_1D_INT_PTX: &str = concat!(include_str!("test_ptx/tex_read_1d_int.ptx"), "\0");
    const TEX_READ_1D_FLOAT_PTX: &str =
        concat!(include_str!("test_ptx/tex_read_1d_float.ptx"), "\0");
    const TEX_READ_2D_INT_PTX: &str = concat!(include_str!("test_ptx/tex_read_2d_int.ptx"), "\0");
    const TEX_READ_2D_FLOAT_PTX: &str =
        concat!(include_str!("test_ptx/tex_read_2d_float.ptx"), "\0");
    const TEX_READ_3D_INT_PTX: &str = concat!(include_str!("test_ptx/tex_read_3d_int.ptx"), "\0");
    const TEX_READ_3D_FLOAT_PTX: &str =
        concat!(include_str!("test_ptx/tex_read_3d_float.ptx"), "\0");

    const TEX_READ_1D_INT_TEXOBJ_PTX: &str =
        concat!(include_str!("test_ptx/tex_read_1d_int_texobj.ptx"), "\0");
    const TEX_READ_1D_FLOAT_TEXOBJ_PTX: &str =
        concat!(include_str!("test_ptx/tex_read_1d_float_texobj.ptx"), "\0");
    const TEX_READ_2D_INT_TEXOBJ_PTX: &str =
        concat!(include_str!("test_ptx/tex_read_2d_int_texobj.ptx"), "\0");
    const TEX_READ_2D_FLOAT_TEXOBJ_PTX: &str =
        concat!(include_str!("test_ptx/tex_read_2d_float_texobj.ptx"), "\0");
    const TEX_READ_3D_INT_TEXOBJ_PTX: &str =
        concat!(include_str!("test_ptx/tex_read_3d_int_texobj.ptx"), "\0");
    const TEX_READ_3D_FLOAT_TEXOBJ_PTX: &str =
        concat!(include_str!("test_ptx/tex_read_3d_float_texobj.ptx"), "\0");

    #[derive(Debug, Clone, Copy)]
    enum TexDim {
        One,
        Two,
        Three,
    }

    /// Returns the byte size of a single element for the given format.
    fn format_byte_size(fmt: CUarray_format) -> usize {
        match fmt {
            CUarray_format_enum::CU_AD_FORMAT_UNSIGNED_INT8
            | CUarray_format_enum::CU_AD_FORMAT_SIGNED_INT8 => 1,
            CUarray_format_enum::CU_AD_FORMAT_UNSIGNED_INT16
            | CUarray_format_enum::CU_AD_FORMAT_SIGNED_INT16
            | CUarray_format_enum::CU_AD_FORMAT_HALF => 2,
            CUarray_format_enum::CU_AD_FORMAT_UNSIGNED_INT32
            | CUarray_format_enum::CU_AD_FORMAT_SIGNED_INT32
            | CUarray_format_enum::CU_AD_FORMAT_FLOAT => 4,
            _ => panic!("unsupported format"),
        }
    }

    /// Returns true if the format is a floating-point type (HALF or FLOAT).
    fn format_is_float(fmt: CUarray_format) -> bool {
        matches!(
            fmt,
            CUarray_format_enum::CU_AD_FORMAT_HALF | CUarray_format_enum::CU_AD_FORMAT_FLOAT
        )
    }

    /// Write a test value into a host buffer at the given byte offset.
    /// Returns the expected 4xi32 or 4xf32 result (as [u32; 4] bit pattern).
    fn write_test_pixel(
        host_buf: &mut [u8],
        offset: usize,
        fmt: CUarray_format,
        num_channels: u32,
    ) -> [u32; 4] {
        let elem_size = format_byte_size(fmt);

        // Channel test values - small enough for all integer formats
        let channel_values: [i32; 4] = [42, 17, 99, 7];
        let mut expected = [0u32; 4];

        for ch in 0..num_channels as usize {
            let val = channel_values[ch];
            let byte_off = offset + ch * elem_size;
            match fmt {
                CUarray_format_enum::CU_AD_FORMAT_UNSIGNED_INT8 => {
                    host_buf[byte_off] = val as u8;
                    expected[ch] = val as u32;
                }
                CUarray_format_enum::CU_AD_FORMAT_SIGNED_INT8 => {
                    host_buf[byte_off] = val as u8;
                    expected[ch] = val as u32;
                }
                CUarray_format_enum::CU_AD_FORMAT_UNSIGNED_INT16 => {
                    let bytes = (val as u16).to_ne_bytes();
                    host_buf[byte_off..byte_off + 2].copy_from_slice(&bytes);
                    expected[ch] = val as u32;
                }
                CUarray_format_enum::CU_AD_FORMAT_SIGNED_INT16 => {
                    let bytes = (val as i16).to_ne_bytes();
                    host_buf[byte_off..byte_off + 2].copy_from_slice(&bytes);
                    expected[ch] = val as u32;
                }
                CUarray_format_enum::CU_AD_FORMAT_UNSIGNED_INT32 => {
                    let bytes = (val as u32).to_ne_bytes();
                    host_buf[byte_off..byte_off + 4].copy_from_slice(&bytes);
                    expected[ch] = val as u32;
                }
                CUarray_format_enum::CU_AD_FORMAT_SIGNED_INT32 => {
                    let bytes = val.to_ne_bytes();
                    host_buf[byte_off..byte_off + 4].copy_from_slice(&bytes);
                    expected[ch] = val as u32;
                }
                CUarray_format_enum::CU_AD_FORMAT_HALF => {
                    let h = half::f16::from_f32(val as f32);
                    let bytes = h.to_ne_bytes();
                    host_buf[byte_off..byte_off + 2].copy_from_slice(&bytes);
                    expected[ch] = (val as f32).to_bits();
                }
                CUarray_format_enum::CU_AD_FORMAT_FLOAT => {
                    let bytes = (val as f32).to_ne_bytes();
                    host_buf[byte_off..byte_off + 4].copy_from_slice(&bytes);
                    expected[ch] = (val as f32).to_bits();
                }
                _ => panic!("unsupported format"),
            }
        }
        expected
    }

    /// Core test logic: create array, upload data, bind to texref, launch
    /// kernel, read back and verify.
    unsafe fn texref_read_test(
        api: &impl CudaApi,
        normalized: bool,
        dim: TexDim,
        fmt: CUarray_format,
        num_channels: u32,
        width: usize,
        height: usize,
        depth: usize,
    ) {
        let elem_size = format_byte_size(fmt);
        let pixel_size = elem_size * num_channels as usize;
        let row_bytes = width * pixel_size;

        let (effective_height, effective_depth) = match dim {
            TexDim::One => (1, 1),
            TexDim::Two => (height, 1),
            TexDim::Three => (height, depth),
        };
        let total_bytes = row_bytes * effective_height * effective_depth;

        // Pick a deterministic "random" pixel based on dimensions
        let px = (width / 2).min(width - 1);
        let py = (effective_height / 2).min(effective_height - 1);
        let pz = (effective_depth / 2).min(effective_depth - 1);

        let pixel_offset = pz * (row_bytes * effective_height) + py * row_bytes + px * pixel_size;

        // Create host buffer filled with zeros, then set test pixel
        let mut host_buf = vec![0u8; total_bytes];
        let expected = write_test_pixel(&mut host_buf, pixel_offset, fmt, num_channels);

        // Create CUDA array and upload data
        let mut array: CUarray = std::mem::zeroed();
        match dim {
            TexDim::One => {
                let desc = CUDA_ARRAY_DESCRIPTOR {
                    Width: width,
                    Height: 0,
                    Format: fmt,
                    NumChannels: num_channels,
                };
                api.cuArrayCreate_v2(&mut array, &desc);
                api.cuMemcpyHtoA_v2(array, 0, host_buf.as_ptr() as *const c_void, total_bytes);
            }
            TexDim::Two => {
                let desc = CUDA_ARRAY_DESCRIPTOR {
                    Width: width,
                    Height: effective_height,
                    Format: fmt,
                    NumChannels: num_channels,
                };
                api.cuArrayCreate_v2(&mut array, &desc);
                let copy_params = CUDA_MEMCPY2D {
                    srcXInBytes: 0,
                    srcY: 0,
                    srcMemoryType: CUmemorytype::CU_MEMORYTYPE_HOST,
                    srcHost: host_buf.as_ptr() as *const c_void,
                    srcDevice: CUdeviceptr_v2(std::ptr::null_mut()),
                    srcArray: std::ptr::null_mut(),
                    srcPitch: row_bytes,
                    dstXInBytes: 0,
                    dstY: 0,
                    dstMemoryType: CUmemorytype::CU_MEMORYTYPE_ARRAY,
                    dstHost: std::ptr::null_mut(),
                    dstDevice: CUdeviceptr_v2(std::ptr::null_mut()),
                    dstArray: array,
                    dstPitch: 0,
                    WidthInBytes: row_bytes,
                    Height: effective_height,
                };
                api.cuMemcpy2D_v2(&copy_params);
            }
            TexDim::Three => {
                let desc = CUDA_ARRAY3D_DESCRIPTOR {
                    Width: width,
                    Height: effective_height,
                    Depth: effective_depth,
                    Format: fmt,
                    NumChannels: num_channels,
                    Flags: 0,
                };
                api.cuArray3DCreate_v2(&mut array, &desc);
                let copy_params = CUDA_MEMCPY3D {
                    srcXInBytes: 0,
                    srcY: 0,
                    srcZ: 0,
                    srcLOD: 0,
                    srcMemoryType: CUmemorytype::CU_MEMORYTYPE_HOST,
                    srcHost: host_buf.as_ptr() as *const c_void,
                    srcDevice: CUdeviceptr_v2(std::ptr::null_mut()),
                    srcArray: std::ptr::null_mut(),
                    reserved0: std::ptr::null_mut(),
                    srcPitch: row_bytes,
                    srcHeight: effective_height,
                    dstXInBytes: 0,
                    dstY: 0,
                    dstZ: 0,
                    dstLOD: 0,
                    dstMemoryType: CUmemorytype::CU_MEMORYTYPE_ARRAY,
                    dstHost: std::ptr::null_mut(),
                    dstDevice: CUdeviceptr_v2(std::ptr::null_mut()),
                    dstArray: array,
                    reserved1: std::ptr::null_mut(),
                    dstPitch: 0,
                    dstHeight: 0,
                    WidthInBytes: row_bytes,
                    Height: effective_height,
                    Depth: effective_depth,
                };
                api.cuMemcpy3D_v2(&copy_params);
            }
        }

        // Load PTX module and get texref + kernel function
        let is_float = format_is_float(fmt);
        let (ptx, kernel_name): (&str, &std::ffi::CStr) = match (dim, is_float) {
            (TexDim::One, false) => (TEX_READ_1D_INT_PTX, c"tex_read_1d_int"),
            (TexDim::One, true) => (TEX_READ_1D_FLOAT_PTX, c"tex_read_1d_float"),
            (TexDim::Two, false) => (TEX_READ_2D_INT_PTX, c"tex_read_2d_int"),
            (TexDim::Two, true) => (TEX_READ_2D_FLOAT_PTX, c"tex_read_2d_float"),
            (TexDim::Three, false) => (TEX_READ_3D_INT_PTX, c"tex_read_3d_int"),
            (TexDim::Three, true) => (TEX_READ_3D_FLOAT_PTX, c"tex_read_3d_float"),
        };

        let mut module = std::mem::zeroed();
        api.cuModuleLoadData(&mut module, ptx.as_ptr() as *const c_void);

        let mut func = std::mem::zeroed();
        api.cuModuleGetFunction(&mut func, module, kernel_name.as_ptr());

        let mut texref = std::mem::zeroed();
        api.cuModuleGetTexRef(&mut texref, module, c"tex".as_ptr());

        // Bind array to texref
        api.cuTexRefSetArray(texref, array, CU_TRSA_OVERRIDE_FORMAT);
        api.cuTexRefSetFormat(texref, fmt, num_channels as i32);
        api.cuTexRefSetFilterMode(texref, CUfilter_mode_enum::CU_TR_FILTER_MODE_POINT);
        for d in 0..3 {
            api.cuTexRefSetAddressMode(texref, d, CUaddress_mode_enum::CU_TR_ADDRESS_MODE_CLAMP);
        }
        let flags = match (!is_float, normalized) {
            (true, true) => CU_TRSF_READ_AS_INTEGER | CU_TRSF_NORMALIZED_COORDINATES,
            (true, false) => CU_TRSF_READ_AS_INTEGER,
            (false, true) => CU_TRSF_NORMALIZED_COORDINATES,
            (false, false) => 0,
        };
        api.cuTexRefSetFlags(texref, flags);

        // Allocate output buffer on device (4 x f32 or 4 x s32 = 16 bytes)
        let mut d_output = std::mem::zeroed();
        api.cuMemAlloc_v2(&mut d_output, 16);

        // Texture coordinates for point sampling: pixel center = (p + 0.5)
        let mut coord_x: f32 = px as f32 + 0.5;
        if normalized {
            coord_x /= width as f32;
        }
        let mut coord_y: f32 = py as f32 + 0.5;
        if normalized {
            coord_y /= height as f32;
        }
        let mut coord_z: f32 = pz as f32 + 0.5;
        if normalized {
            coord_z /= depth as f32;
        }
        let mut params: [*mut c_void; 4] = [
            &d_output as *const _ as *mut _,
            &coord_x as *const _ as *mut _,
            &coord_y as *const _ as *mut _,
            &coord_z as *const _ as *mut _,
        ];
        api.cuLaunchKernel(
            func,
            1,
            1,
            1,
            1,
            1,
            1,
            0,
            CUstream(std::ptr::null_mut()),
            params.as_mut_ptr(),
            std::ptr::null_mut(),
        );
        api.cuStreamSynchronize(CUstream(std::ptr::null_mut()));

        // Read back result
        let mut result = [0u32; 4];
        api.cuMemcpyDtoH_v2(result.as_mut_ptr() as *mut c_void, d_output, 16);

        // Verify
        let fmt_name = format!("{:?}", fmt);
        let dim_name = match dim {
            TexDim::One => "1d",
            TexDim::Two => "2d",
            TexDim::Three => "3d",
        };
        assert_eq!(
            result[..num_channels as usize],
            expected[..num_channels as usize],
            "texref mismatch for dim={dim_name}, format={fmt_name}, channels={num_channels}, \
             size={width}x{effective_height}x{effective_depth}, pixel=({px},{py},{pz}), normalized={normalized}, is_float={is_float}"
        );

        // Cleanup
        api.cuMemFree_v2(d_output);
        api.cuModuleUnload(module);
        api.cuArrayDestroy(array);
    }

    #[test_cuda]
    unsafe fn texref_formats_channels_dimensions(api: impl CudaApi) {
        api.cuInit(0);
        let mut ctx = std::mem::zeroed();
        api.cuCtxCreate_v2(&mut ctx, 0, 0);

        let formats = [
            CUarray_format_enum::CU_AD_FORMAT_UNSIGNED_INT8,
            CUarray_format_enum::CU_AD_FORMAT_UNSIGNED_INT16,
            CUarray_format_enum::CU_AD_FORMAT_UNSIGNED_INT32,
            CUarray_format_enum::CU_AD_FORMAT_SIGNED_INT8,
            CUarray_format_enum::CU_AD_FORMAT_SIGNED_INT16,
            CUarray_format_enum::CU_AD_FORMAT_SIGNED_INT32,
            CUarray_format_enum::CU_AD_FORMAT_HALF,
            CUarray_format_enum::CU_AD_FORMAT_FLOAT,
        ];
        let channel_counts: [u32; 3] = [1, 2, 4];

        // 1D tests: vary width only
        let widths_1d: [usize; 4] = [1, 4, 16, 64];
        for &fmt in &formats {
            for &num_ch in &channel_counts {
                for &w in &widths_1d {
                    for normalized in [false, true] {
                        texref_read_test(&api, normalized, TexDim::One, fmt, num_ch, w, 1, 1);
                    }
                }
            }
        }

        // 2D tests: vary width x height
        let dimensions_2d: [(usize, usize); 4] = [(1, 1), (4, 4), (16, 16), (64, 37)];
        for &fmt in &formats {
            for &num_ch in &channel_counts {
                for &(w, h) in &dimensions_2d {
                    for normalized in [false, true] {
                        texref_read_test(&api, normalized, TexDim::Two, fmt, num_ch, w, h, 1);
                    }
                }
            }
        }

        // 3D tests: vary width x height x depth
        let dimensions_3d: [(usize, usize, usize); 4] =
            [(1, 1, 1), (4, 4, 4), (8, 8, 8), (16, 13, 7)];
        for &fmt in &formats {
            for &num_ch in &channel_counts {
                for &(w, h, d) in &dimensions_3d {
                    for normalized in [false, true] {
                        texref_read_test(&api, normalized, TexDim::Three, fmt, num_ch, w, h, d);
                    }
                }
            }
        }

        api.cuCtxDestroy_v2(ctx);
    }

    /// Core test logic: create array, upload data, create texobj, launch
    /// kernel, read back and verify.
    unsafe fn texobj_read_test(
        api: &impl CudaApi,
        normalized: bool,
        dim: TexDim,
        fmt: CUarray_format,
        num_channels: u32,
        width: usize,
        height: usize,
        depth: usize,
    ) {
        let elem_size = format_byte_size(fmt);
        let pixel_size = elem_size * num_channels as usize;
        let row_bytes = width * pixel_size;

        let (effective_height, effective_depth) = match dim {
            TexDim::One => (1, 1),
            TexDim::Two => (height, 1),
            TexDim::Three => (height, depth),
        };
        let total_bytes = row_bytes * effective_height * effective_depth;

        let px = (width / 2).min(width - 1);
        let py = (effective_height / 2).min(effective_height - 1);
        let pz = (effective_depth / 2).min(effective_depth - 1);

        let pixel_offset = pz * (row_bytes * effective_height) + py * row_bytes + px * pixel_size;

        let mut host_buf = vec![0u8; total_bytes];
        let expected = write_test_pixel(&mut host_buf, pixel_offset, fmt, num_channels);

        // Create CUDA array and upload data
        let mut array: CUarray = std::mem::zeroed();
        match dim {
            TexDim::One => {
                let desc = CUDA_ARRAY_DESCRIPTOR {
                    Width: width,
                    Height: 0,
                    Format: fmt,
                    NumChannels: num_channels,
                };
                api.cuArrayCreate_v2(&mut array, &desc);
                api.cuMemcpyHtoA_v2(array, 0, host_buf.as_ptr() as *const c_void, total_bytes);
            }
            TexDim::Two => {
                let desc = CUDA_ARRAY_DESCRIPTOR {
                    Width: width,
                    Height: effective_height,
                    Format: fmt,
                    NumChannels: num_channels,
                };
                api.cuArrayCreate_v2(&mut array, &desc);
                let copy_params = CUDA_MEMCPY2D {
                    srcXInBytes: 0,
                    srcY: 0,
                    srcMemoryType: CUmemorytype::CU_MEMORYTYPE_HOST,
                    srcHost: host_buf.as_ptr() as *const c_void,
                    srcDevice: CUdeviceptr_v2(std::ptr::null_mut()),
                    srcArray: std::ptr::null_mut(),
                    srcPitch: row_bytes,
                    dstXInBytes: 0,
                    dstY: 0,
                    dstMemoryType: CUmemorytype::CU_MEMORYTYPE_ARRAY,
                    dstHost: std::ptr::null_mut(),
                    dstDevice: CUdeviceptr_v2(std::ptr::null_mut()),
                    dstArray: array,
                    dstPitch: 0,
                    WidthInBytes: row_bytes,
                    Height: effective_height,
                };
                api.cuMemcpy2D_v2(&copy_params);
            }
            TexDim::Three => {
                let desc = CUDA_ARRAY3D_DESCRIPTOR {
                    Width: width,
                    Height: effective_height,
                    Depth: effective_depth,
                    Format: fmt,
                    NumChannels: num_channels,
                    Flags: 0,
                };
                api.cuArray3DCreate_v2(&mut array, &desc);
                let copy_params = CUDA_MEMCPY3D {
                    srcXInBytes: 0,
                    srcY: 0,
                    srcZ: 0,
                    srcLOD: 0,
                    srcMemoryType: CUmemorytype::CU_MEMORYTYPE_HOST,
                    srcHost: host_buf.as_ptr() as *const c_void,
                    srcDevice: CUdeviceptr_v2(std::ptr::null_mut()),
                    srcArray: std::ptr::null_mut(),
                    reserved0: std::ptr::null_mut(),
                    srcPitch: row_bytes,
                    srcHeight: effective_height,
                    dstXInBytes: 0,
                    dstY: 0,
                    dstZ: 0,
                    dstLOD: 0,
                    dstMemoryType: CUmemorytype::CU_MEMORYTYPE_ARRAY,
                    dstHost: std::ptr::null_mut(),
                    dstDevice: CUdeviceptr_v2(std::ptr::null_mut()),
                    dstArray: array,
                    reserved1: std::ptr::null_mut(),
                    dstPitch: 0,
                    dstHeight: 0,
                    WidthInBytes: row_bytes,
                    Height: effective_height,
                    Depth: effective_depth,
                };
                api.cuMemcpy3D_v2(&copy_params);
            }
        }

        // Create texture object
        let res_desc = CUDA_RESOURCE_DESC {
            resType: CUresourcetype_enum::CU_RESOURCE_TYPE_ARRAY,
            res: CUDA_RESOURCE_DESC_st__bindgen_ty_1 {
                array: CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_1 { hArray: array },
            },
            flags: 0,
        };
        let is_float = format_is_float(fmt);
        let flags = match (!is_float, normalized) {
            (true, true) => CU_TRSF_READ_AS_INTEGER | CU_TRSF_NORMALIZED_COORDINATES,
            (true, false) => CU_TRSF_READ_AS_INTEGER,
            (false, true) => CU_TRSF_NORMALIZED_COORDINATES,
            (false, false) => 0,
        };
        let tex_desc = CUDA_TEXTURE_DESC {
            addressMode: [CUaddress_mode_enum::CU_TR_ADDRESS_MODE_CLAMP; 3],
            filterMode: CUfilter_mode_enum::CU_TR_FILTER_MODE_POINT,
            flags,
            maxAnisotropy: 0,
            mipmapFilterMode: CUfilter_mode_enum::CU_TR_FILTER_MODE_POINT,
            mipmapLevelBias: 0.0,
            minMipmapLevelClamp: 0.0,
            maxMipmapLevelClamp: 0.0,
            borderColor: [0.0; 4],
            reserved: [0; 12],
        };
        let mut texobj: CUtexObject = std::mem::zeroed();
        api.cuTexObjectCreate(&mut texobj, &res_desc, &tex_desc, std::ptr::null());

        // Load PTX module and get kernel function
        let (ptx, kernel_name): (&str, &std::ffi::CStr) = match (dim, is_float) {
            (TexDim::One, false) => (TEX_READ_1D_INT_TEXOBJ_PTX, c"tex_read_1d_int_texobj"),
            (TexDim::One, true) => (TEX_READ_1D_FLOAT_TEXOBJ_PTX, c"tex_read_1d_float_texobj"),
            (TexDim::Two, false) => (TEX_READ_2D_INT_TEXOBJ_PTX, c"tex_read_2d_int_texobj"),
            (TexDim::Two, true) => (TEX_READ_2D_FLOAT_TEXOBJ_PTX, c"tex_read_2d_float_texobj"),
            (TexDim::Three, false) => (TEX_READ_3D_INT_TEXOBJ_PTX, c"tex_read_3d_int_texobj"),
            (TexDim::Three, true) => (TEX_READ_3D_FLOAT_TEXOBJ_PTX, c"tex_read_3d_float_texobj"),
        };

        let mut module = std::mem::zeroed();
        api.cuModuleLoadData(&mut module, ptx.as_ptr() as *const c_void);

        let mut func = std::mem::zeroed();
        api.cuModuleGetFunction(&mut func, module, kernel_name.as_ptr());

        // Allocate output buffer on device (4 x f32 or 4 x s32 = 16 bytes)
        let mut d_output = std::mem::zeroed();
        api.cuMemAlloc_v2(&mut d_output, 16);

        // Texture coordinates for point sampling: pixel center = (p + 0.5)
        let mut coord_x: f32 = px as f32 + 0.5;
        if normalized {
            coord_x /= width as f32;
        }
        let mut coord_y: f32 = py as f32 + 0.5;
        if normalized {
            coord_y /= height as f32;
        }
        let mut coord_z: f32 = pz as f32 + 0.5;
        if normalized {
            coord_z /= depth as f32;
        }
        let mut params: [*mut c_void; 5] = [
            &d_output as *const _ as *mut _,
            &texobj as *const _ as *mut _,
            &coord_x as *const _ as *mut _,
            &coord_y as *const _ as *mut _,
            &coord_z as *const _ as *mut _,
        ];
        api.cuLaunchKernel(
            func,
            1,
            1,
            1,
            1,
            1,
            1,
            0,
            CUstream(std::ptr::null_mut()),
            params.as_mut_ptr(),
            std::ptr::null_mut(),
        );
        api.cuStreamSynchronize(CUstream(std::ptr::null_mut()));

        // Read back result
        let mut result = [0u32; 4];
        api.cuMemcpyDtoH_v2(result.as_mut_ptr() as *mut c_void, d_output, 16);

        // Verify
        let fmt_name = format!("{:?}", fmt);
        let dim_name = match dim {
            TexDim::One => "1d",
            TexDim::Two => "2d",
            TexDim::Three => "3d",
        };
        assert_eq!(
            result[..num_channels as usize],
            expected[..num_channels as usize],
            "texobj mismatch for dim={dim_name}, format={fmt_name}, channels={num_channels}, \
             size={width}x{effective_height}x{effective_depth}, pixel=({px},{py},{pz}), normalized={normalized}, is_float={is_float}"
        );

        // Cleanup
        api.cuMemFree_v2(d_output);
        api.cuTexObjectDestroy(texobj);
        api.cuModuleUnload(module);
        api.cuArrayDestroy(array);
    }

    #[test_cuda]
    unsafe fn texobj_formats_channels_dimensions(api: impl CudaApi) {
        api.cuInit(0);
        let mut ctx = std::mem::zeroed();
        api.cuCtxCreate_v2(&mut ctx, 0, 0);

        let formats = [
            CUarray_format_enum::CU_AD_FORMAT_UNSIGNED_INT8,
            CUarray_format_enum::CU_AD_FORMAT_UNSIGNED_INT16,
            CUarray_format_enum::CU_AD_FORMAT_UNSIGNED_INT32,
            CUarray_format_enum::CU_AD_FORMAT_SIGNED_INT8,
            CUarray_format_enum::CU_AD_FORMAT_SIGNED_INT16,
            CUarray_format_enum::CU_AD_FORMAT_SIGNED_INT32,
            CUarray_format_enum::CU_AD_FORMAT_HALF,
            CUarray_format_enum::CU_AD_FORMAT_FLOAT,
        ];
        let channel_counts: [u32; 3] = [1, 2, 4];

        // 1D tests
        let widths_1d: [usize; 4] = [1, 4, 16, 64];
        for &fmt in &formats {
            for &num_ch in &channel_counts {
                for &w in &widths_1d {
                    for normalized in [false, true] {
                        texobj_read_test(&api, normalized, TexDim::One, fmt, num_ch, w, 1, 1);
                    }
                }
            }
        }

        // 2D tests
        let dimensions_2d: [(usize, usize); 4] = [(1, 1), (4, 4), (16, 16), (64, 37)];
        for &fmt in &formats {
            for &num_ch in &channel_counts {
                for &(w, h) in &dimensions_2d {
                    for normalized in [false, true] {
                        texobj_read_test(&api, normalized, TexDim::Two, fmt, num_ch, w, h, 1);
                    }
                }
            }
        }

        // 3D tests
        let dimensions_3d: [(usize, usize, usize); 4] =
            [(1, 1, 1), (4, 4, 4), (8, 8, 8), (16, 13, 7)];
        for &fmt in &formats {
            for &num_ch in &channel_counts {
                for &(w, h, d) in &dimensions_3d {
                    for normalized in [false, true] {
                        texobj_read_test(&api, normalized, TexDim::Three, fmt, num_ch, w, h, d);
                    }
                }
            }
        }

        api.cuCtxDestroy_v2(ctx);
    }
}
