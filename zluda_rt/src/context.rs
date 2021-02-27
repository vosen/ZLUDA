use crate::{
    acceleration::AccelerationData,
    buffer::{Buffer, BufferData, DeviceBuffer},
    cache::KernelCache,
    definitions, div_positive_round_up,
    geometry::GeometryData,
    geometry_group::GeometryGroupData,
    geometry_instance::GeometryInstanceData,
    geometry_triangles::GeometryTrianglesData,
    group::GroupData,
    hip, hiprt,
    material::MaterialData,
    null_check, null_unwrap, null_unwrap_mut,
    program::{self, Program, ProgramData},
    repr_gpu::{self, Scene, TrivialHIPAllocator},
    texture_sampler::TextureSamplerData,
    transform::TransformData,
    variable::{Variable, VariableData},
    MaybeWeakRefMut, OptixCell, OptixObjectData, RcHashSet, TypeTag,
};
use comgr::Comgr;
use hip_common::raytracing::VariablesBlock;
use hip_runtime_sys::*;
use hiprt_sys::*;
use optix_types::*;
use rustc_hash::FxHashMap;
use std::{
    ffi::{CStr, CString},
    fs,
    mem::{self, ManuallyDrop},
    path::PathBuf,
    ptr,
    rc::{Rc, Weak},
};

pub(crate) type Context = *const OptixCell<ContextData>;

pub(crate) struct ContextData {
    pub(crate) hip_context: hipCtx_t,
    pub(crate) hiprt: Rc<HipRt>,
    pub(crate) ray_type_count: u32,
    pub(crate) geometry_group_count: u32,
    pub(crate) context: hiprtContext,
    pub(crate) comgr: Rc<Comgr>,
    pub(crate) isa: CString,
    pub(crate) buffers: RcHashSet<OptixCell<BufferData>>,
    pub(crate) buffers_counter: u32,
    pub(crate) variables: FxHashMap<CString, Rc<OptixCell<VariableData>>>,
    pub(crate) programs: RcHashSet<OptixCell<ProgramData>>,
    pub(crate) callable_program_counter: u32,
    pub(crate) cumulative_attributes: VariablesBlock,
    pub(crate) materials: RcHashSet<OptixCell<MaterialData>>,
    pub(crate) geometry: RcHashSet<OptixCell<GeometryData>>,
    pub(crate) geometry_triangles: RcHashSet<OptixCell<GeometryTrianglesData>>,
    pub(crate) geometry_instances: RcHashSet<OptixCell<GeometryInstanceData>>,
    pub(crate) geometry_groups: RcHashSet<OptixCell<GeometryGroupData>>,
    pub(crate) groups: RcHashSet<OptixCell<GroupData>>,
    pub(crate) accelerations: RcHashSet<OptixCell<AccelerationData>>,
    pub(crate) texture_samplers: RcHashSet<OptixCell<TextureSamplerData>>,
    pub(crate) transforms: RcHashSet<OptixCell<TransformData>>,
    pub(crate) texture_counter: u32,
    pub(crate) entry_points: Vec<Option<Rc<OptixCell<ProgramData>>>>,
    pub(crate) exception_programs: Vec<Option<Rc<OptixCell<ProgramData>>>>,
    pub(crate) miss_programs: Vec<Option<Rc<OptixCell<ProgramData>>>>,
    disk_cache_location: PathBuf,
    pub(crate) optix_salt: [u8; 32],
    pub(crate) vendor_salt: [u8; 32],
    pub(crate) public_vendor_key: Vec<u8>,
    pub(crate) cache: Option<KernelCache>,
    pub(crate) compiler_version: String,
    pub(crate) hiprt_version: String,
    pub(crate) scene_rebuild_pending: bool,
    pub(crate) scene: Scene,
    pub(crate) global_stack: GlobalStack,
}

// 16kB is the maximum allowed in rocm 5.4 and seems to be enough for Arnold
const MAX_GPU_STACK: usize = 16 * 1_024;

impl ContextData {
    pub(crate) fn new() -> Result<Self, RTresult> {
        hip! { hipInit(0), RT_ERROR_CONTEXT_CREATION_FAILED };
        let comgr = comgr::Comgr::find_and_load()
            .map_err(|_| RTresult::RT_ERROR_CONTEXT_CREATION_FAILED)?;
        let mut stack_size = 0;
        hip! { hipDeviceGetLimit(&mut stack_size, hipLimit_t::hipLimitStackSize), RT_ERROR_CONTEXT_CREATION_FAILED };
        if stack_size < MAX_GPU_STACK {
            hip! { hipDeviceSetLimit(hipLimit_t::hipLimitStackSize, MAX_GPU_STACK), RT_ERROR_CONTEXT_CREATION_FAILED };
        }
        let mut hip_context = ptr::null_mut();
        hip! { hipCtxCreate(&mut hip_context, 0, 0), RT_ERROR_CONTEXT_CREATION_FAILED };
        let mut context_input = hiprtContextCreationInput {
            ctxt: hip_context as _,
            device: 0,
            deviceType: hiprtDeviceType::hiprtDeviceAMD,
        };
        let hiprt =
            unsafe { HipRt::load() }.map_err(|_| RTresult::RT_ERROR_CONTEXT_CREATION_FAILED)?;
        let mut context = ptr::null_mut();
        hiprt! {
            hiprt.hiprtCreateContext(HIPRT_API_VERSION, &mut context_input, &mut context),
            RT_ERROR_CONTEXT_CREATION_FAILED
        };
        let isa = unsafe {
            hip_common::comgr_isa(0).map_err(|_| RTresult::RT_ERROR_CONTEXT_CREATION_FAILED)?
        };
        let hiprt_version = hiprt::HIPRT_API_VERSION.to_string();
        let compiler_version = comgr
            .version()
            .map_err(|_| RTresult::RT_ERROR_CONTEXT_CREATION_FAILED)?;
        let mut disk_cache_location =
            dirs::cache_dir().ok_or(RTresult::RT_ERROR_CONTEXT_CREATION_FAILED)?;
        disk_cache_location.push("ZLUDA");
        disk_cache_location.push("OptixCache");
        fs::create_dir_all(&disk_cache_location)
            .map_err(|_| RTresult::RT_ERROR_CONTEXT_CREATION_FAILED)?;
        let cache = KernelCache::new(&disk_cache_location)
            .ok_or(RTresult::RT_ERROR_CONTEXT_CREATION_FAILED)?;
        Ok(ContextData {
            hip_context,
            hiprt: Rc::new(hiprt),
            context,
            ray_type_count: 0,
            geometry_group_count: 0,
            comgr: Rc::new(comgr),
            buffers: RcHashSet::new(),
            buffers_counter: 0,
            variables: FxHashMap::default(),
            programs: RcHashSet::new(),
            callable_program_counter: 0,
            cumulative_attributes: VariablesBlock::empty(),
            materials: RcHashSet::new(),
            geometry: RcHashSet::new(),
            geometry_triangles: RcHashSet::new(),
            geometry_instances: RcHashSet::new(),
            geometry_groups: RcHashSet::new(),
            transforms: RcHashSet::new(),
            groups: RcHashSet::new(),
            accelerations: RcHashSet::new(),
            texture_samplers: RcHashSet::new(),
            texture_counter: 0,
            entry_points: Vec::new(),
            exception_programs: Vec::new(),
            miss_programs: Vec::new(),
            isa,
            disk_cache_location,
            optix_salt: [0; 32],
            vendor_salt: [0; 32],
            public_vendor_key: Vec::new(),
            cache: Some(cache),
            compiler_version,
            hiprt_version,
            scene_rebuild_pending: true,
            scene: Scene::empty(),
            global_stack: GlobalStack::empty(),
        })
    }

    pub fn attributes_layout(&self) -> (u16, u16) {
        let size = div_positive_round_up(
            self.cumulative_attributes.layout.size() as u64,
            mem::size_of::<u32>() as u64,
        );
        let align = div_positive_round_up(
            self.cumulative_attributes.layout.align() as u64,
            mem::size_of::<u32>() as u64,
        );
        (size as u16, align as u16)
    }

    pub fn allocate_miss_programs(
        &self,
        allocator: &mut TrivialHIPAllocator,
    ) -> Result<hipDeviceptr_t, RTresult> {
        let call_chain_visitor = repr_gpu::MissProgramsVisitCallChain {
            context: self,
            miss_programs: &self.miss_programs,
        };
        let chain_layout = repr_gpu::get_layout(0, &call_chain_visitor)?;
        let chain_on_gpu = allocator.allocate(chain_layout.layout.size())?;
        repr_gpu::copy_to_gpu(0, &call_chain_visitor, &chain_layout, chain_on_gpu)?;
        Ok(chain_on_gpu)
    }

    pub fn allocate_callable_programs(
        &self,
        allocator: &mut TrivialHIPAllocator,
    ) -> Result<hipDeviceptr_t, RTresult> {
        let visitor = repr_gpu::CallableProgramsVisitor::new(self)?;
        let chain_layout = repr_gpu::get_layout(self.ray_type_count, &visitor)?;
        let dev_ptr = allocator.allocate(chain_layout.layout.size())?;
        repr_gpu::copy_to_gpu(self.ray_type_count, &visitor, &chain_layout, dev_ptr)?;
        Ok(dev_ptr)
    }

    pub fn allocate_buffers(
        &self,
        allocator: &mut TrivialHIPAllocator,
    ) -> Result<hipDeviceptr_t, RTresult> {
        let mut buffers = (0..self.buffers_counter + 1)
            .into_iter()
            .map(|_| unsafe { mem::zeroed::<DeviceBuffer>() })
            .collect::<Vec<_>>();
        // We allocate this additional buffer for bug compatiblity with OptiX
        // Arnold has a bug where it tries to access buffer with id = 0
        // On OptiX this returns NULL pointer. Furthermore, in OptiX, dereferencing
        // an invalid pointer returns zeros. We emulate this behavior by returning
        // this zero-filled buffer. It actually works well enough to run Arnold
        let page_zero = allocator.allocate(1024 * 4)?;
        hip! { hipMemset(page_zero.0, 0, 1024 * 4), RT_ERROR_MEMORY_ALLOCATION_FAILED };
        buffers[0] = DeviceBuffer {
            pointer: page_zero,
            width: 1024,
            height: 0,
        };
        for buffer in self.buffers.iter() {
            let buffer = buffer.borrow()?;
            buffers[buffer.index as usize] = buffer.get_device_mip0();
        }
        allocator.copy_to_device(&buffers[..])
    }

    pub fn get_uv_offset(&self) -> Result<u32, RTresult> {
        let uv_offset = self
            .cumulative_attributes
            .variables
            .get(unsafe { CStr::from_bytes_with_nul_unchecked(b"rtTriangleBarycentrics\0") })
            .map(|var| var.offset)
            .unwrap_or(!0u32);
        Ok(uv_offset)
    }

    pub fn invalidate(&mut self) {
        self.scene_rebuild_pending = true;
    }

    pub(crate) fn buffers_load_from_callback(&mut self) -> Result<(), RTresult> {
        for buffer_rc in self.buffers.iter() {
            BufferData::load_from_callback(buffer_rc)?;
        }
        Ok(())
    }
}

impl Drop for ContextData {
    #[allow(unused_must_use)]
    fn drop(&mut self) {
        unsafe {
            self.hiprt.hiprtDestroyContext(self.context);
            hipCtxDestroy(self.hip_context);
        }
    }
}

impl OptixObjectData for ContextData {
    const TYPE: TypeTag = TypeTag::Context;

    fn deregister(&mut self, _this: &Rc<OptixCell<Self>>) -> Result<(), RTresult> {
        Err(RTresult::RT_ERROR_UNKNOWN)
    }

    fn context<'a>(&'a mut self) -> crate::MaybeWeakRefMut<'a, ContextData> {
        MaybeWeakRefMut::Ref(self)
    }
}

pub(crate) struct GlobalStack {
    pointer: hipDeviceptr_t,
    thread_space: usize,
}

impl GlobalStack {
    pub(crate) const THREAD_STACK_DEPTH: u16 = 512;

    fn empty() -> GlobalStack {
        GlobalStack {
            pointer: hipDeviceptr_t(ptr::null_mut()),
            thread_space: 0,
        }
    }

    fn reallocate(&mut self, width: u32, height: u32) -> Result<(), RTresult> {
        let (grid_dim_x, block_dim_x) = program::get_launch_dimensions_x(width)?;
        let thread_space = grid_dim_x as usize * block_dim_x as usize * height as usize;
        if thread_space <= self.thread_space {
            return Ok(());
        }
        hip::free(self.pointer).map_err(|_| RTresult::RT_ERROR_MEMORY_ALLOCATION_FAILED)?;
        self.pointer =
            hip::malloc(thread_space * Self::THREAD_STACK_DEPTH as usize * mem::size_of::<u32>())
                .map_err(|_| RTresult::RT_ERROR_MEMORY_ALLOCATION_FAILED)?;
        self.thread_space = thread_space;
        Ok(())
    }
}

pub(crate) unsafe fn get_error_string(
    _context: Context,
    _code: RTresult,
    string_return: *mut *const i8,
) {
    *string_return = b"\0".as_ptr() as _;
}

pub(crate) unsafe fn set_entry_point_count(context: Context, count: u32) -> Result<(), RTresult> {
    let context = null_unwrap(context)?;
    let mut context = (*context).borrow_mut()?;
    context.entry_points.resize(count as usize, None);
    context.exception_programs.resize(count as usize, None);
    Ok(())
}

pub(crate) unsafe fn set_ray_type_count(
    context: Context,
    ray_type_count: u32,
) -> Result<(), RTresult> {
    let context = null_unwrap(context)?;
    let mut context = context.borrow_mut()?;
    context.ray_type_count = ray_type_count;
    context.miss_programs.resize(ray_type_count as usize, None);
    Ok(())
}

pub(crate) unsafe fn create(context: *mut Context) -> Result<(), RTresult> {
    null_check(context)?;
    *context = Rc::into_raw(Rc::new(OptixCell::new(ContextData::new()?))) as *mut _;
    Ok(())
}

pub(crate) unsafe fn destroy(context: Context) -> Result<(), RTresult> {
    null_check(context)?;
    Rc::from_raw(context);
    Ok(())
}

pub(crate) unsafe fn declare_variable(
    context_ptr: Context,
    name: *const i8,
    v: *mut Variable,
) -> Result<(), RTresult> {
    null_check(name)?;
    let v = null_unwrap_mut(v)?;
    let context = null_unwrap(context_ptr)?;
    let variable = VariableData::new_with_context(context)?;
    let mut context = context.borrow_mut()?;
    let name = CStr::from_ptr(name as _).to_owned();
    let result = Rc::as_ptr(&variable);
    context.variables.insert(name, variable);
    *v = result;
    Ok(())
}

// TODO: implement
pub(crate) fn validate(_context: Context) -> Result<(), RTresult> {
    Ok(())
}

pub(crate) unsafe fn set_ray_generation_program(
    context: Context,
    entry_point_index: u32,
    program: Program,
) -> Result<(), RTresult> {
    set_program(context, entry_point_index, program, |ctx| {
        &mut ctx.entry_points
    })
}

pub(crate) unsafe fn set_exception_program(
    context: Context,
    entry_point_index: u32,
    program: Program,
) -> Result<(), RTresult> {
    set_program(context, entry_point_index, program, |ctx| {
        &mut ctx.exception_programs
    })
}

unsafe fn set_program<F>(
    context: Context,
    entry_point_index: u32,
    program: Program,
    setter: F,
) -> Result<(), RTresult>
where
    F: for<'a> FnOnce(&'a mut ContextData) -> &'a mut Vec<Option<Rc<OptixCell<ProgramData>>>>,
{
    let context = null_unwrap(context)?;
    let mut context = (*context).borrow_mut()?;
    match setter(&mut context).get_mut(entry_point_index as usize) {
        None => Err(RTresult::RT_ERROR_INVALID_VALUE),
        Some(context_entry) => {
            let program = ManuallyDrop::new(Rc::from_raw(program));
            *context_entry = Some((&*program).clone());
            Ok(())
        }
    }
}

pub(crate) unsafe fn set_miss_program(
    context: Context,
    ray_type_index: u32,
    program: Program,
) -> Result<(), RTresult> {
    set_program(context, ray_type_index, program, |ctx| {
        &mut ctx.miss_programs
    })
}

pub(crate) unsafe fn launch_2d(
    context: Context,
    entry_point_index: u32,
    width: u64,
    height: u64,
) -> Result<(), RTresult> {
    if width > u32::MAX as u64 || height > u32::MAX as u64 {
        return Err(RTresult::RT_ERROR_NOT_SUPPORTED);
    }
    let context = null_unwrap(context)?;
    let mut context = (context).borrow_mut_no_invalidate()?;
    context.buffers_load_from_callback()?;
    if context.scene_rebuild_pending {
        context.scene = Scene::new(&context)?;
        context.scene_rebuild_pending = false;
    }
    if width == 0 || height == 0 {
        return Ok(());
    }
    context
        .global_stack
        .reallocate(width as u32, height as u32)?;
    let main_program = context
        .entry_points
        .get(entry_point_index as usize)
        .ok_or(RTresult::RT_ERROR_INVALID_VALUE)?
        .as_ref()
        .ok_or(RTresult::RT_ERROR_INVALID_VALUE)?;
    let main_program = main_program.borrow()?;
    let exception_program = context
        .exception_programs
        .get(entry_point_index as usize)
        .map(Option::as_ref)
        .flatten()
        .map(Rc::as_ref)
        .map(OptixCell::borrow)
        .transpose()?;
    context.scene.launch_2d(
        entry_point_index,
        &main_program,
        exception_program,
        width as u32,
        height as u32,
        context.global_stack.pointer,
    )
}

pub(crate) unsafe fn create_subobject<T: OptixObjectData>(
    context: Context,
    constructor: impl FnOnce(Weak<OptixCell<ContextData>>, &mut ContextData) -> T,
    register: impl FnOnce(Rc<OptixCell<T>>, &mut ContextData),
) -> Result<*const OptixCell<T>, RTresult> {
    let context = ManuallyDrop::new(Rc::from_raw(context));
    let weak_context = Rc::downgrade(&context);
    let mut context = (**context).borrow_mut()?;
    let object = Rc::new(OptixCell::new(constructor(weak_context, &mut context)));
    let result = Rc::as_ptr(&object);
    register(object, &mut *context);
    Ok(result)
}

pub(crate) fn set_stack_size(_context: Context, _bytes: u64) -> Result<(), RTresult> {
    Ok(())
}

pub(crate) fn set_max_depth(_context: Context, _max_depth: u32) -> Result<(), RTresult> {
    Ok(())
}

pub(crate) unsafe fn query_variable(
    context: Context,
    name: *const i8,
    v: *mut Variable,
) -> Result<(), RTresult> {
    null_check(name)?;
    null_check(v)?;
    let context = null_unwrap(context)?;
    let context = (context).borrow()?;
    *v = context
        .variables
        .get(CStr::from_ptr(name))
        .map(|variable| Rc::as_ptr(variable))
        .unwrap_or(ptr::null_mut());
    Ok(())
}

pub(crate) unsafe fn set_attribute(
    context: Context,
    attrib: RTcontextattribute,
    size: u64,
    p: *const std::ffi::c_void,
) -> Result<(), RTresult> {
    null_check(p)?;
    match attrib {
        RTcontextattribute::RT_CONTEXT_ATTRIBUTE_DISK_CACHE_LOCATION => {
            let context = null_unwrap(context)?;
            let mut context = OptixCell::borrow_mut_no_invalidate(context)?;
            let cache_location = CStr::from_ptr(p as _);
            let cache_location = cache_location
                .to_str()
                .map_err(|_| RTresult::RT_ERROR_UNKNOWN)?;
            let cache_location = PathBuf::from(cache_location);
            let cache = KernelCache::new(&cache_location).ok_or(RTresult::RT_ERROR_UNKNOWN)?;
            context.disk_cache_location = cache_location;
            context.cache = Some(cache);
        }
        RTcontextattribute::RT_CONTEXT_ATTRIBUTE_DISK_CACHE_ENABLED => {
            if size != mem::size_of::<u32>() as u64 {
                return Err(RTresult::RT_ERROR_INVALID_VALUE);
            }
            let value = p as *const u32;
            let context = null_unwrap(context)?;
            let mut context = OptixCell::borrow_mut_no_invalidate(context)?;
            if *value == 0 {
                context.cache = None;
            } else {
                context.cache = Some(
                    KernelCache::new(&context.disk_cache_location)
                        .ok_or(RTresult::RT_ERROR_UNKNOWN)?,
                );
            }
        }
        RTcontextattribute::RT_CONTEXT_ATTRIBUTE_PREFER_FAST_RECOMPILES => {}
        RTcontextattribute::RT_CONTEXT_ATTRIBUTE_PREFER_WATERTIGHT_TRAVERSAL => {}
        RTcontextattribute::RT_CONTEXT_ATTRIBUTE_PUBLIC_VENDOR_KEY => {
            let context = null_unwrap(context)?;
            let mut context = OptixCell::borrow_mut_no_invalidate(context)?;
            context.public_vendor_key =
                std::slice::from_raw_parts(p as *const u8, size as usize).to_vec();
        }
        RTcontextattribute::RT_CONTEXT_ATTRIBUTE_VENDOR_SALT => {
            if size != 32 {
                return Err(RTresult::RT_ERROR_NOT_SUPPORTED);
            }
            let context = null_unwrap(context)?;
            let mut context = OptixCell::borrow_mut_no_invalidate(context)?;
            context.vendor_salt = *(p as *const [u8; 32]);
        }
        RTcontextattribute::RT_CONTEXT_ATTRIBUTE_OPTIX_SALT => {
            if size != 32 {
                return Err(RTresult::RT_ERROR_NOT_SUPPORTED);
            }
            let context = null_unwrap(context)?;
            let mut context = OptixCell::borrow_mut_no_invalidate(context)?;
            context.optix_salt = *(p as *const [u8; 32]);
        }
        // TODO: implement
        RTcontextattribute::RT_CONTEXT_ATTRIBUTE_DISK_CACHE_MEMORY_LIMITS => {}
        // TODO: reverse
        RTcontextattribute(15)
        | RTcontextattribute(16)
        | RTcontextattribute(17)
        | RTcontextattribute(33554454) => return Err(RTresult::RT_ERROR_NOT_SUPPORTED),
        _ => return Err(definitions::unimplemented()),
    }
    Ok(())
}

pub(crate) fn set_max_callable_program_depth(_context: Context, _max_depth: u32) -> RTresult {
    RTresult::RT_SUCCESS
}

pub(crate) unsafe fn get_attribute(
    context: Context,
    attrib: RTcontextattribute,
    size: u64,
    p: *mut std::ffi::c_void,
) -> Result<(), RTresult> {
    null_check(p)?;
    match attrib {
        RTcontextattribute::RT_CONTEXT_ATTRIBUTE_DISK_CACHE_LOCATION => {
            let context = null_unwrap(context)?;
            let context = OptixCell::borrow(context)?;
            let cache = context
                .disk_cache_location
                .to_str()
                .ok_or(RTresult::RT_ERROR_UNKNOWN)?;
            *(p as *mut _) = cache.as_ptr();
        }
        RTcontextattribute::RT_CONTEXT_ATTRIBUTE_OPTIX_SALT => {
            if size != 32 {
                return Err(RTresult::RT_ERROR_NOT_SUPPORTED);
            }
            let context = null_unwrap(context)?;
            let context = OptixCell::borrow(context)?;
            *(p as *mut [u8; 32]) = context.optix_salt;
        }
        _ => return Err(definitions::unimplemented()),
    }
    Ok(())
}

pub(crate) unsafe fn set_devices(
    context: Context,
    count: u32,
    devices: *const i32,
) -> Result<(), RTresult> {
    null_check(context)?;
    if count != 1 {
        return Err(definitions::unimplemented());
    }
    if *devices != 0 {
        return Err(definitions::unimplemented());
    }
    Ok(())
}

pub(crate) fn set_exception_enabled(
    context: Context,
    _exception: RTexception,
    _enabled: i32,
) -> Result<(), RTresult> {
    null_check(context)?;
    Ok(())
}

pub(crate) fn set_print_enabled(
    context: *const OptixCell<ContextData>,
    _enabled: i32,
) -> Result<(), RTresult> {
    null_check(context)?;
    Ok(())
}

pub(crate) fn set_usage_report_callback(
    context: *const OptixCell<ContextData>,
    _callback: RTusagereportcallback,
    _verbosity: i32,
    _cbdata: *mut std::ffi::c_void,
) -> Result<(), RTresult> {
    null_check(context)?;
    Ok(())
}

pub(crate) fn set_print_launch_index(
    context: *const OptixCell<ContextData>,
    _x: i32,
    _y: i32,
    _z: i32,
) -> Result<(), RTresult> {
    null_check(context)?;
    Ok(())
}

pub(crate) unsafe fn get_device_count(context: Context, count: *mut u32) -> Result<(), RTresult> {
    null_check(context)?;
    *count = 1;
    Ok(())
}

pub(crate) unsafe fn get_devices(context: Context, devices: *mut i32) -> Result<(), RTresult> {
    null_check(context)?;
    *devices = 0;
    Ok(())
}

// Used only during Arnold deinitialization
pub(crate) unsafe fn get_buffer_from_id(
    context: Context,
    buffer_id: i32,
    buffer_output: *mut Buffer,
) -> Result<(), RTresult> {
    null_check(buffer_output)?;
    let context = null_unwrap(context)?;
    let context = context.borrow()?;
    for buffer_rc in context.buffers.iter() {
        let buffer = buffer_rc.borrow()?;
        if buffer.index == (buffer_id as u32) {
            *buffer_output = Rc::as_ptr(buffer_rc);
            return Ok(());
        }
    }
    *buffer_output = ptr::null_mut();
    Err(RTresult::RT_ERROR_INVALID_VALUE)
}

#[cfg(test)]
mod tests {
    use crate::optix_test;
    use crate::test_common::OptixFns;
    use std::ptr;

    optix_test!(create_destroy_context);

    unsafe fn create_destroy_context<Optix: OptixFns>(o: Optix) {
        let mut ctx = ptr::null_mut();
        o.rtContextCreate(&mut ctx);
        o.rtContextDestroy(ctx);
    }
}
