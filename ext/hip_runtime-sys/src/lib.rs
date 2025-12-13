// Generated automatically by zluda_bindgen
// DO NOT EDIT MANUALLY
#![allow(warnings)]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len(),
        );
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len(),
        );
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
pub const hipTextureType1D: u32 = 1;
pub const hipTextureType2D: u32 = 2;
pub const hipTextureType3D: u32 = 3;
pub const hipTextureTypeCubemap: u32 = 12;
pub const hipTextureType1DLayered: u32 = 241;
pub const hipTextureType2DLayered: u32 = 242;
pub const hipTextureTypeCubemapLayered: u32 = 252;
pub const hipIpcMemLazyEnablePeerAccess: u32 = 1;
pub const hipStreamDefault: u32 = 0;
pub const hipStreamNonBlocking: u32 = 1;
pub const hipEventDefault: u32 = 0;
pub const hipEventBlockingSync: u32 = 1;
pub const hipEventDisableTiming: u32 = 2;
pub const hipEventInterprocess: u32 = 4;
pub const hipEventRecordDefault: u32 = 0;
pub const hipEventRecordExternal: u32 = 1;
pub const hipEventWaitDefault: u32 = 0;
pub const hipEventWaitExternal: u32 = 1;
pub const hipEventDisableSystemFence: u32 = 536870912;
pub const hipEventReleaseToDevice: u32 = 1073741824;
pub const hipEventReleaseToSystem: u32 = 2147483648;
pub const hipHostAllocDefault: u32 = 0;
pub const hipHostMallocDefault: u32 = 0;
pub const hipHostAllocPortable: u32 = 1;
pub const hipHostMallocPortable: u32 = 1;
pub const hipHostAllocMapped: u32 = 2;
pub const hipHostMallocMapped: u32 = 2;
pub const hipHostAllocWriteCombined: u32 = 4;
pub const hipHostMallocWriteCombined: u32 = 4;
pub const hipHostMallocNumaUser: u32 = 536870912;
pub const hipHostMallocCoherent: u32 = 1073741824;
pub const hipHostMallocNonCoherent: u32 = 2147483648;
pub const hipMemAttachGlobal: u32 = 1;
pub const hipMemAttachHost: u32 = 2;
pub const hipMemAttachSingle: u32 = 4;
pub const hipDeviceMallocDefault: u32 = 0;
pub const hipDeviceMallocFinegrained: u32 = 1;
pub const hipMallocSignalMemory: u32 = 2;
pub const hipDeviceMallocUncached: u32 = 3;
pub const hipDeviceMallocContiguous: u32 = 4;
pub const hipHostRegisterDefault: u32 = 0;
pub const hipHostRegisterPortable: u32 = 1;
pub const hipHostRegisterMapped: u32 = 2;
pub const hipHostRegisterIoMemory: u32 = 4;
pub const hipHostRegisterReadOnly: u32 = 8;
pub const hipExtHostRegisterCoarseGrained: u32 = 8;
pub const hipDeviceScheduleAuto: u32 = 0;
pub const hipDeviceScheduleSpin: u32 = 1;
pub const hipDeviceScheduleYield: u32 = 2;
pub const hipDeviceScheduleBlockingSync: u32 = 4;
pub const hipDeviceScheduleMask: u32 = 7;
pub const hipDeviceMapHost: u32 = 8;
pub const hipDeviceLmemResizeToMax: u32 = 16;
pub const hipArrayDefault: u32 = 0;
pub const hipArrayLayered: u32 = 1;
pub const hipArraySurfaceLoadStore: u32 = 2;
pub const hipArrayCubemap: u32 = 4;
pub const hipArrayTextureGather: u32 = 8;
pub const hipOccupancyDefault: u32 = 0;
pub const hipOccupancyDisableCachingOverride: u32 = 1;
pub const hipCooperativeLaunchMultiDeviceNoPreSync: u32 = 1;
pub const hipCooperativeLaunchMultiDeviceNoPostSync: u32 = 2;
pub const hipExtAnyOrderLaunch: u32 = 1;
pub const hipStreamWaitValueGte: u32 = 0;
pub const hipStreamWaitValueEq: u32 = 1;
pub const hipStreamWaitValueAnd: u32 = 2;
pub const hipStreamWaitValueNor: u32 = 3;
pub const hipExternalMemoryDedicated: u32 = 1;
pub const hipGraphKernelNodePortDefault: u32 = 0;
pub const hipGraphKernelNodePortLaunchCompletion: u32 = 2;
pub const hipGraphKernelNodePortProgrammatic: u32 = 1;
impl hipJitOption {
    /**< CUDA Only Maximum registers may be used in a thread,
< passed to compiler*/
    pub const hipJitOptionMaxRegisters: hipJitOption = hipJitOption(0);
}
impl hipJitOption {
    ///< CUDA Only Number of thread per block
    pub const hipJitOptionThreadsPerBlock: hipJitOption = hipJitOption(1);
}
impl hipJitOption {
    ///< CUDA Only Value for total wall clock time
    pub const hipJitOptionWallTime: hipJitOption = hipJitOption(2);
}
impl hipJitOption {
    ///< CUDA Only Pointer to the buffer with logged information
    pub const hipJitOptionInfoLogBuffer: hipJitOption = hipJitOption(3);
}
impl hipJitOption {
    ///< CUDA Only Size of the buffer in bytes for logged info
    pub const hipJitOptionInfoLogBufferSizeBytes: hipJitOption = hipJitOption(4);
}
impl hipJitOption {
    ///< CUDA Only Pointer to the buffer with logged error(s)
    pub const hipJitOptionErrorLogBuffer: hipJitOption = hipJitOption(5);
}
impl hipJitOption {
    ///< CUDA Only Size of the buffer in bytes for logged error(s)
    pub const hipJitOptionErrorLogBufferSizeBytes: hipJitOption = hipJitOption(6);
}
impl hipJitOption {
    /**< Value of optimization level for generated codes, acceptable options
< -O0, -O1, -O2, -O3*/
    pub const hipJitOptionOptimizationLevel: hipJitOption = hipJitOption(7);
}
impl hipJitOption {
    ///< CUDA Only The target context, which is the default
    pub const hipJitOptionTargetFromContext: hipJitOption = hipJitOption(8);
}
impl hipJitOption {
    ///< CUDA Only JIT target
    pub const hipJitOptionTarget: hipJitOption = hipJitOption(9);
}
impl hipJitOption {
    ///< CUDA Only Fallback strategy
    pub const hipJitOptionFallbackStrategy: hipJitOption = hipJitOption(10);
}
impl hipJitOption {
    ///< CUDA Only Generate debug information
    pub const hipJitOptionGenerateDebugInfo: hipJitOption = hipJitOption(11);
}
impl hipJitOption {
    ///< CUDA Only Generate log verbose
    pub const hipJitOptionLogVerbose: hipJitOption = hipJitOption(12);
}
impl hipJitOption {
    ///< CUDA Only Generate line number information
    pub const hipJitOptionGenerateLineInfo: hipJitOption = hipJitOption(13);
}
impl hipJitOption {
    ///< CUDA Only Set cache mode
    pub const hipJitOptionCacheMode: hipJitOption = hipJitOption(14);
}
impl hipJitOption {
    ///< @deprecated CUDA Only New SM3X option.
    pub const hipJitOptionSm3xOpt: hipJitOption = hipJitOption(15);
}
impl hipJitOption {
    ///< CUDA Only Set fast compile
    pub const hipJitOptionFastCompile: hipJitOption = hipJitOption(16);
}
impl hipJitOption {
    ///< CUDA Only Array of device symbol names to be relocated to the host
    pub const hipJitOptionGlobalSymbolNames: hipJitOption = hipJitOption(17);
}
impl hipJitOption {
    ///< CUDA Only Array of host addresses to be relocated to the device
    pub const hipJitOptionGlobalSymbolAddresses: hipJitOption = hipJitOption(18);
}
impl hipJitOption {
    ///< CUDA Only Number of symbol count.
    pub const hipJitOptionGlobalSymbolCount: hipJitOption = hipJitOption(19);
}
impl hipJitOption {
    ///< @deprecated CUDA Only Enable link-time optimization for device code
    pub const hipJitOptionLto: hipJitOption = hipJitOption(20);
}
impl hipJitOption {
    ///< @deprecated CUDA Only Set single-precision denormals.
    pub const hipJitOptionFtz: hipJitOption = hipJitOption(21);
}
impl hipJitOption {
    /**< @deprecated CUDA Only Set single-precision floating-point division
< and reciprocals*/
    pub const hipJitOptionPrecDiv: hipJitOption = hipJitOption(22);
}
impl hipJitOption {
    ///< @deprecated CUDA Only Set single-precision floating-point square root
    pub const hipJitOptionPrecSqrt: hipJitOption = hipJitOption(23);
}
impl hipJitOption {
    /**< @deprecated CUDA Only Enable floating-point multiplies and
< adds/subtracts operations*/
    pub const hipJitOptionFma: hipJitOption = hipJitOption(24);
}
impl hipJitOption {
    ///< CUDA Only Generates Position Independent code
    pub const hipJitOptionPositionIndependentCode: hipJitOption = hipJitOption(25);
}
impl hipJitOption {
    /**< CUDA Only Hints to JIT compiler the minimum number of CTAs frin kernel's
< grid to be mapped to SM*/
    pub const hipJitOptionMinCTAPerSM: hipJitOption = hipJitOption(26);
}
impl hipJitOption {
    ///< CUDA only Maximum number of threads in a thread block
    pub const hipJitOptionMaxThreadsPerBlock: hipJitOption = hipJitOption(27);
}
impl hipJitOption {
    ///< Cuda only Override Directive values
    pub const hipJitOptionOverrideDirectiveValues: hipJitOption = hipJitOption(28);
}
impl hipJitOption {
    ///< Number of options
    pub const hipJitOptionNumOptions: hipJitOption = hipJitOption(29);
}
impl hipJitOption {
    ///< Hip Only Linker options to be passed on to compiler
    pub const hipJitOptionIRtoISAOptExt: hipJitOption = hipJitOption(10000);
}
impl hipJitOption {
    ///< Hip Only Count of linker options to be passed on to compiler
    pub const hipJitOptionIRtoISAOptCountExt: hipJitOption = hipJitOption(10001);
}
#[repr(transparent)]
/// hipJitOption
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipJitOption(pub ::core::ffi::c_uint);
impl hipJitInputType {
    ///< Cuda only Input cubin
    pub const hipJitInputCubin: hipJitInputType = hipJitInputType(0);
}
impl hipJitInputType {
    ///< Cuda only Input PTX
    pub const hipJitInputPtx: hipJitInputType = hipJitInputType(1);
}
impl hipJitInputType {
    ///< Cuda Only Input FAT Binary
    pub const hipJitInputFatBinary: hipJitInputType = hipJitInputType(2);
}
impl hipJitInputType {
    ///< Cuda Only Host Object with embedded device code
    pub const hipJitInputObject: hipJitInputType = hipJitInputType(3);
}
impl hipJitInputType {
    /**< Cuda Only Archive of Host Objects with embedded
< device code*/
    pub const hipJitInputLibrary: hipJitInputType = hipJitInputType(4);
}
impl hipJitInputType {
    /**< @deprecated Cuda only High Level intermediate
< code for LTO*/
    pub const hipJitInputNvvm: hipJitInputType = hipJitInputType(5);
}
impl hipJitInputType {
    ///< Count of Legacy Input Types
    pub const hipJitNumLegacyInputTypes: hipJitInputType = hipJitInputType(6);
}
impl hipJitInputType {
    ///< HIP Only LLVM Bitcode or IR assembly
    pub const hipJitInputLLVMBitcode: hipJitInputType = hipJitInputType(100);
}
impl hipJitInputType {
    ///< HIP Only LLVM Clang Bundled Code
    pub const hipJitInputLLVMBundledBitcode: hipJitInputType = hipJitInputType(101);
}
impl hipJitInputType {
    ///< HIP Only LLVM Archive of Bundled Bitcode
    pub const hipJitInputLLVMArchivesOfBundledBitcode: hipJitInputType = hipJitInputType(
        102,
    );
}
impl hipJitInputType {
    ///< HIP Only SPIRV Code Object
    pub const hipJitInputSpirv: hipJitInputType = hipJitInputType(103);
}
impl hipJitInputType {
    ///< Count of Input Types
    pub const hipJitNumInputTypes: hipJitInputType = hipJitInputType(10);
}
#[repr(transparent)]
/// hipJitInputType
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipJitInputType(pub ::core::ffi::c_uint);
impl hipJitCacheMode {
    pub const hipJitCacheOptionNone: hipJitCacheMode = hipJitCacheMode(0);
}
impl hipJitCacheMode {
    pub const hipJitCacheOptionCG: hipJitCacheMode = hipJitCacheMode(1);
}
impl hipJitCacheMode {
    pub const hipJitCacheOptionCA: hipJitCacheMode = hipJitCacheMode(2);
}
#[repr(transparent)]
/// hipJitCacheMode
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipJitCacheMode(pub ::core::ffi::c_uint);
impl hipJitFallback {
    pub const hipJitPreferPTX: hipJitFallback = hipJitFallback(0);
}
impl hipJitFallback {
    pub const hipJitPreferBinary: hipJitFallback = hipJitFallback(1);
}
#[repr(transparent)]
/// hipJitFallback
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipJitFallback(pub ::core::ffi::c_uint);
#[doc = " @defgroup GlobalDefs Global enum and defines\n @{\n\n/\n/**\n hipDeviceArch_t\n"]
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipDeviceArch_t {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 3usize]>,
    pub __bindgen_padding_0: u8,
}
impl hipDeviceArch_t {
    #[inline]
    pub fn hasGlobalInt32Atomics(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasGlobalInt32Atomics(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasGlobalFloatAtomicExch(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasGlobalFloatAtomicExch(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasSharedInt32Atomics(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasSharedInt32Atomics(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasSharedFloatAtomicExch(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasSharedFloatAtomicExch(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasFloatAtomicAdd(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasFloatAtomicAdd(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasGlobalInt64Atomics(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasGlobalInt64Atomics(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasSharedInt64Atomics(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasSharedInt64Atomics(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(6usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasDoubles(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasDoubles(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasWarpVote(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasWarpVote(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(8usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasWarpBallot(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(9usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasWarpBallot(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(9usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasWarpShuffle(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(10usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasWarpShuffle(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(10usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasFunnelShift(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(11usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasFunnelShift(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(11usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasThreadFenceSystem(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(12usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasThreadFenceSystem(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(12usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasSyncThreadsExt(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(13usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasSyncThreadsExt(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(13usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasSurfaceFuncs(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(14usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasSurfaceFuncs(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(14usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn has3dGrid(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(15usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_has3dGrid(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(15usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn hasDynamicParallelism(&self) -> ::core::ffi::c_uint {
        unsafe { ::core::mem::transmute(self._bitfield_1.get(16usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_hasDynamicParallelism(&mut self, val: ::core::ffi::c_uint) {
        unsafe {
            let val: u32 = ::core::mem::transmute(val);
            self._bitfield_1.set(16usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        hasGlobalInt32Atomics: ::core::ffi::c_uint,
        hasGlobalFloatAtomicExch: ::core::ffi::c_uint,
        hasSharedInt32Atomics: ::core::ffi::c_uint,
        hasSharedFloatAtomicExch: ::core::ffi::c_uint,
        hasFloatAtomicAdd: ::core::ffi::c_uint,
        hasGlobalInt64Atomics: ::core::ffi::c_uint,
        hasSharedInt64Atomics: ::core::ffi::c_uint,
        hasDoubles: ::core::ffi::c_uint,
        hasWarpVote: ::core::ffi::c_uint,
        hasWarpBallot: ::core::ffi::c_uint,
        hasWarpShuffle: ::core::ffi::c_uint,
        hasFunnelShift: ::core::ffi::c_uint,
        hasThreadFenceSystem: ::core::ffi::c_uint,
        hasSyncThreadsExt: ::core::ffi::c_uint,
        hasSurfaceFuncs: ::core::ffi::c_uint,
        has3dGrid: ::core::ffi::c_uint,
        hasDynamicParallelism: ::core::ffi::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 3usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 3usize]> = Default::default();
        __bindgen_bitfield_unit
            .set(
                0usize,
                1u8,
                {
                    let hasGlobalInt32Atomics: u32 = unsafe {
                        ::core::mem::transmute(hasGlobalInt32Atomics)
                    };
                    hasGlobalInt32Atomics as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                1usize,
                1u8,
                {
                    let hasGlobalFloatAtomicExch: u32 = unsafe {
                        ::core::mem::transmute(hasGlobalFloatAtomicExch)
                    };
                    hasGlobalFloatAtomicExch as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                2usize,
                1u8,
                {
                    let hasSharedInt32Atomics: u32 = unsafe {
                        ::core::mem::transmute(hasSharedInt32Atomics)
                    };
                    hasSharedInt32Atomics as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                3usize,
                1u8,
                {
                    let hasSharedFloatAtomicExch: u32 = unsafe {
                        ::core::mem::transmute(hasSharedFloatAtomicExch)
                    };
                    hasSharedFloatAtomicExch as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                4usize,
                1u8,
                {
                    let hasFloatAtomicAdd: u32 = unsafe {
                        ::core::mem::transmute(hasFloatAtomicAdd)
                    };
                    hasFloatAtomicAdd as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                5usize,
                1u8,
                {
                    let hasGlobalInt64Atomics: u32 = unsafe {
                        ::core::mem::transmute(hasGlobalInt64Atomics)
                    };
                    hasGlobalInt64Atomics as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                6usize,
                1u8,
                {
                    let hasSharedInt64Atomics: u32 = unsafe {
                        ::core::mem::transmute(hasSharedInt64Atomics)
                    };
                    hasSharedInt64Atomics as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                7usize,
                1u8,
                {
                    let hasDoubles: u32 = unsafe { ::core::mem::transmute(hasDoubles) };
                    hasDoubles as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                8usize,
                1u8,
                {
                    let hasWarpVote: u32 = unsafe {
                        ::core::mem::transmute(hasWarpVote)
                    };
                    hasWarpVote as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                9usize,
                1u8,
                {
                    let hasWarpBallot: u32 = unsafe {
                        ::core::mem::transmute(hasWarpBallot)
                    };
                    hasWarpBallot as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                10usize,
                1u8,
                {
                    let hasWarpShuffle: u32 = unsafe {
                        ::core::mem::transmute(hasWarpShuffle)
                    };
                    hasWarpShuffle as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                11usize,
                1u8,
                {
                    let hasFunnelShift: u32 = unsafe {
                        ::core::mem::transmute(hasFunnelShift)
                    };
                    hasFunnelShift as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                12usize,
                1u8,
                {
                    let hasThreadFenceSystem: u32 = unsafe {
                        ::core::mem::transmute(hasThreadFenceSystem)
                    };
                    hasThreadFenceSystem as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                13usize,
                1u8,
                {
                    let hasSyncThreadsExt: u32 = unsafe {
                        ::core::mem::transmute(hasSyncThreadsExt)
                    };
                    hasSyncThreadsExt as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                14usize,
                1u8,
                {
                    let hasSurfaceFuncs: u32 = unsafe {
                        ::core::mem::transmute(hasSurfaceFuncs)
                    };
                    hasSurfaceFuncs as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                15usize,
                1u8,
                {
                    let has3dGrid: u32 = unsafe { ::core::mem::transmute(has3dGrid) };
                    has3dGrid as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                16usize,
                1u8,
                {
                    let hasDynamicParallelism: u32 = unsafe {
                        ::core::mem::transmute(hasDynamicParallelism)
                    };
                    hasDynamicParallelism as u64
                },
            );
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipUUID_t {
    pub bytes: [::core::ffi::c_char; 16usize],
}
pub type hipUUID = hipUUID_t;
/** hipDeviceProp
*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipDeviceProp_tR0600 {
    ///< Device name.
    pub name: [::core::ffi::c_char; 256usize],
    ///< UUID of a device
    pub uuid: hipUUID,
    ///< 8-byte unique identifier. Only valid on windows
    pub luid: [::core::ffi::c_char; 8usize],
    ///< LUID node mask
    pub luidDeviceNodeMask: ::core::ffi::c_uint,
    ///< Size of global memory region (in bytes).
    pub totalGlobalMem: usize,
    ///< Size of shared memory per block (in bytes).
    pub sharedMemPerBlock: usize,
    ///< Registers per block.
    pub regsPerBlock: ::core::ffi::c_int,
    ///< Warp size.
    pub warpSize: ::core::ffi::c_int,
    /**< Maximum pitch in bytes allowed by memory copies
< pitched memory*/
    pub memPitch: usize,
    ///< Max work items per work group or workgroup max size.
    pub maxThreadsPerBlock: ::core::ffi::c_int,
    ///< Max number of threads in each dimension (XYZ) of a block.
    pub maxThreadsDim: [::core::ffi::c_int; 3usize],
    ///< Max grid dimensions (XYZ).
    pub maxGridSize: [::core::ffi::c_int; 3usize],
    ///< Max clock frequency of the multiProcessors in khz.
    pub clockRate: ::core::ffi::c_int,
    /**< Size of shared constant memory region on the device
< (in bytes).*/
    pub totalConstMem: usize,
    /**< Major compute capability.  On HCC, this is an approximation and features may
< differ from CUDA CC.  See the arch feature flags for portable ways to query
< feature caps.*/
    pub major: ::core::ffi::c_int,
    /**< Minor compute capability.  On HCC, this is an approximation and features may
< differ from CUDA CC.  See the arch feature flags for portable ways to query
< feature caps.*/
    pub minor: ::core::ffi::c_int,
    ///< Alignment requirement for textures
    pub textureAlignment: usize,
    ///< Pitch alignment requirement for texture references bound to
    pub texturePitchAlignment: usize,
    ///< Deprecated. Use asyncEngineCount instead
    pub deviceOverlap: ::core::ffi::c_int,
    ///< Number of multi-processors (compute units).
    pub multiProcessorCount: ::core::ffi::c_int,
    ///< Run time limit for kernels executed on the device
    pub kernelExecTimeoutEnabled: ::core::ffi::c_int,
    ///< APU vs dGPU
    pub integrated: ::core::ffi::c_int,
    ///< Check whether HIP can map host memory
    pub canMapHostMemory: ::core::ffi::c_int,
    ///< Compute mode.
    pub computeMode: ::core::ffi::c_int,
    ///< Maximum number of elements in 1D images
    pub maxTexture1D: ::core::ffi::c_int,
    ///< Maximum 1D mipmap texture size
    pub maxTexture1DMipmap: ::core::ffi::c_int,
    ///< Maximum size for 1D textures bound to linear memory
    pub maxTexture1DLinear: ::core::ffi::c_int,
    ///< Maximum dimensions (width, height) of 2D images, in image elements
    pub maxTexture2D: [::core::ffi::c_int; 2usize],
    ///< Maximum number of elements in 2D array mipmap of images
    pub maxTexture2DMipmap: [::core::ffi::c_int; 2usize],
    ///< Maximum 2D tex dimensions if tex are bound to pitched memory
    pub maxTexture2DLinear: [::core::ffi::c_int; 3usize],
    ///< Maximum 2D tex dimensions if gather has to be performed
    pub maxTexture2DGather: [::core::ffi::c_int; 2usize],
    /**< Maximum dimensions (width, height, depth) of 3D images, in image
< elements*/
    pub maxTexture3D: [::core::ffi::c_int; 3usize],
    ///< Maximum alternate 3D texture dims
    pub maxTexture3DAlt: [::core::ffi::c_int; 3usize],
    ///< Maximum cubemap texture dims
    pub maxTextureCubemap: ::core::ffi::c_int,
    ///< Maximum number of elements in 1D array images
    pub maxTexture1DLayered: [::core::ffi::c_int; 2usize],
    ///< Maximum number of elements in 2D array images
    pub maxTexture2DLayered: [::core::ffi::c_int; 3usize],
    ///< Maximum cubemaps layered texture dims
    pub maxTextureCubemapLayered: [::core::ffi::c_int; 2usize],
    ///< Maximum 1D surface size
    pub maxSurface1D: ::core::ffi::c_int,
    ///< Maximum 2D surface size
    pub maxSurface2D: [::core::ffi::c_int; 2usize],
    ///< Maximum 3D surface size
    pub maxSurface3D: [::core::ffi::c_int; 3usize],
    ///< Maximum 1D layered surface size
    pub maxSurface1DLayered: [::core::ffi::c_int; 2usize],
    ///< Maximum 2D layared surface size
    pub maxSurface2DLayered: [::core::ffi::c_int; 3usize],
    ///< Maximum cubemap surface size
    pub maxSurfaceCubemap: ::core::ffi::c_int,
    ///< Maximum cubemap layered surface size
    pub maxSurfaceCubemapLayered: [::core::ffi::c_int; 2usize],
    ///< Alignment requirement for surface
    pub surfaceAlignment: usize,
    ///< Device can possibly execute multiple kernels concurrently.
    pub concurrentKernels: ::core::ffi::c_int,
    ///< Device has ECC support enabled
    pub ECCEnabled: ::core::ffi::c_int,
    ///< PCI Bus ID.
    pub pciBusID: ::core::ffi::c_int,
    ///< PCI Device ID.
    pub pciDeviceID: ::core::ffi::c_int,
    ///< PCI Domain ID
    pub pciDomainID: ::core::ffi::c_int,
    ///< 1:If device is Tesla device using TCC driver, else 0
    pub tccDriver: ::core::ffi::c_int,
    ///< Number of async engines
    pub asyncEngineCount: ::core::ffi::c_int,
    ///< Does device and host share unified address space
    pub unifiedAddressing: ::core::ffi::c_int,
    ///< Max global memory clock frequency in khz.
    pub memoryClockRate: ::core::ffi::c_int,
    ///< Global memory bus width in bits.
    pub memoryBusWidth: ::core::ffi::c_int,
    ///< L2 cache size.
    pub l2CacheSize: ::core::ffi::c_int,
    ///< Device's max L2 persisting lines in bytes
    pub persistingL2CacheMaxSize: ::core::ffi::c_int,
    ///< Maximum resident threads per multi-processor.
    pub maxThreadsPerMultiProcessor: ::core::ffi::c_int,
    ///< Device supports stream priority
    pub streamPrioritiesSupported: ::core::ffi::c_int,
    ///< Indicates globals are cached in L1
    pub globalL1CacheSupported: ::core::ffi::c_int,
    ///< Locals are cahced in L1
    pub localL1CacheSupported: ::core::ffi::c_int,
    ///< Amount of shared memory available per multiprocessor.
    pub sharedMemPerMultiprocessor: usize,
    ///< registers available per multiprocessor
    pub regsPerMultiprocessor: ::core::ffi::c_int,
    ///< Device supports allocating managed memory on this system
    pub managedMemory: ::core::ffi::c_int,
    ///< 1 if device is on a multi-GPU board, 0 if not.
    pub isMultiGpuBoard: ::core::ffi::c_int,
    ///< Unique identifier for a group of devices on same multiboard GPU
    pub multiGpuBoardGroupID: ::core::ffi::c_int,
    ///< Link between host and device supports native atomics
    pub hostNativeAtomicSupported: ::core::ffi::c_int,
    ///< Deprecated. CUDA only.
    pub singleToDoublePrecisionPerfRatio: ::core::ffi::c_int,
    /**< Device supports coherently accessing pageable memory
< without calling hipHostRegister on it*/
    pub pageableMemoryAccess: ::core::ffi::c_int,
    /**< Device can coherently access managed memory concurrently with
< the CPU*/
    pub concurrentManagedAccess: ::core::ffi::c_int,
    ///< Is compute preemption supported on the device
    pub computePreemptionSupported: ::core::ffi::c_int,
    /**< Device can access host registered memory with same
< address as the host*/
    pub canUseHostPointerForRegisteredMem: ::core::ffi::c_int,
    ///< HIP device supports cooperative launch
    pub cooperativeLaunch: ::core::ffi::c_int,
    /**< HIP device supports cooperative launch on multiple
< devices*/
    pub cooperativeMultiDeviceLaunch: ::core::ffi::c_int,
    ///< Per device m ax shared mem per block usable by special opt in
    pub sharedMemPerBlockOptin: usize,
    /**< Device accesses pageable memory via the host's
< page tables*/
    pub pageableMemoryAccessUsesHostPageTables: ::core::ffi::c_int,
    /**< Host can directly access managed memory on the device
< without migration*/
    pub directManagedMemAccessFromHost: ::core::ffi::c_int,
    ///< Max number of blocks on CU
    pub maxBlocksPerMultiProcessor: ::core::ffi::c_int,
    ///< Max value of access policy window
    pub accessPolicyMaxWindowSize: ::core::ffi::c_int,
    ///< Shared memory reserved by driver per block
    pub reservedSharedMemPerBlock: usize,
    ///< Device supports hipHostRegister
    pub hostRegisterSupported: ::core::ffi::c_int,
    ///< Indicates if device supports sparse hip arrays
    pub sparseHipArraySupported: ::core::ffi::c_int,
    /**< Device supports using the hipHostRegisterReadOnly flag
< with hipHostRegistger*/
    pub hostRegisterReadOnlySupported: ::core::ffi::c_int,
    ///< Indicates external timeline semaphore support
    pub timelineSemaphoreInteropSupported: ::core::ffi::c_int,
    ///< Indicates if device supports hipMallocAsync and hipMemPool APIs
    pub memoryPoolsSupported: ::core::ffi::c_int,
    ///< Indicates device support of RDMA APIs
    pub gpuDirectRDMASupported: ::core::ffi::c_int,
    /**< Bitmask to be interpreted according to
< hipFlushGPUDirectRDMAWritesOptions*/
    pub gpuDirectRDMAFlushWritesOptions: ::core::ffi::c_uint,
    ///< value of hipGPUDirectRDMAWritesOrdering
    pub gpuDirectRDMAWritesOrdering: ::core::ffi::c_int,
    ///< Bitmask of handle types support with mempool based IPC
    pub memoryPoolSupportedHandleTypes: ::core::ffi::c_uint,
    /**< Device supports deferred mapping HIP arrays and HIP
< mipmapped arrays*/
    pub deferredMappingHipArraySupported: ::core::ffi::c_int,
    ///< Device supports IPC events
    pub ipcEventSupported: ::core::ffi::c_int,
    ///< Device supports cluster launch
    pub clusterLaunch: ::core::ffi::c_int,
    ///< Indicates device supports unified function pointers
    pub unifiedFunctionPointers: ::core::ffi::c_int,
    ///< CUDA Reserved.
    pub reserved: [::core::ffi::c_int; 63usize],
    ///< Reserved for adding new entries for HIP/CUDA.
    pub hipReserved: [::core::ffi::c_int; 32usize],
    ///< AMD GCN Arch Name. HIP Only.
    pub gcnArchName: [::core::ffi::c_char; 256usize],
    ///< Maximum Shared Memory Per CU. HIP Only.
    pub maxSharedMemoryPerMultiProcessor: usize,
    /**< Frequency in khz of the timer used by the device-side "clock*"
< instructions.  New for HIP.*/
    pub clockInstructionRate: ::core::ffi::c_int,
    ///< Architectural feature flags.  New for HIP.
    pub arch: hipDeviceArch_t,
    ///< Addres of HDP_MEM_COHERENCY_FLUSH_CNTL register
    pub hdpMemFlushCntl: *mut ::core::ffi::c_uint,
    ///< Addres of HDP_REG_COHERENCY_FLUSH_CNTL register
    pub hdpRegFlushCntl: *mut ::core::ffi::c_uint,
    /**< HIP device supports cooperative launch on
< multiple*/
    pub cooperativeMultiDeviceUnmatchedFunc: ::core::ffi::c_int,
    /**< HIP device supports cooperative launch on
< multiple*/
    pub cooperativeMultiDeviceUnmatchedGridDim: ::core::ffi::c_int,
    /**< HIP device supports cooperative launch on
< multiple*/
    pub cooperativeMultiDeviceUnmatchedBlockDim: ::core::ffi::c_int,
    /**< HIP device supports cooperative launch on
< multiple*/
    pub cooperativeMultiDeviceUnmatchedSharedMem: ::core::ffi::c_int,
    ///< 1: if it is a large PCI bar device, else 0
    pub isLargeBar: ::core::ffi::c_int,
    ///< Revision of the GPU in this device
    pub asicRevision: ::core::ffi::c_int,
}
impl hipMemoryType {
    ///< Unregistered memory
    pub const hipMemoryTypeUnregistered: hipMemoryType = hipMemoryType(0);
}
impl hipMemoryType {
    ///< Memory is physically located on host
    pub const hipMemoryTypeHost: hipMemoryType = hipMemoryType(1);
}
impl hipMemoryType {
    /**< Memory is physically located on device. (see deviceId for
< specific device)*/
    pub const hipMemoryTypeDevice: hipMemoryType = hipMemoryType(2);
}
impl hipMemoryType {
    /**< Managed memory, automaticallly managed by the unified
< memory system
< place holder for new values.*/
    pub const hipMemoryTypeManaged: hipMemoryType = hipMemoryType(3);
}
impl hipMemoryType {
    /**< Array memory, physically located on device. (see deviceId for
< specific device)*/
    pub const hipMemoryTypeArray: hipMemoryType = hipMemoryType(10);
}
impl hipMemoryType {
    ///< unified address space
    pub const hipMemoryTypeUnified: hipMemoryType = hipMemoryType(11);
}
#[repr(transparent)]
/** hipMemoryType (for pointer attributes)

 @note hipMemoryType enum values are combination of cudaMemoryType and cuMemoryType and AMD specific enum values.
*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipMemoryType(pub ::core::ffi::c_uint);
/// Pointer attributes
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipPointerAttribute_t {
    pub type_: hipMemoryType,
    pub device: ::core::ffi::c_int,
    pub devicePointer: *mut ::core::ffi::c_void,
    pub hostPointer: *mut ::core::ffi::c_void,
    pub isManaged: ::core::ffi::c_int,
    pub allocationFlags: ::core::ffi::c_uint,
}
impl hipDeviceAttribute_t {
    pub const hipDeviceAttributeCudaCompatibleBegin: hipDeviceAttribute_t = hipDeviceAttribute_t(
        0,
    );
}
impl hipDeviceAttribute_t {
    ///< Whether ECC support is enabled.
    pub const hipDeviceAttributeEccEnabled: hipDeviceAttribute_t = hipDeviceAttribute_t(
        0,
    );
}
impl hipDeviceAttribute_t {
    ///< Cuda only. The maximum size of the window policy in bytes.
    pub const hipDeviceAttributeAccessPolicyMaxWindowSize: hipDeviceAttribute_t = hipDeviceAttribute_t(
        1,
    );
}
impl hipDeviceAttribute_t {
    ///< Asynchronous engines number.
    pub const hipDeviceAttributeAsyncEngineCount: hipDeviceAttribute_t = hipDeviceAttribute_t(
        2,
    );
}
impl hipDeviceAttribute_t {
    ///< Whether host memory can be mapped into device address space
    pub const hipDeviceAttributeCanMapHostMemory: hipDeviceAttribute_t = hipDeviceAttribute_t(
        3,
    );
}
impl hipDeviceAttribute_t {
    /**< Device can access host registered memory
< at the same virtual address as the CPU*/
    pub const hipDeviceAttributeCanUseHostPointerForRegisteredMem: hipDeviceAttribute_t = hipDeviceAttribute_t(
        4,
    );
}
impl hipDeviceAttribute_t {
    ///< Peak clock frequency in kilohertz.
    pub const hipDeviceAttributeClockRate: hipDeviceAttribute_t = hipDeviceAttribute_t(
        5,
    );
}
impl hipDeviceAttribute_t {
    ///< Compute mode that device is currently in.
    pub const hipDeviceAttributeComputeMode: hipDeviceAttribute_t = hipDeviceAttribute_t(
        6,
    );
}
impl hipDeviceAttribute_t {
    ///< Device supports Compute Preemption.
    pub const hipDeviceAttributeComputePreemptionSupported: hipDeviceAttribute_t = hipDeviceAttribute_t(
        7,
    );
}
impl hipDeviceAttribute_t {
    ///< Device can possibly execute multiple kernels concurrently.
    pub const hipDeviceAttributeConcurrentKernels: hipDeviceAttribute_t = hipDeviceAttribute_t(
        8,
    );
}
impl hipDeviceAttribute_t {
    ///< Device can coherently access managed memory concurrently with the CPU
    pub const hipDeviceAttributeConcurrentManagedAccess: hipDeviceAttribute_t = hipDeviceAttribute_t(
        9,
    );
}
impl hipDeviceAttribute_t {
    ///< Support cooperative launch
    pub const hipDeviceAttributeCooperativeLaunch: hipDeviceAttribute_t = hipDeviceAttribute_t(
        10,
    );
}
impl hipDeviceAttribute_t {
    ///< Support cooperative launch on multiple devices
    pub const hipDeviceAttributeCooperativeMultiDeviceLaunch: hipDeviceAttribute_t = hipDeviceAttribute_t(
        11,
    );
}
impl hipDeviceAttribute_t {
    /**< Device can concurrently copy memory and execute a kernel.
< Deprecated. Use instead asyncEngineCount.*/
    pub const hipDeviceAttributeDeviceOverlap: hipDeviceAttribute_t = hipDeviceAttribute_t(
        12,
    );
}
impl hipDeviceAttribute_t {
    /**< Host can directly access managed memory on
< the device without migration*/
    pub const hipDeviceAttributeDirectManagedMemAccessFromHost: hipDeviceAttribute_t = hipDeviceAttribute_t(
        13,
    );
}
impl hipDeviceAttribute_t {
    ///< Device supports caching globals in L1
    pub const hipDeviceAttributeGlobalL1CacheSupported: hipDeviceAttribute_t = hipDeviceAttribute_t(
        14,
    );
}
impl hipDeviceAttribute_t {
    ///< Link between the device and the host supports native atomic operations
    pub const hipDeviceAttributeHostNativeAtomicSupported: hipDeviceAttribute_t = hipDeviceAttribute_t(
        15,
    );
}
impl hipDeviceAttribute_t {
    ///< Device is integrated GPU
    pub const hipDeviceAttributeIntegrated: hipDeviceAttribute_t = hipDeviceAttribute_t(
        16,
    );
}
impl hipDeviceAttribute_t {
    ///< Multiple GPU devices.
    pub const hipDeviceAttributeIsMultiGpuBoard: hipDeviceAttribute_t = hipDeviceAttribute_t(
        17,
    );
}
impl hipDeviceAttribute_t {
    ///< Run time limit for kernels executed on the device
    pub const hipDeviceAttributeKernelExecTimeout: hipDeviceAttribute_t = hipDeviceAttribute_t(
        18,
    );
}
impl hipDeviceAttribute_t {
    ///< Size of L2 cache in bytes. 0 if the device doesn't have L2 cache.
    pub const hipDeviceAttributeL2CacheSize: hipDeviceAttribute_t = hipDeviceAttribute_t(
        19,
    );
}
impl hipDeviceAttribute_t {
    ///< caching locals in L1 is supported
    pub const hipDeviceAttributeLocalL1CacheSupported: hipDeviceAttribute_t = hipDeviceAttribute_t(
        20,
    );
}
impl hipDeviceAttribute_t {
    ///< 8-byte locally unique identifier in 8 bytes. Undefined on TCC and non-Windows platforms
    pub const hipDeviceAttributeLuid: hipDeviceAttribute_t = hipDeviceAttribute_t(21);
}
impl hipDeviceAttribute_t {
    ///< Luid device node mask. Undefined on TCC and non-Windows platforms
    pub const hipDeviceAttributeLuidDeviceNodeMask: hipDeviceAttribute_t = hipDeviceAttribute_t(
        22,
    );
}
impl hipDeviceAttribute_t {
    ///< Major compute capability version number.
    pub const hipDeviceAttributeComputeCapabilityMajor: hipDeviceAttribute_t = hipDeviceAttribute_t(
        23,
    );
}
impl hipDeviceAttribute_t {
    ///< Device supports allocating managed memory on this system
    pub const hipDeviceAttributeManagedMemory: hipDeviceAttribute_t = hipDeviceAttribute_t(
        24,
    );
}
impl hipDeviceAttribute_t {
    ///< Max block size per multiprocessor
    pub const hipDeviceAttributeMaxBlocksPerMultiProcessor: hipDeviceAttribute_t = hipDeviceAttribute_t(
        25,
    );
}
impl hipDeviceAttribute_t {
    ///< Max block size in width.
    pub const hipDeviceAttributeMaxBlockDimX: hipDeviceAttribute_t = hipDeviceAttribute_t(
        26,
    );
}
impl hipDeviceAttribute_t {
    ///< Max block size in height.
    pub const hipDeviceAttributeMaxBlockDimY: hipDeviceAttribute_t = hipDeviceAttribute_t(
        27,
    );
}
impl hipDeviceAttribute_t {
    ///< Max block size in depth.
    pub const hipDeviceAttributeMaxBlockDimZ: hipDeviceAttribute_t = hipDeviceAttribute_t(
        28,
    );
}
impl hipDeviceAttribute_t {
    ///< Max grid size  in width.
    pub const hipDeviceAttributeMaxGridDimX: hipDeviceAttribute_t = hipDeviceAttribute_t(
        29,
    );
}
impl hipDeviceAttribute_t {
    ///< Max grid size  in height.
    pub const hipDeviceAttributeMaxGridDimY: hipDeviceAttribute_t = hipDeviceAttribute_t(
        30,
    );
}
impl hipDeviceAttribute_t {
    ///< Max grid size  in depth.
    pub const hipDeviceAttributeMaxGridDimZ: hipDeviceAttribute_t = hipDeviceAttribute_t(
        31,
    );
}
impl hipDeviceAttribute_t {
    ///< Maximum size of 1D surface.
    pub const hipDeviceAttributeMaxSurface1D: hipDeviceAttribute_t = hipDeviceAttribute_t(
        32,
    );
}
impl hipDeviceAttribute_t {
    ///< Cuda only. Maximum dimensions of 1D layered surface.
    pub const hipDeviceAttributeMaxSurface1DLayered: hipDeviceAttribute_t = hipDeviceAttribute_t(
        33,
    );
}
impl hipDeviceAttribute_t {
    ///< Maximum dimension (width, height) of 2D surface.
    pub const hipDeviceAttributeMaxSurface2D: hipDeviceAttribute_t = hipDeviceAttribute_t(
        34,
    );
}
impl hipDeviceAttribute_t {
    ///< Cuda only. Maximum dimensions of 2D layered surface.
    pub const hipDeviceAttributeMaxSurface2DLayered: hipDeviceAttribute_t = hipDeviceAttribute_t(
        35,
    );
}
impl hipDeviceAttribute_t {
    ///< Maximum dimension (width, height, depth) of 3D surface.
    pub const hipDeviceAttributeMaxSurface3D: hipDeviceAttribute_t = hipDeviceAttribute_t(
        36,
    );
}
impl hipDeviceAttribute_t {
    ///< Cuda only. Maximum dimensions of Cubemap surface.
    pub const hipDeviceAttributeMaxSurfaceCubemap: hipDeviceAttribute_t = hipDeviceAttribute_t(
        37,
    );
}
impl hipDeviceAttribute_t {
    ///< Cuda only. Maximum dimension of Cubemap layered surface.
    pub const hipDeviceAttributeMaxSurfaceCubemapLayered: hipDeviceAttribute_t = hipDeviceAttribute_t(
        38,
    );
}
impl hipDeviceAttribute_t {
    ///< Maximum size of 1D texture.
    pub const hipDeviceAttributeMaxTexture1DWidth: hipDeviceAttribute_t = hipDeviceAttribute_t(
        39,
    );
}
impl hipDeviceAttribute_t {
    ///< Maximum dimensions of 1D layered texture.
    pub const hipDeviceAttributeMaxTexture1DLayered: hipDeviceAttribute_t = hipDeviceAttribute_t(
        40,
    );
}
impl hipDeviceAttribute_t {
    /**< Maximum number of elements allocatable in a 1D linear texture.
< Use cudaDeviceGetTexture1DLinearMaxWidth() instead on Cuda.*/
    pub const hipDeviceAttributeMaxTexture1DLinear: hipDeviceAttribute_t = hipDeviceAttribute_t(
        41,
    );
}
impl hipDeviceAttribute_t {
    ///< Maximum size of 1D mipmapped texture.
    pub const hipDeviceAttributeMaxTexture1DMipmap: hipDeviceAttribute_t = hipDeviceAttribute_t(
        42,
    );
}
impl hipDeviceAttribute_t {
    ///< Maximum dimension width of 2D texture.
    pub const hipDeviceAttributeMaxTexture2DWidth: hipDeviceAttribute_t = hipDeviceAttribute_t(
        43,
    );
}
impl hipDeviceAttribute_t {
    ///< Maximum dimension hight of 2D texture.
    pub const hipDeviceAttributeMaxTexture2DHeight: hipDeviceAttribute_t = hipDeviceAttribute_t(
        44,
    );
}
impl hipDeviceAttribute_t {
    ///< Maximum dimensions of 2D texture if gather operations  performed.
    pub const hipDeviceAttributeMaxTexture2DGather: hipDeviceAttribute_t = hipDeviceAttribute_t(
        45,
    );
}
impl hipDeviceAttribute_t {
    ///< Maximum dimensions of 2D layered texture.
    pub const hipDeviceAttributeMaxTexture2DLayered: hipDeviceAttribute_t = hipDeviceAttribute_t(
        46,
    );
}
impl hipDeviceAttribute_t {
    ///< Maximum dimensions (width, height, pitch) of 2D textures bound to pitched memory.
    pub const hipDeviceAttributeMaxTexture2DLinear: hipDeviceAttribute_t = hipDeviceAttribute_t(
        47,
    );
}
impl hipDeviceAttribute_t {
    ///< Maximum dimensions of 2D mipmapped texture.
    pub const hipDeviceAttributeMaxTexture2DMipmap: hipDeviceAttribute_t = hipDeviceAttribute_t(
        48,
    );
}
impl hipDeviceAttribute_t {
    ///< Maximum dimension width of 3D texture.
    pub const hipDeviceAttributeMaxTexture3DWidth: hipDeviceAttribute_t = hipDeviceAttribute_t(
        49,
    );
}
impl hipDeviceAttribute_t {
    ///< Maximum dimension height of 3D texture.
    pub const hipDeviceAttributeMaxTexture3DHeight: hipDeviceAttribute_t = hipDeviceAttribute_t(
        50,
    );
}
impl hipDeviceAttribute_t {
    ///< Maximum dimension depth of 3D texture.
    pub const hipDeviceAttributeMaxTexture3DDepth: hipDeviceAttribute_t = hipDeviceAttribute_t(
        51,
    );
}
impl hipDeviceAttribute_t {
    ///< Maximum dimensions of alternate 3D texture.
    pub const hipDeviceAttributeMaxTexture3DAlt: hipDeviceAttribute_t = hipDeviceAttribute_t(
        52,
    );
}
impl hipDeviceAttribute_t {
    ///< Maximum dimensions of Cubemap texture
    pub const hipDeviceAttributeMaxTextureCubemap: hipDeviceAttribute_t = hipDeviceAttribute_t(
        53,
    );
}
impl hipDeviceAttribute_t {
    ///< Maximum dimensions of Cubemap layered texture.
    pub const hipDeviceAttributeMaxTextureCubemapLayered: hipDeviceAttribute_t = hipDeviceAttribute_t(
        54,
    );
}
impl hipDeviceAttribute_t {
    ///< Maximum dimension of a block
    pub const hipDeviceAttributeMaxThreadsDim: hipDeviceAttribute_t = hipDeviceAttribute_t(
        55,
    );
}
impl hipDeviceAttribute_t {
    ///< Maximum number of threads per block.
    pub const hipDeviceAttributeMaxThreadsPerBlock: hipDeviceAttribute_t = hipDeviceAttribute_t(
        56,
    );
}
impl hipDeviceAttribute_t {
    ///< Maximum resident threads per multiprocessor.
    pub const hipDeviceAttributeMaxThreadsPerMultiProcessor: hipDeviceAttribute_t = hipDeviceAttribute_t(
        57,
    );
}
impl hipDeviceAttribute_t {
    ///< Maximum pitch in bytes allowed by memory copies
    pub const hipDeviceAttributeMaxPitch: hipDeviceAttribute_t = hipDeviceAttribute_t(
        58,
    );
}
impl hipDeviceAttribute_t {
    ///< Global memory bus width in bits.
    pub const hipDeviceAttributeMemoryBusWidth: hipDeviceAttribute_t = hipDeviceAttribute_t(
        59,
    );
}
impl hipDeviceAttribute_t {
    ///< Peak memory clock frequency in kilohertz.
    pub const hipDeviceAttributeMemoryClockRate: hipDeviceAttribute_t = hipDeviceAttribute_t(
        60,
    );
}
impl hipDeviceAttribute_t {
    ///< Minor compute capability version number.
    pub const hipDeviceAttributeComputeCapabilityMinor: hipDeviceAttribute_t = hipDeviceAttribute_t(
        61,
    );
}
impl hipDeviceAttribute_t {
    ///< Unique ID of device group on the same multi-GPU board
    pub const hipDeviceAttributeMultiGpuBoardGroupID: hipDeviceAttribute_t = hipDeviceAttribute_t(
        62,
    );
}
impl hipDeviceAttribute_t {
    ///< Number of multiprocessors on the device.
    pub const hipDeviceAttributeMultiprocessorCount: hipDeviceAttribute_t = hipDeviceAttribute_t(
        63,
    );
}
impl hipDeviceAttribute_t {
    ///< Previously hipDeviceAttributeName
    pub const hipDeviceAttributeUnused1: hipDeviceAttribute_t = hipDeviceAttribute_t(64);
}
impl hipDeviceAttribute_t {
    /**< Device supports coherently accessing pageable memory
< without calling hipHostRegister on it*/
    pub const hipDeviceAttributePageableMemoryAccess: hipDeviceAttribute_t = hipDeviceAttribute_t(
        65,
    );
}
impl hipDeviceAttribute_t {
    ///< Device accesses pageable memory via the host's page tables
    pub const hipDeviceAttributePageableMemoryAccessUsesHostPageTables: hipDeviceAttribute_t = hipDeviceAttribute_t(
        66,
    );
}
impl hipDeviceAttribute_t {
    ///< PCI Bus ID.
    pub const hipDeviceAttributePciBusId: hipDeviceAttribute_t = hipDeviceAttribute_t(
        67,
    );
}
impl hipDeviceAttribute_t {
    ///< PCI Device ID.
    pub const hipDeviceAttributePciDeviceId: hipDeviceAttribute_t = hipDeviceAttribute_t(
        68,
    );
}
impl hipDeviceAttribute_t {
    ///< PCI Domain ID.
    pub const hipDeviceAttributePciDomainID: hipDeviceAttribute_t = hipDeviceAttribute_t(
        69,
    );
}
impl hipDeviceAttribute_t {
    ///< Maximum l2 persisting lines capacity in bytes
    pub const hipDeviceAttributePersistingL2CacheMaxSize: hipDeviceAttribute_t = hipDeviceAttribute_t(
        70,
    );
}
impl hipDeviceAttribute_t {
    /**< 32-bit registers available to a thread block. This number is shared
< by all thread blocks simultaneously resident on a multiprocessor.*/
    pub const hipDeviceAttributeMaxRegistersPerBlock: hipDeviceAttribute_t = hipDeviceAttribute_t(
        71,
    );
}
impl hipDeviceAttribute_t {
    ///< 32-bit registers available per block.
    pub const hipDeviceAttributeMaxRegistersPerMultiprocessor: hipDeviceAttribute_t = hipDeviceAttribute_t(
        72,
    );
}
impl hipDeviceAttribute_t {
    ///< Shared memory reserved by CUDA driver per block.
    pub const hipDeviceAttributeReservedSharedMemPerBlock: hipDeviceAttribute_t = hipDeviceAttribute_t(
        73,
    );
}
impl hipDeviceAttribute_t {
    ///< Maximum shared memory available per block in bytes.
    pub const hipDeviceAttributeMaxSharedMemoryPerBlock: hipDeviceAttribute_t = hipDeviceAttribute_t(
        74,
    );
}
impl hipDeviceAttribute_t {
    ///< Maximum shared memory per block usable by special opt in.
    pub const hipDeviceAttributeSharedMemPerBlockOptin: hipDeviceAttribute_t = hipDeviceAttribute_t(
        75,
    );
}
impl hipDeviceAttribute_t {
    ///< Shared memory available per multiprocessor.
    pub const hipDeviceAttributeSharedMemPerMultiprocessor: hipDeviceAttribute_t = hipDeviceAttribute_t(
        76,
    );
}
impl hipDeviceAttribute_t {
    ///< Cuda only. Performance ratio of single precision to double precision.
    pub const hipDeviceAttributeSingleToDoublePrecisionPerfRatio: hipDeviceAttribute_t = hipDeviceAttribute_t(
        77,
    );
}
impl hipDeviceAttribute_t {
    ///< Whether to support stream priorities.
    pub const hipDeviceAttributeStreamPrioritiesSupported: hipDeviceAttribute_t = hipDeviceAttribute_t(
        78,
    );
}
impl hipDeviceAttribute_t {
    ///< Alignment requirement for surfaces
    pub const hipDeviceAttributeSurfaceAlignment: hipDeviceAttribute_t = hipDeviceAttribute_t(
        79,
    );
}
impl hipDeviceAttribute_t {
    ///< Cuda only. Whether device is a Tesla device using TCC driver
    pub const hipDeviceAttributeTccDriver: hipDeviceAttribute_t = hipDeviceAttribute_t(
        80,
    );
}
impl hipDeviceAttribute_t {
    ///< Alignment requirement for textures
    pub const hipDeviceAttributeTextureAlignment: hipDeviceAttribute_t = hipDeviceAttribute_t(
        81,
    );
}
impl hipDeviceAttribute_t {
    ///< Pitch alignment requirement for 2D texture references bound to pitched memory;
    pub const hipDeviceAttributeTexturePitchAlignment: hipDeviceAttribute_t = hipDeviceAttribute_t(
        82,
    );
}
impl hipDeviceAttribute_t {
    ///< Constant memory size in bytes.
    pub const hipDeviceAttributeTotalConstantMemory: hipDeviceAttribute_t = hipDeviceAttribute_t(
        83,
    );
}
impl hipDeviceAttribute_t {
    ///< Global memory available on devicice.
    pub const hipDeviceAttributeTotalGlobalMem: hipDeviceAttribute_t = hipDeviceAttribute_t(
        84,
    );
}
impl hipDeviceAttribute_t {
    ///< Cuda only. An unified address space shared with the host.
    pub const hipDeviceAttributeUnifiedAddressing: hipDeviceAttribute_t = hipDeviceAttribute_t(
        85,
    );
}
impl hipDeviceAttribute_t {
    ///< Previously hipDeviceAttributeUuid
    pub const hipDeviceAttributeUnused2: hipDeviceAttribute_t = hipDeviceAttribute_t(86);
}
impl hipDeviceAttribute_t {
    ///< Warp size in threads.
    pub const hipDeviceAttributeWarpSize: hipDeviceAttribute_t = hipDeviceAttribute_t(
        87,
    );
}
impl hipDeviceAttribute_t {
    ///< Device supports HIP Stream Ordered Memory Allocator
    pub const hipDeviceAttributeMemoryPoolsSupported: hipDeviceAttribute_t = hipDeviceAttribute_t(
        88,
    );
}
impl hipDeviceAttribute_t {
    ///< Device supports HIP virtual memory management
    pub const hipDeviceAttributeVirtualMemoryManagementSupported: hipDeviceAttribute_t = hipDeviceAttribute_t(
        89,
    );
}
impl hipDeviceAttribute_t {
    ///< Can device support host memory registration via hipHostRegister
    pub const hipDeviceAttributeHostRegisterSupported: hipDeviceAttribute_t = hipDeviceAttribute_t(
        90,
    );
}
impl hipDeviceAttribute_t {
    ///< Supported handle mask for HIP Stream Ordered Memory Allocator
    pub const hipDeviceAttributeMemoryPoolSupportedHandleTypes: hipDeviceAttribute_t = hipDeviceAttribute_t(
        91,
    );
}
impl hipDeviceAttribute_t {
    pub const hipDeviceAttributeCudaCompatibleEnd: hipDeviceAttribute_t = hipDeviceAttribute_t(
        9999,
    );
}
impl hipDeviceAttribute_t {
    pub const hipDeviceAttributeAmdSpecificBegin: hipDeviceAttribute_t = hipDeviceAttribute_t(
        10000,
    );
}
impl hipDeviceAttribute_t {
    ///< Frequency in khz of the timer used by the device-side "clock*"
    pub const hipDeviceAttributeClockInstructionRate: hipDeviceAttribute_t = hipDeviceAttribute_t(
        10000,
    );
}
impl hipDeviceAttribute_t {
    ///< Previously hipDeviceAttributeArch
    pub const hipDeviceAttributeUnused3: hipDeviceAttribute_t = hipDeviceAttribute_t(
        10001,
    );
}
impl hipDeviceAttribute_t {
    ///< Maximum Shared Memory PerMultiprocessor.
    pub const hipDeviceAttributeMaxSharedMemoryPerMultiprocessor: hipDeviceAttribute_t = hipDeviceAttribute_t(
        10002,
    );
}
impl hipDeviceAttribute_t {
    ///< Previously hipDeviceAttributeGcnArch
    pub const hipDeviceAttributeUnused4: hipDeviceAttribute_t = hipDeviceAttribute_t(
        10003,
    );
}
impl hipDeviceAttribute_t {
    ///< Previously hipDeviceAttributeGcnArchName
    pub const hipDeviceAttributeUnused5: hipDeviceAttribute_t = hipDeviceAttribute_t(
        10004,
    );
}
impl hipDeviceAttribute_t {
    ///< Address of the HDP_MEM_COHERENCY_FLUSH_CNTL register
    pub const hipDeviceAttributeHdpMemFlushCntl: hipDeviceAttribute_t = hipDeviceAttribute_t(
        10005,
    );
}
impl hipDeviceAttribute_t {
    ///< Address of the HDP_REG_COHERENCY_FLUSH_CNTL register
    pub const hipDeviceAttributeHdpRegFlushCntl: hipDeviceAttribute_t = hipDeviceAttribute_t(
        10006,
    );
}
impl hipDeviceAttribute_t {
    /**< Supports cooperative launch on multiple
< devices with unmatched functions*/
    pub const hipDeviceAttributeCooperativeMultiDeviceUnmatchedFunc: hipDeviceAttribute_t = hipDeviceAttribute_t(
        10007,
    );
}
impl hipDeviceAttribute_t {
    /**< Supports cooperative launch on multiple
< devices with unmatched grid dimensions*/
    pub const hipDeviceAttributeCooperativeMultiDeviceUnmatchedGridDim: hipDeviceAttribute_t = hipDeviceAttribute_t(
        10008,
    );
}
impl hipDeviceAttribute_t {
    /**< Supports cooperative launch on multiple
< devices with unmatched block dimensions*/
    pub const hipDeviceAttributeCooperativeMultiDeviceUnmatchedBlockDim: hipDeviceAttribute_t = hipDeviceAttribute_t(
        10009,
    );
}
impl hipDeviceAttribute_t {
    /**< Supports cooperative launch on multiple
< devices with unmatched shared memories*/
    pub const hipDeviceAttributeCooperativeMultiDeviceUnmatchedSharedMem: hipDeviceAttribute_t = hipDeviceAttribute_t(
        10010,
    );
}
impl hipDeviceAttribute_t {
    ///< Whether it is LargeBar
    pub const hipDeviceAttributeIsLargeBar: hipDeviceAttribute_t = hipDeviceAttribute_t(
        10011,
    );
}
impl hipDeviceAttribute_t {
    ///< Revision of the GPU in this device
    pub const hipDeviceAttributeAsicRevision: hipDeviceAttribute_t = hipDeviceAttribute_t(
        10012,
    );
}
impl hipDeviceAttribute_t {
    /**< '1' if Device supports hipStreamWaitValue32() and
< hipStreamWaitValue64(), '0' otherwise.*/
    pub const hipDeviceAttributeCanUseStreamWaitValue: hipDeviceAttribute_t = hipDeviceAttribute_t(
        10013,
    );
}
impl hipDeviceAttribute_t {
    ///< '1' if Device supports image, '0' otherwise.
    pub const hipDeviceAttributeImageSupport: hipDeviceAttribute_t = hipDeviceAttribute_t(
        10014,
    );
}
impl hipDeviceAttribute_t {
    /**< All available physical compute
< units for the device*/
    pub const hipDeviceAttributePhysicalMultiProcessorCount: hipDeviceAttribute_t = hipDeviceAttribute_t(
        10015,
    );
}
impl hipDeviceAttribute_t {
    ///< '1' if Device supports fine grain, '0' otherwise
    pub const hipDeviceAttributeFineGrainSupport: hipDeviceAttribute_t = hipDeviceAttribute_t(
        10016,
    );
}
impl hipDeviceAttribute_t {
    ///< Constant frequency of wall clock in kilohertz.
    pub const hipDeviceAttributeWallClockRate: hipDeviceAttribute_t = hipDeviceAttribute_t(
        10017,
    );
}
impl hipDeviceAttribute_t {
    pub const hipDeviceAttributeAmdSpecificEnd: hipDeviceAttribute_t = hipDeviceAttribute_t(
        19999,
    );
}
impl hipDeviceAttribute_t {
    pub const hipDeviceAttributeVendorSpecificBegin: hipDeviceAttribute_t = hipDeviceAttribute_t(
        20000,
    );
}
#[repr(transparent)]
/** hipDeviceAttribute_t
 hipDeviceAttributeUnused number: 5*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipDeviceAttribute_t(pub ::core::ffi::c_uint);
impl hipDriverProcAddressQueryResult {
    pub const HIP_GET_PROC_ADDRESS_SUCCESS: hipDriverProcAddressQueryResult = hipDriverProcAddressQueryResult(
        0,
    );
}
impl hipDriverProcAddressQueryResult {
    pub const HIP_GET_PROC_ADDRESS_SYMBOL_NOT_FOUND: hipDriverProcAddressQueryResult = hipDriverProcAddressQueryResult(
        1,
    );
}
impl hipDriverProcAddressQueryResult {
    pub const HIP_GET_PROC_ADDRESS_VERSION_NOT_SUFFICIENT: hipDriverProcAddressQueryResult = hipDriverProcAddressQueryResult(
        2,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipDriverProcAddressQueryResult(pub ::core::ffi::c_uint);
impl hipComputeMode {
    pub const hipComputeModeDefault: hipComputeMode = hipComputeMode(0);
}
impl hipComputeMode {
    pub const hipComputeModeExclusive: hipComputeMode = hipComputeMode(1);
}
impl hipComputeMode {
    pub const hipComputeModeProhibited: hipComputeMode = hipComputeMode(2);
}
impl hipComputeMode {
    pub const hipComputeModeExclusiveProcess: hipComputeMode = hipComputeMode(3);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipComputeMode(pub ::core::ffi::c_uint);
impl hipFlushGPUDirectRDMAWritesOptions {
    pub const hipFlushGPUDirectRDMAWritesOptionHost: hipFlushGPUDirectRDMAWritesOptions = hipFlushGPUDirectRDMAWritesOptions(
        1,
    );
}
impl hipFlushGPUDirectRDMAWritesOptions {
    pub const hipFlushGPUDirectRDMAWritesOptionMemOps: hipFlushGPUDirectRDMAWritesOptions = hipFlushGPUDirectRDMAWritesOptions(
        2,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipFlushGPUDirectRDMAWritesOptions(pub ::core::ffi::c_uint);
impl hipGPUDirectRDMAWritesOrdering {
    pub const hipGPUDirectRDMAWritesOrderingNone: hipGPUDirectRDMAWritesOrdering = hipGPUDirectRDMAWritesOrdering(
        0,
    );
}
impl hipGPUDirectRDMAWritesOrdering {
    pub const hipGPUDirectRDMAWritesOrderingOwner: hipGPUDirectRDMAWritesOrdering = hipGPUDirectRDMAWritesOrdering(
        100,
    );
}
impl hipGPUDirectRDMAWritesOrdering {
    pub const hipGPUDirectRDMAWritesOrderingAllDevices: hipGPUDirectRDMAWritesOrdering = hipGPUDirectRDMAWritesOrdering(
        200,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipGPUDirectRDMAWritesOrdering(pub ::core::ffi::c_uint);
/**  @defgroup DriverTypes Driver Types
  @{
  This section describes the driver data types.
*/
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipDeviceptr_t(pub *mut ::core::ffi::c_void);
impl hipChannelFormatKind {
    ///< Signed channel format
    pub const hipChannelFormatKindSigned: hipChannelFormatKind = hipChannelFormatKind(0);
}
impl hipChannelFormatKind {
    ///< Unsigned channel format
    pub const hipChannelFormatKindUnsigned: hipChannelFormatKind = hipChannelFormatKind(
        1,
    );
}
impl hipChannelFormatKind {
    ///< Float channel format
    pub const hipChannelFormatKindFloat: hipChannelFormatKind = hipChannelFormatKind(2);
}
impl hipChannelFormatKind {
    ///< No channel format
    pub const hipChannelFormatKindNone: hipChannelFormatKind = hipChannelFormatKind(3);
}
#[repr(transparent)]
/// HIP channel format kinds
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipChannelFormatKind(pub ::core::ffi::c_uint);
/// HIP channel format descriptor
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipChannelFormatDesc {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub z: ::core::ffi::c_int,
    pub w: ::core::ffi::c_int,
    ///< Channel format kind
    pub f: hipChannelFormatKind,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hipArray {
    _unused: [u8; 0],
}
pub type hipArray_t = *mut hipArray;
pub type hipArray_const_t = *const hipArray;
impl hipArray_Format {
    ///< Unsigned 8-bit array format
    pub const HIP_AD_FORMAT_UNSIGNED_INT8: hipArray_Format = hipArray_Format(1);
}
impl hipArray_Format {
    ///< Unsigned 16-bit array format
    pub const HIP_AD_FORMAT_UNSIGNED_INT16: hipArray_Format = hipArray_Format(2);
}
impl hipArray_Format {
    ///< Unsigned 32-bit array format
    pub const HIP_AD_FORMAT_UNSIGNED_INT32: hipArray_Format = hipArray_Format(3);
}
impl hipArray_Format {
    ///< Signed 8-bit array format
    pub const HIP_AD_FORMAT_SIGNED_INT8: hipArray_Format = hipArray_Format(8);
}
impl hipArray_Format {
    ///< Signed 16-bit array format
    pub const HIP_AD_FORMAT_SIGNED_INT16: hipArray_Format = hipArray_Format(9);
}
impl hipArray_Format {
    ///< Signed 32-bit array format
    pub const HIP_AD_FORMAT_SIGNED_INT32: hipArray_Format = hipArray_Format(10);
}
impl hipArray_Format {
    ///< Half array format
    pub const HIP_AD_FORMAT_HALF: hipArray_Format = hipArray_Format(16);
}
impl hipArray_Format {
    ///< Float array format
    pub const HIP_AD_FORMAT_FLOAT: hipArray_Format = hipArray_Format(32);
}
#[repr(transparent)]
/// HIP array format
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipArray_Format(pub ::core::ffi::c_uint);
/// HIP array descriptor
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct HIP_ARRAY_DESCRIPTOR {
    ///< Width of the array
    pub Width: usize,
    ///< Height of the array
    pub Height: usize,
    ///< Format of the array
    pub Format: hipArray_Format,
    ///< Number of channels of the array
    pub NumChannels: ::core::ffi::c_uint,
}
/// HIP 3D array descriptor
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct HIP_ARRAY3D_DESCRIPTOR {
    ///< Width of the array
    pub Width: usize,
    ///< Height of the array
    pub Height: usize,
    ///< Depth of the array
    pub Depth: usize,
    ///< Format of the array
    pub Format: hipArray_Format,
    ///< Number of channels of the array
    pub NumChannels: ::core::ffi::c_uint,
    ///< Flags of the array
    pub Flags: ::core::ffi::c_uint,
}
/// HIP 2D memory copy parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hip_Memcpy2D {
    ///< Source width in bytes
    pub srcXInBytes: usize,
    ///< Source height
    pub srcY: usize,
    ///< Source memory type
    pub srcMemoryType: hipMemoryType,
    ///< Source pointer
    pub srcHost: *const ::core::ffi::c_void,
    ///< Source device
    pub srcDevice: hipDeviceptr_t,
    ///< Source array
    pub srcArray: hipArray_t,
    ///< Source pitch
    pub srcPitch: usize,
    ///< Destination width in bytes
    pub dstXInBytes: usize,
    ///< Destination height
    pub dstY: usize,
    ///< Destination memory type
    pub dstMemoryType: hipMemoryType,
    ///< Destination pointer
    pub dstHost: *mut ::core::ffi::c_void,
    ///< Destination device
    pub dstDevice: hipDeviceptr_t,
    ///< Destination array
    pub dstArray: hipArray_t,
    ///< Destination pitch
    pub dstPitch: usize,
    ///< Width in bytes of the 2D memory copy
    pub WidthInBytes: usize,
    ///< Height of the 2D memory copy
    pub Height: usize,
}
/// HIP mipmapped array
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipMipmappedArray {
    ///< Data pointer of the mipmapped array
    pub data: *mut ::core::ffi::c_void,
    ///< Description of the mipmapped array
    pub desc: hipChannelFormatDesc,
    ///< Type of the mipmapped array
    pub type_: ::core::ffi::c_uint,
    ///< Width of the mipmapped array
    pub width: ::core::ffi::c_uint,
    ///< Height of the mipmapped array
    pub height: ::core::ffi::c_uint,
    ///< Depth of the mipmapped array
    pub depth: ::core::ffi::c_uint,
    ///< Minimum level of the mipmapped array
    pub min_mipmap_level: ::core::ffi::c_uint,
    ///< Maximum level of the mipmapped array
    pub max_mipmap_level: ::core::ffi::c_uint,
    ///< Flags of the mipmapped array
    pub flags: ::core::ffi::c_uint,
    ///< Format of the mipmapped array
    pub format: hipArray_Format,
    ///< Number of channels of the mipmapped array
    pub num_channels: ::core::ffi::c_uint,
}
/// HIP mipmapped array pointer
pub type hipMipmappedArray_t = *mut hipMipmappedArray;
pub type hipmipmappedArray = hipMipmappedArray_t;
pub type hipMipmappedArray_const_t = *const hipMipmappedArray;
impl hipResourceType {
    ///< Array resource
    pub const hipResourceTypeArray: hipResourceType = hipResourceType(0);
}
impl hipResourceType {
    ///< Mipmapped array resource
    pub const hipResourceTypeMipmappedArray: hipResourceType = hipResourceType(1);
}
impl hipResourceType {
    ///< Linear resource
    pub const hipResourceTypeLinear: hipResourceType = hipResourceType(2);
}
impl hipResourceType {
    ///< Pitch 2D resource
    pub const hipResourceTypePitch2D: hipResourceType = hipResourceType(3);
}
#[repr(transparent)]
/// HIP resource types
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipResourceType(pub ::core::ffi::c_uint);
impl HIPresourcetype_enum {
    ///< Array resource
    pub const HIP_RESOURCE_TYPE_ARRAY: HIPresourcetype_enum = HIPresourcetype_enum(0);
}
impl HIPresourcetype_enum {
    ///< Mipmapped array resource
    pub const HIP_RESOURCE_TYPE_MIPMAPPED_ARRAY: HIPresourcetype_enum = HIPresourcetype_enum(
        1,
    );
}
impl HIPresourcetype_enum {
    ///< Linear resource
    pub const HIP_RESOURCE_TYPE_LINEAR: HIPresourcetype_enum = HIPresourcetype_enum(2);
}
impl HIPresourcetype_enum {
    ///< Pitch 2D resource
    pub const HIP_RESOURCE_TYPE_PITCH2D: HIPresourcetype_enum = HIPresourcetype_enum(3);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct HIPresourcetype_enum(pub ::core::ffi::c_uint);
pub use self::HIPresourcetype_enum as HIPresourcetype;
pub use self::HIPresourcetype_enum as hipResourcetype;
impl HIPaddress_mode_enum {
    ///< Wrap address mode
    pub const HIP_TR_ADDRESS_MODE_WRAP: HIPaddress_mode_enum = HIPaddress_mode_enum(0);
}
impl HIPaddress_mode_enum {
    ///< Clamp address mode
    pub const HIP_TR_ADDRESS_MODE_CLAMP: HIPaddress_mode_enum = HIPaddress_mode_enum(1);
}
impl HIPaddress_mode_enum {
    ///< Mirror address mode
    pub const HIP_TR_ADDRESS_MODE_MIRROR: HIPaddress_mode_enum = HIPaddress_mode_enum(2);
}
impl HIPaddress_mode_enum {
    ///< Border address mode
    pub const HIP_TR_ADDRESS_MODE_BORDER: HIPaddress_mode_enum = HIPaddress_mode_enum(3);
}
#[repr(transparent)]
/// HIP texture address modes
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct HIPaddress_mode_enum(pub ::core::ffi::c_uint);
/// HIP texture address modes
pub use self::HIPaddress_mode_enum as HIPaddress_mode;
impl HIPfilter_mode_enum {
    ///< Filter mode point
    pub const HIP_TR_FILTER_MODE_POINT: HIPfilter_mode_enum = HIPfilter_mode_enum(0);
}
impl HIPfilter_mode_enum {
    ///< Filter mode linear
    pub const HIP_TR_FILTER_MODE_LINEAR: HIPfilter_mode_enum = HIPfilter_mode_enum(1);
}
#[repr(transparent)]
/// HIP filter modes
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct HIPfilter_mode_enum(pub ::core::ffi::c_uint);
/// HIP filter modes
pub use self::HIPfilter_mode_enum as HIPfilter_mode;
/// HIP texture descriptor
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HIP_TEXTURE_DESC_st {
    ///< Address modes
    pub addressMode: [HIPaddress_mode; 3usize],
    ///< Filter mode
    pub filterMode: HIPfilter_mode,
    ///< Flags
    pub flags: ::core::ffi::c_uint,
    ///< Maximum anisotropy ratio
    pub maxAnisotropy: ::core::ffi::c_uint,
    ///< Mipmap filter mode
    pub mipmapFilterMode: HIPfilter_mode,
    ///< Mipmap level bias
    pub mipmapLevelBias: f32,
    ///< Mipmap minimum level clamp
    pub minMipmapLevelClamp: f32,
    ///< Mipmap maximum level clamp
    pub maxMipmapLevelClamp: f32,
    ///< Border Color
    pub borderColor: [f32; 4usize],
    pub reserved: [::core::ffi::c_int; 12usize],
}
/// HIP texture descriptor
pub type HIP_TEXTURE_DESC = HIP_TEXTURE_DESC_st;
impl hipResourceViewFormat {
    ///< No resource view format (use underlying resource format)
    pub const hipResViewFormatNone: hipResourceViewFormat = hipResourceViewFormat(0);
}
impl hipResourceViewFormat {
    ///< 1 channel, unsigned 8-bit integers
    pub const hipResViewFormatUnsignedChar1: hipResourceViewFormat = hipResourceViewFormat(
        1,
    );
}
impl hipResourceViewFormat {
    ///< 2 channels, unsigned 8-bit integers
    pub const hipResViewFormatUnsignedChar2: hipResourceViewFormat = hipResourceViewFormat(
        2,
    );
}
impl hipResourceViewFormat {
    ///< 4 channels, unsigned 8-bit integers
    pub const hipResViewFormatUnsignedChar4: hipResourceViewFormat = hipResourceViewFormat(
        3,
    );
}
impl hipResourceViewFormat {
    ///< 1 channel, signed 8-bit integers
    pub const hipResViewFormatSignedChar1: hipResourceViewFormat = hipResourceViewFormat(
        4,
    );
}
impl hipResourceViewFormat {
    ///< 2 channels, signed 8-bit integers
    pub const hipResViewFormatSignedChar2: hipResourceViewFormat = hipResourceViewFormat(
        5,
    );
}
impl hipResourceViewFormat {
    ///< 4 channels, signed 8-bit integers
    pub const hipResViewFormatSignedChar4: hipResourceViewFormat = hipResourceViewFormat(
        6,
    );
}
impl hipResourceViewFormat {
    ///< 1 channel, unsigned 16-bit integers
    pub const hipResViewFormatUnsignedShort1: hipResourceViewFormat = hipResourceViewFormat(
        7,
    );
}
impl hipResourceViewFormat {
    ///< 2 channels, unsigned 16-bit integers
    pub const hipResViewFormatUnsignedShort2: hipResourceViewFormat = hipResourceViewFormat(
        8,
    );
}
impl hipResourceViewFormat {
    ///< 4 channels, unsigned 16-bit integers
    pub const hipResViewFormatUnsignedShort4: hipResourceViewFormat = hipResourceViewFormat(
        9,
    );
}
impl hipResourceViewFormat {
    ///< 1 channel, signed 16-bit integers
    pub const hipResViewFormatSignedShort1: hipResourceViewFormat = hipResourceViewFormat(
        10,
    );
}
impl hipResourceViewFormat {
    ///< 2 channels, signed 16-bit integers
    pub const hipResViewFormatSignedShort2: hipResourceViewFormat = hipResourceViewFormat(
        11,
    );
}
impl hipResourceViewFormat {
    ///< 4 channels, signed 16-bit integers
    pub const hipResViewFormatSignedShort4: hipResourceViewFormat = hipResourceViewFormat(
        12,
    );
}
impl hipResourceViewFormat {
    ///< 1 channel, unsigned 32-bit integers
    pub const hipResViewFormatUnsignedInt1: hipResourceViewFormat = hipResourceViewFormat(
        13,
    );
}
impl hipResourceViewFormat {
    ///< 2 channels, unsigned 32-bit integers
    pub const hipResViewFormatUnsignedInt2: hipResourceViewFormat = hipResourceViewFormat(
        14,
    );
}
impl hipResourceViewFormat {
    ///< 4 channels, unsigned 32-bit integers
    pub const hipResViewFormatUnsignedInt4: hipResourceViewFormat = hipResourceViewFormat(
        15,
    );
}
impl hipResourceViewFormat {
    ///< 1 channel, signed 32-bit integers
    pub const hipResViewFormatSignedInt1: hipResourceViewFormat = hipResourceViewFormat(
        16,
    );
}
impl hipResourceViewFormat {
    ///< 2 channels, signed 32-bit integers
    pub const hipResViewFormatSignedInt2: hipResourceViewFormat = hipResourceViewFormat(
        17,
    );
}
impl hipResourceViewFormat {
    ///< 4 channels, signed 32-bit integers
    pub const hipResViewFormatSignedInt4: hipResourceViewFormat = hipResourceViewFormat(
        18,
    );
}
impl hipResourceViewFormat {
    ///< 1 channel, 16-bit floating point
    pub const hipResViewFormatHalf1: hipResourceViewFormat = hipResourceViewFormat(19);
}
impl hipResourceViewFormat {
    ///< 2 channels, 16-bit floating point
    pub const hipResViewFormatHalf2: hipResourceViewFormat = hipResourceViewFormat(20);
}
impl hipResourceViewFormat {
    ///< 4 channels, 16-bit floating point
    pub const hipResViewFormatHalf4: hipResourceViewFormat = hipResourceViewFormat(21);
}
impl hipResourceViewFormat {
    ///< 1 channel, 32-bit floating point
    pub const hipResViewFormatFloat1: hipResourceViewFormat = hipResourceViewFormat(22);
}
impl hipResourceViewFormat {
    ///< 2 channels, 32-bit floating point
    pub const hipResViewFormatFloat2: hipResourceViewFormat = hipResourceViewFormat(23);
}
impl hipResourceViewFormat {
    ///< 4 channels, 32-bit floating point
    pub const hipResViewFormatFloat4: hipResourceViewFormat = hipResourceViewFormat(24);
}
impl hipResourceViewFormat {
    ///< Block-compressed 1
    pub const hipResViewFormatUnsignedBlockCompressed1: hipResourceViewFormat = hipResourceViewFormat(
        25,
    );
}
impl hipResourceViewFormat {
    ///< Block-compressed 2
    pub const hipResViewFormatUnsignedBlockCompressed2: hipResourceViewFormat = hipResourceViewFormat(
        26,
    );
}
impl hipResourceViewFormat {
    ///< Block-compressed 3
    pub const hipResViewFormatUnsignedBlockCompressed3: hipResourceViewFormat = hipResourceViewFormat(
        27,
    );
}
impl hipResourceViewFormat {
    ///< Block-compressed 4 unsigned
    pub const hipResViewFormatUnsignedBlockCompressed4: hipResourceViewFormat = hipResourceViewFormat(
        28,
    );
}
impl hipResourceViewFormat {
    ///< Block-compressed 4 signed
    pub const hipResViewFormatSignedBlockCompressed4: hipResourceViewFormat = hipResourceViewFormat(
        29,
    );
}
impl hipResourceViewFormat {
    ///< Block-compressed 5 unsigned
    pub const hipResViewFormatUnsignedBlockCompressed5: hipResourceViewFormat = hipResourceViewFormat(
        30,
    );
}
impl hipResourceViewFormat {
    ///< Block-compressed 5 signed
    pub const hipResViewFormatSignedBlockCompressed5: hipResourceViewFormat = hipResourceViewFormat(
        31,
    );
}
impl hipResourceViewFormat {
    ///< Block-compressed 6 unsigned half-float
    pub const hipResViewFormatUnsignedBlockCompressed6H: hipResourceViewFormat = hipResourceViewFormat(
        32,
    );
}
impl hipResourceViewFormat {
    ///< Block-compressed 6 signed half-float
    pub const hipResViewFormatSignedBlockCompressed6H: hipResourceViewFormat = hipResourceViewFormat(
        33,
    );
}
impl hipResourceViewFormat {
    ///< Block-compressed 7
    pub const hipResViewFormatUnsignedBlockCompressed7: hipResourceViewFormat = hipResourceViewFormat(
        34,
    );
}
#[repr(transparent)]
/// HIP texture resource view formats
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipResourceViewFormat(pub ::core::ffi::c_uint);
impl HIPresourceViewFormat_enum {
    ///< No resource view format (use underlying resource format)
    pub const HIP_RES_VIEW_FORMAT_NONE: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        0,
    );
}
impl HIPresourceViewFormat_enum {
    ///< 1 channel, unsigned 8-bit integers
    pub const HIP_RES_VIEW_FORMAT_UINT_1X8: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        1,
    );
}
impl HIPresourceViewFormat_enum {
    ///< 2 channels, unsigned 8-bit integers
    pub const HIP_RES_VIEW_FORMAT_UINT_2X8: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        2,
    );
}
impl HIPresourceViewFormat_enum {
    ///< 4 channels, unsigned 8-bit integers
    pub const HIP_RES_VIEW_FORMAT_UINT_4X8: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        3,
    );
}
impl HIPresourceViewFormat_enum {
    ///< 1 channel, signed 8-bit integers
    pub const HIP_RES_VIEW_FORMAT_SINT_1X8: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        4,
    );
}
impl HIPresourceViewFormat_enum {
    ///< 2 channels, signed 8-bit integers
    pub const HIP_RES_VIEW_FORMAT_SINT_2X8: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        5,
    );
}
impl HIPresourceViewFormat_enum {
    ///< 4 channels, signed 8-bit integers
    pub const HIP_RES_VIEW_FORMAT_SINT_4X8: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        6,
    );
}
impl HIPresourceViewFormat_enum {
    ///< 1 channel, unsigned 16-bit integers
    pub const HIP_RES_VIEW_FORMAT_UINT_1X16: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        7,
    );
}
impl HIPresourceViewFormat_enum {
    ///< 2 channels, unsigned 16-bit integers
    pub const HIP_RES_VIEW_FORMAT_UINT_2X16: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        8,
    );
}
impl HIPresourceViewFormat_enum {
    ///< 4 channels, unsigned 16-bit integers
    pub const HIP_RES_VIEW_FORMAT_UINT_4X16: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        9,
    );
}
impl HIPresourceViewFormat_enum {
    ///< 1 channel, signed 16-bit integers
    pub const HIP_RES_VIEW_FORMAT_SINT_1X16: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        10,
    );
}
impl HIPresourceViewFormat_enum {
    ///< 2 channels, signed 16-bit integers
    pub const HIP_RES_VIEW_FORMAT_SINT_2X16: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        11,
    );
}
impl HIPresourceViewFormat_enum {
    ///< 4 channels, signed 16-bit integers
    pub const HIP_RES_VIEW_FORMAT_SINT_4X16: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        12,
    );
}
impl HIPresourceViewFormat_enum {
    ///< 1 channel, unsigned 32-bit integers
    pub const HIP_RES_VIEW_FORMAT_UINT_1X32: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        13,
    );
}
impl HIPresourceViewFormat_enum {
    ///< 2 channels, unsigned 32-bit integers
    pub const HIP_RES_VIEW_FORMAT_UINT_2X32: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        14,
    );
}
impl HIPresourceViewFormat_enum {
    ///< 4 channels, unsigned 32-bit integers
    pub const HIP_RES_VIEW_FORMAT_UINT_4X32: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        15,
    );
}
impl HIPresourceViewFormat_enum {
    ///< 1 channel, signed 32-bit integers
    pub const HIP_RES_VIEW_FORMAT_SINT_1X32: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        16,
    );
}
impl HIPresourceViewFormat_enum {
    ///< 2 channels, signed 32-bit integers
    pub const HIP_RES_VIEW_FORMAT_SINT_2X32: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        17,
    );
}
impl HIPresourceViewFormat_enum {
    ///< 4 channels, signed 32-bit integers
    pub const HIP_RES_VIEW_FORMAT_SINT_4X32: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        18,
    );
}
impl HIPresourceViewFormat_enum {
    ///< 1 channel, 16-bit floating point
    pub const HIP_RES_VIEW_FORMAT_FLOAT_1X16: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        19,
    );
}
impl HIPresourceViewFormat_enum {
    ///< 2 channels, 16-bit floating point
    pub const HIP_RES_VIEW_FORMAT_FLOAT_2X16: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        20,
    );
}
impl HIPresourceViewFormat_enum {
    ///< 4 channels, 16-bit floating point
    pub const HIP_RES_VIEW_FORMAT_FLOAT_4X16: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        21,
    );
}
impl HIPresourceViewFormat_enum {
    ///< 1 channel, 32-bit floating point
    pub const HIP_RES_VIEW_FORMAT_FLOAT_1X32: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        22,
    );
}
impl HIPresourceViewFormat_enum {
    ///< 2 channels, 32-bit floating point
    pub const HIP_RES_VIEW_FORMAT_FLOAT_2X32: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        23,
    );
}
impl HIPresourceViewFormat_enum {
    ///< 4 channels, 32-bit floating point
    pub const HIP_RES_VIEW_FORMAT_FLOAT_4X32: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        24,
    );
}
impl HIPresourceViewFormat_enum {
    ///< Block-compressed 1
    pub const HIP_RES_VIEW_FORMAT_UNSIGNED_BC1: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        25,
    );
}
impl HIPresourceViewFormat_enum {
    ///< Block-compressed 2
    pub const HIP_RES_VIEW_FORMAT_UNSIGNED_BC2: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        26,
    );
}
impl HIPresourceViewFormat_enum {
    ///< Block-compressed 3
    pub const HIP_RES_VIEW_FORMAT_UNSIGNED_BC3: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        27,
    );
}
impl HIPresourceViewFormat_enum {
    ///< Block-compressed 4 unsigned
    pub const HIP_RES_VIEW_FORMAT_UNSIGNED_BC4: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        28,
    );
}
impl HIPresourceViewFormat_enum {
    ///< Block-compressed 4 signed
    pub const HIP_RES_VIEW_FORMAT_SIGNED_BC4: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        29,
    );
}
impl HIPresourceViewFormat_enum {
    ///< Block-compressed 5 unsigned
    pub const HIP_RES_VIEW_FORMAT_UNSIGNED_BC5: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        30,
    );
}
impl HIPresourceViewFormat_enum {
    ///< Block-compressed 5 signed
    pub const HIP_RES_VIEW_FORMAT_SIGNED_BC5: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        31,
    );
}
impl HIPresourceViewFormat_enum {
    ///< Block-compressed 6 unsigned half-float
    pub const HIP_RES_VIEW_FORMAT_UNSIGNED_BC6H: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        32,
    );
}
impl HIPresourceViewFormat_enum {
    ///< Block-compressed 6 signed half-float
    pub const HIP_RES_VIEW_FORMAT_SIGNED_BC6H: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        33,
    );
}
impl HIPresourceViewFormat_enum {
    ///< Block-compressed 7
    pub const HIP_RES_VIEW_FORMAT_UNSIGNED_BC7: HIPresourceViewFormat_enum = HIPresourceViewFormat_enum(
        34,
    );
}
#[repr(transparent)]
/// HIP texture resource view formats
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct HIPresourceViewFormat_enum(pub ::core::ffi::c_uint);
/// HIP texture resource view formats
pub use self::HIPresourceViewFormat_enum as HIPresourceViewFormat;
/// HIP resource descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hipResourceDesc {
    ///< Resource type
    pub resType: hipResourceType,
    pub res: hipResourceDesc__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hipResourceDesc__bindgen_ty_1 {
    pub array: hipResourceDesc__bindgen_ty_1__bindgen_ty_1,
    pub mipmap: hipResourceDesc__bindgen_ty_1__bindgen_ty_2,
    pub linear: hipResourceDesc__bindgen_ty_1__bindgen_ty_3,
    pub pitch2D: hipResourceDesc__bindgen_ty_1__bindgen_ty_4,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipResourceDesc__bindgen_ty_1__bindgen_ty_1 {
    ///< HIP array
    pub array: hipArray_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipResourceDesc__bindgen_ty_1__bindgen_ty_2 {
    ///< HIP mipmapped array
    pub mipmap: hipMipmappedArray_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipResourceDesc__bindgen_ty_1__bindgen_ty_3 {
    ///< Device pointer
    pub devPtr: *mut ::core::ffi::c_void,
    ///< Channel format description
    pub desc: hipChannelFormatDesc,
    ///< Size in bytes
    pub sizeInBytes: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipResourceDesc__bindgen_ty_1__bindgen_ty_4 {
    ///< Device pointer
    pub devPtr: *mut ::core::ffi::c_void,
    ///< Channel format description
    pub desc: hipChannelFormatDesc,
    ///< Width of the array in elements
    pub width: usize,
    ///< Height of the array in elements
    pub height: usize,
    ///< Pitch between two rows in bytes
    pub pitchInBytes: usize,
}
/// HIP resource view descriptor struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HIP_RESOURCE_DESC_st {
    ///< Resource type
    pub resType: HIPresourcetype,
    pub res: HIP_RESOURCE_DESC_st__bindgen_ty_1,
    ///< Flags (must be zero)
    pub flags: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union HIP_RESOURCE_DESC_st__bindgen_ty_1 {
    pub array: HIP_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_1,
    pub mipmap: HIP_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_2,
    pub linear: HIP_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_3,
    pub pitch2D: HIP_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_4,
    pub reserved: HIP_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_5,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct HIP_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_1 {
    ///< HIP array
    pub hArray: hipArray_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct HIP_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_2 {
    ///< HIP mipmapped array
    pub hMipmappedArray: hipMipmappedArray_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct HIP_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_3 {
    ///< Device pointer
    pub devPtr: hipDeviceptr_t,
    ///< Array format
    pub format: hipArray_Format,
    ///< Channels per array element
    pub numChannels: ::core::ffi::c_uint,
    ///< Size in bytes
    pub sizeInBytes: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct HIP_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_4 {
    ///< Device pointer
    pub devPtr: hipDeviceptr_t,
    ///< Array format
    pub format: hipArray_Format,
    ///< Channels per array element
    pub numChannels: ::core::ffi::c_uint,
    ///< Width of the array in elements
    pub width: usize,
    ///< Height of the array in elements
    pub height: usize,
    ///< Pitch between two rows in bytes
    pub pitchInBytes: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct HIP_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_5 {
    pub reserved: [::core::ffi::c_int; 32usize],
}
/// HIP resource view descriptor struct
pub type HIP_RESOURCE_DESC = HIP_RESOURCE_DESC_st;
/// HIP resource view descriptor
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipResourceViewDesc {
    ///< Resource view format
    pub format: hipResourceViewFormat,
    ///< Width of the resource view
    pub width: usize,
    ///< Height of the resource view
    pub height: usize,
    ///< Depth of the resource view
    pub depth: usize,
    ///< First defined mipmap level
    pub firstMipmapLevel: ::core::ffi::c_uint,
    ///< Last defined mipmap level
    pub lastMipmapLevel: ::core::ffi::c_uint,
    ///< First layer index
    pub firstLayer: ::core::ffi::c_uint,
    ///< Last layer index
    pub lastLayer: ::core::ffi::c_uint,
}
/// Resource view descriptor
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct HIP_RESOURCE_VIEW_DESC_st {
    ///< Resource view format
    pub format: HIPresourceViewFormat,
    ///< Width of the resource view
    pub width: usize,
    ///< Height of the resource view
    pub height: usize,
    ///< Depth of the resource view
    pub depth: usize,
    ///< First defined mipmap level
    pub firstMipmapLevel: ::core::ffi::c_uint,
    ///< Last defined mipmap level
    pub lastMipmapLevel: ::core::ffi::c_uint,
    ///< First layer index
    pub firstLayer: ::core::ffi::c_uint,
    ///< Last layer index
    pub lastLayer: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 16usize],
}
/// Resource view descriptor
pub type HIP_RESOURCE_VIEW_DESC = HIP_RESOURCE_VIEW_DESC_st;
impl hipMemcpyKind {
    ///< Host-to-Host Copy
    pub const hipMemcpyHostToHost: hipMemcpyKind = hipMemcpyKind(0);
}
impl hipMemcpyKind {
    ///< Host-to-Device Copy
    pub const hipMemcpyHostToDevice: hipMemcpyKind = hipMemcpyKind(1);
}
impl hipMemcpyKind {
    ///< Device-to-Host Copy
    pub const hipMemcpyDeviceToHost: hipMemcpyKind = hipMemcpyKind(2);
}
impl hipMemcpyKind {
    ///< Device-to-Device Copy
    pub const hipMemcpyDeviceToDevice: hipMemcpyKind = hipMemcpyKind(3);
}
impl hipMemcpyKind {
    /**< Runtime will automatically determine
<copy-kind based on virtual addresses.*/
    pub const hipMemcpyDefault: hipMemcpyKind = hipMemcpyKind(4);
}
impl hipMemcpyKind {
    ///< Device-to-Device Copy without using compute units
    pub const hipMemcpyDeviceToDeviceNoCU: hipMemcpyKind = hipMemcpyKind(1024);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipMemcpyKind(pub ::core::ffi::c_uint);
/// HIP pithed pointer
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipPitchedPtr {
    ///< Pointer to the allocated memory
    pub ptr: *mut ::core::ffi::c_void,
    ///< Pitch in bytes
    pub pitch: usize,
    ///< Logical size of the first dimension of allocation in elements
    pub xsize: usize,
    ///< Logical size of the second dimension of allocation in elements
    pub ysize: usize,
}
/// HIP extent
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipExtent {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
}
///  HIP position
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipPos {
    ///< X coordinate
    pub x: usize,
    ///< Y coordinate
    pub y: usize,
    ///< Z coordinate
    pub z: usize,
}
/// HIP 3D memory copy parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipMemcpy3DParms {
    ///< Source array
    pub srcArray: hipArray_t,
    ///< Source position
    pub srcPos: hipPos,
    ///< Source pointer
    pub srcPtr: hipPitchedPtr,
    ///< Destination array
    pub dstArray: hipArray_t,
    ///< Destination position
    pub dstPos: hipPos,
    ///< Destination pointer
    pub dstPtr: hipPitchedPtr,
    ///< Extent of 3D memory copy
    pub extent: hipExtent,
    ///< Kind of 3D memory copy
    pub kind: hipMemcpyKind,
}
/// HIP 3D memory copy
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct HIP_MEMCPY3D {
    ///< Source X in bytes
    pub srcXInBytes: usize,
    ///< Source Y
    pub srcY: usize,
    ///< Source Z
    pub srcZ: usize,
    ///< Source LOD
    pub srcLOD: usize,
    ///< Source memory type
    pub srcMemoryType: hipMemoryType,
    ///< Source host pointer
    pub srcHost: *const ::core::ffi::c_void,
    ///< Source device
    pub srcDevice: hipDeviceptr_t,
    ///< Source array
    pub srcArray: hipArray_t,
    ///< Source pitch
    pub srcPitch: usize,
    ///< Source height
    pub srcHeight: usize,
    ///< Destination X in bytes
    pub dstXInBytes: usize,
    ///< Destination Y
    pub dstY: usize,
    ///< Destination Z
    pub dstZ: usize,
    ///< Destination LOD
    pub dstLOD: usize,
    ///< Destination memory type
    pub dstMemoryType: hipMemoryType,
    ///< Destination host pointer
    pub dstHost: *mut ::core::ffi::c_void,
    ///< Destination device
    pub dstDevice: hipDeviceptr_t,
    ///< Destination array
    pub dstArray: hipArray_t,
    ///< Destination pitch
    pub dstPitch: usize,
    ///< Destination height
    pub dstHeight: usize,
    ///< Width in bytes of 3D memory copy
    pub WidthInBytes: usize,
    ///< Height in bytes of 3D memory copy
    pub Height: usize,
    ///< Depth in bytes of 3D memory copy
    pub Depth: usize,
}
impl hipFunction_attribute {
    ///< The maximum number of threads per block. Depends on function and device.
    pub const HIP_FUNC_ATTRIBUTE_MAX_THREADS_PER_BLOCK: hipFunction_attribute = hipFunction_attribute(
        0,
    );
}
impl hipFunction_attribute {
    ///< The statically allocated shared memory size in bytes per block required by the function.
    pub const HIP_FUNC_ATTRIBUTE_SHARED_SIZE_BYTES: hipFunction_attribute = hipFunction_attribute(
        1,
    );
}
impl hipFunction_attribute {
    ///< The user-allocated constant memory by the function in bytes.
    pub const HIP_FUNC_ATTRIBUTE_CONST_SIZE_BYTES: hipFunction_attribute = hipFunction_attribute(
        2,
    );
}
impl hipFunction_attribute {
    ///< The local memory usage of each thread by this function in bytes.
    pub const HIP_FUNC_ATTRIBUTE_LOCAL_SIZE_BYTES: hipFunction_attribute = hipFunction_attribute(
        3,
    );
}
impl hipFunction_attribute {
    ///< The number of registers used by each thread of this function.
    pub const HIP_FUNC_ATTRIBUTE_NUM_REGS: hipFunction_attribute = hipFunction_attribute(
        4,
    );
}
impl hipFunction_attribute {
    ///< PTX version
    pub const HIP_FUNC_ATTRIBUTE_PTX_VERSION: hipFunction_attribute = hipFunction_attribute(
        5,
    );
}
impl hipFunction_attribute {
    ///< Binary version
    pub const HIP_FUNC_ATTRIBUTE_BINARY_VERSION: hipFunction_attribute = hipFunction_attribute(
        6,
    );
}
impl hipFunction_attribute {
    ///< Cache mode
    pub const HIP_FUNC_ATTRIBUTE_CACHE_MODE_CA: hipFunction_attribute = hipFunction_attribute(
        7,
    );
}
impl hipFunction_attribute {
    ///< The maximum dynamic shared memory per block for this function in bytes.
    pub const HIP_FUNC_ATTRIBUTE_MAX_DYNAMIC_SHARED_SIZE_BYTES: hipFunction_attribute = hipFunction_attribute(
        8,
    );
}
impl hipFunction_attribute {
    ///< The shared memory carveout preference in percent of the maximum shared memory.
    pub const HIP_FUNC_ATTRIBUTE_PREFERRED_SHARED_MEMORY_CARVEOUT: hipFunction_attribute = hipFunction_attribute(
        9,
    );
}
impl hipFunction_attribute {
    pub const HIP_FUNC_ATTRIBUTE_MAX: hipFunction_attribute = hipFunction_attribute(10);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipFunction_attribute(pub ::core::ffi::c_uint);
impl hipPointer_attribute {
    /**< The context on which a pointer was allocated
< @warning This attribute is not supported in HIP*/
    pub const HIP_POINTER_ATTRIBUTE_CONTEXT: hipPointer_attribute = hipPointer_attribute(
        1,
    );
}
impl hipPointer_attribute {
    ///< memory type describing the location of a pointer
    pub const HIP_POINTER_ATTRIBUTE_MEMORY_TYPE: hipPointer_attribute = hipPointer_attribute(
        2,
    );
}
impl hipPointer_attribute {
    ///< address at which the pointer is allocated on the device
    pub const HIP_POINTER_ATTRIBUTE_DEVICE_POINTER: hipPointer_attribute = hipPointer_attribute(
        3,
    );
}
impl hipPointer_attribute {
    ///< address at which the pointer is allocated on the host
    pub const HIP_POINTER_ATTRIBUTE_HOST_POINTER: hipPointer_attribute = hipPointer_attribute(
        4,
    );
}
impl hipPointer_attribute {
    /**< A pair of tokens for use with Linux kernel interface
< @warning This attribute is not supported in HIP*/
    pub const HIP_POINTER_ATTRIBUTE_P2P_TOKENS: hipPointer_attribute = hipPointer_attribute(
        5,
    );
}
impl hipPointer_attribute {
    /**< Synchronize every synchronous memory operation
< initiated on this region*/
    pub const HIP_POINTER_ATTRIBUTE_SYNC_MEMOPS: hipPointer_attribute = hipPointer_attribute(
        6,
    );
}
impl hipPointer_attribute {
    ///< Unique ID for an allocated memory region
    pub const HIP_POINTER_ATTRIBUTE_BUFFER_ID: hipPointer_attribute = hipPointer_attribute(
        7,
    );
}
impl hipPointer_attribute {
    ///< Indicates if the pointer points to managed memory
    pub const HIP_POINTER_ATTRIBUTE_IS_MANAGED: hipPointer_attribute = hipPointer_attribute(
        8,
    );
}
impl hipPointer_attribute {
    /**< device ordinal of a device on which a pointer
< was allocated or registered*/
    pub const HIP_POINTER_ATTRIBUTE_DEVICE_ORDINAL: hipPointer_attribute = hipPointer_attribute(
        9,
    );
}
impl hipPointer_attribute {
    /**< if this pointer maps to an allocation
< that is suitable for hipIpcGetMemHandle
< @warning This attribute is not supported in HIP*/
    pub const HIP_POINTER_ATTRIBUTE_IS_LEGACY_HIP_IPC_CAPABLE: hipPointer_attribute = hipPointer_attribute(
        10,
    );
}
impl hipPointer_attribute {
    ///< Starting address for this requested pointer
    pub const HIP_POINTER_ATTRIBUTE_RANGE_START_ADDR: hipPointer_attribute = hipPointer_attribute(
        11,
    );
}
impl hipPointer_attribute {
    ///< Size of the address range for this requested pointer
    pub const HIP_POINTER_ATTRIBUTE_RANGE_SIZE: hipPointer_attribute = hipPointer_attribute(
        12,
    );
}
impl hipPointer_attribute {
    /**< tells if this pointer is in a valid address range
< that is mapped to a backing allocation*/
    pub const HIP_POINTER_ATTRIBUTE_MAPPED: hipPointer_attribute = hipPointer_attribute(
        13,
    );
}
impl hipPointer_attribute {
    /**< Bitmask of allowed hipmemAllocationHandleType
< for this allocation @warning This attribute is not supported in HIP*/
    pub const HIP_POINTER_ATTRIBUTE_ALLOWED_HANDLE_TYPES: hipPointer_attribute = hipPointer_attribute(
        14,
    );
}
impl hipPointer_attribute {
    /**< returns if the memory referenced by
< this pointer can be used with the GPUDirect RDMA API
< @warning This attribute is not supported in HIP*/
    pub const HIP_POINTER_ATTRIBUTE_IS_GPU_DIRECT_RDMA_CAPABLE: hipPointer_attribute = hipPointer_attribute(
        15,
    );
}
impl hipPointer_attribute {
    /**< Returns the access flags the device associated with
< for the corresponding memory referenced by the ptr*/
    pub const HIP_POINTER_ATTRIBUTE_ACCESS_FLAGS: hipPointer_attribute = hipPointer_attribute(
        16,
    );
}
impl hipPointer_attribute {
    /**< Returns the mempool handle for the allocation if
< it was allocated from a mempool
< @warning This attribute is not supported in HIP*/
    pub const HIP_POINTER_ATTRIBUTE_MEMPOOL_HANDLE: hipPointer_attribute = hipPointer_attribute(
        17,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipPointer_attribute(pub ::core::ffi::c_uint);
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    pub fn hipCreateChannelDesc(
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        z: ::core::ffi::c_int,
        w: ::core::ffi::c_int,
        f: hipChannelFormatKind,
    ) -> hipChannelFormatDesc;
}
/// An opaque value that represents a hip texture object
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __hip_texture {
    _unused: [u8; 0],
}
pub type hipTextureObject_t = *mut __hip_texture;
impl hipTextureAddressMode {
    pub const hipAddressModeWrap: hipTextureAddressMode = hipTextureAddressMode(0);
}
impl hipTextureAddressMode {
    pub const hipAddressModeClamp: hipTextureAddressMode = hipTextureAddressMode(1);
}
impl hipTextureAddressMode {
    pub const hipAddressModeMirror: hipTextureAddressMode = hipTextureAddressMode(2);
}
impl hipTextureAddressMode {
    pub const hipAddressModeBorder: hipTextureAddressMode = hipTextureAddressMode(3);
}
#[repr(transparent)]
/// hip texture address modes
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipTextureAddressMode(pub ::core::ffi::c_uint);
impl hipTextureFilterMode {
    pub const hipFilterModePoint: hipTextureFilterMode = hipTextureFilterMode(0);
}
impl hipTextureFilterMode {
    pub const hipFilterModeLinear: hipTextureFilterMode = hipTextureFilterMode(1);
}
#[repr(transparent)]
/// hip texture filter modes
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipTextureFilterMode(pub ::core::ffi::c_uint);
impl hipTextureReadMode {
    pub const hipReadModeElementType: hipTextureReadMode = hipTextureReadMode(0);
}
impl hipTextureReadMode {
    pub const hipReadModeNormalizedFloat: hipTextureReadMode = hipTextureReadMode(1);
}
#[repr(transparent)]
/// hip texture read modes
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipTextureReadMode(pub ::core::ffi::c_uint);
/// hip texture reference
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct textureReference {
    pub normalized: ::core::ffi::c_int,
    pub readMode: hipTextureReadMode,
    pub filterMode: hipTextureFilterMode,
    pub addressMode: [hipTextureAddressMode; 3usize],
    pub channelDesc: hipChannelFormatDesc,
    pub sRGB: ::core::ffi::c_int,
    pub maxAnisotropy: ::core::ffi::c_uint,
    pub mipmapFilterMode: hipTextureFilterMode,
    pub mipmapLevelBias: f32,
    pub minMipmapLevelClamp: f32,
    pub maxMipmapLevelClamp: f32,
    pub textureObject: hipTextureObject_t,
    pub numChannels: ::core::ffi::c_int,
    pub format: hipArray_Format,
}
/// hip texture descriptor
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct hipTextureDesc {
    pub addressMode: [hipTextureAddressMode; 3usize],
    pub filterMode: hipTextureFilterMode,
    pub readMode: hipTextureReadMode,
    pub sRGB: ::core::ffi::c_int,
    pub borderColor: [f32; 4usize],
    pub normalizedCoords: ::core::ffi::c_int,
    pub maxAnisotropy: ::core::ffi::c_uint,
    pub mipmapFilterMode: hipTextureFilterMode,
    pub mipmapLevelBias: f32,
    pub minMipmapLevelClamp: f32,
    pub maxMipmapLevelClamp: f32,
}
/// An opaque value that represents a hip surface object
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __hip_surface {
    _unused: [u8; 0],
}
pub type hipSurfaceObject_t = *mut __hip_surface;
impl hipSurfaceBoundaryMode {
    pub const hipBoundaryModeZero: hipSurfaceBoundaryMode = hipSurfaceBoundaryMode(0);
}
impl hipSurfaceBoundaryMode {
    pub const hipBoundaryModeTrap: hipSurfaceBoundaryMode = hipSurfaceBoundaryMode(1);
}
impl hipSurfaceBoundaryMode {
    pub const hipBoundaryModeClamp: hipSurfaceBoundaryMode = hipSurfaceBoundaryMode(2);
}
#[repr(transparent)]
/// hip surface boundary modes
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipSurfaceBoundaryMode(pub ::core::ffi::c_uint);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ihipCtx_t {
    _unused: [u8; 0],
}
pub type hipCtx_t = *mut ihipCtx_t;
pub type hipDevice_t = ::core::ffi::c_int;
impl hipDeviceP2PAttr {
    pub const hipDevP2PAttrPerformanceRank: hipDeviceP2PAttr = hipDeviceP2PAttr(0);
}
impl hipDeviceP2PAttr {
    pub const hipDevP2PAttrAccessSupported: hipDeviceP2PAttr = hipDeviceP2PAttr(1);
}
impl hipDeviceP2PAttr {
    pub const hipDevP2PAttrNativeAtomicSupported: hipDeviceP2PAttr = hipDeviceP2PAttr(2);
}
impl hipDeviceP2PAttr {
    pub const hipDevP2PAttrHipArrayAccessSupported: hipDeviceP2PAttr = hipDeviceP2PAttr(
        3,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipDeviceP2PAttr(pub ::core::ffi::c_uint);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ihipStream_t {
    _unused: [u8; 0],
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipStream_t(pub *mut ihipStream_t);
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipIpcMemHandle_st {
    pub reserved: [::core::ffi::c_char; 64usize],
}
pub type hipIpcMemHandle_t = hipIpcMemHandle_st;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipIpcEventHandle_st {
    pub reserved: [::core::ffi::c_char; 64usize],
}
pub type hipIpcEventHandle_t = hipIpcEventHandle_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ihipModule_t {
    _unused: [u8; 0],
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipModule_t(pub *mut ihipModule_t);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ihipModuleSymbol_t {
    _unused: [u8; 0],
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipFunction_t(pub *mut ihipModuleSymbol_t);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ihipLinkState_t {
    _unused: [u8; 0],
}
pub type hipLinkState_t = *mut ihipLinkState_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ihipMemPoolHandle_t {
    _unused: [u8; 0],
}
/// HIP memory pool
pub type hipMemPool_t = *mut ihipMemPoolHandle_t;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipFuncAttributes {
    pub binaryVersion: ::core::ffi::c_int,
    pub cacheModeCA: ::core::ffi::c_int,
    pub constSizeBytes: usize,
    pub localSizeBytes: usize,
    pub maxDynamicSharedSizeBytes: ::core::ffi::c_int,
    pub maxThreadsPerBlock: ::core::ffi::c_int,
    pub numRegs: ::core::ffi::c_int,
    pub preferredShmemCarveout: ::core::ffi::c_int,
    pub ptxVersion: ::core::ffi::c_int,
    pub sharedSizeBytes: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ihipEvent_t {
    _unused: [u8; 0],
}
pub type hipEvent_t = *mut ihipEvent_t;
impl hipLimit_t {
    /**< Limit of stack size in bytes on the current device, per
< thread. The size is in units of 256 dwords, up to the
< limit of (128K - 16)*/
    pub const hipLimitStackSize: hipLimit_t = hipLimit_t(0);
}
impl hipLimit_t {
    /**< Size limit in bytes of fifo used by printf call on the
< device. Currently not supported*/
    pub const hipLimitPrintfFifoSize: hipLimit_t = hipLimit_t(1);
}
impl hipLimit_t {
    /**< Limit of heap size in bytes on the current device, should
< be less than the global memory size on the device*/
    pub const hipLimitMallocHeapSize: hipLimit_t = hipLimit_t(2);
}
impl hipLimit_t {
    ///< Supported limit range
    pub const hipLimitRange: hipLimit_t = hipLimit_t(3);
}
#[repr(transparent)]
/** hipLimit

 @note In HIP device limit-related APIs, any input limit value other than those defined in the
 enum is treated as "UnsupportedLimit" by default.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipLimit_t(pub ::core::ffi::c_uint);
impl hipStreamBatchMemOpType {
    pub const hipStreamMemOpWaitValue32: hipStreamBatchMemOpType = hipStreamBatchMemOpType(
        1,
    );
}
impl hipStreamBatchMemOpType {
    pub const hipStreamMemOpWriteValue32: hipStreamBatchMemOpType = hipStreamBatchMemOpType(
        2,
    );
}
impl hipStreamBatchMemOpType {
    pub const hipStreamMemOpWaitValue64: hipStreamBatchMemOpType = hipStreamBatchMemOpType(
        4,
    );
}
impl hipStreamBatchMemOpType {
    pub const hipStreamMemOpWriteValue64: hipStreamBatchMemOpType = hipStreamBatchMemOpType(
        5,
    );
}
impl hipStreamBatchMemOpType {
    ///< Currently not supported
    pub const hipStreamMemOpBarrier: hipStreamBatchMemOpType = hipStreamBatchMemOpType(
        6,
    );
}
impl hipStreamBatchMemOpType {
    ///< Currently not supported
    pub const hipStreamMemOpFlushRemoteWrites: hipStreamBatchMemOpType = hipStreamBatchMemOpType(
        3,
    );
}
#[repr(transparent)]
/// Operations for hipStreamBatchMemOp
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipStreamBatchMemOpType(pub ::core::ffi::c_uint);
/** @brief Union representing batch memory operation parameters for HIP streams.

 hipStreamBatchMemOpParams is used to specify the parameters for batch memory
 operations in a HIP stream. This union supports various operations including
 waiting for a specific value, writing a value, and different flags for wait conditions.

 @details
 The union includes fields for different types of operations defined in the
 enum hipStreamBatchMemOpType:
 - hipStreamMemOpWaitValue32:  Wait for a 32-bit value.
 - hipStreamMemOpWriteValue32: Write a 32-bit value.
 - hipStreamMemOpWaitValue64:  Wait for a 64-bit value.
 - hipStreamMemOpWriteValue64: Write a 64-bit value.

 Each operation type includes an address, the value to wait for or write, flags, and an
 optional alias that is not relevant on AMD GPUs. Flags can be used to specify different
 wait conditions such as equality, bitwise AND, greater than or equal, and bitwise NOR.

 Example usage:
 @code
 hipStreamBatchMemOpParams myArray[2];
 myArray[0].operation = hipStreamMemOpWaitValue32;
 myArray[0].waitValue.address = waitAddr1;
 myArray[0].waitValue.value = 0x1;
 myArray[0].waitValue.flags = CU_STREAM_WAIT_VALUE_EQ;

 myArray[1].operation = hipStreamMemOpWriteValue32;
 myArray[1].writeValue.address = writeAddr1;
 myArray[1].writeValue.value = 0x1;
 myArray[1].writeValue.flags = 0x0;

 result = hipStreamBatchMemOp(stream, 2, myArray, 0);
 @endcode*/
#[repr(C)]
#[derive(Copy, Clone)]
pub union hipStreamBatchMemOpParams_union {
    pub operation: hipStreamBatchMemOpType,
    pub waitValue: hipStreamBatchMemOpParams_union_hipStreamMemOpWaitValueParams_t,
    pub writeValue: hipStreamBatchMemOpParams_union_hipStreamMemOpWriteValueParams_t,
    ///< Currently not supported on AMD
    pub flushRemoteWrites: hipStreamBatchMemOpParams_union_hipStreamMemOpFlushRemoteWritesParams_t,
    ///< Currently not supported on AMD
    pub memoryBarrier: hipStreamBatchMemOpParams_union_hipStreamMemOpMemoryBarrierParams_t,
    pub pad: [u64; 6usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hipStreamBatchMemOpParams_union_hipStreamMemOpWaitValueParams_t {
    pub operation: hipStreamBatchMemOpType,
    pub address: hipDeviceptr_t,
    pub __bindgen_anon_1: hipStreamBatchMemOpParams_union_hipStreamMemOpWaitValueParams_t__bindgen_ty_1,
    pub flags: ::core::ffi::c_uint,
    ///< Not valid for AMD backend. Initial value is unimportant
    pub alias: hipDeviceptr_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hipStreamBatchMemOpParams_union_hipStreamMemOpWaitValueParams_t__bindgen_ty_1 {
    pub value: u32,
    pub value64: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hipStreamBatchMemOpParams_union_hipStreamMemOpWriteValueParams_t {
    pub operation: hipStreamBatchMemOpType,
    pub address: hipDeviceptr_t,
    pub __bindgen_anon_1: hipStreamBatchMemOpParams_union_hipStreamMemOpWriteValueParams_t__bindgen_ty_1,
    pub flags: ::core::ffi::c_uint,
    ///< Not valid for AMD backend. Initial value is unimportant
    pub alias: hipDeviceptr_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hipStreamBatchMemOpParams_union_hipStreamMemOpWriteValueParams_t__bindgen_ty_1 {
    pub value: u32,
    pub value64: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipStreamBatchMemOpParams_union_hipStreamMemOpFlushRemoteWritesParams_t {
    pub operation: hipStreamBatchMemOpType,
    pub flags: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipStreamBatchMemOpParams_union_hipStreamMemOpMemoryBarrierParams_t {
    pub operation: hipStreamBatchMemOpType,
    pub flags: ::core::ffi::c_uint,
}
/** @brief Union representing batch memory operation parameters for HIP streams.

 hipStreamBatchMemOpParams is used to specify the parameters for batch memory
 operations in a HIP stream. This union supports various operations including
 waiting for a specific value, writing a value, and different flags for wait conditions.

 @details
 The union includes fields for different types of operations defined in the
 enum hipStreamBatchMemOpType:
 - hipStreamMemOpWaitValue32:  Wait for a 32-bit value.
 - hipStreamMemOpWriteValue32: Write a 32-bit value.
 - hipStreamMemOpWaitValue64:  Wait for a 64-bit value.
 - hipStreamMemOpWriteValue64: Write a 64-bit value.

 Each operation type includes an address, the value to wait for or write, flags, and an
 optional alias that is not relevant on AMD GPUs. Flags can be used to specify different
 wait conditions such as equality, bitwise AND, greater than or equal, and bitwise NOR.

 Example usage:
 @code
 hipStreamBatchMemOpParams myArray[2];
 myArray[0].operation = hipStreamMemOpWaitValue32;
 myArray[0].waitValue.address = waitAddr1;
 myArray[0].waitValue.value = 0x1;
 myArray[0].waitValue.flags = CU_STREAM_WAIT_VALUE_EQ;

 myArray[1].operation = hipStreamMemOpWriteValue32;
 myArray[1].writeValue.address = writeAddr1;
 myArray[1].writeValue.value = 0x1;
 myArray[1].writeValue.flags = 0x0;

 result = hipStreamBatchMemOp(stream, 2, myArray, 0);
 @endcode*/
pub type hipStreamBatchMemOpParams = hipStreamBatchMemOpParams_union;
/** @brief Structure representing node parameters for batch memory operations in HIP graphs.

 hipBatchMemOpNodeParams is used to specify the parameters for batch memory
 operations in HIP graphs. This struct includes the context to use for the operations, the
 number of operations, and an array of hipStreamBatchMemOpParams that describe the operations.

 @details
 The structure includes the following fields:
 - ctx: The HIP context to use for the operations.
 - count: The number of operations in the paramArray.
 - paramArray: A pointer to an array of hipStreamBatchMemOpParams.
 - flags: Flags to control the node.

 Example usage:
 @code
 hipBatchMemOpNodeParams nodeParams;
 nodeParams.ctx = context;
 nodeParams.count = ARRAY_SIZE;
 nodeParams.paramArray = myArray;
 nodeParams.flags = 0;

 Pass nodeParams to a HIP graph APIs hipGraphAddBatchMemOpNode, hipGraphBatchMemOpNodeGetParams,
 hipGraphBatchMemOpNodeSetParams, hipGraphExecBatchMemOpNodeSetParams
 @endcode*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipBatchMemOpNodeParams {
    pub ctx: hipCtx_t,
    pub count: ::core::ffi::c_uint,
    pub paramArray: *mut hipStreamBatchMemOpParams,
    pub flags: ::core::ffi::c_uint,
}
impl hipMemoryAdvise {
    /**< Data will mostly be read and only occassionally
< be written to*/
    pub const hipMemAdviseSetReadMostly: hipMemoryAdvise = hipMemoryAdvise(1);
}
impl hipMemoryAdvise {
    ///< Undo the effect of hipMemAdviseSetReadMostly
    pub const hipMemAdviseUnsetReadMostly: hipMemoryAdvise = hipMemoryAdvise(2);
}
impl hipMemoryAdvise {
    /**< Set the preferred location for the data as
< the specified device*/
    pub const hipMemAdviseSetPreferredLocation: hipMemoryAdvise = hipMemoryAdvise(3);
}
impl hipMemoryAdvise {
    ///< Clear the preferred location for the data
    pub const hipMemAdviseUnsetPreferredLocation: hipMemoryAdvise = hipMemoryAdvise(4);
}
impl hipMemoryAdvise {
    /**< Data will be accessed by the specified device
< so prevent page faults as much as possible*/
    pub const hipMemAdviseSetAccessedBy: hipMemoryAdvise = hipMemoryAdvise(5);
}
impl hipMemoryAdvise {
    /**< Let HIP to decide on the page faulting policy
< for the specified device*/
    pub const hipMemAdviseUnsetAccessedBy: hipMemoryAdvise = hipMemoryAdvise(6);
}
impl hipMemoryAdvise {
    /**< The default memory model is fine-grain. That allows
< coherent operations between host and device, while
< executing kernels. The coarse-grain can be used
< for data that only needs to be coherent at dispatch
< boundaries for better performance*/
    pub const hipMemAdviseSetCoarseGrain: hipMemoryAdvise = hipMemoryAdvise(100);
}
impl hipMemoryAdvise {
    ///< Restores cache coherency policy back to fine-grain
    pub const hipMemAdviseUnsetCoarseGrain: hipMemoryAdvise = hipMemoryAdvise(101);
}
#[repr(transparent)]
/** HIP Memory Advise values

 @note This memory advise enumeration is used on Linux, not Windows.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipMemoryAdvise(pub ::core::ffi::c_uint);
impl hipMemRangeCoherencyMode {
    /**< Updates to memory with this attribute can be
< done coherently from all devices*/
    pub const hipMemRangeCoherencyModeFineGrain: hipMemRangeCoherencyMode = hipMemRangeCoherencyMode(
        0,
    );
}
impl hipMemRangeCoherencyMode {
    /**< Writes to memory with this attribute can be
< performed by a single device at a time*/
    pub const hipMemRangeCoherencyModeCoarseGrain: hipMemRangeCoherencyMode = hipMemRangeCoherencyMode(
        1,
    );
}
impl hipMemRangeCoherencyMode {
    /**< Memory region queried contains subregions with
< both hipMemRangeCoherencyModeFineGrain and
< hipMemRangeCoherencyModeCoarseGrain attributes*/
    pub const hipMemRangeCoherencyModeIndeterminate: hipMemRangeCoherencyMode = hipMemRangeCoherencyMode(
        2,
    );
}
#[repr(transparent)]
/// HIP Coherency Mode
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipMemRangeCoherencyMode(pub ::core::ffi::c_uint);
impl hipMemRangeAttribute {
    /**< Whether the range will mostly be read and
< only occassionally be written to*/
    pub const hipMemRangeAttributeReadMostly: hipMemRangeAttribute = hipMemRangeAttribute(
        1,
    );
}
impl hipMemRangeAttribute {
    ///< The preferred location of the range
    pub const hipMemRangeAttributePreferredLocation: hipMemRangeAttribute = hipMemRangeAttribute(
        2,
    );
}
impl hipMemRangeAttribute {
    /**< Memory range has hipMemAdviseSetAccessedBy
< set for the specified device*/
    pub const hipMemRangeAttributeAccessedBy: hipMemRangeAttribute = hipMemRangeAttribute(
        3,
    );
}
impl hipMemRangeAttribute {
    /**< The last location to where the range was
< prefetched*/
    pub const hipMemRangeAttributeLastPrefetchLocation: hipMemRangeAttribute = hipMemRangeAttribute(
        4,
    );
}
impl hipMemRangeAttribute {
    /**< Returns coherency mode
< @ref hipMemRangeCoherencyMode for the range*/
    pub const hipMemRangeAttributeCoherencyMode: hipMemRangeAttribute = hipMemRangeAttribute(
        100,
    );
}
#[repr(transparent)]
/// HIP range attributes
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipMemRangeAttribute(pub ::core::ffi::c_uint);
impl hipMemPoolAttr {
    /** (value type = int)
 Allow @p hipMemAllocAsync to use memory asynchronously freed
 in another streams as long as a stream ordering dependency
 of the allocating stream on the free action exists.
 hip events and null stream interactions can create the required
 stream ordered dependencies. (default enabled)*/
    pub const hipMemPoolReuseFollowEventDependencies: hipMemPoolAttr = hipMemPoolAttr(1);
}
impl hipMemPoolAttr {
    /** (value type = int)
 Allow reuse of already completed frees when there is no dependency
 between the free and allocation. (default enabled)*/
    pub const hipMemPoolReuseAllowOpportunistic: hipMemPoolAttr = hipMemPoolAttr(2);
}
impl hipMemPoolAttr {
    /** (value type = int)
 Allow @p hipMemAllocAsync to insert new stream dependencies
 in order to establish the stream ordering required to reuse
 a piece of memory released by cuFreeAsync (default enabled).*/
    pub const hipMemPoolReuseAllowInternalDependencies: hipMemPoolAttr = hipMemPoolAttr(
        3,
    );
}
impl hipMemPoolAttr {
    /** (value type = uint64_t)
 Amount of reserved memory in bytes to hold onto before trying
 to release memory back to the OS. When more than the release
 threshold bytes of memory are held by the memory pool, the
 allocator will try to release memory back to the OS on the
 next call to stream, event or context synchronize. (default 0)*/
    pub const hipMemPoolAttrReleaseThreshold: hipMemPoolAttr = hipMemPoolAttr(4);
}
impl hipMemPoolAttr {
    /** (value type = uint64_t)
 Amount of backing memory currently allocated for the mempool.*/
    pub const hipMemPoolAttrReservedMemCurrent: hipMemPoolAttr = hipMemPoolAttr(5);
}
impl hipMemPoolAttr {
    /** (value type = uint64_t)
 High watermark of backing memory allocated for the mempool since the
 last time it was reset. High watermark can only be reset to zero.*/
    pub const hipMemPoolAttrReservedMemHigh: hipMemPoolAttr = hipMemPoolAttr(6);
}
impl hipMemPoolAttr {
    /** (value type = uint64_t)
 Amount of memory from the pool that is currently in use by the application.*/
    pub const hipMemPoolAttrUsedMemCurrent: hipMemPoolAttr = hipMemPoolAttr(7);
}
impl hipMemPoolAttr {
    /** (value type = uint64_t)
 High watermark of the amount of memory from the pool that was in use by the application since
 the last time it was reset. High watermark can only be reset to zero.*/
    pub const hipMemPoolAttrUsedMemHigh: hipMemPoolAttr = hipMemPoolAttr(8);
}
#[repr(transparent)]
/// HIP memory pool attributes
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipMemPoolAttr(pub ::core::ffi::c_uint);
impl hipMemLocationType {
    pub const hipMemLocationTypeInvalid: hipMemLocationType = hipMemLocationType(0);
}
impl hipMemLocationType {
    ///< Device location, thus it's HIP device ID
    pub const hipMemLocationTypeDevice: hipMemLocationType = hipMemLocationType(1);
}
#[repr(transparent)]
/// Specifies the type of location
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipMemLocationType(pub ::core::ffi::c_uint);
/** Specifies a memory location.

 To specify a gpu, set type = @p hipMemLocationTypeDevice and set id = the gpu's device ID*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipMemLocation {
    ///< Specifies the location type, which describes the meaning of id
    pub type_: hipMemLocationType,
    ///< Identifier for the provided location type @p hipMemLocationType
    pub id: ::core::ffi::c_int,
}
impl hipMemAccessFlags {
    ///< Default, make the address range not accessible
    pub const hipMemAccessFlagsProtNone: hipMemAccessFlags = hipMemAccessFlags(0);
}
impl hipMemAccessFlags {
    ///< Set the address range read accessible
    pub const hipMemAccessFlagsProtRead: hipMemAccessFlags = hipMemAccessFlags(1);
}
impl hipMemAccessFlags {
    ///< Set the address range read-write accessible
    pub const hipMemAccessFlagsProtReadWrite: hipMemAccessFlags = hipMemAccessFlags(3);
}
#[repr(transparent)]
/** Specifies the memory protection flags for mapping
*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipMemAccessFlags(pub ::core::ffi::c_uint);
/// Memory access descriptor
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipMemAccessDesc {
    ///< Location on which the accessibility has to change
    pub location: hipMemLocation,
    ///< Accessibility flags to set
    pub flags: hipMemAccessFlags,
}
impl hipMemAllocationType {
    pub const hipMemAllocationTypeInvalid: hipMemAllocationType = hipMemAllocationType(
        0,
    );
}
impl hipMemAllocationType {
    /** This allocation type is 'pinned', i.e. cannot migrate from its current
 location while the application is actively using it*/
    pub const hipMemAllocationTypePinned: hipMemAllocationType = hipMemAllocationType(1);
}
impl hipMemAllocationType {
    /** This allocation type is 'pinned', i.e. cannot migrate from its current
 location while the application is actively using it*/
    pub const hipMemAllocationTypeMax: hipMemAllocationType = hipMemAllocationType(
        2147483647,
    );
}
#[repr(transparent)]
/// Defines the allocation types
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipMemAllocationType(pub ::core::ffi::c_uint);
impl hipMemAllocationHandleType {
    ///< Does not allow any export mechanism
    pub const hipMemHandleTypeNone: hipMemAllocationHandleType = hipMemAllocationHandleType(
        0,
    );
}
impl hipMemAllocationHandleType {
    ///< Allows a file descriptor for exporting. Permitted only on POSIX systems
    pub const hipMemHandleTypePosixFileDescriptor: hipMemAllocationHandleType = hipMemAllocationHandleType(
        1,
    );
}
impl hipMemAllocationHandleType {
    ///< Allows a Win32 NT handle for exporting. (HANDLE)
    pub const hipMemHandleTypeWin32: hipMemAllocationHandleType = hipMemAllocationHandleType(
        2,
    );
}
impl hipMemAllocationHandleType {
    ///< Allows a Win32 KMT handle for exporting. (D3DKMT_HANDLE)
    pub const hipMemHandleTypeWin32Kmt: hipMemAllocationHandleType = hipMemAllocationHandleType(
        4,
    );
}
#[repr(transparent)]
/** Flags for specifying handle types for memory pool allocations
*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipMemAllocationHandleType(pub ::core::ffi::c_uint);
/// Specifies the properties of allocations made from the pool.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipMemPoolProps {
    ///< Allocation type. Currently must be specified as @p hipMemAllocationTypePinned
    pub allocType: hipMemAllocationType,
    ///< Handle types that will be supported by allocations from the pool
    pub handleTypes: hipMemAllocationHandleType,
    ///< Location where allocations should reside
    pub location: hipMemLocation,
    /// Windows-specific LPSECURITYATTRIBUTES required when @p hipMemHandleTypeWin32 is specified
    pub win32SecurityAttributes: *mut ::core::ffi::c_void,
    ///< Maximum pool size. When set to 0, defaults to a system dependent value
    pub maxSize: usize,
    ///< Reserved for future use, must be 0
    pub reserved: [::core::ffi::c_uchar; 56usize],
}
/// Opaque data structure for exporting a pool allocation
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipMemPoolPtrExportData {
    pub reserved: [::core::ffi::c_uchar; 64usize],
}
impl hipFuncAttribute {
    pub const hipFuncAttributeMaxDynamicSharedMemorySize: hipFuncAttribute = hipFuncAttribute(
        8,
    );
}
impl hipFuncAttribute {
    pub const hipFuncAttributePreferredSharedMemoryCarveout: hipFuncAttribute = hipFuncAttribute(
        9,
    );
}
impl hipFuncAttribute {
    pub const hipFuncAttributeMax: hipFuncAttribute = hipFuncAttribute(10);
}
#[repr(transparent)]
/// @warning On AMD devices and some Nvidia devices, these hints and controls are ignored.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipFuncAttribute(pub ::core::ffi::c_uint);
impl hipFuncCache_t {
    ///< no preference for shared memory or L1 (default)
    pub const hipFuncCachePreferNone: hipFuncCache_t = hipFuncCache_t(0);
}
impl hipFuncCache_t {
    ///< prefer larger shared memory and smaller L1 cache
    pub const hipFuncCachePreferShared: hipFuncCache_t = hipFuncCache_t(1);
}
impl hipFuncCache_t {
    ///< prefer larger L1 cache and smaller shared memory
    pub const hipFuncCachePreferL1: hipFuncCache_t = hipFuncCache_t(2);
}
impl hipFuncCache_t {
    ///< prefer equal size L1 cache and shared memory
    pub const hipFuncCachePreferEqual: hipFuncCache_t = hipFuncCache_t(3);
}
#[repr(transparent)]
/// @warning On AMD devices and some Nvidia devices, these hints and controls are ignored.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipFuncCache_t(pub ::core::ffi::c_uint);
impl hipSharedMemConfig {
    ///< The compiler selects a device-specific value for the banking.
    pub const hipSharedMemBankSizeDefault: hipSharedMemConfig = hipSharedMemConfig(0);
}
impl hipSharedMemConfig {
    /**< Shared mem is banked at 4-bytes intervals and performs best
< when adjacent threads access data 4 bytes apart.*/
    pub const hipSharedMemBankSizeFourByte: hipSharedMemConfig = hipSharedMemConfig(1);
}
impl hipSharedMemConfig {
    /**< Shared mem is banked at 8-byte intervals and performs best
< when adjacent threads access data 4 bytes apart.*/
    pub const hipSharedMemBankSizeEightByte: hipSharedMemConfig = hipSharedMemConfig(2);
}
#[repr(transparent)]
/// @warning On AMD devices and some Nvidia devices, these hints and controls are ignored.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipSharedMemConfig(pub ::core::ffi::c_uint);
/// Struct for data in 3D
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct dim3 {
    ///< x
    pub x: u32,
    ///< y
    pub y: u32,
    ///< z
    pub z: u32,
}
/// struct hipLaunchParams_t
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipLaunchParams_t {
    ///< Device function symbol
    pub func: *mut ::core::ffi::c_void,
    ///< Grid dimensions
    pub gridDim: dim3,
    ///< Block dimensions
    pub blockDim: dim3,
    ///< Arguments
    pub args: *mut *mut ::core::ffi::c_void,
    ///< Shared memory
    pub sharedMem: usize,
    ///< Stream identifier
    pub stream: hipStream_t,
}
/// struct hipLaunchParams_t
pub type hipLaunchParams = hipLaunchParams_t;
/// struct hipFunctionLaunchParams_t
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipFunctionLaunchParams_t {
    ///< Kernel to launch
    pub function: hipFunction_t,
    ///< Width(X) of grid in blocks
    pub gridDimX: ::core::ffi::c_uint,
    ///< Height(Y) of grid in blocks
    pub gridDimY: ::core::ffi::c_uint,
    ///< Depth(Z) of grid in blocks
    pub gridDimZ: ::core::ffi::c_uint,
    ///< X dimension of each thread block
    pub blockDimX: ::core::ffi::c_uint,
    ///< Y dimension of each thread block
    pub blockDimY: ::core::ffi::c_uint,
    ///< Z dimension of each thread block
    pub blockDimZ: ::core::ffi::c_uint,
    ///< Shared memory
    pub sharedMemBytes: ::core::ffi::c_uint,
    ///< Stream identifier
    pub hStream: hipStream_t,
    ///< Kernel parameters
    pub kernelParams: *mut *mut ::core::ffi::c_void,
}
/// struct hipFunctionLaunchParams_t
pub type hipFunctionLaunchParams = hipFunctionLaunchParams_t;
impl hipExternalMemoryHandleType_enum {
    pub const hipExternalMemoryHandleTypeOpaqueFd: hipExternalMemoryHandleType_enum = hipExternalMemoryHandleType_enum(
        1,
    );
}
impl hipExternalMemoryHandleType_enum {
    pub const hipExternalMemoryHandleTypeOpaqueWin32: hipExternalMemoryHandleType_enum = hipExternalMemoryHandleType_enum(
        2,
    );
}
impl hipExternalMemoryHandleType_enum {
    pub const hipExternalMemoryHandleTypeOpaqueWin32Kmt: hipExternalMemoryHandleType_enum = hipExternalMemoryHandleType_enum(
        3,
    );
}
impl hipExternalMemoryHandleType_enum {
    pub const hipExternalMemoryHandleTypeD3D12Heap: hipExternalMemoryHandleType_enum = hipExternalMemoryHandleType_enum(
        4,
    );
}
impl hipExternalMemoryHandleType_enum {
    pub const hipExternalMemoryHandleTypeD3D12Resource: hipExternalMemoryHandleType_enum = hipExternalMemoryHandleType_enum(
        5,
    );
}
impl hipExternalMemoryHandleType_enum {
    pub const hipExternalMemoryHandleTypeD3D11Resource: hipExternalMemoryHandleType_enum = hipExternalMemoryHandleType_enum(
        6,
    );
}
impl hipExternalMemoryHandleType_enum {
    pub const hipExternalMemoryHandleTypeD3D11ResourceKmt: hipExternalMemoryHandleType_enum = hipExternalMemoryHandleType_enum(
        7,
    );
}
impl hipExternalMemoryHandleType_enum {
    pub const hipExternalMemoryHandleTypeNvSciBuf: hipExternalMemoryHandleType_enum = hipExternalMemoryHandleType_enum(
        8,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipExternalMemoryHandleType_enum(pub ::core::ffi::c_uint);
pub use self::hipExternalMemoryHandleType_enum as hipExternalMemoryHandleType;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hipExternalMemoryHandleDesc_st {
    pub type_: hipExternalMemoryHandleType,
    pub handle: hipExternalMemoryHandleDesc_st__bindgen_ty_1,
    pub size: ::core::ffi::c_ulonglong,
    pub flags: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 16usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hipExternalMemoryHandleDesc_st__bindgen_ty_1 {
    pub fd: ::core::ffi::c_int,
    pub win32: hipExternalMemoryHandleDesc_st__bindgen_ty_1__bindgen_ty_1,
    pub nvSciBufObject: *const ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipExternalMemoryHandleDesc_st__bindgen_ty_1__bindgen_ty_1 {
    pub handle: *mut ::core::ffi::c_void,
    pub name: *const ::core::ffi::c_void,
}
pub type hipExternalMemoryHandleDesc = hipExternalMemoryHandleDesc_st;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipExternalMemoryBufferDesc_st {
    pub offset: ::core::ffi::c_ulonglong,
    pub size: ::core::ffi::c_ulonglong,
    pub flags: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 16usize],
}
pub type hipExternalMemoryBufferDesc = hipExternalMemoryBufferDesc_st;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipExternalMemoryMipmappedArrayDesc_st {
    pub offset: ::core::ffi::c_ulonglong,
    pub formatDesc: hipChannelFormatDesc,
    pub extent: hipExtent,
    pub flags: ::core::ffi::c_uint,
    pub numLevels: ::core::ffi::c_uint,
}
pub type hipExternalMemoryMipmappedArrayDesc = hipExternalMemoryMipmappedArrayDesc_st;
pub type hipExternalMemory_t = *mut ::core::ffi::c_void;
impl hipExternalSemaphoreHandleType_enum {
    pub const hipExternalSemaphoreHandleTypeOpaqueFd: hipExternalSemaphoreHandleType_enum = hipExternalSemaphoreHandleType_enum(
        1,
    );
}
impl hipExternalSemaphoreHandleType_enum {
    pub const hipExternalSemaphoreHandleTypeOpaqueWin32: hipExternalSemaphoreHandleType_enum = hipExternalSemaphoreHandleType_enum(
        2,
    );
}
impl hipExternalSemaphoreHandleType_enum {
    pub const hipExternalSemaphoreHandleTypeOpaqueWin32Kmt: hipExternalSemaphoreHandleType_enum = hipExternalSemaphoreHandleType_enum(
        3,
    );
}
impl hipExternalSemaphoreHandleType_enum {
    pub const hipExternalSemaphoreHandleTypeD3D12Fence: hipExternalSemaphoreHandleType_enum = hipExternalSemaphoreHandleType_enum(
        4,
    );
}
impl hipExternalSemaphoreHandleType_enum {
    pub const hipExternalSemaphoreHandleTypeD3D11Fence: hipExternalSemaphoreHandleType_enum = hipExternalSemaphoreHandleType_enum(
        5,
    );
}
impl hipExternalSemaphoreHandleType_enum {
    pub const hipExternalSemaphoreHandleTypeNvSciSync: hipExternalSemaphoreHandleType_enum = hipExternalSemaphoreHandleType_enum(
        6,
    );
}
impl hipExternalSemaphoreHandleType_enum {
    pub const hipExternalSemaphoreHandleTypeKeyedMutex: hipExternalSemaphoreHandleType_enum = hipExternalSemaphoreHandleType_enum(
        7,
    );
}
impl hipExternalSemaphoreHandleType_enum {
    pub const hipExternalSemaphoreHandleTypeKeyedMutexKmt: hipExternalSemaphoreHandleType_enum = hipExternalSemaphoreHandleType_enum(
        8,
    );
}
impl hipExternalSemaphoreHandleType_enum {
    pub const hipExternalSemaphoreHandleTypeTimelineSemaphoreFd: hipExternalSemaphoreHandleType_enum = hipExternalSemaphoreHandleType_enum(
        9,
    );
}
impl hipExternalSemaphoreHandleType_enum {
    pub const hipExternalSemaphoreHandleTypeTimelineSemaphoreWin32: hipExternalSemaphoreHandleType_enum = hipExternalSemaphoreHandleType_enum(
        10,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipExternalSemaphoreHandleType_enum(pub ::core::ffi::c_uint);
pub use self::hipExternalSemaphoreHandleType_enum as hipExternalSemaphoreHandleType;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hipExternalSemaphoreHandleDesc_st {
    pub type_: hipExternalSemaphoreHandleType,
    pub handle: hipExternalSemaphoreHandleDesc_st__bindgen_ty_1,
    pub flags: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 16usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hipExternalSemaphoreHandleDesc_st__bindgen_ty_1 {
    pub fd: ::core::ffi::c_int,
    pub win32: hipExternalSemaphoreHandleDesc_st__bindgen_ty_1__bindgen_ty_1,
    pub NvSciSyncObj: *const ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipExternalSemaphoreHandleDesc_st__bindgen_ty_1__bindgen_ty_1 {
    pub handle: *mut ::core::ffi::c_void,
    pub name: *const ::core::ffi::c_void,
}
pub type hipExternalSemaphoreHandleDesc = hipExternalSemaphoreHandleDesc_st;
pub type hipExternalSemaphore_t = *mut ::core::ffi::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hipExternalSemaphoreSignalParams_st {
    pub params: hipExternalSemaphoreSignalParams_st__bindgen_ty_1,
    pub flags: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 16usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hipExternalSemaphoreSignalParams_st__bindgen_ty_1 {
    pub fence: hipExternalSemaphoreSignalParams_st__bindgen_ty_1__bindgen_ty_1,
    pub nvSciSync: hipExternalSemaphoreSignalParams_st__bindgen_ty_1__bindgen_ty_2,
    pub keyedMutex: hipExternalSemaphoreSignalParams_st__bindgen_ty_1__bindgen_ty_3,
    pub reserved: [::core::ffi::c_uint; 12usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipExternalSemaphoreSignalParams_st__bindgen_ty_1__bindgen_ty_1 {
    pub value: ::core::ffi::c_ulonglong,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hipExternalSemaphoreSignalParams_st__bindgen_ty_1__bindgen_ty_2 {
    pub fence: *mut ::core::ffi::c_void,
    pub reserved: ::core::ffi::c_ulonglong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipExternalSemaphoreSignalParams_st__bindgen_ty_1__bindgen_ty_3 {
    pub key: ::core::ffi::c_ulonglong,
}
pub type hipExternalSemaphoreSignalParams = hipExternalSemaphoreSignalParams_st;
/// External semaphore wait parameters, compatible with driver type
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hipExternalSemaphoreWaitParams_st {
    pub params: hipExternalSemaphoreWaitParams_st__bindgen_ty_1,
    pub flags: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 16usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hipExternalSemaphoreWaitParams_st__bindgen_ty_1 {
    pub fence: hipExternalSemaphoreWaitParams_st__bindgen_ty_1__bindgen_ty_1,
    pub nvSciSync: hipExternalSemaphoreWaitParams_st__bindgen_ty_1__bindgen_ty_2,
    pub keyedMutex: hipExternalSemaphoreWaitParams_st__bindgen_ty_1__bindgen_ty_3,
    pub reserved: [::core::ffi::c_uint; 10usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipExternalSemaphoreWaitParams_st__bindgen_ty_1__bindgen_ty_1 {
    pub value: ::core::ffi::c_ulonglong,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hipExternalSemaphoreWaitParams_st__bindgen_ty_1__bindgen_ty_2 {
    pub fence: *mut ::core::ffi::c_void,
    pub reserved: ::core::ffi::c_ulonglong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipExternalSemaphoreWaitParams_st__bindgen_ty_1__bindgen_ty_3 {
    pub key: ::core::ffi::c_ulonglong,
    pub timeoutMs: ::core::ffi::c_uint,
}
/// External semaphore wait parameters, compatible with driver type
pub type hipExternalSemaphoreWaitParams = hipExternalSemaphoreWaitParams_st;
impl hipGraphicsRegisterFlags {
    pub const hipGraphicsRegisterFlagsNone: hipGraphicsRegisterFlags = hipGraphicsRegisterFlags(
        0,
    );
}
impl hipGraphicsRegisterFlags {
    ///< HIP will not write to this registered resource
    pub const hipGraphicsRegisterFlagsReadOnly: hipGraphicsRegisterFlags = hipGraphicsRegisterFlags(
        1,
    );
}
impl hipGraphicsRegisterFlags {
    pub const hipGraphicsRegisterFlagsWriteDiscard: hipGraphicsRegisterFlags = hipGraphicsRegisterFlags(
        2,
    );
}
impl hipGraphicsRegisterFlags {
    ///< HIP will bind this resource to a surface
    pub const hipGraphicsRegisterFlagsSurfaceLoadStore: hipGraphicsRegisterFlags = hipGraphicsRegisterFlags(
        4,
    );
}
impl hipGraphicsRegisterFlags {
    pub const hipGraphicsRegisterFlagsTextureGather: hipGraphicsRegisterFlags = hipGraphicsRegisterFlags(
        8,
    );
}
#[repr(transparent)]
/// HIP Access falgs for Interop resources.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipGraphicsRegisterFlags(pub ::core::ffi::c_uint);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _hipGraphicsResource {
    _unused: [u8; 0],
}
pub type hipGraphicsResource = _hipGraphicsResource;
pub type hipGraphicsResource_t = *mut hipGraphicsResource;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ihipGraph {
    _unused: [u8; 0],
}
/// An opaque value that represents a hip graph
pub type hipGraph_t = *mut ihipGraph;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hipGraphNode {
    _unused: [u8; 0],
}
/// An opaque value that represents a hip graph node
pub type hipGraphNode_t = *mut hipGraphNode;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hipGraphExec {
    _unused: [u8; 0],
}
/// An opaque value that represents a hip graph Exec
pub type hipGraphExec_t = *mut hipGraphExec;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hipUserObject {
    _unused: [u8; 0],
}
/// An opaque value that represents a user obj
pub type hipUserObject_t = *mut hipUserObject;
impl hipGraphNodeType {
    ///< GPU kernel node
    pub const hipGraphNodeTypeKernel: hipGraphNodeType = hipGraphNodeType(0);
}
impl hipGraphNodeType {
    ///< Memcpy node
    pub const hipGraphNodeTypeMemcpy: hipGraphNodeType = hipGraphNodeType(1);
}
impl hipGraphNodeType {
    ///< Memset node
    pub const hipGraphNodeTypeMemset: hipGraphNodeType = hipGraphNodeType(2);
}
impl hipGraphNodeType {
    ///< Host (executable) node
    pub const hipGraphNodeTypeHost: hipGraphNodeType = hipGraphNodeType(3);
}
impl hipGraphNodeType {
    ///< Node which executes an embedded graph
    pub const hipGraphNodeTypeGraph: hipGraphNodeType = hipGraphNodeType(4);
}
impl hipGraphNodeType {
    ///< Empty (no-op) node
    pub const hipGraphNodeTypeEmpty: hipGraphNodeType = hipGraphNodeType(5);
}
impl hipGraphNodeType {
    ///< External event wait node
    pub const hipGraphNodeTypeWaitEvent: hipGraphNodeType = hipGraphNodeType(6);
}
impl hipGraphNodeType {
    ///< External event record node
    pub const hipGraphNodeTypeEventRecord: hipGraphNodeType = hipGraphNodeType(7);
}
impl hipGraphNodeType {
    ///< External Semaphore signal node
    pub const hipGraphNodeTypeExtSemaphoreSignal: hipGraphNodeType = hipGraphNodeType(8);
}
impl hipGraphNodeType {
    ///< External Semaphore wait node
    pub const hipGraphNodeTypeExtSemaphoreWait: hipGraphNodeType = hipGraphNodeType(9);
}
impl hipGraphNodeType {
    ///< Memory alloc node
    pub const hipGraphNodeTypeMemAlloc: hipGraphNodeType = hipGraphNodeType(10);
}
impl hipGraphNodeType {
    ///< Memory free node
    pub const hipGraphNodeTypeMemFree: hipGraphNodeType = hipGraphNodeType(11);
}
impl hipGraphNodeType {
    ///< MemcpyFromSymbol node
    pub const hipGraphNodeTypeMemcpyFromSymbol: hipGraphNodeType = hipGraphNodeType(12);
}
impl hipGraphNodeType {
    ///< MemcpyToSymbol node
    pub const hipGraphNodeTypeMemcpyToSymbol: hipGraphNodeType = hipGraphNodeType(13);
}
impl hipGraphNodeType {
    ///< BatchMemOp node
    pub const hipGraphNodeTypeBatchMemOp: hipGraphNodeType = hipGraphNodeType(14);
}
impl hipGraphNodeType {
    pub const hipGraphNodeTypeCount: hipGraphNodeType = hipGraphNodeType(15);
}
#[repr(transparent)]
/// hipGraphNodeType
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipGraphNodeType(pub ::core::ffi::c_uint);
pub type hipHostFn_t = ::core::option::Option<
    unsafe extern "C" fn(userData: *mut ::core::ffi::c_void),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipHostNodeParams {
    pub fn_: hipHostFn_t,
    pub userData: *mut ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipKernelNodeParams {
    pub blockDim: dim3,
    pub extra: *mut *mut ::core::ffi::c_void,
    pub func: *mut ::core::ffi::c_void,
    pub gridDim: dim3,
    pub kernelParams: *mut *mut ::core::ffi::c_void,
    pub sharedMemBytes: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipMemsetParams {
    pub dst: *mut ::core::ffi::c_void,
    pub elementSize: ::core::ffi::c_uint,
    pub height: usize,
    pub pitch: usize,
    pub value: ::core::ffi::c_uint,
    pub width: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipMemAllocNodeParams {
    /**< Pool properties, which contain where
< the location should reside*/
    pub poolProps: hipMemPoolProps,
    ///< The number of memory access descriptors.
    pub accessDescs: *const hipMemAccessDesc,
    /**< The number of access descriptors.
< Must not be bigger than the number of GPUs*/
    pub accessDescCount: usize,
    ///< The size of the requested allocation in bytes
    pub bytesize: usize,
    ///< Returned device address of the allocation
    pub dptr: *mut ::core::ffi::c_void,
}
impl hipAccessProperty {
    pub const hipAccessPropertyNormal: hipAccessProperty = hipAccessProperty(0);
}
impl hipAccessProperty {
    pub const hipAccessPropertyStreaming: hipAccessProperty = hipAccessProperty(1);
}
impl hipAccessProperty {
    pub const hipAccessPropertyPersisting: hipAccessProperty = hipAccessProperty(2);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipAccessProperty(pub ::core::ffi::c_uint);
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct hipAccessPolicyWindow {
    pub base_ptr: *mut ::core::ffi::c_void,
    pub hitProp: hipAccessProperty,
    pub hitRatio: f32,
    pub missProp: hipAccessProperty,
    pub num_bytes: usize,
}
impl hipLaunchAttributeID {
    ///< Valid for Streams, graph nodes, launches
    pub const hipLaunchAttributeAccessPolicyWindow: hipLaunchAttributeID = hipLaunchAttributeID(
        1,
    );
}
impl hipLaunchAttributeID {
    ///< Valid for graph nodes, launches
    pub const hipLaunchAttributeCooperative: hipLaunchAttributeID = hipLaunchAttributeID(
        2,
    );
}
impl hipLaunchAttributeID {
    ///< Valid for graph node, streams, launches
    pub const hipLaunchAttributePriority: hipLaunchAttributeID = hipLaunchAttributeID(8);
}
#[repr(transparent)]
///  Launch Attribute ID
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipLaunchAttributeID(pub ::core::ffi::c_uint);
///  Launch Attribute Value
#[repr(C)]
#[derive(Copy, Clone)]
pub union hipLaunchAttributeValue {
    /**< Value of launch attribute::
hipLaunchAttributePolicyWindow.*/
    pub accessPolicyWindow: hipAccessPolicyWindow,
    ///< Value of launch attribute ::hipLaunchAttributeCooperative
    pub cooperative: ::core::ffi::c_int,
    /**< Value of launch attribute :: hipLaunchAttributePriority. Execution
priority of kernel.*/
    pub priority: ::core::ffi::c_int,
}
/// Memset node params
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct HIP_MEMSET_NODE_PARAMS {
    ///< Destination pointer on device
    pub dst: hipDeviceptr_t,
    ///< Destination device pointer pitch. Unused if height equals 1
    pub pitch: usize,
    ///< Value of memset to be set
    pub value: ::core::ffi::c_uint,
    ///< Element in bytes. Must be 1, 2, or 4.
    pub elementSize: ::core::ffi::c_uint,
    ///< Width of a row
    pub width: usize,
    ///< Number of rows
    pub height: usize,
}
impl hipGraphExecUpdateResult {
    ///< The update succeeded
    pub const hipGraphExecUpdateSuccess: hipGraphExecUpdateResult = hipGraphExecUpdateResult(
        0,
    );
}
impl hipGraphExecUpdateResult {
    /**< The update failed for an unexpected reason which is described
< in the return value of the function*/
    pub const hipGraphExecUpdateError: hipGraphExecUpdateResult = hipGraphExecUpdateResult(
        1,
    );
}
impl hipGraphExecUpdateResult {
    ///< The update failed because the topology changed
    pub const hipGraphExecUpdateErrorTopologyChanged: hipGraphExecUpdateResult = hipGraphExecUpdateResult(
        2,
    );
}
impl hipGraphExecUpdateResult {
    ///< The update failed because a node type changed
    pub const hipGraphExecUpdateErrorNodeTypeChanged: hipGraphExecUpdateResult = hipGraphExecUpdateResult(
        3,
    );
}
impl hipGraphExecUpdateResult {
    pub const hipGraphExecUpdateErrorFunctionChanged: hipGraphExecUpdateResult = hipGraphExecUpdateResult(
        4,
    );
}
impl hipGraphExecUpdateResult {
    pub const hipGraphExecUpdateErrorParametersChanged: hipGraphExecUpdateResult = hipGraphExecUpdateResult(
        5,
    );
}
impl hipGraphExecUpdateResult {
    pub const hipGraphExecUpdateErrorNotSupported: hipGraphExecUpdateResult = hipGraphExecUpdateResult(
        6,
    );
}
impl hipGraphExecUpdateResult {
    pub const hipGraphExecUpdateErrorUnsupportedFunctionChange: hipGraphExecUpdateResult = hipGraphExecUpdateResult(
        7,
    );
}
#[repr(transparent)]
/// Graph execution update result
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipGraphExecUpdateResult(pub ::core::ffi::c_uint);
impl hipStreamCaptureMode {
    pub const hipStreamCaptureModeGlobal: hipStreamCaptureMode = hipStreamCaptureMode(0);
}
impl hipStreamCaptureMode {
    pub const hipStreamCaptureModeThreadLocal: hipStreamCaptureMode = hipStreamCaptureMode(
        1,
    );
}
impl hipStreamCaptureMode {
    pub const hipStreamCaptureModeRelaxed: hipStreamCaptureMode = hipStreamCaptureMode(
        2,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipStreamCaptureMode(pub ::core::ffi::c_uint);
impl hipStreamCaptureStatus {
    ///< Stream is not capturing
    pub const hipStreamCaptureStatusNone: hipStreamCaptureStatus = hipStreamCaptureStatus(
        0,
    );
}
impl hipStreamCaptureStatus {
    ///< Stream is actively capturing
    pub const hipStreamCaptureStatusActive: hipStreamCaptureStatus = hipStreamCaptureStatus(
        1,
    );
}
impl hipStreamCaptureStatus {
    /**< Stream is part of a capture sequence that has been
< invalidated, but not terminated*/
    pub const hipStreamCaptureStatusInvalidated: hipStreamCaptureStatus = hipStreamCaptureStatus(
        2,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipStreamCaptureStatus(pub ::core::ffi::c_uint);
impl hipStreamUpdateCaptureDependenciesFlags {
    ///< Add new nodes to the dependency set
    pub const hipStreamAddCaptureDependencies: hipStreamUpdateCaptureDependenciesFlags = hipStreamUpdateCaptureDependenciesFlags(
        0,
    );
}
impl hipStreamUpdateCaptureDependenciesFlags {
    ///< Replace the dependency set with the new nodes
    pub const hipStreamSetCaptureDependencies: hipStreamUpdateCaptureDependenciesFlags = hipStreamUpdateCaptureDependenciesFlags(
        1,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipStreamUpdateCaptureDependenciesFlags(pub ::core::ffi::c_uint);
impl hipGraphMemAttributeType {
    ///< Amount of memory, in bytes, currently associated with graphs
    pub const hipGraphMemAttrUsedMemCurrent: hipGraphMemAttributeType = hipGraphMemAttributeType(
        0,
    );
}
impl hipGraphMemAttributeType {
    ///< High watermark of memory, in bytes, associated with graphs since the last time.
    pub const hipGraphMemAttrUsedMemHigh: hipGraphMemAttributeType = hipGraphMemAttributeType(
        1,
    );
}
impl hipGraphMemAttributeType {
    ///< Amount of memory, in bytes, currently allocated for graphs.
    pub const hipGraphMemAttrReservedMemCurrent: hipGraphMemAttributeType = hipGraphMemAttributeType(
        2,
    );
}
impl hipGraphMemAttributeType {
    ///< High watermark of memory, in bytes, currently allocated for graphs
    pub const hipGraphMemAttrReservedMemHigh: hipGraphMemAttributeType = hipGraphMemAttributeType(
        3,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipGraphMemAttributeType(pub ::core::ffi::c_uint);
impl hipUserObjectFlags {
    ///< Destructor execution is not synchronized.
    pub const hipUserObjectNoDestructorSync: hipUserObjectFlags = hipUserObjectFlags(1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipUserObjectFlags(pub ::core::ffi::c_uint);
impl hipUserObjectRetainFlags {
    ///< Add new reference or retain.
    pub const hipGraphUserObjectMove: hipUserObjectRetainFlags = hipUserObjectRetainFlags(
        1,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipUserObjectRetainFlags(pub ::core::ffi::c_uint);
impl hipGraphInstantiateFlags {
    pub const hipGraphInstantiateFlagAutoFreeOnLaunch: hipGraphInstantiateFlags = hipGraphInstantiateFlags(
        1,
    );
}
impl hipGraphInstantiateFlags {
    pub const hipGraphInstantiateFlagUpload: hipGraphInstantiateFlags = hipGraphInstantiateFlags(
        2,
    );
}
impl hipGraphInstantiateFlags {
    pub const hipGraphInstantiateFlagDeviceLaunch: hipGraphInstantiateFlags = hipGraphInstantiateFlags(
        4,
    );
}
impl hipGraphInstantiateFlags {
    pub const hipGraphInstantiateFlagUseNodePriority: hipGraphInstantiateFlags = hipGraphInstantiateFlags(
        8,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipGraphInstantiateFlags(pub ::core::ffi::c_uint);
impl hipGraphDebugDotFlags {
    pub const hipGraphDebugDotFlagsVerbose: hipGraphDebugDotFlags = hipGraphDebugDotFlags(
        1,
    );
}
impl hipGraphDebugDotFlags {
    ///< Adds hipKernelNodeParams to output
    pub const hipGraphDebugDotFlagsKernelNodeParams: hipGraphDebugDotFlags = hipGraphDebugDotFlags(
        4,
    );
}
impl hipGraphDebugDotFlags {
    ///< Adds hipMemcpy3DParms to output
    pub const hipGraphDebugDotFlagsMemcpyNodeParams: hipGraphDebugDotFlags = hipGraphDebugDotFlags(
        8,
    );
}
impl hipGraphDebugDotFlags {
    ///< Adds hipMemsetParams to output
    pub const hipGraphDebugDotFlagsMemsetNodeParams: hipGraphDebugDotFlags = hipGraphDebugDotFlags(
        16,
    );
}
impl hipGraphDebugDotFlags {
    ///< Adds hipHostNodeParams to output
    pub const hipGraphDebugDotFlagsHostNodeParams: hipGraphDebugDotFlags = hipGraphDebugDotFlags(
        32,
    );
}
impl hipGraphDebugDotFlags {
    pub const hipGraphDebugDotFlagsEventNodeParams: hipGraphDebugDotFlags = hipGraphDebugDotFlags(
        64,
    );
}
impl hipGraphDebugDotFlags {
    pub const hipGraphDebugDotFlagsExtSemasSignalNodeParams: hipGraphDebugDotFlags = hipGraphDebugDotFlags(
        128,
    );
}
impl hipGraphDebugDotFlags {
    pub const hipGraphDebugDotFlagsExtSemasWaitNodeParams: hipGraphDebugDotFlags = hipGraphDebugDotFlags(
        256,
    );
}
impl hipGraphDebugDotFlags {
    pub const hipGraphDebugDotFlagsKernelNodeAttributes: hipGraphDebugDotFlags = hipGraphDebugDotFlags(
        512,
    );
}
impl hipGraphDebugDotFlags {
    pub const hipGraphDebugDotFlagsHandles: hipGraphDebugDotFlags = hipGraphDebugDotFlags(
        1024,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipGraphDebugDotFlags(pub ::core::ffi::c_uint);
impl hipGraphInstantiateResult {
    ///< Instantiation Success
    pub const hipGraphInstantiateSuccess: hipGraphInstantiateResult = hipGraphInstantiateResult(
        0,
    );
}
impl hipGraphInstantiateResult {
    /**< Instantiation failed for an
unexpected reason which is described in the return value of the function*/
    pub const hipGraphInstantiateError: hipGraphInstantiateResult = hipGraphInstantiateResult(
        1,
    );
}
impl hipGraphInstantiateResult {
    /**< Instantiation failed due
to invalid structure, such as cycles*/
    pub const hipGraphInstantiateInvalidStructure: hipGraphInstantiateResult = hipGraphInstantiateResult(
        2,
    );
}
impl hipGraphInstantiateResult {
    /**< Instantiation for device launch failed
because the graph contained an unsupported operation*/
    pub const hipGraphInstantiateNodeOperationNotSupported: hipGraphInstantiateResult = hipGraphInstantiateResult(
        3,
    );
}
impl hipGraphInstantiateResult {
    /**< Instantiation for device launch failed
due to the nodes belonging to different contexts*/
    pub const hipGraphInstantiateMultipleDevicesNotSupported: hipGraphInstantiateResult = hipGraphInstantiateResult(
        4,
    );
}
#[repr(transparent)]
/// hipGraphInstantiateWithParams results
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipGraphInstantiateResult(pub ::core::ffi::c_uint);
/// Graph Instantiation parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipGraphInstantiateParams {
    ///< The node which caused instantiation to fail, if any
    pub errNode_out: hipGraphNode_t,
    ///< Instantiation flags
    pub flags: ::core::ffi::c_ulonglong,
    /**< Whether instantiation was successful.
If it failed, the reason why*/
    pub result_out: hipGraphInstantiateResult,
    ///< Upload stream
    pub uploadStream: hipStream_t,
}
/// Memory allocation properties
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipMemAllocationProp {
    ///< Memory allocation type
    pub type_: hipMemAllocationType,
    ///< Requested handle type
    pub requestedHandleType: hipMemAllocationHandleType,
    ///< Memory location
    pub location: hipMemLocation,
    ///< Metadata for Win32 handles
    pub win32HandleMetaData: *mut ::core::ffi::c_void,
    pub allocFlags: hipMemAllocationProp__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipMemAllocationProp__bindgen_ty_1 {
    ///< Compression type
    pub compressionType: ::core::ffi::c_uchar,
    ///< RDMA capable
    pub gpuDirectRDMACapable: ::core::ffi::c_uchar,
    ///< Usage
    pub usage: ::core::ffi::c_ushort,
}
/// External semaphore signal node parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipExternalSemaphoreSignalNodeParams {
    pub extSemArray: *mut hipExternalSemaphore_t,
    pub paramsArray: *const hipExternalSemaphoreSignalParams,
    pub numExtSems: ::core::ffi::c_uint,
}
/// External semaphore wait node parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipExternalSemaphoreWaitNodeParams {
    pub extSemArray: *mut hipExternalSemaphore_t,
    pub paramsArray: *const hipExternalSemaphoreWaitParams,
    pub numExtSems: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ihipMemGenericAllocationHandle {
    _unused: [u8; 0],
}
/// Generic handle for memory allocation
pub type hipMemGenericAllocationHandle_t = *mut ihipMemGenericAllocationHandle;
impl hipMemAllocationGranularity_flags {
    ///< Minimum granularity
    pub const hipMemAllocationGranularityMinimum: hipMemAllocationGranularity_flags = hipMemAllocationGranularity_flags(
        0,
    );
}
impl hipMemAllocationGranularity_flags {
    ///< Recommended granularity for performance
    pub const hipMemAllocationGranularityRecommended: hipMemAllocationGranularity_flags = hipMemAllocationGranularity_flags(
        1,
    );
}
#[repr(transparent)]
/// Flags for granularity
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipMemAllocationGranularity_flags(pub ::core::ffi::c_uint);
impl hipMemHandleType {
    ///< Generic handle type
    pub const hipMemHandleTypeGeneric: hipMemHandleType = hipMemHandleType(0);
}
#[repr(transparent)]
/// Memory handle type
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipMemHandleType(pub ::core::ffi::c_uint);
impl hipMemOperationType {
    ///< Map operation
    pub const hipMemOperationTypeMap: hipMemOperationType = hipMemOperationType(1);
}
impl hipMemOperationType {
    ///< Unmap operation
    pub const hipMemOperationTypeUnmap: hipMemOperationType = hipMemOperationType(2);
}
#[repr(transparent)]
/// Memory operation types
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipMemOperationType(pub ::core::ffi::c_uint);
impl hipArraySparseSubresourceType {
    ///< Sparse level
    pub const hipArraySparseSubresourceTypeSparseLevel: hipArraySparseSubresourceType = hipArraySparseSubresourceType(
        0,
    );
}
impl hipArraySparseSubresourceType {
    ///< Miptail
    pub const hipArraySparseSubresourceTypeMiptail: hipArraySparseSubresourceType = hipArraySparseSubresourceType(
        1,
    );
}
#[repr(transparent)]
/// Subresource types for sparse arrays
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipArraySparseSubresourceType(pub ::core::ffi::c_uint);
/// Map info for arrays
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hipArrayMapInfo {
    ///< Resource type
    pub resourceType: hipResourceType,
    pub resource: hipArrayMapInfo__bindgen_ty_1,
    ///< Sparse subresource type
    pub subresourceType: hipArraySparseSubresourceType,
    pub subresource: hipArrayMapInfo__bindgen_ty_2,
    ///< Memory operation type
    pub memOperationType: hipMemOperationType,
    ///< Memory handle type
    pub memHandleType: hipMemHandleType,
    pub memHandle: hipArrayMapInfo__bindgen_ty_3,
    ///< Offset within the memory
    pub offset: ::core::ffi::c_ulonglong,
    ///< Device ordinal bit mask
    pub deviceBitMask: ::core::ffi::c_uint,
    ///< flags for future use, must be zero now.
    pub flags: ::core::ffi::c_uint,
    ///< Reserved for future use, must be zero now.
    pub reserved: [::core::ffi::c_uint; 2usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hipArrayMapInfo__bindgen_ty_1 {
    pub mipmap: hipMipmappedArray,
    pub array: hipArray_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hipArrayMapInfo__bindgen_ty_2 {
    pub sparseLevel: hipArrayMapInfo__bindgen_ty_2__bindgen_ty_1,
    pub miptail: hipArrayMapInfo__bindgen_ty_2__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipArrayMapInfo__bindgen_ty_2__bindgen_ty_1 {
    ///< For mipmapped arrays must be a valid mipmap level. For arrays must be zero
    pub level: ::core::ffi::c_uint,
    ///< For layered arrays must be a valid layer index. Otherwise, must be zero
    pub layer: ::core::ffi::c_uint,
    ///< X offset in elements
    pub offsetX: ::core::ffi::c_uint,
    ///< Y offset in elements
    pub offsetY: ::core::ffi::c_uint,
    ///< Z offset in elements
    pub offsetZ: ::core::ffi::c_uint,
    ///< Width in elements
    pub extentWidth: ::core::ffi::c_uint,
    ///< Height in elements
    pub extentHeight: ::core::ffi::c_uint,
    ///< Depth in elements
    pub extentDepth: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipArrayMapInfo__bindgen_ty_2__bindgen_ty_2 {
    ///< For layered arrays must be a valid layer index. Otherwise, must be zero
    pub layer: ::core::ffi::c_uint,
    ///< Offset within mip tail
    pub offset: ::core::ffi::c_ulonglong,
    ///< Extent in bytes
    pub size: ::core::ffi::c_ulonglong,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hipArrayMapInfo__bindgen_ty_3 {
    pub memHandle: hipMemGenericAllocationHandle_t,
}
/// Memcpy node params
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipMemcpyNodeParams {
    ///< Must be zero.
    pub flags: ::core::ffi::c_int,
    ///< Must be zero.
    pub reserved: [::core::ffi::c_int; 3usize],
    ///< Params set for the memory copy.
    pub copyParams: hipMemcpy3DParms,
}
/// Child graph node params
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipChildGraphNodeParams {
    /**< Either the child graph to clone into the node, or
< a handle to the graph possesed by the node used during query*/
    pub graph: hipGraph_t,
}
/// Event record node params
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipEventWaitNodeParams {
    ///< Event to wait on
    pub event: hipEvent_t,
}
/// Event record node params
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipEventRecordNodeParams {
    ///< The event to be recorded when node executes
    pub event: hipEvent_t,
}
/// Memory free node params
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipMemFreeNodeParams {
    ///< the pointer to be freed
    pub dptr: *mut ::core::ffi::c_void,
}
/// Params for different graph nodes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hipGraphNodeParams {
    pub type_: hipGraphNodeType,
    pub reserved0: [::core::ffi::c_int; 3usize],
    pub __bindgen_anon_1: hipGraphNodeParams__bindgen_ty_1,
    pub reserved2: ::core::ffi::c_longlong,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hipGraphNodeParams__bindgen_ty_1 {
    pub reserved1: [::core::ffi::c_longlong; 29usize],
    pub kernel: hipKernelNodeParams,
    pub memcpy: hipMemcpyNodeParams,
    pub memset: hipMemsetParams,
    pub host: hipHostNodeParams,
    pub graph: hipChildGraphNodeParams,
    pub eventWait: hipEventWaitNodeParams,
    pub eventRecord: hipEventRecordNodeParams,
    pub extSemSignal: hipExternalSemaphoreSignalNodeParams,
    pub extSemWait: hipExternalSemaphoreWaitNodeParams,
    pub alloc: hipMemAllocNodeParams,
    pub free: hipMemFreeNodeParams,
}
impl hipGraphDependencyType {
    pub const hipGraphDependencyTypeDefault: hipGraphDependencyType = hipGraphDependencyType(
        0,
    );
}
impl hipGraphDependencyType {
    pub const hipGraphDependencyTypeProgrammatic: hipGraphDependencyType = hipGraphDependencyType(
        1,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipGraphDependencyType(pub ::core::ffi::c_uint);
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct hipGraphEdgeData {
    /**< This indicates when the dependency is triggered from the upstream node on the
< edge. The meaning is specfic to the node type. A value of 0 in all cases
< means full completion of the upstream node, with memory visibility to the
< downstream node or portion thereof (indicated by to_port). Only kernel nodes
< define non-zero ports. A kernel node can use the following output port types:
< hipGraphKernelNodePortDefault, hipGraphKernelNodePortProgrammatic, or
< hipGraphKernelNodePortLaunchCompletion.*/
    pub from_port: ::core::ffi::c_uchar,
    ///< These bytes are unused and must be zeroed
    pub reserved: [::core::ffi::c_uchar; 5usize],
    ///< Currently no node types define non-zero ports. This field must be set to zero.
    pub to_port: ::core::ffi::c_uchar,
    ///< This should be populated with a value from hipGraphDependencyType
    pub type_: ::core::ffi::c_uchar,
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " @}\n/\n/**\n  @defgroup API HIP API\n  @{\n\n  Defines the HIP API.  See the individual sections for more information.\n/\n/**\n  @defgroup Driver Initialization and Version\n  @{\n  This section describes the initializtion and version functions of HIP runtime API.\n\n/\n/**\n @brief Explicitly initializes the HIP runtime.\n\n @param [in] flags  Initialization flag, should be zero.\n\n Most HIP APIs implicitly initialize the HIP runtime.\n This API provides control over the timing of the initialization.\n\n @returns #hipSuccess, #hipErrorInvalidValue"]
    pub fn hipInit(flags: ::core::ffi::c_uint) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns the approximate HIP driver version.

 @param [out] driverVersion driver version

 HIP driver version shows up in the format:
 HIP_VERSION_MAJOR * 10000000 + HIP_VERSION_MINOR * 100000 + HIP_VERSION_PATCH.

 @returns #hipSuccess, #hipErrorInvalidValue

 @warning The HIP driver version does not correspond to an exact CUDA driver revision.
 On AMD platform, the API returns the HIP driver version, while on NVIDIA platform, it calls
 the corresponding CUDA runtime API and returns the CUDA driver version.
 There is no mapping/correlation between HIP driver version and CUDA driver version.

 @see hipRuntimeGetVersion*/
    pub fn hipDriverGetVersion(driverVersion: *mut ::core::ffi::c_int) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns the approximate HIP Runtime version.

 @param [out] runtimeVersion HIP runtime version

 @returns #hipSuccess, #hipErrorInvalidValue

 @warning The version definition of HIP runtime is different from CUDA.
 On AMD platform, the function returns HIP runtime version,
 while on NVIDIA platform, it returns CUDA runtime version.
 And there is no mapping/correlation between HIP version and CUDA version.

 @see hipDriverGetVersion*/
    pub fn hipRuntimeGetVersion(runtimeVersion: *mut ::core::ffi::c_int) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns a handle to a compute device
 @param [out] device Handle of device
 @param [in] ordinal Device ordinal

 @returns #hipSuccess, #hipErrorInvalidDevice*/
    pub fn hipDeviceGet(
        device: *mut hipDevice_t,
        ordinal: ::core::ffi::c_int,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns the compute capability of the device
 @param [out] major Major compute capability version number
 @param [out] minor Minor compute capability version number
 @param [in] device Device ordinal

 @returns #hipSuccess, #hipErrorInvalidDevice*/
    pub fn hipDeviceComputeCapability(
        major: *mut ::core::ffi::c_int,
        minor: *mut ::core::ffi::c_int,
        device: hipDevice_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns an identifer string for the device.
 @param [out] name String of the device name
 @param [in] len Maximum length of string to store in device name
 @param [in] device Device ordinal

 @returns #hipSuccess, #hipErrorInvalidDevice*/
    pub fn hipDeviceGetName(
        name: *mut ::core::ffi::c_char,
        len: ::core::ffi::c_int,
        device: hipDevice_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns an UUID for the device.[BETA]
 @param [out] uuid UUID for the device
 @param [in] device device ordinal

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @returns #hipSuccess, #hipErrorInvalidDevice, #hipErrorInvalidValue, #hipErrorNotInitialized,
 #hipErrorDeinitialized*/
    pub fn hipDeviceGetUuid(uuid: *mut hipUUID, device: hipDevice_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns a value for attribute of link between two devices
 @param [out] value Pointer of the value for the attrubute
 @param [in] attr enum of hipDeviceP2PAttr to query
 @param [in] srcDevice The source device of the link
 @param [in] dstDevice The destination device of the link

 @returns #hipSuccess, #hipErrorInvalidDevice*/
    pub fn hipDeviceGetP2PAttribute(
        value: *mut ::core::ffi::c_int,
        attr: hipDeviceP2PAttr,
        srcDevice: ::core::ffi::c_int,
        dstDevice: ::core::ffi::c_int,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns a PCI Bus Id string for the device, overloaded to take int device ID.
 @param [out] pciBusId The string of PCI Bus Id format for the device
 @param [in] len Maximum length of string
 @param [in] device The device ordinal

 @returns #hipSuccess, #hipErrorInvalidDevice*/
    pub fn hipDeviceGetPCIBusId(
        pciBusId: *mut ::core::ffi::c_char,
        len: ::core::ffi::c_int,
        device: ::core::ffi::c_int,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns a handle to a compute device.
 @param [out] device The handle of the device
 @param [in] pciBusId The string of PCI Bus Id for the device

 @returns #hipSuccess, #hipErrorInvalidDevice, #hipErrorInvalidValue*/
    pub fn hipDeviceGetByPCIBusId(
        device: *mut ::core::ffi::c_int,
        pciBusId: *const ::core::ffi::c_char,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns the total amount of memory on the device.
 @param [out] bytes The size of memory in bytes, on the device
 @param [in] device The ordinal of the device

 @returns #hipSuccess, #hipErrorInvalidDevice*/
    pub fn hipDeviceTotalMem(bytes: *mut usize, device: hipDevice_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " @}\n/\n/**\n  @defgroup Device Device Management\n  @{\n  This section describes the device management functions of HIP runtime API.\n/\n/**\n @brief Waits on all active streams on current device\n\n When this command is invoked, the host thread gets blocked until all the commands associated\n with streams associated with the device. HIP does not support multiple blocking modes (yet!).\n\n @returns #hipSuccess\n\n @see hipSetDevice, hipDeviceReset"]
    pub fn hipDeviceSynchronize() -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief The state of current device is discarded and updated to a fresh state.

 Calling this function deletes all streams created, memory allocated, kernels running, events
 created. Make sure that no other thread is using the device or streams, memory, kernels, events
 associated with the current device.

 @returns #hipSuccess

 @see hipDeviceSynchronize*/
    pub fn hipDeviceReset() -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set default device to be used for subsequent hip API calls from this thread.

 @param[in] deviceId Valid device in range 0...hipGetDeviceCount().

 Sets @p device as the default device for the calling host thread.  Valid device id's are 0...
 (hipGetDeviceCount()-1).

 Many HIP APIs implicitly use the "default device" :

 - Any device memory subsequently allocated from this host thread (using hipMalloc) will be
 allocated on device.
 - Any streams or events created from this host thread will be associated with device.
 - Any kernels launched from this host thread (using hipLaunchKernel) will be executed on device
 (unless a specific stream is specified, in which case the device associated with that stream will
 be used).

 This function may be called from any host thread.  Multiple host threads may use the same device.
 This function does no synchronization with the previous or new device, and has very little
 runtime overhead. Applications can use hipSetDevice to quickly switch the default device before
 making a HIP runtime call which uses the default device.

 The default device is stored in thread-local-storage for each thread.
 Thread-pool implementations may inherit the default device of the previous thread.  A good
 practice is to always call hipSetDevice at the start of HIP coding sequency to establish a known
 standard device.

 @returns #hipSuccess, #hipErrorInvalidDevice, #hipErrorNoDevice

 @see #hipGetDevice, #hipGetDeviceCount*/
    pub fn hipSetDevice(deviceId: ::core::ffi::c_int) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set a list of devices that can be used.

 @param[in] device_arr List of devices to try
 @param[in] len Number of devices in specified list

 @returns #hipSuccess, #hipErrorInvalidDevice, #hipErrorInvalidValue

 @see #hipGetDevice, #hipGetDeviceCount. #hipSetDevice. #hipGetDeviceProperties. #hipSetDeviceFlags. #hipChooseDevice
*/
    pub fn hipSetValidDevices(
        device_arr: *mut ::core::ffi::c_int,
        len: ::core::ffi::c_int,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Return the default device id for the calling host thread.

 @param [out] deviceId *device is written with the default device

 HIP maintains an default device for each thread using thread-local-storage.
 This device is used implicitly for HIP runtime APIs called by this thread.
 hipGetDevice returns in * @p device the default device for the calling host thread.

 @returns #hipSuccess, #hipErrorInvalidDevice, #hipErrorInvalidValue

 @see hipSetDevice, hipGetDevicesizeBytes*/
    pub fn hipGetDevice(deviceId: *mut ::core::ffi::c_int) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Return number of compute-capable devices.

 @param [out] count Returns number of compute-capable devices.

 @returns #hipSuccess, #hipErrorNoDevice


 Returns in @p *count the number of devices that have ability to run compute commands.  If there
 are no such devices, then @ref hipGetDeviceCount will return #hipErrorNoDevice. If 1 or more
 devices can be found, then hipGetDeviceCount returns #hipSuccess.*/
    pub fn hipGetDeviceCount(count: *mut ::core::ffi::c_int) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Query for a specific device attribute.

 @param [out] pi pointer to value to return
 @param [in] attr attribute to query
 @param [in] deviceId which device to query for information

 @returns #hipSuccess, #hipErrorInvalidDevice, #hipErrorInvalidValue*/
    pub fn hipDeviceGetAttribute(
        pi: *mut ::core::ffi::c_int,
        attr: hipDeviceAttribute_t,
        deviceId: ::core::ffi::c_int,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns the default memory pool of the specified device

 @param [out] mem_pool Default memory pool to return
 @param [in] device    Device index for query the default memory pool

 @returns #hipSuccess, #hipErrorInvalidDevice, #hipErrorInvalidValue, #hipErrorNotSupported

 @see hipDeviceGetDefaultMemPool, hipMallocAsync, hipMemPoolTrimTo, hipMemPoolGetAttribute,
 hipDeviceSetMemPool, hipMemPoolSetAttribute, hipMemPoolSetAccess, hipMemPoolGetAccess

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipDeviceGetDefaultMemPool(
        mem_pool: *mut hipMemPool_t,
        device: ::core::ffi::c_int,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets the current memory pool of a device

 The memory pool must be local to the specified device.
 @p hipMallocAsync allocates from the current mempool of the provided stream's device.
 By default, a device's current memory pool is its default memory pool.

 @note Use @p hipMallocFromPoolAsync for asynchronous memory allocations from a device
 different than the one the stream runs on.

 @param [in] device   Device index for the update
 @param [in] mem_pool Memory pool for update as the current on the specified device

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidDevice, #hipErrorNotSupported

 @see hipDeviceGetDefaultMemPool, hipMallocAsync, hipMemPoolTrimTo, hipMemPoolGetAttribute,
 hipDeviceSetMemPool, hipMemPoolSetAttribute, hipMemPoolSetAccess, hipMemPoolGetAccess

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipDeviceSetMemPool(
        device: ::core::ffi::c_int,
        mem_pool: hipMemPool_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets the current memory pool for the specified device

 Returns the last pool provided to @p hipDeviceSetMemPool for this device
 or the device's default memory pool if @p hipDeviceSetMemPool has never been called.
 By default the current mempool is the default mempool for a device,
 otherwise the returned pool must have been set with @p hipDeviceSetMemPool.

 @param [out] mem_pool Current memory pool on the specified device
 @param [in] device    Device index to query the current memory pool

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported

 @see hipDeviceGetDefaultMemPool, hipMallocAsync, hipMemPoolTrimTo, hipMemPoolGetAttribute,
 hipDeviceSetMemPool, hipMemPoolSetAttribute, hipMemPoolSetAccess, hipMemPoolGetAccess

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipDeviceGetMemPool(
        mem_pool: *mut hipMemPool_t,
        device: ::core::ffi::c_int,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns device properties.

 @param [out] prop written with device properties
 @param [in]  deviceId which device to query for information

 @returns #hipSuccess, #hipErrorInvalidDevice
 @bug HCC always returns 0 for maxThreadsPerMultiProcessor
 @bug HCC always returns 0 for regsPerBlock
 @bug HCC always returns 0 for l2CacheSize

 Populates hipGetDeviceProperties with information for the specified device.*/
    pub fn hipGetDevicePropertiesR0600(
        prop: *mut hipDeviceProp_tR0600,
        deviceId: ::core::ffi::c_int,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets the maximum width for 1D linear textures on the specified device

 This function queries the maximum width, in elements, of 1D linear textures that can be allocated
 on the specified device. The maximum width depends on the texture element size and the hardware
 limitations of the device.

 @param [out] max_width Maximum width, in elements, of 1D linear textures that the device can support
 @param [in] device     Device index to query for maximum 1D texture width

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidDevice

 @see hipDeviceGetAttribute, hipMalloc, hipTexRefSetAddressMode*/
    pub fn hipDeviceGetTexture1DLinearMaxWidth(
        mem_pool: *mut hipMemPool_t,
        device: ::core::ffi::c_int,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set L1/Shared cache partition.

 @param [in] cacheConfig Cache configuration

 @returns #hipSuccess, #hipErrorNotInitialized, #hipErrorNotSupported

 Note: AMD devices do not support reconfigurable cache. This API is not implemented
 on AMD platform. If the function is called, it will return hipErrorNotSupported.
*/
    pub fn hipDeviceSetCacheConfig(cacheConfig: hipFuncCache_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get Cache configuration for a specific Device

 @param [out] cacheConfig Pointer of cache configuration

 @returns #hipSuccess, #hipErrorNotInitialized
 Note: AMD devices do not support reconfigurable cache. This hint is ignored
 on these architectures.
*/
    pub fn hipDeviceGetCacheConfig(cacheConfig: *mut hipFuncCache_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets resource limits of current device

 The function queries the size of limit value, as required by the input enum value hipLimit_t,
 which can be either #hipLimitStackSize, or #hipLimitMallocHeapSize. Any other input as
 default, the function will return #hipErrorUnsupportedLimit.

 @param [out] pValue Returns the size of the limit in bytes
 @param [in]  limit The limit to query

 @returns #hipSuccess, #hipErrorUnsupportedLimit, #hipErrorInvalidValue
*/
    pub fn hipDeviceGetLimit(pValue: *mut usize, limit: hipLimit_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets resource limits of current device.

 As the input enum limit,
 #hipLimitStackSize sets the limit value of the stack size on the current GPU device, per thread.
 The limit size can get via hipDeviceGetLimit. The size is in units of 256 dwords, up to the limit
 (128K - 16).

 #hipLimitMallocHeapSize sets the limit value of the heap used by the malloc()/free()
 calls. For limit size, use the #hipDeviceGetLimit API.

 Any other input as default, the funtion will return hipErrorUnsupportedLimit.

 @param [in] limit Enum of hipLimit_t to set
 @param [in] value The size of limit value in bytes

 @returns #hipSuccess, #hipErrorUnsupportedLimit, #hipErrorInvalidValue
*/
    pub fn hipDeviceSetLimit(limit: hipLimit_t, value: usize) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns bank width of shared memory for current device

 @param [out] pConfig The pointer of the bank width for shared memory

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotInitialized

 Note: AMD devices and some Nvidia GPUS do not support shared cache banking, and the hint is
 ignored on those architectures.
*/
    pub fn hipDeviceGetSharedMemConfig(pConfig: *mut hipSharedMemConfig) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets the flags set for current device

 @param [out] flags Pointer of the flags

 @returns #hipSuccess, #hipErrorInvalidDevice, #hipErrorInvalidValue*/
    pub fn hipGetDeviceFlags(flags: *mut ::core::ffi::c_uint) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief The bank width of shared memory on current device is set

 @param [in] config Configuration for the bank width of shared memory

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotInitialized

 Note: AMD devices and some Nvidia GPUS do not support shared cache banking, and the hint is
 ignored on those architectures.
*/
    pub fn hipDeviceSetSharedMemConfig(config: hipSharedMemConfig) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief The current device behavior is changed according to the flags passed.

 @param [in] flags Flag to set on the current device

 The schedule flags impact how HIP waits for the completion of a command running on a device.

 #hipDeviceScheduleSpin         : HIP runtime will actively spin in the thread which submitted
 the work until the command completes.  This offers the lowest latency, but will consume a CPU
 core and may increase power.

 #hipDeviceScheduleYield        : The HIP runtime will yield the CPU to system so that other
 tasks can use it. This may increase latency to detect the completion but will consume less
 power and is friendlier to other tasks in the system.

 #hipDeviceScheduleBlockingSync : On ROCm platform, this is a synonym for hipDeviceScheduleYield.

 #hipDeviceScheduleAuto         : This is the default value if the input 'flags' is zero.
 Uses a heuristic to select between Spin and Yield modes. If the number of HIP contexts is
 greater than the number of logical processors in the system, uses Spin scheduling, otherwise
 uses Yield scheduling.

 #hipDeviceMapHost              : Allows mapping host memory. On ROCm, this is always allowed and
 the flag is ignored.

 #hipDeviceLmemResizeToMax      : This flag is silently ignored on ROCm.

 @returns #hipSuccess, #hipErrorNoDevice, #hipErrorInvalidDevice, #hipErrorSetOnActiveProcess

*/
    pub fn hipSetDeviceFlags(flags: ::core::ffi::c_uint) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Device which matches hipDeviceProp_t is returned

 @param [out] device Pointer of the device
 @param [in]  prop Pointer of the properties

 @returns #hipSuccess, #hipErrorInvalidValue*/
    pub fn hipChooseDeviceR0600(
        device: *mut ::core::ffi::c_int,
        prop: *const hipDeviceProp_tR0600,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns the link type and hop count between two devices

 @param [in] device1 Ordinal for device1
 @param [in] device2 Ordinal for device2
 @param [out] linktype Returns the link type (See hsa_amd_link_info_type_t) between the two devices
 @param [out] hopcount Returns the hop count between the two devices

 Queries and returns the HSA link type and the hop count between the two specified devices.

 @returns #hipSuccess, #hipErrorInvalidValue*/
    pub fn hipExtGetLinkTypeAndHopCount(
        device1: ::core::ffi::c_int,
        device2: ::core::ffi::c_int,
        linktype: *mut u32,
        hopcount: *mut u32,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets an interprocess memory handle for an existing device memory allocation.

 Takes a pointer to the base of an existing device memory allocation created with ::hipMalloc
 and exports it for use in another process. This is a lightweight operation and may be called
 multiple times on an allocation without adverse effects.

 If a region of memory is freed with ::hipFree and a subsequent call to ::hipMalloc returns
 memory with the same device address, ::hipIpcGetMemHandle will return a unique handle for
 the new memory.

 @param handle - Pointer to user allocated hipIpcMemHandle to return the handle in.
 @param devPtr - Base pointer to previously allocated device memory.

 @returns #hipSuccess, #hipErrorInvalidHandle, #hipErrorOutOfMemory, #hipErrorMapFailed

 @note This IPC memory related feature API on Windows may behave differently from Linux.
*/
    pub fn hipIpcGetMemHandle(
        handle: *mut hipIpcMemHandle_t,
        devPtr: *mut ::core::ffi::c_void,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Opens an interprocess memory handle exported from another process and returns a device
 pointer usable in the local process.

 Maps memory exported from another process with ::hipIpcGetMemHandle into the current device
 address space. For contexts on different devices ::hipIpcOpenMemHandle can attempt to enable
 peer access between the devices like the user called ::hipDeviceEnablePeerAccess.
 This behavior is controlled by the flag #hipIpcMemLazyEnablePeerAccess.
 The API ::hipDeviceCanAccessPeer can determine if a mapping is possible.

 hipIpcMemHandles from each device in a given process may only be opened by one context per
 device per other process.

 Memory returned from ::hipIpcOpenMemHandle must be freed with ::hipIpcCloseMemHandle.

 Calling ::hipFree on an exported memory region before calling ::hipIpcCloseMemHandle in the
 importing context will result in undefined behavior.

 @param devPtr - Returned device pointer
 @param handle - hipIpcMemHandle to open
 @param flags  - Flags for this operation. Must be specified as hipIpcMemLazyEnablePeerAccess

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidContext,
 #hipErrorInvalidDevicePointer

 @note During multiple processes, using the same memory handle opened by the current context,
 there is no guarantee that the same device pointer will be returned in @p *devPtr.
 This is diffrent from CUDA.

 @note This IPC memory related feature API on Windows may behave differently from Linux.
*/
    pub fn hipIpcOpenMemHandle(
        devPtr: *mut *mut ::core::ffi::c_void,
        handle: hipIpcMemHandle_t,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Close memory mapped with ::hipIpcOpenMemHandle

 Unmaps memory returned by ::hipIpcOpenMemHandle. The original allocation in the exporting
 process as well as imported mappings in other processes will be unaffected.

 Any resources used to enable peer access will be freed if this is the last mapping using them.

 @param devPtr - Device pointer returned by ::hipIpcOpenMemHandle

 @returns #hipSuccess, #hipErrorMapFailed, #hipErrorInvalidHandle

 @note This IPC memory related feature API on Windows may behave differently from Linux.
*/
    pub fn hipIpcCloseMemHandle(devPtr: *mut ::core::ffi::c_void) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets an opaque interprocess handle for an event.

 The event is previously allocated with #hipEventInterprocess and #hipEventDisableTiming flags.
 The opaque interprocess handle may be copied into other processes and opened with
 ::hipIpcOpenEventHandle. Then ::hipEventRecord, ::hipEventSynchronize, ::hipStreamWaitEvent and
 ::hipEventQuery may be used in either process. After the exported event has been freed with
 ::hipEventDestroy, operations on the imported event will result in undefined behavior.

 @param[out]  handle Pointer to #hipIpcEventHandle to return the opaque event handle
 @param[in]   event  Event allocated with #hipEventInterprocess and #hipEventDisableTiming flags

 @returns #hipSuccess, #hipErrorInvalidConfiguration, #hipErrorInvalidValue

 @note This IPC event related feature API is currently applicable on Linux.
*/
    pub fn hipIpcGetEventHandle(
        handle: *mut hipIpcEventHandle_t,
        event: hipEvent_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Opens an interprocess event handle.

 Opens an interprocess event handle exported from another process with ::hipIpcGetEventHandle.
 The returned #hipEvent_t behaves like a locally created event with the #hipEventDisableTiming
 flag specified. This event needs be freed with ::hipEventDestroy. After the exported event
 has been freed with ::hipEventDestroy, operations on the imported event will result in
 undefined behavior. If the input handle is from the same process, it will return
 #hipErrorInvalidContext.

 @param[out]  event  Pointer to hipEvent_t to return the imported event
 @param[in]   handle The opaque interprocess handle to open

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidContext

 @note This IPC event related feature API is currently applicable on Linux.
*/
    pub fn hipIpcOpenEventHandle(
        event: *mut hipEvent_t,
        handle: hipIpcEventHandle_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " @}\n/\n/**\n\n  @defgroup Execution Execution Control\n  @{\n  This section describes the execution control functions of HIP runtime API.\n\n/\n/**\n @brief Set attribute for a specific function\n\n @param [in] func Pointer of the function\n @param [in] attr Attribute to set\n @param [in] value Value to set\n\n @returns #hipSuccess, #hipErrorInvalidDeviceFunction, #hipErrorInvalidValue\n\n Note: AMD devices and some Nvidia GPUS do not support shared cache banking, and the hint is\n ignored on those architectures.\n"]
    pub fn hipFuncSetAttribute(
        func: *const ::core::ffi::c_void,
        attr: hipFuncAttribute,
        value: ::core::ffi::c_int,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set Cache configuration for a specific function

 @param [in] func Pointer of the function.
 @param [in] config Configuration to set.

 @returns #hipSuccess, #hipErrorNotInitialized
 Note: AMD devices and some Nvidia GPUS do not support reconfigurable cache.  This hint is ignored
 on those architectures.
*/
    pub fn hipFuncSetCacheConfig(
        func: *const ::core::ffi::c_void,
        config: hipFuncCache_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set shared memory configuation for a specific function

 @param [in] func Pointer of the function
 @param [in] config Configuration

 @returns #hipSuccess, #hipErrorInvalidDeviceFunction, #hipErrorInvalidValue

 Note: AMD devices and some Nvidia GPUS do not support shared cache banking, and the hint is
 ignored on those architectures.
*/
    pub fn hipFuncSetSharedMemConfig(
        func: *const ::core::ffi::c_void,
        config: hipSharedMemConfig,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " @}\n/\n/**\n-------------------------------------------------------------------------------------------------\n-------------------------------------------------------------------------------------------------\n  @defgroup Error Error Handling\n  @{\n  This section describes the error handling functions of HIP runtime API.\n/\n/**\n @brief Return last error returned by any HIP runtime API call and resets the stored error code to\n #hipSuccess\n\n @returns return code from last HIP called from the active host thread\n\n Returns the last error that has been returned by any of the runtime calls in the same host\n thread, and then resets the saved error to #hipSuccess.\n\n @see hipGetErrorString, hipGetLastError, hipPeakAtLastError, hipError_t"]
    pub fn hipGetLastError() -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Return last error returned by any HIP runtime API call and resets the stored error code to
 #hipSuccess

 @returns return code from last HIP called from the active host thread

 Returns the last error that has been returned by any of the runtime calls in the same host
 thread, and then resets the saved error to #hipSuccess.

 @see hipGetErrorString, hipGetLastError, hipPeakAtLastError, hipError_t*/
    pub fn hipExtGetLastError() -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Return last error returned by any HIP runtime API call.

 @returns #hipSuccess

 Returns the last error that has been returned by any of the runtime calls in the same host
 thread. Unlike hipGetLastError, this function does not reset the saved error code.

 @see hipGetErrorString, hipGetLastError, hipPeakAtLastError, hipError_t*/
    pub fn hipPeekAtLastError() -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    /** @brief Return hip error as text string form.

 @param hip_error Error code to convert to name.
 @returns const char pointer to the NULL-terminated error name

 @see hipGetErrorString, hipGetLastError, hipPeakAtLastError, hipError_t*/
    pub fn hipGetErrorName(hip_error: hipError_t) -> *const ::core::ffi::c_char;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    /** @brief Return handy text string message to explain the error which occurred

 @param hipError Error code to convert to string.
 @returns const char pointer to the NULL-terminated error string

 @see hipGetErrorName, hipGetLastError, hipPeakAtLastError, hipError_t*/
    pub fn hipGetErrorString(hipError: hipError_t) -> *const ::core::ffi::c_char;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Return hip error as text string form.

 @param [in] hipError Error code to convert to string.
 @param [out] errorString char pointer to the NULL-terminated error string
 @returns #hipSuccess, #hipErrorInvalidValue

 @see hipGetErrorName, hipGetLastError, hipPeakAtLastError, hipError_t*/
    pub fn hipDrvGetErrorName(
        hipError: hipError_t,
        errorString: *mut *const ::core::ffi::c_char,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Return handy text string message to explain the error which occurred

 @param [in] hipError Error code to convert to string.
 @param [out] errorString char pointer to the NULL-terminated error string
 @returns #hipSuccess, #hipErrorInvalidValue

 @see hipGetErrorName, hipGetLastError, hipPeakAtLastError, hipError_t*/
    pub fn hipDrvGetErrorString(
        hipError: hipError_t,
        errorString: *mut *const ::core::ffi::c_char,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Create an asynchronous stream.

 @param[in, out] stream Valid pointer to hipStream_t.  This function writes the memory with the
 newly created stream.
 @returns #hipSuccess, #hipErrorInvalidValue

 Create a new asynchronous stream.  @p stream returns an opaque handle that can be used to
 reference the newly created stream in subsequent hipStream* commands.  The stream is allocated on
 the heap and will remain allocated even if the handle goes out-of-scope.  To release the memory
 used by the stream, application must call hipStreamDestroy.

 @returns #hipSuccess, #hipErrorInvalidValue

 @see hipStreamCreateWithFlags, hipStreamCreateWithPriority, hipStreamSynchronize, hipStreamWaitEvent, hipStreamDestroy*/
    pub fn hipStreamCreate(stream: *mut hipStream_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Create an asynchronous stream.

 @param[in, out] stream Pointer to new stream
 @param[in ] flags to control stream creation.
 @returns #hipSuccess, #hipErrorInvalidValue

 Create a new asynchronous stream.  @p stream returns an opaque handle that can be used to
 reference the newly created stream in subsequent hipStream* commands.  The stream is allocated on
 the heap and will remain allocated even if the handle goes out-of-scope.  To release the memory
 used by the stream, application must call hipStreamDestroy. Flags controls behavior of the
 stream.  See #hipStreamDefault, #hipStreamNonBlocking.


 @see hipStreamCreate, hipStreamCreateWithPriority, hipStreamSynchronize, hipStreamWaitEvent, hipStreamDestroy*/
    pub fn hipStreamCreateWithFlags(
        stream: *mut hipStream_t,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Create an asynchronous stream with the specified priority.

 @param[in, out] stream Pointer to new stream
 @param[in ] flags to control stream creation.
 @param[in ] priority of the stream. Lower numbers represent higher priorities.
 @returns #hipSuccess, #hipErrorInvalidValue

 Create a new asynchronous stream with the specified priority.  @p stream returns an opaque handle
 that can be used to reference the newly created stream in subsequent hipStream* commands.  The
 stream is allocated on the heap and will remain allocated even if the handle goes out-of-scope.
 To release the memory used by the stream, application must call hipStreamDestroy. Flags controls
 behavior of the stream.  See #hipStreamDefault, #hipStreamNonBlocking.


 @see hipStreamCreate, hipStreamSynchronize, hipStreamWaitEvent, hipStreamDestroy*/
    pub fn hipStreamCreateWithPriority(
        stream: *mut hipStream_t,
        flags: ::core::ffi::c_uint,
        priority: ::core::ffi::c_int,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns numerical values that correspond to the least and greatest stream priority.

 @param[in, out] leastPriority pointer in which value corresponding to least priority is returned.
 @param[in, out] greatestPriority pointer in which value corresponding to greatest priority is returned.
 @returns #hipSuccess

 Returns in *leastPriority and *greatestPriority the numerical values that correspond to the least
 and greatest stream priority respectively. Stream priorities follow a convention where lower numbers
 imply greater priorities. The range of meaningful stream priorities is given by
 [*greatestPriority, *leastPriority]. If the user attempts to create a stream with a priority value
 that is outside the meaningful range as specified by this API, the priority is automatically
 clamped to within the valid range.*/
    pub fn hipDeviceGetStreamPriorityRange(
        leastPriority: *mut ::core::ffi::c_int,
        greatestPriority: *mut ::core::ffi::c_int,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Destroys the specified stream.

 @param[in] stream stream identifier.
 @returns #hipSuccess #hipErrorInvalidHandle

 Destroys the specified stream.

 If commands are still executing on the specified stream, some may complete execution before the
 queue is deleted.

 The queue may be destroyed while some commands are still inflight, or may wait for all commands
 queued to the stream before destroying it.

 @see hipStreamCreate, hipStreamCreateWithFlags, hipStreamCreateWithPriority, hipStreamQuery,
 hipStreamWaitEvent, hipStreamSynchronize*/
    pub fn hipStreamDestroy(stream: hipStream_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Return #hipSuccess if all of the operations in the specified @p stream have completed, or
 #hipErrorNotReady if not.

 @param[in] stream stream to query

 @returns #hipSuccess, #hipErrorNotReady, #hipErrorInvalidHandle

 This is thread-safe and returns a snapshot of the current state of the queue.  However, if other
 host threads are sending work to the stream, the status may change immediately after the function
 is called.  It is typically used for debug.

 @see hipStreamCreate, hipStreamCreateWithFlags, hipStreamCreateWithPriority, hipStreamWaitEvent,
 hipStreamSynchronize, hipStreamDestroy*/
    pub fn hipStreamQuery(stream: hipStream_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Wait for all commands in stream to complete.

 @param[in] stream stream identifier.

 @returns #hipSuccess, #hipErrorInvalidHandle

 This command is host-synchronous : the host will block until the specified stream is empty.

 This command follows standard null-stream semantics.  Specifically, specifying the null stream
 will cause the command to wait for other streams on the same device to complete all pending
 operations.

 This command honors the hipDeviceLaunchBlocking flag, which controls whether the wait is active
 or blocking.

 @see hipStreamCreate, hipStreamCreateWithFlags, hipStreamCreateWithPriority, hipStreamWaitEvent,
 hipStreamDestroy
*/
    pub fn hipStreamSynchronize(stream: hipStream_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Make the specified compute stream wait for an event

 @param[in] stream  Stream to make wait
 @param[in] event  Event to wait on
 @param[in] flags  Parameters to control the operation

 @returns #hipSuccess, #hipErrorInvalidHandle, #hipErrorInvalidValue,
 #hipErrorStreamCaptureIsolation

 This function inserts a wait operation into the specified stream.
 All future work submitted to @p stream will wait until @p event reports completion before
 beginning execution.

 Flags include:
   hipEventWaitDefault: Default event creation flag.
   hipEventWaitExternal: Wait is captured in the graph as an external event node when
                           performing stream capture

 This function only waits for commands in the current stream to complete.  Notably, this function
 does not implicitly wait for commands in the default stream to complete, even if the specified
 stream is created with hipStreamNonBlocking = 0.

 @see hipStreamCreate, hipStreamCreateWithFlags, hipStreamCreateWithPriority, hipStreamSynchronize, hipStreamDestroy*/
    pub fn hipStreamWaitEvent(
        stream: hipStream_t,
        event: hipEvent_t,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Return flags associated with this stream.

 @param[in] stream stream to be queried
 @param[in,out] flags Pointer to an unsigned integer in which the stream's flags are returned
 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidHandle

 @returns #hipSuccess #hipErrorInvalidValue #hipErrorInvalidHandle

 Return flags associated with this stream in *@p flags.

 @see hipStreamCreateWithFlags*/
    pub fn hipStreamGetFlags(
        stream: hipStream_t,
        flags: *mut ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Query the priority of a stream.

 @param[in] stream stream to be queried
 @param[in,out] priority Pointer to an unsigned integer in which the stream's priority is returned
 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidHandle

 @returns #hipSuccess #hipErrorInvalidValue #hipErrorInvalidHandle

 Query the priority of a stream. The priority is returned in in priority.

 @see hipStreamCreateWithFlags*/
    pub fn hipStreamGetPriority(
        stream: hipStream_t,
        priority: *mut ::core::ffi::c_int,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get the device assocaited with the stream

 @param[in] stream stream to be queried
 @param[out] device device associated with the stream
 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorContextIsDestroyed, #hipErrorInvalidHandle,
 #hipErrorNotInitialized, #hipErrorDeinitialized, #hipErrorInvalidContext

 @see hipStreamCreate, hipStreamDestroy, hipDeviceGetStreamPriorityRange*/
    pub fn hipStreamGetDevice(
        stream: hipStream_t,
        device: *mut hipDevice_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Create an asynchronous stream with the specified CU mask.

 @param[in, out] stream Pointer to new stream
 @param[in ] cuMaskSize Size of CU mask bit array passed in.
 @param[in ] cuMask Bit-vector representing the CU mask. Each active bit represents using one CU.
 The first 32 bits represent the first 32 CUs, and so on. If its size is greater than physical
 CU number (i.e., multiProcessorCount member of hipDeviceProp_t), the extra elements are ignored.
 It is user's responsibility to make sure the input is meaningful.
 @returns #hipSuccess, #hipErrorInvalidHandle, #hipErrorInvalidValue

 Create a new asynchronous stream with the specified CU mask.  @p stream returns an opaque handle
 that can be used to reference the newly created stream in subsequent hipStream* commands.  The
 stream is allocated on the heap and will remain allocated even if the handle goes out-of-scope.
 To release the memory used by the stream, application must call hipStreamDestroy.


 @see hipStreamCreate, hipStreamSynchronize, hipStreamWaitEvent, hipStreamDestroy*/
    pub fn hipExtStreamCreateWithCUMask(
        stream: *mut hipStream_t,
        cuMaskSize: u32,
        cuMask: *const u32,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get CU mask associated with an asynchronous stream

 @param[in] stream stream to be queried
 @param[in] cuMaskSize number of the block of memories (uint32_t *) allocated by user
 @param[out] cuMask Pointer to a pre-allocated block of memories (uint32_t *) in which
 the stream's CU mask is returned. The CU mask is returned in a chunck of 32 bits where
 each active bit represents one active CU
 @returns #hipSuccess, #hipErrorInvalidHandle, #hipErrorInvalidValue

 @see hipStreamCreate, hipStreamSynchronize, hipStreamWaitEvent, hipStreamDestroy*/
    pub fn hipExtStreamGetCUMask(
        stream: hipStream_t,
        cuMaskSize: u32,
        cuMask: *mut u32,
    ) -> hipError_t;
}
/// Stream CallBack struct
pub type hipStreamCallback_t = ::core::option::Option<
    unsafe extern "C" fn(
        stream: hipStream_t,
        status: hipError_t,
        userData: *mut ::core::ffi::c_void,
    ),
>;
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Adds a callback to be called on the host after all currently enqueued
 items in the stream have completed.  For each
 hipStreamAddCallback call, a callback will be executed exactly once.
 The callback will block later work in the stream until it is finished.
 @param[in] stream   - Stream to add callback to
 @param[in] callback - The function to call once preceding stream operations are complete
 @param[in] userData - User specified data to be passed to the callback function
 @param[in] flags    - Reserved for future use, must be 0
 @returns #hipSuccess, #hipErrorInvalidHandle, #hipErrorNotSupported

 @see hipStreamCreate, hipStreamCreateWithFlags, hipStreamQuery, hipStreamSynchronize,
 hipStreamWaitEvent, hipStreamDestroy, hipStreamCreateWithPriority
*/
    pub fn hipStreamAddCallback(
        stream: hipStream_t,
        callback: hipStreamCallback_t,
        userData: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Enqueues a wait command to the stream.[BETA]

 @param [in] stream - Stream identifier
 @param [in] ptr    - Pointer to memory object allocated using #hipMallocSignalMemory flag
 @param [in] value  - Value to be used in compare operation
 @param [in] flags  - Defines the compare operation, supported values are #hipStreamWaitValueGte
 #hipStreamWaitValueEq, #hipStreamWaitValueAnd and #hipStreamWaitValueNor
 @param [in] mask   - Mask to be applied on value at memory before it is compared with value,
 default value is set to enable every bit

 @returns #hipSuccess, #hipErrorInvalidValue

 Enqueues a wait command to the stream, all operations enqueued  on this stream after this, will
 not execute until the defined wait condition is true.

 #hipStreamWaitValueGte: waits until *ptr&mask >= value

 #hipStreamWaitValueEq : waits until *ptr&mask == value

 #hipStreamWaitValueAnd: waits until ((*ptr&mask) & value) != 0

 #hipStreamWaitValueNor: waits until ~((*ptr&mask) | (value&mask)) != 0

 @note when using #hipStreamWaitValueNor, mask is applied on both 'value' and '*ptr'.

 @note Support for #hipStreamWaitValue32 can be queried using 'hipDeviceGetAttribute()' and
 'hipDeviceAttributeCanUseStreamWaitValue' flag.

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @see hipExtMallocWithFlags, hipFree, hipStreamWaitValue64, hipStreamWriteValue64,
 hipStreamWriteValue32, hipDeviceGetAttribute*/
    pub fn hipStreamWaitValue32(
        stream: hipStream_t,
        ptr: *mut ::core::ffi::c_void,
        value: u32,
        flags: ::core::ffi::c_uint,
        mask: u32,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Enqueues a wait command to the stream.[BETA]

 @param [in] stream - Stream identifier
 @param [in] ptr    - Pointer to memory object allocated using 'hipMallocSignalMemory' flag
 @param [in] value  - Value to be used in compare operation
 @param [in] flags  - Defines the compare operation, supported values are #hipStreamWaitValueGte
 #hipStreamWaitValueEq, #hipStreamWaitValueAnd and #hipStreamWaitValueNor.
 @param [in] mask   - Mask to be applied on value at memory before it is compared with value
 default value is set to enable every bit

 @returns #hipSuccess, #hipErrorInvalidValue

 Enqueues a wait command to the stream, all operations enqueued  on this stream after this, will
 not execute until the defined wait condition is true.

 #hipStreamWaitValueGte: waits until *ptr&mask >= value

 #hipStreamWaitValueEq : waits until *ptr&mask == value

 #hipStreamWaitValueAnd: waits until ((*ptr&mask) & value) != 0

 #hipStreamWaitValueNor: waits until ~((*ptr&mask) | (value&mask)) != 0

 @note when using #hipStreamWaitValueNor, mask is applied on both 'value' and '*ptr'.

 @note Support for hipStreamWaitValue64 can be queried using 'hipDeviceGetAttribute()' and
 'hipDeviceAttributeCanUseStreamWaitValue' flag.

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @see hipExtMallocWithFlags, hipFree, hipStreamWaitValue32, hipStreamWriteValue64,
 hipStreamWriteValue32, hipDeviceGetAttribute*/
    pub fn hipStreamWaitValue64(
        stream: hipStream_t,
        ptr: *mut ::core::ffi::c_void,
        value: u64,
        flags: ::core::ffi::c_uint,
        mask: u64,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Enqueues a write command to the stream.[BETA]

 @param [in] stream - Stream identifier
 @param [in] ptr    - Pointer to a GPU accessible memory object
 @param [in] value  - Value to be written
 @param [in] flags  - reserved, ignored for now, will be used in future releases

 @returns #hipSuccess, #hipErrorInvalidValue

 Enqueues a write command to the stream, write operation is performed after all earlier commands
 on this stream have completed the execution.

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @see hipExtMallocWithFlags, hipFree, hipStreamWriteValue32, hipStreamWaitValue32,
 hipStreamWaitValue64*/
    pub fn hipStreamWriteValue32(
        stream: hipStream_t,
        ptr: *mut ::core::ffi::c_void,
        value: u32,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Enqueues a write command to the stream.[BETA]

 @param [in] stream - Stream identifier
 @param [in] ptr    - Pointer to a GPU accessible memory object
 @param [in] value  - Value to be written
 @param [in] flags  - reserved, ignored for now, will be used in future releases

 @returns #hipSuccess, #hipErrorInvalidValue

 Enqueues a write command to the stream, write operation is performed after all earlier commands
 on this stream have completed the execution.

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @see hipExtMallocWithFlags, hipFree, hipStreamWriteValue32, hipStreamWaitValue32,
 hipStreamWaitValue64*/
    pub fn hipStreamWriteValue64(
        stream: hipStream_t,
        ptr: *mut ::core::ffi::c_void,
        value: u64,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Enqueues an array of stream memory operations in the stream.[BETA]

 @param [in] stream      - Stream identifier
 @param [in] count       - The number of operations in the array. Must be less than 256
 @param [in] paramArray  - The types and parameters of the individual operations.
 @param [in] flags       - Reserved for future expansion; must be 0.

 @returns #hipSuccess, #hipErrorInvalidValue

 Batch operations to synchronize the stream via memory operations.

 @warning This API is marked as beta, meaning, while this is feature complete,
 it is still open to changes and may have outstanding issues.

 @see hipStreamWriteValue32, hipStreamWaitValue32,
 hipStreamWaitValue64. hipStreamWriteValue64*/
    pub fn hipStreamBatchMemOp(
        stream: hipStream_t,
        count: ::core::ffi::c_uint,
        paramArray: *mut hipStreamBatchMemOpParams,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a batch memory operation node and adds it to a graph.[BETA]

 @param [in] phGraphNode      - Returns the newly created node
 @param [in] hGraph           - Graph to which to add the node
 @param [in] dependencies     -  Dependencies of the node
 @param [in] numDependencies  - Number of dependencies
 @param [in] nodeParams       - Parameters for the node

 @returns #hipSuccess, #hipErrorInvalidValue

 @warning This API is marked as beta, meaning, while this is feature complete,
 it is still open to changes and may have outstanding issues.

 @see hipStreamWriteValue32, hipStreamWaitValue32,
 hipStreamWaitValue64. hipStreamWriteValue64, hipStreamBatchMemOp*/
    pub fn hipGraphAddBatchMemOpNode(
        phGraphNode: *mut hipGraphNode_t,
        hGraph: hipGraph_t,
        dependencies: *const hipGraphNode_t,
        numDependencies: usize,
        nodeParams: *const hipBatchMemOpNodeParams,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns a batch mem op node's parameters.[BETA]

 @param [in] hNode           - Node to get the parameters for
 @param [in] nodeParams_out  - Pointer to return the parameters

 @returns #hipSuccess, #hipErrorInvalidValue

 Returns the parameters of batch mem op node hNode in nodeParams_out.
 The paramArray returned in nodeParams_out is owned by the node.
 This memory remains valid until the node is destroyed or its parameters are modified,
 and should not be modified directly.

 @warning This API is marked as beta, meaning, while this is feature complete,
 it is still open to changes and may have outstanding issues.

 @see hipStreamWriteValue32, hipStreamWaitValue32,
 hipStreamWaitValue64. hipStreamWriteValue64. hipGraphBatchMemOpNodeSetParams*/
    pub fn hipGraphBatchMemOpNodeGetParams(
        hNode: hipGraphNode_t,
        nodeParams_out: *mut hipBatchMemOpNodeParams,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets the batch mem op node's parameters.[BETA]

 @param [in] hNode       - Node to set the parameters for
 @param [in] nodeParams  - Parameters to copy

 @returns #hipSuccess, #hipErrorInvalidValue

 Sets the parameters of batch mem op node hNode to nodeParams.

 @warning This API is marked as beta, meaning, while this is feature complete,
 it is still open to changes and may have outstanding issues.

 @see hipStreamWriteValue32, hipStreamWaitValue32,
 hipStreamWaitValue64. hipStreamWriteValue64, hipGraphBatchMemOpNodeGetParams*/
    pub fn hipGraphBatchMemOpNodeSetParams(
        hNode: hipGraphNode_t,
        nodeParams: *mut hipBatchMemOpNodeParams,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets the parameters for a batch mem op node in the given graphExec.[BETA]

 @param [in] hGraphExec  - The executable graph in which to set the specified node
 @param [in] hNode       - Batch mem op node from the graph from which graphExec was instantiated
 @param [in] nodeParams  - Updated Parameters to set

 @returns #hipSuccess, #hipErrorInvalidValue

 Sets the parameters of a batch mem op node in an executable graph hGraphExec.
 The node is identified by the corresponding node hNode in the non-executable graph,
 from which the executable graph was instantiated.

 @warning This API is marked as beta, meaning, while this is feature complete,
 it is still open to changes and may have outstanding issues.

 @see hipStreamWriteValue32, hipStreamWaitValue32,
 hipStreamWaitValue64. hipStreamWriteValue64, hipStreamBatchMemOp*/
    pub fn hipGraphExecBatchMemOpNodeSetParams(
        hGraphExec: hipGraphExec_t,
        hNode: hipGraphNode_t,
        nodeParams: *const hipBatchMemOpNodeParams,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " @}\n/\n/**\n-------------------------------------------------------------------------------------------------\n-------------------------------------------------------------------------------------------------\n  @defgroup Event Event Management\n  @{\n  This section describes the event management functions of HIP runtime API.\n/\n/**\n @brief Create an event with the specified flags\n\n @param[in,out] event Returns the newly created event.\n @param[in] flags     Flags to control event behavior.  Valid values are #hipEventDefault,\n#hipEventBlockingSync, #hipEventDisableTiming, #hipEventInterprocess\n #hipEventDefault : Default flag.  The event will use active synchronization and will support\ntiming.  Blocking synchronization provides lowest possible latency at the expense of dedicating a\nCPU to poll on the event.\n #hipEventBlockingSync : The event will use blocking synchronization : if hipEventSynchronize is\ncalled on this event, the thread will block until the event completes.  This can increase latency\nfor the synchroniation but can result in lower power and more resources for other CPU threads.\n #hipEventDisableTiming : Disable recording of timing information. Events created with this flag\nwould not record profiling data and provide best performance if used for synchronization.\n #hipEventInterprocess : The event can be used as an interprocess event. hipEventDisableTiming\nflag also must be set when hipEventInterprocess flag is set.\n #hipEventDisableSystemFence : Disable acquire and release system scope fence. This may\nimprove performance but device memory may not be visible to the host and other devices\nif this flag is set.\n\n @returns #hipSuccess, #hipErrorNotInitialized, #hipErrorInvalidValue,\n#hipErrorLaunchFailure, #hipErrorOutOfMemory\n\n @see hipEventCreate, hipEventSynchronize, hipEventDestroy, hipEventElapsedTime"]
    pub fn hipEventCreateWithFlags(
        event: *mut hipEvent_t,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  Create an event

 @param[in,out] event Returns the newly created event.

 @returns #hipSuccess, #hipErrorNotInitialized, #hipErrorInvalidValue,
 #hipErrorLaunchFailure, #hipErrorOutOfMemory

 @see hipEventCreateWithFlags, hipEventRecord, hipEventQuery, hipEventSynchronize,
 hipEventDestroy, hipEventElapsedTime*/
    pub fn hipEventCreate(event: *mut hipEvent_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Record an event in the specified stream.

 @param[in] event event to record.
 @param[in] stream stream in which to record event.
 @param[in] flags parameter for operations
 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotInitialized,
 #hipErrorInvalidHandle, #hipErrorLaunchFailure

 hipEventQuery() or hipEventSynchronize() must be used to determine when the event
 transitions from "recording" (after hipEventRecord() is called) to "recorded"
 (when timestamps are set, if requested).

 Events which are recorded in a non-NULL stream will transition to
 from recording to "recorded" state when they reach the head of
 the specified stream, after all previous
 commands in that stream have completed executing.

 Flags include:
   hipEventRecordDefault: Default event creation flag.
   hipEventRecordExternal: Event is captured in the graph as an external event node when
                           performing stream capture

 If hipEventRecord() has been previously called on this event, then this call will overwrite any
 existing state in event.

 If this function is called on an event that is currently being recorded, results are undefined
 - either outstanding recording may save state into the event, and the order is not guaranteed.

 @note: If this function is not called before use hipEventQuery() or hipEventSynchronize(),
 #hipSuccess is returned, meaning no pending event in the stream.

 @see hipEventCreate, hipEventCreateWithFlags, hipEventQuery, hipEventSynchronize,
 hipEventDestroy, hipEventElapsedTime
*/
    pub fn hipEventRecordWithFlags(
        event: hipEvent_t,
        stream: hipStream_t,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipEventRecord(event: hipEvent_t, stream: hipStream_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Destroy the specified event.

  @param[in] event Event to destroy.
  @returns #hipSuccess, #hipErrorNotInitialized, #hipErrorInvalidValue,
 #hipErrorLaunchFailure

  Releases memory associated with the event.  If the event is recording but has not completed
 recording when hipEventDestroy() is called, the function will return immediately and the
 completion_future resources will be released later, when the hipDevice is synchronized.

 @see hipEventCreate, hipEventCreateWithFlags, hipEventQuery, hipEventSynchronize, hipEventRecord,
 hipEventElapsedTime

 @returns #hipSuccess*/
    pub fn hipEventDestroy(event: hipEvent_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Wait for an event to complete.

  This function will block until the event is ready, waiting for all previous work in the stream
 specified when event was recorded with hipEventRecord().

  If hipEventRecord() has not been called on @p event, this function returns #hipSuccess when no
  event is captured.


  @param[in] event Event on which to wait.

  @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotInitialized,
 #hipErrorInvalidHandle, #hipErrorLaunchFailure

  @see hipEventCreate, hipEventCreateWithFlags, hipEventQuery, hipEventDestroy, hipEventRecord,
 hipEventElapsedTime*/
    pub fn hipEventSynchronize(event: hipEvent_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Return the elapsed time between two events.

 @param[out] ms : Return time between start and stop in ms.
 @param[in]   start : Start event.
 @param[in]   stop  : Stop event.
 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotReady, #hipErrorInvalidHandle,
 #hipErrorNotInitialized, #hipErrorLaunchFailure

 Computes the elapsed time between two events. Time is computed in ms, with
 a resolution of approximately 1 us.

 Events which are recorded in a NULL stream will block until all commands
 on all other streams complete execution, and then record the timestamp.

 Events which are recorded in a non-NULL stream will record their timestamp
 when they reach the head of the specified stream, after all previous
 commands in that stream have completed executing.  Thus the time that
 the event recorded may be significantly after the host calls hipEventRecord().

 If hipEventRecord() has not been called on either event, then #hipErrorInvalidHandle is
 returned. If hipEventRecord() has been called on both events, but the timestamp has not yet been
 recorded on one or both events (that is, hipEventQuery() would return #hipErrorNotReady on at
 least one of the events), then #hipErrorNotReady is returned.

 @see hipEventCreate, hipEventCreateWithFlags, hipEventQuery, hipEventDestroy, hipEventRecord,
 hipEventSynchronize*/
    pub fn hipEventElapsedTime(
        ms: *mut f32,
        start: hipEvent_t,
        stop: hipEvent_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Query event status

 @param[in] event Event to query.
 @returns #hipSuccess, #hipErrorNotReady, #hipErrorInvalidHandle, #hipErrorInvalidValue,
 #hipErrorNotInitialized, #hipErrorLaunchFailure

 Query the status of the specified event.  This function will return #hipSuccess if all
 commands in the appropriate stream (specified to hipEventRecord()) have completed.  If any execution
 has not completed, then #hipErrorNotReady is returned.

 @note This API returns #hipSuccess, if hipEventRecord() is not called before this API.

 @see hipEventCreate, hipEventCreateWithFlags, hipEventRecord, hipEventDestroy,
 hipEventSynchronize, hipEventElapsedTime*/
    pub fn hipEventQuery(event: hipEvent_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Sets information on the specified pointer.[BETA]

  @param [in]      value     Sets pointer attribute value
  @param [in]      attribute  Attribute to set
  @param [in]      ptr      Pointer to set attributes for

  @returns #hipSuccess, #hipErrorInvalidDevice, #hipErrorInvalidValue

  @warning This API is marked as Beta. While this feature is complete, it can
           change and might have outstanding issues.
*/
    pub fn hipPointerSetAttribute(
        value: *const ::core::ffi::c_void,
        attribute: hipPointer_attribute,
        ptr: hipDeviceptr_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Returns attributes for the specified pointer

  @param [out]  attributes  attributes for the specified pointer
  @param [in]   ptr         pointer to get attributes for

  The output parameter 'attributes' has a member named 'type' that describes what memory the
  pointer is associated with, such as device memory, host memory, managed memory, and others.
  Otherwise, the API cannot handle the pointer and returns #hipErrorInvalidValue.

  @note  The unrecognized memory type is unsupported to keep the HIP functionality backward
  compatibility due to #hipMemoryType enum values.

  @returns #hipSuccess, #hipErrorInvalidDevice, #hipErrorInvalidValue

  @note  The current behavior of this HIP API corresponds to the CUDA API before version 11.0.

  @see hipPointerGetAttribute*/
    pub fn hipPointerGetAttributes(
        attributes: *mut hipPointerAttribute_t,
        ptr: *const ::core::ffi::c_void,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Returns information about the specified pointer.[BETA]

  @param [in, out] data     Returned pointer attribute value
  @param [in]      attribute  Attribute to query for
  @param [in]      ptr      Pointer to get attributes for

  @returns #hipSuccess, #hipErrorInvalidDevice, #hipErrorInvalidValue

  @warning This API is marked as Beta. While this feature is complete, it can
           change and might have outstanding issues.

  @see hipPointerGetAttributes*/
    pub fn hipPointerGetAttribute(
        data: *mut ::core::ffi::c_void,
        attribute: hipPointer_attribute,
        ptr: hipDeviceptr_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Returns information about the specified pointer.[BETA]

  @param [in]  numAttributes   number of attributes to query for
  @param [in]  attributes      attributes to query for
  @param [in, out] data        a two-dimensional containing pointers to memory locations
                               where the result of each attribute query will be written to
  @param [in]  ptr             pointer to get attributes for

  @returns #hipSuccess, #hipErrorInvalidDevice, #hipErrorInvalidValue

  @warning This API is marked as Beta. While this feature is complete, it can
           change and might have outstanding issues.

  @see hipPointerGetAttribute*/
    pub fn hipDrvPointerGetAttributes(
        numAttributes: ::core::ffi::c_uint,
        attributes: *mut hipPointer_attribute,
        data: *mut *mut ::core::ffi::c_void,
        ptr: hipDeviceptr_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = "-------------------------------------------------------------------------------------------------\n-------------------------------------------------------------------------------------------------\n  @defgroup External External Resource Interoperability\n  @{\n  @ingroup API\n\n  This section describes the external resource interoperability functions of HIP runtime API.\n\n/\n/**\n  @brief Imports an external semaphore.\n\n  @param[out] extSem_out  External semaphores to be waited on\n  @param[in] semHandleDesc Semaphore import handle descriptor\n\n  @returns #hipSuccess, #hipErrorInvalidDevice, #hipErrorInvalidValue\n\n  @see\n\n  @note  This API is currently not supported on Linux.\n"]
    pub fn hipImportExternalSemaphore(
        extSem_out: *mut hipExternalSemaphore_t,
        semHandleDesc: *const hipExternalSemaphoreHandleDesc,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Signals a set of external semaphore objects.

  @param[in] extSemArray  External semaphores to be waited on
  @param[in] paramsArray Array of semaphore parameters
  @param[in] numExtSems Number of semaphores to wait on
  @param[in] stream Stream to enqueue the wait operations in

  @returns #hipSuccess, #hipErrorInvalidDevice, #hipErrorInvalidValue

  @see

  @note  This API is currently not supported on Linux.
*/
    pub fn hipSignalExternalSemaphoresAsync(
        extSemArray: *const hipExternalSemaphore_t,
        paramsArray: *const hipExternalSemaphoreSignalParams,
        numExtSems: ::core::ffi::c_uint,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Waits on a set of external semaphore objects

  @param[in] extSemArray  External semaphores to be waited on
  @param[in] paramsArray Array of semaphore parameters
  @param[in] numExtSems Number of semaphores to wait on
  @param[in] stream Stream to enqueue the wait operations in

  @returns #hipSuccess, #hipErrorInvalidDevice, #hipErrorInvalidValue

  @see

  @note  This API is currently not supported on Linux.
*/
    pub fn hipWaitExternalSemaphoresAsync(
        extSemArray: *const hipExternalSemaphore_t,
        paramsArray: *const hipExternalSemaphoreWaitParams,
        numExtSems: ::core::ffi::c_uint,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Destroys an external semaphore object and releases any references to the underlying resource. Any outstanding signals or waits must have completed before the semaphore is destroyed.

  @param[in] extSem handle to an external memory object

  @returns #hipSuccess, #hipErrorInvalidDevice, #hipErrorInvalidValue

  @see

  @note  This API is currently not supported on Linux.
*/
    pub fn hipDestroyExternalSemaphore(extSem: hipExternalSemaphore_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Imports an external memory object.

  @param[out] extMem_out  Returned handle to an external memory object
  @param[in]  memHandleDesc Memory import handle descriptor

  @returns #hipSuccess, #hipErrorInvalidDevice, #hipErrorInvalidValue

  @see*/
    pub fn hipImportExternalMemory(
        extMem_out: *mut hipExternalMemory_t,
        memHandleDesc: *const hipExternalMemoryHandleDesc,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Maps a buffer onto an imported memory object.

  @param[out] devPtr Returned device pointer to buffer
  @param[in]  extMem  Handle to external memory object
  @param[in]  bufferDesc  Buffer descriptor

  @returns #hipSuccess, #hipErrorInvalidDevice, #hipErrorInvalidValue

  @see*/
    pub fn hipExternalMemoryGetMappedBuffer(
        devPtr: *mut *mut ::core::ffi::c_void,
        extMem: hipExternalMemory_t,
        bufferDesc: *const hipExternalMemoryBufferDesc,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Destroys an external memory object.

  @param[in] extMem  External memory object to be destroyed

  @returns #hipSuccess, #hipErrorInvalidDevice, #hipErrorInvalidValue

  @see*/
    pub fn hipDestroyExternalMemory(extMem: hipExternalMemory_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Maps a mipmapped array onto an external memory object.

  @param[out] mipmap mipmapped array to return
  @param[in]  extMem external memory object handle
  @param[in]  mipmapDesc external mipmapped array descriptor

  Returned mipmapped array must be freed using hipFreeMipmappedArray.

  @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidResourceHandle

  @see hipImportExternalMemory, hipDestroyExternalMemory, hipExternalMemoryGetMappedBuffer, hipFreeMipmappedArray*/
    pub fn hipExternalMemoryGetMappedMipmappedArray(
        mipmap: *mut hipMipmappedArray_t,
        extMem: hipExternalMemory_t,
        mipmapDesc: *const hipExternalMemoryMipmappedArrayDesc,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " @}\n/\n/**\n  @brief Allocate memory on the default accelerator\n\n  @param[out] ptr Pointer to the allocated memory\n  @param[in]  size Requested memory size\n\n  If size is 0, no memory is allocated, *ptr returns nullptr, and hipSuccess is returned.\n\n  @returns #hipSuccess, #hipErrorOutOfMemory, #hipErrorInvalidValue (bad context, null *ptr)\n\n  @see hipMallocPitch, hipFree, hipMallocArray, hipFreeArray, hipMalloc3D, hipMalloc3DArray,\n hipHostFree, hipHostMalloc"]
    pub fn hipMalloc(ptr: *mut *mut ::core::ffi::c_void, size: usize) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Allocate memory on the default accelerator

  @param[out] ptr  Pointer to the allocated memory
  @param[in]  sizeBytes  Requested memory size
  @param[in]  flags  Type of memory allocation

  If requested memory size is 0, no memory is allocated, *ptr returns nullptr, and #hipSuccess
  is returned.

  The memory allocation flag should be either #hipDeviceMallocDefault,
  #hipDeviceMallocFinegrained, #hipDeviceMallocUncached, or #hipMallocSignalMemory.
  If the flag is any other value, the API returns #hipErrorInvalidValue.

  @returns #hipSuccess, #hipErrorOutOfMemory, #hipErrorInvalidValue (bad context, null *ptr)

  @see hipMallocPitch, hipFree, hipMallocArray, hipFreeArray, hipMalloc3D, hipMalloc3DArray,
 hipHostFree, hiHostMalloc*/
    pub fn hipExtMallocWithFlags(
        ptr: *mut *mut ::core::ffi::c_void,
        sizeBytes: usize,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Allocate pinned host memory [Deprecated]

  @param[out] ptr Pointer to the allocated host pinned memory
  @param[in]  size Requested memory size

  If size is 0, no memory is allocated, *ptr returns nullptr, and hipSuccess is returned.

  @returns #hipSuccess, #hipErrorOutOfMemory

  @warning  This API is deprecated, use hipHostMalloc() instead*/
    pub fn hipMallocHost(ptr: *mut *mut ::core::ffi::c_void, size: usize) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Allocate pinned host memory [Deprecated]

  @param[out] ptr Pointer to the allocated host pinned memory
  @param[in]  size Requested memory size

  If size is 0, no memory is allocated, *ptr returns nullptr, and hipSuccess is returned.

  @returns #hipSuccess, #hipErrorOutOfMemory

  @warning  This API is deprecated, use hipHostMalloc() instead*/
    pub fn hipMemAllocHost(
        ptr: *mut *mut ::core::ffi::c_void,
        size: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Allocates device accessible page locked (pinned) host memory

  This API allocates pinned host memory which is mapped into the address space of all GPUs
  in the system, the memory can be accessed directly by the GPU device, and can be read or
  written with much higher bandwidth than pageable memory obtained with functions such as
  malloc().

  Using the pinned host memory, applications can implement faster data transfers for HostToDevice
  and DeviceToHost. The runtime tracks the hipHostMalloc allocations and can avoid some of the
  setup required for regular unpinned memory.

  When the memory accesses are infrequent, zero-copy memory can be a good choice, for coherent
  allocation. GPU can directly access the host memory over the CPU/GPU interconnect, without need
  to copy the data.

  Currently the allocation granularity is 4KB for the API.

  Developers need to choose proper allocation flag with consideration of synchronization.

  @param[out] ptr Pointer to the allocated host pinned memory
  @param[in]  size Requested memory size in bytes
  If size is 0, no memory is allocated, *ptr returns nullptr, and hipSuccess is returned.
  @param[in]  flags Type of host memory allocation. See the description of flags in
  hipSetDeviceFlags.

  If no input for flags, it will be the default pinned memory allocation on the host.

  @returns #hipSuccess, #hipErrorOutOfMemory


  @see hipSetDeviceFlags, hiptHostFree*/
    pub fn hipHostMalloc(
        ptr: *mut *mut ::core::ffi::c_void,
        size: usize,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = "-------------------------------------------------------------------------------------------------\n-------------------------------------------------------------------------------------------------\n  @defgroup MemoryM Managed Memory\n\n  @ingroup Memory\n @{\n  This section describes the managed memory management functions of HIP runtime API.\n\n  @note  The managed memory management APIs are implemented on Linux, under developement\n  on Windows.\n\n/\n/**\n @brief Allocates memory that will be automatically managed by HIP.\n\n This API is used for managed memory, allows data be shared and accessible to both CPU and\n GPU using a single pointer.\n\n The API returns the allocation pointer, managed by HMM, can be used further to execute kernels\n on device and fetch data between the host and device as needed.\n\n @note   It is recommend to do the capability check before call this API.\n\n @param [out] dev_ptr - pointer to allocated device memory\n @param [in]  size    - requested allocation size in bytes, it should be granularity of 4KB\n @param [in]  flags   - must be either hipMemAttachGlobal or hipMemAttachHost\n                        (defaults to hipMemAttachGlobal)\n\n @returns #hipSuccess, #hipErrorMemoryAllocation, #hipErrorNotSupported, #hipErrorInvalidValue\n"]
    pub fn hipMallocManaged(
        dev_ptr: *mut *mut ::core::ffi::c_void,
        size: usize,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Prefetches memory to the specified destination device using HIP.

 @param [in] dev_ptr  pointer to be prefetched
 @param [in] count    size in bytes for prefetching
 @param [in] device   destination device to prefetch to
 @param [in] stream   stream to enqueue prefetch operation

 @returns #hipSuccess, #hipErrorInvalidValue

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipMemPrefetchAsync(
        dev_ptr: *const ::core::ffi::c_void,
        count: usize,
        device: ::core::ffi::c_int,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Advise about the usage of a given memory range to HIP.

 @param [in] dev_ptr  pointer to memory to set the advice for
 @param [in] count    size in bytes of the memory range, it should be CPU page size alligned.
 @param [in] advice   advice to be applied for the specified memory range
 @param [in] device   device to apply the advice for

 @returns #hipSuccess, #hipErrorInvalidValue

 This HIP API advises about the usage to be applied on unified memory allocation in the
 range starting from the pointer address devPtr, with the size of count bytes.
 The memory range must refer to managed memory allocated via the API hipMallocManaged, and the
 range will be handled with proper round down and round up respectively in the driver to
 be aligned to CPU page size, the same way as corresponding CUDA API behaves in CUDA version 8.0
 and afterwards.

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipMemAdvise(
        dev_ptr: *const ::core::ffi::c_void,
        count: usize,
        advice: hipMemoryAdvise,
        device: ::core::ffi::c_int,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Query an attribute of a given memory range in HIP.

 @param [in,out] data   a pointer to a memory location where the result of each
                        attribute query will be written to
 @param [in] data_size  the size of data
 @param [in] attribute  the attribute to query
 @param [in] dev_ptr    start of the range to query
 @param [in] count      size of the range to query

 @returns #hipSuccess, #hipErrorInvalidValue

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipMemRangeGetAttribute(
        data: *mut ::core::ffi::c_void,
        data_size: usize,
        attribute: hipMemRangeAttribute,
        dev_ptr: *const ::core::ffi::c_void,
        count: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Query attributes of a given memory range in HIP.

 @param [in,out] data     a two-dimensional array containing pointers to memory locations
                          where the result of each attribute query will be written to
 @param [in] data_sizes   an array, containing the sizes of each result
 @param [in] attributes   the attribute to query
 @param [in] num_attributes  an array of attributes to query (numAttributes and the number
                          of attributes in this array should match)
 @param [in] dev_ptr      start of the range to query
 @param [in] count        size of the range to query

 @returns #hipSuccess, #hipErrorInvalidValue

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipMemRangeGetAttributes(
        data: *mut *mut ::core::ffi::c_void,
        data_sizes: *mut usize,
        attributes: *mut hipMemRangeAttribute,
        num_attributes: usize,
        dev_ptr: *const ::core::ffi::c_void,
        count: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Attach memory to a stream asynchronously in HIP.

 @param [in] stream     - stream in which to enqueue the attach operation
 @param [in] dev_ptr    - pointer to memory (must be a pointer to managed memory or
                          to a valid host-accessible region of system-allocated memory)
 @param [in] length     - length of memory (defaults to zero)
 @param [in] flags      - must be one of hipMemAttachGlobal, hipMemAttachHost or
                          hipMemAttachSingle (defaults to hipMemAttachSingle)

 @returns #hipSuccess, #hipErrorInvalidValue

 @warning This API is under development. Currently it is a no-operation (NOP)
          function on AMD GPUs and returns #hipSuccess.*/
    pub fn hipStreamAttachMemAsync(
        stream: hipStream_t,
        dev_ptr: *mut ::core::ffi::c_void,
        length: usize,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Allocates memory with stream ordered semantics

 Inserts a memory allocation operation into @p stream.
 A pointer to the allocated memory is returned immediately in *dptr.
 The allocation must not be accessed until the allocation operation completes.
 The allocation comes from the memory pool associated with the stream's device.

 @note The default memory pool of a device contains device memory from that device.
 @note Basic stream ordering allows future work submitted into the same stream to use the
  allocation. Stream query, stream synchronize, and HIP events can be used to guarantee that
  the allocation operation completes before work submitted in a separate stream runs.
 @note During stream capture, this function results in the creation of an allocation node.
  In this case, the allocation is owned by the graph instead of the memory pool. The memory
  pool's properties are used to set the node's creation parameters.

 @param [out] dev_ptr  Returned device pointer of memory allocation
 @param [in] size      Number of bytes to allocate
 @param [in] stream    The stream establishing the stream ordering contract and
                       the memory pool to allocate from

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported, #hipErrorOutOfMemory

 @see hipMallocFromPoolAsync, hipFreeAsync, hipMemPoolTrimTo, hipMemPoolGetAttribute,
 hipDeviceSetMemPool, hipMemPoolSetAttribute, hipMemPoolSetAccess, hipMemPoolGetAccess

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipMallocAsync(
        dev_ptr: *mut *mut ::core::ffi::c_void,
        size: usize,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Frees memory with stream ordered semantics

 Inserts a free operation into @p stream.
 The allocation must not be used after stream execution reaches the free.
 After this API returns, accessing the memory from any subsequent work launched on the GPU
 or querying its pointer attributes results in undefined behavior.

 @note During stream capture, this function results in the creation of a free node and
 must therefore be passed the address of a graph allocation.

 @param [in] dev_ptr Pointer to device memory to free
 @param [in] stream  The stream, where the destruciton will occur according to the execution order

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported

 @see hipMallocFromPoolAsync, hipMallocAsync, hipMemPoolTrimTo, hipMemPoolGetAttribute,
 hipDeviceSetMemPool, hipMemPoolSetAttribute, hipMemPoolSetAccess, hipMemPoolGetAccess

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipFreeAsync(
        dev_ptr: *mut ::core::ffi::c_void,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Releases freed memory back to the OS

 Releases memory back to the OS until the pool contains fewer than @p min_bytes_to_keep
 reserved bytes, or there is no more memory that the allocator can safely release.
 The allocator cannot release OS allocations that back outstanding asynchronous allocations.
 The OS allocations may happen at different granularity from the user allocations.

 @note Allocations that have not been freed count as outstanding.
 @note Allocations that have been asynchronously freed but whose completion has
 not been observed on the host (eg. by a synchronize) can count as outstanding.

 @param[in] mem_pool          The memory pool to trim allocations
 @param[in] min_bytes_to_hold If the pool has less than min_bytes_to_hold reserved,
 then the TrimTo operation is a no-op.  Otherwise the memory pool will contain
 at least min_bytes_to_hold bytes reserved after the operation.

 @returns #hipSuccess, #hipErrorInvalidValue

 @see hipMallocFromPoolAsync, hipMallocAsync, hipFreeAsync, hipMemPoolGetAttribute,
 hipDeviceSetMemPool, hipMemPoolSetAttribute, hipMemPoolSetAccess, hipMemPoolGetAccess

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipMemPoolTrimTo(
        mem_pool: hipMemPool_t,
        min_bytes_to_hold: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets attributes of a memory pool

 Supported attributes are:
 - @p hipMemPoolAttrReleaseThreshold: (value type = cuuint64_t)
                                  Amount of reserved memory in bytes to hold onto before trying
                                  to release memory back to the OS. When more than the release
                                  threshold bytes of memory are held by the memory pool, the
                                  allocator will try to release memory back to the OS on the
                                  next call to stream, event or context synchronize. (default 0)
 - @p hipMemPoolReuseFollowEventDependencies: (value type = int)
                                  Allow @p hipMallocAsync to use memory asynchronously freed
                                  in another stream as long as a stream ordering dependency
                                  of the allocating stream on the free action exists.
                                  HIP events and null stream interactions can create the required
                                  stream ordered dependencies. (default enabled)
 - @p hipMemPoolReuseAllowOpportunistic: (value type = int)
                                  Allow reuse of already completed frees when there is no dependency
                                  between the free and allocation. (default enabled)
 - @p hipMemPoolReuseAllowInternalDependencies: (value type = int)
                                  Allow @p hipMallocAsync to insert new stream dependencies
                                  in order to establish the stream ordering required to reuse
                                  a piece of memory released by @p hipFreeAsync (default enabled).

 @param [in] mem_pool The memory pool to modify
 @param [in] attr     The attribute to modify
 @param [in] value    Pointer to the value to assign

 @returns #hipSuccess, #hipErrorInvalidValue

 @see hipMallocFromPoolAsync, hipMallocAsync, hipFreeAsync, hipMemPoolGetAttribute,
 hipMemPoolTrimTo, hipDeviceSetMemPool, hipMemPoolSetAccess, hipMemPoolGetAccess

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipMemPoolSetAttribute(
        mem_pool: hipMemPool_t,
        attr: hipMemPoolAttr,
        value: *mut ::core::ffi::c_void,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets attributes of a memory pool

 Supported attributes are:
 - @p hipMemPoolAttrReleaseThreshold: (value type = cuuint64_t)
                                  Amount of reserved memory in bytes to hold onto before trying
                                  to release memory back to the OS. When more than the release
                                  threshold bytes of memory are held by the memory pool, the
                                  allocator will try to release memory back to the OS on the
                                  next call to stream, event or context synchronize. (default 0)
 - @p hipMemPoolReuseFollowEventDependencies: (value type = int)
                                  Allow @p hipMallocAsync to use memory asynchronously freed
                                  in another stream as long as a stream ordering dependency
                                  of the allocating stream on the free action exists.
                                  HIP events and null stream interactions can create the required
                                  stream ordered dependencies. (default enabled)
 - @p hipMemPoolReuseAllowOpportunistic: (value type = int)
                                  Allow reuse of already completed frees when there is no dependency
                                  between the free and allocation. (default enabled)
 - @p hipMemPoolReuseAllowInternalDependencies: (value type = int)
                                  Allow @p hipMallocAsync to insert new stream dependencies
                                  in order to establish the stream ordering required to reuse
                                  a piece of memory released by @p hipFreeAsync (default enabled).

 @param [in] mem_pool The memory pool to get attributes of
 @param [in] attr     The attribute to get
 @param [in] value    Retrieved value

 @returns  #hipSuccess, #hipErrorInvalidValue

 @see hipMallocFromPoolAsync, hipMallocAsync, hipFreeAsync,
 hipMemPoolTrimTo, hipDeviceSetMemPool, hipMemPoolSetAttribute, hipMemPoolSetAccess, hipMemPoolGetAccess

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipMemPoolGetAttribute(
        mem_pool: hipMemPool_t,
        attr: hipMemPoolAttr,
        value: *mut ::core::ffi::c_void,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Controls visibility of the specified pool between devices

 @param [in] mem_pool   Memory pool for acccess change
 @param [in] desc_list  Array of access descriptors. Each descriptor instructs the access to enable for a single gpu
 @param [in] count  Number of descriptors in the map array.

 @returns  #hipSuccess, #hipErrorInvalidValue

 @see hipMallocFromPoolAsync, hipMallocAsync, hipFreeAsync, hipMemPoolGetAttribute,
 hipMemPoolTrimTo, hipDeviceSetMemPool, hipMemPoolSetAttribute, hipMemPoolGetAccess

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipMemPoolSetAccess(
        mem_pool: hipMemPool_t,
        desc_list: *const hipMemAccessDesc,
        count: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns the accessibility of a pool from a device

 Returns the accessibility of the pool's memory from the specified location.

 @param [out] flags    Accessibility of the memory pool from the specified location/device
 @param [in] mem_pool   Memory pool being queried
 @param [in] location  Location/device for memory pool access

 @returns #hipSuccess, #hipErrorInvalidValue

 @see hipMallocFromPoolAsync, hipMallocAsync, hipFreeAsync, hipMemPoolGetAttribute,
 hipMemPoolTrimTo, hipDeviceSetMemPool, hipMemPoolSetAttribute, hipMemPoolSetAccess

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipMemPoolGetAccess(
        flags: *mut hipMemAccessFlags,
        mem_pool: hipMemPool_t,
        location: *mut hipMemLocation,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a memory pool

 Creates a HIP memory pool and returns the handle in @p mem_pool. The @p pool_props determines
 the properties of the pool such as the backing device and IPC capabilities.

 By default, the memory pool will be accessible from the device it is allocated on.

 @param [out] mem_pool    Contains createed memory pool
 @param [in] pool_props   Memory pool properties

 @note Specifying hipMemHandleTypeNone creates a memory pool that will not support IPC.

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported

 @see hipMallocFromPoolAsync, hipMallocAsync, hipFreeAsync, hipMemPoolGetAttribute, hipMemPoolDestroy,
 hipMemPoolTrimTo, hipDeviceSetMemPool, hipMemPoolSetAttribute, hipMemPoolSetAccess, hipMemPoolGetAccess

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipMemPoolCreate(
        mem_pool: *mut hipMemPool_t,
        pool_props: *const hipMemPoolProps,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Destroys the specified memory pool

 If any pointers obtained from this pool haven't been freed or
 the pool has free operations that haven't completed
 when @p hipMemPoolDestroy is invoked, the function will return immediately and the
 resources associated with the pool will be released automatically
 once there are no more outstanding allocations.

 Destroying the current mempool of a device sets the default mempool of
 that device as the current mempool for that device.

 @param [in] mem_pool Memory pool for destruction

 @note A device's default memory pool cannot be destroyed.

 @returns #hipSuccess, #hipErrorInvalidValue

 @see hipMallocFromPoolAsync, hipMallocAsync, hipFreeAsync, hipMemPoolGetAttribute, hipMemPoolCreate
 hipMemPoolTrimTo, hipDeviceSetMemPool, hipMemPoolSetAttribute, hipMemPoolSetAccess, hipMemPoolGetAccess

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipMemPoolDestroy(mem_pool: hipMemPool_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Allocates memory from a specified pool with stream ordered semantics.

 Inserts an allocation operation into @p stream.
 A pointer to the allocated memory is returned immediately in @p dev_ptr.
 The allocation must not be accessed until the allocation operation completes.
 The allocation comes from the specified memory pool.

 @note The specified memory pool may be from a device different than that of the specified @p stream.

 Basic stream ordering allows future work submitted into the same stream to use the allocation.
 Stream query, stream synchronize, and HIP events can be used to guarantee that the allocation
 operation completes before work submitted in a separate stream runs.

 @note During stream capture, this function results in the creation of an allocation node. In this case,
 the allocation is owned by the graph instead of the memory pool. The memory pool's properties
 are used to set the node's creation parameters.

 @param [out] dev_ptr Returned device pointer
 @param [in] size     Number of bytes to allocate
 @param [in] mem_pool The pool to allocate from
 @param [in] stream   The stream establishing the stream ordering semantic

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported, #hipErrorOutOfMemory

 @see hipMallocAsync, hipFreeAsync, hipMemPoolGetAttribute, hipMemPoolCreate
 hipMemPoolTrimTo, hipDeviceSetMemPool, hipMemPoolSetAttribute, hipMemPoolSetAccess, hipMemPoolGetAccess,

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipMallocFromPoolAsync(
        dev_ptr: *mut *mut ::core::ffi::c_void,
        size: usize,
        mem_pool: hipMemPool_t,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Exports a memory pool to the requested handle type.

 Given an IPC capable mempool, create an OS handle to share the pool with another process.
 A recipient process can convert the shareable handle into a mempool with @p hipMemPoolImportFromShareableHandle.
 Individual pointers can then be shared with the @p hipMemPoolExportPointer and @p hipMemPoolImportPointer APIs.
 The implementation of what the shareable handle is and how it can be transferred is defined by the requested
 handle type.

 @note To create an IPC capable mempool, create a mempool with a @p hipMemAllocationHandleType other
 than @p hipMemHandleTypeNone.

 @param [out] shared_handle Pointer to the location in which to store the requested handle
 @param [in] mem_pool       Pool to export
 @param [in] handle_type    The type of handle to create
 @param [in] flags          Must be 0

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorOutOfMemory

 @see hipMemPoolImportFromShareableHandle

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipMemPoolExportToShareableHandle(
        shared_handle: *mut ::core::ffi::c_void,
        mem_pool: hipMemPool_t,
        handle_type: hipMemAllocationHandleType,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Imports a memory pool from a shared handle.

 Specific allocations can be imported from the imported pool with @p hipMemPoolImportPointer.

 @note Imported memory pools do not support creating new allocations.
 As such imported memory pools may not be used in @p hipDeviceSetMemPool
 or @p hipMallocFromPoolAsync calls.

 @param [out] mem_pool     Returned memory pool
 @param [in] shared_handle OS handle of the pool to open
 @param [in] handle_type   The type of handle being imported
 @param [in] flags         Must be 0

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorOutOfMemory

 @see hipMemPoolExportToShareableHandle

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipMemPoolImportFromShareableHandle(
        mem_pool: *mut hipMemPool_t,
        shared_handle: *mut ::core::ffi::c_void,
        handle_type: hipMemAllocationHandleType,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Export data to share a memory pool allocation between processes.

 Constructs @p export_data for sharing a specific allocation from an already shared memory pool.
 The recipient process can import the allocation with the @p hipMemPoolImportPointer api.
 The data is not a handle and may be shared through any IPC mechanism.

 @param[out] export_data  Returned export data
 @param[in] dev_ptr       Pointer to memory being exported

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorOutOfMemory

 @see hipMemPoolImportPointer

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipMemPoolExportPointer(
        export_data: *mut hipMemPoolPtrExportData,
        dev_ptr: *mut ::core::ffi::c_void,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Import a memory pool allocation from another process.

 Returns in @p dev_ptr a pointer to the imported memory.
 The imported memory must not be accessed before the allocation operation completes
 in the exporting process. The imported memory must be freed from all importing processes before
 being freed in the exporting process. The pointer may be freed with @p hipFree
 or @p hipFreeAsync. If @p hipFreeAsync is used, the free must be completed
 on the importing process before the free operation on the exporting process.

 @note The @p hipFreeAsync api may be used in the exporting process before
 the @p hipFreeAsync operation completes in its stream as long as the
 @p hipFreeAsync in the exporting process specifies a stream with
 a stream dependency on the importing process's @p hipFreeAsync.

 @param [out] dev_ptr     Pointer to imported memory
 @param [in] mem_pool     Memory pool from which to import a pointer
 @param [in] export_data  Data specifying the memory to import

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotInitialized, #hipErrorOutOfMemory

 @see hipMemPoolExportPointer

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipMemPoolImportPointer(
        dev_ptr: *mut *mut ::core::ffi::c_void,
        mem_pool: hipMemPool_t,
        export_data: *mut hipMemPoolPtrExportData,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Allocate device accessible page locked host memory

  @param[out] ptr Pointer to the allocated host pinned memory
  @param[in]  size Requested memory size in bytes
  @param[in]  flags Type of host memory allocation see below

  If size is 0, no memory is allocated, *ptr returns nullptr, and hipSuccess is returned.

  Flags:
  - #hipHostAllocDefault   Default pinned memory allocation on the host.
  - #hipHostAllocPortable  Memory is considered allocated by all contexts.
  - #hipHostAllocMapped    Map the allocation into the address space for the current device.
  - #hipHostAllocWriteCombined  Allocates the memory as write-combined.

  @return #hipSuccess, #hipErrorOutOfMemory, #hipErrorInvalidValue*/
    pub fn hipHostAlloc(
        ptr: *mut *mut ::core::ffi::c_void,
        size: usize,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Get Device pointer from Host Pointer allocated through hipHostMalloc

  @param[out] devPtr Device Pointer mapped to passed host pointer
  @param[in]  hstPtr Host Pointer allocated through hipHostMalloc
  @param[in]  flags Flags to be passed for extension

  @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorOutOfMemory

  @see hipSetDeviceFlags, hipHostMalloc*/
    pub fn hipHostGetDevicePointer(
        devPtr: *mut *mut ::core::ffi::c_void,
        hstPtr: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Return flags associated with host pointer

  @param[out] flagsPtr Memory location to store flags
  @param[in]  hostPtr Host Pointer allocated through hipHostMalloc
  @returns #hipSuccess, #hipErrorInvalidValue

  @see hipHostMalloc*/
    pub fn hipHostGetFlags(
        flagsPtr: *mut ::core::ffi::c_uint,
        hostPtr: *mut ::core::ffi::c_void,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Register host memory so it can be accessed from the current device.

  @param[out] hostPtr Pointer to host memory to be registered.
  @param[in] sizeBytes Size of the host memory
  @param[in] flags  See below.

  Flags:
  - #hipHostRegisterDefault   Memory is Mapped and Portable
  - #hipHostRegisterPortable  Memory is considered registered by all contexts.  HIP only supports
 one context so this is always assumed true.
  - #hipHostRegisterMapped    Map the allocation into the address space for the current device.
 The device pointer can be obtained with #hipHostGetDevicePointer.


  After registering the memory, use #hipHostGetDevicePointer to obtain the mapped device pointer.
  On many systems, the mapped device pointer will have a different value than the mapped host
 pointer.  Applications must use the device pointer in device code, and the host pointer in host
 code.

  On some systems, registered memory is pinned.  On some systems, registered memory may not be
 actually be pinned but uses OS or hardware facilities to all GPU access to the host memory.

  Developers are strongly encouraged to register memory blocks which are aligned to the host
 cache-line size. (typically 64-bytes but can be obtains from the CPUID instruction).

  If registering non-aligned pointers, the application must take care when register pointers from
 the same cache line on different devices.  HIP's coarse-grained synchronization model does not
 guarantee correct results if different devices write to different parts of the same cache block -
 typically one of the writes will "win" and overwrite data from the other registered memory
 region.

  @returns #hipSuccess, #hipErrorOutOfMemory

  @see hipHostUnregister, hipHostGetFlags, hipHostGetDevicePointer*/
    pub fn hipHostRegister(
        hostPtr: *mut ::core::ffi::c_void,
        sizeBytes: usize,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Un-register host pointer

  @param[in] hostPtr Host pointer previously registered with #hipHostRegister
  @returns Error code

  @see hipHostRegister*/
    pub fn hipHostUnregister(hostPtr: *mut ::core::ffi::c_void) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  Allocates at least width (in bytes) * height bytes of linear memory
  Padding may occur to ensure alighnment requirements are met for the given row
  The change in width size due to padding will be returned in *pitch.
  Currently the alignment is set to 128 bytes

  @param[out] ptr Pointer to the allocated device memory
  @param[out] pitch Pitch for allocation (in bytes)
  @param[in]  width Requested pitched allocation width (in bytes)
  @param[in]  height Requested pitched allocation height

  If size is 0, no memory is allocated, *ptr returns nullptr, and hipSuccess is returned.

  @returns Error code

  @see hipMalloc, hipFree, hipMallocArray, hipFreeArray, hipHostFree, hipMalloc3D,
 hipMalloc3DArray, hipHostMalloc*/
    pub fn hipMallocPitch(
        ptr: *mut *mut ::core::ffi::c_void,
        pitch: *mut usize,
        width: usize,
        height: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  Allocates at least width (in bytes) * height bytes of linear memory
  Padding may occur to ensure alighnment requirements are met for the given row
  The change in width size due to padding will be returned in *pitch.
  Currently the alignment is set to 128 bytes

  @param[out] dptr  Pointer to the allocated device memory
  @param[out] pitch  Pitch for allocation (in bytes)
  @param[in]  widthInBytes  Requested pitched allocation width (in bytes)
  @param[in]  height  Requested pitched allocation height
  @param[in]  elementSizeBytes  The size of element bytes, should be 4, 8 or 16

  If size is 0, no memory is allocated, *ptr returns nullptr, and hipSuccess is returned.
  The intended usage of pitch is as a separate parameter of the allocation, used to compute addresses within the 2D array.
  Given the row and column of an array element of type T, the address is computed as:
  T* pElement = (T*)((char*)BaseAddress + Row * Pitch) + Column;

  @returns Error code

  @see hipMalloc, hipFree, hipMallocArray, hipFreeArray, hipHostFree, hipMalloc3D,
 hipMalloc3DArray, hipHostMalloc*/
    pub fn hipMemAllocPitch(
        dptr: *mut hipDeviceptr_t,
        pitch: *mut usize,
        widthInBytes: usize,
        height: usize,
        elementSizeBytes: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Free memory allocated by the hcc hip memory allocation API.
  This API performs an implicit hipDeviceSynchronize() call.
  If pointer is NULL, the hip runtime is initialized and hipSuccess is returned.

  @param[in] ptr Pointer to memory to be freed
  @returns #hipSuccess
  @returns #hipErrorInvalidDevicePointer (if pointer is invalid, including host pointers allocated
 with hipHostMalloc)

  @see hipMalloc, hipMallocPitch, hipMallocArray, hipFreeArray, hipHostFree, hipMalloc3D,
 hipMalloc3DArray, hipHostMalloc*/
    pub fn hipFree(ptr: *mut ::core::ffi::c_void) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Frees page-locked memory
  This API performs an implicit hipDeviceSynchronize() call.
  If pointer is NULL, the hip runtime is initialized and hipSuccess is returned.

  @param[in] ptr Pointer to memory to be freed
  @returns #hipSuccess,
          #hipErrorInvalidValue (if pointer is invalid, including device pointers allocated
  with hipMalloc)
*/
    pub fn hipFreeHost(ptr: *mut ::core::ffi::c_void) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Free memory allocated by the hcc hip host memory allocation API
  This API performs an implicit hipDeviceSynchronize() call.
  If pointer is NULL, the hip runtime is initialized and hipSuccess is returned.

  @ingroup MemoryD

  @param[in] ptr Pointer to memory to be freed
  @returns #hipSuccess,
          #hipErrorInvalidValue (if pointer is invalid, including device pointers allocated with
 hipMalloc)

  @see hipMalloc, hipMallocPitch, hipFree, hipMallocArray, hipFreeArray, hipMalloc3D,
 hipMalloc3DArray, hipHostMalloc
*/
    pub fn hipHostFree(ptr: *mut ::core::ffi::c_void) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copy data from src to dst.

  It supports memory from host to device,
  device to host, device to device and host to host
  The src and dst must not overlap.

  For hipMemcpy, the copy is always performed by the current device (set by hipSetDevice).
  For multi-gpu or peer-to-peer configurations, it is recommended to set the current device to the
  device where the src data is physically located. For optimal peer-to-peer copies, the copy device
  must be able to access the src and dst pointers (by calling hipDeviceEnablePeerAccess with copy
  agent as the current device and src/dest as the peerDevice argument.  if this is not done, the
  hipMemcpy will still work, but will perform the copy using a staging buffer on the host.
  Calling hipMemcpy with dst and src pointers that do not match the hipMemcpyKind results in
  undefined behavior.

  @param[out]  dst Data being copy to
  @param[in]  src Data being copy from
  @param[in]  sizeBytes Data size in bytes
  @param[in]  kind Kind of transfer
  @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorUnknown

  @see hipArrayCreate, hipArrayDestroy, hipArrayGetDescriptor, hipMemAlloc, hipMemAllocHost,
 hipMemAllocPitch, hipMemcpy2D, hipMemcpy2DAsync, hipMemcpy2DUnaligned, hipMemcpyAtoA,
 hipMemcpyAtoD, hipMemcpyAtoH, hipMemcpyAtoHAsync, hipMemcpyDtoA, hipMemcpyDtoD,
 hipMemcpyDtoDAsync, hipMemcpyDtoH, hipMemcpyDtoHAsync, hipMemcpyHtoA, hipMemcpyHtoAAsync,
 hipMemcpyHtoDAsync, hipMemFree, hipMemFreeHost, hipMemGetAddressRange, hipMemGetInfo,
 hipMemHostAlloc, hipMemHostGetDevicePointer*/
    pub fn hipMemcpy(
        dst: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        sizeBytes: usize,
        kind: hipMemcpyKind,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Memory copy on the stream.
  It allows single or multiple devices to do memory copy on single or multiple streams.

  @param[out]  dst Data being copy to
  @param[in]  src Data being copy from
  @param[in]  sizeBytes Data size in bytes
  @param[in]  kind Kind of transfer
  @param[in]  stream Valid stream
  @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorUnknown, #hipErrorContextIsDestroyed

  @see hipMemcpy, hipStreamCreate, hipStreamSynchronize, hipStreamDestroy, hipSetDevice, hipLaunchKernelGGL
*/
    pub fn hipMemcpyWithStream(
        dst: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        sizeBytes: usize,
        kind: hipMemcpyKind,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copy data from Host to Device

  @param[out]  dst Data being copy to
  @param[in]   src Data being copy from
  @param[in]   sizeBytes Data size in bytes

  @returns #hipSuccess, #hipErrorDeinitialized, #hipErrorNotInitialized, #hipErrorInvalidContext,
 #hipErrorInvalidValue

  @see hipArrayCreate, hipArrayDestroy, hipArrayGetDescriptor, hipMemAlloc, hipMemAllocHost,
 hipMemAllocPitch, hipMemcpy2D, hipMemcpy2DAsync, hipMemcpy2DUnaligned, hipMemcpyAtoA,
 hipMemcpyAtoD, hipMemcpyAtoH, hipMemcpyAtoHAsync, hipMemcpyDtoA, hipMemcpyDtoD,
 hipMemcpyDtoDAsync, hipMemcpyDtoH, hipMemcpyDtoHAsync, hipMemcpyHtoA, hipMemcpyHtoAAsync,
 hipMemcpyHtoDAsync, hipMemFree, hipMemFreeHost, hipMemGetAddressRange, hipMemGetInfo,
 hipMemHostAlloc, hipMemHostGetDevicePointer*/
    pub fn hipMemcpyHtoD(
        dst: hipDeviceptr_t,
        src: *mut ::core::ffi::c_void,
        sizeBytes: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copy data from Device to Host

  @param[out]  dst Data being copy to
  @param[in]   src Data being copy from
  @param[in]   sizeBytes Data size in bytes

  @returns #hipSuccess, #hipErrorDeinitialized, #hipErrorNotInitialized, #hipErrorInvalidContext,
 #hipErrorInvalidValue

  @see hipArrayCreate, hipArrayDestroy, hipArrayGetDescriptor, hipMemAlloc, hipMemAllocHost,
 hipMemAllocPitch, hipMemcpy2D, hipMemcpy2DAsync, hipMemcpy2DUnaligned, hipMemcpyAtoA,
 hipMemcpyAtoD, hipMemcpyAtoH, hipMemcpyAtoHAsync, hipMemcpyDtoA, hipMemcpyDtoD,
 hipMemcpyDtoDAsync, hipMemcpyDtoH, hipMemcpyDtoHAsync, hipMemcpyHtoA, hipMemcpyHtoAAsync,
 hipMemcpyHtoDAsync, hipMemFree, hipMemFreeHost, hipMemGetAddressRange, hipMemGetInfo,
 hipMemHostAlloc, hipMemHostGetDevicePointer*/
    pub fn hipMemcpyDtoH(
        dst: *mut ::core::ffi::c_void,
        src: hipDeviceptr_t,
        sizeBytes: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copy data from Device to Device

  @param[out]  dst Data being copy to
  @param[in]   src Data being copy from
  @param[in]   sizeBytes Data size in bytes

  @returns #hipSuccess, #hipErrorDeinitialized, #hipErrorNotInitialized, #hipErrorInvalidContext,
 #hipErrorInvalidValue

  @see hipArrayCreate, hipArrayDestroy, hipArrayGetDescriptor, hipMemAlloc, hipMemAllocHost,
 hipMemAllocPitch, hipMemcpy2D, hipMemcpy2DAsync, hipMemcpy2DUnaligned, hipMemcpyAtoA,
 hipMemcpyAtoD, hipMemcpyAtoH, hipMemcpyAtoHAsync, hipMemcpyDtoA, hipMemcpyDtoD,
 hipMemcpyDtoDAsync, hipMemcpyDtoH, hipMemcpyDtoHAsync, hipMemcpyHtoA, hipMemcpyHtoAAsync,
 hipMemcpyHtoDAsync, hipMemFree, hipMemFreeHost, hipMemGetAddressRange, hipMemGetInfo,
 hipMemHostAlloc, hipMemHostGetDevicePointer*/
    pub fn hipMemcpyDtoD(
        dst: hipDeviceptr_t,
        src: hipDeviceptr_t,
        sizeBytes: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copies from one 1D array to device memory.

  @param[out]  dstDevice Destination device pointer
  @param[in]   srcArray Source array
  @param[in]   srcOffset Offset in bytes of source array
  @param[in]   ByteCount Size of memory copy in bytes

  @returns #hipSuccess, #hipErrorDeinitialized, #hipErrorNotInitialized, #hipErrorInvalidContext,
 #hipErrorInvalidValue

  @see hipArrayCreate, hipArrayDestroy, hipArrayGetDescriptor, hipMemAlloc, hipMemAllocHost,
 hipMemAllocPitch, hipMemcpy2D, hipMemcpy2DAsync, hipMemcpy2DUnaligned, hipMemcpyAtoA,
 hipMemcpyAtoD, hipMemcpyAtoH, hipMemcpyAtoHAsync, hipMemcpyDtoA, hipMemcpyDtoD,
 hipMemcpyDtoDAsync, hipMemcpyDtoH, hipMemcpyDtoHAsync, hipMemcpyHtoA, hipMemcpyHtoAAsync,
 hipMemcpyHtoDAsync, hipMemFree, hipMemFreeHost, hipMemGetAddressRange, hipMemGetInfo,
 hipMemHostAlloc, hipMemHostGetDevicePointer*/
    pub fn hipMemcpyAtoD(
        dstDevice: hipDeviceptr_t,
        srcArray: hipArray_t,
        srcOffset: usize,
        ByteCount: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copies from device memory to a 1D array.

  @param[out]  dstArray Destination array
  @param[in]   dstOffset Offset in bytes of destination array
  @param[in]   srcDevice Source device pointer
  @param[in]   ByteCount Size of memory copy in bytes

  @returns #hipSuccess, #hipErrorDeinitialized, #hipErrorNotInitialized, #hipErrorInvalidContext,
 #hipErrorInvalidValue

  @see hipArrayCreate, hipArrayDestroy, hipArrayGetDescriptor, hipMemAlloc, hipMemAllocHost,
 hipMemAllocPitch, hipMemcpy2D, hipMemcpy2DAsync, hipMemcpy2DUnaligned, hipMemcpyAtoA,
 hipMemcpyAtoD, hipMemcpyAtoH, hipMemcpyAtoHAsync, hipMemcpyDtoA, hipMemcpyDtoD,
 hipMemcpyDtoDAsync, hipMemcpyDtoH, hipMemcpyDtoHAsync, hipMemcpyHtoA, hipMemcpyHtoAAsync,
 hipMemcpyHtoDAsync, hipMemFree, hipMemFreeHost, hipMemGetAddressRange, hipMemGetInfo,
 hipMemHostAlloc, hipMemHostGetDevicePointer*/
    pub fn hipMemcpyDtoA(
        dstArray: hipArray_t,
        dstOffset: usize,
        srcDevice: hipDeviceptr_t,
        ByteCount: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copies from one 1D array to another.

  @param[out]  dstArray Destination array
  @param[in]   dstOffset Offset in bytes of destination array
  @param[in]   srcArray Source array
  @param[in]   srcOffset Offset in bytes of source array
  @param[in]   ByteCount Size of memory copy in bytes

  @returns #hipSuccess, #hipErrorDeinitialized, #hipErrorNotInitialized, #hipErrorInvalidContext,
 #hipErrorInvalidValue

  @see hipArrayCreate, hipArrayDestroy, hipArrayGetDescriptor, hipMemAlloc, hipMemAllocHost,
 hipMemAllocPitch, hipMemcpy2D, hipMemcpy2DAsync, hipMemcpy2DUnaligned, hipMemcpyAtoA,
 hipMemcpyAtoD, hipMemcpyAtoH, hipMemcpyAtoHAsync, hipMemcpyDtoA, hipMemcpyDtoD,
 hipMemcpyDtoDAsync, hipMemcpyDtoH, hipMemcpyDtoHAsync, hipMemcpyHtoA, hipMemcpyHtoAAsync,
 hipMemcpyHtoDAsync, hipMemFree, hipMemFreeHost, hipMemGetAddressRange, hipMemGetInfo,
 hipMemHostAlloc, hipMemHostGetDevicePointer*/
    pub fn hipMemcpyAtoA(
        dstArray: hipArray_t,
        dstOffset: usize,
        srcArray: hipArray_t,
        srcOffset: usize,
        ByteCount: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copy data from Host to Device asynchronously

  @param[out]  dst  Data being copy to
  @param[in]   src  Data being copy from
  @param[in]   sizeBytes  Data size in bytes
  @param[in]   stream  Stream identifier

  @returns #hipSuccess, #hipErrorDeinitialized, #hipErrorNotInitialized, #hipErrorInvalidContext,
 #hipErrorInvalidValue

  @see hipArrayCreate, hipArrayDestroy, hipArrayGetDescriptor, hipMemAlloc, hipMemAllocHost,
 hipMemAllocPitch, hipMemcpy2D, hipMemcpy2DAsync, hipMemcpy2DUnaligned, hipMemcpyAtoA,
 hipMemcpyAtoD, hipMemcpyAtoH, hipMemcpyAtoHAsync, hipMemcpyDtoA, hipMemcpyDtoD,
 hipMemcpyDtoDAsync, hipMemcpyDtoH, hipMemcpyDtoHAsync, hipMemcpyHtoA, hipMemcpyHtoAAsync,
 hipMemcpyHtoDAsync, hipMemFree, hipMemFreeHost, hipMemGetAddressRange, hipMemGetInfo,
 hipMemHostAlloc, hipMemHostGetDevicePointer*/
    pub fn hipMemcpyHtoDAsync(
        dst: hipDeviceptr_t,
        src: *mut ::core::ffi::c_void,
        sizeBytes: usize,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copy data from Device to Host asynchronously

  @param[out]  dst Data being copy to
  @param[in]   src Data being copy from
  @param[in]   sizeBytes Data size in bytes
  @param[in]   stream  Stream identifier

  @returns #hipSuccess, #hipErrorDeinitialized, #hipErrorNotInitialized, #hipErrorInvalidContext,
 #hipErrorInvalidValue

  @see hipArrayCreate, hipArrayDestroy, hipArrayGetDescriptor, hipMemAlloc, hipMemAllocHost,
 hipMemAllocPitch, hipMemcpy2D, hipMemcpy2DAsync, hipMemcpy2DUnaligned, hipMemcpyAtoA,
 hipMemcpyAtoD, hipMemcpyAtoH, hipMemcpyAtoHAsync, hipMemcpyDtoA, hipMemcpyDtoD,
 hipMemcpyDtoDAsync, hipMemcpyDtoH, hipMemcpyDtoHAsync, hipMemcpyHtoA, hipMemcpyHtoAAsync,
 hipMemcpyHtoDAsync, hipMemFree, hipMemFreeHost, hipMemGetAddressRange, hipMemGetInfo,
 hipMemHostAlloc, hipMemHostGetDevicePointer*/
    pub fn hipMemcpyDtoHAsync(
        dst: *mut ::core::ffi::c_void,
        src: hipDeviceptr_t,
        sizeBytes: usize,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copy data from Device to Device asynchronously

  @param[out]  dst  Data being copy to
  @param[in]   src  Data being copy from
  @param[in]   sizeBytes  Data size in bytes
  @param[in]   stream  Stream identifier

  @returns #hipSuccess, #hipErrorDeinitialized, #hipErrorNotInitialized, #hipErrorInvalidContext,
 #hipErrorInvalidValue

  @see hipArrayCreate, hipArrayDestroy, hipArrayGetDescriptor, hipMemAlloc, hipMemAllocHost,
 hipMemAllocPitch, hipMemcpy2D, hipMemcpy2DAsync, hipMemcpy2DUnaligned, hipMemcpyAtoA,
 hipMemcpyAtoD, hipMemcpyAtoH, hipMemcpyAtoHAsync, hipMemcpyDtoA, hipMemcpyDtoD,
 hipMemcpyDtoDAsync, hipMemcpyDtoH, hipMemcpyDtoHAsync, hipMemcpyHtoA, hipMemcpyHtoAAsync,
 hipMemcpyHtoDAsync, hipMemFree, hipMemFreeHost, hipMemGetAddressRange, hipMemGetInfo,
 hipMemHostAlloc, hipMemHostGetDevicePointer*/
    pub fn hipMemcpyDtoDAsync(
        dst: hipDeviceptr_t,
        src: hipDeviceptr_t,
        sizeBytes: usize,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Copies from one 1D array to host memory.

  @param[out]  dstHost Destination pointer
  @param[in]   srcArray Source array
  @param[in]   srcOffset Offset in bytes of source array
  @param[in]   ByteCount Size of memory copy in bytes
  @param[in]   stream Stream identifier

  @returns #hipSuccess, #hipErrorDeinitialized, #hipErrorNotInitialized, #hipErrorInvalidContext,
 #hipErrorInvalidValue

  @see hipArrayCreate, hipArrayDestroy, hipArrayGetDescriptor, hipMemAlloc, hipMemAllocHost,
 hipMemAllocPitch, hipMemcpy2D, hipMemcpy2DAsync, hipMemcpy2DUnaligned, hipMemcpyAtoA,
 hipMemcpyAtoD, hipMemcpyAtoH, hipMemcpyAtoHAsync, hipMemcpyDtoA, hipMemcpyDtoD,
 hipMemcpyDtoDAsync, hipMemcpyDtoH, hipMemcpyDtoHAsync, hipMemcpyHtoA, hipMemcpyHtoAAsync,
 hipMemcpyHtoDAsync, hipMemFree, hipMemFreeHost, hipMemGetAddressRange, hipMemGetInfo,
 hipMemHostAlloc, hipMemHostGetDevicePointer*/
    pub fn hipMemcpyAtoHAsync(
        dstHost: *mut ::core::ffi::c_void,
        srcArray: hipArray_t,
        srcOffset: usize,
        ByteCount: usize,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Copies from host memory to a 1D array.

  @param[out]  dstArray Destination array
  @param[in]   dstOffset Offset in bytes of destination array
  @param[in]   srcHost Source host pointer
  @param[in]   ByteCount Size of memory copy in bytes
  @param[in]   stream Stream identifier

  @returns #hipSuccess, #hipErrorDeinitialized, #hipErrorNotInitialized, #hipErrorInvalidContext,
 #hipErrorInvalidValue

  @see hipArrayCreate, hipArrayDestroy, hipArrayGetDescriptor, hipMemAlloc, hipMemAllocHost,
 hipMemAllocPitch, hipMemcpy2D, hipMemcpy2DAsync, hipMemcpy2DUnaligned, hipMemcpyAtoA,
 hipMemcpyAtoD, hipMemcpyAtoH, hipMemcpyAtoHAsync, hipMemcpyDtoA, hipMemcpyDtoD,
 hipMemcpyDtoDAsync, hipMemcpyDtoH, hipMemcpyDtoHAsync, hipMemcpyHtoA, hipMemcpyHtoAAsync,
 hipMemcpyHtoDAsync, hipMemFree, hipMemFreeHost, hipMemGetAddressRange, hipMemGetInfo,
 hipMemHostAlloc, hipMemHostGetDevicePointer*/
    pub fn hipMemcpyHtoAAsync(
        dstArray: hipArray_t,
        dstOffset: usize,
        srcHost: *const ::core::ffi::c_void,
        ByteCount: usize,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Returns a global pointer from a module.
  @ingroup Module

  Returns in *dptr and *bytes the pointer and size of the global of name name located in module hmod.
  If no variable of that name exists, it returns hipErrorNotFound. Both parameters dptr and bytes are optional.
  If one of them is NULL, it is ignored and hipSuccess is returned.

  @param[out]  dptr  Returns global device pointer
  @param[out]  bytes Returns global size in bytes
  @param[in]   hmod  Module to retrieve global from
  @param[in]   name  Name of global to retrieve

  @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotFound, #hipErrorInvalidContext
*/
    pub fn hipModuleGetGlobal(
        dptr: *mut hipDeviceptr_t,
        bytes: *mut usize,
        hmod: hipModule_t,
        name: *const ::core::ffi::c_char,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Gets device pointer associated with symbol on the device.

  @param[out]  devPtr  pointer to the device associated the symbole
  @param[in]   symbol  pointer to the symbole of the device

  @returns #hipSuccess, #hipErrorInvalidValue
*/
    pub fn hipGetSymbolAddress(
        devPtr: *mut *mut ::core::ffi::c_void,
        symbol: *const ::core::ffi::c_void,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Gets the size of the given symbol on the device.

  @param[in]   symbol  pointer to the device symbole
  @param[out]  size  pointer to the size

  @returns #hipSuccess, #hipErrorInvalidValue
*/
    pub fn hipGetSymbolSize(
        size: *mut usize,
        symbol: *const ::core::ffi::c_void,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets the pointer of requested HIP driver function.

 @param[in] symbol  The Symbol name of the driver function to request.
 @param[out] pfn  Output pointer to the requested driver function.
 @param[in] hipVersion  The HIP version for the requested driver function symbol.
 HIP version is defined as 100*version_major + version_minor. For example, in HIP 6.1, the
 hipversion is 601, for the symbol function "hipGetDeviceProperties", the specified hipVersion 601
 is greater or equal to the version 600, the symbol function will be handle properly as backend
 compatible function.

 @param[in] flags  Currently only default flag is suppported.
 @param[out] symbolStatus  Optional enumeration for returned status of searching for symbol driver
 function based on the input hipVersion.

 Returns hipSuccess if the returned pfn is addressed to the pointer of found driver function.

 @returns #hipSuccess, #hipErrorInvalidValue.*/
    pub fn hipGetProcAddress(
        symbol: *const ::core::ffi::c_char,
        pfn: *mut *mut ::core::ffi::c_void,
        hipVersion: ::core::ffi::c_int,
        flags: u64,
        symbolStatus: *mut hipDriverProcAddressQueryResult,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copies data to the given symbol on the device.
 Symbol HIP APIs allow a kernel to define a device-side data symbol which can be accessed on
 the host side. The symbol can be in __constant or device space.
 Note that the symbol name needs to be encased in the HIP_SYMBOL macro.
 This also applies to hipMemcpyFromSymbol, hipGetSymbolAddress, and hipGetSymbolSize.
 For detailed usage, see the
 <a href="https://rocm.docs.amd.com/projects/HIP/en/latest/how-to/hip_porting_guide.html#memcpytosymbol">memcpyToSymbol example</a>
 in the HIP Porting Guide.


  @param[out]  symbol  pointer to the device symbole
  @param[in]   src  pointer to the source address
  @param[in]   sizeBytes  size in bytes to copy
  @param[in]   offset  offset in bytes from start of symbole
  @param[in]   kind  type of memory transfer

  @returns #hipSuccess, #hipErrorInvalidValue
*/
    pub fn hipMemcpyToSymbol(
        symbol: *const ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        sizeBytes: usize,
        offset: usize,
        kind: hipMemcpyKind,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copies data to the given symbol on the device asynchronously.

  @param[out]  symbol  pointer to the device symbole
  @param[in]   src  pointer to the source address
  @param[in]   sizeBytes  size in bytes to copy
  @param[in]   offset  offset in bytes from start of symbole
  @param[in]   kind  type of memory transfer
  @param[in]   stream  stream identifier

  @returns #hipSuccess, #hipErrorInvalidValue
*/
    pub fn hipMemcpyToSymbolAsync(
        symbol: *const ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        sizeBytes: usize,
        offset: usize,
        kind: hipMemcpyKind,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copies data from the given symbol on the device.

  @param[out]  dst  Returns pointer to destinition memory address
  @param[in]   symbol  Pointer to the symbole address on the device
  @param[in]   sizeBytes  Size in bytes to copy
  @param[in]   offset  Offset in bytes from the start of symbole
  @param[in]   kind  Type of memory transfer

  @returns #hipSuccess, #hipErrorInvalidValue
*/
    pub fn hipMemcpyFromSymbol(
        dst: *mut ::core::ffi::c_void,
        symbol: *const ::core::ffi::c_void,
        sizeBytes: usize,
        offset: usize,
        kind: hipMemcpyKind,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copies data from the given symbol on the device asynchronously.

  @param[out]  dst  Returns pointer to destinition memory address
  @param[in]   symbol  pointer to the symbole address on the device
  @param[in]   sizeBytes  size in bytes to copy
  @param[in]   offset  offset in bytes from the start of symbole
  @param[in]   kind  type of memory transfer
  @param[in]   stream  stream identifier

  @returns #hipSuccess, #hipErrorInvalidValue
*/
    pub fn hipMemcpyFromSymbolAsync(
        dst: *mut ::core::ffi::c_void,
        symbol: *const ::core::ffi::c_void,
        sizeBytes: usize,
        offset: usize,
        kind: hipMemcpyKind,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copy data from src to dst asynchronously.

  @warning If host or dest are not pinned, the memory copy will be performed synchronously.  For
 best performance, use hipHostMalloc to allocate host memory that is transferred asynchronously.

  @warning on HCC hipMemcpyAsync does not support overlapped H2D and D2H copies.
  For hipMemcpy, the copy is always performed by the device associated with the specified stream.

  For multi-gpu or peer-to-peer configurations, it is recommended to use a stream which is a
 attached to the device where the src data is physically located. For optimal peer-to-peer copies,
 the copy device must be able to access the src and dst pointers (by calling
 hipDeviceEnablePeerAccess with copy agent as the current device and src/dest as the peerDevice
 argument.  if this is not done, the hipMemcpy will still work, but will perform the copy using a
 staging buffer on the host.

  @param[out] dst Data being copy to
  @param[in]  src Data being copy from
  @param[in]  sizeBytes Data size in bytes
  @param[in]  kind  Type of memory transfer
  @param[in]  stream  Stream identifier
  @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorUnknown

  @see hipMemcpy, hipMemcpy2D, hipMemcpyToArray, hipMemcpy2DToArray, hipMemcpyFromArray,
 hipMemcpy2DFromArray, hipMemcpyArrayToArray, hipMemcpy2DArrayToArray, hipMemcpyToSymbol,
 hipMemcpyFromSymbol, hipMemcpy2DAsync, hipMemcpyToArrayAsync, hipMemcpy2DToArrayAsync,
 hipMemcpyFromArrayAsync, hipMemcpy2DFromArrayAsync, hipMemcpyToSymbolAsync,
 hipMemcpyFromSymbolAsync*/
    pub fn hipMemcpyAsync(
        dst: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        sizeBytes: usize,
        kind: hipMemcpyKind,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Fills the first sizeBytes bytes of the memory area pointed to by dest with the constant
 byte value value.

  @param[out] dst  Data being filled
  @param[in]  value  Value to be set
  @param[in]  sizeBytes  Data size in bytes
  @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotInitialized*/
    pub fn hipMemset(
        dst: *mut ::core::ffi::c_void,
        value: ::core::ffi::c_int,
        sizeBytes: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Fills the first sizeBytes bytes of the memory area pointed to by dest with the constant
 byte value value.

  @param[out] dest  Data ptr to be filled
  @param[in]  value  Value to be set
  @param[in]  count  Number of values to be set
  @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotInitialized*/
    pub fn hipMemsetD8(
        dest: hipDeviceptr_t,
        value: ::core::ffi::c_uchar,
        count: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Fills the first sizeBytes bytes of the memory area pointed to by dest with the constant
 byte value value.

 hipMemsetD8Async() is asynchronous with respect to the host, so the call may return before the
 memset is complete. The operation can optionally be associated to a stream by passing a non-zero
 stream argument. If stream is non-zero, the operation may overlap with operations in other
 streams.

  @param[out] dest  Data ptr to be filled
  @param[in]  value  Constant value to be set
  @param[in]  count  Number of values to be set
  @param[in]  stream  Stream identifier
  @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotInitialized*/
    pub fn hipMemsetD8Async(
        dest: hipDeviceptr_t,
        value: ::core::ffi::c_uchar,
        count: usize,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Fills the first sizeBytes bytes of the memory area pointed to by dest with the constant
 short value value.

  @param[out] dest  Data ptr to be filled
  @param[in]  value  Constant value to be set
  @param[in]  count  Number of values to be set
  @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotInitialized*/
    pub fn hipMemsetD16(
        dest: hipDeviceptr_t,
        value: ::core::ffi::c_ushort,
        count: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Fills the first sizeBytes bytes of the memory area pointed to by dest with the constant
 short value value.

 hipMemsetD16Async() is asynchronous with respect to the host, so the call may return before the
 memset is complete. The operation can optionally be associated to a stream by passing a non-zero
 stream argument. If stream is non-zero, the operation may overlap with operations in other
 streams.

  @param[out] dest  Data ptr to be filled
  @param[in]  value  Constant value to be set
  @param[in]  count  Number of values to be set
  @param[in]  stream  Stream identifier
  @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotInitialized*/
    pub fn hipMemsetD16Async(
        dest: hipDeviceptr_t,
        value: ::core::ffi::c_ushort,
        count: usize,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Fills the memory area pointed to by dest with the constant integer
 value for specified number of times.

  @param[out] dest  Data being filled
  @param[in]  value  Constant value to be set
  @param[in]  count  Number of values to be set
  @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotInitialized*/
    pub fn hipMemsetD32(
        dest: hipDeviceptr_t,
        value: ::core::ffi::c_int,
        count: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Fills the first sizeBytes bytes of the memory area pointed to by dev with the constant
 byte value value.

 hipMemsetAsync() is asynchronous with respect to the host, so the call may return before the
 memset is complete. The operation can optionally be associated to a stream by passing a non-zero
 stream argument. If stream is non-zero, the operation may overlap with operations in other
 streams.

  @param[out] dst Pointer to device memory
  @param[in]  value  Value to set for each byte of specified memory
  @param[in]  sizeBytes  Size in bytes to set
  @param[in]  stream  Stream identifier
  @return #hipSuccess, #hipErrorInvalidValue*/
    pub fn hipMemsetAsync(
        dst: *mut ::core::ffi::c_void,
        value: ::core::ffi::c_int,
        sizeBytes: usize,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Fills the memory area pointed to by dev with the constant integer
 value for specified number of times.

  hipMemsetD32Async() is asynchronous with respect to the host, so the call may return before the
 memset is complete. The operation can optionally be associated to a stream by passing a non-zero
 stream argument. If stream is non-zero, the operation may overlap with operations in other
 streams.

  @param[out] dst Pointer to device memory
  @param[in]  value  Value to set for each byte of specified memory
  @param[in]  count  Number of values to be set
  @param[in]  stream  Stream identifier
  @return #hipSuccess, #hipErrorInvalidValue*/
    pub fn hipMemsetD32Async(
        dst: hipDeviceptr_t,
        value: ::core::ffi::c_int,
        count: usize,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Fills the memory area pointed to by dst with the constant value.

  @param[out] dst Pointer to device memory
  @param[in]  pitch  Data size in bytes
  @param[in]  value  Constant value to be set
  @param[in]  width
  @param[in]  height
  @returns #hipSuccess, #hipErrorInvalidValue*/
    pub fn hipMemset2D(
        dst: *mut ::core::ffi::c_void,
        pitch: usize,
        value: ::core::ffi::c_int,
        width: usize,
        height: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Fills asynchronously the memory area pointed to by dst with the constant value.

  @param[in]  dst Pointer to 2D device memory
  @param[in]  pitch  Pitch size in bytes
  @param[in]  value  Value to be set for each byte of specified memory
  @param[in]  width  Width of matrix set columns in bytes
  @param[in]  height  Height of matrix set rows in bytes
  @param[in]  stream  Stream identifier
  @returns #hipSuccess, #hipErrorInvalidValue*/
    pub fn hipMemset2DAsync(
        dst: *mut ::core::ffi::c_void,
        pitch: usize,
        value: ::core::ffi::c_int,
        width: usize,
        height: usize,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Fills synchronously the memory area pointed to by pitchedDevPtr with the constant value.

  @param[in] pitchedDevPtr  Pointer to pitched device memory
  @param[in]  value  Value to set for each byte of specified memory
  @param[in]  extent  Size parameters for width field in bytes in device memory
  @returns #hipSuccess, #hipErrorInvalidValue*/
    pub fn hipMemset3D(
        pitchedDevPtr: hipPitchedPtr,
        value: ::core::ffi::c_int,
        extent: hipExtent,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Fills asynchronously the memory area pointed to by pitchedDevPtr with the constant value.

  @param[in] pitchedDevPtr  Pointer to pitched device memory
  @param[in]  value  Value to set for each byte of specified memory
  @param[in]  extent  Size parameters for width field in bytes in device memory
  @param[in]  stream  Stream identifier
  @returns #hipSuccess, #hipErrorInvalidValue*/
    pub fn hipMemset3DAsync(
        pitchedDevPtr: hipPitchedPtr,
        value: ::core::ffi::c_int,
        extent: hipExtent,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Query memory info.

 On ROCM, this function gets the actual free memory left on the current device, so supports
 the cases while running multi-workload (such as multiple processes, multiple threads, and
 multiple GPUs).

 @warning On Windows, the free memory only accounts for memory allocated by this process and may
 be optimistic.

 @param[out] free Returns free memory on the current device in bytes
 @param[out] total Returns total allocatable memory on the current device in bytes

 @returns #hipSuccess, #hipErrorInvalidDevice, #hipErrorInvalidValue
*/
    pub fn hipMemGetInfo(free: *mut usize, total: *mut usize) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get allocated memory size via memory pointer.

 This function gets the allocated shared virtual memory size from memory pointer.

 @param[in] ptr Pointer to allocated memory
 @param[out] size Returns the allocated memory size in bytes

 @returns #hipSuccess, #hipErrorInvalidValue
*/
    pub fn hipMemPtrGetInfo(
        ptr: *mut ::core::ffi::c_void,
        size: *mut usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Allocate an array on the device.

  @param[out]  array  Pointer to allocated array in device memory
  @param[in]   desc   Requested channel format
  @param[in]   width  Requested array allocation width
  @param[in]   height Requested array allocation height
  @param[in]   flags  Requested properties of allocated array
  @returns     #hipSuccess, #hipErrorOutOfMemory

  @see hipMalloc, hipMallocPitch, hipFree, hipFreeArray, hipHostMalloc, hipHostFree*/
    pub fn hipMallocArray(
        array: *mut hipArray_t,
        desc: *const hipChannelFormatDesc,
        width: usize,
        height: usize,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Create an array memory pointer on the device.

  @param[out]  pHandle  Pointer to the array memory
  @param[in]   pAllocateArray   Requested array desciptor

  @returns     #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported

  @see hipMallocArray, hipArrayDestroy, hipFreeArray*/
    pub fn hipArrayCreate(
        pHandle: *mut hipArray_t,
        pAllocateArray: *const HIP_ARRAY_DESCRIPTOR,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Destroy an array memory pointer on the device.

  @param[in]  array  Pointer to the array memory

  @returns     #hipSuccess, #hipErrorInvalidValue

  @see hipArrayCreate, hipArrayDestroy, hipFreeArray*/
    pub fn hipArrayDestroy(array: hipArray_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Create a 3D array memory pointer on the device.

  @param[out]  array  Pointer to the 3D array memory
  @param[in]   pAllocateArray   Requested array desciptor

  @returns     #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported

  @see hipMallocArray, hipArrayDestroy, hipFreeArray*/
    pub fn hipArray3DCreate(
        array: *mut hipArray_t,
        pAllocateArray: *const HIP_ARRAY3D_DESCRIPTOR,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Create a 3D memory pointer on the device.

  @param[out]  pitchedDevPtr  Pointer to the 3D memory
  @param[in]   extent   Requested extent

  @returns     #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported

  @see hipMallocPitch, hipMemGetInfo, hipFree*/
    pub fn hipMalloc3D(
        pitchedDevPtr: *mut hipPitchedPtr,
        extent: hipExtent,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Frees an array on the device.

  @param[in]  array  Pointer to array to free
  @returns    #hipSuccess, #hipErrorInvalidValue, #hipErrorNotInitialized

  @see hipMalloc, hipMallocPitch, hipFree, hipMallocArray, hipHostMalloc, hipHostFree*/
    pub fn hipFreeArray(array: hipArray_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Allocate an array on the device.

  @param[out]  array  Pointer to allocated array in device memory
  @param[in]   desc   Requested channel format
  @param[in]   extent Requested array allocation width, height and depth
  @param[in]   flags  Requested properties of allocated array
  @returns     #hipSuccess, #hipErrorOutOfMemory

  @see hipMalloc, hipMallocPitch, hipFree, hipFreeArray, hipHostMalloc, hipHostFree*/
    pub fn hipMalloc3DArray(
        array: *mut hipArray_t,
        desc: *const hipChannelFormatDesc,
        extent: hipExtent,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets info about the specified array

 @param[out] desc   - Returned array type
 @param[out] extent - Returned array shape. 2D arrays will have depth of zero
 @param[out] flags  - Returned array flags
 @param[in]  array  - The HIP array to get info for

 @returns #hipSuccess, #hipErrorInvalidValue #hipErrorInvalidHandle

 @see hipArrayGetDescriptor, hipArray3DGetDescriptor*/
    pub fn hipArrayGetInfo(
        desc: *mut hipChannelFormatDesc,
        extent: *mut hipExtent,
        flags: *mut ::core::ffi::c_uint,
        array: hipArray_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets a 1D or 2D array descriptor

 @param[out] pArrayDescriptor - Returned array descriptor
 @param[in]  array            - Array to get descriptor of

 @returns #hipSuccess, #hipErrorDeinitialized, #hipErrorNotInitialized, #hipErrorInvalidContext,
 #hipErrorInvalidValue #hipErrorInvalidHandle

 @see hipArray3DCreate, hipArray3DGetDescriptor, hipArrayCreate, hipArrayDestroy, hipMemAlloc,
 hipMemAllocHost, hipMemAllocPitch, hipMemcpy2D, hipMemcpy2DAsync, hipMemcpy2DUnaligned,
 hipMemcpy3D, hipMemcpy3DAsync, hipMemcpyAtoA, hipMemcpyAtoD, hipMemcpyAtoH, hipMemcpyAtoHAsync,
 hipMemcpyDtoA, hipMemcpyDtoD, hipMemcpyDtoDAsync, hipMemcpyDtoH, hipMemcpyDtoHAsync,
 hipMemcpyHtoA, hipMemcpyHtoAAsync, hipMemcpyHtoD, hipMemcpyHtoDAsync, hipMemFree,
 hipMemFreeHost, hipMemGetAddressRange, hipMemGetInfo, hipMemHostAlloc,
 hipMemHostGetDevicePointer, hipMemsetD8, hipMemsetD16, hipMemsetD32, hipArrayGetInfo*/
    pub fn hipArrayGetDescriptor(
        pArrayDescriptor: *mut HIP_ARRAY_DESCRIPTOR,
        array: hipArray_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets a 3D array descriptor

 @param[out] pArrayDescriptor - Returned 3D array descriptor
 @param[in]  array            - 3D array to get descriptor of

 @returns #hipSuccess, #hipErrorDeinitialized, #hipErrorNotInitialized, #hipErrorInvalidContext,
 #hipErrorInvalidValue #hipErrorInvalidHandle, #hipErrorContextIsDestroyed

 @see hipArray3DCreate, hipArrayCreate, hipArrayDestroy, hipArrayGetDescriptor, hipMemAlloc,
 hipMemAllocHost, hipMemAllocPitch, hipMemcpy2D, hipMemcpy2DAsync, hipMemcpy2DUnaligned,
 hipMemcpy3D, hipMemcpy3DAsync, hipMemcpyAtoA, hipMemcpyAtoD, hipMemcpyAtoH, hipMemcpyAtoHAsync,
 hipMemcpyDtoA, hipMemcpyDtoD, hipMemcpyDtoDAsync, hipMemcpyDtoH, hipMemcpyDtoHAsync,
 hipMemcpyHtoA, hipMemcpyHtoAAsync, hipMemcpyHtoD, hipMemcpyHtoDAsync, hipMemFree,
 hipMemFreeHost, hipMemGetAddressRange, hipMemGetInfo, hipMemHostAlloc,
 hipMemHostGetDevicePointer, hipMemsetD8, hipMemsetD16, hipMemsetD32, hipArrayGetInfo*/
    pub fn hipArray3DGetDescriptor(
        pArrayDescriptor: *mut HIP_ARRAY3D_DESCRIPTOR,
        array: hipArray_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copies data between host and device.

  @param[in]   dst    Destination memory address
  @param[in]   dpitch Pitch of destination memory
  @param[in]   src    Source memory address
  @param[in]   spitch Pitch of source memory
  @param[in]   width  Width of matrix transfer (columns in bytes)
  @param[in]   height Height of matrix transfer (rows)
  @param[in]   kind   Type of transfer
  @returns     #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidPitchValue,
 #hipErrorInvalidDevicePointer, #hipErrorInvalidMemcpyDirection

  @see hipMemcpy, hipMemcpyToArray, hipMemcpy2DToArray, hipMemcpyFromArray, hipMemcpyToSymbol,
 hipMemcpyAsync*/
    pub fn hipMemcpy2D(
        dst: *mut ::core::ffi::c_void,
        dpitch: usize,
        src: *const ::core::ffi::c_void,
        spitch: usize,
        width: usize,
        height: usize,
        kind: hipMemcpyKind,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copies memory for 2D arrays.
  @param[in]   pCopy Parameters for the memory copy
  @returns     #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidPitchValue,
  #hipErrorInvalidDevicePointer, #hipErrorInvalidMemcpyDirection

  @see hipMemcpy, hipMemcpy2D, hipMemcpyToArray, hipMemcpy2DToArray, hipMemcpyFromArray,
 hipMemcpyToSymbol, hipMemcpyAsync*/
    pub fn hipMemcpyParam2D(pCopy: *const hip_Memcpy2D) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copies memory for 2D arrays.
  @param[in]   pCopy Parameters for the memory copy
  @param[in]   stream Stream to use
  @returns     #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidPitchValue,
 #hipErrorInvalidDevicePointer, #hipErrorInvalidMemcpyDirection

  @see hipMemcpy, hipMemcpy2D, hipMemcpyToArray, hipMemcpy2DToArray, hipMemcpyFromArray,
 hipMemcpyToSymbol, hipMemcpyAsync*/
    pub fn hipMemcpyParam2DAsync(
        pCopy: *const hip_Memcpy2D,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copies data between host and device.

  @param[in]   dst    Destination memory address
  @param[in]   dpitch Pitch of destination memory
  @param[in]   src    Source memory address
  @param[in]   spitch Pitch of source memory
  @param[in]   width  Width of matrix transfer (columns in bytes)
  @param[in]   height Height of matrix transfer (rows)
  @param[in]   kind   Type of transfer
  @param[in]   stream Stream to use
  @returns     #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidPitchValue,
 #hipErrorInvalidDevicePointer, #hipErrorInvalidMemcpyDirection

  @see hipMemcpy, hipMemcpyToArray, hipMemcpy2DToArray, hipMemcpyFromArray, hipMemcpyToSymbol,
 hipMemcpyAsync*/
    pub fn hipMemcpy2DAsync(
        dst: *mut ::core::ffi::c_void,
        dpitch: usize,
        src: *const ::core::ffi::c_void,
        spitch: usize,
        width: usize,
        height: usize,
        kind: hipMemcpyKind,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copies data between host and device.

  @param[in]   dst     Destination memory address
  @param[in]   wOffset Destination starting X offset
  @param[in]   hOffset Destination starting Y offset
  @param[in]   src     Source memory address
  @param[in]   spitch  Pitch of source memory
  @param[in]   width   Width of matrix transfer (columns in bytes)
  @param[in]   height  Height of matrix transfer (rows)
  @param[in]   kind    Type of transfer
  @returns     #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidPitchValue,
 #hipErrorInvalidDevicePointer, #hipErrorInvalidMemcpyDirection

  @see hipMemcpy, hipMemcpyToArray, hipMemcpy2D, hipMemcpyFromArray, hipMemcpyToSymbol,
 hipMemcpyAsync*/
    pub fn hipMemcpy2DToArray(
        dst: hipArray_t,
        wOffset: usize,
        hOffset: usize,
        src: *const ::core::ffi::c_void,
        spitch: usize,
        width: usize,
        height: usize,
        kind: hipMemcpyKind,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copies data between host and device.

  @param[in]   dst     Destination memory address
  @param[in]   wOffset Destination starting X offset
  @param[in]   hOffset Destination starting Y offset
  @param[in]   src     Source memory address
  @param[in]   spitch  Pitch of source memory
  @param[in]   width   Width of matrix transfer (columns in bytes)
  @param[in]   height  Height of matrix transfer (rows)
  @param[in]   kind    Type of transfer
  @param[in]   stream    Accelerator view which the copy is being enqueued
  @returns     #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidPitchValue,
 #hipErrorInvalidDevicePointer, #hipErrorInvalidMemcpyDirection

  @see hipMemcpy, hipMemcpyToArray, hipMemcpy2D, hipMemcpyFromArray, hipMemcpyToSymbol,
 hipMemcpyAsync*/
    pub fn hipMemcpy2DToArrayAsync(
        dst: hipArray_t,
        wOffset: usize,
        hOffset: usize,
        src: *const ::core::ffi::c_void,
        spitch: usize,
        width: usize,
        height: usize,
        kind: hipMemcpyKind,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copies data between host and device.

  @param[in]   dst Destination memory address
  @param[in]   wOffsetDst Destination starting X offset
  @param[in]   hOffsetDst Destination starting Y offset
  @param[in]   src  Source memory address
  @param[in]   wOffsetSrc Source starting X offset
  @param[in]   hOffsetSrc Source starting Y offset (columns in bytes)
  @param[in]   width  Width of matrix transfer (columns in bytes)
  @param[in]   height  Height of matrix transfer (rows)
  @param[in]   kind Type of transfer

  @returns     #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidMemcpyDirection

  @see hipMemcpy, hipMemcpyToArray, hipMemcpy2D, hipMemcpyFromArray, hipMemcpyToSymbol,
 hipMemcpyAsync*/
    pub fn hipMemcpy2DArrayToArray(
        dst: hipArray_t,
        wOffsetDst: usize,
        hOffsetDst: usize,
        src: hipArray_const_t,
        wOffsetSrc: usize,
        hOffsetSrc: usize,
        width: usize,
        height: usize,
        kind: hipMemcpyKind,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copies data between host and device [Deprecated]

  @ingroup MemoryD

  @param[in]   dst     Destination memory address
  @param[in]   wOffset Destination starting X offset
  @param[in]   hOffset Destination starting Y offset
  @param[in]   src     Source memory address
  @param[in]   count   size in bytes to copy
  @param[in]   kind    Type of transfer
  @returns     #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidPitchValue,
 #hipErrorInvalidDevicePointer, #hipErrorInvalidMemcpyDirection

  @see hipMemcpy, hipMemcpy2DToArray, hipMemcpy2D, hipMemcpyFromArray, hipMemcpyToSymbol,
  hipMemcpyAsync
  @warning  This API is deprecated.*/
    pub fn hipMemcpyToArray(
        dst: hipArray_t,
        wOffset: usize,
        hOffset: usize,
        src: *const ::core::ffi::c_void,
        count: usize,
        kind: hipMemcpyKind,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copies data between host and device [Deprecated]

  @ingroup MemoryD

  @param[in]   dst       Destination memory address
  @param[in]   srcArray  Source memory address
  @param[in]   wOffset   Source starting X offset
  @param[in]   hOffset   Source starting Y offset
  @param[in]   count     Size in bytes to copy
  @param[in]   kind      Type of transfer
  @returns     #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidPitchValue,
 #hipErrorInvalidDevicePointer, #hipErrorInvalidMemcpyDirection

  @see hipMemcpy, hipMemcpy2DToArray, hipMemcpy2D, hipMemcpyFromArray, hipMemcpyToSymbol,
 hipMemcpyAsync
 @warning  This API is deprecated.*/
    pub fn hipMemcpyFromArray(
        dst: *mut ::core::ffi::c_void,
        srcArray: hipArray_const_t,
        wOffset: usize,
        hOffset: usize,
        count: usize,
        kind: hipMemcpyKind,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copies data between host and device.

  @param[in]   dst       Destination memory address
  @param[in]   dpitch    Pitch of destination memory
  @param[in]   src       Source memory address
  @param[in]   wOffset   Source starting X offset
  @param[in]   hOffset   Source starting Y offset
  @param[in]   width     Width of matrix transfer (columns in bytes)
  @param[in]   height    Height of matrix transfer (rows)
  @param[in]   kind      Type of transfer
  @returns     #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidPitchValue,
 #hipErrorInvalidDevicePointer, #hipErrorInvalidMemcpyDirection

  @see hipMemcpy, hipMemcpy2DToArray, hipMemcpy2D, hipMemcpyFromArray, hipMemcpyToSymbol,
 hipMemcpyAsync*/
    pub fn hipMemcpy2DFromArray(
        dst: *mut ::core::ffi::c_void,
        dpitch: usize,
        src: hipArray_const_t,
        wOffset: usize,
        hOffset: usize,
        width: usize,
        height: usize,
        kind: hipMemcpyKind,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copies data between host and device asynchronously.

  @param[in]   dst       Destination memory address
  @param[in]   dpitch    Pitch of destination memory
  @param[in]   src       Source memory address
  @param[in]   wOffset   Source starting X offset
  @param[in]   hOffset   Source starting Y offset
  @param[in]   width     Width of matrix transfer (columns in bytes)
  @param[in]   height    Height of matrix transfer (rows)
  @param[in]   kind      Type of transfer
  @param[in]   stream    Accelerator view which the copy is being enqueued
  @returns     #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidPitchValue,
 #hipErrorInvalidDevicePointer, #hipErrorInvalidMemcpyDirection

  @see hipMemcpy, hipMemcpy2DToArray, hipMemcpy2D, hipMemcpyFromArray, hipMemcpyToSymbol,
 hipMemcpyAsync*/
    pub fn hipMemcpy2DFromArrayAsync(
        dst: *mut ::core::ffi::c_void,
        dpitch: usize,
        src: hipArray_const_t,
        wOffset: usize,
        hOffset: usize,
        width: usize,
        height: usize,
        kind: hipMemcpyKind,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copies data between host and device.

  @param[in]   dst       Destination memory address
  @param[in]   srcArray  Source array
  @param[in]   srcOffset Offset in bytes of source array
  @param[in]   count     Size of memory copy in bytes
  @returns     #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidPitchValue,
 #hipErrorInvalidDevicePointer, #hipErrorInvalidMemcpyDirection

  @see hipMemcpy, hipMemcpy2DToArray, hipMemcpy2D, hipMemcpyFromArray, hipMemcpyToSymbol,
 hipMemcpyAsync*/
    pub fn hipMemcpyAtoH(
        dst: *mut ::core::ffi::c_void,
        srcArray: hipArray_t,
        srcOffset: usize,
        count: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copies data between host and device.

  @param[in]   dstArray   Destination memory address
  @param[in]   dstOffset  Offset in bytes of destination array
  @param[in]   srcHost    Source host pointer
  @param[in]   count      Size of memory copy in bytes
  @returns     #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidPitchValue,
 #hipErrorInvalidDevicePointer, #hipErrorInvalidMemcpyDirection

  @see hipMemcpy, hipMemcpy2DToArray, hipMemcpy2D, hipMemcpyFromArray, hipMemcpyToSymbol,
 hipMemcpyAsync*/
    pub fn hipMemcpyHtoA(
        dstArray: hipArray_t,
        dstOffset: usize,
        srcHost: *const ::core::ffi::c_void,
        count: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copies data between host and device.

  @param[in]   p   3D memory copy parameters
  @returns     #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidPitchValue,
 #hipErrorInvalidDevicePointer, #hipErrorInvalidMemcpyDirection

  @see hipMemcpy, hipMemcpy2DToArray, hipMemcpy2D, hipMemcpyFromArray, hipMemcpyToSymbol,
 hipMemcpyAsync*/
    pub fn hipMemcpy3D(p: *const hipMemcpy3DParms) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copies data between host and device asynchronously.

  @param[in]   p        3D memory copy parameters
  @param[in]   stream   Stream to use
  @returns     #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidPitchValue,
 #hipErrorInvalidDevicePointer, #hipErrorInvalidMemcpyDirection

  @see hipMemcpy, hipMemcpy2DToArray, hipMemcpy2D, hipMemcpyFromArray, hipMemcpyToSymbol,
 hipMemcpyAsync*/
    pub fn hipMemcpy3DAsync(
        p: *const hipMemcpy3DParms,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copies data between host and device.

  @param[in]   pCopy   3D memory copy parameters
  @returns     #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidPitchValue,
  #hipErrorInvalidDevicePointer, #hipErrorInvalidMemcpyDirection

  @see hipMemcpy, hipMemcpy2DToArray, hipMemcpy2D, hipMemcpyFromArray, hipMemcpyToSymbol,
 hipMemcpyAsync*/
    pub fn hipDrvMemcpy3D(pCopy: *const HIP_MEMCPY3D) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /**  @brief Copies data between host and device asynchronously.

  @param[in]   pCopy    3D memory copy parameters
  @param[in]   stream   Stream to use
  @returns     #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidPitchValue,
  #hipErrorInvalidDevicePointer, #hipErrorInvalidMemcpyDirection

  @see hipMemcpy, hipMemcpy2DToArray, hipMemcpy2D, hipMemcpyFromArray, hipMemcpyToSymbol,
 hipMemcpyAsync*/
    pub fn hipDrvMemcpy3DAsync(
        pCopy: *const HIP_MEMCPY3D,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " @}\n/\n/**\n-------------------------------------------------------------------------------------------------\n-------------------------------------------------------------------------------------------------\n  @defgroup PeerToPeer PeerToPeer Device Memory Access\n  @{\n  @warning PeerToPeer support is experimental.\n  This section describes the PeerToPeer device memory access functions of HIP runtime API.\n/\n/**\n @brief Determine if a device can access a peer's memory.\n\n @param [out] canAccessPeer Returns the peer access capability (0 or 1)\n @param [in] deviceId - device from where memory may be accessed.\n @param [in] peerDeviceId - device where memory is physically located\n\n Returns \"1\" in @p canAccessPeer if the specified @p device is capable\n of directly accessing memory physically located on peerDevice , or \"0\" if not.\n\n Returns \"0\" in @p canAccessPeer if deviceId == peerDeviceId, and both are valid devices : a\n device is not a peer of itself.\n\n @returns #hipSuccess,\n @returns #hipErrorInvalidDevice if deviceId or peerDeviceId are not valid devices"]
    pub fn hipDeviceCanAccessPeer(
        canAccessPeer: *mut ::core::ffi::c_int,
        deviceId: ::core::ffi::c_int,
        peerDeviceId: ::core::ffi::c_int,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Enable direct access from current device's virtual address space to memory allocations
 physically located on a peer device.

 Memory which already allocated on peer device will be mapped into the address space of the
 current device.  In addition, all future memory allocations on peerDeviceId will be mapped into
 the address space of the current device when the memory is allocated. The peer memory remains
 accessible from the current device until a call to hipDeviceDisablePeerAccess or hipDeviceReset.


 @param [in] peerDeviceId  Peer device to enable direct access to from the current device
 @param [in] flags  Reserved for future use, must be zero

 Returns #hipSuccess, #hipErrorInvalidDevice, #hipErrorInvalidValue,
 @returns #hipErrorPeerAccessAlreadyEnabled if peer access is already enabled for this device.*/
    pub fn hipDeviceEnablePeerAccess(
        peerDeviceId: ::core::ffi::c_int,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Disable direct access from current device's virtual address space to memory allocations
 physically located on a peer device.

 Returns hipErrorPeerAccessNotEnabled if direct access to memory on peerDevice has not yet been
 enabled from the current device.

 @param [in] peerDeviceId  Peer device to disable direct access to

 @returns #hipSuccess, #hipErrorPeerAccessNotEnabled*/
    pub fn hipDeviceDisablePeerAccess(peerDeviceId: ::core::ffi::c_int) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get information on memory allocations.

 @param [out] pbase - BAse pointer address
 @param [out] psize - Size of allocation
 @param [in]  dptr- Device Pointer

 @returns #hipSuccess, #hipErrorNotFound

 @see hipCtxCreate, hipCtxDestroy, hipCtxGetFlags, hipCtxPopCurrent, hipCtxGetCurrent,
 hipCtxSetCurrent, hipCtxPushCurrent, hipCtxSetCacheConfig, hipCtxSynchronize, hipCtxGetDevice*/
    pub fn hipMemGetAddressRange(
        pbase: *mut hipDeviceptr_t,
        psize: *mut usize,
        dptr: hipDeviceptr_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Copies memory from one device to memory on another device.

 @param [out] dst - Destination device pointer.
 @param [in] dstDeviceId - Destination device
 @param [in] src - Source device pointer
 @param [in] srcDeviceId - Source device
 @param [in] sizeBytes - Size of memory copy in bytes

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidDevice*/
    pub fn hipMemcpyPeer(
        dst: *mut ::core::ffi::c_void,
        dstDeviceId: ::core::ffi::c_int,
        src: *const ::core::ffi::c_void,
        srcDeviceId: ::core::ffi::c_int,
        sizeBytes: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Copies memory from one device to memory on another device.

 @param [out] dst - Destination device pointer.
 @param [in] dstDeviceId - Destination device
 @param [in] src - Source device pointer
 @param [in] srcDevice - Source device
 @param [in] sizeBytes - Size of memory copy in bytes
 @param [in] stream - Stream identifier

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidDevice*/
    pub fn hipMemcpyPeerAsync(
        dst: *mut ::core::ffi::c_void,
        dstDeviceId: ::core::ffi::c_int,
        src: *const ::core::ffi::c_void,
        srcDevice: ::core::ffi::c_int,
        sizeBytes: usize,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Create a context and set it as current/default context

 @param [out] ctx  Context to create
 @param [in] flags  Context creation flags
 @param [in] device  device handle

 @returns #hipSuccess

 @see hipCtxDestroy, hipCtxGetFlags, hipCtxPopCurrent, hipCtxGetCurrent, hipCtxPushCurrent,
 hipCtxSetCacheConfig, hipCtxSynchronize, hipCtxGetDevice

 @warning  This API is deprecated on the AMD platform, only for equivalent cuCtx driver API on the
 NVIDIA platform.
*/
    pub fn hipCtxCreate(
        ctx: *mut hipCtx_t,
        flags: ::core::ffi::c_uint,
        device: hipDevice_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Destroy a HIP context [Deprecated]

 @param [in] ctx Context to destroy

 @returns #hipSuccess, #hipErrorInvalidValue

 @see hipCtxCreate, hipCtxGetFlags, hipCtxPopCurrent, hipCtxGetCurrent,hipCtxSetCurrent,
 hipCtxPushCurrent, hipCtxSetCacheConfig, hipCtxSynchronize , hipCtxGetDevice

 @warning  This API is deprecated on the AMD platform, only for equivalent cuCtx driver API on the
 NVIDIA platform.*/
    pub fn hipCtxDestroy(ctx: hipCtx_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Pop the current/default context and return the popped context [Deprecated]

 @param [out] ctx  The current context to pop

 @returns #hipSuccess, #hipErrorInvalidContext

 @see hipCtxCreate, hipCtxDestroy, hipCtxGetFlags, hipCtxSetCurrent, hipCtxGetCurrent,
 hipCtxPushCurrent, hipCtxSetCacheConfig, hipCtxSynchronize, hipCtxGetDevice

 @warning  This API is deprecated on the AMD platform, only for equivalent cuCtx driver API on the
 NVIDIA platform.*/
    pub fn hipCtxPopCurrent(ctx: *mut hipCtx_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Push the context to be set as current/ default context [Deprecated]

 @param [in] ctx  The current context to push

 @returns #hipSuccess, #hipErrorInvalidContext

 @see hipCtxCreate, hipCtxDestroy, hipCtxGetFlags, hipCtxPopCurrent, hipCtxGetCurrent,
 hipCtxPushCurrent, hipCtxSetCacheConfig, hipCtxSynchronize , hipCtxGetDevice

 @warning  This API is deprecated on the AMD platform, only for equivalent cuCtx driver API on the
 NVIDIA platform.*/
    pub fn hipCtxPushCurrent(ctx: hipCtx_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set the passed context as current/default [Deprecated]

 @param [in] ctx The context to set as current

 @returns #hipSuccess, #hipErrorInvalidContext

 @see hipCtxCreate, hipCtxDestroy, hipCtxGetFlags, hipCtxPopCurrent, hipCtxGetCurrent,
 hipCtxPushCurrent, hipCtxSetCacheConfig, hipCtxSynchronize , hipCtxGetDevice

 @warning  This API is deprecated on the AMD platform, only for equivalent cuCtx driver API on the
 NVIDIA platform.*/
    pub fn hipCtxSetCurrent(ctx: hipCtx_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get the handle of the current/ default context [Deprecated]

 @param [out] ctx  The context to get as current

 @returns #hipSuccess, #hipErrorInvalidContext

 @see hipCtxCreate, hipCtxDestroy, hipCtxGetDevice, hipCtxGetFlags, hipCtxPopCurrent,
 hipCtxPushCurrent, hipCtxSetCacheConfig, hipCtxSynchronize, hipCtxGetDevice

 @warning  This API is deprecated on the AMD platform, only for equivalent cuCtx driver API on the
 NVIDIA platform.*/
    pub fn hipCtxGetCurrent(ctx: *mut hipCtx_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get the handle of the device associated with current/default context [Deprecated]

 @param [out] device The device from the current context

 @returns #hipSuccess, #hipErrorInvalidContext

 @see hipCtxCreate, hipCtxDestroy, hipCtxGetFlags, hipCtxPopCurrent, hipCtxGetCurrent,
 hipCtxPushCurrent, hipCtxSetCacheConfig, hipCtxSynchronize

 @warning  This API is deprecated on the AMD platform, only for equivalent cuCtx driver API on the
 NVIDIA platform.*/
    pub fn hipCtxGetDevice(device: *mut hipDevice_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns the approximate HIP api version.

 @param [in]  ctx Context to check [Deprecated]
 @param [out] apiVersion API version to get

 @returns #hipSuccess

 @warning The HIP feature set does not correspond to an exact CUDA SDK api revision.
 This function always set *apiVersion to 4 as an approximation though HIP supports
 some features which were introduced in later CUDA SDK revisions.
 HIP apps code should not rely on the api revision number here and should
 use arch feature flags to test device capabilities or conditional compilation.

 @see hipCtxCreate, hipCtxDestroy, hipCtxGetDevice, hipCtxGetFlags, hipCtxPopCurrent,
 hipCtxPushCurrent, hipCtxSetCacheConfig, hipCtxSynchronize, hipCtxGetDevice

 @warning  This API is deprecated on the AMD platform, only for equivalent cuCtx driver API on the
 NVIDIA platform.*/
    pub fn hipCtxGetApiVersion(
        ctx: hipCtx_t,
        apiVersion: *mut ::core::ffi::c_int,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get Cache configuration for a specific function [Deprecated]

 @param [out] cacheConfig  Cache configuration

 @returns #hipSuccess

 @warning AMD devices and some Nvidia GPUS do not support reconfigurable cache.  This hint is
 ignored on those architectures.

 @see hipCtxCreate, hipCtxDestroy, hipCtxGetFlags, hipCtxPopCurrent, hipCtxGetCurrent,
 hipCtxSetCurrent, hipCtxPushCurrent, hipCtxSetCacheConfig, hipCtxSynchronize, hipCtxGetDevice

 @warning  This API is deprecated on the AMD platform, only for equivalent cuCtx driver API on the
 NVIDIA platform.*/
    pub fn hipCtxGetCacheConfig(cacheConfig: *mut hipFuncCache_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set L1/Shared cache partition [Deprecated]

 @param [in] cacheConfig  Cache configuration to set

 @return #hipSuccess

 @warning AMD devices and some Nvidia GPUS do not support reconfigurable cache.  This hint is
 ignored on those architectures.

 @see hipCtxCreate, hipCtxDestroy, hipCtxGetFlags, hipCtxPopCurrent, hipCtxGetCurrent,
 hipCtxSetCurrent, hipCtxPushCurrent, hipCtxSetCacheConfig, hipCtxSynchronize, hipCtxGetDevice

 @warning  This API is deprecated on the AMD platform, only for equivalent cuCtx driver API on the
 NVIDIA platform.*/
    pub fn hipCtxSetCacheConfig(cacheConfig: hipFuncCache_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set Shared memory bank configuration  [Deprecated]

 @param [in] config  Shared memory configuration to set

 @return #hipSuccess

 @warning AMD devices and some Nvidia GPUS do not support shared cache banking, and the hint is
 ignored on those architectures.

 @see hipCtxCreate, hipCtxDestroy, hipCtxGetFlags, hipCtxPopCurrent, hipCtxGetCurrent,
 hipCtxSetCurrent, hipCtxPushCurrent, hipCtxSetCacheConfig, hipCtxSynchronize, hipCtxGetDevice

 @warning  This API is deprecated on the AMD platform, only for equivalent cuCtx driver API on the
 NVIDIA platform.*/
    pub fn hipCtxSetSharedMemConfig(config: hipSharedMemConfig) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get Shared memory bank configuration [Deprecated]

 @param [out] pConfig  Pointer of shared memory configuration

 @return #hipSuccess

 @warning AMD devices and some Nvidia GPUS do not support shared cache banking, and the hint is
 ignored on those architectures.

 @see hipCtxCreate, hipCtxDestroy, hipCtxGetFlags, hipCtxPopCurrent, hipCtxGetCurrent,
 hipCtxSetCurrent, hipCtxPushCurrent, hipCtxSetCacheConfig, hipCtxSynchronize, hipCtxGetDevice

 @warning  This API is deprecated on the AMD platform, only for equivalent cuCtx driver API on the
 NVIDIA platform.*/
    pub fn hipCtxGetSharedMemConfig(pConfig: *mut hipSharedMemConfig) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Blocks until the default context has completed all preceding requested tasks [Deprecated]

 @return #hipSuccess

 @warning This function waits for all streams on the default context to complete execution, and
 then returns.

 @see hipCtxCreate, hipCtxDestroy, hipCtxGetFlags, hipCtxPopCurrent, hipCtxGetCurrent,
 hipCtxSetCurrent, hipCtxPushCurrent, hipCtxSetCacheConfig, hipCtxGetDevice

 @warning  This API is deprecated on the AMD platform, only for equivalent cuCtx driver API on the
 NVIDIA platform.*/
    pub fn hipCtxSynchronize() -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Return flags used for creating default context [Deprecated]

 @param [out] flags  Pointer of flags

 @returns #hipSuccess

 @see hipCtxCreate, hipCtxDestroy, hipCtxPopCurrent, hipCtxGetCurrent, hipCtxGetCurrent,
 hipCtxSetCurrent, hipCtxPushCurrent, hipCtxSetCacheConfig, hipCtxSynchronize, hipCtxGetDevice

 @warning  This API is deprecated on the AMD platform, only for equivalent cuCtx driver API on the
 NVIDIA platform.*/
    pub fn hipCtxGetFlags(flags: *mut ::core::ffi::c_uint) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Enables direct access to memory allocations in a peer context [Deprecated]

 Memory which already allocated on peer device will be mapped into the address space of the
 current device.  In addition, all future memory allocations on peerDeviceId will be mapped into
 the address space of the current device when the memory is allocated. The peer memory remains
 accessible from the current device until a call to hipDeviceDisablePeerAccess or hipDeviceReset.


 @param [in] peerCtx  Peer context
 @param [in] flags  flags, need to set as 0

 @returns #hipSuccess, #hipErrorInvalidDevice, #hipErrorInvalidValue,
 #hipErrorPeerAccessAlreadyEnabled

 @see hipCtxCreate, hipCtxDestroy, hipCtxGetFlags, hipCtxPopCurrent, hipCtxGetCurrent,
 hipCtxSetCurrent, hipCtxPushCurrent, hipCtxSetCacheConfig, hipCtxSynchronize, hipCtxGetDevice
 @warning PeerToPeer support is experimental.

 @warning  This API is deprecated on the AMD platform, only for equivalent cuCtx driver API on the
 NVIDIA platform.*/
    pub fn hipCtxEnablePeerAccess(
        peerCtx: hipCtx_t,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Disable direct access from current context's virtual address space to memory allocations
 physically located on a peer context.Disables direct access to memory allocations in a peer
 context and unregisters any registered allocations [Deprecated]

 Returns #hipErrorPeerAccessNotEnabled if direct access to memory on peerDevice has not yet been
 enabled from the current device.

 @param [in] peerCtx  Peer context to be disabled

 @returns #hipSuccess, #hipErrorPeerAccessNotEnabled

 @see hipCtxCreate, hipCtxDestroy, hipCtxGetFlags, hipCtxPopCurrent, hipCtxGetCurrent,
 hipCtxSetCurrent, hipCtxPushCurrent, hipCtxSetCacheConfig, hipCtxSynchronize, hipCtxGetDevice
 @warning PeerToPeer support is experimental.

 @warning  This API is deprecated on the AMD platform, only for equivalent cuCtx driver API on the
 NVIDIA platform.*/
    pub fn hipCtxDisablePeerAccess(peerCtx: hipCtx_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get the state of the primary context [Deprecated]

 @param [in] dev  Device to get primary context flags for
 @param [out] flags  Pointer to store flags
 @param [out] active  Pointer to store context state; 0 = inactive, 1 = active

 @returns #hipSuccess

 @see hipCtxCreate, hipCtxDestroy, hipCtxGetFlags, hipCtxPopCurrent, hipCtxGetCurrent,
 hipCtxSetCurrent, hipCtxPushCurrent, hipCtxSetCacheConfig, hipCtxSynchronize, hipCtxGetDevice

 @warning  This API is deprecated on the AMD platform, only for equivalent driver API on the
 NVIDIA platform.*/
    pub fn hipDevicePrimaryCtxGetState(
        dev: hipDevice_t,
        flags: *mut ::core::ffi::c_uint,
        active: *mut ::core::ffi::c_int,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Release the primary context on the GPU.

 @param [in] dev  Device which primary context is released [Deprecated]

 @returns #hipSuccess

 @see hipCtxCreate, hipCtxDestroy, hipCtxGetFlags, hipCtxPopCurrent, hipCtxGetCurrent,
 hipCtxSetCurrent, hipCtxPushCurrent, hipCtxSetCacheConfig, hipCtxSynchronize, hipCtxGetDevice
 @warning This function return #hipSuccess though doesn't release the primaryCtx by design on
 HIP/HCC path.

 @warning  This API is deprecated on the AMD platform, only for equivalent driver API on the NVIDIA
 platform.*/
    pub fn hipDevicePrimaryCtxRelease(dev: hipDevice_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Retain the primary context on the GPU [Deprecated]

 @param [out] pctx  Returned context handle of the new context
 @param [in] dev  Device which primary context is released

 @returns #hipSuccess

 @see hipCtxCreate, hipCtxDestroy, hipCtxGetFlags, hipCtxPopCurrent, hipCtxGetCurrent,
 hipCtxSetCurrent, hipCtxPushCurrent, hipCtxSetCacheConfig, hipCtxSynchronize, hipCtxGetDevice

 @warning  This API is deprecated on the AMD platform, only for equivalent driver API on the NVIDIA
 platform.*/
    pub fn hipDevicePrimaryCtxRetain(
        pctx: *mut hipCtx_t,
        dev: hipDevice_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Resets the primary context on the GPU [Deprecated]

 @param [in] dev  Device which primary context is reset

 @returns #hipSuccess

 @see hipCtxCreate, hipCtxDestroy, hipCtxGetFlags, hipCtxPopCurrent, hipCtxGetCurrent,
 hipCtxSetCurrent, hipCtxPushCurrent, hipCtxSetCacheConfig, hipCtxSynchronize, hipCtxGetDevice

 @warning  This API is deprecated on the AMD platform, only for equivalent driver API on the NVIDIA
 platform.*/
    pub fn hipDevicePrimaryCtxReset(dev: hipDevice_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set flags for the primary context [Deprecated]

 @param [in] dev  Device for which the primary context flags are set
 @param [in] flags  New flags for the device

 @returns #hipSuccess, #hipErrorContextAlreadyInUse

 @see hipCtxCreate, hipCtxDestroy, hipCtxGetFlags, hipCtxPopCurrent, hipCtxGetCurrent,
 hipCtxSetCurrent, hipCtxPushCurrent, hipCtxSetCacheConfig, hipCtxSynchronize, hipCtxGetDevice

 @warning  This API is deprecated on the AMD platform, only for equivalent driver API on the NVIDIA
 platform.*/
    pub fn hipDevicePrimaryCtxSetFlags(
        dev: hipDevice_t,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " @}\n/\n/**\n-------------------------------------------------------------------------------------------------\n-------------------------------------------------------------------------------------------------\n\n  @defgroup Module Module Management\n  @{\n  @ingroup API\n  This section describes the module management functions of HIP runtime API.\n\n/\n/**\n @brief Loads code object from file into a module the currrent context.\n\n @param [in] fname  Filename of code object to load\n\n @param [out] module  Module\n\n @warning File/memory resources allocated in this function are released only in hipModuleUnload.\n\n @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidContext, #hipErrorFileNotFound,\n #hipErrorOutOfMemory, #hipErrorSharedObjectInitFailed, #hipErrorNotInitialized\n"]
    pub fn hipModuleLoad(
        module: *mut hipModule_t,
        fname: *const ::core::ffi::c_char,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Frees the module

 @param [in] module  Module to free

 @returns #hipSuccess, #hipErrorInvalidResourceHandle

 The module is freed, and the code objects associated with it are destroyed.*/
    pub fn hipModuleUnload(module: hipModule_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Function with kname will be extracted if present in module

 @param [in] module  Module to get function from
 @param [in] kname  Pointer to the name of function
 @param [out] function  Pointer to function handle

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidContext, #hipErrorNotInitialized,
 #hipErrorNotFound,*/
    pub fn hipModuleGetFunction(
        function: *mut hipFunction_t,
        module: hipModule_t,
        kname: *const ::core::ffi::c_char,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Find out attributes for a given function.
 @ingroup Execution
 @param [out] attr  Attributes of funtion
 @param [in] func  Pointer to the function handle

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidDeviceFunction*/
    pub fn hipFuncGetAttributes(
        attr: *mut hipFuncAttributes,
        func: *const ::core::ffi::c_void,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Find out a specific attribute for a given function.
 @ingroup Execution
 @param [out] value  Pointer to the value
 @param [in]  attrib  Attributes of the given funtion
 @param [in]  hfunc  Function to get attributes from

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidDeviceFunction*/
    pub fn hipFuncGetAttribute(
        value: *mut ::core::ffi::c_int,
        attrib: hipFunction_attribute,
        hfunc: hipFunction_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets pointer to device entry function that matches entry function symbolPtr.

 @param [out] functionPtr  Device entry function
 @param [in]  symbolPtr  Pointer to device entry function to search for

 @returns #hipSuccess, #hipErrorInvalidDeviceFunction
*/
    pub fn hipGetFuncBySymbol(
        functionPtr: *mut hipFunction_t,
        symbolPtr: *const ::core::ffi::c_void,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief returns the handle of the texture reference with the name from the module.

 @param [in] hmod  Module
 @param [in] name  Pointer of name of texture reference
 @param [out] texRef  Pointer of texture reference

 @returns #hipSuccess, #hipErrorNotInitialized, #hipErrorNotFound, #hipErrorInvalidValue*/
    pub fn hipModuleGetTexRef(
        texRef: *mut *mut textureReference,
        hmod: hipModule_t,
        name: *const ::core::ffi::c_char,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief builds module from code object which resides in host memory. Image is pointer to that
 location.

 @param [in] image  The pointer to the location of data
 @param [out] module  Retuned module

 @returns hipSuccess, hipErrorNotInitialized, hipErrorOutOfMemory, hipErrorNotInitialized*/
    pub fn hipModuleLoadData(
        module: *mut hipModule_t,
        image: *const ::core::ffi::c_void,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief builds module from code object which resides in host memory. Image is pointer to that
 location. Options are not used. hipModuleLoadData is called.

 @param [in] image  The pointer to the location of data
 @param [out] module  Retuned module
 @param [in] numOptions Number of options
 @param [in] options Options for JIT
 @param [in] optionValues  Option values for JIT

 @returns hipSuccess, hipErrorNotInitialized, hipErrorOutOfMemory, hipErrorNotInitialized*/
    pub fn hipModuleLoadDataEx(
        module: *mut hipModule_t,
        image: *const ::core::ffi::c_void,
        numOptions: ::core::ffi::c_uint,
        options: *mut hipJitOption,
        optionValues: *mut *mut ::core::ffi::c_void,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Completes the linking of the given program.
 @param [in] state hip link state
 @param [in] type  Type of the input data or bitcode
 @param [in] data  Input data which is null terminated
 @param [in] size  Size of the input data
 @param [in] name  Optional name for this input
 @param [in] numOptions  Size of the options
 @param [in] options  Array of options applied to this input
 @param [in] optionValues  Array of option values cast to void*

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidHandle

 If adding the file fails, it will
 @return #hipErrorInvalidConfiguration

 @see hipError_t*/
    pub fn hipLinkAddData(
        state: hipLinkState_t,
        type_: hipJitInputType,
        data: *mut ::core::ffi::c_void,
        size: usize,
        name: *const ::core::ffi::c_char,
        numOptions: ::core::ffi::c_uint,
        options: *mut hipJitOption,
        optionValues: *mut *mut ::core::ffi::c_void,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Adds a file with bit code to be linked with options
 @param [in] state hip link state
 @param [in] type  Type of the input data or bitcode
 @param [in] path  Path to the input file where bitcode is present
 @param [in] numOptions  Size of the options
 @param [in] options  Array of options applied to this input
 @param [in] optionValues  Array of option values cast to void*

 @returns #hipSuccess, #hipErrorInvalidValue

 If adding the file fails, it will
 @return #hipErrorInvalidConfiguration

 @see hipError_t*/
    pub fn hipLinkAddFile(
        state: hipLinkState_t,
        type_: hipJitInputType,
        path: *const ::core::ffi::c_char,
        numOptions: ::core::ffi::c_uint,
        options: *mut hipJitOption,
        optionValues: *mut *mut ::core::ffi::c_void,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Completes the linking of the given program.
 @param [in]   state hip link state
 @param [out]  hipBinOut  Upon success, points to the output binary
 @param [out]  sizeOut  Size of the binary is stored (optional)

 @returns #hipSuccess #hipErrorInvalidValue

 If adding the data fails, it will
 @return #hipErrorInvalidConfiguration

 @see hipError_t*/
    pub fn hipLinkComplete(
        state: hipLinkState_t,
        hipBinOut: *mut *mut ::core::ffi::c_void,
        sizeOut: *mut usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates the link instance via hip APIs.
 @param [in] numOptions  Number of options
 @param [in] option  Array of options
 @param [in] optionValues  Array of option values cast to void*
 @param [out] stateOut  hip link state created upon success

 @returns #hipSuccess #hipErrorInvalidValue #hipErrorInvalidConfiguration

 @see hipSuccess*/
    pub fn hipLinkCreate(
        numOptions: ::core::ffi::c_uint,
        options: *mut hipJitOption,
        optionValues: *mut *mut ::core::ffi::c_void,
        stateOut: *mut hipLinkState_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Deletes the link instance via hip APIs.
 @param [in] state link state instance

 @returns #hipSuccess #hipErrorInvalidValue

 @see hipSuccess*/
    pub fn hipLinkDestroy(state: hipLinkState_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief launches kernel f with launch parameters and shared memory on stream with arguments passed
 to kernelparams or extra
 @ingroup Execution
 @param [in] f         Kernel to launch.
 @param [in] gridDimX  X grid dimension specified as multiple of blockDimX.
 @param [in] gridDimY  Y grid dimension specified as multiple of blockDimY.
 @param [in] gridDimZ  Z grid dimension specified as multiple of blockDimZ.
 @param [in] blockDimX X block dimensions specified in work-items
 @param [in] blockDimY Y grid dimension specified in work-items
 @param [in] blockDimZ Z grid dimension specified in work-items
 @param [in] sharedMemBytes Amount of dynamic shared memory to allocate for this kernel. The
 HIP-Clang compiler provides support for extern shared declarations.
 @param [in] stream    Stream where the kernel should be dispatched.  May be 0, in which case th
 default stream is used with associated synchronization rules.
 @param [in] kernelParams  Kernel parameters to launch
 @param [in] extra     Pointer to kernel arguments.   These are passed directly to the kernel and
 must be in the memory layout and alignment expected by the kernel.
 All passed arguments must be naturally aligned according to their type. The memory address of each
 argument should be a multiple of its size in bytes. Please refer to hip_porting_driver_api.md
 for sample usage.

 Please note, HIP does not support kernel launch with total work items defined in dimension with
 size gridDim x blockDim >= 2^32. So gridDim.x * blockDim.x, gridDim.y * blockDim.y
 and gridDim.z * blockDim.z are always less than 2^32.

 @returns #hipSuccess, #hipErrorNotInitialized, #hipErrorInvalidValue*/
    pub fn hipModuleLaunchKernel(
        f: hipFunction_t,
        gridDimX: ::core::ffi::c_uint,
        gridDimY: ::core::ffi::c_uint,
        gridDimZ: ::core::ffi::c_uint,
        blockDimX: ::core::ffi::c_uint,
        blockDimY: ::core::ffi::c_uint,
        blockDimZ: ::core::ffi::c_uint,
        sharedMemBytes: ::core::ffi::c_uint,
        stream: hipStream_t,
        kernelParams: *mut *mut ::core::ffi::c_void,
        extra: *mut *mut ::core::ffi::c_void,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " \\addtogroup ModuleCooperativeG Cooperative groups kernel launch of Module management.\n \\ingroup Module\n  @{ */\n/**\n @brief launches kernel f with launch parameters and shared memory on stream with arguments passed\n to kernelParams, where thread blocks can cooperate and synchronize as they execute\n\n @param [in] f              Kernel to launch.\n @param [in] gridDimX       X grid dimension specified as multiple of blockDimX.\n @param [in] gridDimY       Y grid dimension specified as multiple of blockDimY.\n @param [in] gridDimZ       Z grid dimension specified as multiple of blockDimZ.\n @param [in] blockDimX      X block dimension specified in work-items.\n @param [in] blockDimY      Y block dimension specified in work-items.\n @param [in] blockDimZ      Z block dimension specified in work-items.\n @param [in] sharedMemBytes Amount of dynamic shared memory to allocate for this kernel. The\n HIP-Clang compiler provides support for extern shared declarations.\n @param [in] stream         Stream where the kernel should be dispatched. May be 0,\n in which case the default stream is used with associated synchronization rules.\n @param [in] kernelParams   A list of kernel arguments.\n\n Please note, HIP does not support kernel launch with total work items defined in dimension with\n size \\f$ gridDim \\cdot blockDim \\geq 2^{32} \\f$.\n\n @returns #hipSuccess, #hipErrorDeinitialized, #hipErrorNotInitialized, #hipErrorInvalidContext,\n #hipErrorInvalidHandle, #hipErrorInvalidImage, #hipErrorInvalidValue,\n #hipErrorInvalidConfiguration, #hipErrorLaunchFailure, #hipErrorLaunchOutOfResources,\n #hipErrorLaunchTimeOut, #hipErrorCooperativeLaunchTooLarge, #hipErrorSharedObjectInitFailed"]
    pub fn hipModuleLaunchCooperativeKernel(
        f: hipFunction_t,
        gridDimX: ::core::ffi::c_uint,
        gridDimY: ::core::ffi::c_uint,
        gridDimZ: ::core::ffi::c_uint,
        blockDimX: ::core::ffi::c_uint,
        blockDimY: ::core::ffi::c_uint,
        blockDimZ: ::core::ffi::c_uint,
        sharedMemBytes: ::core::ffi::c_uint,
        stream: hipStream_t,
        kernelParams: *mut *mut ::core::ffi::c_void,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Launches kernels on multiple devices where thread blocks can cooperate and
 synchronize as they execute.

 @param [in] launchParamsList         List of launch parameters, one per device.
 @param [in] numDevices               Size of the launchParamsList array.
 @param [in] flags                    Flags to control launch behavior.

 @returns #hipSuccess, #hipErrorDeinitialized, #hipErrorNotInitialized, #hipErrorInvalidContext,
 #hipErrorInvalidHandle, #hipErrorInvalidImage, #hipErrorInvalidValue,
 #hipErrorInvalidConfiguration, #hipErrorInvalidResourceHandle, #hipErrorLaunchFailure,
 #hipErrorLaunchOutOfResources, #hipErrorLaunchTimeOut, #hipErrorCooperativeLaunchTooLarge,
 #hipErrorSharedObjectInitFailed*/
    pub fn hipModuleLaunchCooperativeKernelMultiDevice(
        launchParamsList: *mut hipFunctionLaunchParams,
        numDevices: ::core::ffi::c_uint,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Launches kernel f with launch parameters and shared memory on stream with arguments passed
 to kernelparams or extra, where thread blocks can cooperate and synchronize as they execute.

 @param [in] f - Kernel to launch.
 @param [in] gridDim - Grid dimensions specified as multiple of blockDim.
 @param [in] blockDimX - Block dimensions specified in work-items
 @param [in] kernelParams - Pointer of arguments passed to the kernel. If the kernel has multiple
 parameters, 'kernelParams' should be array of pointers, each points the corresponding argument.
 @param [in] sharedMemBytes - Amount of dynamic shared memory to allocate for this kernel. The
 HIP-Clang compiler provides support for extern shared declarations.
 @param [in] stream - Stream where the kernel should be dispatched.  May be 0, in which case th
 default stream is used with associated synchronization rules.

 Please note, HIP does not support kernel launch with total work items defined in dimension with
 size \f$ gridDim \cdot blockDim \geq 2^{32} \f$.

 @returns #hipSuccess, #hipErrorNotInitialized, #hipErrorInvalidValue,
 #hipErrorCooperativeLaunchTooLarge*/
    pub fn hipLaunchCooperativeKernel(
        f: *const ::core::ffi::c_void,
        gridDim: dim3,
        blockDimX: dim3,
        kernelParams: *mut *mut ::core::ffi::c_void,
        sharedMemBytes: ::core::ffi::c_uint,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Launches kernels on multiple devices where thread blocks can cooperate and
 synchronize as they execute.

 @param [in] launchParamsList         List of launch parameters, one per device.
 @param [in] numDevices               Size of the launchParamsList array.
 @param [in] flags                    Flags to control launch behavior.

 @returns #hipSuccess, #hipErrorNotInitialized, #hipErrorInvalidValue,
  #hipErrorCooperativeLaunchTooLarge*/
    pub fn hipLaunchCooperativeKernelMultiDevice(
        launchParamsList: *mut hipLaunchParams,
        numDevices: ::core::ffi::c_int,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Launches kernels on multiple devices and guarantees all specified kernels are dispatched
 on respective streams before enqueuing any other work on the specified streams from any other threads
 @ingroup Execution
 @param [in] launchParamsList          List of launch parameters, one per device.
 @param [in] numDevices               Size of the launchParamsList array.
 @param [in] flags                    Flags to control launch behavior.

 @returns #hipSuccess, #hipErrorNotInitialized, #hipErrorInvalidValue*/
    pub fn hipExtLaunchMultiKernelMultiDevice(
        launchParamsList: *mut hipLaunchParams,
        numDevices: ::core::ffi::c_int,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = "-------------------------------------------------------------------------------------------------\n-------------------------------------------------------------------------------------------------\n  @defgroup Occupancy Occupancy\n  @{\n  This section describes the occupancy functions of HIP runtime API.\n\n/\n/**\n @brief determine the grid and block sizes to achieves maximum occupancy for a kernel\n\n @param [out] gridSize           minimum grid size for maximum potential occupancy\n @param [out] blockSize          block size for maximum potential occupancy\n @param [in]  f                  kernel function for which occupancy is calulated\n @param [in]  dynSharedMemPerBlk dynamic shared memory usage (in bytes) intended for each block\n @param [in]  blockSizeLimit     the maximum block size for the kernel, use 0 for no limit\n\n Please note, HIP does not support kernel launch with total work items defined in dimension with\n size gridDim x blockDim >= 2^32.\n\n @returns #hipSuccess, #hipErrorInvalidValue"]
    pub fn hipModuleOccupancyMaxPotentialBlockSize(
        gridSize: *mut ::core::ffi::c_int,
        blockSize: *mut ::core::ffi::c_int,
        f: hipFunction_t,
        dynSharedMemPerBlk: usize,
        blockSizeLimit: ::core::ffi::c_int,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief determine the grid and block sizes to achieves maximum occupancy for a kernel

 @param [out] gridSize           minimum grid size for maximum potential occupancy
 @param [out] blockSize          block size for maximum potential occupancy
 @param [in]  f                  kernel function for which occupancy is calulated
 @param [in]  dynSharedMemPerBlk dynamic shared memory usage (in bytes) intended for each block
 @param [in]  blockSizeLimit     the maximum block size for the kernel, use 0 for no limit
 @param [in]  flags            Extra flags for occupancy calculation (only default supported)

 Please note, HIP does not support kernel launch with total work items defined in dimension with
 size gridDim x blockDim >= 2^32.

 @returns #hipSuccess, #hipErrorInvalidValue*/
    pub fn hipModuleOccupancyMaxPotentialBlockSizeWithFlags(
        gridSize: *mut ::core::ffi::c_int,
        blockSize: *mut ::core::ffi::c_int,
        f: hipFunction_t,
        dynSharedMemPerBlk: usize,
        blockSizeLimit: ::core::ffi::c_int,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns occupancy for a device function.

 @param [out] numBlocks        Returned occupancy
 @param [in]  f                Kernel function (hipFunction) for which occupancy is calulated
 @param [in]  blockSize        Block size the kernel is intended to be launched with
 @param [in]  dynSharedMemPerBlk Dynamic shared memory usage (in bytes) intended for each block
 @returns  #hipSuccess, #hipErrorInvalidValue*/
    pub fn hipModuleOccupancyMaxActiveBlocksPerMultiprocessor(
        numBlocks: *mut ::core::ffi::c_int,
        f: hipFunction_t,
        blockSize: ::core::ffi::c_int,
        dynSharedMemPerBlk: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns occupancy for a device function.

 @param [out] numBlocks        Returned occupancy
 @param [in]  f                Kernel function(hipFunction_t) for which occupancy is calulated
 @param [in]  blockSize        Block size the kernel is intended to be launched with
 @param [in]  dynSharedMemPerBlk Dynamic shared memory usage (in bytes) intended for each block
 @param [in]  flags            Extra flags for occupancy calculation (only default supported)
 @returns  #hipSuccess, #hipErrorInvalidValue*/
    pub fn hipModuleOccupancyMaxActiveBlocksPerMultiprocessorWithFlags(
        numBlocks: *mut ::core::ffi::c_int,
        f: hipFunction_t,
        blockSize: ::core::ffi::c_int,
        dynSharedMemPerBlk: usize,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns occupancy for a device function.

 @param [out] numBlocks        Returned occupancy
 @param [in]  f                Kernel function for which occupancy is calulated
 @param [in]  blockSize        Block size the kernel is intended to be launched with
 @param [in]  dynSharedMemPerBlk Dynamic shared memory usage (in bytes) intended for each block
 @returns  #hipSuccess, #hipErrorInvalidDeviceFunction, #hipErrorInvalidValue*/
    pub fn hipOccupancyMaxActiveBlocksPerMultiprocessor(
        numBlocks: *mut ::core::ffi::c_int,
        f: *const ::core::ffi::c_void,
        blockSize: ::core::ffi::c_int,
        dynSharedMemPerBlk: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns occupancy for a device function.

 @param [out] numBlocks        Returned occupancy
 @param [in]  f                Kernel function for which occupancy is calulated
 @param [in]  blockSize        Block size the kernel is intended to be launched with
 @param [in]  dynSharedMemPerBlk Dynamic shared memory usage (in bytes) intended for each block
 @param [in]  flags            Extra flags for occupancy calculation (currently ignored)
 @returns  #hipSuccess, #hipErrorInvalidDeviceFunction, #hipErrorInvalidValue*/
    pub fn hipOccupancyMaxActiveBlocksPerMultiprocessorWithFlags(
        numBlocks: *mut ::core::ffi::c_int,
        f: *const ::core::ffi::c_void,
        blockSize: ::core::ffi::c_int,
        dynSharedMemPerBlk: usize,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief determine the grid and block sizes to achieves maximum occupancy for a kernel

 @param [out] gridSize           minimum grid size for maximum potential occupancy
 @param [out] blockSize          block size for maximum potential occupancy
 @param [in]  f                  kernel function for which occupancy is calulated
 @param [in]  dynSharedMemPerBlk dynamic shared memory usage (in bytes) intended for each block
 @param [in]  blockSizeLimit     the maximum block size for the kernel, use 0 for no limit

 Please note, HIP does not support kernel launch with total work items defined in dimension with
 size gridDim x blockDim >= 2^32.

 @returns #hipSuccess, #hipErrorInvalidValue*/
    pub fn hipOccupancyMaxPotentialBlockSize(
        gridSize: *mut ::core::ffi::c_int,
        blockSize: *mut ::core::ffi::c_int,
        f: *const ::core::ffi::c_void,
        dynSharedMemPerBlk: usize,
        blockSizeLimit: ::core::ffi::c_int,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Start recording of profiling information [Deprecated]
 When using this API, start the profiler with profiling disabled.  (--startdisabled)
 @returns  #hipErrorNotSupported
 @warning hipProfilerStart API is deprecated, use roctracer/rocTX instead.*/
    pub fn hipProfilerStart() -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Stop recording of profiling information [Deprecated]
 When using this API, start the profiler with profiling disabled.  (--startdisabled)
 @returns  #hipErrorNotSupported
 @warning  hipProfilerStart API is deprecated, use roctracer/rocTX instead.*/
    pub fn hipProfilerStop() -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    #[doc = " @}\n/\n/**\n-------------------------------------------------------------------------------------------------\n-------------------------------------------------------------------------------------------------\n  @defgroup Clang Launch API to support the triple-chevron syntax\n  @{\n  This section describes the API to support the triple-chevron syntax.\n/\n/**\n @brief Configure a kernel launch.\n\n @param [in] gridDim   grid dimension specified as multiple of blockDim.\n @param [in] blockDim  block dimensions specified in work-items\n @param [in] sharedMem Amount of dynamic shared memory to allocate for this kernel. The\n HIP-Clang compiler provides support for extern shared declarations.\n @param [in] stream    Stream where the kernel should be dispatched.  May be 0, in which case the\n default stream is used with associated synchronization rules.\n\n Please note, HIP does not support kernel launch with total work items defined in dimension with\n size gridDim x blockDim >= 2^32.\n\n @returns #hipSuccess, #hipErrorNotInitialized, #hipErrorInvalidValue\n"]
    pub fn hipConfigureCall(
        gridDim: dim3,
        blockDim: dim3,
        sharedMem: usize,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set a kernel argument.

 @returns #hipSuccess, #hipErrorNotInitialized, #hipErrorInvalidValue

 @param [in] arg    Pointer the argument in host memory.
 @param [in] size   Size of the argument.
 @param [in] offset Offset of the argument on the argument stack.
*/
    pub fn hipSetupArgument(
        arg: *const ::core::ffi::c_void,
        size: usize,
        offset: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Launch a kernel.

 @param [in] func Kernel to launch.

 @returns #hipSuccess, #hipErrorNotInitialized, #hipErrorInvalidValue
*/
    pub fn hipLaunchByPtr(func: *const ::core::ffi::c_void) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief C compliant kernel launch API

 @param [in] function_address - Kernel stub function pointer.
 @param [in] numBlocks - Number of blocks.
 @param [in] dimBlocks - Dimension of a block
 @param [in] args - Pointer of arguments passed to the kernel. If the kernel has multiple
 parameters, 'args' should be array of pointers, each points the corresponding argument.
 @param [in] sharedMemBytes - Amount of dynamic shared memory to allocate for this kernel. The
 HIP-Clang compiler provides support for extern shared declarations.
 @param [in] stream - Stream where the kernel should be dispatched.  May be 0, in which case th
  default stream is used with associated synchronization rules.

 @returns #hipSuccess, #hipErrorInvalidValue
*/
    pub fn hipLaunchKernel(
        function_address: *const ::core::ffi::c_void,
        numBlocks: dim3,
        dimBlocks: dim3,
        args: *mut *mut ::core::ffi::c_void,
        sharedMemBytes: usize,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Enqueues a host function call in a stream.

 @param [in] stream - The stream to enqueue work in.
 @param [in] fn - The function to call once enqueued preceeding operations are complete.
 @param [in] userData - User-specified data to be passed to the function.

 @returns #hipSuccess, #hipErrorInvalidResourceHandle, #hipErrorInvalidValue,
 #hipErrorNotSupported

 The host function to call in this API will be executed after the preceding operations in
 the stream are complete. The function is a blocking operation that blocks operations in the
 stream that follow it, until the function is returned.
 Event synchronization and internal callback functions make sure enqueued operations will
 execute in order, in the stream.

 The host function must not make any HIP API calls. The host function is non-reentrant. It must
 not perform sychronization with any operation that may depend on other processing execution
 but is not enqueued to run earlier in the stream.

 Host functions that are enqueued respectively in different non-blocking streams can run concurrently.

 @warning  This API is marked as beta, meaning, while this is feature complete,
 it is still open to changes and may have outstanding issues.*/
    pub fn hipLaunchHostFunc(
        stream: hipStream_t,
        fn_: hipHostFn_t,
        userData: *mut ::core::ffi::c_void,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** Copies memory for 2D arrays.

 @param pCopy           - Parameters for the memory copy

 @returns #hipSuccess, #hipErrorInvalidValue*/
    pub fn hipDrvMemcpy2DUnaligned(pCopy: *const hip_Memcpy2D) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Launches kernel from the pointer address, with arguments and shared memory on stream.

 @param [in] function_address - Pointer to the Kernel to launch.
 @param [in] numBlocks -  Number of blocks.
 @param [in] dimBlocks - Dimension of a block.
 @param [in] args - Pointer of arguments passed to the kernel. If the kernel has multiple
 parameters, 'args' should be array of pointers, each points the corresponding argument.
 @param [in] sharedMemBytes - Amount of dynamic shared memory to allocate for this kernel.
 HIP-Clang compiler provides support for extern shared declarations.
 @param [in] stream - Stream where the kernel should be dispatched.
 May be 0, in which case the default stream is used with associated synchronization rules.
 @param [in] startEvent - If non-null, specified event will be updated to track the start time of
 the kernel launch. The event must be created before calling this API.
 @param [in] stopEvent - If non-null, specified event will be updated to track the stop time of
 the kernel launch. The event must be created before calling this API.
 @param [in] flags - The value of hipExtAnyOrderLaunch, signifies if kernel can be
 launched in any order.
 @returns #hipSuccess, #hipErrorNotInitialized, #hipErrorInvalidValue.
*/
    pub fn hipExtLaunchKernel(
        function_address: *const ::core::ffi::c_void,
        numBlocks: dim3,
        dimBlocks: dim3,
        args: *mut *mut ::core::ffi::c_void,
        sharedMemBytes: usize,
        stream: hipStream_t,
        startEvent: hipEvent_t,
        stopEvent: hipEvent_t,
        flags: ::core::ffi::c_int,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a texture object.

 @param [out] pTexObject  pointer to the texture object to create
 @param [in] pResDesc  pointer to resource descriptor
 @param [in] pTexDesc  pointer to texture descriptor
 @param [in] pResViewDesc  pointer to resource view descriptor

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported, #hipErrorOutOfMemory

 @note 3D liner filter isn't supported on GFX90A boards, on which the API @p hipCreateTextureObject will
 return hipErrorNotSupported.
*/
    pub fn hipCreateTextureObject(
        pTexObject: *mut hipTextureObject_t,
        pResDesc: *const hipResourceDesc,
        pTexDesc: *const hipTextureDesc,
        pResViewDesc: *const hipResourceViewDesc,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Destroys a texture object.

 @param [in] textureObject  texture object to destroy

 @returns #hipSuccess, #hipErrorInvalidValue
*/
    pub fn hipDestroyTextureObject(textureObject: hipTextureObject_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets the channel descriptor in an array.

 @param [in] desc  pointer to channel format descriptor
 @param [out] array  memory array on the device

 @returns #hipSuccess, #hipErrorInvalidValue
*/
    pub fn hipGetChannelDesc(
        desc: *mut hipChannelFormatDesc,
        array: hipArray_const_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets resource descriptor for the texture object.

 @param [out] pResDesc  pointer to resource descriptor
 @param [in] textureObject  texture object

 @returns #hipSuccess, #hipErrorInvalidValue
*/
    pub fn hipGetTextureObjectResourceDesc(
        pResDesc: *mut hipResourceDesc,
        textureObject: hipTextureObject_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets resource view descriptor for the texture object.

 @param [out] pResViewDesc  pointer to resource view descriptor
 @param [in] textureObject  texture object

 @returns #hipSuccess, #hipErrorInvalidValue
*/
    pub fn hipGetTextureObjectResourceViewDesc(
        pResViewDesc: *mut hipResourceViewDesc,
        textureObject: hipTextureObject_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets texture descriptor for the texture object.

 @param [out] pTexDesc  pointer to texture descriptor
 @param [in] textureObject  texture object

 @returns #hipSuccess, #hipErrorInvalidValue
*/
    pub fn hipGetTextureObjectTextureDesc(
        pTexDesc: *mut hipTextureDesc,
        textureObject: hipTextureObject_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a texture object.

 @param [out] pTexObject  pointer to texture object to create
 @param [in] pResDesc  pointer to resource descriptor
 @param [in] pTexDesc  pointer to texture descriptor
 @param [in] pResViewDesc  pointer to resource view descriptor

 @returns #hipSuccess, #hipErrorInvalidValue
*/
    pub fn hipTexObjectCreate(
        pTexObject: *mut hipTextureObject_t,
        pResDesc: *const HIP_RESOURCE_DESC,
        pTexDesc: *const HIP_TEXTURE_DESC,
        pResViewDesc: *const HIP_RESOURCE_VIEW_DESC,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Destroys a texture object.

 @param [in] texObject  texture object to destroy

 @returns #hipSuccess, #hipErrorInvalidValue
*/
    pub fn hipTexObjectDestroy(texObject: hipTextureObject_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets resource descriptor of a texture object.

 @param [out] pResDesc  pointer to resource descriptor
 @param [in] texObject  texture object

 @returns #hipSuccess, #hipErrorNotSupported, #hipErrorInvalidValue
*/
    pub fn hipTexObjectGetResourceDesc(
        pResDesc: *mut HIP_RESOURCE_DESC,
        texObject: hipTextureObject_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets resource view descriptor of a texture object.

 @param [out] pResViewDesc  pointer to resource view descriptor
 @param [in] texObject  texture object

 @returns #hipSuccess, #hipErrorNotSupported, #hipErrorInvalidValue
*/
    pub fn hipTexObjectGetResourceViewDesc(
        pResViewDesc: *mut HIP_RESOURCE_VIEW_DESC,
        texObject: hipTextureObject_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets texture descriptor of a texture object.

 @param [out] pTexDesc  pointer to texture descriptor
 @param [in] texObject  texture object

 @returns #hipSuccess, #hipErrorNotSupported, #hipErrorInvalidValue
*/
    pub fn hipTexObjectGetTextureDesc(
        pTexDesc: *mut HIP_TEXTURE_DESC,
        texObject: hipTextureObject_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Allocate a mipmapped array on the device.

 @param[out] mipmappedArray  - Pointer to allocated mipmapped array in device memory
 @param[in]  desc            - Requested channel format
 @param[in]  extent          - Requested allocation size (width field in elements)
 @param[in]  numLevels       - Number of mipmap levels to allocate
 @param[in]  flags           - Flags for extensions

 @return #hipSuccess, #hipErrorInvalidValue, #hipErrorMemoryAllocation

 @note  This API is implemented on Linux and is under development on Microsoft Windows.
*/
    pub fn hipMallocMipmappedArray(
        mipmappedArray: *mut hipMipmappedArray_t,
        desc: *const hipChannelFormatDesc,
        extent: hipExtent,
        numLevels: ::core::ffi::c_uint,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Frees a mipmapped array on the device.

 @param[in] mipmappedArray - Pointer to mipmapped array to free

 @return #hipSuccess, #hipErrorInvalidValue

 @note  This API is implemented on Linux and is under development on Microsoft Windows.
*/
    pub fn hipFreeMipmappedArray(mipmappedArray: hipMipmappedArray_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets a mipmap level of a HIP mipmapped array.

 @param[out] levelArray     - Returned mipmap level HIP array
 @param[in]  mipmappedArray - HIP mipmapped array
 @param[in]  level          - Mipmap level

 @return #hipSuccess, #hipErrorInvalidValue

 @note  This API is implemented on Linux and is under development on Microsoft Windows.
*/
    pub fn hipGetMipmappedArrayLevel(
        levelArray: *mut hipArray_t,
        mipmappedArray: hipMipmappedArray_const_t,
        level: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Create a mipmapped array.

 @param [out] pHandle  pointer to mipmapped array
 @param [in] pMipmappedArrayDesc  mipmapped array descriptor
 @param [in] numMipmapLevels  mipmap level

 @returns #hipSuccess, #hipErrorNotSupported, #hipErrorInvalidValue

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipMipmappedArrayCreate(
        pHandle: *mut hipMipmappedArray_t,
        pMipmappedArrayDesc: *mut HIP_ARRAY3D_DESCRIPTOR,
        numMipmapLevels: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Destroy a mipmapped array.

 @param [out] hMipmappedArray  pointer to mipmapped array to destroy

 @returns #hipSuccess, #hipErrorInvalidValue

 @note  This API is implemented on Linux and is under development on Microsoft Windows.
*/
    pub fn hipMipmappedArrayDestroy(hMipmappedArray: hipMipmappedArray_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get a mipmapped array on a mipmapped level.

 @param [in] pLevelArray Pointer of array
 @param [out] hMipMappedArray Pointer of mipmapped array on the requested mipmap level
 @param [out] level  Mipmap level

 @returns #hipSuccess, #hipErrorInvalidValue

 @note  This API is implemented on Linux and is under development on Microsoft Windows.
*/
    pub fn hipMipmappedArrayGetLevel(
        pLevelArray: *mut hipArray_t,
        hMipMappedArray: hipMipmappedArray_t,
        level: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief  Binds a mipmapped array to a texture [Deprecated]

 @param [in] tex  pointer to the texture reference to bind
 @param [in] mipmappedArray memory mipmapped array on the device
 @param [in] desc  opointer to the channel format

 @returns #hipSuccess, #hipErrorInvalidValue
*/
    pub fn hipBindTextureToMipmappedArray(
        tex: *const textureReference,
        mipmappedArray: hipMipmappedArray_const_t,
        desc: *const hipChannelFormatDesc,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets the texture reference related with the symbol [Deprecated]

 @param [out] texref  texture reference
 @param [in] symbol  pointer to the symbol related with the texture for the reference

 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is deprecated.
*/
    pub fn hipGetTextureReference(
        texref: *mut *const textureReference,
        symbol: *const ::core::ffi::c_void,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets the border color used by a texture reference [Deprecated]

 @param [out] pBorderColor  Returned Type and Value of RGBA color.
 @param [in] texRef  Texture reference.

 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is deprecated.
*/
    pub fn hipTexRefGetBorderColor(
        pBorderColor: *mut f32,
        texRef: *const textureReference,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets the array bound to a texture reference [Deprecated]


 @param [in] pArray  Returned array.
 @param [in] texRef  texture reference.

 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is deprecated.
*/
    pub fn hipTexRefGetArray(
        pArray: *mut hipArray_t,
        texRef: *const textureReference,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets address mode for a texture reference [Deprecated]

 @param [in] texRef  texture reference.
 @param [in] dim  Dimension of the texture.
 @param [in] am  Value of the texture address mode.

 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is deprecated.
*/
    pub fn hipTexRefSetAddressMode(
        texRef: *mut textureReference,
        dim: ::core::ffi::c_int,
        am: hipTextureAddressMode,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Binds an array as a texture reference [Deprecated]

 @param [in] tex  Pointer texture reference.
 @param [in] array  Array to bind.
 @param [in] flags  Flags should be set as HIP_TRSA_OVERRIDE_FORMAT, as a valid value.

 @returns #hipSuccess, #hipErrorInvalidValue

 @warning This API is deprecated.
*/
    pub fn hipTexRefSetArray(
        tex: *mut textureReference,
        array: hipArray_const_t,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set filter mode for a texture reference [Deprecated]

 @param [in] texRef  Pointer texture reference.
 @param [in] fm  Value of texture filter mode.

 @returns #hipSuccess, #hipErrorInvalidValue

 @warning This API is deprecated.
*/
    pub fn hipTexRefSetFilterMode(
        texRef: *mut textureReference,
        fm: hipTextureFilterMode,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set flags for a texture reference [Deprecated]

 @param [in] texRef  Pointer texture reference.
 @param [in] Flags  Value of flags.

 @returns #hipSuccess, #hipErrorInvalidValue

 @warning This API is deprecated.
*/
    pub fn hipTexRefSetFlags(
        texRef: *mut textureReference,
        Flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set format for a texture reference [Deprecated]

 @param [in] texRef  Pointer texture reference.
 @param [in] fmt  Value of format.
 @param [in] NumPackedComponents  Number of components per array.

 @returns #hipSuccess, #hipErrorInvalidValue

 @warning This API is deprecated.
*/
    pub fn hipTexRefSetFormat(
        texRef: *mut textureReference,
        fmt: hipArray_Format,
        NumPackedComponents: ::core::ffi::c_int,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Binds a memory area to a texture [Deprecated]

 @param [in] offset  Offset in bytes.
 @param [in] tex  Texture to bind.
 @param [in] devPtr  Pointer of memory on the device.
 @param [in] desc  Pointer of channel format descriptor.
 @param [in] size  Size of memory in bites.

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported

 @warning This API is deprecated.
*/
    pub fn hipBindTexture(
        offset: *mut usize,
        tex: *const textureReference,
        devPtr: *const ::core::ffi::c_void,
        desc: *const hipChannelFormatDesc,
        size: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Binds a 2D memory area to a texture [Deprecated]

 @param [in] offset  Offset in bytes.
 @param [in] tex  Texture to bind.
 @param [in] devPtr  Pointer of 2D memory area on the device.
 @param [in] desc  Pointer of channel format descriptor.
 @param [in] width  Width in texel units.
 @param [in] height  Height in texel units.
 @param [in] pitch  Pitch in bytes.

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported

 @warning This API is deprecated.
*/
    pub fn hipBindTexture2D(
        offset: *mut usize,
        tex: *const textureReference,
        devPtr: *const ::core::ffi::c_void,
        desc: *const hipChannelFormatDesc,
        width: usize,
        height: usize,
        pitch: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Binds a memory area to a texture [Deprecated]

 @param [in] tex  Pointer of texture reference.
 @param [in] array  Array to bind.
 @param [in] desc  Pointer of channel format descriptor.

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported

 @warning This API is deprecated.
*/
    pub fn hipBindTextureToArray(
        tex: *const textureReference,
        array: hipArray_const_t,
        desc: *const hipChannelFormatDesc,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get the offset of the alignment in a texture [Deprecated]

 @param [in] offset  Offset in bytes.
 @param [in] texref  Pointer of texture reference.

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported

 @warning This API is deprecated.
*/
    pub fn hipGetTextureAlignmentOffset(
        offset: *mut usize,
        texref: *const textureReference,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Unbinds a texture [Deprecated]

 @param [in] tex  Texture to unbind.

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported

 @warning This API is deprecated.
*/
    pub fn hipUnbindTexture(tex: *const textureReference) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets the address for a texture reference [Deprecated]

 @param [out] dev_ptr  Pointer of device address.
 @param [in] texRef  Pointer of texture reference.

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported

 @warning This API is deprecated.
*/
    pub fn hipTexRefGetAddress(
        dev_ptr: *mut hipDeviceptr_t,
        texRef: *const textureReference,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets the address mode for a texture reference [Deprecated]

 @param [out] pam  Pointer of address mode.
 @param [in] texRef  Pointer of texture reference.
 @param [in] dim  Dimension.

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported

 @warning This API is deprecated.
*/
    pub fn hipTexRefGetAddressMode(
        pam: *mut hipTextureAddressMode,
        texRef: *const textureReference,
        dim: ::core::ffi::c_int,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets filter mode for a texture reference [Deprecated]

 @param [out] pfm  Pointer of filter mode.
 @param [in] texRef  Pointer of texture reference.

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported

 @warning This API is deprecated.
*/
    pub fn hipTexRefGetFilterMode(
        pfm: *mut hipTextureFilterMode,
        texRef: *const textureReference,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets flags for a texture reference [Deprecated]

 @param [out] pFlags  Pointer of flags.
 @param [in] texRef  Pointer of texture reference.

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported

 @warning This API is deprecated.
*/
    pub fn hipTexRefGetFlags(
        pFlags: *mut ::core::ffi::c_uint,
        texRef: *const textureReference,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets texture format for a texture reference [Deprecated]

 @param [out] pFormat  Pointer of the format.
 @param [out] pNumChannels  Pointer of number of channels.
 @param [in] texRef  Pointer of texture reference.

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported

 @warning This API is deprecated.
*/
    pub fn hipTexRefGetFormat(
        pFormat: *mut hipArray_Format,
        pNumChannels: *mut ::core::ffi::c_int,
        texRef: *const textureReference,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets the maximum anisotropy for a texture reference [Deprecated]

 @param [out] pmaxAnsio  Pointer of the maximum anisotropy.
 @param [in] texRef  Pointer of texture reference.

 @returns #hipErrorInvalidValue, #hipErrorNotSupported

 @warning This API is deprecated.
*/
    pub fn hipTexRefGetMaxAnisotropy(
        pmaxAnsio: *mut ::core::ffi::c_int,
        texRef: *const textureReference,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets the mipmap filter mode for a texture reference [Deprecated]

 @param [out] pfm  Pointer of the mipmap filter mode.
 @param [in] texRef  Pointer of texture reference.

 @returns #hipErrorInvalidValue, #hipErrorNotSupported

 @warning This API is deprecated.
*/
    pub fn hipTexRefGetMipmapFilterMode(
        pfm: *mut hipTextureFilterMode,
        texRef: *const textureReference,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets the mipmap level bias for a texture reference [Deprecated]

 @param [out] pbias  Pointer of the mipmap level bias.
 @param [in] texRef  Pointer of texture reference.

 @returns #hipErrorInvalidValue, #hipErrorNotSupported

 @warning This API is deprecated.
*/
    pub fn hipTexRefGetMipmapLevelBias(
        pbias: *mut f32,
        texRef: *const textureReference,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets the minimum and maximum mipmap level clamps for a texture reference [Deprecated]

 @param [out] pminMipmapLevelClamp  Pointer of the minimum mipmap level clamp.
 @param [out] pmaxMipmapLevelClamp  Pointer of the maximum mipmap level clamp.
 @param [in] texRef  Pointer of texture reference.

 @returns #hipErrorInvalidValue, #hipErrorNotSupported

 @warning This API is deprecated.
*/
    pub fn hipTexRefGetMipmapLevelClamp(
        pminMipmapLevelClamp: *mut f32,
        pmaxMipmapLevelClamp: *mut f32,
        texRef: *const textureReference,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets the mipmapped array bound to a texture reference [Deprecated]

 @param [out] pArray  Pointer of the mipmapped array.
 @param [in] texRef  Pointer of texture reference.

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported

 @warning This API is deprecated.
*/
    pub fn hipTexRefGetMipMappedArray(
        pArray: *mut hipMipmappedArray_t,
        texRef: *const textureReference,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets an bound address for a texture reference [Deprecated]

 @param [out] ByteOffset  Pointer of the offset in bytes.
 @param [in] texRef  Pointer of texture reference.
 @param [in] dptr  Pointer of device address to bind.
 @param [in] bytes  Size in bytes.

 @returns #hipSuccess, #hipErrorInvalidValue

 @warning This API is deprecated.
*/
    pub fn hipTexRefSetAddress(
        ByteOffset: *mut usize,
        texRef: *mut textureReference,
        dptr: hipDeviceptr_t,
        bytes: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set a bind an address as a 2D texture reference [Deprecated]

 @param [in] texRef  Pointer of texture reference.
 @param [in] desc  Pointer of array descriptor.
 @param [in] dptr  Pointer of device address to bind.
 @param [in] Pitch  Pitch in bytes.

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported

 @warning This API is deprecated.
*/
    pub fn hipTexRefSetAddress2D(
        texRef: *mut textureReference,
        desc: *const HIP_ARRAY_DESCRIPTOR,
        dptr: hipDeviceptr_t,
        Pitch: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets the maximum anisotropy for a texture reference [Deprecated]

 @param [in] texRef  Pointer of texture reference.
 @param [out] maxAniso  Value of the maximum anisotropy.

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported

 @warning This API is deprecated.
*/
    pub fn hipTexRefSetMaxAnisotropy(
        texRef: *mut textureReference,
        maxAniso: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets border color for a texture reference [Deprecated]

 @param [in] texRef  Pointer of texture reference.
 @param [in] pBorderColor  Pointer of border color.

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported

 @warning This API is deprecated.
*/
    pub fn hipTexRefSetBorderColor(
        texRef: *mut textureReference,
        pBorderColor: *mut f32,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets mipmap filter mode for a texture reference [Deprecated]

 @param [in] texRef  Pointer of texture reference.
 @param [in] fm  Value of filter mode.

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported

 @warning This API is deprecated.
*/
    pub fn hipTexRefSetMipmapFilterMode(
        texRef: *mut textureReference,
        fm: hipTextureFilterMode,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets mipmap level bias for a texture reference [Deprecated]

 @param [in] texRef  Pointer of texture reference.
 @param [in] bias  Value of mipmap bias.

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported

 @warning This API is deprecated.
*/
    pub fn hipTexRefSetMipmapLevelBias(
        texRef: *mut textureReference,
        bias: f32,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets mipmap level clamp for a texture reference [Deprecated]

 @param [in] texRef  Pointer of texture reference.
 @param [in] minMipMapLevelClamp  Value of minimum mipmap level clamp.
 @param [in] maxMipMapLevelClamp  Value of maximum mipmap level clamp.

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported

 @warning This API is deprecated.
*/
    pub fn hipTexRefSetMipmapLevelClamp(
        texRef: *mut textureReference,
        minMipMapLevelClamp: f32,
        maxMipMapLevelClamp: f32,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Binds mipmapped array to a texture reference [Deprecated]

 @param [in] texRef  Pointer of texture reference to bind.
 @param [in] mipmappedArray  Pointer of mipmapped array to bind.
 @param [in] Flags  Flags should be set as HIP_TRSA_OVERRIDE_FORMAT, as a valid value.

 @returns #hipSuccess, #hipErrorInvalidValue

 @warning This API is deprecated.
*/
    pub fn hipTexRefSetMipmappedArray(
        texRef: *mut textureReference,
        mipmappedArray: *mut hipMipmappedArray,
        Flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[doc = "  @defgroup Callback Callback Activity APIs\n  @{\n  This section describes the callback/Activity of HIP runtime API.\n/\n/**\n @brief Returns HIP API name by ID.\n\n @param [in] id ID of HIP API\n\n @returns #hipSuccess, #hipErrorInvalidValue\n"]
    pub fn hipApiName(id: u32) -> *const ::core::ffi::c_char;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    /** @brief Returns kernel name reference by function name.

 @param [in] f Name of function

 @returns #hipSuccess, #hipErrorInvalidValue
*/
    pub fn hipKernelNameRef(f: hipFunction_t) -> *const ::core::ffi::c_char;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    /** @brief Retrives kernel for a given host pointer, unless stated otherwise.

 @param [in] hostFunction Pointer of host function.
 @param [in] stream Stream the kernel is executed on.

 @returns #hipSuccess, #hipErrorInvalidValue
*/
    pub fn hipKernelNameRefByPtr(
        hostFunction: *const ::core::ffi::c_void,
        stream: hipStream_t,
    ) -> *const ::core::ffi::c_char;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    /** @brief Returns device ID on the stream.

 @param [in] stream Stream of device executed on.

 @returns #hipSuccess, #hipErrorInvalidValue
*/
    pub fn hipGetStreamDeviceId(stream: hipStream_t) -> ::core::ffi::c_int;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Begins graph capture on a stream.

 @param [in] stream - Stream to initiate capture.
 @param [in] mode - Controls the interaction of this capture sequence with other API calls that
 are not safe.

 @returns #hipSuccess, #hipErrorInvalidValue

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.
*/
    pub fn hipStreamBeginCapture(
        stream: hipStream_t,
        mode: hipStreamCaptureMode,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Begins graph capture on a stream to an existing graph.

 @param [in] stream - Stream to initiate capture.
 @param [in] graph - Graph to capture into.
 @param [in] dependencies - Dependencies of the first node captured in the stream. Can be NULL if
 numDependencies is 0.
 @param [in] dependencyData - Optional array of data associated with each dependency.
 @param [in] numDependencies - Number of dependencies.
 @param [in] mode - Controls the interaction of this capture sequence with other API calls that
are not safe.

 @returns #hipSuccess, #hipErrorInvalidValue

 @warning param "const hipGraphEdgeData* dependencyData" is currently not supported and has to be
passed as nullptr. This API is marked as beta, meaning, while this is feature complete, it is still
open to changes and may have outstanding issues.*/
    pub fn hipStreamBeginCaptureToGraph(
        stream: hipStream_t,
        graph: hipGraph_t,
        dependencies: *const hipGraphNode_t,
        dependencyData: *const hipGraphEdgeData,
        numDependencies: usize,
        mode: hipStreamCaptureMode,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Ends capture on a stream, returning the captured graph.

 @param [in] stream - Stream to end capture.
 @param [out] pGraph - Captured graph.

 @returns #hipSuccess, #hipErrorInvalidValue

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.
*/
    pub fn hipStreamEndCapture(
        stream: hipStream_t,
        pGraph: *mut hipGraph_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get capture status of a stream.

 @param [in] stream - Stream of which to get capture status from.
 @param [out] pCaptureStatus - Returns current capture status.
 @param [out] pId - Unique capture ID.

 @returns #hipSuccess, #hipErrorStreamCaptureImplicit

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.
*/
    pub fn hipStreamGetCaptureInfo(
        stream: hipStream_t,
        pCaptureStatus: *mut hipStreamCaptureStatus,
        pId: *mut ::core::ffi::c_ulonglong,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get stream's capture state

 @param [in] stream - Stream of which to get capture status from.
 @param [out] captureStatus_out - Returns current capture status.
 @param [out] id_out - Unique capture ID.
 @param [out] graph_out - Returns the graph being captured into.
 @param [out] dependencies_out - Pointer to an array of nodes representing the graphs dependencies.
 @param [out] numDependencies_out - Returns size of the array returned in dependencies_out.

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorStreamCaptureImplicit

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.
*/
    pub fn hipStreamGetCaptureInfo_v2(
        stream: hipStream_t,
        captureStatus_out: *mut hipStreamCaptureStatus,
        id_out: *mut ::core::ffi::c_ulonglong,
        graph_out: *mut hipGraph_t,
        dependencies_out: *mut *const hipGraphNode_t,
        numDependencies_out: *mut usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get stream's capture state

 @param [in] stream - Stream of which to get capture status from.
 @param [out] pCaptureStatus - Returns current capture status.

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorStreamCaptureImplicit

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.
*/
    pub fn hipStreamIsCapturing(
        stream: hipStream_t,
        pCaptureStatus: *mut hipStreamCaptureStatus,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Update the set of dependencies in a capturing stream

 @param [in] stream  Stream that is being captured.
 @param [in] dependencies  Pointer to an array of nodes to add/replace.
 @param [in] numDependencies  Size of the dependencies array.
 @param [in] flags  Flag to update dependency set. Should be one of the values
 in enum #hipStreamUpdateCaptureDependenciesFlags.
 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorIllegalState

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.
*/
    pub fn hipStreamUpdateCaptureDependencies(
        stream: hipStream_t,
        dependencies: *mut hipGraphNode_t,
        numDependencies: usize,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Swaps the stream capture mode of a thread.

 @param [in] mode - Pointer to mode value to swap with the current mode.
 @returns #hipSuccess, #hipErrorInvalidValue

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.
*/
    pub fn hipThreadExchangeStreamCaptureMode(
        mode: *mut hipStreamCaptureMode,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a graph

 @param [out] pGraph - pointer to graph to create.
 @param [in] flags - flags for graph creation, must be 0.

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorMemoryAllocation

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.
*/
    pub fn hipGraphCreate(
        pGraph: *mut hipGraph_t,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Destroys a graph

 @param [in] graph - instance of graph to destroy.

 @returns #hipSuccess, #hipErrorInvalidValue

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.
*/
    pub fn hipGraphDestroy(graph: hipGraph_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Adds dependency edges to a graph.

 @param [in] graph - Instance of the graph to add dependencies to.
 @param [in] from - Pointer to the graph nodes with dependencies to add from.
 @param [in] to - Pointer to the graph nodes to add dependencies to.
 @param [in] numDependencies - Number of dependencies to add.
 @returns #hipSuccess, #hipErrorInvalidValue

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.
*/
    pub fn hipGraphAddDependencies(
        graph: hipGraph_t,
        from: *const hipGraphNode_t,
        to: *const hipGraphNode_t,
        numDependencies: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Removes dependency edges from a graph.

 @param [in] graph - Instance of the graph to remove dependencies from.
 @param [in] from - Array of nodes that provide the dependencies.
 @param [in] to - Array of dependent nodes.
 @param [in] numDependencies - Number of dependencies to remove.
 @returns #hipSuccess, #hipErrorInvalidValue

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.
*/
    pub fn hipGraphRemoveDependencies(
        graph: hipGraph_t,
        from: *const hipGraphNode_t,
        to: *const hipGraphNode_t,
        numDependencies: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns a graph's dependency edges.

 @param [in] graph - Instance of the graph to get the edges from.
 @param [out] from - Pointer to the graph nodes to return edge endpoints.
 @param [out] to - Pointer to the graph nodes to return edge endpoints.
 @param [out] numEdges - Returns number of edges.
 @returns #hipSuccess, #hipErrorInvalidValue

 from and to may both be NULL, in which case this function only returns the number of edges in
 numEdges. Otherwise, numEdges entries will be filled in. If numEdges is higher than the actual
 number of edges, the remaining entries in from and to will be set to NULL, and the number of
 edges actually returned will be written to numEdges.
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.
*/
    pub fn hipGraphGetEdges(
        graph: hipGraph_t,
        from: *mut hipGraphNode_t,
        to: *mut hipGraphNode_t,
        numEdges: *mut usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns a graph's nodes.

 @param [in] graph - Instance of graph to get the nodes from.
 @param [out] nodes - Pointer to return the  graph nodes.
 @param [out] numNodes - Returns the number of graph nodes.
 @returns #hipSuccess, #hipErrorInvalidValue

 nodes may be NULL, in which case this function will return the number of nodes in numNodes.
 Otherwise, numNodes entries will be filled in. If numNodes is higher than the actual number of
 nodes, the remaining entries in nodes will be set to NULL, and the number of nodes actually
 obtained will be returned in numNodes.
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.
*/
    pub fn hipGraphGetNodes(
        graph: hipGraph_t,
        nodes: *mut hipGraphNode_t,
        numNodes: *mut usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns a graph's root nodes.

 @param [in] graph - Instance of the graph to get the nodes from.
 @param [out] pRootNodes - Pointer to return the graph's root nodes.
 @param [out] pNumRootNodes - Returns the number of graph's root nodes.
 @returns #hipSuccess, #hipErrorInvalidValue

 pRootNodes may be NULL, in which case this function will return the number of root nodes in
 pNumRootNodes. Otherwise, pNumRootNodes entries will be filled in. If pNumRootNodes is higher
 than the actual number of root nodes, the remaining entries in pRootNodes will be set to NULL,
 and the number of nodes actually obtained will be returned in pNumRootNodes.
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.
*/
    pub fn hipGraphGetRootNodes(
        graph: hipGraph_t,
        pRootNodes: *mut hipGraphNode_t,
        pNumRootNodes: *mut usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns a node's dependencies.

 @param [in] node - Graph node to get the dependencies from.
 @param [out] pDependencies - Pointer to return the dependencies.
 @param [out] pNumDependencies -  Returns the number of graph node dependencies.
 @returns #hipSuccess, #hipErrorInvalidValue

 pDependencies may be NULL, in which case this function will return the number of dependencies in
 pNumDependencies. Otherwise, pNumDependencies entries will be filled in. If pNumDependencies is
 higher than the actual number of dependencies, the remaining entries in pDependencies will be set
 to NULL, and the number of nodes actually obtained will be returned in pNumDependencies.
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.
*/
    pub fn hipGraphNodeGetDependencies(
        node: hipGraphNode_t,
        pDependencies: *mut hipGraphNode_t,
        pNumDependencies: *mut usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns a node's dependent nodes.

 @param [in] node - Graph node to get the dependent nodes from.
 @param [out] pDependentNodes - Pointer to return the graph dependent nodes.
 @param [out] pNumDependentNodes - Returns the number of graph node dependent nodes.
 @returns #hipSuccess, #hipErrorInvalidValue

 pDependentNodes may be NULL, in which case this function will return the number of dependent nodes
 in pNumDependentNodes. Otherwise, pNumDependentNodes entries will be filled in. If
 pNumDependentNodes is higher than the actual number of dependent nodes, the remaining entries in
 pDependentNodes will be set to NULL, and the number of nodes actually obtained will be returned
 in pNumDependentNodes.
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.
*/
    pub fn hipGraphNodeGetDependentNodes(
        node: hipGraphNode_t,
        pDependentNodes: *mut hipGraphNode_t,
        pNumDependentNodes: *mut usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns a node's type.

 @param [in] node - Node to get type of.
 @param [out] pType - Returns the node's type.
 @returns #hipSuccess, #hipErrorInvalidValue

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.
*/
    pub fn hipGraphNodeGetType(
        node: hipGraphNode_t,
        pType: *mut hipGraphNodeType,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Remove a node from the graph.

 @param [in] node - graph node to remove
 @returns #hipSuccess, #hipErrorInvalidValue

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.
*/
    pub fn hipGraphDestroyNode(node: hipGraphNode_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Clones a graph.

 @param [out] pGraphClone - Returns newly created cloned graph.
 @param [in] originalGraph - original graph to clone from.
 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorMemoryAllocation

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.
*/
    pub fn hipGraphClone(
        pGraphClone: *mut hipGraph_t,
        originalGraph: hipGraph_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Finds a cloned version of a node.

 @param [out] pNode - Returns the cloned node.
 @param [in] originalNode - original node handle.
 @param [in] clonedGraph - Cloned graph to query.
 @returns #hipSuccess, #hipErrorInvalidValue

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.
*/
    pub fn hipGraphNodeFindInClone(
        pNode: *mut hipGraphNode_t,
        originalNode: hipGraphNode_t,
        clonedGraph: hipGraph_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates an executable graph from a graph

 @param [out] pGraphExec - Pointer to instantiated executable graph.
 @param [in] graph - Instance of graph to instantiate.
 @param [out] pErrorNode - Pointer to error node. In case an error occured during
 graph instantiation, it could modify the corresponding node.
 @param [out] pLogBuffer - Pointer to log buffer.
 @param [out] bufferSize - Size of the log buffer.

 @returns #hipSuccess, #hipErrorOutOfMemory

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.
*/
    pub fn hipGraphInstantiate(
        pGraphExec: *mut hipGraphExec_t,
        graph: hipGraph_t,
        pErrorNode: *mut hipGraphNode_t,
        pLogBuffer: *mut ::core::ffi::c_char,
        bufferSize: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates an executable graph from a graph.

 @param [out] pGraphExec - Pointer to instantiated executable graph.
 @param [in] graph - Instance of graph to instantiate.
 @param [in] flags - Flags to control instantiation.
 @returns #hipSuccess, #hipErrorInvalidValue

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues. It does not support any of
          flag and is behaving as hipGraphInstantiate.*/
    pub fn hipGraphInstantiateWithFlags(
        pGraphExec: *mut hipGraphExec_t,
        graph: hipGraph_t,
        flags: ::core::ffi::c_ulonglong,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates an executable graph from a graph.

 @param [out] pGraphExec - Pointer to instantiated executable graph.
 @param [in] graph - Instance of graph to instantiate.
 @param [in] instantiateParams - Graph instantiation Params
 @returns #hipSuccess, #hipErrorInvalidValue

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphInstantiateWithParams(
        pGraphExec: *mut hipGraphExec_t,
        graph: hipGraph_t,
        instantiateParams: *mut hipGraphInstantiateParams,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Launches an executable graph in the specified stream.

 @param [in] graphExec - Instance of executable graph to launch.
 @param [in] stream - Instance of stream in which to launch executable graph.
 @returns #hipSuccess, #hipErrorInvalidValue

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphLaunch(graphExec: hipGraphExec_t, stream: hipStream_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Uploads an executable graph to a stream

 @param [in] graphExec - Instance of executable graph to be uploaded.
 @param [in] stream - Instance of stream to which the executable graph is uploaded to.
 @returns #hipSuccess, #hipErrorInvalidValue

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphUpload(graphExec: hipGraphExec_t, stream: hipStream_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a kernel execution node and adds it to a graph.

 @param [out] pGraphNode - Pointer to kernel graph node that is created.
 @param [in] graph - Instance of graph to add the created node to.
 @param [in] pDependencies - Pointer to the dependencies on the kernel execution node.
 @param [in] numDependencies - Number of dependencies.
 @param [in] nodeParams - Pointer to the node parameters.
 @returns #hipSuccess, #hipErrorInvalidValue.
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphAddNode(
        pGraphNode: *mut hipGraphNode_t,
        graph: hipGraph_t,
        pDependencies: *const hipGraphNode_t,
        numDependencies: usize,
        nodeParams: *mut hipGraphNodeParams,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Return the flags of an executable graph.

 @param [in] graphExec - Executable graph to get the flags from.
 @param [out] flags - Flags used to instantiate this executable graph.
 @returns #hipSuccess, #hipErrorInvalidValue.
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphExecGetFlags(
        graphExec: hipGraphExec_t,
        flags: *mut ::core::ffi::c_ulonglong,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Updates parameters of a graph's node.

 @param [in] node - Instance of the node to set parameters for.
 @param [in] nodeParams - Pointer to the parameters to be set.
 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidDeviceFunction, #hipErrorNotSupported.
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphNodeSetParams(
        node: hipGraphNode_t,
        nodeParams: *mut hipGraphNodeParams,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Updates parameters of an executable graph's node.

 @param [in] graphExec - Instance of the executable graph.
 @param [in] node - Instance of the node to set parameters to.
 @param [in] nodeParams - Pointer to the parameters to be set.
 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidDeviceFunction, #hipErrorNotSupported.
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphExecNodeSetParams(
        graphExec: hipGraphExec_t,
        node: hipGraphNode_t,
        nodeParams: *mut hipGraphNodeParams,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Destroys an executable graph

 @param [in] graphExec - Instance of executable graph to destroy.

 @returns #hipSuccess.

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphExecDestroy(graphExec: hipGraphExec_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Check whether an executable graph can be updated with a graph and perform the update if  *
 possible.

 @param [in] hGraphExec - instance of executable graph to update.
 @param [in] hGraph - graph that contains the updated parameters.
 @param [in] hErrorNode_out -  node which caused the permissibility check to forbid the update.
 @param [in] updateResult_out - Return code whether the graph update was performed.
 @returns #hipSuccess, #hipErrorGraphExecUpdateFailure

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphExecUpdate(
        hGraphExec: hipGraphExec_t,
        hGraph: hipGraph_t,
        hErrorNode_out: *mut hipGraphNode_t,
        updateResult_out: *mut hipGraphExecUpdateResult,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a kernel execution node and adds it to a graph.

 @param [out] pGraphNode - Pointer to graph node that is created
 @param [in] graph - Instance of graph to add the created node to.
 @param [in] pDependencies - Pointer to the dependencies of the kernel execution node.
 @param [in] numDependencies - The number of the dependencies.
 @param [in] pNodeParams - Pointer to the parameters of the kernel execution node.
 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorInvalidDeviceFunction
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphAddKernelNode(
        pGraphNode: *mut hipGraphNode_t,
        graph: hipGraph_t,
        pDependencies: *const hipGraphNode_t,
        numDependencies: usize,
        pNodeParams: *const hipKernelNodeParams,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets kernel node's parameters.

 @param [in] node - instance of the node to get parameters from.
 @param [out] pNodeParams - pointer to the parameters
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphKernelNodeGetParams(
        node: hipGraphNode_t,
        pNodeParams: *mut hipKernelNodeParams,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets a kernel node's parameters.

 @param [in] node - Instance of the node to set parameters of.
 @param [in] pNodeParams - const pointer to the parameters.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphKernelNodeSetParams(
        node: hipGraphNode_t,
        pNodeParams: *const hipKernelNodeParams,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets the parameters for a kernel node in the given graphExec.

 @param [in] hGraphExec - Instance of the executable graph with the node.
 @param [in] node - Instance of the node to set parameters of.
 @param [in] pNodeParams - const pointer to the kernel node parameters.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphExecKernelNodeSetParams(
        hGraphExec: hipGraphExec_t,
        node: hipGraphNode_t,
        pNodeParams: *const hipKernelNodeParams,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a memcpy node and adds it to a graph.

 @param [out] phGraphNode - Pointer to graph node that is created.
 @param [in] hGraph - Instance of graph to add the created node to.
 @param [in] dependencies - const pointer to the dependencies of the memcpy execution node.
 @param [in] numDependencies - The number of dependencies.
 @param [in] copyParams - const pointer to the parameters for the memory copy.
 @param [in] ctx - context related to current device.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipDrvGraphAddMemcpyNode(
        phGraphNode: *mut hipGraphNode_t,
        hGraph: hipGraph_t,
        dependencies: *const hipGraphNode_t,
        numDependencies: usize,
        copyParams: *const HIP_MEMCPY3D,
        ctx: hipCtx_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a memcpy node and adds it to a graph.

 @param [out] pGraphNode - Pointer to graph node that is created.
 @param [in] graph - Instance of graph to add the created node to.
 @param [in] pDependencies - const pointer to the dependencies of the memcpy execution node.
 @param [in] numDependencies - The number of dependencies.
 @param [in] pCopyParams - const pointer to the parameters for the memory copy.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphAddMemcpyNode(
        pGraphNode: *mut hipGraphNode_t,
        graph: hipGraph_t,
        pDependencies: *const hipGraphNode_t,
        numDependencies: usize,
        pCopyParams: *const hipMemcpy3DParms,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets a memcpy node's parameters.

 @param [in] node - instance of the node to get parameters from.
 @param [out] pNodeParams - pointer to the parameters.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphMemcpyNodeGetParams(
        node: hipGraphNode_t,
        pNodeParams: *mut hipMemcpy3DParms,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets a memcpy node's parameters.

 @param [in] node - instance of the node to set parameters to.
 @param [in] pNodeParams - const pointer to the parameters.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphMemcpyNodeSetParams(
        node: hipGraphNode_t,
        pNodeParams: *const hipMemcpy3DParms,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets a node's attribute.

 @param [in] hNode - Instance of the node to set parameters of.
 @param [in] attr - The attribute type to be set.
 @param [in] value - const pointer to the parameters.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphKernelNodeSetAttribute(
        hNode: hipGraphNode_t,
        attr: hipLaunchAttributeID,
        value: *const hipLaunchAttributeValue,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets a node's attribute.

 @param [in] hNode - Instance of the node to set parameters of.
 @param [in] attr - The attribute type to be set.
 @param [in] value - const pointer to the parameters.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphKernelNodeGetAttribute(
        hNode: hipGraphNode_t,
        attr: hipLaunchAttributeID,
        value: *mut hipLaunchAttributeValue,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets the parameters of a memcpy node in the given graphExec.

 @param [in] hGraphExec - Instance of the executable graph with the node.
 @param [in] node - Instance of the node to set parameters of.
 @param [in] pNodeParams - const pointer to the kernel node parameters.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphExecMemcpyNodeSetParams(
        hGraphExec: hipGraphExec_t,
        node: hipGraphNode_t,
        pNodeParams: *mut hipMemcpy3DParms,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a 1D memcpy node and adds it to a graph.

 @param [out] pGraphNode - Pointer to graph node that is created.
 @param [in] graph - Instance of graph to add the created node to.
 @param [in] pDependencies - const pointer to the dependencies of the memcpy execution node.
 @param [in] numDependencies - The number of dependencies.
 @param [in] dst - Pointer to memory address of the destination.
 @param [in] src - Pointer to memory address of the source.
 @param [in] count - Size of the memory to copy.
 @param [in] kind - Type of memory copy.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphAddMemcpyNode1D(
        pGraphNode: *mut hipGraphNode_t,
        graph: hipGraph_t,
        pDependencies: *const hipGraphNode_t,
        numDependencies: usize,
        dst: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        count: usize,
        kind: hipMemcpyKind,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets a memcpy node's parameters to perform a 1-dimensional copy.

 @param [in] node - Instance of the node to set parameters of.
 @param [in] dst - Pointer to memory address of the destination.
 @param [in] src - Pointer to memory address of the source.
 @param [in] count - Size of the memory to copy.
 @param [in] kind - Type of memory copy.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphMemcpyNodeSetParams1D(
        node: hipGraphNode_t,
        dst: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        count: usize,
        kind: hipMemcpyKind,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets the parameters for a memcpy node in the given graphExec to perform a 1-dimensional
 copy.

 @param [in] hGraphExec - Instance of the executable graph with the node.
 @param [in] node - Instance of the node to set parameters of.
 @param [in] dst - Pointer to memory address of the destination.
 @param [in] src - Pointer to memory address of the source.
 @param [in] count - Size of the memory to copy.
 @param [in] kind - Type of memory copy.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphExecMemcpyNodeSetParams1D(
        hGraphExec: hipGraphExec_t,
        node: hipGraphNode_t,
        dst: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        count: usize,
        kind: hipMemcpyKind,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a memcpy node to copy from a symbol on the device and adds it to a graph.

 @param [out] pGraphNode - Pointer to graph node that is created.
 @param [in] graph - Instance of graph to add the created node to.
 @param [in] pDependencies - const pointer to the dependencies of the memcpy execution node.
 @param [in] numDependencies - Number of the dependencies.
 @param [in] dst - Pointer to memory address of the destination.
 @param [in] symbol - Device symbol address.
 @param [in] count - Size of the memory to copy.
 @param [in] offset - Offset from start of symbol in bytes.
 @param [in] kind - Type of memory copy.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphAddMemcpyNodeFromSymbol(
        pGraphNode: *mut hipGraphNode_t,
        graph: hipGraph_t,
        pDependencies: *const hipGraphNode_t,
        numDependencies: usize,
        dst: *mut ::core::ffi::c_void,
        symbol: *const ::core::ffi::c_void,
        count: usize,
        offset: usize,
        kind: hipMemcpyKind,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets a memcpy node's parameters to copy from a symbol on the device.

 @param [in] node - Instance of the node to set parameters of.
 @param [in] dst - Pointer to memory address of the destination.
 @param [in] symbol - Device symbol address.
 @param [in] count - Size of the memory to copy.
 @param [in] offset - Offset from start of symbol in bytes.
 @param [in] kind - Type of memory copy.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphMemcpyNodeSetParamsFromSymbol(
        node: hipGraphNode_t,
        dst: *mut ::core::ffi::c_void,
        symbol: *const ::core::ffi::c_void,
        count: usize,
        offset: usize,
        kind: hipMemcpyKind,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets the parameters for a memcpy node in the given graphExec to copy from a symbol on the
 * device.

 @param [in] hGraphExec - Instance of the executable graph with the node.
 @param [in] node - Instance of the node to set parameters of.
 @param [in] dst - Pointer to memory address of the destination.
 @param [in] symbol - Device symbol address.
 @param [in] count - Size of the memory to copy.
 @param [in] offset - Offset from start of symbol in bytes.
 @param [in] kind - Type of memory copy.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphExecMemcpyNodeSetParamsFromSymbol(
        hGraphExec: hipGraphExec_t,
        node: hipGraphNode_t,
        dst: *mut ::core::ffi::c_void,
        symbol: *const ::core::ffi::c_void,
        count: usize,
        offset: usize,
        kind: hipMemcpyKind,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a memcpy node to copy to a symbol on the device and adds it to a graph.

 @param [out] pGraphNode - Pointer to graph node that is created.
 @param [in] graph - Instance of graph to add the created node to.
 @param [in] pDependencies - const pointer to the dependencies on the memcpy execution node.
 @param [in] numDependencies - Number of dependencies.
 @param [in] symbol - Device symbol address.
 @param [in] src - Pointer to memory address of the src.
 @param [in] count - Size of the memory to copy.
 @param [in] offset - Offset from start of symbol in bytes.
 @param [in] kind - Type of memory copy.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphAddMemcpyNodeToSymbol(
        pGraphNode: *mut hipGraphNode_t,
        graph: hipGraph_t,
        pDependencies: *const hipGraphNode_t,
        numDependencies: usize,
        symbol: *const ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        count: usize,
        offset: usize,
        kind: hipMemcpyKind,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets a memcpy node's parameters to copy to a symbol on the device.

 @param [in] node - Instance of the node to set parameters of.
 @param [in] symbol - Device symbol address.
 @param [in] src - Pointer to memory address of the src.
 @param [in] count - Size of the memory to copy.
 @param [in] offset - Offset from start of symbol in bytes.
 @param [in] kind - Type of memory copy.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphMemcpyNodeSetParamsToSymbol(
        node: hipGraphNode_t,
        symbol: *const ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        count: usize,
        offset: usize,
        kind: hipMemcpyKind,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets the parameters for a memcpy node in the given graphExec to copy to a symbol on the
 device.
 @param [in] hGraphExec - Instance of the executable graph with the node.
 @param [in] node - Instance of the node to set parameters of.
 @param [in] symbol - Device symbol address.
 @param [in] src - Pointer to memory address of the src.
 @param [in] count - Size of the memory to copy.
 @param [in] offset - Offset from start of symbol in bytes.
 @param [in] kind - Type of memory copy.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphExecMemcpyNodeSetParamsToSymbol(
        hGraphExec: hipGraphExec_t,
        node: hipGraphNode_t,
        symbol: *const ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        count: usize,
        offset: usize,
        kind: hipMemcpyKind,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a memset node and adds it to a graph.

 @param [out] pGraphNode - Pointer to graph node that is created.
 @param [in] graph - Instance of the graph to add the created node to.
 @param [in] pDependencies - const pointer to the dependencies on the memset execution node.
 @param [in] numDependencies - Number of dependencies.
 @param [in] pMemsetParams - const pointer to the parameters for the memory set.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphAddMemsetNode(
        pGraphNode: *mut hipGraphNode_t,
        graph: hipGraph_t,
        pDependencies: *const hipGraphNode_t,
        numDependencies: usize,
        pMemsetParams: *const hipMemsetParams,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets a memset node's parameters.

 @param [in] node - Instance of the node to get parameters of.
 @param [out] pNodeParams - Pointer to the parameters.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphMemsetNodeGetParams(
        node: hipGraphNode_t,
        pNodeParams: *mut hipMemsetParams,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets a memset node's parameters.

 @param [in] node - Instance of the node to set parameters of.
 @param [in] pNodeParams - Pointer to the parameters.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphMemsetNodeSetParams(
        node: hipGraphNode_t,
        pNodeParams: *const hipMemsetParams,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets the parameters for a memset node in the given graphExec.

 @param [in] hGraphExec - Instance of the executable graph with the node.
 @param [in] node - Instance of the node to set parameters of.
 @param [in] pNodeParams - Pointer to the parameters.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphExecMemsetNodeSetParams(
        hGraphExec: hipGraphExec_t,
        node: hipGraphNode_t,
        pNodeParams: *const hipMemsetParams,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a host execution node and adds it to a graph.

 @param [out] pGraphNode - Pointer to graph node that is created.
 @param [in] graph - Instance of the graph to add the created node to.
 @param [in] pDependencies - const pointer to the dependencies of the memset execution node.
 @param [in] numDependencies - Number of dependencies.
 @param [in] pNodeParams - Pointer to the parameters.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphAddHostNode(
        pGraphNode: *mut hipGraphNode_t,
        graph: hipGraph_t,
        pDependencies: *const hipGraphNode_t,
        numDependencies: usize,
        pNodeParams: *const hipHostNodeParams,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns a host node's parameters.

 @param [in] node - Instance of the node to get parameters of.
 @param [out] pNodeParams - Pointer to the parameters.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphHostNodeGetParams(
        node: hipGraphNode_t,
        pNodeParams: *mut hipHostNodeParams,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets a host node's parameters.

 @param [in] node - Instance of the node to set parameters of.
 @param [in] pNodeParams - Pointer to the parameters.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphHostNodeSetParams(
        node: hipGraphNode_t,
        pNodeParams: *const hipHostNodeParams,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets the parameters for a host node in the given graphExec.

 @param [in] hGraphExec - Instance of the executable graph with the node.
 @param [in] node - Instance of the node to set parameters of.
 @param [in] pNodeParams - Pointer to the parameters.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphExecHostNodeSetParams(
        hGraphExec: hipGraphExec_t,
        node: hipGraphNode_t,
        pNodeParams: *const hipHostNodeParams,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a child graph node and adds it to a graph.

 @param [out] pGraphNode - Pointer to graph node that is created.
 @param [in] graph - Instance of the graph to add the created node.
 @param [in] pDependencies - const pointer to the dependencies of the memset execution node.
 @param [in] numDependencies - Number of dependencies.
 @param [in] childGraph - Graph to clone into this node
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphAddChildGraphNode(
        pGraphNode: *mut hipGraphNode_t,
        graph: hipGraph_t,
        pDependencies: *const hipGraphNode_t,
        numDependencies: usize,
        childGraph: hipGraph_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets a handle to the embedded graph of a child graph node.

 @param [in] node - Instance of the node to get child graph of.
 @param [out] pGraph - Pointer to get the graph.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphChildGraphNodeGetGraph(
        node: hipGraphNode_t,
        pGraph: *mut hipGraph_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Updates node parameters in the child graph node in the given graphExec.

 @param [in] hGraphExec - instance of the executable graph with the node.
 @param [in] node - node from the graph which was used to instantiate graphExec.
 @param [in] childGraph - child graph with updated parameters.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphExecChildGraphNodeSetParams(
        hGraphExec: hipGraphExec_t,
        node: hipGraphNode_t,
        childGraph: hipGraph_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates an empty node and adds it to a graph.

 @param [out] pGraphNode - Pointer to graph node that is created.
 @param [in] graph - Instance of the graph the node is added to.
 @param [in] pDependencies - const pointer to the node dependencies.
 @param [in] numDependencies - Number of dependencies.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphAddEmptyNode(
        pGraphNode: *mut hipGraphNode_t,
        graph: hipGraph_t,
        pDependencies: *const hipGraphNode_t,
        numDependencies: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates an event record node and adds it to a graph.

 @param [out] pGraphNode - Pointer to graph node that is created.
 @param [in] graph - Instance of the graph the node is added to.
 @param [in] pDependencies - const pointer to the node dependencies.
 @param [in] numDependencies - Number of dependencies.
 @param [in] event - Event of the node.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphAddEventRecordNode(
        pGraphNode: *mut hipGraphNode_t,
        graph: hipGraph_t,
        pDependencies: *const hipGraphNode_t,
        numDependencies: usize,
        event: hipEvent_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns the event associated with an event record node.

 @param [in] node -  Instance of the node to get event of.
 @param [out] event_out - Pointer to return the event.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphEventRecordNodeGetEvent(
        node: hipGraphNode_t,
        event_out: *mut hipEvent_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets an event record node's event.

 @param [in] node - Instance of the node to set event to.
 @param [in] event - Pointer to the event.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphEventRecordNodeSetEvent(
        node: hipGraphNode_t,
        event: hipEvent_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets the event for an event record node in the given graphExec.

 @param [in] hGraphExec - instance of the executable graph with the node.
 @param [in] hNode - node from the graph which was used to instantiate graphExec.
 @param [in] event - pointer to the event.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphExecEventRecordNodeSetEvent(
        hGraphExec: hipGraphExec_t,
        hNode: hipGraphNode_t,
        event: hipEvent_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates an event wait node and adds it to a graph.

 @param [out] pGraphNode - Pointer to graph node that is created.
 @param [in] graph - Instance of the graph the node to be added.
 @param [in] pDependencies - const pointer to the node dependencies.
 @param [in] numDependencies - Number of dependencies.
 @param [in] event - Event for the node.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphAddEventWaitNode(
        pGraphNode: *mut hipGraphNode_t,
        graph: hipGraph_t,
        pDependencies: *const hipGraphNode_t,
        numDependencies: usize,
        event: hipEvent_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns the event associated with an event wait node.

 @param [in] node -  Instance of the node to get event of.
 @param [out] event_out - Pointer to return the event.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphEventWaitNodeGetEvent(
        node: hipGraphNode_t,
        event_out: *mut hipEvent_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets an event wait node's event.

 @param [in] node - Instance of the node to set event of.
 @param [in] event - Pointer to the event.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphEventWaitNodeSetEvent(
        node: hipGraphNode_t,
        event: hipEvent_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets the event for an event record node in the given graphExec.

 @param [in] hGraphExec - instance of the executable graph with the node.
 @param [in] hNode - node from the graph which was used to instantiate graphExec.
 @param [in] event - pointer to the event.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphExecEventWaitNodeSetEvent(
        hGraphExec: hipGraphExec_t,
        hNode: hipGraphNode_t,
        event: hipEvent_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a memory allocation node and adds it to a graph

 @param [out] pGraphNode      - Pointer to the graph node to create and add to the graph
 @param [in] graph            - Instance of the graph node to be added
 @param [in] pDependencies    - Const pointer to the node dependencies
 @param [in] numDependencies  - The number of dependencies
 @param [in, out] pNodeParams - Node parameters for memory allocation, returns a pointer to the allocated memory.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphAddMemAllocNode(
        pGraphNode: *mut hipGraphNode_t,
        graph: hipGraph_t,
        pDependencies: *const hipGraphNode_t,
        numDependencies: usize,
        pNodeParams: *mut hipMemAllocNodeParams,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns parameters for memory allocation node

 @param [in] node         - Memory allocation node to query
 @param [out] pNodeParams - Parameters for the specified memory allocation node
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphMemAllocNodeGetParams(
        node: hipGraphNode_t,
        pNodeParams: *mut hipMemAllocNodeParams,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a memory free node and adds it to a graph

 @param [out] pGraphNode      - Pointer to the graph node to create and add to the graph
 @param [in] graph            - Instance of the graph node to be added
 @param [in] pDependencies    - Const pointer to the node dependencies
 @param [in] numDependencies  - The number of dependencies
 @param [in] dev_ptr          - Pointer to the memory to be freed
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphAddMemFreeNode(
        pGraphNode: *mut hipGraphNode_t,
        graph: hipGraph_t,
        pDependencies: *const hipGraphNode_t,
        numDependencies: usize,
        dev_ptr: *mut ::core::ffi::c_void,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns parameters for memory free node

 @param [in] node     - Memory free node to query
 @param [out] dev_ptr - Device pointer of the specified memory free node
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphMemFreeNodeGetParams(
        node: hipGraphNode_t,
        dev_ptr: *mut ::core::ffi::c_void,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get the mem attribute for graphs.

 @param [in] device - Device to get attributes from
 @param [in] attr - Attribute type to be queried
 @param [out] value - Value of the queried attribute
 @returns #hipSuccess, #hipErrorInvalidDevice
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipDeviceGetGraphMemAttribute(
        device: ::core::ffi::c_int,
        attr: hipGraphMemAttributeType,
        value: *mut ::core::ffi::c_void,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set the mem attribute for graphs.

 @param [in] device - Device to set attribute of.
 @param [in] attr - Attribute type to be set.
 @param [in] value - Value of the attribute.
 @returns #hipSuccess, #hipErrorInvalidDevice
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipDeviceSetGraphMemAttribute(
        device: ::core::ffi::c_int,
        attr: hipGraphMemAttributeType,
        value: *mut ::core::ffi::c_void,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Free unused memory reserved for graphs on a specific device and return it back to the OS.

 @param [in] device - Device for which memory should be trimmed
 @returns #hipSuccess, #hipErrorInvalidDevice

 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipDeviceGraphMemTrim(device: ::core::ffi::c_int) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Create an instance of userObject to manage lifetime of a resource.

 @param [out] object_out - pointer to instace of userobj.
 @param [in] ptr - pointer to pass to destroy function.
 @param [in] destroy - destroy callback to remove resource.
 @param [in] initialRefcount - reference to resource.
 @param [in] flags - flags passed to API.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipUserObjectCreate(
        object_out: *mut hipUserObject_t,
        ptr: *mut ::core::ffi::c_void,
        destroy: hipHostFn_t,
        initialRefcount: ::core::ffi::c_uint,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Release number of references to resource.

 @param [in] object - pointer to instace of userobj.
 @param [in] count - reference to resource to be retained.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipUserObjectRelease(
        object: hipUserObject_t,
        count: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Retain number of references to resource.

 @param [in] object - pointer to instace of userobj.
 @param [in] count - reference to resource to be retained.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipUserObjectRetain(
        object: hipUserObject_t,
        count: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Retain user object for graphs.

 @param [in] graph - pointer to graph to retain the user object for.
 @param [in] object - pointer to instace of userobj.
 @param [in] count - reference to resource to be retained.
 @param [in] flags - flags passed to API.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphRetainUserObject(
        graph: hipGraph_t,
        object: hipUserObject_t,
        count: ::core::ffi::c_uint,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Release user object from graphs.

 @param [in] graph - pointer to graph to retain the user object for.
 @param [in] object - pointer to instace of userobj.
 @param [in] count - reference to resource to be retained.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphReleaseUserObject(
        graph: hipGraph_t,
        object: hipUserObject_t,
        count: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Write a DOT file describing graph structure.

 @param [in] graph - graph object for which DOT file has to be generated.
 @param [in] path - path to write the DOT file.
 @param [in] flags - Flags from hipGraphDebugDotFlags to get additional node information.
 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorOperatingSystem
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphDebugDotPrint(
        graph: hipGraph_t,
        path: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Copies attributes from source node to destination node.

 Copies attributes from source node to destination node.
 Both node must have the same context.

 @param [out] hDst - Destination node.
 @param [in] hSrc - Source node.
 For list of attributes see ::hipKernelNodeAttrID.

 @returns #hipSuccess, #hipErrorInvalidContext
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphKernelNodeCopyAttributes(
        hSrc: hipGraphNode_t,
        hDst: hipGraphNode_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Enables or disables the specified node in the given graphExec

 Sets hNode to be either enabled or disabled. Disabled nodes are functionally equivalent
 to empty nodes until they are reenabled. Existing node parameters are not affected by
 disabling/enabling the node.

 The node is identified by the corresponding hNode in the non-executable graph, from which the
 executable graph was instantiated.

 hNode must not have been removed from the original graph.

 @note Currently only kernel, memset and memcpy nodes are supported.

 @param [in] hGraphExec - The executable graph in which to set the specified node.
 @param [in] hNode      - Node from the graph from which graphExec was instantiated.
 @param [in] isEnabled  - Node is enabled if != 0, otherwise the node is disabled.

 @returns #hipSuccess, #hipErrorInvalidValue,
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphNodeSetEnabled(
        hGraphExec: hipGraphExec_t,
        hNode: hipGraphNode_t,
        isEnabled: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Query whether a node in the given graphExec is enabled

 Sets isEnabled to 1 if hNode is enabled, or 0 if it is disabled.

 The node is identified by the corresponding node in the non-executable graph, from which the
 executable graph was instantiated.

 hNode must not have been removed from the original graph.

 @note Currently only kernel, memset and memcpy nodes are supported.

 @param [in]  hGraphExec - The executable graph in which to set the specified node.
 @param [in]  hNode      - Node from the graph from which graphExec was instantiated.
 @param [out] isEnabled  - Location to return the enabled status of the node.

 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphNodeGetEnabled(
        hGraphExec: hipGraphExec_t,
        hNode: hipGraphNode_t,
        isEnabled: *mut ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a external semaphor wait node and adds it to a graph.

 @param [out] pGraphNode - pointer to the graph node to create.
 @param [in] graph - instance of the graph to add the created node.
 @param [in] pDependencies - const pointer to the dependencies on the memset execution node.
 @param [in] numDependencies - the number of the dependencies.
 @param [in] nodeParams -pointer to the parameters.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphAddExternalSemaphoresWaitNode(
        pGraphNode: *mut hipGraphNode_t,
        graph: hipGraph_t,
        pDependencies: *const hipGraphNode_t,
        numDependencies: usize,
        nodeParams: *const hipExternalSemaphoreWaitNodeParams,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a external semaphor signal node and adds it to a graph.

 @param [out] pGraphNode - pointer to the graph node to create.
 @param [in] graph - instance of the graph to add the created node.
 @param [in] pDependencies - const pointer to the dependencies on the memset execution node.
 @param [in] numDependencies - the number of the dependencies.
 @param [in] nodeParams -pointer to the parameters.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphAddExternalSemaphoresSignalNode(
        pGraphNode: *mut hipGraphNode_t,
        graph: hipGraph_t,
        pDependencies: *const hipGraphNode_t,
        numDependencies: usize,
        nodeParams: *const hipExternalSemaphoreSignalNodeParams,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Updates node parameters in the external semaphore signal node.

 @param [in]  hNode      - Node from the graph from which graphExec was instantiated.
 @param [in]  nodeParams  - Pointer to the params to be set.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphExternalSemaphoresSignalNodeSetParams(
        hNode: hipGraphNode_t,
        nodeParams: *const hipExternalSemaphoreSignalNodeParams,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Updates node parameters in the external semaphore wait node.

 @param [in]  hNode      - Node from the graph from which graphExec was instantiated.
 @param [in]  nodeParams  - Pointer to the params to be set.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphExternalSemaphoresWaitNodeSetParams(
        hNode: hipGraphNode_t,
        nodeParams: *const hipExternalSemaphoreWaitNodeParams,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns external semaphore signal node params.

 @param [in]   hNode       - Node from the graph from which graphExec was instantiated.
 @param [out]  params_out  - Pointer to params.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphExternalSemaphoresSignalNodeGetParams(
        hNode: hipGraphNode_t,
        params_out: *mut hipExternalSemaphoreSignalNodeParams,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns external semaphore wait node params.

 @param [in]   hNode       - Node from the graph from which graphExec was instantiated.
 @param [out]  params_out  - Pointer to params.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphExternalSemaphoresWaitNodeGetParams(
        hNode: hipGraphNode_t,
        params_out: *mut hipExternalSemaphoreWaitNodeParams,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Updates node parameters in the external semaphore signal node in the given graphExec.

 @param [in]  hGraphExec - The executable graph in which to set the specified node.
 @param [in]  hNode      - Node from the graph from which graphExec was instantiated.
 @param [in]  nodeParams  - Pointer to the params to be set.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphExecExternalSemaphoresSignalNodeSetParams(
        hGraphExec: hipGraphExec_t,
        hNode: hipGraphNode_t,
        nodeParams: *const hipExternalSemaphoreSignalNodeParams,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Updates node parameters in the external semaphore wait node in the given graphExec.

 @param [in]  hGraphExec - The executable graph in which to set the specified node.
 @param [in]  hNode      - Node from the graph from which graphExec was instantiated.
 @param [in]  nodeParams  - Pointer to the params to be set.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipGraphExecExternalSemaphoresWaitNodeSetParams(
        hGraphExec: hipGraphExec_t,
        hNode: hipGraphNode_t,
        nodeParams: *const hipExternalSemaphoreWaitNodeParams,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets a memcpy node's parameters.

 @param [in] hNode - instance of the node to get parameters from.
 @param [out] nodeParams - pointer to the parameters.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipDrvGraphMemcpyNodeGetParams(
        hNode: hipGraphNode_t,
        nodeParams: *mut HIP_MEMCPY3D,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets a memcpy node's parameters.

 @param [in] hNode - instance of the node to Set parameters for.
 @param [out] nodeParams - pointer to the parameters.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipDrvGraphMemcpyNodeSetParams(
        hNode: hipGraphNode_t,
        nodeParams: *const HIP_MEMCPY3D,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a memory free node and adds it to a graph

 @param [out] phGraphNode - Pointer to the graph node to create and add to the graph
 @param [in]  hGraph - Instance of the graph the node to be added
 @param [in]  dependencies - Const pointer to the node dependencies
 @param [in]  numDependencies - The number of dependencies
 @param [in]  dptr - Pointer to the memory to be freed
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipDrvGraphAddMemFreeNode(
        phGraphNode: *mut hipGraphNode_t,
        hGraph: hipGraph_t,
        dependencies: *const hipGraphNode_t,
        numDependencies: usize,
        dptr: hipDeviceptr_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Sets the parameters for a memcpy node in the given graphExec.

 @param [in] hGraphExec - instance of the executable graph with the node.
 @param [in] hNode - instance of the node to set parameters to.
 @param [in] copyParams - const pointer to the memcpy node params.
 @param [in] ctx - cotext related to current device.
 @returns #hipSuccess, #hipErrorInvalidValue
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.*/
    pub fn hipDrvGraphExecMemcpyNodeSetParams(
        hGraphExec: hipGraphExec_t,
        hNode: hipGraphNode_t,
        copyParams: *const HIP_MEMCPY3D,
        ctx: hipCtx_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Frees an address range reservation made via hipMemAddressReserve

 @param [in] devPtr - starting address of the range.
 @param [in] size - size of the range.
 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipMemAddressFree(
        devPtr: *mut ::core::ffi::c_void,
        size: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Reserves an address range

 @param [out] ptr - starting address of the reserved range.
 @param [in] size - size of the reservation.
 @param [in] alignment - alignment of the address.
 @param [in] addr - requested starting address of the range.
 @param [in] flags - currently unused, must be zero.
 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipMemAddressReserve(
        ptr: *mut *mut ::core::ffi::c_void,
        size: usize,
        alignment: usize,
        addr: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_ulonglong,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Creates a memory allocation described by the properties and size

 @param [out] handle - value of the returned handle.
 @param [in] size - size of the allocation.
 @param [in] prop - properties of the allocation.
 @param [in] flags - currently unused, must be zero.
 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipMemCreate(
        handle: *mut hipMemGenericAllocationHandle_t,
        size: usize,
        prop: *const hipMemAllocationProp,
        flags: ::core::ffi::c_ulonglong,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Exports an allocation to a requested shareable handle type.

 @param [out] shareableHandle - value of the returned handle.
 @param [in] handle - handle to share.
 @param [in] handleType - type of the shareable handle.
 @param [in] flags - currently unused, must be zero.
 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipMemExportToShareableHandle(
        shareableHandle: *mut ::core::ffi::c_void,
        handle: hipMemGenericAllocationHandle_t,
        handleType: hipMemAllocationHandleType,
        flags: ::core::ffi::c_ulonglong,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get the access flags set for the given location and ptr.

 @param [out] flags - flags for this location.
 @param [in] location - target location.
 @param [in] ptr - address to check the access flags.
 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipMemGetAccess(
        flags: *mut ::core::ffi::c_ulonglong,
        location: *const hipMemLocation,
        ptr: *mut ::core::ffi::c_void,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Calculates either the minimal or recommended granularity.

 @param [out] granularity - returned granularity.
 @param [in] prop - location properties.
 @param [in] option - determines which granularity to return.
 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @note  This API is implemented on Linux and is under development on Microsoft Windows.
*/
    pub fn hipMemGetAllocationGranularity(
        granularity: *mut usize,
        prop: *const hipMemAllocationProp,
        option: hipMemAllocationGranularity_flags,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Retrieve the property structure of the given handle.

 @param [out] prop - properties of the given handle.
 @param [in] handle - handle to perform the query on.
 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipMemGetAllocationPropertiesFromHandle(
        prop: *mut hipMemAllocationProp,
        handle: hipMemGenericAllocationHandle_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Imports an allocation from a requested shareable handle type.

 @param [out] handle - returned value.
 @param [in] osHandle - shareable handle representing the memory allocation.
 @param [in] shHandleType - handle type.
 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipMemImportFromShareableHandle(
        handle: *mut hipMemGenericAllocationHandle_t,
        osHandle: *mut ::core::ffi::c_void,
        shHandleType: hipMemAllocationHandleType,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Maps an allocation handle to a reserved virtual address range.

 @param [in] ptr - address where the memory will be mapped.
 @param [in] size - size of the mapping.
 @param [in] offset - offset into the memory, currently must be zero.
 @param [in] handle - memory allocation to be mapped.
 @param [in] flags - currently unused, must be zero.
 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipMemMap(
        ptr: *mut ::core::ffi::c_void,
        size: usize,
        offset: usize,
        handle: hipMemGenericAllocationHandle_t,
        flags: ::core::ffi::c_ulonglong,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Maps or unmaps subregions of sparse HIP arrays and sparse HIP mipmapped arrays.

 @param [in] mapInfoList - list of hipArrayMapInfo.
 @param [in] count - number of hipArrayMapInfo in mapInfoList.
 @param [in] stream - stream identifier for the stream to use for map or unmap operations.
 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported
 @warning This API is under development. Currently it is not supported on AMD
          GPUs and returns #hipErrorNotSupported.*/
    pub fn hipMemMapArrayAsync(
        mapInfoList: *mut hipArrayMapInfo,
        count: ::core::ffi::c_uint,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Release a memory handle representing a memory allocation which was previously allocated through hipMemCreate.

 @param [in] handle - handle of the memory allocation.
 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipMemRelease(handle: hipMemGenericAllocationHandle_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Returns the allocation handle of the backing memory allocation given the address.

 @param [out] handle - handle representing addr.
 @param [in] addr - address to look up.
 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipMemRetainAllocationHandle(
        handle: *mut hipMemGenericAllocationHandle_t,
        addr: *mut ::core::ffi::c_void,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Set the access flags for each location specified in desc for the given virtual address range.

 @param [in] ptr - starting address of the virtual address range.
 @param [in] size - size of the range.
 @param [in] desc - array of hipMemAccessDesc.
 @param [in] count - number of hipMemAccessDesc in desc.
 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipMemSetAccess(
        ptr: *mut ::core::ffi::c_void,
        size: usize,
        desc: *const hipMemAccessDesc,
        count: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Unmap memory allocation of a given address range.

 @param [in] ptr - starting address of the range to unmap.
 @param [in] size - size of the virtual address range.
 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorNotSupported
 @warning This API is marked as Beta. While this feature is complete, it can
          change and might have outstanding issues.

 @note  This API is implemented on Linux and is under development on Microsoft Windows.*/
    pub fn hipMemUnmap(ptr: *mut ::core::ffi::c_void, size: usize) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Maps a graphics resource for access.

 @param [in] count - Number of resources to map.
 @param [in] resources - Pointer of resources to map.
 @param [in] stream - Stream for synchronization.

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorUnknown, #hipErrorInvalidResourceHandle
*/
    pub fn hipGraphicsMapResources(
        count: ::core::ffi::c_int,
        resources: *mut hipGraphicsResource_t,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Get an array through which to access a subresource of a mapped graphics resource.

 @param [out] array - Pointer of array through which a subresource of resource may be accessed.
 @param [in] resource - Mapped resource to access.
 @param [in] arrayIndex - Array index for the subresource to access.
 @param [in] mipLevel - Mipmap level for the subresource to access.

 @returns #hipSuccess, #hipErrorInvalidValue

 @note  In this API, the value of arrayIndex higher than zero is currently not supported.
*/
    pub fn hipGraphicsSubResourceGetMappedArray(
        array: *mut hipArray_t,
        resource: hipGraphicsResource_t,
        arrayIndex: ::core::ffi::c_uint,
        mipLevel: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Gets device accessible address of a graphics resource.

 @param [out] devPtr - Pointer of device through which graphic resource may be accessed.
 @param [out] size - Size of the buffer accessible from devPtr.
 @param [in] resource - Mapped resource to access.

 @returns #hipSuccess, #hipErrorInvalidValue
*/
    pub fn hipGraphicsResourceGetMappedPointer(
        devPtr: *mut *mut ::core::ffi::c_void,
        size: *mut usize,
        resource: hipGraphicsResource_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Unmaps graphics resources.

 @param [in] count - Number of resources to unmap.
 @param [in] resources - Pointer of resources to unmap.
 @param [in] stream - Stream for synchronization.

 @returns #hipSuccess, #hipErrorInvalidValue, #hipErrorUnknown, #hipErrorContextIsDestroyed
*/
    pub fn hipGraphicsUnmapResources(
        count: ::core::ffi::c_int,
        resources: *mut hipGraphicsResource_t,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Unregisters a graphics resource.

 @param [in] resource - Graphics resources to unregister.

 @returns #hipSuccess
*/
    pub fn hipGraphicsUnregisterResource(resource: hipGraphicsResource_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Create a surface object.

 @param [out] pSurfObject  Pointer of surface object to be created.
 @param [in] pResDesc  Pointer of suface object descriptor.

 @returns #hipSuccess, #hipErrorInvalidValue
*/
    pub fn hipCreateSurfaceObject(
        pSurfObject: *mut hipSurfaceObject_t,
        pResDesc: *const hipResourceDesc,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    /** @brief Destroy a surface object.

 @param [in] surfaceObject  Surface object to be destroyed.

 @returns #hipSuccess, #hipErrorInvalidValue*/
    pub fn hipDestroySurfaceObject(surfaceObject: hipSurfaceObject_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipMemcpy_spt(
        dst: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        sizeBytes: usize,
        kind: hipMemcpyKind,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipMemcpyToSymbol_spt(
        symbol: *const ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        sizeBytes: usize,
        offset: usize,
        kind: hipMemcpyKind,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipMemcpyFromSymbol_spt(
        dst: *mut ::core::ffi::c_void,
        symbol: *const ::core::ffi::c_void,
        sizeBytes: usize,
        offset: usize,
        kind: hipMemcpyKind,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipMemcpy2D_spt(
        dst: *mut ::core::ffi::c_void,
        dpitch: usize,
        src: *const ::core::ffi::c_void,
        spitch: usize,
        width: usize,
        height: usize,
        kind: hipMemcpyKind,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipMemcpy2DFromArray_spt(
        dst: *mut ::core::ffi::c_void,
        dpitch: usize,
        src: hipArray_const_t,
        wOffset: usize,
        hOffset: usize,
        width: usize,
        height: usize,
        kind: hipMemcpyKind,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipMemcpy3D_spt(p: *const hipMemcpy3DParms) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipMemset_spt(
        dst: *mut ::core::ffi::c_void,
        value: ::core::ffi::c_int,
        sizeBytes: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipMemsetAsync_spt(
        dst: *mut ::core::ffi::c_void,
        value: ::core::ffi::c_int,
        sizeBytes: usize,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipMemset2D_spt(
        dst: *mut ::core::ffi::c_void,
        pitch: usize,
        value: ::core::ffi::c_int,
        width: usize,
        height: usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipMemset2DAsync_spt(
        dst: *mut ::core::ffi::c_void,
        pitch: usize,
        value: ::core::ffi::c_int,
        width: usize,
        height: usize,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipMemset3DAsync_spt(
        pitchedDevPtr: hipPitchedPtr,
        value: ::core::ffi::c_int,
        extent: hipExtent,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipMemset3D_spt(
        pitchedDevPtr: hipPitchedPtr,
        value: ::core::ffi::c_int,
        extent: hipExtent,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipMemcpyAsync_spt(
        dst: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        sizeBytes: usize,
        kind: hipMemcpyKind,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipMemcpy3DAsync_spt(
        p: *const hipMemcpy3DParms,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipMemcpy2DAsync_spt(
        dst: *mut ::core::ffi::c_void,
        dpitch: usize,
        src: *const ::core::ffi::c_void,
        spitch: usize,
        width: usize,
        height: usize,
        kind: hipMemcpyKind,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipMemcpyFromSymbolAsync_spt(
        dst: *mut ::core::ffi::c_void,
        symbol: *const ::core::ffi::c_void,
        sizeBytes: usize,
        offset: usize,
        kind: hipMemcpyKind,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipMemcpyToSymbolAsync_spt(
        symbol: *const ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        sizeBytes: usize,
        offset: usize,
        kind: hipMemcpyKind,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipMemcpyFromArray_spt(
        dst: *mut ::core::ffi::c_void,
        src: hipArray_const_t,
        wOffsetSrc: usize,
        hOffset: usize,
        count: usize,
        kind: hipMemcpyKind,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipMemcpy2DToArray_spt(
        dst: hipArray_t,
        wOffset: usize,
        hOffset: usize,
        src: *const ::core::ffi::c_void,
        spitch: usize,
        width: usize,
        height: usize,
        kind: hipMemcpyKind,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipMemcpy2DFromArrayAsync_spt(
        dst: *mut ::core::ffi::c_void,
        dpitch: usize,
        src: hipArray_const_t,
        wOffsetSrc: usize,
        hOffsetSrc: usize,
        width: usize,
        height: usize,
        kind: hipMemcpyKind,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipMemcpy2DToArrayAsync_spt(
        dst: hipArray_t,
        wOffset: usize,
        hOffset: usize,
        src: *const ::core::ffi::c_void,
        spitch: usize,
        width: usize,
        height: usize,
        kind: hipMemcpyKind,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipStreamQuery_spt(stream: hipStream_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipStreamSynchronize_spt(stream: hipStream_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipStreamGetPriority_spt(
        stream: hipStream_t,
        priority: *mut ::core::ffi::c_int,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipStreamWaitEvent_spt(
        stream: hipStream_t,
        event: hipEvent_t,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipStreamGetFlags_spt(
        stream: hipStream_t,
        flags: *mut ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipStreamAddCallback_spt(
        stream: hipStream_t,
        callback: hipStreamCallback_t,
        userData: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_uint,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipEventRecord_spt(event: hipEvent_t, stream: hipStream_t) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipLaunchCooperativeKernel_spt(
        f: *const ::core::ffi::c_void,
        gridDim: dim3,
        blockDim: dim3,
        kernelParams: *mut *mut ::core::ffi::c_void,
        sharedMemBytes: u32,
        hStream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipLaunchKernel_spt(
        function_address: *const ::core::ffi::c_void,
        numBlocks: dim3,
        dimBlocks: dim3,
        args: *mut *mut ::core::ffi::c_void,
        sharedMemBytes: usize,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipGraphLaunch_spt(
        graphExec: hipGraphExec_t,
        stream: hipStream_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipStreamBeginCapture_spt(
        stream: hipStream_t,
        mode: hipStreamCaptureMode,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipStreamEndCapture_spt(
        stream: hipStream_t,
        pGraph: *mut hipGraph_t,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipStreamIsCapturing_spt(
        stream: hipStream_t,
        pCaptureStatus: *mut hipStreamCaptureStatus,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipStreamGetCaptureInfo_spt(
        stream: hipStream_t,
        pCaptureStatus: *mut hipStreamCaptureStatus,
        pId: *mut ::core::ffi::c_ulonglong,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipStreamGetCaptureInfo_v2_spt(
        stream: hipStream_t,
        captureStatus_out: *mut hipStreamCaptureStatus,
        id_out: *mut ::core::ffi::c_ulonglong,
        graph_out: *mut hipGraph_t,
        dependencies_out: *mut *const hipGraphNode_t,
        numDependencies_out: *mut usize,
    ) -> hipError_t;
}
#[cfg_attr(windows, link(name = "amdhip64_7", kind = "raw-dylib"))]
extern "C" {
    #[must_use]
    pub fn hipLaunchHostFunc_spt(
        stream: hipStream_t,
        fn_: hipHostFn_t,
        userData: *mut ::core::ffi::c_void,
    ) -> hipError_t;
}
impl hipErrorCode_t {
    pub const r#InvalidValue: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(1)
    });
    pub const r#OutOfMemory: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(2)
    });
    pub const r#MemoryAllocation: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(2)
    });
    pub const r#NotInitialized: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(3)
    });
    pub const r#InitializationError: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(3)
    });
    pub const r#Deinitialized: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(4)
    });
    pub const r#ProfilerDisabled: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(5)
    });
    pub const r#ProfilerNotInitialized: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(6)
    });
    pub const r#ProfilerAlreadyStarted: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(7)
    });
    pub const r#ProfilerAlreadyStopped: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(8)
    });
    pub const r#InvalidConfiguration: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(9)
    });
    pub const r#InvalidPitchValue: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(12)
    });
    pub const r#InvalidSymbol: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(13)
    });
    pub const r#InvalidDevicePointer: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(17)
    });
    pub const r#InvalidMemcpyDirection: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(21)
    });
    pub const r#InsufficientDriver: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(35)
    });
    pub const r#MissingConfiguration: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(52)
    });
    pub const r#PriorLaunchFailure: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(53)
    });
    pub const r#InvalidDeviceFunction: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(98)
    });
    pub const r#NoDevice: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(100)
    });
    pub const r#InvalidDevice: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(101)
    });
    pub const r#InvalidImage: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(200)
    });
    pub const r#InvalidContext: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(201)
    });
    pub const r#ContextAlreadyCurrent: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(202)
    });
    pub const r#MapFailed: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(205)
    });
    pub const r#MapBufferObjectFailed: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(205)
    });
    pub const r#UnmapFailed: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(206)
    });
    pub const r#ArrayIsMapped: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(207)
    });
    pub const r#AlreadyMapped: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(208)
    });
    pub const r#NoBinaryForGpu: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(209)
    });
    pub const r#AlreadyAcquired: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(210)
    });
    pub const r#NotMapped: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(211)
    });
    pub const r#NotMappedAsArray: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(212)
    });
    pub const r#NotMappedAsPointer: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(213)
    });
    pub const r#ECCNotCorrectable: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(214)
    });
    pub const r#UnsupportedLimit: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(215)
    });
    pub const r#ContextAlreadyInUse: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(216)
    });
    pub const r#PeerAccessUnsupported: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(217)
    });
    pub const r#InvalidKernelFile: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(218)
    });
    pub const r#InvalidGraphicsContext: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(219)
    });
    pub const r#InvalidSource: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(300)
    });
    pub const r#FileNotFound: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(301)
    });
    pub const r#SharedObjectSymbolNotFound: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(302)
    });
    pub const r#SharedObjectInitFailed: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(303)
    });
    pub const r#OperatingSystem: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(304)
    });
    pub const r#InvalidHandle: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(400)
    });
    pub const r#InvalidResourceHandle: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(400)
    });
    pub const r#IllegalState: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(401)
    });
    pub const r#NotFound: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(500)
    });
    pub const r#NotReady: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(600)
    });
    pub const r#IllegalAddress: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(700)
    });
    pub const r#LaunchOutOfResources: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(701)
    });
    pub const r#LaunchTimeOut: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(702)
    });
    pub const r#PeerAccessAlreadyEnabled: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(704)
    });
    pub const r#PeerAccessNotEnabled: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(705)
    });
    pub const r#SetOnActiveProcess: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(708)
    });
    pub const r#ContextIsDestroyed: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(709)
    });
    pub const r#Assert: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(710)
    });
    pub const r#HostMemoryAlreadyRegistered: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(712)
    });
    pub const r#HostMemoryNotRegistered: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(713)
    });
    pub const r#LaunchFailure: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(719)
    });
    pub const r#CooperativeLaunchTooLarge: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(720)
    });
    pub const r#NotSupported: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(801)
    });
    pub const r#StreamCaptureUnsupported: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(900)
    });
    pub const r#StreamCaptureInvalidated: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(901)
    });
    pub const r#StreamCaptureMerge: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(902)
    });
    pub const r#StreamCaptureUnmatched: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(903)
    });
    pub const r#StreamCaptureUnjoined: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(904)
    });
    pub const r#StreamCaptureIsolation: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(905)
    });
    pub const r#StreamCaptureImplicit: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(906)
    });
    pub const r#CapturedEvent: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(907)
    });
    pub const r#StreamCaptureWrongThread: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(908)
    });
    pub const r#GraphExecUpdateFailure: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(910)
    });
    pub const r#InvalidChannelDescriptor: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(911)
    });
    pub const r#InvalidTexture: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(912)
    });
    pub const r#Unknown: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(999)
    });
    pub const r#RuntimeMemory: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(1052)
    });
    pub const r#RuntimeOther: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(1053)
    });
    pub const r#Tbd: hipErrorCode_t = hipErrorCode_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(1054)
    });
}
#[repr(transparent)]
#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub struct hipErrorCode_t(pub ::core::num::NonZeroU32);
pub trait hipError_tConsts {
    const Success: hipError_t = hipError_t::Ok(());
    const ErrorInvalidValue: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#InvalidValue,
    );
    const ErrorOutOfMemory: hipError_t = hipError_t::Err(hipErrorCode_t::r#OutOfMemory);
    const ErrorMemoryAllocation: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#MemoryAllocation,
    );
    const ErrorNotInitialized: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#NotInitialized,
    );
    const ErrorInitializationError: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#InitializationError,
    );
    const ErrorDeinitialized: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#Deinitialized,
    );
    const ErrorProfilerDisabled: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#ProfilerDisabled,
    );
    const ErrorProfilerNotInitialized: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#ProfilerNotInitialized,
    );
    const ErrorProfilerAlreadyStarted: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#ProfilerAlreadyStarted,
    );
    const ErrorProfilerAlreadyStopped: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#ProfilerAlreadyStopped,
    );
    const ErrorInvalidConfiguration: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#InvalidConfiguration,
    );
    const ErrorInvalidPitchValue: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#InvalidPitchValue,
    );
    const ErrorInvalidSymbol: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#InvalidSymbol,
    );
    const ErrorInvalidDevicePointer: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#InvalidDevicePointer,
    );
    const ErrorInvalidMemcpyDirection: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#InvalidMemcpyDirection,
    );
    const ErrorInsufficientDriver: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#InsufficientDriver,
    );
    const ErrorMissingConfiguration: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#MissingConfiguration,
    );
    const ErrorPriorLaunchFailure: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#PriorLaunchFailure,
    );
    const ErrorInvalidDeviceFunction: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#InvalidDeviceFunction,
    );
    const ErrorNoDevice: hipError_t = hipError_t::Err(hipErrorCode_t::r#NoDevice);
    const ErrorInvalidDevice: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#InvalidDevice,
    );
    const ErrorInvalidImage: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#InvalidImage,
    );
    const ErrorInvalidContext: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#InvalidContext,
    );
    const ErrorContextAlreadyCurrent: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#ContextAlreadyCurrent,
    );
    const ErrorMapFailed: hipError_t = hipError_t::Err(hipErrorCode_t::r#MapFailed);
    const ErrorMapBufferObjectFailed: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#MapBufferObjectFailed,
    );
    const ErrorUnmapFailed: hipError_t = hipError_t::Err(hipErrorCode_t::r#UnmapFailed);
    const ErrorArrayIsMapped: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#ArrayIsMapped,
    );
    const ErrorAlreadyMapped: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#AlreadyMapped,
    );
    const ErrorNoBinaryForGpu: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#NoBinaryForGpu,
    );
    const ErrorAlreadyAcquired: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#AlreadyAcquired,
    );
    const ErrorNotMapped: hipError_t = hipError_t::Err(hipErrorCode_t::r#NotMapped);
    const ErrorNotMappedAsArray: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#NotMappedAsArray,
    );
    const ErrorNotMappedAsPointer: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#NotMappedAsPointer,
    );
    const ErrorECCNotCorrectable: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#ECCNotCorrectable,
    );
    const ErrorUnsupportedLimit: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#UnsupportedLimit,
    );
    const ErrorContextAlreadyInUse: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#ContextAlreadyInUse,
    );
    const ErrorPeerAccessUnsupported: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#PeerAccessUnsupported,
    );
    const ErrorInvalidKernelFile: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#InvalidKernelFile,
    );
    const ErrorInvalidGraphicsContext: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#InvalidGraphicsContext,
    );
    const ErrorInvalidSource: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#InvalidSource,
    );
    const ErrorFileNotFound: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#FileNotFound,
    );
    const ErrorSharedObjectSymbolNotFound: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#SharedObjectSymbolNotFound,
    );
    const ErrorSharedObjectInitFailed: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#SharedObjectInitFailed,
    );
    const ErrorOperatingSystem: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#OperatingSystem,
    );
    const ErrorInvalidHandle: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#InvalidHandle,
    );
    const ErrorInvalidResourceHandle: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#InvalidResourceHandle,
    );
    const ErrorIllegalState: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#IllegalState,
    );
    const ErrorNotFound: hipError_t = hipError_t::Err(hipErrorCode_t::r#NotFound);
    const ErrorNotReady: hipError_t = hipError_t::Err(hipErrorCode_t::r#NotReady);
    const ErrorIllegalAddress: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#IllegalAddress,
    );
    const ErrorLaunchOutOfResources: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#LaunchOutOfResources,
    );
    const ErrorLaunchTimeOut: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#LaunchTimeOut,
    );
    const ErrorPeerAccessAlreadyEnabled: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#PeerAccessAlreadyEnabled,
    );
    const ErrorPeerAccessNotEnabled: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#PeerAccessNotEnabled,
    );
    const ErrorSetOnActiveProcess: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#SetOnActiveProcess,
    );
    const ErrorContextIsDestroyed: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#ContextIsDestroyed,
    );
    const ErrorAssert: hipError_t = hipError_t::Err(hipErrorCode_t::r#Assert);
    const ErrorHostMemoryAlreadyRegistered: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#HostMemoryAlreadyRegistered,
    );
    const ErrorHostMemoryNotRegistered: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#HostMemoryNotRegistered,
    );
    const ErrorLaunchFailure: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#LaunchFailure,
    );
    const ErrorCooperativeLaunchTooLarge: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#CooperativeLaunchTooLarge,
    );
    const ErrorNotSupported: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#NotSupported,
    );
    const ErrorStreamCaptureUnsupported: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#StreamCaptureUnsupported,
    );
    const ErrorStreamCaptureInvalidated: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#StreamCaptureInvalidated,
    );
    const ErrorStreamCaptureMerge: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#StreamCaptureMerge,
    );
    const ErrorStreamCaptureUnmatched: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#StreamCaptureUnmatched,
    );
    const ErrorStreamCaptureUnjoined: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#StreamCaptureUnjoined,
    );
    const ErrorStreamCaptureIsolation: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#StreamCaptureIsolation,
    );
    const ErrorStreamCaptureImplicit: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#StreamCaptureImplicit,
    );
    const ErrorCapturedEvent: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#CapturedEvent,
    );
    const ErrorStreamCaptureWrongThread: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#StreamCaptureWrongThread,
    );
    const ErrorGraphExecUpdateFailure: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#GraphExecUpdateFailure,
    );
    const ErrorInvalidChannelDescriptor: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#InvalidChannelDescriptor,
    );
    const ErrorInvalidTexture: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#InvalidTexture,
    );
    const ErrorUnknown: hipError_t = hipError_t::Err(hipErrorCode_t::r#Unknown);
    const ErrorRuntimeMemory: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#RuntimeMemory,
    );
    const ErrorRuntimeOther: hipError_t = hipError_t::Err(
        hipErrorCode_t::r#RuntimeOther,
    );
    const ErrorTbd: hipError_t = hipError_t::Err(hipErrorCode_t::r#Tbd);
}
impl hipError_tConsts for hipError_t {}
#[must_use]
pub type hipError_t = ::core::result::Result<(), hipErrorCode_t>;
const _: fn() = || {
    let _ = std::mem::transmute::<hipError_t, u32>;
};
unsafe impl Send for hipDeviceptr_t {}
unsafe impl Sync for hipDeviceptr_t {}
unsafe impl Send for hipStream_t {}
unsafe impl Sync for hipStream_t {}
unsafe impl Send for hipModule_t {}
unsafe impl Sync for hipModule_t {}
unsafe impl Send for hipFunction_t {}
unsafe impl Sync for hipFunction_t {}
