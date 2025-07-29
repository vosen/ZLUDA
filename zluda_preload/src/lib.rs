use std::{
    ffi::{c_char, c_int, c_void, CStr},
    mem,
    path::PathBuf,
    ptr::NonNull,
    sync::LazyLock,
};

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

static FILES_FOR_REDIRECT: &[&'static str] = &[
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

static CACHED_RESOLVE: LazyLock<(
    Option<unsafe extern "C" fn(*const c_char, c_int) -> DlopenReturn>,
    Option<PathBuf>,
)> = LazyLock::new(|| {
    let dlopen_next = unsafe { mem::transmute(dlsym(RTLD_NEXT, c"dlopen".as_ptr())) };
    let mut dl_info = unsafe { mem::zeroed::<DLInfo>() };
    let self_dir = if unsafe { dladdr(dlopen as _, &mut dl_info) } != 0 {
        unsafe { CStr::from_ptr(dl_info.dli_fname) }
            .to_str()
            .ok()
            .and_then(|path| {
                let mut pathbuf = PathBuf::from(path);
                if pathbuf.pop() {
                    Some(pathbuf)
                } else {
                    None
                }
            })
    } else {
        None
    };
    (dlopen_next, self_dir)
});

type DlopenReturn = Result<NonNull<c_void>, ()>;

const _: fn() = || {
    let _ = std::mem::transmute::<*mut c_void, DlopenReturn>;
};

#[no_mangle]
unsafe extern "C" fn dlopen(filename: *const c_char, flags: c_int) -> DlopenReturn {
    match &*CACHED_RESOLVE {
        (Some(dlopen_next), self_dir) => dlopen_redirect(*dlopen_next, self_dir, filename, flags)
            .or_else(|| dlopen_next(filename, flags).ok())
            .ok_or(()),
        (None, _) => Err(()),
    }
}

unsafe fn dlopen_redirect(
    dlopen_next: unsafe extern "C" fn(*const c_char, c_int) -> DlopenReturn,
    basedir: &Option<PathBuf>,
    input_path: *const c_char,
    flags: c_int,
) -> Option<NonNull<c_void>> {
    let input_path = CStr::from_ptr(input_path).to_str().ok()?;
    let replaced_file = FILES_FOR_REDIRECT.into_iter().find_map(|file| {
        if input_path.ends_with(file) {
            Some(file)
        } else {
            None
        }
    })?;
    let mut path = basedir.as_ref()?.clone();
    path.push(replaced_file);
    let mut path = path.into_os_string().into_string().ok()?.into_bytes();
    path.push(0);
    unsafe { dlopen_next(path.as_ptr() as _, flags) }.ok()
}
