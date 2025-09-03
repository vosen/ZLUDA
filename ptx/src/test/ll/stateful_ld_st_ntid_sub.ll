@0 = addrspace(4) global i8 0
@1 = addrspace(4) global i64 0
@2 = addrspace(4) global i64 0

declare hidden i32 @__zluda_ptx_impl_sreg_tid(i8) #0

define amdgpu_kernel void @stateful_ld_st_ntid_sub(ptr addrspace(4) byref(i64) %"50", ptr addrspace(4) byref(i64) %"51") #1 {
  %"52" = alloca i64, align 8, addrspace(5)
  %"53" = alloca i64, align 8, addrspace(5)
  %"54" = alloca i64, align 8, addrspace(5)
  %"55" = alloca i64, align 8, addrspace(5)
  %"56" = alloca i64, align 8, addrspace(5)
  %"57" = alloca i64, align 8, addrspace(5)
  %"58" = alloca i32, align 4, addrspace(5)
  %"59" = alloca i64, align 8, addrspace(5)
  %"60" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"47"

"47":                                             ; preds = %1
  %"80" = load i64, ptr addrspace(4) %"50", align 8
  store i64 %"80", ptr addrspace(5) %"52", align 8
  %"81" = load i64, ptr addrspace(4) %"51", align 8
  store i64 %"81", ptr addrspace(5) %"55", align 8
  %"64" = load i64, ptr addrspace(5) %"52", align 8
  %2 = inttoptr i64 %"64" to ptr
  %"63" = addrspacecast ptr %2 to ptr addrspace(1)
  store ptr addrspace(1) %"63", ptr addrspace(5) %"53", align 8
  %"66" = load i64, ptr addrspace(5) %"55", align 8
  %3 = inttoptr i64 %"66" to ptr
  %"65" = addrspacecast ptr %3 to ptr addrspace(1)
  store ptr addrspace(1) %"65", ptr addrspace(5) %"56", align 8
  %"39" = load i8, ptr addrspace(4) @0, align 1
  %"40" = call i32 @__zluda_ptx_impl_sreg_tid(i8 %"39")
  br label %"48"

"48":                                             ; preds = %"47"
  store i32 %"40", ptr addrspace(5) %"58", align 4
  %"69" = load i32, ptr addrspace(5) %"58", align 4
  %"68" = zext i32 %"69" to i64
  store i64 %"68", ptr addrspace(5) %"59", align 8
  %"71" = load i64, ptr addrspace(5) %"53", align 8
  %"72" = load i64, ptr addrspace(5) %"59", align 8
  %"82" = sub i64 %"71", %"72"
  store i64 %"82", ptr addrspace(5) %"54", align 8
  %"74" = load i64, ptr addrspace(5) %"56", align 8
  %"75" = load i64, ptr addrspace(5) %"59", align 8
  %"85" = sub i64 %"74", %"75"
  store i64 %"85", ptr addrspace(5) %"57", align 8
  %"42" = load i64, ptr addrspace(4) @1, align 8
  %"76" = load i64, ptr addrspace(5) %"54", align 8
  %"88" = inttoptr i64 %"76" to ptr addrspace(1)
  %"43" = getelementptr inbounds i8, ptr addrspace(1) %"88", i64 %"42"
  %"77" = load i64, ptr addrspace(1) %"43", align 8
  store i64 %"77", ptr addrspace(5) %"60", align 8
  %"45" = load i64, ptr addrspace(4) @2, align 8
  %"78" = load i64, ptr addrspace(5) %"57", align 8
  %"89" = inttoptr i64 %"78" to ptr addrspace(1)
  %"46" = getelementptr inbounds i8, ptr addrspace(1) %"89", i64 %"45"
  %"79" = load i64, ptr addrspace(5) %"60", align 8
  store i64 %"79", ptr addrspace(1) %"46", align 8
  ret void
}

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="dynamic" "denormal-fp-math-f32"="dynamic" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "uniform-work-group-size"="true" }