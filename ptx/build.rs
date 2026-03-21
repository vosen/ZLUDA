use std::fs::File;
use std::io::Read;

fn main() {
    let bc_path = "lib/zluda_ptx_impl.bc";
    println!("cargo:rerun-if-changed={}", bc_path);
    check_lfs_file(bc_path);
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
