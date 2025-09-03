define amdgpu_kernel void @shl(ptr addrspace(4) byref(i64) %"35", ptr addrspace(4) byref(i64) %"36") #0 {
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i32, align 4, addrspace(5)
  store i32 2, ptr addrspace(5) %"45", align 4
  br label %1

1:                                                ; preds = %0
  br label %"34"

"34":                                             ; preds = %1
  %"41" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"41", ptr addrspace(5) %"37", align 8
  %"42" = load i64, ptr addrspace(4) %"36", align 8
  store i64 %"42", ptr addrspace(5) %"38", align 8
  %"44" = load i64, ptr addrspace(5) %"37", align 8
  %"51" = inttoptr i64 %"44" to ptr
  %"43" = load i64, ptr %"51", align 8
  store i64 %"43", ptr addrspace(5) %"39", align 8
  %"47" = load i64, ptr addrspace(5) %"39", align 8
  %"48" = load i32, ptr addrspace(5) %"45", align 4
  %2 = icmp uge i32 %"48", 64
  %3 = zext i32 %"48" to i64
  %4 = shl i64 %"47", %3
  %"52" = select i1 %2, i64 0, i64 %4
  store i64 %"52", ptr addrspace(5) %"40", align 8
  %"49" = load i64, ptr addrspace(5) %"38", align 8
  %"50" = load i64, ptr addrspace(5) %"40", align 8
  %"54" = inttoptr i64 %"49" to ptr
  store i64 %"50", ptr %"54", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }