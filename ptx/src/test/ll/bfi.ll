declare i32 @__zluda_ptx_impl_bfi_b32(i32, i32, i32, i32) #0

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @bfi(ptr addrspace(4) byref(i64) %"43", ptr addrspace(4) byref(i64) %"44") #0 {
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i32, align 4, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  %"49" = alloca i32, align 4, addrspace(5)
  %"50" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"51" = load i64, ptr addrspace(4) %"43", align 4
  store i64 %"51", ptr addrspace(5) %"45", align 4
  %"52" = load i64, ptr addrspace(4) %"44", align 4
  store i64 %"52", ptr addrspace(5) %"46", align 4
  %"54" = load i64, ptr addrspace(5) %"45", align 4
  %"68" = inttoptr i64 %"54" to ptr
  %"53" = load i32, ptr %"68", align 4
  store i32 %"53", ptr addrspace(5) %"47", align 4
  %"55" = load i64, ptr addrspace(5) %"45", align 4
  %"69" = inttoptr i64 %"55" to ptr
  %"32" = getelementptr inbounds i8, ptr %"69", i64 4
  %"56" = load i32, ptr %"32", align 4
  store i32 %"56", ptr addrspace(5) %"48", align 4
  %"57" = load i64, ptr addrspace(5) %"45", align 4
  %"70" = inttoptr i64 %"57" to ptr
  %"34" = getelementptr inbounds i8, ptr %"70", i64 8
  %"58" = load i32, ptr %"34", align 4
  store i32 %"58", ptr addrspace(5) %"49", align 4
  %"59" = load i64, ptr addrspace(5) %"45", align 4
  %"71" = inttoptr i64 %"59" to ptr
  %"36" = getelementptr inbounds i8, ptr %"71", i64 12
  %"60" = load i32, ptr %"36", align 4
  store i32 %"60", ptr addrspace(5) %"50", align 4
  %"62" = load i32, ptr addrspace(5) %"47", align 4
  %"63" = load i32, ptr addrspace(5) %"48", align 4
  %"64" = load i32, ptr addrspace(5) %"49", align 4
  %"65" = load i32, ptr addrspace(5) %"50", align 4
  %"72" = call i32 @__zluda_ptx_impl_bfi_b32(i32 %"62", i32 %"63", i32 %"64", i32 %"65")
  store i32 %"72", ptr addrspace(5) %"47", align 4
  %"66" = load i64, ptr addrspace(5) %"46", align 4
  %"67" = load i32, ptr addrspace(5) %"47", align 4
  %"75" = inttoptr i64 %"66" to ptr
  store i32 %"67", ptr %"75", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
