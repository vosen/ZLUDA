use cuda_types::cuda::CUerror;
use rand::distr::{Alphanumeric, SampleString};
use rkyv::api::high::HighSerializer;
use rkyv::rend::u32_le;
use rkyv::ser::allocator::ArenaHandle;
use rkyv::util::AlignedVec;
use rkyv::{rancor, Portable, Serialize};
use std::io::{Read, Write};
use std::os::windows::io::{AsHandle, AsRawHandle, FromRawHandle};
// use rkyv::util::AlignedVec;
use std::num::NonZeroU32;
use std::process::{Child, Command, Stdio};
use std::sync::{Mutex, OnceLock};
use std::{mem, ptr};
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
    _child: Child,
    buffer: AlignedVec,
}

unsafe impl Sync for Server {}
unsafe impl Send for Server {}

impl Server {
    fn new(pipe: std::fs::File, child: Child) -> Self {
        Self {
            pipe,
            _child: child,
            buffer: AlignedVec::new(),
        }
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
        zluda_windows::kill_child_on_process_exit(child.as_handle().as_raw_handle())?;
        match ConnectNamedPipe(HANDLE(pipe.as_raw_handle()), None) {
            Ok(_) => Ok(Server::new(pipe, child)),
            Err(e) if e.code() == ERROR_PIPE_CONNECTED.into() => Ok(Server::new(pipe, child)),
            Err(e) => Err(e),
        }
    }

    fn get() -> Result<&'static Mutex<Self>, CUerror> {
        static LOCK: OnceLock<Result<Mutex<Server>, Error>> = OnceLock::new();
        LOCK.get_or_init(|| Ok(Mutex::new(unsafe { Server::start()? })))
            .as_ref()
            .map_err(|_| CUerror::UNKNOWN)
    }

    pub(crate) fn remote_call<Out: Portable + Clone>(
        opcode: Opcode,
        data: impl for<'a, 'b> Serialize<
            HighSerializer<&'a mut AlignedVec, ArenaHandle<'b>, rkyv::rancor::Failure>,
        >,
    ) -> Result<Out, CUerror> {
        let this = &mut *Self::get()?.lock().map_err(|_| CUerror::UNKNOWN)?;
        this.buffer.clear();
        let slice = rkyv::api::high::to_bytes_in::<_, rkyv::rancor::Failure>(
            &Envelope {
                code: opcode as u32,
                data,
            },
            &mut this.buffer,
        )
        .map_err(|_| CUerror::UNKNOWN)?;
        this.pipe.write_all(&slice).map_err(|_| CUerror::UNKNOWN)?;
        read_return_code(this)?;
        let output = unsafe { rkyv::access_unchecked::<Out>(&this.buffer) };
        Ok(output.clone())
    }
}

fn read_return_code(this: &mut Server) -> Result<(), CUerror> {
    let mut code_buffer = [0; 4];
    this.pipe
        .read_exact(&mut code_buffer)
        .map_err(|_| CUerror::UNKNOWN)?;
    let code = u32::from_le_bytes(code_buffer);
    match NonZeroU32::new(code) {
        None => Ok(()),
        Some(code) => Err(CUerror(code)),
    }
}
