declare hidden void @__zluda_ptx_impl___assertfail(i64, i64, i32, i64, i64) #0

define amdgpu_kernel void @assertfail(ptr addrspace(4) byref(i64) %"89", ptr addrspace(4) byref(i64) %"90") #1 {
  %"91" = alloca i64, align 8, addrspace(5)
  %"92" = alloca i64, align 8, addrspace(5)
  %"93" = alloca i64, align 8, addrspace(5)
  %"94" = alloca i64, align 8, addrspace(5)
  %"97" = alloca i32, align 4, addrspace(5)
  %"99" = alloca i64, align 8, addrspace(5)
  %"102" = alloca i64, align 8, addrspace(5)
  %"105" = alloca i32, align 4, addrspace(5)
  %"108" = alloca i64, align 8, addrspace(5)
  %"111" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"87"

"87":                                             ; preds = %1
  %"95" = load i64, ptr addrspace(4) %"89", align 8
  store i64 %"95", ptr addrspace(5) %"91", align 8
  %"96" = load i64, ptr addrspace(4) %"90", align 8
  store i64 %"96", ptr addrspace(5) %"92", align 8
  store i32 0, ptr addrspace(5) %"97", align 4
  %"100" = getelementptr inbounds i8, ptr addrspace(5) %"99", i64 0
  %"101" = load i64, ptr addrspace(5) %"91", align 8
  store i64 %"101", ptr addrspace(5) %"100", align 8
  %"103" = getelementptr inbounds i8, ptr addrspace(5) %"102", i64 0
  %"104" = load i64, ptr addrspace(5) %"91", align 8
  store i64 %"104", ptr addrspace(5) %"103", align 8
  %"106" = getelementptr inbounds i8, ptr addrspace(5) %"105", i64 0
  %"107" = load i32, ptr addrspace(5) %"97", align 4
  store i32 %"107", ptr addrspace(5) %"106", align 4
  %"109" = getelementptr inbounds i8, ptr addrspace(5) %"108", i64 0
  %"110" = load i64, ptr addrspace(5) %"91", align 8
  store i64 %"110", ptr addrspace(5) %"109", align 8
  %"112" = getelementptr inbounds i8, ptr addrspace(5) %"111", i64 0
  %"113" = load i64, ptr addrspace(5) %"91", align 8
  store i64 %"113", ptr addrspace(5) %"112", align 8
  %"77" = load i64, ptr addrspace(5) %"99", align 8
  %"78" = load i64, ptr addrspace(5) %"102", align 8
  %"79" = load i32, ptr addrspace(5) %"105", align 4
  %"80" = load i64, ptr addrspace(5) %"108", align 8
  %"81" = load i64, ptr addrspace(5) %"111", align 8
  call void @__zluda_ptx_impl___assertfail(i64 %"77", i64 %"78", i32 %"79", i64 %"80", i64 %"81")
  br label %"88"

"88":                                             ; preds = %"87"
  %"115" = load i64, ptr addrspace(5) %"91", align 8
  %"125" = inttoptr i64 %"115" to ptr
  %"114" = load i64, ptr %"125", align 8
  store i64 %"114", ptr addrspace(5) %"93", align 8
  %"117" = load i64, ptr addrspace(5) %"93", align 8
  %"116" = add i64 %"117", 1
  store i64 %"116", ptr addrspace(5) %"94", align 8
  %"118" = load i64, ptr addrspace(5) %"92", align 8
  %"119" = load i64, ptr addrspace(5) %"94", align 8
  %"126" = inttoptr i64 %"118" to ptr
  store i64 %"119", ptr %"126", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
