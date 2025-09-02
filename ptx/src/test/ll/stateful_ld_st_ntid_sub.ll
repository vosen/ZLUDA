@0 = addrspace(4) global i8 0
@1 = addrspace(4) global i64 0
@2 = addrspace(4) global i64 0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @stateful_ld_st_ntid_sub(ptr addrspace(4) byref(i64) %"49", ptr addrspace(4) byref(i64) %"50") #1 {
  %"51" = alloca i64, align 8, addrspace(5)
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  %"55" = alloca i64, align 8, addrspace(5)
  %"56" = alloca i64, align 8, addrspace(5)
  %"57" = alloca i32, align 4, addrspace(5)
  %"58" = alloca i64, align 8, addrspace(5)
  %"59" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"46"

"46":                                             ; preds = %1
  %"79" = load i64, ptr addrspace(4) %"49", align 8
  store i64 %"79", ptr addrspace(5) %"51", align 8
  %"80" = load i64, ptr addrspace(4) %"50", align 8
  store i64 %"80", ptr addrspace(5) %"54", align 8
  %"63" = load i64, ptr addrspace(5) %"51", align 8
  %2 = inttoptr i64 %"63" to ptr
  %"62" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"62", ptr addrspace(5) %"52", align 8
  %"65" = load i64, ptr addrspace(5) %"54", align 8
  %3 = inttoptr i64 %"65" to ptr
  %"64" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"64", ptr addrspace(5) %"55", align 8
  %"39" = call i32 @__zluda_ptx_impl_sreg_tid(i8 ptrtoint (ptr addrspace(4) @0 to i8))
  br label %"47"

"47":                                             ; preds = %"46"
  store i32 %"39", ptr addrspace(5) %"57", align 4
  %"68" = load i32, ptr addrspace(5) %"57", align 4
  %"67" = zext i32 %"68" to i64
  store i64 %"67", ptr addrspace(5) %"58", align 8
  %"70" = load i64, ptr addrspace(5) %"52", align 8
  %"71" = load i64, ptr addrspace(5) %"58", align 8
  %"82" = sub i64 %"70", %"71"
  store i64 %"82", ptr addrspace(5) %"53", align 8
  %"73" = load i64, ptr addrspace(5) %"55", align 8
  %"74" = load i64, ptr addrspace(5) %"58", align 8
  %"85" = sub i64 %"73", %"74"
  store i64 %"85", ptr addrspace(5) %"56", align 8
  %"41" = load i64, ptr addrspace(4) @1, align 8
  %"75" = load i64, ptr addrspace(5) %"53", align 8
  %"88" = inttoptr i64 %"75" to ptr addrspace(1)
  %"42" = getelementptr inbounds i8, ptr addrspace(1) %"88", i64 %"41"
  %"76" = load i64, ptr addrspace(1) %"42", align 8
  store i64 %"76", ptr addrspace(5) %"59", align 8
  %"44" = load i64, ptr addrspace(4) @2, align 8
  %"77" = load i64, ptr addrspace(5) %"56", align 8
  %"89" = inttoptr i64 %"77" to ptr addrspace(1)
  %"45" = getelementptr inbounds i8, ptr addrspace(1) %"89", i64 %"44"
  %"78" = load i64, ptr addrspace(5) %"59", align 8
  store i64 %"78", ptr addrspace(1) %"45", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }