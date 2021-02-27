mod common;
#[cfg_attr(unix, path = "unix.rs")]
#[cfg_attr(windows, path = "windows.rs")]
pub mod r#impl;
#[allow(warnings)]
mod nvml;
