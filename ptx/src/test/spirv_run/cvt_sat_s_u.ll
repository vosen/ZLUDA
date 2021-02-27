target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @cvt_sat_s_u(ptr addrspace(4) byref(i64) %"27", ptr addrspace(4) byref(i64) %"28") #0 {
"35":
  %"9" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"9", align 1
  %"10" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"10", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"8" = alloca i32, align 4, addrspace(5)
  %"11" = load i64, ptr addrspace(4) %"27", align 8
  store i64 %"11", ptr addrspace(5) %"4", align 8
  %"12" = load i64, ptr addrspace(4) %"28", align 8
  store i64 %"12", ptr addrspace(5) %"5", align 8
  %"14" = load i64, ptr addrspace(5) %"4", align 8
  %"29" = inttoptr i64 %"14" to ptr
  %"13" = load i32, ptr %"29", align 4
  store i32 %"13", ptr addrspace(5) %"6", align 4
  %"16" = load i32, ptr addrspace(5) %"6", align 4
  %0 = call i32 @llvm.smax.i32(i32 %"16", i32 0)
  %1 = alloca i32, align 4, addrspace(5)
  store i32 %0, ptr addrspace(5) %1, align 4
  %"15" = load i32, ptr addrspace(5) %1, align 4
  store i32 %"15", ptr addrspace(5) %"7", align 4
  %"18" = load i32, ptr addrspace(5) %"7", align 4
  %2 = alloca i32, align 4, addrspace(5)
  store i32 %"18", ptr addrspace(5) %2, align 4
  %"30" = load i32, ptr addrspace(5) %2, align 4
  store i32 %"30", ptr addrspace(5) %"7", align 4
  %"20" = load i32, ptr addrspace(5) %"6", align 4
  %3 = alloca i32, align 4, addrspace(5)
  store i32 %"20", ptr addrspace(5) %3, align 4
  %"31" = load i32, ptr addrspace(5) %3, align 4
  store i32 %"31", ptr addrspace(5) %"8", align 4
  %"21" = load i64, ptr addrspace(5) %"5", align 8
  %"22" = load i32, ptr addrspace(5) %"7", align 4
  %"32" = inttoptr i64 %"21" to ptr
  store i32 %"22", ptr %"32", align 4
  %"23" = load i64, ptr addrspace(5) %"5", align 8
  %"24" = load i32, ptr addrspace(5) %"8", align 4
  %"34" = inttoptr i64 %"23" to ptr
  %"37" = getelementptr inbounds i8, ptr %"34", i64 4
  store i32 %"24", ptr %"37", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare i32 @llvm.smax.i32(i32, i32) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
