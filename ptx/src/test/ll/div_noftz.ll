%struct.f32.f32.f32.i8 = type { float, float, float, i8 }

declare hidden %struct.f32.f32.f32.i8 @__zluda_ptx_impl_div_f32_part1(float, float) #0

declare hidden float @__zluda_ptx_impl_div_f32_part2(float, float, float, float, float, i8) #0

define amdgpu_kernel void @div_noftz(ptr addrspace(4) byref(i64) %"65", ptr addrspace(4) byref(i64) %"66") #1 {
  %"67" = alloca i64, align 8, addrspace(5)
  %"68" = alloca i64, align 8, addrspace(5)
  %"69" = alloca float, align 4, addrspace(5)
  %"70" = alloca float, align 4, addrspace(5)
  %"71" = alloca float, align 4, addrspace(5)
  %"76" = alloca i64, align 8, addrspace(5)
  store i64 4, ptr addrspace(5) %"76", align 4
  %"90" = alloca i64, align 8, addrspace(5)
  store i64 4, ptr addrspace(5) %"90", align 4
  br label %1

1:                                                ; preds = %0
  br label %"57"

"57":                                             ; preds = %1
  %"72" = load i64, ptr addrspace(4) %"65", align 8
  store i64 %"72", ptr addrspace(5) %"67", align 8
  %"73" = load i64, ptr addrspace(4) %"66", align 8
  store i64 %"73", ptr addrspace(5) %"68", align 8
  %"75" = load i64, ptr addrspace(5) %"67", align 8
  %"94" = inttoptr i64 %"75" to ptr
  %"74" = load float, ptr %"94", align 4
  store float %"74", ptr addrspace(5) %"69", align 4
  %"77" = load i64, ptr addrspace(5) %"67", align 8
  %"78" = load i64, ptr addrspace(5) %"76", align 8
  %"95" = inttoptr i64 %"77" to ptr
  %"35" = getelementptr inbounds i8, ptr %"95", i64 %"78"
  %"79" = load float, ptr %"35", align 4
  store float %"79", ptr addrspace(5) %"70", align 4
  %"81" = load float, ptr addrspace(5) %"69", align 4
  %"82" = load float, ptr addrspace(5) %"70", align 4
  %"80" = fmul float %"81", %"82"
  store float %"80", ptr addrspace(5) %"71", align 4
  call void @llvm.amdgcn.s.setreg(i32 6401, i32 3)
  %"83" = load float, ptr addrspace(5) %"69", align 4
  %"84" = load float, ptr addrspace(5) %"70", align 4
  %2 = call %struct.f32.f32.f32.i8 @__zluda_ptx_impl_div_f32_part1(float %"83", float %"84")
  %"40" = extractvalue %struct.f32.f32.f32.i8 %2, 0
  %"41" = extractvalue %struct.f32.f32.f32.i8 %2, 1
  %"42" = extractvalue %struct.f32.f32.f32.i8 %2, 2
  %"43" = extractvalue %struct.f32.f32.f32.i8 %2, 3
  br label %"58"

"58":                                             ; preds = %"57"
  %"86" = load float, ptr addrspace(5) %"69", align 4
  %"87" = load float, ptr addrspace(5) %"70", align 4
  %"85" = call float @__zluda_ptx_impl_div_f32_part2(float %"86", float %"87", float %"40", float %"41", float %"42", i8 %"43")
  store float %"85", ptr addrspace(5) %"69", align 4
  br label %"59"

"59":                                             ; preds = %"58"
  %"88" = load i64, ptr addrspace(5) %"68", align 8
  %"89" = load float, ptr addrspace(5) %"69", align 4
  %"96" = inttoptr i64 %"88" to ptr
  store float %"89", ptr %"96", align 4
  %"91" = load i64, ptr addrspace(5) %"68", align 8
  %"92" = load i64, ptr addrspace(5) %"90", align 8
  %"97" = inttoptr i64 %"91" to ptr
  %"37" = getelementptr inbounds i8, ptr %"97", i64 %"92"
  %"93" = load float, ptr addrspace(5) %"71", align 4
  store float %"93", ptr %"37", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.amdgcn.s.setreg(i32 immarg, i32) #2

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind willreturn }