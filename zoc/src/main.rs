use std::env;
use std::error::Error;
use std::ffi::{CStr, OsStr};
use std::fs::{self, File};
use std::io::{self, Write};
use std::mem::MaybeUninit;
use std::path::{Path, PathBuf};
use std::str::{self, FromStr};

use amd_comgr_sys::amd_comgr_status_s;
use bpaf::Bpaf;
use hip_runtime_sys::hipErrorCode_t;
use ptx_parser::PtxError;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version)]
pub struct Options {
    #[bpaf(external(output_type), optional)]
    output_type: Option<OutputType>,

    #[bpaf(positional("filename"))]
    /// PTX file
    ptx_path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = options().run();

    let output_type = opts.output_type.unwrap_or_default();

    match output_type {
        OutputType::LlvmIrLinked | OutputType::Assembly => todo!(),
        _ => {}
    }

    let ptx_path = Path::new(&opts.ptx_path).to_path_buf();
    check_path(&ptx_path)?;

    let output_path = get_output_path(&ptx_path, &output_type)?;
    check_path(&output_path)?;

    let ptx = fs::read(&ptx_path)?;
    let ptx = str::from_utf8(&ptx)?;
    let llvm = ptx_to_llvm(ptx)?;

    if output_type == OutputType::LlvmIrPreLinked {
        write_to_file(&llvm.llvm_ir, &output_path)?;
        return Ok(());
    }

    let elf = llvm_to_elf(&llvm)?;
    write_to_file(&elf, &output_path)?;

    Ok(())
}

fn ptx_to_llvm(ptx: &str) -> Result<LLVMArtifacts, Box<dyn Error>> {
    let ast = ptx_parser::parse_module_checked(ptx).map_err(join_ptx_errors)?;
    let module = ptx::to_llvm_module(ast)?;
    let bitcode = module.llvm_ir.write_bitcode_to_memory().to_vec();
    let linked_bitcode = module.linked_bitcode().to_vec();
    let llvm_ir = module.llvm_ir.print_module_to_string().to_bytes().to_vec();
    Ok(LLVMArtifacts {
        bitcode,
        linked_bitcode,
        llvm_ir,
    })
}

#[derive(Debug)]
struct LLVMArtifacts {
    bitcode: Vec<u8>,
    linked_bitcode: Vec<u8>,
    llvm_ir: Vec<u8>,
}

fn join_ptx_errors(vector: Vec<PtxError>) -> String {
    let errors: Vec<String> = vector.iter().map(PtxError::to_string).collect();
    errors.join("\n")
}

fn llvm_to_elf(llvm: &LLVMArtifacts) -> Result<Vec<u8>, ElfError> {
    use hip_runtime_sys::*;
    unsafe { hipInit(0) }?;
    let mut dev_props: MaybeUninit<hipDeviceProp_tR0600> = MaybeUninit::uninit();
    unsafe { hipGetDevicePropertiesR0600(dev_props.as_mut_ptr(), 0) }?;
    let dev_props = unsafe { dev_props.assume_init() };
    let gcn_arch = unsafe { CStr::from_ptr(dev_props.gcnArchName.as_ptr()) };

    comgr::compile_bitcode(gcn_arch, &llvm.bitcode, &llvm.linked_bitcode).map_err(ElfError::from)
}

fn check_path(path: &Path) -> Result<(), Box<dyn Error>> {
    if path.try_exists()? && !path.is_file() {
        let error = CheckPathError(path.to_path_buf());
        let error = Box::new(error);
        return Err(error);
    }
    Ok(())
}

fn get_output_path(
    ptx_path: &PathBuf,
    output_type: &OutputType,
) -> Result<PathBuf, Box<dyn Error>> {
    let current_dir = env::current_dir()?;
    let output_path = current_dir.join(
        ptx_path
            .as_path()
            .file_stem()
            .unwrap_or(OsStr::new("output")),
    );
    let output_path = output_path.with_extension(output_type.extension());
    Ok(output_path)
}

fn write_to_file(content: &[u8], path: &Path) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content)?;
    file.flush()?;
    println!("Wrote to {}", path.to_str().unwrap());
    Ok(())
}

#[derive(Bpaf, Clone, Copy, Debug, Default, PartialEq)]
enum OutputType {
    /// Produce pre-linked LLVM IR
    #[bpaf(long("ll"))]
    LlvmIrPreLinked,
    /// Produce linked LLVM IR
    #[bpaf(long("linked-ll"))]
    LlvmIrLinked,
    /// Produce ELF binary (default)
    #[default]
    Elf,
    /// Produce assembly
    #[bpaf(long("asm"))]
    Assembly,
}

impl OutputType {
    fn extension(self) -> String {
        match self {
            OutputType::LlvmIrPreLinked | OutputType::LlvmIrLinked => "ll",
            OutputType::Assembly => "asm",
            OutputType::Elf => "elf",
        }
        .into()
    }
}

impl FromStr for OutputType {
    type Err = ParseOutputTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ll" => Ok(Self::LlvmIrPreLinked),
            "ll_linked" => Ok(Self::LlvmIrLinked),
            "elf" => Ok(Self::Elf),
            "asm" => Ok(Self::Assembly),
            _ => Err(ParseOutputTypeError(s.into())),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Not a regular file: {0}")]
struct CheckPathError(PathBuf);

#[derive(Debug, thiserror::Error)]
#[error("Invalid output type: {0}")]
struct ParseOutputTypeError(String);

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
