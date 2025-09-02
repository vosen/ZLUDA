@foobar = addrspace(1) global [4 x i32] [i32 1, i32 0, i32 0, i32 0]

define amdgpu_kernel void @global_array(ptr addrspace(4) byref(i64) %"34", ptr addrspace(4) byref(i64) %"35") #0 {
  %"36" = alloca i64, align 8, addrspace(5)
  %"37" = alloca i64, align 8, addrspace(5)
  %"38" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"33"

"33":                                             ; preds = %1
  store i64 ptrtoint (ptr addrspace(1) @foobar to i64), ptr addrspace(5) %"36", align 8
  %"40" = load i64, ptr addrspace(4) %"35", align 8
  store i64 %"40", ptr addrspace(5) %"37", align 8
  %"42" = load i64, ptr addrspace(5) %"36", align 8
  %"46" = inttoptr i64 %"42" to ptr addrspace(1)
  %"41" = load i32, ptr addrspace(1) %"46", align 4
  store i32 %"41", ptr addrspace(5) %"38", align 4
  %"43" = load i64, ptr addrspace(5) %"37", align 8
  %"44" = load i32, ptr addrspace(5) %"38", align 4
  %"47" = inttoptr i64 %"43" to ptr addrspace(1)
  store i32 %"44", ptr addrspace(1) %"47", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }