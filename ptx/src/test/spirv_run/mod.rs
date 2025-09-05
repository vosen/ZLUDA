use super::read_test_file;
use crate::pass;
use comgr::Comgr;
use cuda_types::cuda::CUstream;
use hip_runtime_sys::hipError_t;
use pretty_assertions;
use std::env;
use std::error;
use std::ffi::{CStr, CString};
use std::fmt::{self, Debug, Display, Formatter};
use std::fs::{self, File};
use std::io::Write;
use std::mem;
use std::path::Path;
use std::ptr;
use std::str;

macro_rules! test_ptx_llvm {
    ($fn_name:ident) => {
        paste::item! {
            #[test]
            fn [<$fn_name _llvm>]() -> Result<(), Box<dyn std::error::Error>> {
                let ptx = read_test_file!(concat!(stringify!($fn_name), ".ptx"));
                let ll = read_test_file!(concat!("../ll/", stringify!($fn_name), ".ll"));
                test_llvm_assert(stringify!($fn_name), &ptx, ll.trim())
            }
        }
    };
}

macro_rules! test_ptx {
    ($fn_name:ident, $input:expr, $output:expr) => {
        paste::item! {
            #[test]
            fn [<$fn_name _amdgpu>]() -> Result<(), Box<dyn std::error::Error>> {
                let ptx = read_test_file!(concat!(stringify!($fn_name), ".ptx"));
                let input = $input;
                let output = $output;
                test_hip_assert(stringify!($fn_name), &ptx, Some(&input), &output, 1)
            }
        }

        paste::item! {
            #[test]
            fn [<$fn_name _cuda>]() -> Result<(), Box<dyn std::error::Error>> {
                let ptx = read_test_file!(concat!(stringify!($fn_name), ".ptx"));
                let input = $input;
                let output = $output;
                test_cuda_assert(stringify!($fn_name), &ptx, Some(&input), &output, 1)
            }
        }

        test_ptx_llvm!($fn_name);
    };

    ($fn_name:ident) => {
        test_ptx_llvm!($fn_name);
    };
}

macro_rules! test_ptx_warp {
    ($fn_name:ident, $output:expr) => {
        paste::item! {
            #[test]
            fn [<$fn_name _amdgpu>]() -> Result<(), Box<dyn std::error::Error>> {
                let ptx = read_test_file!(concat!(stringify!($fn_name), ".ptx"));
                let mut output = $output;
                test_hip_assert(stringify!($fn_name), &ptx, None::<&[u8]>, &mut output, 64)
            }
        }

        paste::item! {
            #[test]
            fn [<$fn_name _cuda>]() -> Result<(), Box<dyn std::error::Error>> {
                let ptx = read_test_file!(concat!(stringify!($fn_name), ".ptx"));
                let mut output = $output;
                test_cuda_assert(stringify!($fn_name), &ptx, None::<&[u8]>, &mut output, 64)
            }
        }

        test_ptx_llvm!($fn_name);
    };
}

test_ptx!(ld_st, [1u64], [1u64]);
test_ptx!(ld_st_implicit, [0.5f32, 0.25f32], [0.5f32]);
test_ptx!(mov, [1u64], [1u64]);
test_ptx!(mul_lo, [1u64], [2u64]);
test_ptx!(mul_hi, [u64::max_value()], [1u64]);
test_ptx!(add, [1u64], [2u64]);
test_ptx!(
    mul24_lo_u32,
    [0b01110101_01010101_01010101u32],
    [0b00011100_00100011_10001110_00111001u32]
);
test_ptx!(
    mul24_hi_u32,
    [0b01110101_01010101_01010101u32],
    [0b00110101_11000111_00011100_00100011u32]
);
test_ptx!(
    mul24_lo_s32,
    [0b01110101_01010101_01010101i32],
    [-0b0011100_00100011_10001110_00111001i32]
);
test_ptx!(
    mul24_hi_s32,
    [0b01110101_01010101_01010101i32],
    [-0b0110101_11000111_00011100_00100100i32]
);
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
test_ptx!(mad_s32, [2i32, 3i32, 4i32], [10i32]);
test_ptx!(mad_wide, [-1i32, 3, 4, 5], [21474836481i64]);
test_ptx!(
    mul_wide,
    [0x01_00_00_00__01_00_00_00i64],
    [0x1_00_00_00_00_00_00i64]
);
test_ptx!(vector_extract, [1u8, 2u8, 3u8, 4u8], [3u8, 4u8, 1u8, 2u8]);
test_ptx!(shr, [-2i32], [-1i32]);
test_ptx!(shr_oob, [-32768i16], [-1i16]);
test_ptx!(or, [1u64, 2u64], [3u64]);
test_ptx!(sub, [2u64], [1u64]);
test_ptx!(min, [555i32, 444i32], [444i32]);
test_ptx!(max, [555i32, 444i32], [555i32]);
test_ptx!(global_array, [0xDEADu32], [1u32]);
test_ptx!(global_array_f32, [0x0], [0f32]);
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
test_ptx!(abs, [i32::MIN], [i32::MIN]);
test_ptx!(constant_negative, [-101i32], [101i32]);
test_ptx!(and, [6u32, 3u32], [2u32]);
test_ptx!(selp, [100u16, 200u16], [200u16]);
test_ptx!(selp_true, [100u16, 200u16], [100u16]);
test_ptx!(fma, [2f32, 3f32, 5f32], [11f32]);
test_ptx!(
    fma_bf16x2,
    [0x40004040, 0x40404080, 0x40A04040],
    [0x41304170]
);
test_ptx!(shared_variable, [513u64], [513u64]);
test_ptx!(shared_ptr_32, [513u64], [513u64]);
test_ptx!(atom_cas, [91u32, 91u32], [91u32, 100u32]);
test_ptx!(atom_inc, [100u32], [100u32, 101u32, 0u32]);
test_ptx!(atom_add, [2u32, 4u32], [2u32, 6u32]);
test_ptx!(div_approx, [1f32, 2f32], [0.5f32]);
test_ptx!(sqrt, [0.25f32], [0.5f32]);
test_ptx!(sqrt_rn_ftz, [0x1u32], [0x0u32]);
test_ptx!(rsqrt, [0.25f64], [2f64]);
test_ptx!(neg, [181i32], [-181i32]);
test_ptx!(sin, [std::f32::consts::PI / 2f32], [1f32]);
test_ptx!(cos, [std::f32::consts::PI], [-1f32]);
test_ptx!(lg2, [512f32], [9f32]);
test_ptx!(ex2, [10f32], [1024f32]);
test_ptx!(fmax, [0u16, half::f16::NAN.to_bits()], [0u16]);
test_ptx!(cvt_rni, [9.5f32, 10.5f32], [10f32, 10f32]);
test_ptx!(cvt_rzi, [-13.8f32, 12.9f32], [-13f32, 12f32]);
test_ptx!(cvt_s32_f32, [-13.8f32, 12.9f32], [-13i32, 13i32]);
test_ptx!(cvt_rni_u16_f32, [0x477FFF80u32], [65535u16]);
test_ptx!(cvt_rn_satfinite_e4m3x2_f32, [0.40625, 12.9f32], [0x2D55u16]);
test_ptx!(
    cvt_rn_satfinite_e5m2x2_f32,
    [0.375, -5256.6f32],
    [0x36EDu16]
);
test_ptx!(cvt_rn_f16x2_e4m3x2, [0x2D55u16], [0x36804a80u32]);
test_ptx!(cvt_rn_f16x2_e5m2x2, [0x36EDu16], [0x3600ED00u32]);
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
test_ptx!(const_ident, [0u16], [0u64, 0u64]);
test_ptx!(cvt_s16_s8, [0x139231C2u32], [0xFFFFFFC2u32]);
test_ptx!(cvt_f64_f32, [0.125f32], [0.125f64]);
test_ptx!(prmt, [0x70c507d6u32, 0x6fbd4b5cu32], [0x6fbdd65cu32]);
test_ptx!(activemask, [0u32], [1u32]);
test_ptx!(membar, [152731u32], [152731u32]);
test_ptx!(shared_unify_extern, [7681u64, 7682u64], [15363u64]);
test_ptx!(shared_unify_local, [16752u64, 714u64], [17466u64]);
// FIXME: This test currently fails for reasons outside of ZLUDA's control.
// One of the LLVM passes does not understand that setreg instruction changes
// global floating point state and assumes that both floating point
// additions are the exact same expressions and optimizes second addition away.
// test_ptx!(
//     add_ftz,
//     [f32::from_bits(0x800000), f32::from_bits(0x007FFFFF)],
//     [0x800000u32, 0xFFFFFF]
// );
test_ptx!(add_s32_sat, [i32::MIN, -1], [i32::MIN, i32::MAX]);
test_ptx!(malformed_label, [2u64], [3u64]);
test_ptx!(
    call_rnd,
    [
        1.0f32,
        f32::from_bits(0x33800000),
        1.0f32,
        f32::from_bits(0x33800000)
    ],
    [1.0000001, 1.0f32]
);
test_ptx!(multiple_return, [5u32], [6u32, 123u32]);
test_ptx!(warp_sz, [0u8], [32u8]);
test_ptx!(tanh, [f32::INFINITY], [1.0f32]);
test_ptx!(cp_async, [0u32], [1u32, 2u32, 3u32, 0u32]);
// Two test below test very important compiler feature, make sure that you
// understand fully what's going on before you touch it.
// The problem is that the full-precision division gets legalized by LLVM
// using __module attribute__.
// In the two tests below we deliberately force our compiler to emit
// different a module that has a different module-level denormal attribute
// from the denormal attribute of the instruction to catch cases like this
test_ptx!(div_ftz, [0x16A2028Du32, 0x5E89F6AE], [0x0, 900636404u32]);
test_ptx!(
    div_noftz,
    [0x16A2028Du32, 0x5E89F6AE],
    [0x26u32, 900636404u32]
);

test_ptx!(nanosleep, [0u64], [0u64]);
test_ptx!(shf_l, [0x12345678u32, 0x9abcdef0u32, 12], [0xcdef0123u32]);
test_ptx!(shf_r, [0x12345678u32, 0x9abcdef0u32, 12], [0xef012345u32]);
test_ptx!(
    shf_l_clamp,
    [0x12345678u32, 0x9abcdef0u32, 44],
    [0x12345678u32]
);
test_ptx!(
    shf_r_clamp,
    [0x12345678u32, 0x9abcdef0u32, 44],
    [0x9abcdef0u32]
);
test_ptx!(
    shf_l_wrap,
    [0x12345678u32, 0x9abcdef0u32, 44],
    [0xcdef0123u32]
);
test_ptx!(
    shf_r_wrap,
    [0x12345678u32, 0x9abcdef0u32, 44],
    [0xef012345u32]
);
test_ptx!(
    dp4a,
    [0x8e2da590u32, 0xedeaee14, 0x248a9f70],
    [613065134u32]
);

test_ptx!(assertfail);
// TODO: not yet supported
//test_ptx!(func_ptr);
test_ptx!(lanemask_lt);
test_ptx!(extern_func);
test_ptx!(trap);

test_ptx_warp!(
    tid,
    [
        0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8, 16u8,
        17u8, 18u8, 19u8, 20u8, 21u8, 22u8, 23u8, 24u8, 25u8, 26u8, 27u8, 28u8, 29u8, 30u8, 31u8,
        32u8, 33u8, 34u8, 35u8, 36u8, 37u8, 38u8, 39u8, 40u8, 41u8, 42u8, 43u8, 44u8, 45u8, 46u8,
        47u8, 48u8, 49u8, 50u8, 51u8, 52u8, 53u8, 54u8, 55u8, 56u8, 57u8, 58u8, 59u8, 60u8, 61u8,
        62u8, 63u8,
    ]
);
test_ptx_warp!(
    bar_red_and_pred,
    [
        2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32,
        2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32,
        2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32,
        2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32, 2u32,
        2u32, 2u32, 2u32, 2u32,
    ]
);
test_ptx_warp!(
    shfl_sync_up_b32_pred,
    [
        1000u32, 1001u32, 1002u32, 0u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32,
        10u32, 11u32, 12u32, 13u32, 14u32, 15u32, 16u32, 17u32, 18u32, 19u32, 20u32, 21u32, 22u32,
        23u32, 24u32, 25u32, 26u32, 27u32, 28u32, 1032u32, 1033u32, 1034u32, 32u32, 33u32, 34u32,
        35u32, 36u32, 37u32, 38u32, 39u32, 40u32, 41u32, 42u32, 43u32, 44u32, 45u32, 46u32, 47u32,
        48u32, 49u32, 50u32, 51u32, 52u32, 53u32, 54u32, 55u32, 56u32, 57u32, 58u32, 59u32, 60u32,
    ]
);
test_ptx_warp!(
    shfl_sync_down_b32_pred,
    [
        3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32, 10u32, 11u32, 12u32, 13u32, 14u32, 15u32, 16u32,
        17u32, 18u32, 19u32, 20u32, 21u32, 22u32, 23u32, 24u32, 25u32, 26u32, 27u32, 28u32, 29u32,
        30u32, 31u32, 1029u32, 1030u32, 1031u32, 35u32, 36u32, 37u32, 38u32, 39u32, 40u32, 41u32,
        42u32, 43u32, 44u32, 45u32, 46u32, 47u32, 48u32, 49u32, 50u32, 51u32, 52u32, 53u32, 54u32,
        55u32, 56u32, 57u32, 58u32, 59u32, 60u32, 61u32, 62u32, 63u32, 1061u32, 1062u32, 1063u32,
    ]
);
test_ptx_warp!(
    shfl_sync_bfly_b32_pred,
    [
        3u32, 2u32, 1u32, 0u32, 7u32, 6u32, 5u32, 4u32, 11u32, 10u32, 9u32, 8u32, 15u32, 14u32,
        13u32, 12u32, 19u32, 18u32, 17u32, 16u32, 23u32, 22u32, 21u32, 20u32, 27u32, 26u32, 25u32,
        24u32, 31u32, 30u32, 29u32, 28u32, 35u32, 34u32, 33u32, 32u32, 39u32, 38u32, 37u32, 36u32,
        43u32, 42u32, 41u32, 40u32, 47u32, 46u32, 45u32, 44u32, 51u32, 50u32, 49u32, 48u32, 55u32,
        54u32, 53u32, 52u32, 59u32, 58u32, 57u32, 56u32, 63u32, 62u32, 61u32, 60u32,
    ]
);
test_ptx_warp!(
    shfl_sync_idx_b32_pred,
    [
        12u32, 12u32, 12u32, 12u32, 12u32, 12u32, 12u32, 12u32, 12u32, 12u32, 12u32, 12u32, 12u32,
        12u32, 12u32, 12u32, 12u32, 12u32, 12u32, 12u32, 12u32, 12u32, 12u32, 12u32, 12u32, 12u32,
        12u32, 12u32, 12u32, 12u32, 12u32, 12u32, 44u32, 44u32, 44u32, 44u32, 44u32, 44u32, 44u32,
        44u32, 44u32, 44u32, 44u32, 44u32, 44u32, 44u32, 44u32, 44u32, 44u32, 44u32, 44u32, 44u32,
        44u32, 44u32, 44u32, 44u32, 44u32, 44u32, 44u32, 44u32, 44u32, 44u32, 44u32, 44u32,
    ]
);
test_ptx_warp!(
    shfl_sync_mode_b32,
    [
        9u32, 7u32, 8u32, 9u32, 21u32, 19u32, 20u32, 21u32, 33u32, 31u32, 32u32, 33u32, 45u32,
        43u32, 44u32, 45u32, 73u32, 71u32, 72u32, 73u32, 85u32, 83u32, 84u32, 85u32, 97u32, 95u32,
        96u32, 97u32, 109u32, 107u32, 108u32, 109u32, 137u32, 135u32, 136u32, 137u32, 149u32,
        147u32, 148u32, 149u32, 161u32, 159u32, 160u32, 161u32, 173u32, 171u32, 172u32, 173u32,
        201u32, 199u32, 200u32, 201u32, 213u32, 211u32, 212u32, 213u32, 225u32, 223u32, 224u32,
        225u32, 237u32, 235u32, 236u32, 237u32,
    ]
);
test_ptx_warp!(
    vote_all,
    [
        0u32, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1
    ]
);
test_ptx_warp!(
    vote_all_sub,
    [
        0u32, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1
    ]
);
test_ptx_warp!(
    vote_any,
    [
        1u32, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0
    ]
);
test_ptx_warp!(
    vote_ballot,
    [
        0u32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 4294967292, 4294967292, 4294967292, 4294967292, 4294967292, 4294967292,
        4294967292, 4294967292, 4294967292, 4294967292, 4294967292, 4294967292, 4294967292,
        4294967292, 4294967292, 4294967292, 4294967292, 4294967292, 4294967292, 4294967292,
        4294967292, 4294967292, 4294967292, 4294967292, 4294967292, 4294967292, 4294967292,
        4294967292, 4294967292, 4294967292, 4294967292, 4294967292
    ]
);

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
    Input: From<u8> + Debug + Copy + PartialEq,
    Output: From<u8> + Debug + Copy + PartialEq + Default,
>(
    name: &str,
    ptx_text: &str,
    input: Option<&[Input]>,
    output: &[Output],
    block_dim_x: u32,
) -> Result<(), Box<dyn error::Error>> {
    let ast = ptx_parser::parse_module_checked(ptx_text).unwrap();
    let llvm_ir = pass::to_llvm_module(
        ast,
        pass::Attributes {
            clock_rate: 2124000,
        },
    )
    .unwrap();
    let name = CString::new(name)?;
    let result = run_hip(name.as_c_str(), llvm_ir, input, output, block_dim_x)
        .map_err(|err| DisplayError { err })?;
    assert_eq!(result.as_slice(), output);
    Ok(())
}

fn test_llvm_assert(
    name: &str,
    ptx_text: &str,
    expected_ll: &str,
) -> Result<(), Box<dyn error::Error>> {
    let ast = ptx_parser::parse_module_checked(ptx_text).unwrap();
    let llvm_ir = pass::to_llvm_module(
        ast,
        pass::Attributes {
            clock_rate: 2124000,
        },
    )
    .unwrap();
    let actual_ll = llvm_ir.llvm_ir.print_module_to_string();
    let actual_ll = actual_ll.to_str();
    compare_llvm(name, actual_ll, expected_ll);

    let expected_attributes_ll = read_test_file!(concat!("../ll/_attributes.ll"));
    let actual_attributes_ll = llvm_ir.attributes_ir.print_module_to_string();
    let actual_attributes_ll = actual_attributes_ll.to_str();
    compare_llvm("_attributes", actual_attributes_ll, &expected_attributes_ll);
    Ok(())
}

fn compare_llvm(name: &str, actual_ll: &str, expected_ll: &str) {
    if actual_ll != expected_ll {
        let output_dir = env::var("TEST_PTX_LLVM_FAIL_DIR");
        if let Ok(output_dir) = output_dir {
            let output_dir = Path::new(&output_dir);
            fs::create_dir_all(&output_dir).unwrap();
            let output_file = output_dir.join(format!("{}.ll", name));
            let mut output_file = File::create(output_file).unwrap();
            output_file.write_all(actual_ll.as_bytes()).unwrap();
        }
        let comparison = pretty_assertions::StrComparison::new(&expected_ll, &actual_ll);
        panic!("assertion failed: `(left == right)`\n\n{}", comparison);
    }
}

fn test_cuda_assert<
    Input: From<u8> + Debug + Copy + PartialEq,
    Output: From<u8> + Debug + Copy + PartialEq + Default,
>(
    name: &str,
    ptx_text: &str,
    input: Option<&[Input]>,
    output: &[Output],
    block_dim_x: u32,
) -> Result<(), Box<dyn error::Error>> {
    let name = CString::new(name)?;
    let result = run_cuda(name.as_c_str(), ptx_text, input, output, block_dim_x);
    assert_eq!(result.as_slice(), output);
    Ok(())
}

fn run_cuda<Input: From<u8> + Copy + Debug, Output: From<u8> + Copy + Debug + Default>(
    name: &CStr,
    ptx_module: &str,
    input: Option<&[Input]>,
    output: &[Output],
    block_dim_x: u32,
) -> Vec<Output> {
    unsafe { CUDA.cuInit(0) }.unwrap().unwrap();
    let ptx_module = CString::new(ptx_module).unwrap();
    let mut result = vec![0u8.into(); output.len()];
    {
        let mut ctx = unsafe { mem::zeroed() };
        unsafe { CUDA.cuCtxCreate_v2(&mut ctx, 0, 0) }
            .unwrap()
            .unwrap();
        let mut module = unsafe { mem::zeroed() };
        unsafe { CUDA.cuModuleLoadData(&mut module, ptx_module.as_ptr() as _) }
            .unwrap()
            .unwrap();
        let mut kernel = unsafe { mem::zeroed() };
        unsafe { CUDA.cuModuleGetFunction(&mut kernel, module, name.as_ptr()) }
            .unwrap()
            .unwrap();
        let mut out_b = unsafe { mem::zeroed() };
        unsafe { CUDA.cuMemAlloc_v2(&mut out_b, output.len() * mem::size_of::<Output>()) }
            .unwrap()
            .unwrap();
        let mut inp_b = unsafe { mem::zeroed() };
        if let Some(input) = input {
            unsafe { CUDA.cuMemAlloc_v2(&mut inp_b, input.len() * mem::size_of::<Input>()) }
                .unwrap()
                .unwrap();
            unsafe {
                CUDA.cuMemcpyHtoD_v2(
                    inp_b,
                    input.as_ptr() as _,
                    input.len() * mem::size_of::<Input>(),
                )
            }
            .unwrap()
            .unwrap();
        }
        unsafe { CUDA.cuMemsetD8_v2(out_b, 0, output.len() * mem::size_of::<Output>()) }
            .unwrap()
            .unwrap();
        let mut args = if input.is_some() {
            [&inp_b, &out_b]
        } else {
            [&out_b, &out_b]
        };
        unsafe {
            CUDA.cuLaunchKernel(
                kernel,
                1,
                1,
                1,
                block_dim_x,
                1,
                1,
                1024,
                CUstream(ptr::null_mut()),
                args.as_mut_ptr() as _,
                ptr::null_mut(),
            )
        }
        .unwrap()
        .unwrap();
        unsafe {
            CUDA.cuMemcpyDtoH_v2(
                result.as_mut_ptr() as _,
                out_b,
                output.len() * mem::size_of::<Output>(),
            )
        }
        .unwrap()
        .unwrap();
        unsafe { CUDA.cuStreamSynchronize(CUstream(ptr::null_mut())) }
            .unwrap()
            .unwrap();
        unsafe { CUDA.cuMemFree_v2(inp_b) }.unwrap().unwrap();
        unsafe { CUDA.cuMemFree_v2(out_b) }.unwrap().unwrap();
        unsafe { CUDA.cuModuleUnload(module) }.unwrap().unwrap();
        unsafe { CUDA.cuCtxDestroy_v2(ctx) }.unwrap().unwrap();
    }
    result
}

struct DynamicCuda {
    lib: libloading::Library,
}

impl DynamicCuda {
    #[cfg(not(windows))]
    const CUDA_PATH: &'static str = "/usr/lib/x86_64-linux-gnu/libcuda.so.1";
    #[cfg(windows)]
    const CUDA_PATH: &'static str = "C:\\Windows\\System32\\nvcuda.dll";

    pub fn new() -> Result<Self, libloading::Error> {
        let lib = unsafe { libloading::Library::new(Self::CUDA_PATH) }?;
        Ok(Self { lib })
    }
}

macro_rules! dynamic_fns {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        impl DynamicCuda {
        $(
            #[allow(dead_code)]
            unsafe fn $fn_name(&self, $($arg_id : $arg_type),*) -> Result<$ret_type, libloading::Error> {
                let func = unsafe { self.lib.get::<unsafe extern $abi fn ($($arg_type),*) -> $ret_type>(concat!(stringify!($fn_name), "\0").as_bytes()) };
                func.map(|f| f($($arg_id),*) )
            }
        )*
        }
    };
}

cuda_macros::cuda_function_declarations!(dynamic_fns);

static COMGR: std::sync::LazyLock<Comgr> = std::sync::LazyLock::new(|| Comgr::new().unwrap());
static CUDA: std::sync::LazyLock<DynamicCuda> =
    std::sync::LazyLock::new(|| DynamicCuda::new().unwrap());

fn run_hip<Input: From<u8> + Copy + Debug, Output: From<u8> + Copy + Debug + Default>(
    name: &CStr,
    module: pass::Module,
    input: Option<&[Input]>,
    output: &[Output],
    block_dim_x: u32,
) -> Result<Vec<Output>, hipError_t> {
    use hip_runtime_sys::*;
    unsafe { hipInit(0) }.unwrap();
    let comgr = &*COMGR;
    let mut result = vec![0u8.into(); output.len()];
    {
        let dev = 0;
        let mut stream = unsafe { mem::zeroed() };
        unsafe { hipStreamCreate(&mut stream) }.unwrap();
        let mut dev_props = unsafe { mem::zeroed() };
        unsafe { hipGetDevicePropertiesR0600(&mut dev_props, dev) }.unwrap();
        let elf_module = comgr::compile_bitcode(
            &comgr,
            unsafe { CStr::from_ptr(dev_props.gcnArchName.as_ptr()) }
                .to_str()
                .unwrap(),
            &*module.llvm_ir.write_bitcode_to_memory(),
            module.linked_bitcode(),
            &*module.attributes_ir.write_bitcode_to_memory(),
            None,
        )
        .unwrap();
        // TODO: Re-enable when we are able to privatize function-scoped
        // globals and constants
        // let fns = comgr::get_symbols(&comgr, &elf_module).unwrap();
        // verify_symbols(fns);
        let mut module = unsafe { mem::zeroed() };
        unsafe { hipModuleLoadData(&mut module, elf_module.as_ptr() as _) }.unwrap();
        let mut kernel = unsafe { mem::zeroed() };
        unsafe { hipModuleGetFunction(&mut kernel, module, name.as_ptr()) }.unwrap();
        let mut out_b = ptr::null_mut();
        unsafe { hipMalloc(&mut out_b, output.len() * mem::size_of::<Output>()) }.unwrap();
        let mut inp_b = ptr::null_mut();
        if let Some(input) = input {
            unsafe { hipMalloc(&mut inp_b, input.len() * mem::size_of::<Input>()) }.unwrap();
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
        }
        unsafe { hipMemset(out_b, 0, output.len() * mem::size_of::<Output>()) }.unwrap();
        let mut args = if input.is_some() {
            [&inp_b, &out_b]
        } else {
            [&out_b, &out_b]
        };
        unsafe {
            hipModuleLaunchKernel(
                kernel,
                1,
                1,
                1,
                block_dim_x,
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

// TODO: Re-enable when we are able to privatize function-scoped
// globals and constants
/*
fn verify_symbols(mut symbols: Vec<(u32, String)>) {
    symbols.sort();
    if symbols.len() != 2 {
        panic!("Expected exactly two symbols, found: {:?}", symbols);
    }
    assert_eq!(
        symbols[0].0, 1,
        "Wrong symbols exported from binary: {:?}",
        symbols
    );
    assert_eq!(
        symbols[1].0, 2,
        "Wrong symbols exported from binary: {:?}",
        symbols
    );
    assert_eq!(
        symbols[0].1,
        format!("{}.kd", symbols[1].1),
        "Wrong symbols exported from binary: {:?}",
        symbols
    );
}
 */
