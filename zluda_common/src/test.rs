use std::ffi::{c_void, CStr};

pub struct Runtime {
    library: libloading::Library,
    kind: RuntimeKind,
}

enum RuntimeKind {
    Cuda,
    Hip,
}

impl Runtime {
    #[cfg(not(windows))]
    const HIP_PATH: &'static str = "libamdhip64.so.7";
    #[cfg(windows)]
    const HIP_PATH: &'static str = "amdhip64_7.dll";

    #[cfg(not(windows))]
    const CUDA_PATH: &'static str = "/usr/lib/x86_64-linux-gnu/libcuda.so.1";
    #[cfg(windows)]
    const CUDA_PATH: &'static str = "C:\\Windows\\System32\\nvcuda.dll";

    pub fn load_cuda() -> Self {
        let library = unsafe {
            libloading::Library::new(Self::CUDA_PATH).expect("Failed to load CUDA runtime")
        };
        Self {
            library,
            kind: RuntimeKind::Cuda,
        }
    }

    pub fn load_hip() -> Self {
        let library = unsafe {
            libloading::Library::new(Self::HIP_PATH)
                .expect("HIP should have been loaded successfully")
        };
        Self {
            library,
            kind: RuntimeKind::Hip,
        }
    }

    fn function<T: Copy>(&self, cuda: &CStr, hip: &CStr) -> T {
        let name = match self.kind {
            RuntimeKind::Cuda => cuda,
            RuntimeKind::Hip => hip,
        };
        unsafe { *self.library.get::<T>(name.to_bytes_with_nul()).unwrap() }
    }

    pub fn allocate(&self, size: usize) -> *mut c_void {
        let function = self.function::<unsafe extern "system" fn(*mut *mut c_void, usize) -> u32>(
            c"cuMemAlloc_v2",
            c"hipMalloc",
        );
        let mut pointer = std::ptr::null_mut();
        assert_eq!(unsafe { function(&mut pointer, size) }, 0);
        pointer
    }

    pub fn free(&self, pointer: *mut c_void) {
        let function = self
            .function::<unsafe extern "system" fn(*mut c_void) -> u32>(c"cuMemFree_v2", c"hipFree");
        assert_eq!(unsafe { function(pointer) }, 0);
    }

    pub fn init_context(&self) -> *mut c_void {
        let init = self.function::<unsafe extern "system" fn(u32) -> u32>(c"cuInit", c"hipInit");
        assert_eq!(unsafe { init(0) }, 0);
        let function = self
            .function::<unsafe extern "system" fn(*mut *mut c_void, u32, i32) -> u32>(
                c"cuCtxCreate_v2",
                c"hipCtxCreate",
            );
        let mut context = unsafe { std::mem::zeroed() };
        assert_eq!(unsafe { function(&mut context, 0, 0) }, 0);
        context
    }

    pub fn destroy_context(&self, context: *mut c_void) {
        let function = self.function::<unsafe extern "system" fn(*mut c_void) -> u32>(
            c"cuCtxDestroy_v2",
            c"hipCtxDestroy",
        );
        assert_eq!(unsafe { function(context) }, 0);
    }

    pub fn create_stream(&self) -> *mut c_void {
        let function = self.function::<unsafe extern "system" fn(*mut *mut c_void, u32) -> u32>(
            c"cuStreamCreate",
            c"hipStreamCreateWithFlags",
        );
        let mut stream = unsafe { std::mem::zeroed() };
        assert_eq!(unsafe { function(&mut stream, 0) }, 0);
        stream
    }

    pub fn destroy_stream(&self, stream: *mut c_void) {
        let function = self.function::<unsafe extern "system" fn(*mut c_void) -> u32>(
            c"cuStreamDestroy_v2",
            c"hipStreamDestroy",
        );
        assert_eq!(unsafe { function(stream) }, 0);
    }

    pub fn synchronize(&self, stream: *mut c_void) {
        let function = self.function::<unsafe extern "system" fn(*mut c_void) -> u32>(
            c"cuStreamSynchronize",
            c"hipStreamSynchronize",
        );
        assert_eq!(unsafe { function(stream) }, 0);
    }

    pub fn copy_to_device<T>(&self, device: *mut c_void, host: &[T]) {
        let function = self
            .function::<unsafe extern "system" fn(*mut c_void, *const c_void, usize) -> u32>(
                c"cuMemcpyHtoD_v2",
                c"hipMemcpyHtoD",
            );
        assert_eq!(
            unsafe {
                function(
                    device,
                    host.as_ptr().cast(),
                    std::mem::size_of::<T>() * host.len(),
                )
            },
            0
        );
    }

    pub fn copy_to_host<T>(&self, host: &mut [T], device: *mut c_void) {
        let function = self
            .function::<unsafe extern "system" fn(*mut c_void, *const c_void, usize) -> u32>(
                c"cuMemcpyDtoH_v2",
                c"hipMemcpyDtoH",
            );
        assert_eq!(
            unsafe {
                function(
                    host.as_mut_ptr().cast(),
                    device,
                    std::mem::size_of::<T>() * host.len(),
                )
            },
            0
        );
    }
}
