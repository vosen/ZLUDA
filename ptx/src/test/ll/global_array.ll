@foobar = addrspace(1) global [4 x i32] [i32 1, i32 0, i32 0, i32 0]

define amdgpu_kernel void @global_array(ptr addrspace(4) byref(i64) %"31", ptr addrspace(4) byref(i64) %"32") #0 {
  %"33" = alloca i64, align 8, addrspace(5)
  %"34" = alloca i64, align 8, addrspace(5)
  %"35" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"30"

"30":                                             ; preds = %1
  store i64 ptrtoint (ptr addrspace(1) @foobar to i64), ptr addrspace(5) %"33", align 8
  %"37" = load i64, ptr addrspace(4) %"32", align 8
  store i64 %"37", ptr addrspace(5) %"34", align 8
  %"39" = load i64, ptr addrspace(5) %"33", align 8
  %"43" = inttoptr i64 %"39" to ptr addrspace(1)
  %"38" = load i32, ptr addrspace(1) %"43", align 4
  store i32 %"38", ptr addrspace(5) %"35", align 4
  %"40" = load i64, ptr addrspace(5) %"34", align 8
  %"41" = load i32, ptr addrspace(5) %"35", align 4
  %"44" = inttoptr i64 %"40" to ptr addrspace(1)
  store i32 %"41", ptr addrspace(1) %"44", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }