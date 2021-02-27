use crate::{
    acceleration::{Acceleration, AccelerationData, AccelerationOwner},
    context::{self, Context, ContextData},
    geometry_group::{BvhDetails, GeometryGroupData},
    geometry_instance::DeviceGeometryInstance,
    hip, hiprt, null_check, null_unwrap,
    repr_gpu::{self, TrivialHIPAllocator},
    transform::TransformData,
    MaybeWeakRefMut, OptixCell, OptixObjectData, TypeTag, TypedObjectWeak, UntypedObject,
};
use hip_runtime_sys::*;
use hiprt_sys::*;
use optix_types::*;
use std::{
    alloc::Layout,
    ptr,
    rc::{Rc, Weak},
};

pub(crate) type Group = *const OptixCell<GroupData>;

pub(crate) struct GroupData {
    pub(crate) context: Weak<OptixCell<ContextData>>,
    acceleration: Option<Rc<OptixCell<AccelerationData>>>,
    subgroups: Vec<Option<GroupChild>>,
    pub(crate) index: u32,
}

impl GroupData {
    fn new(weak_context: Weak<OptixCell<ContextData>>, context: &mut ContextData) -> Self {
        let index = context.geometry_group_count;
        context.geometry_group_count += 1;
        Self {
            context: weak_context,
            acceleration: None,
            subgroups: Vec::new(),
            index,
        }
    }

    fn register(this: Rc<OptixCell<Self>>, context: &mut ContextData) {
        context.groups.insert(this);
    }

    unsafe fn create(context: Context) -> Result<Group, RTresult> {
        context::create_subobject(context, Self::new, Self::register)
    }

    pub(crate) fn prepare_globals(
        &self,
        allocator: &mut TrivialHIPAllocator,
        ctx: &ContextData,
        global_state: &mut repr_gpu::GlobalState,
    ) -> Result<BvhDetails, RTresult> {
        let ray_type_count = ctx.ray_type_count;
        let mut transforms = Vec::new();
        let mut children = Vec::new();
        let acceleration = self
            .acceleration
            .as_ref()
            .ok_or(RTresult::RT_ERROR_UNKNOWN)?;
        let acceleration = acceleration.borrow()?;
        let mut build_flags = acceleration.to_hiprt().ok_or(RTresult::RT_ERROR_UNKNOWN)?;
        for child in self.subgroups.iter() {
            let child = child.as_ref().ok_or(RTresult::RT_ERROR_INVALID_CONTEXT)?;
            match child {
                GroupChild::GeometryGroup(group) => {
                    let group = Weak::upgrade(&group).ok_or(RTresult::RT_ERROR_INVALID_CONTEXT)?;
                    let group = group.borrow()?;
                    for _ in 0..group.children.len() {
                        transforms.push(None);
                    }
                    children.extend_from_slice(&group.children[..]);
                    let child_acceleration = group
                        .acceleration
                        .as_ref()
                        .ok_or(RTresult::RT_ERROR_UNKNOWN)?
                        .borrow()?;
                    let child_build_flags = child_acceleration
                        .to_hiprt()
                        .ok_or(RTresult::RT_ERROR_UNKNOWN)?;
                    build_flags = Self::join_build_flags(build_flags, child_build_flags);
                }
                GroupChild::Transform(transform) => {
                    let transform_rc =
                        Weak::upgrade(&transform).ok_or(RTresult::RT_ERROR_UNKNOWN)?;
                    let transform = transform_rc.borrow()?;
                    let dev_transform = transform.allocate(allocator)?;
                    let group = transform
                        .child
                        .as_ref()
                        .ok_or(RTresult::RT_ERROR_INVALID_CONTEXT)?;
                    let group = Weak::upgrade(group).ok_or(RTresult::RT_ERROR_UNKNOWN)?;
                    let group = group.borrow()?;
                    for _ in 0..group.children.len() {
                        transforms.push(Some((transform_rc.clone(), dev_transform)));
                    }
                    children.extend_from_slice(&group.children[..]);
                    let child_acceleration = group
                        .acceleration
                        .as_ref()
                        .ok_or(RTresult::RT_ERROR_UNKNOWN)?
                        .borrow()?;
                    let child_build_flags = child_acceleration
                        .to_hiprt()
                        .ok_or(RTresult::RT_ERROR_UNKNOWN)?;
                    build_flags = Self::join_build_flags(build_flags, child_build_flags);
                }
            }
        }
        let mut hiprt_instances_host = Vec::with_capacity(children.len());
        let custom_func_table = allocator.new_func_table()?;
        let mut custom_func_set_counter = 0u32;
        let attribute_visitor = repr_gpu::AttributesVisitCallChain {
            context: ctx,
            children: &*children,
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
                children: &children,
                context: ctx,
            };
            let closest_hit_visitor = repr_gpu::HitProgramsVisitCallChain {
                closest_hit: true,
                ray,
                children: &children,
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
        for (instance_index, instance) in children.iter().enumerate() {
            let instance = instance.as_ref().ok_or(RTresult::RT_ERROR_UNKNOWN)?;
            let instance = instance.borrow()?;
            match instance.prepare_globals(
                allocator,
                ctx,
                build_flags,
                ray_type_count,
                custom_func_set_counter,
                global_state,
                transforms[instance_index]
                    .as_ref()
                    .map(|(_, dev_ptr)| *dev_ptr)
                    .unwrap_or(hipDeviceptr_t(ptr::null_mut())),
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
        let instance_frames_host = transforms
            .iter()
            .map(|transform| {
                Ok::<_, RTresult>(match transform {
                    Some((transform, _)) => {
                        let transform = transform.borrow()?;
                        transform.to_hiprt()
                    }
                    None => hiprtFrame {
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
                    },
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let instance_geometries = allocator.copy_to_device(&hiprt_instances_host[..])?;
        let instance_frames = allocator.copy_to_device(&instance_frames_host[..])?;
        let scene_input = hiprtSceneBuildInput {
            instanceCount: children.len() as u32,
            instanceGeometries: instance_geometries.0,
            frameCount: children.len() as u32,
            instanceFrames: instance_frames.0,
            nodes: ptr::null_mut(),
            instanceTransformHeaders: ptr::null_mut(),
            instanceMasks: ptr::null_mut(),
        };
        let build_options = hiprtBuildOptions {
            buildFlags: build_flags.0,
        };
        let transform_blocks = transforms
            .iter()
            .map(|maybe_transform| {
                maybe_transform
                    .as_ref()
                    .map_or(hipDeviceptr_t(ptr::null_mut()), |(_, ptr)| *ptr)
            })
            .collect::<Vec<_>>();
        let transform_blocks = allocator.copy_to_device(&transform_blocks)?;
        let hit_chains = allocator.copy_to_device(&hit_chains)?;
        let scene = allocator.new_scene(scene_input, build_options)?;
        Ok(BvhDetails {
            scene,
            func_set: custom_func_table,
            attribute_call_chain,
            transform_blocks,
            hit_chains,
        })
    }

    // We assume lower 2 bits are operator, higher bits are flags
    fn join_build_flags(f1: hiprtBuildFlagBits, f2: hiprtBuildFlagBits) -> hiprtBuildFlagBits {
        let op = u32::max(f1.0 & 3, f2.0 & 3);
        let flags = ((f1.0 >> 2) | (f2.0 >> 2)) << 2;
        return hiprtBuildFlagBits(op | flags);
    }
}

impl OptixObjectData for GroupData {
    const TYPE: TypeTag = TypeTag::Group;

    fn deregister(&mut self, this: &Rc<OptixCell<Self>>) -> Result<(), RTresult> {
        if let Some(context) = self.context.upgrade() {
            let mut context = (*context).borrow_mut()?;
            context.groups.remove(this);
        }
        Ok(())
    }

    fn context<'a>(&'a mut self) -> crate::MaybeWeakRefMut<'a, ContextData> {
        MaybeWeakRefMut::Weak(&self.context)
    }
}

#[derive(Clone)]
enum GroupChild {
    GeometryGroup(Weak<OptixCell<GeometryGroupData>>),
    Transform(Weak<OptixCell<TransformData>>),
}

pub(crate) unsafe fn create(context: Context, group: *mut Group) -> Result<(), RTresult> {
    null_check(context)?;
    null_check(group)?;
    *group = GroupData::create(context)?;
    Ok(())
}

pub(crate) unsafe fn set_acceleration(
    group: Group,
    acceleration: Acceleration,
) -> Result<(), RTresult> {
    null_check(acceleration)?;
    let group = null_unwrap(group)?;
    let acceleration = null_unwrap(acceleration)?;
    {
        let mut group = group.borrow_mut()?;
        group.acceleration = Some(OptixCell::clone_rc(acceleration));
    }
    {
        let mut acceleration = acceleration.borrow_mut()?;
        acceleration.owner = Some(AccelerationOwner::Group(OptixCell::clone_weak(group)));
    }
    Ok(())
}

pub(crate) unsafe fn set_child_count(group: Group, count: u32) -> Result<(), RTresult> {
    let group = null_unwrap(group)?;
    let mut group = group.borrow_mut()?;
    group.subgroups.resize(count as usize, None);
    Ok(())
}

pub(crate) unsafe fn get_child(
    group: Group,
    index: u32,
    child: *mut UntypedObject,
) -> Result<(), RTresult> {
    let group = null_unwrap(group)?;
    let group = group.borrow()?;
    if index as usize >= group.subgroups.len() {
        *child = ptr::null_mut();
        return Err(RTresult::RT_ERROR_INVALID_VALUE);
    }
    let result = match &group.subgroups[index as usize] {
        Some(GroupChild::GeometryGroup(geo)) => {
            let geo = Weak::upgrade(&geo).ok_or(RTresult::RT_ERROR_INVALID_CONTEXT)?;
            OptixCell::as_untyped(&*geo)
        }
        Some(GroupChild::Transform(transform)) => {
            let transform = Weak::upgrade(&transform).ok_or(RTresult::RT_ERROR_INVALID_CONTEXT)?;
            OptixCell::as_untyped(&*transform)
        }
        None => ptr::null_mut(),
    };
    *child = result;
    Ok(())
}

pub(crate) unsafe fn get_child_count(group: Group, count: *mut u32) -> Result<(), RTresult> {
    null_check(count)?;
    let group = null_unwrap(group)?;
    let group = group.borrow()?;
    *count = group.subgroups.len() as u32;
    Ok(())
}

pub(crate) unsafe fn set_child(
    group: Group,
    index: u32,
    child: UntypedObject,
) -> Result<(), RTresult> {
    null_check(child)?;
    let group = null_unwrap(group)?;
    let mut group = group.borrow_mut()?;
    match group.subgroups.get_mut(index as usize) {
        Some(instance_slot) => {
            let child = TypedObjectWeak::clone_from(child)?;
            let group = match child {
                TypedObjectWeak::GeometryGroup(group) => GroupChild::GeometryGroup(group),
                TypedObjectWeak::Transform(transform) => GroupChild::Transform(transform),
                _ => return Err(RTresult::RT_ERROR_NOT_SUPPORTED),
            };
            *instance_slot = Some(group);
            Ok(())
        }
        None => Err(RTresult::RT_ERROR_INVALID_VALUE),
    }
}

pub(crate) unsafe fn get_acceleration(
    group: Group,
    acceleration: *mut Acceleration,
) -> Result<(), RTresult> {
    null_check(acceleration)?;
    let group = null_unwrap(group)?;
    let group = group.borrow()?;
    *acceleration = group
        .acceleration
        .as_ref()
        .map(Rc::as_ptr)
        .unwrap_or(ptr::null());
    Ok(())
}

pub(crate) fn destroy(_group: Group) -> Result<(), RTresult> {
    // TODO: implement
    Ok(())
}

pub(crate) unsafe fn get_context(
    group: *const OptixCell<GroupData>,
    context: *mut *const OptixCell<ContextData>,
) -> Result<(), RTresult> {
    let group = null_unwrap(group)?;
    let group = group.borrow()?;
    *context = group.context.as_ptr();
    Ok(())
}
