define amdgpu_kernel void @mul24_hi_u32(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #0 {
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i32, align 4, addrspace(5)
  %"42" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"36"

"36":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"37", align 8
  store i64 %2, ptr addrspace(5) %"39", align 8
  %3 = load i64, ptr addrspace(4) %"38", align 8
  store i64 %3, ptr addrspace(5) %"40", align 8
  %4 = load i64, ptr addrspace(5) %"39", align 8
  %"52" = inttoptr i64 %4 to ptr
  %5 = load i32, ptr %"52", align 4
  store i32 %5, ptr addrspace(5) %"41", align 4
  %6 = load i32, ptr addrspace(5) %"41", align 4
  %7 = load i32, ptr addrspace(5) %"41", align 4
  %8 = call i32 @llvm.amdgcn.mul.u24(i32 %6, i32 %7)
  %9 = call i32 @llvm.amdgcn.mulhi.u24(i32 %6, i32 %7)
  %10 = lshr i32 %8, 16
  %11 = shl i32 %9, 16
  %"47" = or i32 %10, %11
  store i32 %"47", ptr addrspace(5) %"42", align 4
  %12 = load i64, ptr addrspace(5) %"40", align 8
  %13 = load i32, ptr addrspace(5) %"42", align 4
  %"53" = inttoptr i64 %12 to ptr
  store i32 %13, ptr %"53", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.amdgcn.mul.u24(i32, i32) #1

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.amdgcn.mulhi.u24(i32, i32) #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }