// Prevent the console window from appearing when running the server on Windows
#![windows_subsystem = "windows"]

use compio::{
    buf::{buf_try, IoBuf, IoBufMut, ReserveError, ReserveExactError, SetLen},
    fs::named_pipe::{ClientOptions, NamedPipeClient},
    io::{AsyncRead, AsyncReadExt, AsyncReadManaged, AsyncWrite, AsyncWriteExt},
    BufResult,
};
use cuda_macros::cuda_function_declarations;
use cuda_types::cuda::*;
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
    ffi::{c_void, CStr, CString},
    io::{self, Write},
    mem,
    ops::Range,
    ptr::{self, NonNull},
};
use zluda_server_common::*;

struct State {
    handles: Slab<usize>,
    dark_api: dark_api::cuda::ContextLocalStorageInterfaceV0301,
    devmemory: Vec<Allocator>,
    modules: ModuleLaunchData,
}

struct ModuleLaunchData {
    dark_api: dark_api::zluda32::Zluda32Internal,
    modules: FxHashMap<CUmodule, Vec<Global>>,
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
        devmemory: &mut Vec<Allocator>,
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
        let mut device = 0;
        unsafe { cuCtxGetDevice(&mut device) }?;
        let globals = (0..count as usize)
            .map(|i| {
                if alignments[i] > Allocator::ALLOCATION_UNIT {
                    return Err(CUerror::OUT_OF_MEMORY);
                }
                let initializer = std::slice::from_raw_parts(initializers[i], sizes[i] as usize);
                let allocation = devmemory[device as usize].alloc_range(sizes[i])?;
                let devptr = devmemory[device as usize].translate_range(allocation.clone())?;
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
        self.modules.insert(module, globals);
        Ok(())
    }
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
    module: CUmodule,
    explicit_argument_count: u32,
}

struct Allocator {
    device: u32,
    start: Option<*mut c_void>,
    allocator: range_alloc::RangeAllocator<u32>,
    allocation_ends: FxHashMap<u32, u32>,
}

impl Allocator {
    const ALLOCATION_UNIT: u32 = 128;
    const ALLOCATOR_SIZE: u32 = 512 * 1024 * 1024; // 512 MiB

    fn new(device: u32) -> Self {
        Self {
            device,
            start: None,
            allocator: range_alloc::RangeAllocator::new(
                // starting from 1 to avoid handing out null pointers
                1..Self::ALLOCATOR_SIZE / Self::ALLOCATION_UNIT,
            ),
            allocation_ends: FxHashMap::default(),
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
        let end = self
            .allocation_ends
            .remove(&start)
            .ok_or(CUerror::INVALID_VALUE)?;
        self.allocator.free_range(start..end);
        Ok(())
    }

    fn translate(&self, offset: u32) -> Result<*mut c_void, CUerror> {
        let base_ptr = self.get_device_ptr()?;
        Ok(base_ptr.wrapping_byte_add(offset as usize))
    }

    fn translate_range(&self, range: Range<u32>) -> Result<*mut c_void, CUerror> {
        let base_ptr = self.get_device_ptr()?;
        Ok(base_ptr.wrapping_byte_add(range.start as usize * Self::ALLOCATION_UNIT as usize))
    }
}

impl State {
    fn new() -> Result<Self, CUerror> {
        unsafe { cuInit(0) }?;
        let mut table_ptr = std::ptr::null();
        unsafe {
            cuGetExportTable(
                &mut table_ptr,
                &dark_api::cuda::ContextLocalStorageInterfaceV0301::GUID,
            )
        }?;
        let dark_api = unsafe { dark_api::cuda::ContextLocalStorageInterfaceV0301::new(table_ptr) };
        let mut devices = 0;
        unsafe { cuDeviceGetCount(&mut devices) }?;
        Ok(Self {
            handles: Slab::new(),
            dark_api,
            devmemory: (0..devices)
                .map(|device| Allocator::new(device as u32))
                .collect(),
            modules: ModuleLaunchData::new()?,
        })
    }

    fn insert<T: Sized>(&mut self, handle: *mut T) -> u32 {
        (self.handles.insert(handle as usize) as u32) + 1
    }

    fn get<T: Sized>(&self, id: u32) -> Result<*mut T, CUerror> {
        if id == 0 {
            return Err(CUerror::INVALID_VALUE);
        }
        self.handles
            .get((id - 1) as usize)
            .map(|&handle| handle as *mut T)
            .ok_or(CUerror::INVALID_VALUE)
    }
}

#[compio::main]
async fn main() -> std::io::Result<()> {
    let pipe_name = std::env::args_os().nth(1).ok_or(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        "Pipe name not provided",
    ))?;
    let mut client = ClientOptions::new()
        .pipe_mode(compio::fs::named_pipe::PipeMode::Byte)
        .read(true)
        .write(true)
        .open(pipe_name)
        .await?;
    let mut buffer = AlignedVecBuffer(AlignedVec::new());
    let mut state = State::new().map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to initialize CUDA context",
        )
    })?;
    loop {
        let opcode = client.read_u32_le().await?; // Read the length prefix
        buffer.clear();
        match Opcode::from_repr(opcode) {
            Some(Opcode::cuInit) => {
                buffer = handle_cuda_function::<cuInitIn, cuInitOut>(&mut client, buffer, cu_init)
                    .await?;
            }
            Some(Opcode::cuDeviceGetCount) => {
                buffer = handle_cuda_function::<cuDeviceGetCountIn, cuDeviceGetCountOut>(
                    &mut client,
                    buffer,
                    cu_device_get_count,
                )
                .await?;
            }
            Some(Opcode::cuDeviceGetAttribute) => {
                buffer = handle_cuda_function::<cuDeviceGetAttributeIn, cuDeviceGetAttributeOut>(
                    &mut client,
                    buffer,
                    cu_device_get_attribute,
                )
                .await?;
            }
            Some(Opcode::cuDeviceGet) => {
                buffer = handle_cuda_function::<cuDeviceGetIn, cuDeviceGetOut>(
                    &mut client,
                    buffer,
                    cu_device_get,
                )
                .await?;
            }
            Some(Opcode::cuCtxCreate_v2) => {
                buffer = handle_cuda_function::<cuCtxCreate_v2In, cuCtxCreate_v2Out>(
                    &mut client,
                    buffer,
                    |input| cu_ctx_create_v2(input, &mut state),
                )
                .await?;
            }
            Some(Opcode::cuDriverGetVersion) => {
                buffer = handle_cuda_function::<cuDriverGetVersionIn, cuDriverGetVersionOut>(
                    &mut client,
                    buffer,
                    cu_driver_get_version,
                )
                .await?;
            }
            Some(Opcode::cuDeviceGetName) => {
                buffer = handle_cuda_function_framed_out::<cuDeviceGetNameIn, cuDeviceGetNameOut>(
                    &mut client,
                    buffer,
                    cu_device_get_name,
                )
                .await?;
            }
            Some(Opcode::cuDeviceTotalMem_v2) => {
                buffer = handle_cuda_function::<cuDeviceTotalMem_v2In, cuDeviceTotalMem_v2Out>(
                    &mut client,
                    buffer,
                    cu_device_total_mem_v2,
                )
                .await?;
            }
            Some(Opcode::ContextLocalStoragePut) => {
                buffer =
                    handle_cuda_function::<ContextLocalStoragePutIn, ContextLocalStoragePutOut>(
                        &mut client,
                        buffer,
                        |input| context_local_storage_put(&mut state, input),
                    )
                    .await?;
            }
            Some(Opcode::ContextLocalStorageGet) => {
                buffer =
                    handle_cuda_function::<ContextLocalStorageGetIn, ContextLocalStorageGetOut>(
                        &mut client,
                        buffer,
                        |input| context_local_storage_get(&mut state, input),
                    )
                    .await?;
            }
            Some(Opcode::cuCtxGetApiVersion) => {
                buffer = handle_cuda_function::<cuCtxGetApiVersionIn, cuCtxGetApiVersionOut>(
                    &mut client,
                    buffer,
                    |input| cu_ctx_get_api_version(&mut state, input),
                )
                .await?;
            }
            Some(Opcode::cuModuleLoadData) => {
                buffer = handle_cuda_function_framed_in::<cuModuleLoadDataIn, cuModuleLoadDataOut>(
                    &mut client,
                    buffer,
                    |input| cu_module_load_data(&mut state, input),
                )
                .await?;
            }
            Some(Opcode::cuModuleGetFunction) => {
                buffer = handle_cuda_function_framed_in::<
                    cuModuleGetFunctionIn,
                    cuModuleGetFunctionOut,
                >(&mut client, buffer, |input| {
                    cu_module_get_function(&mut state, input)
                })
                .await?;
            }
            Some(Opcode::cuModuleGetGlobal_v2) => {
                buffer = handle_cuda_function_framed_in::<
                    cuModuleGetGlobal_v2In,
                    cuModuleGetGlobal_v2Out,
                >(&mut client, buffer, |input| {
                    cu_module_get_global_v2(&mut state, input)
                })
                .await?;
            }
            Some(Opcode::cuMemAlloc_v2) => {
                buffer = handle_cuda_function::<cuMemAlloc_v2In, cuMemAlloc_v2Out>(
                    &mut client,
                    buffer,
                    |input| cu_mem_alloc_v2(&mut state, input),
                )
                .await?;
            }
            Some(Opcode::cuMemcpyHtoDAsync_v2) => {
                buffer = handle_cuda_function_framed_in::<
                    cuMemcpyHtoDAsync_v2In,
                    cuMemcpyHtoDAsync_v2Out,
                >(&mut client, buffer, |input| {
                    cu_memcpy_hto_d_async_v2(&mut state, input)
                })
                .await?;
            }
            _ => {
                client.write_u32_le(CUerror::NOT_SUPPORTED.0.get()).await?;
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Unsupported operation",
                ));
            }
        }
    }
}

fn cu_memcpy_hto_d_async_v2(
    state: &mut State,
    input: &ArchivedcuMemcpyHtoDAsync_v2In,
) -> Result<cuMemcpyHtoDAsync_v2Out, CUerror> {
    let mut device = 0;
    unsafe { cuCtxGetDevice(&mut device) }?;
    let devptr = state.devmemory[device as usize].translate(input.dst_device.to_native())?;
    let stream = input.stream.to_native();
    let stream = if stream == 0 {
        CUstream(ptr::null_mut())
    } else {
        CUstream(state.get(stream)?)
    };
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
    let mut device = 0;
    unsafe { cuCtxGetDevice(&mut device) }?;
    let fake_ptr = state.devmemory[device as usize].alloc(input.bytesize.to_native())?;
    Ok(cuMemAlloc_v2Out {
        dptr: u32_le::from_native(fake_ptr),
    })
}

fn cu_module_get_global_v2(
    state: &mut State,
    input: &ArchivedcuModuleGetGlobal_v2In,
) -> Result<cuModuleGetGlobal_v2Out, CUerror> {
    let hmod = state.get::<CUmod_st>(input.hmod.to_native())?;
    let globals = state
        .modules
        .modules
        .get(&CUmodule(hmod))
        .ok_or(CUerror::INVALID_VALUE)?;
    let global = globals
        .iter()
        .find(|g| g.name.as_bytes() == input.name.as_slice())
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
    let hmod = state.get(input.hmod.to_native())?;
    unsafe { cuModuleGetFunction(&mut hfunc, CUmodule(hmod), input.name.as_ptr().cast()) }?;
    Ok(cuModuleGetFunctionOut {
        hfunc: u32_le::from_native(state.insert(hfunc.0)),
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
        module: u32_le::from_native(state.insert(module.0)),
    })
}

fn cu_init(input: &ArchivedcuInitIn) -> Result<cuInitOut, CUerror> {
    unsafe { cuInit(input.Flags.to_native()) }?;
    Ok(cuInitOut {})
}

fn cu_device_get_count(
    _input: &ArchivedcuDeviceGetCountIn,
) -> Result<cuDeviceGetCountOut, CUerror> {
    let mut count = 0;
    unsafe { cuDeviceGetCount(&mut count) }?;
    Ok(cuDeviceGetCountOut { count })
}

fn cu_device_get_attribute(
    input: &ArchivedcuDeviceGetAttributeIn,
) -> Result<cuDeviceGetAttributeOut, CUerror> {
    let mut pi = 0;
    unsafe {
        cuDeviceGetAttribute(
            &mut pi,
            CUdevice_attribute_enum(input.attrib.to_native()),
            input.dev.to_native(),
        )
    }?;
    Ok(cuDeviceGetAttributeOut { pi })
}

fn cu_device_get(input: &ArchivedcuDeviceGetIn) -> Result<cuDeviceGetOut, CUerror> {
    let mut device = 0;
    unsafe { cuDeviceGet(&mut device, input.ordinal.to_native()) }?;
    Ok(cuDeviceGetOut { device })
}

fn cu_device_get_name(input: &ArchivedcuDeviceGetNameIn) -> Result<cuDeviceGetNameOut, CUerror> {
    let mut name = vec![0u8; input.len.to_native() as usize];
    unsafe {
        cuDeviceGetName(
            name.as_mut_ptr().cast(),
            input.len.to_native(),
            input.dev.to_native(),
        )
    }?;
    if let Some(pos) = name.iter().copied().position(|c| c == 0) {
        name.truncate(pos);
    }
    Ok(cuDeviceGetNameOut { name })
}

fn cu_ctx_create_v2(
    input: &ArchivedcuCtxCreate_v2In,
    state: &mut State,
) -> Result<cuCtxCreate_v2Out, CUerror> {
    let mut pctx = unsafe { mem::zeroed() };
    unsafe { cuCtxCreate_v2(&mut pctx, input.flags.to_native(), input.dev.to_native()) }?;
    let pctx = state.insert(pctx.0);
    Ok(cuCtxCreate_v2Out {
        pctx: u32_le::from_native(pctx),
    })
}

fn cu_device_total_mem_v2(
    input: &ArchivedcuDeviceTotalMem_v2In,
) -> Result<cuDeviceTotalMem_v2Out, CUerror> {
    let mut bytes = 0usize;
    unsafe { cuDeviceTotalMem_v2(&mut bytes, input.dev.to_native()) }?;
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
    input: &ArchivedcuCtxGetApiVersionIn,
) -> Result<cuCtxGetApiVersionOut, CUerror> {
    let ctx = CUcontext(state.get(input.ctx.to_native())?);
    let mut version = 0;
    unsafe { cuCtxGetApiVersion(ctx, &mut version) }?;
    Ok(cuCtxGetApiVersionOut { version })
}

fn context_local_storage_put(
    state: &mut State,
    input: &ArchivedContextLocalStoragePutIn,
) -> Result<ContextLocalStoragePutOut, CUerror> {
    unsafe {
        state.dark_api.context_local_storage_put(
            CUcontext(state.get(input.cu_ctx.to_native())?),
            input.key.to_native() as _,
            input.value.to_native() as _,
            None,
        )
    }?;
    Ok(ContextLocalStoragePutOut {})
}

fn context_local_storage_get(
    state: &mut State,
    input: &ArchivedContextLocalStorageGetIn,
) -> Result<ContextLocalStorageGetOut, CUerror> {
    let mut value = ptr::null_mut();
    unsafe {
        state.dark_api.context_local_storage_get(
            &mut value,
            CUcontext(state.get(input.cu_ctx.to_native())?),
            input.key.to_native() as _,
        )
    }?;
    Ok(ContextLocalStorageGetOut {
        value: u32_le::from_native(value as u32),
    })
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
        cuCtxCreate_v2,
        // cuCtxDetach,
        cuCtxGetApiVersion,
        // cuCtxGetCurrent,
        cuCtxGetDevice,
        // cuCtxSynchronize,
        // cuDeviceComputeCapability,
        cuDeviceGet,
        cuDeviceGetAttribute,
        cuDeviceGetCount,
        cuDeviceGetName,
        // cuDeviceGetProperties,
        cuDeviceTotalMem_v2,
        cuDriverGetVersion,
        // cuEventCreate,
        // cuEventDestroy_v2,
        cuGetExportTable,
        cuInit,
        // cuLaunchKernel,
        cuMemAlloc_v2,
        // cuMemFreeHost,
        // cuMemFree_v2,
        // cuMemGetAddressRange_v2,
        // cuMemHostAlloc,
        // cuMemcpyDtoDAsync_v2,
        // cuMemcpyDtoHAsync_v2,
        cuMemcpyHtoD_v2,
        cuMemcpyHtoDAsync_v2,
        // cuMemsetD8_v2,
        cuModuleGetFunction,
        cuModuleGetGlobal_v2,
        // cuModuleGetTexRef,
        cuModuleLoadData,
        // cuStreamCreate,
        // cuStreamDestroy_v2,
        // cuTexRefSetAddressMode,
        // cuTexRefSetAddress_v2,
        // cuTexRefSetFilterMode,
        // cuTexRefSetFlags,
        // cuTexRefSetFormat,
        // cuTexRefSetMaxAnisotropy,
        // cuTexRefSetMipmapFilterMode,
        // cuTexRefSetMipmapLevelBias,
        // cuTexRefSetMipmapLevelClamp,
    ]
}
