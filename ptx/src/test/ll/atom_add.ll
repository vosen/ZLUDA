@shared_mem = external addrspace(3) global [1024 x i8], align 4

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @atom_add(ptr addrspace(4) byref(i64) %"42", ptr addrspace(4) byref(i64) %"43") #1 {
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i32, align 4, addrspace(5)
  %"47" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"35"

"35":                                             ; preds = %1
  %"48" = load i64, ptr addrspace(4) %"42", align 4
  store i64 %"48", ptr addrspace(5) %"44", align 4
  %"49" = load i64, ptr addrspace(4) %"43", align 4
  store i64 %"49", ptr addrspace(5) %"45", align 4
  %"51" = load i64, ptr addrspace(5) %"44", align 4
  %"62" = inttoptr i64 %"51" to ptr
  %"50" = load i32, ptr %"62", align 4
  store i32 %"50", ptr addrspace(5) %"46", align 4
  %"52" = load i64, ptr addrspace(5) %"44", align 4
  %"63" = inttoptr i64 %"52" to ptr
  %"32" = getelementptr inbounds i8, ptr %"63", i64 4
  %"53" = load i32, ptr %"32", align 4
  store i32 %"53", ptr addrspace(5) %"47", align 4
  %"54" = load i32, ptr addrspace(5) %"46", align 4
  store i32 %"54", ptr addrspace(3) @shared_mem, align 4
  %"56" = load i32, ptr addrspace(5) %"47", align 4
  %2 = atomicrmw add ptr addrspace(3) @shared_mem, i32 %"56" syncscope("agent-one-as") monotonic, align 4
  store i32 %2, ptr addrspace(5) %"46", align 4
  %"57" = load i32, ptr addrspace(3) @shared_mem, align 4
  store i32 %"57", ptr addrspace(5) %"47", align 4
  %"58" = load i64, ptr addrspace(5) %"45", align 4
  %"59" = load i32, ptr addrspace(5) %"46", align 4
  %"67" = inttoptr i64 %"58" to ptr
  store i32 %"59", ptr %"67", align 4
  %"60" = load i64, ptr addrspace(5) %"45", align 4
  %"68" = inttoptr i64 %"60" to ptr
  %"34" = getelementptr inbounds i8, ptr %"68", i64 4
  %"61" = load i32, ptr addrspace(5) %"47", align 4
  store i32 %"61", ptr %"34", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }