// Generated automatically by zluda_bindgen
// DO NOT EDIT MANUALLY
#![allow(warnings)]
extern "system" {
    #[must_use]
    fn cusparseCreate(
        handle: *mut cuda_types::cusparse::cusparseHandle_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDestroy(
        handle: cuda_types::cusparse::cusparseHandle_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseGetVersion(
        handle: cuda_types::cusparse::cusparseHandle_t,
        version: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseGetProperty(
        type_: cuda_types::cusparse::libraryPropertyType,
        value: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    fn cusparseGetErrorName(
        status: cuda_types::cusparse::cusparseStatus_t,
    ) -> *const ::core::ffi::c_char;
    fn cusparseGetErrorString(
        status: cuda_types::cusparse::cusparseStatus_t,
    ) -> *const ::core::ffi::c_char;
    #[must_use]
    fn cusparseSetStream(
        handle: cuda_types::cusparse::cusparseHandle_t,
        streamId: cuda_types::cusparse::cudaStream_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseGetStream(
        handle: cuda_types::cusparse::cusparseHandle_t,
        streamId: *mut cuda_types::cusparse::cudaStream_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseGetPointerMode(
        handle: cuda_types::cusparse::cusparseHandle_t,
        mode: *mut cuda_types::cusparse::cusparsePointerMode_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSetPointerMode(
        handle: cuda_types::cusparse::cusparseHandle_t,
        mode: cuda_types::cusparse::cusparsePointerMode_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseLoggerSetCallback(
        callback: cuda_types::cusparse::cusparseLoggerCallback_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseLoggerSetFile(
        file: *mut cuda_types::FILE,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseLoggerOpenFile(
        logFile: *const ::core::ffi::c_char,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseLoggerSetLevel(
        level: ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseLoggerSetMask(
        mask: ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseLoggerForceDisable() -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCreateMatDescr(
        descrA: *mut cuda_types::cusparse::cusparseMatDescr_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDestroyMatDescr(
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSetMatType(
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        type_: cuda_types::cusparse::cusparseMatrixType_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    fn cusparseGetMatType(
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
    ) -> cuda_types::cusparse::cusparseMatrixType_t;
    #[must_use]
    fn cusparseSetMatFillMode(
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        fillMode: cuda_types::cusparse::cusparseFillMode_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    fn cusparseGetMatFillMode(
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
    ) -> cuda_types::cusparse::cusparseFillMode_t;
    #[must_use]
    fn cusparseSetMatDiagType(
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        diagType: cuda_types::cusparse::cusparseDiagType_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    fn cusparseGetMatDiagType(
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
    ) -> cuda_types::cusparse::cusparseDiagType_t;
    #[must_use]
    fn cusparseSetMatIndexBase(
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        base: cuda_types::cusparse::cusparseIndexBase_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    fn cusparseGetMatIndexBase(
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
    ) -> cuda_types::cusparse::cusparseIndexBase_t;
    #[must_use]
    fn cusparseCreateCsric02Info(
        info: *mut cuda_types::cusparse::csric02Info_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDestroyCsric02Info(
        info: cuda_types::cusparse::csric02Info_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCreateBsric02Info(
        info: *mut cuda_types::cusparse::bsric02Info_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDestroyBsric02Info(
        info: cuda_types::cusparse::bsric02Info_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCreateCsrilu02Info(
        info: *mut cuda_types::cusparse::csrilu02Info_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDestroyCsrilu02Info(
        info: cuda_types::cusparse::csrilu02Info_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCreateBsrilu02Info(
        info: *mut cuda_types::cusparse::bsrilu02Info_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDestroyBsrilu02Info(
        info: cuda_types::cusparse::bsrilu02Info_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCreateBsrsv2Info(
        info: *mut cuda_types::cusparse::bsrsv2Info_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDestroyBsrsv2Info(
        info: cuda_types::cusparse::bsrsv2Info_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCreateBsrsm2Info(
        info: *mut cuda_types::cusparse::bsrsm2Info_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDestroyBsrsm2Info(
        info: cuda_types::cusparse::bsrsm2Info_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCreateCsru2csrInfo(
        info: *mut cuda_types::cusparse::csru2csrInfo_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDestroyCsru2csrInfo(
        info: cuda_types::cusparse::csru2csrInfo_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCreateColorInfo(
        info: *mut cuda_types::cusparse::cusparseColorInfo_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDestroyColorInfo(
        info: cuda_types::cusparse::cusparseColorInfo_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCreatePruneInfo(
        info: *mut cuda_types::cusparse::pruneInfo_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDestroyPruneInfo(
        info: cuda_types::cusparse::pruneInfo_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSgemvi(
        handle: cuda_types::cusparse::cusparseHandle_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        xVal: *const f32,
        xInd: *const ::core::ffi::c_int,
        beta: *const f32,
        y: *mut f32,
        idxBase: cuda_types::cusparse::cusparseIndexBase_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSgemvi_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        pBufferSize: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDgemvi(
        handle: cuda_types::cusparse::cusparseHandle_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f64,
        A: *const f64,
        lda: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        xVal: *const f64,
        xInd: *const ::core::ffi::c_int,
        beta: *const f64,
        y: *mut f64,
        idxBase: cuda_types::cusparse::cusparseIndexBase_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDgemvi_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        pBufferSize: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCgemvi(
        handle: cuda_types::cusparse::cusparseHandle_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cusparse::cuComplex,
        A: *const cuda_types::cusparse::cuComplex,
        lda: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        xVal: *const cuda_types::cusparse::cuComplex,
        xInd: *const ::core::ffi::c_int,
        beta: *const cuda_types::cusparse::cuComplex,
        y: *mut cuda_types::cusparse::cuComplex,
        idxBase: cuda_types::cusparse::cusparseIndexBase_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCgemvi_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        pBufferSize: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZgemvi(
        handle: cuda_types::cusparse::cusparseHandle_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cusparse::cuDoubleComplex,
        A: *const cuda_types::cusparse::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        xVal: *const cuda_types::cusparse::cuDoubleComplex,
        xInd: *const ::core::ffi::c_int,
        beta: *const cuda_types::cusparse::cuDoubleComplex,
        y: *mut cuda_types::cusparse::cuDoubleComplex,
        idxBase: cuda_types::cusparse::cusparseIndexBase_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZgemvi_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        pBufferSize: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSbsrmv(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const f32,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const f32,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        x: *const f32,
        beta: *const f32,
        y: *mut f32,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDbsrmv(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const f64,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const f64,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        x: *const f64,
        beta: *const f64,
        y: *mut f64,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCbsrmv(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const cuda_types::cusparse::cuComplex,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const cuda_types::cusparse::cuComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        x: *const cuda_types::cusparse::cuComplex,
        beta: *const cuda_types::cusparse::cuComplex,
        y: *mut cuda_types::cusparse::cuComplex,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZbsrmv(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const cuda_types::cusparse::cuDoubleComplex,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const cuda_types::cusparse::cuDoubleComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        x: *const cuda_types::cusparse::cuDoubleComplex,
        beta: *const cuda_types::cusparse::cuDoubleComplex,
        y: *mut cuda_types::cusparse::cuDoubleComplex,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSbsrxmv(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        sizeOfMask: ::core::ffi::c_int,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const f32,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const f32,
        bsrSortedMaskPtrA: *const ::core::ffi::c_int,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedEndPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        x: *const f32,
        beta: *const f32,
        y: *mut f32,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDbsrxmv(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        sizeOfMask: ::core::ffi::c_int,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const f64,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const f64,
        bsrSortedMaskPtrA: *const ::core::ffi::c_int,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedEndPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        x: *const f64,
        beta: *const f64,
        y: *mut f64,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCbsrxmv(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        sizeOfMask: ::core::ffi::c_int,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const cuda_types::cusparse::cuComplex,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const cuda_types::cusparse::cuComplex,
        bsrSortedMaskPtrA: *const ::core::ffi::c_int,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedEndPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        x: *const cuda_types::cusparse::cuComplex,
        beta: *const cuda_types::cusparse::cuComplex,
        y: *mut cuda_types::cusparse::cuComplex,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZbsrxmv(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        sizeOfMask: ::core::ffi::c_int,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const cuda_types::cusparse::cuDoubleComplex,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const cuda_types::cusparse::cuDoubleComplex,
        bsrSortedMaskPtrA: *const ::core::ffi::c_int,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedEndPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        x: *const cuda_types::cusparse::cuDoubleComplex,
        beta: *const cuda_types::cusparse::cuDoubleComplex,
        y: *mut cuda_types::cusparse::cuDoubleComplex,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseXbsrsv2_zeroPivot(
        handle: cuda_types::cusparse::cusparseHandle_t,
        info: cuda_types::cusparse::bsrsv2Info_t,
        position: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSbsrsv2_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *mut f32,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsv2Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDbsrsv2_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *mut f64,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsv2Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCbsrsv2_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *mut cuda_types::cusparse::cuComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsv2Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZbsrsv2_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *mut cuda_types::cusparse::cuDoubleComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsv2Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSbsrsv2_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *mut f32,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsv2Info_t,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDbsrsv2_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *mut f64,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsv2Info_t,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCbsrsv2_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *mut cuda_types::cusparse::cuComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsv2Info_t,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZbsrsv2_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *mut cuda_types::cusparse::cuDoubleComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsv2Info_t,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSbsrsv2_analysis(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const f32,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsv2Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDbsrsv2_analysis(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const f64,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsv2Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCbsrsv2_analysis(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const cuda_types::cusparse::cuComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsv2Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZbsrsv2_analysis(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const cuda_types::cusparse::cuDoubleComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsv2Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSbsrsv2_solve(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const f32,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const f32,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsv2Info_t,
        f: *const f32,
        x: *mut f32,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDbsrsv2_solve(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const f64,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const f64,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsv2Info_t,
        f: *const f64,
        x: *mut f64,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCbsrsv2_solve(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const cuda_types::cusparse::cuComplex,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const cuda_types::cusparse::cuComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsv2Info_t,
        f: *const cuda_types::cusparse::cuComplex,
        x: *mut cuda_types::cusparse::cuComplex,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZbsrsv2_solve(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const cuda_types::cusparse::cuDoubleComplex,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const cuda_types::cusparse::cuDoubleComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsv2Info_t,
        f: *const cuda_types::cusparse::cuDoubleComplex,
        x: *mut cuda_types::cusparse::cuDoubleComplex,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSbsrmm(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        transB: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        kb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const f32,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const f32,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        B: *const f32,
        ldb: ::core::ffi::c_int,
        beta: *const f32,
        C: *mut f32,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDbsrmm(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        transB: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        kb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const f64,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const f64,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        B: *const f64,
        ldb: ::core::ffi::c_int,
        beta: *const f64,
        C: *mut f64,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCbsrmm(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        transB: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        kb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const cuda_types::cusparse::cuComplex,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const cuda_types::cusparse::cuComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        B: *const cuda_types::cusparse::cuComplex,
        ldb: ::core::ffi::c_int,
        beta: *const cuda_types::cusparse::cuComplex,
        C: *mut cuda_types::cusparse::cuComplex,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZbsrmm(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        transB: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        kb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const cuda_types::cusparse::cuDoubleComplex,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const cuda_types::cusparse::cuDoubleComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        B: *const cuda_types::cusparse::cuDoubleComplex,
        ldb: ::core::ffi::c_int,
        beta: *const cuda_types::cusparse::cuDoubleComplex,
        C: *mut cuda_types::cusparse::cuDoubleComplex,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseXbsrsm2_zeroPivot(
        handle: cuda_types::cusparse::cusparseHandle_t,
        info: cuda_types::cusparse::bsrsm2Info_t,
        position: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSbsrsm2_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        transXY: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut f32,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsm2Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDbsrsm2_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        transXY: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut f64,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsm2Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCbsrsm2_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        transXY: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut cuda_types::cusparse::cuComplex,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsm2Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZbsrsm2_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        transXY: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut cuda_types::cusparse::cuDoubleComplex,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsm2Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSbsrsm2_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        transB: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut f32,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsm2Info_t,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDbsrsm2_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        transB: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut f64,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsm2Info_t,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCbsrsm2_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        transB: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut cuda_types::cusparse::cuComplex,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsm2Info_t,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZbsrsm2_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        transB: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut cuda_types::cusparse::cuDoubleComplex,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsm2Info_t,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSbsrsm2_analysis(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        transXY: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *const f32,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsm2Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDbsrsm2_analysis(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        transXY: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *const f64,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsm2Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCbsrsm2_analysis(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        transXY: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *const cuda_types::cusparse::cuComplex,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsm2Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZbsrsm2_analysis(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        transXY: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *const cuda_types::cusparse::cuDoubleComplex,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsm2Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSbsrsm2_solve(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        transXY: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const f32,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *const f32,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsm2Info_t,
        B: *const f32,
        ldb: ::core::ffi::c_int,
        X: *mut f32,
        ldx: ::core::ffi::c_int,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDbsrsm2_solve(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        transXY: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const f64,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *const f64,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsm2Info_t,
        B: *const f64,
        ldb: ::core::ffi::c_int,
        X: *mut f64,
        ldx: ::core::ffi::c_int,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCbsrsm2_solve(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        transXY: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const cuda_types::cusparse::cuComplex,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *const cuda_types::cusparse::cuComplex,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsm2Info_t,
        B: *const cuda_types::cusparse::cuComplex,
        ldb: ::core::ffi::c_int,
        X: *mut cuda_types::cusparse::cuComplex,
        ldx: ::core::ffi::c_int,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZbsrsm2_solve(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        transA: cuda_types::cusparse::cusparseOperation_t,
        transXY: cuda_types::cusparse::cusparseOperation_t,
        mb: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        alpha: *const cuda_types::cusparse::cuDoubleComplex,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *const cuda_types::cusparse::cuDoubleComplex,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrsm2Info_t,
        B: *const cuda_types::cusparse::cuDoubleComplex,
        ldb: ::core::ffi::c_int,
        X: *mut cuda_types::cusparse::cuDoubleComplex,
        ldx: ::core::ffi::c_int,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseScsrilu02_numericBoost(
        handle: cuda_types::cusparse::cusparseHandle_t,
        info: cuda_types::cusparse::csrilu02Info_t,
        enable_boost: ::core::ffi::c_int,
        tol: *mut f64,
        boost_val: *mut f32,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDcsrilu02_numericBoost(
        handle: cuda_types::cusparse::cusparseHandle_t,
        info: cuda_types::cusparse::csrilu02Info_t,
        enable_boost: ::core::ffi::c_int,
        tol: *mut f64,
        boost_val: *mut f64,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCcsrilu02_numericBoost(
        handle: cuda_types::cusparse::cusparseHandle_t,
        info: cuda_types::cusparse::csrilu02Info_t,
        enable_boost: ::core::ffi::c_int,
        tol: *mut f64,
        boost_val: *mut cuda_types::cusparse::cuComplex,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZcsrilu02_numericBoost(
        handle: cuda_types::cusparse::cusparseHandle_t,
        info: cuda_types::cusparse::csrilu02Info_t,
        enable_boost: ::core::ffi::c_int,
        tol: *mut f64,
        boost_val: *mut cuda_types::cusparse::cuDoubleComplex,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseXcsrilu02_zeroPivot(
        handle: cuda_types::cusparse::cusparseHandle_t,
        info: cuda_types::cusparse::csrilu02Info_t,
        position: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseScsrilu02_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *mut f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csrilu02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDcsrilu02_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *mut f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csrilu02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCcsrilu02_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *mut cuda_types::cusparse::cuComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csrilu02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZcsrilu02_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *mut cuda_types::cusparse::cuDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csrilu02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseScsrilu02_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedVal: *mut f32,
        csrSortedRowPtr: *const ::core::ffi::c_int,
        csrSortedColInd: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csrilu02Info_t,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDcsrilu02_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedVal: *mut f64,
        csrSortedRowPtr: *const ::core::ffi::c_int,
        csrSortedColInd: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csrilu02Info_t,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCcsrilu02_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedVal: *mut cuda_types::cusparse::cuComplex,
        csrSortedRowPtr: *const ::core::ffi::c_int,
        csrSortedColInd: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csrilu02Info_t,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZcsrilu02_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedVal: *mut cuda_types::cusparse::cuDoubleComplex,
        csrSortedRowPtr: *const ::core::ffi::c_int,
        csrSortedColInd: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csrilu02Info_t,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseScsrilu02_analysis(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csrilu02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDcsrilu02_analysis(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csrilu02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCcsrilu02_analysis(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const cuda_types::cusparse::cuComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csrilu02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZcsrilu02_analysis(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const cuda_types::cusparse::cuDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csrilu02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseScsrilu02(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA_valM: *mut f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csrilu02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDcsrilu02(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA_valM: *mut f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csrilu02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCcsrilu02(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA_valM: *mut cuda_types::cusparse::cuComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csrilu02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZcsrilu02(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA_valM: *mut cuda_types::cusparse::cuDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csrilu02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSbsrilu02_numericBoost(
        handle: cuda_types::cusparse::cusparseHandle_t,
        info: cuda_types::cusparse::bsrilu02Info_t,
        enable_boost: ::core::ffi::c_int,
        tol: *mut f64,
        boost_val: *mut f32,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDbsrilu02_numericBoost(
        handle: cuda_types::cusparse::cusparseHandle_t,
        info: cuda_types::cusparse::bsrilu02Info_t,
        enable_boost: ::core::ffi::c_int,
        tol: *mut f64,
        boost_val: *mut f64,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCbsrilu02_numericBoost(
        handle: cuda_types::cusparse::cusparseHandle_t,
        info: cuda_types::cusparse::bsrilu02Info_t,
        enable_boost: ::core::ffi::c_int,
        tol: *mut f64,
        boost_val: *mut cuda_types::cusparse::cuComplex,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZbsrilu02_numericBoost(
        handle: cuda_types::cusparse::cusparseHandle_t,
        info: cuda_types::cusparse::bsrilu02Info_t,
        enable_boost: ::core::ffi::c_int,
        tol: *mut f64,
        boost_val: *mut cuda_types::cusparse::cuDoubleComplex,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseXbsrilu02_zeroPivot(
        handle: cuda_types::cusparse::cusparseHandle_t,
        info: cuda_types::cusparse::bsrilu02Info_t,
        position: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSbsrilu02_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut f32,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrilu02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDbsrilu02_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut f64,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrilu02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCbsrilu02_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut cuda_types::cusparse::cuComplex,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrilu02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZbsrilu02_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut cuda_types::cusparse::cuDoubleComplex,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrilu02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSbsrilu02_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut f32,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrilu02Info_t,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDbsrilu02_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut f64,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrilu02Info_t,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCbsrilu02_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut cuda_types::cusparse::cuComplex,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrilu02Info_t,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZbsrilu02_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut cuda_types::cusparse::cuDoubleComplex,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrilu02Info_t,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSbsrilu02_analysis(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut f32,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrilu02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDbsrilu02_analysis(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut f64,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrilu02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCbsrilu02_analysis(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut cuda_types::cusparse::cuComplex,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrilu02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZbsrilu02_analysis(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut cuda_types::cusparse::cuDoubleComplex,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrilu02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSbsrilu02(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut f32,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrilu02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDbsrilu02(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut f64,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrilu02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCbsrilu02(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut cuda_types::cusparse::cuComplex,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrilu02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZbsrilu02(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut cuda_types::cusparse::cuDoubleComplex,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsrilu02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseXcsric02_zeroPivot(
        handle: cuda_types::cusparse::cusparseHandle_t,
        info: cuda_types::cusparse::csric02Info_t,
        position: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseScsric02_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *mut f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csric02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDcsric02_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *mut f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csric02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCcsric02_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *mut cuda_types::cusparse::cuComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csric02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZcsric02_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *mut cuda_types::cusparse::cuDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csric02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseScsric02_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedVal: *mut f32,
        csrSortedRowPtr: *const ::core::ffi::c_int,
        csrSortedColInd: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csric02Info_t,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDcsric02_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedVal: *mut f64,
        csrSortedRowPtr: *const ::core::ffi::c_int,
        csrSortedColInd: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csric02Info_t,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCcsric02_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedVal: *mut cuda_types::cusparse::cuComplex,
        csrSortedRowPtr: *const ::core::ffi::c_int,
        csrSortedColInd: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csric02Info_t,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZcsric02_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedVal: *mut cuda_types::cusparse::cuDoubleComplex,
        csrSortedRowPtr: *const ::core::ffi::c_int,
        csrSortedColInd: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csric02Info_t,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseScsric02_analysis(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csric02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDcsric02_analysis(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csric02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCcsric02_analysis(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const cuda_types::cusparse::cuComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csric02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZcsric02_analysis(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const cuda_types::cusparse::cuDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csric02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseScsric02(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA_valM: *mut f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csric02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDcsric02(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA_valM: *mut f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csric02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCcsric02(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA_valM: *mut cuda_types::cusparse::cuComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csric02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZcsric02(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA_valM: *mut cuda_types::cusparse::cuDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::csric02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseXbsric02_zeroPivot(
        handle: cuda_types::cusparse::cusparseHandle_t,
        info: cuda_types::cusparse::bsric02Info_t,
        position: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSbsric02_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut f32,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsric02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDbsric02_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut f64,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsric02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCbsric02_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut cuda_types::cusparse::cuComplex,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsric02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZbsric02_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut cuda_types::cusparse::cuDoubleComplex,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsric02Info_t,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSbsric02_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut f32,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsric02Info_t,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDbsric02_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut f64,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsric02Info_t,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCbsric02_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut cuda_types::cusparse::cuComplex,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsric02Info_t,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZbsric02_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut cuda_types::cusparse::cuDoubleComplex,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockSize: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsric02Info_t,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSbsric02_analysis(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *const f32,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsric02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pInputBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDbsric02_analysis(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *const f64,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsric02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pInputBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCbsric02_analysis(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *const cuda_types::cusparse::cuComplex,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsric02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pInputBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZbsric02_analysis(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *const cuda_types::cusparse::cuDoubleComplex,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsric02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pInputBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSbsric02(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut f32,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsric02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDbsric02(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut f64,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsric02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCbsric02(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut cuda_types::cusparse::cuComplex,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsric02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZbsric02(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedVal: *mut cuda_types::cusparse::cuDoubleComplex,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        info: cuda_types::cusparse::bsric02Info_t,
        policy: cuda_types::cusparse::cusparseSolvePolicy_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSgtsv2_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const f32,
        d: *const f32,
        du: *const f32,
        B: *const f32,
        ldb: ::core::ffi::c_int,
        bufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDgtsv2_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const f64,
        d: *const f64,
        du: *const f64,
        B: *const f64,
        ldb: ::core::ffi::c_int,
        bufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCgtsv2_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const cuda_types::cusparse::cuComplex,
        d: *const cuda_types::cusparse::cuComplex,
        du: *const cuda_types::cusparse::cuComplex,
        B: *const cuda_types::cusparse::cuComplex,
        ldb: ::core::ffi::c_int,
        bufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZgtsv2_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const cuda_types::cusparse::cuDoubleComplex,
        d: *const cuda_types::cusparse::cuDoubleComplex,
        du: *const cuda_types::cusparse::cuDoubleComplex,
        B: *const cuda_types::cusparse::cuDoubleComplex,
        ldb: ::core::ffi::c_int,
        bufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSgtsv2(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const f32,
        d: *const f32,
        du: *const f32,
        B: *mut f32,
        ldb: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDgtsv2(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const f64,
        d: *const f64,
        du: *const f64,
        B: *mut f64,
        ldb: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCgtsv2(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const cuda_types::cusparse::cuComplex,
        d: *const cuda_types::cusparse::cuComplex,
        du: *const cuda_types::cusparse::cuComplex,
        B: *mut cuda_types::cusparse::cuComplex,
        ldb: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZgtsv2(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const cuda_types::cusparse::cuDoubleComplex,
        d: *const cuda_types::cusparse::cuDoubleComplex,
        du: *const cuda_types::cusparse::cuDoubleComplex,
        B: *mut cuda_types::cusparse::cuDoubleComplex,
        ldb: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSgtsv2_nopivot_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const f32,
        d: *const f32,
        du: *const f32,
        B: *const f32,
        ldb: ::core::ffi::c_int,
        bufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDgtsv2_nopivot_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const f64,
        d: *const f64,
        du: *const f64,
        B: *const f64,
        ldb: ::core::ffi::c_int,
        bufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCgtsv2_nopivot_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const cuda_types::cusparse::cuComplex,
        d: *const cuda_types::cusparse::cuComplex,
        du: *const cuda_types::cusparse::cuComplex,
        B: *const cuda_types::cusparse::cuComplex,
        ldb: ::core::ffi::c_int,
        bufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZgtsv2_nopivot_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const cuda_types::cusparse::cuDoubleComplex,
        d: *const cuda_types::cusparse::cuDoubleComplex,
        du: *const cuda_types::cusparse::cuDoubleComplex,
        B: *const cuda_types::cusparse::cuDoubleComplex,
        ldb: ::core::ffi::c_int,
        bufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSgtsv2_nopivot(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const f32,
        d: *const f32,
        du: *const f32,
        B: *mut f32,
        ldb: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDgtsv2_nopivot(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const f64,
        d: *const f64,
        du: *const f64,
        B: *mut f64,
        ldb: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCgtsv2_nopivot(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const cuda_types::cusparse::cuComplex,
        d: *const cuda_types::cusparse::cuComplex,
        du: *const cuda_types::cusparse::cuComplex,
        B: *mut cuda_types::cusparse::cuComplex,
        ldb: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZgtsv2_nopivot(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        dl: *const cuda_types::cusparse::cuDoubleComplex,
        d: *const cuda_types::cusparse::cuDoubleComplex,
        du: *const cuda_types::cusparse::cuDoubleComplex,
        B: *mut cuda_types::cusparse::cuDoubleComplex,
        ldb: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSgtsv2StridedBatch_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        dl: *const f32,
        d: *const f32,
        du: *const f32,
        x: *const f32,
        batchCount: ::core::ffi::c_int,
        batchStride: ::core::ffi::c_int,
        bufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDgtsv2StridedBatch_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        dl: *const f64,
        d: *const f64,
        du: *const f64,
        x: *const f64,
        batchCount: ::core::ffi::c_int,
        batchStride: ::core::ffi::c_int,
        bufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCgtsv2StridedBatch_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        dl: *const cuda_types::cusparse::cuComplex,
        d: *const cuda_types::cusparse::cuComplex,
        du: *const cuda_types::cusparse::cuComplex,
        x: *const cuda_types::cusparse::cuComplex,
        batchCount: ::core::ffi::c_int,
        batchStride: ::core::ffi::c_int,
        bufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZgtsv2StridedBatch_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        dl: *const cuda_types::cusparse::cuDoubleComplex,
        d: *const cuda_types::cusparse::cuDoubleComplex,
        du: *const cuda_types::cusparse::cuDoubleComplex,
        x: *const cuda_types::cusparse::cuDoubleComplex,
        batchCount: ::core::ffi::c_int,
        batchStride: ::core::ffi::c_int,
        bufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSgtsv2StridedBatch(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        dl: *const f32,
        d: *const f32,
        du: *const f32,
        x: *mut f32,
        batchCount: ::core::ffi::c_int,
        batchStride: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDgtsv2StridedBatch(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        dl: *const f64,
        d: *const f64,
        du: *const f64,
        x: *mut f64,
        batchCount: ::core::ffi::c_int,
        batchStride: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCgtsv2StridedBatch(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        dl: *const cuda_types::cusparse::cuComplex,
        d: *const cuda_types::cusparse::cuComplex,
        du: *const cuda_types::cusparse::cuComplex,
        x: *mut cuda_types::cusparse::cuComplex,
        batchCount: ::core::ffi::c_int,
        batchStride: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZgtsv2StridedBatch(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        dl: *const cuda_types::cusparse::cuDoubleComplex,
        d: *const cuda_types::cusparse::cuDoubleComplex,
        du: *const cuda_types::cusparse::cuDoubleComplex,
        x: *mut cuda_types::cusparse::cuDoubleComplex,
        batchCount: ::core::ffi::c_int,
        batchStride: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSgtsvInterleavedBatch_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        dl: *const f32,
        d: *const f32,
        du: *const f32,
        x: *const f32,
        batchCount: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDgtsvInterleavedBatch_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        dl: *const f64,
        d: *const f64,
        du: *const f64,
        x: *const f64,
        batchCount: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCgtsvInterleavedBatch_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        dl: *const cuda_types::cusparse::cuComplex,
        d: *const cuda_types::cusparse::cuComplex,
        du: *const cuda_types::cusparse::cuComplex,
        x: *const cuda_types::cusparse::cuComplex,
        batchCount: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZgtsvInterleavedBatch_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        dl: *const cuda_types::cusparse::cuDoubleComplex,
        d: *const cuda_types::cusparse::cuDoubleComplex,
        du: *const cuda_types::cusparse::cuDoubleComplex,
        x: *const cuda_types::cusparse::cuDoubleComplex,
        batchCount: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSgtsvInterleavedBatch(
        handle: cuda_types::cusparse::cusparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        dl: *mut f32,
        d: *mut f32,
        du: *mut f32,
        x: *mut f32,
        batchCount: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDgtsvInterleavedBatch(
        handle: cuda_types::cusparse::cusparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        dl: *mut f64,
        d: *mut f64,
        du: *mut f64,
        x: *mut f64,
        batchCount: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCgtsvInterleavedBatch(
        handle: cuda_types::cusparse::cusparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        dl: *mut cuda_types::cusparse::cuComplex,
        d: *mut cuda_types::cusparse::cuComplex,
        du: *mut cuda_types::cusparse::cuComplex,
        x: *mut cuda_types::cusparse::cuComplex,
        batchCount: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZgtsvInterleavedBatch(
        handle: cuda_types::cusparse::cusparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        dl: *mut cuda_types::cusparse::cuDoubleComplex,
        d: *mut cuda_types::cusparse::cuDoubleComplex,
        du: *mut cuda_types::cusparse::cuDoubleComplex,
        x: *mut cuda_types::cusparse::cuDoubleComplex,
        batchCount: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSgpsvInterleavedBatch_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        ds: *const f32,
        dl: *const f32,
        d: *const f32,
        du: *const f32,
        dw: *const f32,
        x: *const f32,
        batchCount: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDgpsvInterleavedBatch_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        ds: *const f64,
        dl: *const f64,
        d: *const f64,
        du: *const f64,
        dw: *const f64,
        x: *const f64,
        batchCount: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCgpsvInterleavedBatch_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        ds: *const cuda_types::cusparse::cuComplex,
        dl: *const cuda_types::cusparse::cuComplex,
        d: *const cuda_types::cusparse::cuComplex,
        du: *const cuda_types::cusparse::cuComplex,
        dw: *const cuda_types::cusparse::cuComplex,
        x: *const cuda_types::cusparse::cuComplex,
        batchCount: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZgpsvInterleavedBatch_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        ds: *const cuda_types::cusparse::cuDoubleComplex,
        dl: *const cuda_types::cusparse::cuDoubleComplex,
        d: *const cuda_types::cusparse::cuDoubleComplex,
        du: *const cuda_types::cusparse::cuDoubleComplex,
        dw: *const cuda_types::cusparse::cuDoubleComplex,
        x: *const cuda_types::cusparse::cuDoubleComplex,
        batchCount: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSgpsvInterleavedBatch(
        handle: cuda_types::cusparse::cusparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        ds: *mut f32,
        dl: *mut f32,
        d: *mut f32,
        du: *mut f32,
        dw: *mut f32,
        x: *mut f32,
        batchCount: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDgpsvInterleavedBatch(
        handle: cuda_types::cusparse::cusparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        ds: *mut f64,
        dl: *mut f64,
        d: *mut f64,
        du: *mut f64,
        dw: *mut f64,
        x: *mut f64,
        batchCount: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCgpsvInterleavedBatch(
        handle: cuda_types::cusparse::cusparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        ds: *mut cuda_types::cusparse::cuComplex,
        dl: *mut cuda_types::cusparse::cuComplex,
        d: *mut cuda_types::cusparse::cuComplex,
        du: *mut cuda_types::cusparse::cuComplex,
        dw: *mut cuda_types::cusparse::cuComplex,
        x: *mut cuda_types::cusparse::cuComplex,
        batchCount: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZgpsvInterleavedBatch(
        handle: cuda_types::cusparse::cusparseHandle_t,
        algo: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        ds: *mut cuda_types::cusparse::cuDoubleComplex,
        dl: *mut cuda_types::cusparse::cuDoubleComplex,
        d: *mut cuda_types::cusparse::cuDoubleComplex,
        du: *mut cuda_types::cusparse::cuDoubleComplex,
        dw: *mut cuda_types::cusparse::cuDoubleComplex,
        x: *mut cuda_types::cusparse::cuDoubleComplex,
        batchCount: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseScsrgeam2_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrSortedValA: *const f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        beta: *const f32,
        descrB: cuda_types::cusparse::cusparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrSortedValB: *const f32,
        csrSortedRowPtrB: *const ::core::ffi::c_int,
        csrSortedColIndB: *const ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *const f32,
        csrSortedRowPtrC: *const ::core::ffi::c_int,
        csrSortedColIndC: *const ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDcsrgeam2_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f64,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrSortedValA: *const f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        beta: *const f64,
        descrB: cuda_types::cusparse::cusparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrSortedValB: *const f64,
        csrSortedRowPtrB: *const ::core::ffi::c_int,
        csrSortedColIndB: *const ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *const f64,
        csrSortedRowPtrC: *const ::core::ffi::c_int,
        csrSortedColIndC: *const ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCcsrgeam2_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cusparse::cuComplex,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrSortedValA: *const cuda_types::cusparse::cuComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        beta: *const cuda_types::cusparse::cuComplex,
        descrB: cuda_types::cusparse::cusparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrSortedValB: *const cuda_types::cusparse::cuComplex,
        csrSortedRowPtrB: *const ::core::ffi::c_int,
        csrSortedColIndB: *const ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *const cuda_types::cusparse::cuComplex,
        csrSortedRowPtrC: *const ::core::ffi::c_int,
        csrSortedColIndC: *const ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZcsrgeam2_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cusparse::cuDoubleComplex,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrSortedValA: *const cuda_types::cusparse::cuDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        beta: *const cuda_types::cusparse::cuDoubleComplex,
        descrB: cuda_types::cusparse::cusparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrSortedValB: *const cuda_types::cusparse::cuDoubleComplex,
        csrSortedRowPtrB: *const ::core::ffi::c_int,
        csrSortedColIndB: *const ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *const cuda_types::cusparse::cuDoubleComplex,
        csrSortedRowPtrC: *const ::core::ffi::c_int,
        csrSortedColIndC: *const ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseXcsrgeam2Nnz(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        descrB: cuda_types::cusparse::cusparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrSortedRowPtrB: *const ::core::ffi::c_int,
        csrSortedColIndB: *const ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedRowPtrC: *mut ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
        workspace: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseScsrgeam2(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrSortedValA: *const f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        beta: *const f32,
        descrB: cuda_types::cusparse::cusparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrSortedValB: *const f32,
        csrSortedRowPtrB: *const ::core::ffi::c_int,
        csrSortedColIndB: *const ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *mut f32,
        csrSortedRowPtrC: *mut ::core::ffi::c_int,
        csrSortedColIndC: *mut ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDcsrgeam2(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f64,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrSortedValA: *const f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        beta: *const f64,
        descrB: cuda_types::cusparse::cusparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrSortedValB: *const f64,
        csrSortedRowPtrB: *const ::core::ffi::c_int,
        csrSortedColIndB: *const ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *mut f64,
        csrSortedRowPtrC: *mut ::core::ffi::c_int,
        csrSortedColIndC: *mut ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCcsrgeam2(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cusparse::cuComplex,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrSortedValA: *const cuda_types::cusparse::cuComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        beta: *const cuda_types::cusparse::cuComplex,
        descrB: cuda_types::cusparse::cusparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrSortedValB: *const cuda_types::cusparse::cuComplex,
        csrSortedRowPtrB: *const ::core::ffi::c_int,
        csrSortedColIndB: *const ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *mut cuda_types::cusparse::cuComplex,
        csrSortedRowPtrC: *mut ::core::ffi::c_int,
        csrSortedColIndC: *mut ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZcsrgeam2(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cusparse::cuDoubleComplex,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        nnzA: ::core::ffi::c_int,
        csrSortedValA: *const cuda_types::cusparse::cuDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        beta: *const cuda_types::cusparse::cuDoubleComplex,
        descrB: cuda_types::cusparse::cusparseMatDescr_t,
        nnzB: ::core::ffi::c_int,
        csrSortedValB: *const cuda_types::cusparse::cuDoubleComplex,
        csrSortedRowPtrB: *const ::core::ffi::c_int,
        csrSortedColIndB: *const ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *mut cuda_types::cusparse::cuDoubleComplex,
        csrSortedRowPtrC: *mut ::core::ffi::c_int,
        csrSortedColIndC: *mut ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseScsrcolor(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        fractionToColor: *const f32,
        ncolors: *mut ::core::ffi::c_int,
        coloring: *mut ::core::ffi::c_int,
        reordering: *mut ::core::ffi::c_int,
        info: cuda_types::cusparse::cusparseColorInfo_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDcsrcolor(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        fractionToColor: *const f64,
        ncolors: *mut ::core::ffi::c_int,
        coloring: *mut ::core::ffi::c_int,
        reordering: *mut ::core::ffi::c_int,
        info: cuda_types::cusparse::cusparseColorInfo_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCcsrcolor(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const cuda_types::cusparse::cuComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        fractionToColor: *const f32,
        ncolors: *mut ::core::ffi::c_int,
        coloring: *mut ::core::ffi::c_int,
        reordering: *mut ::core::ffi::c_int,
        info: cuda_types::cusparse::cusparseColorInfo_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZcsrcolor(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const cuda_types::cusparse::cuDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        fractionToColor: *const f64,
        ncolors: *mut ::core::ffi::c_int,
        coloring: *mut ::core::ffi::c_int,
        reordering: *mut ::core::ffi::c_int,
        info: cuda_types::cusparse::cusparseColorInfo_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSnnz(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        A: *const f32,
        lda: ::core::ffi::c_int,
        nnzPerRowCol: *mut ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDnnz(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        A: *const f64,
        lda: ::core::ffi::c_int,
        nnzPerRowCol: *mut ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCnnz(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        A: *const cuda_types::cusparse::cuComplex,
        lda: ::core::ffi::c_int,
        nnzPerRowCol: *mut ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZnnz(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        A: *const cuda_types::cusparse::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        nnzPerRowCol: *mut ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSnnz_compress(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        descr: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        nnzPerRow: *mut ::core::ffi::c_int,
        nnzC: *mut ::core::ffi::c_int,
        tol: f32,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDnnz_compress(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        descr: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        nnzPerRow: *mut ::core::ffi::c_int,
        nnzC: *mut ::core::ffi::c_int,
        tol: f64,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCnnz_compress(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        descr: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const cuda_types::cusparse::cuComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        nnzPerRow: *mut ::core::ffi::c_int,
        nnzC: *mut ::core::ffi::c_int,
        tol: cuda_types::cusparse::cuComplex,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZnnz_compress(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        descr: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const cuda_types::cusparse::cuDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        nnzPerRow: *mut ::core::ffi::c_int,
        nnzC: *mut ::core::ffi::c_int,
        tol: cuda_types::cusparse::cuDoubleComplex,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseScsr2csr_compress(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const f32,
        csrSortedColIndA: *const ::core::ffi::c_int,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        nnzPerRow: *const ::core::ffi::c_int,
        csrSortedValC: *mut f32,
        csrSortedColIndC: *mut ::core::ffi::c_int,
        csrSortedRowPtrC: *mut ::core::ffi::c_int,
        tol: f32,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDcsr2csr_compress(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const f64,
        csrSortedColIndA: *const ::core::ffi::c_int,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        nnzPerRow: *const ::core::ffi::c_int,
        csrSortedValC: *mut f64,
        csrSortedColIndC: *mut ::core::ffi::c_int,
        csrSortedRowPtrC: *mut ::core::ffi::c_int,
        tol: f64,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCcsr2csr_compress(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const cuda_types::cusparse::cuComplex,
        csrSortedColIndA: *const ::core::ffi::c_int,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        nnzPerRow: *const ::core::ffi::c_int,
        csrSortedValC: *mut cuda_types::cusparse::cuComplex,
        csrSortedColIndC: *mut ::core::ffi::c_int,
        csrSortedRowPtrC: *mut ::core::ffi::c_int,
        tol: cuda_types::cusparse::cuComplex,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZcsr2csr_compress(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const cuda_types::cusparse::cuDoubleComplex,
        csrSortedColIndA: *const ::core::ffi::c_int,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        nnzPerRow: *const ::core::ffi::c_int,
        csrSortedValC: *mut cuda_types::cusparse::cuDoubleComplex,
        csrSortedColIndC: *mut ::core::ffi::c_int,
        csrSortedRowPtrC: *mut ::core::ffi::c_int,
        tol: cuda_types::cusparse::cuDoubleComplex,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseXcoo2csr(
        handle: cuda_types::cusparse::cusparseHandle_t,
        cooRowInd: *const ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        csrSortedRowPtr: *mut ::core::ffi::c_int,
        idxBase: cuda_types::cusparse::cusparseIndexBase_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseXcsr2coo(
        handle: cuda_types::cusparse::cusparseHandle_t,
        csrSortedRowPtr: *const ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        m: ::core::ffi::c_int,
        cooRowInd: *mut ::core::ffi::c_int,
        idxBase: cuda_types::cusparse::cusparseIndexBase_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseXcsr2bsrNnz(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedRowPtrC: *mut ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseScsr2bsr(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValC: *mut f32,
        bsrSortedRowPtrC: *mut ::core::ffi::c_int,
        bsrSortedColIndC: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDcsr2bsr(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValC: *mut f64,
        bsrSortedRowPtrC: *mut ::core::ffi::c_int,
        bsrSortedColIndC: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCcsr2bsr(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const cuda_types::cusparse::cuComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValC: *mut cuda_types::cusparse::cuComplex,
        bsrSortedRowPtrC: *mut ::core::ffi::c_int,
        bsrSortedColIndC: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZcsr2bsr(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const cuda_types::cusparse::cuDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValC: *mut cuda_types::cusparse::cuDoubleComplex,
        bsrSortedRowPtrC: *mut ::core::ffi::c_int,
        bsrSortedColIndC: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSbsr2csr(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const f32,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *mut f32,
        csrSortedRowPtrC: *mut ::core::ffi::c_int,
        csrSortedColIndC: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDbsr2csr(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const f64,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *mut f64,
        csrSortedRowPtrC: *mut ::core::ffi::c_int,
        csrSortedColIndC: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCbsr2csr(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const cuda_types::cusparse::cuComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *mut cuda_types::cusparse::cuComplex,
        csrSortedRowPtrC: *mut ::core::ffi::c_int,
        csrSortedColIndC: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZbsr2csr(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const cuda_types::cusparse::cuDoubleComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        blockDim: ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *mut cuda_types::cusparse::cuDoubleComplex,
        csrSortedRowPtrC: *mut ::core::ffi::c_int,
        csrSortedColIndC: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSgebsr2gebsc_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        bsrSortedVal: *const f32,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDgebsr2gebsc_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        bsrSortedVal: *const f64,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCgebsr2gebsc_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        bsrSortedVal: *const cuda_types::cusparse::cuComplex,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZgebsr2gebsc_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        bsrSortedVal: *const cuda_types::cusparse::cuDoubleComplex,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSgebsr2gebsc_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        bsrSortedVal: *const f32,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDgebsr2gebsc_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        bsrSortedVal: *const f64,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCgebsr2gebsc_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        bsrSortedVal: *const cuda_types::cusparse::cuComplex,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZgebsr2gebsc_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        bsrSortedVal: *const cuda_types::cusparse::cuDoubleComplex,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSgebsr2gebsc(
        handle: cuda_types::cusparse::cusparseHandle_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        bsrSortedVal: *const f32,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        bscVal: *mut f32,
        bscRowInd: *mut ::core::ffi::c_int,
        bscColPtr: *mut ::core::ffi::c_int,
        copyValues: cuda_types::cusparse::cusparseAction_t,
        idxBase: cuda_types::cusparse::cusparseIndexBase_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDgebsr2gebsc(
        handle: cuda_types::cusparse::cusparseHandle_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        bsrSortedVal: *const f64,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        bscVal: *mut f64,
        bscRowInd: *mut ::core::ffi::c_int,
        bscColPtr: *mut ::core::ffi::c_int,
        copyValues: cuda_types::cusparse::cusparseAction_t,
        idxBase: cuda_types::cusparse::cusparseIndexBase_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCgebsr2gebsc(
        handle: cuda_types::cusparse::cusparseHandle_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        bsrSortedVal: *const cuda_types::cusparse::cuComplex,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        bscVal: *mut cuda_types::cusparse::cuComplex,
        bscRowInd: *mut ::core::ffi::c_int,
        bscColPtr: *mut ::core::ffi::c_int,
        copyValues: cuda_types::cusparse::cusparseAction_t,
        idxBase: cuda_types::cusparse::cusparseIndexBase_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZgebsr2gebsc(
        handle: cuda_types::cusparse::cusparseHandle_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        bsrSortedVal: *const cuda_types::cusparse::cuDoubleComplex,
        bsrSortedRowPtr: *const ::core::ffi::c_int,
        bsrSortedColInd: *const ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        bscVal: *mut cuda_types::cusparse::cuDoubleComplex,
        bscRowInd: *mut ::core::ffi::c_int,
        bscColPtr: *mut ::core::ffi::c_int,
        copyValues: cuda_types::cusparse::cusparseAction_t,
        idxBase: cuda_types::cusparse::cusparseIndexBase_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseXgebsr2csr(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedRowPtrC: *mut ::core::ffi::c_int,
        csrSortedColIndC: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSgebsr2csr(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const f32,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *mut f32,
        csrSortedRowPtrC: *mut ::core::ffi::c_int,
        csrSortedColIndC: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDgebsr2csr(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const f64,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *mut f64,
        csrSortedRowPtrC: *mut ::core::ffi::c_int,
        csrSortedColIndC: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCgebsr2csr(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const cuda_types::cusparse::cuComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *mut cuda_types::cusparse::cuComplex,
        csrSortedRowPtrC: *mut ::core::ffi::c_int,
        csrSortedColIndC: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZgebsr2csr(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const cuda_types::cusparse::cuDoubleComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *mut cuda_types::cusparse::cuDoubleComplex,
        csrSortedRowPtrC: *mut ::core::ffi::c_int,
        csrSortedColIndC: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseScsr2gebsr_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDcsr2gebsr_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCcsr2gebsr_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const cuda_types::cusparse::cuComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZcsr2gebsr_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const cuda_types::cusparse::cuDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseScsr2gebsr_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDcsr2gebsr_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCcsr2gebsr_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const cuda_types::cusparse::cuComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZcsr2gebsr_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const cuda_types::cusparse::cuDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseXcsr2gebsrNnz(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedRowPtrC: *mut ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseScsr2gebsr(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValC: *mut f32,
        bsrSortedRowPtrC: *mut ::core::ffi::c_int,
        bsrSortedColIndC: *mut ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDcsr2gebsr(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValC: *mut f64,
        bsrSortedRowPtrC: *mut ::core::ffi::c_int,
        bsrSortedColIndC: *mut ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCcsr2gebsr(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const cuda_types::cusparse::cuComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValC: *mut cuda_types::cusparse::cuComplex,
        bsrSortedRowPtrC: *mut ::core::ffi::c_int,
        bsrSortedColIndC: *mut ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZcsr2gebsr(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const cuda_types::cusparse::cuDoubleComplex,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValC: *mut cuda_types::cusparse::cuDoubleComplex,
        bsrSortedRowPtrC: *mut ::core::ffi::c_int,
        bsrSortedColIndC: *mut ::core::ffi::c_int,
        rowBlockDim: ::core::ffi::c_int,
        colBlockDim: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSgebsr2gebsr_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const f32,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        rowBlockDimA: ::core::ffi::c_int,
        colBlockDimA: ::core::ffi::c_int,
        rowBlockDimC: ::core::ffi::c_int,
        colBlockDimC: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDgebsr2gebsr_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const f64,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        rowBlockDimA: ::core::ffi::c_int,
        colBlockDimA: ::core::ffi::c_int,
        rowBlockDimC: ::core::ffi::c_int,
        colBlockDimC: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCgebsr2gebsr_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const cuda_types::cusparse::cuComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        rowBlockDimA: ::core::ffi::c_int,
        colBlockDimA: ::core::ffi::c_int,
        rowBlockDimC: ::core::ffi::c_int,
        colBlockDimC: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZgebsr2gebsr_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const cuda_types::cusparse::cuDoubleComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        rowBlockDimA: ::core::ffi::c_int,
        colBlockDimA: ::core::ffi::c_int,
        rowBlockDimC: ::core::ffi::c_int,
        colBlockDimC: ::core::ffi::c_int,
        pBufferSizeInBytes: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSgebsr2gebsr_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const f32,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        rowBlockDimA: ::core::ffi::c_int,
        colBlockDimA: ::core::ffi::c_int,
        rowBlockDimC: ::core::ffi::c_int,
        colBlockDimC: ::core::ffi::c_int,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDgebsr2gebsr_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const f64,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        rowBlockDimA: ::core::ffi::c_int,
        colBlockDimA: ::core::ffi::c_int,
        rowBlockDimC: ::core::ffi::c_int,
        colBlockDimC: ::core::ffi::c_int,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCgebsr2gebsr_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const cuda_types::cusparse::cuComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        rowBlockDimA: ::core::ffi::c_int,
        colBlockDimA: ::core::ffi::c_int,
        rowBlockDimC: ::core::ffi::c_int,
        colBlockDimC: ::core::ffi::c_int,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZgebsr2gebsr_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const cuda_types::cusparse::cuDoubleComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        rowBlockDimA: ::core::ffi::c_int,
        colBlockDimA: ::core::ffi::c_int,
        rowBlockDimC: ::core::ffi::c_int,
        colBlockDimC: ::core::ffi::c_int,
        pBufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseXgebsr2gebsrNnz(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        rowBlockDimA: ::core::ffi::c_int,
        colBlockDimA: ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedRowPtrC: *mut ::core::ffi::c_int,
        rowBlockDimC: ::core::ffi::c_int,
        colBlockDimC: ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSgebsr2gebsr(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const f32,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        rowBlockDimA: ::core::ffi::c_int,
        colBlockDimA: ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValC: *mut f32,
        bsrSortedRowPtrC: *mut ::core::ffi::c_int,
        bsrSortedColIndC: *mut ::core::ffi::c_int,
        rowBlockDimC: ::core::ffi::c_int,
        colBlockDimC: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDgebsr2gebsr(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const f64,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        rowBlockDimA: ::core::ffi::c_int,
        colBlockDimA: ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValC: *mut f64,
        bsrSortedRowPtrC: *mut ::core::ffi::c_int,
        bsrSortedColIndC: *mut ::core::ffi::c_int,
        rowBlockDimC: ::core::ffi::c_int,
        colBlockDimC: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCgebsr2gebsr(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const cuda_types::cusparse::cuComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        rowBlockDimA: ::core::ffi::c_int,
        colBlockDimA: ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValC: *mut cuda_types::cusparse::cuComplex,
        bsrSortedRowPtrC: *mut ::core::ffi::c_int,
        bsrSortedColIndC: *mut ::core::ffi::c_int,
        rowBlockDimC: ::core::ffi::c_int,
        colBlockDimC: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZgebsr2gebsr(
        handle: cuda_types::cusparse::cusparseHandle_t,
        dirA: cuda_types::cusparse::cusparseDirection_t,
        mb: ::core::ffi::c_int,
        nb: ::core::ffi::c_int,
        nnzb: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValA: *const cuda_types::cusparse::cuDoubleComplex,
        bsrSortedRowPtrA: *const ::core::ffi::c_int,
        bsrSortedColIndA: *const ::core::ffi::c_int,
        rowBlockDimA: ::core::ffi::c_int,
        colBlockDimA: ::core::ffi::c_int,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        bsrSortedValC: *mut cuda_types::cusparse::cuDoubleComplex,
        bsrSortedRowPtrC: *mut ::core::ffi::c_int,
        bsrSortedColIndC: *mut ::core::ffi::c_int,
        rowBlockDimC: ::core::ffi::c_int,
        colBlockDimC: ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCreateIdentityPermutation(
        handle: cuda_types::cusparse::cusparseHandle_t,
        n: ::core::ffi::c_int,
        p: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseXcoosort_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        cooRowsA: *const ::core::ffi::c_int,
        cooColsA: *const ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseXcoosortByRow(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        cooRowsA: *mut ::core::ffi::c_int,
        cooColsA: *mut ::core::ffi::c_int,
        P: *mut ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseXcoosortByColumn(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        cooRowsA: *mut ::core::ffi::c_int,
        cooColsA: *mut ::core::ffi::c_int,
        P: *mut ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseXcsrsort_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *const ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseXcsrsort(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrRowPtrA: *const ::core::ffi::c_int,
        csrColIndA: *mut ::core::ffi::c_int,
        P: *mut ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseXcscsort_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        cscColPtrA: *const ::core::ffi::c_int,
        cscRowIndA: *const ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseXcscsort(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        cscColPtrA: *const ::core::ffi::c_int,
        cscRowIndA: *mut ::core::ffi::c_int,
        P: *mut ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseScsru2csr_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        csrVal: *mut f32,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *mut ::core::ffi::c_int,
        info: cuda_types::cusparse::csru2csrInfo_t,
        pBufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDcsru2csr_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        csrVal: *mut f64,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *mut ::core::ffi::c_int,
        info: cuda_types::cusparse::csru2csrInfo_t,
        pBufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCcsru2csr_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        csrVal: *mut cuda_types::cusparse::cuComplex,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *mut ::core::ffi::c_int,
        info: cuda_types::cusparse::csru2csrInfo_t,
        pBufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZcsru2csr_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        csrVal: *mut cuda_types::cusparse::cuDoubleComplex,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *mut ::core::ffi::c_int,
        info: cuda_types::cusparse::csru2csrInfo_t,
        pBufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseScsru2csr(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrVal: *mut f32,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *mut ::core::ffi::c_int,
        info: cuda_types::cusparse::csru2csrInfo_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDcsru2csr(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrVal: *mut f64,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *mut ::core::ffi::c_int,
        info: cuda_types::cusparse::csru2csrInfo_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCcsru2csr(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrVal: *mut cuda_types::cusparse::cuComplex,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *mut ::core::ffi::c_int,
        info: cuda_types::cusparse::csru2csrInfo_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZcsru2csr(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrVal: *mut cuda_types::cusparse::cuDoubleComplex,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *mut ::core::ffi::c_int,
        info: cuda_types::cusparse::csru2csrInfo_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseScsr2csru(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrVal: *mut f32,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *mut ::core::ffi::c_int,
        info: cuda_types::cusparse::csru2csrInfo_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDcsr2csru(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrVal: *mut f64,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *mut ::core::ffi::c_int,
        info: cuda_types::cusparse::csru2csrInfo_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCcsr2csru(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrVal: *mut cuda_types::cusparse::cuComplex,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *mut ::core::ffi::c_int,
        info: cuda_types::cusparse::csru2csrInfo_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseZcsr2csru(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrVal: *mut cuda_types::cusparse::cuDoubleComplex,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *mut ::core::ffi::c_int,
        info: cuda_types::cusparse::csru2csrInfo_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpruneDense2csr_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const f32,
        lda: ::core::ffi::c_int,
        threshold: *const f32,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *const f32,
        csrSortedRowPtrC: *const ::core::ffi::c_int,
        csrSortedColIndC: *const ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDpruneDense2csr_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const f64,
        lda: ::core::ffi::c_int,
        threshold: *const f64,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *const f64,
        csrSortedRowPtrC: *const ::core::ffi::c_int,
        csrSortedColIndC: *const ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpruneDense2csrNnz(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const f32,
        lda: ::core::ffi::c_int,
        threshold: *const f32,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrRowPtrC: *mut ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDpruneDense2csrNnz(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const f64,
        lda: ::core::ffi::c_int,
        threshold: *const f64,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedRowPtrC: *mut ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpruneDense2csr(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const f32,
        lda: ::core::ffi::c_int,
        threshold: *const f32,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *mut f32,
        csrSortedRowPtrC: *const ::core::ffi::c_int,
        csrSortedColIndC: *mut ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDpruneDense2csr(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const f64,
        lda: ::core::ffi::c_int,
        threshold: *const f64,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *mut f64,
        csrSortedRowPtrC: *const ::core::ffi::c_int,
        csrSortedColIndC: *mut ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpruneCsr2csr_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        threshold: *const f32,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *const f32,
        csrSortedRowPtrC: *const ::core::ffi::c_int,
        csrSortedColIndC: *const ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDpruneCsr2csr_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        threshold: *const f64,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *const f64,
        csrSortedRowPtrC: *const ::core::ffi::c_int,
        csrSortedColIndC: *const ::core::ffi::c_int,
        pBufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpruneCsr2csrNnz(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        threshold: *const f32,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedRowPtrC: *mut ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDpruneCsr2csrNnz(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        threshold: *const f64,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedRowPtrC: *mut ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpruneCsr2csr(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        threshold: *const f32,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *mut f32,
        csrSortedRowPtrC: *const ::core::ffi::c_int,
        csrSortedColIndC: *mut ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDpruneCsr2csr(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        threshold: *const f64,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *mut f64,
        csrSortedRowPtrC: *const ::core::ffi::c_int,
        csrSortedColIndC: *mut ::core::ffi::c_int,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpruneDense2csrByPercentage_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const f32,
        lda: ::core::ffi::c_int,
        percentage: f32,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *const f32,
        csrSortedRowPtrC: *const ::core::ffi::c_int,
        csrSortedColIndC: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::pruneInfo_t,
        pBufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDpruneDense2csrByPercentage_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const f64,
        lda: ::core::ffi::c_int,
        percentage: f32,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *const f64,
        csrSortedRowPtrC: *const ::core::ffi::c_int,
        csrSortedColIndC: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::pruneInfo_t,
        pBufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpruneDense2csrNnzByPercentage(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const f32,
        lda: ::core::ffi::c_int,
        percentage: f32,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrRowPtrC: *mut ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
        info: cuda_types::cusparse::pruneInfo_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDpruneDense2csrNnzByPercentage(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const f64,
        lda: ::core::ffi::c_int,
        percentage: f32,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrRowPtrC: *mut ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
        info: cuda_types::cusparse::pruneInfo_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpruneDense2csrByPercentage(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const f32,
        lda: ::core::ffi::c_int,
        percentage: f32,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *mut f32,
        csrSortedRowPtrC: *const ::core::ffi::c_int,
        csrSortedColIndC: *mut ::core::ffi::c_int,
        info: cuda_types::cusparse::pruneInfo_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDpruneDense2csrByPercentage(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const f64,
        lda: ::core::ffi::c_int,
        percentage: f32,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *mut f64,
        csrSortedRowPtrC: *const ::core::ffi::c_int,
        csrSortedColIndC: *mut ::core::ffi::c_int,
        info: cuda_types::cusparse::pruneInfo_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpruneCsr2csrByPercentage_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        percentage: f32,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *const f32,
        csrSortedRowPtrC: *const ::core::ffi::c_int,
        csrSortedColIndC: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::pruneInfo_t,
        pBufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDpruneCsr2csrByPercentage_bufferSizeExt(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        percentage: f32,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *const f64,
        csrSortedRowPtrC: *const ::core::ffi::c_int,
        csrSortedColIndC: *const ::core::ffi::c_int,
        info: cuda_types::cusparse::pruneInfo_t,
        pBufferSizeInBytes: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpruneCsr2csrNnzByPercentage(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        percentage: f32,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedRowPtrC: *mut ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
        info: cuda_types::cusparse::pruneInfo_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDpruneCsr2csrNnzByPercentage(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        percentage: f32,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedRowPtrC: *mut ::core::ffi::c_int,
        nnzTotalDevHostPtr: *mut ::core::ffi::c_int,
        info: cuda_types::cusparse::pruneInfo_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpruneCsr2csrByPercentage(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const f32,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        percentage: f32,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *mut f32,
        csrSortedRowPtrC: *const ::core::ffi::c_int,
        csrSortedColIndC: *mut ::core::ffi::c_int,
        info: cuda_types::cusparse::pruneInfo_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDpruneCsr2csrByPercentage(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnzA: ::core::ffi::c_int,
        descrA: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValA: *const f64,
        csrSortedRowPtrA: *const ::core::ffi::c_int,
        csrSortedColIndA: *const ::core::ffi::c_int,
        percentage: f32,
        descrC: cuda_types::cusparse::cusparseMatDescr_t,
        csrSortedValC: *mut f64,
        csrSortedRowPtrC: *const ::core::ffi::c_int,
        csrSortedColIndC: *mut ::core::ffi::c_int,
        info: cuda_types::cusparse::pruneInfo_t,
        pBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCsr2cscEx2(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        csrVal: *const ::core::ffi::c_void,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *const ::core::ffi::c_int,
        cscVal: *mut ::core::ffi::c_void,
        cscColPtr: *mut ::core::ffi::c_int,
        cscRowInd: *mut ::core::ffi::c_int,
        valType: cuda_types::cusparse::cudaDataType,
        copyValues: cuda_types::cusparse::cusparseAction_t,
        idxBase: cuda_types::cusparse::cusparseIndexBase_t,
        alg: cuda_types::cusparse::cusparseCsr2CscAlg_t,
        buffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCsr2cscEx2_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nnz: ::core::ffi::c_int,
        csrVal: *const ::core::ffi::c_void,
        csrRowPtr: *const ::core::ffi::c_int,
        csrColInd: *const ::core::ffi::c_int,
        cscVal: *mut ::core::ffi::c_void,
        cscColPtr: *mut ::core::ffi::c_int,
        cscRowInd: *mut ::core::ffi::c_int,
        valType: cuda_types::cusparse::cudaDataType,
        copyValues: cuda_types::cusparse::cusparseAction_t,
        idxBase: cuda_types::cusparse::cusparseIndexBase_t,
        alg: cuda_types::cusparse::cusparseCsr2CscAlg_t,
        bufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCreateSpVec(
        spVecDescr: *mut cuda_types::cusparse::cusparseSpVecDescr_t,
        size: i64,
        nnz: i64,
        indices: *mut ::core::ffi::c_void,
        values: *mut ::core::ffi::c_void,
        idxType: cuda_types::cusparse::cusparseIndexType_t,
        idxBase: cuda_types::cusparse::cusparseIndexBase_t,
        valueType: cuda_types::cusparse::cudaDataType,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCreateConstSpVec(
        spVecDescr: *mut cuda_types::cusparse::cusparseConstSpVecDescr_t,
        size: i64,
        nnz: i64,
        indices: *const ::core::ffi::c_void,
        values: *const ::core::ffi::c_void,
        idxType: cuda_types::cusparse::cusparseIndexType_t,
        idxBase: cuda_types::cusparse::cusparseIndexBase_t,
        valueType: cuda_types::cusparse::cudaDataType,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDestroySpVec(
        spVecDescr: cuda_types::cusparse::cusparseConstSpVecDescr_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpVecGet(
        spVecDescr: cuda_types::cusparse::cusparseSpVecDescr_t,
        size: *mut i64,
        nnz: *mut i64,
        indices: *mut *mut ::core::ffi::c_void,
        values: *mut *mut ::core::ffi::c_void,
        idxType: *mut cuda_types::cusparse::cusparseIndexType_t,
        idxBase: *mut cuda_types::cusparse::cusparseIndexBase_t,
        valueType: *mut cuda_types::cusparse::cudaDataType,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseConstSpVecGet(
        spVecDescr: cuda_types::cusparse::cusparseConstSpVecDescr_t,
        size: *mut i64,
        nnz: *mut i64,
        indices: *mut *const ::core::ffi::c_void,
        values: *mut *const ::core::ffi::c_void,
        idxType: *mut cuda_types::cusparse::cusparseIndexType_t,
        idxBase: *mut cuda_types::cusparse::cusparseIndexBase_t,
        valueType: *mut cuda_types::cusparse::cudaDataType,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpVecGetIndexBase(
        spVecDescr: cuda_types::cusparse::cusparseConstSpVecDescr_t,
        idxBase: *mut cuda_types::cusparse::cusparseIndexBase_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpVecGetValues(
        spVecDescr: cuda_types::cusparse::cusparseSpVecDescr_t,
        values: *mut *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseConstSpVecGetValues(
        spVecDescr: cuda_types::cusparse::cusparseConstSpVecDescr_t,
        values: *mut *const ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpVecSetValues(
        spVecDescr: cuda_types::cusparse::cusparseSpVecDescr_t,
        values: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCreateDnVec(
        dnVecDescr: *mut cuda_types::cusparse::cusparseDnVecDescr_t,
        size: i64,
        values: *mut ::core::ffi::c_void,
        valueType: cuda_types::cusparse::cudaDataType,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCreateConstDnVec(
        dnVecDescr: *mut cuda_types::cusparse::cusparseConstDnVecDescr_t,
        size: i64,
        values: *const ::core::ffi::c_void,
        valueType: cuda_types::cusparse::cudaDataType,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDestroyDnVec(
        dnVecDescr: cuda_types::cusparse::cusparseConstDnVecDescr_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDnVecGet(
        dnVecDescr: cuda_types::cusparse::cusparseDnVecDescr_t,
        size: *mut i64,
        values: *mut *mut ::core::ffi::c_void,
        valueType: *mut cuda_types::cusparse::cudaDataType,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseConstDnVecGet(
        dnVecDescr: cuda_types::cusparse::cusparseConstDnVecDescr_t,
        size: *mut i64,
        values: *mut *const ::core::ffi::c_void,
        valueType: *mut cuda_types::cusparse::cudaDataType,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDnVecGetValues(
        dnVecDescr: cuda_types::cusparse::cusparseDnVecDescr_t,
        values: *mut *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseConstDnVecGetValues(
        dnVecDescr: cuda_types::cusparse::cusparseConstDnVecDescr_t,
        values: *mut *const ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDnVecSetValues(
        dnVecDescr: cuda_types::cusparse::cusparseDnVecDescr_t,
        values: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDestroySpMat(
        spMatDescr: cuda_types::cusparse::cusparseConstSpMatDescr_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpMatGetFormat(
        spMatDescr: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        format: *mut cuda_types::cusparse::cusparseFormat_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpMatGetIndexBase(
        spMatDescr: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        idxBase: *mut cuda_types::cusparse::cusparseIndexBase_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpMatGetValues(
        spMatDescr: cuda_types::cusparse::cusparseSpMatDescr_t,
        values: *mut *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseConstSpMatGetValues(
        spMatDescr: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        values: *mut *const ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpMatSetValues(
        spMatDescr: cuda_types::cusparse::cusparseSpMatDescr_t,
        values: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpMatGetSize(
        spMatDescr: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        rows: *mut i64,
        cols: *mut i64,
        nnz: *mut i64,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpMatGetStridedBatch(
        spMatDescr: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        batchCount: *mut ::core::ffi::c_int,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCooSetStridedBatch(
        spMatDescr: cuda_types::cusparse::cusparseSpMatDescr_t,
        batchCount: ::core::ffi::c_int,
        batchStride: i64,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCsrSetStridedBatch(
        spMatDescr: cuda_types::cusparse::cusparseSpMatDescr_t,
        batchCount: ::core::ffi::c_int,
        offsetsBatchStride: i64,
        columnsValuesBatchStride: i64,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseBsrSetStridedBatch(
        spMatDescr: cuda_types::cusparse::cusparseSpMatDescr_t,
        batchCount: ::core::ffi::c_int,
        offsetsBatchStride: i64,
        columnsBatchStride: i64,
        ValuesBatchStride: i64,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpMatGetAttribute(
        spMatDescr: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        attribute: cuda_types::cusparse::cusparseSpMatAttribute_t,
        data: *mut ::core::ffi::c_void,
        dataSize: usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpMatSetAttribute(
        spMatDescr: cuda_types::cusparse::cusparseSpMatDescr_t,
        attribute: cuda_types::cusparse::cusparseSpMatAttribute_t,
        data: *mut ::core::ffi::c_void,
        dataSize: usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCreateCsr(
        spMatDescr: *mut cuda_types::cusparse::cusparseSpMatDescr_t,
        rows: i64,
        cols: i64,
        nnz: i64,
        csrRowOffsets: *mut ::core::ffi::c_void,
        csrColInd: *mut ::core::ffi::c_void,
        csrValues: *mut ::core::ffi::c_void,
        csrRowOffsetsType: cuda_types::cusparse::cusparseIndexType_t,
        csrColIndType: cuda_types::cusparse::cusparseIndexType_t,
        idxBase: cuda_types::cusparse::cusparseIndexBase_t,
        valueType: cuda_types::cusparse::cudaDataType,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCreateConstCsr(
        spMatDescr: *mut cuda_types::cusparse::cusparseConstSpMatDescr_t,
        rows: i64,
        cols: i64,
        nnz: i64,
        csrRowOffsets: *const ::core::ffi::c_void,
        csrColInd: *const ::core::ffi::c_void,
        csrValues: *const ::core::ffi::c_void,
        csrRowOffsetsType: cuda_types::cusparse::cusparseIndexType_t,
        csrColIndType: cuda_types::cusparse::cusparseIndexType_t,
        idxBase: cuda_types::cusparse::cusparseIndexBase_t,
        valueType: cuda_types::cusparse::cudaDataType,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCreateCsc(
        spMatDescr: *mut cuda_types::cusparse::cusparseSpMatDescr_t,
        rows: i64,
        cols: i64,
        nnz: i64,
        cscColOffsets: *mut ::core::ffi::c_void,
        cscRowInd: *mut ::core::ffi::c_void,
        cscValues: *mut ::core::ffi::c_void,
        cscColOffsetsType: cuda_types::cusparse::cusparseIndexType_t,
        cscRowIndType: cuda_types::cusparse::cusparseIndexType_t,
        idxBase: cuda_types::cusparse::cusparseIndexBase_t,
        valueType: cuda_types::cusparse::cudaDataType,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCreateConstCsc(
        spMatDescr: *mut cuda_types::cusparse::cusparseConstSpMatDescr_t,
        rows: i64,
        cols: i64,
        nnz: i64,
        cscColOffsets: *const ::core::ffi::c_void,
        cscRowInd: *const ::core::ffi::c_void,
        cscValues: *const ::core::ffi::c_void,
        cscColOffsetsType: cuda_types::cusparse::cusparseIndexType_t,
        cscRowIndType: cuda_types::cusparse::cusparseIndexType_t,
        idxBase: cuda_types::cusparse::cusparseIndexBase_t,
        valueType: cuda_types::cusparse::cudaDataType,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCsrGet(
        spMatDescr: cuda_types::cusparse::cusparseSpMatDescr_t,
        rows: *mut i64,
        cols: *mut i64,
        nnz: *mut i64,
        csrRowOffsets: *mut *mut ::core::ffi::c_void,
        csrColInd: *mut *mut ::core::ffi::c_void,
        csrValues: *mut *mut ::core::ffi::c_void,
        csrRowOffsetsType: *mut cuda_types::cusparse::cusparseIndexType_t,
        csrColIndType: *mut cuda_types::cusparse::cusparseIndexType_t,
        idxBase: *mut cuda_types::cusparse::cusparseIndexBase_t,
        valueType: *mut cuda_types::cusparse::cudaDataType,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseConstCsrGet(
        spMatDescr: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        rows: *mut i64,
        cols: *mut i64,
        nnz: *mut i64,
        csrRowOffsets: *mut *const ::core::ffi::c_void,
        csrColInd: *mut *const ::core::ffi::c_void,
        csrValues: *mut *const ::core::ffi::c_void,
        csrRowOffsetsType: *mut cuda_types::cusparse::cusparseIndexType_t,
        csrColIndType: *mut cuda_types::cusparse::cusparseIndexType_t,
        idxBase: *mut cuda_types::cusparse::cusparseIndexBase_t,
        valueType: *mut cuda_types::cusparse::cudaDataType,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCscGet(
        spMatDescr: cuda_types::cusparse::cusparseSpMatDescr_t,
        rows: *mut i64,
        cols: *mut i64,
        nnz: *mut i64,
        cscColOffsets: *mut *mut ::core::ffi::c_void,
        cscRowInd: *mut *mut ::core::ffi::c_void,
        cscValues: *mut *mut ::core::ffi::c_void,
        cscColOffsetsType: *mut cuda_types::cusparse::cusparseIndexType_t,
        cscRowIndType: *mut cuda_types::cusparse::cusparseIndexType_t,
        idxBase: *mut cuda_types::cusparse::cusparseIndexBase_t,
        valueType: *mut cuda_types::cusparse::cudaDataType,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseConstCscGet(
        spMatDescr: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        rows: *mut i64,
        cols: *mut i64,
        nnz: *mut i64,
        cscColOffsets: *mut *const ::core::ffi::c_void,
        cscRowInd: *mut *const ::core::ffi::c_void,
        cscValues: *mut *const ::core::ffi::c_void,
        cscColOffsetsType: *mut cuda_types::cusparse::cusparseIndexType_t,
        cscRowIndType: *mut cuda_types::cusparse::cusparseIndexType_t,
        idxBase: *mut cuda_types::cusparse::cusparseIndexBase_t,
        valueType: *mut cuda_types::cusparse::cudaDataType,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCsrSetPointers(
        spMatDescr: cuda_types::cusparse::cusparseSpMatDescr_t,
        csrRowOffsets: *mut ::core::ffi::c_void,
        csrColInd: *mut ::core::ffi::c_void,
        csrValues: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCscSetPointers(
        spMatDescr: cuda_types::cusparse::cusparseSpMatDescr_t,
        cscColOffsets: *mut ::core::ffi::c_void,
        cscRowInd: *mut ::core::ffi::c_void,
        cscValues: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCreateBsr(
        spMatDescr: *mut cuda_types::cusparse::cusparseSpMatDescr_t,
        brows: i64,
        bcols: i64,
        bnnz: i64,
        rowBlockSize: i64,
        colBlockSize: i64,
        bsrRowOffsets: *mut ::core::ffi::c_void,
        bsrColInd: *mut ::core::ffi::c_void,
        bsrValues: *mut ::core::ffi::c_void,
        bsrRowOffsetsType: cuda_types::cusparse::cusparseIndexType_t,
        bsrColIndType: cuda_types::cusparse::cusparseIndexType_t,
        idxBase: cuda_types::cusparse::cusparseIndexBase_t,
        valueType: cuda_types::cusparse::cudaDataType,
        order: cuda_types::cusparse::cusparseOrder_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCreateConstBsr(
        spMatDescr: *mut cuda_types::cusparse::cusparseConstSpMatDescr_t,
        brows: i64,
        bcols: i64,
        bnnz: i64,
        rowBlockDim: i64,
        colBlockDim: i64,
        bsrRowOffsets: *const ::core::ffi::c_void,
        bsrColInd: *const ::core::ffi::c_void,
        bsrValues: *const ::core::ffi::c_void,
        bsrRowOffsetsType: cuda_types::cusparse::cusparseIndexType_t,
        bsrColIndType: cuda_types::cusparse::cusparseIndexType_t,
        idxBase: cuda_types::cusparse::cusparseIndexBase_t,
        valueType: cuda_types::cusparse::cudaDataType,
        order: cuda_types::cusparse::cusparseOrder_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCreateCoo(
        spMatDescr: *mut cuda_types::cusparse::cusparseSpMatDescr_t,
        rows: i64,
        cols: i64,
        nnz: i64,
        cooRowInd: *mut ::core::ffi::c_void,
        cooColInd: *mut ::core::ffi::c_void,
        cooValues: *mut ::core::ffi::c_void,
        cooIdxType: cuda_types::cusparse::cusparseIndexType_t,
        idxBase: cuda_types::cusparse::cusparseIndexBase_t,
        valueType: cuda_types::cusparse::cudaDataType,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCreateConstCoo(
        spMatDescr: *mut cuda_types::cusparse::cusparseConstSpMatDescr_t,
        rows: i64,
        cols: i64,
        nnz: i64,
        cooRowInd: *const ::core::ffi::c_void,
        cooColInd: *const ::core::ffi::c_void,
        cooValues: *const ::core::ffi::c_void,
        cooIdxType: cuda_types::cusparse::cusparseIndexType_t,
        idxBase: cuda_types::cusparse::cusparseIndexBase_t,
        valueType: cuda_types::cusparse::cudaDataType,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCooGet(
        spMatDescr: cuda_types::cusparse::cusparseSpMatDescr_t,
        rows: *mut i64,
        cols: *mut i64,
        nnz: *mut i64,
        cooRowInd: *mut *mut ::core::ffi::c_void,
        cooColInd: *mut *mut ::core::ffi::c_void,
        cooValues: *mut *mut ::core::ffi::c_void,
        idxType: *mut cuda_types::cusparse::cusparseIndexType_t,
        idxBase: *mut cuda_types::cusparse::cusparseIndexBase_t,
        valueType: *mut cuda_types::cusparse::cudaDataType,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseConstCooGet(
        spMatDescr: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        rows: *mut i64,
        cols: *mut i64,
        nnz: *mut i64,
        cooRowInd: *mut *const ::core::ffi::c_void,
        cooColInd: *mut *const ::core::ffi::c_void,
        cooValues: *mut *const ::core::ffi::c_void,
        idxType: *mut cuda_types::cusparse::cusparseIndexType_t,
        idxBase: *mut cuda_types::cusparse::cusparseIndexBase_t,
        valueType: *mut cuda_types::cusparse::cudaDataType,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCooSetPointers(
        spMatDescr: cuda_types::cusparse::cusparseSpMatDescr_t,
        cooRows: *mut ::core::ffi::c_void,
        cooColumns: *mut ::core::ffi::c_void,
        cooValues: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCreateBlockedEll(
        spMatDescr: *mut cuda_types::cusparse::cusparseSpMatDescr_t,
        rows: i64,
        cols: i64,
        ellBlockSize: i64,
        ellCols: i64,
        ellColInd: *mut ::core::ffi::c_void,
        ellValue: *mut ::core::ffi::c_void,
        ellIdxType: cuda_types::cusparse::cusparseIndexType_t,
        idxBase: cuda_types::cusparse::cusparseIndexBase_t,
        valueType: cuda_types::cusparse::cudaDataType,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCreateConstBlockedEll(
        spMatDescr: *mut cuda_types::cusparse::cusparseConstSpMatDescr_t,
        rows: i64,
        cols: i64,
        ellBlockSize: i64,
        ellCols: i64,
        ellColInd: *const ::core::ffi::c_void,
        ellValue: *const ::core::ffi::c_void,
        ellIdxType: cuda_types::cusparse::cusparseIndexType_t,
        idxBase: cuda_types::cusparse::cusparseIndexBase_t,
        valueType: cuda_types::cusparse::cudaDataType,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseBlockedEllGet(
        spMatDescr: cuda_types::cusparse::cusparseSpMatDescr_t,
        rows: *mut i64,
        cols: *mut i64,
        ellBlockSize: *mut i64,
        ellCols: *mut i64,
        ellColInd: *mut *mut ::core::ffi::c_void,
        ellValue: *mut *mut ::core::ffi::c_void,
        ellIdxType: *mut cuda_types::cusparse::cusparseIndexType_t,
        idxBase: *mut cuda_types::cusparse::cusparseIndexBase_t,
        valueType: *mut cuda_types::cusparse::cudaDataType,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseConstBlockedEllGet(
        spMatDescr: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        rows: *mut i64,
        cols: *mut i64,
        ellBlockSize: *mut i64,
        ellCols: *mut i64,
        ellColInd: *mut *const ::core::ffi::c_void,
        ellValue: *mut *const ::core::ffi::c_void,
        ellIdxType: *mut cuda_types::cusparse::cusparseIndexType_t,
        idxBase: *mut cuda_types::cusparse::cusparseIndexBase_t,
        valueType: *mut cuda_types::cusparse::cudaDataType,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCreateSlicedEll(
        spMatDescr: *mut cuda_types::cusparse::cusparseSpMatDescr_t,
        rows: i64,
        cols: i64,
        nnz: i64,
        sellValuesSize: i64,
        sliceSize: i64,
        sellSliceOffsets: *mut ::core::ffi::c_void,
        sellColInd: *mut ::core::ffi::c_void,
        sellValues: *mut ::core::ffi::c_void,
        sellSliceOffsetsType: cuda_types::cusparse::cusparseIndexType_t,
        sellColIndType: cuda_types::cusparse::cusparseIndexType_t,
        idxBase: cuda_types::cusparse::cusparseIndexBase_t,
        valueType: cuda_types::cusparse::cudaDataType,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCreateConstSlicedEll(
        spMatDescr: *mut cuda_types::cusparse::cusparseConstSpMatDescr_t,
        rows: i64,
        cols: i64,
        nnz: i64,
        sellValuesSize: i64,
        sliceSize: i64,
        sellSliceOffsets: *const ::core::ffi::c_void,
        sellColInd: *const ::core::ffi::c_void,
        sellValues: *const ::core::ffi::c_void,
        sellSliceOffsetsType: cuda_types::cusparse::cusparseIndexType_t,
        sellColIndType: cuda_types::cusparse::cusparseIndexType_t,
        idxBase: cuda_types::cusparse::cusparseIndexBase_t,
        valueType: cuda_types::cusparse::cudaDataType,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCreateDnMat(
        dnMatDescr: *mut cuda_types::cusparse::cusparseDnMatDescr_t,
        rows: i64,
        cols: i64,
        ld: i64,
        values: *mut ::core::ffi::c_void,
        valueType: cuda_types::cusparse::cudaDataType,
        order: cuda_types::cusparse::cusparseOrder_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseCreateConstDnMat(
        dnMatDescr: *mut cuda_types::cusparse::cusparseConstDnMatDescr_t,
        rows: i64,
        cols: i64,
        ld: i64,
        values: *const ::core::ffi::c_void,
        valueType: cuda_types::cusparse::cudaDataType,
        order: cuda_types::cusparse::cusparseOrder_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDestroyDnMat(
        dnMatDescr: cuda_types::cusparse::cusparseConstDnMatDescr_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDnMatGet(
        dnMatDescr: cuda_types::cusparse::cusparseDnMatDescr_t,
        rows: *mut i64,
        cols: *mut i64,
        ld: *mut i64,
        values: *mut *mut ::core::ffi::c_void,
        type_: *mut cuda_types::cusparse::cudaDataType,
        order: *mut cuda_types::cusparse::cusparseOrder_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseConstDnMatGet(
        dnMatDescr: cuda_types::cusparse::cusparseConstDnMatDescr_t,
        rows: *mut i64,
        cols: *mut i64,
        ld: *mut i64,
        values: *mut *const ::core::ffi::c_void,
        type_: *mut cuda_types::cusparse::cudaDataType,
        order: *mut cuda_types::cusparse::cusparseOrder_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDnMatGetValues(
        dnMatDescr: cuda_types::cusparse::cusparseDnMatDescr_t,
        values: *mut *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseConstDnMatGetValues(
        dnMatDescr: cuda_types::cusparse::cusparseConstDnMatDescr_t,
        values: *mut *const ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDnMatSetValues(
        dnMatDescr: cuda_types::cusparse::cusparseDnMatDescr_t,
        values: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDnMatSetStridedBatch(
        dnMatDescr: cuda_types::cusparse::cusparseDnMatDescr_t,
        batchCount: ::core::ffi::c_int,
        batchStride: i64,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDnMatGetStridedBatch(
        dnMatDescr: cuda_types::cusparse::cusparseConstDnMatDescr_t,
        batchCount: *mut ::core::ffi::c_int,
        batchStride: *mut i64,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseAxpby(
        handle: cuda_types::cusparse::cusparseHandle_t,
        alpha: *const ::core::ffi::c_void,
        vecX: cuda_types::cusparse::cusparseConstSpVecDescr_t,
        beta: *const ::core::ffi::c_void,
        vecY: cuda_types::cusparse::cusparseDnVecDescr_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseGather(
        handle: cuda_types::cusparse::cusparseHandle_t,
        vecY: cuda_types::cusparse::cusparseConstDnVecDescr_t,
        vecX: cuda_types::cusparse::cusparseSpVecDescr_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseScatter(
        handle: cuda_types::cusparse::cusparseHandle_t,
        vecX: cuda_types::cusparse::cusparseConstSpVecDescr_t,
        vecY: cuda_types::cusparse::cusparseDnVecDescr_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseRot(
        handle: cuda_types::cusparse::cusparseHandle_t,
        c_coeff: *const ::core::ffi::c_void,
        s_coeff: *const ::core::ffi::c_void,
        vecX: cuda_types::cusparse::cusparseSpVecDescr_t,
        vecY: cuda_types::cusparse::cusparseDnVecDescr_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpVV_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        opX: cuda_types::cusparse::cusparseOperation_t,
        vecX: cuda_types::cusparse::cusparseConstSpVecDescr_t,
        vecY: cuda_types::cusparse::cusparseConstDnVecDescr_t,
        result: *const ::core::ffi::c_void,
        computeType: cuda_types::cusparse::cudaDataType,
        bufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpVV(
        handle: cuda_types::cusparse::cusparseHandle_t,
        opX: cuda_types::cusparse::cusparseOperation_t,
        vecX: cuda_types::cusparse::cusparseConstSpVecDescr_t,
        vecY: cuda_types::cusparse::cusparseConstDnVecDescr_t,
        result: *mut ::core::ffi::c_void,
        computeType: cuda_types::cusparse::cudaDataType,
        externalBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSparseToDense_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        matA: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        matB: cuda_types::cusparse::cusparseDnMatDescr_t,
        alg: cuda_types::cusparse::cusparseSparseToDenseAlg_t,
        bufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSparseToDense(
        handle: cuda_types::cusparse::cusparseHandle_t,
        matA: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        matB: cuda_types::cusparse::cusparseDnMatDescr_t,
        alg: cuda_types::cusparse::cusparseSparseToDenseAlg_t,
        externalBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDenseToSparse_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        matA: cuda_types::cusparse::cusparseConstDnMatDescr_t,
        matB: cuda_types::cusparse::cusparseSpMatDescr_t,
        alg: cuda_types::cusparse::cusparseDenseToSparseAlg_t,
        bufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDenseToSparse_analysis(
        handle: cuda_types::cusparse::cusparseHandle_t,
        matA: cuda_types::cusparse::cusparseConstDnMatDescr_t,
        matB: cuda_types::cusparse::cusparseSpMatDescr_t,
        alg: cuda_types::cusparse::cusparseDenseToSparseAlg_t,
        externalBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseDenseToSparse_convert(
        handle: cuda_types::cusparse::cusparseHandle_t,
        matA: cuda_types::cusparse::cusparseConstDnMatDescr_t,
        matB: cuda_types::cusparse::cusparseSpMatDescr_t,
        alg: cuda_types::cusparse::cusparseDenseToSparseAlg_t,
        externalBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpMV(
        handle: cuda_types::cusparse::cusparseHandle_t,
        opA: cuda_types::cusparse::cusparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        vecX: cuda_types::cusparse::cusparseConstDnVecDescr_t,
        beta: *const ::core::ffi::c_void,
        vecY: cuda_types::cusparse::cusparseDnVecDescr_t,
        computeType: cuda_types::cusparse::cudaDataType,
        alg: cuda_types::cusparse::cusparseSpMVAlg_t,
        externalBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpMV_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        opA: cuda_types::cusparse::cusparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        vecX: cuda_types::cusparse::cusparseConstDnVecDescr_t,
        beta: *const ::core::ffi::c_void,
        vecY: cuda_types::cusparse::cusparseDnVecDescr_t,
        computeType: cuda_types::cusparse::cudaDataType,
        alg: cuda_types::cusparse::cusparseSpMVAlg_t,
        bufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpMV_preprocess(
        handle: cuda_types::cusparse::cusparseHandle_t,
        opA: cuda_types::cusparse::cusparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        vecX: cuda_types::cusparse::cusparseConstDnVecDescr_t,
        beta: *const ::core::ffi::c_void,
        vecY: cuda_types::cusparse::cusparseDnVecDescr_t,
        computeType: cuda_types::cusparse::cudaDataType,
        alg: cuda_types::cusparse::cusparseSpMVAlg_t,
        externalBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpSV_createDescr(
        descr: *mut cuda_types::cusparse::cusparseSpSVDescr_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpSV_destroyDescr(
        descr: cuda_types::cusparse::cusparseSpSVDescr_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpSV_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        opA: cuda_types::cusparse::cusparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        vecX: cuda_types::cusparse::cusparseConstDnVecDescr_t,
        vecY: cuda_types::cusparse::cusparseDnVecDescr_t,
        computeType: cuda_types::cusparse::cudaDataType,
        alg: cuda_types::cusparse::cusparseSpSVAlg_t,
        spsvDescr: cuda_types::cusparse::cusparseSpSVDescr_t,
        bufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpSV_analysis(
        handle: cuda_types::cusparse::cusparseHandle_t,
        opA: cuda_types::cusparse::cusparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        vecX: cuda_types::cusparse::cusparseConstDnVecDescr_t,
        vecY: cuda_types::cusparse::cusparseDnVecDescr_t,
        computeType: cuda_types::cusparse::cudaDataType,
        alg: cuda_types::cusparse::cusparseSpSVAlg_t,
        spsvDescr: cuda_types::cusparse::cusparseSpSVDescr_t,
        externalBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpSV_solve(
        handle: cuda_types::cusparse::cusparseHandle_t,
        opA: cuda_types::cusparse::cusparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        vecX: cuda_types::cusparse::cusparseConstDnVecDescr_t,
        vecY: cuda_types::cusparse::cusparseDnVecDescr_t,
        computeType: cuda_types::cusparse::cudaDataType,
        alg: cuda_types::cusparse::cusparseSpSVAlg_t,
        spsvDescr: cuda_types::cusparse::cusparseSpSVDescr_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpSV_updateMatrix(
        handle: cuda_types::cusparse::cusparseHandle_t,
        spsvDescr: cuda_types::cusparse::cusparseSpSVDescr_t,
        newValues: *mut ::core::ffi::c_void,
        updatePart: cuda_types::cusparse::cusparseSpSVUpdate_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpSM_createDescr(
        descr: *mut cuda_types::cusparse::cusparseSpSMDescr_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpSM_destroyDescr(
        descr: cuda_types::cusparse::cusparseSpSMDescr_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpSM_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        opA: cuda_types::cusparse::cusparseOperation_t,
        opB: cuda_types::cusparse::cusparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        matB: cuda_types::cusparse::cusparseConstDnMatDescr_t,
        matC: cuda_types::cusparse::cusparseDnMatDescr_t,
        computeType: cuda_types::cusparse::cudaDataType,
        alg: cuda_types::cusparse::cusparseSpSMAlg_t,
        spsmDescr: cuda_types::cusparse::cusparseSpSMDescr_t,
        bufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpSM_analysis(
        handle: cuda_types::cusparse::cusparseHandle_t,
        opA: cuda_types::cusparse::cusparseOperation_t,
        opB: cuda_types::cusparse::cusparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        matB: cuda_types::cusparse::cusparseConstDnMatDescr_t,
        matC: cuda_types::cusparse::cusparseDnMatDescr_t,
        computeType: cuda_types::cusparse::cudaDataType,
        alg: cuda_types::cusparse::cusparseSpSMAlg_t,
        spsmDescr: cuda_types::cusparse::cusparseSpSMDescr_t,
        externalBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpSM_solve(
        handle: cuda_types::cusparse::cusparseHandle_t,
        opA: cuda_types::cusparse::cusparseOperation_t,
        opB: cuda_types::cusparse::cusparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        matB: cuda_types::cusparse::cusparseConstDnMatDescr_t,
        matC: cuda_types::cusparse::cusparseDnMatDescr_t,
        computeType: cuda_types::cusparse::cudaDataType,
        alg: cuda_types::cusparse::cusparseSpSMAlg_t,
        spsmDescr: cuda_types::cusparse::cusparseSpSMDescr_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpSM_updateMatrix(
        handle: cuda_types::cusparse::cusparseHandle_t,
        spsmDescr: cuda_types::cusparse::cusparseSpSMDescr_t,
        newValues: *mut ::core::ffi::c_void,
        updatePart: cuda_types::cusparse::cusparseSpSMUpdate_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpMM_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        opA: cuda_types::cusparse::cusparseOperation_t,
        opB: cuda_types::cusparse::cusparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        matB: cuda_types::cusparse::cusparseConstDnMatDescr_t,
        beta: *const ::core::ffi::c_void,
        matC: cuda_types::cusparse::cusparseDnMatDescr_t,
        computeType: cuda_types::cusparse::cudaDataType,
        alg: cuda_types::cusparse::cusparseSpMMAlg_t,
        bufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpMM_preprocess(
        handle: cuda_types::cusparse::cusparseHandle_t,
        opA: cuda_types::cusparse::cusparseOperation_t,
        opB: cuda_types::cusparse::cusparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        matB: cuda_types::cusparse::cusparseConstDnMatDescr_t,
        beta: *const ::core::ffi::c_void,
        matC: cuda_types::cusparse::cusparseDnMatDescr_t,
        computeType: cuda_types::cusparse::cudaDataType,
        alg: cuda_types::cusparse::cusparseSpMMAlg_t,
        externalBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpMM(
        handle: cuda_types::cusparse::cusparseHandle_t,
        opA: cuda_types::cusparse::cusparseOperation_t,
        opB: cuda_types::cusparse::cusparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        matB: cuda_types::cusparse::cusparseConstDnMatDescr_t,
        beta: *const ::core::ffi::c_void,
        matC: cuda_types::cusparse::cusparseDnMatDescr_t,
        computeType: cuda_types::cusparse::cudaDataType,
        alg: cuda_types::cusparse::cusparseSpMMAlg_t,
        externalBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpGEMM_createDescr(
        descr: *mut cuda_types::cusparse::cusparseSpGEMMDescr_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpGEMM_destroyDescr(
        descr: cuda_types::cusparse::cusparseSpGEMMDescr_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpGEMM_workEstimation(
        handle: cuda_types::cusparse::cusparseHandle_t,
        opA: cuda_types::cusparse::cusparseOperation_t,
        opB: cuda_types::cusparse::cusparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        matB: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        beta: *const ::core::ffi::c_void,
        matC: cuda_types::cusparse::cusparseSpMatDescr_t,
        computeType: cuda_types::cusparse::cudaDataType,
        alg: cuda_types::cusparse::cusparseSpGEMMAlg_t,
        spgemmDescr: cuda_types::cusparse::cusparseSpGEMMDescr_t,
        bufferSize1: *mut usize,
        externalBuffer1: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpGEMM_getNumProducts(
        spgemmDescr: cuda_types::cusparse::cusparseSpGEMMDescr_t,
        num_prods: *mut i64,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpGEMM_estimateMemory(
        handle: cuda_types::cusparse::cusparseHandle_t,
        opA: cuda_types::cusparse::cusparseOperation_t,
        opB: cuda_types::cusparse::cusparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        matB: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        beta: *const ::core::ffi::c_void,
        matC: cuda_types::cusparse::cusparseSpMatDescr_t,
        computeType: cuda_types::cusparse::cudaDataType,
        alg: cuda_types::cusparse::cusparseSpGEMMAlg_t,
        spgemmDescr: cuda_types::cusparse::cusparseSpGEMMDescr_t,
        chunk_fraction: f32,
        bufferSize3: *mut usize,
        externalBuffer3: *mut ::core::ffi::c_void,
        bufferSize2: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpGEMM_compute(
        handle: cuda_types::cusparse::cusparseHandle_t,
        opA: cuda_types::cusparse::cusparseOperation_t,
        opB: cuda_types::cusparse::cusparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        matB: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        beta: *const ::core::ffi::c_void,
        matC: cuda_types::cusparse::cusparseSpMatDescr_t,
        computeType: cuda_types::cusparse::cudaDataType,
        alg: cuda_types::cusparse::cusparseSpGEMMAlg_t,
        spgemmDescr: cuda_types::cusparse::cusparseSpGEMMDescr_t,
        bufferSize2: *mut usize,
        externalBuffer2: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpGEMM_copy(
        handle: cuda_types::cusparse::cusparseHandle_t,
        opA: cuda_types::cusparse::cusparseOperation_t,
        opB: cuda_types::cusparse::cusparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        matB: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        beta: *const ::core::ffi::c_void,
        matC: cuda_types::cusparse::cusparseSpMatDescr_t,
        computeType: cuda_types::cusparse::cudaDataType,
        alg: cuda_types::cusparse::cusparseSpGEMMAlg_t,
        spgemmDescr: cuda_types::cusparse::cusparseSpGEMMDescr_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpGEMMreuse_workEstimation(
        handle: cuda_types::cusparse::cusparseHandle_t,
        opA: cuda_types::cusparse::cusparseOperation_t,
        opB: cuda_types::cusparse::cusparseOperation_t,
        matA: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        matB: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        matC: cuda_types::cusparse::cusparseSpMatDescr_t,
        alg: cuda_types::cusparse::cusparseSpGEMMAlg_t,
        spgemmDescr: cuda_types::cusparse::cusparseSpGEMMDescr_t,
        bufferSize1: *mut usize,
        externalBuffer1: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpGEMMreuse_nnz(
        handle: cuda_types::cusparse::cusparseHandle_t,
        opA: cuda_types::cusparse::cusparseOperation_t,
        opB: cuda_types::cusparse::cusparseOperation_t,
        matA: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        matB: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        matC: cuda_types::cusparse::cusparseSpMatDescr_t,
        alg: cuda_types::cusparse::cusparseSpGEMMAlg_t,
        spgemmDescr: cuda_types::cusparse::cusparseSpGEMMDescr_t,
        bufferSize2: *mut usize,
        externalBuffer2: *mut ::core::ffi::c_void,
        bufferSize3: *mut usize,
        externalBuffer3: *mut ::core::ffi::c_void,
        bufferSize4: *mut usize,
        externalBuffer4: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpGEMMreuse_copy(
        handle: cuda_types::cusparse::cusparseHandle_t,
        opA: cuda_types::cusparse::cusparseOperation_t,
        opB: cuda_types::cusparse::cusparseOperation_t,
        matA: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        matB: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        matC: cuda_types::cusparse::cusparseSpMatDescr_t,
        alg: cuda_types::cusparse::cusparseSpGEMMAlg_t,
        spgemmDescr: cuda_types::cusparse::cusparseSpGEMMDescr_t,
        bufferSize5: *mut usize,
        externalBuffer5: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpGEMMreuse_compute(
        handle: cuda_types::cusparse::cusparseHandle_t,
        opA: cuda_types::cusparse::cusparseOperation_t,
        opB: cuda_types::cusparse::cusparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        matB: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        beta: *const ::core::ffi::c_void,
        matC: cuda_types::cusparse::cusparseSpMatDescr_t,
        computeType: cuda_types::cusparse::cudaDataType,
        alg: cuda_types::cusparse::cusparseSpGEMMAlg_t,
        spgemmDescr: cuda_types::cusparse::cusparseSpGEMMDescr_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSDDMM_bufferSize(
        handle: cuda_types::cusparse::cusparseHandle_t,
        opA: cuda_types::cusparse::cusparseOperation_t,
        opB: cuda_types::cusparse::cusparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: cuda_types::cusparse::cusparseConstDnMatDescr_t,
        matB: cuda_types::cusparse::cusparseConstDnMatDescr_t,
        beta: *const ::core::ffi::c_void,
        matC: cuda_types::cusparse::cusparseSpMatDescr_t,
        computeType: cuda_types::cusparse::cudaDataType,
        alg: cuda_types::cusparse::cusparseSDDMMAlg_t,
        bufferSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSDDMM_preprocess(
        handle: cuda_types::cusparse::cusparseHandle_t,
        opA: cuda_types::cusparse::cusparseOperation_t,
        opB: cuda_types::cusparse::cusparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: cuda_types::cusparse::cusparseConstDnMatDescr_t,
        matB: cuda_types::cusparse::cusparseConstDnMatDescr_t,
        beta: *const ::core::ffi::c_void,
        matC: cuda_types::cusparse::cusparseSpMatDescr_t,
        computeType: cuda_types::cusparse::cudaDataType,
        alg: cuda_types::cusparse::cusparseSDDMMAlg_t,
        externalBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSDDMM(
        handle: cuda_types::cusparse::cusparseHandle_t,
        opA: cuda_types::cusparse::cusparseOperation_t,
        opB: cuda_types::cusparse::cusparseOperation_t,
        alpha: *const ::core::ffi::c_void,
        matA: cuda_types::cusparse::cusparseConstDnMatDescr_t,
        matB: cuda_types::cusparse::cusparseConstDnMatDescr_t,
        beta: *const ::core::ffi::c_void,
        matC: cuda_types::cusparse::cusparseSpMatDescr_t,
        computeType: cuda_types::cusparse::cudaDataType,
        alg: cuda_types::cusparse::cusparseSDDMMAlg_t,
        externalBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpMMOp_createPlan(
        handle: cuda_types::cusparse::cusparseHandle_t,
        plan: *mut cuda_types::cusparse::cusparseSpMMOpPlan_t,
        opA: cuda_types::cusparse::cusparseOperation_t,
        opB: cuda_types::cusparse::cusparseOperation_t,
        matA: cuda_types::cusparse::cusparseConstSpMatDescr_t,
        matB: cuda_types::cusparse::cusparseConstDnMatDescr_t,
        matC: cuda_types::cusparse::cusparseDnMatDescr_t,
        computeType: cuda_types::cusparse::cudaDataType,
        alg: cuda_types::cusparse::cusparseSpMMOpAlg_t,
        addOperationNvvmBuffer: *const ::core::ffi::c_void,
        addOperationBufferSize: usize,
        mulOperationNvvmBuffer: *const ::core::ffi::c_void,
        mulOperationBufferSize: usize,
        epilogueNvvmBuffer: *const ::core::ffi::c_void,
        epilogueBufferSize: usize,
        SpMMWorkspaceSize: *mut usize,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpMMOp(
        plan: cuda_types::cusparse::cusparseSpMMOpPlan_t,
        externalBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cusparse::cusparseStatus_t;
    #[must_use]
    fn cusparseSpMMOp_destroyPlan(
        plan: cuda_types::cusparse::cusparseSpMMOpPlan_t,
    ) -> cuda_types::cusparse::cusparseStatus_t;
}
