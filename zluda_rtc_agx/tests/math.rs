//! Builds tests/math_host.cpp (agx_nvrtc_math.h on the host) with clang++ and checks its error bounds against libm.

use std::path::Path;
use std::process::Command;

#[test]
fn libdevice_math_against_libm() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    let exe = Path::new(env!("CARGO_TARGET_TMPDIR")).join("math_host");
    let cxx = std::env::var("CXX").unwrap_or_else(|_| "clang++".into());
    let built = Command::new(&cxx)
        .args(["-O2", "-std=c++20", "-ffp-contract=off"])
        .arg(dir.join("math_host.cpp"))
        .arg("-o")
        .arg(&exe)
        .status();
    match built {
        Ok(s) if s.success() => {}
        _ => {
            eprintln!("{cxx} could not build math_host.cpp: skipped");
            return;
        }
    }
    let out = Command::new(&exe).output().unwrap();
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success(), "{text}");
}
