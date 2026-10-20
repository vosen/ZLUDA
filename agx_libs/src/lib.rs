//! The engine of the AGX library replacements (zluda_blas_agx, zluda_blaslt_agx, zluda_dnn_agx: cuBLAS, cuBLASLt and
//! cuDNN on Apple GPUs, Asahi Linux).
//!
//! The kernels are MLIR (`kernels/*.mlir`: standard dialects and `agx` operations, written once for every AGX
//! generation), compiled at build time for each chip into fat AGX objects that zluda_agx's `cuModuleLoadData` picks
//! from. The engine calls the CUDA driver API of the process's libcuda (zluda_agx), like NVIDIA's libraries do, and
//! exports no symbols itself.

use cuda_types::cuda::*;
use std::ffi::{c_char, c_void};

/// The fat AGX objects of the kernels (build.rs).
static SGEMM: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sgemm.agxfat"));
static HGEMM: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/hgemm.agxfat"));
static EPILOGUE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/epilogue.agxfat"));
/// The 2-D convolution kernels of zluda_dnn_agx (`kernels/conv.mlir`).
pub static CONV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/conv.agxfat"));
/// The FFT kernels of zluda_fft_agx (`kernels/fft.mlir`).
pub static FFT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/fft.agxfat"));
/// The level 1 and 2 kernels of zluda_blas_agx (`kernels/blas.mlir`).
pub static BLAS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/blas.agxfat"));
/// The sparse kernels of zluda_sparse_agx (`kernels/sparse.mlir`).
pub static SPARSE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sparse.agxfat"));

/// Element types of A and B, and of C.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Types {
    /// f32 A, B and C.
    S,
    /// f16 A and B, f16 C.
    H,
    /// f16 A and B, f32 C.
    HtoS,
}

impl Types {
    fn index(self) -> usize {
        self as usize
    }
    fn ab_bytes(self) -> isize {
        if self == Types::S { 4 } else { 2 }
    }
    fn c_bytes(self) -> isize {
        if self == Types::H { 2 } else { 4 }
    }
}

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

/// IEEE binary16 to f32 (exact).
pub fn f16_to_f32(h: u16) -> f32 {
    let (sign, exp, frac) = ((h as u32 & 0x8000) << 16, (h >> 10) & 0x1f, h as u32 & 0x3ff);
    let bits = match exp {
        0 if frac == 0 => sign,
        0 => {
            // subnormal: normalize
            let shift = frac.leading_zeros() - 21;
            sign | ((113 - shift) << 23) | ((frac << shift) & 0x3ff) << 13
        }
        31 => sign | 0x7f80_0000 | frac << 13,
        e => sign | ((e as u32 + 112) << 23) | frac << 13,
    };
    f32::from_bits(bits)
}

/// The GEMM engine shared with zluda_blaslt_agx: the driver API of the process's libcuda, the kernels (loaded once
/// per context) and their launches.
/// The name and description of a cuBLAS status.
pub fn status_text(s: cuda_types::cublas::cublasStatus_t) -> (&'static std::ffi::CStr, &'static std::ffi::CStr) {
    match s {
        Ok(()) => (c"CUBLAS_STATUS_SUCCESS", c"success"),
        Err(cuda_types::cublas::cublasError_t::NOT_INITIALIZED) => (c"CUBLAS_STATUS_NOT_INITIALIZED", c"the library was not initialized"),
        Err(cuda_types::cublas::cublasError_t::ALLOC_FAILED) => (c"CUBLAS_STATUS_ALLOC_FAILED", c"the resource allocation failed"),
        Err(cuda_types::cublas::cublasError_t::INVALID_VALUE) => (c"CUBLAS_STATUS_INVALID_VALUE", c"an invalid numeric value was used as an argument"),
        Err(cuda_types::cublas::cublasError_t::ARCH_MISMATCH) => (c"CUBLAS_STATUS_ARCH_MISMATCH", c"an absent device architectural feature is required"),
        Err(cuda_types::cublas::cublasError_t::EXECUTION_FAILED) => (c"CUBLAS_STATUS_EXECUTION_FAILED", c"the GPU program failed to execute"),
        Err(cuda_types::cublas::cublasError_t::NOT_SUPPORTED) => (c"CUBLAS_STATUS_NOT_SUPPORTED", c"the functionality requested is not supported"),
        Err(_) => (c"CUBLAS_STATUS_INTERNAL_ERROR", c"an internal operation failed"),
    }
}


pub mod engine {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Error {
        /// No libcuda, or no current context.
        NotInitialized,
        /// The fat objects have no code for this device.
        ArchMismatch,
        /// A driver call failed.
        Failed,
    }

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
    type MemcpyDtoDAsync = unsafe extern "system" fn(CUdeviceptr, CUdeviceptr, usize, CUstream) -> CUresult;
    type CtxGetCurrent = unsafe extern "system" fn(*mut CUcontext) -> CUresult;
    type MemAlloc = unsafe extern "system" fn(*mut CUdeviceptr, usize) -> CUresult;
    type MemFree = unsafe extern "system" fn(CUdeviceptr) -> CUresult;
    type MemcpyHtoD = unsafe extern "system" fn(CUdeviceptr, *const c_void, usize) -> CUresult;
    type StreamSynchronize = unsafe extern "system" fn(CUstream) -> CUresult;

    /// The driver API of the process's libcuda (zluda_agx).
    struct Driver {
        _lib: libloading::Library,
        load: ModuleLoadData,
        function: ModuleGetFunction,
        launch: LaunchKernel,
        dtoh: MemcpyDtoH,
        dtod: MemcpyDtoDAsync,
        current: CtxGetCurrent,
        alloc: MemAlloc,
        free: MemFree,
        htod: MemcpyHtoD,
        sync: StreamSynchronize,
    }

    fn driver() -> Result<&'static Driver, Error> {
        static D: OnceLock<Option<Driver>> = OnceLock::new();
        D.get_or_init(|| unsafe {
            let name = std::env::var_os("ZLUDA_AGX_LIBCUDA").unwrap_or_else(|| "libcuda.so.1".into());
            let lib = libloading::Library::new(name).ok()?;
            Some(Driver {
                load: *lib.get(b"cuModuleLoadData\0").ok()?,
                function: *lib.get(b"cuModuleGetFunction\0").ok()?,
                launch: *lib.get(b"cuLaunchKernel\0").ok()?,
                dtoh: *lib.get(b"cuMemcpyDtoH_v2\0").ok()?,
                dtod: *lib.get(b"cuMemcpyDtoDAsync_v2\0").ok()?,
                current: *lib.get(b"cuCtxGetCurrent\0").ok()?,
                alloc: *lib.get(b"cuMemAlloc_v2\0").ok()?,
                free: *lib.get(b"cuMemFree_v2\0").ok()?,
                htod: *lib.get(b"cuMemcpyHtoD_v2\0").ok()?,
                sync: *lib.get(b"cuStreamSynchronize\0").ok()?,
                _lib: lib,
            })
        })
        .as_ref()
        .ok_or(Error::NotInitialized)
    }

    /// Whether libcuda can be used.
    pub fn available() -> bool {
        driver().is_ok()
    }

    /// The kernels of one context.
    #[derive(Clone, Copy)]
    struct Kernels {
        ctx: usize,
        /// By `Types::index`.
        tile: [CUfunction; 3],
        generic: [CUfunction; 3],
        /// f32 and f16 D.
        epilogue: [CUfunction; 2],
    }
    unsafe impl Send for Kernels {}

    static LOADED: Mutex<Vec<Kernels>> = Mutex::new(Vec::new());

    unsafe fn kernels(d: &Driver) -> Result<Kernels, Error> {
        let mut ctx = CUcontext(std::ptr::null_mut());
        (d.current)(&mut ctx).map_err(|_| Error::NotInitialized)?;
        if ctx.0.is_null() {
            return Err(Error::NotInitialized);
        }
        let key = ctx.0 as usize;
        let mut loaded = LOADED.lock().unwrap();
        if let Some(k) = loaded.iter().find(|k| k.ctx == key) {
            return Ok(*k);
        }
        let load = |image: &[u8]| {
            let mut m = CUmodule(std::ptr::null_mut());
            (d.load)(&mut m, image.as_ptr() as *const c_void).map(|_| m).map_err(|_| Error::ArchMismatch)
        };
        let (sm, hm, em) = (load(SGEMM)?, load(HGEMM)?, load(EPILOGUE)?);
        let get = |m: CUmodule, name: &[u8]| {
            let mut f = CUfunction(std::ptr::null_mut());
            (d.function)(&mut f, m, name.as_ptr() as *const c_char).map(|_| f).map_err(|_| Error::Failed)
        };
        let k = Kernels {
            ctx: key,
            tile: [get(sm, b"agx_sgemm_tile64\0")?, get(hm, b"agx_hgemm_tile64_cf16\0")?, get(hm, b"agx_hgemm_tile64_cf32\0")?],
            generic: [
                get(sm, b"agx_sgemm_generic\0")?,
                get(hm, b"agx_hgemm_generic_cf16\0")?,
                get(hm, b"agx_hgemm_generic_cf32\0")?,
            ],
            epilogue: [get(em, b"agx_epilogue_f32\0")?, get(em, b"agx_epilogue_f16\0")?],
        };
        loaded.push(k);
        Ok(k)
    }

    /// The kernel `name` of a fat AGX object (one of this crate's), loaded once per context.
    pub unsafe fn function(image: &'static [u8], name: &'static std::ffi::CStr) -> Result<CUfunction, Error> {
        #[derive(Clone, Copy)]
        struct F(CUfunction);
        unsafe impl Send for F {}
        static FUNCTIONS: Mutex<Vec<((usize, usize, &'static std::ffi::CStr), F)>> = Mutex::new(Vec::new());
        static MODULES: Mutex<Vec<((usize, usize), usize)>> = Mutex::new(Vec::new());
        let d = driver()?;
        let mut ctx = CUcontext(std::ptr::null_mut());
        (d.current)(&mut ctx).map_err(|_| Error::NotInitialized)?;
        if ctx.0.is_null() {
            return Err(Error::NotInitialized);
        }
        let key = (ctx.0 as usize, image.as_ptr() as usize, name);
        let mut functions = FUNCTIONS.lock().unwrap();
        if let Some((_, f)) = functions.iter().find(|(k, _)| *k == key) {
            return Ok(f.0);
        }
        let mut modules = MODULES.lock().unwrap();
        let m = match modules.iter().find(|(k, _)| *k == (key.0, key.1)) {
            Some(&(_, m)) => CUmodule(m as *mut _),
            None => {
                let mut m = CUmodule(std::ptr::null_mut());
                (d.load)(&mut m, image.as_ptr() as *const c_void).map_err(|_| Error::ArchMismatch)?;
                modules.push(((key.0, key.1), m.0 as usize));
                m
            }
        };
        let mut f = CUfunction(std::ptr::null_mut());
        (d.function)(&mut f, m, name.as_ptr()).map_err(|_| Error::Failed)?;
        functions.push((key, F(f)));
        Ok(f)
    }

    /// Launches `f` on `stream` with `params` (pointers to each argument's value).
    pub unsafe fn launch_kernel(f: CUfunction, grid: [u32; 2], block: [u32; 2], params: &mut [*mut c_void], stream: CUstream) -> Result<(), Error> {
        launch(driver()?, f, grid, block, params, stream)
    }

    /// Device memory for plans and workspaces (`cuMemAlloc`).
    pub unsafe fn alloc(n: usize) -> Result<*mut c_void, Error> {
        let mut p = CUdeviceptr_v2(std::ptr::null_mut());
        (driver()?.alloc)(&mut p, n.max(1)).map_err(|_| Error::Failed)?;
        Ok(p.0)
    }

    pub unsafe fn free(p: *mut c_void) -> Result<(), Error> {
        if p.is_null() {
            return Ok(());
        }
        (driver()?.free)(CUdeviceptr_v2(p)).map_err(|_| Error::Failed)
    }

    /// Copies `n` bytes from host memory to device memory.
    pub unsafe fn write(dst: *mut c_void, src: *const c_void, n: usize) -> Result<(), Error> {
        (driver()?.htod)(CUdeviceptr_v2(dst), src, n).map_err(|_| Error::Failed)
    }

    /// Reads `n` bytes of device memory (alpha and beta in device pointer mode).
    pub unsafe fn read(dst: *mut c_void, src: *const c_void, n: usize) -> Result<(), Error> {
        (driver()?.dtoh)(dst, CUdeviceptr_v2(src as *mut c_void), n).map_err(|_| Error::Failed)
    }

    /// Waits for the work queued on `stream` (results returned to host memory).
    pub unsafe fn synchronize(stream: CUstream) -> Result<(), Error> {
        (driver()?.sync)(stream).map_err(|_| Error::Failed)
    }

    /// Copies `n` bytes of device memory on `stream`.
    pub unsafe fn copy(dst: *mut c_void, src: *const c_void, n: usize, stream: CUstream) -> Result<(), Error> {
        (driver()?.dtod)(CUdeviceptr_v2(dst), CUdeviceptr_v2(src as *mut c_void), n, stream).map_err(|_| Error::Failed)
    }

    /// Column-major `C = alpha op(A) op(B) + beta C` for `batch` matrices `stride_*` elements apart. The arguments
    /// must be valid (the callers check them as cuBLAS specifies).
    pub struct Gemm {
        pub types: Types,
        pub ta: bool,
        pub tb: bool,
        pub m: u64,
        pub n: u64,
        pub k: u64,
        pub alpha: f32,
        pub beta: f32,
        pub a: *const c_void,
        pub lda: u64,
        pub stride_a: i64,
        pub b: *const c_void,
        pub ldb: u64,
        pub stride_b: i64,
        pub c: *mut c_void,
        pub ldc: u64,
        pub stride_c: i64,
        pub batch: u64,
    }

    unsafe fn launch(d: &Driver, f: CUfunction, grid: [u32; 2], block: [u32; 2], params: &mut [*mut c_void], stream: CUstream) -> Result<(), Error> {
        (d.launch)(f, grid[0], grid[1], 1, block[0], block[1], 1, 0, stream, params.as_mut_ptr(), std::ptr::null_mut())
            .map_err(|_| Error::Failed)
    }

    pub unsafe fn gemm(g: &Gemm, stream: CUstream) -> Result<(), Error> {
        if g.m == 0 || g.n == 0 || g.batch == 0 {
            return Ok(());
        }
        let d = driver()?;
        let kernels = kernels(d)?;
        let (types, (a, b, c)) = (g.types, (g.a, g.b, g.c));
        let aligned = [a as usize, b as usize, c as usize].iter().all(|p| p % 8 == 0)
            && [g.stride_a, g.stride_b, g.stride_c].iter().all(|s| s % 2 == 0);
        let p = plan(g.ta, g.tb, g.m, g.n, g.k, g.lda, g.ldb, g.ldc, aligned);
        for i in 0..g.batch as isize {
            let (ab, cb) = (types.ab_bytes(), types.c_bytes());
            let (mut pa, mut pb, mut pc) = (
                a.byte_offset(i * g.stride_a as isize * ab),
                b.byte_offset(i * g.stride_b as isize * ab),
                c.byte_offset(i * g.stride_c as isize * cb),
            );
            let (f, dims, grid, block): (CUfunction, &[u64], [u32; 2], [u32; 2]) = match &p {
                Plan::Tile { swap, dims, groups } => {
                    if *swap {
                        std::mem::swap(&mut pa, &mut pb);
                    }
                    (kernels.tile[types.index()], &dims[..], [*groups, 1], [128, 1])
                }
                Plan::Generic { dims, grid } => (kernels.generic[types.index()], &dims[..], *grid, GENERIC_BLOCK),
            };
            let mut dims = dims.to_vec();
            let (mut alpha, mut beta) = (g.alpha, g.beta);
            let mut params: Vec<*mut c_void> = vec![
                &mut pa as *mut _ as *mut c_void,
                &mut pb as *mut _ as *mut c_void,
                &mut pc as *mut _ as *mut c_void,
            ];
            params.extend(dims.iter_mut().map(|v| v as *mut u64 as *mut c_void));
            params.push(&mut alpha as *mut f32 as *mut c_void);
            params.push(&mut beta as *mut f32 as *mut c_void);
            launch(d, f, grid, block, &mut params, stream)?;
        }
        Ok(())
    }

    /// What `epilogue` applies to each element of D after the product.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Activation {
        None,
        Relu,
        /// The tanh approximation, as cuBLASLt's CUBLASLT_EPILOGUE_GELU.
        Gelu,
    }

    /// `D[i, j] = act(D[i, j] + bias[i])` for column-major m x n matrices (bias has m elements of D's type, or none),
    /// for `batch` matrices `stride` elements apart.
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn epilogue(
        half: bool,
        d_ptr: *mut c_void,
        m: u64,
        n: u64,
        ld: u64,
        stride: i64,
        batch: u64,
        bias: *const c_void,
        act: Activation,
        stream: CUstream,
    ) -> Result<(), Error> {
        if m == 0 || n == 0 || batch == 0 || (bias.is_null() && act == Activation::None) {
            return Ok(());
        }
        let d = driver()?;
        let f = kernels(d)?.epilogue[half as usize];
        let size = if half { 2 } else { 4 };
        for i in 0..batch as isize {
            let mut p = d_ptr.byte_offset(i * stride as isize * size);
            let mut bp = bias;
            let mut dims = [m, n, ld, !bias.is_null() as u64, act as u64];
            let mut params: Vec<*mut c_void> = vec![&mut p as *mut _ as *mut c_void, &mut bp as *mut _ as *mut c_void];
            params.extend(dims.iter_mut().map(|v| v as *mut u64 as *mut c_void));
            launch(d, f, [m.div_ceil(16) as u32, n.div_ceil(8) as u32], [16, 8], &mut params, stream)?;
        }
        Ok(())
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
    fn the_fat_objects_have_the_kernels_for_the_m2_max() {
        for (fat, names) in [
            (SGEMM, &["agx_sgemm_tile64", "agx_sgemm_generic"][..]),
            (HGEMM, &["agx_hgemm_tile64_cf16", "agx_hgemm_tile64_cf32", "agx_hgemm_generic_cf16", "agx_hgemm_generic_cf32"][..]),
        ] {
            assert_eq!(&fat[..8], b"ZAGXFAT1");
            let s = String::from_utf8_lossy(fat);
            assert!(s.contains("agx-g14c") && s.contains("agx-g13g"));
            assert!(names.iter().all(|n| s.contains(n)));
        }
    }

    #[test]
    fn converts_f16() {
        for (h, f) in [(0x3c00, 1.0f32), (0xc000, -2.0), (0x0001, 5.960_464_5e-8), (0x03ff, 6.097_555_2e-5), (0x7bff, 65504.0), (0x3555, 0.333_251_95)] {
            assert_eq!(f16_to_f32(h), f, "{h:#x}");
        }
        assert!(f16_to_f32(0x7e00).is_nan());
        assert_eq!(f16_to_f32(0xfc00), f32::NEG_INFINITY);
        assert_eq!(f16_to_f32(0x8000).to_bits(), 0x8000_0000);
    }
}
