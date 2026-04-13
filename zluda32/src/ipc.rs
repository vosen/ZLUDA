use cuda_types::cuda::CUerror;
use rand::distr::{Alphanumeric, SampleString};
use std::num::NonZeroU32;
use std::process::{Child, Command, Stdio};
use std::ptr;
use std::sync::OnceLock;
use widestring::{u16str, U16CString};
use windows::core::{Error, Owned, PCSTR};
use windows::Win32::Foundation::*;
use windows::Win32::System::Pipes::*;

fn server_path() -> &'static str {
    if cfg!(debug_assertions) {
        r"C:\dev\ZLUDA\target\debug\zluda64_server.exe"
    } else {
        "zluda64_server.exe"
    }
}

pub(crate) struct Server {
    pipe: Owned<HANDLE>,
    child: Child,
}

unsafe impl Sync for Server {}
unsafe impl Send for Server {}

impl Server {
    unsafe fn start() -> Result<Self, Error> {
        let name = Alphanumeric.sample_string(&mut rand::rng(), 32);
        let pipe_path = format!("\\\\.\\pipe\\zluda-{name}\0");
        let pipe = Owned::new(CreateNamedPipeA(
            PCSTR(pipe_path.as_ptr()),
            windows::Win32::Storage::FileSystem::PIPE_ACCESS_DUPLEX,
            PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT | PIPE_REJECT_REMOTE_CLIENTS,
            1,
            128 * 1024,
            1024,
            0,
            None,
        )?);
        let child = Command::new(server_path())
            .arg(&pipe_path[..pipe_path.len() - 1])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        match ConnectNamedPipe(*pipe, None) {
            Ok(_) => Ok(Server { pipe, child }),
            Err(e) if e.code() == ERROR_PIPE_CONNECTED.into() => Ok(Server { pipe, child }),
            Err(e) => Err(e),
        }
    }

    pub(crate) fn get() -> Result<&'static Server, CUerror> {
        static LOCK: OnceLock<Result<Server, Error>> = OnceLock::new();
        LOCK.get_or_init(|| unsafe { Server::start() })
            .as_ref()
            .map_err(|_| CUerror::UNKNOWN)
    }

    pub(crate) fn cu_init(&self, flags: u32) -> Result<(), CUerror> {
        let mut in_buffer = [0xffu8; 8];
        in_buffer[..4].copy_from_slice(1u32.to_le_bytes().as_slice());
        in_buffer[4..].copy_from_slice(flags.to_le_bytes().as_slice());
        let mut out_buffer = [0xffu8; 4];
        let mut read = 0;
        unsafe {
            TransactNamedPipe(
                *self.pipe,
                Some(in_buffer.as_ptr().cast()),
                in_buffer.len() as u32,
                Some(out_buffer.as_mut_ptr().cast()),
                out_buffer.len() as u32,
                &mut read,
                None,
            )
            .map_err(|_| CUerror::UNKNOWN)?;
        }
        let result = u32::from_le_bytes(out_buffer);
        NonZeroU32::new(result).map_or_else(|| Ok(()), |e| Err(CUerror(e)))
    }
}
