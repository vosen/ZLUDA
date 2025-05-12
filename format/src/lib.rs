use cuda_types::cuda::*;
use std::{
    ffi::{c_void, CStr},
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
        write!(
            writer,
            "\"{}\"",
            unsafe { CStr::from_ptr(*self as _) }.to_string_lossy()
        )
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
                CudaDisplay::write(
                    &unsafe { mem::transmute::<_, [u8; 32]>(self.op) },
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

impl CudaDisplay for CUlaunchConfig_st {
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
        _fn_name: &'static str,
        _index: usize,
        _writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        todo!()
    }
}
impl<T: CudaDisplay> CudaDisplay for *mut T {
    fn write(
        &self,
        fn_name: &'static str,
        index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        if *self == ptr::null_mut() {
            writer.write_all(b"NULL")
        } else {
            let this: &T = unsafe { &**self };
            this.write(fn_name, index, writer)
        }
    }
}

impl<T: CudaDisplay> CudaDisplay for *const T {
    fn write(
        &self,
        fn_name: &'static str,
        index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        if *self == ptr::null() {
            writer.write_all(b"NULL")
        } else {
            let this: &T = unsafe { &**self };
            this.write(fn_name, index, writer)
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
    write_launch_attribute(writer, "cuGraphKernelNodeGetAttribute", 2, attr, value_out)?;
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
    write_launch_attribute(writer, "cuStreamGetAttribute", 2, attr, value_out)?;
    writer.write_all(b") ")
}

fn write_launch_attribute(
    writer: &mut (impl std::io::Write + ?Sized),
    fn_name: &'static str,
    index: usize,
    attribute: CUlaunchAttributeID,
    value_out: *mut CUstreamAttrValue,
) -> std::io::Result<()> {
    match attribute {
        CUlaunchAttributeID::CU_LAUNCH_ATTRIBUTE_ACCESS_POLICY_WINDOW => {
            writer.write_all(b", value_out: ")?;
            CudaDisplay::write(
                unsafe { &(*value_out).accessPolicyWindow },
                fn_name,
                index,
                writer,
            )
        }
        CUlaunchAttributeID::CU_LAUNCH_ATTRIBUTE_COOPERATIVE => {
            writer.write_all(b", value_out: ")?;
            CudaDisplay::write(unsafe { &(*value_out).cooperative }, fn_name, index, writer)
        }
        CUlaunchAttributeID::CU_LAUNCH_ATTRIBUTE_SYNCHRONIZATION_POLICY => {
            writer.write_all(b", value_out: ")?;
            CudaDisplay::write(unsafe { &(*value_out).syncPolicy }, fn_name, index, writer)
        }
        CUlaunchAttributeID::CU_LAUNCH_ATTRIBUTE_CLUSTER_DIMENSION => {
            writer.write_all(b", value_out: ")?;
            CudaDisplay::write(unsafe { &(*value_out).clusterDim }, fn_name, index, writer)
        }
        CUlaunchAttributeID::CU_LAUNCH_ATTRIBUTE_CLUSTER_SCHEDULING_POLICY_PREFERENCE => {
            writer.write_all(b", value_out: ")?;
            CudaDisplay::write(
                unsafe { &(*value_out).clusterSchedulingPolicyPreference },
                fn_name,
                index,
                writer,
            )
        }
        CUlaunchAttributeID::CU_LAUNCH_ATTRIBUTE_PROGRAMMATIC_STREAM_SERIALIZATION => {
            writer.write_all(b", value_out: ")?;
            CudaDisplay::write(
                unsafe { &(*value_out).programmaticStreamSerializationAllowed },
                fn_name,
                index,
                writer,
            )
        }
        CUlaunchAttributeID::CU_LAUNCH_ATTRIBUTE_PROGRAMMATIC_EVENT => {
            writer.write_all(b", value_out: ")?;
            CudaDisplay::write(
                unsafe { &(*value_out).programmaticEvent },
                fn_name,
                index,
                writer,
            )
        }
        CUlaunchAttributeID::CU_LAUNCH_ATTRIBUTE_PRIORITY => {
            writer.write_all(b", value_out: ")?;
            CudaDisplay::write(unsafe { &(*value_out).priority }, fn_name, index, writer)
        }
        CUlaunchAttributeID::CU_LAUNCH_ATTRIBUTE_MEM_SYNC_DOMAIN_MAP => {
            writer.write_all(b", value_out: ")?;
            CudaDisplay::write(
                unsafe { &(*value_out).memSyncDomainMap },
                fn_name,
                index,
                writer,
            )
        }
        CUlaunchAttributeID::CU_LAUNCH_ATTRIBUTE_MEM_SYNC_DOMAIN => {
            writer.write_all(b", value_out: ")?;
            CudaDisplay::write(
                unsafe { &(*value_out).memSyncDomain },
                fn_name,
                index,
                writer,
            )
        }
        CUlaunchAttributeID::CU_LAUNCH_ATTRIBUTE_LAUNCH_COMPLETION_EVENT => {
            writer.write_all(b", value_out: ")?;
            CudaDisplay::write(
                unsafe { &(*value_out).launchCompletionEvent },
                fn_name,
                index,
                writer,
            )
        }
        CUlaunchAttributeID::CU_LAUNCH_ATTRIBUTE_DEVICE_UPDATABLE_KERNEL_NODE => {
            writer.write_all(b", value_out: ")?;
            CudaDisplay::write(
                unsafe { &(*value_out).deviceUpdatableKernelNode },
                fn_name,
                index,
                writer,
            )
        }
        _ => writer.write_all(b", ... "),
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

mod format_generated;
pub use format_generated::*;
