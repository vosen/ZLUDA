use std::env::VarError;

fn main() -> Result<(), VarError> {
    println!("cargo:rustc-link-lib=dylib=amdhip64");
    //println!("cargo:rustc-link-search=native=/opt/rocm/lib/");
    println!("cargo:rustc-link-search=native=/home/ubuntu/hipamd/build/lib");
    println!("cargo:rustc-link-search=native=/home/vosen/hipamd/build/lib");
    Ok(())
}
