define amdgpu_kernel void @cvt_s32_f32(ptr addrspace(4) byref(i64) %"44", ptr addrspace(4) byref(i64) %"45") #0 {
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  %"49" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"43"

"43":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"44", align 8
  store i64 %2, ptr addrspace(5) %"46", align 8
  %3 = load i64, ptr addrspace(4) %"45", align 8
  store i64 %3, ptr addrspace(5) %"47", align 8
  %4 = load i64, ptr addrspace(5) %"46", align 8
  %"65" = inttoptr i64 %4 to ptr
  %5 = load float, ptr %"65", align 4
  %"52" = bitcast float %5 to i32
  store i32 %"52", ptr addrspace(5) %"48", align 4
  %6 = load i64, ptr addrspace(5) %"46", align 8
  %"66" = inttoptr i64 %6 to ptr
  %"40" = getelementptr inbounds i8, ptr %"66", i64 4
  %7 = load float, ptr %"40", align 4
  %"55" = bitcast float %7 to i32
  store i32 %"55", ptr addrspace(5) %"49", align 4
  %8 = load i32, ptr addrspace(5) %"48", align 4
  %"69" = bitcast i32 %8 to float
  %9 = call float @llvm.ceil.f32(float %"69")
  %10 = fptosi float %9 to i32
  %"68" = freeze i32 %10
  store i32 %"68", ptr addrspace(5) %"48", align 4
  %11 = load i32, ptr addrspace(5) %"49", align 4
  %"71" = bitcast i32 %11 to float
  %12 = call float @llvm.ceil.f32(float %"71")
  %13 = fptosi float %12 to i32
  %"70" = freeze i32 %13
  store i32 %"70", ptr addrspace(5) %"49", align 4
  %14 = load i64, ptr addrspace(5) %"47", align 8
  %15 = load i32, ptr addrspace(5) %"48", align 4
  %"72" = inttoptr i64 %14 to ptr addrspace(1)
  store i32 %15, ptr addrspace(1) %"72", align 4
  %16 = load i64, ptr addrspace(5) %"47", align 8
  %"74" = inttoptr i64 %16 to ptr addrspace(1)
  %"42" = getelementptr inbounds i8, ptr addrspace(1) %"74", i64 4
  %17 = load i32, ptr addrspace(5) %"49", align 4
  store i32 %17, ptr addrspace(1) %"42", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare float @llvm.ceil.f32(float) #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
