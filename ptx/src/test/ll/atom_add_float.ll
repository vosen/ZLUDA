@shared_mem = external addrspace(3) global [1024 x i8], align 4

declare i32 @__zluda_ptx_impl_sreg_tid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ntid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_ctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_nctaid(i8) #0

declare i32 @__zluda_ptx_impl_sreg_clock() #0

declare i32 @__zluda_ptx_impl_sreg_lanemask_lt() #0

define amdgpu_kernel void @atom_add_float(ptr addrspace(4) byref(i64) %"41", ptr addrspace(4) byref(i64) %"42") #1 {
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca float, align 4, addrspace(5)
  %"46" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"68"

"68":                                             ; preds = %1
  %"47" = load i64, ptr addrspace(4) %"41", align 4
  store i64 %"47", ptr addrspace(5) %"43", align 4
  %"48" = load i64, ptr addrspace(4) %"42", align 4
  store i64 %"48", ptr addrspace(5) %"44", align 4
  %"50" = load i64, ptr addrspace(5) %"43", align 4
  %"61" = inttoptr i64 %"50" to ptr
  %"49" = load float, ptr %"61", align 4
  store float %"49", ptr addrspace(5) %"45", align 4
  %"51" = load i64, ptr addrspace(5) %"43", align 4
  %"62" = inttoptr i64 %"51" to ptr
  %"32" = getelementptr inbounds i8, ptr %"62", i64 4
  %"52" = load float, ptr %"32", align 4
  store float %"52", ptr addrspace(5) %"46", align 4
  %"53" = load float, ptr addrspace(5) %"45", align 4
  store float %"53", ptr addrspace(3) @shared_mem, align 4
  %"55" = load float, ptr addrspace(5) %"46", align 4
  %2 = atomicrmw fadd ptr addrspace(3) @shared_mem, float %"55" syncscope("agent-one-as") monotonic, align 4
  store float %2, ptr addrspace(5) %"45", align 4
  %"56" = load float, ptr addrspace(3) @shared_mem, align 4
  store float %"56", ptr addrspace(5) %"46", align 4
  %"57" = load i64, ptr addrspace(5) %"44", align 4
  %"58" = load float, ptr addrspace(5) %"45", align 4
  %"66" = inttoptr i64 %"57" to ptr
  store float %"58", ptr %"66", align 4
  %"59" = load i64, ptr addrspace(5) %"44", align 4
  %"67" = inttoptr i64 %"59" to ptr
  %"34" = getelementptr inbounds i8, ptr %"67", i64 4
  %"60" = load float, ptr addrspace(5) %"46", align 4
  store float %"60", ptr %"34", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }