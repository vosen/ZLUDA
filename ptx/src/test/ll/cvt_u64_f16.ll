; Function Attrs: denormal_fpenv(float: preservesign)
define amdgpu_kernel void @cvt_u64_f16(ptr addrspace(4) byref(i64) %"52", ptr addrspace(4) byref(i64) %"53") #0 {
  %"54" = alloca i64, align 8, addrspace(5)
  %"55" = alloca i64, align 8, addrspace(5)
  %"56" = alloca half, align 2, addrspace(5)
  %"57" = alloca half, align 2, addrspace(5)
  %"58" = alloca half, align 2, addrspace(5)
  %"59" = alloca i64, align 8, addrspace(5)
  %"60" = alloca i64, align 8, addrspace(5)
  %"61" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"51"

"51":                                             ; preds = %1
  call void @llvm.amdgcn.s.dcache.inv()
  %2 = load i64, ptr addrspace(4) %"52", align 8
  store i64 %2, ptr addrspace(5) %"54", align 8
  %3 = load i64, ptr addrspace(4) %"53", align 8
  store i64 %3, ptr addrspace(5) %"55", align 8
  %4 = load i64, ptr addrspace(5) %"54", align 8
  %"83" = inttoptr i64 %4 to ptr
  %5 = load i16, ptr %"83", align 2
  %"64" = bitcast i16 %5 to half
  store half %"64", ptr addrspace(5) %"56", align 2
  %6 = load i64, ptr addrspace(5) %"54", align 8
  %"84" = inttoptr i64 %6 to ptr
  %"44" = getelementptr inbounds i8, ptr %"84", i64 2
  %7 = load i16, ptr %"44", align 2
  %"67" = bitcast i16 %7 to half
  store half %"67", ptr addrspace(5) %"57", align 2
  %8 = load i64, ptr addrspace(5) %"54", align 8
  %"86" = inttoptr i64 %8 to ptr
  %"46" = getelementptr inbounds i8, ptr %"86", i64 4
  %9 = load i16, ptr %"46", align 2
  %"69" = bitcast i16 %9 to half
  store half %"69", ptr addrspace(5) %"58", align 2
  %10 = load half, ptr addrspace(5) %"56", align 2
  %"70" = call i64 @llvm.fptoui.sat.i64.f16(half %10)
  store i64 %"70", ptr addrspace(5) %"59", align 8
  %11 = load half, ptr addrspace(5) %"57", align 2
  %"72" = call i64 @llvm.fptoui.sat.i64.f16(half %11)
  store i64 %"72", ptr addrspace(5) %"60", align 8
  %12 = load half, ptr addrspace(5) %"58", align 2
  %"74" = call i64 @llvm.fptoui.sat.i64.f16(half %12)
  store i64 %"74", ptr addrspace(5) %"61", align 8
  %13 = load i64, ptr addrspace(5) %"55", align 8
  %14 = load i64, ptr addrspace(5) %"59", align 8
  %"88" = inttoptr i64 %13 to ptr
  store i64 %14, ptr %"88", align 8
  %15 = load i64, ptr addrspace(5) %"55", align 8
  %"89" = inttoptr i64 %15 to ptr
  %"48" = getelementptr inbounds i8, ptr %"89", i64 8
  %16 = load i64, ptr addrspace(5) %"60", align 8
  store i64 %16, ptr %"48", align 8
  %17 = load i64, ptr addrspace(5) %"55", align 8
  %"90" = inttoptr i64 %17 to ptr
  %"50" = getelementptr inbounds i8, ptr %"90", i64 16
  %18 = load i64, ptr addrspace(5) %"61", align 8
  store i64 %18, ptr %"50", align 8
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn
declare void @llvm.amdgcn.s.dcache.inv() #1

; Function Attrs: nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.fptoui.sat.i64.f16(half) #2

attributes #0 = { denormal_fpenv(float: preservesign) "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind willreturn }
attributes #2 = { nocallback nocreateundeforpoison nofree nosync nounwind speculatable willreturn memory(none) }
