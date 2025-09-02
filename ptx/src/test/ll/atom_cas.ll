@0 = addrspace(4) global i64 4
@1 = addrspace(4) global i32 100
@2 = addrspace(4) global i64 4
@3 = addrspace(4) global i64 4

define amdgpu_kernel void @atom_cas(ptr addrspace(4) byref(i64) %"45", ptr addrspace(4) byref(i64) %"46") #0 {
  %"47" = alloca i64, align 8, addrspace(5)
  %"48" = alloca i64, align 8, addrspace(5)
  %"49" = alloca i32, align 4, addrspace(5)
  %"50" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"44"

"44":                                             ; preds = %1
  %"51" = load i64, ptr addrspace(4) %"45", align 8
  store i64 %"51", ptr addrspace(5) %"47", align 8
  %"52" = load i64, ptr addrspace(4) %"46", align 8
  store i64 %"52", ptr addrspace(5) %"48", align 8
  %"54" = load i64, ptr addrspace(5) %"47", align 8
  %"64" = inttoptr i64 %"54" to ptr
  %"53" = load i32, ptr %"64", align 4
  store i32 %"53", ptr addrspace(5) %"49", align 4
  %"34" = load i64, ptr addrspace(4) @0, align 8
  %"55" = load i64, ptr addrspace(5) %"47", align 8
  %"65" = inttoptr i64 %"55" to ptr
  %"35" = getelementptr inbounds i8, ptr %"65", i64 %"34"
  %"37" = load i32, ptr addrspace(4) @1, align 4
  %"57" = load i32, ptr addrspace(5) %"49", align 4
  %2 = cmpxchg ptr %"35", i32 %"57", i32 %"37" syncscope("agent-one-as") monotonic monotonic, align 4
  %"66" = extractvalue { i32, i1 } %2, 0
  store i32 %"66", ptr addrspace(5) %"49", align 4
  %"39" = load i64, ptr addrspace(4) @2, align 8
  %"58" = load i64, ptr addrspace(5) %"47", align 8
  %"68" = inttoptr i64 %"58" to ptr
  %"40" = getelementptr inbounds i8, ptr %"68", i64 %"39"
  %"59" = load i32, ptr %"40", align 4
  store i32 %"59", ptr addrspace(5) %"50", align 4
  %"60" = load i64, ptr addrspace(5) %"48", align 8
  %"61" = load i32, ptr addrspace(5) %"49", align 4
  %"69" = inttoptr i64 %"60" to ptr
  store i32 %"61", ptr %"69", align 4
  %"42" = load i64, ptr addrspace(4) @3, align 8
  %"62" = load i64, ptr addrspace(5) %"48", align 8
  %"70" = inttoptr i64 %"62" to ptr
  %"43" = getelementptr inbounds i8, ptr %"70", i64 %"42"
  %"63" = load i32, ptr addrspace(5) %"50", align 4
  store i32 %"63", ptr %"43", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }