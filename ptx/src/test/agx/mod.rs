//! Compilation of the PTX run tests for Apple GPUs with the AGX backend.
//!
//! There is no AGX runtime yet, so these tests check that the kernels
//! translate, link with zluda_ptx_impl_agx.ll and generate code, and compare
//! the generated code of a few kernels with the expected assembly.

use crate::pass::{self, Target};
use ptx_parser as ast;
use std::path::PathBuf;

fn compile(ptx: &str, cpu: &str) -> Result<llvm_zluda::compile_agx::AgxCompilation, String> {
    compile_with(ptx, cpu, false)
}

fn compile_with(
    ptx: &str,
    cpu: &str,
    mesa_isa: bool,
) -> Result<llvm_zluda::compile_agx::AgxCompilation, String> {
    let ast = ast::parse_module_checked(ptx).map_err(|e| format!("parse: {e:?}"))?;
    let attributes = pass::Attributes {
        clock_rate: 1_398_000,
    };
    let module = pass::to_llvm_module_for_target(ast, attributes, Target::Agx, |_| {})
        .map_err(|e| format!("translate: {e}"))?;
    let impl_ir = module.linked_bitcode();
    llvm_zluda::compile_agx(
        &module.context,
        cpu,
        llvm_zluda::AgxOptions { strict_fp: module.strict_fp, mesa_isa },
        module.llvm_ir,
        impl_ir,
        None,
    )
}

fn spirv_run_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/test/spirv_run")
}

/// Kernels that must compile: integer, floating point, memory, control flow
/// and warp-level tests.
const REQUIRED: &[&str] = &[
    "add",
    "and",
    "atom_add",
    "bfe",
    "bfi",
    "block",
    "brev",
    "call",
    "clz",
    "cvt_rni",
    "cvta",
    "div_approx",
    "fma",
    "global_array",
    "ld_st",
    "ld_st_offset",
    "mad_s32",
    "max",
    "min",
    "mul_hi",
    "mul_lo",
    "neg",
    "not",
    "or",
    "popc",
    "pred_not",
    "rcp",
    "selp",
    "setp",
    "shl",
    "shr",
    "shared_variable",
    "sqrt",
    "sub",
    "vector",
    "xor",
    "shfl_sync_up_b32_pred",
    "vote_all",
    "activemask",
    "bar_red_and_pred",
    "tanh",
    "div_noftz",
    "prmt",
    "dp4a",
    "mma_m16n8k16_f32_f16_f16_f32",
    "mma_m16n8k16_f32_bf16_bf16_f32",
    "mma_m16n8k32_s32_s8_s8_s32",
];

/// Kernels that are not compiled because the backend aborts on them: they
/// convert shared memory addresses to generic ones (`cvta.shared`), and AGX
/// has no generic addressing of threadgroup memory.
const SKIPPED: &[&str] = &[];

#[test]
fn spirv_run_kernels_compile() {
    let mut failures = Vec::new();
    let mut compiled = Vec::new();
    let mut entries: Vec<_> = std::fs::read_dir(spirv_run_dir())
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().map_or(false, |e| e == "ptx"))
        .collect();
    entries.sort();
    for path in &entries {
        let name = path.file_stem().unwrap().to_string_lossy().to_string();
        if SKIPPED.contains(&name.as_str()) {
            failures.push((name, "skipped".to_string()));
            continue;
        }
        let ptx = std::fs::read_to_string(path).unwrap();
        let result = std::panic::catch_unwind(|| compile(&ptx, "agx-g14g"));
        match result {
            Ok(Ok(out)) if !out.object.is_empty() => compiled.push(name),
            Ok(Ok(_)) => failures.push((name, "empty object".to_string())),
            Ok(Err(e)) => failures.push((name, e.lines().next().unwrap_or("").to_string())),
            Err(_) => failures.push((name, "panic".to_string())),
        }
    }
    eprintln!(
        "AGX: {} of {} PTX run tests compile",
        compiled.len(),
        entries.len()
    );
    for (name, error) in &failures {
        eprintln!("  {name}: {error}");
    }
    let missing: Vec<_> = REQUIRED
        .iter()
        .filter(|r| !compiled.iter().any(|c| c == *r))
        .collect();
    assert!(missing.is_empty(), "required kernels failed: {missing:?}");
}

/// In Mesa ISA mode (`zoc --mesa-isa`) the backend reports any instruction that Mesa's asahi compiler would not
/// emit: every run test that compiles must also compile in that mode, for M1 and M2.
#[test]
fn spirv_run_kernels_compile_mesa_isa() {
    let mut entries: Vec<_> = std::fs::read_dir(spirv_run_dir())
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().map_or(false, |e| e == "ptx"))
        .collect();
    entries.sort();
    let mut failures = Vec::new();
    let mut checked = 0;
    for path in &entries {
        let name = path.file_stem().unwrap().to_string_lossy().to_string();
        let ptx = std::fs::read_to_string(path).unwrap();
        if !matches!(std::panic::catch_unwind(|| compile(&ptx, "agx-g14g")), Ok(Ok(_))) {
            continue;
        }
        for cpu in ["agx-g13g", "agx-g14c"] {
            checked += 1;
            match std::panic::catch_unwind(|| compile_with(&ptx, cpu, true)) {
                Ok(Ok(out)) if !out.object.is_empty() => {}
                Ok(Ok(_)) => failures.push(format!("{name} ({cpu}): empty object")),
                Ok(Err(e)) => failures.push(format!("{name} ({cpu}): {}", e.lines().next().unwrap_or(""))),
                Err(_) => failures.push(format!("{name} ({cpu}): panic")),
            }
        }
    }
    eprintln!("AGX Mesa ISA mode: {} of {checked} compilations", checked - failures.len());
    assert!(failures.is_empty(), "{failures:#?}");
}

#[test]
fn vector_add_assembly() {
    let ptx = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/test/vectorAdd_kernel64.ptx"),
    )
    .unwrap();
    for cpu in ["agx-g13g", "agx-g13c", "agx-g14g", "agx-g15g"] {
        let out = compile(&ptx, cpu).unwrap();
        let asm = String::from_utf8(out.assembly).unwrap();
        // The global id is computed from the special registers, the
        // arguments are read through their uniform registers, and the
        // bounds check is an execution mask construct.
        assert!(asm.contains("get_sr"), "{asm}");
        assert!(asm.contains("device_load"), "{asm}");
        assert!(asm.contains("fadd32"), "{asm}");
        assert!(asm.contains("device_store"), "{asm}");
        assert!(asm.contains("if_icmp r0l"), "{asm}");
        assert!(asm.contains("pop_exec r0l, 1"), "{asm}");
        assert!(asm.contains("stop"), "{asm}");
    }
}

#[test]
fn apple9_is_reported() {
    let ptx = std::fs::read_to_string(spirv_run_dir().join("add.ptx")).unwrap();
    // Code generation for the Apple9 encoding is not supported yet, and must
    // fail with a diagnostic rather than produce an object.
    let err = compile(&ptx, "agx-g16g")
        .err()
        .expect("Apple9 compilation succeeded");
    assert!(err.contains("Apple9"), "{err}");
}

/// With ZLUDA_AGX_DUMP_DIR set, write the LLVM IR of each run test (before
/// linking the helpers) for debugging the backend with llc.
#[test]
fn dump_ir() {
    let Some(dir) = std::env::var_os("ZLUDA_AGX_DUMP_DIR") else {
        return;
    };
    let dir = PathBuf::from(dir);
    std::fs::create_dir_all(&dir).unwrap();
    for entry in std::fs::read_dir(spirv_run_dir()).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().map_or(true, |e| e != "ptx") {
            continue;
        }
        let ptx = std::fs::read_to_string(&path).unwrap();
        let Ok(ast) = ast::parse_module_checked(&ptx) else {
            continue;
        };
        let attributes = pass::Attributes { clock_rate: 1 };
        let module = std::panic::catch_unwind(move || {
            pass::to_llvm_module_for_target(ast, attributes, Target::Agx, |_| {})
        });
        if let Ok(Ok(module)) = module {
            let ir = module.llvm_ir.print_module_to_string();
            let name = path.file_stem().unwrap().to_string_lossy().to_string();
            std::fs::write(dir.join(name + ".ll"), ir.to_bytes()).unwrap();
        }
    }
}
