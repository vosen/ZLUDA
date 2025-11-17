#![cfg(target_os = "windows")]

use detours_sys::{
    DetourAttach, DetourDetach, DetourRestoreAfterWith, DetourTransactionAbort,
    DetourTransactionBegin, DetourTransactionCommit, DetourUpdateThread, LPCWSTR,
};
use std::ffi::CStr;
use std::{ffi::c_void, mem, ptr, slice, usize};
use widestring::{U16CStr, U16CString};
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Thread32First, Thread32Next, TH32CS_SNAPTHREAD, THREADENTRY32,
};
use windows::Win32::System::Threading::{
    GetCurrentProcessId, GetCurrentThread, GetCurrentThreadId, OpenThread, ResumeThread,
    SuspendThread, THREAD_SUSPEND_RESUME,
};
use windows_sys::core::{BOOL, PCSTR, PCWSTR, PSTR, PWSTR};
use windows_sys::Win32::Foundation::{FARPROC, NO_ERROR};
use windows_sys::Win32::Security::SECURITY_ATTRIBUTES;
use windows_sys::Win32::System::LibraryLoader::{
    GetProcAddress, LoadLibraryExA, LoadLibraryExW, LOAD_LIBRARY_FLAGS,
};
use windows_sys::Win32::System::Threading::{
    CreateProcessA, CreateProcessAsUserW, CreateProcessW, CreateProcessWithLogonW,
    CreateProcessWithTokenW,
};
use windows_sys::Win32::{
    Foundation::HMODULE,
    System::LibraryLoader::{LoadLibraryA, LoadLibraryW},
};
use zluda_windows::DllLookup;

static mut DETOUR_DROP: Option<DetourDetachGuard> = None;
static mut DETOUR_PATHS: Option<DetourPaths> = None;

struct DetourPaths {
    lookup: DllLookup,
    override_paths: [Option<(&'static CStr, Vec<u16>)>; zluda_windows::LIBRARIES.len()],
}

impl DetourPaths {
    fn new() -> Self {
        let lookup = DllLookup::new();
        let paths = zluda_windows::LIBRARIES.each_ref().map(|lib| {
            get_payload(unsafe { mem::transmute(&lib.guid) }).map(|payload| {
                let utf8 = unsafe { CStr::from_bytes_with_nul_unchecked(payload) };
                let utf16 =
                    unsafe { U16CString::from_str_unchecked(utf8.to_string_lossy()) }.into_vec();
                (utf8, utf16)
            })
        });
        DetourPaths {
            lookup,
            override_paths: paths,
        }
    }

    fn ascii_override(this: &Option<Self>, path: *const u8) -> *const u8 {
        Self::override_impl(
            this,
            path,
            |p| {
                let cstr = unsafe { CStr::from_ptr(p.cast()) };
                cstr.to_bytes()
            },
            |lookup, buffer| lookup.lookup_ascii(buffer),
            |(override_ascii, _)| override_ascii.as_ptr().cast(),
        )
    }

    fn utf16_override(this: &Option<Self>, path: *const u16) -> *const u16 {
        Self::override_impl(
            this,
            path,
            |p| {
                let u16cstr = unsafe { U16CStr::from_ptr_str(p) };
                u16cstr.as_slice()
            },
            |lookup, buffer| lookup.lookup_utf16(buffer),
            |(_, override_utf16)| override_utf16.as_ptr(),
        )
    }

    fn override_impl<T: 'static>(
        this: &Option<Self>,
        path: *const T,
        get_buffer: impl FnOnce(*const T) -> &'static [T],
        lookup: impl FnOnce(&DllLookup, &[T]) -> Option<usize>,
        get_pointer: impl FnOnce(&(&'static CStr, Vec<u16>)) -> *const T,
    ) -> *const T {
        this.as_ref()
            .map(|this| {
                let buffer = get_buffer(path);
                let index = lookup(&this.lookup, buffer);
                index
                    .map(|index| this.override_paths[index].as_ref().map(|p| get_pointer(p)))
                    .flatten()
            })
            .flatten()
            .unwrap_or(path)
    }
}

static mut LOAD_LIBRARY_A: unsafe extern "system" fn(lp_lib_file_name: PCSTR) -> HMODULE =
    LoadLibraryA;

static mut LOAD_LIBRARY_W: unsafe extern "system" fn(lp_lib_file_name: PCWSTR) -> HMODULE =
    LoadLibraryW;

static mut LOAD_LIBRARY_EX_A: unsafe extern "system" fn(
    lp_lib_file_name: PCSTR,
    file: windows_sys::Win32::Foundation::HANDLE,
    flags: LOAD_LIBRARY_FLAGS,
) -> HMODULE = LoadLibraryExA;

static mut LOAD_LIBRARY_EX_W: unsafe extern "system" fn(
    lp_lib_file_name: LPCWSTR,
    file: windows_sys::Win32::Foundation::HANDLE,
    flags: LOAD_LIBRARY_FLAGS,
) -> HMODULE = LoadLibraryExW;

static mut CREATE_PROCESS_A: unsafe extern "system" fn(
    lpapplicationname: PCSTR,
    lpcommandline: PSTR,
    lpprocessattributes: *const SECURITY_ATTRIBUTES,
    lpthreadattributes: *const SECURITY_ATTRIBUTES,
    binherithandles: BOOL,
    dwcreationflags: windows_sys::Win32::System::Threading::PROCESS_CREATION_FLAGS,
    lpenvironment: *const c_void,
    lpcurrentdirectory: PCSTR,
    lpstartupinfo: *const windows_sys::Win32::System::Threading::STARTUPINFOA,
    lpprocessinformation: *mut windows_sys::Win32::System::Threading::PROCESS_INFORMATION,
) -> BOOL = CreateProcessA;

static mut CREATE_PROCESS_W: unsafe extern "system" fn(
    lpapplicationname: PCWSTR,
    lpcommandline: PWSTR,
    lpprocessattributes: *const SECURITY_ATTRIBUTES,
    lpthreadattributes: *const SECURITY_ATTRIBUTES,
    binherithandles: BOOL,
    dwcreationflags: windows_sys::Win32::System::Threading::PROCESS_CREATION_FLAGS,
    lpenvironment: *const c_void,
    lpcurrentdirectory: PCWSTR,
    lpstartupinfo: *const windows_sys::Win32::System::Threading::STARTUPINFOW,
    lpprocessinformation: *mut windows_sys::Win32::System::Threading::PROCESS_INFORMATION,
) -> BOOL = CreateProcessW;

static mut CREATE_PROCESS_AS_USER_W: unsafe extern "system" fn(
    htoken: windows_sys::Win32::Foundation::HANDLE,
    lpapplicationname: PCWSTR,
    lpcommandline: PWSTR,
    lpprocessattributes: *const SECURITY_ATTRIBUTES,
    lpthreadattributes: *const SECURITY_ATTRIBUTES,
    binherithandles: BOOL,
    dwcreationflags: windows_sys::Win32::System::Threading::PROCESS_CREATION_FLAGS,
    lpenvironment: *const c_void,
    lpcurrentdirectory: PCWSTR,
    lpstartupinfo: *const windows_sys::Win32::System::Threading::STARTUPINFOW,
    lpprocessinformation: *mut windows_sys::Win32::System::Threading::PROCESS_INFORMATION,
) -> BOOL = CreateProcessAsUserW;

static mut CREATE_PROCESS_WITH_TOKEN_W: unsafe extern "system" fn(
    htoken: windows_sys::Win32::Foundation::HANDLE,
    dwlogonflags: windows_sys::Win32::System::Threading::CREATE_PROCESS_LOGON_FLAGS,
    lpapplicationname: PCWSTR,
    lpcommandline: PWSTR,
    dwcreationflags: windows_sys::Win32::System::Threading::PROCESS_CREATION_FLAGS,
    lpenvironment: *const c_void,
    lpcurrentdirectory: PCWSTR,
    lpstartupinfo: *const windows_sys::Win32::System::Threading::STARTUPINFOW,
    lpprocessinformation: *mut windows_sys::Win32::System::Threading::PROCESS_INFORMATION,
) -> BOOL = CreateProcessWithTokenW;

static mut CREATE_PROCESS_WITH_LOGON_W: unsafe extern "system" fn(
    lpusername: PCWSTR,
    lpdomain: PCWSTR,
    lppassword: PCWSTR,
    dwlogonflags: windows_sys::Win32::System::Threading::CREATE_PROCESS_LOGON_FLAGS,
    lpapplicationname: PCWSTR,
    lpcommandline: PWSTR,
    dwcreationflags: windows_sys::Win32::System::Threading::PROCESS_CREATION_FLAGS,
    lpenvironment: *const c_void,
    lpcurrentdirectory: PCWSTR,
    lpstartupinfo: *const windows_sys::Win32::System::Threading::STARTUPINFOW,
    lpprocessinformation: *mut windows_sys::Win32::System::Threading::PROCESS_INFORMATION,
) -> BOOL = CreateProcessWithLogonW;

#[no_mangle]
#[allow(non_snake_case)]
unsafe extern "system" fn ZludaGetProcAddress_NoRedirect(
    hmodule: HMODULE,
    lpprocname: PCSTR,
) -> FARPROC {
    GetProcAddress(hmodule, lpprocname)
}

#[no_mangle]
#[allow(non_snake_case)]
unsafe extern "system" fn ZludaLoadLibraryW_NoRedirect(lpLibFileName: LPCWSTR) -> HMODULE {
    (LOAD_LIBRARY_W)(lpLibFileName)
}

#[allow(non_snake_case)]
unsafe extern "system" fn ZludaLoadLibraryA(file_name: PCSTR) -> HMODULE {
    LOAD_LIBRARY_A(DetourPaths::ascii_override(
        &*&raw const DETOUR_PATHS,
        file_name,
    ))
}

#[allow(non_snake_case)]
unsafe extern "system" fn ZludaLoadLibraryW(file_name: LPCWSTR) -> HMODULE {
    LOAD_LIBRARY_W(DetourPaths::utf16_override(
        &*&raw const DETOUR_PATHS,
        file_name,
    ))
}

#[allow(non_snake_case)]
unsafe extern "system" fn ZludaLoadLibraryExA(
    file_name: PCSTR,
    hfile: windows_sys::Win32::Foundation::HANDLE,
    dwflags: LOAD_LIBRARY_FLAGS,
) -> HMODULE {
    LOAD_LIBRARY_EX_A(
        DetourPaths::ascii_override(&*&raw const DETOUR_PATHS, file_name),
        hfile,
        dwflags,
    )
}

#[allow(non_snake_case)]
unsafe extern "system" fn ZludaLoadLibraryExW(
    file_name: PCWSTR,
    hfile: windows_sys::Win32::Foundation::HANDLE,
    dwflags: LOAD_LIBRARY_FLAGS,
) -> HMODULE {
    LOAD_LIBRARY_EX_W(
        DetourPaths::utf16_override(&*&raw const DETOUR_PATHS, file_name),
        hfile,
        dwflags,
    )
}

#[allow(non_snake_case)]
unsafe extern "system" fn ZludaCreateProcessA(
    lpapplicationname: PCSTR,
    lpcommandline: PSTR,
    lpprocessattributes: *const SECURITY_ATTRIBUTES,
    lpthreadattributes: *const SECURITY_ATTRIBUTES,
    binherithandles: BOOL,
    dwcreationflags: windows_sys::Win32::System::Threading::PROCESS_CREATION_FLAGS,
    lpenvironment: *const c_void,
    lpcurrentdirectory: PCSTR,
    lpstartupinfo: *const windows_sys::Win32::System::Threading::STARTUPINFOA,
    lpprocessinformation: *mut windows_sys::Win32::System::Threading::PROCESS_INFORMATION,
) -> BOOL {
    CREATE_PROCESS_A(
        lpapplicationname,
        lpcommandline,
        lpprocessattributes,
        lpthreadattributes,
        binherithandles,
        dwcreationflags,
        lpenvironment,
        lpcurrentdirectory,
        lpstartupinfo,
        lpprocessinformation,
    )
}

#[allow(non_snake_case)]
unsafe extern "system" fn ZludaCreateProcessW(
    lpapplicationname: PCWSTR,
    lpcommandline: PWSTR,
    lpprocessattributes: *const SECURITY_ATTRIBUTES,
    lpthreadattributes: *const SECURITY_ATTRIBUTES,
    binherithandles: BOOL,
    dwcreationflags: windows_sys::Win32::System::Threading::PROCESS_CREATION_FLAGS,
    lpenvironment: *const c_void,
    lpcurrentdirectory: PCWSTR,
    lpstartupinfo: *const windows_sys::Win32::System::Threading::STARTUPINFOW,
    lpprocessinformation: *mut windows_sys::Win32::System::Threading::PROCESS_INFORMATION,
) -> BOOL {
    CREATE_PROCESS_W(
        lpapplicationname,
        lpcommandline,
        lpprocessattributes,
        lpthreadattributes,
        binherithandles,
        dwcreationflags,
        lpenvironment,
        lpcurrentdirectory,
        lpstartupinfo,
        lpprocessinformation,
    )
}

#[allow(non_snake_case)]
unsafe extern "system" fn ZludaCreateProcessAsUserW(
    htoken: windows_sys::Win32::Foundation::HANDLE,
    lpapplicationname: PCWSTR,
    lpcommandline: PWSTR,
    lpprocessattributes: *const SECURITY_ATTRIBUTES,
    lpthreadattributes: *const SECURITY_ATTRIBUTES,
    binherithandles: BOOL,
    dwcreationflags: windows_sys::Win32::System::Threading::PROCESS_CREATION_FLAGS,
    lpenvironment: *const c_void,
    lpcurrentdirectory: PCWSTR,
    lpstartupinfo: *const windows_sys::Win32::System::Threading::STARTUPINFOW,
    lpprocessinformation: *mut windows_sys::Win32::System::Threading::PROCESS_INFORMATION,
) -> BOOL {
    CREATE_PROCESS_AS_USER_W(
        htoken,
        lpapplicationname,
        lpcommandline,
        lpprocessattributes,
        lpthreadattributes,
        binherithandles,
        dwcreationflags,
        lpenvironment,
        lpcurrentdirectory,
        lpstartupinfo,
        lpprocessinformation,
    )
}

#[allow(non_snake_case)]
unsafe extern "system" fn ZludaCreateProcessWithLogonW(
    lpusername: PCWSTR,
    lpdomain: PCWSTR,
    lppassword: PCWSTR,
    dwlogonflags: windows_sys::Win32::System::Threading::CREATE_PROCESS_LOGON_FLAGS,
    lpapplicationname: PCWSTR,
    lpcommandline: PWSTR,
    dwcreationflags: windows_sys::Win32::System::Threading::PROCESS_CREATION_FLAGS,
    lpenvironment: *const c_void,
    lpcurrentdirectory: PCWSTR,
    lpstartupinfo: *const windows_sys::Win32::System::Threading::STARTUPINFOW,
    lpprocessinformation: *mut windows_sys::Win32::System::Threading::PROCESS_INFORMATION,
) -> BOOL {
    CREATE_PROCESS_WITH_LOGON_W(
        lpusername,
        lpdomain,
        lppassword,
        dwlogonflags,
        lpapplicationname,
        lpcommandline,
        dwcreationflags,
        lpenvironment,
        lpcurrentdirectory,
        lpstartupinfo,
        lpprocessinformation,
    )
}

#[allow(non_snake_case)]
unsafe extern "system" fn ZludaCreateProcessWithTokenW(
    htoken: windows_sys::Win32::Foundation::HANDLE,
    dwlogonflags: windows_sys::Win32::System::Threading::CREATE_PROCESS_LOGON_FLAGS,
    lpapplicationname: PCWSTR,
    lpcommandline: PWSTR,
    dwcreationflags: windows_sys::Win32::System::Threading::PROCESS_CREATION_FLAGS,
    lpenvironment: *const c_void,
    lpcurrentdirectory: PCWSTR,
    lpstartupinfo: *const windows_sys::Win32::System::Threading::STARTUPINFOW,
    lpprocessinformation: *mut windows_sys::Win32::System::Threading::PROCESS_INFORMATION,
) -> BOOL {
    CREATE_PROCESS_WITH_TOKEN_W(
        htoken,
        dwlogonflags,
        lpapplicationname,
        lpcommandline,
        dwcreationflags,
        lpenvironment,
        lpcurrentdirectory,
        lpstartupinfo,
        lpprocessinformation,
    )
}

// This type encapsulates typical calling sequence of detours and cleanup.
// We have two ways we do detours:
// * If we are loaded before nvcuda.dll, we hook LoadLibrary*
// * If we are loaded after nvcuda.dll, we override every cu* function
// Additionally, within both of those we attach to CreateProcess*
struct DetourDetachGuard {
    state: DetourUndoState,
    suspended_threads: Vec<HANDLE>,
    // First element is the original fn, second is the new fn
    overriden_non_cuda_fns: Vec<(*mut *mut c_void, *mut c_void)>,
}

impl DetourDetachGuard {
    // First element in the pair is ptr to original fn, second argument is the
    // new function. We accept *mut *mut c_void instead of *mut c_void as the
    // first element in the pair, because somehow otherwise original functions
    // also get overriden, so for example ZludaLoadLibraryExW ends calling
    // itself recursively until stack overflow exception occurs
    unsafe fn new<'a>() -> Option<Self> {
        let mut result = DetourDetachGuard {
            state: DetourUndoState::DoNothing,
            suspended_threads: Vec::new(),
            overriden_non_cuda_fns: Vec::new(),
        };
        if DetourTransactionBegin() != NO_ERROR as i32 {
            return None;
        }
        result.state = DetourUndoState::AbortTransactionResumeThreads;
        if !Self::suspend_all_threads_except_current(&mut result.suspended_threads) {
            return None;
        }
        for thread_handle in result.suspended_threads.iter().copied() {
            if DetourUpdateThread(thread_handle.0) != NO_ERROR as i32 {
                return None;
            }
        }
        result.overriden_non_cuda_fns.extend_from_slice(&[
            (
                &raw mut LOAD_LIBRARY_A as *mut _ as *mut *mut c_void,
                ZludaLoadLibraryA as *mut c_void,
            ),
            (
                &raw mut LOAD_LIBRARY_W as *mut _ as _,
                ZludaLoadLibraryW as _,
            ),
            (
                &raw mut LOAD_LIBRARY_EX_A as *mut _ as _,
                ZludaLoadLibraryExA as _,
            ),
            (
                &raw mut LOAD_LIBRARY_EX_W as *mut _ as _,
                ZludaLoadLibraryExW as _,
            ),
            (
                &raw mut CREATE_PROCESS_A as *mut _ as _,
                ZludaCreateProcessA as _,
            ),
            (
                &raw mut CREATE_PROCESS_W as *mut _ as _,
                ZludaCreateProcessW as _,
            ),
            (
                &raw mut CREATE_PROCESS_AS_USER_W as *mut _ as _,
                ZludaCreateProcessAsUserW as _,
            ),
            (
                &raw mut CREATE_PROCESS_WITH_LOGON_W as *mut _ as _,
                ZludaCreateProcessWithLogonW as _,
            ),
            (
                &raw mut CREATE_PROCESS_WITH_TOKEN_W as *mut _ as _,
                ZludaCreateProcessWithTokenW as _,
            ),
        ]);
        for (original_fn, new_fn) in result.overriden_non_cuda_fns.iter().copied() {
            if DetourAttach(original_fn, new_fn) != NO_ERROR as i32 {
                return None;
            }
        }
        if DetourTransactionCommit() != NO_ERROR as i32 {
            return None;
        }
        result.state = DetourUndoState::DoNothing;
        // HACK ALERT
        // I really have no idea how this could happen.
        // Perhaps a thread was closed?
        if !result.resume_threads() {
            if cfg!(debug_assertions) {
                panic!();
            }
        }
        result.state = DetourUndoState::DetachDetours;
        Some(result)
    }

    unsafe fn suspend_all_threads_except_current(threads: &mut Vec<HANDLE>) -> bool {
        let thread_snapshot = unwrap_or::unwrap_ok_or!(
            CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0),
            _,
            return false
        );
        let current_thread = GetCurrentThreadId();
        let current_process = GetCurrentProcessId();
        let mut thread = THREADENTRY32::default();
        thread.dwSize = mem::size_of::<THREADENTRY32>() as u32;
        if Thread32First(thread_snapshot, &mut thread).is_err() {
            CloseHandle(thread_snapshot).ok();
            return false;
        }
        loop {
            if thread.th32OwnerProcessID == current_process && thread.th32ThreadID != current_thread
            {
                let thread_handle = unwrap_or::unwrap_ok_or!(
                    OpenThread(THREAD_SUSPEND_RESUME, false, thread.th32ThreadID),
                    _,
                    {
                        CloseHandle(thread_snapshot).ok();
                        return false;
                    }
                );
                if SuspendThread(thread_handle) == (-1i32 as u32) {
                    CloseHandle(thread_handle).ok();
                    CloseHandle(thread_snapshot).ok();
                    return false;
                }
                threads.push(thread_handle);
            }
            if Thread32Next(thread_snapshot, &mut thread).is_err() {
                break;
            }
        }
        CloseHandle(thread_snapshot).ok();
        true
    }

    // returns true on success
    unsafe fn resume_threads(&self) -> bool {
        let mut success = true;
        for t in self.suspended_threads.iter().copied() {
            if ResumeThread(t) == -1i32 as u32 {
                success = false;
            }
            if CloseHandle(t).is_err() {
                success = false;
            }
        }
        success
    }
}

impl Drop for DetourDetachGuard {
    fn drop(&mut self) {
        match self.state {
            DetourUndoState::DoNothing => {}
            DetourUndoState::AbortTransactionResumeThreads => {
                unsafe { DetourTransactionAbort() };
                unsafe { self.resume_threads() };
            }
            DetourUndoState::DetachDetours => unsafe {
                DetourTransactionBegin();
                DetourUpdateThread(GetCurrentThread().0);
                for (original_fn, new_fn) in self.overriden_non_cuda_fns.iter().copied() {
                    DetourDetach(original_fn, new_fn);
                }
                DetourTransactionCommit();
            },
        }
    }
}

// Along with Drop impl this forms a state machine for undoing detours.
// I would like to model this as a an usual full state machine with fields in
// variants, but you can't move fields out of type that implements Drop
enum DetourUndoState {
    DoNothing,
    AbortTransactionResumeThreads,
    DetachDetours,
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "system" fn DllMain(_inst_dll: *mut c_void, dwReason: u32, _: *const u8) -> i32 {
    use windows_sys::Win32::Foundation::{FALSE, TRUE};
    use windows_sys::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};
    if dwReason == DLL_PROCESS_ATTACH {
        if DetourRestoreAfterWith() == FALSE {
            return FALSE;
        }
        match DetourDetachGuard::new() {
            Some(g) => {
                DETOUR_DROP = Some(g);
                DETOUR_PATHS = Some(DetourPaths::new());
                TRUE
            }
            None => FALSE,
        }
    } else if dwReason == DLL_PROCESS_DETACH {
        DETOUR_PATHS = None;
        match (&mut *&raw mut DETOUR_DROP).take() {
            Some(_) => TRUE,
            None => FALSE,
        }
    } else {
        TRUE
    }
}

fn get_payload(guid: &detours_sys::GUID) -> Option<&'static [u8]> {
    let mut size = 0;
    let payload_ptr = unsafe { detours_sys::DetourFindPayloadEx(guid, &mut size) };
    if payload_ptr != ptr::null_mut() {
        Some(unsafe { slice::from_raw_parts(payload_ptr as *const _, size as usize) })
    } else {
        None
    }
}
