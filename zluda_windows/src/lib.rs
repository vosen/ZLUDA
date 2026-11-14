#![cfg(windows)]
use std::{
    ffi::{c_void, CStr, CString, OsString},
    iter, mem,
    ops::ControlFlow,
    os::windows::ffi::{OsStrExt, OsStringExt},
    path::PathBuf,
    ptr,
};
use trie_hard::TrieHard;
use widestring::{u16str, U16Str, U16String};
use windows::{
    core::{w, HRESULT, PCWSTR},
    Win32::{
        Foundation::{HANDLE, HINSTANCE, HMODULE, HWND, LPARAM, S_OK, WPARAM},
        System::{
            JobObjects::{
                AssignProcessToJobObject, JobObjectExtendedLimitInformation,
                SetInformationJobObject, JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
                JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
            },
            LibraryLoader::{
                GetModuleFileNameA, GetModuleFileNameW, GetModuleHandleExW, LoadLibraryExW,
                GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
                GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, LOAD_LIBRARY_SEARCH_DEFAULT_DIRS,
                LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR,
            },
            StationsAndDesktops::{
                GetProcessWindowStation, GetUserObjectInformationW, UOI_FLAGS, USEROBJECTFLAGS,
            },
        },
        UI::{
            Controls::{
                TaskDialogIndirect, TASKDIALOGCONFIG, TASKDIALOGCONFIG_0, TASKDIALOGCONFIG_1,
                TASKDIALOG_NOTIFICATIONS, TDCBF_CLOSE_BUTTON, TDF_ENABLE_HYPERLINKS,
                TDN_HYPERLINK_CLICKED, TD_ERROR_ICON,
            },
            WindowsAndMessaging::{IDCLOSE, WSF_VISIBLE},
        },
    },
};

pub static LIBRARIES32: [LibraryInfo; 2] = [NVCUDA32, NVAPI32];

pub static LIBRARIES64: [LibraryInfo; 10] = [
    NVCUDA, NVML, DNN8, DNN9, BLAS, BLASLT, SPARSE, FFT, NVAPI, NVRTC,
];

pub const NVCUDA: LibraryInfo = LibraryInfo {
    name: u16str!("nvcuda.dll"),
    zluda_name: Some(u16str!("nvcuda.dll")),
    name_kind: LibraryNameKind::Exact,
    trace_env_var: Some("ZLUDA_CUDA_LIB"),
    in_system32: true,
};

pub const NVCUDA32: LibraryInfo = LibraryInfo {
    name: u16str!("nvcuda.dll"),
    zluda_name: Some(u16str!("nvcuda32.dll")),
    name_kind: LibraryNameKind::Exact,
    trace_env_var: Some("ZLUDA_CUDA32_LIB"),
    in_system32: true,
};

pub const NVML: LibraryInfo = LibraryInfo {
    name: u16str!("nvml.dll"),
    zluda_name: Some(u16str!("nvml.dll")),
    name_kind: LibraryNameKind::Exact,
    trace_env_var: Some("ZLUDA_ML_LIB"),
    in_system32: true,
};

pub const DNN8: LibraryInfo = LibraryInfo {
    name: u16str!("cudnn64_8.dll"),
    zluda_name: Some(u16str!("cudnn64_8.dll")),
    name_kind: LibraryNameKind::Exact,
    trace_env_var: Some("ZLUDA_DNN8_LIB"),
    in_system32: false,
};

pub const DNN9: LibraryInfo = LibraryInfo {
    name: u16str!("cudnn64_9.dll"),
    zluda_name: Some(u16str!("cudnn64_9.dll")),
    name_kind: LibraryNameKind::Exact,
    trace_env_var: Some("ZLUDA_DNN9_LIB"),
    in_system32: false,
};

pub const BLAS: LibraryInfo = LibraryInfo {
    name: u16str!("cublas64_"),
    zluda_name: Some(u16str!("cublas.dll")),
    name_kind: LibraryNameKind::Versioned,
    trace_env_var: Some("ZLUDA_BLAS_LIB"),
    in_system32: false,
};

pub const BLASLT: LibraryInfo = LibraryInfo {
    name: u16str!("cublaslt64_"),
    zluda_name: Some(u16str!("cublaslt.dll")),
    name_kind: LibraryNameKind::Versioned,
    trace_env_var: Some("ZLUDA_BLASLT_LIB"),
    in_system32: false,
};

pub const SPARSE: LibraryInfo = LibraryInfo {
    name: u16str!("cusparse64_"),
    zluda_name: Some(u16str!("cusparse.dll")),
    name_kind: LibraryNameKind::Versioned,
    trace_env_var: Some("ZLUDA_SPARSE_LIB"),
    in_system32: false,
};

pub const FFT: LibraryInfo = LibraryInfo {
    name: u16str!("cufft64_"),
    zluda_name: Some(u16str!("cufft.dll")),
    name_kind: LibraryNameKind::Versioned,
    trace_env_var: Some("ZLUDA_FFT_LIB"),
    in_system32: false,
};

pub const NVAPI: LibraryInfo = LibraryInfo {
    name: u16str!("nvapi64.dll"),
    zluda_name: Some(u16str!("nvapi64.dll")),
    name_kind: LibraryNameKind::Exact,
    trace_env_var: Some("ZLUDA_NVAPI_LIB"),
    in_system32: true,
};

pub const NVAPI32: LibraryInfo = LibraryInfo {
    name: u16str!("nvapi.dll"),
    zluda_name: Some(u16str!("nvapi.dll")),
    name_kind: LibraryNameKind::Exact,
    trace_env_var: Some("ZLUDA_NVAPI32_LIB"),
    in_system32: true,
};

pub const NVRTC: LibraryInfo = LibraryInfo {
    name: u16str!("nvrtc64_"),
    zluda_name: None,
    trace_env_var: None,
    name_kind: LibraryNameKind::Versioned,
    in_system32: false,
};

#[derive(Debug)]
pub struct LibraryInfo {
    pub name: &'static U16Str,
    pub zluda_name: Option<&'static U16Str>,
    name_kind: LibraryNameKind,
    pub trace_env_var: Option<&'static str>,
    pub in_system32: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LibraryNameKind {
    Exact,
    Versioned,
}

pub struct DllLookup {
    is_64: bool,
    max_dll_len: usize,
    trie: TrieHard<'static, usize>,
}

impl DllLookup {
    pub fn new32() -> Self {
        Self::new(&LIBRARIES32, false)
    }

    pub fn new64() -> Self {
        Self::new(&LIBRARIES64, true)
    }

    fn new(libraries: &[LibraryInfo], is_64: bool) -> Self {
        let library_names = libraries
            .iter()
            .enumerate()
            .map(|(i, lib)| (Self::to_byte_slice(lib.name.as_slice()), i))
            .collect::<Vec<_>>();
        let trie = TrieHard::new(library_names);
        let max_dll_len = trie
            .iter()
            .map(|(dll_name, _)| dll_name.len())
            .max()
            .unwrap_or(0);
        Self {
            is_64,
            max_dll_len,
            trie,
        }
    }

    fn to_byte_slice<'a, T: Copy>(s: &'a [T]) -> &'a [u8] {
        unsafe {
            std::slice::from_raw_parts(s.as_ptr() as *const u8, s.len() * std::mem::size_of::<T>())
        }
    }

    pub fn lookup(&self, dll_name: &[u16]) -> Option<usize> {
        let AncestorMatch {
            file_name,
            library_index,
        } = self.find(dll_name)?;
        let libraries = if self.is_64 {
            &LIBRARIES64[..]
        } else {
            &LIBRARIES32[..]
        };
        match libraries[library_index].name_kind {
            LibraryNameKind::Exact => {
                if file_name.len() == libraries[library_index].name.as_slice().len() {
                    Some(library_index)
                } else {
                    None
                }
            }
            LibraryNameKind::Versioned => {
                if parser::is_version_dll(
                    &file_name[libraries[library_index].name.as_slice().len()..],
                ) {
                    Some(library_index)
                } else {
                    None
                }
            }
        }
    }

    fn find<'a>(&self, dll_name: &'a [u16]) -> Option<AncestorMatch<'a>> {
        let file_name_len = dll_name
            .iter()
            .rev()
            .copied()
            .enumerate()
            .try_fold((), |_, (i, c)| {
                if let Ok(c) = u8::try_from(c) {
                    if !c.is_ascii() {
                        return ControlFlow::Break(None);
                    }
                } else {
                    return ControlFlow::Break(None);
                }
                if i > self.max_dll_len {
                    return ControlFlow::Break(None);
                }
                if c == b'\\' as u16 || c == b'/' as u16 {
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
        alloca::with_alloca(
            file_name.len() * std::mem::size_of::<u16>(),
            |alloca_slice| {
                let alloca_slice = unsafe {
                    std::slice::from_raw_parts_mut(
                        alloca_slice.as_mut_ptr().cast::<mem::MaybeUninit<u16>>(),
                        file_name.len(),
                    )
                };
                for (i, c) in file_name.iter().copied().enumerate() {
                    alloca_slice[i].write((c as u8).to_ascii_lowercase() as u16);
                }
                let init_slice = unsafe { alloca_slice.assume_init_ref() };
                self.trie
                    .ancestor(Self::to_byte_slice(init_slice))
                    .map(|(_, library_index)| AncestorMatch {
                        library_index,
                        file_name,
                    })
            },
        )
    }
}

struct AncestorMatch<'a> {
    library_index: usize,
    file_name: &'a [u16],
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
    NotePreLoadLibrary,
    _NotePreGetProcAddress,
    FailLoadLib,
    _FailGetProc,
    _NoteEndProcessing,
}

pub type PfnDliHook =
    unsafe extern "system" fn(dli_notify: u32, pdli: *const DelayLoadInfo) -> *mut std::ffi::c_void;

pub unsafe extern "system" fn open_already_loaded_amdhip_probe_perflibs(
    dli_notify: u32,
    pdli: *const DelayLoadInfo,
) -> *mut std::ffi::c_void {
    open_already_loaded_amdhip_impl(true, dli_notify, pdli)
}

pub unsafe extern "system" fn open_already_loaded_amdhip(
    dli_notify: u32,
    pdli: *const DelayLoadInfo,
) -> *mut std::ffi::c_void {
    open_already_loaded_amdhip_impl(false, dli_notify, pdli)
}

unsafe extern "system" fn open_already_loaded_amdhip_impl(
    probe_perflibs: bool,
    dli_notify: u32,
    pdli: *const DelayLoadInfo,
) -> *mut std::ffi::c_void {
    unsafe fn redirect_amdhip_impl(
        probe_perflibs: bool,
        dli_notify: u32,
        pdli: *const DelayLoadInfo,
    ) -> Option<HMODULE> {
        delay_load_check(
            ("amdhip64_7.dll", DliNotify::NotePreLoadLibrary),
            dli_notify,
            pdli,
        )?;
        // This is extremally dumb, but there's not much better we can do.
        // We do this because we want to avoid this ordering of events:
        // * First we load ROCm 7 runtime
        // * Later on a performance library (rocBLAS) loads ROCm 6 runtime
        // * Things explode
        if probe_perflibs {
            try_load_from_self_or_hip("hipblaslt.dll")
                .or_else(|| try_load_from_self_or_hip("libhipblaslt.dll"))
                .or_else(|| try_load_from_self_or_hip("rocblas.dll"))
                .or_else(|| try_load_from_self_or_hip("rocsparse.dll"))
                .or_else(|| try_load_from_self_or_hip("miopen.dll"));
        }
        let mut module = mem::zeroed();
        GetModuleHandleExW(0, w!("amdhip64_7.dll"), &mut module)
            .ok()
            .or_else(|| GetModuleHandleExW(0, w!("amdhip64_6.dll"), &mut module).ok())?;
        Some(module)
    }
    match redirect_amdhip_impl(probe_perflibs, dli_notify, pdli) {
        Some(lib) => lib.0,
        None => std::ptr::null_mut(),
    }
}

pub unsafe fn delay_load_failure_hook(
    redirect_name: &'static str,
    alt_name: Option<&'static str>,
    dli_notify: u32,
    pdli: *const DelayLoadInfo,
) -> Option<HMODULE> {
    delay_load_check((redirect_name, DliNotify::FailLoadLib), dli_notify, pdli)?;
    [redirect_name]
        .iter()
        .copied()
        .chain(alt_name)
        .find_map(|name| unsafe { try_load_from_self_or_hip(name) })
}

pub unsafe fn try_load_from_self_or_hip(redirect_name: &'static str) -> Option<HMODULE> {
    try_load_from_self_dir(redirect_name).or_else(|| try_load_from_hip_path(redirect_name))
}

// Taken from .NET Environment.UserInteractive
unsafe fn is_interactive_session() -> bool {
    let Ok(station) = GetProcessWindowStation() else {
        return false;
    };
    let mut flags: USEROBJECTFLAGS = mem::zeroed();
    let mut dummy = 0;
    if GetUserObjectInformationW(
        HANDLE(station.0),
        UOI_FLAGS,
        Some(ptr::from_mut(&mut flags).cast()),
        mem::size_of::<USEROBJECTFLAGS>() as u32,
        Some(&mut dummy),
    )
    .is_err()
    {
        return false;
    }
    return (flags.dwFlags & (WSF_VISIBLE as u32)) != 0;
}

pub unsafe fn try_load_from_self_or_hip_with_message(
    redirect_names: &[&'static str],
) -> Option<HMODULE> {
    let result = redirect_names.iter().copied().find_map(|redirect_name| {
        try_load_from_self_dir(redirect_name).or_else(|| try_load_from_hip_path(redirect_name))
    });
    if result.is_none() && is_interactive_session() {
        let mut title = U16String::from_str("ZLUDA failed to load ");
        title.push_str(redirect_names[0]);
        redirect_names.iter().copied().skip(1).for_each(|name| {
            title.push_str(" or ");
            title.push_str(name);
        });
        title.push_char(0 as char);
        let config = TASKDIALOGCONFIG {
            cbSize: mem::size_of::<TASKDIALOGCONFIG>() as u32,
            hwndParent: HWND(ptr::null_mut()),
            hInstance: HINSTANCE(ptr::null_mut()),
            dwFlags: TDF_ENABLE_HYPERLINKS,
            dwCommonButtons: TDCBF_CLOSE_BUTTON,
            pszWindowTitle: w!("ZLUDA error"),
            Anonymous1: TASKDIALOGCONFIG_0 {
                pszMainIcon: TD_ERROR_ICON,
            },
            pszMainInstruction: PCWSTR(title.as_ptr()),
            pszContent: w!("<A HREF=\"https://zluda.readthedocs.io/latest/hip_sdk.html\">Click here to learn how to set up the HIP SDK</A>"),
            cButtons: 0,
            pButtons: ptr::null_mut(),
            nDefaultButton: IDCLOSE.0,
            cRadioButtons: 0,
            pRadioButtons: ptr::null_mut(),
            nDefaultRadioButton: 0,
            pszVerificationText: PCWSTR::null(),
            pszExpandedInformation: PCWSTR::null(),
            pszExpandedControlText: PCWSTR::null(),
            pszCollapsedControlText: PCWSTR::null(),
            Anonymous2: TASKDIALOGCONFIG_1::default(),
            pszFooter: PCWSTR::null(),
            pfCallback: Some(task_dialog_callback),
            lpCallbackData: 0,
            cxWidth: 0,
        };
        TaskDialogIndirect(&config, None, None, None).ok();
    }
    result
}

unsafe extern "system" fn task_dialog_callback(
    _hwnd: HWND,
    msg: TASKDIALOG_NOTIFICATIONS,
    _wparam: WPARAM,
    lparam: LPARAM,
    _lprefdata: isize,
) -> HRESULT {
    if msg != TDN_HYPERLINK_CLICKED {
        return S_OK;
    }
    windows::Win32::UI::Shell::ShellExecuteW(
        None,
        w!("open"),
        PCWSTR::from_raw(lparam.0 as *const u16),
        None,
        None,
        windows::Win32::UI::WindowsAndMessaging::SW_SHOW,
    );
    S_OK
}

pub unsafe fn delay_load_check(
    (redirect_name, dli_notify_expected): (&'static str, DliNotify),
    dli_notify: u32,
    pdli: *const DelayLoadInfo,
) -> Option<()> {
    if dli_notify != dli_notify_expected as u32 {
        return None;
    }
    let pdli = pdli.as_ref()?;
    let name = CStr::from_ptr(pdli.sz_dll);
    let dll_name = name.to_str().ok()?;
    if dll_name.eq_ignore_ascii_case(redirect_name) {
        Some(())
    } else {
        None
    }
}

pub unsafe fn try_load_from_self_dir(libname: &str) -> Option<HMODULE> {
    let path = get_module_path_for_function(try_load_from_self_dir as *const () as usize)?;
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

pub fn get_module_path_for_function(fn_: usize) -> Option<OsString> {
    let mut hm = HMODULE::default();
    unsafe {
        GetModuleHandleExW(
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
            PCWSTR(fn_ as _),
            &mut hm,
        )
    }
    .ok()?;
    Some(get_module_path_utf16(hm))
}

pub fn get_module_path_utf16(hm: HMODULE) -> OsString {
    let mut buffer = vec![0u16; windows::Win32::Foundation::MAX_PATH as usize];
    let mut copied;
    loop {
        copied = unsafe { GetModuleFileNameW(Some(hm), &mut buffer) };
        if (copied as usize) < buffer.len() {
            break;
        } else {
            buffer.resize(buffer.len() * 2, 0);
        }
    }
    buffer.truncate(copied as usize);
    OsString::from_wide(&buffer)
}

pub fn kill_child_on_process_exit(child: *mut c_void) -> windows::core::Result<()> {
    let job_handle = unsafe { windows::Win32::System::JobObjects::CreateJobObjectA(None, None) }?;
    let mut info = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
    info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
    unsafe {
        SetInformationJobObject(
            job_handle,
            JobObjectExtendedLimitInformation,
            &mut info as *mut _ as *mut _,
            size_of_val(&info) as u32,
        )
    }?;
    unsafe { AssignProcessToJobObject(job_handle, HANDLE(child)) }?;
    Ok(())
}

mod parser {
    use widestring::{u16str, U16Str};
    use winnow::Result;
    use winnow::{
        combinator::*,
        prelude::*,
        stream::{Stream, StreamIsPartial, TokenSlice},
        token::take_while,
    };

    fn is_dec_digit(c: &u16) -> bool {
        u8::try_from(u16::from(*c))
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false)
    }

    fn numeric<'a, S: Stream<Token = &'a u16> + StreamIsPartial>(input: &mut S) -> Result<(), ()> {
        take_while(1.., is_dec_digit).void().parse_next(input)
    }

    fn to_ascii_lowercase(x: u16) -> Option<u16> {
        u8::try_from(x).ok().map(|c| c.to_ascii_lowercase() as u16)
    }

    fn literal<'a, S: Stream<Token = &'a u16> + StreamIsPartial>(
        value: &'static U16Str,
    ) -> impl Parser<S, S::Slice, ()> {
        move |stream: &mut S| {
            let start = stream.checkpoint();

            let mut iter = value.as_slice().iter().copied();
            while let Some(c1) = iter.next() {
                if let Some(c2) = stream.next_token().copied() {
                    if let Some(c1) = to_ascii_lowercase(c1) {
                        if c1 == to_ascii_lowercase(c2).unwrap_or(c2) {
                            continue;
                        }
                    }
                }
                stream.reset(&start);
                return Err(());
            }
            Ok(stream.next_slice(0))
        }
    }

    pub fn is_version_dll(s: &[u16]) -> bool {
        let stream = TokenSlice::new(s);
        (
            numeric,
            opt((literal(u16str!("_")), numeric)),
            literal(u16str!(".dll")),
            eof,
        )
            .parse(stream)
            .is_ok()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_is_version_dll() {
            assert!(is_version_dll(&u16str!("1.dll").as_slice()));
            assert!(is_version_dll(&u16str!("1_2.dll").as_slice()));
            assert!(!is_version_dll(&u16str!("1_2.txt").as_slice()));
            assert!(!is_version_dll(&u16str!("abc.dll").as_slice()));
        }

        #[test]
        fn test_is_version_dll_long_numeric() {
            assert!(is_version_dll(
                &u16str!("123456789012345678901234567890.dll").as_slice()
            ));
            assert!(is_version_dll(
                &u16str!("123456789012345678901234567890_123456789012345678901234567890.dll")
                    .as_slice()
            ));
        }

        #[test]
        fn test_is_version_dll_uppercase() {
            assert!(is_version_dll(&u16str!("11.DLL").as_slice()));
            assert!(is_version_dll(&u16str!("11.Dll").as_slice()));
            assert!(is_version_dll(&u16str!("11_12.DLL").as_slice()));
        }

        #[test]
        fn test_is_version_dll_multiple_numerics() {
            assert!(is_version_dll(&u16str!("11_12.dll").as_slice()));
            assert!(!is_version_dll(&u16str!("1_2_3.dll").as_slice()));
            assert!(!is_version_dll(&u16str!("1__2.dll").as_slice()));
            assert!(!is_version_dll(&u16str!("_1.dll").as_slice()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_look_up_from_full_path() {
        let lookup = DllLookup::new64();
        let path = u16str!(r#"C:\Windows\System32\nvml.dll"#);
        assert_eq!(
            u16str!("nvml.dll"),
            LIBRARIES64[lookup.lookup(path.as_slice()).unwrap()]
                .zluda_name
                .unwrap()
        );
    }

    #[test]
    fn can_look_up_from_just_file_name() {
        let lookup = DllLookup::new64();
        let path = u16str!(r#"cusparse64_11.dll"#);
        assert_eq!(
            u16str!("cusparse.dll"),
            LIBRARIES64[lookup.lookup(path.as_slice()).unwrap()]
                .zluda_name
                .unwrap()
        );
    }

    #[test]
    fn can_look_up_utf16() {
        let lookup = DllLookup::new64();
        let path = u16str!("\\cublaslt64_13.dll");
        assert_eq!(
            u16str!("cublaslt.dll"),
            LIBRARIES64[lookup.lookup(path.as_slice()).unwrap()]
                .zluda_name
                .unwrap()
        );
    }

    #[test]
    fn can_look_up_utf16_case_insensitive() {
        let lookup = DllLookup::new64();
        let path = u16str!("\\CUBLASLT64_13.dll");
        assert_eq!(
            u16str!("cublaslt.dll"),
            LIBRARIES64[lookup.lookup(path.as_slice()).unwrap()]
                .zluda_name
                .unwrap()
        );
    }

    #[test]
    fn fails_on_mismatch() {
        let lookup = DllLookup::new64();
        let path = u16str!("cusparse64_99.dllb");
        assert!(lookup.lookup(path.as_slice()).is_none());
    }

    #[test]
    fn does_not_explode_on_utf16() {
        let lookup = DllLookup::new64();
        let path = u16str!("🇧🇴");
        assert_eq!(path.chars().count(), 2);
        assert_eq!(path.as_slice().len(), 4);
        assert!(lookup.lookup(path.as_slice()).is_none());
    }
}
