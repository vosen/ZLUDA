use crate::pass;
use hip_runtime_sys::hipError_t;
use std::error;
use std::ffi::{CStr, CString};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::mem;
use std::{ptr, str};

macro_rules! test_ptx {
    ($fn_name:ident, $input:expr, $output:expr) => {
        paste::item! {
            #[test]
            fn [<$fn_name _hip>]() -> Result<(), Box<dyn std::error::Error>> {
                let ptx = include_str!(concat!(stringify!($fn_name), ".ptx"));
                let input = $input;
                let mut output = $output;
                test_hip_assert(stringify!($fn_name), ptx, &input, &mut output)
            }
        }

        paste::item! {
            #[test]
            fn [<$fn_name _cuda>]() -> Result<(), Box<dyn std::error::Error>> {
                let ptx = include_str!(concat!(stringify!($fn_name), ".ptx"));
                let input = $input;
                let mut output = $output;
                test_cuda_assert(stringify!($fn_name), ptx, &input, &mut output)
            }
        }
    };

    ($fn_name:ident) => {};
}

test_ptx!(ld_st, [1u64], [1u64]);
test_ptx!(ld_st_implicit, [0.5f32, 0.25f32], [0.5f32]);
test_ptx!(mov, [1u64], [1u64]);
test_ptx!(mul_lo, [1u64], [2u64]);
test_ptx!(mul_hi, [u64::max_value()], [1u64]);
test_ptx!(add, [1u64], [2u64]);
test_ptx!(setp, [10u64, 11u64], [1u64, 0u64]);
test_ptx!(setp_gt, [f32::NAN, 1f32], [1f32]);
test_ptx!(setp_leu, [1f32, f32::NAN], [1f32]);
test_ptx!(bra, [10u64], [11u64]);
test_ptx!(not, [0u64], [u64::max_value()]);
test_ptx!(shl, [11u64], [44u64]);
test_ptx!(cvt_sat_s_u, [-1i32], [0i32]);
test_ptx!(cvta, [3.0f32], [3.0f32]);
test_ptx!(block, [1u64], [2u64]);
test_ptx!(local_align, [1u64], [1u64]);
test_ptx!(call, [1u64], [2u64]);
test_ptx!(vector, [1u32, 2u32], [3u32, 3u32]);
test_ptx!(vector4, [1u32, 2u32, 3u32, 4u32], [4u32]);
test_ptx!(ld_st_offset, [1u32, 2u32], [2u32, 1u32]);
test_ptx!(ntid, [3u32], [4u32]);
test_ptx!(reg_local, [12u64], [13u64]);
test_ptx!(mov_address, [0xDEADu64], [0u64]);
test_ptx!(b64tof64, [111u64], [111u64]);
// This segfaults NV compiler
// test_ptx!(implicit_param, [34u32], [34u32]);
test_ptx!(pred_not, [10u64, 11u64], [2u64, 0u64]);
test_ptx!(mad_s32, [2i32, 3i32, 4i32], [10i32, 10i32, 10i32]);
test_ptx!(
    mul_wide,
    [0x01_00_00_00__01_00_00_00i64],
    [0x1_00_00_00_00_00_00i64]
);
test_ptx!(vector_extract, [1u8, 2u8, 3u8, 4u8], [3u8, 4u8, 1u8, 2u8]);
test_ptx!(shr, [-2i32], [-1i32]);
test_ptx!(or, [1u64, 2u64], [3u64]);
test_ptx!(sub, [2u64], [1u64]);
test_ptx!(min, [555i32, 444i32], [444i32]);
test_ptx!(max, [555i32, 444i32], [555i32]);
test_ptx!(global_array, [0xDEADu32], [1u32]);
test_ptx!(extern_shared, [127u64], [127u64]);
test_ptx!(extern_shared_call, [121u64], [123u64]);
test_ptx!(rcp, [2f32], [0.5f32]);
// 0b1_00000000_10000000000000000000000u32 is a large denormal
// 0x3f000000 is 0.5
test_ptx!(
    mul_ftz,
    [0b1_00000000_10000000000000000000000u32, 0x3f000000u32],
    [0b1_00000000_00000000000000000000000u32]
);
test_ptx!(
    mul_non_ftz,
    [0b1_00000000_10000000000000000000000u32, 0x3f000000u32],
    [0b1_00000000_01000000000000000000000u32]
);
test_ptx!(constant_f32, [10f32], [5f32]);
test_ptx!(constant_negative, [-101i32], [101i32]);
test_ptx!(and, [6u32, 3u32], [2u32]);
test_ptx!(selp, [100u16, 200u16], [200u16]);
test_ptx!(selp_true, [100u16, 200u16], [100u16]);
test_ptx!(fma, [2f32, 3f32, 5f32], [11f32]);
test_ptx!(shared_variable, [513u64], [513u64]);
test_ptx!(shared_ptr_32, [513u64], [513u64]);
test_ptx!(atom_cas, [91u32, 91u32], [91u32, 100u32]);
test_ptx!(atom_inc, [100u32], [100u32, 101u32, 0u32]);
test_ptx!(atom_add, [2u32, 4u32], [2u32, 6u32]);
test_ptx!(div_approx, [1f32, 2f32], [0.5f32]);
test_ptx!(sqrt, [0.25f32], [0.5f32]);
test_ptx!(rsqrt, [0.25f64], [2f64]);
test_ptx!(neg, [181i32], [-181i32]);
test_ptx!(sin, [std::f32::consts::PI / 2f32], [1f32]);
test_ptx!(cos, [std::f32::consts::PI], [-1f32]);
test_ptx!(lg2, [512f32], [9f32]);
test_ptx!(ex2, [10f32], [1024f32]);
test_ptx!(cvt_rni, [9.5f32, 10.5f32], [10f32, 10f32]);
test_ptx!(cvt_rzi, [-13.8f32, 12.9f32], [-13f32, 12f32]);
test_ptx!(cvt_s32_f32, [-13.8f32, 12.9f32], [-13i32, 13i32]);
test_ptx!(clz, [0b00000101_00101101_00010011_10101011u32], [5u32]);
test_ptx!(popc, [0b10111100_10010010_01001001_10001010u32], [14u32]);
test_ptx!(
    brev,
    [0b11000111_01011100_10101110_11111011u32],
    [0b11011111_01110101_00111010_11100011u32]
);
test_ptx!(
    xor,
    [
        0b01010010_00011010_01000000_00001101u32,
        0b11100110_10011011_00001100_00100011u32
    ],
    [0b10110100100000010100110000101110u32]
);
test_ptx!(rem, [21692i32, 13i32], [8i32]);
test_ptx!(
    bfe,
    [0b11111000_11000001_00100010_10100000u32, 16u32, 8u32],
    [0b11000001u32]
);
test_ptx!(bfi, [0b10u32, 0b101u32, 0u32, 2u32], [0b110u32]);
test_ptx!(stateful_ld_st_simple, [121u64], [121u64]);
test_ptx!(stateful_ld_st_ntid, [123u64], [123u64]);
test_ptx!(stateful_ld_st_ntid_chain, [12651u64], [12651u64]);
test_ptx!(stateful_ld_st_ntid_sub, [96311u64], [96311u64]);
test_ptx!(shared_ptr_take_address, [97815231u64], [97815231u64]);
test_ptx!(cvt_s64_s32, [-1i32], [-1i64]);
test_ptx!(add_tuning, [2u64], [3u64]);
test_ptx!(add_non_coherent, [3u64], [4u64]);
test_ptx!(sign_extend, [-1i16], [-1i32]);
test_ptx!(atom_add_float, [1.25f32, 0.5f32], [1.25f32, 1.75f32]);
test_ptx!(
    setp_nan,
    [
        0.5f32,
        f32::NAN,
        f32::NAN,
        0.5f32,
        f32::NAN,
        f32::NAN,
        0.5f32,
        0.5f32
    ],
    [1u32, 1u32, 1u32, 0u32]
);
test_ptx!(
    setp_num,
    [
        0.5f32,
        f32::NAN,
        f32::NAN,
        0.5f32,
        f32::NAN,
        f32::NAN,
        0.5f32,
        0.5f32
    ],
    [0u32, 0u32, 0u32, 2u32]
);
test_ptx!(non_scalar_ptr_offset, [1u32, 2u32, 3u32, 4u32], [7u32]);
test_ptx!(stateful_neg_offset, [1237518u64], [1237518u64]);
test_ptx!(const, [0u16], [10u16, 20, 30, 40]);
test_ptx!(cvt_s16_s8, [0x139231C2u32], [0xFFFFFFC2u32]);
test_ptx!(cvt_f64_f32, [0.125f32], [0.125f64]);
test_ptx!(prmt, [0x70c507d6u32, 0x6fbd4b5cu32], [0x6fbdd65cu32]);
test_ptx!(activemask, [0u32], [1u32]);
test_ptx!(membar, [152731u32], [152731u32]);
test_ptx!(shared_unify_extern, [7681u64, 7682u64], [15363u64]);
test_ptx!(shared_unify_local, [16752u64, 714u64], [17466u64]);
test_ptx!(
    add_ftz,
    [f32::from_bits(0x800000), f32::from_bits(0x007FFFFF)],
    [0x800000u32, 0xFFFFFF]
);

test_ptx!(malformed_label, [2u64], [3u64]);

test_ptx!(assertfail);
test_ptx!(func_ptr);
test_ptx!(lanemask_lt);
test_ptx!(extern_func);

struct DisplayError<T: Debug> {
    err: T,
}

impl<T: Debug> Display for DisplayError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}

impl<T: Debug> Debug for DisplayError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}

impl<T: Debug> error::Error for DisplayError<T> {}

fn test_hip_assert<
    'a,
    Input: From<u8> + Debug + Copy + PartialEq,
    Output: From<u8> + Debug + Copy + PartialEq + Default,
>(
    name: &str,
    ptx_text: &'a str,
    input: &[Input],
    output: &mut [Output],
) -> Result<(), Box<dyn error::Error + 'a>> {
    let ast = ptx_parser::parse_module_checked(ptx_text).unwrap();
    let llvm_ir = pass::to_llvm_module(ast).unwrap();
    let name = CString::new(name)?;
    let result =
        run_hip(name.as_c_str(), llvm_ir, input, output).map_err(|err| DisplayError { err })?;
    assert_eq!(result.as_slice(), output);
    Ok(())
}

fn test_cuda_assert<
    'a,
    Input: From<u8> + Debug + Copy + PartialEq,
    Output: From<u8> + Debug + Copy + PartialEq + Default,
>(
    name: &str,
    ptx_text: &'a str,
    input: &[Input],
    output: &mut [Output],
) -> Result<(), Box<dyn error::Error + 'a>> {
    let name = CString::new(name)?;
    let result =
        run_cuda(name.as_c_str(), ptx_text, input, output).map_err(|err| DisplayError { err })?;
    assert_eq!(result.as_slice(), output);
    Ok(())
}

macro_rules! cuda_call {
    ($expr:expr) => {
        #[allow(unused_unsafe)]
        {
            let err = unsafe { $expr };
            if err != cuda_driver_sys::CUresult::CUDA_SUCCESS {
                return Result::Err(err);
            }
        }
    };
}

fn run_cuda<Input: From<u8> + Copy + Debug, Output: From<u8> + Copy + Debug + Default>(
    name: &CStr,
    ptx_module: &str,
    input: &[Input],
    output: &mut [Output],
) -> Result<Vec<Output>, cuda_driver_sys::CUresult> {
    use cuda_driver_sys::*;
    cuda_call! { cuInit(0) };
    let ptx_module = CString::new(ptx_module).unwrap();
    let mut result = vec![0u8.into(); output.len()];
    {
        let mut ctx = ptr::null_mut();
        cuda_call! { cuCtxCreate_v2(&mut ctx, 0, 0) };
        let mut module = ptr::null_mut();
        cuda_call! { cuModuleLoadData(&mut module, ptx_module.as_ptr() as _) };
        let mut kernel = ptr::null_mut();
        cuda_call! { cuModuleGetFunction(&mut kernel, module, name.as_ptr()) };
        let mut inp_b = unsafe { mem::zeroed() };
        cuda_call! { cuMemAlloc_v2(&mut inp_b, input.len() * mem::size_of::<Input>()) };
        let mut out_b = unsafe { mem::zeroed() };
        cuda_call! { cuMemAlloc_v2(&mut out_b, output.len() * mem::size_of::<Output>()) };
        cuda_call! { cuMemcpyHtoD_v2(inp_b, input.as_ptr() as _, input.len() * mem::size_of::<Input>()) };
        cuda_call! { cuMemsetD8_v2(out_b, 0, output.len() * mem::size_of::<Output>()) };
        let mut args = [&inp_b, &out_b];
        cuda_call! { cuLaunchKernel(kernel, 1,1,1,1,1,1, 1024, 0 as _, args.as_mut_ptr() as _, ptr::null_mut()) };
        cuda_call! { cuMemcpyDtoH_v2(result.as_mut_ptr() as _, out_b, output.len() * mem::size_of::<Output>()) };
        cuda_call! { cuStreamSynchronize(0 as _) };
        cuda_call! { cuMemFree_v2(inp_b) };
        cuda_call! { cuMemFree_v2(out_b) };
        cuda_call! { cuModuleUnload(module) };
        cuda_call! { cuCtxDestroy_v2(ctx) };
    }
    Ok(result)
}

fn run_hip<Input: From<u8> + Copy + Debug, Output: From<u8> + Copy + Debug + Default>(
    name: &CStr,
    module: pass::Module,
    input: &[Input],
    output: &mut [Output],
) -> Result<Vec<Output>, hipError_t> {
    use hip_runtime_sys::*;
    unsafe { hipInit(0) }.unwrap();
    let mut result = vec![0u8.into(); output.len()];
    {
        let dev = 0;
        let mut stream = unsafe { mem::zeroed() };
        unsafe { hipStreamCreate(&mut stream) }.unwrap();
        let mut dev_props = unsafe { mem::zeroed() };
        unsafe { hipGetDevicePropertiesR0600(&mut dev_props, dev) }.unwrap();
        let elf_module = comgr::compile_bitcode(
            unsafe { CStr::from_ptr(dev_props.gcnArchName.as_ptr()) },
            &*module.llvm_ir,
            module.linked_bitcode(),
        )
        .unwrap();
        let mut module = unsafe { mem::zeroed() };
        unsafe { hipModuleLoadData(&mut module, elf_module.as_ptr() as _) }.unwrap();
        let mut kernel = unsafe { mem::zeroed() };
        unsafe { hipModuleGetFunction(&mut kernel, module, name.as_ptr()) }.unwrap();
        let mut inp_b = ptr::null_mut();
        unsafe { hipMalloc(&mut inp_b, input.len() * mem::size_of::<Input>()) }.unwrap();
        let mut out_b = ptr::null_mut();
        unsafe { hipMalloc(&mut out_b, output.len() * mem::size_of::<Output>()) }.unwrap();
        unsafe {
            hipMemcpyWithStream(
                inp_b,
                input.as_ptr() as _,
                input.len() * mem::size_of::<Input>(),
                hipMemcpyKind::hipMemcpyHostToDevice,
                stream,
            )
        }
        .unwrap();
        unsafe { hipMemset(out_b, 0, output.len() * mem::size_of::<Output>()) }.unwrap();
        let mut args = [&inp_b, &out_b];
        unsafe {
            hipModuleLaunchKernel(
                kernel,
                1,
                1,
                1,
                1,
                1,
                1,
                1024,
                stream,
                args.as_mut_ptr() as _,
                ptr::null_mut(),
            )
        }
        .unwrap();
        unsafe {
            hipMemcpyAsync(
                result.as_mut_ptr() as _,
                out_b,
                output.len() * mem::size_of::<Output>(),
                hipMemcpyKind::hipMemcpyDeviceToHost,
                stream,
            )
        }
        .unwrap();
        unsafe { hipStreamSynchronize(stream) }.unwrap();
        unsafe { hipFree(inp_b) }.unwrap();
        unsafe { hipFree(out_b) }.unwrap();
        unsafe { hipModuleUnload(module) }.unwrap();
    }
    Ok(result)
}
