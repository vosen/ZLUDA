use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::ffi::OsStringExt;
use std::ptr;
use std::{error::Error, process};
use std::{ffi::OsString, mem};

use winapi::um::{
    libloaderapi::GetModuleFileNameW,
    processthreadsapi::{GetExitCodeProcess, ResumeThread},
    synchapi::WaitForSingleObject,
};
use winapi::{shared::minwindef, um::errhandlingapi::GetLastError};
use winapi::{
    shared::winerror::ERROR_INSUFFICIENT_BUFFER,
    um::winbase::{INFINITE, WAIT_FAILED},
};

use clap::{App, AppSettings, Arg};

static REDIRECT_DLL: &'static str = "zluda_redirect.dll";
static ZLUDA_DLL: &'static [u16] = wstr!("nvcuda.dll");

include!("../../zluda_redirect/src/payload_guid.rs");

pub fn main_impl() -> Result<(), Box<dyn Error>> {
    let matches = App::new("ZLUDA injector")
        .setting(AppSettings::TrailingVarArg)
        .arg(
            Arg::with_name("EXE")
                .help("Path to the executable to be injected with ZLUDA")
                .required(true),
        )
        .arg(
            Arg::with_name("ARGS")
                .multiple(true)
                .help("Arguments that will be passed to <EXE>"),
        )
        .get_matches();
    let exe = matches.value_of_os("EXE").unwrap();
    let args: Vec<&OsStr> = matches
        .values_of_os("ARGS")
        .map(|x| x.collect())
        .unwrap_or_else(|| Vec::new());
    let mut cmd_line = Vec::<u16>::with_capacity(exe.len() + 2);
    cmd_line.push('\"' as u16);
    copy_to(exe, &mut cmd_line);
    cmd_line.push('\"' as u16);
    cmd_line.push(' ' as u16);
    args.split_last().map(|(last_arg, args)| {
        for arg in args {
            cmd_line.reserve(arg.len());
            copy_to(arg, &mut cmd_line);
            cmd_line.push(' ' as u16);
        }
        copy_to(last_arg, &mut cmd_line);
    });

    cmd_line.push(0);
    let mut injector_path = get_injector_path()?;
    trim_to_parent(&mut injector_path);
    let redirect_path = create_redirect_path(&injector_path);
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
    let mut zluda_path = create_zluda_path(injector_path);
    os_call!(
        detours_sys::DetourCopyPayloadToProcess(
            proc_info.hProcess,
            &PAYLOAD_GUID,
            zluda_path.as_mut_ptr() as *mut _,
            (zluda_path.len() * mem::size_of::<u16>()) as u32
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

fn trim_to_parent(injector_path: &mut Vec<u16>) {
    let slash_idx = injector_path
        .iter()
        .enumerate()
        .rev()
        .find_map(|(idx, char)| {
            if *char == '/' as u16 || *char == '\\' as u16 {
                Some(idx)
            } else {
                None
            }
        });
    if let Some(idx) = slash_idx {
        injector_path.truncate(idx + 1);
    }
}

fn create_redirect_path(injector_dir: &[u16]) -> Vec<u8> {
    let os_string: OsString = OsString::from_wide(injector_dir);
    let mut utf8_string = os_string.to_string_lossy().as_bytes().to_vec();
    utf8_string.extend(REDIRECT_DLL.as_bytes());
    utf8_string.push(0);
    utf8_string
}

fn create_zluda_path(mut injector_dir: Vec<u16>) -> Vec<u16> {
    injector_dir.extend(ZLUDA_DLL);
    injector_dir
}

fn copy_to(from: &OsStr, to: &mut Vec<u16>) {
    for x in from.encode_wide() {
        to.push(x);
    }
}

fn get_injector_path() -> Result<Vec<u16>, Box<dyn Error>> {
    let mut result = vec![0u16; minwindef::MAX_PATH];
    let mut copied;
    loop {
        copied = os_call!(
            GetModuleFileNameW(ptr::null_mut(), result.as_mut_ptr(), result.len() as u32),
            |x| x != 0
        );
        if copied != result.len() as u32 {
            break;
        }
        os_call!(GetLastError(), |x| x != ERROR_INSUFFICIENT_BUFFER);
        result.resize(result.len() * 2, 0);
    }
    result.truncate(copied as usize);
    Ok(result)
}
