use std::{
    collections::HashMap, env, error::Error, ffi::c_void, fs, io::prelude::*, os::raw::c_uint,
    path::PathBuf, slice,
};
use std::{fs::File, ptr};

use cuda::{CUdeviceptr, CUfunction, CUjit_option, CUmodule, CUresult, CUstream};
use ptx::ast;

#[cfg_attr(windows, path = "os_win.rs")]
#[cfg_attr(not(windows), path = "os_unix.rs")]
mod os;

macro_rules! extern_redirect {
    (pub fn $fn_name:ident ( $($arg_id:ident: $arg_type:ty),* $(,)? ) -> $ret_type:ty ;) => {
        #[no_mangle]
        pub fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
            unsafe { $crate::init_libcuda_handle() };
            let name = std::ffi::CString::new(stringify!($fn_name)).unwrap();
            let fn_ptr = unsafe { crate::os::get_proc_address($crate::LIBCUDA_HANDLE, &name) };
            if fn_ptr == std::ptr::null_mut() {
                return CUresult::CUDA_ERROR_UNKNOWN;
            }
            let typed_fn = unsafe { std::mem::transmute::<_, fn( $( $arg_id : $arg_type),* ) -> $ret_type>(fn_ptr) };
            typed_fn($( $arg_id ),*)
        }
    };
}

macro_rules! extern_redirect_with {
    (
        pub fn $fn_name:ident ( $($arg_id:ident: $arg_type:ty),* $(,)? ) -> $ret_type:ty ;
        $receiver:path ;
    ) => {
        #[no_mangle]
        pub fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
            unsafe { $crate::init_libcuda_handle() };
            let continuation = |$( $arg_id : $arg_type),* | {
                let name = std::ffi::CString::new(stringify!($fn_name)).unwrap();
                let fn_ptr = unsafe { crate::os::get_proc_address($crate::LIBCUDA_HANDLE, &name) };
                if fn_ptr == std::ptr::null_mut() {
                    return CUresult::CUDA_ERROR_UNKNOWN;
                }
                let typed_fn = unsafe { std::mem::transmute::<_, fn( $( $arg_id : $arg_type),* ) -> $ret_type>(fn_ptr) };
                typed_fn($( $arg_id ),*)
            };
            unsafe { $receiver($( $arg_id ),* , continuation) }
        }
    };
}

#[allow(warnings)]
mod cuda;

pub static mut LIBCUDA_HANDLE: *mut c_void = ptr::null_mut();
pub static mut MODULES: Option<HashMap<CUmodule, HashMap<String, Vec<usize>>>> = None;
pub static mut KERNEL_ARGS: Option<HashMap<CUfunction, (String, Vec<usize>)>> = None;
pub static mut BUFFERS: Vec<(usize, usize)> = Vec::new();
pub static mut LAUNCH_COUNTER: usize = 0;

// We are doing dlopen here instead of just using LD_PRELOAD,
// it's because CUDA Runtime API does dlopen to open libcuda.so, which ignores LD_PRELOAD
pub unsafe fn init_libcuda_handle() {
    if LIBCUDA_HANDLE == ptr::null_mut() {
        let libcuda_handle = os::load_cuda_library();
        assert_ne!(libcuda_handle, ptr::null_mut());
        LIBCUDA_HANDLE = libcuda_handle;
        eprintln!("[ZLUDA_DUMP] Initialized");
    }
}

#[allow(non_snake_case)]
pub unsafe fn cuModuleLoadData(
    module: *mut CUmodule,
    image: *const ::std::os::raw::c_void,
    cont: impl FnOnce(*mut CUmodule, *const c_void) -> CUresult,
) -> CUresult {
    let result = cont(module, image);
    if result == CUresult::CUDA_SUCCESS {
        record_module_image(module, image);
    }
    result
}

unsafe fn record_module_image(module: *mut CUmodule, raw_image: *const ::std::os::raw::c_void) {
    let image = to_str(raw_image);
    match image {
        None => eprintln!("[ZLUDA_DUMP] Unrecognized module image at {:?}", raw_image),
        Some(image) => {
            if !image.contains(&".address_size") {
                eprintln!(
                    "[ZLUDA_DUMP] Unrecognized module image at {:?}:\n{}",
                    raw_image, image
                )
            } else {
                let mut errors = Vec::new();
                let ast = ptx::ModuleParser::new().parse(&mut errors, image);
                match (&*errors, ast) {
                    (&[], Ok(ast)) => {
                        let kernels = ast
                            .directives
                            .iter()
                            .filter_map(directive_to_kernel)
                            .collect::<HashMap<_, _>>();
                        let modules = MODULES.get_or_insert_with(|| HashMap::new());
                        modules.insert(*module, kernels);
                    }
                    (errs, ast) => {
                        let err_string = errs
                            .iter()
                            .map(|e| format!("{:?}", e))
                            .chain(ast.err().iter().map(|e| format!("{:?}", e)))
                            .collect::<Vec<_>>()
                            .join("\n");
                        eprintln!(
                            "[ZLUDA_DUMP] Errors when parsing module:\n---ERRORS---\n{}\n---MODULE---\n{}",
                            err_string, image
                        );
                    }
                }
            }
        }
    }
}

unsafe fn to_str<T>(image: *const T) -> Option<&'static str> {
    let ptr = image as *const u8;
    let mut offset = 0;
    loop {
        let c = *ptr.add(offset);
        if !c.is_ascii() {
            return None;
        }
        if c == 0 {
            return Some(std::str::from_utf8_unchecked(slice::from_raw_parts(
                ptr, offset,
            )));
        }
        offset += 1;
    }
}

fn directive_to_kernel(dir: &ast::Directive<ast::ParsedArgParams>) -> Option<(String, Vec<usize>)> {
    match dir {
        ast::Directive::Method(ast::Function {
            func_directive: ast::MethodDecl::Kernel { name, in_args },
            ..
        }) => {
            let arg_sizes = in_args
                .iter()
                .map(|arg| ast::Type::from(arg.v_type.clone()).size_of())
                .collect();
            Some((name.to_string(), arg_sizes))
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
pub unsafe fn cuModuleLoadDataEx(
    module: *mut CUmodule,
    image: *const c_void,
    numOptions: c_uint,
    options: *mut CUjit_option,
    optionValues: *mut *mut c_void,
    cont: impl FnOnce(
        *mut CUmodule,
        *const c_void,
        c_uint,
        *mut CUjit_option,
        *mut *mut c_void,
    ) -> CUresult,
) -> CUresult {
    let result = cont(module, image, numOptions, options, optionValues);
    if result == CUresult::CUDA_SUCCESS {
        record_module_image(module, image);
    }
    result
}

#[allow(non_snake_case)]
unsafe fn cuModuleGetFunction(
    hfunc: *mut CUfunction,
    hmod: CUmodule,
    name: *const ::std::os::raw::c_char,
    cont: impl FnOnce(*mut CUfunction, CUmodule, *const ::std::os::raw::c_char) -> CUresult,
) -> CUresult {
    let result = cont(hfunc, hmod, name);
    if result != CUresult::CUDA_SUCCESS {
        return result;
    }
    if let Some(modules) = &MODULES {
        if let Some(kernels) = modules.get(&hmod) {
            if let Some(kernel) = to_str(name) {
                if let Some(args) = kernels.get(kernel) {
                    let kernel_args = KERNEL_ARGS.get_or_insert_with(|| HashMap::new());
                    kernel_args.insert(*hfunc, (kernel.to_string(), args.clone()));
                } else {
                    eprintln!("[ZLUDA_DUMP] Could not find kernel {}", kernel);
                }
            } else {
                eprintln!("[ZLUDA_DUMP] Unrecognized kernel name at {:?}", hfunc);
            }
        } else {
            eprintln!("[ZLUDA_DUMP] No module recorded: {:?}", hmod);
        }
    } else {
        eprintln!("[ZLUDA_DUMP] No module recorded: {:?}", hmod);
    }
    CUresult::CUDA_SUCCESS
}

#[allow(non_snake_case)]
pub unsafe fn cuMemAlloc_v2(
    dptr: *mut CUdeviceptr,
    bytesize: usize,
    cont: impl FnOnce(*mut CUdeviceptr, usize) -> CUresult,
) -> CUresult {
    let result = cont(dptr, bytesize);
    assert_eq!(result, CUresult::CUDA_SUCCESS);
    let start = (*dptr).0 as usize;
    BUFFERS.push((start, bytesize));
    CUresult::CUDA_SUCCESS
}

#[allow(non_snake_case)]
pub unsafe fn cuLaunchKernel(
    f: CUfunction,
    gridDimX: ::std::os::raw::c_uint,
    gridDimY: ::std::os::raw::c_uint,
    gridDimZ: ::std::os::raw::c_uint,
    blockDimX: ::std::os::raw::c_uint,
    blockDimY: ::std::os::raw::c_uint,
    blockDimZ: ::std::os::raw::c_uint,
    sharedMemBytes: ::std::os::raw::c_uint,
    hStream: CUstream,
    kernelParams: *mut *mut ::std::os::raw::c_void,
    extra: *mut *mut ::std::os::raw::c_void,
    cont: impl FnOnce(
        CUfunction,
        ::std::os::raw::c_uint,
        ::std::os::raw::c_uint,
        ::std::os::raw::c_uint,
        ::std::os::raw::c_uint,
        ::std::os::raw::c_uint,
        ::std::os::raw::c_uint,
        ::std::os::raw::c_uint,
        CUstream,
        *mut *mut ::std::os::raw::c_void,
        *mut *mut ::std::os::raw::c_void,
    ) -> CUresult,
) -> CUresult {
    let mut error;
    let maybe_kernel_args = KERNEL_ARGS.as_ref().and_then(|kernels| kernels.get(&f));
    match maybe_kernel_args {
        Some((kernel_name, kernel_args)) => dump_arguments(
            kernelParams,
            "pre",
            &kernel_name,
            LAUNCH_COUNTER,
            kernel_args,
        )
        .unwrap_or_else(|err| eprintln!("[ZLUDA_DUMP] {:#?}", err)),
        None => eprintln!("[ZLUDA_DUMP] Unknown kernel {:?}", f),
    }
    error = cont(
        f,
        gridDimX,
        gridDimY,
        gridDimZ,
        blockDimX,
        blockDimY,
        blockDimZ,
        sharedMemBytes,
        hStream,
        kernelParams,
        extra,
    );
    assert_eq!(error, CUresult::CUDA_SUCCESS);
    error = cuda::cuStreamSynchronize(hStream);
    assert_eq!(error, CUresult::CUDA_SUCCESS);
    if let Some((kernel_name, kernel_args)) = maybe_kernel_args {
        dump_arguments(
            kernelParams,
            "post",
            &kernel_name,
            LAUNCH_COUNTER,
            kernel_args,
        )
        .unwrap_or_else(|err| eprintln!("[ZLUDA_DUMP] {:#?}", err));
    }
    CUresult::CUDA_SUCCESS
}

unsafe fn dump_arguments(
    kernel_params: *mut *mut ::std::os::raw::c_void,
    prefix: &str,
    kernel: &str,
    counter: usize,
    args: &[usize],
) -> Result<(), Box<dyn Error>> {
    let mut dump_dir = get_dump_dir()?;
    dump_dir.push(format!("{:04}_{}", counter, kernel));
    dump_dir.push(prefix);
    if dump_dir.exists() {
        fs::remove_dir_all(&dump_dir)?;
    }
    fs::create_dir_all(&dump_dir)?;
    for (i, arg_len) in args.iter().enumerate() {
        let dev_ptr = *(*kernel_params.add(i) as *mut usize);
        match BUFFERS.iter().find(|(start, _)| *start == dev_ptr as usize) {
            Some((start, len)) => {
                let mut output = vec![0u8; *len];
                let error = cuda::cuMemcpyDtoH_v2(
                    output.as_mut_ptr() as *mut _,
                    CUdeviceptr(*start),
                    *len,
                );
                assert_eq!(error, CUresult::CUDA_SUCCESS);
                let mut path = dump_dir.clone();
                path.push(format!("arg_{:03}.buffer", i));
                let mut file = File::create(path)?;
                file.write_all(&mut output)?;
            }
            None => {
                let mut path = dump_dir.clone();
                path.push(format!("arg_{:03}", i));
                let mut file = File::create(path)?;
                file.write_all(slice::from_raw_parts(
                    *kernel_params.add(i) as *mut u8,
                    *arg_len,
                ))?;
            }
        }
    }
    Ok(())
}

fn get_dump_dir() -> Result<PathBuf, Box<dyn Error>> {
    let dir = env::var("ZLUDA_DUMP_DIR")?;
    let mut main_dir = PathBuf::from(dir);
    let current_exe = env::current_exe()?;
    main_dir.push(current_exe.file_name().unwrap());
    fs::create_dir_all(&main_dir)?;
    Ok(main_dir)
}
