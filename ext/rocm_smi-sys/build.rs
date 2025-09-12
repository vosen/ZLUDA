use std::env::VarError;

fn main() -> Result<(), VarError> {
    println!("cargo:rustc-link-lib=dylib=rocm_smi64");
    println!("cargo:rustc-link-search=native=/opt/rocm/lib/");
    Ok(())
}
