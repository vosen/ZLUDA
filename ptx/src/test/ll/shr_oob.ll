define amdgpu_kernel void @shr_oob(ptr addrspace(4) byref(i64) %"34", ptr addrspace(4) byref(i64) %"35") #0 {
  %"36" = alloca i64, align 8, addrspace(5)
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i16, align 2, addrspace(5)
  %"43" = alloca i32, align 4, addrspace(5)
  store i32 16, ptr addrspace(5) %"43", align 4
  br label %1

1:                                                ; preds = %0
  br label %"33"

"33":                                             ; preds = %1
  %"39" = load i64, ptr addrspace(4) %"34", align 8
  store i64 %"39", ptr addrspace(5) %"36", align 8
  %"40" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"40", ptr addrspace(5) %"37", align 8
  %"42" = load i64, ptr addrspace(5) %"36", align 8
  %"49" = inttoptr i64 %"42" to ptr
  %"41" = load i16, ptr %"49", align 2
  store i16 %"41", ptr addrspace(5) %"38", align 2
  %"45" = load i16, ptr addrspace(5) %"38", align 2
  %"46" = load i32, ptr addrspace(5) %"43", align 4
  %2 = ashr i16 %"45", 15
  %3 = icmp uge i32 %"46", 16
  %4 = trunc i32 %"46" to i16
  %5 = ashr i16 %"45", %4
  %"44" = select i1 %3, i16 %2, i16 %5
  store i16 %"44", ptr addrspace(5) %"38", align 2
  %"47" = load i64, ptr addrspace(5) %"37", align 8
  %"48" = load i16, ptr addrspace(5) %"38", align 2
  %"50" = inttoptr i64 %"47" to ptr
  store i16 %"48", ptr %"50", align 2
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }