define hidden float @add_rm(float %"82", float %"83") #0 {
  %"139" = alloca float, align 4, addrspace(5)
  %"140" = alloca float, align 4, addrspace(5)
  %"141" = alloca float, align 4, addrspace(5)
  %"142" = alloca float, align 4, addrspace(5)
  %"143" = alloca float, align 4, addrspace(5)
  %"144" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"92"

"92":                                             ; preds = %1
  call void @llvm.amdgcn.s.setreg(i32 6145, i32 2)
  br label %"90"

"90":                                             ; preds = %"92"
  store float %"82", ptr addrspace(5) %"141", align 4
  store float %"83", ptr addrspace(5) %"142", align 4
  %"145" = load float, ptr addrspace(5) %"141", align 4
  store float %"145", ptr addrspace(5) %"143", align 4
  %"146" = load float, ptr addrspace(5) %"142", align 4
  store float %"146", ptr addrspace(5) %"144", align 4
  %"148" = load float, ptr addrspace(5) %"143", align 4
  %"149" = load float, ptr addrspace(5) %"144", align 4
  %"147" = fadd float %"148", %"149"
  store float %"147", ptr addrspace(5) %"143", align 4
  %"150" = load float, ptr addrspace(5) %"143", align 4
  store float %"150", ptr addrspace(5) %"140", align 4
  %"151" = load float, ptr addrspace(5) %"140", align 4
  store float %"151", ptr addrspace(5) %"139", align 4
  %2 = load float, ptr addrspace(5) %"139", align 4
  ret float %2
}

define hidden float @add_rp(float %"85", float %"86") #0 {
  %"152" = alloca float, align 4, addrspace(5)
  %"153" = alloca float, align 4, addrspace(5)
  %"154" = alloca float, align 4, addrspace(5)
  %"155" = alloca float, align 4, addrspace(5)
  %"156" = alloca float, align 4, addrspace(5)
  %"157" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"91"

"91":                                             ; preds = %1
  store float %"85", ptr addrspace(5) %"154", align 4
  store float %"86", ptr addrspace(5) %"155", align 4
  %"158" = load float, ptr addrspace(5) %"154", align 4
  store float %"158", ptr addrspace(5) %"156", align 4
  %"159" = load float, ptr addrspace(5) %"155", align 4
  store float %"159", ptr addrspace(5) %"157", align 4
  %"161" = load float, ptr addrspace(5) %"156", align 4
  %"162" = load float, ptr addrspace(5) %"157", align 4
  %"160" = fadd float %"161", %"162"
  store float %"160", ptr addrspace(5) %"156", align 4
  %"163" = load float, ptr addrspace(5) %"156", align 4
  store float %"163", ptr addrspace(5) %"153", align 4
  %"164" = load float, ptr addrspace(5) %"153", align 4
  store float %"164", ptr addrspace(5) %"152", align 4
  %2 = load float, ptr addrspace(5) %"152", align 4
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
  %"115" = alloca i64, align 8, addrspace(5)
  store i64 4, ptr addrspace(5) %"115", align 4
  %"119" = alloca i64, align 8, addrspace(5)
  store i64 8, ptr addrspace(5) %"119", align 4
  %"123" = alloca i64, align 8, addrspace(5)
  store i64 12, ptr addrspace(5) %"123", align 4
  %"135" = alloca i64, align 8, addrspace(5)
  store i64 4, ptr addrspace(5) %"135", align 4
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
  %"165" = inttoptr i64 %"114" to ptr
  %"113" = load float, ptr %"165", align 4
  store float %"113", ptr addrspace(5) %"99", align 4
  %"116" = load i64, ptr addrspace(5) %"97", align 8
  %"117" = load i64, ptr addrspace(5) %"115", align 8
  %"166" = inttoptr i64 %"116" to ptr
  %"62" = getelementptr inbounds i8, ptr %"166", i64 %"117"
  %"118" = load float, ptr %"62", align 4
  store float %"118", ptr addrspace(5) %"100", align 4
  %"120" = load i64, ptr addrspace(5) %"97", align 8
  %"121" = load i64, ptr addrspace(5) %"119", align 8
  %"167" = inttoptr i64 %"120" to ptr
  %"64" = getelementptr inbounds i8, ptr %"167", i64 %"121"
  %"122" = load float, ptr %"64", align 4
  store float %"122", ptr addrspace(5) %"101", align 4
  %"124" = load i64, ptr addrspace(5) %"97", align 8
  %"125" = load i64, ptr addrspace(5) %"123", align 8
  %"168" = inttoptr i64 %"124" to ptr
  %"66" = getelementptr inbounds i8, ptr %"168", i64 %"125"
  %"126" = load float, ptr %"66", align 4
  store float %"126", ptr addrspace(5) %"102", align 4
  %"127" = load float, ptr addrspace(5) %"99", align 4
  store float %"127", ptr addrspace(5) %"105", align 4
  %"128" = load float, ptr addrspace(5) %"100", align 4
  store float %"128", ptr addrspace(5) %"106", align 4
  %"75" = load float, ptr addrspace(5) %"105", align 4
  %"76" = load float, ptr addrspace(5) %"106", align 4
  %"77" = call float @add_rp(float %"75", float %"76")
  br label %"88"

"88":                                             ; preds = %"87"
  store float %"77", ptr addrspace(5) %"107", align 4
  %"129" = load float, ptr addrspace(5) %"107", align 4
  store float %"129", ptr addrspace(5) %"103", align 4
  %"130" = load i64, ptr addrspace(5) %"98", align 8
  %"131" = load float, ptr addrspace(5) %"103", align 4
  %"169" = inttoptr i64 %"130" to ptr
  store float %"131", ptr %"169", align 4
  %"132" = load float, ptr addrspace(5) %"101", align 4
  store float %"132", ptr addrspace(5) %"108", align 4
  %"133" = load float, ptr addrspace(5) %"102", align 4
  store float %"133", ptr addrspace(5) %"109", align 4
  %"78" = load float, ptr addrspace(5) %"108", align 4
  %"79" = load float, ptr addrspace(5) %"109", align 4
  %"80" = call float @add_rm(float %"78", float %"79")
  br label %"89"

"89":                                             ; preds = %"88"
  store float %"80", ptr addrspace(5) %"110", align 4
  %"134" = load float, ptr addrspace(5) %"110", align 4
  store float %"134", ptr addrspace(5) %"104", align 4
  %"136" = load i64, ptr addrspace(5) %"98", align 8
  %"137" = load i64, ptr addrspace(5) %"135", align 8
  %"170" = inttoptr i64 %"136" to ptr
  %"68" = getelementptr inbounds i8, ptr %"170", i64 %"137"
  %"138" = load float, ptr addrspace(5) %"104", align 4
  store float %"138", ptr %"68", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.amdgcn.s.setreg(i32 immarg, i32) #2

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind willreturn }