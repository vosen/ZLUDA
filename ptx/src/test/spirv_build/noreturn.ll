target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

; Function Attrs: noreturn
define private void @noreturn(i64 %"6") #0 {
"9":
  %"3" = alloca i64, align 8, addrspace(5)
  %"4" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"4", align 1
  %"5" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"5", align 1
  %"8" = alloca i64, align 8, addrspace(5)
  store i64 %"6", ptr addrspace(5) %"3", align 8
  %"7" = load i64, ptr addrspace(5) %"3", align 8
  store i64 %"7", ptr addrspace(5) %"8", align 8
  ret void
}

attributes #0 = { noreturn "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
