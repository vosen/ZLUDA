#![cfg(windows)]
use std::{
    ffi::{c_void, CStr, CString, OsString},
    iter,
    ops::ControlFlow,
    os::windows::ffi::{OsStrExt, OsStringExt},
    path::PathBuf,
};
use trie_hard::TrieHard;
use uuid::uuid;
use widestring::{u16str, U16Str};
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::HMODULE,
        System::LibraryLoader::{
            GetModuleFileNameA, GetModuleFileNameW, GetModuleHandleExW, LoadLibraryExW,
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS, GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
            LOAD_LIBRARY_SEARCH_DEFAULT_DIRS, LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR,
        },
    },
};

pub static LIBRARIES: [LibraryInfo; 12] = [
    NVCUDA, NVML, DNN8, DNN9, BLAS13, BLAS12, BLAS_LT13, BLAS_LT12, SPARSE12, SPARSE11, FFT12,
    FFT11,
];

pub const NVCUDA: LibraryInfo = LibraryInfo {
    short_name: "nvcuda",
    is_alias: false,
    ascii_name: "nvcuda.dll",
    utf16_name: u16str!("nvcuda.dll"),
    guid: uuid!("9ab28276-52f4-4fe4-bc19-4c966f3aa468"),
    trace_env_var: "ZLUDA_CUDA_LIB",
    in_system32: true,
};

pub const NVML: LibraryInfo = LibraryInfo {
    short_name: "nvml",
    is_alias: false,
    ascii_name: "nvml.dll",
    utf16_name: u16str!("nvml.dll"),
    guid: uuid!("4fd0ef13-df98-40a7-b34b-6cf55f34a8e9"),
    trace_env_var: "ZLUDA_ML_LIB",
    in_system32: true,
};

pub const DNN8: LibraryInfo = LibraryInfo {
    short_name: "cudnn8",
    is_alias: false,
    ascii_name: "cudnn64_8.dll",
    utf16_name: u16str!("cudnn64_8.dll"),
    guid: uuid!("7ed2e46c-82ce-4779-adbd-b39cd6d9da80"),
    trace_env_var: "ZLUDA_DNN8_LIB",
    in_system32: false,
};

pub const DNN9: LibraryInfo = LibraryInfo {
    short_name: "cudnn9",
    is_alias: false,
    ascii_name: "cudnn64_9.dll",
    utf16_name: u16str!("cudnn64_9.dll"),
    guid: uuid!("6c258b11-104a-4ab0-b343-4e490e136ddd"),
    trace_env_var: "ZLUDA_DNN9_LIB",
    in_system32: false,
};

pub const BLAS13: LibraryInfo = LibraryInfo {
    short_name: "cublas13",
    is_alias: false,
    ascii_name: "cublas64_13.dll",
    utf16_name: u16str!("cublas64_13.dll"),
    guid: uuid!("b9dcf552-c7a1-4dfe-b27a-58b56695effe"),
    trace_env_var: "ZLUDA_BLAS_LIB",
    in_system32: false,
};

pub const BLAS12: LibraryInfo = LibraryInfo {
    short_name: "cublas12",
    is_alias: true,
    ascii_name: "cublas64_12.dll",
    utf16_name: u16str!("cublas64_12.dll"),
    guid: uuid!("2c77f12f-e7dc-4dac-88a0-f06f43ceb566"),
    trace_env_var: "ZLUDA_BLAS_LIB",
    in_system32: false,
};

pub const BLAS_LT13: LibraryInfo = LibraryInfo {
    short_name: "cublaslt",
    is_alias: false,
    ascii_name: "cublaslt64_13.dll",
    utf16_name: u16str!("cublaslt64_13.dll"),
    guid: uuid!("667b6076-c4c2-44f4-8b81-073dea1d6125"),
    trace_env_var: "ZLUDA_BLASLT_LIB",
    in_system32: false,
};

pub const BLAS_LT12: LibraryInfo = LibraryInfo {
    short_name: "cublaslt",
    is_alias: true,
    ascii_name: "cublaslt64_12.dll",
    utf16_name: u16str!("cublaslt64_12.dll"),
    guid: uuid!("14aeb961-4ef2-480b-9813-df4a6dde454f"),
    trace_env_var: "ZLUDA_BLASLT_LIB",
    in_system32: false,
};

pub const SPARSE12: LibraryInfo = LibraryInfo {
    short_name: "cusparse",
    is_alias: false,
    ascii_name: "cusparse64_12.dll",
    utf16_name: u16str!("cusparse64_12.dll"),
    guid: uuid!("741fb1b7-2e24-4484-8205-4794175ccb78"),
    trace_env_var: "ZLUDA_SPARSE_LIB",
    in_system32: false,
};

pub const SPARSE11: LibraryInfo = LibraryInfo {
    short_name: "cusparse",
    is_alias: true,
    ascii_name: "cusparse64_11.dll",
    utf16_name: u16str!("cusparse64_11.dll"),
    guid: uuid!("7d03e8d0-6153-48de-af82-5a53828c9568"),
    trace_env_var: "ZLUDA_SPARSE_LIB",
    in_system32: false,
};

pub const FFT12: LibraryInfo = LibraryInfo {
    short_name: "cufft",
    is_alias: false,
    ascii_name: "cufft64_12.dll",
    utf16_name: u16str!("cufft64_12.dll"),
    guid: uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8"),
    trace_env_var: "ZLUDA_FFT_LIB",
    in_system32: false,
};

pub const FFT11: LibraryInfo = LibraryInfo {
    short_name: "cufft",
    is_alias: true,
    ascii_name: "cufft64_11.dll",
    utf16_name: u16str!("cufft64_11.dll"),
    guid: uuid!("34d2be7e-e992-4aee-8502-b5fc14a02db3"),
    trace_env_var: "ZLUDA_FFT_LIB",
    in_system32: false,
};

#[derive(Debug)]
pub struct LibraryInfo {
    pub short_name: &'static str,
    pub is_alias: bool,
    pub ascii_name: &'static str,
    pub utf16_name: &'static U16Str,
    pub guid: uuid::Uuid,
    pub trace_env_var: &'static str,
    pub in_system32: bool,
}

pub struct DllLookup {
    max_dll_len: usize,
    ascii_trie: TrieHard<'static, usize>,
    utf16_trie: TrieHard<'static, usize>,
}

impl DllLookup {
    pub fn new() -> Self {
        let library_names = LIBRARIES
            .iter()
            .enumerate()
            .map(|(i, lib)| {
                let ascii_bytes = lib.ascii_name.as_bytes();
                (ascii_bytes, i)
            })
            .collect::<Vec<_>>();
        let ascii_trie = TrieHard::new(library_names);
        let max_dll_len = ascii_trie
            .iter()
            .map(|(dll_name, _)| dll_name.len())
            .max()
            .unwrap_or(0);
        let library_names_utf16 = LIBRARIES
            .iter()
            .enumerate()
            .map(|(i, lib)| {
                let utf16_bytes = Self::to_byte_slice(lib.utf16_name.as_slice());
                (utf16_bytes, i)
            })
            .collect::<Vec<_>>();
        let utf16_trie = TrieHard::new(library_names_utf16);
        Self {
            max_dll_len,
            ascii_trie,
            utf16_trie,
        }
    }

    fn to_byte_slice<'a, T: Copy>(s: &'a [T]) -> &'a [u8] {
        unsafe {
            std::slice::from_raw_parts(s.as_ptr() as *const u8, s.len() * std::mem::size_of::<T>())
        }
    }

    pub fn lookup_impl<T: Copy>(
        max_dll_len: usize,
        trie: &TrieHard<'static, usize>,
        dll_name: &[T],
        as_char: impl Fn(T) -> char,
        lower_case: impl Fn(T) -> T,
    ) -> Option<usize> {
        let file_name_len = dll_name
            .iter()
            .rev()
            .copied()
            .enumerate()
            .try_fold((), |_, (i, c)| {
                if i > max_dll_len {
                    return ControlFlow::Break(None);
                }
                let c = as_char(c);
                if c == '\\' || c == '/' {
                    return ControlFlow::Break(Some(i));
                }
                ControlFlow::Continue(())
            });
        let file_name_length = match file_name_len {
            ControlFlow::Break(Some(len)) => len,
            ControlFlow::Break(None) => return None,
            ControlFlow::Continue(()) => dll_name.len(),
        };
        let file_name = &dll_name[dll_name.len() - file_name_length..];
        alloca::with_alloca(file_name.len() * std::mem::size_of::<T>(), |uninit_slice| {
            let lower_case_slice = unsafe {
                std::slice::from_raw_parts_mut(
                    uninit_slice.as_mut_ptr().cast::<T>(),
                    file_name.len(),
                )
            };
            for (dst, src) in lower_case_slice.iter_mut().zip(file_name.iter().copied()) {
                *dst = lower_case(src);
            }
            trie.get(Self::to_byte_slice(lower_case_slice))
        })
    }

    pub fn lookup_ascii(&self, dll_name: &[u8]) -> Option<usize> {
        Self::lookup_impl(
            self.max_dll_len,
            &self.ascii_trie,
            dll_name,
            |c| c as char,
            |c| c.to_ascii_lowercase(),
        )
    }

    pub fn lookup_utf16(&self, dll_name: &[u16]) -> Option<usize> {
        Self::lookup_impl(
            self.max_dll_len,
            &self.utf16_trie,
            dll_name,
            |c| char::from_u32(c as u32).unwrap_or_default(),
            |c| {
                char::from_u32(c as u32)
                    .unwrap_or_default()
                    .to_ascii_lowercase() as u16
            },
        )
    }
}

pub type DWORD = u32;
pub type LPCSTR = *const i8;
pub type FARPROC = Option<unsafe extern "system" fn() -> isize>;
pub type PCImgDelayDescr = *const c_void;
pub type DelayLoadProc = *const c_void;

#[repr(C)]
pub struct DelayLoadInfo {
    pub cb: u32,
    pub pidd: PCImgDelayDescr,
    pub ppfn: *mut FARPROC,
    pub sz_dll: LPCSTR,
    pub dlp: DelayLoadProc,
    pub hmod_cur: *mut c_void,
    pub pfn_cur: FARPROC,
    pub dw_last_error: u32,
}

#[repr(C)]
pub enum DliNotify {
    _NoteStartProcessing,
    _NotePreLoadLibrary,
    _NotePreGetProcAddress,
    FailLoadLib,
    _FailGetProc,
    _NoteEndProcessing,
}

pub type PfnDliHook =
    unsafe extern "system" fn(dli_notify: u32, pdli: *const DelayLoadInfo) -> *mut std::ffi::c_void;

pub unsafe fn delay_load_failure_hook(
    redirect_name: &'static str,
    dli_notify: u32,
    pdli: *const DelayLoadInfo,
) -> Option<HMODULE> {
    if dli_notify != DliNotify::FailLoadLib as u32 {
        return None;
    }
    let pdli = pdli.as_ref()?;
    let name = CStr::from_ptr(pdli.sz_dll);
    if !name.to_str().ok()?.eq_ignore_ascii_case(redirect_name) {
        return None;
    }
    try_load_from_self_dir(redirect_name).or_else(|| try_load_from_hip_path(redirect_name))
}

pub unsafe fn try_load_from_self_dir(libname: &str) -> Option<HMODULE> {
    let mut hm = HMODULE::default();
    GetModuleHandleExW(
        GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
        PCWSTR(try_load_from_self_dir as _),
        &mut hm,
    )
    .ok()?;
    let path = get_module_path_utf16(hm);
    let mut path_buf = PathBuf::from(path);
    path_buf.pop();
    path_buf.push(libname);
    let path_utf16 = path_buf
        .as_os_str()
        .encode_wide()
        .chain(iter::once(0))
        .collect::<Vec<_>>();
    LoadLibraryExW(
        PCWSTR(path_utf16.as_ptr()),
        None,
        LOAD_LIBRARY_SEARCH_DEFAULT_DIRS | LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR,
    )
    .ok()
}

unsafe fn try_load_from_hip_path(redirect_name: &'static str) -> Option<HMODULE> {
    let hip_path = std::env::var_os("HIP_PATH")?;
    let hip_dll_path = PathBuf::from(hip_path)
        .join("bin")
        .join(redirect_name)
        .as_os_str()
        .encode_wide()
        .chain(iter::once(0))
        .collect::<Vec<_>>();
    LoadLibraryExW(
        PCWSTR(hip_dll_path.as_ptr()),
        None,
        LOAD_LIBRARY_SEARCH_DEFAULT_DIRS | LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR,
    )
    .ok()
}

pub fn get_module_path(instance_handle: *mut c_void) -> CString {
    let mut buffer = vec![0u8; windows::Win32::Foundation::MAX_PATH as usize];
    let mut copied;
    loop {
        copied = unsafe { GetModuleFileNameA(Some(HMODULE(instance_handle)), &mut buffer) };
        if (copied as usize) < buffer.len() {
            break;
        } else {
            buffer.resize(buffer.len() * 2, 0);
        }
    }
    buffer.truncate(copied as usize + 1);
    unsafe { CString::from_vec_with_nul_unchecked(buffer) }
}

fn get_module_path_utf16(instance_handle: HMODULE) -> OsString {
    let mut buffer = vec![0u16; windows::Win32::Foundation::MAX_PATH as usize];
    let mut copied;
    loop {
        copied = unsafe { GetModuleFileNameW(Some(instance_handle), &mut buffer) };
        if (copied as usize) < buffer.len() {
            break;
        } else {
            buffer.resize(buffer.len() * 2, 0);
        }
    }
    buffer.truncate(copied as usize);
    OsString::from_wide(&buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_look_up_from_full_path() {
        let lookup = DllLookup::new();
        let path = r#"C:\Windows\System32\nvml.dll"#;
        assert_eq!(
            NVML.guid,
            LIBRARIES[lookup.lookup_ascii(path.as_bytes()).unwrap()].guid
        );
    }

    #[test]
    fn can_look_up_from_just_file_name() {
        let lookup = DllLookup::new();
        let path = r#"cusparse64_11.dll"#;
        assert_eq!(
            SPARSE11.guid,
            LIBRARIES[lookup.lookup_ascii(path.as_bytes()).unwrap()].guid
        );
    }

    #[test]
    fn can_look_up_utf16() {
        let lookup = DllLookup::new();
        let path = u16str!("\\cublaslt64_13.dll");
        assert_eq!(
            BLAS_LT13.guid,
            LIBRARIES[lookup.lookup_utf16(path.as_slice()).unwrap()].guid
        );
    }

    #[test]
    fn can_look_up_utf16_case_insensitive() {
        let lookup = DllLookup::new();
        let path = u16str!("\\CUBLASLT64_13.dll");
        assert_eq!(
            BLAS_LT13.guid,
            LIBRARIES[lookup.lookup_utf16(path.as_slice()).unwrap()].guid
        );
    }

    #[test]
    fn fails_on_mismatch() {
        let lookup = DllLookup::new();
        let path = u16str!("cusparse64_99.dll");
        assert!(lookup.lookup_utf16(path.as_slice()).is_none());
    }

    #[test]
    fn does_not_explode_on_utf16() {
        let lookup = DllLookup::new();
        let path = u16str!("🇧🇴");
        assert_eq!(path.chars().count(), 2);
        assert_eq!(path.as_slice().len(), 4);
        assert!(lookup.lookup_utf16(path.as_slice()).is_none());
    }
}
