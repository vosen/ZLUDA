declare hidden [16 x i8] @foobar(i64) #0

define amdgpu_kernel void @extern_func(ptr addrspace(4) byref(i64) %"50", ptr addrspace(4) byref(i64) %"51") #1 {
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  %"55" = alloca i64, align 8, addrspace(5)
  %"60" = alloca i64, align 8, addrspace(5)
  %"63" = alloca [16 x i8], align 16, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"47"

"47":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"50", align 8
  store i64 %2, ptr addrspace(5) %"52", align 8
  %3 = load i64, ptr addrspace(4) %"51", align 8
  store i64 %3, ptr addrspace(5) %"53", align 8
  %4 = load i64, ptr addrspace(5) %"52", align 8
  %"67" = inttoptr i64 %4 to ptr addrspace(1)
  %5 = load i64, ptr addrspace(1) %"67", align 8
  store i64 %5, ptr addrspace(5) %"54", align 8
  %"61" = getelementptr inbounds i8, ptr addrspace(5) %"60", i64 0
  %6 = load i64, ptr addrspace(5) %"54", align 8
  store i64 %6, ptr addrspace(5) %"61", align 8
  %7 = load i64, ptr addrspace(5) %"60", align 8
  %"46" = call [16 x i8] @foobar(i64 %7)
  br label %"48"

"48":                                             ; preds = %"47"
  store [16 x i8] %"46", ptr addrspace(5) %"63", align 1
  %8 = load i64, ptr addrspace(5) %"63", align 8
  store i64 %8, ptr addrspace(5) %"55", align 8
  %9 = load i64, ptr addrspace(5) %"53", align 8
  %10 = load i64, ptr addrspace(5) %"55", align 8
  %"70" = inttoptr i64 %9 to ptr
  store i64 %10, ptr %"70", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }