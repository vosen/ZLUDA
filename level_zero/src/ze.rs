use sys::zeFenceDestroy;

use crate::sys;
use std::{
    ffi::{c_void, CStr, CString},
    fmt::Debug,
    marker::PhantomData,
    mem,
    ptr::{self, NonNull},
};

/*
    This module is not a user-friendly, safe binding. The problem is tracking
    object lifetimes. E.g. kernel object cannot outlive module object.
    While Rust is relatively good at it, it's tricky to translate it to a safe
    API in a way that we can mix and match them, but here's I'd sketch it:
    - There's no &mut references: all API operations copy data in and out
    - All baseline objects are Send, but not Sync
    - There are some problems with using "naked" Rc and Arc:
      - We should not allow users to create Rc by themselves without including
        parent pointer
      - We should not allow DerefMut in Mutex and moving out of it
    - Objects are wrapped in Rc<ZeCell<_>> and Arc<ZeMutex<_>>, parent
      pointer is part of ZeCell/ZeMutex:
        - Then e.g. zeKernelCreate is mapped three times:
            - unsafe Module(&self) -> Kernel
            - Module(&Rc<ZeCell<Module>>) -> Rc<ZeCell<Kernel>>
            - Module(&Arc<ZeMutex<Module>>) -> Arc<KernelMutex>
        - You create ZeCell<Module> by moving Module and Rc<ZeCell<Context>
        - Pro: Rc and Arc are allowed to be self receivers
    - Open question: should some operations take the parent mutex? If so, should
      it be done recursively?
*/

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

// Mutability: no (list of allocations is under a mutex)
// Lifetime: 'static
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Driver(NonNull<sys::_ze_driver_handle_t>);

unsafe impl Send for Driver {}
unsafe impl Sync for Driver {}

impl Driver {
    pub unsafe fn as_ffi(self) -> sys::ze_driver_handle_t {
        self.0.as_ptr()
    }
    pub unsafe fn from_ffi(x: sys::ze_driver_handle_t) -> Self {
        if x == ptr::null_mut() {
            panic!("FFI handle can't be zero")
        }
        Self(NonNull::new_unchecked(x))
    }

    pub fn get() -> Result<Vec<Self>> {
        let mut len = 0;
        let mut temp = ptr::null_mut();
        check!(sys::zeDriverGet(&mut len, &mut temp));
        let mut result = Vec::with_capacity(len as usize);
        check!(sys::zeDriverGet(&mut len, result.as_mut_ptr() as *mut _));
        unsafe {
            result.set_len(len as usize);
        }
        Ok(result)
    }

    pub fn devices(self) -> Result<Vec<Device>> {
        let mut len = 0;
        let mut temp = ptr::null_mut();
        check!(sys::zeDeviceGet(self.as_ffi(), &mut len, &mut temp));
        let mut result = Vec::with_capacity(len as usize);
        check!(sys::zeDeviceGet(
            self.as_ffi(),
            &mut len,
            result.as_mut_ptr() as *mut _
        ));
        unsafe {
            result.set_len(len as usize);
        }
        Ok(result)
    }

    pub fn get_properties(self, props: &mut sys::ze_driver_properties_t) -> Result<()> {
        check!(sys::zeDriverGetProperties(self.as_ffi(), props));
        Ok(())
    }
}

// Mutability: no (list of peer allocations under a mutex)
// Lifetime: 'static
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Device(NonNull<sys::_ze_device_handle_t>);

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

impl Device {
    pub unsafe fn as_ffi(self) -> sys::ze_device_handle_t {
        self.0.as_ptr()
    }
    pub unsafe fn from_ffi(x: sys::ze_device_handle_t) -> Self {
        if x == ptr::null_mut() {
            panic!("FFI handle can't be zero")
        }
        Self(NonNull::new_unchecked(x))
    }

    pub fn get_properties(self, props: &mut sys::ze_device_properties_t) -> Result<()> {
        check! { sys::zeDeviceGetProperties(self.as_ffi(), props) };
        Ok(())
    }

    pub fn get_image_properties(self, props: &mut sys::ze_device_image_properties_t) -> Result<()> {
        check! { sys::zeDeviceGetImageProperties(self.as_ffi(),  props) };
        Ok(())
    }

    pub fn get_memory_properties(self) -> Result<Vec<sys::ze_device_memory_properties_t>> {
        let mut count = 0u32;
        check! { sys::zeDeviceGetMemoryProperties(self.as_ffi(), &mut count, ptr::null_mut()) };
        if count == 0 {
            return Ok(Vec::new());
        }
        let mut props =
            vec![unsafe { mem::zeroed::<sys::ze_device_memory_properties_t>() }; count as usize];
        check! { sys::zeDeviceGetMemoryProperties(self.as_ffi(), &mut count, props.as_mut_ptr()) };
        Ok(props)
    }

    pub fn get_compute_properties(
        self,
        props: &mut sys::ze_device_compute_properties_t,
    ) -> Result<()> {
        check! { sys::zeDeviceGetComputeProperties(self.as_ffi(), props) };
        Ok(())
    }
}

// Mutability: no
#[repr(transparent)]
pub struct Context(NonNull<sys::_ze_context_handle_t>);

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Context {
    pub unsafe fn as_ffi(&self) -> sys::ze_context_handle_t {
        self.0.as_ptr()
    }
    pub unsafe fn from_ffi(x: sys::ze_context_handle_t) -> Self {
        if x == ptr::null_mut() {
            panic!("FFI handle can't be zero")
        }
        Self(NonNull::new_unchecked(x))
    }

    pub fn new(drv: Driver, devices: Option<&[Device]>) -> Result<Self> {
        let ctx_desc = sys::ze_context_desc_t {
            stype: sys::ze_structure_type_t::ZE_STRUCTURE_TYPE_CONTEXT_DESC,
            pNext: ptr::null(),
            flags: sys::ze_context_flags_t(0),
        };
        let mut result = ptr::null_mut();
        let (dev_ptr, dev_len) = match devices {
            None => (ptr::null(), 0),
            Some(devs) => (devs.as_ptr(), devs.len()),
        };
        check!(sys::zeContextCreateEx(
            drv.as_ffi(),
            &ctx_desc,
            dev_len as u32,
            dev_ptr as *mut _,
            &mut result
        ));
        Ok(unsafe { Self::from_ffi(result) })
    }

    pub fn mem_alloc_device(
        &self,
        size: usize,
        alignment: usize,
        device: Device,
    ) -> Result<*mut c_void> {
        let descr = sys::ze_device_mem_alloc_desc_t {
            stype: sys::ze_structure_type_t::ZE_STRUCTURE_TYPE_DEVICE_MEM_ALLOC_DESC,
            pNext: ptr::null(),
            flags: sys::ze_device_mem_alloc_flags_t(0),
            ordinal: 0,
        };
        let mut result = ptr::null_mut();
        check! {
            sys::zeMemAllocDevice(
                self.as_ffi(),
                &descr,
                size,
                alignment,
                device.as_ffi(),
                &mut result,
            )
        };
        Ok(result)
    }

    // This operation is safe because Level Zero impl tracks allocations
    pub fn mem_free(&self, ptr: *mut c_void) -> Result<()> {
        check! {
            sys::zeMemFree(
                self.as_ffi(),
                ptr,
            )
        };
        Ok(())
    }
}

impl Drop for Context {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        check_panic! { sys::zeContextDestroy(self.as_ffi()) };
    }
}

// Mutability: yes (residency container and others)
// Lifetime parent: Context
#[repr(transparent)]
pub struct CommandQueue<'a>(
    NonNull<sys::_ze_command_queue_handle_t>,
    PhantomData<&'a ()>,
);

unsafe impl<'a> Send for CommandQueue<'a> {}

impl<'a> CommandQueue<'a> {
    pub unsafe fn as_ffi(&self) -> sys::ze_command_queue_handle_t {
        self.0.as_ptr()
    }
    pub unsafe fn from_ffi(x: sys::ze_command_queue_handle_t) -> Self {
        if x == ptr::null_mut() {
            panic!("FFI handle can't be zero")
        }
        Self(NonNull::new_unchecked(x), PhantomData)
    }

    pub fn new(ctx: &'a Context, d: Device) -> Result<Self> {
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
            ctx.as_ffi(),
            d.as_ffi(),
            &que_desc,
            &mut result
        ));
        Ok(unsafe { Self::from_ffi(result) })
    }

    pub fn execute_and_synchronize<'cmd_list>(
        &'a self,
        cmd: CommandList<'cmd_list>,
    ) -> Result<FenceGuard<'cmd_list>>
    where
        'a: 'cmd_list,
    {
        let fence_guard = FenceGuard::new(self, cmd)?;
        unsafe { self.execute(&fence_guard.1, Some(&fence_guard.0))? };
        Ok(fence_guard)
    }

    pub unsafe fn execute<'cmd_list, 'fence>(
        &self,
        cmd: &CommandList<'cmd_list>,
        fence: Option<&Fence<'fence>>,
    ) -> Result<()>
    where
        'cmd_list: 'fence,
        'a: 'cmd_list,
    {
        let fence_ptr = fence.map_or(ptr::null_mut(), |f| f.as_ffi());
        check!(sys::zeCommandQueueExecuteCommandLists(
            self.as_ffi(),
            1,
            &mut cmd.as_ffi(),
            fence_ptr
        ));
        Ok(())
    }

    pub fn synchronize(&self, timeout_ns: u64) -> Result<()> {
        check!(sys::zeCommandQueueSynchronize(self.as_ffi(), timeout_ns));
        Ok(())
    }
}

impl<'a> Drop for CommandQueue<'a> {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        check_panic! { sys::zeCommandQueueDestroy(self.as_ffi()) };
    }
}

pub struct FenceGuard<'a>(Fence<'a>, CommandList<'a>);

impl<'a> FenceGuard<'a> {
    fn new(q: &'a CommandQueue, cmd_list: CommandList<'a>) -> Result<Self> {
        Ok(FenceGuard(Fence::new(q)?, cmd_list))
    }
}

impl<'a> Drop for FenceGuard<'a> {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        if let Err(e) = self.0.host_synchronize() {
            panic!(e)
        }
    }
}

// Mutability: yes (reset)
// Lifetime parent: queue
#[repr(transparent)]
pub struct Fence<'a>(NonNull<sys::_ze_fence_handle_t>, PhantomData<&'a ()>);

unsafe impl<'a> Send for Fence<'a> {}

impl<'a> Fence<'a> {
    pub unsafe fn as_ffi(&self) -> sys::ze_fence_handle_t {
        self.0.as_ptr()
    }
    pub unsafe fn from_ffi(x: sys::ze_fence_handle_t) -> Self {
        if x == ptr::null_mut() {
            panic!("FFI handle can't be zero")
        }
        Self(NonNull::new_unchecked(x), PhantomData)
    }

    pub fn new(queue: &'a CommandQueue) -> Result<Self> {
        let desc = sys::_ze_fence_desc_t {
            stype: sys::ze_structure_type_t::ZE_STRUCTURE_TYPE_FENCE_DESC,
            pNext: ptr::null(),
            flags: sys::ze_fence_flags_t(0),
        };
        let mut result = ptr::null_mut();
        check!(sys::zeFenceCreate(queue.as_ffi(), &desc, &mut result));
        Ok(unsafe { Self::from_ffi(result) })
    }

    pub fn host_synchronize(&self) -> Result<()> {
        check!(sys::zeFenceHostSynchronize(self.as_ffi(), u64::max_value()));
        Ok(())
    }
}

impl<'a> Drop for Fence<'a> {
    fn drop(&mut self) {
        check_panic! { zeFenceDestroy(self.as_ffi()) };
    }
}

// Mutability: yes (building, linking)
// Lifetime parent: Context
#[repr(transparent)]
pub struct Module<'a>(NonNull<sys::_ze_module_handle_t>, PhantomData<&'a ()>);

unsafe impl<'a> Send for Module<'a> {}

impl<'a> Module<'a> {
    pub unsafe fn as_ffi(&self) -> sys::ze_module_handle_t {
        self.0.as_ptr()
    }
    pub unsafe fn from_ffi(x: sys::ze_module_handle_t) -> Self {
        if x == ptr::null_mut() {
            panic!("FFI handle can't be zero")
        }
        Self(NonNull::new_unchecked(x), PhantomData)
    }

    // HACK ALERT
    // We use OpenCL for now to do SPIR-V linking, because Level0
    // does not allow linking. Don't let presence of zeModuleDynamicLink fool
    // you, it's not currently possible to create non-compiled modules.
    // zeModuleCreate always compiles (builds and links).
    pub fn build_link_spirv<'buffers>(
        ctx: &'a Context,
        d: Device,
        binaries: &[&'buffers [u8]],
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
                let (module, build_log) = Self::build_native_logged(ctx, d, &binaries[0]);
                (module, Some(build_log))
            }
            _ => return (Err(sys::ze_result_t::ZE_RESULT_ERROR_UNKNOWN), None),
        }
    }

    fn build_link_spirv_impl<'buffers>(
        binaries: &[&'buffers [u8]],
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
        ctx: &'a Context,
        d: Device,
        bin: &[u8],
        opts: Option<&CStr>,
    ) -> Result<Self> {
        Module::new(ctx, true, d, bin, opts)
    }

    pub fn build_spirv_logged(
        ctx: &'a Context,
        d: Device,
        bin: &[u8],
        opts: Option<&CStr>,
    ) -> (Result<Self>, BuildLog) {
        Module::new_logged(ctx, true, d, bin, opts)
    }

    pub fn build_native_logged(
        ctx: &'a Context,
        d: Device,
        bin: &[u8],
    ) -> (Result<Self>, BuildLog) {
        Module::new_logged(ctx, false, d, bin, None)
    }

    fn new(
        ctx: &'a Context,
        spirv: bool,
        d: Device,
        bin: &[u8],
        opts: Option<&CStr>,
    ) -> Result<Self> {
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
        check! {
            sys::zeModuleCreate(
                ctx.as_ffi(),
                d.as_ffi(),
                &desc,
                &mut result,
                ptr::null_mut(),
            )
        };
        Ok(unsafe { Self::from_ffi(result) })
    }

    fn new_logged(
        ctx: &'a Context,
        spirv: bool,
        d: Device,
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
        let err = unsafe {
            sys::zeModuleCreate(
                ctx.as_ffi(),
                d.as_ffi(),
                &desc,
                &mut result,
                &mut log_handle,
            )
        };
        let log = unsafe { BuildLog::from_ffi(log_handle) };
        if err != sys::ze_result_t::ZE_RESULT_SUCCESS {
            (Result::Err(err), log)
        } else {
            (Ok(unsafe { Self::from_ffi(result) }), log)
        }
    }

    pub fn get_global_pointer(&self, global_name: &CStr) -> Result<(usize, *mut c_void)> {
        let slice = global_name.to_bytes_with_nul();
        let mut result_size = 0;
        let mut result_ptr = ptr::null_mut();
        check!(sys::zeModuleGetGlobalPointer(
            self.as_ffi(),
            slice.as_ptr() as *const _,
            &mut result_size,
            &mut result_ptr,
        ));
        Ok((result_size, result_ptr))
    }

    pub fn dynamic_link(modules: &[&Module]) -> Result<()> {
        unsafe {
            Self::with_raw_slice(modules, |num, ptr| {
                check!(sys::zeModuleDynamicLink(num, ptr, ptr::null_mut()));
                Ok(())
            })
        }
    }

    unsafe fn with_raw_slice<'x, T>(
        modules: &[&Module<'x>],
        f: impl FnOnce(u32, *mut sys::ze_module_handle_t) -> T,
    ) -> T {
        let (ptr, mod_vec) = match modules {
            [] => (ptr::null_mut(), None),
            [e] => (&e.0 as *const _ as *mut _, None),
            _ => {
                let mut ev_vec = modules.iter().map(|e| e.as_ffi()).collect::<Vec<_>>();
                (ev_vec.as_mut_ptr(), Some(ev_vec))
            }
        };
        let result = f(modules.len() as u32, ptr);
        drop(mod_vec);
        result
    }
}

impl<'a> Drop for Module<'a> {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        check_panic! { sys::zeModuleDestroy(self.as_ffi()) };
    }
}

// Mutability: none
// Lifetime parent: none, but need to destroy
pub struct BuildLog(NonNull<sys::_ze_module_build_log_handle_t>);

unsafe impl Sync for BuildLog {}
unsafe impl Send for BuildLog {}

impl BuildLog {
    pub unsafe fn as_ffi(&self) -> sys::ze_module_build_log_handle_t {
        self.0.as_ptr()
    }
    pub unsafe fn from_ffi(x: sys::ze_module_build_log_handle_t) -> Self {
        if x == ptr::null_mut() {
            panic!("FFI handle can't be zero")
        }
        Self(NonNull::new_unchecked(x))
    }

    pub fn to_cstring(&self) -> Result<CString> {
        let mut size = 0;
        check! { sys::zeModuleBuildLogGetString(self.as_ffi(), &mut size, ptr::null_mut()) };
        let mut str_vec = vec![0u8; size];
        check! { sys::zeModuleBuildLogGetString(self.as_ffi(), &mut size, str_vec.as_mut_ptr() as *mut i8) };
        str_vec.push(0);
        Ok(unsafe { CString::from_vec_unchecked(str_vec) })
    }
}

impl Drop for BuildLog {
    fn drop(&mut self) {
        check_panic!(sys::zeModuleBuildLogDestroy(self.as_ffi()));
    }
}

// Mutability: none
// Lifetime parent: Context
pub struct DeviceBuffer<'a, T: Copy> {
    ptr: *mut c_void,
    ctx: sys::ze_context_handle_t,
    len: usize,
    marker: PhantomData<&'a T>,
}

unsafe impl<'a, T: Copy> Sync for DeviceBuffer<'a, T> {}
unsafe impl<'a, T: Copy> Send for DeviceBuffer<'a, T> {}

impl<'a, T: Copy> DeviceBuffer<'a, T> {
    pub unsafe fn as_ffi(&self) -> (sys::ze_context_handle_t, *mut c_void, usize) {
        (self.ctx, self.ptr, self.len)
    }
    pub unsafe fn from_ffi(ctx: sys::ze_context_handle_t, ptr: *mut c_void, len: usize) -> Self {
        let marker = PhantomData::<&'a T>;
        Self {
            ptr,
            ctx,
            len,
            marker,
        }
    }

    pub fn new(ctx: &'a Context, dev: Device, len: usize) -> Result<Self> {
        let desc = sys::_ze_device_mem_alloc_desc_t {
            stype: sys::ze_structure_type_t::ZE_STRUCTURE_TYPE_DEVICE_MEM_ALLOC_DESC,
            pNext: ptr::null(),
            flags: sys::ze_device_mem_alloc_flags_t(0),
            ordinal: 0,
        };
        let mut result = ptr::null_mut();
        check!(sys::zeMemAllocDevice(
            ctx.as_ffi(),
            &desc,
            len * mem::size_of::<T>(),
            mem::align_of::<T>(),
            dev.as_ffi(),
            &mut result
        ));
        Ok(unsafe { Self::from_ffi(ctx.as_ffi(), result, len) })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn data(&self) -> *mut c_void {
        self.ptr
    }
}

impl<'a, T: Copy> Drop for DeviceBuffer<'a, T> {
    fn drop(&mut self) {
        check_panic! { sys::zeMemFree(self.ctx, self.ptr) };
    }
}

// Mutability: yes (appends)
// Lifetime parent: Context
pub struct CommandList<'a>(NonNull<sys::_ze_command_list_handle_t>, PhantomData<&'a ()>);

unsafe impl<'a> Send for CommandList<'a> {}

impl<'a> CommandList<'a> {
    pub unsafe fn as_ffi(&self) -> sys::ze_command_list_handle_t {
        self.0.as_ptr()
    }
    pub unsafe fn from_ffi(x: sys::ze_command_list_handle_t) -> Self {
        if x == ptr::null_mut() {
            panic!("FFI handle can't be zero")
        }
        Self(NonNull::new_unchecked(x), PhantomData)
    }

    pub fn new(ctx: &'a Context, dev: Device) -> Result<Self> {
        let desc = sys::ze_command_list_desc_t {
            stype: sys::_ze_structure_type_t::ZE_STRUCTURE_TYPE_COMMAND_LIST_DESC,
            commandQueueGroupOrdinal: 0,
            pNext: ptr::null(),
            flags: sys::ze_command_list_flags_t(0),
        };
        let mut result: sys::ze_command_list_handle_t = ptr::null_mut();
        check!(sys::zeCommandListCreate(
            ctx.as_ffi(),
            dev.as_ffi(),
            &desc,
            &mut result
        ));
        Ok(unsafe { Self::from_ffi(result) })
    }

    pub fn new_immediate(ctx: &'a Context, dev: Device) -> Result<Self> {
        let queue_desc = sys::ze_command_queue_desc_t {
            stype: sys::ze_structure_type_t::ZE_STRUCTURE_TYPE_COMMAND_QUEUE_DESC,
            pNext: ptr::null(),
            ordinal: 0,
            index: 0,
            flags: sys::ze_command_queue_flags_t(0),
            mode: sys::ze_command_queue_mode_t::ZE_COMMAND_QUEUE_MODE_DEFAULT,
            priority: sys::ze_command_queue_priority_t::ZE_COMMAND_QUEUE_PRIORITY_NORMAL,
        };
        let mut result: sys::ze_command_list_handle_t = ptr::null_mut();
        check!(sys::zeCommandListCreateImmediate(
            ctx.as_ffi(),
            dev.as_ffi(),
            &queue_desc,
            &mut result
        ));
        Ok(unsafe { Self::from_ffi(result) })
    }

    pub unsafe fn append_memory_copy<
        'dep,
        T: 'a + 'dep + Copy + Sized,
        Dst: Into<Slice<'dep, T>>,
        Src: Into<Slice<'dep, T>>,
    >(
        &self,
        dst: Dst,
        src: Src,
        signal: Option<&Event<'dep>>,
        wait: &[&'dep Event<'dep>],
    ) -> Result<()> {
        let dst = dst.into();
        let src = src.into();
        let elements = std::cmp::min(dst.len(), src.len());
        let length = elements * mem::size_of::<T>();
        self.append_memory_copy_raw(dst.as_mut_ptr(), src.as_ptr(), length, signal, wait)
    }

    pub unsafe fn append_memory_copy_raw(
        &self,
        dst: *mut c_void,
        src: *const c_void,
        length: usize,
        signal: Option<&Event>,
        wait: &[&Event],
    ) -> Result<()> {
        let signal_event = signal.map_or(ptr::null_mut(), |e| e.as_ffi());
        Event::with_raw_slice(wait, |wait_len, wait_ptr| {
            check!(sys::zeCommandListAppendMemoryCopy(
                self.as_ffi(),
                dst,
                src,
                length,
                signal_event,
                wait_len,
                wait_ptr
            ));
            Ok(())
        })
    }

    pub unsafe fn append_memory_fill<'dep, T: Copy + Sized + 'dep, Dst: Into<Slice<'dep, T>>>(
        &'a self,
        dst: Dst,
        pattern: &T,
        signal: Option<&Event<'dep>>,
        wait: &[&'dep Event<'dep>],
    ) -> Result<()> {
        let dst = dst.into();
        let raw_pattern = pattern as *const _ as *const _;
        let signal_event = signal.map_or(ptr::null_mut(), |e| e.as_ffi());
        Event::with_raw_slice(wait, |wait_len, wait_ptr| {
            check!(sys::zeCommandListAppendMemoryFill(
                self.as_ffi(),
                dst.as_mut_ptr(),
                raw_pattern,
                mem::size_of::<T>(),
                dst.len() * mem::size_of::<T>(),
                signal_event,
                wait_len,
                wait_ptr
            ));
            Ok(())
        })
    }

    pub unsafe fn append_memory_fill_raw(
        &self,
        dst: *mut c_void,
        pattern: *mut c_void,
        pattern_size: usize,
        size: usize,
        signal: Option<&Event>,
        wait: &[&Event],
    ) -> Result<()> {
        let signal_event = signal.map_or(ptr::null_mut(), |e| e.as_ffi());
        Event::with_raw_slice(wait, |wait_len, wait_ptr| {
            check!(sys::zeCommandListAppendMemoryFill(
                self.as_ffi(),
                dst,
                pattern,
                pattern_size,
                size,
                signal_event,
                wait_len,
                wait_ptr
            ));
            Ok(())
        })
    }

    pub unsafe fn append_launch_kernel(
        &self,
        kernel: &Kernel,
        group_count: &[u32; 3],
        signal: Option<&Event>,
        wait: &[&Event],
    ) -> Result<()> {
        let gr_count = sys::ze_group_count_t {
            groupCountX: group_count[0],
            groupCountY: group_count[1],
            groupCountZ: group_count[2],
        };
        let signal_event = signal.map_or(ptr::null_mut(), |e| e.as_ffi());
        Event::with_raw_slice(wait, |wait_len, wait_ptr| {
            check!(sys::zeCommandListAppendLaunchKernel(
                self.as_ffi(),
                kernel.as_ffi(),
                &gr_count,
                signal_event,
                wait_len,
                wait_ptr,
            ));
            Ok(())
        })
    }

    pub unsafe fn append_barrier(&self, signal: Option<&Event>, wait: &[&Event]) -> Result<()> {
        let signal_event = signal.map_or(ptr::null_mut(), |e| e.as_ffi());
        Event::with_raw_slice(wait, |wait_len, wait_ptr| {
            check!(sys::zeCommandListAppendBarrier(
                self.as_ffi(),
                signal_event,
                wait_len,
                wait_ptr
            ));
            Ok(())
        })
    }

    pub fn close(&self) -> Result<()> {
        check!(sys::zeCommandListClose(self.as_ffi()));
        Ok(())
    }
}

impl<'a> Drop for CommandList<'a> {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        check_panic! { sys::zeCommandListDestroy(self.as_ffi()) };
    }
}

pub struct CommandListBuilder<'a>(CommandList<'a>);

unsafe impl<'a> Send for CommandListBuilder<'a> {}

impl<'a> CommandListBuilder<'a> {
    pub fn new(ctx: &'a Context, dev: Device) -> Result<Self> {
        Ok(CommandListBuilder(CommandList::new(ctx, dev)?))
    }

    pub fn append_memory_copy<
        'dep,
        'result,
        T: 'dep + Copy + Sized,
        Dst: Into<Slice<'dep, T>>,
        Src: Into<Slice<'dep, T>>,
    >(
        self,
        dst: Dst,
        src: Src,
        signal: Option<&'dep Event<'dep>>,
        wait: &[&'dep Event<'dep>],
    ) -> Result<CommandListBuilder<'result>>
    where
        'a: 'result,
        'dep: 'result,
    {
        unsafe { self.0.append_memory_copy(dst, src, signal, wait) }?;
        Ok(self)
    }

    pub fn append_memory_fill<'dep, 'result, T: 'dep + Copy + Sized, Dst: Into<Slice<'dep, T>>>(
        self,
        dst: Dst,
        pattern: &T,
        signal: Option<&Event<'dep>>,
        wait: &[&'dep Event<'dep>],
    ) -> Result<CommandListBuilder<'result>>
    where
        'a: 'result,
        'dep: 'result,
    {
        unsafe { self.0.append_memory_fill(dst, pattern, signal, wait) }?;
        Ok(self)
    }

    pub fn append_launch_kernel<'dep, 'result>(
        self,
        kernel: &'dep Kernel,
        group_count: &[u32; 3],
        signal: Option<&Event<'dep>>,
        wait: &[&'dep Event<'dep>],
    ) -> Result<CommandListBuilder<'result>>
    where
        'a: 'result,
        'dep: 'result,
    {
        unsafe {
            self.0
                .append_launch_kernel(kernel, group_count, signal, wait)
        }?;
        Ok(self)
    }

    pub fn execute(self, q: &'a CommandQueue<'a>) -> Result<FenceGuard<'a>> {
        self.0.close()?;
        q.execute_and_synchronize(self.0)
    }
}

#[derive(Copy, Clone)]
pub struct Slice<'a, T: Copy + Sized> {
    ptr: *mut c_void,
    len: usize,
    marker: PhantomData<&'a T>,
}

unsafe impl<'a, T: Copy + Sized> Send for Slice<'a, T> {}
unsafe impl<'a, T: Copy + Sized> Sync for Slice<'a, T> {}

impl<'a, T: Copy + Sized> Slice<'a, T> {
    pub unsafe fn new(ptr: *mut c_void, len: usize) -> Self {
        Self {
            ptr,
            len,
            marker: PhantomData,
        }
    }

    pub fn as_ptr(&self) -> *const c_void {
        self.ptr
    }

    pub fn as_mut_ptr(&self) -> *mut c_void {
        self.ptr
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<'a, T: Copy + Sized> From<&'a [T]> for Slice<'a, T> {
    fn from(s: &'a [T]) -> Self {
        Slice {
            ptr: s.as_ptr() as *mut _,
            len: s.len(),
            marker: PhantomData,
        }
    }
}

impl<'a, T: Copy + Sized> From<&'a DeviceBuffer<'a, T>> for Slice<'a, T> {
    fn from(b: &'a DeviceBuffer<'a, T>) -> Self {
        Slice {
            ptr: b.ptr,
            len: b.len,
            marker: PhantomData,
        }
    }
}

// Mutability: yes (appends)
// Lifetime parent: Context
pub struct EventPool<'a>(NonNull<sys::_ze_event_pool_handle_t>, PhantomData<&'a ()>);

impl<'a> EventPool<'a> {
    pub unsafe fn as_ffi(&self) -> sys::ze_event_pool_handle_t {
        self.0.as_ptr()
    }
    pub unsafe fn from_ffi(x: sys::ze_event_pool_handle_t) -> Self {
        if x == ptr::null_mut() {
            panic!("FFI handle can't be zero")
        }
        Self(NonNull::new_unchecked(x), PhantomData)
    }

    pub fn new(
        ctx: &'a Context,
        flags: sys::ze_event_pool_flags_t,
        count: u32,
        devs: Option<&[Device]>,
    ) -> Result<Self> {
        let desc = sys::ze_event_pool_desc_t {
            stype: sys::ze_structure_type_t::ZE_STRUCTURE_TYPE_EVENT_POOL_DESC,
            pNext: ptr::null(),
            flags: flags,
            count: count,
        };
        let (dev_len, dev_ptr) = devs.map_or((0, ptr::null_mut()), |devs| {
            (devs.len(), devs.as_ptr() as *mut _)
        });
        let mut result = ptr::null_mut();
        check!(sys::zeEventPoolCreate(
            ctx.as_ffi(),
            &desc,
            dev_len as u32,
            dev_ptr,
            &mut result
        ));
        Ok(unsafe { Self::from_ffi(result) })
    }
}

impl<'a> Drop for EventPool<'a> {
    fn drop(&mut self) {
        check_panic! { sys::zeEventPoolDestroy(self.as_ffi()) };
    }
}

pub struct Event<'a>(NonNull<sys::_ze_event_handle_t>, PhantomData<&'a ()>);

impl<'a> Event<'a> {
    pub unsafe fn as_ffi(&self) -> sys::ze_event_handle_t {
        self.0.as_ptr()
    }
    pub unsafe fn from_ffi(x: sys::ze_event_handle_t) -> Self {
        if x == ptr::null_mut() {
            panic!("FFI handle can't be zero")
        }
        Self(NonNull::new_unchecked(x), PhantomData)
    }

    pub fn new(
        pool: &'a EventPool<'a>,
        index: u32,
        signal: sys::ze_event_scope_flags_t,
        wait: sys::ze_event_scope_flags_t,
    ) -> Result<Self> {
        let desc = sys::ze_event_desc_t {
            stype: sys::ze_structure_type_t::ZE_STRUCTURE_TYPE_EVENT_DESC,
            pNext: ptr::null(),
            index: index,
            signal,
            wait,
        };
        let mut result = ptr::null_mut();
        check!(sys::zeEventCreate(pool.as_ffi(), &desc, &mut result));
        Ok(unsafe { Self::from_ffi(result) })
    }

    pub fn host_synchronize(&self, timeout_ns: u64) -> Result<()> {
        check! { sys::zeEventHostSynchronize(self.as_ffi(), timeout_ns) };
        Ok(())
    }

    pub fn is_ready(&self) -> Result<bool> {
        let status = unsafe { sys::zeEventQueryStatus(self.as_ffi()) };
        match status {
            sys::ze_result_t::ZE_RESULT_SUCCESS => Ok(true),
            sys::ze_result_t::ZE_RESULT_NOT_READY => Ok(false),
            err => Err(err),
        }
    }

    pub fn host_reset(&self) -> Result<()> {
        check! { sys::zeEventHostReset(self.as_ffi()) };
        Ok(())
    }

    unsafe fn with_raw_slice<'x, T>(
        events: &[&Event<'x>],
        f: impl FnOnce(u32, *mut sys::ze_event_handle_t) -> T,
    ) -> T {
        let (ptr, ev_vec) = match events {
            [] => (ptr::null_mut(), None),
            [e] => (&e.0 as *const _ as *mut _, None),
            _ => {
                let mut ev_vec = events.iter().map(|e| e.as_ffi()).collect::<Vec<_>>();
                (ev_vec.as_mut_ptr(), Some(ev_vec))
            }
        };
        let result = f(events.len() as u32, ptr);
        drop(ev_vec);
        result
    }
}

impl<'a> Drop for Event<'a> {
    fn drop(&mut self) {
        check_panic! { sys::zeEventDestroy(self.as_ffi()) };
    }
}

pub struct Kernel<'a>(NonNull<sys::_ze_kernel_handle_t>, PhantomData<&'a ()>);

impl<'a> Kernel<'a> {
    pub unsafe fn as_ffi(&self) -> sys::ze_kernel_handle_t {
        self.0.as_ptr()
    }
    pub unsafe fn from_ffi(x: sys::ze_kernel_handle_t) -> Self {
        if x == ptr::null_mut() {
            panic!("FFI handle can't be zero")
        }
        Self(NonNull::new_unchecked(x), PhantomData)
    }

    pub fn new_resident(module: &'a Module, name: &CStr) -> Result<Self> {
        let desc = sys::ze_kernel_desc_t {
            stype: sys::ze_structure_type_t::ZE_STRUCTURE_TYPE_KERNEL_DESC,
            pNext: ptr::null(),
            flags: sys::ze_kernel_flags_t::ZE_KERNEL_FLAG_FORCE_RESIDENCY,
            pKernelName: name.as_ptr() as *const _,
        };
        let mut result = ptr::null_mut();
        check!(sys::zeKernelCreate(module.as_ffi(), &desc, &mut result));
        Ok(unsafe { Self::from_ffi(result) })
    }

    pub fn set_indirect_access(&self, flags: sys::ze_kernel_indirect_access_flags_t) -> Result<()> {
        check!(sys::zeKernelSetIndirectAccess(self.as_ffi(), flags));
        Ok(())
    }

    pub fn set_arg_buffer<T: 'a + Copy + Sized, Buff: Into<Slice<'a, T>>>(
        &self,
        index: u32,
        buff: Buff,
    ) -> Result<()> {
        let ptr = buff.into().as_mut_ptr();
        check!(sys::zeKernelSetArgumentValue(
            self.as_ffi(),
            index,
            mem::size_of::<*const ()>(),
            &ptr as *const _ as *const _,
        ));
        Ok(())
    }

    pub fn set_arg_scalar<T: Copy>(&self, index: u32, value: &T) -> Result<()> {
        check!(sys::zeKernelSetArgumentValue(
            self.as_ffi(),
            index,
            mem::size_of::<T>(),
            value as *const T as *const _,
        ));
        Ok(())
    }

    pub unsafe fn set_arg_raw(&self, index: u32, size: usize, value: *const c_void) -> Result<()> {
        check!(sys::zeKernelSetArgumentValue(
            self.as_ffi(),
            index,
            size,
            value
        ));
        Ok(())
    }

    pub fn set_group_size(&self, x: u32, y: u32, z: u32) -> Result<()> {
        check!(sys::zeKernelSetGroupSize(self.as_ffi(), x, y, z));
        Ok(())
    }

    pub fn get_properties(&self) -> Result<Box<sys::ze_kernel_properties_t>> {
        let mut props = Box::new(unsafe { mem::zeroed::<sys::ze_kernel_properties_t>() });
        check!(sys::zeKernelGetProperties(
            self.as_ffi(),
            props.as_mut() as *mut _
        ));
        Ok(props)
    }
}

impl<'a> Drop for Kernel<'a> {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        check_panic! { sys::zeKernelDestroy(self.as_ffi()) };
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
