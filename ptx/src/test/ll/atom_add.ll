@shared_mem = external addrspace(3) global [1024 x i8], align 4

define amdgpu_kernel void @atom_add(ptr addrspace(4) byref(i64) %"36", ptr addrspace(4) byref(i64) %"37") #0 {
  %"38" = alloca i64, align 8, addrspace(5)
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i32, align 4, addrspace(5)
  %"41" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"35"

"35":                                             ; preds = %1
  %"42" = load i64, ptr addrspace(4) %"36", align 8
  store i64 %"42", ptr addrspace(5) %"38", align 8
  %"43" = load i64, ptr addrspace(4) %"37", align 8
  store i64 %"43", ptr addrspace(5) %"39", align 8
  %"45" = load i64, ptr addrspace(5) %"38", align 8
  %"56" = inttoptr i64 %"45" to ptr
  %"44" = load i32, ptr %"56", align 4
  store i32 %"44", ptr addrspace(5) %"40", align 4
  %"46" = load i64, ptr addrspace(5) %"38", align 8
  %"57" = inttoptr i64 %"46" to ptr
  %"32" = getelementptr inbounds i8, ptr %"57", i64 4
  %"47" = load i32, ptr %"32", align 4
  store i32 %"47", ptr addrspace(5) %"41", align 4
  %"48" = load i32, ptr addrspace(5) %"40", align 4
  store i32 %"48", ptr addrspace(3) @shared_mem, align 4
  %"50" = load i32, ptr addrspace(5) %"41", align 4
  %2 = atomicrmw add ptr addrspace(3) @shared_mem, i32 %"50" syncscope("agent-one-as") monotonic, align 4
  store i32 %2, ptr addrspace(5) %"40", align 4
  %"51" = load i32, ptr addrspace(3) @shared_mem, align 4
  store i32 %"51", ptr addrspace(5) %"41", align 4
  %"52" = load i64, ptr addrspace(5) %"39", align 8
  %"53" = load i32, ptr addrspace(5) %"40", align 4
  %"61" = inttoptr i64 %"52" to ptr
  store i32 %"53", ptr %"61", align 4
  %"54" = load i64, ptr addrspace(5) %"39", align 8
  %"62" = inttoptr i64 %"54" to ptr
  %"34" = getelementptr inbounds i8, ptr %"62", i64 4
  %"55" = load i32, ptr addrspace(5) %"41", align 4
  store i32 %"55", ptr %"34", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }