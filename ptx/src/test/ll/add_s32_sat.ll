define amdgpu_kernel void @add_s32_sat(ptr addrspace(4) byref(i64) %"43", ptr addrspace(4) byref(i64) %"44") #0 {
  %"45" = alloca i64, align 8, addrspace(5)
  %"46" = alloca i64, align 8, addrspace(5)
  %"47" = alloca i32, align 4, addrspace(5)
  %"48" = alloca i32, align 4, addrspace(5)
  %"49" = alloca i32, align 4, addrspace(5)
  %"50" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"42"

"42":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"43", align 8
  store i64 %2, ptr addrspace(5) %"45", align 8
  %3 = load i64, ptr addrspace(4) %"44", align 8
  store i64 %3, ptr addrspace(5) %"46", align 8
  %4 = load i64, ptr addrspace(5) %"45", align 8
  %"67" = inttoptr i64 %4 to ptr
  %5 = load i32, ptr %"67", align 4
  store i32 %5, ptr addrspace(5) %"47", align 4
  %6 = load i64, ptr addrspace(5) %"45", align 8
  %"68" = inttoptr i64 %6 to ptr
  %"39" = getelementptr inbounds i8, ptr %"68", i64 4
  %7 = load i32, ptr %"39", align 4
  store i32 %7, ptr addrspace(5) %"48", align 4
  %8 = load i32, ptr addrspace(5) %"47", align 4
  %9 = load i32, ptr addrspace(5) %"48", align 4
  %"57" = call i32 @llvm.sadd.sat.i32(i32 %8, i32 %9)
  store i32 %"57", ptr addrspace(5) %"49", align 4
  %10 = load i32, ptr addrspace(5) %"47", align 4
  %11 = load i32, ptr addrspace(5) %"48", align 4
  %"60" = add i32 %10, %11
  store i32 %"60", ptr addrspace(5) %"50", align 4
  %12 = load i64, ptr addrspace(5) %"46", align 8
  %13 = load i32, ptr addrspace(5) %"49", align 4
  %"69" = inttoptr i64 %12 to ptr
  store i32 %13, ptr %"69", align 4
  %14 = load i64, ptr addrspace(5) %"46", align 8
  %"70" = inttoptr i64 %14 to ptr
  %"41" = getelementptr inbounds i8, ptr %"70", i64 4
  %15 = load i32, ptr addrspace(5) %"50", align 4
  store i32 %15, ptr %"41", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.sadd.sat.i32(i32, i32) #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }