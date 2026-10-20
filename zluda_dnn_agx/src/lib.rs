//! cuDNN 9 on Apple GPUs for zluda_agx (Asahi Linux): the legacy convolution API, on the MLIR convolution kernels of
//! agx_libs (`agx_libs/kernels/conv.mlir`, written once for every AGX generation).
//!
//! Supported so far: handles and streams, 4-D tensor descriptors with any strides, packed NCHW filters, 2-D
//! convolution descriptors (padding, strides, dilations, groups, cross-correlation or convolution), and
//! `cudnnConvolutionForward`, `cudnnConvolutionBackwardData` and `cudnnConvolutionBackwardFilter` for f32 and f16
//! tensors (accumulated in f32), with the algorithm queries (one algorithm per direction, no workspace). Other
//! functions return `CUDNN_STATUS_NOT_SUPPORTED`.

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
            cudnnConvolutionBackwardData,
            cudnnConvolutionBackwardFilter,
            cudnnConvolutionForward,
            cudnnCreate,
            cudnnCreateConvolutionDescriptor,
            cudnnCreateFilterDescriptor,
            cudnnCreateTensorDescriptor,
            cudnnDestroy,
            cudnnDestroyConvolutionDescriptor,
            cudnnDestroyFilterDescriptor,
            cudnnDestroyTensorDescriptor,
            cudnnFindConvolutionBackwardDataAlgorithm,
            cudnnFindConvolutionBackwardDataAlgorithmEx,
            cudnnFindConvolutionBackwardFilterAlgorithm,
            cudnnFindConvolutionBackwardFilterAlgorithmEx,
            cudnnFindConvolutionForwardAlgorithm,
            cudnnFindConvolutionForwardAlgorithmEx,
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
            cudnnGetStream,
            cudnnGetTensor4dDescriptor,
            cudnnGetTensorNdDescriptor,
            cudnnGetVersion,
            cudnnSetConvolution2dDescriptor,
            cudnnSetConvolutionGroupCount,
            cudnnSetConvolutionMathType,
            cudnnSetConvolutionNdDescriptor,
            cudnnSetConvolutionReorderType,
            cudnnSetFilter4dDescriptor,
            cudnnSetFilterNdDescriptor,
            cudnnSetStream,
            cudnnSetTensor4dDescriptor,
            cudnnSetTensor4dDescriptorEx,
            cudnnSetTensorNdDescriptor,
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
        let f = engine::function(agx_libs::CONV, name).map_err(status)?;
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
        let elements = s[0] as u64 * s[4] as u64 * s[7] as u64 * s[8] as u64;
        run(h, "fwd", s, half, elements, x, w, y, alpha, beta)
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
        let elements = s[0] as u64 * s[1] as u64 * s[2] as u64 * s[3] as u64;
        run(h, "bwd_data", s, half, elements, dx, w, dy, alpha, beta)
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
        let elements = s[4] as u64 * (s[1] / s[15]) as u64 * s[5] as u64 * s[6] as u64;
        run(h, "bwd_filter", s, half, elements, x, dw, dy, alpha, beta)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
