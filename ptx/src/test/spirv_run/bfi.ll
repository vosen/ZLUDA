target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

declare i32 @__zluda_ptx_impl__bfi_b32(i32, i32, i32, i32) #0

define protected amdgpu_kernel void @bfi(ptr addrspace(4) byref(i64) %"34", ptr addrspace(4) byref(i64) %"35") #1 {
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
  %"11" = load i64, ptr addrspace(4) %"34", align 8
  store i64 %"11", ptr addrspace(5) %"4", align 8
  %"12" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"12", ptr addrspace(5) %"5", align 8
  %"14" = load i64, ptr addrspace(5) %"4", align 8
  %"36" = inttoptr i64 %"14" to ptr
  %"13" = load i32, ptr %"36", align 4
  store i32 %"13", ptr addrspace(5) %"6", align 4
  %"16" = load i64, ptr addrspace(5) %"4", align 8
  %"37" = inttoptr i64 %"16" to ptr
  %"51" = getelementptr inbounds i8, ptr %"37", i64 4
  %"15" = load i32, ptr %"51", align 4
  store i32 %"15", ptr addrspace(5) %"7", align 4
  %"18" = load i64, ptr addrspace(5) %"4", align 8
  %"38" = inttoptr i64 %"18" to ptr
  %"53" = getelementptr inbounds i8, ptr %"38", i64 8
  %"17" = load i32, ptr %"53", align 4
  store i32 %"17", ptr addrspace(5) %"8", align 4
  %"20" = load i64, ptr addrspace(5) %"4", align 8
  %"39" = inttoptr i64 %"20" to ptr
  %"55" = getelementptr inbounds i8, ptr %"39", i64 12
  %"19" = load i32, ptr %"55", align 4
  store i32 %"19", ptr addrspace(5) %"9", align 4
  %"22" = load i32, ptr addrspace(5) %"6", align 4
  %"23" = load i32, ptr addrspace(5) %"7", align 4
  %"24" = load i32, ptr addrspace(5) %"8", align 4
  %"25" = load i32, ptr addrspace(5) %"9", align 4
  %"40" = call i32 @__zluda_ptx_impl__bfi_b32(i32 %"22", i32 %"23", i32 %"24", i32 %"25")
  store i32 %"40", ptr addrspace(5) %"6", align 4
  %"26" = load i64, ptr addrspace(5) %"5", align 8
  %"27" = load i32, ptr addrspace(5) %"6", align 4
  %"43" = inttoptr i64 %"26" to ptr
  store i32 %"27", ptr %"43", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
