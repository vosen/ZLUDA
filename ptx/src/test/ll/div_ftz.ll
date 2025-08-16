%struct.f32.f32.f32.i8 = type { float, float, float, i8 }

declare hidden %struct.f32.f32.f32.i8 @__zluda_ptx_impl_div_f32_part1(float, float) #0

declare hidden float @__zluda_ptx_impl_div_f32_part2(float, float, float, float, float, i8) #0

define amdgpu_kernel void @div_ftz(ptr addrspace(4) byref(i64) %"63", ptr addrspace(4) byref(i64) %"64") #1 {
  %"65" = alloca i64, align 8, addrspace(5)
  %"66" = alloca i64, align 8, addrspace(5)
  %"67" = alloca float, align 4, addrspace(5)
  %"68" = alloca float, align 4, addrspace(5)
  %"69" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"54"

"54":                                             ; preds = %1
  %"70" = load i64, ptr addrspace(4) %"63", align 8
  store i64 %"70", ptr addrspace(5) %"65", align 8
  %"71" = load i64, ptr addrspace(4) %"64", align 8
  store i64 %"71", ptr addrspace(5) %"66", align 8
  %"73" = load i64, ptr addrspace(5) %"65", align 8
  %"88" = inttoptr i64 %"73" to ptr
  %"72" = load float, ptr %"88", align 4
  store float %"72", ptr addrspace(5) %"67", align 4
  %"74" = load i64, ptr addrspace(5) %"65", align 8
  %"89" = inttoptr i64 %"74" to ptr
  %"32" = getelementptr inbounds i8, ptr %"89", i64 4
  %"75" = load float, ptr %"32", align 4
  store float %"75", ptr addrspace(5) %"68", align 4
  %"77" = load float, ptr addrspace(5) %"67", align 4
  %"78" = load float, ptr addrspace(5) %"68", align 4
  %"76" = fmul float %"77", %"78"
  store float %"76", ptr addrspace(5) %"69", align 4
  %"79" = load float, ptr addrspace(5) %"67", align 4
  %"80" = load float, ptr addrspace(5) %"68", align 4
  %2 = call %struct.f32.f32.f32.i8 @__zluda_ptx_impl_div_f32_part1(float %"79", float %"80")
  %"37" = extractvalue %struct.f32.f32.f32.i8 %2, 0
  %"38" = extractvalue %struct.f32.f32.f32.i8 %2, 1
  %"39" = extractvalue %struct.f32.f32.f32.i8 %2, 2
  %"40" = extractvalue %struct.f32.f32.f32.i8 %2, 3
  br label %"57"

"57":                                             ; preds = %"54"
  call void @llvm.amdgcn.s.setreg(i32 6401, i32 0)
  br label %"55"

"55":                                             ; preds = %"57"
  %"82" = load float, ptr addrspace(5) %"67", align 4
  %"83" = load float, ptr addrspace(5) %"68", align 4
  %"81" = call float @__zluda_ptx_impl_div_f32_part2(float %"82", float %"83", float %"37", float %"38", float %"39", i8 %"40")
  store float %"81", ptr addrspace(5) %"67", align 4
  br label %"56"

"56":                                             ; preds = %"55"
  %"84" = load i64, ptr addrspace(5) %"66", align 8
  %"85" = load float, ptr addrspace(5) %"67", align 4
  %"90" = inttoptr i64 %"84" to ptr
  store float %"85", ptr %"90", align 4
  %"86" = load i64, ptr addrspace(5) %"66", align 8
  %"91" = inttoptr i64 %"86" to ptr
  %"34" = getelementptr inbounds i8, ptr %"91", i64 4
  %"87" = load float, ptr addrspace(5) %"69", align 4
  store float %"87", ptr %"34", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.amdgcn.s.setreg(i32 immarg, i32) #2

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind willreturn }