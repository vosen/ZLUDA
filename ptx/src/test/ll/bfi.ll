@0 = addrspace(4) global i64 4
@1 = addrspace(4) global i64 8
@2 = addrspace(4) global i64 12

declare hidden i32 @__zluda_ptx_impl_bfi_b32(i32, i32, i32, i32) #0

define amdgpu_kernel void @bfi(ptr addrspace(4) byref(i64) %"45", ptr addrspace(4) byref(i64) %"46") #1 {
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i32, align 4, addrspace(5)
  %"50" = alloca i32, align 4, addrspace(5)
  %"51" = alloca i32, align 4, addrspace(5)
  %"52" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"44"

"44":                                             ; preds = %1
  %"53" = load i64, ptr addrspace(4) %"45", align 8
  store i64 %"53", ptr addrspace(5) %"47", align 8
  %"54" = load i64, ptr addrspace(4) %"46", align 8
  store i64 %"54", ptr addrspace(5) %"48", align 8
  %"56" = load i64, ptr addrspace(5) %"47", align 8
  %"70" = inttoptr i64 %"56" to ptr
  %"55" = load i32, ptr %"70", align 4
  store i32 %"55", ptr addrspace(5) %"49", align 4
  %"36" = load i64, ptr addrspace(4) @0, align 8
  %"57" = load i64, ptr addrspace(5) %"47", align 8
  %"71" = inttoptr i64 %"57" to ptr
  %"37" = getelementptr inbounds i8, ptr %"71", i64 %"36"
  %"58" = load i32, ptr %"37", align 4
  store i32 %"58", ptr addrspace(5) %"50", align 4
  %"39" = load i64, ptr addrspace(4) @1, align 8
  %"59" = load i64, ptr addrspace(5) %"47", align 8
  %"72" = inttoptr i64 %"59" to ptr
  %"40" = getelementptr inbounds i8, ptr %"72", i64 %"39"
  %"60" = load i32, ptr %"40", align 4
  store i32 %"60", ptr addrspace(5) %"51", align 4
  %"42" = load i64, ptr addrspace(4) @2, align 8
  %"61" = load i64, ptr addrspace(5) %"47", align 8
  %"73" = inttoptr i64 %"61" to ptr
  %"43" = getelementptr inbounds i8, ptr %"73", i64 %"42"
  %"62" = load i32, ptr %"43", align 4
  store i32 %"62", ptr addrspace(5) %"52", align 4
  %"64" = load i32, ptr addrspace(5) %"49", align 4
  %"65" = load i32, ptr addrspace(5) %"50", align 4
  %"66" = load i32, ptr addrspace(5) %"51", align 4
  %"67" = load i32, ptr addrspace(5) %"52", align 4
  %"74" = call i32 @__zluda_ptx_impl_bfi_b32(i32 %"64", i32 %"65", i32 %"66", i32 %"67")
  store i32 %"74", ptr addrspace(5) %"49", align 4
  %"68" = load i64, ptr addrspace(5) %"48", align 8
  %"69" = load i32, ptr addrspace(5) %"49", align 4
  %"77" = inttoptr i64 %"68" to ptr
  store i32 %"69", ptr %"77", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }