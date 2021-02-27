target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

declare void @__zluda_ptx_impl____assertfail(i64, i64, i32, i64, i64) #0

define protected amdgpu_kernel void @assertfail(ptr addrspace(4) byref(i64) %"63", ptr addrspace(4) byref(i64) %"64") #1 {
"82":
  %"35" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"35", align 1
  %"36" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"36", align 1
  %"15" = alloca i64, align 8, addrspace(5)
  %"16" = alloca i64, align 8, addrspace(5)
  %"17" = alloca i64, align 8, addrspace(5)
  %"18" = alloca i64, align 8, addrspace(5)
  %"19" = alloca i32, align 4, addrspace(5)
  %"65" = alloca i64, align 8, addrspace(5)
  %"67" = alloca i64, align 8, addrspace(5)
  %"69" = alloca i32, align 4, addrspace(5)
  %"71" = alloca i64, align 8, addrspace(5)
  %"73" = alloca i64, align 8, addrspace(5)
  %"37" = load i64, ptr addrspace(4) %"63", align 8
  store i64 %"37", ptr addrspace(5) %"15", align 8
  %"38" = load i64, ptr addrspace(4) %"64", align 8
  store i64 %"38", ptr addrspace(5) %"16", align 8
  %0 = alloca i32, align 4, addrspace(5)
  store i32 0, ptr addrspace(5) %0, align 4
  %"75" = load i32, ptr addrspace(5) %0, align 4
  store i32 %"75", ptr addrspace(5) %"19", align 4
  %"40" = load i64, ptr addrspace(5) %"15", align 8
  %"84" = getelementptr inbounds i8, ptr addrspace(5) %"65", i64 0
  store i64 %"40", ptr addrspace(5) %"84", align 8
  %"41" = load i64, ptr addrspace(5) %"15", align 8
  %"86" = getelementptr inbounds i8, ptr addrspace(5) %"67", i64 0
  store i64 %"41", ptr addrspace(5) %"86", align 8
  %"42" = load i32, ptr addrspace(5) %"19", align 4
  %"88" = getelementptr inbounds i8, ptr addrspace(5) %"69", i64 0
  store i32 %"42", ptr addrspace(5) %"88", align 4
  %"43" = load i64, ptr addrspace(5) %"15", align 8
  %"90" = getelementptr inbounds i8, ptr addrspace(5) %"71", i64 0
  store i64 %"43", ptr addrspace(5) %"90", align 8
  %"44" = load i64, ptr addrspace(5) %"15", align 8
  %"92" = getelementptr inbounds i8, ptr addrspace(5) %"73", i64 0
  store i64 %"44", ptr addrspace(5) %"92", align 8
  %"30" = load i64, ptr addrspace(5) %"65", align 8
  %"31" = load i64, ptr addrspace(5) %"67", align 8
  %"32" = load i32, ptr addrspace(5) %"69", align 4
  %"33" = load i64, ptr addrspace(5) %"71", align 8
  %"34" = load i64, ptr addrspace(5) %"73", align 8
  call void @__zluda_ptx_impl____assertfail(i64 %"30", i64 %"31", i32 %"32", i64 %"33", i64 %"34")
  %"46" = load i64, ptr addrspace(5) %"15", align 8
  %"80" = inttoptr i64 %"46" to ptr
  %"45" = load i64, ptr %"80", align 8
  store i64 %"45", ptr addrspace(5) %"17", align 8
  %"48" = load i64, ptr addrspace(5) %"17", align 8
  %"47" = add i64 %"48", 1
  store i64 %"47", ptr addrspace(5) %"18", align 8
  %"49" = load i64, ptr addrspace(5) %"16", align 8
  %"50" = load i64, ptr addrspace(5) %"18", align 8
  %"81" = inttoptr i64 %"49" to ptr
  store i64 %"50", ptr %"81", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
