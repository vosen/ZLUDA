use cuda_types::cuda::CUerror;
use rand::distr::{Alphanumeric, SampleString};
use rkyv::api::high::HighSerializer;
use rkyv::de::Pool;
use rkyv::rancor::{Failure, Strategy};
use rkyv::ser::allocator::ArenaHandle;
use rkyv::util::AlignedVec;
use rkyv::{Archive, Deserialize, Portable, Serialize};
use std::io::{Read, Write};
use std::num::NonZeroU32;
use std::os::windows::io::{AsHandle, AsRawHandle, FromRawHandle};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::{env, mem};
use windows::core::{Error, PCSTR};
use windows::Win32::Foundation::*;
use windows::Win32::System::Pipes::*;
use zluda_server_common::Opcode;

pub(crate) struct Server {
    pipe: std::fs::File,
    _child: Child,
    buffer: AlignedVec,
}

impl Server {
    fn new(pipe: std::fs::File, child: Child) -> Self {
        Self {
            pipe,
            _child: child,
            buffer: AlignedVec::new(),
        }
    }

    pub unsafe fn start() -> Result<Self, Error> {
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
        let mut primary_path = zluda_common::os::self_path().ok_or(Error::new(
            E_FAIL,
            "Could not get path to the executing module",
        ))?;
        primary_path.pop();
        if cfg!(debug_assertions) {
            primary_path.push("../../debug/zluda64_server.exe");
        } else {
            primary_path.push("../zluda64_server.exe");
        };
        let fallback_path = env::var("ZLUDA64_PATH").ok().map(PathBuf::from);
        let spawn_server = |path: &PathBuf| {
            Command::new(path)
                .arg(&pipe_path[..pipe_path.len() - 1])
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .current_dir(path.parent().unwrap())
                .spawn()
        };
        let child = match (spawn_server(&primary_path), fallback_path) {
            (Ok(c), _) => c,
            (Err(_), Some(fallback_path)) => spawn_server(&fallback_path)?,
            (Err(e), None) => return Err(e.into()),
        };
        zluda_windows::kill_child_on_process_exit(child.as_handle().as_raw_handle())?;
        match ConnectNamedPipe(HANDLE(pipe.as_raw_handle()), None) {
            Ok(_) => Ok(Server::new(pipe, child)),
            Err(e) if e.code() == ERROR_PIPE_CONNECTED.into() => Ok(Server::new(pipe, child)),
            Err(e) => Err(e),
        }
    }

    pub(crate) fn remote_call_zero_copy<Out: Portable + Clone>(
        &mut self,
        opcode: Opcode,
        data: impl for<'a, 'b> Serialize<
            HighSerializer<&'a mut AlignedVec, ArenaHandle<'b>, rkyv::rancor::Failure>,
        >,
    ) -> Result<Out, CUerror> {
        self.buffer.clear();
        self.pipe
            .write_all(&(opcode as u32).to_le_bytes()[..])
            .map_err(|_| CUerror::UNKNOWN)?;
        let slice =
            rkyv::api::high::to_bytes_in::<_, rkyv::rancor::Failure>(&data, &mut self.buffer)
                .map_err(|_| CUerror::UNKNOWN)?;
        self.pipe.write_all(&slice).map_err(|_| CUerror::UNKNOWN)?;
        read_return_code(self)?;
        self.buffer.resize(mem::size_of::<Out>(), 0);
        self.pipe
            .read_exact(&mut self.buffer)
            .map_err(|_| CUerror::UNKNOWN)?;
        let output = unsafe { rkyv::access_unchecked::<Out>(&self.buffer) };
        Ok(output.clone())
    }

    pub(crate) fn remote_call_framed_in<Out: Portable + Clone>(
        &mut self,
        opcode: Opcode,
        data: impl for<'a, 'b> Serialize<
            HighSerializer<&'a mut AlignedVec, ArenaHandle<'b>, rkyv::rancor::Failure>,
        >,
    ) -> Result<Out, CUerror> {
        self.buffer.clear();
        self.pipe
            .write_all(&(opcode as u32).to_le_bytes()[..])
            .map_err(|_| CUerror::UNKNOWN)?;
        let slice =
            rkyv::api::high::to_bytes_in::<_, rkyv::rancor::Failure>(&data, &mut self.buffer)
                .map_err(|_| CUerror::UNKNOWN)?;
        self.pipe
            .write_all(&(slice.len() as u32).to_le_bytes()[..])
            .map_err(|_| CUerror::UNKNOWN)?;
        self.pipe.write_all(&slice).map_err(|_| CUerror::UNKNOWN)?;
        read_return_code(self)?;
        self.buffer.resize(mem::size_of::<Out>(), 0);
        self.pipe
            .read_exact(&mut self.buffer)
            .map_err(|_| CUerror::UNKNOWN)?;
        let output = unsafe { rkyv::access_unchecked::<Out>(&self.buffer) };
        Ok(output.clone())
    }

    pub(crate) fn remote_call_framed_out<Out: Archive>(
        &mut self,
        opcode: Opcode,
        data: impl for<'a, 'b> Serialize<
            HighSerializer<&'a mut AlignedVec, ArenaHandle<'b>, rkyv::rancor::Failure>,
        >,
    ) -> Result<Out, CUerror>
    where
        <Out as Archive>::Archived: Deserialize<Out, Strategy<Pool, Failure>>,
    {
        self.buffer.clear();
        self.pipe
            .write_all(&(opcode as u32).to_le_bytes()[..])
            .map_err(|_| CUerror::UNKNOWN)?;
        let slice =
            rkyv::api::high::to_bytes_in::<_, rkyv::rancor::Failure>(&data, &mut self.buffer)
                .map_err(|_| CUerror::UNKNOWN)?;
        self.pipe.write_all(&slice).map_err(|_| CUerror::UNKNOWN)?;
        read_return_code(self)?;
        let out_size = read_u32(self)?;
        self.buffer.resize(out_size as usize, 0);
        self.pipe
            .read_exact(&mut self.buffer)
            .map_err(|_| CUerror::UNKNOWN)?;
        unsafe { rkyv::from_bytes_unchecked::<Out, rkyv::rancor::Failure>(&self.buffer) }
            .map_err(|_| CUerror::UNKNOWN)
    }
}

fn read_return_code(this: &mut Server) -> Result<(), CUerror> {
    let code = read_u32(this)?;
    match NonZeroU32::new(code) {
        None => Ok(()),
        Some(code) => Err(CUerror(code)),
    }
}

fn read_u32(this: &mut Server) -> Result<u32, CUerror> {
    let mut code_buffer = [0u8; 4];
    this.pipe
        .read_exact(&mut code_buffer)
        .map_err(|_| CUerror::UNKNOWN)?;
    let code = u32::from_le_bytes(code_buffer);
    Ok(code)
}
