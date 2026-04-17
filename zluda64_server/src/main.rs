use compio::{
    fs::named_pipe::ClientOptions,
    io::{AsyncRead, AsyncReadExt, AsyncWriteExt},
    BufResult,
};
use slab::Slab;
use windows_sys::Win32::Foundation::ERROR_MORE_DATA;

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
    let mut buffer = vec![0u8; 128 * 1024];
    loop {
        let opcode = client.read_u32_le().await?; // Read the length prefix
        client.read_managed(buffer_pool, len)
    }
    Ok(())
}

struct PipeReader {
    client: compio::fs::named_pipe::NamedPipeClient,
}

impl bincode::de::read::Reader for PipeReader {
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), bincode::error::DecodeError> {
        self.client.read_exact(bytes)
    }
}
