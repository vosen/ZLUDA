@constparams = addrspace(4) global [4 x i16] [i16 10, i16 20, i16 30, i16 40], align 8

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @const(ptr addrspace(4) byref(i64) %"50", ptr addrspace(4) byref(i64) %"51") #0 {
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i16, align 2, addrspace(5)
  %"55" = alloca i16, align 2, addrspace(5)
  %"56" = alloca i16, align 2, addrspace(5)
  %"57" = alloca i16, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  %"58" = load i64, ptr addrspace(4) %"50", align 4
  store i64 %"58", ptr addrspace(5) %"52", align 4
  %"59" = load i64, ptr addrspace(4) %"51", align 4
  store i64 %"59", ptr addrspace(5) %"53", align 4
  %"60" = load i16, ptr addrspace(4) @constparams, align 2
  store i16 %"60", ptr addrspace(5) %"54", align 2
  %"61" = load i16, ptr addrspace(4) getelementptr inbounds (i8, ptr addrspace(4) @constparams, i64 2), align 2
  store i16 %"61", ptr addrspace(5) %"55", align 2
  %"62" = load i16, ptr addrspace(4) getelementptr inbounds (i8, ptr addrspace(4) @constparams, i64 4), align 2
  store i16 %"62", ptr addrspace(5) %"56", align 2
  %"63" = load i16, ptr addrspace(4) getelementptr inbounds (i8, ptr addrspace(4) @constparams, i64 6), align 2
  store i16 %"63", ptr addrspace(5) %"57", align 2
  %"64" = load i64, ptr addrspace(5) %"53", align 4
  %"65" = load i16, ptr addrspace(5) %"54", align 2
  %"76" = inttoptr i64 %"64" to ptr
  store i16 %"65", ptr %"76", align 2
  %"66" = load i64, ptr addrspace(5) %"53", align 4
  %"78" = inttoptr i64 %"66" to ptr
  %"39" = getelementptr inbounds i8, ptr %"78", i64 2
  %"67" = load i16, ptr addrspace(5) %"55", align 2
  store i16 %"67", ptr %"39", align 2
  %"68" = load i64, ptr addrspace(5) %"53", align 4
  %"80" = inttoptr i64 %"68" to ptr
  %"41" = getelementptr inbounds i8, ptr %"80", i64 4
  %"69" = load i16, ptr addrspace(5) %"56", align 2
  store i16 %"69", ptr %"41", align 2
  %"70" = load i64, ptr addrspace(5) %"53", align 4
  %"82" = inttoptr i64 %"70" to ptr
  %"43" = getelementptr inbounds i8, ptr %"82", i64 6
  %"71" = load i16, ptr addrspace(5) %"57", align 2
  store i16 %"71", ptr %"43", align 2
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "uniform-work-group-size"="true" }
