use std::borrow::Cow;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{self, Read, Write};
use std::ops::Add;
use std::os::raw::c_char;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs, mem, ptr, slice};

use hip_runtime_sys::{
    hipCtxGetCurrent, hipCtxGetDevice, hipDeviceGetAttribute, hipDeviceGetName, hipDeviceProp_t,
    hipError_t, hipGetDeviceProperties, hipGetStreamDeviceId, hipModuleLoadData,
};
use tempfile::NamedTempFile;

use crate::cuda::CUmodule;

pub struct SpirvModule {
    pub binaries: Vec<u32>,
    pub kernel_info: HashMap<String, ptx::KernelInfo>,
    pub should_link_ptx_impl: Option<(&'static [u8], &'static [u8])>,
    pub build_options: CString,
}

impl SpirvModule {
    pub fn new_raw<'a>(text: *const c_char) -> Result<Self, hipError_t> {
        let u8_text = unsafe { CStr::from_ptr(text) };
        let ptx_text = u8_text
            .to_str()
            .map_err(|_| hipError_t::hipErrorInvalidImage)?;
        Self::new(ptx_text)
    }

    pub fn new<'a>(ptx_text: &str) -> Result<Self, hipError_t> {
        let mut errors = Vec::new();
        let ast = ptx::ModuleParser::new()
            .parse(&mut errors, ptx_text)
            .map_err(|_| hipError_t::hipErrorInvalidImage)?;
        if errors.len() > 0 {
            return Err(hipError_t::hipErrorInvalidImage);
        }
        let spirv_module =
            ptx::to_spirv_module(ast).map_err(|_| hipError_t::hipErrorInvalidImage)?;
        Ok(SpirvModule {
            binaries: spirv_module.assemble(),
            kernel_info: spirv_module.kernel_info,
            should_link_ptx_impl: spirv_module.should_link_ptx_impl,
            build_options: spirv_module.build_options,
        })
    }
}

pub(crate) fn load(module: *mut CUmodule, fname: *const i8) -> Result<(), hipError_t> {
    let length = (0..)
        .position(|i| unsafe { *fname.add(i) == 0 })
        .ok_or(hipError_t::hipErrorInvalidValue)?;
    let file_name = CStr::from_bytes_with_nul(unsafe { slice::from_raw_parts(fname as _, length) })
        .map_err(|_| hipError_t::hipErrorInvalidValue)?;
    let valid_file_name = file_name
        .to_str()
        .map_err(|_| hipError_t::hipErrorInvalidValue)?;
    let mut file = File::open(valid_file_name).map_err(|_| hipError_t::hipErrorFileNotFound)?;
    let mut file_buffer = Vec::new();
    file.read_to_end(&mut file_buffer)
        .map_err(|_| hipError_t::hipErrorUnknown)?;
    drop(file);
    load_data(module, file_buffer.as_ptr() as _)
}

pub(crate) fn load_data(
    module: *mut CUmodule,
    image: *const std::ffi::c_void,
) -> Result<(), hipError_t> {
    let spirv_data = SpirvModule::new_raw(image as *const _)?;
    load_data_impl(module, spirv_data)
}

pub fn load_data_impl(pmod: *mut CUmodule, spirv_data: SpirvModule) -> Result<(), hipError_t> {
    let mut dev = 0;
    let err = unsafe { hipCtxGetDevice(&mut dev) };
    if err != hipError_t::hipSuccess {
        return Err(err);
    }
    let mut props = unsafe { mem::zeroed() };
    let err = unsafe { hipGetDeviceProperties(&mut props, dev) };
    let arch_binary = compile_amd(
        &props,
        &[&spirv_data.binaries[..]],
        spirv_data.should_link_ptx_impl,
    )
    .map_err(|_| hipError_t::hipErrorUnknown)?;
    let err = unsafe { hipModuleLoadData(pmod as _, arch_binary.as_ptr() as _) };
    if err != hipError_t::hipSuccess {
        return Err(err);
    }
    Ok(())
}

const LLVM_SPIRV: &'static str = "/home/vosen/amd/llvm-project/build/bin/llvm-spirv";
const AMDGPU: &'static str = "/opt/rocm/";
const AMDGPU_TARGET: &'static str = "amdgcn-amd-amdhsa";
const AMDGPU_BITCODE: [&'static str; 8] = [
    "opencl.bc",
    "ocml.bc",
    "ockl.bc",
    "oclc_correctly_rounded_sqrt_off.bc",
    "oclc_daz_opt_on.bc",
    "oclc_finite_only_off.bc",
    "oclc_unsafe_math_off.bc",
    "oclc_wavefrontsize64_off.bc",
];
const AMDGPU_BITCODE_DEVICE_PREFIX: &'static str = "oclc_isa_version_";

fn compile_amd(
    device_pros: &hipDeviceProp_t,
    spirv_il: &[&[u32]],
    ptx_lib: Option<(&'static [u8], &'static [u8])>,
) -> io::Result<Vec<u8>> {
    let null_terminator = device_pros
        .gcnArchName
        .iter()
        .position(|&x| x == 0)
        .unwrap();
    let gcn_arch_slice = unsafe {
        slice::from_raw_parts(device_pros.gcnArchName.as_ptr() as _, null_terminator + 1)
    };
    let device_name =
        if let Ok(Ok(name)) = CStr::from_bytes_with_nul(gcn_arch_slice).map(|x| x.to_str()) {
            name
        } else {
            return Err(io::Error::new(io::ErrorKind::Other, ""));
        };
    let dir = tempfile::tempdir()?;
    let spirv_files = spirv_il
        .iter()
        .map(|spirv| {
            let mut spirv = NamedTempFile::new_in(&dir)?;
            let spirv_il_u8 = unsafe {
                slice::from_raw_parts(
                    spirv_il.as_ptr() as *const u8,
                    spirv_il.len() * mem::size_of::<u32>(),
                )
            };
            spirv.write_all(spirv_il_u8)?;
            Ok::<_, io::Error>(spirv)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let llvm_spirv_path = match env::var("LLVM_SPIRV") {
        Ok(path) => Cow::Owned(path),
        Err(_) => Cow::Borrowed(LLVM_SPIRV),
    };
    let llvm = NamedTempFile::new_in(&dir)?;
    let to_llvm_cmd = Command::new(&*llvm_spirv_path)
        .arg("-r")
        .arg("-o")
        .arg(llvm.path())
        .args(spirv_files.iter().map(|f| f.path()))
        .status()?;
    assert!(to_llvm_cmd.success());
    if cfg!(debug_assertions) {
        persist_file(llvm.path())?;
    }
    let linked_binary = NamedTempFile::new_in(&dir)?;
    let mut llvm_link = PathBuf::from(AMDGPU);
    llvm_link.push("llvm");
    llvm_link.push("bin");
    llvm_link.push("llvm-link");
    let mut linker_cmd = Command::new(&llvm_link);
    linker_cmd
        .arg("--only-needed")
        .arg("-o")
        .arg(linked_binary.path())
        .arg(llvm.path())
        .args(get_bitcode_paths(device_name));
    if cfg!(debug_assertions) {
        linker_cmd.arg("-v");
    }
    let status = linker_cmd.status()?;
    assert!(status.success());
    if cfg!(debug_assertions) {
        persist_file(linked_binary.path())?;
    }
    let mut ptx_lib_bitcode = NamedTempFile::new_in(&dir)?;
    let compiled_binary = NamedTempFile::new_in(&dir)?;
    let mut clang_exe = PathBuf::from(AMDGPU);
    clang_exe.push("llvm");
    clang_exe.push("bin");
    clang_exe.push("clang");
    let mut compiler_cmd = Command::new(&clang_exe);
    compiler_cmd
        .arg(format!("-mcpu={}", device_name))
        .arg("-ffp-contract=off")
        .arg("-nogpulib")
        .arg("-mno-wavefrontsize64")
        .arg("-O3")
        .arg("-Xlinker")
        .arg("--no-undefined")
        .arg("-target")
        .arg(AMDGPU_TARGET)
        .arg("-o")
        .arg(compiled_binary.path())
        .arg("-x")
        .arg("ir")
        .arg(linked_binary.path());
    if let Some((_, bitcode)) = ptx_lib {
        ptx_lib_bitcode.write_all(bitcode)?;
        compiler_cmd.arg(ptx_lib_bitcode.path());
    };
    if cfg!(debug_assertions) {
        compiler_cmd.arg("-v");
    }
    let status = compiler_cmd.status()?;
    assert!(status.success());
    let mut result = Vec::new();
    let compiled_bin_path = compiled_binary.path();
    let mut compiled_binary = File::open(compiled_bin_path)?;
    compiled_binary.read_to_end(&mut result)?;
    if cfg!(debug_assertions) {
        persist_file(compiled_bin_path)?;
    }
    Ok(result)
}

fn persist_file(path: &Path) -> io::Result<()> {
    let mut persistent = PathBuf::from("/tmp/zluda");
    std::fs::create_dir_all(&persistent)?;
    persistent.push(path.file_name().unwrap());
    std::fs::copy(path, persistent)?;
    Ok(())
}

fn get_bitcode_paths(device_name: &str) -> impl Iterator<Item = PathBuf> {
    let generic_paths = AMDGPU_BITCODE.iter().map(|x| {
        let mut path = PathBuf::from(AMDGPU);
        path.push("amdgcn");
        path.push("bitcode");
        path.push(x);
        path
    });
    let suffix = if let Some(suffix_idx) = device_name.find(':') {
        suffix_idx
    } else {
        device_name.len()
    };
    let mut additional_path = PathBuf::from(AMDGPU);
    additional_path.push("amdgcn");
    additional_path.push("bitcode");
    additional_path.push(format!(
        "{}{}{}",
        AMDGPU_BITCODE_DEVICE_PREFIX,
        &device_name[3..suffix],
        ".bc"
    ));
    generic_paths.chain(std::iter::once(additional_path))
}
