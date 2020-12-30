//! Build detours.
fn build_detours() {
    cc::Build::new()
        .host("x86_64-pc-windows-msvc")
        .target("x86_64-pc-windows-msvc")
        .include("../ext/detours/src")
        .define("WIN32_LEAN_AND_MEAN", "1")
        .file("../ext/detours/src/creatwth.cpp")
        .file("../ext/detours/src/detours.cpp")
        .file("../ext/detours/src/disasm.cpp")
        .file("../ext/detours/src/image.cpp")
        .file("../ext/detours/src/modules.cpp")
        .compile("detours");
}

#[cfg(feature = "buildtime_bindgen")]
fn generate_bindings() {
    use std::{env, fs, path::PathBuf};
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::copy("deps/detours-src/src/detours.h", out_path.join("detours.h")).unwrap();
    //
    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}", out_path.to_str().expect("OUTDIR is weird")))
        .clang_arg("-fms-compatibility")
        .clang_arg("-fms-extensions")
        // Detouring APIs
        .whitelist_function("DetourTransactionBegin")
        .whitelist_function("DetourUpdateThread")
        .whitelist_function("DetourAttach")
        .whitelist_function("DetourAttachEx")
        .whitelist_function("DetourDetach")
        .whitelist_function("DetourSetIgnoreTooSmall")
        .whitelist_function("DetourSetRetainRegions")
        .whitelist_function("DetourSetSystemRegionLowerBound")
        .whitelist_function("DetourSetSystemRegionUpperBound")
        .whitelist_function("DetourTransactionAbort")
        .whitelist_function("DetourTransactionCommit")
        .whitelist_function("DetourTransactionCommitEx")
        // Targeting APIs
        .whitelist_function("DetourFindFunction")
        .whitelist_function("DetourCodeFromPointer")
        // Binary and Payload access APIs
        .whitelist_function("DetourEnumerateModules")
        .whitelist_function("DetourGetEntryPoint")
        .whitelist_function("DetourGetModuleSize")
        .whitelist_function("DetourEnumerateExports")
        .whitelist_function("DetourEnumerateImports")
        .whitelist_function("DetourEnumerateImportsEx")
        .whitelist_function("DetourFindPayload")
        .whitelist_function("DetourGetContainingModule")
        .whitelist_function("DetourGetSizeOfPayloads")
        // Binary Modifcation APIs
        .whitelist_function("DetourBinaryOpen")
        .whitelist_function("DetourBinaryEnumeratePayloads")
        .whitelist_function("DetourBinaryFindPayload")
        .whitelist_function("DetourBinarySetPayload")
        .whitelist_function("DetourBinaryDeletePayload")
        .whitelist_function("DetourBinaryPurgePayloads")
        .whitelist_function("DetourBinaryEditImports")
        .whitelist_function("DetourBinaryResetImports")
        .whitelist_function("DetourBinaryWrite")
        .whitelist_function("DetourBinaryClose")
        // Injection APIs
        .whitelist_function("DetourCreateProcessWithDllW")
        .whitelist_function("DetourCreateProcessWithDllExW")
        .whitelist_function("DetourCreateProcessWithDllsW")
        .whitelist_function("DetourCreateProcessWithDllA")
        .whitelist_function("DetourCreateProcessWithDllExA")
        .whitelist_function("DetourCreateProcessWithDllsA")
        .whitelist_function("DetourCopyPayloadToProcess")
        .whitelist_function("DetourFinishHelperProcess")
        .whitelist_function("DetourIsHelperProcess")
        .whitelist_function("DetourRestoreAfterWith")
        //
        .header("build/wrapper.h")
        .generate()
        .expect("Unable to generate bindings");
    //
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[cfg(not(feature = "buildtime_bindgen"))]
fn generate_bindings() {}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    build_detours();
    generate_bindings();
}
