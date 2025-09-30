define amdgpu_kernel void @cvt_sat_s_u(ptr addrspace(4) byref(i64) %"35", ptr addrspace(4) byref(i64) %"36") #0 {
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i32, align 4, addrspace(5)
  %"40" = alloca i32, align 4, addrspace(5)
  %"41" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"34"

"34":                                             ; preds = %1
  %"42" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"42", ptr addrspace(5) %"37", align 8
  %"43" = load i64, ptr addrspace(4) %"36", align 8
  store i64 %"43", ptr addrspace(5) %"38", align 8
  %"45" = load i64, ptr addrspace(5) %"37", align 8
  %"52" = inttoptr i64 %"45" to ptr
  %"44" = load i32, ptr %"52", align 4
  store i32 %"44", ptr addrspace(5) %"39", align 4
  %"47" = load i32, ptr addrspace(5) %"39", align 4
  %2 = call i32 @llvm.smax.i32(i32 %"47", i32 0)
  %3 = call i32 @llvm.smin.i32(i32 %2, i32 2147483647)
  store i32 %3, ptr addrspace(5) %"40", align 4
  %"49" = load i32, ptr addrspace(5) %"40", align 4
  store i32 %"49", ptr addrspace(5) %"41", align 4
  %"50" = load i64, ptr addrspace(5) %"38", align 8
  %"51" = load i32, ptr addrspace(5) %"41", align 4
  %"53" = inttoptr i64 %"50" to ptr
  store i32 %"51", ptr %"53", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.smax.i32(i32, i32) #1

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.smin.i32(i32, i32) #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
