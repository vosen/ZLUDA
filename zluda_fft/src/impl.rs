#![allow(non_snake_case)]

use crate::{hipfft, plan};
use cuda_types::cufft::*;
use std::{ffi::c_void, ptr};

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> cufftResult {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> cufftResult {
    cufftResult::ERROR_NOT_SUPPORTED
}

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
    let type_ = hipfft::transform_type(type_)?;
    unsafe {
        create_plan(plan, |lib, backend| {
            (lib.hipfftPlan1d)(backend, nx, type_, batch)
        })
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
