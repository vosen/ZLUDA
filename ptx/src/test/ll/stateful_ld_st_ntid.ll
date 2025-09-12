declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @stateful_ld_st_ntid(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #1 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i32, align 4, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"36"

"36":                                             ; preds = %1
  %"65" = load i64, ptr addrspace(4) %"39", align 8
  store i64 %"65", ptr addrspace(5) %"41", align 8
  %"66" = load i64, ptr addrspace(4) %"40", align 8
  store i64 %"66", ptr addrspace(5) %"42", align 8
  %"49" = load i64, ptr addrspace(5) %"41", align 8
  %2 = inttoptr i64 %"49" to ptr
  %"48" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"48", ptr addrspace(5) %"41", align 8
  %"51" = load i64, ptr addrspace(5) %"42", align 8
  %3 = inttoptr i64 %"51" to ptr
  %"50" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"50", ptr addrspace(5) %"42", align 8
  %"35" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"37"

"37":                                             ; preds = %"36"
  store i32 %"35", ptr addrspace(5) %"43", align 4
  %"54" = load i32, ptr addrspace(5) %"43", align 4
  %"53" = zext i32 %"54" to i64
  store i64 %"53", ptr addrspace(5) %"44", align 8
  %"56" = load i64, ptr addrspace(5) %"41", align 8
  %"57" = load i64, ptr addrspace(5) %"44", align 8
  %"67" = add i64 %"56", %"57"
  store i64 %"67", ptr addrspace(5) %"41", align 8
  %"59" = load i64, ptr addrspace(5) %"42", align 8
  %"60" = load i64, ptr addrspace(5) %"44", align 8
  %"69" = add i64 %"59", %"60"
  store i64 %"69", ptr addrspace(5) %"42", align 8
  %"62" = load i64, ptr addrspace(5) %"41", align 8
  %"71" = inttoptr i64 %"62" to ptr addrspace(1)
  %"61" = load i64, ptr addrspace(1) %"71", align 8
  store i64 %"61", ptr addrspace(5) %"45", align 8
  %"63" = load i64, ptr addrspace(5) %"42", align 8
  %"64" = load i64, ptr addrspace(5) %"45", align 8
  %"72" = inttoptr i64 %"63" to ptr addrspace(1)
  store i64 %"64", ptr addrspace(1) %"72", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }