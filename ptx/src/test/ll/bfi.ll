declare i32 @__zluda_ptx_impl_bfi_b32(i32, i32, i32, i32) #0

define amdgpu_kernel void @bfi(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #1 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i32, align 4, addrspace(5)
  %"44" = alloca i32, align 4, addrspace(5)
  %"45" = alloca i32, align 4, addrspace(5)
  %"46" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"38"

"38":                                             ; preds = %1
  %"47" = load i64, ptr addrspace(4) %"39", align 8
  store i64 %"47", ptr addrspace(5) %"41", align 8
  %"48" = load i64, ptr addrspace(4) %"40", align 8
  store i64 %"48", ptr addrspace(5) %"42", align 8
  %"50" = load i64, ptr addrspace(5) %"41", align 8
  %"64" = inttoptr i64 %"50" to ptr
  %"49" = load i32, ptr %"64", align 4
  store i32 %"49", ptr addrspace(5) %"43", align 4
  %"51" = load i64, ptr addrspace(5) %"41", align 8
  %"65" = inttoptr i64 %"51" to ptr
  %"33" = getelementptr inbounds i8, ptr %"65", i64 4
  %"52" = load i32, ptr %"33", align 4
  store i32 %"52", ptr addrspace(5) %"44", align 4
  %"53" = load i64, ptr addrspace(5) %"41", align 8
  %"66" = inttoptr i64 %"53" to ptr
  %"35" = getelementptr inbounds i8, ptr %"66", i64 8
  %"54" = load i32, ptr %"35", align 4
  store i32 %"54", ptr addrspace(5) %"45", align 4
  %"55" = load i64, ptr addrspace(5) %"41", align 8
  %"67" = inttoptr i64 %"55" to ptr
  %"37" = getelementptr inbounds i8, ptr %"67", i64 12
  %"56" = load i32, ptr %"37", align 4
  store i32 %"56", ptr addrspace(5) %"46", align 4
  %"58" = load i32, ptr addrspace(5) %"43", align 4
  %"59" = load i32, ptr addrspace(5) %"44", align 4
  %"60" = load i32, ptr addrspace(5) %"45", align 4
  %"61" = load i32, ptr addrspace(5) %"46", align 4
  %"68" = call i32 @__zluda_ptx_impl_bfi_b32(i32 %"58", i32 %"59", i32 %"60", i32 %"61")
  store i32 %"68", ptr addrspace(5) %"43", align 4
  %"62" = load i64, ptr addrspace(5) %"42", align 8
  %"63" = load i32, ptr addrspace(5) %"43", align 4
  %"71" = inttoptr i64 %"62" to ptr
  store i32 %"63", ptr %"71", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }