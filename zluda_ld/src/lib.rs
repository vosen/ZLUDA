use std::{
    ffi::{c_char, c_int, c_long, c_uint, c_void, CStr},
    mem,
    path::PathBuf,
    sync::{LazyLock, Mutex},
};

unsafe extern "C" {
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
    "libcublas.so.12",
    "libcublas.so",
    "libcublasLt.so.12",
    "libcublasLt.so",
    "libcuda.so.1",
    "libcuda.so",
    "libcudnn.so.9",
    "libcudnn.so",
    "libcufft.so.11",
    "libcufft.so",
    "libcusparse.so.12",
    "libcusparse.so",
    "libnvidia-ml.so.1",
    "libnvidia-ml.so",
];

// Global state, caching some computations that would be otherwise repeated
struct GlobalState {
    /// The full paths of the file names from `FILES_FOR_REDIRECT` that will be used for redirection
    replacement_paths: Option<[Vec<u8>; FILES_FOR_REDIRECT.len()]>,
    cookies: Mutex<[usize; FILES_FOR_REDIRECT.len() / 2]>,
}

static GLOBAL_STATE: LazyLock<GlobalState> = LazyLock::new(|| {
    let mut self_dlinfo = unsafe { mem::zeroed::<DLInfo>() };
    let replacement_paths = if unsafe { dladdr(la_version as _, &mut self_dlinfo) } != 0 {
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
    GlobalState {
        replacement_paths,
        cookies: Mutex::new([0; FILES_FOR_REDIRECT.len() / 2]),
    }
});

#[no_mangle]
unsafe extern "C" fn la_version(_: std::ffi::c_uint) -> std::ffi::c_uint {
    const LAV_CURRENT: u32 = 2;
    LAV_CURRENT
}

#[no_mangle]
unsafe extern "C" fn la_objsearch(
    name: *const c_char,
    cookie: *mut usize,
    _flags: std::ffi::c_uint,
) -> *const c_char {
    match la_objsearch_impl(name, cookie) {
        Some(new_name) => new_name.as_ptr().cast(),
        None => name,
    }
}

unsafe fn la_objsearch_impl(
    name: *const c_char,
    requesting_cookie: *mut usize,
) -> Option<&'static [u8]> {
    let GlobalState {
        replacement_paths,
        cookies,
    } = &*GLOBAL_STATE;
    let requesting_cookie = requesting_cookie as usize;
    let input_path = CStr::from_ptr(name).to_str().ok()?;
    let replacement_paths = replacement_paths.as_ref()?;
    let index = FILES_FOR_REDIRECT
        .into_iter()
        .enumerate()
        .find_map(|(index, file)| {
            if input_path.ends_with(file) {
                Some(index)
            } else {
                None
            }
        })?;
    let known_cookie = { cookies.lock().ok()?[index / 2] };
    if known_cookie == requesting_cookie {
        return None;
    }
    Some(&*replacement_paths[index])
}

type Lmid = c_long;

#[no_mangle]
unsafe extern "C" fn la_objopen(map: *mut link_map, _lmid: Lmid, cookie: *mut usize) -> c_uint {
    save_cookie(map, cookie);
    0
}

unsafe fn save_cookie(map: *mut link_map, cookie: *mut usize) -> Option<()> {
    let map = map.as_ref()?;
    let obj_name = CStr::from_ptr(map.l_name).to_str().ok()?;
    let index = FILES_FOR_REDIRECT
        .into_iter()
        .enumerate()
        .find_map(|(index, file)| {
            if obj_name.ends_with(file) {
                Some(index / 2)
            } else {
                None
            }
        })?;
    GLOBAL_STATE
        .cookies
        .lock()
        .ok()?
        .get_mut(index)
        .map(|saved_cookie| {
            *saved_cookie = cookie as usize;
        });
    Some(())
}

// Public portion of glibc's struct link_map. Additional private fields omitted.
#[allow(non_camel_case_types)]
#[repr(C)]
struct link_map {
    /// Difference between the address in the ELF file and the address in memory (load bias)
    l_addr: usize,
    /// Absolute pathname where object was found
    l_name: *const c_char,
    /// Dynamic section of the shared object (opaque to us)
    l_ld: *mut c_void,
    /// Next object in the loaded objects chain
    l_next: *mut link_map,
    /// Previous object in the loaded objects chain
    l_prev: *mut link_map,
    // Additional private / internal glibc fields follow in the real definition.
}
