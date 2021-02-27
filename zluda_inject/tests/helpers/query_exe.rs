#![crate_type = "bin"]

use std::io;
use std::process::Command;

fn main() -> io::Result<()> {
    let status = Command::new("query.exe").arg("session").status()?;
    // App returns 1 on my machine
    assert_eq!(status.code(), Some(1));
    Ok(())
}
