// Generated automatically by zluda_bindgen
// DO NOT EDIT MANUALLY
#![allow(warnings)]
extern "system" {
    #[must_use]
    fn cublasCreate_v2(
        handle: *mut cuda_types::cublas::cublasHandle_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDestroy_v2(
        handle: cuda_types::cublas::cublasHandle_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasGetVersion_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        version: *mut ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasGetProperty(
        type_: cuda_types::cublas::libraryPropertyType,
        value: *mut ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    fn cublasGetCudartVersion() -> usize;
    #[must_use]
    fn cublasSetWorkspace_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        workspace: *mut ::core::ffi::c_void,
        workspaceSizeInBytes: usize,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSetStream_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        streamId: cuda_types::cublas::cudaStream_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasGetStream_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        streamId: *mut cuda_types::cublas::cudaStream_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasGetPointerMode_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        mode: *mut cuda_types::cublas::cublasPointerMode_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSetPointerMode_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        mode: cuda_types::cublas::cublasPointerMode_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasGetAtomicsMode(
        handle: cuda_types::cublas::cublasHandle_t,
        mode: *mut cuda_types::cublas::cublasAtomicsMode_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSetAtomicsMode(
        handle: cuda_types::cublas::cublasHandle_t,
        mode: cuda_types::cublas::cublasAtomicsMode_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasGetMathMode(
        handle: cuda_types::cublas::cublasHandle_t,
        mode: *mut cuda_types::cublas::cublasMath_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSetMathMode(
        handle: cuda_types::cublas::cublasHandle_t,
        mode: cuda_types::cublas::cublasMath_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasGetSmCountTarget(
        handle: cuda_types::cublas::cublasHandle_t,
        smCountTarget: *mut ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSetSmCountTarget(
        handle: cuda_types::cublas::cublasHandle_t,
        smCountTarget: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    fn cublasGetStatusName(
        status: cuda_types::cublas::cublasStatus_t,
    ) -> *const ::core::ffi::c_char;
    fn cublasGetStatusString(
        status: cuda_types::cublas::cublasStatus_t,
    ) -> *const ::core::ffi::c_char;
    #[must_use]
    fn cublasLoggerConfigure(
        logIsOn: ::core::ffi::c_int,
        logToStdOut: ::core::ffi::c_int,
        logToStdErr: ::core::ffi::c_int,
        logFileName: *const ::core::ffi::c_char,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSetLoggerCallback(
        userCallback: cuda_types::cublas::cublasLogCallback,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasGetLoggerCallback(
        userCallback: *mut cuda_types::cublas::cublasLogCallback,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSetVector(
        n: ::core::ffi::c_int,
        elemSize: ::core::ffi::c_int,
        x: *const ::core::ffi::c_void,
        incx: ::core::ffi::c_int,
        devicePtr: *mut ::core::ffi::c_void,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSetVector_64(
        n: cuda_types::cublas::i64,
        elemSize: cuda_types::cublas::i64,
        x: *const ::core::ffi::c_void,
        incx: cuda_types::cublas::i64,
        devicePtr: *mut ::core::ffi::c_void,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasGetVector(
        n: ::core::ffi::c_int,
        elemSize: ::core::ffi::c_int,
        x: *const ::core::ffi::c_void,
        incx: ::core::ffi::c_int,
        y: *mut ::core::ffi::c_void,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasGetVector_64(
        n: cuda_types::cublas::i64,
        elemSize: cuda_types::cublas::i64,
        x: *const ::core::ffi::c_void,
        incx: cuda_types::cublas::i64,
        y: *mut ::core::ffi::c_void,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSetMatrix(
        rows: ::core::ffi::c_int,
        cols: ::core::ffi::c_int,
        elemSize: ::core::ffi::c_int,
        A: *const ::core::ffi::c_void,
        lda: ::core::ffi::c_int,
        B: *mut ::core::ffi::c_void,
        ldb: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSetMatrix_64(
        rows: cuda_types::cublas::i64,
        cols: cuda_types::cublas::i64,
        elemSize: cuda_types::cublas::i64,
        A: *const ::core::ffi::c_void,
        lda: cuda_types::cublas::i64,
        B: *mut ::core::ffi::c_void,
        ldb: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasGetMatrix(
        rows: ::core::ffi::c_int,
        cols: ::core::ffi::c_int,
        elemSize: ::core::ffi::c_int,
        A: *const ::core::ffi::c_void,
        lda: ::core::ffi::c_int,
        B: *mut ::core::ffi::c_void,
        ldb: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasGetMatrix_64(
        rows: cuda_types::cublas::i64,
        cols: cuda_types::cublas::i64,
        elemSize: cuda_types::cublas::i64,
        A: *const ::core::ffi::c_void,
        lda: cuda_types::cublas::i64,
        B: *mut ::core::ffi::c_void,
        ldb: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSetVectorAsync(
        n: ::core::ffi::c_int,
        elemSize: ::core::ffi::c_int,
        hostPtr: *const ::core::ffi::c_void,
        incx: ::core::ffi::c_int,
        devicePtr: *mut ::core::ffi::c_void,
        incy: ::core::ffi::c_int,
        stream: cuda_types::cublas::cudaStream_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSetVectorAsync_64(
        n: cuda_types::cublas::i64,
        elemSize: cuda_types::cublas::i64,
        hostPtr: *const ::core::ffi::c_void,
        incx: cuda_types::cublas::i64,
        devicePtr: *mut ::core::ffi::c_void,
        incy: cuda_types::cublas::i64,
        stream: cuda_types::cublas::cudaStream_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasGetVectorAsync(
        n: ::core::ffi::c_int,
        elemSize: ::core::ffi::c_int,
        devicePtr: *const ::core::ffi::c_void,
        incx: ::core::ffi::c_int,
        hostPtr: *mut ::core::ffi::c_void,
        incy: ::core::ffi::c_int,
        stream: cuda_types::cublas::cudaStream_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasGetVectorAsync_64(
        n: cuda_types::cublas::i64,
        elemSize: cuda_types::cublas::i64,
        devicePtr: *const ::core::ffi::c_void,
        incx: cuda_types::cublas::i64,
        hostPtr: *mut ::core::ffi::c_void,
        incy: cuda_types::cublas::i64,
        stream: cuda_types::cublas::cudaStream_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSetMatrixAsync(
        rows: ::core::ffi::c_int,
        cols: ::core::ffi::c_int,
        elemSize: ::core::ffi::c_int,
        A: *const ::core::ffi::c_void,
        lda: ::core::ffi::c_int,
        B: *mut ::core::ffi::c_void,
        ldb: ::core::ffi::c_int,
        stream: cuda_types::cublas::cudaStream_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSetMatrixAsync_64(
        rows: cuda_types::cublas::i64,
        cols: cuda_types::cublas::i64,
        elemSize: cuda_types::cublas::i64,
        A: *const ::core::ffi::c_void,
        lda: cuda_types::cublas::i64,
        B: *mut ::core::ffi::c_void,
        ldb: cuda_types::cublas::i64,
        stream: cuda_types::cublas::cudaStream_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasGetMatrixAsync(
        rows: ::core::ffi::c_int,
        cols: ::core::ffi::c_int,
        elemSize: ::core::ffi::c_int,
        A: *const ::core::ffi::c_void,
        lda: ::core::ffi::c_int,
        B: *mut ::core::ffi::c_void,
        ldb: ::core::ffi::c_int,
        stream: cuda_types::cublas::cudaStream_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasGetMatrixAsync_64(
        rows: cuda_types::cublas::i64,
        cols: cuda_types::cublas::i64,
        elemSize: cuda_types::cublas::i64,
        A: *const ::core::ffi::c_void,
        lda: cuda_types::cublas::i64,
        B: *mut ::core::ffi::c_void,
        ldb: cuda_types::cublas::i64,
        stream: cuda_types::cublas::cudaStream_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    fn cublasXerbla(srName: *const ::core::ffi::c_char, info: ::core::ffi::c_int) -> ();
    #[must_use]
    fn cublasNrm2Ex(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const ::core::ffi::c_void,
        xType: cuda_types::cublas::cudaDataType,
        incx: ::core::ffi::c_int,
        result: *mut ::core::ffi::c_void,
        resultType: cuda_types::cublas::cudaDataType,
        executionType: cuda_types::cublas::cudaDataType,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasNrm2Ex_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const ::core::ffi::c_void,
        xType: cuda_types::cublas::cudaDataType,
        incx: cuda_types::cublas::i64,
        result: *mut ::core::ffi::c_void,
        resultType: cuda_types::cublas::cudaDataType,
        executionType: cuda_types::cublas::cudaDataType,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSnrm2_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const f32,
        incx: ::core::ffi::c_int,
        result: *mut f32,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSnrm2_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const f32,
        incx: cuda_types::cublas::i64,
        result: *mut f32,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDnrm2_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const f64,
        incx: ::core::ffi::c_int,
        result: *mut f64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDnrm2_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const f64,
        incx: cuda_types::cublas::i64,
        result: *mut f64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasScnrm2_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
        result: *mut f32,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasScnrm2_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
        result: *mut f32,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDznrm2_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
        result: *mut f64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDznrm2_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
        result: *mut f64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDotEx(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const ::core::ffi::c_void,
        xType: cuda_types::cublas::cudaDataType,
        incx: ::core::ffi::c_int,
        y: *const ::core::ffi::c_void,
        yType: cuda_types::cublas::cudaDataType,
        incy: ::core::ffi::c_int,
        result: *mut ::core::ffi::c_void,
        resultType: cuda_types::cublas::cudaDataType,
        executionType: cuda_types::cublas::cudaDataType,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDotEx_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const ::core::ffi::c_void,
        xType: cuda_types::cublas::cudaDataType,
        incx: cuda_types::cublas::i64,
        y: *const ::core::ffi::c_void,
        yType: cuda_types::cublas::cudaDataType,
        incy: cuda_types::cublas::i64,
        result: *mut ::core::ffi::c_void,
        resultType: cuda_types::cublas::cudaDataType,
        executionType: cuda_types::cublas::cudaDataType,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDotcEx(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const ::core::ffi::c_void,
        xType: cuda_types::cublas::cudaDataType,
        incx: ::core::ffi::c_int,
        y: *const ::core::ffi::c_void,
        yType: cuda_types::cublas::cudaDataType,
        incy: ::core::ffi::c_int,
        result: *mut ::core::ffi::c_void,
        resultType: cuda_types::cublas::cudaDataType,
        executionType: cuda_types::cublas::cudaDataType,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDotcEx_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const ::core::ffi::c_void,
        xType: cuda_types::cublas::cudaDataType,
        incx: cuda_types::cublas::i64,
        y: *const ::core::ffi::c_void,
        yType: cuda_types::cublas::cudaDataType,
        incy: cuda_types::cublas::i64,
        result: *mut ::core::ffi::c_void,
        resultType: cuda_types::cublas::cudaDataType,
        executionType: cuda_types::cublas::cudaDataType,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSdot_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const f32,
        incx: ::core::ffi::c_int,
        y: *const f32,
        incy: ::core::ffi::c_int,
        result: *mut f32,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSdot_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const f32,
        incx: cuda_types::cublas::i64,
        y: *const f32,
        incy: cuda_types::cublas::i64,
        result: *mut f32,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDdot_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const f64,
        incx: ::core::ffi::c_int,
        y: *const f64,
        incy: ::core::ffi::c_int,
        result: *mut f64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDdot_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const f64,
        incx: cuda_types::cublas::i64,
        y: *const f64,
        incy: cuda_types::cublas::i64,
        result: *mut f64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCdotu_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
        y: *const cuda_types::cublas::cuComplex,
        incy: ::core::ffi::c_int,
        result: *mut cuda_types::cublas::cuComplex,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCdotu_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
        y: *const cuda_types::cublas::cuComplex,
        incy: cuda_types::cublas::i64,
        result: *mut cuda_types::cublas::cuComplex,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCdotc_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
        y: *const cuda_types::cublas::cuComplex,
        incy: ::core::ffi::c_int,
        result: *mut cuda_types::cublas::cuComplex,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCdotc_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
        y: *const cuda_types::cublas::cuComplex,
        incy: cuda_types::cublas::i64,
        result: *mut cuda_types::cublas::cuComplex,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZdotu_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
        y: *const cuda_types::cublas::cuDoubleComplex,
        incy: ::core::ffi::c_int,
        result: *mut cuda_types::cublas::cuDoubleComplex,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZdotu_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
        y: *const cuda_types::cublas::cuDoubleComplex,
        incy: cuda_types::cublas::i64,
        result: *mut cuda_types::cublas::cuDoubleComplex,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZdotc_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
        y: *const cuda_types::cublas::cuDoubleComplex,
        incy: ::core::ffi::c_int,
        result: *mut cuda_types::cublas::cuDoubleComplex,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZdotc_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
        y: *const cuda_types::cublas::cuDoubleComplex,
        incy: cuda_types::cublas::i64,
        result: *mut cuda_types::cublas::cuDoubleComplex,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasScalEx(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        alpha: *const ::core::ffi::c_void,
        alphaType: cuda_types::cublas::cudaDataType,
        x: *mut ::core::ffi::c_void,
        xType: cuda_types::cublas::cudaDataType,
        incx: ::core::ffi::c_int,
        executionType: cuda_types::cublas::cudaDataType,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasScalEx_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        alpha: *const ::core::ffi::c_void,
        alphaType: cuda_types::cublas::cudaDataType,
        x: *mut ::core::ffi::c_void,
        xType: cuda_types::cublas::cudaDataType,
        incx: cuda_types::cublas::i64,
        executionType: cuda_types::cublas::cudaDataType,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSscal_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        x: *mut f32,
        incx: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSscal_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        alpha: *const f32,
        x: *mut f32,
        incx: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDscal_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        alpha: *const f64,
        x: *mut f64,
        incx: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDscal_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        alpha: *const f64,
        x: *mut f64,
        incx: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCscal_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        x: *mut cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCscal_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        x: *mut cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCsscal_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        x: *mut cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCsscal_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        alpha: *const f32,
        x: *mut cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZscal_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        x: *mut cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZscal_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        x: *mut cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZdscal_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        alpha: *const f64,
        x: *mut cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZdscal_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        alpha: *const f64,
        x: *mut cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasAxpyEx(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        alpha: *const ::core::ffi::c_void,
        alphaType: cuda_types::cublas::cudaDataType,
        x: *const ::core::ffi::c_void,
        xType: cuda_types::cublas::cudaDataType,
        incx: ::core::ffi::c_int,
        y: *mut ::core::ffi::c_void,
        yType: cuda_types::cublas::cudaDataType,
        incy: ::core::ffi::c_int,
        executiontype: cuda_types::cublas::cudaDataType,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasAxpyEx_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        alpha: *const ::core::ffi::c_void,
        alphaType: cuda_types::cublas::cudaDataType,
        x: *const ::core::ffi::c_void,
        xType: cuda_types::cublas::cudaDataType,
        incx: cuda_types::cublas::i64,
        y: *mut ::core::ffi::c_void,
        yType: cuda_types::cublas::cudaDataType,
        incy: cuda_types::cublas::i64,
        executiontype: cuda_types::cublas::cudaDataType,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSaxpy_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        x: *const f32,
        incx: ::core::ffi::c_int,
        y: *mut f32,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSaxpy_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        alpha: *const f32,
        x: *const f32,
        incx: cuda_types::cublas::i64,
        y: *mut f32,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDaxpy_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        alpha: *const f64,
        x: *const f64,
        incx: ::core::ffi::c_int,
        y: *mut f64,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDaxpy_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        alpha: *const f64,
        x: *const f64,
        incx: cuda_types::cublas::i64,
        y: *mut f64,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCaxpy_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        x: *const cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
        y: *mut cuda_types::cublas::cuComplex,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCaxpy_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        x: *const cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
        y: *mut cuda_types::cublas::cuComplex,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZaxpy_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
        y: *mut cuda_types::cublas::cuDoubleComplex,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZaxpy_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
        y: *mut cuda_types::cublas::cuDoubleComplex,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCopyEx(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const ::core::ffi::c_void,
        xType: cuda_types::cublas::cudaDataType,
        incx: ::core::ffi::c_int,
        y: *mut ::core::ffi::c_void,
        yType: cuda_types::cublas::cudaDataType,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCopyEx_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const ::core::ffi::c_void,
        xType: cuda_types::cublas::cudaDataType,
        incx: cuda_types::cublas::i64,
        y: *mut ::core::ffi::c_void,
        yType: cuda_types::cublas::cudaDataType,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasScopy_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const f32,
        incx: ::core::ffi::c_int,
        y: *mut f32,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasScopy_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const f32,
        incx: cuda_types::cublas::i64,
        y: *mut f32,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDcopy_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const f64,
        incx: ::core::ffi::c_int,
        y: *mut f64,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDcopy_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const f64,
        incx: cuda_types::cublas::i64,
        y: *mut f64,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCcopy_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
        y: *mut cuda_types::cublas::cuComplex,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCcopy_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
        y: *mut cuda_types::cublas::cuComplex,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZcopy_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
        y: *mut cuda_types::cublas::cuDoubleComplex,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZcopy_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
        y: *mut cuda_types::cublas::cuDoubleComplex,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSswap_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *mut f32,
        incx: ::core::ffi::c_int,
        y: *mut f32,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSswap_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *mut f32,
        incx: cuda_types::cublas::i64,
        y: *mut f32,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDswap_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *mut f64,
        incx: ::core::ffi::c_int,
        y: *mut f64,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDswap_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *mut f64,
        incx: cuda_types::cublas::i64,
        y: *mut f64,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCswap_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *mut cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
        y: *mut cuda_types::cublas::cuComplex,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCswap_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *mut cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
        y: *mut cuda_types::cublas::cuComplex,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZswap_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *mut cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
        y: *mut cuda_types::cublas::cuDoubleComplex,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZswap_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *mut cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
        y: *mut cuda_types::cublas::cuDoubleComplex,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSwapEx(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *mut ::core::ffi::c_void,
        xType: cuda_types::cublas::cudaDataType,
        incx: ::core::ffi::c_int,
        y: *mut ::core::ffi::c_void,
        yType: cuda_types::cublas::cudaDataType,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSwapEx_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *mut ::core::ffi::c_void,
        xType: cuda_types::cublas::cudaDataType,
        incx: cuda_types::cublas::i64,
        y: *mut ::core::ffi::c_void,
        yType: cuda_types::cublas::cudaDataType,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasIsamax_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const f32,
        incx: ::core::ffi::c_int,
        result: *mut ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasIsamax_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const f32,
        incx: cuda_types::cublas::i64,
        result: *mut cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasIdamax_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const f64,
        incx: ::core::ffi::c_int,
        result: *mut ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasIdamax_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const f64,
        incx: cuda_types::cublas::i64,
        result: *mut cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasIcamax_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
        result: *mut ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasIcamax_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
        result: *mut cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasIzamax_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
        result: *mut ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasIzamax_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
        result: *mut cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasIamaxEx(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const ::core::ffi::c_void,
        xType: cuda_types::cublas::cudaDataType,
        incx: ::core::ffi::c_int,
        result: *mut ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasIamaxEx_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const ::core::ffi::c_void,
        xType: cuda_types::cublas::cudaDataType,
        incx: cuda_types::cublas::i64,
        result: *mut cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasIsamin_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const f32,
        incx: ::core::ffi::c_int,
        result: *mut ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasIsamin_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const f32,
        incx: cuda_types::cublas::i64,
        result: *mut cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasIdamin_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const f64,
        incx: ::core::ffi::c_int,
        result: *mut ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasIdamin_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const f64,
        incx: cuda_types::cublas::i64,
        result: *mut cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasIcamin_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
        result: *mut ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasIcamin_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
        result: *mut cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasIzamin_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
        result: *mut ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasIzamin_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
        result: *mut cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasIaminEx(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const ::core::ffi::c_void,
        xType: cuda_types::cublas::cudaDataType,
        incx: ::core::ffi::c_int,
        result: *mut ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasIaminEx_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const ::core::ffi::c_void,
        xType: cuda_types::cublas::cudaDataType,
        incx: cuda_types::cublas::i64,
        result: *mut cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasAsumEx(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const ::core::ffi::c_void,
        xType: cuda_types::cublas::cudaDataType,
        incx: ::core::ffi::c_int,
        result: *mut ::core::ffi::c_void,
        resultType: cuda_types::cublas::cudaDataType,
        executiontype: cuda_types::cublas::cudaDataType,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasAsumEx_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const ::core::ffi::c_void,
        xType: cuda_types::cublas::cudaDataType,
        incx: cuda_types::cublas::i64,
        result: *mut ::core::ffi::c_void,
        resultType: cuda_types::cublas::cudaDataType,
        executiontype: cuda_types::cublas::cudaDataType,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSasum_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const f32,
        incx: ::core::ffi::c_int,
        result: *mut f32,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSasum_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const f32,
        incx: cuda_types::cublas::i64,
        result: *mut f32,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDasum_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const f64,
        incx: ::core::ffi::c_int,
        result: *mut f64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDasum_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const f64,
        incx: cuda_types::cublas::i64,
        result: *mut f64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasScasum_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
        result: *mut f32,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasScasum_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
        result: *mut f32,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDzasum_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
        result: *mut f64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDzasum_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
        result: *mut f64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSrot_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *mut f32,
        incx: ::core::ffi::c_int,
        y: *mut f32,
        incy: ::core::ffi::c_int,
        c: *const f32,
        s: *const f32,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSrot_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *mut f32,
        incx: cuda_types::cublas::i64,
        y: *mut f32,
        incy: cuda_types::cublas::i64,
        c: *const f32,
        s: *const f32,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDrot_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *mut f64,
        incx: ::core::ffi::c_int,
        y: *mut f64,
        incy: ::core::ffi::c_int,
        c: *const f64,
        s: *const f64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDrot_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *mut f64,
        incx: cuda_types::cublas::i64,
        y: *mut f64,
        incy: cuda_types::cublas::i64,
        c: *const f64,
        s: *const f64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCrot_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *mut cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
        y: *mut cuda_types::cublas::cuComplex,
        incy: ::core::ffi::c_int,
        c: *const f32,
        s: *const cuda_types::cublas::cuComplex,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCrot_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *mut cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
        y: *mut cuda_types::cublas::cuComplex,
        incy: cuda_types::cublas::i64,
        c: *const f32,
        s: *const cuda_types::cublas::cuComplex,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCsrot_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *mut cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
        y: *mut cuda_types::cublas::cuComplex,
        incy: ::core::ffi::c_int,
        c: *const f32,
        s: *const f32,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCsrot_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *mut cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
        y: *mut cuda_types::cublas::cuComplex,
        incy: cuda_types::cublas::i64,
        c: *const f32,
        s: *const f32,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZrot_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *mut cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
        y: *mut cuda_types::cublas::cuDoubleComplex,
        incy: ::core::ffi::c_int,
        c: *const f64,
        s: *const cuda_types::cublas::cuDoubleComplex,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZrot_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *mut cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
        y: *mut cuda_types::cublas::cuDoubleComplex,
        incy: cuda_types::cublas::i64,
        c: *const f64,
        s: *const cuda_types::cublas::cuDoubleComplex,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZdrot_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *mut cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
        y: *mut cuda_types::cublas::cuDoubleComplex,
        incy: ::core::ffi::c_int,
        c: *const f64,
        s: *const f64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZdrot_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *mut cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
        y: *mut cuda_types::cublas::cuDoubleComplex,
        incy: cuda_types::cublas::i64,
        c: *const f64,
        s: *const f64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasRotEx(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *mut ::core::ffi::c_void,
        xType: cuda_types::cublas::cudaDataType,
        incx: ::core::ffi::c_int,
        y: *mut ::core::ffi::c_void,
        yType: cuda_types::cublas::cudaDataType,
        incy: ::core::ffi::c_int,
        c: *const ::core::ffi::c_void,
        s: *const ::core::ffi::c_void,
        csType: cuda_types::cublas::cudaDataType,
        executiontype: cuda_types::cublas::cudaDataType,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasRotEx_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *mut ::core::ffi::c_void,
        xType: cuda_types::cublas::cudaDataType,
        incx: cuda_types::cublas::i64,
        y: *mut ::core::ffi::c_void,
        yType: cuda_types::cublas::cudaDataType,
        incy: cuda_types::cublas::i64,
        c: *const ::core::ffi::c_void,
        s: *const ::core::ffi::c_void,
        csType: cuda_types::cublas::cudaDataType,
        executiontype: cuda_types::cublas::cudaDataType,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSrotg_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        a: *mut f32,
        b: *mut f32,
        c: *mut f32,
        s: *mut f32,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDrotg_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        a: *mut f64,
        b: *mut f64,
        c: *mut f64,
        s: *mut f64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCrotg_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        a: *mut cuda_types::cublas::cuComplex,
        b: *mut cuda_types::cublas::cuComplex,
        c: *mut f32,
        s: *mut cuda_types::cublas::cuComplex,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZrotg_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        a: *mut cuda_types::cublas::cuDoubleComplex,
        b: *mut cuda_types::cublas::cuDoubleComplex,
        c: *mut f64,
        s: *mut cuda_types::cublas::cuDoubleComplex,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasRotgEx(
        handle: cuda_types::cublas::cublasHandle_t,
        a: *mut ::core::ffi::c_void,
        b: *mut ::core::ffi::c_void,
        abType: cuda_types::cublas::cudaDataType,
        c: *mut ::core::ffi::c_void,
        s: *mut ::core::ffi::c_void,
        csType: cuda_types::cublas::cudaDataType,
        executiontype: cuda_types::cublas::cudaDataType,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSrotm_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *mut f32,
        incx: ::core::ffi::c_int,
        y: *mut f32,
        incy: ::core::ffi::c_int,
        param: *const f32,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSrotm_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *mut f32,
        incx: cuda_types::cublas::i64,
        y: *mut f32,
        incy: cuda_types::cublas::i64,
        param: *const f32,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDrotm_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *mut f64,
        incx: ::core::ffi::c_int,
        y: *mut f64,
        incy: ::core::ffi::c_int,
        param: *const f64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDrotm_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *mut f64,
        incx: cuda_types::cublas::i64,
        y: *mut f64,
        incy: cuda_types::cublas::i64,
        param: *const f64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasRotmEx(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        x: *mut ::core::ffi::c_void,
        xType: cuda_types::cublas::cudaDataType,
        incx: ::core::ffi::c_int,
        y: *mut ::core::ffi::c_void,
        yType: cuda_types::cublas::cudaDataType,
        incy: ::core::ffi::c_int,
        param: *const ::core::ffi::c_void,
        paramType: cuda_types::cublas::cudaDataType,
        executiontype: cuda_types::cublas::cudaDataType,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasRotmEx_64(
        handle: cuda_types::cublas::cublasHandle_t,
        n: cuda_types::cublas::i64,
        x: *mut ::core::ffi::c_void,
        xType: cuda_types::cublas::cudaDataType,
        incx: cuda_types::cublas::i64,
        y: *mut ::core::ffi::c_void,
        yType: cuda_types::cublas::cudaDataType,
        incy: cuda_types::cublas::i64,
        param: *const ::core::ffi::c_void,
        paramType: cuda_types::cublas::cudaDataType,
        executiontype: cuda_types::cublas::cudaDataType,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSrotmg_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        d1: *mut f32,
        d2: *mut f32,
        x1: *mut f32,
        y1: *const f32,
        param: *mut f32,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDrotmg_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        d1: *mut f64,
        d2: *mut f64,
        x1: *mut f64,
        y1: *const f64,
        param: *mut f64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasRotmgEx(
        handle: cuda_types::cublas::cublasHandle_t,
        d1: *mut ::core::ffi::c_void,
        d1Type: cuda_types::cublas::cudaDataType,
        d2: *mut ::core::ffi::c_void,
        d2Type: cuda_types::cublas::cudaDataType,
        x1: *mut ::core::ffi::c_void,
        x1Type: cuda_types::cublas::cudaDataType,
        y1: *const ::core::ffi::c_void,
        y1Type: cuda_types::cublas::cudaDataType,
        param: *mut ::core::ffi::c_void,
        paramType: cuda_types::cublas::cudaDataType,
        executiontype: cuda_types::cublas::cudaDataType,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSgemv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::core::ffi::c_int,
        x: *const f32,
        incx: ::core::ffi::c_int,
        beta: *const f32,
        y: *mut f32,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSgemv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const f32,
        A: *const f32,
        lda: cuda_types::cublas::i64,
        x: *const f32,
        incx: cuda_types::cublas::i64,
        beta: *const f32,
        y: *mut f32,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDgemv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f64,
        A: *const f64,
        lda: ::core::ffi::c_int,
        x: *const f64,
        incx: ::core::ffi::c_int,
        beta: *const f64,
        y: *mut f64,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDgemv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const f64,
        A: *const f64,
        lda: cuda_types::cublas::i64,
        x: *const f64,
        incx: cuda_types::cublas::i64,
        beta: *const f64,
        y: *mut f64,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgemv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        x: *const cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuComplex,
        y: *mut cuda_types::cublas::cuComplex,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgemv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
        x: *const cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuComplex,
        y: *mut cuda_types::cublas::cuComplex,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZgemv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        y: *mut cuda_types::cublas::cuDoubleComplex,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZgemv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        y: *mut cuda_types::cublas::cuDoubleComplex,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSgbmv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        kl: ::core::ffi::c_int,
        ku: ::core::ffi::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::core::ffi::c_int,
        x: *const f32,
        incx: ::core::ffi::c_int,
        beta: *const f32,
        y: *mut f32,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSgbmv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        kl: cuda_types::cublas::i64,
        ku: cuda_types::cublas::i64,
        alpha: *const f32,
        A: *const f32,
        lda: cuda_types::cublas::i64,
        x: *const f32,
        incx: cuda_types::cublas::i64,
        beta: *const f32,
        y: *mut f32,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDgbmv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        kl: ::core::ffi::c_int,
        ku: ::core::ffi::c_int,
        alpha: *const f64,
        A: *const f64,
        lda: ::core::ffi::c_int,
        x: *const f64,
        incx: ::core::ffi::c_int,
        beta: *const f64,
        y: *mut f64,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDgbmv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        kl: cuda_types::cublas::i64,
        ku: cuda_types::cublas::i64,
        alpha: *const f64,
        A: *const f64,
        lda: cuda_types::cublas::i64,
        x: *const f64,
        incx: cuda_types::cublas::i64,
        beta: *const f64,
        y: *mut f64,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgbmv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        kl: ::core::ffi::c_int,
        ku: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        x: *const cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuComplex,
        y: *mut cuda_types::cublas::cuComplex,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgbmv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        kl: cuda_types::cublas::i64,
        ku: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
        x: *const cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuComplex,
        y: *mut cuda_types::cublas::cuComplex,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZgbmv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        kl: ::core::ffi::c_int,
        ku: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        y: *mut cuda_types::cublas::cuDoubleComplex,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZgbmv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        kl: cuda_types::cublas::i64,
        ku: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        y: *mut cuda_types::cublas::cuDoubleComplex,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasStrmv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: ::core::ffi::c_int,
        A: *const f32,
        lda: ::core::ffi::c_int,
        x: *mut f32,
        incx: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasStrmv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: cuda_types::cublas::i64,
        A: *const f32,
        lda: cuda_types::cublas::i64,
        x: *mut f32,
        incx: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDtrmv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: ::core::ffi::c_int,
        A: *const f64,
        lda: ::core::ffi::c_int,
        x: *mut f64,
        incx: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDtrmv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: cuda_types::cublas::i64,
        A: *const f64,
        lda: cuda_types::cublas::i64,
        x: *mut f64,
        incx: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCtrmv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: ::core::ffi::c_int,
        A: *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        x: *mut cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCtrmv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: cuda_types::cublas::i64,
        A: *const cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
        x: *mut cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZtrmv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: ::core::ffi::c_int,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        x: *mut cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZtrmv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: cuda_types::cublas::i64,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
        x: *mut cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasStbmv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        A: *const f32,
        lda: ::core::ffi::c_int,
        x: *mut f32,
        incx: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasStbmv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        A: *const f32,
        lda: cuda_types::cublas::i64,
        x: *mut f32,
        incx: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDtbmv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        A: *const f64,
        lda: ::core::ffi::c_int,
        x: *mut f64,
        incx: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDtbmv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        A: *const f64,
        lda: cuda_types::cublas::i64,
        x: *mut f64,
        incx: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCtbmv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        A: *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        x: *mut cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCtbmv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        A: *const cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
        x: *mut cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZtbmv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        x: *mut cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZtbmv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
        x: *mut cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasStpmv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: ::core::ffi::c_int,
        AP: *const f32,
        x: *mut f32,
        incx: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasStpmv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: cuda_types::cublas::i64,
        AP: *const f32,
        x: *mut f32,
        incx: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDtpmv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: ::core::ffi::c_int,
        AP: *const f64,
        x: *mut f64,
        incx: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDtpmv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: cuda_types::cublas::i64,
        AP: *const f64,
        x: *mut f64,
        incx: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCtpmv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: ::core::ffi::c_int,
        AP: *const cuda_types::cublas::cuComplex,
        x: *mut cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCtpmv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: cuda_types::cublas::i64,
        AP: *const cuda_types::cublas::cuComplex,
        x: *mut cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZtpmv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: ::core::ffi::c_int,
        AP: *const cuda_types::cublas::cuDoubleComplex,
        x: *mut cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZtpmv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: cuda_types::cublas::i64,
        AP: *const cuda_types::cublas::cuDoubleComplex,
        x: *mut cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasStrsv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: ::core::ffi::c_int,
        A: *const f32,
        lda: ::core::ffi::c_int,
        x: *mut f32,
        incx: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasStrsv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: cuda_types::cublas::i64,
        A: *const f32,
        lda: cuda_types::cublas::i64,
        x: *mut f32,
        incx: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDtrsv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: ::core::ffi::c_int,
        A: *const f64,
        lda: ::core::ffi::c_int,
        x: *mut f64,
        incx: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDtrsv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: cuda_types::cublas::i64,
        A: *const f64,
        lda: cuda_types::cublas::i64,
        x: *mut f64,
        incx: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCtrsv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: ::core::ffi::c_int,
        A: *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        x: *mut cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCtrsv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: cuda_types::cublas::i64,
        A: *const cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
        x: *mut cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZtrsv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: ::core::ffi::c_int,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        x: *mut cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZtrsv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: cuda_types::cublas::i64,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
        x: *mut cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasStpsv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: ::core::ffi::c_int,
        AP: *const f32,
        x: *mut f32,
        incx: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasStpsv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: cuda_types::cublas::i64,
        AP: *const f32,
        x: *mut f32,
        incx: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDtpsv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: ::core::ffi::c_int,
        AP: *const f64,
        x: *mut f64,
        incx: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDtpsv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: cuda_types::cublas::i64,
        AP: *const f64,
        x: *mut f64,
        incx: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCtpsv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: ::core::ffi::c_int,
        AP: *const cuda_types::cublas::cuComplex,
        x: *mut cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCtpsv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: cuda_types::cublas::i64,
        AP: *const cuda_types::cublas::cuComplex,
        x: *mut cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZtpsv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: ::core::ffi::c_int,
        AP: *const cuda_types::cublas::cuDoubleComplex,
        x: *mut cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZtpsv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: cuda_types::cublas::i64,
        AP: *const cuda_types::cublas::cuDoubleComplex,
        x: *mut cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasStbsv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        A: *const f32,
        lda: ::core::ffi::c_int,
        x: *mut f32,
        incx: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasStbsv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        A: *const f32,
        lda: cuda_types::cublas::i64,
        x: *mut f32,
        incx: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDtbsv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        A: *const f64,
        lda: ::core::ffi::c_int,
        x: *mut f64,
        incx: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDtbsv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        A: *const f64,
        lda: cuda_types::cublas::i64,
        x: *mut f64,
        incx: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCtbsv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        A: *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        x: *mut cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCtbsv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        A: *const cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
        x: *mut cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZtbsv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        x: *mut cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZtbsv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
        x: *mut cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSsymv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::core::ffi::c_int,
        x: *const f32,
        incx: ::core::ffi::c_int,
        beta: *const f32,
        y: *mut f32,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSsymv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        alpha: *const f32,
        A: *const f32,
        lda: cuda_types::cublas::i64,
        x: *const f32,
        incx: cuda_types::cublas::i64,
        beta: *const f32,
        y: *mut f32,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDsymv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        alpha: *const f64,
        A: *const f64,
        lda: ::core::ffi::c_int,
        x: *const f64,
        incx: ::core::ffi::c_int,
        beta: *const f64,
        y: *mut f64,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDsymv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        alpha: *const f64,
        A: *const f64,
        lda: cuda_types::cublas::i64,
        x: *const f64,
        incx: cuda_types::cublas::i64,
        beta: *const f64,
        y: *mut f64,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCsymv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        x: *const cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuComplex,
        y: *mut cuda_types::cublas::cuComplex,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCsymv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
        x: *const cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuComplex,
        y: *mut cuda_types::cublas::cuComplex,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZsymv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        y: *mut cuda_types::cublas::cuDoubleComplex,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZsymv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        y: *mut cuda_types::cublas::cuDoubleComplex,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasChemv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        x: *const cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuComplex,
        y: *mut cuda_types::cublas::cuComplex,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasChemv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
        x: *const cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuComplex,
        y: *mut cuda_types::cublas::cuComplex,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZhemv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        y: *mut cuda_types::cublas::cuDoubleComplex,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZhemv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        y: *mut cuda_types::cublas::cuDoubleComplex,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSsbmv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::core::ffi::c_int,
        x: *const f32,
        incx: ::core::ffi::c_int,
        beta: *const f32,
        y: *mut f32,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSsbmv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const f32,
        A: *const f32,
        lda: cuda_types::cublas::i64,
        x: *const f32,
        incx: cuda_types::cublas::i64,
        beta: *const f32,
        y: *mut f32,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDsbmv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const f64,
        A: *const f64,
        lda: ::core::ffi::c_int,
        x: *const f64,
        incx: ::core::ffi::c_int,
        beta: *const f64,
        y: *mut f64,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDsbmv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const f64,
        A: *const f64,
        lda: cuda_types::cublas::i64,
        x: *const f64,
        incx: cuda_types::cublas::i64,
        beta: *const f64,
        y: *mut f64,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasChbmv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        x: *const cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuComplex,
        y: *mut cuda_types::cublas::cuComplex,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasChbmv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
        x: *const cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuComplex,
        y: *mut cuda_types::cublas::cuComplex,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZhbmv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        y: *mut cuda_types::cublas::cuDoubleComplex,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZhbmv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        y: *mut cuda_types::cublas::cuDoubleComplex,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSspmv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        AP: *const f32,
        x: *const f32,
        incx: ::core::ffi::c_int,
        beta: *const f32,
        y: *mut f32,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSspmv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        alpha: *const f32,
        AP: *const f32,
        x: *const f32,
        incx: cuda_types::cublas::i64,
        beta: *const f32,
        y: *mut f32,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDspmv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        alpha: *const f64,
        AP: *const f64,
        x: *const f64,
        incx: ::core::ffi::c_int,
        beta: *const f64,
        y: *mut f64,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDspmv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        alpha: *const f64,
        AP: *const f64,
        x: *const f64,
        incx: cuda_types::cublas::i64,
        beta: *const f64,
        y: *mut f64,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasChpmv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        AP: *const cuda_types::cublas::cuComplex,
        x: *const cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuComplex,
        y: *mut cuda_types::cublas::cuComplex,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasChpmv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        AP: *const cuda_types::cublas::cuComplex,
        x: *const cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuComplex,
        y: *mut cuda_types::cublas::cuComplex,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZhpmv_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        AP: *const cuda_types::cublas::cuDoubleComplex,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        y: *mut cuda_types::cublas::cuDoubleComplex,
        incy: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZhpmv_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        AP: *const cuda_types::cublas::cuDoubleComplex,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        y: *mut cuda_types::cublas::cuDoubleComplex,
        incy: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSger_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        x: *const f32,
        incx: ::core::ffi::c_int,
        y: *const f32,
        incy: ::core::ffi::c_int,
        A: *mut f32,
        lda: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSger_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const f32,
        x: *const f32,
        incx: cuda_types::cublas::i64,
        y: *const f32,
        incy: cuda_types::cublas::i64,
        A: *mut f32,
        lda: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDger_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f64,
        x: *const f64,
        incx: ::core::ffi::c_int,
        y: *const f64,
        incy: ::core::ffi::c_int,
        A: *mut f64,
        lda: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDger_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const f64,
        x: *const f64,
        incx: cuda_types::cublas::i64,
        y: *const f64,
        incy: cuda_types::cublas::i64,
        A: *mut f64,
        lda: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgeru_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        x: *const cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
        y: *const cuda_types::cublas::cuComplex,
        incy: ::core::ffi::c_int,
        A: *mut cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgeru_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        x: *const cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
        y: *const cuda_types::cublas::cuComplex,
        incy: cuda_types::cublas::i64,
        A: *mut cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgerc_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        x: *const cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
        y: *const cuda_types::cublas::cuComplex,
        incy: ::core::ffi::c_int,
        A: *mut cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgerc_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        x: *const cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
        y: *const cuda_types::cublas::cuComplex,
        incy: cuda_types::cublas::i64,
        A: *mut cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZgeru_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
        y: *const cuda_types::cublas::cuDoubleComplex,
        incy: ::core::ffi::c_int,
        A: *mut cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZgeru_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
        y: *const cuda_types::cublas::cuDoubleComplex,
        incy: cuda_types::cublas::i64,
        A: *mut cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZgerc_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
        y: *const cuda_types::cublas::cuDoubleComplex,
        incy: ::core::ffi::c_int,
        A: *mut cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZgerc_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
        y: *const cuda_types::cublas::cuDoubleComplex,
        incy: cuda_types::cublas::i64,
        A: *mut cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSsyr_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        x: *const f32,
        incx: ::core::ffi::c_int,
        A: *mut f32,
        lda: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSsyr_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        alpha: *const f32,
        x: *const f32,
        incx: cuda_types::cublas::i64,
        A: *mut f32,
        lda: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDsyr_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        alpha: *const f64,
        x: *const f64,
        incx: ::core::ffi::c_int,
        A: *mut f64,
        lda: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDsyr_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        alpha: *const f64,
        x: *const f64,
        incx: cuda_types::cublas::i64,
        A: *mut f64,
        lda: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCsyr_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        x: *const cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
        A: *mut cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCsyr_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        x: *const cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
        A: *mut cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZsyr_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
        A: *mut cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZsyr_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
        A: *mut cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCher_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        x: *const cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
        A: *mut cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCher_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        alpha: *const f32,
        x: *const cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
        A: *mut cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZher_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        alpha: *const f64,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
        A: *mut cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZher_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        alpha: *const f64,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
        A: *mut cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSspr_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        x: *const f32,
        incx: ::core::ffi::c_int,
        AP: *mut f32,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSspr_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        alpha: *const f32,
        x: *const f32,
        incx: cuda_types::cublas::i64,
        AP: *mut f32,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDspr_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        alpha: *const f64,
        x: *const f64,
        incx: ::core::ffi::c_int,
        AP: *mut f64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDspr_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        alpha: *const f64,
        x: *const f64,
        incx: cuda_types::cublas::i64,
        AP: *mut f64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasChpr_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        x: *const cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
        AP: *mut cuda_types::cublas::cuComplex,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasChpr_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        alpha: *const f32,
        x: *const cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
        AP: *mut cuda_types::cublas::cuComplex,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZhpr_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        alpha: *const f64,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
        AP: *mut cuda_types::cublas::cuDoubleComplex,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZhpr_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        alpha: *const f64,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
        AP: *mut cuda_types::cublas::cuDoubleComplex,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSsyr2_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        x: *const f32,
        incx: ::core::ffi::c_int,
        y: *const f32,
        incy: ::core::ffi::c_int,
        A: *mut f32,
        lda: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSsyr2_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        alpha: *const f32,
        x: *const f32,
        incx: cuda_types::cublas::i64,
        y: *const f32,
        incy: cuda_types::cublas::i64,
        A: *mut f32,
        lda: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDsyr2_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        alpha: *const f64,
        x: *const f64,
        incx: ::core::ffi::c_int,
        y: *const f64,
        incy: ::core::ffi::c_int,
        A: *mut f64,
        lda: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDsyr2_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        alpha: *const f64,
        x: *const f64,
        incx: cuda_types::cublas::i64,
        y: *const f64,
        incy: cuda_types::cublas::i64,
        A: *mut f64,
        lda: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCsyr2_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        x: *const cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
        y: *const cuda_types::cublas::cuComplex,
        incy: ::core::ffi::c_int,
        A: *mut cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCsyr2_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        x: *const cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
        y: *const cuda_types::cublas::cuComplex,
        incy: cuda_types::cublas::i64,
        A: *mut cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZsyr2_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
        y: *const cuda_types::cublas::cuDoubleComplex,
        incy: ::core::ffi::c_int,
        A: *mut cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZsyr2_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
        y: *const cuda_types::cublas::cuDoubleComplex,
        incy: cuda_types::cublas::i64,
        A: *mut cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCher2_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        x: *const cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
        y: *const cuda_types::cublas::cuComplex,
        incy: ::core::ffi::c_int,
        A: *mut cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCher2_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        x: *const cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
        y: *const cuda_types::cublas::cuComplex,
        incy: cuda_types::cublas::i64,
        A: *mut cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZher2_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
        y: *const cuda_types::cublas::cuDoubleComplex,
        incy: ::core::ffi::c_int,
        A: *mut cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZher2_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
        y: *const cuda_types::cublas::cuDoubleComplex,
        incy: cuda_types::cublas::i64,
        A: *mut cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSspr2_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        x: *const f32,
        incx: ::core::ffi::c_int,
        y: *const f32,
        incy: ::core::ffi::c_int,
        AP: *mut f32,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSspr2_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        alpha: *const f32,
        x: *const f32,
        incx: cuda_types::cublas::i64,
        y: *const f32,
        incy: cuda_types::cublas::i64,
        AP: *mut f32,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDspr2_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        alpha: *const f64,
        x: *const f64,
        incx: ::core::ffi::c_int,
        y: *const f64,
        incy: ::core::ffi::c_int,
        AP: *mut f64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDspr2_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        alpha: *const f64,
        x: *const f64,
        incx: cuda_types::cublas::i64,
        y: *const f64,
        incy: cuda_types::cublas::i64,
        AP: *mut f64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasChpr2_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        x: *const cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
        y: *const cuda_types::cublas::cuComplex,
        incy: ::core::ffi::c_int,
        AP: *mut cuda_types::cublas::cuComplex,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasChpr2_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        x: *const cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
        y: *const cuda_types::cublas::cuComplex,
        incy: cuda_types::cublas::i64,
        AP: *mut cuda_types::cublas::cuComplex,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZhpr2_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
        y: *const cuda_types::cublas::cuDoubleComplex,
        incy: ::core::ffi::c_int,
        AP: *mut cuda_types::cublas::cuDoubleComplex,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZhpr2_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
        y: *const cuda_types::cublas::cuDoubleComplex,
        incy: cuda_types::cublas::i64,
        AP: *mut cuda_types::cublas::cuDoubleComplex,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSgemvBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        Aarray: *const *const f32,
        lda: ::core::ffi::c_int,
        xarray: *const *const f32,
        incx: ::core::ffi::c_int,
        beta: *const f32,
        yarray: *const *mut f32,
        incy: ::core::ffi::c_int,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSgemvBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const f32,
        Aarray: *const *const f32,
        lda: cuda_types::cublas::i64,
        xarray: *const *const f32,
        incx: cuda_types::cublas::i64,
        beta: *const f32,
        yarray: *const *mut f32,
        incy: cuda_types::cublas::i64,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDgemvBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f64,
        Aarray: *const *const f64,
        lda: ::core::ffi::c_int,
        xarray: *const *const f64,
        incx: ::core::ffi::c_int,
        beta: *const f64,
        yarray: *const *mut f64,
        incy: ::core::ffi::c_int,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDgemvBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const f64,
        Aarray: *const *const f64,
        lda: cuda_types::cublas::i64,
        xarray: *const *const f64,
        incx: cuda_types::cublas::i64,
        beta: *const f64,
        yarray: *const *mut f64,
        incy: cuda_types::cublas::i64,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgemvBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        Aarray: *const *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        xarray: *const *const cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuComplex,
        yarray: *const *mut cuda_types::cublas::cuComplex,
        incy: ::core::ffi::c_int,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgemvBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        Aarray: *const *const cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
        xarray: *const *const cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuComplex,
        yarray: *const *mut cuda_types::cublas::cuComplex,
        incy: cuda_types::cublas::i64,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZgemvBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        Aarray: *const *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        xarray: *const *const cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        yarray: *const *mut cuda_types::cublas::cuDoubleComplex,
        incy: ::core::ffi::c_int,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZgemvBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        Aarray: *const *const cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
        xarray: *const *const cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        yarray: *const *mut cuda_types::cublas::cuDoubleComplex,
        incy: cuda_types::cublas::i64,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasHSHgemvBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        Aarray: *const *const cuda_types::cublas::__half,
        lda: ::core::ffi::c_int,
        xarray: *const *const cuda_types::cublas::__half,
        incx: ::core::ffi::c_int,
        beta: *const f32,
        yarray: *const *mut cuda_types::cublas::__half,
        incy: ::core::ffi::c_int,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasHSHgemvBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const f32,
        Aarray: *const *const cuda_types::cublas::__half,
        lda: cuda_types::cublas::i64,
        xarray: *const *const cuda_types::cublas::__half,
        incx: cuda_types::cublas::i64,
        beta: *const f32,
        yarray: *const *mut cuda_types::cublas::__half,
        incy: cuda_types::cublas::i64,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasHSSgemvBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        Aarray: *const *const cuda_types::cublas::__half,
        lda: ::core::ffi::c_int,
        xarray: *const *const cuda_types::cublas::__half,
        incx: ::core::ffi::c_int,
        beta: *const f32,
        yarray: *const *mut f32,
        incy: ::core::ffi::c_int,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasHSSgemvBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const f32,
        Aarray: *const *const cuda_types::cublas::__half,
        lda: cuda_types::cublas::i64,
        xarray: *const *const cuda_types::cublas::__half,
        incx: cuda_types::cublas::i64,
        beta: *const f32,
        yarray: *const *mut f32,
        incy: cuda_types::cublas::i64,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasTSTgemvBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        Aarray: *const *const cuda_types::cublas::__nv_bfloat16,
        lda: ::core::ffi::c_int,
        xarray: *const *const cuda_types::cublas::__nv_bfloat16,
        incx: ::core::ffi::c_int,
        beta: *const f32,
        yarray: *const *mut cuda_types::cublas::__nv_bfloat16,
        incy: ::core::ffi::c_int,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasTSTgemvBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const f32,
        Aarray: *const *const cuda_types::cublas::__nv_bfloat16,
        lda: cuda_types::cublas::i64,
        xarray: *const *const cuda_types::cublas::__nv_bfloat16,
        incx: cuda_types::cublas::i64,
        beta: *const f32,
        yarray: *const *mut cuda_types::cublas::__nv_bfloat16,
        incy: cuda_types::cublas::i64,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasTSSgemvBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        Aarray: *const *const cuda_types::cublas::__nv_bfloat16,
        lda: ::core::ffi::c_int,
        xarray: *const *const cuda_types::cublas::__nv_bfloat16,
        incx: ::core::ffi::c_int,
        beta: *const f32,
        yarray: *const *mut f32,
        incy: ::core::ffi::c_int,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasTSSgemvBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const f32,
        Aarray: *const *const cuda_types::cublas::__nv_bfloat16,
        lda: cuda_types::cublas::i64,
        xarray: *const *const cuda_types::cublas::__nv_bfloat16,
        incx: cuda_types::cublas::i64,
        beta: *const f32,
        yarray: *const *mut f32,
        incy: cuda_types::cublas::i64,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSgemvStridedBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::core::ffi::c_int,
        strideA: ::core::ffi::c_longlong,
        x: *const f32,
        incx: ::core::ffi::c_int,
        stridex: ::core::ffi::c_longlong,
        beta: *const f32,
        y: *mut f32,
        incy: ::core::ffi::c_int,
        stridey: ::core::ffi::c_longlong,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSgemvStridedBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const f32,
        A: *const f32,
        lda: cuda_types::cublas::i64,
        strideA: ::core::ffi::c_longlong,
        x: *const f32,
        incx: cuda_types::cublas::i64,
        stridex: ::core::ffi::c_longlong,
        beta: *const f32,
        y: *mut f32,
        incy: cuda_types::cublas::i64,
        stridey: ::core::ffi::c_longlong,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDgemvStridedBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f64,
        A: *const f64,
        lda: ::core::ffi::c_int,
        strideA: ::core::ffi::c_longlong,
        x: *const f64,
        incx: ::core::ffi::c_int,
        stridex: ::core::ffi::c_longlong,
        beta: *const f64,
        y: *mut f64,
        incy: ::core::ffi::c_int,
        stridey: ::core::ffi::c_longlong,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDgemvStridedBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const f64,
        A: *const f64,
        lda: cuda_types::cublas::i64,
        strideA: ::core::ffi::c_longlong,
        x: *const f64,
        incx: cuda_types::cublas::i64,
        stridex: ::core::ffi::c_longlong,
        beta: *const f64,
        y: *mut f64,
        incy: cuda_types::cublas::i64,
        stridey: ::core::ffi::c_longlong,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgemvStridedBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        strideA: ::core::ffi::c_longlong,
        x: *const cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
        stridex: ::core::ffi::c_longlong,
        beta: *const cuda_types::cublas::cuComplex,
        y: *mut cuda_types::cublas::cuComplex,
        incy: ::core::ffi::c_int,
        stridey: ::core::ffi::c_longlong,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgemvStridedBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
        strideA: ::core::ffi::c_longlong,
        x: *const cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
        stridex: ::core::ffi::c_longlong,
        beta: *const cuda_types::cublas::cuComplex,
        y: *mut cuda_types::cublas::cuComplex,
        incy: cuda_types::cublas::i64,
        stridey: ::core::ffi::c_longlong,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZgemvStridedBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        strideA: ::core::ffi::c_longlong,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
        stridex: ::core::ffi::c_longlong,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        y: *mut cuda_types::cublas::cuDoubleComplex,
        incy: ::core::ffi::c_int,
        stridey: ::core::ffi::c_longlong,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZgemvStridedBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
        strideA: ::core::ffi::c_longlong,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
        stridex: ::core::ffi::c_longlong,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        y: *mut cuda_types::cublas::cuDoubleComplex,
        incy: cuda_types::cublas::i64,
        stridey: ::core::ffi::c_longlong,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasHSHgemvStridedBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        A: *const cuda_types::cublas::__half,
        lda: ::core::ffi::c_int,
        strideA: ::core::ffi::c_longlong,
        x: *const cuda_types::cublas::__half,
        incx: ::core::ffi::c_int,
        stridex: ::core::ffi::c_longlong,
        beta: *const f32,
        y: *mut cuda_types::cublas::__half,
        incy: ::core::ffi::c_int,
        stridey: ::core::ffi::c_longlong,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasHSHgemvStridedBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const f32,
        A: *const cuda_types::cublas::__half,
        lda: cuda_types::cublas::i64,
        strideA: ::core::ffi::c_longlong,
        x: *const cuda_types::cublas::__half,
        incx: cuda_types::cublas::i64,
        stridex: ::core::ffi::c_longlong,
        beta: *const f32,
        y: *mut cuda_types::cublas::__half,
        incy: cuda_types::cublas::i64,
        stridey: ::core::ffi::c_longlong,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasHSSgemvStridedBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        A: *const cuda_types::cublas::__half,
        lda: ::core::ffi::c_int,
        strideA: ::core::ffi::c_longlong,
        x: *const cuda_types::cublas::__half,
        incx: ::core::ffi::c_int,
        stridex: ::core::ffi::c_longlong,
        beta: *const f32,
        y: *mut f32,
        incy: ::core::ffi::c_int,
        stridey: ::core::ffi::c_longlong,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasHSSgemvStridedBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const f32,
        A: *const cuda_types::cublas::__half,
        lda: cuda_types::cublas::i64,
        strideA: ::core::ffi::c_longlong,
        x: *const cuda_types::cublas::__half,
        incx: cuda_types::cublas::i64,
        stridex: ::core::ffi::c_longlong,
        beta: *const f32,
        y: *mut f32,
        incy: cuda_types::cublas::i64,
        stridey: ::core::ffi::c_longlong,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasTSTgemvStridedBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        A: *const cuda_types::cublas::__nv_bfloat16,
        lda: ::core::ffi::c_int,
        strideA: ::core::ffi::c_longlong,
        x: *const cuda_types::cublas::__nv_bfloat16,
        incx: ::core::ffi::c_int,
        stridex: ::core::ffi::c_longlong,
        beta: *const f32,
        y: *mut cuda_types::cublas::__nv_bfloat16,
        incy: ::core::ffi::c_int,
        stridey: ::core::ffi::c_longlong,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasTSTgemvStridedBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const f32,
        A: *const cuda_types::cublas::__nv_bfloat16,
        lda: cuda_types::cublas::i64,
        strideA: ::core::ffi::c_longlong,
        x: *const cuda_types::cublas::__nv_bfloat16,
        incx: cuda_types::cublas::i64,
        stridex: ::core::ffi::c_longlong,
        beta: *const f32,
        y: *mut cuda_types::cublas::__nv_bfloat16,
        incy: cuda_types::cublas::i64,
        stridey: ::core::ffi::c_longlong,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasTSSgemvStridedBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        A: *const cuda_types::cublas::__nv_bfloat16,
        lda: ::core::ffi::c_int,
        strideA: ::core::ffi::c_longlong,
        x: *const cuda_types::cublas::__nv_bfloat16,
        incx: ::core::ffi::c_int,
        stridex: ::core::ffi::c_longlong,
        beta: *const f32,
        y: *mut f32,
        incy: ::core::ffi::c_int,
        stridey: ::core::ffi::c_longlong,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasTSSgemvStridedBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const f32,
        A: *const cuda_types::cublas::__nv_bfloat16,
        lda: cuda_types::cublas::i64,
        strideA: ::core::ffi::c_longlong,
        x: *const cuda_types::cublas::__nv_bfloat16,
        incx: cuda_types::cublas::i64,
        stridex: ::core::ffi::c_longlong,
        beta: *const f32,
        y: *mut f32,
        incy: cuda_types::cublas::i64,
        stridey: ::core::ffi::c_longlong,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSgemm_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::core::ffi::c_int,
        B: *const f32,
        ldb: ::core::ffi::c_int,
        beta: *const f32,
        C: *mut f32,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSgemm_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const f32,
        A: *const f32,
        lda: cuda_types::cublas::i64,
        B: *const f32,
        ldb: cuda_types::cublas::i64,
        beta: *const f32,
        C: *mut f32,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDgemm_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const f64,
        A: *const f64,
        lda: ::core::ffi::c_int,
        B: *const f64,
        ldb: ::core::ffi::c_int,
        beta: *const f64,
        C: *mut f64,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDgemm_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const f64,
        A: *const f64,
        lda: cuda_types::cublas::i64,
        B: *const f64,
        ldb: cuda_types::cublas::i64,
        beta: *const f64,
        C: *mut f64,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgemm_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        B: *const cuda_types::cublas::cuComplex,
        ldb: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuComplex,
        C: *mut cuda_types::cublas::cuComplex,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgemm_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
        B: *const cuda_types::cublas::cuComplex,
        ldb: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuComplex,
        C: *mut cuda_types::cublas::cuComplex,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgemm3m(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        B: *const cuda_types::cublas::cuComplex,
        ldb: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuComplex,
        C: *mut cuda_types::cublas::cuComplex,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgemm3m_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
        B: *const cuda_types::cublas::cuComplex,
        ldb: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuComplex,
        C: *mut cuda_types::cublas::cuComplex,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgemm3mEx(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const ::core::ffi::c_void,
        Atype: cuda_types::cublas::cudaDataType,
        lda: ::core::ffi::c_int,
        B: *const ::core::ffi::c_void,
        Btype: cuda_types::cublas::cudaDataType,
        ldb: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuComplex,
        C: *mut ::core::ffi::c_void,
        Ctype: cuda_types::cublas::cudaDataType,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgemm3mEx_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const ::core::ffi::c_void,
        Atype: cuda_types::cublas::cudaDataType,
        lda: cuda_types::cublas::i64,
        B: *const ::core::ffi::c_void,
        Btype: cuda_types::cublas::cudaDataType,
        ldb: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuComplex,
        C: *mut ::core::ffi::c_void,
        Ctype: cuda_types::cublas::cudaDataType,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZgemm_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        B: *const cuda_types::cublas::cuDoubleComplex,
        ldb: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        C: *mut cuda_types::cublas::cuDoubleComplex,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZgemm_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
        B: *const cuda_types::cublas::cuDoubleComplex,
        ldb: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        C: *mut cuda_types::cublas::cuDoubleComplex,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZgemm3m(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        B: *const cuda_types::cublas::cuDoubleComplex,
        ldb: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        C: *mut cuda_types::cublas::cuDoubleComplex,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZgemm3m_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
        B: *const cuda_types::cublas::cuDoubleComplex,
        ldb: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        C: *mut cuda_types::cublas::cuDoubleComplex,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasHgemm(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::__half,
        A: *const cuda_types::cublas::__half,
        lda: ::core::ffi::c_int,
        B: *const cuda_types::cublas::__half,
        ldb: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::__half,
        C: *mut cuda_types::cublas::__half,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasHgemm_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::__half,
        A: *const cuda_types::cublas::__half,
        lda: cuda_types::cublas::i64,
        B: *const cuda_types::cublas::__half,
        ldb: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::__half,
        C: *mut cuda_types::cublas::__half,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSgemmEx(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const f32,
        A: *const ::core::ffi::c_void,
        Atype: cuda_types::cublas::cudaDataType,
        lda: ::core::ffi::c_int,
        B: *const ::core::ffi::c_void,
        Btype: cuda_types::cublas::cudaDataType,
        ldb: ::core::ffi::c_int,
        beta: *const f32,
        C: *mut ::core::ffi::c_void,
        Ctype: cuda_types::cublas::cudaDataType,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSgemmEx_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const f32,
        A: *const ::core::ffi::c_void,
        Atype: cuda_types::cublas::cudaDataType,
        lda: cuda_types::cublas::i64,
        B: *const ::core::ffi::c_void,
        Btype: cuda_types::cublas::cudaDataType,
        ldb: cuda_types::cublas::i64,
        beta: *const f32,
        C: *mut ::core::ffi::c_void,
        Ctype: cuda_types::cublas::cudaDataType,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasGemmEx(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        Atype: cuda_types::cublas::cudaDataType,
        lda: ::core::ffi::c_int,
        B: *const ::core::ffi::c_void,
        Btype: cuda_types::cublas::cudaDataType,
        ldb: ::core::ffi::c_int,
        beta: *const ::core::ffi::c_void,
        C: *mut ::core::ffi::c_void,
        Ctype: cuda_types::cublas::cudaDataType,
        ldc: ::core::ffi::c_int,
        computeType: cuda_types::cublas::cublasComputeType_t,
        algo: cuda_types::cublas::cublasGemmAlgo_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasGemmEx_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        Atype: cuda_types::cublas::cudaDataType,
        lda: cuda_types::cublas::i64,
        B: *const ::core::ffi::c_void,
        Btype: cuda_types::cublas::cudaDataType,
        ldb: cuda_types::cublas::i64,
        beta: *const ::core::ffi::c_void,
        C: *mut ::core::ffi::c_void,
        Ctype: cuda_types::cublas::cudaDataType,
        ldc: cuda_types::cublas::i64,
        computeType: cuda_types::cublas::cublasComputeType_t,
        algo: cuda_types::cublas::cublasGemmAlgo_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgemmEx(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const ::core::ffi::c_void,
        Atype: cuda_types::cublas::cudaDataType,
        lda: ::core::ffi::c_int,
        B: *const ::core::ffi::c_void,
        Btype: cuda_types::cublas::cudaDataType,
        ldb: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuComplex,
        C: *mut ::core::ffi::c_void,
        Ctype: cuda_types::cublas::cudaDataType,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgemmEx_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const ::core::ffi::c_void,
        Atype: cuda_types::cublas::cudaDataType,
        lda: cuda_types::cublas::i64,
        B: *const ::core::ffi::c_void,
        Btype: cuda_types::cublas::cudaDataType,
        ldb: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuComplex,
        C: *mut ::core::ffi::c_void,
        Ctype: cuda_types::cublas::cudaDataType,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSsyrk_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::core::ffi::c_int,
        beta: *const f32,
        C: *mut f32,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSsyrk_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const f32,
        A: *const f32,
        lda: cuda_types::cublas::i64,
        beta: *const f32,
        C: *mut f32,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDsyrk_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const f64,
        A: *const f64,
        lda: ::core::ffi::c_int,
        beta: *const f64,
        C: *mut f64,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDsyrk_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const f64,
        A: *const f64,
        lda: cuda_types::cublas::i64,
        beta: *const f64,
        C: *mut f64,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCsyrk_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuComplex,
        C: *mut cuda_types::cublas::cuComplex,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCsyrk_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuComplex,
        C: *mut cuda_types::cublas::cuComplex,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZsyrk_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        C: *mut cuda_types::cublas::cuDoubleComplex,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZsyrk_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        C: *mut cuda_types::cublas::cuDoubleComplex,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCsyrkEx(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const ::core::ffi::c_void,
        Atype: cuda_types::cublas::cudaDataType,
        lda: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuComplex,
        C: *mut ::core::ffi::c_void,
        Ctype: cuda_types::cublas::cudaDataType,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCsyrkEx_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const ::core::ffi::c_void,
        Atype: cuda_types::cublas::cudaDataType,
        lda: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuComplex,
        C: *mut ::core::ffi::c_void,
        Ctype: cuda_types::cublas::cudaDataType,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCsyrk3mEx(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const ::core::ffi::c_void,
        Atype: cuda_types::cublas::cudaDataType,
        lda: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuComplex,
        C: *mut ::core::ffi::c_void,
        Ctype: cuda_types::cublas::cudaDataType,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCsyrk3mEx_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const ::core::ffi::c_void,
        Atype: cuda_types::cublas::cudaDataType,
        lda: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuComplex,
        C: *mut ::core::ffi::c_void,
        Ctype: cuda_types::cublas::cudaDataType,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCherk_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const f32,
        A: *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        beta: *const f32,
        C: *mut cuda_types::cublas::cuComplex,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCherk_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const f32,
        A: *const cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
        beta: *const f32,
        C: *mut cuda_types::cublas::cuComplex,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZherk_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const f64,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        beta: *const f64,
        C: *mut cuda_types::cublas::cuDoubleComplex,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZherk_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const f64,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
        beta: *const f64,
        C: *mut cuda_types::cublas::cuDoubleComplex,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCherkEx(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const f32,
        A: *const ::core::ffi::c_void,
        Atype: cuda_types::cublas::cudaDataType,
        lda: ::core::ffi::c_int,
        beta: *const f32,
        C: *mut ::core::ffi::c_void,
        Ctype: cuda_types::cublas::cudaDataType,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCherkEx_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const f32,
        A: *const ::core::ffi::c_void,
        Atype: cuda_types::cublas::cudaDataType,
        lda: cuda_types::cublas::i64,
        beta: *const f32,
        C: *mut ::core::ffi::c_void,
        Ctype: cuda_types::cublas::cudaDataType,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCherk3mEx(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const f32,
        A: *const ::core::ffi::c_void,
        Atype: cuda_types::cublas::cudaDataType,
        lda: ::core::ffi::c_int,
        beta: *const f32,
        C: *mut ::core::ffi::c_void,
        Ctype: cuda_types::cublas::cudaDataType,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCherk3mEx_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const f32,
        A: *const ::core::ffi::c_void,
        Atype: cuda_types::cublas::cudaDataType,
        lda: cuda_types::cublas::i64,
        beta: *const f32,
        C: *mut ::core::ffi::c_void,
        Ctype: cuda_types::cublas::cudaDataType,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSsyr2k_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::core::ffi::c_int,
        B: *const f32,
        ldb: ::core::ffi::c_int,
        beta: *const f32,
        C: *mut f32,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSsyr2k_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const f32,
        A: *const f32,
        lda: cuda_types::cublas::i64,
        B: *const f32,
        ldb: cuda_types::cublas::i64,
        beta: *const f32,
        C: *mut f32,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDsyr2k_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const f64,
        A: *const f64,
        lda: ::core::ffi::c_int,
        B: *const f64,
        ldb: ::core::ffi::c_int,
        beta: *const f64,
        C: *mut f64,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDsyr2k_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const f64,
        A: *const f64,
        lda: cuda_types::cublas::i64,
        B: *const f64,
        ldb: cuda_types::cublas::i64,
        beta: *const f64,
        C: *mut f64,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCsyr2k_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        B: *const cuda_types::cublas::cuComplex,
        ldb: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuComplex,
        C: *mut cuda_types::cublas::cuComplex,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCsyr2k_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
        B: *const cuda_types::cublas::cuComplex,
        ldb: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuComplex,
        C: *mut cuda_types::cublas::cuComplex,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZsyr2k_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        B: *const cuda_types::cublas::cuDoubleComplex,
        ldb: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        C: *mut cuda_types::cublas::cuDoubleComplex,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZsyr2k_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
        B: *const cuda_types::cublas::cuDoubleComplex,
        ldb: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        C: *mut cuda_types::cublas::cuDoubleComplex,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCher2k_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        B: *const cuda_types::cublas::cuComplex,
        ldb: ::core::ffi::c_int,
        beta: *const f32,
        C: *mut cuda_types::cublas::cuComplex,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCher2k_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
        B: *const cuda_types::cublas::cuComplex,
        ldb: cuda_types::cublas::i64,
        beta: *const f32,
        C: *mut cuda_types::cublas::cuComplex,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZher2k_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        B: *const cuda_types::cublas::cuDoubleComplex,
        ldb: ::core::ffi::c_int,
        beta: *const f64,
        C: *mut cuda_types::cublas::cuDoubleComplex,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZher2k_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
        B: *const cuda_types::cublas::cuDoubleComplex,
        ldb: cuda_types::cublas::i64,
        beta: *const f64,
        C: *mut cuda_types::cublas::cuDoubleComplex,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSsyrkx(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::core::ffi::c_int,
        B: *const f32,
        ldb: ::core::ffi::c_int,
        beta: *const f32,
        C: *mut f32,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSsyrkx_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const f32,
        A: *const f32,
        lda: cuda_types::cublas::i64,
        B: *const f32,
        ldb: cuda_types::cublas::i64,
        beta: *const f32,
        C: *mut f32,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDsyrkx(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const f64,
        A: *const f64,
        lda: ::core::ffi::c_int,
        B: *const f64,
        ldb: ::core::ffi::c_int,
        beta: *const f64,
        C: *mut f64,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDsyrkx_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const f64,
        A: *const f64,
        lda: cuda_types::cublas::i64,
        B: *const f64,
        ldb: cuda_types::cublas::i64,
        beta: *const f64,
        C: *mut f64,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCsyrkx(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        B: *const cuda_types::cublas::cuComplex,
        ldb: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuComplex,
        C: *mut cuda_types::cublas::cuComplex,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCsyrkx_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
        B: *const cuda_types::cublas::cuComplex,
        ldb: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuComplex,
        C: *mut cuda_types::cublas::cuComplex,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZsyrkx(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        B: *const cuda_types::cublas::cuDoubleComplex,
        ldb: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        C: *mut cuda_types::cublas::cuDoubleComplex,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZsyrkx_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
        B: *const cuda_types::cublas::cuDoubleComplex,
        ldb: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        C: *mut cuda_types::cublas::cuDoubleComplex,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCherkx(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        B: *const cuda_types::cublas::cuComplex,
        ldb: ::core::ffi::c_int,
        beta: *const f32,
        C: *mut cuda_types::cublas::cuComplex,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCherkx_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
        B: *const cuda_types::cublas::cuComplex,
        ldb: cuda_types::cublas::i64,
        beta: *const f32,
        C: *mut cuda_types::cublas::cuComplex,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZherkx(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        B: *const cuda_types::cublas::cuDoubleComplex,
        ldb: ::core::ffi::c_int,
        beta: *const f64,
        C: *mut cuda_types::cublas::cuDoubleComplex,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZherkx_64(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
        B: *const cuda_types::cublas::cuDoubleComplex,
        ldb: cuda_types::cublas::i64,
        beta: *const f64,
        C: *mut cuda_types::cublas::cuDoubleComplex,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSsymm_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::core::ffi::c_int,
        B: *const f32,
        ldb: ::core::ffi::c_int,
        beta: *const f32,
        C: *mut f32,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSsymm_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const f32,
        A: *const f32,
        lda: cuda_types::cublas::i64,
        B: *const f32,
        ldb: cuda_types::cublas::i64,
        beta: *const f32,
        C: *mut f32,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDsymm_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f64,
        A: *const f64,
        lda: ::core::ffi::c_int,
        B: *const f64,
        ldb: ::core::ffi::c_int,
        beta: *const f64,
        C: *mut f64,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDsymm_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const f64,
        A: *const f64,
        lda: cuda_types::cublas::i64,
        B: *const f64,
        ldb: cuda_types::cublas::i64,
        beta: *const f64,
        C: *mut f64,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCsymm_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        B: *const cuda_types::cublas::cuComplex,
        ldb: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuComplex,
        C: *mut cuda_types::cublas::cuComplex,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCsymm_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
        B: *const cuda_types::cublas::cuComplex,
        ldb: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuComplex,
        C: *mut cuda_types::cublas::cuComplex,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZsymm_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        B: *const cuda_types::cublas::cuDoubleComplex,
        ldb: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        C: *mut cuda_types::cublas::cuDoubleComplex,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZsymm_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
        B: *const cuda_types::cublas::cuDoubleComplex,
        ldb: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        C: *mut cuda_types::cublas::cuDoubleComplex,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasChemm_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        B: *const cuda_types::cublas::cuComplex,
        ldb: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuComplex,
        C: *mut cuda_types::cublas::cuComplex,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasChemm_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
        B: *const cuda_types::cublas::cuComplex,
        ldb: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuComplex,
        C: *mut cuda_types::cublas::cuComplex,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZhemm_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        B: *const cuda_types::cublas::cuDoubleComplex,
        ldb: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        C: *mut cuda_types::cublas::cuDoubleComplex,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZhemm_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
        B: *const cuda_types::cublas::cuDoubleComplex,
        ldb: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        C: *mut cuda_types::cublas::cuDoubleComplex,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasStrsm_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::core::ffi::c_int,
        B: *mut f32,
        ldb: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasStrsm_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const f32,
        A: *const f32,
        lda: cuda_types::cublas::i64,
        B: *mut f32,
        ldb: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDtrsm_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f64,
        A: *const f64,
        lda: ::core::ffi::c_int,
        B: *mut f64,
        ldb: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDtrsm_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const f64,
        A: *const f64,
        lda: cuda_types::cublas::i64,
        B: *mut f64,
        ldb: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCtrsm_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        B: *mut cuda_types::cublas::cuComplex,
        ldb: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCtrsm_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
        B: *mut cuda_types::cublas::cuComplex,
        ldb: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZtrsm_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        B: *mut cuda_types::cublas::cuDoubleComplex,
        ldb: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZtrsm_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
        B: *mut cuda_types::cublas::cuDoubleComplex,
        ldb: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasStrmm_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::core::ffi::c_int,
        B: *const f32,
        ldb: ::core::ffi::c_int,
        C: *mut f32,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasStrmm_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const f32,
        A: *const f32,
        lda: cuda_types::cublas::i64,
        B: *const f32,
        ldb: cuda_types::cublas::i64,
        C: *mut f32,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDtrmm_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f64,
        A: *const f64,
        lda: ::core::ffi::c_int,
        B: *const f64,
        ldb: ::core::ffi::c_int,
        C: *mut f64,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDtrmm_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const f64,
        A: *const f64,
        lda: cuda_types::cublas::i64,
        B: *const f64,
        ldb: cuda_types::cublas::i64,
        C: *mut f64,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCtrmm_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        B: *const cuda_types::cublas::cuComplex,
        ldb: ::core::ffi::c_int,
        C: *mut cuda_types::cublas::cuComplex,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCtrmm_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
        B: *const cuda_types::cublas::cuComplex,
        ldb: cuda_types::cublas::i64,
        C: *mut cuda_types::cublas::cuComplex,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZtrmm_v2(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        B: *const cuda_types::cublas::cuDoubleComplex,
        ldb: ::core::ffi::c_int,
        C: *mut cuda_types::cublas::cuDoubleComplex,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZtrmm_v2_64(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
        B: *const cuda_types::cublas::cuDoubleComplex,
        ldb: cuda_types::cublas::i64,
        C: *mut cuda_types::cublas::cuDoubleComplex,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasHgemmBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::__half,
        Aarray: *const *const cuda_types::cublas::__half,
        lda: ::core::ffi::c_int,
        Barray: *const *const cuda_types::cublas::__half,
        ldb: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::__half,
        Carray: *const *mut cuda_types::cublas::__half,
        ldc: ::core::ffi::c_int,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasHgemmBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::__half,
        Aarray: *const *const cuda_types::cublas::__half,
        lda: cuda_types::cublas::i64,
        Barray: *const *const cuda_types::cublas::__half,
        ldb: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::__half,
        Carray: *const *mut cuda_types::cublas::__half,
        ldc: cuda_types::cublas::i64,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSgemmBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const f32,
        Aarray: *const *const f32,
        lda: ::core::ffi::c_int,
        Barray: *const *const f32,
        ldb: ::core::ffi::c_int,
        beta: *const f32,
        Carray: *const *mut f32,
        ldc: ::core::ffi::c_int,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSgemmBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const f32,
        Aarray: *const *const f32,
        lda: cuda_types::cublas::i64,
        Barray: *const *const f32,
        ldb: cuda_types::cublas::i64,
        beta: *const f32,
        Carray: *const *mut f32,
        ldc: cuda_types::cublas::i64,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDgemmBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const f64,
        Aarray: *const *const f64,
        lda: ::core::ffi::c_int,
        Barray: *const *const f64,
        ldb: ::core::ffi::c_int,
        beta: *const f64,
        Carray: *const *mut f64,
        ldc: ::core::ffi::c_int,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDgemmBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const f64,
        Aarray: *const *const f64,
        lda: cuda_types::cublas::i64,
        Barray: *const *const f64,
        ldb: cuda_types::cublas::i64,
        beta: *const f64,
        Carray: *const *mut f64,
        ldc: cuda_types::cublas::i64,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgemmBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        Aarray: *const *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        Barray: *const *const cuda_types::cublas::cuComplex,
        ldb: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuComplex,
        Carray: *const *mut cuda_types::cublas::cuComplex,
        ldc: ::core::ffi::c_int,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgemmBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        Aarray: *const *const cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
        Barray: *const *const cuda_types::cublas::cuComplex,
        ldb: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuComplex,
        Carray: *const *mut cuda_types::cublas::cuComplex,
        ldc: cuda_types::cublas::i64,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgemm3mBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        Aarray: *const *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        Barray: *const *const cuda_types::cublas::cuComplex,
        ldb: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuComplex,
        Carray: *const *mut cuda_types::cublas::cuComplex,
        ldc: ::core::ffi::c_int,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgemm3mBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        Aarray: *const *const cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
        Barray: *const *const cuda_types::cublas::cuComplex,
        ldb: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuComplex,
        Carray: *const *mut cuda_types::cublas::cuComplex,
        ldc: cuda_types::cublas::i64,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZgemmBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        Aarray: *const *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        Barray: *const *const cuda_types::cublas::cuDoubleComplex,
        ldb: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        Carray: *const *mut cuda_types::cublas::cuDoubleComplex,
        ldc: ::core::ffi::c_int,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZgemmBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        Aarray: *const *const cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
        Barray: *const *const cuda_types::cublas::cuDoubleComplex,
        ldb: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        Carray: *const *mut cuda_types::cublas::cuDoubleComplex,
        ldc: cuda_types::cublas::i64,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasHgemmStridedBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::__half,
        A: *const cuda_types::cublas::__half,
        lda: ::core::ffi::c_int,
        strideA: ::core::ffi::c_longlong,
        B: *const cuda_types::cublas::__half,
        ldb: ::core::ffi::c_int,
        strideB: ::core::ffi::c_longlong,
        beta: *const cuda_types::cublas::__half,
        C: *mut cuda_types::cublas::__half,
        ldc: ::core::ffi::c_int,
        strideC: ::core::ffi::c_longlong,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasHgemmStridedBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::__half,
        A: *const cuda_types::cublas::__half,
        lda: cuda_types::cublas::i64,
        strideA: ::core::ffi::c_longlong,
        B: *const cuda_types::cublas::__half,
        ldb: cuda_types::cublas::i64,
        strideB: ::core::ffi::c_longlong,
        beta: *const cuda_types::cublas::__half,
        C: *mut cuda_types::cublas::__half,
        ldc: cuda_types::cublas::i64,
        strideC: ::core::ffi::c_longlong,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSgemmStridedBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::core::ffi::c_int,
        strideA: ::core::ffi::c_longlong,
        B: *const f32,
        ldb: ::core::ffi::c_int,
        strideB: ::core::ffi::c_longlong,
        beta: *const f32,
        C: *mut f32,
        ldc: ::core::ffi::c_int,
        strideC: ::core::ffi::c_longlong,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSgemmStridedBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const f32,
        A: *const f32,
        lda: cuda_types::cublas::i64,
        strideA: ::core::ffi::c_longlong,
        B: *const f32,
        ldb: cuda_types::cublas::i64,
        strideB: ::core::ffi::c_longlong,
        beta: *const f32,
        C: *mut f32,
        ldc: cuda_types::cublas::i64,
        strideC: ::core::ffi::c_longlong,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDgemmStridedBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const f64,
        A: *const f64,
        lda: ::core::ffi::c_int,
        strideA: ::core::ffi::c_longlong,
        B: *const f64,
        ldb: ::core::ffi::c_int,
        strideB: ::core::ffi::c_longlong,
        beta: *const f64,
        C: *mut f64,
        ldc: ::core::ffi::c_int,
        strideC: ::core::ffi::c_longlong,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDgemmStridedBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const f64,
        A: *const f64,
        lda: cuda_types::cublas::i64,
        strideA: ::core::ffi::c_longlong,
        B: *const f64,
        ldb: cuda_types::cublas::i64,
        strideB: ::core::ffi::c_longlong,
        beta: *const f64,
        C: *mut f64,
        ldc: cuda_types::cublas::i64,
        strideC: ::core::ffi::c_longlong,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgemmStridedBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        strideA: ::core::ffi::c_longlong,
        B: *const cuda_types::cublas::cuComplex,
        ldb: ::core::ffi::c_int,
        strideB: ::core::ffi::c_longlong,
        beta: *const cuda_types::cublas::cuComplex,
        C: *mut cuda_types::cublas::cuComplex,
        ldc: ::core::ffi::c_int,
        strideC: ::core::ffi::c_longlong,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgemmStridedBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
        strideA: ::core::ffi::c_longlong,
        B: *const cuda_types::cublas::cuComplex,
        ldb: cuda_types::cublas::i64,
        strideB: ::core::ffi::c_longlong,
        beta: *const cuda_types::cublas::cuComplex,
        C: *mut cuda_types::cublas::cuComplex,
        ldc: cuda_types::cublas::i64,
        strideC: ::core::ffi::c_longlong,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgemm3mStridedBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        strideA: ::core::ffi::c_longlong,
        B: *const cuda_types::cublas::cuComplex,
        ldb: ::core::ffi::c_int,
        strideB: ::core::ffi::c_longlong,
        beta: *const cuda_types::cublas::cuComplex,
        C: *mut cuda_types::cublas::cuComplex,
        ldc: ::core::ffi::c_int,
        strideC: ::core::ffi::c_longlong,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgemm3mStridedBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
        strideA: ::core::ffi::c_longlong,
        B: *const cuda_types::cublas::cuComplex,
        ldb: cuda_types::cublas::i64,
        strideB: ::core::ffi::c_longlong,
        beta: *const cuda_types::cublas::cuComplex,
        C: *mut cuda_types::cublas::cuComplex,
        ldc: cuda_types::cublas::i64,
        strideC: ::core::ffi::c_longlong,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZgemmStridedBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        strideA: ::core::ffi::c_longlong,
        B: *const cuda_types::cublas::cuDoubleComplex,
        ldb: ::core::ffi::c_int,
        strideB: ::core::ffi::c_longlong,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        C: *mut cuda_types::cublas::cuDoubleComplex,
        ldc: ::core::ffi::c_int,
        strideC: ::core::ffi::c_longlong,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZgemmStridedBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
        strideA: ::core::ffi::c_longlong,
        B: *const cuda_types::cublas::cuDoubleComplex,
        ldb: cuda_types::cublas::i64,
        strideB: ::core::ffi::c_longlong,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        C: *mut cuda_types::cublas::cuDoubleComplex,
        ldc: cuda_types::cublas::i64,
        strideC: ::core::ffi::c_longlong,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasGemmBatchedEx(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const ::core::ffi::c_void,
        Aarray: *const *const ::core::ffi::c_void,
        Atype: cuda_types::cublas::cudaDataType,
        lda: ::core::ffi::c_int,
        Barray: *const *const ::core::ffi::c_void,
        Btype: cuda_types::cublas::cudaDataType,
        ldb: ::core::ffi::c_int,
        beta: *const ::core::ffi::c_void,
        Carray: *const *mut ::core::ffi::c_void,
        Ctype: cuda_types::cublas::cudaDataType,
        ldc: ::core::ffi::c_int,
        batchCount: ::core::ffi::c_int,
        computeType: cuda_types::cublas::cublasComputeType_t,
        algo: cuda_types::cublas::cublasGemmAlgo_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasGemmBatchedEx_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const ::core::ffi::c_void,
        Aarray: *const *const ::core::ffi::c_void,
        Atype: cuda_types::cublas::cudaDataType,
        lda: cuda_types::cublas::i64,
        Barray: *const *const ::core::ffi::c_void,
        Btype: cuda_types::cublas::cudaDataType,
        ldb: cuda_types::cublas::i64,
        beta: *const ::core::ffi::c_void,
        Carray: *const *mut ::core::ffi::c_void,
        Ctype: cuda_types::cublas::cudaDataType,
        ldc: cuda_types::cublas::i64,
        batchCount: cuda_types::cublas::i64,
        computeType: cuda_types::cublas::cublasComputeType_t,
        algo: cuda_types::cublas::cublasGemmAlgo_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasGemmStridedBatchedEx(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        Atype: cuda_types::cublas::cudaDataType,
        lda: ::core::ffi::c_int,
        strideA: ::core::ffi::c_longlong,
        B: *const ::core::ffi::c_void,
        Btype: cuda_types::cublas::cudaDataType,
        ldb: ::core::ffi::c_int,
        strideB: ::core::ffi::c_longlong,
        beta: *const ::core::ffi::c_void,
        C: *mut ::core::ffi::c_void,
        Ctype: cuda_types::cublas::cudaDataType,
        ldc: ::core::ffi::c_int,
        strideC: ::core::ffi::c_longlong,
        batchCount: ::core::ffi::c_int,
        computeType: cuda_types::cublas::cublasComputeType_t,
        algo: cuda_types::cublas::cublasGemmAlgo_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasGemmStridedBatchedEx_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        k: cuda_types::cublas::i64,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        Atype: cuda_types::cublas::cudaDataType,
        lda: cuda_types::cublas::i64,
        strideA: ::core::ffi::c_longlong,
        B: *const ::core::ffi::c_void,
        Btype: cuda_types::cublas::cudaDataType,
        ldb: cuda_types::cublas::i64,
        strideB: ::core::ffi::c_longlong,
        beta: *const ::core::ffi::c_void,
        C: *mut ::core::ffi::c_void,
        Ctype: cuda_types::cublas::cudaDataType,
        ldc: cuda_types::cublas::i64,
        strideC: ::core::ffi::c_longlong,
        batchCount: cuda_types::cublas::i64,
        computeType: cuda_types::cublas::cublasComputeType_t,
        algo: cuda_types::cublas::cublasGemmAlgo_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSgemmGroupedBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        transa_array: *const cuda_types::cublas::cublasOperation_t,
        transb_array: *const cuda_types::cublas::cublasOperation_t,
        m_array: *const ::core::ffi::c_int,
        n_array: *const ::core::ffi::c_int,
        k_array: *const ::core::ffi::c_int,
        alpha_array: *const f32,
        Aarray: *const *const f32,
        lda_array: *const ::core::ffi::c_int,
        Barray: *const *const f32,
        ldb_array: *const ::core::ffi::c_int,
        beta_array: *const f32,
        Carray: *const *mut f32,
        ldc_array: *const ::core::ffi::c_int,
        group_count: ::core::ffi::c_int,
        group_size: *const ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSgemmGroupedBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa_array: *const cuda_types::cublas::cublasOperation_t,
        transb_array: *const cuda_types::cublas::cublasOperation_t,
        m_array: *const cuda_types::cublas::i64,
        n_array: *const cuda_types::cublas::i64,
        k_array: *const cuda_types::cublas::i64,
        alpha_array: *const f32,
        Aarray: *const *const f32,
        lda_array: *const cuda_types::cublas::i64,
        Barray: *const *const f32,
        ldb_array: *const cuda_types::cublas::i64,
        beta_array: *const f32,
        Carray: *const *mut f32,
        ldc_array: *const cuda_types::cublas::i64,
        group_count: cuda_types::cublas::i64,
        group_size: *const cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDgemmGroupedBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        transa_array: *const cuda_types::cublas::cublasOperation_t,
        transb_array: *const cuda_types::cublas::cublasOperation_t,
        m_array: *const ::core::ffi::c_int,
        n_array: *const ::core::ffi::c_int,
        k_array: *const ::core::ffi::c_int,
        alpha_array: *const f64,
        Aarray: *const *const f64,
        lda_array: *const ::core::ffi::c_int,
        Barray: *const *const f64,
        ldb_array: *const ::core::ffi::c_int,
        beta_array: *const f64,
        Carray: *const *mut f64,
        ldc_array: *const ::core::ffi::c_int,
        group_count: ::core::ffi::c_int,
        group_size: *const ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDgemmGroupedBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa_array: *const cuda_types::cublas::cublasOperation_t,
        transb_array: *const cuda_types::cublas::cublasOperation_t,
        m_array: *const cuda_types::cublas::i64,
        n_array: *const cuda_types::cublas::i64,
        k_array: *const cuda_types::cublas::i64,
        alpha_array: *const f64,
        Aarray: *const *const f64,
        lda_array: *const cuda_types::cublas::i64,
        Barray: *const *const f64,
        ldb_array: *const cuda_types::cublas::i64,
        beta_array: *const f64,
        Carray: *const *mut f64,
        ldc_array: *const cuda_types::cublas::i64,
        group_count: cuda_types::cublas::i64,
        group_size: *const cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasGemmGroupedBatchedEx(
        handle: cuda_types::cublas::cublasHandle_t,
        transa_array: *const cuda_types::cublas::cublasOperation_t,
        transb_array: *const cuda_types::cublas::cublasOperation_t,
        m_array: *const ::core::ffi::c_int,
        n_array: *const ::core::ffi::c_int,
        k_array: *const ::core::ffi::c_int,
        alpha_array: *const ::core::ffi::c_void,
        Aarray: *const *const ::core::ffi::c_void,
        Atype: cuda_types::cublas::cudaDataType_t,
        lda_array: *const ::core::ffi::c_int,
        Barray: *const *const ::core::ffi::c_void,
        Btype: cuda_types::cublas::cudaDataType_t,
        ldb_array: *const ::core::ffi::c_int,
        beta_array: *const ::core::ffi::c_void,
        Carray: *const *mut ::core::ffi::c_void,
        Ctype: cuda_types::cublas::cudaDataType_t,
        ldc_array: *const ::core::ffi::c_int,
        group_count: ::core::ffi::c_int,
        group_size: *const ::core::ffi::c_int,
        computeType: cuda_types::cublas::cublasComputeType_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasGemmGroupedBatchedEx_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa_array: *const cuda_types::cublas::cublasOperation_t,
        transb_array: *const cuda_types::cublas::cublasOperation_t,
        m_array: *const cuda_types::cublas::i64,
        n_array: *const cuda_types::cublas::i64,
        k_array: *const cuda_types::cublas::i64,
        alpha_array: *const ::core::ffi::c_void,
        Aarray: *const *const ::core::ffi::c_void,
        Atype: cuda_types::cublas::cudaDataType_t,
        lda_array: *const cuda_types::cublas::i64,
        Barray: *const *const ::core::ffi::c_void,
        Btype: cuda_types::cublas::cudaDataType_t,
        ldb_array: *const cuda_types::cublas::i64,
        beta_array: *const ::core::ffi::c_void,
        Carray: *const *mut ::core::ffi::c_void,
        Ctype: cuda_types::cublas::cudaDataType_t,
        ldc_array: *const cuda_types::cublas::i64,
        group_count: cuda_types::cublas::i64,
        group_size: *const cuda_types::cublas::i64,
        computeType: cuda_types::cublas::cublasComputeType_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSgeam(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        A: *const f32,
        lda: ::core::ffi::c_int,
        beta: *const f32,
        B: *const f32,
        ldb: ::core::ffi::c_int,
        C: *mut f32,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSgeam_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const f32,
        A: *const f32,
        lda: cuda_types::cublas::i64,
        beta: *const f32,
        B: *const f32,
        ldb: cuda_types::cublas::i64,
        C: *mut f32,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDgeam(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f64,
        A: *const f64,
        lda: ::core::ffi::c_int,
        beta: *const f64,
        B: *const f64,
        ldb: ::core::ffi::c_int,
        C: *mut f64,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDgeam_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const f64,
        A: *const f64,
        lda: cuda_types::cublas::i64,
        beta: *const f64,
        B: *const f64,
        ldb: cuda_types::cublas::i64,
        C: *mut f64,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgeam(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuComplex,
        B: *const cuda_types::cublas::cuComplex,
        ldb: ::core::ffi::c_int,
        C: *mut cuda_types::cublas::cuComplex,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgeam_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuComplex,
        B: *const cuda_types::cublas::cuComplex,
        ldb: cuda_types::cublas::i64,
        C: *mut cuda_types::cublas::cuComplex,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZgeam(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        B: *const cuda_types::cublas::cuDoubleComplex,
        ldb: ::core::ffi::c_int,
        C: *mut cuda_types::cublas::cuDoubleComplex,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZgeam_64(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
        beta: *const cuda_types::cublas::cuDoubleComplex,
        B: *const cuda_types::cublas::cuDoubleComplex,
        ldb: cuda_types::cublas::i64,
        C: *mut cuda_types::cublas::cuDoubleComplex,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasStrsmBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f32,
        A: *const *const f32,
        lda: ::core::ffi::c_int,
        B: *const *mut f32,
        ldb: ::core::ffi::c_int,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasStrsmBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const f32,
        A: *const *const f32,
        lda: cuda_types::cublas::i64,
        B: *const *mut f32,
        ldb: cuda_types::cublas::i64,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDtrsmBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const f64,
        A: *const *const f64,
        lda: ::core::ffi::c_int,
        B: *const *mut f64,
        ldb: ::core::ffi::c_int,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDtrsmBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const f64,
        A: *const *const f64,
        lda: cuda_types::cublas::i64,
        B: *const *mut f64,
        ldb: cuda_types::cublas::i64,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCtrsmBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        B: *const *mut cuda_types::cublas::cuComplex,
        ldb: ::core::ffi::c_int,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCtrsmBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuComplex,
        A: *const *const cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
        B: *const *mut cuda_types::cublas::cuComplex,
        ldb: cuda_types::cublas::i64,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZtrsmBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        B: *const *mut cuda_types::cublas::cuDoubleComplex,
        ldb: ::core::ffi::c_int,
        batchCount: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZtrsmBatched_64(
        handle: cuda_types::cublas::cublasHandle_t,
        side: cuda_types::cublas::cublasSideMode_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        trans: cuda_types::cublas::cublasOperation_t,
        diag: cuda_types::cublas::cublasDiagType_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        alpha: *const cuda_types::cublas::cuDoubleComplex,
        A: *const *const cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
        B: *const *mut cuda_types::cublas::cuDoubleComplex,
        ldb: cuda_types::cublas::i64,
        batchCount: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSdgmm(
        handle: cuda_types::cublas::cublasHandle_t,
        mode: cuda_types::cublas::cublasSideMode_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const f32,
        lda: ::core::ffi::c_int,
        x: *const f32,
        incx: ::core::ffi::c_int,
        C: *mut f32,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSdgmm_64(
        handle: cuda_types::cublas::cublasHandle_t,
        mode: cuda_types::cublas::cublasSideMode_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        A: *const f32,
        lda: cuda_types::cublas::i64,
        x: *const f32,
        incx: cuda_types::cublas::i64,
        C: *mut f32,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDdgmm(
        handle: cuda_types::cublas::cublasHandle_t,
        mode: cuda_types::cublas::cublasSideMode_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const f64,
        lda: ::core::ffi::c_int,
        x: *const f64,
        incx: ::core::ffi::c_int,
        C: *mut f64,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDdgmm_64(
        handle: cuda_types::cublas::cublasHandle_t,
        mode: cuda_types::cublas::cublasSideMode_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        A: *const f64,
        lda: cuda_types::cublas::i64,
        x: *const f64,
        incx: cuda_types::cublas::i64,
        C: *mut f64,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCdgmm(
        handle: cuda_types::cublas::cublasHandle_t,
        mode: cuda_types::cublas::cublasSideMode_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        x: *const cuda_types::cublas::cuComplex,
        incx: ::core::ffi::c_int,
        C: *mut cuda_types::cublas::cuComplex,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCdgmm_64(
        handle: cuda_types::cublas::cublasHandle_t,
        mode: cuda_types::cublas::cublasSideMode_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        A: *const cuda_types::cublas::cuComplex,
        lda: cuda_types::cublas::i64,
        x: *const cuda_types::cublas::cuComplex,
        incx: cuda_types::cublas::i64,
        C: *mut cuda_types::cublas::cuComplex,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZdgmm(
        handle: cuda_types::cublas::cublasHandle_t,
        mode: cuda_types::cublas::cublasSideMode_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: ::core::ffi::c_int,
        C: *mut cuda_types::cublas::cuDoubleComplex,
        ldc: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZdgmm_64(
        handle: cuda_types::cublas::cublasHandle_t,
        mode: cuda_types::cublas::cublasSideMode_t,
        m: cuda_types::cublas::i64,
        n: cuda_types::cublas::i64,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: cuda_types::cublas::i64,
        x: *const cuda_types::cublas::cuDoubleComplex,
        incx: cuda_types::cublas::i64,
        C: *mut cuda_types::cublas::cuDoubleComplex,
        ldc: cuda_types::cublas::i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSmatinvBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        A: *const *const f32,
        lda: ::core::ffi::c_int,
        Ainv: *const *mut f32,
        lda_inv: ::core::ffi::c_int,
        info: *mut ::core::ffi::c_int,
        batchSize: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDmatinvBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        A: *const *const f64,
        lda: ::core::ffi::c_int,
        Ainv: *const *mut f64,
        lda_inv: ::core::ffi::c_int,
        info: *mut ::core::ffi::c_int,
        batchSize: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCmatinvBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        A: *const *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        Ainv: *const *mut cuda_types::cublas::cuComplex,
        lda_inv: ::core::ffi::c_int,
        info: *mut ::core::ffi::c_int,
        batchSize: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZmatinvBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        A: *const *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        Ainv: *const *mut cuda_types::cublas::cuDoubleComplex,
        lda_inv: ::core::ffi::c_int,
        info: *mut ::core::ffi::c_int,
        batchSize: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSgeqrfBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        Aarray: *const *mut f32,
        lda: ::core::ffi::c_int,
        TauArray: *const *mut f32,
        info: *mut ::core::ffi::c_int,
        batchSize: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDgeqrfBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        Aarray: *const *mut f64,
        lda: ::core::ffi::c_int,
        TauArray: *const *mut f64,
        info: *mut ::core::ffi::c_int,
        batchSize: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgeqrfBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        Aarray: *const *mut cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        TauArray: *const *mut cuda_types::cublas::cuComplex,
        info: *mut ::core::ffi::c_int,
        batchSize: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZgeqrfBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        Aarray: *const *mut cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        TauArray: *const *mut cuda_types::cublas::cuDoubleComplex,
        info: *mut ::core::ffi::c_int,
        batchSize: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSgelsBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        Aarray: *const *mut f32,
        lda: ::core::ffi::c_int,
        Carray: *const *mut f32,
        ldc: ::core::ffi::c_int,
        info: *mut ::core::ffi::c_int,
        devInfoArray: *mut ::core::ffi::c_int,
        batchSize: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDgelsBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        Aarray: *const *mut f64,
        lda: ::core::ffi::c_int,
        Carray: *const *mut f64,
        ldc: ::core::ffi::c_int,
        info: *mut ::core::ffi::c_int,
        devInfoArray: *mut ::core::ffi::c_int,
        batchSize: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgelsBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        Aarray: *const *mut cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        Carray: *const *mut cuda_types::cublas::cuComplex,
        ldc: ::core::ffi::c_int,
        info: *mut ::core::ffi::c_int,
        devInfoArray: *mut ::core::ffi::c_int,
        batchSize: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZgelsBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        Aarray: *const *mut cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        Carray: *const *mut cuda_types::cublas::cuDoubleComplex,
        ldc: ::core::ffi::c_int,
        info: *mut ::core::ffi::c_int,
        devInfoArray: *mut ::core::ffi::c_int,
        batchSize: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasStpttr(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        AP: *const f32,
        A: *mut f32,
        lda: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDtpttr(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        AP: *const f64,
        A: *mut f64,
        lda: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCtpttr(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        AP: *const cuda_types::cublas::cuComplex,
        A: *mut cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZtpttr(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        AP: *const cuda_types::cublas::cuDoubleComplex,
        A: *mut cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasStrttp(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        A: *const f32,
        lda: ::core::ffi::c_int,
        AP: *mut f32,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDtrttp(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        A: *const f64,
        lda: ::core::ffi::c_int,
        AP: *mut f64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCtrttp(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        A: *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        AP: *mut cuda_types::cublas::cuComplex,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZtrttp(
        handle: cuda_types::cublas::cublasHandle_t,
        uplo: cuda_types::cublas::cublasFillMode_t,
        n: ::core::ffi::c_int,
        A: *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        AP: *mut cuda_types::cublas::cuDoubleComplex,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSgetrfBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        A: *const *mut f32,
        lda: ::core::ffi::c_int,
        P: *mut ::core::ffi::c_int,
        info: *mut ::core::ffi::c_int,
        batchSize: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDgetrfBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        A: *const *mut f64,
        lda: ::core::ffi::c_int,
        P: *mut ::core::ffi::c_int,
        info: *mut ::core::ffi::c_int,
        batchSize: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgetrfBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        A: *const *mut cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        P: *mut ::core::ffi::c_int,
        info: *mut ::core::ffi::c_int,
        batchSize: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZgetrfBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        A: *const *mut cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        P: *mut ::core::ffi::c_int,
        info: *mut ::core::ffi::c_int,
        batchSize: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSgetriBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        A: *const *const f32,
        lda: ::core::ffi::c_int,
        P: *const ::core::ffi::c_int,
        C: *const *mut f32,
        ldc: ::core::ffi::c_int,
        info: *mut ::core::ffi::c_int,
        batchSize: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDgetriBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        A: *const *const f64,
        lda: ::core::ffi::c_int,
        P: *const ::core::ffi::c_int,
        C: *const *mut f64,
        ldc: ::core::ffi::c_int,
        info: *mut ::core::ffi::c_int,
        batchSize: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgetriBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        A: *const *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        P: *const ::core::ffi::c_int,
        C: *const *mut cuda_types::cublas::cuComplex,
        ldc: ::core::ffi::c_int,
        info: *mut ::core::ffi::c_int,
        batchSize: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZgetriBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        n: ::core::ffi::c_int,
        A: *const *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        P: *const ::core::ffi::c_int,
        C: *const *mut cuda_types::cublas::cuDoubleComplex,
        ldc: ::core::ffi::c_int,
        info: *mut ::core::ffi::c_int,
        batchSize: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasSgetrsBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        Aarray: *const *const f32,
        lda: ::core::ffi::c_int,
        devIpiv: *const ::core::ffi::c_int,
        Barray: *const *mut f32,
        ldb: ::core::ffi::c_int,
        info: *mut ::core::ffi::c_int,
        batchSize: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasDgetrsBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        Aarray: *const *const f64,
        lda: ::core::ffi::c_int,
        devIpiv: *const ::core::ffi::c_int,
        Barray: *const *mut f64,
        ldb: ::core::ffi::c_int,
        info: *mut ::core::ffi::c_int,
        batchSize: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasCgetrsBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        Aarray: *const *const cuda_types::cublas::cuComplex,
        lda: ::core::ffi::c_int,
        devIpiv: *const ::core::ffi::c_int,
        Barray: *const *mut cuda_types::cublas::cuComplex,
        ldb: ::core::ffi::c_int,
        info: *mut ::core::ffi::c_int,
        batchSize: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasZgetrsBatched(
        handle: cuda_types::cublas::cublasHandle_t,
        trans: cuda_types::cublas::cublasOperation_t,
        n: ::core::ffi::c_int,
        nrhs: ::core::ffi::c_int,
        Aarray: *const *const cuda_types::cublas::cuDoubleComplex,
        lda: ::core::ffi::c_int,
        devIpiv: *const ::core::ffi::c_int,
        Barray: *const *mut cuda_types::cublas::cuDoubleComplex,
        ldb: ::core::ffi::c_int,
        info: *mut ::core::ffi::c_int,
        batchSize: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasUint8gemmBias(
        handle: cuda_types::cublas::cublasHandle_t,
        transa: cuda_types::cublas::cublasOperation_t,
        transb: cuda_types::cublas::cublasOperation_t,
        transc: cuda_types::cublas::cublasOperation_t,
        m: ::core::ffi::c_int,
        n: ::core::ffi::c_int,
        k: ::core::ffi::c_int,
        A: *const ::core::ffi::c_uchar,
        A_bias: ::core::ffi::c_int,
        lda: ::core::ffi::c_int,
        B: *const ::core::ffi::c_uchar,
        B_bias: ::core::ffi::c_int,
        ldb: ::core::ffi::c_int,
        C: *mut ::core::ffi::c_uchar,
        C_bias: ::core::ffi::c_int,
        ldc: ::core::ffi::c_int,
        C_mult: ::core::ffi::c_int,
        C_shift: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
}
