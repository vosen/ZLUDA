declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @mad_s32(ptr addrspace(4) byref(i64) %"45", ptr addrspace(4) byref(i64) %"46") #0 {
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i32, align 4, addrspace(5)
  %"50" = alloca i32, align 4, addrspace(5)
  %"51" = alloca i32, align 4, addrspace(5)
  %"52" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"53" = load i64, ptr addrspace(4) %"45", align 4
  store i64 %"53", ptr addrspace(5) %"47", align 4
  %"54" = load i64, ptr addrspace(4) %"46", align 4
  store i64 %"54", ptr addrspace(5) %"48", align 4
  %"56" = load i64, ptr addrspace(5) %"47", align 4
  %"71" = inttoptr i64 %"56" to ptr
  %"55" = load i32, ptr %"71", align 4
  store i32 %"55", ptr addrspace(5) %"50", align 4
  %"57" = load i64, ptr addrspace(5) %"47", align 4
  %"72" = inttoptr i64 %"57" to ptr
  %"32" = getelementptr inbounds i8, ptr %"72", i64 4
  %"58" = load i32, ptr %"32", align 4
  store i32 %"58", ptr addrspace(5) %"51", align 4
  %"59" = load i64, ptr addrspace(5) %"47", align 4
  %"73" = inttoptr i64 %"59" to ptr
  %"34" = getelementptr inbounds i8, ptr %"73", i64 8
  %"60" = load i32, ptr %"34", align 4
  store i32 %"60", ptr addrspace(5) %"52", align 4
  %"62" = load i32, ptr addrspace(5) %"50", align 4
  %"63" = load i32, ptr addrspace(5) %"51", align 4
  %"64" = load i32, ptr addrspace(5) %"52", align 4
  %2 = mul i32 %"62", %"63"
  %"61" = add i32 %2, %"64"
  store i32 %"61", ptr addrspace(5) %"49", align 4
  %"65" = load i64, ptr addrspace(5) %"48", align 4
  %"66" = load i32, ptr addrspace(5) %"49", align 4
  %"74" = inttoptr i64 %"65" to ptr
  store i32 %"66", ptr %"74", align 4
  %"67" = load i64, ptr addrspace(5) %"48", align 4
  %"75" = inttoptr i64 %"67" to ptr
  %"36" = getelementptr inbounds i8, ptr %"75", i64 4
  %"68" = load i32, ptr addrspace(5) %"49", align 4
  store i32 %"68", ptr %"36", align 4
  %"69" = load i64, ptr addrspace(5) %"48", align 4
  %"76" = inttoptr i64 %"69" to ptr
  %"38" = getelementptr inbounds i8, ptr %"76", i64 8
  %"70" = load i32, ptr addrspace(5) %"49", align 4
  store i32 %"70", ptr %"38", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
