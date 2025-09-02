@shared_mem1 = external addrspace(3) global [128 x i8], align 4
@0 = addrspace(4) global i64 0

define amdgpu_kernel void @shared_ptr_32(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #0 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i32, align 4, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"38"

"38":                                             ; preds = %1
  %"46" = load i64, ptr addrspace(4) %"39", align 8
  store i64 %"46", ptr addrspace(5) %"41", align 8
  %"47" = load i64, ptr addrspace(4) %"40", align 8
  store i64 %"47", ptr addrspace(5) %"42", align 8
  store i32 ptrtoint (ptr addrspace(3) @shared_mem1 to i32), ptr addrspace(5) %"43", align 4
  %"50" = load i64, ptr addrspace(5) %"41", align 8
  %"58" = inttoptr i64 %"50" to ptr addrspace(1)
  %"49" = load i64, ptr addrspace(1) %"58", align 8
  store i64 %"49", ptr addrspace(5) %"44", align 8
  %"51" = load i32, ptr addrspace(5) %"43", align 4
  %"52" = load i64, ptr addrspace(5) %"44", align 8
  %"59" = inttoptr i32 %"51" to ptr addrspace(3)
  store i64 %"52", ptr addrspace(3) %"59", align 8
  %"36" = load i64, ptr addrspace(4) @0, align 8
  %"53" = load i32, ptr addrspace(5) %"43", align 4
  %"60" = inttoptr i32 %"53" to ptr addrspace(3)
  %"37" = getelementptr inbounds i8, ptr addrspace(3) %"60", i64 %"36"
  %"54" = load i64, ptr addrspace(3) %"37", align 8
  store i64 %"54", ptr addrspace(5) %"45", align 8
  %"55" = load i64, ptr addrspace(5) %"42", align 8
  %"56" = load i64, ptr addrspace(5) %"45", align 8
  %"61" = inttoptr i64 %"55" to ptr addrspace(1)
  store i64 %"56", ptr addrspace(1) %"61", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }