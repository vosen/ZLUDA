use crate::{
    context::{self, Context, ContextData},
    geometry::{Geometry, GeometryData},
    geometry_triangles::{GeometryTriangles, GeometryTrianglesData},
    hip,
    material::{Material, MaterialData},
    null_check, null_unwrap,
    program::ProgramData,
    repr_gpu::{self, TrivialHIPAllocator},
    variable::{Variable, VariableData},
    MaybeWeakRefMut, OptixCell, OptixObjectData, TypeTag,
};
use hip_runtime_sys::*;
use hiprt_sys::*;
use optix_types::*;
use rustc_hash::FxHashMap;
use std::{
    alloc::{Layout, LayoutError},
    ffi::{c_void, CStr, CString},
    mem, ptr,
    rc::{Rc, Weak},
};

pub(crate) type GeometryInstance = *const OptixCell<GeometryInstanceData>;

pub(crate) struct GeometryInstanceData {
    pub(crate) context: Weak<OptixCell<ContextData>>,
    pub(crate) child: GeometryInstanceChild,
    pub(crate) materials: Vec<Option<Rc<OptixCell<MaterialData>>>>,
    pub(crate) variables: FxHashMap<CString, Rc<OptixCell<VariableData>>>,
}

impl GeometryInstanceData {
    fn new(weak_context: Weak<OptixCell<ContextData>>, _: &mut ContextData) -> Self {
        Self {
            context: weak_context,
            child: GeometryInstanceChild::None,
            materials: Vec::new(),
            variables: FxHashMap::default(),
        }
    }

    fn register(this: Rc<OptixCell<Self>>, context: &mut ContextData) {
        context.geometry_instances.insert(this);
    }

    unsafe fn create(context: Context) -> Result<GeometryInstance, RTresult> {
        context::create_subobject(context, Self::new, Self::register)
    }

    pub(crate) fn prepare_globals(
        &self,
        allocator: &mut TrivialHIPAllocator,
        context: &ContextData,
        build_flags: hiprtBuildFlagBits,
        ray_type_count: u32,
        custom_func_set_counter: u32,
        global_state: &mut repr_gpu::GlobalState,
        transform_block: hipDeviceptr_t,
    ) -> Result<DeviceGeometryInstance, RTresult> {
        match self.child {
            GeometryInstanceChild::None => return Err(RTresult::RT_ERROR_INVALID_CONTEXT),
            GeometryInstanceChild::Geometry(ref geometry) => self.prepare_custom_geometry(
                allocator,
                context,
                build_flags,
                ray_type_count,
                custom_func_set_counter,
                geometry,
                transform_block,
                global_state
            ),
            GeometryInstanceChild::GeometryTriangles(ref triangles) => {
                self.prepare_triangles(allocator, build_flags, triangles)
            }
        }
    }

    fn prepare_custom_geometry(
        &self,
        allocator: &mut TrivialHIPAllocator,
        context: &ContextData,
        build_flags: hiprtBuildFlagBits,
        ray_type_count: u32,
        custom_func_set_counter: u32,
        geometry: &Rc<OptixCell<GeometryData>>,
        transform_block: hipDeviceptr_t,
        global_state: &mut repr_gpu::GlobalState
    ) -> Result<DeviceGeometryInstance, RTresult> {
        let geometry = geometry.borrow()?;
        let program_intersection = geometry
            .program_intersection
            .as_ref()
            .ok_or(RTresult::RT_ERROR_INVALID_CONTEXT)?;
        let program_intersection = program_intersection
            .upgrade()
            .ok_or(RTresult::RT_ERROR_INVALID_CONTEXT)?;
        let program_intersection = program_intersection.borrow()?;
        let program_bounding_box = geometry
            .program_bounding_box
            .as_ref()
            .ok_or(RTresult::RT_ERROR_INVALID_CONTEXT)?;
        let program_bounding_box = program_bounding_box
            .upgrade()
            .ok_or(RTresult::RT_ERROR_INVALID_CONTEXT)?;
        let program_bounding_box = program_bounding_box.borrow()?;
        let mut bb_variable_block = program_bounding_box
            .prepare_variable_block_for_function_non_hit(allocator, self, &geometry, context)?;
        let intersection_input = self.prepare_intersection_input(
            allocator,
            context,
            ray_type_count,
            &*program_intersection,
            &*geometry,
            transform_block,
        )?;
        let intersect_func = program_intersection.get_function()?;
        let mut bounding_box_fn = program_bounding_box.get_function()?;
        let custom_func_set = hiprtCustomFuncSet {
            intersectFunc: unsafe { mem::transmute(intersect_func) },
            intersectFuncData: intersection_input.0,
        };
        let mut bounding_box_primitive_count = geometry.primitive_count;
        let kernel_bounding_box = program_bounding_box
            .shared
            .module
            .get_function(ProgramData::KERNEL_BOUNDING_BOX_NAME)
            .map_err(|_| RTresult::RT_ERROR_UNKNOWN)?;
        let mut bounding_boxes_device = allocator
            .allocate(mem::size_of::<f32>() * 8 * bounding_box_primitive_count as usize)?;
        let mut params = [
            global_state as *mut repr_gpu::GlobalState as *mut c_void, 
            &mut bounding_box_fn as *mut _ as *mut c_void,
            &mut bounding_box_primitive_count as *mut _ as _,
            &mut bounding_boxes_device.0 as *mut _ as _,
            &mut bb_variable_block as *mut _ as _,
        ];
        program_bounding_box
            .shared
            .module
            .launch_kernel_1d(
                kernel_bounding_box,
                bounding_box_primitive_count,
                0,
                ptr::null_mut(),
                params.as_mut_ptr(),
            )
            .map_err(|_| RTresult::RT_ERROR_UNKNOWN)?;
        let mut aabb_list = hiprtAABBListPrimitive {
            aabbs: bounding_boxes_device.0,
            aabbCount: bounding_box_primitive_count,
            aabbStride: (mem::size_of::<f32>() * 8) as u32,
        };
        let geometry_input = hiprtGeometryBuildInput {
            type_: hiprtPrimitiveType::hiprtPrimitiveTypeAABBList,
            __bindgen_anon_1: hiprtGeometryBuildInput__bindgen_ty_1 {
                aabbList: hiprtGeometryBuildInput__bindgen_ty_1__bindgen_ty_2 {
                    primitive: &mut aabb_list,
                    customType: custom_func_set_counter,
                },
            },
            nodes: ptr::null_mut(),
        };
        let build_options = hiprtBuildOptions {
            buildFlags: build_flags.0,
        };
        let hiprt_geometry = allocator.new_geometry(geometry_input, build_options)?;
        Ok(DeviceGeometryInstance::Geometry {
            hiprt_geometry: hipDeviceptr_t(hiprt_geometry),
            custom_func_set,
        })
    }

    fn prepare_intersection_input(
        &self,
        allocator: &mut TrivialHIPAllocator,
        context: &ContextData,
        ray_type_count: u32,
        program: &ProgramData,
        geometry: &GeometryData,
        transform_block: hipDeviceptr_t,
    ) -> Result<hipDeviceptr_t, RTresult> {
        let visitor = repr_gpu::IntersectVisitCallChain {
            context,
            geometry_instance: self,
        };
        let layout_var_block = program.variables_block.layout;
        let call_chain_layout = repr_gpu::get_layout(ray_type_count, &visitor)?;
        let (total_layout, offsets) =
            Self::layout_of_struct([layout_var_block, call_chain_layout.layout].iter().copied())
                .map_err(|_| RTresult::RT_ERROR_UNKNOWN)?;
        let intersection_input = allocator.allocate(total_layout.size())?;
        let prologue = repr_gpu::IntersectionInput {
            transform_block,
            materials_start: offsets[1] as u32,
        };
        let mut staging_var_block = vec![0u8; layout_var_block.size()];
        unsafe { ptr::copy_nonoverlapping(&prologue, staging_var_block.as_mut_ptr() as _, 1) };
        ProgramData::copy_variable_block(
            &program.variables_block,
            &mut staging_var_block,
            |var_name| program.get_variable_for_function_non_hit(self, geometry, context, var_name),
        )?;
        hip! { hipMemcpyHtoD(intersection_input, staging_var_block.as_mut_ptr() as _, layout_var_block.size()), RT_ERROR_UNKNOWN };
        repr_gpu::copy_to_gpu(
            ray_type_count,
            &visitor,
            &call_chain_layout,
            hipDeviceptr_t(unsafe { intersection_input.0.cast::<u8>().add(offsets[1]).cast() }),
        )?;
        Ok(intersection_input)
    }

    fn layout_of_struct(
        mut fields: impl Iterator<Item = Layout>,
    ) -> Result<(Layout, Vec<usize>), LayoutError> {
        fields.try_fold(
            (Layout::new::<()>(), Vec::new()),
            |(total_layout, mut offsets), layout| {
                let (new_layout, offset) = total_layout.extend(layout)?;
                offsets.push(offset);
                Ok((new_layout, offsets))
            },
        )
    }

    fn prepare_triangles(
        &self,
        allocator: &mut TrivialHIPAllocator,
        build_flags: hiprtBuildFlagBits,
        triangles: &Rc<OptixCell<GeometryTrianglesData>>,
    ) -> Result<DeviceGeometryInstance, RTresult> {
        let triangles = triangles.borrow()?;
        let mut primitive = triangles.to_hiprt()?;
        let geometry_input = hiprtGeometryBuildInput {
            type_: hiprtPrimitiveType::hiprtPrimitiveTypeTriangleMesh,
            __bindgen_anon_1: hiprtGeometryBuildInput__bindgen_ty_1 {
                triangleMesh: hiprtGeometryBuildInput__bindgen_ty_1__bindgen_ty_1 {
                    primitive: &mut primitive,
                },
            },
            nodes: ptr::null_mut(),
        };
        let build_options = hiprtBuildOptions {
            buildFlags: build_flags.0,
        };
        let hiprt_geometry = allocator.new_geometry(geometry_input, build_options)?;
        Ok(DeviceGeometryInstance::GeometryTriangles {
            hiprt_geometry: hipDeviceptr_t(hiprt_geometry),
        })
    }
}

impl OptixObjectData for GeometryInstanceData {
    const TYPE: TypeTag = TypeTag::GeometryInstance;

    fn deregister(&mut self, this: &Rc<OptixCell<Self>>) -> Result<(), RTresult> {
        if let Some(context) = self.context.upgrade() {
            let mut context = (*context).borrow_mut()?;
            context.geometry_instances.remove(this);
        }
        Ok(())
    }

    fn context<'a>(&'a mut self) -> crate::MaybeWeakRefMut<'a, ContextData> {
        MaybeWeakRefMut::Weak(&self.context)
    }
}

pub(crate) enum DeviceGeometryInstance {
    Geometry {
        hiprt_geometry: hipDeviceptr_t,
        custom_func_set: hiprtCustomFuncSet,
    },
    GeometryTriangles {
        hiprt_geometry: hipDeviceptr_t,
    },
}

pub(crate) enum GeometryInstanceChild {
    None,
    Geometry(Rc<OptixCell<GeometryData>>),
    GeometryTriangles(Rc<OptixCell<GeometryTrianglesData>>),
}

#[repr(C)]
#[derive(Clone)]
pub(crate) struct DeviceProgram {
    pub(crate) function: hipDeviceptr_t,
    pub(crate) variable_block: hipDeviceptr_t,
}

pub(crate) unsafe fn create(
    context: Context,
    material: *mut GeometryInstance,
) -> Result<(), RTresult> {
    null_check(context)?;
    null_check(material)?;
    *material = GeometryInstanceData::create(context)?;
    Ok(())
}

pub(crate) unsafe fn set_geometry_triangles(
    instance: GeometryInstance,
    triangles: GeometryTriangles,
) -> Result<(), RTresult> {
    null_check(triangles)?;
    let instance = null_unwrap(instance)?;
    let mut instance = instance.borrow_mut()?;
    let triangles = OptixCell::clone_rc(triangles);
    instance.child = GeometryInstanceChild::GeometryTriangles(triangles);
    Ok(())
}

pub(crate) unsafe fn set_geometry(
    instance: GeometryInstance,
    geometry: Geometry,
) -> Result<(), RTresult> {
    null_check(geometry)?;
    let instance = null_unwrap(instance)?;
    let mut instance = instance.borrow_mut()?;
    let triangles = OptixCell::clone_rc(geometry);
    instance.child = GeometryInstanceChild::Geometry(triangles);
    Ok(())
}

pub(crate) unsafe fn set_material(
    instance: GeometryInstance,
    index: u32,
    material: Material,
) -> Result<(), RTresult> {
    null_check(material)?;
    let instance = null_unwrap(instance)?;
    let mut instance = instance.borrow_mut()?;
    match instance.materials.get_mut(index as usize) {
        Some(material_slot) => {
            *material_slot = Some(OptixCell::clone_rc(material));
            Ok(())
        }
        None => Err(RTresult::RT_ERROR_INVALID_VALUE),
    }
}

pub(crate) unsafe fn set_material_count(
    instance: GeometryInstance,
    count: u32,
) -> Result<(), RTresult> {
    let instance = null_unwrap(instance)?;
    let mut instance = instance.borrow_mut()?;
    instance.materials.resize(count as usize, None);
    Ok(())
}

pub(crate) unsafe fn declare_variable(
    instance_ptr: GeometryInstance,
    name: *const i8,
    v: *mut Variable,
) -> Result<(), RTresult> {
    null_check(v)?;
    let instance = null_unwrap(instance_ptr)?;
    let mut instance = instance.borrow_mut()?;
    let variable = VariableData::new(&mut *instance)?;
    let name = CStr::from_ptr(name).to_owned();
    let result = Rc::as_ptr(&variable);
    instance.variables.insert(name, variable);
    *v = result;
    Ok(())
}

pub(crate) unsafe fn get_material_count(
    geometryinstance: GeometryInstance,
    count: *mut u32,
) -> Result<(), RTresult> {
    null_check(count)?;
    let instance = null_unwrap(geometryinstance)?;
    let instance = instance.borrow()?;
    *count = instance.materials.len() as u32;
    Ok(())
}

pub(crate) unsafe fn query_variable(
    geometryinstance: GeometryInstance,
    name: *const i8,
    v: *mut Variable,
) -> Result<(), RTresult> {
    null_check(name)?;
    null_check(v)?;
    let geometryinstance = null_unwrap(geometryinstance)?;
    let geometryinstance = (geometryinstance).borrow()?;
    *v = geometryinstance
        .variables
        .get(CStr::from_ptr(name))
        .map(|variable| Rc::as_ptr(variable))
        .unwrap_or(ptr::null_mut());
    Ok(())
}

pub(crate) fn destroy(_geometryinstance: GeometryInstance) -> Result<(), RTresult> {
    // TODO: implement
    Ok(())
}

pub(crate) unsafe fn get_context(
    geometryinstance: *const OptixCell<GeometryInstanceData>,
    context: *mut *const OptixCell<ContextData>,
) -> Result<(), RTresult> {
    let geometryinstance = null_unwrap(geometryinstance)?;
    let geometryinstance = geometryinstance.borrow()?;
    *context = geometryinstance.context.as_ptr();
    Ok(())
}
