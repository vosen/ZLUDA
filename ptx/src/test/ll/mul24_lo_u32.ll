define amdgpu_kernel void @mul24_lo_u32(ptr addrspace(4) byref(i64) %"34", ptr addrspace(4) byref(i64) %"35") #0 {
  %"36" = alloca i64, align 8, addrspace(5)
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i32, align 4, addrspace(5)
  %"39" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"33"

"33":                                             ; preds = %1
  %"40" = load i64, ptr addrspace(4) %"34", align 8
  store i64 %"40", ptr addrspace(5) %"36", align 8
  %"41" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"41", ptr addrspace(5) %"37", align 8
  %"43" = load i64, ptr addrspace(5) %"36", align 8
  %"49" = inttoptr i64 %"43" to ptr
  %"42" = load i32, ptr %"49", align 4
  store i32 %"42", ptr addrspace(5) %"38", align 4
  %"45" = load i32, ptr addrspace(5) %"38", align 4
  %"46" = load i32, ptr addrspace(5) %"38", align 4
  %"44" = call i32 @llvm.amdgcn.mul.u24(i32 %"45", i32 %"46")
  store i32 %"44", ptr addrspace(5) %"39", align 4
  %"47" = load i64, ptr addrspace(5) %"37", align 8
  %"48" = load i32, ptr addrspace(5) %"39", align 4
  %"50" = inttoptr i64 %"47" to ptr
  store i32 %"48", ptr %"50", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.amdgcn.mul.u24(i32, i32) #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
