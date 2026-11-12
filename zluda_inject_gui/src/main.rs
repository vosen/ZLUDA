#![windows_subsystem = "windows"]

use bpaf::Parser;
use std::{ffi::OsString, mem, os::windows::ffi::OsStringExt};
use windows::{
    core::{w, PCWSTR, PWSTR},
    Win32::UI::Controls::Dialogs::*,
};

fn main() {
    run().unwrap();
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let parser = zluda_inject::args::arguments().to_options();
    if let Ok(config) = parser.run_inner(bpaf::Args::current_args()) {
        return zluda_inject::run(config, true);
    }
    let executable = select_executable();
    zluda_inject::run(
        zluda_inject::args::Arguments {
            paths: zluda_inject::args::ConfigSet::Zluda,
            exe: executable,
        },
        true,
    )
}

fn select_executable() -> OsString {
    let mut path = vec![0u16; 32768];
    let path_ext = std::env::var("PATHEXT").unwrap_or_default();
    let filter = executable_filter(&path_ext);
    let mut dialog = OPENFILENAMEW {
        lStructSize: mem::size_of::<OPENFILENAMEW>() as u32,
        lpstrFilter: PCWSTR(filter.as_ptr()),
        lpstrFile: PWSTR(path.as_mut_ptr()),
        nMaxFile: path.len() as u32,
        nFilterIndex: 1,
        lpstrTitle: w!("Select an application to launch with ZLUDA"),
        Flags: OFN_EXPLORER | OFN_FILEMUSTEXIST | OFN_PATHMUSTEXIST,
        lpstrDefExt: w!("exe"),
        ..Default::default()
    };
    if !unsafe { GetOpenFileNameW(&mut dialog) }.as_bool() {
        std::process::exit(1);
    }
    OsString::from_wide(&path[..path.iter().position(|&c| c == 0).unwrap_or(path.len())])
}

fn executable_filter(path_ext: &str) -> Vec<u16> {
    let patterns = executable_patterns(path_ext);
    format!("Executables ({patterns})\0{patterns}\0Any file (*.*)\0*.*\0\0")
        .encode_utf16()
        .collect()
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
