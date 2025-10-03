define amdgpu_kernel void @cvt_s16_s8(ptr addrspace(4) byref(i64) %"34", ptr addrspace(4) byref(i64) %"35") #0 {
  %"36" = alloca i64, align 8, addrspace(5)
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i32, align 4, addrspace(5)
  %"39" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"33"

"33":                                             ; preds = %1
  %"40" = load i64, ptr addrspace(4) %"34", align 8
  store i64 %"40", ptr addrspace(5) %"36", align 8
  %"41" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"41", ptr addrspace(5) %"37", align 8
  %"43" = load i64, ptr addrspace(5) %"36", align 8
  %"48" = inttoptr i64 %"43" to ptr addrspace(1)
  %"42" = load i32, ptr addrspace(1) %"48", align 4
  store i32 %"42", ptr addrspace(5) %"39", align 4
  %"45" = load i32, ptr addrspace(5) %"39", align 4
  %2 = trunc i32 %"45" to i8
  %"49" = sext i8 %2 to i16
  %"44" = sext i16 %"49" to i32
  store i32 %"44", ptr addrspace(5) %"38", align 4
  %"46" = load i64, ptr addrspace(5) %"37", align 8
  %"47" = load i32, ptr addrspace(5) %"38", align 4
  %"51" = inttoptr i64 %"46" to ptr
  store i32 %"47", ptr %"51", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
