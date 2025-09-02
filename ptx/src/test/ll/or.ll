@0 = addrspace(4) global i64 8

define amdgpu_kernel void @or(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #0 {
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"36"

"36":                                             ; preds = %1
  %"43" = load i64, ptr addrspace(4) %"37", align 8
  store i64 %"43", ptr addrspace(5) %"39", align 8
  %"44" = load i64, ptr addrspace(4) %"38", align 8
  store i64 %"44", ptr addrspace(5) %"40", align 8
  %"46" = load i64, ptr addrspace(5) %"39", align 8
  %"54" = inttoptr i64 %"46" to ptr
  %"45" = load i64, ptr %"54", align 8
  store i64 %"45", ptr addrspace(5) %"41", align 8
  %"34" = load i64, ptr addrspace(4) @0, align 8
  %"47" = load i64, ptr addrspace(5) %"39", align 8
  %"55" = inttoptr i64 %"47" to ptr
  %"35" = getelementptr inbounds i8, ptr %"55", i64 %"34"
  %"48" = load i64, ptr %"35", align 8
  store i64 %"48", ptr addrspace(5) %"42", align 8
  %"50" = load i64, ptr addrspace(5) %"41", align 8
  %"51" = load i64, ptr addrspace(5) %"42", align 8
  %"56" = or i64 %"50", %"51"
  store i64 %"56", ptr addrspace(5) %"41", align 8
  %"52" = load i64, ptr addrspace(5) %"40", align 8
  %"53" = load i64, ptr addrspace(5) %"41", align 8
  %"59" = inttoptr i64 %"52" to ptr
  store i64 %"53", ptr %"59", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }