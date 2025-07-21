define amdgpu_kernel void @non_scalar_ptr_offset(ptr addrspace(4) byref(i64) %"34", ptr addrspace(4) byref(i64) %"35") #0 {
  %"36" = alloca i64, align 8, addrspace(5)
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i32, align 4, addrspace(5)
  %"39" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"33"

"33":                                             ; preds = %1
  %"40" = load i64, ptr addrspace(4) %"34", align 4
  store i64 %"40", ptr addrspace(5) %"36", align 4
  %"41" = load i64, ptr addrspace(4) %"35", align 4
  store i64 %"41", ptr addrspace(5) %"37", align 4
  %"42" = load i64, ptr addrspace(5) %"36", align 4
  %"50" = inttoptr i64 %"42" to ptr addrspace(1)
  %"32" = getelementptr inbounds i8, ptr addrspace(1) %"50", i64 8
  %"30" = load <2 x i32>, ptr addrspace(1) %"32", align 8
  %"43" = extractelement <2 x i32> %"30", i8 0
  %"44" = extractelement <2 x i32> %"30", i8 1
  store i32 %"43", ptr addrspace(5) %"38", align 4
  store i32 %"44", ptr addrspace(5) %"39", align 4
  %"46" = load i32, ptr addrspace(5) %"38", align 4
  %"47" = load i32, ptr addrspace(5) %"39", align 4
  %"45" = add i32 %"46", %"47"
  store i32 %"45", ptr addrspace(5) %"38", align 4
  %"48" = load i64, ptr addrspace(5) %"37", align 4
  %"49" = load i32, ptr addrspace(5) %"38", align 4
  %"51" = inttoptr i64 %"48" to ptr addrspace(1)
  store i32 %"49", ptr addrspace(1) %"51", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }