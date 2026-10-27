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
    run_in(h, agx_libs::SOLVER, name, params, grid, block)
}

/// A kernel of another agx_libs object (the BLAS triangular solve of the blocked factorizations).
unsafe fn run_in(h: &Handle, image: &'static [u8], name: &'static CStr, params: &mut [*mut c_void], grid: [i64; 2], block: [u32; 2]) -> R {
    if grid[0] <= 0 || grid[1] <= 0 {
        return Ok(());
    }
    let groups = |n: i64, b: u32| u32::try_from((n as u64).div_ceil(b as u64)).map_err(|_| cusolverStatus_t::NOT_SUPPORTED);
    let f = engine::function(image, name).map_err(status)?;
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

/// The f32 triangular solves `op(T) X = B` (T n x n, X overwriting the n x nrhs B), on the blocked BLAS trsm, in
/// order: (upper, trans, unit) for each.
unsafe fn solves(h: &Handle, n: i64, nrhs: i64, a: *const c_void, lda: i64, b: *mut c_void, ldb: i64, steps: &[(bool, bool, bool)]) -> R {
    for &(upper, trans, unit) in steps {
        let t = engine::Trsm { right: false, upper, trans, unit, m: n, n: nrhs, alpha: 1.0, a, lda, b, ldb };
        engine::trsm(&t, h.stream).map_err(status)?;
    }
    Ok(())
}

/// Above this many rows of A, the f32 getrs and potrs solve on the blocked BLAS trsm instead of one thread per
/// right-hand side.
const SOLVE_BLOCKED_MIN: i64 = 64;

/// The columns of a panel of the blocked f32 getrf.
const LU_BLOCK: i64 = 64;

/// getrf: P A = L U, one column at a time (pivot, swap, scale, update). In f32 with more than LU_BLOCK columns to
/// factor, by panels of LU_BLOCK columns (right-looking): the columns of a panel as before, with the updates kept to
/// the panel (row swaps still span A), then U12 = L11^-1 A12 (the BLAS triangular solve) and A22 -= A21 U12 (a GEMM
/// on the tile kernels). The pivots are the same; the products are summed in another order.
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
    let mn = m.min(n) as i64;
    let block = if double || mn <= LU_BLOCK { mn } else { LU_BLOCK };
    let at = |r: i64, c: i64| (a as *mut f32).add((r + c * lda as i64) as usize) as *mut c_void;
    for k in 0..mn {
        let mut k_ = k;
        // the end of k's panel: the update stays in it
        let (j0, mut j1) = (k / block * block, (k / block * block + block).min(mn));
        if block == mn {
            j1 = n_;
        }
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
            &mut [arg(&mut pa), arg(&mut m_), arg(&mut j1), arg(&mut lda_), arg(&mut k_)],
            [m_ - k - 1, j1 - k - 1],
            [16, 4],
        )?;
        if k + 1 != j1 || j1 >= n_ {
            continue;
        }
        // the panel j0..j1 is factored: U12 = L11^-1 A12 (unit lower), then A22 -= A21 U12
        let (jb, rest) = (j1 - j0, n_ - j1);
        let a12 = at(j0, j1);
        let u12 = engine::Trsm { right: false, upper: false, trans: false, unit: true, m: jb, n: rest, alpha: 1.0, a: at(j0, j0), lda: lda_, b: a12, ldb: lda_ };
        engine::trsm(&u12, h.stream).map_err(status)?;
        if m_ > j1 {
            let g = engine::Gemm {
                types: agx_libs::Types::S,
                ta: false,
                tb: false,
                m: (m_ - j1) as u64,
                n: rest as u64,
                k: jb as u64,
                alpha: -1.0,
                beta: 1.0,
                a: at(j1, j0),
                lda: lda_ as u64,
                stride_a: 0,
                b: a12,
                ldb: lda_ as u64,
                stride_b: 0,
                c: at(j1, j1),
                ldc: lda_ as u64,
                stride_c: 0,
                batch: 1,
            };
            engine::gemm(&g, h.stream).map_err(status)?;
        }
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
    if n == 0 || nrhs == 0 {
        return Ok(());
    }
    if !double && n_ > SOLVE_BLOCKED_MIN {
        // P A = L U: X = U^-1 L^-1 P B, or for A^T X = B, X = P^T L^-T U^-T B
        let pivots = |reverse: i64| {
            let (mut pi, mut pb, mut n_, mut nrhs_, mut ldb_, mut reverse) = (pi, pb, n_, nrhs_, ldb_, reverse);
            run(
                h,
                c"agx_getrs_pivots",
                &mut [arg(&mut pi), arg(&mut pb), arg(&mut n_), arg(&mut nrhs_), arg(&mut ldb_), arg(&mut reverse)],
                [nrhs_, 1],
                [32, 1],
            )
        };
        if t {
            solves(h, n_, nrhs_, a, lda_, b, ldb_, &[(true, true, false), (false, true, true)])?;
            return pivots(1);
        }
        pivots(0)?;
        return solves(h, n_, nrhs_, a, lda_, b, ldb_, &[(false, false, true), (true, false, false)]);
    }
    run(
        h,
        kernel(double, [c"agx_getrs", c"agx_getrs_f64"]),
        &mut [arg(&mut pa), arg(&mut n_), arg(&mut lda_), arg(&mut pi), arg(&mut pb), arg(&mut nrhs_), arg(&mut ldb_), arg(&mut t_)],
        [nrhs_, 1],
        [32, 1],
    )
}

/// potrf: A = L L^T or U^T U, one column at a time (diagonal, scale, update). In f32 above LU_BLOCK rows, by panels of
/// LU_BLOCK columns: the columns of a panel as before with the update kept to the panel, then the rest of the
/// triangle updated with the panel by a syrk on the tile GEMM. After a non-positive minor the later kernels do
/// nothing, but the syrk of that panel still runs on the trailing matrix.
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
    let lda_ = lda as i64;
    let block = if double || n_ <= LU_BLOCK { n_ } else { LU_BLOCK };
    let at = |r: i64, c: i64| (a as *mut f32).add((r + c * lda_) as usize) as *mut c_void;
    for k in 0..n as i64 {
        let mut k_ = k;
        // the end of k's panel: the update stays in it
        let (j0, mut j1) = (k / block * block, (k / block * block + block).min(n_));
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
            &mut [arg(&mut pa), arg(&mut n_), arg(&mut k_), arg(&mut rs), arg(&mut cs), arg(&mut pinfo), arg(&mut j1)],
            [n_ - k - 1, j1 - k - 1],
            [16, 4],
        )?;
        if k + 1 != j1 || j1 >= n_ {
            continue;
        }
        // the panel j0..j1 is factored: A22 -= L21 L21^T on the stored triangle (U12^T U12 for upper), a syrk of the
        // panel's rows below it, as in zluda_blas_agx: a GEMM on the tile kernels for each corner of the triangle
        // outside the diagonal blocks, and agx_ssyrk on those
        let (jb, rest) = (j1 - j0, n_ - j1);
        let panel = if up { at(j0, j1) } else { at(j1, j0) };
        let a22 = at(j1, j1);
        // rows r.. of op(panel): L21's rows, or U12's columns
        let rows = |r: i64| (panel as *mut f32).add(if up { r * lda_ } else { r } as usize) as *const c_void;
        for [first, second, m, w] in agx_libs::syrk_corners(up, rest, LU_BLOCK) {
            let g = engine::Gemm {
                types: agx_libs::Types::S,
                ta: up,
                tb: !up,
                m: m as u64,
                n: w as u64,
                k: jb as u64,
                alpha: -1.0,
                beta: 1.0,
                a: rows(first),
                lda: lda_ as u64,
                stride_a: 0,
                b: rows(second),
                ldb: lda_ as u64,
                stride_b: 0,
                c: (a22 as *mut f32).add((first + second * lda_) as usize) as *mut c_void,
                ldc: lda_ as u64,
                stride_c: 0,
                batch: 1,
            };
            engine::gemm(&g, h.stream).map_err(status)?;
        }
        let (mut p, mut c, mut nn, mut kk, mut l1, mut l2) = (panel, a22, rest, jb, lda_, lda_);
        let (mut upper_, mut trans, mut alpha, mut beta, mut nb) = (up as i64, up as i64, -1.0f32, 1.0f32, LU_BLOCK);
        run_in(
            h,
            agx_libs::BLAS,
            c"agx_ssyrk",
            &mut [
                arg(&mut p),
                arg(&mut c),
                arg(&mut nn),
                arg(&mut kk),
                arg(&mut l1),
                arg(&mut l2),
                arg(&mut upper_),
                arg(&mut trans),
                arg(&mut alpha),
                arg(&mut beta),
                arg(&mut nb),
            ],
            [rest, rest],
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
    if n == 0 || nrhs == 0 {
        return Ok(());
    }
    if !double && n as i64 > SOLVE_BLOCKED_MIN {
        // A = L L^T: X = L^-T L^-1 B; A = U^T U: X = U^-1 U^-T B
        let steps = if up { [(true, true, false), (true, false, false)] } else { [(false, false, false), (false, true, false)] };
        return solves(h, n as i64, nrhs as i64, a, lda as i64, b, ldb as i64, &steps);
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

/// The columns of a panel of the blocked f32 geqrf.
const QR_BLOCK: i64 = 32;

/// The f32 elements of workspace the blocked geqrf uses (0: it is not blocked): V (m x QR_BLOCK), S and T
/// (QR_BLOCK x QR_BLOCK) and two QR_BLOCK x n products.
fn geqrf_work(double: bool, m: c_int, n: c_int) -> i64 {
    let (m, n) = (m as i64, n as i64);
    if double || m.min(n) <= QR_BLOCK || n <= QR_BLOCK {
        return 0;
    }
    QR_BLOCK * (m + 2 * QR_BLOCK + 2 * n)
}

/// geqrf: A = Q R with Householder reflectors, one column at a time. In f32 with a workspace of geqrf_work elements,
/// by panels of QR_BLOCK columns: the columns of a panel as before with the reflections kept to the panel, then the
/// panel's Q^T = I - V T^T V^T applied to the rest of A with GEMMs on the tile kernels (V by agx_qr_v, S = V^T V,
/// T by agx_qr_t, W = V^T A2, W2 = T^T W, A2 -= V W2). The reflectors are the same; the products are summed in
/// another order.
#[allow(clippy::too_many_arguments)]
unsafe fn geqrf(h: &Handle, double: bool, m: c_int, n: c_int, a: *mut c_void, lda: c_int, tau: *mut c_void, work: *mut c_void, lwork: c_int, info: *mut c_int) -> R {
    dims(m, n, lda)?;
    if tau.is_null() && m.min(n) > 0 {
        return Err(cusolverStatus_t::INVALID_VALUE);
    }
    set_info(h, info, 0)?;
    let (mut pa, mut pt) = (a, tau);
    let (mut m_, n_, mut lda_) = (m as i64, n as i64, lda as i64);
    let need = geqrf_work(double, m, n);
    let block = if need > 0 && !work.is_null() && lwork as i64 >= need { QR_BLOCK } else { i64::MAX };
    let mn = m.min(n) as i64;
    let f = |p: *mut c_void, i: i64| (p as *mut f32).add(i as usize) as *mut c_void;
    for k in 0..mn {
        let mut k_ = k;
        // the end of k's panel: the reflections stay in it
        let j0 = if block == i64::MAX { 0 } else { k / block * block };
        let mut j1 = if block == i64::MAX { n_ } else { (j0 + block).min(mn) };
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
            &mut [arg(&mut pa), arg(&mut m_), arg(&mut j1), arg(&mut k_), arg(&mut lda_), arg(&mut pt)],
            [j1 - k - 1, 1],
            [64, 1],
        )?;
        if k + 1 != j1 || j1 >= n_ {
            continue;
        }
        let (jb, mv, n2) = (j1 - j0, m_ - j0, n_ - j1);
        let (v, sm, t) = (work, f(work, QR_BLOCK * m_), f(work, QR_BLOCK * (m_ + QR_BLOCK)));
        let (w, w2) = (f(work, QR_BLOCK * (m_ + 2 * QR_BLOCK)), f(work, QR_BLOCK * (m_ + 2 * QR_BLOCK + n_)));
        let (mut pv, mut j0_, mut jb_) = (v, j0, jb);
        run(h, c"agx_qr_v", &mut [arg(&mut pa), arg(&mut pv), arg(&mut m_), arg(&mut j0_), arg(&mut jb_), arg(&mut lda_)], [mv, jb], [16, 4])?;
        let gemm = |ta: bool, m: i64, n: i64, k: i64, alpha: f64, a: *const c_void, lda: i64, b: *const c_void, ldb: i64, beta: f64, c: *mut c_void, ldc: i64| {
            let g = engine::Gemm {
                types: agx_libs::Types::S,
                ta,
                tb: false,
                m: m as u64,
                n: n as u64,
                k: k as u64,
                alpha,
                beta,
                a,
                lda: lda as u64,
                stride_a: 0,
                b,
                ldb: ldb as u64,
                stride_b: 0,
                c,
                ldc: ldc as u64,
                stride_c: 0,
                batch: 1,
            };
            engine::gemm(&g, h.stream).map_err(status)
        };
        gemm(true, jb, jb, mv, 1.0, v, mv, v, mv, 0.0, sm, jb)?;
        let (mut ps, mut ptt) = (sm, t);
        run(h, c"agx_qr_t", &mut [arg(&mut ps), arg(&mut pt), arg(&mut ptt), arg(&mut j0_), arg(&mut jb_)], [1, 1], [1, 1])?;
        let a2 = f(a, j0 + j1 * lda_);
        gemm(true, jb, n2, mv, 1.0, v, mv, a2, lda_, 0.0, w, jb)?;
        gemm(true, jb, n2, jb, 1.0, t, jb, w, jb, 0.0, w2, jb)?;
        gemm(false, mv, n2, jb, -1.0, v, mv, w2, jb, 1.0, a2, lda_)?;
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
    work: *mut c_void,
    lwork: i64,
) -> R {
    let need = apply_q_work(double, left, m, n, k);
    if need > 0 && !work.is_null() && lwork >= need {
        return apply_q_blocked(h, left, t, m as i64, n as i64, k as i64, r, ldr as i64, tau, c, ldc as i64, work);
    }
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

/// The f32 elements of workspace the blocked [`apply_q`] uses (0: it is not blocked): V (q x QR_BLOCK, q the order
/// of Q), S and T, and two products of QR_BLOCK and the other dimension of C.
fn apply_q_work(double: bool, left: bool, m: c_int, n: c_int, k: c_int) -> i64 {
    let (q, other) = if left { (m as i64, n as i64) } else { (n as i64, m as i64) };
    if double || k < 4 || q < 64 {
        return 0;
    }
    QR_BLOCK * (q + 2 * QR_BLOCK + 2 * other)
}

/// op(Q) C or C op(Q) by blocks of QR_BLOCK reflectors, each Q_b = I - V T V^T (agx_qr_v, S = V^T V, agx_qr_t) applied
/// with GEMMs on the tile kernels: on the left C -= V op(T) (V^T C), on the right C -= (C V) op(T) V^T, on the rows or
/// columns from the block's first reflector. Q C and C Q^T take the blocks from the last, Q^T C and C Q from the first.
#[allow(clippy::too_many_arguments)]
unsafe fn apply_q_blocked(h: &Handle, left: bool, t: bool, m: i64, n: i64, k: i64, r: *const c_void, ldr: i64, tau: *const c_void, c: *mut c_void, ldc: i64, work: *mut c_void) -> R {
    let (q, other) = if left { (m, n) } else { (n, m) };
    let f = |p: *mut c_void, i: i64| (p as *mut f32).add(i as usize) as *mut c_void;
    let (v, sm, tm) = (work, f(work, QR_BLOCK * q), f(work, QR_BLOCK * (q + QR_BLOCK)));
    let (w, w2) = (f(work, QR_BLOCK * (q + 2 * QR_BLOCK)), f(work, QR_BLOCK * (q + 2 * QR_BLOCK + other)));
    let gemm = |ta: bool, tb: bool, m: i64, n: i64, k: i64, alpha: f64, a: *const c_void, lda: i64, b: *const c_void, ldb: i64, beta: f64, c: *mut c_void, ldc: i64| {
        let g = engine::Gemm {
            types: agx_libs::Types::S,
            ta,
            tb,
            m: m as u64,
            n: n as u64,
            k: k as u64,
            alpha,
            beta,
            a,
            lda: lda as u64,
            stride_a: 0,
            b,
            ldb: ldb as u64,
            stride_b: 0,
            c,
            ldc: ldc as u64,
            stride_c: 0,
            batch: 1,
        };
        engine::gemm(&g, h.stream).map_err(status)
    };
    let mut blocks: Vec<i64> = (0..k).step_by(QR_BLOCK as usize).collect();
    if left != t {
        blocks.reverse();
    }
    for j0 in blocks {
        let (jb, mv) = ((k - j0).min(QR_BLOCK), q - j0);
        let (mut pr, mut pv, mut q_, mut j0_, mut jb_, mut ldr_) = (r as *mut c_void, v, q, j0, jb, ldr);
        run(h, c"agx_qr_v", &mut [arg(&mut pr), arg(&mut pv), arg(&mut q_), arg(&mut j0_), arg(&mut jb_), arg(&mut ldr_)], [mv, jb], [16, 4])?;
        gemm(true, false, jb, jb, mv, 1.0, v, mv, v, mv, 0.0, sm, jb)?;
        let (mut ps, mut pt, mut ptm) = (sm, tau as *mut c_void, tm);
        run(h, c"agx_qr_t", &mut [arg(&mut ps), arg(&mut pt), arg(&mut ptm), arg(&mut j0_), arg(&mut jb_)], [1, 1], [1, 1])?;
        if left {
            let cs = f(c, j0);
            gemm(true, false, jb, n, mv, 1.0, v, mv, cs, ldc, 0.0, w, jb)?;
            gemm(t, false, jb, n, jb, 1.0, tm, jb, w, jb, 0.0, w2, jb)?;
            gemm(false, false, mv, n, jb, -1.0, v, mv, w2, jb, 1.0, cs, ldc)?;
        } else {
            let cs = f(c, j0 * ldc);
            gemm(false, false, m, jb, mv, 1.0, cs, ldc, v, mv, 0.0, w, m)?;
            gemm(false, t, m, jb, jb, 1.0, w, m, tm, jb, 0.0, w2, m)?;
            gemm(false, true, m, mv, jb, -1.0, w2, m, v, mv, 1.0, cs, ldc)?;
        }
    }
    Ok(())
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
    work: *mut c_void,
    lwork: c_int,
    info: *mut c_int,
) -> R {
    let (left, t) = (side(s)?, trans(op)?);
    ormqr_check(left, m, n, k, lda, ldc)?;
    set_info(h, info, 0)?;
    apply_q(h, double, left, t, m, n, k, a, lda, tau, c, ldc, work, lwork as i64)
}

/// orgqr: the first n columns of Q = H(0) ... H(k - 1), with the reflectors copied to the workspace (m k values)
/// and A set to the identity first; the rest of the workspace, when the query's size, for the blocked application.
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
    // the rest of the workspace for the blocked application
    let rest = (work as *mut u8).add(if double { 8 } else { 4 } * (m * k) as usize) as *mut c_void;
    apply_q(h, double, true, false, m, n, k, work, m, tau, a, lda, rest, lwork as i64 - (m * k) as i64)
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
            done(handle(h).and_then(|_| dims(m, n, lda)).and_then(|_| out(lwork, geqrf_work($double, m, n) as usize)))
        }

        #[cfg_attr(not(test), no_mangle)]
        pub unsafe extern "system" fn $geqrf(
            h: cusolverDnHandle_t,
            m: c_int,
            n: c_int,
            a: *mut $ty,
            lda: c_int,
            tau: *mut $ty,
            work: *mut $ty,
            lwork: c_int,
            info: *mut c_int,
        ) -> cusolverStatus_t {
            done(handle(h).and_then(|h| geqrf(h, $double, m, n, a.cast(), lda, tau.cast(), work.cast(), lwork, info)))
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
            done(
                handle(h)
                    .and_then(|_| ormqr_check(side(s)?, m, n, k, lda, ldc))
                    .and_then(|_| trans(op))
                    .and_then(|_| out(lwork, apply_q_work($double, side(s)?, m, n, k) as usize)),
            )
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
            work: *mut $ty,
            lwork: c_int,
            info: *mut c_int,
        ) -> cusolverStatus_t {
            done(handle(h).and_then(|h| ormqr(h, $double, s, op, m, n, k, a.cast(), lda, tau.cast(), c.cast(), ldc, work.cast(), lwork, info)))
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
            done(handle(h).and_then(|_| dims(m, n, lda)).and_then(|_| out(lwork, (m as usize) * (k.max(0) as usize) + apply_q_work($double, true, m, n, k) as usize)))
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
