@shared_mem = external addrspace(3) global [0 x i32], align 4
@0 = addrspace(4) global i64 2

define hidden void @incr_shared_2_global() #0 {
  %"40" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"37"

"37":                                             ; preds = %1
  %"41" = load i64, ptr addrspace(3) @shared_mem, align 8
  store i64 %"41", ptr addrspace(5) %"40", align 8
  %"36" = load i64, ptr addrspace(4) @0, align 8
  %"43" = load i64, ptr addrspace(5) %"40", align 8
  %"42" = add i64 %"43", %"36"
  store i64 %"42", ptr addrspace(5) %"40", align 8
  %"44" = load i64, ptr addrspace(5) %"40", align 8
  store i64 %"44", ptr addrspace(3) @shared_mem, align 8
  ret void
}

define amdgpu_kernel void @extern_shared_call(ptr addrspace(4) byref(i64) %"45", ptr addrspace(4) byref(i64) %"46") #1 {
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"38"

"38":                                             ; preds = %1
  %"50" = load i64, ptr addrspace(4) %"45", align 8
  store i64 %"50", ptr addrspace(5) %"47", align 8
  %"51" = load i64, ptr addrspace(4) %"46", align 8
  store i64 %"51", ptr addrspace(5) %"48", align 8
  %"53" = load i64, ptr addrspace(5) %"47", align 8
  %"60" = inttoptr i64 %"53" to ptr addrspace(1)
  %"52" = load i64, ptr addrspace(1) %"60", align 8
  store i64 %"52", ptr addrspace(5) %"49", align 8
  %"54" = load i64, ptr addrspace(5) %"49", align 8
  store i64 %"54", ptr addrspace(3) @shared_mem, align 8
  call void @incr_shared_2_global()
  br label %"39"

"39":                                             ; preds = %"38"
  %"55" = load i64, ptr addrspace(3) @shared_mem, align 8
  store i64 %"55", ptr addrspace(5) %"49", align 8
  %"56" = load i64, ptr addrspace(5) %"48", align 8
  %"57" = load i64, ptr addrspace(5) %"49", align 8
  %"63" = inttoptr i64 %"56" to ptr addrspace(1)
  store i64 %"57", ptr addrspace(1) %"63", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }