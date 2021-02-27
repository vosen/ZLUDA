// Side-by-side really does three complex operations:
// * Copy unstructured CUDA arguments to structured host representation. In
//   practice, it means inspecting arguments to cuKernelLaunch(...) - firstly
//   copying all explicit arguments to the kernel, secondly copying all texrefs
// * Copy structured host representation to structured device representation.
//   This device representation is in the "space" of a different CUDA driver
// * Copy explicit device representation to structured host representation.
//   It suffices to only copy explicit arguments, texrefs are read-only

use crate::cuda_call;
use crate::log::LogEntry;
use crate::trace::serialize_array_format;
use crate::trace::DumpWriter;
use crate::trace::KernelLaunchParams;
use crate::trace::RecordedFunction;
use crate::trace::TexrefAddress;
use crate::try_get_cuda_function;
use crate::CudaDynamicFns;
use crate::{log, trace};
use cuda_types::*;
use regex::bytes::Regex;
use serde::ser::SerializeMap;
use serde::ser::SerializeSeq;
use serde::ser::SerializeStruct;
use serde::Serialize;
use serde::Serializer;
use std::alloc::Layout;
use std::borrow::Cow;
use std::cmp;
use std::collections::hash_map;
use std::collections::HashMap;
use std::ffi::CStr;
use std::ffi::CString;
use std::mem;
use std::ops::Deref;
use std::os::raw::c_void;
use std::ptr;
use std::ptr::NonNull;

// This struct encapsulates all mutable state of side-by-side CUDA driver
pub(crate) struct SideBySide {
    skip_kernel: Option<Regex>,
    dump_threshold: Option<f32>,
    // Side-by-side function table
    fn_table: CudaDynamicFns,
    // maps underlying CUDA modules to side-by-side CUDA modules
    modules: HashMap<CUmodule, CUmodule>,
    // maps underlying CUDA functions to side-by-side CUDA functions
    functions: HashMap<CUfunction, CUfunction>,
    context: CUcontext,
}

impl SideBySide {
    pub(crate) fn new(
        fn_table: CudaDynamicFns,
        fn_logger: &mut log::FunctionLogger,
        skip_kernel: Option<&String>,
        side_by_side_dump_threshold: Option<f32>,
    ) -> Option<Self> {
        let maybe_error = Self::new_impl(
            fn_table,
            fn_logger,
            skip_kernel,
            side_by_side_dump_threshold,
        );
        fn_logger.log_unwrap(maybe_error)
    }

    fn new_impl(
        mut fn_table: CudaDynamicFns,
        fn_logger: &mut log::FunctionLogger,
        skip_kernel: Option<&String>,
        side_by_side_dump_threshold: Option<f32>,
    ) -> Result<Self, LogEntry> {
        cuda_call!(fn_table.cuInit(0));
        let mut context = ptr::null_mut();
        cuda_call!(fn_table.cuCtxCreate_v2(&mut context, 0, CUdevice_v1(0)));
        cuda_call!(fn_table.cuCtxPopCurrent_v2(&mut context));
        let mut name = vec![0i8; 256];
        cuda_call!(fn_table.cuDeviceGetName(name.as_mut_ptr(), name.len() as i32, CUdevice_v1(0)));
        fn_logger.log(LogEntry::SideBySideStart(
            unsafe { CStr::from_ptr(name.as_ptr()) }.to_owned(),
        ));
        let skip_kernel = skip_kernel.map(|text| Regex::new(&text).unwrap());
        Ok(Self {
            skip_kernel,
            dump_threshold: side_by_side_dump_threshold,
            fn_table,
            context,
            modules: HashMap::new(),
            functions: HashMap::new(),
        })
    }

    pub(crate) fn get_module_and_function(
        &mut self,
        original_module: CUmodule,
        original_func: CUfunction,
        original_func_name: &str,
        original_func_text: &str,
    ) -> Result<(CUmodule, CUfunction), LogEntry> {
        let mapped_module = match self.modules.entry(original_module) {
            hash_map::Entry::Occupied(entry) => *entry.get(),
            hash_map::Entry::Vacant(entry) => {
                let mut cu_module = ptr::null_mut();
                let libcuda = &mut self.fn_table;
                cuda_call!(
                    libcuda.cuModuleLoadData(&mut cu_module, original_func_text.as_ptr() as _)
                );
                *entry.insert(cu_module)
            }
        };
        Ok(match self.functions.entry(original_func) {
            hash_map::Entry::Occupied(entry) => (mapped_module, *entry.get()),
            hash_map::Entry::Vacant(entry) => {
                let mut cu_func = ptr::null_mut();
                let mut name = original_func_name.to_string();
                name.push('\0');
                let libcuda = &mut self.fn_table;
                cuda_call!(libcuda.cuModuleGetFunction(
                    &mut cu_func,
                    mapped_module,
                    name.as_ptr() as _
                ));
                (mapped_module, *entry.insert(cu_func))
            }
        })
    }

    fn activate_context(&mut self) -> Result<SideBySideContext, LogEntry> {
        SideBySideContext::new(self)
    }
}

#[allow(non_snake_case)]
struct SideBySideContext {
    cuCtxPopCurrent_v2: extern "system" fn(*mut CUcontext) -> CUresult,
}

impl SideBySideContext {
    #[allow(non_snake_case)]
    fn new(sbs: &mut SideBySide) -> Result<Self, LogEntry> {
        let context = sbs.context;
        let fn_table = &mut sbs.fn_table;
        cuda_call!(fn_table.cuCtxPushCurrent(context));
        let cuCtxPopCurrent_v2 = try_get_cuda_function!(fn_table, cuCtxPopCurrent_v2)?;
        Ok(Self { cuCtxPopCurrent_v2 })
    }
}

#[allow(unused_must_use)]
impl Drop for SideBySideContext {
    fn drop(&mut self) {
        let mut _unused = ptr::null_mut();
        (self.cuCtxPopCurrent_v2)(&mut _unused);
    }
}

pub(crate) unsafe fn pre_kernel_launch(
    libcuda: &mut CudaDynamicFns,
    state: &mut trace::StateTracker,
    side_by_side: &mut Option<SideBySide>,
    fn_logger: &mut log::FunctionLogger,
    f: CUfunction,
    stream: CUstream,
    kernel_params: *mut *mut ::std::os::raw::c_void,
    extra: *mut *mut ::std::os::raw::c_void,
) -> Option<HostArguments> {
    let side_by_side = side_by_side.as_ref()?;
    let recorded_fn = if let Some(parsed_fn) = state.functions.get(&f) {
        parsed_fn
    } else {
        fn_logger.log(LogEntry::UnknownFunctionUse(f));
        return None;
    };
    if let Some(ref skip_filter) = side_by_side.skip_kernel {
        if skip_filter.is_match(recorded_fn.name.as_bytes()) {
            return None;
        }
    }
    let parsed_fn = recorded_fn.parsed.as_ref()?;
    let texrefs = fn_logger.log_unwrap(state.get_texrefs(recorded_fn.module))?;
    let globals = fn_logger.log_unwrap(state.get_globals(recorded_fn.module))?;
    fn_logger.log_unwrap(synchronize(libcuda))?;
    save_kernel_arguments(
        libcuda,
        fn_logger,
        stream,
        recorded_fn.module,
        texrefs,
        globals,
        &parsed_fn.explicit_arguments,
        kernel_params,
        extra,
    )
}

unsafe fn save_kernel_arguments<'a>(
    libcuda: &mut CudaDynamicFns,
    fn_logger: &mut log::FunctionLogger,
    stream: CUstream,
    module: CUmodule,
    get_texrefs: impl Iterator<Item = (&'a CStr, CUtexref, TexrefAddress)>,
    get_globals: impl Iterator<Item = (&'a CStr, CUdeviceptr)>,
    args_layout: &[Layout],
    kernel_params: *mut *mut ::std::os::raw::c_void,
    extra: *mut *mut ::std::os::raw::c_void,
) -> Option<HostArguments> {
    let mut memory_allocations = HostMemoryAllocations::new();
    if extra == ptr::null_mut() {
        let explicit_arguments = if kernel_params == ptr::null_mut() {
            Vec::new()
        } else {
            fn_logger.log_unwrap(
                args_layout
                    .iter()
                    .enumerate()
                    .map(|(index, arg_layout)| {
                        let raw_argument = *kernel_params.add(index);
                        ExplicitArgument::new(
                            libcuda,
                            stream,
                            &mut memory_allocations,
                            *arg_layout,
                            raw_argument,
                        )
                    })
                    .collect::<Result<Vec<_>, _>>(),
            )?
        };
        let images = fn_logger.log_unwrap(
            get_texrefs
                .map(|(texref_name, texref, address)| {
                    TexrefDetails::new(
                        libcuda,
                        stream,
                        texref_name,
                        texref,
                        address,
                        &mut memory_allocations,
                    )
                })
                .collect::<Result<HashMap<_, _>, _>>(),
        )?;
        let globals = fn_logger.log_unwrap(
            get_globals
                .map(|(name, devptr)| {
                    Ok((
                        name.to_owned(),
                        memory_allocations.alloc_global(libcuda, stream, module, name, devptr)?,
                    ))
                })
                .collect::<Result<HashMap<_, _>, _>>(),
        )?;
        Some(HostArguments {
            explicit_arguments,
            texrefs: images,
            globals,
            memory_allocations,
        })
    } else {
        unimplemented!()
    }
}

fn synchronize(libcuda: &mut CudaDynamicFns) -> Result<(), LogEntry> {
    cuda_call!(libcuda.cuStreamSynchronize(ptr::null_mut()));
    Ok(())
}

pub(crate) struct HostMemoryAllocations(pub(crate) HashMap<*mut c_void, HostBuffer>);

pub(crate) enum HostBuffer {
    Linear {
        data: Vec<u8>,
    },
    Array {
        data: Vec<u8>,
        descriptor: CUDA_ARRAY3D_DESCRIPTOR,
    },
}

impl HostBuffer {
    pub(crate) fn data(&self) -> &[u8] {
        match self {
            HostBuffer::Linear { data } => data,
            HostBuffer::Array { data, .. } => data,
        }
    }
    pub(crate) fn array_descriptor(&self) -> Option<&CUDA_ARRAY3D_DESCRIPTOR> {
        match self {
            HostBuffer::Linear { .. } => None,
            HostBuffer::Array { descriptor, .. } => Some(descriptor),
        }
    }
}

impl HostMemoryAllocations {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn try_alloc(
        &mut self,
        libcuda: &mut CudaDynamicFns,
        stream: CUstream,
        devptr: CUdeviceptr,
    ) -> Result<Option<BufferRef>, LogEntry> {
        Ok(get_address_range(libcuda, devptr)?
            .map(|(start, size)| {
                match self.0.entry(start.0) {
                    hash_map::Entry::Occupied(_) => {}
                    hash_map::Entry::Vacant(entry) => {
                        let fake_buffer = DeviceBuffer::Linear {
                            data: start,
                            size,
                            cuMemFree_v2: dont_drop_device_memory,
                        };
                        let buffer = BufferRef::copy(libcuda, stream, &fake_buffer)?;
                        entry.insert(HostBuffer::Linear { data: buffer });
                    }
                };
                let offset = unsafe { devptr.0.offset_from(start.0) as usize };
                Ok::<_, LogEntry>(BufferRef {
                    offset_into_buffer: offset,
                    buffer_key: start.0,
                })
            })
            .transpose()?)
    }

    fn alloc_texref(
        &mut self,
        libcuda: &mut CudaDynamicFns,
        stream: CUstream,
        address: &TexrefAddress,
    ) -> Result<BufferRef, LogEntry> {
        match address {
            TexrefAddress::OneD { pointer, .. } | TexrefAddress::TwoD { pointer, .. } => self
                .try_alloc(libcuda, stream, *pointer)
                .transpose()
                .unwrap(),
            TexrefAddress::Array { array, .. } => {
                let array: CUarray = *array;
                match self.0.entry(array as _) {
                    hash_map::Entry::Occupied(_) => {}
                    hash_map::Entry::Vacant(entry) => {
                        let mut descriptor = unsafe { mem::zeroed::<CUDA_ARRAY3D_DESCRIPTOR>() };
                        cuda_call!(libcuda.cuArray3DGetDescriptor_v2(&mut descriptor, array));
                        let fake_array = DeviceBuffer::Array {
                            array,
                            cuArrayDestroy: dont_drop_array,
                            descriptor,
                        };
                        let buffer = BufferRef::copy(libcuda, stream, &fake_array)?;
                        entry.insert(HostBuffer::Array {
                            data: buffer,
                            descriptor: descriptor,
                        });
                    }
                }
                Ok(BufferRef {
                    buffer_key: array as _,
                    offset_into_buffer: 0,
                })
            }
        }
    }

    fn alloc_global(
        &mut self,
        libcuda: &mut CudaDynamicFns,
        stream: CUstream,
        module: CUmodule,
        name: &CStr,
        devptr: CUdeviceptr_v2,
    ) -> Result<BufferRef, LogEntry> {
        let mut ptr = unsafe { mem::zeroed() };
        let mut size = unsafe { mem::zeroed() };
        cuda_call!(libcuda.cuModuleGetGlobal_v2(&mut ptr, &mut size, module, name.as_ptr()));
        if devptr != ptr {
            unreachable!()
        }
        let fake_buffer = DeviceBuffer::Linear {
            data: ptr,
            size,
            cuMemFree_v2: dont_drop_device_memory,
        };
        let buffer = BufferRef::copy(libcuda, stream, &fake_buffer)?;
        self.0.insert(ptr.0, HostBuffer::Linear { data: buffer });
        Ok(BufferRef {
            buffer_key: ptr.0,
            offset_into_buffer: 0,
        })
    }
}

extern "system" fn dont_drop_device_memory(_ptr: CUdeviceptr_v2) -> CUresult {
    CUresult::CUDA_SUCCESS
}

extern "system" fn dont_drop_array(_array: CUarray) -> CUresult {
    CUresult::CUDA_SUCCESS
}

#[derive(Serialize)]
pub(crate) struct HostArguments {
    explicit_arguments: Vec<ExplicitArgument>,
    #[serde(serialize_with = "serialize_hashmap_cstring")]
    texrefs: HashMap<CString, TexrefDetails>,
    #[serde(serialize_with = "serialize_hashmap_cstring")]
    globals: HashMap<CString, BufferRef>,
    #[serde(skip)]
    pub(crate) memory_allocations: HostMemoryAllocations,
}

fn serialize_hashmap_cstring<V, S>(
    m: &HashMap<CString, V>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    V: Serialize,
{
    let mut map = serializer.serialize_map(Some(m.len()))?;
    for (k, v) in m {
        map.serialize_entry(&k.to_string_lossy(), v)?;
    }
    map.end()
}

// This struct represents a host-side dump of a single kernel argument
// Every argument is a byte array with certain layout requirements.
// The byte array can contain pointers to buffers, which we try to identify and
// extract
#[derive(Clone, Serialize)]
pub(crate) struct ExplicitArgument {
    buffers: Vec<ExplicitArgumentBuffer>,
    data: AlignedBuffer,
}

impl ExplicitArgument {
    unsafe fn new(
        libcuda: &mut CudaDynamicFns,
        stream: CUstream,
        unique_buffers: &mut HostMemoryAllocations,
        layout: Layout,
        raw_data: *mut c_void,
    ) -> Result<Self, LogEntry> {
        let data = AlignedBuffer::new(layout, raw_data);
        let buffers = std::iter::successors(Some(0), |x| Some(x + mem::size_of::<usize>()))
            .take_while(|offset| offset + mem::size_of::<usize>() <= layout.size())
            .filter_map(|potential_pointer_offset| {
                ExplicitArgumentBuffer::try_new(
                    libcuda,
                    stream,
                    unique_buffers,
                    raw_data,
                    potential_pointer_offset,
                )
                .transpose()
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { data, buffers })
    }

    fn as_ptr(&self) -> *mut c_void {
        self.data.ptr.as_ptr() as _
    }
}

#[derive(Clone, Serialize)]
struct ExplicitArgumentBuffer {
    offset_into_argument: usize,
    buffer: BufferRef,
}

impl ExplicitArgumentBuffer {
    unsafe fn try_new(
        libcuda: &mut CudaDynamicFns,
        stream: CUstream,
        unique_buffers: &mut HostMemoryAllocations,
        data: *mut c_void,
        offset: usize,
    ) -> Result<Option<Self>, LogEntry> {
        let devptr = *(data.add(offset) as *mut CUdeviceptr_v2);
        Ok(unique_buffers
            .try_alloc(libcuda, stream, devptr)?
            .map(|buffer| Self {
                buffer,
                offset_into_argument: offset,
            }))
    }
}

#[derive(Clone, Serialize)]
struct BufferRef {
    offset_into_buffer: usize,
    #[serde(serialize_with = "serialize_mut_ptr")]
    buffer_key: *mut c_void,
}

fn serialize_mut_ptr<T, S>(x: &*mut T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u64(*x as usize as u64)
}

impl BufferRef {
    fn copy(
        libcuda: &mut CudaDynamicFns,
        stream: CUstream,
        buffer: &DeviceBuffer,
    ) -> Result<Vec<u8>, LogEntry> {
        match buffer {
            DeviceBuffer::Linear { data, size, .. } => {
                let mut buffer = vec![0u8; *size];
                cuda_call!(libcuda.cuMemcpyDtoHAsync_v2(
                    buffer.as_mut_ptr() as _,
                    *data,
                    *size,
                    stream
                ));
                Ok(buffer)
            }
            DeviceBuffer::Array {
                array, descriptor, ..
            } => {
                let mut buffer = vec![0u8; array_buffer_size(descriptor)];
                if descriptor.Height == 0 && descriptor.Depth == 0 {
                    cuda_call!(libcuda.cuMemcpyAtoH_v2(
                        buffer.as_mut_ptr() as _,
                        *array,
                        0,
                        buffer.len()
                    ));
                } else {
                    let mut memcpy_descriptor = unsafe { mem::zeroed::<CUDA_MEMCPY3D>() };
                    set_memcpy(&mut memcpy_descriptor, descriptor);
                    memcpy_descriptor.srcMemoryType = CUmemorytype::CU_MEMORYTYPE_ARRAY;
                    memcpy_descriptor.srcArray = *array;
                    memcpy_descriptor.dstMemoryType = CUmemorytype::CU_MEMORYTYPE_HOST;
                    memcpy_descriptor.dstHost = buffer.as_mut_ptr() as _;
                    memcpy_descriptor.dstPitch = memcpy_descriptor.WidthInBytes;
                    memcpy_descriptor.dstHeight = memcpy_descriptor.Height;
                    cuda_call!(libcuda.cuMemcpy3D_v2(&memcpy_descriptor));
                }
                Ok(buffer)
            }
        }
    }
}

fn set_memcpy(memcpy_descriptor: &mut CUDA_MEMCPY3D, descriptor: &CUDA_ARRAY3D_DESCRIPTOR) {
    memcpy_descriptor.WidthInBytes =
        channel_size(descriptor.Format) * descriptor.NumChannels as usize * descriptor.Width;
    memcpy_descriptor.Height = cmp::max(descriptor.Height, 1);
    memcpy_descriptor.Depth = cmp::max(descriptor.Depth, 1);
}

fn array_buffer_size(descriptor: &CUDA_ARRAY3D_DESCRIPTOR) -> usize {
    descriptor.Width
        * cmp::max(descriptor.Height, 1)
        * cmp::max(descriptor.Depth, 1)
        * descriptor.NumChannels as usize
        * channel_size(descriptor.Format)
}

fn channel_size(format: CUarray_format) -> usize {
    match format {
        CUarray_format::CU_AD_FORMAT_UNSIGNED_INT8 | CUarray_format::CU_AD_FORMAT_SIGNED_INT8 => 1,
        CUarray_format::CU_AD_FORMAT_UNSIGNED_INT16
        | CUarray_format::CU_AD_FORMAT_SIGNED_INT16
        | CUarray_format::CU_AD_FORMAT_HALF => 2,
        CUarray_format::CU_AD_FORMAT_UNSIGNED_INT32
        | CUarray_format::CU_AD_FORMAT_SIGNED_INT32
        | CUarray_format::CU_AD_FORMAT_FLOAT => 4,
        _ => unimplemented!(),
    }
}

fn get_address_range(
    libcuda: &mut CudaDynamicFns,
    devptr: CUdeviceptr,
) -> Result<Option<(CUdeviceptr, usize)>, LogEntry> {
    let mut start = CUdeviceptr_v2(ptr::null_mut());
    let mut size = 0usize;
    match libcuda.cuMemGetAddressRange_v2(&mut start, &mut size, devptr) {
        Some(cuda_types::CUresult::CUDA_SUCCESS) => Ok(Some((start, size))),
        Some(_) => return Ok(None),
        None => {
            return Err(LogEntry::NoCudaFunction(Cow::Borrowed(
                "cuMemGetAddressRange_v2",
            )))
        }
    }
}

#[derive(Serialize, Clone)]
struct TexrefDetails {
    address: TexrefAddress,
    allocation: BufferRef,
    #[serde(serialize_with = "serialize_address_mode")]
    address_mode: [CUaddress_mode; 3usize],
    #[serde(serialize_with = "serialize_filter_mode")]
    filter_mode: CUfilter_mode,
    flags: ::std::os::raw::c_uint,
    #[serde(serialize_with = "serialize_filter_mode")]
    mipmap_filter_mode: CUfilter_mode,
    mipmap_level_bias: f32,
    min_mipmap_level_clamp: f32,
    max_mipmap_level_clamp: f32,
    max_anisotropy: i32,
    #[serde(serialize_with = "serialize_array_format")]
    format: CUarray_format,
    num_channels: i32,
}

fn serialize_address_mode<S>(
    address_mode: &[CUaddress_mode; 3usize],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut slice = serializer.serialize_seq(Some(3))?;
    slice.serialize_element(&address_mode[0].0)?;
    slice.serialize_element(&address_mode[1].0)?;
    slice.serialize_element(&address_mode[2].0)?;
    slice.end()
}

fn serialize_filter_mode<S>(filter_mode: &CUfilter_mode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i32(filter_mode.0)
}

impl TexrefDetails {
    unsafe fn new(
        libcuda: &mut CudaDynamicFns,
        stream: CUstream,
        name: &CStr,
        texref: CUtexref,
        address: TexrefAddress,
        unique_buffers: &mut HostMemoryAllocations,
    ) -> Result<(CString, Self), LogEntry> {
        let allocation = unique_buffers.alloc_texref(libcuda, stream, &address)?;
        let mut address_mode = [CUaddress_mode(0); 3];
        cuda_call!(libcuda.cuTexRefGetAddressMode(&mut address_mode[0], texref, 0));
        cuda_call!(libcuda.cuTexRefGetAddressMode(&mut address_mode[1], texref, 1));
        let mut filter_mode = mem::zeroed();
        cuda_call!(libcuda.cuTexRefGetFilterMode(&mut filter_mode, texref));
        let mut flags = 0u32;
        cuda_call!(libcuda.cuTexRefGetFlags(&mut flags, texref));
        let mut mipmap_filter_mode = mem::zeroed();
        cuda_call!(libcuda.cuTexRefGetMipmapFilterMode(&mut mipmap_filter_mode, texref));
        let mut mipmap_level_bias = 0.0;
        cuda_call!(libcuda.cuTexRefGetMipmapLevelBias(&mut mipmap_level_bias, texref));
        let mut min_mipmap_level_clamp = 0.0;
        let mut max_mipmap_level_clamp = 0.0;
        cuda_call!(libcuda.cuTexRefGetMipmapLevelClamp(
            &mut min_mipmap_level_clamp,
            &mut max_mipmap_level_clamp,
            texref
        ));
        let mut format = mem::zeroed();
        let mut num_channels = 0;
        cuda_call!(libcuda.cuTexRefGetFormat(&mut format, &mut num_channels, texref));
        let mut max_anisotropy = 0;
        cuda_call!(libcuda.cuTexRefGetMaxAnisotropy(&mut max_anisotropy, texref));
        Ok((
            name.to_owned(),
            TexrefDetails {
                address,
                allocation,
                address_mode,
                filter_mode,
                flags,
                mipmap_filter_mode,
                mipmap_level_bias,
                min_mipmap_level_clamp,
                max_mipmap_level_clamp,
                max_anisotropy,
                format,
                num_channels,
            },
        ))
    }
}

struct AlignedBuffer {
    layout: Layout,
    ptr: NonNull<u8>,
}

impl AlignedBuffer {
    fn new(layout: Layout, data: *mut c_void) -> Self {
        let ptr = unsafe { std::alloc::alloc(layout) };
        unsafe { std::ptr::copy_nonoverlapping::<u8>(data as _, ptr, layout.size()) };
        let ptr = NonNull::new(ptr).unwrap();
        Self { layout, ptr }
    }
}

impl Clone for AlignedBuffer {
    fn clone(&self) -> Self {
        Self::new(self.layout, self.ptr.as_ptr() as _)
    }
}

impl Deref for AlignedBuffer {
    type Target = [u8];

    fn deref<'a>(&'a self) -> &'a Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.layout.size()) }
    }
}

impl Drop for AlignedBuffer {
    fn drop(&mut self) {
        unsafe { std::alloc::dealloc(self.ptr.as_ptr(), self.layout) }
    }
}

impl Serialize for AlignedBuffer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut struct_ = serializer.serialize_struct("AlignedBuffer", 2)?;
        struct_.serialize_field(
            "layout",
            &LayoutSerialized {
                size: self.layout.size(),
                align: self.layout.align(),
            },
        )?;
        struct_.serialize_field("ptr", self.deref())?;
        struct_.end()
    }
}

#[derive(Serialize)]
#[serde(rename = "Layout")]
struct LayoutSerialized {
    size: usize,
    align: usize,
}

pub(crate) unsafe fn post_kernel_launch(
    libcuda: &mut CudaDynamicFns,
    state: &mut trace::StateTracker,
    side_by_side: &mut Option<SideBySide>,
    fn_logger: &mut log::FunctionLogger,
    pre_kernel_args: Option<HostArguments>,
    f: CUfunction,
    grid_dim_x: ::std::os::raw::c_uint,
    grid_dim_y: ::std::os::raw::c_uint,
    grid_dim_z: ::std::os::raw::c_uint,
    block_dim_x: ::std::os::raw::c_uint,
    block_dim_y: ::std::os::raw::c_uint,
    block_dim_z: ::std::os::raw::c_uint,
    shared_mem_bytes: ::std::os::raw::c_uint,
    stream: CUstream,
    kernel_params: *mut *mut ::std::os::raw::c_void,
    extra: *mut *mut ::std::os::raw::c_void,
) -> Option<()> {
    let pre_args = &pre_kernel_args?;
    let side_by_side = side_by_side.as_mut()?;
    let recorded_fn = if let Some(parsed_fn) = state.functions.get(&f) {
        parsed_fn
    } else {
        fn_logger.log(LogEntry::UnknownFunctionUse(f));
        return None;
    };
    let parsed_fn = recorded_fn.parsed.as_ref()?;
    let texrefs = fn_logger.log_unwrap(state.get_texrefs(recorded_fn.module))?;
    let globals = fn_logger.log_unwrap(state.get_globals(recorded_fn.module))?;
    fn_logger.log_unwrap(synchronize(libcuda))?;
    let post_kernel_args_original = save_kernel_arguments(
        libcuda,
        fn_logger,
        stream,
        recorded_fn.module,
        texrefs,
        globals,
        &parsed_fn.explicit_arguments,
        kernel_params,
        extra,
    )?;
    let _ctx = fn_logger.log_unwrap(side_by_side.activate_context())?;
    let post_kernel_args_side_by_side = launch_kernel_and_get_results(
        side_by_side,
        recorded_fn,
        fn_logger,
        pre_args,
        f,
        grid_dim_x,
        grid_dim_y,
        grid_dim_z,
        block_dim_x,
        block_dim_y,
        block_dim_z,
        shared_mem_bytes,
    )?;
    let launch_params = KernelLaunchParams {
        gridDimX: grid_dim_x,
        gridDimY: grid_dim_y,
        gridDimZ: grid_dim_z,
        blockDimX: block_dim_x,
        blockDimY: block_dim_y,
        blockDimZ: block_dim_z,
        sharedMemBytes: shared_mem_bytes,
    };
    if parsed_fn.text.len() != 1 {
        todo!("Linking not implemented");
    }
    compare_and_notify_if_mismatch(
        fn_logger,
        &recorded_fn.name,
        &state.writer,
        &post_kernel_args_original,
        &post_kernel_args_side_by_side,
        side_by_side.dump_threshold,
        &parsed_fn.text[0],
        pre_args,
        launch_params,
    );
    Some(())
}

fn launch_kernel_and_get_results(
    side_by_side: &mut SideBySide,
    recorded_fn: &RecordedFunction,
    fn_logger: &mut log::FunctionLogger,
    host_args: &HostArguments,
    original_func: CUfunction,
    grid_dim_x: ::std::os::raw::c_uint,
    grid_dim_y: ::std::os::raw::c_uint,
    grid_dim_z: ::std::os::raw::c_uint,
    block_dim_x: ::std::os::raw::c_uint,
    block_dim_y: ::std::os::raw::c_uint,
    block_dim_z: ::std::os::raw::c_uint,
    shared_mem_bytes: ::std::os::raw::c_uint,
) -> Option<HashMap<*mut c_void, Vec<u8>>> {
    let parsed_fn = recorded_fn.parsed.as_ref()?;
    if parsed_fn.text.len() != 1 {
        todo!("Linking not implemented");
    }
    let (side_by_side_module, side_by_side_func) =
        fn_logger.log_unwrap(side_by_side.get_module_and_function(
            recorded_fn.module,
            original_func,
            &recorded_fn.name,
            &parsed_fn.text[0],
        ))?;
    let side_by_side_fn_table = &mut side_by_side.fn_table;
    let device_arguments =
        fn_logger.log_unwrap(DeviceArguments::new(side_by_side_fn_table, &host_args))?;
    fn_logger
        .log_unwrap(device_arguments.bind_globals(side_by_side_fn_table, side_by_side_module))?;
    let mut kernel_params = fn_logger.log_unwrap(
        device_arguments.bind_texrefs_get_arguments(side_by_side_fn_table, side_by_side_module),
    )?;
    fn_logger.log_unwrap(launch_kernel(
        side_by_side_fn_table,
        side_by_side_func,
        grid_dim_x,
        grid_dim_y,
        grid_dim_z,
        block_dim_x,
        block_dim_y,
        block_dim_z,
        shared_mem_bytes,
        &mut kernel_params,
    ))?;
    fn_logger.log_unwrap(
        device_arguments.copy_allocations_to_host(side_by_side_fn_table, ptr::null_mut()),
    )
}

struct DeviceArguments {
    explicit_arguments: Vec<ExplicitArgument>,
    texrefs: HashMap<CString, TexrefDetails>,
    globals: HashMap<CString, BufferRef>,
    memory_allocations: HashMap<*mut c_void, DeviceBuffer>,
}

impl DeviceArguments {
    fn new(libcuda: &mut CudaDynamicFns, host: &HostArguments) -> Result<Self, LogEntry> {
        let memory_allocations = host
            .memory_allocations
            .0
            .iter()
            .map(|(key, buff)| Ok((key.clone(), DeviceBuffer::new(libcuda, buff)?)))
            .collect::<Result<_, LogEntry>>()?;
        let mut explicit_arguments = host.explicit_arguments.clone();
        Self::adjust_device_pointers(&mut explicit_arguments, &memory_allocations);
        let texrefs = host.texrefs.clone();
        let globals = host.globals.clone();
        Ok(DeviceArguments {
            explicit_arguments,
            texrefs,
            globals,
            memory_allocations,
        })
    }

    fn adjust_device_pointers(
        explicit_arguments: &mut Vec<ExplicitArgument>,
        memory_allocations: &HashMap<*mut c_void, DeviceBuffer>,
    ) {
        for arg in explicit_arguments.iter_mut() {
            for buffer in &arg.buffers {
                match &memory_allocations[&buffer.buffer.buffer_key] {
                    DeviceBuffer::Linear { data, .. } => {
                        let start = *data;
                        let actual_ptr = CUdeviceptr_v2(unsafe {
                            start.0.add(buffer.buffer.offset_into_buffer)
                        });
                        unsafe {
                            std::ptr::write::<CUdeviceptr>(
                                arg.data.ptr.as_ptr().add(buffer.offset_into_argument) as _,
                                actual_ptr,
                            )
                        };
                    }
                    DeviceBuffer::Array { .. } => unimplemented!(),
                }
            }
        }
    }

    fn bind_texrefs_get_arguments(
        &self,
        libcuda: &mut CudaDynamicFns,
        module: CUmodule,
    ) -> Result<Vec<*mut c_void>, LogEntry> {
        for (name, texref_details) in self.texrefs.iter() {
            unsafe {
                Self::bind_texref(
                    libcuda,
                    module,
                    name,
                    texref_details,
                    &self.memory_allocations,
                )?
            };
        }
        Ok(self
            .explicit_arguments
            .iter()
            .map(|arg| arg.as_ptr())
            .collect::<Vec<_>>())
    }

    fn bind_globals(&self, libcuda: &mut CudaDynamicFns, module: CUmodule) -> Result<(), LogEntry> {
        for (name, buffer_ref) in self.globals.iter() {
            let mut device_ptr = unsafe { mem::zeroed() };
            let mut size = 0;
            cuda_call!(libcuda.cuModuleGetGlobal_v2(
                &mut device_ptr,
                &mut size,
                module,
                name.as_ptr()
            ));
            let buffer = self.memory_allocations.get(&buffer_ref.buffer_key).unwrap();
            if let DeviceBuffer::Linear { data, .. } = buffer {
                let src = CUdeviceptr_v2(unsafe { data.0.add(buffer_ref.offset_into_buffer) });
                cuda_call!(libcuda.cuMemcpyDtoD_v2(device_ptr, src, size));
            } else {
                unreachable!()
            }
        }
        Ok(())
    }

    unsafe fn bind_texref(
        libcuda: &mut CudaDynamicFns,
        module: CUmodule,
        name: &CStr,
        texref_details: &TexrefDetails,
        device_allocations: &HashMap<*mut c_void, DeviceBuffer>,
    ) -> Result<(), LogEntry> {
        let mut texref = ptr::null_mut();
        cuda_call!(libcuda.cuModuleGetTexRef(&mut texref, module, name.as_ptr()));
        let dev_buffer = device_allocations
            .get(&texref_details.allocation.buffer_key)
            .unwrap();
        cuda_call!(libcuda.cuTexRefSetFormat(
            texref,
            texref_details.format,
            texref_details.num_channels as i32
        ));
        cuda_call!(libcuda.cuTexRefSetAddressMode(texref, 0, texref_details.address_mode[0]));
        cuda_call!(libcuda.cuTexRefSetAddressMode(texref, 1, texref_details.address_mode[1]));
        cuda_call!(libcuda.cuTexRefSetFilterMode(texref, texref_details.filter_mode));
        cuda_call!(libcuda.cuTexRefSetFlags(texref, texref_details.flags));
        cuda_call!(libcuda.cuTexRefSetMipmapFilterMode(texref, texref_details.mipmap_filter_mode));
        cuda_call!(libcuda.cuTexRefSetMipmapLevelBias(texref, texref_details.mipmap_level_bias));
        cuda_call!(libcuda.cuTexRefSetMipmapLevelClamp(
            texref,
            texref_details.min_mipmap_level_clamp,
            texref_details.max_mipmap_level_clamp
        ));
        cuda_call!(libcuda.cuTexRefSetMaxAnisotropy(texref, texref_details.max_anisotropy as u32));
        match (dev_buffer, texref_details.address) {
            (DeviceBuffer::Linear { data, .. }, TexrefAddress::OneD { bytes, .. }) => {
                let devptr =
                    CUdeviceptr_v2(data.0.add(texref_details.allocation.offset_into_buffer));
                cuda_call!(libcuda.cuTexRefSetAddress_v2(ptr::null_mut(), texref, devptr, bytes));
            }
            (
                DeviceBuffer::Linear { data, .. },
                TexrefAddress::TwoD {
                    width,
                    height,
                    format,
                    channels,
                    pitch,
                    ..
                },
            ) => {
                let devptr =
                    CUdeviceptr_v2(data.0.add(texref_details.allocation.offset_into_buffer));
                cuda_call!(libcuda.cuTexRefSetAddress2D_v3(
                    texref,
                    &CUDA_ARRAY_DESCRIPTOR {
                        Width: width,
                        Height: height,
                        Format: format,
                        NumChannels: channels,
                    },
                    devptr,
                    pitch
                ));
            }
            (DeviceBuffer::Array { array, .. }, TexrefAddress::Array { flags, .. }) => {
                cuda_call!(libcuda.cuTexRefSetArray(texref, *array, flags));
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    fn copy_allocations_to_host(
        &self,
        libcuda: &mut CudaDynamicFns,
        stream: CUstream,
    ) -> Result<HashMap<*mut c_void, Vec<u8>>, LogEntry> {
        self.memory_allocations
            .iter()
            .map(|(key, buffer)| Ok((*key, BufferRef::copy(libcuda, stream, buffer)?)))
            .collect::<Result<HashMap<_, _>, LogEntry>>()
    }
}

#[allow(non_snake_case)]
pub(crate) enum DeviceBuffer {
    Linear {
        data: CUdeviceptr_v2,
        size: usize,
        cuMemFree_v2: extern "system" fn(CUdeviceptr_v2) -> CUresult,
    },
    Array {
        array: CUarray,
        descriptor: CUDA_ARRAY3D_DESCRIPTOR,
        cuArrayDestroy: extern "system" fn(CUarray) -> CUresult,
    },
}

impl DeviceBuffer {
    #[allow(non_snake_case)]
    fn new(libcuda: &mut CudaDynamicFns, host: &HostBuffer) -> Result<Self, LogEntry> {
        match host {
            HostBuffer::Linear { data: host_data } => {
                let cuMemFree_v2 = try_get_cuda_function!(libcuda, cuMemFree_v2)?;
                let mut data = CUdeviceptr_v2(ptr::null_mut());
                cuda_call!(libcuda.cuMemAlloc_v2(&mut data, host_data.len()));
                let result = DeviceBuffer::Linear {
                    data,
                    size: host_data.len(),
                    cuMemFree_v2,
                };
                cuda_call!(libcuda.cuMemcpyHtoD_v2(data, host_data.as_ptr() as _, host_data.len()));
                Ok(result)
            }
            HostBuffer::Array { data, descriptor } => {
                let cuArrayDestroy = try_get_cuda_function!(libcuda, cuArrayDestroy)?;
                let mut array = unsafe { mem::zeroed::<CUarray>() };
                cuda_call!(libcuda.cuArray3DCreate_v2(&mut array, descriptor));
                let result = DeviceBuffer::Array {
                    array: array,
                    descriptor: *descriptor,
                    cuArrayDestroy,
                };
                if descriptor.Height == 0 && descriptor.Depth == 0 {
                    cuda_call!(libcuda.cuMemcpyHtoA_v2(array, 0, data.as_ptr() as _, data.len()));
                } else {
                    let mut memcpy_descriptor = unsafe { mem::zeroed::<CUDA_MEMCPY3D>() };
                    set_memcpy(&mut memcpy_descriptor, descriptor);
                    memcpy_descriptor.srcMemoryType = CUmemorytype::CU_MEMORYTYPE_HOST;
                    memcpy_descriptor.srcHost = data.as_ptr() as _;
                    memcpy_descriptor.srcPitch = memcpy_descriptor.WidthInBytes;
                    memcpy_descriptor.srcHeight = memcpy_descriptor.Height;
                    memcpy_descriptor.dstMemoryType = CUmemorytype::CU_MEMORYTYPE_ARRAY;
                    memcpy_descriptor.dstArray = array;
                    cuda_call!(libcuda.cuMemcpy3D_v2(&memcpy_descriptor));
                }
                Ok(result)
            }
        }
    }
}

impl Drop for DeviceBuffer {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        match self {
            DeviceBuffer::Linear {
                data, cuMemFree_v2, ..
            } => {
                (cuMemFree_v2)(*data);
            }
            DeviceBuffer::Array {
                array,
                cuArrayDestroy,
                ..
            } => {
                (cuArrayDestroy)(*array);
            }
        }
    }
}

fn launch_kernel(
    fn_table: &mut CudaDynamicFns,
    side_by_side_func: CUfunction,
    grid_dim_x: u32,
    grid_dim_y: u32,
    grid_dim_z: u32,
    block_dim_x: u32,
    block_dim_y: u32,
    block_dim_z: u32,
    shared_mem_bytes: u32,
    kernel_params: &mut [*mut c_void],
) -> Result<(), LogEntry> {
    cuda_call!(fn_table.cuLaunchKernel(
        side_by_side_func,
        grid_dim_x,
        grid_dim_y,
        grid_dim_z,
        block_dim_x,
        block_dim_y,
        block_dim_z,
        shared_mem_bytes,
        ptr::null_mut(),
        kernel_params.as_mut_ptr(),
        ptr::null_mut()
    ));
    cuda_call!(fn_table.cuStreamSynchronize(ptr::null_mut()));
    Ok(())
}

fn compare_and_notify_if_mismatch(
    fn_logger: &mut log::FunctionLogger,
    name: &str,
    dump_writer: &DumpWriter,
    output_original: &HostArguments,
    output_side_by_side: &HashMap<*mut c_void, Vec<u8>>,
    side_by_side_dump_threshold: Option<f32>,
    module: &str,
    input: &HostArguments,
    parameters: KernelLaunchParams,
) {
    let mut original = output_original
        .memory_allocations
        .0
        .iter()
        .collect::<Vec<_>>();
    original.sort_by_key(|(k, _)| **k);
    let mut side_by_side = output_side_by_side.iter().collect::<Vec<_>>();
    side_by_side.sort_by_key(|(k, _)| **k);
    let mut should_dump = false;
    for ((devptr1, original_buf), (devptr2, side_by_side_buff)) in
        original.iter().zip(side_by_side.iter())
    {
        if **devptr1 != **devptr2 {
            unreachable!()
        }
        let diff_count = unsafe { diff_count(original_buf.data(), side_by_side_buff) };
        let total_count = original_buf.data().len();
        if diff_count > 0 {
            fn_logger.log(LogEntry::ArgumentMismatch {
                devptr: **devptr1,
                diff_count: diff_count as usize,
                total_count,
            })
        }
        if let Some(dump_threshold) = side_by_side_dump_threshold {
            let diff_percentage = (diff_count as f64 / total_count as f64) * 100f64;
            if diff_percentage >= dump_threshold as f64 {
                should_dump = true;
            }
        }
    }
    if should_dump {
        dump_writer.save_kernel_launch(fn_logger, name, module, parameters, input, output_original)
    }
}

unsafe fn diff_count(mut b1: &[u8], mut b2: &[u8]) -> u64 {
    use std::arch::x86_64::*;
    let mut counter = _mm256_setzero_si256();
    let increment = _mm256_set1_epi8(1);
    while b1.len() >= 32 {
        let values1 = _mm256_loadu_si256(b1.as_ptr() as _);
        let values2 = _mm256_loadu_si256(b2.as_ptr() as _);
        let diff = _mm256_cmpeq_epi8(values1, values2);
        let masked = _mm256_add_epi8(diff, increment); // overflow
        let mut shifted = masked;
        // iteration 1
        //shifted = _mm256_srli_si256::<0>(shifted);
        let low_half = _mm256_castsi256_si128(shifted);
        let high_half = _mm256_extractf128_si256::<1>(shifted);
        let extended_low = _mm256_cvtepu8_epi64(low_half);
        let extended_high = _mm256_cvtepu8_epi64(high_half);
        counter = _mm256_add_epi64(counter, _mm256_add_epi64(extended_low, extended_high));
        // iteration 2
        shifted = _mm256_srli_si256::<4>(shifted);
        let low_half = _mm256_castsi256_si128(shifted);
        let high_half = _mm256_extractf128_si256::<1>(shifted);
        let extended_low = _mm256_cvtepu8_epi64(low_half);
        let extended_high = _mm256_cvtepu8_epi64(high_half);
        counter = _mm256_add_epi64(counter, _mm256_add_epi64(extended_low, extended_high));
        // iteration 3
        shifted = _mm256_srli_si256::<4>(shifted);
        let low_half = _mm256_castsi256_si128(shifted);
        let high_half = _mm256_extractf128_si256::<1>(shifted);
        let extended_low = _mm256_cvtepu8_epi64(low_half);
        let extended_high = _mm256_cvtepu8_epi64(high_half);
        counter = _mm256_add_epi64(counter, _mm256_add_epi64(extended_low, extended_high));
        // iteration 4
        shifted = _mm256_srli_si256::<4>(shifted);
        let low_half = _mm256_castsi256_si128(shifted);
        let high_half = _mm256_extractf128_si256::<1>(shifted);
        let extended_low = _mm256_cvtepu8_epi64(low_half);
        let extended_high = _mm256_cvtepu8_epi64(high_half);
        counter = _mm256_add_epi64(counter, _mm256_add_epi64(extended_low, extended_high));
        b1 = &b1[32..];
        b2 = &b2[32..];
    }
    // No horizontal add for 64 bit elements
    let counter_low_half = _mm256_castsi256_si128(counter);
    let counter_high_half = _mm256_extractf128_si256::<1>(counter);
    let counter_128 = _mm_add_epi64(counter_low_half, counter_high_half);
    let elm_0 = _mm_extract_epi64::<0>(counter_128) as u64;
    let elm_1 = _mm_extract_epi64::<1>(counter_128) as u64;
    diff_count_scalar(b1, b2) + elm_0 + elm_1
}

fn diff_count_scalar(b1: &[u8], b2: &[u8]) -> u64 {
    b1.iter()
        .copied()
        .zip(b2.iter().copied())
        .filter(|(x, y)| x != y)
        .count() as u64
}

#[cfg(test)]
mod tests {
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    use super::{diff_count, diff_count_scalar};

    #[test]
    fn diff_count_is_correct() {
        let count = 506;
        let mut rng = ChaCha8Rng::seed_from_u64(0x82edbf28bf683468);
        let mut vec1 = Vec::new();
        let mut vec2 = Vec::new();
        for _ in 0..count {
            let value = rng.gen::<u8>();
            vec1.push(value);
            if rng.gen::<bool>() {
                vec2.push(value);
            } else {
                vec2.push(rng.gen())
            }
        }
        unsafe { assert_eq!(diff_count(&vec1, &vec2), diff_count_scalar(&vec1, &vec2)) }
    }
}
