use hip_runtime_sys::*;
use optix_types::RTresult;
use std::{
    ffi::{c_void, CStr},
    mem, ptr,
};

use crate::div_positive_round_up;

macro_rules! hip {
    ($expr:expr) => {
        #[allow(unused_unsafe)]
        {
            let err = unsafe { $expr };
            if err != hip_runtime_sys::hipError_t::hipSuccess {
                return Result::Err(err);
            }
        }
    };
}

#[repr(transparent)]
pub(crate) struct Module(pub hipModule_t);

impl Module {
    pub(crate) fn launch_kernel_1d(
        &self,
        f: hipFunction_t,
        size: u32,
        shared_mem: u32,
        stream: hipStream_t,
        params: *mut *mut c_void,
    ) -> Result<(), hipError_t> {
        let groups = div_positive_round_up(size as u64, 32u64);
        hip! { hipModuleLaunchKernel(f, groups as u32, 1, 1, 32, 1, 1, shared_mem, stream, params, ptr::null_mut()) };
        Ok(())
    }

    pub(crate) fn load_data(binary: &[u8]) -> Result<Self, hipError_t> {
        let mut raw_module = ptr::null_mut();
        hip! { hipModuleLoadData(&mut raw_module, binary.as_ptr() as _) };
        Ok(Module(raw_module))
    }

    pub(crate) fn get_function(&self, kernel_name: &CStr) -> Result<hipFunction_t, hipError_t> {
        let mut function = ptr::null_mut();
        hip! { hipModuleGetFunction(&mut function, self.0, kernel_name.as_ptr() as _) };
        Ok(function)
    }

    pub(crate) unsafe fn get_global<T>(&self, name: &CStr) -> Result<T, hipError_t> {
        let (ptr, bytes) = self.get_pointer_to_global(name)?;
        if bytes != mem::size_of::<T>() {
            return Err(hipError_t::hipErrorInvalidSymbol);
        }
        let mut result = mem::zeroed::<T>();
        hip! { hipMemcpyDtoH(&mut result as *mut T as _, ptr, bytes) };
        Ok(result)
    }

    pub(crate) fn get_pointer_to_global(
        &self,
        name: &CStr,
    ) -> Result<(hipDeviceptr_t, usize), hipError_t> {
        let mut ptr = unsafe { mem::zeroed() };
        let mut bytes = 0;
        hip! { hipModuleGetGlobal(&mut ptr, &mut bytes, self.0, name.as_ptr() as _) };
        Ok((ptr, bytes))
    }
}

impl Drop for Module {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        unsafe { hipModuleUnload(self.0) };
    }
}

pub(crate) fn copy_to_device<T>(slice: &[T]) -> Result<hipDeviceptr_t, RTresult> {
    copy_to_device_impl(slice).map_err(|_| RTresult::RT_ERROR_MEMORY_ALLOCATION_FAILED)
}

fn copy_to_device_impl<T>(slice: &[T]) -> Result<hipDeviceptr_t, hipError_t> {
    let dev_ptr = malloc(slice.len() * mem::size_of::<T>())?;
    hip! { hipMemcpyHtoD(dev_ptr, slice.as_ptr() as _, slice.len() * mem::size_of::<T>()) };
    Ok(dev_ptr)
}

pub(crate) fn malloc(size: usize) -> Result<hipDeviceptr_t, hipError_t> {
    let mut dev_ptr = ptr::null_mut();
    hip! { hipMalloc(&mut dev_ptr, size) };
    Ok(hipDeviceptr_t(dev_ptr))
}

pub(crate) fn free(ptr: hipDeviceptr_t) -> Result<(), hipError_t> {
    hip! { hipFree(ptr.0) };
    Ok(())
}

pub(crate) fn zero_fill(ptr: hipDeviceptr_t, size: usize) -> Result<(), hipError_t> {
    hip! { hipMemsetD8(ptr, 0, size) };
    Ok(())
}
