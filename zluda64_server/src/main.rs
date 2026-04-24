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
use slab::Slab;
use std::{
    io::{self, Write},
    mem, ptr,
};
use windows_sys::Win32::Foundation::ERROR_MORE_DATA;
use zluda_server_common::*;

struct State {
    handles: Slab<usize>,
    dark_api: dark_api::cuda::ContextLocalStorageInterfaceV0301,
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
        Ok(Self {
            handles: Slab::new(),
            dark_api,
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
                buffer = handle_cuda_function::<ArchivedcuInitIn, cuInitOut>(
                    &mut client,
                    buffer,
                    cu_init,
                )
                .await?;
            }
            Some(Opcode::cuDeviceGetCount) => {
                buffer = handle_cuda_function::<ArchivedcuDeviceGetCountIn, cuDeviceGetCountOut>(
                    &mut client,
                    buffer,
                    cu_device_get_count,
                )
                .await?;
            }
            Some(Opcode::cuDeviceGetAttribute) => {
                buffer = handle_cuda_function::<
                    ArchivedcuDeviceGetAttributeIn,
                    cuDeviceGetAttributeOut,
                >(&mut client, buffer, cu_device_get_attribute)
                .await?;
            }
            Some(Opcode::cuDeviceGet) => {
                buffer = handle_cuda_function::<ArchivedcuDeviceGetIn, cuDeviceGetOut>(
                    &mut client,
                    buffer,
                    cu_device_get,
                )
                .await?;
            }
            Some(Opcode::cuCtxCreate_v2) => {
                buffer = handle_cuda_function::<ArchivedcuCtxCreate_v2In, cuCtxCreate_v2Out>(
                    &mut client,
                    buffer,
                    |input| cu_ctx_create_v2(input, &mut state),
                )
                .await?;
            }
            Some(Opcode::cuDriverGetVersion) => {
                buffer =
                    handle_cuda_function::<ArchivedcuDriverGetVersionIn, cuDriverGetVersionOut>(
                        &mut client,
                        buffer,
                        cu_driver_get_version,
                    )
                    .await?;
            }
            Some(Opcode::cuDeviceGetName) => {
                buffer =
                    handle_cuda_function_framed::<ArchivedcuDeviceGetNameIn, cuDeviceGetNameOut>(
                        &mut client,
                        buffer,
                        cu_device_get_name,
                    )
                    .await?;
            }
            Some(Opcode::cuDeviceTotalMem_v2) => {
                buffer = handle_cuda_function::<
                    ArchivedcuDeviceTotalMem_v2In,
                    cuDeviceTotalMem_v2Out,
                >(&mut client, buffer, cu_device_total_mem_v2)
                .await?;
            }
            Some(Opcode::ContextLocalStoragePut) => {
                buffer = handle_cuda_function::<
                    ArchivedContextLocalStoragePutIn,
                    ContextLocalStoragePutOut,
                >(&mut client, buffer, |input| {
                    context_local_storage_put(&mut state, input)
                })
                .await?;
            }
            Some(Opcode::ContextLocalStorageGet) => {
                buffer = handle_cuda_function::<
                    ArchivedContextLocalStorageGetIn,
                    ContextLocalStorageGetOut,
                >(&mut client, buffer, |input| {
                    context_local_storage_get(&mut state, input)
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
    Ok(cuCtxCreate_v2Out { pctx })
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

async fn handle_cuda_function<In: Portable, Out: Portable>(
    client: &mut NamedPipeClient,
    mut buffer: AlignedVecBuffer,
    handler: impl FnOnce(&In) -> Result<Out, CUerror>,
) -> std::io::Result<AlignedVecBuffer>
where
    Out: for<'a, 'b> Serialize<
        HighSerializer<&'a mut AlignedVec, ArenaHandle<'b>, rkyv::rancor::Failure>,
    >,
{
    buffer = read_all::<In>(buffer, client).await?;
    let input = unsafe { rkyv::access_unchecked::<In>(buffer.as_init()) };
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

async fn handle_cuda_function_framed<In: Portable, Out>(
    client: &mut NamedPipeClient,
    mut buffer: AlignedVecBuffer,
    handler: impl FnOnce(&In) -> Result<Out, CUerror>,
) -> std::io::Result<AlignedVecBuffer>
where
    Out: for<'a, 'b> Serialize<
        HighSerializer<&'a mut AlignedVec, ArenaHandle<'b>, rkyv::rancor::Failure>,
    >,
{
    buffer = read_all::<In>(buffer, client).await?;
    let input = unsafe { rkyv::access_unchecked::<In>(buffer.as_init()) };
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
    mut buffer: AlignedVecBuffer,
    client: &mut NamedPipeClient,
) -> std::io::Result<AlignedVecBuffer> {
    let mut remaining_read = mem::size_of::<T>();
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
        // cuCtxGetApiVersion,
        // cuCtxGetCurrent,
        // cuCtxGetDevice,
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
        // cuMemAlloc_v2,
        // cuMemFreeHost,
        // cuMemFree_v2,
        // cuMemGetAddressRange_v2,
        // cuMemHostAlloc,
        // cuMemcpyDtoDAsync_v2,
        // cuMemcpyDtoHAsync_v2,
        // cuMemcpyHtoDAsync_v2,
        // cuMemsetD8_v2,
        // cuModuleGetFunction,
        // cuModuleGetGlobal_v2,
        // cuModuleGetTexRef,
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
