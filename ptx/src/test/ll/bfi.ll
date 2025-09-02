declare hidden i32 @__zluda_ptx_impl_bfi_b32(i32, i32, i32, i32) #0

define amdgpu_kernel void @bfi(ptr addrspace(4) byref(i64) %"42", ptr addrspace(4) byref(i64) %"43") #1 {
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i32, align 4, addrspace(5)
  %"47" = alloca i32, align 4, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  %"49" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"41"

"41":                                             ; preds = %1
  %"50" = load i64, ptr addrspace(4) %"42", align 8
  store i64 %"50", ptr addrspace(5) %"44", align 8
  %"51" = load i64, ptr addrspace(4) %"43", align 8
  store i64 %"51", ptr addrspace(5) %"45", align 8
  %"53" = load i64, ptr addrspace(5) %"44", align 8
  %"67" = inttoptr i64 %"53" to ptr
  %"52" = load i32, ptr %"67", align 4
  store i32 %"52", ptr addrspace(5) %"46", align 4
  %"54" = load i64, ptr addrspace(5) %"44", align 8
  %"68" = inttoptr i64 %"54" to ptr
  %"36" = getelementptr inbounds i8, ptr %"68", i64 4
  %"55" = load i32, ptr %"36", align 4
  store i32 %"55", ptr addrspace(5) %"47", align 4
  %"56" = load i64, ptr addrspace(5) %"44", align 8
  %"69" = inttoptr i64 %"56" to ptr
  %"38" = getelementptr inbounds i8, ptr %"69", i64 8
  %"57" = load i32, ptr %"38", align 4
  store i32 %"57", ptr addrspace(5) %"48", align 4
  %"58" = load i64, ptr addrspace(5) %"44", align 8
  %"70" = inttoptr i64 %"58" to ptr
  %"40" = getelementptr inbounds i8, ptr %"70", i64 12
  %"59" = load i32, ptr %"40", align 4
  store i32 %"59", ptr addrspace(5) %"49", align 4
  %"61" = load i32, ptr addrspace(5) %"46", align 4
  %"62" = load i32, ptr addrspace(5) %"47", align 4
  %"63" = load i32, ptr addrspace(5) %"48", align 4
  %"64" = load i32, ptr addrspace(5) %"49", align 4
  %"71" = call i32 @__zluda_ptx_impl_bfi_b32(i32 %"61", i32 %"62", i32 %"63", i32 %"64")
  store i32 %"71", ptr addrspace(5) %"46", align 4
  %"65" = load i64, ptr addrspace(5) %"45", align 8
  %"66" = load i32, ptr addrspace(5) %"46", align 4
  %"74" = inttoptr i64 %"65" to ptr
  store i32 %"66", ptr %"74", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }