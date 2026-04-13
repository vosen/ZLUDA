use rand::distr::{Alphanumeric, SampleString};
use std::process::{Command, Stdio};
use std::ptr;
use widestring::{u16str, U16CString};
use windows::core::{Error, Owned, PCSTR};
use windows::Win32::Foundation::*;
use windows::Win32::System::Pipes::*;

unsafe fn start() -> Result<String, Error> {
    let name = Alphanumeric.sample_string(&mut rand::rng(), 32);
    let pipe_path = format!(r"\\.\pipe\zluda-{name}\0");
    let pipe = Owned::new(CreateNamedPipeA(
        PCSTR(pipe_path.as_ptr()),
        windows::Win32::Storage::FileSystem::PIPE_ACCESS_DUPLEX,
        PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT | PIPE_REJECT_REMOTE_CLIENTS,
        1,
        128 * 1024,
        16,
        0,
        None,
    )?);
    let child = Command::new("zluda64.exe")
        .arg(&pipe_path[..pipe_path.len() - 1])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    ConnectNamedPipe(*pipe, None)?;
    let in_buffer = [0xffu8; 128 * 1024];
    let mut out_buffer = [0xffu8; 4];
    let mut read = 0;
    TransactNamedPipe(
        *pipe,
        Some(in_buffer.as_ptr().cast()),
        in_buffer.len() as u32,
        Some(out_buffer.as_mut_ptr().cast()),
        out_buffer.len() as u32,
        &mut read,
        None,
    )?;

    todo!()
}
