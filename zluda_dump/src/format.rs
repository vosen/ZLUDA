use paste::paste;
use std::{
    ffi::{c_void, CStr},
    io::Write,
    mem, ptr,
};

use crate::cuda::*;

pub(crate) trait FormatCudaObject {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write);
}

fn write_post_execution_ptr<T: FormatCudaObject + Copy>(
    t: *const T,
    result: CUresult,
    f: &mut impl Write,
) {
    if t == ptr::null() {
        write!(f, "NULL").ok();
    } else if result != CUresult::CUDA_SUCCESS {
        write!(f, "NONE").ok();
    } else {
        unsafe { *t }.write_post_execution(result, f)
    }
}

impl<T: FormatCudaObject + Copy> FormatCudaObject for *mut T {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write_post_execution_ptr(self, result, f)
    }
}

impl<T: FormatCudaObject + Copy> FormatCudaObject for *const T {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write_post_execution_ptr(self, result, f)
    }
}

impl FormatCudaObject for CUmodule {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(f, "{:p}", self).ok();
    }
}

impl FormatCudaObject for CUfunction {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(f, "{:p}", self).ok();
    }
}

impl FormatCudaObject for *mut c_void {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(f, "{:p}", self).ok();
    }
}

impl FormatCudaObject for *const c_void {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(f, "{:p}", self).ok();
    }
}

impl FormatCudaObject for CUstream {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(f, "{:p}", self).ok();
    }
}

impl FormatCudaObject for CUgraphicsResource {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(f, "{:p}", self).ok();
    }
}

impl FormatCudaObject for CUdeviceptr {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(f, "{:p}", self.0 as *const ()).ok();
    }
}

impl FormatCudaObject for CUtexref {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(f, "{:p}", self as *const ()).ok();
    }
}

impl FormatCudaObject for CUmipmappedArray {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(f, "{:p}", self).ok();
    }
}

impl FormatCudaObject for CUarray {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(f, "{:p}", self).ok();
    }
}

impl FormatCudaObject for CUcontext {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(f, "{:p}", self).ok();
    }
}

impl FormatCudaObject for CUsurfref {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(f, "{:p}", self).ok();
    }
}

impl FormatCudaObject for CUgraphNode {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(f, "{:p}", self).ok();
    }
}

impl FormatCudaObject for CUgraphExec {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(f, "{:p}", self).ok();
    }
}

impl FormatCudaObject for CUevent {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(f, "{:p}", self).ok();
    }
}

impl FormatCudaObject for CUgraph {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(f, "{:p}", self).ok();
    }
}

impl FormatCudaObject for CUexternalSemaphore {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(f, "{:p}", self).ok();
    }
}

impl FormatCudaObject for CUexternalMemory {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(f, "{:p}", self).ok();
    }
}

impl FormatCudaObject for CUhostFn {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        match self.map(|x| unsafe { mem::transmute::<_, *const ()>(x) }) {
            Some(x) => write!(f, "{:p}", x),
            None => write!(f, "NULL"),
        }
        .ok();
    }
}

impl FormatCudaObject for CUoccupancyB2DSize {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        match self.map(|x| unsafe { mem::transmute::<_, *const ()>(x) }) {
            Some(x) => write!(f, "{:p}", x),
            None => write!(f, "NULL"),
        }
        .ok();
    }
}

impl FormatCudaObject for CUDA_RESOURCE_DESC {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        match self.resType {
            CU_RESOURCE_TYPE_ARRAY => {
                write!(
                    f,
                    "{{resType: CU_RESOURCE_TYPE_ARRAY, hArray: {:p}, flags: {}}}",
                    self.res.array.hArray, self.flags
                )
            }
            CU_RESOURCE_TYPE_MIPMAPPED_ARRAY => {
                write!(
                    f,
                    "{{resType: CU_RESOURCE_TYPE_MIPMAPPED_ARRAY, hMipmappedArray: {:p}, flags: {}}}",
                    self.res.mipmap.hMipmappedArray, self.flags
                )
            }
            CU_RESOURCE_TYPE_LINEAR => {
                write!(
                    f,
                    "{{resType: CU_RESOURCE_TYPE_LINEAR, devPtr: {:p}, format:",
                    self.res.linear.devPtr.0 as *const ()
                ).ok();
                self.res.linear.format.write_post_execution(result, f);
                write!(
                    f,
                    ", numChannels: {}, sizeInBytes: {}}}",
                    self.res.linear.numChannels, self.res.linear.sizeInBytes,
                )
            }
            CU_RESOURCE_TYPE_PITCH2D => {
                write!(
                    f,
                    "{{resType: CU_RESOURCE_TYPE_PITCH2D, devPtr: {:p}, format:",
                    self.res.pitch2D.devPtr.0 as *const ()
                ).ok();
                self.res.pitch2D.format.write_post_execution(result, f);
                write!(
                    f,
                    ", numChannels: {}, width: {}, height: {}, pitchInBytes: {}}}",
                    self.res.pitch2D.numChannels, self.res.pitch2D.width, self.res.pitch2D.height, self.res.pitch2D.pitchInBytes
                )
            }
            _ => {
                write!(f, "{{resType: {}, flags: {}}}", self.resType.0, self.flags)
            }
        }
        .ok();
    }
}

impl FormatCudaObject for CUDA_RESOURCE_VIEW_DESC {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(f, "{{format: ").ok();
        self.format.write_post_execution(result, f);
        write!(
            f,
            ", width: {}, height: {}, depth: {}, firstMipmapLevel: {}, lastMipmapLevel: {}, firstLayer: {}, lastLayer: {}}}",
            self.width,
            self.height,
            self.depth,
            self.firstMipmapLevel,
            self.lastMipmapLevel,
            self.firstLayer,
            self.lastLayer
        ).ok();
    }
}

impl FormatCudaObject for CUDA_TEXTURE_DESC {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(f, "{{addressMode: [").ok();
        let [addressMode_0, addressMode_1, addressMode_2] = self.addressMode;
        addressMode_0.write_post_execution(result, f);
        write!(f, ", ").ok();
        addressMode_1.write_post_execution(result, f);
        write!(f, ", ").ok();
        addressMode_2.write_post_execution(result, f);
        write!(
            f,
            "], flags: {}, maxAnisotropy: {}, mipmapFilterMode: ",
            self.flags, self.maxAnisotropy
        );
        self.mipmapFilterMode.write_post_execution(result, f);
        write!(
            f,
            ", mipmapLevelBias: {}, minMipmapLevelClamp: {}, maxMipmapLevelClamp: {}, borderColor: [{}, {}, {}, {}]}}",
            self.mipmapLevelBias,
            self.minMipmapLevelClamp,
            self.maxMipmapLevelClamp,
            self.borderColor[0],
            self.borderColor[1],
            self.borderColor[2],
            self.borderColor[3]
        ).ok();
    }
}

impl FormatCudaObject for CUDA_ARRAY_DESCRIPTOR {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(
            f,
            "{{Width: {}, Height: {}, Format: ",
            self.Width, self.Height
        )
        .ok();
        self.Format.write_post_execution(result, f);
        write!(f, ",  NumChannels: {}}}", self.NumChannels).ok();
    }
}

impl FormatCudaObject for CUDA_MEMCPY3D {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(
            f,
            "{{srcXInBytes: {}, srcY: {}, srcZ: {}, srcLOD: {}, srcMemoryType: ",
            self.srcXInBytes, self.srcY, self.srcZ, self.srcLOD,
        )
        .ok();
        self.srcMemoryType.write_post_execution(result, f);
        write!(
            f,
            ", srcHost: {:p}, srcDevice: {:p}, srcArray: {:p}, srcPitch: {}, srcHeight: {}, dstXInBytes: {}, dstY: {}, dstZ: {}, dstLOD: {}, dstMemoryType: ",
            self.srcHost,
            self.srcDevice.0 as *const (),
            self.srcArray,
            self.srcPitch,
            self.srcHeight,
            self.dstXInBytes,
            self.dstY,
            self.dstZ,
            self.dstLOD,
        ).ok();
        self.dstMemoryType.write_post_execution(result, f);
        write!(
            f,
            ", dstHost: {:p}, dstDevice: {:p}, dstArray: {:p}, dstPitch: {}, dstHeight: {}, WidthInBytes: {}, Height: {}, Depth: {}}}",
            self.dstHost,
            self.dstDevice.0 as *const (),
            self.dstArray,
            self.dstPitch,
            self.dstHeight,
            self.WidthInBytes,
            self.Height,
            self.Depth,
        ).ok();
    }
}

impl FormatCudaObject for CUmemLocation {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(f, "{{type: ").ok();
        self.type_.write_post_execution(result, f);
        write!(f, ", id: {}}}", self.id).ok();
    }
}

impl FormatCudaObject for *const i8 {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(
            f,
            "\"{}\"",
            unsafe { CStr::from_ptr(self) }.to_str().unwrap()
        )
        .ok();
    }
}

impl FormatCudaObject for u32 {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(f, "{}", self).ok();
    }
}

impl FormatCudaObject for u64 {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(f, "{}", self).ok();
    }
}

impl FormatCudaObject for i32 {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(f, "{}", self).ok();
    }
}

impl FormatCudaObject for f32 {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(f, "{}", self).ok();
    }
}

impl FormatCudaObject for CUdevice {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(f, "{}", self.0).ok();
    }
}

impl FormatCudaObject for usize {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(f, "{}", self).ok();
    }
}

// TODO: support it properly
impl FormatCudaObject for CUoutput_mode {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        write!(f, "{}", self.0).ok();
    }
}

impl FormatCudaObject for CUuuid {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        let guid = self.bytes;
        write!(f, "{{{:02X}{:02X}{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}}}", guid[0], guid[1], guid[2], guid[3], guid[4], guid[5], guid[6], guid[7], guid[8], guid[9], guid[10], guid[11], guid[12], guid[13], guid[14], guid[15]).ok();
    }
}

// ENUMS

/*
impl FormatCudaObject for CUjit_option {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        match stringify_cujit_option(self) {
            Some(text) => write!(f, "{}", text),
            None => write!(f, "{}", self.0),
        }
        .ok();
    }
}

impl FormatCudaObject for CUdevice_attribute {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        match stringify_cudevice_attribute(self) {
            Some(text) => write!(f, "{}", text),
            None => write!(f, "{}", self.0),
        }
        .ok();
    }
}

impl FormatCudaObject for CUdevice_P2PAttribute {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        match stringify_cudevice_p2pattribute(self) {
            Some(text) => write!(f, "{}", text),
            None => write!(f, "{}", self.0),
        }
        .ok();
    }
}

impl FormatCudaObject for CUarray_format {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        match stringify_cuarray_format(self) {
            Some(text) => write!(f, "{}", text),
            None => write!(f, "{}", self.0),
        }
        .ok();
    }
}

impl FormatCudaObject for CUresourceViewFormat {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        match stringify_curesourceview_format(self) {
            Some(text) => write!(f, "{}", text),
            None => write!(f, "{}", self.0),
        }
        .ok();
    }
}

impl FormatCudaObject for CUaddress_mode {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        match stringify_cuaddress_mode(self) {
            Some(text) => write!(f, "{}", text),
            None => write!(f, "{}", self.0),
        }
        .ok();
    }
}

impl FormatCudaObject for CUfilter_mode {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        match stringify_cufilter_mode(self) {
            Some(text) => write!(f, "{}", text),
            None => write!(f, "{}", self.0),
        }
        .ok();
    }
}

impl FormatCudaObject for CUgraphExecUpdateResult {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        match stringify_cugraph_exec_updateresult(self) {
            Some(text) => write!(f, "{}", text),
            None => write!(f, "{}", self.0),
        }
        .ok();
    }
}

impl FormatCudaObject for CUmemorytype {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        match stringify_cumemorytype(self) {
            Some(text) => write!(f, "{}", text),
            None => write!(f, "{}", self.0),
        }
        .ok();
    }
}

impl FormatCudaObject for CUmemLocationType {
    fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
        match stringify_cumemorytype(self) {
            Some(text) => write!(f, "{}", text),
            None => write!(f, "{}", self.0),
        }
        .ok();
    }
}
*/

macro_rules! stringify_enum {
    ($type_:ident, [ $($variant:ident),+ ]) => {
        paste! {
            pub(crate) fn [<stringify_ $type_>](x: $type_) -> Option<&'static str> {
                match x {
                    $(
                        $type_::$variant => Some(stringify!($variant)),
                    )+
                    _ => None
                }
            }

            impl FormatCudaObject for $type_ {
                fn write_post_execution(self, result: CUresult, f: &mut impl Write) {
                    match [<stringify_ $type_>](self) {
                        Some(text) => write!(f, "{}", text),
                        None => write!(f, "{}", self.0),
                    }
                    .ok();
                }
            }
        }
    }
}

stringify_enum! {
    CUdevice_attribute_enum,
    [
        CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_BLOCK,
        CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_X,
        CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_Y,
        CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_Z,
        CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_X,
        CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_Y,
        CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_Z,
        CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_BLOCK,
        CU_DEVICE_ATTRIBUTE_SHARED_MEMORY_PER_BLOCK,
        CU_DEVICE_ATTRIBUTE_TOTAL_CONSTANT_MEMORY,
        CU_DEVICE_ATTRIBUTE_WARP_SIZE,
        CU_DEVICE_ATTRIBUTE_MAX_PITCH,
        CU_DEVICE_ATTRIBUTE_MAX_REGISTERS_PER_BLOCK,
        CU_DEVICE_ATTRIBUTE_REGISTERS_PER_BLOCK,
        CU_DEVICE_ATTRIBUTE_CLOCK_RATE,
        CU_DEVICE_ATTRIBUTE_TEXTURE_ALIGNMENT,
        CU_DEVICE_ATTRIBUTE_GPU_OVERLAP,
        CU_DEVICE_ATTRIBUTE_MULTIPROCESSOR_COUNT,
        CU_DEVICE_ATTRIBUTE_KERNEL_EXEC_TIMEOUT,
        CU_DEVICE_ATTRIBUTE_INTEGRATED,
        CU_DEVICE_ATTRIBUTE_CAN_MAP_HOST_MEMORY,
        CU_DEVICE_ATTRIBUTE_COMPUTE_MODE,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_WIDTH,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_WIDTH,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_HEIGHT,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_WIDTH,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_HEIGHT,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_DEPTH,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LAYERED_WIDTH,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LAYERED_HEIGHT,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LAYERED_LAYERS,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_ARRAY_WIDTH,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_ARRAY_HEIGHT,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_ARRAY_NUMSLICES,
        CU_DEVICE_ATTRIBUTE_SURFACE_ALIGNMENT,
        CU_DEVICE_ATTRIBUTE_CONCURRENT_KERNELS,
        CU_DEVICE_ATTRIBUTE_ECC_ENABLED,
        CU_DEVICE_ATTRIBUTE_PCI_BUS_ID,
        CU_DEVICE_ATTRIBUTE_PCI_DEVICE_ID,
        CU_DEVICE_ATTRIBUTE_TCC_DRIVER,
        CU_DEVICE_ATTRIBUTE_MEMORY_CLOCK_RATE,
        CU_DEVICE_ATTRIBUTE_GLOBAL_MEMORY_BUS_WIDTH,
        CU_DEVICE_ATTRIBUTE_L2_CACHE_SIZE,
        CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_MULTIPROCESSOR,
        CU_DEVICE_ATTRIBUTE_ASYNC_ENGINE_COUNT,
        CU_DEVICE_ATTRIBUTE_UNIFIED_ADDRESSING,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LAYERED_WIDTH,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LAYERED_LAYERS,
        CU_DEVICE_ATTRIBUTE_CAN_TEX2D_GATHER,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_GATHER_WIDTH,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_GATHER_HEIGHT,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_WIDTH_ALTERNATE,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_HEIGHT_ALTERNATE,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_DEPTH_ALTERNATE,
        CU_DEVICE_ATTRIBUTE_PCI_DOMAIN_ID,
        CU_DEVICE_ATTRIBUTE_TEXTURE_PITCH_ALIGNMENT,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURECUBEMAP_WIDTH,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURECUBEMAP_LAYERED_WIDTH,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURECUBEMAP_LAYERED_LAYERS,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE1D_WIDTH,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_WIDTH,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_HEIGHT,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE3D_WIDTH,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE3D_HEIGHT,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE3D_DEPTH,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE1D_LAYERED_WIDTH,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE1D_LAYERED_LAYERS,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_LAYERED_WIDTH,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_LAYERED_HEIGHT,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_LAYERED_LAYERS,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACECUBEMAP_WIDTH,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACECUBEMAP_LAYERED_WIDTH,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACECUBEMAP_LAYERED_LAYERS,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LINEAR_WIDTH,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_WIDTH,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_HEIGHT,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_PITCH,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_MIPMAPPED_WIDTH,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_MIPMAPPED_HEIGHT,
        CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR,
        CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR,
        CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_MIPMAPPED_WIDTH,
        CU_DEVICE_ATTRIBUTE_STREAM_PRIORITIES_SUPPORTED,
        CU_DEVICE_ATTRIBUTE_GLOBAL_L1_CACHE_SUPPORTED,
        CU_DEVICE_ATTRIBUTE_LOCAL_L1_CACHE_SUPPORTED,
        CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_MULTIPROCESSOR,
        CU_DEVICE_ATTRIBUTE_MAX_REGISTERS_PER_MULTIPROCESSOR,
        CU_DEVICE_ATTRIBUTE_MANAGED_MEMORY,
        CU_DEVICE_ATTRIBUTE_MULTI_GPU_BOARD,
        CU_DEVICE_ATTRIBUTE_MULTI_GPU_BOARD_GROUP_ID,
        CU_DEVICE_ATTRIBUTE_HOST_NATIVE_ATOMIC_SUPPORTED,
        CU_DEVICE_ATTRIBUTE_SINGLE_TO_DOUBLE_PRECISION_PERF_RATIO,
        CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS,
        CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS,
        CU_DEVICE_ATTRIBUTE_COMPUTE_PREEMPTION_SUPPORTED,
        CU_DEVICE_ATTRIBUTE_CAN_USE_HOST_POINTER_FOR_REGISTERED_MEM,
        CU_DEVICE_ATTRIBUTE_CAN_USE_STREAM_MEM_OPS,
        CU_DEVICE_ATTRIBUTE_CAN_USE_64_BIT_STREAM_MEM_OPS,
        CU_DEVICE_ATTRIBUTE_CAN_USE_STREAM_WAIT_VALUE_NOR,
        CU_DEVICE_ATTRIBUTE_COOPERATIVE_LAUNCH,
        CU_DEVICE_ATTRIBUTE_COOPERATIVE_MULTI_DEVICE_LAUNCH,
        CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_BLOCK_OPTIN,
        CU_DEVICE_ATTRIBUTE_CAN_FLUSH_REMOTE_WRITES,
        CU_DEVICE_ATTRIBUTE_HOST_REGISTER_SUPPORTED,
        CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS_USES_HOST_PAGE_TABLES,
        CU_DEVICE_ATTRIBUTE_DIRECT_MANAGED_MEM_ACCESS_FROM_HOST,
        CU_DEVICE_ATTRIBUTE_VIRTUAL_ADDRESS_MANAGEMENT_SUPPORTED,
        CU_DEVICE_ATTRIBUTE_HANDLE_TYPE_POSIX_FILE_DESCRIPTOR_SUPPORTED,
        CU_DEVICE_ATTRIBUTE_HANDLE_TYPE_WIN32_HANDLE_SUPPORTED,
        CU_DEVICE_ATTRIBUTE_HANDLE_TYPE_WIN32_KMT_HANDLE_SUPPORTED,
        CU_DEVICE_ATTRIBUTE_MAX_BLOCKS_PER_MULTIPROCESSOR,
        CU_DEVICE_ATTRIBUTE_GENERIC_COMPRESSION_SUPPORTED,
        CU_DEVICE_ATTRIBUTE_MAX_PERSISTING_L2_CACHE_SIZE,
        CU_DEVICE_ATTRIBUTE_MAX_ACCESS_POLICY_WINDOW_SIZE,
        CU_DEVICE_ATTRIBUTE_GPU_DIRECT_RDMA_WITH_CUDA_VMM_SUPPORTED,
        CU_DEVICE_ATTRIBUTE_RESERVED_SHARED_MEMORY_PER_BLOCK,
        CU_DEVICE_ATTRIBUTE_SPARSE_CUDA_ARRAY_SUPPORTED,
        CU_DEVICE_ATTRIBUTE_READ_ONLY_HOST_REGISTER_SUPPORTED
    ]
}

stringify_enum! {
    CUjit_option,
    [
        CU_JIT_MAX_REGISTERS,
        CU_JIT_THREADS_PER_BLOCK,
        CU_JIT_WALL_TIME,
        CU_JIT_INFO_LOG_BUFFER,
        CU_JIT_INFO_LOG_BUFFER_SIZE_BYTES,
        CU_JIT_ERROR_LOG_BUFFER,
        CU_JIT_ERROR_LOG_BUFFER_SIZE_BYTES,
        CU_JIT_OPTIMIZATION_LEVEL,
        CU_JIT_TARGET_FROM_CUCONTEXT,
        CU_JIT_TARGET,
        CU_JIT_FALLBACK_STRATEGY,
        CU_JIT_GENERATE_DEBUG_INFO,
        CU_JIT_LOG_VERBOSE,
        CU_JIT_GENERATE_LINE_INFO,
        CU_JIT_CACHE_MODE,
        CU_JIT_NEW_SM3X_OPT,
        CU_JIT_FAST_COMPILE,
        CU_JIT_GLOBAL_SYMBOL_NAMES,
        CU_JIT_GLOBAL_SYMBOL_ADDRESSES,
        CU_JIT_GLOBAL_SYMBOL_COUNT,
        CU_JIT_NUM_OPTIONS
    ]
}

stringify_enum! {
    CUresult,
    [
        CUDA_SUCCESS,
        CUDA_ERROR_INVALID_VALUE,
        CUDA_ERROR_OUT_OF_MEMORY,
        CUDA_ERROR_NOT_INITIALIZED,
        CUDA_ERROR_DEINITIALIZED,
        CUDA_ERROR_PROFILER_DISABLED,
        CUDA_ERROR_PROFILER_NOT_INITIALIZED,
        CUDA_ERROR_PROFILER_ALREADY_STARTED,
        CUDA_ERROR_PROFILER_ALREADY_STOPPED,
        CUDA_ERROR_NO_DEVICE,
        CUDA_ERROR_INVALID_DEVICE,
        CUDA_ERROR_INVALID_IMAGE,
        CUDA_ERROR_INVALID_CONTEXT,
        CUDA_ERROR_CONTEXT_ALREADY_CURRENT,
        CUDA_ERROR_MAP_FAILED,
        CUDA_ERROR_UNMAP_FAILED,
        CUDA_ERROR_ARRAY_IS_MAPPED,
        CUDA_ERROR_ALREADY_MAPPED,
        CUDA_ERROR_NO_BINARY_FOR_GPU,
        CUDA_ERROR_ALREADY_ACQUIRED,
        CUDA_ERROR_NOT_MAPPED,
        CUDA_ERROR_NOT_MAPPED_AS_ARRAY,
        CUDA_ERROR_NOT_MAPPED_AS_POINTER,
        CUDA_ERROR_ECC_UNCORRECTABLE,
        CUDA_ERROR_UNSUPPORTED_LIMIT,
        CUDA_ERROR_CONTEXT_ALREADY_IN_USE,
        CUDA_ERROR_PEER_ACCESS_UNSUPPORTED,
        CUDA_ERROR_INVALID_PTX,
        CUDA_ERROR_INVALID_GRAPHICS_CONTEXT,
        CUDA_ERROR_NVLINK_UNCORRECTABLE,
        CUDA_ERROR_JIT_COMPILER_NOT_FOUND,
        CUDA_ERROR_INVALID_SOURCE,
        CUDA_ERROR_FILE_NOT_FOUND,
        CUDA_ERROR_SHARED_OBJECT_SYMBOL_NOT_FOUND,
        CUDA_ERROR_SHARED_OBJECT_INIT_FAILED,
        CUDA_ERROR_OPERATING_SYSTEM,
        CUDA_ERROR_INVALID_HANDLE,
        CUDA_ERROR_ILLEGAL_STATE,
        CUDA_ERROR_NOT_FOUND,
        CUDA_ERROR_NOT_READY,
        CUDA_ERROR_ILLEGAL_ADDRESS,
        CUDA_ERROR_LAUNCH_OUT_OF_RESOURCES,
        CUDA_ERROR_LAUNCH_TIMEOUT,
        CUDA_ERROR_LAUNCH_INCOMPATIBLE_TEXTURING,
        CUDA_ERROR_PEER_ACCESS_ALREADY_ENABLED,
        CUDA_ERROR_PEER_ACCESS_NOT_ENABLED,
        CUDA_ERROR_PRIMARY_CONTEXT_ACTIVE,
        CUDA_ERROR_CONTEXT_IS_DESTROYED,
        CUDA_ERROR_ASSERT,
        CUDA_ERROR_TOO_MANY_PEERS,
        CUDA_ERROR_HOST_MEMORY_ALREADY_REGISTERED,
        CUDA_ERROR_HOST_MEMORY_NOT_REGISTERED,
        CUDA_ERROR_HARDWARE_STACK_ERROR,
        CUDA_ERROR_ILLEGAL_INSTRUCTION,
        CUDA_ERROR_MISALIGNED_ADDRESS,
        CUDA_ERROR_INVALID_ADDRESS_SPACE,
        CUDA_ERROR_INVALID_PC,
        CUDA_ERROR_LAUNCH_FAILED,
        CUDA_ERROR_COOPERATIVE_LAUNCH_TOO_LARGE,
        CUDA_ERROR_NOT_PERMITTED,
        CUDA_ERROR_NOT_SUPPORTED,
        CUDA_ERROR_SYSTEM_NOT_READY,
        CUDA_ERROR_SYSTEM_DRIVER_MISMATCH,
        CUDA_ERROR_COMPAT_NOT_SUPPORTED_ON_DEVICE,
        CUDA_ERROR_STREAM_CAPTURE_UNSUPPORTED,
        CUDA_ERROR_STREAM_CAPTURE_INVALIDATED,
        CUDA_ERROR_STREAM_CAPTURE_MERGE,
        CUDA_ERROR_STREAM_CAPTURE_UNMATCHED,
        CUDA_ERROR_STREAM_CAPTURE_UNJOINED,
        CUDA_ERROR_STREAM_CAPTURE_ISOLATION,
        CUDA_ERROR_STREAM_CAPTURE_IMPLICIT,
        CUDA_ERROR_CAPTURED_EVENT,
        CUDA_ERROR_STREAM_CAPTURE_WRONG_THREAD,
        CUDA_ERROR_TIMEOUT,
        CUDA_ERROR_GRAPH_EXEC_UPDATE_FAILURE,
        CUDA_ERROR_UNKNOWN
    ]
}

stringify_enum! {
    CUdevice_P2PAttribute,
    [
        CU_DEVICE_P2P_ATTRIBUTE_PERFORMANCE_RANK,
        CU_DEVICE_P2P_ATTRIBUTE_ACCESS_SUPPORTED,
        CU_DEVICE_P2P_ATTRIBUTE_NATIVE_ATOMIC_SUPPORTED,
        CU_DEVICE_P2P_ATTRIBUTE_ACCESS_ACCESS_SUPPORTED,
        CU_DEVICE_P2P_ATTRIBUTE_CUDA_ARRAY_ACCESS_SUPPORTED
    ]
}

stringify_enum! {
    CUarray_format,
    [
        CU_AD_FORMAT_UNSIGNED_INT8,
        CU_AD_FORMAT_UNSIGNED_INT16,
        CU_AD_FORMAT_UNSIGNED_INT32,
        CU_AD_FORMAT_SIGNED_INT8,
        CU_AD_FORMAT_SIGNED_INT16,
        CU_AD_FORMAT_SIGNED_INT32,
        CU_AD_FORMAT_HALF,
        CU_AD_FORMAT_FLOAT
    ]
}

stringify_enum! {
    CUresourceViewFormat,
    [
        CU_RES_VIEW_FORMAT_NONE,
        CU_RES_VIEW_FORMAT_UINT_1X8,
        CU_RES_VIEW_FORMAT_UINT_2X8,
        CU_RES_VIEW_FORMAT_UINT_4X8,
        CU_RES_VIEW_FORMAT_SINT_1X8,
        CU_RES_VIEW_FORMAT_SINT_2X8,
        CU_RES_VIEW_FORMAT_SINT_4X8,
        CU_RES_VIEW_FORMAT_UINT_1X16,
        CU_RES_VIEW_FORMAT_UINT_2X16,
        CU_RES_VIEW_FORMAT_UINT_4X16,
        CU_RES_VIEW_FORMAT_SINT_1X16,
        CU_RES_VIEW_FORMAT_SINT_2X16,
        CU_RES_VIEW_FORMAT_SINT_4X16,
        CU_RES_VIEW_FORMAT_UINT_1X32,
        CU_RES_VIEW_FORMAT_UINT_2X32,
        CU_RES_VIEW_FORMAT_UINT_4X32,
        CU_RES_VIEW_FORMAT_SINT_1X32,
        CU_RES_VIEW_FORMAT_SINT_2X32,
        CU_RES_VIEW_FORMAT_SINT_4X32,
        CU_RES_VIEW_FORMAT_FLOAT_1X16,
        CU_RES_VIEW_FORMAT_FLOAT_2X16,
        CU_RES_VIEW_FORMAT_FLOAT_4X16,
        CU_RES_VIEW_FORMAT_FLOAT_1X32,
        CU_RES_VIEW_FORMAT_FLOAT_2X32,
        CU_RES_VIEW_FORMAT_FLOAT_4X32,
        CU_RES_VIEW_FORMAT_UNSIGNED_BC1,
        CU_RES_VIEW_FORMAT_UNSIGNED_BC2,
        CU_RES_VIEW_FORMAT_UNSIGNED_BC3,
        CU_RES_VIEW_FORMAT_UNSIGNED_BC4,
        CU_RES_VIEW_FORMAT_SIGNED_BC4,
        CU_RES_VIEW_FORMAT_UNSIGNED_BC5,
        CU_RES_VIEW_FORMAT_SIGNED_BC5,
        CU_RES_VIEW_FORMAT_UNSIGNED_BC6H,
        CU_RES_VIEW_FORMAT_SIGNED_BC6H,
        CU_RES_VIEW_FORMAT_UNSIGNED_BC7
    ]
}

stringify_enum! {
    CUaddress_mode,
    [
        CU_TR_ADDRESS_MODE_WRAP,
        CU_TR_ADDRESS_MODE_CLAMP,
        CU_TR_ADDRESS_MODE_MIRROR,
        CU_TR_ADDRESS_MODE_BORDER
    ]
}

stringify_enum! {
    CUfilter_mode,
    [
        CU_TR_FILTER_MODE_POINT,
        CU_TR_FILTER_MODE_LINEAR
    ]
}

stringify_enum! {
    CUgraphExecUpdateResult,
    [
        CU_GRAPH_EXEC_UPDATE_SUCCESS,
        CU_GRAPH_EXEC_UPDATE_ERROR,
        CU_GRAPH_EXEC_UPDATE_ERROR_TOPOLOGY_CHANGED,
        CU_GRAPH_EXEC_UPDATE_ERROR_NODE_TYPE_CHANGED,
        CU_GRAPH_EXEC_UPDATE_ERROR_FUNCTION_CHANGED,
        CU_GRAPH_EXEC_UPDATE_ERROR_PARAMETERS_CHANGED,
        CU_GRAPH_EXEC_UPDATE_ERROR_NOT_SUPPORTED
    ]
}

stringify_enum! {
    CUmemorytype,
    [
        CU_MEMORYTYPE_HOST,
        CU_MEMORYTYPE_DEVICE,
        CU_MEMORYTYPE_ARRAY,
        CU_MEMORYTYPE_UNIFIED
    ]
}

stringify_enum! {
    CUmemLocationType,
    [
        CU_MEM_LOCATION_TYPE_INVALID,
        CU_MEM_LOCATION_TYPE_DEVICE
    ]
}

stringify_enum! {
    CUlimit,
    [
        CU_LIMIT_STACK_SIZE,
        CU_LIMIT_PRINTF_FIFO_SIZE,
        CU_LIMIT_MALLOC_HEAP_SIZE,
        CU_LIMIT_DEV_RUNTIME_SYNC_DEPTH,
        CU_LIMIT_DEV_RUNTIME_PENDING_LAUNCH_COUNT,
        CU_LIMIT_MAX_L2_FETCH_GRANULARITY,
        CU_LIMIT_PERSISTING_L2_CACHE_SIZE
    ]
}

stringify_enum! {
    CUpointer_attribute,
    [
        CU_POINTER_ATTRIBUTE_CONTEXT,
        CU_POINTER_ATTRIBUTE_MEMORY_TYPE,
        CU_POINTER_ATTRIBUTE_DEVICE_POINTER,
        CU_POINTER_ATTRIBUTE_HOST_POINTER,
        CU_POINTER_ATTRIBUTE_P2P_TOKENS,
        CU_POINTER_ATTRIBUTE_SYNC_MEMOPS,
        CU_POINTER_ATTRIBUTE_BUFFER_ID,
        CU_POINTER_ATTRIBUTE_IS_MANAGED,
        CU_POINTER_ATTRIBUTE_DEVICE_ORDINAL,
        CU_POINTER_ATTRIBUTE_IS_LEGACY_CUDA_IPC_CAPABLE,
        CU_POINTER_ATTRIBUTE_RANGE_START_ADDR,
        CU_POINTER_ATTRIBUTE_RANGE_SIZE,
        CU_POINTER_ATTRIBUTE_MAPPED,
        CU_POINTER_ATTRIBUTE_ALLOWED_HANDLE_TYPES,
        CU_POINTER_ATTRIBUTE_IS_GPU_DIRECT_RDMA_CAPABLE,
        CU_POINTER_ATTRIBUTE_ACCESS_FLAGS
    ]
}

stringify_enum! {
    CUfunction_attribute,
    [
        CU_FUNC_ATTRIBUTE_MAX_THREADS_PER_BLOCK,
        CU_FUNC_ATTRIBUTE_SHARED_SIZE_BYTES,
        CU_FUNC_ATTRIBUTE_CONST_SIZE_BYTES,
        CU_FUNC_ATTRIBUTE_LOCAL_SIZE_BYTES,
        CU_FUNC_ATTRIBUTE_NUM_REGS,
        CU_FUNC_ATTRIBUTE_PTX_VERSION,
        CU_FUNC_ATTRIBUTE_BINARY_VERSION,
        CU_FUNC_ATTRIBUTE_CACHE_MODE_CA,
        CU_FUNC_ATTRIBUTE_MAX_DYNAMIC_SHARED_SIZE_BYTES,
        CU_FUNC_ATTRIBUTE_PREFERRED_SHARED_MEMORY_CARVEOUT
    ]
}

stringify_enum! {
    CUmem_range_attribute,
    [
        CU_MEM_RANGE_ATTRIBUTE_READ_MOSTLY,
        CU_MEM_RANGE_ATTRIBUTE_PREFERRED_LOCATION,
        CU_MEM_RANGE_ATTRIBUTE_ACCESSED_BY,
        CU_MEM_RANGE_ATTRIBUTE_LAST_PREFETCH_LOCATION
    ]
}

stringify_enum! {
    CUfunc_cache,
    [
        CU_FUNC_CACHE_PREFER_NONE,
        CU_FUNC_CACHE_PREFER_SHARED,
        CU_FUNC_CACHE_PREFER_L1,
        CU_FUNC_CACHE_PREFER_EQUAL
    ]
}

stringify_enum! {
    CUstreamCaptureMode,
    [
        CU_STREAM_CAPTURE_MODE_GLOBAL,
        CU_STREAM_CAPTURE_MODE_THREAD_LOCAL,
        CU_STREAM_CAPTURE_MODE_RELAXED
    ]
}

stringify_enum! {
    CUmem_advise,
    [
        CU_MEM_ADVISE_SET_READ_MOSTLY,
        CU_MEM_ADVISE_UNSET_READ_MOSTLY,
        CU_MEM_ADVISE_SET_PREFERRED_LOCATION,
        CU_MEM_ADVISE_UNSET_PREFERRED_LOCATION,
        CU_MEM_ADVISE_SET_ACCESSED_BY,
        CU_MEM_ADVISE_UNSET_ACCESSED_BY
    ]
}
