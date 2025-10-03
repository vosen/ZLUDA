use std::{
    ffi::{c_char, c_int, c_long, c_uint, c_void, CStr, OsStr},
    path::Path,
    sync::{LazyLock, Mutex},
};

unsafe extern "C" {
    fn dl_iterate_phdr(
        callback: extern "C" fn(info: *mut DlPhdrInfo, size: usize, data: *mut c_void) -> c_int,
        data: *mut c_void,
    ) -> c_int;
}

#[repr(C)]
struct DlPhdrInfo {
    dlpi_addr: usize,
    dlpi_name: *const c_char,
    dlpi_phdr: *const c_void,
    dlpi_phnum: u16,
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
    // The full paths of the file names from `FILES_FOR_REDIRECT` that will be used for redirection
    replacement_paths: Option<[Vec<u8>; FILES_FOR_REDIRECT.len()]>,
    // List of cookies saved for each redirected file, to avoid self-redirecting
    // when e.g. zluda_trace_blas (libcuda.so) tries to load the real libcublas.so
    cookies: Mutex<[usize; FILES_FOR_REDIRECT.len() / 2]>,
}

static GLOBAL_STATE: LazyLock<GlobalState> = LazyLock::new(|| {
    let replacement_paths = if let Some(self_path) = get_self_path() {
        self_path.parent().and_then(|path| {
            Some(FILES_FOR_REDIRECT.map(|file| {
                let mut buffer = path.join(file).into_os_string().into_encoded_bytes();
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

// I would prefer to just use dladdr(...), but for some reason it segfaults on
// Ubuntu 22.04, even though it works just fine on Ubuntu 24.04
fn get_self_path() -> Option<&'static Path> {
    type IteratorResult = &'static Path;
    fn get_self_path(info: *mut DlPhdrInfo) -> Option<IteratorResult> {
        let info = unsafe { info.as_ref() }?;
        if info.dlpi_addr > la_version as usize {
            return None;
        }
        let path = Path::new(unsafe { CStr::from_ptr(info.dlpi_name) }.to_str().ok()?);
        if path.file_stem() == Some(OsStr::new("zluda_ld")) {
            Some(path)
        } else {
            None
        }
    }
    extern "C" fn iterator(info: *mut DlPhdrInfo, _size: usize, data: *mut c_void) -> c_int {
        if let Some(path) = get_self_path(info) {
            if let Some(output) = unsafe { data.cast::<IteratorResult>().as_mut() } {
                *output = path;
            }
            1
        } else {
            0
        }
    }
    let mut result: Option<IteratorResult> = None;
    unsafe { dl_iterate_phdr(iterator, std::ptr::from_mut(&mut result).cast()) };
    result
}

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
    let requesting_cookie = requesting_cookie as usize;
    let input_path = CStr::from_ptr(name).to_str().ok()?;
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
    let GlobalState {
        replacement_paths,
        cookies,
    } = &*GLOBAL_STATE;
    let replacement_paths = replacement_paths.as_ref()?;
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
        })
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
