#[macro_use]
#[cfg(target_os = "windows")]
mod win;
#[cfg(target_os = "windows")]
mod args;
#[cfg(target_os = "windows")]
mod bin;

#[cfg(target_os = "windows")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    bin::main_impl()
}

#[cfg(not(target_os = "windows"))]
fn main() {}
