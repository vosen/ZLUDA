use crate::acceleration::AccelerationOwner;
use crate::context::GlobalStack;
use crate::geometry_instance::{GeometryInstanceChild, GeometryInstanceData};
use crate::geometry_triangles::GeometryTrianglesData;
use crate::material::MaterialData;
use crate::{
    context::ContextData, hip, program::ProgramData, slice_cast_mut, AlignedBuffer, OptixCell,
};
use crate::{hiprt, unwrap_or_continue};
use hip_runtime_sys::{hipDeviceptr_t, hipMemcpyHtoD};
use hiprt_sys::*;
use optix_types::RTresult;
use static_assertions::const_assert_eq;
use std::rc::Weak;
use std::{alloc::Layout, rc::Rc};
use std::{iter, mem, ptr};

#[repr(C)]
#[derive(Clone)]
pub(crate) struct GlobalState {
    pub(crate) scenes: hipDeviceptr_t,
    pub(crate) miss_programs: hipDeviceptr_t,
    pub(crate) buffers: hipDeviceptr_t,
    pub(crate) callable_programs: hipDeviceptr_t,
    pub(crate) textures: hipDeviceptr_t,
    pub(crate) ray_type_count: u32,
    pub(crate) uv_attribute_offset: u32,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) attribute_block_size: u16,
    pub(crate) attribute_block_align: u16,
    pub(crate) thread_global_stack_size: u16,
    pub(crate) _padding: u16,
}
const_assert_eq!(mem::size_of::<GlobalState>(), 64);

#[repr(C)]
#[allow(dead_code)]
pub(crate) struct HitProgramChain {
    pub(crate) any_hit_start: u32,
    pub(crate) closest_hit_start: u32,
}

#[repr(C)]
#[allow(dead_code)]
pub(crate) struct IntersectionInput {
    pub transform_block: hipDeviceptr_t,
    pub materials_start: u32,
}

#[repr(C)]
#[allow(dead_code)]
pub(crate) struct OptixTransform {
    pub transform: [f32; 16],
    pub inverse_transform: [f32; 16],
}

pub(crate) struct Scene {
    exception_variable_blocks: Vec<Option<hipDeviceptr_t>>,
    entry_point_variable_blocks: Vec<Option<hipDeviceptr_t>>,
    global_state: GlobalState,
    // _allocator is used implictly, it holds all the allocations used by the Scene object
    _allocator: Option<TrivialHIPAllocator>,
}

impl Scene {
    pub fn empty() -> Self {
        Self {
            exception_variable_blocks: Vec::new(),
            entry_point_variable_blocks: Vec::new(),
            global_state: unsafe { mem::zeroed() },
            _allocator: None,
        }
    }

    pub fn new(context: &ContextData) -> Result<Self, RTresult> {
        let mut allocator = TrivialHIPAllocator::new(context.context, context.hiprt.clone());
        let textures = Self::allocate_texture_samplers(context, &mut allocator)?;
        let mut bvhs = (0..context.geometry_group_count)
            .into_iter()
            .map(|_| unsafe { mem::zeroed() })
            .collect::<Vec<_>>();
        let miss_programs = context.allocate_miss_programs(&mut allocator)?;
        let ray_type_count = context.ray_type_count;
        let callable_programs = context.allocate_callable_programs(&mut allocator)?;
        let uv_attribute_offset = context.get_uv_offset()?;
        let buffers = context.allocate_buffers(&mut allocator)?;
        let (attribute_block_size, attribute_block_align) = context.attributes_layout();
        let entry_point_variable_blocks =
            Self::allocate_entry_variable_blocks(&mut allocator, context)?;
        let exception_variable_blocks =
            Self::allocate_exception_variable_blocks(&mut allocator, context)?;
        let mut global_state = GlobalState {
            scenes: hipDeviceptr_t(ptr::null_mut()),
            miss_programs,
            buffers,
            callable_programs,
            textures,
            ray_type_count,
            uv_attribute_offset,
            attribute_block_size,
            attribute_block_align,
            width: 0,
            height: 0,
            thread_global_stack_size: GlobalStack::THREAD_STACK_DEPTH,
            _padding: 0,
        };
        for accel in context.accelerations.iter() {
            let accel = OptixCell::borrow(accel)?;
            if let Some(group) = accel.owner.as_ref() {
                let (index, bvh) = match group {
                    AccelerationOwner::GeometryGroup(geo_group) => {
                        if let Some(geo_group) = geo_group.upgrade() {
                            let geo_group = OptixCell::borrow(&*geo_group)?;
                            let index = geo_group.index;
                            let bvh = geo_group.prepare_globals(
                                &mut allocator,
                                context,
                                &mut global_state,
                            )?;
                            (index, bvh)
                        } else {
                            continue;
                        }
                    }
                    AccelerationOwner::Group(group) => {
                        if let Some(group) = group.upgrade() {
                            let group = OptixCell::borrow(&*group)?;
                            let index = group.index;
                            let bvh = group.prepare_globals(
                                &mut allocator,
                                context,
                                &mut global_state,
                            )?;
                            (index, bvh)
                        } else {
                            continue;
                        }
                    }
                };
                bvhs[index as usize] = bvh;
            }
        }
        global_state.scenes = allocator.copy_to_device(&bvhs)?;
        Ok(Self {
            entry_point_variable_blocks,
            exception_variable_blocks,
            global_state,
            _allocator: Some(allocator),
        })
    }

    fn allocate_texture_samplers(
        context: &ContextData,
        allocator: &mut TrivialHIPAllocator,
    ) -> Result<hipDeviceptr_t, RTresult> {
        let mut sampler_ptrs = vec![ptr::null_mut(); context.texture_counter as usize];
        for sampler in context.texture_samplers.iter() {
            let mut sampler = sampler.borrow_mut()?;
            unsafe { sampler.create_underlying()? };
            sampler_ptrs[sampler.index as usize - 1] = sampler.hip_object;
        }
        allocator.copy_to_device(&sampler_ptrs[..])
    }

    pub fn launch_2d(
        &self,
        entry_point_index: u32,
        main_program: &ProgramData,
        exception: Option<std::cell::Ref<ProgramData>>,
        width: u32,
        height: u32,
        stack: hipDeviceptr_t,
    ) -> Result<(), RTresult> {
        let program_variable_block = self
            .entry_point_variable_blocks
            .get(entry_point_index as usize)
            .ok_or(RTresult::RT_ERROR_INVALID_VALUE)?
            .ok_or(RTresult::RT_ERROR_INVALID_VALUE)?;
        let exception = exception
            .map(|program| {
                let exception = program.get_function()?;
                let var_block = self
                    .exception_variable_blocks
                    .get(entry_point_index as usize)
                    .ok_or(RTresult::RT_ERROR_INVALID_VALUE)?
                    .ok_or(RTresult::RT_ERROR_INVALID_VALUE)?;
                Ok::<_, RTresult>((exception, var_block))
            })
            .transpose()?
            .unwrap_or((
                hipDeviceptr_t(ptr::null_mut()),
                hipDeviceptr_t(ptr::null_mut()),
            ));
        main_program.launch_2d(
            width,
            height,
            &self.global_state,
            stack,
            program_variable_block,
            exception,
        )
    }

    fn allocate_entry_variable_blocks(
        allocator: &mut TrivialHIPAllocator,
        context: &ContextData,
    ) -> Result<Vec<Option<hipDeviceptr_t>>, RTresult> {
        context
            .entry_points
            .iter()
            .map(|program| {
                program
                    .as_ref()
                    .map(|program| {
                        let program = program.borrow()?;
                        program.prepare_variable_block_for_kernel(allocator, context)
                    })
                    .transpose()
            })
            .collect::<Result<Vec<_>, _>>()
    }

    fn allocate_exception_variable_blocks(
        allocator: &mut TrivialHIPAllocator,
        context: &ContextData,
    ) -> Result<Vec<Option<hipDeviceptr_t>>, RTresult> {
        context
            .exception_programs
            .iter()
            .map(|program| {
                program
                    .as_ref()
                    .map(|program| {
                        let program = program.borrow()?;
                        program.prepare_variable_block_for_kernel(allocator, context)
                    })
                    .transpose()
            })
            .collect::<Result<Vec<_>, _>>()
    }
}

pub(crate) fn get_layout<S: VisitCallChain>(
    ray_type_count: u32,
    layout_source: &S,
) -> Result<CallChainLayout, RTresult> {
    let containers_len = layout_source.len(ray_type_count);
    if containers_len == 0 {
        return Ok(CallChainLayout {
            layout: Layout::new::<()>(),
            outer_offsets: vec![u32::MAX; containers_len],
            inner_offsets: vec![vec![]; containers_len],
        });
    }
    let mut layouts = vec![vec![]; containers_len];
    layout_source.visit_alloc(ray_type_count, |outer_index, _, programs| {
        let maybe_layout = programs
            .map(|(_, prog)| S::get_program_block_layout(prog))
            .transpose()?;
        layouts[outer_index].push(maybe_layout);
        Ok(())
    })?;
    let mut layout = Layout::new::<()>();
    layout = layout_extend_by_offset_array(layout, containers_len)?;
    let mut outer_offsets = Vec::with_capacity(containers_len);
    let mut inner_offsets = Vec::with_capacity(containers_len);
    for sublayouts in layouts {
        if sublayouts.len() == 0 {
            outer_offsets.push(u32::MAX);
            inner_offsets.push(Vec::new());
            continue;
        }
        let inner_layout_and_offsets = if sublayouts.len() == 1 {
            let program = sublayouts[0];
            match program {
                Some(inner_layout) => Some((inner_layout, vec![0])),
                None => None,
            }
        } else {
            let mut offsets = Vec::with_capacity(sublayouts.len());
            let mut inner_layout =
                layout_extend_by_offset_array(Layout::new::<()>(), sublayouts.len())?;
            let mut callable = 0usize;
            for sublayout in sublayouts {
                let prog_block_layout = match sublayout {
                    None => {
                        offsets.push(u32::MAX);
                        continue;
                    }
                    Some(l) => l,
                };
                callable += 1;
                let (new_layout, offset) = inner_layout
                    .extend(prog_block_layout)
                    .map_err(|_| RTresult::RT_ERROR_UNKNOWN)?;
                inner_layout = new_layout;
                offsets.push(offset as u32);
            }
            if callable == 0 {
                None
            } else {
                Some((inner_layout, offsets))
            }
        };
        match inner_layout_and_offsets {
            Some((inner_layout, offsets)) => {
                let (outer_layout, mut outer_offset) = layout
                    .extend(inner_layout)
                    .map_err(|_| RTresult::RT_ERROR_UNKNOWN)?;
                if offsets.len() > 1 {
                    //debug_assert!(outer_offset & 1 == 0);
                    outer_offset |= 1;
                }
                outer_offsets.push(outer_offset as u32);
                inner_offsets.push(offsets);
                layout = outer_layout;
            }
            None => {
                outer_offsets.push(u32::MAX);
                inner_offsets.push(Vec::new());
            }
        }
    }
    Ok(CallChainLayout {
        layout,
        outer_offsets,
        inner_offsets,
    })
}

pub(crate) fn copy_to_gpu<S: VisitCallChain>(
    ray_type_count: u32,
    layout_source: &S,
    chain_layout: &CallChainLayout,
    destination: hipDeviceptr_t,
) -> Result<(), RTresult> {
    let staging_buffer = copy_to_cpu(ray_type_count, layout_source, chain_layout)?;
    hip! { hipMemcpyHtoD(destination, staging_buffer.as_ptr(), staging_buffer.len()), RT_ERROR_UNKNOWN };
    Ok(())
}

pub(crate) fn copy_to_cpu<S: VisitCallChain>(
    ray_type_count: u32,
    layout_source: &S,
    chain_layout: &CallChainLayout,
) -> Result<AlignedBuffer, RTresult> {
    let mut staging_buffer = AlignedBuffer::new(chain_layout.layout);
    if cfg!(test) {
        staging_buffer.as_bytes_mut().fill(0u8);
    }
    let dst_buffer = staging_buffer.as_bytes_mut();
    copy_prologue_offsets(chain_layout, dst_buffer);
    layout_source.visit_alloc(ray_type_count, |outer_index, inner_index, program| {
        if let Some((copy_ctx, program)) = program {
            let outer_offset = chain_layout.outer_offsets[outer_index] & !1u32;
            let offset = outer_offset + chain_layout.inner_offsets[outer_index][inner_index];
            layout_source.copy_program_block(
                copy_ctx,
                program,
                &mut dst_buffer[offset as usize..],
            )?;
        }
        Ok(())
    })?;
    Ok(staging_buffer)
}

#[gat]
pub(crate) trait VisitCallChain {
    type ProgramData; // This type parametrization is to enable testing
    type CopyContext<'temp>;
    fn len(&self, ray_type_count: u32) -> usize;
    fn visit_alloc(
        &self,
        ray_type_count: u32,
        visitor: impl FnMut(
            usize,
            usize,
            Option<(Self::CopyContext<'_>, &Self::ProgramData)>,
        ) -> Result<(), RTresult>,
    ) -> Result<(), RTresult>;
    fn get_program_block_layout(p: &Self::ProgramData) -> Result<Layout, RTresult>;
    fn copy_program_block(
        &self,
        copy_ctx: Self::CopyContext<'_>,
        p: &Self::ProgramData,
        dst: &mut [u8],
    ) -> Result<(), RTresult>;
}

trait VisitLayout {
    fn len(&self, ray_type_count: u32) -> usize;
    fn visit_layout(
        &self,
        ray_type_count: u32,
        visitor: impl FnMut(Vec<Option<Layout>>) -> Result<(), RTresult>,
    ) -> Result<(), RTresult>;
}

impl VisitLayout for GeometryInstanceData {
    fn len(&self, ray_type_count: u32) -> usize {
        self.materials.len() * ray_type_count as usize
    }

    fn visit_layout(
        &self,
        ray_type_count: u32,
        mut visitor: impl FnMut(Vec<Option<Layout>>) -> Result<(), RTresult>,
    ) -> Result<(), RTresult> {
        for material in self.materials.iter() {
            for ray in 0..ray_type_count {
                let material = material.as_ref().ok_or(RTresult::RT_ERROR_UNKNOWN)?;
                let material = material.borrow()?;
                let maybe_any_hit_program = material.any_hit_programs.get(&ray);
                visitor(vec![maybe_any_hit_program
                    .map(|any_hit_program| {
                        let any_hit_program = any_hit_program.borrow()?;
                        any_hit_program.get_program_block_layout()
                    })
                    .transpose()?])?
            }
        }
        Ok(())
    }
}

pub(crate) struct IntersectVisitCallChain<'a> {
    pub(crate) context: &'a ContextData,
    pub(crate) geometry_instance: &'a GeometryInstanceData,
}

pub(crate) struct CallableProgramsVisitor<'a> {
    context: &'a ContextData,
    programs: Vec<Option<std::cell::Ref<'a, ProgramData>>>,
}

impl<'a> CallableProgramsVisitor<'a> {
    pub(crate) fn new(context: &'a ContextData) -> Result<Self, RTresult> {
        let mut programs = (0..context.callable_program_counter)
            .into_iter()
            .map(|_| None)
            .collect::<Vec<_>>();
        for program in context.programs.iter() {
            let program = program.borrow()?;
            let callable_index = unwrap_or_continue!(program.callable_index);
            let index = (callable_index - 1) as usize;
            programs[index] = Some(program);
        }
        Ok(Self { programs, context })
    }
}

#[gat]
impl<'a> VisitCallChain for CallableProgramsVisitor<'a> {
    type ProgramData = ProgramData;
    type CopyContext<'unused> = ();

    fn len(&self, _ray_type_count: u32) -> usize {
        self.programs.len()
    }

    fn visit_alloc(
        &self,
        _ray_type_count: u32,
        mut visitor: impl FnMut(usize, usize, Option<((), &Self::ProgramData)>) -> Result<(), RTresult>,
    ) -> Result<(), RTresult> {
        for (index, prog) in self.programs.iter().enumerate() {
            visitor(index, 0, prog.as_deref().map(|prog| ((), prog)))?;
        }
        Ok(())
    }

    fn get_program_block_layout(prog: &Self::ProgramData) -> Result<Layout, RTresult> {
        prog.get_program_block_layout()
    }

    fn copy_program_block(
        &self,
        _: (),
        prog: &Self::ProgramData,
        dst: &mut [u8],
    ) -> Result<(), RTresult> {
        prog.copy_program_block(dst, |name| prog.get_variable_for_kernel(self.context, name))
    }
}

#[gat]
impl<'a> VisitCallChain for IntersectVisitCallChain<'a> {
    type ProgramData = ProgramData;
    type CopyContext<'temp> = &'temp MaterialData;

    fn len(&self, ray_type_count: u32) -> usize {
        self.geometry_instance.materials.len() * ray_type_count as usize
    }

    fn visit_alloc(
        &self,
        ray_type_count: u32,
        mut visitor: impl FnMut(
            usize,
            usize,
            Option<(&MaterialData, &Self::ProgramData)>,
        ) -> Result<(), RTresult>,
    ) -> Result<(), RTresult> {
        for (material_index, material) in self.geometry_instance.materials.iter().enumerate() {
            for ray in 0..ray_type_count {
                let material = material.as_ref().ok_or(RTresult::RT_ERROR_UNKNOWN)?;
                let material = material.borrow()?;
                let maybe_any_hit_program = material.any_hit_programs.get(&ray);
                let outer_index = (ray_type_count as usize * material_index) + ray as usize;
                visitor(
                    outer_index,
                    0,
                    maybe_any_hit_program
                        .map(std::ops::Deref::deref)
                        .map(OptixCell::borrow)
                        .transpose()?
                        .as_deref()
                        .map(|prog| (&*material, prog)),
                )?;
            }
        }
        Ok(())
    }

    fn get_program_block_layout(p: &Self::ProgramData) -> Result<Layout, RTresult> {
        p.get_program_block_layout()
    }

    fn copy_program_block(
        &self,
        material: &MaterialData,
        p: &Self::ProgramData,
        dst: &mut [u8],
    ) -> Result<(), RTresult> {
        p.copy_program_block(dst, |name| {
            p.get_variable_for_function(self.geometry_instance, material, self.context, name)
        })
    }
}

pub(crate) struct AttributesVisitCallChain<'a> {
    pub(crate) context: &'a ContextData,
    pub(crate) children: &'a [Option<Rc<OptixCell<GeometryInstanceData>>>],
}

#[gat]
impl<'a> VisitCallChain for AttributesVisitCallChain<'a> {
    type ProgramData = ProgramData;
    type CopyContext<'temp> = (&'temp GeometryTrianglesData, &'temp GeometryInstanceData);

    fn len(&self, _: u32) -> usize {
        self.children.len()
    }

    fn visit_alloc(
        &self,
        _: u32,
        mut visitor: impl for<'x> FnMut(
            usize,
            usize,
            Option<(
                (&'x GeometryTrianglesData, &'x GeometryInstanceData),
                &Self::ProgramData,
            )>,
        ) -> Result<(), RTresult>,
    ) -> Result<(), RTresult> {
        for (program_index, maybe_instance) in self.children.iter().enumerate() {
            if let Some(instance) = maybe_instance {
                let instance = instance.borrow()?;
                match instance.child {
                    GeometryInstanceChild::None => return Err(RTresult::RT_ERROR_INVALID_CONTEXT),
                    GeometryInstanceChild::Geometry(_) => {}
                    GeometryInstanceChild::GeometryTriangles(ref geo) => {
                        let geo = geo.borrow()?;
                        if let Some(ref program) = geo.attribute_program {
                            if let Some(program) = Weak::upgrade(&program) {
                                let program = program.borrow()?;
                                visitor(program_index, 0, Some(((&geo, &instance), &program)))?;
                                continue;
                            }
                        }
                    }
                }
                visitor(program_index, 0, None)?;
            } else {
                return Err(RTresult::RT_ERROR_INVALID_CONTEXT);
            }
        }
        Ok(())
    }

    fn get_program_block_layout(p: &Self::ProgramData) -> Result<Layout, RTresult> {
        p.get_program_block_layout()
    }

    fn copy_program_block(
        &self,
        (triangles, instance): (&GeometryTrianglesData, &GeometryInstanceData),
        p: &Self::ProgramData,
        dst: &mut [u8],
    ) -> Result<(), RTresult> {
        p.copy_attribute_program_block(dst, |name| {
            p.get_variable_for_attribute(triangles, instance, self.context, name)
        })
    }
}

pub(crate) struct MissProgramsVisitCallChain<'a> {
    pub(crate) context: &'a ContextData,
    pub(crate) miss_programs: &'a [Option<Rc<OptixCell<ProgramData>>>],
}

#[gat]
impl<'a> VisitCallChain for MissProgramsVisitCallChain<'a> {
    type ProgramData = ProgramData;
    type CopyContext<'unused> = ();

    fn len(&self, _: u32) -> usize {
        self.miss_programs.len()
    }

    fn visit_alloc(
        &self,
        _: u32,
        mut visitor: impl FnMut(usize, usize, Option<((), &Self::ProgramData)>) -> Result<(), RTresult>,
    ) -> Result<(), RTresult> {
        for (program_index, maybe_program) in self.miss_programs.iter().enumerate() {
            visitor(
                program_index,
                0,
                maybe_program
                    .as_deref()
                    .map(OptixCell::borrow)
                    .transpose()?
                    .as_deref()
                    .map(|prog| ((), prog)),
            )?;
        }
        Ok(())
    }

    fn get_program_block_layout(p: &Self::ProgramData) -> Result<Layout, RTresult> {
        p.get_program_block_layout()
    }

    fn copy_program_block(
        &self,
        _: (),
        p: &Self::ProgramData,
        dst: &mut [u8],
    ) -> Result<(), RTresult> {
        p.copy_program_block(dst, |name| p.get_variable_for_kernel(self.context, name))
    }
}

pub(crate) struct HitProgramsVisitCallChain<'a> {
    pub(crate) context: &'a ContextData,
    pub(crate) children: &'a [Option<Rc<OptixCell<GeometryInstanceData>>>],
    pub(crate) closest_hit: bool,
    pub(crate) ray: u32,
}

#[gat]
impl<'a> VisitCallChain for HitProgramsVisitCallChain<'a> {
    type ProgramData = ProgramData;
    type CopyContext<'temp> = (&'temp GeometryInstanceData, &'temp MaterialData);

    fn len(&self, _: u32) -> usize {
        self.children.len()
    }

    fn visit_alloc(
        &self,
        _: u32,
        mut visitor: impl for<'x, 'y> FnMut(
            usize,
            usize,
            Option<(
                (&'x GeometryInstanceData, &'x MaterialData),
                &'y Self::ProgramData,
            )>,
        ) -> Result<(), RTresult>,
    ) -> Result<(), RTresult> {
        for (instance_idx, maybe_instance) in self.children.iter().enumerate() {
            let instance = maybe_instance
                .as_ref()
                .ok_or(RTresult::RT_ERROR_INVALID_CONTEXT)?;
            let instance = instance.borrow()?;
            let ignore_any_hit = matches!(instance.child, GeometryInstanceChild::Geometry(..));
            if ignore_any_hit && !self.closest_hit {
                continue;
            }
            for (material_index, material) in instance.materials.iter().enumerate() {
                let material = material
                    .as_ref()
                    .ok_or(RTresult::RT_ERROR_INVALID_CONTEXT)?;
                let material = material.borrow()?;
                let hit_programs = if self.closest_hit {
                    &material.closest_hit_programs
                } else {
                    &material.any_hit_programs
                };
                visitor(
                    instance_idx,
                    material_index,
                    hit_programs
                        .get(&self.ray)
                        .map(std::ops::Deref::deref)
                        .map(OptixCell::borrow)
                        .transpose()?
                        .as_deref()
                        .map(|prog| ((&*instance, &*material), prog)),
                )?;
            }
        }
        Ok(())
    }

    fn get_program_block_layout(p: &Self::ProgramData) -> Result<Layout, RTresult> {
        p.get_program_block_layout()
    }

    fn copy_program_block(
        &self,
        (geometry_instance, material): (&GeometryInstanceData, &MaterialData),
        p: &Self::ProgramData,
        dst: &mut [u8],
    ) -> Result<(), RTresult> {
        p.copy_program_block(dst, |name| {
            p.get_variable_for_function(geometry_instance, material, self.context, name)
        })
    }
}

pub(crate) struct VisitHitPrograms<'a> {
    pub(crate) closest_hit: bool,
    pub(crate) ray: u32,
    pub(crate) children: &'a [Option<Rc<OptixCell<GeometryInstanceData>>>],
}

impl<'a> VisitLayout for VisitHitPrograms<'a> {
    fn len(&self, _: u32) -> usize {
        self.children.len()
    }

    fn visit_layout(
        &self,
        _: u32,
        mut visitor: impl FnMut(Vec<Option<Layout>>) -> Result<(), RTresult>,
    ) -> Result<(), RTresult> {
        for maybe_instance in self.children.iter() {
            let instance = maybe_instance
                .as_ref()
                .ok_or(RTresult::RT_ERROR_INVALID_CONTEXT)?;
            let instance = instance.borrow()?;
            let ignore_any_hit = matches!(instance.child, GeometryInstanceChild::Geometry(..));
            if ignore_any_hit && !self.closest_hit {
                visitor(vec![])?;
                continue;
            }
            visitor(
                instance
                    .materials
                    .iter()
                    .map(|material| {
                        let material = material
                            .as_ref()
                            .ok_or(RTresult::RT_ERROR_INVALID_CONTEXT)?;
                        let material = material.borrow()?;
                        let hit_programs = if self.closest_hit {
                            &material.closest_hit_programs
                        } else {
                            &material.any_hit_programs
                        };
                        hit_programs
                            .get(&self.ray)
                            .map(|prog| {
                                let prog = prog.borrow()?;
                                prog.get_program_block_layout()
                            })
                            .transpose()
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            )?;
        }
        Ok(())
    }
}

pub(crate) struct CallChainLayout {
    pub(crate) layout: Layout,
    pub(crate) outer_offsets: Vec<u32>,
    pub(crate) inner_offsets: Vec<Vec<u32>>,
}

impl CallChainLayout {
    fn offsets(&self) -> impl Iterator<Item = (u32, &[u32])> {
        iter::once((0u32, &self.outer_offsets[..])).chain(
            self.outer_offsets
                .iter()
                .enumerate()
                .filter_map(move |(index, offset)| {
                    if *offset == u32::MAX {
                        None
                    } else if (offset & 1) == 1 {
                        Some((*offset & !1, &self.inner_offsets[index][..]))
                    } else {
                        None
                    }
                }),
        )
    }
}

trait CallChainCopier {
    fn copy_offsets(offset: u32, offsets: &[u32]);
    fn copy_variable(offset: u32, offsets: &[u32]);
}

fn layout_extend_by_offset_array(layout: Layout, programs_len: usize) -> Result<Layout, RTresult> {
    Ok(layout
        .extend(Layout::array::<u32>(programs_len).map_err(|_| RTresult::RT_ERROR_UNKNOWN)?)
        .map_err(|_| RTresult::RT_ERROR_UNKNOWN)?
        .0)
}

fn copy_prologue_offsets(chain_layout: &CallChainLayout, dst_buffer: &mut [u8]) {
    for (offset, chain_prologue) in chain_layout.offsets() {
        let dst =
            unsafe { slice_cast_mut(&mut dst_buffer[offset as usize..], chain_prologue.len()) };
        dst.copy_from_slice(&*chain_prologue);
    }
}

#[must_use]
pub(crate) struct TrivialHIPAllocator {
    context: hiprtContext,
    hiprt: Rc<HipRt>,
    geometries: Vec<hiprtGeometry>,
    func_tables: Vec<hiprtCustomFuncTable>,
    scenes: Vec<hiprtScene>,
    allocations: Vec<hipDeviceptr_t>,
    drop_flag: bool,
}

impl TrivialHIPAllocator {
    pub(crate) fn new(context: hiprtContext, hiprt: Rc<HipRt>) -> Self {
        Self {
            context,
            hiprt,
            geometries: Vec::new(),
            func_tables: Vec::new(),
            scenes: Vec::new(),
            allocations: Vec::new(),
            drop_flag: false,
        }
    }

    pub(crate) fn new_func_table(&mut self) -> Result<hiprtCustomFuncTable, RTresult> {
        let mut custom_func_table = ptr::null_mut();
        hiprt! { self.hiprt.hiprtCreateCustomFuncTable(self.context, &mut custom_func_table), RT_ERROR_UNKNOWN };
        Ok(custom_func_table)
    }

    pub(crate) fn new_scene(
        &mut self,
        scene_input: hiprtSceneBuildInput,
        build_options: hiprtBuildOptions,
    ) -> Result<hiprtScene, RTresult> {
        let mut temp_mem_size = 0;
        hiprt! { self.hiprt.hiprtGetSceneBuildTemporaryBufferSize(self.context, &scene_input, &build_options, &mut temp_mem_size), RT_ERROR_UNKNOWN };
        let temp_mem = self.allocate(temp_mem_size)?;
        let mut scene = ptr::null_mut();
        hiprt! { self.hiprt.hiprtCreateScene(self.context, &scene_input, &build_options, &mut scene), RT_ERROR_UNKNOWN };
        self.scenes.push(scene);
        hiprt! { self.hiprt.hiprtBuildScene(self.context, hiprtBuildOperation::hiprtBuildOperationBuild, &scene_input, &build_options, temp_mem.0, ptr::null_mut(), scene), RT_ERROR_UNKNOWN };
        Ok(scene)
    }

    pub(crate) fn new_geometry(
        &mut self,
        geometry_input: hiprtGeometryBuildInput,
        build_options: hiprtBuildOptions,
    ) -> Result<hiprtGeometry, RTresult> {
        let mut temp_mem_size = 0;
        hiprt! { self.hiprt.hiprtGetGeometryBuildTemporaryBufferSize(self.context, &geometry_input, &build_options, &mut temp_mem_size), RT_ERROR_UNKNOWN };
        let temp_mem = self.allocate(temp_mem_size)?;
        let mut geometry = ptr::null_mut();
        hiprt! { self.hiprt.hiprtCreateGeometry(self.context, &geometry_input, &build_options, &mut geometry), RT_ERROR_UNKNOWN };
        self.geometries.push(geometry);
        hiprt! { self.hiprt.hiprtBuildGeometry(self.context, hiprtBuildOperation::hiprtBuildOperationBuild, &geometry_input, &build_options, temp_mem.0, ptr::null_mut(), geometry), RT_ERROR_UNKNOWN };
        Ok(geometry)
    }

    pub(crate) fn allocate(&mut self, size: usize) -> Result<hipDeviceptr_t, RTresult> {
        let dev_ptr = hip::malloc(size).map_err(|_| RTresult::RT_ERROR_MEMORY_ALLOCATION_FAILED)?;
        self.allocations.push(dev_ptr);
        Ok(dev_ptr)
    }

    pub(crate) fn copy_to_device<T>(&mut self, slice: &[T]) -> Result<hipDeviceptr_t, RTresult> {
        let dev_ptr = hip::copy_to_device(slice)?;
        self.allocations.push(dev_ptr);
        Ok(dev_ptr)
    }

    fn dealloc_impl(&mut self) -> Result<(), RTresult> {
        if self.drop_flag {
            return Ok(());
        }
        self.drop_flag = true;
        let geometries_result = self
            .geometries
            .iter()
            .copied()
            .fold(Ok(()), |result, geometry| {
                let destroy_result =
                    match unsafe { self.hiprt.hiprtDestroyGeometry(self.context, geometry) } {
                        hiprtError::hiprtSuccess => Ok(()),
                        _ => Err(RTresult::RT_ERROR_UNKNOWN),
                    };
                result.and(destroy_result)
            });
        let func_tables_result =
            self.func_tables
                .iter()
                .copied()
                .fold(geometries_result, |result, func_table| {
                    let destroy_result = match unsafe {
                        self.hiprt
                            .hiprtDestroyCustomFuncTable(self.context, func_table)
                    } {
                        hiprtError::hiprtSuccess => Ok(()),
                        _ => Err(RTresult::RT_ERROR_UNKNOWN),
                    };
                    result.and(destroy_result)
                });
        let scenes_result =
            self.scenes
                .iter()
                .copied()
                .fold(func_tables_result, |result, scene| {
                    let destroy_result =
                        match unsafe { self.hiprt.hiprtDestroyScene(self.context, scene) } {
                            hiprtError::hiprtSuccess => Ok(()),
                            _ => Err(RTresult::RT_ERROR_UNKNOWN),
                        };
                    result.and(destroy_result)
                });
        self.allocations
            .iter()
            .copied()
            .fold(scenes_result, |result, dev_ptr| {
                result.and(hip::free(dev_ptr).map_err(|_| RTresult::RT_ERROR_UNKNOWN))
            })
            .map_err(|_| RTresult::RT_ERROR_UNKNOWN)
    }
}

// In case there is an early exit and we don't get to call .dealloc(...)
impl Drop for TrivialHIPAllocator {
    fn drop(&mut self) {
        self.dealloc_impl().ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::ContextData;
    use crate::variable::VariableData;
    use crate::AlignedBuffer;
    use crate::MaybeWeakRefMut;
    use crate::OptixCell;
    use crate::OptixObjectData;
    use crate::TypeTag;
    use hip_common::raytracing::VariablesBlock;
    use optix_types::RTresult;
    use rustc_hash::FxHashMap;
    use std::ffi::CString;
    use std::rc::Rc;
    use std::{alloc::Layout, mem};

    #[test]
    fn get_layout_empty() {
        let chain_layout = get_layout(
            1,
            &TestGroup {
                containers: Vec::new(),
            },
        )
        .unwrap();
        assert_eq!(Layout::new::<()>(), chain_layout.layout);
        assert_eq!(0, chain_layout.inner_offsets.len());
        assert_eq!(0, chain_layout.outer_offsets.len());
    }

    #[test]
    fn get_layout_complex() {
        let context = Rc::new(OptixCell::new(ContextData::new().unwrap()));
        let mut byte = 0x11;
        let containers = vec![
            TestContainer::new(vec![Some(TestProgram::new(vec![], &context, &mut byte))]), // just pointer
            TestContainer::new(vec![Some(TestProgram::new(
                vec![Layout::new::<u8>()],
                &context,
                &mut byte,
            ))]), // single byte
            TestContainer::new(vec![]),                                                    // empty
            TestContainer::new(vec![None]),       // nothing
            TestContainer::new(vec![None, None]), // uncallable
            TestContainer::new(vec![Some(TestProgram::new(
                vec![
                    // three fields
                    Layout::new::<u32>(),
                    Layout::new::<u16>(),
                    Layout::new::<u64>(),
                    Layout::new::<u8>(),
                ],
                &context,
                &mut byte,
            ))]),
            TestContainer::new(vec![
                // four subprograms
                Some(TestProgram::new(
                    vec![
                        Layout::new::<u8>(),
                        Layout::new::<u32>(),
                        Layout::new::<u32>(),
                    ],
                    &context,
                    &mut byte,
                )),
                None,
                Some(TestProgram::new(vec![], &context, &mut byte)),
                None,
            ]),
            TestContainer::new(vec![Some(TestProgram::new(vec![], &context, &mut byte))]),
            TestContainer::new(vec![None]),
        ];
        let chain_layout = get_layout(1, &TestGroup { containers }).unwrap();
        assert_eq!(
            vec![40u32, 48, !0, !0, !0, 64, 97, 144, !0],
            chain_layout.outer_offsets
        );
        assert_eq!(152, chain_layout.layout.size());
        assert_eq!(8, chain_layout.layout.align());
        assert_eq!(
            vec![
                vec![0u32],
                vec![0u32],
                vec![],
                vec![],
                vec![],
                vec![0],
                vec![16, !0, 40, !0],
                vec![0],
                vec![]
            ],
            chain_layout.inner_offsets
        );
    }

    #[test]
    fn copy_to_cpu_complex() {
        let context = Rc::new(OptixCell::new(ContextData::new().unwrap()));
        let chain_layout = CallChainLayout {
            layout: Layout::from_size_align(152, 8).unwrap(),
            inner_offsets: vec![
                vec![0u32],
                vec![0u32],
                vec![],
                vec![],
                vec![],
                vec![0],
                vec![16, !0, 40, !0],
                vec![0],
                vec![],
            ],
            outer_offsets: vec![40u32, 48, !0, !0, !0, 64, 97, 144, !0],
        };
        let mut byte = 0x11;
        let containers = vec![
            TestContainer::new(vec![Some(TestProgram::new(vec![], &context, &mut byte))]), // just pointer
            TestContainer::new(vec![Some(TestProgram::new(
                vec![Layout::new::<u8>()],
                &context,
                &mut byte,
            ))]), // single byte
            TestContainer::new(vec![]),                                                    // empty
            TestContainer::new(vec![None]),       // nothing
            TestContainer::new(vec![None, None]), // uncallable
            TestContainer::new(vec![Some(TestProgram::new(
                vec![
                    // three fields
                    Layout::new::<u32>(),
                    Layout::new::<u16>(),
                    Layout::new::<u64>(),
                    Layout::new::<u8>(),
                ],
                &context,
                &mut byte,
            ))]),
            TestContainer::new(vec![
                // four subprograms
                Some(TestProgram::new(
                    vec![
                        Layout::new::<u8>(),
                        Layout::new::<u32>(),
                        Layout::new::<u32>(),
                    ],
                    &context,
                    &mut byte,
                )),
                None,
                Some(TestProgram::new(vec![], &context, &mut byte)),
                None,
            ]),
            TestContainer::new(vec![Some(TestProgram::new(vec![], &context, &mut byte))]),
            TestContainer::new(vec![None]),
        ];
        let buffer = super::copy_to_cpu(1, &TestGroup { containers }, &chain_layout).unwrap();
        assert_eq!(
            buffer.as_bytes(),
            vec![
                40u8, 0, 0, 0, // prog 0 offset
                48, 0, 0, 0, // prog 1 offset
                255, 255, 255, 255, // prog 2 offset
                255, 255, 255, 255, // prog 3 offset
                255, 255, 255, 255, // prog 4 offset
                64, 0, 0, 0, // prog 5 offset
                97, 0, 0, 0, // multi-prog 6 offset
                144, 0, 0, 0, // prog 7 offset
                255, 255, 255, 255, // prog 8 offset
                0, 0, 0, 0, // offset table padding
                0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, // prog 0, pointer
                0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, // prog 1, pointer
                0x33, // prog 1, field 0
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // prog 1 padding
                0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, // prog 5, pointer
                0x55, 0x55, 0x55, 0x55, // prog 5, field 0
                0x66, 0x66, // prog 5, field 1
                0x00, 0x00, // prog 5, field 1 padding
                0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, 0x77, // prog 5, field 2
                0x88, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // prog 5 padding
                16, 0, 0, 0, // prog 6, subprog 0 offset
                255, 255, 255, 255, // prog 6, subprog 1 offset
                40, 0, 0, 0, // prog 6, subprog 2 offset
                255, 255, 255, 255, // prog 6, subprog 3 offset
                0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, 0x99, // prog 6, subprog 0, pointer
                0xaa, // prog 6, subprog 0, field 0
                0x00, 0x00, 0x00, // prog 6, subprog 0, field 0 padding
                0xbb, 0xbb, 0xbb, 0xbb, // prog 6, subprog 0, field 1
                0xcc, 0xcc, 0xcc, 0xcc, // prog 6, subprog 0, field 2
                0x00, 0x00, 0x00, 0x00, // prog 6, subprog 0 padding
                0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, 0xdd, // prog 6, subprog 2, pointer
                0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, 0xee, // prog 7, pointer
            ]
        );
    }

    struct TestGroup {
        containers: Vec<Rc<OptixCell<TestContainer>>>,
    }

    #[gat]
    impl<'a> VisitCallChain for TestGroup {
        type ProgramData = TestProgram;
        type CopyContext<'temp> = ();

        fn len(&self, _: u32) -> usize {
            self.containers.len()
        }

        fn visit_alloc(
            &self,
            _: u32,
            mut visitor: impl FnMut(
                usize,
                usize,
                Option<((), &Self::ProgramData)>,
            ) -> Result<(), RTresult>,
        ) -> Result<(), RTresult> {
            for (outer_idx, container) in self.containers.iter().enumerate() {
                let container = container.borrow()?;
                for (inner_odx, program) in container.programs.iter().enumerate() {
                    visitor(
                        outer_idx,
                        inner_odx,
                        program
                            .as_deref()
                            .map(OptixCell::borrow)
                            .transpose()?
                            .as_deref()
                            .map(|prog| ((), prog)),
                    )?
                }
            }
            Ok(())
        }

        fn get_program_block_layout(p: &Self::ProgramData) -> Result<Layout, RTresult> {
            Ok(p.variables_block.layout)
        }

        fn copy_program_block(
            &self,
            _: (),
            p: &Self::ProgramData,
            dst: &mut [u8],
        ) -> Result<(), RTresult> {
            dst[..mem::size_of::<usize>()].copy_from_slice(&p.ptr.to_ne_bytes());
            for (name, var) in p.variables_block.variables.iter() {
                let var_data = &p.variables[name];
                let var_data = var_data.borrow()?;
                let variable_offset = var.offset as usize;
                var_data.copy_into_buffer(
                    &mut dst[variable_offset..variable_offset + var.size as usize],
                )?;
            }
            Ok(())
        }
    }

    struct TestContainer {
        programs: Vec<Option<Rc<OptixCell<TestProgram>>>>,
    }

    impl TestContainer {
        fn new(programs: Vec<Option<Rc<OptixCell<TestProgram>>>>) -> Rc<OptixCell<Self>> {
            Rc::new(OptixCell::new(Self { programs }))
        }
    }

    impl OptixObjectData for TestContainer {
        const TYPE: TypeTag = TypeTag::GeometryInstance;

        fn deregister(&mut self, _this: &Rc<OptixCell<Self>>) -> Result<(), RTresult> {
            Ok(())
        }

        fn context<'a>(&'a mut self) -> MaybeWeakRefMut<'a, ContextData> {
            todo!()
        }
    }

    struct TestProgram {
        ptr: usize,
        variables_block: VariablesBlock,
        variables: FxHashMap<CString, Rc<OptixCell<VariableData>>>,
    }

    impl TestProgram {
        fn new(
            vars: Vec<Layout>,
            _context: &Rc<OptixCell<ContextData>>,
            byte: &mut u8,
        ) -> Rc<OptixCell<Self>> {
            let ptr =
                usize::from_ne_bytes([*byte, *byte, *byte, *byte, *byte, *byte, *byte, *byte]);
            *byte += 0x11;
            let mut layout = Layout::new::<usize>();
            let variables = vars
                .iter()
                .enumerate()
                .map(|(idx, l)| {
                    let (new_layout, offset) = layout.extend(*l).unwrap();
                    layout = new_layout;
                    let name = idx.to_string();
                    (
                        CString::new(name).unwrap(),
                        hip_common::raytracing::Variable {
                            offset: offset as u32,
                            size: l.size() as u32,
                            default_value: Vec::new(),
                        },
                    )
                })
                .collect::<FxHashMap<_, _>>();
            let variables_block = VariablesBlock { layout, variables };
            let variables = vars
                .iter()
                .enumerate()
                .map(|(idx, l)| {
                    let (new_layout, _) = layout.extend(*l).unwrap();
                    layout = new_layout;
                    let vec = vec![*byte; l.size()];
                    let mut aligned_vec = AlignedBuffer::new(*l);
                    aligned_vec.as_bytes_mut().copy_from_slice(&*vec);
                    let variable = Rc::new(OptixCell::new(VariableData {
                        value: crate::variable::VariableValue::Boxed(aligned_vec),
                        context: unsafe { mem::zeroed() },
                    }));
                    *byte += 0x11;
                    let name = idx.to_string();
                    (CString::new(name).unwrap(), variable)
                })
                .collect::<FxHashMap<_, _>>();
            Rc::new(OptixCell::new(Self {
                ptr,
                variables_block,
                variables,
            }))
        }
    }

    impl OptixObjectData for TestProgram {
        const TYPE: TypeTag = TypeTag::Program;

        fn deregister(&mut self, _this: &Rc<OptixCell<Self>>) -> Result<(), RTresult> {
            Ok(())
        }

        fn context<'a>(&'a mut self) -> MaybeWeakRefMut<'a, ContextData> {
            panic!()
        }
    }
}
