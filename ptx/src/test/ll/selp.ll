define amdgpu_kernel void @selp(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #0 {
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i16, align 2, addrspace(5)
  %"42" = alloca i16, align 2, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  store i64 2, ptr addrspace(5) %"47", align 4
  %"51" = alloca i1, align 1, addrspace(5)
  store i1 false, ptr addrspace(5) %"51", align 1
  br label %1

1:                                                ; preds = %0
  br label %"36"

"36":                                             ; preds = %1
  %"43" = load i64, ptr addrspace(4) %"37", align 8
  store i64 %"43", ptr addrspace(5) %"39", align 8
  %"44" = load i64, ptr addrspace(4) %"38", align 8
  store i64 %"44", ptr addrspace(5) %"40", align 8
  %"46" = load i64, ptr addrspace(5) %"39", align 8
  %"58" = inttoptr i64 %"46" to ptr
  %"45" = load i16, ptr %"58", align 2
  store i16 %"45", ptr addrspace(5) %"41", align 2
  %"48" = load i64, ptr addrspace(5) %"39", align 8
  %"49" = load i64, ptr addrspace(5) %"47", align 8
  %"59" = inttoptr i64 %"48" to ptr
  %"34" = getelementptr inbounds i8, ptr %"59", i64 %"49"
  %"50" = load i16, ptr %"34", align 2
  store i16 %"50", ptr addrspace(5) %"42", align 2
  %"53" = load i16, ptr addrspace(5) %"41", align 2
  %"54" = load i16, ptr addrspace(5) %"42", align 2
  %"55" = load i1, ptr addrspace(5) %"51", align 1
  %"52" = select i1 %"55", i16 %"53", i16 %"54"
  store i16 %"52", ptr addrspace(5) %"41", align 2
  %"56" = load i64, ptr addrspace(5) %"40", align 8
  %"57" = load i16, ptr addrspace(5) %"41", align 2
  %"60" = inttoptr i64 %"56" to ptr
  store i16 %"57", ptr %"60", align 2
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }