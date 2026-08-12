use cuda_macros::{cuda_function_declarations, generate_input_struct, generate_output_struct};
use cuda_types::cuda::*;
use rand::distr::{Alphanumeric, SampleString};
use rkyv::rancor::{Failure, Strategy};
use rkyv::rend::{u32_le, u64_le};
use rkyv::ser::Allocator;
use rkyv::{Archive, Deserialize, Portable, Serialize};
use std::{mem, ptr, slice};
use strum_macros::FromRepr;
use windows::core::{Error, Owned, PCSTR};
use windows::Win32::Foundation::*;
use windows::Win32::System::Memory::*;
use windows::Win32::System::Threading::*;

macro_rules! noop {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {};
}

macro_rules! generate_messages_inout {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            generate_input_struct!($fn_name, $($arg_id : $arg_type),*);
            generate_output_struct!($fn_name, $($arg_id : $arg_type),*);
        )*

        #[repr(u32)]
        #[derive(FromRepr)]
        #[allow(non_camel_case_types)]
        pub enum Opcode {
            System = 0,
            $(
                $fn_name,
            )*
            cuDeviceGetName,
            cuDeviceTotalMem_v2,
            cuModuleLoadData,
            cuModuleGetFunction,
            cuModuleGetGlobal_v2,
            cuMemAlloc_v2,
            cuMemcpyHtoDAsync_v2,
            cuModuleGetTexRef,
            cuLaunchKernel,
            zludaGetFunctionArgs,
            cuMemcpyDtoHAsync_v2
        }
    };
}

macro_rules! generate_messages_in {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            generate_input_struct!($fn_name, $($arg_id : $arg_type),*);
        )*
    };
}

cuda_function_declarations! {
    noop,
    generate_messages_inout <= [
        // cuCtxCreate_v2,
        cuCtxDetach,
        cuCtxGetApiVersion,
        cuCtxGetCurrent,
        cuCtxGetDevice,
        cuCtxSynchronize,
        cuDeviceComputeCapability,
        cuDeviceGet,
        cuDeviceGetAttribute,
        cuDeviceGetCount,
        cuDeviceGetProperties,
        cuDriverGetVersion,
        cuEventCreate,
        cuEventDestroy_v2,
        cuEventQuery,
        cuEventRecord,
        //cuGetExportTable,
        cuInit,
        // cuLaunchKernel,
        //cuMemAlloc_v2,
        //cuMemFreeHost,
        cuMemFree_v2,
        cuMemGetAddressRange_v2,
        //cuMemHostAlloc,
        cuMemcpyDtoD_v2,
        cuMemcpyDtoDAsync_v2,
        //cuMemcpyDtoHAsync_v2,
        //cuMemcpyHtoDAsync_v2,
        cuMemsetD8_v2,
        // cuModuleGetFunction,
        // cuModuleGetGlobal_v2,
        // cuModuleGetTexRef,
        cuStreamCreate,
        cuStreamDestroy_v2,
        cuTexRefSetAddressMode,
        cuTexRefSetAddress_v2,
        cuTexRefSetFilterMode,
        cuTexRefSetFlags,
        cuTexRefSetFormat,
        cuTexRefSetMaxAnisotropy,
        cuTexRefSetMipmapFilterMode,
        cuTexRefSetMipmapLevelBias,
        cuTexRefSetMipmapLevelClamp,
    ],
    generate_messages_in <= [
        cuDeviceGetName,
        cuDeviceTotalMem_v2
    ]
}

pub trait CudaEncode: Copy {
    type WireObject;
    fn encode(self) -> Self::WireObject;
    fn decode(o: Self::WireObject) -> Self;
}

macro_rules! encode_as_self {
    ($type_:ty) => {
        impl CudaEncode for $type_ {
            type WireObject = Self;
            fn encode(self) -> Self {
                self
            }
            fn decode(o: Self::WireObject) -> Self {
                o
            }
        }
    };
}

macro_rules! encode_as_proxy {
    ($type_:ty, $proxy_type:ty) => {
        impl CudaEncode for $type_ {
            type WireObject = $proxy_type;
            fn encode(self) -> $proxy_type {
                unsafe { std::mem::transmute::<Self, $proxy_type>(self) }
            }
            fn decode(o: Self::WireObject) -> Self {
                unsafe { std::mem::transmute::<$proxy_type, Self>(o) }
            }
        }
    };
}

macro_rules! encode_as_u32 {
    ($type_:ty) => {
        impl CudaEncode for $type_ {
            type WireObject = u32_le;
            fn encode(self) -> u32_le {
                unsafe { std::mem::transmute_copy::<Self, u32_le>(&self) }
            }
            fn decode(o: Self::WireObject) -> Self {
                unsafe { std::mem::transmute_copy::<u32_le, Self>(&o) }
            }
        }
    };
}

encode_as_self!(u8);
encode_as_self!(i8);
encode_as_self!(u32);
encode_as_self!(i32);
encode_as_self!(f32);

encode_as_proxy!(CUdevprop_v1, CUdevprop_v1_Wire);
encode_as_proxy!(CUdevice_attribute, u32_le);
encode_as_proxy!(CUfilter_mode, u32_le);
encode_as_proxy!(CUaddress_mode, u32_le);
encode_as_proxy!(CUarray_format, u32_le);

encode_as_u32!(CUcontext);
encode_as_u32!(CUdeviceptr_v2);
encode_as_u32!(CUevent);
encode_as_u32!(CUfunction);
encode_as_u32!(CUmodule);
encode_as_u32!(CUstream);
encode_as_u32!(CUtexref);

impl CudaEncode for usize {
    type WireObject = u32_le;
    fn encode(self) -> u32_le {
        (self as u32).into()
    }
    fn decode(o: Self::WireObject) -> Self {
        o.to_native() as usize
    }
}

#[repr(C)]
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct CUdevprop_v1_Wire {
    pub max_threads_per_block: ::core::ffi::c_int,
    pub max_threads_dim: [::core::ffi::c_int; 3usize],
    pub max_grid_size: [::core::ffi::c_int; 3usize],
    pub shared_mem_per_block: ::core::ffi::c_int,
    pub total_constant_memory: ::core::ffi::c_int,
    pub simd_width: ::core::ffi::c_int,
    pub mem_pitch: ::core::ffi::c_int,
    pub regs_per_block: ::core::ffi::c_int,
    pub clock_rate: ::core::ffi::c_int,
    pub texture_align: ::core::ffi::c_int,
}

impl From<CUdevprop_v1> for CUdevprop_v1_Wire {
    fn from(devprop: CUdevprop_v1) -> Self {
        Self {
            max_threads_per_block: devprop.maxThreadsPerBlock,
            max_threads_dim: devprop.maxThreadsDim,
            max_grid_size: devprop.maxGridSize,
            shared_mem_per_block: devprop.sharedMemPerBlock,
            total_constant_memory: devprop.totalConstantMemory,
            simd_width: devprop.SIMDWidth,
            mem_pitch: devprop.memPitch,
            regs_per_block: devprop.regsPerBlock,
            clock_rate: devprop.clockRate,
            texture_align: devprop.textureAlign,
        }
    }
}

impl Into<CUdevprop_v1> for CUdevprop_v1_Wire {
    fn into(self) -> CUdevprop_v1 {
        CUdevprop_v1 {
            maxThreadsPerBlock: self.max_threads_per_block,
            maxThreadsDim: self.max_threads_dim,
            maxGridSize: self.max_grid_size,
            sharedMemPerBlock: self.shared_mem_per_block,
            totalConstantMemory: self.total_constant_memory,
            SIMDWidth: self.simd_width,
            memPitch: self.mem_pitch,
            regsPerBlock: self.regs_per_block,
            clockRate: self.clock_rate,
            textureAlign: self.texture_align,
        }
    }
}

unsafe impl Portable for CUdevprop_v1_Wire {}

#[repr(C)]
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct cuDeviceGetNameOut {
    pub name: Vec<u8>,
}

#[repr(C)]
#[derive(Portable, Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct cuDeviceTotalMem_v2Out {
    pub bytes: u64_le,
}

#[repr(C)]
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct cuModuleLoadDataIn {
    pub image: Vec<u8>,
}

#[repr(C)]
#[derive(Portable, Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct cuModuleLoadDataOut {
    pub module: <CUmodule as CudaEncode>::WireObject,
}

#[repr(C)]
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct cuModuleGetFunctionIn {
    pub hmod: <CUmodule as CudaEncode>::WireObject,
    pub name: Vec<u8>,
}

#[repr(C)]
#[derive(Portable, Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct cuModuleGetFunctionOut {
    pub hfunc: <CUfunction as CudaEncode>::WireObject,
}

#[repr(C)]
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct cuModuleGetGlobal_v2In {
    pub hmod: <CUmodule as CudaEncode>::WireObject,
    pub name: Vec<u8>,
}

#[repr(C)]
#[derive(Portable, Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct cuModuleGetGlobal_v2Out {
    pub dptr: <CUdeviceptr_v2 as CudaEncode>::WireObject,
    pub bytes: u64_le,
}

#[repr(C)]
#[derive(Portable, Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct cuMemAlloc_v2In {
    pub bytesize: u32_le,
}

#[repr(C)]
#[derive(Portable, Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct cuMemAlloc_v2Out {
    pub dptr: <CUdeviceptr_v2 as CudaEncode>::WireObject,
}

#[repr(C)]
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct cuMemcpyHtoDAsync_v2In {
    pub dst_device: <CUdeviceptr_v2 as CudaEncode>::WireObject,
    pub src_host: Vec<u8>,
    pub stream: <CUstream as CudaEncode>::WireObject,
}

#[repr(C)]
#[derive(Portable, Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct cuMemcpyHtoDAsync_v2Out {}

#[repr(C)]
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct cuModuleGetTexRefIn {
    pub hmod: <CUmodule as CudaEncode>::WireObject,
    pub name: Vec<u8>,
}

#[repr(C)]
#[derive(Portable, Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct cuModuleGetTexRefOut {
    pub texref: <CUtexref as CudaEncode>::WireObject,
}

#[repr(C)]
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct cuLaunchKernelIn {
    pub f: <CUfunction as CudaEncode>::WireObject,
    pub grid_dim_x: u32_le,
    pub grid_dim_y: u32_le,
    pub grid_dim_z: u32_le,
    pub block_dim_x: u32_le,
    pub block_dim_y: u32_le,
    pub block_dim_z: u32_le,
    pub shared_mem_bytes: u32_le,
    pub stream: <CUstream as CudaEncode>::WireObject,
    pub kernel_params: Vec<Vec<u8>>,
}

#[repr(C)]
#[derive(Portable, Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct cuLaunchKernelOut {}

#[repr(C)]
#[derive(Portable, Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct zludaGetFunctionArgsIn {
    pub f: <CUfunction as CudaEncode>::WireObject,
}

#[repr(C)]
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct zludaGetFunctionArgsOut {
    pub args: Vec<dark_api::FunctionArgInfo>,
}

#[repr(C)]
#[derive(Portable, Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct cuMemcpyDtoHAsync_v2In {
    pub src_device: <CUdeviceptr_v2 as CudaEncode>::WireObject,
    pub stream: <CUstream as CudaEncode>::WireObject,
    pub byte_count: u32_le,
}

#[repr(C)]
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct cuMemcpyDtoHAsync_v2Out {
    pub dst_host: Vec<u8>,
}

pub struct OwnedView(MEMORY_MAPPED_VIEW_ADDRESS);

impl OwnedView {
    fn new(shmem: HANDLE, size: usize) -> windows::core::Result<Self> {
        let view = unsafe { MapViewOfFile(shmem, FILE_MAP_ALL_ACCESS, 0, 0, size) };
        if view.Value.is_null() {
            return Err(windows::core::Error::empty());
        }
        Ok(OwnedView(view))
    }
}

impl Drop for OwnedView {
    fn drop(&mut self) {
        let _ = unsafe { UnmapViewOfFile(self.0) };
    }
}

pub struct Endpoint {
    pub event: Owned<HANDLE>,
    pub event_name: String,
    pub shared_memory: SharedMemory,
}

unsafe impl Send for Endpoint {}
unsafe impl Sync for Endpoint {}

impl Endpoint {
    pub unsafe fn new() -> windows::core::Result<Self> {
        let shared_memory = SharedMemory::new()?;
        let mut event_name = random_global_name();
        let event = Owned::new(CreateEventA(
            None,
            false,
            false,
            PCSTR(event_name.as_ptr()),
        )?);
        event_name.pop();
        Ok(Endpoint {
            event,
            event_name,
            shared_memory,
        })
    }
}

pub struct SharedMemory {
    pub view: OwnedView,
    pub handle: Owned<HANDLE>,
    pub size: usize,
    pub name: String,
}

impl SharedMemory {
    const INITIAL_SHARED_MEMORY_SIZE: usize = 1024 * 1024;
    const OFFSET_HEADER: usize = 0;
    const OFFSET_SIZE: usize = 4;
    const OFFSET_BODY: usize = 16;

    pub unsafe fn new() -> windows::core::Result<Self> {
        let mut shared_memory_name = random_global_name();
        let shared_memory = Owned::new(CreateFileMappingA(
            INVALID_HANDLE_VALUE,
            None,
            PAGE_READWRITE,
            0,
            Self::INITIAL_SHARED_MEMORY_SIZE as u32,
            PCSTR(shared_memory_name.as_ptr()),
        )?);
        shared_memory_name.pop();
        let view = OwnedView::new(*shared_memory, Self::INITIAL_SHARED_MEMORY_SIZE)?;
        Ok(SharedMemory {
            name: shared_memory_name,
            handle: shared_memory,
            view,
            size: Self::INITIAL_SHARED_MEMORY_SIZE,
        })
    }

    fn write<T>(&mut self, offset: usize, data: &T) {
        unsafe {
            ptr::copy_nonoverlapping(data, self.view.0.Value.wrapping_byte_add(offset).cast(), 1)
        };
    }

    fn read<T>(&self, offset: usize) -> T {
        let mut body = std::mem::MaybeUninit::<T>::uninit();
        unsafe {
            ptr::copy_nonoverlapping(
                self.view.0.Value.wrapping_byte_add(offset).cast(),
                body.as_mut_ptr(),
                1,
            )
        };
        unsafe { body.assume_init() }
    }

    pub fn write_header(&mut self, header: u32) {
        self.write(Self::OFFSET_HEADER, &header);
    }

    pub fn read_header(&self) -> u32 {
        self.read(Self::OFFSET_HEADER)
    }

    pub fn write_size(&mut self, size: u32) {
        self.write(Self::OFFSET_SIZE, &size);
    }

    pub fn read_size(&self) -> u32 {
        self.read(Self::OFFSET_SIZE)
    }

    pub fn write_body<T>(&mut self, body: &T) {
        self.write(Self::OFFSET_BODY, body);
    }

    pub fn write_buffer(&mut self, body: &[u8]) {
        self.write_header(body.len() as u32);
        let output = unsafe {
            std::slice::from_raw_parts_mut(
                self.view
                    .0
                    .Value
                    .wrapping_byte_add(Self::OFFSET_BODY)
                    .cast(),
                body.len(),
            )
        };
        output.copy_from_slice(body);
    }

    pub fn read_body<T>(&self) -> T {
        self.read(Self::OFFSET_BODY)
    }

    pub fn deserialize_body<Out: Archive>(&self) -> Result<Out, CUerror>
    where
        <Out as Archive>::Archived: Deserialize<Out, Strategy<(), Failure>>,
    {
        let size = self.read_size();
        let slice = unsafe {
            slice::from_raw_parts(
                self.view
                    .0
                    .Value
                    .wrapping_byte_add(Self::OFFSET_BODY)
                    .cast(),
                size as usize,
            )
        };
        unsafe { rkyv::api::low::from_bytes_unchecked(slice) }.map_err(|_| CUerror::UNKNOWN)
    }

    pub fn serialize_body(
        &mut self,
        arena: &mut stumpalo::Arena,
        body: &impl for<'a, 'b> Serialize<Serializer<'a, 'b>>,
    ) -> Result<Option<SharedMemory>, CUerror> {
        let mut dropped_shmem = None;
        loop {
            let mut serializer_base = rkyv::ser::Serializer::new(
                SliceWriter {
                    offset: 16,
                    slice: unsafe {
                        slice::from_raw_parts_mut(
                            self.view.0.Value.cast(),
                            SharedMemory::INITIAL_SHARED_MEMORY_SIZE,
                        )
                    },
                },
                ScopedArena { arena },
                (),
            );
            let serializer = Strategy::<_, AllocError>::wrap(&mut serializer_base);
            match rkyv::api::serialize_using(body, serializer) {
                Ok(_) => break,
                Err(AllocError::Stumpalo) => {
                    return Err(CUerror::OUT_OF_MEMORY);
                }
                Err(AllocError::NotEnoughMemory { more_bytes }) => {
                    let new_size = (self.size + more_bytes).next_power_of_two();
                    let new_shared_memory_name = random_global_name();
                    let new_shared_memory = unsafe {
                        Owned::new(
                            CreateFileMappingA(
                                INVALID_HANDLE_VALUE,
                                None,
                                PAGE_READWRITE,
                                0,
                                new_size as u32,
                                PCSTR(new_shared_memory_name.as_ptr()),
                            )
                            .map_err(|_| CUerror::MAP_FAILED)?,
                        )
                    };
                    let new_view = OwnedView::new(*new_shared_memory, new_size)
                        .map_err(|_| CUerror::MAP_FAILED)?;
                    let new_shmem = SharedMemory {
                        handle: new_shared_memory,
                        view: new_view,
                        size: new_size,
                        name: new_shared_memory_name,
                    };
                    let old_shmem = mem::replace(self, new_shmem);
                    dropped_shmem.get_or_insert(old_shmem);
                }
            }
        }
        Ok(dropped_shmem)
    }
}

fn random_global_name() -> String {
    let name = Alphanumeric.sample_string(&mut rand::rng(), 32);
    format!("Local\\zluda-{name}\0")
}

#[derive(Debug)]
pub enum AllocError {
    Stumpalo,
    NotEnoughMemory { more_bytes: usize },
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
        let available = self.slice.len().saturating_sub(self.offset);
        if bytes.len() > available {
            return Err(AllocError::NotEnoughMemory {
                more_bytes: bytes.len() - available,
            });
        }
        let end = self.offset + bytes.len();
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
    ) -> Result<ptr::NonNull<[u8]>, AllocError> {
        self.arena
            .try_alloc_layout(layout)
            .map_err(|_| AllocError::Stumpalo)
    }

    unsafe fn pop_alloc(
        &mut self,
        _ptr: ptr::NonNull<u8>,
        _layout: std::alloc::Layout,
    ) -> Result<(), AllocError> {
        Ok(())
    }
}

pub type Serializer<'a, 'local> =
    Strategy<rkyv::ser::Serializer<SliceWriter<'a>, ScopedArena<'local>, ()>, AllocError>;
