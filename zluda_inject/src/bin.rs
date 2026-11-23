use crate::args;
use crate::args::{Arguments, LibraryWithPath};
use rustc_hash::FxHashSet;
use std::error::Error;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::{env, mem, process, ptr};
use windows::Win32::Foundation::{HANDLE, WAIT_FAILED};
use windows::Win32::System::Threading::{
    self, GetExitCodeProcess, ResumeThread, WaitForSingleObject, CREATE_DEFAULT_ERROR_MODE,
    CREATE_SUSPENDED, CREATE_UNICODE_ENVIRONMENT, DETACHED_PROCESS, PROCESS_CREATION_FLAGS,
};
use zluda_windows::LibraryInfo;

macro_rules! os_call {
    ($($path:ident)::+ ($($args:expr),*)) => {
        {
            os_call!($($path)::+ ($($args),*), |x| x == 0)
        }
    };
    ($($path:ident)::+ ($($args:expr),*), $failure:expr) => {
        {
            let result = unsafe{ $($path)::+ ($($args),*) };
            if $failure(result) {
                let name = last_ident!($($path),+);
                let err_code = unsafe { ::windows::Win32::Foundation::GetLastError() };
                Err($crate::win::OsError{
                    function: name,
                    error_code: err_code.0,
                    message: $crate::win::error_string(err_code.0 as i32)
                })?;
            }
            result
        }
    };
}

pub fn main_impl(args: args::Arguments, detached: bool) -> Result<(), Box<dyn Error>> {
    let mut command_line = args.command_line_zero_terminated();
    let injection_env = InjectionConfig::new(args)?;
    let mut startup_info = unsafe { mem::zeroed::<detours_sys::_STARTUPINFOW>() };
    let mut proc_info = unsafe { mem::zeroed::<detours_sys::_PROCESS_INFORMATION>() };
    let mut env_vars_block = injection_env.get_env_variables_block()?;
    let mut redirect_dll_path_ptr = injection_env.redirect_dll_path.as_ptr().cast::<i8>();
    os_call! {
        detours_sys::DetourCreateProcessWithDllsW(
            ptr::null_mut(),
            command_line.as_mut_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
            (!detached) as i32,
            // Without DETACHED_PROCESS blender pops up a console window
            (CREATE_SUSPENDED | CREATE_UNICODE_ENVIRONMENT | CREATE_DEFAULT_ERROR_MODE | if detached { DETACHED_PROCESS } else { PROCESS_CREATION_FLAGS(0) }).0,
            env_vars_block.as_mut_ptr().cast(),
            ptr::null(),
            std::ptr::from_mut(&mut startup_info),
            std::ptr::from_mut(&mut proc_info),
            1,
            &mut redirect_dll_path_ptr,
            Option::None
        )
    };
    zluda_windows::kill_child_on_process_exit(HANDLE(proc_info.hProcess))?;
    injection_env.copy_payloads_to_process(proc_info.hProcess)?;
    os_call! { ResumeThread(HANDLE(proc_info.hThread)), |x| x == -1i32 as u32 };
    os_call! { WaitForSingleObject(HANDLE(proc_info.hProcess), Threading::INFINITE), |x| x == WAIT_FAILED };
    let mut child_exit_code: u32 = 0;
    unsafe { GetExitCodeProcess(HANDLE(proc_info.hProcess), &mut child_exit_code as *mut _) }?;
    process::exit(child_exit_code as i32)
}

#[derive(Debug)]
struct InjectionConfig {
    dll_paths32: Vec<Option<LibraryWithPath>>,
    dll_paths64: Vec<Option<LibraryWithPath>>,
    redirect_dll_path: String,
    env_vars: Vec<(&'static str, PathBuf)>,
}

impl InjectionConfig {
    fn new(args: Arguments) -> Result<Self, Box<dyn Error>> {
        match args.paths {
            args::ConfigSet::Custom {
                dll_paths32,
                dll_paths64,
            } => Self::custom(dll_paths32, dll_paths64),
            args::ConfigSet::Zluda => Self::zluda(),
            args::ConfigSet::ZludaTrace => Self::zluda_trace(),
            args::ConfigSet::NvidiaTrace => Self::nvidia_trace(),
        }
    }

    fn custom(
        dll_paths32: Vec<Option<LibraryWithPath>>,
        dll_paths64: Vec<Option<LibraryWithPath>>,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            redirect_dll_path: Self::redirect_dll_path(Self::current_exe_dir()?),
            dll_paths32,
            dll_paths64,
            env_vars: Vec::new(),
        })
    }

    fn zluda() -> Result<Self, Box<dyn Error>> {
        let current_exe_dir = Self::current_exe_dir()?;
        let dll_paths32 = Self::get_paths(&current_exe_dir, &zluda_windows::LIBRARIES32[..]);
        let dll_paths64 = Self::get_paths(&current_exe_dir, &zluda_windows::LIBRARIES64[..]);
        Ok(Self {
            redirect_dll_path: Self::redirect_dll_path(current_exe_dir),
            dll_paths32,
            dll_paths64,
            env_vars: Vec::new(),
        })
    }

    fn zluda_trace() -> Result<Self, Box<dyn Error>> {
        let current_exe_dir = Self::current_exe_dir()?;
        let trace_dir = Self::trace_dir()?;
        let libraries = zluda_windows::LIBRARIES32
            .iter()
            .chain(zluda_windows::LIBRARIES64.iter());
        let dll_paths32 = Self::get_paths(&trace_dir, &zluda_windows::LIBRARIES32[..]);
        let dll_paths64 = Self::get_paths(&trace_dir, &zluda_windows::LIBRARIES64[..]);
        let mut env_vars = libraries
            .filter_map(|lib| {
                let redirect = lib.redirect.as_ref()?;
                let mut path = current_exe_dir.clone();
                path.push(redirect.zluda_dll_name);
                Some((redirect.trace_env_var, path))
            })
            .collect::<Vec<_>>();
        let mut temp_dir = std::env::temp_dir();
        temp_dir.push("zluda");
        env_vars.push(("ZLUDA_LOG_DIR", temp_dir));
        Ok(Self {
            redirect_dll_path: Self::redirect_dll_path(current_exe_dir),
            dll_paths32,
            dll_paths64,
            env_vars,
        })
    }

    fn nvidia_trace() -> Result<Self, Box<dyn Error>> {
        let trace_dir = Self::trace_dir()?;
        let cuda_install_lib_path =
            std::env::var("CUDA_PATH")
                .ok()
                .map(PathBuf::from)
                .map(|mut path| {
                    path.push("bin");
                    path.push("x64");
                    path
                });
        let (dll_paths32, env_vars32) = Self::collect_nvidia_libraries(
            &zluda_windows::LIBRARIES32[..],
            &trace_dir,
            &cuda_install_lib_path,
        )?;
        let (dll_paths64, env_vars64) = Self::collect_nvidia_libraries(
            &zluda_windows::LIBRARIES64[..],
            &trace_dir,
            &cuda_install_lib_path,
        )?;
        let mut temp_dir = std::env::temp_dir();
        temp_dir.push("zluda");
        let mut env_vars = Vec::new();
        env_vars.extend(env_vars32);
        env_vars.extend(env_vars64);
        env_vars.push(("ZLUDA_LOG_DIR", temp_dir));
        Ok(Self {
            redirect_dll_path: Self::redirect_dll_path(Self::current_exe_dir()?),
            dll_paths32,
            dll_paths64,
            env_vars,
        })
    }

    fn current_exe_dir() -> Result<PathBuf, Box<dyn Error>> {
        let mut current_exe_dir = env::current_exe()?;
        current_exe_dir.pop();
        Ok(current_exe_dir)
    }

    fn trace_dir() -> Result<PathBuf, Box<dyn Error>> {
        let mut trace_dir = env::current_exe()?;
        trace_dir.pop();
        trace_dir.push("trace");
        Ok(trace_dir)
    }

    fn redirect_dll_path(mut current_exe_dir: PathBuf) -> String {
        current_exe_dir.push("zluda_redirect.dll\0");
        current_exe_dir.to_string_lossy().into_owned()
    }

    fn get_paths(
        base_dir: &PathBuf,
        libraries: &'static [LibraryInfo],
    ) -> Vec<Option<LibraryWithPath>> {
        libraries
            .iter()
            .map(|lib| {
                let zluda_name = lib.redirect.as_ref()?.zluda_dll_name;
                let mut path = base_dir.clone();
                path.push(zluda_name);
                Some(LibraryWithPath { library: lib, path })
            })
            .collect()
    }

    fn copy_payloads_to_process(
        self,
        h_process: *mut std::ffi::c_void,
    ) -> Result<(), Box<dyn Error>> {
        let mut path_block = zluda_windows::detours::pack_paths(
            self.dll_paths32
                .iter()
                .map(|library| Some(library.as_ref()?.path.as_os_str())),
            self.dll_paths64
                .iter()
                .map(|library| Some(library.as_ref()?.path.as_os_str())),
            &self.redirect_dll_path,
        );
        let path_block = bytemuck::cast_slice_mut::<_, u8>(&mut path_block[..]);
        os_call! {
            detours_sys::DetourCopyPayloadToProcess(
                h_process,
                &mem::transmute(zluda_windows::detours::GUID),
                path_block.as_mut_ptr().cast(),
                path_block.len() as u32
            )
        };
        Ok(())
    }

    fn get_env_variables_block(&self) -> Result<Vec<u16>, Box<dyn Error>> {
        use std::fmt::Write;
        let mut result = widestring::U16String::new();
        let mut known_vars = FxHashSet::default();
        for (var, value) in env::vars_os() {
            write!(&mut result, "{}={}\0", var.display(), value.display())?;
            known_vars.insert(value);
        }
        if !known_vars.contains(OsStr::new("TORCH_CUDNN_V8_API_DISABLED")) {
            write!(&mut result, "TORCH_CUDNN_V8_API_DISABLED=1\0")?;
        }
        for (key, value) in self.env_vars.iter() {
            let x: &OsStr = key.as_ref();
            if known_vars.contains(x) {
                continue;
            }
            write!(&mut result, "{}={}\0", key, value.display())?;
        }
        result.push_char('\0');
        result.push_char('\0');
        Ok(result.into_vec())
    }

    fn collect_nvidia_libraries(
        libraries: &'static [LibraryInfo],
        trace_dir: &PathBuf,
        cuda_install_lib_path: &Option<PathBuf>,
    ) -> Result<
        (Vec<Option<LibraryWithPath>>, Vec<(&'static str, PathBuf)>),
        Box<dyn Error + 'static>,
    > {
        fn check_single_library(
            trace_dir: &PathBuf,
            cuda_install_lib_path: &Option<PathBuf>,
            lib: &'static LibraryInfo,
        ) -> Result<Option<(LibraryWithPath, &'static str, PathBuf)>, Box<dyn Error>> {
            let Some(redirect) = lib.redirect.as_ref() else {
                return Ok(None);
            };
            let path = if lib.in_system32 {
                Some((
                    PathBuf::from_iter(["C:\\Windows", "System32", redirect.zluda_dll_name]),
                    true,
                ))
            } else {
                cuda_install_lib_path.clone().map(|mut path| {
                    path.push(redirect.zluda_dll_name);
                    (path, false)
                })
            };
            let dll_path = match path {
                Some((path, mandatory)) => {
                    if !path.exists() {
                        if mandatory {
                            return Err(format!(
                                "DLL {} not found at path {:?}",
                                redirect.zluda_dll_name, path
                            )
                            .into());
                        } else {
                            return Ok(None);
                        }
                    } else {
                        path
                    }
                }
                _ => return Ok(None),
            };
            let mut trace_path = trace_dir.clone();
            trace_path.push(redirect.zluda_dll_name);
            Ok(Some((
                LibraryWithPath {
                    library: lib,
                    path: trace_path,
                },
                redirect.trace_env_var,
                dll_path,
            )))
        }
        libraries.iter().try_fold(
            (Vec::new(), Vec::new()),
            |(mut dll_paths, mut env_vars), lib| {
                Ok::<_, Box<dyn Error>>(
                    match check_single_library(&trace_dir, &cuda_install_lib_path, lib)? {
                        Some((dll_path, env_var, trace_path)) => (
                            {
                                dll_paths.push(Some(dll_path));
                                dll_paths
                            },
                            {
                                env_vars.push((env_var, trace_path));
                                env_vars
                            },
                        ),
                        None => {
                            dll_paths.push(None);
                            (dll_paths, env_vars)
                        }
                    },
                )
            },
        )
    }
}
