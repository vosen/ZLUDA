define amdgpu_kernel void @non_scalar_ptr_offset(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #0 {
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
  %"45" = load i64, ptr addrspace(5) %"39", align 8
  %"53" = inttoptr i64 %"45" to ptr addrspace(1)
  %"35" = getelementptr inbounds i8, ptr addrspace(1) %"53", i64 8
  %"33" = load <2 x i32>, ptr addrspace(1) %"35", align 8
  %"46" = extractelement <2 x i32> %"33", i8 0
  %"47" = extractelement <2 x i32> %"33", i8 1
  store i32 %"46", ptr addrspace(5) %"41", align 4
  store i32 %"47", ptr addrspace(5) %"42", align 4
  %"49" = load i32, ptr addrspace(5) %"41", align 4
  %"50" = load i32, ptr addrspace(5) %"42", align 4
  %"48" = add i32 %"49", %"50"
  store i32 %"48", ptr addrspace(5) %"41", align 4
  %"51" = load i64, ptr addrspace(5) %"40", align 8
  %"52" = load i32, ptr addrspace(5) %"41", align 4
  %"54" = inttoptr i64 %"51" to ptr addrspace(1)
  store i32 %"52", ptr addrspace(1) %"54", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
