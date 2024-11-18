use cuda_types::*;
use paste::paste;
use side_by_side::CudaDynamicFns;
use std::io;
use std::{collections::HashMap, env, error::Error, fs, path::PathBuf, rc::Rc, sync::Mutex};

#[macro_use]
extern crate lazy_static;
extern crate cuda_types;

macro_rules! extern_redirect {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:path);*) => {
        $(
            #[no_mangle]
            #[allow(improper_ctypes_definitions)]
            pub extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                let original_fn = |dynamic_fns: &mut crate::side_by_side::CudaDynamicFns| {
                    dynamic_fns.$fn_name($( $arg_id ),*)
                };
                let get_formatted_args = Box::new(move |writer: &mut dyn std::io::Write| {
                    (paste! { format :: [<write_ $fn_name>] }) (
                        writer
                        $(,$arg_id)*
                    )
                });
                crate::handle_cuda_function_call(stringify!($fn_name), original_fn, get_formatted_args)
            }
        )*
    };
}

macro_rules! extern_redirect_with_post {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:path);*) => {
        $(
            #[no_mangle]
            #[allow(improper_ctypes_definitions)]
            pub extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                let original_fn = |dynamic_fns: &mut crate::side_by_side::CudaDynamicFns| {
                    dynamic_fns.$fn_name($( $arg_id ),*)
                };
                let get_formatted_args = Box::new(move |writer: &mut dyn std::io::Write| {
                    (paste! { format :: [<write_ $fn_name>] }) (
                        writer
                        $(,$arg_id)*
                    )
                });
                crate::handle_cuda_function_call_with_probes(
                    stringify!($fn_name),
                    || (), original_fn,
                    get_formatted_args,
                    move |logger, state, _, cuda_result| paste! { [<$fn_name _Post>] } ( $( $arg_id ),* , logger, state, cuda_result )
                )
            }
        )*
    };
}

use cuda_base::cuda_function_declarations;
cuda_function_declarations!(
    cuda_types,
    extern_redirect,
    extern_redirect_with_post,
    [
        cuModuleLoad,
        cuModuleLoadData,
        cuModuleLoadDataEx,
        cuGetExportTable,
        cuModuleGetFunction,
        cuDeviceGetAttribute,
        cuDeviceComputeCapability,
        cuModuleLoadFatBinary
    ]
);

mod dark_api;
mod format;
mod log;
#[cfg_attr(windows, path = "os_win.rs")]
#[cfg_attr(not(windows), path = "os_unix.rs")]
mod os;
mod side_by_side;
mod trace;

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
            Self::Success(t) => Some(t),
            Self::Unitialized => None,
            Self::Error => None,
        }
    }

    pub(crate) fn unwrap_mut(&mut self) -> &mut T {
        match self {
            Self::Success(t) => t,
            Self::Unitialized | Self::Error => panic!(),
        }
    }
}

struct GlobalDelayedState {
    settings: Settings,
    libcuda: CudaDynamicFns,
    side_by_side_lib: Option<CudaDynamicFns>,
    cuda_state: trace::StateTracker,
}

impl GlobalDelayedState {
    fn new<'a>(
        func: &'static str,
        arguments_writer: Box<dyn FnMut(&mut dyn std::io::Write) -> std::io::Result<()>>,
        factory: &'a mut log::Factory,
    ) -> (LateInit<Self>, log::FunctionLogger<'a>) {
        let (mut fn_logger, settings) =
            factory.get_first_logger_and_init_settings(func, arguments_writer);
        let libcuda = match unsafe { CudaDynamicFns::load_library(&settings.libcuda_path) } {
            Some(libcuda) => libcuda,
            None => {
                fn_logger.log(log::LogEntry::ErrorBox(
                    format!("Invalid CUDA library at path {}", &settings.libcuda_path).into(),
                ));
                return (LateInit::Error, fn_logger);
            }
        };
        let side_by_side_lib = settings
            .side_by_side_path
            .as_ref()
            .and_then(|side_by_side_path| {
                match unsafe { CudaDynamicFns::load_library(&*side_by_side_path) } {
                    Some(fns) => Some(fns),
                    None => {
                        fn_logger.log(log::LogEntry::ErrorBox(
                            format!(
                                "Invalid side-by-side CUDA library at path {}",
                                &side_by_side_path
                            )
                            .into(),
                        ));
                        None
                    }
                }
            });
        let cuda_state = trace::StateTracker::new(&settings);
        let delayed_state = GlobalDelayedState {
            settings,
            libcuda,
            cuda_state,
            side_by_side_lib,
        };
        (LateInit::Success(delayed_state), fn_logger)
    }
}

struct Settings {
    dump_dir: Option<PathBuf>,
    libcuda_path: String,
    override_cc_major: Option<u32>,
    side_by_side_path: Option<String>,
}

impl Settings {
    fn read_and_init(logger: &mut log::FunctionLogger) -> Self {
        let maybe_dump_dir = Self::read_and_init_dump_dir();
        let dump_dir = match maybe_dump_dir {
            Ok(Some(dir)) => {
                logger.log(log::LogEntry::CreatedDumpDirectory(dir.clone()));
                Some(dir)
            }
            Ok(None) => None,
            Err(err) => {
                logger.log(log::LogEntry::ErrorBox(err));
                None
            }
        };
        let libcuda_path = match env::var("ZLUDA_CUDA_LIB") {
            Err(env::VarError::NotPresent) => os::LIBCUDA_DEFAULT_PATH.to_string(),
            Err(e) => {
                logger.log(log::LogEntry::ErrorBox(Box::new(e) as _));
                os::LIBCUDA_DEFAULT_PATH.to_string()
            }
            Ok(env_string) => env_string,
        };
        let override_cc_major = match env::var("ZLUDA_OVERRIDE_COMPUTE_CAPABILITY_MAJOR") {
            Err(env::VarError::NotPresent) => None,
            Err(e) => {
                logger.log(log::LogEntry::ErrorBox(Box::new(e) as _));
                None
            }
            Ok(env_string) => match str::parse::<u32>(&*env_string) {
                Err(e) => {
                    logger.log(log::LogEntry::ErrorBox(Box::new(e) as _));
                    None
                }
                Ok(cc) => Some(cc),
            },
        };
        let side_by_side_path = match env::var("ZLUDA_SIDE_BY_SIDE_LIB") {
            Err(env::VarError::NotPresent) => None,
            Err(e) => {
                logger.log(log::LogEntry::ErrorBox(Box::new(e) as _));
                None
            }
            Ok(env_string) => Some(env_string),
        };
        Settings {
            dump_dir,
            libcuda_path,
            override_cc_major,
            side_by_side_path,
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
        let file_name_base = current_exe.file_name().unwrap().to_string_lossy();
        main_dir.push(&*file_name_base);
        let mut suffix = 1;
        // This can get into infinite loop. Unfortunately try_exists is unstable:
        // https://doc.rust-lang.org/std/path/struct.Path.html#method.try_exists
        while main_dir.exists() {
            main_dir.set_file_name(format!("{}_{}", file_name_base, suffix));
            suffix += 1;
        }
        fs::create_dir_all(&*main_dir)?;
        Ok(main_dir)
    }
}

pub struct ModuleDump {
    content: Rc<String>,
    kernels_args: Option<HashMap<String, Vec<usize>>>,
}

fn handle_cuda_function_call(
    func: &'static str,
    original_cuda_fn: impl FnOnce(&mut CudaDynamicFns) -> Option<CUresult>,
    arguments_writer: Box<dyn FnMut(&mut dyn std::io::Write) -> std::io::Result<()>>,
) -> CUresult {
    handle_cuda_function_call_with_probes(
        func,
        || (),
        original_cuda_fn,
        arguments_writer,
        |_, _, _, _| (),
    )
}

fn handle_cuda_function_call_with_probes<T, PostFn>(
    func: &'static str,
    pre_probe: impl FnOnce() -> T,
    original_cuda_fn: impl FnOnce(&mut CudaDynamicFns) -> Option<CUresult>,
    arguments_writer: Box<dyn FnMut(&mut dyn std::io::Write) -> std::io::Result<()>>,
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
    let global_state = &mut *global_state_mutex.lock().unwrap();
    let (mut logger, delayed_state) = match global_state.delayed_state {
        LateInit::Success(ref mut delayed_state) => (
            global_state.log_factory.get_logger(func, arguments_writer),
            delayed_state,
        ),
        // There's no libcuda to load, so we might as well panic
        LateInit::Error => panic!(),
        LateInit::Unitialized => {
            let (new_delayed_state, logger) =
                GlobalDelayedState::new(func, arguments_writer, &mut global_state.log_factory);
            global_state.delayed_state = new_delayed_state;
            (logger, global_state.delayed_state.as_mut().unwrap())
        }
    };
    let pre_result = pre_probe();
    let maybe_cu_result = original_cuda_fn(&mut delayed_state.libcuda);
    let cu_result = match maybe_cu_result {
        Some(result) => result,
        None => {
            logger.log(log::LogEntry::ErrorBox(
                format!("No function {} in the underlying CUDA library", func).into(),
            ));
            CUresult::ERROR_UNKNOWN
        }
    };
    logger.result = maybe_cu_result;
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

#[allow(non_snake_case)]
pub(crate) fn cuModuleLoad_Post(
    module: *mut CUmodule,
    fname: *const ::std::os::raw::c_char,
    fn_logger: &mut log::FunctionLogger,
    state: &mut trace::StateTracker,
    result: CUresult,
) {
    if result.is_err() {
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
    if result.is_err() {
        return;
    }
    state.record_new_module(unsafe { *module }, raw_image, fn_logger)
}

#[allow(non_snake_case)]
pub(crate) fn cuModuleLoadDataEx_Post(
    module: *mut CUmodule,
    raw_image: *const ::std::os::raw::c_void,
    _numOptions: ::std::os::raw::c_uint,
    _options: *mut CUjit_option,
    _optionValues: *mut *mut ::std::os::raw::c_void,
    fn_logger: &mut log::FunctionLogger,
    state: &mut trace::StateTracker,
    result: CUresult,
) {
    cuModuleLoadData_Post(module, raw_image, fn_logger, state, result)
}

#[allow(non_snake_case)]
pub(crate) fn cuGetExportTable_Post(
    ppExportTable: *mut *const ::std::os::raw::c_void,
    pExportTableId: *const CUuuid,
    _fn_logger: &mut log::FunctionLogger,
    state: &mut trace::StateTracker,
    result: CUresult,
) {
    if result.is_err() {
        return;
    }
    dark_api::override_export_table(ppExportTable, pExportTableId, state)
}

#[allow(non_snake_case)]
pub(crate) fn cuModuleGetFunction_Post(
    _hfunc: *mut CUfunction,
    _hmod: CUmodule,
    _name: *const ::std::os::raw::c_char,
    _fn_logger: &mut log::FunctionLogger,
    _state: &mut trace::StateTracker,
    _result: CUresult,
) {
}

#[allow(non_snake_case)]
pub(crate) fn cuDeviceGetAttribute_Post(
    _pi: *mut ::std::os::raw::c_int,
    _attrib: CUdevice_attribute,
    _dev: CUdevice,
    _fn_logger: &mut log::FunctionLogger,
    _state: &mut trace::StateTracker,
    _result: CUresult,
) {
}

#[allow(non_snake_case)]
pub(crate) fn cuDeviceComputeCapability_Post(
    major: *mut ::std::os::raw::c_int,
    _minor: *mut ::std::os::raw::c_int,
    _dev: CUdevice,
    _fn_logger: &mut log::FunctionLogger,
    state: &mut trace::StateTracker,
    _result: CUresult,
) {
    if let Some(major_ver_override) = state.override_cc_major {
        unsafe { *major = major_ver_override as i32 };
    }
}

#[allow(non_snake_case)]
pub(crate) fn cuModuleLoadFatBinary_Post(
    _module: *mut CUmodule,
    _fatCubin: *const ::std::os::raw::c_void,
    _fn_logger: &mut log::FunctionLogger,
    _state: &mut trace::StateTracker,
    result: CUresult,
) {
    if result.is_ok() {
        panic!()
    }
}
