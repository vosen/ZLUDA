//! cuBLASLt on Apple GPUs for zluda_agx (Asahi Linux), on the GEMM engine and MLIR kernels of agx_libs (shared with
//! zluda_blas_agx).
//!
//! `cublasLtMatmul` computes `D = alpha op(A) op(B) + beta C` for column-major matrices with f32 A, B, C and D, or f16
//! A and B with f16 or f32 C and D (accumulated in f32), strided batches, host or device alpha / beta, and the
//! epilogues bias, ReLU, bias + ReLU, GELU and bias + GELU. The heuristic offers one algorithm, which needs no
//! workspace. Other functions return `CUBLAS_STATUS_NOT_SUPPORTED`.

#![allow(non_snake_case)]

use cuda_types::cublas::*;
use cuda_types::cublaslt::*;
use cuda_types::cuda::CUstream;
use std::ffi::{c_char, c_int, c_void};
use agx_libs::engine::{self, Activation};
use agx_libs::Types;

macro_rules! unimplemented_fn {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes_definitions, unused_variables)]
            pub extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                if std::env::var_os("ZLUDA_AGX_LOG").is_some() {
                    eprintln!("[zluda_blaslt_agx] {} is not implemented", stringify!($fn_name));
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

cuda_macros::cublaslt_function_declarations!(
    unimplemented_fn,
    implemented_fn
        <= [
            cublasLtCreate,
            cublasLtDestroy,
            cublasLtDisableCpuInstructionsSetMask,
            cublasLtGetCudartVersion,
            cublasLtGetStatusName,
            cublasLtGetStatusString,
            cublasLtGetVersion,
            cublasLtMatmul,
            cublasLtMatmulAlgoGetHeuristic,
            cublasLtMatmulDescCreate,
            cublasLtMatmulDescDestroy,
            cublasLtMatmulDescGetAttribute,
            cublasLtMatmulDescSetAttribute,
            cublasLtMatmulPreferenceCreate,
            cublasLtMatmulPreferenceDestroy,
            cublasLtMatmulPreferenceSetAttribute,
            cublasLtMatrixLayoutCreate,
            cublasLtMatrixLayoutDestroy,
            cublasLtMatrixLayoutGetAttribute,
            cublasLtMatrixLayoutSetAttribute,
        ]
);

/// A matmul descriptor (behind `cublasLtMatmulDesc_t`).
#[derive(Clone, Debug)]
pub struct Desc {
    pub compute: cublasComputeType_t,
    pub scale: cudaDataType,
    pub pointer_mode: u32,
    pub transa: u32,
    pub transb: u32,
    pub epilogue: u32,
    pub bias: u64,
    pub bias_type: cudaDataType,
    pub bias_batch_stride: i64,
    /// Attributes set to something the library cannot honour (non-zero values of unknown attributes).
    pub unsupported: bool,
}

/// A matrix layout (behind `cublasLtMatrixLayout_t`).
#[derive(Clone, Debug, PartialEq)]
pub struct Layout {
    pub ty: cudaDataType,
    pub order: u32,
    pub rows: u64,
    pub cols: u64,
    pub ld: i64,
    pub batch: i32,
    pub stride: i64,
}

/// The engine's kernel types for the matrix types (A, B, C, D), and whether alpha and beta are f16.
pub fn types(desc: &Desc, a: &Layout, b: &Layout, c: &Layout, d: &Layout) -> Result<(Types, bool), cublasError_t> {
    use cudaDataType as T;
    if c.ty != d.ty || a.ty != b.ty {
        return Err(cublasError_t::NOT_SUPPORTED);
    }
    let t = match (a.ty, d.ty) {
        (T::CUDA_R_32F, T::CUDA_R_32F) => Types::S,
        (T::CUDA_R_16F, T::CUDA_R_16F) => Types::H,
        (T::CUDA_R_16F, T::CUDA_R_32F) => Types::HtoS,
        _ => return Err(cublasError_t::NOT_SUPPORTED),
    };
    let half = match desc.scale {
        T::CUDA_R_32F => false,
        T::CUDA_R_16F if t == Types::H => true,
        _ => return Err(cublasError_t::NOT_SUPPORTED),
    };
    use cublasComputeType_t as K;
    match desc.compute {
        K::CUBLAS_COMPUTE_32F
        | K::CUBLAS_COMPUTE_32F_PEDANTIC
        | K::CUBLAS_COMPUTE_32F_FAST_TF32
        | K::CUBLAS_COMPUTE_32F_FAST_16F
        | K::CUBLAS_COMPUTE_32F_FAST_16BF => {}
        K::CUBLAS_COMPUTE_16F | K::CUBLAS_COMPUTE_16F_PEDANTIC if t == Types::H => {}
        _ => return Err(cublasError_t::NOT_SUPPORTED),
    }
    Ok((t, half))
}

/// The bias and activation of an epilogue, or `None` for one the kernels do not have.
pub fn epilogue(e: cublasLtEpilogue_t) -> Option<(bool, Activation)> {
    use cublasLtEpilogue_t as E;
    Some(match e {
        E::CUBLASLT_EPILOGUE_DEFAULT => (false, Activation::None),
        E::CUBLASLT_EPILOGUE_RELU => (false, Activation::Relu),
        E::CUBLASLT_EPILOGUE_BIAS => (true, Activation::None),
        E::CUBLASLT_EPILOGUE_RELU_BIAS => (true, Activation::Relu),
        E::CUBLASLT_EPILOGUE_GELU => (false, Activation::Gelu),
        E::CUBLASLT_EPILOGUE_GELU_BIAS => (true, Activation::Gelu),
        _ => return None,
    })
}

/// The problem of a matmul: (transa, transb, m, n, k), checked against the layouts.
pub fn shape(desc: &Desc, a: &Layout, b: &Layout, c: &Layout, d: &Layout) -> Result<(bool, bool, u64, u64, u64), cublasError_t> {
    let op = |o: u32| match cublasOperation_t(o) {
        cublasOperation_t::CUBLAS_OP_N => Ok(false),
        cublasOperation_t::CUBLAS_OP_T | cublasOperation_t::CUBLAS_OP_C => Ok(true),
        _ => Err(cublasError_t::INVALID_VALUE),
    };
    let (ta, tb) = (op(desc.transa)?, op(desc.transb)?);
    if [a, b, c, d].iter().any(|l| l.order != cublasLtOrder_t::CUBLASLT_ORDER_COL.0) {
        return Err(cublasError_t::NOT_SUPPORTED);
    }
    let (m, n) = (d.rows, d.cols);
    let (am, ak) = if ta { (a.cols, a.rows) } else { (a.rows, a.cols) };
    let (bk, bn) = if tb { (b.cols, b.rows) } else { (b.rows, b.cols) };
    if am != m || bn != n || ak != bk || c.rows != m || c.cols != n {
        return Err(cublasError_t::INVALID_VALUE);
    }
    for l in [a, b, c, d] {
        if l.ld < (l.rows.max(1) as i64) {
            return Err(cublasError_t::INVALID_VALUE);
        }
    }
    let batch = d.batch;
    if [a.batch, b.batch, c.batch].iter().any(|&x| x != batch && x != 1) || batch < 1 {
        return Err(cublasError_t::INVALID_VALUE);
    }
    Ok((ta, tb, m, n, ak))
}

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

    pub(crate) unsafe fn cublasLtCreate(h: *mut cublasLtHandle_t) -> cublasStatus_t {
        if !engine::available() {
            return Err(cublasError_t::NOT_INITIALIZED);
        }
        // the handle carries no state
        *h.as_mut().ok_or(cublasError_t::INVALID_VALUE)? = Box::into_raw(Box::new(0u64)) as cublasLtHandle_t;
        Ok(())
    }

    pub(crate) unsafe fn cublasLtDestroy(h: cublasLtHandle_t) -> cublasStatus_t {
        if h.is_null() {
            return Err(cublasError_t::NOT_INITIALIZED);
        }
        drop(Box::from_raw(h as *mut u64));
        Ok(())
    }

    pub(crate) unsafe fn cublasLtGetVersion() -> usize {
        120_400
    }

    pub(crate) unsafe fn cublasLtGetCudartVersion() -> usize {
        12_040
    }

    pub(crate) unsafe fn cublasLtDisableCpuInstructionsSetMask(_mask: u32) -> u32 {
        0
    }

    pub(crate) unsafe fn cublasLtGetStatusName(s: cublasStatus_t) -> *const c_char {
        agx_libs::status_text(s).0.as_ptr()
    }

    pub(crate) unsafe fn cublasLtGetStatusString(s: cublasStatus_t) -> *const c_char {
        agx_libs::status_text(s).1.as_ptr()
    }

    unsafe fn desc<'a>(d: cublasLtMatmulDesc_t) -> Result<&'a mut Desc, cublasError_t> {
        (d as *mut Desc).as_mut().ok_or(cublasError_t::INVALID_VALUE)
    }

    unsafe fn layout<'a>(l: cublasLtMatrixLayout_t) -> Result<&'a mut Layout, cublasError_t> {
        (l as *mut Layout).as_mut().ok_or(cublasError_t::INVALID_VALUE)
    }

    /// A little-endian integer attribute value of `size` bytes.
    unsafe fn value(buf: *const c_void, size: usize) -> Result<u64, cublasError_t> {
        if buf.is_null() || !(1..=8).contains(&size) {
            return Err(cublasError_t::INVALID_VALUE);
        }
        let mut v = [0u8; 8];
        std::ptr::copy_nonoverlapping(buf as *const u8, v.as_mut_ptr(), size);
        Ok(u64::from_le_bytes(v))
    }

    /// Writes an attribute value of `n` bytes, as cuBLASLt's GetAttribute functions do.
    unsafe fn put(v: u64, n: usize, buf: *mut c_void, size: usize, written: *mut usize) -> cublasStatus_t {
        if let Some(w) = written.as_mut() {
            *w = n;
        }
        if size == 0 && buf.is_null() {
            return Ok(());
        }
        if size < n || buf.is_null() {
            return Err(cublasError_t::INVALID_VALUE);
        }
        std::ptr::copy_nonoverlapping(v.to_le_bytes().as_ptr(), buf as *mut u8, n);
        Ok(())
    }

    pub(crate) unsafe fn cublasLtMatmulDescCreate(
        out: *mut cublasLtMatmulDesc_t,
        compute: cublasComputeType_t,
        scale: cudaDataType_t,
    ) -> cublasStatus_t {
        let d = Box::new(Desc {
            compute,
            scale,
            pointer_mode: 0,
            transa: 0,
            transb: 0,
            epilogue: cublasLtEpilogue_t::CUBLASLT_EPILOGUE_DEFAULT.0,
            bias: 0,
            bias_type: scale,
            bias_batch_stride: 0,
            unsupported: false,
        });
        *out.as_mut().ok_or(cublasError_t::INVALID_VALUE)? = Box::into_raw(d) as cublasLtMatmulDesc_t;
        Ok(())
    }

    pub(crate) unsafe fn cublasLtMatmulDescDestroy(d: cublasLtMatmulDesc_t) -> cublasStatus_t {
        if d.is_null() {
            return Err(cublasError_t::INVALID_VALUE);
        }
        drop(Box::from_raw(d as *mut Desc));
        Ok(())
    }

    pub(crate) unsafe fn cublasLtMatmulDescSetAttribute(
        d: cublasLtMatmulDesc_t,
        attr: cublasLtMatmulDescAttributes_t,
        buf: *const c_void,
        size: usize,
    ) -> cublasStatus_t {
        use cublasLtMatmulDescAttributes_t as A;
        let d = desc(d)?;
        let v = value(buf, size)?;
        match attr {
            A::CUBLASLT_MATMUL_DESC_COMPUTE_TYPE => d.compute = cublasComputeType_t(v as u32),
            A::CUBLASLT_MATMUL_DESC_SCALE_TYPE => d.scale = cudaDataType(v as u32),
            A::CUBLASLT_MATMUL_DESC_POINTER_MODE => d.pointer_mode = v as u32,
            A::CUBLASLT_MATMUL_DESC_TRANSA => d.transa = v as u32,
            A::CUBLASLT_MATMUL_DESC_TRANSB => d.transb = v as u32,
            A::CUBLASLT_MATMUL_DESC_EPILOGUE => d.epilogue = v as u32,
            A::CUBLASLT_MATMUL_DESC_BIAS_POINTER => d.bias = v,
            A::CUBLASLT_MATMUL_DESC_BIAS_DATA_TYPE => d.bias_type = cudaDataType(v as u32),
            A::CUBLASLT_MATMUL_DESC_BIAS_BATCH_STRIDE => d.bias_batch_stride = v as i64,
            // anything else (scale pointers, aux buffers, fill modes...) only at its default of 0
            _ => d.unsupported |= v != 0,
        }
        Ok(())
    }

    pub(crate) unsafe fn cublasLtMatmulDescGetAttribute(
        d: cublasLtMatmulDesc_t,
        attr: cublasLtMatmulDescAttributes_t,
        buf: *mut c_void,
        size: usize,
        written: *mut usize,
    ) -> cublasStatus_t {
        use cublasLtMatmulDescAttributes_t as A;
        let d = desc(d)?;
        let (v, n) = match attr {
            A::CUBLASLT_MATMUL_DESC_COMPUTE_TYPE => (d.compute.0 as u64, 4),
            A::CUBLASLT_MATMUL_DESC_SCALE_TYPE => (d.scale.0 as u64, 4),
            A::CUBLASLT_MATMUL_DESC_POINTER_MODE => (d.pointer_mode as u64, 4),
            A::CUBLASLT_MATMUL_DESC_TRANSA => (d.transa as u64, 4),
            A::CUBLASLT_MATMUL_DESC_TRANSB => (d.transb as u64, 4),
            A::CUBLASLT_MATMUL_DESC_EPILOGUE => (d.epilogue as u64, 4),
            A::CUBLASLT_MATMUL_DESC_BIAS_POINTER => (d.bias, 8),
            A::CUBLASLT_MATMUL_DESC_BIAS_DATA_TYPE => (d.bias_type.0 as u64, 4),
            A::CUBLASLT_MATMUL_DESC_BIAS_BATCH_STRIDE => (d.bias_batch_stride as u64, 8),
            _ => (0, size.clamp(1, 8)),
        };
        put(v, n, buf, size, written)
    }

    pub(crate) unsafe fn cublasLtMatrixLayoutCreate(
        out: *mut cublasLtMatrixLayout_t,
        ty: cudaDataType,
        rows: u64,
        cols: u64,
        ld: i64,
    ) -> cublasStatus_t {
        let l = Box::new(Layout { ty, order: cublasLtOrder_t::CUBLASLT_ORDER_COL.0, rows, cols, ld, batch: 1, stride: 0 });
        *out.as_mut().ok_or(cublasError_t::INVALID_VALUE)? = Box::into_raw(l) as cublasLtMatrixLayout_t;
        Ok(())
    }

    pub(crate) unsafe fn cublasLtMatrixLayoutDestroy(l: cublasLtMatrixLayout_t) -> cublasStatus_t {
        if l.is_null() {
            return Err(cublasError_t::INVALID_VALUE);
        }
        drop(Box::from_raw(l as *mut Layout));
        Ok(())
    }

    pub(crate) unsafe fn cublasLtMatrixLayoutSetAttribute(
        l: cublasLtMatrixLayout_t,
        attr: cublasLtMatrixLayoutAttribute_t,
        buf: *const c_void,
        size: usize,
    ) -> cublasStatus_t {
        use cublasLtMatrixLayoutAttribute_t as A;
        let l = layout(l)?;
        let v = value(buf, size)?;
        match attr {
            A::CUBLASLT_MATRIX_LAYOUT_TYPE => l.ty = cudaDataType(v as u32),
            A::CUBLASLT_MATRIX_LAYOUT_ORDER => l.order = v as u32,
            A::CUBLASLT_MATRIX_LAYOUT_ROWS => l.rows = v,
            A::CUBLASLT_MATRIX_LAYOUT_COLS => l.cols = v,
            A::CUBLASLT_MATRIX_LAYOUT_LD => l.ld = v as i64,
            A::CUBLASLT_MATRIX_LAYOUT_BATCH_COUNT => l.batch = v as i32,
            A::CUBLASLT_MATRIX_LAYOUT_STRIDED_BATCH_OFFSET => l.stride = v as i64,
            _ if v == 0 => {}
            _ => return Err(cublasError_t::NOT_SUPPORTED),
        }
        Ok(())
    }

    pub(crate) unsafe fn cublasLtMatrixLayoutGetAttribute(
        l: cublasLtMatrixLayout_t,
        attr: cublasLtMatrixLayoutAttribute_t,
        buf: *mut c_void,
        size: usize,
        written: *mut usize,
    ) -> cublasStatus_t {
        use cublasLtMatrixLayoutAttribute_t as A;
        let l = layout(l)?;
        let (v, n) = match attr {
            A::CUBLASLT_MATRIX_LAYOUT_TYPE => (l.ty.0 as u64, 4),
            A::CUBLASLT_MATRIX_LAYOUT_ORDER => (l.order as u64, 4),
            A::CUBLASLT_MATRIX_LAYOUT_ROWS => (l.rows, 8),
            A::CUBLASLT_MATRIX_LAYOUT_COLS => (l.cols, 8),
            A::CUBLASLT_MATRIX_LAYOUT_LD => (l.ld as u64, 8),
            A::CUBLASLT_MATRIX_LAYOUT_BATCH_COUNT => (l.batch as u64, 4),
            A::CUBLASLT_MATRIX_LAYOUT_STRIDED_BATCH_OFFSET => (l.stride as u64, 8),
            _ => (0, size.clamp(1, 8)),
        };
        put(v, n, buf, size, written)
    }

    pub(crate) unsafe fn cublasLtMatmulPreferenceCreate(out: *mut cublasLtMatmulPreference_t) -> cublasStatus_t {
        *out.as_mut().ok_or(cublasError_t::INVALID_VALUE)? = Box::into_raw(Box::new(0u64)) as cublasLtMatmulPreference_t;
        Ok(())
    }

    pub(crate) unsafe fn cublasLtMatmulPreferenceDestroy(p: cublasLtMatmulPreference_t) -> cublasStatus_t {
        if p.is_null() {
            return Err(cublasError_t::INVALID_VALUE);
        }
        drop(Box::from_raw(p as *mut u64));
        Ok(())
    }

    /// Preferences (workspace limits, alignments) do not change the one algorithm.
    pub(crate) unsafe fn cublasLtMatmulPreferenceSetAttribute(
        p: cublasLtMatmulPreference_t,
        _attr: cublasLtMatmulPreferenceAttributes_t,
        buf: *const c_void,
        size: usize,
    ) -> cublasStatus_t {
        if p.is_null() {
            return Err(cublasError_t::INVALID_VALUE);
        }
        value(buf, size).map(|_| ())
    }

    /// Whether the operation is one the kernels can do.
    unsafe fn check(
        d: cublasLtMatmulDesc_t,
        a: cublasLtMatrixLayout_t,
        b: cublasLtMatrixLayout_t,
        c: cublasLtMatrixLayout_t,
        dd: cublasLtMatrixLayout_t,
    ) -> Result<(), cublasError_t> {
        let (d, a, b, c, dd) = (desc(d)?, layout(a)?, layout(b)?, layout(c)?, layout(dd)?);
        types(d, a, b, c, dd)?;
        shape(d, a, b, c, dd)?;
        let (bias, _) = epilogue(cublasLtEpilogue_t(d.epilogue)).ok_or(cublasError_t::NOT_SUPPORTED)?;
        if d.unsupported || (bias && d.bias_type != dd.ty) || (c.ld != dd.ld || c.stride != dd.stride) && c != dd {
            return Err(cublasError_t::NOT_SUPPORTED);
        }
        Ok(())
    }

    pub(crate) unsafe fn cublasLtMatmulAlgoGetHeuristic(
        _h: cublasLtHandle_t,
        d: cublasLtMatmulDesc_t,
        a: cublasLtMatrixLayout_t,
        b: cublasLtMatrixLayout_t,
        c: cublasLtMatrixLayout_t,
        dd: cublasLtMatrixLayout_t,
        _pref: cublasLtMatmulPreference_t,
        requested: c_int,
        results: *mut cublasLtMatmulHeuristicResult_t,
        count: *mut c_int,
    ) -> cublasStatus_t {
        if requested < 1 || results.is_null() || count.is_null() {
            return Err(cublasError_t::INVALID_VALUE);
        }
        check(d, a, b, c, dd)?;
        let mut algo = cublasLtMatmulAlgo_t { data: [0; 8] };
        algo.data[0] = 1;
        results.write(cublasLtMatmulHeuristicResult_t { algo, workspaceSize: 0, state: Ok(()), wavesCount: 1.0, reserved: [0; 4] });
        *count = 1;
        Ok(())
    }

    unsafe fn scalar(d: &Desc, p: *const c_void, half: bool) -> Result<f32, cublasError_t> {
        if p.is_null() {
            return Err(cublasError_t::INVALID_VALUE);
        }
        let mut v = [0u8; 4];
        let n = if half { 2 } else { 4 };
        if d.pointer_mode == cublasLtPointerMode_t::CUBLASLT_POINTER_MODE_DEVICE.0 {
            engine::read(v.as_mut_ptr() as *mut c_void, p, n).map_err(status)?;
        } else if d.pointer_mode == cublasLtPointerMode_t::CUBLASLT_POINTER_MODE_HOST.0 {
            std::ptr::copy_nonoverlapping(p as *const u8, v.as_mut_ptr(), n);
        } else {
            // per-row alpha vectors are not supported
            return Err(cublasError_t::NOT_SUPPORTED);
        }
        Ok(if half { agx_libs::f16_to_f32(u16::from_le_bytes([v[0], v[1]])) } else { f32::from_le_bytes(v) })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) unsafe fn cublasLtMatmul(
        _h: cublasLtHandle_t,
        desc_: cublasLtMatmulDesc_t,
        alpha: *const c_void,
        a: *const c_void,
        adesc: cublasLtMatrixLayout_t,
        b: *const c_void,
        bdesc: cublasLtMatrixLayout_t,
        beta: *const c_void,
        c: *const c_void,
        cdesc: cublasLtMatrixLayout_t,
        dptr: *mut c_void,
        ddesc: cublasLtMatrixLayout_t,
        _algo: *const cublasLtMatmulAlgo_t,
        _workspace: *mut c_void,
        _workspace_size: usize,
        stream: CUstream,
    ) -> cublasStatus_t {
        check(desc_, adesc, bdesc, cdesc, ddesc)?;
        let (d, la, lb, lc, ld) = (desc(desc_)?, layout(adesc)?, layout(bdesc)?, layout(cdesc)?, layout(ddesc)?);
        let (t, half) = types(d, la, lb, lc, ld)?;
        let (ta, tb, m, n, k) = shape(d, la, lb, lc, ld)?;
        let (alpha, beta) = (scalar(d, alpha, half)?, scalar(d, beta, half)?);
        let batch = ld.batch as u64;
        let stride = |l: &Layout| if l.batch == 1 { 0 } else { l.stride };
        let esize = if t == Types::H { 2 } else { 4 };
        // D = beta C first when they are different buffers, then the product accumulates into D
        if beta != 0.0 && c != dptr as *const c_void {
            let span = (ld.ld as u64 * (n - 1) + m) as usize * esize;
            for i in 0..batch as isize {
                let off = i * stride(ld) as isize * esize as isize;
                let src = c.byte_offset(i * stride(lc) as isize * esize as isize);
                engine::copy(dptr.byte_offset(off), src, span, stream).map_err(status)?;
            }
        }
        let g = engine::Gemm {
            types: t,
            ta,
            tb,
            m,
            n,
            k,
            alpha: alpha as f64,
            beta: beta as f64,
            a,
            lda: la.ld as u64,
            stride_a: stride(la),
            b,
            ldb: lb.ld as u64,
            stride_b: stride(lb),
            c: dptr,
            ldc: ld.ld as u64,
            stride_c: stride(ld),
            batch,
        };
        engine::gemm(&g, stream).map_err(status)?;
        let (bias, act) = epilogue(cublasLtEpilogue_t(d.epilogue)).ok_or(cublasError_t::NOT_SUPPORTED)?;
        let bias_ptr = if bias { d.bias as *const c_void } else { std::ptr::null() };
        if bias && bias_ptr.is_null() {
            return Err(cublasError_t::INVALID_VALUE);
        }
        let f16 = t == Types::H;
        if bias && d.bias_batch_stride != 0 && batch > 1 {
            for i in 0..batch as isize {
                let p = dptr.byte_offset(i * stride(ld) as isize * esize as isize);
                let bp = bias_ptr.byte_offset(i * d.bias_batch_stride as isize * esize as isize);
                engine::epilogue(f16, p, m, n, ld.ld as u64, 0, 1, bp, act, stream).map_err(status)?;
            }
        } else {
            engine::epilogue(f16, dptr, m, n, ld.ld as u64, stride(ld), batch, bias_ptr, act, stream).map_err(status)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn layout(ty: cudaDataType, rows: u64, cols: u64, ld: i64) -> Layout {
        Layout { ty, order: 0, rows, cols, ld, batch: 1, stride: 0 }
    }

    fn desc(transa: u32, transb: u32) -> Desc {
        Desc {
            compute: cublasComputeType_t::CUBLAS_COMPUTE_32F,
            scale: cudaDataType::CUDA_R_32F,
            pointer_mode: 0,
            transa,
            transb,
            epilogue: 1,
            bias: 0,
            bias_type: cudaDataType::CUDA_R_32F,
            bias_batch_stride: 0,
            unsupported: false,
        }
    }

    #[test]
    fn shapes_and_types() {
        let f32_ = cudaDataType::CUDA_R_32F;
        let f16_ = cudaDataType::CUDA_R_16F;
        // D (5 x 7) = A^T (A stored 3 x 5) * B (3 x 7)
        let (a, b, c) = (layout(f32_, 3, 5, 3), layout(f32_, 3, 7, 4), layout(f32_, 5, 7, 5));
        assert_eq!(shape(&desc(1, 0), &a, &b, &c, &c), Ok((true, false, 5, 7, 3)));
        assert_eq!(shape(&desc(0, 0), &a, &b, &c, &c), Err(cublasError_t::INVALID_VALUE));
        assert_eq!(types(&desc(1, 0), &a, &b, &c, &c), Ok((Types::S, false)));
        let (h, h32) = (layout(f16_, 5, 7, 5), layout(f32_, 5, 7, 5));
        let (ha, hb) = (layout(f16_, 3, 5, 3), layout(f16_, 3, 7, 4));
        assert_eq!(types(&desc(1, 0), &ha, &hb, &h, &h), Ok((Types::H, false)));
        assert_eq!(types(&desc(1, 0), &ha, &hb, &h32, &h32), Ok((Types::HtoS, false)));
        assert_eq!(types(&desc(1, 0), &ha, &hb, &h, &h32), Err(cublasError_t::NOT_SUPPORTED));
        let mut d16 = desc(1, 0);
        d16.compute = cublasComputeType_t::CUBLAS_COMPUTE_16F;
        d16.scale = f16_;
        assert_eq!(types(&d16, &ha, &hb, &h, &h), Ok((Types::H, true)));
        assert_eq!(types(&d16, &ha, &hb, &h32, &h32), Err(cublasError_t::NOT_SUPPORTED));
    }

    #[test]
    fn epilogues() {
        use cublasLtEpilogue_t as E;
        assert_eq!(epilogue(E::CUBLASLT_EPILOGUE_RELU_BIAS), Some((true, Activation::Relu)));
        assert_eq!(epilogue(E::CUBLASLT_EPILOGUE_GELU), Some((false, Activation::Gelu)));
        assert_eq!(epilogue(E::CUBLASLT_EPILOGUE_DGELU), None);
    }
}
