@0 = addrspace(4) global i64 1

define amdgpu_kernel void @bra(ptr addrspace(4) byref(i64) %"41", ptr addrspace(4) byref(i64) %"42") #0 {
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"40"

"40":                                             ; preds = %1
  %"47" = load i64, ptr addrspace(4) %"41", align 8
  store i64 %"47", ptr addrspace(5) %"43", align 8
  %"48" = load i64, ptr addrspace(4) %"42", align 8
  store i64 %"48", ptr addrspace(5) %"44", align 8
  %"50" = load i64, ptr addrspace(5) %"43", align 8
  %"55" = inttoptr i64 %"50" to ptr
  %"49" = load i64, ptr %"55", align 8
  store i64 %"49", ptr addrspace(5) %"45", align 8
  br label %"11"

"11":                                             ; preds = %"40"
  %"37" = load i64, ptr addrspace(4) @0, align 8
  %"52" = load i64, ptr addrspace(5) %"45", align 8
  %"51" = add i64 %"52", %"37"
  store i64 %"51", ptr addrspace(5) %"46", align 8
  br label %"13"

"13":                                             ; preds = %"11"
  %"53" = load i64, ptr addrspace(5) %"44", align 8
  %"54" = load i64, ptr addrspace(5) %"46", align 8
  %"56" = inttoptr i64 %"53" to ptr
  store i64 %"54", ptr %"56", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }