declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @stateful_ld_st_ntid_sub(ptr addrspace(4) byref(i64) %"47", ptr addrspace(4) byref(i64) %"48") #1 {
  %"49" = alloca i64, align 8, addrspace(5)
  %"50" = alloca i64, align 8, addrspace(5)
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  %"55" = alloca i32, align 4, addrspace(5)
  %"56" = alloca i64, align 8, addrspace(5)
  %"57" = alloca i64, align 8, addrspace(5)
  %"64" = alloca i8, align 1, addrspace(5)
  store i8 0, ptr addrspace(5) %"64", align 1
  %"75" = alloca i64, align 8, addrspace(5)
  store i64 0, ptr addrspace(5) %"75", align 4
  %"79" = alloca i64, align 8, addrspace(5)
  store i64 0, ptr addrspace(5) %"79", align 4
  br label %1

1:                                                ; preds = %0
  br label %"44"

"44":                                             ; preds = %1
  %"83" = load i64, ptr addrspace(4) %"47", align 8
  store i64 %"83", ptr addrspace(5) %"49", align 8
  %"84" = load i64, ptr addrspace(4) %"48", align 8
  store i64 %"84", ptr addrspace(5) %"52", align 8
  %"61" = load i64, ptr addrspace(5) %"49", align 8
  %2 = inttoptr i64 %"61" to ptr
  %"60" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"60", ptr addrspace(5) %"50", align 8
  %"63" = load i64, ptr addrspace(5) %"52", align 8
  %3 = inttoptr i64 %"63" to ptr
  %"62" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"62", ptr addrspace(5) %"53", align 8
  %"65" = load i8, ptr addrspace(5) %"64", align 1
  %"39" = call i32 @__zluda_ptx_impl_sreg_tid(i8 %"65")
  br label %"45"

"45":                                             ; preds = %"44"
  store i32 %"39", ptr addrspace(5) %"55", align 4
  %"68" = load i32, ptr addrspace(5) %"55", align 4
  %"67" = zext i32 %"68" to i64
  store i64 %"67", ptr addrspace(5) %"56", align 8
  %"70" = load i64, ptr addrspace(5) %"50", align 8
  %"71" = load i64, ptr addrspace(5) %"56", align 8
  %"85" = sub i64 %"70", %"71"
  store i64 %"85", ptr addrspace(5) %"51", align 8
  %"73" = load i64, ptr addrspace(5) %"53", align 8
  %"74" = load i64, ptr addrspace(5) %"56", align 8
  %"88" = sub i64 %"73", %"74"
  store i64 %"88", ptr addrspace(5) %"54", align 8
  %"76" = load i64, ptr addrspace(5) %"51", align 8
  %"77" = load i64, ptr addrspace(5) %"75", align 8
  %"91" = inttoptr i64 %"76" to ptr addrspace(1)
  %"41" = getelementptr inbounds i8, ptr addrspace(1) %"91", i64 %"77"
  %"78" = load i64, ptr addrspace(1) %"41", align 8
  store i64 %"78", ptr addrspace(5) %"57", align 8
  %"80" = load i64, ptr addrspace(5) %"54", align 8
  %"81" = load i64, ptr addrspace(5) %"79", align 8
  %"92" = inttoptr i64 %"80" to ptr addrspace(1)
  %"43" = getelementptr inbounds i8, ptr addrspace(1) %"92", i64 %"81"
  %"82" = load i64, ptr addrspace(5) %"57", align 8
  store i64 %"82", ptr addrspace(1) %"43", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }