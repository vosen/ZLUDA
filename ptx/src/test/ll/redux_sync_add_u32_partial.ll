declare hidden i32 @__zluda_ptx_impl_redux_sync_add_u32(i32, i32) #0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @redux_sync_add_u32_partial(ptr addrspace(4) byref(i64) %"46") #1 {
  %"47" = alloca i32, align 4, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i32, align 4, addrspace(5)
  %"51" = alloca i1, align 1, addrspace(5)
  %"62" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"43"

"43":                                             ; preds = %1
  %"52" = load i64, ptr addrspace(4) %"46", align 8
  store i64 %"52", ptr addrspace(5) %"49", align 8
  %"37" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"44"

"44":                                             ; preds = %"43"
  store i32 %"37", ptr addrspace(5) %"47", align 4
  %"55" = load i32, ptr addrspace(5) %"47", align 4
  %"54" = urem i32 %"55", 2
  store i32 %"54", ptr addrspace(5) %"50", align 4
  %"57" = load i32, ptr addrspace(5) %"50", align 4
  %2 = icmp eq i32 %"57", 0
  store i1 %2, ptr addrspace(5) %"51", align 1
  store i32 0, ptr addrspace(5) %"48", align 4
  %"59" = load i1, ptr addrspace(5) %"51", align 1
  br i1 %"59", label %"16", label %"17"

"16":                                             ; preds = %"44"
  %"61" = load i32, ptr addrspace(5) %"47", align 4
  %"60" = call i32 @__zluda_ptx_impl_redux_sync_add_u32(i32 %"61", i32 1431655765)
  store i32 %"60", ptr addrspace(5) %"48", align 4
  br label %"17"

"17":                                             ; preds = %"16", %"44"
  %"64" = load i32, ptr addrspace(5) %"47", align 4
  %3 = zext i32 %"64" to i64
  %"63" = mul i64 %3, 4
  store i64 %"63", ptr addrspace(5) %"62", align 8
  %"66" = load i64, ptr addrspace(5) %"49", align 8
  %"67" = load i64, ptr addrspace(5) %"62", align 8
  %"65" = add i64 %"66", %"67"
  store i64 %"65", ptr addrspace(5) %"49", align 8
  %"68" = load i64, ptr addrspace(5) %"49", align 8
  %"69" = load i32, ptr addrspace(5) %"48", align 4
  %"70" = inttoptr i64 %"68" to ptr
  store i32 %"69", ptr %"70", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }