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

pub(crate) type Material = *const OptixCell<MaterialData>;

pub(crate) struct MaterialData {
    pub(crate) context: Weak<OptixCell<ContextData>>,
    pub(crate) variables: FxHashMap<CString, Rc<OptixCell<VariableData>>>,
    pub(crate) any_hit_programs: FxHashMap<u32, Rc<OptixCell<ProgramData>>>,
    pub(crate) closest_hit_programs: FxHashMap<u32, Rc<OptixCell<ProgramData>>>,
}

impl MaterialData {
    fn new(weak_context: Weak<OptixCell<ContextData>>, _: &mut ContextData) -> Self {
        Self {
            context: weak_context,
            variables: FxHashMap::default(),
            any_hit_programs: FxHashMap::default(),
            closest_hit_programs: FxHashMap::default(),
        }
    }

    fn register(this: Rc<OptixCell<Self>>, context: &mut ContextData) {
        context.materials.insert(this);
    }

    unsafe fn create(context: Context) -> Result<Material, RTresult> {
        context::create_subobject(context, Self::new, Self::register)
    }
}

impl OptixObjectData for MaterialData {
    const TYPE: TypeTag = TypeTag::Material;

    fn deregister(&mut self, this: &std::rc::Rc<OptixCell<Self>>) -> Result<(), RTresult> {
        if let Some(context) = self.context.upgrade() {
            let mut context = (*context).borrow_mut()?;
            context.materials.remove(this);
        }
        Ok(())
    }

    fn context<'a>(&'a mut self) -> MaybeWeakRefMut<'a, ContextData> {
        MaybeWeakRefMut::Weak(&self.context)
    }
}

pub(crate) unsafe fn create(context: Context, material: *mut Material) -> Result<(), RTresult> {
    null_check(context)?;
    null_check(material)?;
    *material = MaterialData::create(context)?;
    Ok(())
}

pub(crate) unsafe fn declare_variable(
    material_ptr: Material,
    name: *const i8,
    v: *mut Variable,
) -> Result<(), RTresult> {
    null_check(v)?;
    let material = null_unwrap(material_ptr)?;
    let mut material = material.borrow_mut()?;
    let variable = VariableData::new(&mut *material)?;
    let name = CStr::from_ptr(name).to_owned();
    let result = Rc::as_ptr(&variable);
    material.variables.insert(name, variable);
    *v = result;
    Ok(())
}

pub(crate) unsafe fn query_variable(
    material_ptr: Material,
    name: *const i8,
    v: *mut Variable,
) -> Result<(), RTresult> {
    null_check(name)?;
    null_check(v)?;
    let material = null_unwrap(material_ptr)?;
    let material = material.borrow()?;
    *v = material
        .variables
        .get(CStr::from_ptr(name))
        .map(|variable| Rc::as_ptr(variable))
        .unwrap_or(ptr::null_mut());
    Ok(())
}

pub(crate) unsafe fn set_any_hit_program(
    material: Material,
    ray_type_index: u32,
    program: Program,
) -> Result<(), RTresult> {
    null_check(program)?;
    let material = null_unwrap(material)?;
    let mut material = material.borrow_mut()?;
    material
        .any_hit_programs
        .insert(ray_type_index, OptixCell::clone_rc(program));
    Ok(())
}

pub(crate) unsafe fn set_closest_hit_program(
    material: Material,
    ray_type_index: u32,
    program: Program,
) -> Result<(), RTresult> {
    null_check(program)?;
    let material = null_unwrap(material)?;
    let mut material = material.borrow_mut()?;
    material
        .closest_hit_programs
        .insert(ray_type_index, OptixCell::clone_rc(program));
    Ok(())
}

pub(crate) unsafe fn get_context(
    material: Material,
    context: *mut Context,
) -> Result<(), RTresult> {
    null_check(context)?;
    let material = null_unwrap(material)?;
    let material = material.borrow()?;
    *context = Weak::as_ptr(&material.context);
    Ok(())
}

pub(crate) fn destroy(_material: Material) -> Result<(), RTresult> {
    // TODO: implement
    Ok(())
}
