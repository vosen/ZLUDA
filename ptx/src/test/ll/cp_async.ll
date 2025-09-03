@from = addrspace(1) global [4 x i32] [i32 1, i32 2, i32 3, i32 4]
@to = external addrspace(3) global [4 x i32]

define amdgpu_kernel void @cp_async(ptr addrspace(4) byref(i64) %"51", ptr addrspace(4) byref(i64) %"52") #0 {
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  %"55" = alloca i32, align 4, addrspace(5)
  %"56" = alloca i32, align 4, addrspace(5)
  %"57" = alloca i32, align 4, addrspace(5)
  %"58" = alloca i32, align 4, addrspace(5)
  %"61" = alloca i64, align 8, addrspace(5)
  store i64 0, ptr addrspace(5) %"61", align 4
  %"64" = alloca i64, align 8, addrspace(5)
  store i64 4, ptr addrspace(5) %"64", align 4
  %"67" = alloca i64, align 8, addrspace(5)
  store i64 8, ptr addrspace(5) %"67", align 4
  %"70" = alloca i64, align 8, addrspace(5)
  store i64 12, ptr addrspace(5) %"70", align 4
  %"75" = alloca i64, align 8, addrspace(5)
  store i64 4, ptr addrspace(5) %"75", align 4
  %"79" = alloca i64, align 8, addrspace(5)
  store i64 8, ptr addrspace(5) %"79", align 4
  %"83" = alloca i64, align 8, addrspace(5)
  store i64 12, ptr addrspace(5) %"83", align 4
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
  %"62" = load i64, ptr addrspace(5) %"61", align 8
  %"63" = load i32, ptr addrspacecast (ptr addrspace(3) @to to ptr), align 4
  store i32 %"63", ptr addrspace(5) %"55", align 4
  %"65" = load i64, ptr addrspace(5) %"64", align 8
  %"39" = getelementptr inbounds i8, ptr addrspacecast (ptr addrspace(3) @to to ptr), i64 %"65"
  %"66" = load i32, ptr %"39", align 4
  store i32 %"66", ptr addrspace(5) %"56", align 4
  %"68" = load i64, ptr addrspace(5) %"67", align 8
  %"41" = getelementptr inbounds i8, ptr addrspacecast (ptr addrspace(3) @to to ptr), i64 %"68"
  %"69" = load i32, ptr %"41", align 4
  store i32 %"69", ptr addrspace(5) %"57", align 4
  %"71" = load i64, ptr addrspace(5) %"70", align 8
  %"43" = getelementptr inbounds i8, ptr addrspacecast (ptr addrspace(3) @to to ptr), i64 %"71"
  %"72" = load i32, ptr %"43", align 4
  store i32 %"72", ptr addrspace(5) %"58", align 4
  %"73" = load i64, ptr addrspace(5) %"54", align 8
  %"74" = load i32, ptr addrspace(5) %"55", align 4
  %"93" = inttoptr i64 %"73" to ptr
  store i32 %"74", ptr %"93", align 4
  %"76" = load i64, ptr addrspace(5) %"54", align 8
  %"77" = load i64, ptr addrspace(5) %"75", align 8
  %"94" = inttoptr i64 %"76" to ptr
  %"45" = getelementptr inbounds i8, ptr %"94", i64 %"77"
  %"78" = load i32, ptr addrspace(5) %"56", align 4
  store i32 %"78", ptr %"45", align 4
  %"80" = load i64, ptr addrspace(5) %"54", align 8
  %"81" = load i64, ptr addrspace(5) %"79", align 8
  %"95" = inttoptr i64 %"80" to ptr
  %"47" = getelementptr inbounds i8, ptr %"95", i64 %"81"
  %"82" = load i32, ptr addrspace(5) %"57", align 4
  store i32 %"82", ptr %"47", align 4
  %"84" = load i64, ptr addrspace(5) %"54", align 8
  %"85" = load i64, ptr addrspace(5) %"83", align 8
  %"96" = inttoptr i64 %"84" to ptr
  %"49" = getelementptr inbounds i8, ptr %"96", i64 %"85"
  %"86" = load i32, ptr addrspace(5) %"58", align 4
  store i32 %"86", ptr %"49", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }