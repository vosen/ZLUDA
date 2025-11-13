use argh::FromArgs;
use bpaf::{OptionParser, Parser};
use mem::size_of_val;
use std::env;
use std::os::windows;
use std::os::windows::ffi::OsStrExt;
use std::{error::Error, process};
use std::{fs, io, ptr};
use std::{mem, path::PathBuf};
use tempfile::TempDir;
use winapi::um::processenv::SearchPathW;
use winapi::um::winbase::{INFINITE, WAIT_FAILED};
use winapi::um::{
    jobapi2::{AssignProcessToJobObject, SetInformationJobObject},
    processthreadsapi::{GetExitCodeProcess, ResumeThread},
    synchapi::WaitForSingleObject,
    winbase::CreateJobObjectA,
    winnt::{
        JobObjectExtendedLimitInformation, HANDLE, JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
        JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
    },
};
use zluda_windows::LibraryInfo;

use crate::args::{self, Arguments, LibraryWithPath};

pub fn main_impl() -> Result<(), Box<dyn Error>> {
    let args = args::arguments().run();
    let injection_env = InjectionConfig::new(args.clone())?;
    dbg!(injection_env);
    todo!()
    /*
    let raw_args = argh::from_env::<ProgramArguments>();
    let normalized_args = NormalizedArguments::new(raw_args)?;
    let mut environment = Environment::setup(normalized_args)?;
    let mut startup_info = unsafe { mem::zeroed::<detours_sys::_STARTUPINFOW>() };
    let mut proc_info = unsafe { mem::zeroed::<detours_sys::_PROCESS_INFORMATION>() };
    let mut dlls_to_inject = [
        environment.nvml_path_zero_terminated.as_ptr() as *const i8,
        environment.nvcuda_path_zero_terminated.as_ptr() as _,
        environment.redirect_path_zero_terminated.as_ptr() as _,
    ];
    os_call!(
        detours_sys::DetourCreateProcessWithDllsW(
            ptr::null(),
            environment.winapi_command_line_zero_terminated.as_mut_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
            0,
            0,
            ptr::null_mut(),
            ptr::null(),
            &mut startup_info as *mut _,
            &mut proc_info as *mut _,
            dlls_to_inject.len() as u32,
            dlls_to_inject.as_mut_ptr(),
            Option::None
        ),
        |x| x != 0
    );
    kill_child_on_process_exit(proc_info.hProcess)?;
    os_call!(
        detours_sys::DetourCopyPayloadToProcess(
            proc_info.hProcess,
            &PAYLOAD_NVCUDA_GUID,
            environment.nvcuda_path_zero_terminated.as_ptr() as *mut _,
            environment.nvcuda_path_zero_terminated.len() as u32
        ),
        |x| x != 0
    );
    os_call!(
        detours_sys::DetourCopyPayloadToProcess(
            proc_info.hProcess,
            &PAYLOAD_NVML_GUID,
            environment.nvml_path_zero_terminated.as_ptr() as *mut _,
            environment.nvml_path_zero_terminated.len() as u32
        ),
        |x| x != 0
    );
    os_call!(ResumeThread(proc_info.hThread), |x| x as i32 != -1);
    os_call!(WaitForSingleObject(proc_info.hProcess, INFINITE), |x| x
        != WAIT_FAILED);
    let mut child_exit_code: u32 = 0;
    os_call!(
        GetExitCodeProcess(proc_info.hProcess, &mut child_exit_code as *mut _),
        |x| x != 0
    );
    process::exit(child_exit_code as i32)
     */
}

#[derive(Debug)]
struct InjectionConfig {
    dll_paths: Vec<LibraryWithPath>,
    env_vars: Vec<(&'static str, PathBuf)>,
}

impl InjectionConfig {
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
}

/*
struct NormalizedArguments {
    nvml_path: PathBuf,
    nvcuda_path: PathBuf,
    redirect_path: PathBuf,
    winapi_command_line_zero_terminated: Vec<u16>,
}

impl NormalizedArguments {
    fn new(prog_args: ProgramArguments) -> Result<Self, Box<dyn Error>> {
        let current_exe = env::current_exe()?;
        let nvml_path = Self::get_absolute_path(&current_exe, prog_args.nvml, NVML_DLL)?;
        let nvcuda_path = Self::get_absolute_path(&current_exe, prog_args.nvcuda, NVCUDA_DLL)?;
        let winapi_command_line_zero_terminated =
            construct_command_line(std::iter::once(prog_args.exe).chain(prog_args.args));
        let mut redirect_path = current_exe.parent().unwrap().to_path_buf();
        redirect_path.push(REDIRECT_DLL);
        Ok(Self {
            nvml_path,
            nvcuda_path,
            redirect_path,
            winapi_command_line_zero_terminated,
        })
    }

    const WIN_MAX_PATH: usize = 260;

    fn get_absolute_path(
        current_exe: &PathBuf,
        dll: Option<PathBuf>,
        default: &str,
    ) -> Result<PathBuf, Box<dyn Error>> {
        Ok(if let Some(dll) = dll {
            if dll.is_absolute() {
                dll
            } else {
                let mut full_dll_path = vec![0; Self::WIN_MAX_PATH];
                let mut dll_utf16 = dll.as_os_str().encode_wide().collect::<Vec<_>>();
                dll_utf16.push(0);
                loop {
                    let copied_len = os_call!(
                        SearchPathW(
                            ptr::null_mut(),
                            dll_utf16.as_ptr(),
                            ptr::null(),
                            full_dll_path.len() as u32,
                            full_dll_path.as_mut_ptr(),
                            ptr::null_mut()
                        ),
                        |x| x != 0
                    ) as usize;
                    if copied_len > full_dll_path.len() {
                        full_dll_path.resize(copied_len + 1, 0);
                    } else {
                        full_dll_path.truncate(copied_len);
                        break;
                    }
                }
                PathBuf::from(String::from_utf16_lossy(&full_dll_path))
            }
        } else {
            let mut dll_path = current_exe.parent().unwrap().to_path_buf();
            dll_path.push(default);
            dll_path
        })
    }
}

struct Environment {
    nvml_path_zero_terminated: String,
    nvcuda_path_zero_terminated: String,
    redirect_path_zero_terminated: String,
    winapi_command_line_zero_terminated: Vec<u16>,
    _temp_dir: TempDir,
}

// This structs represents "enviroment". By environment we mean all paths
// (nvcuda.dll, nvml.dll, etc.) and all related resources like the temporary
// directory which contains nvcuda.dll
impl Environment {
    fn setup(args: NormalizedArguments) -> io::Result<Self> {
        let _temp_dir = TempDir::new()?;
        let nvml_path_zero_terminated = Self::zero_terminate(Self::copy_to_correct_name(
            args.nvml_path,
            &_temp_dir,
            NVML_DLL,
        )?);
        let nvcuda_path_zero_terminated = Self::zero_terminate(Self::copy_to_correct_name(
            args.nvcuda_path,
            &_temp_dir,
            NVCUDA_DLL,
        )?);
        let redirect_path_zero_terminated = Self::zero_terminate(args.redirect_path);
        Ok(Self {
            nvml_path_zero_terminated,
            nvcuda_path_zero_terminated,
            redirect_path_zero_terminated,
            winapi_command_line_zero_terminated: args.winapi_command_line_zero_terminated,
            _temp_dir,
        })
    }

    fn copy_to_correct_name(
        path_buf: PathBuf,
        temp_dir: &TempDir,
        correct_name: &str,
    ) -> io::Result<PathBuf> {
        let file_name = path_buf.file_name().unwrap();
        if file_name == correct_name {
            Ok(path_buf)
        } else {
            let mut temp_file_path = temp_dir.path().to_path_buf();
            temp_file_path.push(correct_name);
            match windows::fs::symlink_file(&path_buf, &temp_file_path) {
                Ok(()) => {}
                Err(_) => {
                    fs::copy(&path_buf, &temp_file_path)?;
                }
            }
            Ok(temp_file_path)
        }
    }

    fn zero_terminate(p: PathBuf) -> String {
        let mut s = p.to_string_lossy().to_string();
        s.push('\0');
        s
    }
}

fn kill_child_on_process_exit(child: HANDLE) -> Result<(), Box<dyn Error>> {
    let job_handle = os_call!(CreateJobObjectA(ptr::null_mut(), ptr::null()), |x| x
        != ptr::null_mut());
    let mut info = unsafe { mem::zeroed::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() };
    info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
    os_call!(
        SetInformationJobObject(
            job_handle,
            JobObjectExtendedLimitInformation,
            &mut info as *mut _ as *mut _,
            size_of_val(&info) as u32
        ),
        |x| x != 0
    );
    os_call!(AssignProcessToJobObject(job_handle, child), |x| x != 0);
    Ok(())
}

// Adapted from https://docs.microsoft.com/en-us/archive/blogs/twistylittlepassagesallalike/everyone-quotes-command-line-arguments-the-wrong-way
fn construct_command_line(args: impl Iterator<Item = String>) -> Vec<u16> {
    let mut cmd_line = Vec::new();
    let args_len = args.size_hint().0;
    for (idx, arg) in args.enumerate() {
        if !arg.contains(&[' ', '\t', '\n', '\u{2B7F}', '\"'][..]) {
            cmd_line.extend(arg.encode_utf16());
        } else {
            cmd_line.push('"' as u16); // "
            let mut char_iter = arg.chars().peekable();
            loop {
                let mut current = char_iter.next();
                let mut backslashes = 0;
                match current {
                    Some('\\') => {
                        backslashes = 1;
                        while let Some('\\') = char_iter.peek() {
                            backslashes += 1;
                            char_iter.next();
                        }
                        current = char_iter.next();
                    }
                    _ => {}
                }
                match current {
                    None => {
                        for _ in 0..(backslashes * 2) {
                            cmd_line.push('\\' as u16);
                        }
                        break;
                    }
                    Some('"') => {
                        for _ in 0..(backslashes * 2 + 1) {
                            cmd_line.push('\\' as u16);
                        }
                        cmd_line.push('"' as u16);
                    }
                    Some(c) => {
                        for _ in 0..backslashes {
                            cmd_line.push('\\' as u16);
                        }
                        let mut temp = [0u16; 2];
                        cmd_line.extend(&*c.encode_utf16(&mut temp));
                    }
                }
            }
            cmd_line.push('"' as u16);
        }
        if idx < args_len - 1 {
            cmd_line.push(' ' as u16);
        }
    }
    cmd_line.push(0);
    cmd_line
}
 */
