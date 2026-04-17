use std::mem;

use compio::{
    buf::{IoBuf, IoBufMut, ReserveError, ReserveExactError, SetLen},
    fs::named_pipe::ClientOptions,
    io::{AsyncRead, AsyncReadExt, AsyncReadManaged, AsyncWriteExt},
    BufResult,
};
use cuda_types::cuda::*;
use rkyv::util::AlignedVec;
use slab::Slab;
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
        let opcode = client.read_u32().await?; // Read the length prefix
        buffer.clear();
        match Opcode::from_repr(opcode) {
            Some(Opcode::cuInit) => {
                let mut remaining_read = mem::size_of::<ArchivedcuInitIn>();
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
                let input = unsafe { rkyv::access_unchecked::<ArchivedcuInitIn>(buffer.as_init()) };
                println!("cuInit({:?})", input.Flags);
            }
            _ => {
                client.write_u32(CUerror::NOT_SUPPORTED.0.get()).await?;
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Unsupported operation",
                ));
            }
        }
    }
    Ok(())
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
