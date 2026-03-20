use crate::MIOpenVtable;
use arrayvec::ArrayVec;
use cuda_types::cudnn9::*;
use hip_runtime_sys::*;
use miopen_sys::*;
use rustc_hash::FxHashMap;
use std::{
    collections::{hash_map, VecDeque},
    mem, ptr,
    sync::{
        atomic::{AtomicPtr, Ordering},
        Mutex, OnceLock,
    },
};
use zluda_common::{from_cuda_object, ZludaObject};

fn miopen() -> Result<&'static super::MIOpenVtable, miopenError_t> {
    static LOCK: OnceLock<Result<super::MIOpenVtable, miopenError_t>> = OnceLock::new();
    let unwrapped: &Result<super::MIOpenVtable, miopenError_t> =
        LOCK.get_or_init(|| unsafe { super::MIOpenVtable::new() });
    unwrapped.as_ref().map_err(|x| *x)
}

pub(crate) struct Context {
    base: miopenHandle_t,
    stream: AtomicPtr<ihipStream_t>,
    search_workspace: Mutex<ContextCache>,
}

from_cuda_object!(Context);

impl Context {
    fn new(base: miopenHandle_t) -> Self {
        Self {
            base,
            stream: AtomicPtr::new(ptr::null_mut()),
            search_workspace: Mutex::new(ContextCache {
                beta_buffer_cache: TemporaryBufferAllocator::new(),
                search_cache: SearchCache::new(),
            }),
        }
    }
}

impl ZludaObject for Context {
    const COOKIE: usize = 0x575f48767c76029a;

    type Error = cudnnError_t;
    type CudaHandle = cudnnHandle_t;

    fn drop_checked(&mut self) -> Result<(), cudnnError_t> {
        let result1 = unsafe { miopen()?.miopenDestroy(self.base) }.map_err(Into::into);
        let result2 = if let Ok(search_workspace) = self.search_workspace.get_mut() {
            search_workspace
                .drop_checked()
                .map_err(|_| cudnnError_t::INTERNAL_ERROR)
        } else {
            Ok(())
        };
        result1.and(result2)
    }
}

struct ContextCache {
    beta_buffer_cache: TemporaryBufferAllocator,
    search_cache: SearchCache,
}

struct SearchCache(FxHashMap<ConvolutionOpCacheKey, ArrayVec<SolutionWithProperties, 6>>);

#[derive(Clone, Copy)]
struct SolutionWithProperties {
    solution: miopenSolution_t,
    time: f32,
    memory: usize,
    algo: miopenConvAlgorithm_t,
}

impl SolutionWithProperties {
    fn new(miopen: &MIOpenVtable, solution: miopenSolution_t) -> Result<Self, miopenError_t> {
        let mut time = 0.0;
        let mut memory = 0;
        unsafe { miopen.miopenGetSolutionTime(solution, &mut time) }?;
        unsafe { miopen.miopenGetSolutionWorkspaceSize(solution, &mut memory) }?;
        let mut solver_id = 0;
        unsafe { miopen.miopenGetSolutionSolverId(solution, &mut solver_id) }?;
        let mut algo = unsafe { mem::zeroed() };
        unsafe { miopen.miopenGetSolverIdConvAlgorithm(solver_id, &mut algo) }?;
        Ok(SolutionWithProperties {
            solution,
            time,
            memory,
            algo,
        })
    }

    fn worst_solution() -> Self {
        Self {
            solution: unsafe { mem::zeroed() },
            time: f32::MAX,
            memory: usize::MAX,
            algo: miopenConvAlgorithm_t(0),
        }
    }
}

impl SearchCache {
    fn new() -> Self {
        Self(FxHashMap::default())
    }

    fn get_or_insert<'a>(
        &'a mut self,
        miopen: &MIOpenVtable,
        handle: miopenHandle_t,
        direction: miopenProblemDirection_t,
        desc: miopenConvolutionDescriptor_t,
        x: miopenTensorDescriptor_t,
        w: miopenTensorDescriptor_t,
        y: miopenTensorDescriptor_t,
    ) -> Result<&'a ArrayVec<SolutionWithProperties, 6>, miopenError_t> {
        let key = ConvolutionOpCacheKey::new(miopen, direction, desc, x, w, y)?;
        Ok(match self.0.entry(key) {
            hash_map::Entry::Occupied(entry) => &*entry.into_mut(),
            hash_map::Entry::Vacant(entry) => {
                let mut problem = unsafe { mem::zeroed() };
                unsafe { miopen.miopenCreateConvProblem(&mut problem, desc, direction) }?;
                let _drop_fn1 = DropFn(move || {
                    unsafe { miopen.miopenDestroyProblem(problem) }.ok();
                });
                unsafe {
                    miopen.miopenSetProblemTensorDescriptor(
                        problem,
                        miopenTensorArgumentId_t::miopenTensorConvolutionX,
                        x,
                    )
                }?;
                unsafe {
                    miopen.miopenSetProblemTensorDescriptor(
                        problem,
                        miopenTensorArgumentId_t::miopenTensorConvolutionW,
                        w,
                    )
                }?;
                unsafe {
                    miopen.miopenSetProblemTensorDescriptor(
                        problem,
                        miopenTensorArgumentId_t::miopenTensorConvolutionY,
                        y,
                    )
                }?;
                let mut solutions = [unsafe { mem::zeroed::<miopenSolution_t>() }; 16];
                let mut solutions_count = 0;
                unsafe {
                    miopen.miopenFindSolutions(
                        handle,
                        problem,
                        miopenFindOptions_t(ptr::null_mut()),
                        solutions.as_mut_ptr(),
                        &mut solutions_count,
                        solutions.len(),
                    )
                }?;
                let solutions = solutions[..solutions_count]
                    .into_iter()
                    .copied()
                    .map(|solution| SolutionWithProperties::new(miopen, solution))
                    .collect::<Result<ArrayVec<_, 16>, _>>()?;
                &*entry.insert(Self::remove_duplicates(solutions))
            }
        })
    }

    // It is possible to receive multiple solvers matching the same algorithm
    fn remove_duplicates(
        solutions: ArrayVec<SolutionWithProperties, 16>,
    ) -> ArrayVec<SolutionWithProperties, 6> {
        let mut fastest_by_algo = [SolutionWithProperties::worst_solution(); 6];
        for solution in solutions {
            let algo_index = solution.algo.0 as usize;
            if fastest_by_algo[algo_index].time < solution.time {
                continue;
            }
            fastest_by_algo[algo_index] = solution;
        }
        fastest_by_algo.sort_unstable_by(|x, y| {
            x.time
                .total_cmp(&y.time)
                .then_with(|| x.memory.cmp(&y.memory))
        });
        fastest_by_algo
            .into_iter()
            .filter(|solution| !solution.solution.0.is_null())
            .collect()
    }
}

struct DropFn<F: FnMut()>(F);

impl<F: FnMut()> Drop for DropFn<F> {
    fn drop(&mut self) {
        (self.0)();
    }
}

#[derive(PartialEq, Eq, Hash)]
struct ConvolutionOpCacheKey {
    direction: miopenProblemDirection_t,
    x: TensorCacheKey,
    w: TensorCacheKey,
    conv: ConvolutionDescriptorCacheKey,
    y: TensorCacheKey,
}

impl ConvolutionOpCacheKey {
    fn new(
        miopen: &MIOpenVtable,
        direction: miopenProblemDirection_t,
        conv_desc: miopenConvolutionDescriptor_t,
        x: miopenTensorDescriptor_t,
        w: miopenTensorDescriptor_t,
        y: miopenTensorDescriptor_t,
    ) -> Result<Self, miopenError_t> {
        Ok(Self {
            direction,
            conv: ConvolutionDescriptorCacheKey::new(miopen, conv_desc)?,
            x: TensorCacheKey::new(miopen, x)?,
            w: TensorCacheKey::new(miopen, w)?,
            y: TensorCacheKey::new(miopen, y)?,
        })
    }
}
impl ContextCache {
    fn drop_checked(&mut self) -> Result<(), hipErrorCode_t> {
        let mut err = None;
        while let Some(mut buffer) = self.beta_buffer_cache.buffers.pop_front() {
            if let Err(e) = buffer.drop_checked() {
                err = Some(e);
            }
        }
        err.map_or(Ok(()), Err)
    }
}

unsafe impl Send for ContextCache {}
unsafe impl Sync for ContextCache {}

// At various point of the execution we need temporary buffers to hold device
// data to support various MIOpen workarounds.
// Normally, their use does not overlap, but it's still possible.
struct TemporaryBufferAllocator {
    buffers: VecDeque<BetaBuffer>,
}

impl TemporaryBufferAllocator {
    fn new() -> Self {
        Self {
            buffers: VecDeque::new(),
        }
    }

    fn scavange_or_allocate_buffer(&mut self, size: usize) -> Result<BetaBuffer, hipErrorCode_t> {
        let mut result = None;
        let mut err = None;
        let mut update_buffer_if_biggest = |buffer: BetaBuffer| {
            if result
                .as_ref()
                .map(|b: &BetaBuffer| buffer.size > b.size)
                .unwrap_or(true)
            {
                result = Some(buffer);
            }
        };
        while let Some(buffer) = self.buffers.pop_front() {
            match buffer.free {
                None => {
                    update_buffer_if_biggest(buffer);
                }
                Some(event) => match unsafe { hipEventQuery(event) } {
                    hipError_t::ErrorNotReady => {
                        self.buffers.push_front(buffer);
                        break;
                    }
                    hipError_t::Success => {
                        update_buffer_if_biggest(buffer);
                    }
                    Err(err_code) => {
                        err = Some(err_code);
                        break;
                    }
                },
            }
        }
        match err {
            Some(err_code) => Err(err_code),
            None => match result {
                Some(buffer) if buffer.size >= size => Ok(buffer),
                _ => Ok(BetaBuffer::new(size)?),
            },
        }
    }

    fn with_async_buffer(
        &mut self,
        size: usize,
        stream: hipStream_t,
        fn_: impl FnOnce(*mut ::std::os::raw::c_void) -> Result<(), miopenError_t>,
    ) -> Result<(), miopenError_t> {
        let mut buffer = self
            .scavange_or_allocate_buffer(size)
            .map_err(|_| miopenError_t::InternalError)?;
        if buffer.free.is_none() {
            let mut event = std::ptr::null_mut();
            unsafe {
                hipEventCreateWithFlags(
                    &mut event,
                    hipEventDisableTiming | hipEventDisableSystemFence,
                )
                .map_err(|_| miopenError_t::InternalError)?
            };
            buffer.free = Some(event);
        }
        fn_(buffer.data)?;
        unsafe { hipEventRecord(buffer.free.unwrap(), stream) }
            .map_err(|_| miopenError_t::InternalError)?;
        self.buffers.push_back(buffer);
        Ok(())
    }
}

struct BetaBuffer {
    size: usize,
    data: *mut ::std::os::raw::c_void,
    free: Option<hipEvent_t>,
}

impl BetaBuffer {
    fn new(size: usize) -> Result<Self, hipErrorCode_t> {
        let mut data = std::ptr::null_mut();
        unsafe { hipMalloc(&mut data, size)? };
        Ok(Self {
            size,
            data,
            free: None,
        })
    }

    fn drop_checked(&mut self) -> Result<(), hipErrorCode_t> {
        let result1 = unsafe { hipFree(self.data) };
        let result2 = self
            .free
            .map(|e| unsafe { hipEventDestroy(e) })
            .unwrap_or(Ok(()));
        result1.and(result2)
    }
}

impl Drop for BetaBuffer {
    fn drop(&mut self) {
        self.drop_checked().ok();
    }
}

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> cudnnStatus_t {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> cudnnStatus_t {
    cudnnStatus_t::ERROR_NOT_SUPPORTED
}

pub(crate) fn get_max_device_version() -> usize {
    todo!()
}
pub(crate) fn get_cudart_version() -> usize {
    todo!()
}
pub(crate) fn get_last_error_string(_message: *mut ::core::ffi::c_char, _max_size: usize) -> () {
    todo!()
}

pub(crate) unsafe fn create(handle: &mut cudnnHandle_t) -> miopenStatus_t {
    let mut miopen_handle = mem::zeroed();
    miopen()?.miopenCreate(&mut miopen_handle)?;
    *handle = Context::new(miopen_handle).wrap();
    Ok(())
}

pub(crate) fn destroy(handle: cudnnHandle_t) -> cudnnStatus_t {
    zluda_common::drop_checked::<Context>(handle)?;
    Ok(())
}

pub(crate) fn create_tensor_descriptor(
    tensor_desc: &mut miopenTensorDescriptor_t,
) -> miopenStatus_t {
    unsafe { miopen()?.miopenCreateTensorDescriptor(tensor_desc) }
}

pub(crate) fn create_filter_descriptor(
    filter_desc: &mut miopenTensorDescriptor_t,
) -> miopenStatus_t {
    unsafe { miopen()?.miopenCreateTensorDescriptor(filter_desc) }
}

pub(crate) unsafe fn set_tensor4d_descriptor(
    tensor_desc: miopenTensorDescriptor_t,
    format: miopenTensorLayout_t,
    data_type: miopenDataType_t,
    n: ::std::os::raw::c_int,
    c: ::std::os::raw::c_int,
    h: ::std::os::raw::c_int,
    w: ::std::os::raw::c_int,
) -> miopenStatus_t {
    // Even if the layout is NHWC, miopenSetNdTensorDescriptorWithLayout still expects NCHW order
    let lens = [n, c, h, w];
    miopen()?.miopenSetNdTensorDescriptorWithLayout(
        tensor_desc,
        data_type,
        format,
        lens.as_ptr(),
        lens.len() as i32,
    )
}

pub(crate) unsafe fn set_filter4d_descriptor(
    filter_desc: miopenTensorDescriptor_t,
    data_type: miopenDataType_t,
    format: miopenTensorLayout_t,
    n: ::std::os::raw::c_int,
    c: ::std::os::raw::c_int,
    h: ::std::os::raw::c_int,
    w: ::std::os::raw::c_int,
) -> miopenStatus_t {
    set_tensor4d_descriptor(filter_desc, format, data_type, n, c, h, w)
}

pub(crate) fn create_convolution_descriptor(
    conv_desc: &mut miopenConvolutionDescriptor_t,
) -> miopenStatus_t {
    unsafe { miopen()?.miopenCreateConvolutionDescriptor(conv_desc) }
}

pub(crate) unsafe fn set_convolution2d_descriptor(
    conv_desc: miopenConvolutionDescriptor_t,
    pad_h: ::std::os::raw::c_int,
    pad_w: ::std::os::raw::c_int,
    u: ::std::os::raw::c_int,
    v: ::std::os::raw::c_int,
    dilation_h: ::std::os::raw::c_int,
    dilation_w: ::std::os::raw::c_int,
    mode: miopenConvolutionMode_t,
    _compute_type: miopenDataType_t,
) -> miopenStatus_t {
    miopen()?.miopenInitConvolutionDescriptor(
        conv_desc, mode, pad_h, pad_w, u, v, dilation_h, dilation_w,
    )
}

pub(crate) unsafe fn set_convolution_group_count(
    conv_desc: miopenConvolutionDescriptor_t,
    group_count: ::core::ffi::c_int,
) -> miopenStatus_t {
    miopen()?.miopenSetConvolutionGroupCount(conv_desc, group_count)
}

pub(crate) unsafe fn set_convolution_nd_descriptor(
    conv_desc: miopenConvolutionDescriptor_t,
    array_length: ::core::ffi::c_int,
    pad_a: *const ::core::ffi::c_int,
    filter_stride_a: *const ::core::ffi::c_int,
    dilation_a: *const ::core::ffi::c_int,
    mode: miopenConvolutionMode_t,
    _compute_type: miopenDataType_t,
) -> miopenStatus_t {
    if array_length != 2 {
        return miopenStatus_t::ErrorNotImplemented;
    }
    let pad = std::slice::from_raw_parts(pad_a, array_length as usize);
    let filter_stride = std::slice::from_raw_parts(filter_stride_a, array_length as usize);
    let dilation = std::slice::from_raw_parts(dilation_a, array_length as usize);
    miopen()?.miopenInitConvolutionDescriptor(
        conv_desc,
        mode,
        pad[0],
        pad[1],
        filter_stride[0],
        filter_stride[1],
        dilation[0],
        dilation[1],
    )
}

pub(crate) unsafe fn set_filter_nd_descriptor(
    filter_desc: miopenTensorDescriptor_t,
    data_type: miopenDataType_t,
    format: miopenTensorLayout_t,
    nb_dims: ::core::ffi::c_int,
    filter_dim_a: *const ::core::ffi::c_int,
) -> miopenStatus_t {
    if nb_dims > 5 {
        return miopenStatus_t::ErrorNotImplemented;
    }
    miopen()?.miopenSetNdTensorDescriptorWithLayout(
        filter_desc,
        data_type,
        format,
        filter_dim_a,
        nb_dims,
    )
}

pub(crate) unsafe fn get_filter_nd_descriptor(
    filter_desc: miopenTensorDescriptor_t,
    nb_dims_requested: ::core::ffi::c_int,
    data_type: &mut cudnnDataType_t,
    format: &mut cudnnTensorFormat_t,
    nb_dims: &mut ::core::ffi::c_int,
    filter_dim_a: *mut ::core::ffi::c_int,
) -> miopenStatus_t {
    get_tensor_nd_descriptor_impl(
        filter_desc,
        nb_dims_requested,
        data_type,
        format,
        nb_dims,
        filter_dim_a,
        ptr::null_mut(),
    )
}

pub(crate) unsafe fn get_tensor_nd_descriptor_impl(
    filter_desc: miopenTensorDescriptor_t,
    nb_dims_requested: ::core::ffi::c_int,
    data_type: &mut cudnnDataType_t,
    format: &mut cudnnTensorFormat_t,
    nb_dims: &mut ::core::ffi::c_int,
    filter_dim_a: *mut i32,
    strides_dim_a: *mut i32,
) -> miopenStatus_t {
    fn to_cuda_data_type(data_type: miopenDataType_t) -> Result<cudnnDataType_t, miopenError_t> {
        Ok(match data_type {
            miopenDataType_t::miopenHalf => cudnnDataType_t::CUDNN_DATA_HALF,
            miopenDataType_t::miopenFloat => cudnnDataType_t::CUDNN_DATA_FLOAT,
            miopenDataType_t::miopenInt32 => cudnnDataType_t::CUDNN_DATA_INT32,
            miopenDataType_t::miopenInt8 => cudnnDataType_t::CUDNN_DATA_INT8,
            miopenDataType_t::miopenBFloat16 => cudnnDataType_t::CUDNN_DATA_BFLOAT16,
            miopenDataType_t::miopenDouble => cudnnDataType_t::CUDNN_DATA_DOUBLE,
            miopenDataType_t::miopenFloat8 => cudnnDataType_t::CUDNN_DATA_FP8_E4M3,
            miopenDataType_t::miopenBFloat8 => cudnnDataType_t::CUDNN_DATA_FP8_E5M2,
            miopenDataType_t::miopenInt64 => cudnnDataType_t::CUDNN_DATA_INT64,
            _ => return Err(miopenError_t::NotImplemented),
        })
    }
    let miopen = miopen()?;
    miopen.miopenGetTensorDescriptorSize(filter_desc, nb_dims)?;
    *format = cudnnTensorFormat_t::CUDNN_TENSOR_NCHW;
    let mut miopen_data_type = mem::zeroed();
    if *nb_dims > nb_dims_requested {
        let miopen_size = *nb_dims as usize;
        let mut dimensions = if filter_dim_a.is_null() {
            None
        } else {
            Some(vec![0; miopen_size])
        };
        let mut strides = if strides_dim_a.is_null() {
            None
        } else {
            Some(vec![0; miopen_size])
        };
        miopen.miopenGetTensorDescriptor(
            filter_desc,
            &mut miopen_data_type,
            dimensions.as_mut().map_or(ptr::null_mut(), Vec::as_mut_ptr),
            strides.as_mut().map_or(ptr::null_mut(), Vec::as_mut_ptr),
        )?;

        if let Some(dimensions) = dimensions {
            ptr::copy_nonoverlapping(
                dimensions.as_ptr(),
                filter_dim_a,
                nb_dims_requested as usize,
            );
        }
        if let Some(strides) = strides {
            ptr::copy_nonoverlapping(strides.as_ptr(), strides_dim_a, nb_dims_requested as usize);
        }
    } else {
        miopen.miopenGetTensorDescriptor(
            filter_desc,
            &mut miopen_data_type,
            filter_dim_a,
            strides_dim_a,
        )?;
    }
    *data_type = to_cuda_data_type(miopen_data_type)?;
    Ok(())
}

pub(crate) unsafe fn set_tensor_nd_descriptor(
    tensor_desc: miopenTensorDescriptor_t,
    data_type: miopenDataType_t,
    nb_dims: ::core::ffi::c_int,
    dim_a: *const ::core::ffi::c_int,
    stride_a: *const ::core::ffi::c_int,
) -> miopenStatus_t {
    miopen()?.miopenSetTensorDescriptor(tensor_desc, data_type, nb_dims, dim_a, stride_a)
}

pub(crate) unsafe fn get_tensor_nd_descriptor(
    tensor_desc: miopenTensorDescriptor_t,
    nb_dims_requested: ::core::ffi::c_int,
    data_type: &mut cudnnDataType_t,
    nb_dims: &mut ::core::ffi::c_int,
    dim_a: *mut ::core::ffi::c_int,
    stride_a: *mut ::core::ffi::c_int,
) -> miopenStatus_t {
    let mut format = mem::zeroed();
    get_tensor_nd_descriptor_impl(
        tensor_desc,
        nb_dims_requested,
        data_type,
        &mut format,
        nb_dims,
        dim_a,
        stride_a,
    )
}

pub(crate) unsafe fn set_convolution_math_type(
    _conv_desc: miopenConvolutionDescriptor_t,
    _math_type: cudnnMathType_t,
) -> miopenStatus_t {
    // miopen does not have an equivalent function to set math type
    miopenStatus_t::Success
}

pub(crate) unsafe fn get_convolution_forward_algorithm_v7(
    handle: &Context,
    x_desc: miopenTensorDescriptor_t,
    w_desc: miopenTensorDescriptor_t,
    conv_desc: miopenConvolutionDescriptor_t,
    y_desc: miopenTensorDescriptor_t,
    requested_algo_count: ::std::os::raw::c_int,
    returned_algo_count: &mut ::std::os::raw::c_int,
    perf_results: &mut cudnnConvolutionFwdAlgoPerf_t,
) -> cudnnStatus_t {
    get_algorithm(
        handle,
        miopenProblemDirection_t::miopenProblemDirectionForward,
        x_desc,
        w_desc,
        y_desc,
        conv_desc,
        requested_algo_count,
        returned_algo_count,
        perf_results,
        algo_perf_to_cudnn,
    )
}

#[derive(PartialEq, Eq, Hash)]
struct TensorCacheKey {
    data_type: miopenDataType_t,
    dimensions: [i32; 5],
    strides: [i32; 5],
}

impl TensorCacheKey {
    fn new(miopen: &MIOpenVtable, desc: miopenTensorDescriptor_t) -> Result<Self, miopenError_t> {
        let mut data_type = unsafe { mem::zeroed() };
        let mut dimensions = [0; 5];
        let mut strides = [0; 5];
        unsafe {
            miopen.miopenGetTensorDescriptor(
                desc,
                &mut data_type,
                dimensions.as_mut_ptr(),
                strides.as_mut_ptr(),
            )?;
        }
        Ok(Self {
            data_type,
            dimensions,
            strides,
        })
    }
}

#[derive(PartialEq, Eq, Hash)]
struct ConvolutionDescriptorCacheKey {
    mode: miopenConvolutionMode_t,
    pad_h: i32,
    pad_w: i32,
    stride_h: i32,
    stride_w: i32,
    dilation_h: i32,
    dilation_w: i32,
}

impl ConvolutionDescriptorCacheKey {
    fn new(
        miopen: &MIOpenVtable,
        desc: miopenConvolutionDescriptor_t,
    ) -> Result<Self, miopenError_t> {
        let mut mode = unsafe { mem::zeroed() };
        let mut pad_h = 0;
        let mut pad_w = 0;
        let mut stride_h = 0;
        let mut stride_w = 0;
        let mut dilation_h = 0;
        let mut dilation_w = 0;
        unsafe {
            miopen.miopenGetConvolutionDescriptor(
                desc,
                &mut mode,
                &mut pad_h,
                &mut pad_w,
                &mut stride_h,
                &mut stride_w,
                &mut dilation_h,
                &mut dilation_w,
            )?;
        }
        Ok(Self {
            mode,
            pad_h,
            pad_w,
            stride_h,
            stride_w,
            dilation_h,
            dilation_w,
        })
    }
}

fn algo_perf_to_cudnn(solution: SolutionWithProperties) -> cudnnConvolutionFwdAlgoPerf_t {
    cudnnConvolutionFwdAlgoPerf_t {
        algo: algo_to_cudnn(solution.algo),
        time: solution.time,
        memory: solution.memory,
        status: cudnnStatus_t::SUCCESS,
        determinism: cudnnDeterminism_t::CUDNN_NON_DETERMINISTIC,
        mathType: cudnnMathType_t::CUDNN_DEFAULT_MATH,
        reserved: [0, 0, 0],
    }
}

fn algo_perf_to_cudnn_bwd_data(
    solution: SolutionWithProperties,
) -> cudnnConvolutionBwdDataAlgoPerf_t {
    cudnnConvolutionBwdDataAlgoPerf_t {
        algo: algo_to_cudnn_bwd_data(solution.algo),
        time: solution.time,
        memory: solution.memory,
        status: cudnnStatus_t::SUCCESS,
        determinism: cudnnDeterminism_t::CUDNN_NON_DETERMINISTIC,
        mathType: cudnnMathType_t::CUDNN_DEFAULT_MATH,
        reserved: [0, 0, 0],
    }
}

fn algo_perf_to_cudnn_bwd_filter(
    solution: SolutionWithProperties,
) -> cudnnConvolutionBwdFilterAlgoPerf_t {
    cudnnConvolutionBwdFilterAlgoPerf_t {
        algo: algo_to_cudnn_bwd_filter(solution.algo),
        time: solution.time,
        memory: solution.memory,
        status: cudnnStatus_t::SUCCESS,
        determinism: cudnnDeterminism_t::CUDNN_NON_DETERMINISTIC,
        mathType: cudnnMathType_t::CUDNN_DEFAULT_MATH,
        reserved: [0, 0, 0],
    }
}

fn algo_to_cudnn(result: miopenConvAlgorithm_t) -> cudnnConvolutionFwdAlgo_t {
    match result {
        miopenConvAlgorithm_t::miopenConvolutionAlgoGEMM => {
            cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_GEMM
        }
        miopenConvAlgorithm_t::miopenConvolutionAlgoDirect => {
            cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_DIRECT
        }
        miopenConvAlgorithm_t::miopenConvolutionAlgoFFT => {
            cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_FFT
        }
        miopenConvAlgorithm_t::miopenConvolutionAlgoWinograd => {
            cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_WINOGRAD
        }
        miopenConvAlgorithm_t::miopenConvolutionAlgoImplicitGEMM => {
            cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_IMPLICIT_GEMM
        }
        _ => cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_GEMM,
    }
}

fn algo_to_cudnn_bwd_filter(algo: miopenConvAlgorithm_t) -> cudnnConvolutionBwdFilterAlgo_t {
    match algo {
        miopenConvAlgorithm_t::miopenConvolutionAlgoGEMM => {
            cudnnConvolutionBwdFilterAlgo_t::CUDNN_CONVOLUTION_BWD_FILTER_ALGO_0
        }
        miopenConvAlgorithm_t::miopenConvolutionAlgoDirect => {
            cudnnConvolutionBwdFilterAlgo_t::CUDNN_CONVOLUTION_BWD_FILTER_ALGO_1
        }
        miopenConvAlgorithm_t::miopenConvolutionAlgoImplicitGEMM => {
            cudnnConvolutionBwdFilterAlgo_t::CUDNN_CONVOLUTION_BWD_FILTER_ALGO_3
        }
        miopenConvAlgorithm_t::miopenConvolutionAlgoFFT => {
            cudnnConvolutionBwdFilterAlgo_t::CUDNN_CONVOLUTION_BWD_FILTER_ALGO_FFT
        }
        miopenConvAlgorithm_t::miopenConvolutionAlgoWinograd => {
            cudnnConvolutionBwdFilterAlgo_t::CUDNN_CONVOLUTION_BWD_FILTER_ALGO_WINOGRAD
        }
        _ => cudnnConvolutionBwdFilterAlgo_t::CUDNN_CONVOLUTION_BWD_FILTER_ALGO_0,
    }
}

fn algo_to_cudnn_bwd_data(algo: miopenConvAlgorithm_t) -> cudnnConvolutionBwdDataAlgo_t {
    match algo {
        miopenConvAlgorithm_t::miopenConvolutionAlgoGEMM => {
            cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_0
        }
        miopenConvAlgorithm_t::miopenConvolutionAlgoDirect => {
            cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_1
        }
        miopenConvAlgorithm_t::miopenConvolutionAlgoFFT => {
            cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_FFT
        }
        miopenConvAlgorithm_t::miopenConvolutionAlgoWinograd => {
            cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_WINOGRAD
        }
        miopenConvAlgorithm_t::miopenConvolutionAlgoImplicitGEMM => cudnnConvolutionBwdDataAlgo_t(
            cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_COUNT.0 + 1,
        ),
        _ => cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_0,
    }
}

pub(crate) unsafe fn get_convolution_forward_workspace_size(
    handle: &Context,
    x_desc: miopenTensorDescriptor_t,
    w_desc: miopenTensorDescriptor_t,
    conv_desc: miopenConvolutionDescriptor_t,
    y_desc: miopenTensorDescriptor_t,
    algo: miopenConvFwdAlgorithm_t,
    size_in_bytes: &mut usize,
) -> miopenStatus_t {
    get_workspace_size(
        handle,
        miopenProblemDirection_t::miopenProblemDirectionForward,
        x_desc,
        w_desc,
        y_desc,
        conv_desc,
        algo.0,
        size_in_bytes,
    )
}

pub(crate) unsafe fn convolution_forward(
    handle: &Context,
    alpha: *const ::std::os::raw::c_void,
    x_desc: miopenTensorDescriptor_t,
    x: *const ::std::os::raw::c_void,
    w_desc: miopenTensorDescriptor_t,
    w: *const ::std::os::raw::c_void,
    conv_desc: miopenConvolutionDescriptor_t,
    algo: miopenConvFwdAlgorithm_t,
    workspace: *mut ::std::os::raw::c_void,
    workspace_size_in_bytes: usize,
    beta: *const ::std::os::raw::c_void,
    y_desc: miopenTensorDescriptor_t,
    y: *mut ::std::os::raw::c_void,
) -> miopenStatus_t {
    run_solution(
        handle,
        miopenProblemDirection_t::miopenProblemDirectionForward,
        alpha,
        beta,
        x_desc,
        x,
        w_desc,
        w,
        y_desc,
        y,
        y_desc,
        y,
        conv_desc,
        algo.0,
        workspace,
        workspace_size_in_bytes,
    )
}

pub(crate) unsafe fn convolution_backward_data(
    handle: &Context,
    alpha: *const ::core::ffi::c_void,
    w_desc: miopenTensorDescriptor_t,
    w: *const ::core::ffi::c_void,
    dy_desc: miopenTensorDescriptor_t,
    dy: *const ::core::ffi::c_void,
    conv_desc: miopenConvolutionDescriptor_t,
    algo: miopenConvBwdDataAlgorithm_t,
    workspace: *mut ::core::ffi::c_void,
    workspace_size_in_bytes: usize,
    beta: *const ::core::ffi::c_void,
    dx_desc: miopenTensorDescriptor_t,
    dx: *mut ::core::ffi::c_void,
) -> miopenStatus_t {
    run_solution(
        handle,
        miopenProblemDirection_t::miopenProblemDirectionBackward,
        alpha,
        beta,
        dx_desc,
        dx,
        w_desc,
        w,
        dy_desc,
        dy,
        dx_desc,
        dx,
        conv_desc,
        algo.0,
        workspace,
        workspace_size_in_bytes,
    )
}

unsafe fn run_solution(
    handle: &Context,
    direction: miopenProblemDirection_t,
    alpha: *const std::ffi::c_void,
    beta: *const std::ffi::c_void,
    mut x_desc: miopenTensorDescriptor_t,
    x: *const std::ffi::c_void,
    mut w_desc: miopenTensorDescriptor_t,
    w: *const std::ffi::c_void,
    mut y_desc: miopenTensorDescriptor_t,
    y: *const std::ffi::c_void,
    target_desc: miopenTensorDescriptor_t,
    target_buffer: *mut std::ffi::c_void,
    conv_desc: miopenConvolutionDescriptor_t,
    algo: u32,
    workspace: *mut std::ffi::c_void,
    workspace_size_in_bytes: usize,
) -> Result<(), miopenError_t> {
    let miopen = miopen()?;
    let tensors = [
        miopenTensorArgument_t {
            id: miopenTensorArgumentId_t::miopenTensorConvolutionX,
            descriptor: &mut x_desc,
            buffer: x.cast_mut(),
        },
        miopenTensorArgument_t {
            id: miopenTensorArgumentId_t::miopenTensorConvolutionW,
            descriptor: &mut w_desc,
            buffer: w.cast_mut(),
        },
        miopenTensorArgument_t {
            id: miopenTensorArgumentId_t::miopenTensorConvolutionY,
            descriptor: &mut y_desc,
            buffer: y.cast_mut(),
        },
    ];
    let do_convolution = move |solution: SolutionWithProperties, pre_op_buffer, is_f64| {
        miopen.miopenRunSolution(
            handle.base,
            solution.solution,
            tensors.len(),
            tensors.as_ptr(),
            workspace,
            workspace_size_in_bytes,
        )?;
        if let Some(pre_op_buffer) = pre_op_buffer {
            let zero = 0u64;
            let one_f32 = 1.0f32;
            let one_f64 = 1.0f64;
            miopen.miopenOpTensor(
                handle.base,
                miopenTensorOp_t::miopenTensorOpAdd,
                beta,
                target_desc,
                pre_op_buffer,
                if is_f64 {
                    std::ptr::from_ref(&one_f64).cast()
                } else {
                    std::ptr::from_ref(&one_f32).cast()
                },
                target_desc,
                target_buffer,
                std::ptr::from_ref(&zero).cast(),
                target_desc,
                target_buffer,
            )?;
        }
        Ok(())
    };
    let needs_beta = needs_beta(miopen, alpha, beta, conv_desc, target_desc)?;
    let search_workspace = &mut *handle
        .search_workspace
        .lock()
        .map_err(|_| miopenError_t::UnknownError)?;
    let solutions = search_workspace.search_cache.get_or_insert(
        miopen,
        handle.base,
        direction,
        conv_desc,
        x_desc,
        w_desc,
        y_desc,
    )?;
    let solution = *solutions
        .iter()
        .find(|s| s.algo.0 == algo && s.memory <= workspace_size_in_bytes)
        .or_else(|| {
            solutions
                .iter()
                .find(|s| s.memory <= workspace_size_in_bytes)
        })
        .ok_or(miopenError_t::UnsupportedOp)?;
    let is_f64 = match needs_beta {
        NeedsBeta::None => return do_convolution(solution, None, false),
        NeedsBeta::F32 => false,
        NeedsBeta::F64 => true,
    };
    let stream = handle.stream.load(Ordering::Acquire);
    let mut target_size = 0;
    miopen.miopenGetTensorNumBytes(target_desc, &mut target_size)?;
    search_workspace.beta_buffer_cache.with_async_buffer(
        target_size,
        hipStream_t(stream),
        |temp_buffer| {
            hipMemcpyDtoDAsync(
                hipDeviceptr_t(temp_buffer),
                hipDeviceptr_t(target_buffer),
                target_size,
                hipStream_t(stream),
            )
            .map_err(|_| miopenError_t::InternalError)?;
            do_convolution(solution, Some(temp_buffer), is_f64)
        },
    )
}

unsafe fn needs_beta(
    miopen: &MIOpenVtable,
    alpha: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    conv_desc: miopenConvolutionDescriptor_t,
    target_desc: miopenTensorDescriptor_t,
) -> Result<NeedsBeta, miopenError_t> {
    let mut dims = 0;
    miopen.miopenGetConvolutionSpatialDim(conv_desc, &mut dims)?;
    if dims != 2 {
        return Ok(NeedsBeta::None);
    }
    if dims > 5 {
        return Err(miopenError_t::UnsupportedOp);
    }
    let mut type_ = unsafe { mem::zeroed() };
    let mut _unused = [0i32; 5];
    miopen.miopenGetTensorDescriptor(
        target_desc,
        &mut type_,
        _unused.as_mut_ptr(),
        _unused.as_mut_ptr(),
    )?;
    let (alpha, beta, is_f64) = if type_ == miopenDataType_t::miopenDouble {
        (
            *alpha.cast::<f64>().as_ref().ok_or(miopenError_t::BadParm)?,
            *beta.cast::<f64>().as_ref().ok_or(miopenError_t::BadParm)?,
            true,
        )
    } else {
        (
            *alpha.cast::<f32>().as_ref().ok_or(miopenError_t::BadParm)? as f64,
            *beta.cast::<f32>().as_ref().ok_or(miopenError_t::BadParm)? as f64,
            false,
        )
    };
    Ok(match (alpha, beta) {
        (1.0, 0.0) => NeedsBeta::None,
        (1.0, _) => {
            if is_f64 {
                NeedsBeta::F64
            } else {
                NeedsBeta::F32
            }
        }
        _ => return Err(miopenError_t::BadParm),
    })
}

enum NeedsBeta {
    None,
    F32,
    F64,
}

pub(crate) unsafe fn get_convolution_backward_data_workspace_size(
    handle: &Context,
    w_desc: miopenTensorDescriptor_t,
    dy_desc: miopenTensorDescriptor_t,
    conv_desc: miopenConvolutionDescriptor_t,
    dx_desc: miopenTensorDescriptor_t,
    algo: miopenConvBwdDataAlgorithm_t,
    workspace_size_in_bytes: &mut usize,
) -> miopenStatus_t {
    get_workspace_size(
        handle,
        miopenProblemDirection_t::miopenProblemDirectionBackward,
        dx_desc,
        w_desc,
        dy_desc,
        conv_desc,
        algo.0,
        workspace_size_in_bytes,
    )
}

fn get_workspace_size(
    handle: &Context,
    direction: miopenProblemDirection_t,
    x_desc: miopenTensorDescriptor_t,
    w_desc: miopenTensorDescriptor_t,
    y_desc: miopenTensorDescriptor_t,
    conv_desc: miopenConvolutionDescriptor_t,
    algo: u32,
    workspace_size_in_bytes: &mut usize,
) -> Result<(), miopenError_t> {
    let miopen = miopen()?;
    let memory = {
        let search_workspace = &mut *handle
            .search_workspace
            .lock()
            .map_err(|_| miopenError_t::UnknownError)?;
        let solutions = search_workspace.search_cache.get_or_insert(
            miopen,
            handle.base,
            direction,
            conv_desc,
            x_desc,
            w_desc,
            y_desc,
        )?;
        solutions
            .iter()
            .find(|s| s.algo.0 == algo)
            .map(|s| s.memory)
            .ok_or(miopenError_t::UnsupportedOp)?
    };
    *workspace_size_in_bytes = memory;
    Ok(())
}

pub(crate) unsafe fn destroy_convolution_descriptor(
    conv_desc: miopenConvolutionDescriptor_t,
) -> miopenStatus_t {
    miopen()?.miopenDestroyConvolutionDescriptor(conv_desc)
}

pub(crate) unsafe fn destroy_filter_descriptor(desc: miopenTensorDescriptor_t) -> miopenStatus_t {
    miopen()?.miopenDestroyTensorDescriptor(desc)
}

pub(crate) unsafe fn destroy_tensor_descriptor(desc: miopenTensorDescriptor_t) -> miopenStatus_t {
    miopen()?.miopenDestroyTensorDescriptor(desc)
}

pub(crate) unsafe fn set_stream(handle: &Context, stream: hipStream_t) -> miopenStatus_t {
    miopen()?.miopenSetStream(handle.base, stream)?;
    handle.stream.store(stream.0, Ordering::Release);
    Ok(())
}

pub(crate) unsafe fn get_stream(handle: &Context, stream: &mut hipStream_t) -> miopenStatus_t {
    stream.0 = handle.stream.load(Ordering::Acquire);
    Ok(())
}

pub(crate) unsafe fn get_convolution_backward_data_algorithm_v7(
    handle: &Context,
    filter_desc: miopenTensorDescriptor_t,
    diff_desc: miopenTensorDescriptor_t,
    conv_desc: miopenConvolutionDescriptor_t,
    grad_desc: miopenTensorDescriptor_t,
    requested_algo_count: ::core::ffi::c_int,
    returned_algo_count: &mut ::core::ffi::c_int,
    perf_results: *mut cudnnConvolutionBwdDataAlgoPerf_t,
) -> Result<(), cudnnError_t> {
    get_algorithm(
        handle,
        miopenProblemDirection_t::miopenProblemDirectionBackward,
        grad_desc,
        filter_desc,
        diff_desc,
        conv_desc,
        requested_algo_count,
        returned_algo_count,
        perf_results,
        algo_perf_to_cudnn_bwd_data,
    )
}

unsafe fn get_algorithm<T>(
    handle: &Context,
    direction: miopenProblemDirection_t,
    x_desc: miopenTensorDescriptor_t,
    w_desc: miopenTensorDescriptor_t,
    y_desc: miopenTensorDescriptor_t,
    conv_desc: miopenConvolutionDescriptor_t,
    requested_algo_count: i32,
    returned_algo_count: &mut i32,
    perf_results: *mut T,
    mut converter: impl FnMut(SolutionWithProperties) -> T,
) -> Result<(), cudnnError_t> {
    if perf_results.is_null() || requested_algo_count <= 0 {
        return Err(cudnnError_t::BAD_PARAM);
    }
    let miopen = miopen()?;
    let search_workspace = &mut *handle
        .search_workspace
        .lock()
        .map_err(|_| miopenError_t::UnknownError)?;
    let solutions = search_workspace.search_cache.get_or_insert(
        miopen,
        handle.base,
        direction,
        conv_desc,
        x_desc,
        w_desc,
        y_desc,
    )?;
    let algo_count = solutions.len().min(requested_algo_count as usize);
    for (i, solution) in solutions.iter().enumerate().take(algo_count) {
        *perf_results.add(i) = converter(*solution);
    }
    *returned_algo_count = algo_count as ::core::ffi::c_int;
    Ok(())
}

pub(crate) unsafe fn get_convolution_backward_filter_algorithm_v7(
    handle: &Context,
    src_desc: miopenTensorDescriptor_t,
    diff_desc: miopenTensorDescriptor_t,
    conv_desc: miopenConvolutionDescriptor_t,
    grad_desc: miopenTensorDescriptor_t,
    requested_algo_count: ::core::ffi::c_int,
    returned_algo_count: &mut ::core::ffi::c_int,
    perf_results: &mut cudnnConvolutionBwdFilterAlgoPerf_t,
) -> Result<(), cudnnError_t> {
    get_algorithm(
        handle,
        miopenProblemDirection_t::miopenProblemDirectionBackwardWeights,
        src_desc,
        grad_desc,
        diff_desc,
        conv_desc,
        requested_algo_count,
        returned_algo_count,
        perf_results,
        algo_perf_to_cudnn_bwd_filter,
    )
}

pub(crate) unsafe fn convolution_backward_filter(
    handle: &Context,
    alpha: *const ::core::ffi::c_void,
    x_desc: miopenTensorDescriptor_t,
    x: *const ::core::ffi::c_void,
    dy_desc: miopenTensorDescriptor_t,
    dy: *const ::core::ffi::c_void,
    conv_desc: miopenConvolutionDescriptor_t,
    algo: miopenConvBwdWeightsAlgorithm_t,
    workspace: *mut ::core::ffi::c_void,
    workspace_size_in_bytes: usize,
    beta: *const ::core::ffi::c_void,
    dw_desc: miopenTensorDescriptor_t,
    dw: *mut ::core::ffi::c_void,
) -> miopenStatus_t {
    run_solution(
        handle,
        miopenProblemDirection_t::miopenProblemDirectionBackwardWeights,
        alpha,
        beta,
        x_desc,
        x,
        dw_desc,
        dw,
        dy_desc,
        dy,
        dw_desc,
        dw,
        conv_desc,
        algo.0,
        workspace,
        workspace_size_in_bytes,
    )
}

pub(crate) unsafe fn get_convolution_backward_filter_workspace_size(
    handle: &Context,
    x_desc: miopenTensorDescriptor_t,
    dy_desc: miopenTensorDescriptor_t,
    conv_desc: miopenConvolutionDescriptor_t,
    dw_desc: miopenTensorDescriptor_t,
    algo: miopenConvBwdWeightsAlgorithm_t,
    workspace_size_in_bytes: &mut usize,
) -> miopenStatus_t {
    get_workspace_size(
        handle,
        miopenProblemDirection_t::miopenProblemDirectionBackwardWeights,
        x_desc,
        dw_desc,
        dy_desc,
        conv_desc,
        algo.0,
        workspace_size_in_bytes,
    )
}

pub mod dnn8 {
    use cuda_types::cudnn8::*;
    use static_assertions::assert_eq_size;

    pub(crate) fn get_version() -> usize {
        return cuda_types::cudnn8::CUDNN_VERSION as usize;
    }

    pub(crate) fn get_error_string(
        status: cuda_types::cudnn8::cudnnStatus_t,
    ) -> *const ::core::ffi::c_char {
        super::dnn9::get_error_string(status8_to_9(status))
    }

    fn status8_to_9(
        status: cuda_types::cudnn8::cudnnStatus_t,
    ) -> cuda_types::cudnn9::cudnnStatus_t {
        match status {
            Ok(()) => Ok(()),
            Err(err) => Err(match err {
                cuda_types::cudnn8::cudnnError_t::NOT_INITIALIZED => {
                    cuda_types::cudnn9::cudnnError_t::NOT_INITIALIZED
                }
                cuda_types::cudnn8::cudnnError_t::ALLOC_FAILED => {
                    cuda_types::cudnn9::cudnnError_t::ALLOC_FAILED
                }
                cuda_types::cudnn8::cudnnError_t::BAD_PARAM => {
                    cuda_types::cudnn9::cudnnError_t::BAD_PARAM
                }
                cuda_types::cudnn8::cudnnError_t::INTERNAL_ERROR => {
                    cuda_types::cudnn9::cudnnError_t::INTERNAL_ERROR
                }
                cuda_types::cudnn8::cudnnError_t::INVALID_VALUE => {
                    cuda_types::cudnn9::cudnnError_t::INVALID_VALUE
                }
                cuda_types::cudnn8::cudnnError_t::ARCH_MISMATCH => {
                    cuda_types::cudnn9::cudnnError_t::ARCH_MISMATCH
                }
                cuda_types::cudnn8::cudnnError_t::MAPPING_ERROR => {
                    cuda_types::cudnn9::cudnnError_t::MAPPING_ERROR
                }
                cuda_types::cudnn8::cudnnError_t::EXECUTION_FAILED => {
                    cuda_types::cudnn9::cudnnError_t::EXECUTION_FAILED
                }
                cuda_types::cudnn8::cudnnError_t::NOT_SUPPORTED => {
                    cuda_types::cudnn9::cudnnError_t::NOT_SUPPORTED
                }
                cuda_types::cudnn8::cudnnError_t::LICENSE_ERROR => {
                    cuda_types::cudnn9::cudnnError_t::LICENSE_ERROR
                }
                cuda_types::cudnn8::cudnnError_t::RUNTIME_PREREQUISITE_MISSING => {
                    cuda_types::cudnn9::cudnnError_t::RUNTIME_PREREQUISITE_MISSING
                }
                cuda_types::cudnn8::cudnnError_t::RUNTIME_IN_PROGRESS => {
                    cuda_types::cudnn9::cudnnError_t::RUNTIME_IN_PROGRESS
                }
                cuda_types::cudnn8::cudnnError_t::RUNTIME_FP_OVERFLOW => {
                    cuda_types::cudnn9::cudnnError_t::RUNTIME_FP_OVERFLOW
                }
                cuda_types::cudnn8::cudnnError_t::VERSION_MISMATCH => {
                    cuda_types::cudnn9::cudnnError_t::VERSION_MISMATCH
                }
                _ => cuda_types::cudnn9::cudnnError_t::INTERNAL_ERROR,
            }),
        }
    }

    pub(crate) unsafe fn get_convolution_forward_algorithm_v7(
        handle: cudnnHandle_t,
        x_desc: cudnnTensorDescriptor_t,
        w_desc: cudnnFilterDescriptor_t,
        conv_desc: cudnnConvolutionDescriptor_t,
        y_desc: cudnnTensorDescriptor_t,
        requested_algo_count: ::std::os::raw::c_int,
        returned_algo_count: *mut ::std::os::raw::c_int,
        perf_results: *mut cudnnConvolutionFwdAlgoPerf_t,
    ) -> Result<(), cudnnError_t> {
        assert_eq_size!(
            cuda_types::cudnn8::cudnnConvolutionFwdAlgoPerf_t,
            cuda_types::cudnn9::cudnnConvolutionFwdAlgoPerf_t
        );
        super::dnn9::get_convolution_forward_algorithm_v7(
            handle,
            x_desc,
            w_desc,
            conv_desc,
            y_desc,
            requested_algo_count,
            returned_algo_count,
            perf_results.cast(),
        )?;
        Ok(())
    }

    pub(crate) unsafe fn get_convolution_backward_data_algorithm_v7(
        handle: cudnnHandle_t,
        filter_desc: cudnnFilterDescriptor_t,
        diff_desc: cudnnTensorDescriptor_t,
        conv_desc: cudnnConvolutionDescriptor_t,
        grad_desc: cudnnTensorDescriptor_t,
        requested_algo_count: ::core::ffi::c_int,
        returned_algo_count: *mut ::core::ffi::c_int,
        perf_results: *mut cudnnConvolutionBwdDataAlgoPerf_t,
    ) -> Result<(), cudnnError_t> {
        assert_eq_size!(
            cuda_types::cudnn8::cudnnConvolutionBwdDataAlgoPerf_t,
            cuda_types::cudnn9::cudnnConvolutionBwdDataAlgoPerf_t
        );
        super::dnn9::get_convolution_backward_data_algorithm_v7(
            handle,
            filter_desc,
            diff_desc,
            conv_desc,
            grad_desc,
            requested_algo_count,
            returned_algo_count,
            perf_results.cast(),
        )?;
        Ok(())
    }

    pub(crate) unsafe fn get_convolution_backward_filter_algorithm_v7(
        handle: cudnnHandle_t,
        src_desc: cudnnTensorDescriptor_t,
        diff_desc: cudnnTensorDescriptor_t,
        conv_desc: cudnnConvolutionDescriptor_t,
        grad_desc: cudnnFilterDescriptor_t,
        requested_algo_count: ::core::ffi::c_int,
        returned_algo_count: *mut ::core::ffi::c_int,
        perf_results: *mut cudnnConvolutionBwdFilterAlgoPerf_t,
    ) -> Result<(), cudnnError_t> {
        assert_eq_size!(
            cuda_types::cudnn8::cudnnConvolutionBwdFilterAlgoPerf_t,
            cuda_types::cudnn9::cudnnConvolutionBwdFilterAlgoPerf_t
        );
        super::dnn9::get_convolution_backward_filter_algorithm_v7(
            handle,
            src_desc,
            diff_desc,
            conv_desc,
            grad_desc,
            requested_algo_count,
            returned_algo_count,
            perf_results.cast(),
        )?;
        Ok(())
    }
}

pub mod dnn9 {
    use cuda_types::cudnn9::*;
    use zluda_common::FromCuda;

    pub(crate) fn get_version() -> usize {
        return cuda_types::cudnn9::CUDNN_VERSION as usize;
    }

    pub(crate) fn get_error_string(status: cudnnStatus_t) -> *const ::core::ffi::c_char {
        match status {
            Ok(()) => c"CUDNN_STATUS_SUCCESS",
            Err(err) => match err {
                cudnnError_t::NOT_INITIALIZED => c"CUDNN_STATUS_NOT_INITIALIZED",
                cudnnError_t::SUBLIBRARY_VERSION_MISMATCH => {
                    c"CUDNN_STATUS_SUBLIBRARY_VERSION_MISMATCH"
                }
                cudnnError_t::SERIALIZATION_VERSION_MISMATCH => {
                    c"CUDNN_STATUS_SERIALIZATION_VERSION_MISMATCH"
                }
                cudnnError_t::DEPRECATED => c"CUDNN_STATUS_DEPRECATED",
                cudnnError_t::LICENSE_ERROR => c"CUDNN_STATUS_LICENSE_ERROR",
                cudnnError_t::RUNTIME_IN_PROGRESS => c"CUDNN_STATUS_RUNTIME_IN_PROGRESS",
                cudnnError_t::RUNTIME_FP_OVERFLOW => c"CUDNN_STATUS_RUNTIME_FP_OVERFLOW",
                cudnnError_t::SUBLIBRARY_LOADING_FAILED => {
                    c"CUDNN_STATUS_SUBLIBRARY_LOADING_FAILED"
                }
                cudnnError_t::BAD_PARAM => c"CUDNN_STATUS_BAD_PARAM",
                cudnnError_t::BAD_PARAM_NULL_POINTER => c"CUDNN_STATUS_BAD_PARAM_NULL_POINTER",
                cudnnError_t::BAD_PARAM_MISALIGNED_POINTER => {
                    c"CUDNN_STATUS_BAD_PARAM_MISALIGNED_POINTER"
                }
                cudnnError_t::BAD_PARAM_NOT_FINALIZED => c"CUDNN_STATUS_BAD_PARAM_NOT_FINALIZED",
                cudnnError_t::BAD_PARAM_OUT_OF_BOUND => c"CUDNN_STATUS_BAD_PARAM_OUT_OF_BOUND",
                cudnnError_t::BAD_PARAM_SIZE_INSUFFICIENT => {
                    c"CUDNN_STATUS_BAD_PARAM_SIZE_INSUFFICIENT"
                }
                cudnnError_t::BAD_PARAM_STREAM_MISMATCH => {
                    c"CUDNN_STATUS_BAD_PARAM_STREAM_MISMATCH"
                }
                cudnnError_t::BAD_PARAM_SHAPE_MISMATCH => c"CUDNN_STATUS_BAD_PARAM_SHAPE_MISMATCH",
                cudnnError_t::BAD_PARAM_DUPLICATED_ENTRIES => {
                    c"CUDNN_STATUS_BAD_PARAM_DUPLICATED_ENTRIES"
                }
                cudnnError_t::BAD_PARAM_ATTRIBUTE_TYPE => c"CUDNN_STATUS_BAD_PARAM_ATTRIBUTE_TYPE",
                cudnnError_t::BAD_PARAM_CUDA_GRAPH_MISMATCH => {
                    c"CUDNN_STATUS_BAD_PARAM_CUDA_GRAPH_MISMATCH"
                }
                cudnnError_t::BAD_PARAM_DESCRIPTOR_TYPE => {
                    c"CUDNN_STATUS_BAD_PARAM_DESCRIPTOR_TYPE"
                }
                cudnnError_t::INVALID_VALUE => c"CUDNN_STATUS_INVALID_VALUE",
                cudnnError_t::NOT_SUPPORTED => c"CUDNN_STATUS_NOT_SUPPORTED",
                cudnnError_t::NOT_SUPPORTED_GRAPH_PATTERN => {
                    c"CUDNN_STATUS_NOT_SUPPORTED_GRAPH_PATTERN"
                }
                cudnnError_t::NOT_SUPPORTED_SHAPE => c"CUDNN_STATUS_NOT_SUPPORTED_SHAPE",
                cudnnError_t::NOT_SUPPORTED_DATA_TYPE => c"CUDNN_STATUS_NOT_SUPPORTED_DATA_TYPE",
                cudnnError_t::NOT_SUPPORTED_LAYOUT => c"CUDNN_STATUS_NOT_SUPPORTED_LAYOUT",
                cudnnError_t::NOT_SUPPORTED_INCOMPATIBLE_CUDA_DRIVER => {
                    c"CUDNN_STATUS_NOT_SUPPORTED_INCOMPATIBLE_CUDA_DRIVER"
                }
                cudnnError_t::NOT_SUPPORTED_INCOMPATIBLE_CUDART => {
                    c"CUDNN_STATUS_NOT_SUPPORTED_INCOMPATIBLE_CUDART"
                }
                cudnnError_t::NOT_SUPPORTED_SUBLIBRARY_UNAVAILABLE => {
                    c"CUDNN_STATUS_NOT_SUPPORTED_SUBLIBRARY_UNAVAILABLE"
                }
                cudnnError_t::NOT_SUPPORTED_SHARED_MEMORY_INSUFFICIENT => {
                    c"CUDNN_STATUS_NOT_SUPPORTED_SHARED_MEMORY_INSUFFICIENT"
                }
                cudnnError_t::NOT_SUPPORTED_PADDING => c"CUDNN_STATUS_NOT_SUPPORTED_PADDING",
                cudnnError_t::NOT_SUPPORTED_BAD_LAUNCH_PARAM => {
                    c"CUDNN_STATUS_NOT_SUPPORTED_BAD_LAUNCH_PARAM"
                }
                cudnnError_t::NOT_SUPPORTED_CUDA_GRAPH_NATIVE_API => {
                    c"CUDNN_STATUS_NOT_SUPPORTED_CUDA_GRAPH_NATIVE_API"
                }
                cudnnError_t::INTERNAL_ERROR => c"CUDNN_STATUS_INTERNAL_ERROR",
                cudnnError_t::INTERNAL_ERROR_COMPILATION_FAILED => {
                    c"CUDNN_STATUS_INTERNAL_ERROR_COMPILATION_FAILED"
                }
                cudnnError_t::INTERNAL_ERROR_UNEXPECTED_VALUE => {
                    c"CUDNN_STATUS_INTERNAL_ERROR_UNEXPECTED_VALUE"
                }
                cudnnError_t::INTERNAL_ERROR_HOST_ALLOCATION_FAILED => {
                    c"CUDNN_STATUS_INTERNAL_ERROR_HOST_ALLOCATION_FAILED"
                }
                cudnnError_t::INTERNAL_ERROR_DEVICE_ALLOCATION_FAILED => {
                    c"CUDNN_STATUS_INTERNAL_ERROR_DEVICE_ALLOCATION_FAILED"
                }
                cudnnError_t::INTERNAL_ERROR_BAD_LAUNCH_PARAM => {
                    c"CUDNN_STATUS_INTERNAL_ERROR_BAD_LAUNCH_PARAM"
                }
                cudnnError_t::INTERNAL_ERROR_TEXTURE_CREATION_FAILED => {
                    c"CUDNN_STATUS_INTERNAL_ERROR_TEXTURE_CREATION_FAILED"
                }
                cudnnError_t::EXECUTION_FAILED => c"CUDNN_STATUS_EXECUTION_FAILED",
                cudnnError_t::EXECUTION_FAILED_CUDA_DRIVER => {
                    c"CUDNN_STATUS_EXECUTION_FAILED_CUDA_DRIVER"
                }
                cudnnError_t::EXECUTION_FAILED_CUBLAS => c"CUDNN_STATUS_EXECUTION_FAILED_CUBLAS",
                cudnnError_t::EXECUTION_FAILED_CUDART => c"CUDNN_STATUS_EXECUTION_FAILED_CUDART",
                cudnnError_t::EXECUTION_FAILED_CURAND => c"CUDNN_STATUS_EXECUTION_FAILED_CURAND",
                _ => c"CUDNN_STATUS_UNKNOWN",
            },
        }
        .as_ptr()
    }

    pub(crate) unsafe fn get_convolution_forward_algorithm_v7(
        handle: cudnnHandle_t,
        x_desc: cudnnTensorDescriptor_t,
        w_desc: cudnnFilterDescriptor_t,
        conv_desc: cudnnConvolutionDescriptor_t,
        y_desc: cudnnTensorDescriptor_t,
        requested_algo_count: ::std::os::raw::c_int,
        returned_algo_count: *mut ::std::os::raw::c_int,
        perf_results: *mut cudnnConvolutionFwdAlgoPerf_t,
    ) -> Result<(), cudnnError_t> {
        super::get_convolution_forward_algorithm_v7(
            FromCuda::<_, cudnnError_t>::from_cuda(&handle)?,
            FromCuda::<_, cudnnError_t>::from_cuda(&x_desc)?,
            FromCuda::<_, cudnnError_t>::from_cuda(&w_desc)?,
            FromCuda::<_, cudnnError_t>::from_cuda(&conv_desc)?,
            FromCuda::<_, cudnnError_t>::from_cuda(&y_desc)?,
            requested_algo_count,
            FromCuda::<_, cudnnError_t>::from_cuda(&returned_algo_count)?,
            FromCuda::<_, cudnnError_t>::from_cuda(&perf_results)?,
        )?;
        Ok(())
    }

    pub(crate) unsafe fn get_convolution_backward_data_algorithm_v7(
        handle: cudnnHandle_t,
        filter_desc: cudnnFilterDescriptor_t,
        diff_desc: cudnnTensorDescriptor_t,
        conv_desc: cudnnConvolutionDescriptor_t,
        grad_desc: cudnnTensorDescriptor_t,
        requested_algo_count: ::core::ffi::c_int,
        returned_algo_count: *mut ::core::ffi::c_int,
        perf_results: *mut cudnnConvolutionBwdDataAlgoPerf_t,
    ) -> Result<(), cudnnError_t> {
        super::get_convolution_backward_data_algorithm_v7(
            FromCuda::<_, cudnnError_t>::from_cuda(&handle)?,
            FromCuda::<_, cudnnError_t>::from_cuda(&filter_desc)?,
            FromCuda::<_, cudnnError_t>::from_cuda(&diff_desc)?,
            FromCuda::<_, cudnnError_t>::from_cuda(&conv_desc)?,
            FromCuda::<_, cudnnError_t>::from_cuda(&grad_desc)?,
            requested_algo_count,
            FromCuda::<_, cudnnError_t>::from_cuda(&returned_algo_count)?,
            perf_results,
        )?;
        Ok(())
    }

    pub(crate) unsafe fn get_convolution_backward_filter_algorithm_v7(
        handle: cudnnHandle_t,
        src_desc: cudnnTensorDescriptor_t,
        diff_desc: cudnnTensorDescriptor_t,
        conv_desc: cudnnConvolutionDescriptor_t,
        grad_desc: cudnnFilterDescriptor_t,
        requested_algo_count: ::core::ffi::c_int,
        returned_algo_count: *mut ::core::ffi::c_int,
        perf_results: *mut cudnnConvolutionBwdFilterAlgoPerf_t,
    ) -> Result<(), cudnnError_t> {
        super::get_convolution_backward_filter_algorithm_v7(
            FromCuda::<_, cudnnError_t>::from_cuda(&handle)?,
            FromCuda::<_, cudnnError_t>::from_cuda(&src_desc)?,
            FromCuda::<_, cudnnError_t>::from_cuda(&diff_desc)?,
            FromCuda::<_, cudnnError_t>::from_cuda(&conv_desc)?,
            FromCuda::<_, cudnnError_t>::from_cuda(&grad_desc)?,
            requested_algo_count,
            FromCuda::<_, cudnnError_t>::from_cuda(&returned_algo_count)?,
            FromCuda::<_, cudnnError_t>::from_cuda(&perf_results)?,
        )?;
        Ok(())
    }
}
