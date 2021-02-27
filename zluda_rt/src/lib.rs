#[macro_use]
extern crate nougat;

mod acceleration;
mod buffer;
mod cache;
mod context;
mod eptx;
mod geometry;
mod geometry_group;
mod geometry_instance;
mod geometry_triangles;
mod group;
mod hip;
mod material;
mod program;
mod repr_gpu;
#[cfg(test)]
mod test_common;
#[cfg(test)]
mod tests;
mod texture_sampler;
mod transform;
mod variable;

use crate::texture_sampler::{TextureSampler, TextureSamplerData};
use acceleration::{Acceleration, AccelerationData};
use buffer::{Buffer, BufferData};
use context::{Context, ContextData};
use geometry::{Geometry, GeometryData};
use geometry_group::{GeometryGroup, GeometryGroupData};
use geometry_instance::{GeometryInstance, GeometryInstanceData};
use geometry_triangles::{GeometryTriangles, GeometryTrianglesData};
use group::{Group, GroupData};
use hip_runtime_sys::{
    hipDeviceAttribute_t, hipDeviceGetAttribute, hipDeviceTotalMem, hipDeviceptr_t, hipMemcpyDtoH,
};
use material::{Material, MaterialData};
use optix_types::*;
use paste::paste;
use program::{Program, ProgramData};
use std::{
    alloc::Layout,
    cell::{Ref, RefCell, RefMut},
    collections::HashSet,
    ffi::c_void,
    hash::{BuildHasherDefault, Hash, Hasher},
    mem::{self, ManuallyDrop},
    os::raw::c_char,
    ptr::{self, NonNull},
    rc::{Rc, Weak},
};
use transform::{Transform, TransformData};
use variable::{Variable, VariableData};

macro_rules! optix6_unimplemented {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* $(,)? ) -> $ret_type:ty);*) => {
        $(
            #[no_mangle]
            unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                definitions::unimplemented()
            }
        )*
    };
}

macro_rules! optix6_fn {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* $(,)? ) -> $ret_type:ty);*) => {
        $(
            #[no_mangle]
            unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                IntoOptix::<$ret_type>::into_optix(definitions::$fn_name( $( InternalRepresentation::to_internal($arg_id) ),* ))
            }
        )*
    };
}

#[macro_export]
macro_rules! unwrap_or_continue {
    ($option:expr) => {
        match $option {
            Some(x) => x,
            None => continue,
        }
    };
}

#[macro_export]
macro_rules! hip {
    ($expr:expr, $err:ident) => {
        #[allow(unused_unsafe)]
        {
            let err = unsafe { $expr };
            if err != hip_runtime_sys::hipError_t::hipSuccess {
                return Result::Err(RTresult::$err);
            }
        }
    };
}

#[macro_export]
macro_rules! hiprt {
    ($expr:expr, $err:ident) => {
        #[allow(unused_unsafe)]
        {
            let err = unsafe { $expr };
            if err != hiprt_sys::hiprtError::hiprtSuccess {
                return Result::Err(RTresult::$err);
            }
        }
    };
}

optix_base::optix6_function_declarations!(
    optix6_unimplemented,
    optix6_fn,
    [
        rtAccelerationCreate,
        rtAccelerationDestroy,
        rtAccelerationGetContext,
        rtAccelerationSetBuilder,
        rtAccelerationMarkDirty,
        rtBufferCreate,
        rtBufferCreateFromCallback,
        rtBufferDestroy,
        rtBufferSetElementSize,
        rtBufferSetFormat,
        rtBufferSetMipLevelCount,
        rtBufferSetSize1D,
        rtBufferSetSize2D,
        rtBufferGetContext,
        rtBufferGetDevicePointer,
        rtBufferGetDimensionality,
        rtBufferGetElementSize,
        rtBufferGetFormat,
        rtBufferGetGLBOId,
        rtBufferGetId,
        rtBufferGetMipLevelCount,
        rtBufferGetMipLevelSize2D,
        rtBufferGetSize1D,
        rtBufferGetSize2D,
        rtBufferGetSizev,
        rtBufferMap,
        rtBufferMapEx,
        rtBufferUnmap,
        rtBufferUnmapEx,
        rtContextCreate,
        rtContextDeclareVariable,
        rtContextDestroy,
        rtContextGetAttribute,
        rtContextGetBufferFromId,
        rtContextGetDeviceCount,
        rtContextGetDevices,
        rtContextGetErrorString,
        rtContextLaunch2D,
        rtContextQueryVariable,
        rtContextSetAttribute,
        rtContextSetDevices,
        rtContextSetEntryPointCount,
        rtContextSetExceptionEnabled,
        rtContextSetExceptionProgram,
        rtContextSetMaxCallableProgramDepth,
        rtContextSetMaxTraceDepth,
        rtContextSetMissProgram,
        rtContextSetPrintEnabled,
        rtContextSetPrintLaunchIndex,
        rtContextSetRayGenerationProgram,
        rtContextSetRayTypeCount,
        rtContextSetStackSize,
        rtContextSetUsageReportCallback,
        rtContextValidate,
        rtDeviceGetAttribute,
        rtDeviceGetDeviceCount,
        rtGeometryCreate,
        rtGeometryDeclareVariable,
        rtGeometryDestroy,
        rtGeometryGetContext,
        rtGeometryQueryVariable,
        rtGeometrySetPrimitiveCount,
        rtGeometrySetBoundingBoxProgram,
        rtGeometrySetIntersectionProgram,
        rtGeometryGroupCreate,
        rtGeometryGroupDestroy,
        rtGeometryGroupGetChildCount,
        rtGeometryGroupGetContext,
        rtGeometryGroupSetAcceleration,
        rtGeometryGroupSetChild,
        rtGeometryGroupSetChildCount,
        rtGeometryGroupSetVisibilityMask,
        rtGeometryInstanceCreate,
        rtGeometryInstanceDeclareVariable,
        rtGeometryInstanceDestroy,
        rtGeometryInstanceQueryVariable,
        rtGeometryInstanceGetContext,
        rtGeometryInstanceGetMaterialCount,
        rtGeometryInstanceSetGeometry,
        rtGeometryInstanceSetGeometryTriangles,
        rtGeometryInstanceSetMaterial,
        rtGeometryInstanceSetMaterialCount,
        rtGeometryTrianglesCreate,
        rtGeometryTrianglesDeclareVariable,
        rtGeometryTrianglesDestroy,
        rtGeometryTrianglesGetContext,
        rtGeometryTrianglesSetAttributeProgram,
        rtGeometryTrianglesSetBuildFlags,
        rtGeometryTrianglesSetFlagsPerMaterial,
        rtGeometryTrianglesSetPrimitiveCount,
        rtGeometryTrianglesSetTriangleIndices,
        rtGeometryTrianglesSetVertices,
        rtGeometryTrianglesQueryVariable,
        rtGeometryTrianglesValidate,
        rtGetVersion,
        rtGlobalGetAttribute,
        rtGlobalSetAttribute,
        rtGroupCreate,
        rtGroupDestroy,
        rtGroupGetAcceleration,
        rtGroupGetChild,
        rtGroupGetChildCount,
        rtGroupGetContext,
        rtGroupSetAcceleration,
        rtGroupSetChild,
        rtGroupSetChildCount,
        rtMaterialCreate,
        rtMaterialDeclareVariable,
        rtMaterialDestroy,
        rtMaterialGetContext,
        rtMaterialQueryVariable,
        rtMaterialSetAnyHitProgram,
        rtMaterialSetClosestHitProgram,
        rtProgramCreateFromProgram,
        rtProgramCreateFromPTXFile,
        rtProgramCreateFromPTXString,
        rtProgramDeclareVariable,
        rtProgramDestroy,
        rtProgramGetContext,
        rtProgramGetId,
        rtProgramQueryVariable,
        rtProgramValidate,
        rtTextureSamplerCreate,
        rtTextureSamplerDestroy,
        rtTextureSamplerGetBuffer,
        rtTextureSamplerGetContext,
        rtTextureSamplerGetId,
        rtTextureSamplerSetArraySize,
        rtTextureSamplerSetBuffer,
        rtTextureSamplerSetFilteringModes,
        rtTextureSamplerSetIndexingMode,
        rtTextureSamplerSetMaxAnisotropy,
        rtTextureSamplerSetMipLevelCount,
        rtTextureSamplerSetReadMode,
        rtTextureSamplerSetWrapMode,
        rtTransformCreate,
        rtTransformDestroy,
        rtTransformGetContext,
        rtTransformGetMotionKeyCount,
        rtTransformSetChild,
        rtTransformSetMatrix,
        rtVariableSet1f,
        rtVariableSet1i,
        rtVariableSet1ui,
        rtVariableSet1ull,
        rtVariableSet3f,
        rtVariableSet3fv,
        rtVariableSet4f,
        rtVariableSet4fv,
        rtVariableSetObject,
        rtVariableGetObject,
        rtVariableSetUserData
    ]
);

pub(crate) trait InternalRepresentation<T: Sized>: Sized {
    fn to_internal(t: T) -> Self {
        unsafe { mem::transmute_copy(&t) }
    }
}

impl<From, Into: InternalRepresentation<From>> InternalRepresentation<*mut From> for *mut Into {}
impl<From, Into: InternalRepresentation<From>> InternalRepresentation<*const From> for *const Into {}

// Unchanged
impl InternalRepresentation<RTgeometrybuildflags> for RTgeometrybuildflags {}
impl InternalRepresentation<RTresult> for RTresult {}
impl InternalRepresentation<RTformat> for RTformat {}
impl InternalRepresentation<RTwrapmode> for RTwrapmode {}
impl InternalRepresentation<RTgeometryflags> for RTgeometryflags {}
impl InternalRepresentation<RTusagereportcallback> for RTusagereportcallback {}
impl InternalRepresentation<RTexception> for RTexception {}
impl InternalRepresentation<RTcontextattribute> for RTcontextattribute {}
impl InternalRepresentation<RTglobalattribute> for RTglobalattribute {}
impl InternalRepresentation<RTtextureindexmode> for RTtextureindexmode {}
impl InternalRepresentation<RTdeviceattribute> for RTdeviceattribute {}
impl InternalRepresentation<RTtexturereadmode> for RTtexturereadmode {}
impl InternalRepresentation<RTfiltermode> for RTfiltermode {}
impl InternalRepresentation<c_char> for c_char {}
impl InternalRepresentation<f32> for f32 {}
impl InternalRepresentation<i32> for i32 {}
impl InternalRepresentation<u32> for u32 {}
impl InternalRepresentation<u64> for u64 {}
impl InternalRepresentation<c_void> for c_void {}
// ZLUDA
impl InternalRepresentation<RTobject> for UntypedObject {}
impl InternalRepresentation<RTbuffercallback>
    for Option<
        unsafe extern "C" fn(
            callbackData: *mut ::std::os::raw::c_void,
            buffer: Buffer,
            block: *mut RTmemoryblock,
        ) -> ::std::os::raw::c_int,
    >
{
}

#[repr(C)]
struct OptixCell<T: OptixObjectData> {
    tag: TypeTag,
    data: RefCell<T>,
}

impl<T: OptixObjectData> OptixCell<T> {
    unsafe fn clone_rc(cell: *const OptixCell<T>) -> Rc<OptixCell<T>> {
        let weak = ManuallyDrop::new(Rc::from_raw(cell));
        Rc::clone(&weak)
    }

    unsafe fn clone_weak(cell: *const OptixCell<T>) -> Weak<OptixCell<T>> {
        let weak = ManuallyDrop::new(Weak::from_raw(cell));
        Weak::clone(&weak)
    }

    unsafe fn as_untyped(cell: *const OptixCell<T>) -> UntypedObject {
        cell.cast()
    }
}

struct RcHashSet<T: Sized>(HashSet<PtrEq<T>, BuildHasherDefault<WritethroughHasher<T>>>);

#[repr(transparent)]
struct PtrEq<T: Sized>(Rc<T>);

impl<T: Sized> PartialEq for PtrEq<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<T: Sized> Eq for PtrEq<T> {}

impl<T: Sized> Hash for PtrEq<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(unsafe { mem::transmute_copy::<Rc<T>, usize>(&self.0) })
    }
}
struct WritethroughHasher<T: Sized>(*const T);

impl<T: Sized> Default for WritethroughHasher<T> {
    fn default() -> Self {
        Self(ptr::null())
    }
}

impl<T> Hasher for WritethroughHasher<T> {
    fn finish(&self) -> u64 {
        unsafe { mem::transmute(self.0) }
    }

    fn write(&mut self, _bytes: &[u8]) {
        unreachable!()
    }

    fn write_usize(&mut self, i: usize) {
        self.0 = i as *const T;
    }
}

impl<T> RcHashSet<T> {
    fn new() -> Self {
        Self(HashSet::default())
    }

    fn insert(&mut self, value: Rc<T>) -> bool {
        self.0.insert(PtrEq(value))
    }

    fn remove(&mut self, value: &Rc<T>) -> bool {
        self.0
            .remove(unsafe { mem::transmute::<&Rc<T>, &PtrEq<T>>(value) })
    }

    fn iter(&self) -> std::collections::hash_set::Iter<'_, Rc<T>> {
        unsafe { mem::transmute(self.0.iter()) }
    }
}

impl<T: OptixObjectData> OptixCell<T> {
    fn new(t: T) -> Self {
        Self {
            tag: T::TYPE,
            data: RefCell::new(t),
        }
    }

    fn borrow(&self) -> Result<Ref<'_, T>, RTresult> {
        self.data
            .try_borrow()
            .map_err(|_| RTresult::RT_ERROR_UNKNOWN)
    }

    fn borrow_mut(&self) -> Result<RefMut<'_, T>, RTresult> {
        let mut this = self
            .data
            .try_borrow_mut()
            .map_err(|_| RTresult::RT_ERROR_UNKNOWN)?;
        match this.context() {
            MaybeWeakRefMut::Weak(weak_ctx) => {
                let ctx = weak_ctx.upgrade().ok_or(RTresult::RT_ERROR_UNKNOWN)?;
                if let Ok(mut ctx) = ctx.borrow_mut() {
                    // We might be called from within launch2d, where ctx is already mutably borrowed
                    ctx.invalidate();
                }
                drop(ctx);
            }
            MaybeWeakRefMut::Ref(ctx) => ctx.invalidate(),
        }
        Ok(this)
    }

    fn borrow_mut_no_invalidate(&self) -> Result<RefMut<'_, T>, RTresult> {
        self.data
            .try_borrow_mut()
            .map_err(|_| RTresult::RT_ERROR_UNKNOWN)
    }

    unsafe fn destroy(ptr: *const OptixCell<T>) -> Result<(), RTresult> {
        let obj = null_unwrap(ptr)?;
        let mut this = obj.borrow_mut()?;
        let rc_ptr = ManuallyDrop::new(Rc::from_raw(obj));
        this.deregister(&*rc_ptr)
    }
}

type UntypedObject = *const TypeTag;

macro_rules! optix_types {
    ($ctx:ident, [$($type_:ident),+]) => {
        #[repr(u8)]
        enum TypeTag {
            $ctx = 1,
            $(
                $type_,
            )+
        }

        optix_types!(@ $ctx, $($type_),+);
    };
    (@ $($type_:ident),+) => {
        $(
            impl InternalRepresentation<paste!{ [<RT $type_:lower>] }> for $type_ {}
        )+

        #[derive(Clone)]
        enum TypedObjectWeak {
            $(
                $type_(Weak<OptixCell< paste! { [<$type_ Data>] } >>),
            )+
        }

        impl TypedObjectWeak {
            fn as_untyped(&self) -> UntypedObject {
                match self {
                    $(
                        TypedObjectWeak::$type_(weak) => {
                            let result: $type_ = Weak::as_ptr(weak);
                            result as UntypedObject
                        },
                    )+
                }
            }


            unsafe fn clone_from(obj: UntypedObject) -> Result<TypedObjectWeak, RTresult> {
                null_check(obj)?;
                Ok(match *obj {
                    $(
                        TypeTag::$type_ => {
                            let fake_strong = ManuallyDrop::new(Rc::from_raw(mem::transmute::<_, $type_>(obj)));
                            TypedObjectWeak::$type_(Rc::downgrade(&fake_strong))
                        }
                    )+
                })
            }

            $(
                paste!{
                    #[allow(dead_code)]
                    #[allow(non_snake_case)]
                    unsafe fn [<clone_from_ $type_:snake>] ($type_: $type_) -> TypedObjectWeak {
                        let fake_strong = ManuallyDrop::new(Rc::from_raw($type_));
                        TypedObjectWeak::$type_(Rc::downgrade(&fake_strong))
                    }
                }
            )+
        }
    };
}

optix_types!(
    Context,
    [
        Buffer,
        Variable,
        Program,
        Material,
        Geometry,
        GeometryTriangles,
        GeometryGroup,
        Group,
        Acceleration,
        GeometryInstance,
        TextureSampler,
        Transform
    ]
);

struct AlignedBuffer {
    size: usize,
    pub(crate) ptr: NonNull<u8>,
    align: u32,
}

impl AlignedBuffer {
    fn new(layout: Layout) -> Self {
        let size = layout.size();
        let align = layout.align() as u32;
        let ptr = NonNull::new(unsafe { std::alloc::alloc(layout) }).unwrap();
        Self { size, align, ptr }
    }

    fn len(&self) -> usize {
        self.size as usize
    }

    fn from_hip(layout: Layout, dev_data: hipDeviceptr_t) -> Result<Self, RTresult> {
        let ptr = unsafe { std::alloc::alloc(layout) };
        hip! { hipMemcpyDtoH(ptr as _, dev_data, layout.size()), RT_ERROR_MEMORY_ALLOCATION_FAILED };
        let ptr = NonNull::new(ptr).unwrap();
        let size = layout.size();
        let align = layout.align() as u32;
        Ok(Self { size, align, ptr })
    }

    fn as_ptr(&self) -> *mut c_void {
        self.ptr.as_ptr() as _
    }

    fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.size) }
    }

    fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.size) }
    }
}

impl Clone for AlignedBuffer {
    fn clone(&self) -> Self {
        let mut buffer = AlignedBuffer::new(unsafe {
            Layout::from_size_align_unchecked(self.size, self.align as usize)
        });
        buffer.as_bytes_mut().copy_from_slice(self.as_bytes());
        buffer
    }
}

impl Drop for AlignedBuffer {
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(
                self.ptr.as_ptr(),
                Layout::from_size_align_unchecked(self.size, self.align as usize),
            )
        }
    }
}

fn div_positive_round_up(dividend: u64, divisor: u64) -> u64 {
    let mut result = dividend / divisor;
    if (dividend % divisor) != 0 {
        result += 1;
    }
    result
}

unsafe fn slice_cast_mut<'a, T: Sized, U: Sized>(from: &'a mut [T], count: usize) -> &'a mut [U] {
    assert!(from.len() * mem::size_of::<T>() >= count * mem::size_of::<U>());
    std::slice::from_raw_parts_mut(from.as_mut_ptr() as _, count)
}

trait OptixObjectData: Sized {
    const TYPE: TypeTag;

    fn deregister(&mut self, this: &Rc<OptixCell<Self>>) -> Result<(), RTresult>;
    fn context<'a>(&'a mut self) -> MaybeWeakRefMut<'a, ContextData>;
}

pub(crate) enum MaybeWeakRefMut<'a, T: OptixObjectData> {
    Weak(&'a Weak<OptixCell<T>>),
    Ref(&'a mut T),
}

pub(crate) trait IntoOptix<T> {
    fn into_optix(self) -> T;
}

impl IntoOptix<()> for () {
    fn into_optix(self) -> () {
        self
    }
}

impl IntoOptix<RTresult> for () {
    fn into_optix(self) -> RTresult {
        RTresult::RT_SUCCESS
    }
}

impl IntoOptix<RTresult> for RTresult {
    fn into_optix(self) -> RTresult {
        self
    }
}

impl IntoOptix<RTresult> for Result<(), RTresult> {
    fn into_optix(self) -> RTresult {
        match self {
            Ok(()) => IntoOptix::into_optix(()),
            Err(err) => IntoOptix::into_optix(err),
        }
    }
}

trait NullablePointer {
    fn null() -> Self;
}

impl<T> NullablePointer for *mut T {
    fn null() -> Self {
        ptr::null_mut()
    }
}

impl<T> NullablePointer for *const T {
    fn null() -> Self {
        ptr::null_mut()
    }
}

#[must_use]
fn null_check<T: NullablePointer + PartialEq>(ptr: T) -> Result<(), RTresult> {
    if ptr == T::null() {
        Err(RTresult::RT_ERROR_INVALID_VALUE)
    } else {
        Ok(())
    }
}

#[must_use]
unsafe fn null_unwrap<'a, T>(t: *const T) -> Result<&'a T, RTresult> {
    t.as_ref().ok_or(RTresult::RT_ERROR_INVALID_VALUE)
}

#[must_use]
unsafe fn null_unwrap_mut<'a, T>(t: *mut T) -> Result<&'a mut T, RTresult> {
    t.as_mut().ok_or(RTresult::RT_ERROR_INVALID_VALUE)
}

#[allow(non_snake_case)]
mod definitions {
    use crate::{
        acceleration::{self, Acceleration},
        buffer::{self, Buffer},
        context::{self, Context},
        geometry::{self, Geometry},
        geometry_group::{self, GeometryGroup},
        geometry_instance::{self, GeometryInstance},
        geometry_triangles::{self, GeometryTriangles},
        group::{self, Group},
        material::{self, Material},
        program::{self, Program},
        texture_sampler::{self, TextureSampler},
        transform::{self, Transform},
        variable::{self, Variable},
        UntypedObject,
    };
    use optix_types::*;
    use std::os::raw::{c_int, c_uint};

    #[cfg(debug_assertions)]
    pub(crate) fn unimplemented() -> RTresult {
        unimplemented!()
    }

    #[cfg(not(debug_assertions))]
    pub(crate) fn unimplemented() -> RTresult {
        RTresult::RT_ERROR_NOT_SUPPORTED
    }

    pub(crate) unsafe fn rtAccelerationCreate(
        context: Context,
        acceleration: *mut Acceleration,
    ) -> Result<(), RTresult> {
        acceleration::create(context, acceleration)
    }

    pub(crate) unsafe fn rtAccelerationDestroy(acceleration: Acceleration) -> Result<(), RTresult> {
        acceleration::destroy(acceleration)
    }

    pub(crate) unsafe fn rtAccelerationGetContext(
        acceleration: Acceleration,
        context: *mut Context,
    ) -> Result<(), RTresult> {
        acceleration::get_context(acceleration, context)
    }

    pub(crate) unsafe fn rtAccelerationSetBuilder(
        acceleration: Acceleration,
        builder: *const ::std::os::raw::c_char,
    ) -> Result<(), RTresult> {
        acceleration::set_builder(acceleration, builder)
    }

    pub(crate) unsafe fn rtAccelerationMarkDirty(
        acceleration: Acceleration,
    ) -> Result<(), RTresult> {
        acceleration::mark_dirty(acceleration)
    }

    pub(crate) unsafe fn rtBufferCreate(
        context: Context,
        bufferdesc: ::std::os::raw::c_uint,
        buffer: *mut Buffer,
    ) -> Result<(), RTresult> {
        buffer::create(context, bufferdesc, buffer)
    }

    pub(crate) unsafe fn rtBufferCreateFromCallback(
        context: Context,
        bufferdesc: ::std::os::raw::c_uint,
        callback: Option<
            unsafe extern "C" fn(
                callbackData: *mut ::std::os::raw::c_void,
                buffer: Buffer,
                block: *mut RTmemoryblock,
            ) -> ::std::os::raw::c_int,
        >,
        callback_data: *mut ::std::os::raw::c_void,
        buffer: *mut Buffer,
    ) -> Result<(), RTresult> {
        buffer::create_from_callback(context, bufferdesc, callback, callback_data, buffer)
    }

    pub(crate) unsafe fn rtBufferDestroy(buffer: Buffer) -> Result<(), RTresult> {
        buffer::destroy(buffer)
    }

    pub(crate) unsafe fn rtBufferSetElementSize(
        buffer: Buffer,
        element_size: u64,
    ) -> Result<(), RTresult> {
        buffer::set_element_size(buffer, element_size)
    }

    pub(crate) unsafe fn rtBufferSetFormat(
        buffer: Buffer,
        format: RTformat,
    ) -> Result<(), RTresult> {
        buffer::set_format(buffer, format)
    }

    pub(crate) unsafe fn rtBufferSetMipLevelCount(
        buffer: Buffer,
        levels: ::std::os::raw::c_uint,
    ) -> Result<(), RTresult> {
        buffer::set_mip_level_count(buffer, levels)
    }

    #[allow(non_snake_case)]
    pub(crate) unsafe fn rtBufferSetSize1D(buffer: Buffer, width: RTsize) -> Result<(), RTresult> {
        buffer::set_size1d(buffer, width)
    }

    #[allow(non_snake_case)]
    pub(crate) unsafe fn rtBufferSetSize2D(
        buffer: Buffer,
        width: RTsize,
        height: RTsize,
    ) -> Result<(), RTresult> {
        buffer::set_size2d(buffer, width, height)
    }

    pub(crate) unsafe fn rtBufferGetContext(
        buffer: Buffer,
        context: *mut Context,
    ) -> Result<(), RTresult> {
        buffer::get_context(buffer, context)
    }

    pub(crate) unsafe fn rtBufferGetDevicePointer(
        buffer: Buffer,
        optix_device_ordinal: ::std::os::raw::c_int,
        device_pointer: *mut *mut ::std::os::raw::c_void,
    ) -> Result<(), RTresult> {
        buffer::get_device_pointer(buffer, optix_device_ordinal, device_pointer)
    }

    #[allow(non_snake_case)]
    pub(crate) unsafe fn rtBufferGetDimensionality(
        buffer: Buffer,
        dimensionality: *mut c_uint,
    ) -> Result<(), RTresult> {
        buffer::get_dimensionality(buffer, dimensionality)
    }

    #[allow(non_snake_case)]
    pub(crate) unsafe fn rtBufferGetElementSize(
        buffer: Buffer,
        elementSize: *mut u64,
    ) -> Result<(), RTresult> {
        buffer::get_element_size(buffer, elementSize)
    }

    #[allow(non_snake_case)]
    pub(crate) unsafe fn rtBufferGetGLBOId(
        buffer: Buffer,
        glid: *mut c_uint,
    ) -> Result<(), RTresult> {
        buffer::get_glboid(buffer, glid)
    }

    #[allow(non_snake_case)]
    pub(crate) unsafe fn rtBufferGetFormat(
        buffer: Buffer,
        format: *mut RTformat,
    ) -> Result<(), RTresult> {
        buffer::get_format(buffer, format)
    }

    pub(crate) unsafe fn rtBufferGetId(
        buffer: Buffer,
        bufferId: *mut ::std::os::raw::c_int,
    ) -> Result<(), RTresult> {
        buffer::get_id(buffer, bufferId)
    }

    pub(crate) unsafe fn rtBufferGetMipLevelCount(
        buffer: Buffer,
        level: *mut ::std::os::raw::c_uint,
    ) -> Result<(), RTresult> {
        buffer::get_miplevel_count(buffer, level)
    }

    pub(crate) unsafe fn rtBufferGetMipLevelSize2D(
        buffer: Buffer,
        level: ::std::os::raw::c_uint,
        width: *mut RTsize,
        height: *mut RTsize,
    ) -> Result<(), RTresult> {
        buffer::get_miplevel_size2d(buffer, level, width, height)
    }

    pub(crate) unsafe fn rtBufferGetSize1D(
        buffer: Buffer,
        width: *mut RTsize,
    ) -> Result<(), RTresult> {
        buffer::get_size1d(buffer, width)
    }

    #[allow(non_snake_case)]
    pub(crate) unsafe fn rtBufferGetSize2D(
        buffer: Buffer,
        width: *mut RTsize,
        height: *mut RTsize,
    ) -> Result<(), RTresult> {
        buffer::get_size2d(buffer, width, height)
    }

    pub(crate) unsafe fn rtBufferGetSizev(
        buffer: Buffer,
        dimensionality: ::std::os::raw::c_uint,
        dims: *mut RTsize,
    ) -> Result<(), RTresult> {
        buffer::get_sizev(buffer, dimensionality, dims)
    }

    #[allow(non_snake_case)]
    pub(crate) unsafe fn rtBufferMap(
        buffer: Buffer,
        userPointer: *mut *mut ::std::os::raw::c_void,
    ) -> Result<(), RTresult> {
        buffer::map(buffer, userPointer)
    }

    #[allow(non_snake_case)]
    pub(crate) unsafe fn rtBufferMapEx(
        buffer: Buffer,
        mapFlags: ::std::os::raw::c_uint,
        level: ::std::os::raw::c_uint,
        userOwned: *mut ::std::os::raw::c_void,
        optixOwned: *mut *mut ::std::os::raw::c_void,
    ) -> Result<(), RTresult> {
        buffer::map_ex(buffer, mapFlags, level, userOwned, optixOwned)
    }

    #[allow(non_snake_case)]
    pub(crate) unsafe fn rtBufferUnmap(buffer: Buffer) -> Result<(), RTresult> {
        buffer::unmap(buffer)
    }

    #[allow(non_snake_case)]
    pub(crate) unsafe fn rtBufferUnmapEx(
        buffer: Buffer,
        level: ::std::os::raw::c_uint,
    ) -> Result<(), RTresult> {
        buffer::unmap_ex(buffer, level)
    }

    #[allow(non_snake_case)]
    pub(crate) unsafe fn rtContextCreate(context: *mut Context) -> Result<(), RTresult> {
        context::create(context)
    }

    #[allow(non_snake_case)]
    pub(crate) unsafe fn rtContextDeclareVariable(
        context: Context,
        name: *const ::std::os::raw::c_char,
        v: *mut Variable,
    ) -> Result<(), RTresult> {
        context::declare_variable(context, name, v)
    }

    pub(crate) unsafe fn rtContextGetAttribute(
        context: Context,
        attrib: RTcontextattribute,
        size: RTsize,
        p: *mut ::std::os::raw::c_void,
    ) -> Result<(), RTresult> {
        context::get_attribute(context, attrib, size, p)
    }

    pub(crate) unsafe fn rtContextGetBufferFromId(
        context: Context,
        bufferId: ::std::os::raw::c_int,
        buffer: *mut Buffer,
    ) -> Result<(), RTresult> {
        context::get_buffer_from_id(context, bufferId, buffer)
    }

    pub(crate) unsafe fn rtContextGetDeviceCount(
        context: Context,
        count: *mut ::std::os::raw::c_uint,
    ) -> Result<(), RTresult> {
        context::get_device_count(context, count)
    }

    pub(crate) unsafe fn rtContextGetDevices(
        context: Context,
        devices: *mut ::std::os::raw::c_int,
    ) -> Result<(), RTresult> {
        context::get_devices(context, devices)
    }

    #[allow(non_snake_case)]
    pub(crate) unsafe fn rtContextDestroy(context: Context) -> Result<(), RTresult> {
        context::destroy(context)
    }

    #[allow(non_snake_case)]
    pub(crate) unsafe fn rtContextGetErrorString(
        ctx: Context,
        code: RTresult,
        string_return: *mut *const ::std::os::raw::c_char,
    ) {
        context::get_error_string(ctx, code, string_return)
    }

    pub(crate) unsafe fn rtContextLaunch2D(
        context: Context,
        entry_point_index: ::std::os::raw::c_uint,
        width: u64,
        height: u64,
    ) -> Result<(), RTresult> {
        context::launch_2d(context, entry_point_index, width, height)
    }

    pub(crate) unsafe fn rtContextQueryVariable(
        context: Context,
        name: *const ::std::os::raw::c_char,
        v: *mut Variable,
    ) -> Result<(), RTresult> {
        context::query_variable(context, name, v)
    }

    pub(crate) unsafe fn rtContextSetAttribute(
        context: Context,
        attrib: RTcontextattribute,
        size: RTsize,
        p: *const ::std::os::raw::c_void,
    ) -> Result<(), RTresult> {
        context::set_attribute(context, attrib, size, p)
    }

    pub(crate) unsafe fn rtContextSetDevices(
        context: Context,
        count: ::std::os::raw::c_uint,
        devices: *const ::std::os::raw::c_int,
    ) -> Result<(), RTresult> {
        context::set_devices(context, count, devices)
    }

    pub(crate) unsafe fn rtContextSetEntryPointCount(
        context: Context,
        count: c_uint,
    ) -> Result<(), RTresult> {
        context::set_entry_point_count(context, count)
    }

    pub(crate) fn rtContextSetExceptionEnabled(
        context: Context,
        exception: RTexception,
        enabled: ::std::os::raw::c_int,
    ) -> Result<(), RTresult> {
        context::set_exception_enabled(context, exception, enabled)
    }

    pub(crate) unsafe fn rtContextSetExceptionProgram(
        context: Context,
        entry_point_index: ::std::os::raw::c_uint,
        program: Program,
    ) -> Result<(), RTresult> {
        context::set_exception_program(context, entry_point_index, program)
    }

    pub(crate) unsafe fn rtContextSetMaxCallableProgramDepth(
        context: Context,
        maxDepth: ::std::os::raw::c_uint,
    ) -> RTresult {
        context::set_max_callable_program_depth(context, maxDepth)
    }

    pub(crate) unsafe fn rtContextSetMaxTraceDepth(
        context: Context,
        max_depth: u32,
    ) -> Result<(), RTresult> {
        context::set_max_depth(context, max_depth)
    }

    pub(crate) unsafe fn rtContextSetMissProgram(
        context: Context,
        ray_type_index: ::std::os::raw::c_uint,
        program: Program,
    ) -> Result<(), RTresult> {
        context::set_miss_program(context, ray_type_index, program)
    }

    pub(crate) unsafe fn rtContextSetPrintEnabled(
        context: Context,
        enabled: ::std::os::raw::c_int,
    ) -> Result<(), RTresult> {
        context::set_print_enabled(context, enabled)
    }

    pub(crate) unsafe fn rtContextSetPrintLaunchIndex(
        context: Context,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        z: ::std::os::raw::c_int,
    ) -> Result<(), RTresult> {
        context::set_print_launch_index(context, x, y, z)
    }

    pub(crate) unsafe fn rtContextSetRayGenerationProgram(
        context: Context,
        entry_point_index: ::std::os::raw::c_uint,
        program: Program,
    ) -> Result<(), RTresult> {
        context::set_ray_generation_program(context, entry_point_index, program)
    }

    pub(crate) unsafe fn rtContextSetRayTypeCount(
        context: Context,
        ray_type_count: c_uint,
    ) -> Result<(), RTresult> {
        context::set_ray_type_count(context, ray_type_count)
    }

    pub(crate) unsafe fn rtContextSetStackSize(
        context: Context,
        bytes: u64,
    ) -> Result<(), RTresult> {
        context::set_stack_size(context, bytes)
    }

    pub(crate) fn rtContextSetUsageReportCallback(
        context: Context,
        callback: RTusagereportcallback,
        verbosity: ::std::os::raw::c_int,
        cbdata: *mut ::std::os::raw::c_void,
    ) -> Result<(), RTresult> {
        context::set_usage_report_callback(context, callback, verbosity, cbdata)
    }

    pub(crate) unsafe fn rtContextValidate(context: Context) -> Result<(), RTresult> {
        context::validate(context)
    }

    pub(crate) unsafe fn rtDeviceGetAttribute(
        ordinal: ::std::os::raw::c_int,
        attrib: RTdeviceattribute,
        size: RTsize,
        p: *mut ::std::os::raw::c_void,
    ) -> Result<(), RTresult> {
        super::device_get_attribute(ordinal, attrib, size, p)
    }

    pub(crate) unsafe fn rtDeviceGetDeviceCount(device_count: *mut u32) -> RTresult {
        super::device_get_count(device_count)
    }

    pub(crate) unsafe fn rtGeometryCreate(
        context: Context,
        geometry: *mut Geometry,
    ) -> Result<(), RTresult> {
        geometry::create(context, geometry)
    }

    pub(crate) unsafe fn rtGeometryDeclareVariable(
        geometry: Geometry,
        name: *const ::std::os::raw::c_char,
        v: *mut Variable,
    ) -> Result<(), RTresult> {
        geometry::declare_variable(geometry, name, v)
    }

    pub(crate) unsafe fn rtGeometryDestroy(geometry: Geometry) -> Result<(), RTresult> {
        geometry::destroy(geometry)
    }

    pub(crate) unsafe fn rtGeometryGetContext(geometry: Geometry, context: *mut Context) -> Result<(), RTresult> {
        geometry::get_context(geometry, context)
    }

    pub(crate) unsafe fn rtGeometryQueryVariable(
        geometry: Geometry,
        name: *const ::std::os::raw::c_char,
        v: *mut Variable,
    ) -> Result<(), RTresult> {
        geometry::query_variable(geometry, name, v)
    }

    pub(crate) unsafe fn rtGeometrySetPrimitiveCount(
        geometry: Geometry,
        primitiveCount: ::std::os::raw::c_uint,
    ) -> Result<(), RTresult> {
        geometry::set_primitive_count(geometry, primitiveCount)
    }

    pub(crate) unsafe fn rtGeometrySetBoundingBoxProgram(
        geometry: Geometry,
        program: Program,
    ) -> Result<(), RTresult> {
        geometry::set_bounding_box_program(geometry, program)
    }

    pub(crate) unsafe fn rtGeometrySetIntersectionProgram(
        geometry: Geometry,
        program: Program,
    ) -> Result<(), RTresult> {
        geometry::set_intersection_program(geometry, program)
    }

    pub(crate) unsafe fn rtGeometryGroupCreate(
        context: Context,
        geometrygroup: *mut GeometryGroup,
    ) -> Result<(), RTresult> {
        geometry_group::create(context, geometrygroup)
    }

    pub(crate) unsafe fn rtGeometryGroupDestroy(
        geometrygroup: GeometryGroup,
    ) -> Result<(), RTresult> {
        geometry_group::destroy(geometrygroup)
    }

    pub(crate) unsafe fn rtGeometryGroupGetChildCount(
        geometrygroup: GeometryGroup,
        count: *mut ::std::os::raw::c_uint,
    ) -> Result<(), RTresult> {
        geometry_group::get_child_count(geometrygroup, count)
    }

    pub(crate) unsafe fn rtGeometryGroupGetContext(
        geometrygroup: GeometryGroup,
        context: *mut Context,
    ) -> Result<(), RTresult> {
        geometry_group::get_context(geometrygroup, context)
    }

    pub(crate) unsafe fn rtGeometryGroupSetAcceleration(
        geometrygroup: GeometryGroup,
        acceleration: Acceleration,
    ) -> Result<(), RTresult> {
        geometry_group::set_acceleration(geometrygroup, acceleration)
    }

    pub(crate) unsafe fn rtGeometryGroupSetChild(
        geometrygroup: GeometryGroup,
        index: ::std::os::raw::c_uint,
        geometryinstance: GeometryInstance,
    ) -> Result<(), RTresult> {
        geometry_group::set_child(geometrygroup, index, geometryinstance)
    }

    pub(crate) unsafe fn rtGeometryGroupSetChildCount(
        geometrygroup: GeometryGroup,
        count: ::std::os::raw::c_uint,
    ) -> Result<(), RTresult> {
        geometry_group::set_child_count(geometrygroup, count)
    }

    pub(crate) unsafe fn rtGeometryGroupSetVisibilityMask(
        geometrygroup: GeometryGroup,
        mask: RTvisibilitymask,
    ) -> Result<(), RTresult> {
        geometry_group::set_visibility_mask(geometrygroup, mask)
    }

    pub(crate) unsafe fn rtGeometryInstanceCreate(
        context: Context,
        geometryinstance: *mut GeometryInstance,
    ) -> Result<(), RTresult> {
        geometry_instance::create(context, geometryinstance)
    }

    pub(crate) unsafe fn rtGeometryInstanceDeclareVariable(
        geometryinstance: GeometryInstance,
        name: *const ::std::os::raw::c_char,
        v: *mut Variable,
    ) -> Result<(), RTresult> {
        geometry_instance::declare_variable(geometryinstance, name, v)
    }

    pub(crate) unsafe fn rtGeometryInstanceDestroy(
        geometryinstance: GeometryInstance,
    ) -> Result<(), RTresult> {
        geometry_instance::destroy(geometryinstance)
    }

    pub(crate) unsafe fn rtGeometryInstanceQueryVariable(
        geometryinstance: GeometryInstance,
        name: *const ::std::os::raw::c_char,
        v: *mut Variable,
    ) -> Result<(), RTresult> {
        geometry_instance::query_variable(geometryinstance, name, v)
    }

    pub(crate) unsafe fn rtGeometryInstanceGetContext(
        geometryinstance: GeometryInstance,
        context: *mut Context,
    ) -> Result<(), RTresult> {
        geometry_instance::get_context(geometryinstance, context)
    }

    pub(crate) unsafe fn rtGeometryInstanceGetMaterialCount(
        geometryinstance: GeometryInstance,
        count: *mut ::std::os::raw::c_uint,
    ) -> Result<(), RTresult> {
        geometry_instance::get_material_count(geometryinstance, count)
    }

    pub(crate) unsafe fn rtGeometryInstanceSetGeometry(
        geometryinstance: GeometryInstance,
        geometry: Geometry,
    ) -> Result<(), RTresult> {
        geometry_instance::set_geometry(geometryinstance, geometry)
    }

    pub(crate) unsafe fn rtGeometryInstanceSetGeometryTriangles(
        geometryinstance: GeometryInstance,
        geometrytriangles: GeometryTriangles,
    ) -> Result<(), RTresult> {
        geometry_instance::set_geometry_triangles(geometryinstance, geometrytriangles)
    }

    pub(crate) unsafe fn rtGeometryInstanceSetMaterial(
        geometryinstance: GeometryInstance,
        index: ::std::os::raw::c_uint,
        material: Material,
    ) -> Result<(), RTresult> {
        geometry_instance::set_material(geometryinstance, index, material)
    }

    pub(crate) unsafe fn rtGeometryInstanceSetMaterialCount(
        geometryinstance: GeometryInstance,
        count: ::std::os::raw::c_uint,
    ) -> Result<(), RTresult> {
        geometry_instance::set_material_count(geometryinstance, count)
    }

    pub(crate) unsafe fn rtGeometryTrianglesCreate(
        context: Context,
        geometrytriangles: *mut GeometryTriangles,
    ) -> Result<(), RTresult> {
        geometry_triangles::create(context, geometrytriangles)
    }

    pub(crate) unsafe fn rtGeometryTrianglesDeclareVariable(
        geometrytriangles: GeometryTriangles,
        name: *const ::std::os::raw::c_char,
        v: *mut Variable,
    ) -> Result<(), RTresult> {
        geometry_triangles::declare_variable(geometrytriangles, name, v)
    }

    pub(crate) unsafe fn rtGeometryTrianglesDestroy(
        geometrytriangles: GeometryTriangles,
    ) -> Result<(), RTresult> {
        geometry_triangles::destroy(geometrytriangles)
    }

    pub(crate) unsafe fn rtGeometryTrianglesGetContext(
        geometrytriangles: GeometryTriangles,
        context: *mut Context,
    ) -> Result<(), RTresult> {
        geometry_triangles::get_context(geometrytriangles, context)
    }

    pub(crate) unsafe fn rtGeometryTrianglesSetAttributeProgram(
        geometrytriangles: GeometryTriangles,
        program: Program,
    ) -> Result<(), RTresult> {
        geometry_triangles::set_attribute(geometrytriangles, program)
    }

    pub(crate) unsafe fn rtGeometryTrianglesSetBuildFlags(
        geometrytriangles: GeometryTriangles,
        _buildFlags: RTgeometrybuildflags,
    ) -> Result<(), RTresult> {
        geometry_triangles::set_build_flags(geometrytriangles, _buildFlags)
    }

    pub(crate) unsafe fn rtGeometryTrianglesSetFlagsPerMaterial(
        geometrytriangles: GeometryTriangles,
        materialIndex: ::std::os::raw::c_uint,
        flags: RTgeometryflags,
    ) -> Result<(), RTresult> {
        geometry_triangles::set_flags_per_material(geometrytriangles, materialIndex, flags)
    }

    #[allow(non_snake_case)]
    pub(crate) unsafe fn rtGeometryTrianglesSetPrimitiveCount(
        geometrytriangles: GeometryTriangles,
        triangle_count: c_uint,
    ) -> Result<(), RTresult> {
        geometry_triangles::set_primitive_count(geometrytriangles, triangle_count)
    }

    pub(crate) unsafe fn rtGeometryTrianglesSetTriangleIndices(
        geometrytriangles: GeometryTriangles,
        indexBuffer: Buffer,
        indexBufferByteOffset: u64,
        triIndicesByteStride: u64,
        triIndicesFormat: RTformat,
    ) -> Result<(), RTresult> {
        geometry_triangles::set_triangle_indices(
            geometrytriangles,
            indexBuffer,
            indexBufferByteOffset,
            triIndicesByteStride,
            triIndicesFormat,
        )
    }

    #[allow(non_snake_case)]
    pub(crate) unsafe fn rtGeometryTrianglesSetVertices(
        geometrytriangles: GeometryTriangles,
        vertexCount: ::std::os::raw::c_uint,
        vertexBuffer: Buffer,
        vertexBufferByteOffset: u64,
        vertexByteStride: u64,
        positionFormat: RTformat,
    ) -> Result<(), RTresult> {
        geometry_triangles::set_vertices(
            geometrytriangles,
            vertexCount,
            vertexBuffer,
            vertexBufferByteOffset,
            vertexByteStride,
            positionFormat,
        )
    }

    pub(crate) unsafe fn rtGeometryTrianglesQueryVariable(
        geometrytriangles: GeometryTriangles,
        name: *const ::std::os::raw::c_char,
        v: *mut Variable,
    ) -> Result<(), RTresult> {
        geometry_triangles::query_variable(geometrytriangles, name, v)
    }

    pub(crate) unsafe fn rtGeometryTrianglesValidate(
        geometrytriangles: GeometryTriangles,
    ) -> Result<(), RTresult> {
        geometry_triangles::validate(geometrytriangles)
    }

    pub(crate) unsafe fn rtGetVersion(version: *mut u32) -> RTresult {
        super::get_version(version)
    }

    pub(crate) unsafe fn rtGlobalGetAttribute(
        attrib: RTglobalattribute,
        size: RTsize,
        p: *mut ::std::os::raw::c_void,
    ) -> RTresult {
        super::global_get_attribute(attrib, size, p)
    }

    pub(crate) unsafe fn rtGlobalSetAttribute(
        attrib: RTglobalattribute,
        size: RTsize,
        p: *const ::std::os::raw::c_void,
    ) -> RTresult {
        super::global_set_attribute(attrib, size, p)
    }

    pub(crate) unsafe fn rtGroupCreate(
        context: Context,
        group: *mut Group,
    ) -> Result<(), RTresult> {
        group::create(context, group)
    }

    pub(crate) unsafe fn rtGroupDestroy(group: Group) -> Result<(), RTresult> {
        group::destroy(group)
    }

    pub(crate) unsafe fn rtGroupGetAcceleration(
        group: Group,
        acceleration: *mut Acceleration,
    ) -> Result<(), RTresult> {
        group::get_acceleration(group, acceleration)
    }

    pub(crate) unsafe fn rtGroupGetChild(
        group: Group,
        index: ::std::os::raw::c_uint,
        child: *mut UntypedObject,
    ) -> Result<(), RTresult> {
        group::get_child(group, index, child)
    }

    pub(crate) unsafe fn rtGroupGetChildCount(
        group: Group,
        count: *mut ::std::os::raw::c_uint,
    ) -> Result<(), RTresult> {
        group::get_child_count(group, count)
    }

    pub(crate) unsafe fn rtGroupGetContext(
        group: Group,
        context: *mut Context,
    ) -> Result<(), RTresult> {
        group::get_context(group, context)
    }

    pub(crate) unsafe fn rtGroupSetAcceleration(
        group: Group,
        acceleration: Acceleration,
    ) -> Result<(), RTresult> {
        group::set_acceleration(group, acceleration)
    }

    pub(crate) unsafe fn rtGroupSetChild(
        group: Group,
        index: ::std::os::raw::c_uint,
        child: UntypedObject,
    ) -> Result<(), RTresult> {
        group::set_child(group, index, child)
    }

    pub(crate) unsafe fn rtGroupSetChildCount(
        group: Group,
        count: ::std::os::raw::c_uint,
    ) -> Result<(), RTresult> {
        group::set_child_count(group, count)
    }

    #[allow(non_snake_case)]
    pub(crate) unsafe fn rtMaterialCreate(
        context: Context,
        material: *mut Material,
    ) -> Result<(), RTresult> {
        material::create(context, material)
    }

    pub(crate) unsafe fn rtMaterialDeclareVariable(
        material: Material,
        name: *const ::std::os::raw::c_char,
        v: *mut Variable,
    ) -> Result<(), RTresult> {
        material::declare_variable(material, name, v)
    }

    pub(crate) unsafe fn rtMaterialDestroy(material: Material) -> Result<(), RTresult> {
        material::destroy(material)
    }

    pub(crate) unsafe fn rtMaterialGetContext(
        material: Material,
        context: *mut Context,
    ) -> Result<(), RTresult> {
        material::get_context(material, context)
    }

    pub(crate) unsafe fn rtMaterialQueryVariable(
        material: Material,
        name: *const ::std::os::raw::c_char,
        v: *mut Variable,
    ) -> Result<(), RTresult> {
        material::query_variable(material, name, v)
    }

    pub(crate) unsafe fn rtMaterialSetAnyHitProgram(
        material: Material,
        rayTypeIndex: ::std::os::raw::c_uint,
        program: Program,
    ) -> Result<(), RTresult> {
        material::set_any_hit_program(material, rayTypeIndex, program)
    }

    pub(crate) unsafe fn rtMaterialSetClosestHitProgram(
        material: Material,
        rayTypeIndex: ::std::os::raw::c_uint,
        program: Program,
    ) -> Result<(), RTresult> {
        material::set_closest_hit_program(material, rayTypeIndex, program)
    }

    pub(crate) unsafe fn rtProgramCreateFromProgram(
        context: Context,
        program_in: Program,
        program_out: *mut Program,
    ) -> Result<(), RTresult> {
        program::create_from_program(context, program_in, program_out)
    }

    pub(crate) unsafe fn rtProgramCreateFromPTXFile(
        context: Context,
        filename: *const ::std::os::raw::c_char,
        program_name: *const ::std::os::raw::c_char,
        program: *mut Program,
    ) -> Result<(), RTresult> {
        program::create_from_ptx_file(context, filename, program_name, program)
    }

    pub(crate) unsafe fn rtProgramCreateFromPTXString(
        context: Context,
        ptx: *const ::std::os::raw::c_char,
        program_name: *const ::std::os::raw::c_char,
        program: *mut Program,
    ) -> Result<(), RTresult> {
        program::create_from_ptx_string(context, ptx, program_name, program)
    }

    #[allow(non_snake_case)]
    pub(crate) unsafe fn rtProgramDeclareVariable(
        program: Program,
        name: *const ::std::os::raw::c_char,
        v: *mut Variable,
    ) -> Result<(), RTresult> {
        program::declare_variable(program, name, v)
    }

    pub(crate) unsafe fn rtProgramDestroy(program: Program) -> Result<(), RTresult> {
        program::destroy(program)
    }

    pub(crate) unsafe fn rtProgramGetContext(program: Program, context: *mut Context) -> Result<(), RTresult> {
        program::get_context(program, context)
    }

    pub(crate) unsafe fn rtProgramGetId(
        program: Program,
        program_id: *mut ::std::os::raw::c_int,
    ) -> Result<(), RTresult> {
        program::get_id(program, program_id)
    }

    pub(crate) unsafe fn rtProgramQueryVariable(
        program: Program,
        name: *const ::std::os::raw::c_char,
        v: *mut Variable,
    ) -> Result<(), RTresult> {
        program::query_variable(program, name, v)
    }

    pub(crate) unsafe fn rtProgramValidate(program: Program) -> Result<(), RTresult> {
        program::validate(program)
    }

    pub(crate) unsafe fn rtTextureSamplerCreate(
        context: Context,
        texturesampler: *mut TextureSampler,
    ) -> Result<(), RTresult> {
        texture_sampler::create(context, texturesampler)
    }

    pub(crate) unsafe fn rtTextureSamplerDestroy(
        texturesampler: TextureSampler,
    ) -> Result<(), RTresult> {
        texture_sampler::destroy(texturesampler)
    }

    pub(crate) unsafe fn rtTextureSamplerGetBuffer(
        texturesampler: TextureSampler,
        deprecated0: ::std::os::raw::c_uint,
        deprecated1: ::std::os::raw::c_uint,
        buffer: *mut Buffer,
    ) -> Result<(), RTresult> {
        texture_sampler::get_buffer(texturesampler, deprecated0, deprecated1, buffer)
    }

    pub(crate) unsafe fn rtTextureSamplerGetContext(
        texturesampler: TextureSampler,
        context: *mut Context,
    ) -> Result<(), RTresult> {
        texture_sampler::get_context(texturesampler, context)
    }

    pub(crate) unsafe fn rtTextureSamplerGetId(
        texturesampler: TextureSampler,
        textureId: *mut ::std::os::raw::c_int,
    ) -> Result<(), RTresult> {
        texture_sampler::get_id(texturesampler, textureId)
    }

    pub(crate) unsafe fn rtTextureSamplerSetArraySize(
        texturesampler: TextureSampler,
        textureCount: ::std::os::raw::c_uint,
    ) -> Result<(), RTresult> {
        texture_sampler::set_array_size(texturesampler, textureCount)
    }

    pub(crate) unsafe fn rtTextureSamplerSetBuffer(
        texturesampler: TextureSampler,
        _deprecated0: ::std::os::raw::c_uint,
        _deprecated1: ::std::os::raw::c_uint,
        buffer: Buffer,
    ) -> Result<(), RTresult> {
        texture_sampler::set_buffer(texturesampler, buffer)
    }

    pub(crate) unsafe fn rtTextureSamplerSetFilteringModes(
        texturesampler: TextureSampler,
        minification: RTfiltermode,
        magnification: RTfiltermode,
        mipmapping: RTfiltermode,
    ) -> Result<(), RTresult> {
        texture_sampler::set_filtering_modes(
            texturesampler,
            minification,
            magnification,
            mipmapping,
        )
    }

    pub(crate) unsafe fn rtTextureSamplerSetIndexingMode(
        texturesampler: TextureSampler,
        indexmode: RTtextureindexmode,
    ) -> Result<(), RTresult> {
        texture_sampler::set_indexing_mode(texturesampler, indexmode)
    }

    pub(crate) unsafe fn rtTextureSamplerSetMaxAnisotropy(
        texturesampler: TextureSampler,
        value: f32,
    ) -> Result<(), RTresult> {
        texture_sampler::set_max_anisotropy(texturesampler, value)
    }

    pub(crate) unsafe fn rtTextureSamplerSetMipLevelCount(
        texturesampler: TextureSampler,
        mipLevelCount: ::std::os::raw::c_uint,
    ) -> Result<(), RTresult> {
        texture_sampler::set_mip_level_count(texturesampler, mipLevelCount)
    }

    pub(crate) unsafe fn rtTextureSamplerSetReadMode(
        texturesampler: TextureSampler,
        readmode: RTtexturereadmode,
    ) -> Result<(), RTresult> {
        texture_sampler::set_read_mode(texturesampler, readmode)
    }

    pub(crate) unsafe fn rtTextureSamplerSetWrapMode(
        texturesampler: TextureSampler,
        dimension: ::std::os::raw::c_uint,
        wrapmode: RTwrapmode,
    ) -> Result<(), RTresult> {
        texture_sampler::set_wrap_mode(texturesampler, dimension, wrapmode)
    }

    pub(crate) unsafe fn rtTransformCreate(
        context: Context,
        transform: *mut Transform,
    ) -> Result<(), RTresult> {
        transform::create(context, transform)
    }

    pub(crate) unsafe fn rtTransformDestroy(transform: Transform) -> Result<(), RTresult> {
        transform::destroy(transform)
    }

    pub(crate) unsafe fn rtTransformGetContext(transform: Transform, context: *mut Context) -> Result<(), RTresult> {
        transform::get_context(transform, context)
    }

    pub(crate) unsafe fn rtTransformGetMotionKeyCount(
        transform: Transform,
        n: *mut ::std::os::raw::c_uint,
    ) -> Result<(), RTresult> {
        transform::get_motion_key_count(transform, n)
    }

    pub(crate) unsafe fn rtTransformSetChild(
        transform: Transform,
        child: UntypedObject,
    ) -> Result<(), RTresult> {
        transform::set_child(transform, child)
    }

    pub(crate) unsafe fn rtTransformSetMatrix(
        transform: Transform,
        transpose: i32,
        matrix: *const f32,
        inverse_matrix: *const f32,
    ) -> Result<(), RTresult> {
        transform::set_matrix(transform, transpose, matrix, inverse_matrix)
    }

    pub(crate) unsafe fn rtVariableSetObject(
        v: Variable,
        object: UntypedObject,
    ) -> Result<(), RTresult> {
        variable::set_object(v, object)
    }

    pub(crate) unsafe fn rtVariableGetObject(
        v: Variable,
        object: *mut UntypedObject,
    ) -> Result<(), RTresult> {
        variable::get_object(v, object)
    }

    pub(crate) unsafe fn rtVariableSet1f(v: Variable, f1: f32) -> Result<(), RTresult> {
        variable::set_1f(v, f1)
    }

    pub(crate) unsafe fn rtVariableSet1i(v: Variable, i1: c_int) -> Result<(), RTresult> {
        variable::set_1i(v, i1)
    }

    pub(crate) unsafe fn rtVariableSet1ui(v: Variable, u1: u32) -> Result<(), RTresult> {
        variable::set_1ui(v, u1)
    }

    pub(crate) unsafe fn rtVariableSet1ull(
        v: Variable,
        ull1: ::std::os::raw::c_ulonglong,
    ) -> Result<(), RTresult> {
        variable::set_1ull(v, ull1)
    }

    #[allow(non_snake_case)]
    pub(crate) unsafe fn rtVariableSet3f(
        v: Variable,
        f1: f32,
        f2: f32,
        f3: f32,
    ) -> Result<(), RTresult> {
        variable::set_3f(v, f1, f2, f3)
    }

    pub(crate) unsafe fn rtVariableSet3fv(v: Variable, f: *const f32) -> Result<(), RTresult> {
        variable::set_3fv(v, f)
    }

    pub(crate) unsafe fn rtVariableSet4f(
        v: Variable,
        f1: f32,
        f2: f32,
        f3: f32,
        f4: f32,
    ) -> Result<(), RTresult> {
        variable::set_4f(v, f1, f2, f3, f4)
    }

    pub(crate) unsafe fn rtVariableSet4fv(v: Variable, f: *const f32) -> Result<(), RTresult> {
        variable::set_4fv(v, f)
    }

    pub(crate) unsafe fn rtVariableSetUserData(
        v: Variable,
        size: RTsize,
        ptr: *const ::std::os::raw::c_void,
    ) -> Result<(), RTresult> {
        variable::set_user_data(v, size, ptr)
    }
}

pub(crate) unsafe fn get_version(version: *mut u32) -> RTresult {
    *version = 60600;
    RTresult::RT_SUCCESS
}

pub(crate) unsafe fn device_get_count(device_count: *mut u32) -> RTresult {
    *device_count = 1;
    RTresult::RT_SUCCESS
}

pub(crate) unsafe fn device_get_attribute(
    ordinal: i32,
    attrib: RTdeviceattribute,
    size: u64,
    p: *mut c_void,
) -> Result<(), RTresult> {
    Ok(match attrib {
        RTdeviceattribute::RT_DEVICE_ATTRIBUTE_CLOCK_RATE => {
            hip! { hipDeviceGetAttribute(
                p as _,
                hipDeviceAttribute_t::hipDeviceAttributeClockRate,
                ordinal,
            ), RT_ERROR_UNKNOWN }
        }
        RTdeviceattribute::RT_DEVICE_ATTRIBUTE_NAME => {
            let dev_name = "Graphics Device";
            let strlen = dev_name.len().min(size as usize - 1);
            ptr::copy_nonoverlapping(dev_name.as_ptr(), p as _, strlen);
            *(p as *mut u8).add(strlen) = 0;
        }
        RTdeviceattribute::RT_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY => {
            *(p as *mut [u32; 2]) = [8u32, 6u32];
        }
        RTdeviceattribute::RT_DEVICE_ATTRIBUTE_TOTAL_MEMORY => {
            hip! { hipDeviceTotalMem(
                p as _,
                ordinal,
            ), RT_ERROR_UNKNOWN }
        }
        RTdeviceattribute::RT_DEVICE_ATTRIBUTE_CUDA_DEVICE_ORDINAL => *(p as *mut i32) = ordinal,
        _ => return Err(definitions::unimplemented()),
    })
}

pub(crate) unsafe fn global_set_attribute(
    attrib: RTglobalattribute,
    _size: u64,
    _p: *const c_void,
) -> RTresult {
    match attrib {
        RTglobalattribute::RT_GLOBAL_ATTRIBUTE_ENABLE_RTX => RTresult::RT_SUCCESS,
        // TODO: reverse
        RTglobalattribute(268435457) | optix_types::RTglobalattribute(4096) => {
            return RTresult::RT_ERROR_NOT_SUPPORTED
        }
        _ => definitions::unimplemented(),
    }
}

fn global_get_attribute(_attrib: RTglobalattribute, _size: u64, _p: *mut c_void) -> RTresult {
    RTresult::RT_ERROR_NOT_SUPPORTED
}
