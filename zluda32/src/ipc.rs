use cuda_types::cuda::CUerror;
use rand::distr::{Alphanumeric, SampleString};
use std::os::windows::io::{AsRawHandle, FromRawHandle};
// use rkyv::util::AlignedVec;
use std::num::NonZeroU32;
use std::process::{Child, Command, Stdio};
use std::ptr;
use std::sync::{Mutex, OnceLock};
use widestring::{u16str, U16CString};
use windows::core::{Error, Owned, PCSTR};
use windows::Win32::Foundation::*;
use windows::Win32::System::Pipes::*;
use zluda_server_common::{Envelope, Opcode};

fn server_path() -> &'static str {
    if cfg!(debug_assertions) {
        r"C:\dev\ZLUDA\target\debug\zluda64_server.exe"
    } else {
        "zluda64_server.exe"
    }
}

pub(crate) struct Server {
    pipe: std::fs::File,
    child: Child,
}

unsafe impl Sync for Server {}
unsafe impl Send for Server {}

impl Server {
    const CONFIG: bincode::config::Configuration<
        bincode::config::LittleEndian,
        bincode::config::Fixint,
    > = bincode::config::standard().with_fixed_int_encoding();

    fn new(pipe: std::fs::File, child: Child) -> Self {
        Self { pipe, child }
    }

    unsafe fn start() -> Result<Self, Error> {
        let name = Alphanumeric.sample_string(&mut rand::rng(), 32);
        let pipe_path = format!("\\\\.\\pipe\\zluda-{name}\0");
        let pipe = std::fs::File::from_raw_handle(
            CreateNamedPipeA(
                PCSTR(pipe_path.as_ptr()),
                windows::Win32::Storage::FileSystem::PIPE_ACCESS_DUPLEX,
                PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT | PIPE_REJECT_REMOTE_CLIENTS,
                1,
                4 * 1024,
                4 * 1024,
                0,
                None,
            )?
            .0,
        );
        let child = Command::new(server_path())
            .arg(&pipe_path[..pipe_path.len() - 1])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        match ConnectNamedPipe(HANDLE(pipe.as_raw_handle()), None) {
            Ok(_) => Ok(Server::new(pipe, child)),
            Err(e) if e.code() == ERROR_PIPE_CONNECTED.into() => Ok(Server::new(pipe, child)),
            Err(e) => Err(e),
        }
    }

    pub(crate) fn get() -> Result<&'static Mutex<Self>, CUerror> {
        static LOCK: OnceLock<Result<Mutex<Server>, Error>> = OnceLock::new();
        LOCK.get_or_init(|| Ok(Mutex::new(unsafe { Server::start()? })))
            .as_ref()
            .map_err(|_| CUerror::UNKNOWN)
    }

    pub(crate) fn call<Out: bincode::Decode<()>>(
        &mut self,
        opcode: Opcode,
        input: &impl bincode::Encode,
    ) -> Result<Out, CUerror> {
        bincode::encode_into_std_write(input, &mut self.pipe, Self::CONFIG)
            .map_err(|_| CUerror::UNKNOWN)?;
        bincode::decode_from_std_read::<Out, _, _>(&mut self.pipe, Self::CONFIG)
            .map_err(|_| CUerror::UNKNOWN)
    }
}
