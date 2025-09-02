define hidden float @add_rm(float %"82", float %"83") #0 {
  %"131" = alloca float, align 4, addrspace(5)
  %"132" = alloca float, align 4, addrspace(5)
  %"133" = alloca float, align 4, addrspace(5)
  %"134" = alloca float, align 4, addrspace(5)
  %"135" = alloca float, align 4, addrspace(5)
  %"136" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"92"

"92":                                             ; preds = %1
  call void @llvm.amdgcn.s.setreg(i32 6145, i32 2)
  br label %"90"

"90":                                             ; preds = %"92"
  store float %"82", ptr addrspace(5) %"133", align 4
  store float %"83", ptr addrspace(5) %"134", align 4
  %"137" = load float, ptr addrspace(5) %"133", align 4
  store float %"137", ptr addrspace(5) %"135", align 4
  %"138" = load float, ptr addrspace(5) %"134", align 4
  store float %"138", ptr addrspace(5) %"136", align 4
  %"140" = load float, ptr addrspace(5) %"135", align 4
  %"141" = load float, ptr addrspace(5) %"136", align 4
  %"139" = fadd float %"140", %"141"
  store float %"139", ptr addrspace(5) %"135", align 4
  %"142" = load float, ptr addrspace(5) %"135", align 4
  store float %"142", ptr addrspace(5) %"132", align 4
  %"143" = load float, ptr addrspace(5) %"132", align 4
  store float %"143", ptr addrspace(5) %"131", align 4
  %2 = load float, ptr addrspace(5) %"131", align 4
  ret float %2
}

define hidden float @add_rp(float %"85", float %"86") #0 {
  %"144" = alloca float, align 4, addrspace(5)
  %"145" = alloca float, align 4, addrspace(5)
  %"146" = alloca float, align 4, addrspace(5)
  %"147" = alloca float, align 4, addrspace(5)
  %"148" = alloca float, align 4, addrspace(5)
  %"149" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"91"

"91":                                             ; preds = %1
  store float %"85", ptr addrspace(5) %"146", align 4
  store float %"86", ptr addrspace(5) %"147", align 4
  %"150" = load float, ptr addrspace(5) %"146", align 4
  store float %"150", ptr addrspace(5) %"148", align 4
  %"151" = load float, ptr addrspace(5) %"147", align 4
  store float %"151", ptr addrspace(5) %"149", align 4
  %"153" = load float, ptr addrspace(5) %"148", align 4
  %"154" = load float, ptr addrspace(5) %"149", align 4
  %"152" = fadd float %"153", %"154"
  store float %"152", ptr addrspace(5) %"148", align 4
  %"155" = load float, ptr addrspace(5) %"148", align 4
  store float %"155", ptr addrspace(5) %"145", align 4
  %"156" = load float, ptr addrspace(5) %"145", align 4
  store float %"156", ptr addrspace(5) %"144", align 4
  %2 = load float, ptr addrspace(5) %"144", align 4
  ret float %2
}

define amdgpu_kernel void @call_rnd(ptr addrspace(4) byref(i64) %"95", ptr addrspace(4) byref(i64) %"96") #1 {
  %"97" = alloca i64, align 8, addrspace(5)
  %"98" = alloca i64, align 8, addrspace(5)
  %"99" = alloca float, align 4, addrspace(5)
  %"100" = alloca float, align 4, addrspace(5)
  %"101" = alloca float, align 4, addrspace(5)
  %"102" = alloca float, align 4, addrspace(5)
  %"103" = alloca float, align 4, addrspace(5)
  %"104" = alloca float, align 4, addrspace(5)
  %"105" = alloca float, align 4, addrspace(5)
  %"106" = alloca float, align 4, addrspace(5)
  %"107" = alloca float, align 4, addrspace(5)
  %"108" = alloca float, align 4, addrspace(5)
  %"109" = alloca float, align 4, addrspace(5)
  %"110" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"87"

"87":                                             ; preds = %1
  call void @llvm.amdgcn.s.setreg(i32 6145, i32 1)
  %"111" = load i64, ptr addrspace(4) %"95", align 8
  store i64 %"111", ptr addrspace(5) %"97", align 8
  %"112" = load i64, ptr addrspace(4) %"96", align 8
  store i64 %"112", ptr addrspace(5) %"98", align 8
  %"114" = load i64, ptr addrspace(5) %"97", align 8
  %"157" = inttoptr i64 %"114" to ptr
  %"113" = load float, ptr %"157", align 4
  store float %"113", ptr addrspace(5) %"99", align 4
  %"115" = load i64, ptr addrspace(5) %"97", align 8
  %"158" = inttoptr i64 %"115" to ptr
  %"62" = getelementptr inbounds i8, ptr %"158", i64 4
  %"116" = load float, ptr %"62", align 4
  store float %"116", ptr addrspace(5) %"100", align 4
  %"117" = load i64, ptr addrspace(5) %"97", align 8
  %"159" = inttoptr i64 %"117" to ptr
  %"64" = getelementptr inbounds i8, ptr %"159", i64 8
  %"118" = load float, ptr %"64", align 4
  store float %"118", ptr addrspace(5) %"101", align 4
  %"119" = load i64, ptr addrspace(5) %"97", align 8
  %"160" = inttoptr i64 %"119" to ptr
  %"66" = getelementptr inbounds i8, ptr %"160", i64 12
  %"120" = load float, ptr %"66", align 4
  store float %"120", ptr addrspace(5) %"102", align 4
  %"121" = load float, ptr addrspace(5) %"99", align 4
  store float %"121", ptr addrspace(5) %"105", align 4
  %"122" = load float, ptr addrspace(5) %"100", align 4
  store float %"122", ptr addrspace(5) %"106", align 4
  %"75" = load float, ptr addrspace(5) %"105", align 4
  %"76" = load float, ptr addrspace(5) %"106", align 4
  %"77" = call float @add_rp(float %"75", float %"76")
  br label %"88"

"88":                                             ; preds = %"87"
  store float %"77", ptr addrspace(5) %"107", align 4
  %"123" = load float, ptr addrspace(5) %"107", align 4
  store float %"123", ptr addrspace(5) %"103", align 4
  %"124" = load i64, ptr addrspace(5) %"98", align 8
  %"125" = load float, ptr addrspace(5) %"103", align 4
  %"161" = inttoptr i64 %"124" to ptr
  store float %"125", ptr %"161", align 4
  %"126" = load float, ptr addrspace(5) %"101", align 4
  store float %"126", ptr addrspace(5) %"108", align 4
  %"127" = load float, ptr addrspace(5) %"102", align 4
  store float %"127", ptr addrspace(5) %"109", align 4
  %"78" = load float, ptr addrspace(5) %"108", align 4
  %"79" = load float, ptr addrspace(5) %"109", align 4
  %"80" = call float @add_rm(float %"78", float %"79")
  br label %"89"

"89":                                             ; preds = %"88"
  store float %"80", ptr addrspace(5) %"110", align 4
  %"128" = load float, ptr addrspace(5) %"110", align 4
  store float %"128", ptr addrspace(5) %"104", align 4
  %"129" = load i64, ptr addrspace(5) %"98", align 8
  %"162" = inttoptr i64 %"129" to ptr
  %"68" = getelementptr inbounds i8, ptr %"162", i64 4
  %"130" = load float, ptr addrspace(5) %"104", align 4
  store float %"130", ptr %"68", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.amdgcn.s.setreg(i32 immarg, i32) #2

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind willreturn }