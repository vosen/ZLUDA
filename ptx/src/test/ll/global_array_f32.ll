@foobar = addrspace(1) global [4 x float] [float 1.000000e+00, float 0.000000e+00, float 0.000000e+00, float 0.000000e+00]

define amdgpu_kernel void @global_array_f32(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #0 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca float, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"38"

"38":                                             ; preds = %1
  store i64 ptrtoint (ptr addrspace(1) @foobar to i64), ptr addrspace(5) %"41", align 8
  %2 = load i64, ptr addrspace(4) %"40", align 8
  store i64 %2, ptr addrspace(5) %"42", align 8
  %3 = load i64, ptr addrspace(5) %"41", align 8
  %"51" = inttoptr i64 %3 to ptr addrspace(1)
  %"37" = getelementptr inbounds i8, ptr addrspace(1) %"51", i64 4
  %4 = load float, ptr addrspace(1) %"37", align 4
  store float %4, ptr addrspace(5) %"43", align 4
  %5 = load i64, ptr addrspace(5) %"42", align 8
  %6 = load float, ptr addrspace(5) %"43", align 4
  %"52" = inttoptr i64 %5 to ptr
  store float %6, ptr %"52", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }