@0 = addrspace(4) global i64 1

define amdgpu_kernel void @malformed_label(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #0 {
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"36"

"36":                                             ; preds = %1
  %"44" = load i64, ptr addrspace(4) %"38", align 8
  store i64 %"44", ptr addrspace(5) %"40", align 8
  %"45" = load i64, ptr addrspace(4) %"39", align 8
  store i64 %"45", ptr addrspace(5) %"41", align 8
  br label %"11"

"11":                                             ; preds = %"36"
  %"47" = load i64, ptr addrspace(5) %"40", align 8
  %"52" = inttoptr i64 %"47" to ptr
  %"46" = load i64, ptr %"52", align 8
  store i64 %"46", ptr addrspace(5) %"42", align 8
  %"35" = load i64, ptr addrspace(4) @0, align 8
  %"49" = load i64, ptr addrspace(5) %"42", align 8
  %"48" = add i64 %"49", %"35"
  store i64 %"48", ptr addrspace(5) %"43", align 8
  %"50" = load i64, ptr addrspace(5) %"41", align 8
  %"51" = load i64, ptr addrspace(5) %"43", align 8
  %"53" = inttoptr i64 %"50" to ptr
  store i64 %"51", ptr %"53", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }