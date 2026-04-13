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
        .pipe_mode(compio::fs::named_pipe::PipeMode::Message)
        .read(true)
        .write(true)
        .open(pipe_name)
        .await?;
    let mut buffer = vec![0u8; 128 * 1024];
    let mut append = false;
    loop {
        let read_result = if append {
            append = false;
            client.append(buffer).await
        } else {
            client.read(buffer).await
        };
        match read_result {
            BufResult(Ok(0), _) => break, // EOF
            BufResult(Ok(n), b) => {
                println!("Read {} bytes", n);
                client.write_u32_le(0).await?;
                buffer = b;
            }
            BufResult(Err(e), b) if e.raw_os_error() == Some(ERROR_MORE_DATA as i32) => {
                append = true;
                buffer = b;
            }
            BufResult(Err(e), _) => return Err(e),
        }
    }
    Ok(())
}
