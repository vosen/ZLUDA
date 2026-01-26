use std::mem;

use bpaf::{construct, pure, Bpaf, Parser};
use owo_colors::{OwoColorize, Stream};
use rand::seq::SliceRandom;
use zluda_windows::LibraryInfo;

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
fn main() {
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

fn print_result(short_name: &str, lib: Result<(), Error>) {
    print!("{:<10}: ", short_name);
    match lib {
        Ok(()) => println!(
            "{}",
            "OK".if_supports_color(Stream::Stdout, |text| text.green())
        ),
        Err(err) => println!(
            "{}",
            format!("ERROR: {:?}", err).if_supports_color(Stream::Stdout, |text| text.red())
        ),
    }
}

unsafe fn try_load_library(lib: &LibraryInfo) -> Result<(), Error> {
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
        _ => Err(Error::Initialization(
            format!("Library check not implemented for {}", lib.short_name),
            0,
        )),
    }
}

unsafe fn check_cuda(library: libloading::Library) -> Result<(), Error> {
    let cu_init =
        library.get::<fn(::core::ffi::c_uint) -> cuda_types::cuda::CUresult>(b"cuInit\0")?;
    cu_init(0).map_err(|err| Error::Initialization("cuInit".to_string(), err.0.get() as usize))
}

unsafe fn check_nvml(library: libloading::Library) -> Result<(), Error> {
    use cuda_types::nvml::nvmlReturn_tConsts;
    let nvml_init = library.get::<fn() -> cuda_types::nvml::nvmlReturn_t>(b"nvmlInit_v2\0")?;
    match nvml_init() {
        Ok(()) => Ok(()),
        cuda_types::nvml::nvmlReturn_t::ERROR_NOT_SUPPORTED => Ok(()),
        Err(err) => Err(Error::Initialization(
            "nvmlInit_v2".to_string(),
            err.0.get() as usize,
        )),
    }
}

unsafe fn check_cudnn8(library: libloading::Library) -> Result<(), Error> {
    let cudnn_create = library.get::<fn(
        handle: *mut cuda_types::cudnn8::cudnnHandle_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t>(b"cudnnCreate\0")?;
    let mut handle = mem::zeroed();
    cudnn_create(&mut handle)
        .map_err(|err| Error::Initialization("cudnnCreate".to_string(), err.0.get() as usize))
}

unsafe fn check_cudnn9(library: libloading::Library) -> Result<(), Error> {
    let cudnn_create = library.get::<fn(
        handle: *mut cuda_types::cudnn9::cudnnHandle_t,
    ) -> cuda_types::cudnn9::cudnnStatus_t>(b"cudnnCreate\0")?;
    let mut handle = mem::zeroed();
    cudnn_create(&mut handle)
        .map_err(|err| Error::Initialization("cudnnCreate".to_string(), err.0.get() as usize))
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
