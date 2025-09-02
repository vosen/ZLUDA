@0 = addrspace(4) global i64 4
@1 = addrspace(4) global i64 8
@2 = addrspace(4) global i64 12
@3 = addrspace(4) global i64 4

define hidden float @add_rm(float %"86", float %"87") #0 {
  %"135" = alloca float, align 4, addrspace(5)
  %"136" = alloca float, align 4, addrspace(5)
  %"137" = alloca float, align 4, addrspace(5)
  %"138" = alloca float, align 4, addrspace(5)
  %"139" = alloca float, align 4, addrspace(5)
  %"140" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"96"

"96":                                             ; preds = %1
  call void @llvm.amdgcn.s.setreg(i32 6145, i32 2)
  br label %"94"

"94":                                             ; preds = %"96"
  store float %"86", ptr addrspace(5) %"137", align 4
  store float %"87", ptr addrspace(5) %"138", align 4
  %"141" = load float, ptr addrspace(5) %"137", align 4
  store float %"141", ptr addrspace(5) %"139", align 4
  %"142" = load float, ptr addrspace(5) %"138", align 4
  store float %"142", ptr addrspace(5) %"140", align 4
  %"144" = load float, ptr addrspace(5) %"139", align 4
  %"145" = load float, ptr addrspace(5) %"140", align 4
  %"143" = fadd float %"144", %"145"
  store float %"143", ptr addrspace(5) %"139", align 4
  %"146" = load float, ptr addrspace(5) %"139", align 4
  store float %"146", ptr addrspace(5) %"136", align 4
  %"147" = load float, ptr addrspace(5) %"136", align 4
  store float %"147", ptr addrspace(5) %"135", align 4
  %2 = load float, ptr addrspace(5) %"135", align 4
  ret float %2
}

define hidden float @add_rp(float %"89", float %"90") #0 {
  %"148" = alloca float, align 4, addrspace(5)
  %"149" = alloca float, align 4, addrspace(5)
  %"150" = alloca float, align 4, addrspace(5)
  %"151" = alloca float, align 4, addrspace(5)
  %"152" = alloca float, align 4, addrspace(5)
  %"153" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"95"

"95":                                             ; preds = %1
  store float %"89", ptr addrspace(5) %"150", align 4
  store float %"90", ptr addrspace(5) %"151", align 4
  %"154" = load float, ptr addrspace(5) %"150", align 4
  store float %"154", ptr addrspace(5) %"152", align 4
  %"155" = load float, ptr addrspace(5) %"151", align 4
  store float %"155", ptr addrspace(5) %"153", align 4
  %"157" = load float, ptr addrspace(5) %"152", align 4
  %"158" = load float, ptr addrspace(5) %"153", align 4
  %"156" = fadd float %"157", %"158"
  store float %"156", ptr addrspace(5) %"152", align 4
  %"159" = load float, ptr addrspace(5) %"152", align 4
  store float %"159", ptr addrspace(5) %"149", align 4
  %"160" = load float, ptr addrspace(5) %"149", align 4
  store float %"160", ptr addrspace(5) %"148", align 4
  %2 = load float, ptr addrspace(5) %"148", align 4
  ret float %2
}

define amdgpu_kernel void @call_rnd(ptr addrspace(4) byref(i64) %"99", ptr addrspace(4) byref(i64) %"100") #1 {
  %"101" = alloca i64, align 8, addrspace(5)
  %"102" = alloca i64, align 8, addrspace(5)
  %"103" = alloca float, align 4, addrspace(5)
  %"104" = alloca float, align 4, addrspace(5)
  %"105" = alloca float, align 4, addrspace(5)
  %"106" = alloca float, align 4, addrspace(5)
  %"107" = alloca float, align 4, addrspace(5)
  %"108" = alloca float, align 4, addrspace(5)
  %"109" = alloca float, align 4, addrspace(5)
  %"110" = alloca float, align 4, addrspace(5)
  %"111" = alloca float, align 4, addrspace(5)
  %"112" = alloca float, align 4, addrspace(5)
  %"113" = alloca float, align 4, addrspace(5)
  %"114" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"91"

"91":                                             ; preds = %1
  call void @llvm.amdgcn.s.setreg(i32 6145, i32 1)
  %"115" = load i64, ptr addrspace(4) %"99", align 8
  store i64 %"115", ptr addrspace(5) %"101", align 8
  %"116" = load i64, ptr addrspace(4) %"100", align 8
  store i64 %"116", ptr addrspace(5) %"102", align 8
  %"118" = load i64, ptr addrspace(5) %"101", align 8
  %"161" = inttoptr i64 %"118" to ptr
  %"117" = load float, ptr %"161", align 4
  store float %"117", ptr addrspace(5) %"103", align 4
  %"62" = load i64, ptr addrspace(4) @0, align 8
  %"119" = load i64, ptr addrspace(5) %"101", align 8
  %"162" = inttoptr i64 %"119" to ptr
  %"63" = getelementptr inbounds i8, ptr %"162", i64 %"62"
  %"120" = load float, ptr %"63", align 4
  store float %"120", ptr addrspace(5) %"104", align 4
  %"65" = load i64, ptr addrspace(4) @1, align 8
  %"121" = load i64, ptr addrspace(5) %"101", align 8
  %"163" = inttoptr i64 %"121" to ptr
  %"66" = getelementptr inbounds i8, ptr %"163", i64 %"65"
  %"122" = load float, ptr %"66", align 4
  store float %"122", ptr addrspace(5) %"105", align 4
  %"68" = load i64, ptr addrspace(4) @2, align 8
  %"123" = load i64, ptr addrspace(5) %"101", align 8
  %"164" = inttoptr i64 %"123" to ptr
  %"69" = getelementptr inbounds i8, ptr %"164", i64 %"68"
  %"124" = load float, ptr %"69", align 4
  store float %"124", ptr addrspace(5) %"106", align 4
  %"125" = load float, ptr addrspace(5) %"103", align 4
  store float %"125", ptr addrspace(5) %"109", align 4
  %"126" = load float, ptr addrspace(5) %"104", align 4
  store float %"126", ptr addrspace(5) %"110", align 4
  %"79" = load float, ptr addrspace(5) %"109", align 4
  %"80" = load float, ptr addrspace(5) %"110", align 4
  %"81" = call float @add_rp(float %"79", float %"80")
  br label %"92"

"92":                                             ; preds = %"91"
  store float %"81", ptr addrspace(5) %"111", align 4
  %"127" = load float, ptr addrspace(5) %"111", align 4
  store float %"127", ptr addrspace(5) %"107", align 4
  %"128" = load i64, ptr addrspace(5) %"102", align 8
  %"129" = load float, ptr addrspace(5) %"107", align 4
  %"165" = inttoptr i64 %"128" to ptr
  store float %"129", ptr %"165", align 4
  %"130" = load float, ptr addrspace(5) %"105", align 4
  store float %"130", ptr addrspace(5) %"112", align 4
  %"131" = load float, ptr addrspace(5) %"106", align 4
  store float %"131", ptr addrspace(5) %"113", align 4
  %"82" = load float, ptr addrspace(5) %"112", align 4
  %"83" = load float, ptr addrspace(5) %"113", align 4
  %"84" = call float @add_rm(float %"82", float %"83")
  br label %"93"

"93":                                             ; preds = %"92"
  store float %"84", ptr addrspace(5) %"114", align 4
  %"132" = load float, ptr addrspace(5) %"114", align 4
  store float %"132", ptr addrspace(5) %"108", align 4
  %"71" = load i64, ptr addrspace(4) @3, align 8
  %"133" = load i64, ptr addrspace(5) %"102", align 8
  %"166" = inttoptr i64 %"133" to ptr
  %"72" = getelementptr inbounds i8, ptr %"166", i64 %"71"
  %"134" = load float, ptr addrspace(5) %"108", align 4
  store float %"134", ptr %"72", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.amdgcn.s.setreg(i32 immarg, i32) #2

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #2 = { nocallback nofree nosync nounwind willreturn }