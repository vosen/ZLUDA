use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use std::ptr;
use std::{env, ops::Deref};
use std::{error::Error, process};
use std::{mem, path::PathBuf};

use mem::size_of_val;
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
static ZLUDA_DLL: &'static str = "nvcuda.dll";
static ZLUDA_ML_DLL: &'static str = "nvml.dll";

include!("../../zluda_redirect/src/payload_guid.rs");

pub fn main_impl() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() <= 1 {
        print_help_and_exit();
    }
    let injector_path = env::current_exe()?;
    let injector_dir = injector_path.parent().unwrap();
    let redirect_path = create_redirect_path(injector_dir);
    let (mut inject_nvcuda_path, mut inject_nvml_path, cmd) =
        create_inject_path(&args[1..], injector_dir)?;
    let mut cmd_line = construct_command_line(cmd);
    let mut startup_info = unsafe { mem::zeroed::<detours_sys::_STARTUPINFOW>() };
    let mut proc_info = unsafe { mem::zeroed::<detours_sys::_PROCESS_INFORMATION>() };
    os_call!(
        detours_sys::DetourCreateProcessWithDllExW(
            ptr::null(),
            cmd_line.as_mut_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
            0,
            0,
            ptr::null_mut(),
            ptr::null(),
            &mut startup_info as *mut _,
            &mut proc_info as *mut _,
            redirect_path.as_ptr() as *const i8,
            Option::None
        ),
        |x| x != 0
    );
    kill_child_on_process_exit(proc_info.hProcess)?;
    os_call!(
        detours_sys::DetourCopyPayloadToProcess(
            proc_info.hProcess,
            &PAYLOAD_NVCUDA_GUID,
            inject_nvcuda_path.as_mut_ptr() as *mut _,
            (inject_nvcuda_path.len() * mem::size_of::<u16>()) as u32
        ),
        |x| x != 0
    );
    os_call!(
        detours_sys::DetourCopyPayloadToProcess(
            proc_info.hProcess,
            &PAYLOAD_NVML_GUID,
            inject_nvml_path.as_mut_ptr() as *mut _,
            (inject_nvml_path.len() * mem::size_of::<u16>()) as u32
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

fn print_help_and_exit() -> ! {
    let current_exe = env::current_exe().unwrap();
    let exe_name = current_exe.file_name().unwrap().to_string_lossy();
    println!(
        "USAGE:
    {0} -- <EXE> [ARGS]...
    {0} <DLL> -- <EXE> [ARGS]...
ARGS:
    <DLL>        DLL to be injected instead of system nvcuda.dll, if not provided
                 will use nvcuda.dll from the directory where {0} is located
    <EXE>        Path to the executable to be injected with <DLL>
    <ARGS>...    Arguments that will be passed to <EXE>
",
        exe_name
    );
    process::exit(1)
}

// Adapted from https://docs.microsoft.com/en-us/archive/blogs/twistylittlepassagesallalike/everyone-quotes-command-line-arguments-the-wrong-way
fn construct_command_line(args: &[String]) -> Vec<u16> {
    let mut cmd_line = Vec::new();
    let args_len = args.len();
    for (idx, arg) in args.iter().enumerate() {
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

fn create_redirect_path(injector_dir: &Path) -> Vec<u8> {
    let mut injector_dir = injector_dir.to_path_buf();
    injector_dir.push(REDIRECT_DLL);
    let mut result = injector_dir.to_string_lossy().into_owned().into_bytes();
    result.push(0);
    result
}

fn create_inject_path<'a>(
    args: &'a [String],
    injector_dir: &Path,
) -> std::io::Result<(Vec<u16>, Vec<u16>, &'a [String])> {
    let injector_dir = injector_dir.to_path_buf();
    let (nvcuda_path, unparsed_args) = if args.get(0).map(Deref::deref) == Some("--") {
        (
            encode_file_in_directory_raw(injector_dir.clone(), ZLUDA_DLL),
            &args[1..],
        )
    } else if args.get(1).map(Deref::deref) == Some("--") {
        let dll_path = make_absolute_and_encode(&args[0])?;
        (dll_path, &args[2..])
    } else {
        print_help_and_exit()
    };
    let nvml_path = encode_file_in_directory_raw(injector_dir, ZLUDA_ML_DLL);
    Ok((nvcuda_path, nvml_path, unparsed_args))
}

fn encode_file_in_directory_raw(mut dir: PathBuf, file: &'static str) -> Vec<u16> {
    dir.push(file);
    let mut result = dir
        .to_string_lossy()
        .as_ref()
        .encode_utf16()
        .collect::<Vec<_>>();
    result.push(0);
    result
}

fn make_absolute_and_encode(maybe_path: &str) -> std::io::Result<Vec<u16>> {
    let path = Path::new(maybe_path);
    let mut encoded_path = if path.is_relative() {
        let mut current_dir = env::current_dir()?;
        current_dir.push(path);
        current_dir.as_os_str().encode_wide().collect::<Vec<_>>()
    } else {
        maybe_path.encode_utf16().collect::<Vec<_>>()
    };
    encoded_path.push(0);
    Ok(encoded_path)
}
