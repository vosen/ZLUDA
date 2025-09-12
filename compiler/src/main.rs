use bpaf::Bpaf;
use error::CompilerError;
use std::ffi::CStr;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::str;
use std::time::Instant;
use std::{env, mem};

mod error;

const DEFAULT_ARCH: &'static str = "gfx1100";

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version)]
pub struct Options {
    #[bpaf(argument("output-dir"))]
    /// Output directory
    output_dir: Option<PathBuf>,

    #[bpaf(long("arch"))]
    /// Target architecture
    arch: Option<String>,

    #[bpaf(positional("filename"))]
    /// PTX file
    ptx_path: String,
}

fn main() -> ExitCode {
    if let Err(e) = main_core() {
        eprintln!("Error: {}", e);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn main_core() -> Result<(), CompilerError> {
    let opts = options().run();
    let comgr = comgr::Comgr::new()?;

    let ptx_path = Path::new(&opts.ptx_path).to_path_buf();
    let filename_base = ptx_path
        .file_name()
        .map(|osstr| osstr.to_str().unwrap_or("output"))
        .unwrap_or("output");

    let mut output_path = match opts.output_dir {
        Some(value) => value,
        None => match ptx_path.parent() {
            Some(dir) => dir.to_path_buf(),
            None => env::current_dir()?,
        },
    };
    output_path.push(filename_base);

    let arch: String = match opts.arch {
        Some(s) => s,
        None => (|| {
            let runtime = hip::Runtime::load()?;
            runtime.init()?;
            get_gpu_arch(&runtime)
        })()
        .unwrap_or_else(|_| DEFAULT_ARCH.to_owned()),
    };

    let ptx = fs::read(&ptx_path).map_err(CompilerError::from)?;
    let ptx = str::from_utf8(&ptx).map_err(CompilerError::from)?;
    let llvm = ptx_to_llvm(ptx).map_err(CompilerError::from)?;

    write_to_file(&llvm.llvm_ir, output_path.with_extension("ll").as_path())?;

    let comgr_hook = |bytes: &Vec<u8>, extension: String| {
        let output_path = output_path.with_extension(extension);
        write_to_file(bytes, &output_path).unwrap();
    };

    let mut start = Instant::now();
    comgr::compile_bitcode(
        &comgr,
        &arch,
        &llvm.bitcode,
        &llvm.linked_bitcode,
        &llvm.attributes_bitcode,
        Some(&comgr_hook),
    )
    .map_err(CompilerError::from)?;
    report_pass_time("compile_bitcode", &mut start);

    Ok(())
}

fn ptx_to_llvm(ptx: &str) -> Result<LLVMArtifacts, CompilerError> {
    let ast = ptx_parser::parse_module_checked(ptx).map_err(CompilerError::from)?;
    let mut start = Instant::now();
    let module = ptx::to_llvm_module(
        ast,
        ptx::Attributes {
            clock_rate: 2124000,
        },
        |pass| {
            report_pass_time(pass, &mut start);
        },
    )
    .map_err(CompilerError::from)?;
    let bitcode = module.llvm_ir.write_bitcode_to_memory().to_vec();
    let linked_bitcode = module.linked_bitcode().to_vec();
    let attributes_bitcode = module.attributes_ir.write_bitcode_to_memory().to_vec();
    let llvm_ir = module.llvm_ir.print_module_to_string().to_bytes().to_vec();
    Ok(LLVMArtifacts {
        bitcode,
        linked_bitcode,
        attributes_bitcode,
        llvm_ir,
    })
}

fn report_pass_time(pass: &str, start: &mut Instant) {
    let duration = start.elapsed();
    println!("Pass {:?} took {:?}", pass, duration);
    *start = Instant::now();
}

#[derive(Debug)]
struct LLVMArtifacts {
    bitcode: Vec<u8>,
    linked_bitcode: Vec<u8>,
    attributes_bitcode: Vec<u8>,
    llvm_ir: Vec<u8>,
}

fn get_gpu_arch(runtime: &hip::Runtime) -> Result<String, CompilerError> {
    let mut dev_props = unsafe { mem::zeroed() };
    runtime.device_get_properties(&mut dev_props, 0)?;
    let gcn_arch_name = &dev_props.gcnArchName;
    let gcn_arch_name = unsafe { CStr::from_ptr(gcn_arch_name.as_ptr()) };
    let gcn_arch_name = gcn_arch_name.to_str()?;
    Ok(gcn_arch_name.to_string())
}

fn write_to_file(content: &[u8], path: &Path) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content)?;
    file.flush()?;
    println!("Wrote to {}", path.to_str().unwrap());
    Ok(())
}

mod hip {
    use crate::error::CompilerError;

    // We lazy load HIP runtime because we want to work on systems with no
    // HIP driver installed
    pub struct Runtime(libloading::Library);

    impl Runtime {
        fn hip_check(err: u32) -> Result<(), CompilerError> {
            match err {
                0 => Ok(()),
                err_code => Err(CompilerError::HipError(err_code)),
            }
        }

        pub fn load() -> Result<Self, CompilerError> {
            #[cfg(windows)]
            let lib_name = "amdhip64_6.dll\0";
            #[cfg(unix)]
            let lib_name = "libamdhip64.so.6\0";
            let library = unsafe { libloading::Library::new(lib_name)? };
            Ok(Self(library))
        }

        pub fn init(&self) -> Result<(), CompilerError> {
            unsafe {
                let hip_init: libloading::Symbol<unsafe extern "C" fn(u32) -> u32> =
                    self.0.get(b"hipInit\0")?;
                Self::hip_check(hip_init(0))
            }
        }

        pub fn device_get_properties(
            &self,
            prop: &mut hipDeviceProp_tR0600,
            device: i32,
        ) -> Result<(), CompilerError> {
            unsafe {
                let hip_get_device_properties: libloading::Symbol<
                    unsafe extern "C" fn(*mut hipDeviceProp_tR0600, i32) -> u32,
                > = self.0.get(b"hipGetDevicePropertiesR0600\0")?;
                Self::hip_check(hip_get_device_properties(prop, device))
            }
        }
    }

    #[allow(non_snake_case, non_camel_case_types)]
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

    #[allow(non_snake_case, non_camel_case_types)]
    #[repr(C)]
    #[repr(align(4))]
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    pub struct hipDeviceArch_t {
        pub _bitfield_align_1: [u8; 0],
        pub _bitfield_1: __BindgenBitfieldUnit<[u8; 3usize]>,
        pub __bindgen_padding_0: u8,
    }

    #[repr(C)]
    #[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct __BindgenBitfieldUnit<Storage> {
        storage: Storage,
    }

    #[allow(non_camel_case_types)]
    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    pub struct hipUUID_t {
        pub bytes: [::core::ffi::c_char; 16usize],
    }
    #[allow(non_camel_case_types)]
    pub type hipUUID = hipUUID_t;
}
