//! cuBLAS on Apple GPUs for zluda_agx (Asahi Linux).
//!
//! The kernels are MLIR (`kernels/*.mlir`: standard dialects and `agx` operations, written once for every AGX
//! generation), compiled at build time for each chip into one fat AGX object that zluda_agx's `cuModuleLoadData` picks
//! from. The library calls the CUDA driver API of the process's libcuda (zluda_agx), like NVIDIA's cuBLAS does.
//!
//! Supported so far: handles, streams, pointer and math modes, and single precision GEMM (`cublasSgemm`, the strided
//! batched form, and `cublasGemmEx` / `cublasGemmStridedBatchedEx` with 32-bit float matrices and compute). Other
//! functions return `CUBLAS_STATUS_NOT_SUPPORTED`.

#![allow(non_snake_case)]

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
            cublasSetMathMode,
            cublasSetPointerMode_v2,
            cublasSetStream_v2,
            cublasSetWorkspace_v2,
            cublasSgemmStridedBatched,
            cublasSgemm_v2,
            cublasXerbla,
        ]
);

/// The fat AGX object of the kernels (build.rs).
static KERNELS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sgemm.agxfat"));

/// How one GEMM runs: which kernel, its launch shape and its arguments after the three matrix pointers.
#[derive(Debug, PartialEq)]
pub enum Plan {
    /// `agx_sgemm_tile64` on row-major matrices: (a, b, c) pointers then M, N, K, lda, ldb, ldc.
    Tile { swap: bool, dims: [u64; 6], groups: u32 },
    /// `agx_sgemm_generic`: (a, b, c) then M, N, K and the (row, column) element strides of op(A), op(B) and C.
    Generic { dims: [u64; 9], grid: [u32; 2] },
}

pub const GENERIC_BLOCK: [u32; 2] = [16, 8];

/// The kernel for column-major `C (m x n, ldc) = alpha op(A) op(B) + beta C`. Seen row-major, column-major C is C^T
/// = op(B)^T op(A)^T, so without transposes the row-major tile kernel computes it with A and B swapped (`swap`).
pub fn plan(ta: bool, tb: bool, m: u64, n: u64, k: u64, lda: u64, ldb: u64, ldc: u64, aligned: bool) -> Plan {
    if !ta && !tb && m % 64 == 0 && n % 64 == 0 && k % 8 == 0 && [lda, ldb, ldc].iter().all(|l| l % 2 == 0) && aligned
    {
        // row-major M' = n, N' = m: A' = B (n x k, ld ldb), B' = A (k x m, ld lda)
        return Plan::Tile { swap: true, dims: [n, m, k, ldb, lda, ldc], groups: ((n / 64) * (m / 64)) as u32 };
    }
    // element (i, k) of op(A): A[i + k lda], or A[k + i lda] transposed; (k, j) of op(B) likewise; C[i + j ldc]
    let sa = if ta { [lda, 1] } else { [1, lda] };
    let sb = if tb { [ldb, 1] } else { [1, ldb] };
    Plan::Generic {
        dims: [m, n, k, sa[0], sa[1], sb[0], sb[1], 1, ldc],
        grid: [n.div_ceil(GENERIC_BLOCK[0] as u64) as u32, m.div_ceil(GENERIC_BLOCK[1] as u64) as u32],
    }
}

mod imp {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    type ModuleLoadData = unsafe extern "system" fn(*mut CUmodule, *const c_void) -> CUresult;
    type ModuleGetFunction = unsafe extern "system" fn(*mut CUfunction, CUmodule, *const c_char) -> CUresult;
    type LaunchKernel = unsafe extern "system" fn(
        CUfunction,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        CUstream,
        *mut *mut c_void,
        *mut *mut c_void,
    ) -> CUresult;
    type MemcpyDtoH = unsafe extern "system" fn(*mut c_void, CUdeviceptr, usize) -> CUresult;
    type CtxGetCurrent = unsafe extern "system" fn(*mut CUcontext) -> CUresult;

    /// The driver API of the process's libcuda (zluda_agx).
    struct Driver {
        _lib: libloading::Library,
        load: ModuleLoadData,
        function: ModuleGetFunction,
        launch: LaunchKernel,
        dtoh: MemcpyDtoH,
        current: CtxGetCurrent,
    }

    fn driver() -> Result<&'static Driver, cublasError_t> {
        static D: OnceLock<Option<Driver>> = OnceLock::new();
        D.get_or_init(|| unsafe {
            let name = std::env::var_os("ZLUDA_AGX_LIBCUDA").unwrap_or_else(|| "libcuda.so.1".into());
            let lib = libloading::Library::new(name).ok()?;
            Some(Driver {
                load: *lib.get(b"cuModuleLoadData\0").ok()?,
                function: *lib.get(b"cuModuleGetFunction\0").ok()?,
                launch: *lib.get(b"cuLaunchKernel\0").ok()?,
                dtoh: *lib.get(b"cuMemcpyDtoH_v2\0").ok()?,
                current: *lib.get(b"cuCtxGetCurrent\0").ok()?,
                _lib: lib,
            })
        })
        .as_ref()
        .ok_or(cublasError_t::NOT_INITIALIZED)
    }

    /// The kernels, loaded once per context.
    #[derive(Clone, Copy)]
    struct Kernels {
        ctx: usize,
        tile: CUfunction,
        generic: CUfunction,
    }
    unsafe impl Send for Kernels {}

    struct Handle {
        stream: CUstream,
        pointer_mode: cublasPointerMode_t,
        math_mode: cublasMath_t,
        kernels: Option<Kernels>,
    }

    static LOADED: Mutex<Vec<Kernels>> = Mutex::new(Vec::new());

    unsafe fn kernels(d: &Driver) -> Result<Kernels, cublasError_t> {
        let mut ctx = CUcontext(std::ptr::null_mut());
        (d.current)(&mut ctx).map_err(|_| cublasError_t::NOT_INITIALIZED)?;
        let key = ctx.0 as usize;
        let mut loaded = LOADED.lock().unwrap();
        if let Some(k) = loaded.iter().find(|k| k.ctx == key) {
            return Ok(*k);
        }
        let mut m = CUmodule(std::ptr::null_mut());
        (d.load)(&mut m, KERNELS.as_ptr() as *const c_void).map_err(|_| cublasError_t::ARCH_MISMATCH)?;
        let get = |name: &[u8]| {
            let mut f = CUfunction(std::ptr::null_mut());
            (d.function)(&mut f, m, name.as_ptr() as *const c_char).map(|_| f).map_err(|_| cublasError_t::INTERNAL_ERROR)
        };
        let k = Kernels { ctx: key, tile: get(b"agx_sgemm_tile64\0")?, generic: get(b"agx_sgemm_generic\0")? };
        loaded.push(k);
        Ok(k)
    }

    unsafe fn handle<'a>(h: cublasHandle_t) -> Result<&'a mut Handle, cublasError_t> {
        (h.0 as *mut Handle).as_mut().ok_or(cublasError_t::NOT_INITIALIZED)
    }

    pub(crate) unsafe fn cublasCreate_v2(out: *mut cublasHandle_t) -> cublasStatus_t {
        driver()?;
        let out = out.as_mut().ok_or(cublasError_t::INVALID_VALUE)?;
        let h = Box::new(Handle {
            stream: CUstream(std::ptr::null_mut()),
            pointer_mode: cublasPointerMode_t::CUBLAS_POINTER_MODE_HOST,
            math_mode: cublasMath_t::CUBLAS_DEFAULT_MATH,
            kernels: None,
        });
        *out = cublasHandle_t(Box::into_raw(h) as *mut cublasContext);
        Ok(())
    }

    pub(crate) unsafe fn cublasDestroy_v2(h: cublasHandle_t) -> cublasStatus_t {
        if h.0.is_null() {
            return Err(cublasError_t::NOT_INITIALIZED);
        }
        drop(Box::from_raw(h.0 as *mut Handle));
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
        status_text(s).0.as_ptr()
    }

    pub(crate) unsafe fn cublasGetStatusString(s: cublasStatus_t) -> *const c_char {
        status_text(s).1.as_ptr()
    }

    fn status_text(s: cublasStatus_t) -> (&'static std::ffi::CStr, &'static std::ffi::CStr) {
        match s {
            Ok(()) => (c"CUBLAS_STATUS_SUCCESS", c"success"),
            Err(cublasError_t::NOT_INITIALIZED) => (c"CUBLAS_STATUS_NOT_INITIALIZED", c"the library was not initialized"),
            Err(cublasError_t::ALLOC_FAILED) => (c"CUBLAS_STATUS_ALLOC_FAILED", c"the resource allocation failed"),
            Err(cublasError_t::INVALID_VALUE) => (c"CUBLAS_STATUS_INVALID_VALUE", c"an invalid numeric value was used as an argument"),
            Err(cublasError_t::ARCH_MISMATCH) => (c"CUBLAS_STATUS_ARCH_MISMATCH", c"an absent device architectural feature is required"),
            Err(cublasError_t::EXECUTION_FAILED) => (c"CUBLAS_STATUS_EXECUTION_FAILED", c"the GPU program failed to execute"),
            Err(cublasError_t::NOT_SUPPORTED) => (c"CUBLAS_STATUS_NOT_SUPPORTED", c"the functionality requested is not supported"),
            Err(_) => (c"CUBLAS_STATUS_INTERNAL_ERROR", c"an internal operation failed"),
        }
    }

    pub(crate) unsafe fn cublasXerbla(name: *const c_char, info: c_int) {
        let name = if name.is_null() { "?".into() } else { std::ffi::CStr::from_ptr(name).to_string_lossy() };
        eprintln!(" ** On entry to {name} parameter number {info} had an illegal value");
    }

    /// alpha or beta, from host or device memory as the handle's pointer mode says.
    unsafe fn scalar(d: &Driver, h: &Handle, p: *const f32) -> Result<f32, cublasError_t> {
        if p.is_null() {
            return Err(cublasError_t::INVALID_VALUE);
        }
        if h.pointer_mode == cublasPointerMode_t::CUBLAS_POINTER_MODE_DEVICE {
            let mut v = 0f32;
            (d.dtoh)(&mut v as *mut f32 as *mut c_void, CUdeviceptr_v2(p as *mut c_void), 4)
                .map_err(|_| cublasError_t::EXECUTION_FAILED)?;
            Ok(v)
        } else {
            Ok(*p)
        }
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
    unsafe fn sgemm(
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
        let d = driver()?;
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
        let (alpha, beta) = (scalar(d, h, alpha)?, scalar(d, h, beta)?);
        let kernels = match h.kernels {
            Some(k) => k,
            None => *h.kernels.insert(kernels(d)?),
        };
        let aligned = [a as usize, b as usize, c as usize].iter().all(|p| p % 8 == 0)
            && [stride_a, stride_b, stride_c].iter().all(|s| s % 2 == 0);
        let p = plan(ta, tb, m as u64, n as u64, k as u64, lda as u64, ldb as u64, ldc as u64, aligned);
        for i in 0..batch as i64 {
            let (mut pa, mut pb, mut pc) =
                (a.offset((i * stride_a) as isize), b.offset((i * stride_b) as isize), c.offset((i * stride_c) as isize));
            let (f, dims, grid, block): (CUfunction, &[u64], [u32; 2], [u32; 2]) = match &p {
                Plan::Tile { swap, dims, groups } => {
                    if *swap {
                        std::mem::swap(&mut pa, &mut pb);
                    }
                    (kernels.tile, &dims[..], [*groups, 1], [128, 1])
                }
                Plan::Generic { dims, grid } => (kernels.generic, &dims[..], *grid, GENERIC_BLOCK),
            };
            let mut dims = dims.to_vec();
            let (mut alpha, mut beta) = (alpha, beta);
            let mut params: Vec<*mut c_void> = vec![
                &mut pa as *mut _ as *mut c_void,
                &mut pb as *mut _ as *mut c_void,
                &mut pc as *mut _ as *mut c_void,
            ];
            params.extend(dims.iter_mut().map(|v| v as *mut u64 as *mut c_void));
            params.push(&mut alpha as *mut f32 as *mut c_void);
            params.push(&mut beta as *mut f32 as *mut c_void);
            (d.launch)(f, grid[0], grid[1], 1, block[0], block[1], 1, 0, h.stream, params.as_mut_ptr(), std::ptr::null_mut())
                .map_err(|_| cublasError_t::EXECUTION_FAILED)?;
        }
        Ok(())
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
        sgemm(h, transa, transb, m, n, k, alpha, a, lda, 0, b, ldb, 0, beta, c, ldc, 0, 1)
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
        sgemm(h, transa, transb, m, n, k, alpha, a, lda, stride_a, b, ldb, stride_b, beta, c, ldc, stride_c, batch)
    }

    /// Only 32-bit float matrices with a 32-bit float computation (f16 and bf16 kernels are not written yet).
    fn f32_only(types: [cudaDataType; 3], compute: cublasComputeType_t) -> cublasStatus_t {
        let f32_compute = [
            cublasComputeType_t::CUBLAS_COMPUTE_32F,
            cublasComputeType_t::CUBLAS_COMPUTE_32F_PEDANTIC,
            cublasComputeType_t::CUBLAS_COMPUTE_32F_FAST_TF32,
        ];
        if types.iter().all(|t| *t == cudaDataType::CUDA_R_32F) && f32_compute.contains(&compute) {
            Ok(())
        } else {
            Err(cublasError_t::NOT_SUPPORTED)
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
        f32_only([a_type, b_type, c_type], compute)?;
        sgemm(h, transa, transb, m, n, k, alpha.cast(), a.cast(), lda, 0, b.cast(), ldb, 0, beta.cast(), c.cast(), ldc, 0, 1)
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
        f32_only([a_type, b_type, c_type], compute)?;
        sgemm(
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
}

#[cfg(test)]
mod tests {
    use super::*;

    /// What the planned kernel computes, on the CPU, with the kernels' documented semantics.
    fn run_plan(p: &Plan, a: &[f32], b: &[f32], c: &mut [f32], alpha: f32, beta: f32) {
        match p {
            Plan::Tile { swap, dims: [mm, nn, kk, lda, ldb, ldc], .. } => {
                let (a, b) = if *swap { (b, a) } else { (a, b) };
                for i in 0..*mm as usize {
                    for j in 0..*nn as usize {
                        let s: f32 = (0..*kk as usize).map(|k| a[i * *lda as usize + k] * b[k * *ldb as usize + j]).sum();
                        let p = i * *ldc as usize + j;
                        c[p] = alpha * s + if beta == 0.0 { 0.0 } else { beta * c[p] };
                    }
                }
            }
            Plan::Generic { dims: [m, n, k, sar, sac, sbr, sbc, scr, scc], .. } => {
                for i in 0..*m as usize {
                    for j in 0..*n as usize {
                        let s: f32 = (0..*k as usize)
                            .map(|k| a[i * *sar as usize + k * *sac as usize] * b[k * *sbr as usize + j * *sbc as usize])
                            .sum();
                        let p = i * *scr as usize + j * *scc as usize;
                        c[p] = alpha * s + if beta == 0.0 { 0.0 } else { beta * c[p] };
                    }
                }
            }
        }
    }

    /// Column-major reference.
    fn reference(ta: bool, tb: bool, m: usize, n: usize, k: usize, a: &[f32], lda: usize, b: &[f32], ldb: usize, c: &mut [f32], ldc: usize, alpha: f32, beta: f32) {
        for i in 0..m {
            for j in 0..n {
                let s: f32 = (0..k)
                    .map(|l| {
                        let x = if ta { a[l + i * lda] } else { a[i + l * lda] };
                        let y = if tb { b[j + l * ldb] } else { b[l + j * ldb] };
                        x * y
                    })
                    .sum();
                c[i + j * ldc] = alpha * s + if beta == 0.0 { 0.0 } else { beta * c[i + j * ldc] };
            }
        }
    }

    #[test]
    fn plans_compute_column_major_gemm() {
        let mut seed = 7u32;
        let mut rnd = move || {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            (seed >> 8) as f32 / (1 << 24) as f32 - 0.5
        };
        for &(ta, tb, m, n, k) in &[
            (false, false, 64, 128, 16),
            (false, false, 13, 7, 5),
            (true, false, 9, 4, 6),
            (false, true, 3, 11, 2),
            (true, true, 8, 8, 8),
        ] {
            let (lda, ldb, ldc) = ((if ta { k } else { m }) + 2, (if tb { n } else { k }) + 4, m + 6);
            let a: Vec<f32> = (0..lda * if ta { m } else { k }).map(|_| rnd()).collect();
            let b: Vec<f32> = (0..ldb * if tb { k } else { n }).map(|_| rnd()).collect();
            let c0: Vec<f32> = (0..ldc * n).map(|_| rnd()).collect();
            let p = plan(ta, tb, m as u64, n as u64, k as u64, lda as u64, ldb as u64, ldc as u64, true);
            assert_eq!(matches!(p, Plan::Tile { .. }), (ta, tb, m, n, k) == (false, false, 64, 128, 16));
            let (mut got, mut want) = (c0.clone(), c0.clone());
            run_plan(&p, &a, &b, &mut got, 1.5, 0.25);
            reference(ta, tb, m, n, k, &a, lda, &b, ldb, &mut want, ldc, 1.5, 0.25);
            for (g, w) in got.iter().zip(&want) {
                assert!((g - w).abs() <= 1e-5 * (1.0 + w.abs()), "{ta} {tb} {m}x{n}x{k}: {g} != {w}");
            }
        }
    }

    #[test]
    fn the_fat_object_has_both_kernels_for_the_m2_max() {
        assert_eq!(&KERNELS[..8], b"ZAGXFAT1");
        let s = String::from_utf8_lossy(KERNELS);
        assert!(s.contains("agx-g14c") && s.contains("agx-g13g"));
        assert!(s.contains("agx_sgemm_tile64") && s.contains("agx_sgemm_generic"));
    }
}
