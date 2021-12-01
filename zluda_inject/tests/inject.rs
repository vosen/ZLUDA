use std::{env, io, path::PathBuf, process::Command};

#[test]
fn direct_cuinit() -> io::Result<()> {
    let zluda_with_exe = PathBuf::from(env!("CARGO_BIN_EXE_zluda_with"));
    let mut zluda_dump_dll = zluda_with_exe.parent().unwrap().to_path_buf();
    zluda_dump_dll.push("zluda_dump.dll");
    let helpers_dir = env!("HELPERS_OUT_DIR");
    let exe_under_test = format!(
        "{}{}direct_cuinit.exe",
        helpers_dir,
        std::path::MAIN_SEPARATOR
    );
    let mut test_cmd = Command::new(&zluda_with_exe);
    let test_cmd = test_cmd.arg(&zluda_dump_dll).arg("--").arg(&exe_under_test);
    let test_output = test_cmd.output()?;
    assert!(test_output.status.success());
    let stderr_text = String::from_utf8(test_output.stderr).unwrap();
    assert!(stderr_text.contains("ZLUDA_DUMP"));
    Ok(())
}
