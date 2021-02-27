use crate::common::CudaDriverFns;
use cuda_types::*;
use half::f16;
use num_traits::AsPrimitive;
use rand::prelude::Distribution;
use rand_chacha::rand_core::SeedableRng;
use std::any::Any;
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
                "{coord_depth, coord_x, coord_y, coord_y}"
            } else {
                unreachable!()
            }
        } else {
            match self.geometry_dimensions {
                1 => "{coord_x}",
                2 => "{coord_x, coord_y}",
                3 => "{coord_x, coord_y, coord_z, coord_z}",
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

    fn address(&self, size: usize, x: u32, y: u32, z: u32, depth: u32) -> usize {
        match (self.is_layered, self.geometry_dimensions) {
            (true, 3) => (depth as usize * size * size) + (y as usize * size) + (x as usize),
            (true, 2) => (depth as usize * size) + (x as usize),
            (false, 3) => (z as usize * size * size) + (y as usize * size) + (x as usize),
            (false, 2) => (y as usize * size) + (x as usize),
            (false, 1) => x as usize,
            _ => unreachable!(),
        }
    }
}

fn prepare_kernel_values<Value: SustValue, Coordinate: SustValue>(
    kernel: &str,
) -> Result<String, fmt::Error> {
    let coordinate_type = Coordinate::ptx_type();
    let value_type = Value::ptx_type();
    let value_storage_type = Value::ptx_storage_type();
    let mut reg_values = String::new();
    let mut values = String::new();
    values.push('{');
    for dim in 0..4 {
        write!(values, "value_{}", dim)?;
        if dim != 4 - 1 {
            write!(values, ",")?;
        }
        writeln!(reg_values, ".reg .{} value_{};", Value::ptx_type(), dim)?;
    }
    values.push('}');
    let mut kernel = kernel.replace("#COORDINATE_TYPE#", coordinate_type);
    kernel = kernel.replace("#VALUE_TYPE#", value_type);
    kernel = kernel.replace("#VALUE_STORAGE_TYPE#", value_storage_type);
    kernel = kernel.replace("#REG_VALUES#", &reg_values);
    kernel = kernel.replace("#VALUES#", &values);
    Ok(kernel)
}

macro_rules! format_to_type {
    (CU_AD_FORMAT_UNSIGNED_INT8) => {
        u8
    };
    (CU_AD_FORMAT_UNSIGNED_INT16) => {
        u16
    };
    (CU_AD_FORMAT_UNSIGNED_INT32) => {
        u32
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
    (@level5 $format:expr, $channels:expr, $geometry:expr, $value_type:expr, {[$($coord_type:expr),+]}) => {
        $(
            paste! {
                #[allow(non_snake_case)]
                unsafe fn  [<kernel_tex_ $format _ $channels _ $geometry _ $value_type _ $coord_type>] <T: CudaDriverFns>(cuda: T) {
                    kernel_tex_impl::<T, format_to_type!($format), $channels, $value_type, $coord_type>(cuda, &$geometry, 0xef5864bda7b0b60f, CUarray_format:: $format)
                }
                cuda_driver_test!([<kernel_tex_ $format _ $channels _ $geometry _ $value_type _ $coord_type>]);
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
        //CU_AD_FORMAT_HALF,
        CU_AD_FORMAT_FLOAT
    ],
    [1, 2, 4],
    [ONED, TWOD, THREED, A1D, A2D],
    [u32, i32, f16, f32],
    [i32, f32]
);

trait SustValue: Copy + Default + Debug + PartialEq + 'static + Any {
    fn ptx_type() -> &'static str;
    fn ptx_storage_type() -> &'static str {
        Self::ptx_type()
    }
    fn gen<R: rand::Rng>(rng: &mut R) -> Self;
}

impl SustValue for u8 {
    fn ptx_type() -> &'static str {
        "b8"
    }

    fn gen<R: rand::Rng>(rng: &mut R) -> Self {
        rng.gen::<Self>()
    }
}

impl SustValue for u16 {
    fn ptx_type() -> &'static str {
        "b16"
    }

    fn gen<R: rand::Rng>(rng: &mut R) -> Self {
        rng.gen::<Self>()
    }
}

impl SustValue for u32 {
    fn ptx_type() -> &'static str {
        "u32"
    }

    fn gen<R: rand::Rng>(rng: &mut R) -> Self {
        rng.gen::<Self>()
    }
}

impl SustValue for u64 {
    fn ptx_type() -> &'static str {
        "b64"
    }

    fn gen<R: rand::Rng>(rng: &mut R) -> Self {
        rng.gen::<Self>()
    }
}

impl SustValue for i8 {
    fn ptx_type() -> &'static str {
        "b8"
    }

    fn gen<R: rand::Rng>(rng: &mut R) -> Self {
        rng.gen::<Self>()
    }
}

impl SustValue for i16 {
    fn ptx_type() -> &'static str {
        "b16"
    }

    fn gen<R: rand::Rng>(rng: &mut R) -> Self {
        rng.gen::<Self>()
    }
}

impl SustValue for i32 {
    fn ptx_type() -> &'static str {
        "s32"
    }

    fn gen<R: rand::Rng>(rng: &mut R) -> Self {
        rng.gen::<Self>()
    }
}

impl SustValue for f16 {
    fn ptx_type() -> &'static str {
        "f16"
    }

    fn ptx_storage_type() -> &'static str {
        "b16"
    }

    fn gen<R: rand::Rng>(rng: &mut R) -> Self {
        f16::from_f32(rng.gen::<f32>())
    }
}

impl SustValue for f32 {
    fn ptx_type() -> &'static str {
        "f32"
    }

    fn gen<R: rand::Rng>(rng: &mut R) -> Self {
        rng.gen::<Self>()
    }
}

unsafe fn byte_fill<T: Copy>(vec: &mut [T], value: u8) {
    let mut_view = std::slice::from_raw_parts_mut::<u8>(
        vec.as_mut_ptr() as _,
        mem::size_of::<T>() * vec.len(),
    );
    mut_view.fill(value);
}

const BYTE_FILLER1: u8 = 0xff;
const BYTE_FILLER2: u8 = 0xfe;

unsafe fn force_transmute<From: SustValue, To: SustValue>(f: From) -> To {
    if mem::size_of::<From>() == mem::size_of::<To>()
        && mem::size_of::<To>() == mem::size_of::<u32>()
    {
        return mem::transmute_copy(&f);
    }
    if mem::size_of::<To>() == mem::size_of::<u32>() {
        if let Some(value) = <dyn Any>::downcast_ref::<f16>(&f) {
            return mem::transmute_copy(&((value.to_f64() / f16::MAX.to_f64()) as f32));
        }
        if let Some(value) = <dyn Any>::downcast_ref::<u8>(&f) {
            return mem::transmute_copy(&((*value as f64 / u8::MAX as f64) as f32));
        }
        if let Some(value) = <dyn Any>::downcast_ref::<u16>(&f) {
            return mem::transmute_copy(&((*value as f64 / u16::MAX as f64) as f32));
        }
        if let Some(value) = <dyn Any>::downcast_ref::<i8>(&f) {
            return mem::transmute_copy(&((*value as f64 / i8::MAX as f64) as f32));
        }
        if let Some(value) = <dyn Any>::downcast_ref::<i16>(&f) {
            return mem::transmute_copy(&((*value as f64 / i16::MAX as f64) as f32));
        }
    }
    if mem::size_of::<To>() == mem::size_of::<f16>() {
        if let Some(value) = <dyn Any>::downcast_ref::<u8>(&f) {
            return mem::transmute_copy(&f16::from_f64(*value as f64 / u8::MAX as f64));
        }
        if let Some(value) = <dyn Any>::downcast_ref::<i8>(&f) {
            return mem::transmute_copy(&f16::from_f64(*value as f64 / i8::MAX as f64));
        }
        if let Some(value) = <dyn Any>::downcast_ref::<u32>(&f) {
            return mem::transmute_copy(&f16::from_f32(mem::transmute::<_, f32>(*value)));
        }
        if let Some(value) = <dyn Any>::downcast_ref::<i32>(&f) {
            return mem::transmute_copy(&f16::from_f32(mem::transmute::<_, f32>(*value)));
        }
        if let Some(value) = <dyn Any>::downcast_ref::<u16>(&f) {
            return mem::transmute_copy(&f16::from_f64(*value as f64 / u16::MAX as f64));
        }
        if let Some(value) = <dyn Any>::downcast_ref::<i16>(&f) {
            return mem::transmute_copy(&f16::from_f64(*value as f64 / i16::MAX as f64));
        }
        if let Some(value) = <dyn Any>::downcast_ref::<f32>(&f) {
            return mem::transmute_copy(&f16::from_f32(*value));
        }
    }
    panic!()
}

unsafe fn kernel_tex_impl<
    T: CudaDriverFns,
    Format: SustValue,
    const CHANNELS: usize,
    ValueType: SustValue,
    CoordinateType: SustValue + 'static + AsPrimitive<u32>,
>(
    cuda: T,
    geo: &GeometryTemplate,
    seed: u64,
    format: CUarray_format,
) where
    u32: AsPrimitive<CoordinateType>,
    Format: AsPrimitive<ValueType>,
{
    // Experimentally, tex1Dfetch (aka tex.1d with s32 index) behaves like
    // buffer indexing and ignores pixel channel+format information
    if geo.geometry_dimensions == 1
        && CoordinateType::ptx_type() == "s32"
        && (CHANNELS != 1 || mem::size_of::<ValueType>() != mem::size_of::<Format>())
    {
        return;
    }
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
    let size = 4usize;
    let random_size = rand::distributions::Uniform::<u32>::new(1, size as u32);
    let _ctx = create_context::<T>(&cuda);
    let (kernel, texref) = create_kernel_texref::<T, ValueType, CoordinateType>(&cuda, geo);
    let host_side_texref = create_host_side_data::<Format, CHANNELS, _>(size, &mut rng);
    create_array::<T, Format, CHANNELS, CoordinateType>(
        &cuda,
        geo,
        format,
        size,
        texref,
        &host_side_texref,
    );
    let result_buffer = allocate_result_buffer::<T, ValueType>(&cuda);
    let x_u32 = random_size.sample(&mut rng);
    let x = x_u32.as_();
    let y_u32 = random_size.sample(&mut rng);
    let y = y_u32.as_();
    let z_u32 = random_size.sample(&mut rng);
    let z = z_u32.as_();
    let depth = random_size.sample(&mut rng);
    launch_kernel::<T, CoordinateType>(&cuda, kernel, result_buffer, x, y, z, depth);
    let result = copy_results::<T, ValueType>(&cuda, result_buffer);
    // we are skipping rest of the components because HIP returns trash in unused components
    assert_eq!(
        &to_results(host_side_texref[geo.address(size, x_u32, y_u32, z_u32, depth)])[..CHANNELS],
        &result[..CHANNELS]
    );
}

unsafe fn allocate_result_buffer<T: CudaDriverFns, ValueType: SustValue>(cuda: &T) -> CUdeviceptr {
    let mut device_memory = mem::zeroed();
    assert_eq!(
        cuda.cuMemAlloc_v2(&mut device_memory, mem::size_of::<ValueType>() * 4),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(
        cuda.cuMemsetD8_v2(device_memory, BYTE_FILLER2, mem::size_of::<ValueType>() * 4),
        CUresult::CUDA_SUCCESS
    );
    device_memory
}

unsafe fn create_context<T: CudaDriverFns>(cuda: &T) -> CUcontext {
    assert_eq!(cuda.cuInit(0), CUresult::CUDA_SUCCESS);
    let mut ctx = ptr::null_mut();
    // We use primary context, because creating&destroying a normal context
    // means creating and destroying a thread, which is relatively slow
    assert_eq!(
        cuda.cuDevicePrimaryCtxRetain(&mut ctx, CUdevice_v1(0)),
        CUresult::CUDA_SUCCESS
    );
    assert_eq!(cuda.cuCtxSetCurrent(ctx), CUresult::CUDA_SUCCESS);
    ctx
}

unsafe fn create_kernel_texref<
    T: CudaDriverFns,
    ValueType: SustValue,
    CoordinateType: SustValue,
>(
    cuda: &T,
    geo: &GeometryTemplate,
) -> (CUfunction, CUtexref) {
    let mut kernel = include_str!("kernel_tex.ptx").to_string();
    kernel = geo.prepare_kernel(&kernel).unwrap();
    kernel = prepare_kernel_values::<ValueType, CoordinateType>(&kernel).unwrap();
    kernel.push('\0');
    let mut module = ptr::null_mut();
    assert_eq!(
        cuda.cuModuleLoadData(&mut module, kernel.as_ptr() as _),
        CUresult::CUDA_SUCCESS
    );
    let mut texref = ptr::null_mut();
    assert_eq!(
        cuda.cuModuleGetTexRef(&mut texref, module, b"image\0".as_ptr() as _),
        CUresult::CUDA_SUCCESS
    );
    let mut kernel = ptr::null_mut();
    assert_eq!(
        cuda.cuModuleGetFunction(&mut kernel, module, b"tex\0".as_ptr() as _),
        CUresult::CUDA_SUCCESS
    );
    (kernel, texref)
}

unsafe fn create_array<
    T: CudaDriverFns,
    Format: SustValue,
    const CHANNELS: usize,
    CoordinateType: SustValue,
>(
    cuda: &T,
    geo: &GeometryTemplate,
    format: CUarray_format,
    size: usize,
    texref: CUtexref,
    host_side_data: &[[Format; CHANNELS]],
) {
    // NVIDIA texrefs have this """fun""" """feature""", where 1d tex works
    // with integer indexing only if the texref has been bound to a buffer
    // and float indexing only if the texref has been bound to an array
    if geo.geometry_dimensions == 1 && CoordinateType::ptx_type() == "s32" {
        let bytesize = mem::size_of::<Format>() * CHANNELS * size;
        let mut devptr = mem::zeroed();
        assert_eq!(
            cuda.cuMemAlloc_v2(&mut devptr, bytesize),
            CUresult::CUDA_SUCCESS
        );
        assert_eq!(
            cuda.cuMemcpyHtoD_v2(devptr, host_side_data.as_ptr().cast(), bytesize),
            CUresult::CUDA_SUCCESS
        );
        let mut should_be_zero = 0;
        assert_eq!(
            cuda.cuTexRefSetAddress_v2(&mut should_be_zero, texref, devptr, bytesize),
            CUresult::CUDA_SUCCESS
        );
        assert_eq!(should_be_zero, 0);
    } else {
        let mut array = ptr::null_mut();
        let mut descriptor = mem::zeroed::<CUDA_ARRAY3D_DESCRIPTOR>();
        descriptor.Format = format;
        descriptor.NumChannels = CHANNELS as u32;
        geo.set_descriptor(&mut descriptor, size);
        assert_eq!(
            cuda.cuArray3DCreate_v2(&mut array, &descriptor),
            CUresult::CUDA_SUCCESS
        );
        copy_to_array::<T, Format, CHANNELS>(&cuda, geo, size, host_side_data, array);
        assert_eq!(
            cuda.cuTexRefSetArray(texref, array, CU_TRSA_OVERRIDE_FORMAT),
            CUresult::CUDA_SUCCESS
        );
    }
}

fn create_host_side_data<Format: SustValue, const CHANNELS: usize, R: rand::Rng>(
    size: usize,
    rng: &mut R,
) -> Vec<[Format; CHANNELS]> {
    let mut host_side_data = vec![[<Format as Default>::default(); CHANNELS]; size * size * size];
    for pixel in host_side_data.iter_mut() {
        for channel_element in pixel.iter_mut() {
            *channel_element = Format::gen::<R>(rng)
        }
    }
    host_side_data
}

unsafe fn copy_to_array<T: CudaDriverFns, Format: SustValue, const CHANNELS: usize>(
    cuda: &T,
    geo: &GeometryTemplate,
    size: usize,
    host_side_data: &[[Format; CHANNELS]],
    cu_array: CUarray,
) {
    let mut memcpy_desc = mem::zeroed::<CUDA_MEMCPY3D>();
    geo.set_memcpy(
        &mut memcpy_desc,
        size,
        (mem::size_of::<Format>() * CHANNELS) as u32,
    );
    memcpy_desc.srcMemoryType = CUmemorytype::CU_MEMORYTYPE_HOST;
    memcpy_desc.srcHost = host_side_data.as_ptr() as _;
    memcpy_desc.dstMemoryType = CUmemorytype::CU_MEMORYTYPE_ARRAY;
    memcpy_desc.dstArray = cu_array;
    assert_eq!(cuda.cuMemcpy3D_v2(&memcpy_desc), CUresult::CUDA_SUCCESS);
}

unsafe fn launch_kernel<T: CudaDriverFns, CoordinateType: SustValue>(
    cuda: &T,
    kernel: CUfunction,
    deviceptr: CUdeviceptr,
    x: CoordinateType,
    y: CoordinateType,
    z: CoordinateType,
    depth: u32,
) {
    let mut args = vec![
        &deviceptr as *const _ as *const c_void,
        &x as *const _ as *const c_void,
        &y as *const _ as *const _,
        &z as *const _ as *const _,
        &depth as *const _ as *const _,
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
}

unsafe fn copy_results<T: CudaDriverFns, Value: SustValue>(
    cuda: &T,
    deviceptr: CUdeviceptr,
) -> [Value; 4] {
    let mut result = [
        Value::default(),
        Value::default(),
        Value::default(),
        Value::default(),
    ];
    byte_fill(&mut result, BYTE_FILLER1);
    assert_eq!(
        cuda.cuMemcpyDtoH_v2(
            result.as_mut_ptr() as _,
            deviceptr,
            mem::size_of::<Value>() * 4,
        ),
        CUresult::CUDA_SUCCESS
    );
    result
}

unsafe fn to_results<
    Format: SustValue + AsPrimitive<Value>,
    Value: SustValue,
    const CHANNELS: usize,
>(
    input: [Format; CHANNELS],
) -> [Value; 4] {
    match &input[..] {
        [x] => [
            force_transmute::<_, Value>(*x),
            Value::default(),
            Value::default(),
            Value::default(),
        ],
        [x, y] => [
            force_transmute::<_, Value>(*x),
            force_transmute::<_, Value>(*y),
            Value::default(),
            Value::default(),
        ],
        [x, y, z, w] => [
            force_transmute::<_, Value>(*x),
            force_transmute::<_, Value>(*y),
            force_transmute::<_, Value>(*z),
            force_transmute::<_, Value>(*w),
        ],
        _ => unreachable!(),
    }
}
