//! cuSPARSE on Apple GPUs for zluda_agx (Asahi Linux), on the sparse kernels of agx_libs
//! (`agx_libs/kernels/sparse.mlir`, written once for every AGX generation).
//!
//! Supported so far: handles, streams and pointer modes, the legacy matrix descriptor, and the generic API for f32
//! and f64 values (f64 on the f64 copies of the kernels, all operands of one type) with 32- or 64-bit indices (row
//! offsets and columns of the same type), zero- or one-based: `cusparseSpMV` on CSR and COO matrices
//! (non-transposed and transposed), `cusparseSpMM` on CSR matrices (`opA` non-transposed, `opB` non-transposed or
//! transposed, column- or row-major dense matrices), sparse vectors (SpVV, Axpby, Gather, Scatter, Rot),
//! SparseToDense, DenseToSparse to CSR, Csr2cscEx2, SpGEMM, SDDMM and the triangular solves SpSV and SpSM on CSR
//! matrices (CSC ones through the CSR matrix of their transpose). Only SpVV, Csr2cscEx2 and SpGEMM need workspace. Other functions return `CUSPARSE_STATUS_NOT_SUPPORTED`.

#![allow(non_snake_case)]

use cuda_types::cuda::cudaDataType_t;
use cuda_types::cusparse::*;
use std::ffi::{c_char, c_int, c_void};

macro_rules! unimplemented_fn {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes_definitions, unused_variables)]
            pub extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                if std::env::var_os("ZLUDA_AGX_LOG").is_some() {
                    eprintln!("[zluda_sparse_agx] {} is not implemented", stringify!($fn_name));
                }
                Err(cusparseError_t::NOT_SUPPORTED)
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

cuda_macros::cusparse_function_declarations!(
    unimplemented_fn,
    implemented_fn
        <= [
            cusparseAxpby,
            cusparseCreate,
            cusparseCreateConstCoo,
            cusparseCreateConstCsc,
            cusparseCreateConstCsr,
            cusparseCreateConstDnMat,
            cusparseCreateConstDnVec,
            cusparseCreateConstSpVec,
            cusparseCreateCoo,
            cusparseCreateCsc,
            cusparseCreateCsr,
            cusparseCreateDnMat,
            cusparseCreateDnVec,
            cusparseCreateMatDescr,
            cusparseCreateSpVec,
            cusparseCsr2cscEx2,
            cusparseCsr2cscEx2_bufferSize,
            cusparseCscSetPointers,
            cusparseCsrSetPointers,
            cusparseDenseToSparse_analysis,
            cusparseDenseToSparse_bufferSize,
            cusparseDenseToSparse_convert,
            cusparseDestroy,
            cusparseDestroyDnMat,
            cusparseDestroyDnVec,
            cusparseDestroyMatDescr,
            cusparseDestroySpMat,
            cusparseDestroySpVec,
            cusparseGather,
            cusparseGetErrorName,
            cusparseGetErrorString,
            cusparseGetMatDiagType,
            cusparseGetMatFillMode,
            cusparseGetMatIndexBase,
            cusparseGetMatType,
            cusparseGetPointerMode,
            cusparseGetProperty,
            cusparseGetStream,
            cusparseGetVersion,
            cusparseRot,
            cusparseSDDMM,
            cusparseSDDMM_bufferSize,
            cusparseSDDMM_preprocess,
            cusparseScatter,
            cusparseSetMatDiagType,
            cusparseSetMatFillMode,
            cusparseSetMatIndexBase,
            cusparseSetMatType,
            cusparseSetPointerMode,
            cusparseSetStream,
            cusparseSpGEMM_compute,
            cusparseSpGEMM_copy,
            cusparseSpGEMM_createDescr,
            cusparseSpGEMM_destroyDescr,
            cusparseSpGEMM_getNumProducts,
            cusparseSpGEMM_workEstimation,
            cusparseSpMM,
            cusparseSpMM_bufferSize,
            cusparseSpMM_preprocess,
            cusparseSpMV,
            cusparseSpMV_bufferSize,
            cusparseSpMatGetAttribute,
            cusparseSpMatGetSize,
            cusparseSpMatSetAttribute,
            cusparseSpSM_analysis,
            cusparseSpSM_bufferSize,
            cusparseSpSM_createDescr,
            cusparseSpSM_destroyDescr,
            cusparseSpSM_solve,
            cusparseSpSV_analysis,
            cusparseSpSV_bufferSize,
            cusparseSpSV_createDescr,
            cusparseSpSV_destroyDescr,
            cusparseSpSV_solve,
            cusparseSpVV,
            cusparseSpVV_bufferSize,
            cusparseSparseToDense,
            cusparseSparseToDense_bufferSize,
        ]
);

/// The cuSPARSE version reported (12.5.0).
pub const VERSION: (c_int, c_int, c_int) = (12, 5, 0);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Format {
    /// `a` holds rows + 1 row offsets.
    Csr,
    /// `a` holds nnz row indices.
    Coo,
    /// `a` holds cols + 1 column offsets and `col` the row indices: the CSR matrix of the transpose
    /// ([`SpMat::csr`]).
    Csc,
}

/// A sparse matrix of the generic API.
#[derive(Clone, Debug, PartialEq)]
pub struct SpMat {
    pub format: Format,
    pub rows: i64,
    pub cols: i64,
    pub nnz: i64,
    pub a: *const c_void,
    pub col: *const c_void,
    pub val: *const c_void,
    /// 64-bit (true) or 32-bit indices.
    pub wide: bool,
    pub base: i64,
    /// f64 (true) or f32 values.
    pub double: bool,
    /// The triangle (CUSPARSE_SPMAT_FILL_MODE) and diagonal (CUSPARSE_SPMAT_DIAG_TYPE) SpSV and SpSM use.
    pub upper: bool,
    pub unit: bool,
}

impl SpMat {
    /// A CSC matrix as the CSR matrix of its transpose, with `op` changed to match (the triangle of SpSV and SpSM
    /// flips with it); other matrices as they are.
    pub fn csr(&self, op: cusparseOperation_t) -> (SpMat, cusparseOperation_t) {
        if self.format != Format::Csc {
            return (self.clone(), op);
        }
        let t = SpMat { format: Format::Csr, rows: self.cols, cols: self.rows, upper: !self.upper, ..self.clone() };
        (t, flip(op))
    }
}

/// The transposed operation (other values stay invalid).
pub fn flip(op: cusparseOperation_t) -> cusparseOperation_t {
    match op {
        cusparseOperation_t::CUSPARSE_OPERATION_NON_TRANSPOSE => cusparseOperation_t::CUSPARSE_OPERATION_TRANSPOSE,
        cusparseOperation_t::CUSPARSE_OPERATION_TRANSPOSE | cusparseOperation_t::CUSPARSE_OPERATION_CONJUGATE_TRANSPOSE => {
            cusparseOperation_t::CUSPARSE_OPERATION_NON_TRANSPOSE
        }
        op => op,
    }
}

/// A sparse vector of the generic API.
#[derive(Clone, Debug, PartialEq)]
pub struct SpVec {
    pub size: i64,
    pub nnz: i64,
    pub idx: *const c_void,
    pub val: *const c_void,
    pub wide: bool,
    pub base: i64,
    pub double: bool,
}

/// A dense vector of the generic API.
#[derive(Clone, Debug, PartialEq)]
pub struct DnVec {
    pub size: i64,
    pub values: *const c_void,
    pub ty: cudaDataType_t,
}

/// A dense matrix of the generic API.
#[derive(Clone, Debug, PartialEq)]
pub struct DnMat {
    pub rows: i64,
    pub cols: i64,
    pub ld: i64,
    pub values: *const c_void,
    pub ty: cudaDataType_t,
    pub row_major: bool,
}

impl DnMat {
    /// The transpose, on the same memory.
    pub fn transposed(&self) -> DnMat {
        DnMat { rows: self.cols, cols: self.rows, row_major: !self.row_major, ..self.clone() }
    }

    /// The element strides of a row and of a column.
    pub fn strides(&self) -> (i64, i64) {
        if self.row_major {
            (self.ld, 1)
        } else {
            (1, self.ld)
        }
    }
}

/// The legacy matrix descriptor (only stored).
#[derive(Clone, Debug, PartialEq)]
pub struct MatDescr {
    pub ty: cusparseMatrixType_t,
    pub fill: cusparseFillMode_t,
    pub diag: cusparseDiagType_t,
    pub base: cusparseIndexBase_t,
}

impl Default for MatDescr {
    fn default() -> MatDescr {
        MatDescr {
            ty: cusparseMatrixType_t::CUSPARSE_MATRIX_TYPE_GENERAL,
            fill: cusparseFillMode_t::CUSPARSE_FILL_MODE_LOWER,
            diag: cusparseDiagType_t::CUSPARSE_DIAG_TYPE_NON_UNIT,
            base: cusparseIndexBase_t::CUSPARSE_INDEX_BASE_ZERO,
        }
    }
}

/// One kernel launch of sparse.mlir; the matrix, x/B and y/C come from the call.
#[derive(Clone, Debug, PartialEq)]
pub enum Launch {
    /// y[i * stride] *= beta for i < n.
    Scale { n: i64, stride: i64 },
    /// y = alpha A x + beta y, one thread per row.
    CsrSpmv,
    /// y += alpha A^T x, one thread per row (atomic).
    CsrSpmvT,
    /// y += alpha op(A) x, one thread per entry (atomic).
    CooSpmv { trans: bool },
    /// C = alpha A op(B) + beta C for n columns, with the element strides of op(B) and C.
    CsrSpmm { n: i64, brs: i64, bcs: i64, crs: i64, ccs: i64 },
}

/// Whether a value type is f64 (else f32); others are not supported.
fn value_type(ty: cudaDataType_t) -> Result<bool, cusparseError_t> {
    match ty {
        cudaDataType_t::CUDA_R_32F => Ok(false),
        cudaDataType_t::CUDA_R_64F => Ok(true),
        _ => Err(cusparseError_t::NOT_SUPPORTED),
    }
}

/// The dense operands and the compute type must have the matrix's value type.
fn same_type(a: &SpMat, tys: &[cudaDataType_t]) -> Result<(), cusparseError_t> {
    for &ty in tys {
        if value_type(ty)? != a.double {
            return Err(cusparseError_t::NOT_SUPPORTED);
        }
    }
    Ok(())
}

/// The launches of `y = alpha op(A) x + beta y`.
pub fn spmv_plan(op: cusparseOperation_t, a: &SpMat, x: &DnVec, y: &DnVec, compute: cudaDataType_t) -> Result<Vec<Launch>, cusparseError_t> {
    let trans = match op {
        cusparseOperation_t::CUSPARSE_OPERATION_NON_TRANSPOSE => false,
        // real values: the conjugate transpose is the transpose
        cusparseOperation_t::CUSPARSE_OPERATION_TRANSPOSE | cusparseOperation_t::CUSPARSE_OPERATION_CONJUGATE_TRANSPOSE => true,
        _ => return Err(cusparseError_t::INVALID_VALUE),
    };
    same_type(a, &[x.ty, y.ty, compute])?;
    let (xn, yn) = if trans { (a.rows, a.cols) } else { (a.cols, a.rows) };
    if x.size != xn || y.size != yn {
        return Err(cusparseError_t::INVALID_VALUE);
    }
    if yn == 0 {
        return Ok(vec![]);
    }
    Ok(match (a.format, trans) {
        (Format::Csr, false) => vec![Launch::CsrSpmv],
        (Format::Csr, true) if a.rows == 0 || a.nnz == 0 => {
            vec![Launch::Scale { n: yn, stride: 1 }]
        }
        (Format::Csr, true) => vec![Launch::Scale { n: yn, stride: 1 }, Launch::CsrSpmvT],
        (Format::Coo, _) if a.nnz == 0 => vec![Launch::Scale { n: yn, stride: 1 }],
        (Format::Coo, trans) => vec![Launch::Scale { n: yn, stride: 1 }, Launch::CooSpmv { trans }],
        // callers pass CSC matrices through SpMat::csr
        (Format::Csc, _) => return Err(cusparseError_t::NOT_SUPPORTED),
    })
}

/// The launch of `C = alpha op(A) op(B) + beta C`.
pub fn spmm_plan(
    op_a: cusparseOperation_t,
    op_b: cusparseOperation_t,
    a: &SpMat,
    b: &DnMat,
    c: &DnMat,
    compute: cudaDataType_t,
) -> Result<Option<Launch>, cusparseError_t> {
    same_type(a, &[b.ty, c.ty, compute])?;
    if op_a != cusparseOperation_t::CUSPARSE_OPERATION_NON_TRANSPOSE || a.format != Format::Csr {
        return Err(cusparseError_t::NOT_SUPPORTED);
    }
    let tb = match op_b {
        cusparseOperation_t::CUSPARSE_OPERATION_NON_TRANSPOSE => false,
        cusparseOperation_t::CUSPARSE_OPERATION_TRANSPOSE | cusparseOperation_t::CUSPARSE_OPERATION_CONJUGATE_TRANSPOSE => true,
        _ => return Err(cusparseError_t::INVALID_VALUE),
    };
    let (bk, bn) = if tb { (b.cols, b.rows) } else { (b.rows, b.cols) };
    if bk != a.cols || c.rows != a.rows || c.cols != bn {
        return Err(cusparseError_t::INVALID_VALUE);
    }
    let fits = |m: &DnMat| m.ld >= if m.row_major { m.cols } else { m.rows }.max(1);
    if !fits(b) || !fits(c) {
        return Err(cusparseError_t::INVALID_VALUE);
    }
    if c.rows == 0 || c.cols == 0 {
        return Ok(None);
    }
    let (brs, bcs) = if tb { (b.strides().1, b.strides().0) } else { b.strides() };
    let (crs, ccs) = c.strides();
    Ok(Some(Launch::CsrSpmm { n: bn, brs, bcs, crs, ccs }))
}

/// Whether `op` transposes; for real values the conjugate transpose is the transpose.
fn transposes(op: cusparseOperation_t) -> Result<bool, cusparseError_t> {
    match op {
        cusparseOperation_t::CUSPARSE_OPERATION_NON_TRANSPOSE => Ok(false),
        cusparseOperation_t::CUSPARSE_OPERATION_TRANSPOSE | cusparseOperation_t::CUSPARSE_OPERATION_CONJUGATE_TRANSPOSE => Ok(true),
        _ => Err(cusparseError_t::INVALID_VALUE),
    }
}

/// The rows, columns and element strides (row, column) of op(m).
pub fn op_dims(op: cusparseOperation_t, m: &DnMat) -> Result<(i64, i64, i64, i64), cusparseError_t> {
    let (rs, cs) = m.strides();
    Ok(if transposes(op)? { (m.cols, m.rows, cs, rs) } else { (m.rows, m.cols, rs, cs) })
}

/// `K` and the strides of op(A) and op(B) of `C = alpha op(A) op(B) ∘ spy(C) + beta C` (SDDMM, C in CSR).
pub fn sddmm_plan(
    op_a: cusparseOperation_t,
    op_b: cusparseOperation_t,
    a: &DnMat,
    b: &DnMat,
    c: &SpMat,
    compute: cudaDataType_t,
) -> Result<[i64; 5], cusparseError_t> {
    same_type(c, &[a.ty, b.ty, compute])?;
    if c.format != Format::Csr {
        return Err(cusparseError_t::NOT_SUPPORTED);
    }
    let (am, ak, ars, acs) = op_dims(op_a, a)?;
    let (bk, bn, brs, bcs) = op_dims(op_b, b)?;
    if am != c.rows || bn != c.cols || ak != bk {
        return Err(cusparseError_t::INVALID_VALUE);
    }
    Ok([ak, ars, acs, brs, bcs])
}

/// The checks of `op(A) C = alpha op(B)` (SpSV with one column, SpSM): A square in CSR, op(B) and C rows x n.
/// Returns whether A is transposed and the strides of op(B) and C.
pub fn spsm_plan(op_a: cusparseOperation_t, a: &SpMat, b: (i64, i64, i64, i64), c: (i64, i64, i64, i64)) -> Result<(bool, [i64; 4]), cusparseError_t> {
    let trans = transposes(op_a)?;
    if a.format != Format::Csr {
        return Err(cusparseError_t::NOT_SUPPORTED);
    }
    if a.rows != a.cols || b.0 != a.rows || c.0 != a.rows || b.1 != c.1 {
        return Err(cusparseError_t::INVALID_VALUE);
    }
    Ok((trans, [b.2, b.3, c.2, c.3]))
}

/// Csr2cscEx2 moves values in words of 4 bytes (f32 kernel) or 8 (f64 kernel): the words per value and whether
/// they are 8 bytes.
pub fn permute_words(ty: cudaDataType_t) -> Result<(i64, bool), cusparseError_t> {
    match ty {
        cudaDataType_t::CUDA_R_32F | cudaDataType_t::CUDA_R_32I => Ok((1, false)),
        cudaDataType_t::CUDA_R_64F | cudaDataType_t::CUDA_C_32F => Ok((1, true)),
        cudaDataType_t::CUDA_C_64F => Ok((2, true)),
        _ => Err(cusparseError_t::NOT_SUPPORTED),
    }
}

/// The partial sums of cusparseSpVV (its workspace holds them and the result).
pub const VV_THREADS: i64 = 256;

mod imp {
    use super::*;
    use agx_libs::engine;

    struct Handle {
        stream: cudaStream_t,
        device_scalars: bool,
    }

    fn status(e: engine::Error) -> cusparseError_t {
        match e {
            engine::Error::NotInitialized => cusparseError_t::NOT_INITIALIZED,
            engine::Error::ArchMismatch => cusparseError_t::ARCH_MISMATCH,
            engine::Error::Failed => cusparseError_t::EXECUTION_FAILED,
        }
    }

    unsafe fn obj<'a, T>(p: *const impl Sized) -> Result<&'a mut T, cusparseError_t> {
        (p as *mut T).as_mut().ok_or(cusparseError_t::INVALID_VALUE)
    }

    unsafe fn out<T>(p: *mut T, v: T) -> cusparseStatus_t {
        *p.as_mut().ok_or(cusparseError_t::INVALID_VALUE)? = v;
        Ok(())
    }

    unsafe fn new<T, P>(p: *mut *mut P, v: T) -> cusparseStatus_t {
        out(p, Box::into_raw(Box::new(v)) as *mut P)
    }

    unsafe fn new_const<T, P>(p: *mut *const P, v: T) -> cusparseStatus_t {
        out(p, Box::into_raw(Box::new(v)) as *const P)
    }

    unsafe fn destroy<T, P>(p: *const P) -> cusparseStatus_t {
        if !p.is_null() {
            drop(Box::from_raw(p as *mut T));
        }
        Ok(())
    }

    pub(crate) unsafe fn cusparseGetErrorName(s: cusparseStatus_t) -> *const c_char {
        error_text(s).0.as_ptr()
    }

    pub(crate) unsafe fn cusparseGetErrorString(s: cusparseStatus_t) -> *const c_char {
        error_text(s).1.as_ptr()
    }

    fn error_text(s: cusparseStatus_t) -> (&'static std::ffi::CStr, &'static std::ffi::CStr) {
        match s {
            Ok(()) => (c"CUSPARSE_STATUS_SUCCESS", c"success"),
            Err(cusparseError_t::NOT_INITIALIZED) => (c"CUSPARSE_STATUS_NOT_INITIALIZED", c"initialization error"),
            Err(cusparseError_t::ALLOC_FAILED) => (c"CUSPARSE_STATUS_ALLOC_FAILED", c"out of memory"),
            Err(cusparseError_t::INVALID_VALUE) => (c"CUSPARSE_STATUS_INVALID_VALUE", c"invalid value"),
            Err(cusparseError_t::ARCH_MISMATCH) => (c"CUSPARSE_STATUS_ARCH_MISMATCH", c"architecture mismatch"),
            Err(cusparseError_t::EXECUTION_FAILED) => (c"CUSPARSE_STATUS_EXECUTION_FAILED", c"execution failed"),
            Err(cusparseError_t::NOT_SUPPORTED) => (c"CUSPARSE_STATUS_NOT_SUPPORTED", c"operation not supported"),
            Err(_) => (c"CUSPARSE_STATUS_INTERNAL_ERROR", c"internal operation failed"),
        }
    }

    pub(crate) unsafe fn cusparseGetProperty(ty: libraryPropertyType, v: *mut c_int) -> cusparseStatus_t {
        out(
            v,
            match ty {
                libraryPropertyType::MAJOR_VERSION => VERSION.0,
                libraryPropertyType::MINOR_VERSION => VERSION.1,
                libraryPropertyType::PATCH_LEVEL => VERSION.2,
                _ => return Err(cusparseError_t::INVALID_VALUE),
            },
        )
    }

    pub(crate) unsafe fn cusparseGetVersion(h: cusparseHandle_t, v: *mut c_int) -> cusparseStatus_t {
        obj::<Handle>(h)?;
        out(v, VERSION.0 * 1000 + VERSION.1 * 100 + VERSION.2)
    }

    pub(crate) unsafe fn cusparseCreate(h: *mut cusparseHandle_t) -> cusparseStatus_t {
        if !engine::available() {
            return Err(cusparseError_t::NOT_INITIALIZED);
        }
        new(h, Handle { stream: cuda_types::cuda::CUstream(std::ptr::null_mut()), device_scalars: false })
    }

    pub(crate) unsafe fn cusparseDestroy(h: cusparseHandle_t) -> cusparseStatus_t {
        destroy::<Handle, _>(h)
    }

    pub(crate) unsafe fn cusparseSetStream(h: cusparseHandle_t, s: cudaStream_t) -> cusparseStatus_t {
        obj::<Handle>(h)?.stream = s;
        Ok(())
    }

    pub(crate) unsafe fn cusparseGetStream(h: cusparseHandle_t, s: *mut cudaStream_t) -> cusparseStatus_t {
        out(s, obj::<Handle>(h)?.stream)
    }

    pub(crate) unsafe fn cusparseSetPointerMode(h: cusparseHandle_t, m: cusparsePointerMode_t) -> cusparseStatus_t {
        obj::<Handle>(h)?.device_scalars = match m {
            cusparsePointerMode_t::CUSPARSE_POINTER_MODE_HOST => false,
            cusparsePointerMode_t::CUSPARSE_POINTER_MODE_DEVICE => true,
            _ => return Err(cusparseError_t::INVALID_VALUE),
        };
        Ok(())
    }

    pub(crate) unsafe fn cusparseGetPointerMode(h: cusparseHandle_t, m: *mut cusparsePointerMode_t) -> cusparseStatus_t {
        out(
            m,
            if obj::<Handle>(h)?.device_scalars {
                cusparsePointerMode_t::CUSPARSE_POINTER_MODE_DEVICE
            } else {
                cusparsePointerMode_t::CUSPARSE_POINTER_MODE_HOST
            },
        )
    }

    // ---- the legacy matrix descriptor

    pub(crate) unsafe fn cusparseCreateMatDescr(d: *mut cusparseMatDescr_t) -> cusparseStatus_t {
        new(d, MatDescr::default())
    }

    pub(crate) unsafe fn cusparseDestroyMatDescr(d: cusparseMatDescr_t) -> cusparseStatus_t {
        destroy::<MatDescr, _>(d)
    }

    pub(crate) unsafe fn cusparseSetMatType(d: cusparseMatDescr_t, v: cusparseMatrixType_t) -> cusparseStatus_t {
        if v.0 > cusparseMatrixType_t::CUSPARSE_MATRIX_TYPE_TRIANGULAR.0 {
            return Err(cusparseError_t::INVALID_VALUE);
        }
        obj::<MatDescr>(d)?.ty = v;
        Ok(())
    }

    pub(crate) unsafe fn cusparseSetMatFillMode(d: cusparseMatDescr_t, v: cusparseFillMode_t) -> cusparseStatus_t {
        if v.0 > cusparseFillMode_t::CUSPARSE_FILL_MODE_UPPER.0 {
            return Err(cusparseError_t::INVALID_VALUE);
        }
        obj::<MatDescr>(d)?.fill = v;
        Ok(())
    }

    pub(crate) unsafe fn cusparseSetMatDiagType(d: cusparseMatDescr_t, v: cusparseDiagType_t) -> cusparseStatus_t {
        if v.0 > cusparseDiagType_t::CUSPARSE_DIAG_TYPE_UNIT.0 {
            return Err(cusparseError_t::INVALID_VALUE);
        }
        obj::<MatDescr>(d)?.diag = v;
        Ok(())
    }

    pub(crate) unsafe fn cusparseSetMatIndexBase(d: cusparseMatDescr_t, v: cusparseIndexBase_t) -> cusparseStatus_t {
        if v.0 > cusparseIndexBase_t::CUSPARSE_INDEX_BASE_ONE.0 {
            return Err(cusparseError_t::INVALID_VALUE);
        }
        obj::<MatDescr>(d)?.base = v;
        Ok(())
    }

    // The getters return the default for a null descriptor, as they cannot report an error.
    unsafe fn descr(d: cusparseMatDescr_t) -> MatDescr {
        obj::<MatDescr>(d).map(|d| d.clone()).unwrap_or_default()
    }

    pub(crate) unsafe fn cusparseGetMatType(d: cusparseMatDescr_t) -> cusparseMatrixType_t {
        descr(d).ty
    }

    pub(crate) unsafe fn cusparseGetMatFillMode(d: cusparseMatDescr_t) -> cusparseFillMode_t {
        descr(d).fill
    }

    pub(crate) unsafe fn cusparseGetMatDiagType(d: cusparseMatDescr_t) -> cusparseDiagType_t {
        descr(d).diag
    }

    pub(crate) unsafe fn cusparseGetMatIndexBase(d: cusparseMatDescr_t) -> cusparseIndexBase_t {
        descr(d).base
    }

    // ---- generic API descriptors

    fn index(ty: cusparseIndexType_t) -> Result<bool, cusparseError_t> {
        match ty {
            cusparseIndexType_t::CUSPARSE_INDEX_32I => Ok(false),
            cusparseIndexType_t::CUSPARSE_INDEX_64I => Ok(true),
            cusparseIndexType_t::CUSPARSE_INDEX_16U => Err(cusparseError_t::NOT_SUPPORTED),
            _ => Err(cusparseError_t::INVALID_VALUE),
        }
    }

    fn base(b: cusparseIndexBase_t) -> Result<i64, cusparseError_t> {
        match b {
            cusparseIndexBase_t::CUSPARSE_INDEX_BASE_ZERO => Ok(0),
            cusparseIndexBase_t::CUSPARSE_INDEX_BASE_ONE => Ok(1),
            _ => Err(cusparseError_t::INVALID_VALUE),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn sp_mat(
        format: Format,
        rows: i64,
        cols: i64,
        nnz: i64,
        a: *const c_void,
        col: *const c_void,
        val: *const c_void,
        a_ty: cusparseIndexType_t,
        col_ty: cusparseIndexType_t,
        idx_base: cusparseIndexBase_t,
        ty: cudaDataType,
    ) -> Result<SpMat, cusparseError_t> {
        if rows < 0 || cols < 0 || nnz < 0 || (format == Format::Csr && rows > 0 && a.is_null()) || (format == Format::Csc && cols > 0 && a.is_null()) {
            return Err(cusparseError_t::INVALID_VALUE);
        }
        if nnz > 0 && (a.is_null() || col.is_null() || val.is_null()) {
            return Err(cusparseError_t::INVALID_VALUE);
        }
        let wide = index(a_ty)?;
        if index(col_ty)? != wide {
            return Err(cusparseError_t::NOT_SUPPORTED);
        }
        let double = value_type(ty)?;
        Ok(SpMat { format, rows, cols, nnz, a, col, val, wide, base: base(idx_base)?, double, upper: false, unit: false })
    }

    pub(crate) unsafe fn cusparseCreateCsr(
        d: *mut cusparseSpMatDescr_t,
        rows: i64,
        cols: i64,
        nnz: i64,
        offsets: *mut c_void,
        col: *mut c_void,
        val: *mut c_void,
        offsets_ty: cusparseIndexType_t,
        col_ty: cusparseIndexType_t,
        idx_base: cusparseIndexBase_t,
        ty: cudaDataType,
    ) -> cusparseStatus_t {
        new(d, sp_mat(Format::Csr, rows, cols, nnz, offsets, col, val, offsets_ty, col_ty, idx_base, ty)?)
    }

    pub(crate) unsafe fn cusparseCreateConstCsr(
        d: *mut cusparseConstSpMatDescr_t,
        rows: i64,
        cols: i64,
        nnz: i64,
        offsets: *const c_void,
        col: *const c_void,
        val: *const c_void,
        offsets_ty: cusparseIndexType_t,
        col_ty: cusparseIndexType_t,
        idx_base: cusparseIndexBase_t,
        ty: cudaDataType,
    ) -> cusparseStatus_t {
        new_const(d, sp_mat(Format::Csr, rows, cols, nnz, offsets, col, val, offsets_ty, col_ty, idx_base, ty)?)
    }

    pub(crate) unsafe fn cusparseCreateCsc(
        d: *mut cusparseSpMatDescr_t,
        rows: i64,
        cols: i64,
        nnz: i64,
        offsets: *mut c_void,
        row: *mut c_void,
        val: *mut c_void,
        offsets_ty: cusparseIndexType_t,
        row_ty: cusparseIndexType_t,
        idx_base: cusparseIndexBase_t,
        ty: cudaDataType,
    ) -> cusparseStatus_t {
        new(d, sp_mat(Format::Csc, rows, cols, nnz, offsets, row, val, offsets_ty, row_ty, idx_base, ty)?)
    }

    pub(crate) unsafe fn cusparseCreateConstCsc(
        d: *mut cusparseConstSpMatDescr_t,
        rows: i64,
        cols: i64,
        nnz: i64,
        offsets: *const c_void,
        row: *const c_void,
        val: *const c_void,
        offsets_ty: cusparseIndexType_t,
        row_ty: cusparseIndexType_t,
        idx_base: cusparseIndexBase_t,
        ty: cudaDataType,
    ) -> cusparseStatus_t {
        new_const(d, sp_mat(Format::Csc, rows, cols, nnz, offsets, row, val, offsets_ty, row_ty, idx_base, ty)?)
    }

    pub(crate) unsafe fn cusparseCreateCoo(
        d: *mut cusparseSpMatDescr_t,
        rows: i64,
        cols: i64,
        nnz: i64,
        row: *mut c_void,
        col: *mut c_void,
        val: *mut c_void,
        idx_ty: cusparseIndexType_t,
        idx_base: cusparseIndexBase_t,
        ty: cudaDataType,
    ) -> cusparseStatus_t {
        new(d, sp_mat(Format::Coo, rows, cols, nnz, row, col, val, idx_ty, idx_ty, idx_base, ty)?)
    }

    pub(crate) unsafe fn cusparseCreateConstCoo(
        d: *mut cusparseConstSpMatDescr_t,
        rows: i64,
        cols: i64,
        nnz: i64,
        row: *const c_void,
        col: *const c_void,
        val: *const c_void,
        idx_ty: cusparseIndexType_t,
        idx_base: cusparseIndexBase_t,
        ty: cudaDataType,
    ) -> cusparseStatus_t {
        new_const(d, sp_mat(Format::Coo, rows, cols, nnz, row, col, val, idx_ty, idx_ty, idx_base, ty)?)
    }

    pub(crate) unsafe fn cusparseDestroySpMat(d: cusparseConstSpMatDescr_t) -> cusparseStatus_t {
        destroy::<SpMat, _>(d)
    }

    pub(crate) unsafe fn cusparseSpMatGetSize(d: cusparseConstSpMatDescr_t, rows: *mut i64, cols: *mut i64, nnz: *mut i64) -> cusparseStatus_t {
        let m = obj::<SpMat>(d)?;
        out(rows, m.rows)?;
        out(cols, m.cols)?;
        out(nnz, m.nnz)
    }

    fn dn_vec(size: i64, values: *const c_void, ty: cudaDataType) -> Result<DnVec, cusparseError_t> {
        if size < 0 || (size > 0 && values.is_null()) {
            return Err(cusparseError_t::INVALID_VALUE);
        }
        Ok(DnVec { size, values, ty })
    }

    pub(crate) unsafe fn cusparseCreateDnVec(d: *mut cusparseDnVecDescr_t, size: i64, values: *mut c_void, ty: cudaDataType) -> cusparseStatus_t {
        new(d, dn_vec(size, values, ty)?)
    }

    pub(crate) unsafe fn cusparseCreateConstDnVec(d: *mut cusparseConstDnVecDescr_t, size: i64, values: *const c_void, ty: cudaDataType) -> cusparseStatus_t {
        new_const(d, dn_vec(size, values, ty)?)
    }

    pub(crate) unsafe fn cusparseDestroyDnVec(d: cusparseConstDnVecDescr_t) -> cusparseStatus_t {
        destroy::<DnVec, _>(d)
    }

    fn dn_mat(rows: i64, cols: i64, ld: i64, values: *const c_void, ty: cudaDataType, order: cusparseOrder_t) -> Result<DnMat, cusparseError_t> {
        let row_major = match order {
            cusparseOrder_t::CUSPARSE_ORDER_COL => false,
            cusparseOrder_t::CUSPARSE_ORDER_ROW => true,
            _ => return Err(cusparseError_t::INVALID_VALUE),
        };
        if rows < 0 || cols < 0 || ld < if row_major { cols } else { rows } || (rows * cols > 0 && values.is_null()) {
            return Err(cusparseError_t::INVALID_VALUE);
        }
        Ok(DnMat { rows, cols, ld, values, ty, row_major })
    }

    pub(crate) unsafe fn cusparseCreateDnMat(
        d: *mut cusparseDnMatDescr_t,
        rows: i64,
        cols: i64,
        ld: i64,
        values: *mut c_void,
        ty: cudaDataType,
        order: cusparseOrder_t,
    ) -> cusparseStatus_t {
        new(d, dn_mat(rows, cols, ld, values, ty, order)?)
    }

    pub(crate) unsafe fn cusparseCreateConstDnMat(
        d: *mut cusparseConstDnMatDescr_t,
        rows: i64,
        cols: i64,
        ld: i64,
        values: *const c_void,
        ty: cudaDataType,
        order: cusparseOrder_t,
    ) -> cusparseStatus_t {
        new_const(d, dn_mat(rows, cols, ld, values, ty, order)?)
    }

    pub(crate) unsafe fn cusparseDestroyDnMat(d: cusparseConstDnMatDescr_t) -> cusparseStatus_t {
        destroy::<DnMat, _>(d)
    }

    // ---- SpMV and SpMM

    /// alpha or beta of the value type (f64 when `double`), from host or device memory.
    unsafe fn scalar(h: &Handle, p: *const c_void, double: bool) -> Result<f64, cusparseError_t> {
        if p.is_null() {
            return Err(cusparseError_t::INVALID_VALUE);
        }
        let mut v = [0u8; 8];
        let n = if double { 8 } else { 4 };
        if h.device_scalars {
            engine::read(v.as_mut_ptr() as *mut c_void, p, n).map_err(status)?;
        } else {
            std::ptr::copy_nonoverlapping(p as *const u8, v.as_mut_ptr(), n);
        }
        Ok(if double { f64::from_le_bytes(v) } else { f32::from_le_bytes(v[..4].try_into().unwrap()) as f64 })
    }

    fn arg<T>(v: &mut T) -> *mut c_void {
        v as *mut T as *mut c_void
    }

    unsafe fn run(h: &Handle, name: &'static std::ffi::CStr, params: &mut [*mut c_void], grid: [i64; 2], block: [u32; 2]) -> cusparseStatus_t {
        if grid[0] == 0 || grid[1] == 0 {
            return Ok(());
        }
        let groups = |n: i64, b: u32| u32::try_from((n as u64).div_ceil(b as u64)).map_err(|_| cusparseError_t::NOT_SUPPORTED);
        let grid = [groups(grid[0], block[0])?, groups(grid[1], block[1])?];
        let f = engine::function(agx_libs::SPARSE, name).map_err(status)?;
        engine::launch_kernel(f, grid, block, params, h.stream).map_err(status)
    }

    /// Runs `launches` on the matrix `a`, the input `x` (a vector or B) and the output `y` (a vector or C).
    unsafe fn execute(h: &Handle, launches: &[Launch], a: &SpMat, x: *const c_void, y: *const c_void, alpha: f64, beta: f64) -> cusparseStatus_t {
        let (mut pa, mut pc, mut pv, mut px, mut py) = (a.a, a.col, a.val, x, y);
        let (mut rows, mut nnz, mut base) = (a.rows, a.nnz, a.base);
        // the kernels take the scalars in the value type
        let (mut alpha32, mut beta32, mut alpha64, mut beta64) = (alpha as f32, beta as f32, alpha, beta);
        let (alpha, beta): (*mut c_void, *mut c_void) = if a.double { (arg(&mut alpha64), arg(&mut beta64)) } else { (arg(&mut alpha32), arg(&mut beta32)) };
        // index width, then the f32 kernel or its f64 copy
        let pick =
            |n32: &'static std::ffi::CStr, n64: &'static std::ffi::CStr, d32: &'static std::ffi::CStr, d64: &'static std::ffi::CStr| match (a.wide, a.double) {
                (false, false) => n32,
                (true, false) => n64,
                (false, true) => d32,
                (true, true) => d64,
            };
        for l in launches {
            match *l {
                Launch::Scale { mut n, mut stride } => run(
                    h,
                    if a.double { c"agx_sparse_scale_f64" } else { c"agx_sparse_scale" },
                    &mut [arg(&mut py), arg(&mut n), arg(&mut stride), beta],
                    [n, 1],
                    [64, 1],
                )?,
                Launch::CsrSpmv => run(
                    h,
                    pick(c"agx_csr_spmv_i32", c"agx_csr_spmv_i64", c"agx_csr_spmv_i32_f64", c"agx_csr_spmv_i64_f64"),
                    &mut [arg(&mut pa), arg(&mut pc), arg(&mut pv), arg(&mut px), arg(&mut py), arg(&mut rows), arg(&mut base), alpha, beta],
                    [rows, 1],
                    [64, 1],
                )?,
                Launch::CsrSpmvT => run(
                    h,
                    pick(c"agx_csr_spmv_t_i32", c"agx_csr_spmv_t_i64", c"agx_csr_spmv_t_i32_f64", c"agx_csr_spmv_t_i64_f64"),
                    &mut [arg(&mut pa), arg(&mut pc), arg(&mut pv), arg(&mut px), arg(&mut py), arg(&mut rows), arg(&mut base), alpha],
                    [rows, 1],
                    [64, 1],
                )?,
                Launch::CooSpmv { trans } => {
                    let mut trans = trans as i64;
                    run(
                        h,
                        pick(c"agx_coo_spmv_i32", c"agx_coo_spmv_i64", c"agx_coo_spmv_i32_f64", c"agx_coo_spmv_i64_f64"),
                        &mut [arg(&mut pa), arg(&mut pc), arg(&mut pv), arg(&mut px), arg(&mut py), arg(&mut nnz), arg(&mut base), arg(&mut trans), alpha],
                        [nnz, 1],
                        [64, 1],
                    )?
                }
                Launch::CsrSpmm { mut n, mut brs, mut bcs, mut crs, mut ccs } => run(
                    h,
                    pick(c"agx_csr_spmm_i32", c"agx_csr_spmm_i64", c"agx_csr_spmm_i32_f64", c"agx_csr_spmm_i64_f64"),
                    &mut [
                        arg(&mut pa),
                        arg(&mut pc),
                        arg(&mut pv),
                        arg(&mut px),
                        arg(&mut py),
                        arg(&mut rows),
                        arg(&mut n),
                        arg(&mut base),
                        arg(&mut brs),
                        arg(&mut bcs),
                        arg(&mut crs),
                        arg(&mut ccs),
                        alpha,
                        beta,
                    ],
                    [rows, n],
                    [16, 4],
                )?,
            }
        }
        Ok(())
    }

    pub(crate) unsafe fn cusparseSpMV_bufferSize(
        h: cusparseHandle_t,
        op: cusparseOperation_t,
        _alpha: *const c_void,
        a: cusparseConstSpMatDescr_t,
        x: cusparseConstDnVecDescr_t,
        _beta: *const c_void,
        y: cusparseDnVecDescr_t,
        compute: cudaDataType,
        _alg: cusparseSpMVAlg_t,
        size: *mut usize,
    ) -> cusparseStatus_t {
        obj::<Handle>(h)?;
        let (a, op) = obj::<SpMat>(a)?.csr(op);
        spmv_plan(op, &a, obj(x)?, obj(y)?, compute)?;
        out(size, 0)
    }

    pub(crate) unsafe fn cusparseSpMV(
        h: cusparseHandle_t,
        op: cusparseOperation_t,
        alpha: *const c_void,
        a: cusparseConstSpMatDescr_t,
        x: cusparseConstDnVecDescr_t,
        beta: *const c_void,
        y: cusparseDnVecDescr_t,
        compute: cudaDataType,
        _alg: cusparseSpMVAlg_t,
        _buffer: *mut c_void,
    ) -> cusparseStatus_t {
        let h = obj::<Handle>(h)?;
        let (a, x, y) = (obj::<SpMat>(a)?, obj::<DnVec>(x)?, obj::<DnVec>(y)?);
        let (a, op) = a.csr(op);
        let launches = spmv_plan(op, &a, x, y, compute)?;
        let (alpha, beta) = (scalar(h, alpha, a.double)?, scalar(h, beta, a.double)?);
        execute(h, &launches, &a, x.values, y.values, alpha, beta)
    }

    pub(crate) unsafe fn cusparseSpMM_bufferSize(
        h: cusparseHandle_t,
        op_a: cusparseOperation_t,
        op_b: cusparseOperation_t,
        _alpha: *const c_void,
        a: cusparseConstSpMatDescr_t,
        b: cusparseConstDnMatDescr_t,
        _beta: *const c_void,
        c: cusparseDnMatDescr_t,
        compute: cudaDataType,
        _alg: cusparseSpMMAlg_t,
        size: *mut usize,
    ) -> cusparseStatus_t {
        obj::<Handle>(h)?;
        let (a, op_a) = obj::<SpMat>(a)?.csr(op_a);
        spmm_plan(op_a, op_b, &a, obj(b)?, obj(c)?, compute)?;
        out(size, 0)
    }

    pub(crate) unsafe fn cusparseSpMM_preprocess(
        h: cusparseHandle_t,
        op_a: cusparseOperation_t,
        op_b: cusparseOperation_t,
        _alpha: *const c_void,
        a: cusparseConstSpMatDescr_t,
        b: cusparseConstDnMatDescr_t,
        _beta: *const c_void,
        c: cusparseDnMatDescr_t,
        compute: cudaDataType,
        _alg: cusparseSpMMAlg_t,
        _buffer: *mut c_void,
    ) -> cusparseStatus_t {
        obj::<Handle>(h)?;
        let (a, op_a) = obj::<SpMat>(a)?.csr(op_a);
        spmm_plan(op_a, op_b, &a, obj(b)?, obj(c)?, compute).map(|_| ())
    }

    pub(crate) unsafe fn cusparseSpMM(
        h: cusparseHandle_t,
        op_a: cusparseOperation_t,
        op_b: cusparseOperation_t,
        alpha: *const c_void,
        a: cusparseConstSpMatDescr_t,
        b: cusparseConstDnMatDescr_t,
        beta: *const c_void,
        c: cusparseDnMatDescr_t,
        compute: cudaDataType,
        _alg: cusparseSpMMAlg_t,
        _buffer: *mut c_void,
    ) -> cusparseStatus_t {
        let h = obj::<Handle>(h)?;
        let (a, b, c) = (obj::<SpMat>(a)?, obj::<DnMat>(b)?, obj::<DnMat>(c)?);
        let (a, op_a) = a.csr(op_a);
        let Some(launch) = spmm_plan(op_a, op_b, &a, b, c, compute)? else {
            return Ok(());
        };
        let (alpha, beta) = (scalar(h, alpha, a.double)?, scalar(h, beta, a.double)?);
        execute(h, &[launch], &a, b.values, c.values, alpha, beta)
    }

    // ---- the other generic API functions

    /// The kernel for the index width and value type: `[i32, i64, i32 f64, i64 f64]`.
    fn kernel(wide: bool, double: bool, names: [&'static std::ffi::CStr; 4]) -> &'static std::ffi::CStr {
        names[wide as usize + 2 * double as usize]
    }

    /// A scalar parameter of the value type.
    struct Val(f32, f64);

    impl Val {
        fn new(v: f64) -> Val {
            Val(v as f32, v)
        }

        fn arg(&mut self, double: bool) -> *mut c_void {
            if double {
                arg(&mut self.1)
            } else {
                arg(&mut self.0)
            }
        }
    }

    fn ptr(p: *const c_void) -> *mut c_void {
        p as *mut c_void
    }

    unsafe fn scale(h: &Handle, y: *const c_void, n: i64, double: bool, beta: f64) -> cusparseStatus_t {
        let (mut py, mut n, mut stride, mut beta) = (ptr(y), n, 1i64, Val::new(beta));
        run(
            h,
            if double { c"agx_sparse_scale_f64" } else { c"agx_sparse_scale" },
            &mut [arg(&mut py), arg(&mut n), arg(&mut stride), beta.arg(double)],
            [n, 1],
            [64, 1],
        )
    }

    pub(crate) unsafe fn cusparseCreateSpVec(
        d: *mut cusparseSpVecDescr_t,
        size: i64,
        nnz: i64,
        idx: *mut c_void,
        val: *mut c_void,
        idx_ty: cusparseIndexType_t,
        idx_base: cusparseIndexBase_t,
        ty: cudaDataType,
    ) -> cusparseStatus_t {
        new(d, sp_vec(size, nnz, idx, val, idx_ty, idx_base, ty)?)
    }

    pub(crate) unsafe fn cusparseCreateConstSpVec(
        d: *mut cusparseConstSpVecDescr_t,
        size: i64,
        nnz: i64,
        idx: *const c_void,
        val: *const c_void,
        idx_ty: cusparseIndexType_t,
        idx_base: cusparseIndexBase_t,
        ty: cudaDataType,
    ) -> cusparseStatus_t {
        new_const(d, sp_vec(size, nnz, idx, val, idx_ty, idx_base, ty)?)
    }

    fn sp_vec(
        size: i64,
        nnz: i64,
        idx: *const c_void,
        val: *const c_void,
        idx_ty: cusparseIndexType_t,
        idx_base: cusparseIndexBase_t,
        ty: cudaDataType,
    ) -> Result<SpVec, cusparseError_t> {
        if size < 0 || nnz < 0 || nnz > size || (nnz > 0 && (idx.is_null() || val.is_null())) {
            return Err(cusparseError_t::INVALID_VALUE);
        }
        Ok(SpVec { size, nnz, idx, val, wide: index(idx_ty)?, base: base(idx_base)?, double: value_type(ty)? })
    }

    pub(crate) unsafe fn cusparseDestroySpVec(d: cusparseConstSpVecDescr_t) -> cusparseStatus_t {
        destroy::<SpVec, _>(d)
    }

    /// x and y of one value type, of the same size.
    fn vec_pair(x: &SpVec, y: &DnVec, compute: Option<cudaDataType_t>) -> cusparseStatus_t {
        if value_type(y.ty)? != x.double || compute.map_or(Ok(x.double), value_type)? != x.double {
            return Err(cusparseError_t::NOT_SUPPORTED);
        }
        if x.size != y.size {
            return Err(cusparseError_t::INVALID_VALUE);
        }
        Ok(())
    }

    /// One of the element-wise operations of agx_spvec (op 0 axpy, 1 gather, 2 scatter, 3 rot).
    unsafe fn spvec(h: &Handle, x: &SpVec, y: &DnVec, op: i64, alpha: f64, beta: f64) -> cusparseStatus_t {
        let (mut pi, mut pv, mut py, mut nnz, mut base, mut op) = (ptr(x.idx), ptr(x.val), ptr(y.values), x.nnz, x.base, op);
        let (mut alpha, mut beta) = (Val::new(alpha), Val::new(beta));
        run(
            h,
            kernel(x.wide, x.double, [c"agx_spvec_i32", c"agx_spvec_i64", c"agx_spvec_i32_f64", c"agx_spvec_i64_f64"]),
            &mut [arg(&mut pi), arg(&mut pv), arg(&mut py), arg(&mut nnz), arg(&mut base), arg(&mut op), alpha.arg(x.double), beta.arg(x.double)],
            [x.nnz, 1],
            [64, 1],
        )
    }

    fn vv_check(op: cusparseOperation_t, x: &SpVec, y: &DnVec, compute: cudaDataType_t) -> cusparseStatus_t {
        match op {
            cusparseOperation_t::CUSPARSE_OPERATION_NON_TRANSPOSE | cusparseOperation_t::CUSPARSE_OPERATION_CONJUGATE_TRANSPOSE => (),
            _ => return Err(cusparseError_t::INVALID_VALUE),
        }
        vec_pair(x, y, Some(compute))
    }

    pub(crate) unsafe fn cusparseSpVV_bufferSize(
        h: cusparseHandle_t,
        op: cusparseOperation_t,
        x: cusparseConstSpVecDescr_t,
        y: cusparseConstDnVecDescr_t,
        _result: *const c_void,
        compute: cudaDataType,
        size: *mut usize,
    ) -> cusparseStatus_t {
        obj::<Handle>(h)?;
        vv_check(op, obj(x)?, obj(y)?, compute)?;
        out(size, (VV_THREADS as usize + 1) * 8)
    }

    pub(crate) unsafe fn cusparseSpVV(
        h: cusparseHandle_t,
        op: cusparseOperation_t,
        x: cusparseConstSpVecDescr_t,
        y: cusparseConstDnVecDescr_t,
        result: *mut c_void,
        compute: cudaDataType,
        buffer: *mut c_void,
    ) -> cusparseStatus_t {
        let h = obj::<Handle>(h)?;
        let (x, y) = (obj::<SpVec>(x)?, obj::<DnVec>(y)?);
        vv_check(op, x, y, compute)?;
        if buffer.is_null() || result.is_null() {
            return Err(cusparseError_t::INVALID_VALUE);
        }
        let slot = buffer.cast::<u8>().add(VV_THREADS as usize * 8).cast::<c_void>();
        let (mut pi, mut pv, mut py, mut part, mut nnz, mut base, mut threads) = (ptr(x.idx), ptr(x.val), ptr(y.values), buffer, x.nnz, x.base, VV_THREADS);
        run(
            h,
            kernel(x.wide, x.double, [c"agx_spvv_i32", c"agx_spvv_i64", c"agx_spvv_i32_f64", c"agx_spvv_i64_f64"]),
            &mut [arg(&mut pi), arg(&mut pv), arg(&mut py), arg(&mut part), arg(&mut nnz), arg(&mut base), arg(&mut threads)],
            [VV_THREADS, 1],
            [64, 1],
        )?;
        let mut dst = if h.device_scalars { result } else { slot };
        run(h, if x.double { c"agx_sparse_sum_f64" } else { c"agx_sparse_sum" }, &mut [arg(&mut part), arg(&mut dst), arg(&mut threads)], [1, 1], [1, 1])?;
        if !h.device_scalars {
            engine::synchronize(h.stream).map_err(status)?;
            engine::read(result, slot, if x.double { 8 } else { 4 }).map_err(status)?;
        }
        Ok(())
    }

    pub(crate) unsafe fn cusparseAxpby(
        h: cusparseHandle_t,
        alpha: *const c_void,
        x: cusparseConstSpVecDescr_t,
        beta: *const c_void,
        y: cusparseDnVecDescr_t,
    ) -> cusparseStatus_t {
        let h = obj::<Handle>(h)?;
        let (x, y) = (obj::<SpVec>(x)?, obj::<DnVec>(y)?);
        vec_pair(x, y, None)?;
        let (alpha, beta) = (scalar(h, alpha, x.double)?, scalar(h, beta, x.double)?);
        if beta != 1.0 {
            scale(h, y.values, y.size, x.double, beta)?;
        }
        spvec(h, x, y, 0, alpha, 0.0)
    }

    pub(crate) unsafe fn cusparseGather(h: cusparseHandle_t, y: cusparseConstDnVecDescr_t, x: cusparseSpVecDescr_t) -> cusparseStatus_t {
        let h = obj::<Handle>(h)?;
        let (x, y) = (obj::<SpVec>(x)?, obj::<DnVec>(y)?);
        vec_pair(x, y, None)?;
        spvec(h, x, y, 1, 0.0, 0.0)
    }

    pub(crate) unsafe fn cusparseScatter(h: cusparseHandle_t, x: cusparseConstSpVecDescr_t, y: cusparseDnVecDescr_t) -> cusparseStatus_t {
        let h = obj::<Handle>(h)?;
        let (x, y) = (obj::<SpVec>(x)?, obj::<DnVec>(y)?);
        vec_pair(x, y, None)?;
        spvec(h, x, y, 2, 0.0, 0.0)
    }

    pub(crate) unsafe fn cusparseRot(
        h: cusparseHandle_t,
        c: *const c_void,
        s: *const c_void,
        x: cusparseSpVecDescr_t,
        y: cusparseDnVecDescr_t,
    ) -> cusparseStatus_t {
        let h = obj::<Handle>(h)?;
        let (x, y) = (obj::<SpVec>(x)?, obj::<DnVec>(y)?);
        vec_pair(x, y, None)?;
        let (c, s) = (scalar(h, c, x.double)?, scalar(h, s, x.double)?);
        spvec(h, x, y, 3, c, s)
    }

    // ---- conversions

    /// A CSC matrix and a dense one as the CSR matrix of the transpose and the dense transpose.
    fn csr_dense(a: &SpMat, d: &DnMat) -> (SpMat, DnMat) {
        if a.format == Format::Csc {
            (a.csr(cusparseOperation_t::CUSPARSE_OPERATION_NON_TRANSPOSE).0, d.transposed())
        } else {
            (a.clone(), d.clone())
        }
    }

    fn same_shape(a: &SpMat, d: &DnMat) -> cusparseStatus_t {
        if value_type(d.ty)? != a.double {
            return Err(cusparseError_t::NOT_SUPPORTED);
        }
        if a.rows != d.rows || a.cols != d.cols {
            return Err(cusparseError_t::INVALID_VALUE);
        }
        Ok(())
    }

    pub(crate) unsafe fn cusparseSparseToDense_bufferSize(
        h: cusparseHandle_t,
        a: cusparseConstSpMatDescr_t,
        b: cusparseDnMatDescr_t,
        _alg: cusparseSparseToDenseAlg_t,
        size: *mut usize,
    ) -> cusparseStatus_t {
        obj::<Handle>(h)?;
        let (a, b) = csr_dense(obj(a)?, obj(b)?);
        same_shape(&a, &b)?;
        out(size, 0)
    }

    pub(crate) unsafe fn cusparseSparseToDense(
        h: cusparseHandle_t,
        a: cusparseConstSpMatDescr_t,
        b: cusparseDnMatDescr_t,
        _alg: cusparseSparseToDenseAlg_t,
        _buffer: *mut c_void,
    ) -> cusparseStatus_t {
        let h = obj::<Handle>(h)?;
        let (a, b) = csr_dense(obj::<SpMat>(a)?, obj::<DnMat>(b)?);
        same_shape(&a, &b)?;
        let (rs, cs) = b.strides();
        let (mut pd, mut rows, mut cols, mut rs, mut cs) = (ptr(b.values), b.rows, b.cols, rs, cs);
        run(
            h,
            if a.double { c"agx_dense_zero_f64" } else { c"agx_dense_zero" },
            &mut [arg(&mut pd), arg(&mut rows), arg(&mut cols), arg(&mut rs), arg(&mut cs)],
            [b.rows, b.cols],
            [16, 4],
        )?;
        let coo = a.format == Format::Coo;
        let (mut pa, mut pc, mut pv, mut n, mut base, mut coo_) = (ptr(a.a), ptr(a.col), ptr(a.val), if coo { a.nnz } else { a.rows }, a.base, coo as i64);
        run(
            h,
            kernel(a.wide, a.double, [c"agx_sp_to_dense_i32", c"agx_sp_to_dense_i64", c"agx_sp_to_dense_i32_f64", c"agx_sp_to_dense_i64_f64"]),
            &mut [arg(&mut pa), arg(&mut pc), arg(&mut pv), arg(&mut pd), arg(&mut n), arg(&mut base), arg(&mut coo_), arg(&mut rs), arg(&mut cs)],
            [n, 1],
            [64, 1],
        )
    }

    fn dense_to_csr_check(a: &DnMat, b: &SpMat) -> cusparseStatus_t {
        same_shape(b, a)?;
        if b.format != Format::Csr {
            return Err(cusparseError_t::NOT_SUPPORTED);
        }
        Ok(())
    }

    /// One pass of agx_dense_to_csr: the row counts (fill false) or the columns and values.
    unsafe fn dense_to_csr(h: &Handle, a: &DnMat, b: &SpMat, fill: bool) -> cusparseStatus_t {
        let (rs, cs) = a.strides();
        let (mut pd, mut pa, mut pc, mut pv) = (ptr(a.values), ptr(b.a), ptr(b.col), ptr(b.val));
        let (mut rows, mut cols, mut rs, mut cs, mut base, mut fill) = (a.rows, a.cols, rs, cs, b.base, fill as i64);
        run(
            h,
            kernel(b.wide, b.double, [c"agx_dense_to_csr_i32", c"agx_dense_to_csr_i64", c"agx_dense_to_csr_i32_f64", c"agx_dense_to_csr_i64_f64"]),
            &mut [
                arg(&mut pd),
                arg(&mut pa),
                arg(&mut pc),
                arg(&mut pv),
                arg(&mut rows),
                arg(&mut cols),
                arg(&mut rs),
                arg(&mut cs),
                arg(&mut base),
                arg(&mut fill),
            ],
            [a.rows, 1],
            [64, 1],
        )
    }

    pub(crate) unsafe fn cusparseDenseToSparse_bufferSize(
        h: cusparseHandle_t,
        a: cusparseConstDnMatDescr_t,
        b: cusparseSpMatDescr_t,
        _alg: cusparseDenseToSparseAlg_t,
        size: *mut usize,
    ) -> cusparseStatus_t {
        obj::<Handle>(h)?;
        let (b, a) = csr_dense(obj(b)?, obj(a)?);
        dense_to_csr_check(&a, &b)?;
        out(size, 0)
    }

    /// Writes the row offsets of B and sets its nnz (cusparseSpMatGetSize reads it).
    pub(crate) unsafe fn cusparseDenseToSparse_analysis(
        h: cusparseHandle_t,
        a: cusparseConstDnMatDescr_t,
        b: cusparseSpMatDescr_t,
        _alg: cusparseDenseToSparseAlg_t,
        _buffer: *mut c_void,
    ) -> cusparseStatus_t {
        let h = obj::<Handle>(h)?;
        let desc = obj::<SpMat>(b)?;
        let (b, a) = csr_dense(desc, obj::<DnMat>(a)?);
        let (a, b) = (&a, &b);
        dense_to_csr_check(a, b)?;
        if b.a.is_null() {
            return Err(cusparseError_t::INVALID_VALUE);
        }
        dense_to_csr(h, a, b, false)?;
        let (mut pa, mut rows, mut base) = (ptr(b.a), b.rows, b.base);
        run(h, if b.wide { c"agx_sparse_scan_i64" } else { c"agx_sparse_scan_i32" }, &mut [arg(&mut pa), arg(&mut rows), arg(&mut base)], [1, 1], [1, 1])?;
        engine::synchronize(h.stream).map_err(status)?;
        let width = if b.wide { 8 } else { 4 };
        let mut last = [0u8; 8];
        engine::read(last.as_mut_ptr() as *mut c_void, b.a.cast::<u8>().add(b.rows as usize * width).cast(), width).map_err(status)?;
        let end = if b.wide { i64::from_le_bytes(last) } else { i32::from_le_bytes(last[..4].try_into().unwrap()) as i64 };
        desc.nnz = end - b.base;
        Ok(())
    }

    pub(crate) unsafe fn cusparseDenseToSparse_convert(
        h: cusparseHandle_t,
        a: cusparseConstDnMatDescr_t,
        b: cusparseSpMatDescr_t,
        _alg: cusparseDenseToSparseAlg_t,
        _buffer: *mut c_void,
    ) -> cusparseStatus_t {
        let h = obj::<Handle>(h)?;
        let (b, a) = csr_dense(obj::<SpMat>(b)?, obj::<DnMat>(a)?);
        dense_to_csr_check(&a, &b)?;
        if b.a.is_null() || (b.nnz > 0 && (b.col.is_null() || b.val.is_null())) {
            return Err(cusparseError_t::INVALID_VALUE);
        }
        dense_to_csr(h, &a, &b, true)
    }

    unsafe fn set_pointers(d: cusparseSpMatDescr_t, format: Format, a: *mut c_void, col: *mut c_void, val: *mut c_void) -> cusparseStatus_t {
        let m = obj::<SpMat>(d)?;
        if m.format != format {
            return Err(cusparseError_t::INVALID_VALUE);
        }
        (m.a, m.col, m.val) = (a, col, val);
        Ok(())
    }

    pub(crate) unsafe fn cusparseCsrSetPointers(d: cusparseSpMatDescr_t, offsets: *mut c_void, col: *mut c_void, val: *mut c_void) -> cusparseStatus_t {
        set_pointers(d, Format::Csr, offsets, col, val)
    }

    pub(crate) unsafe fn cusparseCscSetPointers(d: cusparseSpMatDescr_t, offsets: *mut c_void, row: *mut c_void, val: *mut c_void) -> cusparseStatus_t {
        set_pointers(d, Format::Csc, offsets, row, val)
    }

    pub(crate) unsafe fn cusparseSpMatSetAttribute(
        d: cusparseSpMatDescr_t,
        attr: cusparseSpMatAttribute_t,
        data: *mut c_void,
        size: usize,
    ) -> cusparseStatus_t {
        let m = obj::<SpMat>(d)?;
        if data.is_null() || size != 4 {
            return Err(cusparseError_t::INVALID_VALUE);
        }
        let v = *(data as *const u32);
        match attr {
            cusparseSpMatAttribute_t::CUSPARSE_SPMAT_FILL_MODE => match cusparseFillMode_t(v) {
                cusparseFillMode_t::CUSPARSE_FILL_MODE_LOWER => m.upper = false,
                cusparseFillMode_t::CUSPARSE_FILL_MODE_UPPER => m.upper = true,
                _ => return Err(cusparseError_t::INVALID_VALUE),
            },
            cusparseSpMatAttribute_t::CUSPARSE_SPMAT_DIAG_TYPE => match cusparseDiagType_t(v) {
                cusparseDiagType_t::CUSPARSE_DIAG_TYPE_NON_UNIT => m.unit = false,
                cusparseDiagType_t::CUSPARSE_DIAG_TYPE_UNIT => m.unit = true,
                _ => return Err(cusparseError_t::INVALID_VALUE),
            },
            _ => return Err(cusparseError_t::INVALID_VALUE),
        }
        Ok(())
    }

    pub(crate) unsafe fn cusparseSpMatGetAttribute(
        d: cusparseConstSpMatDescr_t,
        attr: cusparseSpMatAttribute_t,
        data: *mut c_void,
        size: usize,
    ) -> cusparseStatus_t {
        let m = obj::<SpMat>(d)?;
        if size != 4 {
            return Err(cusparseError_t::INVALID_VALUE);
        }
        let v = match attr {
            cusparseSpMatAttribute_t::CUSPARSE_SPMAT_FILL_MODE => {
                if m.upper { cusparseFillMode_t::CUSPARSE_FILL_MODE_UPPER } else { cusparseFillMode_t::CUSPARSE_FILL_MODE_LOWER }.0
            }
            cusparseSpMatAttribute_t::CUSPARSE_SPMAT_DIAG_TYPE => {
                if m.unit { cusparseDiagType_t::CUSPARSE_DIAG_TYPE_UNIT } else { cusparseDiagType_t::CUSPARSE_DIAG_TYPE_NON_UNIT }.0
            }
            _ => return Err(cusparseError_t::INVALID_VALUE),
        };
        out(data as *mut u32, v)
    }

    fn csr2csc_check(m: c_int, n: c_int, nnz: c_int) -> cusparseStatus_t {
        if m < 0 || n < 0 || nnz < 0 {
            return Err(cusparseError_t::INVALID_VALUE);
        }
        Ok(())
    }

    pub(crate) unsafe fn cusparseCsr2cscEx2_bufferSize(
        h: cusparseHandle_t,
        m: c_int,
        n: c_int,
        nnz: c_int,
        _csr_val: *const c_void,
        _csr_rowptr: *const c_int,
        _csr_col: *const c_int,
        _csc_val: *mut c_void,
        _csc_colptr: *mut c_int,
        _csc_row: *mut c_int,
        ty: cudaDataType,
        action: cusparseAction_t,
        idx_base: cusparseIndexBase_t,
        _alg: cusparseCsr2CscAlg_t,
        size: *mut usize,
    ) -> cusparseStatus_t {
        obj::<Handle>(h)?;
        csr2csc_check(m, n, nnz)?;
        base(idx_base)?;
        if action == cusparseAction_t::CUSPARSE_ACTION_NUMERIC {
            permute_words(ty)?;
        }
        out(size, 4 * (n as usize + nnz as usize) + 8)
    }

    /// The columns are counted in the workspace (n words), then it holds the next free position of each column,
    /// followed by the CSR entry of each CSC entry (nnz words).
    pub(crate) unsafe fn cusparseCsr2cscEx2(
        h: cusparseHandle_t,
        m: c_int,
        n: c_int,
        nnz: c_int,
        csr_val: *const c_void,
        csr_rowptr: *const c_int,
        csr_col: *const c_int,
        csc_val: *mut c_void,
        csc_colptr: *mut c_int,
        csc_row: *mut c_int,
        ty: cudaDataType,
        action: cusparseAction_t,
        idx_base: cusparseIndexBase_t,
        _alg: cusparseCsr2CscAlg_t,
        buffer: *mut c_void,
    ) -> cusparseStatus_t {
        let h = obj::<Handle>(h)?;
        csr2csc_check(m, n, nnz)?;
        let numeric = match action {
            cusparseAction_t::CUSPARSE_ACTION_NUMERIC => true,
            cusparseAction_t::CUSPARSE_ACTION_SYMBOLIC => false,
            _ => return Err(cusparseError_t::INVALID_VALUE),
        };
        let words = if numeric { Some(permute_words(ty)?) } else { None };
        let mut base = base(idx_base)?;
        if buffer.is_null() || csc_colptr.is_null() || (m > 0 && csr_rowptr.is_null()) || (nnz > 0 && (csr_col.is_null() || csc_row.is_null())) {
            return Err(cusparseError_t::INVALID_VALUE);
        }
        let (mut rows, mut cols, mut count) = (m as i64, n as i64, nnz as i64);
        let (mut rp, mut ci, mut cnt, mut ptr_, mut ri) = (ptr(csr_rowptr.cast()), ptr(csr_col.cast()), buffer, ptr(csc_colptr.cast()), ptr(csc_row.cast()));
        let mut perm = buffer.cast::<u8>().add(4 * n as usize).cast::<c_void>();
        scale(h, cnt, cols, false, 0.0)?;
        if count > 0 {
            run(h, c"agx_csr2csc_count", &mut [arg(&mut rp), arg(&mut ci), arg(&mut cnt), arg(&mut rows), arg(&mut base)], [rows, 1], [64, 1])?;
        }
        run(h, c"agx_csr2csc_scan", &mut [arg(&mut cnt), arg(&mut ptr_), arg(&mut cols), arg(&mut base)], [1, 1], [1, 1])?;
        if count == 0 {
            return Ok(());
        }
        run(
            h,
            c"agx_csr2csc_fill",
            &mut [arg(&mut rp), arg(&mut ci), arg(&mut cnt), arg(&mut ri), arg(&mut perm), arg(&mut rows), arg(&mut base)],
            [rows, 1],
            [64, 1],
        )?;
        run(h, c"agx_csr2csc_sort", &mut [arg(&mut ptr_), arg(&mut ri), arg(&mut perm), arg(&mut cols), arg(&mut base)], [cols, 1], [64, 1])?;
        if let Some((mut words, double)) = words {
            if csr_val.is_null() || csc_val.is_null() {
                return Err(cusparseError_t::INVALID_VALUE);
            }
            let (mut src, mut dst) = (ptr(csr_val), csc_val);
            run(
                h,
                if double { c"agx_sparse_permute_f64" } else { c"agx_sparse_permute" },
                &mut [arg(&mut src), arg(&mut dst), arg(&mut perm), arg(&mut count), arg(&mut words)],
                [count, 1],
                [64, 1],
            )?;
        }
        Ok(())
    }

    // ---- SpGEMM

    /// The state between the SpGEMM steps: the products of each row (in the first buffer) and the sorted products
    /// (in the second).
    struct SpGemm {
        products: i64,
        buffer1: *mut c_void,
        buffer2: *mut c_void,
    }

    pub(crate) unsafe fn cusparseSpGEMM_createDescr(d: *mut cusparseSpGEMMDescr_t) -> cusparseStatus_t {
        new(d, SpGemm { products: 0, buffer1: std::ptr::null_mut(), buffer2: std::ptr::null_mut() })
    }

    pub(crate) unsafe fn cusparseSpGEMM_destroyDescr(d: cusparseSpGEMMDescr_t) -> cusparseStatus_t {
        destroy::<SpGemm, _>(d)
    }

    pub(crate) unsafe fn cusparseSpGEMM_getNumProducts(d: cusparseSpGEMMDescr_t, n: *mut i64) -> cusparseStatus_t {
        out(n, obj::<SpGemm>(d)?.products)
    }

    /// C = alpha A B with CSR matrices of one index width, base and value type, and beta 0.
    unsafe fn spgemm_check(
        h: &Handle,
        op_a: cusparseOperation_t,
        op_b: cusparseOperation_t,
        a: &SpMat,
        b: &SpMat,
        beta: *const c_void,
        c: &SpMat,
        compute: cudaDataType_t,
    ) -> cusparseStatus_t {
        if transposes(op_a)? || transposes(op_b)? || a.format != Format::Csr || b.format != Format::Csr || c.format != Format::Csr {
            return Err(cusparseError_t::NOT_SUPPORTED);
        }
        if (b.wide, b.base, b.double) != (a.wide, a.base, a.double) || (c.wide, c.base, c.double) != (a.wide, a.base, a.double) {
            return Err(cusparseError_t::NOT_SUPPORTED);
        }
        if value_type(compute)? != a.double || scalar(h, beta, a.double)? != 0.0 {
            return Err(cusparseError_t::NOT_SUPPORTED);
        }
        if a.cols != b.rows || c.rows != a.rows || c.cols != b.cols {
            return Err(cusparseError_t::INVALID_VALUE);
        }
        Ok(())
    }

    /// The last entry of an i64 array in device memory, once the stream is done.
    unsafe fn last_i64(h: &Handle, p: *const c_void, n: i64) -> Result<i64, cusparseError_t> {
        engine::synchronize(h.stream).map_err(status)?;
        let mut v = 0i64;
        engine::read(arg(&mut v), p.cast::<u8>().add(n as usize * 8).cast(), 8).map_err(status)?;
        Ok(v)
    }

    unsafe fn scan_i64(h: &Handle, p: *mut c_void, n: i64) -> cusparseStatus_t {
        let (mut p, mut n, mut base) = (p, n, 0i64);
        run(h, c"agx_sparse_scan_i64", &mut [arg(&mut p), arg(&mut n), arg(&mut base)], [1, 1], [1, 1])
    }

    pub(crate) unsafe fn cusparseSpGEMM_workEstimation(
        h: cusparseHandle_t,
        op_a: cusparseOperation_t,
        op_b: cusparseOperation_t,
        _alpha: *const c_void,
        a: cusparseConstSpMatDescr_t,
        b: cusparseConstSpMatDescr_t,
        beta: *const c_void,
        c: cusparseSpMatDescr_t,
        compute: cudaDataType,
        _alg: cusparseSpGEMMAlg_t,
        d: cusparseSpGEMMDescr_t,
        size1: *mut usize,
        buffer1: *mut c_void,
    ) -> cusparseStatus_t {
        let h = obj::<Handle>(h)?;
        let (a, b, c, d) = (obj::<SpMat>(a)?, obj::<SpMat>(b)?, obj::<SpMat>(c)?, obj::<SpGemm>(d)?);
        spgemm_check(h, op_a, op_b, a, b, beta, c, compute)?;
        if buffer1.is_null() {
            return out(size1, 8 * (a.rows as usize + 1));
        }
        let (mut arp, mut acol, mut brp, mut ub, mut rows, mut base) = (ptr(a.a), ptr(a.col), ptr(b.a), buffer1, a.rows, a.base);
        run(
            h,
            if a.wide { c"agx_spgemm_ub_i64" } else { c"agx_spgemm_ub_i32" },
            &mut [arg(&mut arp), arg(&mut acol), arg(&mut brp), arg(&mut ub), arg(&mut rows), arg(&mut base)],
            [a.rows, 1],
            [64, 1],
        )?;
        scan_i64(h, buffer1, a.rows)?;
        d.products = last_i64(h, buffer1, a.rows)?;
        d.buffer1 = buffer1;
        Ok(())
    }

    /// The second buffer: the products' columns, the row counts (rows + 1) and the products' values.
    fn spgemm_layout(products: i64, rows: i64, double: bool) -> (usize, usize, usize) {
        let (p, r) = (products as usize, rows as usize + 1);
        (8 * p, 8 * (p + r), 8 * (p + r) + p * if double { 8 } else { 4 })
    }

    pub(crate) unsafe fn cusparseSpGEMM_compute(
        h: cusparseHandle_t,
        op_a: cusparseOperation_t,
        op_b: cusparseOperation_t,
        alpha: *const c_void,
        a: cusparseConstSpMatDescr_t,
        b: cusparseConstSpMatDescr_t,
        beta: *const c_void,
        c: cusparseSpMatDescr_t,
        compute: cudaDataType,
        _alg: cusparseSpGEMMAlg_t,
        d: cusparseSpGEMMDescr_t,
        size2: *mut usize,
        buffer2: *mut c_void,
    ) -> cusparseStatus_t {
        let h = obj::<Handle>(h)?;
        let (a, b, c, d) = (obj::<SpMat>(a)?, obj::<SpMat>(b)?, obj::<SpMat>(c)?, obj::<SpGemm>(d)?);
        spgemm_check(h, op_a, op_b, a, b, beta, c, compute)?;
        if d.buffer1.is_null() {
            return Err(cusparseError_t::INVALID_VALUE);
        }
        let (cnt_at, val_at, size) = spgemm_layout(d.products, a.rows, a.double);
        if buffer2.is_null() {
            return out(size2, size);
        }
        let at = |o: usize| buffer2.cast::<u8>().add(o).cast::<c_void>();
        let (mut arp, mut acol, mut aval, mut brp, mut bcol, mut bval) = (ptr(a.a), ptr(a.col), ptr(a.val), ptr(b.a), ptr(b.col), ptr(b.val));
        let (mut ub, mut tcol, mut tval, mut cnt, mut rows, mut base) = (d.buffer1, buffer2, at(val_at), at(cnt_at), a.rows, a.base);
        let mut alpha = Val::new(scalar(h, alpha, a.double)?);
        run(
            h,
            kernel(a.wide, a.double, [c"agx_spgemm_row_i32", c"agx_spgemm_row_i64", c"agx_spgemm_row_i32_f64", c"agx_spgemm_row_i64_f64"]),
            &mut [
                arg(&mut arp),
                arg(&mut acol),
                arg(&mut aval),
                arg(&mut brp),
                arg(&mut bcol),
                arg(&mut bval),
                arg(&mut ub),
                arg(&mut tcol),
                arg(&mut tval),
                arg(&mut cnt),
                arg(&mut rows),
                arg(&mut base),
                alpha.arg(a.double),
            ],
            [a.rows, 1],
            [64, 1],
        )?;
        scan_i64(h, cnt, a.rows)?;
        c.nnz = last_i64(h, cnt, a.rows)?;
        d.buffer2 = buffer2;
        Ok(())
    }

    pub(crate) unsafe fn cusparseSpGEMM_copy(
        h: cusparseHandle_t,
        op_a: cusparseOperation_t,
        op_b: cusparseOperation_t,
        _alpha: *const c_void,
        a: cusparseConstSpMatDescr_t,
        b: cusparseConstSpMatDescr_t,
        beta: *const c_void,
        c: cusparseSpMatDescr_t,
        compute: cudaDataType,
        _alg: cusparseSpGEMMAlg_t,
        d: cusparseSpGEMMDescr_t,
    ) -> cusparseStatus_t {
        let h = obj::<Handle>(h)?;
        let (a, b, c, d) = (obj::<SpMat>(a)?, obj::<SpMat>(b)?, obj::<SpMat>(c)?, obj::<SpGemm>(d)?);
        spgemm_check(h, op_a, op_b, a, b, beta, c, compute)?;
        if d.buffer2.is_null() || c.a.is_null() || (c.nnz > 0 && (c.col.is_null() || c.val.is_null())) {
            return Err(cusparseError_t::INVALID_VALUE);
        }
        let (cnt_at, val_at, _) = spgemm_layout(d.products, a.rows, a.double);
        let at = |o: usize| d.buffer2.cast::<u8>().add(o).cast::<c_void>();
        let (mut ub, mut tcol, mut tval, mut cnt) = (d.buffer1, d.buffer2, at(val_at), at(cnt_at));
        let (mut crp, mut ccol, mut cval, mut rows, mut base) = (ptr(c.a), ptr(c.col), ptr(c.val), c.rows, c.base);
        if c.rows == 0 {
            // only the first row offset
            let mut zero = [0u8; 8];
            zero[..8].copy_from_slice(&c.base.to_le_bytes());
            return engine::write(crp, zero.as_ptr().cast(), if c.wide { 8 } else { 4 }).map_err(status);
        }
        run(
            h,
            kernel(a.wide, a.double, [c"agx_spgemm_copy_i32", c"agx_spgemm_copy_i64", c"agx_spgemm_copy_i32_f64", c"agx_spgemm_copy_i64_f64"]),
            &mut [arg(&mut ub), arg(&mut tcol), arg(&mut tval), arg(&mut cnt), arg(&mut crp), arg(&mut ccol), arg(&mut cval), arg(&mut rows), arg(&mut base)],
            [c.rows, 1],
            [64, 1],
        )
    }

    // ---- SDDMM

    /// C in CSR with the dense operands in the order the kernel takes them: a CSC C is the CSR matrix of
    /// C^T = op(B)^T op(A)^T.
    fn sddmm_args<'a>(
        op_a: cusparseOperation_t,
        op_b: cusparseOperation_t,
        a: &'a DnMat,
        b: &'a DnMat,
        c: &SpMat,
        compute: cudaDataType_t,
    ) -> Result<(SpMat, &'a DnMat, &'a DnMat, [i64; 5]), cusparseError_t> {
        if c.format == Format::Csc {
            let (ct, _) = c.csr(op_a);
            let plan = sddmm_plan(flip(op_b), flip(op_a), b, a, &ct, compute)?;
            Ok((ct, b, a, plan))
        } else {
            Ok((c.clone(), a, b, sddmm_plan(op_a, op_b, a, b, c, compute)?))
        }
    }

    pub(crate) unsafe fn cusparseSDDMM_bufferSize(
        h: cusparseHandle_t,
        op_a: cusparseOperation_t,
        op_b: cusparseOperation_t,
        _alpha: *const c_void,
        a: cusparseConstDnMatDescr_t,
        b: cusparseConstDnMatDescr_t,
        _beta: *const c_void,
        c: cusparseSpMatDescr_t,
        compute: cudaDataType,
        _alg: cusparseSDDMMAlg_t,
        size: *mut usize,
    ) -> cusparseStatus_t {
        obj::<Handle>(h)?;
        sddmm_args(op_a, op_b, obj(a)?, obj(b)?, obj(c)?, compute)?;
        out(size, 0)
    }

    pub(crate) unsafe fn cusparseSDDMM_preprocess(
        h: cusparseHandle_t,
        op_a: cusparseOperation_t,
        op_b: cusparseOperation_t,
        _alpha: *const c_void,
        a: cusparseConstDnMatDescr_t,
        b: cusparseConstDnMatDescr_t,
        _beta: *const c_void,
        c: cusparseSpMatDescr_t,
        compute: cudaDataType,
        _alg: cusparseSDDMMAlg_t,
        _buffer: *mut c_void,
    ) -> cusparseStatus_t {
        obj::<Handle>(h)?;
        sddmm_args(op_a, op_b, obj(a)?, obj(b)?, obj(c)?, compute).map(|_| ())
    }

    pub(crate) unsafe fn cusparseSDDMM(
        h: cusparseHandle_t,
        op_a: cusparseOperation_t,
        op_b: cusparseOperation_t,
        alpha: *const c_void,
        a: cusparseConstDnMatDescr_t,
        b: cusparseConstDnMatDescr_t,
        beta: *const c_void,
        c: cusparseSpMatDescr_t,
        compute: cudaDataType,
        _alg: cusparseSDDMMAlg_t,
        _buffer: *mut c_void,
    ) -> cusparseStatus_t {
        let h = obj::<Handle>(h)?;
        let (a, b, c) = (obj::<DnMat>(a)?, obj::<DnMat>(b)?, obj::<SpMat>(c)?);
        let (c, a, b, [mut k, mut ars, mut acs, mut brs, mut bcs]) = sddmm_args(op_a, op_b, a, b, c, compute)?;
        let (mut alpha, mut beta) = (Val::new(scalar(h, alpha, c.double)?), Val::new(scalar(h, beta, c.double)?));
        let (mut pr, mut pc, mut pv, mut pa, mut pb, mut rows, mut base) = (ptr(c.a), ptr(c.col), ptr(c.val), ptr(a.values), ptr(b.values), c.rows, c.base);
        run(
            h,
            kernel(c.wide, c.double, [c"agx_sddmm_i32", c"agx_sddmm_i64", c"agx_sddmm_i32_f64", c"agx_sddmm_i64_f64"]),
            &mut [
                arg(&mut pr),
                arg(&mut pc),
                arg(&mut pv),
                arg(&mut pa),
                arg(&mut pb),
                arg(&mut rows),
                arg(&mut k),
                arg(&mut base),
                arg(&mut ars),
                arg(&mut acs),
                arg(&mut brs),
                arg(&mut bcs),
                alpha.arg(c.double),
                beta.arg(c.double),
            ],
            [c.rows, 1],
            [64, 1],
        )
    }

    // ---- SpSV and SpSM (one thread per right-hand side)

    /// The (empty) SpSV and SpSM descriptors.
    struct Solve;

    pub(crate) unsafe fn cusparseSpSV_createDescr(d: *mut cusparseSpSVDescr_t) -> cusparseStatus_t {
        new(d, Solve)
    }

    pub(crate) unsafe fn cusparseSpSV_destroyDescr(d: cusparseSpSVDescr_t) -> cusparseStatus_t {
        destroy::<Solve, _>(d)
    }

    pub(crate) unsafe fn cusparseSpSM_createDescr(d: *mut cusparseSpSMDescr_t) -> cusparseStatus_t {
        new(d, Solve)
    }

    pub(crate) unsafe fn cusparseSpSM_destroyDescr(d: cusparseSpSMDescr_t) -> cusparseStatus_t {
        destroy::<Solve, _>(d)
    }

    fn vec_dims(v: &DnVec) -> (i64, i64, i64, i64) {
        (v.size, 1, 1, 1)
    }

    /// The matrix in CSR, whether it is transposed and the strides of op(B) and C.
    type Solve4 = (SpMat, bool, [i64; 4]);

    fn spsv_check(op: cusparseOperation_t, a: &SpMat, x: &DnVec, y: &DnVec, compute: cudaDataType_t) -> Result<Solve4, cusparseError_t> {
        same_type(a, &[x.ty, y.ty, compute])?;
        let (a, op) = a.csr(op);
        let (trans, strides) = spsm_plan(op, &a, vec_dims(x), vec_dims(y))?;
        Ok((a, trans, strides))
    }

    fn spsm_check(
        op_a: cusparseOperation_t,
        op_b: cusparseOperation_t,
        a: &SpMat,
        b: &DnMat,
        c: &DnMat,
        compute: cudaDataType_t,
    ) -> Result<Solve4, cusparseError_t> {
        same_type(a, &[b.ty, c.ty, compute])?;
        let (a, op_a) = a.csr(op_a);
        let (trans, strides) = spsm_plan(op_a, &a, op_dims(op_b, b)?, op_dims(cusparseOperation_t::CUSPARSE_OPERATION_NON_TRANSPOSE, c)?)?;
        Ok((a, trans, strides))
    }

    unsafe fn spsm(h: &Handle, a: &SpMat, b: *const c_void, c: *const c_void, n: i64, trans: bool, strides: [i64; 4], alpha: f64) -> cusparseStatus_t {
        let (mut pr, mut pc, mut pv, mut pb, mut pcc) = (ptr(a.a), ptr(a.col), ptr(a.val), ptr(b), ptr(c));
        let (mut rows, mut n_, mut base) = (a.rows, n, a.base);
        let [mut brs, mut bcs, mut crs, mut ccs] = strides;
        let (mut upper, mut unit, mut trans, mut alpha) = (a.upper as i64, a.unit as i64, trans as i64, Val::new(alpha));
        run(
            h,
            kernel(a.wide, a.double, [c"agx_spsm_i32", c"agx_spsm_i64", c"agx_spsm_i32_f64", c"agx_spsm_i64_f64"]),
            &mut [
                arg(&mut pr),
                arg(&mut pc),
                arg(&mut pv),
                arg(&mut pb),
                arg(&mut pcc),
                arg(&mut rows),
                arg(&mut n_),
                arg(&mut base),
                arg(&mut brs),
                arg(&mut bcs),
                arg(&mut crs),
                arg(&mut ccs),
                arg(&mut upper),
                arg(&mut unit),
                arg(&mut trans),
                alpha.arg(a.double),
            ],
            [n, 1],
            [32, 1],
        )
    }

    pub(crate) unsafe fn cusparseSpSV_bufferSize(
        h: cusparseHandle_t,
        op: cusparseOperation_t,
        _alpha: *const c_void,
        a: cusparseConstSpMatDescr_t,
        x: cusparseConstDnVecDescr_t,
        y: cusparseDnVecDescr_t,
        compute: cudaDataType,
        _alg: cusparseSpSVAlg_t,
        _d: cusparseSpSVDescr_t,
        size: *mut usize,
    ) -> cusparseStatus_t {
        obj::<Handle>(h)?;
        spsv_check(op, obj(a)?, obj(x)?, obj(y)?, compute)?;
        out(size, 0)
    }

    pub(crate) unsafe fn cusparseSpSV_analysis(
        h: cusparseHandle_t,
        op: cusparseOperation_t,
        _alpha: *const c_void,
        a: cusparseConstSpMatDescr_t,
        x: cusparseConstDnVecDescr_t,
        y: cusparseDnVecDescr_t,
        compute: cudaDataType,
        _alg: cusparseSpSVAlg_t,
        d: cusparseSpSVDescr_t,
        _buffer: *mut c_void,
    ) -> cusparseStatus_t {
        obj::<Handle>(h)?;
        obj::<Solve>(d)?;
        spsv_check(op, obj(a)?, obj(x)?, obj(y)?, compute).map(|_| ())
    }

    pub(crate) unsafe fn cusparseSpSV_solve(
        h: cusparseHandle_t,
        op: cusparseOperation_t,
        alpha: *const c_void,
        a: cusparseConstSpMatDescr_t,
        x: cusparseConstDnVecDescr_t,
        y: cusparseDnVecDescr_t,
        compute: cudaDataType,
        _alg: cusparseSpSVAlg_t,
        d: cusparseSpSVDescr_t,
    ) -> cusparseStatus_t {
        let h = obj::<Handle>(h)?;
        obj::<Solve>(d)?;
        let (a, x, y) = (obj::<SpMat>(a)?, obj::<DnVec>(x)?, obj::<DnVec>(y)?);
        let (a, trans, strides) = spsv_check(op, a, x, y, compute)?;
        let alpha = scalar(h, alpha, a.double)?;
        spsm(h, &a, x.values, y.values, 1, trans, strides, alpha)
    }

    pub(crate) unsafe fn cusparseSpSM_bufferSize(
        h: cusparseHandle_t,
        op_a: cusparseOperation_t,
        op_b: cusparseOperation_t,
        _alpha: *const c_void,
        a: cusparseConstSpMatDescr_t,
        b: cusparseConstDnMatDescr_t,
        c: cusparseDnMatDescr_t,
        compute: cudaDataType,
        _alg: cusparseSpSMAlg_t,
        _d: cusparseSpSMDescr_t,
        size: *mut usize,
    ) -> cusparseStatus_t {
        obj::<Handle>(h)?;
        spsm_check(op_a, op_b, obj(a)?, obj(b)?, obj(c)?, compute)?;
        out(size, 0)
    }

    pub(crate) unsafe fn cusparseSpSM_analysis(
        h: cusparseHandle_t,
        op_a: cusparseOperation_t,
        op_b: cusparseOperation_t,
        _alpha: *const c_void,
        a: cusparseConstSpMatDescr_t,
        b: cusparseConstDnMatDescr_t,
        c: cusparseDnMatDescr_t,
        compute: cudaDataType,
        _alg: cusparseSpSMAlg_t,
        d: cusparseSpSMDescr_t,
        _buffer: *mut c_void,
    ) -> cusparseStatus_t {
        obj::<Handle>(h)?;
        obj::<Solve>(d)?;
        spsm_check(op_a, op_b, obj(a)?, obj(b)?, obj(c)?, compute).map(|_| ())
    }

    pub(crate) unsafe fn cusparseSpSM_solve(
        h: cusparseHandle_t,
        op_a: cusparseOperation_t,
        op_b: cusparseOperation_t,
        alpha: *const c_void,
        a: cusparseConstSpMatDescr_t,
        b: cusparseConstDnMatDescr_t,
        c: cusparseDnMatDescr_t,
        compute: cudaDataType,
        _alg: cusparseSpSMAlg_t,
        d: cusparseSpSMDescr_t,
    ) -> cusparseStatus_t {
        let h = obj::<Handle>(h)?;
        obj::<Solve>(d)?;
        let (a, b, c) = (obj::<SpMat>(a)?, obj::<DnMat>(b)?, obj::<DnMat>(c)?);
        let (a, trans, strides) = spsm_check(op_a, op_b, a, b, c, compute)?;
        let alpha = scalar(h, alpha, a.double)?;
        spsm(h, &a, b.values, c.values, c.cols, trans, strides, alpha)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const N: cusparseOperation_t = cusparseOperation_t::CUSPARSE_OPERATION_NON_TRANSPOSE;
    const T: cusparseOperation_t = cusparseOperation_t::CUSPARSE_OPERATION_TRANSPOSE;
    const F32: cudaDataType_t = cudaDataType_t::CUDA_R_32F;

    /// A host matrix: CSR offsets, COO rows, columns and values (zero-based).
    struct Host {
        rows: usize,
        cols: usize,
        rowptr: Vec<i64>,
        row: Vec<i64>,
        col: Vec<i64>,
        val: Vec<f32>,
    }

    fn matrix(rows: usize, cols: usize, seed: u64) -> Host {
        let mut s = seed;
        let mut next = || {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            (s >> 33) as u32
        };
        let (mut rowptr, mut row, mut col, mut val) = (vec![0], vec![], vec![], vec![]);
        for i in 0..rows {
            for j in 0..cols {
                if next() % 3 == 0 {
                    row.push(i as i64);
                    col.push(j as i64);
                    val.push((next() % 17) as f32 - 8.0);
                }
            }
            rowptr.push(col.len() as i64);
        }
        Host { rows, cols, rowptr, row, col, val }
    }

    fn sp(h: &Host, format: Format) -> SpMat {
        SpMat {
            format,
            rows: h.rows as i64,
            cols: h.cols as i64,
            nnz: h.col.len() as i64,
            a: std::ptr::null(),
            col: std::ptr::null(),
            val: std::ptr::null(),
            wide: false,
            base: 0,
            double: false,
            upper: false,
            unit: false,
        }
    }

    fn vec_of(size: usize) -> DnVec {
        DnVec { size: size as i64, values: std::ptr::null(), ty: F32 }
    }

    /// The kernels' semantics (sparse.mlir) on the host.
    fn emulate(launches: &[Launch], a: &Host, x: &[f32], y: &mut [f32], alpha: f32, beta: f32) {
        for l in launches {
            match *l {
                Launch::Scale { n, stride } => {
                    for i in 0..n as usize {
                        let o = i * stride as usize;
                        y[o] = if beta == 0.0 { 0.0 } else { y[o] * beta };
                    }
                }
                Launch::CsrSpmv => {
                    for i in 0..a.rows {
                        let s: f32 = (a.rowptr[i]..a.rowptr[i + 1]).map(|k| a.val[k as usize] * x[a.col[k as usize] as usize]).sum();
                        y[i] = alpha * s + if beta == 0.0 { 0.0 } else { beta * y[i] };
                    }
                }
                Launch::CsrSpmvT | Launch::CooSpmv { trans: true } => {
                    for k in 0..a.col.len() {
                        y[a.col[k] as usize] += alpha * a.val[k] * x[a.row[k] as usize];
                    }
                }
                Launch::CooSpmv { trans: false } => {
                    for k in 0..a.col.len() {
                        y[a.row[k] as usize] += alpha * a.val[k] * x[a.col[k] as usize];
                    }
                }
                Launch::CsrSpmm { n, brs, bcs, crs, ccs } => {
                    for i in 0..a.rows {
                        for j in 0..n as usize {
                            let s: f32 = (a.rowptr[i]..a.rowptr[i + 1])
                                .map(|k| a.val[k as usize] * x[a.col[k as usize] as usize * brs as usize + j * bcs as usize])
                                .sum();
                            let o = i * crs as usize + j * ccs as usize;
                            y[o] = alpha * s + if beta == 0.0 { 0.0 } else { beta * y[o] };
                        }
                    }
                }
            }
        }
    }

    fn dense(a: &Host) -> Vec<Vec<f32>> {
        let mut d = vec![vec![0.0; a.cols]; a.rows];
        for k in 0..a.col.len() {
            d[a.row[k] as usize][a.col[k] as usize] = a.val[k];
        }
        d
    }

    #[test]
    fn spmv_matches_dense() {
        let a = matrix(7, 5, 3);
        let d = dense(&a);
        for format in [Format::Csr, Format::Coo] {
            for (op, trans) in [(N, false), (T, true)] {
                for beta in [0.0, 0.5] {
                    let (xn, yn) = if trans { (a.rows, a.cols) } else { (a.cols, a.rows) };
                    let x: Vec<f32> = (0..xn).map(|i| i as f32 - 2.0).collect();
                    let mut y: Vec<f32> = (0..yn).map(|i| 1.0 + i as f32).collect();
                    let want: Vec<f32> = (0..yn)
                        .map(|i| {
                            let s: f32 = (0..xn).map(|j| if trans { d[j][i] } else { d[i][j] } * x[j]).sum();
                            1.5 * s + beta * y[i]
                        })
                        .collect();
                    let plan = spmv_plan(op, &sp(&a, format), &vec_of(xn), &vec_of(yn), F32).unwrap();
                    emulate(&plan, &a, &x, &mut y, 1.5, beta);
                    assert_eq!(y, want, "{format:?} trans {trans} beta {beta}");
                }
            }
        }
    }

    #[test]
    fn spmv_checks_shapes_and_types() {
        let a = sp(&matrix(4, 3, 1), Format::Csr);
        assert_eq!(spmv_plan(N, &a, &vec_of(4), &vec_of(4), F32), Err(cusparseError_t::INVALID_VALUE));
        assert_eq!(spmv_plan(T, &a, &vec_of(4), &vec_of(3), F32).map(|p| p.len()), Ok(2));
        let mut x = vec_of(3);
        x.ty = cudaDataType_t::CUDA_R_64F;
        assert_eq!(spmv_plan(N, &a, &x, &vec_of(4), F32), Err(cusparseError_t::NOT_SUPPORTED));
        let empty = sp(&matrix(0, 3, 1), Format::Csr);
        assert_eq!(spmv_plan(N, &empty, &vec_of(3), &vec_of(0), F32), Ok(vec![]));
        // f64 values with f64 vectors and compute type
        let d = SpMat { double: true, ..a.clone() };
        let f64v = |n| DnVec { ty: cudaDataType_t::CUDA_R_64F, ..vec_of(n) };
        assert_eq!(spmv_plan(N, &d, &f64v(3), &f64v(4), cudaDataType_t::CUDA_R_64F), Ok(vec![Launch::CsrSpmv]));
        assert_eq!(spmv_plan(N, &d, &f64v(3), &f64v(4), F32), Err(cusparseError_t::NOT_SUPPORTED));
        assert_eq!(spmv_plan(N, &d, &vec_of(3), &f64v(4), cudaDataType_t::CUDA_R_64F), Err(cusparseError_t::NOT_SUPPORTED));
    }

    #[test]
    fn spmm_matches_dense() {
        let a = matrix(6, 4, 9);
        let d = dense(&a);
        let n = 3;
        for tb in [false, true] {
            for row_major in [false, true] {
                // op(B) is cols x n; B is stored transposed for tb
                let (br, bc) = if tb { (n, a.cols) } else { (a.cols, n) };
                let bld = if row_major { bc + 1 } else { br + 2 };
                let cld = if row_major { n + 2 } else { a.rows + 1 };
                let b = DnMat { rows: br as i64, cols: bc as i64, ld: bld as i64, values: std::ptr::null(), ty: F32, row_major };
                let c = DnMat { rows: a.rows as i64, cols: n as i64, ld: cld as i64, values: std::ptr::null(), ty: F32, row_major };
                let bdata: Vec<f32> = (0..bld * if row_major { br } else { bc }).map(|i| (i % 7) as f32 - 3.0).collect();
                let mut cdata: Vec<f32> = (0..cld * if row_major { a.rows } else { n }).map(|i| (i % 5) as f32).collect();
                let at = |m: &DnMat, data: &[f32], i: usize, j: usize| {
                    let (rs, cs) = m.strides();
                    data[i * rs as usize + j * cs as usize]
                };
                let opb = |k: usize, j: usize| {
                    if tb {
                        at(&b, &bdata, j, k)
                    } else {
                        at(&b, &bdata, k, j)
                    }
                };
                let mut want = cdata.clone();
                let (crs, ccs) = c.strides();
                for i in 0..a.rows {
                    for j in 0..n {
                        let s: f32 = (0..a.cols).map(|k| d[i][k] * opb(k, j)).sum();
                        let o = i * crs as usize + j * ccs as usize;
                        want[o] = 2.0 * s + 0.25 * cdata[o];
                    }
                }
                let plan = spmm_plan(N, if tb { T } else { N }, &sp(&a, Format::Csr), &b, &c, F32).unwrap().unwrap();
                emulate(&[plan], &a, &bdata, &mut cdata, 2.0, 0.25);
                assert_eq!(cdata, want, "tb {tb} row_major {row_major}");
            }
        }
    }

    #[test]
    fn spmm_rejects_unsupported() {
        let a = sp(&matrix(6, 4, 9), Format::Csr);
        let b = DnMat { rows: 4, cols: 3, ld: 4, values: std::ptr::null(), ty: F32, row_major: false };
        let c = DnMat { rows: 6, cols: 3, ld: 6, values: std::ptr::null(), ty: F32, row_major: false };
        assert!(spmm_plan(N, N, &a, &b, &c, F32).unwrap().is_some());
        assert_eq!(spmm_plan(T, N, &a, &b, &c, F32), Err(cusparseError_t::NOT_SUPPORTED));
        let coo = SpMat { format: Format::Coo, ..a.clone() };
        assert_eq!(spmm_plan(N, N, &coo, &b, &c, F32), Err(cusparseError_t::NOT_SUPPORTED));
        assert_eq!(spmm_plan(N, T, &a, &b, &c, F32), Err(cusparseError_t::INVALID_VALUE));
    }

    #[test]
    fn sddmm_and_spsm_shapes() {
        let c = sp(&matrix(6, 4, 2), Format::Csr);
        let dn = |rows, cols, row_major| DnMat { rows, cols, ld: if row_major { cols } else { rows }, values: std::ptr::null(), ty: F32, row_major };
        // op(A) 6 x 3, op(B) 3 x 4
        assert_eq!(sddmm_plan(N, N, &dn(6, 3, true), &dn(3, 4, false), &c, F32), Ok([3, 3, 1, 1, 3]));
        assert_eq!(sddmm_plan(T, T, &dn(3, 6, false), &dn(4, 3, true), &c, F32), Ok([3, 3, 1, 1, 3]));
        assert_eq!(sddmm_plan(N, N, &dn(6, 3, true), &dn(2, 4, false), &c, F32), Err(cusparseError_t::INVALID_VALUE));
        let coo = SpMat { format: Format::Coo, ..c.clone() };
        assert_eq!(sddmm_plan(N, N, &dn(6, 3, true), &dn(3, 4, false), &coo, F32), Err(cusparseError_t::NOT_SUPPORTED));
        let sq = sp(&matrix(5, 5, 4), Format::Csr);
        let b = dn(2, 5, true);
        assert_eq!(spsm_plan(T, &sq, op_dims(T, &b).unwrap(), op_dims(N, &dn(5, 2, false)).unwrap()), Ok((true, [1, 5, 1, 5])));
        assert_eq!(spsm_plan(N, &c, (6, 1, 1, 1), (6, 1, 1, 1)), Err(cusparseError_t::INVALID_VALUE));
        assert_eq!(permute_words(cudaDataType_t::CUDA_C_64F), Ok((2, true)));
        assert_eq!(permute_words(cudaDataType_t::CUDA_R_16F), Err(cusparseError_t::NOT_SUPPORTED));
    }

    #[test]
    fn sp_mat_attributes_and_vectors() {
        unsafe {
            let mut d = std::ptr::null_mut();
            let (p, q) = (8usize as *mut c_void, 16usize as *mut c_void);
            imp::cusparseCreateCsr(
                &mut d,
                3,
                3,
                2,
                p,
                q,
                q,
                cusparseIndexType_t::CUSPARSE_INDEX_32I,
                cusparseIndexType_t::CUSPARSE_INDEX_32I,
                cusparseIndexBase_t::CUSPARSE_INDEX_BASE_ZERO,
                F32,
            )
            .unwrap();
            let mut v = cusparseFillMode_t::CUSPARSE_FILL_MODE_UPPER.0;
            imp::cusparseSpMatSetAttribute(d, cusparseSpMatAttribute_t::CUSPARSE_SPMAT_FILL_MODE, &mut v as *mut u32 as *mut c_void, 4).unwrap();
            let mut got = 0u32;
            imp::cusparseSpMatGetAttribute(d, cusparseSpMatAttribute_t::CUSPARSE_SPMAT_FILL_MODE, &mut got as *mut u32 as *mut c_void, 4).unwrap();
            assert_eq!(got, v);
            imp::cusparseSpMatGetAttribute(d, cusparseSpMatAttribute_t::CUSPARSE_SPMAT_DIAG_TYPE, &mut got as *mut u32 as *mut c_void, 4).unwrap();
            assert_eq!(got, cusparseDiagType_t::CUSPARSE_DIAG_TYPE_NON_UNIT.0);
            assert!((*(d as *const SpMat)).upper);
            imp::cusparseCsrSetPointers(d, q, p, p).unwrap();
            assert_eq!((*(d as *const SpMat)).a, q as *const c_void);
            imp::cusparseDestroySpMat(d).unwrap();
            let mut x = std::ptr::null_mut();
            assert_eq!(
                imp::cusparseCreateSpVec(&mut x, 4, 5, p, p, cusparseIndexType_t::CUSPARSE_INDEX_64I, cusparseIndexBase_t::CUSPARSE_INDEX_BASE_ONE, F32),
                Err(cusparseError_t::INVALID_VALUE)
            );
            imp::cusparseCreateSpVec(&mut x, 4, 2, p, p, cusparseIndexType_t::CUSPARSE_INDEX_64I, cusparseIndexBase_t::CUSPARSE_INDEX_BASE_ONE, F32).unwrap();
            assert_eq!(*(x as *const SpVec), SpVec { size: 4, nnz: 2, idx: p, val: p, wide: true, base: 1, double: false });
            imp::cusparseDestroySpVec(x).unwrap();
            let mut sv = std::ptr::null_mut();
            imp::cusparseSpSV_createDescr(&mut sv).unwrap();
            imp::cusparseSpSV_destroyDescr(sv).unwrap();
        }
    }

    #[test]
    fn csc_is_the_csr_transpose() {
        // the CSC arrays of A are the CSR arrays of A^T
        let at = matrix(5, 7, 11);
        let d = dense(&at);
        let csc = SpMat { format: Format::Csc, rows: 7, cols: 5, ..sp(&at, Format::Csr) };
        for (op, trans) in [(N, false), (T, true)] {
            let (view, vop) = csc.csr(op);
            assert_eq!((view.format, view.rows, view.cols, view.upper), (Format::Csr, 5, 7, true));
            // op(A) x with A = D^T
            let (xn, yn) = if trans { (7, 5) } else { (5, 7) };
            let x: Vec<f32> = (0..xn).map(|i| i as f32 - 1.0).collect();
            let mut y = vec![0.0; yn];
            let want: Vec<f32> = (0..yn).map(|i| (0..xn).map(|j| if trans { d[i][j] } else { d[j][i] } * x[j]).sum()).collect();
            let plan = spmv_plan(vop, &view, &vec_of(xn), &vec_of(yn), F32).unwrap();
            emulate(&plan, &at, &x, &mut y, 1.0, 0.0);
            assert_eq!(y, want, "trans {trans}");
        }
        let m = DnMat { rows: 7, cols: 5, ld: 9, values: std::ptr::null(), ty: F32, row_major: false };
        assert_eq!(m.transposed().strides(), (9, 1));
    }

    #[test]
    fn version() {
        let mut v = 0;
        unsafe { imp::cusparseGetProperty(libraryPropertyType::MAJOR_VERSION, &mut v) }.unwrap();
        assert_eq!(v, 12);
    }

    #[test]
    fn mat_descr_round_trip() {
        unsafe {
            let mut d = std::ptr::null_mut();
            imp::cusparseCreateMatDescr(&mut d).unwrap();
            assert_eq!(imp::cusparseGetMatType(d), cusparseMatrixType_t::CUSPARSE_MATRIX_TYPE_GENERAL);
            imp::cusparseSetMatIndexBase(d, cusparseIndexBase_t::CUSPARSE_INDEX_BASE_ONE).unwrap();
            assert_eq!(imp::cusparseGetMatIndexBase(d), cusparseIndexBase_t::CUSPARSE_INDEX_BASE_ONE);
            assert_eq!(imp::cusparseSetMatFillMode(d, cusparseFillMode_t(7)), Err(cusparseError_t::INVALID_VALUE));
            imp::cusparseDestroyMatDescr(d).unwrap();
        }
    }
}
