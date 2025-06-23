@__ZLUDA_PTX_IMPL_ATTRIBUTE_CLOCK_RATE = addrspace(1) global i32 2124000

define amdgpu_kernel void @mul24_hi_s32(ptr addrspace(4) byref(i64) %"32", ptr addrspace(4) byref(i64) %"33") #0 {
  %"34" = alloca i64, align 8, addrspace(5)
  %"35" = alloca i64, align 8, addrspace(5)
  %"36" = alloca i32, align 4, addrspace(5)
  %"37" = alloca i32, align 4, addrspace(5)
  %"38" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"31"

"31":                                             ; preds = %1
  %"39" = load i64, ptr addrspace(4) %"32", align 4
  store i64 %"39", ptr addrspace(5) %"34", align 4
  %"40" = load i64, ptr addrspace(4) %"33", align 4
  store i64 %"40", ptr addrspace(5) %"35", align 4
  %"42" = load i64, ptr addrspace(5) %"34", align 4
  %"50" = inttoptr i64 %"42" to ptr
  %"41" = load i32, ptr %"50", align 4
  store i32 %"41", ptr addrspace(5) %"36", align 4
  %"44" = load i32, ptr addrspace(5) %"36", align 4
  %"43" = sub i32 0, %"44"
  store i32 %"43", ptr addrspace(5) %"37", align 4
  %"46" = load i32, ptr addrspace(5) %"37", align 4
  %"47" = load i32, ptr addrspace(5) %"36", align 4
  %2 = call i32 @llvm.amdgcn.mul.i24(i32 %"46", i32 %"47")
  %3 = call i32 @llvm.amdgcn.mulhi.i24(i32 %"46", i32 %"47")
  %4 = lshr i32 %2, 16
  %5 = shl i32 %3, 16
  %"45" = or i32 %4, %5
  store i32 %"45", ptr addrspace(5) %"38", align 4
  %"48" = load i64, ptr addrspace(5) %"35", align 4
  %"49" = load i32, ptr addrspace(5) %"38", align 4
  %"51" = inttoptr i64 %"48" to ptr
  store i32 %"49", ptr %"51", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.amdgcn.mul.i24(i32, i32) #1

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.amdgcn.mulhi.i24(i32, i32) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }