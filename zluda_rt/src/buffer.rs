use crate::{
    context::{self, Context, ContextData},
    hip, null_check, null_unwrap, null_unwrap_mut, AlignedBuffer, MaybeWeakRefMut, OptixCell,
    OptixObjectData, TypeTag,
};
use hip_runtime_sys::*;
use optix_types::*;
use std::{
    alloc::Layout,
    ffi::c_void,
    mem, ptr,
    rc::{Rc, Weak},
};

pub(crate) type Buffer = *const OptixCell<BufferData>;

#[repr(C)]
pub(crate) struct DeviceBuffer {
    pub(crate) pointer: hipDeviceptr_t,
    pub(crate) width: u64,
    pub(crate) height: u64,
}

struct BufferAllocation {
    size: u64,
    pointer: hipDeviceptr_t,
    host_buffer: Option<AlignedBuffer>,
}

impl Drop for BufferAllocation {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        if self.pointer.0 != ptr::null_mut() {
            unsafe { hipFree(self.pointer.0) };
        }
    }
}

impl BufferAllocation {
    fn empty() -> Self {
        BufferAllocation {
            size: 0,
            pointer: hipDeviceptr_t(ptr::null_mut()),
            host_buffer: None,
        }
    }

    fn new(this: Option<Self>, meta: BufferMetadata) -> Result<(Self, bool), RTresult> {
        let size = meta.byte_size();
        if let Some(this) = this {
            if size == this.size {
                return Ok((this, false));
            }
        }
        let pointer = Self::hip_allocate(meta.allocation_size())?;
        Ok((
            Self {
                size,
                pointer,
                host_buffer: None,
            },
            true,
        ))
    }

    fn hip_allocate(size: u64) -> Result<hipDeviceptr_t, RTresult> {
        Ok(if size > 0 {
            let dev_ptr = hip::malloc(size as usize)
                .map_err(|_| RTresult::RT_ERROR_MEMORY_ALLOCATION_FAILED)?;
            hip::zero_fill(dev_ptr, size as usize)
                .map_err(|_| RTresult::RT_ERROR_MEMORY_ALLOCATION_FAILED)?;
            dev_ptr
        } else {
            hipDeviceptr_t(ptr::null_mut())
        })
    }

    fn map(&mut self, meta: BufferMetadata) -> Result<*mut c_void, RTresult> {
        match self.host_buffer {
            Some(_) => return Err(RTresult::RT_ERROR_ALREADY_MAPPED),
            None => {
                let layout = unsafe {
                    Layout::from_size_align_unchecked(
                        self.size as usize,
                        BufferMetadata::alignment(meta.format)?,
                    )
                };
                let buffer = if self.pointer.0 != ptr::null_mut() {
                    AlignedBuffer::from_hip(layout, self.pointer)?
                } else {
                    AlignedBuffer::new(Layout::new::<u8>())
                };
                let result = buffer.as_ptr();
                self.host_buffer = Some(buffer);
                Ok(result as _)
            }
        }
    }

    fn unmap(&mut self) -> Result<(), RTresult> {
        match &self.host_buffer {
            None => return Err(RTresult::RT_ERROR_INVALID_VALUE),
            Some(buffer) => {
                if self.pointer.0 != ptr::null_mut() {
                    hip! {hipMemcpyHtoD(self.pointer, buffer.as_ptr(), buffer.size), RT_ERROR_MEMORY_ALLOCATION_FAILED};
                }
                self.host_buffer = None;
                Ok(())
            }
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) struct BufferMetadata {
    pub(crate) format: RTformatSafe,
    pub(crate) element_size: u64,
    pub(crate) width: u64,
    pub(crate) height: u64,
}

impl BufferMetadata {
    pub(crate) fn byte_size(&self) -> u64 {
        self.width * self.height.max(1) * self.element_size
    }

    // We allocate one extra element for Arnold
    // Arnold 7.1.4.1 has a buggy binary search which
    // returns an element one past the end
    pub(crate) fn allocation_size(&self) -> u64 {
        (self.width + 1) * self.height.max(1) * self.element_size
    }

    fn dimensions(&self) -> u32 {
        if self.height == 0 {
            1
        } else {
            2
        }
    }

    fn size(&self, dim: u32) -> u64 {
        match dim {
            0 => self.width,
            1 => self.height,
            _ => 0,
        }
    }

    pub(crate) fn depth(&self) -> usize {
        0
    }

    fn element_size(format: RTformatSafe) -> Result<usize, RTresult> {
        Ok(match format {
            RTformatSafe::RT_FORMAT_UNKNOWN => 0,
            RTformatSafe::RT_FORMAT_USER => 0,
            RTformatSafe::RT_FORMAT_FLOAT => mem::size_of::<f32>(),
            RTformatSafe::RT_FORMAT_FLOAT2 => mem::size_of::<f32>() * 2,
            RTformatSafe::RT_FORMAT_FLOAT3 => mem::size_of::<f32>() * 3,
            RTformatSafe::RT_FORMAT_FLOAT4 => mem::size_of::<f32>() * 4,
            RTformatSafe::RT_FORMAT_BYTE => mem::size_of::<i8>(),
            RTformatSafe::RT_FORMAT_BYTE2 => mem::size_of::<i8>() * 2,
            RTformatSafe::RT_FORMAT_BYTE3 => mem::size_of::<i8>() * 3,
            RTformatSafe::RT_FORMAT_BYTE4 => mem::size_of::<i8>() * 4,
            RTformatSafe::RT_FORMAT_UNSIGNED_BYTE => mem::size_of::<u8>(),
            RTformatSafe::RT_FORMAT_UNSIGNED_BYTE2 => mem::size_of::<u8>() * 2,
            RTformatSafe::RT_FORMAT_UNSIGNED_BYTE3 => mem::size_of::<u8>() * 3,
            RTformatSafe::RT_FORMAT_UNSIGNED_BYTE4 => mem::size_of::<u8>() * 4,
            RTformatSafe::RT_FORMAT_SHORT => mem::size_of::<i16>(),
            RTformatSafe::RT_FORMAT_SHORT2 => mem::size_of::<i16>() * 2,
            RTformatSafe::RT_FORMAT_SHORT3 => mem::size_of::<i16>() * 3,
            RTformatSafe::RT_FORMAT_SHORT4 => mem::size_of::<i16>() * 4,
            RTformatSafe::RT_FORMAT_UNSIGNED_SHORT => mem::size_of::<u16>(),
            RTformatSafe::RT_FORMAT_UNSIGNED_SHORT2 => mem::size_of::<u16>() * 2,
            RTformatSafe::RT_FORMAT_UNSIGNED_SHORT3 => mem::size_of::<u16>() * 3,
            RTformatSafe::RT_FORMAT_UNSIGNED_SHORT4 => mem::size_of::<u16>() * 4,
            RTformatSafe::RT_FORMAT_INT => mem::size_of::<i32>(),
            RTformatSafe::RT_FORMAT_INT2 => mem::size_of::<i32>() * 2,
            RTformatSafe::RT_FORMAT_INT3 => mem::size_of::<i32>() * 3,
            RTformatSafe::RT_FORMAT_INT4 => mem::size_of::<i32>() * 4,
            RTformatSafe::RT_FORMAT_UNSIGNED_INT => mem::size_of::<u32>(),
            RTformatSafe::RT_FORMAT_UNSIGNED_INT2 => mem::size_of::<u32>() * 2,
            RTformatSafe::RT_FORMAT_UNSIGNED_INT3 => mem::size_of::<u32>() * 3,
            RTformatSafe::RT_FORMAT_UNSIGNED_INT4 => mem::size_of::<u32>() * 4,
            RTformatSafe::RT_FORMAT_HALF => 2,
            RTformatSafe::RT_FORMAT_HALF2 => 2 * 2,
            RTformatSafe::RT_FORMAT_HALF3 => 2 * 3,
            RTformatSafe::RT_FORMAT_HALF4 => 2 * 4,
            RTformatSafe::RT_FORMAT_LONG_LONG => mem::size_of::<i64>(),
            RTformatSafe::RT_FORMAT_LONG_LONG2 => mem::size_of::<i64>() * 2,
            RTformatSafe::RT_FORMAT_LONG_LONG3 => mem::size_of::<i64>() * 3,
            RTformatSafe::RT_FORMAT_LONG_LONG4 => mem::size_of::<i64>() * 4,
            RTformatSafe::RT_FORMAT_UNSIGNED_LONG_LONG => mem::size_of::<u64>(),
            RTformatSafe::RT_FORMAT_UNSIGNED_LONG_LONG2 => mem::size_of::<u64>() * 2,
            RTformatSafe::RT_FORMAT_UNSIGNED_LONG_LONG3 => mem::size_of::<u64>() * 3,
            RTformatSafe::RT_FORMAT_UNSIGNED_LONG_LONG4 => mem::size_of::<u64>() * 4,
            RTformatSafe::RT_FORMAT_BUFFER_ID => 4,
            RTformatSafe::RT_FORMAT_PROGRAM_ID => 4,
            RTformatSafe::RT_FORMAT_UNSIGNED_BC1 => return Err(RTresult::RT_ERROR_NOT_SUPPORTED),
            RTformatSafe::RT_FORMAT_UNSIGNED_BC2 => return Err(RTresult::RT_ERROR_NOT_SUPPORTED),
            RTformatSafe::RT_FORMAT_UNSIGNED_BC3 => return Err(RTresult::RT_ERROR_NOT_SUPPORTED),
            RTformatSafe::RT_FORMAT_UNSIGNED_BC4 => return Err(RTresult::RT_ERROR_NOT_SUPPORTED),
            RTformatSafe::RT_FORMAT_BC4 => return Err(RTresult::RT_ERROR_NOT_SUPPORTED),
            RTformatSafe::RT_FORMAT_UNSIGNED_BC5 => return Err(RTresult::RT_ERROR_NOT_SUPPORTED),
            RTformatSafe::RT_FORMAT_BC5 => return Err(RTresult::RT_ERROR_NOT_SUPPORTED),
            RTformatSafe::RT_FORMAT_UNSIGNED_BC6H => return Err(RTresult::RT_ERROR_NOT_SUPPORTED),
            RTformatSafe::RT_FORMAT_BC6H => return Err(RTresult::RT_ERROR_NOT_SUPPORTED),
            RTformatSafe::RT_FORMAT_UNSIGNED_BC7 => return Err(RTresult::RT_ERROR_NOT_SUPPORTED),
        })
    }

    fn alignment(format: RTformatSafe) -> Result<usize, RTresult> {
        Ok(match format {
            RTformatSafe::RT_FORMAT_UNKNOWN => 1,
            RTformatSafe::RT_FORMAT_USER => 1,
            RTformatSafe::RT_FORMAT_BYTE
            | RTformatSafe::RT_FORMAT_BYTE2
            | RTformatSafe::RT_FORMAT_BYTE3
            | RTformatSafe::RT_FORMAT_BYTE4
            | RTformatSafe::RT_FORMAT_UNSIGNED_BYTE
            | RTformatSafe::RT_FORMAT_UNSIGNED_BYTE2
            | RTformatSafe::RT_FORMAT_UNSIGNED_BYTE3
            | RTformatSafe::RT_FORMAT_UNSIGNED_BYTE4 => 1,
            RTformatSafe::RT_FORMAT_SHORT
            | RTformatSafe::RT_FORMAT_SHORT2
            | RTformatSafe::RT_FORMAT_SHORT3
            | RTformatSafe::RT_FORMAT_SHORT4
            | RTformatSafe::RT_FORMAT_UNSIGNED_SHORT
            | RTformatSafe::RT_FORMAT_UNSIGNED_SHORT2
            | RTformatSafe::RT_FORMAT_UNSIGNED_SHORT3
            | RTformatSafe::RT_FORMAT_UNSIGNED_SHORT4
            | RTformatSafe::RT_FORMAT_HALF
            | RTformatSafe::RT_FORMAT_HALF2
            | RTformatSafe::RT_FORMAT_HALF3
            | RTformatSafe::RT_FORMAT_HALF4 => 2,
            RTformatSafe::RT_FORMAT_INT
            | RTformatSafe::RT_FORMAT_INT2
            | RTformatSafe::RT_FORMAT_INT3
            | RTformatSafe::RT_FORMAT_INT4
            | RTformatSafe::RT_FORMAT_UNSIGNED_INT
            | RTformatSafe::RT_FORMAT_UNSIGNED_INT2
            | RTformatSafe::RT_FORMAT_UNSIGNED_INT3
            | RTformatSafe::RT_FORMAT_UNSIGNED_INT4
            | RTformatSafe::RT_FORMAT_FLOAT
            | RTformatSafe::RT_FORMAT_FLOAT2
            | RTformatSafe::RT_FORMAT_FLOAT3
            | RTformatSafe::RT_FORMAT_FLOAT4 => 4,
            RTformatSafe::RT_FORMAT_LONG_LONG
            | RTformatSafe::RT_FORMAT_LONG_LONG2
            | RTformatSafe::RT_FORMAT_LONG_LONG3
            | RTformatSafe::RT_FORMAT_LONG_LONG4
            | RTformatSafe::RT_FORMAT_UNSIGNED_LONG_LONG
            | RTformatSafe::RT_FORMAT_UNSIGNED_LONG_LONG2
            | RTformatSafe::RT_FORMAT_UNSIGNED_LONG_LONG3
            | RTformatSafe::RT_FORMAT_UNSIGNED_LONG_LONG4 => 8,
            RTformatSafe::RT_FORMAT_BUFFER_ID => 4,
            RTformatSafe::RT_FORMAT_PROGRAM_ID => 4,
            RTformatSafe::RT_FORMAT_UNSIGNED_BC1 => return Err(RTresult::RT_ERROR_NOT_SUPPORTED),
            RTformatSafe::RT_FORMAT_UNSIGNED_BC2 => return Err(RTresult::RT_ERROR_NOT_SUPPORTED),
            RTformatSafe::RT_FORMAT_UNSIGNED_BC3 => return Err(RTresult::RT_ERROR_NOT_SUPPORTED),
            RTformatSafe::RT_FORMAT_UNSIGNED_BC4 => return Err(RTresult::RT_ERROR_NOT_SUPPORTED),
            RTformatSafe::RT_FORMAT_BC4 => return Err(RTresult::RT_ERROR_NOT_SUPPORTED),
            RTformatSafe::RT_FORMAT_UNSIGNED_BC5 => return Err(RTresult::RT_ERROR_NOT_SUPPORTED),
            RTformatSafe::RT_FORMAT_BC5 => return Err(RTresult::RT_ERROR_NOT_SUPPORTED),
            RTformatSafe::RT_FORMAT_UNSIGNED_BC6H => return Err(RTresult::RT_ERROR_NOT_SUPPORTED),
            RTformatSafe::RT_FORMAT_BC6H => return Err(RTresult::RT_ERROR_NOT_SUPPORTED),
            RTformatSafe::RT_FORMAT_UNSIGNED_BC7 => return Err(RTresult::RT_ERROR_NOT_SUPPORTED),
        })
    }

    pub(crate) fn array_format(&self) -> Result<hipArray_Format, RTresult> {
        Ok(match self.format {
            RTformatSafe::RT_FORMAT_FLOAT
            | RTformatSafe::RT_FORMAT_FLOAT2
            | RTformatSafe::RT_FORMAT_FLOAT3
            | RTformatSafe::RT_FORMAT_FLOAT4 => hipArray_Format::HIP_AD_FORMAT_FLOAT,
            RTformatSafe::RT_FORMAT_BYTE
            | RTformatSafe::RT_FORMAT_BYTE2
            | RTformatSafe::RT_FORMAT_BYTE3
            | RTformatSafe::RT_FORMAT_BYTE4 => hipArray_Format::HIP_AD_FORMAT_SIGNED_INT8,
            RTformatSafe::RT_FORMAT_UNSIGNED_BYTE
            | RTformatSafe::RT_FORMAT_UNSIGNED_BYTE2
            | RTformatSafe::RT_FORMAT_UNSIGNED_BYTE3
            | RTformatSafe::RT_FORMAT_UNSIGNED_BYTE4 => {
                hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT8
            }
            RTformatSafe::RT_FORMAT_SHORT
            | RTformatSafe::RT_FORMAT_SHORT2
            | RTformatSafe::RT_FORMAT_SHORT3
            | RTformatSafe::RT_FORMAT_SHORT4 => hipArray_Format::HIP_AD_FORMAT_SIGNED_INT16,
            RTformatSafe::RT_FORMAT_UNSIGNED_SHORT
            | RTformatSafe::RT_FORMAT_UNSIGNED_SHORT2
            | RTformatSafe::RT_FORMAT_UNSIGNED_SHORT3
            | RTformatSafe::RT_FORMAT_UNSIGNED_SHORT4 => {
                hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT16
            }
            RTformatSafe::RT_FORMAT_INT
            | RTformatSafe::RT_FORMAT_INT2
            | RTformatSafe::RT_FORMAT_INT3
            | RTformatSafe::RT_FORMAT_INT4 => hipArray_Format::HIP_AD_FORMAT_SIGNED_INT32,
            RTformatSafe::RT_FORMAT_UNSIGNED_INT
            | RTformatSafe::RT_FORMAT_UNSIGNED_INT2
            | RTformatSafe::RT_FORMAT_UNSIGNED_INT3
            | RTformatSafe::RT_FORMAT_UNSIGNED_INT4 => {
                hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT32
            }
            RTformatSafe::RT_FORMAT_HALF
            | RTformatSafe::RT_FORMAT_HALF2
            | RTformatSafe::RT_FORMAT_HALF3
            | RTformatSafe::RT_FORMAT_HALF4 => hipArray_Format::HIP_AD_FORMAT_HALF,
            RTformatSafe::RT_FORMAT_LONG_LONG
            | RTformatSafe::RT_FORMAT_LONG_LONG2
            | RTformatSafe::RT_FORMAT_LONG_LONG3
            | RTformatSafe::RT_FORMAT_LONG_LONG4
            | RTformatSafe::RT_FORMAT_UNSIGNED_LONG_LONG
            | RTformatSafe::RT_FORMAT_UNSIGNED_LONG_LONG2
            | RTformatSafe::RT_FORMAT_UNSIGNED_LONG_LONG3
            | RTformatSafe::RT_FORMAT_UNSIGNED_LONG_LONG4 => {
                return Err(RTresult::RT_ERROR_NOT_SUPPORTED)
            }
            RTformatSafe::RT_FORMAT_UNKNOWN
            | RTformatSafe::RT_FORMAT_USER
            | RTformatSafe::RT_FORMAT_BUFFER_ID
            | RTformatSafe::RT_FORMAT_PROGRAM_ID => return Err(RTresult::RT_ERROR_INVALID_CONTEXT),
            RTformatSafe::RT_FORMAT_UNSIGNED_BC1
            | RTformatSafe::RT_FORMAT_UNSIGNED_BC2
            | RTformatSafe::RT_FORMAT_UNSIGNED_BC3
            | RTformatSafe::RT_FORMAT_UNSIGNED_BC4
            | RTformatSafe::RT_FORMAT_BC4
            | RTformatSafe::RT_FORMAT_UNSIGNED_BC5
            | RTformatSafe::RT_FORMAT_BC5
            | RTformatSafe::RT_FORMAT_UNSIGNED_BC6H
            | RTformatSafe::RT_FORMAT_BC6H
            | RTformatSafe::RT_FORMAT_UNSIGNED_BC7 => return Err(RTresult::RT_ERROR_NOT_SUPPORTED),
        })
    }

    pub(crate) fn channels(&self) -> Result<u32, RTresult> {
        Ok(match self.format {
            RTformatSafe::RT_FORMAT_UNKNOWN
            | RTformatSafe::RT_FORMAT_USER
            | RTformatSafe::RT_FORMAT_BUFFER_ID
            | RTformatSafe::RT_FORMAT_PROGRAM_ID
            | RTformatSafe::RT_FORMAT_FLOAT
            | RTformatSafe::RT_FORMAT_BYTE
            | RTformatSafe::RT_FORMAT_UNSIGNED_BYTE
            | RTformatSafe::RT_FORMAT_SHORT
            | RTformatSafe::RT_FORMAT_UNSIGNED_SHORT
            | RTformatSafe::RT_FORMAT_INT
            | RTformatSafe::RT_FORMAT_UNSIGNED_INT
            | RTformatSafe::RT_FORMAT_HALF
            | RTformatSafe::RT_FORMAT_LONG_LONG
            | RTformatSafe::RT_FORMAT_UNSIGNED_LONG_LONG => 1,
            RTformatSafe::RT_FORMAT_FLOAT2
            | RTformatSafe::RT_FORMAT_BYTE2
            | RTformatSafe::RT_FORMAT_UNSIGNED_BYTE2
            | RTformatSafe::RT_FORMAT_SHORT2
            | RTformatSafe::RT_FORMAT_UNSIGNED_SHORT2
            | RTformatSafe::RT_FORMAT_INT2
            | RTformatSafe::RT_FORMAT_UNSIGNED_INT2
            | RTformatSafe::RT_FORMAT_HALF2
            | RTformatSafe::RT_FORMAT_LONG_LONG2
            | RTformatSafe::RT_FORMAT_UNSIGNED_LONG_LONG2 => 2,
            RTformatSafe::RT_FORMAT_FLOAT3
            | RTformatSafe::RT_FORMAT_BYTE3
            | RTformatSafe::RT_FORMAT_UNSIGNED_BYTE3
            | RTformatSafe::RT_FORMAT_SHORT3
            | RTformatSafe::RT_FORMAT_UNSIGNED_SHORT3
            | RTformatSafe::RT_FORMAT_INT3
            | RTformatSafe::RT_FORMAT_UNSIGNED_INT3
            | RTformatSafe::RT_FORMAT_HALF3
            | RTformatSafe::RT_FORMAT_LONG_LONG3
            | RTformatSafe::RT_FORMAT_UNSIGNED_LONG_LONG3 => 3,
            RTformatSafe::RT_FORMAT_FLOAT4
            | RTformatSafe::RT_FORMAT_BYTE4
            | RTformatSafe::RT_FORMAT_UNSIGNED_BYTE4
            | RTformatSafe::RT_FORMAT_SHORT4
            | RTformatSafe::RT_FORMAT_UNSIGNED_SHORT4
            | RTformatSafe::RT_FORMAT_INT4
            | RTformatSafe::RT_FORMAT_UNSIGNED_INT4
            | RTformatSafe::RT_FORMAT_HALF4
            | RTformatSafe::RT_FORMAT_LONG_LONG4
            | RTformatSafe::RT_FORMAT_UNSIGNED_LONG_LONG4 => 4,
            RTformatSafe::RT_FORMAT_UNSIGNED_BC1
            | RTformatSafe::RT_FORMAT_UNSIGNED_BC2
            | RTformatSafe::RT_FORMAT_UNSIGNED_BC3
            | RTformatSafe::RT_FORMAT_UNSIGNED_BC4
            | RTformatSafe::RT_FORMAT_BC4
            | RTformatSafe::RT_FORMAT_UNSIGNED_BC5
            | RTformatSafe::RT_FORMAT_BC5
            | RTformatSafe::RT_FORMAT_UNSIGNED_BC6H
            | RTformatSafe::RT_FORMAT_BC6H
            | RTformatSafe::RT_FORMAT_UNSIGNED_BC7 => return Err(RTresult::RT_ERROR_NOT_SUPPORTED),
        })
    }

    fn next_mip(self) -> Self {
        Self {
            format: self.format,
            element_size: self.element_size,
            width: (self.width / 2).max(1),
            height: (self.height / 2).max(1),
        }
    }

    fn generate_mips(self) -> impl Iterator<Item = Self> {
        std::iter::repeat(()).scan(self, |metadata, _| {
            let result = *metadata;
            *metadata = metadata.next_mip();
            Some(result)
        })
    }
}

struct OnDemandAllocation {
    mip_levels: u32,
    alloc: BufferAllocation,
    callback_data: *mut ::std::os::raw::c_void,
    callback: unsafe extern "C" fn(
        callback_data: *mut ::std::os::raw::c_void,
        buffer: Buffer,
        block: *mut RTmemoryblock,
    ) -> ::std::os::raw::c_int,
}

enum BufferStorage {
    Normal(Vec<BufferAllocation>),
    FromCallback(OnDemandAllocation),
}

pub(crate) struct BufferData {
    pub(crate) context: Weak<OptixCell<ContextData>>,
    pub(crate) metadata: BufferMetadata,
    alloc: BufferStorage,
    pub(crate) index: u32,
}

impl OptixObjectData for BufferData {
    const TYPE: TypeTag = TypeTag::Buffer;

    fn deregister(&mut self, this: &Rc<OptixCell<Self>>) -> Result<(), RTresult> {
        if let Some(context) = self.context.upgrade() {
            let mut context = (*context).borrow_mut()?;
            context.buffers.remove(this);
        }
        Ok(())
    }

    fn context<'a>(&'a mut self) -> crate::MaybeWeakRefMut<'a, ContextData> {
        MaybeWeakRefMut::Weak(&self.context)
    }
}

impl BufferData {
    fn new(weak_context: Weak<OptixCell<ContextData>>, context: &mut ContextData) -> Self {
        context.buffers_counter += 1;
        let metadata = BufferMetadata {
            format: RTformatSafe::RT_FORMAT_BYTE,
            element_size: 1,
            width: 0,
            height: 0,
        };
        Self {
            context: weak_context,
            index: context.buffers_counter,
            metadata,
            alloc: BufferStorage::Normal(vec![BufferAllocation::empty()]),
        }
    }

    fn new_from_callback(
        weak_context: Weak<OptixCell<ContextData>>,
        context: &mut ContextData,
        callback: unsafe extern "C" fn(
            callback_data: *mut ::std::os::raw::c_void,
            buffer: Buffer,
            block: *mut RTmemoryblock,
        ) -> i32,
        callback_data: *mut ::std::os::raw::c_void,
    ) -> Self {
        context.buffers_counter += 1;
        let metadata = BufferMetadata {
            format: RTformatSafe::RT_FORMAT_BYTE,
            element_size: 1,
            width: 0,
            height: 0,
        };
        Self {
            context: weak_context,
            index: context.buffers_counter,
            metadata,
            alloc: BufferStorage::FromCallback(OnDemandAllocation {
                mip_levels: 1,
                alloc: BufferAllocation::empty(),
                callback_data,
                callback,
            }),
        }
    }

    fn alloc0(&self) -> &BufferAllocation {
        match &self.alloc {
            BufferStorage::Normal(alloc) => &alloc[0],
            BufferStorage::FromCallback(on_demand) => &on_demand.alloc,
        }
    }

    fn propagate_size_change(&mut self) -> Result<(), RTresult> {
        match self.alloc {
            BufferStorage::Normal(ref mut alloc) => {
                for (alloc, metadata) in alloc.iter_mut().zip(self.metadata.generate_mips()) {
                    let alloc_copy = mem::replace(alloc, BufferAllocation::empty());
                    *alloc = BufferAllocation::new(Some(alloc_copy), metadata)?.0;
                }
            }
            BufferStorage::FromCallback(_) => {}
        }
        Ok(())
    }

    // We do all this dance because callback function can and does call various buffer functions (eg rtBufferGetSize2D)
    pub(crate) fn load_from_callback(this: &Rc<OptixCell<Self>>) -> Result<(), RTresult> {
        let needs_copy = {
            let mut this = this.borrow_mut_no_invalidate()?;
            let metadata = this.metadata;
            match this.alloc {
                BufferStorage::Normal(_) => return Ok(()),
                BufferStorage::FromCallback(ref mut alloc) => {
                    let alloc_copy = mem::replace(&mut alloc.alloc, BufferAllocation::empty());
                    let (new_alloc, needs_copy) =
                        BufferAllocation::new(Some(alloc_copy), metadata)?;
                    alloc.alloc = new_alloc;
                    if needs_copy {
                        Some((alloc.callback, alloc.callback_data, metadata))
                    } else {
                        None
                    }
                }
            }
        };
        if let Some((callback, callback_data, metadata)) = needs_copy {
            let layout = unsafe {
                Layout::from_size_align_unchecked(
                    metadata.byte_size() as usize,
                    BufferMetadata::alignment(metadata.format)?,
                )
            };
            let mut buffer = AlignedBuffer::new(layout);
            let mut block = RTmemoryblock {
                format: metadata.format.into(),
                baseAddress: buffer.as_bytes_mut().as_mut_ptr().cast(),
                mipLevel: 0u32,
                x: 0,
                y: 0,
                z: 0,
                width: metadata.width as u32,
                height: metadata.height.max(1) as u32,
                depth: metadata.depth().max(1) as u32,
                rowPitch: (metadata.width * metadata.element_size) as u32,
                planePitch: 0,
            };
            if unsafe { (callback)(callback_data, Rc::as_ptr(this), &mut block) } == 0 {
                return Err(RTresult::RT_ERROR_UNKNOWN);
            }
            let mut this = this.borrow_mut()?;
            let alloc = match this.alloc {
                BufferStorage::Normal(_) => return Err(RTresult::RT_ERROR_UNKNOWN),
                BufferStorage::FromCallback(ref mut alloc) => alloc,
            };
            hip! { hipMemcpyHtoD(alloc.alloc.pointer, buffer.as_ptr(), metadata.byte_size() as usize), RT_ERROR_UNKNOWN };
        }
        Ok(())
    }

    fn register(this: Rc<OptixCell<Self>>, context: &mut ContextData) {
        context.buffers.insert(this);
    }

    fn map_ex(&mut self, level: u32) -> Result<*mut c_void, RTresult> {
        match self.alloc {
            BufferStorage::Normal(ref mut alloc) => {
                let alloc = alloc
                    .get_mut(level as usize)
                    .ok_or(RTresult::RT_ERROR_INVALID_VALUE)?;
                alloc.map(self.metadata)
            }
            BufferStorage::FromCallback(_) => Err(RTresult::RT_ERROR_UNKNOWN),
        }
    }

    fn unmap_ex(&mut self, level: u32) -> Result<(), RTresult> {
        match self.alloc {
            BufferStorage::Normal(ref mut alloc) => {
                let alloc = alloc
                    .get_mut(level as usize)
                    .ok_or(RTresult::RT_ERROR_INVALID_VALUE)?;
                alloc.unmap()
            }
            BufferStorage::FromCallback(_) => Err(RTresult::RT_ERROR_UNKNOWN),
        }
    }

    unsafe fn create(context: Context) -> Result<Buffer, RTresult> {
        context::create_subobject(context, BufferData::new, BufferData::register)
    }

    unsafe fn create_from_callback(
        context: Context,
        callback: unsafe extern "C" fn(
            callback_data: *mut ::std::os::raw::c_void,
            buffer: Buffer,
            block: *mut RTmemoryblock,
        ) -> i32,
        callback_data: *mut ::std::os::raw::c_void,
    ) -> Result<Buffer, RTresult> {
        context::create_subobject(
            context,
            |weak_context, context| {
                BufferData::new_from_callback(weak_context, context, callback, callback_data)
            },
            BufferData::register,
        )
    }

    pub(crate) fn get_device_mip0(&self) -> DeviceBuffer {
        let alloc = &self.alloc0();
        DeviceBuffer {
            pointer: alloc.pointer,
            width: self.metadata.width,
            height: self.metadata.height,
        }
    }

    pub(crate) fn pointer_mip0(&self) -> hipDeviceptr_t {
        self.alloc0().pointer
    }
}

pub(crate) unsafe fn create(
    context: Context,
    _bufferdesc: u32,
    buffer: *mut Buffer,
) -> Result<(), RTresult> {
    null_check(context)?;
    null_check(buffer)?;
    *buffer = BufferData::create(context)?;
    Ok(())
}

pub(crate) unsafe fn create_from_callback(
    context: Context,
    _bufferdesc: ::std::os::raw::c_uint,
    callback: Option<
        unsafe extern "C" fn(
            callbackData: *mut ::std::os::raw::c_void,
            buffer: Buffer,
            block: *mut RTmemoryblock,
        ) -> ::std::os::raw::c_int,
    >,
    callback_data: *mut ::std::os::raw::c_void,
    buffer: *mut Buffer,
) -> Result<(), RTresult> {
    null_check(context)?;
    null_check(buffer)?;
    let callback = callback.ok_or(RTresult::RT_ERROR_INVALID_VALUE)?;
    *buffer = BufferData::create_from_callback(context, callback, callback_data)?;
    Ok(())
}

pub(crate) unsafe fn destroy(buffer: Buffer) -> Result<(), RTresult> {
    OptixCell::destroy(buffer)
}

pub(crate) unsafe fn get_dimensionality(
    buffer: Buffer,
    dimensionality: *mut u32,
) -> Result<(), RTresult> {
    let buffer = null_unwrap(buffer)?;
    let dimensionality = null_unwrap_mut(dimensionality)?;
    let buffer = buffer.borrow()?;
    *dimensionality = buffer.metadata.dimensions();
    Ok(())
}

pub(crate) unsafe fn get_format(buffer: Buffer, format: *mut RTformat) -> Result<(), RTresult> {
    let buffer = null_unwrap(buffer)?;
    let format = null_unwrap_mut(format)?;
    let buffer = buffer.borrow()?;
    *format = RTformat(buffer.metadata.format as u32);
    Ok(())
}

pub(crate) unsafe fn get_size1d(buffer: Buffer, width: *mut u64) -> Result<(), RTresult> {
    let buffer = null_unwrap(buffer)?;
    let width = null_unwrap_mut(width)?;
    let buffer = buffer.borrow()?;
    *width = buffer.metadata.size(0);
    Ok(())
}

pub(crate) unsafe fn set_format(buffer_ptr: Buffer, format: RTformat) -> Result<(), RTresult> {
    let buffer = null_unwrap(buffer_ptr)?;
    let mut buffer = buffer.borrow_mut()?;
    buffer.metadata.format = RTformatSafe::new(format).ok_or(RTresult::RT_ERROR_INVALID_VALUE)?;
    buffer.metadata.element_size = BufferMetadata::element_size(buffer.metadata.format)? as u64;
    buffer.propagate_size_change()?;
    Ok(())
}

pub(crate) unsafe fn set_size1d(buffer_ptr: Buffer, width: u64) -> Result<(), RTresult> {
    let buffer = null_unwrap(buffer_ptr)?;
    let mut buffer = buffer.borrow_mut()?;
    buffer.metadata.width = width;
    buffer.metadata.height = 0;
    buffer.propagate_size_change()?;
    Ok(())
}

pub(crate) unsafe fn set_size2d(
    buffer_ptr: Buffer,
    width: u64,
    height: u64,
) -> Result<(), RTresult> {
    let buffer = null_unwrap(buffer_ptr)?;
    let mut buffer = buffer.borrow_mut()?;
    buffer.metadata.width = width;
    buffer.metadata.height = height;
    buffer.propagate_size_change()?;
    Ok(())
}

pub(crate) unsafe fn get_context(buffer: Buffer, context: *mut Context) -> Result<(), RTresult> {
    null_check(context)?;
    let buffer = null_unwrap(buffer)?;
    let buffer = buffer.borrow()?;
    *context = Weak::as_ptr(&buffer.context);
    Ok(())
}

pub(crate) unsafe fn get_size2d(
    buffer: Buffer,
    width: *mut u64,
    height: *mut u64,
) -> Result<(), RTresult> {
    null_check(width)?;
    null_check(height)?;
    let buffer = null_unwrap(buffer)?;
    let buffer = buffer.borrow()?;
    *width = buffer.metadata.width;
    *height = buffer.metadata.height;
    Ok(())
}

pub(crate) unsafe fn map(buffer: Buffer, user_pointer: *mut *mut c_void) -> Result<(), RTresult> {
    map_ex(
        buffer,
        RTbuffermapflag::RT_BUFFER_MAP_READ_WRITE.0,
        0,
        ptr::null_mut(),
        user_pointer,
    )
}

pub(crate) unsafe fn unmap(buffer: Buffer) -> Result<(), RTresult> {
    unmap_ex(buffer, 0)
}

pub(crate) unsafe fn get_glboid(_buffer: Buffer, glid: *mut u32) -> Result<(), RTresult> {
    null_check(glid)?;
    *glid = 0;
    Ok(())
}

pub(crate) unsafe fn map_ex(
    buffer: Buffer,
    _map_flags: u32,
    level: u32,
    user_owned: *mut c_void,
    optix_owned: *mut *mut c_void,
) -> Result<(), RTresult> {
    if user_owned != ptr::null_mut() {
        return Err(RTresult::RT_ERROR_INVALID_VALUE);
    }

    null_check(optix_owned)?;
    let buffer = null_unwrap(buffer)?;
    let mut buffer = buffer.borrow_mut_no_invalidate()?;
    *optix_owned = buffer.map_ex(level)?;
    Ok(())
}

pub(crate) unsafe fn unmap_ex(buffer: Buffer, level: u32) -> Result<(), RTresult> {
    let buffer = null_unwrap(buffer)?;
    let mut buffer = buffer.borrow_mut_no_invalidate()?;
    buffer.unmap_ex(level)
}

pub(crate) unsafe fn get_element_size(
    buffer: Buffer,
    element_size: *mut u64,
) -> Result<(), RTresult> {
    null_check(element_size)?;
    let buffer = null_unwrap(buffer)?;
    let buffer = buffer.borrow()?;
    *element_size = buffer.metadata.element_size;
    Ok(())
}

pub(crate) unsafe fn set_element_size(
    buffer_ptr: Buffer,
    element_size: u64,
) -> Result<(), RTresult> {
    if element_size == 0 {
        return Err(RTresult::RT_ERROR_INVALID_VALUE);
    }
    let buffer = null_unwrap(buffer_ptr)?;
    let mut buffer = buffer.borrow_mut()?;
    if buffer.metadata.format != RTformatSafe::RT_FORMAT_USER {
        return Ok(());
    }
    buffer.metadata.element_size = element_size;
    buffer.propagate_size_change()?;
    Ok(())
}

pub(crate) unsafe fn get_id(buffer: Buffer, buffer_id: *mut i32) -> Result<(), RTresult> {
    null_check(buffer_id)?;
    let buffer = null_unwrap(buffer)?;
    let buffer = buffer.borrow()?;
    *buffer_id = buffer.index as i32;
    Ok(())
}

pub(crate) unsafe fn get_miplevel_count(buffer: Buffer, level: *mut u32) -> Result<(), RTresult> {
    null_check(level)?;
    let buffer = null_unwrap(buffer)?;
    let buffer = buffer.borrow()?;
    let buffer_levels = match &buffer.alloc {
        BufferStorage::Normal(alloc) => alloc.len() as u32,
        BufferStorage::FromCallback(alloc) => alloc.mip_levels,
    };
    *level = buffer_levels;
    Ok(())
}

pub(crate) unsafe fn get_sizev(
    buffer: Buffer,
    dimensionality: u32,
    dims: *mut u64,
) -> Result<(), RTresult> {
    null_check(dims)?;
    let buffer = null_unwrap(buffer)?;
    let buffer = buffer.borrow()?;
    for d in 0..dimensionality {
        let size = match d {
            0 => buffer.metadata.width,
            1 => buffer.metadata.height,
            _ => 0,
        };
        *dims.add(d as usize) = size;
    }
    Ok(())
}

pub(crate) unsafe fn get_device_pointer(
    buffer: Buffer,
    _optix_device_ordinal: i32,
    device_pointer: *mut *mut c_void,
) -> Result<(), RTresult> {
    null_check(device_pointer)?;
    let buffer = null_unwrap(buffer)?;
    let buffer = buffer.borrow()?;
    *device_pointer = buffer.alloc0().pointer.0;
    Ok(())
}

pub(crate) unsafe fn set_mip_level_count(buffer: Buffer, levels: u32) -> Result<(), RTresult> {
    let buffer = null_unwrap(buffer)?;
    let mut buffer = buffer.borrow_mut()?;
    let levels = levels as usize;
    let metadata = buffer.metadata;
    let alloc = match &mut buffer.alloc {
        BufferStorage::Normal(alloc) => alloc,
        // TODO: implement MIP levels when mipmapped textures work in HIP
        BufferStorage::FromCallback(alloc) => {
            alloc.mip_levels = levels as u32;
            return Ok(());
        }
    };
    if levels <= alloc.len() {
        alloc.truncate(levels);
        return Ok(());
    }
    let mut meta_generator = metadata.generate_mips().skip(alloc.len());
    for _ in alloc.len()..levels {
        let metadata = meta_generator.next().unwrap();
        alloc.push(BufferAllocation::new(None, metadata)?.0);
    }
    Ok(())
}

pub(crate) unsafe fn get_miplevel_size2d(
    buffer: Buffer,
    level: u32,
    width: *mut u64,
    height: *mut u64,
) -> Result<(), RTresult> {
    if level != 0 {
        return Err(RTresult::RT_ERROR_NOT_SUPPORTED);
    }
    get_size2d(buffer, width, height)
}

#[cfg(test)]
mod tests {
    use crate::optix_test;
    use crate::test_common::OptixFns;
    use optix_types::*;
    use std::{mem, ptr};

    optix_test!(new_buffer);

    unsafe fn new_buffer<Optix: OptixFns>(o: Optix) {
        let mut ctx = ptr::null_mut();
        o.rtContextCreate(&mut ctx);
        let mut buffer = ptr::null_mut();
        o.rtBufferCreate(ctx, RTbuffertype::RT_BUFFER_OUTPUT.0, &mut buffer);
        let mut dims = mem::zeroed();
        o.rtBufferGetDimensionality(buffer, &mut dims);
        assert_eq!(dims, 1);
        let mut format = mem::zeroed();
        o.rtBufferGetFormat(buffer, &mut format);
        assert_eq!(format, RTformat::RT_FORMAT_BYTE);
        let mut size = mem::zeroed();
        o.rtBufferGetSize1D(buffer, &mut size);
        assert_eq!(size, 0);
        let mut dim_buffer = [164, 2, 3];
        o.rtBufferGetSizev(buffer, 3, dim_buffer.as_mut_ptr());
        assert_eq!(dim_buffer, [0, 0, 0]);
        o.rtBufferDestroy(buffer);
        o.rtContextDestroy(ctx);
    }

    optix_test!(empty_buffer_can_be_mapped);

    unsafe fn empty_buffer_can_be_mapped<Optix: OptixFns>(o: Optix) {
        let mut ctx = ptr::null_mut();
        o.rtContextCreate(&mut ctx);
        let mut buffer = ptr::null_mut();
        o.rtBufferCreate(ctx, RTbuffertype::RT_BUFFER_INPUT.0, &mut buffer);
        o.rtBufferSetFormat(buffer, RTformat::RT_FORMAT_PROGRAM_ID);
        let mut dims = mem::zeroed();
        o.rtBufferGetDimensionality(buffer, &mut dims);
        assert_eq!(dims, 1);
        let mut size = mem::zeroed();
        o.rtBufferGetSize1D(buffer, &mut size);
        assert_eq!(size, 0);
        let mut host_ptr = ptr::null_mut();
        o.rtBufferMapEx(
            buffer,
            RTbuffermapflag::RT_BUFFER_MAP_READ_WRITE.0,
            0,
            ptr::null_mut(),
            &mut host_ptr,
        );
        assert_ne!(host_ptr, ptr::null_mut());
        o.rtBufferUnmapEx(buffer, 0);
        o.rtBufferDestroy(buffer);
        o.rtContextDestroy(ctx);
    }
}
