%struct.f32.f32.f32.i8 = type { float, float, float, i8 }

@0 = addrspace(4) global i64 4
@1 = addrspace(4) global i64 4

declare hidden %struct.f32.f32.f32.i8 @__zluda_ptx_impl_div_f32_part1(float, float) #0

declare hidden float @__zluda_ptx_impl_div_f32_part2(float, float, float, float, float, i8) #0

define amdgpu_kernel void @div_ftz(ptr addrspace(4) byref(i64) %"68", ptr addrspace(4) byref(i64) %"69") #1 {
  %"70" = alloca i64, align 8, addrspace(5)
  %"71" = alloca i64, align 8, addrspace(5)
  %"72" = alloca float, align 4, addrspace(5)
  %"73" = alloca float, align 4, addrspace(5)
  %"74" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"59"

"59":                                             ; preds = %1
  %"75" = load i64, ptr addrspace(4) %"68", align 8
  store i64 %"75", ptr addrspace(5) %"70", align 8
  %"76" = load i64, ptr addrspace(4) %"69", align 8
  store i64 %"76", ptr addrspace(5) %"71", align 8
  %"78" = load i64, ptr addrspace(5) %"70", align 8
  %"93" = inttoptr i64 %"78" to ptr
  %"77" = load float, ptr %"93", align 4
  store float %"77", ptr addrspace(5) %"72", align 4
  %"35" = load i64, ptr addrspace(4) @0, align 8
  %"79" = load i64, ptr addrspace(5) %"70", align 8
  %"94" = inttoptr i64 %"79" to ptr
  %"36" = getelementptr inbounds i8, ptr %"94", i64 %"35"
  %"80" = load float, ptr %"36", align 4
  store float %"80", ptr addrspace(5) %"73", align 4
  %"82" = load float, ptr addrspace(5) %"72", align 4
  %"83" = load float, ptr addrspace(5) %"73", align 4
  %"81" = fmul float %"82", %"83"
  store float %"81", ptr addrspace(5) %"74", align 4
  %"84" = load float, ptr addrspace(5) %"72", align 4
  %"85" = load float, ptr addrspace(5) %"73", align 4
  %2 = call %struct.f32.f32.f32.i8 @__zluda_ptx_impl_div_f32_part1(float %"84", float %"85")
  %"42" = extractvalue %struct.f32.f32.f32.i8 %2, 0
  %"43" = extractvalue %struct.f32.f32.f32.i8 %2, 1
  %"44" = extractvalue %struct.f32.f32.f32.i8 %2, 2
  %"45" = extractvalue %struct.f32.f32.f32.i8 %2, 3
  br label %"62"

"62":                                             ; preds = %"59"
  call void @llvm.amdgcn.s.setreg(i32 6401, i32 0)
  br label %"60"

"60":                                             ; preds = %"62"
  %"87" = load float, ptr addrspace(5) %"72", align 4
  %"88" = load float, ptr addrspace(5) %"73", align 4
  %"86" = call float @__zluda_ptx_impl_div_f32_part2(float %"87", float %"88", float %"42", float %"43", float %"44", i8 %"45")
  store float %"86", ptr addrspace(5) %"72", align 4
  br label %"61"

"61":                                             ; preds = %"60"
  %"89" = load i64, ptr addrspace(5) %"71", align 8
  %"90" = load float, ptr addrspace(5) %"72", align 4
  %"95" = inttoptr i64 %"89" to ptr
  store float %"90", ptr %"95", align 4
  %"38" = load i64, ptr addrspace(4) @1, align 8
  %"91" = load i64, ptr addrspace(5) %"71", align 8
  %"96" = inttoptr i64 %"91" to ptr
  %"39" = getelementptr inbounds i8, ptr %"96", i64 %"38"
  %"92" = load float, ptr addrspace(5) %"74", align 4
  store float %"92", ptr %"39", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.amdgcn.s.setreg(i32 immarg, i32) #2

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind willreturn }