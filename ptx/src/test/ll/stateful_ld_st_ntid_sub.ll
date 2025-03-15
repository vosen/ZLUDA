declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @stateful_ld_st_ntid_sub(ptr addrspace(4) byref(i64) %"49", ptr addrspace(4) byref(i64) %"50") #1 {
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  %"55" = alloca i64, align 8, addrspace(5)
  %"56" = alloca i64, align 8, addrspace(5)
  %"57" = alloca i32, align 4, addrspace(5)
  %"58" = alloca i64, align 8, addrspace(5)
  %"59" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"41"

"41":                                             ; preds = %1
  %"79" = load i64, ptr addrspace(4) %"49", align 4
  store i64 %"79", ptr addrspace(5) %"51", align 4
  %"80" = load i64, ptr addrspace(4) %"50", align 4
  store i64 %"80", ptr addrspace(5) %"54", align 4
  %"63" = load i64, ptr addrspace(5) %"51", align 4
  %2 = inttoptr i64 %"63" to ptr
  %"62" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"62", ptr addrspace(5) %"52", align 8
  %"65" = load i64, ptr addrspace(5) %"54", align 4
  %3 = inttoptr i64 %"65" to ptr
  %"64" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"64", ptr addrspace(5) %"55", align 8
  %"36" = call i32 @__zluda_ptx_impl_sreg_tid(i8 0)
  br label %"42"

"42":                                             ; preds = %"41"
  store i32 %"36", ptr addrspace(5) %"57", align 4
  %"68" = load i32, ptr addrspace(5) %"57", align 4
  %"67" = zext i32 %"68" to i64
  store i64 %"67", ptr addrspace(5) %"58", align 4
  %"70" = load i64, ptr addrspace(5) %"52", align 4
  %"71" = load i64, ptr addrspace(5) %"58", align 4
  %"81" = sub i64 %"70", %"71"
  store i64 %"81", ptr addrspace(5) %"53", align 4
  %"73" = load i64, ptr addrspace(5) %"55", align 4
  %"74" = load i64, ptr addrspace(5) %"58", align 4
  %"84" = sub i64 %"73", %"74"
  store i64 %"84", ptr addrspace(5) %"56", align 4
  %"75" = load i64, ptr addrspace(5) %"53", align 4
  %"87" = inttoptr i64 %"75" to ptr addrspace(1)
  %"38" = getelementptr inbounds i8, ptr addrspace(1) %"87", i64 0
  %"76" = load i64, ptr addrspace(1) %"38", align 4
  store i64 %"76", ptr addrspace(5) %"59", align 4
  %"77" = load i64, ptr addrspace(5) %"56", align 4
  %"88" = inttoptr i64 %"77" to ptr addrspace(1)
  %"40" = getelementptr inbounds i8, ptr addrspace(1) %"88", i64 0
  %"78" = load i64, ptr addrspace(5) %"59", align 4
  store i64 %"78", ptr addrspace(1) %"40", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }