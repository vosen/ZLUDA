//! cuBLAS on Apple GPUs for zluda_agx (Asahi Linux).
//!
//! The work is done by the `agx_libs` engine: MLIR kernels (`agx_libs/kernels/*.mlir`, written once for every AGX
//! generation) in fat AGX objects that zluda_agx's `cuModuleLoadData` picks from, launched through the driver API of
//! the process's libcuda (zluda_agx), as NVIDIA's cuBLAS does.
//!
//! Supported so far: handles, streams, pointer and math modes, and GEMM in single and half precision (`cublasSgemm`,
//! `cublasHgemm`, their strided batched forms, and `cublasGemmEx` / `cublasGemmStridedBatchedEx` with f32 A, B and C,
//! or f16 A and B and f16 or f32 C), always accumulated in f32; and the single precision level 1 routines (axpy, scal,
//! copy, swap, dot, asum, nrm2, amax, amin), level 2 `cublasSgemv` and `cublasSger`, and level 3 `cublasSgeam`,
//! `Sdgmm`, `Ssyrk`, `Ssymm`, `Strmm`, `Strsm` and `cublasSgemmBatched`, with their `_64` forms.
//! Reductions are deterministic: fixed per-thread partial sums combined in order. Other functions return
//! `CUBLAS_STATUS_NOT_SUPPORTED`.

#![allow(non_snake_case)]

use agx_libs::{engine, f16_to_f32, Types};
use cuda_types::cublas::*;
use cuda_types::cuda::*;
use std::ffi::{c_char, c_int, c_longlong, c_void};

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
        let (alpha, beta) = (scalar(h, alpha, half_scalars)?, scalar(h, beta, half_scalars)?);
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
            _ => return Err(cublasError_t::NOT_SUPPORTED),
        };
        match compute {
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
        /// The partials (two f32 per thread) and the result word.
        pub const WORKSPACE: usize = 8 * THREADS as usize + 16;
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

    unsafe fn vec_op(h: cublasHandle_t, op: i64, mut n: i64, alpha: *const f32, mut x: *const f32, mut incx: i64, mut y: *const f32, mut incy: i64) -> cublasStatus_t {
        let h = handle(h)?;
        let mut alpha = if alpha.is_null() { 0.0 } else { scalar(h, alpha as *const c_void, false)? };
        if n <= 0 || (op == level1::AXPY && alpha == 0.0) {
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
            c"agx_blas_vec",
            &mut [arg(&mut x), arg(&mut y), arg(&mut n), arg(&mut incx), arg(&mut incy), arg(&mut op), arg(&mut alpha)],
            [n, 1],
            [64, 1],
        )
    }

    /// Runs a reduction; the result is an f32 (or a 1-based i32 index) in device memory at `result` in device
    /// pointer mode, else copied to the host.
    unsafe fn reduce(h: cublasHandle_t, op: i64, n: i64, mut x: *const f32, incx: i64, mut y: *const f32, incy: i64, result: *mut c_void, wide_index: bool) -> cublasStatus_t {
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
        let device = h.pointer_mode == cublasPointerMode_t::CUBLAS_POINTER_MODE_DEVICE;
        if h.ws.is_null() {
            h.ws = engine::alloc(level1::WORKSPACE).map_err(status)?;
        }
        let mut part = h.ws;
        let slot = (h.ws as *mut u8).add(8 * level1::THREADS as usize) as *mut c_void;
        let mut out = if device && !wide_index { result } else { slot };
        let mut threads = n.clamp(1, level1::THREADS);
        let (mut op, mut incx, mut incy) = (op, incx, incy);
        blas_launch(
            h,
            c"agx_blas_reduce",
            &mut [arg(&mut x), arg(&mut y), arg(&mut part), arg(&mut n), arg(&mut incx), arg(&mut incy), arg(&mut op), arg(&mut threads)],
            [threads, 1],
            [64, 1],
        )?;
        blas_launch(h, c"agx_blas_reduce_final", &mut [arg(&mut part), arg(&mut out), arg(&mut threads), arg(&mut op)], [1, 1], [32, 1])?;
        if device && !wide_index {
            return Ok(());
        }
        engine::synchronize(h.stream).map_err(status)?;
        let mut v = 0i32;
        engine::read(arg(&mut v), slot, 4).map_err(status)?;
        if !wide_index {
            *(result as *mut i32) = v;
        } else if device {
            let mut w = v as i64;
            engine::write(result, arg(&mut w), 8).map_err(status)?;
        } else {
            *(result as *mut i64) = v as i64;
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    unsafe fn sgemv(h: cublasHandle_t, trans: cublasOperation_t, mut m: i64, mut n: i64, alpha: *const f32, mut a: *const f32, mut lda: i64, mut x: *const f32, mut incx: i64, beta: *const f32, mut y: *mut f32, mut incy: i64) -> cublasStatus_t {
        let h = handle(h)?;
        let t = op(trans)?;
        if m < 0 || n < 0 || lda < m.max(1) || incx == 0 || incy == 0 {
            return Err(cublasError_t::INVALID_VALUE);
        }
        let mut alpha = scalar(h, alpha as *const c_void, false)?;
        let mut beta = scalar(h, beta as *const c_void, false)?;
        if m == 0 || n == 0 || (alpha == 0.0 && beta == 1.0) {
            return Ok(());
        }
        if a.is_null() || x.is_null() || y.is_null() {
            return Err(cublasError_t::INVALID_VALUE);
        }
        let mut trans = t as i64;
        let ylen = if t { n } else { m };
        blas_launch(
            h,
            c"agx_sgemv",
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
    unsafe fn sger(h: cublasHandle_t, mut m: i64, mut n: i64, alpha: *const f32, mut x: *const f32, mut incx: i64, mut y: *const f32, mut incy: i64, mut a: *mut f32, mut lda: i64) -> cublasStatus_t {
        let h = handle(h)?;
        if m < 0 || n < 0 || lda < m.max(1) || incx == 0 || incy == 0 {
            return Err(cublasError_t::INVALID_VALUE);
        }
        let mut alpha = scalar(h, alpha as *const c_void, false)?;
        if m == 0 || n == 0 || alpha == 0.0 {
            return Ok(());
        }
        if a.is_null() || x.is_null() || y.is_null() {
            return Err(cublasError_t::INVALID_VALUE);
        }
        blas_launch(
            h,
            c"agx_sger",
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
    wide!(c_int => i64, i64 => i64, *const f32 => *const f32, *mut f32 => *mut f32);
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

    /// The 32-bit (`I` = c_int) and `_64` (`I` = i64) entry points of the level 1 and 2 routines.
    macro_rules! level12 {
        ($($name:ident, $name64:ident: ($($a:ident: $t:ty),*) => $body:expr;)*) => {
            mod l32 {
                use super::*;
                type I = c_int;
                $(
                    #[allow(clippy::too_many_arguments)]
                    pub(crate) unsafe fn $name($($a: $t),*) -> cublasStatus_t {
                        $(let $a = $a.wide();)*
                        $body
                    }
                )*
            }
            mod l64 {
                use super::*;
                type I = i64;
                $(
                    #[allow(clippy::too_many_arguments)]
                    pub(crate) unsafe fn $name64($($a: $t),*) -> cublasStatus_t {
                        $(let $a = $a.wide();)*
                        $body
                    }
                )*
            }
            pub(crate) use l32::{$($name),*};
            pub(crate) use l64::{$($name64),*};
        };
    }

    level12! {
        cublasSaxpy_v2, cublasSaxpy_v2_64: (h: cublasHandle_t, n: I, alpha: *const f32, x: *const f32, incx: I, y: *mut f32, incy: I)
            => vec_op(h, level1::AXPY, n, alpha, x, incx, y, incy);
        cublasSscal_v2, cublasSscal_v2_64: (h: cublasHandle_t, n: I, alpha: *const f32, x: *mut f32, incx: I)
            => vec_op(h, level1::SCAL, n, alpha, x, incx, std::ptr::null(), 0);
        cublasScopy_v2, cublasScopy_v2_64: (h: cublasHandle_t, n: I, x: *const f32, incx: I, y: *mut f32, incy: I)
            => vec_op(h, level1::COPY, n, std::ptr::null(), x, incx, y, incy);
        cublasSswap_v2, cublasSswap_v2_64: (h: cublasHandle_t, n: I, x: *mut f32, incx: I, y: *mut f32, incy: I)
            => vec_op(h, level1::SWAP, n, std::ptr::null(), x, incx, y, incy);
        cublasSdot_v2, cublasSdot_v2_64: (h: cublasHandle_t, n: I, x: *const f32, incx: I, y: *const f32, incy: I, result: *mut f32)
            => reduce(h, level1::DOT, n, x, incx, y, incy, result as *mut c_void, false);
        cublasSasum_v2, cublasSasum_v2_64: (h: cublasHandle_t, n: I, x: *const f32, incx: I, result: *mut f32)
            => reduce(h, level1::ASUM, n, x, incx, std::ptr::null(), 0, result as *mut c_void, false);
        cublasSnrm2_v2, cublasSnrm2_v2_64: (h: cublasHandle_t, n: I, x: *const f32, incx: I, result: *mut f32)
            => reduce(h, level1::NRM2, n, x, incx, std::ptr::null(), 0, result as *mut c_void, false);
        cublasSgemv_v2, cublasSgemv_v2_64: (h: cublasHandle_t, trans: cublasOperation_t, m: I, n: I, alpha: *const f32, a: *const f32, lda: I, x: *const f32, incx: I, beta: *const f32, y: *mut f32, incy: I)
            => sgemv(h, trans, m, n, alpha, a, lda, x, incx, beta, y, incy);
        cublasSger_v2, cublasSger_v2_64: (h: cublasHandle_t, m: I, n: I, alpha: *const f32, x: *const f32, incx: I, y: *const f32, incy: I, a: *mut f32, lda: I)
            => sger(h, m, n, alpha, x, incx, y, incy, a, lda);
    }

    pub(crate) unsafe fn cublasIsamax_v2(h: cublasHandle_t, n: c_int, x: *const f32, incx: c_int, result: *mut c_int) -> cublasStatus_t {
        reduce(h, level1::AMAX, n as i64, x, incx as i64, std::ptr::null(), 0, result as *mut c_void, false)
    }

    pub(crate) unsafe fn cublasIsamax_v2_64(h: cublasHandle_t, n: i64, x: *const f32, incx: i64, result: *mut i64) -> cublasStatus_t {
        reduce(h, level1::AMAX, n, x, incx, std::ptr::null(), 0, result as *mut c_void, true)
    }

    pub(crate) unsafe fn cublasIsamin_v2(h: cublasHandle_t, n: c_int, x: *const f32, incx: c_int, result: *mut c_int) -> cublasStatus_t {
        reduce(h, level1::AMIN, n as i64, x, incx as i64, std::ptr::null(), 0, result as *mut c_void, false)
    }

    pub(crate) unsafe fn cublasIsamin_v2_64(h: cublasHandle_t, n: i64, x: *const f32, incx: i64, result: *mut i64) -> cublasStatus_t {
        reduce(h, level1::AMIN, n, x, incx, std::ptr::null(), 0, result as *mut c_void, true)
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
    unsafe fn sgeam(h: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, mut m: i64, mut n: i64, alpha: *const f32, mut a: *const f32, mut lda: i64, beta: *const f32, mut b: *const f32, mut ldb: i64, mut c: *mut f32, mut ldc: i64) -> cublasStatus_t {
        let h = handle(h)?;
        let (ta, tb) = (op(transa)?, op(transb)?);
        if m < 0 || n < 0 {
            return Err(cublasError_t::INVALID_VALUE);
        }
        ld_ok(lda, if ta { n } else { m })?;
        ld_ok(ldb, if tb { n } else { m })?;
        ld_ok(ldc, m)?;
        let mut alpha = scalar(h, alpha as *const c_void, false)?;
        let mut beta = scalar(h, beta as *const c_void, false)?;
        if m == 0 || n == 0 {
            return Ok(());
        }
        // in place, C may be A or B only untransposed with the same leading dimension
        if (c as *const f32 == a && (ta || lda != ldc)) || (c as *const f32 == b && (tb || ldb != ldc)) {
            return Err(cublasError_t::INVALID_VALUE);
        }
        let (mut ta, mut tb) = (ta as i64, tb as i64);
        blas_launch(
            h,
            c"agx_sgeam",
            &mut [arg(&mut a), arg(&mut b), arg(&mut c), arg(&mut m), arg(&mut n), arg(&mut lda), arg(&mut ldb), arg(&mut ldc), arg(&mut ta), arg(&mut tb), arg(&mut alpha), arg(&mut beta)],
            [m, n],
            [16, 4],
        )
    }

    #[allow(clippy::too_many_arguments)]
    unsafe fn sdgmm(h: cublasHandle_t, mode: cublasSideMode_t, mut m: i64, mut n: i64, mut a: *const f32, mut lda: i64, mut x: *const f32, mut incx: i64, mut c: *mut f32, mut ldc: i64) -> cublasStatus_t {
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
            c"agx_sdgmm",
            &mut [arg(&mut a), arg(&mut x), arg(&mut c), arg(&mut m), arg(&mut n), arg(&mut lda), arg(&mut incx), arg(&mut ldc), arg(&mut right)],
            [m, n],
            [16, 4],
        )
    }

    #[allow(clippy::too_many_arguments)]
    unsafe fn ssyrk(h: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, mut n: i64, mut k: i64, alpha: *const f32, mut a: *const f32, mut lda: i64, beta: *const f32, mut c: *mut f32, mut ldc: i64) -> cublasStatus_t {
        let h = handle(h)?;
        let (upper, t) = (fill(uplo)?, op(trans)?);
        if n < 0 || k < 0 {
            return Err(cublasError_t::INVALID_VALUE);
        }
        ld_ok(lda, if t { k } else { n })?;
        ld_ok(ldc, n)?;
        let mut alpha = scalar(h, alpha as *const c_void, false)?;
        let mut beta = scalar(h, beta as *const c_void, false)?;
        if n == 0 || ((alpha == 0.0 || k == 0) && beta == 1.0) {
            return Ok(());
        }
        let (mut upper, mut t) = (upper as i64, t as i64);
        blas_launch(
            h,
            c"agx_ssyrk",
            &mut [arg(&mut a), arg(&mut c), arg(&mut n), arg(&mut k), arg(&mut lda), arg(&mut ldc), arg(&mut upper), arg(&mut t), arg(&mut alpha), arg(&mut beta)],
            [n, n],
            [16, 4],
        )
    }

    /// symm (`sym`, with beta) and the out-of-place trmm (beta 0).
    #[allow(clippy::too_many_arguments)]
    unsafe fn symm_trmm(h: cublasHandle_t, sd: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, sym: bool, mut m: i64, mut n: i64, alpha: *const f32, mut a: *const f32, mut lda: i64, mut b: *const f32, mut ldb: i64, beta: *const f32, mut c: *mut f32, mut ldc: i64) -> cublasStatus_t {
        let h = handle(h)?;
        let (right, upper, t, unit) = (side(sd)?, fill(uplo)?, op(trans)?, unit_diag(diag)?);
        if m < 0 || n < 0 {
            return Err(cublasError_t::INVALID_VALUE);
        }
        ld_ok(lda, if right { n } else { m })?;
        ld_ok(ldb, m)?;
        ld_ok(ldc, m)?;
        let mut alpha = scalar(h, alpha as *const c_void, false)?;
        let mut beta = if sym { scalar(h, beta as *const c_void, false)? } else { 0.0 };
        if m == 0 || n == 0 {
            return Ok(());
        }
        // each element of C reads a row or column of B: in place, read a copy
        let mut copy = std::ptr::null_mut();
        if c as *const f32 == b {
            let bytes = 4 * ((n - 1) * ldb + m) as usize;
            copy = engine::alloc(bytes).map_err(status)?;
            engine::copy(copy, b as *const c_void, bytes, h.stream).map_err(status)?;
            b = copy as *const f32;
        }
        let (mut right, mut upper, mut t, mut unit, mut sym) = (right as i64, upper as i64, (t && !sym) as i64, (unit && !sym) as i64, sym as i64);
        let r = blas_launch(
            h,
            c"agx_ssymm_trmm",
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
    unsafe fn strsm(h: cublasHandle_t, sd: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, mut m: i64, mut n: i64, alpha: *const f32, mut a: *const f32, mut lda: i64, mut b: *mut f32, mut ldb: i64) -> cublasStatus_t {
        let h = handle(h)?;
        let (right, upper, t, unit) = (side(sd)?, fill(uplo)?, op(trans)?, unit_diag(diag)?);
        if m < 0 || n < 0 {
            return Err(cublasError_t::INVALID_VALUE);
        }
        ld_ok(lda, if right { n } else { m })?;
        ld_ok(ldb, m)?;
        let mut alpha = scalar(h, alpha as *const c_void, false)?;
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
            c"agx_strsm",
            &mut [arg(&mut a), arg(&mut b), arg(&mut m), arg(&mut n), arg(&mut lda), arg(&mut ldb), arg(&mut right), arg(&mut upper), arg(&mut t), arg(&mut unit), arg(&mut lower), arg(&mut alpha)],
            [threads, 1],
            [64, 1],
        )
    }

    /// GEMM over arrays of device pointers to the matrices (read back after the stream is idle).
    #[allow(clippy::too_many_arguments)]
    unsafe fn sgemm_batched(h: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: i64, n: i64, k: i64, alpha: *const f32, a: *const *const f32, lda: i64, b: *const *const f32, ldb: i64, beta: *const f32, c: *const *mut f32, ldc: i64, batch: i64) -> cublasStatus_t {
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
            gemm(Types::S, false, h, transa, transb, m, n, k, alpha as *const c_void, pa[i] as *const c_void, lda, 0, pb[i] as *const c_void, ldb, 0, beta as *const c_void, pc[i] as *mut c_void, ldc, 0, 1)?;
        }
        Ok(())
    }

    macro_rules! both_widths {
        ($($name:ident, $name64:ident => $f:ident($($a:ident: $t:ty $(as $w:ty)?),*);)*) => {
            $(
                #[allow(clippy::too_many_arguments)]
                pub(crate) unsafe fn $name($($a: $t),*) -> cublasStatus_t {
                    $f($($a $(as $w)?),*)
                }
                #[allow(clippy::too_many_arguments)]
                pub(crate) unsafe fn $name64($($a: both_widths!(@w $t $(, $w)?)),*) -> cublasStatus_t {
                    $f($($a),*)
                }
            )*
        };
        (@w $t:ty, $w:ty) => { $w };
        (@w $t:ty) => { $t };
    }

    both_widths! {
        cublasSgeam, cublasSgeam_64 => sgeam(h: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: c_int as i64, n: c_int as i64, alpha: *const f32, a: *const f32, lda: c_int as i64, beta: *const f32, b: *const f32, ldb: c_int as i64, c: *mut f32, ldc: c_int as i64);
        cublasSdgmm, cublasSdgmm_64 => sdgmm(h: cublasHandle_t, mode: cublasSideMode_t, m: c_int as i64, n: c_int as i64, a: *const f32, lda: c_int as i64, x: *const f32, incx: c_int as i64, c: *mut f32, ldc: c_int as i64);
        cublasSsyrk_v2, cublasSsyrk_v2_64 => ssyrk(h: cublasHandle_t, uplo: cublasFillMode_t, trans: cublasOperation_t, n: c_int as i64, k: c_int as i64, alpha: *const f32, a: *const f32, lda: c_int as i64, beta: *const f32, c: *mut f32, ldc: c_int as i64);
        cublasStrsm_v2, cublasStrsm_v2_64 => strsm(h: cublasHandle_t, sd: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: c_int as i64, n: c_int as i64, alpha: *const f32, a: *const f32, lda: c_int as i64, b: *mut f32, ldb: c_int as i64);
        cublasSgemmBatched, cublasSgemmBatched_64 => sgemm_batched(h: cublasHandle_t, transa: cublasOperation_t, transb: cublasOperation_t, m: c_int as i64, n: c_int as i64, k: c_int as i64, alpha: *const f32, a: *const *const f32, lda: c_int as i64, b: *const *const f32, ldb: c_int as i64, beta: *const f32, c: *const *mut f32, ldc: c_int as i64, batch: c_int as i64);
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) unsafe fn cublasSsymm_v2(h: cublasHandle_t, sd: cublasSideMode_t, uplo: cublasFillMode_t, m: c_int, n: c_int, alpha: *const f32, a: *const f32, lda: c_int, b: *const f32, ldb: c_int, beta: *const f32, c: *mut f32, ldc: c_int) -> cublasStatus_t {
        symm_trmm(h, sd, uplo, cublasOperation_t::CUBLAS_OP_N, cublasDiagType_t::CUBLAS_DIAG_NON_UNIT, true, m as i64, n as i64, alpha, a, lda as i64, b, ldb as i64, beta, c, ldc as i64)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) unsafe fn cublasSsymm_v2_64(h: cublasHandle_t, sd: cublasSideMode_t, uplo: cublasFillMode_t, m: i64, n: i64, alpha: *const f32, a: *const f32, lda: i64, b: *const f32, ldb: i64, beta: *const f32, c: *mut f32, ldc: i64) -> cublasStatus_t {
        symm_trmm(h, sd, uplo, cublasOperation_t::CUBLAS_OP_N, cublasDiagType_t::CUBLAS_DIAG_NON_UNIT, true, m, n, alpha, a, lda, b, ldb, beta, c, ldc)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) unsafe fn cublasStrmm_v2(h: cublasHandle_t, sd: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: c_int, n: c_int, alpha: *const f32, a: *const f32, lda: c_int, b: *const f32, ldb: c_int, c: *mut f32, ldc: c_int) -> cublasStatus_t {
        symm_trmm(h, sd, uplo, trans, diag, false, m as i64, n as i64, alpha, a, lda as i64, b, ldb as i64, std::ptr::null(), c, ldc as i64)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) unsafe fn cublasStrmm_v2_64(h: cublasHandle_t, sd: cublasSideMode_t, uplo: cublasFillMode_t, trans: cublasOperation_t, diag: cublasDiagType_t, m: i64, n: i64, alpha: *const f32, a: *const f32, lda: i64, b: *const f32, ldb: i64, c: *mut f32, ldc: i64) -> cublasStatus_t {
        symm_trmm(h, sd, uplo, trans, diag, false, m, n, alpha, a, lda, b, ldb, std::ptr::null(), c, ldc)
    }
}
