use std::{env, io, process::Command};

#[test]
fn direct_cuinit() -> io::Result<()> {
    let helpers_dir = env!("HELPERS_OUT_DIR");
    let mut main_exe = Command::new(format!(
        "{}{}direct_cuinit.exe",
        helpers_dir,
        std::path::MAIN_SEPARATOR
    ));
    assert!(main_exe.status()?.success());
    Ok(())
}
