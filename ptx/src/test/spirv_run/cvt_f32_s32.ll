target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

declare float @__zluda_ptx_impl__cvt_rm_f32_s32(i32) #0

declare float @__zluda_ptx_impl__cvt_rn_f32_s32(i32) #0

declare float @__zluda_ptx_impl__cvt_rp_f32_s32(i32) #0

declare float @__zluda_ptx_impl__cvt_rz_f32_s32(i32) #0

define protected amdgpu_kernel void @cvt_f32_s32(ptr addrspace(4) byref(i64) %"50", ptr addrspace(4) byref(i64) %"51") #1 {
"76":
  %"10" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"10", align 1
  %"11" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"11", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"8" = alloca i32, align 4, addrspace(5)
  %"9" = alloca i32, align 4, addrspace(5)
  %"12" = load i64, ptr addrspace(4) %"50", align 8
  store i64 %"12", ptr addrspace(5) %"4", align 8
  %"13" = load i64, ptr addrspace(4) %"51", align 8
  store i64 %"13", ptr addrspace(5) %"5", align 8
  %"15" = load i64, ptr addrspace(5) %"4", align 8
  %"53" = inttoptr i64 %"15" to ptr
  %"52" = load i32, ptr %"53", align 4
  store i32 %"52", ptr addrspace(5) %"6", align 4
  %"17" = load i64, ptr addrspace(5) %"4", align 8
  %"54" = inttoptr i64 %"17" to ptr
  %"90" = getelementptr inbounds i8, ptr %"54", i64 4
  %"55" = load i32, ptr %"90", align 4
  store i32 %"55", ptr addrspace(5) %"7", align 4
  %"19" = load i64, ptr addrspace(5) %"4", align 8
  %"56" = inttoptr i64 %"19" to ptr
  %"92" = getelementptr inbounds i8, ptr %"56", i64 8
  %"57" = load i32, ptr %"92", align 4
  store i32 %"57", ptr addrspace(5) %"8", align 4
  %"21" = load i64, ptr addrspace(5) %"4", align 8
  %"58" = inttoptr i64 %"21" to ptr
  %"94" = getelementptr inbounds i8, ptr %"58", i64 12
  %"59" = load i32, ptr %"94", align 4
  store i32 %"59", ptr addrspace(5) %"9", align 4
  %"23" = load i32, ptr addrspace(5) %"6", align 4
  %"60" = call float @__zluda_ptx_impl__cvt_rn_f32_s32(i32 %"23")
  %"22" = bitcast float %"60" to i32
  store i32 %"22", ptr addrspace(5) %"6", align 4
  %"25" = load i32, ptr addrspace(5) %"7", align 4
  %"62" = call float @__zluda_ptx_impl__cvt_rz_f32_s32(i32 %"25")
  %"24" = bitcast float %"62" to i32
  store i32 %"24", ptr addrspace(5) %"7", align 4
  %"27" = load i32, ptr addrspace(5) %"8", align 4
  %"64" = call float @__zluda_ptx_impl__cvt_rm_f32_s32(i32 %"27")
  %"26" = bitcast float %"64" to i32
  store i32 %"26", ptr addrspace(5) %"8", align 4
  %"29" = load i32, ptr addrspace(5) %"9", align 4
  %"66" = call float @__zluda_ptx_impl__cvt_rp_f32_s32(i32 %"29")
  %"28" = bitcast float %"66" to i32
  store i32 %"28", ptr addrspace(5) %"9", align 4
  %"30" = load i64, ptr addrspace(5) %"5", align 8
  %"31" = load i32, ptr addrspace(5) %"6", align 4
  %"68" = inttoptr i64 %"30" to ptr addrspace(1)
  %"69" = bitcast i32 %"31" to float
  store float %"69", ptr addrspace(1) %"68", align 4
  %"32" = load i64, ptr addrspace(5) %"5", align 8
  %"33" = load i32, ptr addrspace(5) %"7", align 4
  %"70" = inttoptr i64 %"32" to ptr addrspace(1)
  %"96" = getelementptr inbounds i8, ptr addrspace(1) %"70", i64 4
  %"71" = bitcast i32 %"33" to float
  store float %"71", ptr addrspace(1) %"96", align 4
  %"34" = load i64, ptr addrspace(5) %"5", align 8
  %"35" = load i32, ptr addrspace(5) %"8", align 4
  %"72" = inttoptr i64 %"34" to ptr addrspace(1)
  %"98" = getelementptr inbounds i8, ptr addrspace(1) %"72", i64 8
  %"73" = bitcast i32 %"35" to float
  store float %"73", ptr addrspace(1) %"98", align 4
  %"36" = load i64, ptr addrspace(5) %"5", align 8
  %"37" = load i32, ptr addrspace(5) %"9", align 4
  %"74" = inttoptr i64 %"36" to ptr addrspace(1)
  %"100" = getelementptr inbounds i8, ptr addrspace(1) %"74", i64 12
  %"75" = bitcast i32 %"37" to float
  store float %"75", ptr addrspace(1) %"100", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
