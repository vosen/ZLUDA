// Generated automatically by zluda_bindgen
// DO NOT EDIT MANUALLY
#![allow(warnings)]
extern "system" {
    fn cudnnGetVersion() -> usize;
    fn cudnnGetMaxDeviceVersion() -> usize;
    fn cudnnGetCudartVersion() -> usize;
    fn cudnnGetErrorString(
        status: cuda_types::cudnn8::cudnnStatus_t,
    ) -> *const ::core::ffi::c_char;
    #[must_use]
    fn cudnnQueryRuntimeError(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rstatus: *mut cuda_types::cudnn8::cudnnStatus_t,
        mode: cuda_types::cudnn8::cudnnErrQueryMode_t,
        tag: *mut cuda_types::cudnn8::cudnnRuntimeTag_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetProperty(
        type_: cuda_types::cudnn8::libraryPropertyType,
        value: *mut ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnCreate(
        handle: *mut cuda_types::cudnn8::cudnnHandle_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDestroy(
        handle: cuda_types::cudnn8::cudnnHandle_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetStream(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        streamId: cuda_types::cudnn8::cudaStream_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetStream(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        streamId: *mut cuda_types::cudnn8::cudaStream_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnCreateTensorDescriptor(
        tensorDesc: *mut cuda_types::cudnn8::cudnnTensorDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetTensor4dDescriptor(
        tensorDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        format: cuda_types::cudnn8::cudnnTensorFormat_t,
        dataType: cuda_types::cudnn8::cudnnDataType_t,
        n: ::core::ffi::c_int,
        c: ::core::ffi::c_int,
        h: ::core::ffi::c_int,
        w: ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetTensor4dDescriptorEx(
        tensorDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dataType: cuda_types::cudnn8::cudnnDataType_t,
        n: ::core::ffi::c_int,
        c: ::core::ffi::c_int,
        h: ::core::ffi::c_int,
        w: ::core::ffi::c_int,
        nStride: ::core::ffi::c_int,
        cStride: ::core::ffi::c_int,
        hStride: ::core::ffi::c_int,
        wStride: ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetTensor4dDescriptor(
        tensorDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dataType: *mut cuda_types::cudnn8::cudnnDataType_t,
        n: *mut ::core::ffi::c_int,
        c: *mut ::core::ffi::c_int,
        h: *mut ::core::ffi::c_int,
        w: *mut ::core::ffi::c_int,
        nStride: *mut ::core::ffi::c_int,
        cStride: *mut ::core::ffi::c_int,
        hStride: *mut ::core::ffi::c_int,
        wStride: *mut ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetTensorNdDescriptor(
        tensorDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dataType: cuda_types::cudnn8::cudnnDataType_t,
        nbDims: ::core::ffi::c_int,
        dimA: *const ::core::ffi::c_int,
        strideA: *const ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetTensorNdDescriptorEx(
        tensorDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        format: cuda_types::cudnn8::cudnnTensorFormat_t,
        dataType: cuda_types::cudnn8::cudnnDataType_t,
        nbDims: ::core::ffi::c_int,
        dimA: *const ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetTensorNdDescriptor(
        tensorDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        nbDimsRequested: ::core::ffi::c_int,
        dataType: *mut cuda_types::cudnn8::cudnnDataType_t,
        nbDims: *mut ::core::ffi::c_int,
        dimA: *mut ::core::ffi::c_int,
        strideA: *mut ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetTensorSizeInBytes(
        tensorDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        size: *mut usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDestroyTensorDescriptor(
        tensorDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnInitTransformDest(
        transformDesc: cuda_types::cudnn8::cudnnTensorTransformDescriptor_t,
        srcDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        destDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        destSizeInBytes: *mut usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnCreateTensorTransformDescriptor(
        transformDesc: *mut cuda_types::cudnn8::cudnnTensorTransformDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetTensorTransformDescriptor(
        transformDesc: cuda_types::cudnn8::cudnnTensorTransformDescriptor_t,
        nbDims: u32,
        destFormat: cuda_types::cudnn8::cudnnTensorFormat_t,
        padBeforeA: *const i32,
        padAfterA: *const i32,
        foldA: *const u32,
        direction: cuda_types::cudnn8::cudnnFoldingDirection_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetTensorTransformDescriptor(
        transformDesc: cuda_types::cudnn8::cudnnTensorTransformDescriptor_t,
        nbDimsRequested: u32,
        destFormat: *mut cuda_types::cudnn8::cudnnTensorFormat_t,
        padBeforeA: *mut i32,
        padAfterA: *mut i32,
        foldA: *mut u32,
        direction: *mut cuda_types::cudnn8::cudnnFoldingDirection_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDestroyTensorTransformDescriptor(
        transformDesc: cuda_types::cudnn8::cudnnTensorTransformDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnTransformTensor(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        alpha: *const ::core::ffi::c_void,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnTransformTensorEx(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        transDesc: cuda_types::cudnn8::cudnnTensorTransformDescriptor_t,
        alpha: *const ::core::ffi::c_void,
        srcDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        srcData: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        destDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        destData: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnAddTensor(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        alpha: *const ::core::ffi::c_void,
        aDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        A: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        cDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        C: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnCreateOpTensorDescriptor(
        opTensorDesc: *mut cuda_types::cudnn8::cudnnOpTensorDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetOpTensorDescriptor(
        opTensorDesc: cuda_types::cudnn8::cudnnOpTensorDescriptor_t,
        opTensorOp: cuda_types::cudnn8::cudnnOpTensorOp_t,
        opTensorCompType: cuda_types::cudnn8::cudnnDataType_t,
        opTensorNanOpt: cuda_types::cudnn8::cudnnNanPropagation_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetOpTensorDescriptor(
        opTensorDesc: cuda_types::cudnn8::cudnnOpTensorDescriptor_t,
        opTensorOp: *mut cuda_types::cudnn8::cudnnOpTensorOp_t,
        opTensorCompType: *mut cuda_types::cudnn8::cudnnDataType_t,
        opTensorNanOpt: *mut cuda_types::cudnn8::cudnnNanPropagation_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDestroyOpTensorDescriptor(
        opTensorDesc: cuda_types::cudnn8::cudnnOpTensorDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnOpTensor(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        opTensorDesc: cuda_types::cudnn8::cudnnOpTensorDescriptor_t,
        alpha1: *const ::core::ffi::c_void,
        aDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        A: *const ::core::ffi::c_void,
        alpha2: *const ::core::ffi::c_void,
        bDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        B: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        cDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        C: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnCreateReduceTensorDescriptor(
        reduceTensorDesc: *mut cuda_types::cudnn8::cudnnReduceTensorDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetReduceTensorDescriptor(
        reduceTensorDesc: cuda_types::cudnn8::cudnnReduceTensorDescriptor_t,
        reduceTensorOp: cuda_types::cudnn8::cudnnReduceTensorOp_t,
        reduceTensorCompType: cuda_types::cudnn8::cudnnDataType_t,
        reduceTensorNanOpt: cuda_types::cudnn8::cudnnNanPropagation_t,
        reduceTensorIndices: cuda_types::cudnn8::cudnnReduceTensorIndices_t,
        reduceTensorIndicesType: cuda_types::cudnn8::cudnnIndicesType_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetReduceTensorDescriptor(
        reduceTensorDesc: cuda_types::cudnn8::cudnnReduceTensorDescriptor_t,
        reduceTensorOp: *mut cuda_types::cudnn8::cudnnReduceTensorOp_t,
        reduceTensorCompType: *mut cuda_types::cudnn8::cudnnDataType_t,
        reduceTensorNanOpt: *mut cuda_types::cudnn8::cudnnNanPropagation_t,
        reduceTensorIndices: *mut cuda_types::cudnn8::cudnnReduceTensorIndices_t,
        reduceTensorIndicesType: *mut cuda_types::cudnn8::cudnnIndicesType_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDestroyReduceTensorDescriptor(
        reduceTensorDesc: cuda_types::cudnn8::cudnnReduceTensorDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetReductionIndicesSize(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        reduceTensorDesc: cuda_types::cudnn8::cudnnReduceTensorDescriptor_t,
        aDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        cDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        sizeInBytes: *mut usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetReductionWorkspaceSize(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        reduceTensorDesc: cuda_types::cudnn8::cudnnReduceTensorDescriptor_t,
        aDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        cDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        sizeInBytes: *mut usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnReduceTensor(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        reduceTensorDesc: cuda_types::cudnn8::cudnnReduceTensorDescriptor_t,
        indices: *mut ::core::ffi::c_void,
        indicesSizeInBytes: usize,
        workspace: *mut ::core::ffi::c_void,
        workspaceSizeInBytes: usize,
        alpha: *const ::core::ffi::c_void,
        aDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        A: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        cDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        C: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetTensor(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        valuePtr: *const ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnScaleTensor(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        alpha: *const ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnCreateFilterDescriptor(
        filterDesc: *mut cuda_types::cudnn8::cudnnFilterDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetFilter4dDescriptor(
        filterDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        dataType: cuda_types::cudnn8::cudnnDataType_t,
        format: cuda_types::cudnn8::cudnnTensorFormat_t,
        k: ::core::ffi::c_int,
        c: ::core::ffi::c_int,
        h: ::core::ffi::c_int,
        w: ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetFilter4dDescriptor(
        filterDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        dataType: *mut cuda_types::cudnn8::cudnnDataType_t,
        format: *mut cuda_types::cudnn8::cudnnTensorFormat_t,
        k: *mut ::core::ffi::c_int,
        c: *mut ::core::ffi::c_int,
        h: *mut ::core::ffi::c_int,
        w: *mut ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetFilterNdDescriptor(
        filterDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        dataType: cuda_types::cudnn8::cudnnDataType_t,
        format: cuda_types::cudnn8::cudnnTensorFormat_t,
        nbDims: ::core::ffi::c_int,
        filterDimA: *const ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetFilterNdDescriptor(
        filterDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        nbDimsRequested: ::core::ffi::c_int,
        dataType: *mut cuda_types::cudnn8::cudnnDataType_t,
        format: *mut cuda_types::cudnn8::cudnnTensorFormat_t,
        nbDims: *mut ::core::ffi::c_int,
        filterDimA: *mut ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetFilterSizeInBytes(
        filterDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        size: *mut usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnTransformFilter(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        transDesc: cuda_types::cudnn8::cudnnTensorTransformDescriptor_t,
        alpha: *const ::core::ffi::c_void,
        srcDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        srcData: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        destDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        destData: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDestroyFilterDescriptor(
        filterDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSoftmaxForward(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        algo: cuda_types::cudnn8::cudnnSoftmaxAlgorithm_t,
        mode: cuda_types::cudnn8::cudnnSoftmaxMode_t,
        alpha: *const ::core::ffi::c_void,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnCreatePoolingDescriptor(
        poolingDesc: *mut cuda_types::cudnn8::cudnnPoolingDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetPooling2dDescriptor(
        poolingDesc: cuda_types::cudnn8::cudnnPoolingDescriptor_t,
        mode: cuda_types::cudnn8::cudnnPoolingMode_t,
        maxpoolingNanOpt: cuda_types::cudnn8::cudnnNanPropagation_t,
        windowHeight: ::core::ffi::c_int,
        windowWidth: ::core::ffi::c_int,
        verticalPadding: ::core::ffi::c_int,
        horizontalPadding: ::core::ffi::c_int,
        verticalStride: ::core::ffi::c_int,
        horizontalStride: ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetPooling2dDescriptor(
        poolingDesc: cuda_types::cudnn8::cudnnPoolingDescriptor_t,
        mode: *mut cuda_types::cudnn8::cudnnPoolingMode_t,
        maxpoolingNanOpt: *mut cuda_types::cudnn8::cudnnNanPropagation_t,
        windowHeight: *mut ::core::ffi::c_int,
        windowWidth: *mut ::core::ffi::c_int,
        verticalPadding: *mut ::core::ffi::c_int,
        horizontalPadding: *mut ::core::ffi::c_int,
        verticalStride: *mut ::core::ffi::c_int,
        horizontalStride: *mut ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetPoolingNdDescriptor(
        poolingDesc: cuda_types::cudnn8::cudnnPoolingDescriptor_t,
        mode: cuda_types::cudnn8::cudnnPoolingMode_t,
        maxpoolingNanOpt: cuda_types::cudnn8::cudnnNanPropagation_t,
        nbDims: ::core::ffi::c_int,
        windowDimA: *const ::core::ffi::c_int,
        paddingA: *const ::core::ffi::c_int,
        strideA: *const ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetPoolingNdDescriptor(
        poolingDesc: cuda_types::cudnn8::cudnnPoolingDescriptor_t,
        nbDimsRequested: ::core::ffi::c_int,
        mode: *mut cuda_types::cudnn8::cudnnPoolingMode_t,
        maxpoolingNanOpt: *mut cuda_types::cudnn8::cudnnNanPropagation_t,
        nbDims: *mut ::core::ffi::c_int,
        windowDimA: *mut ::core::ffi::c_int,
        paddingA: *mut ::core::ffi::c_int,
        strideA: *mut ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetPoolingNdForwardOutputDim(
        poolingDesc: cuda_types::cudnn8::cudnnPoolingDescriptor_t,
        inputTensorDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        nbDims: ::core::ffi::c_int,
        outputTensorDimA: *mut ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetPooling2dForwardOutputDim(
        poolingDesc: cuda_types::cudnn8::cudnnPoolingDescriptor_t,
        inputTensorDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        n: *mut ::core::ffi::c_int,
        c: *mut ::core::ffi::c_int,
        h: *mut ::core::ffi::c_int,
        w: *mut ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDestroyPoolingDescriptor(
        poolingDesc: cuda_types::cudnn8::cudnnPoolingDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnPoolingForward(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        poolingDesc: cuda_types::cudnn8::cudnnPoolingDescriptor_t,
        alpha: *const ::core::ffi::c_void,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnCreateActivationDescriptor(
        activationDesc: *mut cuda_types::cudnn8::cudnnActivationDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetActivationDescriptor(
        activationDesc: cuda_types::cudnn8::cudnnActivationDescriptor_t,
        mode: cuda_types::cudnn8::cudnnActivationMode_t,
        reluNanOpt: cuda_types::cudnn8::cudnnNanPropagation_t,
        coef: f64,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetActivationDescriptor(
        activationDesc: cuda_types::cudnn8::cudnnActivationDescriptor_t,
        mode: *mut cuda_types::cudnn8::cudnnActivationMode_t,
        reluNanOpt: *mut cuda_types::cudnn8::cudnnNanPropagation_t,
        coef: *mut f64,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetActivationDescriptorSwishBeta(
        activationDesc: cuda_types::cudnn8::cudnnActivationDescriptor_t,
        swish_beta: f64,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetActivationDescriptorSwishBeta(
        activationDesc: cuda_types::cudnn8::cudnnActivationDescriptor_t,
        swish_beta: *mut f64,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDestroyActivationDescriptor(
        activationDesc: cuda_types::cudnn8::cudnnActivationDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnActivationForward(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        activationDesc: cuda_types::cudnn8::cudnnActivationDescriptor_t,
        alpha: *const ::core::ffi::c_void,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnCreateLRNDescriptor(
        normDesc: *mut cuda_types::cudnn8::cudnnLRNDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetLRNDescriptor(
        normDesc: cuda_types::cudnn8::cudnnLRNDescriptor_t,
        lrnN: ::core::ffi::c_uint,
        lrnAlpha: f64,
        lrnBeta: f64,
        lrnK: f64,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetLRNDescriptor(
        normDesc: cuda_types::cudnn8::cudnnLRNDescriptor_t,
        lrnN: *mut ::core::ffi::c_uint,
        lrnAlpha: *mut f64,
        lrnBeta: *mut f64,
        lrnK: *mut f64,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDestroyLRNDescriptor(
        lrnDesc: cuda_types::cudnn8::cudnnLRNDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnLRNCrossChannelForward(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        normDesc: cuda_types::cudnn8::cudnnLRNDescriptor_t,
        lrnMode: cuda_types::cudnn8::cudnnLRNMode_t,
        alpha: *const ::core::ffi::c_void,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDivisiveNormalizationForward(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        normDesc: cuda_types::cudnn8::cudnnLRNDescriptor_t,
        mode: cuda_types::cudnn8::cudnnDivNormMode_t,
        alpha: *const ::core::ffi::c_void,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        means: *const ::core::ffi::c_void,
        temp: *mut ::core::ffi::c_void,
        temp2: *mut ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDeriveBNTensorDescriptor(
        derivedBnDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        mode: cuda_types::cudnn8::cudnnBatchNormMode_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnBatchNormalizationForwardInference(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        mode: cuda_types::cudnn8::cudnnBatchNormMode_t,
        alpha: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        bnScaleBiasMeanVarDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        bnScale: *const ::core::ffi::c_void,
        bnBias: *const ::core::ffi::c_void,
        estimatedMean: *const ::core::ffi::c_void,
        estimatedVariance: *const ::core::ffi::c_void,
        epsilon: f64,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDeriveNormTensorDescriptor(
        derivedNormScaleBiasDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        derivedNormMeanVarDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        mode: cuda_types::cudnn8::cudnnNormMode_t,
        groupCnt: ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnNormalizationForwardInference(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        mode: cuda_types::cudnn8::cudnnNormMode_t,
        normOps: cuda_types::cudnn8::cudnnNormOps_t,
        algo: cuda_types::cudnn8::cudnnNormAlgo_t,
        alpha: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        normScaleBiasDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        normScale: *const ::core::ffi::c_void,
        normBias: *const ::core::ffi::c_void,
        normMeanVarDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        estimatedMean: *const ::core::ffi::c_void,
        estimatedVariance: *const ::core::ffi::c_void,
        zDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        z: *const ::core::ffi::c_void,
        activationDesc: cuda_types::cudnn8::cudnnActivationDescriptor_t,
        yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        epsilon: f64,
        groupCnt: ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnCreateSpatialTransformerDescriptor(
        stDesc: *mut cuda_types::cudnn8::cudnnSpatialTransformerDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetSpatialTransformerNdDescriptor(
        stDesc: cuda_types::cudnn8::cudnnSpatialTransformerDescriptor_t,
        samplerType: cuda_types::cudnn8::cudnnSamplerType_t,
        dataType: cuda_types::cudnn8::cudnnDataType_t,
        nbDims: ::core::ffi::c_int,
        dimA: *const ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDestroySpatialTransformerDescriptor(
        stDesc: cuda_types::cudnn8::cudnnSpatialTransformerDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSpatialTfGridGeneratorForward(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        stDesc: cuda_types::cudnn8::cudnnSpatialTransformerDescriptor_t,
        theta: *const ::core::ffi::c_void,
        grid: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSpatialTfSamplerForward(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        stDesc: cuda_types::cudnn8::cudnnSpatialTransformerDescriptor_t,
        alpha: *const ::core::ffi::c_void,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        grid: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnCreateDropoutDescriptor(
        dropoutDesc: *mut cuda_types::cudnn8::cudnnDropoutDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDestroyDropoutDescriptor(
        dropoutDesc: cuda_types::cudnn8::cudnnDropoutDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDropoutGetStatesSize(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        sizeInBytes: *mut usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDropoutGetReserveSpaceSize(
        xdesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        sizeInBytes: *mut usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetDropoutDescriptor(
        dropoutDesc: cuda_types::cudnn8::cudnnDropoutDescriptor_t,
        handle: cuda_types::cudnn8::cudnnHandle_t,
        dropout: f32,
        states: *mut ::core::ffi::c_void,
        stateSizeInBytes: usize,
        seed: ::core::ffi::c_ulonglong,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnRestoreDropoutDescriptor(
        dropoutDesc: cuda_types::cudnn8::cudnnDropoutDescriptor_t,
        handle: cuda_types::cudnn8::cudnnHandle_t,
        dropout: f32,
        states: *mut ::core::ffi::c_void,
        stateSizeInBytes: usize,
        seed: ::core::ffi::c_ulonglong,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetDropoutDescriptor(
        dropoutDesc: cuda_types::cudnn8::cudnnDropoutDescriptor_t,
        handle: cuda_types::cudnn8::cudnnHandle_t,
        dropout: *mut f32,
        states: *mut *mut ::core::ffi::c_void,
        seed: *mut ::core::ffi::c_ulonglong,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDropoutForward(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        dropoutDesc: cuda_types::cudnn8::cudnnDropoutDescriptor_t,
        xdesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        ydesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        reserveSpace: *mut ::core::ffi::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnCreateAlgorithmDescriptor(
        algoDesc: *mut cuda_types::cudnn8::cudnnAlgorithmDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetAlgorithmDescriptor(
        algoDesc: cuda_types::cudnn8::cudnnAlgorithmDescriptor_t,
        algorithm: cuda_types::cudnn8::cudnnAlgorithm_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetAlgorithmDescriptor(
        algoDesc: cuda_types::cudnn8::cudnnAlgorithmDescriptor_t,
        algorithm: *mut cuda_types::cudnn8::cudnnAlgorithm_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnCopyAlgorithmDescriptor(
        src: cuda_types::cudnn8::cudnnAlgorithmDescriptor_t,
        dest: cuda_types::cudnn8::cudnnAlgorithmDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDestroyAlgorithmDescriptor(
        algoDesc: cuda_types::cudnn8::cudnnAlgorithmDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnCreateAlgorithmPerformance(
        algoPerf: *mut cuda_types::cudnn8::cudnnAlgorithmPerformance_t,
        numberToCreate: ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetAlgorithmPerformance(
        algoPerf: cuda_types::cudnn8::cudnnAlgorithmPerformance_t,
        algoDesc: cuda_types::cudnn8::cudnnAlgorithmDescriptor_t,
        status: cuda_types::cudnn8::cudnnStatus_t,
        time: f32,
        memory: usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetAlgorithmPerformance(
        algoPerf: cuda_types::cudnn8::cudnnAlgorithmPerformance_t,
        algoDesc: *mut cuda_types::cudnn8::cudnnAlgorithmDescriptor_t,
        status: *mut cuda_types::cudnn8::cudnnStatus_t,
        time: *mut f32,
        memory: *mut usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDestroyAlgorithmPerformance(
        algoPerf: *mut cuda_types::cudnn8::cudnnAlgorithmPerformance_t,
        numberToDestroy: ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetAlgorithmSpaceSize(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        algoDesc: cuda_types::cudnn8::cudnnAlgorithmDescriptor_t,
        algoSpaceSizeInBytes: *mut usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSaveAlgorithm(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        algoDesc: cuda_types::cudnn8::cudnnAlgorithmDescriptor_t,
        algoSpace: *mut ::core::ffi::c_void,
        algoSpaceSizeInBytes: usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnRestoreAlgorithm(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        algoSpace: *mut ::core::ffi::c_void,
        algoSpaceSizeInBytes: usize,
        algoDesc: cuda_types::cudnn8::cudnnAlgorithmDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetCallback(
        mask: ::core::ffi::c_uint,
        udata: *mut ::core::ffi::c_void,
        fptr: cuda_types::cudnn8::cudnnCallback_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetCallback(
        mask: *mut ::core::ffi::c_uint,
        udata: *mut *mut ::core::ffi::c_void,
        fptr: *mut cuda_types::cudnn8::cudnnCallback_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnOpsInferVersionCheck() -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSoftmaxBackward(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        algo: cuda_types::cudnn8::cudnnSoftmaxAlgorithm_t,
        mode: cuda_types::cudnn8::cudnnSoftmaxMode_t,
        alpha: *const ::core::ffi::c_void,
        yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        y: *const ::core::ffi::c_void,
        dyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        dxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnPoolingBackward(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        poolingDesc: cuda_types::cudnn8::cudnnPoolingDescriptor_t,
        alpha: *const ::core::ffi::c_void,
        yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        y: *const ::core::ffi::c_void,
        dyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        dxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnActivationBackward(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        activationDesc: cuda_types::cudnn8::cudnnActivationDescriptor_t,
        alpha: *const ::core::ffi::c_void,
        yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        y: *const ::core::ffi::c_void,
        dyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        dxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnLRNCrossChannelBackward(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        normDesc: cuda_types::cudnn8::cudnnLRNDescriptor_t,
        lrnMode: cuda_types::cudnn8::cudnnLRNMode_t,
        alpha: *const ::core::ffi::c_void,
        yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        y: *const ::core::ffi::c_void,
        dyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        dxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDivisiveNormalizationBackward(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        normDesc: cuda_types::cudnn8::cudnnLRNDescriptor_t,
        mode: cuda_types::cudnn8::cudnnDivNormMode_t,
        alpha: *const ::core::ffi::c_void,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        means: *const ::core::ffi::c_void,
        dy: *const ::core::ffi::c_void,
        temp: *mut ::core::ffi::c_void,
        temp2: *mut ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        dXdMeansDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
        dMeans: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        mode: cuda_types::cudnn8::cudnnBatchNormMode_t,
        bnOps: cuda_types::cudnn8::cudnnBatchNormOps_t,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        zDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        bnScaleBiasMeanVarDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        activationDesc: cuda_types::cudnn8::cudnnActivationDescriptor_t,
        sizeInBytes: *mut usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetBatchNormalizationBackwardExWorkspaceSize(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        mode: cuda_types::cudnn8::cudnnBatchNormMode_t,
        bnOps: cuda_types::cudnn8::cudnnBatchNormOps_t,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dzDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dBnScaleBiasDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        activationDesc: cuda_types::cudnn8::cudnnActivationDescriptor_t,
        sizeInBytes: *mut usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetBatchNormalizationTrainingExReserveSpaceSize(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        mode: cuda_types::cudnn8::cudnnBatchNormMode_t,
        bnOps: cuda_types::cudnn8::cudnnBatchNormOps_t,
        activationDesc: cuda_types::cudnn8::cudnnActivationDescriptor_t,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        sizeInBytes: *mut usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnBatchNormalizationForwardTraining(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        mode: cuda_types::cudnn8::cudnnBatchNormMode_t,
        alpha: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        bnScaleBiasMeanVarDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        bnScale: *const ::core::ffi::c_void,
        bnBias: *const ::core::ffi::c_void,
        exponentialAverageFactor: f64,
        resultRunningMean: *mut ::core::ffi::c_void,
        resultRunningVariance: *mut ::core::ffi::c_void,
        epsilon: f64,
        resultSaveMean: *mut ::core::ffi::c_void,
        resultSaveInvVariance: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnBatchNormalizationForwardTrainingEx(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        mode: cuda_types::cudnn8::cudnnBatchNormMode_t,
        bnOps: cuda_types::cudnn8::cudnnBatchNormOps_t,
        alpha: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        xData: *const ::core::ffi::c_void,
        zDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        zData: *const ::core::ffi::c_void,
        yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        yData: *mut ::core::ffi::c_void,
        bnScaleBiasMeanVarDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        bnScale: *const ::core::ffi::c_void,
        bnBias: *const ::core::ffi::c_void,
        exponentialAverageFactor: f64,
        resultRunningMean: *mut ::core::ffi::c_void,
        resultRunningVariance: *mut ::core::ffi::c_void,
        epsilon: f64,
        resultSaveMean: *mut ::core::ffi::c_void,
        resultSaveInvVariance: *mut ::core::ffi::c_void,
        activationDesc: cuda_types::cudnn8::cudnnActivationDescriptor_t,
        workspace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        reserveSpace: *mut ::core::ffi::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnBatchNormalizationBackward(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        mode: cuda_types::cudnn8::cudnnBatchNormMode_t,
        alphaDataDiff: *const ::core::ffi::c_void,
        betaDataDiff: *const ::core::ffi::c_void,
        alphaParamDiff: *const ::core::ffi::c_void,
        betaParamDiff: *const ::core::ffi::c_void,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        dyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        dxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
        dBnScaleBiasDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        bnScale: *const ::core::ffi::c_void,
        dBnScaleResult: *mut ::core::ffi::c_void,
        dBnBiasResult: *mut ::core::ffi::c_void,
        epsilon: f64,
        savedMean: *const ::core::ffi::c_void,
        savedInvVariance: *const ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnBatchNormalizationBackwardEx(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        mode: cuda_types::cudnn8::cudnnBatchNormMode_t,
        bnOps: cuda_types::cudnn8::cudnnBatchNormOps_t,
        alphaDataDiff: *const ::core::ffi::c_void,
        betaDataDiff: *const ::core::ffi::c_void,
        alphaParamDiff: *const ::core::ffi::c_void,
        betaParamDiff: *const ::core::ffi::c_void,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        xData: *const ::core::ffi::c_void,
        yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        yData: *const ::core::ffi::c_void,
        dyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dyData: *const ::core::ffi::c_void,
        dzDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dzData: *mut ::core::ffi::c_void,
        dxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dxData: *mut ::core::ffi::c_void,
        dBnScaleBiasDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        bnScaleData: *const ::core::ffi::c_void,
        bnBiasData: *const ::core::ffi::c_void,
        dBnScaleData: *mut ::core::ffi::c_void,
        dBnBiasData: *mut ::core::ffi::c_void,
        epsilon: f64,
        savedMean: *const ::core::ffi::c_void,
        savedInvVariance: *const ::core::ffi::c_void,
        activationDesc: cuda_types::cudnn8::cudnnActivationDescriptor_t,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        reserveSpace: *mut ::core::ffi::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetNormalizationForwardTrainingWorkspaceSize(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        mode: cuda_types::cudnn8::cudnnNormMode_t,
        normOps: cuda_types::cudnn8::cudnnNormOps_t,
        algo: cuda_types::cudnn8::cudnnNormAlgo_t,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        zDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        normScaleBiasDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        activationDesc: cuda_types::cudnn8::cudnnActivationDescriptor_t,
        normMeanVarDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        sizeInBytes: *mut usize,
        groupCnt: ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetNormalizationBackwardWorkspaceSize(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        mode: cuda_types::cudnn8::cudnnNormMode_t,
        normOps: cuda_types::cudnn8::cudnnNormOps_t,
        algo: cuda_types::cudnn8::cudnnNormAlgo_t,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dzDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dNormScaleBiasDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        activationDesc: cuda_types::cudnn8::cudnnActivationDescriptor_t,
        normMeanVarDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        sizeInBytes: *mut usize,
        groupCnt: ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetNormalizationTrainingReserveSpaceSize(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        mode: cuda_types::cudnn8::cudnnNormMode_t,
        normOps: cuda_types::cudnn8::cudnnNormOps_t,
        algo: cuda_types::cudnn8::cudnnNormAlgo_t,
        activationDesc: cuda_types::cudnn8::cudnnActivationDescriptor_t,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        sizeInBytes: *mut usize,
        groupCnt: ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnNormalizationForwardTraining(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        mode: cuda_types::cudnn8::cudnnNormMode_t,
        normOps: cuda_types::cudnn8::cudnnNormOps_t,
        algo: cuda_types::cudnn8::cudnnNormAlgo_t,
        alpha: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        xData: *const ::core::ffi::c_void,
        normScaleBiasDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        normScale: *const ::core::ffi::c_void,
        normBias: *const ::core::ffi::c_void,
        exponentialAverageFactor: f64,
        normMeanVarDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        resultRunningMean: *mut ::core::ffi::c_void,
        resultRunningVariance: *mut ::core::ffi::c_void,
        epsilon: f64,
        resultSaveMean: *mut ::core::ffi::c_void,
        resultSaveInvVariance: *mut ::core::ffi::c_void,
        activationDesc: cuda_types::cudnn8::cudnnActivationDescriptor_t,
        zDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        zData: *const ::core::ffi::c_void,
        yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        yData: *mut ::core::ffi::c_void,
        workspace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        reserveSpace: *mut ::core::ffi::c_void,
        reserveSpaceSizeInBytes: usize,
        groupCnt: ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnNormalizationBackward(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        mode: cuda_types::cudnn8::cudnnNormMode_t,
        normOps: cuda_types::cudnn8::cudnnNormOps_t,
        algo: cuda_types::cudnn8::cudnnNormAlgo_t,
        alphaDataDiff: *const ::core::ffi::c_void,
        betaDataDiff: *const ::core::ffi::c_void,
        alphaParamDiff: *const ::core::ffi::c_void,
        betaParamDiff: *const ::core::ffi::c_void,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        xData: *const ::core::ffi::c_void,
        yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        yData: *const ::core::ffi::c_void,
        dyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dyData: *const ::core::ffi::c_void,
        dzDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dzData: *mut ::core::ffi::c_void,
        dxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dxData: *mut ::core::ffi::c_void,
        dNormScaleBiasDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        normScaleData: *const ::core::ffi::c_void,
        normBiasData: *const ::core::ffi::c_void,
        dNormScaleData: *mut ::core::ffi::c_void,
        dNormBiasData: *mut ::core::ffi::c_void,
        epsilon: f64,
        normMeanVarDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        savedMean: *const ::core::ffi::c_void,
        savedInvVariance: *const ::core::ffi::c_void,
        activationDesc: cuda_types::cudnn8::cudnnActivationDescriptor_t,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        reserveSpace: *mut ::core::ffi::c_void,
        reserveSpaceSizeInBytes: usize,
        groupCnt: ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSpatialTfGridGeneratorBackward(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        stDesc: cuda_types::cudnn8::cudnnSpatialTransformerDescriptor_t,
        dgrid: *const ::core::ffi::c_void,
        dtheta: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSpatialTfSamplerBackward(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        stDesc: cuda_types::cudnn8::cudnnSpatialTransformerDescriptor_t,
        alpha: *const ::core::ffi::c_void,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        dxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
        alphaDgrid: *const ::core::ffi::c_void,
        dyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        grid: *const ::core::ffi::c_void,
        betaDgrid: *const ::core::ffi::c_void,
        dgrid: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDropoutBackward(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        dropoutDesc: cuda_types::cudnn8::cudnnDropoutDescriptor_t,
        dydesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        dxdesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
        reserveSpace: *mut ::core::ffi::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnOpsTrainVersionCheck() -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnCreateRNNDescriptor(
        rnnDesc: *mut cuda_types::cudnn8::cudnnRNNDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDestroyRNNDescriptor(
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetRNNDescriptor_v8(
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        algo: cuda_types::cudnn8::cudnnRNNAlgo_t,
        cellMode: cuda_types::cudnn8::cudnnRNNMode_t,
        biasMode: cuda_types::cudnn8::cudnnRNNBiasMode_t,
        dirMode: cuda_types::cudnn8::cudnnDirectionMode_t,
        inputMode: cuda_types::cudnn8::cudnnRNNInputMode_t,
        dataType: cuda_types::cudnn8::cudnnDataType_t,
        mathPrec: cuda_types::cudnn8::cudnnDataType_t,
        mathType: cuda_types::cudnn8::cudnnMathType_t,
        inputSize: i32,
        hiddenSize: i32,
        projSize: i32,
        numLayers: i32,
        dropoutDesc: cuda_types::cudnn8::cudnnDropoutDescriptor_t,
        auxFlags: u32,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetRNNDescriptor_v8(
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        algo: *mut cuda_types::cudnn8::cudnnRNNAlgo_t,
        cellMode: *mut cuda_types::cudnn8::cudnnRNNMode_t,
        biasMode: *mut cuda_types::cudnn8::cudnnRNNBiasMode_t,
        dirMode: *mut cuda_types::cudnn8::cudnnDirectionMode_t,
        inputMode: *mut cuda_types::cudnn8::cudnnRNNInputMode_t,
        dataType: *mut cuda_types::cudnn8::cudnnDataType_t,
        mathPrec: *mut cuda_types::cudnn8::cudnnDataType_t,
        mathType: *mut cuda_types::cudnn8::cudnnMathType_t,
        inputSize: *mut i32,
        hiddenSize: *mut i32,
        projSize: *mut i32,
        numLayers: *mut i32,
        dropoutDesc: *mut cuda_types::cudnn8::cudnnDropoutDescriptor_t,
        auxFlags: *mut u32,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetRNNDescriptor_v6(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        hiddenSize: ::core::ffi::c_int,
        numLayers: ::core::ffi::c_int,
        dropoutDesc: cuda_types::cudnn8::cudnnDropoutDescriptor_t,
        inputMode: cuda_types::cudnn8::cudnnRNNInputMode_t,
        direction: cuda_types::cudnn8::cudnnDirectionMode_t,
        cellMode: cuda_types::cudnn8::cudnnRNNMode_t,
        algo: cuda_types::cudnn8::cudnnRNNAlgo_t,
        mathPrec: cuda_types::cudnn8::cudnnDataType_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetRNNDescriptor_v6(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        hiddenSize: *mut ::core::ffi::c_int,
        numLayers: *mut ::core::ffi::c_int,
        dropoutDesc: *mut cuda_types::cudnn8::cudnnDropoutDescriptor_t,
        inputMode: *mut cuda_types::cudnn8::cudnnRNNInputMode_t,
        direction: *mut cuda_types::cudnn8::cudnnDirectionMode_t,
        cellMode: *mut cuda_types::cudnn8::cudnnRNNMode_t,
        algo: *mut cuda_types::cudnn8::cudnnRNNAlgo_t,
        mathPrec: *mut cuda_types::cudnn8::cudnnDataType_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetRNNMatrixMathType(
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        mType: cuda_types::cudnn8::cudnnMathType_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetRNNMatrixMathType(
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        mType: *mut cuda_types::cudnn8::cudnnMathType_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetRNNBiasMode(
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        biasMode: cuda_types::cudnn8::cudnnRNNBiasMode_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetRNNBiasMode(
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        biasMode: *mut cuda_types::cudnn8::cudnnRNNBiasMode_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnRNNSetClip_v8(
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        clipMode: cuda_types::cudnn8::cudnnRNNClipMode_t,
        clipNanOpt: cuda_types::cudnn8::cudnnNanPropagation_t,
        lclip: f64,
        rclip: f64,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnRNNGetClip_v8(
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        clipMode: *mut cuda_types::cudnn8::cudnnRNNClipMode_t,
        clipNanOpt: *mut cuda_types::cudnn8::cudnnNanPropagation_t,
        lclip: *mut f64,
        rclip: *mut f64,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnRNNSetClip(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        clipMode: cuda_types::cudnn8::cudnnRNNClipMode_t,
        clipNanOpt: cuda_types::cudnn8::cudnnNanPropagation_t,
        lclip: f64,
        rclip: f64,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnRNNGetClip(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        clipMode: *mut cuda_types::cudnn8::cudnnRNNClipMode_t,
        clipNanOpt: *mut cuda_types::cudnn8::cudnnNanPropagation_t,
        lclip: *mut f64,
        rclip: *mut f64,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetRNNProjectionLayers(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        recProjSize: ::core::ffi::c_int,
        outProjSize: ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetRNNProjectionLayers(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        recProjSize: *mut ::core::ffi::c_int,
        outProjSize: *mut ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnCreatePersistentRNNPlan(
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        minibatch: ::core::ffi::c_int,
        dataType: cuda_types::cudnn8::cudnnDataType_t,
        plan: *mut cuda_types::cudnn8::cudnnPersistentRNNPlan_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDestroyPersistentRNNPlan(
        plan: cuda_types::cudnn8::cudnnPersistentRNNPlan_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetPersistentRNNPlan(
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        plan: cuda_types::cudnn8::cudnnPersistentRNNPlan_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnBuildRNNDynamic(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        miniBatch: ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetRNNWorkspaceSize(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        seqLength: ::core::ffi::c_int,
        xDesc: *const cuda_types::cudnn8::cudnnTensorDescriptor_t,
        sizeInBytes: *mut usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetRNNTrainingReserveSize(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        seqLength: ::core::ffi::c_int,
        xDesc: *const cuda_types::cudnn8::cudnnTensorDescriptor_t,
        sizeInBytes: *mut usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetRNNTempSpaceSizes(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        fwdMode: cuda_types::cudnn8::cudnnForwardMode_t,
        xDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
        workSpaceSize: *mut usize,
        reserveSpaceSize: *mut usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetRNNParamsSize(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        sizeInBytes: *mut usize,
        dataType: cuda_types::cudnn8::cudnnDataType_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetRNNWeightSpaceSize(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        weightSpaceSize: *mut usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetRNNLinLayerMatrixParams(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        pseudoLayer: ::core::ffi::c_int,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        wDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        w: *const ::core::ffi::c_void,
        linLayerID: ::core::ffi::c_int,
        linLayerMatDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        linLayerMat: *mut *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetRNNLinLayerBiasParams(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        pseudoLayer: ::core::ffi::c_int,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        wDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        w: *const ::core::ffi::c_void,
        linLayerID: ::core::ffi::c_int,
        linLayerBiasDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        linLayerBias: *mut *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetRNNWeightParams(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        pseudoLayer: i32,
        weightSpaceSize: usize,
        weightSpace: *const ::core::ffi::c_void,
        linLayerID: i32,
        mDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        mAddr: *mut *mut ::core::ffi::c_void,
        bDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        bAddr: *mut *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnRNNForwardInference(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        seqLength: ::core::ffi::c_int,
        xDesc: *const cuda_types::cudnn8::cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        hxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        hx: *const ::core::ffi::c_void,
        cxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        cx: *const ::core::ffi::c_void,
        wDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        w: *const ::core::ffi::c_void,
        yDesc: *const cuda_types::cudnn8::cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        hyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        hy: *mut ::core::ffi::c_void,
        cyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        cy: *mut ::core::ffi::c_void,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetRNNPaddingMode(
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        paddingMode: ::core::ffi::c_uint,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetRNNPaddingMode(
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        paddingMode: *mut ::core::ffi::c_uint,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnCreateRNNDataDescriptor(
        rnnDataDesc: *mut cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDestroyRNNDataDescriptor(
        rnnDataDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetRNNDataDescriptor(
        rnnDataDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
        dataType: cuda_types::cudnn8::cudnnDataType_t,
        layout: cuda_types::cudnn8::cudnnRNNDataLayout_t,
        maxSeqLength: ::core::ffi::c_int,
        batchSize: ::core::ffi::c_int,
        vectorSize: ::core::ffi::c_int,
        seqLengthArray: *const ::core::ffi::c_int,
        paddingFill: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetRNNDataDescriptor(
        rnnDataDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
        dataType: *mut cuda_types::cudnn8::cudnnDataType_t,
        layout: *mut cuda_types::cudnn8::cudnnRNNDataLayout_t,
        maxSeqLength: *mut ::core::ffi::c_int,
        batchSize: *mut ::core::ffi::c_int,
        vectorSize: *mut ::core::ffi::c_int,
        arrayLengthRequested: ::core::ffi::c_int,
        seqLengthArray: *mut ::core::ffi::c_int,
        paddingFill: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnRNNForwardInferenceEx(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        xDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
        x: *const ::core::ffi::c_void,
        hxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        hx: *const ::core::ffi::c_void,
        cxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        cx: *const ::core::ffi::c_void,
        wDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        w: *const ::core::ffi::c_void,
        yDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
        y: *mut ::core::ffi::c_void,
        hyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        hy: *mut ::core::ffi::c_void,
        cyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        cy: *mut ::core::ffi::c_void,
        kDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
        keys: *const ::core::ffi::c_void,
        cDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
        cAttn: *mut ::core::ffi::c_void,
        iDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
        iAttn: *mut ::core::ffi::c_void,
        qDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
        queries: *mut ::core::ffi::c_void,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnRNNForward(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        fwdMode: cuda_types::cudnn8::cudnnForwardMode_t,
        devSeqLengths: *const i32,
        xDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
        x: *const ::core::ffi::c_void,
        yDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
        y: *mut ::core::ffi::c_void,
        hDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        hx: *const ::core::ffi::c_void,
        hy: *mut ::core::ffi::c_void,
        cDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        cx: *const ::core::ffi::c_void,
        cy: *mut ::core::ffi::c_void,
        weightSpaceSize: usize,
        weightSpace: *const ::core::ffi::c_void,
        workSpaceSize: usize,
        workSpace: *mut ::core::ffi::c_void,
        reserveSpaceSize: usize,
        reserveSpace: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetRNNAlgorithmDescriptor(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        algoDesc: cuda_types::cudnn8::cudnnAlgorithmDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetRNNForwardInferenceAlgorithmMaxCount(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        count: *mut ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnFindRNNForwardInferenceAlgorithmEx(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        seqLength: ::core::ffi::c_int,
        xDesc: *const cuda_types::cudnn8::cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        hxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        hx: *const ::core::ffi::c_void,
        cxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        cx: *const ::core::ffi::c_void,
        wDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        w: *const ::core::ffi::c_void,
        yDesc: *const cuda_types::cudnn8::cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        hyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        hy: *mut ::core::ffi::c_void,
        cyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        cy: *mut ::core::ffi::c_void,
        findIntensity: f32,
        requestedAlgoCount: ::core::ffi::c_int,
        returnedAlgoCount: *mut ::core::ffi::c_int,
        perfResults: *mut cuda_types::cudnn8::cudnnAlgorithmPerformance_t,
        workspace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnCreateSeqDataDescriptor(
        seqDataDesc: *mut cuda_types::cudnn8::cudnnSeqDataDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDestroySeqDataDescriptor(
        seqDataDesc: cuda_types::cudnn8::cudnnSeqDataDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetSeqDataDescriptor(
        seqDataDesc: cuda_types::cudnn8::cudnnSeqDataDescriptor_t,
        dataType: cuda_types::cudnn8::cudnnDataType_t,
        nbDims: ::core::ffi::c_int,
        dimA: *const ::core::ffi::c_int,
        axes: *const cuda_types::cudnn8::cudnnSeqDataAxis_t,
        seqLengthArraySize: usize,
        seqLengthArray: *const ::core::ffi::c_int,
        paddingFill: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetSeqDataDescriptor(
        seqDataDesc: cuda_types::cudnn8::cudnnSeqDataDescriptor_t,
        dataType: *mut cuda_types::cudnn8::cudnnDataType_t,
        nbDims: *mut ::core::ffi::c_int,
        nbDimsRequested: ::core::ffi::c_int,
        dimA: *mut ::core::ffi::c_int,
        axes: *mut cuda_types::cudnn8::cudnnSeqDataAxis_t,
        seqLengthArraySize: *mut usize,
        seqLengthSizeRequested: usize,
        seqLengthArray: *mut ::core::ffi::c_int,
        paddingFill: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnCreateAttnDescriptor(
        attnDesc: *mut cuda_types::cudnn8::cudnnAttnDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDestroyAttnDescriptor(
        attnDesc: cuda_types::cudnn8::cudnnAttnDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetAttnDescriptor(
        attnDesc: cuda_types::cudnn8::cudnnAttnDescriptor_t,
        attnMode: ::core::ffi::c_uint,
        nHeads: ::core::ffi::c_int,
        smScaler: f64,
        dataType: cuda_types::cudnn8::cudnnDataType_t,
        computePrec: cuda_types::cudnn8::cudnnDataType_t,
        mathType: cuda_types::cudnn8::cudnnMathType_t,
        attnDropoutDesc: cuda_types::cudnn8::cudnnDropoutDescriptor_t,
        postDropoutDesc: cuda_types::cudnn8::cudnnDropoutDescriptor_t,
        qSize: ::core::ffi::c_int,
        kSize: ::core::ffi::c_int,
        vSize: ::core::ffi::c_int,
        qProjSize: ::core::ffi::c_int,
        kProjSize: ::core::ffi::c_int,
        vProjSize: ::core::ffi::c_int,
        oProjSize: ::core::ffi::c_int,
        qoMaxSeqLength: ::core::ffi::c_int,
        kvMaxSeqLength: ::core::ffi::c_int,
        maxBatchSize: ::core::ffi::c_int,
        maxBeamSize: ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetAttnDescriptor(
        attnDesc: cuda_types::cudnn8::cudnnAttnDescriptor_t,
        attnMode: *mut ::core::ffi::c_uint,
        nHeads: *mut ::core::ffi::c_int,
        smScaler: *mut f64,
        dataType: *mut cuda_types::cudnn8::cudnnDataType_t,
        computePrec: *mut cuda_types::cudnn8::cudnnDataType_t,
        mathType: *mut cuda_types::cudnn8::cudnnMathType_t,
        attnDropoutDesc: *mut cuda_types::cudnn8::cudnnDropoutDescriptor_t,
        postDropoutDesc: *mut cuda_types::cudnn8::cudnnDropoutDescriptor_t,
        qSize: *mut ::core::ffi::c_int,
        kSize: *mut ::core::ffi::c_int,
        vSize: *mut ::core::ffi::c_int,
        qProjSize: *mut ::core::ffi::c_int,
        kProjSize: *mut ::core::ffi::c_int,
        vProjSize: *mut ::core::ffi::c_int,
        oProjSize: *mut ::core::ffi::c_int,
        qoMaxSeqLength: *mut ::core::ffi::c_int,
        kvMaxSeqLength: *mut ::core::ffi::c_int,
        maxBatchSize: *mut ::core::ffi::c_int,
        maxBeamSize: *mut ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetMultiHeadAttnBuffers(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        attnDesc: cuda_types::cudnn8::cudnnAttnDescriptor_t,
        weightSizeInBytes: *mut usize,
        workSpaceSizeInBytes: *mut usize,
        reserveSpaceSizeInBytes: *mut usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetMultiHeadAttnWeights(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        attnDesc: cuda_types::cudnn8::cudnnAttnDescriptor_t,
        wKind: cuda_types::cudnn8::cudnnMultiHeadAttnWeightKind_t,
        weightSizeInBytes: usize,
        weights: *const ::core::ffi::c_void,
        wDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        wAddr: *mut *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnMultiHeadAttnForward(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        attnDesc: cuda_types::cudnn8::cudnnAttnDescriptor_t,
        currIdx: ::core::ffi::c_int,
        loWinIdx: *const ::core::ffi::c_int,
        hiWinIdx: *const ::core::ffi::c_int,
        devSeqLengthsQO: *const ::core::ffi::c_int,
        devSeqLengthsKV: *const ::core::ffi::c_int,
        qDesc: cuda_types::cudnn8::cudnnSeqDataDescriptor_t,
        queries: *const ::core::ffi::c_void,
        residuals: *const ::core::ffi::c_void,
        kDesc: cuda_types::cudnn8::cudnnSeqDataDescriptor_t,
        keys: *const ::core::ffi::c_void,
        vDesc: cuda_types::cudnn8::cudnnSeqDataDescriptor_t,
        values: *const ::core::ffi::c_void,
        oDesc: cuda_types::cudnn8::cudnnSeqDataDescriptor_t,
        out: *mut ::core::ffi::c_void,
        weightSizeInBytes: usize,
        weights: *const ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        workSpace: *mut ::core::ffi::c_void,
        reserveSpaceSizeInBytes: usize,
        reserveSpace: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnAdvInferVersionCheck() -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnRNNForwardTraining(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        seqLength: ::core::ffi::c_int,
        xDesc: *const cuda_types::cudnn8::cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        hxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        hx: *const ::core::ffi::c_void,
        cxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        cx: *const ::core::ffi::c_void,
        wDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        w: *const ::core::ffi::c_void,
        yDesc: *const cuda_types::cudnn8::cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        hyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        hy: *mut ::core::ffi::c_void,
        cyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        cy: *mut ::core::ffi::c_void,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        reserveSpace: *mut ::core::ffi::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnRNNBackwardData(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        seqLength: ::core::ffi::c_int,
        yDesc: *const cuda_types::cudnn8::cudnnTensorDescriptor_t,
        y: *const ::core::ffi::c_void,
        dyDesc: *const cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        dhyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dhy: *const ::core::ffi::c_void,
        dcyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dcy: *const ::core::ffi::c_void,
        wDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        w: *const ::core::ffi::c_void,
        hxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        hx: *const ::core::ffi::c_void,
        cxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        cx: *const ::core::ffi::c_void,
        dxDesc: *const cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
        dhxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dhx: *mut ::core::ffi::c_void,
        dcxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dcx: *mut ::core::ffi::c_void,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        reserveSpace: *mut ::core::ffi::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnRNNBackwardData_v8(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        devSeqLengths: *const i32,
        yDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
        y: *const ::core::ffi::c_void,
        dy: *const ::core::ffi::c_void,
        xDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
        dx: *mut ::core::ffi::c_void,
        hDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        hx: *const ::core::ffi::c_void,
        dhy: *const ::core::ffi::c_void,
        dhx: *mut ::core::ffi::c_void,
        cDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        cx: *const ::core::ffi::c_void,
        dcy: *const ::core::ffi::c_void,
        dcx: *mut ::core::ffi::c_void,
        weightSpaceSize: usize,
        weightSpace: *const ::core::ffi::c_void,
        workSpaceSize: usize,
        workSpace: *mut ::core::ffi::c_void,
        reserveSpaceSize: usize,
        reserveSpace: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnRNNBackwardWeights(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        seqLength: ::core::ffi::c_int,
        xDesc: *const cuda_types::cudnn8::cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        hxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        hx: *const ::core::ffi::c_void,
        yDesc: *const cuda_types::cudnn8::cudnnTensorDescriptor_t,
        y: *const ::core::ffi::c_void,
        workSpace: *const ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        dwDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        dw: *mut ::core::ffi::c_void,
        reserveSpace: *const ::core::ffi::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnRNNBackwardWeights_v8(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        addGrad: cuda_types::cudnn8::cudnnWgradMode_t,
        devSeqLengths: *const i32,
        xDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
        x: *const ::core::ffi::c_void,
        hDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        hx: *const ::core::ffi::c_void,
        yDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
        y: *const ::core::ffi::c_void,
        weightSpaceSize: usize,
        dweightSpace: *mut ::core::ffi::c_void,
        workSpaceSize: usize,
        workSpace: *mut ::core::ffi::c_void,
        reserveSpaceSize: usize,
        reserveSpace: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnRNNForwardTrainingEx(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        xDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
        x: *const ::core::ffi::c_void,
        hxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        hx: *const ::core::ffi::c_void,
        cxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        cx: *const ::core::ffi::c_void,
        wDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        w: *const ::core::ffi::c_void,
        yDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
        y: *mut ::core::ffi::c_void,
        hyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        hy: *mut ::core::ffi::c_void,
        cyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        cy: *mut ::core::ffi::c_void,
        kDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
        keys: *const ::core::ffi::c_void,
        cDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
        cAttn: *mut ::core::ffi::c_void,
        iDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
        iAttn: *mut ::core::ffi::c_void,
        qDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
        queries: *mut ::core::ffi::c_void,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        reserveSpace: *mut ::core::ffi::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnRNNBackwardDataEx(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        yDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
        y: *const ::core::ffi::c_void,
        dyDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
        dy: *const ::core::ffi::c_void,
        dcDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
        dcAttn: *const ::core::ffi::c_void,
        dhyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dhy: *const ::core::ffi::c_void,
        dcyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dcy: *const ::core::ffi::c_void,
        wDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        w: *const ::core::ffi::c_void,
        hxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        hx: *const ::core::ffi::c_void,
        cxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        cx: *const ::core::ffi::c_void,
        dxDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
        dx: *mut ::core::ffi::c_void,
        dhxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dhx: *mut ::core::ffi::c_void,
        dcxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dcx: *mut ::core::ffi::c_void,
        dkDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
        dkeys: *mut ::core::ffi::c_void,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        reserveSpace: *mut ::core::ffi::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnRNNBackwardWeightsEx(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        xDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
        x: *const ::core::ffi::c_void,
        hxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        hx: *const ::core::ffi::c_void,
        yDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
        y: *const ::core::ffi::c_void,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        dwDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        dw: *mut ::core::ffi::c_void,
        reserveSpace: *mut ::core::ffi::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetRNNForwardTrainingAlgorithmMaxCount(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        count: *mut ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnFindRNNForwardTrainingAlgorithmEx(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        seqLength: ::core::ffi::c_int,
        xDesc: *const cuda_types::cudnn8::cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        hxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        hx: *const ::core::ffi::c_void,
        cxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        cx: *const ::core::ffi::c_void,
        wDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        w: *const ::core::ffi::c_void,
        yDesc: *const cuda_types::cudnn8::cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        hyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        hy: *mut ::core::ffi::c_void,
        cyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        cy: *mut ::core::ffi::c_void,
        findIntensity: f32,
        requestedAlgoCount: ::core::ffi::c_int,
        returnedAlgoCount: *mut ::core::ffi::c_int,
        perfResults: *mut cuda_types::cudnn8::cudnnAlgorithmPerformance_t,
        workspace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        reserveSpace: *mut ::core::ffi::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetRNNBackwardDataAlgorithmMaxCount(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        count: *mut ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnFindRNNBackwardDataAlgorithmEx(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        seqLength: ::core::ffi::c_int,
        yDesc: *const cuda_types::cudnn8::cudnnTensorDescriptor_t,
        y: *const ::core::ffi::c_void,
        dyDesc: *const cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        dhyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dhy: *const ::core::ffi::c_void,
        dcyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dcy: *const ::core::ffi::c_void,
        wDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        w: *const ::core::ffi::c_void,
        hxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        hx: *const ::core::ffi::c_void,
        cxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        cx: *const ::core::ffi::c_void,
        dxDesc: *const cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
        dhxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dhx: *mut ::core::ffi::c_void,
        dcxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dcx: *mut ::core::ffi::c_void,
        findIntensity: f32,
        requestedAlgoCount: ::core::ffi::c_int,
        returnedAlgoCount: *mut ::core::ffi::c_int,
        perfResults: *mut cuda_types::cudnn8::cudnnAlgorithmPerformance_t,
        workspace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        reserveSpace: *mut ::core::ffi::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetRNNBackwardWeightsAlgorithmMaxCount(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        count: *mut ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnFindRNNBackwardWeightsAlgorithmEx(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
        seqLength: ::core::ffi::c_int,
        xDesc: *const cuda_types::cudnn8::cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        hxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        hx: *const ::core::ffi::c_void,
        yDesc: *const cuda_types::cudnn8::cudnnTensorDescriptor_t,
        y: *const ::core::ffi::c_void,
        findIntensity: f32,
        requestedAlgoCount: ::core::ffi::c_int,
        returnedAlgoCount: *mut ::core::ffi::c_int,
        perfResults: *mut cuda_types::cudnn8::cudnnAlgorithmPerformance_t,
        workspace: *const ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        dwDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        dw: *mut ::core::ffi::c_void,
        reserveSpace: *const ::core::ffi::c_void,
        reserveSpaceSizeInBytes: usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnMultiHeadAttnBackwardData(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        attnDesc: cuda_types::cudnn8::cudnnAttnDescriptor_t,
        loWinIdx: *const ::core::ffi::c_int,
        hiWinIdx: *const ::core::ffi::c_int,
        devSeqLengthsDQDO: *const ::core::ffi::c_int,
        devSeqLengthsDKDV: *const ::core::ffi::c_int,
        doDesc: cuda_types::cudnn8::cudnnSeqDataDescriptor_t,
        dout: *const ::core::ffi::c_void,
        dqDesc: cuda_types::cudnn8::cudnnSeqDataDescriptor_t,
        dqueries: *mut ::core::ffi::c_void,
        queries: *const ::core::ffi::c_void,
        dkDesc: cuda_types::cudnn8::cudnnSeqDataDescriptor_t,
        dkeys: *mut ::core::ffi::c_void,
        keys: *const ::core::ffi::c_void,
        dvDesc: cuda_types::cudnn8::cudnnSeqDataDescriptor_t,
        dvalues: *mut ::core::ffi::c_void,
        values: *const ::core::ffi::c_void,
        weightSizeInBytes: usize,
        weights: *const ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        workSpace: *mut ::core::ffi::c_void,
        reserveSpaceSizeInBytes: usize,
        reserveSpace: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnMultiHeadAttnBackwardWeights(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        attnDesc: cuda_types::cudnn8::cudnnAttnDescriptor_t,
        addGrad: cuda_types::cudnn8::cudnnWgradMode_t,
        qDesc: cuda_types::cudnn8::cudnnSeqDataDescriptor_t,
        queries: *const ::core::ffi::c_void,
        kDesc: cuda_types::cudnn8::cudnnSeqDataDescriptor_t,
        keys: *const ::core::ffi::c_void,
        vDesc: cuda_types::cudnn8::cudnnSeqDataDescriptor_t,
        values: *const ::core::ffi::c_void,
        doDesc: cuda_types::cudnn8::cudnnSeqDataDescriptor_t,
        dout: *const ::core::ffi::c_void,
        weightSizeInBytes: usize,
        weights: *const ::core::ffi::c_void,
        dweights: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        workSpace: *mut ::core::ffi::c_void,
        reserveSpaceSizeInBytes: usize,
        reserveSpace: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnCreateCTCLossDescriptor(
        ctcLossDesc: *mut cuda_types::cudnn8::cudnnCTCLossDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetCTCLossDescriptor(
        ctcLossDesc: cuda_types::cudnn8::cudnnCTCLossDescriptor_t,
        compType: cuda_types::cudnn8::cudnnDataType_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetCTCLossDescriptorEx(
        ctcLossDesc: cuda_types::cudnn8::cudnnCTCLossDescriptor_t,
        compType: cuda_types::cudnn8::cudnnDataType_t,
        normMode: cuda_types::cudnn8::cudnnLossNormalizationMode_t,
        gradMode: cuda_types::cudnn8::cudnnNanPropagation_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetCTCLossDescriptor_v8(
        ctcLossDesc: cuda_types::cudnn8::cudnnCTCLossDescriptor_t,
        compType: cuda_types::cudnn8::cudnnDataType_t,
        normMode: cuda_types::cudnn8::cudnnLossNormalizationMode_t,
        gradMode: cuda_types::cudnn8::cudnnNanPropagation_t,
        maxLabelLength: ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetCTCLossDescriptor(
        ctcLossDesc: cuda_types::cudnn8::cudnnCTCLossDescriptor_t,
        compType: *mut cuda_types::cudnn8::cudnnDataType_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetCTCLossDescriptorEx(
        ctcLossDesc: cuda_types::cudnn8::cudnnCTCLossDescriptor_t,
        compType: *mut cuda_types::cudnn8::cudnnDataType_t,
        normMode: *mut cuda_types::cudnn8::cudnnLossNormalizationMode_t,
        gradMode: *mut cuda_types::cudnn8::cudnnNanPropagation_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetCTCLossDescriptor_v8(
        ctcLossDesc: cuda_types::cudnn8::cudnnCTCLossDescriptor_t,
        compType: *mut cuda_types::cudnn8::cudnnDataType_t,
        normMode: *mut cuda_types::cudnn8::cudnnLossNormalizationMode_t,
        gradMode: *mut cuda_types::cudnn8::cudnnNanPropagation_t,
        maxLabelLength: *mut ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDestroyCTCLossDescriptor(
        ctcLossDesc: cuda_types::cudnn8::cudnnCTCLossDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnCTCLoss(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        probsDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        probs: *const ::core::ffi::c_void,
        hostLabels: *const ::core::ffi::c_int,
        hostLabelLengths: *const ::core::ffi::c_int,
        hostInputLengths: *const ::core::ffi::c_int,
        costs: *mut ::core::ffi::c_void,
        gradientsDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        gradients: *mut ::core::ffi::c_void,
        algo: cuda_types::cudnn8::cudnnCTCLossAlgo_t,
        ctcLossDesc: cuda_types::cudnn8::cudnnCTCLossDescriptor_t,
        workspace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnCTCLoss_v8(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        algo: cuda_types::cudnn8::cudnnCTCLossAlgo_t,
        ctcLossDesc: cuda_types::cudnn8::cudnnCTCLossDescriptor_t,
        probsDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        probs: *const ::core::ffi::c_void,
        labels: *const ::core::ffi::c_int,
        labelLengths: *const ::core::ffi::c_int,
        inputLengths: *const ::core::ffi::c_int,
        costs: *mut ::core::ffi::c_void,
        gradientsDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        gradients: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        workspace: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetCTCLossWorkspaceSize(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        probsDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        gradientsDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        labels: *const ::core::ffi::c_int,
        labelLengths: *const ::core::ffi::c_int,
        inputLengths: *const ::core::ffi::c_int,
        algo: cuda_types::cudnn8::cudnnCTCLossAlgo_t,
        ctcLossDesc: cuda_types::cudnn8::cudnnCTCLossDescriptor_t,
        sizeInBytes: *mut usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetCTCLossWorkspaceSize_v8(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        algo: cuda_types::cudnn8::cudnnCTCLossAlgo_t,
        ctcLossDesc: cuda_types::cudnn8::cudnnCTCLossDescriptor_t,
        probsDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        gradientsDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        sizeInBytes: *mut usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnAdvTrainVersionCheck() -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnCreateConvolutionDescriptor(
        convDesc: *mut cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDestroyConvolutionDescriptor(
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetConvolutionMathType(
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        mathType: cuda_types::cudnn8::cudnnMathType_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetConvolutionMathType(
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        mathType: *mut cuda_types::cudnn8::cudnnMathType_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetConvolutionGroupCount(
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        groupCount: ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetConvolutionGroupCount(
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        groupCount: *mut ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetConvolutionReorderType(
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        reorderType: cuda_types::cudnn8::cudnnReorderType_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetConvolutionReorderType(
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        reorderType: *mut cuda_types::cudnn8::cudnnReorderType_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetConvolution2dDescriptor(
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        pad_h: ::core::ffi::c_int,
        pad_w: ::core::ffi::c_int,
        u: ::core::ffi::c_int,
        v: ::core::ffi::c_int,
        dilation_h: ::core::ffi::c_int,
        dilation_w: ::core::ffi::c_int,
        mode: cuda_types::cudnn8::cudnnConvolutionMode_t,
        computeType: cuda_types::cudnn8::cudnnDataType_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetConvolution2dDescriptor(
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        pad_h: *mut ::core::ffi::c_int,
        pad_w: *mut ::core::ffi::c_int,
        u: *mut ::core::ffi::c_int,
        v: *mut ::core::ffi::c_int,
        dilation_h: *mut ::core::ffi::c_int,
        dilation_w: *mut ::core::ffi::c_int,
        mode: *mut cuda_types::cudnn8::cudnnConvolutionMode_t,
        computeType: *mut cuda_types::cudnn8::cudnnDataType_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetConvolutionNdDescriptor(
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        arrayLength: ::core::ffi::c_int,
        padA: *const ::core::ffi::c_int,
        filterStrideA: *const ::core::ffi::c_int,
        dilationA: *const ::core::ffi::c_int,
        mode: cuda_types::cudnn8::cudnnConvolutionMode_t,
        computeType: cuda_types::cudnn8::cudnnDataType_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetConvolutionNdDescriptor(
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        arrayLengthRequested: ::core::ffi::c_int,
        arrayLength: *mut ::core::ffi::c_int,
        padA: *mut ::core::ffi::c_int,
        strideA: *mut ::core::ffi::c_int,
        dilationA: *mut ::core::ffi::c_int,
        mode: *mut cuda_types::cudnn8::cudnnConvolutionMode_t,
        computeType: *mut cuda_types::cudnn8::cudnnDataType_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetConvolution2dForwardOutputDim(
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        inputTensorDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        filterDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        n: *mut ::core::ffi::c_int,
        c: *mut ::core::ffi::c_int,
        h: *mut ::core::ffi::c_int,
        w: *mut ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetConvolutionNdForwardOutputDim(
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        inputTensorDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        filterDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        nbDims: ::core::ffi::c_int,
        tensorOuputDimA: *mut ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetConvolutionForwardAlgorithmMaxCount(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        count: *mut ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetConvolutionForwardAlgorithm_v7(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        srcDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        filterDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        destDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        requestedAlgoCount: ::core::ffi::c_int,
        returnedAlgoCount: *mut ::core::ffi::c_int,
        perfResults: *mut cuda_types::cudnn8::cudnnConvolutionFwdAlgoPerf_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnFindConvolutionForwardAlgorithm(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        wDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        requestedAlgoCount: ::core::ffi::c_int,
        returnedAlgoCount: *mut ::core::ffi::c_int,
        perfResults: *mut cuda_types::cudnn8::cudnnConvolutionFwdAlgoPerf_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnFindConvolutionForwardAlgorithmEx(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        wDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        w: *const ::core::ffi::c_void,
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
        requestedAlgoCount: ::core::ffi::c_int,
        returnedAlgoCount: *mut ::core::ffi::c_int,
        perfResults: *mut cuda_types::cudnn8::cudnnConvolutionFwdAlgoPerf_t,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnIm2Col(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        wDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        colBuffer: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnReorderFilterAndBias(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        filterDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        reorderType: cuda_types::cudnn8::cudnnReorderType_t,
        filterData: *const ::core::ffi::c_void,
        reorderedFilterData: *mut ::core::ffi::c_void,
        reorderBias: ::core::ffi::c_int,
        biasData: *const ::core::ffi::c_void,
        reorderedBiasData: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetConvolutionForwardWorkspaceSize(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        wDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        algo: cuda_types::cudnn8::cudnnConvolutionFwdAlgo_t,
        sizeInBytes: *mut usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnConvolutionForward(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        alpha: *const ::core::ffi::c_void,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        wDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        w: *const ::core::ffi::c_void,
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        algo: cuda_types::cudnn8::cudnnConvolutionFwdAlgo_t,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        beta: *const ::core::ffi::c_void,
        yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnConvolutionBiasActivationForward(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        alpha1: *const ::core::ffi::c_void,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        wDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        w: *const ::core::ffi::c_void,
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        algo: cuda_types::cudnn8::cudnnConvolutionFwdAlgo_t,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        alpha2: *const ::core::ffi::c_void,
        zDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        z: *const ::core::ffi::c_void,
        biasDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        bias: *const ::core::ffi::c_void,
        activationDesc: cuda_types::cudnn8::cudnnActivationDescriptor_t,
        yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        y: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetConvolutionBackwardDataAlgorithmMaxCount(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        count: *mut ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnFindConvolutionBackwardDataAlgorithm(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        wDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        dyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        dxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        requestedAlgoCount: ::core::ffi::c_int,
        returnedAlgoCount: *mut ::core::ffi::c_int,
        perfResults: *mut cuda_types::cudnn8::cudnnConvolutionBwdDataAlgoPerf_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnFindConvolutionBackwardDataAlgorithmEx(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        wDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        w: *const ::core::ffi::c_void,
        dyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        dxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
        requestedAlgoCount: ::core::ffi::c_int,
        returnedAlgoCount: *mut ::core::ffi::c_int,
        perfResults: *mut cuda_types::cudnn8::cudnnConvolutionBwdDataAlgoPerf_t,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetConvolutionBackwardDataAlgorithm_v7(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        filterDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        diffDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        gradDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        requestedAlgoCount: ::core::ffi::c_int,
        returnedAlgoCount: *mut ::core::ffi::c_int,
        perfResults: *mut cuda_types::cudnn8::cudnnConvolutionBwdDataAlgoPerf_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetConvolutionBackwardDataWorkspaceSize(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        wDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        dyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        dxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        algo: cuda_types::cudnn8::cudnnConvolutionBwdDataAlgo_t,
        sizeInBytes: *mut usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnConvolutionBackwardData(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        alpha: *const ::core::ffi::c_void,
        wDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        w: *const ::core::ffi::c_void,
        dyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        algo: cuda_types::cudnn8::cudnnConvolutionBwdDataAlgo_t,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        beta: *const ::core::ffi::c_void,
        dxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dx: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetFoldedConvBackwardDataDescriptors(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        filterDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        diffDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        gradDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        transformFormat: cuda_types::cudnn8::cudnnTensorFormat_t,
        foldedFilterDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        paddedDiffDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        foldedConvDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        foldedGradDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        filterFoldTransDesc: cuda_types::cudnn8::cudnnTensorTransformDescriptor_t,
        diffPadTransDesc: cuda_types::cudnn8::cudnnTensorTransformDescriptor_t,
        gradFoldTransDesc: cuda_types::cudnn8::cudnnTensorTransformDescriptor_t,
        gradUnfoldTransDesc: cuda_types::cudnn8::cudnnTensorTransformDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnCnnInferVersionCheck() -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetConvolutionBackwardFilterAlgorithmMaxCount(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        count: *mut ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnFindConvolutionBackwardFilterAlgorithm(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        dwDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        requestedAlgoCount: ::core::ffi::c_int,
        returnedAlgoCount: *mut ::core::ffi::c_int,
        perfResults: *mut cuda_types::cudnn8::cudnnConvolutionBwdFilterAlgoPerf_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnFindConvolutionBackwardFilterAlgorithmEx(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        dyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        y: *const ::core::ffi::c_void,
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        dwDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        dw: *mut ::core::ffi::c_void,
        requestedAlgoCount: ::core::ffi::c_int,
        returnedAlgoCount: *mut ::core::ffi::c_int,
        perfResults: *mut cuda_types::cudnn8::cudnnConvolutionBwdFilterAlgoPerf_t,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetConvolutionBackwardFilterAlgorithm_v7(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        srcDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        diffDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        gradDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        requestedAlgoCount: ::core::ffi::c_int,
        returnedAlgoCount: *mut ::core::ffi::c_int,
        perfResults: *mut cuda_types::cudnn8::cudnnConvolutionBwdFilterAlgoPerf_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetConvolutionBackwardFilterWorkspaceSize(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        gradDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        algo: cuda_types::cudnn8::cudnnConvolutionBwdFilterAlgo_t,
        sizeInBytes: *mut usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnConvolutionBackwardFilter(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        alpha: *const ::core::ffi::c_void,
        xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        x: *const ::core::ffi::c_void,
        dyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
        algo: cuda_types::cudnn8::cudnnConvolutionBwdFilterAlgo_t,
        workSpace: *mut ::core::ffi::c_void,
        workSpaceSizeInBytes: usize,
        beta: *const ::core::ffi::c_void,
        dwDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
        dw: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnConvolutionBackwardBias(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        alpha: *const ::core::ffi::c_void,
        dyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        dy: *const ::core::ffi::c_void,
        beta: *const ::core::ffi::c_void,
        dbDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
        db: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnCreateFusedOpsConstParamPack(
        constPack: *mut cuda_types::cudnn8::cudnnFusedOpsConstParamPack_t,
        ops: cuda_types::cudnn8::cudnnFusedOps_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDestroyFusedOpsConstParamPack(
        constPack: cuda_types::cudnn8::cudnnFusedOpsConstParamPack_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetFusedOpsConstParamPackAttribute(
        constPack: cuda_types::cudnn8::cudnnFusedOpsConstParamPack_t,
        paramLabel: cuda_types::cudnn8::cudnnFusedOpsConstParamLabel_t,
        param: *const ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetFusedOpsConstParamPackAttribute(
        constPack: cuda_types::cudnn8::cudnnFusedOpsConstParamPack_t,
        paramLabel: cuda_types::cudnn8::cudnnFusedOpsConstParamLabel_t,
        param: *mut ::core::ffi::c_void,
        isNULL: *mut ::core::ffi::c_int,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnCreateFusedOpsVariantParamPack(
        varPack: *mut cuda_types::cudnn8::cudnnFusedOpsVariantParamPack_t,
        ops: cuda_types::cudnn8::cudnnFusedOps_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDestroyFusedOpsVariantParamPack(
        varPack: cuda_types::cudnn8::cudnnFusedOpsVariantParamPack_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnSetFusedOpsVariantParamPackAttribute(
        varPack: cuda_types::cudnn8::cudnnFusedOpsVariantParamPack_t,
        paramLabel: cuda_types::cudnn8::cudnnFusedOpsVariantParamLabel_t,
        ptr: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnGetFusedOpsVariantParamPackAttribute(
        varPack: cuda_types::cudnn8::cudnnFusedOpsVariantParamPack_t,
        paramLabel: cuda_types::cudnn8::cudnnFusedOpsVariantParamLabel_t,
        ptr: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnCreateFusedOpsPlan(
        plan: *mut cuda_types::cudnn8::cudnnFusedOpsPlan_t,
        ops: cuda_types::cudnn8::cudnnFusedOps_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnDestroyFusedOpsPlan(
        plan: cuda_types::cudnn8::cudnnFusedOpsPlan_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnMakeFusedOpsPlan(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        plan: cuda_types::cudnn8::cudnnFusedOpsPlan_t,
        constPack: cuda_types::cudnn8::cudnnFusedOpsConstParamPack_t,
        workspaceSizeInBytes: *mut usize,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnFusedOpsExecute(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        plan: cuda_types::cudnn8::cudnnFusedOpsPlan_t,
        varPack: cuda_types::cudnn8::cudnnFusedOpsVariantParamPack_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnCnnTrainVersionCheck() -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnBackendCreateDescriptor(
        descriptorType: cuda_types::cudnn8::cudnnBackendDescriptorType_t,
        descriptor: *mut cuda_types::cudnn8::cudnnBackendDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnBackendDestroyDescriptor(
        descriptor: cuda_types::cudnn8::cudnnBackendDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnBackendInitialize(
        descriptor: cuda_types::cudnn8::cudnnBackendDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnBackendFinalize(
        descriptor: cuda_types::cudnn8::cudnnBackendDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnBackendSetAttribute(
        descriptor: cuda_types::cudnn8::cudnnBackendDescriptor_t,
        attributeName: cuda_types::cudnn8::cudnnBackendAttributeName_t,
        attributeType: cuda_types::cudnn8::cudnnBackendAttributeType_t,
        elementCount: i64,
        arrayOfElements: *const ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnBackendGetAttribute(
        descriptor: cuda_types::cudnn8::cudnnBackendDescriptor_t,
        attributeName: cuda_types::cudnn8::cudnnBackendAttributeName_t,
        attributeType: cuda_types::cudnn8::cudnnBackendAttributeType_t,
        requestedElementCount: i64,
        elementCount: *mut i64,
        arrayOfElements: *mut ::core::ffi::c_void,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
    #[must_use]
    fn cudnnBackendExecute(
        handle: cuda_types::cudnn8::cudnnHandle_t,
        executionPlan: cuda_types::cudnn8::cudnnBackendDescriptor_t,
        variantPack: cuda_types::cudnn8::cudnnBackendDescriptor_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t;
}
