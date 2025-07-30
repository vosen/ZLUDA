#![cfg(windows)]
use std::{env, io, path::PathBuf, process::Command};

#[test]
fn direct_cuinit() -> io::Result<()> {
    run_process_and_check_for_zluda_dump("direct_cuinit")
}

#[test]
fn do_cuinit_early() -> io::Result<()> {
    run_process_and_check_for_zluda_dump("do_cuinit_early")
}

#[test]
fn do_cuinit_late() -> io::Result<()> {
    run_process_and_check_for_zluda_dump("do_cuinit_late")
}

#[test]
fn do_cuinit_late_clr() -> io::Result<()> {
    run_process_and_check_for_zluda_dump("do_cuinit_late_clr")
}

#[test]
fn indirect_cuinit() -> io::Result<()> {
    run_process_and_check_for_zluda_dump("indirect_cuinit")
}

#[test]
fn subprocess() -> io::Result<()> {
    run_process_and_check_for_zluda_dump("subprocess")
}

fn run_process_and_check_for_zluda_dump(name: &'static str) -> io::Result<()> {
    let zluda_with_exe = PathBuf::from(env!("CARGO_BIN_EXE_zluda_with"));
    let mut zluda_dump_dll = zluda_with_exe.parent().unwrap().to_path_buf();
    zluda_dump_dll.push("zluda_dump.dll");
    let helpers_dir = env!("HELPERS_OUT_DIR");
    let exe_under_test = format!("{}{}{}.exe", helpers_dir, std::path::MAIN_SEPARATOR, name);
    let mut test_cmd = Command::new(&zluda_with_exe);
    let test_cmd = test_cmd
        .arg("--nvcuda")
        .arg(&zluda_dump_dll)
        .arg("--")
        .arg(&exe_under_test);
    let test_output = test_cmd.output()?;
    assert!(test_output.status.success());
    let stderr_text = String::from_utf8(test_output.stderr).unwrap();
    assert!(stderr_text.contains("ZLUDA_DUMP"));
    Ok(())
}
