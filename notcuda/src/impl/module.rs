use std::{ffi::c_void, ffi::CStr, mem, os::raw::c_char, ptr, slice, sync::Mutex};

use super::{transmute_lifetime, CUresult};
use ptx;

use super::context;

pub type Module = Mutex<ModuleData>;

pub struct ModuleData {
    base: l0::Module,
}

pub enum ModuleCompileError<'a> {
    Parse(
        Vec<ptx::ast::PtxError>,
        Option<ptx::ParseError<usize, ptx::Token<'a>, ptx::ast::PtxError>>,
    ),
    Compile(ptx::TranslateError),
    L0(l0::sys::ze_result_t),
    CUDA(CUresult),
}

impl<'a> ModuleCompileError<'a> {
    pub fn get_build_log(&self) {}
}

impl<'a> From<ptx::TranslateError> for ModuleCompileError<'a> {
    fn from(err: ptx::TranslateError) -> Self {
        ModuleCompileError::Compile(err)
    }
}

impl<'a> From<l0::sys::ze_result_t> for ModuleCompileError<'a> {
    fn from(err: l0::sys::ze_result_t) -> Self {
        ModuleCompileError::L0(err)
    }
}

impl<'a> From<CUresult> for ModuleCompileError<'a> {
    fn from(err: CUresult) -> Self {
        ModuleCompileError::CUDA(err)
    }
}

impl ModuleData {
    pub fn compile_spirv<'a>(ptx_text: &'a str) -> Result<Module, ModuleCompileError<'a>> {
        let mut errors = Vec::new();
        let ast = ptx::ModuleParser::new().parse(&mut errors, ptx_text);
        let ast = match ast {
            Err(e) => return Err(ModuleCompileError::Parse(errors, Some(e))),
            Ok(_) if errors.len() > 0 => return Err(ModuleCompileError::Parse(errors, None)),
            Ok(ast) => ast,
        };
        let spirv = ptx::to_spirv(ast)?;
        let byte_il = unsafe {
            slice::from_raw_parts::<u8>(
                spirv.as_ptr() as *const _,
                spirv.len() * mem::size_of::<u32>(),
            )
        };
        let module = super::device::with_current_exclusive(|dev| {
            l0::Module::new_spirv(&mut dev.l0_context, &dev.base, byte_il, None)
        });
        match module {
            Ok(Ok(module)) => Ok(Mutex::new(Self { base: module })),
            Ok(Err(err)) => Err(ModuleCompileError::from(err)),
            Err(err) => Err(ModuleCompileError::from(err)),
        }
    }
}

pub struct Function {
    base: l0::Kernel<'static>,
}

pub fn get_function(
    hfunc: *mut *mut Function,
    hmod: *mut Module,
    name: *const c_char,
) -> Result<(), CUresult> {
    if hfunc == ptr::null_mut() || hmod == ptr::null_mut() || name == ptr::null() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    let name = unsafe { CStr::from_ptr(name) };
    let kernel = unsafe { &*hmod }
        .try_lock()
        .map(|module| l0::Kernel::new_resident(unsafe { transmute_lifetime(&module.base) }, name))
        .map_err(|_| CUresult::CUDA_ERROR_ILLEGAL_STATE)??;
    unsafe { *hfunc = Box::into_raw(Box::new(Function { base: kernel })) };
    Ok(())
}
