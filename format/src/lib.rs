use cuda_types::cuda::*;
use std::{
    any::TypeId,
    ffi::{c_ulonglong, c_void, CStr},
    fmt::LowerHex,
    mem, ptr, slice,
};

pub trait CudaDisplay {
    fn write(
        &self,
        fn_name: &'static str,
        index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()>;
}

impl CudaDisplay for () {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "()")
    }
}

impl CudaDisplay for CUuuid {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        let guid = self.bytes;
        let uuid = uuid::Uuid::from_bytes(guid);
        let braced = uuid.as_braced();
        write!(writer, "{braced:#X}")
    }
}

impl CudaDisplay for CUdeviceptr_v1 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{:p}", self.0 as usize as *const ())
    }
}

impl CudaDisplay for bool {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{}", *self)
    }
}

impl CudaDisplay for u8 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{}", *self)
    }
}

impl CudaDisplay for u16 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{}", *self)
    }
}

impl CudaDisplay for i32 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{}", *self)
    }
}

impl CudaDisplay for u32 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{}", *self)
    }
}

impl CudaDisplay for i64 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{}", *self)
    }
}

impl CudaDisplay for u64 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{}", *self)
    }
}

impl CudaDisplay for usize {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{}", *self)
    }
}

impl CudaDisplay for f32 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{}", *self)
    }
}

impl CudaDisplay for f64 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{}", *self)
    }
}

// user by Dark API
impl CudaDisplay
    for Option<
        extern "system" fn(
            cuda_types::cuda::CUcontext,
            *mut std::ffi::c_void,
            *mut std::ffi::c_void,
        ),
    >
{
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        if let Some(fn_ptr) = self {
            write!(writer, "{:p}", *fn_ptr)
        } else {
            writer.write_all(b"NULL")
        }
    }
}

impl CudaDisplay for Option<unsafe extern "C" fn(*const i8)> {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        if let Some(fn_ptr) = self {
            write!(writer, "{:p}", *fn_ptr)
        } else {
            writer.write_all(b"NULL")
        }
    }
}

impl CudaDisplay for Option<unsafe extern "C" fn(i32, *const i8, *const i8)> {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        if let Some(fn_ptr) = self {
            write!(writer, "{:p}", *fn_ptr)
        } else {
            writer.write_all(b"NULL")
        }
    }
}

pub fn write_handle<T: LowerHex>(
    this: &[T; 64],
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"0x")?;
    for i in (0..64).rev() {
        write!(writer, "{:02x}", this[i])?;
    }
    Ok(())
}

impl CudaDisplay for CUipcMemHandle {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write_handle(&self.reserved, writer)
    }
}

impl CudaDisplay for CUipcEventHandle {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write_handle(&self.reserved, writer)
    }
}

impl CudaDisplay for CUmemPoolPtrExportData_v1 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write_handle(&self.reserved, writer)
    }
}

impl CudaDisplay for *mut c_void {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{:p}", *self)
    }
}

impl CudaDisplay for *const c_void {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{:p}", *self)
    }
}

impl CudaDisplay for *const i8 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        if self.is_null() {
            writer.write_all(b"NULL")
        } else {
            write!(
                writer,
                "\"{}\"",
                unsafe { CStr::from_ptr(*self as _) }.to_string_lossy()
            )
        }
    }
}

impl CudaDisplay for *mut cuda_types::FILE {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        if self.is_null() {
            writer.write_all(b"NULL")
        } else {
            write!(writer, "{:p}", *self)
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Luid {
    low_part: u32,
    high_part: u32,
}

impl CudaDisplay for *mut i8 {
    fn write(
        &self,
        fn_name: &'static str,
        index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        if fn_name == "cuDeviceGetLuid" && index == 0 {
            let luid_ptr = *self as *mut Luid;
            let luid = unsafe { *luid_ptr };
            write!(writer, "{{{:08X}-{:08X}}}", luid.low_part, luid.high_part)
        } else {
            write!(
                writer,
                "\"{}\"",
                unsafe { CStr::from_ptr(*self as _) }.to_string_lossy()
            )
        }
    }
}

impl CudaDisplay for CUstreamBatchMemOpParams {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        unsafe {
            match self.operation {
                // The below is not a typo, `WAIT_VALUE` and `WRITE_VALUE` are
                // distinct operations with nominally distinct union variants, but
                // in reality they are structurally different, so we take a little
                // shortcut here
                CUstreamBatchMemOpType::CU_STREAM_MEM_OP_WAIT_VALUE_32
                | CUstreamBatchMemOpType::CU_STREAM_MEM_OP_WRITE_VALUE_32 => {
                    write_wait_value(&self.waitValue, writer, false)
                }
                CUstreamBatchMemOpType::CU_STREAM_MEM_OP_WAIT_VALUE_64
                | CUstreamBatchMemOpType::CU_STREAM_MEM_OP_WRITE_VALUE_64 => {
                    write_wait_value(&self.waitValue, writer, true)
                }
                CUstreamBatchMemOpType::CU_STREAM_MEM_OP_FLUSH_REMOTE_WRITES => {
                    CudaDisplay::write(&self.flushRemoteWrites, "", 0, writer)
                }
                _ => {
                    writer.write_all(b"{ operation: ")?;
                    CudaDisplay::write(&self.operation, "", 0, writer)?;
                    writer.write_all(b", ... }")
                }
            }
        }
    }
}

impl CudaDisplay for CUcheckpointRestoreArgs_st {
    fn write(
        &self,
        fn_name: &'static str,
        index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        CudaDisplay::write(&self.reserved, fn_name, index, writer)
    }
}

impl CudaDisplay for CUcheckpointUnlockArgs_st {
    fn write(
        &self,
        fn_name: &'static str,
        index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        CudaDisplay::write(&self.reserved, fn_name, index, writer)
    }
}

impl CudaDisplay for CUcheckpointCheckpointArgs_st {
    fn write(
        &self,
        fn_name: &'static str,
        index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        CudaDisplay::write(&self.reserved, fn_name, index, writer)
    }
}

impl CudaDisplay for CUmemcpy3DOperand_st {
    fn write(
        &self,
        fn_name: &'static str,
        index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(b"{ type_: ")?;
        CudaDisplay::write(&self.type_, "", 0, writer)?;
        writer.write_all(b", op: ")?;
        match self.type_ {
            CUmemcpy3DOperandType::CU_MEMCPY_OPERAND_TYPE_ARRAY => {
                CudaDisplay::write(unsafe { &self.op.array }, fn_name, index, writer)?;
            }
            CUmemcpy3DOperandType::CU_MEMCPY_OPERAND_TYPE_POINTER => {
                CudaDisplay::write(unsafe { &self.op.ptr }, fn_name, index, writer)?;
            }
            _ => {
                const CU_MEMCPY_3D_OP_SIZE: usize =
                    mem::size_of::<CUmemcpy3DOperand_st__bindgen_ty_1>();
                CudaDisplay::write(
                    &unsafe { mem::transmute::<_, [u8; CU_MEMCPY_3D_OP_SIZE]>(self.op) },
                    fn_name,
                    index,
                    writer,
                )?;
            }
        }
        writer.write_all(b" }")
    }
}

pub fn write_wait_value(
    this: &CUstreamBatchMemOpParams_union_CUstreamMemOpWaitValueParams_st,
    writer: &mut (impl std::io::Write + ?Sized),
    is_64_bit: bool,
) -> std::io::Result<()> {
    writer.write_all(b"{ operation: ")?;
    CudaDisplay::write(&this.operation, "", 0, writer)?;
    writer.write_all(b", address: ")?;
    CudaDisplay::write(&this.address, "", 0, writer)?;
    write_wait_value_32_or_64(&this.__bindgen_anon_1, writer, is_64_bit)?;
    writer.write_all(b", flags: ")?;
    CudaDisplay::write(&this.flags, "", 0, writer)?;
    writer.write_all(b", alias: ")?;
    CudaDisplay::write(&this.alias, "", 0, writer)?;
    writer.write_all(b" }")
}

pub fn write_wait_value_32_or_64(
    this: &CUstreamBatchMemOpParams_union_CUstreamMemOpWaitValueParams_st__bindgen_ty_1,
    writer: &mut (impl std::io::Write + ?Sized),
    is_64_bit: bool,
) -> std::io::Result<()> {
    if is_64_bit {
        writer.write_all(b", value64: ")?;
        CudaDisplay::write(unsafe { &this.value64 }, "", 0, writer)
    } else {
        writer.write_all(b", value: ")?;
        CudaDisplay::write(unsafe { &this.value }, "", 0, writer)
    }
}

impl CudaDisplay for CUDA_RESOURCE_DESC_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(b"{ resType: ")?;
        CudaDisplay::write(&self.resType, "", 0, writer)?;
        match self.resType {
            CUresourcetype::CU_RESOURCE_TYPE_ARRAY => {
                writer.write_all(b", res: ")?;
                CudaDisplay::write(unsafe { &self.res.array }, "", 0, writer)?;
                writer.write_all(b", flags: ")?;
                CudaDisplay::write(&self.flags, "", 0, writer)?;
                writer.write_all(b" }")
            }
            CUresourcetype::CU_RESOURCE_TYPE_MIPMAPPED_ARRAY => {
                writer.write_all(b", res: ")?;
                CudaDisplay::write(unsafe { &self.res.mipmap }, "", 0, writer)?;
                writer.write_all(b", flags: ")?;
                CudaDisplay::write(&self.flags, "", 0, writer)?;
                writer.write_all(b" }")
            }
            CUresourcetype::CU_RESOURCE_TYPE_LINEAR => {
                writer.write_all(b", res: ")?;
                CudaDisplay::write(unsafe { &self.res.linear }, "", 0, writer)?;
                writer.write_all(b", flags: ")?;
                CudaDisplay::write(&self.flags, "", 0, writer)?;
                writer.write_all(b" }")
            }
            CUresourcetype::CU_RESOURCE_TYPE_PITCH2D => {
                writer.write_all(b", res: ")?;
                CudaDisplay::write(unsafe { &self.res.pitch2D }, "", 0, writer)?;
                writer.write_all(b", flags: ")?;
                CudaDisplay::write(&self.flags, "", 0, writer)?;
                writer.write_all(b" }")
            }
            _ => {
                writer.write_all(b", flags: ")?;
                CudaDisplay::write(&self.flags, "", 0, writer)?;
                writer.write_all(b", ... }")
            }
        }
    }
}

impl crate::CudaDisplay for cuda_types::cuda::CUlaunchConfig_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(gridDimX), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.gridDimX, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(gridDimY), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.gridDimY, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(gridDimZ), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.gridDimZ, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(blockDimX), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.blockDimX, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(blockDimY), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.blockDimY, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(blockDimZ), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.blockDimZ, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(sharedMemBytes), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sharedMemBytes, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(hStream), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.hStream, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(numAttrs), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.numAttrs, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(attrs), ": ").as_bytes())?;
        writer.write_all(b"[")?;
        for i in 0..self.numAttrs {
            if i != 0 {
                writer.write_all(b", ")?;
            }
            crate::CudaDisplay::write(&unsafe { *self.attrs.add(i as usize) }, "", 0, writer)?;
        }
        writer.write_all(b"]")?;
        writer.write_all(b" }")
    }
}

impl CudaDisplay for CUDA_EXTERNAL_MEMORY_HANDLE_DESC_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(b"{ type: ")?;
        CudaDisplay::write(&self.type_, "", 0, writer)?;
        match self.type_ {
            CUexternalMemoryHandleType::CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD => {
                writer.write_all(b", handle: ")?;
                CudaDisplay::write(unsafe { &self.handle.fd }, "", 0, writer)?;
            }
            CUexternalMemoryHandleType::CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32
            | CUexternalMemoryHandleType::CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP
            | CUexternalMemoryHandleType::CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE
            | CUexternalMemoryHandleType::CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_RESOURCE => {
                write_win32_handle(unsafe { self.handle.win32 }, writer)?;
            }
            CUexternalMemoryHandleType::CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT
            | CUexternalMemoryHandleType::CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_RESOURCE_KMT => {
                writer.write_all(b", handle: ")?;
                CudaDisplay::write(unsafe { &self.handle.win32.handle }, "", 0, writer)?;
            }
            CUexternalMemoryHandleType::CU_EXTERNAL_MEMORY_HANDLE_TYPE_NVSCIBUF => {
                writer.write_all(b", handle: ")?;
                CudaDisplay::write(unsafe { &self.handle.nvSciBufObject }, "", 0, writer)?;
            }
            _ => {
                writer.write_all(b", size: ")?;
                CudaDisplay::write(&self.size, "", 0, writer)?;
                writer.write_all(b", flags: ")?;
                CudaDisplay::write(&self.flags, "", 0, writer)?;
                return writer.write_all(b", ... }");
            }
        }
        writer.write_all(b", size: ")?;
        CudaDisplay::write(&self.size, "", 0, writer)?;
        writer.write_all(b", flags: ")?;
        CudaDisplay::write(&self.flags, "", 0, writer)?;
        writer.write_all(b" }")
    }
}

pub fn write_win32_handle(
    win32: CUDA_EXTERNAL_MEMORY_HANDLE_DESC_st__bindgen_ty_1__bindgen_ty_1,
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    if win32.handle != ptr::null_mut() {
        writer.write_all(b", handle: ")?;
        CudaDisplay::write(&win32.handle, "", 0, writer)?;
    }
    if win32.name != ptr::null_mut() {
        let name_ptr = win32.name as *const u16;
        let mut strlen = 0usize;
        while unsafe { *name_ptr.add(strlen) } != 0 {
            strlen += 1;
        }
        let text = String::from_utf16_lossy(unsafe { slice::from_raw_parts(name_ptr, strlen) });
        write!(writer, ", name: \"{}\"", text)?;
    }
    Ok(())
}

impl CudaDisplay for CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(b"{ type: ")?;
        CudaDisplay::write(&self.type_, "", 0, writer)?;
        match self.type_ {
            CUexternalSemaphoreHandleType::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD => {
                writer.write_all(b", handle: ")?;
                CudaDisplay::write(unsafe { &self.handle.fd }, "", 0,writer)?;
            }
            CUexternalSemaphoreHandleType::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32
            | CUexternalSemaphoreHandleType::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE
            | CUexternalSemaphoreHandleType::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE
            | CUexternalSemaphoreHandleType::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_KEYED_MUTEX
            | CUexternalSemaphoreHandleType::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_KEYED_MUTEX_KMT => {
                write_win32_handle(unsafe { mem::transmute(self.handle.win32) }, writer)?;
            }
            CUexternalSemaphoreHandleType::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT => {
                writer.write_all(b", handle: ")?;
                CudaDisplay::write(unsafe { &self.handle.win32.handle }, "", 0,writer)?;
            }
            CUexternalSemaphoreHandleType::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_NVSCISYNC => {
                writer.write_all(b", handle: ")?;
                CudaDisplay::write(unsafe { &self.handle.nvSciSyncObj }, "", 0,writer)?;
            }
            _ => {
                writer.write_all(b", flags: ")?;
                CudaDisplay::write(&self.flags, "", 0,writer)?;
                return writer.write_all(b", ... }")
            }
        }
        writer.write_all(b", flags: ")?;
        CudaDisplay::write(&self.flags, "", 0, writer)?;
        writer.write_all(b" }")
    }
}

impl CudaDisplay for CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS_st__bindgen_ty_1__bindgen_ty_2 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(b"{ fence: ")?;
        CudaDisplay::write(&unsafe { self.fence }, "", 0, writer)?;
        writer.write_all(b" }")
    }
}

impl CudaDisplay for CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS_st__bindgen_ty_1__bindgen_ty_2 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(b"{ fence: ")?;
        CudaDisplay::write(&unsafe { self.fence }, "", 0, writer)?;
        writer.write_all(b" }")
    }
}

impl CudaDisplay for CUgraphNodeParams_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        _writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        todo!()
    }
}

impl CudaDisplay for CUeglFrame_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        _writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        todo!()
    }
}

impl CudaDisplay for CUdevResource_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        _writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        todo!()
    }
}

impl CudaDisplay for CUlaunchAttribute_st {
    fn write(
        &self,
        fn_name: &'static str,
        index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write_launch_attribute(writer, fn_name, index, self.id, self.value)
    }
}

impl<T: CudaDisplay + 'static> CudaDisplay for *mut T {
    fn write(
        &self,
        fn_name: &'static str,
        index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        CudaDisplay::write(&self.cast_const(), fn_name, index, writer)
    }
}

impl<T: CudaDisplay + 'static> CudaDisplay for *const T {
    fn write(
        &self,
        fn_name: &'static str,
        index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        if *self == ptr::null() {
            writer.write_all(b"NULL")
        } else {
            if fn_name.len() > 2
                && fn_name.starts_with("cu")
                && fn_name.as_bytes()[2].is_ascii_lowercase()
                && (TypeId::of::<T>() == TypeId::of::<f32>()
                    || TypeId::of::<T>() == TypeId::of::<f64>())
            {
                CudaDisplay::write(&self.cast::<c_void>(), fn_name, index, writer)
            } else {
                let this: &T = unsafe { &**self };
                this.write(fn_name, index, writer)
            }
        }
    }
}

impl<T: CudaDisplay, const N: usize> CudaDisplay for [T; N] {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(b"[")?;
        for i in 0..N {
            CudaDisplay::write(&self[i], "", 0, writer)?;
            if i != N - 1 {
                writer.write_all(b", ")?;
            }
        }
        writer.write_all(b"]")
    }
}

impl<const N: usize> CudaDisplay for [i8; N] {
    fn write(
        &self,
        fn_name: &'static str,
        index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        let slice = unsafe { std::slice::from_raw_parts(self.as_ptr().cast::<u8>(), N) };
        match CStr::from_bytes_until_nul(slice) {
            Ok(cstr) => writer.write_all(cstr.to_bytes()),
            Err(_) => CudaDisplay::write(
                slice,
                fn_name,
                index,
                writer,
            ),
        }
    }
}

impl<T: CudaDisplay> CudaDisplay for [T] {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(b"[")?;
        for i in 0..self.len() {
            CudaDisplay::write(&self[i], "", 0, writer)?;
            if i != self.len() - 1 {
                writer.write_all(b", ")?;
            }
        }
        writer.write_all(b"]")
    }
}

impl CudaDisplay for CUarrayMapInfo_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        _writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        todo!()
    }
}

impl CudaDisplay for CUexecAffinityParam_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        _writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        todo!()
    }
}

impl CudaDisplay for *mut cuda_types::cudnn9::cudnnRuntimeTag_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        _writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        todo!()
    }
}

#[allow(non_snake_case)]
pub fn write_cuGraphKernelNodeGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: CUgraphNode,
    attr: CUkernelNodeAttrID,
    value_out: *mut CUkernelNodeAttrValue,
) -> std::io::Result<()> {
    writer.write_all(b"(hNode: ")?;
    CudaDisplay::write(&hNode, "cuGraphKernelNodeGetAttribute", 0, writer)?;
    writer.write_all(b", attr: ")?;
    CudaDisplay::write(&attr, "cuGraphKernelNodeGetAttribute", 1, writer)?;
    writer.write_all(b", value_out: ")?;
    write_launch_attribute(writer, "cuGraphKernelNodeGetAttribute", 2, attr, unsafe {
        *value_out
    })?;
    writer.write_all(b") ")
}

#[allow(non_snake_case)]
pub fn write_cuGraphKernelNodeSetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: CUgraphNode,
    attr: CUkernelNodeAttrID,
    value_out: *const CUkernelNodeAttrValue,
) -> std::io::Result<()> {
    write_cuGraphKernelNodeGetAttribute(writer, hNode, attr, value_out as *mut _)
}

#[allow(non_snake_case)]
pub fn write_cuPointerGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    data: *mut ::core::ffi::c_void,
    attribute: cuda_types::cuda::CUpointer_attribute,
    ptr: cuda_types::cuda::CUdeviceptr,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(data), ": ").as_bytes())?;
    write_attribute(writer, attribute, data)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attribute), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attribute, "cuPointerGetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ptr, "cuPointerGetAttribute", arg_idx, writer)?;
    writer.write_all(b")")
}

#[allow(non_snake_case)]
pub fn write_cuPointerGetAttributes(
    writer: &mut (impl std::io::Write + ?Sized),
    numAttributes: ::core::ffi::c_uint,
    attributes: *mut cuda_types::cuda::CUpointer_attribute,
    data: *mut *mut ::core::ffi::c_void,
    ptr: cuda_types::cuda::CUdeviceptr,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(numAttributes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numAttributes,
        "cuPointerGetAttributes",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attributes), ": ").as_bytes())?;
    let attributes = unsafe { std::slice::from_raw_parts(attributes, numAttributes as usize) };
    crate::CudaDisplay::write(attributes, "cuPointerGetAttributes", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(data), ": ").as_bytes())?;
    let data = unsafe { std::slice::from_raw_parts(data, numAttributes as usize) };
    writer.write_all(b"[")?;
    for (i, data_ptr) in data.iter().copied().enumerate() {
        if i != 0 {
            writer.write_all(b", ")?;
        }
        write_attribute(writer, attributes[i], data_ptr)?;
    }
    writer.write_all(b"]")?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ptr, "cuPointerGetAttributes", arg_idx, writer)?;
    writer.write_all(b")")
}

fn write_attribute(
    writer: &mut (impl std::io::Write + ?Sized),
    attribute: cuda_types::cuda::CUpointer_attribute,
    data:  *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    match attribute {
        cuda_types::cuda::CUpointer_attribute::CU_POINTER_ATTRIBUTE_CONTEXT => {
            CudaDisplay::write(unsafe { &*(data as *const cuda_types::cuda::CUcontext) }, "", 0, writer)
        }
        cuda_types::cuda::CUpointer_attribute::CU_POINTER_ATTRIBUTE_MEMORY_TYPE => {
            CudaDisplay::write(unsafe { &*(data as *const cuda_types::cuda::CUmemorytype) }, "", 0, writer)
        }
        cuda_types::cuda::CUpointer_attribute::CU_POINTER_ATTRIBUTE_DEVICE_POINTER => {
            CudaDisplay::write(unsafe { &*(data as *const cuda_types::cuda::CUdeviceptr) }, "", 0, writer)
        }
        cuda_types::cuda::CUpointer_attribute::CU_POINTER_ATTRIBUTE_HOST_POINTER => {
            CudaDisplay::write(unsafe { &*(data as *const *mut ::core::ffi::c_void) }, "", 0, writer)
        }
        cuda_types::cuda::CUpointer_attribute::CU_POINTER_ATTRIBUTE_P2P_TOKENS => {
            CudaDisplay::write(unsafe { &*(data as *const cuda_types::cuda::CUDA_POINTER_ATTRIBUTE_P2P_TOKENS) }, "", 0, writer)
        }
        cuda_types::cuda::CUpointer_attribute::CU_POINTER_ATTRIBUTE_SYNC_MEMOPS => {
            CudaDisplay::write(unsafe { &*(data as *const bool) }, "", 0, writer)
        }
        cuda_types::cuda::CUpointer_attribute::CU_POINTER_ATTRIBUTE_BUFFER_ID => {
            CudaDisplay::write(unsafe { &*(data as *const c_ulonglong) }, "", 0, writer)
        }
        cuda_types::cuda::CUpointer_attribute::CU_POINTER_ATTRIBUTE_IS_MANAGED => {
            CudaDisplay::write(unsafe { &*(data as *const bool) }, "", 0, writer)
        }
        cuda_types::cuda::CUpointer_attribute::CU_POINTER_ATTRIBUTE_DEVICE_ORDINAL => {
            CudaDisplay::write(unsafe { &*(data as *const i32) }, "", 0, writer)
        }
        cuda_types::cuda::CUpointer_attribute::CU_POINTER_ATTRIBUTE_IS_LEGACY_CUDA_IPC_CAPABLE => {
            CudaDisplay::write(unsafe { &*(data as *const bool) }, "", 0, writer)
        }
        cuda_types::cuda::CUpointer_attribute::CU_POINTER_ATTRIBUTE_RANGE_START_ADDR => {
            CudaDisplay::write(unsafe { &*(data as *const usize) }, "", 0, writer)
        }
        cuda_types::cuda::CUpointer_attribute::CU_POINTER_ATTRIBUTE_RANGE_SIZE => {
            CudaDisplay::write(unsafe { &*(data as *const usize) }, "", 0, writer)
        }
        cuda_types::cuda::CUpointer_attribute::CU_POINTER_ATTRIBUTE_MAPPED => {
            CudaDisplay::write(unsafe { &*(data as *const bool) }, "", 0, writer)
        }
        cuda_types::cuda::CUpointer_attribute::CU_POINTER_ATTRIBUTE_ALLOWED_HANDLE_TYPES => {
            CudaDisplay::write(unsafe { &*(data as *const CUmemAllocationHandleType) }, "", 0, writer)
        }
        cuda_types::cuda::CUpointer_attribute::CU_POINTER_ATTRIBUTE_IS_GPU_DIRECT_RDMA_CAPABLE => {
            CudaDisplay::write(unsafe { &*(data as *const bool) }, "", 0, writer)
        }
        cuda_types::cuda::CUpointer_attribute::CU_POINTER_ATTRIBUTE_ACCESS_FLAGS => {
            CudaDisplay::write(unsafe { &*(data as *const CUDA_POINTER_ATTRIBUTE_ACCESS_FLAGS) }, "", 0, writer)
        }
        cuda_types::cuda::CUpointer_attribute::CU_POINTER_ATTRIBUTE_MEMPOOL_HANDLE => {
            CudaDisplay::write(unsafe { &*(data as *const CUmemoryPool) }, "", 0, writer)
        }
        cuda_types::cuda::CUpointer_attribute::CU_POINTER_ATTRIBUTE_MAPPING_SIZE => {
            CudaDisplay::write(unsafe { &*(data as *const usize) }, "", 0, writer)
        }
        cuda_types::cuda::CUpointer_attribute::CU_POINTER_ATTRIBUTE_MAPPING_BASE_ADDR => {
            CudaDisplay::write(unsafe { &*(data as *const usize) }, "", 0, writer)
        }
        // We don't know the type of the result
        // cuda_types::cuda::CUpointer_attribute::CU_POINTER_ATTRIBUTE_MEMORY_BLOCK_ID => {
        //     CudaDisplay::write(unsafe { &*(data as *const ???) }, "", 0, writer)
        // }
        cuda_types::cuda::CUpointer_attribute::CU_POINTER_ATTRIBUTE_IS_HW_DECOMPRESS_CAPABLE => {
            CudaDisplay::write(unsafe { &*(data as *const bool) }, "", 0, writer)
        }
        _ => writer.write_all(b"UNKNOWN ATTRIBUTE"),
    }
}

#[allow(non_snake_case)]
pub fn write_cuStreamGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: CUstream,
    attr: CUstreamAttrID,
    value_out: *mut CUstreamAttrValue,
) -> std::io::Result<()> {
    writer.write_all(b"(hStream: ")?;
    CudaDisplay::write(&hStream, "cuStreamGetAttribute", 0, writer)?;
    writer.write_all(b", attr: ")?;
    CudaDisplay::write(&attr, "cuStreamGetAttribute", 1, writer)?;
    writer.write_all(b", value_out: ")?;
    write_launch_attribute(writer, "cuStreamGetAttribute", 2, attr, unsafe {
        *value_out
    })?;
    writer.write_all(b") ")
}

fn write_launch_attribute(
    writer: &mut (impl std::io::Write + ?Sized),
    fn_name: &'static str,
    index: usize,
    attribute: CUlaunchAttributeID,
    value: CUlaunchAttributeValue,
) -> std::io::Result<()> {
    match attribute {
        CUlaunchAttributeID::CU_LAUNCH_ATTRIBUTE_ACCESS_POLICY_WINDOW => {
            writer.write_all(b"CU_LAUNCH_ATTRIBUTE_ACCESS_POLICY_WINDOW = ")?;
            CudaDisplay::write(unsafe { &value.accessPolicyWindow }, fn_name, index, writer)
        }
        CUlaunchAttributeID::CU_LAUNCH_ATTRIBUTE_COOPERATIVE => {
            writer.write_all(b"CU_LAUNCH_ATTRIBUTE_COOPERATIVE = ")?;
            CudaDisplay::write(unsafe { &value.cooperative }, fn_name, index, writer)
        }
        CUlaunchAttributeID::CU_LAUNCH_ATTRIBUTE_SYNCHRONIZATION_POLICY => {
            writer.write_all(b"CU_LAUNCH_ATTRIBUTE_SYNCHRONIZATION_POLICY = ")?;
            CudaDisplay::write(unsafe { &value.syncPolicy }, fn_name, index, writer)
        }
        CUlaunchAttributeID::CU_LAUNCH_ATTRIBUTE_CLUSTER_DIMENSION => {
            writer.write_all(b"CU_LAUNCH_ATTRIBUTE_CLUSTER_DIMENSION = ")?;
            CudaDisplay::write(unsafe { &value.clusterDim }, fn_name, index, writer)
        }
        CUlaunchAttributeID::CU_LAUNCH_ATTRIBUTE_CLUSTER_SCHEDULING_POLICY_PREFERENCE => {
            writer.write_all(b"CU_LAUNCH_ATTRIBUTE_CLUSTER_SCHEDULING_POLICY_PREFERENCE = ")?;
            CudaDisplay::write(
                unsafe { &value.clusterSchedulingPolicyPreference },
                fn_name,
                index,
                writer,
            )
        }
        CUlaunchAttributeID::CU_LAUNCH_ATTRIBUTE_PROGRAMMATIC_STREAM_SERIALIZATION => {
            writer.write_all(b"CU_LAUNCH_ATTRIBUTE_PROGRAMMATIC_STREAM_SERIALIZATION = ")?;
            CudaDisplay::write(
                unsafe { &value.programmaticStreamSerializationAllowed },
                fn_name,
                index,
                writer,
            )
        }
        CUlaunchAttributeID::CU_LAUNCH_ATTRIBUTE_PROGRAMMATIC_EVENT => {
            writer.write_all(b"CU_LAUNCH_ATTRIBUTE_PROGRAMMATIC_EVENT = ")?;
            CudaDisplay::write(unsafe { &value.programmaticEvent }, fn_name, index, writer)
        }
        CUlaunchAttributeID::CU_LAUNCH_ATTRIBUTE_PRIORITY => {
            writer.write_all(b"CU_LAUNCH_ATTRIBUTE_PRIORITY = ")?;
            CudaDisplay::write(unsafe { &value.priority }, fn_name, index, writer)
        }
        CUlaunchAttributeID::CU_LAUNCH_ATTRIBUTE_MEM_SYNC_DOMAIN_MAP => {
            writer.write_all(b"CU_LAUNCH_ATTRIBUTE_MEM_SYNC_DOMAIN_MAP = ")?;
            CudaDisplay::write(unsafe { &value.memSyncDomainMap }, fn_name, index, writer)
        }
        CUlaunchAttributeID::CU_LAUNCH_ATTRIBUTE_MEM_SYNC_DOMAIN => {
            writer.write_all(b"CU_LAUNCH_ATTRIBUTE_MEM_SYNC_DOMAIN = ")?;
            CudaDisplay::write(unsafe { &value.memSyncDomain }, fn_name, index, writer)
        }
        CUlaunchAttributeID::CU_LAUNCH_ATTRIBUTE_LAUNCH_COMPLETION_EVENT => {
            writer.write_all(b"CU_LAUNCH_ATTRIBUTE_LAUNCH_COMPLETION_EVENT = ")?;
            CudaDisplay::write(
                unsafe { &value.launchCompletionEvent },
                fn_name,
                index,
                writer,
            )
        }
        CUlaunchAttributeID::CU_LAUNCH_ATTRIBUTE_DEVICE_UPDATABLE_KERNEL_NODE => {
            writer.write_all(b"CU_LAUNCH_ATTRIBUTE_DEVICE_UPDATABLE_KERNEL_NODE = ")?;
            CudaDisplay::write(
                unsafe { &value.deviceUpdatableKernelNode },
                fn_name,
                index,
                writer,
            )
        }
        _ => writer.write_all(b""),
    }
}

#[allow(non_snake_case)]
pub fn write_cuStreamGetAttribute_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: CUstream,
    attr: CUstreamAttrID,
    value_out: *mut CUstreamAttrValue,
) -> std::io::Result<()> {
    write_cuStreamGetAttribute(writer, hStream, attr, value_out)
}

#[allow(non_snake_case)]
pub fn write_cuStreamSetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: CUstream,
    attr: CUstreamAttrID,
    value_out: *const CUstreamAttrValue,
) -> std::io::Result<()> {
    write_cuStreamGetAttribute(writer, hStream, attr, value_out as *mut _)
}

#[allow(non_snake_case)]
pub fn write_cuStreamSetAttribute_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: CUstream,
    attr: CUstreamAttrID,
    value_out: *const CUstreamAttrValue,
) -> std::io::Result<()> {
    write_cuStreamSetAttribute(writer, hStream, attr, value_out)
}

#[allow(non_snake_case)]
pub fn write_cuGLGetDevices(
    _writer: &mut (impl std::io::Write + ?Sized),
    _pCudaDeviceCount: *mut ::std::os::raw::c_uint,
    _pCudaDevices: *mut CUdevice,
    _cudaDeviceCount: ::std::os::raw::c_uint,
    _deviceList: CUGLDeviceList,
) -> std::io::Result<()> {
    todo!()
}

#[allow(non_snake_case)]
pub fn write_cuGLGetDevices_v2(
    _writer: &mut (impl std::io::Write + ?Sized),
    _pCudaDeviceCount: *mut ::std::os::raw::c_uint,
    _pCudaDevices: *mut CUdevice,
    _cudaDeviceCount: ::std::os::raw::c_uint,
    _deviceList: CUGLDeviceList,
) -> std::io::Result<()> {
    todo!()
}

#[allow(non_snake_case)]
pub fn write_cudnnBackendGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    descriptor: cuda_types::cudnn9::cudnnBackendDescriptor_t,
    attributeName: cuda_types::cudnn9::cudnnBackendAttributeName_t,
    attributeType: cuda_types::cudnn9::cudnnBackendAttributeType_t,
    requestedElementCount: i64,
    elementCount: *mut i64,
    arrayOfElements: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(descriptor), ": ").as_bytes())?;
    crate::CudaDisplay::write(&descriptor, "cudnnBackendGetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attributeName), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attributeName, "cudnnBackendGetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attributeType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attributeType, "cudnnBackendGetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(requestedElementCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &requestedElementCount,
        "cudnnBackendGetAttribute",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(elementCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&elementCount, "cudnnBackendGetAttribute", arg_idx, writer)?;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(arrayOfElements), ": ").as_bytes())?;
    cudnn9_print_elements(
        writer,
        attributeType,
        unsafe { elementCount.as_ref() }
            .copied()
            .unwrap_or(requestedElementCount),
        arrayOfElements,
    )?;
    writer.write_all(b")")
}

#[allow(non_snake_case)]
pub fn write_cudnnBackendSetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    descriptor: cuda_types::cudnn9::cudnnBackendDescriptor_t,
    attributeName: cuda_types::cudnn9::cudnnBackendAttributeName_t,
    attributeType: cuda_types::cudnn9::cudnnBackendAttributeType_t,
    elementCount: i64,
    arrayOfElements: *const ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(descriptor), ": ").as_bytes())?;
    crate::CudaDisplay::write(&descriptor, "cudnnBackendSetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attributeName), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attributeName, "cudnnBackendSetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attributeType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attributeType, "cudnnBackendSetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(elementCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&elementCount, "cudnnBackendSetAttribute", arg_idx, writer)?;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(arrayOfElements), ": ").as_bytes())?;
    cudnn9_print_elements(writer, attributeType, elementCount, arrayOfElements)?;
    writer.write_all(b")")
}

fn cudnn9_print_elements(
    writer: &mut (impl std::io::Write + ?Sized),
    type_: cuda_types::cudnn9::cudnnBackendAttributeType_t,
    element_count: i64,
    array_of_elements: *const ::core::ffi::c_void,
) -> std::io::Result<()> {
    fn print_typed<T: CudaDisplay>(
        writer: &mut (impl std::io::Write + ?Sized),
        element_count: i64,
        array_of_elements: *const ::core::ffi::c_void,
    ) -> std::io::Result<()> {
        if array_of_elements.is_null() {
            return writer.write_all(b"NULL");
        }
        let elements =
            unsafe { slice::from_raw_parts(array_of_elements as *const T, element_count as usize) };
        CudaDisplay::write(elements, "", 0, writer)
    }
    match type_ {
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_HANDLE => {
            print_typed::<cuda_types::cudnn9::cudnnHandle_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_DATA_TYPE => {
            print_typed::<cuda_types::cudnn9::cudnnDataType_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_BOOLEAN => {
            print_typed::<bool>(writer, element_count, array_of_elements)
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_INT64 => {
            print_typed::<i64>(writer, element_count, array_of_elements)
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_FLOAT => {
            print_typed::<f32>(writer, element_count, array_of_elements)
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_DOUBLE => {
            print_typed::<f64>(writer, element_count, array_of_elements)
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_VOID_PTR => {
            print_typed::<*const c_void>(writer, element_count, array_of_elements)
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_CONVOLUTION_MODE => {
            print_typed::<cuda_types::cudnn9::cudnnConvolutionMode_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_HEUR_MODE => {
            print_typed::<cuda_types::cudnn9::cudnnBackendHeurMode_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_KNOB_TYPE => {
            print_typed::<cuda_types::cudnn9::cudnnBackendKnobType_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_NAN_PROPOGATION => {
            print_typed::<cuda_types::cudnn9::cudnnNanPropagation_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_NUMERICAL_NOTE => {
            print_typed::<cuda_types::cudnn9::cudnnBackendNumericalNote_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_LAYOUT_TYPE => {
            print_typed::<cuda_types::cudnn9::cudnnBackendLayoutType_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_ATTRIB_NAME => {
            print_typed::<cuda_types::cudnn9::cudnnBackendAttributeName_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_POINTWISE_MODE => {
            print_typed::<cuda_types::cudnn9::cudnnPointwiseMode_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_BACKEND_DESCRIPTOR => {
            print_typed::<cuda_types::cudnn9::cudnnBackendDescriptor_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_GENSTATS_MODE => {
            print_typed::<cuda_types::cudnn9::cudnnGenStatsMode_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_BN_FINALIZE_STATS_MODE => {
            print_typed::<cuda_types::cudnn9::cudnnBnFinalizeStatsMode_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_REDUCTION_OPERATOR_TYPE => {
            print_typed::<cuda_types::cudnn9::cudnnReduceTensorOp_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_BEHAVIOR_NOTE => {
            print_typed::<cuda_types::cudnn9::cudnnBackendBehaviorNote_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_TENSOR_REORDERING_MODE => {
            print_typed::<cuda_types::cudnn9::cudnnBackendTensorReordering_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_RESAMPLE_MODE => {
            print_typed::<cuda_types::cudnn9::cudnnResampleMode_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_PADDING_MODE => {
            print_typed::<cuda_types::cudnn9::cudnnPaddingMode_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_INT32 => {
            print_typed::<i32>(writer, element_count, array_of_elements)
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_CHAR => {
            CudaDisplay::write(&array_of_elements.cast::<i8>(), "", 0, writer)
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_SIGNAL_MODE => {
            print_typed::<cuda_types::cudnn9::cudnnSignalMode_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_FRACTION => {
            print_typed::<cuda_types::cudnn9::cudnnFraction_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_NORM_MODE => {
            print_typed::<cuda_types::cudnn9::cudnnBackendNormMode_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_NORM_FWD_PHASE => {
            print_typed::<cuda_types::cudnn9::cudnnBackendNormFwdPhase_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        cuda_types::cudnn9::cudnnBackendAttributeType_t::CUDNN_TYPE_RNG_DISTRIBUTION => {
            print_typed::<cuda_types::cudnn9::cudnnRngDistribution_t>(
                writer,
                element_count,
                array_of_elements,
            )
        }
        _ => unimplemented!(),
    }
}

fn write_nvml_value(
    writer: &mut (impl std::io::Write + ?Sized),
    type_: Option<cuda_types::nvml::nvmlValueType_t>,
    value: cuda_types::nvml::nvmlValue_t) -> std::io::Result<()> {
    match type_ {
        Some(cuda_types::nvml::nvmlValueType_t::NVML_VALUE_TYPE_DOUBLE) => {
            writer.write_fmt(format_args!("{}", unsafe { value.dVal } ))
        }
        Some(cuda_types::nvml::nvmlValueType_t::NVML_VALUE_TYPE_UNSIGNED_INT) => {
            writer.write_fmt(format_args!("{}", unsafe { value.uiVal }))
        }
        Some(cuda_types::nvml::nvmlValueType_t::NVML_VALUE_TYPE_UNSIGNED_LONG) => {
            writer.write_fmt(format_args!("{}", unsafe { value.ulVal }))
        }
        Some(cuda_types::nvml::nvmlValueType_t::NVML_VALUE_TYPE_UNSIGNED_LONG_LONG) => {
            writer.write_fmt(format_args!("{}", unsafe { value.ullVal }))
        }
        Some(cuda_types::nvml::nvmlValueType_t::NVML_VALUE_TYPE_SIGNED_LONG_LONG) => {
            writer.write_fmt(format_args!("{}", unsafe { value.sllVal }))
        }
        Some(cuda_types::nvml::nvmlValueType_t::NVML_VALUE_TYPE_SIGNED_INT) => {
            writer.write_fmt(format_args!("{}", unsafe { value.siVal }))
        }
        Some(cuda_types::nvml::nvmlValueType_t::NVML_VALUE_TYPE_UNSIGNED_SHORT) => {
            writer.write_fmt(format_args!("{}", unsafe { value.usVal }))
        }
        Some(_) | None => {
            CudaDisplay::write(&unsafe { mem::transmute::<_, [u8;8]>(value) }, "", 0, writer)
        },
    }
}

impl crate::CudaDisplay for cuda_types::nvml::nvmlSample_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(timeStamp), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.timeStamp, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(sampleValue), ": ").as_bytes())?;
        write_nvml_value(writer, None, self.sampleValue)?;
        writer.write_all(b" }")
    }
}

impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuInstanceUtilizationSample_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(vgpuInstance), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.vgpuInstance, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(timeStamp), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.timeStamp, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(smUtil), ": ").as_bytes())?;
        write_nvml_value(writer, None, self.smUtil)?;
        writer.write_all(concat!(", ", stringify!(memUtil), ": ").as_bytes())?;
        write_nvml_value(writer, None, self.memUtil)?;
        writer.write_all(concat!(", ", stringify!(encUtil), ": ").as_bytes())?;
        write_nvml_value(writer, None, self.encUtil)?;
        writer.write_all(concat!(", ", stringify!(decUtil), ": ").as_bytes())?;
        write_nvml_value(writer, None, self.decUtil)?;
        writer.write_all(b" }")
    }
}

impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuInstanceUtilizationInfo_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(timeStamp), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.timeStamp, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(vgpuInstance), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.vgpuInstance, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(smUtil), ": ").as_bytes())?;
        write_nvml_value(writer, None, self.smUtil)?;
        writer.write_all(concat!(", ", stringify!(memUtil), ": ").as_bytes())?;
        write_nvml_value(writer, None, self.memUtil)?;
        writer.write_all(concat!(", ", stringify!(encUtil), ": ").as_bytes())?;
        write_nvml_value(writer, None, self.encUtil)?;
        writer.write_all(concat!(", ", stringify!(decUtil), ": ").as_bytes())?;
        write_nvml_value(writer, None, self.decUtil)?;
        writer.write_all(concat!(", ", stringify!(jpgUtil), ": ").as_bytes())?;
        write_nvml_value(writer, None, self.jpgUtil)?;
        writer.write_all(concat!(", ", stringify!(ofaUtil), ": ").as_bytes())?;
        write_nvml_value(writer, None, self.ofaUtil)?;
        writer.write_all(b" }")
    }
}

impl crate::CudaDisplay for cuda_types::nvml::nvmlFieldValue_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(fieldId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.fieldId, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(scopeId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.scopeId, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(timestamp), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.timestamp, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(latencyUsec), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.latencyUsec, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(valueType), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.valueType, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(nvmlReturn), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.nvmlReturn, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(value), ": ").as_bytes())?;
        write_nvml_value(writer, Some(self.valueType), self.value)?;
        writer.write_all(b" }")
    }
}

impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuSchedulerSetState_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(schedulerPolicy), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.schedulerPolicy, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(enableARRMode), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.enableARRMode, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(schedulerParams), ": ").as_bytes())?;
        if self.enableARRMode != 0 {
            crate::CudaDisplay::write(&unsafe { self.schedulerParams.vgpuSchedDataWithARR }, "", 0, writer)?;
        } else {
            crate::CudaDisplay::write(&unsafe { self.schedulerParams.vgpuSchedData } , "", 0, writer)?;
        }
        writer.write_all(b" }")
    }
}

impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuSchedulerLog_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(engineId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.engineId, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(schedulerPolicy), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.schedulerPolicy, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(arrMode), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.arrMode, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(schedulerParams), ": ").as_bytes())?;        if self.arrMode != 0 {
            crate::CudaDisplay::write(&unsafe { self.schedulerParams.vgpuSchedDataWithARR }, "", 0, writer)?;
        } else {
            crate::CudaDisplay::write(&unsafe { self.schedulerParams.vgpuSchedData } , "", 0, writer)?;
        }
        writer.write_all(concat!(", ", stringify!(entriesCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.entriesCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(logEntries), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.logEntries, "", 0, writer)?;
        writer.write_all(b" }")
    }
}

impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuSchedulerGetState_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(schedulerPolicy), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.schedulerPolicy, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(arrMode), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.arrMode, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(schedulerParams), ": ").as_bytes())?;
        if self.arrMode != 0 {
            crate::CudaDisplay::write(&unsafe { self.schedulerParams.vgpuSchedDataWithARR }, "", 0, writer)?;
        } else {
            crate::CudaDisplay::write(&unsafe { self.schedulerParams.vgpuSchedData } , "", 0, writer)?;
        }
        writer.write_all(b" }")
    }
}

mod dark_api;
mod format_generated;
pub use format_generated::*;
mod format_generated_blas;
pub use format_generated_blas::*;
mod format_generated_blaslt;
pub use format_generated_blaslt::*;
mod format_generated_blaslt_internal;
pub use format_generated_blaslt_internal::*;
mod format_generated_dnn9;
pub use format_generated_dnn9::*;
mod format_generated_fft;
pub use format_generated_fft::*;
mod format_generated_nvml;
pub use format_generated_nvml::*;
mod format_generated_sparse;
pub use format_generated_sparse::*;
