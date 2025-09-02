@0 = addrspace(4) global i64 4

define amdgpu_kernel void @min(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #0 {
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i32, align 4, addrspace(5)
  %"42" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"36"

"36":                                             ; preds = %1
  %"43" = load i64, ptr addrspace(4) %"37", align 8
  store i64 %"43", ptr addrspace(5) %"39", align 8
  %"44" = load i64, ptr addrspace(4) %"38", align 8
  store i64 %"44", ptr addrspace(5) %"40", align 8
  %"46" = load i64, ptr addrspace(5) %"39", align 8
  %"54" = inttoptr i64 %"46" to ptr
  %"45" = load i32, ptr %"54", align 4
  store i32 %"45", ptr addrspace(5) %"41", align 4
  %"34" = load i64, ptr addrspace(4) @0, align 8
  %"47" = load i64, ptr addrspace(5) %"39", align 8
  %"55" = inttoptr i64 %"47" to ptr
  %"35" = getelementptr inbounds i8, ptr %"55", i64 %"34"
  %"48" = load i32, ptr %"35", align 4
  store i32 %"48", ptr addrspace(5) %"42", align 4
  %"50" = load i32, ptr addrspace(5) %"41", align 4
  %"51" = load i32, ptr addrspace(5) %"42", align 4
  %"49" = call i32 @llvm.smin.i32(i32 %"50", i32 %"51")
  store i32 %"49", ptr addrspace(5) %"41", align 4
  %"52" = load i64, ptr addrspace(5) %"40", align 8
  %"53" = load i32, ptr addrspace(5) %"41", align 4
  %"56" = inttoptr i64 %"52" to ptr
  store i32 %"53", ptr %"56", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.smin.i32(i32, i32) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }