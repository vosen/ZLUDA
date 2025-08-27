use amd_comgr_sys::*;
use std::{ffi::CStr, iter, mem, ptr};

macro_rules! call_dispatch_arg {
    (2, $arg:ident) => {
        $arg.comgr2()
    };
    (2, $arg:tt) => {
        #[allow(unused_braces)]
        $arg
    };
    (3, $arg:ident) => {
        $arg.comgr3()
    };
    (3, $arg:tt) => {
        #[allow(unused_braces)]
        $arg
    };
}

macro_rules! call_dispatch {
    ($src:expr => $fn_:ident( $($arg:tt),+ )) => {
        match $src {
            Comgr::V2(this) => unsafe { this. $fn_(
                $(
                    call_dispatch_arg!(2, $arg),
                )+
            ) }?,
            Comgr::V3(this) => unsafe { this. $fn_(
                $(
                    call_dispatch_arg!(3, $arg),
                )+
            ) }?,
        }
    };
}

macro_rules! comgr_owned {
    ($name:ident, $comgr_type:ident, $ctor:ident, $dtor:ident) => {
        struct $name<'a> {
            handle: u64,
            comgr: &'a Comgr,
        }

        impl<'a> $name<'a> {
            fn new(comgr: &'a Comgr) -> Result<Self, Error> {
                let handle = match comgr {
                    Comgr::V2(comgr) => {
                        let mut result = unsafe { mem::zeroed() };
                        unsafe { comgr.$ctor(&mut result)? };
                        result.handle
                    }
                    Comgr::V3(comgr) => {
                        let mut result = unsafe { mem::zeroed() };
                        unsafe { comgr.$ctor(&mut result)? };
                        result.handle
                    }
                };
                Ok(Self { handle, comgr })
            }

            fn comgr2(&self) -> amd_comgr_sys::comgr2::$comgr_type {
                amd_comgr_sys::comgr2::$comgr_type {
                    handle: self.handle,
                }
            }

            fn comgr3(&self) -> amd_comgr_sys::comgr3::$comgr_type {
                amd_comgr_sys::comgr3::$comgr_type {
                    handle: self.handle,
                }
            }
        }

        impl<'a> Drop for $name<'a> {
            fn drop(&mut self) {
                match self.comgr {
                    Comgr::V2(comgr) => {
                        unsafe {
                            comgr.$dtor(amd_comgr_sys::comgr2::$comgr_type {
                                handle: self.handle,
                            })
                        }
                        .ok();
                    }
                    Comgr::V3(comgr) => {
                        unsafe {
                            comgr.$dtor(amd_comgr_sys::comgr3::$comgr_type {
                                handle: self.handle,
                            })
                        }
                        .ok();
                    }
                }
            }
        }
    };
}

comgr_owned!(
    ActionInfo,
    amd_comgr_action_info_t,
    amd_comgr_create_action_info,
    amd_comgr_destroy_action_info
);

impl<'a> ActionInfo<'a> {
    fn set_isa_name(&self, isa: &str) -> Result<(), Error> {
        let mut full_isa = "amdgcn-amd-amdhsa--".to_string().into_bytes();
        full_isa.extend(isa.as_bytes());
        full_isa.push(0);
        call_dispatch!(self.comgr => amd_comgr_action_info_set_isa_name(self, { full_isa.as_ptr().cast() }));
        Ok(())
    }

    fn set_language(&self, language: Language) -> Result<(), Error> {
        call_dispatch!(self.comgr => amd_comgr_action_info_set_language(self, language));
        Ok(())
    }

    fn set_options<'b>(&self, options: impl Iterator<Item = &'b CStr>) -> Result<(), Error> {
        let options = options.map(|x| x.as_ptr()).collect::<Vec<_>>();
        call_dispatch!(self.comgr => amd_comgr_action_info_set_option_list(self, { options.as_ptr().cast_mut() }, { options.len() }));
        Ok(())
    }
}

comgr_owned!(
    DataSet,
    amd_comgr_data_set_t,
    amd_comgr_create_data_set,
    amd_comgr_destroy_data_set
);

impl<'a> DataSet<'a> {
    fn add(&self, data: &Data) -> Result<(), Error> {
        call_dispatch!(self.comgr => amd_comgr_data_set_add(self, data));
        Ok(())
    }

    pub fn from_data(comgr: &'a Comgr, data: impl Iterator<Item = Data>) -> Result<Self, Error> {
        let dataset = Self::new(comgr)?;
        for data in data {
            dataset.add(&data)?;
        }
        Ok(dataset)
    }

    fn get_data(&self, kind: DataKind, index: usize) -> Result<Data, Error> {
        let mut handle = 0u64;
        call_dispatch!(self.comgr => amd_comgr_action_data_get_data(self, kind, { index }, { std::ptr::from_mut(&mut handle).cast() }));
        Ok(Data(handle))
    }

    fn get_content(&self, comgr: &Comgr, kind: DataKind, index: usize) -> Result<Vec<u8>, Error> {
        self.get_data(kind, index)
            .map(|data| data.copy_content(comgr))?
    }
}

struct Data(u64);

impl Data {
    fn new(comgr: &Comgr, kind: DataKind, name: &CStr, content: &[u8]) -> Result<Self, Error> {
        let mut handle = 0u64;
        call_dispatch!(comgr => amd_comgr_create_data(kind, { std::ptr::from_mut(&mut handle).cast() }));
        let data = Data(handle);
        call_dispatch!(comgr => amd_comgr_set_data_name(data, { name.as_ptr() }));
        call_dispatch!(comgr => amd_comgr_set_data(data, { content.len() }, { content.as_ptr().cast() }));
        Ok(data)
    }

    fn comgr2(&self) -> comgr2::amd_comgr_data_t {
        comgr2::amd_comgr_data_s { handle: self.0 }
    }

    fn comgr3(&self) -> comgr3::amd_comgr_data_t {
        comgr3::amd_comgr_data_s { handle: self.0 }
    }

    fn copy_content(&self, comgr: &Comgr) -> Result<Vec<u8>, Error> {
        let mut size = unsafe { mem::zeroed() };
        call_dispatch!(comgr => amd_comgr_get_data(self, { &mut size }, { ptr::null_mut() }));
        let mut result: Vec<u8> = Vec::with_capacity(size);
        unsafe { result.set_len(size) };
        call_dispatch!(comgr => amd_comgr_get_data(self, { &mut size }, { result.as_mut_ptr().cast() }));
        Ok(result)
    }
}

#[derive(Clone, Copy, Debug)]
struct Symbol(u64);

impl Symbol {
    fn comgr2(&self) -> comgr2::amd_comgr_symbol_t {
        comgr2::amd_comgr_symbol_s { handle: self.0 }
    }

    fn comgr3(&self) -> comgr3::amd_comgr_symbol_t {
        comgr3::amd_comgr_symbol_s { handle: self.0 }
    }
}

pub fn compile_bitcode(
    comgr: &Comgr,
    gcn_arch: &str,
    main_buffer: &[u8],
    ptx_impl: &[u8],
    attributes_buffer: &[u8],
    compiler_hook: Option<&dyn Fn(&Vec<u8>, String)>,
) -> Result<Vec<u8>, Error> {
    let bitcode_data_set = DataSet::from_data(
        comgr,
        [
            Data::new(comgr, DataKind::Bc, c"zluda.bc", main_buffer)?,
            Data::new(comgr, DataKind::Bc, c"ptx_impl.bc", ptx_impl)?,
            Data::new(comgr, DataKind::Bc, c"attributes.bc", attributes_buffer)?,
        ]
        .into_iter(),
    )?;
    let linking_info = ActionInfo::new(comgr)?;
    let linked_data_set =
        comgr.do_action(ActionKind::LinkBcToBc, &linking_info, &bitcode_data_set)?;
    if let Some(hook) = compiler_hook {
        // Run compiler hook on human-readable LLVM IR
        let data = linked_data_set.get_content(comgr, DataKind::Bc, 0)?;
        let data = ptx::bitcode_to_ir(data);
        hook(&data, String::from("linked.ll"));
    }

    let common_options = [
        // This makes no sense, but it makes ockl linking work
        c"-Xclang",
        c"-mno-link-builtin-bitcode-postopt",
        // Otherwise LLVM omits dynamic fp mode for ockl functions during linking
        // and then fails to inline them
        c"-Xclang",
        c"-fdenormal-fp-math=dynamic",
        c"-O3",
        // To consider
        //c"-mllvm",
        //c"-amdgpu-internalize-symbols",
        c"-mno-wavefrontsize64",
        c"-mcumode",
        // Useful for inlining reports, combined with AMD_COMGR_SAVE_TEMPS=1 AMD_COMGR_EMIT_VERBOSE_LOGS=1 AMD_COMGR_REDIRECT_LOGS=stderr
        // c"-fsave-optimization-record=yaml",
    ]
    .into_iter();
    let opt_options = if cfg!(debug_assertions) {
        //[c"-g", c"-mllvm", c"-print-before-all", c"", c""]
        [c"-g", c"", c"", c"", c""]
    } else {
        [
            c"-g0",
            // default inlining threshold times 10
            c"-mllvm",
            c"-inline-threshold=2250",
            c"-mllvm",
            c"-inlinehint-threshold=3250",
        ]
    };
    let exec_data_set: DataSet;
    let executable: Result<Vec<u8>, Error>;
    if let Some(hook) = compiler_hook {
        // Run compiler hook for executable
        hook(
            executable.as_ref().unwrap_or(&Vec::new()),
            String::from("elf"),
        );

        // Disassemble executable and run compiler hook
        let action_info = ActionInfo::new(comgr)?;
        action_info.set_isa_name(gcn_arch)?;
        let disassembly = comgr.do_action(
            ActionKind::DisassembleExecutableToSource,
            &action_info,
            &exec_data_set,
        )?;
        let disassembly = disassembly.get_content(comgr, DataKind::Source, 0)?;
        hook(&disassembly, String::from("asm"))
    } else {
        let compile_to_exec = ActionInfo::new(comgr)?;
        compile_to_exec.set_isa_name(gcn_arch)?;
        compile_to_exec.set_language(Language::LlvmIr)?;
        compile_to_exec.set_options(common_options.chain(opt_options))?;
        let exec_data_set = comgr.do_action(
            ActionKind::CompileSourceToExecutable,
            &compile_to_exec,
            &linked_data_set,
        )?;
        executable = exec_data_set.get_content(comgr, DataKind::Executable, 0);
    }
    executable
}

pub fn get_symbols(comgr: &Comgr, elf: &[u8]) -> Result<Vec<(u32, String)>, Error> {
    let elf = Data::new(comgr, DataKind::Executable, c"elf.o", elf)?;
    let mut symbols = Vec::new();
    comgr.iterate_symbols(&elf, &mut |symbol| {
        let type_ = unsafe { comgr.symbol_get_info::<u32>(symbol, SymbolInfo::Type)? };
        let name_length = unsafe { comgr.symbol_get_info::<u64>(symbol, SymbolInfo::NameLength)? };
        let mut name =
            unsafe { comgr.symbol_get_buffer(symbol, SymbolInfo::Name, name_length as usize + 1)? };
        name.pop();
        let name = String::from_utf8(name).map_err(|_| Error::UNKNOWN)?;
        symbols.push((type_, name));
        Ok(())
    })?;
    Ok(symbols)
}

pub fn get_clang_version(comgr: &Comgr) -> Result<String, Error> {
    let version_string_set = DataSet::from_data(
        comgr,
        iter::once(Data::new(
            comgr,
            DataKind::Source,
            c"version.cpp",
            b"__clang_version__",
        )?),
    )?;
    let preprocessor_info = ActionInfo::new(comgr)?;
    preprocessor_info.set_language(Language::Hip)?;
    preprocessor_info.set_options(iter::once(c"-P"))?;
    let preprocessed = comgr.do_action(
        ActionKind::SourceToPreprocessor,
        &preprocessor_info,
        &version_string_set,
    )?;
    let data = preprocessed.get_content(comgr, DataKind::Source, 0)?;
    String::from_utf8(trim_whitespace_and_quotes(data)?).map_err(|_| Error::UNKNOWN)
}

// When running the preprocessor to expand the macro the output is surrounded by
// quotes (because it is a string literal) and has a trailing newline.
// This function is not strictly necessary, but it makes the output cleaner
fn trim_whitespace_and_quotes(data: Vec<u8>) -> Result<Vec<u8>, Error> {
    fn is_not_whitespace_or_quote(b: u8) -> bool {
        !(b.is_ascii_whitespace() || b == b'"')
    }
    let prefix_length = data
        .iter()
        .copied()
        .position(is_not_whitespace_or_quote)
        .ok_or(Error::UNKNOWN)?;
    let last_letter = data
        .iter()
        .copied()
        .rposition(is_not_whitespace_or_quote)
        .ok_or(Error::UNKNOWN)?;
    Ok(data[prefix_length..=last_letter].to_vec())
}

pub enum Comgr {
    V2(amd_comgr_sys::comgr2::Comgr2),
    V3(amd_comgr_sys::comgr3::Comgr3),
}

type SymbolIterator = dyn FnMut(Symbol) -> Result<(), Error>;

impl Comgr {
    pub fn new() -> Result<Self, Error> {
        unsafe { libloading::Library::new(os::COMGR3) }
            .and_then(|lib| {
                Ok(Comgr::V3(unsafe {
                    amd_comgr_sys::comgr3::Comgr3::from_library(lib)?
                }))
            })
            .or_else(|_| {
                unsafe { libloading::Library::new(os::COMGR2) }.and_then(|lib| {
                    Ok(if Self::is_broken_v2(&lib) {
                        Comgr::V3(unsafe { amd_comgr_sys::comgr3::Comgr3::from_library(lib)? })
                    } else {
                        Comgr::V2(unsafe { amd_comgr_sys::comgr2::Comgr2::from_library(lib)? })
                    })
                })
            })
            .map_err(Into::into)
    }

    // For reasons unknown, on AMD Adrenalin 25.5.1, AMD ships amd_comgr_2.dll that shows up as
    // version 2.9.0, but actually uses the 3.X ABI. This is our best effort to detect it.
    // Version 25.3.1 returns 2.8.0, which seem to be the last version that actually uses the 2 ABI
    fn is_broken_v2(lib: &libloading::Library) -> bool {
        if cfg!(not(windows)) {
            return false;
        }
        let amd_comgr_get_version = match unsafe {
            lib.get::<unsafe extern "C" fn(major: *mut usize, minor: *mut usize)>(
                b"amd_comgr_get_version\0",
            )
        } {
            Ok(symbol) => symbol,
            Err(_) => return false,
        };
        let mut major = 0;
        let mut minor = 0;
        unsafe { (amd_comgr_get_version)(&mut major, &mut minor) };
        (major, minor) >= (2, 9)
    }

    fn do_action<'a>(
        &'a self,
        kind: ActionKind,
        action: &ActionInfo,
        data_set: &DataSet,
    ) -> Result<DataSet<'a>, Error> {
        let result = DataSet::new(self)?;
        call_dispatch!(self => amd_comgr_do_action(kind, action, data_set, result));
        Ok(result)
    }

    fn iterate_symbols<'this>(
        &'this self,
        data: &Data,
        mut fn_: &mut (dyn FnMut(Symbol) -> Result<(), Error> + 'this),
    ) -> Result<(), Error> {
        let thin_pointer = &mut fn_;
        match self {
            Comgr::V2(comgr) => {
                unsafe {
                    comgr.amd_comgr_iterate_symbols(
                        data.comgr2(),
                        Some(Self::iterate_callback_v2),
                        mem::transmute(thin_pointer),
                    )
                }?;
            }
            Comgr::V3(comgr) => {
                unsafe {
                    comgr.amd_comgr_iterate_symbols(
                        data.comgr3(),
                        Some(Self::iterate_callback_v3),
                        mem::transmute(thin_pointer),
                    )
                }?;
            }
        }
        Ok(())
    }

    extern "C" fn iterate_callback_v3(
        symbol: comgr3::amd_comgr_symbol_t,
        user_data: *mut ::std::os::raw::c_void,
    ) -> Result<(), comgr3::amd_comgr_status_s> {
        let user_data = unsafe { mem::transmute::<_, &mut &mut SymbolIterator>(user_data) };
        (*user_data)(Symbol(symbol.handle)).map_err(Into::into)
    }

    extern "C" fn iterate_callback_v2(
        symbol: comgr2::amd_comgr_symbol_t,
        user_data: *mut ::std::os::raw::c_void,
    ) -> Result<(), comgr2::amd_comgr_status_s> {
        let user_data = unsafe { mem::transmute::<_, &mut &mut SymbolIterator>(user_data) };
        (*user_data)(Symbol(symbol.handle)).map_err(Into::into)
    }

    unsafe fn symbol_get_info<T>(&self, symbol: Symbol, attribute: SymbolInfo) -> Result<T, Error> {
        let mut value = unsafe { mem::zeroed::<T>() };
        call_dispatch!(self => amd_comgr_symbol_get_info(symbol, attribute, { ptr::from_mut(&mut value).cast() }));
        Ok(value)
    }

    unsafe fn symbol_get_buffer(
        &self,
        symbol: Symbol,
        attribute: SymbolInfo,
        length: usize,
    ) -> Result<Vec<u8>, Error> {
        let mut value = vec![0u8; length];
        call_dispatch!(self => amd_comgr_symbol_get_info(symbol, attribute, { value.as_mut_ptr().cast() }));
        Ok(value)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Comgr error: {0:?}")]
pub struct Error(pub ::std::num::NonZeroU32);

impl Error {
    #[doc = " A generic error has occurred."]
    pub const UNKNOWN: Error = Error(unsafe { ::std::num::NonZeroU32::new_unchecked(1) });
    #[doc = " One of the actual arguments does not meet a precondition stated\n in the documentation of the corresponding formal argument. This\n includes both invalid Action types, and invalid arguments to\n valid Action types."]
    pub const INVALID_ARGUMENT: Error = Error(unsafe { ::std::num::NonZeroU32::new_unchecked(2) });
    #[doc = " Failed to allocate the necessary resources."]
    pub const OUT_OF_RESOURCES: Error = Error(unsafe { ::std::num::NonZeroU32::new_unchecked(3) });
}

impl From<libloading::Error> for Error {
    fn from(_: libloading::Error) -> Self {
        Self::UNKNOWN
    }
}

impl From<comgr2::amd_comgr_status_s> for Error {
    fn from(status: comgr2::amd_comgr_status_s) -> Self {
        Error(status.0)
    }
}

impl Into<comgr2::amd_comgr_status_s> for Error {
    fn into(self) -> comgr2::amd_comgr_status_s {
        comgr2::amd_comgr_status_s(self.0)
    }
}

impl From<comgr3::amd_comgr_status_s> for Error {
    fn from(status: comgr3::amd_comgr_status_s) -> Self {
        Error(status.0)
    }
}

impl Into<comgr3::amd_comgr_status_s> for Error {
    fn into(self) -> comgr3::amd_comgr_status_s {
        comgr3::amd_comgr_status_s(self.0)
    }
}

macro_rules! impl_into {
    ($self_type:ident, $to_type:ident, [$($from:ident => $to:ident),+]) => {
        #[derive(Copy, Clone)]
        #[allow(unused)]
        enum $self_type {
            $(
                $from,
            )+
        }

        impl $self_type {
            #[allow(dead_code)]
            fn comgr2(self) -> comgr2::$to_type {
                match self {
                    $(
                        Self:: $from => comgr2 :: $to_type :: $to,
                    )+
                }
            }

            #[allow(dead_code)]
            fn comgr3(self) -> comgr3::$to_type {
                match self {
                    $(
                        Self:: $from => comgr3 :: $to_type :: $to,
                    )+
                }
            }
        }
    };
}

impl_into!(
    ActionKind,
    amd_comgr_action_kind_t,
    [
        LinkBcToBc => AMD_COMGR_ACTION_LINK_BC_TO_BC,
        CompileSourceToExecutable => AMD_COMGR_ACTION_COMPILE_SOURCE_TO_EXECUTABLE,
        CompileSourceWithDeviceLibsToBc => AMD_COMGR_ACTION_COMPILE_SOURCE_WITH_DEVICE_LIBS_TO_BC,
        CodegenBcToRelocatable => AMD_COMGR_ACTION_CODEGEN_BC_TO_RELOCATABLE,
        LinkRelocatableToExecutable => AMD_COMGR_ACTION_LINK_RELOCATABLE_TO_EXECUTABLE,
        DisassembleExecutableToSource => AMD_COMGR_ACTION_DISASSEMBLE_EXECUTABLE_TO_SOURCE,
        SourceToPreprocessor => AMD_COMGR_ACTION_SOURCE_TO_PREPROCESSOR
    ]
);

impl_into!(
    SymbolType,
    amd_comgr_symbol_type_t,
    [
        Unknown => AMD_COMGR_SYMBOL_TYPE_UNKNOWN,
        NoType => AMD_COMGR_SYMBOL_TYPE_NOTYPE,
        Object => AMD_COMGR_SYMBOL_TYPE_OBJECT,
        Func => AMD_COMGR_SYMBOL_TYPE_FUNC,
        Section => AMD_COMGR_SYMBOL_TYPE_SECTION,
        File => AMD_COMGR_SYMBOL_TYPE_FILE,
        Common => AMD_COMGR_SYMBOL_TYPE_COMMON,
        AmdgpuHsaKernel => AMD_COMGR_SYMBOL_TYPE_AMDGPU_HSA_KERNEL
    ]
);

impl_into!(
    SymbolInfo,
    amd_comgr_symbol_info_t,
    [
        NameLength => AMD_COMGR_SYMBOL_INFO_NAME_LENGTH,
        Name => AMD_COMGR_SYMBOL_INFO_NAME,
        Type => AMD_COMGR_SYMBOL_INFO_TYPE
    ]
);

impl_into!(
    DataKind,
    amd_comgr_data_kind_t,
    [
        Undef => AMD_COMGR_DATA_KIND_UNDEF,
        Source => AMD_COMGR_DATA_KIND_SOURCE,
        Include => AMD_COMGR_DATA_KIND_INCLUDE,
        PrecompiledHeader => AMD_COMGR_DATA_KIND_PRECOMPILED_HEADER,
        Diagnostic => AMD_COMGR_DATA_KIND_DIAGNOSTIC,
        Log => AMD_COMGR_DATA_KIND_LOG,
        Bc => AMD_COMGR_DATA_KIND_BC,
        Relocatable => AMD_COMGR_DATA_KIND_RELOCATABLE,
        Executable => AMD_COMGR_DATA_KIND_EXECUTABLE,
        Bytes => AMD_COMGR_DATA_KIND_BYTES,
        Fatbin => AMD_COMGR_DATA_KIND_FATBIN,
        Ar => AMD_COMGR_DATA_KIND_AR,
        BcBundle => AMD_COMGR_DATA_KIND_BC_BUNDLE,
        ArBundle => AMD_COMGR_DATA_KIND_AR_BUNDLE,
        ObjBundle => AMD_COMGR_DATA_KIND_OBJ_BUNDLE

    ]
);

impl_into!(
    Language,
    amd_comgr_language_t,
    [
        None => AMD_COMGR_LANGUAGE_NONE,
        OpenCl12 => AMD_COMGR_LANGUAGE_OPENCL_1_2,
        OpenCl20 => AMD_COMGR_LANGUAGE_OPENCL_2_0,
        Hip => AMD_COMGR_LANGUAGE_HIP,
        LlvmIr => AMD_COMGR_LANGUAGE_LLVM_IR
    ]
);

#[cfg(unix)]
mod os {
    pub static COMGR3: &'static str = "libamd_comgr.so.3";
    pub static COMGR2: &'static str = "libamd_comgr.so.2";
}

#[cfg(windows)]
mod os {
    pub static COMGR3: &'static str = "amd_comgr_3.dll";
    pub static COMGR2: &'static str = "amd_comgr_2.dll";
}
