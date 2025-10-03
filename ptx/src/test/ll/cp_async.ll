@from = addrspace(1) global [4 x i32] [i32 1, i32 2, i32 3, i32 4]
@to = external addrspace(3) global [4 x i32]

define amdgpu_kernel void @cp_async(ptr addrspace(4) byref(i64) %"51", ptr addrspace(4) byref(i64) %"52") #0 {
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  %"55" = alloca i32, align 4, addrspace(5)
  %"56" = alloca i32, align 4, addrspace(5)
  %"57" = alloca i32, align 4, addrspace(5)
  %"58" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"50"

"50":                                             ; preds = %1
  %"59" = load i64, ptr addrspace(4) %"51", align 8
  store i64 %"59", ptr addrspace(5) %"53", align 8
  %"60" = load i64, ptr addrspace(4) %"52", align 8
  store i64 %"60", ptr addrspace(5) %"54", align 8
  %2 = load i96, ptr addrspace(1) @from, align 128
  %3 = zext i96 %2 to i128
  store i128 %3, ptr addrspace(3) @to, align 4
  %"61" = load i32, ptr addrspacecast (ptr addrspace(3) @to to ptr), align 4
  store i32 %"61", ptr addrspace(5) %"55", align 4
  %"62" = load i32, ptr getelementptr inbounds (i8, ptr addrspacecast (ptr addrspace(3) @to to ptr), i64 4), align 4
  store i32 %"62", ptr addrspace(5) %"56", align 4
  %"63" = load i32, ptr getelementptr inbounds (i8, ptr addrspacecast (ptr addrspace(3) @to to ptr), i64 8), align 4
  store i32 %"63", ptr addrspace(5) %"57", align 4
  %"64" = load i32, ptr getelementptr inbounds (i8, ptr addrspacecast (ptr addrspace(3) @to to ptr), i64 12), align 4
  store i32 %"64", ptr addrspace(5) %"58", align 4
  %"65" = load i64, ptr addrspace(5) %"54", align 8
  %"66" = load i32, ptr addrspace(5) %"55", align 4
  %"79" = inttoptr i64 %"65" to ptr
  store i32 %"66", ptr %"79", align 4
  %"67" = load i64, ptr addrspace(5) %"54", align 8
  %"80" = inttoptr i64 %"67" to ptr
  %"45" = getelementptr inbounds i8, ptr %"80", i64 4
  %"68" = load i32, ptr addrspace(5) %"56", align 4
  store i32 %"68", ptr %"45", align 4
  %"69" = load i64, ptr addrspace(5) %"54", align 8
  %"81" = inttoptr i64 %"69" to ptr
  %"47" = getelementptr inbounds i8, ptr %"81", i64 8
  %"70" = load i32, ptr addrspace(5) %"57", align 4
  store i32 %"70", ptr %"47", align 4
  %"71" = load i64, ptr addrspace(5) %"54", align 8
  %"82" = inttoptr i64 %"71" to ptr
  %"49" = getelementptr inbounds i8, ptr %"82", i64 12
  %"72" = load i32, ptr addrspace(5) %"58", align 4
  store i32 %"72", ptr %"49", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
