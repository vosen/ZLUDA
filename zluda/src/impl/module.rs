use std::{
    borrow::Cow,
    collections::hash_map,
    collections::HashMap,
    ffi::c_void,
    ffi::CStr,
    ffi::CString,
    fs::File,
    io::{self, Read, Seek, SeekFrom, Write},
    mem,
    os::raw::{c_char, c_int, c_uint},
    path::PathBuf,
    process::{Command, Stdio},
    ptr, slice,
};

const CL_KERNEL_EXEC_INFO_INDIRECT_HOST_ACCESS_INTEL: u32 = 0x4200;
const CL_KERNEL_EXEC_INFO_INDIRECT_DEVICE_ACCESS_INTEL: u32 = 0x4201;
const CL_KERNEL_EXEC_INFO_INDIRECT_SHARED_ACCESS_INTEL: u32 = 0x4202;

use super::{
    device,
    function::Function,
    function::{FunctionData, LegacyArguments},
    CUresult, GlobalState, HasLivenessCookie, LiveCheck,
};
use ptx;
use tempfile::NamedTempFile;

pub type Module = LiveCheck<ModuleData>;

impl HasLivenessCookie for ModuleData {
    #[cfg(target_pointer_width = "64")]
    const COOKIE: usize = 0xf1313bd46505f98a;

    #[cfg(target_pointer_width = "32")]
    const COOKIE: usize = 0xbdbe3f15;

    const LIVENESS_FAIL: CUresult = CUresult::CUDA_ERROR_INVALID_HANDLE;

    fn try_drop(&mut self) -> Result<(), CUresult> {
        Ok(())
    }
}

pub struct ModuleData {
    pub spirv: SpirvModule,
    // This should be a Vec<>, but I'm feeling lazy
    pub device_binaries: HashMap<device::Index, CompiledModule>,
}

pub struct SpirvModule {
    pub binaries: Vec<u32>,
    pub kernel_info: HashMap<String, ptx::KernelInfo>,
    pub should_link_ptx_impl: Option<(&'static [u8], &'static [u8])>,
    pub build_options: CString,
}

pub struct CompiledModule {
    pub base: ocl_core::Program,
    pub kernels: HashMap<CString, Box<Function>>,
}

impl<L, T, E> From<ptx::ParseError<L, T, E>> for CUresult {
    fn from(_: ptx::ParseError<L, T, E>) -> Self {
        CUresult::CUDA_ERROR_INVALID_PTX
    }
}

impl From<ptx::TranslateError> for CUresult {
    fn from(_: ptx::TranslateError) -> Self {
        CUresult::CUDA_ERROR_INVALID_PTX
    }
}

impl SpirvModule {
    pub fn new_raw<'a>(text: *const c_char) -> Result<Self, CUresult> {
        let u8_text = unsafe { CStr::from_ptr(text) };
        let ptx_text = u8_text
            .to_str()
            .map_err(|_| CUresult::CUDA_ERROR_INVALID_PTX)?;
        Self::new(ptx_text)
    }

    pub fn new<'a>(ptx_text: &str) -> Result<Self, CUresult> {
        let mut errors = Vec::new();
        let ast = ptx::ModuleParser::new().parse(&mut errors, ptx_text)?;
        let spirv_module = ptx::to_spirv_module(ast)?;
        Ok(SpirvModule {
            binaries: spirv_module.assemble(),
            kernel_info: spirv_module.kernel_info,
            should_link_ptx_impl: spirv_module.should_link_ptx_impl,
            build_options: spirv_module.build_options,
        })
    }

    const LLVM_SPIRV: &'static str = "/home/vosen/amd/llvm-project/build/bin/llvm-spirv";
    const AMDGPU: &'static str = "/opt/amdgpu-pro/";
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

    fn get_bitcode_paths(device_name: &str) -> impl Iterator<Item = PathBuf> {
        let generic_paths = Self::AMDGPU_BITCODE.iter().map(|x| {
            let mut path = PathBuf::from(Self::AMDGPU);
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
        let mut additional_path = PathBuf::from(Self::AMDGPU);
        additional_path.push("amdgcn");
        additional_path.push("bitcode");
        additional_path.push(format!(
            "{}{}{}",
            Self::AMDGPU_BITCODE_DEVICE_PREFIX,
            &device_name[3..suffix],
            ".bc"
        ));
        generic_paths.chain(std::iter::once(additional_path))
    }

    #[cfg(not(target_os = "linux"))]
    fn compile_amd(
        device_name: &str,
        spirv_il: &[u8],
        ptx_lib: Option<(&'static [u8], &'static [u8])>,
    ) -> io::Result<Vec<u8>> {
        unimplemented!()
    }

    #[cfg(target_os = "linux")]
    fn compile_amd(
        device_name: &str,
        spirv_il: &[u8],
        ptx_lib: Option<(&'static [u8], &'static [u8])>,
    ) -> io::Result<Vec<u8>> {
        use std::env;
        let dir = tempfile::tempdir()?;
        let mut spirv = NamedTempFile::new_in(&dir)?;
        let llvm = NamedTempFile::new_in(&dir)?;
        spirv.write_all(spirv_il)?;
        let llvm_spirv_path = match env::var("LLVM_SPIRV") {
            Ok(path) => Cow::Owned(path),
            Err(_) => Cow::Borrowed(Self::LLVM_SPIRV),
        };
        let to_llvm_cmd = Command::new(&*llvm_spirv_path)
            .arg("-r")
            .arg("-o")
            .arg(llvm.path())
            .arg(spirv.path())
            .status()?;
        assert!(to_llvm_cmd.success());
        let linked_binary = NamedTempFile::new_in(&dir)?;
        let mut llvm_link = PathBuf::from(Self::AMDGPU);
        llvm_link.push("bin");
        llvm_link.push("llvm-link");
        let mut linker_cmd = Command::new(&llvm_link);
        linker_cmd
            .arg("--only-needed")
            .arg("-o")
            .arg(linked_binary.path())
            .arg(llvm.path())
            .args(Self::get_bitcode_paths(device_name));
        if cfg!(debug_assertions) {
            linker_cmd.arg("-v");
        }
        let status = linker_cmd.status()?;
        assert!(status.success());
        let mut ptx_lib_bitcode = NamedTempFile::new_in(&dir)?;
        let compiled_binary = NamedTempFile::new_in(&dir)?;
        let mut cland_exe = PathBuf::from(Self::AMDGPU);
        cland_exe.push("bin");
        cland_exe.push("clang");
        let mut compiler_cmd = Command::new(&cland_exe);
        compiler_cmd
            .arg(format!("-mcpu={}", device_name))
            .arg("-nogpulib")
            .arg("-mno-wavefrontsize64")
            .arg("-O3")
            .arg("-Xlinker")
            .arg("--no-undefined")
            .arg("-target")
            .arg(Self::AMDGPU_TARGET)
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
        let mut persistent = PathBuf::from("/tmp/zluda");
        std::fs::create_dir_all(&persistent)?;
        persistent.push(compiled_bin_path.file_name().unwrap());
        std::fs::copy(compiled_bin_path, persistent)?;
        Ok(result)
    }

    fn compile_intel<'a>(
        ctx: &ocl_core::Context,
        dev: &ocl_core::DeviceId,
        byte_il: &'a [u8],
        build_options: &CString,
        ptx_lib: Option<(&'static [u8], &'static [u8])>,
    ) -> ocl_core::Result<ocl_core::Program> {
        let main_module = ocl_core::create_program_with_il(ctx, byte_il, None)?;
        Ok(match ptx_lib {
            None => {
                ocl_core::build_program(&main_module, Some(&[dev]), build_options, None, None)?;
                main_module
            }
            Some((ptx_impl_intel, _)) => {
                let ptx_impl_prog = ocl_core::create_program_with_il(ctx, ptx_impl_intel, None)?;
                ocl_core::compile_program(
                    &main_module,
                    Some(&[dev]),
                    build_options,
                    &[],
                    &[],
                    None,
                    None,
                    None,
                )?;
                ocl_core::compile_program(
                    &ptx_impl_prog,
                    Some(&[dev]),
                    build_options,
                    &[],
                    &[],
                    None,
                    None,
                    None,
                )?;
                ocl_core::link_program(
                    ctx,
                    Some(&[dev]),
                    build_options,
                    &[&main_module, &ptx_impl_prog],
                    None,
                    None,
                    None,
                )?
            }
        })
    }

    pub fn compile<'a>(
        &self,
        ctx: &ocl_core::Context,
        dev: &ocl_core::DeviceId,
        device_name: &str,
        is_amd: bool,
    ) -> Result<ocl_core::Program, CUresult> {
        let byte_il = unsafe {
            slice::from_raw_parts(
                self.binaries.as_ptr() as *const u8,
                self.binaries.len() * mem::size_of::<u32>(),
            )
        };
        let ocl_program = if is_amd {
            let binary_prog =
                Self::compile_amd(device_name, byte_il, self.should_link_ptx_impl).unwrap();
            let device = dev.as_raw();
            let binary_len = binary_prog.len();
            let binary = binary_prog.as_ptr();
            let mut binary_status = 0;
            let mut errcode_ret = 0;
            let raw_program = unsafe {
                ocl_core::ffi::clCreateProgramWithBinary(
                    ctx.as_ptr(),
                    1,
                    &device,
                    &binary_len,
                    &binary,
                    &mut binary_status,
                    &mut errcode_ret,
                )
            };
            assert_eq!(binary_status, 0, "clCreateProgramWithBinary");
            assert_eq!(errcode_ret, 0, "clCreateProgramWithBinary");
            let ocl_program = unsafe { ocl_core::Program::from_raw_create_ptr(raw_program) };
            ocl_core::build_program(
                &ocl_program,
                Some(&[dev]),
                &CString::new("").unwrap(),
                None,
                None,
            )?;
            ocl_program
        } else {
            Self::compile_amd("gfx1011:xnack-", byte_il, self.should_link_ptx_impl).unwrap();
            Self::compile_intel(
                ctx,
                dev,
                byte_il,
                &self.build_options,
                self.should_link_ptx_impl,
            )?
        };
        Ok(ocl_program)
    }
}

pub fn get_function(
    hfunc: *mut *mut Function,
    hmod: *mut Module,
    name: *const c_char,
) -> Result<(), CUresult> {
    if hfunc == ptr::null_mut() || hmod == ptr::null_mut() || name == ptr::null() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    let name = unsafe { CStr::from_ptr(name) }.to_owned();
    let function: *mut Function = GlobalState::lock_current_context(|ctx| {
        let module = unsafe { &mut *hmod }.as_result_mut()?;
        let device = unsafe { &mut *ctx.device };
        let compiled_module = match module.device_binaries.entry(device.index) {
            hash_map::Entry::Occupied(entry) => entry.into_mut(),
            hash_map::Entry::Vacant(entry) => {
                let new_module = CompiledModule {
                    base: module.spirv.compile(
                        &device.ocl_context,
                        &device.ocl_base,
                        &device.name,
                        device.is_amd,
                    )?,
                    kernels: HashMap::new(),
                };
                entry.insert(new_module)
            }
        };
        let kernel = match compiled_module.kernels.entry(name) {
            hash_map::Entry::Occupied(entry) => entry.into_mut().as_mut(),
            hash_map::Entry::Vacant(entry) => {
                let kernel_info = module
                    .spirv
                    .kernel_info
                    .get(unsafe {
                        std::str::from_utf8_unchecked(entry.key().as_c_str().to_bytes())
                    })
                    .ok_or(CUresult::CUDA_ERROR_NOT_FOUND)?;
                let kernel = ocl_core::create_kernel(
                    &compiled_module.base,
                    &entry.key().as_c_str().to_string_lossy(),
                )?;
                entry.insert(Box::new(Function::new(FunctionData {
                    base: kernel,
                    device: device.ocl_base.clone(),
                    arg_size: kernel_info.arguments_sizes.clone(),
                    use_shared_mem: kernel_info.uses_shared_mem,
                    legacy_args: LegacyArguments::new(),
                })))
            }
        };
        Ok::<_, CUresult>(kernel as *mut _)
    })??;
    unsafe { *hfunc = function };
    Ok(())
}

pub(crate) fn load_data(pmod: *mut *mut Module, image: *const c_void) -> Result<(), CUresult> {
    let spirv_data = SpirvModule::new_raw(image as *const _)?;
    load_data_impl(pmod, spirv_data)
}

pub fn load_data_impl(pmod: *mut *mut Module, spirv_data: SpirvModule) -> Result<(), CUresult> {
    let module = GlobalState::lock_current_context(|ctx| {
        let device = unsafe { &mut *ctx.device };
        let l0_module = spirv_data.compile(
            &device.ocl_context,
            &device.ocl_base,
            &device.name,
            device.is_amd,
        )?;
        let mut device_binaries = HashMap::new();
        let compiled_module = CompiledModule {
            base: l0_module,
            kernels: HashMap::new(),
        };
        device_binaries.insert(device.index, compiled_module);
        let module_data = ModuleData {
            spirv: spirv_data,
            device_binaries,
        };
        Ok::<_, CUresult>(module_data)
    })??;
    let module_ptr = Box::into_raw(Box::new(Module::new(module)));
    unsafe { *pmod = module_ptr };
    Ok(())
}

pub(crate) fn unload(module: *mut Module) -> Result<(), CUresult> {
    if module == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    GlobalState::lock(|_| Module::destroy_impl(module))?
}

pub(crate) fn load(pmod: *mut *mut Module, fname: *const i8) -> Result<(), CUresult> {
    if pmod == ptr::null_mut() || fname == ptr::null() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    let path = unsafe { CStr::from_ptr(fname) };
    let path_utf8 = path
        .to_str()
        .map_err(|_| CUresult::CUDA_ERROR_INVALID_VALUE)?;
    let file = std::fs::read(path_utf8).map_err(|_| CUresult::CUDA_ERROR_FILE_NOT_FOUND)?;
    let module_text = std::str::from_utf8(&file).map_err(|_| CUresult::CUDA_ERROR_INVALID_PTX)?;
    let spirv_data = SpirvModule::new(module_text)?;
    load_data_impl(pmod, spirv_data)
}
