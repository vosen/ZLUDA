define amdgpu_kernel void @prmt(ptr addrspace(4) byref(i64) %"33", ptr addrspace(4) byref(i64) %"34") #0 {
  %"35" = alloca i64, align 8, addrspace(5)
  %"36" = alloca i64, align 8, addrspace(5)
  %"37" = alloca i32, align 4, addrspace(5)
  %"38" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"32"

"32":                                             ; preds = %1
  %"39" = load i64, ptr addrspace(4) %"33", align 8
  store i64 %"39", ptr addrspace(5) %"35", align 8
  %"40" = load i64, ptr addrspace(4) %"34", align 8
  store i64 %"40", ptr addrspace(5) %"36", align 8
  %"42" = load i64, ptr addrspace(5) %"35", align 8
  %"50" = inttoptr i64 %"42" to ptr
  %"41" = load i32, ptr %"50", align 4
  store i32 %"41", ptr addrspace(5) %"37", align 4
  %"43" = load i64, ptr addrspace(5) %"35", align 8
  %"51" = inttoptr i64 %"43" to ptr
  %"31" = getelementptr inbounds i8, ptr %"51", i64 4
  %"44" = load i32, ptr %"31", align 4
  store i32 %"44", ptr addrspace(5) %"38", align 4
  %"46" = load i32, ptr addrspace(5) %"37", align 4
  %"47" = load i32, ptr addrspace(5) %"38", align 4
  %2 = bitcast i32 %"46" to <4 x i8>
  %3 = bitcast i32 %"47" to <4 x i8>
  %"52" = shufflevector <4 x i8> %2, <4 x i8> %3, <4 x i32> <i32 4, i32 0, i32 6, i32 7>
  store <4 x i8> %"52", ptr addrspace(5) %"38", align 4
  %"48" = load i64, ptr addrspace(5) %"36", align 8
  %"49" = load i32, ptr addrspace(5) %"38", align 4
  %"55" = inttoptr i64 %"48" to ptr
  store i32 %"49", ptr %"55", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }