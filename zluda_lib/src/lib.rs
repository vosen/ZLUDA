pub extern crate zluda;

pub use zluda::cuda::*;

// For some reason, on Linux linker strips out all our re-exports,
// there's probably a cleaner solution, but for now just exporting
// the function below stops it from doing so
#[no_mangle]
fn _zluda_very_bad_linker_hack() {
    let _ = unsafe { cuInit(0) };
}
