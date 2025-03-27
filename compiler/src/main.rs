use std::env;
use std::error::Error;
use std::ffi::{CStr, CString, OsStr};
use std::fs::{self, File};
use std::io::{self, Write};
use std::mem::MaybeUninit;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::str::{self, FromStr};

use bpaf::Bpaf;

mod error;
use error::CompilerError;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version)]
pub struct Options {
    #[bpaf(external(output_type), optional)]
    output_type: Option<OutputType>,

    #[bpaf(long("linked"))]
    /// Produce linked LLVM IR (in combination with --ll)
    linked: bool,

    #[bpaf(short('o'), argument("file"))]
    /// Output path
    output_path: Option<PathBuf>,

    #[bpaf(positional("filename"))]
    /// PTX file
    ptx_path: String,
}

fn main() -> ExitCode {
    main_core().map_or_else(
        |e| {
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        },
        |_| ExitCode::SUCCESS,
    )
}

fn main_core() -> Result<(), CompilerError> {
    let opts = options().run();

    let output_type = opts.output_type.unwrap_or_default();

    if opts.linked && output_type != OutputType::LlvmIr {
        println!("Warning: option --linked only makes sense when combined with --ll. Ignoring.");
    }

    let ptx_path = Path::new(&opts.ptx_path).to_path_buf();
    check_path(&ptx_path)?;

    let output_path = match opts.output_path {
        Some(value) => value,
        None => get_output_path(&ptx_path, &output_type)?,
    };
    check_path(&output_path)?;

    let ptx = fs::read(&ptx_path).map_err(CompilerError::from)?;
    let ptx = str::from_utf8(&ptx).map_err(CompilerError::from)?;
    let llvm = ptx_to_llvm(ptx).map_err(CompilerError::from)?;

    let output = match output_type {
        OutputType::LlvmIr => {
            if opts.linked {
                get_linked_bitcode(&llvm)?
            } else {
                llvm.llvm_ir
            }
        }
        OutputType::Elf => get_elf(&llvm)?,
        OutputType::Assembly => get_assembly(&llvm)?,
    };

    write_to_file(&output, &output_path).map_err(CompilerError::from)?;
    Ok(())
}

fn ptx_to_llvm(ptx: &str) -> Result<LLVMArtifacts, CompilerError> {
    let ast = ptx_parser::parse_module_checked(ptx).map_err(CompilerError::from)?;
    let module = ptx::to_llvm_module(ast).map_err(CompilerError::from)?;
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

fn get_arch() -> Result<CString, CompilerError> {
    use hip_runtime_sys::*;
    unsafe { hipInit(0) }?;
    let mut dev_props: MaybeUninit<hipDeviceProp_tR0600> = MaybeUninit::uninit();
    unsafe { hipGetDevicePropertiesR0600(dev_props.as_mut_ptr(), 0) }?;
    let dev_props = unsafe { dev_props.assume_init() };
    let arch = dev_props.gcnArchName;
    let arch: Vec<u8> = arch
        .to_vec()
        .iter()
        .map(|&v| i8::to_ne_bytes(v)[0])
        .collect();
    let arch = CStr::from_bytes_until_nul(arch.as_slice())?;
    Ok(CString::from(arch))
}

fn get_linked_bitcode(llvm: &LLVMArtifacts) -> Result<Vec<u8>, CompilerError> {
    let linked_bitcode = comgr::get_linked_bitcode_as_bytes(&llvm.bitcode, &llvm.linked_bitcode)?;
    Ok(ptx::bitcode_to_ir(linked_bitcode))
}

fn get_elf(llvm: &LLVMArtifacts) -> Result<Vec<u8>, CompilerError> {
    let arch = get_arch()?;
    comgr::get_executable_as_bytes(&arch, &llvm.bitcode, &llvm.linked_bitcode)
        .map_err(CompilerError::from)
}

fn get_assembly(llvm: &LLVMArtifacts) -> Result<Vec<u8>, CompilerError> {
    let arch = get_arch()?;
    comgr::get_assembly_as_bytes(&arch, &llvm.bitcode, &llvm.linked_bitcode)
        .map_err(CompilerError::from)
}

fn check_path(path: &Path) -> Result<(), CompilerError> {
    if path.try_exists().map_err(CompilerError::from)? && !path.is_file() {
        let message = format!("Not a regular file: {:?}", path.to_path_buf());
        let error = CompilerError::GenericError {
            cause: None,
            message,
        };
        return Err(error);
    }
    Ok(())
}

fn get_output_path(ptx_path: &PathBuf, output_type: &OutputType) -> Result<PathBuf, CompilerError> {
    let current_dir = env::current_dir().map_err(CompilerError::from)?;
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
    /// Produce LLVM IR
    #[bpaf(long("ll"))]
    LlvmIr,
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
            OutputType::LlvmIr => "ll",
            OutputType::Assembly => "asm",
            OutputType::Elf => "elf",
        }
        .into()
    }
}

impl FromStr for OutputType {
    type Err = CompilerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ll" => Ok(Self::LlvmIr),
            "elf" => Ok(Self::Elf),
            "asm" => Ok(Self::Assembly),
            _ => {
                let message = format!("Not a valid output type: {}", s);
                Err(CompilerError::GenericError {
                    cause: None,
                    message,
                })
            }
        }
    }
}
