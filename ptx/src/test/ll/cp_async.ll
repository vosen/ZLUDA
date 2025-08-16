@from = addrspace(1) global [4 x i32] [i32 1, i32 2, i32 3, i32 4]
@to = external addrspace(3) global [4 x i32]

define amdgpu_kernel void @cp_async(ptr addrspace(4) byref(i64) %"48", ptr addrspace(4) byref(i64) %"49") #0 {
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i32, align 4, addrspace(5)
  %"53" = alloca i32, align 4, addrspace(5)
  %"54" = alloca i32, align 4, addrspace(5)
  %"55" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"47"

"47":                                             ; preds = %1
  %"56" = load i64, ptr addrspace(4) %"48", align 8
  store i64 %"56", ptr addrspace(5) %"50", align 8
  %"57" = load i64, ptr addrspace(4) %"49", align 8
  store i64 %"57", ptr addrspace(5) %"51", align 8
  %2 = load i96, ptr addrspace(1) @from, align 128
  %3 = zext i96 %2 to i128
  store i128 %3, ptr addrspace(3) @to, align 4
  %"58" = load i32, ptr addrspacecast (ptr addrspace(3) @to to ptr), align 4
  store i32 %"58", ptr addrspace(5) %"52", align 4
  %"59" = load i32, ptr getelementptr inbounds (i8, ptr addrspacecast (ptr addrspace(3) @to to ptr), i64 4), align 4
  store i32 %"59", ptr addrspace(5) %"53", align 4
  %"60" = load i32, ptr getelementptr inbounds (i8, ptr addrspacecast (ptr addrspace(3) @to to ptr), i64 8), align 4
  store i32 %"60", ptr addrspace(5) %"54", align 4
  %"61" = load i32, ptr getelementptr inbounds (i8, ptr addrspacecast (ptr addrspace(3) @to to ptr), i64 12), align 4
  store i32 %"61", ptr addrspace(5) %"55", align 4
  %"62" = load i64, ptr addrspace(5) %"51", align 8
  %"63" = load i32, ptr addrspace(5) %"52", align 4
  %"76" = inttoptr i64 %"62" to ptr
  store i32 %"63", ptr %"76", align 4
  %"64" = load i64, ptr addrspace(5) %"51", align 8
  %"77" = inttoptr i64 %"64" to ptr
  %"42" = getelementptr inbounds i8, ptr %"77", i64 4
  %"65" = load i32, ptr addrspace(5) %"53", align 4
  store i32 %"65", ptr %"42", align 4
  %"66" = load i64, ptr addrspace(5) %"51", align 8
  %"78" = inttoptr i64 %"66" to ptr
  %"44" = getelementptr inbounds i8, ptr %"78", i64 8
  %"67" = load i32, ptr addrspace(5) %"54", align 4
  store i32 %"67", ptr %"44", align 4
  %"68" = load i64, ptr addrspace(5) %"51", align 8
  %"79" = inttoptr i64 %"68" to ptr
  %"46" = getelementptr inbounds i8, ptr %"79", i64 12
  %"69" = load i32, ptr addrspace(5) %"55", align 4
  store i32 %"69", ptr %"46", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }