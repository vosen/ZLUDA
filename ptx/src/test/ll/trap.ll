define amdgpu_kernel void @trap(ptr addrspace(4) byref(i64) %"27", ptr addrspace(4) byref(i64) %"28") #0 {
  br label %1

1:                                                ; preds = %0
  br label %"26"

"26":                                             ; preds = %1
  call void @llvm.trap()
  ret void
}

; Function Attrs: cold noreturn nounwind
declare void @llvm.trap() #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { cold noreturn nounwind }