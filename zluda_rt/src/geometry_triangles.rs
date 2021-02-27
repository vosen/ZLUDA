use crate::{
    buffer::{Buffer, BufferData},
    context::{self, Context, ContextData},
    null_check, null_unwrap,
    program::{Program, ProgramData},
    variable::{Variable, VariableData},
    MaybeWeakRefMut, OptixCell, OptixObjectData, TypeTag,
};
use hiprt_sys::hiprtTriangleMeshPrimitive;
use optix_types::*;
use rustc_hash::FxHashMap;
use std::{
    ffi::{c_void, CStr, CString},
    ptr,
    rc::{Rc, Weak},
};

pub(crate) type GeometryTriangles = *const OptixCell<GeometryTrianglesData>;

pub(crate) struct GeometryTrianglesData {
    pub(crate) context: Weak<OptixCell<ContextData>>,
    pub(crate) attribute_program: Option<Weak<OptixCell<ProgramData>>>,
    pub(crate) variables: FxHashMap<CString, Rc<OptixCell<VariableData>>>,
    triangle_count: u32,
    indices: Option<Indices>,
    vertices: Option<Vertices>,
    flags: FxHashMap<u32, RTgeometryflags>,
}

impl GeometryTrianglesData {
    fn new(context: Weak<OptixCell<ContextData>>, _: &mut ContextData) -> Self {
        Self {
            context,
            attribute_program: None,
            variables: FxHashMap::default(),
            triangle_count: 0,
            indices: None,
            vertices: None,
            flags: FxHashMap::default(),
        }
    }

    fn register(this: Rc<OptixCell<Self>>, context: &mut ContextData) {
        context.geometry_triangles.insert(this);
    }

    unsafe fn create(context: Context) -> Result<GeometryTriangles, RTresult> {
        context::create_subobject(context, Self::new, Self::register)
    }

    pub(crate) fn to_hiprt(&self) -> Result<hiprtTriangleMeshPrimitive, RTresult> {
        let vertices = self.vertices.as_ref().ok_or(RTresult::RT_ERROR_UNKNOWN)?;
        let vertex_buffer = vertices
            .vertex_buffer
            .upgrade()
            .ok_or(RTresult::RT_ERROR_UNKNOWN)?;
        let vertex_buffer = vertex_buffer.borrow()?;
        let mut result = hiprtTriangleMeshPrimitive {
            vertices: unsafe {
                (vertex_buffer.pointer_mip0().0 as *mut u8)
                    .add(vertices.vertex_buffer_byte_offset as usize)
            } as *mut c_void,
            vertexCount: vertices.vertex_count,
            vertexStride: vertices.vertex_byte_stride as u32,
            triangleIndices: ptr::null_mut(),
            triangleCount: 0,
            triangleStride: 0,
        };
        if let Some(ref vertex_indices) = self.indices {
            let index_buffer = vertex_indices
                .index_buffer
                .upgrade()
                .ok_or(RTresult::RT_ERROR_UNKNOWN)?;
            let index_buffer = index_buffer.borrow()?;
            result.triangleIndices = unsafe {
                (index_buffer.pointer_mip0().0 as *mut u8)
                    .add(vertex_indices.index_buffer_byte_offset as usize)
            } as *mut c_void;
            result.triangleCount = self.triangle_count;
            result.triangleStride = vertex_indices.tri_indices_byte_stride as u32;
        } else {
            // TODO: implement
            return Err(RTresult::RT_ERROR_UNKNOWN);
        }
        Ok(result)
    }
}

impl OptixObjectData for GeometryTrianglesData {
    const TYPE: TypeTag = TypeTag::GeometryTriangles;

    fn deregister(&mut self, this: &Rc<OptixCell<Self>>) -> Result<(), RTresult> {
        if let Some(context) = self.context.upgrade() {
            let mut context = (*context).borrow_mut()?;
            context.geometry_triangles.remove(this);
        }
        Ok(())
    }

    fn context<'a>(&'a mut self) -> crate::MaybeWeakRefMut<'a, ContextData> {
        MaybeWeakRefMut::Weak(&self.context)
    }
}

struct Indices {
    index_buffer: Weak<OptixCell<BufferData>>,
    index_buffer_byte_offset: u64,
    tri_indices_byte_stride: u64,
}

struct Vertices {
    vertex_count: ::std::os::raw::c_uint,
    vertex_buffer: Weak<OptixCell<BufferData>>,
    vertex_buffer_byte_offset: u64,
    vertex_byte_stride: u64,
}

pub(crate) unsafe fn create(
    context: Context,
    triangles: *mut GeometryTriangles,
) -> Result<(), RTresult> {
    null_check(context)?;
    null_check(triangles)?;
    *triangles = GeometryTrianglesData::create(context)?;
    Ok(())
}

pub(crate) unsafe fn set_primitive_count(
    triangles: GeometryTriangles,
    triangle_count: u32,
) -> Result<(), RTresult> {
    let triangles = null_unwrap(triangles)?;
    let mut triangles = triangles.borrow_mut()?;
    triangles.triangle_count = triangle_count;
    Ok(())
}

pub(crate) unsafe fn set_triangle_indices(
    triangles: GeometryTriangles,
    index_buffer: Buffer,
    index_buffer_byte_offset: u64,
    tri_indices_byte_stride: u64,
    tri_indices_format: RTformat,
) -> Result<(), RTresult> {
    if tri_indices_format != RTformat::RT_FORMAT_UNSIGNED_INT3 {
        return Err(RTresult::RT_ERROR_NOT_SUPPORTED);
    }
    let triangles = null_unwrap(triangles)?;
    let mut triangles = triangles.borrow_mut()?;
    triangles.indices = Some(Indices {
        index_buffer: OptixCell::clone_weak(index_buffer),
        index_buffer_byte_offset,
        tri_indices_byte_stride,
    });
    Ok(())
}

pub(crate) unsafe fn set_vertices(
    triangles: GeometryTriangles,
    vertex_count: ::std::os::raw::c_uint,
    vertex_buffer: Buffer,
    vertex_buffer_byte_offset: u64,
    vertex_byte_stride: u64,
    position_format: RTformat,
) -> Result<(), RTresult> {
    if position_format != RTformat::RT_FORMAT_FLOAT3 {
        return Err(RTresult::RT_ERROR_NOT_SUPPORTED);
    }
    let triangles = null_unwrap(triangles)?;
    let mut triangles = triangles.borrow_mut()?;
    triangles.vertices = Some(Vertices {
        vertex_count,
        vertex_buffer: OptixCell::clone_weak(vertex_buffer),
        vertex_buffer_byte_offset,
        vertex_byte_stride,
    });
    Ok(())
}

pub(crate) fn validate(_geometrytriangles: GeometryTriangles) -> Result<(), RTresult> {
    // TODO: implement
    Ok(())
}

pub(crate) unsafe fn set_attribute(
    triangles: GeometryTriangles,
    program: Program,
) -> Result<(), RTresult> {
    let triangles = null_unwrap(triangles)?;
    let mut triangles = triangles.borrow_mut()?;
    triangles.attribute_program = Some(OptixCell::clone_weak(program));
    Ok(())
}

pub(crate) unsafe fn set_flags_per_material(
    triangles: GeometryTriangles,
    material_index: u32,
    flags: RTgeometryflags,
) -> Result<(), RTresult> {
    let triangles = null_unwrap(triangles)?;
    let mut triangles = triangles.borrow_mut()?;
    let entry = triangles
        .flags
        .entry(material_index)
        .or_insert(RTgeometryflags(0));
    entry.0 |= flags.0;
    Ok(())
}

pub(crate) unsafe fn get_context(
    geometrytriangles: GeometryTriangles,
    context_ptr: *mut Context,
) -> Result<(), RTresult> {
    let geometrytriangles = null_unwrap(geometrytriangles)?;
    null_check(context_ptr)?;
    let geometrytriangles = geometrytriangles.borrow()?;
    let context = Weak::as_ptr(&geometrytriangles.context);
    *context_ptr = context;
    Ok(())
}

pub(crate) unsafe fn declare_variable(
    geometrytriangles: GeometryTriangles,
    name: *const i8,
    v: *mut Variable,
) -> Result<(), RTresult> {
    null_check(v)?;
    let geometrytriangles = null_unwrap(geometrytriangles)?;
    let mut geometrytriangles = geometrytriangles.borrow_mut()?;
    let variable = VariableData::new(&mut *geometrytriangles)?;
    let name = CStr::from_ptr(name).to_owned();
    let result = Rc::as_ptr(&variable);
    geometrytriangles.variables.insert(name, variable);
    *v = result;
    Ok(())
}

pub(crate) unsafe fn query_variable(
    geometrytriangles: GeometryTriangles,
    name: *const i8,
    v: *mut Variable,
) -> Result<(), RTresult> {
    null_check(name)?;
    null_check(v)?;
    let geometrytriangles = null_unwrap(geometrytriangles)?;
    let geometrytriangles = (geometrytriangles).borrow()?;
    *v = geometrytriangles
        .variables
        .get(CStr::from_ptr(name))
        .map(|variable| Rc::as_ptr(variable))
        .unwrap_or(ptr::null_mut());
    Ok(())
}

pub(crate) unsafe fn set_build_flags(
    _geometrytriangles: GeometryTriangles,
    _build_flags: RTgeometrybuildflags,
) -> Result<(), RTresult> {
    // TODO: implement
    if _build_flags != RTgeometrybuildflags::RT_GEOMETRY_BUILD_FLAG_NONE {
        return Err(RTresult::RT_ERROR_NOT_SUPPORTED);
    }
    Ok(())
}

pub(crate) unsafe fn destroy(_geometrytriangles: GeometryTriangles) -> Result<(), RTresult> {
    // TODO: implement
    Ok(())
}
