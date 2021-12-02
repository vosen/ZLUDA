use std::{
    env::{self, VarError},
    fs::{self, DirEntry},
    io,
    path::{self, PathBuf},
    process::Command,
};

fn main() -> Result<(), VarError> {
    println!("cargo:rerun-if-changed=build.rs");
    if env::var("PROFILE")? != "debug" {
        return Ok(());
    }
    let rustc_exe = env::var("RUSTC")?;
    let out_dir = env::var("OUT_DIR")?;
    let target = env::var("TARGET")?;
    let is_msvc = env::var("CARGO_CFG_TARGET_ENV")? == "msvc";
    let opt_level = env::var("OPT_LEVEL")?;
    let debug = str::parse::<bool>(env::var("DEBUG")?.as_str()).unwrap();
    let mut helpers_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    helpers_dir.push("tests");
    helpers_dir.push("helpers");
    let helpers_dir_as_string = helpers_dir.to_string_lossy();
    println!("cargo:rerun-if-changed={}", helpers_dir_as_string);
    for rust_file in fs::read_dir(&helpers_dir).unwrap().filter_map(rust_file) {
        let full_file_path = format!(
            "{}{}{}",
            helpers_dir_as_string,
            path::MAIN_SEPARATOR,
            rust_file
        );
        let mut rustc_cmd = Command::new(&*rustc_exe);
        if debug {
            rustc_cmd.arg("-g");
        }
        rustc_cmd.arg(format!("-Lnative={}", helpers_dir_as_string));
        if !is_msvc {
            // HACK ALERT
            // I have no idea why the extra library below have to be linked
            rustc_cmd.arg(r"-lucrt");
        }
        rustc_cmd
            .arg("-ldylib=nvcuda")
            .arg("-C")
            .arg(format!("opt-level={}", opt_level))
            .arg("--out-dir")
            .arg(format!("{}", out_dir))
            .arg("--target")
            .arg(format!("{}", target))
            .arg(full_file_path);
        assert!(rustc_cmd.status().unwrap().success());
    }
    println!("cargo:rustc-env=HELPERS_OUT_DIR={}", &out_dir);
    Ok(())
}

fn rust_file(entry: io::Result<DirEntry>) -> Option<String> {
    entry.ok().and_then(|e| {
        let os_file_name = e.file_name();
        let file_name = os_file_name.to_string_lossy();
        let is_file = e.file_type().ok().map(|t| t.is_file()).unwrap_or(false);
        if is_file && file_name.ends_with(".rs") {
            Some(file_name.to_string())
        } else {
            None
        }
    })
}
