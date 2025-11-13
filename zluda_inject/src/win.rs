#![allow(non_snake_case)]

use std::error;
use std::fmt;

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
}

macro_rules! last_ident {
    ($i:ident) => {
        stringify!($i)
    };
    ($start:ident, $($cont:ident),+) => {
        last_ident!($($cont),+)
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

/// Gets a detailed string description for the given error number.
pub fn error_string(mut errnum: i32) -> String {
    pub fn errno() -> i32 {
        unsafe { windows::Win32::Foundation::GetLastError() }.0 as i32
    }

    let mut buf = [0 as c::WCHAR; 2048];

    unsafe {
        let mut module = None;
        let mut flags = 0;

        // NTSTATUS errors may be encoded as HRESULT, which may returned from
        // GetLastError. For more information about Windows error codes, see
        // `[MS-ERREF]`: https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/0642cb2f-2075-4469-918c-4441e69c548a
        if (errnum & c::FACILITY_NT_BIT as i32) != 0 {
            // format according to https://support.microsoft.com/en-us/help/259693
            const NTDLL_DLL: windows::core::PCWSTR = windows::core::w!("NTDLL.DLL");
            let temp_module = windows::Win32::System::LibraryLoader::GetModuleHandleW(NTDLL_DLL);

            if !temp_module.is_err() {
                module = Some(temp_module.unwrap_unchecked().0.cast_const());
                errnum ^= c::FACILITY_NT_BIT as i32;
                flags = c::FORMAT_MESSAGE_FROM_HMODULE;
            }
        }

        let res = windows::Win32::System::Diagnostics::Debug::FormatMessageW(
            windows::Win32::System::Diagnostics::Debug::FORMAT_MESSAGE_OPTIONS(
                flags | c::FORMAT_MESSAGE_FROM_SYSTEM | c::FORMAT_MESSAGE_IGNORE_INSERTS,
            ),
            module,
            errnum as u32,
            0,
            windows_core::PWSTR(buf.as_mut_ptr()),
            buf.len() as u32,
            None,
        ) as usize;
        if res == 0 {
            // Sometimes FormatMessageW can fail e.g., system doesn't like 0 as langId,
            let fm_err = errno();
            return format!("OS Error {errnum} (FormatMessageW() returned error {fm_err})");
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
