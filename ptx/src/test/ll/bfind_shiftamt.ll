define amdgpu_kernel void @bfind_shiftamt(ptr addrspace(4) byref(i64) %"49", ptr addrspace(4) byref(i64) %"50") #0 {
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i32, align 4, addrspace(5)
  %"54" = alloca i32, align 4, addrspace(5)
  %"55" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"48"

"48":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"49", align 8
  store i64 %2, ptr addrspace(5) %"51", align 8
  %3 = load i64, ptr addrspace(4) %"50", align 8
  store i64 %3, ptr addrspace(5) %"52", align 8
  %4 = load i64, ptr addrspace(5) %"51", align 8
  %"76" = inttoptr i64 %4 to ptr
  %5 = load i32, ptr %"76", align 4
  store i32 %5, ptr addrspace(5) %"53", align 4
  %6 = load i64, ptr addrspace(5) %"51", align 8
  %"77" = inttoptr i64 %6 to ptr
  %"41" = getelementptr inbounds i8, ptr %"77", i64 4
  %7 = load i32, ptr %"41", align 4
  store i32 %7, ptr addrspace(5) %"54", align 4
  %8 = load i64, ptr addrspace(5) %"51", align 8
  %"78" = inttoptr i64 %8 to ptr
  %"43" = getelementptr inbounds i8, ptr %"78", i64 8
  %9 = load i32, ptr %"43", align 4
  store i32 %9, ptr addrspace(5) %"55", align 4
  %10 = load i32, ptr addrspace(5) %"53", align 4
  %11 = icmp eq i32 %10, 0
  %12 = call i32 @llvm.ctlz.i32(i32 %10, i1 false)
  %"79" = select i1 %11, i32 -1, i32 %12
  store i32 %"79", ptr addrspace(5) %"53", align 4
  %13 = load i32, ptr addrspace(5) %"54", align 4
  %14 = icmp eq i32 %13, 0
  %15 = call i32 @llvm.ctlz.i32(i32 %13, i1 false)
  %"81" = select i1 %14, i32 -1, i32 %15
  store i32 %"81", ptr addrspace(5) %"54", align 4
  %16 = load i32, ptr addrspace(5) %"55", align 4
  %17 = icmp eq i32 %16, 0
  %18 = call i32 @llvm.ctlz.i32(i32 %16, i1 false)
  %"83" = select i1 %17, i32 -1, i32 %18
  store i32 %"83", ptr addrspace(5) %"55", align 4
  %19 = load i64, ptr addrspace(5) %"52", align 8
  %20 = load i32, ptr addrspace(5) %"53", align 4
  %"85" = inttoptr i64 %19 to ptr
  store i32 %20, ptr %"85", align 4
  %21 = load i64, ptr addrspace(5) %"52", align 8
  %"86" = inttoptr i64 %21 to ptr
  %"45" = getelementptr inbounds i8, ptr %"86", i64 4
  %22 = load i32, ptr addrspace(5) %"54", align 4
  store i32 %22, ptr %"45", align 4
  %23 = load i64, ptr addrspace(5) %"52", align 8
  %"87" = inttoptr i64 %23 to ptr
  %"47" = getelementptr inbounds i8, ptr %"87", i64 8
  %24 = load i32, ptr addrspace(5) %"55", align 4
  store i32 %24, ptr %"47", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.ctlz.i32(i32, i1 immarg) #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
