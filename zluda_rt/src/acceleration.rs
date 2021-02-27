use crate::{
    context::{self, Context, ContextData},
    geometry_group::GeometryGroupData,
    group::GroupData,
    null_check, null_unwrap, MaybeWeakRefMut, OptixCell, OptixObjectData, TypeTag,
};
use hiprt_sys::hiprtBuildFlagBits;
use optix_types::*;
use std::ffi::CStr;
use std::{
    ffi::CString,
    rc::{Rc, Weak},
};

pub(crate) type Acceleration = *const OptixCell<AccelerationData>;

pub(crate) struct AccelerationData {
    pub(crate) context: Weak<OptixCell<ContextData>>,
    pub(crate) owner: Option<AccelerationOwner>,
    builder: CString,
}

impl AccelerationData {
    fn new(weak_context: Weak<OptixCell<ContextData>>, _: &mut ContextData) -> Self {
        Self {
            context: weak_context,
            owner: None,
            builder: CString::new("").unwrap(),
        }
    }

    fn register(this: Rc<OptixCell<Self>>, context: &mut ContextData) {
        context.accelerations.insert(this);
    }

    unsafe fn create(context: Context) -> Result<Acceleration, RTresult> {
        context::create_subobject(context, Self::new, Self::register)
    }

    pub(crate) fn to_hiprt(&self) -> Option<hiprtBuildFlagBits> {
        Some(match self.builder.as_bytes() {
            b"NoAccel" => hiprtBuildFlagBits::hiprtBuildFlagBitPreferFastBuild,
            b"Bvh" => hiprtBuildFlagBits::hiprtBuildFlagBitPreferBalancedBuild,
            // As of version 1.2.0 high quality gives crashes
            b"Sbvh" | b"Trbvh" => hiprtBuildFlagBits::hiprtBuildFlagBitPreferBalancedBuild,
            _ => return None,
        })
    }
}

impl OptixObjectData for AccelerationData {
    const TYPE: TypeTag = TypeTag::Acceleration;

    fn deregister(&mut self, this: &Rc<OptixCell<Self>>) -> Result<(), RTresult> {
        if let Some(context) = self.context.upgrade() {
            let mut context = (*context).borrow_mut()?;
            context.accelerations.remove(this);
        }
        Ok(())
    }

    fn context<'a>(&'a mut self) -> crate::MaybeWeakRefMut<'a, ContextData> {
        MaybeWeakRefMut::Weak(&self.context)
    }
}

pub(crate) enum AccelerationOwner {
    Group(Weak<OptixCell<GroupData>>),
    GeometryGroup(Weak<OptixCell<GeometryGroupData>>),
}

pub(crate) unsafe fn create(
    context: Context,
    acceleration: *mut Acceleration,
) -> Result<(), RTresult> {
    null_check(context)?;
    null_check(acceleration)?;
    *acceleration = AccelerationData::create(context)?;
    Ok(())
}

pub(crate) unsafe fn set_builder(
    acceleration: *const OptixCell<AccelerationData>,
    builder: *const i8,
) -> Result<(), RTresult> {
    null_check(builder)?;
    let acceleration = null_unwrap(acceleration)?;
    let mut acceleration = acceleration.borrow_mut()?;
    acceleration.builder = CStr::from_ptr(builder).to_owned();
    Ok(())
}

pub(crate) unsafe fn mark_dirty(acceleration: Acceleration) -> Result<(), RTresult> {
    let acceleration = null_unwrap(acceleration)?;
    let acceleration = acceleration.borrow()?;
    let context = acceleration
        .context
        .upgrade()
        .ok_or(RTresult::RT_ERROR_INVALID_CONTEXT)?;
    let mut context = context.borrow_mut()?;
    context.invalidate();
    Ok(())
}

pub(crate) fn destroy(_acceleration: Acceleration) -> Result<(), RTresult> {
    // TODO: implement
    Ok(())
}

pub(crate) unsafe fn get_context(
    acceleration: *const OptixCell<AccelerationData>,
    context: *mut *const OptixCell<ContextData>,
) -> Result<(), RTresult> {
    let acceleration = null_unwrap(acceleration)?;
    let acceleration = acceleration.borrow()?;
    *context = acceleration.context.as_ptr();
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::optix_test;
    use crate::test_common::OptixFns;
    use std::{ffi::CStr, ptr};

    optix_test!(default_acceleration);

    unsafe fn default_acceleration<Optix: OptixFns>(o: Optix) {
        let mut ctx = ptr::null_mut();
        o.rtContextCreate(&mut ctx);
        let mut accel = ptr::null_mut();
        o.rtAccelerationCreate(ctx, &mut accel);
        let mut builder = ptr::null();
        o.rtAccelerationGetBuilder(accel, &mut builder);
        let builder_name = CStr::from_ptr(builder);
        assert_eq!(builder_name.to_str().unwrap(), "");
        o.rtContextDestroy(ctx);
    }
}
