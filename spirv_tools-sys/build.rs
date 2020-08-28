extern crate cmake;

use cmake::Config;
use std::{env::VarError, path::PathBuf};

fn main() -> Result<(), VarError> {
    let root_path = std::env::var("CARGO_MANIFEST_DIR")?;
    let mut headers_path = PathBuf::new();
    headers_path.push(root_path);
    headers_path.push("../ext/spirv-headers");
    let spirv_tools_dir = Config::new("../ext/spirv-tools")
        .always_configure(false)
        .define("SPIRV-Headers_SOURCE_DIR", headers_path)
        .define("SPIRV_SKIP_EXECUTABLES", "ON")
        .define("SPIRV_SKIP_TESTS", "ON")
        .build();
    println!(
        "cargo:rustc-link-search=native={}/bin",
        spirv_tools_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib",
        spirv_tools_dir.display()
    );
    // dynamic linking to avoid linking to C++ runtime
    println!("cargo:rustc-link-lib=dylib=SPIRV-Tools-shared");
    Ok(())
}
