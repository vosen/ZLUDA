use libloading::Library;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::ptr;

// Version history taken from here: https://developer.nvidia.com/cuda-toolkit-archive
static CUDA_VERSIONS: &[&'static str] = &[
    "12.2.0", "12.1.1", "12.1.0", "12.0.1", "12.0.0", "11.8.0", "11.7.1", "11.7.0", "11.6.2",
    "11.6.1", "11.6.0", "11.5.2", "11.5.1", "11.5.0", "11.4.4", "11.4.3", "11.4.2", "11.4.1",
    "11.4.0", "11.3.1", "11.3.0", "11.2.2", "11.2.1", "11.2.0", "11.1.1", "11.1.0", "11.0.3",
    "11.0.2", "11.0.1", "11.0", "10.2", "10.1", "10.1", "10.1", "10.0", "9.2", "9.1", "9.0", "8.0",
    "7.5", "7.0", "6.5", "6.0", "5.5", "5.0", "4.2", "4.1", "4.0", "3.2", "3.1", "3.0", "2.3",
    "2.2", "2.1", "2.0", "1.1", "1.0",
];

struct FnVersionTable<'a> {
    fn_: &'a str,
    flag: u64,
    versions: Vec<(u32, &'a str)>,
}

impl<'a> FnVersionTable<'a> {
    fn new(fn_: &'a str, flag: u64) -> Self {
        Self {
            fn_,
            flag,
            versions: Vec::new(),
        }
    }

    fn push(&mut self, ver: u32, name: &'a str) {
        if Some(name) == self.versions.last().map(|(_, n)| *n) {
            return;
        }
        self.versions.push((ver, name));
    }

    fn print(&self) {
        if self.versions.len() == 0 {
            return;
        }
        println!("    (b\"{}\", {}) => {{", &self.fn_, self.flag);
        for (version, name) in self.versions.iter().rev() {
            println!("        if version >= {version} {{");
            println!("            return {name} as _;");
            println!("        }}");
        }
        println!("        usize::MAX as _");
        println!("    }}");
    }
}

fn main() {
    unsafe { main_impl() }
}

unsafe fn main_impl() {
    let all_exports = os::get_nvcuda_exports();
    let mut cuda_versions = CUDA_VERSIONS
        .iter()
        .map(cuda_version_to_integer)
        .collect::<Vec<_>>();
    cuda_versions.sort_unstable();
    let cuda = Library::new(os::CUDA_PATH).unwrap();
    let mut cu_get_proc_address = cuda
        .get::<unsafe extern "C" fn(
            symbol: *const ::std::os::raw::c_char,
            pfn: *mut *mut ::std::os::raw::c_void,
            cudaVersion: ::std::os::raw::c_int,
            flags: u64,
        ) -> u32>(b"cuGetProcAddress\0")
        .unwrap()
        .into_raw();
    let cuda_impl = if cfg!(windows) {
        // Done purely to force load of nvcuda64.dll on Windows
        cu_get_proc_address("cuInit\0".as_ptr() as _, &mut ptr::null_mut(), 0, 0);
        let nvcuda64 = Library::new(os::CUDA_IMPL_LIB).unwrap();
        cu_get_proc_address = nvcuda64
            .get::<unsafe extern "C" fn(
                symbol: *const ::std::os::raw::c_char,
                pfn: *mut *mut ::std::os::raw::c_void,
                cudaVersion: ::std::os::raw::c_int,
                flags: u64,
            ) -> u32>(b"cuGetProcAddress\0")
            .unwrap()
            .into_raw();
        nvcuda64
    } else {
        Library::new(os::CUDA_IMPL_LIB).unwrap()
    };
    let (nvcuda_exports, cuda_impl_exports) = get_impl_fns(&cuda_impl, all_exports);
    println!("// GENERATED AUTOMATICALLY BY process_address_table, DON'T CHANGE MANUALLY");
    println!("match (name, flag) {{");
    for export in nvcuda_exports.iter() {
        for flag in [0, 1, 2] {
            let mut ver_table = FnVersionTable::new(export, flag);
            for ver in cuda_versions.iter().copied() {
                let mut fnptr = ptr::null_mut();
                let error = cu_get_proc_address(export.as_ptr() as _, &mut fnptr, ver as i32, flag);
                if error == 500 {
                    continue;
                }
                assert_eq!(0, error);
                let fn_name = &cuda_impl_exports[&fnptr];
                ver_table.push(ver, fn_name);
            }
            ver_table.print();
        }
    }
    println!("    _ => std::ptr::null_mut()");
    println!("}}");
}

fn cuda_version_to_integer(ver: &&str) -> u32 {
    let parts = ver
        .split('.')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let version_parts = if parts.len() == 2 {
        [parts[0], parts[1], 0]
    } else {
        [parts[0], parts[1], parts[2]]
    };
    (1000 * version_parts[0]) + (10 * version_parts[1]) + version_parts[2]
}

unsafe fn get_impl_fns(
    cuda_impl: &Library,
    all_exports: BTreeSet<String>,
) -> (BTreeSet<String>, HashMap<*mut std::ffi::c_void, String>) {
    let mut unversioned_symbols = BTreeSet::new();
    let mut addressed = HashMap::with_capacity(all_exports.len());
    for mut symbol in all_exports {
        if symbol.starts_with("cuD3D")
            || symbol.starts_with("cuGraphicsD3D")
            || symbol.starts_with("cuGraphicsVDPAU")
            || symbol.starts_with("cudbg")
            || symbol.starts_with("cuVDPAU")
            || symbol.starts_with("cuWGL")
            || symbol.starts_with("cuEGL")
            || symbol.starts_with("cuGraphicsEGL")
            || symbol.contains("NvSci")
            || symbol.ends_with("EglFrame")
        {
            continue;
        }
        symbol.push('\0');
        let fn_ptr = cuda_impl
            .get::<*mut std::ffi::c_void>(symbol.as_bytes())
            .unwrap();
        symbol.truncate(symbol.len() - 1);
        addressed.insert(*fn_ptr, symbol.clone());
        if let Some(version_suffix_idx) = symbol.find("_") {
            assert!(
                symbol.as_bytes()[version_suffix_idx + 2].is_ascii_digit()
                    || symbol.ends_with("_ptsz")
                    || symbol.ends_with("_ptds")
            );
            symbol.truncate(version_suffix_idx);
        }
        unversioned_symbols.insert(symbol);
    }
    (unversioned_symbols, addressed)
}

#[cfg(windows)]
mod os {
    use detours_sys::*;
    use std::collections::BTreeSet;
    use std::ffi::CStr;
    use windows::{core::PCSTR, imp::LoadLibraryA};

    pub const CUDA_PATH: &'static str = "C:\\Windows\\System32\\nvcuda.dll";
    pub const CUDA_IMPL_LIB: &'static str = "nvcuda64";

    pub fn get_nvcuda_exports() -> BTreeSet<String> {
        let nvcuda = unsafe { LoadLibraryA(PCSTR("C:\\Windows\\System32\\nvcuda.dll\0".as_ptr())) };
        let mut nvcuda_exports = BTreeSet::new();
        assert_eq!(1, unsafe {
            DetourEnumerateExports(
                nvcuda as _,
                &mut nvcuda_exports as *mut BTreeSet<_> as _,
                Some(get_unversioned_export),
            )
        });
        nvcuda_exports
    }

    unsafe extern "stdcall" fn get_unversioned_export(
        context: PVOID,
        _ordinal: ULONG,
        name: LPCSTR,
        _code: PVOID,
    ) -> i32 {
        let exports = context as *mut BTreeSet<String>;
        let name = CStr::from_ptr(name).to_str().unwrap().to_string();
        (&mut *exports).insert(name);
        1
    }
}

#[cfg(unix)]
mod os {
    use std::collections::BTreeSet;
    use std::io::BufRead;
    use std::process::Command;

    pub const CUDA_PATH: &'static str = "/usr/lib/x86_64-linux-gnu/libcuda.so";
    pub const CUDA_IMPL_LIB: &'static str = "libcuda.so";

    pub fn get_nvcuda_exports() -> BTreeSet<String> {
        let export_list = Command::new("nm")
            .args([
                "nm",
                "-D",
                "-g",
                "--defined-only",
                "--format=just-symbols",
                CUDA_PATH,
            ])
            .output()
            .unwrap();
        export_list
            .stdout
            .lines()
            .into_iter()
            .collect::<Result<_, _>>()
            .unwrap()
    }
}
