@shared_mem1 = external addrspace(3) global [128 x i8], align 4

define amdgpu_kernel void @shared_ptr_32(ptr addrspace(4) byref(i64) %"41", ptr addrspace(4) byref(i64) %"42") #0 {
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i32, align 4, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"40"

"40":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"41", align 8
  store i64 %2, ptr addrspace(5) %"43", align 8
  %3 = load i64, ptr addrspace(4) %"42", align 8
  store i64 %3, ptr addrspace(5) %"44", align 8
  store i32 ptrtoint (ptr addrspace(3) @shared_mem1 to i32), ptr addrspace(5) %"45", align 4
  %4 = load i64, ptr addrspace(5) %"43", align 8
  %"60" = inttoptr i64 %4 to ptr addrspace(1)
  %5 = load i64, ptr addrspace(1) %"60", align 8
  store i64 %5, ptr addrspace(5) %"46", align 8
  %6 = load i32, ptr addrspace(5) %"45", align 4
  %7 = load i64, ptr addrspace(5) %"46", align 8
  %"61" = inttoptr i32 %6 to ptr addrspace(3)
  store i64 %7, ptr addrspace(3) %"61", align 8
  %8 = load i32, ptr addrspace(5) %"45", align 4
  %"62" = inttoptr i32 %8 to ptr addrspace(3)
  %"39" = getelementptr inbounds i8, ptr addrspace(3) %"62", i64 0
  %9 = load i64, ptr addrspace(3) %"39", align 8
  store i64 %9, ptr addrspace(5) %"47", align 8
  %10 = load i64, ptr addrspace(5) %"44", align 8
  %11 = load i64, ptr addrspace(5) %"47", align 8
  %"63" = inttoptr i64 %10 to ptr addrspace(1)
  store i64 %11, ptr addrspace(1) %"63", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }