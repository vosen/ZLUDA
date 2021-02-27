use argh::FromArgs;
use comgr::Comgr;
use hip_common::CompilationMode;
use hip_runtime_sys::*;
use hiprt_sys::*;
use ptx::ModuleParserExt;
use std::borrow::Cow;
use std::ffi::{c_void, CStr};
use std::path::Path;
use std::rc::Rc;
use std::{ffi::CString, fs, path::PathBuf};
use std::{iter, ptr};

#[derive(FromArgs)]
/// ZLUDA offline compiler
struct CompilerArguments {
    /// LLVM AMDGPU ISA, defaults to "gfx1030"
    #[argh(option, default = "default_isa()")]
    isa: String,
    /// paths to PTX files
    #[argh(positional)]
    inputs: Vec<PathBuf>,
    /// directory with output, will be created if does not exist
    #[argh(option, short = 'o')]
    output: Option<PathBuf>,
    /// name of an OptiX program, if provided PTX will be compiled in raytracing mode
    #[argh(option)]
    rt_program: Option<String>,
    /// compilation mode: 1 - Wave32, 2 - Wave32OnWave64, 3 - DoubleWave32OnWave64, defaults to Wave32
    #[argh(option, short = 'm')]
    mode: Option<u8>,
    /// print LLVM version
    #[argh(switch, short = 'V')]
    version: bool,
}

fn default_isa() -> String {
    "gfx1030".to_string()
}

fn main() {
    let args: CompilerArguments = argh::from_env();
    let comgr = comgr::Comgr::find_and_load().unwrap();
    if args.version {
        println!("{}", comgr.version().unwrap());
        return;
    }
    let mut full_isa = "amdgcn-amd-amdhsa--".to_string();
    full_isa.push_str(&args.isa);
    let main_path = &args.inputs[0];
    let inputs = args
        .inputs
        .iter()
        .map(|input| fs::read_to_string(&input).unwrap())
        .collect::<Vec<_>>();
    let asts = inputs
        .iter()
        .map(|input| ptx::ModuleParser::parse_checked(input).unwrap())
        .collect::<Vec<_>>();
    let output_dir = if let Some(output) = args.output {
        fs::create_dir_all(&output).unwrap();
        Cow::Owned(output)
    } else {
        Cow::Borrowed(args.inputs[0].parent().unwrap())
    };
    match args.rt_program {
        Some(rt_program) => {
            let comgr = Rc::new(comgr);
            if asts.len() != 1 {
                panic!("Raytracing compiler expects single .ptx input file")
            }
            compile_and_dump_raytracing(
                full_isa,
                output_dir,
                &comgr,
                rt_program,
                main_path,
                asts.into_iter().next().unwrap(),
            )
        }
        None => {
            let mode = args
                .mode
                .map(CompilationMode::from_u8)
                .flatten()
                .unwrap_or(CompilationMode::Wave32);
            compile_and_dump(full_isa, output_dir, mode, comgr, main_path, asts)
        }
    }
}

fn compile_and_dump<'input>(
    full_isa: String,
    output_dir: Cow<Path>,
    compilation_mode: CompilationMode,
    comgr: Comgr,
    main_path: &PathBuf,
    asts: Vec<ptx::ast::Module<'input>>,
) {
    let compiled_module = ptx::to_llvm_module(compilation_mode, asts).unwrap();
    let mut output_path = output_dir.to_path_buf();
    output_path.push(main_path.file_name().unwrap());
    output_path.set_extension("ll");
    fs::write(&output_path, compiled_module.get_llvm_text().to_string()).unwrap();
    output_path.set_extension("s");
    let metadata = compiled_module.metadata.to_elf_section();
    fs::write(&output_path, &metadata).unwrap();
    let binary = comgr
        .compile(
            compilation_mode,
            CString::new(&*full_isa).unwrap().as_c_str(),
            ptx::Module::get_bitcode_multi(iter::once(&compiled_module)).into_iter(),
            &metadata,
        )
        .unwrap();
    output_path.set_extension("elf");
    fs::write(&output_path, &binary).unwrap();
}

fn compile_and_dump_raytracing(
    full_isa: String,
    output_dir: Cow<Path>,
    comgr: &Rc<Comgr>,
    rt_program: String,
    filename: &PathBuf,
    ast: ptx::ast::Module,
) {
    let mut empty_variable_block = hip_common::raytracing::VariablesBlock::empty();
    let raytracing_module =
        ptx::to_llvm_module_for_raytracing(ast, &rt_program, &mut empty_variable_block).unwrap();
    let mut output_path = output_dir.into_owned();
    output_path.push(filename.file_name().unwrap());
    output_path.set_extension("ll");
    fs::write(
        &output_path,
        raytracing_module
            .compilation_module
            .get_llvm_text()
            .to_string(),
    )
    .unwrap();
    dump_headers(&output_path, &raytracing_module);
    unsafe { compile_and_dump_relocatable(output_path, raytracing_module, comgr, full_isa) }
}

unsafe fn compile_and_dump_relocatable(
    mut output_path: PathBuf,
    raytracing_module: ptx::raytracing::Module,
    comgr: &Rc<Comgr>,
    full_isa: String,
) {
    let hiprt_bitcode = build_and_dump_hiprt(&mut output_path, &raytracing_module);
    let full_isa = CString::from_vec_unchecked(full_isa.into_bytes());
    let binary = ptx::llvm::MemoryBuffer::create_no_copy(&hiprt_bitcode, false);
    let main_name = CStr::from_bytes_with_nul_unchecked(b"raytracing_main\0");
    let bitcode = comgr
        .link_bitcode(
            raytracing_module.compilation_module.compilation_mode,
            &full_isa,
            iter::once((binary, main_name))
                .chain(raytracing_module.compilation_module.get_bitcode_all()),
        )
        .unwrap();
    dump_bitcode(&output_path, &bitcode);
    let relocatable_object = comgr
        .bitcode_to_relocatable(
            raytracing_module.compilation_module.compilation_mode,
            &full_isa,
            &bitcode,
        )
        .unwrap();
    let relocatable = comgr
        .link_relocatable(&full_isa, iter::once(&relocatable_object))
        .unwrap();
    //let relocatable = relocatable_object.get_data().unwrap();
    output_path.set_extension("elf");
    fs::write(&output_path, relocatable).unwrap();
}

fn dump_bitcode(output_path: &PathBuf, bitcode: &comgr::Bitcode) {
    let mut output_path = output_path.clone();
    let mut file_name = output_path
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    file_name.push_str("_linked.bc");
    output_path.set_file_name(file_name);
    let linked_bitcode = bitcode.get_data().unwrap();
    fs::write(output_path, linked_bitcode).unwrap();
}

unsafe fn build_and_dump_hiprt(
    output_path: &mut PathBuf,
    raytracing_module: &ptx::raytracing::Module,
) -> Vec<u8> {
    let kernel_source = raytracing_module.kernel_source;
    assert_eq!(hipInit(0), hipError_t(0));
    let mut hip_context = ptr::null_mut();
    assert_eq!(hipCtxCreate(&mut hip_context, 0, 0), hipError_t(0));
    let mut context_input = hiprtContextCreationInput {
        ctxt: hip_context as _,
        device: 0,
        deviceType: hiprtDeviceType::hiprtDeviceAMD,
    };
    let hiprt = hiprt_sys::HipRt::load().unwrap();
    let mut context = ptr::null_mut();
    assert!(
        hiprt.hiprtCreateContext(
            hiprt_sys::HIPRT_API_VERSION,
            &mut context_input,
            &mut context,
        ) == hiprtError(0)
    );
    let debug_level = if cfg!(debug_assertions) {
        b"-g\0".as_ptr()
    } else {
        b"-g0\0".as_ptr()
    };
    let options = [
        debug_level,
        // We just want to emit LLVM, we'd use O0, but somehow IR emitted by O0 prevents inling.
        // Weirdly, -disable-llvm-optzns produces much bigger code
        b"-O1\0".as_ptr(),
        // Stop compilation at LLVM
        b"-fgpu-rdc\0".as_ptr(),
        // hiprtc injects -mcumode which we don't want
        b"-mno-cumode\0".as_ptr(),
        // Internalization makes so that _rt_trace_time_mask_flags_64 is module-private
        // and does not get linked with the code generated by ptx compiler
        b"-mllvm\0".as_ptr(),
        b"-amdgpu-internalize-symbols=0\0".as_ptr(),
    ];
    let mut rt_program = ptr::null_mut::<c_void>();
    let headers = raytracing_module
        .headers
        .iter()
        .map(|s| s.as_ptr())
        .collect::<Vec<_>>();
    let header_names = raytracing_module
        .header_names
        .iter()
        .map(|s| s.as_ptr())
        .collect::<Vec<_>>();
    assert!(
        hiprt.hiprtBuildTraceProgram(
            context,
            ptx::raytracing::Module::KERNEL_NAME.as_ptr(),
            kernel_source.as_ptr() as _,
            "zluda_rt_kernel\0".as_ptr() as _,
            headers.len() as i32,
            headers.as_ptr() as _,
            header_names.as_ptr() as _,
            options.as_ptr() as _,
            options.len() as i32,
            (&mut rt_program) as *mut _ as _,
        ) == hiprtError(0)
    );
    let hiprt_bitcode = get_bitcode(rt_program);
    let mut output_path = output_path.clone();
    let mut file_name = output_path
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    file_name.push_str("_hiprt.bc");
    output_path.set_file_name(file_name);
    fs::write(&output_path, &hiprt_bitcode).unwrap();
    hiprt_bitcode
}

#[cfg(windows)]
const HIPRTC: &'static str = "hiprtc\0";

#[cfg(not(windows))]
const HIPRTC: &'static str = "libhiprtc.so\0";

unsafe fn get_bitcode(rt_program: *mut c_void) -> Vec<u8> {
    use libloading::{Library, Symbol};
    let hiprtc = Library::new(HIPRTC).unwrap();
    let hiprtc_get_bitcode_size: Symbol<
        unsafe fn(prog: *mut c_void, bitcode_size: *mut usize) -> u32,
    > = hiprtc.get(b"hiprtcGetBitcodeSize\0").unwrap();
    let hiprtc_get_bitcode: Symbol<unsafe fn(prog: *mut c_void, bitcode: *mut u8) -> u32> =
        hiprtc.get(b"hiprtcGetBitcode\0").unwrap();
    let mut program_size = 0;
    let error = hiprtc_get_bitcode_size(rt_program, &mut program_size);
    if error != 0 {
        panic!("{}", error);
    }
    let mut main_bitcode = vec![0u8; program_size];
    let error = hiprtc_get_bitcode(rt_program, main_bitcode.as_mut_ptr());
    if error != 0 {
        panic!("{}", error);
    }
    main_bitcode
}

fn dump_headers(output_path: &PathBuf, rt_module: &ptx::raytracing::Module) {
    let mut header_path = output_path.clone();
    for (header_name, header) in rt_module.header_names.iter().zip(rt_module.headers.iter()) {
        header_path.set_file_name(header_name.to_str().unwrap());
        fs::write(&header_path, header.as_bytes()).unwrap();
    }
}
