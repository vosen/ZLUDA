// Generated automatically by zluda_bindgen
// DO NOT EDIT MANUALLY
#![allow(warnings)]
pub const DRM_COMMAND_BASE: u32 = 64;
pub const DRM_ASAHI_MAX_CLUSTERS: u32 = 64;
pub const DRM_ASAHI_BARRIER_NONE: u32 = 65535;
pub type __u8 = ::core::ffi::c_uchar;
pub type __u16 = ::core::ffi::c_ushort;
pub type __u32 = ::core::ffi::c_uint;
pub type __u64 = ::core::ffi::c_ulonglong;
/// @DRM_ASAHI_GET_PARAMS: Query device properties.
pub const drm_asahi_ioctl_id_DRM_ASAHI_GET_PARAMS: drm_asahi_ioctl_id = 0;
/// @DRM_ASAHI_GET_TIME: Query device time.
pub const drm_asahi_ioctl_id_DRM_ASAHI_GET_TIME: drm_asahi_ioctl_id = 1;
/// @DRM_ASAHI_VM_CREATE: Create a GPU VM address space.
pub const drm_asahi_ioctl_id_DRM_ASAHI_VM_CREATE: drm_asahi_ioctl_id = 2;
/// @DRM_ASAHI_VM_DESTROY: Destroy a VM.
pub const drm_asahi_ioctl_id_DRM_ASAHI_VM_DESTROY: drm_asahi_ioctl_id = 3;
/// @DRM_ASAHI_VM_BIND: Bind/unbind memory to a VM.
pub const drm_asahi_ioctl_id_DRM_ASAHI_VM_BIND: drm_asahi_ioctl_id = 4;
/// @DRM_ASAHI_GEM_CREATE: Create a buffer object.
pub const drm_asahi_ioctl_id_DRM_ASAHI_GEM_CREATE: drm_asahi_ioctl_id = 5;
/** @DRM_ASAHI_GEM_MMAP_OFFSET: Get offset to pass to mmap() to map a
 given GEM handle.*/
pub const drm_asahi_ioctl_id_DRM_ASAHI_GEM_MMAP_OFFSET: drm_asahi_ioctl_id = 6;
/// @DRM_ASAHI_GEM_BIND_OBJECT: Bind memory as a special object
pub const drm_asahi_ioctl_id_DRM_ASAHI_GEM_BIND_OBJECT: drm_asahi_ioctl_id = 7;
/// @DRM_ASAHI_QUEUE_CREATE: Create a scheduling queue.
pub const drm_asahi_ioctl_id_DRM_ASAHI_QUEUE_CREATE: drm_asahi_ioctl_id = 8;
/// @DRM_ASAHI_QUEUE_DESTROY: Destroy a scheduling queue.
pub const drm_asahi_ioctl_id_DRM_ASAHI_QUEUE_DESTROY: drm_asahi_ioctl_id = 9;
/// @DRM_ASAHI_SUBMIT: Submit commands to a queue.
pub const drm_asahi_ioctl_id_DRM_ASAHI_SUBMIT: drm_asahi_ioctl_id = 10;
/** enum drm_asahi_ioctl_id - IOCTL IDs

 Place new ioctls at the end, don't re-order, don't replace or remove entries.

 These IDs are not meant to be used directly. Use the DRM_IOCTL_ASAHI_xxx
 definitions instead.*/
pub type drm_asahi_ioctl_id = ::core::ffi::c_uint;
/** struct drm_asahi_params_global - Global parameters.

 This struct may be queried by drm_asahi_get_params.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_asahi_params_global {
    /// @features: Feature bits from drm_asahi_feature
    pub features: __u64,
    /// @gpu_generation: GPU generation, e.g. 13 for G13G
    pub gpu_generation: __u32,
    /// @gpu_variant: GPU variant as a character, e.g. 'C' for G13C
    pub gpu_variant: __u32,
    /** @gpu_revision: GPU revision in BCD, e.g. 0x00 for 'A0' or
 0x21 for 'C1'*/
    pub gpu_revision: __u32,
    /// @chip_id: Chip ID in BCD, e.g. 0x8103 for T8103
    pub chip_id: __u32,
    /// @num_dies: Number of dies in the SoC
    pub num_dies: __u32,
    /// @num_clusters_total: Number of GPU clusters (across all dies)
    pub num_clusters_total: __u32,
    /** @num_cores_per_cluster: Number of logical cores per cluster
 (including inactive/nonexistent)*/
    pub num_cores_per_cluster: __u32,
    /// @max_frequency_khz: Maximum GPU core clock frequency
    pub max_frequency_khz: __u32,
    /// @core_masks: Bitmask of present/enabled cores per cluster
    pub core_masks: [__u64; 64usize],
    /** @vm_start: VM range start VMA. Together with @vm_end, this defines
 the window of valid GPU VAs. Userspace is expected to subdivide VAs
 out of this window.

 This window contains all virtual addresses that userspace needs to
 know about. There may be kernel-internal GPU VAs outside this range,
 but that detail is not relevant here.*/
    pub vm_start: __u64,
    /// @vm_end: VM range end VMA
    pub vm_end: __u64,
    /** @vm_kernel_min_size: Minimum kernel VMA window size.

 When creating a VM, userspace is required to carve out a section of
 virtual addresses (within the range given by @vm_start and
 @vm_end). The kernel will allocate various internal structures
 within the specified VA range.

 Allowing userspace to choose the VA range for the kernel, rather than
 the kernel reserving VAs and requiring userspace to cope, can assist
 in implementing SVM.*/
    pub vm_kernel_min_size: __u64,
    /** @max_commands_per_submission: Maximum number of supported commands
 per submission. This mirrors firmware limits. Userspace must split up
 larger command buffers, which may require inserting additional
 synchronization.*/
    pub max_commands_per_submission: __u32,
    /** @max_attachments: Maximum number of drm_asahi_attachment's per
 command*/
    pub max_attachments: __u32,
    /** @command_timestamp_frequency_hz: Timebase frequency for timestamps
 written during command execution, specified via drm_asahi_timestamp
 structures. As this rate is controlled by the firmware, it is a
 queryable parameter.

 Userspace must divide by this frequency to convert timestamps to
 seconds, rather than hardcoding a particular firmware's rate.*/
    pub command_timestamp_frequency_hz: __u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of drm_asahi_params_global",
    ][::core::mem::size_of::<drm_asahi_params_global>() - 592usize];
    [
        "Alignment of drm_asahi_params_global",
    ][::core::mem::align_of::<drm_asahi_params_global>() - 8usize];
    [
        "Offset of field: drm_asahi_params_global::features",
    ][::core::mem::offset_of!(drm_asahi_params_global, features) - 0usize];
    [
        "Offset of field: drm_asahi_params_global::gpu_generation",
    ][::core::mem::offset_of!(drm_asahi_params_global, gpu_generation) - 8usize];
    [
        "Offset of field: drm_asahi_params_global::gpu_variant",
    ][::core::mem::offset_of!(drm_asahi_params_global, gpu_variant) - 12usize];
    [
        "Offset of field: drm_asahi_params_global::gpu_revision",
    ][::core::mem::offset_of!(drm_asahi_params_global, gpu_revision) - 16usize];
    [
        "Offset of field: drm_asahi_params_global::chip_id",
    ][::core::mem::offset_of!(drm_asahi_params_global, chip_id) - 20usize];
    [
        "Offset of field: drm_asahi_params_global::num_dies",
    ][::core::mem::offset_of!(drm_asahi_params_global, num_dies) - 24usize];
    [
        "Offset of field: drm_asahi_params_global::num_clusters_total",
    ][::core::mem::offset_of!(drm_asahi_params_global, num_clusters_total) - 28usize];
    [
        "Offset of field: drm_asahi_params_global::num_cores_per_cluster",
    ][::core::mem::offset_of!(drm_asahi_params_global, num_cores_per_cluster) - 32usize];
    [
        "Offset of field: drm_asahi_params_global::max_frequency_khz",
    ][::core::mem::offset_of!(drm_asahi_params_global, max_frequency_khz) - 36usize];
    [
        "Offset of field: drm_asahi_params_global::core_masks",
    ][::core::mem::offset_of!(drm_asahi_params_global, core_masks) - 40usize];
    [
        "Offset of field: drm_asahi_params_global::vm_start",
    ][::core::mem::offset_of!(drm_asahi_params_global, vm_start) - 552usize];
    [
        "Offset of field: drm_asahi_params_global::vm_end",
    ][::core::mem::offset_of!(drm_asahi_params_global, vm_end) - 560usize];
    [
        "Offset of field: drm_asahi_params_global::vm_kernel_min_size",
    ][::core::mem::offset_of!(drm_asahi_params_global, vm_kernel_min_size) - 568usize];
    [
        "Offset of field: drm_asahi_params_global::max_commands_per_submission",
    ][::core::mem::offset_of!(drm_asahi_params_global, max_commands_per_submission)
        - 576usize];
    [
        "Offset of field: drm_asahi_params_global::max_attachments",
    ][::core::mem::offset_of!(drm_asahi_params_global, max_attachments) - 580usize];
    [
        "Offset of field: drm_asahi_params_global::command_timestamp_frequency_hz",
    ][::core::mem::offset_of!(drm_asahi_params_global, command_timestamp_frequency_hz)
        - 584usize];
};
/** @DRM_ASAHI_FEATURE_SOFT_FAULTS: GPU has "soft fault" enabled. Shader
 loads of unmapped memory will return zero. Shader stores to unmapped
 memory will be silently discarded. Note that only shader load/store
 is affected. Other hardware units are not affected, notably including
 texture sampling.

 Soft fault is set when initializing the GPU and cannot be runtime
 toggled. Therefore, it is exposed as a feature bit and not a
 userspace-settable flag on the VM. When soft fault is enabled,
 userspace can speculate memory accesses more aggressively.*/
pub const drm_asahi_feature_DRM_ASAHI_FEATURE_SOFT_FAULTS: drm_asahi_feature = 1;
/** enum drm_asahi_feature - Feature bits

 This covers only features that userspace cannot infer from the architecture
 version. Most features don't need to be here.*/
pub type drm_asahi_feature = ::core::ffi::c_uint;
/// struct drm_asahi_get_params - Arguments passed to DRM_IOCTL_ASAHI_GET_PARAMS
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_asahi_get_params {
    /// @param_group: Parameter group to fetch (MBZ)
    pub param_group: __u32,
    /// @pad: MBZ
    pub pad: __u32,
    /// @pointer: User pointer to write parameter struct
    pub pointer: __u64,
    /** @size: Size of the user buffer. In case of older userspace, this may
 be less than sizeof(struct drm_asahi_params_global). The kernel will
 not write past the length specified here, allowing extensibility.*/
    pub size: __u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of drm_asahi_get_params",
    ][::core::mem::size_of::<drm_asahi_get_params>() - 24usize];
    [
        "Alignment of drm_asahi_get_params",
    ][::core::mem::align_of::<drm_asahi_get_params>() - 8usize];
    [
        "Offset of field: drm_asahi_get_params::param_group",
    ][::core::mem::offset_of!(drm_asahi_get_params, param_group) - 0usize];
    [
        "Offset of field: drm_asahi_get_params::pad",
    ][::core::mem::offset_of!(drm_asahi_get_params, pad) - 4usize];
    [
        "Offset of field: drm_asahi_get_params::pointer",
    ][::core::mem::offset_of!(drm_asahi_get_params, pointer) - 8usize];
    [
        "Offset of field: drm_asahi_get_params::size",
    ][::core::mem::offset_of!(drm_asahi_get_params, size) - 16usize];
};
/// struct drm_asahi_vm_create - Arguments passed to DRM_IOCTL_ASAHI_VM_CREATE
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_asahi_vm_create {
    /** @kernel_start: Start of the kernel-reserved address range. See
 drm_asahi_params_global::vm_kernel_min_size.

 Both @kernel_start and @kernel_end must be within the range of
 valid VAs given by drm_asahi_params_global::vm_start and
 drm_asahi_params_global::vm_end. The size of the kernel range
 (@kernel_end - @kernel_start) must be at least
 drm_asahi_params_global::vm_kernel_min_size.

 Userspace must not bind any memory on this VM into this reserved
 range, it is for kernel use only.*/
    pub kernel_start: __u64,
    /** @kernel_end: End of the kernel-reserved address range. See
 @kernel_start.*/
    pub kernel_end: __u64,
    /// @vm_id: Returned VM ID
    pub vm_id: __u32,
    /// @pad: MBZ
    pub pad: __u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of drm_asahi_vm_create",
    ][::core::mem::size_of::<drm_asahi_vm_create>() - 24usize];
    [
        "Alignment of drm_asahi_vm_create",
    ][::core::mem::align_of::<drm_asahi_vm_create>() - 8usize];
    [
        "Offset of field: drm_asahi_vm_create::kernel_start",
    ][::core::mem::offset_of!(drm_asahi_vm_create, kernel_start) - 0usize];
    [
        "Offset of field: drm_asahi_vm_create::kernel_end",
    ][::core::mem::offset_of!(drm_asahi_vm_create, kernel_end) - 8usize];
    [
        "Offset of field: drm_asahi_vm_create::vm_id",
    ][::core::mem::offset_of!(drm_asahi_vm_create, vm_id) - 16usize];
    [
        "Offset of field: drm_asahi_vm_create::pad",
    ][::core::mem::offset_of!(drm_asahi_vm_create, pad) - 20usize];
};
/// struct drm_asahi_vm_destroy - Arguments passed to DRM_IOCTL_ASAHI_VM_DESTROY
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_asahi_vm_destroy {
    /// @vm_id: VM ID to be destroyed
    pub vm_id: __u32,
    /// @pad: MBZ
    pub pad: __u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of drm_asahi_vm_destroy",
    ][::core::mem::size_of::<drm_asahi_vm_destroy>() - 8usize];
    [
        "Alignment of drm_asahi_vm_destroy",
    ][::core::mem::align_of::<drm_asahi_vm_destroy>() - 4usize];
    [
        "Offset of field: drm_asahi_vm_destroy::vm_id",
    ][::core::mem::offset_of!(drm_asahi_vm_destroy, vm_id) - 0usize];
    [
        "Offset of field: drm_asahi_vm_destroy::pad",
    ][::core::mem::offset_of!(drm_asahi_vm_destroy, pad) - 4usize];
};
/** @DRM_ASAHI_GEM_WRITEBACK: BO should be CPU-mapped as writeback.

 Map as writeback instead of write-combine. This optimizes for CPU
 reads.*/
pub const drm_asahi_gem_flags_DRM_ASAHI_GEM_WRITEBACK: drm_asahi_gem_flags = 1;
/// @DRM_ASAHI_GEM_VM_PRIVATE: BO is private to this GPU VM (no exports).
pub const drm_asahi_gem_flags_DRM_ASAHI_GEM_VM_PRIVATE: drm_asahi_gem_flags = 2;
/// enum drm_asahi_gem_flags - Flags for GEM creation
pub type drm_asahi_gem_flags = ::core::ffi::c_uint;
/// struct drm_asahi_gem_create - Arguments passed to DRM_IOCTL_ASAHI_GEM_CREATE
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_asahi_gem_create {
    /// @size: Size of the BO
    pub size: __u64,
    /// @flags: Combination of drm_asahi_gem_flags flags.
    pub flags: __u32,
    /// @vm_id: VM ID to assign to the BO, if DRM_ASAHI_GEM_VM_PRIVATE is set
    pub vm_id: __u32,
    /// @handle: Returned GEM handle for the BO
    pub handle: __u32,
    /// @pad: MBZ
    pub pad: __u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of drm_asahi_gem_create",
    ][::core::mem::size_of::<drm_asahi_gem_create>() - 24usize];
    [
        "Alignment of drm_asahi_gem_create",
    ][::core::mem::align_of::<drm_asahi_gem_create>() - 8usize];
    [
        "Offset of field: drm_asahi_gem_create::size",
    ][::core::mem::offset_of!(drm_asahi_gem_create, size) - 0usize];
    [
        "Offset of field: drm_asahi_gem_create::flags",
    ][::core::mem::offset_of!(drm_asahi_gem_create, flags) - 8usize];
    [
        "Offset of field: drm_asahi_gem_create::vm_id",
    ][::core::mem::offset_of!(drm_asahi_gem_create, vm_id) - 12usize];
    [
        "Offset of field: drm_asahi_gem_create::handle",
    ][::core::mem::offset_of!(drm_asahi_gem_create, handle) - 16usize];
    [
        "Offset of field: drm_asahi_gem_create::pad",
    ][::core::mem::offset_of!(drm_asahi_gem_create, pad) - 20usize];
};
/** struct drm_asahi_gem_mmap_offset - Arguments passed to
 DRM_IOCTL_ASAHI_GEM_MMAP_OFFSET*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_asahi_gem_mmap_offset {
    /// @handle: Handle for the object being mapped.
    pub handle: __u32,
    /// @flags: Must be zero
    pub flags: __u32,
    /// @offset: The fake offset to use for subsequent mmap call
    pub offset: __u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of drm_asahi_gem_mmap_offset",
    ][::core::mem::size_of::<drm_asahi_gem_mmap_offset>() - 16usize];
    [
        "Alignment of drm_asahi_gem_mmap_offset",
    ][::core::mem::align_of::<drm_asahi_gem_mmap_offset>() - 8usize];
    [
        "Offset of field: drm_asahi_gem_mmap_offset::handle",
    ][::core::mem::offset_of!(drm_asahi_gem_mmap_offset, handle) - 0usize];
    [
        "Offset of field: drm_asahi_gem_mmap_offset::flags",
    ][::core::mem::offset_of!(drm_asahi_gem_mmap_offset, flags) - 4usize];
    [
        "Offset of field: drm_asahi_gem_mmap_offset::offset",
    ][::core::mem::offset_of!(drm_asahi_gem_mmap_offset, offset) - 8usize];
};
/** @DRM_ASAHI_BIND_UNBIND: Instead of binding a GEM object to the range,
 simply unbind the GPU VMA range.*/
pub const drm_asahi_bind_flags_DRM_ASAHI_BIND_UNBIND: drm_asahi_bind_flags = 1;
/// @DRM_ASAHI_BIND_READ: Map BO with GPU read permission
pub const drm_asahi_bind_flags_DRM_ASAHI_BIND_READ: drm_asahi_bind_flags = 2;
/// @DRM_ASAHI_BIND_WRITE: Map BO with GPU write permission
pub const drm_asahi_bind_flags_DRM_ASAHI_BIND_WRITE: drm_asahi_bind_flags = 4;
/** @DRM_ASAHI_BIND_SINGLE_PAGE: Map a single page of the BO repeatedly
 across the VA range.

 This is useful to fill a VA range with scratch pages or zero pages.
 It is intended as a mechanism to accelerate sparse.*/
pub const drm_asahi_bind_flags_DRM_ASAHI_BIND_SINGLE_PAGE: drm_asahi_bind_flags = 8;
/// enum drm_asahi_bind_flags - Flags for GEM binding
pub type drm_asahi_bind_flags = ::core::ffi::c_uint;
/// struct drm_asahi_gem_bind_op - Description of a single GEM bind operation.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_asahi_gem_bind_op {
    /// @flags: Combination of drm_asahi_bind_flags flags.
    pub flags: __u32,
    /// @handle: GEM object to bind (except for UNBIND)
    pub handle: __u32,
    /** @offset: Offset into the object (except for UNBIND).

 For a regular bind, this is the beginning of the region of the GEM
 object to bind.

 For a single-page bind, this is the offset to the single page that
 will be repeatedly bound.

 Must be page-size aligned.*/
    pub offset: __u64,
    /** @range: Number of bytes to bind/unbind to @addr.

 Must be page-size aligned.*/
    pub range: __u64,
    /** @addr: Address to bind to.

 Must be page-size aligned.*/
    pub addr: __u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of drm_asahi_gem_bind_op",
    ][::core::mem::size_of::<drm_asahi_gem_bind_op>() - 32usize];
    [
        "Alignment of drm_asahi_gem_bind_op",
    ][::core::mem::align_of::<drm_asahi_gem_bind_op>() - 8usize];
    [
        "Offset of field: drm_asahi_gem_bind_op::flags",
    ][::core::mem::offset_of!(drm_asahi_gem_bind_op, flags) - 0usize];
    [
        "Offset of field: drm_asahi_gem_bind_op::handle",
    ][::core::mem::offset_of!(drm_asahi_gem_bind_op, handle) - 4usize];
    [
        "Offset of field: drm_asahi_gem_bind_op::offset",
    ][::core::mem::offset_of!(drm_asahi_gem_bind_op, offset) - 8usize];
    [
        "Offset of field: drm_asahi_gem_bind_op::range",
    ][::core::mem::offset_of!(drm_asahi_gem_bind_op, range) - 16usize];
    [
        "Offset of field: drm_asahi_gem_bind_op::addr",
    ][::core::mem::offset_of!(drm_asahi_gem_bind_op, addr) - 24usize];
};
/** struct drm_asahi_vm_bind - Arguments passed to
 DRM_IOCTL_ASAHI_VM_BIND*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_asahi_vm_bind {
    /// @vm_id: The ID of the VM to bind to
    pub vm_id: __u32,
    /// @num_binds: number of binds in this IOCTL.
    pub num_binds: __u32,
    /** @stride: Stride in bytes between consecutive binds. This allows
 extensibility of drm_asahi_gem_bind_op.*/
    pub stride: __u32,
    /// @pad: MBZ
    pub pad: __u32,
    /** @userptr: User pointer to an array of @num_binds structures of type
 @drm_asahi_gem_bind_op and size @stride bytes.*/
    pub userptr: __u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of drm_asahi_vm_bind"][::core::mem::size_of::<drm_asahi_vm_bind>() - 24usize];
    [
        "Alignment of drm_asahi_vm_bind",
    ][::core::mem::align_of::<drm_asahi_vm_bind>() - 8usize];
    [
        "Offset of field: drm_asahi_vm_bind::vm_id",
    ][::core::mem::offset_of!(drm_asahi_vm_bind, vm_id) - 0usize];
    [
        "Offset of field: drm_asahi_vm_bind::num_binds",
    ][::core::mem::offset_of!(drm_asahi_vm_bind, num_binds) - 4usize];
    [
        "Offset of field: drm_asahi_vm_bind::stride",
    ][::core::mem::offset_of!(drm_asahi_vm_bind, stride) - 8usize];
    [
        "Offset of field: drm_asahi_vm_bind::pad",
    ][::core::mem::offset_of!(drm_asahi_vm_bind, pad) - 12usize];
    [
        "Offset of field: drm_asahi_vm_bind::userptr",
    ][::core::mem::offset_of!(drm_asahi_vm_bind, userptr) - 16usize];
};
/// @DRM_ASAHI_BIND_OBJECT_OP_BIND: Bind a BO as a special GPU object
pub const drm_asahi_bind_object_op_DRM_ASAHI_BIND_OBJECT_OP_BIND: drm_asahi_bind_object_op = 0;
/// @DRM_ASAHI_BIND_OBJECT_OP_UNBIND: Unbind a special GPU object
pub const drm_asahi_bind_object_op_DRM_ASAHI_BIND_OBJECT_OP_UNBIND: drm_asahi_bind_object_op = 1;
/// enum drm_asahi_bind_object_op - Special object bind operation
pub type drm_asahi_bind_object_op = ::core::ffi::c_uint;
/** @DRM_ASAHI_BIND_OBJECT_USAGE_TIMESTAMPS: Map a BO as a timestamp
 buffer.*/
pub const drm_asahi_bind_object_flags_DRM_ASAHI_BIND_OBJECT_USAGE_TIMESTAMPS: drm_asahi_bind_object_flags = 1;
/// enum drm_asahi_bind_object_flags - Special object bind flags
pub type drm_asahi_bind_object_flags = ::core::ffi::c_uint;
/** struct drm_asahi_gem_bind_object - Arguments passed to
 DRM_IOCTL_ASAHI_GEM_BIND_OBJECT*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_asahi_gem_bind_object {
    /// @op: Bind operation (enum drm_asahi_bind_object_op)
    pub op: __u32,
    /// @flags: Combination of drm_asahi_bind_object_flags flags.
    pub flags: __u32,
    /// @handle: GEM object to bind/unbind (BIND)
    pub handle: __u32,
    /// @vm_id: The ID of the VM to operate on (MBZ currently)
    pub vm_id: __u32,
    /// @offset: Offset into the object (BIND only)
    pub offset: __u64,
    /// @range: Number of bytes to bind/unbind (BIND only)
    pub range: __u64,
    /// @object_handle: Object handle (out for BIND, in for UNBIND)
    pub object_handle: __u32,
    /// @pad: MBZ
    pub pad: __u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of drm_asahi_gem_bind_object",
    ][::core::mem::size_of::<drm_asahi_gem_bind_object>() - 40usize];
    [
        "Alignment of drm_asahi_gem_bind_object",
    ][::core::mem::align_of::<drm_asahi_gem_bind_object>() - 8usize];
    [
        "Offset of field: drm_asahi_gem_bind_object::op",
    ][::core::mem::offset_of!(drm_asahi_gem_bind_object, op) - 0usize];
    [
        "Offset of field: drm_asahi_gem_bind_object::flags",
    ][::core::mem::offset_of!(drm_asahi_gem_bind_object, flags) - 4usize];
    [
        "Offset of field: drm_asahi_gem_bind_object::handle",
    ][::core::mem::offset_of!(drm_asahi_gem_bind_object, handle) - 8usize];
    [
        "Offset of field: drm_asahi_gem_bind_object::vm_id",
    ][::core::mem::offset_of!(drm_asahi_gem_bind_object, vm_id) - 12usize];
    [
        "Offset of field: drm_asahi_gem_bind_object::offset",
    ][::core::mem::offset_of!(drm_asahi_gem_bind_object, offset) - 16usize];
    [
        "Offset of field: drm_asahi_gem_bind_object::range",
    ][::core::mem::offset_of!(drm_asahi_gem_bind_object, range) - 24usize];
    [
        "Offset of field: drm_asahi_gem_bind_object::object_handle",
    ][::core::mem::offset_of!(drm_asahi_gem_bind_object, object_handle) - 32usize];
    [
        "Offset of field: drm_asahi_gem_bind_object::pad",
    ][::core::mem::offset_of!(drm_asahi_gem_bind_object, pad) - 36usize];
};
/** @DRM_ASAHI_CMD_RENDER: Render command, executing on the render
 subqueue. Combined vertex and fragment operation.

 Followed by a @drm_asahi_cmd_render payload.*/
pub const drm_asahi_cmd_type_DRM_ASAHI_CMD_RENDER: drm_asahi_cmd_type = 0;
/** @DRM_ASAHI_CMD_COMPUTE: Compute command on the compute subqueue.

 Followed by a @drm_asahi_cmd_compute payload.*/
pub const drm_asahi_cmd_type_DRM_ASAHI_CMD_COMPUTE: drm_asahi_cmd_type = 1;
/** @DRM_ASAHI_SET_VERTEX_ATTACHMENTS: Software command to set
 attachments for subsequent vertex shaders in the same submit.

 Followed by (possibly multiple) @drm_asahi_attachment payloads.*/
pub const drm_asahi_cmd_type_DRM_ASAHI_SET_VERTEX_ATTACHMENTS: drm_asahi_cmd_type = 2;
/** @DRM_ASAHI_SET_FRAGMENT_ATTACHMENTS: Software command to set
 attachments for subsequent fragment shaders in the same submit.

 Followed by (possibly multiple) @drm_asahi_attachment payloads.*/
pub const drm_asahi_cmd_type_DRM_ASAHI_SET_FRAGMENT_ATTACHMENTS: drm_asahi_cmd_type = 3;
/** @DRM_ASAHI_SET_COMPUTE_ATTACHMENTS: Software command to set
 attachments for subsequent compute shaders in the same submit.

 Followed by (possibly multiple) @drm_asahi_attachment payloads.*/
pub const drm_asahi_cmd_type_DRM_ASAHI_SET_COMPUTE_ATTACHMENTS: drm_asahi_cmd_type = 4;
/// enum drm_asahi_cmd_type - Command type
pub type drm_asahi_cmd_type = ::core::ffi::c_uint;
/// @DRM_ASAHI_PRIORITY_LOW: Low priority queue.
pub const drm_asahi_priority_DRM_ASAHI_PRIORITY_LOW: drm_asahi_priority = 0;
/// @DRM_ASAHI_PRIORITY_MEDIUM: Medium priority queue.
pub const drm_asahi_priority_DRM_ASAHI_PRIORITY_MEDIUM: drm_asahi_priority = 1;
/** @DRM_ASAHI_PRIORITY_HIGH: High priority queue.

 Reserved for future extension.*/
pub const drm_asahi_priority_DRM_ASAHI_PRIORITY_HIGH: drm_asahi_priority = 2;
/** @DRM_ASAHI_PRIORITY_REALTIME: Real-time priority queue.

 Reserved for future extension.*/
pub const drm_asahi_priority_DRM_ASAHI_PRIORITY_REALTIME: drm_asahi_priority = 3;
/** enum drm_asahi_priority - Scheduling queue priority.

 These priorities are forwarded to the firmware to influence firmware
 scheduling. The exact policy is ultimately decided by firmware, but
 these enums allow userspace to communicate the intentions.*/
pub type drm_asahi_priority = ::core::ffi::c_uint;
/** struct drm_asahi_queue_create - Arguments passed to
 DRM_IOCTL_ASAHI_QUEUE_CREATE*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_asahi_queue_create {
    /// @flags: MBZ
    pub flags: __u32,
    /// @vm_id: The ID of the VM this queue is bound to
    pub vm_id: __u32,
    /// @priority: One of drm_asahi_priority
    pub priority: __u32,
    /// @queue_id: The returned queue ID
    pub queue_id: __u32,
    /** @usc_exec_base: GPU base address for all USC binaries (shaders) on
 this queue. USC addresses are 32-bit relative to this 64-bit base.

 This sets the following registers on all queue commands:

	USC_EXEC_BASE_TA  (vertex)
	USC_EXEC_BASE_ISP (fragment)
	USC_EXEC_BASE_CP  (compute)

 While the hardware lets us configure these independently per command,
 we do not have a use case for this. Instead, we expect userspace to
 fix a 4GiB VA carveout for USC memory and pass its base address here.*/
    pub usc_exec_base: __u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of drm_asahi_queue_create",
    ][::core::mem::size_of::<drm_asahi_queue_create>() - 24usize];
    [
        "Alignment of drm_asahi_queue_create",
    ][::core::mem::align_of::<drm_asahi_queue_create>() - 8usize];
    [
        "Offset of field: drm_asahi_queue_create::flags",
    ][::core::mem::offset_of!(drm_asahi_queue_create, flags) - 0usize];
    [
        "Offset of field: drm_asahi_queue_create::vm_id",
    ][::core::mem::offset_of!(drm_asahi_queue_create, vm_id) - 4usize];
    [
        "Offset of field: drm_asahi_queue_create::priority",
    ][::core::mem::offset_of!(drm_asahi_queue_create, priority) - 8usize];
    [
        "Offset of field: drm_asahi_queue_create::queue_id",
    ][::core::mem::offset_of!(drm_asahi_queue_create, queue_id) - 12usize];
    [
        "Offset of field: drm_asahi_queue_create::usc_exec_base",
    ][::core::mem::offset_of!(drm_asahi_queue_create, usc_exec_base) - 16usize];
};
/** struct drm_asahi_queue_destroy - Arguments passed to
 DRM_IOCTL_ASAHI_QUEUE_DESTROY*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_asahi_queue_destroy {
    /// @queue_id: The queue ID to be destroyed
    pub queue_id: __u32,
    /// @pad: MBZ
    pub pad: __u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of drm_asahi_queue_destroy",
    ][::core::mem::size_of::<drm_asahi_queue_destroy>() - 8usize];
    [
        "Alignment of drm_asahi_queue_destroy",
    ][::core::mem::align_of::<drm_asahi_queue_destroy>() - 4usize];
    [
        "Offset of field: drm_asahi_queue_destroy::queue_id",
    ][::core::mem::offset_of!(drm_asahi_queue_destroy, queue_id) - 0usize];
    [
        "Offset of field: drm_asahi_queue_destroy::pad",
    ][::core::mem::offset_of!(drm_asahi_queue_destroy, pad) - 4usize];
};
/// @DRM_ASAHI_SYNC_SYNCOBJ: Binary sync object
pub const drm_asahi_sync_type_DRM_ASAHI_SYNC_SYNCOBJ: drm_asahi_sync_type = 0;
/// @DRM_ASAHI_SYNC_TIMELINE_SYNCOBJ: Timeline sync object
pub const drm_asahi_sync_type_DRM_ASAHI_SYNC_TIMELINE_SYNCOBJ: drm_asahi_sync_type = 1;
/// enum drm_asahi_sync_type - Sync item type
pub type drm_asahi_sync_type = ::core::ffi::c_uint;
/// struct drm_asahi_sync - Sync item
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_asahi_sync {
    /// @sync_type: One of drm_asahi_sync_type
    pub sync_type: __u32,
    /// @handle: The sync object handle
    pub handle: __u32,
    /// @timeline_value: Timeline value for timeline sync objects
    pub timeline_value: __u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of drm_asahi_sync"][::core::mem::size_of::<drm_asahi_sync>() - 16usize];
    ["Alignment of drm_asahi_sync"][::core::mem::align_of::<drm_asahi_sync>() - 8usize];
    [
        "Offset of field: drm_asahi_sync::sync_type",
    ][::core::mem::offset_of!(drm_asahi_sync, sync_type) - 0usize];
    [
        "Offset of field: drm_asahi_sync::handle",
    ][::core::mem::offset_of!(drm_asahi_sync, handle) - 4usize];
    [
        "Offset of field: drm_asahi_sync::timeline_value",
    ][::core::mem::offset_of!(drm_asahi_sync, timeline_value) - 8usize];
};
/** struct drm_asahi_cmd_header - Top level command structure

 This struct is core to the command buffer definition and therefore is not
 extensible.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_asahi_cmd_header {
    /// @cmd_type: One of drm_asahi_cmd_type
    pub cmd_type: __u16,
    /** @size: Size of this command, not including this header.

 For hardware commands, this enables extensibility of commands without
 requiring extra command types. Passing a command that is shorter
 than expected is explicitly allowed for backwards-compatibility.
 Truncated fields will be zeroed.

 For the synthetic attachment setting commands, this implicitly
 encodes the number of attachments. These commands take multiple
 fixed-size @drm_asahi_attachment structures as their payload, so size
 equals number of attachments * sizeof(struct drm_asahi_attachment).*/
    pub size: __u16,
    /** @vdm_barrier: VDM (render) command index to wait on.

 Barriers are indices relative to the beginning of a given submit. A
 barrier of 0 waits on commands submitted to the respective subqueue
 in previous submit ioctls. A barrier of N waits on N previous
 commands on the subqueue within the current submit ioctl. As a
 special case, passing @DRM_ASAHI_BARRIER_NONE avoids waiting on any
 commands in the subqueue.

 Examples:

   0: This waits on all previous work.

   NONE: This does not wait for anything on this subqueue.

   1: This waits on the first render command in the submit.
   This is valid only if there are multiple render commands in the
   same submit.

 Barriers are valid only for hardware commands. Synthetic software
 commands to set attachments must pass NONE here.*/
    pub vdm_barrier: __u16,
    /** @cdm_barrier: CDM (compute) command index to wait on.

 See @vdm_barrier, and replace VDM/render with CDM/compute.*/
    pub cdm_barrier: __u16,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of drm_asahi_cmd_header",
    ][::core::mem::size_of::<drm_asahi_cmd_header>() - 8usize];
    [
        "Alignment of drm_asahi_cmd_header",
    ][::core::mem::align_of::<drm_asahi_cmd_header>() - 2usize];
    [
        "Offset of field: drm_asahi_cmd_header::cmd_type",
    ][::core::mem::offset_of!(drm_asahi_cmd_header, cmd_type) - 0usize];
    [
        "Offset of field: drm_asahi_cmd_header::size",
    ][::core::mem::offset_of!(drm_asahi_cmd_header, size) - 2usize];
    [
        "Offset of field: drm_asahi_cmd_header::vdm_barrier",
    ][::core::mem::offset_of!(drm_asahi_cmd_header, vdm_barrier) - 4usize];
    [
        "Offset of field: drm_asahi_cmd_header::cdm_barrier",
    ][::core::mem::offset_of!(drm_asahi_cmd_header, cdm_barrier) - 6usize];
};
/// struct drm_asahi_submit - Arguments passed to DRM_IOCTL_ASAHI_SUBMIT
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_asahi_submit {
    /** @syncs: An optional pointer to an array of drm_asahi_sync. The first
 @in_sync_count elements are in-syncs, then the remaining
 @out_sync_count elements are out-syncs. Using a single array with
 explicit partitioning simplifies handling.*/
    pub syncs: __u64,
    /** @cmdbuf: Pointer to the command buffer to submit.

 This is a flat command buffer. By design, it contains no CPU
 pointers, which makes it suitable for a virtgpu wire protocol without
 requiring any serializing/deserializing step.

 It consists of a series of commands. Each command begins with a
 fixed-size @drm_asahi_cmd_header header and is followed by a
 variable-length payload according to the type and size in the header.

 The combined count of "real" hardware commands must be nonzero and at
 most drm_asahi_params_global::max_commands_per_submission.*/
    pub cmdbuf: __u64,
    /// @flags: Flags for command submission (MBZ)
    pub flags: __u32,
    /// @queue_id: The queue ID to be submitted to
    pub queue_id: __u32,
    /** @in_sync_count: Number of sync objects to wait on before starting
 this job.*/
    pub in_sync_count: __u32,
    /** @out_sync_count: Number of sync objects to signal upon completion of
 this job.*/
    pub out_sync_count: __u32,
    /// @cmdbuf_size: Command buffer size in bytes
    pub cmdbuf_size: __u32,
    /// @pad: MBZ
    pub pad: __u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of drm_asahi_submit"][::core::mem::size_of::<drm_asahi_submit>() - 40usize];
    [
        "Alignment of drm_asahi_submit",
    ][::core::mem::align_of::<drm_asahi_submit>() - 8usize];
    [
        "Offset of field: drm_asahi_submit::syncs",
    ][::core::mem::offset_of!(drm_asahi_submit, syncs) - 0usize];
    [
        "Offset of field: drm_asahi_submit::cmdbuf",
    ][::core::mem::offset_of!(drm_asahi_submit, cmdbuf) - 8usize];
    [
        "Offset of field: drm_asahi_submit::flags",
    ][::core::mem::offset_of!(drm_asahi_submit, flags) - 16usize];
    [
        "Offset of field: drm_asahi_submit::queue_id",
    ][::core::mem::offset_of!(drm_asahi_submit, queue_id) - 20usize];
    [
        "Offset of field: drm_asahi_submit::in_sync_count",
    ][::core::mem::offset_of!(drm_asahi_submit, in_sync_count) - 24usize];
    [
        "Offset of field: drm_asahi_submit::out_sync_count",
    ][::core::mem::offset_of!(drm_asahi_submit, out_sync_count) - 28usize];
    [
        "Offset of field: drm_asahi_submit::cmdbuf_size",
    ][::core::mem::offset_of!(drm_asahi_submit, cmdbuf_size) - 32usize];
    [
        "Offset of field: drm_asahi_submit::pad",
    ][::core::mem::offset_of!(drm_asahi_submit, pad) - 36usize];
};
/** struct drm_asahi_attachment - Describe an "attachment".

 Attachments are any memory written by shaders, notably including render
 target attachments written by the end-of-tile program. This is purely a hint
 about the accessed memory regions. It is optional to specify, which is
 fortunate as it cannot be specified precisely with bindless access anyway.
 But where possible, it's probably a good idea for userspace to include these
 hints, forwarded to the firmware.

 This struct is implicitly sized and therefore is not extensible.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_asahi_attachment {
    /// @pointer: Base address of the attachment
    pub pointer: __u64,
    /// @size: Size of the attachment in bytes
    pub size: __u64,
    /// @pad: MBZ
    pub pad: __u32,
    /// @flags: MBZ
    pub flags: __u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of drm_asahi_attachment",
    ][::core::mem::size_of::<drm_asahi_attachment>() - 24usize];
    [
        "Alignment of drm_asahi_attachment",
    ][::core::mem::align_of::<drm_asahi_attachment>() - 8usize];
    [
        "Offset of field: drm_asahi_attachment::pointer",
    ][::core::mem::offset_of!(drm_asahi_attachment, pointer) - 0usize];
    [
        "Offset of field: drm_asahi_attachment::size",
    ][::core::mem::offset_of!(drm_asahi_attachment, size) - 8usize];
    [
        "Offset of field: drm_asahi_attachment::pad",
    ][::core::mem::offset_of!(drm_asahi_attachment, pad) - 16usize];
    [
        "Offset of field: drm_asahi_attachment::flags",
    ][::core::mem::offset_of!(drm_asahi_attachment, flags) - 20usize];
};
/** @DRM_ASAHI_RENDER_VERTEX_SCRATCH: A vertex stage shader uses scratch
 memory.*/
pub const drm_asahi_render_flags_DRM_ASAHI_RENDER_VERTEX_SCRATCH: drm_asahi_render_flags = 1;
/** @DRM_ASAHI_RENDER_PROCESS_EMPTY_TILES: Process even empty tiles.
 This must be set when clearing render targets.*/
pub const drm_asahi_render_flags_DRM_ASAHI_RENDER_PROCESS_EMPTY_TILES: drm_asahi_render_flags = 2;
/** @DRM_ASAHI_RENDER_NO_VERTEX_CLUSTERING: Run vertex stage on a single
 cluster (on multi-cluster GPUs)

 This harms performance but can workaround certain sync/coherency
 bugs, and therefore is useful for debugging.*/
pub const drm_asahi_render_flags_DRM_ASAHI_RENDER_NO_VERTEX_CLUSTERING: drm_asahi_render_flags = 4;
/** @DRM_ASAHI_RENDER_DBIAS_IS_INT: Use integer depth bias formula.

 Graphics specifications contain two alternate formulas for depth
 bias, a float formula used with floating-point depth buffers and an
 integer formula using with unorm depth buffers. This flag specifies
 that the integer formula should be used. If omitted, the float
 formula is used instead.

 This corresponds to bit 18 of the relevant hardware control register,
 so we match that here for efficiency.*/
pub const drm_asahi_render_flags_DRM_ASAHI_RENDER_DBIAS_IS_INT: drm_asahi_render_flags = 262144;
pub type drm_asahi_render_flags = ::core::ffi::c_uint;
/** struct drm_asahi_zls_buffer - Describe a depth or stencil buffer.

 These fields correspond to hardware registers in the ZLS (Z Load/Store) unit.
 There are three hardware registers for each field respectively for loads,
 stores, and partial renders. In practice, it makes sense to set all to the
 same values, except in exceptional cases not yet implemented in userspace, so
 we do not duplicate here for simplicity/efficiency.

 This struct is embedded in other structs and therefore is not extensible.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_asahi_zls_buffer {
    /// @base: Base address of the buffer
    pub base: __u64,
    /** @comp_base: If the load buffer is compressed, address of the
 compression metadata section.*/
    pub comp_base: __u64,
    /** @stride: If layered rendering is enabled, the number of bytes
 between each layer of the buffer.*/
    pub stride: __u32,
    /** @comp_stride: If layered rendering is enabled, the number of bytes
 between each layer of the compression metadata.*/
    pub comp_stride: __u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of drm_asahi_zls_buffer",
    ][::core::mem::size_of::<drm_asahi_zls_buffer>() - 24usize];
    [
        "Alignment of drm_asahi_zls_buffer",
    ][::core::mem::align_of::<drm_asahi_zls_buffer>() - 8usize];
    [
        "Offset of field: drm_asahi_zls_buffer::base",
    ][::core::mem::offset_of!(drm_asahi_zls_buffer, base) - 0usize];
    [
        "Offset of field: drm_asahi_zls_buffer::comp_base",
    ][::core::mem::offset_of!(drm_asahi_zls_buffer, comp_base) - 8usize];
    [
        "Offset of field: drm_asahi_zls_buffer::stride",
    ][::core::mem::offset_of!(drm_asahi_zls_buffer, stride) - 16usize];
    [
        "Offset of field: drm_asahi_zls_buffer::comp_stride",
    ][::core::mem::offset_of!(drm_asahi_zls_buffer, comp_stride) - 20usize];
};
/** struct drm_asahi_timestamp - Describe a timestamp write.

 The firmware can optionally write the GPU timestamp at render pass
 granularities, but it needs to be mapped specially via
 DRM_IOCTL_ASAHI_GEM_BIND_OBJECT. This structure therefore describes where to
 write as a handle-offset pair, rather than a GPU address like normal.

 This struct is embedded in other structs and therefore is not extensible.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_asahi_timestamp {
    /** @handle: Handle of the timestamp buffer, or 0 to skip this
 timestamp. If nonzero, this must equal the value returned in
 drm_asahi_gem_bind_object::object_handle.*/
    pub handle: __u32,
    /// @offset: Offset to write into the timestamp buffer
    pub offset: __u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of drm_asahi_timestamp",
    ][::core::mem::size_of::<drm_asahi_timestamp>() - 8usize];
    [
        "Alignment of drm_asahi_timestamp",
    ][::core::mem::align_of::<drm_asahi_timestamp>() - 4usize];
    [
        "Offset of field: drm_asahi_timestamp::handle",
    ][::core::mem::offset_of!(drm_asahi_timestamp, handle) - 0usize];
    [
        "Offset of field: drm_asahi_timestamp::offset",
    ][::core::mem::offset_of!(drm_asahi_timestamp, offset) - 4usize];
};
/** struct drm_asahi_timestamps - Describe timestamp writes.

 Each operation that can be timestamped, can be timestamped at the start and
 end. Therefore, drm_asahi_timestamp structs always come in pairs, bundled
 together into drm_asahi_timestamps.

 This struct is embedded in other structs and therefore is not extensible.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_asahi_timestamps {
    /// @start: Timestamp recorded at the start of the operation
    pub start: drm_asahi_timestamp,
    /// @end: Timestamp recorded at the end of the operation
    pub end: drm_asahi_timestamp,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of drm_asahi_timestamps",
    ][::core::mem::size_of::<drm_asahi_timestamps>() - 16usize];
    [
        "Alignment of drm_asahi_timestamps",
    ][::core::mem::align_of::<drm_asahi_timestamps>() - 4usize];
    [
        "Offset of field: drm_asahi_timestamps::start",
    ][::core::mem::offset_of!(drm_asahi_timestamps, start) - 0usize];
    [
        "Offset of field: drm_asahi_timestamps::end",
    ][::core::mem::offset_of!(drm_asahi_timestamps, end) - 8usize];
};
/** struct drm_asahi_helper_program - Describe helper program configuration.

 The helper program is a compute-like kernel required for various hardware
 functionality. Its most important role is dynamically allocating
 scratch/stack memory for individual subgroups, by partitioning a static
 allocation shared for the whole device. It is supplied by userspace via
 drm_asahi_helper_program and internally dispatched by the hardware as needed.

 This struct is embedded in other structs and therefore is not extensible.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_asahi_helper_program {
    /** @binary: USC address to the helper program binary. This is a tagged
 pointer with configuration in the bottom bits.*/
    pub binary: __u32,
    /// @cfg: Additional configuration bits for the helper program.
    pub cfg: __u32,
    /** @data: Data passed to the helper program. This value is not
 interpreted by the kernel, firmware, or hardware in any way. It is
 simply a sideband for userspace, set with the submit ioctl and read
 via special registers inside the helper program.

 In practice, userspace will pass a 64-bit GPU VA here pointing to the
 actual arguments, which presumably don't fit in 64-bits.*/
    pub data: __u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of drm_asahi_helper_program",
    ][::core::mem::size_of::<drm_asahi_helper_program>() - 16usize];
    [
        "Alignment of drm_asahi_helper_program",
    ][::core::mem::align_of::<drm_asahi_helper_program>() - 8usize];
    [
        "Offset of field: drm_asahi_helper_program::binary",
    ][::core::mem::offset_of!(drm_asahi_helper_program, binary) - 0usize];
    [
        "Offset of field: drm_asahi_helper_program::cfg",
    ][::core::mem::offset_of!(drm_asahi_helper_program, cfg) - 4usize];
    [
        "Offset of field: drm_asahi_helper_program::data",
    ][::core::mem::offset_of!(drm_asahi_helper_program, data) - 8usize];
};
/** struct drm_asahi_bg_eot - Describe a background or end-of-tile program.

 The background and end-of-tile programs are dispatched by the hardware at the
 beginning and end of rendering. As the hardware "tilebuffer" is simply local
 memory, these programs are necessary to implement API-level render targets.
 The fragment-like background program is responsible for loading either the
 clear colour or the existing render target contents, while the compute-like
 end-of-tile program stores the tilebuffer contents to memory.

 This struct is embedded in other structs and therefore is not extensible.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_asahi_bg_eot {
    /** @usc: USC address of the hardware USC words binding resources
 (including images and uniforms) and the program itself. Note this is
 an additional layer of indirection compared to the helper program,
 avoiding the need for a sideband for data. This is a tagged pointer
 with additional configuration in the bottom bits.*/
    pub usc: __u32,
    /** @rsrc_spec: Resource specifier for the program. This is a packed
 hardware data structure describing the required number of registers,
 uniforms, bound textures, and bound samplers.*/
    pub rsrc_spec: __u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of drm_asahi_bg_eot"][::core::mem::size_of::<drm_asahi_bg_eot>() - 8usize];
    [
        "Alignment of drm_asahi_bg_eot",
    ][::core::mem::align_of::<drm_asahi_bg_eot>() - 4usize];
    [
        "Offset of field: drm_asahi_bg_eot::usc",
    ][::core::mem::offset_of!(drm_asahi_bg_eot, usc) - 0usize];
    [
        "Offset of field: drm_asahi_bg_eot::rsrc_spec",
    ][::core::mem::offset_of!(drm_asahi_bg_eot, rsrc_spec) - 4usize];
};
/** struct drm_asahi_cmd_render - Command to submit 3D

 This command submits a single render pass. The hardware control stream may
 include many draws and subpasses, but within the command, the framebuffer
 dimensions and attachments are fixed.

 The hardware requires the firmware to set a large number of Control Registers
 setting up state at render pass granularity before each command rendering 3D.
 The firmware bundles this state into data structures. Unfortunately, we
 cannot expose either any of that directly to userspace, because the
 kernel-firmware ABI is not stable. Although we can guarantee the firmware
 updates in tandem with the kernel, we cannot break old userspace when
 upgrading the firmware and kernel. Therefore, we need to abstract well the
 data structures to avoid tying our hands with future firmwares.

 The bulk of drm_asahi_cmd_render therefore consists of values of hardware
 control registers, marshalled via the firmware interface.

 The framebuffer/tilebuffer dimensions are also specified here. In addition to
 being passed to the firmware/hardware, the kernel requires these dimensions
 to calculate various essential tiling-related data structures. It is
 unfortunate that our submits are heavier than on vendors with saner
 hardware-software interfaces. The upshot is all of this information is
 readily available to userspace with all current APIs.

 It looks odd - but it's not overly burdensome and it ensures we can remain
 compatible with old userspace.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_asahi_cmd_render {
    /// @flags: Combination of drm_asahi_render_flags flags.
    pub flags: __u32,
    /** @isp_zls_pixels: ISP_ZLS_PIXELS register value. This contains the
 depth/stencil width/height, which may differ from the framebuffer
 width/height.*/
    pub isp_zls_pixels: __u32,
    /** @vdm_ctrl_stream_base: VDM_CTRL_STREAM_BASE register value. GPU
 address to the beginning of the VDM control stream.*/
    pub vdm_ctrl_stream_base: __u64,
    /// @vertex_helper: Helper program used for the vertex shader
    pub vertex_helper: drm_asahi_helper_program,
    /// @fragment_helper: Helper program used for the fragment shader
    pub fragment_helper: drm_asahi_helper_program,
    /** @isp_scissor_base: ISP_SCISSOR_BASE register value. GPU address of an
 array of scissor descriptors indexed in the render pass.*/
    pub isp_scissor_base: __u64,
    /** @isp_dbias_base: ISP_DBIAS_BASE register value. GPU address of an
 array of depth bias values indexed in the render pass.*/
    pub isp_dbias_base: __u64,
    /** @isp_oclqry_base: ISP_OCLQRY_BASE register value. GPU address of an
 array of occlusion query results written by the render pass.*/
    pub isp_oclqry_base: __u64,
    /// @depth: Depth buffer
    pub depth: drm_asahi_zls_buffer,
    /// @stencil: Stencil buffer
    pub stencil: drm_asahi_zls_buffer,
    /// @zls_ctrl: ZLS_CTRL register value
    pub zls_ctrl: __u64,
    /// @ppp_multisamplectl: PPP_MULTISAMPLECTL register value
    pub ppp_multisamplectl: __u64,
    /** @sampler_heap: Base address of the sampler heap. This heap is used
 for both vertex shaders and fragment shaders. The registers are
 per-stage, but there is no known use case for separate heaps.*/
    pub sampler_heap: __u64,
    /// @ppp_ctrl: PPP_CTRL register value
    pub ppp_ctrl: __u32,
    /// @width_px: Framebuffer width in pixels
    pub width_px: __u16,
    /// @height_px: Framebuffer height in pixels
    pub height_px: __u16,
    /// @layers: Number of layers in the framebuffer
    pub layers: __u16,
    /// @sampler_count: Number of samplers in the sampler heap.
    pub sampler_count: __u16,
    /// @utile_width_px: Width of a logical tilebuffer tile in pixels
    pub utile_width_px: __u8,
    /// @utile_height_px: Height of a logical tilebuffer tile in pixels
    pub utile_height_px: __u8,
    /// @samples: # of samples in the framebuffer. Must be 1, 2, or 4.
    pub samples: __u8,
    /// @sample_size_B: # of bytes in the tilebuffer required per sample.
    pub sample_size_B: __u8,
    /** @isp_merge_upper_x: 32-bit float used in the hardware triangle
 merging. Calculate as: tan(60 deg) * width.

 Making these values UAPI avoids requiring floating-point calculations
 in the kernel in the hot path.*/
    pub isp_merge_upper_x: __u32,
    /** @isp_merge_upper_y: 32-bit float. Calculate as: tan(60 deg) * height.
 See @isp_merge_upper_x.*/
    pub isp_merge_upper_y: __u32,
    /// @bg: Background program run for each tile at the start
    pub bg: drm_asahi_bg_eot,
    /// @eot: End-of-tile program ran for each tile at the end
    pub eot: drm_asahi_bg_eot,
    /** @partial_bg: Background program ran at the start of each tile when
 resuming the render pass during a partial render.*/
    pub partial_bg: drm_asahi_bg_eot,
    /** @partial_eot: End-of-tile program ran at the end of each tile when
 pausing the render pass during a partial render.*/
    pub partial_eot: drm_asahi_bg_eot,
    /** @isp_bgobjdepth: ISP_BGOBJDEPTH register value. This is the depth
 buffer clear value, encoded in the depth buffer's format: either a
 32-bit float or a 16-bit unorm (with upper bits zeroed).*/
    pub isp_bgobjdepth: __u32,
    /** @isp_bgobjvals: ISP_BGOBJVALS register value. The bottom 8-bits
 contain the stencil buffer clear value.*/
    pub isp_bgobjvals: __u32,
    /// @ts_vtx: Timestamps for the vertex portion of the render
    pub ts_vtx: drm_asahi_timestamps,
    /// @ts_frag: Timestamps for the fragment portion of the render
    pub ts_frag: drm_asahi_timestamps,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of drm_asahi_cmd_render",
    ][::core::mem::size_of::<drm_asahi_cmd_render>() - 240usize];
    [
        "Alignment of drm_asahi_cmd_render",
    ][::core::mem::align_of::<drm_asahi_cmd_render>() - 8usize];
    [
        "Offset of field: drm_asahi_cmd_render::flags",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, flags) - 0usize];
    [
        "Offset of field: drm_asahi_cmd_render::isp_zls_pixels",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, isp_zls_pixels) - 4usize];
    [
        "Offset of field: drm_asahi_cmd_render::vdm_ctrl_stream_base",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, vdm_ctrl_stream_base) - 8usize];
    [
        "Offset of field: drm_asahi_cmd_render::vertex_helper",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, vertex_helper) - 16usize];
    [
        "Offset of field: drm_asahi_cmd_render::fragment_helper",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, fragment_helper) - 32usize];
    [
        "Offset of field: drm_asahi_cmd_render::isp_scissor_base",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, isp_scissor_base) - 48usize];
    [
        "Offset of field: drm_asahi_cmd_render::isp_dbias_base",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, isp_dbias_base) - 56usize];
    [
        "Offset of field: drm_asahi_cmd_render::isp_oclqry_base",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, isp_oclqry_base) - 64usize];
    [
        "Offset of field: drm_asahi_cmd_render::depth",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, depth) - 72usize];
    [
        "Offset of field: drm_asahi_cmd_render::stencil",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, stencil) - 96usize];
    [
        "Offset of field: drm_asahi_cmd_render::zls_ctrl",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, zls_ctrl) - 120usize];
    [
        "Offset of field: drm_asahi_cmd_render::ppp_multisamplectl",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, ppp_multisamplectl) - 128usize];
    [
        "Offset of field: drm_asahi_cmd_render::sampler_heap",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, sampler_heap) - 136usize];
    [
        "Offset of field: drm_asahi_cmd_render::ppp_ctrl",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, ppp_ctrl) - 144usize];
    [
        "Offset of field: drm_asahi_cmd_render::width_px",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, width_px) - 148usize];
    [
        "Offset of field: drm_asahi_cmd_render::height_px",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, height_px) - 150usize];
    [
        "Offset of field: drm_asahi_cmd_render::layers",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, layers) - 152usize];
    [
        "Offset of field: drm_asahi_cmd_render::sampler_count",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, sampler_count) - 154usize];
    [
        "Offset of field: drm_asahi_cmd_render::utile_width_px",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, utile_width_px) - 156usize];
    [
        "Offset of field: drm_asahi_cmd_render::utile_height_px",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, utile_height_px) - 157usize];
    [
        "Offset of field: drm_asahi_cmd_render::samples",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, samples) - 158usize];
    [
        "Offset of field: drm_asahi_cmd_render::sample_size_B",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, sample_size_B) - 159usize];
    [
        "Offset of field: drm_asahi_cmd_render::isp_merge_upper_x",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, isp_merge_upper_x) - 160usize];
    [
        "Offset of field: drm_asahi_cmd_render::isp_merge_upper_y",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, isp_merge_upper_y) - 164usize];
    [
        "Offset of field: drm_asahi_cmd_render::bg",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, bg) - 168usize];
    [
        "Offset of field: drm_asahi_cmd_render::eot",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, eot) - 176usize];
    [
        "Offset of field: drm_asahi_cmd_render::partial_bg",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, partial_bg) - 184usize];
    [
        "Offset of field: drm_asahi_cmd_render::partial_eot",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, partial_eot) - 192usize];
    [
        "Offset of field: drm_asahi_cmd_render::isp_bgobjdepth",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, isp_bgobjdepth) - 200usize];
    [
        "Offset of field: drm_asahi_cmd_render::isp_bgobjvals",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, isp_bgobjvals) - 204usize];
    [
        "Offset of field: drm_asahi_cmd_render::ts_vtx",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, ts_vtx) - 208usize];
    [
        "Offset of field: drm_asahi_cmd_render::ts_frag",
    ][::core::mem::offset_of!(drm_asahi_cmd_render, ts_frag) - 224usize];
};
/** struct drm_asahi_cmd_compute - Command to submit compute

 This command submits a control stream consisting of compute dispatches. There
 is essentially no limit on how many compute dispatches may be included in a
 single compute command, although timestamps are at command granularity.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_asahi_cmd_compute {
    /// @flags: MBZ
    pub flags: __u32,
    /// @sampler_count: Number of samplers in the sampler heap.
    pub sampler_count: __u32,
    /** @cdm_ctrl_stream_base: CDM_CTRL_STREAM_BASE register value. GPU
 address to the beginning of the CDM control stream.*/
    pub cdm_ctrl_stream_base: __u64,
    /** @cdm_ctrl_stream_end: GPU base address to the end of the hardware
 control stream. Note this only considers the first contiguous segment
 of the control stream, as the stream might jump elsewhere.*/
    pub cdm_ctrl_stream_end: __u64,
    /// @sampler_heap: Base address of the sampler heap.
    pub sampler_heap: __u64,
    /// @helper: Helper program used for this compute command
    pub helper: drm_asahi_helper_program,
    /// @ts: Timestamps for the compute command
    pub ts: drm_asahi_timestamps,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of drm_asahi_cmd_compute",
    ][::core::mem::size_of::<drm_asahi_cmd_compute>() - 64usize];
    [
        "Alignment of drm_asahi_cmd_compute",
    ][::core::mem::align_of::<drm_asahi_cmd_compute>() - 8usize];
    [
        "Offset of field: drm_asahi_cmd_compute::flags",
    ][::core::mem::offset_of!(drm_asahi_cmd_compute, flags) - 0usize];
    [
        "Offset of field: drm_asahi_cmd_compute::sampler_count",
    ][::core::mem::offset_of!(drm_asahi_cmd_compute, sampler_count) - 4usize];
    [
        "Offset of field: drm_asahi_cmd_compute::cdm_ctrl_stream_base",
    ][::core::mem::offset_of!(drm_asahi_cmd_compute, cdm_ctrl_stream_base) - 8usize];
    [
        "Offset of field: drm_asahi_cmd_compute::cdm_ctrl_stream_end",
    ][::core::mem::offset_of!(drm_asahi_cmd_compute, cdm_ctrl_stream_end) - 16usize];
    [
        "Offset of field: drm_asahi_cmd_compute::sampler_heap",
    ][::core::mem::offset_of!(drm_asahi_cmd_compute, sampler_heap) - 24usize];
    [
        "Offset of field: drm_asahi_cmd_compute::helper",
    ][::core::mem::offset_of!(drm_asahi_cmd_compute, helper) - 32usize];
    [
        "Offset of field: drm_asahi_cmd_compute::ts",
    ][::core::mem::offset_of!(drm_asahi_cmd_compute, ts) - 48usize];
};
/// struct drm_asahi_get_time - Arguments passed to DRM_IOCTL_ASAHI_GET_TIME
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct drm_asahi_get_time {
    /// @flags: MBZ.
    pub flags: __u64,
    /// @gpu_timestamp: On return, the GPU timestamp in nanoseconds.
    pub gpu_timestamp: __u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of drm_asahi_get_time",
    ][::core::mem::size_of::<drm_asahi_get_time>() - 16usize];
    [
        "Alignment of drm_asahi_get_time",
    ][::core::mem::align_of::<drm_asahi_get_time>() - 8usize];
    [
        "Offset of field: drm_asahi_get_time::flags",
    ][::core::mem::offset_of!(drm_asahi_get_time, flags) - 0usize];
    [
        "Offset of field: drm_asahi_get_time::gpu_timestamp",
    ][::core::mem::offset_of!(drm_asahi_get_time, gpu_timestamp) - 8usize];
};
pub const DRM_IOCTL_ASAHI_GET_PARAMS: _bindgen_ty_1 = 1075340352;
pub const DRM_IOCTL_ASAHI_GET_TIME: _bindgen_ty_1 = 3222299713;
pub const DRM_IOCTL_ASAHI_VM_CREATE: _bindgen_ty_1 = 3222824002;
pub const DRM_IOCTL_ASAHI_VM_DESTROY: _bindgen_ty_1 = 1074291779;
pub const DRM_IOCTL_ASAHI_VM_BIND: _bindgen_ty_1 = 1075340356;
pub const DRM_IOCTL_ASAHI_GEM_CREATE: _bindgen_ty_1 = 3222824005;
pub const DRM_IOCTL_ASAHI_GEM_MMAP_OFFSET: _bindgen_ty_1 = 3222299718;
pub const DRM_IOCTL_ASAHI_GEM_BIND_OBJECT: _bindgen_ty_1 = 3223872583;
pub const DRM_IOCTL_ASAHI_QUEUE_CREATE: _bindgen_ty_1 = 3222824008;
pub const DRM_IOCTL_ASAHI_QUEUE_DESTROY: _bindgen_ty_1 = 1074291785;
pub const DRM_IOCTL_ASAHI_SUBMIT: _bindgen_ty_1 = 1076388938;
pub type _bindgen_ty_1 = ::core::ffi::c_uint;
