use crate::sys;
use std::{
    ffi::{c_void, CStr, CString},
    fmt::Debug,
    marker::PhantomData,
    mem, ptr,
};

macro_rules! check {
    ($expr:expr) => {
        #[allow(unused_unsafe)]
        {
            let err = unsafe { $expr };
            if err != crate::sys::ze_result_t::ZE_RESULT_SUCCESS {
                return Result::Err(err);
            }
        }
    };
}

macro_rules! check_panic {
    ($expr:expr) => {
        let err = unsafe { $expr };
        if err != crate::sys::ze_result_t::ZE_RESULT_SUCCESS {
            panic!(err);
        }
    };
}

pub type Result<T> = std::result::Result<T, sys::ze_result_t>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Error(pub sys::ze_result_t);

pub fn init() -> Result<()> {
    match unsafe { sys::zeInit(sys::ze_init_flags_t::ZE_INIT_FLAG_GPU_ONLY) } {
        sys::ze_result_t::ZE_RESULT_SUCCESS => Ok(()),
        e => Err(e),
    }
}

#[repr(transparent)]
pub struct Driver(sys::ze_driver_handle_t);

unsafe impl Send for Driver {}
unsafe impl Sync for Driver {}

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
        if (len as usize) < result.len() {
            result.truncate(len as usize);
        }
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

    pub fn get_properties(&self) -> Result<Box<sys::ze_device_properties_t>> {
        let mut props = Box::new(unsafe { mem::zeroed::<sys::ze_device_properties_t>() });
        check! { sys::zeDeviceGetProperties(self.0, props.as_mut()) };
        Ok(props)
    }

    pub fn get_image_properties(&self) -> Result<Box<sys::ze_device_image_properties_t>> {
        let mut props = Box::new(unsafe { mem::zeroed::<sys::ze_device_image_properties_t>() });
        check! { sys::zeDeviceGetImageProperties(self.0,  props.as_mut()) };
        Ok(props)
    }

    pub fn get_memory_properties(&self) -> Result<Vec<sys::ze_device_memory_properties_t>> {
        let mut count = 0u32;
        check! { sys::zeDeviceGetMemoryProperties(self.0, &mut count, ptr::null_mut()) };
        if count == 0 {
            return Ok(Vec::new());
        }
        let mut props =
            vec![unsafe { mem::zeroed::<sys::ze_device_memory_properties_t>() }; count as usize];
        check! { sys::zeDeviceGetMemoryProperties(self.0, &mut count, props.as_mut_ptr()) };
        Ok(props)
    }

    pub fn get_compute_properties(&self) -> Result<Box<sys::ze_device_compute_properties_t>> {
        let mut props = Box::new(unsafe { mem::zeroed::<sys::ze_device_compute_properties_t>() });
        check! { sys::zeDeviceGetComputeProperties(self.0, props.as_mut()) };
        Ok(props)
    }

    pub unsafe fn mem_alloc_device(
        &mut self,
        ctx: &mut Context,
        size: usize,
        alignment: usize,
    ) -> Result<*mut c_void> {
        let descr = sys::ze_device_mem_alloc_desc_t {
            stype: sys::ze_structure_type_t::ZE_STRUCTURE_TYPE_DEVICE_MEM_ALLOC_DESC,
            pNext: ptr::null(),
            flags: sys::ze_device_mem_alloc_flags_t(0),
            ordinal: 0,
        };
        let mut result = ptr::null_mut();
        // TODO: check current context for the device
        check! {
            sys::zeMemAllocDevice(
                ctx.0,
                &descr,
                size,
                alignment,
                self.0,
                &mut result,
            )
        };
        Ok(result)
    }
}

#[repr(transparent)]
pub struct Context(sys::ze_context_handle_t);

impl Context {
    pub unsafe fn as_ffi(&self) -> sys::ze_context_handle_t {
        self.0
    }
    pub unsafe fn from_ffi(x: sys::ze_context_handle_t) -> Self {
        Self(x)
    }

    pub fn new(drv: &Driver) -> Result<Self> {
        let ctx_desc = sys::ze_context_desc_t {
            stype: sys::ze_structure_type_t::ZE_STRUCTURE_TYPE_CONTEXT_DESC,
            pNext: ptr::null(),
            flags: sys::ze_context_flags_t(0),
        };
        let mut result = ptr::null_mut();
        check!(sys::zeContextCreate(drv.0, &ctx_desc, &mut result));
        Ok(Context(result))
    }

    pub unsafe fn mem_free(&mut self, ptr: *mut c_void) -> Result<()> {
        check! {
            sys::zeMemFree(
                self.0,
                ptr,
            )
        };
        Ok(())
    }
}

impl Drop for Context {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        check_panic! { sys::zeContextDestroy(self.0) };
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

    pub fn new(ctx: &mut Context, d: &Device) -> Result<Self> {
        let que_desc = sys::ze_command_queue_desc_t {
            stype: sys::ze_structure_type_t::ZE_STRUCTURE_TYPE_COMMAND_QUEUE_DESC,
            pNext: ptr::null(),
            ordinal: 0,
            index: 0,
            flags: sys::ze_command_queue_flags_t(0),
            mode: sys::ze_command_queue_mode_t::ZE_COMMAND_QUEUE_MODE_DEFAULT,
            priority: sys::ze_command_queue_priority_t::ZE_COMMAND_QUEUE_PRIORITY_NORMAL,
        };
        let mut result = ptr::null_mut();
        check!(sys::zeCommandQueueCreate(
            ctx.0,
            d.0,
            &que_desc,
            &mut result
        ));
        Ok(CommandQueue(result))
    }

    pub fn execute<'a>(&'a self, cmd: CommandList) -> Result<FenceGuard<'a>> {
        check!(sys::zeCommandListClose(cmd.0));
        let result = FenceGuard::new(self, cmd.0)?;
        let mut raw_cmd = cmd.0;
        mem::forget(cmd);
        check!(sys::zeCommandQueueExecuteCommandLists(
            self.0,
            1,
            &mut raw_cmd,
            result.0
        ));
        Ok(result)
    }
}

impl Drop for CommandQueue {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        check_panic! { sys::zeCommandQueueDestroy(self.0) };
    }
}

pub struct Module(sys::ze_module_handle_t);

impl Module {
    // HACK ALERT
    // We use OpenCL for now to do SPIR-V linking, because Level0
    // does not allow linking. Don't let presence of zeModuleDynamicLink fool
    // you, it's not currently possible to create non-compiled modules.
    // zeModuleCreate always compiles (builds and links).
    pub fn build_link_spirv<'a>(
        ctx: &mut Context,
        d: &Device,
        binaries: &[&'a [u8]],
        opts: Option<&CStr>,
    ) -> (Result<Self>, Option<BuildLog>) {
        let ocl_program = match Self::build_link_spirv_impl(binaries, opts) {
            Err(_) => {
                return (
                    Err(sys::ze_result_t::ZE_RESULT_ERROR_MODULE_LINK_FAILURE),
                    None,
                )
            }
            Ok(prog) => prog,
        };
        match ocl_core::get_program_info(&ocl_program, ocl_core::ProgramInfo::Binaries) {
            Ok(ocl_core::ProgramInfoResult::Binaries(binaries)) => {
                let (module, build_log) = Self::build_native(ctx, d, &binaries[0]);
                (module, Some(build_log))
            }
            _ => return (Err(sys::ze_result_t::ZE_RESULT_ERROR_UNKNOWN), None),
        }
    }

    fn build_link_spirv_impl<'a>(
        binaries: &[&'a [u8]],
        opts: Option<&CStr>,
    ) -> ocl_core::Result<ocl_core::Program> {
        let platforms = ocl_core::get_platform_ids()?;
        let (platform, device) = platforms
            .iter()
            .find_map(|plat| {
                let devices =
                    ocl_core::get_device_ids(plat, Some(ocl_core::DeviceType::GPU), None).ok()?;
                for dev in devices {
                    let vendor =
                        ocl_core::get_device_info(dev, ocl_core::DeviceInfo::VendorId).ok()?;
                    if let ocl_core::DeviceInfoResult::VendorId(0x8086) = vendor {
                        let dev_type =
                            ocl_core::get_device_info(dev, ocl_core::DeviceInfo::Type).ok()?;
                        if let ocl_core::DeviceInfoResult::Type(ocl_core::DeviceType::GPU) =
                            dev_type
                        {
                            return Some((plat.clone(), dev));
                        }
                    }
                }
                None
            })
            .ok_or("")?;
        let ctx_props = ocl_core::ContextProperties::new().platform(platform);
        let ocl_ctx = ocl_core::create_context_from_type::<ocl_core::DeviceId>(
            Some(&ctx_props),
            ocl_core::DeviceType::GPU,
            None,
            None,
        )?;
        let mut programs = Vec::with_capacity(binaries.len());
        for binary in binaries {
            programs.push(ocl_core::create_program_with_il(&ocl_ctx, binary, None)?);
        }
        let options = match opts {
            Some(o) => o.to_owned(),
            None => CString::default(),
        };
        for program in programs.iter() {
            ocl_core::compile_program(
                program,
                Some(&[device]),
                &options,
                &[],
                &[],
                None,
                None,
                None,
            )?;
        }
        ocl_core::link_program::<ocl_core::DeviceId, _>(
            &ocl_ctx,
            Some(&[device]),
            &options,
            &programs.iter().collect::<Vec<_>>(),
            None,
            None,
            None,
        )
    }

    pub fn build_spirv(
        ctx: &mut Context,
        d: &Device,
        bin: &[u8],
        opts: Option<&CStr>,
    ) -> (Result<Self>, BuildLog) {
        Module::new(ctx, true, d, bin, opts)
    }

    pub fn build_native(ctx: &mut Context, d: &Device, bin: &[u8]) -> (Result<Self>, BuildLog) {
        Module::new(ctx, false, d, bin, None)
    }

    fn new(
        ctx: &mut Context,
        spirv: bool,
        d: &Device,
        bin: &[u8],
        opts: Option<&CStr>,
    ) -> (Result<Self>, BuildLog) {
        let desc = sys::ze_module_desc_t {
            stype: sys::ze_structure_type_t::ZE_STRUCTURE_TYPE_MODULE_DESC,
            pNext: ptr::null(),
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
        let mut log_handle = ptr::null_mut();
        let err = unsafe { sys::zeModuleCreate(ctx.0, d.0, &desc, &mut result, &mut log_handle) };
        let log = BuildLog(log_handle);
        if err != crate::sys::ze_result_t::ZE_RESULT_SUCCESS {
            (Result::Err(err), log)
        } else {
            (Ok(Module(result)), log)
        }
    }
}

impl Drop for Module {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        check_panic! { sys::zeModuleDestroy(self.0) };
    }
}

pub struct BuildLog(sys::ze_module_build_log_handle_t);

impl BuildLog {
    pub unsafe fn as_ffi(&self) -> sys::ze_module_build_log_handle_t {
        self.0
    }
    pub unsafe fn from_ffi(x: sys::ze_module_build_log_handle_t) -> Self {
        Self(x)
    }

    pub fn get_cstring(&self) -> Result<CString> {
        let mut size = 0;
        check! { sys::zeModuleBuildLogGetString(self.0, &mut size, ptr::null_mut()) };
        let mut str_vec = vec![0u8; size];
        check! { sys::zeModuleBuildLogGetString(self.0, &mut size, str_vec.as_mut_ptr() as *mut i8) };
        str_vec.pop();
        Ok(CString::new(str_vec).map_err(|_| sys::ze_result_t::ZE_RESULT_ERROR_UNKNOWN)?)
    }
}

impl Drop for BuildLog {
    fn drop(&mut self) {
        check_panic!(sys::zeModuleBuildLogDestroy(self.0));
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
    ctx: sys::ze_context_handle_t,
    len: usize,
    marker: PhantomData<T>,
}

impl<T: SafeRepr> DeviceBuffer<T> {
    pub unsafe fn as_ffi(&self) -> *mut c_void {
        self.ptr
    }
    pub unsafe fn from_ffi(ctx: sys::ze_context_handle_t, ptr: *mut c_void, len: usize) -> Self {
        let marker = PhantomData::<T>;
        Self {
            ptr,
            ctx,
            len,
            marker,
        }
    }

    pub fn new(ctx: &mut Context, dev: &Device, len: usize) -> Result<Self> {
        let desc = sys::_ze_device_mem_alloc_desc_t {
            stype: sys::ze_structure_type_t::ZE_STRUCTURE_TYPE_DEVICE_MEM_ALLOC_DESC,
            pNext: ptr::null(),
            flags: sys::ze_device_mem_alloc_flags_t(0),
            ordinal: 0,
        };
        let mut result = ptr::null_mut();
        check!(sys::zeMemAllocDevice(
            ctx.0,
            &desc,
            len * mem::size_of::<T>(),
            mem::align_of::<T>(),
            dev.0,
            &mut result
        ));
        Ok(unsafe { Self::from_ffi(ctx.0, result, len) })
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T: SafeRepr> Drop for DeviceBuffer<T> {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        check_panic! { sys::zeMemFree(self.ctx, self.ptr) };
    }
}

pub struct CommandList<'a>(sys::ze_command_list_handle_t, PhantomData<&'a ()>);

impl<'a> CommandList<'a> {
    pub unsafe fn as_ffi(&self) -> sys::ze_command_list_handle_t {
        self.0
    }
    pub unsafe fn from_ffi(x: sys::ze_command_list_handle_t) -> Self {
        Self(x, PhantomData)
    }

    pub fn new(ctx: &mut Context, dev: &Device) -> Result<Self> {
        let desc = sys::ze_command_list_desc_t {
            stype: sys::_ze_structure_type_t::ZE_STRUCTURE_TYPE_COMMAND_LIST_DESC,
            commandQueueGroupOrdinal: 0,
            pNext: ptr::null(),
            flags: sys::ze_command_list_flags_t(0),
        };
        let mut result: sys::ze_command_list_handle_t = ptr::null_mut();
        check!(sys::zeCommandListCreate(ctx.0, dev.0, &desc, &mut result));
        Ok(Self(result, PhantomData))
    }

    pub fn append_memory_copy<
        T: 'a,
        Dst: Into<BufferPtrMut<'a, T>>,
        Src: Into<BufferPtr<'a, T>>,
    >(
        &mut self,
        dst: Dst,
        src: Src,
        signal: Option<&mut Event<'a>>,
        wait: &mut [Event<'a>],
    ) -> Result<()> {
        let dst = dst.into();
        let src = src.into();
        let elements = std::cmp::min(dst.len(), src.len());
        let length = elements * mem::size_of::<T>();
        unsafe { self.append_memory_copy_unsafe(dst.get(), src.get(), length, signal, wait) }
    }

    pub unsafe fn append_memory_copy_unsafe(
        &mut self,
        dst: *mut c_void,
        src: *const c_void,
        length: usize,
        signal: Option<&mut Event<'a>>,
        wait: &mut [Event<'a>],
    ) -> Result<()> {
        let signal_event = signal.map(|e| e.0).unwrap_or(ptr::null_mut());
        let (wait_len, wait_ptr) = Event::raw_slice(wait);
        check!(sys::zeCommandListAppendMemoryCopy(
            self.0,
            dst,
            src,
            length,
            signal_event,
            wait_len,
            wait_ptr
        ));
        Ok(())
    }

    pub fn append_memory_fill<T>(
        &mut self,
        dst: BufferPtrMut<'a, T>,
        pattern: u8,
        signal: Option<&mut Event<'a>>,
        wait: &mut [Event<'a>],
    ) -> Result<()> {
        let raw_pattern = &pattern as *const u8 as *const _;
        let signal_event = signal.map(|e| e.0).unwrap_or(ptr::null_mut());
        let (wait_len, wait_ptr) = unsafe { Event::raw_slice(wait) };
        let byte_len = dst.len() * mem::size_of::<T>();
        check!(sys::zeCommandListAppendMemoryFill(
            self.0,
            dst.get(),
            raw_pattern,
            mem::size_of::<u8>(),
            byte_len,
            signal_event,
            wait_len,
            wait_ptr
        ));
        Ok(())
    }

    pub unsafe fn append_memory_fill_unsafe<T: Copy + Sized>(
        &mut self,
        dst: *mut c_void,
        pattern: &T,
        byte_size: usize,
        signal: Option<&mut Event<'a>>,
        wait: &mut [Event<'a>],
    ) -> Result<()> {
        let signal_event = signal.map(|e| e.0).unwrap_or(ptr::null_mut());
        let (wait_len, wait_ptr) = Event::raw_slice(wait);
        check!(sys::zeCommandListAppendMemoryFill(
            self.0,
            dst,
            pattern as *const T as *const _,
            mem::size_of::<T>(),
            byte_size,
            signal_event,
            wait_len,
            wait_ptr
        ));
        Ok(())
    }

    pub fn append_launch_kernel(
        &mut self,
        kernel: &'a Kernel,
        group_count: &[u32; 3],
        signal: Option<&mut Event<'a>>,
        wait: &mut [Event<'a>],
    ) -> Result<()> {
        let gr_count = sys::ze_group_count_t {
            groupCountX: group_count[0],
            groupCountY: group_count[1],
            groupCountZ: group_count[2],
        };
        let signal_event = signal.map(|e| e.0).unwrap_or(ptr::null_mut());
        let (wait_len, wait_ptr) = unsafe { Event::raw_slice(wait) };
        check!(sys::zeCommandListAppendLaunchKernel(
            self.0,
            kernel.0,
            &gr_count,
            signal_event,
            wait_len,
            wait_ptr,
        ));
        Ok(())
    }
}

impl<'a> Drop for CommandList<'a> {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        check_panic! { sys::zeCommandListDestroy(self.0) };
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
            stype: sys::ze_structure_type_t::ZE_STRUCTURE_TYPE_FENCE_DESC,
            pNext: ptr::null(),
            flags: sys::ze_fence_flags_t(0),
        };
        let mut result = ptr::null_mut();
        check!(sys::zeFenceCreate(q.0, &desc, &mut result));
        Ok(FenceGuard(result, cmd_list, PhantomData))
    }
}

impl<'a> Drop for FenceGuard<'a> {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        check_panic! { sys::zeFenceHostSynchronize(self.0, u64::max_value()) };
        check_panic! { sys::zeFenceDestroy(self.0) };
        check_panic! { sys::zeCommandListDestroy(self.1) };
    }
}

#[derive(Copy, Clone)]
pub struct BufferPtr<'a, T> {
    ptr: *const c_void,
    marker: PhantomData<&'a T>,
    elems: usize,
}

impl<'a, T> BufferPtr<'a, T> {
    pub unsafe fn get(self) -> *const c_void {
        return self.ptr;
    }

    pub fn len(&self) -> usize {
        self.elems
    }
}

impl<'a, T> From<&'a [T]> for BufferPtr<'a, T> {
    fn from(s: &'a [T]) -> Self {
        BufferPtr {
            ptr: s.as_ptr() as *const _,
            marker: PhantomData,
            elems: s.len(),
        }
    }
}

impl<'a, T: SafeRepr> From<&'a DeviceBuffer<T>> for BufferPtr<'a, T> {
    fn from(b: &'a DeviceBuffer<T>) -> Self {
        BufferPtr {
            ptr: b.ptr as *const _,
            marker: PhantomData,
            elems: b.len(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct BufferPtrMut<'a, T> {
    ptr: *mut c_void,
    marker: PhantomData<&'a mut T>,
    elems: usize,
}

impl<'a, T> BufferPtrMut<'a, T> {
    pub unsafe fn get(self) -> *mut c_void {
        return self.ptr;
    }

    pub fn len(&self) -> usize {
        self.elems
    }
}

impl<'a, T> From<&'a mut [T]> for BufferPtrMut<'a, T> {
    fn from(s: &'a mut [T]) -> Self {
        BufferPtrMut {
            ptr: s.as_mut_ptr() as *mut _,
            marker: PhantomData,
            elems: s.len(),
        }
    }
}

impl<'a, T: SafeRepr> From<&'a mut DeviceBuffer<T>> for BufferPtrMut<'a, T> {
    fn from(b: &'a mut DeviceBuffer<T>) -> Self {
        BufferPtrMut {
            ptr: b.ptr as *mut _,
            marker: PhantomData,
            elems: b.len(),
        }
    }
}

impl<'a, T: SafeRepr> From<BufferPtrMut<'a, T>> for BufferPtr<'a, T> {
    fn from(b: BufferPtrMut<'a, T>) -> Self {
        BufferPtr {
            ptr: b.ptr,
            marker: PhantomData,
            elems: b.len(),
        }
    }
}
pub struct EventPool<'a>(sys::ze_event_pool_handle_t, PhantomData<&'a ()>);

impl<'a> EventPool<'a> {
    pub unsafe fn as_ffi(&self) -> sys::ze_event_pool_handle_t {
        self.0
    }
    pub unsafe fn from_ffi(x: sys::ze_event_pool_handle_t) -> Self {
        Self(x, PhantomData)
    }
    pub fn new(ctx: &mut Context, count: u32, dev: Option<&[&'a Device]>) -> Result<Self> {
        let desc = sys::ze_event_pool_desc_t {
            stype: sys::ze_structure_type_t::ZE_STRUCTURE_TYPE_EVENT_POOL_DESC,
            pNext: ptr::null(),
            flags: sys::ze_event_pool_flags_t(0),
            count: count,
        };
        let mut dev = dev.map(|d| d.iter().map(|d| d.0).collect::<Vec<_>>());
        let dev_len = dev.as_ref().map_or(0, |d| d.len() as u32);
        let dev_ptr = dev.as_mut().map_or(ptr::null_mut(), |d| d.as_mut_ptr());
        let mut result = ptr::null_mut();
        check!(sys::zeEventPoolCreate(
            ctx.0,
            &desc,
            dev_len,
            dev_ptr,
            &mut result
        ));
        Ok(Self(result, PhantomData))
    }
}

impl<'a> Drop for EventPool<'a> {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        check_panic! { sys::zeEventPoolDestroy(self.0) };
    }
}

pub struct Event<'a>(sys::ze_event_handle_t, PhantomData<&'a ()>);

impl<'a> Event<'a> {
    pub unsafe fn as_ffi(&self) -> sys::ze_event_handle_t {
        self.0
    }

    pub unsafe fn from_ffi(x: sys::ze_event_handle_t) -> Self {
        Self(x, PhantomData)
    }

    pub fn new(pool: &'a EventPool, index: u32) -> Result<Self> {
        let desc = sys::ze_event_desc_t {
            stype: sys::ze_structure_type_t::ZE_STRUCTURE_TYPE_EVENT_DESC,
            pNext: ptr::null(),
            index: index,
            signal: sys::ze_event_scope_flags_t(0),
            wait: sys::ze_event_scope_flags_t(0),
        };
        let mut result = ptr::null_mut();
        check!(sys::zeEventCreate(pool.0, &desc, &mut result));
        Ok(Self(result, PhantomData))
    }

    unsafe fn raw_slice(e: &mut [Event]) -> (u32, *mut sys::ze_event_handle_t) {
        let ptr = if e.len() == 0 {
            ptr::null_mut()
        } else {
            e.as_mut_ptr()
        };
        (e.len() as u32, ptr as *mut sys::ze_event_handle_t)
    }
}

impl<'a> Drop for Event<'a> {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        check_panic! { sys::zeEventDestroy(self.0) };
    }
}

pub struct Kernel<'a>(sys::ze_kernel_handle_t, PhantomData<&'a ()>);

impl<'a> Kernel<'a> {
    pub unsafe fn as_ffi(&self) -> sys::ze_kernel_handle_t {
        self.0
    }

    pub unsafe fn from_ffi(x: sys::ze_kernel_handle_t) -> Self {
        Self(x, PhantomData)
    }

    pub fn new_resident(module: &'a Module, name: &CStr) -> Result<Self> {
        let desc = sys::ze_kernel_desc_t {
            stype: sys::ze_structure_type_t::ZE_STRUCTURE_TYPE_KERNEL_DESC,
            pNext: ptr::null(),
            flags: sys::ze_kernel_flags_t::ZE_KERNEL_FLAG_FORCE_RESIDENCY,
            pKernelName: name.as_ptr() as *const _,
        };
        let mut result = ptr::null_mut();
        check!(sys::zeKernelCreate(module.0, &desc, &mut result));
        Ok(Self(result, PhantomData))
    }

    pub fn set_indirect_access(
        &mut self,
        flags: sys::ze_kernel_indirect_access_flags_t,
    ) -> Result<()> {
        check!(sys::zeKernelSetIndirectAccess(self.0, flags));
        Ok(())
    }

    pub fn set_arg_buffer<T: 'a, Buff: Into<BufferPtr<'a, T>>>(
        &self,
        index: u32,
        buff: Buff,
    ) -> Result<()> {
        let ptr = unsafe { buff.into().get() };
        check!(sys::zeKernelSetArgumentValue(
            self.0,
            index,
            mem::size_of::<*const ()>(),
            &ptr as *const _ as *const _,
        ));
        Ok(())
    }

    pub fn set_arg_scalar<T: Copy>(&self, index: u32, value: &T) -> Result<()> {
        check!(sys::zeKernelSetArgumentValue(
            self.0,
            index,
            mem::size_of::<T>(),
            value as *const T as *const _,
        ));
        Ok(())
    }

    pub unsafe fn set_arg_raw(&self, index: u32, size: usize, value: *const c_void) -> Result<()> {
        check!(sys::zeKernelSetArgumentValue(self.0, index, size, value));
        Ok(())
    }

    pub fn set_group_size(&self, x: u32, y: u32, z: u32) -> Result<()> {
        check!(sys::zeKernelSetGroupSize(self.0, x, y, z));
        Ok(())
    }

    pub fn get_properties(&self) -> Result<Box<sys::ze_kernel_properties_t>> {
        let mut props = Box::new(unsafe { mem::zeroed::<sys::ze_kernel_properties_t>() });
        check!(sys::zeKernelGetProperties(self.0, props.as_mut() as *mut _));
        Ok(props)
    }
}

impl<'a> Drop for Kernel<'a> {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        check_panic! { sys::zeKernelDestroy(self.0) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_has_correct_layout() {
        assert_eq!(
            mem::size_of::<Event>(),
            mem::size_of::<sys::ze_event_handle_t>()
        );
    }
}
