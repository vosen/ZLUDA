use amd_comgr_sys::*;
use std::{ffi::CStr, mem, ptr};

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
    fn set_isa_name(&self, isa: &CStr) -> Result<(), Error> {
        let mut full_isa = "amdgcn-amd-amdhsa--".to_string().into_bytes();
        full_isa.extend(isa.to_bytes_with_nul());
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

    fn get_data(&self, kind: DataKind, index: usize) -> Result<Data, Error> {
        let mut handle = 0u64;
        call_dispatch!(self.comgr => amd_comgr_action_data_get_data(self, kind, { index }, { std::ptr::from_mut(&mut handle).cast() }));
        Ok(Data(handle))
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

pub fn compile_bitcode(
    comgr: &Comgr,
    gcn_arch: &CStr,
    main_buffer: &[u8],
    ptx_impl: &[u8],
) -> Result<Vec<u8>, Error> {
    let bitcode_data_set = DataSet::new(comgr)?;
    let main_bitcode_data = Data::new(comgr, DataKind::Bc, c"zluda.bc", main_buffer)?;
    bitcode_data_set.add(&main_bitcode_data)?;
    let stdlib_bitcode_data = Data::new(comgr, DataKind::Bc, c"ptx_impl.bc", ptx_impl)?;
    bitcode_data_set.add(&stdlib_bitcode_data)?;
    let linking_info = ActionInfo::new(comgr)?;
    comgr.do_action(ActionKind::LinkBcToBc, &linking_info, &bitcode_data_set)?;
    let linked_data_set =
        comgr.do_action(ActionKind::LinkBcToBc, &linking_info, &bitcode_data_set)?;
    let compile_to_exec = ActionInfo::new(comgr)?;
    compile_to_exec.set_isa_name(gcn_arch)?;
    compile_to_exec.set_language(Language::LlvmIr)?;
    let common_options = [
        // This makes no sense, but it makes ockl linking work
        c"-Xclang",
        c"-mno-link-builtin-bitcode-postopt",
        // Otherwise LLVM omits dynamic fp mode for ockl functions during linking
        // and then fails to inline them
        c"-Xclang",
        c"-fdenormal-fp-math=dynamic",
        c"-O3",
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
    compile_to_exec.set_options(common_options.chain(opt_options))?;
    let exec_data_set = comgr.do_action(
        ActionKind::CompileSourceToExecutable,
        &compile_to_exec,
        &linked_data_set,
    )?;
    let executable = exec_data_set.get_data(DataKind::Executable, 0)?;
    executable.copy_content(comgr)
}

pub enum Comgr {
    V2(amd_comgr_sys::comgr2::Comgr2),
    V3(amd_comgr_sys::comgr3::Comgr3),
}

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

    fn do_action(
        &self,
        kind: ActionKind,
        action: &ActionInfo,
        data_set: &DataSet,
    ) -> Result<DataSet, Error> {
        let result = DataSet::new(self)?;
        call_dispatch!(self => amd_comgr_do_action(kind, action, data_set, result));
        Ok(result)
    }
}

#[derive(Debug)]
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
    fn from(_: comgr2::amd_comgr_status_s) -> Self {
        todo!()
    }
}

impl From<comgr3::amd_comgr_status_s> for Error {
    fn from(_: comgr3::amd_comgr_status_s) -> Self {
        todo!()
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
            fn comgr2(self) -> comgr2::$to_type {
                match self {
                    $(
                        Self:: $from => comgr2 :: $to_type :: $to,
                    )+
                }
            }

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
        CompileSourceToExecutable => AMD_COMGR_ACTION_COMPILE_SOURCE_TO_EXECUTABLE
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
