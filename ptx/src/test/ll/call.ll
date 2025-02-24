declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define i64 @incr(i64 %"43") #0 {
  %"66" = alloca i64, align 8, addrspace(5)
  %"67" = alloca i64, align 8, addrspace(5)
  %"68" = alloca i64, align 8, addrspace(5)
  %"69" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"80"

"80":                                             ; preds = %1
  store i64 %"43", ptr addrspace(5) %"68", align 4
  %"70" = load i64, ptr addrspace(5) %"68", align 4
  store i64 %"70", ptr addrspace(5) %"69", align 4
  %"72" = load i64, ptr addrspace(5) %"69", align 4
  %"71" = add i64 %"72", 1
  store i64 %"71", ptr addrspace(5) %"69", align 4
  %"73" = load i64, ptr addrspace(5) %"69", align 4
  store i64 %"73", ptr addrspace(5) %"67", align 4
  %"74" = load i64, ptr addrspace(5) %"67", align 4
  store i64 %"74", ptr addrspace(5) %"66", align 4
  %2 = load i64, ptr addrspace(5) %"66", align 4
  ret i64 %2
}

define amdgpu_kernel void @call(ptr addrspace(4) byref(i64) %"51", ptr addrspace(4) byref(i64) %"52") #1 {
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  %"55" = alloca i64, align 8, addrspace(5)
  %"60" = alloca i64, align 8, addrspace(5)
  %"61" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"79"

"79":                                             ; preds = %1
  %"56" = load i64, ptr addrspace(4) %"51", align 4
  store i64 %"56", ptr addrspace(5) %"53", align 4
  %"57" = load i64, ptr addrspace(4) %"52", align 4
  store i64 %"57", ptr addrspace(5) %"54", align 4
  %"59" = load i64, ptr addrspace(5) %"53", align 4
  %"75" = inttoptr i64 %"59" to ptr addrspace(1)
  %"58" = load i64, ptr addrspace(1) %"75", align 4
  store i64 %"58", ptr addrspace(5) %"55", align 4
  %"62" = load i64, ptr addrspace(5) %"55", align 4
  store i64 %"62", ptr addrspace(5) %"60", align 4
  %"40" = load i64, ptr addrspace(5) %"60", align 4
  %"41" = call i64 @incr(i64 %"40")
  store i64 %"41", ptr addrspace(5) %"61", align 4
  %"63" = load i64, ptr addrspace(5) %"61", align 4
  store i64 %"63", ptr addrspace(5) %"55", align 4
  %"64" = load i64, ptr addrspace(5) %"54", align 4
  %"65" = load i64, ptr addrspace(5) %"55", align 4
  %"78" = inttoptr i64 %"64" to ptr addrspace(1)
  store i64 %"65", ptr addrspace(1) %"78", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }