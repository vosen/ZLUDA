use std::mem;

use cuda_types::cuda::{CUdeviceptr_v2, CUstream};
use rustc_hash::FxHashMap;

struct CudaDynamicFns {
    handle: libloading::Library,
}

impl CudaDynamicFns {
    unsafe fn new(path: &str) -> Result<Self, libloading::Error> {
        let handle = libloading::Library::new(path)?;
        Ok(Self { handle })
    }
}

macro_rules! emit_cuda_fn_table {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        impl CudaDynamicFns {
            $(
                #[allow(dead_code)]
                unsafe fn $fn_name(&self, $($arg_id : $arg_type),*) -> $ret_type {
                    let func = self.handle.get::<unsafe extern $abi fn ($($arg_type),*) -> $ret_type>(concat!(stringify!($fn_name), "\0").as_bytes());
                    (func.unwrap())($($arg_id),*)
                }
            )*
        }
    };
}

cuda_macros::cuda_function_declarations!(emit_cuda_fn_table);

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let libcuda = unsafe { CudaDynamicFns::new(&args[1]).unwrap() };
    unsafe { libcuda.cuInit(0) }.unwrap();
    unsafe { libcuda.cuCtxCreate_v2(&mut mem::zeroed(), 0, 0) }.unwrap();
    let reader = std::fs::File::open(&args[2]).unwrap();
    let (mut manifest, mut source, mut buffers) = zluda_trace_common::replay::load(reader);
    let mut outputs = FxHashMap::default();
    let mut args = manifest
        .parameters
        .iter()
        .enumerate()
        .map(|(i, param)| {
            let mut buffer = buffers.remove(&format!("param_{i}.bin")).unwrap();
            for param_ptr in param.pointer_offsets.iter() {
                let buffer_param_slice = &mut buffer[param_ptr.offset_in_param
                    ..param_ptr.offset_in_param + std::mem::size_of::<usize>()];
                let mut dev_ptr = unsafe { mem::zeroed() };
                let host_buffer = buffers
                    .remove(&format!(
                        "param_{i}_ptr_{}_pre.bin",
                        param_ptr.offset_in_param
                    ))
                    .unwrap();
                unsafe { libcuda.cuMemAlloc_v2(&mut dev_ptr, host_buffer.len()) }.unwrap();
                outputs.insert(
                    format!("param_{i}_ptr_{}_post.bin", param_ptr.offset_in_param),
                    (dev_ptr, host_buffer.len()),
                );
                unsafe {
                    libcuda.cuMemcpyHtoD_v2(dev_ptr, host_buffer.as_ptr().cast(), host_buffer.len())
                }
                .unwrap();
                dev_ptr = CUdeviceptr_v2(unsafe {
                    dev_ptr
                        .0
                        .cast::<u8>()
                        .add(param_ptr.offset_in_buffer)
                        .cast()
                });
                buffer_param_slice.copy_from_slice(&(dev_ptr.0 as usize).to_ne_bytes());
            }
            buffer
        })
        .collect::<Vec<_>>();
    let mut module = unsafe { mem::zeroed() };
    std::fs::write("/tmp/source.ptx", &source).unwrap();
    source.push('\0');
    unsafe { libcuda.cuModuleLoadData(&mut module, source.as_ptr().cast()) }.unwrap();
    let mut function = unsafe { mem::zeroed() };
    manifest.kernel_name.push('\0');
    unsafe {
        libcuda.cuModuleGetFunction(&mut function, module, manifest.kernel_name.as_ptr().cast())
    }
    .unwrap();
    let mut cuda_args = args
        .iter_mut()
        .map(|arg| arg.as_mut_ptr().cast::<std::ffi::c_void>())
        .collect::<Vec<_>>();
    unsafe {
        libcuda.cuLaunchKernel(
            function,
            manifest.config.grid_dim.0,
            manifest.config.grid_dim.1,
            manifest.config.grid_dim.2,
            manifest.config.block_dim.0,
            manifest.config.block_dim.1,
            manifest.config.block_dim.2,
            manifest.config.shared_mem_bytes,
            CUstream(std::ptr::null_mut()),
            cuda_args.as_mut_ptr().cast(),
            std::ptr::null_mut(),
        )
    }
    .unwrap();
    unsafe { libcuda.cuCtxSynchronize() }.unwrap();
    for (path, (dev_ptr, len)) in outputs {
        let mut host_buffer = vec![0u8; len];
        unsafe { libcuda.cuMemcpyDtoH_v2(host_buffer.as_mut_ptr().cast(), dev_ptr, len) }.unwrap();
        std::fs::write(path, host_buffer).unwrap();
    }
}
