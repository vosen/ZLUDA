define amdgpu_kernel void @fma_f16x2(ptr addrspace(4) byref(i64) %"42", ptr addrspace(4) byref(i64) %"43") #0 {
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i32, align 4, addrspace(5)
  %"47" = alloca i32, align 4, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"41"

"41":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"42", align 8
  store i64 %2, ptr addrspace(5) %"44", align 8
  %3 = load i64, ptr addrspace(4) %"43", align 8
  store i64 %3, ptr addrspace(5) %"45", align 8
  %4 = load i64, ptr addrspace(5) %"44", align 8
  %"63" = inttoptr i64 %4 to ptr
  %5 = load i32, ptr %"63", align 4
  store i32 %5, ptr addrspace(5) %"46", align 4
  %6 = load i64, ptr addrspace(5) %"44", align 8
  %"64" = inttoptr i64 %6 to ptr
  %"38" = getelementptr inbounds i8, ptr %"64", i64 4
  %7 = load i32, ptr %"38", align 4
  store i32 %7, ptr addrspace(5) %"47", align 4
  %8 = load i64, ptr addrspace(5) %"44", align 8
  %"65" = inttoptr i64 %8 to ptr
  %"40" = getelementptr inbounds i8, ptr %"65", i64 8
  %9 = load i32, ptr %"40", align 4
  store i32 %9, ptr addrspace(5) %"48", align 4
  %10 = load i32, ptr addrspace(5) %"46", align 4
  %11 = load i32, ptr addrspace(5) %"47", align 4
  %12 = load i32, ptr addrspace(5) %"48", align 4
  %"67" = bitcast i32 %10 to <2 x half>
  %"68" = bitcast i32 %11 to <2 x half>
  %"69" = bitcast i32 %12 to <2 x half>
  %"66" = call <2 x half> @llvm.fma.v2f16(<2 x half> %"67", <2 x half> %"68", <2 x half> %"69")
  %"57" = bitcast <2 x half> %"66" to i32
  store i32 %"57", ptr addrspace(5) %"46", align 4
  %13 = load i64, ptr addrspace(5) %"45", align 8
  %14 = load i32, ptr addrspace(5) %"46", align 4
  %"70" = inttoptr i64 %13 to ptr
  store i32 %14, ptr %"70", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare <2 x half> @llvm.fma.v2f16(<2 x half>, <2 x half>, <2 x half>) #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }