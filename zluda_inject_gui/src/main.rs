#![windows_subsystem = "windows"]

use bpaf::Parser;
use std::{ffi::OsString, os::windows::ffi::OsStringExt};
use windows::{
    core::{w, Interface, HRESULT, HSTRING, PCWSTR},
    Win32::{
        Foundation::ERROR_CANCELLED,
        System::Com::{
            CoCreateInstance, CoInitializeEx, CoTaskMemFree, CLSCTX_INPROC_SERVER,
            COINIT_APARTMENTTHREADED,
        },
        UI::Controls::{TaskDialog, TDCBF_OK_BUTTON, TD_ERROR_ICON},
        UI::Shell::{
            Common::COMDLG_FILTERSPEC, FileOpenDialog, IFileDialogCustomize, IFileOpenDialog,
            FOS_FILEMUSTEXIST, FOS_FORCEFILESYSTEM, FOS_PATHMUSTEXIST, SIGDN_FILESYSPATH,
        },
    },
};

fn main() {
    let _ = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };
    if let Err(error) = run() {
        show_error(&error.to_string());
    }
}

fn show_error(message: &str) {
    let message = HSTRING::from(message);
    unsafe {
        let _ = TaskDialog(
            None,
            None,
            w!("ZLUDA error"),
            w!("ZLUDA error"),
            &message,
            TDCBF_OK_BUTTON,
            TD_ERROR_ICON,
            None,
        );
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let parser = zluda_inject::args::arguments().to_options();
    if let Ok(config) = parser.run_inner(bpaf::Args::current_args()) {
        return zluda_inject::run(config, true);
    }
    let (executable, launch_with_tracing) = unsafe { select_executable() }?;
    let config = if launch_with_tracing {
        zluda_inject::args::ConfigSet::ZludaTrace
    } else {
        zluda_inject::args::ConfigSet::Zluda
    };
    zluda_inject::run(
        zluda_inject::args::Arguments {
            paths: config,
            exe: executable,
        },
        true,
    )
}

const TRACE_CHECKBOX_ID: u32 = 0x1;

unsafe fn select_executable() -> windows::core::Result<(OsString, bool)> {
    let dialog: IFileOpenDialog = CoCreateInstance(&FileOpenDialog, None, CLSCTX_INPROC_SERVER)?;
    let path_ext = std::env::var("PATHEXT").unwrap_or_default();
    let patterns = HSTRING::from(executable_patterns(&path_ext));
    dialog.SetTitle(w!("Launch with ZLUDA"))?;
    dialog.SetOptions(
        dialog.GetOptions()? | FOS_FORCEFILESYSTEM | FOS_PATHMUSTEXIST | FOS_FILEMUSTEXIST,
    )?;
    dialog.SetFileTypes(&[
        COMDLG_FILTERSPEC {
            pszName: w!("Executables"),
            pszSpec: PCWSTR::from_raw(patterns.as_ptr()),
        },
        COMDLG_FILTERSPEC {
            pszName: w!("Any file"),
            pszSpec: w!("*.*"),
        },
    ])?;
    dialog.SetDefaultExtension(w!("exe"))?;
    let customize: IFileDialogCustomize = dialog.cast()?;
    customize.AddCheckButton(TRACE_CHECKBOX_ID, w!("Save trace log"), false)?;
    match dialog.Show(None) {
        Ok(()) => {}
        Err(e) if e.code() == HRESULT::from_win32(ERROR_CANCELLED.0) => std::process::exit(1),
        Err(e) => return Err(e),
    }
    let trace = customize.GetCheckButtonState(TRACE_CHECKBOX_ID)?.as_bool();
    let item = dialog.GetResult()?;
    let path = item.GetDisplayName(SIGDN_FILESYSPATH)?;
    let result = OsString::from_wide(path.as_wide());
    CoTaskMemFree(Some(path.0 as *const core::ffi::c_void));
    Ok((result, trace))
}

// https://superuser.com/a/1027081
const DEFAULT_EXECUTABLE_PATTERNS: &str =
    "*.com;*.exe;*.bat;*.cmd;*.vbs;*.vbe;*.js;*.jse;*.wsf;*.wsh;*.msc";

fn executable_patterns(path_ext: &str) -> String {
    let patterns = path_ext
        .split(';')
        .filter_map(|extension| extract_extension(extension.trim()))
        .collect::<Vec<_>>();
    if patterns.is_empty() {
        return DEFAULT_EXECUTABLE_PATTERNS.to_string();
    }
    format!("*{}", patterns.join(";*")).to_ascii_lowercase()
}

fn extract_extension(path: &str) -> Option<&str> {
    let chars = path.as_bytes();
    if chars.len() < 2 {
        return None;
    }
    if chars[0] != b'.' {
        return None;
    }
    for c in chars.iter().skip(1) {
        if !c.is_ascii_alphanumeric() {
            return None;
        }
    }
    Some(path)
}

#[cfg(test)]
mod tests {}
