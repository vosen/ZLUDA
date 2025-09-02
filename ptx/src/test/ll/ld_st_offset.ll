@0 = addrspace(4) global i64 4
@1 = addrspace(4) global i64 4

define amdgpu_kernel void @ld_st_offset(ptr addrspace(4) byref(i64) %"40", ptr addrspace(4) byref(i64) %"41") #0 {
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i32, align 4, addrspace(5)
  %"45" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"39"

"39":                                             ; preds = %1
  %"46" = load i64, ptr addrspace(4) %"40", align 8
  store i64 %"46", ptr addrspace(5) %"42", align 8
  %"47" = load i64, ptr addrspace(4) %"41", align 8
  store i64 %"47", ptr addrspace(5) %"43", align 8
  %"49" = load i64, ptr addrspace(5) %"42", align 8
  %"56" = inttoptr i64 %"49" to ptr
  %"48" = load i32, ptr %"56", align 4
  store i32 %"48", ptr addrspace(5) %"44", align 4
  %"34" = load i64, ptr addrspace(4) @0, align 8
  %"50" = load i64, ptr addrspace(5) %"42", align 8
  %"57" = inttoptr i64 %"50" to ptr
  %"35" = getelementptr inbounds i8, ptr %"57", i64 %"34"
  %"51" = load i32, ptr %"35", align 4
  store i32 %"51", ptr addrspace(5) %"45", align 4
  %"52" = load i64, ptr addrspace(5) %"43", align 8
  %"53" = load i32, ptr addrspace(5) %"45", align 4
  %"58" = inttoptr i64 %"52" to ptr
  store i32 %"53", ptr %"58", align 4
  %"37" = load i64, ptr addrspace(4) @1, align 8
  %"54" = load i64, ptr addrspace(5) %"43", align 8
  %"59" = inttoptr i64 %"54" to ptr
  %"38" = getelementptr inbounds i8, ptr %"59", i64 %"37"
  %"55" = load i32, ptr addrspace(5) %"44", align 4
  store i32 %"55", ptr %"38", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }