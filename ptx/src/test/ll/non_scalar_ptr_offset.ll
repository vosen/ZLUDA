@0 = addrspace(4) global i64 8

define amdgpu_kernel void @non_scalar_ptr_offset(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #0 {
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i32, align 4, addrspace(5)
  %"43" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"37"

"37":                                             ; preds = %1
  %"44" = load i64, ptr addrspace(4) %"38", align 8
  store i64 %"44", ptr addrspace(5) %"40", align 8
  %"45" = load i64, ptr addrspace(4) %"39", align 8
  store i64 %"45", ptr addrspace(5) %"41", align 8
  %"35" = load i64, ptr addrspace(4) @0, align 8
  %"46" = load i64, ptr addrspace(5) %"40", align 8
  %"54" = inttoptr i64 %"46" to ptr addrspace(1)
  %"36" = getelementptr inbounds i8, ptr addrspace(1) %"54", i64 %"35"
  %"33" = load <2 x i32>, ptr addrspace(1) %"36", align 8
  %"47" = extractelement <2 x i32> %"33", i8 0
  %"48" = extractelement <2 x i32> %"33", i8 1
  store i32 %"47", ptr addrspace(5) %"42", align 4
  store i32 %"48", ptr addrspace(5) %"43", align 4
  %"50" = load i32, ptr addrspace(5) %"42", align 4
  %"51" = load i32, ptr addrspace(5) %"43", align 4
  %"49" = add i32 %"50", %"51"
  store i32 %"49", ptr addrspace(5) %"42", align 4
  %"52" = load i64, ptr addrspace(5) %"41", align 8
  %"53" = load i32, ptr addrspace(5) %"42", align 4
  %"55" = inttoptr i64 %"52" to ptr addrspace(1)
  store i32 %"53", ptr addrspace(1) %"55", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }