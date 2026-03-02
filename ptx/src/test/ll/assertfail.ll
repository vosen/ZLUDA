declare hidden void @__zluda_ptx_impl___assertfail(i64, i64, i32, i64, i64) #0

define amdgpu_kernel void @assertfail(ptr addrspace(4) byref(i64) %"95", ptr addrspace(4) byref(i64) %"96") #1 {
  %"97" = alloca i64, align 8, addrspace(5)
  %"98" = alloca i64, align 8, addrspace(5)
  %"99" = alloca i64, align 8, addrspace(5)
  %"100" = alloca i64, align 8, addrspace(5)
  %"103" = alloca i32, align 4, addrspace(5)
  %"105" = alloca i64, align 8, addrspace(5)
  %"108" = alloca i64, align 8, addrspace(5)
  %"111" = alloca i32, align 4, addrspace(5)
  %"114" = alloca i64, align 8, addrspace(5)
  %"117" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"93"

"93":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"95", align 8
  store i64 %2, ptr addrspace(5) %"97", align 8
  %3 = load i64, ptr addrspace(4) %"96", align 8
  store i64 %3, ptr addrspace(5) %"98", align 8
  store i32 0, ptr addrspace(5) %"103", align 4
  %"106" = getelementptr inbounds i8, ptr addrspace(5) %"105", i64 0
  %4 = load i64, ptr addrspace(5) %"97", align 8
  store i64 %4, ptr addrspace(5) %"106", align 8
  %"109" = getelementptr inbounds i8, ptr addrspace(5) %"108", i64 0
  %5 = load i64, ptr addrspace(5) %"97", align 8
  store i64 %5, ptr addrspace(5) %"109", align 8
  %"112" = getelementptr inbounds i8, ptr addrspace(5) %"111", i64 0
  %6 = load i32, ptr addrspace(5) %"103", align 4
  store i32 %6, ptr addrspace(5) %"112", align 4
  %"115" = getelementptr inbounds i8, ptr addrspace(5) %"114", i64 0
  %7 = load i64, ptr addrspace(5) %"97", align 8
  store i64 %7, ptr addrspace(5) %"115", align 8
  %"118" = getelementptr inbounds i8, ptr addrspace(5) %"117", i64 0
  %8 = load i64, ptr addrspace(5) %"97", align 8
  store i64 %8, ptr addrspace(5) %"118", align 8
  %9 = load i64, ptr addrspace(5) %"105", align 8
  %10 = load i64, ptr addrspace(5) %"108", align 8
  %11 = load i32, ptr addrspace(5) %"111", align 4
  %12 = load i64, ptr addrspace(5) %"114", align 8
  %13 = load i64, ptr addrspace(5) %"117", align 8
  call void @__zluda_ptx_impl___assertfail(i64 %9, i64 %10, i32 %11, i64 %12, i64 %13)
  br label %"94"

"94":                                             ; preds = %"93"
  %14 = load i64, ptr addrspace(5) %"97", align 8
  %"131" = inttoptr i64 %14 to ptr
  %15 = load i64, ptr %"131", align 8
  store i64 %15, ptr addrspace(5) %"99", align 8
  %16 = load i64, ptr addrspace(5) %"99", align 8
  %"122" = add i64 %16, 1
  store i64 %"122", ptr addrspace(5) %"100", align 8
  %17 = load i64, ptr addrspace(5) %"98", align 8
  %18 = load i64, ptr addrspace(5) %"100", align 8
  %"132" = inttoptr i64 %17 to ptr
  store i64 %18, ptr %"132", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
