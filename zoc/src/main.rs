use std::error::Error;
use std::ffi::CStr;
use std::fs::{self, File};
use std::io::{self, Write};
use std::mem::MaybeUninit;
use std::path::Path;
use std::str;

use amd_comgr_sys::amd_comgr_status_s;
use bpaf::Bpaf;
use hip_runtime_sys::hipErrorCode_t;
use ptx_parser::PtxError;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version)]
pub struct Options {
    #[bpaf(positional("file"))]
    /// PTX file
    file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let options = options().run();
    let ptx_path = options.file;
    let ptx_path = Path::new(&ptx_path);

    let ptx = fs::read(ptx_path)?;
    let ptx = str::from_utf8(&ptx)?;
    let llvm_artifacts = ptx_to_llvm(ptx)?;
    
    let ll_path = ptx_path.with_extension("ll");
    write_to_file(&llvm_artifacts.ll, &ll_path)?;

    let elf_module = llvm_to_elf(&llvm_artifacts)?;
    let elf_path = ptx_path.with_extension("elf");
    write_to_file(&elf_module, &elf_path)?;

    Ok(())
}

fn join_ptx_errors(vector: Vec<PtxError>) -> String {
    let errors: Vec<String> = vector.iter().map(PtxError::to_string).collect();
    errors.join("\n")
}

fn ptx_to_llvm(ptx: &str) -> Result<LLVMArtifacts, Box<dyn Error>> {
    let ast = ptx_parser::parse_module_checked(ptx).map_err(join_ptx_errors)?;
    let module = ptx::to_llvm_module(ast)?;
    let bitcode = module.llvm_ir.write_bitcode_to_memory().to_vec();
    let linked_bitcode = module.linked_bitcode().to_vec();
    let ll = module.llvm_ir.print_module_to_string().to_bytes().to_vec();
    Ok(LLVMArtifacts { bitcode, linked_bitcode, ll })
}

fn llvm_to_elf(module: &LLVMArtifacts) -> Result<Vec<u8>, ElfError> {
    use hip_runtime_sys::*;
    let mut dev_props: MaybeUninit<hipDeviceProp_tR0600> = MaybeUninit::uninit();
    unsafe { hipGetDevicePropertiesR0600(dev_props.as_mut_ptr(), 0) }?;
    let dev_props = unsafe { dev_props.assume_init() };

    comgr::compile_bitcode(
        unsafe { CStr::from_ptr(dev_props.gcnArchName.as_ptr()) },
        &module.bitcode,
        &module.linked_bitcode,
    ).map_err(ElfError::from)
}

fn write_to_file(content: &[u8], path: &Path) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content)?;
    file.flush()?;
    Ok(())
}

#[derive(Debug)]
struct LLVMArtifacts {
    bitcode: Vec<u8>,
    linked_bitcode: Vec<u8>,
    ll: Vec<u8>,
}

#[derive(Debug, thiserror::Error)]
enum ElfError {
    #[error("HIP error: {0:?}")]
    HipError(hipErrorCode_t),
    #[error("amd_comgr error: {0:?}")]
    AmdComgrError(amd_comgr_status_s),
}

impl From<hipErrorCode_t> for ElfError {
    fn from(value: hipErrorCode_t) -> Self {
        ElfError::HipError(value)
    }
}

impl From<amd_comgr_status_s> for ElfError {
    fn from(value: amd_comgr_status_s) -> Self {
        ElfError::AmdComgrError(value)
    }
}