define amdgpu_kernel void @max(ptr addrspace(4) byref(i64) %"42", ptr addrspace(4) byref(i64) %"43") #0 {
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i32, align 4, addrspace(5)
  %"47" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"41"

"41":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"42", align 8
  store i64 %2, ptr addrspace(5) %"44", align 8
  %3 = load i64, ptr addrspace(4) %"43", align 8
  store i64 %3, ptr addrspace(5) %"45", align 8
  %4 = load i64, ptr addrspace(5) %"44", align 8
  %"59" = inttoptr i64 %4 to ptr
  %5 = load i32, ptr %"59", align 4
  store i32 %5, ptr addrspace(5) %"46", align 4
  %6 = load i64, ptr addrspace(5) %"44", align 8
  %"60" = inttoptr i64 %6 to ptr
  %"40" = getelementptr inbounds i8, ptr %"60", i64 4
  %7 = load i32, ptr %"40", align 4
  store i32 %7, ptr addrspace(5) %"47", align 4
  %8 = load i32, ptr addrspace(5) %"46", align 4
  %9 = load i32, ptr addrspace(5) %"47", align 4
  %10 = call i32 @llvm.smax.i32(i32 %8, i32 %9)
  store i32 %10, ptr addrspace(5) %"46", align 4
  %11 = load i64, ptr addrspace(5) %"45", align 8
  %12 = load i32, ptr addrspace(5) %"46", align 4
  %"61" = inttoptr i64 %11 to ptr
  store i32 %12, ptr %"61", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.smax.i32(i32, i32) #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
