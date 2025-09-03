define amdgpu_kernel void @shf_l(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #0 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i32, align 4, addrspace(5)
  %"45" = alloca i32, align 4, addrspace(5)
  %"46" = alloca i32, align 4, addrspace(5)
  %"47" = alloca i32, align 4, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  store i64 4, ptr addrspace(5) %"52", align 4
  %"56" = alloca i64, align 8, addrspace(5)
  store i64 8, ptr addrspace(5) %"56", align 4
  br label %1

1:                                                ; preds = %0
  br label %"39"

"39":                                             ; preds = %1
  %"48" = load i64, ptr addrspace(4) %"40", align 8
  store i64 %"48", ptr addrspace(5) %"42", align 8
  %"49" = load i64, ptr addrspace(4) %"41", align 8
  store i64 %"49", ptr addrspace(5) %"43", align 8
  %"51" = load i64, ptr addrspace(5) %"42", align 8
  %"66" = inttoptr i64 %"51" to ptr
  %"50" = load i32, ptr %"66", align 4
  store i32 %"50", ptr addrspace(5) %"44", align 4
  %"53" = load i64, ptr addrspace(5) %"42", align 8
  %"54" = load i64, ptr addrspace(5) %"52", align 8
  %"67" = inttoptr i64 %"53" to ptr
  %"36" = getelementptr inbounds i8, ptr %"67", i64 %"54"
  %"55" = load i32, ptr %"36", align 4
  store i32 %"55", ptr addrspace(5) %"45", align 4
  %"57" = load i64, ptr addrspace(5) %"42", align 8
  %"58" = load i64, ptr addrspace(5) %"56", align 8
  %"68" = inttoptr i64 %"57" to ptr
  %"38" = getelementptr inbounds i8, ptr %"68", i64 %"58"
  %"59" = load i32, ptr %"38", align 4
  store i32 %"59", ptr addrspace(5) %"46", align 4
  %"61" = load i32, ptr addrspace(5) %"44", align 4
  %"62" = load i32, ptr addrspace(5) %"45", align 4
  %"63" = load i32, ptr addrspace(5) %"46", align 4
  %2 = call i32 @llvm.fshl.i32(i32 %"62", i32 %"61", i32 %"63")
  %3 = icmp uge i32 %"63", 32
  %"69" = select i1 %3, i32 %"61", i32 %2
  store i32 %"69", ptr addrspace(5) %"47", align 4
  %"64" = load i64, ptr addrspace(5) %"43", align 8
  %"65" = load i32, ptr addrspace(5) %"47", align 4
  %"70" = inttoptr i64 %"64" to ptr
  store i32 %"65", ptr %"70", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.fshl.i32(i32, i32, i32) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }