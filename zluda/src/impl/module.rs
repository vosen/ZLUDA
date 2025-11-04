use super::driver;
use cuda_types::{cuda::*, dark_api::FatbinFileHeader};
use hip_runtime_sys::*;
use std::{borrow::Cow, ffi::CStr, mem};
use zluda_common::{CodeLibraryRef, CodeModuleRef, ZludaObject};

pub(crate) struct Module {
    pub(crate) base: hipModule_t,
}

impl ZludaObject for Module {
    const COOKIE: usize = 0xe9138bd040487d4a;

    type Error = CUerror;
    type CudaHandle = CUmodule;

    fn drop_checked(&mut self) -> CUresult {
        unsafe { hipModuleUnload(self.base) }?;
        Ok(())
    }
}

static EMPTY_PTX: &str = ".version 6.5
.target sm_30
.address_size 64";

// get_ptx takes an `image` that can be anything we support and returns a
// String containing a ptx extracted from `image`.
fn get_ptx<'a>(image: CodeLibraryRef<'a>) -> Result<Cow<'a, str>, CUerror> {
    let mut ptx_modules = Vec::new();
    unsafe {
        CodeLibraryRef::iterate_modules(image, |_, module| match module {
            Ok(CodeModuleRef::Text(ptx)) => {
                ptx_modules.push(Cow::<'a, _>::Borrowed(ptx));
            }
            Ok(CodeModuleRef::<'a>::File(file)) => {
                if file.header.kind != FatbinFileHeader::HEADER_KIND_PTX {
                    return;
                }
                if let Ok(text) = file.get_or_decompress_content() {
                    if let Some(text) = cow_bytes_to_str(text) {
                        ptx_modules.push(text);
                    }
                }
            }
            _ => {}
        })
    };
    // TODO: instead of getting first PTX module, try and get the best match
    ptx_modules
        .into_iter()
        .next()
        .ok_or(CUerror::NO_BINARY_FOR_GPU)
}

fn cow_bytes_to_str<'a>(data: Cow<'a, [u8]>) -> Option<Cow<'a, str>> {
    match data {
        Cow::Borrowed(bytes) => std::str::from_utf8(bytes).map(Cow::Borrowed).ok(),
        Cow::Owned(bytes) => String::from_utf8(bytes).map(Cow::Owned).ok(),
    }
}

pub(crate) fn load_hip_module(library: CodeLibraryRef) -> Result<hipModule_t, CUerror> {
    let global_state = driver::global_state()?;
    let maybe_ptx = get_ptx(library);
    let text = if cfg!(debug_assertions) {
        maybe_ptx?
    } else {
        maybe_ptx.unwrap_or_else(|_| Cow::Borrowed(EMPTY_PTX))
    };
    // TODO: get this information on initialization
    let hip_properties = get_hip_properties()?;
    let gcn_arch = get_gcn_arch(&hip_properties)?;
    let gfx_version = match gcn_arch.strip_prefix("gfx").unwrap().parse::<u32>() {
        Ok(v) => v,
        Err(_) => return Err(CUerror::UNKNOWN),
    };
    let attributes = ExtraCacheAttributes {
        clock_rate: hip_properties.clockRate as u32,
        is_debug: cfg!(debug_assertions),
    };
    let mut cache_with_key = global_state.cache_path.as_ref().and_then(|p| {
        let cache = zluda_cache::ModuleCache::open(p)?;
        let key = get_cache_key(global_state, gcn_arch, &text, &attributes)?;
        Some((cache, key))
    });
    let cached_binary = load_cached_binary(&mut cache_with_key);
    let elf_module = cached_binary.ok_or(CUerror::UNKNOWN).or_else(|_| {
        compile_from_ptx_and_cache(
            &global_state.comgr,
            gcn_arch,
            gfx_version,
            attributes,
            &text,
            &mut cache_with_key,
        )
    })?;
    let mut hip_module = unsafe { mem::zeroed() };
    unsafe { hipModuleLoadData(&mut hip_module, elf_module.as_ptr().cast()) }?;
    Ok(hip_module)
}

#[derive(serde::Serialize)]
struct ExtraCacheAttributes {
    is_debug: bool,
    clock_rate: u32,
}

fn get_hip_properties<'a>() -> Result<hipDeviceProp_tR0600, CUerror> {
    let hip_dev = super::context::get_current_device()?;
    let mut props = unsafe { mem::zeroed() };
    unsafe { hipGetDevicePropertiesR0600(&mut props, hip_dev) }?;
    Ok(props)
}

fn get_gcn_arch<'a>(props: &'a hipDeviceProp_tR0600) -> Result<&'a str, CUerror> {
    let gcn_arch = unsafe { CStr::from_ptr(props.gcnArchName.as_ptr()) };
    gcn_arch.to_str().map_err(|_| CUerror::UNKNOWN)
}

fn get_cache_key<'a, 'b>(
    global_state: &'static driver::GlobalState,
    isa: &'a str,
    text: &str,
    attributes: &impl serde::Serialize,
) -> Option<zluda_cache::ModuleKey<'a>> {
    // Serialization here is deterministic. When marking a type with
    // #[derive(serde::Serialize)] the derived implementation will just write
    // fields in the order of their declaration. It's not explictly guaranteed
    // by serde, but it is the only sensible thing to do, so I feel safe
    // to rely on it
    let serialized_attributes = serde_json::to_string(attributes).ok()?;
    Some(zluda_cache::ModuleKey {
        hash: blake3::hash(text.as_bytes()).to_hex(),
        compiler_version: &*global_state.comgr_clang_version,
        zluda_version: env!("VERGEN_GIT_SHA"),
        device: isa,
        backend_key: serialized_attributes,
        last_access: zluda_cache::ModuleCache::time_now(),
    })
}

fn load_cached_binary(
    cache_with_key: &mut Option<(zluda_cache::ModuleCache, zluda_cache::ModuleKey)>,
) -> Option<Vec<u8>> {
    cache_with_key
        .as_mut()
        .and_then(|(c, key)| c.get_module_binary(key))
}

fn compile_from_ptx_and_cache(
    comgr: &comgr::Comgr,
    gcn_arch: &str,
    gfx_version: u32,
    attributes: ExtraCacheAttributes,
    text: &str,
    cache_with_key: &mut Option<(zluda_cache::ModuleCache, zluda_cache::ModuleKey)>,
) -> Result<Vec<u8>, CUerror> {
    let ast = if cfg!(debug_assertions) {
        ptx_parser::parse_module_checked(text).map_err(|_| CUerror::NO_BINARY_FOR_GPU)?
    } else {
        ptx_parser::parse_module_unchecked(text)
    };
    let llvm_module = ptx::to_llvm_module(
        ast,
        ptx::Attributes {
            clock_rate: attributes.clock_rate,
            gfx_version,
        },
        |_| {},
    )
    .map_err(|_| CUerror::UNKNOWN)?;
    let elf_module = comgr::compile_bitcode(
        comgr,
        gcn_arch,
        &*llvm_module.llvm_ir.write_bitcode_to_memory(),
        llvm_module.linked_bitcode(),
        &*llvm_module.attributes_ir.write_bitcode_to_memory(),
        None,
    )
    .map_err(|_| CUerror::UNKNOWN)?;
    if let Some((cache, key)) = cache_with_key {
        key.last_access = zluda_cache::ModuleCache::time_now();
        cache.insert_module(key, &elf_module);
    }
    Ok(elf_module)
}

pub(crate) fn load_data(module: &mut CUmodule, image: &std::ffi::c_void) -> CUresult {
    let library =
        unsafe { CodeLibraryRef::try_load(image) }.map_err(|_| CUerror::NO_BINARY_FOR_GPU)?;
    let hip_module = load_hip_module(library)?;
    *module = Module { base: hip_module }.wrap();
    Ok(())
}

pub(crate) fn unload(hmod: CUmodule) -> CUresult {
    zluda_common::drop_checked::<Module>(hmod)
}

pub(crate) fn get_function(
    hfunc: &mut hipFunction_t,
    hmod: &Module,
    name: *const ::core::ffi::c_char,
) -> hipError_t {
    unsafe { hipModuleGetFunction(hfunc, hmod.base, name) }
}

pub(crate) fn get_global_v2(
    dptr: *mut hipDeviceptr_t,
    bytes: *mut usize,
    hmod: &Module,
    name: *const ::core::ffi::c_char,
) -> hipError_t {
    unsafe { hipModuleGetGlobal(dptr, bytes, hmod.base, name) }
}

pub(crate) fn get_loading_mode(mode: &mut cuda_types::cuda::CUmoduleLoadingMode) -> CUresult {
    *mode = cuda_types::cuda::CUmoduleLoadingMode::CU_MODULE_LAZY_LOADING;
    Ok(())
}
