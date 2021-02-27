use crate::{
    acceleration::{Acceleration, AccelerationData, AccelerationOwner},
    context::{self, Context, ContextData},
    geometry_instance::{DeviceGeometryInstance, GeometryInstance, GeometryInstanceData},
    hip, hiprt, null_check, null_unwrap,
    repr_gpu::{self, TrivialHIPAllocator},
    MaybeWeakRefMut, OptixCell, OptixObjectData, TypeTag,
};
use hip_runtime_sys::*;
use hiprt_sys::*;
use optix_types::*;
use std::{
    alloc::Layout,
    ptr,
    rc::{Rc, Weak},
};

pub(crate) type GeometryGroup = *const OptixCell<GeometryGroupData>;

pub(crate) struct GeometryGroupData {
    pub(crate) context: Weak<OptixCell<ContextData>>,
    pub(crate) acceleration: Option<Rc<OptixCell<AccelerationData>>>,
    pub(crate) children: Vec<Option<Rc<OptixCell<GeometryInstanceData>>>>,
    pub(crate) index: u32,
    pub(crate) visibility: u32,
}

impl GeometryGroupData {
    fn new(weak_context: Weak<OptixCell<ContextData>>, context: &mut ContextData) -> Self {
        let index = context.geometry_group_count;
        context.geometry_group_count += 1;
        Self {
            context: weak_context,
            acceleration: None,
            children: Vec::new(),
            index,
            visibility: !0u32,
        }
    }

    fn register(this: Rc<OptixCell<Self>>, context: &mut ContextData) {
        context.geometry_groups.insert(this);
    }

    unsafe fn create(context: Context) -> Result<GeometryGroup, RTresult> {
        context::create_subobject(context, Self::new, Self::register)
    }

    pub(crate) fn prepare_globals(
        &self,
        allocator: &mut TrivialHIPAllocator,
        ctx: &ContextData,
        global_state: &mut repr_gpu::GlobalState,
    ) -> Result<BvhDetails, RTresult> {
        let ray_type_count = ctx.ray_type_count;
        let acceleration = self
            .acceleration
            .as_ref()
            .ok_or(RTresult::RT_ERROR_UNKNOWN)?;
        let acceleration = acceleration.borrow()?;
        let build_flags = acceleration.to_hiprt().ok_or(RTresult::RT_ERROR_UNKNOWN)?;
        let mut hiprt_instances_host = Vec::with_capacity(self.children.len());
        let custom_func_table = allocator.new_func_table()?;
        let mut custom_func_set_counter = 0u32;
        let attribute_visitor = repr_gpu::AttributesVisitCallChain {
            context: ctx,
            children: &*self.children,
        };
        let attribute_chain_layout = repr_gpu::get_layout(ray_type_count, &attribute_visitor)?;
        let attribute_call_chain = allocator.allocate(attribute_chain_layout.layout.size())?;
        repr_gpu::copy_to_gpu(
            ray_type_count,
            &attribute_visitor,
            &attribute_chain_layout,
            attribute_call_chain,
        )?;
        let hit_chains = (0..ray_type_count).map(|ray| {
            let prologue_layout = Layout::new::<repr_gpu::HitProgramChain>();
            let any_hit_visitor = repr_gpu::HitProgramsVisitCallChain {
                closest_hit: false,
                ray,
                children: &self.children,
                context: ctx,
            };
            let closest_hit_visitor = repr_gpu::HitProgramsVisitCallChain {
                closest_hit: true,
                ray,
                children: &self.children,
                context: ctx,
            };
            let any_hit_chain_layout = repr_gpu::get_layout(ray_type_count, &any_hit_visitor)?;
            let closest_hit_chain_layout = repr_gpu::get_layout(ray_type_count, &closest_hit_visitor)?;
            let with_any_hit = prologue_layout
                .extend(any_hit_chain_layout.layout)
                .map_err(|_| RTresult::RT_ERROR_UNKNOWN)?;
            let with_closest_hit = with_any_hit
                .0
                .extend(closest_hit_chain_layout.layout)
                .map_err(|_| RTresult::RT_ERROR_UNKNOWN)?;
            let hit_chain_gpu = allocator.allocate(with_closest_hit.0.size())?;
            let prolog = repr_gpu::HitProgramChain {
                any_hit_start: with_any_hit.1 as u32,
                closest_hit_start: with_closest_hit.1 as u32,
            };
            hip! { hipMemcpyHtoD(hit_chain_gpu, &prolog as *const _ as _, prologue_layout.size()), RT_ERROR_UNKNOWN };
            repr_gpu::copy_to_gpu(
                ray_type_count,
                &any_hit_visitor,
                &any_hit_chain_layout,
                hipDeviceptr_t(unsafe { hit_chain_gpu.0.cast::<u8>().add(with_any_hit.1).cast() }),
            )?;
            repr_gpu::copy_to_gpu(
                ray_type_count,
                &closest_hit_visitor,
                &closest_hit_chain_layout,
                hipDeviceptr_t(unsafe {
                    hit_chain_gpu.0.cast::<u8>().add(with_closest_hit.1).cast()
                }),
            )?;
            Ok(hit_chain_gpu)
        }).collect::<Result<Vec<_>, _>>()?;
        for instance in self.children.iter() {
            let instance = instance.as_ref().ok_or(RTresult::RT_ERROR_UNKNOWN)?;
            let instance = instance.borrow()?;
            match instance.prepare_globals(
                allocator,
                ctx,
                build_flags,
                ray_type_count,
                custom_func_set_counter,
                global_state,
                hipDeviceptr_t(ptr::null_mut()),
            )? {
                DeviceGeometryInstance::Geometry {
                    custom_func_set,
                    hiprt_geometry,
                } => {
                    hiprt_instances_host.push(hiprt_geometry);
                    hiprt! { ctx.hiprt.hiprtSetCustomFuncTable(ctx.context, custom_func_table, custom_func_set_counter, custom_func_set), RT_ERROR_UNKNOWN };
                    custom_func_set_counter += 1;
                }
                DeviceGeometryInstance::GeometryTriangles { hiprt_geometry } => {
                    hiprt_instances_host.push(hiprt_geometry);
                }
            };
        }
        let instance_frames_host = vec![
            hiprtFrame {
                translation: hiprtFloat3 {
                    x: 0f32,
                    y: 0f32,
                    z: 0f32,
                },
                scale: hiprtFloat3 {
                    x: 1f32,
                    y: 1f32,
                    z: 1f32,
                },
                rotation: hiprtFloat4 {
                    x: 0f32,
                    y: 0f32,
                    z: 1.0f32,
                    w: 0f32,
                },
                time: 0f32,
                pad: 0,
            };
            self.children.len()
        ];
        let instance_geometries = allocator.copy_to_device(&hiprt_instances_host[..])?;
        let instance_frames = allocator.copy_to_device(&instance_frames_host[..])?;
        let instance_mask_host = vec![self.visibility; self.children.len()];
        let instance_masks = allocator.copy_to_device(&instance_mask_host[..])?;
        let scene_input = hiprtSceneBuildInput {
            instanceCount: self.children.len() as u32,
            instanceGeometries: instance_geometries.0,
            frameCount: self.children.len() as u32,
            instanceFrames: instance_frames.0,
            nodes: ptr::null_mut(),
            instanceTransformHeaders: ptr::null_mut(),
            instanceMasks: instance_masks.0,
        };
        let build_options = hiprtBuildOptions {
            buildFlags: build_flags.0,
        };
        let transform_blocks = vec![hipDeviceptr_t(ptr::null_mut()); self.children.len()];
        let transform_blocks = allocator.copy_to_device(&transform_blocks)?;
        let hit_chains = allocator.copy_to_device(&hit_chains)?;
        let scene = allocator.new_scene(scene_input, build_options)?;
        Ok(BvhDetails {
            scene,
            attribute_call_chain,
            func_set: custom_func_table,
            transform_blocks,
            hit_chains,
        })
    }
}

impl OptixObjectData for GeometryGroupData {
    const TYPE: TypeTag = TypeTag::GeometryGroup;

    fn deregister(&mut self, this: &Rc<OptixCell<Self>>) -> Result<(), RTresult> {
        if let Some(context) = self.context.upgrade() {
            let mut context = (*context).borrow_mut()?;
            context.geometry_groups.remove(this);
        }
        Ok(())
    }

    fn context<'a>(&'a mut self) -> crate::MaybeWeakRefMut<'a, ContextData> {
        MaybeWeakRefMut::Weak(&self.context)
    }
}

#[repr(C)]
pub(crate) struct BvhDetails {
    pub(crate) scene: hiprtScene,
    pub(crate) func_set: hiprtCustomFuncTable,
    pub(crate) attribute_call_chain: hipDeviceptr_t,
    pub(crate) transform_blocks: hipDeviceptr_t,
    pub(crate) hit_chains: hipDeviceptr_t,
}

pub(crate) unsafe fn create(
    context: Context,
    geometry_group: *mut GeometryGroup,
) -> Result<(), RTresult> {
    null_check(context)?;
    null_check(geometry_group)?;
    *geometry_group = GeometryGroupData::create(context)?;
    Ok(())
}

pub(crate) unsafe fn set_acceleration(
    geometry_group: GeometryGroup,
    acceleration: Acceleration,
) -> Result<(), RTresult> {
    null_check(acceleration)?;
    let geometry_group = null_unwrap(geometry_group)?;
    let acceleration = null_unwrap(acceleration)?;
    {
        let mut geometry_group = geometry_group.borrow_mut()?;
        geometry_group.acceleration = Some(OptixCell::clone_rc(acceleration));
    }
    {
        let mut acceleration = acceleration.borrow_mut()?;
        acceleration.owner = Some(AccelerationOwner::GeometryGroup(OptixCell::clone_weak(
            geometry_group,
        )));
    }
    Ok(())
}

pub(crate) unsafe fn set_child(
    geometry_group: GeometryGroup,
    index: u32,
    geometry_instance: GeometryInstance,
) -> Result<(), RTresult> {
    null_check(geometry_instance)?;
    let geometry_group = null_unwrap(geometry_group)?;
    let mut geometry_group = geometry_group.borrow_mut()?;
    match geometry_group.children.get_mut(index as usize) {
        Some(instance_slot) => {
            *instance_slot = Some(OptixCell::clone_rc(geometry_instance));
            Ok(())
        }
        None => Err(RTresult::RT_ERROR_INVALID_VALUE),
    }
}

pub(crate) unsafe fn set_child_count(
    geometry_group: GeometryGroup,
    count: u32,
) -> Result<(), RTresult> {
    let geometry_group = null_unwrap(geometry_group)?;
    let mut geometry_group = geometry_group.borrow_mut()?;
    geometry_group.children.resize(count as usize, None);
    Ok(())
}

pub(crate) unsafe fn get_child_count(
    geometry_group: GeometryGroup,
    count: *mut u32,
) -> Result<(), RTresult> {
    null_check(count)?;
    let geometry_group = null_unwrap(geometry_group)?;
    let geometry_group = geometry_group.borrow()?;
    *count = geometry_group.children.len() as u32;
    Ok(())
}

pub(crate) unsafe fn set_visibility_mask(
    geometry_group: *const OptixCell<GeometryGroupData>,
    mask: u32,
) -> Result<(), RTresult> {
    let geometry_group = null_unwrap(geometry_group)?;
    let mut geometry_group = geometry_group.borrow_mut()?;
    geometry_group.visibility = mask;
    Ok(())
}

pub(crate) unsafe fn get_context(
    geometrygroup: GeometryGroup,
    context: *mut Context,
) -> Result<(), RTresult> {
    let geometrygroup = null_unwrap(geometrygroup)?;
    let geometrygroup = geometrygroup.borrow()?;
    *context = geometrygroup.context.as_ptr();
    Ok(())
}

pub(crate) fn destroy(_geometrygroup: GeometryGroup) -> Result<(), RTresult> {
    // TODO: implement
    Ok(())
}
