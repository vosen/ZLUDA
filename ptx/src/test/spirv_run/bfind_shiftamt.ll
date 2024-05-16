target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @bfind_shiftamt(ptr addrspace(4) byref(i64) %"41", ptr addrspace(4) byref(i64) %"42") #0 {
  %"12" = alloca i1, align 1, addrspace(5)
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"8" = alloca i32, align 4, addrspace(5)
  %"9" = alloca i32, align 4, addrspace(5)
  %"10" = alloca i32, align 4, addrspace(5)
  %"11" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  store i1 false, ptr addrspace(5) %"12", align 1
  %"13" = load i64, ptr addrspace(4) %"41", align 8
  store i64 %"13", ptr addrspace(5) %"4", align 8
  %"14" = load i64, ptr addrspace(4) %"42", align 8
  store i64 %"14", ptr addrspace(5) %"5", align 8
  %"16" = load i64, ptr addrspace(5) %"4", align 8
  %"43" = inttoptr i64 %"16" to ptr
  %"15" = load i32, ptr %"43", align 4
  store i32 %"15", ptr addrspace(5) %"6", align 4
  %"18" = load i64, ptr addrspace(5) %"4", align 8
  %"44" = inttoptr i64 %"18" to ptr
  %"53" = getelementptr inbounds i8, ptr %"44", i64 4
  %"17" = load i32, ptr %"53", align 4
  store i32 %"17", ptr addrspace(5) %"7", align 4
  %"20" = load i64, ptr addrspace(5) %"4", align 8
  %"45" = inttoptr i64 %"20" to ptr
  %"55" = getelementptr inbounds i8, ptr %"45", i64 8
  %"19" = load i32, ptr %"55", align 4
  store i32 %"19", ptr addrspace(5) %"8", align 4
  %"22" = load i32, ptr addrspace(5) %"6", align 4
  %2 = icmp eq i32 %"22", 0
  %3 = call i32 @llvm.ctlz.i32(i32 %"22", i1 true)
  %"46" = select i1 %2, i32 -1, i32 %3
  store i32 %"46", ptr addrspace(5) %"9", align 4
  %"24" = load i32, ptr addrspace(5) %"7", align 4
  %4 = icmp eq i32 %"24", 0
  %5 = call i32 @llvm.ctlz.i32(i32 %"24", i1 true)
  %"47" = select i1 %4, i32 -1, i32 %5
  store i32 %"47", ptr addrspace(5) %"10", align 4
  %"26" = load i32, ptr addrspace(5) %"8", align 4
  %6 = icmp eq i32 %"26", 0
  %7 = call i32 @llvm.ctlz.i32(i32 %"26", i1 true)
  %"48" = select i1 %6, i32 -1, i32 %7
  store i32 %"48", ptr addrspace(5) %"11", align 4
  %"27" = load i64, ptr addrspace(5) %"5", align 8
  %"28" = load i32, ptr addrspace(5) %"9", align 4
  %"49" = inttoptr i64 %"27" to ptr
  store i32 %"28", ptr %"49", align 4
  %"29" = load i64, ptr addrspace(5) %"5", align 8
  %"30" = load i32, ptr addrspace(5) %"10", align 4
  %"50" = inttoptr i64 %"29" to ptr
  %"57" = getelementptr inbounds i8, ptr %"50", i64 4
  store i32 %"30", ptr %"57", align 4
  %"31" = load i64, ptr addrspace(5) %"5", align 8
  %"32" = load i32, ptr addrspace(5) %"11", align 4
  %"51" = inttoptr i64 %"31" to ptr
  %"59" = getelementptr inbounds i8, ptr %"51", i64 8
  store i32 %"32", ptr %"59", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare i32 @llvm.ctlz.i32(i32, i1 immarg) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
