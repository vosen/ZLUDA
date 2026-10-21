//! cuSOLVER's dense LU, Cholesky and QR routines on Apple GPUs for zluda_agx (Asahi Linux), on the kernels of
//! agx_libs (`agx_libs/kernels/solver.mlir`, written once for every AGX generation; double precision on its f64
//! copies).
//!
//! `cusolverDn{S,D}getrf` / `getrs` (partial pivoting; `devIpiv` is required), `potrf` / `potrs` (either
//! triangle), `geqrf`, `ormqr` (either side, `N` or `T`) and `orgqr`, with their `_bufferSize` queries, on handles
//! with streams. The factorizations run one column at a time (a pivot or reflector step, then a parallel update of
//! the rest of the matrix); the solves run one thread per right-hand side. `devInfo` reports a zero pivot (getrf) or
//! a non-positive leading minor (potrf) like LAPACK. Other functions are not exported.

#![allow(non_camel_case_types, non_snake_case)]

use agx_libs::engine;
use cuda_types::cublas::{cublasFillMode_t, cublasOperation_t, cublasSideMode_t};
use cuda_types::cuda::{libraryPropertyType_t, CUstream};
use std::ffi::{c_int, c_void, CStr};

/// `cusolverStatus_t`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct cusolverStatus_t(pub u32);

impl cusolverStatus_t {
    pub const SUCCESS: cusolverStatus_t = cusolverStatus_t(0);
    pub const NOT_INITIALIZED: cusolverStatus_t = cusolverStatus_t(1);
    pub const ALLOC_FAILED: cusolverStatus_t = cusolverStatus_t(2);
    pub const INVALID_VALUE: cusolverStatus_t = cusolverStatus_t(3);
    pub const ARCH_MISMATCH: cusolverStatus_t = cusolverStatus_t(4);
    pub const EXECUTION_FAILED: cusolverStatus_t = cusolverStatus_t(6);
    pub const INTERNAL_ERROR: cusolverStatus_t = cusolverStatus_t(7);
    pub const NOT_SUPPORTED: cusolverStatus_t = cusolverStatus_t(9);
}

/// The cuSOLVER version reported (11.6.3, of CUDA 12.5).
pub const VERSION: (c_int, c_int, c_int) = (11, 6, 3);

pub struct Handle {
    stream: CUstream,
}

pub type cusolverDnHandle_t = *mut Handle;

type R = Result<(), cusolverStatus_t>;

fn status(e: engine::Error) -> cusolverStatus_t {
    match e {
        engine::Error::NotInitialized => cusolverStatus_t::NOT_INITIALIZED,
        engine::Error::ArchMismatch => cusolverStatus_t::ARCH_MISMATCH,
        engine::Error::Failed => cusolverStatus_t::EXECUTION_FAILED,
    }
}

fn done(r: R) -> cusolverStatus_t {
    r.err().unwrap_or(cusolverStatus_t::SUCCESS)
}

unsafe fn handle<'a>(h: cusolverDnHandle_t) -> Result<&'a Handle, cusolverStatus_t> {
    h.as_ref().ok_or(cusolverStatus_t::NOT_INITIALIZED)
}

fn arg<T>(v: &mut T) -> *mut c_void {
    v as *mut T as *mut c_void
}

/// The f32 kernel or its f64 copy.
fn kernel(double: bool, names: [&'static CStr; 2]) -> &'static CStr {
    names[double as usize]
}

/// Launches a kernel over `grid` threads (rounded up to whole groups of `block`).
unsafe fn run(h: &Handle, name: &'static CStr, params: &mut [*mut c_void], grid: [i64; 2], block: [u32; 2]) -> R {
    if grid[0] <= 0 || grid[1] <= 0 {
        return Ok(());
    }
    let groups = |n: i64, b: u32| u32::try_from((n as u64).div_ceil(b as u64)).map_err(|_| cusolverStatus_t::NOT_SUPPORTED);
    let f = engine::function(agx_libs::SOLVER, name).map_err(status)?;
    engine::launch_kernel(f, [groups(grid[0], block[0])?, groups(grid[1], block[1])?], block, params, h.stream).map_err(status)
}

unsafe fn set_info(h: &Handle, info: *mut c_int, v: i32) -> R {
    if info.is_null() {
        return Err(cusolverStatus_t::INVALID_VALUE);
    }
    let (mut p, mut v) = (info as *mut c_void, v);
    run(h, c"agx_sv_set_info", &mut [arg(&mut p), arg(&mut v)], [1, 1], [1, 1])
}

/// The checks of an m x n matrix with leading dimension lda.
fn dims(m: c_int, n: c_int, lda: c_int) -> R {
    if m < 0 || n < 0 || lda < m.max(1) {
        return Err(cusolverStatus_t::INVALID_VALUE);
    }
    Ok(())
}

fn trans(op: cublasOperation_t) -> Result<bool, cusolverStatus_t> {
    match op {
        cublasOperation_t::CUBLAS_OP_N => Ok(false),
        cublasOperation_t::CUBLAS_OP_T | cublasOperation_t::CUBLAS_OP_C => Ok(true),
        _ => Err(cusolverStatus_t::INVALID_VALUE),
    }
}

fn upper(f: cublasFillMode_t) -> Result<bool, cusolverStatus_t> {
    match f {
        cublasFillMode_t::CUBLAS_FILL_MODE_LOWER => Ok(false),
        cublasFillMode_t::CUBLAS_FILL_MODE_UPPER => Ok(true),
        _ => Err(cusolverStatus_t::INVALID_VALUE),
    }
}

unsafe fn out(p: *mut c_int, v: usize) -> R {
    *p.as_mut().ok_or(cusolverStatus_t::INVALID_VALUE)? = c_int::try_from(v.max(1)).map_err(|_| cusolverStatus_t::INVALID_VALUE)?;
    Ok(())
}

/// getrf: P A = L U, one column at a time (pivot, swap, scale, update).
unsafe fn getrf(h: &Handle, double: bool, m: c_int, n: c_int, a: *mut c_void, lda: c_int, ipiv: *mut c_int, info: *mut c_int) -> R {
    dims(m, n, lda)?;
    if ipiv.is_null() {
        return Err(cusolverStatus_t::NOT_SUPPORTED);
    }
    if m.min(n) == 0 {
        return set_info(h, info, 0);
    }
    if info.is_null() {
        return Err(cusolverStatus_t::INVALID_VALUE);
    }
    let (mut pa, mut pi, mut pinfo) = (a, ipiv as *mut c_void, info as *mut c_void);
    let (mut m_, mut n_, mut lda_) = (m as i64, n as i64, lda as i64);
    for k in 0..m.min(n) as i64 {
        let mut k_ = k;
        run(
            h,
            kernel(double, [c"agx_lu_pivot", c"agx_lu_pivot_f64"]),
            &mut [arg(&mut pa), arg(&mut m_), arg(&mut k_), arg(&mut lda_), arg(&mut pi), arg(&mut pinfo)],
            [1, 1],
            [1, 1],
        )?;
        run(
            h,
            kernel(double, [c"agx_lu_swap", c"agx_lu_swap_f64"]),
            &mut [arg(&mut pa), arg(&mut n_), arg(&mut lda_), arg(&mut k_), arg(&mut pi)],
            [n_, 1],
            [64, 1],
        )?;
        run(
            h,
            kernel(double, [c"agx_lu_scale", c"agx_lu_scale_f64"]),
            &mut [arg(&mut pa), arg(&mut m_), arg(&mut lda_), arg(&mut k_)],
            [m_ - k - 1, 1],
            [64, 1],
        )?;
        run(
            h,
            kernel(double, [c"agx_lu_update", c"agx_lu_update_f64"]),
            &mut [arg(&mut pa), arg(&mut m_), arg(&mut n_), arg(&mut lda_), arg(&mut k_)],
            [m_ - k - 1, n_ - k - 1],
            [16, 4],
        )?;
    }
    Ok(())
}

unsafe fn getrs(
    h: &Handle,
    double: bool,
    op: cublasOperation_t,
    n: c_int,
    nrhs: c_int,
    a: *const c_void,
    lda: c_int,
    ipiv: *const c_int,
    b: *mut c_void,
    ldb: c_int,
    info: *mut c_int,
) -> R {
    let t = trans(op)?;
    dims(n, n, lda)?;
    dims(n, nrhs, ldb)?;
    if ipiv.is_null() && n > 0 {
        return Err(cusolverStatus_t::INVALID_VALUE);
    }
    set_info(h, info, 0)?;
    let (mut pa, mut pi, mut pb) = (a as *mut c_void, ipiv as *mut c_void, b);
    let (mut n_, mut lda_, mut nrhs_, mut ldb_, mut t_) = (n as i64, lda as i64, nrhs as i64, ldb as i64, t as i64);
    if n == 0 {
        return Ok(());
    }
    run(
        h,
        kernel(double, [c"agx_getrs", c"agx_getrs_f64"]),
        &mut [arg(&mut pa), arg(&mut n_), arg(&mut lda_), arg(&mut pi), arg(&mut pb), arg(&mut nrhs_), arg(&mut ldb_), arg(&mut t_)],
        [nrhs_, 1],
        [32, 1],
    )
}

/// potrf: A = L L^T or U^T U, one column at a time (diagonal, scale, update).
unsafe fn potrf(h: &Handle, double: bool, uplo: cublasFillMode_t, n: c_int, a: *mut c_void, lda: c_int, info: *mut c_int) -> R {
    let up = upper(uplo)?;
    dims(n, n, lda)?;
    if n == 0 {
        return set_info(h, info, 0);
    }
    if info.is_null() {
        return Err(cusolverStatus_t::INVALID_VALUE);
    }
    let (mut pa, mut pinfo) = (a, info as *mut c_void);
    let (mut n_, mut rs, mut cs) = (n as i64, if up { lda as i64 } else { 1 }, if up { 1 } else { lda as i64 });
    for k in 0..n as i64 {
        let mut k_ = k;
        run(
            h,
            kernel(double, [c"agx_chol_diag", c"agx_chol_diag_f64"]),
            &mut [arg(&mut pa), arg(&mut k_), arg(&mut rs), arg(&mut cs), arg(&mut pinfo)],
            [1, 1],
            [1, 1],
        )?;
        run(
            h,
            kernel(double, [c"agx_chol_scale", c"agx_chol_scale_f64"]),
            &mut [arg(&mut pa), arg(&mut n_), arg(&mut k_), arg(&mut rs), arg(&mut cs), arg(&mut pinfo)],
            [n_ - k - 1, 1],
            [64, 1],
        )?;
        run(
            h,
            kernel(double, [c"agx_chol_update", c"agx_chol_update_f64"]),
            &mut [arg(&mut pa), arg(&mut n_), arg(&mut k_), arg(&mut rs), arg(&mut cs), arg(&mut pinfo)],
            [n_ - k - 1, n_ - k - 1],
            [16, 4],
        )?;
    }
    Ok(())
}

unsafe fn potrs(
    h: &Handle,
    double: bool,
    uplo: cublasFillMode_t,
    n: c_int,
    nrhs: c_int,
    a: *const c_void,
    lda: c_int,
    b: *mut c_void,
    ldb: c_int,
    info: *mut c_int,
) -> R {
    let up = upper(uplo)?;
    dims(n, n, lda)?;
    dims(n, nrhs, ldb)?;
    set_info(h, info, 0)?;
    if n == 0 {
        return Ok(());
    }
    let (mut pa, mut pb) = (a as *mut c_void, b);
    let (mut n_, mut lda_, mut nrhs_, mut ldb_, mut up_) = (n as i64, lda as i64, nrhs as i64, ldb as i64, up as i64);
    run(
        h,
        kernel(double, [c"agx_potrs", c"agx_potrs_f64"]),
        &mut [arg(&mut pa), arg(&mut n_), arg(&mut lda_), arg(&mut pb), arg(&mut nrhs_), arg(&mut ldb_), arg(&mut up_)],
        [nrhs_, 1],
        [32, 1],
    )
}

/// geqrf: A = Q R with Householder reflectors, one column at a time.
unsafe fn geqrf(h: &Handle, double: bool, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void, info: *mut c_int) -> R {
    dims(m, n, lda)?;
    if tau.is_null() && m.min(n) > 0 {
        return Err(cusolverStatus_t::INVALID_VALUE);
    }
    set_info(h, info, 0)?;
    let (mut pa, mut pt) = (a, tau);
    let (mut m_, mut n_, mut lda_) = (m as i64, n as i64, lda as i64);
    for k in 0..m.min(n) as i64 {
        let mut k_ = k;
        run(
            h,
            kernel(double, [c"agx_qr_house", c"agx_qr_house_f64"]),
            &mut [arg(&mut pa), arg(&mut m_), arg(&mut k_), arg(&mut lda_), arg(&mut pt)],
            [1, 1],
            [1, 1],
        )?;
        run(
            h,
            kernel(double, [c"agx_qr_apply", c"agx_qr_apply_f64"]),
            &mut [arg(&mut pa), arg(&mut m_), arg(&mut n_), arg(&mut k_), arg(&mut lda_), arg(&mut pt)],
            [n_ - k - 1, 1],
            [64, 1],
        )?;
    }
    Ok(())
}

/// op(Q) C or C op(Q) with Q from k reflectors in r (leading dimension ldr).
#[allow(clippy::too_many_arguments)]
unsafe fn apply_q(
    h: &Handle,
    double: bool,
    left: bool,
    t: bool,
    m: c_int,
    n: c_int,
    k: c_int,
    r: *const c_void,
    ldr: c_int,
    tau: *const c_void,
    c: *mut c_void,
    ldc: c_int,
) -> R {
    let (mut pr, mut pt, mut pc) = (r as *mut c_void, tau as *mut c_void, c);
    let (mut ldr_, mut m_, mut n_, mut k_, mut ldc_, mut left_, mut t_) = (ldr as i64, m as i64, n as i64, k as i64, ldc as i64, left as i64, t as i64);
    run(
        h,
        kernel(double, [c"agx_ormqr", c"agx_ormqr_f64"]),
        &mut [
            arg(&mut pr),
            arg(&mut ldr_),
            arg(&mut pt),
            arg(&mut pc),
            arg(&mut m_),
            arg(&mut n_),
            arg(&mut k_),
            arg(&mut ldc_),
            arg(&mut left_),
            arg(&mut t_),
        ],
        [if left { n_ } else { m_ }, 1],
        [32, 1],
    )
}

fn side(s: cublasSideMode_t) -> Result<bool, cusolverStatus_t> {
    match s {
        cublasSideMode_t::CUBLAS_SIDE_LEFT => Ok(true),
        cublasSideMode_t::CUBLAS_SIDE_RIGHT => Ok(false),
        _ => Err(cusolverStatus_t::INVALID_VALUE),
    }
}

/// The checks of ormqr: Q is m x m (left) or n x n (right) from k <= its order reflectors.
fn ormqr_check(left: bool, m: c_int, n: c_int, k: c_int, lda: c_int, ldc: c_int) -> R {
    let q = if left { m } else { n };
    dims(m, n, ldc)?;
    if k < 0 || k > q || lda < q.max(1) {
        return Err(cusolverStatus_t::INVALID_VALUE);
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
unsafe fn ormqr(
    h: &Handle,
    double: bool,
    s: cublasSideMode_t,
    op: cublasOperation_t,
    m: c_int,
    n: c_int,
    k: c_int,
    a: *const c_void,
    lda: c_int,
    tau: *const c_void,
    c: *mut c_void,
    ldc: c_int,
    info: *mut c_int,
) -> R {
    let (left, t) = (side(s)?, trans(op)?);
    ormqr_check(left, m, n, k, lda, ldc)?;
    set_info(h, info, 0)?;
    apply_q(h, double, left, t, m, n, k, a, lda, tau, c, ldc)
}

/// orgqr: the first n columns of Q = H(0) ... H(k - 1), with the reflectors copied to the workspace (m k values)
/// and A set to the identity first.
#[allow(clippy::too_many_arguments)]
unsafe fn orgqr(
    h: &Handle,
    double: bool,
    m: c_int,
    n: c_int,
    k: c_int,
    a: *mut c_void,
    lda: c_int,
    tau: *const c_void,
    work: *mut c_void,
    lwork: c_int,
    info: *mut c_int,
) -> R {
    dims(m, n, lda)?;
    if n > m || k < 0 || k > n || lwork < (m * k).max(1) || (work.is_null() && m * k > 0) {
        return Err(cusolverStatus_t::INVALID_VALUE);
    }
    set_info(h, info, 0)?;
    let copy = kernel(double, [c"agx_sv_copy", c"agx_sv_copy_f64"]);
    let (mut src, mut dst) = (a, work);
    let (mut lda_, mut ldw, mut m_, mut n_, mut k_, mut no, mut yes) = (lda as i64, m as i64, m as i64, n as i64, k as i64, 0i64, 1i64);
    run(h, copy, &mut [arg(&mut src), arg(&mut lda_), arg(&mut dst), arg(&mut ldw), arg(&mut m_), arg(&mut k_), arg(&mut no)], [m_, k_], [16, 4])?;
    run(h, copy, &mut [arg(&mut src), arg(&mut lda_), arg(&mut src), arg(&mut lda_), arg(&mut m_), arg(&mut n_), arg(&mut yes)], [m_, n_], [16, 4])?;
    apply_q(h, double, true, false, m, n, k, work, m, tau, a, lda)
}

macro_rules! precision {
    ($double:literal, $ty:ty, $getrf_bs:ident, $getrf:ident, $getrs:ident, $potrf_bs:ident, $potrf:ident, $potrs:ident,
     $geqrf_bs:ident, $geqrf:ident, $ormqr_bs:ident, $ormqr:ident, $orgqr_bs:ident, $orgqr:ident) => {
        #[cfg_attr(not(test), no_mangle)]
        pub unsafe extern "system" fn $getrf_bs(h: cusolverDnHandle_t, m: c_int, n: c_int, _a: *mut $ty, lda: c_int, lwork: *mut c_int) -> cusolverStatus_t {
            done(handle(h).and_then(|_| dims(m, n, lda)).and_then(|_| out(lwork, 1)))
        }

        #[cfg_attr(not(test), no_mangle)]
        pub unsafe extern "system" fn $getrf(
            h: cusolverDnHandle_t,
            m: c_int,
            n: c_int,
            a: *mut $ty,
            lda: c_int,
            _work: *mut $ty,
            ipiv: *mut c_int,
            info: *mut c_int,
        ) -> cusolverStatus_t {
            done(handle(h).and_then(|h| getrf(h, $double, m, n, a.cast(), lda, ipiv, info)))
        }

        #[cfg_attr(not(test), no_mangle)]
        pub unsafe extern "system" fn $getrs(
            h: cusolverDnHandle_t,
            op: cublasOperation_t,
            n: c_int,
            nrhs: c_int,
            a: *const $ty,
            lda: c_int,
            ipiv: *const c_int,
            b: *mut $ty,
            ldb: c_int,
            info: *mut c_int,
        ) -> cusolverStatus_t {
            done(handle(h).and_then(|h| getrs(h, $double, op, n, nrhs, a.cast(), lda, ipiv, b.cast(), ldb, info)))
        }

        #[cfg_attr(not(test), no_mangle)]
        pub unsafe extern "system" fn $potrf_bs(
            h: cusolverDnHandle_t,
            uplo: cublasFillMode_t,
            n: c_int,
            _a: *mut $ty,
            lda: c_int,
            lwork: *mut c_int,
        ) -> cusolverStatus_t {
            done(handle(h).and_then(|_| upper(uplo)).and_then(|_| dims(n, n, lda)).and_then(|_| out(lwork, 1)))
        }

        #[cfg_attr(not(test), no_mangle)]
        pub unsafe extern "system" fn $potrf(
            h: cusolverDnHandle_t,
            uplo: cublasFillMode_t,
            n: c_int,
            a: *mut $ty,
            lda: c_int,
            _work: *mut $ty,
            _lwork: c_int,
            info: *mut c_int,
        ) -> cusolverStatus_t {
            done(handle(h).and_then(|h| potrf(h, $double, uplo, n, a.cast(), lda, info)))
        }

        #[cfg_attr(not(test), no_mangle)]
        pub unsafe extern "system" fn $potrs(
            h: cusolverDnHandle_t,
            uplo: cublasFillMode_t,
            n: c_int,
            nrhs: c_int,
            a: *const $ty,
            lda: c_int,
            b: *mut $ty,
            ldb: c_int,
            info: *mut c_int,
        ) -> cusolverStatus_t {
            done(handle(h).and_then(|h| potrs(h, $double, uplo, n, nrhs, a.cast(), lda, b.cast(), ldb, info)))
        }

        #[cfg_attr(not(test), no_mangle)]
        pub unsafe extern "system" fn $geqrf_bs(h: cusolverDnHandle_t, m: c_int, n: c_int, _a: *mut $ty, lda: c_int, lwork: *mut c_int) -> cusolverStatus_t {
            done(handle(h).and_then(|_| dims(m, n, lda)).and_then(|_| out(lwork, 1)))
        }

        #[cfg_attr(not(test), no_mangle)]
        pub unsafe extern "system" fn $geqrf(
            h: cusolverDnHandle_t,
            m: c_int,
            n: c_int,
            a: *mut $ty,
            lda: c_int,
            tau: *mut $ty,
            _work: *mut $ty,
            _lwork: c_int,
            info: *mut c_int,
        ) -> cusolverStatus_t {
            done(handle(h).and_then(|h| geqrf(h, $double, m, n, a.cast(), lda, tau.cast(), info)))
        }

        #[cfg_attr(not(test), no_mangle)]
        pub unsafe extern "system" fn $ormqr_bs(
            h: cusolverDnHandle_t,
            s: cublasSideMode_t,
            op: cublasOperation_t,
            m: c_int,
            n: c_int,
            k: c_int,
            _a: *const $ty,
            lda: c_int,
            _tau: *const $ty,
            _c: *const $ty,
            ldc: c_int,
            lwork: *mut c_int,
        ) -> cusolverStatus_t {
            done(handle(h).and_then(|_| ormqr_check(side(s)?, m, n, k, lda, ldc)).and_then(|_| trans(op)).and_then(|_| out(lwork, 1)))
        }

        #[cfg_attr(not(test), no_mangle)]
        pub unsafe extern "system" fn $ormqr(
            h: cusolverDnHandle_t,
            s: cublasSideMode_t,
            op: cublasOperation_t,
            m: c_int,
            n: c_int,
            k: c_int,
            a: *const $ty,
            lda: c_int,
            tau: *const $ty,
            c: *mut $ty,
            ldc: c_int,
            _work: *mut $ty,
            _lwork: c_int,
            info: *mut c_int,
        ) -> cusolverStatus_t {
            done(handle(h).and_then(|h| ormqr(h, $double, s, op, m, n, k, a.cast(), lda, tau.cast(), c.cast(), ldc, info)))
        }

        #[cfg_attr(not(test), no_mangle)]
        pub unsafe extern "system" fn $orgqr_bs(
            h: cusolverDnHandle_t,
            m: c_int,
            n: c_int,
            k: c_int,
            _a: *const $ty,
            lda: c_int,
            _tau: *const $ty,
            lwork: *mut c_int,
        ) -> cusolverStatus_t {
            done(handle(h).and_then(|_| dims(m, n, lda)).and_then(|_| out(lwork, (m as usize) * (k.max(0) as usize))))
        }

        #[cfg_attr(not(test), no_mangle)]
        pub unsafe extern "system" fn $orgqr(
            h: cusolverDnHandle_t,
            m: c_int,
            n: c_int,
            k: c_int,
            a: *mut $ty,
            lda: c_int,
            tau: *const $ty,
            work: *mut $ty,
            lwork: c_int,
            info: *mut c_int,
        ) -> cusolverStatus_t {
            done(handle(h).and_then(|h| orgqr(h, $double, m, n, k, a.cast(), lda, tau.cast(), work.cast(), lwork, info)))
        }
    };
}

precision!(
    false,
    f32,
    cusolverDnSgetrf_bufferSize,
    cusolverDnSgetrf,
    cusolverDnSgetrs,
    cusolverDnSpotrf_bufferSize,
    cusolverDnSpotrf,
    cusolverDnSpotrs,
    cusolverDnSgeqrf_bufferSize,
    cusolverDnSgeqrf,
    cusolverDnSormqr_bufferSize,
    cusolverDnSormqr,
    cusolverDnSorgqr_bufferSize,
    cusolverDnSorgqr
);
precision!(
    true,
    f64,
    cusolverDnDgetrf_bufferSize,
    cusolverDnDgetrf,
    cusolverDnDgetrs,
    cusolverDnDpotrf_bufferSize,
    cusolverDnDpotrf,
    cusolverDnDpotrs,
    cusolverDnDgeqrf_bufferSize,
    cusolverDnDgeqrf,
    cusolverDnDormqr_bufferSize,
    cusolverDnDormqr,
    cusolverDnDorgqr_bufferSize,
    cusolverDnDorgqr
);

/// A scalar kernel argument of the value type.
struct Val(f32, f64);

impl Val {
    fn arg(&mut self, double: bool) -> *mut c_void {
        if double {
            arg(&mut self.1)
        } else {
            arg(&mut self.0)
        }
    }
}

/// The relative orthogonality below which Jacobi stops rotating a pair.
fn jacobi_tol(double: bool) -> f64 {
    if double {
        1e-14
    } else {
        1e-6
    }
}

/// One-sided Jacobi on the columns of A (m x n) with V (n x n) collecting the rotations, until a sweep rotates no
/// pair; `count` is a device word. Returns whether it converged within the sweep limit.
#[allow(clippy::too_many_arguments)]
unsafe fn jacobi(
    h: &Handle,
    double: bool,
    a: *mut c_void,
    m: i64,
    n: i64,
    lda: i64,
    v: *mut c_void,
    ldv: i64,
    count: *mut c_void,
) -> Result<bool, cusolverStatus_t> {
    let round = kernel(double, [c"agx_jacobi_round", c"agx_jacobi_round_f64"]);
    let tol = jacobi_tol(double);
    for _ in 0..60 {
        set_info(h, count.cast(), 0)?;
        for r in 0..n + n % 2 - 1 {
            let (mut pa, mut pv, mut pc) = (a, v, count);
            let (mut m_, mut n_, mut lda_, mut vr, mut ldv_, mut r_) = (m, n, lda, n, ldv, r);
            let mut tol = Val(tol as f32, tol);
            run(
                h,
                round,
                &mut [
                    arg(&mut pa),
                    arg(&mut m_),
                    arg(&mut n_),
                    arg(&mut lda_),
                    arg(&mut pv),
                    arg(&mut vr),
                    arg(&mut ldv_),
                    arg(&mut r_),
                    arg(&mut pc),
                    tol.arg(double),
                ],
                [(n + 1) / 2, 1],
                [32, 1],
            )?;
        }
        engine::synchronize(h.stream).map_err(status)?;
        let mut rotations = 0i32;
        engine::read(arg(&mut rotations), count, 4).map_err(status)?;
        if rotations == 0 {
            return Ok(true);
        }
    }
    Ok(false)
}

/// dst = src (m x n) or the identity.
#[allow(clippy::too_many_arguments)]
unsafe fn copy(h: &Handle, double: bool, src: *const c_void, lds: i64, dst: *mut c_void, ldd: i64, m: i64, n: i64, eye: bool) -> R {
    let (mut ps, mut pd, mut lds, mut ldd, mut m_, mut n_, mut eye) = (src as *mut c_void, dst, lds, ldd, m, n, eye as i64);
    run(
        h,
        kernel(double, [c"agx_sv_copy", c"agx_sv_copy_f64"]),
        &mut [arg(&mut ps), arg(&mut lds), arg(&mut pd), arg(&mut ldd), arg(&mut m_), arg(&mut n_), arg(&mut eye)],
        [m, n],
        [16, 4],
    )
}

/// Sorts s with the columns of U (ur rows) and V (vr rows).
#[allow(clippy::too_many_arguments)]
unsafe fn sort(h: &Handle, double: bool, s: *mut c_void, n: i64, u: *mut c_void, ur: i64, ldu: i64, v: *mut c_void, vr: i64, ldv: i64, ascending: bool) -> R {
    let (mut ps, mut pu, mut pv, mut n_, mut ur, mut ldu, mut vr, mut ldv, mut asc) = (s, u, v, n, ur, ldu.max(1), vr, ldv.max(1), ascending as i64);
    run(
        h,
        kernel(double, [c"agx_jacobi_sort", c"agx_jacobi_sort_f64"]),
        &mut [arg(&mut ps), arg(&mut n_), arg(&mut pu), arg(&mut ur), arg(&mut ldu), arg(&mut pv), arg(&mut vr), arg(&mut ldv), arg(&mut asc)],
        [1, 1],
        [1, 1],
    )
}

fn value_bytes(double: bool) -> usize {
    if double {
        8
    } else {
        4
    }
}

/// gesvd: A = U S V^T (m >= n) by one-sided Jacobi on A's columns. The workspace holds V and a counter.
#[allow(clippy::too_many_arguments)]
unsafe fn gesvd(
    h: &Handle,
    double: bool,
    jobu: i8,
    jobvt: i8,
    m: c_int,
    n: c_int,
    a: *mut c_void,
    lda: c_int,
    s: *mut c_void,
    u: *mut c_void,
    ldu: c_int,
    vt: *mut c_void,
    ldvt: c_int,
    work: *mut c_void,
    lwork: c_int,
    info: *mut c_int,
) -> R {
    let (ju, jv) = (jobu as u8, jobvt as u8);
    dims(m, n, lda)?;
    if m < n || !b"ASON".contains(&ju) || !b"ASON".contains(&jv) || (ju == b'O' && jv == b'O') {
        return Err(cusolverStatus_t::INVALID_VALUE);
    }
    if ju == b'A' && m != n {
        return Err(cusolverStatus_t::NOT_SUPPORTED);
    }
    if (b"AS".contains(&ju) && (u.is_null() || ldu < m.max(1))) || (b"AS".contains(&jv) && (vt.is_null() || ldvt < n.max(1))) {
        return Err(cusolverStatus_t::INVALID_VALUE);
    }
    if lwork < n * n + 2 || work.is_null() || (s.is_null() && n > 0) {
        return Err(cusolverStatus_t::INVALID_VALUE);
    }
    set_info(h, info, 0)?;
    if n == 0 {
        return Ok(());
    }
    let (m, n, lda) = (m as i64, n as i64, lda as i64);
    let v = work;
    let count = work.cast::<u8>().add(n as usize * n as usize * value_bytes(double)).cast::<c_void>();
    copy(h, double, std::ptr::null(), 1, v, n, n, n, true)?;
    if !jacobi(h, double, a, m, n, lda, v, n, count)? {
        set_info(h, info, 1)?;
    }
    let (mut pa, mut ps, mut m_, mut n_, mut lda_) = (a, s, m, n, lda);
    run(
        h,
        kernel(double, [c"agx_jacobi_norms", c"agx_jacobi_norms_f64"]),
        &mut [arg(&mut pa), arg(&mut m_), arg(&mut n_), arg(&mut lda_), arg(&mut ps)],
        [n, 1],
        [32, 1],
    )?;
    sort(h, double, s, n, a, m, lda, v, n, n, false)?;
    if b"AS".contains(&ju) {
        copy(h, double, a, lda, u, ldu as i64, m, n, false)?;
    }
    let transpose = |dst: *mut c_void, ldd: i64| {
        let (mut src, mut dst, mut lds, mut ldd, mut m_, mut n_) = (v, dst, n, ldd, n, n);
        run(
            h,
            kernel(double, [c"agx_sv_transpose", c"agx_sv_transpose_f64"]),
            &mut [arg(&mut src), arg(&mut lds), arg(&mut dst), arg(&mut ldd), arg(&mut m_), arg(&mut n_)],
            [n, n],
            [16, 4],
        )
    };
    match jv {
        b'A' | b'S' => transpose(vt, ldvt as i64),
        // V^T over the first n rows of A
        b'O' => transpose(a, lda),
        _ => Ok(()),
    }
}

/// syevd: the eigenvalues (ascending) and with `vectors` the eigenvectors of the symmetric A, from one-sided Jacobi
/// on A + |A|_F I. The workspace holds that matrix, V and two words.
#[allow(clippy::too_many_arguments)]
unsafe fn syevd(
    h: &Handle,
    double: bool,
    vectors: bool,
    uplo: cublasFillMode_t,
    n: c_int,
    a: *mut c_void,
    lda: c_int,
    w: *mut c_void,
    work: *mut c_void,
    lwork: c_int,
    info: *mut c_int,
) -> R {
    let up = upper(uplo)?;
    dims(n, n, lda)?;
    if lwork < 2 * n * n + 4 || work.is_null() || (w.is_null() && n > 0) {
        return Err(cusolverStatus_t::INVALID_VALUE);
    }
    set_info(h, info, 0)?;
    if n == 0 {
        return Ok(());
    }
    let (n, lda) = (n as i64, lda as i64);
    let at = |elems: usize| work.cast::<u8>().add(elems * value_bytes(double)).cast::<c_void>();
    let nn = (n * n) as usize;
    let (wm, v, slot, count) = (work, at(nn), at(2 * nn), at(2 * nn + 2));
    let (mut pa, mut lda_, mut n_, mut up_, mut pslot, mut pw, mut pv) = (a, lda, n, up as i64, slot, wm, v);
    run(
        h,
        kernel(double, [c"agx_eig_shift", c"agx_eig_shift_f64"]),
        &mut [arg(&mut pa), arg(&mut lda_), arg(&mut n_), arg(&mut up_), arg(&mut pslot)],
        [1, 1],
        [1, 1],
    )?;
    run(
        h,
        kernel(double, [c"agx_eig_prep", c"agx_eig_prep_f64"]),
        &mut [arg(&mut pa), arg(&mut lda_), arg(&mut n_), arg(&mut up_), arg(&mut pw), arg(&mut pv), arg(&mut pslot)],
        [n, n],
        [16, 4],
    )?;
    if !jacobi(h, double, wm, n, n, n, v, n, count)? {
        set_info(h, info, 1)?;
    }
    let mut pvals = w;
    run(
        h,
        kernel(double, [c"agx_eig_values", c"agx_eig_values_f64"]),
        &mut [arg(&mut pa), arg(&mut lda_), arg(&mut n_), arg(&mut up_), arg(&mut pv), arg(&mut pvals)],
        [n, 1],
        [32, 1],
    )?;
    sort(h, double, w, n, std::ptr::null_mut(), 0, 1, v, n, n, true)?;
    if vectors {
        copy(h, double, v, n, a, lda, n, n, false)?;
    }
    Ok(())
}

/// `cusolverEigMode_t`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct cusolverEigMode_t(pub u32);

impl cusolverEigMode_t {
    pub const NOVECTOR: cusolverEigMode_t = cusolverEigMode_t(0);
    pub const VECTOR: cusolverEigMode_t = cusolverEigMode_t(1);
}

fn eig_mode(j: cusolverEigMode_t) -> Result<bool, cusolverStatus_t> {
    match j {
        cusolverEigMode_t::NOVECTOR => Ok(false),
        cusolverEigMode_t::VECTOR => Ok(true),
        _ => Err(cusolverStatus_t::INVALID_VALUE),
    }
}

macro_rules! eigen {
    ($double:literal, $ty:ty, $gesvd_bs:ident, $gesvd:ident, $syevd_bs:ident, $syevd:ident) => {
        #[cfg_attr(not(test), no_mangle)]
        pub unsafe extern "system" fn $gesvd_bs(h: cusolverDnHandle_t, m: c_int, n: c_int, lwork: *mut c_int) -> cusolverStatus_t {
            done(handle(h).and_then(|_| dims(m, n, m.max(1))).and_then(|_| out(lwork, (n as usize) * (n as usize) + 2)))
        }

        #[cfg_attr(not(test), no_mangle)]
        pub unsafe extern "system" fn $gesvd(
            h: cusolverDnHandle_t,
            jobu: i8,
            jobvt: i8,
            m: c_int,
            n: c_int,
            a: *mut $ty,
            lda: c_int,
            s: *mut $ty,
            u: *mut $ty,
            ldu: c_int,
            vt: *mut $ty,
            ldvt: c_int,
            work: *mut $ty,
            lwork: c_int,
            _rwork: *mut $ty,
            info: *mut c_int,
        ) -> cusolverStatus_t {
            done(
                handle(h).and_then(|h| gesvd(h, $double, jobu, jobvt, m, n, a.cast(), lda, s.cast(), u.cast(), ldu, vt.cast(), ldvt, work.cast(), lwork, info)),
            )
        }

        #[cfg_attr(not(test), no_mangle)]
        pub unsafe extern "system" fn $syevd_bs(
            h: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: c_int,
            _a: *const $ty,
            lda: c_int,
            _w: *const $ty,
            lwork: *mut c_int,
        ) -> cusolverStatus_t {
            done(
                handle(h)
                    .and_then(|_| eig_mode(jobz))
                    .and_then(|_| upper(uplo))
                    .and_then(|_| dims(n, n, lda))
                    .and_then(|_| out(lwork, 2 * (n as usize) * (n as usize) + 4)),
            )
        }

        #[cfg_attr(not(test), no_mangle)]
        pub unsafe extern "system" fn $syevd(
            h: cusolverDnHandle_t,
            jobz: cusolverEigMode_t,
            uplo: cublasFillMode_t,
            n: c_int,
            a: *mut $ty,
            lda: c_int,
            w: *mut $ty,
            work: *mut $ty,
            lwork: c_int,
            info: *mut c_int,
        ) -> cusolverStatus_t {
            done(handle(h).and_then(|h| syevd(h, $double, eig_mode(jobz)?, uplo, n, a.cast(), lda, w.cast(), work.cast(), lwork, info)))
        }
    };
}

eigen!(false, f32, cusolverDnSgesvd_bufferSize, cusolverDnSgesvd, cusolverDnSsyevd_bufferSize, cusolverDnSsyevd);
eigen!(true, f64, cusolverDnDgesvd_bufferSize, cusolverDnDgesvd, cusolverDnDsyevd_bufferSize, cusolverDnDsyevd);

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "system" fn cusolverDnCreate(h: *mut cusolverDnHandle_t) -> cusolverStatus_t {
    let Some(h) = h.as_mut() else {
        return cusolverStatus_t::INVALID_VALUE;
    };
    if !engine::available() {
        return cusolverStatus_t::NOT_INITIALIZED;
    }
    *h = Box::into_raw(Box::new(Handle { stream: CUstream(std::ptr::null_mut()) }));
    cusolverStatus_t::SUCCESS
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "system" fn cusolverDnDestroy(h: cusolverDnHandle_t) -> cusolverStatus_t {
    if h.is_null() {
        return cusolverStatus_t::NOT_INITIALIZED;
    }
    drop(Box::from_raw(h));
    cusolverStatus_t::SUCCESS
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "system" fn cusolverDnSetStream(h: cusolverDnHandle_t, s: CUstream) -> cusolverStatus_t {
    match h.as_mut() {
        Some(h) => {
            h.stream = s;
            cusolverStatus_t::SUCCESS
        }
        None => cusolverStatus_t::NOT_INITIALIZED,
    }
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "system" fn cusolverDnGetStream(h: cusolverDnHandle_t, s: *mut CUstream) -> cusolverStatus_t {
    match (h.as_ref(), s.as_mut()) {
        (Some(h), Some(s)) => {
            *s = h.stream;
            cusolverStatus_t::SUCCESS
        }
        (None, _) => cusolverStatus_t::NOT_INITIALIZED,
        _ => cusolverStatus_t::INVALID_VALUE,
    }
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "system" fn cusolverGetVersion(v: *mut c_int) -> cusolverStatus_t {
    match v.as_mut() {
        Some(v) => {
            *v = VERSION.0 * 1000 + VERSION.1 * 100 + VERSION.2;
            cusolverStatus_t::SUCCESS
        }
        None => cusolverStatus_t::INVALID_VALUE,
    }
}

#[cfg_attr(not(test), no_mangle)]
pub unsafe extern "system" fn cusolverGetProperty(ty: libraryPropertyType_t, v: *mut c_int) -> cusolverStatus_t {
    let Some(v) = v.as_mut() else {
        return cusolverStatus_t::INVALID_VALUE;
    };
    *v = match ty {
        libraryPropertyType_t::MAJOR_VERSION => VERSION.0,
        libraryPropertyType_t::MINOR_VERSION => VERSION.1,
        libraryPropertyType_t::PATCH_LEVEL => VERSION.2,
        _ => return cusolverStatus_t::INVALID_VALUE,
    };
    cusolverStatus_t::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn argument_checks() {
        let h = Handle { stream: CUstream(std::ptr::null_mut()) };
        let p = 8usize as *mut c_void;
        unsafe {
            assert_eq!(getrf(&h, false, 4, 4, p, 3, p.cast(), p.cast()), Err(cusolverStatus_t::INVALID_VALUE));
            assert_eq!(getrf(&h, false, 4, 4, p, 4, std::ptr::null_mut(), p.cast()), Err(cusolverStatus_t::NOT_SUPPORTED));
            assert_eq!(potrf(&h, false, cublasFillMode_t(9), 4, p, 4, p.cast()), Err(cusolverStatus_t::INVALID_VALUE));
            assert_eq!(potrf(&h, false, cublasFillMode_t::CUBLAS_FILL_MODE_LOWER, 3, p, 4, std::ptr::null_mut()), Err(cusolverStatus_t::INVALID_VALUE));
            assert_eq!(orgqr(&h, false, 4, 5, 2, p, 4, p, p, 100, p.cast()), Err(cusolverStatus_t::INVALID_VALUE));
            assert_eq!(orgqr(&h, false, 4, 3, 2, p, 4, p, p, 7, p.cast()), Err(cusolverStatus_t::INVALID_VALUE));
        }
        assert_eq!(ormqr_check(true, 5, 3, 6, 5, 5), Err(cusolverStatus_t::INVALID_VALUE));
        assert_eq!(ormqr_check(false, 5, 3, 3, 3, 5), Ok(()));
        let mut lw = 0;
        unsafe {
            let hp = &h as *const Handle as cusolverDnHandle_t;
            assert_eq!(cusolverDnDorgqr_bufferSize(hp, 6, 4, 3, std::ptr::null(), 6, std::ptr::null(), &mut lw), cusolverStatus_t::SUCCESS);
        }
        assert_eq!(lw, 18);
        unsafe {
            let p = 8usize as *mut c_void;
            assert_eq!(gesvd(&h, false, b'A' as i8, b'N' as i8, 5, 4, p, 5, p, p, 5, p, 4, p, 100, p.cast()), Err(cusolverStatus_t::NOT_SUPPORTED));
            assert_eq!(gesvd(&h, false, b'O' as i8, b'O' as i8, 5, 4, p, 5, p, p, 5, p, 4, p, 100, p.cast()), Err(cusolverStatus_t::INVALID_VALUE));
            assert_eq!(gesvd(&h, false, b'S' as i8, b'S' as i8, 3, 4, p, 5, p, p, 5, p, 4, p, 100, p.cast()), Err(cusolverStatus_t::INVALID_VALUE));
            assert_eq!(syevd(&h, false, true, cublasFillMode_t::CUBLAS_FILL_MODE_LOWER, 4, p, 4, p, p, 35, p.cast()), Err(cusolverStatus_t::INVALID_VALUE));
        }
    }

    #[test]
    fn version() {
        let mut v = 0;
        unsafe { cusolverGetVersion(&mut v) };
        assert_eq!(v, 11603);
    }
}
