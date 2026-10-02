use zluda_common::test;

pub(crate) struct Zluda {
    runtime: test::Runtime,
}

impl Zluda {
    fn load() -> Self {
        let runtime = test::Runtime::load_hip();
        Self { runtime }
    }
}

pub(crate) struct Cuda {
    runtime: test::Runtime,
    cufft: libloading::Library,
}

impl Cuda {
    #[cfg(windows)]
    fn cufft_path() -> String {
        std::env::var("CUDA_PATH")
            .map(|path| format!("{path}\\bin\\cufft64_12.dll"))
            .unwrap()
    }

    #[cfg(unix)]
    fn cufft_path() -> String {
        "/usr/local/cuda/lib64/libcufft.so.12".to_string()
    }

    fn load() -> Self {
        let runtime = test::Runtime::load_cuda();
        let cufft = unsafe { libloading::Library::new(Self::cufft_path()) }.unwrap();
        Self { runtime, cufft }
    }
}

macro_rules! api {
    ($($abi:literal fn $fn_name:ident( $( $arg_id:ident : $arg_type:ty ),* ) -> $ret_type:ty;)* ) => {
        pub(crate) trait CufftApi {
            fn new() -> Self;
            fn runtime(&self) -> &zluda_common::test::Runtime;
            $(
                #[allow(non_snake_case, dead_code)]
                fn $fn_name(&self, $( $arg_id : $arg_type ),* ) {
                    paste::paste!{ self.[< $fn_name _unchecked >]( $( $arg_id ),* ) }.unwrap()
                }
                paste::paste!{ #[allow(non_snake_case, dead_code)] fn [< $fn_name _unchecked>](&self, $( $arg_id : $arg_type ),* ) -> $ret_type; }
            )*
        }

        impl CufftApi for Zluda {
            fn new() -> Self { Self::load() }
            fn runtime(&self) -> &zluda_common::test::Runtime { &self.runtime }
            $(
                paste::paste!{ fn [< $fn_name _unchecked >](&self, $( $arg_id : $arg_type ),* )  -> $ret_type {
                    unsafe { super::$fn_name( $( $arg_id ),* ) }
                }}
            )*
        }

        impl CufftApi for Cuda {
            fn new() -> Self { Self::load() }
            fn runtime(&self) -> &zluda_common::test::Runtime { &self.runtime }
            $(
                paste::paste!{ fn [< $fn_name _unchecked >](&self, $( $arg_id : $arg_type ),* )  -> $ret_type {
                    let func = unsafe { self.cufft.get::<unsafe extern $abi fn ( $( $arg_type ),* ) -> $ret_type>(concat!(stringify!($fn_name), "\0").as_bytes()) }.unwrap();
                    unsafe { (func)( $( $arg_id ),* ) }
                }}
            )*
        }
    };
}

macro_rules! ignore {
    ($($abi:literal fn $name:ident($($argument:ident: $type_:ty),*) -> $result:ty;)*) => {};
}

cuda_macros::cufft_function_declarations!(
    ignore,
    api <= [
        cufftPlan3d,
        cufftMakePlan3d,
        cufftMakePlanMany64,
        cufftCreate,
        cufftGetSize,
        cufftGetSizeMany64,
        cufftSetWorkArea,
        cufftSetAutoAllocation,
        cufftSetStream,
        cufftExecR2C,
        cufftExecC2R,
        cufftDestroy,
        cufftGetVersion,
        cufftGetProperty,
    ]
);

mod api_tests {
    use super::CufftApi;
    use cuda_macros::test_cuda;
    use cuda_types::{
        cuda::CUstream,
        cufft::{cufftComplex, cufftResult, cufftResultConsts, cufftType, libraryPropertyType},
    };
    use std::mem;

    #[test_cuda]
    fn create_destroy_and_reject_stale_handle(api: impl CufftApi) {
        let mut handle = unsafe { mem::zeroed() };
        api.cufftCreate(&mut handle);
        assert_ne!(handle.0, 0);
        api.cufftDestroy(handle);
        assert_ne!(api.cufftDestroy_unchecked(handle), cufftResult::SUCCESS);
    }

    #[test_cuda]
    fn plan_non_cubic_3d_transforms(api: impl CufftApi) {
        for type_ in [
            cufftType::CUFFT_C2C,
            cufftType::CUFFT_R2C,
            cufftType::CUFFT_C2R,
            cufftType::CUFFT_Z2Z,
            cufftType::CUFFT_D2Z,
            cufftType::CUFFT_Z2D,
        ] {
            let mut handle = unsafe { mem::zeroed() };
            api.cufftPlan3d(&mut handle, 3, 4, 5, type_);
            api.cufftDestroy(handle);
        }
    }

    #[test_cuda]
    fn plan_lifecycle(api: impl CufftApi) {
        let context = api.runtime().init_context();
        let mut handle = unsafe { mem::zeroed() };
        api.cufftCreate(&mut handle);
        let stream = api.runtime().create_stream();
        api.cufftSetStream(handle, CUstream(stream.cast()));
        api.cufftSetAutoAllocation(handle, 0);

        let mut work_size = 0;

        api.cufftMakePlan3d(handle, 3, 4, 5, cufftType::CUFFT_R2C, &mut work_size);
        let mut queried_work_size = 0;

        api.cufftGetSize(handle, &mut queried_work_size);
        assert_eq!(queried_work_size, work_size);
        let workspace: *mut std::ffi::c_void = api.runtime().allocate(work_size.max(1));

        api.cufftSetWorkArea(handle, workspace);
        api.cufftDestroy(handle);
        api.runtime().free(workspace);
        api.runtime().destroy_stream(stream);
        api.runtime().destroy_context(context);
    }

    #[test_cuda]
    fn plan_many_64_bit(api: impl CufftApi) {
        let mut handle = unsafe { mem::zeroed() };
        api.cufftCreate(&mut handle);
        let mut dimensions = [3_i64, 4, 5];
        let mut work_size = 0;
        api.cufftMakePlanMany64(
            handle,
            dimensions.len() as i32,
            dimensions.as_mut_ptr(),
            std::ptr::null_mut(),
            1,
            60,
            std::ptr::null_mut(),
            1,
            36,
            cufftType::CUFFT_R2C,
            1,
            &mut work_size,
        );
        api.cufftDestroy(handle);

        api.cufftCreate(&mut handle);
        let mut queried_work_size = 0;
        api.cufftGetSizeMany64(
            handle,
            dimensions.len() as i32,
            dimensions.as_mut_ptr(),
            std::ptr::null_mut(),
            1,
            60,
            std::ptr::null_mut(),
            1,
            36,
            cufftType::CUFFT_R2C,
            1,
            &mut queried_work_size,
        );
        api.cufftDestroy(handle);
    }

    #[test_cuda]
    fn real_3d_execution(api: impl CufftApi) {
        let context = api.runtime().init_context();
        const NX: usize = 3;
        const NY: usize = 4;
        const NZ: usize = 5;
        const ELEMENTS: usize = NX * NY * NZ;
        const COMPLEX_ELEMENTS: usize = NX * NY * (NZ / 2 + 1);

        let input = (0..ELEMENTS)
            .map(|index| ((index % 11) as f32 - 5.0) / 7.0)
            .collect::<Vec<_>>();
        let device_input = api
            .runtime()
            .allocate(std::mem::size_of_val(input.as_slice()));
        let device_spectrum = api
            .runtime()
            .allocate(COMPLEX_ELEMENTS * std::mem::size_of::<cufftComplex>());
        let device_round_trip = api
            .runtime()
            .allocate(ELEMENTS * std::mem::size_of::<f32>());
        api.runtime().copy_to_device(device_input, &input);

        let stream = api.runtime().create_stream();
        let mut forward = cuda_types::cufft::cufftHandle(0);
        api.cufftPlan3d(
            &mut forward,
            NX as i32,
            NY as i32,
            NZ as i32,
            cufftType::CUFFT_R2C,
        );
        api.cufftSetStream(forward, CUstream(stream.cast()));
        api.cufftExecR2C(forward, device_input.cast(), device_spectrum.cast());
        api.runtime().synchronize(stream);

        let mut spectrum = vec![cufftComplex { x: 0.0, y: 0.0 }; COMPLEX_ELEMENTS];
        api.runtime().copy_to_host(&mut spectrum, device_spectrum);
        for kx in 0..NX {
            for ky in 0..NY {
                for kz in 0..(NZ / 2 + 1) {
                    let mut expected = cufftComplex { x: 0.0, y: 0.0 };
                    for x in 0..NX {
                        for y in 0..NY {
                            for z in 0..NZ {
                                let phase = -std::f32::consts::TAU
                                    * ((kx * x) as f32 / NX as f32
                                        + (ky * y) as f32 / NY as f32
                                        + (kz * z) as f32 / NZ as f32);
                                let value = input[(x * NY + y) * NZ + z];
                                expected.x += value * phase.cos();
                                expected.y += value * phase.sin();
                            }
                        }
                    }
                    let actual = spectrum[(kx * NY + ky) * (NZ / 2 + 1) + kz];
                    let tolerance = 2.0e-4 * expected.x.abs().max(expected.y.abs()).max(1.0);
                    assert!((actual.x - expected.x).abs() <= tolerance);
                    assert!((actual.y - expected.y).abs() <= tolerance);
                }
            }
        }

        let mut inverse = cuda_types::cufft::cufftHandle(0);
        api.cufftPlan3d(
            &mut inverse,
            NX as i32,
            NY as i32,
            NZ as i32,
            cufftType::CUFFT_C2R,
        );
        api.cufftSetStream(inverse, CUstream(stream.cast()));
        api.cufftExecC2R(inverse, device_spectrum.cast(), device_round_trip.cast());
        api.runtime().synchronize(stream);

        let mut round_trip = vec![0.0; ELEMENTS];
        api.runtime()
            .copy_to_host(&mut round_trip, device_round_trip);
        for (actual, input) in round_trip.into_iter().zip(input) {
            let expected = input * ELEMENTS as f32;
            let tolerance = 2.0e-4 * expected.abs().max(1.0);
            assert!((actual - expected).abs() <= tolerance);
        }

        api.cufftDestroy(inverse);
        api.cufftDestroy(forward);
        api.runtime().destroy_stream(stream);
        api.runtime().free(device_round_trip);
        api.runtime().free(device_spectrum);
        api.runtime().free(device_input);
        api.runtime().destroy_context(context);
    }

    #[test_cuda]
    fn version_and_properties(api: impl CufftApi) {
        let mut version = 0;
        api.cufftGetVersion(&mut version);
        assert!(version > 0);

        for property in [
            libraryPropertyType::MAJOR_VERSION,
            libraryPropertyType::MINOR_VERSION,
            libraryPropertyType::PATCH_LEVEL,
        ] {
            let mut value = -1;
            api.cufftGetProperty(property, &mut value);
            assert!(value >= 0);
        }
    }
}
