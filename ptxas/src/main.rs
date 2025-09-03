use bpaf::{any, doc::Style, Bpaf, Parser};
use hip_runtime_sys::{hipDeviceProp_tR0600, hipGetDevicePropertiesR0600};
use std::{ffi::CStr, mem};

#[derive(Debug, Clone, Bpaf)]
#[allow(dead_code)]
#[bpaf(options, version("Cuda compilation tools, release 12.8, V12.8.0"))]
pub struct Options {
    #[bpaf(short, long)]
    output: String,
    warn_on_spills: bool,
    #[bpaf(short, long)]
    verbose: bool,
    #[bpaf(external)]
    gpu_name: String,
    #[bpaf(long, short('O'), fallback(3))]
    opt_level: usize,
    #[bpaf(positional)]
    input: String,
}

// #[bpaf(long, long("gpu_name"), fallback_with(default_arch))]
fn gpu_name() -> impl Parser<String> {
    any("", move |s: String| {
        Some(s.strip_prefix("-arch=")?.to_owned())
    })
    .metavar(&[("-arch=", Style::Literal), ("ARG", Style::Metavar)])
    .anywhere()
    .fallback_with(|| Ok::<String, &'static str>("sm_52".to_string()))
}

fn main() {
    let options = options().run();
    let comgr = comgr::Comgr::new().unwrap();
    unsafe { hip_runtime_sys::hipInit(0) }.unwrap();
    let mut dev_props: hipDeviceProp_tR0600 = unsafe { mem::zeroed() };
    let (gpu_arch, clock_rate) = get_gpu_arch_and_clock_rate(&mut dev_props);
    let input = std::fs::read_to_string(options.input).unwrap();
    let ast = ptx_parser::parse_module_checked(&input).unwrap();
    let llvm = ptx::to_llvm_module(
        ast,
        ptx::Attributes {
            clock_rate: clock_rate as u32,
        },
    )
    .unwrap();
    let elf_binary = comgr::compile_bitcode(
        &comgr,
        gpu_arch,
        &*llvm.llvm_ir.write_bitcode_to_memory(),
        &*llvm.linked_bitcode(),
        &*llvm.attributes_ir.write_bitcode_to_memory(),
        None,
    )
    .unwrap();
    std::fs::write(options.output, elf_binary).unwrap();
}

fn get_gpu_arch_and_clock_rate<'a>(dev_props: &'a mut hipDeviceProp_tR0600) -> (&'a str, i32) {
    unsafe { hipGetDevicePropertiesR0600(dev_props, 0) }.unwrap();
    let gcn_arch_name = &dev_props.gcnArchName;
    let gcn_arch_name = unsafe { CStr::from_ptr(gcn_arch_name.as_ptr()) };
    let gcn_arch_name = gcn_arch_name.to_str();
    (gcn_arch_name.unwrap(), dev_props.clockRate)
}
