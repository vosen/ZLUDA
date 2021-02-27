#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[path = "amd_comgr.rs"]
pub mod sys;

use hip_common::CompilationMode;
use itertools::Either;
use std::{
    borrow::Borrow,
    ffi::{CStr, CString},
    iter, mem, ptr,
    rc::Rc,
    sync::atomic::{AtomicU64, Ordering},
};
use sys::LibComgr;

#[cfg(windows)]
static CODE_OBJECT_VERSION_FLAG_COMGR: &'static [u8] = b"code_object_v4\0"; // Code Object v5 is broken as of Adrenaline 23.11.1
#[cfg(windows)]
static CODE_OBJECT_VERSION_FLAG_CLANG: &'static [u8] = b"-mcode-object-version=4\0"; // Code Object v5 is broken as of Adrenaline 23.11.1
#[cfg(not(windows))]
static CODE_OBJECT_VERSION_FLAG_COMGR: &'static [u8] = b"code_object_v5\0";
#[cfg(not(windows))]
static CODE_OBJECT_VERSION_FLAG_CLANG: &'static [u8] = b"-mcode-object-version=5\0";

macro_rules! call {
    ($expr:expr) => {
        #[allow(unused_unsafe)]
        {
            unsafe {
                let result = $expr;
                if result != sys::amd_comgr_status_t::AMD_COMGR_STATUS_SUCCESS {
                    return Err(result);
                }
            }
        }
    };
}

pub type Result<T> = std::result::Result<T, sys::amd_comgr_status_t>;

pub struct Comgr(LibComgr, AtomicU64);

static WAVE32_MODULE: &'static [u8] = include_bytes!("wave32.ll");
static WAVE32_ON_WAVE64_MODULE: &'static [u8] = include_bytes!("wave32_on_wave64.ll");
static DOUBLE_WAVE32_ON_WAVE64_MODULE: &'static [u8] = include_bytes!("double_wave32_on_wave64.ll");

#[cfg(windows)]
static OS_MODULE: &'static [u8] = include_bytes!("windows.ll");
#[cfg(not(windows))]
static OS_MODULE: &'static [u8] = include_bytes!("linux.ll");

impl Comgr {
    pub fn find_and_load() -> Result<Self> {
        match unsafe { Self::load_library() } {
            Ok(libcomgr) => Ok(Self(libcomgr, AtomicU64::new(1))),
            Err(_) => Err(sys::amd_comgr_status_t::AMD_COMGR_STATUS_ERROR),
        }
    }

    #[cfg(windows)]
    unsafe fn load_library() -> std::result::Result<LibComgr, libloading::Error> {
        LibComgr::new("amd_comgr.dll")
    }

    #[cfg(not(windows))]
    unsafe fn load_library() -> std::result::Result<LibComgr, libloading::Error> {
        LibComgr::new("libamd_comgr.so.2")
            .or_else(|_| LibComgr::new("/opt/rocm/lib/libamd_comgr.so.2"))
    }

    fn get(&self) -> &LibComgr {
        &self.0
    }

    pub fn compile<'a>(
        &self,
        compilation_mode: CompilationMode,
        isa: &'a CStr,
        input_bitcode: impl Iterator<Item = (impl AsRef<[u8]>, impl AsRef<CStr>)>,
        linker_module: &[u8],
    ) -> Result<Vec<u8>> {
        let bitcode = self.link_bitcode_impl(compilation_mode, isa, input_bitcode)?;
        let relocatable = self.build_relocatable_impl(compilation_mode, isa, &bitcode)?;
        if !linker_module.is_empty() {
            let source = self.assemble_source(isa, linker_module)?;
            self.link_relocatable_impl(
                isa,
                IntoIterator::into_iter([
                    &relocatable.get_data(
                        sys::amd_comgr_data_kind_t::AMD_COMGR_DATA_KIND_RELOCATABLE,
                        0,
                    )?,
                    &source,
                ]),
            )
        } else {
            self.link_relocatable_impl(
                isa,
                iter::once(&relocatable.get_data(
                    sys::amd_comgr_data_kind_t::AMD_COMGR_DATA_KIND_RELOCATABLE,
                    0,
                )?),
            )
        }
    }

    pub fn link_bitcode<'this, 'a>(
        &'this self,
        compilation_mode: CompilationMode,
        isa: &'a CStr,
        input_bitcode: impl Iterator<Item = (impl AsRef<[u8]>, &'a CStr)>,
    ) -> Result<Bitcode<'this>> {
        let data_set_bitcode = self.link_bitcode_impl(compilation_mode, isa, input_bitcode)?;
        Ok(Bitcode(data_set_bitcode))
    }

    pub fn bitcode_to_relocatable<'a>(
        self: &Rc<Self>,
        compilation_mode: CompilationMode,
        isa: &'a CStr,
        bc: &Bitcode,
    ) -> Result<Relocatable> {
        let data_set_relocatable = self.build_relocatable_impl(compilation_mode, isa, &bc.0)?;
        Ok(Relocatable::from_data(data_set_relocatable.get_data_rc(
            self.clone(),
            sys::amd_comgr_data_kind_t::AMD_COMGR_DATA_KIND_RELOCATABLE,
            0,
        )?)?)
    }

    pub fn build_relocatable<'a>(
        self: &Rc<Self>,
        compilation_mode: CompilationMode,
        isa: &'a CStr,
        input_bitcode: impl Iterator<Item = (impl AsRef<[u8]>, &'a CStr)>,
    ) -> Result<Relocatable> {
        let bitcode = self.link_bitcode_impl(compilation_mode, isa, input_bitcode)?;
        let data_set_relocatable = self.build_relocatable_impl(compilation_mode, isa, &bitcode)?;
        Ok(Relocatable::from_data(data_set_relocatable.get_data_rc(
            self.clone(),
            sys::amd_comgr_data_kind_t::AMD_COMGR_DATA_KIND_RELOCATABLE,
            0,
        )?)?)
    }

    pub fn link_relocatable<'a>(
        self: &Rc<Self>,
        isa: &'a CStr,
        modules: impl Iterator<Item = &'a Relocatable>,
    ) -> Result<Vec<u8>> {
        self.link_relocatable_impl(isa, modules.map(|reloc| &reloc.0))
    }

    pub fn version(&self) -> Result<String> {
        let mut data_set = DataSet::new(self)?;
        let data = Data::new(
            self,
            sys::amd_comgr_data_kind_t::AMD_COMGR_DATA_KIND_SOURCE,
            b"__VERSION__",
            unsafe { CStr::from_bytes_with_nul_unchecked(b"version.h\0") },
        )?;
        data_set.add(&data)?;
        let result = self.do_action(
            sys::amd_comgr_action_kind_t::AMD_COMGR_ACTION_SOURCE_TO_PREPROCESSOR,
            &data_set,
            unsafe { CStr::from_bytes_with_nul_unchecked(b"\0") },
            iter::once(unsafe { CStr::from_bytes_with_nul_unchecked(b"-nogpuinc\0") }),
            Some(sys::amd_comgr_language_t::AMD_COMGR_LANGUAGE_HIP),
        )?;
        let result = result.get_data(sys::amd_comgr_data_kind_t::AMD_COMGR_DATA_KIND_SOURCE, 0)?;
        let result = result.get_data()?;
        let end_quote = result
            .iter()
            .copied()
            .rposition(|c| c as char == '"')
            .ok_or(sys::amd_comgr_status_t::AMD_COMGR_STATUS_ERROR)?;
        let start_quote = result[..end_quote]
            .iter()
            .copied()
            .rposition(|c| c as char == '"')
            .ok_or(sys::amd_comgr_status_t::AMD_COMGR_STATUS_ERROR)?;
        String::from_utf8(result[start_quote + 1..end_quote].to_vec())
            .map_err(|_| sys::amd_comgr_status_t::AMD_COMGR_STATUS_ERROR)
    }

    fn link_bitcode_impl<'this, 'a>(
        &'this self,
        compilation_mode: CompilationMode,
        isa: &'a CStr,
        input_bitcode: impl Iterator<Item = (impl AsRef<[u8]>, impl AsRef<CStr>)>,
    ) -> Result<DataSet<'this>> {
        let mut bitcode_modules = DataSet::new(self)?;
        for (bc, name) in input_bitcode {
            bitcode_modules.add(&Data::new(
                self,
                sys::amd_comgr_data_kind_t::AMD_COMGR_DATA_KIND_BC,
                bc.as_ref(),
                name.as_ref(),
            )?)?;
        }
        let wave_module_text = match compilation_mode {
            CompilationMode::Wave32 => WAVE32_MODULE,
            CompilationMode::Wave32OnWave64 => WAVE32_ON_WAVE64_MODULE,
            CompilationMode::DoubleWave32OnWave64 => DOUBLE_WAVE32_ON_WAVE64_MODULE,
        };
        let wave_module = Data::new(
            self,
            sys::amd_comgr_data_kind_t::AMD_COMGR_DATA_KIND_BC,
            wave_module_text,
            unsafe { CStr::from_bytes_with_nul_unchecked(b"wave.ll\0") },
        )?;
        bitcode_modules.add(&wave_module)?;
        let os_module = Data::new(
            self,
            sys::amd_comgr_data_kind_t::AMD_COMGR_DATA_KIND_BC,
            OS_MODULE,
            unsafe { CStr::from_bytes_with_nul_unchecked(b"os.ll\0") },
        )?;
        bitcode_modules.add(&os_module)?;
        let lib_options = unsafe {
            match compilation_mode {
                CompilationMode::Wave32 => Either::Left([CStr::from_bytes_with_nul_unchecked(
                    CODE_OBJECT_VERSION_FLAG_COMGR,
                )]),
                CompilationMode::Wave32OnWave64 | CompilationMode::DoubleWave32OnWave64 => {
                    Either::Right([
                        CStr::from_bytes_with_nul_unchecked(CODE_OBJECT_VERSION_FLAG_COMGR),
                        CStr::from_bytes_with_nul_unchecked(b"wavefrontsize64\0"),
                    ])
                }
            }
        };
        let device_linking_output: DataSet<'_> = self.do_action(
            sys::amd_comgr_action_kind_t::AMD_COMGR_ACTION_ADD_DEVICE_LIBRARIES,
            &bitcode_modules,
            isa,
            lib_options.into_iter(),
            Some(sys::amd_comgr_language_t::AMD_COMGR_LANGUAGE_OPENCL_2_0),
        )?;
        self.do_action(
            sys::amd_comgr_action_kind_t::AMD_COMGR_ACTION_LINK_BC_TO_BC,
            &device_linking_output,
            isa,
            unsafe {
                [CStr::from_bytes_with_nul_unchecked(
                    CODE_OBJECT_VERSION_FLAG_COMGR,
                )]
                .iter()
                .copied()
            },
            None,
        )
    }

    fn build_relocatable_impl<'this, 'a>(
        &'this self,
        compilation_mode: CompilationMode,
        isa: &'a CStr,
        bc_linking_output: &DataSet<'this>,
    ) -> Result<DataSet<'this>> {
        let debug_level = if cfg!(debug_assertions) {
            unsafe {
                [
                    CStr::from_bytes_with_nul_unchecked(b"-g\0"),
                    CStr::from_bytes_with_nul_unchecked(b"\0"),
                    CStr::from_bytes_with_nul_unchecked(b"\0"),
                ]
            }
        } else {
            unsafe {
                [
                    CStr::from_bytes_with_nul_unchecked(b"-g0\0"),
                    // TODO: tweak and measure impact
                    CStr::from_bytes_with_nul_unchecked(b"-mllvm\0"),
                    CStr::from_bytes_with_nul_unchecked(b"-inline-threshold=5000\0"),
                ]
            }
        };
        let compilation_mode = unsafe {
            if compilation_mode == CompilationMode::Wave32 {
                CStr::from_bytes_with_nul_unchecked(b"-mno-wavefrontsize64\0")
            } else {
                CStr::from_bytes_with_nul_unchecked(b"-mwavefrontsize64\0")
            }
        };
        let relocatable = self.do_action(
            sys::amd_comgr_action_kind_t::AMD_COMGR_ACTION_CODEGEN_BC_TO_RELOCATABLE,
            bc_linking_output,
            isa,
            unsafe {
                [
                    compilation_mode,
                    CStr::from_bytes_with_nul_unchecked(CODE_OBJECT_VERSION_FLAG_CLANG),
                    CStr::from_bytes_with_nul_unchecked(b"-O3\0"),
                    // TODO: measure more
                    // Slightly more efficient in Blender
                    CStr::from_bytes_with_nul_unchecked(b"-mcumode\0"),
                    CStr::from_bytes_with_nul_unchecked(b"-ffp-contract=off\0"),
                    CStr::from_bytes_with_nul_unchecked(b"-mllvm\0"),
                    CStr::from_bytes_with_nul_unchecked(b"-amdgpu-internalize-symbols\0"),
                    // TODO: This emits scratch_ instead of buffer_ instructions
                    // for stack spills&fills, measure impact
                    // CStr::from_bytes_with_nul_unchecked(b"-Xclang\0"),
                    // CStr::from_bytes_with_nul_unchecked(b"-target-feature\0"),
                    // CStr::from_bytes_with_nul_unchecked(b"-Xclang\0"),
                    // CStr::from_bytes_with_nul_unchecked(b"+enable-flat-scratch\0"),
                    // Useful for debugging miscompilations
                    // CStr::from_bytes_with_nul_unchecked(b"-mllvm\0"),
                    // CStr::from_bytes_with_nul_unchecked(b"-opt-bisect-limit=-1\0"),
                ]
            }
            .iter()
            .copied()
            .chain(debug_level.iter().copied()),
            None,
        )?;
        Ok(relocatable)
    }

    fn link_relocatable_impl<'this, 'a, C: Borrow<Comgr> + 'a>(
        &'this self,
        isa: &'a CStr,
        modules: impl Iterator<Item = &'a Data<C>>,
    ) -> Result<Vec<u8>> {
        let mut input = DataSet::new(self)?;
        for module in modules {
            input.add(module)?;
        }
        let executable_set: DataSet = self.do_action(
            sys::amd_comgr_action_kind_t::AMD_COMGR_ACTION_LINK_RELOCATABLE_TO_EXECUTABLE,
            &input,
            isa,
            unsafe {
                [
                    CStr::from_bytes_with_nul_unchecked(b"-Xlinker\0"),
                    CStr::from_bytes_with_nul_unchecked(b"--no-undefined\0"),
                ]
            }
            .iter()
            .copied(),
            None,
        )?;
        let executable_data = executable_set.get_data(
            sys::amd_comgr_data_kind_t::AMD_COMGR_DATA_KIND_EXECUTABLE,
            0,
        )?;
        executable_data.get_data()
    }

    fn assemble_source(&self, isa: &CStr, src: &[u8]) -> Result<Data<&Self>> {
        let data = Data::new(
            self,
            sys::amd_comgr_data_kind_t::AMD_COMGR_DATA_KIND_SOURCE,
            src,
            unsafe { CStr::from_bytes_with_nul_unchecked(b"input.s\0") },
        )?;
        let mut data_set = DataSet::new(self)?;
        data_set.add(&data)?;
        let assembled = self.do_action(
            sys::amd_comgr_action_kind_t::AMD_COMGR_ACTION_ASSEMBLE_SOURCE_TO_RELOCATABLE,
            &data_set,
            isa,
            iter::once(unsafe {
                CStr::from_bytes_with_nul_unchecked(CODE_OBJECT_VERSION_FLAG_CLANG)
            }),
            None,
        )?;
        assembled.get_data(
            sys::amd_comgr_data_kind_t::AMD_COMGR_DATA_KIND_RELOCATABLE,
            0,
        )
    }

    fn do_action<'a, 'cstr>(
        &'a self,
        kind: sys::amd_comgr_action_kind_t,
        input: &DataSet,
        isa: &CStr,
        options: impl Iterator<Item = &'cstr CStr>,
        language: Option<sys::amd_comgr_language_t>,
    ) -> Result<DataSet<'a>> {
        let output = DataSet::new(self)?;
        let action = ActionInfo::new(self, isa)?;
        if options.size_hint().1.unwrap() > 0 {
            action.set_options(options)?;
        }
        if let Some(lang) = language {
            action.set_language(lang)?;
        }
        action.execute(kind, &input, &output)?;
        Ok(output)
    }
}

pub struct Bitcode<'a>(DataSet<'a>);

impl<'a> Bitcode<'a> {
    pub fn get_data(&self) -> Result<Vec<u8>> {
        self.0
            .get_data(sys::amd_comgr_data_kind_t::AMD_COMGR_DATA_KIND_BC, 0)?
            .get_data()
    }
}

pub struct Relocatable(Data<Rc<Comgr>>);

impl Relocatable {
    fn from_data(data: Data<Rc<Comgr>>) -> Result<Self> {
        let suffix = data.1 .1.fetch_add(1, Ordering::Relaxed);
        let new_name = format!("reloc_{}.o\0", suffix);
        call!(data
            .1
            .get()
            .amd_comgr_set_data_name(data.0, new_name.as_ptr() as _));
        Ok(Self(data))
    }

    pub fn new(comgr: &Rc<Comgr>, vec: &[u8]) -> Result<Self> {
        let suffix = comgr.1.fetch_add(1, Ordering::Relaxed);
        let new_name = format!("reloc_{}.o", suffix);
        let data = Data::new(
            comgr.clone(),
            sys::amd_comgr_data_kind_t::AMD_COMGR_DATA_KIND_RELOCATABLE,
            vec,
            &CString::new(new_name).map_err(|_| sys::amd_comgr_status_t::AMD_COMGR_STATUS_ERROR)?,
        )?;
        Ok(Self(data))
    }

    pub fn get_data(&self) -> Result<Vec<u8>> {
        self.0.get_data()
    }
}

struct ActionInfo<'a>(sys::amd_comgr_action_info_t, &'a Comgr);

impl<'a> ActionInfo<'a> {
    fn new(comgr: &'a Comgr, isa: &'a CStr) -> Result<Self> {
        unsafe {
            let mut action_info = mem::zeroed();
            call!(comgr.get().amd_comgr_create_action_info(&mut action_info));
            call!(comgr
                .get()
                .amd_comgr_action_info_set_isa_name(action_info, isa.as_ptr() as _));
            Ok(ActionInfo(action_info, comgr))
        }
    }

    fn get(&self) -> sys::amd_comgr_action_info_t {
        self.0
    }

    fn set_options<'cstr>(&self, options: impl Iterator<Item = &'cstr CStr>) -> Result<()> {
        let mut options_c = options.map(CStr::as_ptr).collect::<Vec<_>>();
        call!(self.1.get().amd_comgr_action_info_set_option_list(
            self.get(),
            options_c.as_mut_ptr(),
            options_c.len()
        ));
        Ok(())
    }

    fn set_language(&self, lang: sys::amd_comgr_language_t) -> Result<()> {
        call!(self
            .1
            .get()
            .amd_comgr_action_info_set_language(self.get(), lang));
        Ok(())
    }

    fn execute(
        &self,
        kind: sys::amd_comgr_action_kind_t,
        input: &DataSet<'a>,
        output: &DataSet<'a>,
    ) -> Result<()> {
        call!(self
            .1
            .get()
            .amd_comgr_do_action(kind, self.get(), input.get(), output.get()));
        Ok(())
    }
}

#[allow(unused_must_use)]
impl<'a> Drop for ActionInfo<'a> {
    fn drop(&mut self) {
        unsafe { self.1.get().amd_comgr_destroy_action_info(self.get()) };
    }
}

struct DataSet<'a> {
    base: sys::amd_comgr_data_set_t,
    comgr: &'a Comgr,
}

impl<'a> DataSet<'a> {
    fn new(comgr: &'a Comgr) -> Result<DataSet<'a>> {
        unsafe {
            let mut base = mem::zeroed();
            call!(comgr.get().amd_comgr_create_data_set(&mut base));
            Ok(DataSet { base, comgr })
        }
    }

    fn add<C: Borrow<Comgr>>(&mut self, data: &Data<C>) -> Result<()> {
        call!(self
            .comgr
            .get()
            .amd_comgr_data_set_add(self.base, data.get()));
        Ok(())
    }

    fn get_data(&self, kind: sys::amd_comgr_data_kind_t, index: usize) -> Result<Data<&'a Comgr>> {
        let mut output = unsafe { mem::zeroed() };
        call!(self.comgr.get().amd_comgr_action_data_get_data(
            self.get(),
            kind,
            index,
            &mut output
        ));
        Ok(Data(output, self.comgr))
    }

    fn get_data_rc(
        &self,
        comgr: Rc<Comgr>,
        kind: sys::amd_comgr_data_kind_t,
        index: usize,
    ) -> Result<Data<Rc<Comgr>>> {
        assert!(std::ptr::eq::<Comgr>(self.comgr as *const _, &*comgr));
        let mut output = unsafe { mem::zeroed() };
        call!(self.comgr.get().amd_comgr_action_data_get_data(
            self.get(),
            kind,
            index,
            &mut output
        ));
        Ok(Data(output, comgr))
    }

    fn get(&self) -> sys::amd_comgr_data_set_t {
        self.base
    }
}

#[allow(unused_must_use)]
impl<'a> Drop for DataSet<'a> {
    fn drop(&mut self) {
        unsafe { self.comgr.get().amd_comgr_destroy_data_set(self.get()) };
    }
}

struct Data<C: Borrow<Comgr>>(sys::amd_comgr_data_t, C);

impl<C: Borrow<Comgr>> Data<C> {
    fn new(
        comgr: C,
        kind: sys::amd_comgr_data_kind_t,
        data: &[u8],
        name: &CStr,
    ) -> Result<Data<C>> {
        let mut comgr_data = unsafe { mem::zeroed() };
        call!(comgr
            .borrow()
            .get()
            .amd_comgr_create_data(kind, &mut comgr_data));
        call!(comgr
            .borrow()
            .get()
            .amd_comgr_set_data_name(comgr_data, name.as_ptr() as _));
        call!(comgr
            .borrow()
            .get()
            .amd_comgr_set_data(comgr_data, data.len(), data.as_ptr() as _));
        Ok(Self(comgr_data, comgr))
    }

    fn get(&self) -> sys::amd_comgr_data_t {
        self.0
    }

    fn get_data(&self) -> Result<Vec<u8>> {
        let mut size = 0;
        call!(self
            .1
            .borrow()
            .get()
            .amd_comgr_get_data(self.get(), &mut size, ptr::null_mut()));
        let mut output = vec![0u8; size];
        call!(self.1.borrow().get().amd_comgr_get_data(
            self.get(),
            &mut size,
            output.as_mut_ptr() as _
        ));
        Ok(output)
    }
}

impl<C: Borrow<Comgr> + Clone> Clone for Data<C> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
}

#[allow(unused_must_use)]
impl<C: Borrow<Comgr>> Drop for Data<C> {
    fn drop(&mut self) {
        unsafe { self.1.borrow().get().amd_comgr_release_data(self.get()) };
    }
}

#[cfg(test)]
mod tests {
    use crate::Comgr;

    #[test]
    fn version() {
        let comgr = Comgr::find_and_load().unwrap();
        let version = comgr.version().unwrap();
        assert!(version.contains("Clang"));
        assert!(!version.contains("\""));
        assert!(!version.contains("\n"));
    }
}
