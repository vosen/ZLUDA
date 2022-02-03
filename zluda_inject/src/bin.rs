use std::env;
use std::os::windows;
use std::os::windows::ffi::OsStrExt;
use std::{error::Error, process};
use std::{fs, io, ptr};
use std::{mem, path::PathBuf};

use argh::FromArgs;
use mem::size_of_val;
use tempfile::TempDir;
use winapi::um::processenv::SearchPathW;
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

use winapi::um::winbase::{INFINITE, WAIT_FAILED};

static REDIRECT_DLL: &'static str = "zluda_redirect.dll";
static NVCUDA_DLL: &'static str = "nvcuda.dll";
static NVML_DLL: &'static str = "nvml.dll";

include!("../../zluda_redirect/src/payload_guid.rs");

#[derive(FromArgs)]
/// Launch application with custom CUDA libraries
struct ProgramArguments {
    /// DLL to be injected instead of system nvcuda.dll. If not provided {0} will use nvcuda.dll from its directory
    #[argh(option)]
    nvcuda: Option<PathBuf>,

    /// DLL to be injected instead of system nvml.dll. If not provided {0} will use nvml.dll from its directory
    #[argh(option)]
    nvml: Option<PathBuf>,

    /// executable to be injected with custom CUDA libraries
    #[argh(positional)]
    exe: String,

    /// arguments to the executable
    #[argh(positional)]
    args: Vec<String>,
}

pub fn main_impl() -> Result<(), Box<dyn Error>> {
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
}

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
