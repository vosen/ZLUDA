%struct.f32.f32.f32.i8 = type { float, float, float, i8 }

declare hidden %struct.f32.f32.f32.i8 @__zluda_ptx_impl_div_f32_part1(float, float) #0

declare hidden float @__zluda_ptx_impl_div_f32_part2(float, float, float, float, float, i8) #0

define amdgpu_kernel void @div_ftz(ptr addrspace(4) byref(i64) %"66", ptr addrspace(4) byref(i64) %"67") #1 {
  %"68" = alloca i64, align 8, addrspace(5)
  %"69" = alloca i64, align 8, addrspace(5)
  %"70" = alloca float, align 4, addrspace(5)
  %"71" = alloca float, align 4, addrspace(5)
  %"72" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"57"

"57":                                             ; preds = %1
  %"73" = load i64, ptr addrspace(4) %"66", align 8
  store i64 %"73", ptr addrspace(5) %"68", align 8
  %"74" = load i64, ptr addrspace(4) %"67", align 8
  store i64 %"74", ptr addrspace(5) %"69", align 8
  %"76" = load i64, ptr addrspace(5) %"68", align 8
  %"91" = inttoptr i64 %"76" to ptr
  %"75" = load float, ptr %"91", align 4
  store float %"75", ptr addrspace(5) %"70", align 4
  %"77" = load i64, ptr addrspace(5) %"68", align 8
  %"92" = inttoptr i64 %"77" to ptr
  %"35" = getelementptr inbounds i8, ptr %"92", i64 4
  %"78" = load float, ptr %"35", align 4
  store float %"78", ptr addrspace(5) %"71", align 4
  %"80" = load float, ptr addrspace(5) %"70", align 4
  %"81" = load float, ptr addrspace(5) %"71", align 4
  %"79" = fmul float %"80", %"81"
  store float %"79", ptr addrspace(5) %"72", align 4
  %"82" = load float, ptr addrspace(5) %"70", align 4
  %"83" = load float, ptr addrspace(5) %"71", align 4
  %2 = call %struct.f32.f32.f32.i8 @__zluda_ptx_impl_div_f32_part1(float %"82", float %"83")
  %"40" = extractvalue %struct.f32.f32.f32.i8 %2, 0
  %"41" = extractvalue %struct.f32.f32.f32.i8 %2, 1
  %"42" = extractvalue %struct.f32.f32.f32.i8 %2, 2
  %"43" = extractvalue %struct.f32.f32.f32.i8 %2, 3
  br label %"60"

"60":                                             ; preds = %"57"
  call void @llvm.amdgcn.s.setreg(i32 6401, i32 0)
  br label %"58"

"58":                                             ; preds = %"60"
  %"85" = load float, ptr addrspace(5) %"70", align 4
  %"86" = load float, ptr addrspace(5) %"71", align 4
  %"84" = call float @__zluda_ptx_impl_div_f32_part2(float %"85", float %"86", float %"40", float %"41", float %"42", i8 %"43")
  store float %"84", ptr addrspace(5) %"70", align 4
  br label %"59"

"59":                                             ; preds = %"58"
  %"87" = load i64, ptr addrspace(5) %"69", align 8
  %"88" = load float, ptr addrspace(5) %"70", align 4
  %"93" = inttoptr i64 %"87" to ptr
  store float %"88", ptr %"93", align 4
  %"89" = load i64, ptr addrspace(5) %"69", align 8
  %"94" = inttoptr i64 %"89" to ptr
  %"37" = getelementptr inbounds i8, ptr %"94", i64 4
  %"90" = load float, ptr addrspace(5) %"72", align 4
  store float %"90", ptr %"37", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.amdgcn.s.setreg(i32 immarg, i32) #2

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind willreturn }
