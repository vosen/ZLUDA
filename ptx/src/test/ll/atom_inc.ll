@0 = addrspace(4) global i32 101
@1 = addrspace(4) global i32 101
@2 = addrspace(4) global i64 4
@3 = addrspace(4) global i64 8

define amdgpu_kernel void @atom_inc(ptr addrspace(4) byref(i64) %"45", ptr addrspace(4) byref(i64) %"46") #0 {
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i32, align 4, addrspace(5)
  %"50" = alloca i32, align 4, addrspace(5)
  %"51" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"44"

"44":                                             ; preds = %1
  %"52" = load i64, ptr addrspace(4) %"45", align 8
  store i64 %"52", ptr addrspace(5) %"47", align 8
  %"53" = load i64, ptr addrspace(4) %"46", align 8
  store i64 %"53", ptr addrspace(5) %"48", align 8
  %"35" = load i32, ptr addrspace(4) @0, align 4
  %"55" = load i64, ptr addrspace(5) %"47", align 8
  %"66" = inttoptr i64 %"55" to ptr
  %2 = atomicrmw uinc_wrap ptr %"66", i32 %"35" syncscope("agent-one-as") monotonic, align 4
  store i32 %2, ptr addrspace(5) %"49", align 4
  %"37" = load i32, ptr addrspace(4) @1, align 4
  %"57" = load i64, ptr addrspace(5) %"47", align 8
  %"67" = inttoptr i64 %"57" to ptr addrspace(1)
  %3 = atomicrmw uinc_wrap ptr addrspace(1) %"67", i32 %"37" syncscope("agent-one-as") monotonic, align 4
  store i32 %3, ptr addrspace(5) %"50", align 4
  %"59" = load i64, ptr addrspace(5) %"47", align 8
  %"68" = inttoptr i64 %"59" to ptr
  %"58" = load i32, ptr %"68", align 4
  store i32 %"58", ptr addrspace(5) %"51", align 4
  %"60" = load i64, ptr addrspace(5) %"48", align 8
  %"61" = load i32, ptr addrspace(5) %"49", align 4
  %"69" = inttoptr i64 %"60" to ptr
  store i32 %"61", ptr %"69", align 4
  %"39" = load i64, ptr addrspace(4) @2, align 8
  %"62" = load i64, ptr addrspace(5) %"48", align 8
  %"70" = inttoptr i64 %"62" to ptr
  %"40" = getelementptr inbounds i8, ptr %"70", i64 %"39"
  %"63" = load i32, ptr addrspace(5) %"50", align 4
  store i32 %"63", ptr %"40", align 4
  %"42" = load i64, ptr addrspace(4) @3, align 8
  %"64" = load i64, ptr addrspace(5) %"48", align 8
  %"71" = inttoptr i64 %"64" to ptr
  %"43" = getelementptr inbounds i8, ptr %"71", i64 %"42"
  %"65" = load i32, ptr addrspace(5) %"51", align 4
  store i32 %"65", ptr %"43", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }