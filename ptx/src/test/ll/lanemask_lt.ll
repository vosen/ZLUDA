@0 = addrspace(4) global i32 1

declare hidden i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @lanemask_lt(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #1 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i32, align 4, addrspace(5)
  %"45" = alloca i32, align 4, addrspace(5)
  %"46" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"37"

"37":                                             ; preds = %1
  %"47" = load i64, ptr addrspace(4) %"40", align 8
  store i64 %"47", ptr addrspace(5) %"42", align 8
  %"48" = load i64, ptr addrspace(4) %"41", align 8
  store i64 %"48", ptr addrspace(5) %"43", align 8
  %"50" = load i64, ptr addrspace(5) %"42", align 8
  %"60" = inttoptr i64 %"50" to ptr
  %"59" = load i32, ptr %"60", align 4
  store i32 %"59", ptr addrspace(5) %"44", align 4
  %"36" = load i32, ptr addrspace(4) @0, align 4
  %"52" = load i32, ptr addrspace(5) %"44", align 4
  %"61" = add i32 %"52", %"36"
  store i32 %"61", ptr addrspace(5) %"45", align 4
  %"34" = call i32 @__zluda_ptx_impl_sreg_lanemask_lt()
  br label %"38"

"38":                                             ; preds = %"37"
  store i32 %"34", ptr addrspace(5) %"46", align 4
  %"55" = load i32, ptr addrspace(5) %"45", align 4
  %"56" = load i32, ptr addrspace(5) %"46", align 4
  %"64" = add i32 %"55", %"56"
  store i32 %"64", ptr addrspace(5) %"45", align 4
  %"57" = load i64, ptr addrspace(5) %"43", align 8
  %"58" = load i32, ptr addrspace(5) %"45", align 4
  %"67" = inttoptr i64 %"57" to ptr
  store i32 %"58", ptr %"67", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }