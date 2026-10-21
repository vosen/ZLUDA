//! cuBLAS on Apple GPUs for zluda_agx (Asahi Linux).
//!
//! The work is done by the `agx_libs` engine: MLIR kernels (`agx_libs/kernels/*.mlir`, written once for every AGX
//! generation) in fat AGX objects that zluda_agx's `cuModuleLoadData` picks from, launched through the driver API of
//! the process's libcuda (zluda_agx), as NVIDIA's cuBLAS does.
//!
//! Supported so far: handles, streams, pointer and math modes, and GEMM in single and half precision (`cublasSgemm`,
//! `cublasHgemm`, their strided batched forms, and `cublasGemmEx` / `cublasGemmStridedBatchedEx` with f32 A, B and C,
//! or f16 A and B and f16 or f32 C), always accumulated in f32, and `cublasDgemm`; and the single and double precision
//! level 1 routines (axpy, scal, copy, swap, dot, asum, nrm2, amax, amin), level 2 gemv and ger, and level 3 geam,
//! dgmm, syrk, symm, trmm, trsm and batched GEMM (`cublasS*` and `cublasD*`, with their `_64` forms). The double
//! precision routines run the f64 copies of the f32 kernels (agx_libs/build.rs), in the backend's software double
//! precision.
//! Reductions are deterministic: fixed per-thread partial sums combined in order. Other functions return
//! `CUBLAS_STATUS_NOT_SUPPORTED`.

#![allow(non_snake_case)]

use agx_libs::{engine, f16_to_f32, Types};
use cuda_types::cublas::*;
use cuda_types::cuda::*;
use std::ffi::{c_char, c_int, c_longlong, c_void, CStr};

macro_rules! unimplemented_fn {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes_definitions, unused_variables)]
            pub extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                if std::env::var_os("ZLUDA_AGX_LOG").is_some() {
                    eprintln!("[zluda_blas_agx] {} is not implemented", stringify!($fn_name));
                }
                Err(cublasError_t::NOT_SUPPORTED)
            }
        )*
    };
}

macro_rules! implemented_fn {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes_definitions)]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                imp::$fn_name($($arg_id),*)
            }
        )*
    };
}

cuda_macros::cublas_function_declarations!(
    unimplemented_fn,
    implemented_fn
        <= [
            cublasCreate_v2,
            cublasDestroy_v2,
            cublasGemmEx,
            cublasGemmStridedBatchedEx,
            cublasGetCudartVersion,
            cublasGetMathMode,
            cublasGetPointerMode_v2,
            cublasGetStatusName,
            cublasGetStatusString,
            cublasGetStream_v2,
            cublasGetVersion_v2,
            cublasHgemm,
            cublasHgemmStridedBatched,
            cublasIsamax_v2,
            cublasIsamax_v2_64,
            cublasIsamin_v2,
            cublasIsamin_v2_64,
            cublasSasum_v2,
            cublasSasum_v2_64,
            cublasSaxpy_v2,
            cublasSaxpy_v2_64,
            cublasScopy_v2,
            cublasScopy_v2_64,
            cublasSdgmm,
            cublasSdgmm_64,
            cublasSdot_v2,
            cublasSdot_v2_64,
            cublasSetMathMode,
            cublasSetPointerMode_v2,
            cublasSetStream_v2,
            cublasSetWorkspace_v2,
            cublasDgemmStridedBatched,
            cublasDgemm_v2,
            cublasSgeam,
            cublasSgeam_64,
            cublasSgemmBatched,
            cublasSgemmBatched_64,
            cublasSgemmStridedBatched,
            cublasSgemm_v2,
            cublasSgemv_v2,
            cublasSgemv_v2_64,
            cublasSger_v2,
            cublasSger_v2_64,
            cublasSnrm2_v2,
            cublasSnrm2_v2_64,
            cublasSscal_v2,
            cublasSscal_v2_64,
            cublasSswap_v2,
            cublasSswap_v2_64,
            cublasSsymm_v2,
            cublasSsymm_v2_64,
            cublasSsyrk_v2,
            cublasSsyrk_v2_64,
            cublasStrmm_v2,
            cublasStrmm_v2_64,
            cublasStrsm_v2,
            cublasStrsm_v2_64,
            cublasXerbla,
            cublasDaxpy_v2,
            cublasDaxpy_v2_64,
            cublasDscal_v2,
            cublasDscal_v2_64,
            cublasDcopy_v2,
            cublasDcopy_v2_64,
            cublasDswap_v2,
            cublasDswap_v2_64,
            cublasDdot_v2,
            cublasDdot_v2_64,
            cublasDasum_v2,
            cublasDasum_v2_64,
            cublasDnrm2_v2,
            cublasDnrm2_v2_64,
            cublasDgemv_v2,
            cublasDgemv_v2_64,
            cublasDger_v2,
            cublasDger_v2_64,
            cublasDsymm_v2,
            cublasDsymm_v2_64,
            cublasDsyrk_v2,
            cublasDsyrk_v2_64,
            cublasDtrmm_v2,
            cublasDtrmm_v2_64,
            cublasDtrsm_v2,
            cublasDtrsm_v2_64,
            cublasIdamax_v2,
            cublasIdamax_v2_64,
            cublasIdamin_v2,
            cublasIdamin_v2_64,
            cublasDgeam,
            cublasDgeam_64,
            cublasDdgmm,
            cublasDdgmm_64,
            cublasDgemmBatched,
            cublasDgemmBatched_64,
        ]
);

mod imp {
    use super::*;
    use engine::Error as E;

    fn status(e: E) -> cublasError_t {
        match e {
            E::NotInitialized => cublasError_t::NOT_INITIALIZED,
            E::ArchMismatch => cublasError_t::ARCH_MISMATCH,
            E::Failed => cublasError_t::EXECUTION_FAILED,
        }
    }

    struct Handle {
        stream: CUstream,
        pointer_mode: cublasPointerMode_t,
        math_mode: cublasMath_t,
        /// The partial sums of reductions and their result (`level1::WORKSPACE` bytes), allocated on first use.
        ws: *mut c_void,
    }

    unsafe fn handle<'a>(h: cublasHandle_t) -> Result<&'a mut Handle, cublasError_t> {
        (h.0 as *mut Handle).as_mut().ok_or(cublasError_t::NOT_INITIALIZED)
    }

    pub(crate) unsafe fn cublasCreate_v2(out: *mut cublasHandle_t) -> cublasStatus_t {
        if !engine::available() {
            return Err(cublasError_t::NOT_INITIALIZED);
        }
        let out = out.as_mut().ok_or(cublasError_t::INVALID_VALUE)?;
        let h = Box::new(Handle {
            stream: CUstream(std::ptr::null_mut()),
            pointer_mode: cublasPointerMode_t::CUBLAS_POINTER_MODE_HOST,
            math_mode: cublasMath_t::CUBLAS_DEFAULT_MATH,
            ws: std::ptr::null_mut(),
        });
        *out = cublasHandle_t(Box::into_raw(h) as *mut cublasContext);
        Ok(())
    }

    pub(crate) unsafe fn cublasDestroy_v2(h: cublasHandle_t) -> cublasStatus_t {
        if h.0.is_null() {
            return Err(cublasError_t::NOT_INITIALIZED);
        }
        let h = Box::from_raw(h.0 as *mut Handle);
        engine::free(h.ws).map_err(status)?;
        Ok(())
    }

    pub(crate) unsafe fn cublasGetVersion_v2(h: cublasHandle_t, v: *mut c_int) -> cublasStatus_t {
        handle(h)?;
        *v.as_mut().ok_or(cublasError_t::INVALID_VALUE)? = 120_400;
        Ok(())
    }

    pub(crate) unsafe fn cublasGetCudartVersion() -> usize {
        12_040
    }

    pub(crate) unsafe fn cublasSetStream_v2(h: cublasHandle_t, s: cudaStream_t) -> cublasStatus_t {
        handle(h)?.stream = s;
        Ok(())
    }

    pub(crate) unsafe fn cublasGetStream_v2(h: cublasHandle_t, s: *mut cudaStream_t) -> cublasStatus_t {
        *s.as_mut().ok_or(cublasError_t::INVALID_VALUE)? = handle(h)?.stream;
        Ok(())
    }

    pub(crate) unsafe fn cublasSetPointerMode_v2(h: cublasHandle_t, m: cublasPointerMode_t) -> cublasStatus_t {
        handle(h)?.pointer_mode = m;
        Ok(())
    }

    pub(crate) unsafe fn cublasGetPointerMode_v2(h: cublasHandle_t, m: *mut cublasPointerMode_t) -> cublasStatus_t {
        *m.as_mut().ok_or(cublasError_t::INVALID_VALUE)? = handle(h)?.pointer_mode;
        Ok(())
    }

    /// Every math mode computes in f32 (there is no TF32 path to allow).
    pub(crate) unsafe fn cublasSetMathMode(h: cublasHandle_t, m: cublasMath_t) -> cublasStatus_t {
        handle(h)?.math_mode = m;
        Ok(())
    }

    pub(crate) unsafe fn cublasGetMathMode(h: cublasHandle_t, m: *mut cublasMath_t) -> cublasStatus_t {
        *m.as_mut().ok_or(cublasError_t::INVALID_VALUE)? = handle(h)?.math_mode;
        Ok(())
    }

    /// The kernels need no workspace.
    pub(crate) unsafe fn cublasSetWorkspace_v2(h: cublasHandle_t, _w: *mut c_void, _size: usize) -> cublasStatus_t {
        handle(h).map(|_| ())
    }

    pub(crate) unsafe fn cublasGetStatusName(s: cublasStatus_t) -> *const c_char {
        agx_libs::status_text(s).0.as_ptr()
    }

    pub(crate) unsafe fn cublasGetStatusString(s: cublasStatus_t) -> *const c_char {
        agx_libs::status_text(s).1.as_ptr()
    }

    pub(crate) unsafe fn cublasXerbla(name: *const c_char, info: c_int) {
        let name = if name.is_null() { "?".into() } else { std::ffi::CStr::from_ptr(name).to_string_lossy() };
        eprintln!(" ** On entry to {name} parameter number {info} had an illegal value");
    }

    /// alpha or beta (an f32, or an f16 when `half`), from host or device memory as the handle's pointer mode says.
    // a double precision alpha or beta
    unsafe fn scalar64(h: &Handle, p: *const c_void) -> Result<f64, cublasError_t> {
        if p.is_null() {
            return Err(cublasError_t::INVALID_VALUE);
        }
        let mut v = [0u8; 8];
        if h.pointer_mode == cublasPointerMode_t::CUBLAS_POINTER_MODE_DEVICE {
            engine::read(v.as_mut_ptr() as *mut c_void, p, 8).map_err(status)?;
        } else {
            std::ptr::copy_nonoverlapping(p as *const u8, v.as_mut_ptr(), 8);
        }
        Ok(f64::from_le_bytes(v))
    }

    unsafe fn scalar(h: &Handle, p: *const c_void, half: bool) -> Result<f32, cublasError_t> {
        if p.is_null() {
            return Err(cublasError_t::INVALID_VALUE);
        }
        let mut v = [0u8; 4];
        let n = if half { 2 } else { 4 };
        if h.pointer_mode == cublasPointerMode_t::CUBLAS_POINTER_MODE_DEVICE {
            engine::read(v.as_mut_ptr() as *mut c_void, p, n).map_err(status)?;
        } else {
            std::ptr::copy_nonoverlapping(p as *const u8, v.as_mut_ptr(), n);
        }
        Ok(if half { f16_to_f32(u16::from_le_bytes([v[0], v[1]])) } else { f32::from_le_bytes(v) })
    }

    fn op(o: cublasOperation_t) -> Result<bool, cublasError_t> {
        match o {
            cublasOperation_t::CUBLAS_OP_N => Ok(false),
            // conjugation does nothing to real matrices
            cublasOperation_t::CUBLAS_OP_T | cublasOperation_t::CUBLAS_OP_C => Ok(true),
            _ => Err(cublasError_t::INVALID_VALUE),
        }
    }

    #[allow(clippy::too_many_arguments)]
    unsafe fn gemm(
        types: Types,
        half_scalars: bool,
        h: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: c_int,
        n: c_int,
        k: c_int,
        alpha: *const c_void,
        a: *const c_void,
        lda: c_int,
        stride_a: c_longlong,
        b: *const c_void,
        ldb: c_int,
        stride_b: c_longlong,
        beta: *const c_void,
        c: *mut c_void,
        ldc: c_int,
        stride_c: c_longlong,
        batch: c_int,
    ) -> cublasStatus_t {
        let h = handle(h)?;
        let (ta, tb) = (op(transa)?, op(transb)?);
        let rows_a = if ta { k } else { m };
        let rows_b = if tb { n } else { k };
        if m < 0 || n < 0 || k < 0 || batch < 0 || lda < rows_a.max(1) || ldb < rows_b.max(1) || ldc < m.max(1) {
            return Err(cublasError_t::INVALID_VALUE);
        }
        if m == 0 || n == 0 || batch == 0 {
            return Ok(());
        }
        let (alpha, beta) = if types == Types::D {
            (scalar64(h, alpha)?, scalar64(h, beta)?)
        } else {
            (scalar(h, alpha, half_scalars)? as f64, scalar(h, beta, half_scalars)? as f64)
        };
        let g = engine::Gemm {
            types,
            ta,
            tb,
            m: m as u64,
            n: n as u64,
            k: k as u64,
            alpha,
            beta,
            a,
            lda: lda as u64,
            stride_a,
            b,
            ldb: ldb as u64,
            stride_b,
            c,
            ldc: ldc as u64,
            stride_c,
            batch: batch as u64,
        };
        engine::gemm(&g, h.stream).map_err(status)
    }

    pub(crate) unsafe fn cublasSgemm_v2(
        h: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: c_int,
        n: c_int,
        k: c_int,
        alpha: *const f32,
        a: *const f32,
        lda: c_int,
        b: *const f32,
        ldb: c_int,
        beta: *const f32,
        c: *mut f32,
        ldc: c_int,
    ) -> cublasStatus_t {
        gemm(Types::S, false, h, transa, transb, m, n, k, alpha.cast(), a.cast(), lda, 0, b.cast(), ldb, 0, beta.cast(), c.cast(), ldc, 0, 1)
    }

    pub(crate) unsafe fn cublasDgemm_v2(
        h: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: c_int,
        n: c_int,
        k: c_int,
        alpha: *const f64,
        a: *const f64,
        lda: c_int,
        b: *const f64,
        ldb: c_int,
        beta: *const f64,
        c: *mut f64,
        ldc: c_int,
    ) -> cublasStatus_t {
        gemm(Types::D, false, h, transa, transb, m, n, k, alpha.cast(), a.cast(), lda, 0, b.cast(), ldb, 0, beta.cast(), c.cast(), ldc, 0, 1)
    }

    pub(crate) unsafe fn cublasDgemmStridedBatched(
        h: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: c_int,
        n: c_int,
        k: c_int,
        alpha: *const f64,
        a: *const f64,
        lda: c_int,
        stride_a: c_longlong,
        b: *const f64,
        ldb: c_int,
        stride_b: c_longlong,
        beta: *const f64,
        c: *mut f64,
        ldc: c_int,
        stride_c: c_longlong,
        batch: c_int,
    ) -> cublasStatus_t {
        gemm(
            Types::D,
            false,
            h,
            transa,
            transb,
            m,
            n,
            k,
            alpha.cast(),
            a.cast(),
            lda,
            stride_a,
            b.cast(),
            ldb,
            stride_b,
            beta.cast(),
            c.cast(),
            ldc,
            stride_c,
            batch,
        )
    }

    pub(crate) unsafe fn cublasSgemmStridedBatched(
        h: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: c_int,
        n: c_int,
        k: c_int,
        alpha: *const f32,
        a: *const f32,
        lda: c_int,
        stride_a: c_longlong,
        b: *const f32,
        ldb: c_int,
        stride_b: c_longlong,
        beta: *const f32,
        c: *mut f32,
        ldc: c_int,
        stride_c: c_longlong,
        batch: c_int,
    ) -> cublasStatus_t {
        gemm(
            Types::S,
            false,
            h,
            transa,
            transb,
            m,
            n,
            k,
            alpha.cast(),
            a.cast(),
            lda,
            stride_a,
            b.cast(),
            ldb,
            stride_b,
            beta.cast(),
            c.cast(),
            ldc,
            stride_c,
            batch,
        )
    }

    pub(crate) unsafe fn cublasHgemm(
        h: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: c_int,
        n: c_int,
        k: c_int,
        alpha: *const __half,
        a: *const __half,
        lda: c_int,
        b: *const __half,
        ldb: c_int,
        beta: *const __half,
        c: *mut __half,
        ldc: c_int,
    ) -> cublasStatus_t {
        gemm(Types::H, true, h, transa, transb, m, n, k, alpha.cast(), a.cast(), lda, 0, b.cast(), ldb, 0, beta.cast(), c.cast(), ldc, 0, 1)
    }

    pub(crate) unsafe fn cublasHgemmStridedBatched(
        h: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: c_int,
        n: c_int,
        k: c_int,
        alpha: *const __half,
        a: *const __half,
        lda: c_int,
        stride_a: c_longlong,
        b: *const __half,
        ldb: c_int,
        stride_b: c_longlong,
        beta: *const __half,
        c: *mut __half,
        ldc: c_int,
        stride_c: c_longlong,
        batch: c_int,
    ) -> cublasStatus_t {
        gemm(
            Types::H,
            true,
            h,
            transa,
            transb,
            m,
            n,
            k,
            alpha.cast(),
            a.cast(),
            lda,
            stride_a,
            b.cast(),
            ldb,
            stride_b,
            beta.cast(),
            c.cast(),
            ldc,
            stride_c,
            batch,
        )
    }

    /// The kernel types for GemmEx matrix types, and whether alpha and beta are f16 (a 16-bit computation, which runs
    /// in f32 here). f32 A, B and C, or f16 A and B with f16 or f32 C; other types are not written yet.
    fn ex_types(types: [cudaDataType; 3], compute: cublasComputeType_t) -> Result<(Types, bool), cublasError_t> {
        use cublasComputeType_t as K;
        let (f32_, f16_) = (cudaDataType::CUDA_R_32F, cudaDataType::CUDA_R_16F);
        let t = match types {
            [a, b, c] if a == f32_ && b == f32_ && c == f32_ => Types::S,
            [a, b, c] if a == f16_ && b == f16_ && c == f16_ => Types::H,
            [a, b, c] if a == f16_ && b == f16_ && c == f32_ => Types::HtoS,
            [a, b, c] if [a, b, c].iter().all(|t| *t == cudaDataType::CUDA_R_64F) => Types::D,
            _ => return Err(cublasError_t::NOT_SUPPORTED),
        };
        match compute {
            K::CUBLAS_COMPUTE_64F | K::CUBLAS_COMPUTE_64F_PEDANTIC if t == Types::D => Ok((t, false)),
            _ if t == Types::D => Err(cublasError_t::NOT_SUPPORTED),
            K::CUBLAS_COMPUTE_32F | K::CUBLAS_COMPUTE_32F_PEDANTIC | K::CUBLAS_COMPUTE_32F_FAST_TF32 => Ok((t, false)),
            K::CUBLAS_COMPUTE_32F_FAST_16F | K::CUBLAS_COMPUTE_32F_FAST_16BF => Ok((t, false)),
            K::CUBLAS_COMPUTE_16F | K::CUBLAS_COMPUTE_16F_PEDANTIC if t == Types::H => Ok((t, true)),
            _ => Err(cublasError_t::NOT_SUPPORTED),
        }
    }

    pub(crate) unsafe fn cublasGemmEx(
        h: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: c_int,
        n: c_int,
        k: c_int,
        alpha: *const c_void,
        a: *const c_void,
        a_type: cudaDataType,
        lda: c_int,
        b: *const c_void,
        b_type: cudaDataType,
        ldb: c_int,
        beta: *const c_void,
        c: *mut c_void,
        c_type: cudaDataType,
        ldc: c_int,
        compute: cublasComputeType_t,
        _algo: cublasGemmAlgo_t,
    ) -> cublasStatus_t {
        let (t, half) = ex_types([a_type, b_type, c_type], compute)?;
        gemm(t, half, h, transa, transb, m, n, k, alpha, a, lda, 0, b, ldb, 0, beta, c, ldc, 0, 1)
    }

    pub(crate) unsafe fn cublasGemmStridedBatchedEx(
        h: cublasHandle_t,
        transa: cublasOperation_t,
        transb: cublasOperation_t,
        m: c_int,
        n: c_int,
        k: c_int,
        alpha: *const c_void,
        a: *const c_void,
        a_type: cudaDataType,
        lda: c_int,
        stride_a: c_longlong,
        b: *const c_void,
        b_type: cudaDataType,
        ldb: c_int,
        stride_b: c_longlong,
        beta: *const c_void,
        c: *mut c_void,
        c_type: cudaDataType,
        ldc: c_int,
        stride_c: c_longlong,
        batch: c_int,
        compute: cublasComputeType_t,
        _algo: cublasGemmAlgo_t,
    ) -> cublasStatus_t {
        let (t, half) = ex_types([a_type, b_type, c_type], compute)?;
        gemm(t, half, h, transa, transb, m, n, k, alpha, a, lda, stride_a, b, ldb, stride_b, beta, c, ldc, stride_c, batch)
    }
    // ---- level 1 and 2 (agx_libs/kernels/blas.mlir)

    pub(crate) mod level1 {
        /// Threads of the partial reductions.
        pub const THREADS: i64 = 1024;
        /// The partials (two floats of up to 8 bytes per thread) and the result word.
        pub const WORKSPACE: usize = 2 * 8 * THREADS as usize + 16;
        pub const AXPY: i64 = 0;
        pub const SCAL: i64 = 1;
        pub const COPY: i64 = 2;
        pub const SWAP: i64 = 3;
        pub const DOT: i64 = 0;
        pub const ASUM: i64 = 1;
        pub const NRM2: i64 = 2;
        pub const AMAX: i64 = 3;
        pub const AMIN: i64 = 4;
    }

    fn arg<T>(v: &mut T) -> *mut c_void {
        v as *mut T as *mut c_void
    }

    unsafe fn blas_launch(h: &Handle, name: &'static std::ffi::CStr, params: &mut [*mut c_void], grid: [i64; 2], block: [u32; 2]) -> cublasStatus_t {
        let groups = |n: i64, b: u32| u32::try_from((n.max(1) as u64).div_ceil(b as u64)).map_err(|_| cublasError_t::NOT_SUPPORTED);
        let grid = [groups(grid[0], block[0])?, groups(grid[1], block[1])?];
        let f = engine::function(agx_libs::BLAS, name).map_err(status)?;
        engine::launch_kernel(f, grid, block, params, h.stream).map_err(status)
    }

    /// The element types of the S and D routines: the D ones run the f64 copies of the f32 kernels
    /// (`<kernel>_f64`, generated by agx_libs/build.rs).
    trait Real: Copy + 'static {
        const BYTES: usize;
        const GEMM: Types;
        /// alpha or beta, from host or device memory as the handle's pointer mode says.
        unsafe fn scalar(h: &Handle, p: *const c_void) -> Result<Self, cublasError_t>;
        fn to_f64(self) -> f64;
        fn scalar_zero() -> Self;
        /// The f32 kernel, or its f64 copy.
        fn kernel(f32_name: &'static CStr, f64_name: &'static CStr) -> &'static CStr;
    }
    impl Real for f32 {
        const BYTES: usize = 4;
        const GEMM: Types = Types::S;
        unsafe fn scalar(h: &Handle, p: *const c_void) -> Result<f32, cublasError_t> {
            scalar(h, p, false)
        }
        fn to_f64(self) -> f64 {
            self as f64
        }
        fn scalar_zero() -> f32 {
            0.0
        }
        fn kernel(f32_name: &'static CStr, _: &'static CStr) -> &'static CStr {
            f32_name
        }
    }
    impl Real for f64 {
        const BYTES: usize = 8;
        const GEMM: Types = Types::D;
        unsafe fn scalar(h: &Handle, p: *const c_void) -> Result<f64, cublasError_t> {
            scalar64(h, p)
        }
        fn to_f64(self) -> f64 {
            self
        }
        fn scalar_zero() -> f64 {
            0.0
        }
        fn kernel(_: &'static CStr, f64_name: &'static CStr) -> &'static CStr {
            f64_name
        }
    }

    unsafe fn vec_op<T: Real>(h: cublasHandle_t, op: i64, mut n: i64, alpha: *const T, mut x: *const T, mut incx: i64, mut y: *const T, mut incy: i64) -> cublasStatus_t {
        let h = handle(h)?;
        let mut alpha = if alpha.is_null() { T::scalar_zero() } else { T::scalar(h, alpha as *const c_void)? };
        if n <= 0 || (op == level1::AXPY && alpha.to_f64() == 0.0) {
            return Ok(());
        }
        if x.is_null() || (op != level1::SCAL && y.is_null()) {
            return Err(cublasError_t::INVALID_VALUE);
        }
        if op == level1::SCAL {
            // cuBLAS ignores a non-positive increment for scal
            if incx <= 0 {
                return Ok(());
            }
            y = x;
            incy = incx;
        }
        let mut op = op;
        blas_launch(
            h,
            T::kernel(c"agx_blas_vec", c"agx_blas_vec_f64"),
            &mut [arg(&mut x), arg(&mut y), arg(&mut n), arg(&mut incx), arg(&mut incy), arg(&mut op), arg(&mut alpha)],
            [n, 1],
            [64, 1],
        )
    }

    /// Runs a reduction; the result is a float of the element type (or a 1-based index) in device memory at
    /// `result` in device pointer mode, else copied to the host. The f32 kernels return an i32 index, the f64 ones
    /// an i64.
    unsafe fn reduce<T: Real>(h: cublasHandle_t, op: i64, n: i64, mut x: *const T, incx: i64, mut y: *const T, incy: i64, result: *mut c_void, wide_index: bool) -> cublasStatus_t {
        let h = handle(h)?;
        if result.is_null() {
            return Err(cublasError_t::INVALID_VALUE);
        }
        // BLAS: no elements for n <= 0, and for a non-positive increment except in dot
        let mut n = if n <= 0 || (op != level1::DOT && incx <= 0) { 0 } else { n };
        if n > 0 && (x.is_null() || (op == level1::DOT && y.is_null())) {
            return Err(cublasError_t::INVALID_VALUE);
        }
        if op != level1::DOT {
            y = x;
        }
        let index = op == level1::AMAX || op == level1::AMIN;
        let device = h.pointer_mode == cublasPointerMode_t::CUBLAS_POINTER_MODE_DEVICE;
        if h.ws.is_null() {
            h.ws = engine::alloc(level1::WORKSPACE).map_err(status)?;
        }
        let mut part = h.ws;
        let slot = (h.ws as *mut u8).add(2 * 8 * level1::THREADS as usize) as *mut c_void;
        // the kernel writes the result in place when it has the width the caller wants
        let direct = device && (!index || (!wide_index && T::BYTES == 4) || (wide_index && T::BYTES == 8));
        let mut out = if direct { result } else { slot };
        let mut threads = n.clamp(1, level1::THREADS);
        let (mut op, mut incx, mut incy) = (op, incx, incy);
        blas_launch(
            h,
            T::kernel(c"agx_blas_reduce", c"agx_blas_reduce_f64"),
            &mut [arg(&mut x), arg(&mut y), arg(&mut part), arg(&mut n), arg(&mut incx), arg(&mut incy), arg(&mut op), arg(&mut threads)],
            [threads, 1],
            [64, 1],
        )?;
        blas_launch(
            h,
            T::kernel(c"agx_blas_reduce_final", c"agx_blas_reduce_final_f64"),
            &mut [arg(&mut part), arg(&mut out), arg(&mut threads), arg(&mut op)],
            [1, 1],
            [32, 1],
        )?;
        if direct {
            return Ok(());
        }
        engine::synchronize(h.stream).map_err(status)?;
        let mut v = [0u8; 8];
        engine::read(v.as_mut_ptr() as *mut c_void, slot, T::BYTES).map_err(status)?;
        if !index {
            // a float of the element type, to the host
            std::ptr::copy_nonoverlapping(v.as_ptr(), result as *mut u8, T::BYTES);
            return Ok(());
        }
        let i = if T::BYTES == 4 { i32::from_le_bytes(v[..4].try_into().unwrap()) as i64 } else { i64::from_le_bytes(v) };
        match (wide_index, device) {
            (false, false) => *(result as *mut i32) = i as i32,
            (true, false) => *(result as *mut i64) = i,
            (false, true) => {
                let mut w = i as i32;
                engine::write(result, arg(&mut w), 4).map_err(status)?;
            }
            (true, true) => {
                let mut w = i;
                engine::write(result, arg(&mut w), 8).map_err(status)?;
            }
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    unsafe fn gemv<T: Real>(h: cublasHandle_t, trans: cublasOperation_t, mut m: i64, mut n: i64, alpha: *const T, mut a: *const T, mut lda: i64, mut x: *const T, mut incx: i64, beta: *const T, mut y: *mut T, mut incy: i64) -> cublasStatus_t {
        let h = handle(h)?;
        let t = op(trans)?;
        if m < 0 || n < 0 || lda < m.max(1) || incx == 0 || incy == 0 {
            return Err(cublasError_t::INVALID_VALUE);
        }
        let mut alpha = T::scalar(h, alpha as *const c_void)?;
        let mut beta = T::scalar(h, beta as *const c_void)?;
        if m == 0 || n == 0 || (alpha.to_f64() == 0.0 && beta.to_f64() == 1.0) {
            return Ok(());
        }
        if a.is_null() || x.is_null() || y.is_null() {
            return Err(cublasError_t::INVALID_VALUE);
        }
        let mut trans = t as i64;
        let ylen = if t { n } else { m };
        blas_launch(
            h,
            T::kernel(c"agx_sgemv", c"agx_sgemv_f64"),
            &mut [
                arg(&mut a),
                arg(&mut x),
                arg(&mut y),
                arg(&mut m),
                arg(&mut n),
                arg(&mut lda),
                arg(&mut incx),
                arg(&mut incy),
                arg(&mut trans),
                arg(&mut alpha),
                arg(&mut beta),
            ],
            [ylen, 1],
            [64, 1],
        )
    }

    #[allow(clippy::too_many_arguments)]
    unsafe fn ger<T: Real>(h: cublasHandle_t, mut m: i64, mut n: i64, alpha: *const T, mut x: *const T, mut incx: i64, mut y: *const T, mut incy: i64, mut a: *mut T, mut lda: i64) -> cublasStatus_t {
        let h = handle(h)?;
        if m < 0 || n < 0 || lda < m.max(1) || incx == 0 || incy == 0 {
            return Err(cublasError_t::INVALID_VALUE);
        }
        let mut alpha = T::scalar(h, alpha as *const c_void)?;
        if m == 0 || n == 0 || alpha.to_f64() == 0.0 {
            return Ok(());
        }
        if a.is_null() || x.is_null() || y.is_null() {
            return Err(cublasError_t::INVALID_VALUE);
        }
        blas_launch(
            h,
            T::kernel(c"agx_sger", c"agx_sger_f64"),
            &mut [arg(&mut a), arg(&mut x), arg(&mut y), arg(&mut m), arg(&mut n), arg(&mut lda), arg(&mut incx), arg(&mut incy), arg(&mut alpha)],
            [m, n],
            [16, 4],
        )
    }

    /// Widens the integer arguments of the 32-bit entry points to the i64 of the `_64` ones.
    trait Wide {
        type W;
        fn wide(self) -> Self::W;
    }
    macro_rules! wide {
        ($($t:ty => $w:ty),*) => { $(impl Wide for $t { type W = $w; fn wide(self) -> $w { self as $w } })* };
    }
    wide!(c_int => i64, i64 => i64, *const f32 => *const f32, *mut f32 => *mut f32, *const f64 => *const f64, *mut f64 => *mut f64);
    impl Wide for cublasHandle_t {
        type W = cublasHandle_t;
        fn wide(self) -> cublasHandle_t {
            self
        }
    }
    impl Wide for cublasOperation_t {
        type W = cublasOperation_t;
        fn wide(self) -> cublasOperation_t {
            self
        }
    }

    /// The 32-bit (`I` = c_int) and `_64` (`I` = i64) entry points of the level 1 and 2 routines, for each element
    /// type (`R` = f32 for the S routines, f64 for the D ones).
    macro_rules! level12 {
        ($mod32:ident, $mod64:ident, $r:ty: $($name:ident, $name64:ident: ($($a:ident: $t:ty),*) => $body:expr;)*) => {
            mod $mod32 {
                use super::*;
                type I = c_int;
                #[allow(dead_code)]
                type R = $r;
                $(
                    #[allow(clippy::too_many_arguments)]
                    pub(crate) unsafe fn $name($($a: $t),*) -> cublasStatus_t {
                        $(let $a = $a.wide();)*
                        $body
                    }
                )*
            }
            mod $mod64 {
                use super::*;
                type I = i64;
                #[allow(dead_code)]
                type R = $r;
                $(
                    #[allow(clippy::too_many_arguments)]
                    pub(crate) unsafe fn $name64($($a: $t),*) -> cublasStatus_t {
                        $(let $a = $a.wide();)*
                        $body
                    }
                )*
            }
            pub(crate) use $mod32::{$($name),*};
            pub(crate) use $mod64::{$($name64),*};
        };
    }

    macro_rules! level12_for {
        ($mod32:ident, $mod64:ident, $r:ty, $axpy:ident, $axpy64:ident, $scal:ident, $scal64:ident, $copy:ident, $copy64:ident, $swap:ident, $swap64:ident, $dot:ident, $dot64:ident, $asum:ident, $asum64:ident, $nrm2:ident, $nrm264:ident, $gemv:ident, $gemv64:ident, $ger:ident, $ger64:ident) => {
            level12! { $mod32, $mod64, $r:
                $axpy, $axpy64: (h: cublasHandle_t, n: I, alpha: *const R, x: *const R, incx: I, y: *mut R, incy: I)
                    => vec_op::<R>(h, level1::AXPY, n, alpha, x, incx, y, incy);
                $scal, $scal64: (h: cublasHandle_t, n: I, alpha: *const R, x: *mut R, incx: I)
                    => vec_op::<R>(h, level1::SCAL, n, alpha, x, incx, std::ptr::null(), 0);
                $copy, $copy64: (h: cublasHandle_t, n: I, x: *const R, incx: I, y: *mut R, incy: I)
                    => vec_op::<R>(h, level1::COPY, n, std::ptr::null(), x, incx, y, incy);
                $swap, $swap64: (h: cublasHandle_t, n: I, x: *mut R, incx: I, y: *mut R, incy: I)
                    => vec_op::<R>(h, level1::SWAP, n, std::ptr::null(), x, incx, y, incy);
                $dot, $dot64: (h: cublasHandle_t, n: I, x: *const R, incx: I, y: *const R, incy: I, result: *mut R)
                    => reduce::<R>(h, level1::DOT, n, x, incx, y, incy, result as *mut c_void, false);
                $asum, $asum64: (h: cublasHandle_t, n: I, x: *const R, incx: I, result: *mut R)
                    => reduce::<R>(h, level1::ASUM, n, x, incx, std::ptr::null(), 0, result as *mut c_void, false);
                $nrm2, $nrm264: (h: cublasHandle_t, n: I, x: *const R, incx: I, result: *mut R)
                    => reduce::<R>(h, level1::NRM2, n, x, incx, std::ptr::null(), 0, result as *mut c_void, false);
                $gemv, $gemv64: (h: cublasHandle_t, trans: cublasOperation_t, m: I, n: I, alpha: *const R, a: *const R, lda: I, x: *const R, incx: I, beta: *const R, y: *mut R, incy: I)
                    => gemv::<R>(h, trans, m, n, alpha, a, lda, x, incx, beta, y, incy);
                $ger, $ger64: (h: cublasHandle_t, m: I, n: I, alpha: *const R, x: *const R, incx: I, y: *const R, incy: I, a: *mut R, lda: I)
                    => ger::<R>(h, m, n, alpha, x, incx, y, incy, a, lda);
            }
        };
    }

    level12_for!(s32, s64, f32, cublasSaxpy_v2, cublasSaxpy_v2_64, cublasSscal_v2, cublasSscal_v2_64, cublasScopy_v2, cublasScopy_v2_64, cublasSswap_v2, cublasSswap_v2_64, cublasSdot_v2, cublasSdot_v2_64, cublasSasum_v2, cublasSasum_v2_64, cublasSnrm2_v2, cublasSnrm2_v2_64, cublasSgemv_v2, cublasSgemv_v2_64, cublasSger_v2, cublasSger_v2_64);
    level12_for!(d32, d64, f64, cublasDaxpy_v2, cublasDaxpy_v2_64, cublasDscal_v2, cublasDscal_v2_64, cublasDcopy_v2, cublasDcopy_v2_64, cublasDswap_v2, cublasDswap_v2_64, cublasDdot_v2, cublasDdot_v2_64, cublasDasum_v2, cublasDasum_v2_64, cublasDnrm2_v2, cublasDnrm2_v2_64, cublasDgemv_v2, cublasDgemv_v2_64, cublasDger_v2, cublasDger_v2_64);

    macro_rules! amax_amin {
        ($($name:ident, $name64:ident: $r:ty, $op:expr;)*) => {
            $(
                pub(crate) unsafe fn $name(h: cublasHandle_t, n: c_int, x: *const $r, incx: c_int, result: *mut c_int) -> cublasStatus_t {
                    reduce::<$r>(h, $op, n as i64, x, incx as i64, std::ptr::null(), 0, result as *mut c_void, false)
                }
                pub(crate) unsafe fn $name64(h: cublasHandle_t, n: i64, x: *const $r, incx: i64, result: *mut i64) -> cublasStatus_t {
                    reduce::<$r>(h, $op, n, x, incx, std::ptr::null(), 0, result as *mut c_void, true)
                }
            )*
        };
    }
    amax_amin! {
        cublasIsamax_v2, cublasIsamax_v2_64: f32, level1::AMAX;
        cublasIsamin_v2, cublasIsamin_v2_64: f32, level1::AMIN;
        cublasIdamax_v2, cublasIdamax_v2_64: f64, level1::AMAX;
        cublasIdamin_v2, cublasIdamin_v2_64: f64, level1::AMIN;
    }
    // ---- level 3 beyond GEMM (agx_libs/kernels/blas.mlir)

    fn fill(u: cublasFillMode_t) -> Result<bool, cublasError_t> {
        match u {
            cublasFillMode_t::CUBLAS_FILL_MODE_LOWER => Ok(false),
            cublasFillMode_t::CUBLAS_FILL_MODE_UPPER => Ok(true),
            _ => Err(cublasError_t::INVALID_VALUE),
        }
    }

    fn side(s: cublasSideMode_t) -> Result<bool, cublasError_t> {
        match s {
            cublasSideMode_t::CUBLAS_SIDE_LEFT => Ok(false),
            cublasSideMode_t::CUBLAS_SIDE_RIGHT => Ok(true),
            _ => Err(cublasError_t::INVALID_VALUE),
        }
    }

    fn unit_diag(d: cublasDiagType_t) -> Result<bool, cublasError_t> {
        match d {
            cublasDiagType_t::CUBLAS_DIAG_NON_UNIT => Ok(false),
            cublasDiagType_t::CUBLAS_DIAG_UNIT => Ok(true),
            _ => Err(cublasError_t::INVALID_VALUE),
        }
    }

    /// A column-major matrix of `rows` rows needs `ld >= max(1, rows)`.
    fn ld_ok(ld: i64, rows: i64) -> Result<(), cublasError_t> {
        if ld < rows.max(1) {
            Err(cublasError_t::INVALID_VALUE)
        } else {
            Ok(())
        }
    }

    #[allow(clippy::too_many_arguments)]
    unsafe fn geam<T: Real>(h: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, mut m: i64, mut n: i64, alpha: *const T, mut a: *const T, mut lda: i64, beta: *const T, mut b: *const T, mut ldb: i64, mut c: *mut T, mut ldc: i64) -> cublasStatus_t {
        let h = handle(h)?;
        let (ta, tb) = (op(transa)?, op(transb)?);
        if m < 0 || n < 0 {
            return Err(cublasError_t::INVALID_VALUE);
        }
        ld_ok(lda, if ta { n } else { m })?;
        ld_ok(ldb, if tb { n } else { m })?;
        ld_ok(ldc, m)?;
        let mut alpha = T::scalar(h, alpha as *const c_void)?;
        let mut beta = T::scalar(h, beta as *const c_void)?;
        if m == 0 || n == 0 {
            return Ok(());
        }
        // in place, C may be A or B only untransposed with the same leading dimension
        if (c as *const T == a && (ta || lda != ldc)) || (c as *const T == b && (tb || ldb != ldc)) {
            return Err(cublasError_t::INVALID_VALUE);
        }
        let (mut ta, mut tb) = (ta as i64, tb as i64);
        blas_launch(
            h,
            T::kernel(c"agx_sgeam", c"agx_sgeam_f64"),
            &mut [arg(&mut a), arg(&mut b), arg(&mut c), arg(&mut m), arg(&mut n), arg(&mut lda), arg(&mut ldb), arg(&mut ldc), arg(&mut ta), arg(&mut tb), arg(&mut alpha), arg(&mut beta)],
            [m, n],
            [16, 4],
        )
    }

    #[allow(clippy::too_many_arguments)]
    unsafe fn dgmm<T: Real>(h: cublasHandle_t, mode: cublasSideMode_t, mut m: i64, mut n: i64, mut a: *const T, mut lda: i64, mut x: *const T, mut incx: i64, mut c: *mut T, mut ldc: i64) -> cublasStatus_t {
        let h = handle(h)?;
        let mut right = side(mode)? as i64;
        if m < 0 || n < 0 {
            return Err(cublasError_t::INVALID_VALUE);
        }
        ld_ok(lda, m)?;
        ld_ok(ldc, m)?;
        if m == 0 || n == 0 {
            return Ok(());
        }
        blas_launch(
            h,
            T::kernel(c"agx_sdgmm", c"agx_sdgmm_f64"),
            &mut [arg(&mut a), arg(&mut x), arg(&mut c), arg(&mut m), arg(&mut n), arg(&mut lda), arg(&mut incx), arg(&mut ldc), arg(&mut right)],
            [m, n],
            [16, 4],
        )
    }

    #[allow(clippy::too_many_arguments)]
    unsafe fn syrk<T: Real>(h: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, mut n: i64, mut k: i64, alpha: *const T, mut a: *const T, mut lda: i64, beta: *const T, mut c: *mut T, mut ldc: i64) -> cublasStatus_t {
        let h = handle(h)?;
        let (upper, t) = (fill(uplo)?, op(trans)?);
        if n < 0 || k < 0 {
            return Err(cublasError_t::INVALID_VALUE);
        }
        ld_ok(lda, if t { k } else { n })?;
        ld_ok(ldc, n)?;
        let mut alpha = T::scalar(h, alpha as *const c_void)?;
        let mut beta = T::scalar(h, beta as *const c_void)?;
        if n == 0 || ((alpha.to_f64() == 0.0 || k == 0) && beta.to_f64() == 1.0) {
            return Ok(());
        }
        let (mut upper, mut t) = (upper as i64, t as i64);
        blas_launch(
            h,
            T::kernel(c"agx_ssyrk", c"agx_ssyrk_f64"),
            &mut [arg(&mut a), arg(&mut c), arg(&mut n), arg(&mut k), arg(&mut lda), arg(&mut ldc), arg(&mut upper), arg(&mut t), arg(&mut alpha), arg(&mut beta)],
            [n, n],
            [16, 4],
        )
    }

    /// symm (`sym`, with beta) and the out-of-place trmm (beta 0).
    #[allow(clippy::too_many_arguments)]
    unsafe fn symm_trmm<T: Real>(h: cublasHandle_t, sd: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, sym: bool, mut m: i64, mut n: i64, alpha: *const T, mut a: *const T, mut lda: i64, mut b: *const T, mut ldb: i64, beta: *const T, mut c: *mut T, mut ldc: i64) -> cublasStatus_t {
        let h = handle(h)?;
        let (right, upper, t, unit) = (side(sd)?, fill(uplo)?, op(trans)?, unit_diag(diag)?);
        if m < 0 || n < 0 {
            return Err(cublasError_t::INVALID_VALUE);
        }
        ld_ok(lda, if right { n } else { m })?;
        ld_ok(ldb, m)?;
        ld_ok(ldc, m)?;
        let mut alpha = T::scalar(h, alpha as *const c_void)?;
        let mut beta = if sym { T::scalar(h, beta as *const c_void)? } else { T::scalar_zero() };
        if m == 0 || n == 0 {
            return Ok(());
        }
        // each element of C reads a row or column of B: in place, read a copy
        let mut copy = std::ptr::null_mut();
        if c as *const T == b {
            let bytes = T::BYTES * ((n - 1) * ldb + m) as usize;
            copy = engine::alloc(bytes).map_err(status)?;
            engine::copy(copy, b as *const c_void, bytes, h.stream).map_err(status)?;
            b = copy as *const T;
        }
        let (mut right, mut upper, mut t, mut unit, mut sym) = (right as i64, upper as i64, (t && !sym) as i64, (unit && !sym) as i64, sym as i64);
        let r = blas_launch(
            h,
            T::kernel(c"agx_ssymm_trmm", c"agx_ssymm_trmm_f64"),
            &mut [
                arg(&mut a),
                arg(&mut b),
                arg(&mut c),
                arg(&mut m),
                arg(&mut n),
                arg(&mut lda),
                arg(&mut ldb),
                arg(&mut ldc),
                arg(&mut right),
                arg(&mut upper),
                arg(&mut t),
                arg(&mut unit),
                arg(&mut sym),
                arg(&mut alpha),
                arg(&mut beta),
            ],
            [m, n],
            [16, 4],
        );
        if !copy.is_null() {
            engine::synchronize(h.stream).map_err(status)?;
            engine::free(copy).map_err(status)?;
        }
        r
    }

    #[allow(clippy::too_many_arguments)]
    unsafe fn trsm<T: Real>(h: cublasHandle_t, sd: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, mut m: i64, mut n: i64, alpha: *const T, mut a: *const T, mut lda: i64, mut b: *mut T, mut ldb: i64) -> cublasStatus_t {
        let h = handle(h)?;
        let (right, upper, t, unit) = (side(sd)?, fill(uplo)?, op(trans)?, unit_diag(diag)?);
        if m < 0 || n < 0 {
            return Err(cublasError_t::INVALID_VALUE);
        }
        ld_ok(lda, if right { n } else { m })?;
        ld_ok(ldb, m)?;
        let mut alpha = T::scalar(h, alpha as *const c_void)?;
        if m == 0 || n == 0 {
            return Ok(());
        }
        // the per-thread system is lower triangular when an even number of the stored lower triangle, the
        // transpose and the right side flip it
        let mut lower = (!upper ^ t ^ right) as i64;
        let (mut right, mut upper, mut t, mut unit) = (right as i64, upper as i64, t as i64, unit as i64);
        let threads = if right == 1 { m } else { n };
        blas_launch(
            h,
            T::kernel(c"agx_strsm", c"agx_strsm_f64"),
            &mut [arg(&mut a), arg(&mut b), arg(&mut m), arg(&mut n), arg(&mut lda), arg(&mut ldb), arg(&mut right), arg(&mut upper), arg(&mut t), arg(&mut unit), arg(&mut lower), arg(&mut alpha)],
            [threads, 1],
            [64, 1],
        )
    }

    /// GEMM over arrays of device pointers to the matrices (read back after the stream is idle).
    #[allow(clippy::too_many_arguments)]
    unsafe fn gemm_batched<T: Real>(h: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: i64, n: i64, k: i64, alpha: *const T, a: *const *const T, lda: i64, b: *const *const T, ldb: i64, beta: *const T, c: *const *mut T, ldc: i64, batch: i64) -> cublasStatus_t {
        let hh = handle(h)?;
        if batch < 0 {
            return Err(cublasError_t::INVALID_VALUE);
        }
        if batch == 0 || m == 0 || n == 0 {
            return Ok(());
        }
        let int = |v: i64| c_int::try_from(v).map_err(|_| cublasError_t::NOT_SUPPORTED);
        let (m, n, k, lda, ldb, ldc) = (int(m)?, int(n)?, int(k)?, int(lda)?, int(ldb)?, int(ldc)?);
        // alpha and beta stay where they are: gemm reads them in the handle's pointer mode
        let fetch = |p: *const c_void| -> Result<Vec<usize>, cublasError_t> {
            if p.is_null() {
                return Err(cublasError_t::INVALID_VALUE);
            }
            let mut v = vec![0usize; batch as usize];
            engine::read(v.as_mut_ptr() as *mut c_void, p, 8 * batch as usize).map_err(status)?;
            Ok(v)
        };
        engine::synchronize(hh.stream).map_err(status)?;
        let (pa, pb, pc) = (fetch(a as *const c_void)?, fetch(b as *const c_void)?, fetch(c as *const c_void)?);
        for i in 0..batch as usize {
            gemm(T::GEMM, false, h, transa, transb, m, n, k, alpha as *const c_void, pa[i] as *const c_void, lda, 0, pb[i] as *const c_void, ldb, 0, beta as *const c_void, pc[i] as *mut c_void, ldc, 0, 1)?;
        }
        Ok(())
    }

    macro_rules! both_widths {
        ($($name:ident, $name64:ident => $f:ident::<$r:ty>($($a:ident: $t:ty $(as $w:ty)?),*);)*) => {
            $(
                #[allow(clippy::too_many_arguments)]
                pub(crate) unsafe fn $name($($a: $t),*) -> cublasStatus_t {
                    $f::<$r>($($a $(as $w)?),*)
                }
                #[allow(clippy::too_many_arguments)]
                pub(crate) unsafe fn $name64($($a: both_widths!(@w $t $(, $w)?)),*) -> cublasStatus_t {
                    $f::<$r>($($a),*)
                }
            )*
        };
        (@w $t:ty, $w:ty) => { $w };
        (@w $t:ty) => { $t };
    }

    macro_rules! level3_for {
        ($r:ty, $geam:ident, $geam64:ident, $dgmm:ident, $dgmm64:ident, $syrk:ident, $syrk64:ident, $trsm:ident, $trsm64:ident, $batched:ident, $batched64:ident, $symm:ident, $symm64:ident, $trmm:ident, $trmm64:ident) => {
            both_widths! {
                $geam, $geam64 => geam::<$r>(h: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: c_int as i64, n: c_int as i64, alpha: *const $r, a: *const $r, lda: c_int as i64, beta: *const $r, b: *const $r, ldb: c_int as i64, c: *mut $r, ldc: c_int as i64);
                $dgmm, $dgmm64 => dgmm::<$r>(h: cublasHandle_t, mode: cublasSideMode_t, m: c_int as i64, n: c_int as i64, a: *const $r, lda: c_int as i64, x: *const $r, incx: c_int as i64, c: *mut $r, ldc: c_int as i64);
                $syrk, $syrk64 => syrk::<$r>(h: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: c_int as i64, k: c_int as i64, alpha: *const $r, a: *const $r, lda: c_int as i64, beta: *const $r, c: *mut $r, ldc: c_int as i64);
                $trsm, $trsm64 => trsm::<$r>(h: cublasHandle_t, sd: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: c_int as i64, n: c_int as i64, alpha: *const $r, a: *const $r, lda: c_int as i64, b: *mut $r, ldb: c_int as i64);
                $batched, $batched64 => gemm_batched::<$r>(h: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: c_int as i64, n: c_int as i64, k: c_int as i64, alpha: *const $r, a: *const *const $r, lda: c_int as i64, b: *const *const $r, ldb: c_int as i64, beta: *const $r, c: *const *mut $r, ldc: c_int as i64, batch: c_int as i64);
            }

            #[allow(clippy::too_many_arguments)]
            pub(crate) unsafe fn $symm(h: cublasHandle_t, sd: cublasSideMode_t, uplo: cublasFillMode_t, m: c_int, n: c_int, alpha: *const $r, a: *const $r, lda: c_int, b: *const $r, ldb: c_int, beta: *const $r, c: *mut $r, ldc: c_int) -> cublasStatus_t {
                symm_trmm::<$r>(h, sd, uplo, cublasOperation_t::CUBLAS_OP_N, cublasDiagType_t::CUBLAS_DIAG_NON_UNIT, true, m as i64, n as i64, alpha, a, lda as i64, b, ldb as i64, beta, c, ldc as i64)
            }

            #[allow(clippy::too_many_arguments)]
            pub(crate) unsafe fn $symm64(h: cublasHandle_t, sd: cublasSideMode_t, uplo: cublasFillMode_t, m: i64, n: i64, alpha: *const $r, a: *const $r, lda: i64, b: *const $r, ldb: i64, beta: *const $r, c: *mut $r, ldc: i64) -> cublasStatus_t {
                symm_trmm::<$r>(h, sd, uplo, cublasOperation_t::CUBLAS_OP_N, cublasDiagType_t::CUBLAS_DIAG_NON_UNIT, true, m, n, alpha, a, lda, b, ldb, beta, c, ldc)
            }

            #[allow(clippy::too_many_arguments)]
            pub(crate) unsafe fn $trmm(h: cublasHandle_t, sd: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: c_int, n: c_int, alpha: *const $r, a: *const $r, lda: c_int, b: *const $r, ldb: c_int, c: *mut $r, ldc: c_int) -> cublasStatus_t {
                symm_trmm::<$r>(h, sd, uplo, trans, diag, false, m as i64, n as i64, alpha, a, lda as i64, b, ldb as i64, std::ptr::null(), c, ldc as i64)
            }

            #[allow(clippy::too_many_arguments)]
            pub(crate) unsafe fn $trmm64(h: cublasHandle_t, sd: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const $r, a: *const $r, lda: i64, b: *const $r, ldb: i64, c: *mut $r, ldc: i64) -> cublasStatus_t {
                symm_trmm::<$r>(h, sd, uplo, trans, diag, false, m, n, alpha, a, lda, b, ldb, std::ptr::null(), c, ldc)
            }
        };
    }

    level3_for!(f32, cublasSgeam, cublasSgeam_64, cublasSdgmm, cublasSdgmm_64, cublasSsyrk_v2, cublasSsyrk_v2_64, cublasStrsm_v2, cublasStrsm_v2_64, cublasSgemmBatched, cublasSgemmBatched_64, cublasSsymm_v2, cublasSsymm_v2_64, cublasStrmm_v2, cublasStrmm_v2_64);
    level3_for!(f64, cublasDgeam, cublasDgeam_64, cublasDdgmm, cublasDdgmm_64, cublasDsyrk_v2, cublasDsyrk_v2_64, cublasDtrsm_v2, cublasDtrsm_v2_64, cublasDgemmBatched, cublasDgemmBatched_64, cublasDsymm_v2, cublasDsymm_v2_64, cublasDtrmm_v2, cublasDtrmm_v2_64);
}
