// Generated automatically by zluda_bindgen
// DO NOT EDIT MANUALLY
#![allow(warnings)]
impl crate::CudaDisplay for cuda_types::cuda::CUdeviceptr_v2 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{:p}", self.0)
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUcontext {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{:p}", self.0)
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUmodule {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{:p}", self.0)
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUfunction {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{:p}", self.0)
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUlibrary {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{:p}", self.0)
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUkernel {
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
impl crate::CudaDisplay for cuda_types::cuda::CUarray {
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
impl crate::CudaDisplay for cuda_types::cuda::CUmipmappedArray {
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
impl crate::CudaDisplay for cuda_types::cuda::CUtexref {
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
impl crate::CudaDisplay for cuda_types::cuda::CUsurfref {
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
impl crate::CudaDisplay for cuda_types::cuda::CUevent {
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
impl crate::CudaDisplay for cuda_types::cuda::CUstream {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(writer, "{:p}", self.0)
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUgraphicsResource {
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
impl crate::CudaDisplay for cuda_types::cuda::CUexternalMemory {
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
impl crate::CudaDisplay for cuda_types::cuda::CUexternalSemaphore {
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
impl crate::CudaDisplay for cuda_types::cuda::CUgraph {
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
impl crate::CudaDisplay for cuda_types::cuda::CUgraphNode {
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
impl crate::CudaDisplay for cuda_types::cuda::CUgraphExec {
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
impl crate::CudaDisplay for cuda_types::cuda::CUmemoryPool {
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
impl crate::CudaDisplay for cuda_types::cuda::CUuserObject {
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
impl crate::CudaDisplay for cuda_types::cuda::CUgraphDeviceNode {
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
impl crate::CudaDisplay for cuda_types::cuda::CUasyncCallbackHandle {
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
impl crate::CudaDisplay for cuda_types::cuda::CUgreenCtx {
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
impl crate::CudaDisplay for cuda_types::cuda::CUmemFabricHandle_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(data), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.data, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUipcMem_flags_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUipcMem_flags_enum::CU_IPC_MEM_LAZY_ENABLE_PEER_ACCESS => {
                writer
                    .write_all(stringify!(CU_IPC_MEM_LAZY_ENABLE_PEER_ACCESS).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUmemAttach_flags_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUmemAttach_flags_enum::CU_MEM_ATTACH_GLOBAL => {
                writer.write_all(stringify!(CU_MEM_ATTACH_GLOBAL).as_bytes())
            }
            &cuda_types::cuda::CUmemAttach_flags_enum::CU_MEM_ATTACH_HOST => {
                writer.write_all(stringify!(CU_MEM_ATTACH_HOST).as_bytes())
            }
            &cuda_types::cuda::CUmemAttach_flags_enum::CU_MEM_ATTACH_SINGLE => {
                writer.write_all(stringify!(CU_MEM_ATTACH_SINGLE).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUctx_flags_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUctx_flags_enum::CU_CTX_SCHED_AUTO => {
                writer.write_all(stringify!(CU_CTX_SCHED_AUTO).as_bytes())
            }
            &cuda_types::cuda::CUctx_flags_enum::CU_CTX_SCHED_SPIN => {
                writer.write_all(stringify!(CU_CTX_SCHED_SPIN).as_bytes())
            }
            &cuda_types::cuda::CUctx_flags_enum::CU_CTX_SCHED_YIELD => {
                writer.write_all(stringify!(CU_CTX_SCHED_YIELD).as_bytes())
            }
            &cuda_types::cuda::CUctx_flags_enum::CU_CTX_SCHED_BLOCKING_SYNC => {
                writer.write_all(stringify!(CU_CTX_SCHED_BLOCKING_SYNC).as_bytes())
            }
            &cuda_types::cuda::CUctx_flags_enum::CU_CTX_BLOCKING_SYNC => {
                writer.write_all(stringify!(CU_CTX_BLOCKING_SYNC).as_bytes())
            }
            &cuda_types::cuda::CUctx_flags_enum::CU_CTX_SCHED_MASK => {
                writer.write_all(stringify!(CU_CTX_SCHED_MASK).as_bytes())
            }
            &cuda_types::cuda::CUctx_flags_enum::CU_CTX_MAP_HOST => {
                writer.write_all(stringify!(CU_CTX_MAP_HOST).as_bytes())
            }
            &cuda_types::cuda::CUctx_flags_enum::CU_CTX_LMEM_RESIZE_TO_MAX => {
                writer.write_all(stringify!(CU_CTX_LMEM_RESIZE_TO_MAX).as_bytes())
            }
            &cuda_types::cuda::CUctx_flags_enum::CU_CTX_COREDUMP_ENABLE => {
                writer.write_all(stringify!(CU_CTX_COREDUMP_ENABLE).as_bytes())
            }
            &cuda_types::cuda::CUctx_flags_enum::CU_CTX_USER_COREDUMP_ENABLE => {
                writer.write_all(stringify!(CU_CTX_USER_COREDUMP_ENABLE).as_bytes())
            }
            &cuda_types::cuda::CUctx_flags_enum::CU_CTX_SYNC_MEMOPS => {
                writer.write_all(stringify!(CU_CTX_SYNC_MEMOPS).as_bytes())
            }
            &cuda_types::cuda::CUctx_flags_enum::CU_CTX_FLAGS_MASK => {
                writer.write_all(stringify!(CU_CTX_FLAGS_MASK).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUevent_sched_flags_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUevent_sched_flags_enum::CU_EVENT_SCHED_AUTO => {
                writer.write_all(stringify!(CU_EVENT_SCHED_AUTO).as_bytes())
            }
            &cuda_types::cuda::CUevent_sched_flags_enum::CU_EVENT_SCHED_SPIN => {
                writer.write_all(stringify!(CU_EVENT_SCHED_SPIN).as_bytes())
            }
            &cuda_types::cuda::CUevent_sched_flags_enum::CU_EVENT_SCHED_YIELD => {
                writer.write_all(stringify!(CU_EVENT_SCHED_YIELD).as_bytes())
            }
            &cuda_types::cuda::CUevent_sched_flags_enum::CU_EVENT_SCHED_BLOCKING_SYNC => {
                writer.write_all(stringify!(CU_EVENT_SCHED_BLOCKING_SYNC).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUstream_flags_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUstream_flags_enum::CU_STREAM_DEFAULT => {
                writer.write_all(stringify!(CU_STREAM_DEFAULT).as_bytes())
            }
            &cuda_types::cuda::CUstream_flags_enum::CU_STREAM_NON_BLOCKING => {
                writer.write_all(stringify!(CU_STREAM_NON_BLOCKING).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUevent_flags_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUevent_flags_enum::CU_EVENT_DEFAULT => {
                writer.write_all(stringify!(CU_EVENT_DEFAULT).as_bytes())
            }
            &cuda_types::cuda::CUevent_flags_enum::CU_EVENT_BLOCKING_SYNC => {
                writer.write_all(stringify!(CU_EVENT_BLOCKING_SYNC).as_bytes())
            }
            &cuda_types::cuda::CUevent_flags_enum::CU_EVENT_DISABLE_TIMING => {
                writer.write_all(stringify!(CU_EVENT_DISABLE_TIMING).as_bytes())
            }
            &cuda_types::cuda::CUevent_flags_enum::CU_EVENT_INTERPROCESS => {
                writer.write_all(stringify!(CU_EVENT_INTERPROCESS).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUevent_record_flags_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUevent_record_flags_enum::CU_EVENT_RECORD_DEFAULT => {
                writer.write_all(stringify!(CU_EVENT_RECORD_DEFAULT).as_bytes())
            }
            &cuda_types::cuda::CUevent_record_flags_enum::CU_EVENT_RECORD_EXTERNAL => {
                writer.write_all(stringify!(CU_EVENT_RECORD_EXTERNAL).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUevent_wait_flags_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUevent_wait_flags_enum::CU_EVENT_WAIT_DEFAULT => {
                writer.write_all(stringify!(CU_EVENT_WAIT_DEFAULT).as_bytes())
            }
            &cuda_types::cuda::CUevent_wait_flags_enum::CU_EVENT_WAIT_EXTERNAL => {
                writer.write_all(stringify!(CU_EVENT_WAIT_EXTERNAL).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUstreamWaitValue_flags_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUstreamWaitValue_flags_enum::CU_STREAM_WAIT_VALUE_GEQ => {
                writer.write_all(stringify!(CU_STREAM_WAIT_VALUE_GEQ).as_bytes())
            }
            &cuda_types::cuda::CUstreamWaitValue_flags_enum::CU_STREAM_WAIT_VALUE_EQ => {
                writer.write_all(stringify!(CU_STREAM_WAIT_VALUE_EQ).as_bytes())
            }
            &cuda_types::cuda::CUstreamWaitValue_flags_enum::CU_STREAM_WAIT_VALUE_AND => {
                writer.write_all(stringify!(CU_STREAM_WAIT_VALUE_AND).as_bytes())
            }
            &cuda_types::cuda::CUstreamWaitValue_flags_enum::CU_STREAM_WAIT_VALUE_NOR => {
                writer.write_all(stringify!(CU_STREAM_WAIT_VALUE_NOR).as_bytes())
            }
            &cuda_types::cuda::CUstreamWaitValue_flags_enum::CU_STREAM_WAIT_VALUE_FLUSH => {
                writer.write_all(stringify!(CU_STREAM_WAIT_VALUE_FLUSH).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUstreamWriteValue_flags_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUstreamWriteValue_flags_enum::CU_STREAM_WRITE_VALUE_DEFAULT => {
                writer.write_all(stringify!(CU_STREAM_WRITE_VALUE_DEFAULT).as_bytes())
            }
            &cuda_types::cuda::CUstreamWriteValue_flags_enum::CU_STREAM_WRITE_VALUE_NO_MEMORY_BARRIER => {
                writer
                    .write_all(
                        stringify!(CU_STREAM_WRITE_VALUE_NO_MEMORY_BARRIER).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUstreamBatchMemOpType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUstreamBatchMemOpType_enum::CU_STREAM_MEM_OP_WAIT_VALUE_32 => {
                writer.write_all(stringify!(CU_STREAM_MEM_OP_WAIT_VALUE_32).as_bytes())
            }
            &cuda_types::cuda::CUstreamBatchMemOpType_enum::CU_STREAM_MEM_OP_WRITE_VALUE_32 => {
                writer.write_all(stringify!(CU_STREAM_MEM_OP_WRITE_VALUE_32).as_bytes())
            }
            &cuda_types::cuda::CUstreamBatchMemOpType_enum::CU_STREAM_MEM_OP_WAIT_VALUE_64 => {
                writer.write_all(stringify!(CU_STREAM_MEM_OP_WAIT_VALUE_64).as_bytes())
            }
            &cuda_types::cuda::CUstreamBatchMemOpType_enum::CU_STREAM_MEM_OP_WRITE_VALUE_64 => {
                writer.write_all(stringify!(CU_STREAM_MEM_OP_WRITE_VALUE_64).as_bytes())
            }
            &cuda_types::cuda::CUstreamBatchMemOpType_enum::CU_STREAM_MEM_OP_BARRIER => {
                writer.write_all(stringify!(CU_STREAM_MEM_OP_BARRIER).as_bytes())
            }
            &cuda_types::cuda::CUstreamBatchMemOpType_enum::CU_STREAM_MEM_OP_FLUSH_REMOTE_WRITES => {
                writer
                    .write_all(
                        stringify!(CU_STREAM_MEM_OP_FLUSH_REMOTE_WRITES).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUstreamMemoryBarrier_flags_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUstreamMemoryBarrier_flags_enum::CU_STREAM_MEMORY_BARRIER_TYPE_SYS => {
                writer
                    .write_all(stringify!(CU_STREAM_MEMORY_BARRIER_TYPE_SYS).as_bytes())
            }
            &cuda_types::cuda::CUstreamMemoryBarrier_flags_enum::CU_STREAM_MEMORY_BARRIER_TYPE_GPU => {
                writer
                    .write_all(stringify!(CU_STREAM_MEMORY_BARRIER_TYPE_GPU).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay
for cuda_types::cuda::CUstreamBatchMemOpParams_union_CUstreamMemOpFlushRemoteWritesParams_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(operation), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.operation, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(flags), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.flags, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay
for cuda_types::cuda::CUstreamBatchMemOpParams_union_CUstreamMemOpMemoryBarrierParams_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(operation), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.operation, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(flags), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.flags, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_BATCH_MEM_OP_NODE_PARAMS_v1_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(ctx), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.ctx, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(count), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.count, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(paramArray), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.paramArray, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(flags), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.flags, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_BATCH_MEM_OP_NODE_PARAMS_v2_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(ctx), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.ctx, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(count), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.count, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(paramArray), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.paramArray, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(flags), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.flags, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUoccupancy_flags_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUoccupancy_flags_enum::CU_OCCUPANCY_DEFAULT => {
                writer.write_all(stringify!(CU_OCCUPANCY_DEFAULT).as_bytes())
            }
            &cuda_types::cuda::CUoccupancy_flags_enum::CU_OCCUPANCY_DISABLE_CACHING_OVERRIDE => {
                writer
                    .write_all(
                        stringify!(CU_OCCUPANCY_DISABLE_CACHING_OVERRIDE).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay
for cuda_types::cuda::CUstreamUpdateCaptureDependencies_flags_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUstreamUpdateCaptureDependencies_flags_enum::CU_STREAM_ADD_CAPTURE_DEPENDENCIES => {
                writer
                    .write_all(stringify!(CU_STREAM_ADD_CAPTURE_DEPENDENCIES).as_bytes())
            }
            &cuda_types::cuda::CUstreamUpdateCaptureDependencies_flags_enum::CU_STREAM_SET_CAPTURE_DEPENDENCIES => {
                writer
                    .write_all(stringify!(CU_STREAM_SET_CAPTURE_DEPENDENCIES).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUasyncNotificationType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUasyncNotificationType_enum::CU_ASYNC_NOTIFICATION_TYPE_OVER_BUDGET => {
                writer
                    .write_all(
                        stringify!(CU_ASYNC_NOTIFICATION_TYPE_OVER_BUDGET).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay
for cuda_types::cuda::CUasyncNotificationInfo_st__bindgen_ty_1__bindgen_ty_1 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(bytesOverBudget), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.bytesOverBudget, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUasyncCallback {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(
            writer,
            "{:p}",
            unsafe {
                std::mem::transmute::<
                    cuda_types::cuda::CUasyncCallback,
                    *mut ::std::ffi::c_void,
                >(*self)
            },
        )
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUarray_format_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_UNSIGNED_INT8 => {
                writer.write_all(stringify!(CU_AD_FORMAT_UNSIGNED_INT8).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_UNSIGNED_INT16 => {
                writer.write_all(stringify!(CU_AD_FORMAT_UNSIGNED_INT16).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_UNSIGNED_INT32 => {
                writer.write_all(stringify!(CU_AD_FORMAT_UNSIGNED_INT32).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_SIGNED_INT8 => {
                writer.write_all(stringify!(CU_AD_FORMAT_SIGNED_INT8).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_SIGNED_INT16 => {
                writer.write_all(stringify!(CU_AD_FORMAT_SIGNED_INT16).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_SIGNED_INT32 => {
                writer.write_all(stringify!(CU_AD_FORMAT_SIGNED_INT32).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_HALF => {
                writer.write_all(stringify!(CU_AD_FORMAT_HALF).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_FLOAT => {
                writer.write_all(stringify!(CU_AD_FORMAT_FLOAT).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_NV12 => {
                writer.write_all(stringify!(CU_AD_FORMAT_NV12).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_UNORM_INT8X1 => {
                writer.write_all(stringify!(CU_AD_FORMAT_UNORM_INT8X1).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_UNORM_INT8X2 => {
                writer.write_all(stringify!(CU_AD_FORMAT_UNORM_INT8X2).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_UNORM_INT8X4 => {
                writer.write_all(stringify!(CU_AD_FORMAT_UNORM_INT8X4).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_UNORM_INT16X1 => {
                writer.write_all(stringify!(CU_AD_FORMAT_UNORM_INT16X1).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_UNORM_INT16X2 => {
                writer.write_all(stringify!(CU_AD_FORMAT_UNORM_INT16X2).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_UNORM_INT16X4 => {
                writer.write_all(stringify!(CU_AD_FORMAT_UNORM_INT16X4).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_SNORM_INT8X1 => {
                writer.write_all(stringify!(CU_AD_FORMAT_SNORM_INT8X1).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_SNORM_INT8X2 => {
                writer.write_all(stringify!(CU_AD_FORMAT_SNORM_INT8X2).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_SNORM_INT8X4 => {
                writer.write_all(stringify!(CU_AD_FORMAT_SNORM_INT8X4).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_SNORM_INT16X1 => {
                writer.write_all(stringify!(CU_AD_FORMAT_SNORM_INT16X1).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_SNORM_INT16X2 => {
                writer.write_all(stringify!(CU_AD_FORMAT_SNORM_INT16X2).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_SNORM_INT16X4 => {
                writer.write_all(stringify!(CU_AD_FORMAT_SNORM_INT16X4).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_BC1_UNORM => {
                writer.write_all(stringify!(CU_AD_FORMAT_BC1_UNORM).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_BC1_UNORM_SRGB => {
                writer.write_all(stringify!(CU_AD_FORMAT_BC1_UNORM_SRGB).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_BC2_UNORM => {
                writer.write_all(stringify!(CU_AD_FORMAT_BC2_UNORM).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_BC2_UNORM_SRGB => {
                writer.write_all(stringify!(CU_AD_FORMAT_BC2_UNORM_SRGB).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_BC3_UNORM => {
                writer.write_all(stringify!(CU_AD_FORMAT_BC3_UNORM).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_BC3_UNORM_SRGB => {
                writer.write_all(stringify!(CU_AD_FORMAT_BC3_UNORM_SRGB).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_BC4_UNORM => {
                writer.write_all(stringify!(CU_AD_FORMAT_BC4_UNORM).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_BC4_SNORM => {
                writer.write_all(stringify!(CU_AD_FORMAT_BC4_SNORM).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_BC5_UNORM => {
                writer.write_all(stringify!(CU_AD_FORMAT_BC5_UNORM).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_BC5_SNORM => {
                writer.write_all(stringify!(CU_AD_FORMAT_BC5_SNORM).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_BC6H_UF16 => {
                writer.write_all(stringify!(CU_AD_FORMAT_BC6H_UF16).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_BC6H_SF16 => {
                writer.write_all(stringify!(CU_AD_FORMAT_BC6H_SF16).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_BC7_UNORM => {
                writer.write_all(stringify!(CU_AD_FORMAT_BC7_UNORM).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_BC7_UNORM_SRGB => {
                writer.write_all(stringify!(CU_AD_FORMAT_BC7_UNORM_SRGB).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_P010 => {
                writer.write_all(stringify!(CU_AD_FORMAT_P010).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_P016 => {
                writer.write_all(stringify!(CU_AD_FORMAT_P016).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_NV16 => {
                writer.write_all(stringify!(CU_AD_FORMAT_NV16).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_P210 => {
                writer.write_all(stringify!(CU_AD_FORMAT_P210).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_P216 => {
                writer.write_all(stringify!(CU_AD_FORMAT_P216).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_YUY2 => {
                writer.write_all(stringify!(CU_AD_FORMAT_YUY2).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_Y210 => {
                writer.write_all(stringify!(CU_AD_FORMAT_Y210).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_Y216 => {
                writer.write_all(stringify!(CU_AD_FORMAT_Y216).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_AYUV => {
                writer.write_all(stringify!(CU_AD_FORMAT_AYUV).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_Y410 => {
                writer.write_all(stringify!(CU_AD_FORMAT_Y410).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_Y416 => {
                writer.write_all(stringify!(CU_AD_FORMAT_Y416).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_Y444_PLANAR8 => {
                writer.write_all(stringify!(CU_AD_FORMAT_Y444_PLANAR8).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_Y444_PLANAR10 => {
                writer.write_all(stringify!(CU_AD_FORMAT_Y444_PLANAR10).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_YUV444_8bit_SemiPlanar => {
                writer
                    .write_all(
                        stringify!(CU_AD_FORMAT_YUV444_8bit_SemiPlanar).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_YUV444_16bit_SemiPlanar => {
                writer
                    .write_all(
                        stringify!(CU_AD_FORMAT_YUV444_16bit_SemiPlanar).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_UNORM_INT_101010_2 => {
                writer.write_all(stringify!(CU_AD_FORMAT_UNORM_INT_101010_2).as_bytes())
            }
            &cuda_types::cuda::CUarray_format_enum::CU_AD_FORMAT_MAX => {
                writer.write_all(stringify!(CU_AD_FORMAT_MAX).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUaddress_mode_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUaddress_mode_enum::CU_TR_ADDRESS_MODE_WRAP => {
                writer.write_all(stringify!(CU_TR_ADDRESS_MODE_WRAP).as_bytes())
            }
            &cuda_types::cuda::CUaddress_mode_enum::CU_TR_ADDRESS_MODE_CLAMP => {
                writer.write_all(stringify!(CU_TR_ADDRESS_MODE_CLAMP).as_bytes())
            }
            &cuda_types::cuda::CUaddress_mode_enum::CU_TR_ADDRESS_MODE_MIRROR => {
                writer.write_all(stringify!(CU_TR_ADDRESS_MODE_MIRROR).as_bytes())
            }
            &cuda_types::cuda::CUaddress_mode_enum::CU_TR_ADDRESS_MODE_BORDER => {
                writer.write_all(stringify!(CU_TR_ADDRESS_MODE_BORDER).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUfilter_mode_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUfilter_mode_enum::CU_TR_FILTER_MODE_POINT => {
                writer.write_all(stringify!(CU_TR_FILTER_MODE_POINT).as_bytes())
            }
            &cuda_types::cuda::CUfilter_mode_enum::CU_TR_FILTER_MODE_LINEAR => {
                writer.write_all(stringify!(CU_TR_FILTER_MODE_LINEAR).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUdevice_attribute_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_BLOCK => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_BLOCK).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_X => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_X).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_Y => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_Y).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_Z => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_Z).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_X => {
                writer
                    .write_all(stringify!(CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_X).as_bytes())
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_Y => {
                writer
                    .write_all(stringify!(CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_Y).as_bytes())
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_Z => {
                writer
                    .write_all(stringify!(CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_Z).as_bytes())
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_BLOCK => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_BLOCK)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_SHARED_MEMORY_PER_BLOCK => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_SHARED_MEMORY_PER_BLOCK)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_TOTAL_CONSTANT_MEMORY => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_TOTAL_CONSTANT_MEMORY).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_WARP_SIZE => {
                writer.write_all(stringify!(CU_DEVICE_ATTRIBUTE_WARP_SIZE).as_bytes())
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAX_PITCH => {
                writer.write_all(stringify!(CU_DEVICE_ATTRIBUTE_MAX_PITCH).as_bytes())
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAX_REGISTERS_PER_BLOCK => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAX_REGISTERS_PER_BLOCK)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_REGISTERS_PER_BLOCK => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_REGISTERS_PER_BLOCK).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_CLOCK_RATE => {
                writer.write_all(stringify!(CU_DEVICE_ATTRIBUTE_CLOCK_RATE).as_bytes())
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_TEXTURE_ALIGNMENT => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_TEXTURE_ALIGNMENT).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_GPU_OVERLAP => {
                writer.write_all(stringify!(CU_DEVICE_ATTRIBUTE_GPU_OVERLAP).as_bytes())
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MULTIPROCESSOR_COUNT => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MULTIPROCESSOR_COUNT).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_KERNEL_EXEC_TIMEOUT => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_KERNEL_EXEC_TIMEOUT).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_INTEGRATED => {
                writer.write_all(stringify!(CU_DEVICE_ATTRIBUTE_INTEGRATED).as_bytes())
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_CAN_MAP_HOST_MEMORY => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_CAN_MAP_HOST_MEMORY).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_COMPUTE_MODE => {
                writer.write_all(stringify!(CU_DEVICE_ATTRIBUTE_COMPUTE_MODE).as_bytes())
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_WIDTH => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_WIDTH)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_WIDTH => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_WIDTH)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_HEIGHT => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_HEIGHT)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_WIDTH => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_WIDTH)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_HEIGHT => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_HEIGHT)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_DEPTH => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_DEPTH)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LAYERED_WIDTH => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LAYERED_WIDTH)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LAYERED_HEIGHT => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LAYERED_HEIGHT)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LAYERED_LAYERS => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LAYERED_LAYERS)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_ARRAY_WIDTH => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_ARRAY_WIDTH)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_ARRAY_HEIGHT => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_ARRAY_HEIGHT)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_ARRAY_NUMSLICES => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_ARRAY_NUMSLICES)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_SURFACE_ALIGNMENT => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_SURFACE_ALIGNMENT).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_CONCURRENT_KERNELS => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_CONCURRENT_KERNELS).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_ECC_ENABLED => {
                writer.write_all(stringify!(CU_DEVICE_ATTRIBUTE_ECC_ENABLED).as_bytes())
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_PCI_BUS_ID => {
                writer.write_all(stringify!(CU_DEVICE_ATTRIBUTE_PCI_BUS_ID).as_bytes())
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_PCI_DEVICE_ID => {
                writer
                    .write_all(stringify!(CU_DEVICE_ATTRIBUTE_PCI_DEVICE_ID).as_bytes())
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_TCC_DRIVER => {
                writer.write_all(stringify!(CU_DEVICE_ATTRIBUTE_TCC_DRIVER).as_bytes())
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MEMORY_CLOCK_RATE => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MEMORY_CLOCK_RATE).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_GLOBAL_MEMORY_BUS_WIDTH => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_GLOBAL_MEMORY_BUS_WIDTH)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_L2_CACHE_SIZE => {
                writer
                    .write_all(stringify!(CU_DEVICE_ATTRIBUTE_L2_CACHE_SIZE).as_bytes())
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_MULTIPROCESSOR => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_MULTIPROCESSOR)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_ASYNC_ENGINE_COUNT => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_ASYNC_ENGINE_COUNT).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_UNIFIED_ADDRESSING => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_UNIFIED_ADDRESSING).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LAYERED_WIDTH => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LAYERED_WIDTH)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LAYERED_LAYERS => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LAYERED_LAYERS)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_CAN_TEX2D_GATHER => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_CAN_TEX2D_GATHER).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_GATHER_WIDTH => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_GATHER_WIDTH)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_GATHER_HEIGHT => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_GATHER_HEIGHT)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_WIDTH_ALTERNATE => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_WIDTH_ALTERNATE)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_HEIGHT_ALTERNATE => {
                writer
                    .write_all(
                        stringify!(
                            CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_HEIGHT_ALTERNATE
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_DEPTH_ALTERNATE => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_DEPTH_ALTERNATE)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_PCI_DOMAIN_ID => {
                writer
                    .write_all(stringify!(CU_DEVICE_ATTRIBUTE_PCI_DOMAIN_ID).as_bytes())
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_TEXTURE_PITCH_ALIGNMENT => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_TEXTURE_PITCH_ALIGNMENT)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURECUBEMAP_WIDTH => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURECUBEMAP_WIDTH)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURECUBEMAP_LAYERED_WIDTH => {
                writer
                    .write_all(
                        stringify!(
                            CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURECUBEMAP_LAYERED_WIDTH
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURECUBEMAP_LAYERED_LAYERS => {
                writer
                    .write_all(
                        stringify!(
                            CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURECUBEMAP_LAYERED_LAYERS
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE1D_WIDTH => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE1D_WIDTH)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_WIDTH => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_WIDTH)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_HEIGHT => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_HEIGHT)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE3D_WIDTH => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE3D_WIDTH)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE3D_HEIGHT => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE3D_HEIGHT)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE3D_DEPTH => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE3D_DEPTH)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE1D_LAYERED_WIDTH => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE1D_LAYERED_WIDTH)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE1D_LAYERED_LAYERS => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE1D_LAYERED_LAYERS)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_LAYERED_WIDTH => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_LAYERED_WIDTH)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_LAYERED_HEIGHT => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_LAYERED_HEIGHT)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_LAYERED_LAYERS => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_LAYERED_LAYERS)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACECUBEMAP_WIDTH => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACECUBEMAP_WIDTH)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACECUBEMAP_LAYERED_WIDTH => {
                writer
                    .write_all(
                        stringify!(
                            CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACECUBEMAP_LAYERED_WIDTH
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACECUBEMAP_LAYERED_LAYERS => {
                writer
                    .write_all(
                        stringify!(
                            CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACECUBEMAP_LAYERED_LAYERS
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LINEAR_WIDTH => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LINEAR_WIDTH)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_WIDTH => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_WIDTH)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_HEIGHT => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_HEIGHT)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_PITCH => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_PITCH)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_MIPMAPPED_WIDTH => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_MIPMAPPED_WIDTH)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_MIPMAPPED_HEIGHT => {
                writer
                    .write_all(
                        stringify!(
                            CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_MIPMAPPED_HEIGHT
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_MIPMAPPED_WIDTH => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_MIPMAPPED_WIDTH)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_STREAM_PRIORITIES_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_STREAM_PRIORITIES_SUPPORTED)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_GLOBAL_L1_CACHE_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_GLOBAL_L1_CACHE_SUPPORTED)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_LOCAL_L1_CACHE_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_LOCAL_L1_CACHE_SUPPORTED)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_MULTIPROCESSOR => {
                writer
                    .write_all(
                        stringify!(
                            CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_MULTIPROCESSOR
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAX_REGISTERS_PER_MULTIPROCESSOR => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAX_REGISTERS_PER_MULTIPROCESSOR)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MANAGED_MEMORY => {
                writer
                    .write_all(stringify!(CU_DEVICE_ATTRIBUTE_MANAGED_MEMORY).as_bytes())
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MULTI_GPU_BOARD => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MULTI_GPU_BOARD).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MULTI_GPU_BOARD_GROUP_ID => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MULTI_GPU_BOARD_GROUP_ID)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_HOST_NATIVE_ATOMIC_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_HOST_NATIVE_ATOMIC_SUPPORTED)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_SINGLE_TO_DOUBLE_PRECISION_PERF_RATIO => {
                writer
                    .write_all(
                        stringify!(
                            CU_DEVICE_ATTRIBUTE_SINGLE_TO_DOUBLE_PRECISION_PERF_RATIO
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_COMPUTE_PREEMPTION_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_COMPUTE_PREEMPTION_SUPPORTED)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_CAN_USE_HOST_POINTER_FOR_REGISTERED_MEM => {
                writer
                    .write_all(
                        stringify!(
                            CU_DEVICE_ATTRIBUTE_CAN_USE_HOST_POINTER_FOR_REGISTERED_MEM
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_CAN_USE_STREAM_MEM_OPS_V1 => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_CAN_USE_STREAM_MEM_OPS_V1)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_CAN_USE_64_BIT_STREAM_MEM_OPS_V1 => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_CAN_USE_64_BIT_STREAM_MEM_OPS_V1)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_CAN_USE_STREAM_WAIT_VALUE_NOR_V1 => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_CAN_USE_STREAM_WAIT_VALUE_NOR_V1)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_COOPERATIVE_LAUNCH => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_COOPERATIVE_LAUNCH).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_COOPERATIVE_MULTI_DEVICE_LAUNCH => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_COOPERATIVE_MULTI_DEVICE_LAUNCH)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_BLOCK_OPTIN => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_BLOCK_OPTIN)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_CAN_FLUSH_REMOTE_WRITES => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_CAN_FLUSH_REMOTE_WRITES)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_HOST_REGISTER_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_HOST_REGISTER_SUPPORTED)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS_USES_HOST_PAGE_TABLES => {
                writer
                    .write_all(
                        stringify!(
                            CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS_USES_HOST_PAGE_TABLES
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_DIRECT_MANAGED_MEM_ACCESS_FROM_HOST => {
                writer
                    .write_all(
                        stringify!(
                            CU_DEVICE_ATTRIBUTE_DIRECT_MANAGED_MEM_ACCESS_FROM_HOST
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_VIRTUAL_ADDRESS_MANAGEMENT_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(
                            CU_DEVICE_ATTRIBUTE_VIRTUAL_ADDRESS_MANAGEMENT_SUPPORTED
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_VIRTUAL_MEMORY_MANAGEMENT_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(
                            CU_DEVICE_ATTRIBUTE_VIRTUAL_MEMORY_MANAGEMENT_SUPPORTED
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_HANDLE_TYPE_POSIX_FILE_DESCRIPTOR_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(
                            CU_DEVICE_ATTRIBUTE_HANDLE_TYPE_POSIX_FILE_DESCRIPTOR_SUPPORTED
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_HANDLE_TYPE_WIN32_HANDLE_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(
                            CU_DEVICE_ATTRIBUTE_HANDLE_TYPE_WIN32_HANDLE_SUPPORTED
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_HANDLE_TYPE_WIN32_KMT_HANDLE_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(
                            CU_DEVICE_ATTRIBUTE_HANDLE_TYPE_WIN32_KMT_HANDLE_SUPPORTED
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAX_BLOCKS_PER_MULTIPROCESSOR => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAX_BLOCKS_PER_MULTIPROCESSOR)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_GENERIC_COMPRESSION_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_GENERIC_COMPRESSION_SUPPORTED)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAX_PERSISTING_L2_CACHE_SIZE => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAX_PERSISTING_L2_CACHE_SIZE)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAX_ACCESS_POLICY_WINDOW_SIZE => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MAX_ACCESS_POLICY_WINDOW_SIZE)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_GPU_DIRECT_RDMA_WITH_CUDA_VMM_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(
                            CU_DEVICE_ATTRIBUTE_GPU_DIRECT_RDMA_WITH_CUDA_VMM_SUPPORTED
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_RESERVED_SHARED_MEMORY_PER_BLOCK => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_RESERVED_SHARED_MEMORY_PER_BLOCK)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_SPARSE_CUDA_ARRAY_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_SPARSE_CUDA_ARRAY_SUPPORTED)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_READ_ONLY_HOST_REGISTER_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_READ_ONLY_HOST_REGISTER_SUPPORTED)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_TIMELINE_SEMAPHORE_INTEROP_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(
                            CU_DEVICE_ATTRIBUTE_TIMELINE_SEMAPHORE_INTEROP_SUPPORTED
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MEMORY_POOLS_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MEMORY_POOLS_SUPPORTED).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_GPU_DIRECT_RDMA_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_GPU_DIRECT_RDMA_SUPPORTED)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_GPU_DIRECT_RDMA_FLUSH_WRITES_OPTIONS => {
                writer
                    .write_all(
                        stringify!(
                            CU_DEVICE_ATTRIBUTE_GPU_DIRECT_RDMA_FLUSH_WRITES_OPTIONS
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_GPU_DIRECT_RDMA_WRITES_ORDERING => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_GPU_DIRECT_RDMA_WRITES_ORDERING)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MEMPOOL_SUPPORTED_HANDLE_TYPES => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MEMPOOL_SUPPORTED_HANDLE_TYPES)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_CLUSTER_LAUNCH => {
                writer
                    .write_all(stringify!(CU_DEVICE_ATTRIBUTE_CLUSTER_LAUNCH).as_bytes())
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_DEFERRED_MAPPING_CUDA_ARRAY_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(
                            CU_DEVICE_ATTRIBUTE_DEFERRED_MAPPING_CUDA_ARRAY_SUPPORTED
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_CAN_USE_64_BIT_STREAM_MEM_OPS => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_CAN_USE_64_BIT_STREAM_MEM_OPS)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_CAN_USE_STREAM_WAIT_VALUE_NOR => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_CAN_USE_STREAM_WAIT_VALUE_NOR)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_DMA_BUF_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_DMA_BUF_SUPPORTED).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_IPC_EVENT_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_IPC_EVENT_SUPPORTED).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MEM_SYNC_DOMAIN_COUNT => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MEM_SYNC_DOMAIN_COUNT).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_TENSOR_MAP_ACCESS_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_TENSOR_MAP_ACCESS_SUPPORTED)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_HANDLE_TYPE_FABRIC_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_HANDLE_TYPE_FABRIC_SUPPORTED)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_UNIFIED_FUNCTION_POINTERS => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_UNIFIED_FUNCTION_POINTERS)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_NUMA_CONFIG => {
                writer.write_all(stringify!(CU_DEVICE_ATTRIBUTE_NUMA_CONFIG).as_bytes())
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_NUMA_ID => {
                writer.write_all(stringify!(CU_DEVICE_ATTRIBUTE_NUMA_ID).as_bytes())
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MULTICAST_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MULTICAST_SUPPORTED).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MPS_ENABLED => {
                writer.write_all(stringify!(CU_DEVICE_ATTRIBUTE_MPS_ENABLED).as_bytes())
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_HOST_NUMA_ID => {
                writer.write_all(stringify!(CU_DEVICE_ATTRIBUTE_HOST_NUMA_ID).as_bytes())
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_D3D12_CIG_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_D3D12_CIG_SUPPORTED).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MEM_DECOMPRESS_ALGORITHM_MASK => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MEM_DECOMPRESS_ALGORITHM_MASK)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MEM_DECOMPRESS_MAXIMUM_LENGTH => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_MEM_DECOMPRESS_MAXIMUM_LENGTH)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_GPU_PCI_DEVICE_ID => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_GPU_PCI_DEVICE_ID).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_GPU_PCI_SUBSYSTEM_ID => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_GPU_PCI_SUBSYSTEM_ID).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_HOST_NUMA_MULTINODE_IPC_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_ATTRIBUTE_HOST_NUMA_MULTINODE_IPC_SUPPORTED)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_MAX => {
                writer.write_all(stringify!(CU_DEVICE_ATTRIBUTE_MAX).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUdevprop_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer
            .write_all(concat!("{ ", stringify!(maxThreadsPerBlock), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.maxThreadsPerBlock, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(maxThreadsDim), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.maxThreadsDim, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(maxGridSize), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.maxGridSize, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(sharedMemPerBlock), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sharedMemPerBlock, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(totalConstantMemory), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.totalConstantMemory, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(SIMDWidth), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.SIMDWidth, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(memPitch), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.memPitch, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(regsPerBlock), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.regsPerBlock, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(clockRate), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.clockRate, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(textureAlign), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.textureAlign, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUpointer_attribute_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUpointer_attribute_enum::CU_POINTER_ATTRIBUTE_CONTEXT => {
                writer.write_all(stringify!(CU_POINTER_ATTRIBUTE_CONTEXT).as_bytes())
            }
            &cuda_types::cuda::CUpointer_attribute_enum::CU_POINTER_ATTRIBUTE_MEMORY_TYPE => {
                writer.write_all(stringify!(CU_POINTER_ATTRIBUTE_MEMORY_TYPE).as_bytes())
            }
            &cuda_types::cuda::CUpointer_attribute_enum::CU_POINTER_ATTRIBUTE_DEVICE_POINTER => {
                writer
                    .write_all(
                        stringify!(CU_POINTER_ATTRIBUTE_DEVICE_POINTER).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUpointer_attribute_enum::CU_POINTER_ATTRIBUTE_HOST_POINTER => {
                writer
                    .write_all(stringify!(CU_POINTER_ATTRIBUTE_HOST_POINTER).as_bytes())
            }
            &cuda_types::cuda::CUpointer_attribute_enum::CU_POINTER_ATTRIBUTE_P2P_TOKENS => {
                writer.write_all(stringify!(CU_POINTER_ATTRIBUTE_P2P_TOKENS).as_bytes())
            }
            &cuda_types::cuda::CUpointer_attribute_enum::CU_POINTER_ATTRIBUTE_SYNC_MEMOPS => {
                writer.write_all(stringify!(CU_POINTER_ATTRIBUTE_SYNC_MEMOPS).as_bytes())
            }
            &cuda_types::cuda::CUpointer_attribute_enum::CU_POINTER_ATTRIBUTE_BUFFER_ID => {
                writer.write_all(stringify!(CU_POINTER_ATTRIBUTE_BUFFER_ID).as_bytes())
            }
            &cuda_types::cuda::CUpointer_attribute_enum::CU_POINTER_ATTRIBUTE_IS_MANAGED => {
                writer.write_all(stringify!(CU_POINTER_ATTRIBUTE_IS_MANAGED).as_bytes())
            }
            &cuda_types::cuda::CUpointer_attribute_enum::CU_POINTER_ATTRIBUTE_DEVICE_ORDINAL => {
                writer
                    .write_all(
                        stringify!(CU_POINTER_ATTRIBUTE_DEVICE_ORDINAL).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUpointer_attribute_enum::CU_POINTER_ATTRIBUTE_IS_LEGACY_CUDA_IPC_CAPABLE => {
                writer
                    .write_all(
                        stringify!(CU_POINTER_ATTRIBUTE_IS_LEGACY_CUDA_IPC_CAPABLE)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUpointer_attribute_enum::CU_POINTER_ATTRIBUTE_RANGE_START_ADDR => {
                writer
                    .write_all(
                        stringify!(CU_POINTER_ATTRIBUTE_RANGE_START_ADDR).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUpointer_attribute_enum::CU_POINTER_ATTRIBUTE_RANGE_SIZE => {
                writer.write_all(stringify!(CU_POINTER_ATTRIBUTE_RANGE_SIZE).as_bytes())
            }
            &cuda_types::cuda::CUpointer_attribute_enum::CU_POINTER_ATTRIBUTE_MAPPED => {
                writer.write_all(stringify!(CU_POINTER_ATTRIBUTE_MAPPED).as_bytes())
            }
            &cuda_types::cuda::CUpointer_attribute_enum::CU_POINTER_ATTRIBUTE_ALLOWED_HANDLE_TYPES => {
                writer
                    .write_all(
                        stringify!(CU_POINTER_ATTRIBUTE_ALLOWED_HANDLE_TYPES).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUpointer_attribute_enum::CU_POINTER_ATTRIBUTE_IS_GPU_DIRECT_RDMA_CAPABLE => {
                writer
                    .write_all(
                        stringify!(CU_POINTER_ATTRIBUTE_IS_GPU_DIRECT_RDMA_CAPABLE)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUpointer_attribute_enum::CU_POINTER_ATTRIBUTE_ACCESS_FLAGS => {
                writer
                    .write_all(stringify!(CU_POINTER_ATTRIBUTE_ACCESS_FLAGS).as_bytes())
            }
            &cuda_types::cuda::CUpointer_attribute_enum::CU_POINTER_ATTRIBUTE_MEMPOOL_HANDLE => {
                writer
                    .write_all(
                        stringify!(CU_POINTER_ATTRIBUTE_MEMPOOL_HANDLE).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUpointer_attribute_enum::CU_POINTER_ATTRIBUTE_MAPPING_SIZE => {
                writer
                    .write_all(stringify!(CU_POINTER_ATTRIBUTE_MAPPING_SIZE).as_bytes())
            }
            &cuda_types::cuda::CUpointer_attribute_enum::CU_POINTER_ATTRIBUTE_MAPPING_BASE_ADDR => {
                writer
                    .write_all(
                        stringify!(CU_POINTER_ATTRIBUTE_MAPPING_BASE_ADDR).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUpointer_attribute_enum::CU_POINTER_ATTRIBUTE_MEMORY_BLOCK_ID => {
                writer
                    .write_all(
                        stringify!(CU_POINTER_ATTRIBUTE_MEMORY_BLOCK_ID).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUpointer_attribute_enum::CU_POINTER_ATTRIBUTE_IS_HW_DECOMPRESS_CAPABLE => {
                writer
                    .write_all(
                        stringify!(CU_POINTER_ATTRIBUTE_IS_HW_DECOMPRESS_CAPABLE)
                            .as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUfunction_attribute_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUfunction_attribute_enum::CU_FUNC_ATTRIBUTE_MAX_THREADS_PER_BLOCK => {
                writer
                    .write_all(
                        stringify!(CU_FUNC_ATTRIBUTE_MAX_THREADS_PER_BLOCK).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUfunction_attribute_enum::CU_FUNC_ATTRIBUTE_SHARED_SIZE_BYTES => {
                writer
                    .write_all(
                        stringify!(CU_FUNC_ATTRIBUTE_SHARED_SIZE_BYTES).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUfunction_attribute_enum::CU_FUNC_ATTRIBUTE_CONST_SIZE_BYTES => {
                writer
                    .write_all(stringify!(CU_FUNC_ATTRIBUTE_CONST_SIZE_BYTES).as_bytes())
            }
            &cuda_types::cuda::CUfunction_attribute_enum::CU_FUNC_ATTRIBUTE_LOCAL_SIZE_BYTES => {
                writer
                    .write_all(stringify!(CU_FUNC_ATTRIBUTE_LOCAL_SIZE_BYTES).as_bytes())
            }
            &cuda_types::cuda::CUfunction_attribute_enum::CU_FUNC_ATTRIBUTE_NUM_REGS => {
                writer.write_all(stringify!(CU_FUNC_ATTRIBUTE_NUM_REGS).as_bytes())
            }
            &cuda_types::cuda::CUfunction_attribute_enum::CU_FUNC_ATTRIBUTE_PTX_VERSION => {
                writer.write_all(stringify!(CU_FUNC_ATTRIBUTE_PTX_VERSION).as_bytes())
            }
            &cuda_types::cuda::CUfunction_attribute_enum::CU_FUNC_ATTRIBUTE_BINARY_VERSION => {
                writer.write_all(stringify!(CU_FUNC_ATTRIBUTE_BINARY_VERSION).as_bytes())
            }
            &cuda_types::cuda::CUfunction_attribute_enum::CU_FUNC_ATTRIBUTE_CACHE_MODE_CA => {
                writer.write_all(stringify!(CU_FUNC_ATTRIBUTE_CACHE_MODE_CA).as_bytes())
            }
            &cuda_types::cuda::CUfunction_attribute_enum::CU_FUNC_ATTRIBUTE_MAX_DYNAMIC_SHARED_SIZE_BYTES => {
                writer
                    .write_all(
                        stringify!(CU_FUNC_ATTRIBUTE_MAX_DYNAMIC_SHARED_SIZE_BYTES)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUfunction_attribute_enum::CU_FUNC_ATTRIBUTE_PREFERRED_SHARED_MEMORY_CARVEOUT => {
                writer
                    .write_all(
                        stringify!(CU_FUNC_ATTRIBUTE_PREFERRED_SHARED_MEMORY_CARVEOUT)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUfunction_attribute_enum::CU_FUNC_ATTRIBUTE_CLUSTER_SIZE_MUST_BE_SET => {
                writer
                    .write_all(
                        stringify!(CU_FUNC_ATTRIBUTE_CLUSTER_SIZE_MUST_BE_SET).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUfunction_attribute_enum::CU_FUNC_ATTRIBUTE_REQUIRED_CLUSTER_WIDTH => {
                writer
                    .write_all(
                        stringify!(CU_FUNC_ATTRIBUTE_REQUIRED_CLUSTER_WIDTH).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUfunction_attribute_enum::CU_FUNC_ATTRIBUTE_REQUIRED_CLUSTER_HEIGHT => {
                writer
                    .write_all(
                        stringify!(CU_FUNC_ATTRIBUTE_REQUIRED_CLUSTER_HEIGHT).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUfunction_attribute_enum::CU_FUNC_ATTRIBUTE_REQUIRED_CLUSTER_DEPTH => {
                writer
                    .write_all(
                        stringify!(CU_FUNC_ATTRIBUTE_REQUIRED_CLUSTER_DEPTH).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUfunction_attribute_enum::CU_FUNC_ATTRIBUTE_NON_PORTABLE_CLUSTER_SIZE_ALLOWED => {
                writer
                    .write_all(
                        stringify!(CU_FUNC_ATTRIBUTE_NON_PORTABLE_CLUSTER_SIZE_ALLOWED)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUfunction_attribute_enum::CU_FUNC_ATTRIBUTE_CLUSTER_SCHEDULING_POLICY_PREFERENCE => {
                writer
                    .write_all(
                        stringify!(
                            CU_FUNC_ATTRIBUTE_CLUSTER_SCHEDULING_POLICY_PREFERENCE
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUfunction_attribute_enum::CU_FUNC_ATTRIBUTE_MAX => {
                writer.write_all(stringify!(CU_FUNC_ATTRIBUTE_MAX).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUfunc_cache_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUfunc_cache_enum::CU_FUNC_CACHE_PREFER_NONE => {
                writer.write_all(stringify!(CU_FUNC_CACHE_PREFER_NONE).as_bytes())
            }
            &cuda_types::cuda::CUfunc_cache_enum::CU_FUNC_CACHE_PREFER_SHARED => {
                writer.write_all(stringify!(CU_FUNC_CACHE_PREFER_SHARED).as_bytes())
            }
            &cuda_types::cuda::CUfunc_cache_enum::CU_FUNC_CACHE_PREFER_L1 => {
                writer.write_all(stringify!(CU_FUNC_CACHE_PREFER_L1).as_bytes())
            }
            &cuda_types::cuda::CUfunc_cache_enum::CU_FUNC_CACHE_PREFER_EQUAL => {
                writer.write_all(stringify!(CU_FUNC_CACHE_PREFER_EQUAL).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUsharedconfig_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUsharedconfig_enum::CU_SHARED_MEM_CONFIG_DEFAULT_BANK_SIZE => {
                writer
                    .write_all(
                        stringify!(CU_SHARED_MEM_CONFIG_DEFAULT_BANK_SIZE).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUsharedconfig_enum::CU_SHARED_MEM_CONFIG_FOUR_BYTE_BANK_SIZE => {
                writer
                    .write_all(
                        stringify!(CU_SHARED_MEM_CONFIG_FOUR_BYTE_BANK_SIZE).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUsharedconfig_enum::CU_SHARED_MEM_CONFIG_EIGHT_BYTE_BANK_SIZE => {
                writer
                    .write_all(
                        stringify!(CU_SHARED_MEM_CONFIG_EIGHT_BYTE_BANK_SIZE).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUshared_carveout_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUshared_carveout_enum::CU_SHAREDMEM_CARVEOUT_DEFAULT => {
                writer.write_all(stringify!(CU_SHAREDMEM_CARVEOUT_DEFAULT).as_bytes())
            }
            &cuda_types::cuda::CUshared_carveout_enum::CU_SHAREDMEM_CARVEOUT_MAX_SHARED => {
                writer.write_all(stringify!(CU_SHAREDMEM_CARVEOUT_MAX_SHARED).as_bytes())
            }
            &cuda_types::cuda::CUshared_carveout_enum::CU_SHAREDMEM_CARVEOUT_MAX_L1 => {
                writer.write_all(stringify!(CU_SHAREDMEM_CARVEOUT_MAX_L1).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUmemorytype_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUmemorytype_enum::CU_MEMORYTYPE_HOST => {
                writer.write_all(stringify!(CU_MEMORYTYPE_HOST).as_bytes())
            }
            &cuda_types::cuda::CUmemorytype_enum::CU_MEMORYTYPE_DEVICE => {
                writer.write_all(stringify!(CU_MEMORYTYPE_DEVICE).as_bytes())
            }
            &cuda_types::cuda::CUmemorytype_enum::CU_MEMORYTYPE_ARRAY => {
                writer.write_all(stringify!(CU_MEMORYTYPE_ARRAY).as_bytes())
            }
            &cuda_types::cuda::CUmemorytype_enum::CU_MEMORYTYPE_UNIFIED => {
                writer.write_all(stringify!(CU_MEMORYTYPE_UNIFIED).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUcomputemode_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUcomputemode_enum::CU_COMPUTEMODE_DEFAULT => {
                writer.write_all(stringify!(CU_COMPUTEMODE_DEFAULT).as_bytes())
            }
            &cuda_types::cuda::CUcomputemode_enum::CU_COMPUTEMODE_PROHIBITED => {
                writer.write_all(stringify!(CU_COMPUTEMODE_PROHIBITED).as_bytes())
            }
            &cuda_types::cuda::CUcomputemode_enum::CU_COMPUTEMODE_EXCLUSIVE_PROCESS => {
                writer.write_all(stringify!(CU_COMPUTEMODE_EXCLUSIVE_PROCESS).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUmem_advise_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUmem_advise_enum::CU_MEM_ADVISE_SET_READ_MOSTLY => {
                writer.write_all(stringify!(CU_MEM_ADVISE_SET_READ_MOSTLY).as_bytes())
            }
            &cuda_types::cuda::CUmem_advise_enum::CU_MEM_ADVISE_UNSET_READ_MOSTLY => {
                writer.write_all(stringify!(CU_MEM_ADVISE_UNSET_READ_MOSTLY).as_bytes())
            }
            &cuda_types::cuda::CUmem_advise_enum::CU_MEM_ADVISE_SET_PREFERRED_LOCATION => {
                writer
                    .write_all(
                        stringify!(CU_MEM_ADVISE_SET_PREFERRED_LOCATION).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUmem_advise_enum::CU_MEM_ADVISE_UNSET_PREFERRED_LOCATION => {
                writer
                    .write_all(
                        stringify!(CU_MEM_ADVISE_UNSET_PREFERRED_LOCATION).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUmem_advise_enum::CU_MEM_ADVISE_SET_ACCESSED_BY => {
                writer.write_all(stringify!(CU_MEM_ADVISE_SET_ACCESSED_BY).as_bytes())
            }
            &cuda_types::cuda::CUmem_advise_enum::CU_MEM_ADVISE_UNSET_ACCESSED_BY => {
                writer.write_all(stringify!(CU_MEM_ADVISE_UNSET_ACCESSED_BY).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUmem_range_attribute_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUmem_range_attribute_enum::CU_MEM_RANGE_ATTRIBUTE_READ_MOSTLY => {
                writer
                    .write_all(stringify!(CU_MEM_RANGE_ATTRIBUTE_READ_MOSTLY).as_bytes())
            }
            &cuda_types::cuda::CUmem_range_attribute_enum::CU_MEM_RANGE_ATTRIBUTE_PREFERRED_LOCATION => {
                writer
                    .write_all(
                        stringify!(CU_MEM_RANGE_ATTRIBUTE_PREFERRED_LOCATION).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUmem_range_attribute_enum::CU_MEM_RANGE_ATTRIBUTE_ACCESSED_BY => {
                writer
                    .write_all(stringify!(CU_MEM_RANGE_ATTRIBUTE_ACCESSED_BY).as_bytes())
            }
            &cuda_types::cuda::CUmem_range_attribute_enum::CU_MEM_RANGE_ATTRIBUTE_LAST_PREFETCH_LOCATION => {
                writer
                    .write_all(
                        stringify!(CU_MEM_RANGE_ATTRIBUTE_LAST_PREFETCH_LOCATION)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUmem_range_attribute_enum::CU_MEM_RANGE_ATTRIBUTE_PREFERRED_LOCATION_TYPE => {
                writer
                    .write_all(
                        stringify!(CU_MEM_RANGE_ATTRIBUTE_PREFERRED_LOCATION_TYPE)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUmem_range_attribute_enum::CU_MEM_RANGE_ATTRIBUTE_PREFERRED_LOCATION_ID => {
                writer
                    .write_all(
                        stringify!(CU_MEM_RANGE_ATTRIBUTE_PREFERRED_LOCATION_ID)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUmem_range_attribute_enum::CU_MEM_RANGE_ATTRIBUTE_LAST_PREFETCH_LOCATION_TYPE => {
                writer
                    .write_all(
                        stringify!(CU_MEM_RANGE_ATTRIBUTE_LAST_PREFETCH_LOCATION_TYPE)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUmem_range_attribute_enum::CU_MEM_RANGE_ATTRIBUTE_LAST_PREFETCH_LOCATION_ID => {
                writer
                    .write_all(
                        stringify!(CU_MEM_RANGE_ATTRIBUTE_LAST_PREFETCH_LOCATION_ID)
                            .as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUjit_option_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_MAX_REGISTERS => {
                writer.write_all(stringify!(CU_JIT_MAX_REGISTERS).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_THREADS_PER_BLOCK => {
                writer.write_all(stringify!(CU_JIT_THREADS_PER_BLOCK).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_WALL_TIME => {
                writer.write_all(stringify!(CU_JIT_WALL_TIME).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_INFO_LOG_BUFFER => {
                writer.write_all(stringify!(CU_JIT_INFO_LOG_BUFFER).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_INFO_LOG_BUFFER_SIZE_BYTES => {
                writer
                    .write_all(stringify!(CU_JIT_INFO_LOG_BUFFER_SIZE_BYTES).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_ERROR_LOG_BUFFER => {
                writer.write_all(stringify!(CU_JIT_ERROR_LOG_BUFFER).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_ERROR_LOG_BUFFER_SIZE_BYTES => {
                writer
                    .write_all(stringify!(CU_JIT_ERROR_LOG_BUFFER_SIZE_BYTES).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_OPTIMIZATION_LEVEL => {
                writer.write_all(stringify!(CU_JIT_OPTIMIZATION_LEVEL).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_TARGET_FROM_CUCONTEXT => {
                writer.write_all(stringify!(CU_JIT_TARGET_FROM_CUCONTEXT).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_TARGET => {
                writer.write_all(stringify!(CU_JIT_TARGET).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_FALLBACK_STRATEGY => {
                writer.write_all(stringify!(CU_JIT_FALLBACK_STRATEGY).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_GENERATE_DEBUG_INFO => {
                writer.write_all(stringify!(CU_JIT_GENERATE_DEBUG_INFO).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_LOG_VERBOSE => {
                writer.write_all(stringify!(CU_JIT_LOG_VERBOSE).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_GENERATE_LINE_INFO => {
                writer.write_all(stringify!(CU_JIT_GENERATE_LINE_INFO).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_CACHE_MODE => {
                writer.write_all(stringify!(CU_JIT_CACHE_MODE).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_NEW_SM3X_OPT => {
                writer.write_all(stringify!(CU_JIT_NEW_SM3X_OPT).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_FAST_COMPILE => {
                writer.write_all(stringify!(CU_JIT_FAST_COMPILE).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_GLOBAL_SYMBOL_NAMES => {
                writer.write_all(stringify!(CU_JIT_GLOBAL_SYMBOL_NAMES).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_GLOBAL_SYMBOL_ADDRESSES => {
                writer.write_all(stringify!(CU_JIT_GLOBAL_SYMBOL_ADDRESSES).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_GLOBAL_SYMBOL_COUNT => {
                writer.write_all(stringify!(CU_JIT_GLOBAL_SYMBOL_COUNT).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_LTO => {
                writer.write_all(stringify!(CU_JIT_LTO).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_FTZ => {
                writer.write_all(stringify!(CU_JIT_FTZ).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_PREC_DIV => {
                writer.write_all(stringify!(CU_JIT_PREC_DIV).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_PREC_SQRT => {
                writer.write_all(stringify!(CU_JIT_PREC_SQRT).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_FMA => {
                writer.write_all(stringify!(CU_JIT_FMA).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_REFERENCED_KERNEL_NAMES => {
                writer.write_all(stringify!(CU_JIT_REFERENCED_KERNEL_NAMES).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_REFERENCED_KERNEL_COUNT => {
                writer.write_all(stringify!(CU_JIT_REFERENCED_KERNEL_COUNT).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_REFERENCED_VARIABLE_NAMES => {
                writer.write_all(stringify!(CU_JIT_REFERENCED_VARIABLE_NAMES).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_REFERENCED_VARIABLE_COUNT => {
                writer.write_all(stringify!(CU_JIT_REFERENCED_VARIABLE_COUNT).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_OPTIMIZE_UNUSED_DEVICE_VARIABLES => {
                writer
                    .write_all(
                        stringify!(CU_JIT_OPTIMIZE_UNUSED_DEVICE_VARIABLES).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_POSITION_INDEPENDENT_CODE => {
                writer.write_all(stringify!(CU_JIT_POSITION_INDEPENDENT_CODE).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_MIN_CTA_PER_SM => {
                writer.write_all(stringify!(CU_JIT_MIN_CTA_PER_SM).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_MAX_THREADS_PER_BLOCK => {
                writer.write_all(stringify!(CU_JIT_MAX_THREADS_PER_BLOCK).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_OVERRIDE_DIRECTIVE_VALUES => {
                writer.write_all(stringify!(CU_JIT_OVERRIDE_DIRECTIVE_VALUES).as_bytes())
            }
            &cuda_types::cuda::CUjit_option_enum::CU_JIT_NUM_OPTIONS => {
                writer.write_all(stringify!(CU_JIT_NUM_OPTIONS).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUjit_target_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUjit_target_enum::CU_TARGET_COMPUTE_30 => {
                writer.write_all(stringify!(CU_TARGET_COMPUTE_30).as_bytes())
            }
            &cuda_types::cuda::CUjit_target_enum::CU_TARGET_COMPUTE_32 => {
                writer.write_all(stringify!(CU_TARGET_COMPUTE_32).as_bytes())
            }
            &cuda_types::cuda::CUjit_target_enum::CU_TARGET_COMPUTE_35 => {
                writer.write_all(stringify!(CU_TARGET_COMPUTE_35).as_bytes())
            }
            &cuda_types::cuda::CUjit_target_enum::CU_TARGET_COMPUTE_37 => {
                writer.write_all(stringify!(CU_TARGET_COMPUTE_37).as_bytes())
            }
            &cuda_types::cuda::CUjit_target_enum::CU_TARGET_COMPUTE_50 => {
                writer.write_all(stringify!(CU_TARGET_COMPUTE_50).as_bytes())
            }
            &cuda_types::cuda::CUjit_target_enum::CU_TARGET_COMPUTE_52 => {
                writer.write_all(stringify!(CU_TARGET_COMPUTE_52).as_bytes())
            }
            &cuda_types::cuda::CUjit_target_enum::CU_TARGET_COMPUTE_53 => {
                writer.write_all(stringify!(CU_TARGET_COMPUTE_53).as_bytes())
            }
            &cuda_types::cuda::CUjit_target_enum::CU_TARGET_COMPUTE_60 => {
                writer.write_all(stringify!(CU_TARGET_COMPUTE_60).as_bytes())
            }
            &cuda_types::cuda::CUjit_target_enum::CU_TARGET_COMPUTE_61 => {
                writer.write_all(stringify!(CU_TARGET_COMPUTE_61).as_bytes())
            }
            &cuda_types::cuda::CUjit_target_enum::CU_TARGET_COMPUTE_62 => {
                writer.write_all(stringify!(CU_TARGET_COMPUTE_62).as_bytes())
            }
            &cuda_types::cuda::CUjit_target_enum::CU_TARGET_COMPUTE_70 => {
                writer.write_all(stringify!(CU_TARGET_COMPUTE_70).as_bytes())
            }
            &cuda_types::cuda::CUjit_target_enum::CU_TARGET_COMPUTE_72 => {
                writer.write_all(stringify!(CU_TARGET_COMPUTE_72).as_bytes())
            }
            &cuda_types::cuda::CUjit_target_enum::CU_TARGET_COMPUTE_75 => {
                writer.write_all(stringify!(CU_TARGET_COMPUTE_75).as_bytes())
            }
            &cuda_types::cuda::CUjit_target_enum::CU_TARGET_COMPUTE_80 => {
                writer.write_all(stringify!(CU_TARGET_COMPUTE_80).as_bytes())
            }
            &cuda_types::cuda::CUjit_target_enum::CU_TARGET_COMPUTE_86 => {
                writer.write_all(stringify!(CU_TARGET_COMPUTE_86).as_bytes())
            }
            &cuda_types::cuda::CUjit_target_enum::CU_TARGET_COMPUTE_87 => {
                writer.write_all(stringify!(CU_TARGET_COMPUTE_87).as_bytes())
            }
            &cuda_types::cuda::CUjit_target_enum::CU_TARGET_COMPUTE_89 => {
                writer.write_all(stringify!(CU_TARGET_COMPUTE_89).as_bytes())
            }
            &cuda_types::cuda::CUjit_target_enum::CU_TARGET_COMPUTE_90 => {
                writer.write_all(stringify!(CU_TARGET_COMPUTE_90).as_bytes())
            }
            &cuda_types::cuda::CUjit_target_enum::CU_TARGET_COMPUTE_100 => {
                writer.write_all(stringify!(CU_TARGET_COMPUTE_100).as_bytes())
            }
            &cuda_types::cuda::CUjit_target_enum::CU_TARGET_COMPUTE_101 => {
                writer.write_all(stringify!(CU_TARGET_COMPUTE_101).as_bytes())
            }
            &cuda_types::cuda::CUjit_target_enum::CU_TARGET_COMPUTE_120 => {
                writer.write_all(stringify!(CU_TARGET_COMPUTE_120).as_bytes())
            }
            &cuda_types::cuda::CUjit_target_enum::CU_TARGET_COMPUTE_90A => {
                writer.write_all(stringify!(CU_TARGET_COMPUTE_90A).as_bytes())
            }
            &cuda_types::cuda::CUjit_target_enum::CU_TARGET_COMPUTE_100A => {
                writer.write_all(stringify!(CU_TARGET_COMPUTE_100A).as_bytes())
            }
            &cuda_types::cuda::CUjit_target_enum::CU_TARGET_COMPUTE_101A => {
                writer.write_all(stringify!(CU_TARGET_COMPUTE_101A).as_bytes())
            }
            &cuda_types::cuda::CUjit_target_enum::CU_TARGET_COMPUTE_120A => {
                writer.write_all(stringify!(CU_TARGET_COMPUTE_120A).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUjit_fallback_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUjit_fallback_enum::CU_PREFER_PTX => {
                writer.write_all(stringify!(CU_PREFER_PTX).as_bytes())
            }
            &cuda_types::cuda::CUjit_fallback_enum::CU_PREFER_BINARY => {
                writer.write_all(stringify!(CU_PREFER_BINARY).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUjit_cacheMode_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUjit_cacheMode_enum::CU_JIT_CACHE_OPTION_NONE => {
                writer.write_all(stringify!(CU_JIT_CACHE_OPTION_NONE).as_bytes())
            }
            &cuda_types::cuda::CUjit_cacheMode_enum::CU_JIT_CACHE_OPTION_CG => {
                writer.write_all(stringify!(CU_JIT_CACHE_OPTION_CG).as_bytes())
            }
            &cuda_types::cuda::CUjit_cacheMode_enum::CU_JIT_CACHE_OPTION_CA => {
                writer.write_all(stringify!(CU_JIT_CACHE_OPTION_CA).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUjitInputType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUjitInputType_enum::CU_JIT_INPUT_CUBIN => {
                writer.write_all(stringify!(CU_JIT_INPUT_CUBIN).as_bytes())
            }
            &cuda_types::cuda::CUjitInputType_enum::CU_JIT_INPUT_PTX => {
                writer.write_all(stringify!(CU_JIT_INPUT_PTX).as_bytes())
            }
            &cuda_types::cuda::CUjitInputType_enum::CU_JIT_INPUT_FATBINARY => {
                writer.write_all(stringify!(CU_JIT_INPUT_FATBINARY).as_bytes())
            }
            &cuda_types::cuda::CUjitInputType_enum::CU_JIT_INPUT_OBJECT => {
                writer.write_all(stringify!(CU_JIT_INPUT_OBJECT).as_bytes())
            }
            &cuda_types::cuda::CUjitInputType_enum::CU_JIT_INPUT_LIBRARY => {
                writer.write_all(stringify!(CU_JIT_INPUT_LIBRARY).as_bytes())
            }
            &cuda_types::cuda::CUjitInputType_enum::CU_JIT_INPUT_NVVM => {
                writer.write_all(stringify!(CU_JIT_INPUT_NVVM).as_bytes())
            }
            &cuda_types::cuda::CUjitInputType_enum::CU_JIT_NUM_INPUT_TYPES => {
                writer.write_all(stringify!(CU_JIT_NUM_INPUT_TYPES).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUlinkState {
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
impl crate::CudaDisplay for cuda_types::cuda::CUgraphicsRegisterFlags_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUgraphicsRegisterFlags_enum::CU_GRAPHICS_REGISTER_FLAGS_NONE => {
                writer.write_all(stringify!(CU_GRAPHICS_REGISTER_FLAGS_NONE).as_bytes())
            }
            &cuda_types::cuda::CUgraphicsRegisterFlags_enum::CU_GRAPHICS_REGISTER_FLAGS_READ_ONLY => {
                writer
                    .write_all(
                        stringify!(CU_GRAPHICS_REGISTER_FLAGS_READ_ONLY).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphicsRegisterFlags_enum::CU_GRAPHICS_REGISTER_FLAGS_WRITE_DISCARD => {
                writer
                    .write_all(
                        stringify!(CU_GRAPHICS_REGISTER_FLAGS_WRITE_DISCARD).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphicsRegisterFlags_enum::CU_GRAPHICS_REGISTER_FLAGS_SURFACE_LDST => {
                writer
                    .write_all(
                        stringify!(CU_GRAPHICS_REGISTER_FLAGS_SURFACE_LDST).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphicsRegisterFlags_enum::CU_GRAPHICS_REGISTER_FLAGS_TEXTURE_GATHER => {
                writer
                    .write_all(
                        stringify!(CU_GRAPHICS_REGISTER_FLAGS_TEXTURE_GATHER).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUgraphicsMapResourceFlags_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUgraphicsMapResourceFlags_enum::CU_GRAPHICS_MAP_RESOURCE_FLAGS_NONE => {
                writer
                    .write_all(
                        stringify!(CU_GRAPHICS_MAP_RESOURCE_FLAGS_NONE).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphicsMapResourceFlags_enum::CU_GRAPHICS_MAP_RESOURCE_FLAGS_READ_ONLY => {
                writer
                    .write_all(
                        stringify!(CU_GRAPHICS_MAP_RESOURCE_FLAGS_READ_ONLY).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphicsMapResourceFlags_enum::CU_GRAPHICS_MAP_RESOURCE_FLAGS_WRITE_DISCARD => {
                writer
                    .write_all(
                        stringify!(CU_GRAPHICS_MAP_RESOURCE_FLAGS_WRITE_DISCARD)
                            .as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUarray_cubemap_face_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUarray_cubemap_face_enum::CU_CUBEMAP_FACE_POSITIVE_X => {
                writer.write_all(stringify!(CU_CUBEMAP_FACE_POSITIVE_X).as_bytes())
            }
            &cuda_types::cuda::CUarray_cubemap_face_enum::CU_CUBEMAP_FACE_NEGATIVE_X => {
                writer.write_all(stringify!(CU_CUBEMAP_FACE_NEGATIVE_X).as_bytes())
            }
            &cuda_types::cuda::CUarray_cubemap_face_enum::CU_CUBEMAP_FACE_POSITIVE_Y => {
                writer.write_all(stringify!(CU_CUBEMAP_FACE_POSITIVE_Y).as_bytes())
            }
            &cuda_types::cuda::CUarray_cubemap_face_enum::CU_CUBEMAP_FACE_NEGATIVE_Y => {
                writer.write_all(stringify!(CU_CUBEMAP_FACE_NEGATIVE_Y).as_bytes())
            }
            &cuda_types::cuda::CUarray_cubemap_face_enum::CU_CUBEMAP_FACE_POSITIVE_Z => {
                writer.write_all(stringify!(CU_CUBEMAP_FACE_POSITIVE_Z).as_bytes())
            }
            &cuda_types::cuda::CUarray_cubemap_face_enum::CU_CUBEMAP_FACE_NEGATIVE_Z => {
                writer.write_all(stringify!(CU_CUBEMAP_FACE_NEGATIVE_Z).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUlimit_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUlimit_enum::CU_LIMIT_STACK_SIZE => {
                writer.write_all(stringify!(CU_LIMIT_STACK_SIZE).as_bytes())
            }
            &cuda_types::cuda::CUlimit_enum::CU_LIMIT_PRINTF_FIFO_SIZE => {
                writer.write_all(stringify!(CU_LIMIT_PRINTF_FIFO_SIZE).as_bytes())
            }
            &cuda_types::cuda::CUlimit_enum::CU_LIMIT_MALLOC_HEAP_SIZE => {
                writer.write_all(stringify!(CU_LIMIT_MALLOC_HEAP_SIZE).as_bytes())
            }
            &cuda_types::cuda::CUlimit_enum::CU_LIMIT_DEV_RUNTIME_SYNC_DEPTH => {
                writer.write_all(stringify!(CU_LIMIT_DEV_RUNTIME_SYNC_DEPTH).as_bytes())
            }
            &cuda_types::cuda::CUlimit_enum::CU_LIMIT_DEV_RUNTIME_PENDING_LAUNCH_COUNT => {
                writer
                    .write_all(
                        stringify!(CU_LIMIT_DEV_RUNTIME_PENDING_LAUNCH_COUNT).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUlimit_enum::CU_LIMIT_MAX_L2_FETCH_GRANULARITY => {
                writer
                    .write_all(stringify!(CU_LIMIT_MAX_L2_FETCH_GRANULARITY).as_bytes())
            }
            &cuda_types::cuda::CUlimit_enum::CU_LIMIT_PERSISTING_L2_CACHE_SIZE => {
                writer
                    .write_all(stringify!(CU_LIMIT_PERSISTING_L2_CACHE_SIZE).as_bytes())
            }
            &cuda_types::cuda::CUlimit_enum::CU_LIMIT_SHMEM_SIZE => {
                writer.write_all(stringify!(CU_LIMIT_SHMEM_SIZE).as_bytes())
            }
            &cuda_types::cuda::CUlimit_enum::CU_LIMIT_CIG_ENABLED => {
                writer.write_all(stringify!(CU_LIMIT_CIG_ENABLED).as_bytes())
            }
            &cuda_types::cuda::CUlimit_enum::CU_LIMIT_CIG_SHMEM_FALLBACK_ENABLED => {
                writer
                    .write_all(
                        stringify!(CU_LIMIT_CIG_SHMEM_FALLBACK_ENABLED).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUlimit_enum::CU_LIMIT_MAX => {
                writer.write_all(stringify!(CU_LIMIT_MAX).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUresourcetype_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUresourcetype_enum::CU_RESOURCE_TYPE_ARRAY => {
                writer.write_all(stringify!(CU_RESOURCE_TYPE_ARRAY).as_bytes())
            }
            &cuda_types::cuda::CUresourcetype_enum::CU_RESOURCE_TYPE_MIPMAPPED_ARRAY => {
                writer.write_all(stringify!(CU_RESOURCE_TYPE_MIPMAPPED_ARRAY).as_bytes())
            }
            &cuda_types::cuda::CUresourcetype_enum::CU_RESOURCE_TYPE_LINEAR => {
                writer.write_all(stringify!(CU_RESOURCE_TYPE_LINEAR).as_bytes())
            }
            &cuda_types::cuda::CUresourcetype_enum::CU_RESOURCE_TYPE_PITCH2D => {
                writer.write_all(stringify!(CU_RESOURCE_TYPE_PITCH2D).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUhostFn {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(
            writer,
            "{:p}",
            unsafe {
                std::mem::transmute::<
                    cuda_types::cuda::CUhostFn,
                    *mut ::std::ffi::c_void,
                >(*self)
            },
        )
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUaccessProperty_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUaccessProperty_enum::CU_ACCESS_PROPERTY_NORMAL => {
                writer.write_all(stringify!(CU_ACCESS_PROPERTY_NORMAL).as_bytes())
            }
            &cuda_types::cuda::CUaccessProperty_enum::CU_ACCESS_PROPERTY_STREAMING => {
                writer.write_all(stringify!(CU_ACCESS_PROPERTY_STREAMING).as_bytes())
            }
            &cuda_types::cuda::CUaccessProperty_enum::CU_ACCESS_PROPERTY_PERSISTING => {
                writer.write_all(stringify!(CU_ACCESS_PROPERTY_PERSISTING).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUaccessPolicyWindow_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(base_ptr), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.base_ptr, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(num_bytes), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.num_bytes, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(hitRatio), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.hitRatio, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(hitProp), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.hitProp, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(missProp), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.missProp, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_KERNEL_NODE_PARAMS_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(func), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.func, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(gridDimX), ": ").as_bytes())?;
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
        writer.write_all(concat!(", ", stringify!(kernelParams), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.kernelParams, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(extra), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.extra, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_KERNEL_NODE_PARAMS_v2_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(func), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.func, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(gridDimX), ": ").as_bytes())?;
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
        writer.write_all(concat!(", ", stringify!(kernelParams), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.kernelParams, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(extra), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.extra, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(kern), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.kern, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(ctx), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.ctx, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_KERNEL_NODE_PARAMS_v3_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(func), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.func, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(gridDimX), ": ").as_bytes())?;
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
        writer.write_all(concat!(", ", stringify!(kernelParams), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.kernelParams, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(extra), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.extra, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(kern), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.kern, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(ctx), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.ctx, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_MEMSET_NODE_PARAMS_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(dst), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dst, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(pitch), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.pitch, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(value), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.value, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(elementSize), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.elementSize, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(width), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.width, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(height), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.height, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_MEMSET_NODE_PARAMS_v2_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(dst), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dst, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(pitch), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.pitch, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(value), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.value, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(elementSize), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.elementSize, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(width), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.width, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(height), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.height, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(ctx), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.ctx, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_HOST_NODE_PARAMS_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(fn_), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.fn_, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(userData), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.userData, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_HOST_NODE_PARAMS_v2_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(fn_), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.fn_, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(userData), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.userData, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUgraphConditionalNodeType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUgraphConditionalNodeType_enum::CU_GRAPH_COND_TYPE_IF => {
                writer.write_all(stringify!(CU_GRAPH_COND_TYPE_IF).as_bytes())
            }
            &cuda_types::cuda::CUgraphConditionalNodeType_enum::CU_GRAPH_COND_TYPE_WHILE => {
                writer.write_all(stringify!(CU_GRAPH_COND_TYPE_WHILE).as_bytes())
            }
            &cuda_types::cuda::CUgraphConditionalNodeType_enum::CU_GRAPH_COND_TYPE_SWITCH => {
                writer.write_all(stringify!(CU_GRAPH_COND_TYPE_SWITCH).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_CONDITIONAL_NODE_PARAMS {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(handle), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.handle, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(type_), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.type_, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(size), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.size, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(phGraph_out), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.phGraph_out, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(ctx), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.ctx, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUgraphNodeType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUgraphNodeType_enum::CU_GRAPH_NODE_TYPE_KERNEL => {
                writer.write_all(stringify!(CU_GRAPH_NODE_TYPE_KERNEL).as_bytes())
            }
            &cuda_types::cuda::CUgraphNodeType_enum::CU_GRAPH_NODE_TYPE_MEMCPY => {
                writer.write_all(stringify!(CU_GRAPH_NODE_TYPE_MEMCPY).as_bytes())
            }
            &cuda_types::cuda::CUgraphNodeType_enum::CU_GRAPH_NODE_TYPE_MEMSET => {
                writer.write_all(stringify!(CU_GRAPH_NODE_TYPE_MEMSET).as_bytes())
            }
            &cuda_types::cuda::CUgraphNodeType_enum::CU_GRAPH_NODE_TYPE_HOST => {
                writer.write_all(stringify!(CU_GRAPH_NODE_TYPE_HOST).as_bytes())
            }
            &cuda_types::cuda::CUgraphNodeType_enum::CU_GRAPH_NODE_TYPE_GRAPH => {
                writer.write_all(stringify!(CU_GRAPH_NODE_TYPE_GRAPH).as_bytes())
            }
            &cuda_types::cuda::CUgraphNodeType_enum::CU_GRAPH_NODE_TYPE_EMPTY => {
                writer.write_all(stringify!(CU_GRAPH_NODE_TYPE_EMPTY).as_bytes())
            }
            &cuda_types::cuda::CUgraphNodeType_enum::CU_GRAPH_NODE_TYPE_WAIT_EVENT => {
                writer.write_all(stringify!(CU_GRAPH_NODE_TYPE_WAIT_EVENT).as_bytes())
            }
            &cuda_types::cuda::CUgraphNodeType_enum::CU_GRAPH_NODE_TYPE_EVENT_RECORD => {
                writer.write_all(stringify!(CU_GRAPH_NODE_TYPE_EVENT_RECORD).as_bytes())
            }
            &cuda_types::cuda::CUgraphNodeType_enum::CU_GRAPH_NODE_TYPE_EXT_SEMAS_SIGNAL => {
                writer
                    .write_all(
                        stringify!(CU_GRAPH_NODE_TYPE_EXT_SEMAS_SIGNAL).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphNodeType_enum::CU_GRAPH_NODE_TYPE_EXT_SEMAS_WAIT => {
                writer
                    .write_all(stringify!(CU_GRAPH_NODE_TYPE_EXT_SEMAS_WAIT).as_bytes())
            }
            &cuda_types::cuda::CUgraphNodeType_enum::CU_GRAPH_NODE_TYPE_MEM_ALLOC => {
                writer.write_all(stringify!(CU_GRAPH_NODE_TYPE_MEM_ALLOC).as_bytes())
            }
            &cuda_types::cuda::CUgraphNodeType_enum::CU_GRAPH_NODE_TYPE_MEM_FREE => {
                writer.write_all(stringify!(CU_GRAPH_NODE_TYPE_MEM_FREE).as_bytes())
            }
            &cuda_types::cuda::CUgraphNodeType_enum::CU_GRAPH_NODE_TYPE_BATCH_MEM_OP => {
                writer.write_all(stringify!(CU_GRAPH_NODE_TYPE_BATCH_MEM_OP).as_bytes())
            }
            &cuda_types::cuda::CUgraphNodeType_enum::CU_GRAPH_NODE_TYPE_CONDITIONAL => {
                writer.write_all(stringify!(CU_GRAPH_NODE_TYPE_CONDITIONAL).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUgraphDependencyType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUgraphDependencyType_enum::CU_GRAPH_DEPENDENCY_TYPE_DEFAULT => {
                writer.write_all(stringify!(CU_GRAPH_DEPENDENCY_TYPE_DEFAULT).as_bytes())
            }
            &cuda_types::cuda::CUgraphDependencyType_enum::CU_GRAPH_DEPENDENCY_TYPE_PROGRAMMATIC => {
                writer
                    .write_all(
                        stringify!(CU_GRAPH_DEPENDENCY_TYPE_PROGRAMMATIC).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUgraphEdgeData_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(from_port), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.from_port, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(to_port), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.to_port, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(type_), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.type_, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUgraphInstantiateResult_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUgraphInstantiateResult_enum::CUDA_GRAPH_INSTANTIATE_SUCCESS => {
                writer.write_all(stringify!(CUDA_GRAPH_INSTANTIATE_SUCCESS).as_bytes())
            }
            &cuda_types::cuda::CUgraphInstantiateResult_enum::CUDA_GRAPH_INSTANTIATE_ERROR => {
                writer.write_all(stringify!(CUDA_GRAPH_INSTANTIATE_ERROR).as_bytes())
            }
            &cuda_types::cuda::CUgraphInstantiateResult_enum::CUDA_GRAPH_INSTANTIATE_INVALID_STRUCTURE => {
                writer
                    .write_all(
                        stringify!(CUDA_GRAPH_INSTANTIATE_INVALID_STRUCTURE).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphInstantiateResult_enum::CUDA_GRAPH_INSTANTIATE_NODE_OPERATION_NOT_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(CUDA_GRAPH_INSTANTIATE_NODE_OPERATION_NOT_SUPPORTED)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphInstantiateResult_enum::CUDA_GRAPH_INSTANTIATE_MULTIPLE_CTXS_NOT_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(CUDA_GRAPH_INSTANTIATE_MULTIPLE_CTXS_NOT_SUPPORTED)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphInstantiateResult_enum::CUDA_GRAPH_INSTANTIATE_CONDITIONAL_HANDLE_UNUSED => {
                writer
                    .write_all(
                        stringify!(CUDA_GRAPH_INSTANTIATE_CONDITIONAL_HANDLE_UNUSED)
                            .as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_GRAPH_INSTANTIATE_PARAMS_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(flags), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.flags, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(hUploadStream), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.hUploadStream, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(hErrNode_out), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.hErrNode_out, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(result_out), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.result_out, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUsynchronizationPolicy_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUsynchronizationPolicy_enum::CU_SYNC_POLICY_AUTO => {
                writer.write_all(stringify!(CU_SYNC_POLICY_AUTO).as_bytes())
            }
            &cuda_types::cuda::CUsynchronizationPolicy_enum::CU_SYNC_POLICY_SPIN => {
                writer.write_all(stringify!(CU_SYNC_POLICY_SPIN).as_bytes())
            }
            &cuda_types::cuda::CUsynchronizationPolicy_enum::CU_SYNC_POLICY_YIELD => {
                writer.write_all(stringify!(CU_SYNC_POLICY_YIELD).as_bytes())
            }
            &cuda_types::cuda::CUsynchronizationPolicy_enum::CU_SYNC_POLICY_BLOCKING_SYNC => {
                writer.write_all(stringify!(CU_SYNC_POLICY_BLOCKING_SYNC).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUclusterSchedulingPolicy_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUclusterSchedulingPolicy_enum::CU_CLUSTER_SCHEDULING_POLICY_DEFAULT => {
                writer
                    .write_all(
                        stringify!(CU_CLUSTER_SCHEDULING_POLICY_DEFAULT).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUclusterSchedulingPolicy_enum::CU_CLUSTER_SCHEDULING_POLICY_SPREAD => {
                writer
                    .write_all(
                        stringify!(CU_CLUSTER_SCHEDULING_POLICY_SPREAD).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUclusterSchedulingPolicy_enum::CU_CLUSTER_SCHEDULING_POLICY_LOAD_BALANCING => {
                writer
                    .write_all(
                        stringify!(CU_CLUSTER_SCHEDULING_POLICY_LOAD_BALANCING)
                            .as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUlaunchMemSyncDomain_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUlaunchMemSyncDomain_enum::CU_LAUNCH_MEM_SYNC_DOMAIN_DEFAULT => {
                writer
                    .write_all(stringify!(CU_LAUNCH_MEM_SYNC_DOMAIN_DEFAULT).as_bytes())
            }
            &cuda_types::cuda::CUlaunchMemSyncDomain_enum::CU_LAUNCH_MEM_SYNC_DOMAIN_REMOTE => {
                writer.write_all(stringify!(CU_LAUNCH_MEM_SYNC_DOMAIN_REMOTE).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUlaunchMemSyncDomainMap_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(default_), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.default_, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(remote), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.remote, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUlaunchAttributeID_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUlaunchAttributeID_enum::CU_LAUNCH_ATTRIBUTE_IGNORE => {
                writer.write_all(stringify!(CU_LAUNCH_ATTRIBUTE_IGNORE).as_bytes())
            }
            &cuda_types::cuda::CUlaunchAttributeID_enum::CU_LAUNCH_ATTRIBUTE_ACCESS_POLICY_WINDOW => {
                writer
                    .write_all(
                        stringify!(CU_LAUNCH_ATTRIBUTE_ACCESS_POLICY_WINDOW).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUlaunchAttributeID_enum::CU_LAUNCH_ATTRIBUTE_COOPERATIVE => {
                writer.write_all(stringify!(CU_LAUNCH_ATTRIBUTE_COOPERATIVE).as_bytes())
            }
            &cuda_types::cuda::CUlaunchAttributeID_enum::CU_LAUNCH_ATTRIBUTE_SYNCHRONIZATION_POLICY => {
                writer
                    .write_all(
                        stringify!(CU_LAUNCH_ATTRIBUTE_SYNCHRONIZATION_POLICY).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUlaunchAttributeID_enum::CU_LAUNCH_ATTRIBUTE_CLUSTER_DIMENSION => {
                writer
                    .write_all(
                        stringify!(CU_LAUNCH_ATTRIBUTE_CLUSTER_DIMENSION).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUlaunchAttributeID_enum::CU_LAUNCH_ATTRIBUTE_CLUSTER_SCHEDULING_POLICY_PREFERENCE => {
                writer
                    .write_all(
                        stringify!(
                            CU_LAUNCH_ATTRIBUTE_CLUSTER_SCHEDULING_POLICY_PREFERENCE
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUlaunchAttributeID_enum::CU_LAUNCH_ATTRIBUTE_PROGRAMMATIC_STREAM_SERIALIZATION => {
                writer
                    .write_all(
                        stringify!(CU_LAUNCH_ATTRIBUTE_PROGRAMMATIC_STREAM_SERIALIZATION)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUlaunchAttributeID_enum::CU_LAUNCH_ATTRIBUTE_PROGRAMMATIC_EVENT => {
                writer
                    .write_all(
                        stringify!(CU_LAUNCH_ATTRIBUTE_PROGRAMMATIC_EVENT).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUlaunchAttributeID_enum::CU_LAUNCH_ATTRIBUTE_PRIORITY => {
                writer.write_all(stringify!(CU_LAUNCH_ATTRIBUTE_PRIORITY).as_bytes())
            }
            &cuda_types::cuda::CUlaunchAttributeID_enum::CU_LAUNCH_ATTRIBUTE_MEM_SYNC_DOMAIN_MAP => {
                writer
                    .write_all(
                        stringify!(CU_LAUNCH_ATTRIBUTE_MEM_SYNC_DOMAIN_MAP).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUlaunchAttributeID_enum::CU_LAUNCH_ATTRIBUTE_MEM_SYNC_DOMAIN => {
                writer
                    .write_all(
                        stringify!(CU_LAUNCH_ATTRIBUTE_MEM_SYNC_DOMAIN).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUlaunchAttributeID_enum::CU_LAUNCH_ATTRIBUTE_PREFERRED_CLUSTER_DIMENSION => {
                writer
                    .write_all(
                        stringify!(CU_LAUNCH_ATTRIBUTE_PREFERRED_CLUSTER_DIMENSION)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUlaunchAttributeID_enum::CU_LAUNCH_ATTRIBUTE_LAUNCH_COMPLETION_EVENT => {
                writer
                    .write_all(
                        stringify!(CU_LAUNCH_ATTRIBUTE_LAUNCH_COMPLETION_EVENT)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUlaunchAttributeID_enum::CU_LAUNCH_ATTRIBUTE_DEVICE_UPDATABLE_KERNEL_NODE => {
                writer
                    .write_all(
                        stringify!(CU_LAUNCH_ATTRIBUTE_DEVICE_UPDATABLE_KERNEL_NODE)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUlaunchAttributeID_enum::CU_LAUNCH_ATTRIBUTE_PREFERRED_SHARED_MEMORY_CARVEOUT => {
                writer
                    .write_all(
                        stringify!(CU_LAUNCH_ATTRIBUTE_PREFERRED_SHARED_MEMORY_CARVEOUT)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUlaunchAttributeID_enum::CU_LAUNCH_ATTRIBUTE_MAX => {
                writer.write_all(stringify!(CU_LAUNCH_ATTRIBUTE_MAX).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay
for cuda_types::cuda::CUlaunchAttributeValue_union__bindgen_ty_1 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(x), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.x, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(y), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.y, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(z), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.z, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay
for cuda_types::cuda::CUlaunchAttributeValue_union__bindgen_ty_2 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(event), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.event, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(flags), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.flags, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(triggerAtBlockStart), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.triggerAtBlockStart, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay
for cuda_types::cuda::CUlaunchAttributeValue_union__bindgen_ty_3 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(event), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.event, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(flags), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.flags, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay
for cuda_types::cuda::CUlaunchAttributeValue_union__bindgen_ty_4 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(x), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.x, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(y), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.y, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(z), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.z, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay
for cuda_types::cuda::CUlaunchAttributeValue_union__bindgen_ty_5 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(deviceUpdatable), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.deviceUpdatable, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(devNode), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.devNode, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUstreamCaptureStatus_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUstreamCaptureStatus_enum::CU_STREAM_CAPTURE_STATUS_NONE => {
                writer.write_all(stringify!(CU_STREAM_CAPTURE_STATUS_NONE).as_bytes())
            }
            &cuda_types::cuda::CUstreamCaptureStatus_enum::CU_STREAM_CAPTURE_STATUS_ACTIVE => {
                writer.write_all(stringify!(CU_STREAM_CAPTURE_STATUS_ACTIVE).as_bytes())
            }
            &cuda_types::cuda::CUstreamCaptureStatus_enum::CU_STREAM_CAPTURE_STATUS_INVALIDATED => {
                writer
                    .write_all(
                        stringify!(CU_STREAM_CAPTURE_STATUS_INVALIDATED).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUstreamCaptureMode_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUstreamCaptureMode_enum::CU_STREAM_CAPTURE_MODE_GLOBAL => {
                writer.write_all(stringify!(CU_STREAM_CAPTURE_MODE_GLOBAL).as_bytes())
            }
            &cuda_types::cuda::CUstreamCaptureMode_enum::CU_STREAM_CAPTURE_MODE_THREAD_LOCAL => {
                writer
                    .write_all(
                        stringify!(CU_STREAM_CAPTURE_MODE_THREAD_LOCAL).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUstreamCaptureMode_enum::CU_STREAM_CAPTURE_MODE_RELAXED => {
                writer.write_all(stringify!(CU_STREAM_CAPTURE_MODE_RELAXED).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUdriverProcAddress_flags_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUdriverProcAddress_flags_enum::CU_GET_PROC_ADDRESS_DEFAULT => {
                writer.write_all(stringify!(CU_GET_PROC_ADDRESS_DEFAULT).as_bytes())
            }
            &cuda_types::cuda::CUdriverProcAddress_flags_enum::CU_GET_PROC_ADDRESS_LEGACY_STREAM => {
                writer
                    .write_all(stringify!(CU_GET_PROC_ADDRESS_LEGACY_STREAM).as_bytes())
            }
            &cuda_types::cuda::CUdriverProcAddress_flags_enum::CU_GET_PROC_ADDRESS_PER_THREAD_DEFAULT_STREAM => {
                writer
                    .write_all(
                        stringify!(CU_GET_PROC_ADDRESS_PER_THREAD_DEFAULT_STREAM)
                            .as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUdriverProcAddressQueryResult_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUdriverProcAddressQueryResult_enum::CU_GET_PROC_ADDRESS_SUCCESS => {
                writer.write_all(stringify!(CU_GET_PROC_ADDRESS_SUCCESS).as_bytes())
            }
            &cuda_types::cuda::CUdriverProcAddressQueryResult_enum::CU_GET_PROC_ADDRESS_SYMBOL_NOT_FOUND => {
                writer
                    .write_all(
                        stringify!(CU_GET_PROC_ADDRESS_SYMBOL_NOT_FOUND).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdriverProcAddressQueryResult_enum::CU_GET_PROC_ADDRESS_VERSION_NOT_SUFFICIENT => {
                writer
                    .write_all(
                        stringify!(CU_GET_PROC_ADDRESS_VERSION_NOT_SUFFICIENT).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUexecAffinityType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUexecAffinityType_enum::CU_EXEC_AFFINITY_TYPE_SM_COUNT => {
                writer.write_all(stringify!(CU_EXEC_AFFINITY_TYPE_SM_COUNT).as_bytes())
            }
            &cuda_types::cuda::CUexecAffinityType_enum::CU_EXEC_AFFINITY_TYPE_MAX => {
                writer.write_all(stringify!(CU_EXEC_AFFINITY_TYPE_MAX).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUexecAffinitySmCount_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(val), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.val, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUcigDataType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUcigDataType_enum::CIG_DATA_TYPE_D3D12_COMMAND_QUEUE => {
                writer
                    .write_all(stringify!(CIG_DATA_TYPE_D3D12_COMMAND_QUEUE).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUctxCigParam_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(sharedDataType), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sharedDataType, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(sharedData), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sharedData, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUctxCreateParams_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer
            .write_all(concat!("{ ", stringify!(execAffinityParams), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.execAffinityParams, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(numExecAffinityParams), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.numExecAffinityParams, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(cigParams), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.cigParams, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUlibraryOption_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUlibraryOption_enum::CU_LIBRARY_HOST_UNIVERSAL_FUNCTION_AND_DATA_TABLE => {
                writer
                    .write_all(
                        stringify!(CU_LIBRARY_HOST_UNIVERSAL_FUNCTION_AND_DATA_TABLE)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUlibraryOption_enum::CU_LIBRARY_BINARY_IS_PRESERVED => {
                writer.write_all(stringify!(CU_LIBRARY_BINARY_IS_PRESERVED).as_bytes())
            }
            &cuda_types::cuda::CUlibraryOption_enum::CU_LIBRARY_NUM_OPTIONS => {
                writer.write_all(stringify!(CU_LIBRARY_NUM_OPTIONS).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay
for cuda_types::cuda::CUlibraryHostUniversalFunctionAndDataTable_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(functionTable), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.functionTable, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(functionWindowSize), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.functionWindowSize, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dataTable), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dataTable, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dataWindowSize), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dataWindowSize, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUdevice_P2PAttribute_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUdevice_P2PAttribute_enum::CU_DEVICE_P2P_ATTRIBUTE_PERFORMANCE_RANK => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_P2P_ATTRIBUTE_PERFORMANCE_RANK).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_P2PAttribute_enum::CU_DEVICE_P2P_ATTRIBUTE_ACCESS_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_P2P_ATTRIBUTE_ACCESS_SUPPORTED).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_P2PAttribute_enum::CU_DEVICE_P2P_ATTRIBUTE_NATIVE_ATOMIC_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_P2P_ATTRIBUTE_NATIVE_ATOMIC_SUPPORTED)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_P2PAttribute_enum::CU_DEVICE_P2P_ATTRIBUTE_ACCESS_ACCESS_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_P2P_ATTRIBUTE_ACCESS_ACCESS_SUPPORTED)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevice_P2PAttribute_enum::CU_DEVICE_P2P_ATTRIBUTE_CUDA_ARRAY_ACCESS_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(CU_DEVICE_P2P_ATTRIBUTE_CUDA_ARRAY_ACCESS_SUPPORTED)
                            .as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUstreamCallback {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(
            writer,
            "{:p}",
            unsafe {
                std::mem::transmute::<
                    cuda_types::cuda::CUstreamCallback,
                    *mut ::std::ffi::c_void,
                >(*self)
            },
        )
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUoccupancyB2DSize {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(
            writer,
            "{:p}",
            unsafe {
                std::mem::transmute::<
                    cuda_types::cuda::CUoccupancyB2DSize,
                    *mut ::std::ffi::c_void,
                >(*self)
            },
        )
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_MEMCPY2D_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(srcXInBytes), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcXInBytes, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcY), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcY, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcMemoryType), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcMemoryType, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcHost), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcHost, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcDevice), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcDevice, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcArray), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcArray, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcPitch), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcPitch, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstXInBytes), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstXInBytes, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstY), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstY, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstMemoryType), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstMemoryType, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstHost), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstHost, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstDevice), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstDevice, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstArray), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstArray, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstPitch), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstPitch, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(WidthInBytes), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.WidthInBytes, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(Height), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.Height, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_MEMCPY3D_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(srcXInBytes), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcXInBytes, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcY), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcY, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcZ), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcZ, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcLOD), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcLOD, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcMemoryType), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcMemoryType, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcHost), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcHost, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcDevice), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcDevice, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcArray), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcArray, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcPitch), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcPitch, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcHeight), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcHeight, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstXInBytes), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstXInBytes, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstY), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstY, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstZ), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstZ, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstLOD), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstLOD, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstMemoryType), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstMemoryType, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstHost), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstHost, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstDevice), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstDevice, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstArray), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstArray, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstPitch), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstPitch, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstHeight), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstHeight, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(WidthInBytes), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.WidthInBytes, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(Height), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.Height, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(Depth), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.Depth, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_MEMCPY3D_PEER_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(srcXInBytes), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcXInBytes, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcY), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcY, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcZ), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcZ, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcLOD), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcLOD, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcMemoryType), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcMemoryType, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcHost), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcHost, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcDevice), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcDevice, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcArray), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcArray, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcContext), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcContext, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcPitch), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcPitch, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcHeight), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcHeight, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstXInBytes), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstXInBytes, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstY), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstY, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstZ), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstZ, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstLOD), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstLOD, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstMemoryType), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstMemoryType, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstHost), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstHost, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstDevice), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstDevice, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstArray), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstArray, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstContext), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstContext, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstPitch), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstPitch, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstHeight), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstHeight, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(WidthInBytes), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.WidthInBytes, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(Height), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.Height, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(Depth), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.Depth, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_MEMCPY_NODE_PARAMS_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(flags), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.flags, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(copyCtx), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.copyCtx, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(copyParams), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.copyParams, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_ARRAY_DESCRIPTOR_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(Width), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.Width, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(Height), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.Height, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(Format), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.Format, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(NumChannels), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.NumChannels, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_ARRAY3D_DESCRIPTOR_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(Width), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.Width, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(Height), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.Height, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(Depth), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.Depth, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(Format), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.Format, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(NumChannels), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.NumChannels, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(Flags), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.Flags, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_ARRAY_SPARSE_PROPERTIES_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(tileExtent), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.tileExtent, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(miptailFirstLevel), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.miptailFirstLevel, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(miptailSize), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.miptailSize, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(flags), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.flags, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay
for cuda_types::cuda::CUDA_ARRAY_SPARSE_PROPERTIES_st__bindgen_ty_1 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(width), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.width, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(height), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.height, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(depth), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.depth, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_ARRAY_MEMORY_REQUIREMENTS_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(size), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.size, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(alignment), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.alignment, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay
for cuda_types::cuda::CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_1 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(hArray), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.hArray, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay
for cuda_types::cuda::CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_2 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(hMipmappedArray), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.hMipmappedArray, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay
for cuda_types::cuda::CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_3 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(devPtr), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.devPtr, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(format), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.format, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(numChannels), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.numChannels, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(sizeInBytes), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sizeInBytes, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay
for cuda_types::cuda::CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_4 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(devPtr), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.devPtr, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(format), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.format, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(numChannels), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.numChannels, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(width), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.width, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(height), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.height, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(pitchInBytes), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.pitchInBytes, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_TEXTURE_DESC_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(addressMode), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.addressMode, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(filterMode), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.filterMode, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(flags), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.flags, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(maxAnisotropy), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.maxAnisotropy, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(mipmapFilterMode), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.mipmapFilterMode, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(mipmapLevelBias), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.mipmapLevelBias, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(minMipmapLevelClamp), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.minMipmapLevelClamp, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(maxMipmapLevelClamp), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.maxMipmapLevelClamp, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(borderColor), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.borderColor, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUresourceViewFormat_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_NONE => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_NONE).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_UINT_1X8 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_UINT_1X8).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_UINT_2X8 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_UINT_2X8).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_UINT_4X8 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_UINT_4X8).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_SINT_1X8 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_SINT_1X8).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_SINT_2X8 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_SINT_2X8).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_SINT_4X8 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_SINT_4X8).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_UINT_1X16 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_UINT_1X16).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_UINT_2X16 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_UINT_2X16).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_UINT_4X16 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_UINT_4X16).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_SINT_1X16 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_SINT_1X16).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_SINT_2X16 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_SINT_2X16).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_SINT_4X16 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_SINT_4X16).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_UINT_1X32 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_UINT_1X32).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_UINT_2X32 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_UINT_2X32).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_UINT_4X32 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_UINT_4X32).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_SINT_1X32 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_SINT_1X32).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_SINT_2X32 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_SINT_2X32).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_SINT_4X32 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_SINT_4X32).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_FLOAT_1X16 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_FLOAT_1X16).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_FLOAT_2X16 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_FLOAT_2X16).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_FLOAT_4X16 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_FLOAT_4X16).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_FLOAT_1X32 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_FLOAT_1X32).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_FLOAT_2X32 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_FLOAT_2X32).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_FLOAT_4X32 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_FLOAT_4X32).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_UNSIGNED_BC1 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_UNSIGNED_BC1).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_UNSIGNED_BC2 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_UNSIGNED_BC2).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_UNSIGNED_BC3 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_UNSIGNED_BC3).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_UNSIGNED_BC4 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_UNSIGNED_BC4).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_SIGNED_BC4 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_SIGNED_BC4).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_UNSIGNED_BC5 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_UNSIGNED_BC5).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_SIGNED_BC5 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_SIGNED_BC5).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_UNSIGNED_BC6H => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_UNSIGNED_BC6H).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_SIGNED_BC6H => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_SIGNED_BC6H).as_bytes())
            }
            &cuda_types::cuda::CUresourceViewFormat_enum::CU_RES_VIEW_FORMAT_UNSIGNED_BC7 => {
                writer.write_all(stringify!(CU_RES_VIEW_FORMAT_UNSIGNED_BC7).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_RESOURCE_VIEW_DESC_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(format), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.format, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(width), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.width, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(height), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.height, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(depth), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.depth, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(firstMipmapLevel), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.firstMipmapLevel, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(lastMipmapLevel), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.lastMipmapLevel, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(firstLayer), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.firstLayer, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(lastLayer), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.lastLayer, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUtensorMap_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(opaque), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.opaque, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUtensorMapDataType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUtensorMapDataType_enum::CU_TENSOR_MAP_DATA_TYPE_UINT8 => {
                writer.write_all(stringify!(CU_TENSOR_MAP_DATA_TYPE_UINT8).as_bytes())
            }
            &cuda_types::cuda::CUtensorMapDataType_enum::CU_TENSOR_MAP_DATA_TYPE_UINT16 => {
                writer.write_all(stringify!(CU_TENSOR_MAP_DATA_TYPE_UINT16).as_bytes())
            }
            &cuda_types::cuda::CUtensorMapDataType_enum::CU_TENSOR_MAP_DATA_TYPE_UINT32 => {
                writer.write_all(stringify!(CU_TENSOR_MAP_DATA_TYPE_UINT32).as_bytes())
            }
            &cuda_types::cuda::CUtensorMapDataType_enum::CU_TENSOR_MAP_DATA_TYPE_INT32 => {
                writer.write_all(stringify!(CU_TENSOR_MAP_DATA_TYPE_INT32).as_bytes())
            }
            &cuda_types::cuda::CUtensorMapDataType_enum::CU_TENSOR_MAP_DATA_TYPE_UINT64 => {
                writer.write_all(stringify!(CU_TENSOR_MAP_DATA_TYPE_UINT64).as_bytes())
            }
            &cuda_types::cuda::CUtensorMapDataType_enum::CU_TENSOR_MAP_DATA_TYPE_INT64 => {
                writer.write_all(stringify!(CU_TENSOR_MAP_DATA_TYPE_INT64).as_bytes())
            }
            &cuda_types::cuda::CUtensorMapDataType_enum::CU_TENSOR_MAP_DATA_TYPE_FLOAT16 => {
                writer.write_all(stringify!(CU_TENSOR_MAP_DATA_TYPE_FLOAT16).as_bytes())
            }
            &cuda_types::cuda::CUtensorMapDataType_enum::CU_TENSOR_MAP_DATA_TYPE_FLOAT32 => {
                writer.write_all(stringify!(CU_TENSOR_MAP_DATA_TYPE_FLOAT32).as_bytes())
            }
            &cuda_types::cuda::CUtensorMapDataType_enum::CU_TENSOR_MAP_DATA_TYPE_FLOAT64 => {
                writer.write_all(stringify!(CU_TENSOR_MAP_DATA_TYPE_FLOAT64).as_bytes())
            }
            &cuda_types::cuda::CUtensorMapDataType_enum::CU_TENSOR_MAP_DATA_TYPE_BFLOAT16 => {
                writer.write_all(stringify!(CU_TENSOR_MAP_DATA_TYPE_BFLOAT16).as_bytes())
            }
            &cuda_types::cuda::CUtensorMapDataType_enum::CU_TENSOR_MAP_DATA_TYPE_FLOAT32_FTZ => {
                writer
                    .write_all(
                        stringify!(CU_TENSOR_MAP_DATA_TYPE_FLOAT32_FTZ).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUtensorMapDataType_enum::CU_TENSOR_MAP_DATA_TYPE_TFLOAT32 => {
                writer.write_all(stringify!(CU_TENSOR_MAP_DATA_TYPE_TFLOAT32).as_bytes())
            }
            &cuda_types::cuda::CUtensorMapDataType_enum::CU_TENSOR_MAP_DATA_TYPE_TFLOAT32_FTZ => {
                writer
                    .write_all(
                        stringify!(CU_TENSOR_MAP_DATA_TYPE_TFLOAT32_FTZ).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUtensorMapDataType_enum::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN8B => {
                writer
                    .write_all(
                        stringify!(CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN8B).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUtensorMapDataType_enum::CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN16B => {
                writer
                    .write_all(
                        stringify!(CU_TENSOR_MAP_DATA_TYPE_16U4_ALIGN16B).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUtensorMapDataType_enum::CU_TENSOR_MAP_DATA_TYPE_16U6_ALIGN16B => {
                writer
                    .write_all(
                        stringify!(CU_TENSOR_MAP_DATA_TYPE_16U6_ALIGN16B).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUtensorMapInterleave_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUtensorMapInterleave_enum::CU_TENSOR_MAP_INTERLEAVE_NONE => {
                writer.write_all(stringify!(CU_TENSOR_MAP_INTERLEAVE_NONE).as_bytes())
            }
            &cuda_types::cuda::CUtensorMapInterleave_enum::CU_TENSOR_MAP_INTERLEAVE_16B => {
                writer.write_all(stringify!(CU_TENSOR_MAP_INTERLEAVE_16B).as_bytes())
            }
            &cuda_types::cuda::CUtensorMapInterleave_enum::CU_TENSOR_MAP_INTERLEAVE_32B => {
                writer.write_all(stringify!(CU_TENSOR_MAP_INTERLEAVE_32B).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUtensorMapSwizzle_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUtensorMapSwizzle_enum::CU_TENSOR_MAP_SWIZZLE_NONE => {
                writer.write_all(stringify!(CU_TENSOR_MAP_SWIZZLE_NONE).as_bytes())
            }
            &cuda_types::cuda::CUtensorMapSwizzle_enum::CU_TENSOR_MAP_SWIZZLE_32B => {
                writer.write_all(stringify!(CU_TENSOR_MAP_SWIZZLE_32B).as_bytes())
            }
            &cuda_types::cuda::CUtensorMapSwizzle_enum::CU_TENSOR_MAP_SWIZZLE_64B => {
                writer.write_all(stringify!(CU_TENSOR_MAP_SWIZZLE_64B).as_bytes())
            }
            &cuda_types::cuda::CUtensorMapSwizzle_enum::CU_TENSOR_MAP_SWIZZLE_128B => {
                writer.write_all(stringify!(CU_TENSOR_MAP_SWIZZLE_128B).as_bytes())
            }
            &cuda_types::cuda::CUtensorMapSwizzle_enum::CU_TENSOR_MAP_SWIZZLE_128B_ATOM_32B => {
                writer
                    .write_all(
                        stringify!(CU_TENSOR_MAP_SWIZZLE_128B_ATOM_32B).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUtensorMapSwizzle_enum::CU_TENSOR_MAP_SWIZZLE_128B_ATOM_32B_FLIP_8B => {
                writer
                    .write_all(
                        stringify!(CU_TENSOR_MAP_SWIZZLE_128B_ATOM_32B_FLIP_8B)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUtensorMapSwizzle_enum::CU_TENSOR_MAP_SWIZZLE_128B_ATOM_64B => {
                writer
                    .write_all(
                        stringify!(CU_TENSOR_MAP_SWIZZLE_128B_ATOM_64B).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUtensorMapL2promotion_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUtensorMapL2promotion_enum::CU_TENSOR_MAP_L2_PROMOTION_NONE => {
                writer.write_all(stringify!(CU_TENSOR_MAP_L2_PROMOTION_NONE).as_bytes())
            }
            &cuda_types::cuda::CUtensorMapL2promotion_enum::CU_TENSOR_MAP_L2_PROMOTION_L2_64B => {
                writer
                    .write_all(stringify!(CU_TENSOR_MAP_L2_PROMOTION_L2_64B).as_bytes())
            }
            &cuda_types::cuda::CUtensorMapL2promotion_enum::CU_TENSOR_MAP_L2_PROMOTION_L2_128B => {
                writer
                    .write_all(stringify!(CU_TENSOR_MAP_L2_PROMOTION_L2_128B).as_bytes())
            }
            &cuda_types::cuda::CUtensorMapL2promotion_enum::CU_TENSOR_MAP_L2_PROMOTION_L2_256B => {
                writer
                    .write_all(stringify!(CU_TENSOR_MAP_L2_PROMOTION_L2_256B).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUtensorMapFloatOOBfill_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUtensorMapFloatOOBfill_enum::CU_TENSOR_MAP_FLOAT_OOB_FILL_NONE => {
                writer
                    .write_all(stringify!(CU_TENSOR_MAP_FLOAT_OOB_FILL_NONE).as_bytes())
            }
            &cuda_types::cuda::CUtensorMapFloatOOBfill_enum::CU_TENSOR_MAP_FLOAT_OOB_FILL_NAN_REQUEST_ZERO_FMA => {
                writer
                    .write_all(
                        stringify!(CU_TENSOR_MAP_FLOAT_OOB_FILL_NAN_REQUEST_ZERO_FMA)
                            .as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUtensorMapIm2ColWideMode_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUtensorMapIm2ColWideMode_enum::CU_TENSOR_MAP_IM2COL_WIDE_MODE_W => {
                writer.write_all(stringify!(CU_TENSOR_MAP_IM2COL_WIDE_MODE_W).as_bytes())
            }
            &cuda_types::cuda::CUtensorMapIm2ColWideMode_enum::CU_TENSOR_MAP_IM2COL_WIDE_MODE_W128 => {
                writer
                    .write_all(
                        stringify!(CU_TENSOR_MAP_IM2COL_WIDE_MODE_W128).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_POINTER_ATTRIBUTE_P2P_TOKENS_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(p2pToken), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.p2pToken, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(vaSpaceToken), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.vaSpaceToken, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_POINTER_ATTRIBUTE_ACCESS_FLAGS_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUDA_POINTER_ATTRIBUTE_ACCESS_FLAGS_enum::CU_POINTER_ATTRIBUTE_ACCESS_FLAG_NONE => {
                writer
                    .write_all(
                        stringify!(CU_POINTER_ATTRIBUTE_ACCESS_FLAG_NONE).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUDA_POINTER_ATTRIBUTE_ACCESS_FLAGS_enum::CU_POINTER_ATTRIBUTE_ACCESS_FLAG_READ => {
                writer
                    .write_all(
                        stringify!(CU_POINTER_ATTRIBUTE_ACCESS_FLAG_READ).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUDA_POINTER_ATTRIBUTE_ACCESS_FLAGS_enum::CU_POINTER_ATTRIBUTE_ACCESS_FLAG_READWRITE => {
                writer
                    .write_all(
                        stringify!(CU_POINTER_ATTRIBUTE_ACCESS_FLAG_READWRITE).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_LAUNCH_PARAMS_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(function), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.function, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(gridDimX), ": ").as_bytes())?;
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
        writer.write_all(concat!(", ", stringify!(kernelParams), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.kernelParams, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUexternalMemoryHandleType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUexternalMemoryHandleType_enum::CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD => {
                writer
                    .write_all(
                        stringify!(CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUexternalMemoryHandleType_enum::CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32 => {
                writer
                    .write_all(
                        stringify!(CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUexternalMemoryHandleType_enum::CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT => {
                writer
                    .write_all(
                        stringify!(CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUexternalMemoryHandleType_enum::CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP => {
                writer
                    .write_all(
                        stringify!(CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUexternalMemoryHandleType_enum::CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE => {
                writer
                    .write_all(
                        stringify!(CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUexternalMemoryHandleType_enum::CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_RESOURCE => {
                writer
                    .write_all(
                        stringify!(CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_RESOURCE)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUexternalMemoryHandleType_enum::CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_RESOURCE_KMT => {
                writer
                    .write_all(
                        stringify!(CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_RESOURCE_KMT)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUexternalMemoryHandleType_enum::CU_EXTERNAL_MEMORY_HANDLE_TYPE_NVSCIBUF => {
                writer
                    .write_all(
                        stringify!(CU_EXTERNAL_MEMORY_HANDLE_TYPE_NVSCIBUF).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay
for cuda_types::cuda::CUDA_EXTERNAL_MEMORY_HANDLE_DESC_st__bindgen_ty_1__bindgen_ty_1 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(handle), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.handle, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(name), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.name, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_EXTERNAL_MEMORY_BUFFER_DESC_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(offset), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.offset, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(size), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.size, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(flags), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.flags, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay
for cuda_types::cuda::CUDA_EXTERNAL_MEMORY_MIPMAPPED_ARRAY_DESC_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(offset), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.offset, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(arrayDesc), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.arrayDesc, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(numLevels), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.numLevels, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUexternalSemaphoreHandleType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUexternalSemaphoreHandleType_enum::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD => {
                writer
                    .write_all(
                        stringify!(CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUexternalSemaphoreHandleType_enum::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32 => {
                writer
                    .write_all(
                        stringify!(CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUexternalSemaphoreHandleType_enum::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT => {
                writer
                    .write_all(
                        stringify!(CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUexternalSemaphoreHandleType_enum::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE => {
                writer
                    .write_all(
                        stringify!(CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUexternalSemaphoreHandleType_enum::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE => {
                writer
                    .write_all(
                        stringify!(CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUexternalSemaphoreHandleType_enum::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_NVSCISYNC => {
                writer
                    .write_all(
                        stringify!(CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_NVSCISYNC)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUexternalSemaphoreHandleType_enum::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_KEYED_MUTEX => {
                writer
                    .write_all(
                        stringify!(CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_KEYED_MUTEX)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUexternalSemaphoreHandleType_enum::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_KEYED_MUTEX_KMT => {
                writer
                    .write_all(
                        stringify!(
                            CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_KEYED_MUTEX_KMT
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUexternalSemaphoreHandleType_enum::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_TIMELINE_SEMAPHORE_FD => {
                writer
                    .write_all(
                        stringify!(
                            CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_TIMELINE_SEMAPHORE_FD
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUexternalSemaphoreHandleType_enum::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_TIMELINE_SEMAPHORE_WIN32 => {
                writer
                    .write_all(
                        stringify!(
                            CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_TIMELINE_SEMAPHORE_WIN32
                        )
                            .as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay
for cuda_types::cuda::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC_st__bindgen_ty_1__bindgen_ty_1 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(handle), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.handle, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(name), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.name, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(params), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.params, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(flags), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.flags, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay
for cuda_types::cuda::CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS_st__bindgen_ty_1 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(fence), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.fence, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(nvSciSync), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.nvSciSync, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(keyedMutex), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.keyedMutex, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay
for cuda_types::cuda::CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS_st__bindgen_ty_1__bindgen_ty_1 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(value), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.value, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay
for cuda_types::cuda::CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS_st__bindgen_ty_1__bindgen_ty_3 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(key), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.key, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(params), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.params, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(flags), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.flags, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay
for cuda_types::cuda::CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS_st__bindgen_ty_1 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(fence), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.fence, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(nvSciSync), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.nvSciSync, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(keyedMutex), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.keyedMutex, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay
for cuda_types::cuda::CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS_st__bindgen_ty_1__bindgen_ty_1 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(value), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.value, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay
for cuda_types::cuda::CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS_st__bindgen_ty_1__bindgen_ty_3 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(key), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.key, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(timeoutMs), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.timeoutMs, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_EXT_SEM_SIGNAL_NODE_PARAMS_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(extSemArray), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.extSemArray, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(paramsArray), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.paramsArray, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(numExtSems), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.numExtSems, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_EXT_SEM_SIGNAL_NODE_PARAMS_v2_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(extSemArray), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.extSemArray, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(paramsArray), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.paramsArray, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(numExtSems), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.numExtSems, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_EXT_SEM_WAIT_NODE_PARAMS_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(extSemArray), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.extSemArray, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(paramsArray), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.paramsArray, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(numExtSems), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.numExtSems, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_EXT_SEM_WAIT_NODE_PARAMS_v2_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(extSemArray), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.extSemArray, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(paramsArray), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.paramsArray, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(numExtSems), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.numExtSems, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUmemAllocationHandleType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUmemAllocationHandleType_enum::CU_MEM_HANDLE_TYPE_NONE => {
                writer.write_all(stringify!(CU_MEM_HANDLE_TYPE_NONE).as_bytes())
            }
            &cuda_types::cuda::CUmemAllocationHandleType_enum::CU_MEM_HANDLE_TYPE_POSIX_FILE_DESCRIPTOR => {
                writer
                    .write_all(
                        stringify!(CU_MEM_HANDLE_TYPE_POSIX_FILE_DESCRIPTOR).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUmemAllocationHandleType_enum::CU_MEM_HANDLE_TYPE_WIN32 => {
                writer.write_all(stringify!(CU_MEM_HANDLE_TYPE_WIN32).as_bytes())
            }
            &cuda_types::cuda::CUmemAllocationHandleType_enum::CU_MEM_HANDLE_TYPE_WIN32_KMT => {
                writer.write_all(stringify!(CU_MEM_HANDLE_TYPE_WIN32_KMT).as_bytes())
            }
            &cuda_types::cuda::CUmemAllocationHandleType_enum::CU_MEM_HANDLE_TYPE_FABRIC => {
                writer.write_all(stringify!(CU_MEM_HANDLE_TYPE_FABRIC).as_bytes())
            }
            &cuda_types::cuda::CUmemAllocationHandleType_enum::CU_MEM_HANDLE_TYPE_MAX => {
                writer.write_all(stringify!(CU_MEM_HANDLE_TYPE_MAX).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUmemAccess_flags_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUmemAccess_flags_enum::CU_MEM_ACCESS_FLAGS_PROT_NONE => {
                writer.write_all(stringify!(CU_MEM_ACCESS_FLAGS_PROT_NONE).as_bytes())
            }
            &cuda_types::cuda::CUmemAccess_flags_enum::CU_MEM_ACCESS_FLAGS_PROT_READ => {
                writer.write_all(stringify!(CU_MEM_ACCESS_FLAGS_PROT_READ).as_bytes())
            }
            &cuda_types::cuda::CUmemAccess_flags_enum::CU_MEM_ACCESS_FLAGS_PROT_READWRITE => {
                writer
                    .write_all(stringify!(CU_MEM_ACCESS_FLAGS_PROT_READWRITE).as_bytes())
            }
            &cuda_types::cuda::CUmemAccess_flags_enum::CU_MEM_ACCESS_FLAGS_PROT_MAX => {
                writer.write_all(stringify!(CU_MEM_ACCESS_FLAGS_PROT_MAX).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUmemLocationType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUmemLocationType_enum::CU_MEM_LOCATION_TYPE_INVALID => {
                writer.write_all(stringify!(CU_MEM_LOCATION_TYPE_INVALID).as_bytes())
            }
            &cuda_types::cuda::CUmemLocationType_enum::CU_MEM_LOCATION_TYPE_DEVICE => {
                writer.write_all(stringify!(CU_MEM_LOCATION_TYPE_DEVICE).as_bytes())
            }
            &cuda_types::cuda::CUmemLocationType_enum::CU_MEM_LOCATION_TYPE_HOST => {
                writer.write_all(stringify!(CU_MEM_LOCATION_TYPE_HOST).as_bytes())
            }
            &cuda_types::cuda::CUmemLocationType_enum::CU_MEM_LOCATION_TYPE_HOST_NUMA => {
                writer.write_all(stringify!(CU_MEM_LOCATION_TYPE_HOST_NUMA).as_bytes())
            }
            &cuda_types::cuda::CUmemLocationType_enum::CU_MEM_LOCATION_TYPE_HOST_NUMA_CURRENT => {
                writer
                    .write_all(
                        stringify!(CU_MEM_LOCATION_TYPE_HOST_NUMA_CURRENT).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUmemLocationType_enum::CU_MEM_LOCATION_TYPE_MAX => {
                writer.write_all(stringify!(CU_MEM_LOCATION_TYPE_MAX).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUmemAllocationType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUmemAllocationType_enum::CU_MEM_ALLOCATION_TYPE_INVALID => {
                writer.write_all(stringify!(CU_MEM_ALLOCATION_TYPE_INVALID).as_bytes())
            }
            &cuda_types::cuda::CUmemAllocationType_enum::CU_MEM_ALLOCATION_TYPE_PINNED => {
                writer.write_all(stringify!(CU_MEM_ALLOCATION_TYPE_PINNED).as_bytes())
            }
            &cuda_types::cuda::CUmemAllocationType_enum::CU_MEM_ALLOCATION_TYPE_MAX => {
                writer.write_all(stringify!(CU_MEM_ALLOCATION_TYPE_MAX).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUmemAllocationGranularity_flags_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUmemAllocationGranularity_flags_enum::CU_MEM_ALLOC_GRANULARITY_MINIMUM => {
                writer.write_all(stringify!(CU_MEM_ALLOC_GRANULARITY_MINIMUM).as_bytes())
            }
            &cuda_types::cuda::CUmemAllocationGranularity_flags_enum::CU_MEM_ALLOC_GRANULARITY_RECOMMENDED => {
                writer
                    .write_all(
                        stringify!(CU_MEM_ALLOC_GRANULARITY_RECOMMENDED).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUmemRangeHandleType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUmemRangeHandleType_enum::CU_MEM_RANGE_HANDLE_TYPE_DMA_BUF_FD => {
                writer
                    .write_all(
                        stringify!(CU_MEM_RANGE_HANDLE_TYPE_DMA_BUF_FD).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUmemRangeHandleType_enum::CU_MEM_RANGE_HANDLE_TYPE_MAX => {
                writer.write_all(stringify!(CU_MEM_RANGE_HANDLE_TYPE_MAX).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUmemRangeFlags_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUmemRangeFlags_enum::CU_MEM_RANGE_FLAG_DMA_BUF_MAPPING_TYPE_PCIE => {
                writer
                    .write_all(
                        stringify!(CU_MEM_RANGE_FLAG_DMA_BUF_MAPPING_TYPE_PCIE)
                            .as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUarraySparseSubresourceType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUarraySparseSubresourceType_enum::CU_ARRAY_SPARSE_SUBRESOURCE_TYPE_SPARSE_LEVEL => {
                writer
                    .write_all(
                        stringify!(CU_ARRAY_SPARSE_SUBRESOURCE_TYPE_SPARSE_LEVEL)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUarraySparseSubresourceType_enum::CU_ARRAY_SPARSE_SUBRESOURCE_TYPE_MIPTAIL => {
                writer
                    .write_all(
                        stringify!(CU_ARRAY_SPARSE_SUBRESOURCE_TYPE_MIPTAIL).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUmemOperationType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUmemOperationType_enum::CU_MEM_OPERATION_TYPE_MAP => {
                writer.write_all(stringify!(CU_MEM_OPERATION_TYPE_MAP).as_bytes())
            }
            &cuda_types::cuda::CUmemOperationType_enum::CU_MEM_OPERATION_TYPE_UNMAP => {
                writer.write_all(stringify!(CU_MEM_OPERATION_TYPE_UNMAP).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUmemHandleType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUmemHandleType_enum::CU_MEM_HANDLE_TYPE_GENERIC => {
                writer.write_all(stringify!(CU_MEM_HANDLE_TYPE_GENERIC).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay
for cuda_types::cuda::CUarrayMapInfo_st__bindgen_ty_2__bindgen_ty_1 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(level), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.level, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(layer), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.layer, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(offsetX), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.offsetX, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(offsetY), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.offsetY, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(offsetZ), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.offsetZ, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(extentWidth), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.extentWidth, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(extentHeight), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.extentHeight, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(extentDepth), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.extentDepth, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay
for cuda_types::cuda::CUarrayMapInfo_st__bindgen_ty_2__bindgen_ty_2 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(layer), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.layer, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(offset), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.offset, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(size), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.size, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUmemLocation_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(type_), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.type_, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(id), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.id, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUmemAllocationCompType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUmemAllocationCompType_enum::CU_MEM_ALLOCATION_COMP_NONE => {
                writer.write_all(stringify!(CU_MEM_ALLOCATION_COMP_NONE).as_bytes())
            }
            &cuda_types::cuda::CUmemAllocationCompType_enum::CU_MEM_ALLOCATION_COMP_GENERIC => {
                writer.write_all(stringify!(CU_MEM_ALLOCATION_COMP_GENERIC).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUmemAllocationProp_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(type_), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.type_, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(requestedHandleTypes), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.requestedHandleTypes, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(location), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.location, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(win32HandleMetaData), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.win32HandleMetaData, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(allocFlags), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.allocFlags, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUmemAllocationProp_st__bindgen_ty_1 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(compressionType), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.compressionType, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(gpuDirectRDMACapable), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.gpuDirectRDMACapable, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(usage), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.usage, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUmulticastGranularity_flags_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUmulticastGranularity_flags_enum::CU_MULTICAST_GRANULARITY_MINIMUM => {
                writer.write_all(stringify!(CU_MULTICAST_GRANULARITY_MINIMUM).as_bytes())
            }
            &cuda_types::cuda::CUmulticastGranularity_flags_enum::CU_MULTICAST_GRANULARITY_RECOMMENDED => {
                writer
                    .write_all(
                        stringify!(CU_MULTICAST_GRANULARITY_RECOMMENDED).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUmulticastObjectProp_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(numDevices), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.numDevices, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(size), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.size, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(handleTypes), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.handleTypes, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(flags), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.flags, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUmemAccessDesc_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(location), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.location, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(flags), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.flags, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUgraphExecUpdateResult_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUgraphExecUpdateResult_enum::CU_GRAPH_EXEC_UPDATE_SUCCESS => {
                writer.write_all(stringify!(CU_GRAPH_EXEC_UPDATE_SUCCESS).as_bytes())
            }
            &cuda_types::cuda::CUgraphExecUpdateResult_enum::CU_GRAPH_EXEC_UPDATE_ERROR => {
                writer.write_all(stringify!(CU_GRAPH_EXEC_UPDATE_ERROR).as_bytes())
            }
            &cuda_types::cuda::CUgraphExecUpdateResult_enum::CU_GRAPH_EXEC_UPDATE_ERROR_TOPOLOGY_CHANGED => {
                writer
                    .write_all(
                        stringify!(CU_GRAPH_EXEC_UPDATE_ERROR_TOPOLOGY_CHANGED)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphExecUpdateResult_enum::CU_GRAPH_EXEC_UPDATE_ERROR_NODE_TYPE_CHANGED => {
                writer
                    .write_all(
                        stringify!(CU_GRAPH_EXEC_UPDATE_ERROR_NODE_TYPE_CHANGED)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphExecUpdateResult_enum::CU_GRAPH_EXEC_UPDATE_ERROR_FUNCTION_CHANGED => {
                writer
                    .write_all(
                        stringify!(CU_GRAPH_EXEC_UPDATE_ERROR_FUNCTION_CHANGED)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphExecUpdateResult_enum::CU_GRAPH_EXEC_UPDATE_ERROR_PARAMETERS_CHANGED => {
                writer
                    .write_all(
                        stringify!(CU_GRAPH_EXEC_UPDATE_ERROR_PARAMETERS_CHANGED)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphExecUpdateResult_enum::CU_GRAPH_EXEC_UPDATE_ERROR_NOT_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(CU_GRAPH_EXEC_UPDATE_ERROR_NOT_SUPPORTED).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphExecUpdateResult_enum::CU_GRAPH_EXEC_UPDATE_ERROR_UNSUPPORTED_FUNCTION_CHANGE => {
                writer
                    .write_all(
                        stringify!(
                            CU_GRAPH_EXEC_UPDATE_ERROR_UNSUPPORTED_FUNCTION_CHANGE
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphExecUpdateResult_enum::CU_GRAPH_EXEC_UPDATE_ERROR_ATTRIBUTES_CHANGED => {
                writer
                    .write_all(
                        stringify!(CU_GRAPH_EXEC_UPDATE_ERROR_ATTRIBUTES_CHANGED)
                            .as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUgraphExecUpdateResultInfo_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(result), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.result, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(errorNode), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.errorNode, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(errorFromNode), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.errorFromNode, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUmemPool_attribute_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUmemPool_attribute_enum::CU_MEMPOOL_ATTR_REUSE_FOLLOW_EVENT_DEPENDENCIES => {
                writer
                    .write_all(
                        stringify!(CU_MEMPOOL_ATTR_REUSE_FOLLOW_EVENT_DEPENDENCIES)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUmemPool_attribute_enum::CU_MEMPOOL_ATTR_REUSE_ALLOW_OPPORTUNISTIC => {
                writer
                    .write_all(
                        stringify!(CU_MEMPOOL_ATTR_REUSE_ALLOW_OPPORTUNISTIC).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUmemPool_attribute_enum::CU_MEMPOOL_ATTR_REUSE_ALLOW_INTERNAL_DEPENDENCIES => {
                writer
                    .write_all(
                        stringify!(CU_MEMPOOL_ATTR_REUSE_ALLOW_INTERNAL_DEPENDENCIES)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUmemPool_attribute_enum::CU_MEMPOOL_ATTR_RELEASE_THRESHOLD => {
                writer
                    .write_all(stringify!(CU_MEMPOOL_ATTR_RELEASE_THRESHOLD).as_bytes())
            }
            &cuda_types::cuda::CUmemPool_attribute_enum::CU_MEMPOOL_ATTR_RESERVED_MEM_CURRENT => {
                writer
                    .write_all(
                        stringify!(CU_MEMPOOL_ATTR_RESERVED_MEM_CURRENT).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUmemPool_attribute_enum::CU_MEMPOOL_ATTR_RESERVED_MEM_HIGH => {
                writer
                    .write_all(stringify!(CU_MEMPOOL_ATTR_RESERVED_MEM_HIGH).as_bytes())
            }
            &cuda_types::cuda::CUmemPool_attribute_enum::CU_MEMPOOL_ATTR_USED_MEM_CURRENT => {
                writer.write_all(stringify!(CU_MEMPOOL_ATTR_USED_MEM_CURRENT).as_bytes())
            }
            &cuda_types::cuda::CUmemPool_attribute_enum::CU_MEMPOOL_ATTR_USED_MEM_HIGH => {
                writer.write_all(stringify!(CU_MEMPOOL_ATTR_USED_MEM_HIGH).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUmemPoolProps_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(allocType), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.allocType, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(handleTypes), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.handleTypes, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(location), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.location, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(win32SecurityAttributes), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.win32SecurityAttributes, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(maxSize), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.maxSize, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(usage), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.usage, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_MEM_ALLOC_NODE_PARAMS_v1_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(poolProps), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.poolProps, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(accessDescs), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.accessDescs, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(accessDescCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.accessDescCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(bytesize), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.bytesize, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dptr), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dptr, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_MEM_ALLOC_NODE_PARAMS_v2_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(poolProps), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.poolProps, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(accessDescs), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.accessDescs, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(accessDescCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.accessDescCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(bytesize), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.bytesize, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dptr), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dptr, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_MEM_FREE_NODE_PARAMS_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(dptr), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dptr, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUgraphMem_attribute_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUgraphMem_attribute_enum::CU_GRAPH_MEM_ATTR_USED_MEM_CURRENT => {
                writer
                    .write_all(stringify!(CU_GRAPH_MEM_ATTR_USED_MEM_CURRENT).as_bytes())
            }
            &cuda_types::cuda::CUgraphMem_attribute_enum::CU_GRAPH_MEM_ATTR_USED_MEM_HIGH => {
                writer.write_all(stringify!(CU_GRAPH_MEM_ATTR_USED_MEM_HIGH).as_bytes())
            }
            &cuda_types::cuda::CUgraphMem_attribute_enum::CU_GRAPH_MEM_ATTR_RESERVED_MEM_CURRENT => {
                writer
                    .write_all(
                        stringify!(CU_GRAPH_MEM_ATTR_RESERVED_MEM_CURRENT).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphMem_attribute_enum::CU_GRAPH_MEM_ATTR_RESERVED_MEM_HIGH => {
                writer
                    .write_all(
                        stringify!(CU_GRAPH_MEM_ATTR_RESERVED_MEM_HIGH).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_CHILD_GRAPH_NODE_PARAMS_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(graph), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.graph, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_EVENT_RECORD_NODE_PARAMS_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(event), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.event, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_EVENT_WAIT_NODE_PARAMS_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(event), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.event, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUflushGPUDirectRDMAWritesOptions_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUflushGPUDirectRDMAWritesOptions_enum::CU_FLUSH_GPU_DIRECT_RDMA_WRITES_OPTION_HOST => {
                writer
                    .write_all(
                        stringify!(CU_FLUSH_GPU_DIRECT_RDMA_WRITES_OPTION_HOST)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUflushGPUDirectRDMAWritesOptions_enum::CU_FLUSH_GPU_DIRECT_RDMA_WRITES_OPTION_MEMOPS => {
                writer
                    .write_all(
                        stringify!(CU_FLUSH_GPU_DIRECT_RDMA_WRITES_OPTION_MEMOPS)
                            .as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUGPUDirectRDMAWritesOrdering_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUGPUDirectRDMAWritesOrdering_enum::CU_GPU_DIRECT_RDMA_WRITES_ORDERING_NONE => {
                writer
                    .write_all(
                        stringify!(CU_GPU_DIRECT_RDMA_WRITES_ORDERING_NONE).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUGPUDirectRDMAWritesOrdering_enum::CU_GPU_DIRECT_RDMA_WRITES_ORDERING_OWNER => {
                writer
                    .write_all(
                        stringify!(CU_GPU_DIRECT_RDMA_WRITES_ORDERING_OWNER).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUGPUDirectRDMAWritesOrdering_enum::CU_GPU_DIRECT_RDMA_WRITES_ORDERING_ALL_DEVICES => {
                writer
                    .write_all(
                        stringify!(CU_GPU_DIRECT_RDMA_WRITES_ORDERING_ALL_DEVICES)
                            .as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUflushGPUDirectRDMAWritesScope_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUflushGPUDirectRDMAWritesScope_enum::CU_FLUSH_GPU_DIRECT_RDMA_WRITES_TO_OWNER => {
                writer
                    .write_all(
                        stringify!(CU_FLUSH_GPU_DIRECT_RDMA_WRITES_TO_OWNER).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUflushGPUDirectRDMAWritesScope_enum::CU_FLUSH_GPU_DIRECT_RDMA_WRITES_TO_ALL_DEVICES => {
                writer
                    .write_all(
                        stringify!(CU_FLUSH_GPU_DIRECT_RDMA_WRITES_TO_ALL_DEVICES)
                            .as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUflushGPUDirectRDMAWritesTarget_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUflushGPUDirectRDMAWritesTarget_enum::CU_FLUSH_GPU_DIRECT_RDMA_WRITES_TARGET_CURRENT_CTX => {
                writer
                    .write_all(
                        stringify!(CU_FLUSH_GPU_DIRECT_RDMA_WRITES_TARGET_CURRENT_CTX)
                            .as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUgraphDebugDot_flags_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUgraphDebugDot_flags_enum::CU_GRAPH_DEBUG_DOT_FLAGS_VERBOSE => {
                writer.write_all(stringify!(CU_GRAPH_DEBUG_DOT_FLAGS_VERBOSE).as_bytes())
            }
            &cuda_types::cuda::CUgraphDebugDot_flags_enum::CU_GRAPH_DEBUG_DOT_FLAGS_RUNTIME_TYPES => {
                writer
                    .write_all(
                        stringify!(CU_GRAPH_DEBUG_DOT_FLAGS_RUNTIME_TYPES).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphDebugDot_flags_enum::CU_GRAPH_DEBUG_DOT_FLAGS_KERNEL_NODE_PARAMS => {
                writer
                    .write_all(
                        stringify!(CU_GRAPH_DEBUG_DOT_FLAGS_KERNEL_NODE_PARAMS)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphDebugDot_flags_enum::CU_GRAPH_DEBUG_DOT_FLAGS_MEMCPY_NODE_PARAMS => {
                writer
                    .write_all(
                        stringify!(CU_GRAPH_DEBUG_DOT_FLAGS_MEMCPY_NODE_PARAMS)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphDebugDot_flags_enum::CU_GRAPH_DEBUG_DOT_FLAGS_MEMSET_NODE_PARAMS => {
                writer
                    .write_all(
                        stringify!(CU_GRAPH_DEBUG_DOT_FLAGS_MEMSET_NODE_PARAMS)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphDebugDot_flags_enum::CU_GRAPH_DEBUG_DOT_FLAGS_HOST_NODE_PARAMS => {
                writer
                    .write_all(
                        stringify!(CU_GRAPH_DEBUG_DOT_FLAGS_HOST_NODE_PARAMS).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphDebugDot_flags_enum::CU_GRAPH_DEBUG_DOT_FLAGS_EVENT_NODE_PARAMS => {
                writer
                    .write_all(
                        stringify!(CU_GRAPH_DEBUG_DOT_FLAGS_EVENT_NODE_PARAMS).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphDebugDot_flags_enum::CU_GRAPH_DEBUG_DOT_FLAGS_EXT_SEMAS_SIGNAL_NODE_PARAMS => {
                writer
                    .write_all(
                        stringify!(CU_GRAPH_DEBUG_DOT_FLAGS_EXT_SEMAS_SIGNAL_NODE_PARAMS)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphDebugDot_flags_enum::CU_GRAPH_DEBUG_DOT_FLAGS_EXT_SEMAS_WAIT_NODE_PARAMS => {
                writer
                    .write_all(
                        stringify!(CU_GRAPH_DEBUG_DOT_FLAGS_EXT_SEMAS_WAIT_NODE_PARAMS)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphDebugDot_flags_enum::CU_GRAPH_DEBUG_DOT_FLAGS_KERNEL_NODE_ATTRIBUTES => {
                writer
                    .write_all(
                        stringify!(CU_GRAPH_DEBUG_DOT_FLAGS_KERNEL_NODE_ATTRIBUTES)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphDebugDot_flags_enum::CU_GRAPH_DEBUG_DOT_FLAGS_HANDLES => {
                writer.write_all(stringify!(CU_GRAPH_DEBUG_DOT_FLAGS_HANDLES).as_bytes())
            }
            &cuda_types::cuda::CUgraphDebugDot_flags_enum::CU_GRAPH_DEBUG_DOT_FLAGS_MEM_ALLOC_NODE_PARAMS => {
                writer
                    .write_all(
                        stringify!(CU_GRAPH_DEBUG_DOT_FLAGS_MEM_ALLOC_NODE_PARAMS)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphDebugDot_flags_enum::CU_GRAPH_DEBUG_DOT_FLAGS_MEM_FREE_NODE_PARAMS => {
                writer
                    .write_all(
                        stringify!(CU_GRAPH_DEBUG_DOT_FLAGS_MEM_FREE_NODE_PARAMS)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphDebugDot_flags_enum::CU_GRAPH_DEBUG_DOT_FLAGS_BATCH_MEM_OP_NODE_PARAMS => {
                writer
                    .write_all(
                        stringify!(CU_GRAPH_DEBUG_DOT_FLAGS_BATCH_MEM_OP_NODE_PARAMS)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphDebugDot_flags_enum::CU_GRAPH_DEBUG_DOT_FLAGS_EXTRA_TOPO_INFO => {
                writer
                    .write_all(
                        stringify!(CU_GRAPH_DEBUG_DOT_FLAGS_EXTRA_TOPO_INFO).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphDebugDot_flags_enum::CU_GRAPH_DEBUG_DOT_FLAGS_CONDITIONAL_NODE_PARAMS => {
                writer
                    .write_all(
                        stringify!(CU_GRAPH_DEBUG_DOT_FLAGS_CONDITIONAL_NODE_PARAMS)
                            .as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUuserObject_flags_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUuserObject_flags_enum::CU_USER_OBJECT_NO_DESTRUCTOR_SYNC => {
                writer
                    .write_all(stringify!(CU_USER_OBJECT_NO_DESTRUCTOR_SYNC).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUuserObjectRetain_flags_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUuserObjectRetain_flags_enum::CU_GRAPH_USER_OBJECT_MOVE => {
                writer.write_all(stringify!(CU_GRAPH_USER_OBJECT_MOVE).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUgraphInstantiate_flags_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUgraphInstantiate_flags_enum::CUDA_GRAPH_INSTANTIATE_FLAG_AUTO_FREE_ON_LAUNCH => {
                writer
                    .write_all(
                        stringify!(CUDA_GRAPH_INSTANTIATE_FLAG_AUTO_FREE_ON_LAUNCH)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphInstantiate_flags_enum::CUDA_GRAPH_INSTANTIATE_FLAG_UPLOAD => {
                writer
                    .write_all(stringify!(CUDA_GRAPH_INSTANTIATE_FLAG_UPLOAD).as_bytes())
            }
            &cuda_types::cuda::CUgraphInstantiate_flags_enum::CUDA_GRAPH_INSTANTIATE_FLAG_DEVICE_LAUNCH => {
                writer
                    .write_all(
                        stringify!(CUDA_GRAPH_INSTANTIATE_FLAG_DEVICE_LAUNCH).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUgraphInstantiate_flags_enum::CUDA_GRAPH_INSTANTIATE_FLAG_USE_NODE_PRIORITY => {
                writer
                    .write_all(
                        stringify!(CUDA_GRAPH_INSTANTIATE_FLAG_USE_NODE_PRIORITY)
                            .as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUdeviceNumaConfig_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUdeviceNumaConfig_enum::CU_DEVICE_NUMA_CONFIG_NONE => {
                writer.write_all(stringify!(CU_DEVICE_NUMA_CONFIG_NONE).as_bytes())
            }
            &cuda_types::cuda::CUdeviceNumaConfig_enum::CU_DEVICE_NUMA_CONFIG_NUMA_NODE => {
                writer.write_all(stringify!(CU_DEVICE_NUMA_CONFIG_NUMA_NODE).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUprocessState_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUprocessState_enum::CU_PROCESS_STATE_RUNNING => {
                writer.write_all(stringify!(CU_PROCESS_STATE_RUNNING).as_bytes())
            }
            &cuda_types::cuda::CUprocessState_enum::CU_PROCESS_STATE_LOCKED => {
                writer.write_all(stringify!(CU_PROCESS_STATE_LOCKED).as_bytes())
            }
            &cuda_types::cuda::CUprocessState_enum::CU_PROCESS_STATE_CHECKPOINTED => {
                writer.write_all(stringify!(CU_PROCESS_STATE_CHECKPOINTED).as_bytes())
            }
            &cuda_types::cuda::CUprocessState_enum::CU_PROCESS_STATE_FAILED => {
                writer.write_all(stringify!(CU_PROCESS_STATE_FAILED).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUcheckpointLockArgs_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(timeoutMs), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.timeoutMs, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUmemcpyFlags_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUmemcpyFlags_enum::CU_MEMCPY_FLAG_DEFAULT => {
                writer.write_all(stringify!(CU_MEMCPY_FLAG_DEFAULT).as_bytes())
            }
            &cuda_types::cuda::CUmemcpyFlags_enum::CU_MEMCPY_FLAG_PREFER_OVERLAP_WITH_COMPUTE => {
                writer
                    .write_all(
                        stringify!(CU_MEMCPY_FLAG_PREFER_OVERLAP_WITH_COMPUTE).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUmemcpySrcAccessOrder_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUmemcpySrcAccessOrder_enum::CU_MEMCPY_SRC_ACCESS_ORDER_INVALID => {
                writer
                    .write_all(stringify!(CU_MEMCPY_SRC_ACCESS_ORDER_INVALID).as_bytes())
            }
            &cuda_types::cuda::CUmemcpySrcAccessOrder_enum::CU_MEMCPY_SRC_ACCESS_ORDER_STREAM => {
                writer
                    .write_all(stringify!(CU_MEMCPY_SRC_ACCESS_ORDER_STREAM).as_bytes())
            }
            &cuda_types::cuda::CUmemcpySrcAccessOrder_enum::CU_MEMCPY_SRC_ACCESS_ORDER_DURING_API_CALL => {
                writer
                    .write_all(
                        stringify!(CU_MEMCPY_SRC_ACCESS_ORDER_DURING_API_CALL).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUmemcpySrcAccessOrder_enum::CU_MEMCPY_SRC_ACCESS_ORDER_ANY => {
                writer.write_all(stringify!(CU_MEMCPY_SRC_ACCESS_ORDER_ANY).as_bytes())
            }
            &cuda_types::cuda::CUmemcpySrcAccessOrder_enum::CU_MEMCPY_SRC_ACCESS_ORDER_MAX => {
                writer.write_all(stringify!(CU_MEMCPY_SRC_ACCESS_ORDER_MAX).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUmemcpyAttributes_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(srcAccessOrder), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcAccessOrder, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcLocHint), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcLocHint, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstLocHint), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstLocHint, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(flags), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.flags, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUmemcpy3DOperandType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUmemcpy3DOperandType_enum::CU_MEMCPY_OPERAND_TYPE_POINTER => {
                writer.write_all(stringify!(CU_MEMCPY_OPERAND_TYPE_POINTER).as_bytes())
            }
            &cuda_types::cuda::CUmemcpy3DOperandType_enum::CU_MEMCPY_OPERAND_TYPE_ARRAY => {
                writer.write_all(stringify!(CU_MEMCPY_OPERAND_TYPE_ARRAY).as_bytes())
            }
            &cuda_types::cuda::CUmemcpy3DOperandType_enum::CU_MEMCPY_OPERAND_TYPE_MAX => {
                writer.write_all(stringify!(CU_MEMCPY_OPERAND_TYPE_MAX).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUoffset3D_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(x), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.x, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(y), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.y, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(z), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.z, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUextent3D_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(width), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.width, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(height), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.height, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(depth), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.depth, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay
for cuda_types::cuda::CUmemcpy3DOperand_st__bindgen_ty_1__bindgen_ty_1 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(ptr), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.ptr, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(rowLength), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.rowLength, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(layerHeight), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.layerHeight, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(locHint), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.locHint, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay
for cuda_types::cuda::CUmemcpy3DOperand_st__bindgen_ty_1__bindgen_ty_2 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(array), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.array, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(offset), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.offset, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_MEMCPY3D_BATCH_OP_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(src), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.src, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dst), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dst, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(extent), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.extent, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcAccessOrder), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcAccessOrder, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(flags), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.flags, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
pub fn write_cuGetErrorString(
    writer: &mut (impl std::io::Write + ?Sized),
    error: cuda_types::cuda::CUresult,
    pStr: *mut *const ::core::ffi::c_char,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(error), ": ").as_bytes())?;
    crate::CudaDisplay::write(&error, "cuGetErrorString", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pStr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pStr, "cuGetErrorString", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGetErrorName(
    writer: &mut (impl std::io::Write + ?Sized),
    error: cuda_types::cuda::CUresult,
    pStr: *mut *const ::core::ffi::c_char,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(error), ": ").as_bytes())?;
    crate::CudaDisplay::write(&error, "cuGetErrorName", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pStr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pStr, "cuGetErrorName", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuInit(
    writer: &mut (impl std::io::Write + ?Sized),
    Flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(Flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Flags, "cuInit", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuDriverGetVersion(
    writer: &mut (impl std::io::Write + ?Sized),
    driverVersion: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(driverVersion), ": ").as_bytes())?;
    crate::CudaDisplay::write(&driverVersion, "cuDriverGetVersion", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuDeviceGet(
    writer: &mut (impl std::io::Write + ?Sized),
    device: *mut cuda_types::cuda::CUdevice,
    ordinal: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "cuDeviceGet", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ordinal), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ordinal, "cuDeviceGet", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuDeviceGetCount(
    writer: &mut (impl std::io::Write + ?Sized),
    count: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "cuDeviceGetCount", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuDeviceGetName(
    writer: &mut (impl std::io::Write + ?Sized),
    name: *mut ::core::ffi::c_char,
    len: ::core::ffi::c_int,
    dev: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(name), ": ").as_bytes())?;
    crate::CudaDisplay::write(&name, "cuDeviceGetName", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(len), ": ").as_bytes())?;
    crate::CudaDisplay::write(&len, "cuDeviceGetName", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuDeviceGetName", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuDeviceGetUuid(
    writer: &mut (impl std::io::Write + ?Sized),
    uuid: *mut cuda_types::cuda::CUuuid,
    dev: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(uuid), ": ").as_bytes())?;
    crate::CudaDisplay::write(&uuid, "cuDeviceGetUuid", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuDeviceGetUuid", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuDeviceGetUuid_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    uuid: *mut cuda_types::cuda::CUuuid,
    dev: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(uuid), ": ").as_bytes())?;
    crate::CudaDisplay::write(&uuid, "cuDeviceGetUuid_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuDeviceGetUuid_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuDeviceGetLuid(
    writer: &mut (impl std::io::Write + ?Sized),
    luid: *mut ::core::ffi::c_char,
    deviceNodeMask: *mut ::core::ffi::c_uint,
    dev: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(luid), ": ").as_bytes())?;
    crate::CudaDisplay::write(&luid, "cuDeviceGetLuid", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(deviceNodeMask), ": ").as_bytes())?;
    crate::CudaDisplay::write(&deviceNodeMask, "cuDeviceGetLuid", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuDeviceGetLuid", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuDeviceTotalMem_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    bytes: *mut usize,
    dev: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(bytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&bytes, "cuDeviceTotalMem_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuDeviceTotalMem_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuDeviceGetTexture1DLinearMaxWidth(
    writer: &mut (impl std::io::Write + ?Sized),
    maxWidthInElements: *mut usize,
    format: cuda_types::cuda::CUarray_format,
    numChannels: ::core::ffi::c_uint,
    dev: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(maxWidthInElements), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &maxWidthInElements,
        "cuDeviceGetTexture1DLinearMaxWidth",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(format), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &format,
        "cuDeviceGetTexture1DLinearMaxWidth",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numChannels), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numChannels,
        "cuDeviceGetTexture1DLinearMaxWidth",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dev,
        "cuDeviceGetTexture1DLinearMaxWidth",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuDeviceGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    pi: *mut ::core::ffi::c_int,
    attrib: cuda_types::cuda::CUdevice_attribute,
    dev: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pi), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pi, "cuDeviceGetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attrib), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attrib, "cuDeviceGetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuDeviceGetAttribute", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuDeviceGetNvSciSyncAttributes(
    writer: &mut (impl std::io::Write + ?Sized),
    nvSciSyncAttrList: *mut ::core::ffi::c_void,
    dev: cuda_types::cuda::CUdevice,
    flags: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(nvSciSyncAttrList), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nvSciSyncAttrList,
        "cuDeviceGetNvSciSyncAttributes",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuDeviceGetNvSciSyncAttributes", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &flags,
        "cuDeviceGetNvSciSyncAttributes",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuDeviceSetMemPool(
    writer: &mut (impl std::io::Write + ?Sized),
    dev: cuda_types::cuda::CUdevice,
    pool: cuda_types::cuda::CUmemoryPool,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuDeviceSetMemPool", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pool), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pool, "cuDeviceSetMemPool", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuDeviceGetMemPool(
    writer: &mut (impl std::io::Write + ?Sized),
    pool: *mut cuda_types::cuda::CUmemoryPool,
    dev: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pool), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pool, "cuDeviceGetMemPool", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuDeviceGetMemPool", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuDeviceGetDefaultMemPool(
    writer: &mut (impl std::io::Write + ?Sized),
    pool_out: *mut cuda_types::cuda::CUmemoryPool,
    dev: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pool_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pool_out, "cuDeviceGetDefaultMemPool", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuDeviceGetDefaultMemPool", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuDeviceGetExecAffinitySupport(
    writer: &mut (impl std::io::Write + ?Sized),
    pi: *mut ::core::ffi::c_int,
    type_: cuda_types::cuda::CUexecAffinityType,
    dev: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pi), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pi, "cuDeviceGetExecAffinitySupport", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &type_,
        "cuDeviceGetExecAffinitySupport",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuDeviceGetExecAffinitySupport", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuFlushGPUDirectRDMAWrites(
    writer: &mut (impl std::io::Write + ?Sized),
    target: cuda_types::cuda::CUflushGPUDirectRDMAWritesTarget,
    scope: cuda_types::cuda::CUflushGPUDirectRDMAWritesScope,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(target), ": ").as_bytes())?;
    crate::CudaDisplay::write(&target, "cuFlushGPUDirectRDMAWrites", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(scope), ": ").as_bytes())?;
    crate::CudaDisplay::write(&scope, "cuFlushGPUDirectRDMAWrites", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuDeviceGetProperties(
    writer: &mut (impl std::io::Write + ?Sized),
    prop: *mut cuda_types::cuda::CUdevprop,
    dev: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(prop), ": ").as_bytes())?;
    crate::CudaDisplay::write(&prop, "cuDeviceGetProperties", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuDeviceGetProperties", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuDeviceComputeCapability(
    writer: &mut (impl std::io::Write + ?Sized),
    major: *mut ::core::ffi::c_int,
    minor: *mut ::core::ffi::c_int,
    dev: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(major), ": ").as_bytes())?;
    crate::CudaDisplay::write(&major, "cuDeviceComputeCapability", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(minor), ": ").as_bytes())?;
    crate::CudaDisplay::write(&minor, "cuDeviceComputeCapability", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuDeviceComputeCapability", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuDevicePrimaryCtxRetain(
    writer: &mut (impl std::io::Write + ?Sized),
    pctx: *mut cuda_types::cuda::CUcontext,
    dev: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pctx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pctx, "cuDevicePrimaryCtxRetain", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuDevicePrimaryCtxRetain", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuDevicePrimaryCtxRelease_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    dev: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuDevicePrimaryCtxRelease_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuDevicePrimaryCtxSetFlags_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    dev: cuda_types::cuda::CUdevice,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuDevicePrimaryCtxSetFlags_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuDevicePrimaryCtxSetFlags_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuDevicePrimaryCtxGetState(
    writer: &mut (impl std::io::Write + ?Sized),
    dev: cuda_types::cuda::CUdevice,
    flags: *mut ::core::ffi::c_uint,
    active: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuDevicePrimaryCtxGetState", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuDevicePrimaryCtxGetState", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(active), ": ").as_bytes())?;
    crate::CudaDisplay::write(&active, "cuDevicePrimaryCtxGetState", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuDevicePrimaryCtxReset_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    dev: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuDevicePrimaryCtxReset_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxCreate_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    pctx: *mut cuda_types::cuda::CUcontext,
    flags: ::core::ffi::c_uint,
    dev: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pctx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pctx, "cuCtxCreate_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuCtxCreate_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuCtxCreate_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxCreate_v3(
    writer: &mut (impl std::io::Write + ?Sized),
    pctx: *mut cuda_types::cuda::CUcontext,
    paramsArray: *mut cuda_types::cuda::CUexecAffinityParam,
    numParams: ::core::ffi::c_int,
    flags: ::core::ffi::c_uint,
    dev: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pctx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pctx, "cuCtxCreate_v3", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(paramsArray), ": ").as_bytes())?;
    writer.write_all(b"[")?;
    for i in 0..numParams {
        if i != 0 {
            writer.write_all(b", ")?;
        }
        crate::CudaDisplay::write(
            unsafe { &*paramsArray.add(i as usize) },
            "cuCtxCreate_v3",
            arg_idx,
            writer,
        )?;
    }
    writer.write_all(b"]")?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numParams, "cuCtxCreate_v3", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuCtxCreate_v3", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuCtxCreate_v3", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxCreate_v4(
    writer: &mut (impl std::io::Write + ?Sized),
    pctx: *mut cuda_types::cuda::CUcontext,
    ctxCreateParams: *mut cuda_types::cuda::CUctxCreateParams,
    flags: ::core::ffi::c_uint,
    dev: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pctx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pctx, "cuCtxCreate_v4", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ctxCreateParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ctxCreateParams, "cuCtxCreate_v4", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuCtxCreate_v4", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuCtxCreate_v4", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxDestroy_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    ctx: cuda_types::cuda::CUcontext,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(ctx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ctx, "cuCtxDestroy_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxPushCurrent_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    ctx: cuda_types::cuda::CUcontext,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(ctx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ctx, "cuCtxPushCurrent_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxPopCurrent_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    pctx: *mut cuda_types::cuda::CUcontext,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pctx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pctx, "cuCtxPopCurrent_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxSetCurrent(
    writer: &mut (impl std::io::Write + ?Sized),
    ctx: cuda_types::cuda::CUcontext,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(ctx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ctx, "cuCtxSetCurrent", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxGetCurrent(
    writer: &mut (impl std::io::Write + ?Sized),
    pctx: *mut cuda_types::cuda::CUcontext,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pctx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pctx, "cuCtxGetCurrent", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxGetDevice(
    writer: &mut (impl std::io::Write + ?Sized),
    device: *mut cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "cuCtxGetDevice", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxGetFlags(
    writer: &mut (impl std::io::Write + ?Sized),
    flags: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuCtxGetFlags", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxSetFlags(
    writer: &mut (impl std::io::Write + ?Sized),
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuCtxSetFlags", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxGetId(
    writer: &mut (impl std::io::Write + ?Sized),
    ctx: cuda_types::cuda::CUcontext,
    ctxId: *mut ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(ctx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ctx, "cuCtxGetId", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ctxId), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ctxId, "cuCtxGetId", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxSynchronize(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
pub fn write_cuCtxSetLimit(
    writer: &mut (impl std::io::Write + ?Sized),
    limit: cuda_types::cuda::CUlimit,
    value: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(limit), ": ").as_bytes())?;
    crate::CudaDisplay::write(&limit, "cuCtxSetLimit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cuCtxSetLimit", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxGetLimit(
    writer: &mut (impl std::io::Write + ?Sized),
    pvalue: *mut usize,
    limit: cuda_types::cuda::CUlimit,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pvalue), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pvalue, "cuCtxGetLimit", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(limit), ": ").as_bytes())?;
    crate::CudaDisplay::write(&limit, "cuCtxGetLimit", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxGetCacheConfig(
    writer: &mut (impl std::io::Write + ?Sized),
    pconfig: *mut cuda_types::cuda::CUfunc_cache,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pconfig), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pconfig, "cuCtxGetCacheConfig", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxSetCacheConfig(
    writer: &mut (impl std::io::Write + ?Sized),
    config: cuda_types::cuda::CUfunc_cache,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(config), ": ").as_bytes())?;
    crate::CudaDisplay::write(&config, "cuCtxSetCacheConfig", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxGetApiVersion(
    writer: &mut (impl std::io::Write + ?Sized),
    ctx: cuda_types::cuda::CUcontext,
    version: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(ctx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ctx, "cuCtxGetApiVersion", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(version), ": ").as_bytes())?;
    crate::CudaDisplay::write(&version, "cuCtxGetApiVersion", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxGetStreamPriorityRange(
    writer: &mut (impl std::io::Write + ?Sized),
    leastPriority: *mut ::core::ffi::c_int,
    greatestPriority: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(leastPriority), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &leastPriority,
        "cuCtxGetStreamPriorityRange",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(greatestPriority), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &greatestPriority,
        "cuCtxGetStreamPriorityRange",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuCtxResetPersistingL2Cache(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
pub fn write_cuCtxGetExecAffinity(
    writer: &mut (impl std::io::Write + ?Sized),
    pExecAffinity: *mut cuda_types::cuda::CUexecAffinityParam,
    type_: cuda_types::cuda::CUexecAffinityType,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pExecAffinity), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pExecAffinity, "cuCtxGetExecAffinity", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cuCtxGetExecAffinity", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxRecordEvent(
    writer: &mut (impl std::io::Write + ?Sized),
    hCtx: cuda_types::cuda::CUcontext,
    hEvent: cuda_types::cuda::CUevent,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hCtx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hCtx, "cuCtxRecordEvent", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hEvent), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hEvent, "cuCtxRecordEvent", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxWaitEvent(
    writer: &mut (impl std::io::Write + ?Sized),
    hCtx: cuda_types::cuda::CUcontext,
    hEvent: cuda_types::cuda::CUevent,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hCtx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hCtx, "cuCtxWaitEvent", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hEvent), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hEvent, "cuCtxWaitEvent", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxAttach(
    writer: &mut (impl std::io::Write + ?Sized),
    pctx: *mut cuda_types::cuda::CUcontext,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pctx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pctx, "cuCtxAttach", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuCtxAttach", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxDetach(
    writer: &mut (impl std::io::Write + ?Sized),
    ctx: cuda_types::cuda::CUcontext,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(ctx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ctx, "cuCtxDetach", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxGetSharedMemConfig(
    writer: &mut (impl std::io::Write + ?Sized),
    pConfig: *mut cuda_types::cuda::CUsharedconfig,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pConfig), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pConfig, "cuCtxGetSharedMemConfig", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxSetSharedMemConfig(
    writer: &mut (impl std::io::Write + ?Sized),
    config: cuda_types::cuda::CUsharedconfig,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(config), ": ").as_bytes())?;
    crate::CudaDisplay::write(&config, "cuCtxSetSharedMemConfig", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuModuleLoad(
    writer: &mut (impl std::io::Write + ?Sized),
    module: *mut cuda_types::cuda::CUmodule,
    fname: *const ::core::ffi::c_char,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(module), ": ").as_bytes())?;
    crate::CudaDisplay::write(&module, "cuModuleLoad", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(fname), ": ").as_bytes())?;
    crate::CudaDisplay::write(&fname, "cuModuleLoad", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuModuleLoadData(
    writer: &mut (impl std::io::Write + ?Sized),
    module: *mut cuda_types::cuda::CUmodule,
    image: *const ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(module), ": ").as_bytes())?;
    crate::CudaDisplay::write(&module, "cuModuleLoadData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(image), ": ").as_bytes())?;
    crate::CudaDisplay::write(&image, "cuModuleLoadData", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuModuleLoadDataEx(
    writer: &mut (impl std::io::Write + ?Sized),
    module: *mut cuda_types::cuda::CUmodule,
    image: *const ::core::ffi::c_void,
    numOptions: ::core::ffi::c_uint,
    options: *mut cuda_types::cuda::CUjit_option,
    optionValues: *mut *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(module), ": ").as_bytes())?;
    crate::CudaDisplay::write(&module, "cuModuleLoadDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(image), ": ").as_bytes())?;
    crate::CudaDisplay::write(&image, "cuModuleLoadDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numOptions), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numOptions, "cuModuleLoadDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(options), ": ").as_bytes())?;
    crate::CudaDisplay::write(&options, "cuModuleLoadDataEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(optionValues), ": ").as_bytes())?;
    crate::CudaDisplay::write(&optionValues, "cuModuleLoadDataEx", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuModuleLoadFatBinary(
    writer: &mut (impl std::io::Write + ?Sized),
    module: *mut cuda_types::cuda::CUmodule,
    fatCubin: *const ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(module), ": ").as_bytes())?;
    crate::CudaDisplay::write(&module, "cuModuleLoadFatBinary", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(fatCubin), ": ").as_bytes())?;
    crate::CudaDisplay::write(&fatCubin, "cuModuleLoadFatBinary", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuModuleUnload(
    writer: &mut (impl std::io::Write + ?Sized),
    hmod: cuda_types::cuda::CUmodule,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hmod), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hmod, "cuModuleUnload", arg_idx, writer)?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::cuda::CUmoduleLoadingMode_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUmoduleLoadingMode_enum::CU_MODULE_EAGER_LOADING => {
                writer.write_all(stringify!(CU_MODULE_EAGER_LOADING).as_bytes())
            }
            &cuda_types::cuda::CUmoduleLoadingMode_enum::CU_MODULE_LAZY_LOADING => {
                writer.write_all(stringify!(CU_MODULE_LAZY_LOADING).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cuModuleGetLoadingMode(
    writer: &mut (impl std::io::Write + ?Sized),
    mode: *mut cuda_types::cuda::CUmoduleLoadingMode,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mode, "cuModuleGetLoadingMode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuModuleGetFunction(
    writer: &mut (impl std::io::Write + ?Sized),
    hfunc: *mut cuda_types::cuda::CUfunction,
    hmod: cuda_types::cuda::CUmodule,
    name: *const ::core::ffi::c_char,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hfunc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hfunc, "cuModuleGetFunction", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hmod), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hmod, "cuModuleGetFunction", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(name), ": ").as_bytes())?;
    crate::CudaDisplay::write(&name, "cuModuleGetFunction", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuModuleGetFunctionCount(
    writer: &mut (impl std::io::Write + ?Sized),
    count: *mut ::core::ffi::c_uint,
    mod_: cuda_types::cuda::CUmodule,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "cuModuleGetFunctionCount", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mod_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mod_, "cuModuleGetFunctionCount", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuModuleEnumerateFunctions(
    writer: &mut (impl std::io::Write + ?Sized),
    functions: *mut cuda_types::cuda::CUfunction,
    numFunctions: ::core::ffi::c_uint,
    mod_: cuda_types::cuda::CUmodule,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(functions), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &functions,
        "cuModuleEnumerateFunctions",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numFunctions), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numFunctions,
        "cuModuleEnumerateFunctions",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mod_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mod_, "cuModuleEnumerateFunctions", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuModuleGetGlobal_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    dptr: *mut cuda_types::cuda::CUdeviceptr,
    bytes: *mut usize,
    hmod: cuda_types::cuda::CUmodule,
    name: *const ::core::ffi::c_char,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuModuleGetGlobal_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&bytes, "cuModuleGetGlobal_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hmod), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hmod, "cuModuleGetGlobal_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(name), ": ").as_bytes())?;
    crate::CudaDisplay::write(&name, "cuModuleGetGlobal_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuLinkCreate_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    numOptions: ::core::ffi::c_uint,
    options: *mut cuda_types::cuda::CUjit_option,
    optionValues: *mut *mut ::core::ffi::c_void,
    stateOut: *mut cuda_types::cuda::CUlinkState,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(numOptions), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numOptions, "cuLinkCreate_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(options), ": ").as_bytes())?;
    crate::CudaDisplay::write(&options, "cuLinkCreate_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(optionValues), ": ").as_bytes())?;
    crate::CudaDisplay::write(&optionValues, "cuLinkCreate_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(stateOut), ": ").as_bytes())?;
    crate::CudaDisplay::write(&stateOut, "cuLinkCreate_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuLinkAddData_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    state: cuda_types::cuda::CUlinkState,
    type_: cuda_types::cuda::CUjitInputType,
    data: *mut ::core::ffi::c_void,
    size: usize,
    name: *const ::core::ffi::c_char,
    numOptions: ::core::ffi::c_uint,
    options: *mut cuda_types::cuda::CUjit_option,
    optionValues: *mut *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(state), ": ").as_bytes())?;
    crate::CudaDisplay::write(&state, "cuLinkAddData_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cuLinkAddData_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(data), ": ").as_bytes())?;
    crate::CudaDisplay::write(&data, "cuLinkAddData_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(&size, "cuLinkAddData_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(name), ": ").as_bytes())?;
    crate::CudaDisplay::write(&name, "cuLinkAddData_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numOptions), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numOptions, "cuLinkAddData_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(options), ": ").as_bytes())?;
    crate::CudaDisplay::write(&options, "cuLinkAddData_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(optionValues), ": ").as_bytes())?;
    crate::CudaDisplay::write(&optionValues, "cuLinkAddData_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuLinkAddFile_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    state: cuda_types::cuda::CUlinkState,
    type_: cuda_types::cuda::CUjitInputType,
    path: *const ::core::ffi::c_char,
    numOptions: ::core::ffi::c_uint,
    options: *mut cuda_types::cuda::CUjit_option,
    optionValues: *mut *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(state), ": ").as_bytes())?;
    crate::CudaDisplay::write(&state, "cuLinkAddFile_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cuLinkAddFile_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(path), ": ").as_bytes())?;
    crate::CudaDisplay::write(&path, "cuLinkAddFile_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numOptions), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numOptions, "cuLinkAddFile_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(options), ": ").as_bytes())?;
    crate::CudaDisplay::write(&options, "cuLinkAddFile_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(optionValues), ": ").as_bytes())?;
    crate::CudaDisplay::write(&optionValues, "cuLinkAddFile_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuLinkComplete(
    writer: &mut (impl std::io::Write + ?Sized),
    state: cuda_types::cuda::CUlinkState,
    cubinOut: *mut *mut ::core::ffi::c_void,
    sizeOut: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(state), ": ").as_bytes())?;
    crate::CudaDisplay::write(&state, "cuLinkComplete", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cubinOut), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cubinOut, "cuLinkComplete", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeOut), ": ").as_bytes())?;
    crate::CudaDisplay::write(&sizeOut, "cuLinkComplete", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuLinkDestroy(
    writer: &mut (impl std::io::Write + ?Sized),
    state: cuda_types::cuda::CUlinkState,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(state), ": ").as_bytes())?;
    crate::CudaDisplay::write(&state, "cuLinkDestroy", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuModuleGetTexRef(
    writer: &mut (impl std::io::Write + ?Sized),
    pTexRef: *mut cuda_types::cuda::CUtexref,
    hmod: cuda_types::cuda::CUmodule,
    name: *const ::core::ffi::c_char,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pTexRef, "cuModuleGetTexRef", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hmod), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hmod, "cuModuleGetTexRef", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(name), ": ").as_bytes())?;
    crate::CudaDisplay::write(&name, "cuModuleGetTexRef", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuModuleGetSurfRef(
    writer: &mut (impl std::io::Write + ?Sized),
    pSurfRef: *mut cuda_types::cuda::CUsurfref,
    hmod: cuda_types::cuda::CUmodule,
    name: *const ::core::ffi::c_char,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pSurfRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pSurfRef, "cuModuleGetSurfRef", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hmod), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hmod, "cuModuleGetSurfRef", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(name), ": ").as_bytes())?;
    crate::CudaDisplay::write(&name, "cuModuleGetSurfRef", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuLibraryLoadData(
    writer: &mut (impl std::io::Write + ?Sized),
    library: *mut cuda_types::cuda::CUlibrary,
    code: *const ::core::ffi::c_void,
    jitOptions: *mut cuda_types::cuda::CUjit_option,
    jitOptionsValues: *mut *mut ::core::ffi::c_void,
    numJitOptions: ::core::ffi::c_uint,
    libraryOptions: *mut cuda_types::cuda::CUlibraryOption,
    libraryOptionValues: *mut *mut ::core::ffi::c_void,
    numLibraryOptions: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(library), ": ").as_bytes())?;
    crate::CudaDisplay::write(&library, "cuLibraryLoadData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(code), ": ").as_bytes())?;
    crate::CudaDisplay::write(&code, "cuLibraryLoadData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(jitOptions), ": ").as_bytes())?;
    crate::CudaDisplay::write(&jitOptions, "cuLibraryLoadData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(jitOptionsValues), ": ").as_bytes())?;
    crate::CudaDisplay::write(&jitOptionsValues, "cuLibraryLoadData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numJitOptions), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numJitOptions, "cuLibraryLoadData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(libraryOptions), ": ").as_bytes())?;
    crate::CudaDisplay::write(&libraryOptions, "cuLibraryLoadData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(libraryOptionValues), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &libraryOptionValues,
        "cuLibraryLoadData",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numLibraryOptions), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numLibraryOptions, "cuLibraryLoadData", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuLibraryLoadFromFile(
    writer: &mut (impl std::io::Write + ?Sized),
    library: *mut cuda_types::cuda::CUlibrary,
    fileName: *const ::core::ffi::c_char,
    jitOptions: *mut cuda_types::cuda::CUjit_option,
    jitOptionsValues: *mut *mut ::core::ffi::c_void,
    numJitOptions: ::core::ffi::c_uint,
    libraryOptions: *mut cuda_types::cuda::CUlibraryOption,
    libraryOptionValues: *mut *mut ::core::ffi::c_void,
    numLibraryOptions: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(library), ": ").as_bytes())?;
    crate::CudaDisplay::write(&library, "cuLibraryLoadFromFile", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(fileName), ": ").as_bytes())?;
    crate::CudaDisplay::write(&fileName, "cuLibraryLoadFromFile", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(jitOptions), ": ").as_bytes())?;
    crate::CudaDisplay::write(&jitOptions, "cuLibraryLoadFromFile", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(jitOptionsValues), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &jitOptionsValues,
        "cuLibraryLoadFromFile",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numJitOptions), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numJitOptions, "cuLibraryLoadFromFile", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(libraryOptions), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &libraryOptions,
        "cuLibraryLoadFromFile",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(libraryOptionValues), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &libraryOptionValues,
        "cuLibraryLoadFromFile",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numLibraryOptions), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numLibraryOptions,
        "cuLibraryLoadFromFile",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuLibraryUnload(
    writer: &mut (impl std::io::Write + ?Sized),
    library: cuda_types::cuda::CUlibrary,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(library), ": ").as_bytes())?;
    crate::CudaDisplay::write(&library, "cuLibraryUnload", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuLibraryGetKernel(
    writer: &mut (impl std::io::Write + ?Sized),
    pKernel: *mut cuda_types::cuda::CUkernel,
    library: cuda_types::cuda::CUlibrary,
    name: *const ::core::ffi::c_char,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pKernel), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pKernel, "cuLibraryGetKernel", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(library), ": ").as_bytes())?;
    crate::CudaDisplay::write(&library, "cuLibraryGetKernel", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(name), ": ").as_bytes())?;
    crate::CudaDisplay::write(&name, "cuLibraryGetKernel", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuLibraryGetKernelCount(
    writer: &mut (impl std::io::Write + ?Sized),
    count: *mut ::core::ffi::c_uint,
    lib: cuda_types::cuda::CUlibrary,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "cuLibraryGetKernelCount", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(lib), ": ").as_bytes())?;
    crate::CudaDisplay::write(&lib, "cuLibraryGetKernelCount", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuLibraryEnumerateKernels(
    writer: &mut (impl std::io::Write + ?Sized),
    kernels: *mut cuda_types::cuda::CUkernel,
    numKernels: ::core::ffi::c_uint,
    lib: cuda_types::cuda::CUlibrary,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(kernels), ": ").as_bytes())?;
    crate::CudaDisplay::write(&kernels, "cuLibraryEnumerateKernels", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numKernels), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numKernels,
        "cuLibraryEnumerateKernels",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(lib), ": ").as_bytes())?;
    crate::CudaDisplay::write(&lib, "cuLibraryEnumerateKernels", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuLibraryGetModule(
    writer: &mut (impl std::io::Write + ?Sized),
    pMod: *mut cuda_types::cuda::CUmodule,
    library: cuda_types::cuda::CUlibrary,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pMod), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pMod, "cuLibraryGetModule", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(library), ": ").as_bytes())?;
    crate::CudaDisplay::write(&library, "cuLibraryGetModule", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuKernelGetFunction(
    writer: &mut (impl std::io::Write + ?Sized),
    pFunc: *mut cuda_types::cuda::CUfunction,
    kernel: cuda_types::cuda::CUkernel,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pFunc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pFunc, "cuKernelGetFunction", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(kernel), ": ").as_bytes())?;
    crate::CudaDisplay::write(&kernel, "cuKernelGetFunction", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuKernelGetLibrary(
    writer: &mut (impl std::io::Write + ?Sized),
    pLib: *mut cuda_types::cuda::CUlibrary,
    kernel: cuda_types::cuda::CUkernel,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pLib), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pLib, "cuKernelGetLibrary", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(kernel), ": ").as_bytes())?;
    crate::CudaDisplay::write(&kernel, "cuKernelGetLibrary", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuLibraryGetGlobal(
    writer: &mut (impl std::io::Write + ?Sized),
    dptr: *mut cuda_types::cuda::CUdeviceptr,
    bytes: *mut usize,
    library: cuda_types::cuda::CUlibrary,
    name: *const ::core::ffi::c_char,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuLibraryGetGlobal", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&bytes, "cuLibraryGetGlobal", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(library), ": ").as_bytes())?;
    crate::CudaDisplay::write(&library, "cuLibraryGetGlobal", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(name), ": ").as_bytes())?;
    crate::CudaDisplay::write(&name, "cuLibraryGetGlobal", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuLibraryGetManaged(
    writer: &mut (impl std::io::Write + ?Sized),
    dptr: *mut cuda_types::cuda::CUdeviceptr,
    bytes: *mut usize,
    library: cuda_types::cuda::CUlibrary,
    name: *const ::core::ffi::c_char,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuLibraryGetManaged", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&bytes, "cuLibraryGetManaged", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(library), ": ").as_bytes())?;
    crate::CudaDisplay::write(&library, "cuLibraryGetManaged", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(name), ": ").as_bytes())?;
    crate::CudaDisplay::write(&name, "cuLibraryGetManaged", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuLibraryGetUnifiedFunction(
    writer: &mut (impl std::io::Write + ?Sized),
    fptr: *mut *mut ::core::ffi::c_void,
    library: cuda_types::cuda::CUlibrary,
    symbol: *const ::core::ffi::c_char,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(fptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&fptr, "cuLibraryGetUnifiedFunction", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(library), ": ").as_bytes())?;
    crate::CudaDisplay::write(&library, "cuLibraryGetUnifiedFunction", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(symbol), ": ").as_bytes())?;
    crate::CudaDisplay::write(&symbol, "cuLibraryGetUnifiedFunction", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuKernelGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    pi: *mut ::core::ffi::c_int,
    attrib: cuda_types::cuda::CUfunction_attribute,
    kernel: cuda_types::cuda::CUkernel,
    dev: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pi), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pi, "cuKernelGetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attrib), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attrib, "cuKernelGetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(kernel), ": ").as_bytes())?;
    crate::CudaDisplay::write(&kernel, "cuKernelGetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuKernelGetAttribute", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuKernelSetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    attrib: cuda_types::cuda::CUfunction_attribute,
    val: ::core::ffi::c_int,
    kernel: cuda_types::cuda::CUkernel,
    dev: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(attrib), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attrib, "cuKernelSetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(val), ": ").as_bytes())?;
    crate::CudaDisplay::write(&val, "cuKernelSetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(kernel), ": ").as_bytes())?;
    crate::CudaDisplay::write(&kernel, "cuKernelSetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuKernelSetAttribute", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuKernelSetCacheConfig(
    writer: &mut (impl std::io::Write + ?Sized),
    kernel: cuda_types::cuda::CUkernel,
    config: cuda_types::cuda::CUfunc_cache,
    dev: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(kernel), ": ").as_bytes())?;
    crate::CudaDisplay::write(&kernel, "cuKernelSetCacheConfig", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(config), ": ").as_bytes())?;
    crate::CudaDisplay::write(&config, "cuKernelSetCacheConfig", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuKernelSetCacheConfig", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuKernelGetName(
    writer: &mut (impl std::io::Write + ?Sized),
    name: *mut *const ::core::ffi::c_char,
    hfunc: cuda_types::cuda::CUkernel,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(name), ": ").as_bytes())?;
    crate::CudaDisplay::write(&name, "cuKernelGetName", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hfunc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hfunc, "cuKernelGetName", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuKernelGetParamInfo(
    writer: &mut (impl std::io::Write + ?Sized),
    kernel: cuda_types::cuda::CUkernel,
    paramIndex: usize,
    paramOffset: *mut usize,
    paramSize: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(kernel), ": ").as_bytes())?;
    crate::CudaDisplay::write(&kernel, "cuKernelGetParamInfo", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(paramIndex), ": ").as_bytes())?;
    crate::CudaDisplay::write(&paramIndex, "cuKernelGetParamInfo", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(paramOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&paramOffset, "cuKernelGetParamInfo", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(paramSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&paramSize, "cuKernelGetParamInfo", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemGetInfo_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    free: *mut usize,
    total: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(free), ": ").as_bytes())?;
    crate::CudaDisplay::write(&free, "cuMemGetInfo_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(total), ": ").as_bytes())?;
    crate::CudaDisplay::write(&total, "cuMemGetInfo_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemAlloc_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    dptr: *mut cuda_types::cuda::CUdeviceptr,
    bytesize: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuMemAlloc_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bytesize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&bytesize, "cuMemAlloc_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemAllocPitch_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    dptr: *mut cuda_types::cuda::CUdeviceptr,
    pPitch: *mut usize,
    WidthInBytes: usize,
    Height: usize,
    ElementSizeBytes: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuMemAllocPitch_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pPitch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pPitch, "cuMemAllocPitch_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(WidthInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&WidthInBytes, "cuMemAllocPitch_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Height), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Height, "cuMemAllocPitch_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ElementSizeBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ElementSizeBytes, "cuMemAllocPitch_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemFree_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    dptr: cuda_types::cuda::CUdeviceptr,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuMemFree_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemGetAddressRange_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    pbase: *mut cuda_types::cuda::CUdeviceptr,
    psize: *mut usize,
    dptr: cuda_types::cuda::CUdeviceptr,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pbase), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pbase, "cuMemGetAddressRange_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(psize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&psize, "cuMemGetAddressRange_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuMemGetAddressRange_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemAllocHost_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    pp: *mut *mut ::core::ffi::c_void,
    bytesize: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pp), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pp, "cuMemAllocHost_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bytesize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&bytesize, "cuMemAllocHost_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemFreeHost(
    writer: &mut (impl std::io::Write + ?Sized),
    p: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(p), ": ").as_bytes())?;
    crate::CudaDisplay::write(&p, "cuMemFreeHost", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemHostAlloc(
    writer: &mut (impl std::io::Write + ?Sized),
    pp: *mut *mut ::core::ffi::c_void,
    bytesize: usize,
    Flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pp), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pp, "cuMemHostAlloc", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bytesize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&bytesize, "cuMemHostAlloc", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Flags, "cuMemHostAlloc", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemHostGetDevicePointer_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    pdptr: *mut cuda_types::cuda::CUdeviceptr,
    p: *mut ::core::ffi::c_void,
    Flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pdptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pdptr, "cuMemHostGetDevicePointer_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(p), ": ").as_bytes())?;
    crate::CudaDisplay::write(&p, "cuMemHostGetDevicePointer_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Flags, "cuMemHostGetDevicePointer_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemHostGetFlags(
    writer: &mut (impl std::io::Write + ?Sized),
    pFlags: *mut ::core::ffi::c_uint,
    p: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pFlags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pFlags, "cuMemHostGetFlags", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(p), ": ").as_bytes())?;
    crate::CudaDisplay::write(&p, "cuMemHostGetFlags", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemAllocManaged(
    writer: &mut (impl std::io::Write + ?Sized),
    dptr: *mut cuda_types::cuda::CUdeviceptr,
    bytesize: usize,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuMemAllocManaged", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bytesize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&bytesize, "cuMemAllocManaged", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuMemAllocManaged", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuDeviceRegisterAsyncNotification(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::cuda::CUdevice,
    callbackFunc: cuda_types::cuda::CUasyncCallback,
    userData: *mut ::core::ffi::c_void,
    callback: *mut cuda_types::cuda::CUasyncCallbackHandle,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "cuDeviceRegisterAsyncNotification",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(callbackFunc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &callbackFunc,
        "cuDeviceRegisterAsyncNotification",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(userData), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &userData,
        "cuDeviceRegisterAsyncNotification",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(callback), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &callback,
        "cuDeviceRegisterAsyncNotification",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuDeviceUnregisterAsyncNotification(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::cuda::CUdevice,
    callback: cuda_types::cuda::CUasyncCallbackHandle,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "cuDeviceUnregisterAsyncNotification",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(callback), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &callback,
        "cuDeviceUnregisterAsyncNotification",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuDeviceGetByPCIBusId(
    writer: &mut (impl std::io::Write + ?Sized),
    dev: *mut cuda_types::cuda::CUdevice,
    pciBusId: *const ::core::ffi::c_char,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuDeviceGetByPCIBusId", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pciBusId), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pciBusId, "cuDeviceGetByPCIBusId", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuDeviceGetPCIBusId(
    writer: &mut (impl std::io::Write + ?Sized),
    pciBusId: *mut ::core::ffi::c_char,
    len: ::core::ffi::c_int,
    dev: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pciBusId), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pciBusId, "cuDeviceGetPCIBusId", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(len), ": ").as_bytes())?;
    crate::CudaDisplay::write(&len, "cuDeviceGetPCIBusId", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuDeviceGetPCIBusId", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuIpcGetEventHandle(
    writer: &mut (impl std::io::Write + ?Sized),
    pHandle: *mut cuda_types::cuda::CUipcEventHandle,
    event: cuda_types::cuda::CUevent,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pHandle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pHandle, "cuIpcGetEventHandle", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(event), ": ").as_bytes())?;
    crate::CudaDisplay::write(&event, "cuIpcGetEventHandle", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuIpcOpenEventHandle(
    writer: &mut (impl std::io::Write + ?Sized),
    phEvent: *mut cuda_types::cuda::CUevent,
    handle: cuda_types::cuda::CUipcEventHandle,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phEvent), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phEvent, "cuIpcOpenEventHandle", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cuIpcOpenEventHandle", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuIpcGetMemHandle(
    writer: &mut (impl std::io::Write + ?Sized),
    pHandle: *mut cuda_types::cuda::CUipcMemHandle,
    dptr: cuda_types::cuda::CUdeviceptr,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pHandle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pHandle, "cuIpcGetMemHandle", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuIpcGetMemHandle", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuIpcOpenMemHandle_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    pdptr: *mut cuda_types::cuda::CUdeviceptr,
    handle: cuda_types::cuda::CUipcMemHandle,
    Flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pdptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pdptr, "cuIpcOpenMemHandle_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cuIpcOpenMemHandle_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Flags, "cuIpcOpenMemHandle_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuIpcCloseMemHandle(
    writer: &mut (impl std::io::Write + ?Sized),
    dptr: cuda_types::cuda::CUdeviceptr,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuIpcCloseMemHandle", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemHostRegister_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    p: *mut ::core::ffi::c_void,
    bytesize: usize,
    Flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(p), ": ").as_bytes())?;
    crate::CudaDisplay::write(&p, "cuMemHostRegister_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bytesize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&bytesize, "cuMemHostRegister_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Flags, "cuMemHostRegister_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemHostUnregister(
    writer: &mut (impl std::io::Write + ?Sized),
    p: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(p), ": ").as_bytes())?;
    crate::CudaDisplay::write(&p, "cuMemHostUnregister", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpy_ptds(
    writer: &mut (impl std::io::Write + ?Sized),
    dst: cuda_types::cuda::CUdeviceptr,
    src: cuda_types::cuda::CUdeviceptr,
    ByteCount: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dst), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dst, "cuMemcpy_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(src), ": ").as_bytes())?;
    crate::CudaDisplay::write(&src, "cuMemcpy_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpy_ptds", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyPeer_ptds(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    dstContext: cuda_types::cuda::CUcontext,
    srcDevice: cuda_types::cuda::CUdeviceptr,
    srcContext: cuda_types::cuda::CUcontext,
    ByteCount: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemcpyPeer_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstContext), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstContext, "cuMemcpyPeer_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcDevice, "cuMemcpyPeer_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcContext), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcContext, "cuMemcpyPeer_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyPeer_ptds", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyHtoD_v2_ptds(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    srcHost: *const ::core::ffi::c_void,
    ByteCount: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemcpyHtoD_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcHost), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcHost, "cuMemcpyHtoD_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyHtoD_v2_ptds", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyDtoH_v2_ptds(
    writer: &mut (impl std::io::Write + ?Sized),
    dstHost: *mut ::core::ffi::c_void,
    srcDevice: cuda_types::cuda::CUdeviceptr,
    ByteCount: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstHost), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstHost, "cuMemcpyDtoH_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcDevice, "cuMemcpyDtoH_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyDtoH_v2_ptds", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyDtoD_v2_ptds(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    srcDevice: cuda_types::cuda::CUdeviceptr,
    ByteCount: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemcpyDtoD_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcDevice, "cuMemcpyDtoD_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyDtoD_v2_ptds", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyDtoA_v2_ptds(
    writer: &mut (impl std::io::Write + ?Sized),
    dstArray: cuda_types::cuda::CUarray,
    dstOffset: usize,
    srcDevice: cuda_types::cuda::CUdeviceptr,
    ByteCount: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstArray, "cuMemcpyDtoA_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstOffset, "cuMemcpyDtoA_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcDevice, "cuMemcpyDtoA_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyDtoA_v2_ptds", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyAtoD_v2_ptds(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    srcArray: cuda_types::cuda::CUarray,
    srcOffset: usize,
    ByteCount: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemcpyAtoD_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcArray, "cuMemcpyAtoD_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcOffset, "cuMemcpyAtoD_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyAtoD_v2_ptds", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyHtoA_v2_ptds(
    writer: &mut (impl std::io::Write + ?Sized),
    dstArray: cuda_types::cuda::CUarray,
    dstOffset: usize,
    srcHost: *const ::core::ffi::c_void,
    ByteCount: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstArray, "cuMemcpyHtoA_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstOffset, "cuMemcpyHtoA_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcHost), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcHost, "cuMemcpyHtoA_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyHtoA_v2_ptds", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyAtoH_v2_ptds(
    writer: &mut (impl std::io::Write + ?Sized),
    dstHost: *mut ::core::ffi::c_void,
    srcArray: cuda_types::cuda::CUarray,
    srcOffset: usize,
    ByteCount: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstHost), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstHost, "cuMemcpyAtoH_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcArray, "cuMemcpyAtoH_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcOffset, "cuMemcpyAtoH_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyAtoH_v2_ptds", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyAtoA_v2_ptds(
    writer: &mut (impl std::io::Write + ?Sized),
    dstArray: cuda_types::cuda::CUarray,
    dstOffset: usize,
    srcArray: cuda_types::cuda::CUarray,
    srcOffset: usize,
    ByteCount: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstArray, "cuMemcpyAtoA_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstOffset, "cuMemcpyAtoA_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcArray, "cuMemcpyAtoA_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcOffset, "cuMemcpyAtoA_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyAtoA_v2_ptds", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpy2D_v2_ptds(
    writer: &mut (impl std::io::Write + ?Sized),
    pCopy: *const cuda_types::cuda::CUDA_MEMCPY2D,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pCopy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pCopy, "cuMemcpy2D_v2_ptds", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpy2DUnaligned_v2_ptds(
    writer: &mut (impl std::io::Write + ?Sized),
    pCopy: *const cuda_types::cuda::CUDA_MEMCPY2D,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pCopy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pCopy, "cuMemcpy2DUnaligned_v2_ptds", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpy3D_v2_ptds(
    writer: &mut (impl std::io::Write + ?Sized),
    pCopy: *const cuda_types::cuda::CUDA_MEMCPY3D,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pCopy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pCopy, "cuMemcpy3D_v2_ptds", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpy3DPeer_ptds(
    writer: &mut (impl std::io::Write + ?Sized),
    pCopy: *const cuda_types::cuda::CUDA_MEMCPY3D_PEER,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pCopy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pCopy, "cuMemcpy3DPeer_ptds", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyAsync_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    dst: cuda_types::cuda::CUdeviceptr,
    src: cuda_types::cuda::CUdeviceptr,
    ByteCount: usize,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dst), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dst, "cuMemcpyAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(src), ": ").as_bytes())?;
    crate::CudaDisplay::write(&src, "cuMemcpyAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpyAsync_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyPeerAsync_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    dstContext: cuda_types::cuda::CUcontext,
    srcDevice: cuda_types::cuda::CUdeviceptr,
    srcContext: cuda_types::cuda::CUcontext,
    ByteCount: usize,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemcpyPeerAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstContext), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstContext, "cuMemcpyPeerAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcDevice, "cuMemcpyPeerAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcContext), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcContext, "cuMemcpyPeerAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyPeerAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpyPeerAsync_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyHtoDAsync_v2_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    srcHost: *const ::core::ffi::c_void,
    ByteCount: usize,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemcpyHtoDAsync_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcHost), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcHost, "cuMemcpyHtoDAsync_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyHtoDAsync_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpyHtoDAsync_v2_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyDtoHAsync_v2_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    dstHost: *mut ::core::ffi::c_void,
    srcDevice: cuda_types::cuda::CUdeviceptr,
    ByteCount: usize,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstHost), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstHost, "cuMemcpyDtoHAsync_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcDevice, "cuMemcpyDtoHAsync_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyDtoHAsync_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpyDtoHAsync_v2_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyDtoDAsync_v2_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    srcDevice: cuda_types::cuda::CUdeviceptr,
    ByteCount: usize,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemcpyDtoDAsync_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcDevice, "cuMemcpyDtoDAsync_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyDtoDAsync_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpyDtoDAsync_v2_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyHtoAAsync_v2_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    dstArray: cuda_types::cuda::CUarray,
    dstOffset: usize,
    srcHost: *const ::core::ffi::c_void,
    ByteCount: usize,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstArray, "cuMemcpyHtoAAsync_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstOffset, "cuMemcpyHtoAAsync_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcHost), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcHost, "cuMemcpyHtoAAsync_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyHtoAAsync_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpyHtoAAsync_v2_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyAtoHAsync_v2_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    dstHost: *mut ::core::ffi::c_void,
    srcArray: cuda_types::cuda::CUarray,
    srcOffset: usize,
    ByteCount: usize,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstHost), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstHost, "cuMemcpyAtoHAsync_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcArray, "cuMemcpyAtoHAsync_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcOffset, "cuMemcpyAtoHAsync_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyAtoHAsync_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpyAtoHAsync_v2_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpy2DAsync_v2_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    pCopy: *const cuda_types::cuda::CUDA_MEMCPY2D,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pCopy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pCopy, "cuMemcpy2DAsync_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpy2DAsync_v2_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpy3DAsync_v2_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    pCopy: *const cuda_types::cuda::CUDA_MEMCPY3D,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pCopy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pCopy, "cuMemcpy3DAsync_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpy3DAsync_v2_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpy3DPeerAsync_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    pCopy: *const cuda_types::cuda::CUDA_MEMCPY3D_PEER,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pCopy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pCopy, "cuMemcpy3DPeerAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpy3DPeerAsync_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyBatchAsync_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    dsts: *mut cuda_types::cuda::CUdeviceptr,
    srcs: *mut cuda_types::cuda::CUdeviceptr,
    sizes: *mut usize,
    count: usize,
    attrs: *mut cuda_types::cuda::CUmemcpyAttributes,
    attrsIdxs: *mut usize,
    numAttrs: usize,
    failIdx: *mut usize,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dsts), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dsts, "cuMemcpyBatchAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcs), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcs, "cuMemcpyBatchAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&sizes, "cuMemcpyBatchAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "cuMemcpyBatchAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attrs), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attrs, "cuMemcpyBatchAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attrsIdxs), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attrsIdxs, "cuMemcpyBatchAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numAttrs), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numAttrs, "cuMemcpyBatchAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(failIdx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&failIdx, "cuMemcpyBatchAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpyBatchAsync_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpy3DBatchAsync_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    numOps: usize,
    opList: *mut cuda_types::cuda::CUDA_MEMCPY3D_BATCH_OP,
    failIdx: *mut usize,
    flags: ::core::ffi::c_ulonglong,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(numOps), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numOps, "cuMemcpy3DBatchAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(opList), ": ").as_bytes())?;
    crate::CudaDisplay::write(&opList, "cuMemcpy3DBatchAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(failIdx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&failIdx, "cuMemcpy3DBatchAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuMemcpy3DBatchAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpy3DBatchAsync_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemsetD8_v2_ptds(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    uc: ::core::ffi::c_uchar,
    N: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemsetD8_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(uc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&uc, "cuMemsetD8_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(N), ": ").as_bytes())?;
    crate::CudaDisplay::write(&N, "cuMemsetD8_v2_ptds", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemsetD16_v2_ptds(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    us: ::core::ffi::c_ushort,
    N: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemsetD16_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(us), ": ").as_bytes())?;
    crate::CudaDisplay::write(&us, "cuMemsetD16_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(N), ": ").as_bytes())?;
    crate::CudaDisplay::write(&N, "cuMemsetD16_v2_ptds", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemsetD32_v2_ptds(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    ui: ::core::ffi::c_uint,
    N: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemsetD32_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ui), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ui, "cuMemsetD32_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(N), ": ").as_bytes())?;
    crate::CudaDisplay::write(&N, "cuMemsetD32_v2_ptds", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemsetD2D8_v2_ptds(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    dstPitch: usize,
    uc: ::core::ffi::c_uchar,
    Width: usize,
    Height: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemsetD2D8_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstPitch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstPitch, "cuMemsetD2D8_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(uc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&uc, "cuMemsetD2D8_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Width), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Width, "cuMemsetD2D8_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Height), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Height, "cuMemsetD2D8_v2_ptds", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemsetD2D16_v2_ptds(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    dstPitch: usize,
    us: ::core::ffi::c_ushort,
    Width: usize,
    Height: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemsetD2D16_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstPitch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstPitch, "cuMemsetD2D16_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(us), ": ").as_bytes())?;
    crate::CudaDisplay::write(&us, "cuMemsetD2D16_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Width), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Width, "cuMemsetD2D16_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Height), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Height, "cuMemsetD2D16_v2_ptds", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemsetD2D32_v2_ptds(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    dstPitch: usize,
    ui: ::core::ffi::c_uint,
    Width: usize,
    Height: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemsetD2D32_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstPitch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstPitch, "cuMemsetD2D32_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ui), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ui, "cuMemsetD2D32_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Width), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Width, "cuMemsetD2D32_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Height), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Height, "cuMemsetD2D32_v2_ptds", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemsetD8Async_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    uc: ::core::ffi::c_uchar,
    N: usize,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemsetD8Async_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(uc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&uc, "cuMemsetD8Async_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(N), ": ").as_bytes())?;
    crate::CudaDisplay::write(&N, "cuMemsetD8Async_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemsetD8Async_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemsetD16Async_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    us: ::core::ffi::c_ushort,
    N: usize,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemsetD16Async_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(us), ": ").as_bytes())?;
    crate::CudaDisplay::write(&us, "cuMemsetD16Async_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(N), ": ").as_bytes())?;
    crate::CudaDisplay::write(&N, "cuMemsetD16Async_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemsetD16Async_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemsetD32Async_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    ui: ::core::ffi::c_uint,
    N: usize,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemsetD32Async_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ui), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ui, "cuMemsetD32Async_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(N), ": ").as_bytes())?;
    crate::CudaDisplay::write(&N, "cuMemsetD32Async_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemsetD32Async_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemsetD2D8Async_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    dstPitch: usize,
    uc: ::core::ffi::c_uchar,
    Width: usize,
    Height: usize,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemsetD2D8Async_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstPitch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstPitch, "cuMemsetD2D8Async_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(uc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&uc, "cuMemsetD2D8Async_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Width), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Width, "cuMemsetD2D8Async_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Height), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Height, "cuMemsetD2D8Async_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemsetD2D8Async_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemsetD2D16Async_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    dstPitch: usize,
    us: ::core::ffi::c_ushort,
    Width: usize,
    Height: usize,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemsetD2D16Async_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstPitch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstPitch, "cuMemsetD2D16Async_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(us), ": ").as_bytes())?;
    crate::CudaDisplay::write(&us, "cuMemsetD2D16Async_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Width), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Width, "cuMemsetD2D16Async_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Height), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Height, "cuMemsetD2D16Async_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemsetD2D16Async_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemsetD2D32Async_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    dstPitch: usize,
    ui: ::core::ffi::c_uint,
    Width: usize,
    Height: usize,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemsetD2D32Async_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstPitch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstPitch, "cuMemsetD2D32Async_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ui), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ui, "cuMemsetD2D32Async_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Width), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Width, "cuMemsetD2D32Async_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Height), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Height, "cuMemsetD2D32Async_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemsetD2D32Async_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuArrayCreate_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    pHandle: *mut cuda_types::cuda::CUarray,
    pAllocateArray: *const cuda_types::cuda::CUDA_ARRAY_DESCRIPTOR,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pHandle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pHandle, "cuArrayCreate_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pAllocateArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pAllocateArray, "cuArrayCreate_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuArrayGetDescriptor_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    pArrayDescriptor: *mut cuda_types::cuda::CUDA_ARRAY_DESCRIPTOR,
    hArray: cuda_types::cuda::CUarray,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pArrayDescriptor), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pArrayDescriptor,
        "cuArrayGetDescriptor_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hArray, "cuArrayGetDescriptor_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuArrayGetSparseProperties(
    writer: &mut (impl std::io::Write + ?Sized),
    sparseProperties: *mut cuda_types::cuda::CUDA_ARRAY_SPARSE_PROPERTIES,
    array: cuda_types::cuda::CUarray,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(sparseProperties), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sparseProperties,
        "cuArrayGetSparseProperties",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(array), ": ").as_bytes())?;
    crate::CudaDisplay::write(&array, "cuArrayGetSparseProperties", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMipmappedArrayGetSparseProperties(
    writer: &mut (impl std::io::Write + ?Sized),
    sparseProperties: *mut cuda_types::cuda::CUDA_ARRAY_SPARSE_PROPERTIES,
    mipmap: cuda_types::cuda::CUmipmappedArray,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(sparseProperties), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sparseProperties,
        "cuMipmappedArrayGetSparseProperties",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mipmap), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mipmap,
        "cuMipmappedArrayGetSparseProperties",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuArrayGetMemoryRequirements(
    writer: &mut (impl std::io::Write + ?Sized),
    memoryRequirements: *mut cuda_types::cuda::CUDA_ARRAY_MEMORY_REQUIREMENTS,
    array: cuda_types::cuda::CUarray,
    device: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(memoryRequirements), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &memoryRequirements,
        "cuArrayGetMemoryRequirements",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(array), ": ").as_bytes())?;
    crate::CudaDisplay::write(&array, "cuArrayGetMemoryRequirements", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "cuArrayGetMemoryRequirements", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMipmappedArrayGetMemoryRequirements(
    writer: &mut (impl std::io::Write + ?Sized),
    memoryRequirements: *mut cuda_types::cuda::CUDA_ARRAY_MEMORY_REQUIREMENTS,
    mipmap: cuda_types::cuda::CUmipmappedArray,
    device: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(memoryRequirements), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &memoryRequirements,
        "cuMipmappedArrayGetMemoryRequirements",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mipmap), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mipmap,
        "cuMipmappedArrayGetMemoryRequirements",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "cuMipmappedArrayGetMemoryRequirements",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuArrayGetPlane(
    writer: &mut (impl std::io::Write + ?Sized),
    pPlaneArray: *mut cuda_types::cuda::CUarray,
    hArray: cuda_types::cuda::CUarray,
    planeIdx: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pPlaneArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pPlaneArray, "cuArrayGetPlane", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hArray, "cuArrayGetPlane", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(planeIdx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&planeIdx, "cuArrayGetPlane", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuArrayDestroy(
    writer: &mut (impl std::io::Write + ?Sized),
    hArray: cuda_types::cuda::CUarray,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hArray, "cuArrayDestroy", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuArray3DCreate_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    pHandle: *mut cuda_types::cuda::CUarray,
    pAllocateArray: *const cuda_types::cuda::CUDA_ARRAY3D_DESCRIPTOR,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pHandle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pHandle, "cuArray3DCreate_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pAllocateArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pAllocateArray, "cuArray3DCreate_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuArray3DGetDescriptor_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    pArrayDescriptor: *mut cuda_types::cuda::CUDA_ARRAY3D_DESCRIPTOR,
    hArray: cuda_types::cuda::CUarray,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pArrayDescriptor), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pArrayDescriptor,
        "cuArray3DGetDescriptor_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hArray, "cuArray3DGetDescriptor_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMipmappedArrayCreate(
    writer: &mut (impl std::io::Write + ?Sized),
    pHandle: *mut cuda_types::cuda::CUmipmappedArray,
    pMipmappedArrayDesc: *const cuda_types::cuda::CUDA_ARRAY3D_DESCRIPTOR,
    numMipmapLevels: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pHandle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pHandle, "cuMipmappedArrayCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pMipmappedArrayDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pMipmappedArrayDesc,
        "cuMipmappedArrayCreate",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numMipmapLevels), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numMipmapLevels,
        "cuMipmappedArrayCreate",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuMipmappedArrayGetLevel(
    writer: &mut (impl std::io::Write + ?Sized),
    pLevelArray: *mut cuda_types::cuda::CUarray,
    hMipmappedArray: cuda_types::cuda::CUmipmappedArray,
    level: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pLevelArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pLevelArray,
        "cuMipmappedArrayGetLevel",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hMipmappedArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hMipmappedArray,
        "cuMipmappedArrayGetLevel",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(level), ": ").as_bytes())?;
    crate::CudaDisplay::write(&level, "cuMipmappedArrayGetLevel", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMipmappedArrayDestroy(
    writer: &mut (impl std::io::Write + ?Sized),
    hMipmappedArray: cuda_types::cuda::CUmipmappedArray,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hMipmappedArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hMipmappedArray,
        "cuMipmappedArrayDestroy",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuMemGetHandleForAddressRange(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: *mut ::core::ffi::c_void,
    dptr: cuda_types::cuda::CUdeviceptr,
    size: usize,
    handleType: cuda_types::cuda::CUmemRangeHandleType,
    flags: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cuMemGetHandleForAddressRange",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuMemGetHandleForAddressRange", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(&size, "cuMemGetHandleForAddressRange", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(handleType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handleType,
        "cuMemGetHandleForAddressRange",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuMemGetHandleForAddressRange", arg_idx, writer)?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::cuda::CUmemDecompressAlgorithm_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUmemDecompressAlgorithm_enum::CU_MEM_DECOMPRESS_UNSUPPORTED => {
                writer.write_all(stringify!(CU_MEM_DECOMPRESS_UNSUPPORTED).as_bytes())
            }
            &cuda_types::cuda::CUmemDecompressAlgorithm_enum::CU_MEM_DECOMPRESS_ALGORITHM_DEFLATE => {
                writer
                    .write_all(
                        stringify!(CU_MEM_DECOMPRESS_ALGORITHM_DEFLATE).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUmemDecompressAlgorithm_enum::CU_MEM_DECOMPRESS_ALGORITHM_SNAPPY => {
                writer
                    .write_all(stringify!(CU_MEM_DECOMPRESS_ALGORITHM_SNAPPY).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUmemDecompressParams_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(srcNumBytes), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcNumBytes, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstNumBytes), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstNumBytes, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstActBytes), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstActBytes, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(src), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.src, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dst), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dst, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(algo), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.algo, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(padding), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.padding, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
pub fn write_cuMemBatchDecompressAsync_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    paramsArray: *mut cuda_types::cuda::CUmemDecompressParams,
    count: usize,
    flags: ::core::ffi::c_uint,
    errorIndex: *mut usize,
    stream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(paramsArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &paramsArray,
        "cuMemBatchDecompressAsync_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &count,
        "cuMemBatchDecompressAsync_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &flags,
        "cuMemBatchDecompressAsync_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(errorIndex), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &errorIndex,
        "cuMemBatchDecompressAsync_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &stream,
        "cuMemBatchDecompressAsync_ptsz",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuMemAddressReserve(
    writer: &mut (impl std::io::Write + ?Sized),
    ptr: *mut cuda_types::cuda::CUdeviceptr,
    size: usize,
    alignment: usize,
    addr: cuda_types::cuda::CUdeviceptr,
    flags: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(ptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ptr, "cuMemAddressReserve", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(&size, "cuMemAddressReserve", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(alignment), ": ").as_bytes())?;
    crate::CudaDisplay::write(&alignment, "cuMemAddressReserve", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(addr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&addr, "cuMemAddressReserve", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuMemAddressReserve", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemAddressFree(
    writer: &mut (impl std::io::Write + ?Sized),
    ptr: cuda_types::cuda::CUdeviceptr,
    size: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(ptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ptr, "cuMemAddressFree", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(&size, "cuMemAddressFree", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemCreate(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: *mut cuda_types::cuda::CUmemGenericAllocationHandle,
    size: usize,
    prop: *const cuda_types::cuda::CUmemAllocationProp,
    flags: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cuMemCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(&size, "cuMemCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(prop), ": ").as_bytes())?;
    crate::CudaDisplay::write(&prop, "cuMemCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuMemCreate", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemRelease(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: cuda_types::cuda::CUmemGenericAllocationHandle,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cuMemRelease", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemMap(
    writer: &mut (impl std::io::Write + ?Sized),
    ptr: cuda_types::cuda::CUdeviceptr,
    size: usize,
    offset: usize,
    handle: cuda_types::cuda::CUmemGenericAllocationHandle,
    flags: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(ptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ptr, "cuMemMap", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(&size, "cuMemMap", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(offset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&offset, "cuMemMap", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cuMemMap", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuMemMap", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemMapArrayAsync_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    mapInfoList: *mut cuda_types::cuda::CUarrayMapInfo,
    count: ::core::ffi::c_uint,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(mapInfoList), ": ").as_bytes())?;
    writer.write_all(b"[")?;
    for i in 0..count {
        if i != 0 {
            writer.write_all(b", ")?;
        }
        crate::CudaDisplay::write(
            unsafe { &*mapInfoList.add(i as usize) },
            "cuMemMapArrayAsync_ptsz",
            arg_idx,
            writer,
        )?;
    }
    writer.write_all(b"]")?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "cuMemMapArrayAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemMapArrayAsync_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemUnmap(
    writer: &mut (impl std::io::Write + ?Sized),
    ptr: cuda_types::cuda::CUdeviceptr,
    size: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(ptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ptr, "cuMemUnmap", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(&size, "cuMemUnmap", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemSetAccess(
    writer: &mut (impl std::io::Write + ?Sized),
    ptr: cuda_types::cuda::CUdeviceptr,
    size: usize,
    desc: *const cuda_types::cuda::CUmemAccessDesc,
    count: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(ptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ptr, "cuMemSetAccess", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(&size, "cuMemSetAccess", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(desc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&desc, "cuMemSetAccess", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "cuMemSetAccess", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemGetAccess(
    writer: &mut (impl std::io::Write + ?Sized),
    flags: *mut ::core::ffi::c_ulonglong,
    location: *const cuda_types::cuda::CUmemLocation,
    ptr: cuda_types::cuda::CUdeviceptr,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuMemGetAccess", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(location), ": ").as_bytes())?;
    crate::CudaDisplay::write(&location, "cuMemGetAccess", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ptr, "cuMemGetAccess", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemExportToShareableHandle(
    writer: &mut (impl std::io::Write + ?Sized),
    shareableHandle: *mut ::core::ffi::c_void,
    handle: cuda_types::cuda::CUmemGenericAllocationHandle,
    handleType: cuda_types::cuda::CUmemAllocationHandleType,
    flags: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(shareableHandle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &shareableHandle,
        "cuMemExportToShareableHandle",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cuMemExportToShareableHandle", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(handleType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handleType,
        "cuMemExportToShareableHandle",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuMemExportToShareableHandle", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemImportFromShareableHandle(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: *mut cuda_types::cuda::CUmemGenericAllocationHandle,
    osHandle: *mut ::core::ffi::c_void,
    shHandleType: cuda_types::cuda::CUmemAllocationHandleType,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cuMemImportFromShareableHandle",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(osHandle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &osHandle,
        "cuMemImportFromShareableHandle",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(shHandleType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &shHandleType,
        "cuMemImportFromShareableHandle",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuMemGetAllocationGranularity(
    writer: &mut (impl std::io::Write + ?Sized),
    granularity: *mut usize,
    prop: *const cuda_types::cuda::CUmemAllocationProp,
    option: cuda_types::cuda::CUmemAllocationGranularity_flags,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(granularity), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &granularity,
        "cuMemGetAllocationGranularity",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(prop), ": ").as_bytes())?;
    crate::CudaDisplay::write(&prop, "cuMemGetAllocationGranularity", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(option), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &option,
        "cuMemGetAllocationGranularity",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuMemGetAllocationPropertiesFromHandle(
    writer: &mut (impl std::io::Write + ?Sized),
    prop: *mut cuda_types::cuda::CUmemAllocationProp,
    handle: cuda_types::cuda::CUmemGenericAllocationHandle,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(prop), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &prop,
        "cuMemGetAllocationPropertiesFromHandle",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cuMemGetAllocationPropertiesFromHandle",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuMemRetainAllocationHandle(
    writer: &mut (impl std::io::Write + ?Sized),
    handle: *mut cuda_types::cuda::CUmemGenericAllocationHandle,
    addr: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cuMemRetainAllocationHandle", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(addr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&addr, "cuMemRetainAllocationHandle", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemFreeAsync_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    dptr: cuda_types::cuda::CUdeviceptr,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuMemFreeAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemFreeAsync_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemAllocAsync_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    dptr: *mut cuda_types::cuda::CUdeviceptr,
    bytesize: usize,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuMemAllocAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bytesize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&bytesize, "cuMemAllocAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemAllocAsync_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemPoolTrimTo(
    writer: &mut (impl std::io::Write + ?Sized),
    pool: cuda_types::cuda::CUmemoryPool,
    minBytesToKeep: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pool), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pool, "cuMemPoolTrimTo", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(minBytesToKeep), ": ").as_bytes())?;
    crate::CudaDisplay::write(&minBytesToKeep, "cuMemPoolTrimTo", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemPoolSetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    pool: cuda_types::cuda::CUmemoryPool,
    attr: cuda_types::cuda::CUmemPool_attribute,
    value: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pool), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pool, "cuMemPoolSetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attr, "cuMemPoolSetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cuMemPoolSetAttribute", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemPoolGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    pool: cuda_types::cuda::CUmemoryPool,
    attr: cuda_types::cuda::CUmemPool_attribute,
    value: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pool), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pool, "cuMemPoolGetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attr, "cuMemPoolGetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cuMemPoolGetAttribute", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemPoolSetAccess(
    writer: &mut (impl std::io::Write + ?Sized),
    pool: cuda_types::cuda::CUmemoryPool,
    map: *const cuda_types::cuda::CUmemAccessDesc,
    count: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pool), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pool, "cuMemPoolSetAccess", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(map), ": ").as_bytes())?;
    crate::CudaDisplay::write(&map, "cuMemPoolSetAccess", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "cuMemPoolSetAccess", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemPoolGetAccess(
    writer: &mut (impl std::io::Write + ?Sized),
    flags: *mut cuda_types::cuda::CUmemAccess_flags,
    memPool: cuda_types::cuda::CUmemoryPool,
    location: *mut cuda_types::cuda::CUmemLocation,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuMemPoolGetAccess", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(memPool), ": ").as_bytes())?;
    crate::CudaDisplay::write(&memPool, "cuMemPoolGetAccess", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(location), ": ").as_bytes())?;
    crate::CudaDisplay::write(&location, "cuMemPoolGetAccess", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemPoolCreate(
    writer: &mut (impl std::io::Write + ?Sized),
    pool: *mut cuda_types::cuda::CUmemoryPool,
    poolProps: *const cuda_types::cuda::CUmemPoolProps,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pool), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pool, "cuMemPoolCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(poolProps), ": ").as_bytes())?;
    crate::CudaDisplay::write(&poolProps, "cuMemPoolCreate", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemPoolDestroy(
    writer: &mut (impl std::io::Write + ?Sized),
    pool: cuda_types::cuda::CUmemoryPool,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pool), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pool, "cuMemPoolDestroy", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemAllocFromPoolAsync_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    dptr: *mut cuda_types::cuda::CUdeviceptr,
    bytesize: usize,
    pool: cuda_types::cuda::CUmemoryPool,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuMemAllocFromPoolAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bytesize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &bytesize,
        "cuMemAllocFromPoolAsync_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pool), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pool, "cuMemAllocFromPoolAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hStream,
        "cuMemAllocFromPoolAsync_ptsz",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuMemPoolExportToShareableHandle(
    writer: &mut (impl std::io::Write + ?Sized),
    handle_out: *mut ::core::ffi::c_void,
    pool: cuda_types::cuda::CUmemoryPool,
    handleType: cuda_types::cuda::CUmemAllocationHandleType,
    flags: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(handle_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle_out,
        "cuMemPoolExportToShareableHandle",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pool), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pool,
        "cuMemPoolExportToShareableHandle",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(handleType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handleType,
        "cuMemPoolExportToShareableHandle",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &flags,
        "cuMemPoolExportToShareableHandle",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuMemPoolImportFromShareableHandle(
    writer: &mut (impl std::io::Write + ?Sized),
    pool_out: *mut cuda_types::cuda::CUmemoryPool,
    handle: *mut ::core::ffi::c_void,
    handleType: cuda_types::cuda::CUmemAllocationHandleType,
    flags: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pool_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pool_out,
        "cuMemPoolImportFromShareableHandle",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handle,
        "cuMemPoolImportFromShareableHandle",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(handleType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &handleType,
        "cuMemPoolImportFromShareableHandle",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &flags,
        "cuMemPoolImportFromShareableHandle",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuMemPoolExportPointer(
    writer: &mut (impl std::io::Write + ?Sized),
    shareData_out: *mut cuda_types::cuda::CUmemPoolPtrExportData,
    ptr: cuda_types::cuda::CUdeviceptr,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(shareData_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &shareData_out,
        "cuMemPoolExportPointer",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ptr, "cuMemPoolExportPointer", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemPoolImportPointer(
    writer: &mut (impl std::io::Write + ?Sized),
    ptr_out: *mut cuda_types::cuda::CUdeviceptr,
    pool: cuda_types::cuda::CUmemoryPool,
    shareData: *mut cuda_types::cuda::CUmemPoolPtrExportData,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(ptr_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ptr_out, "cuMemPoolImportPointer", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pool), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pool, "cuMemPoolImportPointer", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(shareData), ": ").as_bytes())?;
    crate::CudaDisplay::write(&shareData, "cuMemPoolImportPointer", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMulticastCreate(
    writer: &mut (impl std::io::Write + ?Sized),
    mcHandle: *mut cuda_types::cuda::CUmemGenericAllocationHandle,
    prop: *const cuda_types::cuda::CUmulticastObjectProp,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(mcHandle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mcHandle, "cuMulticastCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(prop), ": ").as_bytes())?;
    crate::CudaDisplay::write(&prop, "cuMulticastCreate", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMulticastAddDevice(
    writer: &mut (impl std::io::Write + ?Sized),
    mcHandle: cuda_types::cuda::CUmemGenericAllocationHandle,
    dev: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(mcHandle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mcHandle, "cuMulticastAddDevice", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuMulticastAddDevice", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMulticastBindMem(
    writer: &mut (impl std::io::Write + ?Sized),
    mcHandle: cuda_types::cuda::CUmemGenericAllocationHandle,
    mcOffset: usize,
    memHandle: cuda_types::cuda::CUmemGenericAllocationHandle,
    memOffset: usize,
    size: usize,
    flags: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(mcHandle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mcHandle, "cuMulticastBindMem", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mcOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mcOffset, "cuMulticastBindMem", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(memHandle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&memHandle, "cuMulticastBindMem", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(memOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&memOffset, "cuMulticastBindMem", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(&size, "cuMulticastBindMem", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuMulticastBindMem", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMulticastBindAddr(
    writer: &mut (impl std::io::Write + ?Sized),
    mcHandle: cuda_types::cuda::CUmemGenericAllocationHandle,
    mcOffset: usize,
    memptr: cuda_types::cuda::CUdeviceptr,
    size: usize,
    flags: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(mcHandle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mcHandle, "cuMulticastBindAddr", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mcOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mcOffset, "cuMulticastBindAddr", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(memptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&memptr, "cuMulticastBindAddr", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(&size, "cuMulticastBindAddr", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuMulticastBindAddr", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMulticastUnbind(
    writer: &mut (impl std::io::Write + ?Sized),
    mcHandle: cuda_types::cuda::CUmemGenericAllocationHandle,
    dev: cuda_types::cuda::CUdevice,
    mcOffset: usize,
    size: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(mcHandle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mcHandle, "cuMulticastUnbind", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuMulticastUnbind", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mcOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mcOffset, "cuMulticastUnbind", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(&size, "cuMulticastUnbind", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMulticastGetGranularity(
    writer: &mut (impl std::io::Write + ?Sized),
    granularity: *mut usize,
    prop: *const cuda_types::cuda::CUmulticastObjectProp,
    option: cuda_types::cuda::CUmulticastGranularity_flags,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(granularity), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &granularity,
        "cuMulticastGetGranularity",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(prop), ": ").as_bytes())?;
    crate::CudaDisplay::write(&prop, "cuMulticastGetGranularity", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(option), ": ").as_bytes())?;
    crate::CudaDisplay::write(&option, "cuMulticastGetGranularity", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuPointerGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    data: *mut ::core::ffi::c_void,
    attribute: cuda_types::cuda::CUpointer_attribute,
    ptr: cuda_types::cuda::CUdeviceptr,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(data), ": ").as_bytes())?;
    crate::CudaDisplay::write(&data, "cuPointerGetAttribute", arg_idx, writer)?;
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
pub fn write_cuMemPrefetchAsync_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    devPtr: cuda_types::cuda::CUdeviceptr,
    count: usize,
    dstDevice: cuda_types::cuda::CUdevice,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(devPtr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&devPtr, "cuMemPrefetchAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "cuMemPrefetchAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemPrefetchAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemPrefetchAsync_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemPrefetchAsync_v2_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    devPtr: cuda_types::cuda::CUdeviceptr,
    count: usize,
    location: cuda_types::cuda::CUmemLocation,
    flags: ::core::ffi::c_uint,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(devPtr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&devPtr, "cuMemPrefetchAsync_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "cuMemPrefetchAsync_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(location), ": ").as_bytes())?;
    crate::CudaDisplay::write(&location, "cuMemPrefetchAsync_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuMemPrefetchAsync_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemPrefetchAsync_v2_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemAdvise(
    writer: &mut (impl std::io::Write + ?Sized),
    devPtr: cuda_types::cuda::CUdeviceptr,
    count: usize,
    advice: cuda_types::cuda::CUmem_advise,
    device: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(devPtr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&devPtr, "cuMemAdvise", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "cuMemAdvise", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(advice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&advice, "cuMemAdvise", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "cuMemAdvise", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemAdvise_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    devPtr: cuda_types::cuda::CUdeviceptr,
    count: usize,
    advice: cuda_types::cuda::CUmem_advise,
    location: cuda_types::cuda::CUmemLocation,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(devPtr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&devPtr, "cuMemAdvise_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "cuMemAdvise_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(advice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&advice, "cuMemAdvise_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(location), ": ").as_bytes())?;
    crate::CudaDisplay::write(&location, "cuMemAdvise_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemRangeGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    data: *mut ::core::ffi::c_void,
    dataSize: usize,
    attribute: cuda_types::cuda::CUmem_range_attribute,
    devPtr: cuda_types::cuda::CUdeviceptr,
    count: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(data), ": ").as_bytes())?;
    crate::CudaDisplay::write(&data, "cuMemRangeGetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dataSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dataSize, "cuMemRangeGetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attribute), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attribute, "cuMemRangeGetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(devPtr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&devPtr, "cuMemRangeGetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "cuMemRangeGetAttribute", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemRangeGetAttributes(
    writer: &mut (impl std::io::Write + ?Sized),
    data: *mut *mut ::core::ffi::c_void,
    dataSizes: *mut usize,
    attributes: *mut cuda_types::cuda::CUmem_range_attribute,
    numAttributes: usize,
    devPtr: cuda_types::cuda::CUdeviceptr,
    count: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(data), ": ").as_bytes())?;
    crate::CudaDisplay::write(&data, "cuMemRangeGetAttributes", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dataSizes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dataSizes, "cuMemRangeGetAttributes", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attributes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attributes, "cuMemRangeGetAttributes", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numAttributes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numAttributes,
        "cuMemRangeGetAttributes",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(devPtr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&devPtr, "cuMemRangeGetAttributes", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "cuMemRangeGetAttributes", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuPointerSetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    value: *const ::core::ffi::c_void,
    attribute: cuda_types::cuda::CUpointer_attribute,
    ptr: cuda_types::cuda::CUdeviceptr,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cuPointerSetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attribute), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attribute, "cuPointerSetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ptr, "cuPointerSetAttribute", arg_idx, writer)?;
    writer.write_all(b")")
}
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
    crate::CudaDisplay::write(&attributes, "cuPointerGetAttributes", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(data), ": ").as_bytes())?;
    crate::CudaDisplay::write(&data, "cuPointerGetAttributes", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ptr, "cuPointerGetAttributes", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamCreate(
    writer: &mut (impl std::io::Write + ?Sized),
    phStream: *mut cuda_types::cuda::CUstream,
    Flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phStream, "cuStreamCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Flags, "cuStreamCreate", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamCreateWithPriority(
    writer: &mut (impl std::io::Write + ?Sized),
    phStream: *mut cuda_types::cuda::CUstream,
    flags: ::core::ffi::c_uint,
    priority: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phStream, "cuStreamCreateWithPriority", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuStreamCreateWithPriority", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(priority), ": ").as_bytes())?;
    crate::CudaDisplay::write(&priority, "cuStreamCreateWithPriority", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamGetPriority_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    priority: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamGetPriority_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(priority), ": ").as_bytes())?;
    crate::CudaDisplay::write(&priority, "cuStreamGetPriority_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamGetDevice_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    device: *mut cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamGetDevice_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "cuStreamGetDevice_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamGetFlags_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    flags: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamGetFlags_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuStreamGetFlags_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamGetId_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    streamId: *mut ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamGetId_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(streamId), ": ").as_bytes())?;
    crate::CudaDisplay::write(&streamId, "cuStreamGetId_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamGetCtx_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    pctx: *mut cuda_types::cuda::CUcontext,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamGetCtx_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pctx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pctx, "cuStreamGetCtx_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamGetCtx_v2_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    pCtx: *mut cuda_types::cuda::CUcontext,
    pGreenCtx: *mut cuda_types::cuda::CUgreenCtx,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamGetCtx_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pCtx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pCtx, "cuStreamGetCtx_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pGreenCtx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pGreenCtx, "cuStreamGetCtx_v2_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamWaitEvent_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    hEvent: cuda_types::cuda::CUevent,
    Flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamWaitEvent_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hEvent), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hEvent, "cuStreamWaitEvent_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Flags, "cuStreamWaitEvent_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamAddCallback_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    callback: cuda_types::cuda::CUstreamCallback,
    userData: *mut ::core::ffi::c_void,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamAddCallback_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(callback), ": ").as_bytes())?;
    crate::CudaDisplay::write(&callback, "cuStreamAddCallback_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(userData), ": ").as_bytes())?;
    crate::CudaDisplay::write(&userData, "cuStreamAddCallback_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuStreamAddCallback_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamBeginCapture_v2_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    mode: cuda_types::cuda::CUstreamCaptureMode,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hStream,
        "cuStreamBeginCapture_v2_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mode, "cuStreamBeginCapture_v2_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamBeginCaptureToGraph_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    hGraph: cuda_types::cuda::CUgraph,
    dependencies: *const cuda_types::cuda::CUgraphNode,
    dependencyData: *const cuda_types::cuda::CUgraphEdgeData,
    numDependencies: usize,
    mode: cuda_types::cuda::CUstreamCaptureMode,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hStream,
        "cuStreamBeginCaptureToGraph_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hGraph,
        "cuStreamBeginCaptureToGraph_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dependencies,
        "cuStreamBeginCaptureToGraph_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencyData), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dependencyData,
        "cuStreamBeginCaptureToGraph_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDependencies,
        "cuStreamBeginCaptureToGraph_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mode,
        "cuStreamBeginCaptureToGraph_ptsz",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuThreadExchangeStreamCaptureMode(
    writer: &mut (impl std::io::Write + ?Sized),
    mode: *mut cuda_types::cuda::CUstreamCaptureMode,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mode,
        "cuThreadExchangeStreamCaptureMode",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuStreamEndCapture_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    phGraph: *mut cuda_types::cuda::CUgraph,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamEndCapture_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(phGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phGraph, "cuStreamEndCapture_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamIsCapturing_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    captureStatus: *mut cuda_types::cuda::CUstreamCaptureStatus,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamIsCapturing_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(captureStatus), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &captureStatus,
        "cuStreamIsCapturing_ptsz",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuStreamGetCaptureInfo_v2_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    captureStatus_out: *mut cuda_types::cuda::CUstreamCaptureStatus,
    id_out: *mut cuda_types::cuda::cuuint64_t,
    graph_out: *mut cuda_types::cuda::CUgraph,
    dependencies_out: *mut *const cuda_types::cuda::CUgraphNode,
    numDependencies_out: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hStream,
        "cuStreamGetCaptureInfo_v2_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(captureStatus_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &captureStatus_out,
        "cuStreamGetCaptureInfo_v2_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(id_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &id_out,
        "cuStreamGetCaptureInfo_v2_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(graph_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &graph_out,
        "cuStreamGetCaptureInfo_v2_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencies_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dependencies_out,
        "cuStreamGetCaptureInfo_v2_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDependencies_out,
        "cuStreamGetCaptureInfo_v2_ptsz",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuStreamGetCaptureInfo_v3_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    captureStatus_out: *mut cuda_types::cuda::CUstreamCaptureStatus,
    id_out: *mut cuda_types::cuda::cuuint64_t,
    graph_out: *mut cuda_types::cuda::CUgraph,
    dependencies_out: *mut *const cuda_types::cuda::CUgraphNode,
    edgeData_out: *mut *const cuda_types::cuda::CUgraphEdgeData,
    numDependencies_out: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hStream,
        "cuStreamGetCaptureInfo_v3_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(captureStatus_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &captureStatus_out,
        "cuStreamGetCaptureInfo_v3_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(id_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &id_out,
        "cuStreamGetCaptureInfo_v3_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(graph_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &graph_out,
        "cuStreamGetCaptureInfo_v3_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencies_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dependencies_out,
        "cuStreamGetCaptureInfo_v3_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(edgeData_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &edgeData_out,
        "cuStreamGetCaptureInfo_v3_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDependencies_out,
        "cuStreamGetCaptureInfo_v3_ptsz",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuStreamUpdateCaptureDependencies_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    dependencies: *mut cuda_types::cuda::CUgraphNode,
    numDependencies: usize,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hStream,
        "cuStreamUpdateCaptureDependencies_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dependencies,
        "cuStreamUpdateCaptureDependencies_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDependencies,
        "cuStreamUpdateCaptureDependencies_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &flags,
        "cuStreamUpdateCaptureDependencies_ptsz",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuStreamUpdateCaptureDependencies_v2_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    dependencies: *mut cuda_types::cuda::CUgraphNode,
    dependencyData: *const cuda_types::cuda::CUgraphEdgeData,
    numDependencies: usize,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hStream,
        "cuStreamUpdateCaptureDependencies_v2_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dependencies,
        "cuStreamUpdateCaptureDependencies_v2_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencyData), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dependencyData,
        "cuStreamUpdateCaptureDependencies_v2_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDependencies,
        "cuStreamUpdateCaptureDependencies_v2_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &flags,
        "cuStreamUpdateCaptureDependencies_v2_ptsz",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuStreamAttachMemAsync_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    dptr: cuda_types::cuda::CUdeviceptr,
    length: usize,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamAttachMemAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuStreamAttachMemAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(length), ": ").as_bytes())?;
    crate::CudaDisplay::write(&length, "cuStreamAttachMemAsync_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuStreamAttachMemAsync_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamQuery_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamQuery_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamSynchronize_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamSynchronize_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamDestroy_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamDestroy_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamCopyAttributes_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    dst: cuda_types::cuda::CUstream,
    src: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dst), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dst, "cuStreamCopyAttributes_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(src), ": ").as_bytes())?;
    crate::CudaDisplay::write(&src, "cuStreamCopyAttributes_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuEventCreate(
    writer: &mut (impl std::io::Write + ?Sized),
    phEvent: *mut cuda_types::cuda::CUevent,
    Flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phEvent), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phEvent, "cuEventCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Flags, "cuEventCreate", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuEventRecord_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    hEvent: cuda_types::cuda::CUevent,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hEvent), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hEvent, "cuEventRecord_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuEventRecord_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuEventRecordWithFlags_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    hEvent: cuda_types::cuda::CUevent,
    hStream: cuda_types::cuda::CUstream,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hEvent), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hEvent, "cuEventRecordWithFlags_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuEventRecordWithFlags_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuEventRecordWithFlags_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuEventQuery(
    writer: &mut (impl std::io::Write + ?Sized),
    hEvent: cuda_types::cuda::CUevent,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hEvent), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hEvent, "cuEventQuery", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuEventSynchronize(
    writer: &mut (impl std::io::Write + ?Sized),
    hEvent: cuda_types::cuda::CUevent,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hEvent), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hEvent, "cuEventSynchronize", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuEventDestroy_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    hEvent: cuda_types::cuda::CUevent,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hEvent), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hEvent, "cuEventDestroy_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuEventElapsedTime(
    writer: &mut (impl std::io::Write + ?Sized),
    pMilliseconds: *mut f32,
    hStart: cuda_types::cuda::CUevent,
    hEnd: cuda_types::cuda::CUevent,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pMilliseconds), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pMilliseconds, "cuEventElapsedTime", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStart), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStart, "cuEventElapsedTime", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hEnd), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hEnd, "cuEventElapsedTime", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuEventElapsedTime_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    pMilliseconds: *mut f32,
    hStart: cuda_types::cuda::CUevent,
    hEnd: cuda_types::cuda::CUevent,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pMilliseconds), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pMilliseconds, "cuEventElapsedTime_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStart), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStart, "cuEventElapsedTime_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hEnd), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hEnd, "cuEventElapsedTime_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuImportExternalMemory(
    writer: &mut (impl std::io::Write + ?Sized),
    extMem_out: *mut cuda_types::cuda::CUexternalMemory,
    memHandleDesc: *const cuda_types::cuda::CUDA_EXTERNAL_MEMORY_HANDLE_DESC,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(extMem_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(&extMem_out, "cuImportExternalMemory", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(memHandleDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &memHandleDesc,
        "cuImportExternalMemory",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuExternalMemoryGetMappedBuffer(
    writer: &mut (impl std::io::Write + ?Sized),
    devPtr: *mut cuda_types::cuda::CUdeviceptr,
    extMem: cuda_types::cuda::CUexternalMemory,
    bufferDesc: *const cuda_types::cuda::CUDA_EXTERNAL_MEMORY_BUFFER_DESC,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(devPtr), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &devPtr,
        "cuExternalMemoryGetMappedBuffer",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(extMem), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &extMem,
        "cuExternalMemoryGetMappedBuffer",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bufferDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &bufferDesc,
        "cuExternalMemoryGetMappedBuffer",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuExternalMemoryGetMappedMipmappedArray(
    writer: &mut (impl std::io::Write + ?Sized),
    mipmap: *mut cuda_types::cuda::CUmipmappedArray,
    extMem: cuda_types::cuda::CUexternalMemory,
    mipmapDesc: *const cuda_types::cuda::CUDA_EXTERNAL_MEMORY_MIPMAPPED_ARRAY_DESC,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(mipmap), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mipmap,
        "cuExternalMemoryGetMappedMipmappedArray",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(extMem), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &extMem,
        "cuExternalMemoryGetMappedMipmappedArray",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mipmapDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mipmapDesc,
        "cuExternalMemoryGetMappedMipmappedArray",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuDestroyExternalMemory(
    writer: &mut (impl std::io::Write + ?Sized),
    extMem: cuda_types::cuda::CUexternalMemory,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(extMem), ": ").as_bytes())?;
    crate::CudaDisplay::write(&extMem, "cuDestroyExternalMemory", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuImportExternalSemaphore(
    writer: &mut (impl std::io::Write + ?Sized),
    extSem_out: *mut cuda_types::cuda::CUexternalSemaphore,
    semHandleDesc: *const cuda_types::cuda::CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(extSem_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &extSem_out,
        "cuImportExternalSemaphore",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(semHandleDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &semHandleDesc,
        "cuImportExternalSemaphore",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuSignalExternalSemaphoresAsync_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    extSemArray: *const cuda_types::cuda::CUexternalSemaphore,
    paramsArray: *const cuda_types::cuda::CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS,
    numExtSems: ::core::ffi::c_uint,
    stream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(extSemArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &extSemArray,
        "cuSignalExternalSemaphoresAsync_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(paramsArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &paramsArray,
        "cuSignalExternalSemaphoresAsync_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numExtSems), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numExtSems,
        "cuSignalExternalSemaphoresAsync_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &stream,
        "cuSignalExternalSemaphoresAsync_ptsz",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuWaitExternalSemaphoresAsync_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    extSemArray: *const cuda_types::cuda::CUexternalSemaphore,
    paramsArray: *const cuda_types::cuda::CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS,
    numExtSems: ::core::ffi::c_uint,
    stream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(extSemArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &extSemArray,
        "cuWaitExternalSemaphoresAsync_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(paramsArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &paramsArray,
        "cuWaitExternalSemaphoresAsync_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numExtSems), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numExtSems,
        "cuWaitExternalSemaphoresAsync_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &stream,
        "cuWaitExternalSemaphoresAsync_ptsz",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuDestroyExternalSemaphore(
    writer: &mut (impl std::io::Write + ?Sized),
    extSem: cuda_types::cuda::CUexternalSemaphore,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(extSem), ": ").as_bytes())?;
    crate::CudaDisplay::write(&extSem, "cuDestroyExternalSemaphore", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamWaitValue32_v2_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    stream: cuda_types::cuda::CUstream,
    addr: cuda_types::cuda::CUdeviceptr,
    value: cuda_types::cuda::cuuint32_t,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&stream, "cuStreamWaitValue32_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(addr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&addr, "cuStreamWaitValue32_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cuStreamWaitValue32_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuStreamWaitValue32_v2_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamWaitValue64_v2_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    stream: cuda_types::cuda::CUstream,
    addr: cuda_types::cuda::CUdeviceptr,
    value: cuda_types::cuda::cuuint64_t,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&stream, "cuStreamWaitValue64_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(addr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&addr, "cuStreamWaitValue64_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cuStreamWaitValue64_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuStreamWaitValue64_v2_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamWriteValue32_v2_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    stream: cuda_types::cuda::CUstream,
    addr: cuda_types::cuda::CUdeviceptr,
    value: cuda_types::cuda::cuuint32_t,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&stream, "cuStreamWriteValue32_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(addr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&addr, "cuStreamWriteValue32_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cuStreamWriteValue32_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuStreamWriteValue32_v2_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamWriteValue64_v2_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    stream: cuda_types::cuda::CUstream,
    addr: cuda_types::cuda::CUdeviceptr,
    value: cuda_types::cuda::cuuint64_t,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&stream, "cuStreamWriteValue64_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(addr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&addr, "cuStreamWriteValue64_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cuStreamWriteValue64_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuStreamWriteValue64_v2_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamBatchMemOp_v2_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    stream: cuda_types::cuda::CUstream,
    count: ::core::ffi::c_uint,
    paramArray: *mut cuda_types::cuda::CUstreamBatchMemOpParams,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&stream, "cuStreamBatchMemOp_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "cuStreamBatchMemOp_v2_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(paramArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &paramArray,
        "cuStreamBatchMemOp_v2_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuStreamBatchMemOp_v2_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuFuncGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    pi: *mut ::core::ffi::c_int,
    attrib: cuda_types::cuda::CUfunction_attribute,
    hfunc: cuda_types::cuda::CUfunction,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pi), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pi, "cuFuncGetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attrib), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attrib, "cuFuncGetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hfunc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hfunc, "cuFuncGetAttribute", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuFuncSetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    hfunc: cuda_types::cuda::CUfunction,
    attrib: cuda_types::cuda::CUfunction_attribute,
    value: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hfunc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hfunc, "cuFuncSetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attrib), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attrib, "cuFuncSetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cuFuncSetAttribute", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuFuncSetCacheConfig(
    writer: &mut (impl std::io::Write + ?Sized),
    hfunc: cuda_types::cuda::CUfunction,
    config: cuda_types::cuda::CUfunc_cache,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hfunc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hfunc, "cuFuncSetCacheConfig", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(config), ": ").as_bytes())?;
    crate::CudaDisplay::write(&config, "cuFuncSetCacheConfig", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuFuncGetModule(
    writer: &mut (impl std::io::Write + ?Sized),
    hmod: *mut cuda_types::cuda::CUmodule,
    hfunc: cuda_types::cuda::CUfunction,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hmod), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hmod, "cuFuncGetModule", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hfunc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hfunc, "cuFuncGetModule", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuFuncGetName(
    writer: &mut (impl std::io::Write + ?Sized),
    name: *mut *const ::core::ffi::c_char,
    hfunc: cuda_types::cuda::CUfunction,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(name), ": ").as_bytes())?;
    crate::CudaDisplay::write(&name, "cuFuncGetName", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hfunc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hfunc, "cuFuncGetName", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuFuncGetParamInfo(
    writer: &mut (impl std::io::Write + ?Sized),
    func: cuda_types::cuda::CUfunction,
    paramIndex: usize,
    paramOffset: *mut usize,
    paramSize: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(func), ": ").as_bytes())?;
    crate::CudaDisplay::write(&func, "cuFuncGetParamInfo", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(paramIndex), ": ").as_bytes())?;
    crate::CudaDisplay::write(&paramIndex, "cuFuncGetParamInfo", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(paramOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&paramOffset, "cuFuncGetParamInfo", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(paramSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&paramSize, "cuFuncGetParamInfo", arg_idx, writer)?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::cuda::CUfunctionLoadingState_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUfunctionLoadingState_enum::CU_FUNCTION_LOADING_STATE_UNLOADED => {
                writer
                    .write_all(stringify!(CU_FUNCTION_LOADING_STATE_UNLOADED).as_bytes())
            }
            &cuda_types::cuda::CUfunctionLoadingState_enum::CU_FUNCTION_LOADING_STATE_LOADED => {
                writer.write_all(stringify!(CU_FUNCTION_LOADING_STATE_LOADED).as_bytes())
            }
            &cuda_types::cuda::CUfunctionLoadingState_enum::CU_FUNCTION_LOADING_STATE_MAX => {
                writer.write_all(stringify!(CU_FUNCTION_LOADING_STATE_MAX).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cuFuncIsLoaded(
    writer: &mut (impl std::io::Write + ?Sized),
    state: *mut cuda_types::cuda::CUfunctionLoadingState,
    function: cuda_types::cuda::CUfunction,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(state), ": ").as_bytes())?;
    crate::CudaDisplay::write(&state, "cuFuncIsLoaded", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(function), ": ").as_bytes())?;
    crate::CudaDisplay::write(&function, "cuFuncIsLoaded", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuFuncLoad(
    writer: &mut (impl std::io::Write + ?Sized),
    function: cuda_types::cuda::CUfunction,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(function), ": ").as_bytes())?;
    crate::CudaDisplay::write(&function, "cuFuncLoad", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuLaunchKernel_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    f: cuda_types::cuda::CUfunction,
    gridDimX: ::core::ffi::c_uint,
    gridDimY: ::core::ffi::c_uint,
    gridDimZ: ::core::ffi::c_uint,
    blockDimX: ::core::ffi::c_uint,
    blockDimY: ::core::ffi::c_uint,
    blockDimZ: ::core::ffi::c_uint,
    sharedMemBytes: ::core::ffi::c_uint,
    hStream: cuda_types::cuda::CUstream,
    kernelParams: *mut *mut ::core::ffi::c_void,
    extra: *mut *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(f), ": ").as_bytes())?;
    crate::CudaDisplay::write(&f, "cuLaunchKernel_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gridDimX), ": ").as_bytes())?;
    crate::CudaDisplay::write(&gridDimX, "cuLaunchKernel_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gridDimY), ": ").as_bytes())?;
    crate::CudaDisplay::write(&gridDimY, "cuLaunchKernel_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gridDimZ), ": ").as_bytes())?;
    crate::CudaDisplay::write(&gridDimZ, "cuLaunchKernel_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(blockDimX), ": ").as_bytes())?;
    crate::CudaDisplay::write(&blockDimX, "cuLaunchKernel_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(blockDimY), ": ").as_bytes())?;
    crate::CudaDisplay::write(&blockDimY, "cuLaunchKernel_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(blockDimZ), ": ").as_bytes())?;
    crate::CudaDisplay::write(&blockDimZ, "cuLaunchKernel_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sharedMemBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&sharedMemBytes, "cuLaunchKernel_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuLaunchKernel_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(kernelParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(&kernelParams, "cuLaunchKernel_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(extra), ": ").as_bytes())?;
    crate::CudaDisplay::write(&extra, "cuLaunchKernel_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuLaunchKernelEx_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    config: *const cuda_types::cuda::CUlaunchConfig,
    f: cuda_types::cuda::CUfunction,
    kernelParams: *mut *mut ::core::ffi::c_void,
    extra: *mut *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(config), ": ").as_bytes())?;
    crate::CudaDisplay::write(&config, "cuLaunchKernelEx_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(f), ": ").as_bytes())?;
    crate::CudaDisplay::write(&f, "cuLaunchKernelEx_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(kernelParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(&kernelParams, "cuLaunchKernelEx_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(extra), ": ").as_bytes())?;
    crate::CudaDisplay::write(&extra, "cuLaunchKernelEx_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuLaunchCooperativeKernel_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    f: cuda_types::cuda::CUfunction,
    gridDimX: ::core::ffi::c_uint,
    gridDimY: ::core::ffi::c_uint,
    gridDimZ: ::core::ffi::c_uint,
    blockDimX: ::core::ffi::c_uint,
    blockDimY: ::core::ffi::c_uint,
    blockDimZ: ::core::ffi::c_uint,
    sharedMemBytes: ::core::ffi::c_uint,
    hStream: cuda_types::cuda::CUstream,
    kernelParams: *mut *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(f), ": ").as_bytes())?;
    crate::CudaDisplay::write(&f, "cuLaunchCooperativeKernel_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gridDimX), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gridDimX,
        "cuLaunchCooperativeKernel_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gridDimY), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gridDimY,
        "cuLaunchCooperativeKernel_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gridDimZ), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gridDimZ,
        "cuLaunchCooperativeKernel_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(blockDimX), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &blockDimX,
        "cuLaunchCooperativeKernel_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(blockDimY), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &blockDimY,
        "cuLaunchCooperativeKernel_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(blockDimZ), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &blockDimZ,
        "cuLaunchCooperativeKernel_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sharedMemBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sharedMemBytes,
        "cuLaunchCooperativeKernel_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hStream,
        "cuLaunchCooperativeKernel_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(kernelParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &kernelParams,
        "cuLaunchCooperativeKernel_ptsz",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuLaunchCooperativeKernelMultiDevice(
    writer: &mut (impl std::io::Write + ?Sized),
    launchParamsList: *mut cuda_types::cuda::CUDA_LAUNCH_PARAMS,
    numDevices: ::core::ffi::c_uint,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(launchParamsList), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &launchParamsList,
        "cuLaunchCooperativeKernelMultiDevice",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDevices), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDevices,
        "cuLaunchCooperativeKernelMultiDevice",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &flags,
        "cuLaunchCooperativeKernelMultiDevice",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuLaunchHostFunc_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    fn_: cuda_types::cuda::CUhostFn,
    userData: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuLaunchHostFunc_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(fn_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&fn_, "cuLaunchHostFunc_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(userData), ": ").as_bytes())?;
    crate::CudaDisplay::write(&userData, "cuLaunchHostFunc_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuFuncSetBlockShape(
    writer: &mut (impl std::io::Write + ?Sized),
    hfunc: cuda_types::cuda::CUfunction,
    x: ::core::ffi::c_int,
    y: ::core::ffi::c_int,
    z: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hfunc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hfunc, "cuFuncSetBlockShape", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(x), ": ").as_bytes())?;
    crate::CudaDisplay::write(&x, "cuFuncSetBlockShape", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(y), ": ").as_bytes())?;
    crate::CudaDisplay::write(&y, "cuFuncSetBlockShape", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(z), ": ").as_bytes())?;
    crate::CudaDisplay::write(&z, "cuFuncSetBlockShape", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuFuncSetSharedSize(
    writer: &mut (impl std::io::Write + ?Sized),
    hfunc: cuda_types::cuda::CUfunction,
    bytes: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hfunc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hfunc, "cuFuncSetSharedSize", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&bytes, "cuFuncSetSharedSize", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuParamSetSize(
    writer: &mut (impl std::io::Write + ?Sized),
    hfunc: cuda_types::cuda::CUfunction,
    numbytes: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hfunc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hfunc, "cuParamSetSize", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numbytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numbytes, "cuParamSetSize", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuParamSeti(
    writer: &mut (impl std::io::Write + ?Sized),
    hfunc: cuda_types::cuda::CUfunction,
    offset: ::core::ffi::c_int,
    value: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hfunc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hfunc, "cuParamSeti", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(offset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&offset, "cuParamSeti", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cuParamSeti", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuParamSetf(
    writer: &mut (impl std::io::Write + ?Sized),
    hfunc: cuda_types::cuda::CUfunction,
    offset: ::core::ffi::c_int,
    value: f32,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hfunc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hfunc, "cuParamSetf", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(offset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&offset, "cuParamSetf", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cuParamSetf", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuParamSetv(
    writer: &mut (impl std::io::Write + ?Sized),
    hfunc: cuda_types::cuda::CUfunction,
    offset: ::core::ffi::c_int,
    ptr: *mut ::core::ffi::c_void,
    numbytes: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hfunc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hfunc, "cuParamSetv", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(offset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&offset, "cuParamSetv", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ptr, "cuParamSetv", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numbytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numbytes, "cuParamSetv", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuLaunch(
    writer: &mut (impl std::io::Write + ?Sized),
    f: cuda_types::cuda::CUfunction,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(f), ": ").as_bytes())?;
    crate::CudaDisplay::write(&f, "cuLaunch", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuLaunchGrid(
    writer: &mut (impl std::io::Write + ?Sized),
    f: cuda_types::cuda::CUfunction,
    grid_width: ::core::ffi::c_int,
    grid_height: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(f), ": ").as_bytes())?;
    crate::CudaDisplay::write(&f, "cuLaunchGrid", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(grid_width), ": ").as_bytes())?;
    crate::CudaDisplay::write(&grid_width, "cuLaunchGrid", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(grid_height), ": ").as_bytes())?;
    crate::CudaDisplay::write(&grid_height, "cuLaunchGrid", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuLaunchGridAsync(
    writer: &mut (impl std::io::Write + ?Sized),
    f: cuda_types::cuda::CUfunction,
    grid_width: ::core::ffi::c_int,
    grid_height: ::core::ffi::c_int,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(f), ": ").as_bytes())?;
    crate::CudaDisplay::write(&f, "cuLaunchGridAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(grid_width), ": ").as_bytes())?;
    crate::CudaDisplay::write(&grid_width, "cuLaunchGridAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(grid_height), ": ").as_bytes())?;
    crate::CudaDisplay::write(&grid_height, "cuLaunchGridAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuLaunchGridAsync", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuParamSetTexRef(
    writer: &mut (impl std::io::Write + ?Sized),
    hfunc: cuda_types::cuda::CUfunction,
    texunit: ::core::ffi::c_int,
    hTexRef: cuda_types::cuda::CUtexref,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hfunc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hfunc, "cuParamSetTexRef", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(texunit), ": ").as_bytes())?;
    crate::CudaDisplay::write(&texunit, "cuParamSetTexRef", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuParamSetTexRef", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuFuncSetSharedMemConfig(
    writer: &mut (impl std::io::Write + ?Sized),
    hfunc: cuda_types::cuda::CUfunction,
    config: cuda_types::cuda::CUsharedconfig,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hfunc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hfunc, "cuFuncSetSharedMemConfig", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(config), ": ").as_bytes())?;
    crate::CudaDisplay::write(&config, "cuFuncSetSharedMemConfig", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphCreate(
    writer: &mut (impl std::io::Write + ?Sized),
    phGraph: *mut cuda_types::cuda::CUgraph,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phGraph, "cuGraphCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuGraphCreate", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphAddKernelNode_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    phGraphNode: *mut cuda_types::cuda::CUgraphNode,
    hGraph: cuda_types::cuda::CUgraph,
    dependencies: *const cuda_types::cuda::CUgraphNode,
    numDependencies: usize,
    nodeParams: *const cuda_types::cuda::CUDA_KERNEL_NODE_PARAMS,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phGraphNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phGraphNode, "cuGraphAddKernelNode_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphAddKernelNode_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dependencies,
        "cuGraphAddKernelNode_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDependencies,
        "cuGraphAddKernelNode_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nodeParams, "cuGraphAddKernelNode_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphKernelNodeGetParams_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::cuda::CUgraphNode,
    nodeParams: *mut cuda_types::cuda::CUDA_KERNEL_NODE_PARAMS,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hNode, "cuGraphKernelNodeGetParams_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nodeParams,
        "cuGraphKernelNodeGetParams_v2",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphKernelNodeSetParams_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::cuda::CUgraphNode,
    nodeParams: *const cuda_types::cuda::CUDA_KERNEL_NODE_PARAMS,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hNode, "cuGraphKernelNodeSetParams_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nodeParams,
        "cuGraphKernelNodeSetParams_v2",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphAddMemcpyNode(
    writer: &mut (impl std::io::Write + ?Sized),
    phGraphNode: *mut cuda_types::cuda::CUgraphNode,
    hGraph: cuda_types::cuda::CUgraph,
    dependencies: *const cuda_types::cuda::CUgraphNode,
    numDependencies: usize,
    copyParams: *const cuda_types::cuda::CUDA_MEMCPY3D,
    ctx: cuda_types::cuda::CUcontext,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phGraphNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phGraphNode, "cuGraphAddMemcpyNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphAddMemcpyNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dependencies, "cuGraphAddMemcpyNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDependencies,
        "cuGraphAddMemcpyNode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(copyParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(&copyParams, "cuGraphAddMemcpyNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ctx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ctx, "cuGraphAddMemcpyNode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphMemcpyNodeGetParams(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::cuda::CUgraphNode,
    nodeParams: *mut cuda_types::cuda::CUDA_MEMCPY3D,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hNode, "cuGraphMemcpyNodeGetParams", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nodeParams,
        "cuGraphMemcpyNodeGetParams",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphMemcpyNodeSetParams(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::cuda::CUgraphNode,
    nodeParams: *const cuda_types::cuda::CUDA_MEMCPY3D,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hNode, "cuGraphMemcpyNodeSetParams", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nodeParams,
        "cuGraphMemcpyNodeSetParams",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphAddMemsetNode(
    writer: &mut (impl std::io::Write + ?Sized),
    phGraphNode: *mut cuda_types::cuda::CUgraphNode,
    hGraph: cuda_types::cuda::CUgraph,
    dependencies: *const cuda_types::cuda::CUgraphNode,
    numDependencies: usize,
    memsetParams: *const cuda_types::cuda::CUDA_MEMSET_NODE_PARAMS,
    ctx: cuda_types::cuda::CUcontext,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phGraphNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phGraphNode, "cuGraphAddMemsetNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphAddMemsetNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dependencies, "cuGraphAddMemsetNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDependencies,
        "cuGraphAddMemsetNode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(memsetParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(&memsetParams, "cuGraphAddMemsetNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ctx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ctx, "cuGraphAddMemsetNode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphMemsetNodeGetParams(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::cuda::CUgraphNode,
    nodeParams: *mut cuda_types::cuda::CUDA_MEMSET_NODE_PARAMS,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hNode, "cuGraphMemsetNodeGetParams", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nodeParams,
        "cuGraphMemsetNodeGetParams",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphMemsetNodeSetParams(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::cuda::CUgraphNode,
    nodeParams: *const cuda_types::cuda::CUDA_MEMSET_NODE_PARAMS,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hNode, "cuGraphMemsetNodeSetParams", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nodeParams,
        "cuGraphMemsetNodeSetParams",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphAddHostNode(
    writer: &mut (impl std::io::Write + ?Sized),
    phGraphNode: *mut cuda_types::cuda::CUgraphNode,
    hGraph: cuda_types::cuda::CUgraph,
    dependencies: *const cuda_types::cuda::CUgraphNode,
    numDependencies: usize,
    nodeParams: *const cuda_types::cuda::CUDA_HOST_NODE_PARAMS,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phGraphNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phGraphNode, "cuGraphAddHostNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphAddHostNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dependencies, "cuGraphAddHostNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numDependencies, "cuGraphAddHostNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nodeParams, "cuGraphAddHostNode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphHostNodeGetParams(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::cuda::CUgraphNode,
    nodeParams: *mut cuda_types::cuda::CUDA_HOST_NODE_PARAMS,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hNode, "cuGraphHostNodeGetParams", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nodeParams, "cuGraphHostNodeGetParams", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphHostNodeSetParams(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::cuda::CUgraphNode,
    nodeParams: *const cuda_types::cuda::CUDA_HOST_NODE_PARAMS,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hNode, "cuGraphHostNodeSetParams", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nodeParams, "cuGraphHostNodeSetParams", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphAddChildGraphNode(
    writer: &mut (impl std::io::Write + ?Sized),
    phGraphNode: *mut cuda_types::cuda::CUgraphNode,
    hGraph: cuda_types::cuda::CUgraph,
    dependencies: *const cuda_types::cuda::CUgraphNode,
    numDependencies: usize,
    childGraph: cuda_types::cuda::CUgraph,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phGraphNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &phGraphNode,
        "cuGraphAddChildGraphNode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphAddChildGraphNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dependencies,
        "cuGraphAddChildGraphNode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDependencies,
        "cuGraphAddChildGraphNode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(childGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&childGraph, "cuGraphAddChildGraphNode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphChildGraphNodeGetGraph(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::cuda::CUgraphNode,
    phGraph: *mut cuda_types::cuda::CUgraph,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hNode, "cuGraphChildGraphNodeGetGraph", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(phGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &phGraph,
        "cuGraphChildGraphNodeGetGraph",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphAddEmptyNode(
    writer: &mut (impl std::io::Write + ?Sized),
    phGraphNode: *mut cuda_types::cuda::CUgraphNode,
    hGraph: cuda_types::cuda::CUgraph,
    dependencies: *const cuda_types::cuda::CUgraphNode,
    numDependencies: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phGraphNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phGraphNode, "cuGraphAddEmptyNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphAddEmptyNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dependencies, "cuGraphAddEmptyNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numDependencies, "cuGraphAddEmptyNode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphAddEventRecordNode(
    writer: &mut (impl std::io::Write + ?Sized),
    phGraphNode: *mut cuda_types::cuda::CUgraphNode,
    hGraph: cuda_types::cuda::CUgraph,
    dependencies: *const cuda_types::cuda::CUgraphNode,
    numDependencies: usize,
    event: cuda_types::cuda::CUevent,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phGraphNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &phGraphNode,
        "cuGraphAddEventRecordNode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphAddEventRecordNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dependencies,
        "cuGraphAddEventRecordNode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDependencies,
        "cuGraphAddEventRecordNode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(event), ": ").as_bytes())?;
    crate::CudaDisplay::write(&event, "cuGraphAddEventRecordNode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphEventRecordNodeGetEvent(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::cuda::CUgraphNode,
    event_out: *mut cuda_types::cuda::CUevent,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hNode,
        "cuGraphEventRecordNodeGetEvent",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(event_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &event_out,
        "cuGraphEventRecordNodeGetEvent",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphEventRecordNodeSetEvent(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::cuda::CUgraphNode,
    event: cuda_types::cuda::CUevent,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hNode,
        "cuGraphEventRecordNodeSetEvent",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(event), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &event,
        "cuGraphEventRecordNodeSetEvent",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphAddEventWaitNode(
    writer: &mut (impl std::io::Write + ?Sized),
    phGraphNode: *mut cuda_types::cuda::CUgraphNode,
    hGraph: cuda_types::cuda::CUgraph,
    dependencies: *const cuda_types::cuda::CUgraphNode,
    numDependencies: usize,
    event: cuda_types::cuda::CUevent,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phGraphNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phGraphNode, "cuGraphAddEventWaitNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphAddEventWaitNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dependencies,
        "cuGraphAddEventWaitNode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDependencies,
        "cuGraphAddEventWaitNode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(event), ": ").as_bytes())?;
    crate::CudaDisplay::write(&event, "cuGraphAddEventWaitNode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphEventWaitNodeGetEvent(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::cuda::CUgraphNode,
    event_out: *mut cuda_types::cuda::CUevent,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hNode, "cuGraphEventWaitNodeGetEvent", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(event_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &event_out,
        "cuGraphEventWaitNodeGetEvent",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphEventWaitNodeSetEvent(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::cuda::CUgraphNode,
    event: cuda_types::cuda::CUevent,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hNode, "cuGraphEventWaitNodeSetEvent", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(event), ": ").as_bytes())?;
    crate::CudaDisplay::write(&event, "cuGraphEventWaitNodeSetEvent", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphAddExternalSemaphoresSignalNode(
    writer: &mut (impl std::io::Write + ?Sized),
    phGraphNode: *mut cuda_types::cuda::CUgraphNode,
    hGraph: cuda_types::cuda::CUgraph,
    dependencies: *const cuda_types::cuda::CUgraphNode,
    numDependencies: usize,
    nodeParams: *const cuda_types::cuda::CUDA_EXT_SEM_SIGNAL_NODE_PARAMS,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phGraphNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &phGraphNode,
        "cuGraphAddExternalSemaphoresSignalNode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hGraph,
        "cuGraphAddExternalSemaphoresSignalNode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dependencies,
        "cuGraphAddExternalSemaphoresSignalNode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDependencies,
        "cuGraphAddExternalSemaphoresSignalNode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nodeParams,
        "cuGraphAddExternalSemaphoresSignalNode",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphExternalSemaphoresSignalNodeGetParams(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::cuda::CUgraphNode,
    params_out: *mut cuda_types::cuda::CUDA_EXT_SEM_SIGNAL_NODE_PARAMS,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hNode,
        "cuGraphExternalSemaphoresSignalNodeGetParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(params_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &params_out,
        "cuGraphExternalSemaphoresSignalNodeGetParams",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphExternalSemaphoresSignalNodeSetParams(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::cuda::CUgraphNode,
    nodeParams: *const cuda_types::cuda::CUDA_EXT_SEM_SIGNAL_NODE_PARAMS,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hNode,
        "cuGraphExternalSemaphoresSignalNodeSetParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nodeParams,
        "cuGraphExternalSemaphoresSignalNodeSetParams",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphAddExternalSemaphoresWaitNode(
    writer: &mut (impl std::io::Write + ?Sized),
    phGraphNode: *mut cuda_types::cuda::CUgraphNode,
    hGraph: cuda_types::cuda::CUgraph,
    dependencies: *const cuda_types::cuda::CUgraphNode,
    numDependencies: usize,
    nodeParams: *const cuda_types::cuda::CUDA_EXT_SEM_WAIT_NODE_PARAMS,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phGraphNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &phGraphNode,
        "cuGraphAddExternalSemaphoresWaitNode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hGraph,
        "cuGraphAddExternalSemaphoresWaitNode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dependencies,
        "cuGraphAddExternalSemaphoresWaitNode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDependencies,
        "cuGraphAddExternalSemaphoresWaitNode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nodeParams,
        "cuGraphAddExternalSemaphoresWaitNode",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphExternalSemaphoresWaitNodeGetParams(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::cuda::CUgraphNode,
    params_out: *mut cuda_types::cuda::CUDA_EXT_SEM_WAIT_NODE_PARAMS,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hNode,
        "cuGraphExternalSemaphoresWaitNodeGetParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(params_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &params_out,
        "cuGraphExternalSemaphoresWaitNodeGetParams",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphExternalSemaphoresWaitNodeSetParams(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::cuda::CUgraphNode,
    nodeParams: *const cuda_types::cuda::CUDA_EXT_SEM_WAIT_NODE_PARAMS,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hNode,
        "cuGraphExternalSemaphoresWaitNodeSetParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nodeParams,
        "cuGraphExternalSemaphoresWaitNodeSetParams",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphAddBatchMemOpNode(
    writer: &mut (impl std::io::Write + ?Sized),
    phGraphNode: *mut cuda_types::cuda::CUgraphNode,
    hGraph: cuda_types::cuda::CUgraph,
    dependencies: *const cuda_types::cuda::CUgraphNode,
    numDependencies: usize,
    nodeParams: *const cuda_types::cuda::CUDA_BATCH_MEM_OP_NODE_PARAMS,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phGraphNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &phGraphNode,
        "cuGraphAddBatchMemOpNode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphAddBatchMemOpNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dependencies,
        "cuGraphAddBatchMemOpNode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDependencies,
        "cuGraphAddBatchMemOpNode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nodeParams, "cuGraphAddBatchMemOpNode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphBatchMemOpNodeGetParams(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::cuda::CUgraphNode,
    nodeParams_out: *mut cuda_types::cuda::CUDA_BATCH_MEM_OP_NODE_PARAMS,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hNode,
        "cuGraphBatchMemOpNodeGetParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nodeParams_out,
        "cuGraphBatchMemOpNodeGetParams",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphBatchMemOpNodeSetParams(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::cuda::CUgraphNode,
    nodeParams: *const cuda_types::cuda::CUDA_BATCH_MEM_OP_NODE_PARAMS,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hNode,
        "cuGraphBatchMemOpNodeSetParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nodeParams,
        "cuGraphBatchMemOpNodeSetParams",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphExecBatchMemOpNodeSetParams(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraphExec: cuda_types::cuda::CUgraphExec,
    hNode: cuda_types::cuda::CUgraphNode,
    nodeParams: *const cuda_types::cuda::CUDA_BATCH_MEM_OP_NODE_PARAMS,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraphExec), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hGraphExec,
        "cuGraphExecBatchMemOpNodeSetParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hNode,
        "cuGraphExecBatchMemOpNodeSetParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nodeParams,
        "cuGraphExecBatchMemOpNodeSetParams",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphAddMemAllocNode(
    writer: &mut (impl std::io::Write + ?Sized),
    phGraphNode: *mut cuda_types::cuda::CUgraphNode,
    hGraph: cuda_types::cuda::CUgraph,
    dependencies: *const cuda_types::cuda::CUgraphNode,
    numDependencies: usize,
    nodeParams: *mut cuda_types::cuda::CUDA_MEM_ALLOC_NODE_PARAMS,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phGraphNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phGraphNode, "cuGraphAddMemAllocNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphAddMemAllocNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dependencies, "cuGraphAddMemAllocNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDependencies,
        "cuGraphAddMemAllocNode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nodeParams, "cuGraphAddMemAllocNode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphMemAllocNodeGetParams(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::cuda::CUgraphNode,
    params_out: *mut cuda_types::cuda::CUDA_MEM_ALLOC_NODE_PARAMS,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hNode, "cuGraphMemAllocNodeGetParams", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(params_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &params_out,
        "cuGraphMemAllocNodeGetParams",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphAddMemFreeNode(
    writer: &mut (impl std::io::Write + ?Sized),
    phGraphNode: *mut cuda_types::cuda::CUgraphNode,
    hGraph: cuda_types::cuda::CUgraph,
    dependencies: *const cuda_types::cuda::CUgraphNode,
    numDependencies: usize,
    dptr: cuda_types::cuda::CUdeviceptr,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phGraphNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phGraphNode, "cuGraphAddMemFreeNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphAddMemFreeNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dependencies, "cuGraphAddMemFreeNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDependencies,
        "cuGraphAddMemFreeNode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuGraphAddMemFreeNode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphMemFreeNodeGetParams(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::cuda::CUgraphNode,
    dptr_out: *mut cuda_types::cuda::CUdeviceptr,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hNode, "cuGraphMemFreeNodeGetParams", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dptr_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dptr_out,
        "cuGraphMemFreeNodeGetParams",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuDeviceGraphMemTrim(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "cuDeviceGraphMemTrim", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuDeviceGetGraphMemAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::cuda::CUdevice,
    attr: cuda_types::cuda::CUgraphMem_attribute,
    value: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "cuDeviceGetGraphMemAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attr, "cuDeviceGetGraphMemAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cuDeviceGetGraphMemAttribute", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuDeviceSetGraphMemAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::cuda::CUdevice,
    attr: cuda_types::cuda::CUgraphMem_attribute,
    value: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "cuDeviceSetGraphMemAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attr, "cuDeviceSetGraphMemAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cuDeviceSetGraphMemAttribute", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphClone(
    writer: &mut (impl std::io::Write + ?Sized),
    phGraphClone: *mut cuda_types::cuda::CUgraph,
    originalGraph: cuda_types::cuda::CUgraph,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phGraphClone), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phGraphClone, "cuGraphClone", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(originalGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&originalGraph, "cuGraphClone", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphNodeFindInClone(
    writer: &mut (impl std::io::Write + ?Sized),
    phNode: *mut cuda_types::cuda::CUgraphNode,
    hOriginalNode: cuda_types::cuda::CUgraphNode,
    hClonedGraph: cuda_types::cuda::CUgraph,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phNode, "cuGraphNodeFindInClone", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hOriginalNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hOriginalNode,
        "cuGraphNodeFindInClone",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hClonedGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hClonedGraph, "cuGraphNodeFindInClone", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphNodeGetType(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::cuda::CUgraphNode,
    type_: *mut cuda_types::cuda::CUgraphNodeType,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hNode, "cuGraphNodeGetType", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cuGraphNodeGetType", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphGetNodes(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraph: cuda_types::cuda::CUgraph,
    nodes: *mut cuda_types::cuda::CUgraphNode,
    numNodes: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphGetNodes", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nodes, "cuGraphGetNodes", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numNodes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numNodes, "cuGraphGetNodes", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphGetRootNodes(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraph: cuda_types::cuda::CUgraph,
    rootNodes: *mut cuda_types::cuda::CUgraphNode,
    numRootNodes: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphGetRootNodes", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rootNodes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&rootNodes, "cuGraphGetRootNodes", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numRootNodes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numRootNodes, "cuGraphGetRootNodes", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphGetEdges(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraph: cuda_types::cuda::CUgraph,
    from: *mut cuda_types::cuda::CUgraphNode,
    to: *mut cuda_types::cuda::CUgraphNode,
    numEdges: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphGetEdges", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(from), ": ").as_bytes())?;
    crate::CudaDisplay::write(&from, "cuGraphGetEdges", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(to), ": ").as_bytes())?;
    crate::CudaDisplay::write(&to, "cuGraphGetEdges", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numEdges), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numEdges, "cuGraphGetEdges", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphGetEdges_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraph: cuda_types::cuda::CUgraph,
    from: *mut cuda_types::cuda::CUgraphNode,
    to: *mut cuda_types::cuda::CUgraphNode,
    edgeData: *mut cuda_types::cuda::CUgraphEdgeData,
    numEdges: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphGetEdges_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(from), ": ").as_bytes())?;
    crate::CudaDisplay::write(&from, "cuGraphGetEdges_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(to), ": ").as_bytes())?;
    crate::CudaDisplay::write(&to, "cuGraphGetEdges_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(edgeData), ": ").as_bytes())?;
    crate::CudaDisplay::write(&edgeData, "cuGraphGetEdges_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numEdges), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numEdges, "cuGraphGetEdges_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphNodeGetDependencies(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::cuda::CUgraphNode,
    dependencies: *mut cuda_types::cuda::CUgraphNode,
    numDependencies: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hNode, "cuGraphNodeGetDependencies", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dependencies,
        "cuGraphNodeGetDependencies",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDependencies,
        "cuGraphNodeGetDependencies",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphNodeGetDependencies_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::cuda::CUgraphNode,
    dependencies: *mut cuda_types::cuda::CUgraphNode,
    edgeData: *mut cuda_types::cuda::CUgraphEdgeData,
    numDependencies: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hNode, "cuGraphNodeGetDependencies_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dependencies,
        "cuGraphNodeGetDependencies_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(edgeData), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &edgeData,
        "cuGraphNodeGetDependencies_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDependencies,
        "cuGraphNodeGetDependencies_v2",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphNodeGetDependentNodes(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::cuda::CUgraphNode,
    dependentNodes: *mut cuda_types::cuda::CUgraphNode,
    numDependentNodes: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hNode, "cuGraphNodeGetDependentNodes", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependentNodes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dependentNodes,
        "cuGraphNodeGetDependentNodes",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependentNodes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDependentNodes,
        "cuGraphNodeGetDependentNodes",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphNodeGetDependentNodes_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::cuda::CUgraphNode,
    dependentNodes: *mut cuda_types::cuda::CUgraphNode,
    edgeData: *mut cuda_types::cuda::CUgraphEdgeData,
    numDependentNodes: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hNode,
        "cuGraphNodeGetDependentNodes_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependentNodes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dependentNodes,
        "cuGraphNodeGetDependentNodes_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(edgeData), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &edgeData,
        "cuGraphNodeGetDependentNodes_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependentNodes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDependentNodes,
        "cuGraphNodeGetDependentNodes_v2",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphAddDependencies(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraph: cuda_types::cuda::CUgraph,
    from: *const cuda_types::cuda::CUgraphNode,
    to: *const cuda_types::cuda::CUgraphNode,
    numDependencies: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphAddDependencies", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(from), ": ").as_bytes())?;
    crate::CudaDisplay::write(&from, "cuGraphAddDependencies", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(to), ": ").as_bytes())?;
    crate::CudaDisplay::write(&to, "cuGraphAddDependencies", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDependencies,
        "cuGraphAddDependencies",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphAddDependencies_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraph: cuda_types::cuda::CUgraph,
    from: *const cuda_types::cuda::CUgraphNode,
    to: *const cuda_types::cuda::CUgraphNode,
    edgeData: *const cuda_types::cuda::CUgraphEdgeData,
    numDependencies: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphAddDependencies_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(from), ": ").as_bytes())?;
    crate::CudaDisplay::write(&from, "cuGraphAddDependencies_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(to), ": ").as_bytes())?;
    crate::CudaDisplay::write(&to, "cuGraphAddDependencies_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(edgeData), ": ").as_bytes())?;
    crate::CudaDisplay::write(&edgeData, "cuGraphAddDependencies_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDependencies,
        "cuGraphAddDependencies_v2",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphRemoveDependencies(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraph: cuda_types::cuda::CUgraph,
    from: *const cuda_types::cuda::CUgraphNode,
    to: *const cuda_types::cuda::CUgraphNode,
    numDependencies: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphRemoveDependencies", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(from), ": ").as_bytes())?;
    crate::CudaDisplay::write(&from, "cuGraphRemoveDependencies", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(to), ": ").as_bytes())?;
    crate::CudaDisplay::write(&to, "cuGraphRemoveDependencies", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDependencies,
        "cuGraphRemoveDependencies",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphRemoveDependencies_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraph: cuda_types::cuda::CUgraph,
    from: *const cuda_types::cuda::CUgraphNode,
    to: *const cuda_types::cuda::CUgraphNode,
    edgeData: *const cuda_types::cuda::CUgraphEdgeData,
    numDependencies: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphRemoveDependencies_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(from), ": ").as_bytes())?;
    crate::CudaDisplay::write(&from, "cuGraphRemoveDependencies_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(to), ": ").as_bytes())?;
    crate::CudaDisplay::write(&to, "cuGraphRemoveDependencies_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(edgeData), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &edgeData,
        "cuGraphRemoveDependencies_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDependencies,
        "cuGraphRemoveDependencies_v2",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphDestroyNode(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::cuda::CUgraphNode,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hNode, "cuGraphDestroyNode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphInstantiateWithFlags(
    writer: &mut (impl std::io::Write + ?Sized),
    phGraphExec: *mut cuda_types::cuda::CUgraphExec,
    hGraph: cuda_types::cuda::CUgraph,
    flags: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phGraphExec), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &phGraphExec,
        "cuGraphInstantiateWithFlags",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphInstantiateWithFlags", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuGraphInstantiateWithFlags", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphInstantiateWithParams_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    phGraphExec: *mut cuda_types::cuda::CUgraphExec,
    hGraph: cuda_types::cuda::CUgraph,
    instantiateParams: *mut cuda_types::cuda::CUDA_GRAPH_INSTANTIATE_PARAMS,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phGraphExec), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &phGraphExec,
        "cuGraphInstantiateWithParams_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hGraph,
        "cuGraphInstantiateWithParams_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(instantiateParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &instantiateParams,
        "cuGraphInstantiateWithParams_ptsz",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphExecGetFlags(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraphExec: cuda_types::cuda::CUgraphExec,
    flags: *mut cuda_types::cuda::cuuint64_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraphExec), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraphExec, "cuGraphExecGetFlags", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuGraphExecGetFlags", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphExecKernelNodeSetParams_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraphExec: cuda_types::cuda::CUgraphExec,
    hNode: cuda_types::cuda::CUgraphNode,
    nodeParams: *const cuda_types::cuda::CUDA_KERNEL_NODE_PARAMS,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraphExec), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hGraphExec,
        "cuGraphExecKernelNodeSetParams_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hNode,
        "cuGraphExecKernelNodeSetParams_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nodeParams,
        "cuGraphExecKernelNodeSetParams_v2",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphExecMemcpyNodeSetParams(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraphExec: cuda_types::cuda::CUgraphExec,
    hNode: cuda_types::cuda::CUgraphNode,
    copyParams: *const cuda_types::cuda::CUDA_MEMCPY3D,
    ctx: cuda_types::cuda::CUcontext,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraphExec), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hGraphExec,
        "cuGraphExecMemcpyNodeSetParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hNode,
        "cuGraphExecMemcpyNodeSetParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(copyParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &copyParams,
        "cuGraphExecMemcpyNodeSetParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ctx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ctx, "cuGraphExecMemcpyNodeSetParams", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphExecMemsetNodeSetParams(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraphExec: cuda_types::cuda::CUgraphExec,
    hNode: cuda_types::cuda::CUgraphNode,
    memsetParams: *const cuda_types::cuda::CUDA_MEMSET_NODE_PARAMS,
    ctx: cuda_types::cuda::CUcontext,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraphExec), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hGraphExec,
        "cuGraphExecMemsetNodeSetParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hNode,
        "cuGraphExecMemsetNodeSetParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(memsetParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &memsetParams,
        "cuGraphExecMemsetNodeSetParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ctx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ctx, "cuGraphExecMemsetNodeSetParams", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphExecHostNodeSetParams(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraphExec: cuda_types::cuda::CUgraphExec,
    hNode: cuda_types::cuda::CUgraphNode,
    nodeParams: *const cuda_types::cuda::CUDA_HOST_NODE_PARAMS,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraphExec), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hGraphExec,
        "cuGraphExecHostNodeSetParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hNode, "cuGraphExecHostNodeSetParams", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nodeParams,
        "cuGraphExecHostNodeSetParams",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphExecChildGraphNodeSetParams(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraphExec: cuda_types::cuda::CUgraphExec,
    hNode: cuda_types::cuda::CUgraphNode,
    childGraph: cuda_types::cuda::CUgraph,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraphExec), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hGraphExec,
        "cuGraphExecChildGraphNodeSetParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hNode,
        "cuGraphExecChildGraphNodeSetParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(childGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &childGraph,
        "cuGraphExecChildGraphNodeSetParams",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphExecEventRecordNodeSetEvent(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraphExec: cuda_types::cuda::CUgraphExec,
    hNode: cuda_types::cuda::CUgraphNode,
    event: cuda_types::cuda::CUevent,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraphExec), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hGraphExec,
        "cuGraphExecEventRecordNodeSetEvent",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hNode,
        "cuGraphExecEventRecordNodeSetEvent",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(event), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &event,
        "cuGraphExecEventRecordNodeSetEvent",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphExecEventWaitNodeSetEvent(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraphExec: cuda_types::cuda::CUgraphExec,
    hNode: cuda_types::cuda::CUgraphNode,
    event: cuda_types::cuda::CUevent,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraphExec), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hGraphExec,
        "cuGraphExecEventWaitNodeSetEvent",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hNode,
        "cuGraphExecEventWaitNodeSetEvent",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(event), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &event,
        "cuGraphExecEventWaitNodeSetEvent",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphExecExternalSemaphoresSignalNodeSetParams(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraphExec: cuda_types::cuda::CUgraphExec,
    hNode: cuda_types::cuda::CUgraphNode,
    nodeParams: *const cuda_types::cuda::CUDA_EXT_SEM_SIGNAL_NODE_PARAMS,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraphExec), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hGraphExec,
        "cuGraphExecExternalSemaphoresSignalNodeSetParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hNode,
        "cuGraphExecExternalSemaphoresSignalNodeSetParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nodeParams,
        "cuGraphExecExternalSemaphoresSignalNodeSetParams",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphExecExternalSemaphoresWaitNodeSetParams(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraphExec: cuda_types::cuda::CUgraphExec,
    hNode: cuda_types::cuda::CUgraphNode,
    nodeParams: *const cuda_types::cuda::CUDA_EXT_SEM_WAIT_NODE_PARAMS,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraphExec), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hGraphExec,
        "cuGraphExecExternalSemaphoresWaitNodeSetParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hNode,
        "cuGraphExecExternalSemaphoresWaitNodeSetParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nodeParams,
        "cuGraphExecExternalSemaphoresWaitNodeSetParams",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphNodeSetEnabled(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraphExec: cuda_types::cuda::CUgraphExec,
    hNode: cuda_types::cuda::CUgraphNode,
    isEnabled: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraphExec), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraphExec, "cuGraphNodeSetEnabled", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hNode, "cuGraphNodeSetEnabled", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(isEnabled), ": ").as_bytes())?;
    crate::CudaDisplay::write(&isEnabled, "cuGraphNodeSetEnabled", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphNodeGetEnabled(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraphExec: cuda_types::cuda::CUgraphExec,
    hNode: cuda_types::cuda::CUgraphNode,
    isEnabled: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraphExec), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraphExec, "cuGraphNodeGetEnabled", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hNode, "cuGraphNodeGetEnabled", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(isEnabled), ": ").as_bytes())?;
    crate::CudaDisplay::write(&isEnabled, "cuGraphNodeGetEnabled", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphUpload_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraphExec: cuda_types::cuda::CUgraphExec,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraphExec), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraphExec, "cuGraphUpload_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuGraphUpload_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphLaunch_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraphExec: cuda_types::cuda::CUgraphExec,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraphExec), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraphExec, "cuGraphLaunch_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuGraphLaunch_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphExecDestroy(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraphExec: cuda_types::cuda::CUgraphExec,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraphExec), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraphExec, "cuGraphExecDestroy", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphDestroy(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraph: cuda_types::cuda::CUgraph,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphDestroy", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphExecUpdate_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraphExec: cuda_types::cuda::CUgraphExec,
    hGraph: cuda_types::cuda::CUgraph,
    resultInfo: *mut cuda_types::cuda::CUgraphExecUpdateResultInfo,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraphExec), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraphExec, "cuGraphExecUpdate_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphExecUpdate_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(resultInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(&resultInfo, "cuGraphExecUpdate_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphKernelNodeCopyAttributes(
    writer: &mut (impl std::io::Write + ?Sized),
    dst: cuda_types::cuda::CUgraphNode,
    src: cuda_types::cuda::CUgraphNode,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dst), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dst, "cuGraphKernelNodeCopyAttributes", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(src), ": ").as_bytes())?;
    crate::CudaDisplay::write(&src, "cuGraphKernelNodeCopyAttributes", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphDebugDotPrint(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraph: cuda_types::cuda::CUgraph,
    path: *const ::core::ffi::c_char,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphDebugDotPrint", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(path), ": ").as_bytes())?;
    crate::CudaDisplay::write(&path, "cuGraphDebugDotPrint", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuGraphDebugDotPrint", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuUserObjectCreate(
    writer: &mut (impl std::io::Write + ?Sized),
    object_out: *mut cuda_types::cuda::CUuserObject,
    ptr: *mut ::core::ffi::c_void,
    destroy: cuda_types::cuda::CUhostFn,
    initialRefcount: ::core::ffi::c_uint,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(object_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(&object_out, "cuUserObjectCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ptr, "cuUserObjectCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(destroy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&destroy, "cuUserObjectCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(initialRefcount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&initialRefcount, "cuUserObjectCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuUserObjectCreate", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuUserObjectRetain(
    writer: &mut (impl std::io::Write + ?Sized),
    object: cuda_types::cuda::CUuserObject,
    count: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(object), ": ").as_bytes())?;
    crate::CudaDisplay::write(&object, "cuUserObjectRetain", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "cuUserObjectRetain", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuUserObjectRelease(
    writer: &mut (impl std::io::Write + ?Sized),
    object: cuda_types::cuda::CUuserObject,
    count: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(object), ": ").as_bytes())?;
    crate::CudaDisplay::write(&object, "cuUserObjectRelease", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "cuUserObjectRelease", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphRetainUserObject(
    writer: &mut (impl std::io::Write + ?Sized),
    graph: cuda_types::cuda::CUgraph,
    object: cuda_types::cuda::CUuserObject,
    count: ::core::ffi::c_uint,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(graph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&graph, "cuGraphRetainUserObject", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(object), ": ").as_bytes())?;
    crate::CudaDisplay::write(&object, "cuGraphRetainUserObject", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "cuGraphRetainUserObject", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuGraphRetainUserObject", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphReleaseUserObject(
    writer: &mut (impl std::io::Write + ?Sized),
    graph: cuda_types::cuda::CUgraph,
    object: cuda_types::cuda::CUuserObject,
    count: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(graph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&graph, "cuGraphReleaseUserObject", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(object), ": ").as_bytes())?;
    crate::CudaDisplay::write(&object, "cuGraphReleaseUserObject", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "cuGraphReleaseUserObject", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphAddNode(
    writer: &mut (impl std::io::Write + ?Sized),
    phGraphNode: *mut cuda_types::cuda::CUgraphNode,
    hGraph: cuda_types::cuda::CUgraph,
    dependencies: *const cuda_types::cuda::CUgraphNode,
    numDependencies: usize,
    nodeParams: *mut cuda_types::cuda::CUgraphNodeParams,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phGraphNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phGraphNode, "cuGraphAddNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphAddNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dependencies, "cuGraphAddNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numDependencies, "cuGraphAddNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nodeParams, "cuGraphAddNode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphAddNode_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    phGraphNode: *mut cuda_types::cuda::CUgraphNode,
    hGraph: cuda_types::cuda::CUgraph,
    dependencies: *const cuda_types::cuda::CUgraphNode,
    dependencyData: *const cuda_types::cuda::CUgraphEdgeData,
    numDependencies: usize,
    nodeParams: *mut cuda_types::cuda::CUgraphNodeParams,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phGraphNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phGraphNode, "cuGraphAddNode_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphAddNode_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dependencies, "cuGraphAddNode_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencyData), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dependencyData, "cuGraphAddNode_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numDependencies, "cuGraphAddNode_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nodeParams, "cuGraphAddNode_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphNodeSetParams(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::cuda::CUgraphNode,
    nodeParams: *mut cuda_types::cuda::CUgraphNodeParams,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hNode, "cuGraphNodeSetParams", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nodeParams, "cuGraphNodeSetParams", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphExecNodeSetParams(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraphExec: cuda_types::cuda::CUgraphExec,
    hNode: cuda_types::cuda::CUgraphNode,
    nodeParams: *mut cuda_types::cuda::CUgraphNodeParams,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraphExec), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraphExec, "cuGraphExecNodeSetParams", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hNode, "cuGraphExecNodeSetParams", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nodeParams, "cuGraphExecNodeSetParams", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphConditionalHandleCreate(
    writer: &mut (impl std::io::Write + ?Sized),
    pHandle_out: *mut cuda_types::cuda::CUgraphConditionalHandle,
    hGraph: cuda_types::cuda::CUgraph,
    ctx: cuda_types::cuda::CUcontext,
    defaultLaunchValue: ::core::ffi::c_uint,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pHandle_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pHandle_out,
        "cuGraphConditionalHandleCreate",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hGraph,
        "cuGraphConditionalHandleCreate",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ctx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ctx, "cuGraphConditionalHandleCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(defaultLaunchValue), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &defaultLaunchValue,
        "cuGraphConditionalHandleCreate",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &flags,
        "cuGraphConditionalHandleCreate",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuOccupancyMaxActiveBlocksPerMultiprocessor(
    writer: &mut (impl std::io::Write + ?Sized),
    numBlocks: *mut ::core::ffi::c_int,
    func: cuda_types::cuda::CUfunction,
    blockSize: ::core::ffi::c_int,
    dynamicSMemSize: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(numBlocks), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numBlocks,
        "cuOccupancyMaxActiveBlocksPerMultiprocessor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(func), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &func,
        "cuOccupancyMaxActiveBlocksPerMultiprocessor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(blockSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &blockSize,
        "cuOccupancyMaxActiveBlocksPerMultiprocessor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dynamicSMemSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dynamicSMemSize,
        "cuOccupancyMaxActiveBlocksPerMultiprocessor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags(
    writer: &mut (impl std::io::Write + ?Sized),
    numBlocks: *mut ::core::ffi::c_int,
    func: cuda_types::cuda::CUfunction,
    blockSize: ::core::ffi::c_int,
    dynamicSMemSize: usize,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(numBlocks), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numBlocks,
        "cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(func), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &func,
        "cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(blockSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &blockSize,
        "cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dynamicSMemSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dynamicSMemSize,
        "cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &flags,
        "cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuOccupancyMaxPotentialBlockSize(
    writer: &mut (impl std::io::Write + ?Sized),
    minGridSize: *mut ::core::ffi::c_int,
    blockSize: *mut ::core::ffi::c_int,
    func: cuda_types::cuda::CUfunction,
    blockSizeToDynamicSMemSize: cuda_types::cuda::CUoccupancyB2DSize,
    dynamicSMemSize: usize,
    blockSizeLimit: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(minGridSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &minGridSize,
        "cuOccupancyMaxPotentialBlockSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(blockSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &blockSize,
        "cuOccupancyMaxPotentialBlockSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(func), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &func,
        "cuOccupancyMaxPotentialBlockSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(blockSizeToDynamicSMemSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &blockSizeToDynamicSMemSize,
        "cuOccupancyMaxPotentialBlockSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dynamicSMemSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dynamicSMemSize,
        "cuOccupancyMaxPotentialBlockSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(blockSizeLimit), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &blockSizeLimit,
        "cuOccupancyMaxPotentialBlockSize",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuOccupancyMaxPotentialBlockSizeWithFlags(
    writer: &mut (impl std::io::Write + ?Sized),
    minGridSize: *mut ::core::ffi::c_int,
    blockSize: *mut ::core::ffi::c_int,
    func: cuda_types::cuda::CUfunction,
    blockSizeToDynamicSMemSize: cuda_types::cuda::CUoccupancyB2DSize,
    dynamicSMemSize: usize,
    blockSizeLimit: ::core::ffi::c_int,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(minGridSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &minGridSize,
        "cuOccupancyMaxPotentialBlockSizeWithFlags",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(blockSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &blockSize,
        "cuOccupancyMaxPotentialBlockSizeWithFlags",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(func), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &func,
        "cuOccupancyMaxPotentialBlockSizeWithFlags",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(blockSizeToDynamicSMemSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &blockSizeToDynamicSMemSize,
        "cuOccupancyMaxPotentialBlockSizeWithFlags",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dynamicSMemSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dynamicSMemSize,
        "cuOccupancyMaxPotentialBlockSizeWithFlags",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(blockSizeLimit), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &blockSizeLimit,
        "cuOccupancyMaxPotentialBlockSizeWithFlags",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &flags,
        "cuOccupancyMaxPotentialBlockSizeWithFlags",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuOccupancyAvailableDynamicSMemPerBlock(
    writer: &mut (impl std::io::Write + ?Sized),
    dynamicSmemSize: *mut usize,
    func: cuda_types::cuda::CUfunction,
    numBlocks: ::core::ffi::c_int,
    blockSize: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dynamicSmemSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dynamicSmemSize,
        "cuOccupancyAvailableDynamicSMemPerBlock",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(func), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &func,
        "cuOccupancyAvailableDynamicSMemPerBlock",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numBlocks), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numBlocks,
        "cuOccupancyAvailableDynamicSMemPerBlock",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(blockSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &blockSize,
        "cuOccupancyAvailableDynamicSMemPerBlock",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuOccupancyMaxPotentialClusterSize(
    writer: &mut (impl std::io::Write + ?Sized),
    clusterSize: *mut ::core::ffi::c_int,
    func: cuda_types::cuda::CUfunction,
    config: *const cuda_types::cuda::CUlaunchConfig,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(clusterSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &clusterSize,
        "cuOccupancyMaxPotentialClusterSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(func), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &func,
        "cuOccupancyMaxPotentialClusterSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(config), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &config,
        "cuOccupancyMaxPotentialClusterSize",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuOccupancyMaxActiveClusters(
    writer: &mut (impl std::io::Write + ?Sized),
    numClusters: *mut ::core::ffi::c_int,
    func: cuda_types::cuda::CUfunction,
    config: *const cuda_types::cuda::CUlaunchConfig,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(numClusters), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numClusters,
        "cuOccupancyMaxActiveClusters",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(func), ": ").as_bytes())?;
    crate::CudaDisplay::write(&func, "cuOccupancyMaxActiveClusters", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(config), ": ").as_bytes())?;
    crate::CudaDisplay::write(&config, "cuOccupancyMaxActiveClusters", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexRefSetArray(
    writer: &mut (impl std::io::Write + ?Sized),
    hTexRef: cuda_types::cuda::CUtexref,
    hArray: cuda_types::cuda::CUarray,
    Flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuTexRefSetArray", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hArray, "cuTexRefSetArray", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Flags, "cuTexRefSetArray", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexRefSetMipmappedArray(
    writer: &mut (impl std::io::Write + ?Sized),
    hTexRef: cuda_types::cuda::CUtexref,
    hMipmappedArray: cuda_types::cuda::CUmipmappedArray,
    Flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuTexRefSetMipmappedArray", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hMipmappedArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hMipmappedArray,
        "cuTexRefSetMipmappedArray",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Flags, "cuTexRefSetMipmappedArray", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexRefSetAddress_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    ByteOffset: *mut usize,
    hTexRef: cuda_types::cuda::CUtexref,
    dptr: cuda_types::cuda::CUdeviceptr,
    bytes: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(ByteOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteOffset, "cuTexRefSetAddress_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuTexRefSetAddress_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuTexRefSetAddress_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&bytes, "cuTexRefSetAddress_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexRefSetAddress2D_v3(
    writer: &mut (impl std::io::Write + ?Sized),
    hTexRef: cuda_types::cuda::CUtexref,
    desc: *const cuda_types::cuda::CUDA_ARRAY_DESCRIPTOR,
    dptr: cuda_types::cuda::CUdeviceptr,
    Pitch: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuTexRefSetAddress2D_v3", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(desc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&desc, "cuTexRefSetAddress2D_v3", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuTexRefSetAddress2D_v3", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Pitch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Pitch, "cuTexRefSetAddress2D_v3", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexRefSetFormat(
    writer: &mut (impl std::io::Write + ?Sized),
    hTexRef: cuda_types::cuda::CUtexref,
    fmt: cuda_types::cuda::CUarray_format,
    NumPackedComponents: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuTexRefSetFormat", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(fmt), ": ").as_bytes())?;
    crate::CudaDisplay::write(&fmt, "cuTexRefSetFormat", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(NumPackedComponents), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &NumPackedComponents,
        "cuTexRefSetFormat",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuTexRefSetAddressMode(
    writer: &mut (impl std::io::Write + ?Sized),
    hTexRef: cuda_types::cuda::CUtexref,
    dim: ::core::ffi::c_int,
    am: cuda_types::cuda::CUaddress_mode,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuTexRefSetAddressMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dim), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dim, "cuTexRefSetAddressMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(am), ": ").as_bytes())?;
    crate::CudaDisplay::write(&am, "cuTexRefSetAddressMode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexRefSetFilterMode(
    writer: &mut (impl std::io::Write + ?Sized),
    hTexRef: cuda_types::cuda::CUtexref,
    fm: cuda_types::cuda::CUfilter_mode,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuTexRefSetFilterMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(fm), ": ").as_bytes())?;
    crate::CudaDisplay::write(&fm, "cuTexRefSetFilterMode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexRefSetMipmapFilterMode(
    writer: &mut (impl std::io::Write + ?Sized),
    hTexRef: cuda_types::cuda::CUtexref,
    fm: cuda_types::cuda::CUfilter_mode,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuTexRefSetMipmapFilterMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(fm), ": ").as_bytes())?;
    crate::CudaDisplay::write(&fm, "cuTexRefSetMipmapFilterMode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexRefSetMipmapLevelBias(
    writer: &mut (impl std::io::Write + ?Sized),
    hTexRef: cuda_types::cuda::CUtexref,
    bias: f32,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuTexRefSetMipmapLevelBias", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bias), ": ").as_bytes())?;
    crate::CudaDisplay::write(&bias, "cuTexRefSetMipmapLevelBias", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexRefSetMipmapLevelClamp(
    writer: &mut (impl std::io::Write + ?Sized),
    hTexRef: cuda_types::cuda::CUtexref,
    minMipmapLevelClamp: f32,
    maxMipmapLevelClamp: f32,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuTexRefSetMipmapLevelClamp", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(minMipmapLevelClamp), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &minMipmapLevelClamp,
        "cuTexRefSetMipmapLevelClamp",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(maxMipmapLevelClamp), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &maxMipmapLevelClamp,
        "cuTexRefSetMipmapLevelClamp",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuTexRefSetMaxAnisotropy(
    writer: &mut (impl std::io::Write + ?Sized),
    hTexRef: cuda_types::cuda::CUtexref,
    maxAniso: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuTexRefSetMaxAnisotropy", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(maxAniso), ": ").as_bytes())?;
    crate::CudaDisplay::write(&maxAniso, "cuTexRefSetMaxAnisotropy", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexRefSetBorderColor(
    writer: &mut (impl std::io::Write + ?Sized),
    hTexRef: cuda_types::cuda::CUtexref,
    pBorderColor: *mut f32,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuTexRefSetBorderColor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pBorderColor), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pBorderColor, "cuTexRefSetBorderColor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexRefSetFlags(
    writer: &mut (impl std::io::Write + ?Sized),
    hTexRef: cuda_types::cuda::CUtexref,
    Flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuTexRefSetFlags", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Flags, "cuTexRefSetFlags", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexRefGetAddress_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    pdptr: *mut cuda_types::cuda::CUdeviceptr,
    hTexRef: cuda_types::cuda::CUtexref,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pdptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pdptr, "cuTexRefGetAddress_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuTexRefGetAddress_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexRefGetArray(
    writer: &mut (impl std::io::Write + ?Sized),
    phArray: *mut cuda_types::cuda::CUarray,
    hTexRef: cuda_types::cuda::CUtexref,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phArray, "cuTexRefGetArray", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuTexRefGetArray", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexRefGetMipmappedArray(
    writer: &mut (impl std::io::Write + ?Sized),
    phMipmappedArray: *mut cuda_types::cuda::CUmipmappedArray,
    hTexRef: cuda_types::cuda::CUtexref,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phMipmappedArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &phMipmappedArray,
        "cuTexRefGetMipmappedArray",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuTexRefGetMipmappedArray", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexRefGetAddressMode(
    writer: &mut (impl std::io::Write + ?Sized),
    pam: *mut cuda_types::cuda::CUaddress_mode,
    hTexRef: cuda_types::cuda::CUtexref,
    dim: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pam), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pam, "cuTexRefGetAddressMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuTexRefGetAddressMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dim), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dim, "cuTexRefGetAddressMode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexRefGetFilterMode(
    writer: &mut (impl std::io::Write + ?Sized),
    pfm: *mut cuda_types::cuda::CUfilter_mode,
    hTexRef: cuda_types::cuda::CUtexref,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pfm), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pfm, "cuTexRefGetFilterMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuTexRefGetFilterMode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexRefGetFormat(
    writer: &mut (impl std::io::Write + ?Sized),
    pFormat: *mut cuda_types::cuda::CUarray_format,
    pNumChannels: *mut ::core::ffi::c_int,
    hTexRef: cuda_types::cuda::CUtexref,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pFormat), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pFormat, "cuTexRefGetFormat", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pNumChannels), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pNumChannels, "cuTexRefGetFormat", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuTexRefGetFormat", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexRefGetMipmapFilterMode(
    writer: &mut (impl std::io::Write + ?Sized),
    pfm: *mut cuda_types::cuda::CUfilter_mode,
    hTexRef: cuda_types::cuda::CUtexref,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pfm), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pfm, "cuTexRefGetMipmapFilterMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuTexRefGetMipmapFilterMode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexRefGetMipmapLevelBias(
    writer: &mut (impl std::io::Write + ?Sized),
    pbias: *mut f32,
    hTexRef: cuda_types::cuda::CUtexref,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pbias), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pbias, "cuTexRefGetMipmapLevelBias", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuTexRefGetMipmapLevelBias", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexRefGetMipmapLevelClamp(
    writer: &mut (impl std::io::Write + ?Sized),
    pminMipmapLevelClamp: *mut f32,
    pmaxMipmapLevelClamp: *mut f32,
    hTexRef: cuda_types::cuda::CUtexref,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pminMipmapLevelClamp), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pminMipmapLevelClamp,
        "cuTexRefGetMipmapLevelClamp",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pmaxMipmapLevelClamp), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pmaxMipmapLevelClamp,
        "cuTexRefGetMipmapLevelClamp",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuTexRefGetMipmapLevelClamp", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexRefGetMaxAnisotropy(
    writer: &mut (impl std::io::Write + ?Sized),
    pmaxAniso: *mut ::core::ffi::c_int,
    hTexRef: cuda_types::cuda::CUtexref,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pmaxAniso), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pmaxAniso, "cuTexRefGetMaxAnisotropy", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuTexRefGetMaxAnisotropy", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexRefGetBorderColor(
    writer: &mut (impl std::io::Write + ?Sized),
    pBorderColor: *mut f32,
    hTexRef: cuda_types::cuda::CUtexref,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pBorderColor), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pBorderColor, "cuTexRefGetBorderColor", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuTexRefGetBorderColor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexRefGetFlags(
    writer: &mut (impl std::io::Write + ?Sized),
    pFlags: *mut ::core::ffi::c_uint,
    hTexRef: cuda_types::cuda::CUtexref,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pFlags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pFlags, "cuTexRefGetFlags", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuTexRefGetFlags", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexRefCreate(
    writer: &mut (impl std::io::Write + ?Sized),
    pTexRef: *mut cuda_types::cuda::CUtexref,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pTexRef, "cuTexRefCreate", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexRefDestroy(
    writer: &mut (impl std::io::Write + ?Sized),
    hTexRef: cuda_types::cuda::CUtexref,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuTexRefDestroy", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuSurfRefSetArray(
    writer: &mut (impl std::io::Write + ?Sized),
    hSurfRef: cuda_types::cuda::CUsurfref,
    hArray: cuda_types::cuda::CUarray,
    Flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hSurfRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hSurfRef, "cuSurfRefSetArray", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hArray, "cuSurfRefSetArray", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Flags, "cuSurfRefSetArray", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuSurfRefGetArray(
    writer: &mut (impl std::io::Write + ?Sized),
    phArray: *mut cuda_types::cuda::CUarray,
    hSurfRef: cuda_types::cuda::CUsurfref,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phArray, "cuSurfRefGetArray", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hSurfRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hSurfRef, "cuSurfRefGetArray", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexObjectCreate(
    writer: &mut (impl std::io::Write + ?Sized),
    pTexObject: *mut cuda_types::cuda::CUtexObject,
    pResDesc: *const cuda_types::cuda::CUDA_RESOURCE_DESC,
    pTexDesc: *const cuda_types::cuda::CUDA_TEXTURE_DESC,
    pResViewDesc: *const cuda_types::cuda::CUDA_RESOURCE_VIEW_DESC,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pTexObject), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pTexObject, "cuTexObjectCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pResDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pResDesc, "cuTexObjectCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pTexDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pTexDesc, "cuTexObjectCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pResViewDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pResViewDesc, "cuTexObjectCreate", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexObjectDestroy(
    writer: &mut (impl std::io::Write + ?Sized),
    texObject: cuda_types::cuda::CUtexObject,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(texObject), ": ").as_bytes())?;
    crate::CudaDisplay::write(&texObject, "cuTexObjectDestroy", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexObjectGetResourceDesc(
    writer: &mut (impl std::io::Write + ?Sized),
    pResDesc: *mut cuda_types::cuda::CUDA_RESOURCE_DESC,
    texObject: cuda_types::cuda::CUtexObject,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pResDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pResDesc, "cuTexObjectGetResourceDesc", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(texObject), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &texObject,
        "cuTexObjectGetResourceDesc",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuTexObjectGetTextureDesc(
    writer: &mut (impl std::io::Write + ?Sized),
    pTexDesc: *mut cuda_types::cuda::CUDA_TEXTURE_DESC,
    texObject: cuda_types::cuda::CUtexObject,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pTexDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pTexDesc, "cuTexObjectGetTextureDesc", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(texObject), ": ").as_bytes())?;
    crate::CudaDisplay::write(&texObject, "cuTexObjectGetTextureDesc", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexObjectGetResourceViewDesc(
    writer: &mut (impl std::io::Write + ?Sized),
    pResViewDesc: *mut cuda_types::cuda::CUDA_RESOURCE_VIEW_DESC,
    texObject: cuda_types::cuda::CUtexObject,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pResViewDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pResViewDesc,
        "cuTexObjectGetResourceViewDesc",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(texObject), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &texObject,
        "cuTexObjectGetResourceViewDesc",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuSurfObjectCreate(
    writer: &mut (impl std::io::Write + ?Sized),
    pSurfObject: *mut cuda_types::cuda::CUsurfObject,
    pResDesc: *const cuda_types::cuda::CUDA_RESOURCE_DESC,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pSurfObject), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pSurfObject, "cuSurfObjectCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pResDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pResDesc, "cuSurfObjectCreate", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuSurfObjectDestroy(
    writer: &mut (impl std::io::Write + ?Sized),
    surfObject: cuda_types::cuda::CUsurfObject,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(surfObject), ": ").as_bytes())?;
    crate::CudaDisplay::write(&surfObject, "cuSurfObjectDestroy", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuSurfObjectGetResourceDesc(
    writer: &mut (impl std::io::Write + ?Sized),
    pResDesc: *mut cuda_types::cuda::CUDA_RESOURCE_DESC,
    surfObject: cuda_types::cuda::CUsurfObject,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pResDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pResDesc,
        "cuSurfObjectGetResourceDesc",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(surfObject), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &surfObject,
        "cuSurfObjectGetResourceDesc",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuTensorMapEncodeTiled(
    writer: &mut (impl std::io::Write + ?Sized),
    tensorMap: *mut cuda_types::cuda::CUtensorMap,
    tensorDataType: cuda_types::cuda::CUtensorMapDataType,
    tensorRank: cuda_types::cuda::cuuint32_t,
    globalAddress: *mut ::core::ffi::c_void,
    globalDim: *const cuda_types::cuda::cuuint64_t,
    globalStrides: *const cuda_types::cuda::cuuint64_t,
    boxDim: *const cuda_types::cuda::cuuint32_t,
    elementStrides: *const cuda_types::cuda::cuuint32_t,
    interleave: cuda_types::cuda::CUtensorMapInterleave,
    swizzle: cuda_types::cuda::CUtensorMapSwizzle,
    l2Promotion: cuda_types::cuda::CUtensorMapL2promotion,
    oobFill: cuda_types::cuda::CUtensorMapFloatOOBfill,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(tensorMap), ": ").as_bytes())?;
    crate::CudaDisplay::write(&tensorMap, "cuTensorMapEncodeTiled", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(tensorDataType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &tensorDataType,
        "cuTensorMapEncodeTiled",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(tensorRank), ": ").as_bytes())?;
    crate::CudaDisplay::write(&tensorRank, "cuTensorMapEncodeTiled", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(globalAddress), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &globalAddress,
        "cuTensorMapEncodeTiled",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(globalDim), ": ").as_bytes())?;
    crate::CudaDisplay::write(&globalDim, "cuTensorMapEncodeTiled", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(globalStrides), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &globalStrides,
        "cuTensorMapEncodeTiled",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(boxDim), ": ").as_bytes())?;
    crate::CudaDisplay::write(&boxDim, "cuTensorMapEncodeTiled", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(elementStrides), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &elementStrides,
        "cuTensorMapEncodeTiled",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(interleave), ": ").as_bytes())?;
    crate::CudaDisplay::write(&interleave, "cuTensorMapEncodeTiled", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(swizzle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&swizzle, "cuTensorMapEncodeTiled", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(l2Promotion), ": ").as_bytes())?;
    crate::CudaDisplay::write(&l2Promotion, "cuTensorMapEncodeTiled", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(oobFill), ": ").as_bytes())?;
    crate::CudaDisplay::write(&oobFill, "cuTensorMapEncodeTiled", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTensorMapEncodeIm2col(
    writer: &mut (impl std::io::Write + ?Sized),
    tensorMap: *mut cuda_types::cuda::CUtensorMap,
    tensorDataType: cuda_types::cuda::CUtensorMapDataType,
    tensorRank: cuda_types::cuda::cuuint32_t,
    globalAddress: *mut ::core::ffi::c_void,
    globalDim: *const cuda_types::cuda::cuuint64_t,
    globalStrides: *const cuda_types::cuda::cuuint64_t,
    pixelBoxLowerCorner: *const ::core::ffi::c_int,
    pixelBoxUpperCorner: *const ::core::ffi::c_int,
    channelsPerPixel: cuda_types::cuda::cuuint32_t,
    pixelsPerColumn: cuda_types::cuda::cuuint32_t,
    elementStrides: *const cuda_types::cuda::cuuint32_t,
    interleave: cuda_types::cuda::CUtensorMapInterleave,
    swizzle: cuda_types::cuda::CUtensorMapSwizzle,
    l2Promotion: cuda_types::cuda::CUtensorMapL2promotion,
    oobFill: cuda_types::cuda::CUtensorMapFloatOOBfill,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(tensorMap), ": ").as_bytes())?;
    crate::CudaDisplay::write(&tensorMap, "cuTensorMapEncodeIm2col", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(tensorDataType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &tensorDataType,
        "cuTensorMapEncodeIm2col",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(tensorRank), ": ").as_bytes())?;
    crate::CudaDisplay::write(&tensorRank, "cuTensorMapEncodeIm2col", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(globalAddress), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &globalAddress,
        "cuTensorMapEncodeIm2col",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(globalDim), ": ").as_bytes())?;
    crate::CudaDisplay::write(&globalDim, "cuTensorMapEncodeIm2col", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(globalStrides), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &globalStrides,
        "cuTensorMapEncodeIm2col",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pixelBoxLowerCorner), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pixelBoxLowerCorner,
        "cuTensorMapEncodeIm2col",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pixelBoxUpperCorner), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pixelBoxUpperCorner,
        "cuTensorMapEncodeIm2col",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(channelsPerPixel), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &channelsPerPixel,
        "cuTensorMapEncodeIm2col",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pixelsPerColumn), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pixelsPerColumn,
        "cuTensorMapEncodeIm2col",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(elementStrides), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &elementStrides,
        "cuTensorMapEncodeIm2col",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(interleave), ": ").as_bytes())?;
    crate::CudaDisplay::write(&interleave, "cuTensorMapEncodeIm2col", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(swizzle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&swizzle, "cuTensorMapEncodeIm2col", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(l2Promotion), ": ").as_bytes())?;
    crate::CudaDisplay::write(&l2Promotion, "cuTensorMapEncodeIm2col", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(oobFill), ": ").as_bytes())?;
    crate::CudaDisplay::write(&oobFill, "cuTensorMapEncodeIm2col", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTensorMapEncodeIm2colWide(
    writer: &mut (impl std::io::Write + ?Sized),
    tensorMap: *mut cuda_types::cuda::CUtensorMap,
    tensorDataType: cuda_types::cuda::CUtensorMapDataType,
    tensorRank: cuda_types::cuda::cuuint32_t,
    globalAddress: *mut ::core::ffi::c_void,
    globalDim: *const cuda_types::cuda::cuuint64_t,
    globalStrides: *const cuda_types::cuda::cuuint64_t,
    pixelBoxLowerCornerWidth: ::core::ffi::c_int,
    pixelBoxUpperCornerWidth: ::core::ffi::c_int,
    channelsPerPixel: cuda_types::cuda::cuuint32_t,
    pixelsPerColumn: cuda_types::cuda::cuuint32_t,
    elementStrides: *const cuda_types::cuda::cuuint32_t,
    interleave: cuda_types::cuda::CUtensorMapInterleave,
    mode: cuda_types::cuda::CUtensorMapIm2ColWideMode,
    swizzle: cuda_types::cuda::CUtensorMapSwizzle,
    l2Promotion: cuda_types::cuda::CUtensorMapL2promotion,
    oobFill: cuda_types::cuda::CUtensorMapFloatOOBfill,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(tensorMap), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &tensorMap,
        "cuTensorMapEncodeIm2colWide",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(tensorDataType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &tensorDataType,
        "cuTensorMapEncodeIm2colWide",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(tensorRank), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &tensorRank,
        "cuTensorMapEncodeIm2colWide",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(globalAddress), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &globalAddress,
        "cuTensorMapEncodeIm2colWide",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(globalDim), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &globalDim,
        "cuTensorMapEncodeIm2colWide",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(globalStrides), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &globalStrides,
        "cuTensorMapEncodeIm2colWide",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pixelBoxLowerCornerWidth), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pixelBoxLowerCornerWidth,
        "cuTensorMapEncodeIm2colWide",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pixelBoxUpperCornerWidth), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pixelBoxUpperCornerWidth,
        "cuTensorMapEncodeIm2colWide",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(channelsPerPixel), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &channelsPerPixel,
        "cuTensorMapEncodeIm2colWide",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pixelsPerColumn), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pixelsPerColumn,
        "cuTensorMapEncodeIm2colWide",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(elementStrides), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &elementStrides,
        "cuTensorMapEncodeIm2colWide",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(interleave), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &interleave,
        "cuTensorMapEncodeIm2colWide",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mode, "cuTensorMapEncodeIm2colWide", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(swizzle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&swizzle, "cuTensorMapEncodeIm2colWide", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(l2Promotion), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &l2Promotion,
        "cuTensorMapEncodeIm2colWide",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(oobFill), ": ").as_bytes())?;
    crate::CudaDisplay::write(&oobFill, "cuTensorMapEncodeIm2colWide", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTensorMapReplaceAddress(
    writer: &mut (impl std::io::Write + ?Sized),
    tensorMap: *mut cuda_types::cuda::CUtensorMap,
    globalAddress: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(tensorMap), ": ").as_bytes())?;
    crate::CudaDisplay::write(&tensorMap, "cuTensorMapReplaceAddress", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(globalAddress), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &globalAddress,
        "cuTensorMapReplaceAddress",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuDeviceCanAccessPeer(
    writer: &mut (impl std::io::Write + ?Sized),
    canAccessPeer: *mut ::core::ffi::c_int,
    dev: cuda_types::cuda::CUdevice,
    peerDev: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(canAccessPeer), ": ").as_bytes())?;
    crate::CudaDisplay::write(&canAccessPeer, "cuDeviceCanAccessPeer", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuDeviceCanAccessPeer", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(peerDev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&peerDev, "cuDeviceCanAccessPeer", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxEnablePeerAccess(
    writer: &mut (impl std::io::Write + ?Sized),
    peerContext: cuda_types::cuda::CUcontext,
    Flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(peerContext), ": ").as_bytes())?;
    crate::CudaDisplay::write(&peerContext, "cuCtxEnablePeerAccess", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Flags, "cuCtxEnablePeerAccess", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxDisablePeerAccess(
    writer: &mut (impl std::io::Write + ?Sized),
    peerContext: cuda_types::cuda::CUcontext,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(peerContext), ": ").as_bytes())?;
    crate::CudaDisplay::write(&peerContext, "cuCtxDisablePeerAccess", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuDeviceGetP2PAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    value: *mut ::core::ffi::c_int,
    attrib: cuda_types::cuda::CUdevice_P2PAttribute,
    srcDevice: cuda_types::cuda::CUdevice,
    dstDevice: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cuDeviceGetP2PAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attrib), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attrib, "cuDeviceGetP2PAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcDevice, "cuDeviceGetP2PAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuDeviceGetP2PAttribute", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphicsUnregisterResource(
    writer: &mut (impl std::io::Write + ?Sized),
    resource: cuda_types::cuda::CUgraphicsResource,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(resource), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &resource,
        "cuGraphicsUnregisterResource",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphicsSubResourceGetMappedArray(
    writer: &mut (impl std::io::Write + ?Sized),
    pArray: *mut cuda_types::cuda::CUarray,
    resource: cuda_types::cuda::CUgraphicsResource,
    arrayIndex: ::core::ffi::c_uint,
    mipLevel: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pArray,
        "cuGraphicsSubResourceGetMappedArray",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(resource), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &resource,
        "cuGraphicsSubResourceGetMappedArray",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(arrayIndex), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &arrayIndex,
        "cuGraphicsSubResourceGetMappedArray",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mipLevel), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mipLevel,
        "cuGraphicsSubResourceGetMappedArray",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphicsResourceGetMappedMipmappedArray(
    writer: &mut (impl std::io::Write + ?Sized),
    pMipmappedArray: *mut cuda_types::cuda::CUmipmappedArray,
    resource: cuda_types::cuda::CUgraphicsResource,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pMipmappedArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pMipmappedArray,
        "cuGraphicsResourceGetMappedMipmappedArray",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(resource), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &resource,
        "cuGraphicsResourceGetMappedMipmappedArray",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphicsResourceGetMappedPointer_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    pDevPtr: *mut cuda_types::cuda::CUdeviceptr,
    pSize: *mut usize,
    resource: cuda_types::cuda::CUgraphicsResource,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pDevPtr), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pDevPtr,
        "cuGraphicsResourceGetMappedPointer_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pSize,
        "cuGraphicsResourceGetMappedPointer_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(resource), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &resource,
        "cuGraphicsResourceGetMappedPointer_v2",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphicsResourceSetMapFlags_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    resource: cuda_types::cuda::CUgraphicsResource,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(resource), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &resource,
        "cuGraphicsResourceSetMapFlags_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &flags,
        "cuGraphicsResourceSetMapFlags_v2",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphicsMapResources_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    count: ::core::ffi::c_uint,
    resources: *mut cuda_types::cuda::CUgraphicsResource,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "cuGraphicsMapResources_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(resources), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &resources,
        "cuGraphicsMapResources_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuGraphicsMapResources_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphicsUnmapResources_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    count: ::core::ffi::c_uint,
    resources: *mut cuda_types::cuda::CUgraphicsResource,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "cuGraphicsUnmapResources_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(resources), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &resources,
        "cuGraphicsUnmapResources_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hStream,
        "cuGraphicsUnmapResources_ptsz",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGetProcAddress_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    symbol: *const ::core::ffi::c_char,
    pfn: *mut *mut ::core::ffi::c_void,
    cudaVersion: ::core::ffi::c_int,
    flags: cuda_types::cuda::cuuint64_t,
    symbolStatus: *mut cuda_types::cuda::CUdriverProcAddressQueryResult,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(symbol), ": ").as_bytes())?;
    crate::CudaDisplay::write(&symbol, "cuGetProcAddress_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pfn), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pfn, "cuGetProcAddress_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cudaVersion), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cudaVersion, "cuGetProcAddress_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuGetProcAddress_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(symbolStatus), ": ").as_bytes())?;
    crate::CudaDisplay::write(&symbolStatus, "cuGetProcAddress_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::cuda::CUcoredumpSettings_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUcoredumpSettings_enum::CU_COREDUMP_ENABLE_ON_EXCEPTION => {
                writer.write_all(stringify!(CU_COREDUMP_ENABLE_ON_EXCEPTION).as_bytes())
            }
            &cuda_types::cuda::CUcoredumpSettings_enum::CU_COREDUMP_TRIGGER_HOST => {
                writer.write_all(stringify!(CU_COREDUMP_TRIGGER_HOST).as_bytes())
            }
            &cuda_types::cuda::CUcoredumpSettings_enum::CU_COREDUMP_LIGHTWEIGHT => {
                writer.write_all(stringify!(CU_COREDUMP_LIGHTWEIGHT).as_bytes())
            }
            &cuda_types::cuda::CUcoredumpSettings_enum::CU_COREDUMP_ENABLE_USER_TRIGGER => {
                writer.write_all(stringify!(CU_COREDUMP_ENABLE_USER_TRIGGER).as_bytes())
            }
            &cuda_types::cuda::CUcoredumpSettings_enum::CU_COREDUMP_FILE => {
                writer.write_all(stringify!(CU_COREDUMP_FILE).as_bytes())
            }
            &cuda_types::cuda::CUcoredumpSettings_enum::CU_COREDUMP_PIPE => {
                writer.write_all(stringify!(CU_COREDUMP_PIPE).as_bytes())
            }
            &cuda_types::cuda::CUcoredumpSettings_enum::CU_COREDUMP_GENERATION_FLAGS => {
                writer.write_all(stringify!(CU_COREDUMP_GENERATION_FLAGS).as_bytes())
            }
            &cuda_types::cuda::CUcoredumpSettings_enum::CU_COREDUMP_MAX => {
                writer.write_all(stringify!(CU_COREDUMP_MAX).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUCoredumpGenerationFlags {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUCoredumpGenerationFlags::CU_COREDUMP_DEFAULT_FLAGS => {
                writer.write_all(stringify!(CU_COREDUMP_DEFAULT_FLAGS).as_bytes())
            }
            &cuda_types::cuda::CUCoredumpGenerationFlags::CU_COREDUMP_SKIP_NONRELOCATED_ELF_IMAGES => {
                writer
                    .write_all(
                        stringify!(CU_COREDUMP_SKIP_NONRELOCATED_ELF_IMAGES).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUCoredumpGenerationFlags::CU_COREDUMP_SKIP_GLOBAL_MEMORY => {
                writer.write_all(stringify!(CU_COREDUMP_SKIP_GLOBAL_MEMORY).as_bytes())
            }
            &cuda_types::cuda::CUCoredumpGenerationFlags::CU_COREDUMP_SKIP_SHARED_MEMORY => {
                writer.write_all(stringify!(CU_COREDUMP_SKIP_SHARED_MEMORY).as_bytes())
            }
            &cuda_types::cuda::CUCoredumpGenerationFlags::CU_COREDUMP_SKIP_LOCAL_MEMORY => {
                writer.write_all(stringify!(CU_COREDUMP_SKIP_LOCAL_MEMORY).as_bytes())
            }
            &cuda_types::cuda::CUCoredumpGenerationFlags::CU_COREDUMP_SKIP_ABORT => {
                writer.write_all(stringify!(CU_COREDUMP_SKIP_ABORT).as_bytes())
            }
            &cuda_types::cuda::CUCoredumpGenerationFlags::CU_COREDUMP_SKIP_CONSTBANK_MEMORY => {
                writer
                    .write_all(stringify!(CU_COREDUMP_SKIP_CONSTBANK_MEMORY).as_bytes())
            }
            &cuda_types::cuda::CUCoredumpGenerationFlags::CU_COREDUMP_LIGHTWEIGHT_FLAGS => {
                writer.write_all(stringify!(CU_COREDUMP_LIGHTWEIGHT_FLAGS).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cuCoredumpGetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    attrib: cuda_types::cuda::CUcoredumpSettings,
    value: *mut ::core::ffi::c_void,
    size: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(attrib), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attrib, "cuCoredumpGetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cuCoredumpGetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(&size, "cuCoredumpGetAttribute", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCoredumpGetAttributeGlobal(
    writer: &mut (impl std::io::Write + ?Sized),
    attrib: cuda_types::cuda::CUcoredumpSettings,
    value: *mut ::core::ffi::c_void,
    size: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(attrib), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attrib, "cuCoredumpGetAttributeGlobal", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cuCoredumpGetAttributeGlobal", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(&size, "cuCoredumpGetAttributeGlobal", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCoredumpSetAttribute(
    writer: &mut (impl std::io::Write + ?Sized),
    attrib: cuda_types::cuda::CUcoredumpSettings,
    value: *mut ::core::ffi::c_void,
    size: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(attrib), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attrib, "cuCoredumpSetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cuCoredumpSetAttribute", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(&size, "cuCoredumpSetAttribute", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCoredumpSetAttributeGlobal(
    writer: &mut (impl std::io::Write + ?Sized),
    attrib: cuda_types::cuda::CUcoredumpSettings,
    value: *mut ::core::ffi::c_void,
    size: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(attrib), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attrib, "cuCoredumpSetAttributeGlobal", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cuCoredumpSetAttributeGlobal", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(&size, "cuCoredumpSetAttributeGlobal", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGetExportTable(
    writer: &mut (impl std::io::Write + ?Sized),
    ppExportTable: *mut *const ::core::ffi::c_void,
    pExportTableId: *const cuda_types::cuda::CUuuid,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(ppExportTable), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ppExportTable, "cuGetExportTable", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pExportTableId), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pExportTableId, "cuGetExportTable", arg_idx, writer)?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::cuda::CUdevResourceDesc {
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
impl crate::CudaDisplay for cuda_types::cuda::CUgreenCtxCreate_flags {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUgreenCtxCreate_flags::CU_GREEN_CTX_DEFAULT_STREAM => {
                writer.write_all(stringify!(CU_GREEN_CTX_DEFAULT_STREAM).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUdevSmResourceSplit_flags {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUdevSmResourceSplit_flags::CU_DEV_SM_RESOURCE_SPLIT_IGNORE_SM_COSCHEDULING => {
                writer
                    .write_all(
                        stringify!(CU_DEV_SM_RESOURCE_SPLIT_IGNORE_SM_COSCHEDULING)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUdevSmResourceSplit_flags::CU_DEV_SM_RESOURCE_SPLIT_MAX_POTENTIAL_CLUSTER_SIZE => {
                writer
                    .write_all(
                        stringify!(CU_DEV_SM_RESOURCE_SPLIT_MAX_POTENTIAL_CLUSTER_SIZE)
                            .as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUdevResourceType {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUdevResourceType::CU_DEV_RESOURCE_TYPE_INVALID => {
                writer.write_all(stringify!(CU_DEV_RESOURCE_TYPE_INVALID).as_bytes())
            }
            &cuda_types::cuda::CUdevResourceType::CU_DEV_RESOURCE_TYPE_SM => {
                writer.write_all(stringify!(CU_DEV_RESOURCE_TYPE_SM).as_bytes())
            }
            &cuda_types::cuda::CUdevResourceType::CU_DEV_RESOURCE_TYPE_MAX => {
                writer.write_all(stringify!(CU_DEV_RESOURCE_TYPE_MAX).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUdevSmResource_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(smCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.smCount, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
pub fn write_cuGreenCtxCreate(
    writer: &mut (impl std::io::Write + ?Sized),
    phCtx: *mut cuda_types::cuda::CUgreenCtx,
    desc: cuda_types::cuda::CUdevResourceDesc,
    dev: cuda_types::cuda::CUdevice,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phCtx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phCtx, "cuGreenCtxCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(desc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&desc, "cuGreenCtxCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuGreenCtxCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuGreenCtxCreate", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGreenCtxDestroy(
    writer: &mut (impl std::io::Write + ?Sized),
    hCtx: cuda_types::cuda::CUgreenCtx,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hCtx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hCtx, "cuGreenCtxDestroy", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxFromGreenCtx(
    writer: &mut (impl std::io::Write + ?Sized),
    pContext: *mut cuda_types::cuda::CUcontext,
    hCtx: cuda_types::cuda::CUgreenCtx,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pContext), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pContext, "cuCtxFromGreenCtx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hCtx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hCtx, "cuCtxFromGreenCtx", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuDeviceGetDevResource(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::cuda::CUdevice,
    resource: *mut cuda_types::cuda::CUdevResource,
    type_: cuda_types::cuda::CUdevResourceType,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "cuDeviceGetDevResource", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(resource), ": ").as_bytes())?;
    crate::CudaDisplay::write(&resource, "cuDeviceGetDevResource", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cuDeviceGetDevResource", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxGetDevResource(
    writer: &mut (impl std::io::Write + ?Sized),
    hCtx: cuda_types::cuda::CUcontext,
    resource: *mut cuda_types::cuda::CUdevResource,
    type_: cuda_types::cuda::CUdevResourceType,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hCtx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hCtx, "cuCtxGetDevResource", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(resource), ": ").as_bytes())?;
    crate::CudaDisplay::write(&resource, "cuCtxGetDevResource", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cuCtxGetDevResource", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGreenCtxGetDevResource(
    writer: &mut (impl std::io::Write + ?Sized),
    hCtx: cuda_types::cuda::CUgreenCtx,
    resource: *mut cuda_types::cuda::CUdevResource,
    type_: cuda_types::cuda::CUdevResourceType,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hCtx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hCtx, "cuGreenCtxGetDevResource", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(resource), ": ").as_bytes())?;
    crate::CudaDisplay::write(&resource, "cuGreenCtxGetDevResource", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cuGreenCtxGetDevResource", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuDevSmResourceSplitByCount(
    writer: &mut (impl std::io::Write + ?Sized),
    result: *mut cuda_types::cuda::CUdevResource,
    nbGroups: *mut ::core::ffi::c_uint,
    input: *const cuda_types::cuda::CUdevResource,
    remaining: *mut cuda_types::cuda::CUdevResource,
    useFlags: ::core::ffi::c_uint,
    minCount: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(result), ": ").as_bytes())?;
    crate::CudaDisplay::write(&result, "cuDevSmResourceSplitByCount", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nbGroups), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nbGroups,
        "cuDevSmResourceSplitByCount",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(input), ": ").as_bytes())?;
    crate::CudaDisplay::write(&input, "cuDevSmResourceSplitByCount", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(remaining), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &remaining,
        "cuDevSmResourceSplitByCount",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(useFlags), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &useFlags,
        "cuDevSmResourceSplitByCount",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(minCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &minCount,
        "cuDevSmResourceSplitByCount",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuDevResourceGenerateDesc(
    writer: &mut (impl std::io::Write + ?Sized),
    phDesc: *mut cuda_types::cuda::CUdevResourceDesc,
    resources: *mut cuda_types::cuda::CUdevResource,
    nbResources: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phDesc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phDesc, "cuDevResourceGenerateDesc", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(resources), ": ").as_bytes())?;
    crate::CudaDisplay::write(&resources, "cuDevResourceGenerateDesc", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nbResources), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nbResources,
        "cuDevResourceGenerateDesc",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGreenCtxRecordEvent(
    writer: &mut (impl std::io::Write + ?Sized),
    hCtx: cuda_types::cuda::CUgreenCtx,
    hEvent: cuda_types::cuda::CUevent,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hCtx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hCtx, "cuGreenCtxRecordEvent", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hEvent), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hEvent, "cuGreenCtxRecordEvent", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGreenCtxWaitEvent(
    writer: &mut (impl std::io::Write + ?Sized),
    hCtx: cuda_types::cuda::CUgreenCtx,
    hEvent: cuda_types::cuda::CUevent,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hCtx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hCtx, "cuGreenCtxWaitEvent", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hEvent), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hEvent, "cuGreenCtxWaitEvent", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamGetGreenCtx(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    phCtx: *mut cuda_types::cuda::CUgreenCtx,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamGetGreenCtx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(phCtx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phCtx, "cuStreamGetGreenCtx", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGreenCtxStreamCreate(
    writer: &mut (impl std::io::Write + ?Sized),
    phStream: *mut cuda_types::cuda::CUstream,
    greenCtx: cuda_types::cuda::CUgreenCtx,
    flags: ::core::ffi::c_uint,
    priority: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phStream, "cuGreenCtxStreamCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(greenCtx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&greenCtx, "cuGreenCtxStreamCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuGreenCtxStreamCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(priority), ": ").as_bytes())?;
    crate::CudaDisplay::write(&priority, "cuGreenCtxStreamCreate", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemHostRegister(
    writer: &mut (impl std::io::Write + ?Sized),
    p: *mut ::core::ffi::c_void,
    bytesize: usize,
    Flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(p), ": ").as_bytes())?;
    crate::CudaDisplay::write(&p, "cuMemHostRegister", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bytesize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&bytesize, "cuMemHostRegister", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Flags, "cuMemHostRegister", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphicsResourceSetMapFlags(
    writer: &mut (impl std::io::Write + ?Sized),
    resource: cuda_types::cuda::CUgraphicsResource,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(resource), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &resource,
        "cuGraphicsResourceSetMapFlags",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuGraphicsResourceSetMapFlags", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuLinkCreate(
    writer: &mut (impl std::io::Write + ?Sized),
    numOptions: ::core::ffi::c_uint,
    options: *mut cuda_types::cuda::CUjit_option,
    optionValues: *mut *mut ::core::ffi::c_void,
    stateOut: *mut cuda_types::cuda::CUlinkState,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(numOptions), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numOptions, "cuLinkCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(options), ": ").as_bytes())?;
    crate::CudaDisplay::write(&options, "cuLinkCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(optionValues), ": ").as_bytes())?;
    crate::CudaDisplay::write(&optionValues, "cuLinkCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(stateOut), ": ").as_bytes())?;
    crate::CudaDisplay::write(&stateOut, "cuLinkCreate", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuLinkAddData(
    writer: &mut (impl std::io::Write + ?Sized),
    state: cuda_types::cuda::CUlinkState,
    type_: cuda_types::cuda::CUjitInputType,
    data: *mut ::core::ffi::c_void,
    size: usize,
    name: *const ::core::ffi::c_char,
    numOptions: ::core::ffi::c_uint,
    options: *mut cuda_types::cuda::CUjit_option,
    optionValues: *mut *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(state), ": ").as_bytes())?;
    crate::CudaDisplay::write(&state, "cuLinkAddData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cuLinkAddData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(data), ": ").as_bytes())?;
    crate::CudaDisplay::write(&data, "cuLinkAddData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(&size, "cuLinkAddData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(name), ": ").as_bytes())?;
    crate::CudaDisplay::write(&name, "cuLinkAddData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numOptions), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numOptions, "cuLinkAddData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(options), ": ").as_bytes())?;
    crate::CudaDisplay::write(&options, "cuLinkAddData", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(optionValues), ": ").as_bytes())?;
    crate::CudaDisplay::write(&optionValues, "cuLinkAddData", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuLinkAddFile(
    writer: &mut (impl std::io::Write + ?Sized),
    state: cuda_types::cuda::CUlinkState,
    type_: cuda_types::cuda::CUjitInputType,
    path: *const ::core::ffi::c_char,
    numOptions: ::core::ffi::c_uint,
    options: *mut cuda_types::cuda::CUjit_option,
    optionValues: *mut *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(state), ": ").as_bytes())?;
    crate::CudaDisplay::write(&state, "cuLinkAddFile", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "cuLinkAddFile", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(path), ": ").as_bytes())?;
    crate::CudaDisplay::write(&path, "cuLinkAddFile", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numOptions), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numOptions, "cuLinkAddFile", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(options), ": ").as_bytes())?;
    crate::CudaDisplay::write(&options, "cuLinkAddFile", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(optionValues), ": ").as_bytes())?;
    crate::CudaDisplay::write(&optionValues, "cuLinkAddFile", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexRefSetAddress2D_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    hTexRef: cuda_types::cuda::CUtexref,
    desc: *const cuda_types::cuda::CUDA_ARRAY_DESCRIPTOR,
    dptr: cuda_types::cuda::CUdeviceptr,
    Pitch: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuTexRefSetAddress2D_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(desc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&desc, "cuTexRefSetAddress2D_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuTexRefSetAddress2D_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Pitch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Pitch, "cuTexRefSetAddress2D_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_MEMCPY2D_v1_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(srcXInBytes), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcXInBytes, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcY), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcY, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcMemoryType), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcMemoryType, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcHost), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcHost, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcDevice), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcDevice, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcArray), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcArray, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcPitch), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcPitch, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstXInBytes), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstXInBytes, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstY), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstY, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstMemoryType), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstMemoryType, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstHost), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstHost, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstDevice), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstDevice, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstArray), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstArray, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstPitch), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstPitch, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(WidthInBytes), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.WidthInBytes, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(Height), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.Height, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_MEMCPY3D_v1_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(srcXInBytes), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcXInBytes, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcY), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcY, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcZ), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcZ, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcLOD), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcLOD, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcMemoryType), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcMemoryType, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcHost), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcHost, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcDevice), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcDevice, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcArray), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcArray, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcPitch), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcPitch, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(srcHeight), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.srcHeight, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstXInBytes), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstXInBytes, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstY), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstY, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstZ), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstZ, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstLOD), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstLOD, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstMemoryType), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstMemoryType, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstHost), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstHost, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstDevice), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstDevice, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstArray), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstArray, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstPitch), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstPitch, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dstHeight), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dstHeight, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(WidthInBytes), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.WidthInBytes, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(Height), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.Height, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(Depth), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.Depth, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_ARRAY_DESCRIPTOR_v1_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(Width), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.Width, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(Height), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.Height, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(Format), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.Format, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(NumChannels), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.NumChannels, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUDA_ARRAY3D_DESCRIPTOR_v1_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(Width), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.Width, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(Height), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.Height, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(Depth), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.Depth, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(Format), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.Format, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(NumChannels), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.NumChannels, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(Flags), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.Flags, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
pub fn write_cuDeviceTotalMem(
    writer: &mut (impl std::io::Write + ?Sized),
    bytes: *mut ::core::ffi::c_uint,
    dev: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(bytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&bytes, "cuDeviceTotalMem", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuDeviceTotalMem", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxCreate(
    writer: &mut (impl std::io::Write + ?Sized),
    pctx: *mut cuda_types::cuda::CUcontext,
    flags: ::core::ffi::c_uint,
    dev: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pctx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pctx, "cuCtxCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuCtxCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuCtxCreate", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuModuleGetGlobal(
    writer: &mut (impl std::io::Write + ?Sized),
    dptr: *mut cuda_types::cuda::CUdeviceptr_v1,
    bytes: *mut ::core::ffi::c_uint,
    hmod: cuda_types::cuda::CUmodule,
    name: *const ::core::ffi::c_char,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuModuleGetGlobal", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&bytes, "cuModuleGetGlobal", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hmod), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hmod, "cuModuleGetGlobal", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(name), ": ").as_bytes())?;
    crate::CudaDisplay::write(&name, "cuModuleGetGlobal", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemGetInfo(
    writer: &mut (impl std::io::Write + ?Sized),
    free: *mut ::core::ffi::c_uint,
    total: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(free), ": ").as_bytes())?;
    crate::CudaDisplay::write(&free, "cuMemGetInfo", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(total), ": ").as_bytes())?;
    crate::CudaDisplay::write(&total, "cuMemGetInfo", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemAlloc(
    writer: &mut (impl std::io::Write + ?Sized),
    dptr: *mut cuda_types::cuda::CUdeviceptr_v1,
    bytesize: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuMemAlloc", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bytesize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&bytesize, "cuMemAlloc", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemAllocPitch(
    writer: &mut (impl std::io::Write + ?Sized),
    dptr: *mut cuda_types::cuda::CUdeviceptr_v1,
    pPitch: *mut ::core::ffi::c_uint,
    WidthInBytes: ::core::ffi::c_uint,
    Height: ::core::ffi::c_uint,
    ElementSizeBytes: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuMemAllocPitch", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pPitch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pPitch, "cuMemAllocPitch", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(WidthInBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&WidthInBytes, "cuMemAllocPitch", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Height), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Height, "cuMemAllocPitch", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ElementSizeBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ElementSizeBytes, "cuMemAllocPitch", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemFree(
    writer: &mut (impl std::io::Write + ?Sized),
    dptr: cuda_types::cuda::CUdeviceptr_v1,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuMemFree", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemGetAddressRange(
    writer: &mut (impl std::io::Write + ?Sized),
    pbase: *mut cuda_types::cuda::CUdeviceptr_v1,
    psize: *mut ::core::ffi::c_uint,
    dptr: cuda_types::cuda::CUdeviceptr_v1,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pbase), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pbase, "cuMemGetAddressRange", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(psize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&psize, "cuMemGetAddressRange", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuMemGetAddressRange", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemAllocHost(
    writer: &mut (impl std::io::Write + ?Sized),
    pp: *mut *mut ::core::ffi::c_void,
    bytesize: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pp), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pp, "cuMemAllocHost", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bytesize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&bytesize, "cuMemAllocHost", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemHostGetDevicePointer(
    writer: &mut (impl std::io::Write + ?Sized),
    pdptr: *mut cuda_types::cuda::CUdeviceptr_v1,
    p: *mut ::core::ffi::c_void,
    Flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pdptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pdptr, "cuMemHostGetDevicePointer", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(p), ": ").as_bytes())?;
    crate::CudaDisplay::write(&p, "cuMemHostGetDevicePointer", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Flags, "cuMemHostGetDevicePointer", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyHtoD(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr_v1,
    srcHost: *const ::core::ffi::c_void,
    ByteCount: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemcpyHtoD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcHost), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcHost, "cuMemcpyHtoD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyHtoD", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyDtoH(
    writer: &mut (impl std::io::Write + ?Sized),
    dstHost: *mut ::core::ffi::c_void,
    srcDevice: cuda_types::cuda::CUdeviceptr_v1,
    ByteCount: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstHost), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstHost, "cuMemcpyDtoH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcDevice, "cuMemcpyDtoH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyDtoH", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyDtoD(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr_v1,
    srcDevice: cuda_types::cuda::CUdeviceptr_v1,
    ByteCount: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemcpyDtoD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcDevice, "cuMemcpyDtoD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyDtoD", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyDtoA(
    writer: &mut (impl std::io::Write + ?Sized),
    dstArray: cuda_types::cuda::CUarray,
    dstOffset: ::core::ffi::c_uint,
    srcDevice: cuda_types::cuda::CUdeviceptr_v1,
    ByteCount: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstArray, "cuMemcpyDtoA", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstOffset, "cuMemcpyDtoA", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcDevice, "cuMemcpyDtoA", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyDtoA", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyAtoD(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr_v1,
    srcArray: cuda_types::cuda::CUarray,
    srcOffset: ::core::ffi::c_uint,
    ByteCount: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemcpyAtoD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcArray, "cuMemcpyAtoD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcOffset, "cuMemcpyAtoD", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyAtoD", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyHtoA(
    writer: &mut (impl std::io::Write + ?Sized),
    dstArray: cuda_types::cuda::CUarray,
    dstOffset: ::core::ffi::c_uint,
    srcHost: *const ::core::ffi::c_void,
    ByteCount: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstArray, "cuMemcpyHtoA", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstOffset, "cuMemcpyHtoA", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcHost), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcHost, "cuMemcpyHtoA", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyHtoA", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyAtoH(
    writer: &mut (impl std::io::Write + ?Sized),
    dstHost: *mut ::core::ffi::c_void,
    srcArray: cuda_types::cuda::CUarray,
    srcOffset: ::core::ffi::c_uint,
    ByteCount: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstHost), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstHost, "cuMemcpyAtoH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcArray, "cuMemcpyAtoH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcOffset, "cuMemcpyAtoH", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyAtoH", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyAtoA(
    writer: &mut (impl std::io::Write + ?Sized),
    dstArray: cuda_types::cuda::CUarray,
    dstOffset: ::core::ffi::c_uint,
    srcArray: cuda_types::cuda::CUarray,
    srcOffset: ::core::ffi::c_uint,
    ByteCount: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstArray, "cuMemcpyAtoA", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstOffset, "cuMemcpyAtoA", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcArray, "cuMemcpyAtoA", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcOffset, "cuMemcpyAtoA", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyAtoA", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyHtoAAsync(
    writer: &mut (impl std::io::Write + ?Sized),
    dstArray: cuda_types::cuda::CUarray,
    dstOffset: ::core::ffi::c_uint,
    srcHost: *const ::core::ffi::c_void,
    ByteCount: ::core::ffi::c_uint,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstArray, "cuMemcpyHtoAAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstOffset, "cuMemcpyHtoAAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcHost), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcHost, "cuMemcpyHtoAAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyHtoAAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpyHtoAAsync", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyAtoHAsync(
    writer: &mut (impl std::io::Write + ?Sized),
    dstHost: *mut ::core::ffi::c_void,
    srcArray: cuda_types::cuda::CUarray,
    srcOffset: ::core::ffi::c_uint,
    ByteCount: ::core::ffi::c_uint,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstHost), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstHost, "cuMemcpyAtoHAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcArray, "cuMemcpyAtoHAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcOffset, "cuMemcpyAtoHAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyAtoHAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpyAtoHAsync", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpy2D(
    writer: &mut (impl std::io::Write + ?Sized),
    pCopy: *const cuda_types::cuda::CUDA_MEMCPY2D_v1,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pCopy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pCopy, "cuMemcpy2D", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpy2DUnaligned(
    writer: &mut (impl std::io::Write + ?Sized),
    pCopy: *const cuda_types::cuda::CUDA_MEMCPY2D_v1,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pCopy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pCopy, "cuMemcpy2DUnaligned", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpy3D(
    writer: &mut (impl std::io::Write + ?Sized),
    pCopy: *const cuda_types::cuda::CUDA_MEMCPY3D_v1,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pCopy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pCopy, "cuMemcpy3D", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyHtoDAsync(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr_v1,
    srcHost: *const ::core::ffi::c_void,
    ByteCount: ::core::ffi::c_uint,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemcpyHtoDAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcHost), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcHost, "cuMemcpyHtoDAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyHtoDAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpyHtoDAsync", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyDtoHAsync(
    writer: &mut (impl std::io::Write + ?Sized),
    dstHost: *mut ::core::ffi::c_void,
    srcDevice: cuda_types::cuda::CUdeviceptr_v1,
    ByteCount: ::core::ffi::c_uint,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstHost), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstHost, "cuMemcpyDtoHAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcDevice, "cuMemcpyDtoHAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyDtoHAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpyDtoHAsync", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyDtoDAsync(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr_v1,
    srcDevice: cuda_types::cuda::CUdeviceptr_v1,
    ByteCount: ::core::ffi::c_uint,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemcpyDtoDAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcDevice, "cuMemcpyDtoDAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyDtoDAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpyDtoDAsync", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpy2DAsync(
    writer: &mut (impl std::io::Write + ?Sized),
    pCopy: *const cuda_types::cuda::CUDA_MEMCPY2D_v1,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pCopy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pCopy, "cuMemcpy2DAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpy2DAsync", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpy3DAsync(
    writer: &mut (impl std::io::Write + ?Sized),
    pCopy: *const cuda_types::cuda::CUDA_MEMCPY3D_v1,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pCopy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pCopy, "cuMemcpy3DAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpy3DAsync", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemsetD8(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr_v1,
    uc: ::core::ffi::c_uchar,
    N: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemsetD8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(uc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&uc, "cuMemsetD8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(N), ": ").as_bytes())?;
    crate::CudaDisplay::write(&N, "cuMemsetD8", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemsetD16(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr_v1,
    us: ::core::ffi::c_ushort,
    N: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemsetD16", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(us), ": ").as_bytes())?;
    crate::CudaDisplay::write(&us, "cuMemsetD16", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(N), ": ").as_bytes())?;
    crate::CudaDisplay::write(&N, "cuMemsetD16", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemsetD32(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr_v1,
    ui: ::core::ffi::c_uint,
    N: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemsetD32", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ui), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ui, "cuMemsetD32", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(N), ": ").as_bytes())?;
    crate::CudaDisplay::write(&N, "cuMemsetD32", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemsetD2D8(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr_v1,
    dstPitch: ::core::ffi::c_uint,
    uc: ::core::ffi::c_uchar,
    Width: ::core::ffi::c_uint,
    Height: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemsetD2D8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstPitch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstPitch, "cuMemsetD2D8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(uc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&uc, "cuMemsetD2D8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Width), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Width, "cuMemsetD2D8", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Height), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Height, "cuMemsetD2D8", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemsetD2D16(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr_v1,
    dstPitch: ::core::ffi::c_uint,
    us: ::core::ffi::c_ushort,
    Width: ::core::ffi::c_uint,
    Height: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemsetD2D16", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstPitch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstPitch, "cuMemsetD2D16", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(us), ": ").as_bytes())?;
    crate::CudaDisplay::write(&us, "cuMemsetD2D16", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Width), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Width, "cuMemsetD2D16", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Height), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Height, "cuMemsetD2D16", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemsetD2D32(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr_v1,
    dstPitch: ::core::ffi::c_uint,
    ui: ::core::ffi::c_uint,
    Width: ::core::ffi::c_uint,
    Height: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemsetD2D32", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstPitch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstPitch, "cuMemsetD2D32", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ui), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ui, "cuMemsetD2D32", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Width), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Width, "cuMemsetD2D32", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Height), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Height, "cuMemsetD2D32", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuArrayCreate(
    writer: &mut (impl std::io::Write + ?Sized),
    pHandle: *mut cuda_types::cuda::CUarray,
    pAllocateArray: *const cuda_types::cuda::CUDA_ARRAY_DESCRIPTOR_v1,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pHandle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pHandle, "cuArrayCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pAllocateArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pAllocateArray, "cuArrayCreate", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuArrayGetDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    pArrayDescriptor: *mut cuda_types::cuda::CUDA_ARRAY_DESCRIPTOR_v1,
    hArray: cuda_types::cuda::CUarray,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pArrayDescriptor), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pArrayDescriptor,
        "cuArrayGetDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hArray, "cuArrayGetDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuArray3DCreate(
    writer: &mut (impl std::io::Write + ?Sized),
    pHandle: *mut cuda_types::cuda::CUarray,
    pAllocateArray: *const cuda_types::cuda::CUDA_ARRAY3D_DESCRIPTOR_v1,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pHandle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pHandle, "cuArray3DCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pAllocateArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pAllocateArray, "cuArray3DCreate", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuArray3DGetDescriptor(
    writer: &mut (impl std::io::Write + ?Sized),
    pArrayDescriptor: *mut cuda_types::cuda::CUDA_ARRAY3D_DESCRIPTOR_v1,
    hArray: cuda_types::cuda::CUarray,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pArrayDescriptor), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pArrayDescriptor,
        "cuArray3DGetDescriptor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hArray, "cuArray3DGetDescriptor", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexRefSetAddress(
    writer: &mut (impl std::io::Write + ?Sized),
    ByteOffset: *mut ::core::ffi::c_uint,
    hTexRef: cuda_types::cuda::CUtexref,
    dptr: cuda_types::cuda::CUdeviceptr_v1,
    bytes: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(ByteOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteOffset, "cuTexRefSetAddress", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuTexRefSetAddress", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuTexRefSetAddress", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&bytes, "cuTexRefSetAddress", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexRefSetAddress2D(
    writer: &mut (impl std::io::Write + ?Sized),
    hTexRef: cuda_types::cuda::CUtexref,
    desc: *const cuda_types::cuda::CUDA_ARRAY_DESCRIPTOR_v1,
    dptr: cuda_types::cuda::CUdeviceptr_v1,
    Pitch: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuTexRefSetAddress2D", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(desc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&desc, "cuTexRefSetAddress2D", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuTexRefSetAddress2D", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Pitch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Pitch, "cuTexRefSetAddress2D", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuTexRefGetAddress(
    writer: &mut (impl std::io::Write + ?Sized),
    pdptr: *mut cuda_types::cuda::CUdeviceptr_v1,
    hTexRef: cuda_types::cuda::CUtexref,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pdptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pdptr, "cuTexRefGetAddress", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hTexRef), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hTexRef, "cuTexRefGetAddress", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphicsResourceGetMappedPointer(
    writer: &mut (impl std::io::Write + ?Sized),
    pDevPtr: *mut cuda_types::cuda::CUdeviceptr_v1,
    pSize: *mut ::core::ffi::c_uint,
    resource: cuda_types::cuda::CUgraphicsResource,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pDevPtr), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pDevPtr,
        "cuGraphicsResourceGetMappedPointer",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pSize,
        "cuGraphicsResourceGetMappedPointer",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(resource), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &resource,
        "cuGraphicsResourceGetMappedPointer",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuCtxDestroy(
    writer: &mut (impl std::io::Write + ?Sized),
    ctx: cuda_types::cuda::CUcontext,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(ctx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ctx, "cuCtxDestroy", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxPopCurrent(
    writer: &mut (impl std::io::Write + ?Sized),
    pctx: *mut cuda_types::cuda::CUcontext,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pctx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pctx, "cuCtxPopCurrent", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCtxPushCurrent(
    writer: &mut (impl std::io::Write + ?Sized),
    ctx: cuda_types::cuda::CUcontext,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(ctx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ctx, "cuCtxPushCurrent", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamDestroy(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamDestroy", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuEventDestroy(
    writer: &mut (impl std::io::Write + ?Sized),
    hEvent: cuda_types::cuda::CUevent,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hEvent), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hEvent, "cuEventDestroy", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuDevicePrimaryCtxRelease(
    writer: &mut (impl std::io::Write + ?Sized),
    dev: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuDevicePrimaryCtxRelease", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuDevicePrimaryCtxReset(
    writer: &mut (impl std::io::Write + ?Sized),
    dev: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuDevicePrimaryCtxReset", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuDevicePrimaryCtxSetFlags(
    writer: &mut (impl std::io::Write + ?Sized),
    dev: cuda_types::cuda::CUdevice,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dev), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dev, "cuDevicePrimaryCtxSetFlags", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuDevicePrimaryCtxSetFlags", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyHtoD_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    srcHost: *const ::core::ffi::c_void,
    ByteCount: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemcpyHtoD_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcHost), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcHost, "cuMemcpyHtoD_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyHtoD_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyDtoH_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    dstHost: *mut ::core::ffi::c_void,
    srcDevice: cuda_types::cuda::CUdeviceptr,
    ByteCount: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstHost), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstHost, "cuMemcpyDtoH_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcDevice, "cuMemcpyDtoH_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyDtoH_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyDtoD_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    srcDevice: cuda_types::cuda::CUdeviceptr,
    ByteCount: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemcpyDtoD_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcDevice, "cuMemcpyDtoD_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyDtoD_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyDtoA_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    dstArray: cuda_types::cuda::CUarray,
    dstOffset: usize,
    srcDevice: cuda_types::cuda::CUdeviceptr,
    ByteCount: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstArray, "cuMemcpyDtoA_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstOffset, "cuMemcpyDtoA_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcDevice, "cuMemcpyDtoA_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyDtoA_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyAtoD_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    srcArray: cuda_types::cuda::CUarray,
    srcOffset: usize,
    ByteCount: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemcpyAtoD_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcArray, "cuMemcpyAtoD_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcOffset, "cuMemcpyAtoD_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyAtoD_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyHtoA_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    dstArray: cuda_types::cuda::CUarray,
    dstOffset: usize,
    srcHost: *const ::core::ffi::c_void,
    ByteCount: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstArray, "cuMemcpyHtoA_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstOffset, "cuMemcpyHtoA_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcHost), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcHost, "cuMemcpyHtoA_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyHtoA_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyAtoH_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    dstHost: *mut ::core::ffi::c_void,
    srcArray: cuda_types::cuda::CUarray,
    srcOffset: usize,
    ByteCount: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstHost), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstHost, "cuMemcpyAtoH_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcArray, "cuMemcpyAtoH_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcOffset, "cuMemcpyAtoH_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyAtoH_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyAtoA_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    dstArray: cuda_types::cuda::CUarray,
    dstOffset: usize,
    srcArray: cuda_types::cuda::CUarray,
    srcOffset: usize,
    ByteCount: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstArray, "cuMemcpyAtoA_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstOffset, "cuMemcpyAtoA_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcArray, "cuMemcpyAtoA_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcOffset, "cuMemcpyAtoA_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyAtoA_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyHtoAAsync_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    dstArray: cuda_types::cuda::CUarray,
    dstOffset: usize,
    srcHost: *const ::core::ffi::c_void,
    ByteCount: usize,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstArray, "cuMemcpyHtoAAsync_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstOffset, "cuMemcpyHtoAAsync_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcHost), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcHost, "cuMemcpyHtoAAsync_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyHtoAAsync_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpyHtoAAsync_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyAtoHAsync_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    dstHost: *mut ::core::ffi::c_void,
    srcArray: cuda_types::cuda::CUarray,
    srcOffset: usize,
    ByteCount: usize,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstHost), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstHost, "cuMemcpyAtoHAsync_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcArray, "cuMemcpyAtoHAsync_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcOffset, "cuMemcpyAtoHAsync_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyAtoHAsync_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpyAtoHAsync_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpy2D_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    pCopy: *const cuda_types::cuda::CUDA_MEMCPY2D,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pCopy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pCopy, "cuMemcpy2D_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpy2DUnaligned_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    pCopy: *const cuda_types::cuda::CUDA_MEMCPY2D,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pCopy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pCopy, "cuMemcpy2DUnaligned_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpy3D_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    pCopy: *const cuda_types::cuda::CUDA_MEMCPY3D,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pCopy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pCopy, "cuMemcpy3D_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyHtoDAsync_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    srcHost: *const ::core::ffi::c_void,
    ByteCount: usize,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemcpyHtoDAsync_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcHost), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcHost, "cuMemcpyHtoDAsync_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyHtoDAsync_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpyHtoDAsync_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyDtoHAsync_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    dstHost: *mut ::core::ffi::c_void,
    srcDevice: cuda_types::cuda::CUdeviceptr,
    ByteCount: usize,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstHost), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstHost, "cuMemcpyDtoHAsync_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcDevice, "cuMemcpyDtoHAsync_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyDtoHAsync_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpyDtoHAsync_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyDtoDAsync_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    srcDevice: cuda_types::cuda::CUdeviceptr,
    ByteCount: usize,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemcpyDtoDAsync_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcDevice, "cuMemcpyDtoDAsync_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyDtoDAsync_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpyDtoDAsync_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpy2DAsync_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    pCopy: *const cuda_types::cuda::CUDA_MEMCPY2D,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pCopy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pCopy, "cuMemcpy2DAsync_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpy2DAsync_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpy3DAsync_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    pCopy: *const cuda_types::cuda::CUDA_MEMCPY3D,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pCopy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pCopy, "cuMemcpy3DAsync_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpy3DAsync_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemsetD8_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    uc: ::core::ffi::c_uchar,
    N: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemsetD8_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(uc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&uc, "cuMemsetD8_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(N), ": ").as_bytes())?;
    crate::CudaDisplay::write(&N, "cuMemsetD8_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemsetD16_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    us: ::core::ffi::c_ushort,
    N: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemsetD16_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(us), ": ").as_bytes())?;
    crate::CudaDisplay::write(&us, "cuMemsetD16_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(N), ": ").as_bytes())?;
    crate::CudaDisplay::write(&N, "cuMemsetD16_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemsetD32_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    ui: ::core::ffi::c_uint,
    N: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemsetD32_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ui), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ui, "cuMemsetD32_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(N), ": ").as_bytes())?;
    crate::CudaDisplay::write(&N, "cuMemsetD32_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemsetD2D8_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    dstPitch: usize,
    uc: ::core::ffi::c_uchar,
    Width: usize,
    Height: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemsetD2D8_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstPitch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstPitch, "cuMemsetD2D8_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(uc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&uc, "cuMemsetD2D8_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Width), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Width, "cuMemsetD2D8_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Height), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Height, "cuMemsetD2D8_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemsetD2D16_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    dstPitch: usize,
    us: ::core::ffi::c_ushort,
    Width: usize,
    Height: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemsetD2D16_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstPitch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstPitch, "cuMemsetD2D16_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(us), ": ").as_bytes())?;
    crate::CudaDisplay::write(&us, "cuMemsetD2D16_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Width), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Width, "cuMemsetD2D16_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Height), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Height, "cuMemsetD2D16_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemsetD2D32_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    dstPitch: usize,
    ui: ::core::ffi::c_uint,
    Width: usize,
    Height: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemsetD2D32_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstPitch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstPitch, "cuMemsetD2D32_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ui), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ui, "cuMemsetD2D32_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Width), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Width, "cuMemsetD2D32_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Height), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Height, "cuMemsetD2D32_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpy(
    writer: &mut (impl std::io::Write + ?Sized),
    dst: cuda_types::cuda::CUdeviceptr,
    src: cuda_types::cuda::CUdeviceptr,
    ByteCount: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dst), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dst, "cuMemcpy", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(src), ": ").as_bytes())?;
    crate::CudaDisplay::write(&src, "cuMemcpy", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpy", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyAsync(
    writer: &mut (impl std::io::Write + ?Sized),
    dst: cuda_types::cuda::CUdeviceptr,
    src: cuda_types::cuda::CUdeviceptr,
    ByteCount: usize,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dst), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dst, "cuMemcpyAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(src), ": ").as_bytes())?;
    crate::CudaDisplay::write(&src, "cuMemcpyAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpyAsync", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyPeer(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    dstContext: cuda_types::cuda::CUcontext,
    srcDevice: cuda_types::cuda::CUdeviceptr,
    srcContext: cuda_types::cuda::CUcontext,
    ByteCount: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemcpyPeer", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstContext), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstContext, "cuMemcpyPeer", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcDevice, "cuMemcpyPeer", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcContext), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcContext, "cuMemcpyPeer", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyPeer", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyPeerAsync(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    dstContext: cuda_types::cuda::CUcontext,
    srcDevice: cuda_types::cuda::CUdeviceptr,
    srcContext: cuda_types::cuda::CUcontext,
    ByteCount: usize,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemcpyPeerAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstContext), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstContext, "cuMemcpyPeerAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcDevice, "cuMemcpyPeerAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcContext), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcContext, "cuMemcpyPeerAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ByteCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ByteCount, "cuMemcpyPeerAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpyPeerAsync", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpy3DPeer(
    writer: &mut (impl std::io::Write + ?Sized),
    pCopy: *const cuda_types::cuda::CUDA_MEMCPY3D_PEER,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pCopy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pCopy, "cuMemcpy3DPeer", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpy3DPeerAsync(
    writer: &mut (impl std::io::Write + ?Sized),
    pCopy: *const cuda_types::cuda::CUDA_MEMCPY3D_PEER,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pCopy), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pCopy, "cuMemcpy3DPeerAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpy3DPeerAsync", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpyBatchAsync(
    writer: &mut (impl std::io::Write + ?Sized),
    dsts: *mut cuda_types::cuda::CUdeviceptr,
    srcs: *mut cuda_types::cuda::CUdeviceptr,
    sizes: *mut usize,
    count: usize,
    attrs: *mut cuda_types::cuda::CUmemcpyAttributes,
    attrsIdxs: *mut usize,
    numAttrs: usize,
    failIdx: *mut usize,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dsts), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dsts, "cuMemcpyBatchAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcs), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcs, "cuMemcpyBatchAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&sizes, "cuMemcpyBatchAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "cuMemcpyBatchAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attrs), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attrs, "cuMemcpyBatchAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attrsIdxs), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attrsIdxs, "cuMemcpyBatchAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numAttrs), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numAttrs, "cuMemcpyBatchAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(failIdx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&failIdx, "cuMemcpyBatchAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpyBatchAsync", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemcpy3DBatchAsync(
    writer: &mut (impl std::io::Write + ?Sized),
    numOps: usize,
    opList: *mut cuda_types::cuda::CUDA_MEMCPY3D_BATCH_OP,
    failIdx: *mut usize,
    flags: ::core::ffi::c_ulonglong,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(numOps), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numOps, "cuMemcpy3DBatchAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(opList), ": ").as_bytes())?;
    crate::CudaDisplay::write(&opList, "cuMemcpy3DBatchAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(failIdx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&failIdx, "cuMemcpy3DBatchAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuMemcpy3DBatchAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemcpy3DBatchAsync", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemsetD8Async(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    uc: ::core::ffi::c_uchar,
    N: usize,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemsetD8Async", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(uc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&uc, "cuMemsetD8Async", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(N), ": ").as_bytes())?;
    crate::CudaDisplay::write(&N, "cuMemsetD8Async", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemsetD8Async", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemsetD16Async(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    us: ::core::ffi::c_ushort,
    N: usize,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemsetD16Async", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(us), ": ").as_bytes())?;
    crate::CudaDisplay::write(&us, "cuMemsetD16Async", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(N), ": ").as_bytes())?;
    crate::CudaDisplay::write(&N, "cuMemsetD16Async", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemsetD16Async", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemsetD32Async(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    ui: ::core::ffi::c_uint,
    N: usize,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemsetD32Async", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ui), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ui, "cuMemsetD32Async", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(N), ": ").as_bytes())?;
    crate::CudaDisplay::write(&N, "cuMemsetD32Async", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemsetD32Async", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemsetD2D8Async(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    dstPitch: usize,
    uc: ::core::ffi::c_uchar,
    Width: usize,
    Height: usize,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemsetD2D8Async", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstPitch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstPitch, "cuMemsetD2D8Async", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(uc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&uc, "cuMemsetD2D8Async", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Width), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Width, "cuMemsetD2D8Async", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Height), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Height, "cuMemsetD2D8Async", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemsetD2D8Async", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemsetD2D16Async(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    dstPitch: usize,
    us: ::core::ffi::c_ushort,
    Width: usize,
    Height: usize,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemsetD2D16Async", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstPitch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstPitch, "cuMemsetD2D16Async", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(us), ": ").as_bytes())?;
    crate::CudaDisplay::write(&us, "cuMemsetD2D16Async", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Width), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Width, "cuMemsetD2D16Async", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Height), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Height, "cuMemsetD2D16Async", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemsetD2D16Async", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemsetD2D32Async(
    writer: &mut (impl std::io::Write + ?Sized),
    dstDevice: cuda_types::cuda::CUdeviceptr,
    dstPitch: usize,
    ui: ::core::ffi::c_uint,
    Width: usize,
    Height: usize,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemsetD2D32Async", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstPitch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstPitch, "cuMemsetD2D32Async", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ui), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ui, "cuMemsetD2D32Async", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Width), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Width, "cuMemsetD2D32Async", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Height), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Height, "cuMemsetD2D32Async", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemsetD2D32Async", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamGetPriority(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    priority: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamGetPriority", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(priority), ": ").as_bytes())?;
    crate::CudaDisplay::write(&priority, "cuStreamGetPriority", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamGetId(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    streamId: *mut ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamGetId", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(streamId), ": ").as_bytes())?;
    crate::CudaDisplay::write(&streamId, "cuStreamGetId", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamGetFlags(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    flags: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamGetFlags", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuStreamGetFlags", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamGetDevice(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    device: *mut cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamGetDevice", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "cuStreamGetDevice", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamGetCtx(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    pctx: *mut cuda_types::cuda::CUcontext,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamGetCtx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pctx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pctx, "cuStreamGetCtx", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamGetCtx_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    pCtx: *mut cuda_types::cuda::CUcontext,
    pGreenCtx: *mut cuda_types::cuda::CUgreenCtx,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamGetCtx_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pCtx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pCtx, "cuStreamGetCtx_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pGreenCtx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pGreenCtx, "cuStreamGetCtx_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamWaitEvent(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    hEvent: cuda_types::cuda::CUevent,
    Flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamWaitEvent", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hEvent), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hEvent, "cuStreamWaitEvent", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Flags, "cuStreamWaitEvent", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamAddCallback(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    callback: cuda_types::cuda::CUstreamCallback,
    userData: *mut ::core::ffi::c_void,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamAddCallback", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(callback), ": ").as_bytes())?;
    crate::CudaDisplay::write(&callback, "cuStreamAddCallback", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(userData), ": ").as_bytes())?;
    crate::CudaDisplay::write(&userData, "cuStreamAddCallback", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuStreamAddCallback", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamAttachMemAsync(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    dptr: cuda_types::cuda::CUdeviceptr,
    length: usize,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamAttachMemAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuStreamAttachMemAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(length), ": ").as_bytes())?;
    crate::CudaDisplay::write(&length, "cuStreamAttachMemAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuStreamAttachMemAsync", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamQuery(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamQuery", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamSynchronize(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamSynchronize", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuEventRecord(
    writer: &mut (impl std::io::Write + ?Sized),
    hEvent: cuda_types::cuda::CUevent,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hEvent), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hEvent, "cuEventRecord", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuEventRecord", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuEventRecordWithFlags(
    writer: &mut (impl std::io::Write + ?Sized),
    hEvent: cuda_types::cuda::CUevent,
    hStream: cuda_types::cuda::CUstream,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hEvent), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hEvent, "cuEventRecordWithFlags", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuEventRecordWithFlags", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuEventRecordWithFlags", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuLaunchKernel(
    writer: &mut (impl std::io::Write + ?Sized),
    f: cuda_types::cuda::CUfunction,
    gridDimX: ::core::ffi::c_uint,
    gridDimY: ::core::ffi::c_uint,
    gridDimZ: ::core::ffi::c_uint,
    blockDimX: ::core::ffi::c_uint,
    blockDimY: ::core::ffi::c_uint,
    blockDimZ: ::core::ffi::c_uint,
    sharedMemBytes: ::core::ffi::c_uint,
    hStream: cuda_types::cuda::CUstream,
    kernelParams: *mut *mut ::core::ffi::c_void,
    extra: *mut *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(f), ": ").as_bytes())?;
    crate::CudaDisplay::write(&f, "cuLaunchKernel", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gridDimX), ": ").as_bytes())?;
    crate::CudaDisplay::write(&gridDimX, "cuLaunchKernel", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gridDimY), ": ").as_bytes())?;
    crate::CudaDisplay::write(&gridDimY, "cuLaunchKernel", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gridDimZ), ": ").as_bytes())?;
    crate::CudaDisplay::write(&gridDimZ, "cuLaunchKernel", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(blockDimX), ": ").as_bytes())?;
    crate::CudaDisplay::write(&blockDimX, "cuLaunchKernel", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(blockDimY), ": ").as_bytes())?;
    crate::CudaDisplay::write(&blockDimY, "cuLaunchKernel", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(blockDimZ), ": ").as_bytes())?;
    crate::CudaDisplay::write(&blockDimZ, "cuLaunchKernel", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sharedMemBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&sharedMemBytes, "cuLaunchKernel", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuLaunchKernel", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(kernelParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(&kernelParams, "cuLaunchKernel", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(extra), ": ").as_bytes())?;
    crate::CudaDisplay::write(&extra, "cuLaunchKernel", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuLaunchKernelEx(
    writer: &mut (impl std::io::Write + ?Sized),
    config: *const cuda_types::cuda::CUlaunchConfig,
    f: cuda_types::cuda::CUfunction,
    kernelParams: *mut *mut ::core::ffi::c_void,
    extra: *mut *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(config), ": ").as_bytes())?;
    crate::CudaDisplay::write(&config, "cuLaunchKernelEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(f), ": ").as_bytes())?;
    crate::CudaDisplay::write(&f, "cuLaunchKernelEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(kernelParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(&kernelParams, "cuLaunchKernelEx", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(extra), ": ").as_bytes())?;
    crate::CudaDisplay::write(&extra, "cuLaunchKernelEx", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuLaunchHostFunc(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    fn_: cuda_types::cuda::CUhostFn,
    userData: *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuLaunchHostFunc", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(fn_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&fn_, "cuLaunchHostFunc", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(userData), ": ").as_bytes())?;
    crate::CudaDisplay::write(&userData, "cuLaunchHostFunc", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphicsMapResources(
    writer: &mut (impl std::io::Write + ?Sized),
    count: ::core::ffi::c_uint,
    resources: *mut cuda_types::cuda::CUgraphicsResource,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "cuGraphicsMapResources", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(resources), ": ").as_bytes())?;
    crate::CudaDisplay::write(&resources, "cuGraphicsMapResources", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuGraphicsMapResources", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphicsUnmapResources(
    writer: &mut (impl std::io::Write + ?Sized),
    count: ::core::ffi::c_uint,
    resources: *mut cuda_types::cuda::CUgraphicsResource,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "cuGraphicsUnmapResources", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(resources), ": ").as_bytes())?;
    crate::CudaDisplay::write(&resources, "cuGraphicsUnmapResources", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuGraphicsUnmapResources", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamWriteValue32(
    writer: &mut (impl std::io::Write + ?Sized),
    stream: cuda_types::cuda::CUstream,
    addr: cuda_types::cuda::CUdeviceptr,
    value: cuda_types::cuda::cuuint32_t,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&stream, "cuStreamWriteValue32", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(addr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&addr, "cuStreamWriteValue32", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cuStreamWriteValue32", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuStreamWriteValue32", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamWaitValue32(
    writer: &mut (impl std::io::Write + ?Sized),
    stream: cuda_types::cuda::CUstream,
    addr: cuda_types::cuda::CUdeviceptr,
    value: cuda_types::cuda::cuuint32_t,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&stream, "cuStreamWaitValue32", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(addr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&addr, "cuStreamWaitValue32", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cuStreamWaitValue32", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuStreamWaitValue32", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamWriteValue64(
    writer: &mut (impl std::io::Write + ?Sized),
    stream: cuda_types::cuda::CUstream,
    addr: cuda_types::cuda::CUdeviceptr,
    value: cuda_types::cuda::cuuint64_t,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&stream, "cuStreamWriteValue64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(addr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&addr, "cuStreamWriteValue64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cuStreamWriteValue64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuStreamWriteValue64", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamWaitValue64(
    writer: &mut (impl std::io::Write + ?Sized),
    stream: cuda_types::cuda::CUstream,
    addr: cuda_types::cuda::CUdeviceptr,
    value: cuda_types::cuda::cuuint64_t,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&stream, "cuStreamWaitValue64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(addr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&addr, "cuStreamWaitValue64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cuStreamWaitValue64", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuStreamWaitValue64", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamBatchMemOp(
    writer: &mut (impl std::io::Write + ?Sized),
    stream: cuda_types::cuda::CUstream,
    count: ::core::ffi::c_uint,
    paramArray: *mut cuda_types::cuda::CUstreamBatchMemOpParams,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&stream, "cuStreamBatchMemOp", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "cuStreamBatchMemOp", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(paramArray), ": ").as_bytes())?;
    writer.write_all(b"[")?;
    for i in 0..count {
        if i != 0 {
            writer.write_all(b", ")?;
        }
        crate::CudaDisplay::write(
            unsafe { &*paramArray.add(i as usize) },
            "cuStreamBatchMemOp",
            arg_idx,
            writer,
        )?;
    }
    writer.write_all(b"]")?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuStreamBatchMemOp", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamWriteValue32_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    stream: cuda_types::cuda::CUstream,
    addr: cuda_types::cuda::CUdeviceptr,
    value: cuda_types::cuda::cuuint32_t,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&stream, "cuStreamWriteValue32_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(addr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&addr, "cuStreamWriteValue32_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cuStreamWriteValue32_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuStreamWriteValue32_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamWaitValue32_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    stream: cuda_types::cuda::CUstream,
    addr: cuda_types::cuda::CUdeviceptr,
    value: cuda_types::cuda::cuuint32_t,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&stream, "cuStreamWaitValue32_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(addr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&addr, "cuStreamWaitValue32_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cuStreamWaitValue32_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuStreamWaitValue32_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamWriteValue64_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    stream: cuda_types::cuda::CUstream,
    addr: cuda_types::cuda::CUdeviceptr,
    value: cuda_types::cuda::cuuint64_t,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&stream, "cuStreamWriteValue64_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(addr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&addr, "cuStreamWriteValue64_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cuStreamWriteValue64_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuStreamWriteValue64_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamWaitValue64_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    stream: cuda_types::cuda::CUstream,
    addr: cuda_types::cuda::CUdeviceptr,
    value: cuda_types::cuda::cuuint64_t,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&stream, "cuStreamWaitValue64_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(addr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&addr, "cuStreamWaitValue64_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cuStreamWaitValue64_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuStreamWaitValue64_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamBatchMemOp_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    stream: cuda_types::cuda::CUstream,
    count: ::core::ffi::c_uint,
    paramArray: *mut cuda_types::cuda::CUstreamBatchMemOpParams,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&stream, "cuStreamBatchMemOp_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "cuStreamBatchMemOp_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(paramArray), ": ").as_bytes())?;
    writer.write_all(b"[")?;
    for i in 0..count {
        if i != 0 {
            writer.write_all(b", ")?;
        }
        crate::CudaDisplay::write(
            unsafe { &*paramArray.add(i as usize) },
            "cuStreamBatchMemOp_ptsz",
            arg_idx,
            writer,
        )?;
    }
    writer.write_all(b"]")?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuStreamBatchMemOp_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamWriteValue32_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    stream: cuda_types::cuda::CUstream,
    addr: cuda_types::cuda::CUdeviceptr,
    value: cuda_types::cuda::cuuint32_t,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&stream, "cuStreamWriteValue32_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(addr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&addr, "cuStreamWriteValue32_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cuStreamWriteValue32_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuStreamWriteValue32_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamWaitValue32_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    stream: cuda_types::cuda::CUstream,
    addr: cuda_types::cuda::CUdeviceptr,
    value: cuda_types::cuda::cuuint32_t,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&stream, "cuStreamWaitValue32_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(addr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&addr, "cuStreamWaitValue32_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cuStreamWaitValue32_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuStreamWaitValue32_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamWriteValue64_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    stream: cuda_types::cuda::CUstream,
    addr: cuda_types::cuda::CUdeviceptr,
    value: cuda_types::cuda::cuuint64_t,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&stream, "cuStreamWriteValue64_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(addr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&addr, "cuStreamWriteValue64_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cuStreamWriteValue64_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuStreamWriteValue64_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamWaitValue64_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    stream: cuda_types::cuda::CUstream,
    addr: cuda_types::cuda::CUdeviceptr,
    value: cuda_types::cuda::cuuint64_t,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&stream, "cuStreamWaitValue64_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(addr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&addr, "cuStreamWaitValue64_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "cuStreamWaitValue64_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuStreamWaitValue64_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamBatchMemOp_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    stream: cuda_types::cuda::CUstream,
    count: ::core::ffi::c_uint,
    paramArray: *mut cuda_types::cuda::CUstreamBatchMemOpParams,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&stream, "cuStreamBatchMemOp_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "cuStreamBatchMemOp_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(paramArray), ": ").as_bytes())?;
    writer.write_all(b"[")?;
    for i in 0..count {
        if i != 0 {
            writer.write_all(b", ")?;
        }
        crate::CudaDisplay::write(
            unsafe { &*paramArray.add(i as usize) },
            "cuStreamBatchMemOp_v2",
            arg_idx,
            writer,
        )?;
    }
    writer.write_all(b"]")?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuStreamBatchMemOp_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemPrefetchAsync(
    writer: &mut (impl std::io::Write + ?Sized),
    devPtr: cuda_types::cuda::CUdeviceptr,
    count: usize,
    dstDevice: cuda_types::cuda::CUdevice,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(devPtr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&devPtr, "cuMemPrefetchAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "cuMemPrefetchAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dstDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstDevice, "cuMemPrefetchAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemPrefetchAsync", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemPrefetchAsync_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    devPtr: cuda_types::cuda::CUdeviceptr,
    count: usize,
    location: cuda_types::cuda::CUmemLocation,
    flags: ::core::ffi::c_uint,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(devPtr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&devPtr, "cuMemPrefetchAsync_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "cuMemPrefetchAsync_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(location), ": ").as_bytes())?;
    crate::CudaDisplay::write(&location, "cuMemPrefetchAsync_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuMemPrefetchAsync_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemPrefetchAsync_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuLaunchCooperativeKernel(
    writer: &mut (impl std::io::Write + ?Sized),
    f: cuda_types::cuda::CUfunction,
    gridDimX: ::core::ffi::c_uint,
    gridDimY: ::core::ffi::c_uint,
    gridDimZ: ::core::ffi::c_uint,
    blockDimX: ::core::ffi::c_uint,
    blockDimY: ::core::ffi::c_uint,
    blockDimZ: ::core::ffi::c_uint,
    sharedMemBytes: ::core::ffi::c_uint,
    hStream: cuda_types::cuda::CUstream,
    kernelParams: *mut *mut ::core::ffi::c_void,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(f), ": ").as_bytes())?;
    crate::CudaDisplay::write(&f, "cuLaunchCooperativeKernel", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gridDimX), ": ").as_bytes())?;
    crate::CudaDisplay::write(&gridDimX, "cuLaunchCooperativeKernel", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gridDimY), ": ").as_bytes())?;
    crate::CudaDisplay::write(&gridDimY, "cuLaunchCooperativeKernel", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gridDimZ), ": ").as_bytes())?;
    crate::CudaDisplay::write(&gridDimZ, "cuLaunchCooperativeKernel", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(blockDimX), ": ").as_bytes())?;
    crate::CudaDisplay::write(&blockDimX, "cuLaunchCooperativeKernel", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(blockDimY), ": ").as_bytes())?;
    crate::CudaDisplay::write(&blockDimY, "cuLaunchCooperativeKernel", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(blockDimZ), ": ").as_bytes())?;
    crate::CudaDisplay::write(&blockDimZ, "cuLaunchCooperativeKernel", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sharedMemBytes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sharedMemBytes,
        "cuLaunchCooperativeKernel",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuLaunchCooperativeKernel", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(kernelParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &kernelParams,
        "cuLaunchCooperativeKernel",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuSignalExternalSemaphoresAsync(
    writer: &mut (impl std::io::Write + ?Sized),
    extSemArray: *const cuda_types::cuda::CUexternalSemaphore,
    paramsArray: *const cuda_types::cuda::CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS,
    numExtSems: ::core::ffi::c_uint,
    stream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(extSemArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &extSemArray,
        "cuSignalExternalSemaphoresAsync",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(paramsArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &paramsArray,
        "cuSignalExternalSemaphoresAsync",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numExtSems), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numExtSems,
        "cuSignalExternalSemaphoresAsync",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &stream,
        "cuSignalExternalSemaphoresAsync",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuWaitExternalSemaphoresAsync(
    writer: &mut (impl std::io::Write + ?Sized),
    extSemArray: *const cuda_types::cuda::CUexternalSemaphore,
    paramsArray: *const cuda_types::cuda::CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS,
    numExtSems: ::core::ffi::c_uint,
    stream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(extSemArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &extSemArray,
        "cuWaitExternalSemaphoresAsync",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(paramsArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &paramsArray,
        "cuWaitExternalSemaphoresAsync",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numExtSems), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numExtSems,
        "cuWaitExternalSemaphoresAsync",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &stream,
        "cuWaitExternalSemaphoresAsync",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuStreamBeginCapture(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamBeginCapture", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamBeginCapture_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamBeginCapture_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamBeginCapture_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    mode: cuda_types::cuda::CUstreamCaptureMode,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamBeginCapture_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mode, "cuStreamBeginCapture_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamBeginCaptureToGraph(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    hGraph: cuda_types::cuda::CUgraph,
    dependencies: *const cuda_types::cuda::CUgraphNode,
    dependencyData: *const cuda_types::cuda::CUgraphEdgeData,
    numDependencies: usize,
    mode: cuda_types::cuda::CUstreamCaptureMode,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamBeginCaptureToGraph", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuStreamBeginCaptureToGraph", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dependencies,
        "cuStreamBeginCaptureToGraph",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencyData), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dependencyData,
        "cuStreamBeginCaptureToGraph",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDependencies,
        "cuStreamBeginCaptureToGraph",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mode, "cuStreamBeginCaptureToGraph", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamEndCapture(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    phGraph: *mut cuda_types::cuda::CUgraph,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamEndCapture", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(phGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phGraph, "cuStreamEndCapture", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamIsCapturing(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    captureStatus: *mut cuda_types::cuda::CUstreamCaptureStatus,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamIsCapturing", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(captureStatus), ": ").as_bytes())?;
    crate::CudaDisplay::write(&captureStatus, "cuStreamIsCapturing", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamGetCaptureInfo(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    captureStatus_out: *mut cuda_types::cuda::CUstreamCaptureStatus,
    id_out: *mut cuda_types::cuda::cuuint64_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamGetCaptureInfo", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(captureStatus_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &captureStatus_out,
        "cuStreamGetCaptureInfo",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(id_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(&id_out, "cuStreamGetCaptureInfo", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamGetCaptureInfo_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    captureStatus_out: *mut cuda_types::cuda::CUstreamCaptureStatus,
    id_out: *mut cuda_types::cuda::cuuint64_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamGetCaptureInfo_ptsz", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(captureStatus_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &captureStatus_out,
        "cuStreamGetCaptureInfo_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(id_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(&id_out, "cuStreamGetCaptureInfo_ptsz", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamGetCaptureInfo_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    captureStatus_out: *mut cuda_types::cuda::CUstreamCaptureStatus,
    id_out: *mut cuda_types::cuda::cuuint64_t,
    graph_out: *mut cuda_types::cuda::CUgraph,
    dependencies_out: *mut *const cuda_types::cuda::CUgraphNode,
    numDependencies_out: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamGetCaptureInfo_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(captureStatus_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &captureStatus_out,
        "cuStreamGetCaptureInfo_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(id_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(&id_out, "cuStreamGetCaptureInfo_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(graph_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(&graph_out, "cuStreamGetCaptureInfo_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencies_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dependencies_out,
        "cuStreamGetCaptureInfo_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDependencies_out,
        "cuStreamGetCaptureInfo_v2",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuStreamGetCaptureInfo_v3(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    captureStatus_out: *mut cuda_types::cuda::CUstreamCaptureStatus,
    id_out: *mut cuda_types::cuda::cuuint64_t,
    graph_out: *mut cuda_types::cuda::CUgraph,
    dependencies_out: *mut *const cuda_types::cuda::CUgraphNode,
    edgeData_out: *mut *const cuda_types::cuda::CUgraphEdgeData,
    numDependencies_out: *mut usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuStreamGetCaptureInfo_v3", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(captureStatus_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &captureStatus_out,
        "cuStreamGetCaptureInfo_v3",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(id_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(&id_out, "cuStreamGetCaptureInfo_v3", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(graph_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(&graph_out, "cuStreamGetCaptureInfo_v3", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencies_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dependencies_out,
        "cuStreamGetCaptureInfo_v3",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(edgeData_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &edgeData_out,
        "cuStreamGetCaptureInfo_v3",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDependencies_out,
        "cuStreamGetCaptureInfo_v3",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphAddKernelNode(
    writer: &mut (impl std::io::Write + ?Sized),
    phGraphNode: *mut cuda_types::cuda::CUgraphNode,
    hGraph: cuda_types::cuda::CUgraph,
    dependencies: *const cuda_types::cuda::CUgraphNode,
    numDependencies: usize,
    nodeParams: *const cuda_types::cuda::CUDA_KERNEL_NODE_PARAMS_v1,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phGraphNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phGraphNode, "cuGraphAddKernelNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphAddKernelNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dependencies, "cuGraphAddKernelNode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDependencies,
        "cuGraphAddKernelNode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nodeParams, "cuGraphAddKernelNode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphKernelNodeGetParams(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::cuda::CUgraphNode,
    nodeParams: *mut cuda_types::cuda::CUDA_KERNEL_NODE_PARAMS_v1,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hNode, "cuGraphKernelNodeGetParams", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nodeParams,
        "cuGraphKernelNodeGetParams",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphKernelNodeSetParams(
    writer: &mut (impl std::io::Write + ?Sized),
    hNode: cuda_types::cuda::CUgraphNode,
    nodeParams: *const cuda_types::cuda::CUDA_KERNEL_NODE_PARAMS_v1,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hNode, "cuGraphKernelNodeSetParams", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nodeParams,
        "cuGraphKernelNodeSetParams",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphExecKernelNodeSetParams(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraphExec: cuda_types::cuda::CUgraphExec,
    hNode: cuda_types::cuda::CUgraphNode,
    nodeParams: *const cuda_types::cuda::CUDA_KERNEL_NODE_PARAMS_v1,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraphExec), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hGraphExec,
        "cuGraphExecKernelNodeSetParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hNode,
        "cuGraphExecKernelNodeSetParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nodeParams,
        "cuGraphExecKernelNodeSetParams",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphInstantiateWithParams(
    writer: &mut (impl std::io::Write + ?Sized),
    phGraphExec: *mut cuda_types::cuda::CUgraphExec,
    hGraph: cuda_types::cuda::CUgraph,
    instantiateParams: *mut cuda_types::cuda::CUDA_GRAPH_INSTANTIATE_PARAMS,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phGraphExec), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &phGraphExec,
        "cuGraphInstantiateWithParams",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphInstantiateWithParams", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(instantiateParams), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &instantiateParams,
        "cuGraphInstantiateWithParams",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphExecUpdate(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraphExec: cuda_types::cuda::CUgraphExec,
    hGraph: cuda_types::cuda::CUgraph,
    hErrorNode_out: *mut cuda_types::cuda::CUgraphNode,
    updateResult_out: *mut cuda_types::cuda::CUgraphExecUpdateResult,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraphExec), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraphExec, "cuGraphExecUpdate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphExecUpdate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hErrorNode_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hErrorNode_out, "cuGraphExecUpdate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(updateResult_out), ": ").as_bytes())?;
    crate::CudaDisplay::write(&updateResult_out, "cuGraphExecUpdate", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphUpload(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraph: cuda_types::cuda::CUgraphExec,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphUpload", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuGraphUpload", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphLaunch(
    writer: &mut (impl std::io::Write + ?Sized),
    hGraph: cuda_types::cuda::CUgraphExec,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphLaunch", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuGraphLaunch", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamCopyAttributes(
    writer: &mut (impl std::io::Write + ?Sized),
    dstStream: cuda_types::cuda::CUstream,
    srcStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dstStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dstStream, "cuStreamCopyAttributes", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(srcStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&srcStream, "cuStreamCopyAttributes", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuIpcOpenMemHandle(
    writer: &mut (impl std::io::Write + ?Sized),
    pdptr: *mut cuda_types::cuda::CUdeviceptr,
    handle: cuda_types::cuda::CUipcMemHandle,
    Flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pdptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pdptr, "cuIpcOpenMemHandle", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(handle), ": ").as_bytes())?;
    crate::CudaDisplay::write(&handle, "cuIpcOpenMemHandle", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Flags, "cuIpcOpenMemHandle", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphInstantiate(
    writer: &mut (impl std::io::Write + ?Sized),
    phGraphExec: *mut cuda_types::cuda::CUgraphExec,
    hGraph: cuda_types::cuda::CUgraph,
    phErrorNode: *mut cuda_types::cuda::CUgraphNode,
    logBuffer: *mut ::core::ffi::c_char,
    bufferSize: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phGraphExec), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phGraphExec, "cuGraphInstantiate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphInstantiate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(phErrorNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phErrorNode, "cuGraphInstantiate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(logBuffer), ": ").as_bytes())?;
    crate::CudaDisplay::write(&logBuffer, "cuGraphInstantiate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bufferSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&bufferSize, "cuGraphInstantiate", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphInstantiate_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    phGraphExec: *mut cuda_types::cuda::CUgraphExec,
    hGraph: cuda_types::cuda::CUgraph,
    phErrorNode: *mut cuda_types::cuda::CUgraphNode,
    logBuffer: *mut ::core::ffi::c_char,
    bufferSize: usize,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phGraphExec), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phGraphExec, "cuGraphInstantiate_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hGraph), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hGraph, "cuGraphInstantiate_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(phErrorNode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phErrorNode, "cuGraphInstantiate_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(logBuffer), ": ").as_bytes())?;
    crate::CudaDisplay::write(&logBuffer, "cuGraphInstantiate_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bufferSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&bufferSize, "cuGraphInstantiate_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemMapArrayAsync(
    writer: &mut (impl std::io::Write + ?Sized),
    mapInfoList: *mut cuda_types::cuda::CUarrayMapInfo,
    count: ::core::ffi::c_uint,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(mapInfoList), ": ").as_bytes())?;
    writer.write_all(b"[")?;
    for i in 0..count {
        if i != 0 {
            writer.write_all(b", ")?;
        }
        crate::CudaDisplay::write(
            unsafe { &*mapInfoList.add(i as usize) },
            "cuMemMapArrayAsync",
            arg_idx,
            writer,
        )?;
    }
    writer.write_all(b"]")?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "cuMemMapArrayAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemMapArrayAsync", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemFreeAsync(
    writer: &mut (impl std::io::Write + ?Sized),
    dptr: cuda_types::cuda::CUdeviceptr,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuMemFreeAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemFreeAsync", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemAllocAsync(
    writer: &mut (impl std::io::Write + ?Sized),
    dptr: *mut cuda_types::cuda::CUdeviceptr,
    bytesize: usize,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuMemAllocAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bytesize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&bytesize, "cuMemAllocAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemAllocAsync", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuMemAllocFromPoolAsync(
    writer: &mut (impl std::io::Write + ?Sized),
    dptr: *mut cuda_types::cuda::CUdeviceptr,
    bytesize: usize,
    pool: cuda_types::cuda::CUmemoryPool,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuMemAllocFromPoolAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bytesize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&bytesize, "cuMemAllocFromPoolAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pool), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pool, "cuMemAllocFromPoolAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuMemAllocFromPoolAsync", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuStreamUpdateCaptureDependencies(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    dependencies: *mut cuda_types::cuda::CUgraphNode,
    numDependencies: usize,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hStream,
        "cuStreamUpdateCaptureDependencies",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dependencies,
        "cuStreamUpdateCaptureDependencies",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDependencies,
        "cuStreamUpdateCaptureDependencies",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &flags,
        "cuStreamUpdateCaptureDependencies",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuStreamUpdateCaptureDependencies_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    hStream: cuda_types::cuda::CUstream,
    dependencies: *mut cuda_types::cuda::CUgraphNode,
    dependencyData: *const cuda_types::cuda::CUgraphEdgeData,
    numDependencies: usize,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hStream,
        "cuStreamUpdateCaptureDependencies_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dependencies,
        "cuStreamUpdateCaptureDependencies_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dependencyData), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dependencyData,
        "cuStreamUpdateCaptureDependencies_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDependencies), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDependencies,
        "cuStreamUpdateCaptureDependencies_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &flags,
        "cuStreamUpdateCaptureDependencies_v2",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuMemBatchDecompressAsync(
    writer: &mut (impl std::io::Write + ?Sized),
    paramsArray: *mut cuda_types::cuda::CUmemDecompressParams,
    count: usize,
    flags: ::core::ffi::c_uint,
    errorIndex: *mut usize,
    stream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(paramsArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &paramsArray,
        "cuMemBatchDecompressAsync",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "cuMemBatchDecompressAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuMemBatchDecompressAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(errorIndex), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &errorIndex,
        "cuMemBatchDecompressAsync",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&stream, "cuMemBatchDecompressAsync", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGetProcAddress(
    writer: &mut (impl std::io::Write + ?Sized),
    symbol: *const ::core::ffi::c_char,
    pfn: *mut *mut ::core::ffi::c_void,
    cudaVersion: ::core::ffi::c_int,
    flags: cuda_types::cuda::cuuint64_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(symbol), ": ").as_bytes())?;
    crate::CudaDisplay::write(&symbol, "cuGetProcAddress", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pfn), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pfn, "cuGetProcAddress", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cudaVersion), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cudaVersion, "cuGetProcAddress", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuGetProcAddress", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCheckpointProcessGetRestoreThreadId(
    writer: &mut (impl std::io::Write + ?Sized),
    pid: ::core::ffi::c_int,
    tid: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pid), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pid,
        "cuCheckpointProcessGetRestoreThreadId",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(tid), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &tid,
        "cuCheckpointProcessGetRestoreThreadId",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuCheckpointProcessGetState(
    writer: &mut (impl std::io::Write + ?Sized),
    pid: ::core::ffi::c_int,
    state: *mut cuda_types::cuda::CUprocessState,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pid), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pid, "cuCheckpointProcessGetState", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(state), ": ").as_bytes())?;
    crate::CudaDisplay::write(&state, "cuCheckpointProcessGetState", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCheckpointProcessLock(
    writer: &mut (impl std::io::Write + ?Sized),
    pid: ::core::ffi::c_int,
    args: *mut cuda_types::cuda::CUcheckpointLockArgs,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pid), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pid, "cuCheckpointProcessLock", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(args), ": ").as_bytes())?;
    crate::CudaDisplay::write(&args, "cuCheckpointProcessLock", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCheckpointProcessCheckpoint(
    writer: &mut (impl std::io::Write + ?Sized),
    pid: ::core::ffi::c_int,
    args: *mut cuda_types::cuda::CUcheckpointCheckpointArgs,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pid), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pid, "cuCheckpointProcessCheckpoint", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(args), ": ").as_bytes())?;
    crate::CudaDisplay::write(&args, "cuCheckpointProcessCheckpoint", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCheckpointProcessRestore(
    writer: &mut (impl std::io::Write + ?Sized),
    pid: ::core::ffi::c_int,
    args: *mut cuda_types::cuda::CUcheckpointRestoreArgs,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pid), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pid, "cuCheckpointProcessRestore", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(args), ": ").as_bytes())?;
    crate::CudaDisplay::write(&args, "cuCheckpointProcessRestore", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuCheckpointProcessUnlock(
    writer: &mut (impl std::io::Write + ?Sized),
    pid: ::core::ffi::c_int,
    args: *mut cuda_types::cuda::CUcheckpointUnlockArgs,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pid), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pid, "cuCheckpointProcessUnlock", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(args), ": ").as_bytes())?;
    crate::CudaDisplay::write(&args, "cuCheckpointProcessUnlock", arg_idx, writer)?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::cuda::CUoutput_mode_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUoutput_mode_enum::CU_OUT_KEY_VALUE_PAIR => {
                writer.write_all(stringify!(CU_OUT_KEY_VALUE_PAIR).as_bytes())
            }
            &cuda_types::cuda::CUoutput_mode_enum::CU_OUT_CSV => {
                writer.write_all(stringify!(CU_OUT_CSV).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cuProfilerInitialize(
    writer: &mut (impl std::io::Write + ?Sized),
    configFile: *const ::core::ffi::c_char,
    outputFile: *const ::core::ffi::c_char,
    outputMode: cuda_types::cuda::CUoutput_mode,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(configFile), ": ").as_bytes())?;
    crate::CudaDisplay::write(&configFile, "cuProfilerInitialize", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(outputFile), ": ").as_bytes())?;
    crate::CudaDisplay::write(&outputFile, "cuProfilerInitialize", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(outputMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&outputMode, "cuProfilerInitialize", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuProfilerStart(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
pub fn write_cuProfilerStop(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
pub fn write_cuGraphicsGLRegisterBuffer(
    writer: &mut (impl std::io::Write + ?Sized),
    pCudaResource: *mut cuda_types::cuda::CUgraphicsResource,
    buffer: cuda_types::cuda::GLuint,
    Flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pCudaResource), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pCudaResource,
        "cuGraphicsGLRegisterBuffer",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(buffer), ": ").as_bytes())?;
    crate::CudaDisplay::write(&buffer, "cuGraphicsGLRegisterBuffer", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Flags, "cuGraphicsGLRegisterBuffer", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGraphicsGLRegisterImage(
    writer: &mut (impl std::io::Write + ?Sized),
    pCudaResource: *mut cuda_types::cuda::CUgraphicsResource,
    image: cuda_types::cuda::GLuint,
    target: cuda_types::cuda::GLenum,
    Flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pCudaResource), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pCudaResource,
        "cuGraphicsGLRegisterImage",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(image), ": ").as_bytes())?;
    crate::CudaDisplay::write(&image, "cuGraphicsGLRegisterImage", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(target), ": ").as_bytes())?;
    crate::CudaDisplay::write(&target, "cuGraphicsGLRegisterImage", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Flags, "cuGraphicsGLRegisterImage", arg_idx, writer)?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::cuda::CUGLDeviceList_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUGLDeviceList_enum::CU_GL_DEVICE_LIST_ALL => {
                writer.write_all(stringify!(CU_GL_DEVICE_LIST_ALL).as_bytes())
            }
            &cuda_types::cuda::CUGLDeviceList_enum::CU_GL_DEVICE_LIST_CURRENT_FRAME => {
                writer.write_all(stringify!(CU_GL_DEVICE_LIST_CURRENT_FRAME).as_bytes())
            }
            &cuda_types::cuda::CUGLDeviceList_enum::CU_GL_DEVICE_LIST_NEXT_FRAME => {
                writer.write_all(stringify!(CU_GL_DEVICE_LIST_NEXT_FRAME).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUGLmap_flags_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUGLmap_flags_enum::CU_GL_MAP_RESOURCE_FLAGS_NONE => {
                writer.write_all(stringify!(CU_GL_MAP_RESOURCE_FLAGS_NONE).as_bytes())
            }
            &cuda_types::cuda::CUGLmap_flags_enum::CU_GL_MAP_RESOURCE_FLAGS_READ_ONLY => {
                writer
                    .write_all(stringify!(CU_GL_MAP_RESOURCE_FLAGS_READ_ONLY).as_bytes())
            }
            &cuda_types::cuda::CUGLmap_flags_enum::CU_GL_MAP_RESOURCE_FLAGS_WRITE_DISCARD => {
                writer
                    .write_all(
                        stringify!(CU_GL_MAP_RESOURCE_FLAGS_WRITE_DISCARD).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_cuGLCtxCreate_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    pCtx: *mut cuda_types::cuda::CUcontext,
    Flags: ::core::ffi::c_uint,
    device: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pCtx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pCtx, "cuGLCtxCreate_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Flags, "cuGLCtxCreate_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "cuGLCtxCreate_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGLInit(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
pub fn write_cuGLRegisterBufferObject(
    writer: &mut (impl std::io::Write + ?Sized),
    buffer: cuda_types::cuda::GLuint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(buffer), ": ").as_bytes())?;
    crate::CudaDisplay::write(&buffer, "cuGLRegisterBufferObject", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGLMapBufferObject_v2_ptds(
    writer: &mut (impl std::io::Write + ?Sized),
    dptr: *mut cuda_types::cuda::CUdeviceptr,
    size: *mut usize,
    buffer: cuda_types::cuda::GLuint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuGLMapBufferObject_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(&size, "cuGLMapBufferObject_v2_ptds", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(buffer), ": ").as_bytes())?;
    crate::CudaDisplay::write(&buffer, "cuGLMapBufferObject_v2_ptds", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGLUnmapBufferObject(
    writer: &mut (impl std::io::Write + ?Sized),
    buffer: cuda_types::cuda::GLuint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(buffer), ": ").as_bytes())?;
    crate::CudaDisplay::write(&buffer, "cuGLUnmapBufferObject", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGLUnregisterBufferObject(
    writer: &mut (impl std::io::Write + ?Sized),
    buffer: cuda_types::cuda::GLuint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(buffer), ": ").as_bytes())?;
    crate::CudaDisplay::write(&buffer, "cuGLUnregisterBufferObject", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGLSetBufferObjectMapFlags(
    writer: &mut (impl std::io::Write + ?Sized),
    buffer: cuda_types::cuda::GLuint,
    Flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(buffer), ": ").as_bytes())?;
    crate::CudaDisplay::write(&buffer, "cuGLSetBufferObjectMapFlags", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Flags, "cuGLSetBufferObjectMapFlags", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGLMapBufferObjectAsync_v2_ptsz(
    writer: &mut (impl std::io::Write + ?Sized),
    dptr: *mut cuda_types::cuda::CUdeviceptr,
    size: *mut usize,
    buffer: cuda_types::cuda::GLuint,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dptr,
        "cuGLMapBufferObjectAsync_v2_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &size,
        "cuGLMapBufferObjectAsync_v2_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(buffer), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &buffer,
        "cuGLMapBufferObjectAsync_v2_ptsz",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &hStream,
        "cuGLMapBufferObjectAsync_v2_ptsz",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGLUnmapBufferObjectAsync(
    writer: &mut (impl std::io::Write + ?Sized),
    buffer: cuda_types::cuda::GLuint,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(buffer), ": ").as_bytes())?;
    crate::CudaDisplay::write(&buffer, "cuGLUnmapBufferObjectAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuGLUnmapBufferObjectAsync", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGLMapBufferObject_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    dptr: *mut cuda_types::cuda::CUdeviceptr,
    size: *mut usize,
    buffer: cuda_types::cuda::GLuint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuGLMapBufferObject_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(&size, "cuGLMapBufferObject_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(buffer), ": ").as_bytes())?;
    crate::CudaDisplay::write(&buffer, "cuGLMapBufferObject_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGLMapBufferObjectAsync_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    dptr: *mut cuda_types::cuda::CUdeviceptr,
    size: *mut usize,
    buffer: cuda_types::cuda::GLuint,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuGLMapBufferObjectAsync_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(&size, "cuGLMapBufferObjectAsync_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(buffer), ": ").as_bytes())?;
    crate::CudaDisplay::write(&buffer, "cuGLMapBufferObjectAsync_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuGLMapBufferObjectAsync_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGLCtxCreate(
    writer: &mut (impl std::io::Write + ?Sized),
    pCtx: *mut cuda_types::cuda::CUcontext,
    Flags: ::core::ffi::c_uint,
    device: cuda_types::cuda::CUdevice,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pCtx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pCtx, "cuGLCtxCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(Flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&Flags, "cuGLCtxCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "cuGLCtxCreate", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGLMapBufferObject(
    writer: &mut (impl std::io::Write + ?Sized),
    dptr: *mut cuda_types::cuda::CUdeviceptr_v1,
    size: *mut ::core::ffi::c_uint,
    buffer: cuda_types::cuda::GLuint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuGLMapBufferObject", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(&size, "cuGLMapBufferObject", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(buffer), ": ").as_bytes())?;
    crate::CudaDisplay::write(&buffer, "cuGLMapBufferObject", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuGLMapBufferObjectAsync(
    writer: &mut (impl std::io::Write + ?Sized),
    dptr: *mut cuda_types::cuda::CUdeviceptr_v1,
    size: *mut ::core::ffi::c_uint,
    buffer: cuda_types::cuda::GLuint,
    hStream: cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(dptr), ": ").as_bytes())?;
    crate::CudaDisplay::write(&dptr, "cuGLMapBufferObjectAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(&size, "cuGLMapBufferObjectAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(buffer), ": ").as_bytes())?;
    crate::CudaDisplay::write(&buffer, "cuGLMapBufferObjectAsync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hStream, "cuGLMapBufferObjectAsync", arg_idx, writer)?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::cuda::CUeglFrameType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUeglFrameType_enum::CU_EGL_FRAME_TYPE_ARRAY => {
                writer.write_all(stringify!(CU_EGL_FRAME_TYPE_ARRAY).as_bytes())
            }
            &cuda_types::cuda::CUeglFrameType_enum::CU_EGL_FRAME_TYPE_PITCH => {
                writer.write_all(stringify!(CU_EGL_FRAME_TYPE_PITCH).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUeglResourceLocationFlags_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUeglResourceLocationFlags_enum::CU_EGL_RESOURCE_LOCATION_SYSMEM => {
                writer.write_all(stringify!(CU_EGL_RESOURCE_LOCATION_SYSMEM).as_bytes())
            }
            &cuda_types::cuda::CUeglResourceLocationFlags_enum::CU_EGL_RESOURCE_LOCATION_VIDMEM => {
                writer.write_all(stringify!(CU_EGL_RESOURCE_LOCATION_VIDMEM).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUeglColorFormat_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YUV420_PLANAR => {
                writer
                    .write_all(stringify!(CU_EGL_COLOR_FORMAT_YUV420_PLANAR).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YUV420_SEMIPLANAR => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_YUV420_SEMIPLANAR).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YUV422_PLANAR => {
                writer
                    .write_all(stringify!(CU_EGL_COLOR_FORMAT_YUV422_PLANAR).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YUV422_SEMIPLANAR => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_YUV422_SEMIPLANAR).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_RGB => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_RGB).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BGR => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BGR).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_ARGB => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_ARGB).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_RGBA => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_RGBA).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_L => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_L).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_R => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_R).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YUV444_PLANAR => {
                writer
                    .write_all(stringify!(CU_EGL_COLOR_FORMAT_YUV444_PLANAR).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YUV444_SEMIPLANAR => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_YUV444_SEMIPLANAR).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YUYV_422 => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_YUYV_422).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_UYVY_422 => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_UYVY_422).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_ABGR => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_ABGR).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BGRA => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BGRA).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_A => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_A).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_RG => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_RG).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_AYUV => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_AYUV).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YVU444_SEMIPLANAR => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_YVU444_SEMIPLANAR).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YVU422_SEMIPLANAR => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_YVU422_SEMIPLANAR).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YVU420_SEMIPLANAR => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_YVU420_SEMIPLANAR).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_Y10V10U10_444_SEMIPLANAR => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_Y10V10U10_444_SEMIPLANAR)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_Y10V10U10_420_SEMIPLANAR => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_Y10V10U10_420_SEMIPLANAR)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_Y12V12U12_444_SEMIPLANAR => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_Y12V12U12_444_SEMIPLANAR)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_Y12V12U12_420_SEMIPLANAR => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_Y12V12U12_420_SEMIPLANAR)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_VYUY_ER => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_VYUY_ER).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_UYVY_ER => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_UYVY_ER).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YUYV_ER => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_YUYV_ER).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YVYU_ER => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_YVYU_ER).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YUV_ER => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_YUV_ER).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YUVA_ER => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_YUVA_ER).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_AYUV_ER => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_AYUV_ER).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YUV444_PLANAR_ER => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_YUV444_PLANAR_ER).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YUV422_PLANAR_ER => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_YUV422_PLANAR_ER).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YUV420_PLANAR_ER => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_YUV420_PLANAR_ER).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YUV444_SEMIPLANAR_ER => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_YUV444_SEMIPLANAR_ER).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YUV422_SEMIPLANAR_ER => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_YUV422_SEMIPLANAR_ER).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YUV420_SEMIPLANAR_ER => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_YUV420_SEMIPLANAR_ER).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YVU444_PLANAR_ER => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_YVU444_PLANAR_ER).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YVU422_PLANAR_ER => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_YVU422_PLANAR_ER).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YVU420_PLANAR_ER => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_YVU420_PLANAR_ER).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YVU444_SEMIPLANAR_ER => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_YVU444_SEMIPLANAR_ER).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YVU422_SEMIPLANAR_ER => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_YVU422_SEMIPLANAR_ER).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YVU420_SEMIPLANAR_ER => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_YVU420_SEMIPLANAR_ER).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER_RGGB => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER_RGGB).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER_BGGR => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER_BGGR).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER_GRBG => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER_GRBG).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER_GBRG => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER_GBRG).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER10_RGGB => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER10_RGGB).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER10_BGGR => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER10_BGGR).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER10_GRBG => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER10_GRBG).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER10_GBRG => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER10_GBRG).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER12_RGGB => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER12_RGGB).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER12_BGGR => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER12_BGGR).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER12_GRBG => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER12_GRBG).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER12_GBRG => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER12_GBRG).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER14_RGGB => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER14_RGGB).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER14_BGGR => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER14_BGGR).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER14_GRBG => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER14_GRBG).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER14_GBRG => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER14_GBRG).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER20_RGGB => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER20_RGGB).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER20_BGGR => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER20_BGGR).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER20_GRBG => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER20_GRBG).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER20_GBRG => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER20_GBRG).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YVU444_PLANAR => {
                writer
                    .write_all(stringify!(CU_EGL_COLOR_FORMAT_YVU444_PLANAR).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YVU422_PLANAR => {
                writer
                    .write_all(stringify!(CU_EGL_COLOR_FORMAT_YVU422_PLANAR).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YVU420_PLANAR => {
                writer
                    .write_all(stringify!(CU_EGL_COLOR_FORMAT_YVU420_PLANAR).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER_ISP_RGGB => {
                writer
                    .write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER_ISP_RGGB).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER_ISP_BGGR => {
                writer
                    .write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER_ISP_BGGR).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER_ISP_GRBG => {
                writer
                    .write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER_ISP_GRBG).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER_ISP_GBRG => {
                writer
                    .write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER_ISP_GBRG).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER_BCCR => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER_BCCR).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER_RCCB => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER_RCCB).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER_CRBC => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER_CRBC).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER_CBRC => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER_CBRC).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER10_CCCC => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER10_CCCC).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER12_BCCR => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER12_BCCR).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER12_RCCB => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER12_RCCB).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER12_CRBC => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER12_CRBC).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER12_CBRC => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER12_CBRC).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_BAYER12_CCCC => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_BAYER12_CCCC).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_Y => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_Y).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YUV420_SEMIPLANAR_2020 => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_YUV420_SEMIPLANAR_2020).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YVU420_SEMIPLANAR_2020 => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_YVU420_SEMIPLANAR_2020).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YUV420_PLANAR_2020 => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_YUV420_PLANAR_2020).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YVU420_PLANAR_2020 => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_YVU420_PLANAR_2020).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YUV420_SEMIPLANAR_709 => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_YUV420_SEMIPLANAR_709).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YVU420_SEMIPLANAR_709 => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_YVU420_SEMIPLANAR_709).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YUV420_PLANAR_709 => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_YUV420_PLANAR_709).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YVU420_PLANAR_709 => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_YVU420_PLANAR_709).as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_Y10V10U10_420_SEMIPLANAR_709 => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_Y10V10U10_420_SEMIPLANAR_709)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_Y10V10U10_420_SEMIPLANAR_2020 => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_Y10V10U10_420_SEMIPLANAR_2020)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_Y10V10U10_422_SEMIPLANAR_2020 => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_Y10V10U10_422_SEMIPLANAR_2020)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_Y10V10U10_422_SEMIPLANAR => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_Y10V10U10_422_SEMIPLANAR)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_Y10V10U10_422_SEMIPLANAR_709 => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_Y10V10U10_422_SEMIPLANAR_709)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_Y_ER => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_Y_ER).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_Y_709_ER => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_Y_709_ER).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_Y10_ER => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_Y10_ER).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_Y10_709_ER => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_Y10_709_ER).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_Y12_ER => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_Y12_ER).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_Y12_709_ER => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_Y12_709_ER).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YUVA => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_YUVA).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YUV => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_YUV).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_YVYU => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_YVYU).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_VYUY => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_VYUY).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_Y10V10U10_420_SEMIPLANAR_ER => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_Y10V10U10_420_SEMIPLANAR_ER)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_Y10V10U10_420_SEMIPLANAR_709_ER => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_Y10V10U10_420_SEMIPLANAR_709_ER)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_Y10V10U10_444_SEMIPLANAR_ER => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_Y10V10U10_444_SEMIPLANAR_ER)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_Y10V10U10_444_SEMIPLANAR_709_ER => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_Y10V10U10_444_SEMIPLANAR_709_ER)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_Y12V12U12_420_SEMIPLANAR_ER => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_Y12V12U12_420_SEMIPLANAR_ER)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_Y12V12U12_420_SEMIPLANAR_709_ER => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_Y12V12U12_420_SEMIPLANAR_709_ER)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_Y12V12U12_444_SEMIPLANAR_ER => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_Y12V12U12_444_SEMIPLANAR_ER)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_Y12V12U12_444_SEMIPLANAR_709_ER => {
                writer
                    .write_all(
                        stringify!(CU_EGL_COLOR_FORMAT_Y12V12U12_444_SEMIPLANAR_709_ER)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_UYVY_709 => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_UYVY_709).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_UYVY_709_ER => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_UYVY_709_ER).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_UYVY_2020 => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_UYVY_2020).as_bytes())
            }
            &cuda_types::cuda::CUeglColorFormat_enum::CU_EGL_COLOR_FORMAT_MAX => {
                writer.write_all(stringify!(CU_EGL_COLOR_FORMAT_MAX).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUeglStreamConnection {
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
pub fn write_cuGraphicsEGLRegisterImage(
    writer: &mut (impl std::io::Write + ?Sized),
    pCudaResource: *mut cuda_types::cuda::CUgraphicsResource,
    image: cuda_types::cuda::EGLImageKHR,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pCudaResource), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pCudaResource,
        "cuGraphicsEGLRegisterImage",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(image), ": ").as_bytes())?;
    crate::CudaDisplay::write(&image, "cuGraphicsEGLRegisterImage", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuGraphicsEGLRegisterImage", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuEGLStreamConsumerConnect(
    writer: &mut (impl std::io::Write + ?Sized),
    conn: *mut cuda_types::cuda::CUeglStreamConnection,
    stream: cuda_types::cuda::EGLStreamKHR,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(conn), ": ").as_bytes())?;
    crate::CudaDisplay::write(&conn, "cuEGLStreamConsumerConnect", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&stream, "cuEGLStreamConsumerConnect", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuEGLStreamConsumerConnectWithFlags(
    writer: &mut (impl std::io::Write + ?Sized),
    conn: *mut cuda_types::cuda::CUeglStreamConnection,
    stream: cuda_types::cuda::EGLStreamKHR,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(conn), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &conn,
        "cuEGLStreamConsumerConnectWithFlags",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &stream,
        "cuEGLStreamConsumerConnectWithFlags",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &flags,
        "cuEGLStreamConsumerConnectWithFlags",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuEGLStreamConsumerDisconnect(
    writer: &mut (impl std::io::Write + ?Sized),
    conn: *mut cuda_types::cuda::CUeglStreamConnection,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(conn), ": ").as_bytes())?;
    crate::CudaDisplay::write(&conn, "cuEGLStreamConsumerDisconnect", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuEGLStreamConsumerAcquireFrame(
    writer: &mut (impl std::io::Write + ?Sized),
    conn: *mut cuda_types::cuda::CUeglStreamConnection,
    pCudaResource: *mut cuda_types::cuda::CUgraphicsResource,
    pStream: *mut cuda_types::cuda::CUstream,
    timeout: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(conn), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &conn,
        "cuEGLStreamConsumerAcquireFrame",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pCudaResource), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pCudaResource,
        "cuEGLStreamConsumerAcquireFrame",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pStream,
        "cuEGLStreamConsumerAcquireFrame",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(timeout), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &timeout,
        "cuEGLStreamConsumerAcquireFrame",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuEGLStreamConsumerReleaseFrame(
    writer: &mut (impl std::io::Write + ?Sized),
    conn: *mut cuda_types::cuda::CUeglStreamConnection,
    pCudaResource: cuda_types::cuda::CUgraphicsResource,
    pStream: *mut cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(conn), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &conn,
        "cuEGLStreamConsumerReleaseFrame",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pCudaResource), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pCudaResource,
        "cuEGLStreamConsumerReleaseFrame",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pStream,
        "cuEGLStreamConsumerReleaseFrame",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuEGLStreamProducerConnect(
    writer: &mut (impl std::io::Write + ?Sized),
    conn: *mut cuda_types::cuda::CUeglStreamConnection,
    stream: cuda_types::cuda::EGLStreamKHR,
    width: cuda_types::cuda::EGLint,
    height: cuda_types::cuda::EGLint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(conn), ": ").as_bytes())?;
    crate::CudaDisplay::write(&conn, "cuEGLStreamProducerConnect", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(stream), ": ").as_bytes())?;
    crate::CudaDisplay::write(&stream, "cuEGLStreamProducerConnect", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(width), ": ").as_bytes())?;
    crate::CudaDisplay::write(&width, "cuEGLStreamProducerConnect", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(height), ": ").as_bytes())?;
    crate::CudaDisplay::write(&height, "cuEGLStreamProducerConnect", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuEGLStreamProducerDisconnect(
    writer: &mut (impl std::io::Write + ?Sized),
    conn: *mut cuda_types::cuda::CUeglStreamConnection,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(conn), ": ").as_bytes())?;
    crate::CudaDisplay::write(&conn, "cuEGLStreamProducerDisconnect", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuEGLStreamProducerPresentFrame(
    writer: &mut (impl std::io::Write + ?Sized),
    conn: *mut cuda_types::cuda::CUeglStreamConnection,
    eglframe: cuda_types::cuda::CUeglFrame,
    pStream: *mut cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(conn), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &conn,
        "cuEGLStreamProducerPresentFrame",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(eglframe), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &eglframe,
        "cuEGLStreamProducerPresentFrame",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pStream,
        "cuEGLStreamProducerPresentFrame",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuEGLStreamProducerReturnFrame(
    writer: &mut (impl std::io::Write + ?Sized),
    conn: *mut cuda_types::cuda::CUeglStreamConnection,
    eglframe: *mut cuda_types::cuda::CUeglFrame,
    pStream: *mut cuda_types::cuda::CUstream,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(conn), ": ").as_bytes())?;
    crate::CudaDisplay::write(&conn, "cuEGLStreamProducerReturnFrame", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(eglframe), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &eglframe,
        "cuEGLStreamProducerReturnFrame",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pStream), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pStream,
        "cuEGLStreamProducerReturnFrame",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphicsResourceGetMappedEglFrame(
    writer: &mut (impl std::io::Write + ?Sized),
    eglFrame: *mut cuda_types::cuda::CUeglFrame,
    resource: cuda_types::cuda::CUgraphicsResource,
    index: ::core::ffi::c_uint,
    mipLevel: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(eglFrame), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &eglFrame,
        "cuGraphicsResourceGetMappedEglFrame",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(resource), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &resource,
        "cuGraphicsResourceGetMappedEglFrame",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(index), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &index,
        "cuGraphicsResourceGetMappedEglFrame",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mipLevel), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mipLevel,
        "cuGraphicsResourceGetMappedEglFrame",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuEventCreateFromEGLSync(
    writer: &mut (impl std::io::Write + ?Sized),
    phEvent: *mut cuda_types::cuda::CUevent,
    eglSync: cuda_types::cuda::EGLSyncKHR,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(phEvent), ": ").as_bytes())?;
    crate::CudaDisplay::write(&phEvent, "cuEventCreateFromEGLSync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(eglSync), ": ").as_bytes())?;
    crate::CudaDisplay::write(&eglSync, "cuEventCreateFromEGLSync", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuEventCreateFromEGLSync", arg_idx, writer)?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::cuda::VdpStatus {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::VdpStatus::VDP_STATUS_OK => {
                writer.write_all(stringify!(VDP_STATUS_OK).as_bytes())
            }
            &cuda_types::cuda::VdpStatus::VDP_STATUS_NO_IMPLEMENTATION => {
                writer.write_all(stringify!(VDP_STATUS_NO_IMPLEMENTATION).as_bytes())
            }
            &cuda_types::cuda::VdpStatus::VDP_STATUS_DISPLAY_PREEMPTED => {
                writer.write_all(stringify!(VDP_STATUS_DISPLAY_PREEMPTED).as_bytes())
            }
            &cuda_types::cuda::VdpStatus::VDP_STATUS_INVALID_HANDLE => {
                writer.write_all(stringify!(VDP_STATUS_INVALID_HANDLE).as_bytes())
            }
            &cuda_types::cuda::VdpStatus::VDP_STATUS_INVALID_POINTER => {
                writer.write_all(stringify!(VDP_STATUS_INVALID_POINTER).as_bytes())
            }
            &cuda_types::cuda::VdpStatus::VDP_STATUS_INVALID_CHROMA_TYPE => {
                writer.write_all(stringify!(VDP_STATUS_INVALID_CHROMA_TYPE).as_bytes())
            }
            &cuda_types::cuda::VdpStatus::VDP_STATUS_INVALID_Y_CB_CR_FORMAT => {
                writer
                    .write_all(stringify!(VDP_STATUS_INVALID_Y_CB_CR_FORMAT).as_bytes())
            }
            &cuda_types::cuda::VdpStatus::VDP_STATUS_INVALID_RGBA_FORMAT => {
                writer.write_all(stringify!(VDP_STATUS_INVALID_RGBA_FORMAT).as_bytes())
            }
            &cuda_types::cuda::VdpStatus::VDP_STATUS_INVALID_INDEXED_FORMAT => {
                writer
                    .write_all(stringify!(VDP_STATUS_INVALID_INDEXED_FORMAT).as_bytes())
            }
            &cuda_types::cuda::VdpStatus::VDP_STATUS_INVALID_COLOR_STANDARD => {
                writer
                    .write_all(stringify!(VDP_STATUS_INVALID_COLOR_STANDARD).as_bytes())
            }
            &cuda_types::cuda::VdpStatus::VDP_STATUS_INVALID_COLOR_TABLE_FORMAT => {
                writer
                    .write_all(
                        stringify!(VDP_STATUS_INVALID_COLOR_TABLE_FORMAT).as_bytes(),
                    )
            }
            &cuda_types::cuda::VdpStatus::VDP_STATUS_INVALID_BLEND_FACTOR => {
                writer.write_all(stringify!(VDP_STATUS_INVALID_BLEND_FACTOR).as_bytes())
            }
            &cuda_types::cuda::VdpStatus::VDP_STATUS_INVALID_BLEND_EQUATION => {
                writer
                    .write_all(stringify!(VDP_STATUS_INVALID_BLEND_EQUATION).as_bytes())
            }
            &cuda_types::cuda::VdpStatus::VDP_STATUS_INVALID_FLAG => {
                writer.write_all(stringify!(VDP_STATUS_INVALID_FLAG).as_bytes())
            }
            &cuda_types::cuda::VdpStatus::VDP_STATUS_INVALID_DECODER_PROFILE => {
                writer
                    .write_all(stringify!(VDP_STATUS_INVALID_DECODER_PROFILE).as_bytes())
            }
            &cuda_types::cuda::VdpStatus::VDP_STATUS_INVALID_VIDEO_MIXER_FEATURE => {
                writer
                    .write_all(
                        stringify!(VDP_STATUS_INVALID_VIDEO_MIXER_FEATURE).as_bytes(),
                    )
            }
            &cuda_types::cuda::VdpStatus::VDP_STATUS_INVALID_VIDEO_MIXER_PARAMETER => {
                writer
                    .write_all(
                        stringify!(VDP_STATUS_INVALID_VIDEO_MIXER_PARAMETER).as_bytes(),
                    )
            }
            &cuda_types::cuda::VdpStatus::VDP_STATUS_INVALID_VIDEO_MIXER_ATTRIBUTE => {
                writer
                    .write_all(
                        stringify!(VDP_STATUS_INVALID_VIDEO_MIXER_ATTRIBUTE).as_bytes(),
                    )
            }
            &cuda_types::cuda::VdpStatus::VDP_STATUS_INVALID_VIDEO_MIXER_PICTURE_STRUCTURE => {
                writer
                    .write_all(
                        stringify!(VDP_STATUS_INVALID_VIDEO_MIXER_PICTURE_STRUCTURE)
                            .as_bytes(),
                    )
            }
            &cuda_types::cuda::VdpStatus::VDP_STATUS_INVALID_FUNC_ID => {
                writer.write_all(stringify!(VDP_STATUS_INVALID_FUNC_ID).as_bytes())
            }
            &cuda_types::cuda::VdpStatus::VDP_STATUS_INVALID_SIZE => {
                writer.write_all(stringify!(VDP_STATUS_INVALID_SIZE).as_bytes())
            }
            &cuda_types::cuda::VdpStatus::VDP_STATUS_INVALID_VALUE => {
                writer.write_all(stringify!(VDP_STATUS_INVALID_VALUE).as_bytes())
            }
            &cuda_types::cuda::VdpStatus::VDP_STATUS_INVALID_STRUCT_VERSION => {
                writer
                    .write_all(stringify!(VDP_STATUS_INVALID_STRUCT_VERSION).as_bytes())
            }
            &cuda_types::cuda::VdpStatus::VDP_STATUS_RESOURCES => {
                writer.write_all(stringify!(VDP_STATUS_RESOURCES).as_bytes())
            }
            &cuda_types::cuda::VdpStatus::VDP_STATUS_HANDLE_DEVICE_MISMATCH => {
                writer
                    .write_all(stringify!(VDP_STATUS_HANDLE_DEVICE_MISMATCH).as_bytes())
            }
            &cuda_types::cuda::VdpStatus::VDP_STATUS_ERROR => {
                writer.write_all(stringify!(VDP_STATUS_ERROR).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::VdpGetProcAddress {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        write!(
            writer,
            "{:p}",
            unsafe {
                std::mem::transmute::<
                    cuda_types::cuda::VdpGetProcAddress,
                    *mut ::std::ffi::c_void,
                >(*self)
            },
        )
    }
}
pub fn write_cuVDPAUGetDevice(
    writer: &mut (impl std::io::Write + ?Sized),
    pDevice: *mut cuda_types::cuda::CUdevice,
    vdpDevice: cuda_types::cuda::VdpDevice,
    vdpGetProcAddress: cuda_types::cuda::VdpGetProcAddress,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pDevice, "cuVDPAUGetDevice", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vdpDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&vdpDevice, "cuVDPAUGetDevice", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vdpGetProcAddress), ": ").as_bytes())?;
    crate::CudaDisplay::write(&vdpGetProcAddress, "cuVDPAUGetDevice", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_cuVDPAUCtxCreate_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    pCtx: *mut cuda_types::cuda::CUcontext,
    flags: ::core::ffi::c_uint,
    device: cuda_types::cuda::CUdevice,
    vdpDevice: cuda_types::cuda::VdpDevice,
    vdpGetProcAddress: cuda_types::cuda::VdpGetProcAddress,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pCtx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pCtx, "cuVDPAUCtxCreate_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuVDPAUCtxCreate_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "cuVDPAUCtxCreate_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vdpDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&vdpDevice, "cuVDPAUCtxCreate_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vdpGetProcAddress), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vdpGetProcAddress,
        "cuVDPAUCtxCreate_v2",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphicsVDPAURegisterVideoSurface(
    writer: &mut (impl std::io::Write + ?Sized),
    pCudaResource: *mut cuda_types::cuda::CUgraphicsResource,
    vdpSurface: cuda_types::cuda::VdpVideoSurface,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pCudaResource), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pCudaResource,
        "cuGraphicsVDPAURegisterVideoSurface",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vdpSurface), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vdpSurface,
        "cuGraphicsVDPAURegisterVideoSurface",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &flags,
        "cuGraphicsVDPAURegisterVideoSurface",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuGraphicsVDPAURegisterOutputSurface(
    writer: &mut (impl std::io::Write + ?Sized),
    pCudaResource: *mut cuda_types::cuda::CUgraphicsResource,
    vdpSurface: cuda_types::cuda::VdpOutputSurface,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pCudaResource), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pCudaResource,
        "cuGraphicsVDPAURegisterOutputSurface",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vdpSurface), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vdpSurface,
        "cuGraphicsVDPAURegisterOutputSurface",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &flags,
        "cuGraphicsVDPAURegisterOutputSurface",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_cuVDPAUCtxCreate(
    writer: &mut (impl std::io::Write + ?Sized),
    pCtx: *mut cuda_types::cuda::CUcontext,
    flags: ::core::ffi::c_uint,
    device: cuda_types::cuda::CUdevice,
    vdpDevice: cuda_types::cuda::VdpDevice,
    vdpGetProcAddress: cuda_types::cuda::VdpGetProcAddress,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pCtx), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pCtx, "cuVDPAUCtxCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "cuVDPAUCtxCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "cuVDPAUCtxCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vdpDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(&vdpDevice, "cuVDPAUCtxCreate", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vdpGetProcAddress), ": ").as_bytes())?;
    crate::CudaDisplay::write(&vdpGetProcAddress, "cuVDPAUCtxCreate", arg_idx, writer)?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::cuda::cudaDataType_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::cudaDataType_t::CUDA_R_16F => {
                writer.write_all(stringify!(CUDA_R_16F).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_C_16F => {
                writer.write_all(stringify!(CUDA_C_16F).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_R_16BF => {
                writer.write_all(stringify!(CUDA_R_16BF).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_C_16BF => {
                writer.write_all(stringify!(CUDA_C_16BF).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_R_32F => {
                writer.write_all(stringify!(CUDA_R_32F).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_C_32F => {
                writer.write_all(stringify!(CUDA_C_32F).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_R_64F => {
                writer.write_all(stringify!(CUDA_R_64F).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_C_64F => {
                writer.write_all(stringify!(CUDA_C_64F).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_R_4I => {
                writer.write_all(stringify!(CUDA_R_4I).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_C_4I => {
                writer.write_all(stringify!(CUDA_C_4I).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_R_4U => {
                writer.write_all(stringify!(CUDA_R_4U).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_C_4U => {
                writer.write_all(stringify!(CUDA_C_4U).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_R_8I => {
                writer.write_all(stringify!(CUDA_R_8I).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_C_8I => {
                writer.write_all(stringify!(CUDA_C_8I).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_R_8U => {
                writer.write_all(stringify!(CUDA_R_8U).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_C_8U => {
                writer.write_all(stringify!(CUDA_C_8U).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_R_16I => {
                writer.write_all(stringify!(CUDA_R_16I).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_C_16I => {
                writer.write_all(stringify!(CUDA_C_16I).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_R_16U => {
                writer.write_all(stringify!(CUDA_R_16U).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_C_16U => {
                writer.write_all(stringify!(CUDA_C_16U).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_R_32I => {
                writer.write_all(stringify!(CUDA_R_32I).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_C_32I => {
                writer.write_all(stringify!(CUDA_C_32I).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_R_32U => {
                writer.write_all(stringify!(CUDA_R_32U).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_C_32U => {
                writer.write_all(stringify!(CUDA_C_32U).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_R_64I => {
                writer.write_all(stringify!(CUDA_R_64I).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_C_64I => {
                writer.write_all(stringify!(CUDA_C_64I).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_R_64U => {
                writer.write_all(stringify!(CUDA_R_64U).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_C_64U => {
                writer.write_all(stringify!(CUDA_C_64U).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_R_8F_E4M3 => {
                writer.write_all(stringify!(CUDA_R_8F_E4M3).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_R_8F_UE4M3 => {
                writer.write_all(stringify!(CUDA_R_8F_UE4M3).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_R_8F_E5M2 => {
                writer.write_all(stringify!(CUDA_R_8F_E5M2).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_R_8F_UE8M0 => {
                writer.write_all(stringify!(CUDA_R_8F_UE8M0).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_R_6F_E2M3 => {
                writer.write_all(stringify!(CUDA_R_6F_E2M3).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_R_6F_E3M2 => {
                writer.write_all(stringify!(CUDA_R_6F_E3M2).as_bytes())
            }
            &cuda_types::cuda::cudaDataType_t::CUDA_R_4F_E2M1 => {
                writer.write_all(stringify!(CUDA_R_4F_E2M1).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::libraryPropertyType_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::cuda::libraryPropertyType_t::MAJOR_VERSION => {
                writer.write_all(stringify!(MAJOR_VERSION).as_bytes())
            }
            &cuda_types::cuda::libraryPropertyType_t::MINOR_VERSION => {
                writer.write_all(stringify!(MINOR_VERSION).as_bytes())
            }
            &cuda_types::cuda::libraryPropertyType_t::PATCH_LEVEL => {
                writer.write_all(stringify!(PATCH_LEVEL).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::cuda::float2 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(x), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.x, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(y), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.y, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::double2 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(x), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.x, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(y), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.y, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::cuda::CUresult {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            Ok(()) => writer.write_all(b"CUDA_SUCCESS"),
            Err(err) => {
                match err.0.get() {
                    1 => writer.write_all("CUDA_ERROR_INVALID_VALUE".as_bytes()),
                    2 => writer.write_all("CUDA_ERROR_OUT_OF_MEMORY".as_bytes()),
                    3 => writer.write_all("CUDA_ERROR_NOT_INITIALIZED".as_bytes()),
                    4 => writer.write_all("CUDA_ERROR_DEINITIALIZED".as_bytes()),
                    5 => writer.write_all("CUDA_ERROR_PROFILER_DISABLED".as_bytes()),
                    6 => {
                        writer
                            .write_all("CUDA_ERROR_PROFILER_NOT_INITIALIZED".as_bytes())
                    }
                    7 => {
                        writer
                            .write_all("CUDA_ERROR_PROFILER_ALREADY_STARTED".as_bytes())
                    }
                    8 => {
                        writer
                            .write_all("CUDA_ERROR_PROFILER_ALREADY_STOPPED".as_bytes())
                    }
                    34 => writer.write_all("CUDA_ERROR_STUB_LIBRARY".as_bytes()),
                    46 => writer.write_all("CUDA_ERROR_DEVICE_UNAVAILABLE".as_bytes()),
                    100 => writer.write_all("CUDA_ERROR_NO_DEVICE".as_bytes()),
                    101 => writer.write_all("CUDA_ERROR_INVALID_DEVICE".as_bytes()),
                    102 => writer.write_all("CUDA_ERROR_DEVICE_NOT_LICENSED".as_bytes()),
                    200 => writer.write_all("CUDA_ERROR_INVALID_IMAGE".as_bytes()),
                    201 => writer.write_all("CUDA_ERROR_INVALID_CONTEXT".as_bytes()),
                    202 => {
                        writer.write_all("CUDA_ERROR_CONTEXT_ALREADY_CURRENT".as_bytes())
                    }
                    205 => writer.write_all("CUDA_ERROR_MAP_FAILED".as_bytes()),
                    206 => writer.write_all("CUDA_ERROR_UNMAP_FAILED".as_bytes()),
                    207 => writer.write_all("CUDA_ERROR_ARRAY_IS_MAPPED".as_bytes()),
                    208 => writer.write_all("CUDA_ERROR_ALREADY_MAPPED".as_bytes()),
                    209 => writer.write_all("CUDA_ERROR_NO_BINARY_FOR_GPU".as_bytes()),
                    210 => writer.write_all("CUDA_ERROR_ALREADY_ACQUIRED".as_bytes()),
                    211 => writer.write_all("CUDA_ERROR_NOT_MAPPED".as_bytes()),
                    212 => writer.write_all("CUDA_ERROR_NOT_MAPPED_AS_ARRAY".as_bytes()),
                    213 => {
                        writer.write_all("CUDA_ERROR_NOT_MAPPED_AS_POINTER".as_bytes())
                    }
                    214 => writer.write_all("CUDA_ERROR_ECC_UNCORRECTABLE".as_bytes()),
                    215 => writer.write_all("CUDA_ERROR_UNSUPPORTED_LIMIT".as_bytes()),
                    216 => {
                        writer.write_all("CUDA_ERROR_CONTEXT_ALREADY_IN_USE".as_bytes())
                    }
                    217 => {
                        writer.write_all("CUDA_ERROR_PEER_ACCESS_UNSUPPORTED".as_bytes())
                    }
                    218 => writer.write_all("CUDA_ERROR_INVALID_PTX".as_bytes()),
                    219 => {
                        writer
                            .write_all("CUDA_ERROR_INVALID_GRAPHICS_CONTEXT".as_bytes())
                    }
                    220 => writer.write_all("CUDA_ERROR_NVLINK_UNCORRECTABLE".as_bytes()),
                    221 => {
                        writer.write_all("CUDA_ERROR_JIT_COMPILER_NOT_FOUND".as_bytes())
                    }
                    222 => {
                        writer.write_all("CUDA_ERROR_UNSUPPORTED_PTX_VERSION".as_bytes())
                    }
                    223 => {
                        writer
                            .write_all("CUDA_ERROR_JIT_COMPILATION_DISABLED".as_bytes())
                    }
                    224 => {
                        writer
                            .write_all("CUDA_ERROR_UNSUPPORTED_EXEC_AFFINITY".as_bytes())
                    }
                    225 => {
                        writer
                            .write_all("CUDA_ERROR_UNSUPPORTED_DEVSIDE_SYNC".as_bytes())
                    }
                    226 => writer.write_all("CUDA_ERROR_CONTAINED".as_bytes()),
                    300 => writer.write_all("CUDA_ERROR_INVALID_SOURCE".as_bytes()),
                    301 => writer.write_all("CUDA_ERROR_FILE_NOT_FOUND".as_bytes()),
                    302 => {
                        writer
                            .write_all(
                                "CUDA_ERROR_SHARED_OBJECT_SYMBOL_NOT_FOUND".as_bytes(),
                            )
                    }
                    303 => {
                        writer
                            .write_all("CUDA_ERROR_SHARED_OBJECT_INIT_FAILED".as_bytes())
                    }
                    304 => writer.write_all("CUDA_ERROR_OPERATING_SYSTEM".as_bytes()),
                    400 => writer.write_all("CUDA_ERROR_INVALID_HANDLE".as_bytes()),
                    401 => writer.write_all("CUDA_ERROR_ILLEGAL_STATE".as_bytes()),
                    402 => writer.write_all("CUDA_ERROR_LOSSY_QUERY".as_bytes()),
                    500 => writer.write_all("CUDA_ERROR_NOT_FOUND".as_bytes()),
                    600 => writer.write_all("CUDA_ERROR_NOT_READY".as_bytes()),
                    700 => writer.write_all("CUDA_ERROR_ILLEGAL_ADDRESS".as_bytes()),
                    701 => {
                        writer.write_all("CUDA_ERROR_LAUNCH_OUT_OF_RESOURCES".as_bytes())
                    }
                    702 => writer.write_all("CUDA_ERROR_LAUNCH_TIMEOUT".as_bytes()),
                    703 => {
                        writer
                            .write_all(
                                "CUDA_ERROR_LAUNCH_INCOMPATIBLE_TEXTURING".as_bytes(),
                            )
                    }
                    704 => {
                        writer
                            .write_all(
                                "CUDA_ERROR_PEER_ACCESS_ALREADY_ENABLED".as_bytes(),
                            )
                    }
                    705 => {
                        writer.write_all("CUDA_ERROR_PEER_ACCESS_NOT_ENABLED".as_bytes())
                    }
                    708 => {
                        writer.write_all("CUDA_ERROR_PRIMARY_CONTEXT_ACTIVE".as_bytes())
                    }
                    709 => writer.write_all("CUDA_ERROR_CONTEXT_IS_DESTROYED".as_bytes()),
                    710 => writer.write_all("CUDA_ERROR_ASSERT".as_bytes()),
                    711 => writer.write_all("CUDA_ERROR_TOO_MANY_PEERS".as_bytes()),
                    712 => {
                        writer
                            .write_all(
                                "CUDA_ERROR_HOST_MEMORY_ALREADY_REGISTERED".as_bytes(),
                            )
                    }
                    713 => {
                        writer
                            .write_all(
                                "CUDA_ERROR_HOST_MEMORY_NOT_REGISTERED".as_bytes(),
                            )
                    }
                    714 => writer.write_all("CUDA_ERROR_HARDWARE_STACK_ERROR".as_bytes()),
                    715 => writer.write_all("CUDA_ERROR_ILLEGAL_INSTRUCTION".as_bytes()),
                    716 => writer.write_all("CUDA_ERROR_MISALIGNED_ADDRESS".as_bytes()),
                    717 => {
                        writer.write_all("CUDA_ERROR_INVALID_ADDRESS_SPACE".as_bytes())
                    }
                    718 => writer.write_all("CUDA_ERROR_INVALID_PC".as_bytes()),
                    719 => writer.write_all("CUDA_ERROR_LAUNCH_FAILED".as_bytes()),
                    720 => {
                        writer
                            .write_all(
                                "CUDA_ERROR_COOPERATIVE_LAUNCH_TOO_LARGE".as_bytes(),
                            )
                    }
                    721 => writer.write_all("CUDA_ERROR_TENSOR_MEMORY_LEAK".as_bytes()),
                    800 => writer.write_all("CUDA_ERROR_NOT_PERMITTED".as_bytes()),
                    801 => writer.write_all("CUDA_ERROR_NOT_SUPPORTED".as_bytes()),
                    802 => writer.write_all("CUDA_ERROR_SYSTEM_NOT_READY".as_bytes()),
                    803 => {
                        writer.write_all("CUDA_ERROR_SYSTEM_DRIVER_MISMATCH".as_bytes())
                    }
                    804 => {
                        writer
                            .write_all(
                                "CUDA_ERROR_COMPAT_NOT_SUPPORTED_ON_DEVICE".as_bytes(),
                            )
                    }
                    805 => {
                        writer.write_all("CUDA_ERROR_MPS_CONNECTION_FAILED".as_bytes())
                    }
                    806 => writer.write_all("CUDA_ERROR_MPS_RPC_FAILURE".as_bytes()),
                    807 => writer.write_all("CUDA_ERROR_MPS_SERVER_NOT_READY".as_bytes()),
                    808 => {
                        writer.write_all("CUDA_ERROR_MPS_MAX_CLIENTS_REACHED".as_bytes())
                    }
                    809 => {
                        writer
                            .write_all(
                                "CUDA_ERROR_MPS_MAX_CONNECTIONS_REACHED".as_bytes(),
                            )
                    }
                    810 => {
                        writer.write_all("CUDA_ERROR_MPS_CLIENT_TERMINATED".as_bytes())
                    }
                    811 => writer.write_all("CUDA_ERROR_CDP_NOT_SUPPORTED".as_bytes()),
                    812 => writer.write_all("CUDA_ERROR_CDP_VERSION_MISMATCH".as_bytes()),
                    900 => {
                        writer
                            .write_all(
                                "CUDA_ERROR_STREAM_CAPTURE_UNSUPPORTED".as_bytes(),
                            )
                    }
                    901 => {
                        writer
                            .write_all(
                                "CUDA_ERROR_STREAM_CAPTURE_INVALIDATED".as_bytes(),
                            )
                    }
                    902 => writer.write_all("CUDA_ERROR_STREAM_CAPTURE_MERGE".as_bytes()),
                    903 => {
                        writer
                            .write_all("CUDA_ERROR_STREAM_CAPTURE_UNMATCHED".as_bytes())
                    }
                    904 => {
                        writer.write_all("CUDA_ERROR_STREAM_CAPTURE_UNJOINED".as_bytes())
                    }
                    905 => {
                        writer
                            .write_all("CUDA_ERROR_STREAM_CAPTURE_ISOLATION".as_bytes())
                    }
                    906 => {
                        writer.write_all("CUDA_ERROR_STREAM_CAPTURE_IMPLICIT".as_bytes())
                    }
                    907 => writer.write_all("CUDA_ERROR_CAPTURED_EVENT".as_bytes()),
                    908 => {
                        writer
                            .write_all(
                                "CUDA_ERROR_STREAM_CAPTURE_WRONG_THREAD".as_bytes(),
                            )
                    }
                    909 => writer.write_all("CUDA_ERROR_TIMEOUT".as_bytes()),
                    910 => {
                        writer
                            .write_all("CUDA_ERROR_GRAPH_EXEC_UPDATE_FAILURE".as_bytes())
                    }
                    911 => writer.write_all("CUDA_ERROR_EXTERNAL_DEVICE".as_bytes()),
                    912 => writer.write_all("CUDA_ERROR_INVALID_CLUSTER_SIZE".as_bytes()),
                    913 => writer.write_all("CUDA_ERROR_FUNCTION_NOT_LOADED".as_bytes()),
                    914 => {
                        writer.write_all("CUDA_ERROR_INVALID_RESOURCE_TYPE".as_bytes())
                    }
                    915 => {
                        writer
                            .write_all(
                                "CUDA_ERROR_INVALID_RESOURCE_CONFIGURATION".as_bytes(),
                            )
                    }
                    916 => writer.write_all("CUDA_ERROR_KEY_ROTATION".as_bytes()),
                    999 => writer.write_all("CUDA_ERROR_UNKNOWN".as_bytes()),
                    err => write!(writer, "{}", err),
                }
            }
        }
    }
}
