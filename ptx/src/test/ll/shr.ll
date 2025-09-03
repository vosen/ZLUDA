define amdgpu_kernel void @shr(ptr addrspace(4) byref(i64) %"34", ptr addrspace(4) byref(i64) %"35") #0 {
  %"36" = alloca i64, align 8, addrspace(5)
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i32, align 4, addrspace(5)
  %"43" = alloca i32, align 4, addrspace(5)
  store i32 1, ptr addrspace(5) %"43", align 4
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
  %"41" = load i32, ptr %"49", align 4
  store i32 %"41", ptr addrspace(5) %"38", align 4
  %"45" = load i32, ptr addrspace(5) %"38", align 4
  %"46" = load i32, ptr addrspace(5) %"43", align 4
  %2 = ashr i32 %"45", 31
  %3 = icmp uge i32 %"46", 32
  %4 = ashr i32 %"45", %"46"
  %"44" = select i1 %3, i32 %2, i32 %4
  store i32 %"44", ptr addrspace(5) %"38", align 4
  %"47" = load i64, ptr addrspace(5) %"37", align 8
  %"48" = load i32, ptr addrspace(5) %"38", align 4
  %"50" = inttoptr i64 %"47" to ptr
  store i32 %"48", ptr %"50", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }