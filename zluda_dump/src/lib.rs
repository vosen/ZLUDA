use std::{
    collections::{hash_map, BTreeMap, HashMap},
    env,
    error::Error,
    ffi::{c_void, CStr},
    fs,
    io::{self, prelude::*},
    mem,
    os::raw::{c_int, c_uint, c_ulong, c_ushort},
    path::PathBuf,
    ptr::NonNull,
    rc::Rc,
    slice,
    sync::Mutex,
};
use std::{fs::File, ptr};

use cuda::{
    CUdevice, CUdevice_attribute, CUdeviceptr, CUfunction, CUjitInputType, CUjit_option,
    CUlinkState, CUmodule, CUresult, CUstream, CUuuid,
};
use ptx::ast;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

const CU_LAUNCH_PARAM_END: *mut c_void = 0 as *mut _;
const CU_LAUNCH_PARAM_BUFFER_POINTER: *mut c_void = 1 as *mut _;
const CU_LAUNCH_PARAM_BUFFER_SIZE: *mut c_void = 2 as *mut _;

macro_rules! extern_redirect {
    (pub fn $fn_name:ident ( $($arg_id:ident: $arg_type:ty),* $(,)? ) -> $ret_type:ty ;) => {
        #[no_mangle]
        pub extern "system" fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
            let original_fn = |fn_ptr| {
                let typed_fn = unsafe { std::mem::transmute::<_, extern "system" fn( $( $arg_id : $arg_type),* ) -> $ret_type>(fn_ptr) };
                typed_fn($( $arg_id ),*)
            };
            crate::handle_cuda_function_call(stringify!($fn_name), original_fn)
        }
    };
}

macro_rules! extern_redirect_with_post {
    (
        pub fn $fn_name:ident ( $($arg_id:ident: $arg_type:ty),* $(,)? ) -> $ret_type:ty ;
        $post_fn:path ;
    ) => {
        #[no_mangle]
        pub extern "system" fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
            let original_fn = |fn_ptr| {
                let typed_fn = unsafe { std::mem::transmute::<_, extern "system" fn( $( $arg_id : $arg_type),* ) -> $ret_type>(fn_ptr) };
                typed_fn($( $arg_id ),*)
            };
            crate::handle_cuda_function_call_with_probes(
                stringify!($fn_name),
                || (), original_fn,
                move |logger, state, _, cuda_result| $post_fn ( $( $arg_id ),* , logger, state, cuda_result )
            )
        }
    };
}

macro_rules! extern_redirect_with {
    (
        pub fn $fn_name:ident ( $($arg_id:ident: $arg_type:ty),* $(,)? ) -> $ret_type:ty ;
        $receiver:path ;
    ) => {
        #[no_mangle]
        pub extern "system" fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
            let original_fn = |fn_ptr| {
                let typed_fn = unsafe { std::mem::transmute::<_, extern "system" fn( $( $arg_id : $arg_type),* ) -> $ret_type>(fn_ptr) };
                typed_fn($( $arg_id ),*)
            };
            crate::handle_cuda_function_call(stringify!($fn_name), original_fn)
        }
    };
}

#[allow(warnings)]
mod cuda;
mod log;
#[cfg_attr(windows, path = "os_win.rs")]
#[cfg_attr(not(windows), path = "os_unix.rs")]
mod os;
mod trace;

pub static mut LIBCUDA_HANDLE: *mut c_void = ptr::null_mut();
pub static mut PENDING_LINKING: Option<HashMap<CUlinkState, Vec<ModuleDump>>> = None;
pub static mut LINKED_CUBINS: Option<HashMap<*mut c_void, ModuleDump>> = None;
pub static mut MODULES: Option<HashMap<CUmodule, ModuleDump>> = None;
pub static mut MODULE_DUMP_COUNTER: usize = 0;
pub static mut KERNELS: Option<HashMap<CUfunction, KernelDump>> = None;
static mut BUFFERS: Option<BTreeMap<usize, (usize, AllocLocation)>> = None;
pub static mut LAUNCH_COUNTER: usize = 0;
pub static mut KERNEL_PATTERN: Option<Regex> = None;
pub static mut OVERRIDE_COMPUTE_CAPABILITY_MAJOR: Option<i32> = None;
pub static mut KERNEL_INDEX_MINIMUM: usize = 0;
pub static mut KERNEL_INDEX_MAXIMUM: usize = usize::MAX;
static mut LOG_FACTORY: Option<log::Factory> = None;

lazy_static! {
    static ref GLOBAL_STATE: Mutex<GlobalState> = Mutex::new(GlobalState::new());
}

struct GlobalState {
    log_factory: log::Factory,
    // We split off fields that require a mutable reference to log factory to be
    // created, additionally creation of some fields in this struct can fail
    // initalization (e.g. we passed path a non-existant path to libcuda)
    delayed_state: LateInit<GlobalDelayedState>,
}

unsafe impl Send for GlobalState {}

impl GlobalState {
    fn new() -> Self {
        GlobalState {
            log_factory: log::Factory::new(),
            delayed_state: LateInit::Unitialized,
        }
    }
}

enum LateInit<T> {
    Success(T),
    Unitialized,
    Error,
}

impl<T> LateInit<T> {
    fn as_mut(&mut self) -> Option<&mut T> {
        match self {
            LateInit::Success(t) => Some(t),
            LateInit::Unitialized => None,
            LateInit::Error => None,
        }
    }
}

struct GlobalDelayedState {
    settings: Settings,
    libcuda_handle: NonNull<c_void>,
    cuda_state: trace::StateTracker,
}

impl GlobalDelayedState {
    fn new<'a>(
        func: &'static str,
        factory: &'a mut log::Factory,
    ) -> (LateInit<Self>, log::FunctionLogger<'a>) {
        let (mut fn_logger, settings) = factory.get_first_logger_and_init_settings(func);
        let maybe_libcuda_handle = unsafe { os::load_cuda_library(&settings.libcuda_path) };
        let libcuda_handle = match NonNull::new(maybe_libcuda_handle) {
            Some(h) => h,
            None => {
                fn_logger.log(log::LogEntry::ErrorBox(
                    format!("Invalid CUDA library at path {}", &settings.libcuda_path).into(),
                ));
                return (LateInit::Error, fn_logger);
            }
        };
        let cuda_state = trace::StateTracker::new(&settings);
        let delayed_state = GlobalDelayedState {
            settings,
            libcuda_handle,
            cuda_state,
        };
        (LateInit::Success(delayed_state), fn_logger)
    }
}

struct Settings {
    dump_dir: Option<PathBuf>,
    libcuda_path: String,
}

impl Settings {
    fn read_and_init(logger: &mut log::FunctionLogger) -> Self {
        let maybe_dump_dir = Self::read_and_init_dump_dir();
        let dump_dir = match maybe_dump_dir {
            Ok(d) => d,
            Err(err) => {
                logger.log(log::LogEntry::ErrorBox(err));
                None
            }
        };
        let libcuda_path = match env::var("ZLUDA_DUMP_LIBCUDA_FILE") {
            Err(env::VarError::NotPresent) => os::LIBCUDA_DEFAULT_PATH.to_owned(),
            Err(e) => {
                logger.log(log::LogEntry::ErrorBox(Box::new(e) as _));
                os::LIBCUDA_DEFAULT_PATH.to_owned()
            }
            Ok(env_string) => env_string,
        };
        Settings {
            dump_dir,
            libcuda_path,
        }
    }

    fn read_and_init_dump_dir() -> Result<Option<PathBuf>, Box<dyn Error>> {
        let dir = match env::var("ZLUDA_DUMP_DIR") {
            Ok(dir) => dir,
            Err(env::VarError::NotPresent) => return Ok(None),
            Err(err) => return Err(Box::new(err) as Box<_>),
        };
        Ok(Some(Self::create_dump_directory(dir)?))
    }

    fn create_dump_directory(dir: String) -> io::Result<PathBuf> {
        let mut main_dir = PathBuf::from(dir);
        let current_exe = env::current_exe()?;
        main_dir.push(current_exe.file_name().unwrap());
        fs::create_dir_all(&main_dir)?;
        Ok(main_dir)
    }
}

pub struct ModuleDump {
    content: Rc<String>,
    kernels_args: Option<HashMap<String, Vec<usize>>>,
}

fn handle_cuda_function_call(
    func: &'static str,
    original_cuda_fn: impl FnOnce(NonNull<c_void>) -> CUresult,
) -> CUresult {
    let global_state_mutex = &*GLOBAL_STATE;
    // We unwrap because there's really no sensible thing we could do,
    // alternatively we could return a CUDA error, but I think it's fine to
    // crash. This is a diagnostic utility, if the lock was poisoned we can't
    // extract any useful trace or logging anyway
    let mut global_state = &mut *global_state_mutex.lock().unwrap();
    let (mut logger, delayed_state) = match global_state.delayed_state {
        LateInit::Success(ref mut delayed_state) => {
            (global_state.log_factory.get_logger(func), delayed_state)
        }
        // There's no libcuda to load, so we might as well panic
        LateInit::Error => panic!(),
        LateInit::Unitialized => {
            let (new_delayed_state, logger) =
                GlobalDelayedState::new(func, &mut global_state.log_factory);
            global_state.delayed_state = new_delayed_state;
            (logger, global_state.delayed_state.as_mut().unwrap())
        }
    };
    let name = std::ffi::CString::new(func).unwrap();
    let fn_ptr =
        unsafe { os::get_proc_address(delayed_state.libcuda_handle.as_ptr(), name.as_c_str()) };
    let cu_result = original_cuda_fn(NonNull::new(fn_ptr).unwrap());
    logger.result = Some(cu_result);
    cu_result
}

fn handle_cuda_function_call_with_probes<T, PostFn>(
    func: &'static str,
    pre_probe: impl FnOnce() -> T,
    original_cuda_fn: impl FnOnce(NonNull<c_void>) -> CUresult,
    post_probe: PostFn,
) -> CUresult
where
    for<'a> PostFn: FnOnce(&'a mut log::FunctionLogger, &'a mut trace::StateTracker, T, CUresult),
{
    let global_state_mutex = &*GLOBAL_STATE;
    // We unwrap because there's really no sensible thing we could do,
    // alternatively we could return a CUDA error, but I think it's fine to
    // crash. This is a diagnostic utility, if the lock was poisoned we can't
    // extract any useful trace or logging anyway
    let mut global_state = &mut *global_state_mutex.lock().unwrap();
    let (mut logger, delayed_state) = match global_state.delayed_state {
        LateInit::Success(ref mut delayed_state) => {
            (global_state.log_factory.get_logger(func), delayed_state)
        }
        // There's no libcuda to load, so we might as well panic
        LateInit::Error => panic!(),
        LateInit::Unitialized => {
            let (new_delayed_state, logger) =
                GlobalDelayedState::new(func, &mut global_state.log_factory);
            global_state.delayed_state = new_delayed_state;
            (logger, global_state.delayed_state.as_mut().unwrap())
        }
    };
    let name = std::ffi::CString::new(func).unwrap();
    let fn_ptr =
        unsafe { os::get_proc_address(delayed_state.libcuda_handle.as_ptr(), name.as_c_str()) };
    let fn_ptr = NonNull::new(fn_ptr).unwrap();
    let pre_result = pre_probe();
    let cu_result = original_cuda_fn(fn_ptr);
    logger.result = Some(cu_result);
    post_probe(
        &mut logger,
        &mut delayed_state.cuda_state,
        pre_result,
        cu_result,
    );
    cu_result
}

#[derive(Clone, Copy)]
enum AllocLocation {
    Device,
    DeviceV2,
    Host,
}

pub struct KernelDump {
    module_content: Rc<String>,
    name: String,
    arguments: Option<Vec<usize>>,
}

// We are doing dlopen here instead of just using LD_PRELOAD,
// it's because CUDA Runtime API does dlopen to open libcuda.so, which ignores LD_PRELOAD
pub unsafe fn init_libcuda_handle(func: &'static str) {
    if LIBCUDA_HANDLE == ptr::null_mut() {
        let mut log_factory = log::Factory::new();
        let (logger, settings) = log_factory.get_first_logger_and_init_settings(func);
        MODULES = Some(HashMap::new());
        KERNELS = Some(HashMap::new());
        BUFFERS = Some(BTreeMap::new());
        let libcuda_handle = ptr::null_mut();
        assert_ne!(libcuda_handle, ptr::null_mut());
        LIBCUDA_HANDLE = libcuda_handle;
        match env::var("ZLUDA_DUMP_KERNEL") {
            Ok(kernel_filter) => match Regex::new(&kernel_filter) {
                Ok(r) => KERNEL_PATTERN = Some(r),
                Err(err) => {
                    os_log!("Error parsing ZLUDA_DUMP_KERNEL: {}", err);
                }
            },
            Err(_) => (),
        }
        if let Ok(kernel_min_str) = env::var("ZLUDA_DUMP_MIN_INDEX") {
            match kernel_min_str.parse::<usize>() {
                Ok(kernel_min_value) => KERNEL_INDEX_MINIMUM = kernel_min_value,
                Err(err) => {
                    os_log!("Error parsing ZLUDA_DUMP_MIN_INDEX: {}", err);
                }
            }
        }
        if let Ok(kernel_max_str) = env::var("ZLUDA_DUMP_MAX_INDEX") {
            match kernel_max_str.parse::<usize>() {
                Ok(kernel_max_value) => KERNEL_INDEX_MAXIMUM = kernel_max_value,
                Err(err) => {
                    os_log!("Error parsing ZLUDA_DUMP_MAX_INDEX: {}", err);
                }
            }
        }
        match env::var("ZLUDA_OVERRIDE_COMPUTE_CAPABILITY_MAJOR") {
            Ok(cc_override) => match str::parse::<i32>(&cc_override) {
                Ok(ver) => OVERRIDE_COMPUTE_CAPABILITY_MAJOR = Some(ver),
                Err(err) => {
                    os_log!(
                        "Error parsing ZLUDA_OVERRIDE_COMPUTE_CAPABILITY_MAJOR: {}",
                        err
                    );
                }
            },
            Err(_) => (),
        }
        drop(logger);
    }
}

#[allow(non_snake_case)]
pub unsafe fn cuModuleLoadData(
    module: *mut CUmodule,
    raw_image: *const ::std::os::raw::c_void,
    cont: impl FnOnce(*mut CUmodule, *const c_void) -> CUresult,
) -> CUresult {
    let result = cont(module, raw_image);
    if result == CUresult::CUDA_SUCCESS {
        record_module_image_raw(*module, raw_image);
    }
    result
}

#[allow(non_snake_case)]
pub(crate) fn cuModuleLoad_Post(
    module: *mut CUmodule,
    fname: *const ::std::os::raw::c_char,
    fn_logger: &mut log::FunctionLogger,
    state: &mut trace::StateTracker,
    result: CUresult,
) {
    if result != CUresult::CUDA_SUCCESS {
        return;
    }
    state.record_new_module_file(unsafe { *module }, fname, fn_logger)
}

#[allow(non_snake_case)]
pub(crate) fn cuModuleLoadData_Post(
    module: *mut CUmodule,
    raw_image: *const ::std::os::raw::c_void,
    fn_logger: &mut log::FunctionLogger,
    state: &mut trace::StateTracker,
    result: CUresult,
) {
    if result != CUresult::CUDA_SUCCESS {
        return;
    }
    state.record_new_module(unsafe { *module }, raw_image, fn_logger)
}

unsafe fn record_module_image_raw(module: CUmodule, raw_image: *const ::std::os::raw::c_void) {
    if *(raw_image as *const u32) == 0x464c457f {
        os_log!("Unsupported ELF module image: {:?}", raw_image);
        return;
    }
    let image = to_str(raw_image);
    match image {
        None => os_log!("Malformed module image: {:?}", raw_image),
        Some(image) => record_module_image_with_module(module, raw_image, image),
    };
}

unsafe fn record_module_image_with_module(
    module: CUmodule,
    raw_image: *const ::std::os::raw::c_void,
    image: &str,
) {
    match record_module_image_impl(raw_image, image) {
        Ok(dump) => {
            MODULES
                .get_or_insert_with(|| HashMap::new())
                .insert(module, dump);
        }
        Err(e) => {
            os_log!("{}", e);
        }
    }
}

unsafe fn record_module_image_with_linker(
    link_obj: CUlinkState,
    raw_image: *const ::std::os::raw::c_void,
    image: &str,
) {
    match record_module_image_impl(raw_image, image) {
        Ok(dump) => {
            match PENDING_LINKING
                .get_or_insert_with(|| HashMap::new())
                .entry(link_obj)
            {
                hash_map::Entry::Occupied(mut vec) => {
                    vec.get_mut().push(dump);
                }
                hash_map::Entry::Vacant(e) => {
                    e.insert(vec![dump]);
                }
            };
        }
        Err(e) => {
            os_log!("{}", e);
        }
    }
}

unsafe fn record_module_image_impl(
    raw_image: *const ::std::os::raw::c_void,
    image: &str,
) -> Result<ModuleDump, Box<dyn Error>> {
    if !image.contains(&".version") {
        return Err(format!(
            "Malformed module image (no `.version`): {:?}",
            raw_image
        ))?;
    }
    let mut errors = Vec::new();
    let ast = ptx::ModuleParser::new().parse(&mut errors, image);
    let kernels_args = match (&*errors, ast) {
        (&[], Ok(ast)) => {
            let kernels_args = ast
                .directives
                .iter()
                .filter_map(directive_to_kernel)
                .collect::<HashMap<_, _>>();
            Some(kernels_args)
        }
        (err_vec, res) => {
            // Don't print errors - it's usually too verbose to be useful
            os_log!(
                "{} errors when parsing module image: {:?}",
                err_vec.len() + res.iter().len(),
                raw_image
            );
            None
        }
    };
    let dump = ModuleDump {
        content: Rc::new(image.to_string()),
        kernels_args,
    };
    if let Err(e) = try_dump_module_image(image) {
        return Err(format!(
            "Errors when saving module image: {:?}, {}",
            raw_image, e
        ))?;
    }
    Ok(dump)
}

unsafe fn try_dump_module_image(image: &str) -> Result<(), Box<dyn Error>> {
    let mut dump_path = get_dump_dir()?;
    dump_path.push(format!("module_{:04}.ptx", MODULE_DUMP_COUNTER));
    MODULE_DUMP_COUNTER += 1;
    let mut file = File::create(dump_path)?;
    file.write_all(image.as_bytes())?;
    Ok(())
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
        ast::Directive::Method(
            _,
            ast::Function {
                func_directive:
                    ast::MethodDeclaration {
                        name: ast::MethodName::Kernel(name),
                        input_arguments,
                        ..
                    },
                ..
            },
        ) => {
            let arg_sizes = input_arguments
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
        record_module_image_raw(*module, image);
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
        if let Some(module_dump) = modules.get(&hmod) {
            if let Some(kernel) = to_str(name) {
                let kernel_args = if let Some(kernels) = &module_dump.kernels_args {
                    if let Some(args) = kernels.get(kernel) {
                        Some(args.clone())
                    } else {
                        None
                    }
                } else {
                    None
                };
                KERNELS.as_mut().unwrap().insert(
                    *hfunc,
                    KernelDump {
                        module_content: module_dump.content.clone(),
                        name: kernel.to_string(),
                        arguments: kernel_args,
                    },
                );
            } else {
                os_log!("Malformed name at: {:?}", hfunc);
            }
        } else {
            os_log!("Unknown module: {:?}", hmod);
        }
    } else {
        os_log!("Unknown module: {:?}", hmod);
    }
    CUresult::CUDA_SUCCESS
}

#[allow(non_snake_case)]
pub unsafe fn cuMemAlloc(
    dptr: *mut CUdeviceptr,
    bytesize: usize,
    cont: impl FnOnce(*mut CUdeviceptr, usize) -> CUresult,
) -> CUresult {
    cuMemAlloc_impl(false, dptr, bytesize, cont)
}

#[allow(non_snake_case)]
pub unsafe fn cuMemAlloc_v2(
    dptr: *mut CUdeviceptr,
    bytesize: usize,
    cont: impl FnOnce(*mut CUdeviceptr, usize) -> CUresult,
) -> CUresult {
    cuMemAlloc_impl(true, dptr, bytesize, cont)
}

#[allow(non_snake_case)]
pub unsafe fn cuMemAlloc_impl(
    is_v2: bool,
    dptr: *mut CUdeviceptr,
    bytesize: usize,
    cont: impl FnOnce(*mut CUdeviceptr, usize) -> CUresult,
) -> CUresult {
    let result = cont(dptr, bytesize);
    assert_eq!(result, CUresult::CUDA_SUCCESS);
    let start = (*dptr).0 as usize;
    let location = if is_v2 {
        AllocLocation::DeviceV2
    } else {
        AllocLocation::Device
    };
    BUFFERS
        .as_mut()
        .unwrap()
        .insert(start, (bytesize, location));
    CUresult::CUDA_SUCCESS
}

#[allow(non_snake_case)]
pub unsafe fn cuMemHostAlloc(
    pp: *mut *mut c_void,
    bytesize: usize,
    flags: c_uint,
    cont: impl FnOnce(*mut *mut c_void, usize, c_uint) -> CUresult,
) -> CUresult {
    let result = cont(pp, bytesize, flags);
    assert_eq!(result, CUresult::CUDA_SUCCESS);
    let start = (*pp) as usize;
    BUFFERS
        .as_mut()
        .unwrap()
        .insert(start, (bytesize, AllocLocation::Host));
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
    let dump_env = match create_dump_dir(f, LAUNCH_COUNTER) {
        Ok(dump_env) => dump_env,
        Err(err) => {
            os_log!("Error when creating the dump directory: {}", err);
            None
        }
    };
    if let Some(dump_env) = &dump_env {
        dump_pre_data(
            gridDimX,
            gridDimY,
            gridDimZ,
            blockDimX,
            blockDimY,
            blockDimZ,
            sharedMemBytes,
            kernelParams,
            extra,
            dump_env,
        )
        .unwrap_or_else(|err| os_log!("{}", err));
    };
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
    if let Some((_, kernel_dump)) = &dump_env {
        dump_arguments(
            kernelParams,
            extra,
            "post",
            &kernel_dump.name,
            LAUNCH_COUNTER,
            kernel_dump.arguments.as_ref().map(|vec| &vec[..]),
        )
        .unwrap_or_else(|err| os_log!("{}", err));
    }
    LAUNCH_COUNTER += 1;
    CUresult::CUDA_SUCCESS
}

#[allow(non_snake_case)]
fn dump_launch_arguments(
    gridDimX: u32,
    gridDimY: u32,
    gridDimZ: u32,
    blockDimX: u32,
    blockDimY: u32,
    blockDimZ: u32,
    sharedMemBytes: u32,
    dump_dir: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    let mut module_file_path = dump_dir.clone();
    module_file_path.push("launch.txt");
    let mut module_file = File::create(module_file_path)?;
    write!(&mut module_file, "{}\n", gridDimX)?;
    write!(&mut module_file, "{}\n", gridDimY)?;
    write!(&mut module_file, "{}\n", gridDimZ)?;
    write!(&mut module_file, "{}\n", blockDimX)?;
    write!(&mut module_file, "{}\n", blockDimY)?;
    write!(&mut module_file, "{}\n", blockDimZ)?;
    write!(&mut module_file, "{}\n", sharedMemBytes)?;
    Ok(())
}

unsafe fn should_dump_kernel(counter: usize, name: &str) -> bool {
    if counter < KERNEL_INDEX_MINIMUM {
        return false;
    }
    if counter > KERNEL_INDEX_MAXIMUM {
        return false;
    }
    match &KERNEL_PATTERN {
        Some(pattern) => pattern.is_match(name),
        None => true,
    }
}

unsafe fn create_dump_dir(
    f: CUfunction,
    counter: usize,
) -> Result<Option<(PathBuf, &'static KernelDump)>, Box<dyn Error>> {
    match KERNELS.as_ref().and_then(|kernels| kernels.get(&f)) {
        Some(kernel_dump) => {
            if !should_dump_kernel(counter, &kernel_dump.name) {
                return Ok(None);
            }
            let mut dump_dir = get_dump_dir()?;
            dump_dir.push(format!("{:04}_{}", counter, kernel_dump.name));
            fs::create_dir_all(&dump_dir)?;
            Ok(Some((dump_dir, kernel_dump)))
        }
        None => Err(format!("Unknown kernel: {:?}", f))?,
    }
}

#[allow(non_snake_case)]
unsafe fn dump_pre_data(
    gridDimX: ::std::os::raw::c_uint,
    gridDimY: ::std::os::raw::c_uint,
    gridDimZ: ::std::os::raw::c_uint,
    blockDimX: ::std::os::raw::c_uint,
    blockDimY: ::std::os::raw::c_uint,
    blockDimZ: ::std::os::raw::c_uint,
    sharedMemBytes: ::std::os::raw::c_uint,
    kernelParams: *mut *mut ::std::os::raw::c_void,
    extra: *mut *mut ::std::os::raw::c_void,
    (dump_dir, kernel_dump): &(PathBuf, &'static KernelDump),
) -> Result<(), Box<dyn Error>> {
    dump_launch_arguments(
        gridDimX,
        gridDimY,
        gridDimZ,
        blockDimX,
        blockDimY,
        blockDimZ,
        sharedMemBytes,
        dump_dir,
    )?;
    let mut module_file_path = dump_dir.clone();
    module_file_path.push("module.ptx");
    let mut module_file = File::create(module_file_path)?;
    module_file.write_all(kernel_dump.module_content.as_bytes())?;
    dump_arguments(
        kernelParams,
        extra,
        "pre",
        &kernel_dump.name,
        LAUNCH_COUNTER,
        kernel_dump.arguments.as_ref().map(|vec| &vec[..]),
    )?;
    Ok(())
}

fn dump_arguments(
    kernel_params: *mut *mut ::std::os::raw::c_void,
    extra: *mut *mut ::std::os::raw::c_void,
    prefix: &str,
    kernel_name: &str,
    counter: usize,
    args: Option<&[usize]>,
) -> Result<(), Box<dyn Error>> {
    let args = match args {
        None => return Ok(()),
        Some(a) => a,
    };
    let mut dump_dir = get_dump_dir()?;
    dump_dir.push(format!("{:04}_{}", counter, kernel_name));
    dump_dir.push(prefix);
    if dump_dir.exists() {
        fs::remove_dir_all(&dump_dir)?;
    }
    fs::create_dir_all(&dump_dir)?;
    if kernel_params != ptr::null_mut() {
        for (i, arg_len) in args.iter().enumerate() {
            unsafe { dump_argument_to_file(&dump_dir, i, *arg_len, *kernel_params.add(i))? };
        }
    } else {
        let mut offset = 0;
        let mut buffer_ptr = None;
        let mut buffer_size = None;
        loop {
            match unsafe { *extra.add(offset) } {
                CU_LAUNCH_PARAM_END => break,
                CU_LAUNCH_PARAM_BUFFER_POINTER => {
                    buffer_ptr = Some(unsafe { *extra.add(offset + 1) as *mut u8 });
                }
                CU_LAUNCH_PARAM_BUFFER_SIZE => {
                    buffer_size = Some(unsafe { *(*extra.add(offset + 1) as *mut usize) });
                }
                _ => return Err("Malformed `extra` parameter to kernel launch")?,
            }
            offset += 2;
        }
        match (buffer_size, buffer_ptr) {
            (Some(buffer_size), Some(buffer_ptr)) => {
                let sum_of_kernel_argument_sizes = args.iter().fold(0, |offset, size_of_arg| {
                    size_of_arg + round_up_to_multiple(offset, *size_of_arg)
                });
                if buffer_size < sum_of_kernel_argument_sizes {
                    return Err("Malformed `extra` parameter to kernel launch")?;
                }
                let mut offset = 0;
                for (i, arg_size) in args.iter().enumerate() {
                    let buffer_offset = round_up_to_multiple(offset, *arg_size);
                    unsafe {
                        dump_argument_to_file(
                            &dump_dir,
                            i,
                            *arg_size,
                            buffer_ptr.add(buffer_offset) as *const _,
                        )?
                    };
                    offset = buffer_offset + *arg_size;
                }
            }
            _ => return Err("Malformed `extra` parameter to kernel launch")?,
        }
    }
    Ok(())
}

fn round_up_to_multiple(x: usize, multiple: usize) -> usize {
    ((x + multiple - 1) / multiple) * multiple
}

unsafe fn dump_argument_to_file(
    dump_dir: &PathBuf,
    i: usize,
    arg_len: usize,
    ptr: *const c_void,
) -> Result<(), Box<dyn Error>> {
    // Don't check if arg_len == sizeof(void*), there are libraries
    // which for some reason pass 32 pointers (4 bytes) in 8 byte arguments
    match get_buffer_length(*(ptr as *mut usize)) {
        Some((start, len, location)) => {
            let mut output = vec![0u8; len];
            let memcpy_fn = match location {
                AllocLocation::Device => |src, dst: usize, len| {
                    let error = cuda::cuMemcpyDtoH(dst as *mut _, CUdeviceptr(src), len);
                    assert_eq!(error, CUresult::CUDA_SUCCESS);
                },
                AllocLocation::DeviceV2 => |src, dst: usize, len| {
                    let error = cuda::cuMemcpyDtoH_v2(dst as *mut _, CUdeviceptr(src), len);
                    assert_eq!(error, CUresult::CUDA_SUCCESS);
                },
                AllocLocation::Host => |src, dst: usize, len| {
                    ptr::copy_nonoverlapping(src as *mut u8, dst as *mut u8, len);
                },
            };
            memcpy_fn(start, output.as_mut_ptr() as usize, len);
            let mut path = dump_dir.clone();
            path.push(format!("arg_{:03}.buffer", i));
            let mut file = File::create(path)?;
            file.write_all(&mut output)?;
        }
        None => {
            let mut path = dump_dir.clone();
            path.push(format!("arg_{:03}", i));
            let mut file = File::create(path)?;
            file.write_all(slice::from_raw_parts(ptr as *mut u8, arg_len))?;
        }
    }
    Ok(())
}

unsafe fn get_buffer_length(ptr: usize) -> Option<(usize, usize, AllocLocation)> {
    BUFFERS
        .as_mut()
        .unwrap()
        .range(..=ptr)
        .next_back()
        .and_then(|(start, (len, loc))| {
            let end = *start + *len;
            if ptr < end {
                Some((ptr, end - ptr, *loc))
            } else {
                None
            }
        })
}

fn get_dump_dir() -> Result<PathBuf, Box<dyn Error>> {
    let dir = env::var("ZLUDA_DUMP_DIR")?;
    let mut main_dir = PathBuf::from(dir);
    let current_exe = env::current_exe()?;
    main_dir.push(current_exe.file_name().unwrap());
    fs::create_dir_all(&main_dir)?;
    Ok(main_dir)
}

// TODO make this more common with ZLUDA implementation
const CUDART_INTERFACE_GUID: CUuuid = CUuuid {
    bytes: [
        0x6b, 0xd5, 0xfb, 0x6c, 0x5b, 0xf4, 0xe7, 0x4a, 0x89, 0x87, 0xd9, 0x39, 0x12, 0xfd, 0x9d,
        0xf9,
    ],
};

static mut OVERRIDEN_INTERFACE_VTABLES: Option<HashMap<Box<CUuuid>, Vec<*const c_void>>> = None;

static mut ORIGINAL_GET_MODULE_FROM_CUBIN: Option<
    unsafe extern "system" fn(
        result: *mut CUmodule,
        fatbinc_wrapper: *const FatbincWrapper,
    ) -> CUresult,
> = None;
static mut ORIGINAL_GET_MODULE_FROM_CUBIN_EXT1: Option<
    unsafe extern "system" fn(
        result: *mut CUmodule,
        fatbinc_wrapper: *const FatbincWrapper,
        ptr1: *mut c_void,
        ptr2: *mut c_void,
        _unknown: usize,
    ) -> CUresult,
> = None;
static mut ORIGINAL_GET_MODULE_FROM_CUBIN_EXT2: Option<
    unsafe extern "system" fn(
        fatbinc_wrapper: *const FatbinHeader,
        result: *mut CUmodule,
        ptr1: *mut c_void,
        ptr2: *mut c_void,
        _unknown: usize,
    ) -> CUresult,
> = None;

unsafe extern "system" fn report_unknown_export_table_call(
    export_table: *const CUuuid,
    idx: usize,
) {
    let guid = (*export_table).bytes;
    os_log!("Call to an unsupported export table function: {{{:02X}{:02X}{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}}}::{}", guid[0], guid[1], guid[2], guid[3], guid[4], guid[5], guid[6], guid[7], guid[8], guid[9], guid[10], guid[11], guid[12], guid[13], guid[14], guid[15], idx);
}

#[allow(non_snake_case)]
pub unsafe fn cuGetExportTable(
    ppExportTable: *mut *const ::std::os::raw::c_void,
    pExportTableId: *const CUuuid,
    cont: impl FnOnce(*mut *const ::std::os::raw::c_void, *const CUuuid) -> CUresult,
) -> CUresult {
    if ppExportTable == ptr::null_mut() || pExportTableId == ptr::null() {
        return CUresult::CUDA_ERROR_INVALID_VALUE;
    }
    let guid = (*pExportTableId).bytes;
    os_log!("Requested export table id: {{{:02X}{:02X}{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}}}", guid[0], guid[1], guid[2], guid[3], guid[4], guid[5], guid[6], guid[7], guid[8], guid[9], guid[10], guid[11], guid[12], guid[13], guid[14], guid[15]);
    override_export_table(ppExportTable, pExportTableId, cont)
}

unsafe fn override_export_table(
    export_table_ptr: *mut *const ::std::os::raw::c_void,
    export_table_id: *const CUuuid,
    cont: impl FnOnce(*mut *const ::std::os::raw::c_void, *const CUuuid) -> CUresult,
) -> CUresult {
    let overrides_map = OVERRIDEN_INTERFACE_VTABLES.get_or_insert_with(|| HashMap::new());
    if let Some(override_table) = overrides_map.get(&*export_table_id) {
        *export_table_ptr = override_table.as_ptr() as *const _;
        return CUresult::CUDA_SUCCESS;
    }
    let base_result = cont(export_table_ptr, export_table_id);
    if base_result != CUresult::CUDA_SUCCESS {
        return base_result;
    }
    let export_table = (*export_table_ptr) as *mut *const c_void;
    let boxed_guid = Box::new(*export_table_id);
    let byte_length: usize = *(export_table as *const usize);
    let mut override_table = Vec::new();
    if byte_length < 0x10000 {
        override_table.push(byte_length as *const _);
        let length = byte_length / mem::size_of::<usize>();
        for i in 1..length {
            let current_fn = export_table.add(i);
            if (*current_fn as usize) == usize::max_value() {
                override_table.push(usize::max_value() as *const _);
                break;
            }
            override_table.push(get_export_override_fn(*current_fn, &*boxed_guid, i));
        }
    } else {
        let mut i = 0;
        loop {
            let current_fn = export_table.add(i);
            if (*current_fn as usize) == usize::max_value() {
                override_table.push(usize::max_value() as *const _);
                break;
            }
            override_table.push(get_export_override_fn(*current_fn, &*boxed_guid, i));
            i += 1;
        }
    }
    *export_table_ptr = override_table.as_ptr() as *const _;
    overrides_map.insert(boxed_guid, override_table);
    CUresult::CUDA_SUCCESS
}

const TOOLS_RUNTIME_CALLBACK_HOOKS_GUID: CUuuid = CUuuid {
    bytes: [
        0xa0, 0x94, 0x79, 0x8c, 0x2e, 0x74, 0x2e, 0x74, 0x93, 0xf2, 0x08, 0x00, 0x20, 0x0c, 0x0a,
        0x66,
    ],
};

const CONTEXT_LOCAL_STORAGE_INTERFACE_V0301_GUID: CUuuid = CUuuid {
    bytes: [
        0xc6, 0x93, 0x33, 0x6e, 0x11, 0x21, 0xdf, 0x11, 0xa8, 0xc3, 0x68, 0xf3, 0x55, 0xd8, 0x95,
        0x93,
    ],
};

const CTX_CREATE_BYPASS_GUID: CUuuid = CUuuid {
    bytes: [
        0x0C, 0xA5, 0x0B, 0x8C, 0x10, 0x04, 0x92, 0x9A, 0x89, 0xA7, 0xD0, 0xDF, 0x10, 0xE7, 0x72,
        0x86,
    ],
};

const HEAP_ACCESS_GUID: CUuuid = CUuuid {
    bytes: [
        0x19, 0x5B, 0xCB, 0xF4, 0xD6, 0x7D, 0x02, 0x4A, 0xAC, 0xC5, 0x1D, 0x29, 0xCE, 0xA6, 0x31,
        0xAE,
    ],
};

const DEVICE_EXTENDED_RT_GUID: CUuuid = CUuuid {
    bytes: [
        0xB1u8, 0x05, 0x41, 0xE1, 0xF7, 0xC7, 0xC7, 0x4A, 0x9F, 0x64, 0xF2, 0x23, 0xBE, 0x99, 0xF1,
        0xE2,
    ],
};

unsafe fn get_export_override_fn(
    original_fn: *const c_void,
    guid: *const CUuuid,
    idx: usize,
) -> *const c_void {
    match (*guid, idx) {
        (TOOLS_RUNTIME_CALLBACK_HOOKS_GUID, 2)
        | (TOOLS_RUNTIME_CALLBACK_HOOKS_GUID, 6)
        | (CUDART_INTERFACE_GUID, 2)
        | (CUDART_INTERFACE_GUID, 7)
        | (CONTEXT_LOCAL_STORAGE_INTERFACE_V0301_GUID, 0)
        | (CONTEXT_LOCAL_STORAGE_INTERFACE_V0301_GUID, 1)
        | (CONTEXT_LOCAL_STORAGE_INTERFACE_V0301_GUID, 2)
        | (CTX_CREATE_BYPASS_GUID, 1)
        | (HEAP_ACCESS_GUID, 1)
        | (HEAP_ACCESS_GUID, 2)
        | (DEVICE_EXTENDED_RT_GUID, 5)
        | (DEVICE_EXTENDED_RT_GUID, 13) => original_fn,
        (CUDART_INTERFACE_GUID, 1) => {
            ORIGINAL_GET_MODULE_FROM_CUBIN = mem::transmute(original_fn);
            get_module_from_cubin as *const _
        }
        (CUDART_INTERFACE_GUID, 6) => {
            ORIGINAL_GET_MODULE_FROM_CUBIN_EXT1 = mem::transmute(original_fn);
            get_module_from_cubin_ext1 as *const _
        }
        (CUDART_INTERFACE_GUID, 8) => {
            ORIGINAL_GET_MODULE_FROM_CUBIN_EXT2 = mem::transmute(original_fn);
            get_module_from_cubin_ext2 as *const _
        }
        _ => os::get_thunk(original_fn, report_unknown_export_table_call, guid, idx),
    }
}

const FATBINC_MAGIC: c_uint = 0x466243B1;
const FATBINC_VERSION: c_uint = 0x1;

#[repr(C)]
struct FatbincWrapper {
    magic: c_uint,
    version: c_uint,
    data: *const FatbinHeader,
    filename_or_fatbins: *const c_void,
}

const FATBIN_MAGIC: c_uint = 0xBA55ED50;
const FATBIN_VERSION: c_ushort = 0x01;

#[repr(C, align(8))]
struct FatbinHeader {
    magic: c_uint,
    version: c_ushort,
    header_size: c_ushort,
    files_size: c_ulong, // excluding frame header, size of all blocks framed by this frame
}

const FATBIN_FILE_HEADER_KIND_PTX: c_ushort = 0x01;
const FATBIN_FILE_HEADER_VERSION_CURRENT: c_ushort = 0x101;

// assembly file header is a bit different, but we don't care
#[repr(C)]
#[derive(Debug)]
struct FatbinFileHeader {
    kind: c_ushort,
    version: c_ushort,
    header_size: c_uint,
    padded_payload_size: c_uint,
    unknown0: c_uint, // check if it's written into separately
    payload_size: c_uint,
    unknown1: c_uint,
    unknown2: c_uint,
    sm_version: c_uint,
    bit_width: c_uint,
    unknown3: c_uint,
    unknown4: c_ulong,
    unknown5: c_ulong,
    uncompressed_payload: c_ulong,
}

unsafe fn get_module_from_cubin_impl(
    module: *mut CUmodule,
    fatbinc_wrapper: *const FatbincWrapper,
    get_module_base: impl FnOnce() -> CUresult,
) -> CUresult {
    if module == ptr::null_mut()
        || (*fatbinc_wrapper).magic != FATBINC_MAGIC
        || (*fatbinc_wrapper).version != FATBINC_VERSION
    {
        return CUresult::CUDA_ERROR_INVALID_VALUE;
    }
    get_module_from_cubin_unwrapped(module, (*fatbinc_wrapper).data, get_module_base)
}

unsafe fn get_module_from_cubin_unwrapped(
    module: *mut CUmodule,
    fatbin_header: *const FatbinHeader,
    get_module_base: impl FnOnce() -> CUresult,
) -> CUresult {
    if (*fatbin_header).magic != FATBIN_MAGIC || (*fatbin_header).version != FATBIN_VERSION {
        return CUresult::CUDA_ERROR_INVALID_VALUE;
    }
    let file = (fatbin_header as *const u8).add((*fatbin_header).header_size as usize);
    let end = file.add((*fatbin_header).files_size as usize);
    let mut ptx_files = get_ptx_files(file, end);
    ptx_files.sort_unstable_by_key(|f| c_uint::max_value() - (**f).sm_version);
    let mut maybe_kernel_text = None;
    for file in ptx_files {
        match decompress_kernel_module(file) {
            None => continue,
            Some(vec) => {
                maybe_kernel_text = Some(vec);
                break;
            }
        };
    }
    let result = get_module_base();
    if result != CUresult::CUDA_SUCCESS {
        return result;
    }
    if let Some(text) = maybe_kernel_text {
        match CStr::from_bytes_with_nul(&text) {
            Ok(cstr) => match cstr.to_str() {
                Ok(utf8_str) => {
                    record_module_image_with_module(*module, text.as_ptr() as _, utf8_str)
                }
                Err(_) => {}
            },
            Err(_) => {}
        }
    } else {
        os_log!("Unsupported runtime module: {:?}", *module);
    }
    result
}

unsafe extern "system" fn get_module_from_cubin(
    module: *mut CUmodule,
    fatbinc_wrapper: *const FatbincWrapper,
) -> CUresult {
    get_module_from_cubin_impl(module, fatbinc_wrapper, || {
        ORIGINAL_GET_MODULE_FROM_CUBIN.unwrap()(module, fatbinc_wrapper)
    })
}

unsafe extern "system" fn get_module_from_cubin_ext1(
    module: *mut CUmodule,
    fatbinc_wrapper: *const FatbincWrapper,
    ptr1: *mut c_void,
    ptr2: *mut c_void,
    _unknown: usize,
) -> CUresult {
    get_module_from_cubin_impl(module, fatbinc_wrapper, || {
        ORIGINAL_GET_MODULE_FROM_CUBIN_EXT1.unwrap()(module, fatbinc_wrapper, ptr1, ptr2, _unknown)
    })
}

unsafe extern "system" fn get_module_from_cubin_ext2(
    fatbin_header: *const FatbinHeader,
    module: *mut CUmodule,
    ptr1: *mut c_void,
    ptr2: *mut c_void,
    _unknown: usize,
) -> CUresult {
    get_module_from_cubin_unwrapped(module, fatbin_header, || {
        ORIGINAL_GET_MODULE_FROM_CUBIN_EXT2.unwrap()(fatbin_header, module, ptr1, ptr2, _unknown)
    })
}

unsafe fn get_ptx_files(file: *const u8, end: *const u8) -> Vec<*const FatbinFileHeader> {
    let mut index = file;
    let mut result = Vec::new();
    while index < end {
        let file = index as *const FatbinFileHeader;
        if (*file).kind == FATBIN_FILE_HEADER_KIND_PTX
            && (*file).version == FATBIN_FILE_HEADER_VERSION_CURRENT
        {
            result.push(file)
        }
        index = index.add((*file).header_size as usize + (*file).padded_payload_size as usize);
    }
    result
}

const MAX_PTX_MODULE_DECOMPRESSION_BOUND: usize = 16 * 1024 * 1024;

unsafe fn decompress_kernel_module(file: *const FatbinFileHeader) -> Option<Vec<u8>> {
    let decompressed_size = usize::max(1024, (*file).uncompressed_payload as usize);
    let mut decompressed_vec = vec![0u8; decompressed_size];
    loop {
        match lz4_sys::LZ4_decompress_safe(
            (file as *const u8).add((*file).header_size as usize) as *const _,
            decompressed_vec.as_mut_ptr() as *mut _,
            (*file).payload_size as c_int,
            decompressed_vec.len() as c_int,
        ) {
            error if error < 0 => {
                let new_size = decompressed_vec.len() * 2;
                if new_size > MAX_PTX_MODULE_DECOMPRESSION_BOUND {
                    return None;
                }
                decompressed_vec.resize(decompressed_vec.len() * 2, 0);
            }
            real_decompressed_size => {
                decompressed_vec.truncate(real_decompressed_size as usize);
                if decompressed_vec.last().copied().unwrap_or(1) != 0 {
                    decompressed_vec.push(0);
                }
                return Some(decompressed_vec);
            }
        }
    }
}

#[allow(non_snake_case)]
pub unsafe fn cuDeviceGetAttribute(
    pi: *mut ::std::os::raw::c_int,
    attrib: CUdevice_attribute,
    dev: CUdevice,
    cont: impl FnOnce(*mut ::std::os::raw::c_int, CUdevice_attribute, CUdevice) -> CUresult,
) -> CUresult {
    if attrib == CUdevice_attribute::CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR {
        if let Some(ver) = OVERRIDE_COMPUTE_CAPABILITY_MAJOR {
            *pi = ver;
            return CUresult::CUDA_SUCCESS;
        }
    }
    cont(pi, attrib, dev)
}

#[allow(non_snake_case)]
pub unsafe fn cuLinkAddData(
    state: CUlinkState,
    type_: CUjitInputType,
    data: *mut ::std::os::raw::c_void,
    size: usize,
    name: *const ::std::os::raw::c_char,
    numOptions: ::std::os::raw::c_uint,
    options: *mut CUjit_option,
    optionValues: *mut *mut ::std::os::raw::c_void,
    cont: impl FnOnce(
        CUlinkState,
        CUjitInputType,
        *mut ::std::os::raw::c_void,
        usize,
        *const ::std::os::raw::c_char,
        ::std::os::raw::c_uint,
        *mut CUjit_option,
        *mut *mut ::std::os::raw::c_void,
    ) -> CUresult,
) -> CUresult {
    if let Some(image) = to_str(data) {
        record_module_image_with_linker(state, data, image)
    } else {
        os_log!("PTX module not a string: {:?}", data);
    }
    cont(
        state,
        type_,
        data,
        size,
        name,
        numOptions,
        options,
        optionValues,
    )
}

#[allow(non_snake_case)]
pub unsafe fn cuLinkAddFile(
    state: CUlinkState,
    type_: CUjitInputType,
    path: *const ::std::os::raw::c_char,
    numOptions: ::std::os::raw::c_uint,
    options: *mut CUjit_option,
    optionValues: *mut *mut ::std::os::raw::c_void,
    cont: impl FnOnce(
        CUlinkState,
        CUjitInputType,
        *const ::std::os::raw::c_char,
        ::std::os::raw::c_uint,
        *mut CUjit_option,
        *mut *mut ::std::os::raw::c_void,
    ) -> CUresult,
) -> CUresult {
    cont(state, type_, path, numOptions, options, optionValues)
}
