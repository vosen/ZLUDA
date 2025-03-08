@constparams = addrspace(4) global [4 x i16] [i16 10, i16 20, i16 30, i16 40], align 8

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @const(ptr addrspace(4) byref(i64) %"51", ptr addrspace(4) byref(i64) %"52") #1 {
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  %"55" = alloca i16, align 2, addrspace(5)
  %"56" = alloca i16, align 2, addrspace(5)
  %"57" = alloca i16, align 2, addrspace(5)
  %"58" = alloca i16, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"85"

"85":                                             ; preds = %1
  %"59" = load i64, ptr addrspace(4) %"51", align 4
  store i64 %"59", ptr addrspace(5) %"53", align 4
  %"60" = load i64, ptr addrspace(4) %"52", align 4
  store i64 %"60", ptr addrspace(5) %"54", align 4
  %"61" = load i16, ptr addrspace(4) @constparams, align 2
  store i16 %"61", ptr addrspace(5) %"55", align 2
  %"62" = load i16, ptr addrspace(4) getelementptr inbounds (i8, ptr addrspace(4) @constparams, i64 2), align 2
  store i16 %"62", ptr addrspace(5) %"56", align 2
  %"63" = load i16, ptr addrspace(4) getelementptr inbounds (i8, ptr addrspace(4) @constparams, i64 4), align 2
  store i16 %"63", ptr addrspace(5) %"57", align 2
  %"64" = load i16, ptr addrspace(4) getelementptr inbounds (i8, ptr addrspace(4) @constparams, i64 6), align 2
  store i16 %"64", ptr addrspace(5) %"58", align 2
  %"65" = load i64, ptr addrspace(5) %"54", align 4
  %"66" = load i16, ptr addrspace(5) %"55", align 2
  %"77" = inttoptr i64 %"65" to ptr
  store i16 %"66", ptr %"77", align 2
  %"67" = load i64, ptr addrspace(5) %"54", align 4
  %"79" = inttoptr i64 %"67" to ptr
  %"40" = getelementptr inbounds i8, ptr %"79", i64 2
  %"68" = load i16, ptr addrspace(5) %"56", align 2
  store i16 %"68", ptr %"40", align 2
  %"69" = load i64, ptr addrspace(5) %"54", align 4
  %"81" = inttoptr i64 %"69" to ptr
  %"42" = getelementptr inbounds i8, ptr %"81", i64 4
  %"70" = load i16, ptr addrspace(5) %"57", align 2
  store i16 %"70", ptr %"42", align 2
  %"71" = load i64, ptr addrspace(5) %"54", align 4
  %"83" = inttoptr i64 %"71" to ptr
  %"44" = getelementptr inbounds i8, ptr %"83", i64 6
  %"72" = load i16, ptr addrspace(5) %"58", align 2
  store i16 %"72", ptr %"44", align 2
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }