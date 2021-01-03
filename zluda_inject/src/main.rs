#[macro_use]
#[cfg(target_os = "windows")]
mod win;
#[cfg(target_os = "windows")]
mod bin;

use std::error::Error;

#[cfg(target_os = "windows")]
fn main() -> Result<(), Box<dyn Error>> {
    bin::main_impl()
}

#[cfg(not(target_os = "windows"))]
fn main() {}
