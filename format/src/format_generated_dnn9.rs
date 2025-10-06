// Generated automatically by zluda_bindgen
// DO NOT EDIT MANUALLY
#![allow(warnings)]
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnHandle_t {
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
    status: cuda_types::cudnn9::cudnnStatus_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(status), ": ").as_bytes())?;
    crate::CudaDisplay::write(&status, "cudnnGetErrorString", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnGetLastErrorString(
    writer: &mut (impl std::io::Write + ?Sized),
    message: *mut ::core::ffi::c_char,
    max_size: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(message), ": ").as_bytes())?;
    crate::CudaDisplay::write(&message, "cudnnGetLastErrorString", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(max_size), ": ").as_bytes())?;
    crate::CudaDisplay::write(&max_size, "cudnnGetLastErrorString", arg_idx, writer)?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnErrQueryMode_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnErrQueryMode_t::CUDNN_ERRQUERY_RAWCODE => {
                writer.write_all(stringify!(CUDNN_ERRQUERY_RAWCODE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnErrQueryMode_t::CUDNN_ERRQUERY_NONBLOCKING => {
                writer.write_all(stringify!(CUDNN_ERRQUERY_NONBLOCKING).as_bytes())
            }
            &cuda_types::cudnn9::cudnnErrQueryMode_t::CUDNN_ERRQUERY_BLOCKING => {
                writer.write_all(stringify!(CUDNN_ERRQUERY_BLOCKING).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cudnnQueryRuntimeError(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn9::cudnnHandle_t,
    rstatus: *mut cuda_types::cudnn9::cudnnStatus_t,
    mode: cuda_types::cudnn9::cudnnErrQueryMode_t,
    tag: *mut cuda_types::cudnn9::cudnnRuntimeTag_t,
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
    type_: cuda_types::cudnn9::libraryPropertyType,
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
    handle: *mut cuda_types::cudnn9::cudnnHandle_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnCreate", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnDestroy(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn9::cudnnHandle_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnDestroy", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnSetStream(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn9::cudnnHandle_t,
    streamId: cuda_types::cudnn9::cudaStream_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    streamId: *mut cuda_types::cudnn9::cudaStream_t,
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnDataType_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnDataType_t::CUDNN_DATA_FLOAT => {
                writer.write_all(stringify!(CUDNN_DATA_FLOAT).as_bytes())
            }
            &cuda_types::cudnn9::cudnnDataType_t::CUDNN_DATA_DOUBLE => {
                writer.write_all(stringify!(CUDNN_DATA_DOUBLE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnDataType_t::CUDNN_DATA_HALF => {
                writer.write_all(stringify!(CUDNN_DATA_HALF).as_bytes())
            }
            &cuda_types::cudnn9::cudnnDataType_t::CUDNN_DATA_INT8 => {
                writer.write_all(stringify!(CUDNN_DATA_INT8).as_bytes())
            }
            &cuda_types::cudnn9::cudnnDataType_t::CUDNN_DATA_INT32 => {
                writer.write_all(stringify!(CUDNN_DATA_INT32).as_bytes())
            }
            &cuda_types::cudnn9::cudnnDataType_t::CUDNN_DATA_INT8x4 => {
                writer.write_all(stringify!(CUDNN_DATA_INT8x4).as_bytes())
            }
            &cuda_types::cudnn9::cudnnDataType_t::CUDNN_DATA_UINT8 => {
                writer.write_all(stringify!(CUDNN_DATA_UINT8).as_bytes())
            }
            &cuda_types::cudnn9::cudnnDataType_t::CUDNN_DATA_UINT8x4 => {
                writer.write_all(stringify!(CUDNN_DATA_UINT8x4).as_bytes())
            }
            &cuda_types::cudnn9::cudnnDataType_t::CUDNN_DATA_INT8x32 => {
                writer.write_all(stringify!(CUDNN_DATA_INT8x32).as_bytes())
            }
            &cuda_types::cudnn9::cudnnDataType_t::CUDNN_DATA_BFLOAT16 => {
                writer.write_all(stringify!(CUDNN_DATA_BFLOAT16).as_bytes())
            }
            &cuda_types::cudnn9::cudnnDataType_t::CUDNN_DATA_INT64 => {
                writer.write_all(stringify!(CUDNN_DATA_INT64).as_bytes())
            }
            &cuda_types::cudnn9::cudnnDataType_t::CUDNN_DATA_BOOLEAN => {
                writer.write_all(stringify!(CUDNN_DATA_BOOLEAN).as_bytes())
            }
            &cuda_types::cudnn9::cudnnDataType_t::CUDNN_DATA_FP8_E4M3 => {
                writer.write_all(stringify!(CUDNN_DATA_FP8_E4M3).as_bytes())
            }
            &cuda_types::cudnn9::cudnnDataType_t::CUDNN_DATA_FP8_E5M2 => {
                writer.write_all(stringify!(CUDNN_DATA_FP8_E5M2).as_bytes())
            }
            &cuda_types::cudnn9::cudnnDataType_t::CUDNN_DATA_FAST_FLOAT_FOR_FP8 => {
                writer.write_all(stringify!(CUDNN_DATA_FAST_FLOAT_FOR_FP8).as_bytes())
            }
            &cuda_types::cudnn9::cudnnDataType_t::CUDNN_DATA_FP8_E8M0 => {
                writer.write_all(stringify!(CUDNN_DATA_FP8_E8M0).as_bytes())
            }
            &cuda_types::cudnn9::cudnnDataType_t::CUDNN_DATA_FP4_E2M1 => {
                writer.write_all(stringify!(CUDNN_DATA_FP4_E2M1).as_bytes())
            }
            &cuda_types::cudnn9::cudnnDataType_t::CUDNN_DATA_INT4 => {
                writer.write_all(stringify!(CUDNN_DATA_INT4).as_bytes())
            }
            &cuda_types::cudnn9::cudnnDataType_t::CUDNN_DATA_UINT4 => {
                writer.write_all(stringify!(CUDNN_DATA_UINT4).as_bytes())
            }
            &cuda_types::cudnn9::cudnnDataType_t::CUDNN_DATA_UINT32 => {
                writer.write_all(stringify!(CUDNN_DATA_UINT32).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnMathType_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnMathType_t::CUDNN_DEFAULT_MATH => {
                writer.write_all(stringify!(CUDNN_DEFAULT_MATH).as_bytes())
            }
            &cuda_types::cudnn9::cudnnMathType_t::CUDNN_TENSOR_OP_MATH => {
                writer.write_all(stringify!(CUDNN_TENSOR_OP_MATH).as_bytes())
            }
            &cuda_types::cudnn9::cudnnMathType_t::CUDNN_TENSOR_OP_MATH_ALLOW_CONVERSION => {
                writer
                    .write_all(
                        stringify!(CUDNN_TENSOR_OP_MATH_ALLOW_CONVERSION).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnMathType_t::CUDNN_FMA_MATH => {
                writer.write_all(stringify!(CUDNN_FMA_MATH).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnNanPropagation_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnNanPropagation_t::CUDNN_NOT_PROPAGATE_NAN => {
                writer.write_all(stringify!(CUDNN_NOT_PROPAGATE_NAN).as_bytes())
            }
            &cuda_types::cudnn9::cudnnNanPropagation_t::CUDNN_PROPAGATE_NAN => {
                writer.write_all(stringify!(CUDNN_PROPAGATE_NAN).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnCTCGradMode_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnCTCGradMode_t::CUDNN_CTC_ZERO_OOB_GRADIENTS => {
                writer.write_all(stringify!(CUDNN_CTC_ZERO_OOB_GRADIENTS).as_bytes())
            }
            &cuda_types::cudnn9::cudnnCTCGradMode_t::CUDNN_CTC_SKIP_OOB_GRADIENTS => {
                writer.write_all(stringify!(CUDNN_CTC_SKIP_OOB_GRADIENTS).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnTensorFormat_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnTensorFormat_t::CUDNN_TENSOR_NCHW => {
                writer.write_all(stringify!(CUDNN_TENSOR_NCHW).as_bytes())
            }
            &cuda_types::cudnn9::cudnnTensorFormat_t::CUDNN_TENSOR_NHWC => {
                writer.write_all(stringify!(CUDNN_TENSOR_NHWC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnTensorFormat_t::CUDNN_TENSOR_NCHW_VECT_C => {
                writer.write_all(stringify!(CUDNN_TENSOR_NCHW_VECT_C).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnReduceTensorOp_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnReduceTensorOp_t::CUDNN_REDUCE_TENSOR_ADD => {
                writer.write_all(stringify!(CUDNN_REDUCE_TENSOR_ADD).as_bytes())
            }
            &cuda_types::cudnn9::cudnnReduceTensorOp_t::CUDNN_REDUCE_TENSOR_MUL => {
                writer.write_all(stringify!(CUDNN_REDUCE_TENSOR_MUL).as_bytes())
            }
            &cuda_types::cudnn9::cudnnReduceTensorOp_t::CUDNN_REDUCE_TENSOR_MIN => {
                writer.write_all(stringify!(CUDNN_REDUCE_TENSOR_MIN).as_bytes())
            }
            &cuda_types::cudnn9::cudnnReduceTensorOp_t::CUDNN_REDUCE_TENSOR_MAX => {
                writer.write_all(stringify!(CUDNN_REDUCE_TENSOR_MAX).as_bytes())
            }
            &cuda_types::cudnn9::cudnnReduceTensorOp_t::CUDNN_REDUCE_TENSOR_AMAX => {
                writer.write_all(stringify!(CUDNN_REDUCE_TENSOR_AMAX).as_bytes())
            }
            &cuda_types::cudnn9::cudnnReduceTensorOp_t::CUDNN_REDUCE_TENSOR_AVG => {
                writer.write_all(stringify!(CUDNN_REDUCE_TENSOR_AVG).as_bytes())
            }
            &cuda_types::cudnn9::cudnnReduceTensorOp_t::CUDNN_REDUCE_TENSOR_NORM1 => {
                writer.write_all(stringify!(CUDNN_REDUCE_TENSOR_NORM1).as_bytes())
            }
            &cuda_types::cudnn9::cudnnReduceTensorOp_t::CUDNN_REDUCE_TENSOR_NORM2 => {
                writer.write_all(stringify!(CUDNN_REDUCE_TENSOR_NORM2).as_bytes())
            }
            &cuda_types::cudnn9::cudnnReduceTensorOp_t::CUDNN_REDUCE_TENSOR_MUL_NO_ZEROS => {
                writer.write_all(stringify!(CUDNN_REDUCE_TENSOR_MUL_NO_ZEROS).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnActivationMode_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnActivationMode_t::CUDNN_ACTIVATION_SIGMOID => {
                writer.write_all(stringify!(CUDNN_ACTIVATION_SIGMOID).as_bytes())
            }
            &cuda_types::cudnn9::cudnnActivationMode_t::CUDNN_ACTIVATION_RELU => {
                writer.write_all(stringify!(CUDNN_ACTIVATION_RELU).as_bytes())
            }
            &cuda_types::cudnn9::cudnnActivationMode_t::CUDNN_ACTIVATION_TANH => {
                writer.write_all(stringify!(CUDNN_ACTIVATION_TANH).as_bytes())
            }
            &cuda_types::cudnn9::cudnnActivationMode_t::CUDNN_ACTIVATION_CLIPPED_RELU => {
                writer.write_all(stringify!(CUDNN_ACTIVATION_CLIPPED_RELU).as_bytes())
            }
            &cuda_types::cudnn9::cudnnActivationMode_t::CUDNN_ACTIVATION_ELU => {
                writer.write_all(stringify!(CUDNN_ACTIVATION_ELU).as_bytes())
            }
            &cuda_types::cudnn9::cudnnActivationMode_t::CUDNN_ACTIVATION_IDENTITY => {
                writer.write_all(stringify!(CUDNN_ACTIVATION_IDENTITY).as_bytes())
            }
            &cuda_types::cudnn9::cudnnActivationMode_t::CUDNN_ACTIVATION_SWISH => {
                writer.write_all(stringify!(CUDNN_ACTIVATION_SWISH).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnSeverity_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnSeverity_t::CUDNN_SEV_FATAL => {
                writer.write_all(stringify!(CUDNN_SEV_FATAL).as_bytes())
            }
            &cuda_types::cudnn9::cudnnSeverity_t::CUDNN_SEV_ERROR => {
                writer.write_all(stringify!(CUDNN_SEV_ERROR).as_bytes())
            }
            &cuda_types::cudnn9::cudnnSeverity_t::CUDNN_SEV_WARNING => {
                writer.write_all(stringify!(CUDNN_SEV_WARNING).as_bytes())
            }
            &cuda_types::cudnn9::cudnnSeverity_t::CUDNN_SEV_INFO => {
                writer.write_all(stringify!(CUDNN_SEV_INFO).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnDebugStruct {
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnCallback_t {
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
                    cuda_types::cudnn9::cudnnCallback_t,
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
    fptr: cuda_types::cudnn9::cudnnCallback_t,
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
    fptr: *mut cuda_types::cudnn9::cudnnCallback_t,
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
pub fn write_cudnnGraphVersionCheck(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnConvolutionMode_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnConvolutionMode_t::CUDNN_CONVOLUTION => {
                writer.write_all(stringify!(CUDNN_CONVOLUTION).as_bytes())
            }
            &cuda_types::cudnn9::cudnnConvolutionMode_t::CUDNN_CROSS_CORRELATION => {
                writer.write_all(stringify!(CUDNN_CROSS_CORRELATION).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnReorderType_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnReorderType_t::CUDNN_DEFAULT_REORDER => {
                writer.write_all(stringify!(CUDNN_DEFAULT_REORDER).as_bytes())
            }
            &cuda_types::cudnn9::cudnnReorderType_t::CUDNN_NO_REORDER => {
                writer.write_all(stringify!(CUDNN_NO_REORDER).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnFractionStruct {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(numerator), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.numerator, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(denominator), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.denominator, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnPointwiseMode_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_ADD => {
                writer.write_all(stringify!(CUDNN_POINTWISE_ADD).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_ADD_SQUARE => {
                writer.write_all(stringify!(CUDNN_POINTWISE_ADD_SQUARE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_DIV => {
                writer.write_all(stringify!(CUDNN_POINTWISE_DIV).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_MAX => {
                writer.write_all(stringify!(CUDNN_POINTWISE_MAX).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_MIN => {
                writer.write_all(stringify!(CUDNN_POINTWISE_MIN).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_MOD => {
                writer.write_all(stringify!(CUDNN_POINTWISE_MOD).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_MUL => {
                writer.write_all(stringify!(CUDNN_POINTWISE_MUL).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_POW => {
                writer.write_all(stringify!(CUDNN_POINTWISE_POW).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_SUB => {
                writer.write_all(stringify!(CUDNN_POINTWISE_SUB).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_ABS => {
                writer.write_all(stringify!(CUDNN_POINTWISE_ABS).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_CEIL => {
                writer.write_all(stringify!(CUDNN_POINTWISE_CEIL).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_COS => {
                writer.write_all(stringify!(CUDNN_POINTWISE_COS).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_EXP => {
                writer.write_all(stringify!(CUDNN_POINTWISE_EXP).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_FLOOR => {
                writer.write_all(stringify!(CUDNN_POINTWISE_FLOOR).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_LOG => {
                writer.write_all(stringify!(CUDNN_POINTWISE_LOG).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_NEG => {
                writer.write_all(stringify!(CUDNN_POINTWISE_NEG).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_RSQRT => {
                writer.write_all(stringify!(CUDNN_POINTWISE_RSQRT).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_SIN => {
                writer.write_all(stringify!(CUDNN_POINTWISE_SIN).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_SQRT => {
                writer.write_all(stringify!(CUDNN_POINTWISE_SQRT).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_TAN => {
                writer.write_all(stringify!(CUDNN_POINTWISE_TAN).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_ERF => {
                writer.write_all(stringify!(CUDNN_POINTWISE_ERF).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_IDENTITY => {
                writer.write_all(stringify!(CUDNN_POINTWISE_IDENTITY).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_RECIPROCAL => {
                writer.write_all(stringify!(CUDNN_POINTWISE_RECIPROCAL).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_ATAN2 => {
                writer.write_all(stringify!(CUDNN_POINTWISE_ATAN2).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_RELU_FWD => {
                writer.write_all(stringify!(CUDNN_POINTWISE_RELU_FWD).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_TANH_FWD => {
                writer.write_all(stringify!(CUDNN_POINTWISE_TANH_FWD).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_SIGMOID_FWD => {
                writer.write_all(stringify!(CUDNN_POINTWISE_SIGMOID_FWD).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_ELU_FWD => {
                writer.write_all(stringify!(CUDNN_POINTWISE_ELU_FWD).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_GELU_FWD => {
                writer.write_all(stringify!(CUDNN_POINTWISE_GELU_FWD).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_SOFTPLUS_FWD => {
                writer.write_all(stringify!(CUDNN_POINTWISE_SOFTPLUS_FWD).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_SWISH_FWD => {
                writer.write_all(stringify!(CUDNN_POINTWISE_SWISH_FWD).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_GELU_APPROX_TANH_FWD => {
                writer
                    .write_all(
                        stringify!(CUDNN_POINTWISE_GELU_APPROX_TANH_FWD).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_RELU_BWD => {
                writer.write_all(stringify!(CUDNN_POINTWISE_RELU_BWD).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_TANH_BWD => {
                writer.write_all(stringify!(CUDNN_POINTWISE_TANH_BWD).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_SIGMOID_BWD => {
                writer.write_all(stringify!(CUDNN_POINTWISE_SIGMOID_BWD).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_ELU_BWD => {
                writer.write_all(stringify!(CUDNN_POINTWISE_ELU_BWD).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_GELU_BWD => {
                writer.write_all(stringify!(CUDNN_POINTWISE_GELU_BWD).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_SOFTPLUS_BWD => {
                writer.write_all(stringify!(CUDNN_POINTWISE_SOFTPLUS_BWD).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_SWISH_BWD => {
                writer.write_all(stringify!(CUDNN_POINTWISE_SWISH_BWD).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_GELU_APPROX_TANH_BWD => {
                writer
                    .write_all(
                        stringify!(CUDNN_POINTWISE_GELU_APPROX_TANH_BWD).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_CMP_EQ => {
                writer.write_all(stringify!(CUDNN_POINTWISE_CMP_EQ).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_CMP_NEQ => {
                writer.write_all(stringify!(CUDNN_POINTWISE_CMP_NEQ).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_CMP_GT => {
                writer.write_all(stringify!(CUDNN_POINTWISE_CMP_GT).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_CMP_GE => {
                writer.write_all(stringify!(CUDNN_POINTWISE_CMP_GE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_CMP_LT => {
                writer.write_all(stringify!(CUDNN_POINTWISE_CMP_LT).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_CMP_LE => {
                writer.write_all(stringify!(CUDNN_POINTWISE_CMP_LE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_LOGICAL_AND => {
                writer.write_all(stringify!(CUDNN_POINTWISE_LOGICAL_AND).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_LOGICAL_OR => {
                writer.write_all(stringify!(CUDNN_POINTWISE_LOGICAL_OR).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_LOGICAL_NOT => {
                writer.write_all(stringify!(CUDNN_POINTWISE_LOGICAL_NOT).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_GEN_INDEX => {
                writer.write_all(stringify!(CUDNN_POINTWISE_GEN_INDEX).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPointwiseMode_t::CUDNN_POINTWISE_BINARY_SELECT => {
                writer.write_all(stringify!(CUDNN_POINTWISE_BINARY_SELECT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnResampleMode_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnResampleMode_t::CUDNN_RESAMPLE_NEAREST => {
                writer.write_all(stringify!(CUDNN_RESAMPLE_NEAREST).as_bytes())
            }
            &cuda_types::cudnn9::cudnnResampleMode_t::CUDNN_RESAMPLE_BILINEAR => {
                writer.write_all(stringify!(CUDNN_RESAMPLE_BILINEAR).as_bytes())
            }
            &cuda_types::cudnn9::cudnnResampleMode_t::CUDNN_RESAMPLE_AVGPOOL => {
                writer.write_all(stringify!(CUDNN_RESAMPLE_AVGPOOL).as_bytes())
            }
            &cuda_types::cudnn9::cudnnResampleMode_t::CUDNN_RESAMPLE_AVGPOOL_INCLUDE_PADDING => {
                writer
                    .write_all(
                        stringify!(CUDNN_RESAMPLE_AVGPOOL_INCLUDE_PADDING).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnResampleMode_t::CUDNN_RESAMPLE_AVGPOOL_EXCLUDE_PADDING => {
                writer
                    .write_all(
                        stringify!(CUDNN_RESAMPLE_AVGPOOL_EXCLUDE_PADDING).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnResampleMode_t::CUDNN_RESAMPLE_MAXPOOL => {
                writer.write_all(stringify!(CUDNN_RESAMPLE_MAXPOOL).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnSignalMode_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnSignalMode_t::CUDNN_SIGNAL_SET => {
                writer.write_all(stringify!(CUDNN_SIGNAL_SET).as_bytes())
            }
            &cuda_types::cudnn9::cudnnSignalMode_t::CUDNN_SIGNAL_WAIT => {
                writer.write_all(stringify!(CUDNN_SIGNAL_WAIT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnGenStatsMode_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnGenStatsMode_t::CUDNN_GENSTATS_SUM_SQSUM => {
                writer.write_all(stringify!(CUDNN_GENSTATS_SUM_SQSUM).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnBnFinalizeStatsMode_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnBnFinalizeStatsMode_t::CUDNN_BN_FINALIZE_STATISTICS_TRAINING => {
                writer
                    .write_all(
                        stringify!(CUDNN_BN_FINALIZE_STATISTICS_TRAINING).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBnFinalizeStatsMode_t::CUDNN_BN_FINALIZE_STATISTICS_INFERENCE => {
                writer
                    .write_all(
                        stringify!(CUDNN_BN_FINALIZE_STATISTICS_INFERENCE).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnRngDistribution_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnRngDistribution_t::CUDNN_RNG_DISTRIBUTION_BERNOULLI => {
                writer.write_all(stringify!(CUDNN_RNG_DISTRIBUTION_BERNOULLI).as_bytes())
            }
            &cuda_types::cudnn9::cudnnRngDistribution_t::CUDNN_RNG_DISTRIBUTION_UNIFORM => {
                writer.write_all(stringify!(CUDNN_RNG_DISTRIBUTION_UNIFORM).as_bytes())
            }
            &cuda_types::cudnn9::cudnnRngDistribution_t::CUDNN_RNG_DISTRIBUTION_NORMAL => {
                writer.write_all(stringify!(CUDNN_RNG_DISTRIBUTION_NORMAL).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnBackendAttributeName_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_POINTWISE_MODE => {
                writer.write_all(stringify!(CUDNN_ATTR_POINTWISE_MODE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_POINTWISE_MATH_PREC => {
                writer.write_all(stringify!(CUDNN_ATTR_POINTWISE_MATH_PREC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_POINTWISE_NAN_PROPAGATION => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_POINTWISE_NAN_PROPAGATION).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_POINTWISE_RELU_LOWER_CLIP => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_POINTWISE_RELU_LOWER_CLIP).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_POINTWISE_RELU_UPPER_CLIP => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_POINTWISE_RELU_UPPER_CLIP).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_POINTWISE_RELU_LOWER_CLIP_SLOPE => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_POINTWISE_RELU_LOWER_CLIP_SLOPE).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_POINTWISE_ELU_ALPHA => {
                writer.write_all(stringify!(CUDNN_ATTR_POINTWISE_ELU_ALPHA).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_POINTWISE_SOFTPLUS_BETA => {
                writer
                    .write_all(stringify!(CUDNN_ATTR_POINTWISE_SOFTPLUS_BETA).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_POINTWISE_SWISH_BETA => {
                writer.write_all(stringify!(CUDNN_ATTR_POINTWISE_SWISH_BETA).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_POINTWISE_AXIS => {
                writer.write_all(stringify!(CUDNN_ATTR_POINTWISE_AXIS).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_CONVOLUTION_COMP_TYPE => {
                writer.write_all(stringify!(CUDNN_ATTR_CONVOLUTION_COMP_TYPE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_CONVOLUTION_CONV_MODE => {
                writer.write_all(stringify!(CUDNN_ATTR_CONVOLUTION_CONV_MODE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_CONVOLUTION_DILATIONS => {
                writer.write_all(stringify!(CUDNN_ATTR_CONVOLUTION_DILATIONS).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_CONVOLUTION_FILTER_STRIDES => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_CONVOLUTION_FILTER_STRIDES).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_CONVOLUTION_POST_PADDINGS => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_CONVOLUTION_POST_PADDINGS).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_CONVOLUTION_PRE_PADDINGS => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_CONVOLUTION_PRE_PADDINGS).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_CONVOLUTION_SPATIAL_DIMS => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_CONVOLUTION_SPATIAL_DIMS).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_ENGINEHEUR_MODE => {
                writer.write_all(stringify!(CUDNN_ATTR_ENGINEHEUR_MODE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_ENGINEHEUR_OPERATION_GRAPH => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_ENGINEHEUR_OPERATION_GRAPH).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_ENGINEHEUR_RESULTS => {
                writer.write_all(stringify!(CUDNN_ATTR_ENGINEHEUR_RESULTS).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_ENGINEHEUR_SM_COUNT_TARGET => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_ENGINEHEUR_SM_COUNT_TARGET).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_ENGINEHEUR_DEVICEPROP => {
                writer.write_all(stringify!(CUDNN_ATTR_ENGINEHEUR_DEVICEPROP).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_ENGINECFG_ENGINE => {
                writer.write_all(stringify!(CUDNN_ATTR_ENGINECFG_ENGINE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_ENGINECFG_INTERMEDIATE_INFO => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_ENGINECFG_INTERMEDIATE_INFO).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_ENGINECFG_KNOB_CHOICES => {
                writer
                    .write_all(stringify!(CUDNN_ATTR_ENGINECFG_KNOB_CHOICES).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_ENGINECFG_WORKSPACE_SIZE => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_ENGINECFG_WORKSPACE_SIZE).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_ENGINECFG_SHARED_MEMORY_USED => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_ENGINECFG_SHARED_MEMORY_USED).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_EXECUTION_PLAN_HANDLE => {
                writer.write_all(stringify!(CUDNN_ATTR_EXECUTION_PLAN_HANDLE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_EXECUTION_PLAN_ENGINE_CONFIG => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_EXECUTION_PLAN_ENGINE_CONFIG).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_EXECUTION_PLAN_WORKSPACE_SIZE => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_EXECUTION_PLAN_WORKSPACE_SIZE).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_EXECUTION_PLAN_COMPUTED_INTERMEDIATE_UIDS => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_EXECUTION_PLAN_COMPUTED_INTERMEDIATE_UIDS)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_EXECUTION_PLAN_RUN_ONLY_INTERMEDIATE_UIDS => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_EXECUTION_PLAN_RUN_ONLY_INTERMEDIATE_UIDS)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_EXECUTION_PLAN_JSON_REPRESENTATION => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_EXECUTION_PLAN_JSON_REPRESENTATION)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_EXECUTION_PLAN_KERNEL_CACHE => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_EXECUTION_PLAN_KERNEL_CACHE).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_EXECUTION_PLAN_DEVICEPROP => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_EXECUTION_PLAN_DEVICEPROP).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_INTERMEDIATE_INFO_UNIQUE_ID => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_INTERMEDIATE_INFO_UNIQUE_ID).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_INTERMEDIATE_INFO_SIZE => {
                writer
                    .write_all(stringify!(CUDNN_ATTR_INTERMEDIATE_INFO_SIZE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_INTERMEDIATE_INFO_DEPENDENT_DATA_UIDS => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_INTERMEDIATE_INFO_DEPENDENT_DATA_UIDS)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_INTERMEDIATE_INFO_DEPENDENT_ATTRIBUTES => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_INTERMEDIATE_INFO_DEPENDENT_ATTRIBUTES)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_KNOB_CHOICE_KNOB_TYPE => {
                writer.write_all(stringify!(CUDNN_ATTR_KNOB_CHOICE_KNOB_TYPE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_KNOB_CHOICE_KNOB_VALUE => {
                writer
                    .write_all(stringify!(CUDNN_ATTR_KNOB_CHOICE_KNOB_VALUE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_ALPHA => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_ALPHA)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_BETA => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_BETA)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_CONV_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_CONV_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_W => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_W).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_X => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_X).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_Y => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_CONVOLUTION_FORWARD_Y).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_ALPHA => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_ALPHA)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_BETA => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_BETA)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_CONV_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_CONV_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_W => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_W)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_DX => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_DX)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_DY => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_DATA_DY)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_ALPHA => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_ALPHA)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_BETA => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_BETA)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_CONV_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_CONV_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_DW => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_DW)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_X => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_X)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_DY => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_CONVOLUTION_BWD_FILTER_DY)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_POINTWISE_PW_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_POINTWISE_PW_DESCRIPTOR)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_POINTWISE_XDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_POINTWISE_XDESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_POINTWISE_BDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_POINTWISE_BDESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_POINTWISE_YDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_POINTWISE_YDESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_POINTWISE_ALPHA1 => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_POINTWISE_ALPHA1).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_POINTWISE_ALPHA2 => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_POINTWISE_ALPHA2).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_POINTWISE_DXDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_POINTWISE_DXDESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_POINTWISE_DYDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_POINTWISE_DYDESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_POINTWISE_TDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_POINTWISE_TDESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_GENSTATS_MODE => {
                writer
                    .write_all(stringify!(CUDNN_ATTR_OPERATION_GENSTATS_MODE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_GENSTATS_MATH_PREC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_GENSTATS_MATH_PREC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_GENSTATS_XDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_GENSTATS_XDESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_GENSTATS_SUMDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_GENSTATS_SUMDESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_GENSTATS_SQSUMDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_GENSTATS_SQSUMDESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BN_FINALIZE_STATS_MODE => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BN_FINALIZE_STATS_MODE)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BN_FINALIZE_MATH_PREC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BN_FINALIZE_MATH_PREC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BN_FINALIZE_Y_SUM_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BN_FINALIZE_Y_SUM_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BN_FINALIZE_Y_SQ_SUM_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BN_FINALIZE_Y_SQ_SUM_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BN_FINALIZE_SCALE_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BN_FINALIZE_SCALE_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BN_FINALIZE_BIAS_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BN_FINALIZE_BIAS_DESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BN_FINALIZE_PREV_RUNNING_MEAN_DESC => {
                writer
                    .write_all(
                        stringify!(
                            CUDNN_ATTR_OPERATION_BN_FINALIZE_PREV_RUNNING_MEAN_DESC
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BN_FINALIZE_PREV_RUNNING_VAR_DESC => {
                writer
                    .write_all(
                        stringify!(
                            CUDNN_ATTR_OPERATION_BN_FINALIZE_PREV_RUNNING_VAR_DESC
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BN_FINALIZE_UPDATED_RUNNING_MEAN_DESC => {
                writer
                    .write_all(
                        stringify!(
                            CUDNN_ATTR_OPERATION_BN_FINALIZE_UPDATED_RUNNING_MEAN_DESC
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BN_FINALIZE_UPDATED_RUNNING_VAR_DESC => {
                writer
                    .write_all(
                        stringify!(
                            CUDNN_ATTR_OPERATION_BN_FINALIZE_UPDATED_RUNNING_VAR_DESC
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BN_FINALIZE_SAVED_MEAN_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BN_FINALIZE_SAVED_MEAN_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BN_FINALIZE_SAVED_INV_STD_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BN_FINALIZE_SAVED_INV_STD_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BN_FINALIZE_EQ_SCALE_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BN_FINALIZE_EQ_SCALE_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BN_FINALIZE_EQ_BIAS_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BN_FINALIZE_EQ_BIAS_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BN_FINALIZE_ACCUM_COUNT_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BN_FINALIZE_ACCUM_COUNT_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BN_FINALIZE_EPSILON_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BN_FINALIZE_EPSILON_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BN_FINALIZE_EXP_AVERATE_FACTOR_DESC => {
                writer
                    .write_all(
                        stringify!(
                            CUDNN_ATTR_OPERATION_BN_FINALIZE_EXP_AVERATE_FACTOR_DESC
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATIONGRAPH_HANDLE => {
                writer.write_all(stringify!(CUDNN_ATTR_OPERATIONGRAPH_HANDLE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATIONGRAPH_OPS => {
                writer.write_all(stringify!(CUDNN_ATTR_OPERATIONGRAPH_OPS).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATIONGRAPH_ENGINE_GLOBAL_COUNT => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATIONGRAPH_ENGINE_GLOBAL_COUNT)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATIONGRAPH_IS_DYNAMIC_SHAPE_ENABLED => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATIONGRAPH_IS_DYNAMIC_SHAPE_ENABLED)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATIONGRAPH_IS_SAME_TOPOLOGY => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATIONGRAPH_IS_SAME_TOPOLOGY).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_TENSOR_BYTE_ALIGNMENT => {
                writer.write_all(stringify!(CUDNN_ATTR_TENSOR_BYTE_ALIGNMENT).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_TENSOR_DATA_TYPE => {
                writer.write_all(stringify!(CUDNN_ATTR_TENSOR_DATA_TYPE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_TENSOR_DIMENSIONS => {
                writer.write_all(stringify!(CUDNN_ATTR_TENSOR_DIMENSIONS).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_TENSOR_STRIDES => {
                writer.write_all(stringify!(CUDNN_ATTR_TENSOR_STRIDES).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_TENSOR_VECTOR_COUNT => {
                writer.write_all(stringify!(CUDNN_ATTR_TENSOR_VECTOR_COUNT).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_TENSOR_VECTORIZED_DIMENSION => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_TENSOR_VECTORIZED_DIMENSION).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_TENSOR_UNIQUE_ID => {
                writer.write_all(stringify!(CUDNN_ATTR_TENSOR_UNIQUE_ID).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_TENSOR_IS_VIRTUAL => {
                writer.write_all(stringify!(CUDNN_ATTR_TENSOR_IS_VIRTUAL).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_TENSOR_IS_BY_VALUE => {
                writer.write_all(stringify!(CUDNN_ATTR_TENSOR_IS_BY_VALUE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_TENSOR_REORDERING_MODE => {
                writer
                    .write_all(stringify!(CUDNN_ATTR_TENSOR_REORDERING_MODE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_TENSOR_RAGGED_OFFSET_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_TENSOR_RAGGED_OFFSET_DESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_VARIANT_PACK_UNIQUE_IDS => {
                writer
                    .write_all(stringify!(CUDNN_ATTR_VARIANT_PACK_UNIQUE_IDS).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_VARIANT_PACK_DATA_POINTERS => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_VARIANT_PACK_DATA_POINTERS).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_VARIANT_PACK_INTERMEDIATES => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_VARIANT_PACK_INTERMEDIATES).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_VARIANT_PACK_WORKSPACE => {
                writer
                    .write_all(stringify!(CUDNN_ATTR_VARIANT_PACK_WORKSPACE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_LAYOUT_INFO_TENSOR_UID => {
                writer
                    .write_all(stringify!(CUDNN_ATTR_LAYOUT_INFO_TENSOR_UID).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_LAYOUT_INFO_TYPES => {
                writer.write_all(stringify!(CUDNN_ATTR_LAYOUT_INFO_TYPES).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_KNOB_INFO_TYPE => {
                writer.write_all(stringify!(CUDNN_ATTR_KNOB_INFO_TYPE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_KNOB_INFO_MAXIMUM_VALUE => {
                writer
                    .write_all(stringify!(CUDNN_ATTR_KNOB_INFO_MAXIMUM_VALUE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_KNOB_INFO_MINIMUM_VALUE => {
                writer
                    .write_all(stringify!(CUDNN_ATTR_KNOB_INFO_MINIMUM_VALUE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_KNOB_INFO_STRIDE => {
                writer.write_all(stringify!(CUDNN_ATTR_KNOB_INFO_STRIDE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_ENGINE_OPERATION_GRAPH => {
                writer
                    .write_all(stringify!(CUDNN_ATTR_ENGINE_OPERATION_GRAPH).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_ENGINE_GLOBAL_INDEX => {
                writer.write_all(stringify!(CUDNN_ATTR_ENGINE_GLOBAL_INDEX).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_ENGINE_KNOB_INFO => {
                writer.write_all(stringify!(CUDNN_ATTR_ENGINE_KNOB_INFO).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_ENGINE_NUMERICAL_NOTE => {
                writer.write_all(stringify!(CUDNN_ATTR_ENGINE_NUMERICAL_NOTE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_ENGINE_LAYOUT_INFO => {
                writer.write_all(stringify!(CUDNN_ATTR_ENGINE_LAYOUT_INFO).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_ENGINE_BEHAVIOR_NOTE => {
                writer.write_all(stringify!(CUDNN_ATTR_ENGINE_BEHAVIOR_NOTE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_ENGINE_SM_COUNT_TARGET => {
                writer
                    .write_all(stringify!(CUDNN_ATTR_ENGINE_SM_COUNT_TARGET).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_ENGINE_DEVICEPROP => {
                writer.write_all(stringify!(CUDNN_ATTR_ENGINE_DEVICEPROP).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_MATMUL_COMP_TYPE => {
                writer.write_all(stringify!(CUDNN_ATTR_MATMUL_COMP_TYPE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_MATMUL_PADDING_VALUE => {
                writer.write_all(stringify!(CUDNN_ATTR_MATMUL_PADDING_VALUE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_MATMUL_ADESC => {
                writer
                    .write_all(stringify!(CUDNN_ATTR_OPERATION_MATMUL_ADESC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_MATMUL_BDESC => {
                writer
                    .write_all(stringify!(CUDNN_ATTR_OPERATION_MATMUL_BDESC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_MATMUL_CDESC => {
                writer
                    .write_all(stringify!(CUDNN_ATTR_OPERATION_MATMUL_CDESC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_MATMUL_DESC => {
                writer.write_all(stringify!(CUDNN_ATTR_OPERATION_MATMUL_DESC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_MATMUL_IRREGULARLY_STRIDED_BATCH_COUNT => {
                writer
                    .write_all(
                        stringify!(
                            CUDNN_ATTR_OPERATION_MATMUL_IRREGULARLY_STRIDED_BATCH_COUNT
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_MATMUL_GEMM_M_OVERRIDE_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_MATMUL_GEMM_M_OVERRIDE_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_MATMUL_GEMM_N_OVERRIDE_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_MATMUL_GEMM_N_OVERRIDE_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_MATMUL_GEMM_K_OVERRIDE_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_MATMUL_GEMM_K_OVERRIDE_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_REDUCTION_OPERATOR => {
                writer.write_all(stringify!(CUDNN_ATTR_REDUCTION_OPERATOR).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_REDUCTION_COMP_TYPE => {
                writer.write_all(stringify!(CUDNN_ATTR_REDUCTION_COMP_TYPE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_REDUCTION_IS_DETERMINISTIC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_REDUCTION_IS_DETERMINISTIC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_REDUCTION_XDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_REDUCTION_XDESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_REDUCTION_YDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_REDUCTION_YDESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_REDUCTION_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_REDUCTION_DESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_MATH_PREC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_MATH_PREC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_MEAN_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_MEAN_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_INVSTD_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_INVSTD_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_BN_SCALE_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_BN_SCALE_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_X_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_X_DESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_DY_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_DY_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_DBN_SCALE_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_DBN_SCALE_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_DBN_BIAS_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_DBN_BIAS_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_EQ_DY_SCALE_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_EQ_DY_SCALE_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_EQ_X_SCALE_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_EQ_X_SCALE_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_EQ_BIAS => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BN_BWD_WEIGHTS_EQ_BIAS)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_RESAMPLE_MODE => {
                writer.write_all(stringify!(CUDNN_ATTR_RESAMPLE_MODE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_RESAMPLE_COMP_TYPE => {
                writer.write_all(stringify!(CUDNN_ATTR_RESAMPLE_COMP_TYPE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_RESAMPLE_SPATIAL_DIMS => {
                writer.write_all(stringify!(CUDNN_ATTR_RESAMPLE_SPATIAL_DIMS).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_RESAMPLE_POST_PADDINGS => {
                writer
                    .write_all(stringify!(CUDNN_ATTR_RESAMPLE_POST_PADDINGS).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_RESAMPLE_PRE_PADDINGS => {
                writer.write_all(stringify!(CUDNN_ATTR_RESAMPLE_PRE_PADDINGS).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_RESAMPLE_STRIDES => {
                writer.write_all(stringify!(CUDNN_ATTR_RESAMPLE_STRIDES).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_RESAMPLE_WINDOW_DIMS => {
                writer.write_all(stringify!(CUDNN_ATTR_RESAMPLE_WINDOW_DIMS).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_RESAMPLE_NAN_PROPAGATION => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_RESAMPLE_NAN_PROPAGATION).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_RESAMPLE_PADDING_MODE => {
                writer.write_all(stringify!(CUDNN_ATTR_RESAMPLE_PADDING_MODE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_RESAMPLE_FWD_XDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_RESAMPLE_FWD_XDESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_RESAMPLE_FWD_YDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_RESAMPLE_FWD_YDESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_RESAMPLE_FWD_IDXDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_RESAMPLE_FWD_IDXDESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_RESAMPLE_FWD_ALPHA => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_RESAMPLE_FWD_ALPHA).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_RESAMPLE_FWD_BETA => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_RESAMPLE_FWD_BETA).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_RESAMPLE_FWD_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_RESAMPLE_FWD_DESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_RESAMPLE_BWD_DXDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_RESAMPLE_BWD_DXDESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_RESAMPLE_BWD_DYDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_RESAMPLE_BWD_DYDESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_RESAMPLE_BWD_IDXDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_RESAMPLE_BWD_IDXDESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_RESAMPLE_BWD_ALPHA => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_RESAMPLE_BWD_ALPHA).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_RESAMPLE_BWD_BETA => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_RESAMPLE_BWD_BETA).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_RESAMPLE_BWD_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_RESAMPLE_BWD_DESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_RESAMPLE_BWD_XDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_RESAMPLE_BWD_XDESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_RESAMPLE_BWD_YDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_RESAMPLE_BWD_YDESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_CONCAT_AXIS => {
                writer.write_all(stringify!(CUDNN_ATTR_OPERATION_CONCAT_AXIS).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_CONCAT_INPUT_DESCS => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_CONCAT_INPUT_DESCS).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_CONCAT_INPLACE_INDEX => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_CONCAT_INPLACE_INDEX).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_CONCAT_OUTPUT_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_CONCAT_OUTPUT_DESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_SIGNAL_MODE => {
                writer.write_all(stringify!(CUDNN_ATTR_OPERATION_SIGNAL_MODE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_SIGNAL_FLAGDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_SIGNAL_FLAGDESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_SIGNAL_VALUE => {
                writer
                    .write_all(stringify!(CUDNN_ATTR_OPERATION_SIGNAL_VALUE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_SIGNAL_XDESC => {
                writer
                    .write_all(stringify!(CUDNN_ATTR_OPERATION_SIGNAL_XDESC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_SIGNAL_YDESC => {
                writer
                    .write_all(stringify!(CUDNN_ATTR_OPERATION_SIGNAL_YDESC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_PAGED_CACHE_LOAD_CONTAINER_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_PAGED_CACHE_LOAD_CONTAINER_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_PAGED_CACHE_LOAD_YDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_PAGED_CACHE_LOAD_YDESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_PAGED_CACHE_LOAD_SEQUENCE_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_PAGED_CACHE_LOAD_SEQUENCE_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_PAGED_CACHE_LOAD_PAGE_TABLE_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_PAGED_CACHE_LOAD_PAGE_TABLE_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_NORM_FWD_MODE => {
                writer
                    .write_all(stringify!(CUDNN_ATTR_OPERATION_NORM_FWD_MODE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_NORM_FWD_PHASE => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_NORM_FWD_PHASE).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_NORM_FWD_XDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_NORM_FWD_XDESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_NORM_FWD_MEAN_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_NORM_FWD_MEAN_DESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_NORM_FWD_INV_VARIANCE_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_NORM_FWD_INV_VARIANCE_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_NORM_FWD_SCALE_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_NORM_FWD_SCALE_DESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_NORM_FWD_BIAS_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_NORM_FWD_BIAS_DESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_NORM_FWD_EPSILON_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_NORM_FWD_EPSILON_DESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_NORM_FWD_EXP_AVG_FACTOR_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_NORM_FWD_EXP_AVG_FACTOR_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_NORM_FWD_INPUT_RUNNING_MEAN_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_NORM_FWD_INPUT_RUNNING_MEAN_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_NORM_FWD_INPUT_RUNNING_VAR_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_NORM_FWD_INPUT_RUNNING_VAR_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_NORM_FWD_OUTPUT_RUNNING_MEAN_DESC => {
                writer
                    .write_all(
                        stringify!(
                            CUDNN_ATTR_OPERATION_NORM_FWD_OUTPUT_RUNNING_MEAN_DESC
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_NORM_FWD_OUTPUT_RUNNING_VAR_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_NORM_FWD_OUTPUT_RUNNING_VAR_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_NORM_FWD_YDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_NORM_FWD_YDESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_NORM_FWD_PEER_STAT_DESCS => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_NORM_FWD_PEER_STAT_DESCS)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_NORM_BWD_MODE => {
                writer
                    .write_all(stringify!(CUDNN_ATTR_OPERATION_NORM_BWD_MODE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_NORM_BWD_XDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_NORM_BWD_XDESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_NORM_BWD_MEAN_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_NORM_BWD_MEAN_DESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_NORM_BWD_INV_VARIANCE_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_NORM_BWD_INV_VARIANCE_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_NORM_BWD_DYDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_NORM_BWD_DYDESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_NORM_BWD_SCALE_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_NORM_BWD_SCALE_DESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_NORM_BWD_EPSILON_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_NORM_BWD_EPSILON_DESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_NORM_BWD_DSCALE_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_NORM_BWD_DSCALE_DESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_NORM_BWD_DBIAS_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_NORM_BWD_DBIAS_DESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_NORM_BWD_DXDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_NORM_BWD_DXDESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_NORM_BWD_PEER_STAT_DESCS => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_NORM_BWD_PEER_STAT_DESCS)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_RESHAPE_XDESC => {
                writer
                    .write_all(stringify!(CUDNN_ATTR_OPERATION_RESHAPE_XDESC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_RESHAPE_YDESC => {
                writer
                    .write_all(stringify!(CUDNN_ATTR_OPERATION_RESHAPE_YDESC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_XDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_XDESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_YDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_YDESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_LOWER_BANDWIDTH => {
                writer
                    .write_all(
                        stringify!(
                            CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_LOWER_BANDWIDTH
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_UPPER_BANDWIDTH => {
                writer
                    .write_all(
                        stringify!(
                            CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_UPPER_BANDWIDTH
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_AXIS => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_AXIS)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_PAD_VALUE => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_PAD_VALUE)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_KV_TOKEN_OFFSET_DESC => {
                writer
                    .write_all(
                        stringify!(
                            CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_KV_TOKEN_OFFSET_DESC
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_SPECULATIVE_MASK_DESC => {
                writer
                    .write_all(
                        stringify!(
                            CUDNN_ATTR_OPERATION_EXPAND_BAND_MATRIX_SPECULATIVE_MASK_DESC
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_CONTRACT_BAND_MATRIX_XDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_CONTRACT_BAND_MATRIX_XDESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_CONTRACT_BAND_MATRIX_YDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_CONTRACT_BAND_MATRIX_YDESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_CONTRACT_BAND_MATRIX_LOWER_BANDWIDTH => {
                writer
                    .write_all(
                        stringify!(
                            CUDNN_ATTR_OPERATION_CONTRACT_BAND_MATRIX_LOWER_BANDWIDTH
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_CONTRACT_BAND_MATRIX_UPPER_BANDWIDTH => {
                writer
                    .write_all(
                        stringify!(
                            CUDNN_ATTR_OPERATION_CONTRACT_BAND_MATRIX_UPPER_BANDWIDTH
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_CONTRACT_BAND_MATRIX_AXIS => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_CONTRACT_BAND_MATRIX_AXIS)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_CONTRACT_BAND_MATRIX_PAD_VALUE => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_CONTRACT_BAND_MATRIX_PAD_VALUE)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_CONTRACT_BAND_MAX_TOKEN_VALUE => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_CONTRACT_BAND_MAX_TOKEN_VALUE)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_RNG_DISTRIBUTION => {
                writer.write_all(stringify!(CUDNN_ATTR_RNG_DISTRIBUTION).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_RNG_NORMAL_DIST_MEAN => {
                writer.write_all(stringify!(CUDNN_ATTR_RNG_NORMAL_DIST_MEAN).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_RNG_NORMAL_DIST_STANDARD_DEVIATION => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_RNG_NORMAL_DIST_STANDARD_DEVIATION)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_RNG_UNIFORM_DIST_MAXIMUM => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_RNG_UNIFORM_DIST_MAXIMUM).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_RNG_UNIFORM_DIST_MINIMUM => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_RNG_UNIFORM_DIST_MINIMUM).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_RNG_BERNOULLI_DIST_PROBABILITY => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_RNG_BERNOULLI_DIST_PROBABILITY).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_RNG_YDESC => {
                writer.write_all(stringify!(CUDNN_ATTR_OPERATION_RNG_YDESC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_RNG_SEED => {
                writer.write_all(stringify!(CUDNN_ATTR_OPERATION_RNG_SEED).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_RNG_DESC => {
                writer.write_all(stringify!(CUDNN_ATTR_OPERATION_RNG_DESC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_RNG_OFFSET_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_RNG_OFFSET_DESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_KERNEL_CACHE_OPERATION_GRAPH => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_KERNEL_CACHE_OPERATION_GRAPH).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_KERNEL_CACHE_IS_ENGINECFG_KERNEL_CACHED => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_KERNEL_CACHE_IS_ENGINECFG_KERNEL_CACHED)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_KERNEL_CACHE_JSON_REPRESENTATION => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_KERNEL_CACHE_JSON_REPRESENTATION)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BLOCK_SCALE_QUANTIZE_XDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BLOCK_SCALE_QUANTIZE_XDESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BLOCK_SCALE_QUANTIZE_YDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BLOCK_SCALE_QUANTIZE_YDESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BLOCK_SCALE_QUANTIZE_SCALE_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BLOCK_SCALE_QUANTIZE_SCALE_DESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BLOCK_SCALE_QUANTIZE_MATH_PREC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BLOCK_SCALE_QUANTIZE_MATH_PREC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BLOCK_SCALE_QUANTIZE_BLOCK_SIZE => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BLOCK_SCALE_QUANTIZE_BLOCK_SIZE)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BLOCK_SCALE_DEQUANTIZE_XDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BLOCK_SCALE_DEQUANTIZE_XDESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BLOCK_SCALE_DEQUANTIZE_SCALE_DESC => {
                writer
                    .write_all(
                        stringify!(
                            CUDNN_ATTR_OPERATION_BLOCK_SCALE_DEQUANTIZE_SCALE_DESC
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BLOCK_SCALE_DEQUANTIZE_YDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BLOCK_SCALE_DEQUANTIZE_YDESC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BLOCK_SCALE_DEQUANTIZE_MATH_PREC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BLOCK_SCALE_DEQUANTIZE_MATH_PREC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BLOCK_SCALE_DEQUANTIZE_BLOCK_SIZE => {
                writer
                    .write_all(
                        stringify!(
                            CUDNN_ATTR_OPERATION_BLOCK_SCALE_DEQUANTIZE_BLOCK_SIZE
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_BLOCK_SCALE_DEQUANTIZE_NEG_SCALE => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_BLOCK_SCALE_DEQUANTIZE_NEG_SCALE)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_DEVICEPROP_DEVICE_ID => {
                writer.write_all(stringify!(CUDNN_ATTR_DEVICEPROP_DEVICE_ID).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_DEVICEPROP_HANDLE => {
                writer.write_all(stringify!(CUDNN_ATTR_DEVICEPROP_HANDLE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_DEVICEPROP_JSON_REPRESENTATION => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_DEVICEPROP_JSON_REPRESENTATION).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_SDPA_FWD_QDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_SDPA_FWD_QDESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_SDPA_FWD_KDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_SDPA_FWD_KDESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_SDPA_FWD_VDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_SDPA_FWD_VDESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_SDPA_FWD_ODESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_SDPA_FWD_ODESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_SDPA_FWD_STATSDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_SDPA_FWD_STATSDESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendAttributeName_t::CUDNN_ATTR_OPERATION_SDPA_FWD_SCALEDESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_ATTR_OPERATION_SDPA_FWD_SCALEDESC).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnBackendAttributeType_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_HANDLE => {
                writer.write_all(stringify!(CUDNN_TYPE_HANDLE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_DATA_TYPE => {
                writer.write_all(stringify!(CUDNN_TYPE_DATA_TYPE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_BOOLEAN => {
                writer.write_all(stringify!(CUDNN_TYPE_BOOLEAN).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_INT64 => {
                writer.write_all(stringify!(CUDNN_TYPE_INT64).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_FLOAT => {
                writer.write_all(stringify!(CUDNN_TYPE_FLOAT).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_DOUBLE => {
                writer.write_all(stringify!(CUDNN_TYPE_DOUBLE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_VOID_PTR => {
                writer.write_all(stringify!(CUDNN_TYPE_VOID_PTR).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_CONVOLUTION_MODE => {
                writer.write_all(stringify!(CUDNN_TYPE_CONVOLUTION_MODE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_HEUR_MODE => {
                writer.write_all(stringify!(CUDNN_TYPE_HEUR_MODE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_KNOB_TYPE => {
                writer.write_all(stringify!(CUDNN_TYPE_KNOB_TYPE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_NAN_PROPOGATION => {
                writer.write_all(stringify!(CUDNN_TYPE_NAN_PROPOGATION).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_NUMERICAL_NOTE => {
                writer.write_all(stringify!(CUDNN_TYPE_NUMERICAL_NOTE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_LAYOUT_TYPE => {
                writer.write_all(stringify!(CUDNN_TYPE_LAYOUT_TYPE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_ATTRIB_NAME => {
                writer.write_all(stringify!(CUDNN_TYPE_ATTRIB_NAME).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_POINTWISE_MODE => {
                writer.write_all(stringify!(CUDNN_TYPE_POINTWISE_MODE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_BACKEND_DESCRIPTOR => {
                writer.write_all(stringify!(CUDNN_TYPE_BACKEND_DESCRIPTOR).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_GENSTATS_MODE => {
                writer.write_all(stringify!(CUDNN_TYPE_GENSTATS_MODE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_BN_FINALIZE_STATS_MODE => {
                writer
                    .write_all(stringify!(CUDNN_TYPE_BN_FINALIZE_STATS_MODE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_REDUCTION_OPERATOR_TYPE => {
                writer
                    .write_all(stringify!(CUDNN_TYPE_REDUCTION_OPERATOR_TYPE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_BEHAVIOR_NOTE => {
                writer.write_all(stringify!(CUDNN_TYPE_BEHAVIOR_NOTE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_TENSOR_REORDERING_MODE => {
                writer
                    .write_all(stringify!(CUDNN_TYPE_TENSOR_REORDERING_MODE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_RESAMPLE_MODE => {
                writer.write_all(stringify!(CUDNN_TYPE_RESAMPLE_MODE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_PADDING_MODE => {
                writer.write_all(stringify!(CUDNN_TYPE_PADDING_MODE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_INT32 => {
                writer.write_all(stringify!(CUDNN_TYPE_INT32).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_CHAR => {
                writer.write_all(stringify!(CUDNN_TYPE_CHAR).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_SIGNAL_MODE => {
                writer.write_all(stringify!(CUDNN_TYPE_SIGNAL_MODE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_FRACTION => {
                writer.write_all(stringify!(CUDNN_TYPE_FRACTION).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_NORM_MODE => {
                writer.write_all(stringify!(CUDNN_TYPE_NORM_MODE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_NORM_FWD_PHASE => {
                writer.write_all(stringify!(CUDNN_TYPE_NORM_FWD_PHASE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_RNG_DISTRIBUTION => {
                writer.write_all(stringify!(CUDNN_TYPE_RNG_DISTRIBUTION).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnBackendDescriptorType_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_POINTWISE_DESCRIPTOR => {
                writer
                    .write_all(stringify!(CUDNN_BACKEND_POINTWISE_DESCRIPTOR).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_CONVOLUTION_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(CUDNN_BACKEND_CONVOLUTION_DESCRIPTOR).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_ENGINE_DESCRIPTOR => {
                writer.write_all(stringify!(CUDNN_BACKEND_ENGINE_DESCRIPTOR).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_ENGINECFG_DESCRIPTOR => {
                writer
                    .write_all(stringify!(CUDNN_BACKEND_ENGINECFG_DESCRIPTOR).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_ENGINEHEUR_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(CUDNN_BACKEND_ENGINEHEUR_DESCRIPTOR).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_EXECUTION_PLAN_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(CUDNN_BACKEND_EXECUTION_PLAN_DESCRIPTOR).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_INTERMEDIATE_INFO_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(CUDNN_BACKEND_INTERMEDIATE_INFO_DESCRIPTOR).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_KNOB_CHOICE_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(CUDNN_BACKEND_KNOB_CHOICE_DESCRIPTOR).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_KNOB_INFO_DESCRIPTOR => {
                writer
                    .write_all(stringify!(CUDNN_BACKEND_KNOB_INFO_DESCRIPTOR).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_LAYOUT_INFO_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(CUDNN_BACKEND_LAYOUT_INFO_DESCRIPTOR).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_OPERATION_CONVOLUTION_FORWARD_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(
                            CUDNN_BACKEND_OPERATION_CONVOLUTION_FORWARD_DESCRIPTOR
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_OPERATION_CONVOLUTION_BACKWARD_FILTER_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(
                            CUDNN_BACKEND_OPERATION_CONVOLUTION_BACKWARD_FILTER_DESCRIPTOR
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_OPERATION_CONVOLUTION_BACKWARD_DATA_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(
                            CUDNN_BACKEND_OPERATION_CONVOLUTION_BACKWARD_DATA_DESCRIPTOR
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_OPERATION_POINTWISE_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(CUDNN_BACKEND_OPERATION_POINTWISE_DESCRIPTOR)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_OPERATION_GEN_STATS_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(CUDNN_BACKEND_OPERATION_GEN_STATS_DESCRIPTOR)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_OPERATIONGRAPH_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(CUDNN_BACKEND_OPERATIONGRAPH_DESCRIPTOR).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_VARIANT_PACK_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(CUDNN_BACKEND_VARIANT_PACK_DESCRIPTOR).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_TENSOR_DESCRIPTOR => {
                writer.write_all(stringify!(CUDNN_BACKEND_TENSOR_DESCRIPTOR).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_MATMUL_DESCRIPTOR => {
                writer.write_all(stringify!(CUDNN_BACKEND_MATMUL_DESCRIPTOR).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_OPERATION_MATMUL_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(CUDNN_BACKEND_OPERATION_MATMUL_DESCRIPTOR).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_OPERATION_BN_FINALIZE_STATISTICS_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(
                            CUDNN_BACKEND_OPERATION_BN_FINALIZE_STATISTICS_DESCRIPTOR
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_REDUCTION_DESCRIPTOR => {
                writer
                    .write_all(stringify!(CUDNN_BACKEND_REDUCTION_DESCRIPTOR).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_OPERATION_REDUCTION_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(CUDNN_BACKEND_OPERATION_REDUCTION_DESCRIPTOR)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_OPERATION_BN_BWD_WEIGHTS_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(CUDNN_BACKEND_OPERATION_BN_BWD_WEIGHTS_DESCRIPTOR)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_RESAMPLE_DESCRIPTOR => {
                writer
                    .write_all(stringify!(CUDNN_BACKEND_RESAMPLE_DESCRIPTOR).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_OPERATION_RESAMPLE_FWD_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(CUDNN_BACKEND_OPERATION_RESAMPLE_FWD_DESCRIPTOR)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_OPERATION_RESAMPLE_BWD_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(CUDNN_BACKEND_OPERATION_RESAMPLE_BWD_DESCRIPTOR)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_OPERATION_CONCAT_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(CUDNN_BACKEND_OPERATION_CONCAT_DESCRIPTOR).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_OPERATION_SIGNAL_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(CUDNN_BACKEND_OPERATION_SIGNAL_DESCRIPTOR).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_OPERATION_NORM_FORWARD_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(CUDNN_BACKEND_OPERATION_NORM_FORWARD_DESCRIPTOR)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_OPERATION_NORM_BACKWARD_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(CUDNN_BACKEND_OPERATION_NORM_BACKWARD_DESCRIPTOR)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_OPERATION_RESHAPE_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(CUDNN_BACKEND_OPERATION_RESHAPE_DESCRIPTOR).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_RNG_DESCRIPTOR => {
                writer.write_all(stringify!(CUDNN_BACKEND_RNG_DESCRIPTOR).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_OPERATION_RNG_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(CUDNN_BACKEND_OPERATION_RNG_DESCRIPTOR).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_KERNEL_CACHE_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(CUDNN_BACKEND_KERNEL_CACHE_DESCRIPTOR).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_OPERATION_PAGED_CACHE_LOAD_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(CUDNN_BACKEND_OPERATION_PAGED_CACHE_LOAD_DESCRIPTOR)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_OPERATION_BLOCK_SCALE_QUANTIZE_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(
                            CUDNN_BACKEND_OPERATION_BLOCK_SCALE_QUANTIZE_DESCRIPTOR
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_OPERATION_BLOCK_SCALE_DEQUANTIZE_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(
                            CUDNN_BACKEND_OPERATION_BLOCK_SCALE_DEQUANTIZE_DESCRIPTOR
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_DEVICEPROP_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(CUDNN_BACKEND_DEVICEPROP_DESCRIPTOR).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_OPERATION_EXPAND_BAND_MATRIX_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(CUDNN_BACKEND_OPERATION_EXPAND_BAND_MATRIX_DESCRIPTOR)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_OPERATION_CONTRACT_BAND_MATRIX_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(
                            CUDNN_BACKEND_OPERATION_CONTRACT_BAND_MATRIX_DESCRIPTOR
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendDescriptorType_t::CUDNN_BACKEND_OPERATION_SDPA_FWD_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(CUDNN_BACKEND_OPERATION_SDPA_FWD_DESCRIPTOR)
                            .as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnBackendNumericalNote_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnBackendNumericalNote_t::CUDNN_NUMERICAL_NOTE_TENSOR_CORE => {
                writer.write_all(stringify!(CUDNN_NUMERICAL_NOTE_TENSOR_CORE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendNumericalNote_t::CUDNN_NUMERICAL_NOTE_DOWN_CONVERT_INPUTS => {
                writer
                    .write_all(
                        stringify!(CUDNN_NUMERICAL_NOTE_DOWN_CONVERT_INPUTS).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendNumericalNote_t::CUDNN_NUMERICAL_NOTE_REDUCED_PRECISION_REDUCTION => {
                writer
                    .write_all(
                        stringify!(CUDNN_NUMERICAL_NOTE_REDUCED_PRECISION_REDUCTION)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendNumericalNote_t::CUDNN_NUMERICAL_NOTE_FFT => {
                writer.write_all(stringify!(CUDNN_NUMERICAL_NOTE_FFT).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendNumericalNote_t::CUDNN_NUMERICAL_NOTE_NONDETERMINISTIC => {
                writer
                    .write_all(
                        stringify!(CUDNN_NUMERICAL_NOTE_NONDETERMINISTIC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendNumericalNote_t::CUDNN_NUMERICAL_NOTE_WINOGRAD => {
                writer.write_all(stringify!(CUDNN_NUMERICAL_NOTE_WINOGRAD).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendNumericalNote_t::CUDNN_NUMERICAL_NOTE_WINOGRAD_TILE_4x4 => {
                writer
                    .write_all(
                        stringify!(CUDNN_NUMERICAL_NOTE_WINOGRAD_TILE_4x4).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendNumericalNote_t::CUDNN_NUMERICAL_NOTE_WINOGRAD_TILE_6x6 => {
                writer
                    .write_all(
                        stringify!(CUDNN_NUMERICAL_NOTE_WINOGRAD_TILE_6x6).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendNumericalNote_t::CUDNN_NUMERICAL_NOTE_WINOGRAD_TILE_13x13 => {
                writer
                    .write_all(
                        stringify!(CUDNN_NUMERICAL_NOTE_WINOGRAD_TILE_13x13).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendNumericalNote_t::CUDNN_NUMERICAL_NOTE_STRICT_NAN_PROP => {
                writer
                    .write_all(
                        stringify!(CUDNN_NUMERICAL_NOTE_STRICT_NAN_PROP).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendNumericalNote_t::CUDNN_NUMERICAL_NOTE_TYPE_COUNT => {
                writer.write_all(stringify!(CUDNN_NUMERICAL_NOTE_TYPE_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnBackendBehaviorNote_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnBackendBehaviorNote_t::CUDNN_BEHAVIOR_NOTE_RUNTIME_COMPILATION => {
                writer
                    .write_all(
                        stringify!(CUDNN_BEHAVIOR_NOTE_RUNTIME_COMPILATION).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendBehaviorNote_t::CUDNN_BEHAVIOR_NOTE_REQUIRES_FILTER_INT8x32_REORDER => {
                writer
                    .write_all(
                        stringify!(CUDNN_BEHAVIOR_NOTE_REQUIRES_FILTER_INT8x32_REORDER)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendBehaviorNote_t::CUDNN_BEHAVIOR_NOTE_REQUIRES_BIAS_INT8x32_REORDER => {
                writer
                    .write_all(
                        stringify!(CUDNN_BEHAVIOR_NOTE_REQUIRES_BIAS_INT8x32_REORDER)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendBehaviorNote_t::CUDNN_BEHAVIOR_NOTE_SUPPORTS_CUDA_GRAPH_NATIVE_API => {
                writer
                    .write_all(
                        stringify!(CUDNN_BEHAVIOR_NOTE_SUPPORTS_CUDA_GRAPH_NATIVE_API)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendBehaviorNote_t::CUDNN_BEHAVIOR_NOTE_TYPE_COUNT => {
                writer.write_all(stringify!(CUDNN_BEHAVIOR_NOTE_TYPE_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnBackendKnobType_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_SPLIT_K => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_SPLIT_K).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_SWIZZLE => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_SWIZZLE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_TILE_SIZE => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_TILE_SIZE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_USE_TEX => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_USE_TEX).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_EDGE => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_EDGE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_KBLOCK => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_KBLOCK).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_LDGA => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_LDGA).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_LDGB => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_LDGB).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_CHUNK_K => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_CHUNK_K).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_SPLIT_H => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_SPLIT_H).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_WINO_TILE => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_WINO_TILE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_MULTIPLY => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_MULTIPLY).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_SPLIT_K_BUF => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_SPLIT_K_BUF).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_TILEK => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_TILEK).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_STAGES => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_STAGES).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_REDUCTION_MODE => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_REDUCTION_MODE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_CTA_SPLIT_K_MODE => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_CTA_SPLIT_K_MODE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_SPLIT_K_SLC => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_SPLIT_K_SLC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_IDX_MODE => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_IDX_MODE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_SLICED => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_SLICED).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_SPLIT_RS => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_SPLIT_RS).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_SINGLEBUFFER => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_SINGLEBUFFER).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_LDGC => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_LDGC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_SPECFILT => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_SPECFILT).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_KERNEL_CFG => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_KERNEL_CFG).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_WORKSPACE => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_WORKSPACE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_TILE_CGA => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_TILE_CGA).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_TILE_CGA_M => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_TILE_CGA_M).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_TILE_CGA_N => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_TILE_CGA_N).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_BLOCK_SIZE => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_BLOCK_SIZE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_OCCUPANCY => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_OCCUPANCY).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_ARRAY_SIZE_PER_THREAD => {
                writer
                    .write_all(
                        stringify!(CUDNN_KNOB_TYPE_ARRAY_SIZE_PER_THREAD).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_NUM_C_PER_BLOCK => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_NUM_C_PER_BLOCK).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_SPLIT_COLS => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_SPLIT_COLS).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_TILE_ROWS => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_TILE_ROWS).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_TILE_COLS => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_TILE_COLS).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_LOAD_SIZE => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_LOAD_SIZE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_CTA_COUNT => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_CTA_COUNT).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_STREAM_K => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_STREAM_K).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_SPLIT_P_SLC => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_SPLIT_P_SLC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_TILE_M => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_TILE_M).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_TILE_N => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_TILE_N).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_WARP_SPEC_CFG => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_WARP_SPEC_CFG).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendKnobType_t::CUDNN_KNOB_TYPE_COUNTS => {
                writer.write_all(stringify!(CUDNN_KNOB_TYPE_COUNTS).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnBackendLayoutType_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnBackendLayoutType_t::CUDNN_LAYOUT_TYPE_PREFERRED_NCHW => {
                writer.write_all(stringify!(CUDNN_LAYOUT_TYPE_PREFERRED_NCHW).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendLayoutType_t::CUDNN_LAYOUT_TYPE_PREFERRED_NHWC => {
                writer.write_all(stringify!(CUDNN_LAYOUT_TYPE_PREFERRED_NHWC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendLayoutType_t::CUDNN_LAYOUT_TYPE_PREFERRED_PAD4CK => {
                writer
                    .write_all(stringify!(CUDNN_LAYOUT_TYPE_PREFERRED_PAD4CK).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendLayoutType_t::CUDNN_LAYOUT_TYPE_PREFERRED_PAD8CK => {
                writer
                    .write_all(stringify!(CUDNN_LAYOUT_TYPE_PREFERRED_PAD8CK).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendLayoutType_t::CUDNN_LAYOUT_TYPE_COUNT => {
                writer.write_all(stringify!(CUDNN_LAYOUT_TYPE_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnBackendHeurMode_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnBackendHeurMode_t::CUDNN_HEUR_MODE_INSTANT => {
                writer.write_all(stringify!(CUDNN_HEUR_MODE_INSTANT).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendHeurMode_t::CUDNN_HEUR_MODE_B => {
                writer.write_all(stringify!(CUDNN_HEUR_MODE_B).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendHeurMode_t::CUDNN_HEUR_MODE_FALLBACK => {
                writer.write_all(stringify!(CUDNN_HEUR_MODE_FALLBACK).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendHeurMode_t::CUDNN_HEUR_MODE_A => {
                writer.write_all(stringify!(CUDNN_HEUR_MODE_A).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendHeurMode_t::CUDNN_HEUR_MODES_COUNT => {
                writer.write_all(stringify!(CUDNN_HEUR_MODES_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnBackendTensorReordering_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnBackendTensorReordering_t::CUDNN_TENSOR_REORDERING_NONE => {
                writer.write_all(stringify!(CUDNN_TENSOR_REORDERING_NONE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendTensorReordering_t::CUDNN_TENSOR_REORDERING_INT8x32 => {
                writer.write_all(stringify!(CUDNN_TENSOR_REORDERING_INT8x32).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendTensorReordering_t::CUDNN_TENSOR_REORDERING_F16x16 => {
                writer.write_all(stringify!(CUDNN_TENSOR_REORDERING_F16x16).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendTensorReordering_t::CUDNN_TENSOR_REORDERING_F8_128x4 => {
                writer.write_all(stringify!(CUDNN_TENSOR_REORDERING_F8_128x4).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnPaddingMode_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnPaddingMode_t::CUDNN_ZERO_PAD => {
                writer.write_all(stringify!(CUDNN_ZERO_PAD).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPaddingMode_t::CUDNN_NEG_INF_PAD => {
                writer.write_all(stringify!(CUDNN_NEG_INF_PAD).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPaddingMode_t::CUDNN_EDGE_VAL_PAD => {
                writer.write_all(stringify!(CUDNN_EDGE_VAL_PAD).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnBackendNormMode_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnBackendNormMode_t::CUDNN_LAYER_NORM => {
                writer.write_all(stringify!(CUDNN_LAYER_NORM).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendNormMode_t::CUDNN_INSTANCE_NORM => {
                writer.write_all(stringify!(CUDNN_INSTANCE_NORM).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendNormMode_t::CUDNN_BATCH_NORM => {
                writer.write_all(stringify!(CUDNN_BATCH_NORM).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendNormMode_t::CUDNN_GROUP_NORM => {
                writer.write_all(stringify!(CUDNN_GROUP_NORM).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendNormMode_t::CUDNN_RMS_NORM => {
                writer.write_all(stringify!(CUDNN_RMS_NORM).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendNormMode_t::CUDNN_ADA_LAYER_NORM => {
                writer.write_all(stringify!(CUDNN_ADA_LAYER_NORM).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnBackendNormFwdPhase_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnBackendNormFwdPhase_t::CUDNN_NORM_FWD_INFERENCE => {
                writer.write_all(stringify!(CUDNN_NORM_FWD_INFERENCE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBackendNormFwdPhase_t::CUDNN_NORM_FWD_TRAINING => {
                writer.write_all(stringify!(CUDNN_NORM_FWD_TRAINING).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cudnnBackendCreateDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    descriptorType: cuda_types::cudnn9::cudnnBackendDescriptorType_t,
    descriptor: *mut cuda_types::cudnn9::cudnnBackendDescriptor_t,
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
    descriptor: cuda_types::cudnn9::cudnnBackendDescriptor_t,
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
    descriptor: cuda_types::cudnn9::cudnnBackendDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(descriptor), ": ").as_bytes())?;
    crate::CudaDisplay::write(&descriptor, "cudnnBackendInitialize", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnBackendFinalize(
    writer: &mut (impl std::io::Write + ?Sized),
    descriptor: cuda_types::cudnn9::cudnnBackendDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(descriptor), ": ").as_bytes())?;
    crate::CudaDisplay::write(&descriptor, "cudnnBackendFinalize", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnBackendExecute(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn9::cudnnHandle_t,
    executionPlan: cuda_types::cudnn9::cudnnBackendDescriptor_t,
    variantPack: cuda_types::cudnn9::cudnnBackendDescriptor_t,
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
pub fn write_cudnnBackendPopulateCudaGraph(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn9::cudnnHandle_t,
    executionPlan: cuda_types::cudnn9::cudnnBackendDescriptor_t,
    variantPack: cuda_types::cudnn9::cudnnBackendDescriptor_t,
    graph: cuda_types::cudnn9::cudaGraph_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cudnnBackendPopulateCudaGraph",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(executionPlan), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &executionPlan,
        "cudnnBackendPopulateCudaGraph",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(variantPack), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &variantPack,
        "cudnnBackendPopulateCudaGraph",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(graph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&graph, "cudnnBackendPopulateCudaGraph", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnBackendUpdateCudaGraph(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn9::cudnnHandle_t,
    executionPlan: cuda_types::cudnn9::cudnnBackendDescriptor_t,
    variantPack: cuda_types::cudnn9::cudnnBackendDescriptor_t,
    graph: cuda_types::cudnn9::cudaGraph_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cudnnBackendUpdateCudaGraph", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(executionPlan), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &executionPlan,
        "cudnnBackendUpdateCudaGraph",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(variantPack), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &variantPack,
        "cudnnBackendUpdateCudaGraph",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(graph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&graph, "cudnnBackendUpdateCudaGraph", arg_idx, writer)?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnTensorDescriptor_t {
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnPoolingDescriptor_t {
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnFilterDescriptor_t {
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnLRNDescriptor_t {
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnActivationDescriptor_t {
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnSpatialTransformerDescriptor_t {
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnOpTensorDescriptor_t {
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnReduceTensorDescriptor_t {
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnCTCLossDescriptor_t {
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnTensorTransformDescriptor_t {
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnDeterminism_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnDeterminism_t::CUDNN_NON_DETERMINISTIC => {
                writer.write_all(stringify!(CUDNN_NON_DETERMINISTIC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnDeterminism_t::CUDNN_DETERMINISTIC => {
                writer.write_all(stringify!(CUDNN_DETERMINISTIC).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cudnnCreateTensorDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    tensorDesc: *mut cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    tensorDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    format: cuda_types::cudnn9::cudnnTensorFormat_t,
    dataType: cuda_types::cudnn9::cudnnDataType_t,
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
    tensorDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dataType: cuda_types::cudnn9::cudnnDataType_t,
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
    tensorDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dataType: *mut cuda_types::cudnn9::cudnnDataType_t,
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
    tensorDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dataType: cuda_types::cudnn9::cudnnDataType_t,
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
    tensorDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    format: cuda_types::cudnn9::cudnnTensorFormat_t,
    dataType: cuda_types::cudnn9::cudnnDataType_t,
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
    tensorDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    nbDimsRequested: ::core::ffi::c_int,
    dataType: *mut cuda_types::cudnn9::cudnnDataType_t,
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
    tensorDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    tensorDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnFoldingDirection_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnFoldingDirection_t::CUDNN_TRANSFORM_FOLD => {
                writer.write_all(stringify!(CUDNN_TRANSFORM_FOLD).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFoldingDirection_t::CUDNN_TRANSFORM_UNFOLD => {
                writer.write_all(stringify!(CUDNN_TRANSFORM_UNFOLD).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cudnnInitTransformDest(
    writer: &mut (impl std::io::Write + ?Sized),
    transformDesc: cuda_types::cudnn9::cudnnTensorTransformDescriptor_t,
    srcDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    destDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    transformDesc: *mut cuda_types::cudnn9::cudnnTensorTransformDescriptor_t,
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
    transformDesc: cuda_types::cudnn9::cudnnTensorTransformDescriptor_t,
    nbDims: u32,
    destFormat: cuda_types::cudnn9::cudnnTensorFormat_t,
    padBeforeA: *const i32,
    padAfterA: *const i32,
    foldA: *const u32,
    direction: cuda_types::cudnn9::cudnnFoldingDirection_t,
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
    transformDesc: cuda_types::cudnn9::cudnnTensorTransformDescriptor_t,
    nbDimsRequested: u32,
    destFormat: *mut cuda_types::cudnn9::cudnnTensorFormat_t,
    padBeforeA: *mut i32,
    padAfterA: *mut i32,
    foldA: *mut u32,
    direction: *mut cuda_types::cudnn9::cudnnFoldingDirection_t,
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
    transformDesc: cuda_types::cudnn9::cudnnTensorTransformDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    alpha: *const ::core::ffi::c_void,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    yDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    transDesc: cuda_types::cudnn9::cudnnTensorTransformDescriptor_t,
    alpha: *const ::core::ffi::c_void,
    srcDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    srcData: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    destDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    alpha: *const ::core::ffi::c_void,
    aDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    A: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    cDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnOpTensorOp_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnOpTensorOp_t::CUDNN_OP_TENSOR_ADD => {
                writer.write_all(stringify!(CUDNN_OP_TENSOR_ADD).as_bytes())
            }
            &cuda_types::cudnn9::cudnnOpTensorOp_t::CUDNN_OP_TENSOR_MUL => {
                writer.write_all(stringify!(CUDNN_OP_TENSOR_MUL).as_bytes())
            }
            &cuda_types::cudnn9::cudnnOpTensorOp_t::CUDNN_OP_TENSOR_MIN => {
                writer.write_all(stringify!(CUDNN_OP_TENSOR_MIN).as_bytes())
            }
            &cuda_types::cudnn9::cudnnOpTensorOp_t::CUDNN_OP_TENSOR_MAX => {
                writer.write_all(stringify!(CUDNN_OP_TENSOR_MAX).as_bytes())
            }
            &cuda_types::cudnn9::cudnnOpTensorOp_t::CUDNN_OP_TENSOR_SQRT => {
                writer.write_all(stringify!(CUDNN_OP_TENSOR_SQRT).as_bytes())
            }
            &cuda_types::cudnn9::cudnnOpTensorOp_t::CUDNN_OP_TENSOR_NOT => {
                writer.write_all(stringify!(CUDNN_OP_TENSOR_NOT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cudnnCreateOpTensorDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    opTensorDesc: *mut cuda_types::cudnn9::cudnnOpTensorDescriptor_t,
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
    opTensorDesc: cuda_types::cudnn9::cudnnOpTensorDescriptor_t,
    opTensorOp: cuda_types::cudnn9::cudnnOpTensorOp_t,
    opTensorCompType: cuda_types::cudnn9::cudnnDataType_t,
    opTensorNanOpt: cuda_types::cudnn9::cudnnNanPropagation_t,
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
    opTensorDesc: cuda_types::cudnn9::cudnnOpTensorDescriptor_t,
    opTensorOp: *mut cuda_types::cudnn9::cudnnOpTensorOp_t,
    opTensorCompType: *mut cuda_types::cudnn9::cudnnDataType_t,
    opTensorNanOpt: *mut cuda_types::cudnn9::cudnnNanPropagation_t,
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
    opTensorDesc: cuda_types::cudnn9::cudnnOpTensorDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    opTensorDesc: cuda_types::cudnn9::cudnnOpTensorDescriptor_t,
    alpha1: *const ::core::ffi::c_void,
    aDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    A: *const ::core::ffi::c_void,
    alpha2: *const ::core::ffi::c_void,
    bDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    B: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    cDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnReduceTensorIndices_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnReduceTensorIndices_t::CUDNN_REDUCE_TENSOR_NO_INDICES => {
                writer.write_all(stringify!(CUDNN_REDUCE_TENSOR_NO_INDICES).as_bytes())
            }
            &cuda_types::cudnn9::cudnnReduceTensorIndices_t::CUDNN_REDUCE_TENSOR_FLATTENED_INDICES => {
                writer
                    .write_all(
                        stringify!(CUDNN_REDUCE_TENSOR_FLATTENED_INDICES).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnIndicesType_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnIndicesType_t::CUDNN_32BIT_INDICES => {
                writer.write_all(stringify!(CUDNN_32BIT_INDICES).as_bytes())
            }
            &cuda_types::cudnn9::cudnnIndicesType_t::CUDNN_64BIT_INDICES => {
                writer.write_all(stringify!(CUDNN_64BIT_INDICES).as_bytes())
            }
            &cuda_types::cudnn9::cudnnIndicesType_t::CUDNN_16BIT_INDICES => {
                writer.write_all(stringify!(CUDNN_16BIT_INDICES).as_bytes())
            }
            &cuda_types::cudnn9::cudnnIndicesType_t::CUDNN_8BIT_INDICES => {
                writer.write_all(stringify!(CUDNN_8BIT_INDICES).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cudnnCreateReduceTensorDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    reduceTensorDesc: *mut cuda_types::cudnn9::cudnnReduceTensorDescriptor_t,
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
    reduceTensorDesc: cuda_types::cudnn9::cudnnReduceTensorDescriptor_t,
    reduceTensorOp: cuda_types::cudnn9::cudnnReduceTensorOp_t,
    reduceTensorCompType: cuda_types::cudnn9::cudnnDataType_t,
    reduceTensorNanOpt: cuda_types::cudnn9::cudnnNanPropagation_t,
    reduceTensorIndices: cuda_types::cudnn9::cudnnReduceTensorIndices_t,
    reduceTensorIndicesType: cuda_types::cudnn9::cudnnIndicesType_t,
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
    reduceTensorDesc: cuda_types::cudnn9::cudnnReduceTensorDescriptor_t,
    reduceTensorOp: *mut cuda_types::cudnn9::cudnnReduceTensorOp_t,
    reduceTensorCompType: *mut cuda_types::cudnn9::cudnnDataType_t,
    reduceTensorNanOpt: *mut cuda_types::cudnn9::cudnnNanPropagation_t,
    reduceTensorIndices: *mut cuda_types::cudnn9::cudnnReduceTensorIndices_t,
    reduceTensorIndicesType: *mut cuda_types::cudnn9::cudnnIndicesType_t,
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
    reduceTensorDesc: cuda_types::cudnn9::cudnnReduceTensorDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    reduceTensorDesc: cuda_types::cudnn9::cudnnReduceTensorDescriptor_t,
    aDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    cDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    reduceTensorDesc: cuda_types::cudnn9::cudnnReduceTensorDescriptor_t,
    aDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    cDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    reduceTensorDesc: cuda_types::cudnn9::cudnnReduceTensorDescriptor_t,
    indices: *mut ::core::ffi::c_void,
    indicesSizeInBytes: usize,
    workspace: *mut ::core::ffi::c_void,
    workspaceSizeInBytes: usize,
    alpha: *const ::core::ffi::c_void,
    aDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    A: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    cDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    yDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    yDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    filterDesc: *mut cuda_types::cudnn9::cudnnFilterDescriptor_t,
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
    filterDesc: cuda_types::cudnn9::cudnnFilterDescriptor_t,
    dataType: cuda_types::cudnn9::cudnnDataType_t,
    format: cuda_types::cudnn9::cudnnTensorFormat_t,
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
    filterDesc: cuda_types::cudnn9::cudnnFilterDescriptor_t,
    dataType: *mut cuda_types::cudnn9::cudnnDataType_t,
    format: *mut cuda_types::cudnn9::cudnnTensorFormat_t,
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
    filterDesc: cuda_types::cudnn9::cudnnFilterDescriptor_t,
    dataType: cuda_types::cudnn9::cudnnDataType_t,
    format: cuda_types::cudnn9::cudnnTensorFormat_t,
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
    filterDesc: cuda_types::cudnn9::cudnnFilterDescriptor_t,
    nbDimsRequested: ::core::ffi::c_int,
    dataType: *mut cuda_types::cudnn9::cudnnDataType_t,
    format: *mut cuda_types::cudnn9::cudnnTensorFormat_t,
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
    filterDesc: cuda_types::cudnn9::cudnnFilterDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    transDesc: cuda_types::cudnn9::cudnnTensorTransformDescriptor_t,
    alpha: *const ::core::ffi::c_void,
    srcDesc: cuda_types::cudnn9::cudnnFilterDescriptor_t,
    srcData: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    destDesc: cuda_types::cudnn9::cudnnFilterDescriptor_t,
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
    filterDesc: cuda_types::cudnn9::cudnnFilterDescriptor_t,
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnSoftmaxAlgorithm_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnSoftmaxAlgorithm_t::CUDNN_SOFTMAX_FAST => {
                writer.write_all(stringify!(CUDNN_SOFTMAX_FAST).as_bytes())
            }
            &cuda_types::cudnn9::cudnnSoftmaxAlgorithm_t::CUDNN_SOFTMAX_ACCURATE => {
                writer.write_all(stringify!(CUDNN_SOFTMAX_ACCURATE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnSoftmaxAlgorithm_t::CUDNN_SOFTMAX_LOG => {
                writer.write_all(stringify!(CUDNN_SOFTMAX_LOG).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnSoftmaxMode_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnSoftmaxMode_t::CUDNN_SOFTMAX_MODE_INSTANCE => {
                writer.write_all(stringify!(CUDNN_SOFTMAX_MODE_INSTANCE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnSoftmaxMode_t::CUDNN_SOFTMAX_MODE_CHANNEL => {
                writer.write_all(stringify!(CUDNN_SOFTMAX_MODE_CHANNEL).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cudnnSoftmaxForward(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn9::cudnnHandle_t,
    algo: cuda_types::cudnn9::cudnnSoftmaxAlgorithm_t,
    mode: cuda_types::cudnn9::cudnnSoftmaxMode_t,
    alpha: *const ::core::ffi::c_void,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    yDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnPoolingMode_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnPoolingMode_t::CUDNN_POOLING_MAX => {
                writer.write_all(stringify!(CUDNN_POOLING_MAX).as_bytes())
            }
            &cuda_types::cudnn9::cudnnPoolingMode_t::CUDNN_POOLING_AVERAGE_COUNT_INCLUDE_PADDING => {
                writer
                    .write_all(
                        stringify!(CUDNN_POOLING_AVERAGE_COUNT_INCLUDE_PADDING)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnPoolingMode_t::CUDNN_POOLING_AVERAGE_COUNT_EXCLUDE_PADDING => {
                writer
                    .write_all(
                        stringify!(CUDNN_POOLING_AVERAGE_COUNT_EXCLUDE_PADDING)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnPoolingMode_t::CUDNN_POOLING_MAX_DETERMINISTIC => {
                writer.write_all(stringify!(CUDNN_POOLING_MAX_DETERMINISTIC).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cudnnCreatePoolingDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    poolingDesc: *mut cuda_types::cudnn9::cudnnPoolingDescriptor_t,
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
    poolingDesc: cuda_types::cudnn9::cudnnPoolingDescriptor_t,
    mode: cuda_types::cudnn9::cudnnPoolingMode_t,
    maxpoolingNanOpt: cuda_types::cudnn9::cudnnNanPropagation_t,
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
    poolingDesc: cuda_types::cudnn9::cudnnPoolingDescriptor_t,
    mode: *mut cuda_types::cudnn9::cudnnPoolingMode_t,
    maxpoolingNanOpt: *mut cuda_types::cudnn9::cudnnNanPropagation_t,
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
    poolingDesc: cuda_types::cudnn9::cudnnPoolingDescriptor_t,
    mode: cuda_types::cudnn9::cudnnPoolingMode_t,
    maxpoolingNanOpt: cuda_types::cudnn9::cudnnNanPropagation_t,
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
    poolingDesc: cuda_types::cudnn9::cudnnPoolingDescriptor_t,
    nbDimsRequested: ::core::ffi::c_int,
    mode: *mut cuda_types::cudnn9::cudnnPoolingMode_t,
    maxpoolingNanOpt: *mut cuda_types::cudnn9::cudnnNanPropagation_t,
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
    poolingDesc: cuda_types::cudnn9::cudnnPoolingDescriptor_t,
    inputTensorDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    poolingDesc: cuda_types::cudnn9::cudnnPoolingDescriptor_t,
    inputTensorDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    poolingDesc: cuda_types::cudnn9::cudnnPoolingDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    poolingDesc: cuda_types::cudnn9::cudnnPoolingDescriptor_t,
    alpha: *const ::core::ffi::c_void,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    yDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    activationDesc: *mut cuda_types::cudnn9::cudnnActivationDescriptor_t,
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
    activationDesc: cuda_types::cudnn9::cudnnActivationDescriptor_t,
    mode: cuda_types::cudnn9::cudnnActivationMode_t,
    reluNanOpt: cuda_types::cudnn9::cudnnNanPropagation_t,
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
    activationDesc: cuda_types::cudnn9::cudnnActivationDescriptor_t,
    mode: *mut cuda_types::cudnn9::cudnnActivationMode_t,
    reluNanOpt: *mut cuda_types::cudnn9::cudnnNanPropagation_t,
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
    activationDesc: cuda_types::cudnn9::cudnnActivationDescriptor_t,
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
    activationDesc: cuda_types::cudnn9::cudnnActivationDescriptor_t,
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
    activationDesc: cuda_types::cudnn9::cudnnActivationDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    activationDesc: cuda_types::cudnn9::cudnnActivationDescriptor_t,
    alpha: *const ::core::ffi::c_void,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    yDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    normDesc: *mut cuda_types::cudnn9::cudnnLRNDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(normDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&normDesc, "cudnnCreateLRNDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnLRNMode_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnLRNMode_t::CUDNN_LRN_CROSS_CHANNEL_DIM1 => {
                writer.write_all(stringify!(CUDNN_LRN_CROSS_CHANNEL_DIM1).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cudnnSetLRNDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    normDesc: cuda_types::cudnn9::cudnnLRNDescriptor_t,
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
    normDesc: cuda_types::cudnn9::cudnnLRNDescriptor_t,
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
    lrnDesc: cuda_types::cudnn9::cudnnLRNDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(lrnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&lrnDesc, "cudnnDestroyLRNDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnLRNCrossChannelForward(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn9::cudnnHandle_t,
    normDesc: cuda_types::cudnn9::cudnnLRNDescriptor_t,
    lrnMode: cuda_types::cudnn9::cudnnLRNMode_t,
    alpha: *const ::core::ffi::c_void,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    yDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnDivNormMode_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnDivNormMode_t::CUDNN_DIVNORM_PRECOMPUTED_MEANS => {
                writer.write_all(stringify!(CUDNN_DIVNORM_PRECOMPUTED_MEANS).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cudnnDivisiveNormalizationForward(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn9::cudnnHandle_t,
    normDesc: cuda_types::cudnn9::cudnnLRNDescriptor_t,
    mode: cuda_types::cudnn9::cudnnDivNormMode_t,
    alpha: *const ::core::ffi::c_void,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    means: *const ::core::ffi::c_void,
    temp: *mut ::core::ffi::c_void,
    temp2: *mut ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    yDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnBatchNormMode_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnBatchNormMode_t::CUDNN_BATCHNORM_PER_ACTIVATION => {
                writer.write_all(stringify!(CUDNN_BATCHNORM_PER_ACTIVATION).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBatchNormMode_t::CUDNN_BATCHNORM_SPATIAL => {
                writer.write_all(stringify!(CUDNN_BATCHNORM_SPATIAL).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBatchNormMode_t::CUDNN_BATCHNORM_SPATIAL_PERSISTENT => {
                writer
                    .write_all(stringify!(CUDNN_BATCHNORM_SPATIAL_PERSISTENT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cudnnDeriveBNTensorDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    derivedBnDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    mode: cuda_types::cudnn9::cudnnBatchNormMode_t,
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnBatchNormOps_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnBatchNormOps_t::CUDNN_BATCHNORM_OPS_BN => {
                writer.write_all(stringify!(CUDNN_BATCHNORM_OPS_BN).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBatchNormOps_t::CUDNN_BATCHNORM_OPS_BN_ACTIVATION => {
                writer
                    .write_all(stringify!(CUDNN_BATCHNORM_OPS_BN_ACTIVATION).as_bytes())
            }
            &cuda_types::cudnn9::cudnnBatchNormOps_t::CUDNN_BATCHNORM_OPS_BN_ADD_ACTIVATION => {
                writer
                    .write_all(
                        stringify!(CUDNN_BATCHNORM_OPS_BN_ADD_ACTIVATION).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cudnnBatchNormalizationForwardInference(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn9::cudnnHandle_t,
    mode: cuda_types::cudnn9::cudnnBatchNormMode_t,
    alpha: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    yDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    y: *mut ::core::ffi::c_void,
    bnScaleBiasMeanVarDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnNormMode_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnNormMode_t::CUDNN_NORM_PER_ACTIVATION => {
                writer.write_all(stringify!(CUDNN_NORM_PER_ACTIVATION).as_bytes())
            }
            &cuda_types::cudnn9::cudnnNormMode_t::CUDNN_NORM_PER_CHANNEL => {
                writer.write_all(stringify!(CUDNN_NORM_PER_CHANNEL).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnNormAlgo_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnNormAlgo_t::CUDNN_NORM_ALGO_STANDARD => {
                writer.write_all(stringify!(CUDNN_NORM_ALGO_STANDARD).as_bytes())
            }
            &cuda_types::cudnn9::cudnnNormAlgo_t::CUDNN_NORM_ALGO_PERSIST => {
                writer.write_all(stringify!(CUDNN_NORM_ALGO_PERSIST).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cudnnDeriveNormTensorDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    derivedNormScaleBiasDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    derivedNormMeanVarDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    mode: cuda_types::cudnn9::cudnnNormMode_t,
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnNormOps_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnNormOps_t::CUDNN_NORM_OPS_NORM => {
                writer.write_all(stringify!(CUDNN_NORM_OPS_NORM).as_bytes())
            }
            &cuda_types::cudnn9::cudnnNormOps_t::CUDNN_NORM_OPS_NORM_ACTIVATION => {
                writer.write_all(stringify!(CUDNN_NORM_OPS_NORM_ACTIVATION).as_bytes())
            }
            &cuda_types::cudnn9::cudnnNormOps_t::CUDNN_NORM_OPS_NORM_ADD_ACTIVATION => {
                writer
                    .write_all(stringify!(CUDNN_NORM_OPS_NORM_ADD_ACTIVATION).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cudnnNormalizationForwardInference(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn9::cudnnHandle_t,
    mode: cuda_types::cudnn9::cudnnNormMode_t,
    normOps: cuda_types::cudnn9::cudnnNormOps_t,
    algo: cuda_types::cudnn9::cudnnNormAlgo_t,
    alpha: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    normScaleBiasDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    normScale: *const ::core::ffi::c_void,
    normBias: *const ::core::ffi::c_void,
    normMeanVarDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    estimatedMean: *const ::core::ffi::c_void,
    estimatedVariance: *const ::core::ffi::c_void,
    zDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    z: *const ::core::ffi::c_void,
    activationDesc: cuda_types::cudnn9::cudnnActivationDescriptor_t,
    yDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnSamplerType_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnSamplerType_t::CUDNN_SAMPLER_BILINEAR => {
                writer.write_all(stringify!(CUDNN_SAMPLER_BILINEAR).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cudnnCreateSpatialTransformerDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    stDesc: *mut cuda_types::cudnn9::cudnnSpatialTransformerDescriptor_t,
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
    stDesc: cuda_types::cudnn9::cudnnSpatialTransformerDescriptor_t,
    samplerType: cuda_types::cudnn9::cudnnSamplerType_t,
    dataType: cuda_types::cudnn9::cudnnDataType_t,
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
    stDesc: cuda_types::cudnn9::cudnnSpatialTransformerDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    stDesc: cuda_types::cudnn9::cudnnSpatialTransformerDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    stDesc: cuda_types::cudnn9::cudnnSpatialTransformerDescriptor_t,
    alpha: *const ::core::ffi::c_void,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    grid: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    yDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnDropoutDescriptor_t {
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
pub fn write_cudnnCreateDropoutDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    dropoutDesc: *mut cuda_types::cudnn9::cudnnDropoutDescriptor_t,
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
    dropoutDesc: cuda_types::cudnn9::cudnnDropoutDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
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
    xdesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    dropoutDesc: cuda_types::cudnn9::cudnnDropoutDescriptor_t,
    handle: cuda_types::cudnn9::cudnnHandle_t,
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
    dropoutDesc: cuda_types::cudnn9::cudnnDropoutDescriptor_t,
    handle: cuda_types::cudnn9::cudnnHandle_t,
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
    dropoutDesc: cuda_types::cudnn9::cudnnDropoutDescriptor_t,
    handle: cuda_types::cudnn9::cudnnHandle_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    dropoutDesc: cuda_types::cudnn9::cudnnDropoutDescriptor_t,
    xdesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    ydesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnConvolutionFwdAlgo_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_IMPLICIT_GEMM => {
                writer
                    .write_all(
                        stringify!(CUDNN_CONVOLUTION_FWD_ALGO_IMPLICIT_GEMM).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_IMPLICIT_PRECOMP_GEMM => {
                writer
                    .write_all(
                        stringify!(CUDNN_CONVOLUTION_FWD_ALGO_IMPLICIT_PRECOMP_GEMM)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_GEMM => {
                writer.write_all(stringify!(CUDNN_CONVOLUTION_FWD_ALGO_GEMM).as_bytes())
            }
            &cuda_types::cudnn9::cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_DIRECT => {
                writer
                    .write_all(stringify!(CUDNN_CONVOLUTION_FWD_ALGO_DIRECT).as_bytes())
            }
            &cuda_types::cudnn9::cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_FFT => {
                writer.write_all(stringify!(CUDNN_CONVOLUTION_FWD_ALGO_FFT).as_bytes())
            }
            &cuda_types::cudnn9::cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_FFT_TILING => {
                writer
                    .write_all(
                        stringify!(CUDNN_CONVOLUTION_FWD_ALGO_FFT_TILING).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_WINOGRAD => {
                writer
                    .write_all(
                        stringify!(CUDNN_CONVOLUTION_FWD_ALGO_WINOGRAD).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_WINOGRAD_NONFUSED => {
                writer
                    .write_all(
                        stringify!(CUDNN_CONVOLUTION_FWD_ALGO_WINOGRAD_NONFUSED)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_COUNT => {
                writer.write_all(stringify!(CUDNN_CONVOLUTION_FWD_ALGO_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnConvolutionBwdFilterAlgo_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnConvolutionBwdFilterAlgo_t::CUDNN_CONVOLUTION_BWD_FILTER_ALGO_0 => {
                writer
                    .write_all(
                        stringify!(CUDNN_CONVOLUTION_BWD_FILTER_ALGO_0).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnConvolutionBwdFilterAlgo_t::CUDNN_CONVOLUTION_BWD_FILTER_ALGO_1 => {
                writer
                    .write_all(
                        stringify!(CUDNN_CONVOLUTION_BWD_FILTER_ALGO_1).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnConvolutionBwdFilterAlgo_t::CUDNN_CONVOLUTION_BWD_FILTER_ALGO_FFT => {
                writer
                    .write_all(
                        stringify!(CUDNN_CONVOLUTION_BWD_FILTER_ALGO_FFT).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnConvolutionBwdFilterAlgo_t::CUDNN_CONVOLUTION_BWD_FILTER_ALGO_3 => {
                writer
                    .write_all(
                        stringify!(CUDNN_CONVOLUTION_BWD_FILTER_ALGO_3).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnConvolutionBwdFilterAlgo_t::CUDNN_CONVOLUTION_BWD_FILTER_ALGO_WINOGRAD => {
                writer
                    .write_all(
                        stringify!(CUDNN_CONVOLUTION_BWD_FILTER_ALGO_WINOGRAD).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnConvolutionBwdFilterAlgo_t::CUDNN_CONVOLUTION_BWD_FILTER_ALGO_WINOGRAD_NONFUSED => {
                writer
                    .write_all(
                        stringify!(CUDNN_CONVOLUTION_BWD_FILTER_ALGO_WINOGRAD_NONFUSED)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnConvolutionBwdFilterAlgo_t::CUDNN_CONVOLUTION_BWD_FILTER_ALGO_FFT_TILING => {
                writer
                    .write_all(
                        stringify!(CUDNN_CONVOLUTION_BWD_FILTER_ALGO_FFT_TILING)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnConvolutionBwdFilterAlgo_t::CUDNN_CONVOLUTION_BWD_FILTER_ALGO_COUNT => {
                writer
                    .write_all(
                        stringify!(CUDNN_CONVOLUTION_BWD_FILTER_ALGO_COUNT).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnConvolutionBwdDataAlgo_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_0 => {
                writer
                    .write_all(stringify!(CUDNN_CONVOLUTION_BWD_DATA_ALGO_0).as_bytes())
            }
            &cuda_types::cudnn9::cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_1 => {
                writer
                    .write_all(stringify!(CUDNN_CONVOLUTION_BWD_DATA_ALGO_1).as_bytes())
            }
            &cuda_types::cudnn9::cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_FFT => {
                writer
                    .write_all(
                        stringify!(CUDNN_CONVOLUTION_BWD_DATA_ALGO_FFT).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_FFT_TILING => {
                writer
                    .write_all(
                        stringify!(CUDNN_CONVOLUTION_BWD_DATA_ALGO_FFT_TILING).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_WINOGRAD => {
                writer
                    .write_all(
                        stringify!(CUDNN_CONVOLUTION_BWD_DATA_ALGO_WINOGRAD).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_WINOGRAD_NONFUSED => {
                writer
                    .write_all(
                        stringify!(CUDNN_CONVOLUTION_BWD_DATA_ALGO_WINOGRAD_NONFUSED)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_COUNT => {
                writer
                    .write_all(
                        stringify!(CUDNN_CONVOLUTION_BWD_DATA_ALGO_COUNT).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnCTCLossAlgo_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnCTCLossAlgo_t::CUDNN_CTC_LOSS_ALGO_DETERMINISTIC => {
                writer
                    .write_all(stringify!(CUDNN_CTC_LOSS_ALGO_DETERMINISTIC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnCTCLossAlgo_t::CUDNN_CTC_LOSS_ALGO_NON_DETERMINISTIC => {
                writer
                    .write_all(
                        stringify!(CUDNN_CTC_LOSS_ALGO_NON_DETERMINISTIC).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cudnnOpsVersionCheck(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
pub fn write_cudnnSoftmaxBackward(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn9::cudnnHandle_t,
    algo: cuda_types::cudnn9::cudnnSoftmaxAlgorithm_t,
    mode: cuda_types::cudnn9::cudnnSoftmaxMode_t,
    alpha: *const ::core::ffi::c_void,
    yDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    y: *const ::core::ffi::c_void,
    dyDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dy: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    dxDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    poolingDesc: cuda_types::cudnn9::cudnnPoolingDescriptor_t,
    alpha: *const ::core::ffi::c_void,
    yDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    y: *const ::core::ffi::c_void,
    dyDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dy: *const ::core::ffi::c_void,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    dxDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    activationDesc: cuda_types::cudnn9::cudnnActivationDescriptor_t,
    alpha: *const ::core::ffi::c_void,
    yDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    y: *const ::core::ffi::c_void,
    dyDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dy: *const ::core::ffi::c_void,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    dxDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    normDesc: cuda_types::cudnn9::cudnnLRNDescriptor_t,
    lrnMode: cuda_types::cudnn9::cudnnLRNMode_t,
    alpha: *const ::core::ffi::c_void,
    yDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    y: *const ::core::ffi::c_void,
    dyDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dy: *const ::core::ffi::c_void,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    dxDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    normDesc: cuda_types::cudnn9::cudnnLRNDescriptor_t,
    mode: cuda_types::cudnn9::cudnnDivNormMode_t,
    alpha: *const ::core::ffi::c_void,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    means: *const ::core::ffi::c_void,
    dy: *const ::core::ffi::c_void,
    temp: *mut ::core::ffi::c_void,
    temp2: *mut ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    dXdMeansDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    mode: cuda_types::cudnn9::cudnnBatchNormMode_t,
    bnOps: cuda_types::cudnn9::cudnnBatchNormOps_t,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    zDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    yDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    bnScaleBiasMeanVarDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    activationDesc: cuda_types::cudnn9::cudnnActivationDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    mode: cuda_types::cudnn9::cudnnBatchNormMode_t,
    bnOps: cuda_types::cudnn9::cudnnBatchNormOps_t,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    yDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dyDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dzDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dxDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dBnScaleBiasDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    activationDesc: cuda_types::cudnn9::cudnnActivationDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    mode: cuda_types::cudnn9::cudnnBatchNormMode_t,
    bnOps: cuda_types::cudnn9::cudnnBatchNormOps_t,
    activationDesc: cuda_types::cudnn9::cudnnActivationDescriptor_t,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    mode: cuda_types::cudnn9::cudnnBatchNormMode_t,
    alpha: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    yDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    y: *mut ::core::ffi::c_void,
    bnScaleBiasMeanVarDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    mode: cuda_types::cudnn9::cudnnBatchNormMode_t,
    bnOps: cuda_types::cudnn9::cudnnBatchNormOps_t,
    alpha: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    xData: *const ::core::ffi::c_void,
    zDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    zData: *const ::core::ffi::c_void,
    yDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    yData: *mut ::core::ffi::c_void,
    bnScaleBiasMeanVarDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    bnScale: *const ::core::ffi::c_void,
    bnBias: *const ::core::ffi::c_void,
    exponentialAverageFactor: f64,
    resultRunningMean: *mut ::core::ffi::c_void,
    resultRunningVariance: *mut ::core::ffi::c_void,
    epsilon: f64,
    resultSaveMean: *mut ::core::ffi::c_void,
    resultSaveInvVariance: *mut ::core::ffi::c_void,
    activationDesc: cuda_types::cudnn9::cudnnActivationDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    mode: cuda_types::cudnn9::cudnnBatchNormMode_t,
    alphaDataDiff: *const ::core::ffi::c_void,
    betaDataDiff: *const ::core::ffi::c_void,
    alphaParamDiff: *const ::core::ffi::c_void,
    betaParamDiff: *const ::core::ffi::c_void,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    dyDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dy: *const ::core::ffi::c_void,
    dxDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dx: *mut ::core::ffi::c_void,
    dBnScaleBiasDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    mode: cuda_types::cudnn9::cudnnBatchNormMode_t,
    bnOps: cuda_types::cudnn9::cudnnBatchNormOps_t,
    alphaDataDiff: *const ::core::ffi::c_void,
    betaDataDiff: *const ::core::ffi::c_void,
    alphaParamDiff: *const ::core::ffi::c_void,
    betaParamDiff: *const ::core::ffi::c_void,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    xData: *const ::core::ffi::c_void,
    yDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    yData: *const ::core::ffi::c_void,
    dyDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dyData: *const ::core::ffi::c_void,
    dzDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dzData: *mut ::core::ffi::c_void,
    dxDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dxData: *mut ::core::ffi::c_void,
    dBnScaleBiasDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    bnScaleData: *const ::core::ffi::c_void,
    bnBiasData: *const ::core::ffi::c_void,
    dBnScaleData: *mut ::core::ffi::c_void,
    dBnBiasData: *mut ::core::ffi::c_void,
    epsilon: f64,
    savedMean: *const ::core::ffi::c_void,
    savedInvVariance: *const ::core::ffi::c_void,
    activationDesc: cuda_types::cudnn9::cudnnActivationDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    mode: cuda_types::cudnn9::cudnnNormMode_t,
    normOps: cuda_types::cudnn9::cudnnNormOps_t,
    algo: cuda_types::cudnn9::cudnnNormAlgo_t,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    zDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    yDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    normScaleBiasDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    activationDesc: cuda_types::cudnn9::cudnnActivationDescriptor_t,
    normMeanVarDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    mode: cuda_types::cudnn9::cudnnNormMode_t,
    normOps: cuda_types::cudnn9::cudnnNormOps_t,
    algo: cuda_types::cudnn9::cudnnNormAlgo_t,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    yDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dyDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dzDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dxDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dNormScaleBiasDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    activationDesc: cuda_types::cudnn9::cudnnActivationDescriptor_t,
    normMeanVarDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    mode: cuda_types::cudnn9::cudnnNormMode_t,
    normOps: cuda_types::cudnn9::cudnnNormOps_t,
    algo: cuda_types::cudnn9::cudnnNormAlgo_t,
    activationDesc: cuda_types::cudnn9::cudnnActivationDescriptor_t,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    mode: cuda_types::cudnn9::cudnnNormMode_t,
    normOps: cuda_types::cudnn9::cudnnNormOps_t,
    algo: cuda_types::cudnn9::cudnnNormAlgo_t,
    alpha: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    xData: *const ::core::ffi::c_void,
    normScaleBiasDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    normScale: *const ::core::ffi::c_void,
    normBias: *const ::core::ffi::c_void,
    exponentialAverageFactor: f64,
    normMeanVarDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    resultRunningMean: *mut ::core::ffi::c_void,
    resultRunningVariance: *mut ::core::ffi::c_void,
    epsilon: f64,
    resultSaveMean: *mut ::core::ffi::c_void,
    resultSaveInvVariance: *mut ::core::ffi::c_void,
    activationDesc: cuda_types::cudnn9::cudnnActivationDescriptor_t,
    zDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    zData: *const ::core::ffi::c_void,
    yDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    mode: cuda_types::cudnn9::cudnnNormMode_t,
    normOps: cuda_types::cudnn9::cudnnNormOps_t,
    algo: cuda_types::cudnn9::cudnnNormAlgo_t,
    alphaDataDiff: *const ::core::ffi::c_void,
    betaDataDiff: *const ::core::ffi::c_void,
    alphaParamDiff: *const ::core::ffi::c_void,
    betaParamDiff: *const ::core::ffi::c_void,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    xData: *const ::core::ffi::c_void,
    yDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    yData: *const ::core::ffi::c_void,
    dyDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dyData: *const ::core::ffi::c_void,
    dzDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dzData: *mut ::core::ffi::c_void,
    dxDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dxData: *mut ::core::ffi::c_void,
    dNormScaleBiasDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    normScaleData: *const ::core::ffi::c_void,
    normBiasData: *const ::core::ffi::c_void,
    dNormScaleData: *mut ::core::ffi::c_void,
    dNormBiasData: *mut ::core::ffi::c_void,
    epsilon: f64,
    normMeanVarDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    savedMean: *const ::core::ffi::c_void,
    savedInvVariance: *const ::core::ffi::c_void,
    activationDesc: cuda_types::cudnn9::cudnnActivationDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    stDesc: cuda_types::cudnn9::cudnnSpatialTransformerDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    stDesc: cuda_types::cudnn9::cudnnSpatialTransformerDescriptor_t,
    alpha: *const ::core::ffi::c_void,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    dxDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dx: *mut ::core::ffi::c_void,
    alphaDgrid: *const ::core::ffi::c_void,
    dyDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    dropoutDesc: cuda_types::cudnn9::cudnnDropoutDescriptor_t,
    dydesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dy: *const ::core::ffi::c_void,
    dxdesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnRNNAlgo_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnRNNAlgo_t::CUDNN_RNN_ALGO_STANDARD => {
                writer.write_all(stringify!(CUDNN_RNN_ALGO_STANDARD).as_bytes())
            }
            &cuda_types::cudnn9::cudnnRNNAlgo_t::CUDNN_RNN_ALGO_PERSIST_STATIC => {
                writer.write_all(stringify!(CUDNN_RNN_ALGO_PERSIST_STATIC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnRNNAlgo_t::CUDNN_RNN_ALGO_PERSIST_DYNAMIC => {
                writer.write_all(stringify!(CUDNN_RNN_ALGO_PERSIST_DYNAMIC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnRNNAlgo_t::CUDNN_RNN_ALGO_PERSIST_STATIC_SMALL_H => {
                writer
                    .write_all(
                        stringify!(CUDNN_RNN_ALGO_PERSIST_STATIC_SMALL_H).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnRNNAlgo_t::CUDNN_RNN_ALGO_COUNT => {
                writer.write_all(stringify!(CUDNN_RNN_ALGO_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnForwardMode_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnForwardMode_t::CUDNN_FWD_MODE_INFERENCE => {
                writer.write_all(stringify!(CUDNN_FWD_MODE_INFERENCE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnForwardMode_t::CUDNN_FWD_MODE_TRAINING => {
                writer.write_all(stringify!(CUDNN_FWD_MODE_TRAINING).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnRNNMode_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnRNNMode_t::CUDNN_RNN_RELU => {
                writer.write_all(stringify!(CUDNN_RNN_RELU).as_bytes())
            }
            &cuda_types::cudnn9::cudnnRNNMode_t::CUDNN_RNN_TANH => {
                writer.write_all(stringify!(CUDNN_RNN_TANH).as_bytes())
            }
            &cuda_types::cudnn9::cudnnRNNMode_t::CUDNN_LSTM => {
                writer.write_all(stringify!(CUDNN_LSTM).as_bytes())
            }
            &cuda_types::cudnn9::cudnnRNNMode_t::CUDNN_GRU => {
                writer.write_all(stringify!(CUDNN_GRU).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnRNNBiasMode_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnRNNBiasMode_t::CUDNN_RNN_NO_BIAS => {
                writer.write_all(stringify!(CUDNN_RNN_NO_BIAS).as_bytes())
            }
            &cuda_types::cudnn9::cudnnRNNBiasMode_t::CUDNN_RNN_SINGLE_INP_BIAS => {
                writer.write_all(stringify!(CUDNN_RNN_SINGLE_INP_BIAS).as_bytes())
            }
            &cuda_types::cudnn9::cudnnRNNBiasMode_t::CUDNN_RNN_DOUBLE_BIAS => {
                writer.write_all(stringify!(CUDNN_RNN_DOUBLE_BIAS).as_bytes())
            }
            &cuda_types::cudnn9::cudnnRNNBiasMode_t::CUDNN_RNN_SINGLE_REC_BIAS => {
                writer.write_all(stringify!(CUDNN_RNN_SINGLE_REC_BIAS).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnDirectionMode_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnDirectionMode_t::CUDNN_UNIDIRECTIONAL => {
                writer.write_all(stringify!(CUDNN_UNIDIRECTIONAL).as_bytes())
            }
            &cuda_types::cudnn9::cudnnDirectionMode_t::CUDNN_BIDIRECTIONAL => {
                writer.write_all(stringify!(CUDNN_BIDIRECTIONAL).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnRNNInputMode_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnRNNInputMode_t::CUDNN_LINEAR_INPUT => {
                writer.write_all(stringify!(CUDNN_LINEAR_INPUT).as_bytes())
            }
            &cuda_types::cudnn9::cudnnRNNInputMode_t::CUDNN_SKIP_INPUT => {
                writer.write_all(stringify!(CUDNN_SKIP_INPUT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnRNNClipMode_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnRNNClipMode_t::CUDNN_RNN_CLIP_NONE => {
                writer.write_all(stringify!(CUDNN_RNN_CLIP_NONE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnRNNClipMode_t::CUDNN_RNN_CLIP_MINMAX => {
                writer.write_all(stringify!(CUDNN_RNN_CLIP_MINMAX).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnRNNDataLayout_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnRNNDataLayout_t::CUDNN_RNN_DATA_LAYOUT_SEQ_MAJOR_UNPACKED => {
                writer
                    .write_all(
                        stringify!(CUDNN_RNN_DATA_LAYOUT_SEQ_MAJOR_UNPACKED).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnRNNDataLayout_t::CUDNN_RNN_DATA_LAYOUT_SEQ_MAJOR_PACKED => {
                writer
                    .write_all(
                        stringify!(CUDNN_RNN_DATA_LAYOUT_SEQ_MAJOR_PACKED).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnRNNDataLayout_t::CUDNN_RNN_DATA_LAYOUT_BATCH_MAJOR_UNPACKED => {
                writer
                    .write_all(
                        stringify!(CUDNN_RNN_DATA_LAYOUT_BATCH_MAJOR_UNPACKED).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnRNNDescriptor_t {
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnRNNDataDescriptor_t {
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
    rnnDesc: *mut cuda_types::cudnn9::cudnnRNNDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnCreateRNNDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnDestroyRNNDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    rnnDesc: cuda_types::cudnn9::cudnnRNNDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnDestroyRNNDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnSetRNNDescriptor_v8(
    writer: &mut (impl std::io::Write + ?Sized),
    rnnDesc: cuda_types::cudnn9::cudnnRNNDescriptor_t,
    algo: cuda_types::cudnn9::cudnnRNNAlgo_t,
    cellMode: cuda_types::cudnn9::cudnnRNNMode_t,
    biasMode: cuda_types::cudnn9::cudnnRNNBiasMode_t,
    dirMode: cuda_types::cudnn9::cudnnDirectionMode_t,
    inputMode: cuda_types::cudnn9::cudnnRNNInputMode_t,
    dataType: cuda_types::cudnn9::cudnnDataType_t,
    mathPrec: cuda_types::cudnn9::cudnnDataType_t,
    mathType: cuda_types::cudnn9::cudnnMathType_t,
    inputSize: i32,
    hiddenSize: i32,
    projSize: i32,
    numLayers: i32,
    dropoutDesc: cuda_types::cudnn9::cudnnDropoutDescriptor_t,
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
    rnnDesc: cuda_types::cudnn9::cudnnRNNDescriptor_t,
    algo: *mut cuda_types::cudnn9::cudnnRNNAlgo_t,
    cellMode: *mut cuda_types::cudnn9::cudnnRNNMode_t,
    biasMode: *mut cuda_types::cudnn9::cudnnRNNBiasMode_t,
    dirMode: *mut cuda_types::cudnn9::cudnnDirectionMode_t,
    inputMode: *mut cuda_types::cudnn9::cudnnRNNInputMode_t,
    dataType: *mut cuda_types::cudnn9::cudnnDataType_t,
    mathPrec: *mut cuda_types::cudnn9::cudnnDataType_t,
    mathType: *mut cuda_types::cudnn9::cudnnMathType_t,
    inputSize: *mut i32,
    hiddenSize: *mut i32,
    projSize: *mut i32,
    numLayers: *mut i32,
    dropoutDesc: *mut cuda_types::cudnn9::cudnnDropoutDescriptor_t,
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
pub fn write_cudnnRNNSetClip_v8(
    writer: &mut (impl std::io::Write + ?Sized),
    rnnDesc: cuda_types::cudnn9::cudnnRNNDescriptor_t,
    clipMode: cuda_types::cudnn9::cudnnRNNClipMode_t,
    clipNanOpt: cuda_types::cudnn9::cudnnNanPropagation_t,
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
pub fn write_cudnnRNNSetClip_v9(
    writer: &mut (impl std::io::Write + ?Sized),
    rnnDesc: cuda_types::cudnn9::cudnnRNNDescriptor_t,
    clipMode: cuda_types::cudnn9::cudnnRNNClipMode_t,
    lclip: f64,
    rclip: f64,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnRNNSetClip_v9", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(clipMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&clipMode, "cudnnRNNSetClip_v9", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(lclip), ": ").as_bytes())?;
    crate::CudaDisplay::write(&lclip, "cudnnRNNSetClip_v9", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rclip), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rclip, "cudnnRNNSetClip_v9", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnRNNGetClip_v8(
    writer: &mut (impl std::io::Write + ?Sized),
    rnnDesc: cuda_types::cudnn9::cudnnRNNDescriptor_t,
    clipMode: *mut cuda_types::cudnn9::cudnnRNNClipMode_t,
    clipNanOpt: *mut cuda_types::cudnn9::cudnnNanPropagation_t,
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
pub fn write_cudnnRNNGetClip_v9(
    writer: &mut (impl std::io::Write + ?Sized),
    rnnDesc: cuda_types::cudnn9::cudnnRNNDescriptor_t,
    clipMode: *mut cuda_types::cudnn9::cudnnRNNClipMode_t,
    lclip: *mut f64,
    rclip: *mut f64,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(rnnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rnnDesc, "cudnnRNNGetClip_v9", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(clipMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&clipMode, "cudnnRNNGetClip_v9", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(lclip), ": ").as_bytes())?;
    crate::CudaDisplay::write(&lclip, "cudnnRNNGetClip_v9", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rclip), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rclip, "cudnnRNNGetClip_v9", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnBuildRNNDynamic(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn9::cudnnHandle_t,
    rnnDesc: cuda_types::cudnn9::cudnnRNNDescriptor_t,
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
pub fn write_cudnnGetRNNTempSpaceSizes(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn9::cudnnHandle_t,
    rnnDesc: cuda_types::cudnn9::cudnnRNNDescriptor_t,
    fwdMode: cuda_types::cudnn9::cudnnForwardMode_t,
    xDesc: cuda_types::cudnn9::cudnnRNNDataDescriptor_t,
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
pub fn write_cudnnGetRNNWeightSpaceSize(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn9::cudnnHandle_t,
    rnnDesc: cuda_types::cudnn9::cudnnRNNDescriptor_t,
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
pub fn write_cudnnGetRNNWeightParams(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn9::cudnnHandle_t,
    rnnDesc: cuda_types::cudnn9::cudnnRNNDescriptor_t,
    pseudoLayer: i32,
    weightSpaceSize: usize,
    weightSpace: *const ::core::ffi::c_void,
    linLayerID: i32,
    mDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    mAddr: *mut *mut ::core::ffi::c_void,
    bDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
pub fn write_cudnnCreateRNNDataDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    rnnDataDesc: *mut cuda_types::cudnn9::cudnnRNNDataDescriptor_t,
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
    rnnDataDesc: cuda_types::cudnn9::cudnnRNNDataDescriptor_t,
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
    rnnDataDesc: cuda_types::cudnn9::cudnnRNNDataDescriptor_t,
    dataType: cuda_types::cudnn9::cudnnDataType_t,
    layout: cuda_types::cudnn9::cudnnRNNDataLayout_t,
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
    rnnDataDesc: cuda_types::cudnn9::cudnnRNNDataDescriptor_t,
    dataType: *mut cuda_types::cudnn9::cudnnDataType_t,
    layout: *mut cuda_types::cudnn9::cudnnRNNDataLayout_t,
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
pub fn write_cudnnRNNForward(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn9::cudnnHandle_t,
    rnnDesc: cuda_types::cudnn9::cudnnRNNDescriptor_t,
    fwdMode: cuda_types::cudnn9::cudnnForwardMode_t,
    devSeqLengths: *const i32,
    xDesc: cuda_types::cudnn9::cudnnRNNDataDescriptor_t,
    x: *const ::core::ffi::c_void,
    yDesc: cuda_types::cudnn9::cudnnRNNDataDescriptor_t,
    y: *mut ::core::ffi::c_void,
    hDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    hx: *const ::core::ffi::c_void,
    hy: *mut ::core::ffi::c_void,
    cDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnSeqDataAxis_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnSeqDataAxis_t::CUDNN_SEQDATA_TIME_DIM => {
                writer.write_all(stringify!(CUDNN_SEQDATA_TIME_DIM).as_bytes())
            }
            &cuda_types::cudnn9::cudnnSeqDataAxis_t::CUDNN_SEQDATA_BATCH_DIM => {
                writer.write_all(stringify!(CUDNN_SEQDATA_BATCH_DIM).as_bytes())
            }
            &cuda_types::cudnn9::cudnnSeqDataAxis_t::CUDNN_SEQDATA_BEAM_DIM => {
                writer.write_all(stringify!(CUDNN_SEQDATA_BEAM_DIM).as_bytes())
            }
            &cuda_types::cudnn9::cudnnSeqDataAxis_t::CUDNN_SEQDATA_VECT_DIM => {
                writer.write_all(stringify!(CUDNN_SEQDATA_VECT_DIM).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnSeqDataDescriptor_t {
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
pub fn write_cudnnCreateSeqDataDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    seqDataDesc: *mut cuda_types::cudnn9::cudnnSeqDataDescriptor_t,
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
    seqDataDesc: cuda_types::cudnn9::cudnnSeqDataDescriptor_t,
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
    seqDataDesc: cuda_types::cudnn9::cudnnSeqDataDescriptor_t,
    dataType: cuda_types::cudnn9::cudnnDataType_t,
    nbDims: ::core::ffi::c_int,
    dimA: *const ::core::ffi::c_int,
    axes: *const cuda_types::cudnn9::cudnnSeqDataAxis_t,
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
    seqDataDesc: cuda_types::cudnn9::cudnnSeqDataDescriptor_t,
    dataType: *mut cuda_types::cudnn9::cudnnDataType_t,
    nbDims: *mut ::core::ffi::c_int,
    nbDimsRequested: ::core::ffi::c_int,
    dimA: *mut ::core::ffi::c_int,
    axes: *mut cuda_types::cudnn9::cudnnSeqDataAxis_t,
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnAttnDescriptor_t {
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
pub fn write_cudnnCreateAttnDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    attnDesc: *mut cuda_types::cudnn9::cudnnAttnDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(attnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attnDesc, "cudnnCreateAttnDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnDestroyAttnDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    attnDesc: cuda_types::cudnn9::cudnnAttnDescriptor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(attnDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attnDesc, "cudnnDestroyAttnDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnSetAttnDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    attnDesc: cuda_types::cudnn9::cudnnAttnDescriptor_t,
    attnMode: ::core::ffi::c_uint,
    nHeads: ::core::ffi::c_int,
    smScaler: f64,
    dataType: cuda_types::cudnn9::cudnnDataType_t,
    computePrec: cuda_types::cudnn9::cudnnDataType_t,
    mathType: cuda_types::cudnn9::cudnnMathType_t,
    attnDropoutDesc: cuda_types::cudnn9::cudnnDropoutDescriptor_t,
    postDropoutDesc: cuda_types::cudnn9::cudnnDropoutDescriptor_t,
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
    attnDesc: cuda_types::cudnn9::cudnnAttnDescriptor_t,
    attnMode: *mut ::core::ffi::c_uint,
    nHeads: *mut ::core::ffi::c_int,
    smScaler: *mut f64,
    dataType: *mut cuda_types::cudnn9::cudnnDataType_t,
    computePrec: *mut cuda_types::cudnn9::cudnnDataType_t,
    mathType: *mut cuda_types::cudnn9::cudnnMathType_t,
    attnDropoutDesc: *mut cuda_types::cudnn9::cudnnDropoutDescriptor_t,
    postDropoutDesc: *mut cuda_types::cudnn9::cudnnDropoutDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    attnDesc: cuda_types::cudnn9::cudnnAttnDescriptor_t,
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnMultiHeadAttnWeightKind_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnMultiHeadAttnWeightKind_t::CUDNN_MH_ATTN_Q_WEIGHTS => {
                writer.write_all(stringify!(CUDNN_MH_ATTN_Q_WEIGHTS).as_bytes())
            }
            &cuda_types::cudnn9::cudnnMultiHeadAttnWeightKind_t::CUDNN_MH_ATTN_K_WEIGHTS => {
                writer.write_all(stringify!(CUDNN_MH_ATTN_K_WEIGHTS).as_bytes())
            }
            &cuda_types::cudnn9::cudnnMultiHeadAttnWeightKind_t::CUDNN_MH_ATTN_V_WEIGHTS => {
                writer.write_all(stringify!(CUDNN_MH_ATTN_V_WEIGHTS).as_bytes())
            }
            &cuda_types::cudnn9::cudnnMultiHeadAttnWeightKind_t::CUDNN_MH_ATTN_O_WEIGHTS => {
                writer.write_all(stringify!(CUDNN_MH_ATTN_O_WEIGHTS).as_bytes())
            }
            &cuda_types::cudnn9::cudnnMultiHeadAttnWeightKind_t::CUDNN_MH_ATTN_Q_BIASES => {
                writer.write_all(stringify!(CUDNN_MH_ATTN_Q_BIASES).as_bytes())
            }
            &cuda_types::cudnn9::cudnnMultiHeadAttnWeightKind_t::CUDNN_MH_ATTN_K_BIASES => {
                writer.write_all(stringify!(CUDNN_MH_ATTN_K_BIASES).as_bytes())
            }
            &cuda_types::cudnn9::cudnnMultiHeadAttnWeightKind_t::CUDNN_MH_ATTN_V_BIASES => {
                writer.write_all(stringify!(CUDNN_MH_ATTN_V_BIASES).as_bytes())
            }
            &cuda_types::cudnn9::cudnnMultiHeadAttnWeightKind_t::CUDNN_MH_ATTN_O_BIASES => {
                writer.write_all(stringify!(CUDNN_MH_ATTN_O_BIASES).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cudnnGetMultiHeadAttnWeights(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn9::cudnnHandle_t,
    attnDesc: cuda_types::cudnn9::cudnnAttnDescriptor_t,
    wKind: cuda_types::cudnn9::cudnnMultiHeadAttnWeightKind_t,
    weightSizeInBytes: usize,
    weights: *const ::core::ffi::c_void,
    wDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    attnDesc: cuda_types::cudnn9::cudnnAttnDescriptor_t,
    currIdx: ::core::ffi::c_int,
    loWinIdx: *const ::core::ffi::c_int,
    hiWinIdx: *const ::core::ffi::c_int,
    devSeqLengthsQO: *const ::core::ffi::c_int,
    devSeqLengthsKV: *const ::core::ffi::c_int,
    qDesc: cuda_types::cudnn9::cudnnSeqDataDescriptor_t,
    queries: *const ::core::ffi::c_void,
    residuals: *const ::core::ffi::c_void,
    kDesc: cuda_types::cudnn9::cudnnSeqDataDescriptor_t,
    keys: *const ::core::ffi::c_void,
    vDesc: cuda_types::cudnn9::cudnnSeqDataDescriptor_t,
    values: *const ::core::ffi::c_void,
    oDesc: cuda_types::cudnn9::cudnnSeqDataDescriptor_t,
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
pub fn write_cudnnAdvVersionCheck(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnWgradMode_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnWgradMode_t::CUDNN_WGRAD_MODE_ADD => {
                writer.write_all(stringify!(CUDNN_WGRAD_MODE_ADD).as_bytes())
            }
            &cuda_types::cudnn9::cudnnWgradMode_t::CUDNN_WGRAD_MODE_SET => {
                writer.write_all(stringify!(CUDNN_WGRAD_MODE_SET).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cudnnRNNBackwardData_v8(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn9::cudnnHandle_t,
    rnnDesc: cuda_types::cudnn9::cudnnRNNDescriptor_t,
    devSeqLengths: *const i32,
    yDesc: cuda_types::cudnn9::cudnnRNNDataDescriptor_t,
    y: *const ::core::ffi::c_void,
    dy: *const ::core::ffi::c_void,
    xDesc: cuda_types::cudnn9::cudnnRNNDataDescriptor_t,
    dx: *mut ::core::ffi::c_void,
    hDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    hx: *const ::core::ffi::c_void,
    dhy: *const ::core::ffi::c_void,
    dhx: *mut ::core::ffi::c_void,
    cDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
pub fn write_cudnnRNNBackwardWeights_v8(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn9::cudnnHandle_t,
    rnnDesc: cuda_types::cudnn9::cudnnRNNDescriptor_t,
    addGrad: cuda_types::cudnn9::cudnnWgradMode_t,
    devSeqLengths: *const i32,
    xDesc: cuda_types::cudnn9::cudnnRNNDataDescriptor_t,
    x: *const ::core::ffi::c_void,
    hDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    hx: *const ::core::ffi::c_void,
    yDesc: cuda_types::cudnn9::cudnnRNNDataDescriptor_t,
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
pub fn write_cudnnMultiHeadAttnBackwardData(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn9::cudnnHandle_t,
    attnDesc: cuda_types::cudnn9::cudnnAttnDescriptor_t,
    loWinIdx: *const ::core::ffi::c_int,
    hiWinIdx: *const ::core::ffi::c_int,
    devSeqLengthsDQDO: *const ::core::ffi::c_int,
    devSeqLengthsDKDV: *const ::core::ffi::c_int,
    doDesc: cuda_types::cudnn9::cudnnSeqDataDescriptor_t,
    dout: *const ::core::ffi::c_void,
    dqDesc: cuda_types::cudnn9::cudnnSeqDataDescriptor_t,
    dqueries: *mut ::core::ffi::c_void,
    queries: *const ::core::ffi::c_void,
    dkDesc: cuda_types::cudnn9::cudnnSeqDataDescriptor_t,
    dkeys: *mut ::core::ffi::c_void,
    keys: *const ::core::ffi::c_void,
    dvDesc: cuda_types::cudnn9::cudnnSeqDataDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    attnDesc: cuda_types::cudnn9::cudnnAttnDescriptor_t,
    addGrad: cuda_types::cudnn9::cudnnWgradMode_t,
    qDesc: cuda_types::cudnn9::cudnnSeqDataDescriptor_t,
    queries: *const ::core::ffi::c_void,
    kDesc: cuda_types::cudnn9::cudnnSeqDataDescriptor_t,
    keys: *const ::core::ffi::c_void,
    vDesc: cuda_types::cudnn9::cudnnSeqDataDescriptor_t,
    values: *const ::core::ffi::c_void,
    doDesc: cuda_types::cudnn9::cudnnSeqDataDescriptor_t,
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnLossNormalizationMode_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnLossNormalizationMode_t::CUDNN_LOSS_NORMALIZATION_NONE => {
                writer.write_all(stringify!(CUDNN_LOSS_NORMALIZATION_NONE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnLossNormalizationMode_t::CUDNN_LOSS_NORMALIZATION_SOFTMAX => {
                writer.write_all(stringify!(CUDNN_LOSS_NORMALIZATION_SOFTMAX).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cudnnCreateCTCLossDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    ctcLossDesc: *mut cuda_types::cudnn9::cudnnCTCLossDescriptor_t,
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
    ctcLossDesc: cuda_types::cudnn9::cudnnCTCLossDescriptor_t,
    compType: cuda_types::cudnn9::cudnnDataType_t,
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
    ctcLossDesc: cuda_types::cudnn9::cudnnCTCLossDescriptor_t,
    compType: cuda_types::cudnn9::cudnnDataType_t,
    normMode: cuda_types::cudnn9::cudnnLossNormalizationMode_t,
    gradMode: cuda_types::cudnn9::cudnnNanPropagation_t,
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
    ctcLossDesc: cuda_types::cudnn9::cudnnCTCLossDescriptor_t,
    compType: cuda_types::cudnn9::cudnnDataType_t,
    normMode: cuda_types::cudnn9::cudnnLossNormalizationMode_t,
    gradMode: cuda_types::cudnn9::cudnnNanPropagation_t,
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
pub fn write_cudnnSetCTCLossDescriptor_v9(
    writer: &mut (impl std::io::Write + ?Sized),
    ctcLossDesc: cuda_types::cudnn9::cudnnCTCLossDescriptor_t,
    compType: cuda_types::cudnn9::cudnnDataType_t,
    normMode: cuda_types::cudnn9::cudnnLossNormalizationMode_t,
    ctcGradMode: cuda_types::cudnn9::cudnnCTCGradMode_t,
    maxLabelLength: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(ctcLossDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &ctcLossDesc,
        "cudnnSetCTCLossDescriptor_v9",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(compType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &compType,
        "cudnnSetCTCLossDescriptor_v9",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(normMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &normMode,
        "cudnnSetCTCLossDescriptor_v9",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ctcGradMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &ctcGradMode,
        "cudnnSetCTCLossDescriptor_v9",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(maxLabelLength), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &maxLabelLength,
        "cudnnSetCTCLossDescriptor_v9",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnGetCTCLossDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    ctcLossDesc: cuda_types::cudnn9::cudnnCTCLossDescriptor_t,
    compType: *mut cuda_types::cudnn9::cudnnDataType_t,
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
    ctcLossDesc: cuda_types::cudnn9::cudnnCTCLossDescriptor_t,
    compType: *mut cuda_types::cudnn9::cudnnDataType_t,
    normMode: *mut cuda_types::cudnn9::cudnnLossNormalizationMode_t,
    gradMode: *mut cuda_types::cudnn9::cudnnNanPropagation_t,
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
    ctcLossDesc: cuda_types::cudnn9::cudnnCTCLossDescriptor_t,
    compType: *mut cuda_types::cudnn9::cudnnDataType_t,
    normMode: *mut cuda_types::cudnn9::cudnnLossNormalizationMode_t,
    gradMode: *mut cuda_types::cudnn9::cudnnNanPropagation_t,
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
pub fn write_cudnnGetCTCLossDescriptor_v9(
    writer: &mut (impl std::io::Write + ?Sized),
    ctcLossDesc: cuda_types::cudnn9::cudnnCTCLossDescriptor_t,
    compType: *mut cuda_types::cudnn9::cudnnDataType_t,
    normMode: *mut cuda_types::cudnn9::cudnnLossNormalizationMode_t,
    ctcGradMode: *mut cuda_types::cudnn9::cudnnCTCGradMode_t,
    maxLabelLength: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(ctcLossDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &ctcLossDesc,
        "cudnnGetCTCLossDescriptor_v9",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(compType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &compType,
        "cudnnGetCTCLossDescriptor_v9",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(normMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &normMode,
        "cudnnGetCTCLossDescriptor_v9",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ctcGradMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &ctcGradMode,
        "cudnnGetCTCLossDescriptor_v9",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(maxLabelLength), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &maxLabelLength,
        "cudnnGetCTCLossDescriptor_v9",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cudnnDestroyCTCLossDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    ctcLossDesc: cuda_types::cudnn9::cudnnCTCLossDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    probsDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    probs: *const ::core::ffi::c_void,
    hostLabels: *const ::core::ffi::c_int,
    hostLabelLengths: *const ::core::ffi::c_int,
    hostInputLengths: *const ::core::ffi::c_int,
    costs: *mut ::core::ffi::c_void,
    gradientsDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    gradients: *mut ::core::ffi::c_void,
    algo: cuda_types::cudnn9::cudnnCTCLossAlgo_t,
    ctcLossDesc: cuda_types::cudnn9::cudnnCTCLossDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    algo: cuda_types::cudnn9::cudnnCTCLossAlgo_t,
    ctcLossDesc: cuda_types::cudnn9::cudnnCTCLossDescriptor_t,
    probsDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    probs: *const ::core::ffi::c_void,
    labels: *const ::core::ffi::c_int,
    labelLengths: *const ::core::ffi::c_int,
    inputLengths: *const ::core::ffi::c_int,
    costs: *mut ::core::ffi::c_void,
    gradientsDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    probsDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    gradientsDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    labels: *const ::core::ffi::c_int,
    labelLengths: *const ::core::ffi::c_int,
    inputLengths: *const ::core::ffi::c_int,
    algo: cuda_types::cudnn9::cudnnCTCLossAlgo_t,
    ctcLossDesc: cuda_types::cudnn9::cudnnCTCLossDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    algo: cuda_types::cudnn9::cudnnCTCLossAlgo_t,
    ctcLossDesc: cuda_types::cudnn9::cudnnCTCLossDescriptor_t,
    probsDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    gradientsDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnConvolutionDescriptor_t {
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnConvolutionFwdAlgoPerfStruct {
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
    convDesc: *mut cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
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
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
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
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
    mathType: cuda_types::cudnn9::cudnnMathType_t,
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
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
    mathType: *mut cuda_types::cudnn9::cudnnMathType_t,
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
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
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
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
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
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
    reorderType: cuda_types::cudnn9::cudnnReorderType_t,
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
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
    reorderType: *mut cuda_types::cudnn9::cudnnReorderType_t,
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
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
    pad_h: ::core::ffi::c_int,
    pad_w: ::core::ffi::c_int,
    u: ::core::ffi::c_int,
    v: ::core::ffi::c_int,
    dilation_h: ::core::ffi::c_int,
    dilation_w: ::core::ffi::c_int,
    mode: cuda_types::cudnn9::cudnnConvolutionMode_t,
    computeType: cuda_types::cudnn9::cudnnDataType_t,
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
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
    pad_h: *mut ::core::ffi::c_int,
    pad_w: *mut ::core::ffi::c_int,
    u: *mut ::core::ffi::c_int,
    v: *mut ::core::ffi::c_int,
    dilation_h: *mut ::core::ffi::c_int,
    dilation_w: *mut ::core::ffi::c_int,
    mode: *mut cuda_types::cudnn9::cudnnConvolutionMode_t,
    computeType: *mut cuda_types::cudnn9::cudnnDataType_t,
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
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
    arrayLength: ::core::ffi::c_int,
    padA: *const ::core::ffi::c_int,
    filterStrideA: *const ::core::ffi::c_int,
    dilationA: *const ::core::ffi::c_int,
    mode: cuda_types::cudnn9::cudnnConvolutionMode_t,
    computeType: cuda_types::cudnn9::cudnnDataType_t,
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
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
    arrayLengthRequested: ::core::ffi::c_int,
    arrayLength: *mut ::core::ffi::c_int,
    padA: *mut ::core::ffi::c_int,
    strideA: *mut ::core::ffi::c_int,
    dilationA: *mut ::core::ffi::c_int,
    mode: *mut cuda_types::cudnn9::cudnnConvolutionMode_t,
    computeType: *mut cuda_types::cudnn9::cudnnDataType_t,
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
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
    inputTensorDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    filterDesc: cuda_types::cudnn9::cudnnFilterDescriptor_t,
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
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
    inputTensorDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    filterDesc: cuda_types::cudnn9::cudnnFilterDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    srcDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    filterDesc: cuda_types::cudnn9::cudnnFilterDescriptor_t,
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
    destDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    requestedAlgoCount: ::core::ffi::c_int,
    returnedAlgoCount: *mut ::core::ffi::c_int,
    perfResults: *mut cuda_types::cudnn9::cudnnConvolutionFwdAlgoPerf_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    wDesc: cuda_types::cudnn9::cudnnFilterDescriptor_t,
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
    yDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    requestedAlgoCount: ::core::ffi::c_int,
    returnedAlgoCount: *mut ::core::ffi::c_int,
    perfResults: *mut cuda_types::cudnn9::cudnnConvolutionFwdAlgoPerf_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    wDesc: cuda_types::cudnn9::cudnnFilterDescriptor_t,
    w: *const ::core::ffi::c_void,
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
    yDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    y: *mut ::core::ffi::c_void,
    requestedAlgoCount: ::core::ffi::c_int,
    returnedAlgoCount: *mut ::core::ffi::c_int,
    perfResults: *mut cuda_types::cudnn9::cudnnConvolutionFwdAlgoPerf_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    wDesc: cuda_types::cudnn9::cudnnFilterDescriptor_t,
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    filterDesc: cuda_types::cudnn9::cudnnFilterDescriptor_t,
    reorderType: cuda_types::cudnn9::cudnnReorderType_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    wDesc: cuda_types::cudnn9::cudnnFilterDescriptor_t,
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
    yDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    algo: cuda_types::cudnn9::cudnnConvolutionFwdAlgo_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    alpha: *const ::core::ffi::c_void,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    wDesc: cuda_types::cudnn9::cudnnFilterDescriptor_t,
    w: *const ::core::ffi::c_void,
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
    algo: cuda_types::cudnn9::cudnnConvolutionFwdAlgo_t,
    workSpace: *mut ::core::ffi::c_void,
    workSpaceSizeInBytes: usize,
    beta: *const ::core::ffi::c_void,
    yDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    alpha1: *const ::core::ffi::c_void,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    wDesc: cuda_types::cudnn9::cudnnFilterDescriptor_t,
    w: *const ::core::ffi::c_void,
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
    algo: cuda_types::cudnn9::cudnnConvolutionFwdAlgo_t,
    workSpace: *mut ::core::ffi::c_void,
    workSpaceSizeInBytes: usize,
    alpha2: *const ::core::ffi::c_void,
    zDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    z: *const ::core::ffi::c_void,
    biasDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    bias: *const ::core::ffi::c_void,
    activationDesc: cuda_types::cudnn9::cudnnActivationDescriptor_t,
    yDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnConvolutionBwdDataAlgoPerfStruct {
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    wDesc: cuda_types::cudnn9::cudnnFilterDescriptor_t,
    dyDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
    dxDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    requestedAlgoCount: ::core::ffi::c_int,
    returnedAlgoCount: *mut ::core::ffi::c_int,
    perfResults: *mut cuda_types::cudnn9::cudnnConvolutionBwdDataAlgoPerf_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    wDesc: cuda_types::cudnn9::cudnnFilterDescriptor_t,
    w: *const ::core::ffi::c_void,
    dyDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dy: *const ::core::ffi::c_void,
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
    dxDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dx: *mut ::core::ffi::c_void,
    requestedAlgoCount: ::core::ffi::c_int,
    returnedAlgoCount: *mut ::core::ffi::c_int,
    perfResults: *mut cuda_types::cudnn9::cudnnConvolutionBwdDataAlgoPerf_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    filterDesc: cuda_types::cudnn9::cudnnFilterDescriptor_t,
    diffDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
    gradDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    requestedAlgoCount: ::core::ffi::c_int,
    returnedAlgoCount: *mut ::core::ffi::c_int,
    perfResults: *mut cuda_types::cudnn9::cudnnConvolutionBwdDataAlgoPerf_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    wDesc: cuda_types::cudnn9::cudnnFilterDescriptor_t,
    dyDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
    dxDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    algo: cuda_types::cudnn9::cudnnConvolutionBwdDataAlgo_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    alpha: *const ::core::ffi::c_void,
    wDesc: cuda_types::cudnn9::cudnnFilterDescriptor_t,
    w: *const ::core::ffi::c_void,
    dyDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dy: *const ::core::ffi::c_void,
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
    algo: cuda_types::cudnn9::cudnnConvolutionBwdDataAlgo_t,
    workSpace: *mut ::core::ffi::c_void,
    workSpaceSizeInBytes: usize,
    beta: *const ::core::ffi::c_void,
    dxDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    filterDesc: cuda_types::cudnn9::cudnnFilterDescriptor_t,
    diffDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
    gradDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    transformFormat: cuda_types::cudnn9::cudnnTensorFormat_t,
    foldedFilterDesc: cuda_types::cudnn9::cudnnFilterDescriptor_t,
    paddedDiffDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    foldedConvDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
    foldedGradDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    filterFoldTransDesc: cuda_types::cudnn9::cudnnTensorTransformDescriptor_t,
    diffPadTransDesc: cuda_types::cudnn9::cudnnTensorTransformDescriptor_t,
    gradFoldTransDesc: cuda_types::cudnn9::cudnnTensorTransformDescriptor_t,
    gradUnfoldTransDesc: cuda_types::cudnn9::cudnnTensorTransformDescriptor_t,
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnFusedOpsConstParamPack_t {
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnFusedOpsVariantParamPack_t {
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnFusedOpsPlan_t {
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnFusedOps_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnFusedOps_t::CUDNN_FUSED_SCALE_BIAS_ACTIVATION_CONV_BNSTATS => {
                writer
                    .write_all(
                        stringify!(CUDNN_FUSED_SCALE_BIAS_ACTIVATION_CONV_BNSTATS)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnFusedOps_t::CUDNN_FUSED_SCALE_BIAS_ACTIVATION_WGRAD => {
                writer
                    .write_all(
                        stringify!(CUDNN_FUSED_SCALE_BIAS_ACTIVATION_WGRAD).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnFusedOps_t::CUDNN_FUSED_BN_FINALIZE_STATISTICS_TRAINING => {
                writer
                    .write_all(
                        stringify!(CUDNN_FUSED_BN_FINALIZE_STATISTICS_TRAINING)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnFusedOps_t::CUDNN_FUSED_BN_FINALIZE_STATISTICS_INFERENCE => {
                writer
                    .write_all(
                        stringify!(CUDNN_FUSED_BN_FINALIZE_STATISTICS_INFERENCE)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnFusedOps_t::CUDNN_FUSED_CONV_SCALE_BIAS_ADD_ACTIVATION => {
                writer
                    .write_all(
                        stringify!(CUDNN_FUSED_CONV_SCALE_BIAS_ADD_ACTIVATION).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnFusedOps_t::CUDNN_FUSED_SCALE_BIAS_ADD_ACTIVATION_GEN_BITMASK => {
                writer
                    .write_all(
                        stringify!(CUDNN_FUSED_SCALE_BIAS_ADD_ACTIVATION_GEN_BITMASK)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnFusedOps_t::CUDNN_FUSED_DACTIVATION_FORK_DBATCHNORM => {
                writer
                    .write_all(
                        stringify!(CUDNN_FUSED_DACTIVATION_FORK_DBATCHNORM).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_XDESC => {
                writer.write_all(stringify!(CUDNN_PARAM_XDESC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_XDATA_PLACEHOLDER => {
                writer.write_all(stringify!(CUDNN_PARAM_XDATA_PLACEHOLDER).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_BN_MODE => {
                writer.write_all(stringify!(CUDNN_PARAM_BN_MODE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_BN_EQSCALEBIAS_DESC => {
                writer.write_all(stringify!(CUDNN_PARAM_BN_EQSCALEBIAS_DESC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_BN_EQSCALE_PLACEHOLDER => {
                writer
                    .write_all(stringify!(CUDNN_PARAM_BN_EQSCALE_PLACEHOLDER).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_BN_EQBIAS_PLACEHOLDER => {
                writer
                    .write_all(stringify!(CUDNN_PARAM_BN_EQBIAS_PLACEHOLDER).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_ACTIVATION_DESC => {
                writer.write_all(stringify!(CUDNN_PARAM_ACTIVATION_DESC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_CONV_DESC => {
                writer.write_all(stringify!(CUDNN_PARAM_CONV_DESC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_WDESC => {
                writer.write_all(stringify!(CUDNN_PARAM_WDESC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_WDATA_PLACEHOLDER => {
                writer.write_all(stringify!(CUDNN_PARAM_WDATA_PLACEHOLDER).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_DWDESC => {
                writer.write_all(stringify!(CUDNN_PARAM_DWDESC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_DWDATA_PLACEHOLDER => {
                writer.write_all(stringify!(CUDNN_PARAM_DWDATA_PLACEHOLDER).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_YDESC => {
                writer.write_all(stringify!(CUDNN_PARAM_YDESC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_YDATA_PLACEHOLDER => {
                writer.write_all(stringify!(CUDNN_PARAM_YDATA_PLACEHOLDER).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_DYDESC => {
                writer.write_all(stringify!(CUDNN_PARAM_DYDESC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_DYDATA_PLACEHOLDER => {
                writer.write_all(stringify!(CUDNN_PARAM_DYDATA_PLACEHOLDER).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_YSTATS_DESC => {
                writer.write_all(stringify!(CUDNN_PARAM_YSTATS_DESC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_YSUM_PLACEHOLDER => {
                writer.write_all(stringify!(CUDNN_PARAM_YSUM_PLACEHOLDER).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_YSQSUM_PLACEHOLDER => {
                writer.write_all(stringify!(CUDNN_PARAM_YSQSUM_PLACEHOLDER).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_BN_SCALEBIAS_MEANVAR_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_PARAM_BN_SCALEBIAS_MEANVAR_DESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_BN_SCALE_PLACEHOLDER => {
                writer.write_all(stringify!(CUDNN_PARAM_BN_SCALE_PLACEHOLDER).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_BN_BIAS_PLACEHOLDER => {
                writer.write_all(stringify!(CUDNN_PARAM_BN_BIAS_PLACEHOLDER).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_BN_SAVED_MEAN_PLACEHOLDER => {
                writer
                    .write_all(
                        stringify!(CUDNN_PARAM_BN_SAVED_MEAN_PLACEHOLDER).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_BN_SAVED_INVSTD_PLACEHOLDER => {
                writer
                    .write_all(
                        stringify!(CUDNN_PARAM_BN_SAVED_INVSTD_PLACEHOLDER).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_BN_RUNNING_MEAN_PLACEHOLDER => {
                writer
                    .write_all(
                        stringify!(CUDNN_PARAM_BN_RUNNING_MEAN_PLACEHOLDER).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_BN_RUNNING_VAR_PLACEHOLDER => {
                writer
                    .write_all(
                        stringify!(CUDNN_PARAM_BN_RUNNING_VAR_PLACEHOLDER).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_ZDESC => {
                writer.write_all(stringify!(CUDNN_PARAM_ZDESC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_ZDATA_PLACEHOLDER => {
                writer.write_all(stringify!(CUDNN_PARAM_ZDATA_PLACEHOLDER).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_BN_Z_EQSCALEBIAS_DESC => {
                writer
                    .write_all(stringify!(CUDNN_PARAM_BN_Z_EQSCALEBIAS_DESC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_BN_Z_EQSCALE_PLACEHOLDER => {
                writer
                    .write_all(
                        stringify!(CUDNN_PARAM_BN_Z_EQSCALE_PLACEHOLDER).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_BN_Z_EQBIAS_PLACEHOLDER => {
                writer
                    .write_all(
                        stringify!(CUDNN_PARAM_BN_Z_EQBIAS_PLACEHOLDER).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_ACTIVATION_BITMASK_DESC => {
                writer
                    .write_all(
                        stringify!(CUDNN_PARAM_ACTIVATION_BITMASK_DESC).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_ACTIVATION_BITMASK_PLACEHOLDER => {
                writer
                    .write_all(
                        stringify!(CUDNN_PARAM_ACTIVATION_BITMASK_PLACEHOLDER).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_DXDESC => {
                writer.write_all(stringify!(CUDNN_PARAM_DXDESC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_DXDATA_PLACEHOLDER => {
                writer.write_all(stringify!(CUDNN_PARAM_DXDATA_PLACEHOLDER).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_DZDESC => {
                writer.write_all(stringify!(CUDNN_PARAM_DZDESC).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_DZDATA_PLACEHOLDER => {
                writer.write_all(stringify!(CUDNN_PARAM_DZDATA_PLACEHOLDER).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_BN_DSCALE_PLACEHOLDER => {
                writer
                    .write_all(stringify!(CUDNN_PARAM_BN_DSCALE_PLACEHOLDER).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t::CUDNN_PARAM_BN_DBIAS_PLACEHOLDER => {
                writer.write_all(stringify!(CUDNN_PARAM_BN_DBIAS_PLACEHOLDER).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnFusedOpsPointerPlaceHolder_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnFusedOpsPointerPlaceHolder_t::CUDNN_PTR_NULL => {
                writer.write_all(stringify!(CUDNN_PTR_NULL).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsPointerPlaceHolder_t::CUDNN_PTR_ELEM_ALIGNED => {
                writer.write_all(stringify!(CUDNN_PTR_ELEM_ALIGNED).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsPointerPlaceHolder_t::CUDNN_PTR_16B_ALIGNED => {
                writer.write_all(stringify!(CUDNN_PTR_16B_ALIGNED).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t::CUDNN_PTR_XDATA => {
                writer.write_all(stringify!(CUDNN_PTR_XDATA).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t::CUDNN_PTR_BN_EQSCALE => {
                writer.write_all(stringify!(CUDNN_PTR_BN_EQSCALE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t::CUDNN_PTR_BN_EQBIAS => {
                writer.write_all(stringify!(CUDNN_PTR_BN_EQBIAS).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t::CUDNN_PTR_WDATA => {
                writer.write_all(stringify!(CUDNN_PTR_WDATA).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t::CUDNN_PTR_DWDATA => {
                writer.write_all(stringify!(CUDNN_PTR_DWDATA).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t::CUDNN_PTR_YDATA => {
                writer.write_all(stringify!(CUDNN_PTR_YDATA).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t::CUDNN_PTR_DYDATA => {
                writer.write_all(stringify!(CUDNN_PTR_DYDATA).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t::CUDNN_PTR_YSUM => {
                writer.write_all(stringify!(CUDNN_PTR_YSUM).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t::CUDNN_PTR_YSQSUM => {
                writer.write_all(stringify!(CUDNN_PTR_YSQSUM).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t::CUDNN_PTR_WORKSPACE => {
                writer.write_all(stringify!(CUDNN_PTR_WORKSPACE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t::CUDNN_PTR_BN_SCALE => {
                writer.write_all(stringify!(CUDNN_PTR_BN_SCALE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t::CUDNN_PTR_BN_BIAS => {
                writer.write_all(stringify!(CUDNN_PTR_BN_BIAS).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t::CUDNN_PTR_BN_SAVED_MEAN => {
                writer.write_all(stringify!(CUDNN_PTR_BN_SAVED_MEAN).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t::CUDNN_PTR_BN_SAVED_INVSTD => {
                writer.write_all(stringify!(CUDNN_PTR_BN_SAVED_INVSTD).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t::CUDNN_PTR_BN_RUNNING_MEAN => {
                writer.write_all(stringify!(CUDNN_PTR_BN_RUNNING_MEAN).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t::CUDNN_PTR_BN_RUNNING_VAR => {
                writer.write_all(stringify!(CUDNN_PTR_BN_RUNNING_VAR).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t::CUDNN_PTR_ZDATA => {
                writer.write_all(stringify!(CUDNN_PTR_ZDATA).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t::CUDNN_PTR_BN_Z_EQSCALE => {
                writer.write_all(stringify!(CUDNN_PTR_BN_Z_EQSCALE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t::CUDNN_PTR_BN_Z_EQBIAS => {
                writer.write_all(stringify!(CUDNN_PTR_BN_Z_EQBIAS).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t::CUDNN_PTR_ACTIVATION_BITMASK => {
                writer.write_all(stringify!(CUDNN_PTR_ACTIVATION_BITMASK).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t::CUDNN_PTR_DXDATA => {
                writer.write_all(stringify!(CUDNN_PTR_DXDATA).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t::CUDNN_PTR_DZDATA => {
                writer.write_all(stringify!(CUDNN_PTR_DZDATA).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t::CUDNN_PTR_BN_DSCALE => {
                writer.write_all(stringify!(CUDNN_PTR_BN_DSCALE).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t::CUDNN_PTR_BN_DBIAS => {
                writer.write_all(stringify!(CUDNN_PTR_BN_DBIAS).as_bytes())
            }
            &cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t::CUDNN_SCALAR_SIZE_T_WORKSPACE_SIZE_IN_BYTES => {
                writer
                    .write_all(
                        stringify!(CUDNN_SCALAR_SIZE_T_WORKSPACE_SIZE_IN_BYTES)
                            .as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t::CUDNN_SCALAR_INT64_T_BN_ACCUMULATION_COUNT => {
                writer
                    .write_all(
                        stringify!(CUDNN_SCALAR_INT64_T_BN_ACCUMULATION_COUNT).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t::CUDNN_SCALAR_DOUBLE_BN_EXP_AVG_FACTOR => {
                writer
                    .write_all(
                        stringify!(CUDNN_SCALAR_DOUBLE_BN_EXP_AVG_FACTOR).as_bytes(),
                    )
            }
            &cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t::CUDNN_SCALAR_DOUBLE_BN_EPSILON => {
                writer.write_all(stringify!(CUDNN_SCALAR_DOUBLE_BN_EPSILON).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cudnnCnnVersionCheck(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnConvolutionBwdFilterAlgoPerfStruct {
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dyDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
    dwDesc: cuda_types::cudnn9::cudnnFilterDescriptor_t,
    requestedAlgoCount: ::core::ffi::c_int,
    returnedAlgoCount: *mut ::core::ffi::c_int,
    perfResults: *mut cuda_types::cudnn9::cudnnConvolutionBwdFilterAlgoPerf_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    dyDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    y: *const ::core::ffi::c_void,
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
    dwDesc: cuda_types::cudnn9::cudnnFilterDescriptor_t,
    dw: *mut ::core::ffi::c_void,
    requestedAlgoCount: ::core::ffi::c_int,
    returnedAlgoCount: *mut ::core::ffi::c_int,
    perfResults: *mut cuda_types::cudnn9::cudnnConvolutionBwdFilterAlgoPerf_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    srcDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    diffDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
    gradDesc: cuda_types::cudnn9::cudnnFilterDescriptor_t,
    requestedAlgoCount: ::core::ffi::c_int,
    returnedAlgoCount: *mut ::core::ffi::c_int,
    perfResults: *mut cuda_types::cudnn9::cudnnConvolutionBwdFilterAlgoPerf_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dyDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
    gradDesc: cuda_types::cudnn9::cudnnFilterDescriptor_t,
    algo: cuda_types::cudnn9::cudnnConvolutionBwdFilterAlgo_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    alpha: *const ::core::ffi::c_void,
    xDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    dyDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dy: *const ::core::ffi::c_void,
    convDesc: cuda_types::cudnn9::cudnnConvolutionDescriptor_t,
    algo: cuda_types::cudnn9::cudnnConvolutionBwdFilterAlgo_t,
    workSpace: *mut ::core::ffi::c_void,
    workSpaceSizeInBytes: usize,
    beta: *const ::core::ffi::c_void,
    dwDesc: cuda_types::cudnn9::cudnnFilterDescriptor_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    alpha: *const ::core::ffi::c_void,
    dyDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
    dy: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    dbDesc: cuda_types::cudnn9::cudnnTensorDescriptor_t,
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
    constPack: *mut cuda_types::cudnn9::cudnnFusedOpsConstParamPack_t,
    ops: cuda_types::cudnn9::cudnnFusedOps_t,
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
    constPack: cuda_types::cudnn9::cudnnFusedOpsConstParamPack_t,
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
    constPack: cuda_types::cudnn9::cudnnFusedOpsConstParamPack_t,
    paramLabel: cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t,
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
    constPack: cuda_types::cudnn9::cudnnFusedOpsConstParamPack_t,
    paramLabel: cuda_types::cudnn9::cudnnFusedOpsConstParamLabel_t,
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
    varPack: *mut cuda_types::cudnn9::cudnnFusedOpsVariantParamPack_t,
    ops: cuda_types::cudnn9::cudnnFusedOps_t,
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
    varPack: cuda_types::cudnn9::cudnnFusedOpsVariantParamPack_t,
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
    varPack: cuda_types::cudnn9::cudnnFusedOpsVariantParamPack_t,
    paramLabel: cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t,
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
    varPack: cuda_types::cudnn9::cudnnFusedOpsVariantParamPack_t,
    paramLabel: cuda_types::cudnn9::cudnnFusedOpsVariantParamLabel_t,
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
    plan: *mut cuda_types::cudnn9::cudnnFusedOpsPlan_t,
    ops: cuda_types::cudnn9::cudnnFusedOps_t,
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
    plan: cuda_types::cudnn9::cudnnFusedOpsPlan_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(plan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&plan, "cudnnDestroyFusedOpsPlan", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cudnnMakeFusedOpsPlan(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cudnn9::cudnnHandle_t,
    plan: cuda_types::cudnn9::cudnnFusedOpsPlan_t,
    constPack: cuda_types::cudnn9::cudnnFusedOpsConstParamPack_t,
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
    handle: cuda_types::cudnn9::cudnnHandle_t,
    plan: cuda_types::cudnn9::cudnnFusedOpsPlan_t,
    varPack: cuda_types::cudnn9::cudnnFusedOpsVariantParamPack_t,
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
impl crate::CudaDisplay for cuda_types::cudnn9::cudnnStatus_t {
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
                    1001 => writer.write_all("CUDNN_STATUS_NOT_INITIALIZED".as_bytes()),
                    1002 => {
                        writer
                            .write_all(
                                "CUDNN_STATUS_SUBLIBRARY_VERSION_MISMATCH".as_bytes(),
                            )
                    }
                    1003 => {
                        writer
                            .write_all(
                                "CUDNN_STATUS_SERIALIZATION_VERSION_MISMATCH".as_bytes(),
                            )
                    }
                    1004 => writer.write_all("CUDNN_STATUS_DEPRECATED".as_bytes()),
                    1005 => writer.write_all("CUDNN_STATUS_LICENSE_ERROR".as_bytes()),
                    1006 => {
                        writer.write_all("CUDNN_STATUS_RUNTIME_IN_PROGRESS".as_bytes())
                    }
                    1007 => {
                        writer.write_all("CUDNN_STATUS_RUNTIME_FP_OVERFLOW".as_bytes())
                    }
                    1008 => {
                        writer
                            .write_all(
                                "CUDNN_STATUS_SUBLIBRARY_LOADING_FAILED".as_bytes(),
                            )
                    }
                    2000 => writer.write_all("CUDNN_STATUS_BAD_PARAM".as_bytes()),
                    2002 => {
                        writer
                            .write_all("CUDNN_STATUS_BAD_PARAM_NULL_POINTER".as_bytes())
                    }
                    2003 => {
                        writer
                            .write_all(
                                "CUDNN_STATUS_BAD_PARAM_MISALIGNED_POINTER".as_bytes(),
                            )
                    }
                    2004 => {
                        writer
                            .write_all("CUDNN_STATUS_BAD_PARAM_NOT_FINALIZED".as_bytes())
                    }
                    2005 => {
                        writer
                            .write_all("CUDNN_STATUS_BAD_PARAM_OUT_OF_BOUND".as_bytes())
                    }
                    2006 => {
                        writer
                            .write_all(
                                "CUDNN_STATUS_BAD_PARAM_SIZE_INSUFFICIENT".as_bytes(),
                            )
                    }
                    2007 => {
                        writer
                            .write_all(
                                "CUDNN_STATUS_BAD_PARAM_STREAM_MISMATCH".as_bytes(),
                            )
                    }
                    2008 => {
                        writer
                            .write_all(
                                "CUDNN_STATUS_BAD_PARAM_SHAPE_MISMATCH".as_bytes(),
                            )
                    }
                    2009 => {
                        writer
                            .write_all(
                                "CUDNN_STATUS_BAD_PARAM_DUPLICATED_ENTRIES".as_bytes(),
                            )
                    }
                    2010 => {
                        writer
                            .write_all(
                                "CUDNN_STATUS_BAD_PARAM_ATTRIBUTE_TYPE".as_bytes(),
                            )
                    }
                    2011 => {
                        writer
                            .write_all(
                                "CUDNN_STATUS_BAD_PARAM_CUDA_GRAPH_MISMATCH".as_bytes(),
                            )
                    }
                    2012 => {
                        writer
                            .write_all(
                                "CUDNN_STATUS_BAD_PARAM_DESCRIPTOR_TYPE".as_bytes(),
                            )
                    }
                    3000 => writer.write_all("CUDNN_STATUS_NOT_SUPPORTED".as_bytes()),
                    3001 => {
                        writer
                            .write_all(
                                "CUDNN_STATUS_NOT_SUPPORTED_GRAPH_PATTERN".as_bytes(),
                            )
                    }
                    3002 => {
                        writer.write_all("CUDNN_STATUS_NOT_SUPPORTED_SHAPE".as_bytes())
                    }
                    3003 => {
                        writer
                            .write_all("CUDNN_STATUS_NOT_SUPPORTED_DATA_TYPE".as_bytes())
                    }
                    3004 => {
                        writer.write_all("CUDNN_STATUS_NOT_SUPPORTED_LAYOUT".as_bytes())
                    }
                    3005 => {
                        writer
                            .write_all(
                                "CUDNN_STATUS_NOT_SUPPORTED_INCOMPATIBLE_CUDA_DRIVER"
                                    .as_bytes(),
                            )
                    }
                    3006 => {
                        writer
                            .write_all(
                                "CUDNN_STATUS_NOT_SUPPORTED_INCOMPATIBLE_CUDART".as_bytes(),
                            )
                    }
                    3007 => {
                        writer
                            .write_all(
                                "CUDNN_STATUS_NOT_SUPPORTED_ARCH_MISMATCH".as_bytes(),
                            )
                    }
                    3008 => {
                        writer
                            .write_all(
                                "CUDNN_STATUS_NOT_SUPPORTED_RUNTIME_PREREQUISITE_MISSING"
                                    .as_bytes(),
                            )
                    }
                    3009 => {
                        writer
                            .write_all(
                                "CUDNN_STATUS_NOT_SUPPORTED_SUBLIBRARY_UNAVAILABLE"
                                    .as_bytes(),
                            )
                    }
                    3010 => {
                        writer
                            .write_all(
                                "CUDNN_STATUS_NOT_SUPPORTED_SHARED_MEMORY_INSUFFICIENT"
                                    .as_bytes(),
                            )
                    }
                    3011 => {
                        writer.write_all("CUDNN_STATUS_NOT_SUPPORTED_PADDING".as_bytes())
                    }
                    3012 => {
                        writer
                            .write_all(
                                "CUDNN_STATUS_NOT_SUPPORTED_BAD_LAUNCH_PARAM".as_bytes(),
                            )
                    }
                    3013 => {
                        writer
                            .write_all(
                                "CUDNN_STATUS_NOT_SUPPORTED_CUDA_GRAPH_NATIVE_API"
                                    .as_bytes(),
                            )
                    }
                    4000 => writer.write_all("CUDNN_STATUS_INTERNAL_ERROR".as_bytes()),
                    4001 => {
                        writer
                            .write_all(
                                "CUDNN_STATUS_INTERNAL_ERROR_COMPILATION_FAILED".as_bytes(),
                            )
                    }
                    4002 => {
                        writer
                            .write_all(
                                "CUDNN_STATUS_INTERNAL_ERROR_UNEXPECTED_VALUE".as_bytes(),
                            )
                    }
                    4003 => {
                        writer
                            .write_all(
                                "CUDNN_STATUS_INTERNAL_ERROR_HOST_ALLOCATION_FAILED"
                                    .as_bytes(),
                            )
                    }
                    4004 => {
                        writer
                            .write_all(
                                "CUDNN_STATUS_INTERNAL_ERROR_DEVICE_ALLOCATION_FAILED"
                                    .as_bytes(),
                            )
                    }
                    4005 => {
                        writer
                            .write_all(
                                "CUDNN_STATUS_INTERNAL_ERROR_BAD_LAUNCH_PARAM".as_bytes(),
                            )
                    }
                    4006 => {
                        writer
                            .write_all(
                                "CUDNN_STATUS_INTERNAL_ERROR_TEXTURE_CREATION_FAILED"
                                    .as_bytes(),
                            )
                    }
                    5000 => writer.write_all("CUDNN_STATUS_EXECUTION_FAILED".as_bytes()),
                    5001 => {
                        writer
                            .write_all(
                                "CUDNN_STATUS_EXECUTION_FAILED_CUDA_DRIVER".as_bytes(),
                            )
                    }
                    5002 => {
                        writer
                            .write_all("CUDNN_STATUS_EXECUTION_FAILED_CUBLAS".as_bytes())
                    }
                    5003 => {
                        writer
                            .write_all("CUDNN_STATUS_EXECUTION_FAILED_CUDART".as_bytes())
                    }
                    5004 => {
                        writer
                            .write_all("CUDNN_STATUS_EXECUTION_FAILED_CURAND".as_bytes())
                    }
                    4003 => writer.write_all("CUDNN_STATUS_ALLOC_FAILED".as_bytes()),
                    2001 => writer.write_all("CUDNN_STATUS_INVALID_VALUE".as_bytes()),
                    3007 => writer.write_all("CUDNN_STATUS_ARCH_MISMATCH".as_bytes()),
                    4006 => writer.write_all("CUDNN_STATUS_MAPPING_ERROR".as_bytes()),
                    3008 => {
                        writer
                            .write_all(
                                "CUDNN_STATUS_RUNTIME_PREREQUISITE_MISSING".as_bytes(),
                            )
                    }
                    1002 => writer.write_all("CUDNN_STATUS_VERSION_MISMATCH".as_bytes()),
                    err => write!(writer, "{}", err),
                }
            }
        }
    }
}
