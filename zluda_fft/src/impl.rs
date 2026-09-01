use crate::plan;
use cuda_types::cufft::{self, *};
use hip_runtime_sys::hipStream_t;
use hipfft_sys::*;
use std::{
    mem,
    sync::{Mutex, OnceLock},
};

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> cufftResult {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> cufftResult {
    cufftResult::ERROR_NOT_SUPPORTED
}

fn hipfft() -> Result<&'static super::HipfftVtable, hipfftError> {
    static LOCK: OnceLock<Result<super::HipfftVtable, hipfftError>> = OnceLock::new();
    let unwrapped: &Result<super::HipfftVtable, hipfftError> = LOCK.get_or_init(|| {
        let hipfft = unsafe { super::HipfftVtable::new()? };
        Ok(hipfft)
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

pub(crate) unsafe fn plan1d(
    output: &mut cufftHandle,
    nx: i32,
    cu_type: hipfftType,
    batch: i32,
) -> Result<(), cufftError_t> {
    create_plan(output, |hipfft, result| {
        hipfft.hipfftPlan1d(result, nx, cu_type, batch)
    })
}

pub(crate) unsafe fn plan2d(
    output: &mut cufftHandle,
    nx: ::core::ffi::c_int,
    ny: ::core::ffi::c_int,
    cu_type: hipfftType,
) -> Result<(), cufftError_t> {
    create_plan(output, |hipfft, result| {
        hipfft.hipfftPlan2d(result, nx, ny, cu_type)
    })
}

pub(crate) unsafe fn plan3d(
    output: &mut cufftHandle,
    nx: ::core::ffi::c_int,
    ny: ::core::ffi::c_int,
    nz: ::core::ffi::c_int,
    cu_type: hipfftType,
) -> Result<(), cufftError_t> {
    create_plan(output, |hipfft, result| {
        hipfft.hipfftPlan3d(result, nx, ny, nz, cu_type)
    })
}

pub(crate) unsafe fn plan_many(
    output: &mut cufftHandle,
    rank: ::core::ffi::c_int,
    n: *mut ::core::ffi::c_int,
    inembed: *mut ::core::ffi::c_int,
    istride: ::core::ffi::c_int,
    idist: ::core::ffi::c_int,
    onembed: *mut ::core::ffi::c_int,
    ostride: ::core::ffi::c_int,
    odist: ::core::ffi::c_int,
    type_: hipfftType,
    batch: ::core::ffi::c_int,
) -> Result<(), cufftError_t> {
    create_plan(output, |hipfft, result| {
        hipfft.hipfftPlanMany(
            result, rank, n, inembed, istride, idist, onembed, ostride, odist, type_, batch,
        )
    })
}

pub(crate) unsafe fn make_plan1d(
    plan: cufftHandle,
    nx: i32,
    cu_type: hipfftType,
    batch: i32,
    work_size: &mut usize,
) -> Result<(), cufftError_t> {
    with_plan(plan, |hipfft, plan| {
        hipfft.hipfftMakePlan1d(plan, nx, cu_type, batch, work_size)
    })
}

pub(crate) unsafe fn make_plan2d(
    plan: cufftHandle,
    nx: i32,
    ny: i32,
    cu_type: hipfftType,
    work_size: &mut usize,
) -> Result<(), cufftError_t> {
    with_plan(plan, |hipfft, plan| {
        hipfft.hipfftMakePlan2d(plan, nx, ny, cu_type, work_size)
    })
}

pub(crate) unsafe fn make_plan3d(
    plan: cufftHandle,
    nx: i32,
    ny: i32,
    nz: i32,
    cu_type: hipfftType,
    work_size: &mut usize,
) -> Result<(), cufftError_t> {
    with_plan(plan, |hipfft, plan| {
        hipfft.hipfftMakePlan3d(plan, nx, ny, nz, cu_type, work_size)
    })
}

pub(crate) unsafe fn make_plan_many(
    plan: cufftHandle,
    rank: ::core::ffi::c_int,
    n: *mut ::core::ffi::c_int,
    inembed: *mut ::core::ffi::c_int,
    istride: ::core::ffi::c_int,
    idist: ::core::ffi::c_int,
    onembed: *mut ::core::ffi::c_int,
    ostride: ::core::ffi::c_int,
    odist: ::core::ffi::c_int,
    type_: hipfftType,
    batch: ::core::ffi::c_int,
    work_size: *mut usize,
) -> Result<(), cufftError_t> {
    with_plan(plan, |hipfft, plan| {
        hipfft.hipfftMakePlanMany(
            plan, rank, n, inembed, istride, idist, onembed, ostride, odist, type_, batch,
            work_size,
        )
    })
}

pub(crate) unsafe fn make_plan_many64(
    plan: cufftHandle,
    rank: ::core::ffi::c_int,
    n: *mut ::core::ffi::c_longlong,
    inembed: *mut ::core::ffi::c_longlong,
    istride: ::core::ffi::c_longlong,
    idist: ::core::ffi::c_longlong,
    onembed: *mut ::core::ffi::c_longlong,
    ostride: ::core::ffi::c_longlong,
    odist: ::core::ffi::c_longlong,
    type_: hipfftType,
    batch: ::core::ffi::c_longlong,
    work_size: *mut usize,
) -> Result<(), cufftError_t> {
    with_plan(plan, |hipfft, plan| {
        hipfft.hipfftMakePlanMany64(
            plan, rank, n, inembed, istride, idist, onembed, ostride, odist, type_, batch,
            work_size,
        )
    })
}

pub(crate) unsafe fn get_size_many64(
    plan: cufftHandle,
    rank: ::core::ffi::c_int,
    n: *mut ::core::ffi::c_longlong,
    inembed: *mut ::core::ffi::c_longlong,
    istride: ::core::ffi::c_longlong,
    idist: ::core::ffi::c_longlong,
    onembed: *mut ::core::ffi::c_longlong,
    ostride: ::core::ffi::c_longlong,
    odist: ::core::ffi::c_longlong,
    type_: hipfftType,
    batch: ::core::ffi::c_longlong,
    work_size: *mut usize,
) -> Result<(), cufftError_t> {
    with_plan(plan, |hipfft, plan| {
        hipfft.hipfftGetSizeMany64(
            plan, rank, n, inembed, istride, idist, onembed, ostride, odist, type_, batch,
            work_size,
        )
    })
}

pub(crate) unsafe fn estimate1d(
    nx: i32,
    type_: hipfftType,
    batch: i32,
    work_size: *mut usize,
) -> cufftResult {
    hipfft()?.hipfftEstimate1d(nx, type_, batch, work_size)?;
    Ok(())
}

pub(crate) unsafe fn estimate2d(
    nx: i32,
    ny: i32,
    type_: hipfftType,
    work_size: *mut usize,
) -> cufftResult {
    hipfft()?.hipfftEstimate2d(nx, ny, type_, work_size)?;
    Ok(())
}

pub(crate) unsafe fn estimate3d(
    nx: i32,
    ny: i32,
    nz: i32,
    type_: hipfftType,
    work_size: *mut usize,
) -> cufftResult {
    hipfft()?.hipfftEstimate3d(nx, ny, nz, type_, work_size)?;
    Ok(())
}

pub(crate) unsafe fn estimate_many(
    rank: i32,
    n: *mut i32,
    inembed: *mut i32,
    istride: i32,
    idist: i32,
    onembed: *mut i32,
    ostride: i32,
    odist: i32,
    type_: hipfftType,
    batch: i32,
    work_size: *mut usize,
) -> cufftResult {
    hipfft()?.hipfftEstimateMany(
        rank, n, inembed, istride, idist, onembed, ostride, odist, type_, batch, work_size,
    )?;
    Ok(())
}

pub(crate) unsafe fn create(handle: &mut cufftHandle) -> Result<(), cufftError_t> {
    create_plan(handle, |hipfft, result| hipfft.hipfftCreate(result))
}

unsafe fn create_plan(
    output: &mut cufftHandle,
    fn_: impl FnOnce(&super::HipfftVtable, &mut hipfftHandle) -> Result<(), hipfftError>,
) -> Result<(), cufftError_t> {
    let hipfft = hipfft()?;
    let mut plan = mem::zeroed();
    fn_(&hipfft, &mut plan)?;
    let plan = GlobalState::with(|state| Ok(state.registry.insert(plan)))?;
    *output = plan;
    Ok(())
}

unsafe fn with_plan(
    cu_plan: cufftHandle,
    fn_: impl FnOnce(&super::HipfftVtable, hipfftHandle) -> Result<(), hipfftError>,
) -> Result<(), cufftError_t> {
    let hipfft = hipfft()?;
    GlobalState::with(|state| {
        let plan = state.registry.get(cu_plan)?;
        fn_(&hipfft, *plan)?;
        Ok(())
    })?;
    Ok(())
}

pub(crate) unsafe fn get_size1d(
    plan: cufftHandle,
    nx: i32,
    type_: hipfftType,
    batch: i32,
    work_size: *mut usize,
) -> cufftResult {
    with_plan(plan, |hipfft, plan| {
        hipfft.hipfftGetSize1d(plan, nx, type_, batch, work_size)
    })
}

pub(crate) unsafe fn get_size2d(
    plan: cufftHandle,
    nx: i32,
    ny: i32,
    type_: hipfftType,
    work_size: *mut usize,
) -> cufftResult {
    with_plan(plan, |hipfft, plan| {
        hipfft.hipfftGetSize2d(plan, nx, ny, type_, work_size)
    })
}

pub(crate) unsafe fn get_size3d(
    plan: cufftHandle,
    nx: i32,
    ny: i32,
    nz: i32,
    type_: hipfftType,
    work_size: *mut usize,
) -> cufftResult {
    with_plan(plan, |hipfft, plan| {
        hipfft.hipfftGetSize3d(plan, nx, ny, nz, type_, work_size)
    })
}

pub(crate) unsafe fn get_size_many(
    plan: cufftHandle,
    rank: i32,
    n: *mut i32,
    inembed: *mut i32,
    istride: i32,
    idist: i32,
    onembed: *mut i32,
    ostride: i32,
    odist: i32,
    type_: hipfftType,
    batch: i32,
    work_size: *mut usize,
) -> cufftResult {
    with_plan(plan, |hipfft, plan| {
        hipfft.hipfftGetSizeMany(
            plan, rank, n, inembed, istride, idist, onembed, ostride, odist, type_, batch,
            work_size,
        )
    })
}

pub(crate) unsafe fn get_size(plan: cufftHandle, work_size: *mut usize) -> cufftResult {
    with_plan(plan, |hipfft, plan| hipfft.hipfftGetSize(plan, work_size))
}

pub(crate) unsafe fn set_work_area(
    plan: cufftHandle,
    work_area: *mut std::ffi::c_void,
) -> cufftResult {
    with_plan(plan, |hipfft, plan| {
        hipfft.hipfftSetWorkArea(plan, work_area)
    })
}

pub(crate) unsafe fn set_auto_allocation(plan: cufftHandle, auto_allocate: i32) -> cufftResult {
    with_plan(plan, |hipfft, plan| {
        hipfft.hipfftSetAutoAllocation(plan, auto_allocate)
    })
}

pub(crate) unsafe fn set_stream(plan: cufftHandle, stream: hipStream_t) -> cufftResult {
    with_plan(plan, |hipfft, plan| hipfft.hipfftSetStream(plan, stream))
}

pub(crate) unsafe fn destroy(plan: cufftHandle) -> cufftResult {
    with_plan(plan, |hipfft, plan| hipfft.hipfftDestroy(plan))
}

pub(crate) unsafe fn get_version(version: &mut i32) -> cufftResult {
    *version = cufft::CUFFT_VERSION as i32;
    Ok(())
}

pub(crate) unsafe fn get_property(type_: libraryPropertyType, value: &mut i32) -> cufftResult {
    let result = match type_ {
        libraryPropertyType::MAJOR_VERSION => cufft::CUFFT_VER_MAJOR,
        libraryPropertyType::MINOR_VERSION => cufft::CUFFT_VER_MINOR,
        libraryPropertyType::PATCH_LEVEL => cufft::CUFFT_VER_PATCH,
        _ => return Err(cufftError_t::INVALID_VALUE),
    };
    *value = result as i32;
    Ok(())
}

pub(crate) unsafe fn exec_c2_c(
    plan: cufftHandle,
    idata: *mut cufftComplex,
    odata: *mut cufftComplex,
    direction: i32,
) -> cufftResult {
    with_plan(plan, |hipfft, plan| {
        hipfft.hipfftExecC2C(plan, idata.cast(), odata.cast(), direction)
    })
}

pub(crate) unsafe fn exec_r2_c(
    plan: cufftHandle,
    idata: *mut cufftReal,
    odata: *mut cufftComplex,
) -> cufftResult {
    with_plan(plan, |hipfft, plan| {
        hipfft.hipfftExecR2C(plan, idata, odata.cast())
    })
}

pub(crate) unsafe fn exec_c2_r(
    plan: cufftHandle,
    idata: *mut cufftComplex,
    odata: *mut cufftReal,
) -> cufftResult {
    with_plan(plan, |hipfft, plan| {
        hipfft.hipfftExecC2R(plan, idata.cast(), odata)
    })
}

pub(crate) unsafe fn exec_z2_z(
    plan: cufftHandle,
    idata: *mut cufftDoubleComplex,
    odata: *mut cufftDoubleComplex,
    direction: i32,
) -> cufftResult {
    with_plan(plan, |hipfft, plan| {
        hipfft.hipfftExecZ2Z(plan, idata.cast(), odata.cast(), direction)
    })
}

pub(crate) unsafe fn exec_d2_z(
    plan: cufftHandle,
    idata: *mut cufftDoubleReal,
    odata: *mut cufftDoubleComplex,
) -> cufftResult {
    with_plan(plan, |hipfft, plan| {
        hipfft.hipfftExecD2Z(plan, idata, odata.cast())
    })
}

pub(crate) unsafe fn exec_z_2d(
    plan: cufftHandle,
    idata: *mut cufftDoubleComplex,
    odata: *mut cufftDoubleReal,
) -> cufftResult {
    with_plan(plan, |hipfft, plan| {
        hipfft.hipfftExecZ2D(plan, idata.cast(), odata)
    })
}
