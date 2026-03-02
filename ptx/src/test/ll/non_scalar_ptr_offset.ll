define amdgpu_kernel void @non_scalar_ptr_offset(ptr addrspace(4) byref(i64) %"43", ptr addrspace(4) byref(i64) %"44") #0 {
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i32, align 4, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"42"

"42":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"43", align 8
  store i64 %2, ptr addrspace(5) %"45", align 8
  %3 = load i64, ptr addrspace(4) %"44", align 8
  store i64 %3, ptr addrspace(5) %"46", align 8
  %4 = load i64, ptr addrspace(5) %"45", align 8
  %"59" = inttoptr i64 %4 to ptr addrspace(1)
  %"41" = getelementptr inbounds i8, ptr addrspace(1) %"59", i64 8
  %5 = load <2 x i32>, ptr addrspace(1) %"41", align 8
  %"52" = extractelement <2 x i32> %5, i8 0
  %"53" = extractelement <2 x i32> %5, i8 1
  store i32 %"52", ptr addrspace(5) %"47", align 4
  store i32 %"53", ptr addrspace(5) %"48", align 4
  %6 = load i32, ptr addrspace(5) %"47", align 4
  %7 = load i32, ptr addrspace(5) %"48", align 4
  %"54" = add i32 %6, %7
  store i32 %"54", ptr addrspace(5) %"47", align 4
  %8 = load i64, ptr addrspace(5) %"46", align 8
  %9 = load i32, ptr addrspace(5) %"47", align 4
  %"60" = inttoptr i64 %8 to ptr addrspace(1)
  store i32 %9, ptr addrspace(1) %"60", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
