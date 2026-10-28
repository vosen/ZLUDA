//! cuDNN 9 on Apple GPUs for zluda_agx (Asahi Linux): the legacy convolution API, on the MLIR convolution kernels of
//! agx_libs (`agx_libs/kernels/conv.mlir`, written once for every AGX generation).
//!
//! Supported so far: handles and streams, 4-D tensor descriptors with any strides, packed NCHW filters, 2-D
//! convolution descriptors (padding, strides, dilations, groups, cross-correlation or convolution), and
//! `cudnnConvolutionForward`, `cudnnConvolutionBackwardData` and `cudnnConvolutionBackwardFilter` for f32 and f16
//! tensors (accumulated in f32), with the algorithm queries (one algorithm per direction, no workspace); and, on
//! `agx_libs/kernels/dnn.mlir`, activations, add / set / scale tensor, softmax, 2-D pooling and batch normalization,
//! forward and backward, for f32 tensors and for f16 and bf16 ones through packed f32 copies. The backend (graph)
//! API runs graphs of convolutions, matmuls, pointwise operations and reductions (`imp/graph.rs`). Other functions return `CUDNN_STATUS_NOT_SUPPORTED`.

#![allow(non_snake_case)]

use cuda_types::cudnn9::*;
use std::ffi::{c_char, c_int, c_void};

macro_rules! unimplemented_fn {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[cfg_attr(not(test), no_mangle)]
            #[allow(improper_ctypes_definitions, unused_variables)]
            pub extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                if std::env::var_os("ZLUDA_AGX_LOG").is_some() {
                    eprintln!("[zluda_dnn_agx] {} is not implemented", stringify!($fn_name));
                }
                Err(cudnnError_t::NOT_SUPPORTED)
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

cuda_macros::cudnn9_function_declarations!(
    unimplemented_fn,
    implemented_fn
        <= [
            cudnnBackendSetAttribute,
            cudnnBackendInitialize,
            cudnnBackendGetAttribute,
            cudnnBackendFinalize,
            cudnnBackendExecute,
            cudnnBackendDestroyDescriptor,
            cudnnBackendCreateDescriptor,
            cudnnActivationBackward,
            cudnnActivationForward,
            cudnnAddTensor,
            cudnnBatchNormalizationBackward,
            cudnnBatchNormalizationForwardInference,
            cudnnBatchNormalizationForwardTraining,
            cudnnConvolutionBackwardData,
            cudnnConvolutionBackwardFilter,
            cudnnConvolutionForward,
            cudnnCreate,
            cudnnCreateActivationDescriptor,
            cudnnCreateConvolutionDescriptor,
            cudnnCreateFilterDescriptor,
            cudnnCreatePoolingDescriptor,
            cudnnCreateTensorDescriptor,
            cudnnDeriveBNTensorDescriptor,
            cudnnDestroy,
            cudnnDestroyActivationDescriptor,
            cudnnDestroyConvolutionDescriptor,
            cudnnDestroyFilterDescriptor,
            cudnnDestroyPoolingDescriptor,
            cudnnDestroyTensorDescriptor,
            cudnnFindConvolutionBackwardDataAlgorithm,
            cudnnFindConvolutionBackwardDataAlgorithmEx,
            cudnnFindConvolutionBackwardFilterAlgorithm,
            cudnnFindConvolutionBackwardFilterAlgorithmEx,
            cudnnFindConvolutionForwardAlgorithm,
            cudnnFindConvolutionForwardAlgorithmEx,
            cudnnGetActivationDescriptor,
            cudnnGetConvolution2dDescriptor,
            cudnnGetConvolution2dForwardOutputDim,
            cudnnGetConvolutionBackwardDataAlgorithmMaxCount,
            cudnnGetConvolutionBackwardDataAlgorithm_v7,
            cudnnGetConvolutionBackwardDataWorkspaceSize,
            cudnnGetConvolutionBackwardFilterAlgorithmMaxCount,
            cudnnGetConvolutionBackwardFilterAlgorithm_v7,
            cudnnGetConvolutionBackwardFilterWorkspaceSize,
            cudnnGetConvolutionForwardAlgorithmMaxCount,
            cudnnGetConvolutionForwardAlgorithm_v7,
            cudnnGetConvolutionForwardWorkspaceSize,
            cudnnGetConvolutionGroupCount,
            cudnnGetConvolutionMathType,
            cudnnGetConvolutionNdForwardOutputDim,
            cudnnGetCudartVersion,
            cudnnGetErrorString,
            cudnnGetFilter4dDescriptor,
            cudnnGetLastErrorString,
            cudnnGetMaxDeviceVersion,
            cudnnGetPooling2dDescriptor,
            cudnnGetPooling2dForwardOutputDim,
            cudnnGetStream,
            cudnnGetTensor4dDescriptor,
            cudnnGetTensorNdDescriptor,
            cudnnGetVersion,
            cudnnPoolingBackward,
            cudnnPoolingForward,
            cudnnScaleTensor,
            cudnnSetActivationDescriptor,
            cudnnSetActivationDescriptorSwishBeta,
            cudnnSetConvolution2dDescriptor,
            cudnnSetConvolutionGroupCount,
            cudnnSetConvolutionMathType,
            cudnnSetConvolutionNdDescriptor,
            cudnnSetConvolutionReorderType,
            cudnnSetFilter4dDescriptor,
            cudnnSetFilterNdDescriptor,
            cudnnSetPooling2dDescriptor,
            cudnnSetStream,
            cudnnSetTensor,
            cudnnSetTensor4dDescriptor,
            cudnnSetTensor4dDescriptorEx,
            cudnnSetTensorNdDescriptor,
            cudnnSoftmaxBackward,
            cudnnSoftmaxForward,
        ]
);

/// A tensor descriptor: up to 8 dimensions with element strides.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Tensor {
    pub ty: u32,
    pub dims: Vec<i32>,
    pub strides: Vec<i32>,
}

/// A filter descriptor: K x C x R x S, packed in `format` (only NCHW is supported by the kernels).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Filter {
    pub ty: u32,
    pub format: u32,
    pub dims: Vec<i32>,
}

/// A 2-D convolution descriptor.
#[derive(Clone, Debug, PartialEq)]
pub struct Conv {
    pub pad: [i32; 2],
    pub stride: [i32; 2],
    pub dilation: [i32; 2],
    /// CUDNN_CONVOLUTION (0, the filter flipped) or CUDNN_CROSS_CORRELATION (1).
    pub mode: u32,
    pub compute: u32,
    pub groups: i32,
    pub math: u32,
}

impl Default for Conv {
    fn default() -> Conv {
        Conv { pad: [0; 2], stride: [1; 2], dilation: [1; 2], mode: 0, compute: 0, groups: 1, math: 0 }
    }
}

/// The output height and width of a convolution (cuDNN's formula).
pub fn output_dim(input: i32, pad: i32, stride: i32, dilation: i32, filter: i32) -> i32 {
    (input + 2 * pad - dilation * (filter - 1) - 1).div_euclid(stride) + 1
}

/// The 25 shape arguments of the conv.mlir kernels for x (N C H W), the filter and y (N K P Q):
/// N C H W K R S P Q, pads, strides, dilations, groups, flip, then the element strides of x and y.
pub fn kernel_shape(x: &Tensor, w: &Filter, c: &Conv, y: &Tensor) -> Result<[i32; 25], cudnnError_t> {
    if x.dims.len() != 4 || y.dims.len() != 4 || w.dims.len() != 4 {
        return Err(cudnnError_t::NOT_SUPPORTED);
    }
    if w.format != cudnnTensorFormat_t::CUDNN_TENSOR_NCHW.0 {
        return Err(cudnnError_t::NOT_SUPPORTED);
    }
    if x.ty != y.ty || x.ty != w.ty || !(x.ty == cudnnDataType_t::CUDNN_DATA_FLOAT.0 || x.ty == cudnnDataType_t::CUDNN_DATA_HALF.0) {
        return Err(cudnnError_t::NOT_SUPPORTED);
    }
    let (n, ch, h, wd) = (x.dims[0], x.dims[1], x.dims[2], x.dims[3]);
    let (k, cpg, r, s) = (w.dims[0], w.dims[1], w.dims[2], w.dims[3]);
    let g = c.groups;
    if g < 1 || ch != cpg * g || k % g != 0 || c.stride.iter().any(|&v| v < 1) || c.dilation.iter().any(|&v| v < 1) || c.pad.iter().any(|&v| v < 0) {
        return Err(cudnnError_t::BAD_PARAM);
    }
    let p = output_dim(h, c.pad[0], c.stride[0], c.dilation[0], r);
    let q = output_dim(wd, c.pad[1], c.stride[1], c.dilation[1], s);
    if y.dims != [n, k, p, q] || p < 1 || q < 1 {
        return Err(cudnnError_t::BAD_PARAM);
    }
    let flip = (c.mode == 0) as i32;
    let (xs, ys) = (&x.strides, &y.strides);
    Ok([
        n, ch, h, wd, k, r, s, p, q, c.pad[0], c.pad[1], c.stride[0], c.stride[1], c.dilation[0], c.dilation[1], g, flip,
        xs[0], xs[1], xs[2], xs[3], ys[0], ys[1], ys[2], ys[3],
    ])
}

/// Packed strides for dimensions in NCHW order, or NHWC (`nhwc`) for 4-D tensors.
pub fn packed_strides(dims: &[i32], nhwc: bool) -> Vec<i32> {
    if nhwc && dims.len() == 4 {
        let (c, h, w) = (dims[1], dims[2], dims[3]);
        return vec![h * w * c, 1, w * c, c];
    }
    let mut s = vec![1; dims.len()];
    for i in (0..dims.len().saturating_sub(1)).rev() {
        s[i] = s[i + 1] * dims[i + 1];
    }
    s
}

/// An activation descriptor.
#[derive(Clone, Debug, PartialEq)]
pub struct Activation {
    pub mode: u32,
    pub nan: u32,
    pub coef: f64,
    pub swish_beta: f64,
}

impl Default for Activation {
    fn default() -> Activation {
        Activation { mode: cudnnActivationMode_t::CUDNN_ACTIVATION_RELU.0, nan: 0, coef: 0.0, swish_beta: 1.0 }
    }
}

/// A 2-D pooling descriptor.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Pooling {
    pub mode: u32,
    pub nan: u32,
    pub window: [i32; 2],
    pub pad: [i32; 2],
    pub stride: [i32; 2],
}

/// A 4-D f32 tensor as the dnn.mlir kernels take it: dimensions and element strides, as i64.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct View {
    pub dims: [i64; 4],
    pub strides: [i64; 4],
}

impl View {
    pub fn total(&self) -> i64 {
        self.dims.iter().product()
    }
}

/// The 4-D view of an f32 tensor of 1 to 4 dimensions (missing trailing dimensions are 1).
pub fn view(t: &Tensor) -> Result<View, cudnnError_t> {
    if t.ty != cudnnDataType_t::CUDNN_DATA_FLOAT.0 {
        return Err(cudnnError_t::NOT_SUPPORTED);
    }
    if t.dims.is_empty() || t.dims.len() > 4 || t.strides.len() != t.dims.len() {
        return Err(if t.dims.is_empty() { cudnnError_t::BAD_PARAM } else { cudnnError_t::NOT_SUPPORTED });
    }
    let mut v = View { dims: [1; 4], strides: [0; 4] };
    for (i, (&d, &s)) in t.dims.iter().zip(&t.strides).enumerate() {
        v.dims[i] = d as i64;
        v.strides[i] = s as i64;
    }
    Ok(v)
}

/// The strides of `a` broadcast over `c`'s dimensions (cudnnAddTensor): each dimension of `a` equals `c`'s or is 1.
pub fn broadcast(a: &View, c: &View) -> Result<[i64; 4], cudnnError_t> {
    let mut s = [0; 4];
    for i in 0..4 {
        s[i] = match a.dims[i] {
            d if d == c.dims[i] => a.strides[i],
            1 => 0,
            _ => return Err(cudnnError_t::BAD_PARAM),
        };
    }
    Ok(s)
}

/// The output height and width of 2-D pooling.
pub fn pool_output(p: &Pooling, h: i64, w: i64) -> (i64, i64) {
    let d = |i: i64, k: usize| (i + 2 * p.pad[k] as i64 - p.window[k] as i64).div_euclid(p.stride[k] as i64) + 1;
    (d(h, 0), d(w, 1))
}

/// The dnn.mlir activation mode and coefficient of a descriptor.
pub fn activation_args(a: &Activation) -> Result<(i64, f32), cudnnError_t> {
    match a.mode {
        0..=5 => Ok((a.mode as i64, a.coef as f32)),
        6 => Ok((6, a.swish_beta as f32)),
        _ => Err(cudnnError_t::NOT_SUPPORTED),
    }
}

mod imp {
    use super::*;
    use agx_libs::engine;

    struct Handle {
        stream: cudaStream_t,
    }

    fn status(e: engine::Error) -> cudnnError_t {
        match e {
            engine::Error::NotInitialized => cudnnError_t::NOT_INITIALIZED,
            engine::Error::ArchMismatch => cudnnError_t::ARCH_MISMATCH,
            engine::Error::Failed => cudnnError_t::EXECUTION_FAILED,
        }
    }

    unsafe fn obj<'a, T>(p: *mut impl Sized) -> Result<&'a mut T, cudnnError_t> {
        (p as *mut T).as_mut().ok_or(cudnnError_t::BAD_PARAM)
    }

    unsafe fn create<T: Default, P>(out: *mut *mut P) -> cudnnStatus_t {
        *out.as_mut().ok_or(cudnnError_t::BAD_PARAM)? = Box::into_raw(Box::<T>::default()) as *mut P;
        Ok(())
    }

    unsafe fn destroy<T, P>(p: *mut P) -> cudnnStatus_t {
        if !p.is_null() {
            drop(Box::from_raw(p as *mut T));
        }
        Ok(())
    }

    unsafe fn out<T>(p: *mut T, v: T) -> cudnnStatus_t {
        *p.as_mut().ok_or(cudnnError_t::BAD_PARAM)? = v;
        Ok(())
    }

    pub(crate) unsafe fn cudnnGetVersion() -> usize {
        90_100
    }

    pub(crate) unsafe fn cudnnGetMaxDeviceVersion() -> usize {
        // the newest device architecture "supported", as an sm number
        900
    }

    pub(crate) unsafe fn cudnnGetCudartVersion() -> usize {
        12_040
    }

    pub(crate) unsafe fn cudnnGetErrorString(s: cudnnStatus_t) -> *const c_char {
        match s {
            Ok(()) => c"CUDNN_STATUS_SUCCESS",
            Err(cudnnError_t::NOT_INITIALIZED) => c"CUDNN_STATUS_NOT_INITIALIZED",
            Err(cudnnError_t::BAD_PARAM) => c"CUDNN_STATUS_BAD_PARAM",
            Err(cudnnError_t::NOT_SUPPORTED) => c"CUDNN_STATUS_NOT_SUPPORTED",
            Err(cudnnError_t::ARCH_MISMATCH) => c"CUDNN_STATUS_ARCH_MISMATCH",
            Err(cudnnError_t::EXECUTION_FAILED) => c"CUDNN_STATUS_EXECUTION_FAILED",
            Err(_) => c"CUDNN_STATUS_INTERNAL_ERROR",
        }
        .as_ptr()
    }

    pub(crate) unsafe fn cudnnGetLastErrorString(message: *mut c_char, max_size: usize) {
        if !message.is_null() && max_size > 0 {
            *message = 0;
        }
    }

    pub(crate) unsafe fn cudnnCreate(h: *mut cudnnHandle_t) -> cudnnStatus_t {
        if !engine::available() {
            return Err(cudnnError_t::NOT_INITIALIZED);
        }
        out(h, Box::into_raw(Box::new(Handle { stream: cuda_types::cuda::CUstream(std::ptr::null_mut()) })) as cudnnHandle_t)
    }

    pub(crate) unsafe fn cudnnDestroy(h: cudnnHandle_t) -> cudnnStatus_t {
        destroy::<Handle, _>(h)
    }

    pub(crate) unsafe fn cudnnSetStream(h: cudnnHandle_t, s: cudaStream_t) -> cudnnStatus_t {
        obj::<Handle>(h)?.stream = s;
        Ok(())
    }

    pub(crate) unsafe fn cudnnGetStream(h: cudnnHandle_t, s: *mut cudaStream_t) -> cudnnStatus_t {
        out(s, obj::<Handle>(h)?.stream)
    }

    // ---- descriptors

    pub(crate) unsafe fn cudnnCreateTensorDescriptor(d: *mut cudnnTensorDescriptor_t) -> cudnnStatus_t {
        create::<Tensor, _>(d)
    }

    pub(crate) unsafe fn cudnnDestroyTensorDescriptor(d: cudnnTensorDescriptor_t) -> cudnnStatus_t {
        destroy::<Tensor, _>(d)
    }

    pub(crate) unsafe fn cudnnSetTensor4dDescriptor(
        d: cudnnTensorDescriptor_t,
        format: cudnnTensorFormat_t,
        ty: cudnnDataType_t,
        n: c_int,
        c: c_int,
        h: c_int,
        w: c_int,
    ) -> cudnnStatus_t {
        let dims = vec![n, c, h, w];
        if dims.iter().any(|&v| v < 1) {
            return Err(cudnnError_t::BAD_PARAM);
        }
        let nhwc = match format {
            cudnnTensorFormat_t::CUDNN_TENSOR_NCHW => false,
            cudnnTensorFormat_t::CUDNN_TENSOR_NHWC => true,
            _ => return Err(cudnnError_t::NOT_SUPPORTED),
        };
        *obj::<Tensor>(d)? = Tensor { ty: ty.0, strides: packed_strides(&dims, nhwc), dims };
        Ok(())
    }

    pub(crate) unsafe fn cudnnSetTensor4dDescriptorEx(
        d: cudnnTensorDescriptor_t,
        ty: cudnnDataType_t,
        n: c_int,
        c: c_int,
        h: c_int,
        w: c_int,
        ns: c_int,
        cs: c_int,
        hs: c_int,
        ws: c_int,
    ) -> cudnnStatus_t {
        if [n, c, h, w].iter().any(|&v| v < 1) {
            return Err(cudnnError_t::BAD_PARAM);
        }
        *obj::<Tensor>(d)? = Tensor { ty: ty.0, dims: vec![n, c, h, w], strides: vec![ns, cs, hs, ws] };
        Ok(())
    }

    pub(crate) unsafe fn cudnnSetTensorNdDescriptor(
        d: cudnnTensorDescriptor_t,
        ty: cudnnDataType_t,
        nb: c_int,
        dims: *const c_int,
        strides: *const c_int,
    ) -> cudnnStatus_t {
        if !(1..=8).contains(&nb) || dims.is_null() || strides.is_null() {
            return Err(cudnnError_t::BAD_PARAM);
        }
        let dims = std::slice::from_raw_parts(dims, nb as usize).to_vec();
        let strides = std::slice::from_raw_parts(strides, nb as usize).to_vec();
        *obj::<Tensor>(d)? = Tensor { ty: ty.0, dims, strides };
        Ok(())
    }

    pub(crate) unsafe fn cudnnGetTensor4dDescriptor(
        d: cudnnTensorDescriptor_t,
        ty: *mut cudnnDataType_t,
        n: *mut c_int,
        c: *mut c_int,
        h: *mut c_int,
        w: *mut c_int,
        ns: *mut c_int,
        cs: *mut c_int,
        hs: *mut c_int,
        ws: *mut c_int,
    ) -> cudnnStatus_t {
        let t = obj::<Tensor>(d)?;
        if t.dims.len() != 4 {
            return Err(cudnnError_t::BAD_PARAM);
        }
        out(ty, cudnnDataType_t(t.ty))?;
        for (p, v) in [n, c, h, w].into_iter().zip(&t.dims) {
            out(p, *v)?;
        }
        for (p, v) in [ns, cs, hs, ws].into_iter().zip(&t.strides) {
            out(p, *v)?;
        }
        Ok(())
    }

    pub(crate) unsafe fn cudnnGetTensorNdDescriptor(
        d: cudnnTensorDescriptor_t,
        requested: c_int,
        ty: *mut cudnnDataType_t,
        nb: *mut c_int,
        dims: *mut c_int,
        strides: *mut c_int,
    ) -> cudnnStatus_t {
        let t = obj::<Tensor>(d)?;
        out(ty, cudnnDataType_t(t.ty))?;
        out(nb, t.dims.len() as c_int)?;
        for i in 0..(requested.max(0) as usize).min(t.dims.len()) {
            if !dims.is_null() {
                *dims.add(i) = t.dims[i];
            }
            if !strides.is_null() {
                *strides.add(i) = t.strides[i];
            }
        }
        Ok(())
    }

    pub(crate) unsafe fn cudnnCreateFilterDescriptor(d: *mut cudnnFilterDescriptor_t) -> cudnnStatus_t {
        create::<Filter, _>(d)
    }

    pub(crate) unsafe fn cudnnDestroyFilterDescriptor(d: cudnnFilterDescriptor_t) -> cudnnStatus_t {
        destroy::<Filter, _>(d)
    }

    pub(crate) unsafe fn cudnnSetFilter4dDescriptor(
        d: cudnnFilterDescriptor_t,
        ty: cudnnDataType_t,
        format: cudnnTensorFormat_t,
        k: c_int,
        c: c_int,
        h: c_int,
        w: c_int,
    ) -> cudnnStatus_t {
        if [k, c, h, w].iter().any(|&v| v < 1) {
            return Err(cudnnError_t::BAD_PARAM);
        }
        *obj::<Filter>(d)? = Filter { ty: ty.0, format: format.0, dims: vec![k, c, h, w] };
        Ok(())
    }

    pub(crate) unsafe fn cudnnSetFilterNdDescriptor(
        d: cudnnFilterDescriptor_t,
        ty: cudnnDataType_t,
        format: cudnnTensorFormat_t,
        nb: c_int,
        dims: *const c_int,
    ) -> cudnnStatus_t {
        if !(1..=8).contains(&nb) || dims.is_null() {
            return Err(cudnnError_t::BAD_PARAM);
        }
        *obj::<Filter>(d)? = Filter { ty: ty.0, format: format.0, dims: std::slice::from_raw_parts(dims, nb as usize).to_vec() };
        Ok(())
    }

    pub(crate) unsafe fn cudnnGetFilter4dDescriptor(
        d: cudnnFilterDescriptor_t,
        ty: *mut cudnnDataType_t,
        format: *mut cudnnTensorFormat_t,
        k: *mut c_int,
        c: *mut c_int,
        h: *mut c_int,
        w: *mut c_int,
    ) -> cudnnStatus_t {
        let f = obj::<Filter>(d)?;
        if f.dims.len() != 4 {
            return Err(cudnnError_t::BAD_PARAM);
        }
        out(ty, cudnnDataType_t(f.ty))?;
        out(format, cudnnTensorFormat_t(f.format))?;
        for (p, v) in [k, c, h, w].into_iter().zip(&f.dims) {
            out(p, *v)?;
        }
        Ok(())
    }

    pub(crate) unsafe fn cudnnCreateConvolutionDescriptor(d: *mut cudnnConvolutionDescriptor_t) -> cudnnStatus_t {
        create::<Conv, _>(d)
    }

    pub(crate) unsafe fn cudnnDestroyConvolutionDescriptor(d: cudnnConvolutionDescriptor_t) -> cudnnStatus_t {
        destroy::<Conv, _>(d)
    }

    pub(crate) unsafe fn cudnnSetConvolution2dDescriptor(
        d: cudnnConvolutionDescriptor_t,
        ph: c_int,
        pw: c_int,
        u: c_int,
        v: c_int,
        dh: c_int,
        dw: c_int,
        mode: cudnnConvolutionMode_t,
        compute: cudnnDataType_t,
    ) -> cudnnStatus_t {
        if ph < 0 || pw < 0 || u < 1 || v < 1 || dh < 1 || dw < 1 {
            return Err(cudnnError_t::BAD_PARAM);
        }
        let c = obj::<Conv>(d)?;
        *c = Conv { pad: [ph, pw], stride: [u, v], dilation: [dh, dw], mode: mode.0, compute: compute.0, ..c.clone() };
        Ok(())
    }

    pub(crate) unsafe fn cudnnSetConvolutionNdDescriptor(
        d: cudnnConvolutionDescriptor_t,
        len: c_int,
        pad: *const c_int,
        stride: *const c_int,
        dilation: *const c_int,
        mode: cudnnConvolutionMode_t,
        compute: cudnnDataType_t,
    ) -> cudnnStatus_t {
        if len != 2 || pad.is_null() || stride.is_null() || dilation.is_null() {
            return Err(cudnnError_t::NOT_SUPPORTED);
        }
        cudnnSetConvolution2dDescriptor(d, *pad, *pad.add(1), *stride, *stride.add(1), *dilation, *dilation.add(1), mode, compute)
    }

    pub(crate) unsafe fn cudnnGetConvolution2dDescriptor(
        d: cudnnConvolutionDescriptor_t,
        ph: *mut c_int,
        pw: *mut c_int,
        u: *mut c_int,
        v: *mut c_int,
        dh: *mut c_int,
        dw: *mut c_int,
        mode: *mut cudnnConvolutionMode_t,
        compute: *mut cudnnDataType_t,
    ) -> cudnnStatus_t {
        let c = obj::<Conv>(d)?;
        for (p, x) in [(ph, c.pad[0]), (pw, c.pad[1]), (u, c.stride[0]), (v, c.stride[1]), (dh, c.dilation[0]), (dw, c.dilation[1])] {
            out(p, x)?;
        }
        out(mode, cudnnConvolutionMode_t(c.mode))?;
        out(compute, cudnnDataType_t(c.compute))
    }

    pub(crate) unsafe fn cudnnSetConvolutionGroupCount(d: cudnnConvolutionDescriptor_t, g: c_int) -> cudnnStatus_t {
        if g < 1 {
            return Err(cudnnError_t::BAD_PARAM);
        }
        obj::<Conv>(d)?.groups = g;
        Ok(())
    }

    pub(crate) unsafe fn cudnnGetConvolutionGroupCount(d: cudnnConvolutionDescriptor_t, g: *mut c_int) -> cudnnStatus_t {
        out(g, obj::<Conv>(d)?.groups)
    }

    /// Every math type computes in f32.
    pub(crate) unsafe fn cudnnSetConvolutionMathType(d: cudnnConvolutionDescriptor_t, m: cudnnMathType_t) -> cudnnStatus_t {
        obj::<Conv>(d)?.math = m.0;
        Ok(())
    }

    pub(crate) unsafe fn cudnnGetConvolutionMathType(d: cudnnConvolutionDescriptor_t, m: *mut cudnnMathType_t) -> cudnnStatus_t {
        out(m, cudnnMathType_t(obj::<Conv>(d)?.math))
    }

    pub(crate) unsafe fn cudnnSetConvolutionReorderType(d: cudnnConvolutionDescriptor_t, r: cudnnReorderType_t) -> cudnnStatus_t {
        obj::<Conv>(d)?;
        // filters are never reordered here; only the default can be honoured
        if r == cudnnReorderType_t::CUDNN_DEFAULT_REORDER { Ok(()) } else { Err(cudnnError_t::NOT_SUPPORTED) }
    }

    pub(crate) unsafe fn cudnnGetConvolution2dForwardOutputDim(
        d: cudnnConvolutionDescriptor_t,
        x: cudnnTensorDescriptor_t,
        w: cudnnFilterDescriptor_t,
        n: *mut c_int,
        c: *mut c_int,
        h: *mut c_int,
        wo: *mut c_int,
    ) -> cudnnStatus_t {
        let (cv, t, f) = (obj::<Conv>(d)?, obj::<Tensor>(x)?, obj::<Filter>(w)?);
        if t.dims.len() != 4 || f.dims.len() != 4 {
            return Err(cudnnError_t::BAD_PARAM);
        }
        out(n, t.dims[0])?;
        out(c, f.dims[0])?;
        out(h, output_dim(t.dims[2], cv.pad[0], cv.stride[0], cv.dilation[0], f.dims[2]))?;
        out(wo, output_dim(t.dims[3], cv.pad[1], cv.stride[1], cv.dilation[1], f.dims[3]))
    }

    pub(crate) unsafe fn cudnnGetConvolutionNdForwardOutputDim(
        d: cudnnConvolutionDescriptor_t,
        x: cudnnTensorDescriptor_t,
        w: cudnnFilterDescriptor_t,
        nb: c_int,
        dims: *mut c_int,
    ) -> cudnnStatus_t {
        if nb != 4 || dims.is_null() {
            return Err(cudnnError_t::NOT_SUPPORTED);
        }
        cudnnGetConvolution2dForwardOutputDim(d, x, w, dims, dims.add(1), dims.add(2), dims.add(3))
    }

    // ---- algorithms: one per direction, no workspace

    unsafe fn one_algo<P>(requested: c_int, returned: *mut c_int, perf: *mut P, make: impl FnOnce() -> P) -> cudnnStatus_t {
        if requested < 1 || perf.is_null() {
            return Err(cudnnError_t::BAD_PARAM);
        }
        perf.write(make());
        out(returned, 1)
    }

    fn fwd_perf() -> cudnnConvolutionFwdAlgoPerf_t {
        cudnnConvolutionFwdAlgoPerf_t {
            algo: cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_IMPLICIT_GEMM,
            status: Ok(()),
            time: 1.0,
            memory: 0,
            determinism: cudnnDeterminism_t::CUDNN_DETERMINISTIC,
            mathType: cudnnMathType_t::CUDNN_DEFAULT_MATH,
            reserved: [0; 3],
        }
    }

    fn bwd_data_perf() -> cudnnConvolutionBwdDataAlgoPerf_t {
        cudnnConvolutionBwdDataAlgoPerf_t {
            algo: cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_1,
            status: Ok(()),
            time: 1.0,
            memory: 0,
            determinism: cudnnDeterminism_t::CUDNN_DETERMINISTIC,
            mathType: cudnnMathType_t::CUDNN_DEFAULT_MATH,
            reserved: [0; 3],
        }
    }

    fn bwd_filter_perf() -> cudnnConvolutionBwdFilterAlgoPerf_t {
        cudnnConvolutionBwdFilterAlgoPerf_t {
            algo: cudnnConvolutionBwdFilterAlgo_t::CUDNN_CONVOLUTION_BWD_FILTER_ALGO_1,
            status: Ok(()),
            time: 1.0,
            memory: 0,
            determinism: cudnnDeterminism_t::CUDNN_DETERMINISTIC,
            mathType: cudnnMathType_t::CUDNN_DEFAULT_MATH,
            reserved: [0; 3],
        }
    }

    /// The kernel arguments after checking the problem (x and y as the kernels name them).
    unsafe fn problem(
        x: cudnnTensorDescriptor_t,
        w: cudnnFilterDescriptor_t,
        c: cudnnConvolutionDescriptor_t,
        y: cudnnTensorDescriptor_t,
    ) -> Result<([i32; 25], bool), cudnnError_t> {
        let (x, w, c, y) = (obj::<Tensor>(x)?, obj::<Filter>(w)?, obj::<Conv>(c)?, obj::<Tensor>(y)?);
        Ok((kernel_shape(x, w, c, y)?, x.ty == cudnnDataType_t::CUDNN_DATA_HALF.0))
    }

    pub(crate) unsafe fn cudnnGetConvolutionForwardAlgorithmMaxCount(h: cudnnHandle_t, n: *mut c_int) -> cudnnStatus_t {
        obj::<Handle>(h)?;
        out(n, 1)
    }

    pub(crate) unsafe fn cudnnGetConvolutionBackwardDataAlgorithmMaxCount(h: cudnnHandle_t, n: *mut c_int) -> cudnnStatus_t {
        obj::<Handle>(h)?;
        out(n, 1)
    }

    pub(crate) unsafe fn cudnnGetConvolutionBackwardFilterAlgorithmMaxCount(h: cudnnHandle_t, n: *mut c_int) -> cudnnStatus_t {
        obj::<Handle>(h)?;
        out(n, 1)
    }

    pub(crate) unsafe fn cudnnGetConvolutionForwardAlgorithm_v7(
        _h: cudnnHandle_t,
        x: cudnnTensorDescriptor_t,
        w: cudnnFilterDescriptor_t,
        c: cudnnConvolutionDescriptor_t,
        y: cudnnTensorDescriptor_t,
        requested: c_int,
        returned: *mut c_int,
        perf: *mut cudnnConvolutionFwdAlgoPerf_t,
    ) -> cudnnStatus_t {
        problem(x, w, c, y)?;
        one_algo(requested, returned, perf, fwd_perf)
    }

    pub(crate) unsafe fn cudnnFindConvolutionForwardAlgorithm(
        h: cudnnHandle_t,
        x: cudnnTensorDescriptor_t,
        w: cudnnFilterDescriptor_t,
        c: cudnnConvolutionDescriptor_t,
        y: cudnnTensorDescriptor_t,
        requested: c_int,
        returned: *mut c_int,
        perf: *mut cudnnConvolutionFwdAlgoPerf_t,
    ) -> cudnnStatus_t {
        cudnnGetConvolutionForwardAlgorithm_v7(h, x, w, c, y, requested, returned, perf)
    }

    pub(crate) unsafe fn cudnnFindConvolutionForwardAlgorithmEx(
        h: cudnnHandle_t,
        x: cudnnTensorDescriptor_t,
        _xp: *const c_void,
        w: cudnnFilterDescriptor_t,
        _wp: *const c_void,
        c: cudnnConvolutionDescriptor_t,
        y: cudnnTensorDescriptor_t,
        _yp: *mut c_void,
        requested: c_int,
        returned: *mut c_int,
        perf: *mut cudnnConvolutionFwdAlgoPerf_t,
        _ws: *mut c_void,
        _ws_size: usize,
    ) -> cudnnStatus_t {
        cudnnGetConvolutionForwardAlgorithm_v7(h, x, w, c, y, requested, returned, perf)
    }

    pub(crate) unsafe fn cudnnGetConvolutionForwardWorkspaceSize(
        _h: cudnnHandle_t,
        x: cudnnTensorDescriptor_t,
        w: cudnnFilterDescriptor_t,
        c: cudnnConvolutionDescriptor_t,
        y: cudnnTensorDescriptor_t,
        _algo: cudnnConvolutionFwdAlgo_t,
        size: *mut usize,
    ) -> cudnnStatus_t {
        problem(x, w, c, y)?;
        out(size, 0)
    }

    pub(crate) unsafe fn cudnnGetConvolutionBackwardDataAlgorithm_v7(
        _h: cudnnHandle_t,
        w: cudnnFilterDescriptor_t,
        dy: cudnnTensorDescriptor_t,
        c: cudnnConvolutionDescriptor_t,
        dx: cudnnTensorDescriptor_t,
        requested: c_int,
        returned: *mut c_int,
        perf: *mut cudnnConvolutionBwdDataAlgoPerf_t,
    ) -> cudnnStatus_t {
        problem(dx, w, c, dy)?;
        one_algo(requested, returned, perf, bwd_data_perf)
    }

    pub(crate) unsafe fn cudnnFindConvolutionBackwardDataAlgorithm(
        h: cudnnHandle_t,
        w: cudnnFilterDescriptor_t,
        dy: cudnnTensorDescriptor_t,
        c: cudnnConvolutionDescriptor_t,
        dx: cudnnTensorDescriptor_t,
        requested: c_int,
        returned: *mut c_int,
        perf: *mut cudnnConvolutionBwdDataAlgoPerf_t,
    ) -> cudnnStatus_t {
        cudnnGetConvolutionBackwardDataAlgorithm_v7(h, w, dy, c, dx, requested, returned, perf)
    }

    pub(crate) unsafe fn cudnnFindConvolutionBackwardDataAlgorithmEx(
        h: cudnnHandle_t,
        w: cudnnFilterDescriptor_t,
        _wp: *const c_void,
        dy: cudnnTensorDescriptor_t,
        _dyp: *const c_void,
        c: cudnnConvolutionDescriptor_t,
        dx: cudnnTensorDescriptor_t,
        _dxp: *mut c_void,
        requested: c_int,
        returned: *mut c_int,
        perf: *mut cudnnConvolutionBwdDataAlgoPerf_t,
        _ws: *mut c_void,
        _ws_size: usize,
    ) -> cudnnStatus_t {
        cudnnGetConvolutionBackwardDataAlgorithm_v7(h, w, dy, c, dx, requested, returned, perf)
    }

    pub(crate) unsafe fn cudnnGetConvolutionBackwardDataWorkspaceSize(
        _h: cudnnHandle_t,
        w: cudnnFilterDescriptor_t,
        dy: cudnnTensorDescriptor_t,
        c: cudnnConvolutionDescriptor_t,
        dx: cudnnTensorDescriptor_t,
        _algo: cudnnConvolutionBwdDataAlgo_t,
        size: *mut usize,
    ) -> cudnnStatus_t {
        problem(dx, w, c, dy)?;
        out(size, 0)
    }

    pub(crate) unsafe fn cudnnGetConvolutionBackwardFilterAlgorithm_v7(
        _h: cudnnHandle_t,
        x: cudnnTensorDescriptor_t,
        dy: cudnnTensorDescriptor_t,
        c: cudnnConvolutionDescriptor_t,
        dw: cudnnFilterDescriptor_t,
        requested: c_int,
        returned: *mut c_int,
        perf: *mut cudnnConvolutionBwdFilterAlgoPerf_t,
    ) -> cudnnStatus_t {
        problem(x, dw, c, dy)?;
        one_algo(requested, returned, perf, bwd_filter_perf)
    }

    pub(crate) unsafe fn cudnnFindConvolutionBackwardFilterAlgorithm(
        h: cudnnHandle_t,
        x: cudnnTensorDescriptor_t,
        dy: cudnnTensorDescriptor_t,
        c: cudnnConvolutionDescriptor_t,
        dw: cudnnFilterDescriptor_t,
        requested: c_int,
        returned: *mut c_int,
        perf: *mut cudnnConvolutionBwdFilterAlgoPerf_t,
    ) -> cudnnStatus_t {
        cudnnGetConvolutionBackwardFilterAlgorithm_v7(h, x, dy, c, dw, requested, returned, perf)
    }

    pub(crate) unsafe fn cudnnFindConvolutionBackwardFilterAlgorithmEx(
        h: cudnnHandle_t,
        x: cudnnTensorDescriptor_t,
        _xp: *const c_void,
        dy: cudnnTensorDescriptor_t,
        _dyp: *const c_void,
        c: cudnnConvolutionDescriptor_t,
        dw: cudnnFilterDescriptor_t,
        _dwp: *mut c_void,
        requested: c_int,
        returned: *mut c_int,
        perf: *mut cudnnConvolutionBwdFilterAlgoPerf_t,
        _ws: *mut c_void,
        _ws_size: usize,
    ) -> cudnnStatus_t {
        cudnnGetConvolutionBackwardFilterAlgorithm_v7(h, x, dy, c, dw, requested, returned, perf)
    }

    pub(crate) unsafe fn cudnnGetConvolutionBackwardFilterWorkspaceSize(
        _h: cudnnHandle_t,
        x: cudnnTensorDescriptor_t,
        dy: cudnnTensorDescriptor_t,
        c: cudnnConvolutionDescriptor_t,
        dw: cudnnFilterDescriptor_t,
        _algo: cudnnConvolutionBwdFilterAlgo_t,
        size: *mut usize,
    ) -> cudnnStatus_t {
        problem(x, dw, c, dy)?;
        out(size, 0)
    }

    // ---- execution

    /// Whether the convolution `s` (and `extra` more elements, a temporary's) keeps every element offset and
    /// thread index below 2^31: agx_libs' `_i32` kernels (32-bit indices) then compute it.
    pub(crate) fn index32(s: &[i32; 25], extra: i64) -> bool {
        if s.iter().any(|&v| v < 0) {
            return false;
        }
        let v: Vec<i64> = s.iter().map(|&v| v as i64).collect();
        let [n, c, h, w, k, r, sw_, p, q, ph, pw, sh, sw, dh, dw, g, _, xsn, xsc, xsh, xsw, ysn, ysc, ysh, ysw] =
            v[..] else { return false };
        let _ = sw_;
        let last = |e: i64| (e - 1).max(0);
        let x = last(n) * xsn + last(c) * xsc + last(h) * xsh + last(w) * xsw;
        let y = last(n) * ysn + last(k) * ysc + last(p) * ysh + last(q) * ysw;
        let wt = k * (c / g.max(1)) * r * sw_;
        let rows = p * sh + r * dh + ph + h;
        let cols = q * sw + sw_ * dw + pw + w;
        let threads = (n * k * p * q).max(n * c * h * w).max(wt) + 256;
        [x, y, wt, rows, cols, threads, extra].iter().all(|&e| e < 1 << 31)
    }

    /// agx_libs' kernel `name`, its 32-bit index build when `narrow`.
    unsafe fn conv_function(name: &'static std::ffi::CStr, narrow: bool) -> Result<cuda_types::cuda::CUfunction, cudnnError_t> {
        let narrow_name: Option<&'static std::ffi::CStr> = match name.to_bytes() {
            b"agx_conv2d_fwd_f32" => Some(c"agx_conv2d_fwd_f32_i32"),
            b"agx_conv2d_fwd_f16" => Some(c"agx_conv2d_fwd_f16_i32"),
            b"agx_conv2d_bwd_data_f32" => Some(c"agx_conv2d_bwd_data_f32_i32"),
            b"agx_conv2d_bwd_data_f16" => Some(c"agx_conv2d_bwd_data_f16_i32"),
            b"agx_conv2d_bwd_filter_f32" => Some(c"agx_conv2d_bwd_filter_f32_i32"),
            b"agx_conv2d_bwd_filter_f16" => Some(c"agx_conv2d_bwd_filter_f16_i32"),
            b"agx_conv2d_im2col_f32" => Some(c"agx_conv2d_im2col_f32_i32"),
            b"agx_conv2d_im2col_f16" => Some(c"agx_conv2d_im2col_f16_i32"),
            b"agx_conv2d_col2im_f32" => Some(c"agx_conv2d_col2im_f32_i32"),
            b"agx_conv2d_col2im_f16" => Some(c"agx_conv2d_col2im_f16_i32"),
            _ => None,
        };
        match narrow_name {
            Some(n32) if narrow && std::env::var("ZLUDA_AGX_CONV_I32").map_or(true, |v| v != "0") => {
                engine::function(agx_libs::CONV_I32, n32).map_err(status)
            }
            _ => engine::function(agx_libs::CONV, name).map_err(status),
        }
    }

    /// Runs `kernel_<f32|f16>` over `elements` outputs. Scaling factors are host floats (cuDNN's rule for f32 and f16
    /// data).
    #[allow(clippy::too_many_arguments)]
    unsafe fn run(
        h: cudnnHandle_t,
        kernel: &str,
        shape: [i32; 25],
        half: bool,
        elements: u64,
        mut x: *const c_void,
        mut w: *const c_void,
        mut y: *const c_void,
        alpha: *const c_void,
        beta: *const c_void,
    ) -> cudnnStatus_t {
        let h = obj::<Handle>(h)?;
        if alpha.is_null() || beta.is_null() {
            return Err(cudnnError_t::BAD_PARAM);
        }
        let (mut alpha, mut beta) = (*(alpha as *const f32), *(beta as *const f32));
        if elements == 0 {
            return Ok(());
        }
        let name: &'static std::ffi::CStr = match (kernel, half) {
            ("fwd", false) => c"agx_conv2d_fwd_f32",
            ("fwd", true) => c"agx_conv2d_fwd_f16",
            ("bwd_data", false) => c"agx_conv2d_bwd_data_f32",
            ("bwd_data", true) => c"agx_conv2d_bwd_data_f16",
            ("bwd_filter", false) => c"agx_conv2d_bwd_filter_f32",
            _ => c"agx_conv2d_bwd_filter_f16",
        };
        let f = conv_function(name, index32(&shape, elements as i64))?;
        let mut shape = shape;
        let mut params: Vec<*mut c_void> =
            vec![&mut x as *mut _ as *mut c_void, &mut w as *mut _ as *mut c_void, &mut y as *mut _ as *mut c_void];
        params.extend(shape.iter_mut().map(|v| v as *mut i32 as *mut c_void));
        params.push(&mut alpha as *mut f32 as *mut c_void);
        params.push(&mut beta as *mut f32 as *mut c_void);
        let groups = u32::try_from(elements.div_ceil(256)).map_err(|_| cudnnError_t::NOT_SUPPORTED)?;
        engine::launch_kernel(f, [groups, 1], [256, 1], &mut params, h.stream).map_err(status)
    }

    pub(crate) unsafe fn cudnnConvolutionForward(
        h: cudnnHandle_t,
        alpha: *const c_void,
        xd: cudnnTensorDescriptor_t,
        x: *const c_void,
        wd: cudnnFilterDescriptor_t,
        w: *const c_void,
        cd: cudnnConvolutionDescriptor_t,
        _algo: cudnnConvolutionFwdAlgo_t,
        _ws: *mut c_void,
        _ws_size: usize,
        beta: *const c_void,
        yd: cudnnTensorDescriptor_t,
        y: *mut c_void,
    ) -> cudnnStatus_t {
        let (s, half) = problem(xd, wd, cd, yd)?;
        convolve(h, "fwd", s, half, x, w, y, alpha, beta)
    }

    /// Runs a convolution in direction `dir` ("fwd", "bwd_data", "bwd_filter"; x, w, y are dx or dw and dy as the
    /// direction has them): on agx_libs' GEMM path where it applies (`pointwise_gemm`, `im2col_gemm`,
    /// `col2im_gemm`), on the direct kernels otherwise.
    #[allow(clippy::too_many_arguments)]
    unsafe fn convolve(
        h: cudnnHandle_t,
        dir: &str,
        s: [i32; 25],
        half: bool,
        x: *const c_void,
        w: *const c_void,
        y: *const c_void,
        alpha: *const c_void,
        beta: *const c_void,
    ) -> cudnnStatus_t {
        let hd = obj::<Handle>(h)?;
        let (xm, wm, ym) = (x as *mut c_void, w as *mut c_void, y as *mut c_void);
        let elements = match dir {
            "fwd" => {
                if let Some(g) = pointwise_gemm(&s, half, x, w, ym, alpha, beta) {
                    return engine::gemm(&g, hd.stream).map_err(status);
                }
                if let Some(r) = im2col_gemm(hd, &s, half, x, wm, ym, alpha, beta, false) {
                    return r;
                }
                s[0] as u64 * s[4] as u64 * s[7] as u64 * s[8] as u64
            }
            "bwd_data" => {
                if let Some(r) = col2im_gemm(hd, &s, half, xm, w, y, alpha, beta) {
                    return r;
                }
                s[0] as u64 * s[1] as u64 * s[2] as u64 * s[3] as u64
            }
            _ => {
                if let Some(r) = im2col_gemm(hd, &s, half, x, wm, ym, alpha, beta, true) {
                    return r;
                }
                s[4] as u64 * (s[1] / s[15]) as u64 * s[5] as u64 * s[6] as u64
            }
        };
        run(h, dir, s, half, elements, x, w, y, alpha, beta)
    }

    /// A convolution with one group and dense planes of y (dy), large enough for the matrix unit: each image becomes
    /// its im2col matrix col (C R S x P Q, `agx_conv2d_im2col_*`, in a temporary buffer), then agx_libs' GEMM runs
    /// the forward y[n] (K x PQ) = w (K x C R S) col (column-major y[n]^T = col^T w^T), or, for the filter gradient
    /// (`filter`, w is dw and y is dy), dw (K x C R S) = sum over n of dy[n] (K x PQ) col^T (column-major
    /// dw^T = col dy[n]^T, beta 1 after the first image). None when it does not apply.
    #[allow(clippy::too_many_arguments)]
    unsafe fn im2col_gemm(
        h: &Handle,
        s: &[i32; 25],
        half: bool,
        mut x: *const c_void,
        w: *mut c_void,
        y: *mut c_void,
        alpha: *const c_void,
        beta: *const c_void,
        filter: bool,
    ) -> Option<cudnnStatus_t> {
        let [n, ch, _, _, k, r, sw, p, q, .., g, _, _, _, _, _, _, ys1, ys2, ys3] = *s;
        let (pq, crs) = (p as i64 * q as i64, ch as i64 * r as i64 * sw as i64);
        if g != 1 || (ys1 as i64, ys2, ys3) != (pq, q, 1) || k < 16 || crs < 16 || pq < 16 || alpha.is_null()
            || beta.is_null()
        {
            return None;
        }
        let elements = crs * pq;
        let groups = u32::try_from((elements as u64).div_ceil(256)).ok()?;
        let size = if half { 2 } else { 4 };
        Some((|| {
            let col = temp(h, elements * size)?;
            let name = if half { c"agx_conv2d_im2col_f16" } else { c"agx_conv2d_im2col_f32" };
            let f = conv_function(name, index32(s, elements))?;
            let (alpha, beta) = (*(alpha as *const f32) as f64, *(beta as *const f32) as f64);
            for image in 0..n {
                let mut colp = col.0;
                let mut shape: [i32; 22] = std::array::from_fn(|i| if i < 21 { s[i] } else { image });
                let mut params: Vec<*mut c_void> = vec![&mut x as *mut _ as *mut c_void, &mut colp as *mut _ as *mut c_void];
                params.extend(shape.iter_mut().map(|v| v as *mut i32 as *mut c_void));
                engine::launch_kernel(f, [groups, 1], [256, 1], &mut params, h.stream).map_err(status)?;
                let y_n = y.byte_offset(image as isize * s[21] as isize * size as isize);
                let types = if half { agx_libs::Types::H } else { agx_libs::Types::S };
                let g = if filter {
                    engine::Gemm {
                        types,
                        ta: true,
                        tb: false,
                        m: crs as u64,
                        n: k as u64,
                        k: pq as u64,
                        alpha,
                        beta: if image == 0 { beta } else { 1.0 },
                        a: col.0,
                        lda: pq as u64,
                        stride_a: 0,
                        b: y_n,
                        ldb: pq as u64,
                        stride_b: 0,
                        c: w,
                        ldc: crs as u64,
                        stride_c: 0,
                        batch: 1,
                    }
                } else {
                    engine::Gemm {
                        types,
                        ta: false,
                        tb: false,
                        m: pq as u64,
                        n: k as u64,
                        k: crs as u64,
                        alpha,
                        beta,
                        a: col.0,
                        lda: pq as u64,
                        stride_a: 0,
                        b: w,
                        ldb: crs as u64,
                        stride_b: 0,
                        c: y_n,
                        ldc: pq as u64,
                        stride_c: 0,
                        batch: 1,
                    }
                };
                engine::gemm(&g, h.stream).map_err(status)?;
            }
            Ok(())
        })())
    }

    /// The data gradient of a convolution with one group and dense dy planes, large enough for the matrix unit: per
    /// image, col (C R S x P Q, f32) = w^T (C R S x K) dy[n] (K x PQ) on agx_libs' GEMM (column-major col^T =
    /// dy[n]^T w, B transposed), then `agx_conv2d_col2im_*` gathers dx[n] from col. None when it does not apply.
    #[allow(clippy::too_many_arguments)]
    unsafe fn col2im_gemm(
        h: &Handle,
        s: &[i32; 25],
        half: bool,
        mut dx: *mut c_void,
        w: *const c_void,
        dy: *const c_void,
        alpha: *const c_void,
        beta: *const c_void,
    ) -> Option<cudnnStatus_t> {
        let [n, ch, hh, wd, k, r, sw, p, q, .., g, _, _, _, _, _, ys0, ys1, ys2, ys3] = *s;
        let (pq, crs) = (p as i64 * q as i64, ch as i64 * r as i64 * sw as i64);
        if g != 1 || (ys1 as i64, ys2, ys3) != (pq, q, 1) || k < 16 || crs < 16 || pq < 16 || alpha.is_null()
            || beta.is_null()
        {
            return None;
        }
        let groups = u32::try_from((ch as u64 * hh as u64 * wd as u64).div_ceil(256)).ok()?;
        let size = if half { 2 } else { 4 };
        Some((|| {
            let col = temp(h, crs * pq * 4)?;
            let name = if half { c"agx_conv2d_col2im_f16" } else { c"agx_conv2d_col2im_f32" };
            let f = conv_function(name, index32(s, crs * pq))?;
            let (mut alpha, mut beta) = (*(alpha as *const f32), *(beta as *const f32));
            for image in 0..n {
                let g = engine::Gemm {
                    types: if half { agx_libs::Types::HtoS } else { agx_libs::Types::S },
                    ta: false,
                    tb: true,
                    m: pq as u64,
                    n: crs as u64,
                    k: k as u64,
                    alpha: 1.0,
                    beta: 0.0,
                    a: dy.byte_offset(image as isize * ys0 as isize * size),
                    lda: pq as u64,
                    stride_a: 0,
                    b: w,
                    ldb: crs as u64,
                    stride_b: 0,
                    c: col.0,
                    ldc: pq as u64,
                    stride_c: 0,
                    batch: 1,
                };
                engine::gemm(&g, h.stream).map_err(status)?;
                let mut colp = col.0;
                let mut shape: [i32; 22] = std::array::from_fn(|i| if i < 21 { s[i] } else { image });
                let mut params: Vec<*mut c_void> = vec![&mut colp as *mut _ as *mut c_void, &mut dx as *mut _ as *mut c_void];
                params.extend(shape.iter_mut().map(|v| v as *mut i32 as *mut c_void));
                params.push(&mut alpha as *mut f32 as *mut c_void);
                params.push(&mut beta as *mut f32 as *mut c_void);
                engine::launch_kernel(f, [groups, 1], [256, 1], &mut params, h.stream).map_err(status)?;
            }
            Ok(())
        })())
    }

    /// A 1x1 convolution (stride 1, no padding, one group) of NCHW images whose planes are dense: per image n,
    /// y[n] (K x HW) = w (K x C) x[n] (C x HW), which agx_libs' GEMM runs on its tile kernel. Seen column-major, that
    /// is y[n]^T (HW x K, ld HW) = x[n]^T (HW x C, ld HW) w^T (C x K, ld C), batched over n.
    unsafe fn pointwise_gemm(
        s: &[i32; 25],
        half: bool,
        x: *const c_void,
        w: *const c_void,
        y: *mut c_void,
        alpha: *const c_void,
        beta: *const c_void,
    ) -> Option<engine::Gemm> {
        let [n, ch, hh, wd, k, r, sw, p, q, pad0, pad1, st0, st1, _, _, g, _, xs0, xs1, xs2, xs3, ys0, ys1, ys2, ys3] = *s;
        let hw = hh * wd;
        if r != 1 || sw != 1 || pad0 != 0 || pad1 != 0 || st0 != 1 || st1 != 1 || g != 1 || (p, q) != (hh, wd)
            || (xs1, xs2, xs3) != (hw, wd, 1) || (ys1, ys2, ys3) != (hw, wd, 1) || alpha.is_null() || beta.is_null()
        {
            return None;
        }
        Some(engine::Gemm {
            types: if half { agx_libs::Types::H } else { agx_libs::Types::S },
            ta: false,
            tb: false,
            m: hw as u64,
            n: k as u64,
            k: ch as u64,
            alpha: *(alpha as *const f32) as f64,
            beta: *(beta as *const f32) as f64,
            a: x,
            lda: hw as u64,
            stride_a: xs0 as i64,
            b: w,
            ldb: ch as u64,
            stride_b: 0,
            c: y,
            ldc: hw as u64,
            stride_c: ys0 as i64,
            batch: n as u64,
        })
    }

    pub(crate) unsafe fn cudnnConvolutionBackwardData(
        h: cudnnHandle_t,
        alpha: *const c_void,
        wd: cudnnFilterDescriptor_t,
        w: *const c_void,
        dyd: cudnnTensorDescriptor_t,
        dy: *const c_void,
        cd: cudnnConvolutionDescriptor_t,
        _algo: cudnnConvolutionBwdDataAlgo_t,
        _ws: *mut c_void,
        _ws_size: usize,
        beta: *const c_void,
        dxd: cudnnTensorDescriptor_t,
        dx: *mut c_void,
    ) -> cudnnStatus_t {
        let (s, half) = problem(dxd, wd, cd, dyd)?;
        convolve(h, "bwd_data", s, half, dx, w, dy, alpha, beta)
    }

    pub(crate) unsafe fn cudnnConvolutionBackwardFilter(
        h: cudnnHandle_t,
        alpha: *const c_void,
        xd: cudnnTensorDescriptor_t,
        x: *const c_void,
        dyd: cudnnTensorDescriptor_t,
        dy: *const c_void,
        cd: cudnnConvolutionDescriptor_t,
        _algo: cudnnConvolutionBwdFilterAlgo_t,
        _ws: *mut c_void,
        _ws_size: usize,
        beta: *const c_void,
        dwd: cudnnFilterDescriptor_t,
        dw: *mut c_void,
    ) -> cudnnStatus_t {
        let (s, half) = problem(xd, dwd, cd, dyd)?;
        convolve(h, "bwd_filter", s, half, x, dw, dy, alpha, beta)
    }

    // ---- the backend (graph) API

    mod graph;

    unsafe fn backend<'a>(d: cudnnBackendDescriptor_t) -> Result<&'a mut graph::Desc, cudnnError_t> {
        (d as *mut graph::Desc).as_mut().ok_or(cudnnError_t::BAD_PARAM)
    }

    pub(crate) unsafe fn cudnnBackendCreateDescriptor(ty: cudnnBackendDescriptorType_t, d: *mut cudnnBackendDescriptor_t) -> cudnnStatus_t {
        let desc = graph::create(ty.0)?;
        out(d, Box::into_raw(desc) as cudnnBackendDescriptor_t)
    }

    pub(crate) unsafe fn cudnnBackendDestroyDescriptor(d: cudnnBackendDescriptor_t) -> cudnnStatus_t {
        if !d.is_null() {
            drop(Box::from_raw(d as *mut graph::Desc));
        }
        Ok(())
    }

    pub(crate) unsafe fn cudnnBackendInitialize(d: cudnnBackendDescriptor_t) -> cudnnStatus_t {
        backend(d).map(|_| ())
    }

    pub(crate) unsafe fn cudnnBackendFinalize(d: cudnnBackendDescriptor_t) -> cudnnStatus_t {
        graph::finalize(backend(d)?)
    }

    pub(crate) unsafe fn cudnnBackendSetAttribute(
        d: cudnnBackendDescriptor_t,
        name: cudnnBackendAttributeName_t,
        ty: cudnnBackendAttributeType_t,
        count: i64,
        data: *const c_void,
    ) -> cudnnStatus_t {
        graph::set_attribute(backend(d)?, name.0, ty.0, count, data)
    }

    pub(crate) unsafe fn cudnnBackendGetAttribute(
        d: cudnnBackendDescriptor_t,
        name: cudnnBackendAttributeName_t,
        ty: cudnnBackendAttributeType_t,
        requested: i64,
        count: *mut i64,
        data: *mut c_void,
    ) -> cudnnStatus_t {
        graph::get_attribute(backend(d)?, name.0, ty.0, requested, count, data)
    }

    pub(crate) unsafe fn cudnnBackendExecute(h: cudnnHandle_t, plan: cudnnBackendDescriptor_t, pack: cudnnBackendDescriptor_t) -> cudnnStatus_t {
        graph::execute(h, backend(plan)?, backend(pack)?)
    }

    // ---- pointwise, softmax, pooling and batch normalization (agx_libs/kernels/dnn.mlir), f32 tensors

    fn arg<T>(v: &mut T) -> *mut c_void {
        v as *mut T as *mut c_void
    }

    /// alpha / beta: host f32 for f32 tensors.
    unsafe fn f32_scalar(p: *const c_void) -> Result<f32, cudnnError_t> {
        (p as *const f32).as_ref().copied().ok_or(cudnnError_t::BAD_PARAM)
    }

    unsafe fn tensor_view(d: cudnnTensorDescriptor_t) -> Result<View, cudnnError_t> {
        view(obj::<Tensor>(d)?)
    }

    /// One launch of 64-thread groups covering `threads`, with `params` the kernel's arguments in order.
    unsafe fn dnn_launch(h: &Handle, name: &'static std::ffi::CStr, params: &mut [*mut c_void], threads: i64) -> cudnnStatus_t {
        if threads <= 0 {
            return Ok(());
        }
        let groups = u32::try_from((threads as u64).div_ceil(64)).map_err(|_| cudnnError_t::NOT_SUPPORTED)?;
        let f = engine::function(agx_libs::DNN, name).map_err(status)?;
        engine::launch_kernel(f, [groups, 1], [64, 1], params, h.stream).map_err(status)
    }

    /// Kernel arguments: the shape C H W after `total`, then each view's strides.
    struct Args(Vec<i64>);

    impl Args {
        fn shape(v: &View) -> Args {
            Args(vec![v.dims[1], v.dims[2], v.dims[3]])
        }
        fn strides(mut self, v: &View) -> Args {
            self.0.extend_from_slice(&v.strides);
            self
        }
        fn push(&mut self, v: i64) {
            self.0.push(v);
        }
    }

    /// f16 and bf16 tensors of a call through packed f32 copies, so the f32 kernels run them: each tensor
    /// (descriptor, pointer, read, written) of the list becomes an f32 descriptor and buffer; `finish` converts the
    /// written ones back.
    struct Staging {
        _descs: Vec<Box<Tensor>>,
        back: Vec<(View, View, *mut c_void, *mut c_void, i64)>,
        temps: Vec<Temp>,
    }

    /// The f16 (0) or bf16 (1) kind of a tensor type, None for f32.
    fn half_kind(ty: u32) -> Result<Option<i64>, cudnnError_t> {
        match ty {
            t if t == cudnnDataType_t::CUDNN_DATA_FLOAT.0 => Ok(None),
            t if t == cudnnDataType_t::CUDNN_DATA_HALF.0 => Ok(Some(0)),
            t if t == cudnnDataType_t::CUDNN_DATA_BFLOAT16.0 => Ok(Some(1)),
            _ => Err(cudnnError_t::NOT_SUPPORTED),
        }
    }

    unsafe fn convert(h: &Handle, name: &'static std::ffi::CStr, mut src: *mut c_void, sv: &View, mut dst: *mut c_void, dv: &View, mut kind: i64) -> cudnnStatus_t {
        let mut total = sv.total();
        let mut a = Args::shape(sv).strides(sv).strides(dv);
        let mut p = vec![arg(&mut src), arg(&mut dst), arg(&mut total)];
        p.extend(a.0.iter_mut().map(|v| arg(v)));
        p.push(arg(&mut kind));
        dnn_launch(h, name, &mut p, total)
    }

    /// None when every tensor is f32, else the staging and the f32 descriptors and pointers to call with.
    unsafe fn stage(
        h: cudnnHandle_t,
        tensors: &[(cudnnTensorDescriptor_t, *const c_void, bool, bool)],
    ) -> Result<Option<(Staging, Vec<(cudnnTensorDescriptor_t, *mut c_void)>)>, cudnnError_t> {
        let mut any = false;
        for &(d, _, _, _) in tensors {
            any |= half_kind(obj::<Tensor>(d)?.ty)?.is_some();
        }
        if !any {
            return Ok(None);
        }
        let handle = obj::<Handle>(h)?;
        let mut st = Staging { _descs: Vec::new(), back: Vec::new(), temps: Vec::new() };
        let mut calls = Vec::new();
        for &(d, p, read, write) in tensors {
            let t = obj::<Tensor>(d)?.clone();
            let Some(kind) = half_kind(t.ty)? else {
                calls.push((d, p as *mut c_void));
                continue;
            };
            let orig = view(&Tensor { ty: cudnnDataType_t::CUDNN_DATA_FLOAT.0, ..t.clone() })?;
            let mut desc = Box::new(Tensor { ty: cudnnDataType_t::CUDNN_DATA_FLOAT.0, strides: packed_strides(&t.dims, false), dims: t.dims.clone() });
            let packed = view(&desc)?;
            let tmp = temp(handle, 4 * packed.total())?;
            if read && !p.is_null() {
                convert(handle, c"agx_dnn_to_f32", p as *mut c_void, &orig, tmp.0, &packed, kind)?;
            }
            if write {
                st.back.push((packed, orig, tmp.0, p as *mut c_void, kind));
            }
            calls.push((&mut *desc as *mut Tensor as cudnnTensorDescriptor_t, tmp.0));
            st._descs.push(desc);
            st.temps.push(tmp);
        }
        Ok(Some((st, calls)))
    }

    impl Staging {
        unsafe fn finish(self, h: cudnnHandle_t, result: cudnnStatus_t) -> cudnnStatus_t {
            result?;
            let handle = obj::<Handle>(h)?;
            for (packed, orig, tmp, p, kind) in &self.back {
                convert(handle, c"agx_dnn_from_f32", *tmp, packed, *p, orig, *kind)?;
            }
            Ok(())
        }
    }

    /// beta != 0: the output is read too.
    unsafe fn reads_output(beta: *const c_void) -> Result<bool, cudnnError_t> {
        Ok(f32_scalar(beta)? != 0.0)
    }

    pub(crate) unsafe fn cudnnCreateActivationDescriptor(d: *mut cudnnActivationDescriptor_t) -> cudnnStatus_t {
        create::<Activation, _>(d)
    }

    pub(crate) unsafe fn cudnnDestroyActivationDescriptor(d: cudnnActivationDescriptor_t) -> cudnnStatus_t {
        destroy::<Activation, _>(d)
    }

    pub(crate) unsafe fn cudnnSetActivationDescriptor(
        d: cudnnActivationDescriptor_t,
        mode: cudnnActivationMode_t,
        nan: cudnnNanPropagation_t,
        coef: f64,
    ) -> cudnnStatus_t {
        let a = obj::<Activation>(d)?;
        a.mode = mode.0;
        a.nan = nan.0;
        a.coef = coef;
        Ok(())
    }

    pub(crate) unsafe fn cudnnGetActivationDescriptor(
        d: cudnnActivationDescriptor_t,
        mode: *mut cudnnActivationMode_t,
        nan: *mut cudnnNanPropagation_t,
        coef: *mut f64,
    ) -> cudnnStatus_t {
        let a = obj::<Activation>(d)?;
        out(mode, cudnnActivationMode_t(a.mode))?;
        out(nan, cudnnNanPropagation_t(a.nan))?;
        out(coef, a.coef)
    }

    pub(crate) unsafe fn cudnnSetActivationDescriptorSwishBeta(d: cudnnActivationDescriptor_t, beta: f64) -> cudnnStatus_t {
        obj::<Activation>(d)?.swish_beta = beta;
        Ok(())
    }

    pub(crate) unsafe fn cudnnActivationForward(
        h: cudnnHandle_t,
        d: cudnnActivationDescriptor_t,
        alpha: *const c_void,
        x_desc: cudnnTensorDescriptor_t,
        mut x: *const c_void,
        beta: *const c_void,
        y_desc: cudnnTensorDescriptor_t,
        mut y: *mut c_void,
    ) -> cudnnStatus_t {
        if let Some((st, t)) = stage(h, &[(x_desc, x, true, false), (y_desc, y, reads_output(beta)?, true)])? {
            return st.finish(h, cudnnActivationForward(h, d, alpha, t[0].0, t[0].1, beta, t[1].0, t[1].1));
        }
        let h = obj::<Handle>(h)?;
        let (mut mode, mut coef) = activation_args(obj::<Activation>(d)?)?;
        let (xv, yv) = (tensor_view(x_desc)?, tensor_view(y_desc)?);
        if xv.dims != yv.dims {
            return Err(cudnnError_t::BAD_PARAM);
        }
        let (mut alpha, mut beta) = (f32_scalar(alpha)?, f32_scalar(beta)?);
        let mut total = xv.total();
        let mut a = Args::shape(&xv).strides(&xv).strides(&yv);
        a.push(mode);
        let mut p = vec![arg(&mut x), arg(&mut y), arg(&mut total)];
        p.extend(a.0.iter_mut().map(|v| arg(v)));
        p.extend([arg(&mut coef), arg(&mut alpha), arg(&mut beta)]);
        let _ = &mut mode;
        dnn_launch(h, c"agx_dnn_act_fwd", &mut p, total)
    }

    pub(crate) unsafe fn cudnnActivationBackward(
        h: cudnnHandle_t,
        d: cudnnActivationDescriptor_t,
        alpha: *const c_void,
        y_desc: cudnnTensorDescriptor_t,
        mut y: *const c_void,
        dy_desc: cudnnTensorDescriptor_t,
        mut dy: *const c_void,
        x_desc: cudnnTensorDescriptor_t,
        mut x: *const c_void,
        beta: *const c_void,
        dx_desc: cudnnTensorDescriptor_t,
        mut dx: *mut c_void,
    ) -> cudnnStatus_t {
        if let Some((st, t)) = stage(h, &[(y_desc, y, true, false), (dy_desc, dy, true, false), (x_desc, x, true, false), (dx_desc, dx, reads_output(beta)?, true)])? {
            return st.finish(h, cudnnActivationBackward(h, d, alpha, t[0].0, t[0].1, t[1].0, t[1].1, t[2].0, t[2].1, beta, t[3].0, t[3].1));
        }
        let h = obj::<Handle>(h)?;
        let (mode, mut coef) = activation_args(obj::<Activation>(d)?)?;
        let views = [tensor_view(y_desc)?, tensor_view(dy_desc)?, tensor_view(x_desc)?, tensor_view(dx_desc)?];
        if views.iter().any(|v| v.dims != views[0].dims) {
            return Err(cudnnError_t::BAD_PARAM);
        }
        let (mut alpha, mut beta) = (f32_scalar(alpha)?, f32_scalar(beta)?);
        let mut total = views[0].total();
        let mut a = Args::shape(&views[0]).strides(&views[0]).strides(&views[1]).strides(&views[2]).strides(&views[3]);
        a.push(mode);
        let mut p = vec![arg(&mut y), arg(&mut dy), arg(&mut x), arg(&mut dx), arg(&mut total)];
        p.extend(a.0.iter_mut().map(|v| arg(v)));
        p.extend([arg(&mut coef), arg(&mut alpha), arg(&mut beta)]);
        dnn_launch(h, c"agx_dnn_act_bwd", &mut p, total)
    }

    unsafe fn pointwise(h: cudnnHandle_t, a_view: Option<View>, mut a: *const c_void, c_desc: cudnnTensorDescriptor_t, mut c: *mut c_void, op: i64, mut alpha: f32, mut beta: f32) -> cudnnStatus_t {
        let h = obj::<Handle>(h)?;
        let cv = tensor_view(c_desc)?;
        let astr = match a_view {
            Some(av) => broadcast(&av, &cv)?,
            None => {
                a = c;
                [0; 4]
            }
        };
        let mut total = cv.total();
        let mut args = Args::shape(&cv);
        args.0.extend_from_slice(&astr);
        let mut args = args.strides(&cv);
        args.push(op);
        let mut p = vec![arg(&mut a), arg(&mut c), arg(&mut total)];
        p.extend(args.0.iter_mut().map(|v| arg(v)));
        p.extend([arg(&mut alpha), arg(&mut beta)]);
        dnn_launch(h, c"agx_dnn_pointwise", &mut p, total)
    }

    pub(crate) unsafe fn cudnnAddTensor(
        h: cudnnHandle_t,
        alpha: *const c_void,
        a_desc: cudnnTensorDescriptor_t,
        a: *const c_void,
        beta: *const c_void,
        c_desc: cudnnTensorDescriptor_t,
        c: *mut c_void,
    ) -> cudnnStatus_t {
        if let Some((st, t)) = stage(h, &[(a_desc, a, true, false), (c_desc, c, reads_output(beta)?, true)])? {
            return st.finish(h, cudnnAddTensor(h, alpha, t[0].0, t[0].1, beta, t[1].0, t[1].1));
        }
        pointwise(h, Some(tensor_view(a_desc)?), a, c_desc, c, 0, f32_scalar(alpha)?, f32_scalar(beta)?)
    }

    pub(crate) unsafe fn cudnnSetTensor(h: cudnnHandle_t, y_desc: cudnnTensorDescriptor_t, y: *mut c_void, value: *const c_void) -> cudnnStatus_t {
        if let Some((st, t)) = stage(h, &[(y_desc, y, false, true)])? {
            return st.finish(h, cudnnSetTensor(h, t[0].0, t[0].1, value));
        }
        pointwise(h, None, std::ptr::null(), y_desc, y, 1, f32_scalar(value)?, 0.0)
    }

    pub(crate) unsafe fn cudnnScaleTensor(h: cudnnHandle_t, y_desc: cudnnTensorDescriptor_t, y: *mut c_void, alpha: *const c_void) -> cudnnStatus_t {
        if let Some((st, t)) = stage(h, &[(y_desc, y, true, true)])? {
            return st.finish(h, cudnnScaleTensor(h, t[0].0, t[0].1, alpha));
        }
        pointwise(h, None, std::ptr::null(), y_desc, y, 2, f32_scalar(alpha)?, 0.0)
    }

    /// The softmax rows and row length, and the instance flag, of `v` in `mode`.
    fn softmax_rows(v: &View, mode: cudnnSoftmaxMode_t) -> Result<(i64, i64, i64), cudnnError_t> {
        let [n, c, hh, w] = v.dims;
        match mode {
            cudnnSoftmaxMode_t::CUDNN_SOFTMAX_MODE_INSTANCE => Ok((n, c * hh * w, 1)),
            cudnnSoftmaxMode_t::CUDNN_SOFTMAX_MODE_CHANNEL => Ok((n * hh * w, c, 0)),
            _ => Err(cudnnError_t::BAD_PARAM),
        }
    }

    fn softmax_algo(a: cudnnSoftmaxAlgorithm_t) -> Result<i64, cudnnError_t> {
        if a.0 <= 2 {
            Ok(a.0 as i64)
        } else {
            Err(cudnnError_t::BAD_PARAM)
        }
    }

    pub(crate) unsafe fn cudnnSoftmaxForward(
        h: cudnnHandle_t,
        algo: cudnnSoftmaxAlgorithm_t,
        mode: cudnnSoftmaxMode_t,
        alpha: *const c_void,
        x_desc: cudnnTensorDescriptor_t,
        mut x: *const c_void,
        beta: *const c_void,
        y_desc: cudnnTensorDescriptor_t,
        mut y: *mut c_void,
    ) -> cudnnStatus_t {
        if let Some((st, t)) = stage(h, &[(x_desc, x, true, false), (y_desc, y, reads_output(beta)?, true)])? {
            return st.finish(h, cudnnSoftmaxForward(h, algo, mode, alpha, t[0].0, t[0].1, beta, t[1].0, t[1].1));
        }
        let h = obj::<Handle>(h)?;
        let (xv, yv) = (tensor_view(x_desc)?, tensor_view(y_desc)?);
        if xv.dims != yv.dims {
            return Err(cudnnError_t::BAD_PARAM);
        }
        let (mut rows, mut k, instance) = softmax_rows(&xv, mode)?;
        let algo = softmax_algo(algo)?;
        let (mut alpha, mut beta) = (f32_scalar(alpha)?, f32_scalar(beta)?);
        let mut a = Args::shape(&xv).strides(&xv).strides(&yv);
        a.push(instance);
        a.push(algo);
        let mut p = vec![arg(&mut x), arg(&mut y), arg(&mut rows), arg(&mut k)];
        p.extend(a.0.iter_mut().map(|v| arg(v)));
        p.extend([arg(&mut alpha), arg(&mut beta)]);
        dnn_launch(h, c"agx_dnn_softmax_fwd", &mut p, rows * 32)
    }

    pub(crate) unsafe fn cudnnSoftmaxBackward(
        h: cudnnHandle_t,
        algo: cudnnSoftmaxAlgorithm_t,
        mode: cudnnSoftmaxMode_t,
        alpha: *const c_void,
        y_desc: cudnnTensorDescriptor_t,
        mut y: *const c_void,
        dy_desc: cudnnTensorDescriptor_t,
        mut dy: *const c_void,
        beta: *const c_void,
        dx_desc: cudnnTensorDescriptor_t,
        mut dx: *mut c_void,
    ) -> cudnnStatus_t {
        if let Some((st, t)) = stage(h, &[(y_desc, y, true, false), (dy_desc, dy, true, false), (dx_desc, dx, reads_output(beta)?, true)])? {
            return st.finish(h, cudnnSoftmaxBackward(h, algo, mode, alpha, t[0].0, t[0].1, t[1].0, t[1].1, beta, t[2].0, t[2].1));
        }
        let h = obj::<Handle>(h)?;
        let (yv, dyv, dxv) = (tensor_view(y_desc)?, tensor_view(dy_desc)?, tensor_view(dx_desc)?);
        if yv.dims != dyv.dims || yv.dims != dxv.dims {
            return Err(cudnnError_t::BAD_PARAM);
        }
        let (mut rows, mut k, instance) = softmax_rows(&yv, mode)?;
        let algo = softmax_algo(algo)?;
        let (mut alpha, mut beta) = (f32_scalar(alpha)?, f32_scalar(beta)?);
        let mut a = Args::shape(&yv).strides(&yv).strides(&dyv).strides(&dxv);
        a.push(instance);
        a.push(algo);
        let mut p = vec![arg(&mut y), arg(&mut dy), arg(&mut dx), arg(&mut rows), arg(&mut k)];
        p.extend(a.0.iter_mut().map(|v| arg(v)));
        p.extend([arg(&mut alpha), arg(&mut beta)]);
        dnn_launch(h, c"agx_dnn_softmax_bwd", &mut p, rows * 32)
    }

    pub(crate) unsafe fn cudnnCreatePoolingDescriptor(d: *mut cudnnPoolingDescriptor_t) -> cudnnStatus_t {
        create::<Pooling, _>(d)
    }

    pub(crate) unsafe fn cudnnDestroyPoolingDescriptor(d: cudnnPoolingDescriptor_t) -> cudnnStatus_t {
        destroy::<Pooling, _>(d)
    }

    pub(crate) unsafe fn cudnnSetPooling2dDescriptor(
        d: cudnnPoolingDescriptor_t,
        mode: cudnnPoolingMode_t,
        nan: cudnnNanPropagation_t,
        wh: c_int,
        ww: c_int,
        ph: c_int,
        pw: c_int,
        sh: c_int,
        sw: c_int,
    ) -> cudnnStatus_t {
        if mode.0 > 3 {
            return Err(cudnnError_t::BAD_PARAM);
        }
        if wh < 1 || ww < 1 || ph < 0 || pw < 0 || sh < 1 || sw < 1 {
            return Err(cudnnError_t::BAD_PARAM);
        }
        *obj::<Pooling>(d)? = Pooling { mode: mode.0, nan: nan.0, window: [wh, ww], pad: [ph, pw], stride: [sh, sw] };
        Ok(())
    }

    pub(crate) unsafe fn cudnnGetPooling2dDescriptor(
        d: cudnnPoolingDescriptor_t,
        mode: *mut cudnnPoolingMode_t,
        nan: *mut cudnnNanPropagation_t,
        wh: *mut c_int,
        ww: *mut c_int,
        ph: *mut c_int,
        pw: *mut c_int,
        sh: *mut c_int,
        sw: *mut c_int,
    ) -> cudnnStatus_t {
        let p = obj::<Pooling>(d)?.clone();
        out(mode, cudnnPoolingMode_t(p.mode))?;
        out(nan, cudnnNanPropagation_t(p.nan))?;
        out(wh, p.window[0])?;
        out(ww, p.window[1])?;
        out(ph, p.pad[0])?;
        out(pw, p.pad[1])?;
        out(sh, p.stride[0])?;
        out(sw, p.stride[1])
    }

    pub(crate) unsafe fn cudnnGetPooling2dForwardOutputDim(
        d: cudnnPoolingDescriptor_t,
        x_desc: cudnnTensorDescriptor_t,
        n: *mut c_int,
        c: *mut c_int,
        h: *mut c_int,
        w: *mut c_int,
    ) -> cudnnStatus_t {
        let p = obj::<Pooling>(d)?;
        let t = obj::<Tensor>(x_desc)?;
        if t.dims.len() != 4 {
            return Err(cudnnError_t::BAD_PARAM);
        }
        let (oh, ow) = pool_output(p, t.dims[2] as i64, t.dims[3] as i64);
        out(n, t.dims[0])?;
        out(c, t.dims[1])?;
        out(h, oh as c_int)?;
        out(w, ow as c_int)
    }

    /// The window arguments C H W P Q R S pads strides of a pooling launch.
    fn pool_args(p: &Pooling, x: &View, y: &View) -> Result<Args, cudnnError_t> {
        let [n, c, hh, w] = x.dims;
        let (oh, ow) = pool_output(p, hh, w);
        if y.dims != [n, c, oh, ow] {
            return Err(cudnnError_t::BAD_PARAM);
        }
        let mut a = Args::shape(x);
        a.0.extend([oh, ow, p.window[0] as i64, p.window[1] as i64, p.pad[0] as i64, p.pad[1] as i64, p.stride[0] as i64, p.stride[1] as i64]);
        Ok(a)
    }

    pub(crate) unsafe fn cudnnPoolingForward(
        h: cudnnHandle_t,
        d: cudnnPoolingDescriptor_t,
        alpha: *const c_void,
        x_desc: cudnnTensorDescriptor_t,
        mut x: *const c_void,
        beta: *const c_void,
        y_desc: cudnnTensorDescriptor_t,
        mut y: *mut c_void,
    ) -> cudnnStatus_t {
        if let Some((st, t)) = stage(h, &[(x_desc, x, true, false), (y_desc, y, reads_output(beta)?, true)])? {
            return st.finish(h, cudnnPoolingForward(h, d, alpha, t[0].0, t[0].1, beta, t[1].0, t[1].1));
        }
        let h = obj::<Handle>(h)?;
        let pool = obj::<Pooling>(d)?.clone();
        let (xv, yv) = (tensor_view(x_desc)?, tensor_view(y_desc)?);
        let mut a = pool_args(&pool, &xv, &yv)?.strides(&xv).strides(&yv);
        a.push(pool.mode as i64);
        let (mut alpha, mut beta) = (f32_scalar(alpha)?, f32_scalar(beta)?);
        let mut total = yv.total();
        let mut p = vec![arg(&mut x), arg(&mut y), arg(&mut total)];
        p.extend(a.0.iter_mut().map(|v| arg(v)));
        p.extend([arg(&mut alpha), arg(&mut beta)]);
        dnn_launch(h, c"agx_dnn_pool_fwd", &mut p, total)
    }

    /// Max pooling gradients go to the first maximum of each window (the deterministic choice), whatever y holds.
    pub(crate) unsafe fn cudnnPoolingBackward(
        h: cudnnHandle_t,
        d: cudnnPoolingDescriptor_t,
        alpha: *const c_void,
        y_desc: cudnnTensorDescriptor_t,
        _y: *const c_void,
        dy_desc: cudnnTensorDescriptor_t,
        mut dy: *const c_void,
        x_desc: cudnnTensorDescriptor_t,
        mut x: *const c_void,
        beta: *const c_void,
        dx_desc: cudnnTensorDescriptor_t,
        mut dx: *mut c_void,
    ) -> cudnnStatus_t {
        if let Some((st, t)) = stage(h, &[(y_desc, _y, false, false), (dy_desc, dy, true, false), (x_desc, x, true, false), (dx_desc, dx, reads_output(beta)?, true)])? {
            return st.finish(h, cudnnPoolingBackward(h, d, alpha, t[0].0, t[0].1, t[1].0, t[1].1, t[2].0, t[2].1, beta, t[3].0, t[3].1));
        }
        let h = obj::<Handle>(h)?;
        let pool = obj::<Pooling>(d)?.clone();
        let (yv, dyv, xv, dxv) = (tensor_view(y_desc)?, tensor_view(dy_desc)?, tensor_view(x_desc)?, tensor_view(dx_desc)?);
        if yv.dims != dyv.dims || xv.dims != dxv.dims {
            return Err(cudnnError_t::BAD_PARAM);
        }
        let mut a = pool_args(&pool, &xv, &dyv)?.strides(&xv).strides(&dyv).strides(&dxv);
        a.push(pool.mode as i64);
        let (mut alpha, mut beta) = (f32_scalar(alpha)?, f32_scalar(beta)?);
        let mut total = xv.total();
        let mut p = vec![arg(&mut dy), arg(&mut x), arg(&mut dx), arg(&mut total)];
        p.extend(a.0.iter_mut().map(|v| arg(v)));
        p.extend([arg(&mut alpha), arg(&mut beta)]);
        dnn_launch(h, c"agx_dnn_pool_bwd", &mut p, total)
    }

    /// 1 for spatial batch normalization (parameters per channel), 0 per activation.
    fn bn_spatial(mode: cudnnBatchNormMode_t) -> Result<i64, cudnnError_t> {
        match mode.0 {
            0 => Ok(0),
            1 | 2 => Ok(1),
            _ => Err(cudnnError_t::BAD_PARAM),
        }
    }

    /// The parameter count and the reduction length of batch normalization over `x`.
    fn bn_sizes(x: &View, spatial: i64) -> (i64, i64) {
        let [n, c, hh, w] = x.dims;
        if spatial == 1 {
            (c, n * hh * w)
        } else {
            (c * hh * w, n)
        }
    }

    pub(crate) unsafe fn cudnnDeriveBNTensorDescriptor(d: cudnnTensorDescriptor_t, x_desc: cudnnTensorDescriptor_t, mode: cudnnBatchNormMode_t) -> cudnnStatus_t {
        let x = obj::<Tensor>(x_desc)?.clone();
        if x.dims.len() < 4 {
            return Err(cudnnError_t::BAD_PARAM);
        }
        let mut dims = vec![1; x.dims.len()];
        dims[1] = x.dims[1];
        if bn_spatial(mode)? == 0 {
            dims[2..].copy_from_slice(&x.dims[2..]);
        }
        let ty = if x.ty == cudnnDataType_t::CUDNN_DATA_HALF.0 || x.ty == cudnnDataType_t::CUDNN_DATA_BFLOAT16.0 {
            cudnnDataType_t::CUDNN_DATA_FLOAT.0
        } else {
            x.ty
        };
        *obj::<Tensor>(d)? = Tensor { ty, strides: packed_strides(&dims, false), dims };
        Ok(())
    }

    /// Device memory released after the handle's stream finishes with it.
    struct Temp(*mut c_void, cudaStream_t);

    impl Drop for Temp {
        fn drop(&mut self) {
            unsafe {
                let _ = engine::synchronize(self.1);
                let _ = engine::free(self.0);
            }
        }
    }

    unsafe fn temp(h: &Handle, bytes: i64) -> Result<Temp, cudnnError_t> {
        Ok(Temp(engine::alloc(bytes.max(4) as usize).map_err(status)?, h.stream))
    }

    #[allow(clippy::too_many_arguments)]
    unsafe fn bn_apply(h: &Handle, xv: &View, mut x: *const c_void, yv: &View, mut y: *mut c_void, mut scale: *const c_void, mut bias: *const c_void, mut mean: *const c_void, mut var: *const c_void, spatial: i64, invstd_given: i64, mut eps: f32, mut alpha: f32, mut beta: f32) -> cudnnStatus_t {
        let mut total = xv.total();
        let mut a = Args::shape(xv).strides(xv).strides(yv);
        a.push(spatial);
        a.push(invstd_given);
        let mut p = vec![arg(&mut x), arg(&mut y), arg(&mut scale), arg(&mut bias), arg(&mut mean), arg(&mut var), arg(&mut total)];
        p.extend(a.0.iter_mut().map(|v| arg(v)));
        p.extend([arg(&mut eps), arg(&mut alpha), arg(&mut beta)]);
        dnn_launch(h, c"agx_dnn_bn_apply", &mut p, total)
    }

    #[allow(clippy::too_many_arguments)]
    unsafe fn bn_stats(h: &Handle, xv: &View, mut x: *const c_void, mut rmean: *mut c_void, mut rvar: *mut c_void, mut smean: *mut c_void, mut sinv: *mut c_void, spatial: i64, update: bool, mut factor: f32, mut eps: f32) -> cudnnStatus_t {
        let (mut params, mut m) = bn_sizes(xv, spatial);
        if !update {
            (rmean, rvar) = (smean, sinv);
        }
        let mut a = Args::shape(xv).strides(xv);
        a.push(spatial);
        a.push(update as i64);
        let mut p = vec![arg(&mut x), arg(&mut rmean), arg(&mut rvar), arg(&mut smean), arg(&mut sinv), arg(&mut params), arg(&mut m)];
        p.extend(a.0.iter_mut().map(|v| arg(v)));
        p.extend([arg(&mut factor), arg(&mut eps)]);
        dnn_launch(h, c"agx_dnn_bn_stats", &mut p, params * 32)
    }

    pub(crate) unsafe fn cudnnBatchNormalizationForwardInference(
        h: cudnnHandle_t,
        mode: cudnnBatchNormMode_t,
        alpha: *const c_void,
        beta: *const c_void,
        x_desc: cudnnTensorDescriptor_t,
        x: *const c_void,
        y_desc: cudnnTensorDescriptor_t,
        y: *mut c_void,
        _p_desc: cudnnTensorDescriptor_t,
        scale: *const c_void,
        bias: *const c_void,
        mean: *const c_void,
        var: *const c_void,
        eps: f64,
    ) -> cudnnStatus_t {
        if let Some((st, t)) = stage(h, &[(x_desc, x, true, false), (y_desc, y, reads_output(beta)?, true)])? {
            return st.finish(h, cudnnBatchNormalizationForwardInference(h, mode, alpha, beta, t[0].0, t[0].1, t[1].0, t[1].1, _p_desc, scale, bias, mean, var, eps));
        }
        let h = obj::<Handle>(h)?;
        let (xv, yv) = (tensor_view(x_desc)?, tensor_view(y_desc)?);
        if xv.dims != yv.dims || [scale, bias, mean, var].iter().any(|p| p.is_null()) {
            return Err(cudnnError_t::BAD_PARAM);
        }
        bn_apply(h, &xv, x, &yv, y, scale, bias, mean, var, bn_spatial(mode)?, 0, eps as f32, f32_scalar(alpha)?, f32_scalar(beta)?)
    }

    pub(crate) unsafe fn cudnnBatchNormalizationForwardTraining(
        h: cudnnHandle_t,
        mode: cudnnBatchNormMode_t,
        alpha: *const c_void,
        beta: *const c_void,
        x_desc: cudnnTensorDescriptor_t,
        x: *const c_void,
        y_desc: cudnnTensorDescriptor_t,
        y: *mut c_void,
        _p_desc: cudnnTensorDescriptor_t,
        scale: *const c_void,
        bias: *const c_void,
        factor: f64,
        rmean: *mut c_void,
        rvar: *mut c_void,
        eps: f64,
        smean: *mut c_void,
        sinv: *mut c_void,
    ) -> cudnnStatus_t {
        if let Some((st, t)) = stage(h, &[(x_desc, x, true, false), (y_desc, y, reads_output(beta)?, true)])? {
            return st.finish(
                h,
                cudnnBatchNormalizationForwardTraining(h, mode, alpha, beta, t[0].0, t[0].1, t[1].0, t[1].1, _p_desc, scale, bias, factor, rmean, rvar, eps, smean, sinv),
            );
        }
        let h = obj::<Handle>(h)?;
        let (xv, yv) = (tensor_view(x_desc)?, tensor_view(y_desc)?);
        if xv.dims != yv.dims || scale.is_null() || bias.is_null() || rmean.is_null() != rvar.is_null() || smean.is_null() != sinv.is_null() {
            return Err(cudnnError_t::BAD_PARAM);
        }
        let spatial = bn_spatial(mode)?;
        let (params, _) = bn_sizes(&xv, spatial);
        let (alpha, beta) = (f32_scalar(alpha)?, f32_scalar(beta)?);
        let scratch = if smean.is_null() { Some(temp(h, 8 * params)?) } else { None };
        let (smean, sinv) = match &scratch {
            Some(t) => (t.0, (t.0 as *mut f32).add(params as usize) as *mut c_void),
            None => (smean, sinv),
        };
        bn_stats(h, &xv, x, rmean, rvar, smean, sinv, spatial, !rmean.is_null(), factor as f32, eps as f32)?;
        bn_apply(h, &xv, x, &yv, y, scale, bias, smean, sinv, spatial, 1, 0.0, alpha, beta)
    }

    pub(crate) unsafe fn cudnnBatchNormalizationBackward(
        h: cudnnHandle_t,
        mode: cudnnBatchNormMode_t,
        alpha_data: *const c_void,
        beta_data: *const c_void,
        alpha_param: *const c_void,
        beta_param: *const c_void,
        x_desc: cudnnTensorDescriptor_t,
        mut x: *const c_void,
        dy_desc: cudnnTensorDescriptor_t,
        mut dy: *const c_void,
        dx_desc: cudnnTensorDescriptor_t,
        mut dx: *mut c_void,
        _p_desc: cudnnTensorDescriptor_t,
        mut scale: *const c_void,
        mut dscale: *mut c_void,
        mut dbias: *mut c_void,
        eps: f64,
        smean: *const c_void,
        sinv: *const c_void,
    ) -> cudnnStatus_t {
        if let Some((st, t)) = stage(h, &[(x_desc, x, true, false), (dy_desc, dy, true, false), (dx_desc, dx, reads_output(beta_data)?, true)])? {
            return st.finish(
                h,
                cudnnBatchNormalizationBackward(
                    h, mode, alpha_data, beta_data, alpha_param, beta_param, t[0].0, t[0].1, t[1].0, t[1].1, t[2].0, t[2].1, _p_desc, scale, dscale, dbias, eps, smean, sinv,
                ),
            );
        }
        let h = obj::<Handle>(h)?;
        let (xv, dyv, dxv) = (tensor_view(x_desc)?, tensor_view(dy_desc)?, tensor_view(dx_desc)?);
        if xv.dims != dyv.dims || xv.dims != dxv.dims || scale.is_null() || dscale.is_null() || dbias.is_null() || smean.is_null() != sinv.is_null() {
            return Err(cudnnError_t::BAD_PARAM);
        }
        let spatial = bn_spatial(mode)?;
        let (mut params, mut m) = bn_sizes(&xv, spatial);
        let (mut ad, mut bd) = (f32_scalar(alpha_data)?, f32_scalar(beta_data)?);
        let (mut ap, mut bp) = (f32_scalar(alpha_param)?, f32_scalar(beta_param)?);
        // the parameter sums, and the batch statistics when they were not saved
        let scratch = temp(h, 16 * params)?;
        let mut sums = scratch.0;
        let (mut smean, mut sinv) = (smean as *mut c_void, sinv as *mut c_void);
        if smean.is_null() {
            smean = (scratch.0 as *mut f32).add(2 * params as usize) as *mut c_void;
            sinv = (scratch.0 as *mut f32).add(3 * params as usize) as *mut c_void;
            bn_stats(h, &xv, x, std::ptr::null_mut(), std::ptr::null_mut(), smean, sinv, spatial, false, 0.0, eps as f32)?;
        }
        let mut a = Args::shape(&xv).strides(&xv).strides(&dyv);
        a.push(spatial);
        let mut p = vec![arg(&mut x), arg(&mut dy), arg(&mut smean), arg(&mut sinv), arg(&mut dscale), arg(&mut dbias), arg(&mut sums), arg(&mut params), arg(&mut m)];
        p.extend(a.0.iter_mut().map(|v| arg(v)));
        p.extend([arg(&mut ap), arg(&mut bp)]);
        dnn_launch(h, c"agx_dnn_bn_bwd_params", &mut p, params * 32)?;
        let mut total = xv.total();
        let mut a = Args::shape(&xv).strides(&xv).strides(&dyv).strides(&dxv);
        a.push(spatial);
        let mut p = vec![arg(&mut x), arg(&mut dy), arg(&mut dx), arg(&mut scale), arg(&mut smean), arg(&mut sinv), arg(&mut sums), arg(&mut total), arg(&mut m)];
        p.extend(a.0.iter_mut().map(|v| arg(v)));
        p.extend([arg(&mut ad), arg(&mut bd)]);
        dnn_launch(h, c"agx_dnn_bn_bwd_data", &mut p, total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index32_choice() {
        // N C H W K R S P Q ph pw sh sw dh dw G flip, x strides, y strides
        let mut s = [8, 64, 56, 56, 64, 3, 3, 56, 56, 1, 1, 1, 1, 1, 1, 1, 0, 200704, 3136, 56, 1, 200704, 3136, 56, 1];
        assert!(imp::index32(&s, 0));
        // x spans 2^31 elements
        s[0] = 12000;
        assert!(!imp::index32(&s, 0));
        s[0] = 8;
        assert!(!imp::index32(&s, 1 << 31));
        s[17] = -1;
        assert!(!imp::index32(&s, 0));
    }

    #[test]
    fn output_dims_and_strides() {
        assert_eq!(output_dim(7, 1, 2, 1, 3), 4);
        assert_eq!(output_dim(6, 0, 1, 2, 2), 4);
        assert_eq!(output_dim(5, 0, 2, 1, 1), 3);
        assert_eq!(packed_strides(&[2, 3, 4, 5], false), [60, 20, 5, 1]);
        assert_eq!(packed_strides(&[2, 3, 4, 5], true), [60, 1, 15, 3]);
    }

    #[test]
    fn kernel_shapes() {
        let f32_ = cudnnDataType_t::CUDNN_DATA_FLOAT.0;
        let x = Tensor { ty: f32_, dims: vec![2, 4, 7, 6], strides: packed_strides(&[2, 4, 7, 6], false) };
        let w = Filter { ty: f32_, format: 0, dims: vec![6, 2, 3, 2] };
        let c = Conv { pad: [1, 0], stride: [2, 1], dilation: [1, 2], mode: 1, groups: 2, ..Conv::default() };
        let y = Tensor { ty: f32_, dims: vec![2, 6, 4, 4], strides: packed_strides(&[2, 6, 4, 4], false) };
        let s = kernel_shape(&x, &w, &c, &y).unwrap();
        assert_eq!(&s[..17], &[2, 4, 7, 6, 6, 3, 2, 4, 4, 1, 0, 2, 1, 1, 2, 2, 0]);
        assert_eq!(&s[17..], &[168, 42, 6, 1, 96, 16, 4, 1]);
        // the convolution mode flips the filter
        assert_eq!(kernel_shape(&x, &w, &Conv { mode: 0, ..c.clone() }, &y).unwrap()[16], 1);
        // wrong output shape, channel count, layout and type
        let y_bad = Tensor { dims: vec![2, 6, 4, 5], ..y.clone() };
        assert_eq!(kernel_shape(&x, &w, &c, &y_bad), Err(cudnnError_t::BAD_PARAM));
        assert_eq!(kernel_shape(&x, &w, &Conv { groups: 1, ..c.clone() }, &y), Err(cudnnError_t::BAD_PARAM));
        assert_eq!(kernel_shape(&x, &Filter { format: 1, ..w.clone() }, &c, &y), Err(cudnnError_t::NOT_SUPPORTED));
        let x64 = Tensor { ty: cudnnDataType_t::CUDNN_DATA_DOUBLE.0, ..x.clone() };
        assert_eq!(kernel_shape(&x64, &w, &c, &y), Err(cudnnError_t::NOT_SUPPORTED));
    }

    #[test]
    fn views_and_broadcast() {
        let t = Tensor { ty: cudnnDataType_t::CUDNN_DATA_FLOAT.0, dims: vec![2, 3, 4], strides: vec![12, 4, 1] };
        let v = view(&t).unwrap();
        assert_eq!(v, View { dims: [2, 3, 4, 1], strides: [12, 4, 1, 0] });
        let bias = View { dims: [1, 3, 1, 1], strides: [3, 1, 1, 1] };
        let c = View { dims: [2, 3, 4, 1], strides: [12, 4, 1, 1] };
        assert_eq!(broadcast(&bias, &c), Ok([0, 1, 0, 1]));
        let bad = View { dims: [1, 2, 1, 1], ..bias };
        assert_eq!(broadcast(&bad, &c), Err(cudnnError_t::BAD_PARAM));
        let half = Tensor { ty: cudnnDataType_t::CUDNN_DATA_HALF.0, ..t };
        assert_eq!(view(&half), Err(cudnnError_t::NOT_SUPPORTED));
    }

    #[test]
    fn pooling_output() {
        let p = Pooling { mode: 0, nan: 0, window: [3, 2], pad: [1, 0], stride: [2, 1] };
        assert_eq!(pool_output(&p, 7, 6), (4, 5));
        let a = Activation { mode: 6, swish_beta: 2.0, ..Activation::default() };
        assert_eq!(activation_args(&a), Ok((6, 2.0)));
    }
}
