@0 = addrspace(4) global i64 4
@1 = addrspace(4) global i64 8

declare hidden i32 @__zluda_ptx_impl_bfe_u32(i32, i32, i32) #0

define amdgpu_kernel void @bfe(ptr addrspace(4) byref(i64) %"41", ptr addrspace(4) byref(i64) %"42") #1 {
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i32, align 4, addrspace(5)
  %"46" = alloca i32, align 4, addrspace(5)
  %"47" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"40"

"40":                                             ; preds = %1
  %"48" = load i64, ptr addrspace(4) %"41", align 8
  store i64 %"48", ptr addrspace(5) %"43", align 8
  %"49" = load i64, ptr addrspace(4) %"42", align 8
  store i64 %"49", ptr addrspace(5) %"44", align 8
  %"51" = load i64, ptr addrspace(5) %"43", align 8
  %"62" = inttoptr i64 %"51" to ptr
  %"50" = load i32, ptr %"62", align 4
  store i32 %"50", ptr addrspace(5) %"45", align 4
  %"35" = load i64, ptr addrspace(4) @0, align 8
  %"52" = load i64, ptr addrspace(5) %"43", align 8
  %"63" = inttoptr i64 %"52" to ptr
  %"36" = getelementptr inbounds i8, ptr %"63", i64 %"35"
  %"53" = load i32, ptr %"36", align 4
  store i32 %"53", ptr addrspace(5) %"46", align 4
  %"38" = load i64, ptr addrspace(4) @1, align 8
  %"54" = load i64, ptr addrspace(5) %"43", align 8
  %"64" = inttoptr i64 %"54" to ptr
  %"39" = getelementptr inbounds i8, ptr %"64", i64 %"38"
  %"55" = load i32, ptr %"39", align 4
  store i32 %"55", ptr addrspace(5) %"47", align 4
  %"57" = load i32, ptr addrspace(5) %"45", align 4
  %"58" = load i32, ptr addrspace(5) %"46", align 4
  %"59" = load i32, ptr addrspace(5) %"47", align 4
  %"56" = call i32 @__zluda_ptx_impl_bfe_u32(i32 %"57", i32 %"58", i32 %"59")
  store i32 %"56", ptr addrspace(5) %"45", align 4
  %"60" = load i64, ptr addrspace(5) %"44", align 8
  %"61" = load i32, ptr addrspace(5) %"45", align 4
  %"65" = inttoptr i64 %"60" to ptr
  store i32 %"61", ptr %"65", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }