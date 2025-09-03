declare hidden i32 @__zluda_ptx_impl_bfi_b32(i32, i32, i32, i32) #0

define amdgpu_kernel void @bfi(ptr addrspace(4) byref(i64) %"42", ptr addrspace(4) byref(i64) %"43") #1 {
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i32, align 4, addrspace(5)
  %"47" = alloca i32, align 4, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  %"49" = alloca i32, align 4, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  store i64 4, ptr addrspace(5) %"54", align 4
  %"58" = alloca i64, align 8, addrspace(5)
  store i64 8, ptr addrspace(5) %"58", align 4
  %"62" = alloca i64, align 8, addrspace(5)
  store i64 12, ptr addrspace(5) %"62", align 4
  br label %1

1:                                                ; preds = %0
  br label %"41"

"41":                                             ; preds = %1
  %"50" = load i64, ptr addrspace(4) %"42", align 8
  store i64 %"50", ptr addrspace(5) %"44", align 8
  %"51" = load i64, ptr addrspace(4) %"43", align 8
  store i64 %"51", ptr addrspace(5) %"45", align 8
  %"53" = load i64, ptr addrspace(5) %"44", align 8
  %"73" = inttoptr i64 %"53" to ptr
  %"52" = load i32, ptr %"73", align 4
  store i32 %"52", ptr addrspace(5) %"46", align 4
  %"55" = load i64, ptr addrspace(5) %"44", align 8
  %"56" = load i64, ptr addrspace(5) %"54", align 8
  %"74" = inttoptr i64 %"55" to ptr
  %"36" = getelementptr inbounds i8, ptr %"74", i64 %"56"
  %"57" = load i32, ptr %"36", align 4
  store i32 %"57", ptr addrspace(5) %"47", align 4
  %"59" = load i64, ptr addrspace(5) %"44", align 8
  %"60" = load i64, ptr addrspace(5) %"58", align 8
  %"75" = inttoptr i64 %"59" to ptr
  %"38" = getelementptr inbounds i8, ptr %"75", i64 %"60"
  %"61" = load i32, ptr %"38", align 4
  store i32 %"61", ptr addrspace(5) %"48", align 4
  %"63" = load i64, ptr addrspace(5) %"44", align 8
  %"64" = load i64, ptr addrspace(5) %"62", align 8
  %"76" = inttoptr i64 %"63" to ptr
  %"40" = getelementptr inbounds i8, ptr %"76", i64 %"64"
  %"65" = load i32, ptr %"40", align 4
  store i32 %"65", ptr addrspace(5) %"49", align 4
  %"67" = load i32, ptr addrspace(5) %"46", align 4
  %"68" = load i32, ptr addrspace(5) %"47", align 4
  %"69" = load i32, ptr addrspace(5) %"48", align 4
  %"70" = load i32, ptr addrspace(5) %"49", align 4
  %"77" = call i32 @__zluda_ptx_impl_bfi_b32(i32 %"67", i32 %"68", i32 %"69", i32 %"70")
  store i32 %"77", ptr addrspace(5) %"46", align 4
  %"71" = load i64, ptr addrspace(5) %"45", align 8
  %"72" = load i32, ptr addrspace(5) %"46", align 4
  %"80" = inttoptr i64 %"71" to ptr
  store i32 %"72", ptr %"80", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }