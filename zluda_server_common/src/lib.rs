use bincode::{Decode, Encode};
use cuda_macros::{cuda_function_declarations, generate_input_struct, generate_output_struct};
use cuda_types::cuda::*;
use paste::paste;
use rkyv::rend::{u32_le, u64_le};
use rkyv::{rend, Archive, Deserialize, Portable, Serialize};
use std::ffi::CString;
use std::num::NonZeroU32;
use std::{ffi::CStr, num::NonZero};
use strum_macros::FromRepr;

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
        pub enum Opcode {
            System = 0,
            $(
                $fn_name,
            )*
            cuDeviceGetName,
            cuDeviceTotalMem_v2,
            cuModuleLoadData,
            ContextLocalStoragePut,
            ContextLocalStorageGet,
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
        cuCtxCreate_v2,
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
        //cuGetExportTable,
        cuInit,
        // cuLaunchKernel,
        //cuMemAlloc_v2,
        //cuMemFreeHost,
        //cuMemFree_v2,
        cuMemGetAddressRange_v2,
        //cuMemHostAlloc,
        //cuMemcpyDtoDAsync_v2,
        //cuMemcpyDtoHAsync_v2,
        //cuMemcpyHtoDAsync_v2,
        // cuMemsetD8_v2,
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
    pub maxThreadsPerBlock: ::core::ffi::c_int,
    pub maxThreadsDim: [::core::ffi::c_int; 3usize],
    pub maxGridSize: [::core::ffi::c_int; 3usize],
    pub sharedMemPerBlock: ::core::ffi::c_int,
    pub totalConstantMemory: ::core::ffi::c_int,
    pub SIMDWidth: ::core::ffi::c_int,
    pub memPitch: ::core::ffi::c_int,
    pub regsPerBlock: ::core::ffi::c_int,
    pub clockRate: ::core::ffi::c_int,
    pub textureAlign: ::core::ffi::c_int,
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
#[derive(Portable, Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct ContextLocalStoragePutIn {
    pub cu_ctx: u32_le,
    pub key: u32_le,
    pub value: u32_le,
}

#[repr(C)]
#[derive(Portable, Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct ContextLocalStoragePutOut {}

#[repr(C)]
#[derive(Portable, Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct ContextLocalStorageGetIn {
    pub cu_ctx: u32_le,
    pub key: u32_le,
}

#[repr(C)]
#[derive(Portable, Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct ContextLocalStorageGetOut {
    pub value: u32_le,
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
