define amdgpu_kernel void @fma_bf16x2(ptr addrspace(4) byref(i64) %"39", ptr addrspace(4) byref(i64) %"40") #0 {
  %"41" = alloca i64, align 8, addrspace(5)
  %"42" = alloca i64, align 8, addrspace(5)
  %"43" = alloca i32, align 4, addrspace(5)
  %"44" = alloca i32, align 4, addrspace(5)
  %"45" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"38"

"38":                                             ; preds = %1
  %"46" = load i64, ptr addrspace(4) %"39", align 8
  store i64 %"46", ptr addrspace(5) %"41", align 8
  %"47" = load i64, ptr addrspace(4) %"40", align 8
  store i64 %"47", ptr addrspace(5) %"42", align 8
  %"49" = load i64, ptr addrspace(5) %"41", align 8
  %"60" = inttoptr i64 %"49" to ptr
  %"48" = load i32, ptr %"60", align 4
  store i32 %"48", ptr addrspace(5) %"43", align 4
  %"50" = load i64, ptr addrspace(5) %"41", align 8
  %"61" = inttoptr i64 %"50" to ptr
  %"35" = getelementptr inbounds i8, ptr %"61", i64 4
  %"51" = load i32, ptr %"35", align 4
  store i32 %"51", ptr addrspace(5) %"44", align 4
  %"52" = load i64, ptr addrspace(5) %"41", align 8
  %"62" = inttoptr i64 %"52" to ptr
  %"37" = getelementptr inbounds i8, ptr %"62", i64 8
  %"53" = load i32, ptr %"37", align 4
  store i32 %"53", ptr addrspace(5) %"45", align 4
  %"55" = load i32, ptr addrspace(5) %"43", align 4
  %"56" = load i32, ptr addrspace(5) %"44", align 4
  %"57" = load i32, ptr addrspace(5) %"45", align 4
  %"64" = bitcast i32 %"55" to <2 x bfloat>
  %"65" = bitcast i32 %"56" to <2 x bfloat>
  %"66" = bitcast i32 %"57" to <2 x bfloat>
  %"63" = call <2 x bfloat> @llvm.fma.v2bf16(<2 x bfloat> %"64", <2 x bfloat> %"65", <2 x bfloat> %"66")
  %"54" = bitcast <2 x bfloat> %"63" to i32
  store i32 %"54", ptr addrspace(5) %"43", align 4
  %"58" = load i64, ptr addrspace(5) %"42", align 8
  %"59" = load i32, ptr addrspace(5) %"43", align 4
  %"67" = inttoptr i64 %"58" to ptr
  store i32 %"59", ptr %"67", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare <2 x bfloat> @llvm.fma.v2bf16(<2 x bfloat>, <2 x bfloat>, <2 x bfloat>) #1

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }