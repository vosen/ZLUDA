#![cfg(target_os = "windows")]

use detours_sys::{
    DetourAttach, DetourCopyPayloadToProcess, DetourDetach, DetourIsHelperProcess,
    DetourProcessViaHelperDllsW, DetourRestoreAfterWith, DetourTransactionAbort,
    DetourTransactionBegin, DetourTransactionCommit, DetourUpdateProcessWithDll,
    DetourUpdateThread, LPCWSTR,
};
use ntapi::ntapi_base::CLIENT_ID;
use ntapi::ntpsapi::{NtResumeThread, THREAD_CREATE_FLAGS_CREATE_SUSPENDED};
use ntapi::ntrtl::{RtlCreateUnicodeString, RtlFreeUnicodeString};
use rustc_hash::FxHashMap;
use std::collections::hash_map;
use std::ffi::CStr;
use std::sync::{Mutex, OnceLock};
use std::{ffi::c_void, mem, ptr, slice, usize};
use windows::Win32::Foundation::{CloseHandle, HANDLE, NTSTATUS, UNICODE_STRING};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Thread32First, Thread32Next, TH32CS_SNAPTHREAD, THREADENTRY32,
};
use windows::Win32::System::Threading::{
    GetCurrentProcessId, GetCurrentThread, GetCurrentThreadId, GetProcessId, OpenThread,
    ResumeThread, SuspendThread, THREAD_SUSPEND_RESUME,
};
use windows_sys::Win32::Foundation::HMODULE;
use windows_sys::Win32::Foundation::NO_ERROR;
use windows_sys::Win32::System::LibraryLoader::GetProcAddress;

static mut DETOUR_DROP: Option<DetourDetachGuard> = None;
static mut DETOUR_PATHS: Option<DetourPaths> = None;

#[cfg(target_pointer_width = "32")]
static LIBRARIES: &[zluda_windows::LibraryInfo; 2] = &zluda_windows::LIBRARIES32;
#[cfg(target_pointer_width = "64")]
static LIBRARIES: &[zluda_windows::LibraryInfo; 10] = &zluda_windows::LIBRARIES64;

type DllLookup = zluda_windows::DllLookup<{ LIBRARIES.len() }>;

struct DetourPaths {
    lookup: DllLookup,
    override_paths: [Option<&'static [u16]>; LIBRARIES.len()],
    redirect_dll: &'static [u8],
    payload: &'static [u8],
}

impl DetourPaths {
    fn new() -> Option<Self> {
        let lookup = DllLookup::new(LIBRARIES);
        let payload = get_payload(&unsafe { mem::transmute(zluda_windows::detours::GUID) })?;
        let all_override_paths = zluda_windows::detours::unpack_paths(payload)?;
        #[cfg(target_pointer_width = "32")]
        let override_paths = all_override_paths.0;
        #[cfg(target_pointer_width = "64")]
        let override_paths = all_override_paths.1;
        Some(DetourPaths {
            lookup,
            override_paths,
            redirect_dll: all_override_paths.2,
            payload,
        })
    }

    fn get_override(this: &Option<Self>, path: &[u16]) -> Option<PathOverride> {
        let this = this.as_ref()?;
        if path.is_empty() {
            return None;
        }
        let override_paths = &this.override_paths;
        let library_index = this.lookup.lookup(path)?;
        Some(
            if let Some(_) = LIBRARIES[library_index].redirect.as_ref() {
                PathOverride::Library(override_paths[library_index]?)
            } else {
                PathOverride::Nvrtc
            },
        )
    }
}

enum PathOverride {
    Library(&'static [u16]),
    Nvrtc,
}

static mut LDR_LOAD_DLL: ntapi::ntldr::LdrLoadDll = ntapi::ntldr::LdrLoadDll;

static mut NT_CREATE_USER_PROCESS: ntapi::ntpsapi::NtCreateUserProcess =
    ntapi::ntpsapi::NtCreateUserProcess;

struct UnicodeString(winapi::shared::ntdef::UNICODE_STRING);

impl UnicodeString {
    fn new(string: LPCWSTR) -> Option<Self> {
        let mut unicode_string = unsafe { mem::zeroed() };
        if unsafe { RtlCreateUnicodeString(&mut unicode_string, string) } == 0 {
            return None;
        }
        Some(Self(unicode_string))
    }
}

impl Drop for UnicodeString {
    fn drop(&mut self) {
        unsafe {
            RtlFreeUnicodeString(&mut self.0);
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
unsafe extern "system" fn ZludaLoadLibraryW_NoRedirect(lpLibFileName: LPCWSTR) -> HMODULE {
    let mut file_name = if let Some(file_name) = UnicodeString::new(lpLibFileName) {
        file_name
    } else {
        return ptr::null_mut();
    };
    let mut handle = ptr::null_mut();
    let result = (LDR_LOAD_DLL)(
        ptr::null_mut(),
        ptr::null_mut(),
        &mut file_name.0,
        &mut handle,
    );
    if result == 0 {
        return std::ptr::null_mut();
    }
    handle.cast()
}

#[allow(non_snake_case)]
unsafe extern "system" fn ZludaLdrLoadDll(
    mut dll_path_arg: LPCWSTR,
    dll_characteristics: *mut u32,
    mut dll_name: *const UNICODE_STRING,
    dll_handle: *mut detours_sys::PVOID,
) -> NTSTATUS {
    let dll_path_slice =
        std::slice::from_raw_parts::<u16>((*dll_name).Buffer.0, (*dll_name).Length as usize / 2);
    #[allow(static_mut_refs)]
    let override_ = DetourPaths::get_override(&DETOUR_PATHS, dll_path_slice);
    let dll_name_override: Option<UnicodeString>;
    if let Some(PathOverride::Library(path)) = override_ {
        dll_name_override = UnicodeString::new(path.as_ptr());
        if let Some(ref dll_name_override) = dll_name_override {
            dll_path_arg = ptr::null();
            dll_name = ptr::from_ref(&dll_name_override.0).cast();
        }
    }
    let result = NTSTATUS((LDR_LOAD_DLL)(
        dll_path_arg.cast_mut(),
        dll_characteristics,
        dll_name.cast_mut().cast(),
        dll_handle.cast(),
    ));
    if matches!(override_, Some(PathOverride::Nvrtc)) && result.is_ok() {
        detour_nvrtc(*dll_handle);
    }
    result
}

thread_local! {
    static NESTED_CREATE_PROCESS: std::cell::Cell<bool>  = const { std::cell::Cell::new(false) };
}

#[allow(non_snake_case)]
unsafe extern "system" fn ZludaNtCreateUserProcess(
    ProcessHandle: *mut winapi::um::winnt::HANDLE,
    ThreadHandle: *mut winapi::um::winnt::HANDLE,
    ProcessDesiredAccess: winapi::um::winnt::ACCESS_MASK,
    ThreadDesiredAccess: winapi::um::winnt::ACCESS_MASK,
    ProcessObjectAttributes: *mut winapi::shared::ntdef::OBJECT_ATTRIBUTES,
    ThreadObjectAttributes: *mut winapi::shared::ntdef::OBJECT_ATTRIBUTES,
    ProcessFlags: u32,
    ThreadFlags: u32,
    ProcessParameters: *mut winapi::ctypes::c_void,
    CreateInfo: *mut ntapi::ntpsapi::PS_CREATE_INFO,
    AttributeList: *mut ntapi::ntpsapi::PS_ATTRIBUTE_LIST,
) -> NTSTATUS {
    unsafe fn get_process_id(
        handle: winapi::um::winnt::HANDLE,
        attribute_list: *mut ntapi::ntpsapi::PS_ATTRIBUTE_LIST,
    ) -> Option<u32> {
        if let Some(attribute_list) = attribute_list.as_ref() {
            let attributes_count = ((attribute_list.TotalLength as usize
                - std::mem::size_of::<ntapi::ntpsapi::PS_ATTRIBUTE_LIST>())
                / std::mem::size_of::<ntapi::ntpsapi::PS_ATTRIBUTE>())
                + 1;
            let attributes =
                slice::from_raw_parts(attribute_list.Attributes.as_ptr(), attributes_count);
            for attribute in attributes {
                if attribute.Attribute == ntapi::ntpsapi::PS_ATTRIBUTE_CLIENT_ID {
                    if attribute.Size == std::mem::size_of::<CLIENT_ID>() {
                        let client_id = *(attribute.u.ValuePtr as *const CLIENT_ID);
                        return Some(client_id.UniqueProcess as u32);
                    }
                    break;
                }
            }
        }
        let process_id = GetProcessId(HANDLE(handle.cast()));
        if process_id == 0 {
            return None;
        }
        Some(process_id)
    }
    #[allow(static_mut_refs)]
    let detour_paths = DETOUR_PATHS.as_ref();
    if NESTED_CREATE_PROCESS.get() || detour_paths.is_none() {
        return NTSTATUS((NT_CREATE_USER_PROCESS)(
            ProcessHandle,
            ThreadHandle,
            ProcessDesiredAccess,
            ThreadDesiredAccess,
            ProcessObjectAttributes,
            ThreadObjectAttributes,
            ProcessFlags,
            ThreadFlags,
            ProcessParameters,
            CreateInfo,
            AttributeList,
        ));
    }
    let detour_paths = detour_paths.unwrap();
    let suspend_requested = (ThreadFlags & THREAD_CREATE_FLAGS_CREATE_SUSPENDED)
        == THREAD_CREATE_FLAGS_CREATE_SUSPENDED;
    let status = NTSTATUS((NT_CREATE_USER_PROCESS)(
        ProcessHandle,
        ThreadHandle,
        ProcessDesiredAccess,
        ThreadDesiredAccess,
        ProcessObjectAttributes,
        ThreadObjectAttributes,
        ProcessFlags,
        ThreadFlags | THREAD_CREATE_FLAGS_CREATE_SUSPENDED,
        ProcessParameters,
        CreateInfo,
        AttributeList,
    ));
    if status.is_err() {
        return status;
    }
    let process_handle = *ProcessHandle;
    DetourCopyPayloadToProcess(
        process_handle.cast(),
        ptr::from_ref(&unsafe {
            mem::transmute::<_, detours_sys::GUID>(zluda_windows::detours::GUID)
        })
        .cast_mut(),
        detour_paths.payload.as_ptr().cast_mut().cast(),
        detour_paths.payload.len() as u32,
    );
    let mut redirect_dll = detour_paths.redirect_dll.as_ptr().cast::<i8>();
    if DetourUpdateProcessWithDll(process_handle.cast(), &mut redirect_dll, 1) == 0 {
        if let Some(process_id) = get_process_id(process_handle, AttributeList) {
            NESTED_CREATE_PROCESS.set(true);
            DetourProcessViaHelperDllsW(
                process_id,
                1,
                &mut redirect_dll,
                Some(mem::transmute(
                    windows_sys::Win32::System::Threading::CreateProcessW as *const (),
                )),
            );
            NESTED_CREATE_PROCESS.set(false);
        }
    }
    if !suspend_requested {
        NtResumeThread(*ThreadHandle, ptr::null_mut());
    }
    status
}

#[allow(non_snake_case)]
unsafe extern "system" fn Zluda_nvrtcCompileProgram(
    prog: *const c_void,
    num_options: std::ffi::c_int,
    options: *const *const std::ffi::c_char,
) -> u32 {
    // TODO:
    // Right now we pick any one of the detoured nvrtcCompileProgram
    // functions, but if there are multiple nvrtc instances loaded this will be
    // incorrect. We should create a thunk for each library, but I don't have
    // the time right now.
    let original_fn = {
        nvrtc_detours()
            .lock()
            .ok()
            .and_then(|nvrtc_detours| nvrtc_detours.values().next().copied())
    };
    let nvrtcCompileProgram = match original_fn {
        Some(original_fn) => original_fn,
        None => return 11, // NVRTC_ERROR_INTERNAL_ERROR
    };
    let old_options = std::slice::from_raw_parts(options, num_options as usize);
    let mut options = vec![c"-arch=compute_86".as_ptr()];
    for &option in old_options {
        if nvrtc::is_arch_option(CStr::from_ptr(option)) {
            continue;
        }
        options.push(option);
    }
    (mem::transmute::<
        _,
        unsafe extern "system" fn(
            *const c_void,
            std::ffi::c_int,
            *const *const std::ffi::c_char,
        ) -> u32,
    >(nvrtcCompileProgram))(prog, options.len() as _, options.as_ptr())
}

// There might be multiple nvrtc instances loaded,
// so we need to keep track of which ones we've detoured
fn nvrtc_detours() -> &'static Mutex<FxHashMap<usize, usize>> {
    static NVRTC_DETOURS: OnceLock<Mutex<FxHashMap<usize, usize>>> = OnceLock::new();
    NVRTC_DETOURS.get_or_init(|| Mutex::new(FxHashMap::default()))
}

unsafe fn detour_nvrtc(handle: *mut c_void) -> Option<()> {
    let mut nvrtc_detours = nvrtc_detours().lock().ok()?;
    let nvrtc_entry = match nvrtc_detours.entry(handle as usize) {
        hash_map::Entry::Occupied(_) => return Some(()),
        hash_map::Entry::Vacant(entry) => entry,
    };
    let get_cubin = GetProcAddress(handle, c"nvrtcGetCUBIN".as_ptr().cast())?;
    let get_cubin_size = GetProcAddress(handle, c"nvrtcGetCUBINSize".as_ptr().cast())?;
    let get_ptx = GetProcAddress(handle, c"nvrtcGetPTX".as_ptr().cast())?;
    let get_ptx_size = GetProcAddress(handle, c"nvrtcGetPTXSize".as_ptr().cast())?;
    let nvrtc_compile_program = GetProcAddress(handle, c"nvrtcCompileProgram".as_ptr().cast())?;
    if !apply_detours(
        nvrtc_entry,
        get_cubin,
        get_cubin_size,
        get_ptx,
        get_ptx_size,
        nvrtc_compile_program,
    ) {
        DetourTransactionAbort();
    }
    Some(())
}

unsafe fn apply_detours(
    nvrtc_entry: hash_map::VacantEntry<usize, usize>,
    mut get_cubin: unsafe extern "system" fn() -> isize,
    mut get_cubin_size: unsafe extern "system" fn() -> isize,
    get_ptx: unsafe extern "system" fn() -> isize,
    get_ptx_size: unsafe extern "system" fn() -> isize,
    mut nvrtc_compile_program: unsafe extern "system" fn() -> isize,
) -> bool {
    if DetourTransactionBegin() != NO_ERROR as i32 {
        return false;
    }
    if DetourUpdateThread(GetCurrentThread().0) != NO_ERROR as i32 {
        return false;
    }
    if DetourAttach(std::ptr::from_mut(&mut get_cubin).cast(), get_ptx as _) != NO_ERROR as i32 {
        return false;
    }
    if DetourAttach(
        std::ptr::from_mut(&mut get_cubin_size).cast(),
        get_ptx_size as _,
    ) != NO_ERROR as i32
    {
        return false;
    }
    if DetourAttach(
        std::ptr::from_mut(&mut get_cubin_size).cast(),
        get_ptx_size as _,
    ) != NO_ERROR as i32
    {
        return false;
    }
    if DetourAttach(
        std::ptr::from_mut(&mut nvrtc_compile_program).cast(),
        Zluda_nvrtcCompileProgram as _,
    ) != NO_ERROR as i32
    {
        return false;
    }
    if DetourTransactionCommit() != NO_ERROR as i32 {
        return false;
    }
    nvrtc_entry.insert(nvrtc_compile_program as usize);
    true
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
    ldr_load_dll: Option<(*mut *mut c_void, *mut c_void)>,
    nt_create_user_process: Option<(*mut *mut c_void, *mut c_void)>,
}

impl DetourDetachGuard {
    // First element in the pair is ptr to original fn, second argument is the
    // new function. We accept *mut *mut c_void instead of *mut c_void as the
    // first element in the pair, because somehow otherwise original functions
    // also get overridden, so for example ZludaLoadLibraryExW ends calling
    // itself recursively until stack overflow exception occurs
    unsafe fn new<'a>() -> Option<Self> {
        let mut result = DetourDetachGuard {
            state: DetourUndoState::DoNothing,
            suspended_threads: Vec::new(),
            ldr_load_dll: None,
            nt_create_user_process: None,
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
        result.ldr_load_dll = Some(((&raw mut LDR_LOAD_DLL).cast(), ZludaLdrLoadDll as _));
        result.nt_create_user_process = Some((
            (&raw mut NT_CREATE_USER_PROCESS).cast(),
            ZludaNtCreateUserProcess as _,
        ));
        if let Some((original_fn, new_fn)) = result.ldr_load_dll {
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
                for detour in [self.ldr_load_dll, self.nt_create_user_process] {
                    if let Some((original_fn, new_fn)) = detour {
                        DetourDetach(original_fn, new_fn);
                    }
                }
                if let Ok(mut nvrtc_detours) = nvrtc_detours().lock() {
                    for (_, mut detoured) in nvrtc_detours.drain() {
                        DetourDetach(
                            std::ptr::from_mut(&mut detoured).cast(),
                            Zluda_nvrtcCompileProgram as _,
                        );
                    }
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
unsafe extern "system" fn DllMain(
    _instance_handle: *mut c_void,
    dwReason: u32,
    _: *const u8,
) -> i32 {
    use windows_sys::Win32::Foundation::{FALSE, TRUE};
    use windows_sys::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};
    if dwReason == DLL_PROCESS_ATTACH {
        if DetourIsHelperProcess() != FALSE {
            return TRUE;
        }
        if DetourRestoreAfterWith() == FALSE {
            return FALSE;
        }
        match DetourDetachGuard::new() {
            Some(g) => {
                DETOUR_DROP = Some(g);
                DETOUR_PATHS = DetourPaths::new();
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

mod nvrtc {
    use std::ffi::CStr;

    use winnow::ascii::alphanumeric1;
    use winnow::ascii::multispace0;
    use winnow::combinator::alt;
    use winnow::prelude::*;
    use winnow::PResult;

    fn continuation(input: &mut &str) -> PResult<()> {
        (
            multispace0,
            '=',
            multispace0,
            alphanumeric1,
            '_',
            alphanumeric1,
        )
            .void()
            .parse_next(input)
    }

    pub(crate) fn is_arch_option(s: &CStr) -> bool {
        let mut text = match s.to_str() {
            Ok(text) => text,
            Err(_) => return false,
        };
        (alt(("--gpu-architecture", "-arch")), continuation)
            .parse_next(&mut text)
            .is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::is_nvrtc;
    use widestring::{u16cstr, U16CStr};
    use windows::{core::PWSTR, Win32::Foundation::UNICODE_STRING};

    fn assert_is_nvrtc(text: &U16CStr) {
        assert!(unsafe {
            is_nvrtc(&UNICODE_STRING {
                Buffer: PWSTR(text.as_ptr().cast_mut()),
                Length: (text.len() * 2) as u16,
                MaximumLength: 0,
            })
            .unwrap()
        });
    }

    #[test]
    fn is_nvrtc1() {
        assert_is_nvrtc(u16cstr!("nvrtc64_112_0.dll"));
    }

    #[test]
    fn is_nvrtc2() {
        assert_is_nvrtc(u16cstr!("nvrtc64_130.dll"));
    }

    #[test]
    fn is_nvrtc3() {
        assert_is_nvrtc(u16cstr!(
            r#"C:\Users\vosen\.conda\envs\pytorch\lib\site-packages\torch\lib\nvrtc64_112_0.dll"#
        ));
    }

    #[test]
    fn is_nvrtc4() {
        assert_is_nvrtc(u16cstr!(
            r#"C:\Users\vosen\.conda\envs\pytorch\lib\site-packages\torch\lib\nvrtc64_120.DLL"#
        ));
    }
}
