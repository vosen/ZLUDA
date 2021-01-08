use env::VarError;
use std::{env, path::PathBuf};

// HACK ALERT
// This is a temporary hack to to make sure that linker does not pick up
// NVIDIA OpenCL .lib using paths injected by cl-sys

fn main() -> Result<(), VarError> {
    if cfg!(windows) {
        let env = env::var("CARGO_CFG_TARGET_ENV")?;
        if env == "msvc" {
            let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
            path.push("lib");
            println!("cargo:rustc-link-search=native={}", path.display());
        } else {
            println!("cargo:rustc-link-search=native=C:\\Windows\\System32");
        };
    }
    Ok(())
}
