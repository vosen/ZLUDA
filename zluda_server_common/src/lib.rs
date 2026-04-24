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
            ContextLocalStoragePut,
            ContextLocalStorageGet,
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
        //cuLaunchKernel,
        // cuMemAlloc_v2,
        //cuMemFreeHost,
        //cuMemFree_v2,
        // cuMemGetAddressRange_v2,
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
        // cuTexRefSetAddress_v2,
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
            type WireObject = u32;
            fn encode(self) -> u32 {
                unsafe { std::mem::transmute_copy::<Self, u32>(&self) }
            }
            fn decode(o: Self::WireObject) -> Self {
                unsafe { std::mem::transmute_copy::<u32, Self>(&o) }
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
encode_as_proxy!(CUdevice_attribute, u32);
encode_as_proxy!(CUfilter_mode, u32);
encode_as_proxy!(CUaddress_mode, u32);
encode_as_proxy!(CUarray_format, u32);

encode_as_u32!(CUcontext);
encode_as_u32!(CUdeviceptr_v2);
encode_as_u32!(CUevent);
encode_as_u32!(CUfunction);
encode_as_u32!(CUmodule);
encode_as_u32!(CUstream);
encode_as_u32!(CUtexref);

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
