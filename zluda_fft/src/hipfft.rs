use cuda_types::cufft::{
    cudaStream_t, cufftComplex, cufftDoubleComplex, cufftDoubleReal, cufftError_t, cufftReal,
    cufftResult, cufftType, libraryPropertyType,
};
use std::{ffi::c_void, sync::OnceLock};

pub(crate) type Handle = *mut c_void;
type HipfftResult = u32;
type HipfftType = u32;
type HipfftProperty = u32;

macro_rules! vtable {
    ($($name:ident: $type_:ty),* $(,)?) => {
        #[allow(non_snake_case)]
        pub(crate) struct Vtable {
            _library: Library,
            $(pub(crate) $name: $type_,)*
        }

        impl Vtable {
            unsafe fn new() -> std::result::Result<Self, cufftError_t> {
                let library = unsafe { load_library()? };
                Ok(Self {
                    $($name: unsafe {
                        *library
                            .get(concat!(stringify!($name), "\0").as_bytes())
                            .map_err(|_| cufftError_t::MISSING_DEPENDENCY)?
                    },)*
                    _library: library,
                })
            }
        }
    };
}

vtable! {
    hipfftPlan1d: unsafe extern "C" fn(*mut Handle, i32, HipfftType, i32) -> HipfftResult,
    hipfftPlan2d: unsafe extern "C" fn(*mut Handle, i32, i32, HipfftType) -> HipfftResult,
    hipfftPlan3d: unsafe extern "C" fn(*mut Handle, i32, i32, i32, HipfftType) -> HipfftResult,
    hipfftPlanMany: unsafe extern "C" fn(*mut Handle, i32, *mut i32, *mut i32, i32, i32, *mut i32, i32, i32, HipfftType, i32) -> HipfftResult,
    hipfftCreate: unsafe extern "C" fn(*mut Handle) -> HipfftResult,
    hipfftMakePlan1d: unsafe extern "C" fn(Handle, i32, HipfftType, i32, *mut usize) -> HipfftResult,
    hipfftMakePlan2d: unsafe extern "C" fn(Handle, i32, i32, HipfftType, *mut usize) -> HipfftResult,
    hipfftMakePlan3d: unsafe extern "C" fn(Handle, i32, i32, i32, HipfftType, *mut usize) -> HipfftResult,
    hipfftMakePlanMany: unsafe extern "C" fn(Handle, i32, *mut i32, *mut i32, i32, i32, *mut i32, i32, i32, HipfftType, i32, *mut usize) -> HipfftResult,
    hipfftMakePlanMany64: unsafe extern "C" fn(Handle, i32, *mut i64, *mut i64, i64, i64, *mut i64, i64, i64, HipfftType, i64, *mut usize) -> HipfftResult,
    hipfftEstimate1d: unsafe extern "C" fn(i32, HipfftType, i32, *mut usize) -> HipfftResult,
    hipfftEstimate2d: unsafe extern "C" fn(i32, i32, HipfftType, *mut usize) -> HipfftResult,
    hipfftEstimate3d: unsafe extern "C" fn(i32, i32, i32, HipfftType, *mut usize) -> HipfftResult,
    hipfftEstimateMany: unsafe extern "C" fn(i32, *mut i32, *mut i32, i32, i32, *mut i32, i32, i32, HipfftType, i32, *mut usize) -> HipfftResult,
    hipfftGetSize1d: unsafe extern "C" fn(Handle, i32, HipfftType, i32, *mut usize) -> HipfftResult,
    hipfftGetSize2d: unsafe extern "C" fn(Handle, i32, i32, HipfftType, *mut usize) -> HipfftResult,
    hipfftGetSize3d: unsafe extern "C" fn(Handle, i32, i32, i32, HipfftType, *mut usize) -> HipfftResult,
    hipfftGetSizeMany: unsafe extern "C" fn(Handle, i32, *mut i32, *mut i32, i32, i32, *mut i32, i32, i32, HipfftType, i32, *mut usize) -> HipfftResult,
    hipfftGetSizeMany64: unsafe extern "C" fn(Handle, i32, *mut i64, *mut i64, i64, i64, *mut i64, i64, i64, HipfftType, i64, *mut usize) -> HipfftResult,
    hipfftGetSize: unsafe extern "C" fn(Handle, *mut usize) -> HipfftResult,
    hipfftSetAutoAllocation: unsafe extern "C" fn(Handle, i32) -> HipfftResult,
    hipfftSetWorkArea: unsafe extern "C" fn(Handle, *mut c_void) -> HipfftResult,
    hipfftExecC2C: unsafe extern "C" fn(Handle, *mut cufftComplex, *mut cufftComplex, i32) -> HipfftResult,
    hipfftExecR2C: unsafe extern "C" fn(Handle, *mut cufftReal, *mut cufftComplex) -> HipfftResult,
    hipfftExecC2R: unsafe extern "C" fn(Handle, *mut cufftComplex, *mut cufftReal) -> HipfftResult,
    hipfftExecZ2Z: unsafe extern "C" fn(Handle, *mut cufftDoubleComplex, *mut cufftDoubleComplex, i32) -> HipfftResult,
    hipfftExecD2Z: unsafe extern "C" fn(Handle, *mut cufftDoubleReal, *mut cufftDoubleComplex) -> HipfftResult,
    hipfftExecZ2D: unsafe extern "C" fn(Handle, *mut cufftDoubleComplex, *mut cufftDoubleReal) -> HipfftResult,
    hipfftSetStream: unsafe extern "C" fn(Handle, cudaStream_t) -> HipfftResult,
    hipfftDestroy: unsafe extern "C" fn(Handle) -> HipfftResult,
    hipfftGetVersion: unsafe extern "C" fn(*mut i32) -> HipfftResult,
    hipfftGetProperty: unsafe extern "C" fn(HipfftProperty, *mut i32) -> HipfftResult,
}

pub(crate) fn library() -> std::result::Result<&'static Vtable, cufftError_t> {
    static VTABLE: OnceLock<std::result::Result<Vtable, cufftError_t>> = OnceLock::new();
    VTABLE
        .get_or_init(|| unsafe { Vtable::new() })
        .as_ref()
        .map_err(|error| *error)
}

pub(crate) fn status(result: HipfftResult) -> cufftResult {
    match result {
        0 => Ok(()),
        1 => Err(cufftError_t::INVALID_PLAN),
        2 => Err(cufftError_t::ALLOC_FAILED),
        3 => Err(cufftError_t::INVALID_TYPE),
        4 | 10 => Err(cufftError_t::INVALID_VALUE),
        5 | 12 => Err(cufftError_t::INTERNAL_ERROR),
        6 => Err(cufftError_t::EXEC_FAILED),
        7 => Err(cufftError_t::SETUP_FAILED),
        8 => Err(cufftError_t::INVALID_SIZE),
        9 => Err(cufftError_t::UNALIGNED_DATA),
        11 => Err(cufftError_t::INVALID_DEVICE),
        13 => Err(cufftError_t::NO_WORKSPACE),
        14 => Err(cufftError_t::NOT_IMPLEMENTED),
        16 => Err(cufftError_t::NOT_SUPPORTED),
        _ => Err(cufftError_t::INTERNAL_ERROR),
    }
}

pub(crate) fn transform_type(type_: cufftType) -> std::result::Result<HipfftType, cufftError_t> {
    match type_ {
        cufftType::CUFFT_C2C
        | cufftType::CUFFT_R2C
        | cufftType::CUFFT_C2R
        | cufftType::CUFFT_Z2Z
        | cufftType::CUFFT_D2Z
        | cufftType::CUFFT_Z2D => Ok(type_.0),
        _ => Err(cufftError_t::INVALID_TYPE),
    }
}

pub(crate) fn property_type(
    property: libraryPropertyType,
) -> std::result::Result<HipfftProperty, cufftError_t> {
    match property {
        libraryPropertyType::MAJOR_VERSION
        | libraryPropertyType::MINOR_VERSION
        | libraryPropertyType::PATCH_LEVEL => Ok(property.0),
        _ => Err(cufftError_t::INVALID_VALUE),
    }
}

#[cfg(windows)]
type Library = libloading::os::windows::Library;

#[cfg(windows)]
unsafe fn load_library() -> std::result::Result<Library, cufftError_t> {
    unsafe { load_library_from_names(&["hipfft.dll"], true) }
}

#[cfg(windows)]
unsafe fn load_library_from_names(
    names: &[&'static str],
    show_error: bool,
) -> std::result::Result<Library, cufftError_t> {
    let module = if show_error {
        unsafe { zluda_windows::try_load_from_self_or_hip_with_message(names) }
    } else {
        names
            .iter()
            .find_map(|name| unsafe { zluda_windows::try_load_from_self_or_hip(name) })
    }
    .ok_or(cufftError_t::MISSING_DEPENDENCY)?;
    Ok(unsafe { Library::from_raw(module.0 as _) })
}

#[cfg(unix)]
type Library = libloading::Library;

#[cfg(unix)]
unsafe fn load_library() -> std::result::Result<Library, cufftError_t> {
    unsafe { load_library_from_names(&["libhipfft.so", "libhipfft.so.0"], false) }
}

#[cfg(unix)]
unsafe fn load_library_from_names(
    names: &[&'static str],
    _show_error: bool,
) -> std::result::Result<Library, cufftError_t> {
    for name in names {
        if let Ok(library) = unsafe { Library::new(name) } {
            return Ok(library);
        }
    }
    Err(cufftError_t::MISSING_DEPENDENCY)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cuda_types::cufft::cufftResultConsts;

    #[test]
    fn maps_all_documented_statuses() {
        let expected = [
            (0, cufftResult::SUCCESS),
            (1, cufftResult::ERROR_INVALID_PLAN),
            (2, cufftResult::ERROR_ALLOC_FAILED),
            (3, cufftResult::ERROR_INVALID_TYPE),
            (4, cufftResult::ERROR_INVALID_VALUE),
            (5, cufftResult::ERROR_INTERNAL_ERROR),
            (6, cufftResult::ERROR_EXEC_FAILED),
            (7, cufftResult::ERROR_SETUP_FAILED),
            (8, cufftResult::ERROR_INVALID_SIZE),
            (9, cufftResult::ERROR_UNALIGNED_DATA),
            (10, cufftResult::ERROR_INVALID_VALUE),
            (11, cufftResult::ERROR_INVALID_DEVICE),
            (12, cufftResult::ERROR_INTERNAL_ERROR),
            (13, cufftResult::ERROR_NO_WORKSPACE),
            (14, cufftResult::ERROR_NOT_IMPLEMENTED),
            (16, cufftResult::ERROR_NOT_SUPPORTED),
            (u32::MAX, cufftResult::ERROR_INTERNAL_ERROR),
        ];
        for (hipfft, cufft) in expected {
            assert_eq!(status(hipfft), cufft);
        }
    }

    #[test]
    fn validates_transform_type() {
        for type_ in [
            cufftType::CUFFT_C2C,
            cufftType::CUFFT_R2C,
            cufftType::CUFFT_C2R,
            cufftType::CUFFT_Z2Z,
            cufftType::CUFFT_D2Z,
            cufftType::CUFFT_Z2D,
        ] {
            assert_eq!(transform_type(type_), Ok(type_.0));
        }
        assert_eq!(
            transform_type(cufftType(u32::MAX)),
            Err(cufftError_t::INVALID_TYPE)
        );
    }

    #[test]
    fn reports_missing_backend_library() {
        assert!(matches!(
            unsafe { load_library_from_names(&["zluda-test-missing-hipfft-library"], false) },
            Err(cufftError_t::MISSING_DEPENDENCY)
        ));
    }
}
