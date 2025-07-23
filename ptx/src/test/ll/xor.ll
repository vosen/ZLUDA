define amdgpu_kernel void @xor(ptr addrspace(4) byref(i64) %"33", ptr addrspace(4) byref(i64) %"34") #0 {
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
  %"45" = xor i32 %"46", %"47"
  store i32 %"45", ptr addrspace(5) %"37", align 4
  %"48" = load i64, ptr addrspace(5) %"36", align 8
  %"49" = load i32, ptr addrspace(5) %"37", align 4
  %"52" = inttoptr i64 %"48" to ptr
  store i32 %"49", ptr %"52", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }