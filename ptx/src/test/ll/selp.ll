declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @selp(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #1 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i16, align 2, addrspace(5)
  %"44" = alloca i16, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"59"

"59":                                             ; preds = %1
  %"45" = load i64, ptr addrspace(4) %"39", align 4
  store i64 %"45", ptr addrspace(5) %"41", align 4
  %"46" = load i64, ptr addrspace(4) %"40", align 4
  store i64 %"46", ptr addrspace(5) %"42", align 4
  %"48" = load i64, ptr addrspace(5) %"41", align 4
  %"56" = inttoptr i64 %"48" to ptr
  %"47" = load i16, ptr %"56", align 2
  store i16 %"47", ptr addrspace(5) %"43", align 2
  %"49" = load i64, ptr addrspace(5) %"41", align 4
  %"57" = inttoptr i64 %"49" to ptr
  %"31" = getelementptr inbounds i8, ptr %"57", i64 2
  %"50" = load i16, ptr %"31", align 2
  store i16 %"50", ptr addrspace(5) %"44", align 2
  %"52" = load i16, ptr addrspace(5) %"43", align 2
  %"53" = load i16, ptr addrspace(5) %"44", align 2
  %"51" = select i1 false, i16 %"52", i16 %"53"
  store i16 %"51", ptr addrspace(5) %"43", align 2
  %"54" = load i64, ptr addrspace(5) %"42", align 4
  %"55" = load i16, ptr addrspace(5) %"43", align 2
  %"58" = inttoptr i64 %"54" to ptr
  store i16 %"55", ptr %"58", align 2
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }