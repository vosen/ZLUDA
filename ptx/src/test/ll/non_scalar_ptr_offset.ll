define amdgpu_kernel void @non_scalar_ptr_offset(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #0 {
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i32, align 4, addrspace(5)
  %"42" = alloca i32, align 4, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  store i64 8, ptr addrspace(5) %"45", align 4
  br label %1

1:                                                ; preds = %0
  br label %"36"

"36":                                             ; preds = %1
  %"43" = load i64, ptr addrspace(4) %"37", align 8
  store i64 %"43", ptr addrspace(5) %"39", align 8
  %"44" = load i64, ptr addrspace(4) %"38", align 8
  store i64 %"44", ptr addrspace(5) %"40", align 8
  %"46" = load i64, ptr addrspace(5) %"39", align 8
  %"47" = load i64, ptr addrspace(5) %"45", align 8
  %"55" = inttoptr i64 %"46" to ptr addrspace(1)
  %"35" = getelementptr inbounds i8, ptr addrspace(1) %"55", i64 %"47"
  %"33" = load <2 x i32>, ptr addrspace(1) %"35", align 8
  %"48" = extractelement <2 x i32> %"33", i8 0
  %"49" = extractelement <2 x i32> %"33", i8 1
  store i32 %"48", ptr addrspace(5) %"41", align 4
  store i32 %"49", ptr addrspace(5) %"42", align 4
  %"51" = load i32, ptr addrspace(5) %"41", align 4
  %"52" = load i32, ptr addrspace(5) %"42", align 4
  %"50" = add i32 %"51", %"52"
  store i32 %"50", ptr addrspace(5) %"41", align 4
  %"53" = load i64, ptr addrspace(5) %"40", align 8
  %"54" = load i32, ptr addrspace(5) %"41", align 4
  %"56" = inttoptr i64 %"53" to ptr addrspace(1)
  store i32 %"54", ptr addrspace(1) %"56", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }