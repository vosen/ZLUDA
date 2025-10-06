@shared_mem = external addrspace(3) global [1024 x i8], align 4

define amdgpu_kernel void @atom_add(ptr addrspace(4) byref(i64) %"42", ptr addrspace(4) byref(i64) %"43") #0 {
  %"44" = alloca i64, align 8, addrspace(5)
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i32, align 4, addrspace(5)
  %"47" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"41"

"41":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"42", align 8
  store i64 %2, ptr addrspace(5) %"44", align 8
  %3 = load i64, ptr addrspace(4) %"43", align 8
  store i64 %3, ptr addrspace(5) %"45", align 8
  %4 = load i64, ptr addrspace(5) %"44", align 8
  %"62" = inttoptr i64 %4 to ptr
  %5 = load i32, ptr %"62", align 4
  store i32 %5, ptr addrspace(5) %"46", align 4
  %6 = load i64, ptr addrspace(5) %"44", align 8
  %"63" = inttoptr i64 %6 to ptr
  %"38" = getelementptr inbounds i8, ptr %"63", i64 4
  %7 = load i32, ptr %"38", align 4
  store i32 %7, ptr addrspace(5) %"47", align 4
  %8 = load i32, ptr addrspace(5) %"46", align 4
  store i32 %8, ptr addrspace(3) @shared_mem, align 4
  %9 = load i32, ptr addrspace(5) %"47", align 4
  %10 = atomicrmw add ptr addrspace(3) @shared_mem, i32 %9 syncscope("agent-one-as") monotonic, align 4
  store i32 %10, ptr addrspace(5) %"46", align 4
  %11 = load i32, ptr addrspace(3) @shared_mem, align 4
  store i32 %11, ptr addrspace(5) %"47", align 4
  %12 = load i64, ptr addrspace(5) %"45", align 8
  %13 = load i32, ptr addrspace(5) %"46", align 4
  %"67" = inttoptr i64 %12 to ptr
  store i32 %13, ptr %"67", align 4
  %14 = load i64, ptr addrspace(5) %"45", align 8
  %"68" = inttoptr i64 %14 to ptr
  %"40" = getelementptr inbounds i8, ptr %"68", i64 4
  %15 = load i32, ptr addrspace(5) %"47", align 4
  store i32 %15, ptr %"40", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }