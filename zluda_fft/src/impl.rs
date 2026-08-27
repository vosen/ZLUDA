use crate::plan;
use cuda_types::cufft::*;
use rocfft_sys::rocfft_error;
use std::sync::{Mutex, OnceLock};

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> cufftResult {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> cufftResult {
    cufftResult::ERROR_NOT_SUPPORTED
}

fn rocfft() -> Result<&'static super::RocfftVtable, rocfft_error> {
    static LOCK: OnceLock<Result<super::RocfftVtable, rocfft_error>> = OnceLock::new();
    let unwrapped: &Result<super::RocfftVtable, rocfft_error> = LOCK.get_or_init(|| {
        let rocfft = unsafe { super::RocfftVtable::new()? };
        unsafe { rocfft.rocfft_setup() }?;
        Ok(rocfft)
    });
    unwrapped.as_ref().map_err(|x| *x)
}

struct GlobalState {
    registry: plan::Registry,
}

impl GlobalState {
    fn get() -> &'static Mutex<Self> {
        static LOCK: OnceLock<Mutex<GlobalState>> = OnceLock::new();
        LOCK.get_or_init(|| {
            Mutex::new(GlobalState {
                registry: plan::Registry::new(),
            })
        })
    }

    fn with<T>(f: impl FnOnce(&mut Self) -> Result<T, cufftError_t>) -> Result<T, cufftError_t> {
        let mut lock = Self::get()
            .lock()
            .map_err(|_| cufftError_t::INTERNAL_ERROR)?;
        f(&mut *lock)
    }
}

pub(crate) unsafe fn create(handle: &mut cufftHandle) -> Result<(), cufftError_t> {
    let plan = GlobalState::with(|state| Ok(state.registry.new_empty()))?;
    *handle = plan;
    Ok(())
}

pub(crate) unsafe fn make_plan1d(
    _plan: cufftHandle,
    _nx: i32,
    _type_: cufftType,
    _batch: i32,
    _work_size: *mut usize,
) -> Result<(), cufftError_t> {
    todo!()
}

pub(crate) unsafe fn plan1d(
    _plan: &mut cufftHandle,
    _nx: i32,
    _type_: cufftType,
    _batch: i32,
) -> Result<(), cufftError_t> {
    todo!()
}

/*
unsafe fn create_plan(
    output: *mut cufftHandle,
    create: impl FnOnce(&hipfft::Vtable, *mut hipfft::Handle) -> u32,
) -> cufftResult {
    if output.is_null() {
        return cufftResult::ERROR_INVALID_VALUE;
    }
    let library = hipfft::library()?;
    let mut backend = ptr::null_mut();
    hipfft::status(create(library, &mut backend))?;
    if backend.is_null() {
        return cufftResult::ERROR_INTERNAL_ERROR;
    }
    match plan::insert(backend) {
        Ok(handle) => {
            unsafe { output.write(handle) };
            Ok(())
        }
        Err(error) => {
            let _ = hipfft::status(unsafe { (library.hipfftDestroy)(backend) });
            Err(error)
        }
    }
}

pub(crate) unsafe fn cufftPlan1d(
    plan: *mut cufftHandle,
    nx: i32,
    type_: cufftType,
    batch: i32,
) -> cufftResult {
    if plan.is_null() {
        return cufftResult::ERROR_INVALID_VALUE;
    }

    let transform = match type_ {
        cufftType::CUFFT_R2C => rocfft_sys::rocfft_transform_type_e::rocfft_transform_type_real_forward,
        cufftType::CUFFT_C2R => rocfft_sys::rocfft_transform_type_e::rocfft_transform_type_real_inverse,
        cufftType::CUFFT_C2C => rocfft_sys::rocfft_transform_type_e::rocfft_transform_type_complex_forward,
        cufftType::CUFFT_D2Z => rocfft_sys::rocfft_transform_type_e::rocfft_transform_type_real_forward,
        cufftType::CUFFT_Z2D => rocfft_sys::rocfft_transform_type_e::rocfft_transform_type_real_inverse,
        cufftType::CUFFT_Z2Z => rocfft_sys::rocfft_transform_type_e::rocfft_transform_type_complex_forward,
        _ => return cufftResult::ERROR_INVALID_TYPE,
    };

    let precision = match type_ {
        cufftType::CUFFT_R2C
        | cufftType::CUFFT_C2R
        | cufftType::CUFFT_C2C => rocfft_sys::rocfft_precision_e::rocfft_precision_single,
        cufftType::CUFFT_D2Z
        | cufftType::CUFFT_Z2D
        | cufftType::CUFFT_Z2Z => rocfft_sys::rocfft_precision_e::rocfft_precision_double,
        _ => return cufftResult::ERROR_INVALID_TYPE,
    };

    let dimensions = 1usize;
    let lengths = [nx as usize];
    let mut backend = ptr::null_mut();
    let status = rocfft_sys::rocfft_plan_create(
        &mut backend,
        rocfft_sys::rocfft_result_placement_e::rocfft_placement_notinplace,
        transform,
        precision,
        dimensions,
        lengths.as_ptr(),
        batch as usize,
        ptr::null_mut(),
    );

    if status != rocfft_sys::rocfft_status_e::rocfft_status_success {
        return match status {
            rocfft_sys::rocfft_status_e::rocfft_status_invalid_arg_value => {
                cufftResult::ERROR_INVALID_VALUE
            }
            rocfft_sys::rocfft_status_e::rocfft_status_invalid_dimensions => {
                cufftResult::ERROR_INVALID_SIZE
            }
            rocfft_sys::rocfft_status_e::rocfft_status_invalid_array_type => {
                cufftResult::ERROR_INVALID_TYPE
            }
            rocfft_sys::rocfft_status_e::rocfft_status_invalid_work_buffer => {
                cufftResult::ERROR_NO_WORKSPACE
            }
            _ => cufftResult::ERROR_INTERNAL_ERROR,
        };
    }

    match plan::insert(backend) {
        Ok(handle) => {
            plan.write(handle);
            Ok(())
        }
        Err(error) => {
            let _ = rocfft_sys::rocfft_plan_destroy(backend);
            Err(error)
        }
    }
}

pub(crate) unsafe fn cufftPlan2d(
    plan: *mut cufftHandle,
    nx: i32,
    ny: i32,
    type_: cufftType,
) -> cufftResult {
    let type_ = hipfft::transform_type(type_)?;
    unsafe {
        create_plan(plan, |lib, backend| {
            (lib.hipfftPlan2d)(backend, nx, ny, type_)
        })
    }
}

pub(crate) unsafe fn cufftPlan3d(
    plan: *mut cufftHandle,
    nx: i32,
    ny: i32,
    nz: i32,
    type_: cufftType,
) -> cufftResult {
    let type_ = hipfft::transform_type(type_)?;
    unsafe {
        create_plan(plan, |lib, backend| {
            (lib.hipfftPlan3d)(backend, nx, ny, nz, type_)
        })
    }
}

pub(crate) unsafe fn cufftPlanMany(
    plan: *mut cufftHandle,
    rank: i32,
    n: *mut i32,
    inembed: *mut i32,
    istride: i32,
    idist: i32,
    onembed: *mut i32,
    ostride: i32,
    odist: i32,
    type_: cufftType,
    batch: i32,
) -> cufftResult {
    let type_ = hipfft::transform_type(type_)?;
    unsafe {
        create_plan(plan, |lib, backend| {
            (lib.hipfftPlanMany)(
                backend, rank, n, inembed, istride, idist, onembed, ostride, odist, type_, batch,
            )
        })
    }
}

pub(crate) unsafe fn cufftCreate(handle: *mut cufftHandle) -> cufftResult {
    unsafe { create_plan(handle, |lib, backend| (lib.hipfftCreate)(backend)) }
}

pub(crate) unsafe fn cufftMakePlan1d(
    plan: cufftHandle,
    nx: i32,
    type_: cufftType,
    batch: i32,
    workSize: *mut usize,
) -> cufftResult {
    let type_ = hipfft::transform_type(type_)?;
    plan::with(plan, |backend| {
        hipfft::status(unsafe {
            (hipfft::library()?.hipfftMakePlan1d)(backend, nx, type_, batch, workSize)
        })
    })
}

pub(crate) unsafe fn cufftMakePlan2d(
    plan: cufftHandle,
    nx: i32,
    ny: i32,
    type_: cufftType,
    workSize: *mut usize,
) -> cufftResult {
    let type_ = hipfft::transform_type(type_)?;
    plan::with(plan, |backend| {
        hipfft::status(unsafe {
            (hipfft::library()?.hipfftMakePlan2d)(backend, nx, ny, type_, workSize)
        })
    })
}

pub(crate) unsafe fn cufftMakePlan3d(
    plan: cufftHandle,
    nx: i32,
    ny: i32,
    nz: i32,
    type_: cufftType,
    workSize: *mut usize,
) -> cufftResult {
    let type_ = hipfft::transform_type(type_)?;
    plan::with(plan, |backend| {
        hipfft::status(unsafe {
            (hipfft::library()?.hipfftMakePlan3d)(backend, nx, ny, nz, type_, workSize)
        })
    })
}

pub(crate) unsafe fn cufftMakePlanMany(
    plan: cufftHandle,
    rank: i32,
    n: *mut i32,
    inembed: *mut i32,
    istride: i32,
    idist: i32,
    onembed: *mut i32,
    ostride: i32,
    odist: i32,
    type_: cufftType,
    batch: i32,
    workSize: *mut usize,
) -> cufftResult {
    let type_ = hipfft::transform_type(type_)?;
    plan::with(plan, |backend| {
        hipfft::status(unsafe {
            (hipfft::library()?.hipfftMakePlanMany)(
                backend, rank, n, inembed, istride, idist, onembed, ostride, odist, type_, batch,
                workSize,
            )
        })
    })
}

pub(crate) unsafe fn cufftMakePlanMany64(
    plan: cufftHandle,
    rank: i32,
    n: *mut i64,
    inembed: *mut i64,
    istride: i64,
    idist: i64,
    onembed: *mut i64,
    ostride: i64,
    odist: i64,
    type_: cufftType,
    batch: i64,
    workSize: *mut usize,
) -> cufftResult {
    let type_ = hipfft::transform_type(type_)?;
    plan::with(plan, |backend| {
        hipfft::status(unsafe {
            (hipfft::library()?.hipfftMakePlanMany64)(
                backend, rank, n, inembed, istride, idist, onembed, ostride, odist, type_, batch,
                workSize,
            )
        })
    })
}

pub(crate) unsafe fn cufftEstimate1d(
    nx: i32,
    type_: cufftType,
    batch: i32,
    workSize: *mut usize,
) -> cufftResult {
    let type_ = hipfft::transform_type(type_)?;
    hipfft::status(unsafe { (hipfft::library()?.hipfftEstimate1d)(nx, type_, batch, workSize) })
}

pub(crate) unsafe fn cufftEstimate2d(
    nx: i32,
    ny: i32,
    type_: cufftType,
    workSize: *mut usize,
) -> cufftResult {
    let type_ = hipfft::transform_type(type_)?;
    hipfft::status(unsafe { (hipfft::library()?.hipfftEstimate2d)(nx, ny, type_, workSize) })
}

pub(crate) unsafe fn cufftEstimate3d(
    nx: i32,
    ny: i32,
    nz: i32,
    type_: cufftType,
    workSize: *mut usize,
) -> cufftResult {
    let type_ = hipfft::transform_type(type_)?;
    hipfft::status(unsafe { (hipfft::library()?.hipfftEstimate3d)(nx, ny, nz, type_, workSize) })
}

pub(crate) unsafe fn cufftEstimateMany(
    rank: i32,
    n: *mut i32,
    inembed: *mut i32,
    istride: i32,
    idist: i32,
    onembed: *mut i32,
    ostride: i32,
    odist: i32,
    type_: cufftType,
    batch: i32,
    workSize: *mut usize,
) -> cufftResult {
    let type_ = hipfft::transform_type(type_)?;
    hipfft::status(unsafe {
        (hipfft::library()?.hipfftEstimateMany)(
            rank, n, inembed, istride, idist, onembed, ostride, odist, type_, batch, workSize,
        )
    })
}

pub(crate) unsafe fn cufftGetSize1d(
    plan: cufftHandle,
    nx: i32,
    type_: cufftType,
    batch: i32,
    workSize: *mut usize,
) -> cufftResult {
    let type_ = hipfft::transform_type(type_)?;
    plan::with(plan, |backend| {
        hipfft::status(unsafe {
            (hipfft::library()?.hipfftGetSize1d)(backend, nx, type_, batch, workSize)
        })
    })
}

pub(crate) unsafe fn cufftGetSize2d(
    plan: cufftHandle,
    nx: i32,
    ny: i32,
    type_: cufftType,
    workSize: *mut usize,
) -> cufftResult {
    let type_ = hipfft::transform_type(type_)?;
    plan::with(plan, |backend| {
        hipfft::status(unsafe {
            (hipfft::library()?.hipfftGetSize2d)(backend, nx, ny, type_, workSize)
        })
    })
}

pub(crate) unsafe fn cufftGetSize3d(
    plan: cufftHandle,
    nx: i32,
    ny: i32,
    nz: i32,
    type_: cufftType,
    workSize: *mut usize,
) -> cufftResult {
    let type_ = hipfft::transform_type(type_)?;
    plan::with(plan, |backend| {
        hipfft::status(unsafe {
            (hipfft::library()?.hipfftGetSize3d)(backend, nx, ny, nz, type_, workSize)
        })
    })
}

pub(crate) unsafe fn cufftGetSizeMany(
    plan: cufftHandle,
    rank: i32,
    n: *mut i32,
    inembed: *mut i32,
    istride: i32,
    idist: i32,
    onembed: *mut i32,
    ostride: i32,
    odist: i32,
    type_: cufftType,
    batch: i32,
    workArea: *mut usize,
) -> cufftResult {
    let type_ = hipfft::transform_type(type_)?;
    plan::with(plan, |backend| {
        hipfft::status(unsafe {
            (hipfft::library()?.hipfftGetSizeMany)(
                backend, rank, n, inembed, istride, idist, onembed, ostride, odist, type_, batch,
                workArea,
            )
        })
    })
}

pub(crate) unsafe fn cufftGetSizeMany64(
    plan: cufftHandle,
    rank: i32,
    n: *mut i64,
    inembed: *mut i64,
    istride: i64,
    idist: i64,
    onembed: *mut i64,
    ostride: i64,
    odist: i64,
    type_: cufftType,
    batch: i64,
    workSize: *mut usize,
) -> cufftResult {
    let type_ = hipfft::transform_type(type_)?;
    plan::with(plan, |backend| {
        hipfft::status(unsafe {
            (hipfft::library()?.hipfftGetSizeMany64)(
                backend, rank, n, inembed, istride, idist, onembed, ostride, odist, type_, batch,
                workSize,
            )
        })
    })
}

pub(crate) unsafe fn cufftGetSize(plan: cufftHandle, workSize: *mut usize) -> cufftResult {
    plan::with(plan, |backend| {
        hipfft::status(unsafe { (hipfft::library()?.hipfftGetSize)(backend, workSize) })
    })
}

pub(crate) unsafe fn cufftSetWorkArea(plan: cufftHandle, workArea: *mut c_void) -> cufftResult {
    plan::with(plan, |backend| {
        hipfft::status(unsafe { (hipfft::library()?.hipfftSetWorkArea)(backend, workArea) })
    })
}

pub(crate) unsafe fn cufftSetAutoAllocation(plan: cufftHandle, autoAllocate: i32) -> cufftResult {
    plan::with(plan, |backend| {
        hipfft::status(unsafe {
            (hipfft::library()?.hipfftSetAutoAllocation)(backend, autoAllocate)
        })
    })
}

pub(crate) unsafe fn cufftExecC2C(
    plan: cufftHandle,
    idata: *mut cufftComplex,
    odata: *mut cufftComplex,
    direction: i32,
) -> cufftResult {
    plan::with(plan, |backend| {
        hipfft::status(unsafe {
            (hipfft::library()?.hipfftExecC2C)(backend, idata, odata, direction)
        })
    })
}

pub(crate) unsafe fn cufftExecR2C(
    plan: cufftHandle,
    idata: *mut cufftReal,
    odata: *mut cufftComplex,
) -> cufftResult {
    plan::with(plan, |backend| {
        hipfft::status(unsafe { (hipfft::library()?.hipfftExecR2C)(backend, idata, odata) })
    })
}

pub(crate) unsafe fn cufftExecC2R(
    plan: cufftHandle,
    idata: *mut cufftComplex,
    odata: *mut cufftReal,
) -> cufftResult {
    plan::with(plan, |backend| {
        hipfft::status(unsafe { (hipfft::library()?.hipfftExecC2R)(backend, idata, odata) })
    })
}

pub(crate) unsafe fn cufftExecZ2Z(
    plan: cufftHandle,
    idata: *mut cufftDoubleComplex,
    odata: *mut cufftDoubleComplex,
    direction: i32,
) -> cufftResult {
    plan::with(plan, |backend| {
        hipfft::status(unsafe {
            (hipfft::library()?.hipfftExecZ2Z)(backend, idata, odata, direction)
        })
    })
}

pub(crate) unsafe fn cufftExecD2Z(
    plan: cufftHandle,
    idata: *mut cufftDoubleReal,
    odata: *mut cufftDoubleComplex,
) -> cufftResult {
    plan::with(plan, |backend| {
        hipfft::status(unsafe { (hipfft::library()?.hipfftExecD2Z)(backend, idata, odata) })
    })
}

pub(crate) unsafe fn cufftExecZ2D(
    plan: cufftHandle,
    idata: *mut cufftDoubleComplex,
    odata: *mut cufftDoubleReal,
) -> cufftResult {
    plan::with(plan, |backend| {
        hipfft::status(unsafe { (hipfft::library()?.hipfftExecZ2D)(backend, idata, odata) })
    })
}

pub(crate) unsafe fn cufftSetStream(plan: cufftHandle, stream: cudaStream_t) -> cufftResult {
    plan::with(plan, |backend| {
        hipfft::status(unsafe { (hipfft::library()?.hipfftSetStream)(backend, stream) })
    })
}

pub(crate) unsafe fn cufftDestroy(plan: cufftHandle) -> cufftResult {
    plan::remove(plan, |backend| {
        hipfft::status(unsafe { (hipfft::library()?.hipfftDestroy)(backend) })
    })
}

pub(crate) unsafe fn cufftGetVersion(version: *mut i32) -> cufftResult {
    hipfft::status(unsafe { (hipfft::library()?.hipfftGetVersion)(version) })
}

pub(crate) unsafe fn cufftGetProperty(type_: libraryPropertyType, value: *mut i32) -> cufftResult {
    let type_ = hipfft::property_type(type_)?;
    hipfft::status(unsafe { (hipfft::library()?.hipfftGetProperty)(type_, value) })
}
 */
