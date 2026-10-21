//! cuSPARSE on Apple GPUs for zluda_agx (Asahi Linux), on the sparse kernels of agx_libs
//! (`agx_libs/kernels/sparse.mlir`, written once for every AGX generation).
//!
//! Supported so far: handles, streams and pointer modes, the legacy matrix descriptor, and the generic API for f32
//! and f64 values (f64 on the f64 copies of the kernels, all operands of one type) with 32- or 64-bit indices (row offsets and columns of the same type), zero- or one-based:
//! `cusparseSpMV` on CSR and COO matrices (non-transposed and transposed) and `cusparseSpMM` on CSR matrices
//! (`opA` non-transposed, `opB` non-transposed or transposed, column- or row-major dense matrices). No workspace is
//! needed. Other functions return `CUSPARSE_STATUS_NOT_SUPPORTED`.

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
            cusparseCreate,
            cusparseCreateConstCoo,
            cusparseCreateConstCsr,
            cusparseCreateConstDnMat,
            cusparseCreateConstDnVec,
            cusparseCreateCoo,
            cusparseCreateCsr,
            cusparseCreateDnMat,
            cusparseCreateDnVec,
            cusparseCreateMatDescr,
            cusparseDestroy,
            cusparseDestroyDnMat,
            cusparseDestroyDnVec,
            cusparseDestroyMatDescr,
            cusparseDestroySpMat,
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
            cusparseSetMatDiagType,
            cusparseSetMatFillMode,
            cusparseSetMatIndexBase,
            cusparseSetMatType,
            cusparseSetPointerMode,
            cusparseSetStream,
            cusparseSpMM,
            cusparseSpMM_bufferSize,
            cusparseSpMM_preprocess,
            cusparseSpMV,
            cusparseSpMV_bufferSize,
            cusparseSpMatGetSize,
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
        (Format::Csr, true) if a.rows == 0 || a.nnz == 0 => vec![Launch::Scale { n: yn, stride: 1 }],
        (Format::Csr, true) => vec![Launch::Scale { n: yn, stride: 1 }, Launch::CsrSpmvT],
        (Format::Coo, _) if a.nnz == 0 => vec![Launch::Scale { n: yn, stride: 1 }],
        (Format::Coo, trans) => vec![Launch::Scale { n: yn, stride: 1 }, Launch::CooSpmv { trans }],
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
        if rows < 0 || cols < 0 || nnz < 0 || (format == Format::Csr && rows > 0 && a.is_null()) {
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
        Ok(SpMat { format, rows, cols, nnz, a, col, val, wide, base: base(idx_base)?, double })
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

    pub(crate) unsafe fn cusparseCreateConstDnVec(
        d: *mut cusparseConstDnVecDescr_t,
        size: i64,
        values: *const c_void,
        ty: cudaDataType,
    ) -> cusparseStatus_t {
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
        let (alpha, beta): (*mut c_void, *mut c_void) =
            if a.double { (arg(&mut alpha64), arg(&mut beta64)) } else { (arg(&mut alpha32), arg(&mut beta32)) };
        // index width, then the f32 kernel or its f64 copy
        let pick = |n32: &'static std::ffi::CStr, n64: &'static std::ffi::CStr, d32: &'static std::ffi::CStr, d64: &'static std::ffi::CStr| {
            match (a.wide, a.double) {
                (false, false) => n32,
                (true, false) => n64,
                (false, true) => d32,
                (true, true) => d64,
            }
        };
        for l in launches {
            match *l {
                Launch::Scale { mut n, mut stride } => {
                    run(h, if a.double { c"agx_sparse_scale_f64" } else { c"agx_sparse_scale" }, &mut [arg(&mut py), arg(&mut n), arg(&mut stride), beta], [n, 1], [64, 1])?
                }
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
        spmv_plan(op, obj(a)?, obj(x)?, obj(y)?, compute)?;
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
        let launches = spmv_plan(op, a, x, y, compute)?;
        let (alpha, beta) = (scalar(h, alpha, a.double)?, scalar(h, beta, a.double)?);
        execute(h, &launches, a, x.values, y.values, alpha, beta)
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
        spmm_plan(op_a, op_b, obj(a)?, obj(b)?, obj(c)?, compute)?;
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
        spmm_plan(op_a, op_b, obj(a)?, obj(b)?, obj(c)?, compute).map(|_| ())
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
        let Some(launch) = spmm_plan(op_a, op_b, a, b, c, compute)? else {
            return Ok(());
        };
        let (alpha, beta) = (scalar(h, alpha, a.double)?, scalar(h, beta, a.double)?);
        execute(h, &[launch], a, b.values, c.values, alpha, beta)
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
                let opb = |k: usize, j: usize| if tb { at(&b, &bdata, j, k) } else { at(&b, &bdata, k, j) };
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
