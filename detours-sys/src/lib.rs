//! Bindings to the Microsoft Detours API.
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Rebuild with:
// 
include!("bundled_bindings.rs");

#[cfg(test)]
mod tests {
    use super::*;
    use std::{ffi, ptr};
    use winapi::{
        shared::minwindef::LPVOID,
        um::{processthreadsapi::GetCurrentThread, synchapi::Sleep, sysinfoapi::GetTickCount},
    };

    static mut TRUE_SLEEP: unsafe extern "system" fn(DWORD) = Sleep;
    static mut SLEPT: LONG = 0;

    // Detour function that replaces the Sleep API.
    unsafe extern "system" fn TimedSleep(dwMilliseconds: DWORD) {
        // Save the before and after times around calling the Sleep API.
        let dwBeg: DWORD = GetTickCount();
        TRUE_SLEEP(dwMilliseconds);
        let dwEnd: DWORD = GetTickCount();

        SLEPT = (dwEnd - dwBeg) as i32;
    }

    extern "system" fn DllMain(_: HINSTANCE, reason: DWORD, _: LPVOID) -> BOOL {
        if unsafe { DetourIsHelperProcess() } == 1 {
            return 1;
        }

        let tru = unsafe { &mut TRUE_SLEEP as *mut _ as *mut *mut ffi::c_void };
        let new = TimedSleep as *mut ffi::c_void;

        match reason {
            // DLL_PROCESS_ATTACH
            1 => unsafe {
                DetourRestoreAfterWith();

                DetourTransactionBegin();
                DetourUpdateThread(GetCurrentThread() as _);
                DetourAttach(tru, new);
                DetourTransactionCommit();
            },
            // DLL_PROCESS_DETACH
            0 => unsafe {
                DetourTransactionBegin();
                DetourUpdateThread(GetCurrentThread() as _);
                DetourDetach(tru, new);
                DetourTransactionCommit();
            },
            _ => (),
        }
        1
    }

    #[test]
    fn hook_self() {
        unsafe {
            DllMain(ptr::null_mut(), 1, ptr::null_mut());
            let slept;

            Sleep(500);
            slept = SLEPT;
            assert_ne!(SLEPT, 0);

            DllMain(ptr::null_mut(), 0, ptr::null_mut());
            Sleep(500);
            assert_eq!(slept, SLEPT);
        }
    }
}
