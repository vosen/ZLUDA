@shared_mem1 = external addrspace(3) global [128 x i8], align 4

define amdgpu_kernel void @shared_ptr_32(ptr addrspace(4) byref(i64) %"38", ptr addrspace(4) byref(i64) %"39") #0 {
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i32, align 4, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  store i64 0, ptr addrspace(5) %"52", align 4
  br label %1

1:                                                ; preds = %0
  br label %"37"

"37":                                             ; preds = %1
  %"45" = load i64, ptr addrspace(4) %"38", align 8
  store i64 %"45", ptr addrspace(5) %"40", align 8
  %"46" = load i64, ptr addrspace(4) %"39", align 8
  store i64 %"46", ptr addrspace(5) %"41", align 8
  store i32 ptrtoint (ptr addrspace(3) @shared_mem1 to i32), ptr addrspace(5) %"42", align 4
  %"49" = load i64, ptr addrspace(5) %"40", align 8
  %"59" = inttoptr i64 %"49" to ptr addrspace(1)
  %"48" = load i64, ptr addrspace(1) %"59", align 8
  store i64 %"48", ptr addrspace(5) %"43", align 8
  %"50" = load i32, ptr addrspace(5) %"42", align 4
  %"51" = load i64, ptr addrspace(5) %"43", align 8
  %"60" = inttoptr i32 %"50" to ptr addrspace(3)
  store i64 %"51", ptr addrspace(3) %"60", align 8
  %"53" = load i32, ptr addrspace(5) %"42", align 4
  %"54" = load i64, ptr addrspace(5) %"52", align 8
  %"61" = inttoptr i32 %"53" to ptr addrspace(3)
  %"36" = getelementptr inbounds i8, ptr addrspace(3) %"61", i64 %"54"
  %"55" = load i64, ptr addrspace(3) %"36", align 8
  store i64 %"55", ptr addrspace(5) %"44", align 8
  %"56" = load i64, ptr addrspace(5) %"41", align 8
  %"57" = load i64, ptr addrspace(5) %"44", align 8
  %"62" = inttoptr i64 %"56" to ptr addrspace(1)
  store i64 %"57", ptr addrspace(1) %"62", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }