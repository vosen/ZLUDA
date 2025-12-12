define amdgpu_kernel void @cvt_relu_f16x2_f32(ptr addrspace(4) byref(i64) %"49", ptr addrspace(4) byref(i64) %"50") #0 {
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i32, align 4, addrspace(5)
  %"54" = alloca i32, align 4, addrspace(5)
  %"55" = alloca i32, align 4, addrspace(5)
  %"56" = alloca i32, align 4, addrspace(5)
  %"57" = alloca <2 x half>, align 4, addrspace(5)
  %"58" = alloca <2 x half>, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"48"

"48":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"49", align 8
  store i64 %2, ptr addrspace(5) %"51", align 8
  %3 = load i64, ptr addrspace(4) %"50", align 8
  store i64 %3, ptr addrspace(5) %"52", align 8
  %4 = load i64, ptr addrspace(5) %"51", align 8
  %"80" = inttoptr i64 %4 to ptr
  %5 = load float, ptr %"80", align 4
  %"61" = bitcast float %5 to i32
  store i32 %"61", ptr addrspace(5) %"53", align 4
  %6 = load i64, ptr addrspace(5) %"51", align 8
  %"81" = inttoptr i64 %6 to ptr
  %"41" = getelementptr inbounds i8, ptr %"81", i64 4
  %7 = load float, ptr %"41", align 4
  %"64" = bitcast float %7 to i32
  store i32 %"64", ptr addrspace(5) %"54", align 4
  %8 = load i64, ptr addrspace(5) %"51", align 8
  %"83" = inttoptr i64 %8 to ptr
  %"43" = getelementptr inbounds i8, ptr %"83", i64 8
  %9 = load float, ptr %"43", align 4
  %"66" = bitcast float %9 to i32
  store i32 %"66", ptr addrspace(5) %"55", align 4
  %10 = load i64, ptr addrspace(5) %"51", align 8
  %"85" = inttoptr i64 %10 to ptr
  %"45" = getelementptr inbounds i8, ptr %"85", i64 12
  %11 = load float, ptr %"45", align 4
  %"68" = bitcast float %11 to i32
  store i32 %"68", ptr addrspace(5) %"56", align 4
  %12 = load i32, ptr addrspace(5) %"53", align 4
  %13 = load i32, ptr addrspace(5) %"54", align 4
  %"87" = bitcast i32 %12 to float
  %"88" = bitcast i32 %13 to float
  %14 = fptrunc float %"87" to half
  %15 = fptrunc float %"88" to half
  %16 = call half @llvm.maximum.f16(half %14, half 0xH0000)
  %17 = call half @llvm.maximum.f16(half %15, half 0xH0000)
  %18 = insertelement <2 x half> poison, half %16, i32 1
  %"69" = insertelement <2 x half> %18, half %17, i32 0
  store <2 x half> %"69", ptr addrspace(5) %"57", align 4
  %19 = load i32, ptr addrspace(5) %"55", align 4
  %20 = load i32, ptr addrspace(5) %"56", align 4
  %"89" = bitcast i32 %19 to float
  %"90" = bitcast i32 %20 to float
  %21 = fptrunc float %"89" to half
  %22 = fptrunc float %"90" to half
  %23 = call half @llvm.maximum.f16(half %21, half 0xH0000)
  %24 = call half @llvm.maximum.f16(half %22, half 0xH0000)
  %25 = insertelement <2 x half> poison, half %23, i32 1
  %"72" = insertelement <2 x half> %25, half %24, i32 0
  store <2 x half> %"72", ptr addrspace(5) %"58", align 4
  %26 = load i64, ptr addrspace(5) %"52", align 8
  %27 = load <2 x half>, ptr addrspace(5) %"57", align 4
  %"91" = inttoptr i64 %26 to ptr addrspace(1)
  %"92" = bitcast <2 x half> %27 to i32
  store i32 %"92", ptr addrspace(1) %"91", align 4
  %28 = load i64, ptr addrspace(5) %"52", align 8
  %"93" = inttoptr i64 %28 to ptr addrspace(1)
  %"47" = getelementptr inbounds i8, ptr addrspace(1) %"93", i64 4
  %29 = load <2 x half>, ptr addrspace(5) %"58", align 4
  %"94" = bitcast <2 x half> %29 to i32
  store i32 %"94", ptr addrspace(1) %"47", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare half @llvm.maximum.f16(half, half) #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }