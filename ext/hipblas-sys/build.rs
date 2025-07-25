use std::env::VarError;
use std::{env, path::PathBuf};

fn main() -> Result<(), VarError> {
    if cfg!(windows) {
        println!("cargo:rustc-link-lib=dylib=hipblas_2");
        let env = env::var("CARGO_CFG_TARGET_ENV")?;
        if env == "msvc" {
            let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
            path.push("lib");
            println!("cargo:rustc-link-search=native={}", path.display());
        } else {
            println!("cargo:rustc-link-search=native=C:\\Windows\\System32");
        };
    } else {
        println!("cargo:rustc-link-lib=dylib:+verbatim=libhipblas.so.2");
        println!("cargo:rustc-link-search=native=/opt/rocm/lib/");
    }
    Ok(())
}
