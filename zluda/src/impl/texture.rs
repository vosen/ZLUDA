#[cfg(test)]
mod tests {
    use crate::tests::CudaApi;
    use cuda_macros::test_cuda;
    use cuda_types::cuda::*;
    use core::f32;
    use std::mem;

    #[test_cuda]
    fn create_image(api: impl CudaApi) {
        api.cuInit(0);
        api.cuCtxCreate_v2(&mut unsafe { mem::zeroed() }, 0, 0);
        let mut texref = unsafe { mem::zeroed() };
        api.cuTexRefCreate(&mut texref);
        let mut array = unsafe { mem::zeroed() };
        assert_eq!(
            CUresult::ERROR_INVALID_VALUE,
            api.cuTexRefGetAddress_v2_unchecked(&mut array, texref)
        );
        let mut filter_mode = unsafe { mem::zeroed() };
        api.cuTexRefGetFilterMode(&mut filter_mode, texref);
        assert_eq!(filter_mode, CUfilter_mode::CU_TR_FILTER_MODE_POINT);
        let mut flags = u32::MAX;
        api.cuTexRefGetFlags(&mut flags, texref);
        assert_eq!(flags, 0);
        let mut format = unsafe { mem::zeroed() };
        let mut channels = 0;
        api.cuTexRefGetFormat(&mut format, &mut channels, texref);
        assert_eq!(format, CUarray_format::CU_AD_FORMAT_FLOAT);
        assert_eq!(channels, 1);
        let mut max_aniso = i32::MAX;
        api.cuTexRefGetMaxAnisotropy(&mut max_aniso, texref);
        assert_eq!(max_aniso, 0);
        let mut mipmap_filter = unsafe { mem::zeroed() };
        api.cuTexRefGetMipmapFilterMode(&mut mipmap_filter, texref);
        assert_eq!(mipmap_filter, CUfilter_mode::CU_TR_FILTER_MODE_POINT);
        let mut bias = f32::MAX;
        api.cuTexRefGetMipmapLevelBias(&mut bias, texref);
        assert_eq!(bias, 0.0);
        let mut min_clamp = f32::MAX;
        let mut max_clamp = f32::MAX;
        api.cuTexRefGetMipmapLevelClamp(&mut min_clamp, &mut max_clamp, texref);
        assert_eq!(min_clamp, 0.0);
        assert_eq!(max_clamp, 0.0);
        api.cuTexRefDestroy(texref);
    }
}
