use core::slice;
use cuda_types::cuda::CUerror;
use rkyv::api::high::HighSerializer;
use rkyv::api::low::LowSerializer;
use rkyv::de::Pool;
use rkyv::rancor::{Failure, Fallible, Strategy};
use rkyv::ser::allocator::ArenaHandle;
use rkyv::ser::Allocator;
use rkyv::util::AlignedVec;
use rkyv::{Archive, Deserialize, Portable, Serialize};
use std::io::{Read, Write};
use std::num::NonZeroU32;
use std::os::windows::io::{AsHandle, AsRawHandle};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::{env, mem, ptr};
use windows::core::{Error, Owned, PCSTR};
use windows::Win32::Foundation::*;
use windows::Win32::System::Memory::*;
use windows::Win32::System::Threading::*;
use zluda_server_common::{Opcode, Serializer};

pub(crate) struct Server {
    local: zluda_server_common::Endpoint,
    remote: zluda_server_common::Endpoint,
    _child: Child,
    arena: stumpalo::Arena,
}

impl Server {
    pub unsafe fn start() -> Result<Self, Error> {
        let local = zluda_server_common::Endpoint::new()?;
        let remote = zluda_server_common::Endpoint::new()?;
        let spawn_server = |path: &PathBuf| {
            Command::new(path)
                .args([
                    &local.event_name,
                    &local.shared_memory.name,
                    &remote.event_name,
                    &remote.shared_memory.name,
                ])
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .current_dir(path.parent().unwrap())
                .spawn()
        };
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
        let child = match (spawn_server(&primary_path), fallback_path) {
            (Ok(c), _) => c,
            (Err(_), Some(fallback_path)) => spawn_server(&fallback_path)?,
            (Err(e), None) => return Err(e.into()),
        };
        zluda_windows::kill_child_on_process_exit(child.as_handle().as_raw_handle())?;
        let arena = stumpalo::Arena::new();
        Ok(Server {
            local,
            remote,
            _child: child,
            arena,
        })
    }

    pub(crate) fn remote_call_zero_copy<Out: Portable + Clone>(
        &mut self,
        opcode: Opcode,
        data: impl for<'a, 'b> Serialize<Serializer<'a, 'b>>,
    ) -> Result<Out, CUerror> {
        self.remote.shared_memory.write_header(opcode as u32);
        self.remote.shared_memory.write_body(&data);
        unsafe { SignalObjectAndWait(*self.remote.event, *self.local.event, INFINITE, false) };
        let return_value = self.local.shared_memory.read_header();
        match NonZeroU32::new(return_value) {
            None => Ok(()),
            Some(code) => Err(CUerror(code)),
        }?;
        Ok(self.local.shared_memory.read_body())
    }

    pub(crate) fn remote_call_framed_in<Out: Portable + Clone>(
        &mut self,
        opcode: Opcode,
        data: impl for<'a, 'b> Serialize<Serializer<'a, 'b>>,
    ) -> Result<Out, CUerror> {
        self.remote.shared_memory.write_header(opcode as u32);
        let old_shmem = self
            .remote
            .shared_memory
            .serialize_body(&mut self.arena, &data)?;
        if let Some(mut old_shmem) = old_shmem {
            old_shmem.write_header(u32::MAX);
            old_shmem.write_buffer(self.remote.shared_memory.name.as_bytes());
        }
        unsafe { SignalObjectAndWait(*self.remote.event, *self.local.event, INFINITE, false) };
        let return_value = self.local.shared_memory.read_header();
        match NonZeroU32::new(return_value) {
            None => Ok(()),
            Some(code) => Err(CUerror(code)),
        }?;
        Ok(self.local.shared_memory.read_body())
    }

    pub(crate) fn remote_call_framed_out<Out: Archive>(
        &mut self,
        opcode: Opcode,
        data: impl for<'a, 'b> Serialize<Serializer<'a, 'b>>,
    ) -> Result<Out, CUerror>
    where
        <Out as Archive>::Archived: Deserialize<Out, Strategy<(), Failure>>,
    {
        self.remote.shared_memory.write_header(opcode as u32);
        self.remote.shared_memory.write_body(&data);
        unsafe { SignalObjectAndWait(*self.remote.event, *self.local.event, INFINITE, false) };
        let return_value = self.local.shared_memory.read_header();
        match NonZeroU32::new(return_value) {
            None => Ok(()),
            Some(code) => Err(CUerror(code)),
        }?;
        self.local.shared_memory.deserialize_body()
    }
}
