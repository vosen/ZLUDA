define hidden i64 @incr(i64 %"52") #0 {
  %"72" = alloca i64, align 8, addrspace(5)
  %"73" = alloca i64, align 8, addrspace(5)
  %"74" = alloca i64, align 8, addrspace(5)
  %"75" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"55"

"55":                                             ; preds = %1
  store i64 %"52", ptr addrspace(5) %"74", align 8
  %2 = load i64, ptr addrspace(5) %"74", align 8
  store i64 %2, ptr addrspace(5) %"75", align 8
  %3 = load i64, ptr addrspace(5) %"75", align 8
  %"77" = add i64 %3, 1
  store i64 %"77", ptr addrspace(5) %"75", align 8
  %4 = load i64, ptr addrspace(5) %"75", align 8
  store i64 %4, ptr addrspace(5) %"73", align 8
  %5 = load i64, ptr addrspace(5) %"73", align 8
  store i64 %5, ptr addrspace(5) %"72", align 8
  %6 = load i64, ptr addrspace(5) %"72", align 8
  ret i64 %6
}

define amdgpu_kernel void @call(ptr addrspace(4) byref(i64) %"57", ptr addrspace(4) byref(i64) %"58") #1 {
  %"59" = alloca i64, align 8, addrspace(5)
  %"60" = alloca i64, align 8, addrspace(5)
  %"61" = alloca i64, align 8, addrspace(5)
  %"66" = alloca i64, align 8, addrspace(5)
  %"67" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"53"

"53":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"57", align 8
  store i64 %2, ptr addrspace(5) %"59", align 8
  %3 = load i64, ptr addrspace(4) %"58", align 8
  store i64 %3, ptr addrspace(5) %"60", align 8
  %4 = load i64, ptr addrspace(5) %"59", align 8
  %"81" = inttoptr i64 %4 to ptr addrspace(1)
  %5 = load i64, ptr addrspace(1) %"81", align 8
  store i64 %5, ptr addrspace(5) %"61", align 8
  %6 = load i64, ptr addrspace(5) %"61", align 8
  store i64 %6, ptr addrspace(5) %"66", align 8
  %7 = load i64, ptr addrspace(5) %"66", align 8
  %"50" = call i64 @incr(i64 %7)
  br label %"54"

"54":                                             ; preds = %"53"
  store i64 %"50", ptr addrspace(5) %"67", align 8
  %8 = load i64, ptr addrspace(5) %"67", align 8
  store i64 %8, ptr addrspace(5) %"61", align 8
  %9 = load i64, ptr addrspace(5) %"60", align 8
  %10 = load i64, ptr addrspace(5) %"61", align 8
  %"84" = inttoptr i64 %9 to ptr addrspace(1)
  store i64 %10, ptr addrspace(1) %"84", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
