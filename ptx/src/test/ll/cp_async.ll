@from = addrspace(1) global [4 x i32] [i32 1, i32 2, i32 3, i32 4]
@to = external addrspace(3) global [4 x i32]
@0 = addrspace(4) global i64 0
@1 = addrspace(4) global i64 4
@2 = addrspace(4) global i64 8
@3 = addrspace(4) global i64 12
@4 = addrspace(4) global i64 4
@5 = addrspace(4) global i64 8
@6 = addrspace(4) global i64 12

define amdgpu_kernel void @cp_async(ptr addrspace(4) byref(i64) %"58", ptr addrspace(4) byref(i64) %"59") #0 {
  %"60" = alloca i64, align 8, addrspace(5)
  %"61" = alloca i64, align 8, addrspace(5)
  %"62" = alloca i32, align 4, addrspace(5)
  %"63" = alloca i32, align 4, addrspace(5)
  %"64" = alloca i32, align 4, addrspace(5)
  %"65" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"57"

"57":                                             ; preds = %1
  %"66" = load i64, ptr addrspace(4) %"58", align 8
  store i64 %"66", ptr addrspace(5) %"60", align 8
  %"67" = load i64, ptr addrspace(4) %"59", align 8
  store i64 %"67", ptr addrspace(5) %"61", align 8
  %2 = load i96, ptr addrspace(1) @from, align 128
  %3 = zext i96 %2 to i128
  store i128 %3, ptr addrspace(3) @to, align 4
  %"38" = load i64, ptr addrspace(4) @0, align 8
  %"68" = load i32, ptr addrspacecast (ptr addrspace(3) @to to ptr), align 4
  store i32 %"68", ptr addrspace(5) %"62", align 4
  %"40" = load i64, ptr addrspace(4) @1, align 8
  %"41" = getelementptr inbounds i8, ptr addrspacecast (ptr addrspace(3) @to to ptr), i64 %"40"
  %"69" = load i32, ptr %"41", align 4
  store i32 %"69", ptr addrspace(5) %"63", align 4
  %"43" = load i64, ptr addrspace(4) @2, align 8
  %"44" = getelementptr inbounds i8, ptr addrspacecast (ptr addrspace(3) @to to ptr), i64 %"43"
  %"70" = load i32, ptr %"44", align 4
  store i32 %"70", ptr addrspace(5) %"64", align 4
  %"46" = load i64, ptr addrspace(4) @3, align 8
  %"47" = getelementptr inbounds i8, ptr addrspacecast (ptr addrspace(3) @to to ptr), i64 %"46"
  %"71" = load i32, ptr %"47", align 4
  store i32 %"71", ptr addrspace(5) %"65", align 4
  %"72" = load i64, ptr addrspace(5) %"61", align 8
  %"73" = load i32, ptr addrspace(5) %"62", align 4
  %"86" = inttoptr i64 %"72" to ptr
  store i32 %"73", ptr %"86", align 4
  %"49" = load i64, ptr addrspace(4) @4, align 8
  %"74" = load i64, ptr addrspace(5) %"61", align 8
  %"87" = inttoptr i64 %"74" to ptr
  %"50" = getelementptr inbounds i8, ptr %"87", i64 %"49"
  %"75" = load i32, ptr addrspace(5) %"63", align 4
  store i32 %"75", ptr %"50", align 4
  %"52" = load i64, ptr addrspace(4) @5, align 8
  %"76" = load i64, ptr addrspace(5) %"61", align 8
  %"88" = inttoptr i64 %"76" to ptr
  %"53" = getelementptr inbounds i8, ptr %"88", i64 %"52"
  %"77" = load i32, ptr addrspace(5) %"64", align 4
  store i32 %"77", ptr %"53", align 4
  %"55" = load i64, ptr addrspace(4) @6, align 8
  %"78" = load i64, ptr addrspace(5) %"61", align 8
  %"89" = inttoptr i64 %"78" to ptr
  %"56" = getelementptr inbounds i8, ptr %"89", i64 %"55"
  %"79" = load i32, ptr addrspace(5) %"65", align 4
  store i32 %"79", ptr %"56", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }