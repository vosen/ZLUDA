@constparams = addrspace(4) global [4 x i16] [i16 10, i16 20, i16 30, i16 40], align 8

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @const(ptr addrspace(4) byref(i64) %"52", ptr addrspace(4) byref(i64) %"53") #1 {
  %"54" = alloca i64, align 8, addrspace(5)
  %"55" = alloca i64, align 8, addrspace(5)
  %"56" = alloca i16, align 2, addrspace(5)
  %"57" = alloca i16, align 2, addrspace(5)
  %"58" = alloca i16, align 2, addrspace(5)
  %"59" = alloca i16, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"45"

"45":                                             ; preds = %1
  %"60" = load i64, ptr addrspace(4) %"52", align 4
  store i64 %"60", ptr addrspace(5) %"54", align 4
  %"61" = load i64, ptr addrspace(4) %"53", align 4
  store i64 %"61", ptr addrspace(5) %"55", align 4
  %"62" = load i16, ptr addrspace(4) @constparams, align 2
  store i16 %"62", ptr addrspace(5) %"56", align 2
  %"63" = load i16, ptr addrspace(4) getelementptr inbounds (i8, ptr addrspace(4) @constparams, i64 2), align 2
  store i16 %"63", ptr addrspace(5) %"57", align 2
  %"64" = load i16, ptr addrspace(4) getelementptr inbounds (i8, ptr addrspace(4) @constparams, i64 4), align 2
  store i16 %"64", ptr addrspace(5) %"58", align 2
  %"65" = load i16, ptr addrspace(4) getelementptr inbounds (i8, ptr addrspace(4) @constparams, i64 6), align 2
  store i16 %"65", ptr addrspace(5) %"59", align 2
  %"66" = load i64, ptr addrspace(5) %"55", align 4
  %"67" = load i16, ptr addrspace(5) %"56", align 2
  %"78" = inttoptr i64 %"66" to ptr
  store i16 %"67", ptr %"78", align 2
  %"68" = load i64, ptr addrspace(5) %"55", align 4
  %"80" = inttoptr i64 %"68" to ptr
  %"40" = getelementptr inbounds i8, ptr %"80", i64 2
  %"69" = load i16, ptr addrspace(5) %"57", align 2
  store i16 %"69", ptr %"40", align 2
  %"70" = load i64, ptr addrspace(5) %"55", align 4
  %"82" = inttoptr i64 %"70" to ptr
  %"42" = getelementptr inbounds i8, ptr %"82", i64 4
  %"71" = load i16, ptr addrspace(5) %"58", align 2
  store i16 %"71", ptr %"42", align 2
  %"72" = load i64, ptr addrspace(5) %"55", align 4
  %"84" = inttoptr i64 %"72" to ptr
  %"44" = getelementptr inbounds i8, ptr %"84", i64 6
  %"73" = load i16, ptr addrspace(5) %"59", align 2
  store i16 %"73", ptr %"44", align 2
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }