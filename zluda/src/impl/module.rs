use super::context::Context;
use super::{context, function, LiveCheck, ZludaObject};
use crate::hip_call_cuda;
use crate::r#impl::function::FunctionData;
use crate::r#impl::{comgr_error_to_cuda, device, hipfix, GLOBAL_STATE};
use cuda_types::{CUmoduleLoadingMode, CUresult};
use hip_common::CompilationMode;
use hip_runtime_sys::*;
use ptx::ModuleParserExt;
use rustc_hash::FxHashMap;
use std::borrow::Cow;
use std::cmp;
use std::collections::hash_map;
use std::ffi::{CStr, CString};
use std::ptr::{self, NonNull};
use std::sync::Mutex;
use zluda_dark_api::{CUmoduleContent, FatbinFileKind};

const EMPTY_MODULE: &'static str = include_str!("empty_module.ptx");

pub(crate) type Module = LiveCheck<ModuleData>;

impl ZludaObject for ModuleData {
    #[cfg(target_pointer_width = "64")]
    const LIVENESS_COOKIE: usize = 0xe522cee57bd3cd26;
    #[cfg(target_pointer_width = "32")]
    const LIVENESS_COOKIE: usize = 0x5f39cc5b;
    const LIVENESS_FAIL: CUresult = CUresult::CUDA_ERROR_INVALID_HANDLE;

    fn drop_with_result(&mut self, by_owner: bool) -> Result<(), CUresult> {
        let deregistration_err = if !by_owner {
            if let Some(ctx) = self.owner {
                let ctx = unsafe { LiveCheck::as_result(ctx.as_ptr())? };
                ctx.with_inner_mut(|ctx_mutable| {
                    ctx_mutable
                        .modules
                        .remove(&unsafe { LiveCheck::from_raw(self) });
                })?;
            }
            Ok(())
        } else {
            Ok(())
        };
        // Crashes HIP in 5.6 and 5.7.1
        //deregistration_err.and(unsafe { hipModuleUnload(self.base) }.into_cuda().into())
        deregistration_err
    }
}

pub(crate) struct ModuleData {
    // If module is part of a library, then there's no owning context
    pub(crate) owner: Option<NonNull<Context>>,
    pub(crate) base: hipModule_t,
    functions: Mutex<FxHashMap<CString, Box<function::Function>>>,
    sm_version: u32,
    device_version: u32,
    hipfix_max_group_sizes: FxHashMap<CString, (u32, u32)>,
    compilation_mode: CompilationMode,
}

impl ModuleData {
    pub(crate) unsafe fn alloc(self) -> *mut Module {
        Box::into_raw(Box::new(Module::new(self)))
    }
}

pub(crate) unsafe fn load(module: *mut *mut Module, fname: *const i8) -> Result<(), CUresult> {
    if fname == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    load_impl(module, CUmoduleContent::File(fname))
}

pub(crate) unsafe fn load_data(
    module: *mut *mut Module,
    image: *const ::std::os::raw::c_void,
) -> Result<(), CUresult> {
    if image == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    load_impl(
        module,
        CUmoduleContent::from_ptr(image.cast()).map_err(|_| CUresult::CUDA_ERROR_INVALID_VALUE)?,
    )
}

pub(crate) unsafe fn load_impl(
    output: *mut *mut Module,
    input: CUmoduleContent,
) -> Result<(), CUresult> {
    if output == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    context::with_current(|ctx| {
        let device = ctx.device;
        let device = GLOBAL_STATE.get()?.device(device)?;
        let isa = &device.comgr_isa;
        let owner = LiveCheck::from_ref(ctx);
        let module = ModuleData::alloc(load_data_any(
            Some(owner),
            device.compilation_mode,
            isa,
            input,
        )?);
        ctx.with_inner_mut(|ctx_mutable| {
            ctx_mutable.modules.insert(module);
        })?;
        *output = module;
        Ok(())
    })?
}

unsafe fn link_build_or_load_cuda_module(
    global_state: &super::GlobalState,
    compilation_mode: CompilationMode,
    isa: &CStr,
    input: CUmoduleContent,
) -> Result<Cow<'static, [u8]>, CUresult> {
    match input {
        CUmoduleContent::Elf(ptr) => Ok(Cow::Borrowed(hip_common::elf::as_slice(ptr))),
        CUmoduleContent::Archive(..) => return Err(CUresult::CUDA_ERROR_NOT_SUPPORTED),
        CUmoduleContent::RawText(ptr) => {
            let ptx = CStr::from_ptr(ptr.cast())
                .to_str()
                .map_err(|_| CUresult::CUDA_ERROR_INVALID_VALUE)?;
            link_build_zluda_module(global_state, compilation_mode, isa, &[Cow::Borrowed(ptx)])
                .map(Cow::Owned)
        }
        CUmoduleContent::File(file) => {
            let name = CStr::from_ptr(file)
                .to_str()
                .map_err(|_| CUresult::CUDA_ERROR_INVALID_VALUE)?;
            let ptx =
                std::fs::read_to_string(name).map_err(|_| CUresult::CUDA_ERROR_INVALID_VALUE)?;
            link_build_zluda_module(global_state, compilation_mode, isa, &[Cow::Owned(ptx)])
                .map(Cow::Owned)
        }
        CUmoduleContent::Fatbin(files) => match files {
            zluda_dark_api::CudaFatbin::Version1(module) => {
                link_build_or_load_fatbin_module(global_state, compilation_mode, isa, module)
                    .map(Cow::Owned)
            }
            zluda_dark_api::CudaFatbin::Version2 {
                post_link,
                pre_link,
            } => {
                if let Ok(binary) =
                    link_build_or_load_fatbin_module(global_state, compilation_mode, isa, post_link)
                {
                    return Ok(Cow::Owned(binary));
                }
                let ptx_files = pre_link
                    .iter()
                    .map(|module| {
                        let module = unsafe { module.get() }
                            .map_err(|_| CUresult::CUDA_ERROR_NOT_SUPPORTED)?;
                        match module {
                            zluda_dark_api::FatbinModule::Elf(_) => {
                                return Err(CUresult::CUDA_ERROR_NOT_SUPPORTED);
                            }
                            zluda_dark_api::FatbinModule::Files(files) => {
                                let ptx_files = extract_ptx(files);
                                if ptx_files.is_empty() {
                                    return Err(CUresult::CUDA_ERROR_NOT_SUPPORTED);
                                }
                                Ok(ptx_files.into_iter().next().unwrap().0)
                            }
                        }
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                link_build_zluda_module(global_state, compilation_mode, isa, &*ptx_files)
                    .map(Cow::Owned)
            }
        },
    }
}

fn link_build_or_load_fatbin_module(
    global_state: &super::GlobalState,
    compilation_mode: CompilationMode,
    isa: &CStr,
    module: zluda_dark_api::FatbinModuleHandle,
) -> Result<Vec<u8>, CUresult> {
    let module = unsafe { module.get() }.map_err(|_| CUresult::CUDA_ERROR_NOT_SUPPORTED)?;
    match module {
        zluda_dark_api::FatbinModule::Elf(_) => {
            return Err(CUresult::CUDA_ERROR_NOT_SUPPORTED);
        }
        zluda_dark_api::FatbinModule::Files(files) => {
            let ptx_files = extract_ptx(files);
            for (ptx, _) in ptx_files {
                if let Ok(binary) =
                    link_build_zluda_module(global_state, compilation_mode, isa, &[ptx])
                {
                    return Ok(binary);
                }
            }
            Err(CUresult::CUDA_ERROR_NOT_SUPPORTED)
        }
    }
}

fn extract_ptx(files: zluda_dark_api::FatbinModuleFiles) -> Vec<(Cow<'static, str>, u32)> {
    let mut ptx_files = files
        .filter_map(|file| {
            file.ok()
                .map(|file| {
                    if file.kind == FatbinFileKind::Ptx {
                        unsafe { file.get_or_decompress() }
                            .ok()
                            .map(|f| {
                                // TODO: implement support for envreg
                                // %envreg is currently used by global grid sync in PETSc on never CUDA architectures:
                                //  auto g = cooperative_groups::this_grid();
                                //  g.sync();
                                if memchr::memmem::find(&*f, b"%envreg").is_some() {
                                    return None;
                                }
                                let text = match f {
                                    Cow::Borrowed(slice) => {
                                        Cow::Borrowed(std::str::from_utf8(slice).ok()?)
                                    }
                                    Cow::Owned(vec) => Cow::Owned(String::from_utf8(vec).ok()?),
                                };
                                Some((text, file.sm_version))
                            })
                            .flatten()
                    } else {
                        None
                    }
                })
                .flatten()
        })
        .collect::<Vec<_>>();
    ptx_files.sort_unstable_by_key(|(_, sm_version)| cmp::Reverse(*sm_version));
    ptx_files
}

pub(crate) unsafe fn load_data_any(
    owner: Option<NonNull<Context>>,
    compilation_mode: CompilationMode,
    isa: &CStr,
    input: CUmoduleContent,
) -> Result<ModuleData, CUresult> {
    let global_state = GLOBAL_STATE.get()?;
    let gpu_module = link_build_or_load_cuda_module(global_state, compilation_mode, isa, input)?;
    let (hipfix_max_group_sizes, sm_version) = load_kernel_metadata(&*gpu_module)?;
    let mut hip_module = ptr::null_mut();
    hip_call_cuda! { hipModuleLoadData(&mut hip_module, gpu_module.as_ptr() as _) };
    let device_version = device::COMPUTE_CAPABILITY_MAJOR * 10 + device::COMPUTE_CAPABILITY_MINOR;
    Ok(ModuleData {
        compilation_mode,
        base: hip_module,
        owner,
        device_version,
        sm_version,
        hipfix_max_group_sizes,
        functions: Mutex::new(FxHashMap::default()),
    })
}

fn load_kernel_metadata(
    gpu_module: &[u8],
) -> Result<(FxHashMap<CString, (u32, u32)>, u32), CUresult> {
    let zluda_rt_section = hip_common::kernel_metadata::get_section(
        hip_common::kernel_metadata::zluda::SECTION_STR,
        gpu_module,
    )
    .ok_or(CUresult::CUDA_ERROR_UNKNOWN)?;
    let mut hipfix_max_group_sizes = FxHashMap::default();
    let sm_version =
        hip_common::kernel_metadata::zluda::read(zluda_rt_section, |name, mut min, mut max| {
            if min == 0 && max == 0 {
                return;
            }
            if min == 0 {
                min = 1;
            }
            if max == 0 {
                max = u32::MAX;
            }
            if let Ok(name) = CString::new(name) {
                hipfix_max_group_sizes.insert(name, (min, max));
            }
        })
        .map_err(|_| CUresult::CUDA_ERROR_UNKNOWN)?;
    Ok((hipfix_max_group_sizes, sm_version))
}

pub(crate) fn link_build_zluda_module(
    global_state: &super::GlobalState,
    compilation_mode: CompilationMode,
    isa: &CStr,
    ptx_text: &[Cow<'_, str>],
) -> Result<Vec<u8>, CUresult> {
    if ptx_text.is_empty() {
        return Err(CUresult::CUDA_ERROR_UNKNOWN);
    }
    if let Some(ref cache) = global_state.kernel_cache {
        if let Some(binary) =
            cache.try_load_program(&global_state.comgr_version, isa, ptx_text, compilation_mode)
        {
            return Ok(binary);
        }
    }
    // Older CUDA applications have no notion of lazy loading
    // and will eager load everything even if the module is unused.
    // For this reason we fallback to empty module since that has potential
    // to enable a few applications (but only in release mode)
    let asts = ptx_text
        .iter()
        .map(|ptx_mod| {
            let mut module = ptx::ModuleParser::parse_checked(&*ptx_mod);
            if !cfg!(debug_assertions) {
                module = module.or_else(|_| ptx::ModuleParser::parse_checked(EMPTY_MODULE))
            }
            module
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| CUresult::CUDA_ERROR_INVALID_PTX)?;
    let mut llvm_module = ptx::to_llvm_module(compilation_mode, asts);
    if !cfg!(debug_assertions) {
        llvm_module = llvm_module.or_else(|_| {
            ptx::to_llvm_module(
                compilation_mode,
                vec![ptx::ModuleParser::parse_checked(EMPTY_MODULE)
                    .map_err(|_| ptx::TranslateError::Todo)?],
            )
        });
    }
    let llvm_module = llvm_module.map_err(|_| CUresult::CUDA_ERROR_INVALID_PTX)?;
    let binary = global_state
        .comgr
        .compile(
            compilation_mode,
            isa,
            ptx::Module::get_bitcode_multi(std::iter::once(&llvm_module)).into_iter(),
            &llvm_module.metadata.to_elf_section(),
        )
        .map_err(comgr_error_to_cuda)?;
    if let Some(ref cache) = global_state.kernel_cache {
        cache.save_program(
            &global_state.comgr_version,
            isa,
            ptx_text,
            compilation_mode,
            &binary,
        );
    }
    Ok(binary)
}

pub(crate) unsafe fn unload(hmod: *mut Module) -> Result<(), CUresult> {
    if hmod == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    let module = LiveCheck::as_result(hmod)?;
    if module.owner.is_none() {
        return Err(CUresult::CUDA_ERROR_NOT_PERMITTED);
    }
    LiveCheck::drop_box_with_result(hmod, false)
}

pub(crate) unsafe fn get_function(
    hfunc: *mut *mut function::Function,
    hmod: *mut Module,
    name: *const i8,
) -> Result<(), CUresult> {
    if hfunc == ptr::null_mut() || hmod == ptr::null_mut() || name == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    let module = LiveCheck::as_result(hmod)?;
    let name = CStr::from_ptr(name).to_owned();
    let mut functions = module
        .functions
        .lock()
        .map_err(|_| CUresult::CUDA_ERROR_UNKNOWN)?;
    let function = match functions.entry(name.to_owned()) {
        hash_map::Entry::Occupied(entry) => {
            let function: &function::Function = &*entry.get();
            function as *const function::Function as *mut _
        }
        hash_map::Entry::Vacant(entry) => {
            let mut hip_func = ptr::null_mut();
            hip_call_cuda!(hipModuleGetFunction(
                &mut hip_func,
                module.base,
                name.as_ptr() as _
            ));
            let function: &function::Function =
                &*entry.insert(Box::new(LiveCheck::new(FunctionData {
                    base: hip_func,
                    binary_version: module.device_version,
                    ptx_version: module.sm_version,
                    group_size: module.hipfix_max_group_sizes.get(&name).copied(),
                    compilation_mode: module.compilation_mode,
                })));
            function as *const function::Function as *mut _
        }
    };
    *hfunc = function;
    Ok(())
}

pub(crate) unsafe fn get_global(
    dptr: *mut hipDeviceptr_t,
    bytes: *mut usize,
    hmod: *mut Module,
    name: *const i8,
) -> Result<(), CUresult> {
    if (dptr == ptr::null_mut() && bytes == ptr::null_mut()) || name == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    if hmod == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_HANDLE);
    }
    let hip_module = LiveCheck::as_result(hmod)?.base;
    hip_call_cuda!(hipfix::module_get_global(dptr, bytes, hip_module, name));
    Ok(())
}

pub(crate) unsafe fn get_tex_ref(
    tex_ref: *mut *mut textureReference,
    hmod: *mut Module,
    name: *const i8,
) -> Result<(), CUresult> {
    if tex_ref == ptr::null_mut() || hmod == ptr::null_mut() || name == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_HANDLE);
    }
    let hip_module = LiveCheck::as_result(hmod)?.base;
    hip_call_cuda!(hipModuleGetTexRef(tex_ref, hip_module, name));
    hip_call_cuda!(hipTexRefSetFormat(
        *tex_ref,
        hipArray_Format::HIP_AD_FORMAT_FLOAT,
        1
    ));
    Ok(())
}

const HIP_TRSF_READ_AS_INTEGER: u32 = 1;

pub(crate) unsafe fn get_surf_ref(
    texref: *mut *mut textureReference,
    hmod: *mut Module,
    name: *const i8,
) -> Result<(), CUresult> {
    get_tex_ref(texref, hmod, name)?;
    hip_call_cuda!(hipTexRefSetFlags(*texref, HIP_TRSF_READ_AS_INTEGER));
    Ok(())
}

pub(crate) unsafe fn get_loading_mode(result: *mut CUmoduleLoadingMode) -> CUresult {
    if result == ptr::null_mut() {
        CUresult::CUDA_ERROR_INVALID_VALUE
    } else {
        let mode = if matches!(std::env::var("CUDA_MODULE_LOADING").as_deref(), Ok("EAGER")) {
            CUmoduleLoadingMode::CU_MODULE_EAGER_LOADING
        } else {
            CUmoduleLoadingMode::CU_MODULE_LAZY_LOADING
        };
        *result = mode;
        CUresult::CUDA_SUCCESS
    }
}
