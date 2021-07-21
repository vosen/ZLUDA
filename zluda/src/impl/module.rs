use std::{
    collections::hash_map,
    collections::HashMap,
    ffi::c_void,
    ffi::CStr,
    ffi::CString,
    mem,
    os::raw::{c_char, c_int, c_uint},
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
    pub should_link_ptx_impl: Option<&'static [u8]>,
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

    pub fn compile<'a>(
        &self,
        ctx: &ocl_core::Context,
        dev: &ocl_core::DeviceId,
    ) -> Result<ocl_core::Program, CUresult> {
        let byte_il = unsafe {
            slice::from_raw_parts(
                self.binaries.as_ptr() as *const u8,
                self.binaries.len() * mem::size_of::<u32>(),
            )
        };
        let main_module = ocl_core::create_program_with_il(ctx, byte_il, None)?;
        let main_module = match self.should_link_ptx_impl {
            None => {
                ocl_core::build_program(
                    &main_module,
                    Some(&[dev]),
                    &self.build_options,
                    None,
                    None,
                )?;
                main_module
            }
            Some(ptx_impl) => {
                let ptx_impl_prog = ocl_core::create_program_with_il(ctx, ptx_impl, None)?;
                ocl_core::compile_program(
                    &main_module,
                    Some(&[dev]),
                    &self.build_options,
                    &[],
                    &[],
                    None,
                    None,
                    None,
                )?;
                ocl_core::compile_program(
                    &ptx_impl_prog,
                    Some(&[dev]),
                    &self.build_options,
                    &[],
                    &[],
                    None,
                    None,
                    None,
                )?;
                ocl_core::link_program(
                    ctx,
                    Some(&[dev]),
                    &self.build_options,
                    &[&main_module, &ptx_impl_prog],
                    None,
                    None,
                    None,
                )?
            }
        };
        Ok(main_module)
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
                    base: module
                        .spirv
                        .compile(&device.ocl_context, &device.ocl_base)?,
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
                let true_b: ocl_core::ffi::cl_bool = 1;
                let err = unsafe {
                    ocl_core::ffi::clSetKernelExecInfo(
                        kernel.as_ptr(),
                        CL_KERNEL_EXEC_INFO_INDIRECT_HOST_ACCESS_INTEL,
                        mem::size_of::<ocl_core::ffi::cl_bool>(),
                        &true_b as *const _ as *const _,
                    )
                };
                assert_eq!(err, 0);
                let err = unsafe {
                    ocl_core::ffi::clSetKernelExecInfo(
                        kernel.as_ptr(),
                        CL_KERNEL_EXEC_INFO_INDIRECT_DEVICE_ACCESS_INTEL,
                        mem::size_of::<ocl_core::ffi::cl_bool>(),
                        &true_b as *const _ as *const _,
                    )
                };
                assert_eq!(err, 0);
                let err = unsafe {
                    ocl_core::ffi::clSetKernelExecInfo(
                        kernel.as_ptr(),
                        CL_KERNEL_EXEC_INFO_INDIRECT_SHARED_ACCESS_INTEL,
                        mem::size_of::<ocl_core::ffi::cl_bool>(),
                        &true_b as *const _ as *const _,
                    )
                };
                assert_eq!(err, 0);
                entry.insert(Box::new(Function::new(FunctionData {
                    base: kernel,
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
        let l0_module = spirv_data.compile(&device.ocl_context, &device.ocl_base)?;
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
