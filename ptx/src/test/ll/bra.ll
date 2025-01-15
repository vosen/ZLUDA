declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @bra(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #0 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"46" = load i64, ptr addrspace(4) %"40", align 4
  store i64 %"46", ptr addrspace(5) %"42", align 4
  %"47" = load i64, ptr addrspace(4) %"41", align 4
  store i64 %"47", ptr addrspace(5) %"43", align 4
  %"49" = load i64, ptr addrspace(5) %"42", align 4
  %"56" = inttoptr i64 %"49" to ptr
  %"48" = load i64, ptr %"56", align 4
  store i64 %"48", ptr addrspace(5) %"44", align 4
  br label %"9"

"9":                                              ; preds = %1
  %"51" = load i64, ptr addrspace(5) %"44", align 4
  %"50" = add i64 %"51", 1
  store i64 %"50", ptr addrspace(5) %"45", align 4
  br label %"11"

"10":                                             ; No predecessors!
  %"53" = load i64, ptr addrspace(5) %"44", align 4
  %"52" = add i64 %"53", 2
  store i64 %"52", ptr addrspace(5) %"45", align 4
  br label %"11"

"11":                                             ; preds = %"10", %"9"
  %"54" = load i64, ptr addrspace(5) %"43", align 4
  %"55" = load i64, ptr addrspace(5) %"45", align 4
  %"57" = inttoptr i64 %"54" to ptr
  store i64 %"55", ptr %"57", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
