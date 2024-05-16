target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

declare void @__zluda_ptx_impl____assertfail(i64, i64, i32, i64, i64) #0

define protected amdgpu_kernel void @assertfail(ptr addrspace(4) byref(i64) %"62", ptr addrspace(4) byref(i64) %"63") #1 {
  %"35" = alloca i1, align 1, addrspace(5)
  %"15" = alloca i64, align 8, addrspace(5)
  %"16" = alloca i64, align 8, addrspace(5)
  %"17" = alloca i64, align 8, addrspace(5)
  %"18" = alloca i64, align 8, addrspace(5)
  %"19" = alloca i32, align 4, addrspace(5)
  %1 = alloca i32, align 4, addrspace(5)
  %"64" = alloca i64, align 8, addrspace(5)
  %"66" = alloca i64, align 8, addrspace(5)
  %"68" = alloca i32, align 4, addrspace(5)
  %"70" = alloca i64, align 8, addrspace(5)
  %"72" = alloca i64, align 8, addrspace(5)
  br label %2

2:                                                ; preds = %0
  store i1 false, ptr addrspace(5) %"35", align 1
  %"36" = load i64, ptr addrspace(4) %"62", align 8
  store i64 %"36", ptr addrspace(5) %"15", align 8
  %"37" = load i64, ptr addrspace(4) %"63", align 8
  store i64 %"37", ptr addrspace(5) %"16", align 8
  store i32 0, ptr addrspace(5) %1, align 4
  %"74" = load i32, ptr addrspace(5) %1, align 4
  store i32 %"74", ptr addrspace(5) %"19", align 4
  %"39" = load i64, ptr addrspace(5) %"15", align 8
  %"82" = getelementptr inbounds i8, ptr addrspace(5) %"64", i64 0
  store i64 %"39", ptr addrspace(5) %"82", align 8
  %"40" = load i64, ptr addrspace(5) %"15", align 8
  %"84" = getelementptr inbounds i8, ptr addrspace(5) %"66", i64 0
  store i64 %"40", ptr addrspace(5) %"84", align 8
  %"41" = load i32, ptr addrspace(5) %"19", align 4
  %"86" = getelementptr inbounds i8, ptr addrspace(5) %"68", i64 0
  store i32 %"41", ptr addrspace(5) %"86", align 4
  %"42" = load i64, ptr addrspace(5) %"15", align 8
  %"88" = getelementptr inbounds i8, ptr addrspace(5) %"70", i64 0
  store i64 %"42", ptr addrspace(5) %"88", align 8
  %"43" = load i64, ptr addrspace(5) %"15", align 8
  %"90" = getelementptr inbounds i8, ptr addrspace(5) %"72", i64 0
  store i64 %"43", ptr addrspace(5) %"90", align 8
  %"30" = load i64, ptr addrspace(5) %"64", align 8
  %"31" = load i64, ptr addrspace(5) %"66", align 8
  %"32" = load i32, ptr addrspace(5) %"68", align 4
  %"33" = load i64, ptr addrspace(5) %"70", align 8
  %"34" = load i64, ptr addrspace(5) %"72", align 8
  call void @__zluda_ptx_impl____assertfail(i64 %"30", i64 %"31", i32 %"32", i64 %"33", i64 %"34")
  %"45" = load i64, ptr addrspace(5) %"15", align 8
  %"79" = inttoptr i64 %"45" to ptr
  %"44" = load i64, ptr %"79", align 8
  store i64 %"44", ptr addrspace(5) %"17", align 8
  %"47" = load i64, ptr addrspace(5) %"17", align 8
  %"46" = add i64 %"47", 1
  store i64 %"46", ptr addrspace(5) %"18", align 8
  %"48" = load i64, ptr addrspace(5) %"16", align 8
  %"49" = load i64, ptr addrspace(5) %"18", align 8
  %"80" = inttoptr i64 %"48" to ptr
  store i64 %"49", ptr %"80", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
