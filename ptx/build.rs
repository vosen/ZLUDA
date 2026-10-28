use std::fs::File;
use std::io::Read;

fn main() {
    let bc_path = "lib/zluda_ptx_impl.bc";
    let bc_constrained_path = "lib/zluda_ptx_impl_constrained.bc";
    println!("cargo:rerun-if-changed={}", bc_path);
    println!("cargo:rerun-if-changed={}", bc_constrained_path);
    println!("cargo:rerun-if-changed=lib/zluda_ptx_impl_agx.ll");
    println!("cargo:rerun-if-changed=lib/zluda_ptx_impl_air.ll");
    agx_impl_bitcode();
    air_impl_bitcode();
    // The AMDGPU helper bitcode is only linked when targeting AMDGPU.
    if std::env::var_os("CARGO_FEATURE_AMDGPU").is_some() {
        check_lfs_file(bc_path);
        check_lfs_file(bc_constrained_path);
    }
}

/// The AGX helpers as bitcode in OUT_DIR (zluda_ptx_impl_agx.bc), assembled by the llvm-as of the LLVM that
/// llvm_zluda links (ZLUDA_LLVM_BUILD_DIR): every AGX compilation links them, and parsing the 1.3 MB of text took
/// most of a small kernel's compile time. Without that LLVM the file holds the text, which the linker also reads.
fn agx_impl_bitcode() {
    println!("cargo:rerun-if-env-changed=ZLUDA_LLVM_BUILD_DIR");
    let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("zluda_ptx_impl_agx.bc");
    let assembled = std::env::var_os("ZLUDA_LLVM_BUILD_DIR").is_some_and(|dir| {
        let llvm_as = std::path::Path::new(&dir).join("bin").join("llvm-as");
        std::process::Command::new(llvm_as)
            .args(["lib/zluda_ptx_impl_agx.ll", "-o"])
            .arg(&out)
            .status()
            .is_ok_and(|s| s.success())
    });
    if !assembled {
        std::fs::copy("lib/zluda_ptx_impl_agx.ll", &out).unwrap();
    }
}

/// The helpers of the Air target (zluda_ptx_impl_air.bc in OUT_DIR): the AIR helpers of
/// zluda_ptx_impl_agx.ll with each AGX intrinsic (llvm.agx.*) renamed to the primitive (__zluda_air.*) that
/// zluda_ptx_impl_air.ll defines in AIR, linked with those definitions. It needs the LLVM that llvm_zluda links
/// (ZLUDA_LLVM_BUILD_DIR: llvm-as and llvm-link); without it the file is empty and the Air target reports that.
fn air_impl_bitcode() {
    let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let out = out_dir.join("zluda_ptx_impl_air.bc");
    let built = std::env::var_os("ZLUDA_LLVM_BUILD_DIR").is_some_and(|dir| {
        let bin = std::path::Path::new(&dir).join("bin");
        // (immarg is for intrinsics only: the primitives are ordinary functions)
        let shared = std::fs::read_to_string("lib/zluda_ptx_impl_agx.ll")
            .unwrap()
            .lines()
            .map(|l| {
                let l = l.replace("@llvm.agx.", "@__zluda_air.");
                if l.starts_with("declare") && l.contains("@__zluda_air.") { l.replace(" immarg", "") } else { l }
            })
            .collect::<Vec<_>>()
            .join("\n");
        let shared_ll = out_dir.join("zluda_ptx_impl_air_shared.ll");
        std::fs::write(&shared_ll, shared).unwrap();
        let shared_bc = out_dir.join("zluda_ptx_impl_air_shared.bc");
        let prims_bc = out_dir.join("zluda_ptx_impl_air_prims.bc");
        let run = |cmd: &str, args: &[&std::ffi::OsStr]| {
            std::process::Command::new(bin.join(cmd)).args(args).status().is_ok_and(|s| s.success())
        };
        run("llvm-as", &[shared_ll.as_os_str(), "-o".as_ref(), shared_bc.as_os_str()])
            && run("llvm-as", &["lib/zluda_ptx_impl_air.ll".as_ref(), "-o".as_ref(), prims_bc.as_os_str()])
            && run("llvm-link", &[shared_bc.as_os_str(), prims_bc.as_os_str(), "-o".as_ref(), out.as_os_str()])
    });
    if !built {
        std::fs::write(&out, b"").unwrap();
    }
}

fn check_lfs_file(bc_path: &str) {
    let mut magic = [0u8; 2];
    File::open(bc_path)
        .unwrap_or_else(|e| panic!("Failed to open {}: {}", bc_path, e))
        .read_exact(&mut magic)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", bc_path, e));
    assert!(
        &magic == b"BC",
        "{} is a git lfs stub and not the actual file. Run `git lfs pull` to fetch it",
        bc_path
    );
}
