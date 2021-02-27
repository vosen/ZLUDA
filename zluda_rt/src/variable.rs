use crate::{
    context::ContextData, null_check, null_unwrap, AlignedBuffer, MaybeWeakRefMut, OptixCell,
    OptixObjectData, TypeTag, TypedObjectWeak, UntypedObject,
};
use optix_types::RTresult;
use std::{
    alloc::Layout,
    mem, ptr,
    rc::{Rc, Weak},
    slice,
};

pub(crate) type Variable = *const OptixCell<VariableData>;

#[derive(Clone)]
pub(crate) struct VariableData {
    pub(crate) context: Weak<OptixCell<ContextData>>,
    pub(crate) value: VariableValue,
}

#[derive(Clone)]
pub(crate) enum VariableValue {
    None,
    Object(TypedObjectWeak),
    Inline {
        data: [u8; 4 * mem::size_of::<f32>()],
        size: u8,
    },
    Boxed(AlignedBuffer),
}

impl VariableValue {
    fn with_bytes<T>(&self, f: impl FnOnce(&[u8]) -> T) -> Result<T, RTresult> {
        Ok(match self {
            VariableValue::None => return Err(RTresult::RT_ERROR_UNKNOWN),
            VariableValue::Object(object) => match object {
                TypedObjectWeak::Buffer(buffer) => {
                    let buffer = buffer.upgrade().ok_or(RTresult::RT_ERROR_UNKNOWN)?;
                    let buffer = buffer.borrow()?;
                    let device_buffer = buffer.get_device_mip0();
                    let byte_slice = unsafe {
                        slice::from_raw_parts(
                            &device_buffer as *const _ as *const u8,
                            mem::size_of_val(&device_buffer),
                        )
                    };
                    f(byte_slice)
                }
                TypedObjectWeak::GeometryGroup(gg) => {
                    let gg = gg.upgrade().ok_or(RTresult::RT_ERROR_UNKNOWN)?;
                    let gg = gg.borrow()?;
                    f(&gg.index.to_ne_bytes())
                }
                TypedObjectWeak::Group(group) => {
                    let group = group.upgrade().ok_or(RTresult::RT_ERROR_UNKNOWN)?;
                    let group = group.borrow()?;
                    f(&group.index.to_ne_bytes())
                }
                TypedObjectWeak::TextureSampler(texture) => {
                    let texture = texture.upgrade().ok_or(RTresult::RT_ERROR_UNKNOWN)?;
                    let texture = texture.borrow()?;
                    f(&(texture.hip_object as usize).to_ne_bytes())
                }
                TypedObjectWeak::Transform(_)
                | TypedObjectWeak::Material(_)
                | TypedObjectWeak::Geometry(_)
                | TypedObjectWeak::GeometryTriangles(_)
                | TypedObjectWeak::GeometryInstance(_) => todo!(),
                TypedObjectWeak::Context(_)
                | TypedObjectWeak::Variable(_)
                | TypedObjectWeak::Program(_)
                | TypedObjectWeak::Acceleration(_) => return Err(RTresult::RT_ERROR_NOT_SUPPORTED),
            },
            VariableValue::Inline { data, size } => f(&data[..*size as usize]),
            VariableValue::Boxed(data) => f(data.as_bytes()),
        })
    }
}

impl VariableData {
    pub(crate) fn new<T: OptixObjectData>(
        owner: &mut T,
    ) -> Result<Rc<OptixCell<VariableData>>, RTresult> {
        let context = owner.context();
        let context = match context {
            MaybeWeakRefMut::Weak(weak_ctx) => weak_ctx.clone(),
            MaybeWeakRefMut::Ref(_) => return Err(RTresult::RT_ERROR_UNKNOWN),
        };
        Ok(Rc::new(OptixCell::new(Self {
            context,
            value: VariableValue::None,
        })))
    }

    pub(crate) fn new_with_context(
        context: &OptixCell<ContextData>,
    ) -> Result<Rc<OptixCell<VariableData>>, RTresult> {
        let context = unsafe { OptixCell::clone_weak(context as _) };
        Ok(Rc::new(OptixCell::new(Self {
            context: context,
            value: VariableValue::None,
        })))
    }

    pub(crate) fn copy_into_buffer(&self, buffer: &mut [u8]) -> Result<(), RTresult> {
        self.value.with_bytes(|byte_buffer_host| {
            if buffer.len() != byte_buffer_host.len() {
                return Err(RTresult::RT_ERROR_UNKNOWN);
            }
            buffer.copy_from_slice(byte_buffer_host);
            Ok(())
        })?
    }
}

impl OptixObjectData for VariableData {
    const TYPE: TypeTag = TypeTag::Variable;

    fn deregister(&mut self, _this: &Rc<OptixCell<Self>>) -> Result<(), RTresult> {
        // Variables are only ever destroyed implicitly
        Err(RTresult::RT_ERROR_UNKNOWN)
    }

    fn context<'a>(&'a mut self) -> MaybeWeakRefMut<'a, ContextData> {
        MaybeWeakRefMut::Weak(&self.context)
    }
}

pub(crate) unsafe fn set_object(v: Variable, object: UntypedObject) -> Result<(), RTresult> {
    let var = null_unwrap(v)?;
    let object = TypedObjectWeak::clone_from(object)?;
    let mut var = var.borrow_mut()?;
    var.value = VariableValue::Object(object);
    Ok(())
}

pub(crate) unsafe fn get_object(v: Variable, result: *mut UntypedObject) -> Result<(), RTresult> {
    null_check(result)?;
    let var = null_unwrap(v)?;
    let var = var.borrow()?;
    let object = match var.value {
        VariableValue::Object(ref obj) => obj,
        _ => return Err(RTresult::RT_ERROR_INVALID_VALUE),
    };
    *result = object.as_untyped();
    Ok(())
}

pub(crate) unsafe fn set_1f(v: Variable, f1: f32) -> Result<(), RTresult> {
    let var = null_unwrap(v)?;
    let mut var = var.borrow_mut()?;
    var.value = pack_into_value(f1);
    Ok(())
}

pub(crate) unsafe fn set_1i(v: *const OptixCell<VariableData>, i1: i32) -> Result<(), RTresult> {
    let var = null_unwrap(v)?;
    let mut var = var.borrow_mut()?;
    var.value = pack_into_value(i1);
    Ok(())
}

pub(crate) unsafe fn set_1ui(v: Variable, u1: u32) -> Result<(), RTresult> {
    let var = null_unwrap(v)?;
    let mut var = var.borrow_mut()?;
    var.value = pack_into_value(u1);
    Ok(())
}

pub(crate) unsafe fn set_3f(v: Variable, f1: f32, f2: f32, f3: f32) -> Result<(), RTresult> {
    let var = null_unwrap(v)?;
    let mut var = var.borrow_mut()?;
    var.value = pack_into_value([f1, f2, f3]);
    Ok(())
}

pub(crate) unsafe fn set_3fv(v: Variable, f: *const f32) -> Result<(), RTresult> {
    null_check(f)?;
    let f = *(f as *const [f32; 3]);
    set_3f(v, f[0], f[1], f[2])
}

pub(crate) unsafe fn set_4f(
    v: Variable,
    f1: f32,
    f2: f32,
    f3: f32,
    f4: f32,
) -> Result<(), RTresult> {
    let var = null_unwrap(v)?;
    let mut var = var.borrow_mut()?;
    var.value = pack_into_value([f1, f2, f3, f4]);
    Ok(())
}

pub(crate) unsafe fn set_4fv(v: Variable, f: *const f32) -> Result<(), RTresult> {
    null_check(f)?;
    let f = *(f as *const [f32; 4]);
    set_4f(v, f[0], f[1], f[2], f[3])
}

unsafe fn pack_into_value<T>(t: T) -> VariableValue {
    if mem::size_of::<T>() > 4 * mem::size_of::<f32>() {
        let buffer = AlignedBuffer::new(Layout::new::<T>());
        VariableValue::Boxed(buffer)
    } else {
        let mut data = [0u8; 4 * mem::size_of::<f32>()];
        ptr::copy_nonoverlapping::<T>(&t, data.as_mut_ptr() as _, 1);
        let size = mem::size_of::<T>() as u8;
        VariableValue::Inline { data, size }
    }
}

pub(crate) unsafe fn set_user_data(
    v: Variable,
    size: u64,
    ptr: *const std::ffi::c_void,
) -> Result<(), RTresult> {
    null_check(ptr)?;
    let var = null_unwrap(v)?;
    let mut var = var.borrow_mut()?;
    let buffer = AlignedBuffer::new(Layout::from_size_align_unchecked(size as usize, 1));
    ptr::copy_nonoverlapping(
        ptr.cast::<u8>(),
        buffer.as_ptr().cast::<u8>(),
        size as usize,
    );
    var.value = VariableValue::Boxed(buffer);
    Ok(())
}

pub(crate) unsafe fn set_1ull(v: Variable, ull1: u64) -> Result<(), RTresult> {
    let var = null_unwrap(v)?;
    let mut var = var.borrow_mut()?;
    var.value = pack_into_value(ull1);
    Ok(())
}
