target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @isspacep(ptr addrspace(4) byref(i64) %"32", ptr addrspace(4) byref(i64) %"33") #0 {
"36":
  %"10" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"10", align 1
  %"11" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"11", align 1
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i1, align 1, addrspace(5)
  %"7" = alloca i1, align 1, addrspace(5)
  %"8" = alloca i32, align 4, addrspace(5)
  %"9" = alloca i32, align 4, addrspace(5)
  %"12" = load i64, ptr addrspace(4) %"32", align 8
  store i64 %"12", ptr addrspace(5) %"4", align 8
  %"13" = load i64, ptr addrspace(4) %"33", align 8
  store i64 %"13", ptr addrspace(5) %"5", align 8
  %"15" = load i64, ptr addrspace(5) %"4", align 8
  %0 = inttoptr i64 %"15" to ptr
  %1 = call i1 @llvm.amdgcn.is.private(ptr %0)
  %2 = inttoptr i64 %"15" to ptr
  %3 = call i1 @llvm.amdgcn.is.shared(ptr %2)
  %4 = or i1 %1, %3
  %"14" = sub i1 true, %4
  store i1 %"14", ptr addrspace(5) %"6", align 1
  %"17" = load i1, ptr addrspace(5) %"6", align 1
  %"16" = select i1 %"17", i32 1, i32 0
  store i32 %"16", ptr addrspace(5) %"8", align 4
  %"19" = load i64, ptr addrspace(5) %"4", align 8
  %5 = inttoptr i64 %"19" to ptr
  %"18" = call i1 @llvm.amdgcn.is.shared(ptr %5)
  store i1 %"18", ptr addrspace(5) %"7", align 1
  %"21" = load i1, ptr addrspace(5) %"7", align 1
  %"20" = select i1 %"21", i32 1, i32 0
  store i32 %"20", ptr addrspace(5) %"9", align 4
  %"22" = load i64, ptr addrspace(5) %"5", align 8
  %"23" = load i32, ptr addrspace(5) %"8", align 4
  %"34" = inttoptr i64 %"22" to ptr
  store i32 %"23", ptr %"34", align 4
  %"24" = load i64, ptr addrspace(5) %"5", align 8
  %"25" = load i32, ptr addrspace(5) %"9", align 4
  %"35" = inttoptr i64 %"24" to ptr
  %"38" = getelementptr inbounds i8, ptr %"35", i64 4
  store i32 %"25", ptr %"38", align 4
  ret void
}

; Function Attrs: nounwind readnone speculatable willreturn
declare i1 @llvm.amdgcn.is.private(ptr nocapture) #1

; Function Attrs: nounwind readnone speculatable willreturn
declare i1 @llvm.amdgcn.is.shared(ptr nocapture) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nounwind readnone speculatable willreturn }
