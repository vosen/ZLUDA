target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @bfind(ptr addrspace(4) byref(i64) %"42", ptr addrspace(4) byref(i64) %"43") #0 {
"53":
  %"12" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"12", align 1
  %"13" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"13", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"8" = alloca i32, align 4, addrspace(5)
  %"9" = alloca i32, align 4, addrspace(5)
  %"10" = alloca i32, align 4, addrspace(5)
  %"11" = alloca i32, align 4, addrspace(5)
  %"14" = load i64, ptr addrspace(4) %"42", align 8
  store i64 %"14", ptr addrspace(5) %"4", align 8
  %"15" = load i64, ptr addrspace(4) %"43", align 8
  store i64 %"15", ptr addrspace(5) %"5", align 8
  %"17" = load i64, ptr addrspace(5) %"4", align 8
  %"44" = inttoptr i64 %"17" to ptr
  %"16" = load i32, ptr %"44", align 4
  store i32 %"16", ptr addrspace(5) %"6", align 4
  %"19" = load i64, ptr addrspace(5) %"4", align 8
  %"45" = inttoptr i64 %"19" to ptr
  %"55" = getelementptr inbounds i8, ptr %"45", i64 4
  %"18" = load i32, ptr %"55", align 4
  store i32 %"18", ptr addrspace(5) %"7", align 4
  %"21" = load i64, ptr addrspace(5) %"4", align 8
  %"46" = inttoptr i64 %"21" to ptr
  %"57" = getelementptr inbounds i8, ptr %"46", i64 8
  %"20" = load i32, ptr %"57", align 4
  store i32 %"20", ptr addrspace(5) %"8", align 4
  %"23" = load i32, ptr addrspace(5) %"6", align 4
  %0 = icmp eq i32 %"23", 0
  %1 = call i32 @llvm.ctlz.i32(i32 %"23", i1 true)
  %2 = sub i32 31, %1
  %"47" = select i1 %0, i32 -1, i32 %2
  store i32 %"47", ptr addrspace(5) %"9", align 4
  %"25" = load i32, ptr addrspace(5) %"7", align 4
  %3 = icmp eq i32 %"25", 0
  %4 = call i32 @llvm.ctlz.i32(i32 %"25", i1 true)
  %5 = sub i32 31, %4
  %"48" = select i1 %3, i32 -1, i32 %5
  store i32 %"48", ptr addrspace(5) %"10", align 4
  %"27" = load i32, ptr addrspace(5) %"8", align 4
  %6 = icmp eq i32 %"27", 0
  %7 = call i32 @llvm.ctlz.i32(i32 %"27", i1 true)
  %8 = sub i32 31, %7
  %"49" = select i1 %6, i32 -1, i32 %8
  store i32 %"49", ptr addrspace(5) %"11", align 4
  %"28" = load i64, ptr addrspace(5) %"5", align 8
  %"29" = load i32, ptr addrspace(5) %"9", align 4
  %"50" = inttoptr i64 %"28" to ptr
  store i32 %"29", ptr %"50", align 4
  %"30" = load i64, ptr addrspace(5) %"5", align 8
  %"31" = load i32, ptr addrspace(5) %"10", align 4
  %"51" = inttoptr i64 %"30" to ptr
  %"59" = getelementptr inbounds i8, ptr %"51", i64 4
  store i32 %"31", ptr %"59", align 4
  %"32" = load i64, ptr addrspace(5) %"5", align 8
  %"33" = load i32, ptr addrspace(5) %"11", align 4
  %"52" = inttoptr i64 %"32" to ptr
  %"61" = getelementptr inbounds i8, ptr %"52", i64 8
  store i32 %"33", ptr %"61", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare i32 @llvm.ctlz.i32(i32, i1 immarg) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
