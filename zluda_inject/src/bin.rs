use crate::args;
use crate::args::{Arguments, LibraryWithPath};
use bpaf::Parser;
use rustc_hash::FxHashSet;
use std::error::Error;
use std::ffi::{CString, OsStr};
use std::path::PathBuf;
use std::{env, mem, process, ptr};
use windows::Win32::Foundation::{HANDLE, WAIT_FAILED};
use windows::Win32::System::JobObjects::{
    AssignProcessToJobObject, JobObjectExtendedLimitInformation, SetInformationJobObject,
    JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
};
use windows::Win32::System::Threading::{
    self, GetExitCodeProcess, ResumeThread, WaitForSingleObject,
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

pub fn main_impl() -> Result<(), Box<dyn Error>> {
    let args = args::arguments().run();
    let injection_env = InjectionConfig::new(args.clone())?;
    let mut command_line = args.command_line_zero_terminated();
    let mut startup_info = unsafe { mem::zeroed::<detours_sys::_STARTUPINFOW>() };
    let mut proc_info = unsafe { mem::zeroed::<detours_sys::_PROCESS_INFORMATION>() };
    let mut env_vars_block = injection_env.get_env_variables_block()?;
    let dlls_to_inject = injection_env.get_injection_paths_zero_terminated()?;
    let mut dll_to_inject_pointers = dlls_to_inject
        .iter()
        .map(|p| p.as_ptr())
        .collect::<Vec<_>>();
    os_call! {
        detours_sys::DetourCreateProcessWithDllsW(
            ptr::null(),
            command_line.as_mut_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
            0,
            0,
            env_vars_block.as_mut_ptr().cast(),
            ptr::null(),
            std::ptr::from_mut(&mut startup_info),
            std::ptr::from_mut(&mut proc_info),
            dll_to_inject_pointers.len() as u32,
            dll_to_inject_pointers.as_mut_ptr(),
            Option::None
        )
    };
    kill_child_on_process_exit(proc_info.hProcess)?;
    injection_env.copy_payloads_to_process(proc_info.hProcess)?;
    os_call! { ResumeThread(HANDLE(proc_info.hThread)), |x| x == -1i32 as u32 };
    os_call! { WaitForSingleObject(HANDLE(proc_info.hProcess), Threading::INFINITE), |x| x == WAIT_FAILED };
    let mut child_exit_code: u32 = 0;
    unsafe { GetExitCodeProcess(HANDLE(proc_info.hProcess), &mut child_exit_code as *mut _) }?;
    process::exit(child_exit_code as i32)
}

#[derive(Debug)]
struct InjectionConfig {
    redirect_dll_path: PathBuf,
    dll_paths: Vec<LibraryWithPath>,
    env_vars: Vec<(&'static str, PathBuf)>,
}

impl InjectionConfig {
    fn get_injection_paths_zero_terminated(&self) -> Result<Vec<CString>, Box<dyn Error>> {
        let mut injection_paths = self
            .dll_paths
            .iter()
            .map(|lib| {
                let path_utf16 = lib.path.as_os_str().to_str().ok_or_else(|| {
                    let err: Box<dyn Error> = format!(
                        "Failed to convert path {:?} to UTF-8 string",
                        lib.path.as_os_str()
                    )
                    .into();
                    err
                })?;
                Ok::<_, Box<dyn Error>>(unsafe {
                    CString::from_vec_unchecked(path_utf16.as_bytes().to_vec())
                })
            })
            .collect::<Result<Vec<CString>, _>>()?;
        injection_paths.push(CString::new(self.redirect_dll_path.to_str().ok_or_else(
            || {
                let err: Box<dyn Error> = format!(
                    "Failed to convert path {:?} to UTF-8 string",
                    self.redirect_dll_path.as_os_str()
                )
                .into();
                err
            },
        )?)?);
        Ok(injection_paths)
    }

    fn new(args: Arguments) -> Result<Self, Box<dyn Error>> {
        match args.paths {
            args::ConfigSet::Custom(libs) => Self::custom(libs),
            args::ConfigSet::Zluda => Self::zluda(),
            args::ConfigSet::ZludaTrace => Self::zluda_trace(),
            args::ConfigSet::NvidiaTrace => Self::nvidia_trace(),
        }
    }

    fn custom(libs: Vec<LibraryWithPath>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            redirect_dll_path: Self::redirect_dll_path(Self::current_exe_dir()?),
            dll_paths: libs,
            env_vars: Vec::new(),
        })
    }

    fn zluda() -> Result<Self, Box<dyn Error>> {
        let current_exe_dir = Self::current_exe_dir()?;
        let dll_paths = zluda_windows::LIBRARIES
            .iter()
            .map(|lib| {
                let mut path = current_exe_dir.clone();
                path.push(lib.ascii_name);
                LibraryWithPath { library: lib, path }
            })
            .collect();
        Ok(Self {
            redirect_dll_path: Self::redirect_dll_path(current_exe_dir),
            dll_paths,
            env_vars: Vec::new(),
        })
    }

    fn zluda_trace() -> Result<Self, Box<dyn Error>> {
        let current_exe_dir = Self::current_exe_dir()?;
        let trace_dir = Self::trace_dir()?;
        let dll_paths = zluda_windows::LIBRARIES
            .iter()
            .map(|lib| {
                let mut path = trace_dir.clone();
                path.push(lib.ascii_name);
                LibraryWithPath { library: lib, path }
            })
            .collect::<Vec<_>>();
        let env_vars = zluda_windows::LIBRARIES
            .iter()
            .filter_map(|lib| {
                if lib.is_alias {
                    None
                } else {
                    let mut path = current_exe_dir.clone();
                    path.push(lib.ascii_name);
                    Some((lib.trace_env_var, path))
                }
            })
            .collect::<Vec<_>>();
        Ok(Self {
            redirect_dll_path: Self::redirect_dll_path(current_exe_dir),
            dll_paths,
            env_vars,
        })
    }

    fn nvidia_trace() -> Result<Self, Box<dyn Error>> {
        fn check_single_library(
            trace_dir: &PathBuf,
            cuda_install_lib_path: &Option<PathBuf>,
            lib: &'static LibraryInfo,
        ) -> Result<Option<(LibraryWithPath, &'static str, PathBuf)>, Box<dyn Error>> {
            let path = if lib.in_system32 {
                Some((
                    PathBuf::from_iter(["C:\\Windows", "System32", lib.ascii_name]),
                    true,
                ))
            } else {
                cuda_install_lib_path.clone().map(|mut path| {
                    path.push(lib.ascii_name);
                    (path, false)
                })
            };
            let dll_path = match path {
                Some((path, mandatory)) => {
                    if !path.exists() {
                        if mandatory {
                            return Err(format!(
                                "DLL {} not found at path {:?}",
                                lib.ascii_name, path
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
            trace_path.push(lib.ascii_name);
            Ok(Some((
                LibraryWithPath {
                    library: lib,
                    path: trace_path,
                },
                lib.trace_env_var,
                dll_path,
            )))
        }
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
        let (dll_paths, env_vars) = zluda_windows::LIBRARIES.iter().try_fold(
            (Vec::new(), Vec::new()),
            |(mut dll_paths, mut env_vars), lib| {
                Ok::<_, Box<dyn Error>>(
                    match check_single_library(&trace_dir, &cuda_install_lib_path, lib)? {
                        Some((dll_path, env_var, trace_path)) => (
                            {
                                dll_paths.push(dll_path);
                                dll_paths
                            },
                            {
                                env_vars.push((env_var, trace_path));
                                env_vars
                            },
                        ),
                        None => (dll_paths, env_vars),
                    },
                )
            },
        )?;
        Ok(Self {
            redirect_dll_path: Self::redirect_dll_path(Self::current_exe_dir()?),
            dll_paths,
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

    fn redirect_dll_path(mut current_exe_dir: PathBuf) -> PathBuf {
        current_exe_dir.push("zluda_redirect.dll");
        current_exe_dir
    }

    fn copy_payloads_to_process(
        self,
        h_process: *mut std::ffi::c_void,
    ) -> Result<(), Box<dyn Error>> {
        for LibraryWithPath { library, path } in self.dll_paths {
            let path = path.to_str().ok_or_else(|| {
                let err: Box<dyn Error> = format!(
                    "Failed to convert path {:?} to UTF-8 string",
                    path.as_os_str()
                )
                .into();
                err
            })?;
            let mut path = path.to_string();
            os_call! {
                detours_sys::DetourCopyPayloadToProcess(
                    h_process,
                    std::ptr::from_ref(&library.guid).cast(),
                    path.as_mut_ptr().cast(),
                    path.len() as u32
                )
            };
        }
        Ok(())
    }

    fn get_env_variables_block(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        use std::io::Write;
        let mut result = Vec::new();
        let mut known_vars = FxHashSet::default();
        for (var, value) in env::vars_os() {
            write!(&mut result, "{}={}\0", var.display(), value.display())?;
            known_vars.insert(value);
        }
        for (key, value) in self.env_vars.iter() {
            let x: &OsStr = key.as_ref();
            if known_vars.contains(x) {
                continue;
            }
            write!(&mut result, "{}={}\0", key, value.display())?;
        }
        result.push(0);
        result.push(0);
        Ok(result)
    }
}

fn kill_child_on_process_exit(child: detours_sys::HANDLE) -> Result<(), Box<dyn Error>> {
    let job_handle = unsafe { windows::Win32::System::JobObjects::CreateJobObjectA(None, None) }?;
    let mut info = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
    info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
    unsafe {
        SetInformationJobObject(
            job_handle,
            JobObjectExtendedLimitInformation,
            &mut info as *mut _ as *mut _,
            size_of_val(&info) as u32,
        )
    }?;
    unsafe { AssignProcessToJobObject(job_handle, HANDLE(child)) }?;
    Ok(())
}
