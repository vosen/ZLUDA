define amdgpu_kernel void @selp_true(ptr addrspace(4) byref(i64) %"34", ptr addrspace(4) byref(i64) %"35") #0 {
  %"36" = alloca i64, align 8, addrspace(5)
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i16, align 2, addrspace(5)
  %"39" = alloca i16, align 2, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"33"

"33":                                             ; preds = %1
  %"40" = load i64, ptr addrspace(4) %"34", align 4
  store i64 %"40", ptr addrspace(5) %"36", align 4
  %"41" = load i64, ptr addrspace(4) %"35", align 4
  store i64 %"41", ptr addrspace(5) %"37", align 4
  %"43" = load i64, ptr addrspace(5) %"36", align 4
  %"51" = inttoptr i64 %"43" to ptr
  %"42" = load i16, ptr %"51", align 2
  store i16 %"42", ptr addrspace(5) %"38", align 2
  %"44" = load i64, ptr addrspace(5) %"36", align 4
  %"52" = inttoptr i64 %"44" to ptr
  %"31" = getelementptr inbounds i8, ptr %"52", i64 2
  %"45" = load i16, ptr %"31", align 2
  store i16 %"45", ptr addrspace(5) %"39", align 2
  %"47" = load i16, ptr addrspace(5) %"38", align 2
  %"48" = load i16, ptr addrspace(5) %"39", align 2
  %"46" = select i1 true, i16 %"47", i16 %"48"
  store i16 %"46", ptr addrspace(5) %"38", align 2
  %"49" = load i64, ptr addrspace(5) %"37", align 4
  %"50" = load i16, ptr addrspace(5) %"38", align 2
  %"53" = inttoptr i64 %"49" to ptr
  store i16 %"50", ptr %"53", align 2
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }