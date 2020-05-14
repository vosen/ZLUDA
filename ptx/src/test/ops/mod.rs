use crate::ptx;
use crate::translate;
use ocl::{Buffer, Context, Device, Kernel, OclPrm, Platform, Program, Queue};
use std::error;
use std::ffi::{c_void, CStr, CString};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::mem;
use std::slice;
use std::{ptr, str};

macro_rules! test_ptx {
    ($fn_name:ident, $input:expr, $output:expr) => {
        #[test]
        fn $fn_name() -> Result<(), Box<dyn std::error::Error>> {
            let ptx = include_str!(concat!(stringify!($fn_name), ".ptx"));
            let input = $input;
            let mut output = $output;
            crate::test::ops::test_ptx_assert(stringify!($fn_name), ptx, &input, &mut output)
        }
    };
}

mod ld_st;

const CL_DEVICE_IL_VERSION: u32 = 0x105B;

struct DisplayError<T: Display + Debug> {
    err: T,
}

impl<T: Display + Debug> Display for DisplayError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.err, f)
    }
}

impl<T: Display + Debug> Debug for DisplayError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}

impl<T: Display + Debug> error::Error for DisplayError<T> {}

fn test_ptx_assert<'a, T: OclPrm + From<u8> + ze::SafeRepr>(
    name: &str,
    ptx_text: &'a str,
    input: &[T],
    output: &mut [T],
) -> Result<(), Box<dyn error::Error + 'a>> {
    let mut errors = Vec::new();
    let ast = ptx::ModuleParser::new().parse(&mut errors, ptx_text)?;
    assert!(errors.len() == 0);
    let spirv = translate::to_spirv(ast)?;
    let result = run_spirv(name, &spirv, input, output).map_err(|err| DisplayError { err })?;
    assert_eq!(&output, &&*result);
    Ok(())
}

fn run_spirv<T: OclPrm + From<u8> + ze::SafeRepr>(
    name: &str,
    spirv: &[u32],
    input: &[T],
    output: &mut [T],
) -> ze::Result<Vec<T>> {
    ze::init()?;
    let mut result = vec![0u8.into(); output.len()];
    let mut drivers = ze::Driver::get()?;
    let drv = drivers.drain(0..1).next().unwrap();
    let mut devices = drv.devices()?;
    let dev = devices.drain(0..1).next().unwrap();
    let queue = ze::CommandQueue::new(&dev)?;
    let native_bins = get_program_native().unwrap();
    let module = ze::Module::new_native(&dev, &native_bins[0])?;
    let kernel = ze::Kernel::new(&module, CStr::from_bytes_with_nul(b"ld_st\0").unwrap())?;
    let mut inp_b = ze::DeviceBuffer::<T>::new(&drv, &dev, input.len())?;
    let mut out_b = ze::DeviceBuffer::<T>::new(&drv, &dev, output.len())?;
    let inp_b_ptr_mut: ze::BufferPtrMut<T> = (&mut inp_b).into();
    let event_pool = ze::EventPool::new(&drv, 3, Some(&[&dev]))?;
    let ev0 = ze::Event::new(&event_pool, 0)?;
    let ev1 = ze::Event::new(&event_pool, 1)?;
    let mut cmd_list = ze::CommandList::new(&dev)?;
    let out_b_ptr: ze::BufferPtrMut<T> = (&mut out_b).into();
    cmd_list.append_memory_copy(inp_b_ptr_mut, input, None, Some(&ev0))?;
    cmd_list.append_memory_fill(out_b_ptr, 0u8.into(), Some(&ev1))?;
    kernel.set_group_size(1, 1, 1)?;
    kernel.set_arg_buffer(0, inp_b_ptr_mut)?;
    kernel.set_arg_buffer(1, out_b_ptr)?;
    cmd_list.append_launch_kernel(&kernel, &[1, 1, 1], None, &[&ev0, &ev1])?;
    cmd_list.append_memory_copy(result.as_mut_slice(), inp_b_ptr_mut, None, Some(&ev0))?;
    queue.execute(cmd_list)?;
    Ok(result)
}

fn get_ocl_platform_device() -> (Platform, Device) {
    for p in Platform::list() {
        if p.extensions()
            .unwrap()
            .iter()
            .find(|ext| *ext == "cl_intel_unified_shared_memory_preview")
            .is_none()
        {
            continue;
        }
        for d in Device::list_all(p).unwrap() {
            let typ = d.info(ocl::enums::DeviceInfo::Type).unwrap();
            if let ocl::enums::DeviceInfoResult::Type(typ) = typ {
                if typ.cpu() == ocl::flags::DeviceType::CPU {
                    continue;
                }
            }
            if let Ok(version) = d.info_raw(CL_DEVICE_IL_VERSION) {
                let name = str::from_utf8(&version).unwrap();
                if name.starts_with("SPIR-V") {
                    return (p, d);
                }
            }
        }
    }
    panic!("No OpenCL device with SPIR-V and USM support found")
}

fn get_cl_device_mem_alloc_intel(
    p: &Platform,
) -> ocl::core::Result<
    extern "C" fn(
        ocl::core::ffi::cl_context,
        ocl::core::ffi::cl_device_id,
        *const ocl::core::ffi::cl_bitfield,
        ocl::core::ffi::size_t,
        ocl::core::ffi::cl_uint,
        *mut ocl::core::ffi::cl_int,
    ) -> *const c_void,
> {
    let ptr = unsafe {
        ocl::core::get_extension_function_address_for_platform(
            p.as_core(),
            "clDeviceMemAllocINTEL",
            None,
        )
    }?;
    Ok(unsafe { std::mem::transmute(ptr) })
}

fn get_cl_enqueue_memcpy_intel(
    p: &Platform,
) -> ocl::core::Result<
    extern "C" fn(
        ocl::core::ffi::cl_command_queue,
        ocl::core::ffi::cl_bool,
        *mut c_void,
        *const c_void,
        ocl::core::ffi::size_t,
        ocl::core::ffi::cl_uint,
        *const ocl::core::ffi::cl_event,
        *mut ocl::core::ffi::cl_event,
    ) -> ocl::core::ffi::cl_int,
> {
    let ptr = unsafe {
        ocl::core::get_extension_function_address_for_platform(
            p.as_core(),
            "clEnqueueMemcpyINTEL",
            None,
        )
    }?;
    Ok(unsafe { std::mem::transmute(ptr) })
}

fn get_cl_enqueue_memset_intel(
    p: &Platform,
) -> ocl::core::Result<
    extern "C" fn(
        ocl::core::ffi::cl_command_queue,
        *mut c_void,
        ocl::core::ffi::cl_int,
        ocl::core::ffi::size_t,
        ocl::core::ffi::cl_uint,
        *const ocl::core::ffi::cl_event,
        *mut ocl::core::ffi::cl_event,
    ) -> ocl::core::ffi::cl_int,
> {
    let ptr = unsafe {
        ocl::core::get_extension_function_address_for_platform(
            p.as_core(),
            "clEnqueueMemsetINTEL",
            None,
        )
    }?;
    Ok(unsafe { std::mem::transmute(ptr) })
}

fn get_cl_set_kernel_arg_mem_pointer_intel(
    p: &Platform,
) -> ocl::core::Result<
    extern "C" fn(
        ocl::core::ffi::cl_kernel,
        ocl::core::ffi::cl_uint,
        *const c_void,
    ) -> ocl::core::ffi::cl_int,
> {
    let ptr = unsafe {
        ocl::core::get_extension_function_address_for_platform(
            p.as_core(),
            "clSetKernelArgMemPointerINTEL",
            None,
        )
    }?;
    Ok(unsafe { std::mem::transmute(ptr) })
}

unsafe fn l0_init() -> (
    l0::ze_driver_handle_t,
    l0::ze_device_handle_t,
    l0::ze_command_queue_handle_t,
) {
    let mut err = l0::ze_result_t::ZE_RESULT_SUCCESS;
    err = l0::zeInit(l0::ze_init_flag_t::ZE_INIT_FLAG_NONE);
    assert_eq!(err, l0::ze_result_t::ZE_RESULT_SUCCESS);
    let mut len = 1;
    let mut driver: l0::ze_driver_handle_t = ptr::null_mut();
    err = l0::zeDriverGet(&mut len, &mut driver);
    assert_eq!(err, l0::ze_result_t::ZE_RESULT_SUCCESS);
    let mut device: l0::ze_device_handle_t = ptr::null_mut();
    err = l0::zeDeviceGet(driver, &mut len, &mut device);
    assert_eq!(err, l0::ze_result_t::ZE_RESULT_SUCCESS);
    let que_desc = l0::ze_command_queue_desc_t {
        version: l0::ze_command_queue_desc_version_t::ZE_COMMAND_QUEUE_DESC_VERSION_CURRENT,
        flags: l0::ze_command_queue_flag_t::ZE_COMMAND_QUEUE_FLAG_NONE,
        mode: l0::ze_command_queue_mode_t::ZE_COMMAND_QUEUE_MODE_SYNCHRONOUS,
        priority: l0::ze_command_queue_priority_t::ZE_COMMAND_QUEUE_PRIORITY_NORMAL,
        ordinal: 0,
    };
    let mut queue: l0::ze_command_queue_handle_t = ptr::null_mut();
    err = l0::zeCommandQueueCreate(device, &que_desc, &mut queue);
    assert_eq!(err, l0::ze_result_t::ZE_RESULT_SUCCESS);
    (driver, device, queue)
}

fn l0_create_module(dev: l0::ze_device_handle_t, bin: &[u8]) -> l0::ze_module_handle_t {
    let desc = l0::ze_module_desc_t {
        version: l0::ze_module_desc_version_t::ZE_MODULE_DESC_VERSION_CURRENT,
        format: l0::ze_module_format_t::ZE_MODULE_FORMAT_NATIVE,
        inputSize: bin.len(),
        pInputModule: bin.as_ptr(),
        pBuildFlags: ptr::null(),
        pConstants: ptr::null(),
    };
    let mut result: l0::ze_module_handle_t = ptr::null_mut();
    let err = unsafe { l0::zeModuleCreate(dev, &desc, &mut result, ptr::null_mut()) };
    assert_eq!(err, l0::ze_result_t::ZE_RESULT_SUCCESS);
    result
}

fn l0_allocate_buffer<T>(
    drv: l0::ze_driver_handle_t,
    dev: l0::ze_device_handle_t,
    based: &[T],
) -> *mut c_void {
    let desc = l0::_ze_device_mem_alloc_desc_t {
        version: l0::ze_device_mem_alloc_desc_version_t::ZE_DEVICE_MEM_ALLOC_DESC_VERSION_CURRENT,
        flags: l0::_ze_device_mem_alloc_flag_t::ZE_DEVICE_MEM_ALLOC_FLAG_DEFAULT,
        ordinal: 0,
    };
    let mut result = ptr::null_mut();
    let err = unsafe {
        l0::zeDriverAllocDeviceMem(
            drv,
            &desc,
            based.len() * mem::size_of::<T>(),
            mem::align_of::<T>(),
            dev,
            &mut result,
        )
    };
    assert_eq!(err, l0::ze_result_t::ZE_RESULT_SUCCESS);
    result
}

fn l0_create_cmd_list(dev: l0::ze_device_handle_t) -> l0::ze_command_list_handle_t {
    let desc = l0::_ze_command_list_desc_t {
        version: l0::ze_command_list_desc_version_t::ZE_COMMAND_LIST_DESC_VERSION_CURRENT,
        flags: l0::ze_command_list_flag_t::ZE_COMMAND_LIST_FLAG_EXPLICIT_ONLY,
    };
    let mut result: l0::ze_command_list_handle_t = ptr::null_mut();
    let err = unsafe { l0::zeCommandListCreate(dev, &desc, &mut result) };
    assert_eq!(err, l0::ze_result_t::ZE_RESULT_SUCCESS);
    result
}

fn get_program_native() -> ocl::Result<Vec<Vec<u8>>> {
    let (ocl_plat, ocl_dev) = get_ocl_platform_device();
    let ocl_ctx = Context::builder()
        .platform(ocl_plat)
        .devices(ocl_dev)
        .build()?;
    let empty_cstr = CString::new("-cl-intel-greater-than-4GB-buffer-required").unwrap();
    let src = CString::new(
        "
    __kernel void ld_st(ulong a, ulong b)
    {
        __global ulong* a_copy = (__global ulong*)a;
        __global ulong* b_copy = (__global ulong*)b;
        *b_copy = *a_copy;
    }",
    )?;
    let prog = Program::with_source(&ocl_ctx, &[src], None, &empty_cstr)?;
    let binaries_wrapped = prog.info(ocl::core::ProgramInfo::Binaries)?;
    if let ocl::core::ProgramInfoResult::Binaries(bins) = binaries_wrapped {
        Ok(bins)
    } else {
        panic!()
    }
}
