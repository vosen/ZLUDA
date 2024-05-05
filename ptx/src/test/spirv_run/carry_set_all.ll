target datalayout = "e-p:64:64-p1:64:64-p2:32:32-p3:32:32-p4:64:64-p5:32:32-p6:32:32-i64:64-v16:16-v24:32-v32:32-v48:64-v96:128-v192:256-v256:256-v512:512-v1024:1024-v2048:2048-n32:64-S32-A5-G1-ni:7"
target triple = "amdgcn-amd-amdhsa"

define protected amdgpu_kernel void @carry_set_all(ptr addrspace(4) byref(i64) %"208", ptr addrspace(4) byref(i64) %"209") #0 {
  %"22" = alloca i1, align 1, addrspace(5)
  %"4" = alloca i64, align 8, addrspace(5)
  %"5" = alloca i64, align 8, addrspace(5)
  %"6" = alloca i32, align 4, addrspace(5)
  %"7" = alloca i32, align 4, addrspace(5)
  %"8" = alloca i32, align 4, addrspace(5)
  %"9" = alloca i32, align 4, addrspace(5)
  %"10" = alloca i32, align 4, addrspace(5)
  %"11" = alloca i32, align 4, addrspace(5)
  %"12" = alloca i32, align 4, addrspace(5)
  %"13" = alloca i32, align 4, addrspace(5)
  %"14" = alloca i32, align 4, addrspace(5)
  %"15" = alloca i32, align 4, addrspace(5)
  %"16" = alloca i32, align 4, addrspace(5)
  %"17" = alloca i32, align 4, addrspace(5)
  %"18" = alloca i32, align 4, addrspace(5)
  %"19" = alloca i32, align 4, addrspace(5)
  %"20" = alloca i32, align 4, addrspace(5)
  %"21" = alloca i32, align 4, addrspace(5)
  br label %1

1:                                                ; preds = %0
  store i1 false, ptr addrspace(5) %"22", align 1
  %"37" = load i64, ptr addrspace(4) %"209", align 8
  store i64 %"37", ptr addrspace(5) %"5", align 8
  %2 = call { i32, i1 } @llvm.usub.with.overflow.i32(i32 0, i32 0)
  %"210" = extractvalue { i32, i1 } %2, 0
  %"23" = extractvalue { i32, i1 } %2, 1
  store i32 %"210", ptr addrspace(5) %"6", align 4
  %"39" = xor i1 %"23", true
  store i1 %"39", ptr addrspace(5) %"22", align 1
  %"41" = load i1, ptr addrspace(5) %"22", align 1
  %3 = zext i1 %"41" to i32
  %"211" = add i32 0, %3
  store i32 %"211", ptr addrspace(5) %"6", align 4
  %"42" = load i1, ptr addrspace(5) %"22", align 1
  %"24" = xor i1 %"42", true
  %4 = zext i1 %"24" to i32
  %"212" = sub i32 0, %4
  store i32 %"212", ptr addrspace(5) %"7", align 4
  %5 = call { i32, i1 } @llvm.usub.with.overflow.i32(i32 0, i32 1)
  %"213" = extractvalue { i32, i1 } %5, 0
  %"25" = extractvalue { i32, i1 } %5, 1
  store i32 %"213", ptr addrspace(5) %"8", align 4
  %"45" = xor i1 %"25", true
  store i1 %"45", ptr addrspace(5) %"22", align 1
  %"47" = load i1, ptr addrspace(5) %"22", align 1
  %6 = zext i1 %"47" to i32
  %"214" = add i32 0, %6
  store i32 %"214", ptr addrspace(5) %"8", align 4
  %"48" = load i1, ptr addrspace(5) %"22", align 1
  %"26" = xor i1 %"48", true
  %7 = zext i1 %"26" to i32
  %"215" = sub i32 0, %7
  store i32 %"215", ptr addrspace(5) %"9", align 4
  %8 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 0, i32 0)
  %"216" = extractvalue { i32, i1 } %8, 0
  %"51" = extractvalue { i32, i1 } %8, 1
  store i32 %"216", ptr addrspace(5) %"10", align 4
  store i1 %"51", ptr addrspace(5) %"22", align 1
  %"53" = load i1, ptr addrspace(5) %"22", align 1
  %9 = zext i1 %"53" to i32
  %"217" = add i32 0, %9
  store i32 %"217", ptr addrspace(5) %"10", align 4
  %"54" = load i1, ptr addrspace(5) %"22", align 1
  %"27" = xor i1 %"54", true
  %10 = zext i1 %"27" to i32
  %"218" = sub i32 0, %10
  store i32 %"218", ptr addrspace(5) %"11", align 4
  %11 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 -1, i32 -1)
  %"219" = extractvalue { i32, i1 } %11, 0
  %"57" = extractvalue { i32, i1 } %11, 1
  store i32 %"219", ptr addrspace(5) %"12", align 4
  store i1 %"57", ptr addrspace(5) %"22", align 1
  %"59" = load i1, ptr addrspace(5) %"22", align 1
  %12 = zext i1 %"59" to i32
  %"220" = add i32 0, %12
  store i32 %"220", ptr addrspace(5) %"12", align 4
  %"60" = load i1, ptr addrspace(5) %"22", align 1
  %"28" = xor i1 %"60", true
  %13 = zext i1 %"28" to i32
  %"221" = sub i32 0, %13
  store i32 %"221", ptr addrspace(5) %"13", align 4
  %14 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 0, i32 0)
  %"222" = extractvalue { i32, i1 } %14, 0
  %"63" = extractvalue { i32, i1 } %14, 1
  store i32 %"222", ptr addrspace(5) %"14", align 4
  store i1 %"63", ptr addrspace(5) %"22", align 1
  %"65" = load i1, ptr addrspace(5) %"22", align 1
  %15 = zext i1 %"65" to i32
  %"223" = add i32 0, %15
  store i32 %"223", ptr addrspace(5) %"14", align 4
  %"66" = load i1, ptr addrspace(5) %"22", align 1
  %"29" = xor i1 %"66", true
  %16 = zext i1 %"29" to i32
  %"224" = sub i32 0, %16
  store i32 %"224", ptr addrspace(5) %"15", align 4
  %17 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 -1, i32 -1)
  %"225" = extractvalue { i32, i1 } %17, 0
  %"69" = extractvalue { i32, i1 } %17, 1
  store i32 %"225", ptr addrspace(5) %"16", align 4
  store i1 %"69", ptr addrspace(5) %"22", align 1
  %"71" = load i1, ptr addrspace(5) %"22", align 1
  %18 = zext i1 %"71" to i32
  %"226" = add i32 0, %18
  store i32 %"226", ptr addrspace(5) %"16", align 4
  %"72" = load i1, ptr addrspace(5) %"22", align 1
  %"30" = xor i1 %"72", true
  %19 = zext i1 %"30" to i32
  %"227" = sub i32 0, %19
  store i32 %"227", ptr addrspace(5) %"17", align 4
  %20 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 0, i32 0)
  %"228" = extractvalue { i32, i1 } %20, 0
  %"75" = extractvalue { i32, i1 } %20, 1
  store i32 %"228", ptr addrspace(5) %"18", align 4
  store i1 %"75", ptr addrspace(5) %"22", align 1
  %"76" = load i1, ptr addrspace(5) %"22", align 1
  %"31" = xor i1 %"76", true
  %21 = zext i1 %"31" to i32
  %22 = call { i32, i1 } @llvm.usub.with.overflow.i32(i32 0, i32 0)
  %23 = extractvalue { i32, i1 } %22, 0
  %24 = extractvalue { i32, i1 } %22, 1
  %25 = call { i32, i1 } @llvm.usub.with.overflow.i32(i32 %23, i32 %21)
  %"229" = extractvalue { i32, i1 } %25, 0
  %26 = extractvalue { i32, i1 } %25, 1
  %"32" = xor i1 %24, %26
  store i32 %"229", ptr addrspace(5) %"18", align 4
  %"78" = xor i1 %"32", true
  store i1 %"78", ptr addrspace(5) %"22", align 1
  %"80" = load i1, ptr addrspace(5) %"22", align 1
  %27 = zext i1 %"80" to i32
  %"230" = add i32 0, %27
  store i32 %"230", ptr addrspace(5) %"18", align 4
  %"81" = load i1, ptr addrspace(5) %"22", align 1
  %"33" = xor i1 %"81", true
  %28 = zext i1 %"33" to i32
  %"231" = sub i32 0, %28
  store i32 %"231", ptr addrspace(5) %"19", align 4
  %29 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 0, i32 0)
  %"232" = extractvalue { i32, i1 } %29, 0
  %"84" = extractvalue { i32, i1 } %29, 1
  store i32 %"232", ptr addrspace(5) %"20", align 4
  store i1 %"84", ptr addrspace(5) %"22", align 1
  %"85" = load i1, ptr addrspace(5) %"22", align 1
  %"34" = xor i1 %"85", true
  %30 = zext i1 %"34" to i32
  %31 = call { i32, i1 } @llvm.usub.with.overflow.i32(i32 0, i32 1)
  %32 = extractvalue { i32, i1 } %31, 0
  %33 = extractvalue { i32, i1 } %31, 1
  %34 = call { i32, i1 } @llvm.usub.with.overflow.i32(i32 %32, i32 %30)
  %"233" = extractvalue { i32, i1 } %34, 0
  %35 = extractvalue { i32, i1 } %34, 1
  %"35" = xor i1 %33, %35
  store i32 %"233", ptr addrspace(5) %"20", align 4
  %"87" = xor i1 %"35", true
  store i1 %"87", ptr addrspace(5) %"22", align 1
  %"89" = load i1, ptr addrspace(5) %"22", align 1
  %36 = zext i1 %"89" to i32
  %"234" = add i32 0, %36
  store i32 %"234", ptr addrspace(5) %"20", align 4
  %"90" = load i1, ptr addrspace(5) %"22", align 1
  %"36" = xor i1 %"90", true
  %37 = zext i1 %"36" to i32
  %"235" = sub i32 0, %37
  store i32 %"235", ptr addrspace(5) %"21", align 4
  %"92" = load i64, ptr addrspace(5) %"5", align 8
  %"93" = load i32, ptr addrspace(5) %"6", align 4
  %"236" = inttoptr i64 %"92" to ptr
  store i32 %"93", ptr %"236", align 4
  %"94" = load i64, ptr addrspace(5) %"5", align 8
  %"95" = load i32, ptr addrspace(5) %"8", align 4
  %"238" = inttoptr i64 %"94" to ptr
  %"269" = getelementptr inbounds i8, ptr %"238", i64 4
  store i32 %"95", ptr %"269", align 4
  %"96" = load i64, ptr addrspace(5) %"5", align 8
  %"97" = load i32, ptr addrspace(5) %"10", align 4
  %"240" = inttoptr i64 %"96" to ptr
  %"271" = getelementptr inbounds i8, ptr %"240", i64 8
  store i32 %"97", ptr %"271", align 4
  %"98" = load i64, ptr addrspace(5) %"5", align 8
  %"99" = load i32, ptr addrspace(5) %"12", align 4
  %"242" = inttoptr i64 %"98" to ptr
  %"273" = getelementptr inbounds i8, ptr %"242", i64 12
  store i32 %"99", ptr %"273", align 4
  %"100" = load i64, ptr addrspace(5) %"5", align 8
  %"101" = load i32, ptr addrspace(5) %"14", align 4
  %"244" = inttoptr i64 %"100" to ptr
  %"275" = getelementptr inbounds i8, ptr %"244", i64 16
  store i32 %"101", ptr %"275", align 4
  %"102" = load i64, ptr addrspace(5) %"5", align 8
  %"103" = load i32, ptr addrspace(5) %"16", align 4
  %"246" = inttoptr i64 %"102" to ptr
  %"277" = getelementptr inbounds i8, ptr %"246", i64 20
  store i32 %"103", ptr %"277", align 4
  %"104" = load i64, ptr addrspace(5) %"5", align 8
  %"105" = load i32, ptr addrspace(5) %"18", align 4
  %"248" = inttoptr i64 %"104" to ptr
  %"279" = getelementptr inbounds i8, ptr %"248", i64 24
  store i32 %"105", ptr %"279", align 4
  %"106" = load i64, ptr addrspace(5) %"5", align 8
  %"107" = load i32, ptr addrspace(5) %"20", align 4
  %"250" = inttoptr i64 %"106" to ptr
  %"281" = getelementptr inbounds i8, ptr %"250", i64 28
  store i32 %"107", ptr %"281", align 4
  %"108" = load i64, ptr addrspace(5) %"5", align 8
  %"109" = load i32, ptr addrspace(5) %"7", align 4
  %"252" = inttoptr i64 %"108" to ptr
  %"283" = getelementptr inbounds i8, ptr %"252", i64 32
  store i32 %"109", ptr %"283", align 4
  %"110" = load i64, ptr addrspace(5) %"5", align 8
  %"111" = load i32, ptr addrspace(5) %"9", align 4
  %"254" = inttoptr i64 %"110" to ptr
  %"285" = getelementptr inbounds i8, ptr %"254", i64 36
  store i32 %"111", ptr %"285", align 4
  %"112" = load i64, ptr addrspace(5) %"5", align 8
  %"113" = load i32, ptr addrspace(5) %"11", align 4
  %"256" = inttoptr i64 %"112" to ptr
  %"287" = getelementptr inbounds i8, ptr %"256", i64 40
  store i32 %"113", ptr %"287", align 4
  %"114" = load i64, ptr addrspace(5) %"5", align 8
  %"115" = load i32, ptr addrspace(5) %"13", align 4
  %"258" = inttoptr i64 %"114" to ptr
  %"289" = getelementptr inbounds i8, ptr %"258", i64 44
  store i32 %"115", ptr %"289", align 4
  %"116" = load i64, ptr addrspace(5) %"5", align 8
  %"117" = load i32, ptr addrspace(5) %"15", align 4
  %"260" = inttoptr i64 %"116" to ptr
  %"291" = getelementptr inbounds i8, ptr %"260", i64 48
  store i32 %"117", ptr %"291", align 4
  %"118" = load i64, ptr addrspace(5) %"5", align 8
  %"119" = load i32, ptr addrspace(5) %"17", align 4
  %"262" = inttoptr i64 %"118" to ptr
  %"293" = getelementptr inbounds i8, ptr %"262", i64 52
  store i32 %"119", ptr %"293", align 4
  %"120" = load i64, ptr addrspace(5) %"5", align 8
  %"121" = load i32, ptr addrspace(5) %"19", align 4
  %"264" = inttoptr i64 %"120" to ptr
  %"295" = getelementptr inbounds i8, ptr %"264", i64 56
  store i32 %"121", ptr %"295", align 4
  %"122" = load i64, ptr addrspace(5) %"5", align 8
  %"123" = load i32, ptr addrspace(5) %"21", align 4
  %"266" = inttoptr i64 %"122" to ptr
  %"297" = getelementptr inbounds i8, ptr %"266", i64 60
  store i32 %"123", ptr %"297", align 4
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare { i32, i1 } @llvm.usub.with.overflow.i32(i32, i32) #1

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare { i32, i1 } @llvm.uadd.with.overflow.i32(i32, i32) #1

attributes #0 = { "amdgpu-unsafe-fp-atomics"="true" "denormal-fp-math"="ieee,ieee" "denormal-fp-math-f32"="ieee,ieee" "no-trapping-math"="true" "uniform-work-group-size"="true" }
attributes #1 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
