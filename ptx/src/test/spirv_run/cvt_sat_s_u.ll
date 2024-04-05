target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @cvt_sat_s_u(ptr addrspace(4) byref(i64) %"26", ptr addrspace(4) byref(i64) %"27") #0 {
"34":
  %"9" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"9", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"8" = alloca i32, align 4, addrspace(5)
  %"10" = load i64, ptr addrspace(4) %"26", align 8
  store i64 %"10", ptr addrspace(5) %"4", align 8
  %"11" = load i64, ptr addrspace(4) %"27", align 8
  store i64 %"11", ptr addrspace(5) %"5", align 8
  %"13" = load i64, ptr addrspace(5) %"4", align 8
  %"28" = inttoptr i64 %"13" to ptr
  %"12" = load i32, ptr %"28", align 4
  store i32 %"12", ptr addrspace(5) %"6", align 4
  %"15" = load i32, ptr addrspace(5) %"6", align 4
  %0 = call i32 @llvm.smax.i32(i32 %"15", i32 0)
  %1 = alloca i32, align 4, addrspace(5)
  store i32 %0, ptr addrspace(5) %1, align 4
  %"14" = load i32, ptr addrspace(5) %1, align 4
  store i32 %"14", ptr addrspace(5) %"7", align 4
  %"17" = load i32, ptr addrspace(5) %"7", align 4
  %2 = alloca i32, align 4, addrspace(5)
  store i32 %"17", ptr addrspace(5) %2, align 4
  %"29" = load i32, ptr addrspace(5) %2, align 4
  store i32 %"29", ptr addrspace(5) %"7", align 4
  %"19" = load i32, ptr addrspace(5) %"6", align 4
  %3 = alloca i32, align 4, addrspace(5)
  store i32 %"19", ptr addrspace(5) %3, align 4
  %"30" = load i32, ptr addrspace(5) %3, align 4
  store i32 %"30", ptr addrspace(5) %"8", align 4
  %"20" = load i64, ptr addrspace(5) %"5", align 8
  %"21" = load i32, ptr addrspace(5) %"7", align 4
  %"31" = inttoptr i64 %"20" to ptr
  store i32 %"21", ptr %"31", align 4
  %"22" = load i64, ptr addrspace(5) %"5", align 8
  %"23" = load i32, ptr addrspace(5) %"8", align 4
  %"33" = inttoptr i64 %"22" to ptr
  %"36" = getelementptr inbounds i8, ptr %"33", i64 4
  store i32 %"23", ptr %"36", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare i32 @llvm.smax.i32(i32, i32) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
