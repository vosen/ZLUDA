target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @_Z13callback_onlyIdEvPvS0_10callback_tx(ptr addrspace(4) byref(i64) %"43", ptr addrspace(4) byref(i64) %"44", ptr addrspace(4) byref(i64) %"45", ptr addrspace(4) byref(i64) %"46") #0 {
"59":
  %"22" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"22", align 1
  %"23" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"23", align 1
  %"7" = alloca i1, align 1, addrspace(5)
  %"8" = alloca double, align 8, addrspace(5)
  %"9" = alloca double, align 8, addrspace(5)
  %"10" = alloca i64, align 8, addrspace(5)
  %"11" = alloca i64, align 8, addrspace(5)
  %"12" = alloca i64, align 8, addrspace(5)
  %"13" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  %"49" = alloca [4 x i32], align 16, addrspace(5)
  %"51" = load i64, ptr addrspace(4) %"43", align 8
  store i64 %"51", ptr addrspace(5) %"10", align 8
  %"52" = load i64, ptr addrspace(4) %"44", align 8
  store i64 %"52", ptr addrspace(5) %"11", align 8
  %"53" = load i64, ptr addrspace(4) %"45", align 8
  store i64 %"53", ptr addrspace(5) %"12", align 8
  %"54" = load i64, ptr addrspace(4) %"46", align 8
  store i64 %"54", ptr addrspace(5) %"13", align 8
  %"29" = load i64, ptr addrspace(5) %"12", align 8
  %"30" = load i64, ptr addrspace(5) %"13", align 8
  %"28" = icmp sge i64 %"29", %"30"
  store i1 %"28", ptr addrspace(5) %"7", align 1
  %"31" = load i1, ptr addrspace(5) %"7", align 1
  br i1 %"31", label %"6", label %"18"

"18":                                             ; preds = %"59"
  %"32" = load i64, ptr addrspace(5) %"11", align 8
  %"61" = getelementptr inbounds i8, ptr addrspace(5) %"47", i64 0
  store i64 %"32", ptr addrspace(5) %"61", align 8
  %"33" = load i64, ptr addrspace(5) %"11", align 8
  %0 = inttoptr i64 %"33" to ptr
  %"21" = call [4 x i32] %0()
  store [4 x i32] %"21", ptr addrspace(5) %"49", align 4
  %"63" = getelementptr inbounds i8, ptr addrspace(5) %"49", i64 0
  %"19" = load <2 x double>, ptr addrspace(5) %"63", align 16
  %"34" = extractelement <2 x double> %"19", i32 0
  %"35" = extractelement <2 x double> %"19", i32 1
  store double %"34", ptr addrspace(5) %"8", align 8
  store double %"35", ptr addrspace(5) %"9", align 8
  %"36" = load double, ptr addrspace(5) %"8", align 8
  %"37" = load double, ptr addrspace(5) %"9", align 8
  %1 = insertelement <2 x double> undef, double %"36", i32 0
  %"20" = insertelement <2 x double> %1, double %"37", i32 1
  %"38" = load i64, ptr addrspace(5) %"10", align 8
  %"58" = inttoptr i64 %"38" to ptr addrspace(1)
  store <2 x double> %"20", ptr addrspace(1) %"58", align 16
  br label %"6"

"6":                                              ; preds = %"18", %"59"
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
