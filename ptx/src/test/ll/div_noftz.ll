%struct.f32.f32.f32.i8 = type { float, float, float, i8 }

declare hidden %struct.f32.f32.f32.i8 @__zluda_ptx_impl_div_f32_part1(float, float) #0

declare hidden float @__zluda_ptx_impl_div_f32_part2(float, float, float, float, float, i8) #0

define amdgpu_kernel void @div_noftz(ptr addrspace(4) byref(i64) %"65", ptr addrspace(4) byref(i64) %"66") #1 {
  %"67" = alloca i64, align 8, addrspace(5)
  %"68" = alloca i64, align 8, addrspace(5)
  %"69" = alloca float, align 4, addrspace(5)
  %"70" = alloca float, align 4, addrspace(5)
  %"71" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"57"

"57":                                             ; preds = %1
  %"72" = load i64, ptr addrspace(4) %"65", align 8
  store i64 %"72", ptr addrspace(5) %"67", align 8
  %"73" = load i64, ptr addrspace(4) %"66", align 8
  store i64 %"73", ptr addrspace(5) %"68", align 8
  %"75" = load i64, ptr addrspace(5) %"67", align 8
  %"90" = inttoptr i64 %"75" to ptr
  %"74" = load float, ptr %"90", align 4
  store float %"74", ptr addrspace(5) %"69", align 4
  %"76" = load i64, ptr addrspace(5) %"67", align 8
  %"91" = inttoptr i64 %"76" to ptr
  %"35" = getelementptr inbounds i8, ptr %"91", i64 4
  %"77" = load float, ptr %"35", align 4
  store float %"77", ptr addrspace(5) %"70", align 4
  %"79" = load float, ptr addrspace(5) %"69", align 4
  %"80" = load float, ptr addrspace(5) %"70", align 4
  %"78" = fmul float %"79", %"80"
  store float %"78", ptr addrspace(5) %"71", align 4
  call void @llvm.amdgcn.s.setreg(i32 6401, i32 3)
  %"81" = load float, ptr addrspace(5) %"69", align 4
  %"82" = load float, ptr addrspace(5) %"70", align 4
  %2 = call %struct.f32.f32.f32.i8 @__zluda_ptx_impl_div_f32_part1(float %"81", float %"82")
  %"40" = extractvalue %struct.f32.f32.f32.i8 %2, 0
  %"41" = extractvalue %struct.f32.f32.f32.i8 %2, 1
  %"42" = extractvalue %struct.f32.f32.f32.i8 %2, 2
  %"43" = extractvalue %struct.f32.f32.f32.i8 %2, 3
  br label %"58"

"58":                                             ; preds = %"57"
  %"84" = load float, ptr addrspace(5) %"69", align 4
  %"85" = load float, ptr addrspace(5) %"70", align 4
  %"83" = call float @__zluda_ptx_impl_div_f32_part2(float %"84", float %"85", float %"40", float %"41", float %"42", i8 %"43")
  store float %"83", ptr addrspace(5) %"69", align 4
  br label %"59"

"59":                                             ; preds = %"58"
  %"86" = load i64, ptr addrspace(5) %"68", align 8
  %"87" = load float, ptr addrspace(5) %"69", align 4
  %"92" = inttoptr i64 %"86" to ptr
  store float %"87", ptr %"92", align 4
  %"88" = load i64, ptr addrspace(5) %"68", align 8
  %"93" = inttoptr i64 %"88" to ptr
  %"37" = getelementptr inbounds i8, ptr %"93", i64 4
  %"89" = load float, ptr addrspace(5) %"71", align 4
  store float %"89", ptr %"37", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.amdgcn.s.setreg(i32 immarg, i32) #2

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind willreturn }