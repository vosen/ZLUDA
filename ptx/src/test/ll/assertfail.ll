@__ZLUDA_PTX_IMPL_ATTRIBUTE_CLOCK_RATE = addrspace(1) global i32 2124000

declare void @__zluda_ptx_impl___assertfail(i64, i64, i32, i64, i64) #0

define amdgpu_kernel void @assertfail(ptr addrspace(4) byref(i64) %"86", ptr addrspace(4) byref(i64) %"87") #1 {
  %"88" = alloca i64, align 8, addrspace(5)
  %"89" = alloca i64, align 8, addrspace(5)
  %"90" = alloca i64, align 8, addrspace(5)
  %"91" = alloca i64, align 8, addrspace(5)
  %"94" = alloca i32, align 4, addrspace(5)
  %"96" = alloca i64, align 8, addrspace(5)
  %"99" = alloca i64, align 8, addrspace(5)
  %"102" = alloca i32, align 4, addrspace(5)
  %"105" = alloca i64, align 8, addrspace(5)
  %"108" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"84"

"84":                                             ; preds = %1
  %"92" = load i64, ptr addrspace(4) %"86", align 4
  store i64 %"92", ptr addrspace(5) %"88", align 4
  %"93" = load i64, ptr addrspace(4) %"87", align 4
  store i64 %"93", ptr addrspace(5) %"89", align 4
  store i32 0, ptr addrspace(5) %"94", align 4
  %"97" = getelementptr inbounds i8, ptr addrspace(5) %"96", i64 0
  %"98" = load i64, ptr addrspace(5) %"88", align 4
  store i64 %"98", ptr addrspace(5) %"97", align 4
  %"100" = getelementptr inbounds i8, ptr addrspace(5) %"99", i64 0
  %"101" = load i64, ptr addrspace(5) %"88", align 4
  store i64 %"101", ptr addrspace(5) %"100", align 4
  %"103" = getelementptr inbounds i8, ptr addrspace(5) %"102", i64 0
  %"104" = load i32, ptr addrspace(5) %"94", align 4
  store i32 %"104", ptr addrspace(5) %"103", align 4
  %"106" = getelementptr inbounds i8, ptr addrspace(5) %"105", i64 0
  %"107" = load i64, ptr addrspace(5) %"88", align 4
  store i64 %"107", ptr addrspace(5) %"106", align 4
  %"109" = getelementptr inbounds i8, ptr addrspace(5) %"108", i64 0
  %"110" = load i64, ptr addrspace(5) %"88", align 4
  store i64 %"110", ptr addrspace(5) %"109", align 4
  %"74" = load i64, ptr addrspace(5) %"96", align 4
  %"75" = load i64, ptr addrspace(5) %"99", align 4
  %"76" = load i32, ptr addrspace(5) %"102", align 4
  %"77" = load i64, ptr addrspace(5) %"105", align 4
  %"78" = load i64, ptr addrspace(5) %"108", align 4
  call void @__zluda_ptx_impl___assertfail(i64 %"74", i64 %"75", i32 %"76", i64 %"77", i64 %"78")
  br label %"85"

"85":                                             ; preds = %"84"
  %"112" = load i64, ptr addrspace(5) %"88", align 4
  %"122" = inttoptr i64 %"112" to ptr
  %"111" = load i64, ptr %"122", align 4
  store i64 %"111", ptr addrspace(5) %"90", align 4
  %"114" = load i64, ptr addrspace(5) %"90", align 4
  %"113" = add i64 %"114", 1
  store i64 %"113", ptr addrspace(5) %"91", align 4
  %"115" = load i64, ptr addrspace(5) %"89", align 4
  %"116" = load i64, ptr addrspace(5) %"91", align 4
  %"123" = inttoptr i64 %"115" to ptr
  store i64 %"116", ptr %"123", align 4
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }