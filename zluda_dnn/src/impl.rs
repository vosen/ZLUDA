use crate::MIOpenVtable;
use arrayvec::ArrayVec;
use cuda_types::cudnn9::*;
use hip_runtime_sys::*;
use miopen_sys::*;
use rustc_hash::FxHashMap;
use std::{
    alloc::Layout,
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
                search_cache: SearchCache {
                    conv_forward: FxHashMap::default(),
                    conv_backward_data: FxHashMap::default(),
                    conv_backward_filter: FxHashMap::default(),
                },
                search_cache2: SearchCache2::new(),
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

trait Operation {
    type Descriptor: Copy;
    type PerfResult: Clone;
    type CacheKey: std::hash::Hash + Eq;

    fn cache_key(
        miopen: &MIOpenVtable,
        descriptor: Self::Descriptor,
        tensors: &[miopenTensorDescriptor_t],
    ) -> Result<Self::CacheKey, miopenError_t>;

    fn get_cache<'a>(
        context: &'a mut SearchCache,
    ) -> &'a mut FxHashMap<Self::CacheKey, Self::PerfResult>;

    unsafe fn get_workspace_size(
        miopen: &MIOpenVtable,
        handle: miopenHandle_t,
        descriptor: Self::Descriptor,
        tensors: &[miopenTensorDescriptor_t],
    ) -> Result<usize, miopenError_t>;

    unsafe fn find_algorithm(
        miopen: &MIOpenVtable,
        handle: miopenHandle_t,
        descriptor: Self::Descriptor,
        tensors: &[miopenTensorDescriptor_t],
        fake_tensor: *mut ::std::os::raw::c_void,
        required_search_workspace_size: usize,
    ) -> Result<Option<Self::PerfResult>, miopenError_t>;
}

unsafe fn search_algorithm<O: Operation>(
    miopen: &MIOpenVtable,
    handle: miopenHandle_t,
    descriptor: O::Descriptor,
    tensors: &[miopenTensorDescriptor_t],
    scratchpad: &mut TemporaryBufferAllocator,
) -> Result<Option<O::PerfResult>, miopenError_t> {
    fn get_tensor_size(
        miopen: &MIOpenVtable,
        desc: miopenTensorDescriptor_t,
    ) -> Result<usize, miopenError_t> {
        let mut size_in_bytes = 0;
        unsafe { miopen.miopenGetTensorNumBytes(desc, &mut size_in_bytes)? };
        Ok(size_in_bytes)
    }
    let required_search_workspace_size =
        O::get_workspace_size(miopen, handle, descriptor, tensors)?;
    let fake_buffer_size = tensors
        .into_iter()
        .try_fold(required_search_workspace_size, |acc, tensor| {
            Ok(acc.max(get_tensor_size(miopen, *tensor)?))
        })?;
    let fake_tensor = scratchpad
        .get_or_allocate(fake_buffer_size)
        .map_err(|_| miopenError_t::AllocFailed)?;
    O::find_algorithm(
        miopen,
        handle,
        descriptor,
        tensors,
        fake_tensor.data,
        required_search_workspace_size,
    )
}

unsafe fn get_or_search_algorithm<O: Operation>(
    handle: &Context,
    descriptor: O::Descriptor,
    tensors: &[miopenTensorDescriptor_t],
) -> Result<Option<O::PerfResult>, miopenError_t> {
    let miopen = miopen()?;
    let mut search_workspace = handle
        .search_workspace
        .lock()
        .map_err(|_| miopenError_t::UnknownError)?;
    let search_workspace = &mut *search_workspace;
    let cache_key = O::cache_key(miopen, descriptor, tensors)?;
    let scratchpad = &mut search_workspace.beta_buffer_cache;
    let cache = O::get_cache(&mut search_workspace.search_cache);
    Ok(match cache.entry(cache_key) {
        std::collections::hash_map::Entry::Occupied(occupied_entry) => {
            Some(occupied_entry.get().clone())
        }
        std::collections::hash_map::Entry::Vacant(vacant_entry) => {
            search_algorithm::<O>(miopen, handle.base, descriptor, tensors, scratchpad)?
                .map(|x| vacant_entry.insert(x).clone())
        }
    })
}

struct ConvolutionForward;

// cuDNN is [x, w, y]
impl Operation for ConvolutionForward {
    type Descriptor = miopenConvolutionDescriptor_t;
    type PerfResult = miopenConvAlgoPerf_t;
    type CacheKey = ConvolutionOpCacheKey;

    fn cache_key(
        miopen: &MIOpenVtable,
        conv: Self::Descriptor,
        tensors: &[miopenTensorDescriptor_t],
    ) -> Result<Self::CacheKey, miopenError_t> {
        ConvolutionOpCacheKey::new(miopen, conv, tensors[0], tensors[1], tensors[2])
    }

    fn get_cache<'a>(
        context: &'a mut SearchCache,
    ) -> &'a mut FxHashMap<Self::CacheKey, Self::PerfResult> {
        &mut context.conv_forward
    }

    unsafe fn get_workspace_size(
        miopen: &MIOpenVtable,
        handle: miopenHandle_t,
        descriptor: Self::Descriptor,
        tensors: &[miopenTensorDescriptor_t],
    ) -> Result<usize, miopenError_t> {
        let mut required_search_workspace_size = 0;
        miopen.miopenConvolutionForwardGetWorkSpaceSize(
            handle,
            tensors[1],
            tensors[0],
            descriptor,
            tensors[2],
            &mut required_search_workspace_size,
        )?;
        Ok(required_search_workspace_size)
    }

    unsafe fn find_algorithm(
        miopen: &MIOpenVtable,
        handle: miopenHandle_t,
        descriptor: Self::Descriptor,
        tensors: &[miopenTensorDescriptor_t],
        fake_tensor: *mut ::std::os::raw::c_void,
        required_search_workspace_size: usize,
    ) -> Result<Option<Self::PerfResult>, miopenError_t> {
        let mut algo_count = 0;
        let mut perf_result = mem::zeroed();
        miopen.miopenFindConvolutionForwardAlgorithm(
            handle,
            tensors[0],
            fake_tensor,
            tensors[1],
            fake_tensor,
            descriptor,
            tensors[2],
            fake_tensor,
            1,
            &mut algo_count,
            &mut perf_result,
            fake_tensor,
            required_search_workspace_size,
            false,
        )?;
        if algo_count == 0 {
            Ok(None)
        } else {
            Ok(Some(perf_result))
        }
    }
}

struct ConvolutionBackwardData;

// cuDNN is [w, dy, dx]
// MIOpen is  [dy, w, dx]
impl Operation for ConvolutionBackwardData {
    type Descriptor = miopenConvolutionDescriptor_t;
    type PerfResult = miopenConvAlgoPerf_t;
    type CacheKey = ConvolutionOpCacheKey;

    fn cache_key(
        miopen: &MIOpenVtable,
        descriptor: Self::Descriptor,
        tensors: &[miopenTensorDescriptor_t],
    ) -> Result<Self::CacheKey, miopenError_t> {
        ConvolutionOpCacheKey::new(miopen, descriptor, tensors[1], tensors[0], tensors[2])
    }

    fn get_cache<'a>(
        context: &'a mut SearchCache,
    ) -> &'a mut FxHashMap<Self::CacheKey, Self::PerfResult> {
        &mut context.conv_backward_data
    }

    unsafe fn get_workspace_size(
        miopen: &MIOpenVtable,
        handle: miopenHandle_t,
        descriptor: Self::Descriptor,
        tensors: &[miopenTensorDescriptor_t],
    ) -> Result<usize, miopenError_t> {
        let mut required_search_workspace_size = 0;
        miopen.miopenConvolutionBackwardDataGetWorkSpaceSize(
            handle,
            tensors[1],
            tensors[0],
            descriptor,
            tensors[2],
            &mut required_search_workspace_size,
        )?;
        Ok(required_search_workspace_size)
    }

    unsafe fn find_algorithm(
        miopen: &MIOpenVtable,
        handle: miopenHandle_t,
        descriptor: Self::Descriptor,
        tensors: &[miopenTensorDescriptor_t],
        fake_tensor: *mut ::std::os::raw::c_void,
        required_search_workspace_size: usize,
    ) -> Result<Option<Self::PerfResult>, miopenError_t> {
        let mut algo_count = 0;
        let mut perf_result = mem::zeroed();
        miopen.miopenFindConvolutionBackwardDataAlgorithm(
            handle,
            tensors[1],
            fake_tensor,
            tensors[0],
            fake_tensor,
            descriptor,
            tensors[2],
            fake_tensor,
            1,
            &mut algo_count,
            &mut perf_result,
            fake_tensor,
            required_search_workspace_size,
            false,
        )?;
        if algo_count == 0 {
            Ok(None)
        } else {
            Ok(Some(perf_result))
        }
    }
}

struct ConvolutionBackwardFilter;

// cuDNN is [x, dy, dw]
// MIOpen is  [dy, x, dw]
impl Operation for ConvolutionBackwardFilter {
    type Descriptor = miopenConvolutionDescriptor_t;
    type PerfResult = miopenConvAlgoPerf_t;
    type CacheKey = ConvolutionOpCacheKey;

    fn cache_key(
        miopen: &MIOpenVtable,
        descriptor: Self::Descriptor,
        tensors: &[miopenTensorDescriptor_t],
    ) -> Result<Self::CacheKey, miopenError_t> {
        ConvolutionOpCacheKey::new(miopen, descriptor, tensors[1], tensors[0], tensors[2])
    }

    fn get_cache<'a>(
        context: &'a mut SearchCache,
    ) -> &'a mut FxHashMap<Self::CacheKey, Self::PerfResult> {
        &mut context.conv_backward_filter
    }

    unsafe fn get_workspace_size(
        miopen: &MIOpenVtable,
        handle: miopenHandle_t,
        descriptor: Self::Descriptor,
        tensors: &[miopenTensorDescriptor_t],
    ) -> Result<usize, miopenError_t> {
        let mut required_search_workspace_size = 0;
        miopen.miopenConvolutionBackwardWeightsGetWorkSpaceSize(
            handle,
            tensors[1],
            tensors[0],
            descriptor,
            tensors[2],
            &mut required_search_workspace_size,
        )?;
        Ok(required_search_workspace_size)
    }

    unsafe fn find_algorithm(
        miopen: &MIOpenVtable,
        handle: miopenHandle_t,
        descriptor: Self::Descriptor,
        tensors: &[miopenTensorDescriptor_t],
        fake_tensor: *mut ::std::os::raw::c_void,
        required_search_workspace_size: usize,
    ) -> Result<Option<Self::PerfResult>, miopenError_t> {
        let mut algo_count = 0;
        let mut perf_result = mem::zeroed();
        miopen.miopenFindConvolutionBackwardWeightsAlgorithm(
            handle,
            tensors[1],
            fake_tensor,
            tensors[0],
            fake_tensor,
            descriptor,
            tensors[2],
            fake_tensor,
            1,
            &mut algo_count,
            &mut perf_result,
            fake_tensor,
            required_search_workspace_size,
            false,
        )?;
        if algo_count == 0 {
            Ok(None)
        } else {
            Ok(Some(perf_result))
        }
    }
}

struct ContextCache {
    beta_buffer_cache: TemporaryBufferAllocator,
    search_cache: SearchCache,
    search_cache2: SearchCache2,
}

struct SearchCache2(FxHashMap<ConvolutionOpCacheKey2, Vec<miopenConvSolution_t>>);

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

impl SearchCache2 {
    fn new() -> Self {
        Self(FxHashMap::default())
    }

    fn get_or_insert<'a>(
        &'a mut self,
        miopen: &MIOpenVtable,
        handle: miopenHandle_t,
        direction: miopenProblemDirection_t,
        beta_buffer_cache: &mut TemporaryBufferAllocator,
        desc: miopenConvolutionDescriptor_t,
        x: miopenTensorDescriptor_t,
        w: miopenTensorDescriptor_t,
        y: miopenTensorDescriptor_t,
    ) -> Result<&'a Vec<miopenConvSolution_t>, miopenError_t> {
        let key = ConvolutionOpCacheKey2::new(miopen, direction, desc, x, w, y)?;
        Ok(match self.0.entry(key) {
            hash_map::Entry::Occupied(entry) => &*entry.into_mut(),
            hash_map::Entry::Vacant(entry) => {
                let fake_tensor_size = get_tensor_size(miopen, x)?
                    .max(get_tensor_size(miopen, w)?)
                    .max(get_tensor_size(miopen, y)?);
                let workspace_required =
                    get_workspace_size2(miopen, handle, direction, x, w, y, desc)?;
                let (layout, workspace_offset) =
                    unsafe { Layout::from_size_align_unchecked(fake_tensor_size, 1) }
                        .extend(unsafe {
                            Layout::from_size_align_unchecked(workspace_required, 128)
                        })
                        .map_err(|_| miopenError_t::InternalError)?;
                let buffer = beta_buffer_cache
                    .scavange_or_allocate_buffer(layout.size())
                    .map_err(|_| miopenError_t::InternalError)?;
                let solutions = find_algorithm2(
                    miopen,
                    handle,
                    direction,
                    x,
                    w,
                    y,
                    desc,
                    buffer.data,
                    workspace_offset,
                    workspace_required,
                )?;
                &*entry.insert(solutions)
            }
        })
    }

    // In theory MIOpen is capable of allocating the temporary buffers by itself
    // In practice, it sometimes fails when deallocating them, leading to
    // undebuggable crashes
    unsafe fn set_solution_options(
        miopen: &MIOpenVtable,
        handle: miopenHandle_t,
        direction: miopenProblemDirection_t,
        beta_buffer_cache: &mut TemporaryBufferAllocator,
        x: miopenTensorDescriptor_t,
        w: miopenTensorDescriptor_t,
        y: miopenTensorDescriptor_t,
        conv_desc: miopenConvolutionDescriptor_t,
        options: miopenFindOptions_t,
    ) -> Result<(), miopenError_t> {
        let fake_tensor_size = get_tensor_size(miopen, x)?
            .max(get_tensor_size(miopen, w)?)
            .max(get_tensor_size(miopen, y)?);
        let workspace_required =
            get_workspace_size2(miopen, handle, direction, x, w, y, conv_desc)?;
        let (layout, workspace_offset) = Layout::from_size_align_unchecked(fake_tensor_size, 1)
            .extend(Layout::from_size_align_unchecked(workspace_required, 128))
            .map_err(|_| miopenError_t::InternalError)?;
        let buffer = beta_buffer_cache
            .scavange_or_allocate_buffer(layout.size())
            .map_err(|_| miopenError_t::InternalError)?;
        miopen.miopenSetFindOptionPreallocatedWorkspace(
            options,
            buffer.data.cast::<u8>().add(workspace_offset).cast(),
            workspace_required,
        )?;
        miopen.miopenSetFindOptionPreallocatedTensor(
            options,
            miopenTensorArgumentId_t::miopenTensorConvolutionX,
            buffer.data,
        )?;
        miopen.miopenSetFindOptionPreallocatedTensor(
            options,
            miopenTensorArgumentId_t::miopenTensorConvolutionW,
            buffer.data,
        )?;
        miopen.miopenSetFindOptionPreallocatedTensor(
            options,
            miopenTensorArgumentId_t::miopenTensorConvolutionY,
            buffer.data,
        )?;
        Ok(())
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

fn get_tensor_size(
    miopen: &MIOpenVtable,
    desc: miopenTensorDescriptor_t,
) -> Result<usize, miopenError_t> {
    let mut size_in_bytes = 0;
    unsafe { miopen.miopenGetTensorNumBytes(desc, &mut size_in_bytes)? };
    Ok(size_in_bytes)
}

fn get_workspace_size2(
    miopen: &MIOpenVtable,
    handle: miopenHandle_t,
    direction: miopenProblemDirection_t,
    x: miopenTensorDescriptor_t,
    w: miopenTensorDescriptor_t,
    y: miopenTensorDescriptor_t,
    conv_desc: miopenConvolutionDescriptor_t,
) -> Result<usize, miopenError_t> {
    let mut required_search_workspace_size = 0;
    match direction {
        miopenProblemDirection_t::miopenProblemDirectionForward => unsafe {
            miopen.miopenConvolutionForwardGetWorkSpaceSize(
                handle,
                w,
                x,
                conv_desc,
                y,
                &mut required_search_workspace_size,
            )
        },
        miopenProblemDirection_t::miopenProblemDirectionBackward => unsafe {
            miopen.miopenConvolutionBackwardDataGetWorkSpaceSize(
                handle,
                y,
                w,
                conv_desc,
                x,
                &mut required_search_workspace_size,
            )
        },
        miopenProblemDirection_t::miopenProblemDirectionBackwardWeights => unsafe {
            miopen.miopenConvolutionBackwardWeightsGetWorkSpaceSize(
                handle,
                y,
                x,
                conv_desc,
                w,
                &mut required_search_workspace_size,
            )
        },
        _ => return Err(miopenError_t::UnsupportedOp),
    }?;
    Ok(required_search_workspace_size)
}

fn find_algorithm2(
    miopen: &MIOpenVtable,
    handle: miopenHandle_t,
    direction: miopenProblemDirection_t,
    x: miopenTensorDescriptor_t,
    w: miopenTensorDescriptor_t,
    y: miopenTensorDescriptor_t,
    conv_desc: miopenConvolutionDescriptor_t,
    temporary_memory: *mut ::std::os::raw::c_void,
    workspace_memory_offset: usize,
    workspace_memory_size: usize,
) -> Result<Vec<miopenConvSolution_t>, miopenError_t> {
    let mut returned_algo_count = 0;
    let mut perf_results = unsafe { mem::zeroed::<miopenConvAlgoPerf_t>() };
    match direction {
        miopenProblemDirection_t::miopenProblemDirectionForward => unsafe {
            miopen.miopenFindConvolutionForwardAlgorithm(
                handle,
                x,
                temporary_memory,
                w,
                temporary_memory,
                conv_desc,
                y,
                temporary_memory,
                1,
                &mut returned_algo_count,
                &mut perf_results,
                temporary_memory
                    .cast::<u8>()
                    .add(workspace_memory_offset)
                    .cast(),
                workspace_memory_size,
                false,
            )?;
            let mut solution_count = 0;
            miopen.miopenConvolutionForwardGetSolutionCount(
                handle,
                w,
                x,
                conv_desc,
                y,
                &mut solution_count,
            )?;
            let mut solutions =
                vec![mem::zeroed::<miopenConvSolution_t>(); solution_count as usize];
            let mut returned_solution_count = 0;
            miopen.miopenConvolutionForwardGetSolution(
                handle,
                w,
                x,
                conv_desc,
                y,
                solution_count,
                &mut returned_solution_count,
                solutions.as_mut_ptr(),
            )?;
            solutions.truncate(returned_solution_count as usize);
            Ok(solutions)
        },
        miopenProblemDirection_t::miopenProblemDirectionBackward => unsafe {
            miopen.miopenFindConvolutionBackwardDataAlgorithm(
                handle,
                y,
                temporary_memory,
                w,
                temporary_memory,
                conv_desc,
                x,
                temporary_memory,
                1,
                &mut returned_algo_count,
                &mut perf_results,
                temporary_memory
                    .cast::<u8>()
                    .add(workspace_memory_offset)
                    .cast(),
                workspace_memory_size,
                false,
            )?;
            let mut solution_count = 0;
            miopen.miopenConvolutionBackwardDataGetSolutionCount(
                handle,
                y,
                w,
                conv_desc,
                x,
                &mut solution_count,
            )?;
            let mut solutions =
                vec![mem::zeroed::<miopenConvSolution_t>(); solution_count as usize];
            let mut returned_solution_count = 0;
            miopen.miopenConvolutionBackwardDataGetSolution(
                handle,
                y,
                w,
                conv_desc,
                x,
                solution_count,
                &mut returned_solution_count,
                solutions.as_mut_ptr(),
            )?;
            solutions.truncate(returned_solution_count as usize);
            Ok(solutions)
        },
        miopenProblemDirection_t::miopenProblemDirectionBackwardWeights => unsafe {
            miopen.miopenFindConvolutionBackwardWeightsAlgorithm(
                handle,
                y,
                temporary_memory,
                x,
                temporary_memory,
                conv_desc,
                w,
                temporary_memory,
                1,
                &mut returned_algo_count,
                &mut perf_results,
                temporary_memory
                    .cast::<u8>()
                    .add(workspace_memory_offset)
                    .cast(),
                workspace_memory_size,
                true,
            )?;
            let mut solution_count = 0;
            miopen.miopenConvolutionBackwardWeightsGetSolutionCount(
                handle,
                y,
                w,
                conv_desc,
                x,
                &mut solution_count,
            )?;
            let mut solutions =
                vec![mem::zeroed::<miopenConvSolution_t>(); solution_count as usize];
            let mut returned_solution_count = 0;
            miopen.miopenConvolutionBackwardWeightsGetSolution(
                handle,
                y,
                w,
                conv_desc,
                x,
                solution_count,
                &mut returned_solution_count,
                solutions.as_mut_ptr(),
            )?;
            solutions.truncate(returned_solution_count as usize);
            Ok(solutions)
        },
        _ => return Err(miopenError_t::UnsupportedOp),
    }
}

struct DropFn<F: FnMut()>(F);

impl<F: FnMut()> Drop for DropFn<F> {
    fn drop(&mut self) {
        (self.0)();
    }
}

#[derive(PartialEq, Eq, Hash)]
struct ConvolutionOpCacheKey2 {
    direction: miopenProblemDirection_t,
    x: TensorCacheKey,
    w: TensorCacheKey,
    conv: ConvolutionDescriptorCacheKey,
    y: TensorCacheKey,
}

impl ConvolutionOpCacheKey2 {
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

struct SearchCache {
    conv_forward: FxHashMap<ConvolutionOpCacheKey, miopenConvAlgoPerf_t>,
    conv_backward_data: FxHashMap<ConvolutionOpCacheKey, miopenConvAlgoPerf_t>,
    conv_backward_filter: FxHashMap<ConvolutionOpCacheKey, miopenConvAlgoPerf_t>,
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

    fn get_or_allocate(&mut self, size: usize) -> Result<&BetaBuffer, hipErrorCode_t> {
        let mut buffer = self.scavange_or_allocate_buffer(size)?;
        if let Some(event) = buffer.free.take() {
            unsafe { hipEventDestroy(event) }.ok();
        }
        self.buffers.push_front(buffer);
        Ok(&self.buffers.front().unwrap())
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
        algo_perf_to_cudnn2,
    )
}

#[derive(PartialEq, Eq, Hash)]
struct ConvolutionOpCacheKey {
    x: TensorCacheKey,
    w: TensorCacheKey,
    conv: ConvolutionDescriptorCacheKey,
    y: TensorCacheKey,
}

impl ConvolutionOpCacheKey {
    fn new(
        miopen: &MIOpenVtable,
        conv_desc: miopenConvolutionDescriptor_t,
        x_desc: miopenTensorDescriptor_t,
        w_desc: miopenTensorDescriptor_t,
        y_desc: miopenTensorDescriptor_t,
    ) -> Result<Self, miopenError_t> {
        Ok(Self {
            x: TensorCacheKey::new(miopen, x_desc)?,
            w: TensorCacheKey::new(miopen, w_desc)?,
            conv: ConvolutionDescriptorCacheKey::new(miopen, conv_desc)?,
            y: TensorCacheKey::new(miopen, y_desc)?,
        })
    }
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

unsafe fn algo_perf_to_cudnn(result: miopenConvAlgoPerf_t) -> cudnnConvolutionFwdAlgoPerf_t {
    cudnnConvolutionFwdAlgoPerf_t {
        algo: algo_to_cudnn(&result),
        time: result.time,
        memory: result.memory,
        status: cudnnStatus_t::SUCCESS,
        determinism: cudnnDeterminism_t::CUDNN_NON_DETERMINISTIC,
        mathType: cudnnMathType_t::CUDNN_DEFAULT_MATH,
        reserved: [0, 0, 0],
    }
}

unsafe fn algo_perf_to_cudnn_bwd_data(
    result: miopenConvAlgoPerf_t,
) -> cudnnConvolutionBwdDataAlgoPerf_t {
    cudnnConvolutionBwdDataAlgoPerf_t {
        algo: algo_to_cudnn_bwd_data(&result),
        time: result.time,
        memory: result.memory,
        status: cudnnStatus_t::SUCCESS,
        determinism: cudnnDeterminism_t::CUDNN_NON_DETERMINISTIC,
        mathType: cudnnMathType_t::CUDNN_DEFAULT_MATH,
        reserved: [0, 0, 0],
    }
}

fn algo_perf_to_cudnn2(solution: &miopenConvSolution_t) -> cudnnConvolutionFwdAlgoPerf_t {
    cudnnConvolutionFwdAlgoPerf_t {
        algo: algo_to_cudnn2(solution.algorithm),
        time: solution.time,
        memory: solution.workspace_size,
        status: cudnnStatus_t::SUCCESS,
        determinism: cudnnDeterminism_t::CUDNN_NON_DETERMINISTIC,
        mathType: cudnnMathType_t::CUDNN_DEFAULT_MATH,
        reserved: [0, 0, 0],
    }
}

fn algo_perf_to_cudnn_bwd_data2(
    solution: &miopenConvSolution_t,
) -> cudnnConvolutionBwdDataAlgoPerf_t {
    cudnnConvolutionBwdDataAlgoPerf_t {
        algo: algo_to_cudnn_bwd_data2(solution.algorithm),
        time: solution.time,
        memory: solution.workspace_size,
        status: cudnnStatus_t::SUCCESS,
        determinism: cudnnDeterminism_t::CUDNN_NON_DETERMINISTIC,
        mathType: cudnnMathType_t::CUDNN_DEFAULT_MATH,
        reserved: [0, 0, 0],
    }
}

unsafe fn algo_perf_to_cudnn_bwd_filter(
    result: miopenConvAlgoPerf_t,
) -> cudnnConvolutionBwdFilterAlgoPerf_t {
    cudnnConvolutionBwdFilterAlgoPerf_t {
        algo: algo_to_cudnn_bwd_filter(&result),
        time: result.time,
        memory: result.memory,
        status: cudnnStatus_t::SUCCESS,
        determinism: cudnnDeterminism_t::CUDNN_NON_DETERMINISTIC,
        mathType: cudnnMathType_t::CUDNN_DEFAULT_MATH,
        reserved: [0, 0, 0],
    }
}

fn algo_perf_to_cudnn_bwd_filter2(
    solution: &miopenConvSolution_t,
) -> cudnnConvolutionBwdFilterAlgoPerf_t {
    cudnnConvolutionBwdFilterAlgoPerf_t {
        algo: algo_to_cudnn_bwd_filter2(solution.algorithm),
        time: solution.time,
        memory: solution.workspace_size,
        status: cudnnStatus_t::SUCCESS,
        determinism: cudnnDeterminism_t::CUDNN_NON_DETERMINISTIC,
        mathType: cudnnMathType_t::CUDNN_DEFAULT_MATH,
        reserved: [0, 0, 0],
    }
}

unsafe fn algo_to_cudnn(result: &miopenConvAlgoPerf_t) -> cudnnConvolutionFwdAlgo_t {
    match result.__bindgen_anon_1.fwd_algo {
        miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoGEMM => {
            cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_GEMM
        }
        miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoDirect => {
            cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_DIRECT
        }
        miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoFFT => {
            cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_FFT
        }
        miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoWinograd => {
            cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_WINOGRAD
        }
        miopenConvFwdAlgorithm_t::miopenConvolutionFwdAlgoImplicitGEMM => {
            cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_IMPLICIT_GEMM
        }
        _ => cudnnConvolutionFwdAlgo_t::CUDNN_CONVOLUTION_FWD_ALGO_GEMM,
    }
}

fn algo_to_cudnn2(result: miopenConvAlgorithm_t) -> cudnnConvolutionFwdAlgo_t {
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

unsafe fn algo_to_cudnn_bwd_data(result: &miopenConvAlgoPerf_t) -> cudnnConvolutionBwdDataAlgo_t {
    match result.__bindgen_anon_1.bwd_data_algo {
        miopenConvBwdDataAlgorithm_t::miopenConvolutionBwdDataAlgoGEMM => {
            cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_0
        }
        miopenConvBwdDataAlgorithm_t::miopenConvolutionBwdDataAlgoDirect => {
            cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_1
        }
        miopenConvBwdDataAlgorithm_t::miopenConvolutionBwdDataAlgoFFT => {
            cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_FFT
        }
        miopenConvBwdDataAlgorithm_t::miopenConvolutionBwdDataAlgoWinograd => {
            cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_WINOGRAD
        }
        _ => cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_0,
    }
}

fn algo_to_cudnn_bwd_filter2(algo: miopenConvAlgorithm_t) -> cudnnConvolutionBwdFilterAlgo_t {
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

fn algo_to_cudnn_bwd_data2(algo: miopenConvAlgorithm_t) -> cudnnConvolutionBwdDataAlgo_t {
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
        miopenConvAlgorithm_t::miopenConvolutionAlgoImplicitGEMM => {
            cudnnConvolutionBwdDataAlgo_t(7)
        }
        _ => cudnnConvolutionBwdDataAlgo_t::CUDNN_CONVOLUTION_BWD_DATA_ALGO_0,
    }
}

unsafe fn algo_to_cudnn_bwd_filter(
    result: &miopenConvAlgoPerf_t,
) -> cudnnConvolutionBwdFilterAlgo_t {
    match result.__bindgen_anon_1.bwd_weights_algo {
        miopenConvBwdWeightsAlgorithm_t::miopenConvolutionBwdWeightsAlgoGEMM => {
            cudnnConvolutionBwdFilterAlgo_t::CUDNN_CONVOLUTION_BWD_FILTER_ALGO_0
        }
        miopenConvBwdWeightsAlgorithm_t::miopenConvolutionBwdWeightsAlgoDirect => {
            cudnnConvolutionBwdFilterAlgo_t::CUDNN_CONVOLUTION_BWD_FILTER_ALGO_1
        }
        miopenConvBwdWeightsAlgorithm_t::miopenConvolutionBwdWeightsAlgoWinograd => {
            cudnnConvolutionBwdFilterAlgo_t::CUDNN_CONVOLUTION_BWD_FILTER_ALGO_WINOGRAD
        }
        miopenConvBwdWeightsAlgorithm_t::miopenConvolutionBwdWeightsAlgoImplicitGEMM => {
            cudnnConvolutionBwdFilterAlgo_t::CUDNN_CONVOLUTION_BWD_FILTER_ALGO_3
        }
        _ => cudnnConvolutionBwdFilterAlgo_t::CUDNN_CONVOLUTION_BWD_FILTER_ALGO_0,
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
    conv_desc: miopenConvolutionDescriptor_t,
    algo: u32,
    workspace: *mut std::ffi::c_void,
    workspace_size_in_bytes: usize,
) -> Result<(), miopenError_t> {
    let miopen = miopen()?;
    check_alpha_beta(miopen, alpha, beta, target_desc)?;
    let solution = {
        let search_workspace = &mut *handle
            .search_workspace
            .lock()
            .map_err(|_| miopenError_t::UnknownError)?;
        let solutions = search_workspace.search_cache2.get_or_insert(
            miopen,
            handle.base,
            direction,
            &mut search_workspace.beta_buffer_cache,
            conv_desc,
            x_desc,
            w_desc,
            y_desc,
        )?;
        *solutions
            .iter()
            .find(|s| s.algorithm.0 == algo)
            .unwrap_or(solutions.first().ok_or(miopenError_t::UnsupportedOp)?)
    };
    match direction {
        miopenProblemDirection_t::miopenProblemDirectionForward => miopen
            .miopenConvolutionForwardImmediate(
                handle.base,
                w_desc,
                w,
                x_desc,
                x,
                conv_desc,
                y_desc,
                y.cast_mut(),
                workspace,
                workspace_size_in_bytes,
                solution.solution_id,
            ),
        miopenProblemDirection_t::miopenProblemDirectionBackward => miopen
            .miopenConvolutionBackwardDataImmediate(
                handle.base,
                y_desc,
                y,
                w_desc,
                w,
                conv_desc,
                x_desc,
                x.cast_mut(),
                workspace,
                workspace_size_in_bytes,
                solution.solution_id,
            ),
        miopenProblemDirection_t::miopenProblemDirectionBackwardWeights => miopen
            .miopenConvolutionBackwardWeightsImmediate(
                handle.base,
                y_desc,
                y,
                x_desc,
                x,
                conv_desc,
                w_desc,
                w.cast_mut(),
                workspace,
                workspace_size_in_bytes,
                solution.solution_id,
            ),
        _ => return Err(miopenError_t::UnsupportedOp),
    }
}

unsafe fn check_alpha_beta(
    miopen: &MIOpenVtable,
    alpha: *const ::core::ffi::c_void,
    beta: *const ::core::ffi::c_void,
    conv_desc: miopenTensorDescriptor_t,
) -> Result<(), miopenError_t> {
    let mut type_ = unsafe { mem::zeroed() };
    miopen.miopenGetTensorDescriptor(conv_desc, &mut type_, &mut 0, &mut 0)?;
    let (alpha, beta) = if type_ == miopenDataType_t::miopenDouble {
        (
            *alpha.cast::<f64>().as_ref().ok_or(miopenError_t::BadParm)?,
            *beta.cast::<f64>().as_ref().ok_or(miopenError_t::BadParm)?,
        )
    } else {
        (
            *alpha.cast::<f32>().as_ref().ok_or(miopenError_t::BadParm)? as f64,
            *beta.cast::<f32>().as_ref().ok_or(miopenError_t::BadParm)? as f64,
        )
    };
    if (alpha, beta) != (1.0, 0.0) {
        return Err(miopenError_t::BadParm);
    }
    Ok(())
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
        let solutions = search_workspace.search_cache2.get_or_insert(
            miopen,
            handle.base,
            direction,
            &mut search_workspace.beta_buffer_cache,
            conv_desc,
            x_desc,
            w_desc,
            y_desc,
        )?;
        solutions
            .iter()
            .find(|s| s.algorithm.0 == algo)
            .map(|s| s.workspace_size)
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
        algo_perf_to_cudnn_bwd_data2,
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
    mut converter: impl FnMut(&miopenConvSolution_t) -> T,
) -> Result<(), cudnnError_t> {
    if perf_results.is_null() || requested_algo_count <= 0 {
        return Err(cudnnError_t::BAD_PARAM);
    }
    let miopen = miopen()?;
    let search_workspace = &mut *handle
        .search_workspace
        .lock()
        .map_err(|_| miopenError_t::UnknownError)?;
    let solutions = search_workspace.search_cache2.get_or_insert(
        miopen,
        handle.base,
        direction,
        &mut search_workspace.beta_buffer_cache,
        conv_desc,
        x_desc,
        w_desc,
        y_desc,
    )?;
    if direction == miopenProblemDirection_t::miopenProblemDirectionBackward {
        for s in solutions {
            println!(
                "Found solution: algo={:?}, time={}ms, memory={} bytes, solution_id={}",
                s.algorithm, s.time, s.workspace_size, s.solution_id
            );
        }
    }
    println!("Total solutions found: {}", solutions.len());
    let algo_count = solutions.len().min(requested_algo_count as usize);
    println!("Returning top {} solutions", algo_count);
    for (i, solution) in solutions.iter().enumerate().take(algo_count) {
        *perf_results.add(i) = converter(solution);
    }
    println!("Returned {} solutions", algo_count);
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
        algo_perf_to_cudnn_bwd_filter2,
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
    use miopen_sys::miopenProblemDirection_t;
    use std::mem;

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
        if requested_algo_count <= 0 {
            return Err(cudnnError_t::BAD_PARAM);
        }
        let mut perf_results_dnn9 = mem::zeroed();
        super::dnn9::get_convolution_forward_algorithm_v7(
            handle,
            x_desc,
            w_desc,
            conv_desc,
            y_desc,
            1,
            returned_algo_count,
            &mut perf_results_dnn9,
        )?;
        *perf_results = cudnnConvolutionFwdAlgoPerf_t {
            algo: perf_results_dnn9.algo,
            time: perf_results_dnn9.time,
            memory: perf_results_dnn9.memory,
            status: status9_to_8(perf_results_dnn9.status),
            determinism: perf_results_dnn9.determinism,
            mathType: perf_results_dnn9.mathType,
            reserved: [0, 0, 0],
        };
        Ok(())
    }

    fn status9_to_8(
        status: cuda_types::cudnn9::cudnnStatus_t,
    ) -> cuda_types::cudnn8::cudnnStatus_t {
        match status {
            Ok(()) => Ok(()),
            Err(err) => Err(match err {
                cuda_types::cudnn9::cudnnError_t::NOT_INITIALIZED => {
                    cuda_types::cudnn8::cudnnError_t::NOT_INITIALIZED
                }
                cuda_types::cudnn9::cudnnError_t::ALLOC_FAILED => {
                    cuda_types::cudnn8::cudnnError_t::ALLOC_FAILED
                }
                cuda_types::cudnn9::cudnnError_t::BAD_PARAM => {
                    cuda_types::cudnn8::cudnnError_t::BAD_PARAM
                }
                cuda_types::cudnn9::cudnnError_t::INTERNAL_ERROR => {
                    cuda_types::cudnn8::cudnnError_t::INTERNAL_ERROR
                }
                cuda_types::cudnn9::cudnnError_t::INVALID_VALUE => {
                    cuda_types::cudnn8::cudnnError_t::INVALID_VALUE
                }
                cuda_types::cudnn9::cudnnError_t::ARCH_MISMATCH => {
                    cuda_types::cudnn8::cudnnError_t::ARCH_MISMATCH
                }
                cuda_types::cudnn9::cudnnError_t::MAPPING_ERROR => {
                    cuda_types::cudnn8::cudnnError_t::MAPPING_ERROR
                }
                cuda_types::cudnn9::cudnnError_t::EXECUTION_FAILED => {
                    cuda_types::cudnn8::cudnnError_t::EXECUTION_FAILED
                }
                cuda_types::cudnn9::cudnnError_t::NOT_SUPPORTED => {
                    cuda_types::cudnn8::cudnnError_t::NOT_SUPPORTED
                }
                cuda_types::cudnn9::cudnnError_t::LICENSE_ERROR => {
                    cuda_types::cudnn8::cudnnError_t::LICENSE_ERROR
                }
                cuda_types::cudnn9::cudnnError_t::RUNTIME_PREREQUISITE_MISSING => {
                    cuda_types::cudnn8::cudnnError_t::RUNTIME_PREREQUISITE_MISSING
                }
                cuda_types::cudnn9::cudnnError_t::RUNTIME_IN_PROGRESS => {
                    cuda_types::cudnn8::cudnnError_t::RUNTIME_IN_PROGRESS
                }
                cuda_types::cudnn9::cudnnError_t::RUNTIME_FP_OVERFLOW => {
                    cuda_types::cudnn8::cudnnError_t::RUNTIME_FP_OVERFLOW
                }
                cuda_types::cudnn9::cudnnError_t::VERSION_MISMATCH => {
                    cuda_types::cudnn8::cudnnError_t::VERSION_MISMATCH
                }
                _ => cuda_types::cudnn8::cudnnError_t::INTERNAL_ERROR,
            }),
        }
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
        if requested_algo_count <= 0 {
            return Err(cudnnError_t::BAD_PARAM);
        }
        let mut perf_results_dnn9 = mem::zeroed();
        super::dnn9::get_convolution_backward_data_algorithm_v7(
            handle,
            filter_desc,
            diff_desc,
            conv_desc,
            grad_desc,
            requested_algo_count,
            returned_algo_count,
            &mut perf_results_dnn9,
        )?;
        *perf_results = cudnnConvolutionBwdDataAlgoPerf_t {
            algo: perf_results_dnn9.algo,
            time: perf_results_dnn9.time,
            memory: perf_results_dnn9.memory,
            status: status9_to_8(perf_results_dnn9.status),
            determinism: perf_results_dnn9.determinism,
            mathType: perf_results_dnn9.mathType,
            reserved: [0, 0, 0],
        };
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
        if requested_algo_count <= 0 {
            return Err(cudnnError_t::BAD_PARAM);
        }
        let mut perf_results_dnn9 = mem::zeroed();
        super::dnn9::get_convolution_backward_filter_algorithm_v7(
            handle,
            src_desc,
            diff_desc,
            conv_desc,
            grad_desc,
            requested_algo_count,
            returned_algo_count,
            &mut perf_results_dnn9,
        )?;
        *perf_results = cudnnConvolutionBwdFilterAlgoPerf_t {
            algo: perf_results_dnn9.algo,
            time: perf_results_dnn9.time,
            memory: perf_results_dnn9.memory,
            status: status9_to_8(perf_results_dnn9.status),
            determinism: perf_results_dnn9.determinism,
            mathType: perf_results_dnn9.mathType,
            reserved: [0, 0, 0],
        };
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
