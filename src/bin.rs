extern crate clap;
extern crate detours_sys;

use std::error::Error;
use std::ffi::OsStr;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use std::ptr;

use clap::{App, AppSettings, Arg};

mod win_err;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("notCUDA injector")
        .setting(AppSettings::TrailingVarArg)
        .arg(
            Arg::with_name("EXE")
                .help("Path to the executable to be injected with notCUDA")
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
    let mut startup_info = unsafe { mem::zeroed::<detours_sys::_STARTUPINFOW>() };
    let mut proc_info = unsafe { mem::zeroed::<detours_sys::_PROCESS_INFORMATION>() };
    let process_success = unsafe {
        detours_sys::DetourCreateProcessWithDllExW(
            ptr::null(),
            cmd_line.as_mut_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
            0,
            0x10,
            ptr::null_mut(),
            ptr::null(),
            &mut startup_info as *mut _,
            &mut proc_info as *mut _,
            "nvcuda_redirect.dll".as_ptr() as *const i8,
            Option::None,
        )
    };
    if process_success == 0 {
        return Err(win_err::error_string(win_err::errno()))?;
    }
    Ok(())
}

fn copy_to(from: &OsStr, to: &mut Vec<u16>) {
    for x in from.encode_wide() {
        to.push(x);
    }
}
