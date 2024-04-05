target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

declare i32 @__zluda_ptx_impl__cvt_rp_s32_f32(float) #0

define protected amdgpu_kernel void @cvt_s32_f32(ptr addrspace(4) byref(i64) %"27", ptr addrspace(4) byref(i64) %"28") #1 {
"41":
  %"8" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"8", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"9" = load i64, ptr addrspace(4) %"27", align 8
  store i64 %"9", ptr addrspace(5) %"4", align 8
  %"10" = load i64, ptr addrspace(4) %"28", align 8
  store i64 %"10", ptr addrspace(5) %"5", align 8
  %"12" = load i64, ptr addrspace(5) %"4", align 8
  %"30" = inttoptr i64 %"12" to ptr
  %"29" = load float, ptr %"30", align 4
  %"11" = bitcast float %"29" to i32
  store i32 %"11", ptr addrspace(5) %"6", align 4
  %"14" = load i64, ptr addrspace(5) %"4", align 8
  %"31" = inttoptr i64 %"14" to ptr
  %"46" = getelementptr inbounds i8, ptr %"31", i64 4
  %"32" = load float, ptr %"46", align 4
  %"13" = bitcast float %"32" to i32
  store i32 %"13", ptr addrspace(5) %"7", align 4
  %"16" = load i32, ptr addrspace(5) %"6", align 4
  %"34" = bitcast i32 %"16" to float
  %"33" = call i32 @__zluda_ptx_impl__cvt_rp_s32_f32(float %"34")
  store i32 %"33", ptr addrspace(5) %"6", align 4
  %"18" = load i32, ptr addrspace(5) %"7", align 4
  %"36" = bitcast i32 %"18" to float
  %"35" = call i32 @__zluda_ptx_impl__cvt_rp_s32_f32(float %"36")
  store i32 %"35", ptr addrspace(5) %"7", align 4
  %"19" = load i64, ptr addrspace(5) %"5", align 8
  %"20" = load i32, ptr addrspace(5) %"6", align 4
  %"37" = inttoptr i64 %"19" to ptr addrspace(1)
  store i32 %"20", ptr addrspace(1) %"37", align 4
  %"21" = load i64, ptr addrspace(5) %"5", align 8
  %"22" = load i32, ptr addrspace(5) %"7", align 4
  %"39" = inttoptr i64 %"21" to ptr addrspace(1)
  %"48" = getelementptr inbounds i8, ptr addrspace(1) %"39", i64 4
  store i32 %"22", ptr addrspace(1) %"48", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
