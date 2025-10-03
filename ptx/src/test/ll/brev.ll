define amdgpu_kernel void @brev(ptr addrspace(4) byref(i64) %"33", ptr addrspace(4) byref(i64) %"34") #0 {
  %"35" = alloca i64, align 8, addrspace(5)
  %"36" = alloca i64, align 8, addrspace(5)
  %"37" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"32"

"32":                                             ; preds = %1
  %"38" = load i64, ptr addrspace(4) %"33", align 8
  store i64 %"38", ptr addrspace(5) %"35", align 8
  %"39" = load i64, ptr addrspace(4) %"34", align 8
  store i64 %"39", ptr addrspace(5) %"36", align 8
  %"41" = load i64, ptr addrspace(5) %"35", align 8
  %"46" = inttoptr i64 %"41" to ptr
  %"40" = load i32, ptr %"46", align 4
  store i32 %"40", ptr addrspace(5) %"37", align 4
  %"43" = load i32, ptr addrspace(5) %"37", align 4
  %"42" = call i32 @llvm.bitreverse.i32(i32 %"43")
  store i32 %"42", ptr addrspace(5) %"37", align 4
  %"44" = load i64, ptr addrspace(5) %"36", align 8
  %"45" = load i32, ptr addrspace(5) %"37", align 4
  %"47" = inttoptr i64 %"44" to ptr
  store i32 %"45", ptr %"47", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.bitreverse.i32(i32) #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
