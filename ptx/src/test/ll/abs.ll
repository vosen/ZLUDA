define amdgpu_kernel void @abs(ptr addrspace(4) byref(i64) %"31", ptr addrspace(4) byref(i64) %"32") #0 {
  %"33" = alloca i64, align 8, addrspace(5)
  %"34" = alloca i64, align 8, addrspace(5)
  %"35" = alloca i32, align 4, addrspace(5)
  %"36" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"30"

"30":                                             ; preds = %1
  %"37" = load i64, ptr addrspace(4) %"31", align 4
  store i64 %"37", ptr addrspace(5) %"33", align 4
  %"38" = load i64, ptr addrspace(4) %"32", align 4
  store i64 %"38", ptr addrspace(5) %"34", align 4
  %"40" = load i64, ptr addrspace(5) %"33", align 4
  %"45" = inttoptr i64 %"40" to ptr
  %"39" = load i32, ptr %"45", align 4
  store i32 %"39", ptr addrspace(5) %"35", align 4
  %"42" = load i32, ptr addrspace(5) %"35", align 4
  %"41" = call i32 @llvm.abs.i32(i32 %"42", i1 false)
  store i32 %"41", ptr addrspace(5) %"36", align 4
  %"43" = load i64, ptr addrspace(5) %"34", align 4
  %"44" = load i32, ptr addrspace(5) %"36", align 4
  %"46" = inttoptr i64 %"43" to ptr
  store i32 %"44", ptr %"46", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.abs.i32(i32, i1 immarg) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }