/* automatically generated by rust-bindgen 0.56.0 */

pub type wchar_t = ::std::os::raw::c_ushort;
pub type ULONG = ::std::os::raw::c_ulong;
pub type DWORD = ::std::os::raw::c_ulong;
pub type BOOL = ::std::os::raw::c_int;
pub type BYTE = ::std::os::raw::c_uchar;
pub type WORD = ::std::os::raw::c_ushort;
pub type LPBYTE = *mut BYTE;
pub type PDWORD = *mut DWORD;
pub type LPVOID = *mut ::std::os::raw::c_void;
pub type LPCVOID = *const ::std::os::raw::c_void;
pub type INT = ::std::os::raw::c_int;
pub type PVOID = *mut ::std::os::raw::c_void;
pub type CHAR = ::std::os::raw::c_char;
pub type LONG = ::std::os::raw::c_long;
pub type WCHAR = wchar_t;
pub type LPWSTR = *mut WCHAR;
pub type LPCWSTR = *const WCHAR;
pub type LPSTR = *mut CHAR;
pub type LPCSTR = *const CHAR;
pub type HANDLE = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GUID {
    pub Data1: ::std::os::raw::c_ulong,
    pub Data2: ::std::os::raw::c_ushort,
    pub Data3: ::std::os::raw::c_ushort,
    pub Data4: [::std::os::raw::c_uchar; 8usize],
}
#[test]
fn bindgen_test_layout__GUID() {
    assert_eq!(
        ::std::mem::size_of::<_GUID>(),
        16usize,
        concat!("Size of: ", stringify!(_GUID))
    );
    assert_eq!(
        ::std::mem::align_of::<_GUID>(),
        4usize,
        concat!("Alignment of ", stringify!(_GUID))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GUID>())).Data1 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_GUID),
            "::",
            stringify!(Data1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GUID>())).Data2 as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_GUID),
            "::",
            stringify!(Data2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GUID>())).Data3 as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(_GUID),
            "::",
            stringify!(Data3)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_GUID>())).Data4 as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_GUID),
            "::",
            stringify!(Data4)
        )
    );
}
pub type GUID = _GUID;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HINSTANCE__ {
    pub unused: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_HINSTANCE__() {
    assert_eq!(
        ::std::mem::size_of::<HINSTANCE__>(),
        4usize,
        concat!("Size of: ", stringify!(HINSTANCE__))
    );
    assert_eq!(
        ::std::mem::align_of::<HINSTANCE__>(),
        4usize,
        concat!("Alignment of ", stringify!(HINSTANCE__))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<HINSTANCE__>())).unused as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(HINSTANCE__),
            "::",
            stringify!(unused)
        )
    );
}
pub type HINSTANCE = *mut HINSTANCE__;
pub type HMODULE = HINSTANCE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HWND__ {
    pub unused: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_HWND__() {
    assert_eq!(
        ::std::mem::size_of::<HWND__>(),
        4usize,
        concat!("Size of: ", stringify!(HWND__))
    );
    assert_eq!(
        ::std::mem::align_of::<HWND__>(),
        4usize,
        concat!("Alignment of ", stringify!(HWND__))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<HWND__>())).unused as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(HWND__),
            "::",
            stringify!(unused)
        )
    );
}
pub type HWND = *mut HWND__;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _SECURITY_ATTRIBUTES {
    pub nLength: DWORD,
    pub lpSecurityDescriptor: LPVOID,
    pub bInheritHandle: BOOL,
}
#[test]
fn bindgen_test_layout__SECURITY_ATTRIBUTES() {
    assert_eq!(
        ::std::mem::size_of::<_SECURITY_ATTRIBUTES>(),
        24usize,
        concat!("Size of: ", stringify!(_SECURITY_ATTRIBUTES))
    );
    assert_eq!(
        ::std::mem::align_of::<_SECURITY_ATTRIBUTES>(),
        8usize,
        concat!("Alignment of ", stringify!(_SECURITY_ATTRIBUTES))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_SECURITY_ATTRIBUTES>())).nLength as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_SECURITY_ATTRIBUTES),
            "::",
            stringify!(nLength)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_SECURITY_ATTRIBUTES>())).lpSecurityDescriptor as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_SECURITY_ATTRIBUTES),
            "::",
            stringify!(lpSecurityDescriptor)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_SECURITY_ATTRIBUTES>())).bInheritHandle as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_SECURITY_ATTRIBUTES),
            "::",
            stringify!(bInheritHandle)
        )
    );
}
pub type LPSECURITY_ATTRIBUTES = *mut _SECURITY_ATTRIBUTES;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PROCESS_INFORMATION {
    pub hProcess: HANDLE,
    pub hThread: HANDLE,
    pub dwProcessId: DWORD,
    pub dwThreadId: DWORD,
}
#[test]
fn bindgen_test_layout__PROCESS_INFORMATION() {
    assert_eq!(
        ::std::mem::size_of::<_PROCESS_INFORMATION>(),
        24usize,
        concat!("Size of: ", stringify!(_PROCESS_INFORMATION))
    );
    assert_eq!(
        ::std::mem::align_of::<_PROCESS_INFORMATION>(),
        8usize,
        concat!("Alignment of ", stringify!(_PROCESS_INFORMATION))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PROCESS_INFORMATION>())).hProcess as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_PROCESS_INFORMATION),
            "::",
            stringify!(hProcess)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PROCESS_INFORMATION>())).hThread as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_PROCESS_INFORMATION),
            "::",
            stringify!(hThread)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_PROCESS_INFORMATION>())).dwProcessId as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_PROCESS_INFORMATION),
            "::",
            stringify!(dwProcessId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_PROCESS_INFORMATION>())).dwThreadId as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(_PROCESS_INFORMATION),
            "::",
            stringify!(dwThreadId)
        )
    );
}
pub type LPPROCESS_INFORMATION = *mut _PROCESS_INFORMATION;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _STARTUPINFOA {
    pub cb: DWORD,
    pub lpReserved: LPSTR,
    pub lpDesktop: LPSTR,
    pub lpTitle: LPSTR,
    pub dwX: DWORD,
    pub dwY: DWORD,
    pub dwXSize: DWORD,
    pub dwYSize: DWORD,
    pub dwXCountChars: DWORD,
    pub dwYCountChars: DWORD,
    pub dwFillAttribute: DWORD,
    pub dwFlags: DWORD,
    pub wShowWindow: WORD,
    pub cbReserved2: WORD,
    pub lpReserved2: LPBYTE,
    pub hStdInput: HANDLE,
    pub hStdOutput: HANDLE,
    pub hStdError: HANDLE,
}
#[test]
fn bindgen_test_layout__STARTUPINFOA() {
    assert_eq!(
        ::std::mem::size_of::<_STARTUPINFOA>(),
        104usize,
        concat!("Size of: ", stringify!(_STARTUPINFOA))
    );
    assert_eq!(
        ::std::mem::align_of::<_STARTUPINFOA>(),
        8usize,
        concat!("Alignment of ", stringify!(_STARTUPINFOA))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOA>())).cb as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOA),
            "::",
            stringify!(cb)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOA>())).lpReserved as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOA),
            "::",
            stringify!(lpReserved)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOA>())).lpDesktop as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOA),
            "::",
            stringify!(lpDesktop)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOA>())).lpTitle as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOA),
            "::",
            stringify!(lpTitle)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOA>())).dwX as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOA),
            "::",
            stringify!(dwX)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOA>())).dwY as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOA),
            "::",
            stringify!(dwY)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOA>())).dwXSize as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOA),
            "::",
            stringify!(dwXSize)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOA>())).dwYSize as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOA),
            "::",
            stringify!(dwYSize)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOA>())).dwXCountChars as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOA),
            "::",
            stringify!(dwXCountChars)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOA>())).dwYCountChars as *const _ as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOA),
            "::",
            stringify!(dwYCountChars)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOA>())).dwFillAttribute as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOA),
            "::",
            stringify!(dwFillAttribute)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOA>())).dwFlags as *const _ as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOA),
            "::",
            stringify!(dwFlags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOA>())).wShowWindow as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOA),
            "::",
            stringify!(wShowWindow)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOA>())).cbReserved2 as *const _ as usize },
        66usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOA),
            "::",
            stringify!(cbReserved2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOA>())).lpReserved2 as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOA),
            "::",
            stringify!(lpReserved2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOA>())).hStdInput as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOA),
            "::",
            stringify!(hStdInput)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOA>())).hStdOutput as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOA),
            "::",
            stringify!(hStdOutput)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOA>())).hStdError as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOA),
            "::",
            stringify!(hStdError)
        )
    );
}
pub type LPSTARTUPINFOA = *mut _STARTUPINFOA;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _STARTUPINFOW {
    pub cb: DWORD,
    pub lpReserved: LPWSTR,
    pub lpDesktop: LPWSTR,
    pub lpTitle: LPWSTR,
    pub dwX: DWORD,
    pub dwY: DWORD,
    pub dwXSize: DWORD,
    pub dwYSize: DWORD,
    pub dwXCountChars: DWORD,
    pub dwYCountChars: DWORD,
    pub dwFillAttribute: DWORD,
    pub dwFlags: DWORD,
    pub wShowWindow: WORD,
    pub cbReserved2: WORD,
    pub lpReserved2: LPBYTE,
    pub hStdInput: HANDLE,
    pub hStdOutput: HANDLE,
    pub hStdError: HANDLE,
}
#[test]
fn bindgen_test_layout__STARTUPINFOW() {
    assert_eq!(
        ::std::mem::size_of::<_STARTUPINFOW>(),
        104usize,
        concat!("Size of: ", stringify!(_STARTUPINFOW))
    );
    assert_eq!(
        ::std::mem::align_of::<_STARTUPINFOW>(),
        8usize,
        concat!("Alignment of ", stringify!(_STARTUPINFOW))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOW>())).cb as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOW),
            "::",
            stringify!(cb)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOW>())).lpReserved as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOW),
            "::",
            stringify!(lpReserved)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOW>())).lpDesktop as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOW),
            "::",
            stringify!(lpDesktop)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOW>())).lpTitle as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOW),
            "::",
            stringify!(lpTitle)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOW>())).dwX as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOW),
            "::",
            stringify!(dwX)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOW>())).dwY as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOW),
            "::",
            stringify!(dwY)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOW>())).dwXSize as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOW),
            "::",
            stringify!(dwXSize)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOW>())).dwYSize as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOW),
            "::",
            stringify!(dwYSize)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOW>())).dwXCountChars as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOW),
            "::",
            stringify!(dwXCountChars)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOW>())).dwYCountChars as *const _ as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOW),
            "::",
            stringify!(dwYCountChars)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOW>())).dwFillAttribute as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOW),
            "::",
            stringify!(dwFillAttribute)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOW>())).dwFlags as *const _ as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOW),
            "::",
            stringify!(dwFlags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOW>())).wShowWindow as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOW),
            "::",
            stringify!(wShowWindow)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOW>())).cbReserved2 as *const _ as usize },
        66usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOW),
            "::",
            stringify!(cbReserved2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOW>())).lpReserved2 as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOW),
            "::",
            stringify!(lpReserved2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOW>())).hStdInput as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOW),
            "::",
            stringify!(hStdInput)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOW>())).hStdOutput as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOW),
            "::",
            stringify!(hStdOutput)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_STARTUPINFOW>())).hStdError as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_STARTUPINFOW),
            "::",
            stringify!(hStdError)
        )
    );
}
pub type LPSTARTUPINFOW = *mut _STARTUPINFOW;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _DETOUR_TRAMPOLINE {
    _unused: [u8; 0],
}
pub type PDETOUR_TRAMPOLINE = *mut _DETOUR_TRAMPOLINE;
#[doc = " Binary Typedefs."]
pub type PF_DETOUR_BINARY_BYWAY_CALLBACK = ::std::option::Option<
    unsafe extern "stdcall" fn(pContext: PVOID, pszFile: LPCSTR, ppszOutFile: *mut LPCSTR) -> BOOL,
>;
pub type PF_DETOUR_BINARY_FILE_CALLBACK = ::std::option::Option<
    unsafe extern "stdcall" fn(
        pContext: PVOID,
        pszOrigFile: LPCSTR,
        pszFile: LPCSTR,
        ppszOutFile: *mut LPCSTR,
    ) -> BOOL,
>;
pub type PF_DETOUR_BINARY_SYMBOL_CALLBACK = ::std::option::Option<
    unsafe extern "stdcall" fn(
        pContext: PVOID,
        nOrigOrdinal: ULONG,
        nOrdinal: ULONG,
        pnOutOrdinal: *mut ULONG,
        pszOrigSymbol: LPCSTR,
        pszSymbol: LPCSTR,
        ppszOutSymbol: *mut LPCSTR,
    ) -> BOOL,
>;
pub type PF_DETOUR_BINARY_COMMIT_CALLBACK =
    ::std::option::Option<unsafe extern "stdcall" fn(pContext: PVOID) -> BOOL>;
pub type PF_DETOUR_ENUMERATE_EXPORT_CALLBACK = ::std::option::Option<
    unsafe extern "stdcall" fn(pContext: PVOID, nOrdinal: ULONG, pszName: LPCSTR, pCode: PVOID) -> BOOL,
>;
pub type PF_DETOUR_IMPORT_FILE_CALLBACK = ::std::option::Option<
    unsafe extern "stdcall" fn(pContext: PVOID, hModule: HMODULE, pszFile: LPCSTR) -> BOOL,
>;
pub type PF_DETOUR_IMPORT_FUNC_CALLBACK = ::std::option::Option<
    unsafe extern "stdcall" fn(pContext: PVOID, nOrdinal: DWORD, pszFunc: LPCSTR, pvFunc: PVOID) -> BOOL,
>;
pub type PF_DETOUR_IMPORT_FUNC_CALLBACK_EX = ::std::option::Option<
    unsafe extern "stdcall" fn(
        pContext: PVOID,
        nOrdinal: DWORD,
        pszFunc: LPCSTR,
        ppvFunc: *mut PVOID,
    ) -> BOOL,
>;
pub type PDETOUR_BINARY = *mut ::std::os::raw::c_void;
extern "stdcall" {
    #[doc = " Transaction APIs."]
    pub fn DetourTransactionBegin() -> LONG;
}
extern "stdcall" {
    pub fn DetourTransactionAbort() -> LONG;
}
extern "stdcall" {
    pub fn DetourTransactionCommit() -> LONG;
}
extern "stdcall" {
    pub fn DetourTransactionCommitEx(pppFailedPointer: *mut *mut PVOID) -> LONG;
}
extern "stdcall" {
    pub fn DetourUpdateThread(hThread: HANDLE) -> LONG;
}
extern "stdcall" {
    pub fn DetourAttach(ppPointer: *mut PVOID, pDetour: PVOID) -> LONG;
}
extern "stdcall" {
    pub fn DetourAttachEx(
        ppPointer: *mut PVOID,
        pDetour: PVOID,
        ppRealTrampoline: *mut PDETOUR_TRAMPOLINE,
        ppRealTarget: *mut PVOID,
        ppRealDetour: *mut PVOID,
    ) -> LONG;
}
extern "stdcall" {
    pub fn DetourDetach(ppPointer: *mut PVOID, pDetour: PVOID) -> LONG;
}
extern "stdcall" {
    pub fn DetourSetIgnoreTooSmall(fIgnore: BOOL) -> BOOL;
}
extern "stdcall" {
    pub fn DetourSetRetainRegions(fRetain: BOOL) -> BOOL;
}
extern "stdcall" {
    pub fn DetourSetSystemRegionLowerBound(pSystemRegionLowerBound: PVOID) -> PVOID;
}
extern "stdcall" {
    pub fn DetourSetSystemRegionUpperBound(pSystemRegionUpperBound: PVOID) -> PVOID;
}
extern "stdcall" {
    #[doc = " Code Functions."]
    pub fn DetourFindFunction(pszModule: LPCSTR, pszFunction: LPCSTR) -> PVOID;
}
extern "stdcall" {
    pub fn DetourCodeFromPointer(pPointer: PVOID, ppGlobals: *mut PVOID) -> PVOID;
}
extern "stdcall" {
    pub fn DetourCopyInstruction(
        pDst: PVOID,
        ppDstPool: *mut PVOID,
        pSrc: PVOID,
        ppTarget: *mut PVOID,
        plExtra: *mut LONG,
    ) -> PVOID;
}
extern "stdcall" {
    pub fn DetourSetCodeModule(hModule: HMODULE, fLimitReferencesToModule: BOOL) -> BOOL;
}
extern "stdcall" {
    pub fn DetourAllocateRegionWithinJumpBounds(
        pbTarget: LPCVOID,
        pcbAllocatedSize: PDWORD,
    ) -> PVOID;
}
extern "stdcall" {
    #[doc = " Loaded Binary Functions."]
    pub fn DetourGetContainingModule(pvAddr: PVOID) -> HMODULE;
}
extern "stdcall" {
    pub fn DetourEnumerateModules(hModuleLast: HMODULE) -> HMODULE;
}
extern "stdcall" {
    pub fn DetourGetEntryPoint(hModule: HMODULE) -> PVOID;
}
extern "stdcall" {
    pub fn DetourGetModuleSize(hModule: HMODULE) -> ULONG;
}
extern "stdcall" {
    pub fn DetourEnumerateExports(
        hModule: HMODULE,
        pContext: PVOID,
        pfExport: PF_DETOUR_ENUMERATE_EXPORT_CALLBACK,
    ) -> BOOL;
}
extern "stdcall" {
    pub fn DetourEnumerateImports(
        hModule: HMODULE,
        pContext: PVOID,
        pfImportFile: PF_DETOUR_IMPORT_FILE_CALLBACK,
        pfImportFunc: PF_DETOUR_IMPORT_FUNC_CALLBACK,
    ) -> BOOL;
}
extern "stdcall" {
    pub fn DetourEnumerateImportsEx(
        hModule: HMODULE,
        pContext: PVOID,
        pfImportFile: PF_DETOUR_IMPORT_FILE_CALLBACK,
        pfImportFuncEx: PF_DETOUR_IMPORT_FUNC_CALLBACK_EX,
    ) -> BOOL;
}
extern "stdcall" {
    pub fn DetourFindPayload(hModule: HMODULE, rguid: *const GUID, pcbData: *mut DWORD) -> PVOID;
}
extern "stdcall" {
    pub fn DetourFindPayloadEx(rguid: *const GUID, pcbData: *mut DWORD) -> PVOID;
}
extern "stdcall" {
    pub fn DetourGetSizeOfPayloads(hModule: HMODULE) -> DWORD;
}
extern "stdcall" {
    #[doc = " Persistent Binary Functions."]
    pub fn DetourBinaryOpen(hFile: HANDLE) -> PDETOUR_BINARY;
}
extern "stdcall" {
    pub fn DetourBinaryEnumeratePayloads(
        pBinary: PDETOUR_BINARY,
        pGuid: *mut GUID,
        pcbData: *mut DWORD,
        pnIterator: *mut DWORD,
    ) -> PVOID;
}
extern "stdcall" {
    pub fn DetourBinaryFindPayload(
        pBinary: PDETOUR_BINARY,
        rguid: *const GUID,
        pcbData: *mut DWORD,
    ) -> PVOID;
}
extern "stdcall" {
    pub fn DetourBinarySetPayload(
        pBinary: PDETOUR_BINARY,
        rguid: *const GUID,
        pData: PVOID,
        cbData: DWORD,
    ) -> PVOID;
}
extern "stdcall" {
    pub fn DetourBinaryDeletePayload(pBinary: PDETOUR_BINARY, rguid: *const GUID) -> BOOL;
}
extern "stdcall" {
    pub fn DetourBinaryPurgePayloads(pBinary: PDETOUR_BINARY) -> BOOL;
}
extern "stdcall" {
    pub fn DetourBinaryResetImports(pBinary: PDETOUR_BINARY) -> BOOL;
}
extern "stdcall" {
    pub fn DetourBinaryEditImports(
        pBinary: PDETOUR_BINARY,
        pContext: PVOID,
        pfByway: PF_DETOUR_BINARY_BYWAY_CALLBACK,
        pfFile: PF_DETOUR_BINARY_FILE_CALLBACK,
        pfSymbol: PF_DETOUR_BINARY_SYMBOL_CALLBACK,
        pfCommit: PF_DETOUR_BINARY_COMMIT_CALLBACK,
    ) -> BOOL;
}
extern "stdcall" {
    pub fn DetourBinaryWrite(pBinary: PDETOUR_BINARY, hFile: HANDLE) -> BOOL;
}
extern "stdcall" {
    pub fn DetourBinaryClose(pBinary: PDETOUR_BINARY) -> BOOL;
}
#[doc = " Create Process & Load Dll."]
pub type PDETOUR_CREATE_PROCESS_ROUTINEA = ::std::option::Option<
    unsafe extern "stdcall" fn(
        lpApplicationName: LPCSTR,
        lpCommandLine: LPSTR,
        lpProcessAttributes: LPSECURITY_ATTRIBUTES,
        lpThreadAttributes: LPSECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: DWORD,
        lpEnvironment: LPVOID,
        lpCurrentDirectory: LPCSTR,
        lpStartupInfo: LPSTARTUPINFOA,
        lpProcessInformation: LPPROCESS_INFORMATION,
    ) -> BOOL,
>;
pub type PDETOUR_CREATE_PROCESS_ROUTINEW = ::std::option::Option<
    unsafe extern "stdcall" fn(
        lpApplicationName: LPCWSTR,
        lpCommandLine: LPWSTR,
        lpProcessAttributes: LPSECURITY_ATTRIBUTES,
        lpThreadAttributes: LPSECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: DWORD,
        lpEnvironment: LPVOID,
        lpCurrentDirectory: LPCWSTR,
        lpStartupInfo: LPSTARTUPINFOW,
        lpProcessInformation: LPPROCESS_INFORMATION,
    ) -> BOOL,
>;
extern "stdcall" {
    pub fn DetourCreateProcessWithDllA(
        lpApplicationName: LPCSTR,
        lpCommandLine: LPSTR,
        lpProcessAttributes: LPSECURITY_ATTRIBUTES,
        lpThreadAttributes: LPSECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: DWORD,
        lpEnvironment: LPVOID,
        lpCurrentDirectory: LPCSTR,
        lpStartupInfo: LPSTARTUPINFOA,
        lpProcessInformation: LPPROCESS_INFORMATION,
        lpDllName: LPCSTR,
        pfCreateProcessA: PDETOUR_CREATE_PROCESS_ROUTINEA,
    ) -> BOOL;
}
extern "stdcall" {
    pub fn DetourCreateProcessWithDllW(
        lpApplicationName: LPCWSTR,
        lpCommandLine: LPWSTR,
        lpProcessAttributes: LPSECURITY_ATTRIBUTES,
        lpThreadAttributes: LPSECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: DWORD,
        lpEnvironment: LPVOID,
        lpCurrentDirectory: LPCWSTR,
        lpStartupInfo: LPSTARTUPINFOW,
        lpProcessInformation: LPPROCESS_INFORMATION,
        lpDllName: LPCSTR,
        pfCreateProcessW: PDETOUR_CREATE_PROCESS_ROUTINEW,
    ) -> BOOL;
}
extern "stdcall" {
    pub fn DetourCreateProcessWithDllExA(
        lpApplicationName: LPCSTR,
        lpCommandLine: LPSTR,
        lpProcessAttributes: LPSECURITY_ATTRIBUTES,
        lpThreadAttributes: LPSECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: DWORD,
        lpEnvironment: LPVOID,
        lpCurrentDirectory: LPCSTR,
        lpStartupInfo: LPSTARTUPINFOA,
        lpProcessInformation: LPPROCESS_INFORMATION,
        lpDllName: LPCSTR,
        pfCreateProcessA: PDETOUR_CREATE_PROCESS_ROUTINEA,
    ) -> BOOL;
}
extern "stdcall" {
    pub fn DetourCreateProcessWithDllExW(
        lpApplicationName: LPCWSTR,
        lpCommandLine: LPWSTR,
        lpProcessAttributes: LPSECURITY_ATTRIBUTES,
        lpThreadAttributes: LPSECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: DWORD,
        lpEnvironment: LPVOID,
        lpCurrentDirectory: LPCWSTR,
        lpStartupInfo: LPSTARTUPINFOW,
        lpProcessInformation: LPPROCESS_INFORMATION,
        lpDllName: LPCSTR,
        pfCreateProcessW: PDETOUR_CREATE_PROCESS_ROUTINEW,
    ) -> BOOL;
}
extern "stdcall" {
    pub fn DetourCreateProcessWithDllsA(
        lpApplicationName: LPCSTR,
        lpCommandLine: LPSTR,
        lpProcessAttributes: LPSECURITY_ATTRIBUTES,
        lpThreadAttributes: LPSECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: DWORD,
        lpEnvironment: LPVOID,
        lpCurrentDirectory: LPCSTR,
        lpStartupInfo: LPSTARTUPINFOA,
        lpProcessInformation: LPPROCESS_INFORMATION,
        nDlls: DWORD,
        rlpDlls: *mut LPCSTR,
        pfCreateProcessA: PDETOUR_CREATE_PROCESS_ROUTINEA,
    ) -> BOOL;
}
extern "stdcall" {
    pub fn DetourCreateProcessWithDllsW(
        lpApplicationName: LPCWSTR,
        lpCommandLine: LPWSTR,
        lpProcessAttributes: LPSECURITY_ATTRIBUTES,
        lpThreadAttributes: LPSECURITY_ATTRIBUTES,
        bInheritHandles: BOOL,
        dwCreationFlags: DWORD,
        lpEnvironment: LPVOID,
        lpCurrentDirectory: LPCWSTR,
        lpStartupInfo: LPSTARTUPINFOW,
        lpProcessInformation: LPPROCESS_INFORMATION,
        nDlls: DWORD,
        rlpDlls: *mut LPCSTR,
        pfCreateProcessW: PDETOUR_CREATE_PROCESS_ROUTINEW,
    ) -> BOOL;
}
extern "stdcall" {
    pub fn DetourProcessViaHelperA(
        dwTargetPid: DWORD,
        lpDllName: LPCSTR,
        pfCreateProcessA: PDETOUR_CREATE_PROCESS_ROUTINEA,
    ) -> BOOL;
}
extern "stdcall" {
    pub fn DetourProcessViaHelperW(
        dwTargetPid: DWORD,
        lpDllName: LPCSTR,
        pfCreateProcessW: PDETOUR_CREATE_PROCESS_ROUTINEW,
    ) -> BOOL;
}
extern "stdcall" {
    pub fn DetourProcessViaHelperDllsA(
        dwTargetPid: DWORD,
        nDlls: DWORD,
        rlpDlls: *mut LPCSTR,
        pfCreateProcessA: PDETOUR_CREATE_PROCESS_ROUTINEA,
    ) -> BOOL;
}
extern "stdcall" {
    pub fn DetourProcessViaHelperDllsW(
        dwTargetPid: DWORD,
        nDlls: DWORD,
        rlpDlls: *mut LPCSTR,
        pfCreateProcessW: PDETOUR_CREATE_PROCESS_ROUTINEW,
    ) -> BOOL;
}
extern "stdcall" {
    pub fn DetourUpdateProcessWithDll(hProcess: HANDLE, rlpDlls: *mut LPCSTR, nDlls: DWORD)
        -> BOOL;
}
extern "stdcall" {
    pub fn DetourUpdateProcessWithDllEx(
        hProcess: HANDLE,
        hImage: HMODULE,
        bIs32Bit: BOOL,
        rlpDlls: *mut LPCSTR,
        nDlls: DWORD,
    ) -> BOOL;
}
extern "stdcall" {
    pub fn DetourCopyPayloadToProcess(
        hProcess: HANDLE,
        rguid: *const GUID,
        pvData: PVOID,
        cbData: DWORD,
    ) -> BOOL;
}
extern "stdcall" {
    pub fn DetourRestoreAfterWith() -> BOOL;
}
extern "stdcall" {
    pub fn DetourRestoreAfterWithEx(pvData: PVOID, cbData: DWORD) -> BOOL;
}
extern "stdcall" {
    pub fn DetourIsHelperProcess() -> BOOL;
}
extern "stdcall" {
    pub fn DetourFinishHelperProcess(arg1: HWND, arg2: HINSTANCE, arg3: LPSTR, arg4: INT);
}
