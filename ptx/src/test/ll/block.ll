define amdgpu_kernel void @block(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #0 {
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  store i64 1, ptr addrspace(5) %"47", align 4
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  store i64 1, ptr addrspace(5) %"52", align 4
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
  %"45" = load i64, ptr %"58", align 8
  store i64 %"45", ptr addrspace(5) %"41", align 8
  %"49" = load i64, ptr addrspace(5) %"41", align 8
  %"50" = load i64, ptr addrspace(5) %"47", align 8
  %"48" = add i64 %"49", %"50"
  store i64 %"48", ptr addrspace(5) %"42", align 8
  %"54" = load i64, ptr addrspace(5) %"51", align 8
  %"55" = load i64, ptr addrspace(5) %"52", align 8
  %"53" = add i64 %"54", %"55"
  store i64 %"53", ptr addrspace(5) %"51", align 8
  %"56" = load i64, ptr addrspace(5) %"40", align 8
  %"57" = load i64, ptr addrspace(5) %"42", align 8
  %"59" = inttoptr i64 %"56" to ptr
  store i64 %"57", ptr %"59", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }