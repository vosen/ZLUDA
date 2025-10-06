define amdgpu_kernel void @cvt_s16_s8(ptr addrspace(4) byref(i64) %"37", ptr addrspace(4) byref(i64) %"38") #0 {
  %"39" = alloca i64, align 8, addrspace(5)
  %"40" = alloca i64, align 8, addrspace(5)
  %"41" = alloca i32, align 4, addrspace(5)
  %"42" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"36"

"36":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"37", align 8
  store i64 %2, ptr addrspace(5) %"39", align 8
  %3 = load i64, ptr addrspace(4) %"38", align 8
  store i64 %3, ptr addrspace(5) %"40", align 8
  %4 = load i64, ptr addrspace(5) %"39", align 8
  %"51" = inttoptr i64 %4 to ptr addrspace(1)
  %5 = load i32, ptr addrspace(1) %"51", align 4
  store i32 %5, ptr addrspace(5) %"42", align 4
  %6 = load i32, ptr addrspace(5) %"42", align 4
  %7 = trunc i32 %6 to i8
  %"52" = sext i8 %7 to i16
  %"47" = sext i16 %"52" to i32
  store i32 %"47", ptr addrspace(5) %"41", align 4
  %8 = load i64, ptr addrspace(5) %"40", align 8
  %9 = load i32, ptr addrspace(5) %"41", align 4
  %"54" = inttoptr i64 %8 to ptr
  store i32 %9, ptr %"54", align 4
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }