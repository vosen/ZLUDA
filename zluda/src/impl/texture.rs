#[cfg(test)]
mod tests {
    use crate::tests::CudaApi;
    use core::f32;
    use cuda_macros::test_cuda;
    use cuda_types::cuda::*;
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

    #[test_cuda]
    fn create_image_from_array(api: impl CudaApi) {
        api.cuInit(0);
        api.cuCtxCreate_v2(&mut unsafe { mem::zeroed() }, 0, 0);
        let mut texref = unsafe { mem::zeroed() };
        api.cuTexRefCreate(&mut texref);
        let mut array = unsafe { mem::zeroed() };
        api.cuArrayCreate_v2(
            &mut array,
            &CUDA_ARRAY_DESCRIPTOR_v2 {
                Width: 1,
                Height: 1,
                Format: CUarray_format::CU_AD_FORMAT_HALF,
                NumChannels: 4,
            },
        );
        api.cuTexRefSetArray(texref, array, CU_TRSA_OVERRIDE_FORMAT);
        let mut format = unsafe { mem::zeroed() };
        let mut channels = 0;
        api.cuTexRefGetFormat(&mut format, &mut channels, texref);
        assert_eq!(format, CUarray_format::CU_AD_FORMAT_HALF);
        assert_eq!(channels, 4);
        api.cuTexRefDestroy(texref);
        api.cuArrayDestroy(array);
    }

    const MODULE_PTX: &'static str = "
.version 6.5
.target sm_60
.address_size 64

.global .texref texture;

.visible .entry run(
	.param .#COORD_TYPE# input_x,
	.param .#COORD_TYPE# input_y,
	.param .#COORD_TYPE# input_z,
	.param .u64 output
)
{
    .reg .#COORD_TYPE# x;
    .reg .#COORD_TYPE# y;
    .reg .#COORD_TYPE# z;
    ld.param.#COORD_TYPE# x, [input_x];
    ld.param.#COORD_TYPE# y, [input_y];
    ld.param.#COORD_TYPE# z, [input_z];
    .reg .#COORD_TYPE# _unused;
    mov.#COORD_TYPE# _unused, x;

    .reg .u64 	    out_addr;
    ld.param.u64 	out_addr, [output];

    .reg .#VALUE_TYPE# #VALUES#;
    
    tex.#DIMENSIONS#.v4.#VALUE_TYPE#.#COORD_TYPE# { #VALUES# }, [texture, { #COORDINATES# }];
    st.global.v4.#VALUE_TYPE#          [out_addr], { #VALUES# };

	ret;
}\0";

    #[test_cuda]
    fn texref_formats(api: impl CudaApi) {
        api.cuInit(0);
        api.cuCtxCreate_v2(&mut unsafe { mem::zeroed() }, 0, 0);
        let formats = [
            CUarray_format::CU_AD_FORMAT_UNSIGNED_INT8,
            CUarray_format::CU_AD_FORMAT_UNSIGNED_INT16,
            CUarray_format::CU_AD_FORMAT_UNSIGNED_INT32,
            CUarray_format::CU_AD_FORMAT_SIGNED_INT8,
            CUarray_format::CU_AD_FORMAT_SIGNED_INT16,
            CUarray_format::CU_AD_FORMAT_SIGNED_INT32,
            CUarray_format::CU_AD_FORMAT_HALF,
            CUarray_format::CU_AD_FORMAT_FLOAT,
        ];
        let coord_types = [
            CUarray_format::CU_AD_FORMAT_SIGNED_INT32,
            CUarray_format::CU_AD_FORMAT_FLOAT,
        ];
        let channels = [1u32, 2, 4];
        for format in formats {
            for dim in [1, 2, 3] {
                for channel in channels {
                    for coord_type in coord_types {
                        let mut texref = unsafe { mem::zeroed() };
                        api.cuTexRefCreate(&mut texref);
                        let mut array = unsafe { mem::zeroed() };
                        let (width, height, depth) = match dim {
                            1 => (4, 0, 0),
                            2 => (4, 4, 0),
                            3 => (4, 4, 4),
                            _ => panic!(),
                        };
                        let coordinates = match dim {
                            1 => "x",
                            2 => "x, y",
                            3 => "x, y, z, _unused",
                            _ => panic!(),
                        };
                        api.cuArray3DCreate_v2(
                            &mut array,
                            &CUDA_ARRAY3D_DESCRIPTOR_v2 {
                                Width: width,
                                Height: height,
                                Format: format,
                                NumChannels: channel,
                                Depth: depth,
                                Flags: 0,
                            },
                        );
                        let module_ptx = MODULE_PTX
                            .replace("#VALUE_TYPE#", to_ptx(format))
                            .replace("#DIMENSIONS#", &*format!("{dim}d"))
                            .replace("#COORD_TYPE#", to_ptx(coord_type))
                            .replace("#VALUES#", "x_value, y_value, z_value, w_value")
                            .replace("#COORDINATES#", coordinates);
                        let mut module = unsafe { mem::zeroed() };
                        api.cuModuleLoadData(&mut module, module_ptx.as_ptr().cast());
                        api.cuArrayDestroy(array);
                        api.cuTexRefDestroy(texref);
                        api.cuModuleUnload(module);
                    }
                }
            }
        }
    }

    fn to_ptx(format: CUarray_format) -> &'static str {
        match format {
            CUarray_format::CU_AD_FORMAT_UNSIGNED_INT8 => "u32",
            CUarray_format::CU_AD_FORMAT_UNSIGNED_INT16 => "u32",
            CUarray_format::CU_AD_FORMAT_UNSIGNED_INT32 => "u32",
            CUarray_format::CU_AD_FORMAT_SIGNED_INT8 => "s32",
            CUarray_format::CU_AD_FORMAT_SIGNED_INT16 => "s32",
            CUarray_format::CU_AD_FORMAT_SIGNED_INT32 => "s32",
            CUarray_format::CU_AD_FORMAT_HALF => "f32",
            CUarray_format::CU_AD_FORMAT_FLOAT => "f32",
            _ => panic!(),
        }
    }
}
