use crate::FromCuda;
use std::{mem, path::PathBuf};
use windows::core::Interface;

pub fn self_path() -> Option<PathBuf> {
    zluda_windows::get_module_path_for_function(self_path as *const () as usize).map(PathBuf::from)
}

impl<'a, E: crate::CudaErrorType> FromCuda<'a, *mut cuda_types::cuda::IDXGIAdapter, E>
    for &'a windows::Win32::Graphics::Dxgi::IDXGIAdapter
{
    fn from_cuda(adapter: &'a *mut cuda_types::cuda::IDXGIAdapter) -> Result<Self, E> {
        unsafe {
            windows::Win32::Graphics::Dxgi::IDXGIAdapter::from_raw_borrowed(mem::transmute(adapter))
        }
        .ok_or(E::INVALID_VALUE)
    }
}
