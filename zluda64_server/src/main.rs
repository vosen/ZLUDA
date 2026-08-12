// Prevent the console window from appearing when running the server on Windows
#![windows_subsystem = "windows"]

use compio::{
    buf::{buf_try, IoBuf, IoBufMut, ReserveError, ReserveExactError, SetLen},
    fs::named_pipe::{ClientOptions, NamedPipeClient},
    io::{AsyncReadExt, AsyncWriteExt},
    BufResult,
};
use cuda_macros::cuda_function_declarations;
use cuda_types::cuda::*;
use dark_api::FunctionArgInfo;
use rkyv::{
    api::high::HighSerializer,
    rend::{u32_le, u64_le},
    ser::allocator::ArenaHandle,
    util::AlignedVec,
    Archive, Portable, Serialize,
};
use rustc_hash::FxHashMap;
use slab::Slab;
use std::{
    collections::BTreeMap,
    ffi::{c_void, CStr},
    mem,
    ops::Range,
    ptr,
    rc::Rc,
};
use windows::core::PCSTR;
use windows::Win32::Foundation::*;
use windows::Win32::System::Memory::*;
use windows::Win32::System::Threading::*;
use zluda_server_common::*;

struct State {
    ctx: CUcontext,
    device: i32,
    handles: HandlePool,
    devmemory: Allocator,
    modules: ModuleLaunchData,
}

struct HandlePool {
    handles: Slab<usize>,
}

impl HandlePool {
    const OFFSET: u32 = 512 * 1024 * 1024;

    fn new() -> Self {
        Self {
            handles: Slab::new(),
        }
    }

    fn insert<T: Sized>(&mut self, handle: *mut T) -> u32 {
        (self.handles.insert(handle as usize) as u32) + Self::OFFSET
    }

    fn get<T: Sized>(&self, id: u32) -> Result<*mut T, CUerror> {
        if id == 0 {
            return Ok(ptr::null_mut());
        }
        let idx = id.checked_sub(Self::OFFSET).ok_or(CUerror::INVALID_VALUE)? as usize;
        self.handles
            .get(idx)
            .map(|&handle| handle as *mut T)
            .ok_or(CUerror::INVALID_VALUE)
    }

    fn remove<T: Sized>(&mut self, id: u32) -> Result<*mut T, CUerror> {
        if id == 0 {
            return Err(CUerror::INVALID_VALUE);
        }
        let idx = id.checked_sub(Self::OFFSET).ok_or(CUerror::INVALID_VALUE)? as usize;
        self.handles
            .try_remove(idx)
            .map(|handle| handle as *mut T)
            .ok_or(CUerror::INVALID_VALUE)
    }
}

struct ModuleLaunchData {
    dark_api: dark_api::zluda32::Zluda32Internal,
    modules: FxHashMap<CUmodule, Module>,
    functions: FxHashMap<CUfunction, Function>,
}

impl ModuleLaunchData {
    fn new() -> Result<Self, CUerror> {
        let mut zluda32_ptr = unsafe { mem::zeroed() };
        unsafe { cuGetExportTable(&mut zluda32_ptr, &dark_api::zluda32::Zluda32Internal::GUID) }?;
        let zluda32 = unsafe { dark_api::zluda32::Zluda32Internal::new(zluda32_ptr) };
        Ok(Self {
            dark_api: zluda32,
            modules: FxHashMap::default(),
            functions: FxHashMap::default(),
        })
    }

    unsafe fn new_module(
        &mut self,
        devmemory: &mut Allocator,
        module: CUmodule,
    ) -> Result<(), CUerror> {
        let mut count = 0;
        self.dark_api.get_module_globals(
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            &mut count,
            module,
        )?;
        let mut names = vec![mem::zeroed(); count as usize];
        let mut initializers = vec![mem::zeroed(); count as usize];
        let mut sizes = vec![mem::zeroed(); count as usize];
        let mut alignments = vec![mem::zeroed(); count as usize];
        self.dark_api.get_module_globals(
            names.as_mut_ptr(),
            initializers.as_mut_ptr(),
            sizes.as_mut_ptr(),
            alignments.as_mut_ptr(),
            &mut count,
            module,
        )?;
        let globals = (0..count as usize)
            .map(|i| {
                if alignments[i] > Allocator::ALLOCATION_UNIT {
                    return Err(CUerror::OUT_OF_MEMORY);
                }
                let initializer = std::slice::from_raw_parts(initializers[i], sizes[i] as usize);
                let allocation = devmemory.alloc_range(sizes[i])?;
                let devptr = devmemory.translate_range(allocation.clone())?;
                unsafe {
                    cuMemcpyHtoD_v2(
                        CUdeviceptr_v2(devptr),
                        initializer.as_ptr().cast(),
                        initializer.len(),
                    )
                }?;
                Ok::<_, CUerror>(Global {
                    name: unsafe { CStr::from_ptr(names[i].cast()) }
                        .to_string_lossy()
                        .into_owned(),
                    allocation,
                    size: sizes[i],
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let module_data = Module {
            globals: Rc::new(globals),
            texrefs: FxHashMap::default(),
        };
        self.modules.insert(module, module_data);
        Ok(())
    }
}

struct Module {
    globals: Rc<Vec<Global>>,
    texrefs: FxHashMap<Vec<u8>, u32>,
}

struct Global {
    name: String,
    allocation: Range<u32>,
    size: u32,
}

impl Global {
    fn address(&self) -> u32 {
        self.allocation.start * Allocator::ALLOCATION_UNIT
    }
}

struct Function {
    globals: Rc<Vec<Global>>,
}

struct Allocator {
    start: Option<*mut c_void>,
    allocator: range_alloc::RangeAllocator<u32>,
    allocation_ends: BTreeMap<u32, u32>,
}

impl Allocator {
    // This should be at least a multiple of texture pitch
    // otherwise fluidmark fails in myserious ways
    // I could query at runtime, but it's hard to imagine a GPU
    // with larger pitch
    const ALLOCATION_UNIT: u32 = 256;
    const ALLOCATOR_SIZE: u32 = 512 * 1024 * 1024; // 512 MiB

    fn new() -> Self {
        Self {
            start: None,
            allocator: range_alloc::RangeAllocator::new(
                // starting from 1 to avoid handing out null pointers
                1..Self::ALLOCATOR_SIZE / Self::ALLOCATION_UNIT,
            ),
            allocation_ends: BTreeMap::new(),
        }
    }

    fn get_or_allocate_device_ptr(&mut self) -> Result<*mut c_void, CUerror> {
        match self.start {
            Some(ptr) => Ok(ptr),
            None => {
                let mut dev_ptr = CUdeviceptr_v2(ptr::null_mut());
                unsafe { cuMemAlloc_v2(&mut dev_ptr, Self::ALLOCATOR_SIZE as usize) }?;
                self.start = Some(dev_ptr.0);
                Ok(dev_ptr.0)
            }
        }
    }

    fn get_device_ptr(&self) -> Result<*mut c_void, CUerror> {
        self.start.ok_or(CUerror::INVALID_VALUE)
    }

    fn alloc_range(&mut self, size: u32) -> Result<Range<u32>, CUerror> {
        self.get_or_allocate_device_ptr()?;
        let units = size.next_multiple_of(Self::ALLOCATION_UNIT) / Self::ALLOCATION_UNIT;
        self.allocator
            .allocate_range(units)
            .map_err(|_| CUerror::OUT_OF_MEMORY)
    }

    fn alloc(&mut self, size: u32) -> Result<u32, CUerror> {
        let offset = self.alloc_range(size)?;
        self.allocation_ends.insert(offset.start, offset.end);
        Ok(offset.start * Self::ALLOCATION_UNIT)
    }

    fn free(&mut self, start: u32) -> Result<(), CUerror> {
        let start_in_units = start / Allocator::ALLOCATION_UNIT;
        let end = self
            .allocation_ends
            .remove(&start_in_units)
            .ok_or(CUerror::INVALID_VALUE)?;
        self.allocator.free_range(start_in_units..end);
        Ok(())
    }

    fn translate(&self, offset: u32) -> Result<*mut c_void, CUerror> {
        if offset == 0 {
            return Ok(ptr::null_mut());
        }
        let base_ptr = self.get_device_ptr()?;
        Ok(base_ptr.wrapping_byte_add(offset as usize))
    }

    fn translate_range(&self, range: Range<u32>) -> Result<*mut c_void, CUerror> {
        let base_ptr = self.get_device_ptr()?;
        Ok(base_ptr.wrapping_byte_add(range.start as usize * Self::ALLOCATION_UNIT as usize))
    }

    fn get_range(&self, mut offset: u32) -> Option<Range<u32>> {
        offset /= Self::ALLOCATION_UNIT;
        // Find last pair where `start <= ptr`
        let (start, alloc) = self.allocation_ends.range(..=offset).rev().next()?;
        let range = *start..*alloc;
        // Check if allocation contains the pointer
        if range.contains(&offset) {
            Some(range)
        } else {
            None
        }
    }
}

impl State {
    fn new() -> Result<Self, CUerror> {
        unsafe { cuInit(0) }?;
        let mut ctx = unsafe { mem::zeroed() };
        let device = 0;
        unsafe { cuDevicePrimaryCtxRetain(&mut ctx, device) }?;
        unsafe { cuCtxSetCurrent(ctx) }?;
        Ok(Self {
            ctx,
            device,
            handles: HandlePool::new(),
            devmemory: Allocator::new(),
            modules: ModuleLaunchData::new()?,
        })
    }
}

fn main() -> std::io::Result<()> {
    let args = std::env::args();
    let mut args = args.skip(1);
    let remote_event = args.next().unwrap();
    let remote_shared_memory = args.next().unwrap();
    let local_event = args.next().unwrap();
    let local_shared_memory = args.next().unwrap();
    let mut remote =
        unsafe { Endpoint::open(remote_event, remote_shared_memory) }.map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to open remote endpoint")
        })?;
    let mut local = unsafe { Endpoint::open(local_event, local_shared_memory) }.map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::Other, "Failed to open local endpoint")
    })?;
    let mut state = State::new().map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to initialize CUDA context",
        )
    })?;
    let mut arena = stumpalo::Arena::new();
    unsafe { WaitForSingleObject(*local.event, INFINITE) };
    loop {
        let opcode = local.shared_memory.read_header();
        if opcode == u32::MAX {
            todo!()
        }
        match Opcode::from_repr(opcode) {
            Some(Opcode::cuInit) => {
                handle_cuda_function2::<cuInitIn, cuInitOut>(&mut local, &mut remote, cu_init);
            }
            Some(Opcode::cuDeviceGetCount) => {
                handle_cuda_function2::<cuDeviceGetCountIn, cuDeviceGetCountOut>(
                    &mut local,
                    &mut remote,
                    cu_device_get_count,
                );
            }
            Some(Opcode::cuDeviceGetAttribute) => {
                handle_cuda_function2::<cuDeviceGetAttributeIn, cuDeviceGetAttributeOut>(
                    &mut local,
                    &mut remote,
                    |input| cu_device_get_attribute(&mut state, input),
                );
            }
            Some(Opcode::cuDeviceGet) => {
                handle_cuda_function2::<cuDeviceGetIn, cuDeviceGetOut>(
                    &mut local,
                    &mut remote,
                    |input| cu_device_get(&mut state, input),
                );
            }
            Some(Opcode::cuDriverGetVersion) => {
                handle_cuda_function2::<cuDriverGetVersionIn, cuDriverGetVersionOut>(
                    &mut local,
                    &mut remote,
                    cu_driver_get_version,
                );
            }
            Some(Opcode::cuDeviceGetName) => {
                handle_cuda_function_framed_out2::<cuDeviceGetNameIn, cuDeviceGetNameOut>(
                    &mut local,
                    &mut remote,
                    &mut arena,
                    |input| cu_device_get_name(&mut state, input),
                );
            }
            Some(Opcode::cuDeviceTotalMem_v2) => {
                handle_cuda_function2::<cuDeviceTotalMem_v2In, cuDeviceTotalMem_v2Out>(
                    &mut local,
                    &mut remote,
                    |input| cu_device_total_mem_v2(&mut state, input),
                );
            }
            Some(Opcode::cuCtxGetApiVersion) => {
                handle_cuda_function2::<cuCtxGetApiVersionIn, cuCtxGetApiVersionOut>(
                    &mut local,
                    &mut remote,
                    |input| cu_ctx_get_api_version(&mut state, input),
                );
            }
            Some(Opcode::cuModuleLoadData) => {
                handle_cuda_function_framed_in2::<cuModuleLoadDataIn, cuModuleLoadDataOut>(
                    &mut local,
                    &mut remote,
                    |input| cu_module_load_data(&mut state, input),
                );
            }
            Some(Opcode::cuModuleGetFunction) => {
                handle_cuda_function_framed_in2::<cuModuleGetFunctionIn, cuModuleGetFunctionOut>(
                    &mut local,
                    &mut remote,
                    |input| cu_module_get_function(&mut state, input),
                );
            }
            Some(Opcode::cuModuleGetGlobal_v2) => {
                handle_cuda_function_framed_in2::<cuModuleGetGlobal_v2In, cuModuleGetGlobal_v2Out>(
                    &mut local,
                    &mut remote,
                    |input| cu_module_get_global_v2(&mut state, input),
                );
            }
            Some(Opcode::cuMemAlloc_v2) => {
                handle_cuda_function2::<cuMemAlloc_v2In, cuMemAlloc_v2Out>(
                    &mut local,
                    &mut remote,
                    |input| cu_mem_alloc_v2(&mut state, input),
                );
            }
            Some(Opcode::cuMemcpyHtoDAsync_v2) => {
                handle_cuda_function_framed_in2::<cuMemcpyHtoDAsync_v2In, cuMemcpyHtoDAsync_v2Out>(
                    &mut local,
                    &mut remote,
                    |input| cu_memcpy_hto_d_async_v2(&mut state, input),
                );
            }
            Some(Opcode::cuModuleGetTexRef) => {
                handle_cuda_function_framed_in2::<cuModuleGetTexRefIn, cuModuleGetTexRefOut>(
                    &mut local,
                    &mut remote,
                    |input| cu_module_get_tex_ref(&mut state, input),
                );
            }
            Some(Opcode::zludaGetFunctionArgs) => {
                handle_cuda_function_framed_out2::<zludaGetFunctionArgsIn, zludaGetFunctionArgsOut>(
                    &mut local,
                    &mut remote,
                    &mut arena,
                    |input| zluda_get_function_args(&mut state, input),
                );
            }
            Some(Opcode::cuLaunchKernel) => {
                handle_cuda_function_framed_in2::<cuLaunchKernelIn, cuLaunchKernelOut>(
                    &mut local,
                    &mut remote,
                    |input| cu_launch_kernel(&mut state, input),
                );
            }
            Some(Opcode::cuCtxSynchronize) => {
                handle_cuda_function2::<cuCtxSynchronizeIn, cuCtxSynchronizeOut>(
                    &mut local,
                    &mut remote,
                    cu_ctx_synchronize,
                );
            }
            Some(Opcode::cuMemcpyDtoHAsync_v2) => {
                handle_cuda_function_framed_out2::<cuMemcpyDtoHAsync_v2In, cuMemcpyDtoHAsync_v2Out>(
                    &mut local,
                    &mut remote,
                    &mut arena,
                    |input| cu_memcpy_dtoh_async_v2(&mut state, input),
                );
            }
            Some(Opcode::cuMemGetAddressRange_v2) => {
                handle_cuda_function2::<cuMemGetAddressRange_v2In, cuMemGetAddressRange_v2Out>(
                    &mut local,
                    &mut remote,
                    |input| cu_mem_get_address_range_v2(&mut state, input),
                );
            }
            Some(Opcode::cuTexRefSetAddress_v2) => {
                handle_cuda_function2::<cuTexRefSetAddress_v2In, cuTexRefSetAddress_v2Out>(
                    &mut local,
                    &mut remote,
                    |input| cu_tex_ref_set_address_v2(&mut state, input),
                );
            }
            Some(Opcode::cuTexRefSetFlags) => {
                handle_cuda_function2::<cuTexRefSetFlagsIn, cuTexRefSetFlagsOut>(
                    &mut local,
                    &mut remote,
                    |input| cu_tex_ref_set_flags(&mut state, input),
                );
            }
            Some(Opcode::cuTexRefSetFormat) => {
                handle_cuda_function2::<cuTexRefSetFormatIn, cuTexRefSetFormatOut>(
                    &mut local,
                    &mut remote,
                    |input| cu_tex_ref_set_format(&mut state, input),
                );
            }
            Some(Opcode::cuMemFree_v2) => {
                handle_cuda_function2::<cuMemFree_v2In, cuMemFree_v2Out>(
                    &mut local,
                    &mut remote,
                    |input| cu_mem_free_v2(&mut state, input),
                );
            }
            Some(Opcode::cuDeviceComputeCapability) => {
                handle_cuda_function2::<cuDeviceComputeCapabilityIn, cuDeviceComputeCapabilityOut>(
                    &mut local,
                    &mut remote,
                    cu_device_compute_capability,
                );
            }
            Some(Opcode::cuDeviceGetProperties) => {
                handle_cuda_function2::<cuDeviceGetPropertiesIn, cuDeviceGetPropertiesOut>(
                    &mut local,
                    &mut remote,
                    cu_device_get_properties,
                );
            }
            Some(Opcode::cuStreamCreate) => {
                handle_cuda_function2::<cuStreamCreateIn, cuStreamCreateOut>(
                    &mut local,
                    &mut remote,
                    |input| cu_stream_create(&mut state, input),
                );
            }
            Some(Opcode::cuStreamDestroy_v2) => {
                handle_cuda_function2::<cuStreamDestroy_v2In, cuStreamDestroy_v2Out>(
                    &mut local,
                    &mut remote,
                    |input| cu_stream_destroy_v2(&mut state, input),
                );
            }
            Some(Opcode::cuEventCreate) => {
                handle_cuda_function2::<cuEventCreateIn, cuEventCreateOut>(
                    &mut local,
                    &mut remote,
                    |input| cu_event_create(&mut state, input),
                );
            }
            Some(Opcode::cuEventDestroy_v2) => {
                handle_cuda_function2::<cuEventDestroy_v2In, cuEventDestroy_v2Out>(
                    &mut local,
                    &mut remote,
                    |input| cu_event_destroy_v2(&mut state, input),
                );
            }
            Some(Opcode::cuMemsetD8_v2) => {
                handle_cuda_function2::<cuMemsetD8_v2In, cuMemsetD8_v2Out>(
                    &mut local,
                    &mut remote,
                    |input| cu_memset_d8_v2(&mut state, input),
                );
            }
            Some(Opcode::cuMemcpyDtoDAsync_v2) => {
                handle_cuda_function2::<cuMemcpyDtoDAsync_v2In, cuMemcpyDtoDAsync_v2Out>(
                    &mut local,
                    &mut remote,
                    |input| cu_memcpy_dto_d_async_v2(&mut state, input),
                );
            }
            Some(Opcode::cuMemcpyDtoD_v2) => {
                handle_cuda_function2::<cuMemcpyDtoD_v2In, cuMemcpyDtoD_v2Out>(
                    &mut local,
                    &mut remote,
                    |input| cu_memcpy_dto_d_v2(&mut state, input),
                );
            }
            Some(Opcode::cuEventQuery) => {
                handle_cuda_function2::<cuEventQueryIn, cuEventQueryOut>(
                    &mut local,
                    &mut remote,
                    |input| cu_event_query(&mut state, input),
                );
            }
            Some(Opcode::cuEventRecord) => {
                handle_cuda_function2::<cuEventRecordIn, cuEventRecordOut>(
                    &mut local,
                    &mut remote,
                    |input| cu_event_record(&mut state, input),
                );
            }
            _ => {
                let return_code = CUerror::NOT_SUPPORTED.0.get();
                remote.shared_memory.write_header(return_code);
                unsafe { SignalObjectAndWait(*remote.event, *local.event, INFINITE, false) };
            }
        }
    }
}

fn cu_event_query(
    state: &mut State,
    input: &ArchivedcuEventQueryIn,
) -> Result<cuEventQueryOut, CUerror> {
    let event = input.hEvent.to_native();
    let cu_event = state.handles.get(event)?;
    unsafe { cuEventQuery(cu_event) }?;
    Ok(cuEventQueryOut {})
}

fn cu_event_record(
    state: &mut State,
    input: &ArchivedcuEventRecordIn,
) -> Result<cuEventRecordOut, CUerror> {
    let event = input.hEvent.to_native();
    let cu_event = state.handles.get(event)?;
    let stream = input.hStream.to_native();
    let cu_stream = CUstream(state.handles.get(stream)?);
    unsafe { cuEventRecord(cu_event, cu_stream) }?;
    Ok(cuEventRecordOut {})
}

fn cu_memcpy_dto_d_v2(
    state: &mut State,
    input: &ArchivedcuMemcpyDtoD_v2In,
) -> Result<cuMemcpyDtoD_v2Out, CUerror> {
    let dst_device = CUdeviceptr_v2(state.devmemory.translate(input.dstDevice.to_native())?);
    let src_device = CUdeviceptr_v2(state.devmemory.translate(input.srcDevice.to_native())?);
    let byte_count = input.ByteCount.to_native();
    unsafe { cuMemcpyDtoD_v2(dst_device, src_device, byte_count as usize) }?;
    Ok(cuMemcpyDtoD_v2Out {})
}

fn cu_memcpy_dto_d_async_v2(
    state: &mut State,
    input: &ArchivedcuMemcpyDtoDAsync_v2In,
) -> Result<cuMemcpyDtoDAsync_v2Out, CUerror> {
    let dst_device = CUdeviceptr_v2(state.devmemory.translate(input.dstDevice.to_native())?);
    let src_device = CUdeviceptr_v2(state.devmemory.translate(input.srcDevice.to_native())?);
    let byte_count = input.ByteCount.to_native();
    let cu_stream = CUstream(state.handles.get(input.hStream.to_native())?);
    unsafe { cuMemcpyDtoDAsync_v2(dst_device, src_device, byte_count as usize, cu_stream) }?;
    Ok(cuMemcpyDtoDAsync_v2Out {})
}

fn cu_memset_d8_v2(
    state: &mut State,
    input: &ArchivedcuMemsetD8_v2In,
) -> Result<cuMemsetD8_v2Out, CUerror> {
    let dptr = CUdeviceptr_v2(state.devmemory.translate(input.dstDevice.to_native())?);
    let uc = input.uc;
    let n = input.N.to_native();
    unsafe { cuMemsetD8_v2(dptr, uc, n as usize) }?;
    Ok(cuMemsetD8_v2Out {})
}

fn cu_event_create(
    state: &mut State,
    input: &ArchivedcuEventCreateIn,
) -> Result<cuEventCreateOut, CUerror> {
    let mut event = ptr::null_mut();
    unsafe { cuEventCreate(&mut event, input.Flags.to_native()) }?;
    let handle = state.handles.insert(event);
    Ok(cuEventCreateOut {
        phEvent: u32_le::from_native(handle),
    })
}

fn cu_event_destroy_v2(
    state: &mut State,
    input: &ArchivedcuEventDestroy_v2In,
) -> Result<cuEventDestroy_v2Out, CUerror> {
    let event = input.hEvent.to_native();
    let cu_event = state.handles.remove(event)?;
    unsafe { cuEventDestroy_v2(cu_event) }?;
    Ok(cuEventDestroy_v2Out {})
}

fn cu_stream_destroy_v2(
    state: &mut State,
    input: &ArchivedcuStreamDestroy_v2In,
) -> Result<cuStreamDestroy_v2Out, CUerror> {
    let stream = input.hStream.to_native();
    let cu_stream = CUstream(state.handles.remove(stream)?);
    unsafe { cuStreamDestroy_v2(cu_stream) }?;
    Ok(cuStreamDestroy_v2Out {})
}

fn cu_stream_create(
    state: &mut State,
    input: &ArchivedcuStreamCreateIn,
) -> Result<cuStreamCreateOut, CUerror> {
    let mut stream = CUstream(ptr::null_mut());
    unsafe { cuStreamCreate(&mut stream, input.Flags.to_native()) }?;
    let handle = state.handles.insert(stream.0);
    Ok(cuStreamCreateOut {
        phStream: u32_le::from_native(handle),
    })
}

fn cu_device_get_properties(
    input: &ArchivedcuDeviceGetPropertiesIn,
) -> Result<cuDeviceGetPropertiesOut, CUerror> {
    let mut props = unsafe { mem::zeroed() };
    unsafe { cuDeviceGetProperties(&mut props, input.dev.to_native()) }?;
    Ok(cuDeviceGetPropertiesOut { prop: props.into() })
}

fn cu_device_compute_capability(
    input: &ArchivedcuDeviceComputeCapabilityIn,
) -> Result<cuDeviceComputeCapabilityOut, CUerror> {
    let mut major = 0;
    let mut minor = 0;
    unsafe { cuDeviceComputeCapability(&mut major, &mut minor, input.dev.to_native()) }?;
    Ok(cuDeviceComputeCapabilityOut { major, minor })
}

fn cu_mem_free_v2(
    state: &mut State,
    input: &ArchivedcuMemFree_v2In,
) -> Result<cuMemFree_v2Out, CUerror> {
    let dptr = input.dptr.to_native();
    state.devmemory.free(dptr)?;
    Ok(cuMemFree_v2Out {})
}

fn cu_tex_ref_set_format(
    state: &mut State,
    input: &ArchivedcuTexRefSetFormatIn,
) -> Result<cuTexRefSetFormatOut, CUerror> {
    let tex_ref = state.handles.get(input.hTexRef.to_native())?;
    unsafe {
        cuTexRefSetFormat(
            tex_ref,
            CUarray_format(input.fmt.to_native()),
            input.NumPackedComponents.to_native(),
        )
    }?;
    Ok(cuTexRefSetFormatOut {})
}

fn cu_tex_ref_set_flags(
    state: &mut State,
    input: &ArchivedcuTexRefSetFlagsIn,
) -> Result<cuTexRefSetFlagsOut, CUerror> {
    let tex_ref = state.handles.get(input.hTexRef.to_native())?;
    unsafe { cuTexRefSetFlags(tex_ref, input.Flags.to_native()) }?;
    Ok(cuTexRefSetFlagsOut {})
}

fn cu_tex_ref_set_address_v2(
    state: &mut State,
    input: &ArchivedcuTexRefSetAddress_v2In,
) -> Result<cuTexRefSetAddress_v2Out, CUerror> {
    let mut byte_offset = 0;
    let dptr = input.dptr.to_native();
    let dptr = CUdeviceptr_v2(state.devmemory.translate(dptr)?);
    unsafe {
        cuTexRefSetAddress_v2(
            &mut byte_offset,
            state.handles.get(input.hTexRef.to_native())?,
            dptr,
            input.bytes.to_native() as usize,
        )
    }?;
    Ok(cuTexRefSetAddress_v2Out {
        ByteOffset: (byte_offset as u32).into(),
    })
}

fn cu_mem_get_address_range_v2(
    state: &mut State,
    input: &ArchivedcuMemGetAddressRange_v2In,
) -> Result<cuMemGetAddressRange_v2Out, CUerror> {
    let range = state
        .devmemory
        .get_range(input.dptr.to_native())
        .ok_or(CUerror::INVALID_VALUE)?;
    Ok(cuMemGetAddressRange_v2Out {
        pbase: (range.start * Allocator::ALLOCATION_UNIT).into(),
        psize: ((range.end - range.start) * Allocator::ALLOCATION_UNIT).into(),
    })
}

fn cu_memcpy_dtoh_async_v2(
    state: &mut State,
    input: &ArchivedcuMemcpyDtoHAsync_v2In,
) -> Result<cuMemcpyDtoHAsync_v2Out, CUerror> {
    let devptr = state.devmemory.translate(input.src_device.to_native())?;
    let stream = input.stream.to_native();
    let stream = CUstream(state.handles.get(stream)?);
    let mut dst_host = vec![0u8; input.byte_count.to_native() as usize];
    unsafe {
        cuMemcpyDtoHAsync_v2(
            dst_host.as_mut_ptr().cast(),
            CUdeviceptr_v2(devptr),
            input.byte_count.to_native() as usize,
            stream,
        )
    }?;
    unsafe { cuStreamSynchronize(stream) }?;
    Ok(cuMemcpyDtoHAsync_v2Out { dst_host })
}

fn cu_ctx_synchronize(_input: &ArchivedcuCtxSynchronizeIn) -> Result<cuCtxSynchronizeOut, CUerror> {
    unsafe { cuCtxSynchronize() }?;
    Ok(cuCtxSynchronizeOut {})
}

fn cu_launch_kernel(
    state: &mut State,
    input: &ArchivedcuLaunchKernelIn,
) -> Result<cuLaunchKernelOut, CUerror> {
    let mut device = 0;
    unsafe { cuCtxGetDevice(&mut device) }?;
    let mut params = input
        .kernel_params
        .iter()
        .map(|p| p.as_ptr().cast_mut().cast::<c_void>())
        .collect::<Vec<_>>();
    let mut base64_ptr = state.devmemory.get_device_ptr()?;
    params.push(ptr::from_mut(&mut base64_ptr).cast());
    let function = CUfunction(state.handles.get(input.f.to_native())?);
    let function_info = state
        .modules
        .functions
        .get(&function)
        .ok_or(CUerror::INVALID_VALUE)?;
    let globals = function_info
        .globals
        .iter()
        .map(|global| global.allocation.start * Allocator::ALLOCATION_UNIT)
        .collect::<Vec<_>>();
    for g in globals.iter() {
        params.push(ptr::from_ref(g).cast_mut().cast());
    }
    unsafe {
        cuLaunchKernel(
            function,
            input.grid_dim_x.to_native(),
            input.grid_dim_y.to_native(),
            input.grid_dim_z.to_native(),
            input.block_dim_x.to_native(),
            input.block_dim_y.to_native(),
            input.block_dim_z.to_native(),
            input.shared_mem_bytes.to_native(),
            CUstream(state.handles.get(input.stream.to_native())?),
            params.as_mut_ptr(),
            std::ptr::null_mut(),
        )
    }?;
    Ok(cuLaunchKernelOut {})
}

fn zluda_get_function_args(
    state: &mut State,
    input: &ArchivedzludaGetFunctionArgsIn,
) -> Result<zludaGetFunctionArgsOut, CUerror> {
    let hfunc = state.handles.get(input.f.to_native())?;
    let mut count = 0;
    unsafe {
        state.modules.dark_api.get_function_info(
            std::ptr::null_mut(),
            &mut count,
            CUfunction(hfunc),
        )
    }?;
    let mut arg_sizes = vec![FunctionArgInfo { size: 0, align: 0 }; count as usize];
    unsafe {
        state.modules.dark_api.get_function_info(
            arg_sizes.as_mut_ptr(),
            &mut count,
            CUfunction(hfunc),
        )
    }?;
    Ok(zludaGetFunctionArgsOut { args: arg_sizes })
}

fn cu_module_get_tex_ref(
    state: &mut State,
    input: &ArchivedcuModuleGetTexRefIn,
) -> Result<cuModuleGetTexRefOut, CUerror> {
    let hmod = state.handles.get::<CUmod_st>(input.hmod.to_native())?;
    let module = state
        .modules
        .modules
        .get_mut(&CUmodule(hmod))
        .ok_or(CUerror::INVALID_VALUE)?;
    let texref = match module.texrefs.entry(input.name.to_vec()) {
        std::collections::hash_map::Entry::Occupied(entry) => *entry.get(),
        std::collections::hash_map::Entry::Vacant(entry) => {
            let mut texref = unsafe { mem::zeroed() };
            unsafe { cuModuleGetTexRef(&mut texref, CUmodule(hmod), input.name.as_ptr().cast()) }?;
            let handle = state.handles.insert(texref);
            *entry.insert(handle)
        }
    };
    Ok(cuModuleGetTexRefOut {
        texref: u32_le::from_native(texref),
    })
}

fn cu_memcpy_hto_d_async_v2(
    state: &mut State,
    input: &ArchivedcuMemcpyHtoDAsync_v2In,
) -> Result<cuMemcpyHtoDAsync_v2Out, CUerror> {
    let devptr = state.devmemory.translate(input.dst_device.to_native())?;
    let stream = CUstream(state.handles.get(input.stream.to_native())?);
    unsafe {
        cuMemcpyHtoDAsync_v2(
            CUdeviceptr_v2(devptr),
            input.src_host.as_ptr().cast(),
            input.src_host.len(),
            stream,
        )
    }?;
    Ok(cuMemcpyHtoDAsync_v2Out {})
}

fn cu_mem_alloc_v2(
    state: &mut State,
    input: &ArchivedcuMemAlloc_v2In,
) -> Result<cuMemAlloc_v2Out, CUerror> {
    let fake_ptr = state.devmemory.alloc(input.bytesize.to_native())?;
    Ok(cuMemAlloc_v2Out {
        dptr: u32_le::from_native(fake_ptr),
    })
}

fn cu_module_get_global_v2(
    state: &mut State,
    input: &ArchivedcuModuleGetGlobal_v2In,
) -> Result<cuModuleGetGlobal_v2Out, CUerror> {
    let hmod = state.handles.get::<CUmod_st>(input.hmod.to_native())?;
    let module = state
        .modules
        .modules
        .get(&CUmodule(hmod))
        .ok_or(CUerror::INVALID_VALUE)?;
    let global = module
        .globals
        .iter()
        .find(|global| global.name.as_bytes() == input.name.as_slice())
        .ok_or(CUerror::NOT_FOUND)?;
    Ok(cuModuleGetGlobal_v2Out {
        dptr: u32_le::from_native(global.address()),
        bytes: u64_le::from_native(global.size as u64),
    })
}

fn cu_module_get_function(
    state: &mut State,
    input: &ArchivedcuModuleGetFunctionIn,
) -> Result<cuModuleGetFunctionOut, CUerror> {
    let mut hfunc = unsafe { mem::zeroed() };
    let hmod = state.handles.get(input.hmod.to_native())?;
    unsafe { cuModuleGetFunction(&mut hfunc, CUmodule(hmod), input.name.as_ptr().cast()) }?;
    state
        .modules
        .functions
        .entry(hfunc)
        .or_insert_with(|| Function {
            globals: Rc::clone(&state.modules.modules[&CUmodule(hmod)].globals),
        });
    Ok(cuModuleGetFunctionOut {
        hfunc: u32_le::from_native(state.handles.insert(hfunc.0)),
    })
}

fn cu_module_load_data(
    state: &mut State,
    input: &ArchivedcuModuleLoadDataIn,
) -> Result<cuModuleLoadDataOut, CUerror> {
    let mut module = unsafe { mem::zeroed() };
    unsafe { cuModuleLoadData(&mut module, input.image.as_ptr().cast()) }?;
    unsafe { state.modules.new_module(&mut state.devmemory, module) }?;
    Ok(cuModuleLoadDataOut {
        module: u32_le::from_native(state.handles.insert(module.0)),
    })
}

fn cu_init(input: &ArchivedcuInitIn) -> Result<cuInitOut, CUerror> {
    unsafe { cuInit(input.Flags.to_native()) }?;
    Ok(cuInitOut {})
}

fn cu_device_get_count(
    _input: &ArchivedcuDeviceGetCountIn,
) -> Result<cuDeviceGetCountOut, CUerror> {
    Ok(cuDeviceGetCountOut { count: 1 })
}

fn cu_device_get_attribute(
    state: &mut State,
    input: &ArchivedcuDeviceGetAttributeIn,
) -> Result<cuDeviceGetAttributeOut, CUerror> {
    if input.dev.to_native() != state.device {
        return Err(CUerror::INVALID_DEVICE);
    }
    let mut pi = 0;
    unsafe {
        cuDeviceGetAttribute(
            &mut pi,
            CUdevice_attribute_enum(input.attrib.to_native()),
            state.device,
        )
    }?;
    Ok(cuDeviceGetAttributeOut { pi })
}

fn cu_device_get(
    state: &mut State,
    _input: &ArchivedcuDeviceGetIn,
) -> Result<cuDeviceGetOut, CUerror> {
    Ok(cuDeviceGetOut {
        device: state.device.into(),
    })
}

fn cu_device_get_name(
    state: &mut State,
    input: &ArchivedcuDeviceGetNameIn,
) -> Result<cuDeviceGetNameOut, CUerror> {
    if input.dev.to_native() != state.device {
        return Err(CUerror::INVALID_DEVICE);
    }
    let mut name = vec![0u8; input.len.to_native() as usize];
    unsafe {
        cuDeviceGetName(
            name.as_mut_ptr().cast(),
            input.len.to_native(),
            state.device,
        )
    }?;
    if let Some(pos) = name.iter().copied().position(|c| c == 0) {
        name.truncate(pos);
    }
    Ok(cuDeviceGetNameOut { name })
}

fn cu_device_total_mem_v2(
    state: &mut State,
    input: &ArchivedcuDeviceTotalMem_v2In,
) -> Result<cuDeviceTotalMem_v2Out, CUerror> {
    if input.dev.to_native() != state.device {
        return Err(CUerror::INVALID_DEVICE);
    }
    let mut bytes = 0usize;
    unsafe { cuDeviceTotalMem_v2(&mut bytes, state.device) }?;
    Ok(cuDeviceTotalMem_v2Out {
        bytes: u64_le::from_native(bytes as u64),
    })
}

fn cu_driver_get_version(
    _input: &ArchivedcuDriverGetVersionIn,
) -> Result<cuDriverGetVersionOut, CUerror> {
    let mut driver_version = 0;
    unsafe { cuDriverGetVersion(&mut driver_version) }?;
    Ok(cuDriverGetVersionOut {
        driverVersion: driver_version,
    })
}

fn cu_ctx_get_api_version(
    state: &mut State,
    _input: &ArchivedcuCtxGetApiVersionIn,
) -> Result<cuCtxGetApiVersionOut, CUerror> {
    let mut version = 0;
    unsafe { cuCtxGetApiVersion(state.ctx, &mut version) }?;
    Ok(cuCtxGetApiVersionOut { version })
}

fn handle_cuda_function2<In: rkyv::Archive + Portable, Out: Portable>(
    local: &mut Endpoint,
    remote: &mut Endpoint,
    handler: impl FnOnce(&In::Archived) -> Result<Out, CUerror>,
) {
    let input = local.shared_memory.read_body();
    match handler(&input) {
        Ok(output) => {
            remote.shared_memory.write_header(0);
            remote.shared_memory.write_body(&output);
        }
        Err(e) => remote.shared_memory.write_header(e.0.get()),
    }
    unsafe { SignalObjectAndWait(*remote.event, *local.event, INFINITE, false) };
}

fn handle_cuda_function_framed_out2_impl<In: Archive + Portable, Out>(
    local: &mut Endpoint,
    remote: &mut Endpoint,
    arena: &mut stumpalo::Arena,
    handler: impl FnOnce(&In::Archived) -> Result<Out, CUerror>,
) -> Result<(), CUerror>
where
    Out: for<'a, 'b> Serialize<Serializer<'a, 'b>>,
{
    let input = local.shared_memory.read_body();
    let output = handler(&input)?;
    remote.shared_memory.write_header(0);
    let old_shmem = remote.shared_memory.serialize_body(arena, &output)?;
    if let Some(mut old_shmem) = old_shmem {
        old_shmem.write_header(u32::MAX);
        old_shmem.write_size(remote.shared_memory.name.as_bytes().len() as u32);
        old_shmem.write_buffer(remote.shared_memory.name.as_bytes());
    }
    Ok(())
}

fn handle_cuda_function_framed_out2<In: Archive + Portable, Out>(
    local: &mut Endpoint,
    remote: &mut Endpoint,
    arena: &mut stumpalo::Arena,
    handler: impl FnOnce(&In::Archived) -> Result<Out, CUerror>,
) where
    Out: for<'a, 'b> Serialize<Serializer<'a, 'b>>,
{
    if let Err(err) =
        handle_cuda_function_framed_out2_impl::<In, Out>(local, remote, arena, handler)
    {
        remote.shared_memory.write_header(err.0.get());
    }
    unsafe { SignalObjectAndWait(*remote.event, *local.event, INFINITE, false) };
}

fn handle_cuda_function_framed_in2_impl<In: Archive, Out: Portable>(
    local: &mut Endpoint,
    remote: &mut Endpoint,
    handler: impl FnOnce(&In::Archived) -> Result<Out, CUerror>,
) -> Result<(), CUerror>
where
    Out: for<'a, 'b> Serialize<Serializer<'a, 'b>>,
{
    let input = local.shared_memory.deserialize_body2::<In>();
    match handler(input) {
        Ok(output) => {
            remote.shared_memory.write_header(0);
            remote.shared_memory.write_body(&output);
        }
        Err(e) => remote.shared_memory.write_header(e.0.get()),
    }
    Ok(())
}

fn handle_cuda_function_framed_in2<In: Archive, Out: Portable>(
    local: &mut Endpoint,
    remote: &mut Endpoint,
    handler: impl FnOnce(&In::Archived) -> Result<Out, CUerror>,
) where
    Out: for<'a, 'b> Serialize<Serializer<'a, 'b>>,
{
    if let Err(err) = handle_cuda_function_framed_in2_impl::<In, Out>(local, remote, handler) {
        remote.shared_memory.write_header(err.0.get());
    }
    unsafe { SignalObjectAndWait(*remote.event, *local.event, INFINITE, false) };
}

async fn handle_cuda_function<In: rkyv::Archive + Portable, Out: Portable>(
    client: &mut NamedPipeClient,
    mut buffer: AlignedVecBuffer,
    handler: impl FnOnce(&In::Archived) -> Result<Out, CUerror>,
) -> std::io::Result<AlignedVecBuffer>
where
    Out: for<'a, 'b> Serialize<
        HighSerializer<&'a mut AlignedVec, ArenaHandle<'b>, rkyv::rancor::Failure>,
    >,
{
    buffer = read_all::<In::Archived>(buffer, client).await?;
    let input = unsafe { rkyv::access_unchecked::<In::Archived>(buffer.as_init()) };
    match handler(input) {
        Ok(output) => {
            buffer.clear();
            client.write_u32_le(0).await?;
            rkyv::api::high::to_bytes_in::<_, rkyv::rancor::Failure>(&output, &mut buffer.0)
                .map_err(|_| {
                    std::io::Error::new(std::io::ErrorKind::Other, "Failed to serialize response")
                })?;
            let ((), new_buffer) = buf_try!(@try client.write_all(buffer).await);
            Ok(new_buffer)
        }
        Err(e) => {
            client.write_u32_le(e.0.get()).await?;
            Ok(buffer)
        }
    }
}

async fn handle_cuda_function_framed_in<In: Archive, Out: Portable>(
    client: &mut NamedPipeClient,
    mut buffer: AlignedVecBuffer,
    handler: impl FnOnce(&In::Archived) -> Result<Out, CUerror>,
) -> std::io::Result<AlignedVecBuffer>
where
    Out: for<'a, 'b> Serialize<
        HighSerializer<&'a mut AlignedVec, ArenaHandle<'b>, rkyv::rancor::Failure>,
    >,
{
    let length_prefix = client.read_u32_le().await? as usize;
    buffer = read_sized(buffer, client, length_prefix).await?;
    let input = unsafe { rkyv::access_unchecked::<In::Archived>(buffer.as_init()) };
    match handler(input) {
        Ok(output) => {
            buffer.clear();
            client.write_u32_le(0).await?;
            rkyv::api::high::to_bytes_in::<_, rkyv::rancor::Failure>(&output, &mut buffer.0)
                .map_err(|_| {
                    std::io::Error::new(std::io::ErrorKind::Other, "Failed to serialize response")
                })?;
            let ((), new_buffer) = buf_try!(@try client.write_all(buffer).await);
            Ok(new_buffer)
        }
        Err(e) => {
            client.write_u32_le(e.0.get()).await?;
            Ok(buffer)
        }
    }
}

async fn handle_cuda_function_framed_out<In: Archive + Portable, Out>(
    client: &mut NamedPipeClient,
    mut buffer: AlignedVecBuffer,
    handler: impl FnOnce(&In::Archived) -> Result<Out, CUerror>,
) -> std::io::Result<AlignedVecBuffer>
where
    Out: for<'a, 'b> Serialize<
        HighSerializer<&'a mut AlignedVec, ArenaHandle<'b>, rkyv::rancor::Failure>,
    >,
{
    buffer = read_all::<In::Archived>(buffer, client).await?;
    let input = unsafe { rkyv::access_unchecked::<In::Archived>(buffer.as_init()) };
    match handler(input) {
        Ok(output) => {
            buffer.clear();
            rkyv::api::high::to_bytes_in::<_, rkyv::rancor::Failure>(&output, &mut buffer.0)
                .map_err(|_| {
                    std::io::Error::new(std::io::ErrorKind::Other, "Failed to serialize response")
                })?;
            let code_and_len = unsafe {
                std::mem::transmute::<(u32, u32), [u8; 8]>((0u32, buffer.0.len() as u32))
            };
            client.write_all(code_and_len).await.0?;
            let ((), new_buffer) = buf_try!(@try client.write_all(buffer).await);
            Ok(new_buffer)
        }
        Err(e) => {
            client.write_u32_le(e.0.get()).await?;
            Ok(buffer)
        }
    }
}

async fn read_all<T>(
    buffer: AlignedVecBuffer,
    client: &mut NamedPipeClient,
) -> std::io::Result<AlignedVecBuffer> {
    read_sized(buffer, client, mem::size_of::<T>()).await
}

async fn read_sized(
    mut buffer: AlignedVecBuffer,
    client: &mut NamedPipeClient,
    mut remaining_read: usize,
) -> std::io::Result<AlignedVecBuffer> {
    buffer.clear();
    buffer.reserve(remaining_read)?;
    while remaining_read > 0 {
        let BufResult(read_result, new_buffer) = client.append(buffer).await;
        let n = read_result?;
        remaining_read = remaining_read.checked_sub(n).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Read more bytes than expected",
            )
        })?;
        buffer = new_buffer;
    }
    Ok(buffer)
}

struct AlignedVecBuffer(AlignedVec);

impl AlignedVecBuffer {
    fn clear(&mut self) {
        self.0.clear();
    }
}

impl IoBuf for AlignedVecBuffer {
    fn as_init(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl SetLen for AlignedVecBuffer {
    unsafe fn set_len(&mut self, len: usize) {
        unsafe { self.0.set_len(len) }
    }
}

impl IoBufMut for AlignedVecBuffer {
    fn as_uninit(&mut self) -> &mut [std::mem::MaybeUninit<u8>] {
        let ptr = self.0.as_mut_ptr() as *mut std::mem::MaybeUninit<u8>;
        let cap = self.0.capacity();
        unsafe { std::slice::from_raw_parts_mut(ptr, cap) }
    }

    fn reserve(&mut self, len: usize) -> Result<(), ReserveError> {
        self.0.reserve(len);
        Ok(())
    }

    fn reserve_exact(&mut self, len: usize) -> Result<(), ReserveExactError> {
        if self.0.capacity() - self.0.len() >= len {
            return Ok(());
        }

        self.0.reserve_exact(len);

        if self.0.capacity() - self.0.len() != len {
            return Err(ReserveExactError::ExactSizeMismatch {
                reserved: self.0.capacity() - self.0.len(),
                expected: len,
            });
        }
        Ok(())
    }
}

macro_rules! nop {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {};
}

macro_rules! implemented {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[link(name = "nvcuda", kind = "raw-dylib")]
            unsafe extern $abi {
                fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type;
            }
        )*

    };
}

cuda_function_declarations! {
    nop,
    implemented <= [
        // cuCtxCreate_v2,
        // cuCtxDetach,
        cuCtxGetApiVersion,
        // cuCtxGetCurrent,
        cuCtxGetDevice,
        cuCtxSynchronize,
        cuDeviceComputeCapability,
        // cuDeviceGet,
        cuDeviceGetAttribute,
        // cuDeviceGetCount,
        cuDeviceGetName,
        cuDeviceGetProperties,
        cuDeviceTotalMem_v2,
        cuDriverGetVersion,
        cuEventCreate,
        cuEventDestroy_v2,
        cuEventQuery,
        cuEventRecord,
        cuGetExportTable,
        cuInit,
        cuLaunchKernel,
        cuMemAlloc_v2,
        // cuMemFreeHost,
        // cuMemFree_v2,
        // cuMemGetAddressRange_v2,
        // cuMemHostAlloc,
        cuMemcpyDtoD_v2,
        cuMemcpyDtoDAsync_v2,
        cuMemcpyDtoHAsync_v2,
        cuMemcpyHtoD_v2,
        cuMemcpyHtoDAsync_v2,
        cuMemsetD8_v2,
        cuModuleGetFunction,
        // cuModuleGetGlobal_v2,
        cuModuleGetTexRef,
        cuModuleLoadData,
        cuStreamCreate,
        cuStreamDestroy_v2,
        // cuTexRefSetAddressMode,
        cuTexRefSetAddress_v2,
        // cuTexRefSetFilterMode,
        cuTexRefSetFlags,
        cuTexRefSetFormat,
        // cuTexRefSetMaxAnisotropy,
        // cuTexRefSetMipmapFilterMode,
        // cuTexRefSetMipmapLevelBias,
        // cuTexRefSetMipmapLevelClamp,
        cuStreamSynchronize,
        cuDevicePrimaryCtxRetain,
        cuCtxSetCurrent,
    ]
}
