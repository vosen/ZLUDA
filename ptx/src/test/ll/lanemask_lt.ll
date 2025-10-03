declare hidden i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @lanemask_lt(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #1 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i32, align 4, addrspace(5)
  %"44" = alloca i32, align 4, addrspace(5)
  %"45" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"36"

"36":                                             ; preds = %1
  %"46" = load i64, ptr addrspace(4) %"39", align 8
  store i64 %"46", ptr addrspace(5) %"41", align 8
  %"47" = load i64, ptr addrspace(4) %"40", align 8
  store i64 %"47", ptr addrspace(5) %"42", align 8
  %"49" = load i64, ptr addrspace(5) %"41", align 8
  %"59" = inttoptr i64 %"49" to ptr
  %"58" = load i32, ptr %"59", align 4
  store i32 %"58", ptr addrspace(5) %"43", align 4
  %"51" = load i32, ptr addrspace(5) %"43", align 4
  %"60" = add i32 %"51", 1
  store i32 %"60", ptr addrspace(5) %"44", align 4
  %"34" = call i32 @__zluda_ptx_impl_sreg_lanemask_lt()
  br label %"37"

"37":                                             ; preds = %"36"
  store i32 %"34", ptr addrspace(5) %"45", align 4
  %"54" = load i32, ptr addrspace(5) %"44", align 4
  %"55" = load i32, ptr addrspace(5) %"45", align 4
  %"63" = add i32 %"54", %"55"
  store i32 %"63", ptr addrspace(5) %"44", align 4
  %"56" = load i64, ptr addrspace(5) %"42", align 8
  %"57" = load i32, ptr addrspace(5) %"44", align 4
  %"66" = inttoptr i64 %"56" to ptr
  store i32 %"57", ptr %"66", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
