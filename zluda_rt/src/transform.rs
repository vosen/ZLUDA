use crate::{
    context::{self, Context, ContextData},
    geometry_group::GeometryGroupData,
    null_check, null_unwrap,
    repr_gpu::{self, TrivialHIPAllocator},
    MaybeWeakRefMut, OptixCell, OptixObjectData, TypeTag, TypedObjectWeak, UntypedObject,
};

use glam::{Quat, Vec3};
use hip_runtime_sys::hipDeviceptr_t;
use hiprt_sys::{hiprtFloat3, hiprtFloat4, hiprtFrame};
use optix_types::*;
use std::{
    ptr,
    rc::{Rc, Weak},
};

pub(crate) type Transform = *const OptixCell<TransformData>;

pub(crate) struct TransformData {
    pub(crate) context: Weak<OptixCell<ContextData>>,
    pub(crate) transform: [f32; 16],
    pub(crate) inverse_transform: [f32; 16],
    pub(crate) scale: Vec3,
    pub(crate) rotation: Quat,
    pub(crate) translation: Vec3,
    pub(crate) child: Option<Weak<OptixCell<GeometryGroupData>>>,
}

impl TransformData {
    fn new(weak_context: Weak<OptixCell<ContextData>>, _: &mut ContextData) -> Self {
        let scale = Vec3::ONE;
        let rotation = Quat::IDENTITY;
        let translation = Vec3::ZERO;
        let matrix = glam::Mat4::from_scale_rotation_translation(scale, rotation, translation);
        let transform = matrix.transpose().to_cols_array();
        let inverse_transform = transform.clone();
        Self {
            context: weak_context,
            child: None,
            scale,
            rotation,
            translation,
            transform,
            inverse_transform,
        }
    }

    fn register(this: Rc<OptixCell<Self>>, context: &mut ContextData) {
        context.transforms.insert(this);
    }

    unsafe fn create(context: Context) -> Result<Transform, RTresult> {
        context::create_subobject(context, Self::new, Self::register)
    }

    pub(crate) fn allocate(
        &self,
        allocator: &mut TrivialHIPAllocator,
    ) -> Result<hipDeviceptr_t, RTresult> {
        let host_side = [repr_gpu::OptixTransform {
            transform: self.transform,
            inverse_transform: self.inverse_transform,
        }];
        allocator
            .copy_to_device(&host_side)
            .map_err(|_| RTresult::RT_ERROR_UNKNOWN)
    }

    pub(crate) fn to_hiprt(&self) -> hiprtFrame {
        let rotation = Self::quat_to_hiprt(self.rotation);
        let scale = Self::vec3_to_hiprt(self.scale);
        let translation = Self::vec3_to_hiprt(self.translation);
        hiprtFrame {
            rotation,
            scale,
            translation,
            time: 0.0,
            pad: 0,
        }
    }

    fn vec3_to_hiprt(v: Vec3) -> hiprtFloat3 {
        hiprtFloat3 {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }

    fn quat_to_hiprt(q: Quat) -> hiprtFloat4 {
        let (axis, angle) = q.to_axis_angle();
        hiprtFloat4 {
            x: axis.x,
            y: axis.y,
            z: axis.z,
            w: angle,
        }
    }
}

impl OptixObjectData for TransformData {
    const TYPE: TypeTag = TypeTag::Transform;

    fn deregister(&mut self, this: &Rc<OptixCell<Self>>) -> Result<(), RTresult> {
        if let Some(context) = self.context.upgrade() {
            let mut context = (*context).borrow_mut()?;
            context.transforms.remove(this);
        }
        Ok(())
    }

    fn context<'a>(&'a mut self) -> crate::MaybeWeakRefMut<'a, ContextData> {
        MaybeWeakRefMut::Weak(&self.context)
    }
}

pub(crate) unsafe fn create(context: Context, transform: *mut Transform) -> Result<(), RTresult> {
    null_check(context)?;
    null_check(transform)?;
    *transform = TransformData::create(context)?;
    Ok(())
}

pub(crate) unsafe fn set_matrix(
    transform: Transform,
    transpose: i32,
    matrix: *const f32,
    inverse_matrix: *const f32,
) -> Result<(), RTresult> {
    null_check(matrix)?;
    let transform = null_unwrap(transform)?;
    let mut transform = transform.borrow_mut()?;
    let mut matrix4 = glam::Mat4::from_cols_array(&*(matrix as *const [f32; 16]));
    if transpose == 0 {
        matrix4 = matrix4.transpose();
    }
    let inverse_matrix = if inverse_matrix != ptr::null_mut() {
        if transpose == 0 {
            *&*(matrix as *const [f32; 16])
        } else {
            let transposed_inverse_matrix4 =
                glam::Mat4::from_cols_array(&*(matrix as *const [f32; 16]));
            transposed_inverse_matrix4.to_cols_array()
        }
    } else {
        matrix4.inverse().transpose().to_cols_array()
    };
    let (scale, rotation, translation) = matrix4.to_scale_rotation_translation();
    transform.transform = matrix4.transpose().to_cols_array();
    transform.inverse_transform = inverse_matrix;
    transform.scale = scale;
    transform.rotation = rotation;
    transform.translation = translation;
    Ok(())
}

pub(crate) unsafe fn set_child(
    transform: Transform,
    object: UntypedObject,
) -> Result<(), RTresult> {
    let object = TypedObjectWeak::clone_from(object)?;
    let transform = null_unwrap(transform)?;
    let mut transform = transform.borrow_mut()?;
    let geometry_group = match object {
        TypedObjectWeak::GeometryGroup(geometry_group) => geometry_group,
        _ => return Err(RTresult::RT_ERROR_NOT_SUPPORTED),
    };
    transform.child = Some(geometry_group);
    Ok(())
}

pub(crate) unsafe fn get_motion_key_count(
    transform: Transform,
    n: *mut u32,
) -> Result<(), RTresult> {
    null_check(transform)?;
    null_check(n)?;
    *n = 1;
    Ok(())
}

pub(crate) fn destroy(_transform: Transform) -> Result<(), RTresult> {
    // TODO: implement
    Ok(())
}

pub(crate) unsafe fn get_context(
    transform: *const OptixCell<TransformData>,
    context: *mut *const OptixCell<ContextData>,
) -> Result<(), RTresult> {
    let transform = null_unwrap(transform)?;
    let transform = transform.borrow()?;
    *context = transform.context.as_ptr();
    Ok(())
}
