declare hidden void @__zluda_ptx_impl___assertfail(i64, i64, i32, i64, i64) #0

define amdgpu_kernel void @assertfail(ptr addrspace(4) byref(i64) %"89", ptr addrspace(4) byref(i64) %"90") #1 {
  %"91" = alloca i64, align 8, addrspace(5)
  %"92" = alloca i64, align 8, addrspace(5)
  %"93" = alloca i64, align 8, addrspace(5)
  %"94" = alloca i64, align 8, addrspace(5)
  %"97" = alloca i32, align 4, addrspace(5)
  %"98" = alloca i32, align 4, addrspace(5)
  store i32 0, ptr addrspace(5) %"98", align 4
  %"101" = alloca i64, align 8, addrspace(5)
  %"102" = alloca i64, align 8, addrspace(5)
  store i64 0, ptr addrspace(5) %"102", align 4
  %"106" = alloca i64, align 8, addrspace(5)
  %"107" = alloca i64, align 8, addrspace(5)
  store i64 0, ptr addrspace(5) %"107", align 4
  %"111" = alloca i32, align 4, addrspace(5)
  %"112" = alloca i64, align 8, addrspace(5)
  store i64 0, ptr addrspace(5) %"112", align 4
  %"116" = alloca i64, align 8, addrspace(5)
  %"117" = alloca i64, align 8, addrspace(5)
  store i64 0, ptr addrspace(5) %"117", align 4
  %"121" = alloca i64, align 8, addrspace(5)
  %"122" = alloca i64, align 8, addrspace(5)
  store i64 0, ptr addrspace(5) %"122", align 4
  %"128" = alloca i64, align 8, addrspace(5)
  store i64 1, ptr addrspace(5) %"128", align 4
  br label %1

1:                                                ; preds = %0
  br label %"87"

"87":                                             ; preds = %1
  %"95" = load i64, ptr addrspace(4) %"89", align 8
  store i64 %"95", ptr addrspace(5) %"91", align 8
  %"96" = load i64, ptr addrspace(4) %"90", align 8
  store i64 %"96", ptr addrspace(5) %"92", align 8
  %"100" = load i32, ptr addrspace(5) %"98", align 4
  store i32 %"100", ptr addrspace(5) %"97", align 4
  %"104" = load i64, ptr addrspace(5) %"102", align 8
  %"103" = getelementptr inbounds i8, ptr addrspace(5) %"101", i64 %"104"
  %"105" = load i64, ptr addrspace(5) %"91", align 8
  store i64 %"105", ptr addrspace(5) %"103", align 8
  %"109" = load i64, ptr addrspace(5) %"107", align 8
  %"108" = getelementptr inbounds i8, ptr addrspace(5) %"106", i64 %"109"
  %"110" = load i64, ptr addrspace(5) %"91", align 8
  store i64 %"110", ptr addrspace(5) %"108", align 8
  %"114" = load i64, ptr addrspace(5) %"112", align 8
  %"113" = getelementptr inbounds i8, ptr addrspace(5) %"111", i64 %"114"
  %"115" = load i32, ptr addrspace(5) %"97", align 4
  store i32 %"115", ptr addrspace(5) %"113", align 4
  %"119" = load i64, ptr addrspace(5) %"117", align 8
  %"118" = getelementptr inbounds i8, ptr addrspace(5) %"116", i64 %"119"
  %"120" = load i64, ptr addrspace(5) %"91", align 8
  store i64 %"120", ptr addrspace(5) %"118", align 8
  %"124" = load i64, ptr addrspace(5) %"122", align 8
  %"123" = getelementptr inbounds i8, ptr addrspace(5) %"121", i64 %"124"
  %"125" = load i64, ptr addrspace(5) %"91", align 8
  store i64 %"125", ptr addrspace(5) %"123", align 8
  %"77" = load i64, ptr addrspace(5) %"101", align 8
  %"78" = load i64, ptr addrspace(5) %"106", align 8
  %"79" = load i32, ptr addrspace(5) %"111", align 4
  %"80" = load i64, ptr addrspace(5) %"116", align 8
  %"81" = load i64, ptr addrspace(5) %"121", align 8
  call void @__zluda_ptx_impl___assertfail(i64 %"77", i64 %"78", i32 %"79", i64 %"80", i64 %"81")
  br label %"88"

"88":                                             ; preds = %"87"
  %"127" = load i64, ptr addrspace(5) %"91", align 8
  %"139" = inttoptr i64 %"127" to ptr
  %"126" = load i64, ptr %"139", align 8
  store i64 %"126", ptr addrspace(5) %"93", align 8
  %"130" = load i64, ptr addrspace(5) %"93", align 8
  %"131" = load i64, ptr addrspace(5) %"128", align 8
  %"129" = add i64 %"130", %"131"
  store i64 %"129", ptr addrspace(5) %"94", align 8
  %"132" = load i64, ptr addrspace(5) %"92", align 8
  %"133" = load i64, ptr addrspace(5) %"94", align 8
  %"140" = inttoptr i64 %"132" to ptr
  store i64 %"133", ptr %"140", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }