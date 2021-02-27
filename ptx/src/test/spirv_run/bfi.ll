target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

declare i32 @__zluda_ptx_impl__bfi_b32(i32, i32, i32, i32) #0

define protected amdgpu_kernel void @bfi(ptr addrspace(4) byref(i64) %"35", ptr addrspace(4) byref(i64) %"36") #1 {
"45":
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
  %"12" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"12", ptr addrspace(5) %"4", align 8
  %"13" = load i64, ptr addrspace(4) %"36", align 8
  store i64 %"13", ptr addrspace(5) %"5", align 8
  %"15" = load i64, ptr addrspace(5) %"4", align 8
  %"37" = inttoptr i64 %"15" to ptr
  %"14" = load i32, ptr %"37", align 4
  store i32 %"14", ptr addrspace(5) %"6", align 4
  %"17" = load i64, ptr addrspace(5) %"4", align 8
  %"38" = inttoptr i64 %"17" to ptr
  %"53" = getelementptr inbounds i8, ptr %"38", i64 4
  %"16" = load i32, ptr %"53", align 4
  store i32 %"16", ptr addrspace(5) %"7", align 4
  %"19" = load i64, ptr addrspace(5) %"4", align 8
  %"39" = inttoptr i64 %"19" to ptr
  %"55" = getelementptr inbounds i8, ptr %"39", i64 8
  %"18" = load i32, ptr %"55", align 4
  store i32 %"18", ptr addrspace(5) %"8", align 4
  %"21" = load i64, ptr addrspace(5) %"4", align 8
  %"40" = inttoptr i64 %"21" to ptr
  %"57" = getelementptr inbounds i8, ptr %"40", i64 12
  %"20" = load i32, ptr %"57", align 4
  store i32 %"20", ptr addrspace(5) %"9", align 4
  %"23" = load i32, ptr addrspace(5) %"6", align 4
  %"24" = load i32, ptr addrspace(5) %"7", align 4
  %"25" = load i32, ptr addrspace(5) %"8", align 4
  %"26" = load i32, ptr addrspace(5) %"9", align 4
  %"41" = call i32 @__zluda_ptx_impl__bfi_b32(i32 %"23", i32 %"24", i32 %"25", i32 %"26")
  store i32 %"41", ptr addrspace(5) %"6", align 4
  %"27" = load i64, ptr addrspace(5) %"5", align 8
  %"28" = load i32, ptr addrspace(5) %"6", align 4
  %"44" = inttoptr i64 %"27" to ptr
  store i32 %"28", ptr %"44", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
