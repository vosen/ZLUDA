use crate::sys;
use std::{
    ffi::c_void,
    fmt::{Debug, Display},
    marker::PhantomData,
    mem, ptr,
};

pub type Result<T> = std::result::Result<T, Error>;

macro_rules! check {
    ($expr:expr) => {
        let err = unsafe { $expr };
        if err != crate::sys::ze_result_t::ZE_RESULT_SUCCESS {
            return Result::Err(Error::err(err));
        }
    };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Error {
    NotReady = 1,
    DeviceLost = 1879048193,
    OutOfHostMemory = 1879048194,
    OutOfDeviceMemory = 1879048195,
    ModuleBuildFailure = 1879048196,
    InsufficientPermissions = 1879113728,
    NotAvailable = 1879113729,
    Uninitialized = 2013265921,
    UnsupportedVersion = 2013265922,
    UnsupportedFeature = 2013265923,
    InvalidArgument = 2013265924,
    InvalidNullHandle = 2013265925,
    HandleObjectInUse = 2013265926,
    InvalidNullPointer = 2013265927,
    InvalidSize = 2013265928,
    UnsupportedSize = 2013265929,
    UnsupportedAlignment = 2013265930,
    InvalidSynchronizationObject = 2013265931,
    InvalidEnumeration = 2013265932,
    UnsupportedEnumeration = 2013265933,
    UnsupportedImageFormat = 2013265934,
    InvalidNativeBinary = 2013265935,
    InvalidGlobalName = 2013265936,
    InvalidKernelName = 2013265937,
    InvalidFunctionName = 2013265938,
    InvalidGroupSizeDimension = 2013265939,
    InvalidGlobalWidthDimension = 2013265940,
    InvalidKernelArgumentIndex = 2013265941,
    InvalidKernelArgumentSize = 2013265942,
    InvalidKernelAttributeValue = 2013265943,
    InvalidCommandListType = 2013265944,
    OverlappingRegions = 2013265945,
    Unknown = 2147483647,
}

impl Error {
    pub fn new<T>(r: sys::ze_result_t, t: T) -> Result<T> {
        Error::new_with(r, || t)
    }

    pub fn new_with<T, F: FnOnce() -> T>(r: sys::ze_result_t, f: F) -> Result<T> {
        match r {
            sys::ze_result_t::ZE_RESULT_SUCCESS => Ok(f()),
            sys::ze_result_t::ZE_RESULT_NOT_READY => Err(Error::NotReady),
            sys::ze_result_t::ZE_RESULT_ERROR_DEVICE_LOST => Err(Error::DeviceLost),
            sys::ze_result_t::ZE_RESULT_ERROR_OUT_OF_HOST_MEMORY => Err(Error::OutOfHostMemory),
            sys::ze_result_t::ZE_RESULT_ERROR_OUT_OF_DEVICE_MEMORY => Err(Error::OutOfDeviceMemory),
            sys::ze_result_t::ZE_RESULT_ERROR_MODULE_BUILD_FAILURE => {
                Err(Error::ModuleBuildFailure)
            }
            sys::ze_result_t::ZE_RESULT_ERROR_INSUFFICIENT_PERMISSIONS => {
                Err(Error::InsufficientPermissions)
            }
            sys::ze_result_t::ZE_RESULT_ERROR_NOT_AVAILABLE => Err(Error::NotAvailable),
            sys::ze_result_t::ZE_RESULT_ERROR_UNINITIALIZED => Err(Error::Uninitialized),
            sys::ze_result_t::ZE_RESULT_ERROR_UNSUPPORTED_VERSION => Err(Error::UnsupportedVersion),
            sys::ze_result_t::ZE_RESULT_ERROR_UNSUPPORTED_FEATURE => Err(Error::UnsupportedFeature),
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_ARGUMENT => Err(Error::InvalidArgument),
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_NULL_HANDLE => Err(Error::InvalidNullHandle),
            sys::ze_result_t::ZE_RESULT_ERROR_HANDLE_OBJECT_IN_USE => Err(Error::HandleObjectInUse),
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_NULL_POINTER => {
                Err(Error::InvalidNullPointer)
            }
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_SIZE => Err(Error::InvalidSize),
            sys::ze_result_t::ZE_RESULT_ERROR_UNSUPPORTED_SIZE => Err(Error::UnsupportedSize),
            sys::ze_result_t::ZE_RESULT_ERROR_UNSUPPORTED_ALIGNMENT => {
                Err(Error::UnsupportedAlignment)
            }
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT => {
                Err(Error::InvalidSynchronizationObject)
            }
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_ENUMERATION => Err(Error::InvalidEnumeration),
            sys::ze_result_t::ZE_RESULT_ERROR_UNSUPPORTED_ENUMERATION => {
                Err(Error::UnsupportedEnumeration)
            }
            sys::ze_result_t::ZE_RESULT_ERROR_UNSUPPORTED_IMAGE_FORMAT => {
                Err(Error::UnsupportedImageFormat)
            }
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_NATIVE_BINARY => {
                Err(Error::InvalidNativeBinary)
            }
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_GLOBAL_NAME => Err(Error::InvalidGlobalName),
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_KERNEL_NAME => Err(Error::InvalidKernelName),
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_FUNCTION_NAME => {
                Err(Error::InvalidFunctionName)
            }
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_GROUP_SIZE_DIMENSION => {
                Err(Error::InvalidGroupSizeDimension)
            }
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_GLOBAL_WIDTH_DIMENSION => {
                Err(Error::InvalidGlobalWidthDimension)
            }
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_KERNEL_ARGUMENT_INDEX => {
                Err(Error::InvalidKernelArgumentIndex)
            }
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_KERNEL_ARGUMENT_SIZE => {
                Err(Error::InvalidKernelArgumentSize)
            }
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_KERNEL_ATTRIBUTE_VALUE => {
                Err(Error::InvalidKernelAttributeValue)
            }
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_COMMAND_LIST_TYPE => {
                Err(Error::InvalidCommandListType)
            }
            sys::ze_result_t::ZE_RESULT_ERROR_OVERLAPPING_REGIONS => Err(Error::OverlappingRegions),
            sys::ze_result_t::ZE_RESULT_ERROR_UNKNOWN => Err(Error::Unknown),
        }
    }

    fn err(r: sys::ze_result_t) -> Self {
        match r {
            sys::ze_result_t::ZE_RESULT_SUCCESS => unreachable!(),
            sys::ze_result_t::ZE_RESULT_NOT_READY => Error::NotReady,
            sys::ze_result_t::ZE_RESULT_ERROR_DEVICE_LOST => Error::DeviceLost,
            sys::ze_result_t::ZE_RESULT_ERROR_OUT_OF_HOST_MEMORY => Error::OutOfHostMemory,
            sys::ze_result_t::ZE_RESULT_ERROR_OUT_OF_DEVICE_MEMORY => Error::OutOfDeviceMemory,
            sys::ze_result_t::ZE_RESULT_ERROR_MODULE_BUILD_FAILURE => Error::ModuleBuildFailure,
            sys::ze_result_t::ZE_RESULT_ERROR_INSUFFICIENT_PERMISSIONS => {
                Error::InsufficientPermissions
            }
            sys::ze_result_t::ZE_RESULT_ERROR_NOT_AVAILABLE => Error::NotAvailable,
            sys::ze_result_t::ZE_RESULT_ERROR_UNINITIALIZED => Error::Uninitialized,
            sys::ze_result_t::ZE_RESULT_ERROR_UNSUPPORTED_VERSION => Error::UnsupportedVersion,
            sys::ze_result_t::ZE_RESULT_ERROR_UNSUPPORTED_FEATURE => Error::UnsupportedFeature,
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_ARGUMENT => Error::InvalidArgument,
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_NULL_HANDLE => Error::InvalidNullHandle,
            sys::ze_result_t::ZE_RESULT_ERROR_HANDLE_OBJECT_IN_USE => Error::HandleObjectInUse,
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_NULL_POINTER => Error::InvalidNullPointer,
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_SIZE => Error::InvalidSize,
            sys::ze_result_t::ZE_RESULT_ERROR_UNSUPPORTED_SIZE => Error::UnsupportedSize,
            sys::ze_result_t::ZE_RESULT_ERROR_UNSUPPORTED_ALIGNMENT => Error::UnsupportedAlignment,
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_SYNCHRONIZATION_OBJECT => {
                Error::InvalidSynchronizationObject
            }
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_ENUMERATION => Error::InvalidEnumeration,
            sys::ze_result_t::ZE_RESULT_ERROR_UNSUPPORTED_ENUMERATION => {
                Error::UnsupportedEnumeration
            }
            sys::ze_result_t::ZE_RESULT_ERROR_UNSUPPORTED_IMAGE_FORMAT => {
                Error::UnsupportedImageFormat
            }
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_NATIVE_BINARY => Error::InvalidNativeBinary,
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_GLOBAL_NAME => Error::InvalidGlobalName,
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_KERNEL_NAME => Error::InvalidKernelName,
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_FUNCTION_NAME => Error::InvalidFunctionName,
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_GROUP_SIZE_DIMENSION => {
                Error::InvalidGroupSizeDimension
            }
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_GLOBAL_WIDTH_DIMENSION => {
                Error::InvalidGlobalWidthDimension
            }
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_KERNEL_ARGUMENT_INDEX => {
                Error::InvalidKernelArgumentIndex
            }
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_KERNEL_ARGUMENT_SIZE => {
                Error::InvalidKernelArgumentSize
            }
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_KERNEL_ATTRIBUTE_VALUE => {
                Error::InvalidKernelAttributeValue
            }
            sys::ze_result_t::ZE_RESULT_ERROR_INVALID_COMMAND_LIST_TYPE => {
                Error::InvalidCommandListType
            }
            sys::ze_result_t::ZE_RESULT_ERROR_OVERLAPPING_REGIONS => Error::OverlappingRegions,
            sys::ze_result_t::ZE_RESULT_ERROR_UNKNOWN => Error::Unknown,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl std::error::Error for Error {}

pub fn init() -> Result<()> {
    Error::new(
        unsafe { sys::zeInit(sys::ze_init_flag_t::ZE_INIT_FLAG_NONE) },
        (),
    )
}

#[repr(transparent)]
pub struct Driver(sys::ze_driver_handle_t);

impl Driver {
    pub unsafe fn as_ffi(&self) -> sys::ze_driver_handle_t {
        self.0
    }
    pub unsafe fn from_ffi(x: sys::ze_driver_handle_t) -> Self {
        Self(x)
    }

    pub fn get() -> Result<Vec<Self>> {
        let mut len = 0;
        let mut temp = ptr::null_mut();
        check!(sys::zeDriverGet(&mut len, &mut temp));
        let mut result = (0..len)
            .map(|_| Driver(ptr::null_mut()))
            .collect::<Vec<_>>();
        check!(sys::zeDriverGet(&mut len, result.as_mut_ptr() as *mut _));
        Ok(result)
    }

    pub fn devices(&self) -> Result<Vec<Device>> {
        let mut len = 0;
        let mut temp = ptr::null_mut();
        check!(sys::zeDeviceGet(self.0, &mut len, &mut temp));
        let mut result = (0..len)
            .map(|_| Device(ptr::null_mut()))
            .collect::<Vec<_>>();
        check!(sys::zeDeviceGet(
            self.0,
            &mut len,
            result.as_mut_ptr() as *mut _
        ));
        Ok(result)
    }
}

#[repr(transparent)]
pub struct Device(sys::ze_device_handle_t);

impl Device {
    pub unsafe fn as_ffi(&self) -> sys::ze_device_handle_t {
        self.0
    }
    pub unsafe fn from_ffi(x: sys::ze_device_handle_t) -> Self {
        Self(x)
    }
}

#[repr(transparent)]
pub struct CommandQueue(sys::ze_command_queue_handle_t);

impl CommandQueue {
    pub unsafe fn as_ffi(&self) -> sys::ze_command_queue_handle_t {
        self.0
    }
    pub unsafe fn from_ffi(x: sys::ze_command_queue_handle_t) -> Self {
        Self(x)
    }

    pub fn new(d: &Device) -> Result<Self> {
        let que_desc = sys::ze_command_queue_desc_t {
            version: sys::ze_command_queue_desc_version_t::ZE_COMMAND_QUEUE_DESC_VERSION_CURRENT,
            flags: sys::ze_command_queue_flag_t::ZE_COMMAND_QUEUE_FLAG_NONE,
            mode: sys::ze_command_queue_mode_t::ZE_COMMAND_QUEUE_MODE_DEFAULT,
            priority: sys::ze_command_queue_priority_t::ZE_COMMAND_QUEUE_PRIORITY_NORMAL,
            ordinal: 0,
        };
        let mut result = ptr::null_mut();
        check!(sys::zeCommandQueueCreate(d.0, &que_desc, &mut result));
        Ok(CommandQueue(result))
    }

    pub fn execute<'a>(&'a self, cmd: CommandList) -> Result<FenceGuard<'a>> {
        check!(sys::zeCommandListClose(cmd.0));
        let result = FenceGuard::new(self, cmd.0)?;
        let mut raw_cmd = cmd.0;
        mem::forget(cmd);
        check!(sys::zeCommandQueueExecuteCommandLists(
            self.0, 1, &mut raw_cmd, result.0
        ));
        Ok(result)
    }
}

impl Drop for CommandQueue {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        unsafe { sys::zeCommandQueueDestroy(self.0) };
    }
}

pub struct Module(sys::ze_module_handle_t);

impl Module {
    pub unsafe fn as_ffi(&self) -> sys::ze_module_handle_t {
        self.0
    }
    pub unsafe fn from_ffi(x: sys::ze_module_handle_t) -> Self {
        Self(x)
    }

    pub fn new_spirv(d: &Device, bin: &[u8], opts: Option<&str>) -> Result<Self> {
        Module::new(true, d, bin, opts)
    }

    pub fn new_native(d: &Device, bin: &[u8]) -> Result<Self> {
        Module::new(false, d, bin, None)
    }

    fn new(spirv: bool, d: &Device, bin: &[u8], opts: Option<&str>) -> Result<Self> {
        let desc = sys::ze_module_desc_t {
            version: sys::ze_module_desc_version_t::ZE_MODULE_DESC_VERSION_CURRENT,
            format: if spirv {
                sys::ze_module_format_t::ZE_MODULE_FORMAT_IL_SPIRV
            } else {
                sys::ze_module_format_t::ZE_MODULE_FORMAT_NATIVE
            },
            inputSize: bin.len(),
            pInputModule: bin.as_ptr(),
            pBuildFlags: opts.map(|s| s.as_ptr() as *const _).unwrap_or(ptr::null()),
            pConstants: ptr::null(),
        };
        let mut result: sys::ze_module_handle_t = ptr::null_mut();
        check!(sys::zeModuleCreate(
            d.0,
            &desc,
            &mut result,
            ptr::null_mut()
        ));
        Ok(Module(result))
    }
}

impl Drop for Module {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        unsafe { sys::zeModuleDestroy(self.0) };
    }
}

pub trait SafeRepr {}
impl SafeRepr for u8 {}
impl SafeRepr for i8 {}
impl SafeRepr for u16 {}
impl SafeRepr for i16 {}
impl SafeRepr for u32 {}
impl SafeRepr for i32 {}
impl SafeRepr for u64 {}
impl SafeRepr for i64 {}
impl SafeRepr for f32 {}
impl SafeRepr for f64 {}

pub struct DeviceBuffer<T: SafeRepr> {
    ptr: *mut c_void,
    driver: sys::ze_driver_handle_t,
    marker: PhantomData<T>,
}

impl<T: SafeRepr> DeviceBuffer<T> {
    pub unsafe fn as_ffi(&self) -> *mut c_void {
        self.ptr
    }
    pub unsafe fn from_ffi(driver: sys::ze_driver_handle_t, ptr: *mut c_void) -> Self {
        let marker = PhantomData::<T>;
        Self {
            ptr,
            driver,
            marker,
        }
    }

    pub fn new(drv: &Driver, dev: &Device, len: usize) -> Result<Self> {
        let desc = sys::_ze_device_mem_alloc_desc_t {
            version:
                sys::ze_device_mem_alloc_desc_version_t::ZE_DEVICE_MEM_ALLOC_DESC_VERSION_CURRENT,
            flags: sys::ze_device_mem_alloc_flag_t::ZE_DEVICE_MEM_ALLOC_FLAG_DEFAULT,
            ordinal: 0,
        };
        let mut result = ptr::null_mut();
        check!(sys::zeDriverAllocDeviceMem(
            drv.0,
            &desc,
            len * mem::size_of::<T>(),
            mem::align_of::<T>(),
            dev.0,
            &mut result
        ));
        Ok(unsafe { Self::from_ffi(drv.0, result) })
    }
}

impl<T: SafeRepr> Drop for DeviceBuffer<T> {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        unsafe { sys::zeDriverFreeMem(self.driver, self.ptr) };
    }
}

pub struct CommandList(sys::ze_command_list_handle_t);

impl CommandList {
    pub unsafe fn as_ffi(&self) -> sys::ze_command_list_handle_t {
        self.0
    }
    pub unsafe fn from_ffi(x: sys::ze_command_list_handle_t) -> Self {
        Self(x)
    }

    pub fn new(dev: &Device) -> Result<Self> {
        let desc = sys::_ze_command_list_desc_t {
            version: sys::ze_command_list_desc_version_t::ZE_COMMAND_LIST_DESC_VERSION_CURRENT,
            flags: sys::ze_command_list_flag_t::ZE_COMMAND_LIST_FLAG_NONE,
        };
        let mut result: sys::ze_command_list_handle_t = ptr::null_mut();
        check!(sys::zeCommandListCreate(dev.0, &desc, &mut result));
        Ok(Self(result))
    }
}

impl Drop for CommandList {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        unsafe { sys::zeCommandListDestroy(self.0) };
    }
}

pub struct FenceGuard<'a>(
    sys::ze_fence_handle_t,
    sys::ze_command_list_handle_t,
    PhantomData<&'a ()>,
);

impl<'a> FenceGuard<'a> {
    fn new(q: &'a CommandQueue, cmd_list: sys::ze_command_list_handle_t) -> Result<Self> {
        let desc = sys::_ze_fence_desc_t {
            version: sys::ze_fence_desc_version_t::ZE_FENCE_DESC_VERSION_CURRENT,
            flags: sys::ze_fence_flag_t::ZE_FENCE_FLAG_NONE,
        };
        let mut result = ptr::null_mut();
        check!(sys::zeFenceCreate(q.0, &desc, &mut result));
        Ok(FenceGuard(result, cmd_list, PhantomData))
    }
}

impl<'a> Drop for FenceGuard<'a> {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        unsafe { sys::zeFenceHostSynchronize(self.0, u32::max_value()) };
        unsafe { sys::zeFenceDestroy(self.0) };
        unsafe { sys::zeCommandListDestroy(self.1) };
    }
}
