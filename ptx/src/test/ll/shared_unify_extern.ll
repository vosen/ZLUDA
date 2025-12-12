@shared_ex = external addrspace(3) global [0 x i32]
@shared_mod = external addrspace(3) global [4 x i32]

define hidden i64 @add() #0 {
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"47"

"47":                                             ; preds = %1
  %2 = load i64, ptr addrspace(3) @shared_mod, align 8
  store i64 %2, ptr addrspace(5) %"53", align 8
  %3 = load i64, ptr addrspace(3) @shared_ex, align 8
  store i64 %3, ptr addrspace(5) %"54", align 8
  %4 = load i64, ptr addrspace(5) %"54", align 8
  %5 = load i64, ptr addrspace(5) %"53", align 8
  %"81" = add i64 %4, %5
  store i64 %"81", ptr addrspace(5) %"52", align 8
  %6 = load i64, ptr addrspace(5) %"52", align 8
  ret i64 %6
}

define hidden i64 @set_shared_temp1(i64 %"17") #0 {
  %"60" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"48"

"48":                                             ; preds = %1
  store i64 %"17", ptr addrspace(3) @shared_ex, align 8
  %"61" = call i64 @add()
  store i64 %"61", ptr addrspace(5) %"60", align 8
  br label %"49"

"49":                                             ; preds = %"48"
  %2 = load i64, ptr addrspace(5) %"60", align 8
  ret i64 %2
}

define amdgpu_kernel void @shared_unify_extern(ptr addrspace(4) byref(i64) %"62", ptr addrspace(4) byref(i64) %"63") #1 {
  %"64" = alloca i64, align 8, addrspace(5)
  %"65" = alloca i64, align 8, addrspace(5)
  %"66" = alloca i64, align 8, addrspace(5)
  %"67" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"50"

"50":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"62", align 8
  store i64 %2, ptr addrspace(5) %"64", align 8
  %3 = load i64, ptr addrspace(4) %"63", align 8
  store i64 %3, ptr addrspace(5) %"65", align 8
  %4 = load i64, ptr addrspace(5) %"64", align 8
  %"84" = inttoptr i64 %4 to ptr addrspace(1)
  %5 = load i64, ptr addrspace(1) %"84", align 8
  store i64 %5, ptr addrspace(5) %"66", align 8
  %6 = load i64, ptr addrspace(5) %"64", align 8
  %"85" = inttoptr i64 %6 to ptr addrspace(1)
  %"46" = getelementptr inbounds i8, ptr addrspace(1) %"85", i64 8
  %7 = load i64, ptr addrspace(1) %"46", align 8
  store i64 %7, ptr addrspace(5) %"67", align 8
  %8 = load i64, ptr addrspace(5) %"67", align 8
  store i64 %8, ptr addrspace(3) @shared_mod, align 8
  %9 = load i64, ptr addrspace(5) %"66", align 8
  %"87" = call i64 @set_shared_temp1(i64 %9)
  store i64 %"87", ptr addrspace(5) %"67", align 8
  br label %"51"

"51":                                             ; preds = %"50"
  %10 = load i64, ptr addrspace(5) %"65", align 8
  %11 = load i64, ptr addrspace(5) %"67", align 8
  %"89" = inttoptr i64 %10 to ptr
  store i64 %11, ptr %"89", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }