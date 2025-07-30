use std::{
    ffi::{c_char, c_int, c_void, CStr},
    mem,
    path::PathBuf,
    ptr::{self, NonNull},
    sync::LazyLock,
};
use unwrap_or::unwrap_some_or;

const RTLD_NEXT: *mut c_void = -1isize as _;

unsafe extern "C" {
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dladdr(addr: *const c_void, info: *mut DLInfo) -> c_int;
}

#[repr(C)]
struct DLInfo {
    dli_fname: *const c_char,
    dli_fbase: *mut c_void,
    dli_sname: *const c_char,
    dli_saddr: *mut c_void,
}

static FILES_FOR_REDIRECT: [&'static str; 14] = [
    "libcublas.so",
    "libcublas.so.12",
    "libcublasLt.so",
    "libcublasLt.so.12",
    "libcuda.so",
    "libcuda.so.1",
    "libcudnn.so",
    "libcudnn.so.9",
    "libcufft.so",
    "libcufft.so.11",
    "libcusparse.so",
    "libcusparse.so.12",
    "libnvidia-ml.so",
    "libnvidia-ml.so.1",
];

static GLOBALS: LazyLock<(
    Option<unsafe extern "C" fn(*const c_char, c_int) -> DlopenResult>,
    Option<[Vec<u8>; FILES_FOR_REDIRECT.len()]>,
)> = LazyLock::new(|| {
    let dlopen_next = unsafe { mem::transmute(dlsym(RTLD_NEXT, c"dlopen".as_ptr())) };
    let mut self_dlinfo = unsafe { mem::zeroed::<DLInfo>() };
    let self_dir = if unsafe { dladdr(dlopen as _, &mut self_dlinfo) } != 0 {
        unsafe { CStr::from_ptr(self_dlinfo.dli_fname) }
            .to_str()
            .ok()
            .and_then(|path| {
                let mut pathbuf = PathBuf::from(path);
                if !pathbuf.pop() {
                    return None;
                }
                Some(FILES_FOR_REDIRECT.map(|file| {
                    let mut buffer = pathbuf.join(file).into_os_string().into_encoded_bytes();
                    buffer.push(0);
                    buffer
                }))
            })
    } else {
        None
    };
    (dlopen_next, self_dir)
});

pub const RTLD_GLOBAL: c_int = 0x100;
pub const RTLD_LAZY: c_int = 1;

#[ctor::ctor]
unsafe fn ctor() {
    let (dlopen_next, replacement_paths) = &*GLOBALS;
    let dlopen_next = unwrap_some_or!(dlopen_next, return);
    let replacement_paths = unwrap_some_or!(replacement_paths, return);
    for replacement in replacement_paths.into_iter() {
        dlopen_next(replacement.as_ptr().cast(), RTLD_GLOBAL | RTLD_LAZY).ok();
    }
}

type DlopenResult = Result<NonNull<c_void>, ()>;

const _: fn() = || {
    let _ = std::mem::transmute::<*mut c_void, DlopenResult>;
};

#[no_mangle]
unsafe extern "C" fn dlopen(filename: *const c_char, flags: c_int) -> DlopenResult {
    let (dlopen_next, replacement_paths) = &*GLOBALS;
    let dlopen_next = dlopen_next.ok_or(())?;
    dlopen_redirect(dlopen_next, replacement_paths, filename, flags)
        .or_else(|| dlopen_next(filename, flags).ok())
        .ok_or(())
}

#[no_mangle]
unsafe extern "C" fn zluda_dlopen_noredirect(
    filename: *const c_char,
    flags: c_int,
) -> DlopenResult {
    let dlopen_next = GLOBALS.0.ok_or(())?;
    dlopen_next(filename, flags)
}

unsafe fn dlopen_redirect<'a>(
    dlopen_next: unsafe extern "C" fn(*const c_char, c_int) -> DlopenResult,
    replacement_paths: &'a Option<[Vec<u8>; FILES_FOR_REDIRECT.len()]>,
    input_path: *const c_char,
    flags: c_int,
) -> Option<NonNull<c_void>> {
    if input_path == ptr::null() {
        return None;
    }
    let input_path = CStr::from_ptr(input_path).to_str().ok()?;
    let replacement_paths = replacement_paths.as_ref()?;
    let replacement_path = FILES_FOR_REDIRECT
        .into_iter()
        .zip(replacement_paths.into_iter())
        .find_map(|(file, path)| {
            if input_path.ends_with(file) {
                Some(path)
            } else {
                None
            }
        })?;
    unsafe { dlopen_next(replacement_path.as_ptr() as _, flags) }.ok()
}
