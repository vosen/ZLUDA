use bpaf::{construct, pure, Bpaf, Parser};
use owo_colors::{OwoColorize, Stream};
use rand::seq::SliceRandom;
use std::{ffi::OsString, mem};
use windows::Win32::Foundation::HMODULE;
use zluda_windows::{get_module_path_utf16, LibraryInfo};

#[derive(Debug, Bpaf)]
#[bpaf(options)]
struct Options {
    #[bpaf(external)]
    libraries: Vec<&'static LibraryInfo>,
    #[bpaf(switch)]
    driver_first: bool,
}

fn libraries() -> impl Parser<Vec<&'static LibraryInfo>> {
    zluda_windows::LIBRARIES.iter().fold(
        Parser::boxed(pure(Vec::with_capacity(zluda_windows::LIBRARIES.len()))),
        |parser, library| {
            let dlls = library.ascii_name;
            let arg = bpaf::long(library.short_name)
                .help(&*format!("Look for {dlls}"))
                .switch();
            construct!(parser, arg)
                .map(move |(mut acc, cur)| {
                    if cur {
                        acc.push(library);
                    }
                    acc
                })
                .boxed()
        },
    )
}

pub fn main() {
    let mut opts = options().run();
    if opts.libraries.is_empty() {
        opts.libraries = zluda_windows::LIBRARIES.iter().collect();
    }
    let mut lib_set = opts.libraries;
    if !opts.driver_first {
        lib_set.shuffle(&mut rand::rng());
    } else {
        let (_, remainder) = lib_set.split_first_mut().unwrap();
        remainder.shuffle(&mut rand::rng());
    }
    for lib in lib_set {
        print_result(lib.short_name, unsafe { try_load_library(lib) });
    }
}

fn print_result(short_name: &str, lib: Result<Option<OsString>, Error>) {
    print!("{:<10}: ", short_name);
    match lib {
        Ok(None) => println!(
            "{}",
            "OK".if_supports_color(Stream::Stdout, |text| text.green())
        ),
        Ok(Some(path)) => println!(
            "{} ({})",
            "OK".if_supports_color(Stream::Stdout, |text| text.green()),
            path.display()
        ),
        Err(err) => println!(
            "{}",
            format!("ERROR: {:?}", err).if_supports_color(Stream::Stdout, |text| text.red())
        ),
    }
}

unsafe fn try_load_library(lib: &LibraryInfo) -> Result<Option<OsString>, Error> {
    let library = if lib.in_system32 {
        libloading::Library::new(lib.ascii_name)?
    } else {
        match std::env::var("CUDA_PATH") {
            Ok(cuda_path) => {
                let path = std::path::Path::new(&cuda_path)
                    .join("bin")
                    .join("x64")
                    .join(lib.ascii_name);
                libloading::Library::new(path)?
            }
            Err(_) => libloading::Library::new(lib.ascii_name)?,
        }
    };
    match lib.short_name {
        "nvcuda" => check_cuda(library),
        "nvml" => check_nvml(library),
        "cudnn8" => check_cudnn8(library),
        "cudnn9" => check_cudnn9(library),
        "cublas12" => check_cublas(library),
        "cublas13" => check_cublas(library),
        "cublaslt12" => check_cublaslt(library),
        "cublaslt13" => check_cublaslt(library),
        "cusparse11" => check_cusparse(library),
        "cusparse12" => check_cusparse(library),
        "cufft11" => check_cufft(library),
        "cufft12" => check_cufft(library),
        _ => Err(Error::Initialization(
            format!("Library check not implemented for {}", lib.short_name),
            0,
        )),
    }
}

unsafe fn path_for_loaded_lib(lib: &'static str) -> Option<OsString> {
    let lib = libloading::os::windows::Library::open_already_loaded(lib).ok()?;
    let lib_handle = lib.into_raw();
    let path = get_module_path_utf16(HMODULE(lib_handle as _));
    libloading::os::windows::Library::from_raw(lib_handle);
    Some(path)
}

unsafe fn check_cufft(library: libloading::Library) -> Result<Option<OsString>, Error> {
    let hip_path = || path_for_loaded_lib("hipfft.dll");
    let cufft_create = library.get::<extern "system" fn(
        handle: *mut cuda_types::cufft::cufftHandle,
    ) -> cuda_types::cufft::cufftResult>(b"cufftCreate\0")?;
    let cufft_destroy = library.get::<extern "system" fn(
        handle: cuda_types::cufft::cufftHandle,
    ) -> cuda_types::cufft::cufftResult>(b"cufftDestroy\0")?;
    let mut handle = mem::zeroed();
    match cufft_create(&mut handle) {
        Ok(()) => {}
        Err(cuda_types::cufft::cufftError_t::NOT_SUPPORTED) => {
            return Ok(hip_path());
        }
        Err(err) => {
            return Err(Error::Initialization(
                "cufftCreate".to_string(),
                err.0.get() as usize,
            ));
        }
    }
    let result = hip_path();
    cufft_destroy(handle)
        .map_err(|err| Error::Initialization("cufftDestroy".to_string(), err.0.get() as usize))?;
    Ok(result)
}

unsafe fn check_cublas(library: libloading::Library) -> Result<Option<OsString>, Error> {
    let hip_path = || path_for_loaded_lib("rocblas.dll");
    let cublas_create = library.get::<extern "system" fn(
        handle: *mut cuda_types::cublas::cublasHandle_t,
    ) -> cuda_types::cublas::cublasStatus_t>(b"cublasCreate_v2\0")?;
    let cublas_destroy =
        library.get::<extern "system" fn(
            handle: cuda_types::cublas::cublasHandle_t,
        ) -> cuda_types::cublas::cublasStatus_t>(b"cublasDestroy_v2\0")?;
    let mut handle = mem::zeroed();
    cublas_create(&mut handle).map_err(|err| {
        Error::Initialization("cublasCreate_v2".to_string(), err.0.get() as usize)
    })?;
    let result = hip_path();
    cublas_destroy(handle).map_err(|err| {
        Error::Initialization("cublasDestroy_v2".to_string(), err.0.get() as usize)
    })?;
    Ok(result)
}

unsafe fn check_cusparse(library: libloading::Library) -> Result<Option<OsString>, Error> {
    let hip_path = || path_for_loaded_lib("rocsparse.dll");
    let cusparse_create =
        library.get::<extern "system" fn(
            handle: *mut cuda_types::cusparse::cusparseHandle_t,
        ) -> cuda_types::cusparse::cusparseStatus_t>(b"cusparseCreate\0")?;
    let cusparse_destroy =
        library.get::<extern "system" fn(
            handle: cuda_types::cusparse::cusparseHandle_t,
        ) -> cuda_types::cusparse::cusparseStatus_t>(b"cusparseDestroy\0")?;
    let mut handle = mem::zeroed();
    match cusparse_create(&mut handle) {
        Ok(()) => {}
        Err(cuda_types::cusparse::cusparseError_t::NOT_SUPPORTED) => {
            return Ok(hip_path());
        }
        Err(err) => {
            return Err(Error::Initialization(
                "cusparseCreate".to_string(),
                err.0.get() as usize,
            ));
        }
    }
    let result = hip_path();
    cusparse_destroy(handle).map_err(|err| {
        Error::Initialization("cusparseDestroy".to_string(), err.0.get() as usize)
    })?;
    Ok(result)
}

unsafe fn check_cublaslt(library: libloading::Library) -> Result<Option<OsString>, Error> {
    let hip_path =
        || path_for_loaded_lib("hipblaslt.dll").or_else(|| path_for_loaded_lib("libhipblaslt.dll"));
    let cublaslt_create =
        library.get::<extern "system" fn(
            handle: *mut cuda_types::cublaslt::cublasLtHandle_t,
        ) -> cuda_types::cublas::cublasStatus_t>(b"cublasLtCreate\0")?;
    let cublaslt_destroy =
        library.get::<extern "system" fn(
            handle: cuda_types::cublaslt::cublasLtHandle_t,
        ) -> cuda_types::cublas::cublasStatus_t>(b"cublasLtDestroy\0")?;
    let mut handle = mem::zeroed();
    cublaslt_create(&mut handle)
        .map_err(|err| Error::Initialization("cublasLtCreate".to_string(), err.0.get() as usize))?;
    let result = hip_path();
    cublaslt_destroy(handle).map_err(|err| {
        Error::Initialization("cublasLtDestroy".to_string(), err.0.get() as usize)
    })?;
    Ok(result)
}

unsafe fn check_cuda(library: libloading::Library) -> Result<Option<OsString>, Error> {
    let cu_init = library
        .get::<extern "system" fn(::core::ffi::c_uint) -> cuda_types::cuda::CUresult>(
            b"cuInit\0",
        )?;
    cu_init(0).map_err(|err| Error::Initialization("cuInit".to_string(), err.0.get() as usize))?;
    Ok(path_for_loaded_lib("amdhip64_7.dll").or_else(|| path_for_loaded_lib("amdhip64_6.dll")))
}

unsafe fn check_nvml(library: libloading::Library) -> Result<Option<OsString>, Error> {
    use cuda_types::nvml::nvmlReturn_tConsts;
    let nvml_init =
        library.get::<extern "system" fn() -> cuda_types::nvml::nvmlReturn_t>(b"nvmlInit_v2\0")?;
    match nvml_init() {
        Ok(()) | cuda_types::nvml::nvmlReturn_t::ERROR_NOT_SUPPORTED => {
            Ok(path_for_loaded_lib("rocm_smi64.dll"))
        }
        Err(err) => Err(Error::Initialization(
            "nvmlInit_v2".to_string(),
            err.0.get() as usize,
        )),
    }
}

unsafe fn check_cudnn8(library: libloading::Library) -> Result<Option<OsString>, Error> {
    let hip_path = || path_for_loaded_lib("MIOpen.dll");
    let cudnn_create = library.get::<extern "system" fn(
        handle: *mut cuda_types::cudnn8::cudnnHandle_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t>(b"cudnnCreate\0")?;
    let cudnn_destroy = library.get::<extern "system" fn(
        handle: cuda_types::cudnn8::cudnnHandle_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t>(b"cudnnDestroy\0")?;
    let mut handle = mem::zeroed();
    cudnn_create(&mut handle)
        .map_err(|err| Error::Initialization("cudnnCreate".to_string(), err.0.get() as usize))?;
    let result = hip_path();
    cudnn_destroy(handle)
        .map_err(|err| Error::Initialization("cudnnDestroy".to_string(), err.0.get() as usize))?;
    Ok(result)
}

unsafe fn check_cudnn9(library: libloading::Library) -> Result<Option<OsString>, Error> {
    let hip_path = || path_for_loaded_lib("MIOpen.dll");
    let cudnn_create = library.get::<extern "system" fn(
        handle: *mut cuda_types::cudnn9::cudnnHandle_t,
    ) -> cuda_types::cudnn9::cudnnStatus_t>(b"cudnnCreate\0")?;
    let cudnn_destroy = library.get::<extern "system" fn(
        handle: cuda_types::cudnn9::cudnnHandle_t,
    ) -> cuda_types::cudnn9::cudnnStatus_t>(b"cudnnDestroy\0")?;
    let mut handle = mem::zeroed();
    cudnn_create(&mut handle)
        .map_err(|err| Error::Initialization("cudnnCreate".to_string(), err.0.get() as usize))?;
    let result = hip_path();
    cudnn_destroy(handle)
        .map_err(|err| Error::Initialization("cudnnDestroy".to_string(), err.0.get() as usize))?;
    Ok(result)
}

#[derive(Debug)]
#[allow(dead_code)]
enum Error {
    Loading(libloading::Error),
    Initialization(String, usize),
}

impl From<libloading::Error> for Error {
    fn from(err: libloading::Error) -> Self {
        Error::Loading(err)
    }
}
