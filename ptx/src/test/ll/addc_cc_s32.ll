define amdgpu_kernel void @addc_cc_s32(ptr addrspace(4) byref(i64) %"49", ptr addrspace(4) byref(i64) %"50") #0 {
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i32, align 4, addrspace(5)
  %"54" = alloca i32, align 4, addrspace(5)
  %"55" = alloca i64, align 8, addrspace(5)
  %"62" = alloca i32, align 4, addrspace(5)
  %__ZLUDA_REG_CC_CF = alloca i1, align 1, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"48"

"48":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"49", align 8
  store i64 %2, ptr addrspace(5) %"51", align 8
  %3 = load i64, ptr addrspace(4) %"50", align 8
  store i64 %3, ptr addrspace(5) %"52", align 8
  %4 = load i64, ptr addrspace(5) %"51", align 8
  %"73" = inttoptr i64 %4 to ptr
  %5 = load i32, ptr %"73", align 4
  store i32 %5, ptr addrspace(5) %"53", align 4
  %6 = load i64, ptr addrspace(5) %"51", align 8
  %"74" = inttoptr i64 %6 to ptr
  %"42" = getelementptr inbounds i8, ptr %"74", i64 4
  %7 = load i32, ptr %"42", align 4
  store i32 %7, ptr addrspace(5) %"54", align 4
  %8 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 -1, i32 1)
  %9 = extractvalue { i32, i1 } %8, 0
  %10 = extractvalue { i32, i1 } %8, 1
  store i1 %10, ptr addrspace(5) %__ZLUDA_REG_CC_CF, align 1
  store i32 %9, ptr addrspace(5) %"62", align 4
  %11 = load i32, ptr addrspace(5) %"53", align 4
  %12 = load i32, ptr addrspace(5) %"54", align 4
  %13 = load i1, ptr addrspace(5) %__ZLUDA_REG_CC_CF, align 1
  %14 = zext i1 %13 to i32
  %15 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %11, i32 %12)
  %16 = extractvalue { i32, i1 } %15, 0
  %17 = extractvalue { i32, i1 } %15, 1
  %18 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %16, i32 %14)
  %19 = extractvalue { i32, i1 } %18, 0
  %20 = extractvalue { i32, i1 } %18, 1
  %21 = or i1 %17, %20
  store i1 %21, ptr addrspace(5) %__ZLUDA_REG_CC_CF, align 1
  store i32 %19, ptr addrspace(5) %"53", align 4
  %22 = load i1, ptr addrspace(5) %__ZLUDA_REG_CC_CF, align 1
  %23 = zext i1 %22 to i32
  %24 = add i32 0, %23
  store i32 %24, ptr addrspace(5) %"54", align 4
  %25 = load i32, ptr addrspace(5) %"53", align 4
  %26 = load i32, ptr addrspace(5) %"54", align 4
  %27 = insertelement <2 x i32> undef, i32 %25, i8 0
  %"47" = insertelement <2 x i32> %27, i32 %26, i8 1
  %"80" = bitcast <2 x i32> %"47" to i64
  store i64 %"80", ptr addrspace(5) %"55", align 8
  %28 = load i64, ptr addrspace(5) %"52", align 8
  %29 = load i64, ptr addrspace(5) %"55", align 8
  %"81" = inttoptr i64 %28 to ptr
  store i64 %29, ptr %"81", align 8
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.uadd.with.overflow.i32(i32, i32) #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
