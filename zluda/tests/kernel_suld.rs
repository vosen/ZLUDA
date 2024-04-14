use crate::common::CudaDriverFns;
use cuda_types::*;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;
use rand_chacha::rand_core::SeedableRng;
use std::fmt::Debug;
use std::fmt::{self, Write};
use std::{ffi::c_void, mem, ptr};

mod common;

const ONED: GeometryTemplate = GeometryTemplate {
    geometry_dimensions: 1,
    is_layered: false,
    ptx_name: "1d",
};

const TWOD: GeometryTemplate = GeometryTemplate {
    geometry_dimensions: 2,
    is_layered: false,
    ptx_name: "2d",
};

const THREED: GeometryTemplate = GeometryTemplate {
    geometry_dimensions: 3,
    is_layered: false,
    ptx_name: "3d",
};

const A1D: GeometryTemplate = GeometryTemplate {
    geometry_dimensions: 2,
    is_layered: true,
    ptx_name: "a1d",
};

const A2D: GeometryTemplate = GeometryTemplate {
    geometry_dimensions: 3,
    is_layered: true,
    ptx_name: "a2d",
};

struct GeometryTemplate {
    geometry_dimensions: usize,
    is_layered: bool,
    ptx_name: &'static str,
}

impl GeometryTemplate {
    fn prepare_kernel(&self, kernel: &str) -> Result<String, fmt::Error> {
        let coordinates = if self.is_layered {
            if self.geometry_dimensions == 2 {
                "{coord_depth, coord_x}"
            } else if self.geometry_dimensions == 3 {
                "{coord_depth, coord_x, coord_y, 0}"
            } else {
                unreachable!()
            }
        } else {
            match self.geometry_dimensions {
                1 => "{coord_x}",
                2 => "{coord_x, coord_y}",
                3 => "{coord_x, coord_y, coord_z, 0}",
                _ => unreachable!(),
            }
        };
        let mut kernel = kernel.replace("#GEOMETRY#", self.ptx_name);
        kernel = kernel.replace("#COORDINATES#", coordinates);
        Ok(kernel)
    }

    fn set_descriptor(&self, desc: &mut CUDA_ARRAY3D_DESCRIPTOR, size: usize) {
        desc.Width = size;
        if self.is_layered {
            desc.Flags |= CUDA_ARRAY3D_LAYERED;
            desc.Depth = size;
            if self.geometry_dimensions >= 3 {
                desc.Height = size;
            }
        } else {
            if self.geometry_dimensions >= 2 {
                desc.Height = size;
            }
            if self.geometry_dimensions >= 3 {
                desc.Depth = size;
            }
        }
    }

    fn set_memcpy(&self, memcpy_desc: &mut CUDA_MEMCPY3D, size: usize, size_of_pixel: u32) {
        memcpy_desc.WidthInBytes = size_of_pixel as usize * size;
        if self.is_layered {
            memcpy_desc.Depth = size;
            if self.geometry_dimensions >= 3 {
                memcpy_desc.Height = size;
            } else {
                memcpy_desc.Height = 1;
            }
        } else {
            if self.geometry_dimensions >= 2 {
                memcpy_desc.Height = size;
            } else {
                memcpy_desc.Height = 1;
            }
            if self.geometry_dimensions >= 3 {
                memcpy_desc.Depth = size;
            } else {
                memcpy_desc.Depth = 1;
            }
        }
    }

    fn address(&self, size: usize, x: u32, y: u32, z: u32, size_of_pixel: u32) -> usize {
        match (self.is_layered, self.geometry_dimensions) {
            (true, 3) => {
                (z as usize * size * size) + (y as usize * size) + ((x / size_of_pixel) as usize)
            }
            (true, 2) => (z as usize * size) + ((x / size_of_pixel) as usize),
            (false, 3) => {
                (z as usize * size * size) + (y as usize * size) + ((x / size_of_pixel) as usize)
            }
            (false, 2) => (y as usize * size) + ((x / size_of_pixel) as usize),
            (false, 1) => (x / size_of_pixel) as usize,
            _ => unreachable!(),
        }
    }
}

fn prepare_kernel_values<U: SustValue, const N: usize>(
    kernel: &str,
    bindless: bool,
) -> Result<String, fmt::Error> {
    let mut param_values = String::new();
    let mut reg_values = String::new();
    let mut values = String::new();
    values.push('{');
    for dim in 0..N {
        write!(
            param_values,
            ".param .{} param_value_{}",
            U::ptx_type(),
            dim
        )?;
        if dim != N - 1 {
            param_values.push_str(",");
        }
        writeln!(reg_values, ".reg .{} value_{};", U::ptx_type(), dim)?;
        write!(values, "value_{}", dim)?;
        if dim != N - 1 {
            write!(values, ",")?;
        }
    }
    values.push('}');
    let vec_prefix = match N {
        0 | 1 => ".",
        2 => ".v2.",
        4 => ".v4.",
        _ => panic!(),
    };
    let mut format = vec_prefix.to_string();
    format.push_str(U::ptx_type());
    let mut kernel = kernel.replace("#PARAM_VALUES#", &param_values);
    kernel = kernel.replace("#REG_VALUES#", &reg_values);
    kernel = kernel.replace("#VALUES#", &values);
    kernel = kernel.replace("#FORMAT#", &format);
    kernel = kernel.replace(
        "#IMAGE_SRC#",
        if bindless { "image_bindless" } else { "image" },
    );
    Ok(kernel)
}

fn sizeof_pixel(format: CUarray_format, channels: u32) -> u32 {
    let channel_size = match format {
        CUarray_format::CU_AD_FORMAT_UNSIGNED_INT8 | CUarray_format::CU_AD_FORMAT_SIGNED_INT8 => 1,
        CUarray_format::CU_AD_FORMAT_UNSIGNED_INT16
        | CUarray_format::CU_AD_FORMAT_SIGNED_INT16
        | CUarray_format::CU_AD_FORMAT_HALF => 2,
        CUarray_format::CU_AD_FORMAT_UNSIGNED_INT32
        | CUarray_format::CU_AD_FORMAT_SIGNED_INT32
        | CUarray_format::CU_AD_FORMAT_FLOAT => 4,
        _ => unimplemented!(),
    };
    channel_size * channels
}

macro_rules! format_to_type {
    (CU_AD_FORMAT_UNSIGNED_INT8) => {
        u8
    };
    (CU_AD_FORMAT_UNSIGNED_INT16) => {
        i16
    };
    (CU_AD_FORMAT_UNSIGNED_INT32) => {
        i32
    };
    (CU_AD_FORMAT_SIGNED_INT8) => {
        i8
    };
    (CU_AD_FORMAT_SIGNED_INT16) => {
        i16
    };
    (CU_AD_FORMAT_SIGNED_INT32) => {
        i32
    };
    (CU_AD_FORMAT_HALF) => {
        half::f16
    };
    (CU_AD_FORMAT_FLOAT) => {
        f32
    };
}

use paste::paste;
macro_rules! generate_tests {
    ($format:tt, $channels:tt, $geometry:tt, $inst_size:tt, $inst_vec:tt)=> {
        generate_tests!(@level1 $format, {$channels, {$geometry, {$inst_size, {$inst_vec}}}});
    };
    (@level1 [$($format:expr),+], $rest:tt) => {
        $(generate_tests!(@level2 $format, $rest);)+
    };
    (@level2 $format:expr, {[$($channels:expr),+], $rest:tt}) => {
        $(generate_tests!(@level3 $format, $channels, $rest);)+
    };
    (@level3 $format:expr, $channels:expr, {[$($geometry:expr),+], $rest:tt}) => {
        $(generate_tests!(@level4 $format, $channels, $geometry, $rest);)+
    };
    (@level4 $format:expr, $channels:expr, $geometry:expr, {[$($inst_size:expr),+], $rest:tt}) => {
        $(generate_tests!(@level5 $format, $channels, $geometry, $inst_size, $rest);)+
    };
    (@level5 $format:expr, $channels:expr, $geometry:expr, $inst_size:expr, {[$($inst_vec:expr),+]}) => {
        $(
            paste! {
                #[allow(non_snake_case)]
                unsafe fn  [<kernel_suld_ $format _ $channels _ $geometry _ $inst_size _ $inst_vec>] <T: CudaDriverFns>(cuda: T) {
                    kernel_suld_impl::<T, format_to_type!($format), $channels, $inst_size, $inst_vec>(cuda, &$geometry, 0xef5864bda7b0b60f, CUarray_format:: $format, false)
                }
                cuda_driver_test!([<kernel_suld_ $format _ $channels _ $geometry _ $inst_size _ $inst_vec>]);

                #[allow(non_snake_case)]
                unsafe fn  [<kernel_suld_ $format _ $channels _ $geometry _ $inst_size _ $inst_vec _bindless>] <T: CudaDriverFns>(cuda: T) {
                    kernel_suld_impl::<T, format_to_type!($format), $channels, $inst_size, $inst_vec>(cuda, &$geometry, 0xef5864bda7b0b60f, CUarray_format:: $format, true)
                }
                cuda_driver_test!([<kernel_suld_ $format _ $channels _ $geometry _ $inst_size _ $inst_vec _bindless>]);
            }
        )+
    };
}

generate_tests!(
    [
        CU_AD_FORMAT_UNSIGNED_INT8,
        CU_AD_FORMAT_UNSIGNED_INT16,
        CU_AD_FORMAT_UNSIGNED_INT32,
        CU_AD_FORMAT_SIGNED_INT8,
        CU_AD_FORMAT_SIGNED_INT16,
        CU_AD_FORMAT_SIGNED_INT32,
        CU_AD_FORMAT_HALF,
        CU_AD_FORMAT_FLOAT
    ],
    [1, 2, 4],
    [ONED, TWOD, THREED, A1D, A2D],
    [u8, u16, u32, u64],
    [1, 2, 4]
);

trait SustValue: Copy + Default + Debug + PartialEq {
    fn ptx_type() -> &'static str;
}

impl SustValue for u8 {
    fn ptx_type() -> &'static str {
        "b8"
    }
}

impl SustValue for u16 {
    fn ptx_type() -> &'static str {
        "b16"
    }
}

impl SustValue for u32 {
    fn ptx_type() -> &'static str {
        "b32"
    }
}

impl SustValue for u64 {
    fn ptx_type() -> &'static str {
        "b64"
    }
}

unsafe fn as_bytes_mut<'a, T>(t: &'a mut T) -> &'a mut [u8] {
    std::slice::from_raw_parts_mut::<u8>(t as *mut T as _, mem::size_of::<T>())
}

unsafe fn byte_fill<T: Copy>(vec: &mut Vec<T>, value: u8) {
    let mut_view = std::slice::from_raw_parts_mut::<u8>(
        vec.as_mut_ptr() as _,
        mem::size_of::<T>() * vec.len(),
    );
    mut_view.fill(value);
}

const BYTE_FILLER1: u8 = 0xff;
const BYTE_FILLER2: u8 = 0xfe;
const BYTE_FILLER3: u8 = 0xfd;

#[repr(C)]
union UnionHack<From: Copy, To: Copy> {
    from: From,
    to: To,
}

unsafe fn force_transmute<From: Copy, To: Copy>(f: From, filler: u8) -> To {
    let mut u: UnionHack<From, To> = mem::zeroed();
    as_bytes_mut(&mut u).fill(filler);
    u.from = f;
    u.to
}

unsafe fn kernel_suld_impl<
    T: CudaDriverFns,
    Format: Default + Copy + Debug,
    const CHANNELS: usize,
    SustType: SustValue,
    const SULD_N: usize,
>(
    cuda: T,
    geo: &GeometryTemplate,
    seed: u64,
    format: CUarray_format,
    bindless: bool,
) where
    Standard: Distribution<SustType>,
{
    // CUDA kernels fail at runtime if the pixel is smaller than `sust` write size
    if mem::size_of::<Format>() * CHANNELS < mem::size_of::<SustType>() * SULD_N {
        return;
    }
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
    let size = 4usize;
    let random_size = rand::distributions::Uniform::<u32>::new(1, size as u32);
    let mut kernel = include_str!("kernel_suld.ptx").to_string();
    kernel = geo.prepare_kernel(&kernel).unwrap();
    kernel = prepare_kernel_values::<SustType, SULD_N>(&kernel, bindless).unwrap();
    kernel.push('\0');
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    // We use primary context, because creating&destroying a normal context
    // means creating and destroying a thread, which is relatively slow
    assert_eq!(
        cuda.cuDevicePrimaryCtxRetain(&mut ctx, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(cuda.cuCtxSetCurrent(ctx), CUresult::CUDA_SUCCESS);
    let mut module = ptr::null_mut();
    assert_eq!(
        cuda.cuModuleLoadData(&mut module, kernel.as_ptr() as _),
        CUresult::CUDA_SUCCESS
    );
    let mut array = ptr::null_mut();
    let depth = size;
    let width = size;
    let height = size;
    let mut descriptor = mem::zeroed::<CUDA_ARRAY3D_DESCRIPTOR>();
    descriptor.Flags = CUDA_ARRAY3D_SURFACE_LDST;
    descriptor.Format = format;
    descriptor.NumChannels = CHANNELS as u32;
    geo.set_descriptor(&mut descriptor, size);
    let mut host_side_data =
        vec![[<Format as Default>::default(); CHANNELS]; width * height * depth];
    byte_fill(&mut host_side_data, BYTE_FILLER1);
    let sizeof_pixel = sizeof_pixel(format, CHANNELS as u32);
    let x = random_size.sample(&mut rng) * sizeof_pixel;
    let y = random_size.sample(&mut rng);
    let z = random_size.sample(&mut rng);
    let values = [rng.gen::<SustType>(); SULD_N];
    let converted_values = force_transmute(values, BYTE_FILLER3);
    *host_side_data.get_unchecked_mut(geo.address(size, x, y, z, sizeof_pixel)) = converted_values;
    assert_eq!(
        cuda.cuArray3DCreate_v2(&mut array, &descriptor),
        CUresult::CUDA_SUCCESS
    );
    let mut bindless_image = 0u64;
    if bindless {
        assert_eq!(
            cuda.cuSurfObjectCreate(
                &mut bindless_image,
                &CUDA_RESOURCE_DESC {
                    resType: CUresourcetype::CU_RESOURCE_TYPE_ARRAY,
                    res: CUDA_RESOURCE_DESC_st__bindgen_ty_1 {
                        array: CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_1 { hArray: array }
                    },
                    flags: 0
                }
            ),
            CUresult::CUDA_SUCCESS
        );
    } else {
        let mut surfref = ptr::null_mut();
        assert_eq!(
            cuda.cuModuleGetSurfRef(&mut surfref, module, b"image\0".as_ptr() as _),
            CUresult::CUDA_SUCCESS
        );
        assert_eq!(
            cuda.cuSurfRefSetArray(surfref, array, 0),
            CUresult::CUDA_SUCCESS
        );
    }
    let mut memcpy_desc = mem::zeroed::<CUDA_MEMCPY3D>();
    geo.set_memcpy(&mut memcpy_desc, size, sizeof_pixel);
    memcpy_desc.srcMemoryType = CUmemorytype::CU_MEMORYTYPE_HOST;
    memcpy_desc.srcHost = host_side_data.as_mut_ptr() as _;
    memcpy_desc.dstMemoryType = CUmemorytype::CU_MEMORYTYPE_ARRAY;
    memcpy_desc.dstArray = array;
    assert_eq!(cuda.cuMemcpy3D_v2(&memcpy_desc), CUresult::CUDA_SUCCESS);
    let mut kernel = ptr::null_mut();
    assert_eq!(
        cuda.cuModuleGetFunction(&mut kernel, module, b"suld\0".as_ptr() as _),
        CUresult::CUDA_SUCCESS
    );
    let mut device_memory = mem::zeroed();
    assert_eq!(
        cuda.cuMemAlloc_v2(&mut device_memory, mem::size_of::<SustType>() * SULD_N),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(
        cuda.cuMemsetD8_v2(
            device_memory,
            BYTE_FILLER2,
            mem::size_of::<SustType>() * SULD_N
        ),
        CUresult::CUDA_SUCCESS
    );
    let mut args = vec![
        &device_memory as *const _ as *const c_void,
        &x as *const _ as *const c_void,
        &y as *const _ as *const _,
        &z as *const _ as *const _,
        &bindless_image as *const _ as *const _,
    ];
    assert_eq!(
        cuda.cuLaunchKernel(
            kernel,
            1,
            1,
            1,
            1,
            1,
            1,
            0,
            0 as _,
            args.as_mut_ptr() as _,
            ptr::null_mut(),
        ),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(cuda.cuStreamSynchronize(0 as _), CUresult::CUDA_SUCCESS);
    let mut actual_values = [SustType::default(); SULD_N];
    let actual_values_buffer = as_bytes_mut(&mut actual_values);
    assert_eq!(
        cuda.cuMemcpyDtoH_v2(
            actual_values_buffer.as_mut_ptr() as _,
            device_memory,
            actual_values_buffer.len(),
        ),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(values, actual_values);
    let mut unused = mem::zeroed();
    assert_eq!(cuda.cuCtxPopCurrent(&mut unused), CUresult::CUDA_SUCCESS);
}
