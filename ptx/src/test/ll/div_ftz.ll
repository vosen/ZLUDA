declare { float, float, float, i1 } @__zluda_ptx_impl_div_rn_ftz_f32_part1(float, float) #0

declare float @__zluda_ptx_impl_div_rn_ftz_f32_part2(float, float, float, float, float, i1) #0

define amdgpu_kernel void @div_ftz(ptr addrspace(4) byref(i64) %"62", ptr addrspace(4) byref(i64) %"63") #1 {
  %"64" = alloca i64, align 8, addrspace(5)
  %"65" = alloca i64, align 8, addrspace(5)
  %"66" = alloca float, align 4, addrspace(5)
  %"67" = alloca float, align 4, addrspace(5)
  %"68" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"54"

"54":                                             ; preds = %1
  %"69" = load i64, ptr addrspace(4) %"62", align 8
  store i64 %"69", ptr addrspace(5) %"64", align 8
  %"70" = load i64, ptr addrspace(4) %"63", align 8
  store i64 %"70", ptr addrspace(5) %"65", align 8
  %"72" = load i64, ptr addrspace(5) %"64", align 8
  %"87" = inttoptr i64 %"72" to ptr
  %"71" = load float, ptr %"87", align 4
  store float %"71", ptr addrspace(5) %"66", align 4
  %"73" = load i64, ptr addrspace(5) %"64", align 8
  %"88" = inttoptr i64 %"73" to ptr
  %"32" = getelementptr inbounds i8, ptr %"88", i64 4
  %"74" = load float, ptr %"32", align 4
  store float %"74", ptr addrspace(5) %"67", align 4
  %"76" = load float, ptr addrspace(5) %"66", align 4
  %"77" = load float, ptr addrspace(5) %"67", align 4
  %"75" = fmul float %"76", %"77"
  store float %"75", ptr addrspace(5) %"68", align 4
  %"78" = load float, ptr addrspace(5) %"66", align 4
  %"79" = load float, ptr addrspace(5) %"67", align 4
  %2 = call { float, float, float, i1 } @__zluda_ptx_impl_div_rn_ftz_f32_part1(float %"78", float %"79")
  %"37" = extractvalue { float, float, float, i1 } %2, 0
  %"38" = extractvalue { float, float, float, i1 } %2, 1
  %"39" = extractvalue { float, float, float, i1 } %2, 2
  %"40" = extractvalue { float, float, float, i1 } %2, 3
  br label %"55"

"55":                                             ; preds = %"54"
  %"81" = load float, ptr addrspace(5) %"66", align 4
  %"82" = load float, ptr addrspace(5) %"67", align 4
  %"80" = call float @__zluda_ptx_impl_div_rn_ftz_f32_part2(float %"81", float %"82", float %"37", float %"38", float %"39", i1 %"40")
  store float %"80", ptr addrspace(5) %"66", align 4
  br label %"56"

"56":                                             ; preds = %"55"
  %"83" = load i64, ptr addrspace(5) %"65", align 8
  %"84" = load float, ptr addrspace(5) %"66", align 4
  %"89" = inttoptr i64 %"83" to ptr
  store float %"84", ptr %"89", align 4
  %"85" = load i64, ptr addrspace(5) %"65", align 8
  %"90" = inttoptr i64 %"85" to ptr
  %"34" = getelementptr inbounds i8, ptr %"90", i64 4
  %"86" = load float, ptr addrspace(5) %"68", align 4
  store float %"86", ptr %"34", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }