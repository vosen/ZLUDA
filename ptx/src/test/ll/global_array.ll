@foobar = addrspace(1) externally_initialized global [4 x i32] [i32 1, i32 0, i32 0, i32 0]

define amdgpu_kernel void @global_array(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #0 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"39"

"39":                                             ; preds = %1
  store i64 ptrtoint (ptr addrspace(1) @foobar to i64), ptr addrspace(5) %"42", align 8
  %2 = load i64, ptr addrspace(4) %"41", align 8
  store i64 %2, ptr addrspace(5) %"43", align 8
  %3 = load i64, ptr addrspace(5) %"42", align 8
  %"52" = inttoptr i64 %3 to ptr addrspace(1)
  %4 = load i32, ptr addrspace(1) %"52", align 4
  store i32 %4, ptr addrspace(5) %"44", align 4
  %5 = load i64, ptr addrspace(5) %"43", align 8
  %6 = load i32, ptr addrspace(5) %"44", align 4
  %"53" = inttoptr i64 %5 to ptr addrspace(1)
  store i32 %6, ptr addrspace(1) %"53", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
