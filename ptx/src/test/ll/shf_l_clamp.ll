define amdgpu_kernel void @shf_l_clamp(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #0 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i32, align 4, addrspace(5)
  %"45" = alloca i32, align 4, addrspace(5)
  %"46" = alloca i32, align 4, addrspace(5)
  %"47" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"39"

"39":                                             ; preds = %1
  %"48" = load i64, ptr addrspace(4) %"40", align 8
  store i64 %"48", ptr addrspace(5) %"42", align 8
  %"49" = load i64, ptr addrspace(4) %"41", align 8
  store i64 %"49", ptr addrspace(5) %"43", align 8
  %"51" = load i64, ptr addrspace(5) %"42", align 8
  %"62" = inttoptr i64 %"51" to ptr
  %"50" = load i32, ptr %"62", align 4
  store i32 %"50", ptr addrspace(5) %"44", align 4
  %"52" = load i64, ptr addrspace(5) %"42", align 8
  %"63" = inttoptr i64 %"52" to ptr
  %"36" = getelementptr inbounds i8, ptr %"63", i64 4
  %"53" = load i32, ptr %"36", align 4
  store i32 %"53", ptr addrspace(5) %"45", align 4
  %"54" = load i64, ptr addrspace(5) %"42", align 8
  %"64" = inttoptr i64 %"54" to ptr
  %"38" = getelementptr inbounds i8, ptr %"64", i64 8
  %"55" = load i32, ptr %"38", align 4
  store i32 %"55", ptr addrspace(5) %"46", align 4
  %"57" = load i32, ptr addrspace(5) %"44", align 4
  %"58" = load i32, ptr addrspace(5) %"45", align 4
  %"59" = load i32, ptr addrspace(5) %"46", align 4
  %2 = call i32 @llvm.fshl.i32(i32 %"58", i32 %"57", i32 %"59")
  %3 = icmp uge i32 %"59", 32
  %"65" = select i1 %3, i32 %"57", i32 %2
  store i32 %"65", ptr addrspace(5) %"47", align 4
  %"60" = load i64, ptr addrspace(5) %"43", align 8
  %"61" = load i32, ptr addrspace(5) %"47", align 4
  %"66" = inttoptr i64 %"60" to ptr
  store i32 %"61", ptr %"66", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.fshl.i32(i32, i32, i32) #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }