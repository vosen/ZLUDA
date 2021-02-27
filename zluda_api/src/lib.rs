#![cfg(target_os = "windows")]

#[allow(warnings)]
mod nvapi;

use crate::nvapi::NvAPI_Status;
use cuda_types::*;
use nvapi::*;
use once_cell::sync::OnceCell;
use std::{ffi::c_void, io::Write, mem, ptr};
use windows::Win32::Graphics::Direct3D11::*;
use windows::Win32::Graphics::Direct3D12::*;
//use winapi::um::d3d12::{ID3D12Device, ID3D12GraphicsCommandList};

static NVCUDA: OnceCell<libloading::Library> = OnceCell::new();

macro_rules! cuda {
    ($expr:expr) => {
        #[allow(unused_unsafe)]
        {
            let err = unsafe { $expr };
            if err != cuda_types::CUresult::CUDA_SUCCESS {
                return NvAPI_Status::NVAPI_ERROR;
            }
        }
    };
}

#[no_mangle]
pub unsafe extern "system" fn nvapi_QueryInterface(key: u32) -> *mut c_void {
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(r"c:\temp\nvapi.log")
        .unwrap();
    writeln!(file, "{:x}", key).unwrap();
    match key {
        0x150E828 => NvAPI_Initialize as _,
        0xe5ac921f => NvAPI_EnumPhysicalGPUs as _,
        0xD8265D24 => NvAPI_GPU_GetArchInfo as _,
        0x2926AAAD => NvAPI_SYS_GetDriverAndBranchVersion as _,
        0x4b708b54 => NvAPI_D3D_GetCurrentSLIState as _,
        0x6086bd93 => NvAPI_D3D11_IsFatbinPTXSupported as _,
        0xb672be19 => NvAPI_D3D11_CreateCubinComputeShaderWithName as _,
        0x427e236d => NvAPI_D3D11_LaunchCubinShader as _,
        0x70C07832 => NvAPI_D3D12_IsFatbinPTXSupported as _,
        0x1dc7261f => NvAPI_D3D12_CreateCubinComputeShaderWithName as _,
        0x5C52BB86 => NvAPI_D3D12_LaunchCubinShader as _,
        0x89eca416 => NvAPI_D3D11_CreateSamplerState as _,
        _ => ptr::null_mut(),
    }
}

#[allow(non_snake_case)]
unsafe extern "system" fn NvAPI_Initialize() -> NvAPI_Status {
    NVCUDA
        .get_or_try_init(|| libloading::Library::new(r"C:\Windows\System32\nvcuda.dll"))
        .unwrap();
    NvAPI_Status::NVAPI_OK
}

#[allow(non_snake_case)]
unsafe extern "system" fn NvAPI_EnumPhysicalGPUs(
    nvGPUHandle: *mut [NvPhysicalGpuHandle; NVAPI_MAX_PHYSICAL_GPUS as usize],
    pGpuCount: *mut u32,
) -> NvAPI_Status {
    (*nvGPUHandle)[0] = ptr::null_mut();
    *pGpuCount = 1;
    NvAPI_Status::NVAPI_OK
}

#[allow(non_snake_case)]
unsafe extern "system" fn NvAPI_GPU_GetArchInfo(
    _hPhysicalGpu: NvPhysicalGpuHandle,
    pGpuArchInfo: *mut NV_GPU_ARCH_INFO,
) -> NvAPI_Status {
    if pGpuArchInfo == ptr::null_mut() {
        return NvAPI_Status::NVAPI_INVALID_ARGUMENT;
    }
    let version = (*pGpuArchInfo).version;
    if version_check::<NV_GPU_ARCH_INFO>(version, 2).is_none() {
        return NvAPI_Status::NVAPI_INCOMPATIBLE_STRUCT_VERSION;
    }
    *pGpuArchInfo = NV_GPU_ARCH_INFO {
        version,
        __bindgen_anon_1: NV_GPU_ARCH_INFO_V2__bindgen_ty_1 {
            architecture_id: NV_GPU_ARCHITECTURE_ID::NV_GPU_ARCHITECTURE_GA100,
        },
        __bindgen_anon_2: NV_GPU_ARCH_INFO_V2__bindgen_ty_2 {
            implementation_id: NV_GPU_ARCH_IMPLEMENTATION_ID::NV_GPU_ARCH_IMPLEMENTATION_GA104,
        },
        __bindgen_anon_3: NV_GPU_ARCH_INFO_V2__bindgen_ty_3 {
            revision_id: NV_GPU_CHIP_REVISION::NV_GPU_CHIP_REV_A01,
        },
    };
    NvAPI_Status::NVAPI_OK
}

fn version_check<T>(packed_version: u32, nominal_version: u32) -> Option<usize> {
    let version = packed_version >> 16;
    let size = (packed_version & 0xffffu32) as usize;
    if mem::size_of::<T>() > size {
        return None;
    }
    if nominal_version != version {
        return None;
    }
    Some(size)
}

#[allow(non_snake_case)]
unsafe extern "system" fn NvAPI_SYS_GetDriverAndBranchVersion(
    pDriverVersion: *mut u32,
    szBuildBranchString: *mut NvAPI_ShortString,
) -> NvAPI_Status {
    match (pDriverVersion.as_mut(), szBuildBranchString.as_mut()) {
        (Some(driver_version), Some(build_string)) => {
            *driver_version = 51109;
            let name = slice_cast(b"r511_09\0");
            build_string[..name.len()].copy_from_slice(name);
            NvAPI_Status::NVAPI_OK
        }
        _ => NvAPI_Status::NVAPI_INVALID_ARGUMENT,
    }
}

fn slice_cast(t: &'static [u8]) -> &'static [i8] {
    unsafe { std::slice::from_raw_parts(t.as_ptr().cast(), t.len()) }
}

struct Shader {
    function: CUfunction,
    block_x: u32,
    block_y: u32,
    block_z: u32,
}

#[allow(non_snake_case)]
unsafe extern "system" fn NvAPI_D3D12_IsFatbinPTXSupported(
    _pDevice: *const ID3D12Device,
    pSupported: *mut bool,
) -> NvAPI_Status {
    *pSupported = true;
    NvAPI_Status::NVAPI_OK
}

#[allow(non_snake_case)]
unsafe extern "system" fn NvAPI_D3D12_CreateCubinComputeShaderWithName(
    pDevice: *const ID3D12Device,
    pCubin: *const c_void,
    size: u32,
    block_x: u32,
    block_y: u32,
    block_z: u32,
    pShaderName: *const i8,
    phShader: *mut NVDX_ObjectHandle,
) -> NvAPI_Status {
    let load_module_data = NVCUDA
        .get()
        .unwrap()
        .get::<unsafe extern "C" fn(
            module: *mut CUmodule,
            image: *const ::std::os::raw::c_void,
        ) -> CUresult>(b"cuModuleLoadData")
        .unwrap();
    let mut module = ptr::null_mut();
    cuda! { load_module_data(&mut module, pCubin as _) };
    let module_get_function = NVCUDA
        .get()
        .unwrap()
        .get::<unsafe extern "C" fn(
            hfunc: *mut CUfunction,
            hmod: CUmodule,
            name: *const ::std::os::raw::c_char,
        ) -> CUresult>(b"cuModuleGetFunction")
        .unwrap();
    let mut function = ptr::null_mut();
    cuda! { module_get_function(&mut function, module, pShaderName) };
    *phShader = Box::into_raw(Box::new(Shader {
        function,
        block_x,
        block_y,
        block_z,
    })) as _;
    NvAPI_Status::NVAPI_OK
}

#[allow(non_snake_case)]
unsafe extern "system" fn NvAPI_D3D12_LaunchCubinShader(
    pCommandList: *const ID3D12GraphicsCommandList,
    hShader: NVDX_ObjectHandle,
    gridX: u32,
    gridY: u32,
    gridZ: u32,
    pParams: *const c_void,
    paramSize: u32,
) -> NvAPI_Status {
    NvAPI_Status::NVAPI_OK
}

#[allow(non_snake_case)]
unsafe extern "system" fn NvAPI_D3D11_IsFatbinPTXSupported(
    _pDevice: *const ID3D11Device,
    pSupported: *mut bool,
) -> NvAPI_Status {
    *pSupported = true;
    NvAPI_Status::NVAPI_OK
}

#[allow(non_snake_case)]
unsafe extern "system" fn NvAPI_D3D11_CreateCubinComputeShaderWithName(
    _pDevice: *const ID3D11Device,
    pCubin: *const c_void,
    _size: u32,
    block_x: u32,
    block_y: u32,
    block_z: u32,
    pShaderName: *const i8,
    phShader: *mut NVDX_ObjectHandle,
) -> NvAPI_Status {
    let load_module_data = NVCUDA
        .get()
        .unwrap()
        .get::<unsafe extern "C" fn(
            module: *mut CUmodule,
            image: *const ::std::os::raw::c_void,
        ) -> CUresult>(b"cuModuleLoadData")
        .unwrap();
    let mut module = ptr::null_mut();
    cuda! { load_module_data(&mut module, pCubin as _) };
    let module_get_function = NVCUDA
        .get()
        .unwrap()
        .get::<unsafe extern "C" fn(
            hfunc: *mut CUfunction,
            hmod: CUmodule,
            name: *const ::std::os::raw::c_char,
        ) -> CUresult>(b"cuModuleGetFunction")
        .unwrap();
    let mut function = ptr::null_mut();
    cuda! { module_get_function(&mut function, module, pShaderName) };
    *phShader = Box::into_raw(Box::new(Shader {
        function,
        block_x,
        block_y,
        block_z,
    })) as _;
    NvAPI_Status::NVAPI_OK
}

#[allow(non_snake_case)]
unsafe extern "system" fn NvAPI_D3D11_LaunchCubinShader(
    pCommandList: *const ID3D11DeviceContext,
    hShader: NVDX_ObjectHandle,
    gridX: u32,
    gridY: u32,
    gridZ: u32,
    pParams: *const c_void,
    paramSize: u32,
    pReadResources: *const NVDX_ObjectHandle,
    numReadResources: NvU32,
    pWriteResources: *const NVDX_ObjectHandle,
    numWriteResources: NvU32,
) -> NvAPI_Status {
    NvAPI_Status::NVAPI_OK
}

#[allow(non_snake_case)]
unsafe extern "system" fn NvAPI_D3D_GetCurrentSLIState(
    _pDevice: *const IUnknown,
    pSliState: *mut NV_GET_CURRENT_SLI_STATE_V1,
) -> NvAPI_Status {
    let version = (*pSliState).version;
    if let Some(size) = version_check::<NV_GET_CURRENT_SLI_STATE_V1>(version, 1) {
        std::ptr::write_bytes((pSliState as *mut u8).add(4), 0u8, size - 4);
        (*pSliState).maxNumAFRGroups = 1;
        (*pSliState).numAFRGroups = 1;
        (*pSliState).currentAFRIndex = 0;
        (*pSliState).nextFrameAFRIndex = 0;
        (*pSliState).previousFrameAFRIndex = 0;
        (*pSliState).bIsCurAFRGroupNew = 0;
        NvAPI_Status::NVAPI_OK
    } else {
        NvAPI_Status::NVAPI_INCOMPATIBLE_STRUCT_VERSION
    }
}

#[allow(non_snake_case)]
unsafe extern "system" fn NvAPI_D3D11_CreateSamplerState(
    pDevice: *const ID3D11Device,
    pSamplerDesc: *const D3D11_SAMPLER_DESC,
    ppSamplerState: *mut *const ID3D11SamplerState,
    pDriverHandle: *mut u32,
) -> NvAPI_Status {
    let mut state = None;
    (&*pDevice)
        .CreateSamplerState(pSamplerDesc, Some(&mut state))
        .unwrap();
    *ppSamplerState = &state.unwrap();
    *pDriverHandle = 0;
    NvAPI_Status::NVAPI_OK
}
