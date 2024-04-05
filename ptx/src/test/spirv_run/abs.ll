target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @abs(ptr addrspace(4) byref(i64) %"27", ptr addrspace(4) byref(i64) %"28") #0 {
"37":
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
  %"29" = load i32, ptr %"30", align 4
  store i32 %"29", ptr addrspace(5) %"6", align 4
  %"14" = load i64, ptr addrspace(5) %"4", align 8
  %"31" = inttoptr i64 %"14" to ptr
  %"39" = getelementptr inbounds i8, ptr %"31", i64 4
  %"32" = load i32, ptr %"39", align 4
  store i32 %"32", ptr addrspace(5) %"7", align 4
  %"16" = load i32, ptr addrspace(5) %"6", align 4
  %"15" = call i32 @llvm.abs.i32(i32 %"16", i1 false)
  store i32 %"15", ptr addrspace(5) %"6", align 4
  %"18" = load i32, ptr addrspace(5) %"7", align 4
  %"17" = call i32 @llvm.abs.i32(i32 %"18", i1 false)
  store i32 %"17", ptr addrspace(5) %"7", align 4
  %"19" = load i64, ptr addrspace(5) %"5", align 8
  %"20" = load i32, ptr addrspace(5) %"6", align 4
  %"33" = inttoptr i64 %"19" to ptr
  store i32 %"20", ptr %"33", align 4
  %"21" = load i64, ptr addrspace(5) %"5", align 8
  %"22" = load i32, ptr addrspace(5) %"7", align 4
  %"35" = inttoptr i64 %"21" to ptr
  %"41" = getelementptr inbounds i8, ptr %"35", i64 4
  store i32 %"22", ptr %"41", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare i32 @llvm.abs.i32(i32, i1 immarg) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
