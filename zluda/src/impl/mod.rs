use comgr::{sys::amd_comgr_status_t, Comgr};
use cuda_types::*;
use hip_runtime_sys::*;
use memoffset::offset_of;
use static_assertions::assert_impl_one;
use std::{
    cell::Cell,
    ffi::{c_void, CStr},
    fs,
    mem::{self, ManuallyDrop, MaybeUninit},
    ptr::{self, NonNull},
    sync::{atomic::AtomicI32, Once},
};

use self::cache::KernelCache;

pub(crate) mod array;
pub(crate) mod cache;
pub(crate) mod context;
pub(crate) mod dark_api;
pub(crate) mod device;
pub(crate) mod function;
pub(crate) mod gl;
pub(crate) mod graph;
pub(crate) mod hipfix;
pub(crate) mod library;
pub(crate) mod link;
pub(crate) mod memory;
pub(crate) mod module;
#[cfg_attr(windows, path = "os_win.rs")]
#[cfg_attr(not(windows), path = "os_unix.rs")]
pub(crate) mod os;
pub(crate) mod pointer;
pub(crate) mod stream;
pub(crate) mod surface;
pub(crate) mod surfref;
pub(crate) mod texobj;
pub(crate) mod texref;

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> cuda_types::CUresult {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> cuda_types::CUresult {
    cuda_types::CUresult::CUDA_ERROR_NOT_SUPPORTED
}

#[macro_export]
macro_rules! hip_call {
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

#[macro_export]
macro_rules! hip_call_cuda {
    ($expr:expr) => {
        #[allow(unused_unsafe)]
        {
            use crate::r#impl::IntoCuda;
            let err = unsafe { $expr };
            if err != hip_runtime_sys::hipError_t::hipSuccess {
                return Result::Err(err.into_cuda());
            }
        }
    };
}

static GLOBAL_STATE: Lazy<GlobalState> = Lazy::INIT;

pub(crate) struct GlobalState {
    pub(crate) devices: Vec<device::Device>,
    _dark_api_heap: *mut c_void,
    pub(crate) kernel_cache: Option<KernelCache>,
    pub(crate) comgr: Comgr,
    pub(crate) comgr_version: String,
    pub(crate) zero_buffers: bool,
}
assert_impl_one!(GlobalState: Sync);

impl GlobalState {
    pub(crate) fn device(&self, device: hipDevice_t) -> Result<&device::Device, CUresult> {
        if device < 0 || device as usize >= self.devices.len() {
            Err(CUresult::CUDA_ERROR_INVALID_DEVICE)
        } else {
            Ok(&self.devices[device as usize])
        }
    }
}

unsafe impl Sync for GlobalState {}

pub(crate) trait ZludaObject: Sized {
    const LIVENESS_COOKIE: usize;
    const LIVENESS_FAIL: CUresult;
    // This function exists to support "drop-with-return-value"
    // By default Drop returns nothing, while we want to signal that e.g.
    // cuCtxDestroy returned an error destroying underlying resources
    // * by_owner patameter tells us if the drop comes from CUDA owner
    //   (typically context), in this cane we must skip deregistration
    fn drop_with_result(&mut self, by_owner: bool) -> Result<(), CUresult>;
}

pub(crate) trait HasLivenessCookie: Sized {
    const COOKIE: usize;
    const LIVENESS_FAIL: CUresult;

    fn try_drop(&mut self) -> Result<(), CUresult>;
}

// This struct is a best-effort check if wrapped value has been dropped,
// while it's inherently safe, its use coming from FFI is very unsafe
#[repr(C)]
pub(crate) struct LiveCheck<T: ZludaObject> {
    cookie: usize,
    data: ManuallyDrop<T>,
}

impl<T: ZludaObject> LiveCheck<T> {
    pub fn new(data: T) -> Self {
        LiveCheck {
            cookie: T::LIVENESS_COOKIE,
            data: ManuallyDrop::new(data),
        }
    }

    pub unsafe fn drop_box_with_result(this: *mut Self, by_owner: bool) -> Result<(), CUresult> {
        (&mut *this).try_drop(by_owner)?;
        drop(Box::from_raw(this));
        Ok(())
    }

    unsafe fn from_ref(this: &T) -> NonNull<Self> {
        NonNull::new_unchecked(Self::from_raw(this as *const T as *mut T))
    }

    unsafe fn from_raw(this: *mut T) -> *mut Self {
        let offset = offset_of!(Self, data);
        let outer_ptr = (this as *mut u8).wrapping_sub(offset);
        outer_ptr as *mut Self
    }

    pub unsafe fn as_mut_unchecked(&mut self) -> &mut T {
        &mut self.data
    }

    pub unsafe fn as_result<'a>(this: *mut Self) -> Result<&'a T, CUresult> {
        if this == ptr::null_mut() {
            return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
        }
        if (*this).cookie == T::LIVENESS_COOKIE {
            Ok(&(*this).data)
        } else {
            Err(T::LIVENESS_FAIL)
        }
    }

    #[must_use]
    pub fn try_drop(&mut self, by_owner: bool) -> Result<(), CUresult> {
        if self.cookie == T::LIVENESS_COOKIE {
            self.cookie = 0;
            self.data.drop_with_result(by_owner)?;
            unsafe { ManuallyDrop::drop(&mut self.data) };
            return Ok(());
        }
        Err(T::LIVENESS_FAIL)
    }
}

impl<T: ZludaObject> Drop for LiveCheck<T> {
    fn drop(&mut self) {
        self.cookie = 0;
    }
}

pub(crate) trait FromCuda<T: Sized>: Sized {
    fn from_cuda(t: T) -> Self {
        unsafe { mem::transmute_copy(&t) }
    }
}

impl FromCuda<i8> for i8 {}
impl FromCuda<u8> for u8 {}
impl FromCuda<u16> for u16 {}
impl FromCuda<i32> for i32 {}
impl FromCuda<u32> for u32 {}
impl FromCuda<f32> for f32 {}
impl FromCuda<usize> for usize {}
impl FromCuda<u64> for u64 {}
impl FromCuda<CUuuid> for CUuuid {}
impl FromCuda<CUdevice_attribute> for CUdevice_attribute {}
impl FromCuda<CUdevprop> for CUdevprop {}
impl FromCuda<CUlimit> for CUlimit {}
impl FromCuda<CUfunc_cache> for CUfunc_cache {}
impl FromCuda<CUjit_option> for CUjit_option {}
impl FromCuda<CUfunction_attribute> for CUfunction_attribute {}
// Same layout, but if it's a an array resource it needs an adjustment in hipfix
impl FromCuda<CUDA_MEMCPY2D> for CUDA_MEMCPY2D {}
impl FromCuda<CUDA_MEMCPY3D> for CUDA_MEMCPY3D {}
impl FromCuda<CUDA_ARRAY3D_DESCRIPTOR> for CUDA_ARRAY3D_DESCRIPTOR {}
impl FromCuda<c_void> for c_void {}
impl FromCuda<CUarray> for CUarray {}
impl FromCuda<CUhostFn> for CUhostFn {}
impl FromCuda<CUoccupancyB2DSize> for CUoccupancyB2DSize {}
impl FromCuda<CUdriverProcAddressQueryResult_enum> for CUdriverProcAddressQueryResult_enum {}
impl FromCuda<CUmoduleLoadingMode> for CUmoduleLoadingMode {}
impl FromCuda<CUlibraryOption> for CUlibraryOption {}
impl FromCuda<CUDA_KERNEL_NODE_PARAMS_v1> for CUDA_KERNEL_NODE_PARAMS_v1 {}
impl FromCuda<CUjitInputType> for CUjitInputType {}
impl FromCuda<CUDA_RESOURCE_DESC> for CUDA_RESOURCE_DESC {}
impl FromCuda<CUmipmappedArray> for CUmipmappedArray {}

impl FromCuda<CUcontext> for *mut context::Context {}
impl FromCuda<CUstream> for *mut stream::Stream {}
impl FromCuda<CUdevice> for hipDevice_t {}
impl FromCuda<CUdeviceptr> for hipDeviceptr_t {}
impl FromCuda<CUmodule> for *mut module::Module {}
impl FromCuda<CUlibrary> for *mut library::Library {}
impl FromCuda<CUfunction> for *mut function::Function {}
impl FromCuda<CUlinkState> for *mut link::LinkState {}
impl FromCuda<CUtexref> for *mut textureReference {}
impl FromCuda<CUsurfref> for *mut textureReference {}
impl FromCuda<CUevent> for hipEvent_t {}
impl FromCuda<CUtexObject> for hipTextureObject_t {}
impl FromCuda<CUmemoryPool> for hipMemPool_t {}
// values are compatible
impl FromCuda<CUstreamCaptureStatus> for hipStreamCaptureStatus {}
// values are compatible
impl FromCuda<CUmemPool_attribute> for hipMemPoolAttr {}
// values are compatible
impl FromCuda<CUpointer_attribute> for hipPointer_attribute {}
impl FromCuda<CUfunction_attribute> for hipFunction_attribute {}
impl FromCuda<CUfilter_mode> for hipTextureFilterMode {}
impl FromCuda<CUaddress_mode> for hipTextureAddressMode {}
impl FromCuda<CUarray_format> for hipArray_Format {}
impl FromCuda<CUDA_ARRAY_DESCRIPTOR> for HIP_ARRAY_DESCRIPTOR {}
impl FromCuda<CUDA_ARRAY3D_DESCRIPTOR> for HIP_ARRAY3D_DESCRIPTOR {}
// Same layout, but if it's a an array resource it needs an adjustment in hipfix
// impl FromCuda<CUDA_RESOURCE_DESC> for HIP_RESOURCE_DESC {}
impl FromCuda<CUDA_TEXTURE_DESC> for HIP_TEXTURE_DESC {}
impl FromCuda<CUDA_RESOURCE_VIEW_DESC> for HIP_RESOURCE_VIEW_DESC {}
impl FromCuda<CUfunc_cache> for hipFuncCache_t {}
impl FromCuda<CUgraph> for hipGraph_t {}
impl FromCuda<CUgraphNode> for hipGraphNode_t {}
impl FromCuda<CUgraphExec> for hipGraphExec_t {}
impl FromCuda<CUgraphicsResource> for hipGraphicsResource_t {}
impl FromCuda<CUlimit> for hipLimit_t {}
impl FromCuda<CUsurfObject> for hipSurfaceObject_t {}

impl<From, Into: FromCuda<From>> FromCuda<*mut From> for *mut Into {}
impl<From, Into: FromCuda<From>> FromCuda<*const From> for *const Into {}

pub(crate) fn memcpy2d_from_cuda(this: &CUDA_MEMCPY2D) -> hip_Memcpy2D {
    hip_Memcpy2D {
        srcXInBytes: this.srcXInBytes,
        srcY: this.srcY,
        srcMemoryType: memory_type_from_cuda(this.srcMemoryType),
        srcHost: this.srcHost,
        srcDevice: FromCuda::from_cuda(this.srcDevice),
        srcArray: hipfix::array::get(this.srcArray),
        srcPitch: this.srcPitch,
        dstXInBytes: this.dstXInBytes,
        dstY: this.dstY,
        dstMemoryType: memory_type_from_cuda(this.dstMemoryType),
        dstHost: this.dstHost,
        dstDevice: FromCuda::from_cuda(this.dstDevice),
        dstArray: hipfix::array::get(this.dstArray),
        dstPitch: this.dstPitch,
        WidthInBytes: this.WidthInBytes,
        Height: this.Height,
    }
}

#[macro_export]
macro_rules! try_downcast {
    ($expr:expr, $type_from:ty => $type_to:ty) => {{
        {
            let value = $expr;
            if value <= (<$type_to>::MAX as $type_from) {
                value as $type_to
            } else {
                return Err(CUresult::CUDA_ERROR_NOT_SUPPORTED);
            }
        }
    }};
}

#[allow(non_snake_case)]
pub(crate) fn memcpy3d_from_cuda(this: &CUDA_MEMCPY3D) -> Result<HIP_MEMCPY3D, CUresult> {
    // TODO: remove the casts when HIP fixes it
    let srcXInBytes = try_downcast!(this.srcXInBytes, usize => u32);
    let srcY = try_downcast!(this.srcY, usize => u32);
    let srcZ = try_downcast!(this.srcZ, usize => u32);
    let srcLOD = try_downcast!(this.srcLOD, usize => u32);
    let srcPitch = try_downcast!(this.srcPitch, usize => u32);
    let srcHeight = try_downcast!(this.srcHeight, usize => u32);
    let dstXInBytes = try_downcast!(this.dstXInBytes, usize => u32);
    let dstY = try_downcast!(this.dstY, usize => u32);
    let dstZ = try_downcast!(this.dstZ, usize => u32);
    let dstLOD = try_downcast!(this.dstLOD, usize => u32);
    let dstPitch = try_downcast!(this.dstPitch, usize => u32);
    let dstHeight = try_downcast!(this.dstHeight, usize => u32);
    let WidthInBytes = try_downcast!(this.WidthInBytes, usize => u32);
    let Height = try_downcast!(this.Height, usize => u32);
    let Depth = try_downcast!(this.Depth, usize => u32);
    Ok(HIP_MEMCPY3D {
        srcXInBytes,
        srcY,
        srcZ,
        srcLOD,
        srcMemoryType: memory_type_from_cuda(this.srcMemoryType),
        srcHost: this.srcHost,
        srcDevice: FromCuda::from_cuda(this.srcDevice),
        srcArray: hipfix::array::get(this.srcArray),
        srcPitch,
        srcHeight,
        dstXInBytes,
        dstY,
        dstZ,
        dstLOD,
        dstMemoryType: memory_type_from_cuda(this.dstMemoryType),
        dstHost: this.dstHost,
        dstDevice: FromCuda::from_cuda(this.dstDevice),
        dstArray: hipfix::array::get(this.dstArray),
        dstPitch,
        dstHeight,
        WidthInBytes,
        Height,
        Depth,
    })
}

pub(crate) fn memory_type_from_cuda(this: CUmemorytype) -> hipMemoryType {
    match this {
        CUmemorytype::CU_MEMORYTYPE_HOST => hipMemoryType::hipMemoryTypeHost,
        CUmemorytype::CU_MEMORYTYPE_DEVICE => hipMemoryType::hipMemoryTypeDevice,
        CUmemorytype::CU_MEMORYTYPE_ARRAY => hipMemoryType::hipMemoryTypeArray,
        CUmemorytype::CU_MEMORYTYPE_UNIFIED => hipMemoryType::hipMemoryTypeUnified,
        CUmemorytype(val) => hipMemoryType(val - 1),
    }
}

impl FromCuda<CUresult> for hipError_t {
    fn from_cuda(this: CUresult) -> hipError_t {
        hipError_t(this.0)
    }
}

pub(crate) trait IntoCuda {
    fn into_cuda(self) -> CUresult;
}

impl IntoCuda for CUresult {
    fn into_cuda(self) -> CUresult {
        self
    }
}

impl IntoCuda for () {
    fn into_cuda(self) -> CUresult {
        CUresult::CUDA_SUCCESS
    }
}

pub(crate) fn comgr_error_to_cuda(this: amd_comgr_status_t) -> CUresult {
    match this {
        amd_comgr_status_t::AMD_COMGR_STATUS_ERROR_INVALID_ARGUMENT => {
            CUresult::CUDA_ERROR_INVALID_VALUE
        }
        amd_comgr_status_t::AMD_COMGR_STATUS_ERROR_OUT_OF_RESOURCES => {
            CUresult::CUDA_ERROR_OUT_OF_MEMORY
        }
        _ => CUresult::CUDA_ERROR_UNKNOWN,
    }
}

impl<T1: IntoCuda, T2: IntoCuda> IntoCuda for Result<T1, T2> {
    fn into_cuda(self) -> CUresult {
        match self {
            Ok(e) => e.into_cuda(),
            Err(e) => e.into_cuda(),
        }
    }
}

impl IntoCuda for hipError_t {
    fn into_cuda(self) -> CUresult {
        if self.0 >= hipError_t::hipErrorUnknown.0 {
            CUresult::CUDA_ERROR_UNKNOWN
        } else {
            CUresult(self.0 as i32)
        }
    }
}

fn fold_cuda_errors(iter: impl Iterator<Item = Result<(), CUresult>>) -> Result<(), CUresult> {
    iter.fold(Ok(()), Result::and)
}

// very similar to lazy_static implementation, but more suitable to our use
struct Lazy<T: Sync> {
    once: Once,
    value: Cell<MaybeUninit<T>>,
}

unsafe impl<T: Sync> Sync for Lazy<T> {}

impl<T: Sync> Lazy<T> {
    const INIT: Self = Lazy {
        once: Once::new(),
        value: Cell::new(MaybeUninit::uninit()),
    };

    fn init(&self, ctor: impl FnOnce() -> T) {
        self.once.call_once(|| {
            self.value.set(MaybeUninit::new(ctor()));
        });
    }

    fn is_initalized(&self) -> bool {
        self.once.is_completed()
    }

    fn get<'a>(&'a self) -> Result<&'a T, CUresult> {
        if self.once.is_completed() {
            Ok(unsafe { &*(&*self.value.as_ptr()).as_ptr() })
        } else {
            Err(CUresult::CUDA_ERROR_NOT_INITIALIZED)
        }
    }
}

pub(crate) fn init(flags: u32) -> Result<(), CUresult> {
    if GLOBAL_STATE.is_initalized() {
        return Ok(());
    }
    let comgr = Comgr::find_and_load().map_err(comgr_error_to_cuda)?;
    let comgr_version = comgr.version().map_err(comgr_error_to_cuda)?;
    hip_call_cuda!(hipInit(flags));
    let mut dev_count = 0;
    hip_call_cuda!(hipGetDeviceCount(&mut dev_count));
    let devices = (0..dev_count as usize)
        .map(|index| device::Device::new(index))
        .collect::<Result<Vec<_>, _>>()?;
    let global_heap = unsafe { os::heap_create() };
    if global_heap == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_OUT_OF_MEMORY);
    }
    let kernel_cache = create_default_cache();
    let zero_buffers = hipfix::should_zero_buffers().unwrap_or(false);
    GLOBAL_STATE.init(|| GlobalState {
        devices,
        kernel_cache,
        _dark_api_heap: global_heap,
        comgr,
        comgr_version,
        zero_buffers,
    });
    Ok(())
}

fn create_default_cache() -> Option<KernelCache> {
    let mut disk_cache_location = dirs::cache_dir()?;
    disk_cache_location.push("ZLUDA");
    disk_cache_location.push("ComputeCache");
    fs::create_dir_all(&disk_cache_location).ok()?;
    KernelCache::new(&disk_cache_location)
}

pub(crate) static MAXIMUM_PROC_VERSION: AtomicI32 = AtomicI32::new(0);

pub(crate) unsafe fn get_proc_address_v2(
    symbol: *const ::std::os::raw::c_char,
    pfn: *mut *mut ::std::os::raw::c_void,
    cuda_version: ::std::os::raw::c_int,
    flags: cuuint64_t,
    symbol_status: *mut CUdriverProcAddressQueryResult,
) -> CUresult {
    if symbol == ptr::null() || pfn == ptr::null_mut() {
        return CUresult::CUDA_ERROR_INVALID_VALUE;
    }
    MAXIMUM_PROC_VERSION.fetch_max(cuda_version, std::sync::atomic::Ordering::SeqCst);
    let symbol = unsafe { CStr::from_ptr(symbol) };
    let fn_ptr = get_proc_address(symbol.to_bytes(), flags, cuda_version as u32);
    let (status, result) = if fn_ptr == ptr::null_mut() {
        (
            CUdriverProcAddressQueryResult::CU_GET_PROC_ADDRESS_SYMBOL_NOT_FOUND,
            CUresult::CUDA_ERROR_NOT_FOUND,
        )
    } else if fn_ptr == usize::MAX as _ {
        (
            CUdriverProcAddressQueryResult::CU_GET_PROC_ADDRESS_VERSION_NOT_SUFFICIENT,
            CUresult::CUDA_ERROR_NOT_FOUND,
        )
    } else {
        *pfn = fn_ptr;
        (
            CUdriverProcAddressQueryResult::CU_GET_PROC_ADDRESS_SUCCESS,
            CUresult::CUDA_SUCCESS,
        )
    };
    if let Some(symbol_status) = symbol_status.as_mut() {
        *symbol_status = status;
    }
    result
}

fn get_proc_address(name: &[u8], flag: u64, version: u32) -> *mut ::std::os::raw::c_void {
    use crate::cuda::*;
    include!("../../../process_address_table/table.rs")
}
