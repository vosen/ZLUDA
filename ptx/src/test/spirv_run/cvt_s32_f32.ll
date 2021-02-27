target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

declare i32 @__zluda_ptx_impl__cvt_rp_s32_f32(float) #0

define protected amdgpu_kernel void @cvt_s32_f32(ptr addrspace(4) byref(i64) %"28", ptr addrspace(4) byref(i64) %"29") #1 {
"42":
  %"8" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"8", align 1
  %"9" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"9", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"10" = load i64, ptr addrspace(4) %"28", align 8
  store i64 %"10", ptr addrspace(5) %"4", align 8
  %"11" = load i64, ptr addrspace(4) %"29", align 8
  store i64 %"11", ptr addrspace(5) %"5", align 8
  %"13" = load i64, ptr addrspace(5) %"4", align 8
  %"31" = inttoptr i64 %"13" to ptr
  %"30" = load float, ptr %"31", align 4
  %"12" = bitcast float %"30" to i32
  store i32 %"12", ptr addrspace(5) %"6", align 4
  %"15" = load i64, ptr addrspace(5) %"4", align 8
  %"32" = inttoptr i64 %"15" to ptr
  %"47" = getelementptr inbounds i8, ptr %"32", i64 4
  %"33" = load float, ptr %"47", align 4
  %"14" = bitcast float %"33" to i32
  store i32 %"14", ptr addrspace(5) %"7", align 4
  %"17" = load i32, ptr addrspace(5) %"6", align 4
  %"35" = bitcast i32 %"17" to float
  %"34" = call i32 @__zluda_ptx_impl__cvt_rp_s32_f32(float %"35")
  store i32 %"34", ptr addrspace(5) %"6", align 4
  %"19" = load i32, ptr addrspace(5) %"7", align 4
  %"37" = bitcast i32 %"19" to float
  %"36" = call i32 @__zluda_ptx_impl__cvt_rp_s32_f32(float %"37")
  store i32 %"36", ptr addrspace(5) %"7", align 4
  %"20" = load i64, ptr addrspace(5) %"5", align 8
  %"21" = load i32, ptr addrspace(5) %"6", align 4
  %"38" = inttoptr i64 %"20" to ptr addrspace(1)
  store i32 %"21", ptr addrspace(1) %"38", align 4
  %"22" = load i64, ptr addrspace(5) %"5", align 8
  %"23" = load i32, ptr addrspace(5) %"7", align 4
  %"40" = inttoptr i64 %"22" to ptr addrspace(1)
  %"49" = getelementptr inbounds i8, ptr addrspace(1) %"40", i64 4
  store i32 %"23", ptr addrspace(1) %"49", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
