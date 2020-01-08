#![allow(non_snake_case)]

use std::error;
use std::fmt;
use std::ptr;

mod c {
    use std::ffi::c_void;
    use std::os::raw::c_ulong;

    pub type DWORD = c_ulong;
    pub type HANDLE = LPVOID;
    pub type LPVOID = *mut c_void;
    pub type HINSTANCE = HANDLE;
    pub type HMODULE = HINSTANCE;
    pub type WCHAR = u16;
    pub type LPCWSTR = *const WCHAR;
    pub type LPWSTR = *mut WCHAR;

    pub const FACILITY_NT_BIT: DWORD = 0x1000_0000;
    pub const FORMAT_MESSAGE_FROM_HMODULE: DWORD = 0x00000800;
    pub const FORMAT_MESSAGE_FROM_SYSTEM: DWORD = 0x00001000;
    pub const FORMAT_MESSAGE_IGNORE_INSERTS: DWORD = 0x00000200;

    extern "system" {
        pub fn GetLastError() -> DWORD;
        pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;
        pub fn FormatMessageW(
            flags: DWORD,
            lpSrc: LPVOID,
            msgId: DWORD,
            langId: DWORD,
            buf: LPWSTR,
            nsize: DWORD,
            args: *const c_void,
        ) -> DWORD;
    }
}

macro_rules! last_ident {
    ($i:ident) => {
        stringify!($i)
    };
    ($start:ident, $($cont:ident),+) => {
        last_ident!($($cont),+)
    };
}

macro_rules! os_call {
    ($($path:ident)::+ ($($args:expr),*), $success:expr) => {
        let result = unsafe{ $($path)::+ ($($args),+) };
        if !($success)(result) {
            let name = last_ident!($($path),+);
            let err_code = $crate::win::errno();
            Err($crate::win::OsError{
                function: name,
                error_code: err_code as u32,
                message: $crate::win::error_string(err_code)
            })?;
        }
    };
}

#[derive(Debug)]
pub struct OsError {
    pub function: &'static str,
    pub error_code: u32,
    pub message: String,
}

impl fmt::Display for OsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for OsError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

pub fn errno() -> i32 {
    unsafe { c::GetLastError() as i32 }
}

/// Gets a detailed string description for the given error number.
pub fn error_string(mut errnum: i32) -> String {
    // This value is calculated from the macro
    // MAKELANGID(LANG_SYSTEM_DEFAULT, SUBLANG_SYS_DEFAULT)
    let langId = 0x0800 as c::DWORD;

    let mut buf = [0 as c::WCHAR; 2048];

    unsafe {
        let mut module = ptr::null_mut();
        let mut flags = 0;

        // NTSTATUS errors may be encoded as HRESULT, which may returned from
        // GetLastError. For more information about Windows error codes, see
        // `[MS-ERREF]`: https://msdn.microsoft.com/en-us/library/cc231198.aspx
        if (errnum & c::FACILITY_NT_BIT as i32) != 0 {
            // format according to https://support.microsoft.com/en-us/help/259693
            const NTDLL_DLL: &[u16] = &[
                'N' as _, 'T' as _, 'D' as _, 'L' as _, 'L' as _, '.' as _, 'D' as _, 'L' as _,
                'L' as _, 0,
            ];
            module = c::GetModuleHandleW(NTDLL_DLL.as_ptr());

            if module != ptr::null_mut() {
                errnum ^= c::FACILITY_NT_BIT as i32;
                flags = c::FORMAT_MESSAGE_FROM_HMODULE;
            }
        }

        let res = c::FormatMessageW(
            flags | c::FORMAT_MESSAGE_FROM_SYSTEM | c::FORMAT_MESSAGE_IGNORE_INSERTS,
            module,
            errnum as c::DWORD,
            langId,
            buf.as_mut_ptr(),
            buf.len() as c::DWORD,
            ptr::null(),
        ) as usize;
        if res == 0 {
            // Sometimes FormatMessageW can fail e.g., system doesn't like langId,
            let fm_err = errno();
            return format!(
                "OS Error {} (FormatMessageW() returned error {})",
                errnum, fm_err
            );
        }

        match String::from_utf16(&buf[..res]) {
            Ok(mut msg) => {
                // Trim trailing CRLF inserted by FormatMessageW
                let len = msg.trim_end().len();
                msg.truncate(len);
                msg
            }
            Err(..) => format!(
                "OS Error {} (FormatMessageW() returned \
                 invalid UTF-16)",
                errnum
            ),
        }
    }
}
