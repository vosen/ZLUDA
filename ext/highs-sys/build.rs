use std::env;

/*
use std::path::Path;

fn generate_bindings<'a>(include_paths: impl Iterator<Item = &'a Path>) {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = include_paths.fold(bindgen::Builder::default(), |builder, path| {
        builder.clang_arg(format!("-I{}", path.to_string_lossy()))
    });
    let c_bindings = builder
        // The input header we would like to generate bindings for.
        // This is a trivial wrapper header so that the HiGHS headers
        // can be discovered from the include path.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_function("^Highs.*")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    c_bindings
        .write_to_file("src/c_bindings.rs")
        .expect("Couldn't write bindings!");
}
*/

#[cfg(feature = "build")]
fn build() -> bool {
    use cmake::Config;
    let mut dst = Config::new("../HiGHS");
    try_use_ninja(&mut dst);

    // Avoid using downstream project's profile setting for HiGHS build.
    if cfg!(feature = "highs_release") {
        dst.profile("Release");
    }

    let dst = dst
        .define("FAST_BUILD", "ON")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_MSVC_RUNTIME_LIBRARY", "MultiThreaded")
        .define("ZLIB", if cfg!(feature = "libz") { "ON" } else { "OFF" })
        .build();

    //let include_path = dst.join("include").join("highs");
    //generate_bindings(Some(include_path.as_path()).into_iter());

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-search=native={}/lib64", dst.display());
    println!("cargo:rustc-link-lib=static=highs");

    if cfg!(feature = "libz") {
        println!("cargo:rustc-link-lib=z");
    }

    let target = env::var("TARGET").unwrap();
    let apple = target.contains("apple");
    let linux = target.contains("linux");
    let mingw = target.contains("pc-windows-gnu");
    if apple {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if linux || mingw {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
    println!("cargo:rerun-if-changed=../HiGHS/src/interfaces/highs_c_api.h");

    true
}

fn try_use_ninja(cmake: &mut cmake::Config) {
    let mut cmd = std::process::Command::new("ninja");
    cmd.arg("--version");
    if let Ok(status) = cmd.status() {
        if status.success() {
            cmake.generator("Ninja");
        }
    }
}

#[cfg(not(feature = "build"))]
fn build() -> bool {
    false
}

#[cfg(feature = "discover")]
fn discover() -> bool {
    let lib = match pkg_config::Config::new()
        .atleast_version("1.5.0")
        .probe("highs")
    {
        Ok(lib) => lib,
        Err(_e) => return false,
    };

    generate_bindings(lib.include_paths.iter().map(|p| p.as_path()));

    true
}

#[cfg(not(feature = "discover"))]
fn discover() -> bool {
    false
}

fn main() {
    if cfg!(all(
        any(
            feature = "highs_release",
            feature = "libz",
            feature = "ninja"
        ),
        not(feature = "build")
    )) {
        panic!("You have enabled features that control how HiGHS is built, but have not enabled the 'build' feature.\n\
               Thus, your features will never have any effect. Please enable the 'build' feature on highs-sys if you want to build HiGHS or disable the 'libz', 'ninja' and 'highs_release' features.");
    }

    if !discover() && !build() {
        panic!("Could neither discover nor build HiGHS");
    }
}
