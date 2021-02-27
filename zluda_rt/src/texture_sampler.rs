use crate::{
    buffer::{Buffer, BufferData},
    context::{self, Context, ContextData},
    hip, null_check, null_unwrap, MaybeWeakRefMut, OptixCell, OptixObjectData, TypeTag,
};
use hip_runtime_sys::*;
use optix_types::*;
use std::{
    mem, ptr,
    rc::{Rc, Weak},
};

pub(crate) type TextureSampler = *const OptixCell<TextureSamplerData>;

pub(crate) struct TextureSamplerData {
    pub(crate) context: Weak<OptixCell<ContextData>>,
    pub(crate) index: u32,
    pub(crate) hip_object: hipTextureObject_t,
    wrap_mode: [hipTextureAddressMode; 3],
    normalized_coordinates: bool,
    read_mode: hipTextureReadMode,
    perform_srgb_conversion: bool,
    max_anisotropy: f32,
    mip_level_count: u32,
    array_size: u32,
    minification: RTfiltermode,
    magnification: RTfiltermode,
    mipmapping: RTfiltermode,
    buffer: Option<Weak<OptixCell<BufferData>>>,
}

impl TextureSamplerData {
    fn new(weak_context: Weak<OptixCell<ContextData>>, context: &mut ContextData) -> Self {
        context.texture_counter += 1;
        let index = context.texture_counter;
        Self {
            context: weak_context,
            index,
            hip_object: ptr::null_mut(),
            wrap_mode: [
                hipTextureAddressMode::hipAddressModeWrap,
                hipTextureAddressMode::hipAddressModeWrap,
                hipTextureAddressMode::hipAddressModeWrap,
            ],
            normalized_coordinates: true,
            read_mode: hipTextureReadMode::hipReadModeNormalizedFloat,
            perform_srgb_conversion: false,
            max_anisotropy: 1.0,
            mip_level_count: 0,
            array_size: 0,
            buffer: None,
            minification: RTfiltermode::RT_FILTER_LINEAR,
            magnification: RTfiltermode::RT_FILTER_LINEAR,
            mipmapping: RTfiltermode::RT_FILTER_LINEAR,
        }
    }

    fn register(this: Rc<OptixCell<Self>>, context: &mut ContextData) {
        context.texture_samplers.insert(this);
    }

    unsafe fn create(context: Context) -> Result<TextureSampler, RTresult> {
        context::create_subobject(context, Self::new, Self::register)
    }

    pub(crate) unsafe fn create_underlying(&mut self) -> Result<(), RTresult> {
        let buffer = match self.buffer {
            Some(ref buffer) => buffer.upgrade().ok_or(RTresult::RT_ERROR_INVALID_CONTEXT)?,
            None => return Err(RTresult::RT_ERROR_INVALID_CONTEXT),
        };
        let buffer = buffer.borrow()?;
        // TODO: create mipmapped arrays when they work in HIP
        // Currently hipMipmappedArrayCreate(...) fails, because it's "unuspported"
        // (tested on Linux ROCm 5.4.3 on RX 6800 XT)
        let mut hip_texture = ptr::null_mut();
        let mut array = ptr::null_mut();
        let array_desc = HIP_ARRAY3D_DESCRIPTOR {
            Width: buffer.metadata.width as usize,
            Height: buffer.metadata.height as usize,
            Depth: buffer.metadata.depth(),
            Format: buffer.metadata.array_format()?,
            NumChannels: buffer.metadata.channels()?,
            Flags: 0,
        };
        hip! { hipArray3DCreate(&mut array, &array_desc), RT_ERROR_UNKNOWN };
        let copy_height = buffer.metadata.height.max(1);
        let copy_depth = buffer.metadata.depth().max(1);
        let params = hipMemcpy3DParms {
            srcArray: ptr::null_mut(),
            srcPos: hipPos { x: 0, y: 0, z: 0 },
            srcPtr: hipPitchedPtr {
                ptr: buffer.pointer_mip0().0,
                pitch: (buffer.metadata.width * buffer.metadata.element_size) as usize,
                xsize: buffer.metadata.width as usize,
                ysize: copy_height as usize,
            },
            dstArray: array,
            dstPos: hipPos { x: 0, y: 0, z: 0 },
            dstPtr: mem::zeroed::<hipPitchedPtr>(),
            extent: hipExtent {
                width: buffer.metadata.width as usize,
                height: copy_height as usize,
                depth: copy_depth,
            },
            kind: hipMemcpyKind::hipMemcpyDeviceToDevice,
        };
        hip! { hipMemcpy3D(&params), RT_ERROR_UNKNOWN };
        let resource_desc = hipResourceDesc {
            resType: hipResourceType::hipResourceTypeArray,
            res: hipResourceDesc__bindgen_ty_1 {
                array: hipResourceDesc__bindgen_ty_1__bindgen_ty_1 { array },
            },
        };
        let tex_desc = hipTextureDesc {
            addressMode: self.wrap_mode,
            filterMode: self.filter_mode(),
            readMode: self.read_mode,
            sRGB: self.perform_srgb_conversion as i32,
            borderColor: [0.0, 0.0, 0.0, 0.0],
            normalizedCoords: self.normalized_coordinates as i32,
            maxAnisotropy: 0, // other values not supported by HIP
            mipmapFilterMode: if self.mipmapping == RTfiltermode::RT_FILTER_LINEAR {
                hipTextureFilterMode::hipFilterModeLinear
            } else {
                hipTextureFilterMode::hipFilterModePoint
            },
            mipmapLevelBias: 0.0,     // other values not supported by HIP
            minMipmapLevelClamp: 0.0, // other values not supported by HIP
            maxMipmapLevelClamp: 0.0, // other values not supported by HIP
        };
        hip! { hipCreateTextureObject(&mut hip_texture, &resource_desc, &tex_desc, ptr::null()), RT_ERROR_UNKNOWN };
        self.hip_object = hip_texture;
        Ok(())
    }

    // TODO: this is as good as it gets under CUDA/HIP
    fn filter_mode(&self) -> hipTextureFilterMode {
        if self.magnification == RTfiltermode::RT_FILTER_LINEAR
            || self.minification == RTfiltermode::RT_FILTER_LINEAR
        {
            hipTextureFilterMode::hipFilterModeLinear
        } else {
            hipTextureFilterMode::hipFilterModePoint
        }
    }
}

impl OptixObjectData for TextureSamplerData {
    const TYPE: TypeTag = TypeTag::TextureSampler;

    fn deregister(&mut self, this: &Rc<OptixCell<Self>>) -> Result<(), RTresult> {
        if let Some(context) = self.context.upgrade() {
            let mut context = (*context).borrow_mut()?;
            context.texture_samplers.remove(this);
        }
        Ok(())
    }

    fn context<'a>(&'a mut self) -> crate::MaybeWeakRefMut<'a, ContextData> {
        MaybeWeakRefMut::Weak(&self.context)
    }
}

pub(crate) unsafe fn create(
    context: *const OptixCell<ContextData>,
    texturesampler: *mut TextureSampler,
) -> Result<(), RTresult> {
    null_check(context)?;
    null_check(texturesampler)?;
    *texturesampler = TextureSamplerData::create(context)?;
    Ok(())
}

pub(crate) unsafe fn set_wrap_mode(
    texturesampler: TextureSampler,
    dimension: u32,
    wrap_mode: RTwrapmode,
) -> Result<(), RTresult> {
    if dimension > 2 {
        return Err(RTresult::RT_ERROR_INVALID_VALUE);
    }
    let wrap_mode = to_hip_address_mode(wrap_mode)?;
    let texturesampler = null_unwrap(texturesampler)?;
    let mut texturesampler = texturesampler.borrow_mut()?;
    texturesampler.wrap_mode[dimension as usize] = wrap_mode;
    Ok(())
}

fn to_hip_address_mode(wrap_mode: RTwrapmode) -> Result<hipTextureAddressMode, RTresult> {
    Ok(match wrap_mode {
        RTwrapmode::RT_WRAP_REPEAT => hipTextureAddressMode::hipAddressModeWrap,
        RTwrapmode::RT_WRAP_CLAMP_TO_EDGE => hipTextureAddressMode::hipAddressModeClamp,
        RTwrapmode::RT_WRAP_MIRROR => hipTextureAddressMode::hipAddressModeMirror,
        RTwrapmode::RT_WRAP_CLAMP_TO_BORDER => hipTextureAddressMode::hipAddressModeBorder,
        _ => return Err(RTresult::RT_ERROR_INVALID_VALUE),
    })
}

pub(crate) unsafe fn set_indexing_mode(
    texturesampler: TextureSampler,
    index_mode: RTtextureindexmode,
) -> Result<(), RTresult> {
    let normalized_coordinates = match index_mode {
        RTtextureindexmode::RT_TEXTURE_INDEX_NORMALIZED_COORDINATES => true,
        RTtextureindexmode::RT_TEXTURE_INDEX_ARRAY_INDEX => false,
        _ => return Err(RTresult::RT_ERROR_INVALID_VALUE),
    };
    let texturesampler = null_unwrap(texturesampler)?;
    let mut texturesampler = texturesampler.borrow_mut()?;
    texturesampler.normalized_coordinates = normalized_coordinates;
    Ok(())
}

pub(crate) unsafe fn set_read_mode(
    texturesampler: TextureSampler,
    read_mode: RTtexturereadmode,
) -> Result<(), RTresult> {
    let (read_mode, perform_srgb_conversion) = to_hip_read_mode(read_mode)?;
    let texturesampler = null_unwrap(texturesampler)?;
    let mut texturesampler = texturesampler.borrow_mut()?;
    texturesampler.read_mode = read_mode;
    texturesampler.perform_srgb_conversion = perform_srgb_conversion;
    Ok(())
}

fn to_hip_read_mode(read_mode: RTtexturereadmode) -> Result<(hipTextureReadMode, bool), RTresult> {
    Ok(match read_mode {
        RTtexturereadmode::RT_TEXTURE_READ_ELEMENT_TYPE => {
            (hipTextureReadMode::hipReadModeElementType, false)
        }
        RTtexturereadmode::RT_TEXTURE_READ_NORMALIZED_FLOAT => {
            (hipTextureReadMode::hipReadModeNormalizedFloat, false)
        }
        RTtexturereadmode::RT_TEXTURE_READ_ELEMENT_TYPE_SRGB => {
            (hipTextureReadMode::hipReadModeElementType, true)
        }
        RTtexturereadmode::RT_TEXTURE_READ_NORMALIZED_FLOAT_SRGB => {
            (hipTextureReadMode::hipReadModeNormalizedFloat, true)
        }
        _ => return Err(RTresult::RT_ERROR_INVALID_VALUE),
    })
}

pub(crate) unsafe fn set_max_anisotropy(
    texturesampler: TextureSampler,
    max_anisotropy: f32,
) -> Result<(), RTresult> {
    let texturesampler = null_unwrap(texturesampler)?;
    let mut texturesampler = texturesampler.borrow_mut()?;
    texturesampler.max_anisotropy = max_anisotropy;
    Ok(())
}

pub(crate) unsafe fn set_mip_level_count(
    texturesampler: TextureSampler,
    mip_level_count: u32,
) -> Result<(), RTresult> {
    let texturesampler = null_unwrap(texturesampler)?;
    let mut texturesampler = texturesampler.borrow_mut()?;
    texturesampler.mip_level_count = mip_level_count;
    Ok(())
}

pub(crate) unsafe fn set_array_size(
    texturesampler: TextureSampler,
    array_size: u32,
) -> Result<(), RTresult> {
    let texturesampler = null_unwrap(texturesampler)?;
    let mut texturesampler = texturesampler.borrow_mut()?;
    texturesampler.array_size = array_size;
    Ok(())
}

pub(crate) unsafe fn set_buffer(
    texturesampler: TextureSampler,
    buffer: Buffer,
) -> Result<(), RTresult> {
    let buffer = if buffer == ptr::null() {
        None
    } else {
        Some(OptixCell::clone_weak(buffer))
    };
    let texturesampler = null_unwrap(texturesampler)?;
    let mut texturesampler = texturesampler.borrow_mut()?;
    texturesampler.buffer = buffer;
    Ok(())
}

pub(crate) unsafe fn set_filtering_modes(
    texturesampler: TextureSampler,
    minification: RTfiltermode,
    magnification: RTfiltermode,
    mipmapping: RTfiltermode,
) -> Result<(), RTresult> {
    let texturesampler = null_unwrap(texturesampler)?;
    let mut texturesampler = texturesampler.borrow_mut()?;
    texturesampler.minification = minification;
    texturesampler.magnification = magnification;
    texturesampler.mipmapping = mipmapping;
    Ok(())
}

pub(crate) unsafe fn get_id(
    texturesampler: TextureSampler,
    texture_id: *mut i32,
) -> Result<(), RTresult> {
    let texturesampler = null_unwrap(texturesampler)?;
    let texturesampler = texturesampler.borrow()?;
    *texture_id = texturesampler.index as i32;
    Ok(())
}

pub(crate) unsafe fn get_buffer(
    texturesampler: *const OptixCell<TextureSamplerData>,
    _deprecated0: u32,
    _deprecated1: u32,
    buffer: *mut *const OptixCell<BufferData>,
) -> Result<(), RTresult> {
    null_check(buffer)?;
    let texturesampler = null_unwrap(texturesampler)?;
    let texturesampler = texturesampler.borrow()?;
    match texturesampler.buffer {
        Some(ref weak_buffer) => {
            *buffer = Weak::as_ptr(weak_buffer);
            Ok(())
        }
        None => {
            *buffer = ptr::null_mut();
            Err(RTresult::RT_ERROR_INVALID_VALUE)
        }
    }
}

pub(crate) unsafe fn get_context(
    texturesampler: TextureSampler,
    context: *mut *const OptixCell<ContextData>,
) -> Result<(), RTresult> {
    let texturesampler = null_unwrap(texturesampler)?;
    let texturesampler = texturesampler.borrow()?;
    *context = texturesampler.context.as_ptr();
    Ok(())
}

pub(crate) fn destroy(
    _texturesampler: *const OptixCell<TextureSamplerData>,
) -> Result<(), RTresult> {
    // TODO: implement
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::optix_test;
    use crate::test_common::OptixFns;
    use optix_types::*;
    use std::{mem, ptr};

    optix_test!(default_texture_sampler);

    unsafe fn default_texture_sampler<Optix: OptixFns>(o: Optix) {
        let mut ctx = ptr::null_mut();
        o.rtContextCreate(&mut ctx);
        let mut sampler = ptr::null_mut();
        o.rtTextureSamplerCreate(ctx, &mut sampler);
        let mut array_size = mem::zeroed();
        o.rtTextureSamplerGetArraySize(sampler, &mut array_size);
        let mut buffer = mem::zeroed();
        o.rtTextureSamplerGetBuffer(sampler, 0, 0, &mut buffer);
        let mut minification = mem::zeroed();
        let mut magnification = mem::zeroed();
        let mut mipmapping = mem::zeroed();
        o.rtTextureSamplerGetFilteringModes(
            sampler,
            &mut minification,
            &mut magnification,
            &mut mipmapping,
        );
        let mut index_mode = mem::zeroed();
        o.rtTextureSamplerGetIndexingMode(sampler, &mut index_mode);
        let mut max_aniso = mem::zeroed();
        o.rtTextureSamplerGetMaxAnisotropy(sampler, &mut max_aniso);
        let mut mip_level = mem::zeroed();
        o.rtTextureSamplerGetMipLevelCount(sampler, &mut mip_level);
        let mut read_mode = mem::zeroed();
        o.rtTextureSamplerGetReadMode(sampler, &mut read_mode);
        let mut wrapmode0 = mem::zeroed();
        let mut wrapmode1 = mem::zeroed();
        let mut wrapmode2 = mem::zeroed();
        o.rtTextureSamplerGetWrapMode(sampler, 0, &mut wrapmode0);
        o.rtTextureSamplerGetWrapMode(sampler, 0, &mut wrapmode1);
        o.rtTextureSamplerGetWrapMode(sampler, 0, &mut wrapmode2);
        assert_eq!(array_size, 0);
        assert_eq!(buffer, ptr::null_mut());
        assert_eq!(minification, RTfiltermode::RT_FILTER_LINEAR);
        assert_eq!(magnification, RTfiltermode::RT_FILTER_LINEAR);
        assert_eq!(mipmapping, RTfiltermode::RT_FILTER_LINEAR);
        assert_eq!(
            index_mode,
            RTtextureindexmode::RT_TEXTURE_INDEX_NORMALIZED_COORDINATES
        );
        assert_eq!(max_aniso, 1.0);
        assert_eq!(mip_level, 0);
        assert_eq!(
            read_mode,
            RTtexturereadmode::RT_TEXTURE_READ_NORMALIZED_FLOAT
        );
        assert_eq!(wrapmode0, RTwrapmode::RT_WRAP_REPEAT);
        assert_eq!(wrapmode1, RTwrapmode::RT_WRAP_REPEAT);
        assert_eq!(wrapmode2, RTwrapmode::RT_WRAP_REPEAT);
    }
}
