@x = addrspace(4) externally_initialized global i64 1
@y = addrspace(4) externally_initialized global [4 x i64] [i64 4, i64 5, i64 6, i64 0]
@constparams = addrspace(4) externally_initialized global [2 x i64] [i64 ptrtoint (ptr addrspace(4) @x to i64), i64 ptrtoint (ptr addrspace(4) @y to i64)]

define amdgpu_kernel void @const_ident(ptr addrspace(4) byref(i64) %"55", ptr addrspace(4) byref(i64) %"56") #0 {
  %"57" = alloca i64, align 8, addrspace(5)
  %"58" = alloca i64, align 8, addrspace(5)
  %"59" = alloca i64, align 8, addrspace(5)
  %"60" = alloca i64, align 8, addrspace(5)
  %"61" = alloca i64, align 8, addrspace(5)
  %"62" = alloca i64, align 8, addrspace(5)
  %"63" = alloca i64, align 8, addrspace(5)
  %"64" = alloca i64, align 8, addrspace(5)
  br label %1

1:                                                ; preds = %0
  br label %"54"

"54":                                             ; preds = %1
  %2 = load i64, ptr addrspace(4) %"55", align 8
  store i64 %2, ptr addrspace(5) %"57", align 8
  %3 = load i64, ptr addrspace(4) %"56", align 8
  store i64 %3, ptr addrspace(5) %"58", align 8
  store i64 ptrtoint (ptr addrspace(4) @x to i64), ptr addrspace(5) %"59", align 8
  store i64 ptrtoint (ptr addrspace(4) @y to i64), ptr addrspace(5) %"60", align 8
  %4 = load i64, ptr addrspace(4) @constparams, align 8
  store i64 %4, ptr addrspace(5) %"61", align 8
  %5 = load i64, ptr addrspace(4) getelementptr inbounds (i8, ptr addrspace(4) @constparams, i64 8), align 8
  store i64 %5, ptr addrspace(5) %"62", align 8
  %6 = load i64, ptr addrspace(5) %"59", align 8
  %7 = load i64, ptr addrspace(5) %"61", align 8
  %"71" = xor i64 %6, %7
  store i64 %"71", ptr addrspace(5) %"63", align 8
  %8 = load i64, ptr addrspace(5) %"60", align 8
  %9 = load i64, ptr addrspace(5) %"62", align 8
  %"74" = xor i64 %8, %9
  store i64 %"74", ptr addrspace(5) %"64", align 8
  %10 = load i64, ptr addrspace(5) %"58", align 8
  %11 = load i64, ptr addrspace(5) %"63", align 8
  %"91" = inttoptr i64 %10 to ptr
  store i64 %11, ptr %"91", align 8
  %12 = load i64, ptr addrspace(5) %"58", align 8
  %"93" = inttoptr i64 %12 to ptr
  %"51" = getelementptr inbounds i8, ptr %"93", i64 8
  %13 = load i64, ptr addrspace(5) %"64", align 8
  store i64 %13, ptr %"51", align 8
  %14 = load i64, ptr addrspace(5) %"58", align 8
  %"95" = inttoptr i64 %14 to ptr
  %"53" = getelementptr inbounds i8, ptr %"95", i64 8
  %15 = load i64, ptr addrspace(5) %"64", align 8
  store i64 %15, ptr %"53", align 8
  ret void
}

attributes #0 = { "amdgpu-ieee"="false" "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="preserve-sign" "denormal-fp-math-f32"="preserve-sign" "no-trapping-math"="true" "target-features"="+wavefrontsize32,-wavefrontsize64,+cumode,+precise-memory" "uniform-work-group-size"="true" }
