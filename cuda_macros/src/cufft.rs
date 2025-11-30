// Generated automatically by zluda_bindgen
// DO NOT EDIT MANUALLY
#![allow(warnings)]
extern "system" {
    fn cufftPlan1d(
        plan: *mut cuda_types::cufft::cufftHandle,
        nx: ::core::ffi::c_int,
        type_: cuda_types::cufft::cufftType,
        batch: ::core::ffi::c_int,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftPlan2d(
        plan: *mut cuda_types::cufft::cufftHandle,
        nx: ::core::ffi::c_int,
        ny: ::core::ffi::c_int,
        type_: cuda_types::cufft::cufftType,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftPlan3d(
        plan: *mut cuda_types::cufft::cufftHandle,
        nx: ::core::ffi::c_int,
        ny: ::core::ffi::c_int,
        nz: ::core::ffi::c_int,
        type_: cuda_types::cufft::cufftType,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftPlanMany(
        plan: *mut cuda_types::cufft::cufftHandle,
        rank: ::core::ffi::c_int,
        n: *mut ::core::ffi::c_int,
        inembed: *mut ::core::ffi::c_int,
        istride: ::core::ffi::c_int,
        idist: ::core::ffi::c_int,
        onembed: *mut ::core::ffi::c_int,
        ostride: ::core::ffi::c_int,
        odist: ::core::ffi::c_int,
        type_: cuda_types::cufft::cufftType,
        batch: ::core::ffi::c_int,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftMakePlan1d(
        plan: cuda_types::cufft::cufftHandle,
        nx: ::core::ffi::c_int,
        type_: cuda_types::cufft::cufftType,
        batch: ::core::ffi::c_int,
        workSize: *mut usize,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftMakePlan2d(
        plan: cuda_types::cufft::cufftHandle,
        nx: ::core::ffi::c_int,
        ny: ::core::ffi::c_int,
        type_: cuda_types::cufft::cufftType,
        workSize: *mut usize,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftMakePlan3d(
        plan: cuda_types::cufft::cufftHandle,
        nx: ::core::ffi::c_int,
        ny: ::core::ffi::c_int,
        nz: ::core::ffi::c_int,
        type_: cuda_types::cufft::cufftType,
        workSize: *mut usize,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftMakePlanMany(
        plan: cuda_types::cufft::cufftHandle,
        rank: ::core::ffi::c_int,
        n: *mut ::core::ffi::c_int,
        inembed: *mut ::core::ffi::c_int,
        istride: ::core::ffi::c_int,
        idist: ::core::ffi::c_int,
        onembed: *mut ::core::ffi::c_int,
        ostride: ::core::ffi::c_int,
        odist: ::core::ffi::c_int,
        type_: cuda_types::cufft::cufftType,
        batch: ::core::ffi::c_int,
        workSize: *mut usize,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftMakePlanMany64(
        plan: cuda_types::cufft::cufftHandle,
        rank: ::core::ffi::c_int,
        n: *mut ::core::ffi::c_longlong,
        inembed: *mut ::core::ffi::c_longlong,
        istride: ::core::ffi::c_longlong,
        idist: ::core::ffi::c_longlong,
        onembed: *mut ::core::ffi::c_longlong,
        ostride: ::core::ffi::c_longlong,
        odist: ::core::ffi::c_longlong,
        type_: cuda_types::cufft::cufftType,
        batch: ::core::ffi::c_longlong,
        workSize: *mut usize,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftGetSizeMany64(
        plan: cuda_types::cufft::cufftHandle,
        rank: ::core::ffi::c_int,
        n: *mut ::core::ffi::c_longlong,
        inembed: *mut ::core::ffi::c_longlong,
        istride: ::core::ffi::c_longlong,
        idist: ::core::ffi::c_longlong,
        onembed: *mut ::core::ffi::c_longlong,
        ostride: ::core::ffi::c_longlong,
        odist: ::core::ffi::c_longlong,
        type_: cuda_types::cufft::cufftType,
        batch: ::core::ffi::c_longlong,
        workSize: *mut usize,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftEstimate1d(
        nx: ::core::ffi::c_int,
        type_: cuda_types::cufft::cufftType,
        batch: ::core::ffi::c_int,
        workSize: *mut usize,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftEstimate2d(
        nx: ::core::ffi::c_int,
        ny: ::core::ffi::c_int,
        type_: cuda_types::cufft::cufftType,
        workSize: *mut usize,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftEstimate3d(
        nx: ::core::ffi::c_int,
        ny: ::core::ffi::c_int,
        nz: ::core::ffi::c_int,
        type_: cuda_types::cufft::cufftType,
        workSize: *mut usize,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftEstimateMany(
        rank: ::core::ffi::c_int,
        n: *mut ::core::ffi::c_int,
        inembed: *mut ::core::ffi::c_int,
        istride: ::core::ffi::c_int,
        idist: ::core::ffi::c_int,
        onembed: *mut ::core::ffi::c_int,
        ostride: ::core::ffi::c_int,
        odist: ::core::ffi::c_int,
        type_: cuda_types::cufft::cufftType,
        batch: ::core::ffi::c_int,
        workSize: *mut usize,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftCreate(
        handle: *mut cuda_types::cufft::cufftHandle,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftGetSize1d(
        handle: cuda_types::cufft::cufftHandle,
        nx: ::core::ffi::c_int,
        type_: cuda_types::cufft::cufftType,
        batch: ::core::ffi::c_int,
        workSize: *mut usize,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftGetSize2d(
        handle: cuda_types::cufft::cufftHandle,
        nx: ::core::ffi::c_int,
        ny: ::core::ffi::c_int,
        type_: cuda_types::cufft::cufftType,
        workSize: *mut usize,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftGetSize3d(
        handle: cuda_types::cufft::cufftHandle,
        nx: ::core::ffi::c_int,
        ny: ::core::ffi::c_int,
        nz: ::core::ffi::c_int,
        type_: cuda_types::cufft::cufftType,
        workSize: *mut usize,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftGetSizeMany(
        handle: cuda_types::cufft::cufftHandle,
        rank: ::core::ffi::c_int,
        n: *mut ::core::ffi::c_int,
        inembed: *mut ::core::ffi::c_int,
        istride: ::core::ffi::c_int,
        idist: ::core::ffi::c_int,
        onembed: *mut ::core::ffi::c_int,
        ostride: ::core::ffi::c_int,
        odist: ::core::ffi::c_int,
        type_: cuda_types::cufft::cufftType,
        batch: ::core::ffi::c_int,
        workArea: *mut usize,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftGetSize(
        handle: cuda_types::cufft::cufftHandle,
        workSize: *mut usize,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftSetWorkArea(
        plan: cuda_types::cufft::cufftHandle,
        workArea: *mut ::core::ffi::c_void,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftSetAutoAllocation(
        plan: cuda_types::cufft::cufftHandle,
        autoAllocate: ::core::ffi::c_int,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftExecC2C(
        plan: cuda_types::cufft::cufftHandle,
        idata: *mut cuda_types::cufft::cufftComplex,
        odata: *mut cuda_types::cufft::cufftComplex,
        direction: ::core::ffi::c_int,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftExecR2C(
        plan: cuda_types::cufft::cufftHandle,
        idata: *mut cuda_types::cufft::cufftReal,
        odata: *mut cuda_types::cufft::cufftComplex,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftExecC2R(
        plan: cuda_types::cufft::cufftHandle,
        idata: *mut cuda_types::cufft::cufftComplex,
        odata: *mut cuda_types::cufft::cufftReal,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftExecZ2Z(
        plan: cuda_types::cufft::cufftHandle,
        idata: *mut cuda_types::cufft::cufftDoubleComplex,
        odata: *mut cuda_types::cufft::cufftDoubleComplex,
        direction: ::core::ffi::c_int,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftExecD2Z(
        plan: cuda_types::cufft::cufftHandle,
        idata: *mut cuda_types::cufft::cufftDoubleReal,
        odata: *mut cuda_types::cufft::cufftDoubleComplex,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftExecZ2D(
        plan: cuda_types::cufft::cufftHandle,
        idata: *mut cuda_types::cufft::cufftDoubleComplex,
        odata: *mut cuda_types::cufft::cufftDoubleReal,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftSetStream(
        plan: cuda_types::cufft::cufftHandle,
        stream: cuda_types::cufft::cudaStream_t,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftDestroy(
        plan: cuda_types::cufft::cufftHandle,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftGetVersion(
        version: *mut ::core::ffi::c_int,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftGetProperty(
        type_: cuda_types::cufft::libraryPropertyType,
        value: *mut ::core::ffi::c_int,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftSetPlanPropertyInt64(
        plan: cuda_types::cufft::cufftHandle,
        property: cuda_types::cufft::cufftProperty,
        inputValueInt: ::core::ffi::c_longlong,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftGetPlanPropertyInt64(
        plan: cuda_types::cufft::cufftHandle,
        property: cuda_types::cufft::cufftProperty,
        returnPtrValue: *mut ::core::ffi::c_longlong,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftResetPlanProperty(
        plan: cuda_types::cufft::cufftHandle,
        property: cuda_types::cufft::cufftProperty,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftXtSetGPUs(
        handle: cuda_types::cufft::cufftHandle,
        nGPUs: ::core::ffi::c_int,
        whichGPUs: *mut ::core::ffi::c_int,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftXtMalloc(
        plan: cuda_types::cufft::cufftHandle,
        descriptor: *mut *mut cuda_types::cufft::cudaLibXtDesc,
        format: cuda_types::cufft::cufftXtSubFormat,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftXtMemcpy(
        plan: cuda_types::cufft::cufftHandle,
        dstPointer: *mut ::core::ffi::c_void,
        srcPointer: *mut ::core::ffi::c_void,
        type_: cuda_types::cufft::cufftXtCopyType,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftXtFree(
        descriptor: *mut cuda_types::cufft::cudaLibXtDesc,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftXtSetWorkArea(
        plan: cuda_types::cufft::cufftHandle,
        workArea: *mut *mut ::core::ffi::c_void,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftXtExecDescriptorC2C(
        plan: cuda_types::cufft::cufftHandle,
        input: *mut cuda_types::cufft::cudaLibXtDesc,
        output: *mut cuda_types::cufft::cudaLibXtDesc,
        direction: ::core::ffi::c_int,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftXtExecDescriptorR2C(
        plan: cuda_types::cufft::cufftHandle,
        input: *mut cuda_types::cufft::cudaLibXtDesc,
        output: *mut cuda_types::cufft::cudaLibXtDesc,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftXtExecDescriptorC2R(
        plan: cuda_types::cufft::cufftHandle,
        input: *mut cuda_types::cufft::cudaLibXtDesc,
        output: *mut cuda_types::cufft::cudaLibXtDesc,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftXtExecDescriptorZ2Z(
        plan: cuda_types::cufft::cufftHandle,
        input: *mut cuda_types::cufft::cudaLibXtDesc,
        output: *mut cuda_types::cufft::cudaLibXtDesc,
        direction: ::core::ffi::c_int,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftXtExecDescriptorD2Z(
        plan: cuda_types::cufft::cufftHandle,
        input: *mut cuda_types::cufft::cudaLibXtDesc,
        output: *mut cuda_types::cufft::cudaLibXtDesc,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftXtExecDescriptorZ2D(
        plan: cuda_types::cufft::cufftHandle,
        input: *mut cuda_types::cufft::cudaLibXtDesc,
        output: *mut cuda_types::cufft::cudaLibXtDesc,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftXtQueryPlan(
        plan: cuda_types::cufft::cufftHandle,
        queryStruct: *mut ::core::ffi::c_void,
        queryType: cuda_types::cufft::cufftXtQueryType,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftXtSetCallback(
        plan: cuda_types::cufft::cufftHandle,
        callback_routine: *mut *mut ::core::ffi::c_void,
        cbType: cuda_types::cufft::cufftXtCallbackType,
        caller_info: *mut *mut ::core::ffi::c_void,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftXtClearCallback(
        plan: cuda_types::cufft::cufftHandle,
        cbType: cuda_types::cufft::cufftXtCallbackType,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftXtSetCallbackSharedSize(
        plan: cuda_types::cufft::cufftHandle,
        cbType: cuda_types::cufft::cufftXtCallbackType,
        sharedSize: usize,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftXtSetJITCallback(
        plan: cuda_types::cufft::cufftHandle,
        lto_callback_symbol_name: *const ::core::ffi::c_char,
        lto_callback_fatbin: *const ::core::ffi::c_void,
        lto_callback_fatbin_size: usize,
        type_: cuda_types::cufft::cufftXtCallbackType,
        caller_info: *mut *mut ::core::ffi::c_void,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftXtMakePlanMany(
        plan: cuda_types::cufft::cufftHandle,
        rank: ::core::ffi::c_int,
        n: *mut ::core::ffi::c_longlong,
        inembed: *mut ::core::ffi::c_longlong,
        istride: ::core::ffi::c_longlong,
        idist: ::core::ffi::c_longlong,
        inputtype: cuda_types::cufft::cudaDataType,
        onembed: *mut ::core::ffi::c_longlong,
        ostride: ::core::ffi::c_longlong,
        odist: ::core::ffi::c_longlong,
        outputtype: cuda_types::cufft::cudaDataType,
        batch: ::core::ffi::c_longlong,
        workSize: *mut usize,
        executiontype: cuda_types::cufft::cudaDataType,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftXtGetSizeMany(
        plan: cuda_types::cufft::cufftHandle,
        rank: ::core::ffi::c_int,
        n: *mut ::core::ffi::c_longlong,
        inembed: *mut ::core::ffi::c_longlong,
        istride: ::core::ffi::c_longlong,
        idist: ::core::ffi::c_longlong,
        inputtype: cuda_types::cufft::cudaDataType,
        onembed: *mut ::core::ffi::c_longlong,
        ostride: ::core::ffi::c_longlong,
        odist: ::core::ffi::c_longlong,
        outputtype: cuda_types::cufft::cudaDataType,
        batch: ::core::ffi::c_longlong,
        workSize: *mut usize,
        executiontype: cuda_types::cufft::cudaDataType,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftXtExec(
        plan: cuda_types::cufft::cufftHandle,
        input: *mut ::core::ffi::c_void,
        output: *mut ::core::ffi::c_void,
        direction: ::core::ffi::c_int,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftXtExecDescriptor(
        plan: cuda_types::cufft::cufftHandle,
        input: *mut cuda_types::cufft::cudaLibXtDesc,
        output: *mut cuda_types::cufft::cudaLibXtDesc,
        direction: ::core::ffi::c_int,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftXtSetWorkAreaPolicy(
        plan: cuda_types::cufft::cufftHandle,
        policy: cuda_types::cufft::cufftXtWorkAreaPolicy,
        workSize: *mut usize,
    ) -> cuda_types::cufft::cufftResult;
    fn cufftXtMakePlanGuru64(
        param_1: ::core::ffi::c_uint,
        param_2: ::core::ffi::c_int,
        param_3: ::core::ffi::c_ulonglong,
        param_4: ::core::ffi::c_int,
        param_5: ::core::ffi::c_ulonglong,
        param_6: ::core::ffi::c_uint,
        param_7: ::core::ffi::c_uint,
        param_8: ::core::ffi::c_uint,
        param_9: ::core::ffi::c_ulonglong,
        param_10: ::core::ffi::c_uint,
    ) -> cuda_types::cufft::cufftResult;
}
