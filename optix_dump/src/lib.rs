#[macro_use]
extern crate lazy_static;

use cuda_types::*;
use paste::paste;
use std::io::Write;
use std::{
    ffi::{c_void, CStr},
    fs::File,
    mem, ptr,
    sync::{atomic::AtomicUsize, Mutex},
};
use winapi::{
    shared::{
        minwindef::{HINSTANCE, HMODULE},
        ntdef::LPCWSTR,
    },
    um::libloaderapi::{GetModuleHandleA, GetProcAddress, LoadLibraryW},
};

mod eptx;

optix_base::optix_type_declarations!();
optix_base::optix6_type_declarations!();

macro_rules! optix_fn_table {
        ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* $(,)? ) -> $ret_type:ty);*) => {
            trait OptixFunctionTableTrait {
                $(
                    unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                        let mut global = match GLOBAL_STATE.lock() {
                            Ok(global) => global,
                            Err(_) => return <$ret_type as DefaultError>::error(),
                        };
                        let global = global.as_mut().unwrap();
                        panic!()
                        //(global.fn_table).$fn_name.unwrap() ( $( $arg_id),* )
                    }
                )*
            }

            struct DumpOptixFunctionTable;

            impl OptixFunctionTableTrait for DumpOptixFunctionTable {

            }

            fn get_table<T:OptixFunctionTableTrait>() -> OptixFunctionTable {
                OptixFunctionTable {
                    $(
                        $fn_name: Some(T::$fn_name),
                    )*
                }
            }
        };
}

optix_base::optix_function_declarations!(optix_fn_table);

macro_rules! optix6_fn {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* $(,)? ) -> $ret_type:ty);*) => {
        $(
            #[no_mangle]
            unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                let fn_ptr = GlobalState::with_state(|global| {
                    writeln!(&mut global.log_file, stringify!($fn_name));
                    GetProcAddress(global.optix6_handle, concat!(stringify!($fn_name), "\0").as_ptr() as _ )
                });
                let fn_ptr = match fn_ptr {
                    Some(f) if f == ptr::null_mut() => panic!(stringify!($fn_name)),
                    Some(f) => f,
                    None => panic!(stringify!($fn_name))
                };
                let fn_ptr = mem::transmute::<_, unsafe extern $abi fn ( $( $arg_type),* ) -> $ret_type>(fn_ptr);
                (fn_ptr)( $( $arg_id ),* )
            }
        )*
    };
}

macro_rules! optix6_fn_override {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* $(,)? ) -> $ret_type:ty);*) => {
        $(
            #[no_mangle]
            unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                let fn_ptr = GlobalState::with_state(|global| {
                    writeln!(&mut global.log_file, stringify!($fn_name)).unwrap();
                    GetProcAddress(global.optix6_handle, concat!(stringify!($fn_name), "\0").as_ptr() as _ )
                });
                let fn_ptr = match fn_ptr {
                    Some(f) if f == ptr::null_mut() => panic!(stringify!($fn_name)),
                    Some(f) => f,
                    None => panic!(stringify!($fn_name))
                };
                let fn_ptr = mem::transmute::<_, unsafe extern $abi fn ( $( $arg_type),* ) -> $ret_type>(fn_ptr);
                let result = (fn_ptr)( $( $arg_id ),* );
                (paste! { [<$fn_name _Post >] }) ( $( $arg_id ),* );
                result
            }
        )*
    };
}

optix_base::optix6_function_declarations!(
    optix6_fn,
    optix6_fn_override,
    [
        rtContextGetAttribute,
        rtContextSetAttribute,
        rtContextLaunch2D,
        rtGeometryTrianglesSetFlagsPerMaterial,
        rtProgramCreateFromPTXString,
        rtGeometryInstanceSetMaterialCount,
        rtBufferSetFormat
    ]
);

trait DefaultError {
    fn error() -> Self;
}

impl DefaultError for OptixResult {
    fn error() -> Self {
        OptixResult::OPTIX_ERROR_INTERNAL_ERROR
    }
}

impl DefaultError for RTresult {
    fn error() -> Self {
        RTresult::RT_ERROR_UNKNOWN
    }
}

impl DefaultError for *const i8 {
    fn error() -> Self {
        ptr::null()
    }
}

impl DefaultError for () {
    fn error() -> Self {
        ()
    }
}

#[allow(non_snake_case)]
struct GlobalState {
    log_file: File,
    optix6_handle: HINSTANCE,
    optix_salt: Vec<u8>,
    vendor_salt: Vec<u8>,
    vendor_key: Vec<u8>,
    /*
    fn_table: OptixFunctionTable,
    optixQueryFunctionTable: fn(
        abi_id: c_int,
        num_options: c_uint,
        option_keys: *const c_void,
        option_values: *mut *const c_void,
        function_table: *mut OptixFunctionTable,
        size_of_table: usize,
    ) -> OptixResult,
     */
}

impl GlobalState {
    unsafe fn with_state<T>(f: impl FnOnce(&mut GlobalState) -> T) -> Option<T> {
        let mut global = match GLOBAL_STATE.lock() {
            Ok(global) => global,
            Err(_) => return None,
        };
        let global = match &mut *global {
            Some(global) => global,
            None => {
                let redirect_handle = GetModuleHandleA(b"zluda_redirect\0".as_ptr() as _);
                let load_library: unsafe extern "system" fn(LPCWSTR) -> HMODULE =
                    if redirect_handle == ptr::null_mut() {
                        LoadLibraryW
                    } else {
                        mem::transmute(GetProcAddress(
                            redirect_handle,
                            b"ZludaLoadLibraryW_NoRedirect\0".as_ptr() as _,
                        ))
                    };
                //let optix_handle =
                //    load_library(OPTIX_PATH.encode_utf16().collect::<Vec<_>>().as_ptr());
                let optix6_handle =
                    load_library(OPTIX6_PATH.encode_utf16().collect::<Vec<_>>().as_ptr());
                if optix6_handle == std::ptr::null_mut() {
                    return None;
                }
                /*
                let optixQueryFunctionTable =
                    GetProcAddress(optix_handle, "optixQueryFunctionTable\0".as_ptr() as _);
                if optixQueryFunctionTable == ptr::null_mut() {
                    return None;
                }
                let optixQueryFunctionTable = mem::transmute::<
                    _,
                    fn(
                        abi_id: c_int,
                        num_options: c_uint,
                        option_keys: *const c_void,
                        option_values: *mut *const c_void,
                        function_table: *mut OptixFunctionTable,
                        size_of_table: usize,
                    ) -> OptixResult,
                >(optixQueryFunctionTable);
                let mut fn_table = mem::zeroed::<OptixFunctionTable>();
                let error = optixQueryFunctionTable(
                    55,
                    0,
                    ptr::null(),
                    ptr::null_mut(),
                    &mut fn_table,
                    mem::size_of::<OptixFunctionTable>(),
                );
                if error != OptixResult::OPTIX_SUCCESS {
                    return None;
                }
                global.insert(GlobalState {
                    optix6_handle,
                    fn_table,
                    optixQueryFunctionTable,
                })
                 */
                let log_file = std::fs::File::create(OPTIX_LOG).unwrap();
                //let log_file = std::io::stdout();
                global.insert(GlobalState {
                    log_file,
                    optix6_handle,
                    optix_salt: Vec::new(),
                    vendor_salt: Vec::new(),
                    vendor_key: Vec::new(),
                    //fn_table: mem::zeroed(),
                    //optixQueryFunctionTable: mem::transmute(ptr::null_mut::<()>()),
                })
            }
        };
        Some(f(global))
    }
}

unsafe impl Send for GlobalState {}

lazy_static! {
    static ref GLOBAL_STATE: Mutex<Option<GlobalState>> = Mutex::new(None);
}

const OPTIX_PATH: &'static str = "C:\\Windows\\System32\\DriverStore\\FileRepository\\nvmdui.inf_amd64_0085e40375a3f44a\\nvoptix.dll\0";
const OPTIX6_PATH: &'static str = "C:\\dev\\OptiX SDK 6.5.0\\bin64\\optix.6.5.0.dll\0";
//"C:\\ProgramData\\Autodesk\\ApplicationPlugins\\MAXtoA_2022\\optix.6.6.0.dll\0";
const OPTIX_LOG: &'static str = r#"C:\temp\optix.log"#;

/*
#[no_mangle]
unsafe extern "C" fn optixQueryFunctionTable(
    abi_id: c_int,
    num_options: c_uint,
    option_keys: *const c_void,
    option_values: *mut *const c_void,
    function_table: *mut OptixFunctionTable,
    size_of_table: usize,
) -> OptixResult {
    //file_append(OPTIX_LOG, "optixQueryFunctionTable");
    if GlobalState::with_state(|global| {}).is_none() {
        return DefaultError::error();
    }
    *function_table = get_table::<DumpOptixFunctionTable>();
    OptixResult::OPTIX_SUCCESS
}
 */

static mut PROGRAM_COUNTER: AtomicUsize = AtomicUsize::new(1);

pub(crate) unsafe extern "C" fn rtProgramCreateFromPTXString_Post(
    _context: RTcontext,
    ptx: *const ::std::os::raw::c_char,
    programName: *const ::std::os::raw::c_char,
    _program: *mut RTprogram,
) {
    GlobalState::with_state(|global| {
        writeln!(&mut global.log_file, "{:?}", CStr::from_ptr(programName)).unwrap();
        let program_number = PROGRAM_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let text = CStr::from_ptr(ptx).to_bytes();
        if text.starts_with(b"eptx0001") {
            let mut encoded_content = text.to_vec();
            let encoded_file = format!("C:\\temp\\optix_dump\\module_{}.eptx", program_number);
            std::fs::write(encoded_file, &encoded_content).unwrap();
            let decoded_content = eptx::decode_ptx(
                &mut encoded_content[..],
                &global.optix_salt[..],
                &global.vendor_salt[..],
                &global.vendor_key[..],
            );
            let decoded_file = format!("C:\\temp\\optix_dump\\module_{}.ptx", program_number);
            std::fs::write(&decoded_file, decoded_content).unwrap();
            // INJECTING CUSTOM PTX
            //if &*CStr::from_ptr(programName).to_string_lossy() == "exception" {
            //    let fn_ptr = GetProcAddress(
            //        global.optix6_handle,
            //        "rtProgramCreateFromPTXString\0".as_ptr() as _,
            //    );
            //    let fn_ptr = mem::transmute::<
            //        _,
            //        unsafe extern "C" fn(
            //            RTcontext,
            //            *const ::std::os::raw::c_char,
            //            *const ::std::os::raw::c_char,
            //            *mut RTprogram,
            //        ) -> OptixResult,
            //    >(fn_ptr);
            //    let mut encoded = std::fs::read("C:\\dev\\module_1.ptx").unwrap();
            //    encoded = eptx::encode_ptx(
            //        &mut encoded,
            //        &global.optix_salt[..],
            //        &global.vendor_salt[..],
            //        &global.vendor_key[..],
            //    );
            //    encoded.push(0);
            //    let result = fn_ptr(_context, encoded.as_ptr() as _, programName, _program);
            //    if result != OptixResult::OPTIX_SUCCESS {
            //        panic!("{:?}", result);
            //    }
            //}
        } else {
            let file = format!("C:\\temp\\optix_dump\\module_{}.eptx", program_number);
            std::fs::write(file, text).unwrap();
        }
    });
}

pub(crate) unsafe extern "C" fn rtContextGetAttribute_Post(
    context: RTcontext,
    attrib: RTcontextattribute,
    size: RTsize,
    p: *mut ::std::os::raw::c_void,
) {
    let fn_ptr = GlobalState::with_state(|global| {
        save_attribute(global, attrib, size, p);
    });
}

pub(crate) unsafe extern "C" fn rtContextSetAttribute_Post(
    context: RTcontext,
    attrib: RTcontextattribute,
    size: RTsize,
    p: *const ::std::os::raw::c_void,
) {
    let fn_ptr = GlobalState::with_state(|global| {
        save_attribute(global, attrib, size, p);
    });
}

pub(crate) unsafe extern "C" fn rtContextLaunch2D_Post(
    context: RTcontext,
    entry_point_index: ::std::os::raw::c_uint,
    width: u64,
    height: u64,
) {
    let fn_ptr = GlobalState::with_state(|global| {
        writeln!(&mut global.log_file, "({}, {})", width, height).unwrap();
    });
}

pub(crate) unsafe extern "C" fn rtGeometryTrianglesSetFlagsPerMaterial_Post(
    geometrytriangles: RTgeometrytriangles,
    materialIndex: ::std::os::raw::c_uint,
    flags: RTgeometryflags,
) {
    GlobalState::with_state(|global| {
        writeln!(&mut global.log_file, "{}", flags.0).unwrap();
    });
}

unsafe fn save_attribute(
    global: &mut GlobalState,
    attrib: RTcontextattribute,
    size: RTsize,
    p: *const c_void,
) {
    let (attrib_name, vec) = match attrib {
        RTcontextattribute::RT_CONTEXT_ATTRIBUTE_OPTIX_SALT => {
            ("RT_CONTEXT_ATTRIBUTE_OPTIX_SALT", &mut global.optix_salt)
        }
        RTcontextattribute::RT_CONTEXT_ATTRIBUTE_VENDOR_SALT => {
            ("RT_CONTEXT_ATTRIBUTE_VENDOR_SALT", &mut global.vendor_salt)
        }
        RTcontextattribute::RT_CONTEXT_ATTRIBUTE_PUBLIC_VENDOR_KEY => (
            "RT_CONTEXT_ATTRIBUTE_PUBLIC_VENDOR_KEY",
            &mut global.vendor_key,
        ),
        _ => return,
    };
    let size = size as usize;
    let mut value = vec![0u8; size];
    value.copy_from_slice(std::slice::from_raw_parts(p as *const u8, size));
    writeln!(&mut global.log_file, "{}: {:?}", attrib_name, value).unwrap();
    *vec = value;
}

pub(crate) unsafe extern "C" fn rtGeometryInstanceSetMaterialCount_Post(
    geometryinstance: RTgeometryinstance,
    count: ::std::os::raw::c_uint,
) {
    GlobalState::with_state(|global| {
        writeln!(&mut global.log_file, "{}", count).unwrap();
    });
}

pub(crate) unsafe extern "C" fn rtBufferSetFormat_Post(_buffer: RTbuffer, format: RTformat) {
    GlobalState::with_state(|global| {
        writeln!(&mut global.log_file, "{:?}", format).unwrap();
    });
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tabe_size() {
        assert_eq!(std::mem::size_of::<OptixFunctionTable>(), 0x158);
    }
}
