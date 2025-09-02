@shared_mem = external addrspace(3) global [1024 x i8], align 4
@0 = addrspace(4) global i64 4
@1 = addrspace(4) global i64 4

define amdgpu_kernel void @atom_add(ptr addrspace(4) byref(i64) %"41", ptr addrspace(4) byref(i64) %"42") #0 {
  %"43" = alloca i64, align 8, addrspace(5)
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i32, align 4, addrspace(5)
  %"46" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"40"

"40":                                             ; preds = %1
  %"47" = load i64, ptr addrspace(4) %"41", align 8
  store i64 %"47", ptr addrspace(5) %"43", align 8
  %"48" = load i64, ptr addrspace(4) %"42", align 8
  store i64 %"48", ptr addrspace(5) %"44", align 8
  %"50" = load i64, ptr addrspace(5) %"43", align 8
  %"61" = inttoptr i64 %"50" to ptr
  %"49" = load i32, ptr %"61", align 4
  store i32 %"49", ptr addrspace(5) %"45", align 4
  %"35" = load i64, ptr addrspace(4) @0, align 8
  %"51" = load i64, ptr addrspace(5) %"43", align 8
  %"62" = inttoptr i64 %"51" to ptr
  %"36" = getelementptr inbounds i8, ptr %"62", i64 %"35"
  %"52" = load i32, ptr %"36", align 4
  store i32 %"52", ptr addrspace(5) %"46", align 4
  %"53" = load i32, ptr addrspace(5) %"45", align 4
  store i32 %"53", ptr addrspace(3) @shared_mem, align 4
  %"55" = load i32, ptr addrspace(5) %"46", align 4
  %2 = atomicrmw add ptr addrspace(3) @shared_mem, i32 %"55" syncscope("agent-one-as") monotonic, align 4
  store i32 %2, ptr addrspace(5) %"45", align 4
  %"56" = load i32, ptr addrspace(3) @shared_mem, align 4
  store i32 %"56", ptr addrspace(5) %"46", align 4
  %"57" = load i64, ptr addrspace(5) %"44", align 8
  %"58" = load i32, ptr addrspace(5) %"45", align 4
  %"66" = inttoptr i64 %"57" to ptr
  store i32 %"58", ptr %"66", align 4
  %"38" = load i64, ptr addrspace(4) @1, align 8
  %"59" = load i64, ptr addrspace(5) %"44", align 8
  %"67" = inttoptr i64 %"59" to ptr
  %"39" = getelementptr inbounds i8, ptr %"67", i64 %"38"
  %"60" = load i32, ptr addrspace(5) %"46", align 4
  store i32 %"60", ptr %"39", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }