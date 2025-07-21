define float @add_rm(float %"79", float %"80") #0 {
  %"128" = alloca float, align 4, addrspace(5)
  %"129" = alloca float, align 4, addrspace(5)
  %"130" = alloca float, align 4, addrspace(5)
  %"131" = alloca float, align 4, addrspace(5)
  %"132" = alloca float, align 4, addrspace(5)
  %"133" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"89"

"89":                                             ; preds = %1
  call void @llvm.amdgcn.s.setreg(i32 6145, i32 2)
  br label %"87"

"87":                                             ; preds = %"89"
  store float %"79", ptr addrspace(5) %"130", align 4
  store float %"80", ptr addrspace(5) %"131", align 4
  %"134" = load float, ptr addrspace(5) %"130", align 4
  store float %"134", ptr addrspace(5) %"132", align 4
  %"135" = load float, ptr addrspace(5) %"131", align 4
  store float %"135", ptr addrspace(5) %"133", align 4
  %"137" = load float, ptr addrspace(5) %"132", align 4
  %"138" = load float, ptr addrspace(5) %"133", align 4
  %"136" = fadd float %"137", %"138"
  store float %"136", ptr addrspace(5) %"132", align 4
  %"139" = load float, ptr addrspace(5) %"132", align 4
  store float %"139", ptr addrspace(5) %"129", align 4
  %"140" = load float, ptr addrspace(5) %"129", align 4
  store float %"140", ptr addrspace(5) %"128", align 4
  %2 = load float, ptr addrspace(5) %"128", align 4
  ret float %2
}

define float @add_rp(float %"82", float %"83") #0 {
  %"141" = alloca float, align 4, addrspace(5)
  %"142" = alloca float, align 4, addrspace(5)
  %"143" = alloca float, align 4, addrspace(5)
  %"144" = alloca float, align 4, addrspace(5)
  %"145" = alloca float, align 4, addrspace(5)
  %"146" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"88"

"88":                                             ; preds = %1
  store float %"82", ptr addrspace(5) %"143", align 4
  store float %"83", ptr addrspace(5) %"144", align 4
  %"147" = load float, ptr addrspace(5) %"143", align 4
  store float %"147", ptr addrspace(5) %"145", align 4
  %"148" = load float, ptr addrspace(5) %"144", align 4
  store float %"148", ptr addrspace(5) %"146", align 4
  %"150" = load float, ptr addrspace(5) %"145", align 4
  %"151" = load float, ptr addrspace(5) %"146", align 4
  %"149" = fadd float %"150", %"151"
  store float %"149", ptr addrspace(5) %"145", align 4
  %"152" = load float, ptr addrspace(5) %"145", align 4
  store float %"152", ptr addrspace(5) %"142", align 4
  %"153" = load float, ptr addrspace(5) %"142", align 4
  store float %"153", ptr addrspace(5) %"141", align 4
  %2 = load float, ptr addrspace(5) %"141", align 4
  ret float %2
}

define amdgpu_kernel void @call_rnd(ptr addrspace(4) byref(i64) %"92", ptr addrspace(4) byref(i64) %"93") #1 {
  %"94" = alloca i64, align 8, addrspace(5)
  %"95" = alloca i64, align 8, addrspace(5)
  %"96" = alloca float, align 4, addrspace(5)
  %"97" = alloca float, align 4, addrspace(5)
  %"98" = alloca float, align 4, addrspace(5)
  %"99" = alloca float, align 4, addrspace(5)
  %"100" = alloca float, align 4, addrspace(5)
  %"101" = alloca float, align 4, addrspace(5)
  %"102" = alloca float, align 4, addrspace(5)
  %"103" = alloca float, align 4, addrspace(5)
  %"104" = alloca float, align 4, addrspace(5)
  %"105" = alloca float, align 4, addrspace(5)
  %"106" = alloca float, align 4, addrspace(5)
  %"107" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"84"

"84":                                             ; preds = %1
  call void @llvm.amdgcn.s.setreg(i32 6145, i32 1)
  %"108" = load i64, ptr addrspace(4) %"92", align 4
  store i64 %"108", ptr addrspace(5) %"94", align 4
  %"109" = load i64, ptr addrspace(4) %"93", align 4
  store i64 %"109", ptr addrspace(5) %"95", align 4
  %"111" = load i64, ptr addrspace(5) %"94", align 4
  %"154" = inttoptr i64 %"111" to ptr
  %"110" = load float, ptr %"154", align 4
  store float %"110", ptr addrspace(5) %"96", align 4
  %"112" = load i64, ptr addrspace(5) %"94", align 4
  %"155" = inttoptr i64 %"112" to ptr
  %"59" = getelementptr inbounds i8, ptr %"155", i64 4
  %"113" = load float, ptr %"59", align 4
  store float %"113", ptr addrspace(5) %"97", align 4
  %"114" = load i64, ptr addrspace(5) %"94", align 4
  %"156" = inttoptr i64 %"114" to ptr
  %"61" = getelementptr inbounds i8, ptr %"156", i64 8
  %"115" = load float, ptr %"61", align 4
  store float %"115", ptr addrspace(5) %"98", align 4
  %"116" = load i64, ptr addrspace(5) %"94", align 4
  %"157" = inttoptr i64 %"116" to ptr
  %"63" = getelementptr inbounds i8, ptr %"157", i64 12
  %"117" = load float, ptr %"63", align 4
  store float %"117", ptr addrspace(5) %"99", align 4
  %"118" = load float, ptr addrspace(5) %"96", align 4
  store float %"118", ptr addrspace(5) %"102", align 4
  %"119" = load float, ptr addrspace(5) %"97", align 4
  store float %"119", ptr addrspace(5) %"103", align 4
  %"72" = load float, ptr addrspace(5) %"102", align 4
  %"73" = load float, ptr addrspace(5) %"103", align 4
  %"74" = call float @add_rp(float %"72", float %"73")
  br label %"85"

"85":                                             ; preds = %"84"
  store float %"74", ptr addrspace(5) %"104", align 4
  %"120" = load float, ptr addrspace(5) %"104", align 4
  store float %"120", ptr addrspace(5) %"100", align 4
  %"121" = load i64, ptr addrspace(5) %"95", align 4
  %"122" = load float, ptr addrspace(5) %"100", align 4
  %"158" = inttoptr i64 %"121" to ptr
  store float %"122", ptr %"158", align 4
  %"123" = load float, ptr addrspace(5) %"98", align 4
  store float %"123", ptr addrspace(5) %"105", align 4
  %"124" = load float, ptr addrspace(5) %"99", align 4
  store float %"124", ptr addrspace(5) %"106", align 4
  %"75" = load float, ptr addrspace(5) %"105", align 4
  %"76" = load float, ptr addrspace(5) %"106", align 4
  %"77" = call float @add_rm(float %"75", float %"76")
  br label %"86"

"86":                                             ; preds = %"85"
  store float %"77", ptr addrspace(5) %"107", align 4
  %"125" = load float, ptr addrspace(5) %"107", align 4
  store float %"125", ptr addrspace(5) %"101", align 4
  %"126" = load i64, ptr addrspace(5) %"95", align 4
  %"159" = inttoptr i64 %"126" to ptr
  %"65" = getelementptr inbounds i8, ptr %"159", i64 4
  %"127" = load float, ptr addrspace(5) %"101", align 4
  store float %"127", ptr %"65", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.amdgcn.s.setreg(i32 immarg, i32) #2

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind willreturn }