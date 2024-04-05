use crate::llvm;
use crate::ptx;
use crate::translate;
use comgr::Comgr;
use half::f16;
use hip_common::CompilationMode;
use hip_runtime_sys::*;
use paste::paste;
use std::error;
use std::ffi::{CStr, CString};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::mem;
use std::sync::Once;
use std::{env, fs, path::PathBuf, ptr, str};
use zluda_llvm::bit_writer::*;

macro_rules! test_ptx {
    ($fn_name:ident, $input:expr, $output:expr) => {
        paste! {
            #[test]
            fn [<$fn_name _hip>]() -> Result<(), Box<dyn std::error::Error>> {
                let ptx = include_str!(concat!(stringify!($fn_name), ".ptx"));
                let input = $input;
                let mut output = $output;
                test_hip_assert(stringify!($fn_name), ptx, &input, &mut output)
            }
        }

        paste! {
            #[test]
            fn [<$fn_name _cuda>]() -> Result<(), Box<dyn std::error::Error>> {
                let ptx = include_str!(concat!(stringify!($fn_name), ".ptx"));
                let input = $input;
                let mut output = $output;
                test_cuda_assert(stringify!($fn_name), ptx, Some(&input), &mut output, 1)
            }
        }

        paste! {
            #[test]
            fn [<$fn_name _llvm_ir>]() -> Result<(), Box<dyn std::error::Error>> {
                let ptx_txt = include_str!(concat!(stringify!($fn_name), ".ptx"));
                let llvm_file_name = concat!(stringify!($fn_name), ".ll");
                let llvm_ir = include_bytes!(concat!(stringify!($fn_name), ".ll"));
                unsafe { test_llvm_assert(ptx_txt, llvm_ir, llvm_file_name) }
            }
        }
    };

    ($fn_name:ident) => {
        paste! {
            #[test]
            fn [<$fn_name _comgr>]() -> Result<(), Box<dyn std::error::Error>> {
                let ptx_txt = include_str!(concat!(stringify!($fn_name), ".ptx"));
                unsafe { test_compile_assert(ptx_txt) }
            }
        }

        paste! {
            #[test]
            fn [<$fn_name _llvm_ir>]() -> Result<(), Box<dyn std::error::Error>> {
                let ptx_txt = include_str!(concat!(stringify!($fn_name), ".ptx"));
                let llvm_file_name = concat!(stringify!($fn_name), ".ll");
                let llvm_ir = include_bytes!(concat!(stringify!($fn_name), ".ll"));
                unsafe { test_llvm_assert(ptx_txt, llvm_ir, llvm_file_name) }
            }
        }
    };
}

macro_rules! test_ptx_warp {
    ($fn_name:ident, $expected:expr) => {
        paste! {
            #[test]
            fn [<$fn_name _cuda>]() -> Result<(), Box<dyn std::error::Error>> {
                let ptx = include_str!(concat!(stringify!($fn_name), ".ptx"));
                let mut expected = $expected;
                test_cuda_assert::<u8, _>(stringify!($fn_name), ptx, None, &mut expected, 64)
            }

            #[test]
            fn [<$fn_name _hip_wave32>]() -> Result<(), Box<dyn std::error::Error>> {
                let ptx = include_str!(concat!(stringify!($fn_name), ".ptx"));
                let mut expected = $expected;
                test_hip_assert_output(CompilationMode::Wave32, stringify!($fn_name), ptx, &mut expected)
            }

            #[test]
            fn [<$fn_name _hip_wave32onwave64>]() -> Result<(), Box<dyn std::error::Error>> {
                let ptx = include_str!(concat!(stringify!($fn_name), ".ptx"));
                let mut expected = $expected;
                test_hip_assert_output(CompilationMode::Wave32OnWave64,stringify!($fn_name), ptx, &mut expected)
            }

            #[test]
            fn [<$fn_name _hip_doublewave32onwave64>]() -> Result<(), Box<dyn std::error::Error>> {
                let ptx = include_str!(concat!(stringify!($fn_name), ".ptx"));
                let mut expected = $expected;
                test_hip_assert_output(CompilationMode::DoubleWave32OnWave64, stringify!($fn_name), ptx, &mut expected)
            }
        }
    }
}

test_ptx!(ld_st, [1u64], [1u64]);
test_ptx!(ld_st_implicit, [0.5f32, 0.25f32], [0.5f32]);
test_ptx!(mov, [1u64], [1u64]);
test_ptx!(mul_lo, [1u64], [2u64]);
test_ptx!(mul_hi, [u64::max_value()], [1u64]);
test_ptx!(add, [1u64], [2u64]);
test_ptx!(add_global, [1f32], [0x408487EEu32]);
test_ptx!(amdgpu_unnamed, [2u64], [3u64]);
test_ptx!(setp, [10u64, 11u64], [1u64, 0u64]);
test_ptx!(setp_gt, [f32::NAN, 1f32], [1f32]);
test_ptx!(setp_pred2, [100f32, 23f32], [100f32]);
test_ptx!(setp_bool, [100f32, 23f32, 9f32], [9f32]);
test_ptx!(setp_leu, [1f32, f32::NAN], [1f32]);
test_ptx!(bra, [10u64], [11u64]);
test_ptx!(not, [0u64], [u64::max_value()]);
test_ptx!(shf, [11u32, 12u32], [196608u32]);
test_ptx!(shl, [11u64], [44u64]);
test_ptx!(shl_link_hack, [11u64], [44u64]);
test_ptx!(shl_overflow, [1u32, 31, 32, 33], [2147483648u32, 0, 0]);
test_ptx!(cvt_sat_s_u, [-1i32], [0i32, -1i32]);
test_ptx!(cvta, [3.0f32], [3.0f32]);
test_ptx!(block, [1u64], [2u64]);
test_ptx!(local_align, [1u64], [1u64]);
test_ptx!(call, [1u64], [2u64]);
// In certain situations LLVM will miscompile AMDGPU binaries.
// This happens if the return type of a function is a .b8 array.
// This test checks if our workaround for this bug works
test_ptx!(call_bug, [1u64], [2u64]);
test_ptx!(callprototype, [1u64], [2u64]);
test_ptx!(call_multi_return, [2u32, 3u32], [5u64, 6u64]);
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
test_ptx!(
    mad_s32,
    [0xffffffu32, 0xffffffu32, 1u32, 0u32, 1u32],
    [0xFE000002u64, 0x10000u64, 0xFFFFFE000002u64]
);
// 16777216 * -268435456 = -4503599627370496
test_ptx!(
    mul_wide,
    [0x01_00_00_00__f0_00_00_00i64],
    [0xff_f0_00_00_00_00_00_00u64]
);
test_ptx!(vector_extract, [1u8, 2u8, 3u8, 4u8], [3u8, 4u8, 1u8, 2u8]);
test_ptx!(shr_s32, [-4i32, 32i32], [-1i32]);
test_ptx!(shr_u32, [u32::MAX, 31u32, 32u32], [1u32, 0u32]);
test_ptx!(or, [1u64, 2u64], [3u64]);
test_ptx!(sub, [2u64], [1u64]);
test_ptx!(min, [555i32, 444i32], [444i32]);
test_ptx!(max, [555i32, -1i32], [555i32]);
test_ptx!(global_array, [0xDEADu32], [4294967295u32]);
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
test_ptx!(
    ex2,
    [10f32, f32::NEG_INFINITY, 0f32, f32::INFINITY],
    [1024f32, 0f32, 1f32, f32::INFINITY]
);
test_ptx!(cvt_rni, [9.5f32, 10.5f32], [10f32, 10f32]);
test_ptx!(cvt_rzi, [-13.8f32, 12.9f32], [-13f32, 12f32]);
// Logically, 33554434i32 with `rn` rounding could round to either 33554432f32 or 33554436f32
// Maybe IEEE is more precise than NV PTX docs?
test_ptx!(
    cvt_f32_s32,
    [33554434i32, 33554435i32, 33554435i32, 33554435i32],
    [33554432f32, 33554432f32, 33554432f32, 33554436f32]
);
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
test_ptx!(const, [0u16], [10u16, 20, 30, 40]);
test_ptx!(cvt_s16_s8, [0x139231C2u32], [0xFFFFFFC2u32]);
test_ptx!(cvt_f64_f32, [0.125f32], [0.125f64]);
test_ptx!(cvt_f32_f16, [0xa1u16], [0x37210000u32]);
test_ptx!(
    prmt,
    [0x70c507d6u32, 0x6fbd4b5cu32],
    [0x6fbdd65cu32, 0x6FFFD600]
);
test_ptx!(
    prmt_non_immediate,
    [0x70c507d6u32, 0x6fbd4b5cu32],
    [0xD6D65CD6u32]
);
test_ptx!(activemask, [0u32], [1u32]);
test_ptx!(membar, [152731u32], [152731u32]);
test_ptx!(shared_unify_decl, [7681u64, 7682u64], [15363u64]);
test_ptx!(shared_unify_extern, [7681u64, 7682u64], [15363u64]);
test_ptx!(shared_unify_local, [16752u64, 714u64], [17466u64]);
test_ptx!(cvt_u32_s16, [-1i16, -1i16], [0xffffffffu32]);
test_ptx!(abs, [i32::MIN, -134i32], [i32::MIN, 134i32]);
test_ptx!(
    madc_cc,
    [65521u32, 2147549199, 0x1000],
    [2147487519u32, 4294934539]
);
test_ptx!(
    mad_hi_cc,
    [0x26223377u32, 0x70777766u32, 0x60666633u32],
    [0x71272866u32, 0u32, 1u32]
); // Multi-tap :)
test_ptx!(mov_vector_cast, [0x200000001u64], [2u32, 1u32]);
test_ptx!(
    cvt_clamp,
    [f32::NAN, f32::NEG_INFINITY, f32::INFINITY, 1.00001],
    [0f32, 0.0, 1.0, 1.0]
);
test_ptx!(generic, [0xDEADu32], [210u32]);
test_ptx!(vote_ballot, [0xDEADu32], [1u32, 0, 0, 1]);
test_ptx!(param_ptr, [1u64], [2u64]);
test_ptx!(s64_min, [0xDEADu32], [i64::MIN]);
test_ptx!(multireg, [441u64], [442u64]);
test_ptx!(
    addc_cc,
    [
        2_147_483_650u32,
        2_147_483_649u32,
        4_294_967_294u32,
        4_294_967_294u32
    ],
    [3u32, 2u32, 1u32]
);
test_ptx!(addc_cc2, [0xDEADu32], [1u32, 1u32]);
test_ptx!(
    subc_cc,
    [
        2_147_483_649u32,
        2_147_483_650u32,
        4_294_967_294u32,
        4_294_967_294u32
    ],
    [4294967295u32, 0, 2]
);
test_ptx!(
    carry_set_all,
    [0xDEADu32],
    [1u32, 0, 0, 1, 0, 1, 0u32, 4294967295, 4294967295, 0u32, 4294967295, 0u32]
);
test_ptx!(vshr, [0x6f3650f4u32, 22, 0xc62d4586], [0xC62D4742u32]);
test_ptx!(bfind, [0u32, 1u32, 0x64eb0414], [u32::MAX, 0, 30]);
test_ptx!(bfind_shiftamt, [0u32, 1u32, 0x19bea67d], [u32::MAX, 31, 3]);
test_ptx!(
    atom_add_f16,
    [f16::from_f32(2.0), f16::from_f32(3.0)],
    [f16::from_f32(2.0), f16::from_f32(5.0)]
);
test_ptx!(
    set_f16x2,
    [0xc1690e6eu32, 0x13739444u32, 0x424834CC, 0x4248B4CC],
    [0xffffu32, 0x3C000000]
);
test_ptx!(
    dp4a,
    [0xde3032f5u32, 0x2474fe15, 0xf51d8d6c],
    [0xF51D9D19u32]
);
test_ptx!(add_param_ptr, [61382u64], [61383u64]);
test_ptx!(atom_max_u32, [1u32, u32::MAX], [u32::MAX]);
test_ptx!(atom_ld_st, [1923569713u32], [1923569713u32]);
test_ptx!(
    atom_ld_st_vec,
    [1923569713u64, 1923569712],
    [1923569713u64, 1923569712]
);

test_ptx_warp!(
    shfl,
    [
        1u32, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 31, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 63
    ]
);
test_ptx_warp!(
    laneid,
    [
        0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
        17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
    ]
);
test_ptx_warp!(
    match_any_32,
    [
        369229872u32,
        1077973120,
        2157985796,
        1077973120,
        369229872,
        369229872,
        2157985796,
        1077973120,
        369229872,
        1077973120,
        369229872,
        369229872,
        1077973120,
        2157985796,
        2157985796,
        1077973120,
        1077973120,
        369229872,
        2157985796,
        369229872,
        369229872,
        2157985796,
        1077973120,
        2157985796,
        1077973120,
        369229872,
        369229872,
        369229872,
        369229872,
        1077973120,
        1077973120,
        2157985796,
        4148,
        348176512,
        4148,
        3257008128,
        4148,
        4148,
        348176512,
        348176512,
        3257008128,
        4148,
        3257008128,
        348176512,
        4148,
        348176512,
        348176512,
        348176512,
        3257008128,
        3257008128,
        4148,
        348176512,
        4148,
        3257008128,
        348176512,
        348176512,
        3257008128,
        3257008128,
        348176512,
        3257008128,
        348176512,
        3257008128,
        3257008128,
        3257008128
    ]
);
test_ptx_warp!(
    red_shared,
    [
        1025u32, 1058, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
        47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64
    ]
);

test_ptx!(barrier);
test_ptx!(assertfail);
test_ptx!(func_ptr);
test_ptx!(lanemask_lt);
test_ptx!(alloca_call);

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
    let mut errors = Vec::new();
    let ast = ptx::ModuleParser::new().parse(&mut errors, ptx_text)?;
    assert!(errors.len() == 0);
    let zluda_module = translate::to_llvm_module(CompilationMode::Wave32, vec![ast])?;
    let name = CString::new(name)?;
    let result = run_hip(
        CompilationMode::Wave32,
        name.as_c_str(),
        zluda_module,
        Some(input),
        output,
        [1, 1, 1],
    )
    .map_err(|err| DisplayError { err })?;
    assert_eq!(result.as_slice(), output);
    Ok(())
}

fn test_hip_assert_output<'a>(
    compilation_mode: CompilationMode,
    name: &str,
    ptx_text: &'a str,
    expected: &mut [u32],
) -> Result<(), Box<dyn error::Error + 'a>> {
    let mut errors = Vec::new();
    let ast = ptx::ModuleParser::new().parse(&mut errors, ptx_text)?;
    assert!(errors.len() == 0);
    let zluda_module = translate::to_llvm_module(compilation_mode, vec![ast])?;
    let name = CString::new(name)?;
    let z_dimension = if compilation_mode == CompilationMode::Wave32OnWave64 {
        2
    } else {
        1
    };
    let result = run_hip::<u32, _>(
        compilation_mode,
        name.as_c_str(),
        zluda_module,
        None,
        expected,
        [64, 1, z_dimension],
    )
    .map_err(|err| DisplayError { err })?;
    assert_eq!(result.as_slice(), expected);
    Ok(())
}

fn test_cuda_assert<
    'a,
    Input: From<u8> + Debug + Copy + PartialEq,
    Output: From<u8> + Debug + Copy + PartialEq + Default,
>(
    name: &str,
    ptx_text: &'a str,
    input: Option<&[Input]>,
    output: &mut [Output],
    block_size_x: u32,
) -> Result<(), Box<dyn error::Error + 'a>> {
    let name = CString::new(name)?;
    let result = unsafe { run_cuda(name.as_c_str(), ptx_text, input, output, block_size_x) };
    assert_eq!(result.as_slice(), output);
    Ok(())
}

macro_rules! hip_call {
    ($expr:expr) => {
        #[allow(unused_unsafe)]
        {
            let err = unsafe { $expr };
            if err != hip_runtime_sys::hipError_t::hipSuccess {
                return Result::Err(err);
            }
        }
    };
}

unsafe fn run_cuda<Input: From<u8> + Copy + Debug, Output: From<u8> + Copy + Debug + Default>(
    name: &CStr,
    ptx_module: &str,
    input: Option<&[Input]>,
    output: &mut [Output],
    block_size_x: u32,
) -> Vec<Output> {
    use cuda_types::*;
    let cuda = CudaTestLibrary::new();
    cuda.cuInit(0);
    let ptx_module = CString::new(ptx_module).unwrap();
    let mut result = vec![0u8.into(); output.len()];
    {
        let mut ctx = ptr::null_mut();
        cuda.cuCtxCreate_v2(&mut ctx, 0, CUdevice_v1(0));
        let mut module = ptr::null_mut();
        cuda.cuModuleLoadData(&mut module, ptx_module.as_ptr() as _);
        let mut kernel = ptr::null_mut();
        cuda.cuModuleGetFunction(&mut kernel, module, name.as_ptr());
        let mut inp_b = unsafe { mem::zeroed() };
        let mut out_b = unsafe { mem::zeroed() };
        cuda.cuMemAlloc_v2(&mut out_b, output.len() * mem::size_of::<Output>());
        let mut args = if let Some(input) = input {
            cuda.cuMemAlloc_v2(&mut inp_b, input.len() * mem::size_of::<Input>());
            cuda.cuMemcpyHtoD_v2(
                inp_b,
                input.as_ptr() as _,
                input.len() * mem::size_of::<Input>(),
            );
            [&inp_b, &out_b]
        } else {
            [&out_b, &inp_b]
        };
        cuda.cuMemsetD8_v2(out_b, 0, output.len() * mem::size_of::<Output>());
        cuda.cuLaunchKernel(
            kernel,
            1,
            1,
            1,
            block_size_x,
            1,
            1,
            1024,
            0 as _,
            args.as_mut_ptr() as _,
            ptr::null_mut(),
        );
        cuda.cuMemcpyDtoH_v2(
            result.as_mut_ptr() as _,
            out_b,
            output.len() * mem::size_of::<Output>(),
        );
        cuda.cuStreamSynchronize(0 as _);
        cuda.cuMemFree_v2(inp_b);
        cuda.cuMemFree_v2(out_b);
        cuda.cuModuleUnload(module);
        cuda.cuCtxDestroy_v2(ctx);
    }
    result
}

static mut COMGR: comgr::Result<Comgr> =
    comgr::Result::Err(comgr::sys::amd_comgr_status_t::AMD_COMGR_STATUS_ERROR);
static COMGR_INIT: Once = Once::new();

fn get_comgr() -> comgr::Result<&'static Comgr> {
    COMGR_INIT.call_once(|| unsafe { COMGR = Comgr::find_and_load() });
    match unsafe { &COMGR } {
        Ok(c) => Ok(c),
        Err(e) => Err(*e),
    }
}

fn run_hip<Input: From<u8> + Copy + Debug, Output: From<u8> + Copy + Debug + Default>(
    compilation_mode: CompilationMode,
    name: &CStr,
    module: translate::Module,
    input: Option<&[Input]>,
    output: &mut [Output],
    block_size: [u32; 3],
) -> Result<Vec<Output>, hipError_t> {
    use hip_runtime_sys::*;
    let mut result = vec![0u8.into(); output.len()];
    let comgr = get_comgr().unwrap();
    let isa = unsafe { hip_common::comgr_isa(0)? };
    let compiled = comgr
        .compile(
            compilation_mode,
            &isa,
            module.get_bitcode_all(),
            &module.metadata.to_elf_section(),
        )
        .unwrap();
    hip_call! { hipInit(0) };
    {
        let dev = 0;
        let mut stream = ptr::null_mut();
        hip_call! { hipStreamCreateWithFlags(&mut stream, hipStreamNonBlocking) };
        let mut dev_props = unsafe { mem::zeroed() };
        hip_call! { hipGetDeviceProperties(&mut dev_props, dev) };
        let mut module = ptr::null_mut();
        hip_call! { hipModuleLoadData(&mut module, compiled.as_ptr() as _) };
        let mut kernel = ptr::null_mut();
        hip_call! { hipModuleGetFunction(&mut kernel, module, name.as_ptr()) };
        let mut inp_b = ptr::null_mut();
        let mut out_b = ptr::null_mut();
        hip_call! { hipMalloc(&mut out_b, output.len() * mem::size_of::<Output>()) };
        let mut args = if let Some(input) = input {
            hip_call! { hipMalloc(&mut inp_b, input.len() * mem::size_of::<Input>()) };
            hip_call! { hipMemcpyWithStream(inp_b, input.as_ptr() as _, input.len() * mem::size_of::<Input>(), hipMemcpyKind::hipMemcpyHostToDevice, stream) };
            [&inp_b, &out_b]
        } else {
            [&out_b, &out_b]
        };
        hip_call! { hipMemsetAsync(out_b, 0, output.len() * mem::size_of::<Output>(), stream) };
        hip_call! { hipModuleLaunchKernel(kernel, 1,1,1, block_size[0],block_size[1],block_size[2], 1024, stream, args.as_mut_ptr().cast(), ptr::null_mut()) };
        hip_call! { hipMemcpyAsync(result.as_mut_ptr() as _, out_b, output.len() * mem::size_of::<Output>(), hipMemcpyKind::hipMemcpyDeviceToHost, stream) };
        hip_call! { hipStreamSynchronize(stream) };
        hip_call! { hipFree(inp_b) };
        hip_call! { hipFree(out_b) };
        hip_call! { hipModuleUnload(module) };
    }
    Ok(result)
}

unsafe fn test_llvm_assert<'a>(
    ptx_txt: &'a str,
    llvm_ir: &'a [u8],
    llvm_file_name: &'a str,
) -> Result<(), Box<dyn error::Error + 'a>> {
    let mut errors = Vec::new();
    let ast = ptx::ModuleParser::new().parse(&mut errors, ptx_txt)?;
    assert!(errors.len() == 0);
    let llvm_module_from_ptx = translate::to_llvm_module(CompilationMode::Wave32, vec![ast])?;
    let llvm_bitcode_from_ptx = llvm_module_from_ptx.get_bitcode_main();
    let mut llvm_ir_copy = llvm_ir.to_vec();
    llvm_ir_copy.push(0);
    let reference_llvm_ir_buffer = llvm::MemoryBuffer::create_no_copy(&*llvm_ir_copy, true);
    let reference_module = llvm::parse_ir_in_context(
        &llvm_module_from_ptx._llvm_context,
        reference_llvm_ir_buffer,
    )?;
    let reference_llvm_bitcode_buffer =
        llvm::MemoryBuffer::from_ffi(LLVMWriteBitcodeToMemoryBuffer(reference_module.get()));
    if reference_llvm_bitcode_buffer.as_slice() != llvm_bitcode_from_ptx.as_slice() {
        let ptx_string = llvm_module_from_ptx.get_llvm_text();
        if ptx_string.as_cstr().to_bytes() != llvm_ir {
            if let Ok(dump_path) = env::var("ZLUDA_TEST_LLVM_DUMP_DIR") {
                let mut path = PathBuf::from(dump_path);
                if let Ok(()) = fs::create_dir_all(&path) {
                    path.push(llvm_file_name);
                    fs::write(path, &*ptx_string.as_cstr().to_string_lossy()).ok();
                }
            }
            return Err(ptx_string.into());
        }
    }
    Ok(())
}

unsafe fn test_compile_assert<'a>(ptx_txt: &'a str) -> Result<(), Box<dyn error::Error + 'a>> {
    let mut errors = Vec::new();
    let ast = ptx::ModuleParser::new().parse(&mut errors, ptx_txt)?;
    assert!(errors.is_empty());
    let zluda_module = translate::to_llvm_module(CompilationMode::Wave32, vec![ast])?;
    let comgr = get_comgr().unwrap();
    let compilation_mode = CompilationMode::Wave32;
    let isa = unsafe { CStr::from_bytes_with_nul_unchecked(b"amdgcn-amd-amdhsa--gfx1030\0") };
    comgr
        .compile(
            compilation_mode,
            isa,
            zluda_module.get_bitcode_all(),
            &zluda_module.metadata.to_elf_section(),
        )
        .unwrap();
    Ok(())
}
pub(crate) struct CudaTestLibrary {
    pub(crate) lib_handle: libloading::Library,
}

impl CudaTestLibrary {
    // We use full path because otherwise we will open ZLUDA's CUDA binary from target/debug
    #[cfg(target_os = "windows")]
    const CUDA_PATH: &'static str = "C:\\Windows\\System32\\nvcuda.dll";
    #[cfg(not(target_os = "windows"))]
    const CUDA_PATH: &'static str = "/usr/lib/x86_64-linux-gnu/libcuda.so";

    unsafe fn new() -> Self {
        let lib_handle = libloading::Library::new(Self::CUDA_PATH).unwrap();
        Self { lib_handle }
    }
}

macro_rules! emit_cuda_fn_table {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:path);*) => {
        impl CudaTestLibrary {
            $(
                #[allow(dead_code)]
                unsafe fn $fn_name(&self, $($arg_id : $arg_type),*) {
                    let fn_ = self.lib_handle.get::<unsafe extern $abi fn ( $($arg_type),* ) -> $ret_type>(stringify!($fn_name).as_bytes()).unwrap();
                    let result = fn_($($arg_id),*);
                    if result != cuda_types::CUresult::CUDA_SUCCESS {
                        panic!("{:?}", result);
                    }
                }
            )*
        }
    };
}

use cuda_base::cuda_function_declarations;
cuda_function_declarations!(cuda_types, emit_cuda_fn_table, emit_cuda_fn_table, []);
