target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

declare float @__zluda_ptx_impl__cvt_rm_f32_s32(i32) #0

declare float @__zluda_ptx_impl__cvt_rn_f32_s32(i32) #0

declare float @__zluda_ptx_impl__cvt_rp_f32_s32(i32) #0

declare float @__zluda_ptx_impl__cvt_rz_f32_s32(i32) #0

define protected amdgpu_kernel void @cvt_f32_s32(ptr addrspace(4) byref(i64) %"49", ptr addrspace(4) byref(i64) %"50") #1 {
  %"10" = alloca i1, align 1, addrspace(5)
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"8" = alloca i32, align 4, addrspace(5)
  %"9" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  store i1 false, ptr addrspace(5) %"10", align 1
  %"11" = load i64, ptr addrspace(4) %"49", align 8
  store i64 %"11", ptr addrspace(5) %"4", align 8
  %"12" = load i64, ptr addrspace(4) %"50", align 8
  store i64 %"12", ptr addrspace(5) %"5", align 8
  %"14" = load i64, ptr addrspace(5) %"4", align 8
  %"52" = inttoptr i64 %"14" to ptr
  %"51" = load i32, ptr %"52", align 4
  store i32 %"51", ptr addrspace(5) %"6", align 4
  %"16" = load i64, ptr addrspace(5) %"4", align 8
  %"53" = inttoptr i64 %"16" to ptr
  %"88" = getelementptr inbounds i8, ptr %"53", i64 4
  %"54" = load i32, ptr %"88", align 4
  store i32 %"54", ptr addrspace(5) %"7", align 4
  %"18" = load i64, ptr addrspace(5) %"4", align 8
  %"55" = inttoptr i64 %"18" to ptr
  %"90" = getelementptr inbounds i8, ptr %"55", i64 8
  %"56" = load i32, ptr %"90", align 4
  store i32 %"56", ptr addrspace(5) %"8", align 4
  %"20" = load i64, ptr addrspace(5) %"4", align 8
  %"57" = inttoptr i64 %"20" to ptr
  %"92" = getelementptr inbounds i8, ptr %"57", i64 12
  %"58" = load i32, ptr %"92", align 4
  store i32 %"58", ptr addrspace(5) %"9", align 4
  %"22" = load i32, ptr addrspace(5) %"6", align 4
  %"59" = call float @__zluda_ptx_impl__cvt_rn_f32_s32(i32 %"22")
  %"21" = bitcast float %"59" to i32
  store i32 %"21", ptr addrspace(5) %"6", align 4
  %"24" = load i32, ptr addrspace(5) %"7", align 4
  %"61" = call float @__zluda_ptx_impl__cvt_rz_f32_s32(i32 %"24")
  %"23" = bitcast float %"61" to i32
  store i32 %"23", ptr addrspace(5) %"7", align 4
  %"26" = load i32, ptr addrspace(5) %"8", align 4
  %"63" = call float @__zluda_ptx_impl__cvt_rm_f32_s32(i32 %"26")
  %"25" = bitcast float %"63" to i32
  store i32 %"25", ptr addrspace(5) %"8", align 4
  %"28" = load i32, ptr addrspace(5) %"9", align 4
  %"65" = call float @__zluda_ptx_impl__cvt_rp_f32_s32(i32 %"28")
  %"27" = bitcast float %"65" to i32
  store i32 %"27", ptr addrspace(5) %"9", align 4
  %"29" = load i64, ptr addrspace(5) %"5", align 8
  %"30" = load i32, ptr addrspace(5) %"6", align 4
  %"67" = inttoptr i64 %"29" to ptr addrspace(1)
  %"68" = bitcast i32 %"30" to float
  store float %"68", ptr addrspace(1) %"67", align 4
  %"31" = load i64, ptr addrspace(5) %"5", align 8
  %"32" = load i32, ptr addrspace(5) %"7", align 4
  %"69" = inttoptr i64 %"31" to ptr addrspace(1)
  %"94" = getelementptr inbounds i8, ptr addrspace(1) %"69", i64 4
  %"70" = bitcast i32 %"32" to float
  store float %"70", ptr addrspace(1) %"94", align 4
  %"33" = load i64, ptr addrspace(5) %"5", align 8
  %"34" = load i32, ptr addrspace(5) %"8", align 4
  %"71" = inttoptr i64 %"33" to ptr addrspace(1)
  %"96" = getelementptr inbounds i8, ptr addrspace(1) %"71", i64 8
  %"72" = bitcast i32 %"34" to float
  store float %"72", ptr addrspace(1) %"96", align 4
  %"35" = load i64, ptr addrspace(5) %"5", align 8
  %"36" = load i32, ptr addrspace(5) %"9", align 4
  %"73" = inttoptr i64 %"35" to ptr addrspace(1)
  %"98" = getelementptr inbounds i8, ptr addrspace(1) %"73", i64 12
  %"74" = bitcast i32 %"36" to float
  store float %"74", ptr addrspace(1) %"98", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
