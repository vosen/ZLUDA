use cuda_types::cuda::*;
use hip_runtime_sys::*;
use windows::Win32::Graphics::Dxgi::IDXGIAdapter;

use crate::r#impl::driver::global_state;

pub(crate) unsafe fn get_device(device: &mut CUdevice, adapter: &IDXGIAdapter) -> CUresult {
    let dx_desc = adapter.GetDesc().map_err(|_| CUerror::UNKNOWN)?;
    let device_count = global_state()?.devices.len();
    for dev in 0..device_count {
        let mut props = std::mem::zeroed();
        hipGetDevicePropertiesR0600(&mut props, dev as i32)?;
        if props.luid == std::mem::transmute::<_, [i8; 8]>(dx_desc.AdapterLuid) {
            *device = dev as i32;
            return Ok(());
        }
    }
    Err(CUerror::NOT_FOUND)
}
