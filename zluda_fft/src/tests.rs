use cuda_types::cufft::cudaStream_t;
use std::ffi::c_void;
use std::sync::{Mutex, MutexGuard};

static GPU_TEST_LOCK: Mutex<()> = Mutex::new(());

fn gpu_test_guard() -> MutexGuard<'static, ()> {
    GPU_TEST_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

enum RuntimeKind {
    Cuda,
    Hip,
}

pub(crate) struct Runtime {
    library: libloading::Library,
    kind: RuntimeKind,
}

impl Runtime {
    fn function<T: Copy>(&self, cuda: &[u8], hip: &[u8]) -> T {
        let name = match self.kind {
            RuntimeKind::Cuda => cuda,
            RuntimeKind::Hip => hip,
        };
        unsafe { *self.library.get::<T>(name).unwrap() }
    }

    fn allocate(&self, size: usize) -> *mut c_void {
        let function = self.function::<unsafe extern "system" fn(*mut *mut c_void, usize) -> i32>(
            b"cudaMalloc\0",
            b"hipMalloc\0",
        );
        let mut pointer = std::ptr::null_mut();
        assert_eq!(unsafe { function(&mut pointer, size) }, 0);
        pointer
    }

    fn free(&self, pointer: *mut c_void) {
        let function = self
            .function::<unsafe extern "system" fn(*mut c_void) -> i32>(b"cudaFree\0", b"hipFree\0");
        assert_eq!(unsafe { function(pointer) }, 0);
    }

    fn create_stream(&self) -> cudaStream_t {
        let function = self.function::<unsafe extern "system" fn(*mut cudaStream_t) -> i32>(
            b"cudaStreamCreate\0",
            b"hipStreamCreate\0",
        );
        let mut stream = unsafe { std::mem::zeroed() };
        assert_eq!(unsafe { function(&mut stream) }, 0);
        stream
    }

    fn destroy_stream(&self, stream: cudaStream_t) {
        let function = self.function::<unsafe extern "system" fn(cudaStream_t) -> i32>(
            b"cudaStreamDestroy\0",
            b"hipStreamDestroy\0",
        );
        assert_eq!(unsafe { function(stream) }, 0);
    }

    fn synchronize(&self, stream: cudaStream_t) {
        let function = self.function::<unsafe extern "system" fn(cudaStream_t) -> i32>(
            b"cudaStreamSynchronize\0",
            b"hipStreamSynchronize\0",
        );
        assert_eq!(unsafe { function(stream) }, 0);
    }

    fn copy_to_device<T>(&self, device: *mut T, host: &[T]) {
        self.copy(
            device.cast(),
            host.as_ptr().cast(),
            std::mem::size_of_val(host),
            1,
        );
    }

    fn copy_to_host<T>(&self, host: &mut [T], device: *const T) {
        self.copy(
            host.as_mut_ptr().cast(),
            device.cast(),
            std::mem::size_of_val(host),
            2,
        );
    }

    fn copy(&self, destination: *mut c_void, source: *const c_void, size: usize, kind: i32) {
        let function = self
            .function::<unsafe extern "system" fn(*mut c_void, *const c_void, usize, i32) -> i32>(
                b"cudaMemcpy\0",
                b"hipMemcpy\0",
            );
        assert_eq!(unsafe { function(destination, source, size, kind) }, 0);
    }
}

pub(crate) struct Zluda {
    runtime: Runtime,
    _gpu_test_guard: MutexGuard<'static, ()>,
}

pub(crate) struct Cuda {
    _driver: libloading::Library,
    cufft: libloading::Library,
    runtime: Runtime,
    _gpu_test_guard: MutexGuard<'static, ()>,
}

impl Cuda {
    #[cfg(windows)]
    const DRIVER_PATH: &'static str = "C:\\Windows\\System32\\nvcuda.dll";
    #[cfg(unix)]
    const DRIVER_PATH: &'static str = "/usr/lib/x86_64-linux-gnu/libcuda.so.1";

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

    #[cfg(windows)]
    fn runtime_path() -> String {
        std::env::var("CUDA_PATH")
            .map(|path| format!("{path}\\bin\\cudart64_12.dll"))
            .unwrap()
    }

    #[cfg(unix)]
    fn runtime_path() -> String {
        "/usr/local/cuda/lib64/libcudart.so.12".to_string()
    }
}

impl Zluda {
    #[cfg(windows)]
    fn runtime() -> Runtime {
        let module = unsafe {
            zluda_windows::try_load_from_self_or_hip("amdhip64_7.dll")
                .or_else(|| zluda_windows::try_load_from_self_or_hip("amdhip64_6.dll"))
        }
        .unwrap();
        let library = unsafe { libloading::os::windows::Library::from_raw(module.0 as _) };
        Runtime {
            library: library.into(),
            kind: RuntimeKind::Hip,
        }
    }

    #[cfg(unix)]
    fn runtime() -> Runtime {
        Runtime {
            library: unsafe { libloading::Library::new("libamdhip64.so") }.unwrap(),
            kind: RuntimeKind::Hip,
        }
    }
}

macro_rules! api {
    ($($abi:literal fn $name:ident($($argument:ident: $type_:ty),*) -> $result:ty;)*) => {
        pub(crate) trait CufftApi {
            fn new() -> Self;
            fn runtime(&self) -> &Runtime;
            $(
                #[allow(non_snake_case)]
                fn $name(&self, $($argument: $type_),*) -> $result;
            )*
        }

        impl CufftApi for Zluda {
            fn new() -> Self {
                Self {
                    _gpu_test_guard: gpu_test_guard(),
                    runtime: Self::runtime(),
                }
            }

            fn runtime(&self) -> &Runtime {
                &self.runtime
            }

            $(
                fn $name(&self, $($argument: $type_),*) -> $result {
                    unsafe { crate::$name($($argument),*) }
                }
            )*
        }

        impl CufftApi for Cuda {
            fn new() -> Self {
                let gpu_test_guard = gpu_test_guard();
                let driver = unsafe { libloading::Library::new(Self::DRIVER_PATH) }.unwrap();
                let cufft = unsafe { libloading::Library::new(Self::cufft_path()) }.unwrap();
                let runtime = Runtime {
                    library: unsafe { libloading::Library::new(Self::runtime_path()) }.unwrap(),
                    kind: RuntimeKind::Cuda,
                };
                Self {
                    _gpu_test_guard: gpu_test_guard,
                    _driver: driver,
                    cufft,
                    runtime,
                }
            }

            fn runtime(&self) -> &Runtime {
                &self.runtime
            }

            $(
                fn $name(&self, $($argument: $type_),*) -> $result {
                    let function = unsafe {
                        self.cufft
                            .get::<unsafe extern $abi fn($($type_),*) -> $result>(
                                concat!(stringify!($name), "\0").as_bytes(),
                            )
                            .unwrap()
                    };
                    unsafe { function($($argument),*) }
                }
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
    use cuda_types::cufft::{
        cufftComplex, cufftResult, cufftResultConsts, cufftType, libraryPropertyType,
    };

    #[test_cuda]
    fn create_destroy_and_reject_stale_handle(api: impl CufftApi) {
        let mut handle = 0;
        assert_eq!(api.cufftCreate(&mut handle), cufftResult::SUCCESS);
        assert_ne!(handle, 0);
        assert_eq!(api.cufftDestroy(handle), cufftResult::SUCCESS);
        assert_eq!(api.cufftDestroy(handle), cufftResult::ERROR_INVALID_PLAN);
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
            let mut handle = 0;
            assert_eq!(
                api.cufftPlan3d(&mut handle, 3, 4, 5, type_),
                cufftResult::SUCCESS
            );
            assert_eq!(api.cufftDestroy(handle), cufftResult::SUCCESS);
        }
    }

    #[test_cuda]
    fn plan_lifecycle(api: impl CufftApi) {
        let mut handle = 0;
        assert_eq!(api.cufftCreate(&mut handle), cufftResult::SUCCESS);
        let stream = api.runtime().create_stream();
        assert_eq!(api.cufftSetStream(handle, stream), cufftResult::SUCCESS);
        assert_eq!(api.cufftSetAutoAllocation(handle, 0), cufftResult::SUCCESS);

        let mut work_size = 0;
        assert_eq!(
            api.cufftMakePlan3d(handle, 3, 4, 5, cufftType::CUFFT_R2C, &mut work_size),
            cufftResult::SUCCESS
        );
        let mut queried_work_size = 0;
        assert_eq!(
            api.cufftGetSize(handle, &mut queried_work_size),
            cufftResult::SUCCESS
        );
        assert_eq!(queried_work_size, work_size);
        let workspace = api.runtime().allocate(work_size.max(1));
        assert_eq!(
            api.cufftSetWorkArea(handle, workspace),
            cufftResult::SUCCESS
        );
        assert_eq!(api.cufftDestroy(handle), cufftResult::SUCCESS);
        api.runtime().free(workspace);
        api.runtime().destroy_stream(stream);
    }

    #[test_cuda]
    fn plan_many_64_bit(api: impl CufftApi) {
        let mut handle = 0;
        assert_eq!(api.cufftCreate(&mut handle), cufftResult::SUCCESS);
        let mut dimensions = [3_i64, 4, 5];
        let mut work_size = 0;
        assert_eq!(
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
            ),
            cufftResult::SUCCESS
        );
        assert_eq!(api.cufftDestroy(handle), cufftResult::SUCCESS);

        assert_eq!(api.cufftCreate(&mut handle), cufftResult::SUCCESS);
        let mut queried_work_size = 0;
        assert_eq!(
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
            ),
            cufftResult::SUCCESS
        );
        assert_eq!(api.cufftDestroy(handle), cufftResult::SUCCESS);
    }

    #[test_cuda]
    fn real_3d_execution(api: impl CufftApi) {
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
            .allocate(std::mem::size_of_val(input.as_slice()))
            .cast::<f32>();
        let device_spectrum = api
            .runtime()
            .allocate(COMPLEX_ELEMENTS * std::mem::size_of::<cufftComplex>())
            .cast::<cufftComplex>();
        let device_round_trip = api
            .runtime()
            .allocate(ELEMENTS * std::mem::size_of::<f32>())
            .cast::<f32>();
        api.runtime().copy_to_device(device_input, &input);

        let stream = api.runtime().create_stream();
        let mut forward = 0;
        assert_eq!(
            api.cufftPlan3d(
                &mut forward,
                NX as i32,
                NY as i32,
                NZ as i32,
                cufftType::CUFFT_R2C,
            ),
            cufftResult::SUCCESS
        );
        assert_eq!(api.cufftSetStream(forward, stream), cufftResult::SUCCESS);
        assert_eq!(
            api.cufftExecR2C(forward, device_input, device_spectrum),
            cufftResult::SUCCESS
        );
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

        let mut inverse = 0;
        assert_eq!(
            api.cufftPlan3d(
                &mut inverse,
                NX as i32,
                NY as i32,
                NZ as i32,
                cufftType::CUFFT_C2R,
            ),
            cufftResult::SUCCESS
        );
        assert_eq!(api.cufftSetStream(inverse, stream), cufftResult::SUCCESS);
        assert_eq!(
            api.cufftExecC2R(inverse, device_spectrum, device_round_trip),
            cufftResult::SUCCESS
        );
        api.runtime().synchronize(stream);

        let mut round_trip = vec![0.0; ELEMENTS];
        api.runtime()
            .copy_to_host(&mut round_trip, device_round_trip);
        for (actual, input) in round_trip.into_iter().zip(input) {
            let expected = input * ELEMENTS as f32;
            let tolerance = 2.0e-4 * expected.abs().max(1.0);
            assert!((actual - expected).abs() <= tolerance);
        }

        assert_eq!(api.cufftDestroy(inverse), cufftResult::SUCCESS);
        assert_eq!(api.cufftDestroy(forward), cufftResult::SUCCESS);
        api.runtime().destroy_stream(stream);
        api.runtime().free(device_round_trip.cast());
        api.runtime().free(device_spectrum.cast());
        api.runtime().free(device_input.cast());
    }

    #[test_cuda]
    fn version_and_properties(api: impl CufftApi) {
        let mut version = 0;
        assert_eq!(api.cufftGetVersion(&mut version), cufftResult::SUCCESS);
        assert!(version > 0);

        for property in [
            libraryPropertyType::MAJOR_VERSION,
            libraryPropertyType::MINOR_VERSION,
            libraryPropertyType::PATCH_LEVEL,
        ] {
            let mut value = -1;
            assert_eq!(
                api.cufftGetProperty(property, &mut value),
                cufftResult::SUCCESS
            );
            assert!(value >= 0);
        }
    }
}
