use crate::{
    context::{self, Context, ContextData},
    null_check, null_unwrap,
    program::{Program, ProgramData},
    variable::{Variable, VariableData},
    MaybeWeakRefMut, OptixCell, OptixObjectData, TypeTag,
};
use optix_types::*;
use rustc_hash::FxHashMap;
use std::{
    ffi::{CStr, CString},
    ptr,
    rc::{Rc, Weak},
};

pub(crate) type Geometry = *const OptixCell<GeometryData>;

pub(crate) struct GeometryData {
    pub(crate) context: Weak<OptixCell<ContextData>>,
    pub(crate) program_intersection: Option<Weak<OptixCell<ProgramData>>>,
    pub(crate) program_bounding_box: Option<Weak<OptixCell<ProgramData>>>,
    pub(crate) variables: FxHashMap<CString, Rc<OptixCell<VariableData>>>,
    pub(crate) primitive_count: u32,
}

impl GeometryData {
    fn new(_context: Weak<OptixCell<ContextData>>, _: &mut ContextData) -> Self {
        Self {
            context: _context,
            program_intersection: None,
            program_bounding_box: None,
            variables: FxHashMap::default(),
            primitive_count: 0,
        }
    }

    fn register(this: Rc<OptixCell<Self>>, context: &mut ContextData) {
        context.geometry.insert(this);
    }

    unsafe fn create(context: Context) -> Result<Geometry, RTresult> {
        context::create_subobject(context, Self::new, Self::register)
    }
}

impl OptixObjectData for GeometryData {
    const TYPE: TypeTag = TypeTag::Geometry;

    fn deregister(&mut self, this: &Rc<OptixCell<Self>>) -> Result<(), RTresult> {
        if let Some(context) = self.context.upgrade() {
            let mut context = (*context).borrow_mut()?;
            context.geometry.remove(this);
        }
        Ok(())
    }

    fn context<'a>(&'a mut self) -> crate::MaybeWeakRefMut<'a, ContextData> {
        MaybeWeakRefMut::Weak(&self.context)
    }
}

pub(crate) unsafe fn create(context: Context, geometry: *mut Geometry) -> Result<(), RTresult> {
    null_check(context)?;
    null_check(geometry)?;
    *geometry = GeometryData::create(context)?;
    Ok(())
}

pub(crate) unsafe fn set_primitive_count(
    geometry: Geometry,
    primitive_count: u32,
) -> Result<(), RTresult> {
    let geometry = null_unwrap(geometry)?;
    let mut geometry = geometry.borrow_mut()?;
    geometry.primitive_count = primitive_count;
    Ok(())
}

pub(crate) unsafe fn set_bounding_box_program(
    geometry: Geometry,
    program: Program,
) -> Result<(), RTresult> {
    null_check(program)?;
    let geometry = null_unwrap(geometry)?;
    let mut geometry = geometry.borrow_mut()?;
    geometry.program_bounding_box = Some(OptixCell::clone_weak(program));
    Ok(())
}

pub(crate) unsafe fn set_intersection_program(
    geometry: Geometry,
    program: Program,
) -> Result<(), RTresult> {
    null_check(program)?;
    let geometry = null_unwrap(geometry)?;
    let mut geometry = geometry.borrow_mut()?;
    geometry.program_intersection = Some(OptixCell::clone_weak(program));
    Ok(())
}

pub(crate) unsafe fn declare_variable(
    geometry: Geometry,
    name: *const i8,
    v: *mut Variable,
) -> Result<(), RTresult> {
    null_check(v)?;
    let instance = null_unwrap(geometry)?;
    let mut material = instance.borrow_mut()?;
    let variable = VariableData::new(&mut *material)?;
    let name = CStr::from_ptr(name).to_owned();
    let result = Rc::as_ptr(&variable);
    material.variables.insert(name, variable);
    *v = result;
    Ok(())
}

pub(crate) unsafe fn query_variable(
    geometry: Geometry,
    name: *const i8,
    v: *mut Variable,
) -> Result<(), RTresult> {
    null_check(name)?;
    null_check(v)?;
    let geometry = null_unwrap(geometry)?;
    let geometry = (geometry).borrow()?;
    *v = geometry
        .variables
        .get(CStr::from_ptr(name))
        .map(|variable| Rc::as_ptr(variable))
        .unwrap_or(ptr::null_mut());
    Ok(())
}

pub(crate) fn destroy(_geometry: Geometry) -> Result<(), RTresult> {
    // TODO: implement
    Ok(())
}

pub(crate) unsafe fn get_context(
    geometry: *const OptixCell<GeometryData>,
    context: *mut *const OptixCell<ContextData>,
) -> Result<(), RTresult> {
    let geometry = null_unwrap(geometry)?;
    let geometry = geometry.borrow()?;
    *context = geometry.context.as_ptr();
    Ok(())
}
