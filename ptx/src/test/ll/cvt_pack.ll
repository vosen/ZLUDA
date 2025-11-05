define amdgpu_kernel void @cvt_pack(ptr addrspace(4) byref(i64) %"46", ptr addrspace(4) byref(i64) %"47") #0 {
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i32, align 4, addrspace(5)
  %"51" = alloca i32, align 4, addrspace(5)
  %"52" = alloca i32, align 4, addrspace(5)
  %"53" = alloca i32, align 4, addrspace(5)
  %"54" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"45"

"45":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"46", align 8
  store i64 %2, ptr addrspace(5) %"48", align 8
  %3 = load i64, ptr addrspace(4) %"47", align 8
  store i64 %3, ptr addrspace(5) %"49", align 8
  %4 = load i64, ptr addrspace(5) %"48", align 8
  %"75" = inttoptr i64 %4 to ptr
  %5 = load i32, ptr %"75", align 4
  store i32 %5, ptr addrspace(5) %"50", align 4
  %6 = load i64, ptr addrspace(5) %"48", align 8
  %"76" = inttoptr i64 %6 to ptr
  %"40" = getelementptr inbounds i8, ptr %"76", i64 4
  %7 = load i32, ptr %"40", align 4
  store i32 %7, ptr addrspace(5) %"51", align 4
  %8 = load i64, ptr addrspace(5) %"48", align 8
  %"77" = inttoptr i64 %8 to ptr
  %"42" = getelementptr inbounds i8, ptr %"77", i64 8
  %9 = load i32, ptr %"42", align 4
  store i32 %9, ptr addrspace(5) %"52", align 4
  %10 = load i32, ptr addrspace(5) %"50", align 4
  %11 = load i32, ptr addrspace(5) %"51", align 4
  %12 = load i32, ptr addrspace(5) %"52", align 4
  %13 = call i32 @llvm.smax.i32(i32 %10, i32 0)
  %14 = call i32 @llvm.smin.i32(i32 %13, i32 255)
  %15 = call i32 @llvm.smax.i32(i32 %11, i32 0)
  %16 = call i32 @llvm.smin.i32(i32 %15, i32 255)
  %17 = shl i32 %14, 8
  %18 = shl i32 %12, 16
  %19 = or i32 %17, %16
  %"63" = or i32 %19, %18
  store i32 %"63", ptr addrspace(5) %"53", align 4
  %20 = load i32, ptr addrspace(5) %"50", align 4
  %21 = load i32, ptr addrspace(5) %"51", align 4
  %22 = load i32, ptr addrspace(5) %"52", align 4
  %23 = call i32 @llvm.smax.i32(i32 %20, i32 -128)
  %24 = call i32 @llvm.smin.i32(i32 %23, i32 127)
  %25 = call i32 @llvm.smax.i32(i32 %21, i32 -128)
  %26 = call i32 @llvm.smin.i32(i32 %25, i32 127)
  %27 = trunc i32 %24 to i8
  %28 = zext i8 %27 to i32
  %29 = trunc i32 %26 to i8
  %30 = zext i8 %29 to i32
  %31 = shl i32 %28, 8
  %32 = shl i32 %22, 16
  %33 = or i32 %31, %30
  %"67" = or i32 %33, %32
  store i32 %"67", ptr addrspace(5) %"54", align 4
  %34 = load i64, ptr addrspace(5) %"49", align 8
  %35 = load i32, ptr addrspace(5) %"53", align 4
  %"82" = inttoptr i64 %34 to ptr
  store i32 %35, ptr %"82", align 4
  %36 = load i64, ptr addrspace(5) %"49", align 8
  %"84" = inttoptr i64 %36 to ptr
  %"44" = getelementptr inbounds i8, ptr %"84", i64 4
  %37 = load i32, ptr addrspace(5) %"54", align 4
  store i32 %37, ptr %"44", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.smax.i32(i32, i32) #1

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.smin.i32(i32, i32) #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }