use env::VarError;
use std::{env, path::PathBuf};

fn main() -> Result<(), VarError> {
    println!("cargo:rustc-link-lib=dylib=ze_loader");
    if env::var("CARGO_CFG_WINDOWS").is_ok() {
        let env = env::var("CARGO_CFG_TARGET_ENV")?;
        if env == "gnu" {
            println!("cargo:rustc-link-search=native=C:\\Windows\\System32");
        } else {
            let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
            path.push("src");
            println!("cargo:rustc-link-search=native={}", path.display());
        };
    }
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
