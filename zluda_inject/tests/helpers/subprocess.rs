#![crate_type = "bin"]

use std::io;
use std::process::Command;

fn main() -> io::Result<()> {
    let status = Command::new("direct_cuinit.exe").status()?;
    assert!(status.success());
    Ok(())
}
