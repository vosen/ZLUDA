use crate::ptx;
use crate::translate;
use ocl::{Buffer, Context, Device, Kernel, OclPrm, Platform, Program, Queue};
use std::error;
use std::ffi::{c_void, CString};
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

fn test_ptx_assert<'a, T: OclPrm + From<u8>>(
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

fn run_spirv<T: OclPrm + From<u8>>(
    name: &str,
    spirv: &[u32],
    input: &[T],
    output: &mut [T],
) -> ocl::Result<Vec<T>> {
    let (drv, device, queue) = unsafe { l0_init() };
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
    let binaries = if let ocl::core::ProgramInfoResult::Binaries(bins) = binaries_wrapped {
        bins
    } else {
        panic!()
    };
    let module = l0_create_module(device, &binaries[0]);
    let kernel_desc = l0::ze_kernel_desc_t {
        version: l0::ze_kernel_desc_version_t::ZE_KERNEL_DESC_VERSION_CURRENT,
        flags: l0::ze_kernel_flag_t::ZE_KERNEL_FLAG_NONE,
        pKernelName: "ld_st".as_ptr() as *const _,
    };
    let mut kernel: l0::ze_kernel_handle_t = ptr::null_mut();
    let mut err = unsafe { l0::zeKernelCreate(module, &kernel_desc, &mut kernel) };
    assert_eq!(err, l0::ze_result_t::ZE_RESULT_SUCCESS);
    let inp_b = l0_allocate_buffer(drv, device, &input);
    let out_b = l0_allocate_buffer(drv, device, &output);
    println!("inp_b: {:?}", inp_b);
    println!("out_b: {:?}", out_b);
    let mut cmd_list = l0_create_cmd_list(device);
    println!("input: {:?}", input);
    err = unsafe {
        l0::zeCommandListAppendMemoryCopy(
            cmd_list,
            inp_b,
            input.as_ptr() as *const _,
            input.len() * mem::size_of::<T>(),
            ptr::null_mut(),
        )
    };
    assert_eq!(err, l0::ze_result_t::ZE_RESULT_SUCCESS);
    let pattern = 0u8;
    err = unsafe {
        l0::zeCommandListAppendMemoryFill(
            cmd_list,
            out_b,
            &pattern as *const u8 as *const _,
            1,
            input.len() * mem::size_of::<T>(),
            ptr::null_mut(),
        )
    };
    assert_eq!(err, l0::ze_result_t::ZE_RESULT_SUCCESS);
    err = unsafe { l0::zeKernelSetGroupSize(kernel, 1, 1, 1) };
    assert_eq!(err, l0::ze_result_t::ZE_RESULT_SUCCESS);
    let wg_size = l0::ze_group_count_t {
        groupCountX: 1,
        groupCountY: 1,
        groupCountZ: 1,
    };
    err = unsafe {
        l0::zeKernelSetArgumentValue(
            kernel,
            0,
            mem::size_of::<*mut c_void>(),
            &inp_b as *const *mut _ as *const _,
        )
    };
    assert_eq!(err, l0::ze_result_t::ZE_RESULT_SUCCESS);
    err = unsafe {
        l0::zeKernelSetArgumentValue(
            kernel,
            1,
            mem::size_of::<*mut c_void>(),
            &out_b as *const *mut _ as *const _,
        )
    };
    assert_eq!(err, l0::ze_result_t::ZE_RESULT_SUCCESS);
    err = unsafe {
        l0::zeCommandListAppendBarrier(
            cmd_list,
            ptr::null_mut(),
            0,
            ptr::null_mut(),
        )
    };
    assert_eq!(err, l0::ze_result_t::ZE_RESULT_SUCCESS);
    err = unsafe {
        l0::zeCommandListAppendLaunchKernel(
            cmd_list,
            kernel,
            &wg_size,
            ptr::null_mut(),
            0,
            ptr::null_mut(),
        )
    };
    assert_eq!(err, l0::ze_result_t::ZE_RESULT_SUCCESS);
    err = unsafe {
        l0::zeCommandListAppendBarrier(
            cmd_list,
            ptr::null_mut(),
            0,
            ptr::null_mut(),
        )
    };
    assert_eq!(err, l0::ze_result_t::ZE_RESULT_SUCCESS);
    let mut result: Vec<T> = vec![0u8.into(); output.len()];
    err = unsafe {
        l0::zeCommandListAppendMemoryCopy(
            cmd_list,
            result.as_mut_ptr() as *mut _,
            out_b,
            result.len() * mem::size_of::<T>(),
            ptr::null_mut(),
        )
    };
    assert_eq!(err, l0::ze_result_t::ZE_RESULT_SUCCESS);
    err = unsafe { l0::zeCommandListClose(cmd_list) };
    assert_eq!(err, l0::ze_result_t::ZE_RESULT_SUCCESS);
    err =
        unsafe { l0::zeCommandQueueExecuteCommandLists(queue, 1, &mut cmd_list, ptr::null_mut()) };
    assert_eq!(err, l0::ze_result_t::ZE_RESULT_SUCCESS);
    err = unsafe { l0::zeCommandQueueSynchronize(queue, u32::max_value()) };
    assert_eq!(err, l0::ze_result_t::ZE_RESULT_SUCCESS);
    /*
    let (plat, dev) = get_ocl_platform_device();
    let ctx = Context::builder().platform(plat).devices(dev).build()?;
    let empty_cstr = CString::new("-cl-intel-greater-than-4GB-buffer-required").unwrap();
    let byte_il = unsafe {
        slice::from_raw_parts::<u8>(
            spirv.as_ptr() as *const _,
            spirv.len() * mem::size_of::<u32>(),
        )
    };
    let src = CString::new(
        "
    __kernel void ld_st(ulong a, ulong b)
    {
        __global ulong* a_copy = (__global ulong*)a;
        __global ulong* b_copy = (__global ulong*)b;
        *b_copy = *a_copy;
    }",
    )
    .unwrap();
    //let prog = Program::with_il(byte_il, Some(&[dev]), &empty_cstr, &ctx)?;
    let prog = Program::with_source(&ctx, &[src], Some(&[dev]), &empty_cstr)?;
    let queue = Queue::new(&ctx, dev, None)?;
    let cl_device_mem_alloc_intel = get_cl_device_mem_alloc_intel(&plat)?;
    let cl_enqueue_memcpy_intel = get_cl_enqueue_memcpy_intel(&plat)?;
    let cl_enqueue_memset_intel = get_cl_enqueue_memset_intel(&plat)?;
    let cl_set_kernel_arg_mem_pointer_intel = get_cl_set_kernel_arg_mem_pointer_intel(&plat)?;
    let mut err_code = 0;
    let inp_b = cl_device_mem_alloc_intel(
        ctx.as_ptr(),
        dev.as_raw(),
        ptr::null_mut(),
        input.len() * mem::size_of::<T>(),
        mem::align_of::<T>() as u32,
        &mut err_code,
    );
    assert_eq!(err_code, 0);
    let out_b = cl_device_mem_alloc_intel(
        ctx.as_ptr(),
        dev.as_raw(),
        ptr::null_mut(),
        output.len() * mem::size_of::<T>(),
        mem::align_of::<T>() as u32,
        &mut err_code,
    );
    assert_eq!(err_code, 0);
    err_code = cl_enqueue_memcpy_intel(
        queue.as_ptr(),
        1,
        inp_b as *mut _,
        input.as_ptr() as *const _,
        input.len() * mem::size_of::<T>(),
        0,
        ptr::null(),
        ptr::null_mut(),
    );
    assert_eq!(err_code, 0);
    err_code = cl_enqueue_memset_intel(
        queue.as_ptr(),
        out_b as *mut _,
        0,
        input.len() * mem::size_of::<T>(),
        0,
        ptr::null(),
        ptr::null_mut(),
    );
    assert_eq!(err_code, 0);
    let kernel = ocl::core::create_kernel(prog.as_core(), name)?;
    err_code = cl_set_kernel_arg_mem_pointer_intel(kernel.as_ptr(), 0, inp_b);
    assert_eq!(err_code, 0);
    err_code = cl_set_kernel_arg_mem_pointer_intel(kernel.as_ptr(), 1, out_b);
    assert_eq!(err_code, 0);
    unsafe {
        ocl::core::enqueue_kernel::<(), ()>(
            queue.as_core(),
            &kernel,
            1,
            None,
            &[1, 0, 0],
            None,
            None,
            None,
        )
    }?;
    let mut result: Vec<T> = vec![0u8.into(); output.len()];
    err_code = cl_enqueue_memcpy_intel(
        queue.as_ptr(),
        1,
        result.as_mut_ptr() as *mut _,
        inp_b,
        result.len() * mem::size_of::<T>(),
        0,
        ptr::null(),
        ptr::null_mut(),
    );
    assert_eq!(err_code, 0);
    queue.finish()?;
    */
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
