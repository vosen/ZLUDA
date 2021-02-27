use cuda_types::*;
use log::LogEntry;
use paste::paste;
use std::ffi::{c_void, CStr};
use std::fmt::Display;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::str::FromStr;
use std::{env, error::Error, fs, path::PathBuf, sync::Mutex};
use std::{io, mem, ptr};
use trace::TexrefAddress;
use zluda_dark_api::CUmoduleContent;

#[macro_use]
extern crate lazy_static;
extern crate cuda_types;

macro_rules! extern_redirect {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:path);*) => {
        $(
            #[no_mangle]
            pub extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                let original_fn = |dynamic_fns: &mut crate::CudaDynamicFns| {
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
            pub extern "system" fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                let original_fn = |dynamic_fns: &mut crate::CudaDynamicFns| {
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
                    move |logger, state| paste! { [<$fn_name _Pre>] } ( $( $arg_id ),* , logger, state ),
                    original_fn,
                    get_formatted_args,
                    move |logger, state, pre_result, cuda_result| paste! { [<$fn_name _Post>] } ( $( $arg_id ),* , logger, state, pre_result, cuda_result )
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
        cuModuleGetTexRef,
        cuModuleLoad,
        cuModuleLoadData,
        cuModuleLoadDataEx,
        cuGetExportTable,
        cuModuleGetFunction,
        cuDeviceGetAttribute,
        cuDeviceComputeCapability,
        cuModuleLoadFatBinary,
        cuLaunchKernel,
        cuTexRefSetAddress_v2,
        cuTexRefSetAddress2D_v2,
        cuTexRefSetAddress2D_v3,
        cuTexRefSetArray,
        cuModuleGetGlobal_v2,
        cuGetProcAddress,
        cuGetProcAddress_v2,
        cuLinkAddData_v2,
        cuLibraryLoadData,
        cuLibraryGetModule
    ]
);

mod dark_api;
mod format;
mod log;
#[cfg_attr(windows, path = "os_win.rs")]
#[cfg_attr(not(windows), path = "os_unix.rs")]
mod os;
mod profiler;
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
    _settings: Settings,
    libcuda: CudaDynamicFns,
    cuda_state: trace::StateTracker,
    pub(crate) side_by_side: Option<side_by_side::SideBySide>,
    pub(crate) profiler: Option<profiler::Profiler>,
}

impl GlobalDelayedState {
    fn new<'a>(
        func: &'static str,
        arguments_writer: Box<dyn FnMut(&mut dyn std::io::Write) -> std::io::Result<()>>,
        factory: &'a mut log::Factory,
    ) -> (LateInit<Self>, log::FunctionLogger<'a>) {
        let (mut fn_logger, settings) =
            factory.get_first_logger_and_init_settings(func, arguments_writer);
        let mut libcuda = match unsafe { CudaDynamicFns::load_library(&settings.libcuda_path) } {
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
        let side_by_side = side_by_side_lib
            .map(|lib| {
                side_by_side::SideBySide::new(
                    lib,
                    &mut fn_logger,
                    settings.side_by_side_skip_kernel.as_ref(),
                    settings.side_by_side_dump_threshold,
                )
            })
            .flatten();
        let profiler = profiler::Profiler::new(&settings, &mut libcuda, &mut fn_logger);
        let delayed_state = GlobalDelayedState {
            _settings: settings,
            libcuda,
            cuda_state,
            side_by_side,
            profiler,
        };
        (LateInit::Success(delayed_state), fn_logger)
    }
}

struct Settings {
    dump_dir: Option<PathBuf>,
    libcuda_path: String,
    log_enabled: bool,
    override_cc_major: Option<u32>,
    side_by_side_path: Option<String>,
    side_by_side_skip_kernel: Option<String>,
    side_by_side_dump_threshold: Option<f32>,
    profiler_output: Option<String>,
}

impl Settings {
    fn read_and_init(log_enabled: bool, logger: &mut log::FunctionLogger) -> Self {
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
        let mut report_err = |e| logger.log(e);
        let libcuda_path: String = parse_env_var("ZLUDA_CUDA_LIB", &mut report_err)
            .unwrap_or_else(|| os::LIBCUDA_DEFAULT_PATH.to_owned());
        let override_cc_major =
            parse_env_var::<u32, _>("ZLUDA_OVERRIDE_COMPUTE_CAPABILITY_MAJOR", &mut report_err);
        let side_by_side_path =
            parse_env_var::<String, _>("ZLUDA_SIDE_BY_SIDE_LIB", &mut report_err);
        let side_by_side_skip_kernel =
            parse_env_var::<String, _>("ZLUDA_SIDE_BY_SIDE_SKIP_KERNEL", &mut report_err);
        let side_by_side_dump_threshold =
            parse_env_var::<f32, _>("ZLUDA_SIDE_BY_DUMP_THRESHOLD", &mut report_err);
        let profiler_output = parse_env_var::<String, _>("ZLUDA_PROFILER_OUTPUT", &mut report_err);
        Settings {
            dump_dir,
            log_enabled,
            libcuda_path,
            override_cc_major,
            side_by_side_path,
            side_by_side_skip_kernel,
            side_by_side_dump_threshold,
            profiler_output,
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

fn parse_env_var<T: FromStr, FnErr>(key: &'static str, report_err: &mut FnErr) -> Option<T>
where
    T::Err: Display + 'static,
    FnErr: FnMut(log::LogEntry),
{
    let value = match env::var(key) {
        Ok(env_var) => env_var,
        Err(env::VarError::NotPresent) => return None,
        Err(err) => {
            report_err(LogEntry::EnvVarError(err));
            return None;
        }
    };
    match value.parse::<T>() {
        Ok(value) => Some(value),
        Err(err) => {
            let err = Box::new(err);
            report_err(LogEntry::MalformedEnvVar { key, value, err });
            return None;
        }
    }
}

fn handle_cuda_function_call(
    func: &'static str,
    original_cuda_fn: impl FnOnce(&mut CudaDynamicFns) -> Option<CUresult>,
    arguments_writer: Box<dyn FnMut(&mut dyn std::io::Write) -> std::io::Result<()>>,
) -> CUresult {
    handle_cuda_function_call_with_probes(
        func,
        |_, _| (),
        original_cuda_fn,
        arguments_writer,
        |_, _, _, _| (),
    )
}

fn handle_cuda_function_call_with_probes<T, PreFn, PostFn>(
    func: &'static str,
    pre_probe: PreFn,
    original_cuda_fn: impl FnOnce(&mut CudaDynamicFns) -> Option<CUresult>,
    arguments_writer: Box<dyn FnMut(&mut dyn std::io::Write) -> std::io::Result<()>>,
    post_probe: PostFn,
) -> CUresult
where
    for<'a> PreFn: FnOnce(&'a mut log::FunctionLogger, &'a mut GlobalDelayedState) -> T,
    for<'a> PostFn: FnOnce(&'a mut log::FunctionLogger, &'a mut GlobalDelayedState, T, CUresult),
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
    let pre_result = pre_probe(&mut logger, delayed_state);
    let maybe_cu_result = {
        let task = delayed_state.profiler.as_ref().map(|p| p.record_task(func));
        if task.is_some() && func == "cuDevicePrimaryCtxReset" {
            Some(CUresult::CUDA_SUCCESS)
        } else {
            original_cuda_fn(&mut delayed_state.libcuda)
        }
    };
    let cu_result = match maybe_cu_result {
        Some(result) => result,
        None => {
            logger.log(log::LogEntry::ErrorBox(
                format!("No function {} in the underlying CUDA library", func).into(),
            ));
            CUresult::CUDA_ERROR_UNKNOWN
        }
    };
    logger.result = maybe_cu_result;
    post_probe(&mut logger, delayed_state, pre_result, cu_result);
    cu_result
}

#[allow(non_snake_case)]
pub(crate) fn cuModuleLoad_Pre(
    _module: *mut CUmodule,
    _fname: *const ::std::os::raw::c_char,
    _fn_logger: &mut log::FunctionLogger,
    _state: &mut GlobalDelayedState,
) {
}

#[allow(non_snake_case)]
pub(crate) fn cuModuleLoad_Post(
    module: *mut CUmodule,
    fname: *const ::std::os::raw::c_char,
    fn_logger: &mut log::FunctionLogger,
    state: &mut GlobalDelayedState,
    _pre_result: (),
    result: CUresult,
) {
    if result != CUresult::CUDA_SUCCESS {
        return;
    }
    state.cuda_state.record_module(
        fn_logger,
        Some(unsafe { *module }),
        CUmoduleContent::File(fname),
    );
}

#[allow(non_snake_case)]
pub(crate) fn cuModuleLoadData_Pre(
    _module: *mut CUmodule,
    _raw_image: *const ::std::os::raw::c_void,
    _fn_logger: &mut log::FunctionLogger,
    _state: &mut GlobalDelayedState,
) {
}

#[allow(non_snake_case)]
pub(crate) fn cuModuleLoadData_Post(
    module: *mut CUmodule,
    raw_image: *const ::std::os::raw::c_void,
    fn_logger: &mut log::FunctionLogger,
    state: &mut GlobalDelayedState,
    _pre_result: (),
    result: CUresult,
) {
    if result != CUresult::CUDA_SUCCESS {
        return;
    }
    if let Some(module_content) =
        fn_logger.log_unwrap(unsafe { CUmoduleContent::from_ptr(raw_image.cast()) }.map_err(LogEntry::from))
    {
        state
            .cuda_state
            .record_module(fn_logger, Some(unsafe { *module }), module_content);
    }
}

#[allow(non_snake_case)]
pub(crate) fn cuModuleLoadDataEx_Pre(
    _module: *mut CUmodule,
    _raw_image: *const ::std::os::raw::c_void,
    _numOptions: ::std::os::raw::c_uint,
    _options: *mut CUjit_option,
    _optionValues: *mut *mut ::std::os::raw::c_void,
    _fn_logger: &mut log::FunctionLogger,
    _state: &mut GlobalDelayedState,
) {
}

#[allow(non_snake_case)]
pub(crate) fn cuModuleLoadDataEx_Post(
    module: *mut CUmodule,
    raw_image: *const ::std::os::raw::c_void,
    _numOptions: ::std::os::raw::c_uint,
    _options: *mut CUjit_option,
    _optionValues: *mut *mut ::std::os::raw::c_void,
    fn_logger: &mut log::FunctionLogger,
    state: &mut GlobalDelayedState,
    pre_result: (),
    result: CUresult,
) {
    cuModuleLoadData_Post(module, raw_image, fn_logger, state, pre_result, result)
}

#[allow(non_snake_case)]
pub(crate) fn cuGetExportTable_Pre(
    _ppExportTable: *mut *const ::std::os::raw::c_void,
    _pExportTableId: *const CUuuid,
    _fn_logger: &mut log::FunctionLogger,
    _state: &mut GlobalDelayedState,
) {
}

#[allow(non_snake_case)]
pub(crate) fn cuGetExportTable_Post(
    ppExportTable: *mut *const ::std::os::raw::c_void,
    pExportTableId: *const CUuuid,
    fn_logger: &mut log::FunctionLogger,
    state: &mut GlobalDelayedState,
    _pre_result: (),
    result: CUresult,
) {
    if result != CUresult::CUDA_SUCCESS {
        return;
    }
    dark_api::override_export_table(
        fn_logger,
        ppExportTable,
        pExportTableId,
        &mut state.cuda_state,
    )
}

#[allow(non_snake_case)]
pub(crate) fn cuModuleGetFunction_Pre(
    _hfunc: *mut CUfunction,
    _hmod: CUmodule,
    _name: *const ::std::os::raw::c_char,
    _fn_logger: &mut log::FunctionLogger,
    _state: &mut GlobalDelayedState,
) {
}

#[allow(non_snake_case)]
pub(crate) fn cuModuleGetFunction_Post(
    hfunc: *mut CUfunction,
    hmod: CUmodule,
    name: *const ::std::os::raw::c_char,
    fn_logger: &mut log::FunctionLogger,
    state: &mut GlobalDelayedState,
    _pre_result: (),
    result: CUresult,
) {
    if result != CUresult::CUDA_SUCCESS {
        return;
    }
    unsafe {
        state
            .cuda_state
            .record_function(*hfunc, hmod, name, fn_logger)
    }
}

#[allow(non_snake_case)]
pub(crate) fn cuDeviceGetAttribute_Pre(
    _pi: *mut ::std::os::raw::c_int,
    _attrib: CUdevice_attribute,
    _dev: CUdevice,
    _fn_logger: &mut log::FunctionLogger,
    _state: &mut GlobalDelayedState,
) {
}

#[allow(non_snake_case)]
pub(crate) fn cuDeviceGetAttribute_Post(
    pi: *mut ::std::os::raw::c_int,
    attrib: CUdevice_attribute,
    _dev: CUdevice,
    _fn_logger: &mut log::FunctionLogger,
    state: &mut GlobalDelayedState,
    _pre_result: (),
    _result: CUresult,
) {
    if attrib == CUdevice_attribute::CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR {
        if let Some(major_ver_override) = state.cuda_state.override_cc_major {
            unsafe { *pi = major_ver_override as i32 };
        }
    }
}

#[allow(non_snake_case)]
pub(crate) fn cuDeviceComputeCapability_Pre(
    _major: *mut ::std::os::raw::c_int,
    _minor: *mut ::std::os::raw::c_int,
    _dev: CUdevice,
    _fn_logger: &mut log::FunctionLogger,
    _state: &mut GlobalDelayedState,
) {
}

#[allow(non_snake_case)]
pub(crate) fn cuDeviceComputeCapability_Post(
    major: *mut ::std::os::raw::c_int,
    _minor: *mut ::std::os::raw::c_int,
    _dev: CUdevice,
    _fn_logger: &mut log::FunctionLogger,
    state: &mut GlobalDelayedState,
    _pre_result: (),
    _result: CUresult,
) {
    if let Some(major_ver_override) = state.cuda_state.override_cc_major {
        unsafe { *major = major_ver_override as i32 };
    }
}

#[allow(non_snake_case)]
pub(crate) fn cuModuleLoadFatBinary_Pre(
    _module: *mut CUmodule,
    _fatCubin: *const ::std::os::raw::c_void,
    _fn_logger: &mut log::FunctionLogger,
    _state: &mut GlobalDelayedState,
) {
}

#[allow(non_snake_case)]
pub(crate) fn cuModuleLoadFatBinary_Post(
    _module: *mut CUmodule,
    _fatCubin: *const ::std::os::raw::c_void,
    _fn_logger: &mut log::FunctionLogger,
    _state: &mut GlobalDelayedState,
    _pre_result: (),
    result: CUresult,
) {
    if result == CUresult::CUDA_SUCCESS {
        panic!()
    }
}

#[allow(non_snake_case)]
pub(crate) fn cuLaunchKernel_Pre(
    f: CUfunction,
    _gridDimX: ::std::os::raw::c_uint,
    _gridDimY: ::std::os::raw::c_uint,
    _gridDimZ: ::std::os::raw::c_uint,
    _blockDimX: ::std::os::raw::c_uint,
    _blockDimY: ::std::os::raw::c_uint,
    _blockDimZ: ::std::os::raw::c_uint,
    _sharedMemBytes: ::std::os::raw::c_uint,
    stream: CUstream,
    kernelParams: *mut *mut ::std::os::raw::c_void,
    extra: *mut *mut ::std::os::raw::c_void,
    fn_logger: &mut log::FunctionLogger,
    state: &mut GlobalDelayedState,
) -> (Option<side_by_side::HostArguments>, Option<CUevent>) {
    let side_by_side_args = unsafe {
        side_by_side::pre_kernel_launch(
            &mut state.libcuda,
            &mut state.cuda_state,
            &mut state.side_by_side,
            fn_logger,
            f,
            stream,
            kernelParams,
            extra,
        )
    };
    let start_event = if state.profiler.is_some() {
        fn_logger.log_unwrap(record_event(stream, &mut state.libcuda))
    } else {
        None
    };
    (side_by_side_args, start_event)
}

// TODO: stop leaking CUevent on failure
fn record_event(stream: CUstream, libcuda: &mut CudaDynamicFns) -> Result<CUevent, LogEntry> {
    let mut event = ptr::null_mut();
    cuda_call!(libcuda.cuEventCreate(&mut event, 0));
    cuda_call!(libcuda.cuEventRecord(event, stream));
    Ok(event)
}

#[allow(non_snake_case)]
pub(crate) fn cuLaunchKernel_Post(
    f: CUfunction,
    gridDimX: ::std::os::raw::c_uint,
    gridDimY: ::std::os::raw::c_uint,
    gridDimZ: ::std::os::raw::c_uint,
    blockDimX: ::std::os::raw::c_uint,
    blockDimY: ::std::os::raw::c_uint,
    blockDimZ: ::std::os::raw::c_uint,
    sharedMemBytes: ::std::os::raw::c_uint,
    stream: CUstream,
    kernelParams: *mut *mut ::std::os::raw::c_void,
    extra: *mut *mut ::std::os::raw::c_void,
    fn_logger: &mut log::FunctionLogger,
    state: &mut GlobalDelayedState,
    pre_result: (Option<side_by_side::HostArguments>, Option<CUevent>),
    result: CUresult,
) {
    if result != CUresult::CUDA_SUCCESS {
        return;
    }
    let (side_by_side_args, start_event) = pre_result;
    if let Some(start_event) = start_event {
        if let Some(end_event) = fn_logger.log_unwrap(record_event(stream, &mut state.libcuda)) {
            let func_name = match state.cuda_state.functions.get(&f) {
                Some(recorded_func) => profiler::FunctionName::Resolved(recorded_func.name.clone()),
                None => profiler::FunctionName::Unresolved(f),
            };
            state.profiler.as_ref().unwrap().record_kernel(
                stream,
                func_name,
                start_event,
                end_event,
            );
        }
    }
    unsafe {
        side_by_side::post_kernel_launch(
            &mut state.libcuda,
            &mut state.cuda_state,
            &mut state.side_by_side,
            fn_logger,
            side_by_side_args,
            f,
            gridDimX,
            gridDimY,
            gridDimZ,
            blockDimX,
            blockDimY,
            blockDimZ,
            sharedMemBytes,
            stream,
            kernelParams,
            extra,
        )
    }
    .unwrap_or_default()
}

#[allow(non_snake_case)]
pub(crate) fn cuTexRefSetAddress_v2_Pre(
    _ByteOffset: *mut usize,
    _hTexRef: CUtexref,
    _dptr: CUdeviceptr,
    _bytes: usize,
    _fn_logger: &mut log::FunctionLogger,
    _state: &mut GlobalDelayedState,
) -> () {
}

#[allow(non_snake_case)]
pub(crate) fn cuTexRefSetAddress_v2_Post(
    _ByteOffset: *mut usize,
    hTexRef: CUtexref,
    pointer: CUdeviceptr,
    bytes: usize,
    fn_logger: &mut log::FunctionLogger,
    state: &mut GlobalDelayedState,
    _pre_result: (),
    result: CUresult,
) -> () {
    if result != CUresult::CUDA_SUCCESS {
        return;
    }
    if pointer.0 == ptr::null_mut() {
        state.cuda_state.remove_texref_address(fn_logger, hTexRef);
    } else {
        state.cuda_state.record_texref_address(
            fn_logger,
            hTexRef,
            Some(TexrefAddress::OneD { pointer, bytes }),
        )
    }
}

#[allow(non_snake_case)]
pub(crate) fn cuTexRefSetAddress2D_v2_Pre(
    _hTexRef: CUtexref,
    _desc: *const CUDA_ARRAY_DESCRIPTOR,
    _dptr: CUdeviceptr,
    _Pitch: usize,
    _fn_logger: &mut log::FunctionLogger,
    _state: &mut GlobalDelayedState,
) -> () {
}

#[allow(non_snake_case)]
pub(crate) fn cuTexRefSetAddress2D_v3_Pre(
    _hTexRef: CUtexref,
    _desc: *const CUDA_ARRAY_DESCRIPTOR,
    _dptr: CUdeviceptr,
    _Pitch: usize,
    _fn_logger: &mut log::FunctionLogger,
    _state: &mut GlobalDelayedState,
) {
}

#[allow(non_snake_case)]
pub(crate) fn cuTexRefSetAddress2D_v2_Post(
    hTexRef: CUtexref,
    desc: *const CUDA_ARRAY_DESCRIPTOR,
    dptr: CUdeviceptr,
    pitch: usize,
    fn_logger: &mut log::FunctionLogger,
    state: &mut GlobalDelayedState,
    _pre_result: (),
    result: CUresult,
) -> () {
    if result != CUresult::CUDA_SUCCESS {
        return;
    }
    let desc = unsafe { *desc };
    state.cuda_state.record_texref_address(
        fn_logger,
        hTexRef,
        Some(TexrefAddress::new_2d(desc, dptr, pitch)),
    )
}

#[allow(non_snake_case)]
pub(crate) fn cuTexRefSetAddress2D_v3_Post(
    hTexRef: CUtexref,
    desc: *const CUDA_ARRAY_DESCRIPTOR,
    dptr: CUdeviceptr,
    Pitch: usize,
    fn_logger: &mut log::FunctionLogger,
    state: &mut GlobalDelayedState,
    pre_result: (),
    result: CUresult,
) {
    cuTexRefSetAddress2D_v2_Post(
        hTexRef, desc, dptr, Pitch, fn_logger, state, pre_result, result,
    )
}

#[allow(non_snake_case)]
pub(crate) fn cuModuleGetTexRef_Pre(
    _pTexRef: *mut CUtexref,
    _hmod: CUmodule,
    _name: *const ::std::os::raw::c_char,
    _fn_logger: &mut log::FunctionLogger,
    _state: &mut GlobalDelayedState,
) {
}

#[allow(non_snake_case)]
pub(crate) fn cuModuleGetTexRef_Post(
    pTexRef: *mut CUtexref,
    hmod: CUmodule,
    name: *const ::std::os::raw::c_char,
    fn_logger: &mut log::FunctionLogger,
    state: &mut GlobalDelayedState,
    _pre_result: (),
    result: CUresult,
) {
    if result != CUresult::CUDA_SUCCESS {
        return;
    }
    state
        .cuda_state
        .record_texref(fn_logger, name, unsafe { *pTexRef }, hmod)
}

#[allow(non_snake_case)]
pub(crate) fn cuTexRefSetArray_Pre(
    _hTexRef: CUtexref,
    _hArray: CUarray,
    _Flags: ::std::os::raw::c_uint,
    _fn_logger: &mut log::FunctionLogger,
    _state: &mut GlobalDelayedState,
) {
}

#[allow(non_snake_case)]
pub(crate) fn cuTexRefSetArray_Post(
    texref: CUtexref,
    array: CUarray,
    flags: ::std::os::raw::c_uint,
    fn_logger: &mut log::FunctionLogger,
    state: &mut GlobalDelayedState,
    _pre_result: (),
    result: CUresult,
) {
    if result != CUresult::CUDA_SUCCESS {
        return;
    }
    state.cuda_state.record_texref_address(
        fn_logger,
        texref,
        Some(TexrefAddress::Array { array, flags }),
    )
}

#[allow(non_snake_case)]
pub(crate) fn cuModuleGetGlobal_v2_Pre(
    _dptr: *mut CUdeviceptr,
    _bytes: *mut usize,
    _hmod: CUmodule,
    _name: *const ::std::os::raw::c_char,
    _fn_logger: &mut log::FunctionLogger,
    _state: &mut GlobalDelayedState,
) {
}

#[allow(non_snake_case)]
pub(crate) fn cuModuleGetGlobal_v2_Post(
    dptr: *mut CUdeviceptr,
    _bytes: *mut usize,
    hmod: CUmodule,
    name: *const ::std::os::raw::c_char,
    fn_logger: &mut log::FunctionLogger,
    state: &mut GlobalDelayedState,
    _pre_result: (),
    result: CUresult,
) {
    if result != CUresult::CUDA_SUCCESS {
        return;
    }
    if dptr == ptr::null_mut() {
        return;
    }
    // We don't collect byte size, because some applications call this function
    // exclusively with `bytes` == NULL
    state
        .cuda_state
        .record_global(fn_logger, hmod, name, unsafe { *dptr })
}

#[allow(non_snake_case)]
pub(crate) fn cuGetProcAddress_Pre(
    _symbol: *const ::std::os::raw::c_char,
    _pfn: *mut *mut ::std::os::raw::c_void,
    _cudaVersion: ::std::os::raw::c_int,
    _flags: cuuint64_t,
    _fn_logger: &mut log::FunctionLogger,
    _state: &mut GlobalDelayedState,
) {
}

#[allow(non_snake_case)]
pub(crate) fn cuGetProcAddress_Post(
    symbol: *const ::std::os::raw::c_char,
    result_value: *mut *mut ::std::os::raw::c_void,
    cudaVersion: ::std::os::raw::c_int,
    flags: cuuint64_t,
    fn_logger: &mut log::FunctionLogger,
    _state: &mut GlobalDelayedState,
    _pre_result: (),
    result_code: CUresult,
) {
    if result_code != CUresult::CUDA_SUCCESS {
        return;
    }
    let symbol = unsafe { CStr::from_ptr(symbol) };
    let name = symbol.to_bytes();
    let version = cudaVersion as u32;
    let fn_ptr = get_proc_address(name, flags, version);
    if fn_ptr == ptr::null_mut() || fn_ptr == usize::MAX as _ {
        fn_logger.log(LogEntry::NoCudaFunction(
            symbol.to_string_lossy().to_owned(),
        ));
    } else {
        unsafe { *result_value = fn_ptr };
    }
}

#[allow(non_snake_case)]
pub(crate) fn cuGetProcAddress_v2_Pre(
    _symbol: *const ::std::os::raw::c_char,
    _pfn: *mut *mut ::std::os::raw::c_void,
    _cudaVersion: ::std::os::raw::c_int,
    _flags: cuuint64_t,
    _symbolStatus: *mut CUdriverProcAddressQueryResult,
    _fn_logger: &mut log::FunctionLogger,
    _state: &mut GlobalDelayedState,
) {
}

#[allow(non_snake_case)]
pub(crate) fn cuGetProcAddress_v2_Post(
    symbol: *const ::std::os::raw::c_char,
    result_value: *mut *mut ::std::os::raw::c_void,
    cudaVersion: ::std::os::raw::c_int,
    flags: cuuint64_t,
    _symbolStatus: *mut CUdriverProcAddressQueryResult,
    fn_logger: &mut log::FunctionLogger,
    _state: &mut GlobalDelayedState,
    _pre_result: (),
    result_code: CUresult,
) {
    if result_code != CUresult::CUDA_SUCCESS {
        return;
    }
    let symbol = unsafe { CStr::from_ptr(symbol) };
    let name = symbol.to_bytes();
    let version = cudaVersion as u32;
    let fn_ptr = get_proc_address(name, flags, version);
    if fn_ptr == ptr::null_mut() || fn_ptr == usize::MAX as _ {
        fn_logger.log(LogEntry::NoCudaFunction(
            symbol.to_string_lossy().to_owned(),
        ));
    } else {
        unsafe { *result_value = fn_ptr };
    }
}

#[allow(non_snake_case)]
pub(crate) fn cuLinkAddData_v2_Pre(
    _link_state: CUlinkState,
    _type_: CUjitInputType,
    _data: *mut ::std::os::raw::c_void,
    _size: usize,
    _name: *const ::std::os::raw::c_char,
    _numOptions: ::std::os::raw::c_uint,
    _options: *mut CUjit_option,
    _optionValues: *mut *mut ::std::os::raw::c_void,
    _fn_logger: &mut log::FunctionLogger,
    _state: &mut GlobalDelayedState,
) {
}

#[allow(non_snake_case)]
pub(crate) fn cuLinkAddData_v2_Post(
    _link_state: CUlinkState,
    type_: CUjitInputType,
    data: *mut ::std::os::raw::c_void,
    size: usize,
    _name: *const ::std::os::raw::c_char,
    _numOptions: ::std::os::raw::c_uint,
    _options: *mut CUjit_option,
    _optionValues: *mut *mut ::std::os::raw::c_void,
    fn_logger: &mut log::FunctionLogger,
    state: &mut GlobalDelayedState,
    _pre_result: (),
    result: CUresult,
) {
    if result != CUresult::CUDA_SUCCESS {
        return;
    }
    let module = match type_ {
        CUjitInputType::CU_JIT_INPUT_CUBIN => CUmoduleContent::Elf(data.cast()),
        CUjitInputType::CU_JIT_INPUT_PTX => CUmoduleContent::RawText(data.cast()),
        CUjitInputType::CU_JIT_INPUT_LIBRARY => {
            let data_slice = unsafe { std::slice::from_raw_parts(data.cast::<u8>(), size) };
            CUmoduleContent::Archive(data_slice)
        }
        _ => return,
    };
    state.cuda_state.record_module(fn_logger, None, module);
}

#[allow(non_snake_case)]
pub(crate) fn cuLibraryGetModule_Pre(
    _pMod: *mut CUmodule,
    _library: CUlibrary,
    _fn_logger: &mut log::FunctionLogger,
    _state: &mut GlobalDelayedState,
) {
}

#[allow(non_snake_case)]
pub(crate) fn cuLibraryGetModule_Post(
    _pMod: *mut CUmodule,
    _library: CUlibrary,
    _fn_logger: &mut log::FunctionLogger,
    _state: &mut GlobalDelayedState,
    _pre_result: (),
    _result: CUresult,
) {
}

#[allow(non_snake_case)]
pub(crate) fn cuLibraryLoadData_Pre(
    _library: *mut CUlibrary,
    _code: *const ::std::os::raw::c_void,
    _jitOptions: *mut CUjit_option,
    _jitOptionsValues: *mut *mut ::std::os::raw::c_void,
    _numJitOptions: ::std::os::raw::c_uint,
    _libraryOptions: *mut CUlibraryOption,
    _libraryOptionValues: *mut *mut ::std::os::raw::c_void,
    _numLibraryOptions: ::std::os::raw::c_uint,
    _fn_logger: &mut log::FunctionLogger,
    _state: &mut GlobalDelayedState,
) {
}

#[allow(non_snake_case)]
pub(crate) fn cuLibraryLoadData_Post(
    _library: *mut CUlibrary,
    code: *const ::std::os::raw::c_void,
    _jitOptions: *mut CUjit_option,
    _jitOptionsValues: *mut *mut ::std::os::raw::c_void,
    _numJitOptions: ::std::os::raw::c_uint,
    _libraryOptions: *mut CUlibraryOption,
    _libraryOptionValues: *mut *mut ::std::os::raw::c_void,
    _numLibraryOptions: ::std::os::raw::c_uint,
    fn_logger: &mut log::FunctionLogger,
    state: &mut GlobalDelayedState,
    _pre_result: (),
    result: CUresult,
) {
    if !matches!(
        result,
        CUresult::CUDA_SUCCESS
            | CUresult::CUDA_ERROR_INVALID_PTX
            | CUresult::CUDA_ERROR_NOT_SUPPORTED
    ) {
        return;
    }
    if let Some(module_content) =
        fn_logger.log_unwrap(unsafe { CUmoduleContent::from_ptr(code.cast()) }.map_err(LogEntry::from))
    {
        state
            .cuda_state
            .record_module(fn_logger, None, module_content);
    }
}

fn get_proc_address(name: &[u8], flag: u64, version: u32) -> *mut ::std::os::raw::c_void {
    include!("../../process_address_table/table.rs")
}

pub(crate) struct DynamicFn<T> {
    pointer: usize,
    _marker: PhantomData<T>,
}

impl<T> Default for DynamicFn<T> {
    fn default() -> Self {
        DynamicFn {
            pointer: 0,
            _marker: PhantomData,
        }
    }
}

impl<T> DynamicFn<T> {
    pub(crate) unsafe fn get(&mut self, lib: *mut c_void, name: &[u8]) -> Option<T> {
        match self.pointer {
            0 => {
                let addr = os::get_proc_address(lib, CStr::from_bytes_with_nul_unchecked(name));
                if addr == ptr::null_mut() {
                    self.pointer = 1;
                    return None;
                } else {
                    self.pointer = addr as _;
                }
            }
            1 => return None,
            _ => {}
        }
        Some(mem::transmute_copy(&self.pointer))
    }
}

pub(crate) struct CudaDynamicFns {
    pub(crate) lib_handle: NonNull<::std::ffi::c_void>,
    pub(crate) fn_table: CudaFnTable,
}

#[macro_export]
macro_rules! try_get_cuda_function {
    ($libcuda:expr, $name:ident) => {
        unsafe {
            let libcuda: &mut _ = $libcuda;
            libcuda
                .fn_table
                .$name
                .get(
                    libcuda.lib_handle.as_ptr(),
                    concat!(stringify!($name), "\0").as_bytes(),
                )
                .ok_or(LogEntry::NoCudaFunction(std::borrow::Cow::Borrowed(
                    stringify!($name),
                )))
        }
    };
}

impl CudaDynamicFns {
    pub(crate) unsafe fn load_library(path: &str) -> Option<Self> {
        let lib_handle = NonNull::new(os::load_library(path));
        lib_handle.map(|lib_handle| CudaDynamicFns {
            lib_handle,
            fn_table: CudaFnTable::default(),
        })
    }
}

macro_rules! emit_cuda_fn_table {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:path);*) => {
        #[derive(Default)]
        pub(crate) struct CudaFnTable {
            pub(crate) $($fn_name: DynamicFn<extern $abi fn ( $($arg_id : $arg_type),* ) -> $ret_type>),*
        }

        impl CudaDynamicFns {
            $(
                #[allow(dead_code)]
                pub(crate) fn $fn_name(&mut self, $($arg_id : $arg_type),*) -> Option<$ret_type> {
                    let func = unsafe { self.fn_table.$fn_name.get(self.lib_handle.as_ptr(), concat!(stringify!($fn_name), "\0").as_bytes()) };
                    func.map(|f| f($($arg_id),*) )
                }
            )*
        }
    };
}

cuda_function_declarations!(cuda_types, emit_cuda_fn_table, emit_cuda_fn_table, []);

#[macro_export]
macro_rules! cuda_call {
    ($fn_table:ident . $fn_:ident ( $($arg:expr),* ) ) => {
        {
            {
                match $fn_table . $fn_ ( $($arg),* ) {
                    None => return Err(LogEntry::NoCudaFunction(std::borrow::Cow::Borrowed(stringify!($fn_)))),
                    Some(cuda_types::CUresult::CUDA_SUCCESS) => (),
                    Some(err) => return Err(LogEntry::CudaError(err))
                }
            }
        }
    };

    ($fn_:ident ( $($arg:expr),* ) ) => {
        {
            {
                match $fn_ ( $($arg),* ) {
                    cuda_types::CUresult::CUDA_SUCCESS => (),
                    err => return Err(LogEntry::CudaError(err))
                }
            }
        }
    };
}
