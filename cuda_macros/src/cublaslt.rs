// Generated automatically by zluda_bindgen
// DO NOT EDIT MANUALLY
#![allow(warnings)]
extern "system" {
    #[must_use]
    fn cublasLtCreate(
        lightHandle: *mut cuda_types::cublaslt::cublasLtHandle_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasLtDestroy(
        lightHandle: cuda_types::cublaslt::cublasLtHandle_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    fn cublasLtGetStatusName(
        status: cuda_types::cublas::cublasStatus_t,
    ) -> *const ::core::ffi::c_char;
    fn cublasLtGetStatusString(
        status: cuda_types::cublas::cublasStatus_t,
    ) -> *const ::core::ffi::c_char;
    fn cublasLtGetVersion() -> usize;
    fn cublasLtGetCudartVersion() -> usize;
    #[must_use]
    fn cublasLtGetProperty(
        type_: cuda_types::cublaslt::libraryPropertyType,
        value: *mut ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasLtHeuristicsCacheGetCapacity(
        capacity: *mut usize,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    fn cublasLtHeuristicsCacheSetCapacity(
        capacity: usize,
    ) -> cuda_types::cublas::cublasStatus_t;
    /** Restricts usage of CPU instructions (ISA) specified by the flags in the mask.

 Flags can be combined with bitwise OR(|) operator. Supported flags:
 - 0x1 -- x86-64 AVX512 ISA

 Default mask: 0 (any applicable ISA is allowed).

 The function returns the previous value of the mask.
 The function takes precedence over the environment variable CUBLASLT_DISABLE_CPU_INSTRUCTIONS_MASK.*/
    fn cublasLtDisableCpuInstructionsSetMask(
        mask: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_uint;
    #[must_use]
    /** Execute matrix multiplication (D = alpha * op(A) * op(B) + beta * C).

 \retval     CUBLAS_STATUS_NOT_INITIALIZED   if cuBLASLt handle has not been initialized
 \retval     CUBLAS_STATUS_INVALID_VALUE     if parameters are in conflict or in an impossible configuration; e.g.
                                             when workspaceSizeInBytes is less than workspace required by configured
                                             algo
 \retval     CUBLAS_STATUS_NOT_SUPPORTED     if current implementation on selected device doesn't support configured
                                             operation
 \retval     CUBLAS_STATUS_ARCH_MISMATCH     if configured operation cannot be run using selected device
 \retval     CUBLAS_STATUS_EXECUTION_FAILED  if cuda reported execution error from the device
 \retval     CUBLAS_STATUS_SUCCESS           if the operation completed successfully*/
    fn cublasLtMatmul(
        lightHandle: cuda_types::cublaslt::cublasLtHandle_t,
        computeDesc: cuda_types::cublaslt::cublasLtMatmulDesc_t,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        Adesc: cuda_types::cublaslt::cublasLtMatrixLayout_t,
        B: *const ::core::ffi::c_void,
        Bdesc: cuda_types::cublaslt::cublasLtMatrixLayout_t,
        beta: *const ::core::ffi::c_void,
        C: *const ::core::ffi::c_void,
        Cdesc: cuda_types::cublaslt::cublasLtMatrixLayout_t,
        D: *mut ::core::ffi::c_void,
        Ddesc: cuda_types::cublaslt::cublasLtMatrixLayout_t,
        algo: *const cuda_types::cublaslt::cublasLtMatmulAlgo_t,
        workspace: *mut ::core::ffi::c_void,
        workspaceSizeInBytes: usize,
        stream: cuda_types::cublaslt::cudaStream_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /** Matrix layout conversion helper (C = alpha * op(A) + beta * op(B))

 Can be used to change memory order of data or to scale and shift the values.

 \retval     CUBLAS_STATUS_NOT_INITIALIZED   if cuBLASLt handle has not been initialized
 \retval     CUBLAS_STATUS_INVALID_VALUE     if parameters are in conflict or in an impossible configuration; e.g.
                                             when A is not NULL, but Adesc is NULL
 \retval     CUBLAS_STATUS_NOT_SUPPORTED     if current implementation on selected device doesn't support configured
                                             operation
 \retval     CUBLAS_STATUS_ARCH_MISMATCH     if configured operation cannot be run using selected device
 \retval     CUBLAS_STATUS_EXECUTION_FAILED  if cuda reported execution error from the device
 \retval     CUBLAS_STATUS_SUCCESS           if the operation completed successfully*/
    fn cublasLtMatrixTransform(
        lightHandle: cuda_types::cublaslt::cublasLtHandle_t,
        transformDesc: cuda_types::cublaslt::cublasLtMatrixTransformDesc_t,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        Adesc: cuda_types::cublaslt::cublasLtMatrixLayout_t,
        beta: *const ::core::ffi::c_void,
        B: *const ::core::ffi::c_void,
        Bdesc: cuda_types::cublaslt::cublasLtMatrixLayout_t,
        C: *mut ::core::ffi::c_void,
        Cdesc: cuda_types::cublaslt::cublasLtMatrixLayout_t,
        stream: cuda_types::cublaslt::cudaStream_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /// Internal. Do not use directly.
    fn cublasLtMatrixLayoutInit_internal(
        matLayout: cuda_types::cublaslt::cublasLtMatrixLayout_t,
        size: usize,
        type_: cuda_types::cublaslt::cudaDataType,
        rows: u64,
        cols: u64,
        ld: i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /** Create new matrix layout descriptor.

 \retval     CUBLAS_STATUS_ALLOC_FAILED  if memory could not be allocated
 \retval     CUBLAS_STATUS_SUCCESS       if desciptor was created successfully*/
    fn cublasLtMatrixLayoutCreate(
        matLayout: *mut cuda_types::cublaslt::cublasLtMatrixLayout_t,
        type_: cuda_types::cublaslt::cudaDataType,
        rows: u64,
        cols: u64,
        ld: i64,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /** Destroy matrix layout descriptor.

 \retval     CUBLAS_STATUS_SUCCESS  if operation was successful*/
    fn cublasLtMatrixLayoutDestroy(
        matLayout: cuda_types::cublaslt::cublasLtMatrixLayout_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /** Set matrix layout descriptor attribute.

 \param[in]  matLayout    The descriptor
 \param[in]  attr         The attribute
 \param[in]  buf          memory address containing the new value
 \param[in]  sizeInBytes  size of buf buffer for verification (in bytes)

 \retval     CUBLAS_STATUS_INVALID_VALUE  if buf is NULL or sizeInBytes doesn't match size of internal storage for
                                          selected attribute
 \retval     CUBLAS_STATUS_SUCCESS        if attribute was set successfully*/
    fn cublasLtMatrixLayoutSetAttribute(
        matLayout: cuda_types::cublaslt::cublasLtMatrixLayout_t,
        attr: cuda_types::cublaslt::cublasLtMatrixLayoutAttribute_t,
        buf: *const ::core::ffi::c_void,
        sizeInBytes: usize,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /** Get matrix layout descriptor attribute.

 \param[in]  matLayout    The descriptor
 \param[in]  attr         The attribute
 \param[out] buf          memory address containing the new value
 \param[in]  sizeInBytes  size of buf buffer for verification (in bytes)
 \param[out] sizeWritten  only valid when return value is CUBLAS_STATUS_SUCCESS. If sizeInBytes is non-zero: number of
                          bytes actually written, if sizeInBytes is 0: number of bytes needed to write full contents

 \retval     CUBLAS_STATUS_INVALID_VALUE  if sizeInBytes is 0 and sizeWritten is NULL, or if  sizeInBytes is non-zero
                                          and buf is NULL or sizeInBytes doesn't match size of internal storage for
                                          selected attribute
 \retval     CUBLAS_STATUS_SUCCESS        if attribute's value was successfully written to user memory*/
    fn cublasLtMatrixLayoutGetAttribute(
        matLayout: cuda_types::cublaslt::cublasLtMatrixLayout_t,
        attr: cuda_types::cublaslt::cublasLtMatrixLayoutAttribute_t,
        buf: *mut ::core::ffi::c_void,
        sizeInBytes: usize,
        sizeWritten: *mut usize,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /// Internal. Do not use directly.
    fn cublasLtMatmulDescInit_internal(
        matmulDesc: cuda_types::cublaslt::cublasLtMatmulDesc_t,
        size: usize,
        computeType: cuda_types::cublas::cublasComputeType_t,
        scaleType: cuda_types::cublaslt::cudaDataType_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /** Create new matmul operation descriptor.

 \retval     CUBLAS_STATUS_ALLOC_FAILED  if memory could not be allocated
 \retval     CUBLAS_STATUS_SUCCESS       if desciptor was created successfully*/
    fn cublasLtMatmulDescCreate(
        matmulDesc: *mut cuda_types::cublaslt::cublasLtMatmulDesc_t,
        computeType: cuda_types::cublas::cublasComputeType_t,
        scaleType: cuda_types::cublaslt::cudaDataType_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /** Destroy matmul operation descriptor.

 \retval     CUBLAS_STATUS_SUCCESS  if operation was successful*/
    fn cublasLtMatmulDescDestroy(
        matmulDesc: cuda_types::cublaslt::cublasLtMatmulDesc_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /** Set matmul operation descriptor attribute.

 \param[in]  matmulDesc   The descriptor
 \param[in]  attr         The attribute
 \param[in]  buf          memory address containing the new value
 \param[in]  sizeInBytes  size of buf buffer for verification (in bytes)

 \retval     CUBLAS_STATUS_INVALID_VALUE  if buf is NULL or sizeInBytes doesn't match size of internal storage for
                                          selected attribute
 \retval     CUBLAS_STATUS_SUCCESS        if attribute was set successfully*/
    fn cublasLtMatmulDescSetAttribute(
        matmulDesc: cuda_types::cublaslt::cublasLtMatmulDesc_t,
        attr: cuda_types::cublaslt::cublasLtMatmulDescAttributes_t,
        buf: *const ::core::ffi::c_void,
        sizeInBytes: usize,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /** Get matmul operation descriptor attribute.

 \param[in]  matmulDesc   The descriptor
 \param[in]  attr         The attribute
 \param[out] buf          memory address containing the new value
 \param[in]  sizeInBytes  size of buf buffer for verification (in bytes)
 \param[out] sizeWritten  only valid when return value is CUBLAS_STATUS_SUCCESS. If sizeInBytes is non-zero: number of
                          bytes actually written, if sizeInBytes is 0: number of bytes needed to write full contents

 \retval     CUBLAS_STATUS_INVALID_VALUE  if sizeInBytes is 0 and sizeWritten is NULL, or if  sizeInBytes is non-zero
                                          and buf is NULL or sizeInBytes doesn't match size of internal storage for
                                          selected attribute
 \retval     CUBLAS_STATUS_SUCCESS        if attribute's value was successfully written to user memory*/
    fn cublasLtMatmulDescGetAttribute(
        matmulDesc: cuda_types::cublaslt::cublasLtMatmulDesc_t,
        attr: cuda_types::cublaslt::cublasLtMatmulDescAttributes_t,
        buf: *mut ::core::ffi::c_void,
        sizeInBytes: usize,
        sizeWritten: *mut usize,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /// Internal. Do not use directly.
    fn cublasLtMatrixTransformDescInit_internal(
        transformDesc: cuda_types::cublaslt::cublasLtMatrixTransformDesc_t,
        size: usize,
        scaleType: cuda_types::cublaslt::cudaDataType,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /** Create new matrix transform operation descriptor.

 \retval     CUBLAS_STATUS_ALLOC_FAILED  if memory could not be allocated
 \retval     CUBLAS_STATUS_SUCCESS       if desciptor was created successfully*/
    fn cublasLtMatrixTransformDescCreate(
        transformDesc: *mut cuda_types::cublaslt::cublasLtMatrixTransformDesc_t,
        scaleType: cuda_types::cublaslt::cudaDataType,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /** Destroy matrix transform operation descriptor.

 \retval     CUBLAS_STATUS_SUCCESS  if operation was successful*/
    fn cublasLtMatrixTransformDescDestroy(
        transformDesc: cuda_types::cublaslt::cublasLtMatrixTransformDesc_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /** Set matrix transform operation descriptor attribute.

 \param[in]  transformDesc  The descriptor
 \param[in]  attr           The attribute
 \param[in]  buf            memory address containing the new value
 \param[in]  sizeInBytes    size of buf buffer for verification (in bytes)

 \retval     CUBLAS_STATUS_INVALID_VALUE  if buf is NULL or sizeInBytes doesn't match size of internal storage for
                                          selected attribute
 \retval     CUBLAS_STATUS_SUCCESS        if attribute was set successfully*/
    fn cublasLtMatrixTransformDescSetAttribute(
        transformDesc: cuda_types::cublaslt::cublasLtMatrixTransformDesc_t,
        attr: cuda_types::cublaslt::cublasLtMatrixTransformDescAttributes_t,
        buf: *const ::core::ffi::c_void,
        sizeInBytes: usize,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /** Get matrix transform operation descriptor attribute.

 \param[in]  transformDesc  The descriptor
 \param[in]  attr           The attribute
 \param[out] buf            memory address containing the new value
 \param[in]  sizeInBytes    size of buf buffer for verification (in bytes)
 \param[out] sizeWritten    only valid when return value is CUBLAS_STATUS_SUCCESS. If sizeInBytes is non-zero: number
 of bytes actually written, if sizeInBytes is 0: number of bytes needed to write full contents

 \retval     CUBLAS_STATUS_INVALID_VALUE  if sizeInBytes is 0 and sizeWritten is NULL, or if  sizeInBytes is non-zero
                                          and buf is NULL or sizeInBytes doesn't match size of internal storage for
                                          selected attribute
 \retval     CUBLAS_STATUS_SUCCESS        if attribute's value was successfully written to user memory*/
    fn cublasLtMatrixTransformDescGetAttribute(
        transformDesc: cuda_types::cublaslt::cublasLtMatrixTransformDesc_t,
        attr: cuda_types::cublaslt::cublasLtMatrixTransformDescAttributes_t,
        buf: *mut ::core::ffi::c_void,
        sizeInBytes: usize,
        sizeWritten: *mut usize,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /// Internal. Do not use directly.
    fn cublasLtMatmulPreferenceInit_internal(
        pref: cuda_types::cublaslt::cublasLtMatmulPreference_t,
        size: usize,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /** Create new matmul heuristic search preference descriptor.

 \retval     CUBLAS_STATUS_ALLOC_FAILED  if memory could not be allocated
 \retval     CUBLAS_STATUS_SUCCESS       if desciptor was created successfully*/
    fn cublasLtMatmulPreferenceCreate(
        pref: *mut cuda_types::cublaslt::cublasLtMatmulPreference_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /** Destroy matmul heuristic search preference descriptor.

 \retval     CUBLAS_STATUS_SUCCESS  if operation was successful*/
    fn cublasLtMatmulPreferenceDestroy(
        pref: cuda_types::cublaslt::cublasLtMatmulPreference_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /** Set matmul heuristic search preference descriptor attribute.

 \param[in]  pref         The descriptor
 \param[in]  attr         The attribute
 \param[in]  buf          memory address containing the new value
 \param[in]  sizeInBytes  size of buf buffer for verification (in bytes)

 \retval     CUBLAS_STATUS_INVALID_VALUE  if buf is NULL or sizeInBytes doesn't match size of internal storage for
                                          selected attribute
 \retval     CUBLAS_STATUS_SUCCESS        if attribute was set successfully*/
    fn cublasLtMatmulPreferenceSetAttribute(
        pref: cuda_types::cublaslt::cublasLtMatmulPreference_t,
        attr: cuda_types::cublaslt::cublasLtMatmulPreferenceAttributes_t,
        buf: *const ::core::ffi::c_void,
        sizeInBytes: usize,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /** Get matmul heuristic search preference descriptor attribute.

 \param[in]  pref         The descriptor
 \param[in]  attr         The attribute
 \param[out] buf          memory address containing the new value
 \param[in]  sizeInBytes  size of buf buffer for verification (in bytes)
 \param[out] sizeWritten  only valid when return value is CUBLAS_STATUS_SUCCESS. If sizeInBytes is non-zero: number of
                          bytes actually written, if sizeInBytes is 0: number of bytes needed to write full contents

 \retval     CUBLAS_STATUS_INVALID_VALUE  if sizeInBytes is 0 and sizeWritten is NULL, or if  sizeInBytes is non-zero
                                          and buf is NULL or sizeInBytes doesn't match size of internal storage for
                                          selected attribute
 \retval     CUBLAS_STATUS_SUCCESS        if attribute's value was successfully written to user memory*/
    fn cublasLtMatmulPreferenceGetAttribute(
        pref: cuda_types::cublaslt::cublasLtMatmulPreference_t,
        attr: cuda_types::cublaslt::cublasLtMatmulPreferenceAttributes_t,
        buf: *mut ::core::ffi::c_void,
        sizeInBytes: usize,
        sizeWritten: *mut usize,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /** Query cublasLt heuristic for algorithm appropriate for given use case.

 \param[in]      lightHandle            Pointer to the allocated cuBLASLt handle for the cuBLASLt
                                        context. See cublasLtHandle_t.
 \param[in]      operationDesc          Handle to the matrix multiplication descriptor.
 \param[in]      Adesc                  Handle to the layout descriptors for matrix A.
 \param[in]      Bdesc                  Handle to the layout descriptors for matrix B.
 \param[in]      Cdesc                  Handle to the layout descriptors for matrix C.
 \param[in]      Ddesc                  Handle to the layout descriptors for matrix D.
 \param[in]      preference             Pointer to the structure holding the heuristic search
                                        preferences descriptor. See cublasLtMatrixLayout_t.
 \param[in]      requestedAlgoCount     Size of heuristicResultsArray (in elements) and requested
                                        maximum number of algorithms to return.
 \param[in, out] heuristicResultsArray  Output algorithms and associated runtime characteristics,
                                        ordered in increasing estimated compute time.
 \param[out]     returnAlgoCount        The number of heuristicResultsArray elements written.

 \retval  CUBLAS_STATUS_INVALID_VALUE   if requestedAlgoCount is less or equal to zero
 \retval  CUBLAS_STATUS_NOT_SUPPORTED   if no heuristic function available for current configuration
 \retval  CUBLAS_STATUS_SUCCESS         if query was successful, inspect
                                        heuristicResultsArray[0 to (returnAlgoCount - 1)].state
                                        for detail status of results*/
    fn cublasLtMatmulAlgoGetHeuristic(
        lightHandle: cuda_types::cublaslt::cublasLtHandle_t,
        operationDesc: cuda_types::cublaslt::cublasLtMatmulDesc_t,
        Adesc: cuda_types::cublaslt::cublasLtMatrixLayout_t,
        Bdesc: cuda_types::cublaslt::cublasLtMatrixLayout_t,
        Cdesc: cuda_types::cublaslt::cublasLtMatrixLayout_t,
        Ddesc: cuda_types::cublaslt::cublasLtMatrixLayout_t,
        preference: cuda_types::cublaslt::cublasLtMatmulPreference_t,
        requestedAlgoCount: ::core::ffi::c_int,
        heuristicResultsArray: *mut cuda_types::cublaslt::cublasLtMatmulHeuristicResult_t,
        returnAlgoCount: *mut ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /** Routine to get all algo IDs that can potentially run

 \param[in]  int              requestedAlgoCount requested number of algos (must be less or equal to size of algoIdsA
 (in elements)) \param[out] algoIdsA         array to write algoIds to \param[out] returnAlgoCount  number of algoIds
 actually written

 \retval     CUBLAS_STATUS_INVALID_VALUE  if requestedAlgoCount is less or equal to zero
 \retval     CUBLAS_STATUS_SUCCESS        if query was successful, inspect returnAlgoCount to get actual number of IDs
                                          available*/
    fn cublasLtMatmulAlgoGetIds(
        lightHandle: cuda_types::cublaslt::cublasLtHandle_t,
        computeType: cuda_types::cublas::cublasComputeType_t,
        scaleType: cuda_types::cublaslt::cudaDataType_t,
        Atype: cuda_types::cublaslt::cudaDataType_t,
        Btype: cuda_types::cublaslt::cudaDataType_t,
        Ctype: cuda_types::cublaslt::cudaDataType_t,
        Dtype: cuda_types::cublaslt::cudaDataType_t,
        requestedAlgoCount: ::core::ffi::c_int,
        algoIdsArray: *mut ::core::ffi::c_int,
        returnAlgoCount: *mut ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /** Initialize algo structure

 \retval     CUBLAS_STATUS_INVALID_VALUE  if algo is NULL or algoId is outside of recognized range
 \retval     CUBLAS_STATUS_NOT_SUPPORTED  if algoId is not supported for given combination of data types
 \retval     CUBLAS_STATUS_SUCCESS        if the structure was successfully initialized*/
    fn cublasLtMatmulAlgoInit(
        lightHandle: cuda_types::cublaslt::cublasLtHandle_t,
        computeType: cuda_types::cublas::cublasComputeType_t,
        scaleType: cuda_types::cublaslt::cudaDataType_t,
        Atype: cuda_types::cublaslt::cudaDataType_t,
        Btype: cuda_types::cublaslt::cudaDataType_t,
        Ctype: cuda_types::cublaslt::cudaDataType_t,
        Dtype: cuda_types::cublaslt::cudaDataType_t,
        algoId: ::core::ffi::c_int,
        algo: *mut cuda_types::cublaslt::cublasLtMatmulAlgo_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /** Check configured algo descriptor for correctness and support on current device.

 Result includes required workspace size and calculated wave count.

 CUBLAS_STATUS_SUCCESS doesn't fully guarantee algo will run (will fail if e.g. buffers are not correctly aligned);
 but if cublasLtMatmulAlgoCheck fails, the algo will not run.

 \param[in]  algo    algo configuration to check
 \param[out] result  result structure to report algo runtime characteristics; algo field is never updated

 \retval     CUBLAS_STATUS_INVALID_VALUE  if matrix layout descriptors or operation descriptor don't match algo
                                          descriptor
 \retval     CUBLAS_STATUS_NOT_SUPPORTED  if algo configuration or data type combination is not currently supported on
                                          given device
 \retval     CUBLAS_STATUS_ARCH_MISMATCH  if algo configuration cannot be run using the selected device
 \retval     CUBLAS_STATUS_SUCCESS        if check was successful*/
    fn cublasLtMatmulAlgoCheck(
        lightHandle: cuda_types::cublaslt::cublasLtHandle_t,
        operationDesc: cuda_types::cublaslt::cublasLtMatmulDesc_t,
        Adesc: cuda_types::cublaslt::cublasLtMatrixLayout_t,
        Bdesc: cuda_types::cublaslt::cublasLtMatrixLayout_t,
        Cdesc: cuda_types::cublaslt::cublasLtMatrixLayout_t,
        Ddesc: cuda_types::cublaslt::cublasLtMatrixLayout_t,
        algo: *const cuda_types::cublaslt::cublasLtMatmulAlgo_t,
        result: *mut cuda_types::cublaslt::cublasLtMatmulHeuristicResult_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /** Get algo capability attribute.

 E.g. to get list of supported Tile IDs:
      cublasLtMatmulTile_t tiles[CUBLASLT_MATMUL_TILE_END];
      size_t num_tiles, size_written;
      if (cublasLtMatmulAlgoCapGetAttribute(algo, CUBLASLT_ALGO_CAP_TILE_IDS, tiles, sizeof(tiles), size_written) ==
 CUBLAS_STATUS_SUCCESS) { num_tiles = size_written / sizeof(tiles[0]);
      }

 \param[in]  algo         The algo descriptor
 \param[in]  attr         The attribute
 \param[out] buf          memory address containing the new value
 \param[in]  sizeInBytes  size of buf buffer for verification (in bytes)
 \param[out] sizeWritten  only valid when return value is CUBLAS_STATUS_SUCCESS. If sizeInBytes is non-zero: number of
                          bytes actually written, if sizeInBytes is 0: number of bytes needed to write full contents

 \retval     CUBLAS_STATUS_INVALID_VALUE  if sizeInBytes is 0 and sizeWritten is NULL, or if  sizeInBytes is non-zero
                                          and buf is NULL or sizeInBytes doesn't match size of internal storage for
                                          selected attribute
 \retval     CUBLAS_STATUS_SUCCESS        if attribute's value was successfully written to user memory*/
    fn cublasLtMatmulAlgoCapGetAttribute(
        algo: *const cuda_types::cublaslt::cublasLtMatmulAlgo_t,
        attr: cuda_types::cublaslt::cublasLtMatmulAlgoCapAttributes_t,
        buf: *mut ::core::ffi::c_void,
        sizeInBytes: usize,
        sizeWritten: *mut usize,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /** Set algo configuration attribute.

 \param[in]  algo         The algo descriptor
 \param[in]  attr         The attribute
 \param[in]  buf          memory address containing the new value
 \param[in]  sizeInBytes  size of buf buffer for verification (in bytes)

 \retval     CUBLAS_STATUS_INVALID_VALUE  if buf is NULL or sizeInBytes doesn't match size of internal storage for
                                          selected attribute
 \retval     CUBLAS_STATUS_SUCCESS        if attribute was set successfully*/
    fn cublasLtMatmulAlgoConfigSetAttribute(
        algo: *mut cuda_types::cublaslt::cublasLtMatmulAlgo_t,
        attr: cuda_types::cublaslt::cublasLtMatmulAlgoConfigAttributes_t,
        buf: *const ::core::ffi::c_void,
        sizeInBytes: usize,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /** Get algo configuration attribute.

 \param[in]  algo         The algo descriptor
 \param[in]  attr         The attribute
 \param[out] buf          memory address containing the new value
 \param[in]  sizeInBytes  size of buf buffer for verification (in bytes)
 \param[out] sizeWritten  only valid when return value is CUBLAS_STATUS_SUCCESS. If sizeInBytes is non-zero: number of
                          bytes actually written, if sizeInBytes is 0: number of bytes needed to write full contents

 \retval     CUBLAS_STATUS_INVALID_VALUE  if sizeInBytes is 0 and sizeWritten is NULL, or if  sizeInBytes is non-zero
                                          and buf is NULL or sizeInBytes doesn't match size of internal storage for
                                          selected attribute
 \retval     CUBLAS_STATUS_SUCCESS        if attribute's value was successfully written to user memory*/
    fn cublasLtMatmulAlgoConfigGetAttribute(
        algo: *const cuda_types::cublaslt::cublasLtMatmulAlgo_t,
        attr: cuda_types::cublaslt::cublasLtMatmulAlgoConfigAttributes_t,
        buf: *mut ::core::ffi::c_void,
        sizeInBytes: usize,
        sizeWritten: *mut usize,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /** Experimental: Logger callback setter.

 \param[in]  callback                     a user defined callback function to be called by the logger

 \retval     CUBLAS_STATUS_SUCCESS        if callback was set successfully*/
    fn cublasLtLoggerSetCallback(
        callback: cuda_types::cublaslt::cublasLtLoggerCallback_t,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /** Experimental: Log file setter.

 \param[in]  file                         an open file with write permissions

 \retval     CUBLAS_STATUS_SUCCESS        if log file was set successfully*/
    fn cublasLtLoggerSetFile(
        file: *mut cuda_types::FILE,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /** Experimental: Open log file.

 \param[in]  logFile                      log file path. if the log file does not exist, it will be created

 \retval     CUBLAS_STATUS_SUCCESS        if log file was created successfully*/
    fn cublasLtLoggerOpenFile(
        logFile: *const ::core::ffi::c_char,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /** Experimental: Log level setter.

 \param[in]  level                        log level, should be one of the following:
                                          0. Off
                                          1. Errors
                                          2. Performance Trace
                                          3. Performance Hints
                                          4. Heuristics Trace
                                          5. API Trace

 \retval     CUBLAS_STATUS_INVALID_VALUE  if log level is not one of the above levels

 \retval     CUBLAS_STATUS_SUCCESS        if log level was set successfully*/
    fn cublasLtLoggerSetLevel(
        level: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /** Experimental: Log mask setter.

 \param[in]  mask                         log mask, should be a combination of the following masks:
                                          0.  Off
                                          1.  Errors
                                          2.  Performance Trace
                                          4.  Performance Hints
                                          8.  Heuristics Trace
                                          16. API Trace

 \retval     CUBLAS_STATUS_SUCCESS        if log mask was set successfully*/
    fn cublasLtLoggerSetMask(
        mask: ::core::ffi::c_int,
    ) -> cuda_types::cublas::cublasStatus_t;
    #[must_use]
    /** Experimental: Disable logging for the entire session.

 \retval     CUBLAS_STATUS_SUCCESS        if disabled logging*/
    fn cublasLtLoggerForceDisable() -> cuda_types::cublas::cublasStatus_t;
}
