use cuda_types::cuda::{CUresult, CUuuid};

pub unsafe fn get_dump_table() -> Result<(), libloading::Error> {
    let driver = open_driver()?;
    let export_table_fn = driver.get::<
    unsafe extern "system" fn(
        *mut *const ::core::ffi::c_void,
        *const CUuuid,
    ) -> CUresult>(b"cuGetExportTable\0")?;
    Ok(())
}

fn open_driver() -> Result<libloading::Library, libloading::Error> {
    os::open_driver()
}

#[cfg(unix)]
pub(crate) mod os {
    use libloading::os;

    const RTLD_NOLOAD: i32 = 0x4;

    pub fn open_driver() -> Result<libloading::Library, libloading::Error> {
        unsafe {
            os::unix::Library::open(Some("libcuda.so.1"), RTLD_NOLOAD | os::unix::RTLD_LOCAL)
                .or_else(|_| {
                    os::unix::Library::open(Some("libcuda.so"), RTLD_NOLOAD | os::unix::RTLD_LOCAL)
                })
                .map(Into::into)
        }
    }
}

#[cfg(windows)]
pub(crate) mod os {
    use libloading::os;

    const RTLD_NOLOAD: i32 = 0x4;

    pub fn open_driver() -> Result<libloading::Library, libloading::Error> {
        unsafe { os::windows::Library::open_already_loaded("nvcuda.dll").map(Into::into) }
    }
}
