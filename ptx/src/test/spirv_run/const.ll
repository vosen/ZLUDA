target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

@constparams = protected addrspace(4) externally_initialized global [4 x i16] [i16 10, i16 20, i16 30, i16 40], align 8

define protected amdgpu_kernel void @const(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #0 {
"53":
  %"11" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"11", align 1
  %"12" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"12", align 1
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i64, align 8, addrspace(5)
  %"7" = alloca i16, align 2, addrspace(5)
  %"8" = alloca i16, align 2, addrspace(5)
  %"9" = alloca i16, align 2, addrspace(5)
  %"10" = alloca i16, align 2, addrspace(5)
  %"13" = load i64, ptr addrspace(4) %"39", align 8
  store i64 %"13", ptr addrspace(5) %"5", align 8
  %"14" = load i64, ptr addrspace(4) %"40", align 8
  store i64 %"14", ptr addrspace(5) %"6", align 8
  %"15" = load i16, ptr addrspace(4) @constparams, align 2
  store i16 %"15", ptr addrspace(5) %"7", align 2
  %"16" = load i16, ptr addrspace(4) getelementptr inbounds (i8, ptr addrspace(4) @constparams, i64 2), align 2
  store i16 %"16", ptr addrspace(5) %"8", align 2
  %"17" = load i16, ptr addrspace(4) getelementptr inbounds (i8, ptr addrspace(4) @constparams, i64 4), align 2
  store i16 %"17", ptr addrspace(5) %"9", align 2
  %"18" = load i16, ptr addrspace(4) getelementptr inbounds (i8, ptr addrspace(4) @constparams, i64 6), align 2
  store i16 %"18", ptr addrspace(5) %"10", align 2
  %"19" = load i64, ptr addrspace(5) %"6", align 8
  %"20" = load i16, ptr addrspace(5) %"7", align 2
  %"45" = inttoptr i64 %"19" to ptr
  store i16 %"20", ptr %"45", align 2
  %"21" = load i64, ptr addrspace(5) %"6", align 8
  %"22" = load i16, ptr addrspace(5) %"8", align 2
  %"47" = inttoptr i64 %"21" to ptr
  %"61" = getelementptr inbounds i8, ptr %"47", i64 2
  store i16 %"22", ptr %"61", align 2
  %"23" = load i64, ptr addrspace(5) %"6", align 8
  %"24" = load i16, ptr addrspace(5) %"9", align 2
  %"49" = inttoptr i64 %"23" to ptr
  %"63" = getelementptr inbounds i8, ptr %"49", i64 4
  store i16 %"24", ptr %"63", align 2
  %"25" = load i64, ptr addrspace(5) %"6", align 8
  %"26" = load i16, ptr addrspace(5) %"10", align 2
  %"51" = inttoptr i64 %"25" to ptr
  %"65" = getelementptr inbounds i8, ptr %"51", i64 6
  store i16 %"26", ptr %"65", align 2
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
