// Prevent the console window from appearing when running the server on Windows
#![windows_subsystem = "windows"]

use compio::{
    buf::{IoBuf, IoBufMut, ReserveError, ReserveExactError, SetLen},
    fs::named_pipe::{ClientOptions, NamedPipeClient},
    io::{AsyncRead, AsyncReadExt, AsyncReadManaged, AsyncWrite, AsyncWriteExt},
    BufResult,
};
use cuda_macros::cuda_function_declarations;
use cuda_types::cuda::*;
use rkyv::{
    api::high::HighSerializer, ser::allocator::ArenaHandle, util::AlignedVec, Archive, Portable,
    Serialize,
};
use slab::Slab;
use std::mem;
use windows_sys::Win32::Foundation::ERROR_MORE_DATA;
use zluda_server_common::*;

struct State {
    handles: Slab<usize>,
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

async fn handle_cuda_function<In: Portable, Out>(
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
            rkyv::api::high::to_bytes_in::<_, rkyv::rancor::Failure>(
                &Envelope {
                    code: 0,
                    data: output,
                },
                &mut buffer.0,
            )
            .map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::Other, "Failed to serialize response")
            })?;
            let BufResult(result, new_buffer) = client.write_all(buffer).await;
            result?;
            Ok(new_buffer)
        }
        Err(e) => {
            buffer.clear();
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
        // cuCtxGetApiVersion,
        // cuCtxGetCurrent,
        // cuCtxGetDevice,
        // cuCtxSynchronize,
        // cuDeviceComputeCapability,
        // cuDeviceGet,
        // cuDeviceGetAttribute,
        cuDeviceGetCount,
        // cuDeviceGetName,
        // cuDeviceGetProperties,
        // cuDeviceTotalMem_v2,
        // cuDriverGetVersion,
        // cuEventCreate,
        // cuEventDestroy_v2,
        // cuGetExportTable,
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
