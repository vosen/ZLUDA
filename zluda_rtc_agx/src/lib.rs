//! NVRTC and nvJitLink for zluda_agx (Asahi Linux).
//!
//! NVRTC compiles CUDA C++ to PTX, which zluda_agx then compiles for the GPU like any other module. The compiler is
//! the system's clang (its NVPTX target; `ZLUDA_NVRTC_CLANG` names another binary), run with clang's own CUDA
//! headers and a prelude that stands in for the CUDA toolkit's: the qualifiers, vector types, atomics, warp
//! intrinsics, and the libdevice math functions (`src/include/agx_nvrtc_math.h`, a few ulp from correctly rounded;
//! the `__*f` fast intrinsics and `--use_fast_math` use the hardware's approximations). Header files given to
//! `nvrtcCreateProgram`, name expressions (lowered names come from the PTX) and the common options are supported;
//! a program that calls a libdevice function the prelude lacks fails to compile with its name in the log.
//! `nvrtcGetCUBIN` returns the PTX too: zluda_agx loads PTX wherever a cubin is expected.
//!
//! nvJitLink hands its inputs (PTX, fat binaries, AGX objects) to the driver's `cuLink*`, which links several PTX
//! modules into one AGX object: the "cubin" `cuModuleLoadData` takes. LTO IR is not supported.

#![allow(non_camel_case_types, non_snake_case)]

use std::ffi::{c_char, c_int, c_uint, c_void, CStr, CString};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

/// `nvrtcResult`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct nvrtcResult(pub u32);

impl nvrtcResult {
    pub const SUCCESS: nvrtcResult = nvrtcResult(0);
    pub const OUT_OF_MEMORY: nvrtcResult = nvrtcResult(1);
    pub const PROGRAM_CREATION_FAILURE: nvrtcResult = nvrtcResult(2);
    pub const INVALID_INPUT: nvrtcResult = nvrtcResult(3);
    pub const INVALID_PROGRAM: nvrtcResult = nvrtcResult(4);
    pub const INVALID_OPTION: nvrtcResult = nvrtcResult(5);
    pub const COMPILATION: nvrtcResult = nvrtcResult(6);
    pub const BUILTIN_OPERATION_FAILURE: nvrtcResult = nvrtcResult(7);
    pub const NO_NAME_EXPRESSIONS_AFTER_COMPILATION: nvrtcResult = nvrtcResult(8);
    pub const NO_LOWERED_NAMES_BEFORE_COMPILATION: nvrtcResult = nvrtcResult(9);
    pub const NAME_EXPRESSION_NOT_VALID: nvrtcResult = nvrtcResult(10);
    pub const INTERNAL_ERROR: nvrtcResult = nvrtcResult(11);
}

/// The NVRTC version reported (12.5).
pub const VERSION: (c_int, c_int) = (12, 5);

/// The architectures `--gpu-architecture` accepts (the PTX targets them all the same way).
pub const ARCHS: [c_int; 14] = [50, 52, 53, 60, 61, 62, 70, 72, 75, 80, 86, 87, 89, 90];

const PRELUDE: &str = include_str!("include/agx_nvrtc_prelude.h");
const MATH: &str = include_str!("include/agx_nvrtc_math.h");
const SM70_RT: &str = include_str!("include/crt/sm_70_rt.hpp");

/// A program: its source and headers, then the result of compiling it.
pub struct Program {
    source: String,
    name: String,
    headers: Vec<(String, String)>,
    names: Vec<String>,
    lowered: Vec<(String, CString)>,
    ptx: Option<CString>,
    log: CString,
}

pub type nvrtcProgram = *mut Program;

/// What the options of `nvrtcCompileProgram` turn into for clang.
#[derive(Debug, Default, PartialEq)]
pub struct Options {
    pub arch: u32,
    pub clang: Vec<String>,
    /// `--device-as-default-execution-space`
    pub default_device: bool,
    /// options that change nothing here
    pub ignored: Vec<String>,
}

/// Translates NVRTC's options.
pub fn options(opts: &[String]) -> Result<Options, String> {
    let mut o = Options { arch: 52, ..Default::default() };
    let mut i = 0;
    // `--name=value`, `-name=value` or `-name value`
    let value = |i: &mut usize, flag: &str, names: &[&str]| -> Option<String> {
        for n in names {
            if let Some(v) = flag.strip_prefix(n).and_then(|r| r.strip_prefix('=')) {
                return Some(v.to_string());
            }
            if flag == *n {
                *i += 1;
                return opts.get(*i).cloned();
            }
        }
        None
    };
    while i < opts.len() {
        let f = opts[i].as_str();
        if let Some(a) = value(&mut i, f, &["--gpu-architecture", "-arch"]) {
            let digits: String = a.chars().filter(|c| c.is_ascii_digit()).collect();
            o.arch = digits.parse().map_err(|_| format!("bad architecture {a}"))?;
        } else if let Some(v) = value(&mut i, f, &["--define-macro", "-D"]).or_else(|| f.strip_prefix("-D").map(String::from)) {
            o.clang.push(format!("-D{v}"));
        } else if let Some(v) = value(&mut i, f, &["--undefine-macro", "-U"]).or_else(|| f.strip_prefix("-U").map(String::from)) {
            o.clang.push(format!("-U{v}"));
        } else if let Some(v) = value(&mut i, f, &["--include-path", "-I"]).or_else(|| f.strip_prefix("-I").map(String::from)) {
            o.clang.push(format!("-I{v}"));
        } else if let Some(v) = value(&mut i, f, &["--pre-include", "-include"]) {
            o.clang.extend(["-include".to_string(), v]);
        } else if let Some(v) = value(&mut i, f, &["--std", "-std"]) {
            o.clang.push(format!("-std={v}"));
        } else if matches!(f, "--use_fast_math" | "-use_fast_math") {
            o.clang.push("-ffast-math".into());
        } else if let Some(v) = value(&mut i, f, &["--ftz", "-ftz"]) {
            if v == "true" {
                o.clang.push("-fgpu-flush-denormals-to-zero".into());
            }
        } else if let Some(v) = value(&mut i, f, &["--fmad", "-fmad"]) {
            o.clang.push(if v == "false" { "-ffp-contract=off" } else { "-ffp-contract=fast" }.into());
        } else if matches!(f, "-G" | "--device-debug" | "-lineinfo" | "--generate-line-info") {
            o.clang.push("-gline-tables-only".into());
        } else if matches!(f, "-dc" | "-rdc=true" | "--relocatable-device-code=true") {
            o.clang.push("-fgpu-rdc".into());
        } else if matches!(f, "-default-device" | "--device-as-default-execution-space") {
            o.default_device = true;
        } else if matches!(f, "-w" | "--disable-warnings") {
            o.clang.push("-w".into());
        } else if matches!(f, "-dlto" | "--dlto" | "-gen-lto") {
            return Err(format!("{f}: LTO IR is not supported"));
        } else {
            o.ignored.push(f.to_string());
        }
        i += 1;
    }
    Ok(o)
}

/// The globals of the name expressions appended to the source: `__agx_nvrtc_name_<i>` holds the address of
/// expression i, so its initializer in the PTX names the lowered symbol.
fn name_globals(names: &[String]) -> String {
    names.iter().enumerate().map(|(i, n)| format!("\n__device__ void *__agx_nvrtc_name_{i} = (void *)&({n});\n")).collect()
}

/// The lowered names of the name expressions, and the PTX without their globals.
pub fn lowered_names(ptx: &str, count: usize) -> (Vec<Option<String>>, String) {
    let mut names = vec![None; count];
    let mut kept = String::with_capacity(ptx.len());
    for line in ptx.split_inclusive('\n') {
        if let Some(rest) = line.split("__agx_nvrtc_name_").nth(1) {
            let index: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
            if let (Ok(i), Some(init)) = (index.parse::<usize>(), rest.split_once('=').map(|x| x.1)) {
                let sym = init.trim().trim_end_matches(';').trim();
                let sym = sym.strip_prefix("generic(").and_then(|s| s.strip_suffix(')')).unwrap_or(sym);
                if i < count {
                    names[i] = Some(sym.to_string());
                }
                continue;
            }
        }
        kept.push_str(line);
    }
    (names, kept)
}

/// The functions the PTX calls but does not define (libdevice functions the prelude lacks).
pub fn missing_functions(ptx: &str) -> Vec<String> {
    ptx.lines()
        .filter_map(|l| l.trim_start().strip_prefix(".extern .func"))
        .map(|rest| {
            let mut rest = rest.trim_start();
            // the return value's parameter list comes first
            if rest.starts_with('(') {
                rest = rest.split_once(')').map_or("", |r| r.1).trim_start();
            }
            rest.chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '$').collect()
        })
        .collect()
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn clang() -> String {
    std::env::var("ZLUDA_NVRTC_CLANG").unwrap_or_else(|_| "clang".into())
}

/// Compiles `p` with clang in a fresh directory holding the prelude, the program's headers and its source.
fn compile(p: &mut Program, opts: &[String]) -> Result<CString, String> {
    let o = options(opts)?;
    let dir = std::env::temp_dir().join(format!("zluda_nvrtc_{}_{}", std::process::id(), COUNTER.fetch_add(1, Ordering::Relaxed)));
    let result = (|| {
        let write = |path: &Path, text: &str| -> Result<(), String> {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            std::fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))
        };
        write(&dir.join("agx_nvrtc_prelude.h"), PRELUDE)?;
        write(&dir.join("agx_nvrtc_math.h"), MATH)?;
        write(&dir.join("crt/sm_70_rt.hpp"), SM70_RT)?;
        let headers = dir.join("headers");
        for (name, text) in &p.headers {
            write(&headers.join(name), text)?;
        }
        let file_name = Path::new(&p.name).file_name().map_or("program.cu".into(), |f| f.to_string_lossy().into_owned());
        let src_path: PathBuf = dir.join(&file_name);
        let mut src = String::new();
        if o.default_device {
            src.push_str("#pragma clang force_cuda_host_device begin\n");
        }
        src.push_str("#line 1 \"");
        src.push_str(&p.name);
        src.push_str("\"\n");
        src.push_str(&p.source);
        src.push('\n');
        if o.default_device {
            src.push_str("#pragma clang force_cuda_host_device end\n");
        }
        src.push_str(&name_globals(&p.names));
        write(&src_path, &src)?;
        let out_path = dir.join("out.ptx");
        let mut cmd = Command::new(clang());
        cmd.args(["-x", "cuda", "--cuda-device-only", "-nocudainc", "-nocudalib", "-Wno-unknown-cuda-version"])
            .arg(format!("--cuda-gpu-arch=sm_{}", o.arch))
            .args(["-Xclang", "-target-feature", "-Xclang", "+ptx70", "-O3", "-std=c++17", "-S"])
            .arg("-I")
            .arg(&dir)
            .arg("-I")
            .arg(&headers)
            .args(["-include", "agx_nvrtc_prelude.h"])
            .args(&o.clang)
            .arg("-o")
            .arg(&out_path)
            .arg(&src_path);
        let run = cmd.output().map_err(|e| format!("{}: {e} (set ZLUDA_NVRTC_CLANG to a clang with the NVPTX target)", clang()))?;
        let mut log = String::from_utf8_lossy(&run.stderr).replace(&format!("{}/", dir.display()), "");
        if !o.ignored.is_empty() {
            log.push_str(&format!("zluda_rtc_agx: ignored options: {}\n", o.ignored.join(" ")));
        }
        if !run.status.success() {
            return Err(log);
        }
        let ptx = std::fs::read_to_string(&out_path).map_err(|e| e.to_string())?;
        let missing = missing_functions(&ptx);
        if !missing.is_empty() {
            return Err(format!("{log}zluda_rtc_agx: these functions are not available: {}\n", missing.join(", ")));
        }
        let (lowered, ptx) = lowered_names(&ptx, p.names.len());
        p.lowered.clear();
        for (expr, name) in p.names.iter().zip(lowered) {
            let name = name.ok_or_else(|| format!("{log}zluda_rtc_agx: no lowered name for {expr}\n"))?;
            p.lowered.push((expr.clone(), CString::new(name).unwrap()));
        }
        p.log = CString::new(log).unwrap_or_default();
        CString::new(ptx).map_err(|e| e.to_string())
    })();
    let _ = std::fs::remove_dir_all(&dir);
    result
}

unsafe fn prog<'a>(p: nvrtcProgram) -> Result<&'a mut Program, nvrtcResult> {
    p.as_mut().ok_or(nvrtcResult::INVALID_PROGRAM)
}

fn done(r: Result<(), nvrtcResult>) -> nvrtcResult {
    r.err().unwrap_or(nvrtcResult::SUCCESS)
}

unsafe fn string(p: *const c_char) -> Option<String> {
    (!p.is_null()).then(|| CStr::from_ptr(p).to_string_lossy().into_owned())
}

unsafe fn copy_out(src: &[u8], dst: *mut c_char) -> Result<(), nvrtcResult> {
    if dst.is_null() {
        return Err(nvrtcResult::INVALID_INPUT);
    }
    std::ptr::copy_nonoverlapping(src.as_ptr(), dst as *mut u8, src.len());
    Ok(())
}

unsafe fn size_out(dst: *mut usize, n: usize) -> Result<(), nvrtcResult> {
    *dst.as_mut().ok_or(nvrtcResult::INVALID_INPUT)? = n;
    Ok(())
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvrtcGetErrorString(r: nvrtcResult) -> *const c_char {
    match r {
        nvrtcResult::SUCCESS => c"NVRTC_SUCCESS",
        nvrtcResult::OUT_OF_MEMORY => c"NVRTC_ERROR_OUT_OF_MEMORY",
        nvrtcResult::PROGRAM_CREATION_FAILURE => c"NVRTC_ERROR_PROGRAM_CREATION_FAILURE",
        nvrtcResult::INVALID_INPUT => c"NVRTC_ERROR_INVALID_INPUT",
        nvrtcResult::INVALID_PROGRAM => c"NVRTC_ERROR_INVALID_PROGRAM",
        nvrtcResult::INVALID_OPTION => c"NVRTC_ERROR_INVALID_OPTION",
        nvrtcResult::COMPILATION => c"NVRTC_ERROR_COMPILATION",
        nvrtcResult::BUILTIN_OPERATION_FAILURE => c"NVRTC_ERROR_BUILTIN_OPERATION_FAILURE",
        nvrtcResult::NO_NAME_EXPRESSIONS_AFTER_COMPILATION => c"NVRTC_ERROR_NO_NAME_EXPRESSIONS_AFTER_COMPILATION",
        nvrtcResult::NO_LOWERED_NAMES_BEFORE_COMPILATION => c"NVRTC_ERROR_NO_LOWERED_NAMES_BEFORE_COMPILATION",
        nvrtcResult::NAME_EXPRESSION_NOT_VALID => c"NVRTC_ERROR_NAME_EXPRESSION_NOT_VALID",
        _ => c"NVRTC_ERROR_INTERNAL_ERROR",
    }
    .as_ptr()
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvrtcVersion(major: *mut c_int, minor: *mut c_int) -> nvrtcResult {
    match (major.as_mut(), minor.as_mut()) {
        (Some(a), Some(b)) => {
            (*a, *b) = VERSION;
            nvrtcResult::SUCCESS
        }
        _ => nvrtcResult::INVALID_INPUT,
    }
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvrtcGetNumSupportedArchs(n: *mut c_int) -> nvrtcResult {
    match n.as_mut() {
        Some(n) => {
            *n = ARCHS.len() as c_int;
            nvrtcResult::SUCCESS
        }
        None => nvrtcResult::INVALID_INPUT,
    }
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvrtcGetSupportedArchs(a: *mut c_int) -> nvrtcResult {
    if a.is_null() {
        return nvrtcResult::INVALID_INPUT;
    }
    std::ptr::copy_nonoverlapping(ARCHS.as_ptr(), a, ARCHS.len());
    nvrtcResult::SUCCESS
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvrtcCreateProgram(
    p: *mut nvrtcProgram,
    src: *const c_char,
    name: *const c_char,
    num_headers: c_int,
    headers: *const *const c_char,
    include_names: *const *const c_char,
) -> nvrtcResult {
    let (Some(p), Some(source)) = (p.as_mut(), string(src)) else {
        return nvrtcResult::INVALID_INPUT;
    };
    if num_headers < 0 || (num_headers > 0 && (headers.is_null() || include_names.is_null())) {
        return nvrtcResult::INVALID_INPUT;
    }
    let mut hs = Vec::new();
    for i in 0..num_headers as usize {
        let (Some(text), Some(name)) = (string(*headers.add(i)), string(*include_names.add(i))) else {
            return nvrtcResult::INVALID_INPUT;
        };
        hs.push((name, text));
    }
    *p = Box::into_raw(Box::new(Program {
        source,
        name: string(name).unwrap_or_else(|| "default_program".into()),
        headers: hs,
        names: Vec::new(),
        lowered: Vec::new(),
        ptx: None,
        log: CString::default(),
    }));
    nvrtcResult::SUCCESS
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvrtcDestroyProgram(p: *mut nvrtcProgram) -> nvrtcResult {
    match p.as_mut() {
        Some(p) if !p.is_null() => {
            drop(Box::from_raw(*p));
            *p = std::ptr::null_mut();
            nvrtcResult::SUCCESS
        }
        _ => nvrtcResult::INVALID_PROGRAM,
    }
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvrtcAddNameExpression(p: nvrtcProgram, name: *const c_char) -> nvrtcResult {
    done((|| {
        let p = prog(p)?;
        if p.ptx.is_some() {
            return Err(nvrtcResult::NO_NAME_EXPRESSIONS_AFTER_COMPILATION);
        }
        p.names.push(string(name).ok_or(nvrtcResult::INVALID_INPUT)?);
        Ok(())
    })())
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvrtcGetLoweredName(p: nvrtcProgram, name: *const c_char, lowered: *mut *const c_char) -> nvrtcResult {
    done((|| {
        let p = prog(p)?;
        let name = string(name).ok_or(nvrtcResult::INVALID_INPUT)?;
        if p.ptx.is_none() {
            return Err(nvrtcResult::NO_LOWERED_NAMES_BEFORE_COMPILATION);
        }
        let l = p.lowered.iter().find(|(n, _)| *n == name).ok_or(nvrtcResult::NAME_EXPRESSION_NOT_VALID)?;
        *lowered.as_mut().ok_or(nvrtcResult::INVALID_INPUT)? = l.1.as_ptr();
        Ok(())
    })())
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvrtcCompileProgram(p: nvrtcProgram, num_options: c_int, options: *const *const c_char) -> nvrtcResult {
    done((|| {
        let p = prog(p)?;
        if num_options < 0 || (num_options > 0 && options.is_null()) {
            return Err(nvrtcResult::INVALID_INPUT);
        }
        let opts: Vec<String> = (0..num_options as usize).filter_map(|i| string(*options.add(i))).collect();
        if let Err(e) = self::options(&opts) {
            p.log = CString::new(e).unwrap_or_default();
            return Err(nvrtcResult::INVALID_OPTION);
        }
        match compile(p, &opts) {
            Ok(ptx) => {
                p.ptx = Some(ptx);
                Ok(())
            }
            Err(log) => {
                p.ptx = None;
                p.log = CString::new(log).unwrap_or_default();
                Err(nvrtcResult::COMPILATION)
            }
        }
    })())
}

unsafe fn ptx_of<'a>(p: nvrtcProgram) -> Result<&'a CString, nvrtcResult> {
    prog(p)?.ptx.as_ref().ok_or(nvrtcResult::INVALID_PROGRAM)
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvrtcGetPTXSize(p: nvrtcProgram, size: *mut usize) -> nvrtcResult {
    done(ptx_of(p).and_then(|ptx| size_out(size, ptx.as_bytes_with_nul().len())))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvrtcGetPTX(p: nvrtcProgram, out: *mut c_char) -> nvrtcResult {
    done(ptx_of(p).and_then(|ptx| copy_out(ptx.as_bytes_with_nul(), out)))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvrtcGetCUBINSize(p: nvrtcProgram, size: *mut usize) -> nvrtcResult {
    nvrtcGetPTXSize(p, size)
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvrtcGetCUBIN(p: nvrtcProgram, out: *mut c_char) -> nvrtcResult {
    nvrtcGetPTX(p, out)
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvrtcGetLTOIRSize(p: nvrtcProgram, _size: *mut usize) -> nvrtcResult {
    done(prog(p).and(Err(nvrtcResult::INVALID_INPUT)))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvrtcGetLTOIR(p: nvrtcProgram, _out: *mut c_char) -> nvrtcResult {
    done(prog(p).and(Err(nvrtcResult::INVALID_INPUT)))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvrtcGetProgramLogSize(p: nvrtcProgram, size: *mut usize) -> nvrtcResult {
    done(prog(p).and_then(|p| size_out(size, p.log.as_bytes_with_nul().len())))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvrtcGetProgramLog(p: nvrtcProgram, out: *mut c_char) -> nvrtcResult {
    done(prog(p).and_then(|p| copy_out(p.log.as_bytes_with_nul(), out)))
}

// ---- nvJitLink

/// `nvJitLinkResult`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct nvJitLinkResult(pub u32);

impl nvJitLinkResult {
    pub const SUCCESS: nvJitLinkResult = nvJitLinkResult(0);
    pub const UNRECOGNIZED_OPTION: nvJitLinkResult = nvJitLinkResult(1);
    pub const MISSING_ARCH: nvJitLinkResult = nvJitLinkResult(2);
    pub const INVALID_INPUT: nvJitLinkResult = nvJitLinkResult(3);
    pub const PTX_COMPILE: nvJitLinkResult = nvJitLinkResult(4);
    pub const NVVM_COMPILE: nvJitLinkResult = nvJitLinkResult(5);
    pub const INTERNAL: nvJitLinkResult = nvJitLinkResult(6);
}

/// `nvJitLinkInputType`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct nvJitLinkInputType(pub u32);

impl nvJitLinkInputType {
    pub const NONE: nvJitLinkInputType = nvJitLinkInputType(0);
    pub const CUBIN: nvJitLinkInputType = nvJitLinkInputType(1);
    pub const PTX: nvJitLinkInputType = nvJitLinkInputType(2);
    pub const LTOIR: nvJitLinkInputType = nvJitLinkInputType(3);
    pub const FATBIN: nvJitLinkInputType = nvJitLinkInputType(4);
    pub const OBJECT: nvJitLinkInputType = nvJitLinkInputType(5);
    pub const LIBRARY: nvJitLinkInputType = nvJitLinkInputType(6);
    pub const INDEX: nvJitLinkInputType = nvJitLinkInputType(7);
    pub const ANY: nvJitLinkInputType = nvJitLinkInputType(10);
}

/// The driver's `CUjitInputType` of an input (ANY guessed from its first bytes).
pub fn jit_input(ty: nvJitLinkInputType, data: &[u8]) -> Result<u32, nvJitLinkResult> {
    const CUBIN: u32 = 0;
    const PTX: u32 = 1;
    const FATBIN: u32 = 2;
    const OBJECT: u32 = 3;
    const LIBRARY: u32 = 4;
    match ty {
        nvJitLinkInputType::CUBIN => Ok(CUBIN),
        nvJitLinkInputType::PTX => Ok(PTX),
        nvJitLinkInputType::FATBIN => Ok(FATBIN),
        nvJitLinkInputType::OBJECT => Ok(OBJECT),
        nvJitLinkInputType::LIBRARY => Ok(LIBRARY),
        nvJitLinkInputType::ANY => Ok(if data.starts_with(b"\x7fELF") {
            CUBIN
        } else if data.len() >= 4 && matches!(u32::from_le_bytes(data[..4].try_into().unwrap()), 0x466243b1 | 0xba55ed50) {
            FATBIN
        } else {
            PTX
        }),
        _ => Err(nvJitLinkResult::INVALID_INPUT),
    }
}

type LinkCreate = unsafe extern "system" fn(c_uint, *mut c_int, *mut *mut c_void, *mut *mut c_void) -> c_uint;
type LinkAddData = unsafe extern "system" fn(*mut c_void, c_uint, *mut c_void, usize, *const c_char, c_uint, *mut c_int, *mut *mut c_void) -> c_uint;
type LinkComplete = unsafe extern "system" fn(*mut c_void, *mut *mut c_void, *mut usize) -> c_uint;
type LinkDestroy = unsafe extern "system" fn(*mut c_void) -> c_uint;

/// The driver's linker (zluda_agx's libcuda).
struct Linker {
    _lib: libloading::Library,
    create: LinkCreate,
    add: LinkAddData,
    complete: LinkComplete,
    destroy: LinkDestroy,
}

fn linker() -> Option<&'static Linker> {
    static L: std::sync::OnceLock<Option<Linker>> = std::sync::OnceLock::new();
    L.get_or_init(|| unsafe {
        let name = std::env::var_os("ZLUDA_AGX_LIBCUDA").unwrap_or_else(|| "libcuda.so.1".into());
        let lib = libloading::Library::new(name).ok()?;
        Some(Linker {
            create: *lib.get(b"cuLinkCreate_v2\0").ok()?,
            add: *lib.get(b"cuLinkAddData_v2\0").ok()?,
            complete: *lib.get(b"cuLinkComplete\0").ok()?,
            destroy: *lib.get(b"cuLinkDestroy\0").ok()?,
            _lib: lib,
        })
    })
    .as_ref()
}

/// A link: its inputs, then the linked object.
pub struct JitLink {
    inputs: Vec<(u32, Vec<u8>, CString)>,
    linked: Option<Vec<u8>>,
    log: CString,
}

pub type nvJitLinkHandle = *mut JitLink;

fn jl_done(r: Result<(), nvJitLinkResult>) -> nvJitLinkResult {
    r.err().unwrap_or(nvJitLinkResult::SUCCESS)
}

unsafe fn jl<'a>(h: nvJitLinkHandle) -> Result<&'a mut JitLink, nvJitLinkResult> {
    h.as_mut().ok_or(nvJitLinkResult::INVALID_INPUT)
}

unsafe fn jl_create(h: *mut nvJitLinkHandle, n: c_uint, opts: *const *const c_char) -> nvJitLinkResult {
    let Some(h) = h.as_mut() else {
        return nvJitLinkResult::INVALID_INPUT;
    };
    let opts: Vec<String> = (0..n as usize).filter_map(|i| string(*opts.add(i))).collect();
    if !opts.iter().any(|o| o.starts_with("-arch")) {
        return nvJitLinkResult::MISSING_ARCH;
    }
    if opts.iter().any(|o| o == "-lto") {
        return nvJitLinkResult::UNRECOGNIZED_OPTION;
    }
    *h = Box::into_raw(Box::new(JitLink { inputs: Vec::new(), linked: None, log: CString::default() }));
    nvJitLinkResult::SUCCESS
}

unsafe fn jl_add(h: nvJitLinkHandle, ty: nvJitLinkInputType, data: *const c_void, size: usize, name: *const c_char) -> nvJitLinkResult {
    jl_done((|| {
        let l = jl(h)?;
        if data.is_null() {
            return Err(nvJitLinkResult::INVALID_INPUT);
        }
        let bytes = std::slice::from_raw_parts(data as *const u8, size).to_vec();
        if ty == nvJitLinkInputType::LTOIR {
            l.log = c"zluda_rtc_agx: LTO IR is not supported".into();
            return Err(nvJitLinkResult::INVALID_INPUT);
        }
        let t = jit_input(ty, &bytes)?;
        let mut bytes = bytes;
        if t == 1 && bytes.last() != Some(&0) {
            bytes.push(0);
        }
        l.inputs.push((t, bytes, CString::new(string(name).unwrap_or_default()).unwrap_or_default()));
        Ok(())
    })())
}

unsafe fn jl_complete(h: nvJitLinkHandle) -> nvJitLinkResult {
    jl_done((|| {
        let l = jl(h)?;
        let d = linker().ok_or(nvJitLinkResult::INTERNAL)?;
        let mut state = std::ptr::null_mut();
        if (d.create)(0, std::ptr::null_mut(), std::ptr::null_mut(), &mut state) != 0 {
            return Err(nvJitLinkResult::INTERNAL);
        }
        let r = (|| {
            for (t, bytes, name) in &l.inputs {
                if (d.add)(state, *t, bytes.as_ptr() as *mut c_void, bytes.len(), name.as_ptr(), 0, std::ptr::null_mut(), std::ptr::null_mut()) != 0 {
                    l.log = CString::new(format!("zluda_rtc_agx: the driver did not take input {}", name.to_string_lossy())).unwrap_or_default();
                    return Err(nvJitLinkResult::INVALID_INPUT);
                }
            }
            let (mut out, mut size) = (std::ptr::null_mut(), 0usize);
            if (d.complete)(state, &mut out, &mut size) != 0 {
                l.log = c"zluda_rtc_agx: linking failed (ZLUDA_AGX_LOG=1 shows why)".into();
                return Err(nvJitLinkResult::PTX_COMPILE);
            }
            l.linked = Some(std::slice::from_raw_parts(out as *const u8, size).to_vec());
            Ok(())
        })();
        (d.destroy)(state);
        r
    })())
}

unsafe fn linked<'a>(h: nvJitLinkHandle) -> Result<&'a [u8], nvJitLinkResult> {
    jl(h)?.linked.as_deref().ok_or(nvJitLinkResult::INVALID_INPUT)
}

/// The PTX of a link of exactly one PTX input (LTO, which produces PTX, is not supported).
unsafe fn linked_ptx<'a>(h: nvJitLinkHandle) -> Result<&'a [u8], nvJitLinkResult> {
    let l = jl(h)?;
    match &l.inputs[..] {
        [(1, ptx, _)] if l.linked.is_some() => Ok(ptx),
        _ => Err(nvJitLinkResult::INVALID_INPUT),
    }
}

/// nvJitLink's functions, under their plain names and the versioned ones (`__nvJitLinkCreate_12_5`, ...) the
/// header's macros call.
macro_rules! jitlink_api {
    ($($m:ident $v:literal)*) => {$(
        #[cfg_attr(test, allow(dead_code))]
        mod $m {
            use super::*;

            #[cfg_attr(not(test), export_name = concat!("__nvJitLinkCreate_", $v))]
            pub unsafe extern "C" fn create(h: *mut nvJitLinkHandle, n: c_uint, opts: *const *const c_char) -> nvJitLinkResult {
                jl_create(h, n, opts)
            }
            #[cfg_attr(not(test), export_name = concat!("__nvJitLinkDestroy_", $v))]
            pub unsafe extern "C" fn destroy(h: *mut nvJitLinkHandle) -> nvJitLinkResult {
                nvJitLinkDestroy(h)
            }
            #[cfg_attr(not(test), export_name = concat!("__nvJitLinkAddData_", $v))]
            pub unsafe extern "C" fn add_data(h: nvJitLinkHandle, ty: nvJitLinkInputType, data: *const c_void, size: usize, name: *const c_char) -> nvJitLinkResult {
                jl_add(h, ty, data, size, name)
            }
            #[cfg_attr(not(test), export_name = concat!("__nvJitLinkAddFile_", $v))]
            pub unsafe extern "C" fn add_file(h: nvJitLinkHandle, ty: nvJitLinkInputType, file: *const c_char) -> nvJitLinkResult {
                nvJitLinkAddFile(h, ty, file)
            }
            #[cfg_attr(not(test), export_name = concat!("__nvJitLinkComplete_", $v))]
            pub unsafe extern "C" fn complete(h: nvJitLinkHandle) -> nvJitLinkResult {
                jl_complete(h)
            }
            #[cfg_attr(not(test), export_name = concat!("__nvJitLinkGetLinkedCubinSize_", $v))]
            pub unsafe extern "C" fn cubin_size(h: nvJitLinkHandle, size: *mut usize) -> nvJitLinkResult {
                nvJitLinkGetLinkedCubinSize(h, size)
            }
            #[cfg_attr(not(test), export_name = concat!("__nvJitLinkGetLinkedCubin_", $v))]
            pub unsafe extern "C" fn cubin(h: nvJitLinkHandle, out: *mut c_void) -> nvJitLinkResult {
                nvJitLinkGetLinkedCubin(h, out)
            }
            #[cfg_attr(not(test), export_name = concat!("__nvJitLinkGetLinkedPtxSize_", $v))]
            pub unsafe extern "C" fn ptx_size(h: nvJitLinkHandle, size: *mut usize) -> nvJitLinkResult {
                nvJitLinkGetLinkedPtxSize(h, size)
            }
            #[cfg_attr(not(test), export_name = concat!("__nvJitLinkGetLinkedPtx_", $v))]
            pub unsafe extern "C" fn ptx(h: nvJitLinkHandle, out: *mut c_char) -> nvJitLinkResult {
                nvJitLinkGetLinkedPtx(h, out)
            }
            #[cfg_attr(not(test), export_name = concat!("__nvJitLinkGetErrorLogSize_", $v))]
            pub unsafe extern "C" fn error_log_size(h: nvJitLinkHandle, size: *mut usize) -> nvJitLinkResult {
                nvJitLinkGetErrorLogSize(h, size)
            }
            #[cfg_attr(not(test), export_name = concat!("__nvJitLinkGetErrorLog_", $v))]
            pub unsafe extern "C" fn error_log(h: nvJitLinkHandle, out: *mut c_char) -> nvJitLinkResult {
                nvJitLinkGetErrorLog(h, out)
            }
            #[cfg_attr(not(test), export_name = concat!("__nvJitLinkGetInfoLogSize_", $v))]
            pub unsafe extern "C" fn info_log_size(h: nvJitLinkHandle, size: *mut usize) -> nvJitLinkResult {
                nvJitLinkGetInfoLogSize(h, size)
            }
            #[cfg_attr(not(test), export_name = concat!("__nvJitLinkGetInfoLog_", $v))]
            pub unsafe extern "C" fn info_log(h: nvJitLinkHandle, out: *mut c_char) -> nvJitLinkResult {
                nvJitLinkGetInfoLog(h, out)
            }
        }
    )*};
}

jitlink_api!(v12_0 "12_0" v12_1 "12_1" v12_2 "12_2" v12_3 "12_3" v12_4 "12_4" v12_5 "12_5" v12_6 "12_6" v12_7 "12_7" v12_8 "12_8" v12_9 "12_9");

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvJitLinkCreate(h: *mut nvJitLinkHandle, n: c_uint, opts: *const *const c_char) -> nvJitLinkResult {
    jl_create(h, n, opts)
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvJitLinkDestroy(h: *mut nvJitLinkHandle) -> nvJitLinkResult {
    match h.as_mut() {
        Some(p) if !p.is_null() => {
            drop(Box::from_raw(*p));
            *p = std::ptr::null_mut();
            nvJitLinkResult::SUCCESS
        }
        _ => nvJitLinkResult::INVALID_INPUT,
    }
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvJitLinkAddData(
    h: nvJitLinkHandle,
    ty: nvJitLinkInputType,
    data: *const c_void,
    size: usize,
    name: *const c_char,
) -> nvJitLinkResult {
    jl_add(h, ty, data, size, name)
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvJitLinkAddFile(h: nvJitLinkHandle, ty: nvJitLinkInputType, file: *const c_char) -> nvJitLinkResult {
    let Some(path) = string(file) else {
        return nvJitLinkResult::INVALID_INPUT;
    };
    match std::fs::read(&path) {
        Ok(data) => jl_add(h, ty, data.as_ptr() as *const c_void, data.len(), file),
        Err(_) => nvJitLinkResult::INVALID_INPUT,
    }
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvJitLinkComplete(h: nvJitLinkHandle) -> nvJitLinkResult {
    jl_complete(h)
}

fn jl_size(dst: *mut usize, n: usize) -> Result<(), nvJitLinkResult> {
    unsafe { *dst.as_mut().ok_or(nvJitLinkResult::INVALID_INPUT)? = n };
    Ok(())
}

unsafe fn jl_copy(src: &[u8], dst: *mut c_void) -> Result<(), nvJitLinkResult> {
    if dst.is_null() {
        return Err(nvJitLinkResult::INVALID_INPUT);
    }
    std::ptr::copy_nonoverlapping(src.as_ptr(), dst as *mut u8, src.len());
    Ok(())
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvJitLinkGetLinkedCubinSize(h: nvJitLinkHandle, size: *mut usize) -> nvJitLinkResult {
    jl_done(linked(h).and_then(|c| jl_size(size, c.len())))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvJitLinkGetLinkedCubin(h: nvJitLinkHandle, out: *mut c_void) -> nvJitLinkResult {
    jl_done(linked(h).and_then(|c| jl_copy(c, out)))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvJitLinkGetLinkedPtxSize(h: nvJitLinkHandle, size: *mut usize) -> nvJitLinkResult {
    jl_done(linked_ptx(h).and_then(|p| jl_size(size, p.len())))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvJitLinkGetLinkedPtx(h: nvJitLinkHandle, out: *mut c_char) -> nvJitLinkResult {
    jl_done(linked_ptx(h).and_then(|p| jl_copy(p, out.cast())))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvJitLinkGetErrorLogSize(h: nvJitLinkHandle, size: *mut usize) -> nvJitLinkResult {
    jl_done(jl(h).and_then(|l| jl_size(size, l.log.as_bytes_with_nul().len())))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvJitLinkGetErrorLog(h: nvJitLinkHandle, out: *mut c_char) -> nvJitLinkResult {
    jl_done(jl(h).and_then(|l| jl_copy(l.log.as_bytes_with_nul(), out.cast())))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvJitLinkGetInfoLogSize(h: nvJitLinkHandle, size: *mut usize) -> nvJitLinkResult {
    jl_done(jl(h).and_then(|_| jl_size(size, 1)))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvJitLinkGetInfoLog(h: nvJitLinkHandle, out: *mut c_char) -> nvJitLinkResult {
    jl_done(jl(h).and_then(|_| jl_copy(b"\0", out.cast())))
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "C" fn nvJitLinkVersion(major: *mut c_uint, minor: *mut c_uint) -> nvJitLinkResult {
    match (major.as_mut(), minor.as_mut()) {
        (Some(a), Some(b)) => {
            (*a, *b) = (VERSION.0 as c_uint, VERSION.1 as c_uint);
            nvJitLinkResult::SUCCESS
        }
        _ => nvJitLinkResult::INVALID_INPUT,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn opts(o: &[&str]) -> Vec<String> {
        o.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn option_translation() {
        let o =
            options(&opts(&["--gpu-architecture=compute_80", "-DN=4", "-I", "inc", "--std=c++20", "-default-device", "--use_fast_math", "-dopt=on"])).unwrap();
        assert_eq!(o.arch, 80);
        assert_eq!(o.clang, ["-DN=4", "-Iinc", "-std=c++20", "-ffast-math"]);
        assert!(o.default_device);
        assert_eq!(o.ignored, ["-dopt=on"]);
        assert_eq!(options(&opts(&["-arch", "sm_75", "-fmad=false"])).unwrap().clang, ["-ffp-contract=off"]);
        assert!(options(&opts(&["-dlto"])).is_err());
    }

    #[test]
    fn lowered_names_from_the_ptx() {
        let ptx = ".visible .global .align 4 .u32 _ZN2ns7counterE;\n\
                   .visible .global .align 8 .u64 __agx_nvrtc_name_0 = _ZN2ns5scaleIfLi3EEEvPT_S1_;\n\
                   .visible .global .align 8 .u64 __agx_nvrtc_name_1 = generic(_ZN2ns7counterE);\n.visible .entry k()\n";
        let (names, kept) = lowered_names(ptx, 2);
        assert_eq!(names, [Some("_ZN2ns5scaleIfLi3EEEvPT_S1_".to_string()), Some("_ZN2ns7counterE".to_string())]);
        assert!(!kept.contains("__agx_nvrtc_name") && kept.contains("_ZN2ns7counterE;") && kept.contains(".entry k"));
        assert_eq!(missing_functions(".extern .func  (.param .b32 func_retval0) __nv_j0f\n(\n.param .b32 a\n);\n"), ["__nv_j0f"]);
        assert_eq!(missing_functions(".extern .func __nv_foo(.param .b32 a);"), ["__nv_foo"]);
    }

    #[test]
    fn jit_input_types() {
        assert_eq!(jit_input(nvJitLinkInputType::ANY, b"\x7fELF...."), Ok(0));
        assert_eq!(jit_input(nvJitLinkInputType::ANY, b"//\n.version 7.0"), Ok(1));
        assert_eq!(jit_input(nvJitLinkInputType::ANY, &0xba55ed50u32.to_le_bytes()), Ok(2));
        assert_eq!(jit_input(nvJitLinkInputType::LTOIR, b""), Err(nvJitLinkResult::INVALID_INPUT));
    }

    /// The whole NVRTC path with the system's clang, when it has the NVPTX target.
    #[test]
    fn compiles_cuda_to_ptx() {
        if Command::new(clang()).arg("--version").output().is_err() {
            eprintln!("no clang: skipped");
            return;
        }
        let src = c"#include \"util.h\"\n\
            template <typename T> __global__ void axpy(T a, const T *x, T *y, int n) {\n\
              int i = blockIdx.x * blockDim.x + threadIdx.x;\n\
              if (i < n) y[i] = twice(a) * x[i] + y[i] + sinf(x[i]) + (T)exp((double)x[i]);\n\
              atomicAdd(&y[0], (T)1); }\n";
        let header = c"__device__ inline float twice(float a) { return 2 * a; }";
        let header_name = c"util.h";
        unsafe {
            let mut p = std::ptr::null_mut();
            assert_eq!(nvrtcCreateProgram(&mut p, src.as_ptr(), c"axpy.cu".as_ptr(), 1, &header.as_ptr(), &header_name.as_ptr()), nvrtcResult::SUCCESS);
            assert_eq!(nvrtcAddNameExpression(p, c"axpy<float>".as_ptr()), nvrtcResult::SUCCESS);
            let o = [c"--gpu-architecture=compute_80".as_ptr(), c"-default-device".as_ptr()];
            let r = nvrtcCompileProgram(p, 2, o.as_ptr());
            let mut n = 0;
            nvrtcGetProgramLogSize(p, &mut n);
            let mut log = vec![0u8; n];
            nvrtcGetProgramLog(p, log.as_mut_ptr() as *mut c_char);
            assert_eq!(r, nvrtcResult::SUCCESS, "{}", String::from_utf8_lossy(&log));
            let mut lowered = std::ptr::null();
            assert_eq!(nvrtcGetLoweredName(p, c"axpy<float>".as_ptr(), &mut lowered), nvrtcResult::SUCCESS);
            let lowered = CStr::from_ptr(lowered).to_str().unwrap().to_string();
            assert_eq!(lowered, "_Z4axpyIfEvT_PKS0_PS0_i");
            nvrtcGetPTXSize(p, &mut n);
            let mut ptx = vec![0u8; n];
            nvrtcGetPTX(p, ptx.as_mut_ptr() as *mut c_char);
            let ptx = String::from_utf8_lossy(&ptx[..n - 1]).into_owned();
            assert!(ptx.contains(&format!(".visible .entry {lowered}(")), "{ptx}");
            assert!(!ptx.contains(".extern") && !ptx.contains("__agx_nvrtc_name"));
            assert_eq!(nvrtcAddNameExpression(p, c"x".as_ptr()), nvrtcResult::NO_NAME_EXPRESSIONS_AFTER_COMPILATION);
            if let Some(dir) = std::env::var_os("ZLUDA_NVRTC_TEST_PTX") {
                std::fs::write(Path::new(&dir).join("axpy.ptx"), &ptx).unwrap();
            }
            nvrtcDestroyProgram(&mut p);
            // a compile error comes back in the log
            let mut q = std::ptr::null_mut();
            nvrtcCreateProgram(&mut q, c"__global__ void k() { undeclared(); }".as_ptr(), c"bad.cu".as_ptr(), 0, std::ptr::null(), std::ptr::null());
            assert_eq!(nvrtcCompileProgram(q, 0, std::ptr::null()), nvrtcResult::COMPILATION);
            nvrtcGetProgramLogSize(q, &mut n);
            let mut log = vec![0u8; n];
            nvrtcGetProgramLog(q, log.as_mut_ptr() as *mut c_char);
            assert!(String::from_utf8_lossy(&log).contains("bad.cu:1"), "{}", String::from_utf8_lossy(&log));
            nvrtcDestroyProgram(&mut q);
        }
    }
}
