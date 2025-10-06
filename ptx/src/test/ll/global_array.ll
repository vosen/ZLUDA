@foobar = addrspace(1) global [4 x i32] [i32 1, i32 0, i32 0, i32 0]

define amdgpu_kernel void @global_array(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #0 {
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"36"

"36":                                             ; preds = %1
  store i64 ptrtoint (ptr addrspace(1) @foobar to i64), ptr addrspace(5) %"39", align 8
  %2 = load i64, ptr addrspace(4) %"38", align 8
  store i64 %2, ptr addrspace(5) %"40", align 8
  %3 = load i64, ptr addrspace(5) %"39", align 8
  %"49" = inttoptr i64 %3 to ptr addrspace(1)
  %4 = load i32, ptr addrspace(1) %"49", align 4
  store i32 %4, ptr addrspace(5) %"41", align 4
  %5 = load i64, ptr addrspace(5) %"40", align 8
  %6 = load i32, ptr addrspace(5) %"41", align 4
  %"50" = inttoptr i64 %5 to ptr addrspace(1)
  store i32 %6, ptr addrspace(1) %"50", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }