// Generated automatically by zluda_bindgen
// DO NOT EDIT MANUALLY
#![allow(warnings)]
pub fn write_cudnnGetVersion(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
pub fn write_cudnnGetMaxDeviceVersion(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
pub fn write_cudnnGetCudartVersion(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
pub fn write_cudnnGetErrorString(
    writer: &mut (impl std::io::Write + ?Sized),
    status: cuda_types::cudnn8::cudnnStatus_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(status), ": ").as_bytes())?;
    crate::CudaDisplay::write(&status, "cudnnGetErrorString", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnQueryRuntimeError(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    rstatus: *mut cuda_types::cudnn8::cudnnStatus_t,
    mode: cuda_types::cudnn8::cudnnErrQueryMode_t,
    tag: *mut cuda_types::cudnn8::cudnnRuntimeTag_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnQueryRuntimeError", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rstatus), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rstatus, "cudnnQueryRuntimeError", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mode, "cudnnQueryRuntimeError", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(tag), ": ").as_bytes())?;
    crate::CudaDisplay::write(&tag, "cudnnQueryRuntimeError", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnGetProperty(
    writer: &mut (impl std::io::Write + ?Sized),
    type_: cuda_types::cudnn8::libraryPropertyType,
    value: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cudnnGetProperty", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cudnnGetProperty", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnCreate(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: *mut cuda_types::cudnn8::cudnnHandle_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnCreate", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnDestroy(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnDestroy", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnSetStream(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    streamId: cuda_types::cudnn8::cudaStream_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnSetStream", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(streamId), ": ").as_bytes())?;
    crate::CudaDisplay::write(&streamId, "cudnnSetStream", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnGetStream(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    streamId: *mut cuda_types::cudnn8::cudaStream_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnGetStream", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(streamId), ": ").as_bytes())?;
    crate::CudaDisplay::write(&streamId, "cudnnGetStream", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnCreateTensorDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    tensorDesc: *mut cuda_types::cudnn8::cudnnTensorDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(tensorDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &tensorDesc,
        "cudnnCreateTensorDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnSetTensor4dDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    tensorDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    format: cuda_types::cudnn8::cudnnTensorFormat_t,
    dataType: cuda_types::cudnn8::cudnnDataType_t,
    n: ::core::ffi::c_int,
    c: ::core::ffi::c_int,
    h: ::core::ffi::c_int,
    w: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(tensorDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &tensorDesc,
        "cudnnSetTensor4dDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(format), ": ").as_bytes())?;
    crate::CudaDisplay::write(&format, "cudnnSetTensor4dDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dataType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dataType, "cudnnSetTensor4dDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(n), ": ").as_bytes())?;
    crate::CudaDisplay::write(&n, "cudnnSetTensor4dDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(c), ": ").as_bytes())?;
    crate::CudaDisplay::write(&c, "cudnnSetTensor4dDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(h), ": ").as_bytes())?;
    crate::CudaDisplay::write(&h, "cudnnSetTensor4dDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(w), ": ").as_bytes())?;
    crate::CudaDisplay::write(&w, "cudnnSetTensor4dDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnSetTensor4dDescriptorEx(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(tensorDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &tensorDesc,
        "cudnnSetTensor4dDescriptorEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dataType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dataType,
        "cudnnSetTensor4dDescriptorEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(n), ": ").as_bytes())?;
    crate::CudaDisplay::write(&n, "cudnnSetTensor4dDescriptorEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(c), ": ").as_bytes())?;
    crate::CudaDisplay::write(&c, "cudnnSetTensor4dDescriptorEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(h), ": ").as_bytes())?;
    crate::CudaDisplay::write(&h, "cudnnSetTensor4dDescriptorEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(w), ": ").as_bytes())?;
    crate::CudaDisplay::write(&w, "cudnnSetTensor4dDescriptorEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nStride), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nStride,
        "cudnnSetTensor4dDescriptorEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cStride), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &cStride,
        "cudnnSetTensor4dDescriptorEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStride), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hStride,
        "cudnnSetTensor4dDescriptorEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(wStride), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &wStride,
        "cudnnSetTensor4dDescriptorEx",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetTensor4dDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(tensorDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &tensorDesc,
        "cudnnGetTensor4dDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dataType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dataType, "cudnnGetTensor4dDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(n), ": ").as_bytes())?;
    crate::CudaDisplay::write(&n, "cudnnGetTensor4dDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(c), ": ").as_bytes())?;
    crate::CudaDisplay::write(&c, "cudnnGetTensor4dDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(h), ": ").as_bytes())?;
    crate::CudaDisplay::write(&h, "cudnnGetTensor4dDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(w), ": ").as_bytes())?;
    crate::CudaDisplay::write(&w, "cudnnGetTensor4dDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nStride), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nStride, "cudnnGetTensor4dDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cStride), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cStride, "cudnnGetTensor4dDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStride), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStride, "cudnnGetTensor4dDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(wStride), ": ").as_bytes())?;
    crate::CudaDisplay::write(&wStride, "cudnnGetTensor4dDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnSetTensorNdDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    tensorDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    dataType: cuda_types::cudnn8::cudnnDataType_t,
    nbDims: ::core::ffi::c_int,
    dimA: *const ::core::ffi::c_int,
    strideA: *const ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(tensorDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &tensorDesc,
        "cudnnSetTensorNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dataType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dataType, "cudnnSetTensorNdDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nbDims), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nbDims, "cudnnSetTensorNdDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dimA), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dimA, "cudnnSetTensorNdDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(strideA), ": ").as_bytes())?;
    crate::CudaDisplay::write(&strideA, "cudnnSetTensorNdDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnSetTensorNdDescriptorEx(
    writer: &mut (impl std::io::Write + ?Sized),
    tensorDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    format: cuda_types::cudnn8::cudnnTensorFormat_t,
    dataType: cuda_types::cudnn8::cudnnDataType_t,
    nbDims: ::core::ffi::c_int,
    dimA: *const ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(tensorDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &tensorDesc,
        "cudnnSetTensorNdDescriptorEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(format), ": ").as_bytes())?;
    crate::CudaDisplay::write(&format, "cudnnSetTensorNdDescriptorEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dataType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dataType,
        "cudnnSetTensorNdDescriptorEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nbDims), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nbDims, "cudnnSetTensorNdDescriptorEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dimA), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dimA, "cudnnSetTensorNdDescriptorEx", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnGetTensorNdDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    tensorDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    nbDimsRequested: ::core::ffi::c_int,
    dataType: *mut cuda_types::cudnn8::cudnnDataType_t,
    nbDims: *mut ::core::ffi::c_int,
    dimA: *mut ::core::ffi::c_int,
    strideA: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(tensorDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &tensorDesc,
        "cudnnGetTensorNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nbDimsRequested), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nbDimsRequested,
        "cudnnGetTensorNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dataType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dataType, "cudnnGetTensorNdDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nbDims), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nbDims, "cudnnGetTensorNdDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dimA), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dimA, "cudnnGetTensorNdDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(strideA), ": ").as_bytes())?;
    crate::CudaDisplay::write(&strideA, "cudnnGetTensorNdDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnGetTensorSizeInBytes(
    writer: &mut (impl std::io::Write + ?Sized),
    tensorDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    size: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(tensorDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &tensorDesc,
        "cudnnGetTensorSizeInBytes",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(&size, "cudnnGetTensorSizeInBytes", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnDestroyTensorDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    tensorDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(tensorDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &tensorDesc,
        "cudnnDestroyTensorDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnInitTransformDest(
    writer: &mut (impl std::io::Write + ?Sized),
    transformDesc: cuda_types::cudnn8::cudnnTensorTransformDescriptor_t,
    srcDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    destDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    destSizeInBytes: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(transformDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &transformDesc,
        "cudnnInitTransformDest",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcDesc, "cudnnInitTransformDest", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(destDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&destDesc, "cudnnInitTransformDest", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(destSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &destSizeInBytes,
        "cudnnInitTransformDest",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnCreateTensorTransformDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    transformDesc: *mut cuda_types::cudnn8::cudnnTensorTransformDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(transformDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &transformDesc,
        "cudnnCreateTensorTransformDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnSetTensorTransformDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    transformDesc: cuda_types::cudnn8::cudnnTensorTransformDescriptor_t,
    nbDims: u32,
    destFormat: cuda_types::cudnn8::cudnnTensorFormat_t,
    padBeforeA: *const i32,
    padAfterA: *const i32,
    foldA: *const u32,
    direction: cuda_types::cudnn8::cudnnFoldingDirection_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(transformDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &transformDesc,
        "cudnnSetTensorTransformDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nbDims), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nbDims,
        "cudnnSetTensorTransformDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(destFormat), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &destFormat,
        "cudnnSetTensorTransformDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(padBeforeA), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &padBeforeA,
        "cudnnSetTensorTransformDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(padAfterA), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &padAfterA,
        "cudnnSetTensorTransformDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(foldA), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &foldA,
        "cudnnSetTensorTransformDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(direction), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &direction,
        "cudnnSetTensorTransformDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetTensorTransformDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    transformDesc: cuda_types::cudnn8::cudnnTensorTransformDescriptor_t,
    nbDimsRequested: u32,
    destFormat: *mut cuda_types::cudnn8::cudnnTensorFormat_t,
    padBeforeA: *mut i32,
    padAfterA: *mut i32,
    foldA: *mut u32,
    direction: *mut cuda_types::cudnn8::cudnnFoldingDirection_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(transformDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &transformDesc,
        "cudnnGetTensorTransformDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nbDimsRequested), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nbDimsRequested,
        "cudnnGetTensorTransformDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(destFormat), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &destFormat,
        "cudnnGetTensorTransformDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(padBeforeA), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &padBeforeA,
        "cudnnGetTensorTransformDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(padAfterA), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &padAfterA,
        "cudnnGetTensorTransformDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(foldA), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &foldA,
        "cudnnGetTensorTransformDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(direction), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &direction,
        "cudnnGetTensorTransformDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnDestroyTensorTransformDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    transformDesc: cuda_types::cudnn8::cudnnTensorTransformDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(transformDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &transformDesc,
        "cudnnDestroyTensorTransformDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnTransformTensor(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    alpha: *const ::core::ffi::c_void,
    xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    y: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnTransformTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(&alpha, "cudnnTransformTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&xDesc, "cudnnTransformTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(&x, "cudnnTransformTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(&beta, "cudnnTransformTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&yDesc, "cudnnTransformTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(&y, "cudnnTransformTensor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnTransformTensorEx(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    transDesc: cuda_types::cudnn8::cudnnTensorTransformDescriptor_t,
    alpha: *const ::core::ffi::c_void,
    srcDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    srcData: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    destDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    destData: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnTransformTensorEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(transDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&transDesc, "cudnnTransformTensorEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(&alpha, "cudnnTransformTensorEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcDesc, "cudnnTransformTensorEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcData), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcData, "cudnnTransformTensorEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(&beta, "cudnnTransformTensorEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(destDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&destDesc, "cudnnTransformTensorEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(destData), ": ").as_bytes())?;
    crate::CudaDisplay::write(&destData, "cudnnTransformTensorEx", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnAddTensor(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    alpha: *const ::core::ffi::c_void,
    aDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    A: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    cDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    C: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnAddTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(&alpha, "cudnnAddTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(aDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&aDesc, "cudnnAddTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(A), ": ").as_bytes())?;
    crate::CudaDisplay::write(&A, "cudnnAddTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(&beta, "cudnnAddTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cDesc, "cudnnAddTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(C), ": ").as_bytes())?;
    crate::CudaDisplay::write(&C, "cudnnAddTensor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnCreateOpTensorDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    opTensorDesc: *mut cuda_types::cudnn8::cudnnOpTensorDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(opTensorDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &opTensorDesc,
        "cudnnCreateOpTensorDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnSetOpTensorDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    opTensorDesc: cuda_types::cudnn8::cudnnOpTensorDescriptor_t,
    opTensorOp: cuda_types::cudnn8::cudnnOpTensorOp_t,
    opTensorCompType: cuda_types::cudnn8::cudnnDataType_t,
    opTensorNanOpt: cuda_types::cudnn8::cudnnNanPropagation_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(opTensorDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &opTensorDesc,
        "cudnnSetOpTensorDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(opTensorOp), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &opTensorOp,
        "cudnnSetOpTensorDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(opTensorCompType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &opTensorCompType,
        "cudnnSetOpTensorDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(opTensorNanOpt), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &opTensorNanOpt,
        "cudnnSetOpTensorDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetOpTensorDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    opTensorDesc: cuda_types::cudnn8::cudnnOpTensorDescriptor_t,
    opTensorOp: *mut cuda_types::cudnn8::cudnnOpTensorOp_t,
    opTensorCompType: *mut cuda_types::cudnn8::cudnnDataType_t,
    opTensorNanOpt: *mut cuda_types::cudnn8::cudnnNanPropagation_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(opTensorDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &opTensorDesc,
        "cudnnGetOpTensorDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(opTensorOp), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &opTensorOp,
        "cudnnGetOpTensorDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(opTensorCompType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &opTensorCompType,
        "cudnnGetOpTensorDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(opTensorNanOpt), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &opTensorNanOpt,
        "cudnnGetOpTensorDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnDestroyOpTensorDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    opTensorDesc: cuda_types::cudnn8::cudnnOpTensorDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(opTensorDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &opTensorDesc,
        "cudnnDestroyOpTensorDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnOpTensor(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnOpTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(opTensorDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&opTensorDesc, "cudnnOpTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&alpha1, "cudnnOpTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(aDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&aDesc, "cudnnOpTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(A), ": ").as_bytes())?;
    crate::CudaDisplay::write(&A, "cudnnOpTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&alpha2, "cudnnOpTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&bDesc, "cudnnOpTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(B), ": ").as_bytes())?;
    crate::CudaDisplay::write(&B, "cudnnOpTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(&beta, "cudnnOpTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cDesc, "cudnnOpTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(C), ": ").as_bytes())?;
    crate::CudaDisplay::write(&C, "cudnnOpTensor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnCreateReduceTensorDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    reduceTensorDesc: *mut cuda_types::cudnn8::cudnnReduceTensorDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(reduceTensorDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reduceTensorDesc,
        "cudnnCreateReduceTensorDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnSetReduceTensorDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    reduceTensorDesc: cuda_types::cudnn8::cudnnReduceTensorDescriptor_t,
    reduceTensorOp: cuda_types::cudnn8::cudnnReduceTensorOp_t,
    reduceTensorCompType: cuda_types::cudnn8::cudnnDataType_t,
    reduceTensorNanOpt: cuda_types::cudnn8::cudnnNanPropagation_t,
    reduceTensorIndices: cuda_types::cudnn8::cudnnReduceTensorIndices_t,
    reduceTensorIndicesType: cuda_types::cudnn8::cudnnIndicesType_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(reduceTensorDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reduceTensorDesc,
        "cudnnSetReduceTensorDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reduceTensorOp), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reduceTensorOp,
        "cudnnSetReduceTensorDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reduceTensorCompType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reduceTensorCompType,
        "cudnnSetReduceTensorDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reduceTensorNanOpt), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reduceTensorNanOpt,
        "cudnnSetReduceTensorDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reduceTensorIndices), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reduceTensorIndices,
        "cudnnSetReduceTensorDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reduceTensorIndicesType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reduceTensorIndicesType,
        "cudnnSetReduceTensorDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetReduceTensorDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    reduceTensorDesc: cuda_types::cudnn8::cudnnReduceTensorDescriptor_t,
    reduceTensorOp: *mut cuda_types::cudnn8::cudnnReduceTensorOp_t,
    reduceTensorCompType: *mut cuda_types::cudnn8::cudnnDataType_t,
    reduceTensorNanOpt: *mut cuda_types::cudnn8::cudnnNanPropagation_t,
    reduceTensorIndices: *mut cuda_types::cudnn8::cudnnReduceTensorIndices_t,
    reduceTensorIndicesType: *mut cuda_types::cudnn8::cudnnIndicesType_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(reduceTensorDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reduceTensorDesc,
        "cudnnGetReduceTensorDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reduceTensorOp), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reduceTensorOp,
        "cudnnGetReduceTensorDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reduceTensorCompType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reduceTensorCompType,
        "cudnnGetReduceTensorDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reduceTensorNanOpt), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reduceTensorNanOpt,
        "cudnnGetReduceTensorDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reduceTensorIndices), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reduceTensorIndices,
        "cudnnGetReduceTensorDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reduceTensorIndicesType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reduceTensorIndicesType,
        "cudnnGetReduceTensorDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnDestroyReduceTensorDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    reduceTensorDesc: cuda_types::cudnn8::cudnnReduceTensorDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(reduceTensorDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reduceTensorDesc,
        "cudnnDestroyReduceTensorDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetReductionIndicesSize(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    reduceTensorDesc: cuda_types::cudnn8::cudnnReduceTensorDescriptor_t,
    aDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    cDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    sizeInBytes: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnGetReductionIndicesSize", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reduceTensorDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reduceTensorDesc,
        "cudnnGetReductionIndicesSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(aDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&aDesc, "cudnnGetReductionIndicesSize", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cDesc, "cudnnGetReductionIndicesSize", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeInBytes,
        "cudnnGetReductionIndicesSize",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetReductionWorkspaceSize(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    reduceTensorDesc: cuda_types::cudnn8::cudnnReduceTensorDescriptor_t,
    aDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    cDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    sizeInBytes: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnGetReductionWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reduceTensorDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reduceTensorDesc,
        "cudnnGetReductionWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(aDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &aDesc,
        "cudnnGetReductionWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &cDesc,
        "cudnnGetReductionWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeInBytes,
        "cudnnGetReductionWorkspaceSize",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnReduceTensor(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnReduceTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reduceTensorDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&reduceTensorDesc, "cudnnReduceTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(indices), ": ").as_bytes())?;
    crate::CudaDisplay::write(&indices, "cudnnReduceTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(indicesSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &indicesSizeInBytes,
        "cudnnReduceTensor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workspace), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workspace, "cudnnReduceTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workspaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workspaceSizeInBytes,
        "cudnnReduceTensor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(&alpha, "cudnnReduceTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(aDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&aDesc, "cudnnReduceTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(A), ": ").as_bytes())?;
    crate::CudaDisplay::write(&A, "cudnnReduceTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(&beta, "cudnnReduceTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cDesc, "cudnnReduceTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(C), ": ").as_bytes())?;
    crate::CudaDisplay::write(&C, "cudnnReduceTensor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnSetTensor(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    y: *mut ::core::ffi::c_void,
    valuePtr: *const ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnSetTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&yDesc, "cudnnSetTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(&y, "cudnnSetTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(valuePtr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&valuePtr, "cudnnSetTensor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnScaleTensor(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    y: *mut ::core::ffi::c_void,
    alpha: *const ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnScaleTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&yDesc, "cudnnScaleTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(&y, "cudnnScaleTensor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(&alpha, "cudnnScaleTensor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnCreateFilterDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    filterDesc: *mut cuda_types::cudnn8::cudnnFilterDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(filterDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &filterDesc,
        "cudnnCreateFilterDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnSetFilter4dDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    filterDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
    dataType: cuda_types::cudnn8::cudnnDataType_t,
    format: cuda_types::cudnn8::cudnnTensorFormat_t,
    k: ::core::ffi::c_int,
    c: ::core::ffi::c_int,
    h: ::core::ffi::c_int,
    w: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(filterDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &filterDesc,
        "cudnnSetFilter4dDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dataType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dataType, "cudnnSetFilter4dDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(format), ": ").as_bytes())?;
    crate::CudaDisplay::write(&format, "cudnnSetFilter4dDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(k), ": ").as_bytes())?;
    crate::CudaDisplay::write(&k, "cudnnSetFilter4dDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(c), ": ").as_bytes())?;
    crate::CudaDisplay::write(&c, "cudnnSetFilter4dDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(h), ": ").as_bytes())?;
    crate::CudaDisplay::write(&h, "cudnnSetFilter4dDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(w), ": ").as_bytes())?;
    crate::CudaDisplay::write(&w, "cudnnSetFilter4dDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnGetFilter4dDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    filterDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
    dataType: *mut cuda_types::cudnn8::cudnnDataType_t,
    format: *mut cuda_types::cudnn8::cudnnTensorFormat_t,
    k: *mut ::core::ffi::c_int,
    c: *mut ::core::ffi::c_int,
    h: *mut ::core::ffi::c_int,
    w: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(filterDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &filterDesc,
        "cudnnGetFilter4dDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dataType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dataType, "cudnnGetFilter4dDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(format), ": ").as_bytes())?;
    crate::CudaDisplay::write(&format, "cudnnGetFilter4dDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(k), ": ").as_bytes())?;
    crate::CudaDisplay::write(&k, "cudnnGetFilter4dDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(c), ": ").as_bytes())?;
    crate::CudaDisplay::write(&c, "cudnnGetFilter4dDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(h), ": ").as_bytes())?;
    crate::CudaDisplay::write(&h, "cudnnGetFilter4dDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(w), ": ").as_bytes())?;
    crate::CudaDisplay::write(&w, "cudnnGetFilter4dDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnSetFilterNdDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    filterDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
    dataType: cuda_types::cudnn8::cudnnDataType_t,
    format: cuda_types::cudnn8::cudnnTensorFormat_t,
    nbDims: ::core::ffi::c_int,
    filterDimA: *const ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(filterDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &filterDesc,
        "cudnnSetFilterNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dataType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dataType, "cudnnSetFilterNdDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(format), ": ").as_bytes())?;
    crate::CudaDisplay::write(&format, "cudnnSetFilterNdDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nbDims), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nbDims, "cudnnSetFilterNdDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(filterDimA), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &filterDimA,
        "cudnnSetFilterNdDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetFilterNdDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    filterDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
    nbDimsRequested: ::core::ffi::c_int,
    dataType: *mut cuda_types::cudnn8::cudnnDataType_t,
    format: *mut cuda_types::cudnn8::cudnnTensorFormat_t,
    nbDims: *mut ::core::ffi::c_int,
    filterDimA: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(filterDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &filterDesc,
        "cudnnGetFilterNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nbDimsRequested), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nbDimsRequested,
        "cudnnGetFilterNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dataType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dataType, "cudnnGetFilterNdDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(format), ": ").as_bytes())?;
    crate::CudaDisplay::write(&format, "cudnnGetFilterNdDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nbDims), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nbDims, "cudnnGetFilterNdDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(filterDimA), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &filterDimA,
        "cudnnGetFilterNdDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetFilterSizeInBytes(
    writer: &mut (impl std::io::Write + ?Sized),
    filterDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
    size: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(filterDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &filterDesc,
        "cudnnGetFilterSizeInBytes",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(&size, "cudnnGetFilterSizeInBytes", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnTransformFilter(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    transDesc: cuda_types::cudnn8::cudnnTensorTransformDescriptor_t,
    alpha: *const ::core::ffi::c_void,
    srcDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
    srcData: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    destDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
    destData: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnTransformFilter", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(transDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&transDesc, "cudnnTransformFilter", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(&alpha, "cudnnTransformFilter", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcDesc, "cudnnTransformFilter", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcData), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcData, "cudnnTransformFilter", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(&beta, "cudnnTransformFilter", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(destDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&destDesc, "cudnnTransformFilter", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(destData), ": ").as_bytes())?;
    crate::CudaDisplay::write(&destData, "cudnnTransformFilter", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnDestroyFilterDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    filterDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(filterDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &filterDesc,
        "cudnnDestroyFilterDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnSoftmaxForward(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    algo: cuda_types::cudnn8::cudnnSoftmaxAlgorithm_t,
    mode: cuda_types::cudnn8::cudnnSoftmaxMode_t,
    alpha: *const ::core::ffi::c_void,
    xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    y: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnSoftmaxForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algo), ": ").as_bytes())?;
    crate::CudaDisplay::write(&algo, "cudnnSoftmaxForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mode, "cudnnSoftmaxForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(&alpha, "cudnnSoftmaxForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&xDesc, "cudnnSoftmaxForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(&x, "cudnnSoftmaxForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(&beta, "cudnnSoftmaxForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&yDesc, "cudnnSoftmaxForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(&y, "cudnnSoftmaxForward", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnCreatePoolingDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    poolingDesc: *mut cuda_types::cudnn8::cudnnPoolingDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(poolingDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &poolingDesc,
        "cudnnCreatePoolingDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnSetPooling2dDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    poolingDesc: cuda_types::cudnn8::cudnnPoolingDescriptor_t,
    mode: cuda_types::cudnn8::cudnnPoolingMode_t,
    maxpoolingNanOpt: cuda_types::cudnn8::cudnnNanPropagation_t,
    windowHeight: ::core::ffi::c_int,
    windowWidth: ::core::ffi::c_int,
    verticalPadding: ::core::ffi::c_int,
    horizontalPadding: ::core::ffi::c_int,
    verticalStride: ::core::ffi::c_int,
    horizontalStride: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(poolingDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &poolingDesc,
        "cudnnSetPooling2dDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mode, "cudnnSetPooling2dDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(maxpoolingNanOpt), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &maxpoolingNanOpt,
        "cudnnSetPooling2dDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(windowHeight), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &windowHeight,
        "cudnnSetPooling2dDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(windowWidth), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &windowWidth,
        "cudnnSetPooling2dDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(verticalPadding), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &verticalPadding,
        "cudnnSetPooling2dDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(horizontalPadding), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &horizontalPadding,
        "cudnnSetPooling2dDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(verticalStride), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &verticalStride,
        "cudnnSetPooling2dDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(horizontalStride), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &horizontalStride,
        "cudnnSetPooling2dDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetPooling2dDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    poolingDesc: cuda_types::cudnn8::cudnnPoolingDescriptor_t,
    mode: *mut cuda_types::cudnn8::cudnnPoolingMode_t,
    maxpoolingNanOpt: *mut cuda_types::cudnn8::cudnnNanPropagation_t,
    windowHeight: *mut ::core::ffi::c_int,
    windowWidth: *mut ::core::ffi::c_int,
    verticalPadding: *mut ::core::ffi::c_int,
    horizontalPadding: *mut ::core::ffi::c_int,
    verticalStride: *mut ::core::ffi::c_int,
    horizontalStride: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(poolingDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &poolingDesc,
        "cudnnGetPooling2dDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mode, "cudnnGetPooling2dDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(maxpoolingNanOpt), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &maxpoolingNanOpt,
        "cudnnGetPooling2dDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(windowHeight), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &windowHeight,
        "cudnnGetPooling2dDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(windowWidth), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &windowWidth,
        "cudnnGetPooling2dDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(verticalPadding), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &verticalPadding,
        "cudnnGetPooling2dDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(horizontalPadding), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &horizontalPadding,
        "cudnnGetPooling2dDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(verticalStride), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &verticalStride,
        "cudnnGetPooling2dDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(horizontalStride), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &horizontalStride,
        "cudnnGetPooling2dDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnSetPoolingNdDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    poolingDesc: cuda_types::cudnn8::cudnnPoolingDescriptor_t,
    mode: cuda_types::cudnn8::cudnnPoolingMode_t,
    maxpoolingNanOpt: cuda_types::cudnn8::cudnnNanPropagation_t,
    nbDims: ::core::ffi::c_int,
    windowDimA: *const ::core::ffi::c_int,
    paddingA: *const ::core::ffi::c_int,
    strideA: *const ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(poolingDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &poolingDesc,
        "cudnnSetPoolingNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mode, "cudnnSetPoolingNdDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(maxpoolingNanOpt), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &maxpoolingNanOpt,
        "cudnnSetPoolingNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nbDims), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nbDims, "cudnnSetPoolingNdDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(windowDimA), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &windowDimA,
        "cudnnSetPoolingNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(paddingA), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &paddingA,
        "cudnnSetPoolingNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(strideA), ": ").as_bytes())?;
    crate::CudaDisplay::write(&strideA, "cudnnSetPoolingNdDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnGetPoolingNdDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    poolingDesc: cuda_types::cudnn8::cudnnPoolingDescriptor_t,
    nbDimsRequested: ::core::ffi::c_int,
    mode: *mut cuda_types::cudnn8::cudnnPoolingMode_t,
    maxpoolingNanOpt: *mut cuda_types::cudnn8::cudnnNanPropagation_t,
    nbDims: *mut ::core::ffi::c_int,
    windowDimA: *mut ::core::ffi::c_int,
    paddingA: *mut ::core::ffi::c_int,
    strideA: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(poolingDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &poolingDesc,
        "cudnnGetPoolingNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nbDimsRequested), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nbDimsRequested,
        "cudnnGetPoolingNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mode, "cudnnGetPoolingNdDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(maxpoolingNanOpt), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &maxpoolingNanOpt,
        "cudnnGetPoolingNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nbDims), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nbDims, "cudnnGetPoolingNdDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(windowDimA), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &windowDimA,
        "cudnnGetPoolingNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(paddingA), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &paddingA,
        "cudnnGetPoolingNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(strideA), ": ").as_bytes())?;
    crate::CudaDisplay::write(&strideA, "cudnnGetPoolingNdDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnGetPoolingNdForwardOutputDim(
    writer: &mut (impl std::io::Write + ?Sized),
    poolingDesc: cuda_types::cudnn8::cudnnPoolingDescriptor_t,
    inputTensorDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    nbDims: ::core::ffi::c_int,
    outputTensorDimA: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(poolingDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &poolingDesc,
        "cudnnGetPoolingNdForwardOutputDim",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(inputTensorDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &inputTensorDesc,
        "cudnnGetPoolingNdForwardOutputDim",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nbDims), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nbDims,
        "cudnnGetPoolingNdForwardOutputDim",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(outputTensorDimA), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &outputTensorDimA,
        "cudnnGetPoolingNdForwardOutputDim",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetPooling2dForwardOutputDim(
    writer: &mut (impl std::io::Write + ?Sized),
    poolingDesc: cuda_types::cudnn8::cudnnPoolingDescriptor_t,
    inputTensorDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    n: *mut ::core::ffi::c_int,
    c: *mut ::core::ffi::c_int,
    h: *mut ::core::ffi::c_int,
    w: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(poolingDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &poolingDesc,
        "cudnnGetPooling2dForwardOutputDim",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(inputTensorDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &inputTensorDesc,
        "cudnnGetPooling2dForwardOutputDim",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(n), ": ").as_bytes())?;
    crate::CudaDisplay::write(&n, "cudnnGetPooling2dForwardOutputDim", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(c), ": ").as_bytes())?;
    crate::CudaDisplay::write(&c, "cudnnGetPooling2dForwardOutputDim", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(h), ": ").as_bytes())?;
    crate::CudaDisplay::write(&h, "cudnnGetPooling2dForwardOutputDim", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(w), ": ").as_bytes())?;
    crate::CudaDisplay::write(&w, "cudnnGetPooling2dForwardOutputDim", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnDestroyPoolingDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    poolingDesc: cuda_types::cudnn8::cudnnPoolingDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(poolingDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &poolingDesc,
        "cudnnDestroyPoolingDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnPoolingForward(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    poolingDesc: cuda_types::cudnn8::cudnnPoolingDescriptor_t,
    alpha: *const ::core::ffi::c_void,
    xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    y: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnPoolingForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(poolingDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&poolingDesc, "cudnnPoolingForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(&alpha, "cudnnPoolingForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&xDesc, "cudnnPoolingForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(&x, "cudnnPoolingForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(&beta, "cudnnPoolingForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&yDesc, "cudnnPoolingForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(&y, "cudnnPoolingForward", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnCreateActivationDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    activationDesc: *mut cuda_types::cudnn8::cudnnActivationDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(activationDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &activationDesc,
        "cudnnCreateActivationDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnSetActivationDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    activationDesc: cuda_types::cudnn8::cudnnActivationDescriptor_t,
    mode: cuda_types::cudnn8::cudnnActivationMode_t,
    reluNanOpt: cuda_types::cudnn8::cudnnNanPropagation_t,
    coef: f64,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(activationDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &activationDesc,
        "cudnnSetActivationDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mode, "cudnnSetActivationDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reluNanOpt), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reluNanOpt,
        "cudnnSetActivationDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(coef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&coef, "cudnnSetActivationDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnGetActivationDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    activationDesc: cuda_types::cudnn8::cudnnActivationDescriptor_t,
    mode: *mut cuda_types::cudnn8::cudnnActivationMode_t,
    reluNanOpt: *mut cuda_types::cudnn8::cudnnNanPropagation_t,
    coef: *mut f64,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(activationDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &activationDesc,
        "cudnnGetActivationDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mode, "cudnnGetActivationDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reluNanOpt), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reluNanOpt,
        "cudnnGetActivationDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(coef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&coef, "cudnnGetActivationDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnSetActivationDescriptorSwishBeta(
    writer: &mut (impl std::io::Write + ?Sized),
    activationDesc: cuda_types::cudnn8::cudnnActivationDescriptor_t,
    swish_beta: f64,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(activationDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &activationDesc,
        "cudnnSetActivationDescriptorSwishBeta",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(swish_beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &swish_beta,
        "cudnnSetActivationDescriptorSwishBeta",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetActivationDescriptorSwishBeta(
    writer: &mut (impl std::io::Write + ?Sized),
    activationDesc: cuda_types::cudnn8::cudnnActivationDescriptor_t,
    swish_beta: *mut f64,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(activationDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &activationDesc,
        "cudnnGetActivationDescriptorSwishBeta",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(swish_beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &swish_beta,
        "cudnnGetActivationDescriptorSwishBeta",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnDestroyActivationDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    activationDesc: cuda_types::cudnn8::cudnnActivationDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(activationDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &activationDesc,
        "cudnnDestroyActivationDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnActivationForward(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    activationDesc: cuda_types::cudnn8::cudnnActivationDescriptor_t,
    alpha: *const ::core::ffi::c_void,
    xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    y: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnActivationForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(activationDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &activationDesc,
        "cudnnActivationForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(&alpha, "cudnnActivationForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&xDesc, "cudnnActivationForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(&x, "cudnnActivationForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(&beta, "cudnnActivationForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&yDesc, "cudnnActivationForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(&y, "cudnnActivationForward", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnCreateLRNDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    normDesc: *mut cuda_types::cudnn8::cudnnLRNDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(normDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&normDesc, "cudnnCreateLRNDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnSetLRNDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    normDesc: cuda_types::cudnn8::cudnnLRNDescriptor_t,
    lrnN: ::core::ffi::c_uint,
    lrnAlpha: f64,
    lrnBeta: f64,
    lrnK: f64,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(normDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&normDesc, "cudnnSetLRNDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(lrnN), ": ").as_bytes())?;
    crate::CudaDisplay::write(&lrnN, "cudnnSetLRNDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(lrnAlpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(&lrnAlpha, "cudnnSetLRNDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(lrnBeta), ": ").as_bytes())?;
    crate::CudaDisplay::write(&lrnBeta, "cudnnSetLRNDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(lrnK), ": ").as_bytes())?;
    crate::CudaDisplay::write(&lrnK, "cudnnSetLRNDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnGetLRNDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    normDesc: cuda_types::cudnn8::cudnnLRNDescriptor_t,
    lrnN: *mut ::core::ffi::c_uint,
    lrnAlpha: *mut f64,
    lrnBeta: *mut f64,
    lrnK: *mut f64,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(normDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&normDesc, "cudnnGetLRNDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(lrnN), ": ").as_bytes())?;
    crate::CudaDisplay::write(&lrnN, "cudnnGetLRNDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(lrnAlpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(&lrnAlpha, "cudnnGetLRNDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(lrnBeta), ": ").as_bytes())?;
    crate::CudaDisplay::write(&lrnBeta, "cudnnGetLRNDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(lrnK), ": ").as_bytes())?;
    crate::CudaDisplay::write(&lrnK, "cudnnGetLRNDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnDestroyLRNDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    lrnDesc: cuda_types::cudnn8::cudnnLRNDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(lrnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&lrnDesc, "cudnnDestroyLRNDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnLRNCrossChannelForward(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    normDesc: cuda_types::cudnn8::cudnnLRNDescriptor_t,
    lrnMode: cuda_types::cudnn8::cudnnLRNMode_t,
    alpha: *const ::core::ffi::c_void,
    xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    y: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnLRNCrossChannelForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(normDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &normDesc,
        "cudnnLRNCrossChannelForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(lrnMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&lrnMode, "cudnnLRNCrossChannelForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(&alpha, "cudnnLRNCrossChannelForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&xDesc, "cudnnLRNCrossChannelForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(&x, "cudnnLRNCrossChannelForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(&beta, "cudnnLRNCrossChannelForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&yDesc, "cudnnLRNCrossChannelForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(&y, "cudnnLRNCrossChannelForward", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnDivisiveNormalizationForward(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnDivisiveNormalizationForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(normDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &normDesc,
        "cudnnDivisiveNormalizationForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mode,
        "cudnnDivisiveNormalizationForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &alpha,
        "cudnnDivisiveNormalizationForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xDesc,
        "cudnnDivisiveNormalizationForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(&x, "cudnnDivisiveNormalizationForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(means), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &means,
        "cudnnDivisiveNormalizationForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(temp), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &temp,
        "cudnnDivisiveNormalizationForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(temp2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &temp2,
        "cudnnDivisiveNormalizationForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &beta,
        "cudnnDivisiveNormalizationForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &yDesc,
        "cudnnDivisiveNormalizationForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(&y, "cudnnDivisiveNormalizationForward", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnDeriveBNTensorDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    derivedBnDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    mode: cuda_types::cudnn8::cudnnBatchNormMode_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(derivedBnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &derivedBnDesc,
        "cudnnDeriveBNTensorDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&xDesc, "cudnnDeriveBNTensorDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mode, "cudnnDeriveBNTensorDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnBatchNormalizationForwardInference(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnBatchNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mode,
        "cudnnBatchNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &alpha,
        "cudnnBatchNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &beta,
        "cudnnBatchNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xDesc,
        "cudnnBatchNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &x,
        "cudnnBatchNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &yDesc,
        "cudnnBatchNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &y,
        "cudnnBatchNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bnScaleBiasMeanVarDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &bnScaleBiasMeanVarDesc,
        "cudnnBatchNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bnScale), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &bnScale,
        "cudnnBatchNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bnBias), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &bnBias,
        "cudnnBatchNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(estimatedMean), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &estimatedMean,
        "cudnnBatchNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(estimatedVariance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &estimatedVariance,
        "cudnnBatchNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(epsilon), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &epsilon,
        "cudnnBatchNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnDeriveNormTensorDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    derivedNormScaleBiasDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    derivedNormMeanVarDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    mode: cuda_types::cudnn8::cudnnNormMode_t,
    groupCnt: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(derivedNormScaleBiasDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &derivedNormScaleBiasDesc,
        "cudnnDeriveNormTensorDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(derivedNormMeanVarDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &derivedNormMeanVarDesc,
        "cudnnDeriveNormTensorDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xDesc,
        "cudnnDeriveNormTensorDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mode,
        "cudnnDeriveNormTensorDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(groupCnt), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &groupCnt,
        "cudnnDeriveNormTensorDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnNormalizationForwardInference(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mode,
        "cudnnNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(normOps), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &normOps,
        "cudnnNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &algo,
        "cudnnNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &alpha,
        "cudnnNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &beta,
        "cudnnNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xDesc,
        "cudnnNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &x,
        "cudnnNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(normScaleBiasDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &normScaleBiasDesc,
        "cudnnNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(normScale), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &normScale,
        "cudnnNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(normBias), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &normBias,
        "cudnnNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(normMeanVarDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &normMeanVarDesc,
        "cudnnNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(estimatedMean), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &estimatedMean,
        "cudnnNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(estimatedVariance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &estimatedVariance,
        "cudnnNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(zDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &zDesc,
        "cudnnNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(z), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &z,
        "cudnnNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(activationDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &activationDesc,
        "cudnnNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &yDesc,
        "cudnnNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &y,
        "cudnnNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(epsilon), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &epsilon,
        "cudnnNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(groupCnt), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &groupCnt,
        "cudnnNormalizationForwardInference",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnCreateSpatialTransformerDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    stDesc: *mut cuda_types::cudnn8::cudnnSpatialTransformerDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(stDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &stDesc,
        "cudnnCreateSpatialTransformerDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnSetSpatialTransformerNdDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    stDesc: cuda_types::cudnn8::cudnnSpatialTransformerDescriptor_t,
    samplerType: cuda_types::cudnn8::cudnnSamplerType_t,
    dataType: cuda_types::cudnn8::cudnnDataType_t,
    nbDims: ::core::ffi::c_int,
    dimA: *const ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(stDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &stDesc,
        "cudnnSetSpatialTransformerNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(samplerType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &samplerType,
        "cudnnSetSpatialTransformerNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dataType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dataType,
        "cudnnSetSpatialTransformerNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nbDims), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nbDims,
        "cudnnSetSpatialTransformerNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dimA), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dimA,
        "cudnnSetSpatialTransformerNdDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnDestroySpatialTransformerDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    stDesc: cuda_types::cudnn8::cudnnSpatialTransformerDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(stDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &stDesc,
        "cudnnDestroySpatialTransformerDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnSpatialTfGridGeneratorForward(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    stDesc: cuda_types::cudnn8::cudnnSpatialTransformerDescriptor_t,
    theta: *const ::core::ffi::c_void,
    grid: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnSpatialTfGridGeneratorForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(stDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &stDesc,
        "cudnnSpatialTfGridGeneratorForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(theta), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &theta,
        "cudnnSpatialTfGridGeneratorForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(grid), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &grid,
        "cudnnSpatialTfGridGeneratorForward",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnSpatialTfSamplerForward(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    stDesc: cuda_types::cudnn8::cudnnSpatialTransformerDescriptor_t,
    alpha: *const ::core::ffi::c_void,
    xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    grid: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    y: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnSpatialTfSamplerForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(stDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&stDesc, "cudnnSpatialTfSamplerForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(&alpha, "cudnnSpatialTfSamplerForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&xDesc, "cudnnSpatialTfSamplerForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(&x, "cudnnSpatialTfSamplerForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(grid), ": ").as_bytes())?;
    crate::CudaDisplay::write(&grid, "cudnnSpatialTfSamplerForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(&beta, "cudnnSpatialTfSamplerForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&yDesc, "cudnnSpatialTfSamplerForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(&y, "cudnnSpatialTfSamplerForward", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnCreateDropoutDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    dropoutDesc: *mut cuda_types::cudnn8::cudnnDropoutDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dropoutDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dropoutDesc,
        "cudnnCreateDropoutDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnDestroyDropoutDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    dropoutDesc: cuda_types::cudnn8::cudnnDropoutDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dropoutDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dropoutDesc,
        "cudnnDestroyDropoutDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnDropoutGetStatesSize(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    sizeInBytes: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnDropoutGetStatesSize", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeInBytes,
        "cudnnDropoutGetStatesSize",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnDropoutGetReserveSpaceSize(
    writer: &mut (impl std::io::Write + ?Sized),
    xdesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    sizeInBytes: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(xdesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xdesc,
        "cudnnDropoutGetReserveSpaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeInBytes,
        "cudnnDropoutGetReserveSpaceSize",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnSetDropoutDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    dropoutDesc: cuda_types::cudnn8::cudnnDropoutDescriptor_t,
    handle: cuda_types::cudnn8::cudnnHandle_t,
    dropout: f32,
    states: *mut ::core::ffi::c_void,
    stateSizeInBytes: usize,
    seed: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dropoutDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dropoutDesc,
        "cudnnSetDropoutDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnSetDropoutDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dropout), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dropout, "cudnnSetDropoutDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(states), ": ").as_bytes())?;
    crate::CudaDisplay::write(&states, "cudnnSetDropoutDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(stateSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &stateSizeInBytes,
        "cudnnSetDropoutDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(seed), ": ").as_bytes())?;
    crate::CudaDisplay::write(&seed, "cudnnSetDropoutDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnRestoreDropoutDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    dropoutDesc: cuda_types::cudnn8::cudnnDropoutDescriptor_t,
    handle: cuda_types::cudnn8::cudnnHandle_t,
    dropout: f32,
    states: *mut ::core::ffi::c_void,
    stateSizeInBytes: usize,
    seed: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dropoutDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dropoutDesc,
        "cudnnRestoreDropoutDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnRestoreDropoutDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dropout), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dropout,
        "cudnnRestoreDropoutDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(states), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &states,
        "cudnnRestoreDropoutDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(stateSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &stateSizeInBytes,
        "cudnnRestoreDropoutDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(seed), ": ").as_bytes())?;
    crate::CudaDisplay::write(&seed, "cudnnRestoreDropoutDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnGetDropoutDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    dropoutDesc: cuda_types::cudnn8::cudnnDropoutDescriptor_t,
    handle: cuda_types::cudnn8::cudnnHandle_t,
    dropout: *mut f32,
    states: *mut *mut ::core::ffi::c_void,
    seed: *mut ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dropoutDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dropoutDesc,
        "cudnnGetDropoutDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnGetDropoutDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dropout), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dropout, "cudnnGetDropoutDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(states), ": ").as_bytes())?;
    crate::CudaDisplay::write(&states, "cudnnGetDropoutDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(seed), ": ").as_bytes())?;
    crate::CudaDisplay::write(&seed, "cudnnGetDropoutDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnDropoutForward(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    dropoutDesc: cuda_types::cudnn8::cudnnDropoutDescriptor_t,
    xdesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    ydesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    y: *mut ::core::ffi::c_void,
    reserveSpace: *mut ::core::ffi::c_void,
    reserveSpaceSizeInBytes: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnDropoutForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dropoutDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dropoutDesc, "cudnnDropoutForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xdesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&xdesc, "cudnnDropoutForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(&x, "cudnnDropoutForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ydesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ydesc, "cudnnDropoutForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(&y, "cudnnDropoutForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(&reserveSpace, "cudnnDropoutForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpaceSizeInBytes,
        "cudnnDropoutForward",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::cudnn8::cudnnAlgorithmDescriptor_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        if self.is_null() {
            writer.write_all(b"NULL")
        } else {
            write!(writer, "{:p}", *self)
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn8::cudnnAlgorithmPerformance_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        if self.is_null() {
            writer.write_all(b"NULL")
        } else {
            write!(writer, "{:p}", *self)
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn8::cudnnAlgorithmUnionStruct {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(algo), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.algo, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
pub fn write_cudnnCreateAlgorithmDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    algoDesc: *mut cuda_types::cudnn8::cudnnAlgorithmDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(algoDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &algoDesc,
        "cudnnCreateAlgorithmDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnSetAlgorithmDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    algoDesc: cuda_types::cudnn8::cudnnAlgorithmDescriptor_t,
    algorithm: cuda_types::cudnn8::cudnnAlgorithm_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(algoDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &algoDesc,
        "cudnnSetAlgorithmDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algorithm), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &algorithm,
        "cudnnSetAlgorithmDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetAlgorithmDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    algoDesc: cuda_types::cudnn8::cudnnAlgorithmDescriptor_t,
    algorithm: *mut cuda_types::cudnn8::cudnnAlgorithm_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(algoDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &algoDesc,
        "cudnnGetAlgorithmDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algorithm), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &algorithm,
        "cudnnGetAlgorithmDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnCopyAlgorithmDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    src: cuda_types::cudnn8::cudnnAlgorithmDescriptor_t,
    dest: cuda_types::cudnn8::cudnnAlgorithmDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(src), ": ").as_bytes())?;
    crate::CudaDisplay::write(&src, "cudnnCopyAlgorithmDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dest), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dest, "cudnnCopyAlgorithmDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnDestroyAlgorithmDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    algoDesc: cuda_types::cudnn8::cudnnAlgorithmDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(algoDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &algoDesc,
        "cudnnDestroyAlgorithmDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnCreateAlgorithmPerformance(
    writer: &mut (impl std::io::Write + ?Sized),
    algoPerf: *mut cuda_types::cudnn8::cudnnAlgorithmPerformance_t,
    numberToCreate: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(algoPerf), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &algoPerf,
        "cudnnCreateAlgorithmPerformance",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numberToCreate), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numberToCreate,
        "cudnnCreateAlgorithmPerformance",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnSetAlgorithmPerformance(
    writer: &mut (impl std::io::Write + ?Sized),
    algoPerf: cuda_types::cudnn8::cudnnAlgorithmPerformance_t,
    algoDesc: cuda_types::cudnn8::cudnnAlgorithmDescriptor_t,
    status: cuda_types::cudnn8::cudnnStatus_t,
    time: f32,
    memory: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(algoPerf), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &algoPerf,
        "cudnnSetAlgorithmPerformance",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algoDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &algoDesc,
        "cudnnSetAlgorithmPerformance",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(status), ": ").as_bytes())?;
    crate::CudaDisplay::write(&status, "cudnnSetAlgorithmPerformance", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(time), ": ").as_bytes())?;
    crate::CudaDisplay::write(&time, "cudnnSetAlgorithmPerformance", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(memory), ": ").as_bytes())?;
    crate::CudaDisplay::write(&memory, "cudnnSetAlgorithmPerformance", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnGetAlgorithmPerformance(
    writer: &mut (impl std::io::Write + ?Sized),
    algoPerf: cuda_types::cudnn8::cudnnAlgorithmPerformance_t,
    algoDesc: *mut cuda_types::cudnn8::cudnnAlgorithmDescriptor_t,
    status: *mut cuda_types::cudnn8::cudnnStatus_t,
    time: *mut f32,
    memory: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(algoPerf), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &algoPerf,
        "cudnnGetAlgorithmPerformance",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algoDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &algoDesc,
        "cudnnGetAlgorithmPerformance",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(status), ": ").as_bytes())?;
    crate::CudaDisplay::write(&status, "cudnnGetAlgorithmPerformance", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(time), ": ").as_bytes())?;
    crate::CudaDisplay::write(&time, "cudnnGetAlgorithmPerformance", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(memory), ": ").as_bytes())?;
    crate::CudaDisplay::write(&memory, "cudnnGetAlgorithmPerformance", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnDestroyAlgorithmPerformance(
    writer: &mut (impl std::io::Write + ?Sized),
    algoPerf: *mut cuda_types::cudnn8::cudnnAlgorithmPerformance_t,
    numberToDestroy: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(algoPerf), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &algoPerf,
        "cudnnDestroyAlgorithmPerformance",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numberToDestroy), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numberToDestroy,
        "cudnnDestroyAlgorithmPerformance",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetAlgorithmSpaceSize(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    algoDesc: cuda_types::cudnn8::cudnnAlgorithmDescriptor_t,
    algoSpaceSizeInBytes: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnGetAlgorithmSpaceSize", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algoDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&algoDesc, "cudnnGetAlgorithmSpaceSize", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algoSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &algoSpaceSizeInBytes,
        "cudnnGetAlgorithmSpaceSize",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnSaveAlgorithm(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    algoDesc: cuda_types::cudnn8::cudnnAlgorithmDescriptor_t,
    algoSpace: *mut ::core::ffi::c_void,
    algoSpaceSizeInBytes: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnSaveAlgorithm", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algoDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&algoDesc, "cudnnSaveAlgorithm", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algoSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(&algoSpace, "cudnnSaveAlgorithm", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algoSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &algoSpaceSizeInBytes,
        "cudnnSaveAlgorithm",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnRestoreAlgorithm(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    algoSpace: *mut ::core::ffi::c_void,
    algoSpaceSizeInBytes: usize,
    algoDesc: cuda_types::cudnn8::cudnnAlgorithmDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnRestoreAlgorithm", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algoSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(&algoSpace, "cudnnRestoreAlgorithm", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algoSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &algoSpaceSizeInBytes,
        "cudnnRestoreAlgorithm",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algoDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&algoDesc, "cudnnRestoreAlgorithm", arg_idx, writer)?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::cudnn8::cudnnDebugStruct {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(cudnn_version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.cudnn_version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(cudnnStatus), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.cudnnStatus, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(time_sec), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.time_sec, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(time_usec), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.time_usec, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(time_delta), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.time_delta, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(handle), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.handle, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(stream), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.stream, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(pid), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.pid, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(tid), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.tid, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(cudaDeviceId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.cudaDeviceId, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cudnn8::cudnnCallback_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(
            writer,
            "{:p}",
            unsafe {
                std::mem::transmute::<
                    cuda_types::cudnn8::cudnnCallback_t,
                    *mut ::std::ffi::c_void,
                >(*self)
            },
        )
    }
}
pub fn write_cudnnSetCallback(
    writer: &mut (impl std::io::Write + ?Sized),
    mask: ::core::ffi::c_uint,
    udata: *mut ::core::ffi::c_void,
    fptr: cuda_types::cudnn8::cudnnCallback_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(mask), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mask, "cudnnSetCallback", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(udata), ": ").as_bytes())?;
    crate::CudaDisplay::write(&udata, "cudnnSetCallback", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(fptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&fptr, "cudnnSetCallback", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnGetCallback(
    writer: &mut (impl std::io::Write + ?Sized),
    mask: *mut ::core::ffi::c_uint,
    udata: *mut *mut ::core::ffi::c_void,
    fptr: *mut cuda_types::cudnn8::cudnnCallback_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(mask), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mask, "cudnnGetCallback", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(udata), ": ").as_bytes())?;
    crate::CudaDisplay::write(&udata, "cudnnGetCallback", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(fptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&fptr, "cudnnGetCallback", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnOpsInferVersionCheck(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
pub fn write_cudnnSoftmaxBackward(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnSoftmaxBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algo), ": ").as_bytes())?;
    crate::CudaDisplay::write(&algo, "cudnnSoftmaxBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mode, "cudnnSoftmaxBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(&alpha, "cudnnSoftmaxBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&yDesc, "cudnnSoftmaxBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(&y, "cudnnSoftmaxBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dyDesc, "cudnnSoftmaxBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dy, "cudnnSoftmaxBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(&beta, "cudnnSoftmaxBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dxDesc, "cudnnSoftmaxBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dx, "cudnnSoftmaxBackward", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnPoolingBackward(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnPoolingBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(poolingDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&poolingDesc, "cudnnPoolingBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(&alpha, "cudnnPoolingBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&yDesc, "cudnnPoolingBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(&y, "cudnnPoolingBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dyDesc, "cudnnPoolingBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dy, "cudnnPoolingBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&xDesc, "cudnnPoolingBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(&x, "cudnnPoolingBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(&beta, "cudnnPoolingBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dxDesc, "cudnnPoolingBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dx, "cudnnPoolingBackward", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnActivationBackward(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnActivationBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(activationDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &activationDesc,
        "cudnnActivationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(&alpha, "cudnnActivationBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&yDesc, "cudnnActivationBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(&y, "cudnnActivationBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dyDesc, "cudnnActivationBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dy, "cudnnActivationBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&xDesc, "cudnnActivationBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(&x, "cudnnActivationBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(&beta, "cudnnActivationBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dxDesc, "cudnnActivationBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dx, "cudnnActivationBackward", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnLRNCrossChannelBackward(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnLRNCrossChannelBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(normDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &normDesc,
        "cudnnLRNCrossChannelBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(lrnMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &lrnMode,
        "cudnnLRNCrossChannelBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(&alpha, "cudnnLRNCrossChannelBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&yDesc, "cudnnLRNCrossChannelBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(&y, "cudnnLRNCrossChannelBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dyDesc, "cudnnLRNCrossChannelBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dy, "cudnnLRNCrossChannelBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&xDesc, "cudnnLRNCrossChannelBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(&x, "cudnnLRNCrossChannelBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(&beta, "cudnnLRNCrossChannelBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dxDesc, "cudnnLRNCrossChannelBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dx, "cudnnLRNCrossChannelBackward", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnDivisiveNormalizationBackward(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnDivisiveNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(normDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &normDesc,
        "cudnnDivisiveNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mode,
        "cudnnDivisiveNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &alpha,
        "cudnnDivisiveNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xDesc,
        "cudnnDivisiveNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &x,
        "cudnnDivisiveNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(means), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &means,
        "cudnnDivisiveNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dy), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dy,
        "cudnnDivisiveNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(temp), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &temp,
        "cudnnDivisiveNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(temp2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &temp2,
        "cudnnDivisiveNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &beta,
        "cudnnDivisiveNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dXdMeansDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dXdMeansDesc,
        "cudnnDivisiveNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dx), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dx,
        "cudnnDivisiveNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dMeans), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dMeans,
        "cudnnDivisiveNormalizationBackward",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    mode: cuda_types::cudnn8::cudnnBatchNormMode_t,
    bnOps: cuda_types::cudnn8::cudnnBatchNormOps_t,
    xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    zDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    bnScaleBiasMeanVarDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    activationDesc: cuda_types::cudnn8::cudnnActivationDescriptor_t,
    sizeInBytes: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mode,
        "cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bnOps), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &bnOps,
        "cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xDesc,
        "cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(zDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &zDesc,
        "cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &yDesc,
        "cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bnScaleBiasMeanVarDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &bnScaleBiasMeanVarDesc,
        "cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(activationDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &activationDesc,
        "cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeInBytes,
        "cudnnGetBatchNormalizationForwardTrainingExWorkspaceSize",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetBatchNormalizationBackwardExWorkspaceSize(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnGetBatchNormalizationBackwardExWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mode,
        "cudnnGetBatchNormalizationBackwardExWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bnOps), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &bnOps,
        "cudnnGetBatchNormalizationBackwardExWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xDesc,
        "cudnnGetBatchNormalizationBackwardExWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &yDesc,
        "cudnnGetBatchNormalizationBackwardExWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dyDesc,
        "cudnnGetBatchNormalizationBackwardExWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dzDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dzDesc,
        "cudnnGetBatchNormalizationBackwardExWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dxDesc,
        "cudnnGetBatchNormalizationBackwardExWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dBnScaleBiasDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dBnScaleBiasDesc,
        "cudnnGetBatchNormalizationBackwardExWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(activationDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &activationDesc,
        "cudnnGetBatchNormalizationBackwardExWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeInBytes,
        "cudnnGetBatchNormalizationBackwardExWorkspaceSize",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetBatchNormalizationTrainingExReserveSpaceSize(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    mode: cuda_types::cudnn8::cudnnBatchNormMode_t,
    bnOps: cuda_types::cudnn8::cudnnBatchNormOps_t,
    activationDesc: cuda_types::cudnn8::cudnnActivationDescriptor_t,
    xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    sizeInBytes: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnGetBatchNormalizationTrainingExReserveSpaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mode,
        "cudnnGetBatchNormalizationTrainingExReserveSpaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bnOps), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &bnOps,
        "cudnnGetBatchNormalizationTrainingExReserveSpaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(activationDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &activationDesc,
        "cudnnGetBatchNormalizationTrainingExReserveSpaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xDesc,
        "cudnnGetBatchNormalizationTrainingExReserveSpaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeInBytes,
        "cudnnGetBatchNormalizationTrainingExReserveSpaceSize",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnBatchNormalizationForwardTraining(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnBatchNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mode,
        "cudnnBatchNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &alpha,
        "cudnnBatchNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &beta,
        "cudnnBatchNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xDesc,
        "cudnnBatchNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &x,
        "cudnnBatchNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &yDesc,
        "cudnnBatchNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &y,
        "cudnnBatchNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bnScaleBiasMeanVarDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &bnScaleBiasMeanVarDesc,
        "cudnnBatchNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bnScale), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &bnScale,
        "cudnnBatchNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bnBias), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &bnBias,
        "cudnnBatchNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(exponentialAverageFactor), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &exponentialAverageFactor,
        "cudnnBatchNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(resultRunningMean), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &resultRunningMean,
        "cudnnBatchNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(resultRunningVariance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &resultRunningVariance,
        "cudnnBatchNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(epsilon), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &epsilon,
        "cudnnBatchNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(resultSaveMean), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &resultSaveMean,
        "cudnnBatchNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(resultSaveInvVariance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &resultSaveInvVariance,
        "cudnnBatchNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnBatchNormalizationForwardTrainingEx(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnBatchNormalizationForwardTrainingEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mode,
        "cudnnBatchNormalizationForwardTrainingEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bnOps), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &bnOps,
        "cudnnBatchNormalizationForwardTrainingEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &alpha,
        "cudnnBatchNormalizationForwardTrainingEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &beta,
        "cudnnBatchNormalizationForwardTrainingEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xDesc,
        "cudnnBatchNormalizationForwardTrainingEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xData), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xData,
        "cudnnBatchNormalizationForwardTrainingEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(zDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &zDesc,
        "cudnnBatchNormalizationForwardTrainingEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(zData), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &zData,
        "cudnnBatchNormalizationForwardTrainingEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &yDesc,
        "cudnnBatchNormalizationForwardTrainingEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yData), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &yData,
        "cudnnBatchNormalizationForwardTrainingEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bnScaleBiasMeanVarDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &bnScaleBiasMeanVarDesc,
        "cudnnBatchNormalizationForwardTrainingEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bnScale), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &bnScale,
        "cudnnBatchNormalizationForwardTrainingEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bnBias), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &bnBias,
        "cudnnBatchNormalizationForwardTrainingEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(exponentialAverageFactor), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &exponentialAverageFactor,
        "cudnnBatchNormalizationForwardTrainingEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(resultRunningMean), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &resultRunningMean,
        "cudnnBatchNormalizationForwardTrainingEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(resultRunningVariance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &resultRunningVariance,
        "cudnnBatchNormalizationForwardTrainingEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(epsilon), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &epsilon,
        "cudnnBatchNormalizationForwardTrainingEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(resultSaveMean), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &resultSaveMean,
        "cudnnBatchNormalizationForwardTrainingEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(resultSaveInvVariance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &resultSaveInvVariance,
        "cudnnBatchNormalizationForwardTrainingEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(activationDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &activationDesc,
        "cudnnBatchNormalizationForwardTrainingEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workspace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workspace,
        "cudnnBatchNormalizationForwardTrainingEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSizeInBytes,
        "cudnnBatchNormalizationForwardTrainingEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpace,
        "cudnnBatchNormalizationForwardTrainingEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpaceSizeInBytes,
        "cudnnBatchNormalizationForwardTrainingEx",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnBatchNormalizationBackward(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnBatchNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mode,
        "cudnnBatchNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alphaDataDiff), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &alphaDataDiff,
        "cudnnBatchNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(betaDataDiff), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &betaDataDiff,
        "cudnnBatchNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alphaParamDiff), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &alphaParamDiff,
        "cudnnBatchNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(betaParamDiff), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &betaParamDiff,
        "cudnnBatchNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xDesc,
        "cudnnBatchNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(&x, "cudnnBatchNormalizationBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dyDesc,
        "cudnnBatchNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dy, "cudnnBatchNormalizationBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dxDesc,
        "cudnnBatchNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dx, "cudnnBatchNormalizationBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dBnScaleBiasDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dBnScaleBiasDesc,
        "cudnnBatchNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bnScale), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &bnScale,
        "cudnnBatchNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dBnScaleResult), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dBnScaleResult,
        "cudnnBatchNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dBnBiasResult), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dBnBiasResult,
        "cudnnBatchNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(epsilon), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &epsilon,
        "cudnnBatchNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(savedMean), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &savedMean,
        "cudnnBatchNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(savedInvVariance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &savedInvVariance,
        "cudnnBatchNormalizationBackward",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnBatchNormalizationBackwardEx(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnBatchNormalizationBackwardEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mode,
        "cudnnBatchNormalizationBackwardEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bnOps), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &bnOps,
        "cudnnBatchNormalizationBackwardEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alphaDataDiff), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &alphaDataDiff,
        "cudnnBatchNormalizationBackwardEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(betaDataDiff), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &betaDataDiff,
        "cudnnBatchNormalizationBackwardEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alphaParamDiff), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &alphaParamDiff,
        "cudnnBatchNormalizationBackwardEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(betaParamDiff), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &betaParamDiff,
        "cudnnBatchNormalizationBackwardEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xDesc,
        "cudnnBatchNormalizationBackwardEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xData), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xData,
        "cudnnBatchNormalizationBackwardEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &yDesc,
        "cudnnBatchNormalizationBackwardEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yData), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &yData,
        "cudnnBatchNormalizationBackwardEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dyDesc,
        "cudnnBatchNormalizationBackwardEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dyData), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dyData,
        "cudnnBatchNormalizationBackwardEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dzDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dzDesc,
        "cudnnBatchNormalizationBackwardEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dzData), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dzData,
        "cudnnBatchNormalizationBackwardEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dxDesc,
        "cudnnBatchNormalizationBackwardEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dxData), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dxData,
        "cudnnBatchNormalizationBackwardEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dBnScaleBiasDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dBnScaleBiasDesc,
        "cudnnBatchNormalizationBackwardEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bnScaleData), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &bnScaleData,
        "cudnnBatchNormalizationBackwardEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bnBiasData), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &bnBiasData,
        "cudnnBatchNormalizationBackwardEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dBnScaleData), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dBnScaleData,
        "cudnnBatchNormalizationBackwardEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dBnBiasData), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dBnBiasData,
        "cudnnBatchNormalizationBackwardEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(epsilon), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &epsilon,
        "cudnnBatchNormalizationBackwardEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(savedMean), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &savedMean,
        "cudnnBatchNormalizationBackwardEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(savedInvVariance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &savedInvVariance,
        "cudnnBatchNormalizationBackwardEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(activationDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &activationDesc,
        "cudnnBatchNormalizationBackwardEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpace,
        "cudnnBatchNormalizationBackwardEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSizeInBytes,
        "cudnnBatchNormalizationBackwardEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpace,
        "cudnnBatchNormalizationBackwardEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpaceSizeInBytes,
        "cudnnBatchNormalizationBackwardEx",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetNormalizationForwardTrainingWorkspaceSize(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnGetNormalizationForwardTrainingWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mode,
        "cudnnGetNormalizationForwardTrainingWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(normOps), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &normOps,
        "cudnnGetNormalizationForwardTrainingWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &algo,
        "cudnnGetNormalizationForwardTrainingWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xDesc,
        "cudnnGetNormalizationForwardTrainingWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(zDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &zDesc,
        "cudnnGetNormalizationForwardTrainingWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &yDesc,
        "cudnnGetNormalizationForwardTrainingWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(normScaleBiasDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &normScaleBiasDesc,
        "cudnnGetNormalizationForwardTrainingWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(activationDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &activationDesc,
        "cudnnGetNormalizationForwardTrainingWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(normMeanVarDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &normMeanVarDesc,
        "cudnnGetNormalizationForwardTrainingWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeInBytes,
        "cudnnGetNormalizationForwardTrainingWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(groupCnt), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &groupCnt,
        "cudnnGetNormalizationForwardTrainingWorkspaceSize",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetNormalizationBackwardWorkspaceSize(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnGetNormalizationBackwardWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mode,
        "cudnnGetNormalizationBackwardWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(normOps), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &normOps,
        "cudnnGetNormalizationBackwardWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &algo,
        "cudnnGetNormalizationBackwardWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xDesc,
        "cudnnGetNormalizationBackwardWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &yDesc,
        "cudnnGetNormalizationBackwardWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dyDesc,
        "cudnnGetNormalizationBackwardWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dzDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dzDesc,
        "cudnnGetNormalizationBackwardWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dxDesc,
        "cudnnGetNormalizationBackwardWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dNormScaleBiasDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dNormScaleBiasDesc,
        "cudnnGetNormalizationBackwardWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(activationDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &activationDesc,
        "cudnnGetNormalizationBackwardWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(normMeanVarDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &normMeanVarDesc,
        "cudnnGetNormalizationBackwardWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeInBytes,
        "cudnnGetNormalizationBackwardWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(groupCnt), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &groupCnt,
        "cudnnGetNormalizationBackwardWorkspaceSize",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetNormalizationTrainingReserveSpaceSize(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    mode: cuda_types::cudnn8::cudnnNormMode_t,
    normOps: cuda_types::cudnn8::cudnnNormOps_t,
    algo: cuda_types::cudnn8::cudnnNormAlgo_t,
    activationDesc: cuda_types::cudnn8::cudnnActivationDescriptor_t,
    xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    sizeInBytes: *mut usize,
    groupCnt: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnGetNormalizationTrainingReserveSpaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mode,
        "cudnnGetNormalizationTrainingReserveSpaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(normOps), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &normOps,
        "cudnnGetNormalizationTrainingReserveSpaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &algo,
        "cudnnGetNormalizationTrainingReserveSpaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(activationDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &activationDesc,
        "cudnnGetNormalizationTrainingReserveSpaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xDesc,
        "cudnnGetNormalizationTrainingReserveSpaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeInBytes,
        "cudnnGetNormalizationTrainingReserveSpaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(groupCnt), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &groupCnt,
        "cudnnGetNormalizationTrainingReserveSpaceSize",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnNormalizationForwardTraining(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mode,
        "cudnnNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(normOps), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &normOps,
        "cudnnNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &algo,
        "cudnnNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &alpha,
        "cudnnNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &beta,
        "cudnnNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xDesc,
        "cudnnNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xData), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xData,
        "cudnnNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(normScaleBiasDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &normScaleBiasDesc,
        "cudnnNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(normScale), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &normScale,
        "cudnnNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(normBias), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &normBias,
        "cudnnNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(exponentialAverageFactor), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &exponentialAverageFactor,
        "cudnnNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(normMeanVarDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &normMeanVarDesc,
        "cudnnNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(resultRunningMean), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &resultRunningMean,
        "cudnnNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(resultRunningVariance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &resultRunningVariance,
        "cudnnNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(epsilon), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &epsilon,
        "cudnnNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(resultSaveMean), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &resultSaveMean,
        "cudnnNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(resultSaveInvVariance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &resultSaveInvVariance,
        "cudnnNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(activationDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &activationDesc,
        "cudnnNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(zDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &zDesc,
        "cudnnNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(zData), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &zData,
        "cudnnNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &yDesc,
        "cudnnNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yData), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &yData,
        "cudnnNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workspace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workspace,
        "cudnnNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSizeInBytes,
        "cudnnNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpace,
        "cudnnNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpaceSizeInBytes,
        "cudnnNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(groupCnt), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &groupCnt,
        "cudnnNormalizationForwardTraining",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnNormalizationBackward(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnNormalizationBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mode, "cudnnNormalizationBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(normOps), ": ").as_bytes())?;
    crate::CudaDisplay::write(&normOps, "cudnnNormalizationBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algo), ": ").as_bytes())?;
    crate::CudaDisplay::write(&algo, "cudnnNormalizationBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alphaDataDiff), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &alphaDataDiff,
        "cudnnNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(betaDataDiff), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &betaDataDiff,
        "cudnnNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alphaParamDiff), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &alphaParamDiff,
        "cudnnNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(betaParamDiff), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &betaParamDiff,
        "cudnnNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&xDesc, "cudnnNormalizationBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xData), ": ").as_bytes())?;
    crate::CudaDisplay::write(&xData, "cudnnNormalizationBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&yDesc, "cudnnNormalizationBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yData), ": ").as_bytes())?;
    crate::CudaDisplay::write(&yData, "cudnnNormalizationBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dyDesc, "cudnnNormalizationBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dyData), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dyData, "cudnnNormalizationBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dzDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dzDesc, "cudnnNormalizationBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dzData), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dzData, "cudnnNormalizationBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dxDesc, "cudnnNormalizationBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dxData), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dxData, "cudnnNormalizationBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dNormScaleBiasDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dNormScaleBiasDesc,
        "cudnnNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(normScaleData), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &normScaleData,
        "cudnnNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(normBiasData), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &normBiasData,
        "cudnnNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dNormScaleData), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dNormScaleData,
        "cudnnNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dNormBiasData), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dNormBiasData,
        "cudnnNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(epsilon), ": ").as_bytes())?;
    crate::CudaDisplay::write(&epsilon, "cudnnNormalizationBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(normMeanVarDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &normMeanVarDesc,
        "cudnnNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(savedMean), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &savedMean,
        "cudnnNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(savedInvVariance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &savedInvVariance,
        "cudnnNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(activationDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &activationDesc,
        "cudnnNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpace,
        "cudnnNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSizeInBytes,
        "cudnnNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpace,
        "cudnnNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpaceSizeInBytes,
        "cudnnNormalizationBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(groupCnt), ": ").as_bytes())?;
    crate::CudaDisplay::write(&groupCnt, "cudnnNormalizationBackward", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnSpatialTfGridGeneratorBackward(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    stDesc: cuda_types::cudnn8::cudnnSpatialTransformerDescriptor_t,
    dgrid: *const ::core::ffi::c_void,
    dtheta: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnSpatialTfGridGeneratorBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(stDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &stDesc,
        "cudnnSpatialTfGridGeneratorBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dgrid), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dgrid,
        "cudnnSpatialTfGridGeneratorBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dtheta), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dtheta,
        "cudnnSpatialTfGridGeneratorBackward",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnSpatialTfSamplerBackward(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnSpatialTfSamplerBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(stDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &stDesc,
        "cudnnSpatialTfSamplerBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(&alpha, "cudnnSpatialTfSamplerBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&xDesc, "cudnnSpatialTfSamplerBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(&x, "cudnnSpatialTfSamplerBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(&beta, "cudnnSpatialTfSamplerBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dxDesc,
        "cudnnSpatialTfSamplerBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dx, "cudnnSpatialTfSamplerBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alphaDgrid), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &alphaDgrid,
        "cudnnSpatialTfSamplerBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dyDesc,
        "cudnnSpatialTfSamplerBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dy, "cudnnSpatialTfSamplerBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(grid), ": ").as_bytes())?;
    crate::CudaDisplay::write(&grid, "cudnnSpatialTfSamplerBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(betaDgrid), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &betaDgrid,
        "cudnnSpatialTfSamplerBackward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dgrid), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dgrid, "cudnnSpatialTfSamplerBackward", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnDropoutBackward(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    dropoutDesc: cuda_types::cudnn8::cudnnDropoutDescriptor_t,
    dydesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    dy: *const ::core::ffi::c_void,
    dxdesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    dx: *mut ::core::ffi::c_void,
    reserveSpace: *mut ::core::ffi::c_void,
    reserveSpaceSizeInBytes: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnDropoutBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dropoutDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dropoutDesc, "cudnnDropoutBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dydesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dydesc, "cudnnDropoutBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dy, "cudnnDropoutBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dxdesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dxdesc, "cudnnDropoutBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dx, "cudnnDropoutBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(&reserveSpace, "cudnnDropoutBackward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpaceSizeInBytes,
        "cudnnDropoutBackward",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnOpsTrainVersionCheck(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
impl crate::CudaDisplay for cuda_types::cudnn8::cudnnPersistentRNNPlan_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        if self.is_null() {
            writer.write_all(b"NULL")
        } else {
            write!(writer, "{:p}", *self)
        }
    }
}
pub fn write_cudnnCreateRNNDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    rnnDesc: *mut cuda_types::cudnn8::cudnnRNNDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnCreateRNNDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnDestroyRNNDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnDestroyRNNDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnSetRNNDescriptor_v8(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnSetRNNDescriptor_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algo), ": ").as_bytes())?;
    crate::CudaDisplay::write(&algo, "cudnnSetRNNDescriptor_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cellMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cellMode, "cudnnSetRNNDescriptor_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(biasMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&biasMode, "cudnnSetRNNDescriptor_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dirMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dirMode, "cudnnSetRNNDescriptor_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(inputMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&inputMode, "cudnnSetRNNDescriptor_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dataType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dataType, "cudnnSetRNNDescriptor_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mathPrec), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mathPrec, "cudnnSetRNNDescriptor_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mathType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mathType, "cudnnSetRNNDescriptor_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(inputSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&inputSize, "cudnnSetRNNDescriptor_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hiddenSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hiddenSize, "cudnnSetRNNDescriptor_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(projSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&projSize, "cudnnSetRNNDescriptor_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numLayers), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numLayers, "cudnnSetRNNDescriptor_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dropoutDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dropoutDesc,
        "cudnnSetRNNDescriptor_v8",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(auxFlags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&auxFlags, "cudnnSetRNNDescriptor_v8", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnGetRNNDescriptor_v8(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnGetRNNDescriptor_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algo), ": ").as_bytes())?;
    crate::CudaDisplay::write(&algo, "cudnnGetRNNDescriptor_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cellMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cellMode, "cudnnGetRNNDescriptor_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(biasMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&biasMode, "cudnnGetRNNDescriptor_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dirMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dirMode, "cudnnGetRNNDescriptor_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(inputMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&inputMode, "cudnnGetRNNDescriptor_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dataType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dataType, "cudnnGetRNNDescriptor_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mathPrec), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mathPrec, "cudnnGetRNNDescriptor_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mathType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mathType, "cudnnGetRNNDescriptor_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(inputSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&inputSize, "cudnnGetRNNDescriptor_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hiddenSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hiddenSize, "cudnnGetRNNDescriptor_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(projSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&projSize, "cudnnGetRNNDescriptor_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numLayers), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numLayers, "cudnnGetRNNDescriptor_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dropoutDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dropoutDesc,
        "cudnnGetRNNDescriptor_v8",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(auxFlags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&auxFlags, "cudnnGetRNNDescriptor_v8", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnSetRNNDescriptor_v6(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnSetRNNDescriptor_v6", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnSetRNNDescriptor_v6", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hiddenSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hiddenSize, "cudnnSetRNNDescriptor_v6", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numLayers), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numLayers, "cudnnSetRNNDescriptor_v6", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dropoutDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dropoutDesc,
        "cudnnSetRNNDescriptor_v6",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(inputMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&inputMode, "cudnnSetRNNDescriptor_v6", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(direction), ": ").as_bytes())?;
    crate::CudaDisplay::write(&direction, "cudnnSetRNNDescriptor_v6", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cellMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cellMode, "cudnnSetRNNDescriptor_v6", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algo), ": ").as_bytes())?;
    crate::CudaDisplay::write(&algo, "cudnnSetRNNDescriptor_v6", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mathPrec), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mathPrec, "cudnnSetRNNDescriptor_v6", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnGetRNNDescriptor_v6(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnGetRNNDescriptor_v6", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnGetRNNDescriptor_v6", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hiddenSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hiddenSize, "cudnnGetRNNDescriptor_v6", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numLayers), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numLayers, "cudnnGetRNNDescriptor_v6", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dropoutDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dropoutDesc,
        "cudnnGetRNNDescriptor_v6",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(inputMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&inputMode, "cudnnGetRNNDescriptor_v6", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(direction), ": ").as_bytes())?;
    crate::CudaDisplay::write(&direction, "cudnnGetRNNDescriptor_v6", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cellMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cellMode, "cudnnGetRNNDescriptor_v6", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algo), ": ").as_bytes())?;
    crate::CudaDisplay::write(&algo, "cudnnGetRNNDescriptor_v6", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mathPrec), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mathPrec, "cudnnGetRNNDescriptor_v6", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnSetRNNMatrixMathType(
    writer: &mut (impl std::io::Write + ?Sized),
    rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
    mType: cuda_types::cudnn8::cudnnMathType_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnSetRNNMatrixMathType", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mType, "cudnnSetRNNMatrixMathType", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnGetRNNMatrixMathType(
    writer: &mut (impl std::io::Write + ?Sized),
    rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
    mType: *mut cuda_types::cudnn8::cudnnMathType_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnGetRNNMatrixMathType", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mType, "cudnnGetRNNMatrixMathType", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnSetRNNBiasMode(
    writer: &mut (impl std::io::Write + ?Sized),
    rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
    biasMode: cuda_types::cudnn8::cudnnRNNBiasMode_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnSetRNNBiasMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(biasMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&biasMode, "cudnnSetRNNBiasMode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnGetRNNBiasMode(
    writer: &mut (impl std::io::Write + ?Sized),
    rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
    biasMode: *mut cuda_types::cudnn8::cudnnRNNBiasMode_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnGetRNNBiasMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(biasMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&biasMode, "cudnnGetRNNBiasMode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnRNNSetClip_v8(
    writer: &mut (impl std::io::Write + ?Sized),
    rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
    clipMode: cuda_types::cudnn8::cudnnRNNClipMode_t,
    clipNanOpt: cuda_types::cudnn8::cudnnNanPropagation_t,
    lclip: f64,
    rclip: f64,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnRNNSetClip_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(clipMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&clipMode, "cudnnRNNSetClip_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(clipNanOpt), ": ").as_bytes())?;
    crate::CudaDisplay::write(&clipNanOpt, "cudnnRNNSetClip_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(lclip), ": ").as_bytes())?;
    crate::CudaDisplay::write(&lclip, "cudnnRNNSetClip_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rclip), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rclip, "cudnnRNNSetClip_v8", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnRNNGetClip_v8(
    writer: &mut (impl std::io::Write + ?Sized),
    rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
    clipMode: *mut cuda_types::cudnn8::cudnnRNNClipMode_t,
    clipNanOpt: *mut cuda_types::cudnn8::cudnnNanPropagation_t,
    lclip: *mut f64,
    rclip: *mut f64,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnRNNGetClip_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(clipMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&clipMode, "cudnnRNNGetClip_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(clipNanOpt), ": ").as_bytes())?;
    crate::CudaDisplay::write(&clipNanOpt, "cudnnRNNGetClip_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(lclip), ": ").as_bytes())?;
    crate::CudaDisplay::write(&lclip, "cudnnRNNGetClip_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rclip), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rclip, "cudnnRNNGetClip_v8", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnRNNSetClip(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
    clipMode: cuda_types::cudnn8::cudnnRNNClipMode_t,
    clipNanOpt: cuda_types::cudnn8::cudnnNanPropagation_t,
    lclip: f64,
    rclip: f64,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnRNNSetClip", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnRNNSetClip", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(clipMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&clipMode, "cudnnRNNSetClip", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(clipNanOpt), ": ").as_bytes())?;
    crate::CudaDisplay::write(&clipNanOpt, "cudnnRNNSetClip", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(lclip), ": ").as_bytes())?;
    crate::CudaDisplay::write(&lclip, "cudnnRNNSetClip", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rclip), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rclip, "cudnnRNNSetClip", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnRNNGetClip(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
    clipMode: *mut cuda_types::cudnn8::cudnnRNNClipMode_t,
    clipNanOpt: *mut cuda_types::cudnn8::cudnnNanPropagation_t,
    lclip: *mut f64,
    rclip: *mut f64,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnRNNGetClip", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnRNNGetClip", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(clipMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&clipMode, "cudnnRNNGetClip", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(clipNanOpt), ": ").as_bytes())?;
    crate::CudaDisplay::write(&clipNanOpt, "cudnnRNNGetClip", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(lclip), ": ").as_bytes())?;
    crate::CudaDisplay::write(&lclip, "cudnnRNNGetClip", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rclip), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rclip, "cudnnRNNGetClip", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnSetRNNProjectionLayers(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
    recProjSize: ::core::ffi::c_int,
    outProjSize: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnSetRNNProjectionLayers", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnSetRNNProjectionLayers", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(recProjSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &recProjSize,
        "cudnnSetRNNProjectionLayers",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(outProjSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &outProjSize,
        "cudnnSetRNNProjectionLayers",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetRNNProjectionLayers(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
    recProjSize: *mut ::core::ffi::c_int,
    outProjSize: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnGetRNNProjectionLayers", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnGetRNNProjectionLayers", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(recProjSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &recProjSize,
        "cudnnGetRNNProjectionLayers",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(outProjSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &outProjSize,
        "cudnnGetRNNProjectionLayers",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnCreatePersistentRNNPlan(
    writer: &mut (impl std::io::Write + ?Sized),
    rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
    minibatch: ::core::ffi::c_int,
    dataType: cuda_types::cudnn8::cudnnDataType_t,
    plan: *mut cuda_types::cudnn8::cudnnPersistentRNNPlan_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &rnnDesc,
        "cudnnCreatePersistentRNNPlan",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(minibatch), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &minibatch,
        "cudnnCreatePersistentRNNPlan",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dataType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dataType,
        "cudnnCreatePersistentRNNPlan",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cudnnCreatePersistentRNNPlan", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnDestroyPersistentRNNPlan(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cudnn8::cudnnPersistentRNNPlan_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cudnnDestroyPersistentRNNPlan", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnSetPersistentRNNPlan(
    writer: &mut (impl std::io::Write + ?Sized),
    rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
    plan: cuda_types::cudnn8::cudnnPersistentRNNPlan_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnSetPersistentRNNPlan", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cudnnSetPersistentRNNPlan", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnBuildRNNDynamic(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
    miniBatch: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnBuildRNNDynamic", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnBuildRNNDynamic", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(miniBatch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&miniBatch, "cudnnBuildRNNDynamic", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnGetRNNWorkspaceSize(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
    seqLength: ::core::ffi::c_int,
    xDesc: *const cuda_types::cudnn8::cudnnTensorDescriptor_t,
    sizeInBytes: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnGetRNNWorkspaceSize", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnGetRNNWorkspaceSize", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(seqLength), ": ").as_bytes())?;
    crate::CudaDisplay::write(&seqLength, "cudnnGetRNNWorkspaceSize", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&xDesc, "cudnnGetRNNWorkspaceSize", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeInBytes,
        "cudnnGetRNNWorkspaceSize",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetRNNTrainingReserveSize(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
    seqLength: ::core::ffi::c_int,
    xDesc: *const cuda_types::cudnn8::cudnnTensorDescriptor_t,
    sizeInBytes: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnGetRNNTrainingReserveSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &rnnDesc,
        "cudnnGetRNNTrainingReserveSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(seqLength), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &seqLength,
        "cudnnGetRNNTrainingReserveSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xDesc,
        "cudnnGetRNNTrainingReserveSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeInBytes,
        "cudnnGetRNNTrainingReserveSize",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetRNNTempSpaceSizes(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
    fwdMode: cuda_types::cudnn8::cudnnForwardMode_t,
    xDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
    workSpaceSize: *mut usize,
    reserveSpaceSize: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnGetRNNTempSpaceSizes", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnGetRNNTempSpaceSizes", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(fwdMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&fwdMode, "cudnnGetRNNTempSpaceSizes", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&xDesc, "cudnnGetRNNTempSpaceSizes", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSize,
        "cudnnGetRNNTempSpaceSizes",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpaceSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpaceSize,
        "cudnnGetRNNTempSpaceSizes",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetRNNParamsSize(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
    xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    sizeInBytes: *mut usize,
    dataType: cuda_types::cudnn8::cudnnDataType_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnGetRNNParamsSize", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnGetRNNParamsSize", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&xDesc, "cudnnGetRNNParamsSize", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&sizeInBytes, "cudnnGetRNNParamsSize", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dataType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dataType, "cudnnGetRNNParamsSize", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnGetRNNWeightSpaceSize(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
    weightSpaceSize: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnGetRNNWeightSpaceSize", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnGetRNNWeightSpaceSize", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(weightSpaceSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &weightSpaceSize,
        "cudnnGetRNNWeightSpaceSize",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetRNNLinLayerMatrixParams(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
    pseudoLayer: ::core::ffi::c_int,
    xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    wDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
    w: *const ::core::ffi::c_void,
    linLayerID: ::core::ffi::c_int,
    linLayerMatDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
    linLayerMat: *mut *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnGetRNNLinLayerMatrixParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &rnnDesc,
        "cudnnGetRNNLinLayerMatrixParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pseudoLayer), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pseudoLayer,
        "cudnnGetRNNLinLayerMatrixParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xDesc,
        "cudnnGetRNNLinLayerMatrixParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(wDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &wDesc,
        "cudnnGetRNNLinLayerMatrixParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(w), ": ").as_bytes())?;
    crate::CudaDisplay::write(&w, "cudnnGetRNNLinLayerMatrixParams", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(linLayerID), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &linLayerID,
        "cudnnGetRNNLinLayerMatrixParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(linLayerMatDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &linLayerMatDesc,
        "cudnnGetRNNLinLayerMatrixParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(linLayerMat), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &linLayerMat,
        "cudnnGetRNNLinLayerMatrixParams",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetRNNLinLayerBiasParams(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
    pseudoLayer: ::core::ffi::c_int,
    xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    wDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
    w: *const ::core::ffi::c_void,
    linLayerID: ::core::ffi::c_int,
    linLayerBiasDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
    linLayerBias: *mut *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnGetRNNLinLayerBiasParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &rnnDesc,
        "cudnnGetRNNLinLayerBiasParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pseudoLayer), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pseudoLayer,
        "cudnnGetRNNLinLayerBiasParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&xDesc, "cudnnGetRNNLinLayerBiasParams", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(wDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&wDesc, "cudnnGetRNNLinLayerBiasParams", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(w), ": ").as_bytes())?;
    crate::CudaDisplay::write(&w, "cudnnGetRNNLinLayerBiasParams", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(linLayerID), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &linLayerID,
        "cudnnGetRNNLinLayerBiasParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(linLayerBiasDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &linLayerBiasDesc,
        "cudnnGetRNNLinLayerBiasParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(linLayerBias), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &linLayerBias,
        "cudnnGetRNNLinLayerBiasParams",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetRNNWeightParams(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnGetRNNWeightParams", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnGetRNNWeightParams", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pseudoLayer), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pseudoLayer, "cudnnGetRNNWeightParams", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(weightSpaceSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &weightSpaceSize,
        "cudnnGetRNNWeightParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(weightSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(&weightSpace, "cudnnGetRNNWeightParams", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(linLayerID), ": ").as_bytes())?;
    crate::CudaDisplay::write(&linLayerID, "cudnnGetRNNWeightParams", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mDesc, "cudnnGetRNNWeightParams", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mAddr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mAddr, "cudnnGetRNNWeightParams", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&bDesc, "cudnnGetRNNWeightParams", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bAddr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&bAddr, "cudnnGetRNNWeightParams", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnRNNForwardInference(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnRNNForwardInference", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnRNNForwardInference", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(seqLength), ": ").as_bytes())?;
    crate::CudaDisplay::write(&seqLength, "cudnnRNNForwardInference", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&xDesc, "cudnnRNNForwardInference", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(&x, "cudnnRNNForwardInference", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hxDesc, "cudnnRNNForwardInference", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hx, "cudnnRNNForwardInference", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cxDesc, "cudnnRNNForwardInference", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cx, "cudnnRNNForwardInference", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(wDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&wDesc, "cudnnRNNForwardInference", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(w), ": ").as_bytes())?;
    crate::CudaDisplay::write(&w, "cudnnRNNForwardInference", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&yDesc, "cudnnRNNForwardInference", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(&y, "cudnnRNNForwardInference", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hyDesc, "cudnnRNNForwardInference", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hy, "cudnnRNNForwardInference", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cyDesc, "cudnnRNNForwardInference", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cy, "cudnnRNNForwardInference", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workSpace, "cudnnRNNForwardInference", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSizeInBytes,
        "cudnnRNNForwardInference",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnSetRNNPaddingMode(
    writer: &mut (impl std::io::Write + ?Sized),
    rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
    paddingMode: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnSetRNNPaddingMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(paddingMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&paddingMode, "cudnnSetRNNPaddingMode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnGetRNNPaddingMode(
    writer: &mut (impl std::io::Write + ?Sized),
    rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
    paddingMode: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnGetRNNPaddingMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(paddingMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&paddingMode, "cudnnGetRNNPaddingMode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnCreateRNNDataDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    rnnDataDesc: *mut cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(rnnDataDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &rnnDataDesc,
        "cudnnCreateRNNDataDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnDestroyRNNDataDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    rnnDataDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(rnnDataDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &rnnDataDesc,
        "cudnnDestroyRNNDataDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnSetRNNDataDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    rnnDataDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
    dataType: cuda_types::cudnn8::cudnnDataType_t,
    layout: cuda_types::cudnn8::cudnnRNNDataLayout_t,
    maxSeqLength: ::core::ffi::c_int,
    batchSize: ::core::ffi::c_int,
    vectorSize: ::core::ffi::c_int,
    seqLengthArray: *const ::core::ffi::c_int,
    paddingFill: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(rnnDataDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &rnnDataDesc,
        "cudnnSetRNNDataDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dataType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dataType, "cudnnSetRNNDataDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(layout), ": ").as_bytes())?;
    crate::CudaDisplay::write(&layout, "cudnnSetRNNDataDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(maxSeqLength), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &maxSeqLength,
        "cudnnSetRNNDataDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(batchSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&batchSize, "cudnnSetRNNDataDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vectorSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vectorSize,
        "cudnnSetRNNDataDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(seqLengthArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &seqLengthArray,
        "cudnnSetRNNDataDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(paddingFill), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &paddingFill,
        "cudnnSetRNNDataDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetRNNDataDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    rnnDataDesc: cuda_types::cudnn8::cudnnRNNDataDescriptor_t,
    dataType: *mut cuda_types::cudnn8::cudnnDataType_t,
    layout: *mut cuda_types::cudnn8::cudnnRNNDataLayout_t,
    maxSeqLength: *mut ::core::ffi::c_int,
    batchSize: *mut ::core::ffi::c_int,
    vectorSize: *mut ::core::ffi::c_int,
    arrayLengthRequested: ::core::ffi::c_int,
    seqLengthArray: *mut ::core::ffi::c_int,
    paddingFill: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(rnnDataDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &rnnDataDesc,
        "cudnnGetRNNDataDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dataType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dataType, "cudnnGetRNNDataDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(layout), ": ").as_bytes())?;
    crate::CudaDisplay::write(&layout, "cudnnGetRNNDataDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(maxSeqLength), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &maxSeqLength,
        "cudnnGetRNNDataDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(batchSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&batchSize, "cudnnGetRNNDataDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vectorSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vectorSize,
        "cudnnGetRNNDataDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(arrayLengthRequested), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &arrayLengthRequested,
        "cudnnGetRNNDataDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(seqLengthArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &seqLengthArray,
        "cudnnGetRNNDataDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(paddingFill), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &paddingFill,
        "cudnnGetRNNDataDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnRNNForwardInferenceEx(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnRNNForwardInferenceEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnRNNForwardInferenceEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&xDesc, "cudnnRNNForwardInferenceEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(&x, "cudnnRNNForwardInferenceEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hxDesc, "cudnnRNNForwardInferenceEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hx, "cudnnRNNForwardInferenceEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cxDesc, "cudnnRNNForwardInferenceEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cx, "cudnnRNNForwardInferenceEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(wDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&wDesc, "cudnnRNNForwardInferenceEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(w), ": ").as_bytes())?;
    crate::CudaDisplay::write(&w, "cudnnRNNForwardInferenceEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&yDesc, "cudnnRNNForwardInferenceEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(&y, "cudnnRNNForwardInferenceEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hyDesc, "cudnnRNNForwardInferenceEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hy, "cudnnRNNForwardInferenceEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cyDesc, "cudnnRNNForwardInferenceEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cy, "cudnnRNNForwardInferenceEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(kDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&kDesc, "cudnnRNNForwardInferenceEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(keys), ": ").as_bytes())?;
    crate::CudaDisplay::write(&keys, "cudnnRNNForwardInferenceEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cDesc, "cudnnRNNForwardInferenceEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cAttn), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cAttn, "cudnnRNNForwardInferenceEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(iDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&iDesc, "cudnnRNNForwardInferenceEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(iAttn), ": ").as_bytes())?;
    crate::CudaDisplay::write(&iAttn, "cudnnRNNForwardInferenceEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(qDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&qDesc, "cudnnRNNForwardInferenceEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(queries), ": ").as_bytes())?;
    crate::CudaDisplay::write(&queries, "cudnnRNNForwardInferenceEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpace,
        "cudnnRNNForwardInferenceEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSizeInBytes,
        "cudnnRNNForwardInferenceEx",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnRNNForward(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnRNNForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnRNNForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(fwdMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&fwdMode, "cudnnRNNForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(devSeqLengths), ": ").as_bytes())?;
    crate::CudaDisplay::write(&devSeqLengths, "cudnnRNNForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&xDesc, "cudnnRNNForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(&x, "cudnnRNNForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&yDesc, "cudnnRNNForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(&y, "cudnnRNNForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hDesc, "cudnnRNNForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hx, "cudnnRNNForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hy, "cudnnRNNForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cDesc, "cudnnRNNForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cx, "cudnnRNNForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cy, "cudnnRNNForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(weightSpaceSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&weightSpaceSize, "cudnnRNNForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(weightSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(&weightSpace, "cudnnRNNForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workSpaceSize, "cudnnRNNForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workSpace, "cudnnRNNForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpaceSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&reserveSpaceSize, "cudnnRNNForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(&reserveSpace, "cudnnRNNForward", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnSetRNNAlgorithmDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
    algoDesc: cuda_types::cudnn8::cudnnAlgorithmDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnSetRNNAlgorithmDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &rnnDesc,
        "cudnnSetRNNAlgorithmDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algoDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &algoDesc,
        "cudnnSetRNNAlgorithmDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetRNNForwardInferenceAlgorithmMaxCount(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
    count: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnGetRNNForwardInferenceAlgorithmMaxCount",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &rnnDesc,
        "cudnnGetRNNForwardInferenceAlgorithmMaxCount",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &count,
        "cudnnGetRNNForwardInferenceAlgorithmMaxCount",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnFindRNNForwardInferenceAlgorithmEx(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnFindRNNForwardInferenceAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &rnnDesc,
        "cudnnFindRNNForwardInferenceAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(seqLength), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &seqLength,
        "cudnnFindRNNForwardInferenceAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xDesc,
        "cudnnFindRNNForwardInferenceAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &x,
        "cudnnFindRNNForwardInferenceAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hxDesc,
        "cudnnFindRNNForwardInferenceAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hx), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hx,
        "cudnnFindRNNForwardInferenceAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &cxDesc,
        "cudnnFindRNNForwardInferenceAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cx), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &cx,
        "cudnnFindRNNForwardInferenceAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(wDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &wDesc,
        "cudnnFindRNNForwardInferenceAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(w), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &w,
        "cudnnFindRNNForwardInferenceAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &yDesc,
        "cudnnFindRNNForwardInferenceAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &y,
        "cudnnFindRNNForwardInferenceAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hyDesc,
        "cudnnFindRNNForwardInferenceAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hy), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hy,
        "cudnnFindRNNForwardInferenceAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &cyDesc,
        "cudnnFindRNNForwardInferenceAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cy), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &cy,
        "cudnnFindRNNForwardInferenceAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(findIntensity), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &findIntensity,
        "cudnnFindRNNForwardInferenceAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(requestedAlgoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &requestedAlgoCount,
        "cudnnFindRNNForwardInferenceAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(returnedAlgoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &returnedAlgoCount,
        "cudnnFindRNNForwardInferenceAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(perfResults), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &perfResults,
        "cudnnFindRNNForwardInferenceAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workspace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workspace,
        "cudnnFindRNNForwardInferenceAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSizeInBytes,
        "cudnnFindRNNForwardInferenceAlgorithmEx",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnCreateSeqDataDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    seqDataDesc: *mut cuda_types::cudnn8::cudnnSeqDataDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(seqDataDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &seqDataDesc,
        "cudnnCreateSeqDataDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnDestroySeqDataDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    seqDataDesc: cuda_types::cudnn8::cudnnSeqDataDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(seqDataDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &seqDataDesc,
        "cudnnDestroySeqDataDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnSetSeqDataDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    seqDataDesc: cuda_types::cudnn8::cudnnSeqDataDescriptor_t,
    dataType: cuda_types::cudnn8::cudnnDataType_t,
    nbDims: ::core::ffi::c_int,
    dimA: *const ::core::ffi::c_int,
    axes: *const cuda_types::cudnn8::cudnnSeqDataAxis_t,
    seqLengthArraySize: usize,
    seqLengthArray: *const ::core::ffi::c_int,
    paddingFill: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(seqDataDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &seqDataDesc,
        "cudnnSetSeqDataDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dataType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dataType, "cudnnSetSeqDataDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nbDims), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nbDims, "cudnnSetSeqDataDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dimA), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dimA, "cudnnSetSeqDataDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(axes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&axes, "cudnnSetSeqDataDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(seqLengthArraySize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &seqLengthArraySize,
        "cudnnSetSeqDataDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(seqLengthArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &seqLengthArray,
        "cudnnSetSeqDataDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(paddingFill), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &paddingFill,
        "cudnnSetSeqDataDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetSeqDataDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(seqDataDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &seqDataDesc,
        "cudnnGetSeqDataDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dataType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dataType, "cudnnGetSeqDataDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nbDims), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nbDims, "cudnnGetSeqDataDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nbDimsRequested), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nbDimsRequested,
        "cudnnGetSeqDataDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dimA), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dimA, "cudnnGetSeqDataDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(axes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&axes, "cudnnGetSeqDataDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(seqLengthArraySize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &seqLengthArraySize,
        "cudnnGetSeqDataDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(seqLengthSizeRequested), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &seqLengthSizeRequested,
        "cudnnGetSeqDataDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(seqLengthArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &seqLengthArray,
        "cudnnGetSeqDataDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(paddingFill), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &paddingFill,
        "cudnnGetSeqDataDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnCreateAttnDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    attnDesc: *mut cuda_types::cudnn8::cudnnAttnDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(attnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attnDesc, "cudnnCreateAttnDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnDestroyAttnDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    attnDesc: cuda_types::cudnn8::cudnnAttnDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(attnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attnDesc, "cudnnDestroyAttnDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnSetAttnDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(attnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attnDesc, "cudnnSetAttnDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attnMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attnMode, "cudnnSetAttnDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nHeads), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nHeads, "cudnnSetAttnDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(smScaler), ": ").as_bytes())?;
    crate::CudaDisplay::write(&smScaler, "cudnnSetAttnDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dataType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dataType, "cudnnSetAttnDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(computePrec), ": ").as_bytes())?;
    crate::CudaDisplay::write(&computePrec, "cudnnSetAttnDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mathType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mathType, "cudnnSetAttnDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attnDropoutDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &attnDropoutDesc,
        "cudnnSetAttnDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(postDropoutDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &postDropoutDesc,
        "cudnnSetAttnDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(qSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&qSize, "cudnnSetAttnDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(kSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&kSize, "cudnnSetAttnDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&vSize, "cudnnSetAttnDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(qProjSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&qProjSize, "cudnnSetAttnDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(kProjSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&kProjSize, "cudnnSetAttnDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vProjSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&vProjSize, "cudnnSetAttnDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(oProjSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&oProjSize, "cudnnSetAttnDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(qoMaxSeqLength), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &qoMaxSeqLength,
        "cudnnSetAttnDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(kvMaxSeqLength), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &kvMaxSeqLength,
        "cudnnSetAttnDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(maxBatchSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&maxBatchSize, "cudnnSetAttnDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(maxBeamSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&maxBeamSize, "cudnnSetAttnDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnGetAttnDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(attnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attnDesc, "cudnnGetAttnDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attnMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attnMode, "cudnnGetAttnDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nHeads), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nHeads, "cudnnGetAttnDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(smScaler), ": ").as_bytes())?;
    crate::CudaDisplay::write(&smScaler, "cudnnGetAttnDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dataType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dataType, "cudnnGetAttnDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(computePrec), ": ").as_bytes())?;
    crate::CudaDisplay::write(&computePrec, "cudnnGetAttnDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mathType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mathType, "cudnnGetAttnDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attnDropoutDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &attnDropoutDesc,
        "cudnnGetAttnDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(postDropoutDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &postDropoutDesc,
        "cudnnGetAttnDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(qSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&qSize, "cudnnGetAttnDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(kSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&kSize, "cudnnGetAttnDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&vSize, "cudnnGetAttnDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(qProjSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&qProjSize, "cudnnGetAttnDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(kProjSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&kProjSize, "cudnnGetAttnDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vProjSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&vProjSize, "cudnnGetAttnDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(oProjSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&oProjSize, "cudnnGetAttnDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(qoMaxSeqLength), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &qoMaxSeqLength,
        "cudnnGetAttnDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(kvMaxSeqLength), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &kvMaxSeqLength,
        "cudnnGetAttnDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(maxBatchSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&maxBatchSize, "cudnnGetAttnDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(maxBeamSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&maxBeamSize, "cudnnGetAttnDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnGetMultiHeadAttnBuffers(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    attnDesc: cuda_types::cudnn8::cudnnAttnDescriptor_t,
    weightSizeInBytes: *mut usize,
    workSpaceSizeInBytes: *mut usize,
    reserveSpaceSizeInBytes: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnGetMultiHeadAttnBuffers", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &attnDesc,
        "cudnnGetMultiHeadAttnBuffers",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(weightSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &weightSizeInBytes,
        "cudnnGetMultiHeadAttnBuffers",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSizeInBytes,
        "cudnnGetMultiHeadAttnBuffers",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpaceSizeInBytes,
        "cudnnGetMultiHeadAttnBuffers",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetMultiHeadAttnWeights(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    attnDesc: cuda_types::cudnn8::cudnnAttnDescriptor_t,
    wKind: cuda_types::cudnn8::cudnnMultiHeadAttnWeightKind_t,
    weightSizeInBytes: usize,
    weights: *const ::core::ffi::c_void,
    wDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    wAddr: *mut *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnGetMultiHeadAttnWeights", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &attnDesc,
        "cudnnGetMultiHeadAttnWeights",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(wKind), ": ").as_bytes())?;
    crate::CudaDisplay::write(&wKind, "cudnnGetMultiHeadAttnWeights", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(weightSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &weightSizeInBytes,
        "cudnnGetMultiHeadAttnWeights",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(weights), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &weights,
        "cudnnGetMultiHeadAttnWeights",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(wDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&wDesc, "cudnnGetMultiHeadAttnWeights", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(wAddr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&wAddr, "cudnnGetMultiHeadAttnWeights", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnMultiHeadAttnForward(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnMultiHeadAttnForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attnDesc, "cudnnMultiHeadAttnForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(currIdx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&currIdx, "cudnnMultiHeadAttnForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(loWinIdx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&loWinIdx, "cudnnMultiHeadAttnForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hiWinIdx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hiWinIdx, "cudnnMultiHeadAttnForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(devSeqLengthsQO), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &devSeqLengthsQO,
        "cudnnMultiHeadAttnForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(devSeqLengthsKV), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &devSeqLengthsKV,
        "cudnnMultiHeadAttnForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(qDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&qDesc, "cudnnMultiHeadAttnForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(queries), ": ").as_bytes())?;
    crate::CudaDisplay::write(&queries, "cudnnMultiHeadAttnForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(residuals), ": ").as_bytes())?;
    crate::CudaDisplay::write(&residuals, "cudnnMultiHeadAttnForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(kDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&kDesc, "cudnnMultiHeadAttnForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(keys), ": ").as_bytes())?;
    crate::CudaDisplay::write(&keys, "cudnnMultiHeadAttnForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&vDesc, "cudnnMultiHeadAttnForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(values), ": ").as_bytes())?;
    crate::CudaDisplay::write(&values, "cudnnMultiHeadAttnForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(oDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&oDesc, "cudnnMultiHeadAttnForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(out), ": ").as_bytes())?;
    crate::CudaDisplay::write(&out, "cudnnMultiHeadAttnForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(weightSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &weightSizeInBytes,
        "cudnnMultiHeadAttnForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(weights), ": ").as_bytes())?;
    crate::CudaDisplay::write(&weights, "cudnnMultiHeadAttnForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSizeInBytes,
        "cudnnMultiHeadAttnForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workSpace, "cudnnMultiHeadAttnForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpaceSizeInBytes,
        "cudnnMultiHeadAttnForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpace,
        "cudnnMultiHeadAttnForward",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnAdvInferVersionCheck(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
pub fn write_cudnnRNNForwardTraining(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnRNNForwardTraining", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnRNNForwardTraining", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(seqLength), ": ").as_bytes())?;
    crate::CudaDisplay::write(&seqLength, "cudnnRNNForwardTraining", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&xDesc, "cudnnRNNForwardTraining", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(&x, "cudnnRNNForwardTraining", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hxDesc, "cudnnRNNForwardTraining", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hx, "cudnnRNNForwardTraining", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cxDesc, "cudnnRNNForwardTraining", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cx, "cudnnRNNForwardTraining", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(wDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&wDesc, "cudnnRNNForwardTraining", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(w), ": ").as_bytes())?;
    crate::CudaDisplay::write(&w, "cudnnRNNForwardTraining", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&yDesc, "cudnnRNNForwardTraining", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(&y, "cudnnRNNForwardTraining", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hyDesc, "cudnnRNNForwardTraining", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hy, "cudnnRNNForwardTraining", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cyDesc, "cudnnRNNForwardTraining", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cy, "cudnnRNNForwardTraining", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workSpace, "cudnnRNNForwardTraining", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSizeInBytes,
        "cudnnRNNForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpace,
        "cudnnRNNForwardTraining",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpaceSizeInBytes,
        "cudnnRNNForwardTraining",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnRNNBackwardData(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnRNNBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnRNNBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(seqLength), ": ").as_bytes())?;
    crate::CudaDisplay::write(&seqLength, "cudnnRNNBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&yDesc, "cudnnRNNBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(&y, "cudnnRNNBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dyDesc, "cudnnRNNBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dy, "cudnnRNNBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dhyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dhyDesc, "cudnnRNNBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dhy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dhy, "cudnnRNNBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dcyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dcyDesc, "cudnnRNNBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dcy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dcy, "cudnnRNNBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(wDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&wDesc, "cudnnRNNBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(w), ": ").as_bytes())?;
    crate::CudaDisplay::write(&w, "cudnnRNNBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hxDesc, "cudnnRNNBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hx, "cudnnRNNBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cxDesc, "cudnnRNNBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cx, "cudnnRNNBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dxDesc, "cudnnRNNBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dx, "cudnnRNNBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dhxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dhxDesc, "cudnnRNNBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dhx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dhx, "cudnnRNNBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dcxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dcxDesc, "cudnnRNNBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dcx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dcx, "cudnnRNNBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workSpace, "cudnnRNNBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSizeInBytes,
        "cudnnRNNBackwardData",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(&reserveSpace, "cudnnRNNBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpaceSizeInBytes,
        "cudnnRNNBackwardData",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnRNNBackwardData_v8(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnRNNBackwardData_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnRNNBackwardData_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(devSeqLengths), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &devSeqLengths,
        "cudnnRNNBackwardData_v8",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&yDesc, "cudnnRNNBackwardData_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(&y, "cudnnRNNBackwardData_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dy, "cudnnRNNBackwardData_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&xDesc, "cudnnRNNBackwardData_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dx, "cudnnRNNBackwardData_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hDesc, "cudnnRNNBackwardData_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hx, "cudnnRNNBackwardData_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dhy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dhy, "cudnnRNNBackwardData_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dhx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dhx, "cudnnRNNBackwardData_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cDesc, "cudnnRNNBackwardData_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cx, "cudnnRNNBackwardData_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dcy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dcy, "cudnnRNNBackwardData_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dcx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dcx, "cudnnRNNBackwardData_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(weightSpaceSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &weightSpaceSize,
        "cudnnRNNBackwardData_v8",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(weightSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(&weightSpace, "cudnnRNNBackwardData_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSize,
        "cudnnRNNBackwardData_v8",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workSpace, "cudnnRNNBackwardData_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpaceSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpaceSize,
        "cudnnRNNBackwardData_v8",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpace,
        "cudnnRNNBackwardData_v8",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnRNNBackwardWeights(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnRNNBackwardWeights", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnRNNBackwardWeights", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(seqLength), ": ").as_bytes())?;
    crate::CudaDisplay::write(&seqLength, "cudnnRNNBackwardWeights", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&xDesc, "cudnnRNNBackwardWeights", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(&x, "cudnnRNNBackwardWeights", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hxDesc, "cudnnRNNBackwardWeights", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hx, "cudnnRNNBackwardWeights", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&yDesc, "cudnnRNNBackwardWeights", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(&y, "cudnnRNNBackwardWeights", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workSpace, "cudnnRNNBackwardWeights", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSizeInBytes,
        "cudnnRNNBackwardWeights",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dwDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dwDesc, "cudnnRNNBackwardWeights", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dw), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dw, "cudnnRNNBackwardWeights", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpace,
        "cudnnRNNBackwardWeights",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpaceSizeInBytes,
        "cudnnRNNBackwardWeights",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnRNNBackwardWeights_v8(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnRNNBackwardWeights_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnRNNBackwardWeights_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(addGrad), ": ").as_bytes())?;
    crate::CudaDisplay::write(&addGrad, "cudnnRNNBackwardWeights_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(devSeqLengths), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &devSeqLengths,
        "cudnnRNNBackwardWeights_v8",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&xDesc, "cudnnRNNBackwardWeights_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(&x, "cudnnRNNBackwardWeights_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hDesc, "cudnnRNNBackwardWeights_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hx, "cudnnRNNBackwardWeights_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&yDesc, "cudnnRNNBackwardWeights_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(&y, "cudnnRNNBackwardWeights_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(weightSpaceSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &weightSpaceSize,
        "cudnnRNNBackwardWeights_v8",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dweightSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dweightSpace,
        "cudnnRNNBackwardWeights_v8",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSize,
        "cudnnRNNBackwardWeights_v8",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpace,
        "cudnnRNNBackwardWeights_v8",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpaceSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpaceSize,
        "cudnnRNNBackwardWeights_v8",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpace,
        "cudnnRNNBackwardWeights_v8",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnRNNForwardTrainingEx(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnRNNForwardTrainingEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnRNNForwardTrainingEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&xDesc, "cudnnRNNForwardTrainingEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(&x, "cudnnRNNForwardTrainingEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hxDesc, "cudnnRNNForwardTrainingEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hx, "cudnnRNNForwardTrainingEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cxDesc, "cudnnRNNForwardTrainingEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cx, "cudnnRNNForwardTrainingEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(wDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&wDesc, "cudnnRNNForwardTrainingEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(w), ": ").as_bytes())?;
    crate::CudaDisplay::write(&w, "cudnnRNNForwardTrainingEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&yDesc, "cudnnRNNForwardTrainingEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(&y, "cudnnRNNForwardTrainingEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hyDesc, "cudnnRNNForwardTrainingEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hy, "cudnnRNNForwardTrainingEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cyDesc, "cudnnRNNForwardTrainingEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cy, "cudnnRNNForwardTrainingEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(kDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&kDesc, "cudnnRNNForwardTrainingEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(keys), ": ").as_bytes())?;
    crate::CudaDisplay::write(&keys, "cudnnRNNForwardTrainingEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cDesc, "cudnnRNNForwardTrainingEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cAttn), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cAttn, "cudnnRNNForwardTrainingEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(iDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&iDesc, "cudnnRNNForwardTrainingEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(iAttn), ": ").as_bytes())?;
    crate::CudaDisplay::write(&iAttn, "cudnnRNNForwardTrainingEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(qDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&qDesc, "cudnnRNNForwardTrainingEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(queries), ": ").as_bytes())?;
    crate::CudaDisplay::write(&queries, "cudnnRNNForwardTrainingEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workSpace, "cudnnRNNForwardTrainingEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSizeInBytes,
        "cudnnRNNForwardTrainingEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpace,
        "cudnnRNNForwardTrainingEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpaceSizeInBytes,
        "cudnnRNNForwardTrainingEx",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnRNNBackwardDataEx(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnRNNBackwardDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnRNNBackwardDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&yDesc, "cudnnRNNBackwardDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(&y, "cudnnRNNBackwardDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dyDesc, "cudnnRNNBackwardDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dy, "cudnnRNNBackwardDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dcDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dcDesc, "cudnnRNNBackwardDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dcAttn), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dcAttn, "cudnnRNNBackwardDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dhyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dhyDesc, "cudnnRNNBackwardDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dhy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dhy, "cudnnRNNBackwardDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dcyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dcyDesc, "cudnnRNNBackwardDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dcy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dcy, "cudnnRNNBackwardDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(wDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&wDesc, "cudnnRNNBackwardDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(w), ": ").as_bytes())?;
    crate::CudaDisplay::write(&w, "cudnnRNNBackwardDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hxDesc, "cudnnRNNBackwardDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hx, "cudnnRNNBackwardDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cxDesc, "cudnnRNNBackwardDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cx, "cudnnRNNBackwardDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dxDesc, "cudnnRNNBackwardDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dx, "cudnnRNNBackwardDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dhxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dhxDesc, "cudnnRNNBackwardDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dhx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dhx, "cudnnRNNBackwardDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dcxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dcxDesc, "cudnnRNNBackwardDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dcx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dcx, "cudnnRNNBackwardDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dkDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dkDesc, "cudnnRNNBackwardDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dkeys), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dkeys, "cudnnRNNBackwardDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workSpace, "cudnnRNNBackwardDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSizeInBytes,
        "cudnnRNNBackwardDataEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(&reserveSpace, "cudnnRNNBackwardDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpaceSizeInBytes,
        "cudnnRNNBackwardDataEx",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnRNNBackwardWeightsEx(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnRNNBackwardWeightsEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnRNNBackwardWeightsEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&xDesc, "cudnnRNNBackwardWeightsEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(&x, "cudnnRNNBackwardWeightsEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hxDesc, "cudnnRNNBackwardWeightsEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hx, "cudnnRNNBackwardWeightsEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&yDesc, "cudnnRNNBackwardWeightsEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(&y, "cudnnRNNBackwardWeightsEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workSpace, "cudnnRNNBackwardWeightsEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSizeInBytes,
        "cudnnRNNBackwardWeightsEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dwDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dwDesc, "cudnnRNNBackwardWeightsEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dw), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dw, "cudnnRNNBackwardWeightsEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpace,
        "cudnnRNNBackwardWeightsEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpaceSizeInBytes,
        "cudnnRNNBackwardWeightsEx",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetRNNForwardTrainingAlgorithmMaxCount(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
    count: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnGetRNNForwardTrainingAlgorithmMaxCount",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &rnnDesc,
        "cudnnGetRNNForwardTrainingAlgorithmMaxCount",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &count,
        "cudnnGetRNNForwardTrainingAlgorithmMaxCount",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnFindRNNForwardTrainingAlgorithmEx(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnFindRNNForwardTrainingAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &rnnDesc,
        "cudnnFindRNNForwardTrainingAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(seqLength), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &seqLength,
        "cudnnFindRNNForwardTrainingAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xDesc,
        "cudnnFindRNNForwardTrainingAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &x,
        "cudnnFindRNNForwardTrainingAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hxDesc,
        "cudnnFindRNNForwardTrainingAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hx), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hx,
        "cudnnFindRNNForwardTrainingAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &cxDesc,
        "cudnnFindRNNForwardTrainingAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cx), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &cx,
        "cudnnFindRNNForwardTrainingAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(wDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &wDesc,
        "cudnnFindRNNForwardTrainingAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(w), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &w,
        "cudnnFindRNNForwardTrainingAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &yDesc,
        "cudnnFindRNNForwardTrainingAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &y,
        "cudnnFindRNNForwardTrainingAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hyDesc,
        "cudnnFindRNNForwardTrainingAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hy), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hy,
        "cudnnFindRNNForwardTrainingAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &cyDesc,
        "cudnnFindRNNForwardTrainingAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cy), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &cy,
        "cudnnFindRNNForwardTrainingAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(findIntensity), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &findIntensity,
        "cudnnFindRNNForwardTrainingAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(requestedAlgoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &requestedAlgoCount,
        "cudnnFindRNNForwardTrainingAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(returnedAlgoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &returnedAlgoCount,
        "cudnnFindRNNForwardTrainingAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(perfResults), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &perfResults,
        "cudnnFindRNNForwardTrainingAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workspace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workspace,
        "cudnnFindRNNForwardTrainingAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSizeInBytes,
        "cudnnFindRNNForwardTrainingAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpace,
        "cudnnFindRNNForwardTrainingAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpaceSizeInBytes,
        "cudnnFindRNNForwardTrainingAlgorithmEx",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetRNNBackwardDataAlgorithmMaxCount(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
    count: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnGetRNNBackwardDataAlgorithmMaxCount",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &rnnDesc,
        "cudnnGetRNNBackwardDataAlgorithmMaxCount",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &count,
        "cudnnGetRNNBackwardDataAlgorithmMaxCount",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnFindRNNBackwardDataAlgorithmEx(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &rnnDesc,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(seqLength), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &seqLength,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &yDesc,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &y,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dyDesc,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dy), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dy,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dhyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dhyDesc,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dhy), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dhy,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dcyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dcyDesc,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dcy), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dcy,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(wDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &wDesc,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(w), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &w,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hxDesc,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hx), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hx,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &cxDesc,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cx), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &cx,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dxDesc,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dx), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dx,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dhxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dhxDesc,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dhx), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dhx,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dcxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dcxDesc,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dcx), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dcx,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(findIntensity), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &findIntensity,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(requestedAlgoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &requestedAlgoCount,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(returnedAlgoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &returnedAlgoCount,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(perfResults), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &perfResults,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workspace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workspace,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSizeInBytes,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpace,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpaceSizeInBytes,
        "cudnnFindRNNBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetRNNBackwardWeightsAlgorithmMaxCount(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    rnnDesc: cuda_types::cudnn8::cudnnRNNDescriptor_t,
    count: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnGetRNNBackwardWeightsAlgorithmMaxCount",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &rnnDesc,
        "cudnnGetRNNBackwardWeightsAlgorithmMaxCount",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &count,
        "cudnnGetRNNBackwardWeightsAlgorithmMaxCount",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnFindRNNBackwardWeightsAlgorithmEx(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnFindRNNBackwardWeightsAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &rnnDesc,
        "cudnnFindRNNBackwardWeightsAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(seqLength), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &seqLength,
        "cudnnFindRNNBackwardWeightsAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xDesc,
        "cudnnFindRNNBackwardWeightsAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &x,
        "cudnnFindRNNBackwardWeightsAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hxDesc,
        "cudnnFindRNNBackwardWeightsAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hx), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hx,
        "cudnnFindRNNBackwardWeightsAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &yDesc,
        "cudnnFindRNNBackwardWeightsAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &y,
        "cudnnFindRNNBackwardWeightsAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(findIntensity), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &findIntensity,
        "cudnnFindRNNBackwardWeightsAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(requestedAlgoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &requestedAlgoCount,
        "cudnnFindRNNBackwardWeightsAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(returnedAlgoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &returnedAlgoCount,
        "cudnnFindRNNBackwardWeightsAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(perfResults), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &perfResults,
        "cudnnFindRNNBackwardWeightsAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workspace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workspace,
        "cudnnFindRNNBackwardWeightsAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSizeInBytes,
        "cudnnFindRNNBackwardWeightsAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dwDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dwDesc,
        "cudnnFindRNNBackwardWeightsAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dw), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dw,
        "cudnnFindRNNBackwardWeightsAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpace,
        "cudnnFindRNNBackwardWeightsAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpaceSizeInBytes,
        "cudnnFindRNNBackwardWeightsAlgorithmEx",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnMultiHeadAttnBackwardData(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnMultiHeadAttnBackwardData",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &attnDesc,
        "cudnnMultiHeadAttnBackwardData",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(loWinIdx), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &loWinIdx,
        "cudnnMultiHeadAttnBackwardData",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hiWinIdx), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hiWinIdx,
        "cudnnMultiHeadAttnBackwardData",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(devSeqLengthsDQDO), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &devSeqLengthsDQDO,
        "cudnnMultiHeadAttnBackwardData",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(devSeqLengthsDKDV), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &devSeqLengthsDKDV,
        "cudnnMultiHeadAttnBackwardData",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(doDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &doDesc,
        "cudnnMultiHeadAttnBackwardData",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dout), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dout, "cudnnMultiHeadAttnBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dqDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dqDesc,
        "cudnnMultiHeadAttnBackwardData",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dqueries), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dqueries,
        "cudnnMultiHeadAttnBackwardData",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(queries), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &queries,
        "cudnnMultiHeadAttnBackwardData",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dkDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dkDesc,
        "cudnnMultiHeadAttnBackwardData",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dkeys), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dkeys,
        "cudnnMultiHeadAttnBackwardData",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(keys), ": ").as_bytes())?;
    crate::CudaDisplay::write(&keys, "cudnnMultiHeadAttnBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dvDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dvDesc,
        "cudnnMultiHeadAttnBackwardData",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dvalues), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dvalues,
        "cudnnMultiHeadAttnBackwardData",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(values), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &values,
        "cudnnMultiHeadAttnBackwardData",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(weightSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &weightSizeInBytes,
        "cudnnMultiHeadAttnBackwardData",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(weights), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &weights,
        "cudnnMultiHeadAttnBackwardData",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSizeInBytes,
        "cudnnMultiHeadAttnBackwardData",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpace,
        "cudnnMultiHeadAttnBackwardData",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpaceSizeInBytes,
        "cudnnMultiHeadAttnBackwardData",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpace,
        "cudnnMultiHeadAttnBackwardData",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnMultiHeadAttnBackwardWeights(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnMultiHeadAttnBackwardWeights",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &attnDesc,
        "cudnnMultiHeadAttnBackwardWeights",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(addGrad), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &addGrad,
        "cudnnMultiHeadAttnBackwardWeights",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(qDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &qDesc,
        "cudnnMultiHeadAttnBackwardWeights",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(queries), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &queries,
        "cudnnMultiHeadAttnBackwardWeights",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(kDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &kDesc,
        "cudnnMultiHeadAttnBackwardWeights",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(keys), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &keys,
        "cudnnMultiHeadAttnBackwardWeights",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vDesc,
        "cudnnMultiHeadAttnBackwardWeights",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(values), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &values,
        "cudnnMultiHeadAttnBackwardWeights",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(doDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &doDesc,
        "cudnnMultiHeadAttnBackwardWeights",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dout), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dout,
        "cudnnMultiHeadAttnBackwardWeights",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(weightSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &weightSizeInBytes,
        "cudnnMultiHeadAttnBackwardWeights",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(weights), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &weights,
        "cudnnMultiHeadAttnBackwardWeights",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dweights), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dweights,
        "cudnnMultiHeadAttnBackwardWeights",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSizeInBytes,
        "cudnnMultiHeadAttnBackwardWeights",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpace,
        "cudnnMultiHeadAttnBackwardWeights",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpaceSizeInBytes,
        "cudnnMultiHeadAttnBackwardWeights",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reserveSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reserveSpace,
        "cudnnMultiHeadAttnBackwardWeights",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnCreateCTCLossDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    ctcLossDesc: *mut cuda_types::cudnn8::cudnnCTCLossDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(ctcLossDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &ctcLossDesc,
        "cudnnCreateCTCLossDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnSetCTCLossDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    ctcLossDesc: cuda_types::cudnn8::cudnnCTCLossDescriptor_t,
    compType: cuda_types::cudnn8::cudnnDataType_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(ctcLossDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &ctcLossDesc,
        "cudnnSetCTCLossDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(compType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&compType, "cudnnSetCTCLossDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnSetCTCLossDescriptorEx(
    writer: &mut (impl std::io::Write + ?Sized),
    ctcLossDesc: cuda_types::cudnn8::cudnnCTCLossDescriptor_t,
    compType: cuda_types::cudnn8::cudnnDataType_t,
    normMode: cuda_types::cudnn8::cudnnLossNormalizationMode_t,
    gradMode: cuda_types::cudnn8::cudnnNanPropagation_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(ctcLossDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &ctcLossDesc,
        "cudnnSetCTCLossDescriptorEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(compType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &compType,
        "cudnnSetCTCLossDescriptorEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(normMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &normMode,
        "cudnnSetCTCLossDescriptorEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gradMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gradMode,
        "cudnnSetCTCLossDescriptorEx",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnSetCTCLossDescriptor_v8(
    writer: &mut (impl std::io::Write + ?Sized),
    ctcLossDesc: cuda_types::cudnn8::cudnnCTCLossDescriptor_t,
    compType: cuda_types::cudnn8::cudnnDataType_t,
    normMode: cuda_types::cudnn8::cudnnLossNormalizationMode_t,
    gradMode: cuda_types::cudnn8::cudnnNanPropagation_t,
    maxLabelLength: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(ctcLossDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &ctcLossDesc,
        "cudnnSetCTCLossDescriptor_v8",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(compType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &compType,
        "cudnnSetCTCLossDescriptor_v8",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(normMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &normMode,
        "cudnnSetCTCLossDescriptor_v8",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gradMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gradMode,
        "cudnnSetCTCLossDescriptor_v8",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(maxLabelLength), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &maxLabelLength,
        "cudnnSetCTCLossDescriptor_v8",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetCTCLossDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    ctcLossDesc: cuda_types::cudnn8::cudnnCTCLossDescriptor_t,
    compType: *mut cuda_types::cudnn8::cudnnDataType_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(ctcLossDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &ctcLossDesc,
        "cudnnGetCTCLossDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(compType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&compType, "cudnnGetCTCLossDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnGetCTCLossDescriptorEx(
    writer: &mut (impl std::io::Write + ?Sized),
    ctcLossDesc: cuda_types::cudnn8::cudnnCTCLossDescriptor_t,
    compType: *mut cuda_types::cudnn8::cudnnDataType_t,
    normMode: *mut cuda_types::cudnn8::cudnnLossNormalizationMode_t,
    gradMode: *mut cuda_types::cudnn8::cudnnNanPropagation_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(ctcLossDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &ctcLossDesc,
        "cudnnGetCTCLossDescriptorEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(compType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &compType,
        "cudnnGetCTCLossDescriptorEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(normMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &normMode,
        "cudnnGetCTCLossDescriptorEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gradMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gradMode,
        "cudnnGetCTCLossDescriptorEx",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetCTCLossDescriptor_v8(
    writer: &mut (impl std::io::Write + ?Sized),
    ctcLossDesc: cuda_types::cudnn8::cudnnCTCLossDescriptor_t,
    compType: *mut cuda_types::cudnn8::cudnnDataType_t,
    normMode: *mut cuda_types::cudnn8::cudnnLossNormalizationMode_t,
    gradMode: *mut cuda_types::cudnn8::cudnnNanPropagation_t,
    maxLabelLength: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(ctcLossDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &ctcLossDesc,
        "cudnnGetCTCLossDescriptor_v8",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(compType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &compType,
        "cudnnGetCTCLossDescriptor_v8",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(normMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &normMode,
        "cudnnGetCTCLossDescriptor_v8",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gradMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gradMode,
        "cudnnGetCTCLossDescriptor_v8",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(maxLabelLength), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &maxLabelLength,
        "cudnnGetCTCLossDescriptor_v8",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnDestroyCTCLossDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    ctcLossDesc: cuda_types::cudnn8::cudnnCTCLossDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(ctcLossDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &ctcLossDesc,
        "cudnnDestroyCTCLossDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnCTCLoss(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnCTCLoss", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(probsDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&probsDesc, "cudnnCTCLoss", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(probs), ": ").as_bytes())?;
    crate::CudaDisplay::write(&probs, "cudnnCTCLoss", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hostLabels), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hostLabels, "cudnnCTCLoss", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hostLabelLengths), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hostLabelLengths, "cudnnCTCLoss", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hostInputLengths), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hostInputLengths, "cudnnCTCLoss", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(costs), ": ").as_bytes())?;
    crate::CudaDisplay::write(&costs, "cudnnCTCLoss", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gradientsDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&gradientsDesc, "cudnnCTCLoss", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gradients), ": ").as_bytes())?;
    crate::CudaDisplay::write(&gradients, "cudnnCTCLoss", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algo), ": ").as_bytes())?;
    crate::CudaDisplay::write(&algo, "cudnnCTCLoss", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ctcLossDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ctcLossDesc, "cudnnCTCLoss", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workspace), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workspace, "cudnnCTCLoss", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workSpaceSizeInBytes, "cudnnCTCLoss", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnCTCLoss_v8(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnCTCLoss_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algo), ": ").as_bytes())?;
    crate::CudaDisplay::write(&algo, "cudnnCTCLoss_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ctcLossDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ctcLossDesc, "cudnnCTCLoss_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(probsDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&probsDesc, "cudnnCTCLoss_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(probs), ": ").as_bytes())?;
    crate::CudaDisplay::write(&probs, "cudnnCTCLoss_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(labels), ": ").as_bytes())?;
    crate::CudaDisplay::write(&labels, "cudnnCTCLoss_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(labelLengths), ": ").as_bytes())?;
    crate::CudaDisplay::write(&labelLengths, "cudnnCTCLoss_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(inputLengths), ": ").as_bytes())?;
    crate::CudaDisplay::write(&inputLengths, "cudnnCTCLoss_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(costs), ": ").as_bytes())?;
    crate::CudaDisplay::write(&costs, "cudnnCTCLoss_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gradientsDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&gradientsDesc, "cudnnCTCLoss_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gradients), ": ").as_bytes())?;
    crate::CudaDisplay::write(&gradients, "cudnnCTCLoss_v8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSizeInBytes,
        "cudnnCTCLoss_v8",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workspace), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workspace, "cudnnCTCLoss_v8", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnGetCTCLossWorkspaceSize(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    probsDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    gradientsDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    labels: *const ::core::ffi::c_int,
    labelLengths: *const ::core::ffi::c_int,
    inputLengths: *const ::core::ffi::c_int,
    algo: cuda_types::cudnn8::cudnnCTCLossAlgo_t,
    ctcLossDesc: cuda_types::cudnn8::cudnnCTCLossDescriptor_t,
    sizeInBytes: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnGetCTCLossWorkspaceSize", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(probsDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &probsDesc,
        "cudnnGetCTCLossWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gradientsDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gradientsDesc,
        "cudnnGetCTCLossWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(labels), ": ").as_bytes())?;
    crate::CudaDisplay::write(&labels, "cudnnGetCTCLossWorkspaceSize", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(labelLengths), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &labelLengths,
        "cudnnGetCTCLossWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(inputLengths), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &inputLengths,
        "cudnnGetCTCLossWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algo), ": ").as_bytes())?;
    crate::CudaDisplay::write(&algo, "cudnnGetCTCLossWorkspaceSize", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ctcLossDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &ctcLossDesc,
        "cudnnGetCTCLossWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeInBytes,
        "cudnnGetCTCLossWorkspaceSize",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetCTCLossWorkspaceSize_v8(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    algo: cuda_types::cudnn8::cudnnCTCLossAlgo_t,
    ctcLossDesc: cuda_types::cudnn8::cudnnCTCLossDescriptor_t,
    probsDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    gradientsDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    sizeInBytes: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnGetCTCLossWorkspaceSize_v8",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &algo,
        "cudnnGetCTCLossWorkspaceSize_v8",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ctcLossDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &ctcLossDesc,
        "cudnnGetCTCLossWorkspaceSize_v8",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(probsDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &probsDesc,
        "cudnnGetCTCLossWorkspaceSize_v8",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gradientsDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gradientsDesc,
        "cudnnGetCTCLossWorkspaceSize_v8",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeInBytes,
        "cudnnGetCTCLossWorkspaceSize_v8",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnAdvTrainVersionCheck(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
impl crate::CudaDisplay for cuda_types::cudnn8::cudnnConvolutionFwdAlgoPerfStruct {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(algo), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.algo, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(status), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.status, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(time), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.time, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(memory), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.memory, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(determinism), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.determinism, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(mathType), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.mathType, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
pub fn write_cudnnCreateConvolutionDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    convDesc: *mut cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &convDesc,
        "cudnnCreateConvolutionDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnDestroyConvolutionDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &convDesc,
        "cudnnDestroyConvolutionDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnSetConvolutionMathType(
    writer: &mut (impl std::io::Write + ?Sized),
    convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
    mathType: cuda_types::cudnn8::cudnnMathType_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &convDesc,
        "cudnnSetConvolutionMathType",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mathType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mathType,
        "cudnnSetConvolutionMathType",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetConvolutionMathType(
    writer: &mut (impl std::io::Write + ?Sized),
    convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
    mathType: *mut cuda_types::cudnn8::cudnnMathType_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &convDesc,
        "cudnnGetConvolutionMathType",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mathType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mathType,
        "cudnnGetConvolutionMathType",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnSetConvolutionGroupCount(
    writer: &mut (impl std::io::Write + ?Sized),
    convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
    groupCount: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &convDesc,
        "cudnnSetConvolutionGroupCount",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(groupCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &groupCount,
        "cudnnSetConvolutionGroupCount",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetConvolutionGroupCount(
    writer: &mut (impl std::io::Write + ?Sized),
    convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
    groupCount: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &convDesc,
        "cudnnGetConvolutionGroupCount",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(groupCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &groupCount,
        "cudnnGetConvolutionGroupCount",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnSetConvolutionReorderType(
    writer: &mut (impl std::io::Write + ?Sized),
    convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
    reorderType: cuda_types::cudnn8::cudnnReorderType_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &convDesc,
        "cudnnSetConvolutionReorderType",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reorderType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reorderType,
        "cudnnSetConvolutionReorderType",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetConvolutionReorderType(
    writer: &mut (impl std::io::Write + ?Sized),
    convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
    reorderType: *mut cuda_types::cudnn8::cudnnReorderType_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &convDesc,
        "cudnnGetConvolutionReorderType",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reorderType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reorderType,
        "cudnnGetConvolutionReorderType",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnSetConvolution2dDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
    pad_h: ::core::ffi::c_int,
    pad_w: ::core::ffi::c_int,
    u: ::core::ffi::c_int,
    v: ::core::ffi::c_int,
    dilation_h: ::core::ffi::c_int,
    dilation_w: ::core::ffi::c_int,
    mode: cuda_types::cudnn8::cudnnConvolutionMode_t,
    computeType: cuda_types::cudnn8::cudnnDataType_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &convDesc,
        "cudnnSetConvolution2dDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pad_h), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pad_h,
        "cudnnSetConvolution2dDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pad_w), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pad_w,
        "cudnnSetConvolution2dDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(u), ": ").as_bytes())?;
    crate::CudaDisplay::write(&u, "cudnnSetConvolution2dDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(v), ": ").as_bytes())?;
    crate::CudaDisplay::write(&v, "cudnnSetConvolution2dDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dilation_h), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dilation_h,
        "cudnnSetConvolution2dDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dilation_w), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dilation_w,
        "cudnnSetConvolution2dDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mode,
        "cudnnSetConvolution2dDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(computeType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &computeType,
        "cudnnSetConvolution2dDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetConvolution2dDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
    pad_h: *mut ::core::ffi::c_int,
    pad_w: *mut ::core::ffi::c_int,
    u: *mut ::core::ffi::c_int,
    v: *mut ::core::ffi::c_int,
    dilation_h: *mut ::core::ffi::c_int,
    dilation_w: *mut ::core::ffi::c_int,
    mode: *mut cuda_types::cudnn8::cudnnConvolutionMode_t,
    computeType: *mut cuda_types::cudnn8::cudnnDataType_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &convDesc,
        "cudnnGetConvolution2dDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pad_h), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pad_h,
        "cudnnGetConvolution2dDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pad_w), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pad_w,
        "cudnnGetConvolution2dDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(u), ": ").as_bytes())?;
    crate::CudaDisplay::write(&u, "cudnnGetConvolution2dDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(v), ": ").as_bytes())?;
    crate::CudaDisplay::write(&v, "cudnnGetConvolution2dDescriptor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dilation_h), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dilation_h,
        "cudnnGetConvolution2dDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dilation_w), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dilation_w,
        "cudnnGetConvolution2dDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mode,
        "cudnnGetConvolution2dDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(computeType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &computeType,
        "cudnnGetConvolution2dDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnSetConvolutionNdDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
    arrayLength: ::core::ffi::c_int,
    padA: *const ::core::ffi::c_int,
    filterStrideA: *const ::core::ffi::c_int,
    dilationA: *const ::core::ffi::c_int,
    mode: cuda_types::cudnn8::cudnnConvolutionMode_t,
    computeType: cuda_types::cudnn8::cudnnDataType_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &convDesc,
        "cudnnSetConvolutionNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(arrayLength), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &arrayLength,
        "cudnnSetConvolutionNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(padA), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &padA,
        "cudnnSetConvolutionNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(filterStrideA), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &filterStrideA,
        "cudnnSetConvolutionNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dilationA), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dilationA,
        "cudnnSetConvolutionNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mode,
        "cudnnSetConvolutionNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(computeType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &computeType,
        "cudnnSetConvolutionNdDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetConvolutionNdDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
    arrayLengthRequested: ::core::ffi::c_int,
    arrayLength: *mut ::core::ffi::c_int,
    padA: *mut ::core::ffi::c_int,
    strideA: *mut ::core::ffi::c_int,
    dilationA: *mut ::core::ffi::c_int,
    mode: *mut cuda_types::cudnn8::cudnnConvolutionMode_t,
    computeType: *mut cuda_types::cudnn8::cudnnDataType_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &convDesc,
        "cudnnGetConvolutionNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(arrayLengthRequested), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &arrayLengthRequested,
        "cudnnGetConvolutionNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(arrayLength), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &arrayLength,
        "cudnnGetConvolutionNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(padA), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &padA,
        "cudnnGetConvolutionNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(strideA), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &strideA,
        "cudnnGetConvolutionNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dilationA), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dilationA,
        "cudnnGetConvolutionNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mode,
        "cudnnGetConvolutionNdDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(computeType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &computeType,
        "cudnnGetConvolutionNdDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetConvolution2dForwardOutputDim(
    writer: &mut (impl std::io::Write + ?Sized),
    convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
    inputTensorDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    filterDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
    n: *mut ::core::ffi::c_int,
    c: *mut ::core::ffi::c_int,
    h: *mut ::core::ffi::c_int,
    w: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &convDesc,
        "cudnnGetConvolution2dForwardOutputDim",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(inputTensorDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &inputTensorDesc,
        "cudnnGetConvolution2dForwardOutputDim",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(filterDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &filterDesc,
        "cudnnGetConvolution2dForwardOutputDim",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(n), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &n,
        "cudnnGetConvolution2dForwardOutputDim",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(c), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &c,
        "cudnnGetConvolution2dForwardOutputDim",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(h), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &h,
        "cudnnGetConvolution2dForwardOutputDim",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(w), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &w,
        "cudnnGetConvolution2dForwardOutputDim",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetConvolutionNdForwardOutputDim(
    writer: &mut (impl std::io::Write + ?Sized),
    convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
    inputTensorDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    filterDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
    nbDims: ::core::ffi::c_int,
    tensorOuputDimA: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &convDesc,
        "cudnnGetConvolutionNdForwardOutputDim",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(inputTensorDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &inputTensorDesc,
        "cudnnGetConvolutionNdForwardOutputDim",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(filterDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &filterDesc,
        "cudnnGetConvolutionNdForwardOutputDim",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nbDims), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nbDims,
        "cudnnGetConvolutionNdForwardOutputDim",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(tensorOuputDimA), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &tensorOuputDimA,
        "cudnnGetConvolutionNdForwardOutputDim",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetConvolutionForwardAlgorithmMaxCount(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    count: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnGetConvolutionForwardAlgorithmMaxCount",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &count,
        "cudnnGetConvolutionForwardAlgorithmMaxCount",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetConvolutionForwardAlgorithm_v7(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    srcDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    filterDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
    convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
    destDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    requestedAlgoCount: ::core::ffi::c_int,
    returnedAlgoCount: *mut ::core::ffi::c_int,
    perfResults: *mut cuda_types::cudnn8::cudnnConvolutionFwdAlgoPerf_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnGetConvolutionForwardAlgorithm_v7",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &srcDesc,
        "cudnnGetConvolutionForwardAlgorithm_v7",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(filterDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &filterDesc,
        "cudnnGetConvolutionForwardAlgorithm_v7",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &convDesc,
        "cudnnGetConvolutionForwardAlgorithm_v7",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(destDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &destDesc,
        "cudnnGetConvolutionForwardAlgorithm_v7",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(requestedAlgoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &requestedAlgoCount,
        "cudnnGetConvolutionForwardAlgorithm_v7",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(returnedAlgoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &returnedAlgoCount,
        "cudnnGetConvolutionForwardAlgorithm_v7",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(perfResults), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &perfResults,
        "cudnnGetConvolutionForwardAlgorithm_v7",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnFindConvolutionForwardAlgorithm(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    wDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
    convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
    yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    requestedAlgoCount: ::core::ffi::c_int,
    returnedAlgoCount: *mut ::core::ffi::c_int,
    perfResults: *mut cuda_types::cudnn8::cudnnConvolutionFwdAlgoPerf_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnFindConvolutionForwardAlgorithm",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xDesc,
        "cudnnFindConvolutionForwardAlgorithm",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(wDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &wDesc,
        "cudnnFindConvolutionForwardAlgorithm",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &convDesc,
        "cudnnFindConvolutionForwardAlgorithm",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &yDesc,
        "cudnnFindConvolutionForwardAlgorithm",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(requestedAlgoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &requestedAlgoCount,
        "cudnnFindConvolutionForwardAlgorithm",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(returnedAlgoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &returnedAlgoCount,
        "cudnnFindConvolutionForwardAlgorithm",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(perfResults), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &perfResults,
        "cudnnFindConvolutionForwardAlgorithm",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnFindConvolutionForwardAlgorithmEx(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnFindConvolutionForwardAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xDesc,
        "cudnnFindConvolutionForwardAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &x,
        "cudnnFindConvolutionForwardAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(wDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &wDesc,
        "cudnnFindConvolutionForwardAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(w), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &w,
        "cudnnFindConvolutionForwardAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &convDesc,
        "cudnnFindConvolutionForwardAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &yDesc,
        "cudnnFindConvolutionForwardAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &y,
        "cudnnFindConvolutionForwardAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(requestedAlgoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &requestedAlgoCount,
        "cudnnFindConvolutionForwardAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(returnedAlgoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &returnedAlgoCount,
        "cudnnFindConvolutionForwardAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(perfResults), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &perfResults,
        "cudnnFindConvolutionForwardAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpace,
        "cudnnFindConvolutionForwardAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSizeInBytes,
        "cudnnFindConvolutionForwardAlgorithmEx",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnIm2Col(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    wDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
    convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
    colBuffer: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnIm2Col", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&xDesc, "cudnnIm2Col", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(&x, "cudnnIm2Col", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(wDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&wDesc, "cudnnIm2Col", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&convDesc, "cudnnIm2Col", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(colBuffer), ": ").as_bytes())?;
    crate::CudaDisplay::write(&colBuffer, "cudnnIm2Col", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnReorderFilterAndBias(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    filterDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
    reorderType: cuda_types::cudnn8::cudnnReorderType_t,
    filterData: *const ::core::ffi::c_void,
    reorderedFilterData: *mut ::core::ffi::c_void,
    reorderBias: ::core::ffi::c_int,
    biasData: *const ::core::ffi::c_void,
    reorderedBiasData: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnReorderFilterAndBias", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(filterDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &filterDesc,
        "cudnnReorderFilterAndBias",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reorderType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reorderType,
        "cudnnReorderFilterAndBias",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(filterData), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &filterData,
        "cudnnReorderFilterAndBias",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reorderedFilterData), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reorderedFilterData,
        "cudnnReorderFilterAndBias",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reorderBias), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reorderBias,
        "cudnnReorderFilterAndBias",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(biasData), ": ").as_bytes())?;
    crate::CudaDisplay::write(&biasData, "cudnnReorderFilterAndBias", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reorderedBiasData), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reorderedBiasData,
        "cudnnReorderFilterAndBias",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetConvolutionForwardWorkspaceSize(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    wDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
    convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
    yDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    algo: cuda_types::cudnn8::cudnnConvolutionFwdAlgo_t,
    sizeInBytes: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnGetConvolutionForwardWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xDesc,
        "cudnnGetConvolutionForwardWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(wDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &wDesc,
        "cudnnGetConvolutionForwardWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &convDesc,
        "cudnnGetConvolutionForwardWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &yDesc,
        "cudnnGetConvolutionForwardWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &algo,
        "cudnnGetConvolutionForwardWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeInBytes,
        "cudnnGetConvolutionForwardWorkspaceSize",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnConvolutionForward(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnConvolutionForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(&alpha, "cudnnConvolutionForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&xDesc, "cudnnConvolutionForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(&x, "cudnnConvolutionForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(wDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&wDesc, "cudnnConvolutionForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(w), ": ").as_bytes())?;
    crate::CudaDisplay::write(&w, "cudnnConvolutionForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&convDesc, "cudnnConvolutionForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algo), ": ").as_bytes())?;
    crate::CudaDisplay::write(&algo, "cudnnConvolutionForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(&workSpace, "cudnnConvolutionForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSizeInBytes,
        "cudnnConvolutionForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(&beta, "cudnnConvolutionForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&yDesc, "cudnnConvolutionForward", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(&y, "cudnnConvolutionForward", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnConvolutionBiasActivationForward(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnConvolutionBiasActivationForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &alpha1,
        "cudnnConvolutionBiasActivationForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xDesc,
        "cudnnConvolutionBiasActivationForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &x,
        "cudnnConvolutionBiasActivationForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(wDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &wDesc,
        "cudnnConvolutionBiasActivationForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(w), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &w,
        "cudnnConvolutionBiasActivationForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &convDesc,
        "cudnnConvolutionBiasActivationForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &algo,
        "cudnnConvolutionBiasActivationForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpace,
        "cudnnConvolutionBiasActivationForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSizeInBytes,
        "cudnnConvolutionBiasActivationForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &alpha2,
        "cudnnConvolutionBiasActivationForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(zDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &zDesc,
        "cudnnConvolutionBiasActivationForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(z), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &z,
        "cudnnConvolutionBiasActivationForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(biasDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &biasDesc,
        "cudnnConvolutionBiasActivationForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bias), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &bias,
        "cudnnConvolutionBiasActivationForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(activationDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &activationDesc,
        "cudnnConvolutionBiasActivationForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(yDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &yDesc,
        "cudnnConvolutionBiasActivationForward",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &y,
        "cudnnConvolutionBiasActivationForward",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::cudnn8::cudnnConvolutionBwdDataAlgoPerfStruct {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(algo), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.algo, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(status), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.status, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(time), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.time, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(memory), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.memory, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(determinism), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.determinism, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(mathType), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.mathType, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
pub fn write_cudnnGetConvolutionBackwardDataAlgorithmMaxCount(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    count: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnGetConvolutionBackwardDataAlgorithmMaxCount",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &count,
        "cudnnGetConvolutionBackwardDataAlgorithmMaxCount",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnFindConvolutionBackwardDataAlgorithm(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    wDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
    dyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
    dxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    requestedAlgoCount: ::core::ffi::c_int,
    returnedAlgoCount: *mut ::core::ffi::c_int,
    perfResults: *mut cuda_types::cudnn8::cudnnConvolutionBwdDataAlgoPerf_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnFindConvolutionBackwardDataAlgorithm",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(wDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &wDesc,
        "cudnnFindConvolutionBackwardDataAlgorithm",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dyDesc,
        "cudnnFindConvolutionBackwardDataAlgorithm",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &convDesc,
        "cudnnFindConvolutionBackwardDataAlgorithm",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dxDesc,
        "cudnnFindConvolutionBackwardDataAlgorithm",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(requestedAlgoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &requestedAlgoCount,
        "cudnnFindConvolutionBackwardDataAlgorithm",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(returnedAlgoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &returnedAlgoCount,
        "cudnnFindConvolutionBackwardDataAlgorithm",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(perfResults), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &perfResults,
        "cudnnFindConvolutionBackwardDataAlgorithm",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnFindConvolutionBackwardDataAlgorithmEx(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnFindConvolutionBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(wDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &wDesc,
        "cudnnFindConvolutionBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(w), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &w,
        "cudnnFindConvolutionBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dyDesc,
        "cudnnFindConvolutionBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dy), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dy,
        "cudnnFindConvolutionBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &convDesc,
        "cudnnFindConvolutionBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dxDesc,
        "cudnnFindConvolutionBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dx), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dx,
        "cudnnFindConvolutionBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(requestedAlgoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &requestedAlgoCount,
        "cudnnFindConvolutionBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(returnedAlgoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &returnedAlgoCount,
        "cudnnFindConvolutionBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(perfResults), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &perfResults,
        "cudnnFindConvolutionBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpace,
        "cudnnFindConvolutionBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSizeInBytes,
        "cudnnFindConvolutionBackwardDataAlgorithmEx",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetConvolutionBackwardDataAlgorithm_v7(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    filterDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
    diffDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
    gradDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    requestedAlgoCount: ::core::ffi::c_int,
    returnedAlgoCount: *mut ::core::ffi::c_int,
    perfResults: *mut cuda_types::cudnn8::cudnnConvolutionBwdDataAlgoPerf_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnGetConvolutionBackwardDataAlgorithm_v7",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(filterDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &filterDesc,
        "cudnnGetConvolutionBackwardDataAlgorithm_v7",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(diffDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &diffDesc,
        "cudnnGetConvolutionBackwardDataAlgorithm_v7",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &convDesc,
        "cudnnGetConvolutionBackwardDataAlgorithm_v7",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gradDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gradDesc,
        "cudnnGetConvolutionBackwardDataAlgorithm_v7",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(requestedAlgoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &requestedAlgoCount,
        "cudnnGetConvolutionBackwardDataAlgorithm_v7",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(returnedAlgoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &returnedAlgoCount,
        "cudnnGetConvolutionBackwardDataAlgorithm_v7",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(perfResults), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &perfResults,
        "cudnnGetConvolutionBackwardDataAlgorithm_v7",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetConvolutionBackwardDataWorkspaceSize(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    wDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
    dyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
    dxDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    algo: cuda_types::cudnn8::cudnnConvolutionBwdDataAlgo_t,
    sizeInBytes: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnGetConvolutionBackwardDataWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(wDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &wDesc,
        "cudnnGetConvolutionBackwardDataWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dyDesc,
        "cudnnGetConvolutionBackwardDataWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &convDesc,
        "cudnnGetConvolutionBackwardDataWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dxDesc,
        "cudnnGetConvolutionBackwardDataWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &algo,
        "cudnnGetConvolutionBackwardDataWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeInBytes,
        "cudnnGetConvolutionBackwardDataWorkspaceSize",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnConvolutionBackwardData(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnConvolutionBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(&alpha, "cudnnConvolutionBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(wDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&wDesc, "cudnnConvolutionBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(w), ": ").as_bytes())?;
    crate::CudaDisplay::write(&w, "cudnnConvolutionBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dyDesc, "cudnnConvolutionBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dy, "cudnnConvolutionBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &convDesc,
        "cudnnConvolutionBackwardData",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algo), ": ").as_bytes())?;
    crate::CudaDisplay::write(&algo, "cudnnConvolutionBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpace,
        "cudnnConvolutionBackwardData",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSizeInBytes,
        "cudnnConvolutionBackwardData",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(&beta, "cudnnConvolutionBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dxDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dxDesc, "cudnnConvolutionBackwardData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dx, "cudnnConvolutionBackwardData", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnGetFoldedConvBackwardDataDescriptors(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnGetFoldedConvBackwardDataDescriptors",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(filterDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &filterDesc,
        "cudnnGetFoldedConvBackwardDataDescriptors",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(diffDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &diffDesc,
        "cudnnGetFoldedConvBackwardDataDescriptors",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &convDesc,
        "cudnnGetFoldedConvBackwardDataDescriptors",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gradDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gradDesc,
        "cudnnGetFoldedConvBackwardDataDescriptors",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(transformFormat), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &transformFormat,
        "cudnnGetFoldedConvBackwardDataDescriptors",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(foldedFilterDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &foldedFilterDesc,
        "cudnnGetFoldedConvBackwardDataDescriptors",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(paddedDiffDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &paddedDiffDesc,
        "cudnnGetFoldedConvBackwardDataDescriptors",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(foldedConvDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &foldedConvDesc,
        "cudnnGetFoldedConvBackwardDataDescriptors",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(foldedGradDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &foldedGradDesc,
        "cudnnGetFoldedConvBackwardDataDescriptors",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(filterFoldTransDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &filterFoldTransDesc,
        "cudnnGetFoldedConvBackwardDataDescriptors",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(diffPadTransDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &diffPadTransDesc,
        "cudnnGetFoldedConvBackwardDataDescriptors",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gradFoldTransDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gradFoldTransDesc,
        "cudnnGetFoldedConvBackwardDataDescriptors",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gradUnfoldTransDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gradUnfoldTransDesc,
        "cudnnGetFoldedConvBackwardDataDescriptors",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnCnnInferVersionCheck(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
impl crate::CudaDisplay for cuda_types::cudnn8::cudnnConvolutionBwdFilterAlgoPerfStruct {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(algo), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.algo, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(status), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.status, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(time), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.time, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(memory), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.memory, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(determinism), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.determinism, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(mathType), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.mathType, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
pub fn write_cudnnGetConvolutionBackwardFilterAlgorithmMaxCount(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    count: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnGetConvolutionBackwardFilterAlgorithmMaxCount",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &count,
        "cudnnGetConvolutionBackwardFilterAlgorithmMaxCount",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnFindConvolutionBackwardFilterAlgorithm(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    dyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
    dwDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
    requestedAlgoCount: ::core::ffi::c_int,
    returnedAlgoCount: *mut ::core::ffi::c_int,
    perfResults: *mut cuda_types::cudnn8::cudnnConvolutionBwdFilterAlgoPerf_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnFindConvolutionBackwardFilterAlgorithm",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xDesc,
        "cudnnFindConvolutionBackwardFilterAlgorithm",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dyDesc,
        "cudnnFindConvolutionBackwardFilterAlgorithm",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &convDesc,
        "cudnnFindConvolutionBackwardFilterAlgorithm",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dwDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dwDesc,
        "cudnnFindConvolutionBackwardFilterAlgorithm",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(requestedAlgoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &requestedAlgoCount,
        "cudnnFindConvolutionBackwardFilterAlgorithm",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(returnedAlgoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &returnedAlgoCount,
        "cudnnFindConvolutionBackwardFilterAlgorithm",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(perfResults), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &perfResults,
        "cudnnFindConvolutionBackwardFilterAlgorithm",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnFindConvolutionBackwardFilterAlgorithmEx(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnFindConvolutionBackwardFilterAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xDesc,
        "cudnnFindConvolutionBackwardFilterAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &x,
        "cudnnFindConvolutionBackwardFilterAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dyDesc,
        "cudnnFindConvolutionBackwardFilterAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &y,
        "cudnnFindConvolutionBackwardFilterAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &convDesc,
        "cudnnFindConvolutionBackwardFilterAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dwDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dwDesc,
        "cudnnFindConvolutionBackwardFilterAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dw), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dw,
        "cudnnFindConvolutionBackwardFilterAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(requestedAlgoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &requestedAlgoCount,
        "cudnnFindConvolutionBackwardFilterAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(returnedAlgoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &returnedAlgoCount,
        "cudnnFindConvolutionBackwardFilterAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(perfResults), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &perfResults,
        "cudnnFindConvolutionBackwardFilterAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpace,
        "cudnnFindConvolutionBackwardFilterAlgorithmEx",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSizeInBytes,
        "cudnnFindConvolutionBackwardFilterAlgorithmEx",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetConvolutionBackwardFilterAlgorithm_v7(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    srcDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    diffDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
    gradDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
    requestedAlgoCount: ::core::ffi::c_int,
    returnedAlgoCount: *mut ::core::ffi::c_int,
    perfResults: *mut cuda_types::cudnn8::cudnnConvolutionBwdFilterAlgoPerf_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnGetConvolutionBackwardFilterAlgorithm_v7",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &srcDesc,
        "cudnnGetConvolutionBackwardFilterAlgorithm_v7",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(diffDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &diffDesc,
        "cudnnGetConvolutionBackwardFilterAlgorithm_v7",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &convDesc,
        "cudnnGetConvolutionBackwardFilterAlgorithm_v7",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gradDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gradDesc,
        "cudnnGetConvolutionBackwardFilterAlgorithm_v7",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(requestedAlgoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &requestedAlgoCount,
        "cudnnGetConvolutionBackwardFilterAlgorithm_v7",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(returnedAlgoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &returnedAlgoCount,
        "cudnnGetConvolutionBackwardFilterAlgorithm_v7",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(perfResults), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &perfResults,
        "cudnnGetConvolutionBackwardFilterAlgorithm_v7",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetConvolutionBackwardFilterWorkspaceSize(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    xDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    dyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    convDesc: cuda_types::cudnn8::cudnnConvolutionDescriptor_t,
    gradDesc: cuda_types::cudnn8::cudnnFilterDescriptor_t,
    algo: cuda_types::cudnn8::cudnnConvolutionBwdFilterAlgo_t,
    sizeInBytes: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnGetConvolutionBackwardFilterWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xDesc,
        "cudnnGetConvolutionBackwardFilterWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dyDesc,
        "cudnnGetConvolutionBackwardFilterWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &convDesc,
        "cudnnGetConvolutionBackwardFilterWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gradDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gradDesc,
        "cudnnGetConvolutionBackwardFilterWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &algo,
        "cudnnGetConvolutionBackwardFilterWorkspaceSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeInBytes,
        "cudnnGetConvolutionBackwardFilterWorkspaceSize",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnConvolutionBackwardFilter(
    writer: &mut (impl std::io::Write + ?Sized),
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
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnConvolutionBackwardFilter",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &alpha,
        "cudnnConvolutionBackwardFilter",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &xDesc,
        "cudnnConvolutionBackwardFilter",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(&x, "cudnnConvolutionBackwardFilter", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dyDesc,
        "cudnnConvolutionBackwardFilter",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dy, "cudnnConvolutionBackwardFilter", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(convDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &convDesc,
        "cudnnConvolutionBackwardFilter",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(algo), ": ").as_bytes())?;
    crate::CudaDisplay::write(&algo, "cudnnConvolutionBackwardFilter", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpace), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpace,
        "cudnnConvolutionBackwardFilter",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workSpaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workSpaceSizeInBytes,
        "cudnnConvolutionBackwardFilter",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(&beta, "cudnnConvolutionBackwardFilter", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dwDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dwDesc,
        "cudnnConvolutionBackwardFilter",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dw), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dw, "cudnnConvolutionBackwardFilter", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnConvolutionBackwardBias(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    alpha: *const ::core::ffi::c_void,
    dyDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    dy: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    dbDesc: cuda_types::cudnn8::cudnnTensorDescriptor_t,
    db: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnConvolutionBackwardBias", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alpha), ": ").as_bytes())?;
    crate::CudaDisplay::write(&alpha, "cudnnConvolutionBackwardBias", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dyDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dyDesc, "cudnnConvolutionBackwardBias", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dy, "cudnnConvolutionBackwardBias", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(beta), ": ").as_bytes())?;
    crate::CudaDisplay::write(&beta, "cudnnConvolutionBackwardBias", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dbDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dbDesc, "cudnnConvolutionBackwardBias", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(db), ": ").as_bytes())?;
    crate::CudaDisplay::write(&db, "cudnnConvolutionBackwardBias", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnCreateFusedOpsConstParamPack(
    writer: &mut (impl std::io::Write + ?Sized),
    constPack: *mut cuda_types::cudnn8::cudnnFusedOpsConstParamPack_t,
    ops: cuda_types::cudnn8::cudnnFusedOps_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(constPack), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &constPack,
        "cudnnCreateFusedOpsConstParamPack",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ops), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &ops,
        "cudnnCreateFusedOpsConstParamPack",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnDestroyFusedOpsConstParamPack(
    writer: &mut (impl std::io::Write + ?Sized),
    constPack: cuda_types::cudnn8::cudnnFusedOpsConstParamPack_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(constPack), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &constPack,
        "cudnnDestroyFusedOpsConstParamPack",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnSetFusedOpsConstParamPackAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    constPack: cuda_types::cudnn8::cudnnFusedOpsConstParamPack_t,
    paramLabel: cuda_types::cudnn8::cudnnFusedOpsConstParamLabel_t,
    param: *const ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(constPack), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &constPack,
        "cudnnSetFusedOpsConstParamPackAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(paramLabel), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &paramLabel,
        "cudnnSetFusedOpsConstParamPackAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param,
        "cudnnSetFusedOpsConstParamPackAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetFusedOpsConstParamPackAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    constPack: cuda_types::cudnn8::cudnnFusedOpsConstParamPack_t,
    paramLabel: cuda_types::cudnn8::cudnnFusedOpsConstParamLabel_t,
    param: *mut ::core::ffi::c_void,
    isNULL: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(constPack), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &constPack,
        "cudnnGetFusedOpsConstParamPackAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(paramLabel), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &paramLabel,
        "cudnnGetFusedOpsConstParamPackAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(param), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &param,
        "cudnnGetFusedOpsConstParamPackAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(isNULL), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &isNULL,
        "cudnnGetFusedOpsConstParamPackAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnCreateFusedOpsVariantParamPack(
    writer: &mut (impl std::io::Write + ?Sized),
    varPack: *mut cuda_types::cudnn8::cudnnFusedOpsVariantParamPack_t,
    ops: cuda_types::cudnn8::cudnnFusedOps_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(varPack), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &varPack,
        "cudnnCreateFusedOpsVariantParamPack",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ops), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &ops,
        "cudnnCreateFusedOpsVariantParamPack",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnDestroyFusedOpsVariantParamPack(
    writer: &mut (impl std::io::Write + ?Sized),
    varPack: cuda_types::cudnn8::cudnnFusedOpsVariantParamPack_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(varPack), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &varPack,
        "cudnnDestroyFusedOpsVariantParamPack",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnSetFusedOpsVariantParamPackAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    varPack: cuda_types::cudnn8::cudnnFusedOpsVariantParamPack_t,
    paramLabel: cuda_types::cudnn8::cudnnFusedOpsVariantParamLabel_t,
    ptr: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(varPack), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &varPack,
        "cudnnSetFusedOpsVariantParamPackAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(paramLabel), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &paramLabel,
        "cudnnSetFusedOpsVariantParamPackAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &ptr,
        "cudnnSetFusedOpsVariantParamPackAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetFusedOpsVariantParamPackAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    varPack: cuda_types::cudnn8::cudnnFusedOpsVariantParamPack_t,
    paramLabel: cuda_types::cudnn8::cudnnFusedOpsVariantParamLabel_t,
    ptr: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(varPack), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &varPack,
        "cudnnGetFusedOpsVariantParamPackAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(paramLabel), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &paramLabel,
        "cudnnGetFusedOpsVariantParamPackAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &ptr,
        "cudnnGetFusedOpsVariantParamPackAttribute",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnCreateFusedOpsPlan(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: *mut cuda_types::cudnn8::cudnnFusedOpsPlan_t,
    ops: cuda_types::cudnn8::cudnnFusedOps_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cudnnCreateFusedOpsPlan", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ops), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ops, "cudnnCreateFusedOpsPlan", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnDestroyFusedOpsPlan(
    writer: &mut (impl std::io::Write + ?Sized),
    plan: cuda_types::cudnn8::cudnnFusedOpsPlan_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cudnnDestroyFusedOpsPlan", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnMakeFusedOpsPlan(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    plan: cuda_types::cudnn8::cudnnFusedOpsPlan_t,
    constPack: cuda_types::cudnn8::cudnnFusedOpsConstParamPack_t,
    workspaceSizeInBytes: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnMakeFusedOpsPlan", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cudnnMakeFusedOpsPlan", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(constPack), ": ").as_bytes())?;
    crate::CudaDisplay::write(&constPack, "cudnnMakeFusedOpsPlan", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(workspaceSizeInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &workspaceSizeInBytes,
        "cudnnMakeFusedOpsPlan",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnFusedOpsExecute(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    plan: cuda_types::cudnn8::cudnnFusedOpsPlan_t,
    varPack: cuda_types::cudnn8::cudnnFusedOpsVariantParamPack_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnFusedOpsExecute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cudnnFusedOpsExecute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(varPack), ": ").as_bytes())?;
    crate::CudaDisplay::write(&varPack, "cudnnFusedOpsExecute", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnCnnTrainVersionCheck(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
impl crate::CudaDisplay for cuda_types::cudnn8::cudnnBackendNumericalNote_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn8::cudnnBackendNumericalNote_t::CUDNN_NUMERICAL_NOTE_TENSOR_CORE => {
                writer.write_all(stringify!(CUDNN_NUMERICAL_NOTE_TENSOR_CORE).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendNumericalNote_t::CUDNN_NUMERICAL_NOTE_DOWN_CONVERT_INPUTS => {
                writer
                    .write_all(
                        stringify!(CUDNN_NUMERICAL_NOTE_DOWN_CONVERT_INPUTS).as_bytes(),
                    )
            }
            &cuda_types::cudnn8::cudnnBackendNumericalNote_t::CUDNN_NUMERICAL_NOTE_REDUCED_PRECISION_REDUCTION => {
                writer
                    .write_all(
                        stringify!(CUDNN_NUMERICAL_NOTE_REDUCED_PRECISION_REDUCTION)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn8::cudnnBackendNumericalNote_t::CUDNN_NUMERICAL_NOTE_FFT => {
                writer.write_all(stringify!(CUDNN_NUMERICAL_NOTE_FFT).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendNumericalNote_t::CUDNN_NUMERICAL_NOTE_NONDETERMINISTIC => {
                writer
                    .write_all(
                        stringify!(CUDNN_NUMERICAL_NOTE_NONDETERMINISTIC).as_bytes(),
                    )
            }
            &cuda_types::cudnn8::cudnnBackendNumericalNote_t::CUDNN_NUMERICAL_NOTE_WINOGRAD => {
                writer.write_all(stringify!(CUDNN_NUMERICAL_NOTE_WINOGRAD).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendNumericalNote_t::CUDNN_NUMERICAL_NOTE_WINOGRAD_TILE_4x4 => {
                writer
                    .write_all(
                        stringify!(CUDNN_NUMERICAL_NOTE_WINOGRAD_TILE_4x4).as_bytes(),
                    )
            }
            &cuda_types::cudnn8::cudnnBackendNumericalNote_t::CUDNN_NUMERICAL_NOTE_WINOGRAD_TILE_6x6 => {
                writer
                    .write_all(
                        stringify!(CUDNN_NUMERICAL_NOTE_WINOGRAD_TILE_6x6).as_bytes(),
                    )
            }
            &cuda_types::cudnn8::cudnnBackendNumericalNote_t::CUDNN_NUMERICAL_NOTE_WINOGRAD_TILE_13x13 => {
                writer
                    .write_all(
                        stringify!(CUDNN_NUMERICAL_NOTE_WINOGRAD_TILE_13x13).as_bytes(),
                    )
            }
            &cuda_types::cudnn8::cudnnBackendNumericalNote_t::CUDNN_NUMERICAL_NOTE_TYPE_COUNT => {
                writer.write_all(stringify!(CUDNN_NUMERICAL_NOTE_TYPE_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn8::cudnnBackendBehaviorNote_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn8::cudnnBackendBehaviorNote_t::CUDNN_BEHAVIOR_NOTE_RUNTIME_COMPILATION => {
                writer
                    .write_all(
                        stringify!(CUDNN_BEHAVIOR_NOTE_RUNTIME_COMPILATION).as_bytes(),
                    )
            }
            &cuda_types::cudnn8::cudnnBackendBehaviorNote_t::CUDNN_BEHAVIOR_NOTE_REQUIRES_FILTER_INT8x32_REORDER => {
                writer
                    .write_all(
                        stringify!(CUDNN_BEHAVIOR_NOTE_REQUIRES_FILTER_INT8x32_REORDER)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn8::cudnnBackendBehaviorNote_t::CUDNN_BEHAVIOR_NOTE_REQUIRES_BIAS_INT8x32_REORDER => {
                writer
                    .write_all(
                        stringify!(CUDNN_BEHAVIOR_NOTE_REQUIRES_BIAS_INT8x32_REORDER)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn8::cudnnBackendBehaviorNote_t::CUDNN_BEHAVIOR_NOTE_TYPE_COUNT => {
                writer.write_all(stringify!(CUDNN_BEHAVIOR_NOTE_TYPE_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn8::cudnnBackendKnobType_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_SPLIT_K => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_SPLIT_K).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_SWIZZLE => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_SWIZZLE).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_TILE_SIZE => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_TILE_SIZE).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_USE_TEX => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_USE_TEX).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_EDGE => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_EDGE).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_KBLOCK => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_KBLOCK).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_LDGA => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_LDGA).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_LDGB => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_LDGB).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_CHUNK_K => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_CHUNK_K).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_SPLIT_H => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_SPLIT_H).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_WINO_TILE => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_WINO_TILE).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_MULTIPLY => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_MULTIPLY).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_SPLIT_K_BUF => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_SPLIT_K_BUF).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_TILEK => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_TILEK).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_STAGES => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_STAGES).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_REDUCTION_MODE => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_REDUCTION_MODE).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_CTA_SPLIT_K_MODE => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_CTA_SPLIT_K_MODE).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_SPLIT_K_SLC => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_SPLIT_K_SLC).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_IDX_MODE => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_IDX_MODE).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_SLICED => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_SLICED).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_SPLIT_RS => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_SPLIT_RS).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_SINGLEBUFFER => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_SINGLEBUFFER).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_LDGC => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_LDGC).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_SPECFILT => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_SPECFILT).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_KERNEL_CFG => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_KERNEL_CFG).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_WORKSPACE => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_WORKSPACE).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_TILE_CGA => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_TILE_CGA).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_TILE_CGA_M => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_TILE_CGA_M).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_TILE_CGA_N => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_TILE_CGA_N).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_BLOCK_SIZE => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_BLOCK_SIZE).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_OCCUPANCY => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_OCCUPANCY).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_ARRAY_SIZE_PER_THREAD => {
                writer
                    .write_all(
                        stringify!(CUDNN_KNOB_TYPE_ARRAY_SIZE_PER_THREAD).as_bytes(),
                    )
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_NUM_C_PER_BLOCK => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_NUM_C_PER_BLOCK).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_SPLIT_COLS => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_SPLIT_COLS).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_TILE_ROWS => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_TILE_ROWS).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_TILE_COLS => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_TILE_COLS).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_LOAD_SIZE => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_LOAD_SIZE).as_bytes())
            }
            &cuda_types::cudnn8::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_COUNTS => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_COUNTS).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cudnnBackendCreateDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    descriptorType: cuda_types::cudnn8::cudnnBackendDescriptorType_t,
    descriptor: *mut cuda_types::cudnn8::cudnnBackendDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(descriptorType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &descriptorType,
        "cudnnBackendCreateDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(descriptor), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &descriptor,
        "cudnnBackendCreateDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnBackendDestroyDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    descriptor: cuda_types::cudnn8::cudnnBackendDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(descriptor), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &descriptor,
        "cudnnBackendDestroyDescriptor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnBackendInitialize(
    writer: &mut (impl std::io::Write + ?Sized),
    descriptor: cuda_types::cudnn8::cudnnBackendDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(descriptor), ": ").as_bytes())?;
    crate::CudaDisplay::write(&descriptor, "cudnnBackendInitialize", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnBackendFinalize(
    writer: &mut (impl std::io::Write + ?Sized),
    descriptor: cuda_types::cudnn8::cudnnBackendDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(descriptor), ": ").as_bytes())?;
    crate::CudaDisplay::write(&descriptor, "cudnnBackendFinalize", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnBackendExecute(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn8::cudnnHandle_t,
    executionPlan: cuda_types::cudnn8::cudnnBackendDescriptor_t,
    variantPack: cuda_types::cudnn8::cudnnBackendDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnBackendExecute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(executionPlan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&executionPlan, "cudnnBackendExecute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(variantPack), ": ").as_bytes())?;
    crate::CudaDisplay::write(&variantPack, "cudnnBackendExecute", arg_idx, writer)?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::cudnn8::cudnnStatus_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            Ok(()) => writer.write_all(b"CUDNN_STATUS_SUCCESS"),
            Err(err) => {
                match err.0.get() {
                    1 => writer.write_all("CUDNN_STATUS_NOT_INITIALIZED".as_bytes()),
                    2 => writer.write_all("CUDNN_STATUS_ALLOC_FAILED".as_bytes()),
                    3 => writer.write_all("CUDNN_STATUS_BAD_PARAM".as_bytes()),
                    4 => writer.write_all("CUDNN_STATUS_INTERNAL_ERROR".as_bytes()),
                    5 => writer.write_all("CUDNN_STATUS_INVALID_VALUE".as_bytes()),
                    6 => writer.write_all("CUDNN_STATUS_ARCH_MISMATCH".as_bytes()),
                    7 => writer.write_all("CUDNN_STATUS_MAPPING_ERROR".as_bytes()),
                    8 => writer.write_all("CUDNN_STATUS_EXECUTION_FAILED".as_bytes()),
                    9 => writer.write_all("CUDNN_STATUS_NOT_SUPPORTED".as_bytes()),
                    10 => writer.write_all("CUDNN_STATUS_LICENSE_ERROR".as_bytes()),
                    11 => {
                        writer
                            .write_all(
                                "CUDNN_STATUS_RUNTIME_PREREQUISITE_MISSING".as_bytes(),
                            )
                    }
                    12 => writer.write_all("CUDNN_STATUS_RUNTIME_IN_PROGRESS".as_bytes()),
                    13 => writer.write_all("CUDNN_STATUS_RUNTIME_FP_OVERFLOW".as_bytes()),
                    14 => writer.write_all("CUDNN_STATUS_VERSION_MISMATCH".as_bytes()),
                    err => write!(writer, "{}", err),
                }
            }
        }
    }
}
