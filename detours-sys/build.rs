use std::error::Error;

#[cfg(not(target_os = "windows"))]
fn main() {}

#[cfg(target_os = "windows")]
fn main() -> Result<(), Box<dyn Error>> {
    windows::main_impl()
}

#[cfg(target_os = "windows")]
mod windows {
    use std::error::Error;

    const CPP_FILES: [&'static str; 5] = [
        "../ext/detours/src/creatwth.cpp",
        "../ext/detours/src/detours.cpp",
        "../ext/detours/src/disasm.cpp",
        "../ext/detours/src/image.cpp",
        "../ext/detours/src/modules.cpp",
    ];

    pub fn main_impl() -> Result<(), Box<dyn Error>> {
        println!("cargo:rerun-if-changed=build/wrapper.h");
        for f in &CPP_FILES {
            println!("cargo:rerun-if-changed={}", f);
        }
        build_detours()
    }

    fn build_detours() -> Result<(), Box<dyn Error>> {
        add_target_options(
            cc::Build::new()
                .include("../ext/detours/src")
                .files(&CPP_FILES),
        )
        .try_compile("detours")?;
        Ok(())
    }
    fn add_target_options(build: &mut cc::Build) -> &mut cc::Build {
        if std::env::var("CARGO_CFG_TARGET_ENV").unwrap() != "msvc" {
            build
                .compiler("clang")
                .cpp(true)
                .flag("-fms-extensions")
                .flag("-Wno-everything")
        } else {
            build
        }
    }
}
