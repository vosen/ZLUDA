use std::ffi::CStr;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::str;
use std::{env, mem};

use bpaf::Bpaf;

mod error;
use error::CompilerError;

const DEFAULT_ARCH: &'static str = "gfx1100";

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version)]
pub struct Options {
    #[bpaf(argument("output-dir"))]
    /// Output directory
    output_dir: Option<PathBuf>,

    #[bpaf(long("arch"))]
    /// Target architecture
    arch: Option<String>,

    #[bpaf(positional("filename"))]
    /// PTX file
    ptx_path: String,
}

fn main() -> ExitCode {
    if let Err(e) = main_core() {
        eprintln!("Error: {}", e);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn main_core() -> Result<(), CompilerError> {
    let opts = options().run();
    let comgr = comgr::Comgr::new()?;

    let ptx_path = Path::new(&opts.ptx_path).to_path_buf();
    let filename_base = ptx_path
        .file_name()
        .map(|osstr| osstr.to_str().unwrap_or("output"))
        .unwrap_or("output");

    let mut output_path = match opts.output_dir {
        Some(value) => value,
        None => match ptx_path.parent() {
            Some(dir) => dir.to_path_buf(),
            None => env::current_dir()?,
        },
    };
    output_path.push(filename_base);

    let arch: String = match opts.arch {
        Some(s) => s,
        None => get_gpu_arch()
            .map(String::from)
            .unwrap_or(DEFAULT_ARCH.to_owned()),
    };

    let ptx = fs::read(&ptx_path).map_err(CompilerError::from)?;
    let ptx = str::from_utf8(&ptx).map_err(CompilerError::from)?;
    let llvm = ptx_to_llvm(ptx).map_err(CompilerError::from)?;

    write_to_file(&llvm.llvm_ir, output_path.with_extension("ll").as_path())?;

    let comgr_hook = |bytes: &Vec<u8>, extension: String| {
        let output_path = output_path.with_extension(extension);
        write_to_file(bytes, &output_path).unwrap();
    };

    comgr::compile_bitcode(
        &comgr,
        &arch,
        &llvm.bitcode,
        &llvm.attributes_bitcode,
        &llvm.linked_bitcode,
        Some(&comgr_hook),
    )
    .map_err(CompilerError::from)?;

    Ok(())
}

fn ptx_to_llvm(ptx: &str) -> Result<LLVMArtifacts, CompilerError> {
    let ast = ptx_parser::parse_module_checked(ptx).map_err(CompilerError::from)?;
    let module = ptx::to_llvm_module(
        ast,
        ptx::Attributes {
            clock_rate: 2124000,
        },
    )
    .map_err(CompilerError::from)?;
    let bitcode = module.llvm_ir.write_bitcode_to_memory().to_vec();
    let linked_bitcode = module.linked_bitcode().to_vec();
    let attributes_bitcode = module.attributes_ir.write_bitcode_to_memory().to_vec();
    let llvm_ir = module.llvm_ir.print_module_to_string().to_bytes().to_vec();
    Ok(LLVMArtifacts {
        bitcode,
        linked_bitcode,
        attributes_bitcode,
        llvm_ir,
    })
}

#[derive(Debug)]
struct LLVMArtifacts {
    bitcode: Vec<u8>,
    linked_bitcode: Vec<u8>,
    attributes_bitcode: Vec<u8>,
    llvm_ir: Vec<u8>,
}

fn get_gpu_arch() -> Result<&'static str, CompilerError> {
    use hip_runtime_sys::*;
    unsafe { hipInit(0) }?;
    let mut dev_props: hipDeviceProp_tR0600 = unsafe { mem::zeroed() };
    unsafe { hipGetDevicePropertiesR0600(&mut dev_props, 0) }?;
    let gcn_arch_name = &dev_props.gcnArchName;
    let gcn_arch_name = unsafe { CStr::from_ptr(gcn_arch_name.as_ptr()) };
    let gcn_arch_name = gcn_arch_name.to_str();
    gcn_arch_name.map_err(CompilerError::from)
}

fn write_to_file(content: &[u8], path: &Path) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content)?;
    file.flush()?;
    println!("Wrote to {}", path.to_str().unwrap());
    Ok(())
}
