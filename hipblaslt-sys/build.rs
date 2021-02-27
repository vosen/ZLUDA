use std::env::VarError;

fn main() -> Result<(), VarError> {
    println!("cargo:rustc-link-lib=dylib=hipblaslt");
    println!("cargo:rustc-link-search=native=/opt/rocm/lib/");
    Ok(())
}
