#!/usr/bin/env python3

import os
import itertools
from itertools import chain, islice


# rg __[a-z]+2[a-z]+_r[nduz] -o -N --no-filename | sort | uniq | sed -e 's/\(^.*$\)/    \"\1\",/'
HIP_CONV = [
    "__double2float_rd",
    "__double2float_rn",
    "__double2float_ru",
    "__double2float_rz",
    "__double2int_rd",
    "__double2int_rn",
    "__double2int_ru",
    "__double2int_rz",
    "__double2ll_rd",
    "__double2ll_rn",
    "__double2ll_ru",
    "__double2ll_rz",
    "__double2uint_rd",
    "__double2uint_rn",
    "__double2uint_ru",
    "__double2uint_rz",
    "__double2ull_rd",
    "__double2ull_rn",
    "__double2ull_ru",
    "__double2ull_rz",
    "__float2half_rd",
    "__float2half_rd",
    "__float2half_rd",
    "__float2half_rn",
    "__float2half_rn",
    "__float2half_rn",
    "__float2half_rn",
    "__float2half_ru",
    "__float2half_ru",
    "__float2half_ru",
    "__float2half_rz",
    "__float2half_rz",
    "__float2half_rz",
    "__float2int_rd",
    "__float2int_rn",
    "__float2int_ru",
    "__float2int_rz",
    "__float2ll_rd",
    "__float2ll_rn",
    "__float2ll_ru",
    "__float2ll_rz",
    "__float2uint_rd",
    "__float2uint_rn",
    "__float2uint_ru",
    "__float2uint_rz",
    "__float2ull_rd",
    "__float2ull_rn",
    "__float2ull_ru",
    "__float2ull_rz",
    "__half2int_rd",
    "__half2int_rn",
    "__half2int_ru",
    "__half2int_rz",
    "__half2ll_rd",
    "__half2ll_rn",
    "__half2ll_ru",
    "__half2ll_rz",
    "__half2short_rd",
    "__half2short_rn",
    "__half2short_ru",
    "__half2short_rz",
    "__half2uint_rd",
    "__half2uint_rn",
    "__half2uint_ru",
    "__half2uint_rz",
    "__half2ull_rd",
    "__half2ull_rn",
    "__half2ull_ru",
    "__half2ull_rz",
    "__half2ushort_rd",
    "__half2ushort_rn",
    "__half2ushort_ru",
    "__half2ushort_rz",
    "__int2double_rn",
    "__int2float_rd",
    "__int2float_rn",
    "__int2float_ru",
    "__int2float_rz",
    "__int2half_rd",
    "__int2half_rn",
    "__int2half_ru",
    "__int2half_rz",
    "__ll2double_rd",
    "__ll2double_rn",
    "__ll2double_ru",
    "__ll2double_rz",
    "__ll2float_rd",
    "__ll2float_rn",
    "__ll2float_ru",
    "__ll2float_rz",
    "__ll2half_rd",
    "__ll2half_rn",
    "__ll2half_ru",
    "__ll2half_rz",
    "__short2half_rd",
    "__short2half_rn",
    "__short2half_ru",
    "__short2half_rz",
    # broken in ROCm 5.6
    #"__uint2double_rn",
    "__uint2float_rd",
    "__uint2float_rn",
    "__uint2float_ru",
    "__uint2float_rz",
    "__uint2half_rd",
    "__uint2half_rn",
    "__uint2half_ru",
    "__uint2half_rz",
    "__ull2double_rd",
    "__ull2double_rn",
    "__ull2double_ru",
    "__ull2double_rz",
    "__ull2float_rd",
    "__ull2float_rn",
    "__ull2float_ru",
    "__ull2float_rz",
    "__ull2half_rd",
    "__ull2half_rn",
    "__ull2half_ru",
    "__ull2half_rz",
    "__ushort2half_rd",
    "__ushort2half_rn",
    "__ushort2half_ru",
    "__ushort2half_rz"
]

# rg  __ocml_cvtrt[eznp]_[^_]+_[^\(]+ -o -N --no-filename | sort | uniq | sed -e 's/\(^.*$\)/    \"\1\",/'
OCML_CONV = [
    "__ocml_cvtrtn_f16_f32",
    "__ocml_cvtrtn_f32_f64",
    "__ocml_cvtrtn_f32_s32",
    "__ocml_cvtrtn_f32_s64",
    "__ocml_cvtrtn_f32_u32",
    "__ocml_cvtrtn_f32_u64",
    "__ocml_cvtrtn_f64_s64",
    "__ocml_cvtrtn_f64_u64",
    "__ocml_cvtrtp_f16_f32",
    "__ocml_cvtrtp_f32_f64",
    "__ocml_cvtrtp_f32_s32",
    "__ocml_cvtrtp_f32_s64",
    "__ocml_cvtrtp_f32_u32",
    "__ocml_cvtrtp_f32_u64",
    "__ocml_cvtrtp_f64_s64",
    "__ocml_cvtrtp_f64_u64",
    "__ocml_cvtrtz_f16_f32",
    "__ocml_cvtrtz_f32_f64",
    "__ocml_cvtrtz_f32_s32",
    "__ocml_cvtrtz_f32_s64",
    "__ocml_cvtrtz_f32_u32",
    "__ocml_cvtrtz_f32_u64",
    "__ocml_cvtrtz_f64_s64",
    "__ocml_cvtrtz_f64_u64",
]


class Rounding:
    def __init__(self, ptx, hip, opencl):
        self.ptx = ptx
        self.hip = hip
        self.opencl = opencl


ROUNDING = [
    Rounding("rn", "rn", "rte"),
    Rounding("rz", "rz", "rtz"),
    Rounding("rm", "rd", "rtn"),
    Rounding("rp", "ru", "rtp")
]

class Type:
    def __init__(self, ptx, hip, opencl = None):
        self.ptx = ptx
        self.hip = hip
        if opencl is not None:
            self.opencl = opencl
        else:
            self.opencl = hip


SIGNED_TYPES = [
    Type("s8", "char", "char"),
    Type("s16", "short", "short"),
    Type("s32", "int", "int"),
    Type("s64", "ll", "long")
]


UNSIGNED_TYPES = [
    Type("u8", "uchar", "uchar"),
    Type("u16", "ushort", "ushort"),
    Type("u32", "uint", "uint"),
    Type("u64", "ull", "ulong")
]


FLOAT_TYPES = [
    Type("f16", "half"),
    Type("f32", "float"),
    Type("f64", "double")
]


def hip_func(from_, to, rnd):
    return f"__{from_.hip}2{to.hip}_{rnd.hip}"


def ocml_func(from_, to, rnd):
    return f"__ocml_cvt{rnd.hip}_{to.ptx}_{from_.ptx}"


def convert_func(from_, to, rnd):
    return f"convert_{to.opencl}_{rnd.opencl}"


def convert_func_decl(from_, to, rnd):
    return f"{to.opencl} convert_{to.opencl}_{rnd.opencl}({from_.opencl}) __attribute__((overloadable)) __attribute__((device)) __attribute__((const));"


def identity_iterator(x, _):
    return x


def emit_cvt(hip_convs, ocml_convs, from_types, to_types, iter_func):
    for idx, from_ in enumerate(from_types):
        for to in iter_func(to_types, idx):
            for rnd in ROUNDING:
                print(convert_func_decl(from_, to, rnd))
                hip = hip_func(from_, to, rnd)
                if hip in hip_convs:
                    if from_.ptx == "f16":
                        print(f"__attribute__((always_inline)) {to.opencl} FUNC(cvt_{rnd.ptx}_{to.ptx}_{from_.ptx})({from_.opencl} x) {{ return {hip}(reinterpret_cast<{from_.opencl}&>(x)); }}")
                    elif to.ptx == "f16":
                        print(f"__attribute__((always_inline)) {to.opencl} FUNC(cvt_{rnd.ptx}_{to.ptx}_{from_.ptx})({from_.opencl} x) {{ auto temp = {hip}(x); return *reinterpret_cast<{to.opencl}*>(&temp); }}")
                    else:
                        print(f"__attribute__((always_inline)) {to.opencl} FUNC(cvt_{rnd.ptx}_{to.ptx}_{from_.ptx})({from_.opencl} x) {{ return {hip}(x); }}")
                else:
                    ocml = ocml_func(from_, to, rnd)
                    if ocml in ocml_convs:
                        print(f"__attribute__((always_inline)) {to.opencl} FUNC(cvt_{rnd.ptx}_{to.ptx}_{from_.ptx})({from_.opencl} x) {{ return {ocml}(x); }}")
                    else:
                        print(f"__attribute__((always_inline)) {to.opencl} FUNC(cvt_{rnd.ptx}_{to.ptx}_{from_.ptx})({from_.opencl} x) {{ return {convert_func(from_, to, rnd)}(x); }}")


def main():
    hip_convs = set(HIP_CONV)
    ocml_convs = set(OCML_CONV)
    signed_types_ptx = ["s8", "s16", "s32", "s64"]
    signed_types_opencl = ["char", "short", "int", "long"]
    unsigned_types_ptx = ["u8", "u16", "u32", "u64"]
    unsigned_types_opencl = ["uchar", "ushort", "uint", "ulong"]
    float_types_ptx = ["f16", "f32", "f64"]
    float_types_opencl = ["half", "float", "double"]
    # somehow in LLVM `half` codegens to i16 and _Float codegens to half
    float_types_hip = ["half", "float", "double"]
    print(f"// GENERATED AUTOMATICALLY BY {os.path.basename(__file__)}, DON'T CHANGE MANUALLY")
    print("extern \"C\"")
    print("{")
    print("typedef __fp16 half;")
    # float from float truncation
    emit_cvt(hip_convs, ocml_convs, FLOAT_TYPES, FLOAT_TYPES, islice)
    # float from int, "Floating-point rounding is required for float-to-float conversions that result in loss of precision, and for integer-to-float conversions"
    emit_cvt(hip_convs, ocml_convs, SIGNED_TYPES, FLOAT_TYPES, identity_iterator)
    emit_cvt(hip_convs, ocml_convs, UNSIGNED_TYPES, FLOAT_TYPES, identity_iterator)
    # int from float, "Integer rounding is required for float-to-integer conversions"
    emit_cvt(hip_convs, ocml_convs, FLOAT_TYPES, SIGNED_TYPES, identity_iterator)
    emit_cvt(hip_convs, ocml_convs, FLOAT_TYPES, UNSIGNED_TYPES, identity_iterator)
    print("}")

if __name__ == "__main__":
    main()
