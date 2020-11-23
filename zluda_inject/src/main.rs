#[cfg(target_os = "windows")]
mod bin;

#[cfg(not(target_os = "windows"))]
fn main() {}