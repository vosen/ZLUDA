define amdgpu_kernel void @vector_operand(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #0 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i16, align 2, addrspace(5)
  %"44" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"38"

"38":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"39", align 8
  store i64 %2, ptr addrspace(5) %"41", align 8
  %3 = load i64, ptr addrspace(4) %"40", align 8
  store i64 %3, ptr addrspace(5) %"42", align 8
  %4 = load i64, ptr addrspace(5) %"41", align 8
  %"53" = inttoptr i64 %4 to ptr
  %5 = load i16, ptr %"53", align 2
  store i16 %5, ptr addrspace(5) %"43", align 2
  %6 = load i16, ptr addrspace(5) %"43", align 2
  %"37" = insertelement <2 x i16> <i16 22136, i16 undef>, i16 %6, i8 1
  %"54" = bitcast <2 x i16> %"37" to i32
  store i32 %"54", ptr addrspace(5) %"44", align 4
  %7 = load i64, ptr addrspace(5) %"42", align 8
  %8 = load i32, ptr addrspace(5) %"44", align 4
  %"55" = inttoptr i64 %7 to ptr
  store i32 %8, ptr %"55", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }