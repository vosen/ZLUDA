use cuda_types::cuda::CUerror;
use rand::distr::{Alphanumeric, SampleString};
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
use std::{env, mem};
use windows::core::{Error, PCSTR};
use windows::Win32::Foundation::*;
use windows::Win32::System::Memory::*;
use windows::Win32::System::Threading::*;
use zluda_server_common::Opcode;

struct Endpoint {
    event: HANDLE,
    event_name: String,
    view: MEMORY_MAPPED_VIEW_ADDRESS,
    shared_memory: HANDLE,
    shared_memory_name: String,
}

unsafe impl Send for Endpoint {}
unsafe impl Sync for Endpoint {}

impl Endpoint {
    const INITIAL_SHARED_MEMORY_SIZE: usize = 1024 * 1024;

    unsafe fn new() -> windows::core::Result<Self> {
        let mut shared_memory_name = Self::random_global_name();
        let shared_memory = CreateFileMappingA(
            INVALID_HANDLE_VALUE,
            None,
            PAGE_READWRITE,
            0,
            Self::INITIAL_SHARED_MEMORY_SIZE as u32,
            PCSTR(shared_memory_name.as_ptr()),
        )?;
        shared_memory_name.pop();
        let view = MapViewOfFile(
            shared_memory,
            FILE_MAP_ALL_ACCESS,
            0,
            0,
            Self::INITIAL_SHARED_MEMORY_SIZE,
        );
        if view.Value.is_null() {
            return Err(windows::core::Error::empty());
        }
        let mut event_name = Self::random_global_name();
        let event = CreateEventA(None, false, false, PCSTR(event_name.as_ptr()))?;
        event_name.pop();
        Ok(Endpoint {
            shared_memory_name,
            shared_memory,
            view,
            event_name,
            event,
        })
    }

    fn random_global_name() -> String {
        let name = Alphanumeric.sample_string(&mut rand::rng(), 32);
        format!("Global\\zluda-{name}\0")
    }
}
pub(crate) struct Server {
    client: Endpoint,
    server: Endpoint,
    _child: Child,
    arena: stumpalo::Arena,
}

impl Server {
    pub unsafe fn start() -> Result<Self, Error> {
        let client = Endpoint::new()?;
        let server = Endpoint::new()?;
        let spawn_server = |path: &PathBuf| {
            Command::new(path)
                .args([
                    &client.event_name,
                    &client.shared_memory_name,
                    &server.event_name,
                    &server.shared_memory_name,
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
            client,
            server,
            _child: child,
            arena,
        })
    }

    pub(crate) fn remote_call_zero_copy<Out: Portable + Clone>(
        &mut self,
        opcode: Opcode,
        data: impl for<'a, 'b> Serialize<Serializer<'a, 'b>>,
    ) -> Result<Out, CUerror> {
        unsafe {
            std::ptr::copy_nonoverlapping(&(opcode as u32), self.server.shared_memory.0.cast(), 1)
        };
        let arena = ScopedArena {
            arena: &mut self.arena,
        };
        let mut serializer = rkyv::ser::Serializer::new(
            SliceWriter {
                offset: 0,
                slice: unsafe {
                    std::slice::from_raw_parts_mut(
                        self.server.shared_memory.0.cast(),
                        Endpoint::INITIAL_SHARED_MEMORY_SIZE,
                    )
                },
            },
            arena,
            (),
        );
        let mut serializer = Strategy::<_, AllocError>::wrap(&mut serializer);
        let x = data.serialize(serializer);
        todo!();
        /*
        self.pipe.write_all().map_err(|_| CUerror::UNKNOWN)?;
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
         */
    }

    pub(crate) fn remote_call_framed_in<Out: Portable + Clone>(
        &mut self,
        opcode: Opcode,
        data: impl for<'a, 'b> Serialize<Serializer<'a, 'b>>,
    ) -> Result<Out, CUerror> {
        todo!()
        /*
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
         */
    }

    pub(crate) fn remote_call_framed_out<Out: Archive>(
        &mut self,
        opcode: Opcode,
        data: impl for<'a, 'b> Serialize<Serializer<'a, 'b>>,
    ) -> Result<Out, CUerror>
    where
        <Out as Archive>::Archived: Deserialize<Out, Strategy<Pool, Failure>>,
    {
        todo!()
        /*
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
        */
    }
}

pub struct SliceWriter<'a> {
    offset: usize,
    slice: &'a mut [u8],
}

impl<'a> rkyv::ser::Positional for SliceWriter<'a> {
    fn pos(&self) -> usize {
        self.offset
    }
}

impl<'a> rkyv::ser::Writer<AllocError> for SliceWriter<'a> {
    fn write(&mut self, bytes: &[u8]) -> Result<(), AllocError> {
        let end = self
            .offset
            .checked_add(bytes.len())
            .ok_or(AllocError::NotEnoughMemory {
                more_bytes: bytes.len(),
            })?;
        if end > self.slice.len() {
            return Err(AllocError::NotEnoughMemory {
                more_bytes: end - self.slice.len(),
            });
        }
        self.slice[self.offset..end].copy_from_slice(bytes);
        self.offset = end;
        Ok(())
    }
}

pub struct ScopedArena<'a> {
    arena: &'a mut stumpalo::Arena,
}

impl Drop for ScopedArena<'_> {
    fn drop(&mut self) {
        self.arena.clear();
    }
}

unsafe impl<'a> Allocator<AllocError> for ScopedArena<'a> {
    unsafe fn push_alloc(
        &mut self,
        layout: std::alloc::Layout,
    ) -> Result<std::ptr::NonNull<[u8]>, AllocError> {
        self.arena
            .try_alloc_layout(layout)
            .map_err(|_| AllocError::StumpaloError)
    }

    unsafe fn pop_alloc(
        &mut self,
        _ptr: std::ptr::NonNull<u8>,
        _layout: std::alloc::Layout,
    ) -> Result<(), AllocError> {
        Ok(())
    }
}

pub enum AllocError {
    StumpaloError,
    NotEnoughMemory { more_bytes: usize },
}

pub type Serializer<'a, 'local> =
    Strategy<rkyv::ser::Serializer<SliceWriter<'a>, ScopedArena<'local>, ()>, AllocError>;
