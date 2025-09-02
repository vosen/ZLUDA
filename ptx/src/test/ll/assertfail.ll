@0 = addrspace(4) global i32 0
@1 = addrspace(4) global i64 0
@2 = addrspace(4) global i64 0
@3 = addrspace(4) global i64 0
@4 = addrspace(4) global i64 0
@5 = addrspace(4) global i64 0
@6 = addrspace(4) global i64 1

declare hidden void @__zluda_ptx_impl___assertfail(i64, i64, i32, i64, i64) #0

define amdgpu_kernel void @assertfail(ptr addrspace(4) byref(i64) %"96", ptr addrspace(4) byref(i64) %"97") #1 {
  %"98" = alloca i64, align 8, addrspace(5)
  %"99" = alloca i64, align 8, addrspace(5)
  %"100" = alloca i64, align 8, addrspace(5)
  %"101" = alloca i64, align 8, addrspace(5)
  %"104" = alloca i32, align 4, addrspace(5)
  %"106" = alloca i64, align 8, addrspace(5)
  %"109" = alloca i64, align 8, addrspace(5)
  %"112" = alloca i32, align 4, addrspace(5)
  %"115" = alloca i64, align 8, addrspace(5)
  %"118" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"94"

"94":                                             ; preds = %1
  %"102" = load i64, ptr addrspace(4) %"96", align 8
  store i64 %"102", ptr addrspace(5) %"98", align 8
  %"103" = load i64, ptr addrspace(4) %"97", align 8
  store i64 %"103", ptr addrspace(5) %"99", align 8
  %"56" = load i32, ptr addrspace(4) @0, align 4
  store i32 %"56", ptr addrspace(5) %"104", align 4
  %"58" = load i64, ptr addrspace(4) @1, align 8
  %"107" = getelementptr inbounds i8, ptr addrspace(5) %"106", i64 %"58"
  %"108" = load i64, ptr addrspace(5) %"98", align 8
  store i64 %"108", ptr addrspace(5) %"107", align 8
  %"61" = load i64, ptr addrspace(4) @2, align 8
  %"110" = getelementptr inbounds i8, ptr addrspace(5) %"109", i64 %"61"
  %"111" = load i64, ptr addrspace(5) %"98", align 8
  store i64 %"111", ptr addrspace(5) %"110", align 8
  %"64" = load i64, ptr addrspace(4) @3, align 8
  %"113" = getelementptr inbounds i8, ptr addrspace(5) %"112", i64 %"64"
  %"114" = load i32, ptr addrspace(5) %"104", align 4
  store i32 %"114", ptr addrspace(5) %"113", align 4
  %"67" = load i64, ptr addrspace(4) @4, align 8
  %"116" = getelementptr inbounds i8, ptr addrspace(5) %"115", i64 %"67"
  %"117" = load i64, ptr addrspace(5) %"98", align 8
  store i64 %"117", ptr addrspace(5) %"116", align 8
  %"70" = load i64, ptr addrspace(4) @5, align 8
  %"119" = getelementptr inbounds i8, ptr addrspace(5) %"118", i64 %"70"
  %"120" = load i64, ptr addrspace(5) %"98", align 8
  store i64 %"120", ptr addrspace(5) %"119", align 8
  %"84" = load i64, ptr addrspace(5) %"106", align 8
  %"85" = load i64, ptr addrspace(5) %"109", align 8
  %"86" = load i32, ptr addrspace(5) %"112", align 4
  %"87" = load i64, ptr addrspace(5) %"115", align 8
  %"88" = load i64, ptr addrspace(5) %"118", align 8
  call void @__zluda_ptx_impl___assertfail(i64 %"84", i64 %"85", i32 %"86", i64 %"87", i64 %"88")
  br label %"95"

"95":                                             ; preds = %"94"
  %"122" = load i64, ptr addrspace(5) %"98", align 8
  %"132" = inttoptr i64 %"122" to ptr
  %"121" = load i64, ptr %"132", align 8
  store i64 %"121", ptr addrspace(5) %"100", align 8
  %"73" = load i64, ptr addrspace(4) @6, align 8
  %"124" = load i64, ptr addrspace(5) %"100", align 8
  %"123" = add i64 %"124", %"73"
  store i64 %"123", ptr addrspace(5) %"101", align 8
  %"125" = load i64, ptr addrspace(5) %"99", align 8
  %"126" = load i64, ptr addrspace(5) %"101", align 8
  %"133" = inttoptr i64 %"125" to ptr
  store i64 %"126", ptr %"133", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }