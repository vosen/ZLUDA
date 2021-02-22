use std::{
    collections::hash_map, collections::HashMap, ffi::c_void, ffi::CStr, ffi::CString, mem,
    os::raw::c_char, ptr, slice,
};

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
    pub base: l0::Module,
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

    pub fn compile(&self, ctx: &mut l0::Context, dev: &l0::Device) -> Result<l0::Module, CUresult> {
        let byte_il = unsafe {
            slice::from_raw_parts(
                self.binaries.as_ptr() as *const u8,
                self.binaries.len() * mem::size_of::<u32>(),
            )
        };
        let l0_module = match self.should_link_ptx_impl {
            None => {
                l0::Module::build_spirv(ctx, dev, byte_il, Some(self.build_options.as_c_str()))
            }
            Some(ptx_impl) => {
                l0::Module::build_link_spirv(
                    ctx,
                    &dev,
                    &[ptx_impl, byte_il],
                    Some(self.build_options.as_c_str()),
                )
                .0
            }
        };
        Ok(l0_module?)
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
                    base: module.spirv.compile(&mut device.l0_context, &device.base)?,
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
                let mut kernel =
                    l0::Kernel::new_resident(&compiled_module.base, entry.key().as_c_str())?;
                kernel.set_indirect_access(
                    l0::sys::ze_kernel_indirect_access_flags_t::ZE_KERNEL_INDIRECT_ACCESS_FLAG_DEVICE
                    | l0::sys::ze_kernel_indirect_access_flags_t::ZE_KERNEL_INDIRECT_ACCESS_FLAG_HOST
                    | l0::sys::ze_kernel_indirect_access_flags_t::ZE_KERNEL_INDIRECT_ACCESS_FLAG_SHARED
                )?;
                entry.insert(Box::new(Function::new(FunctionData {
                    base: kernel,
                    arg_size: kernel_info.arguments_sizes.clone(),
                    use_shared_mem: kernel_info.uses_shared_mem,
                    properties: None,
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
        let l0_module = spirv_data.compile(&mut device.l0_context, &device.base)?;
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
