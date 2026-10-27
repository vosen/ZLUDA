// Half precision GEMM kernels of zluda_blas_agx: f16 A and B, f32 accumulation,
// and C in f16 (_cf16) or f32 (_cf32). Like sgemm.mlir, for every AGX
// generation; alpha and beta are f32.

gpu.module @agx_blas_f16 {
  // As agx_sgemm_tile64 in sgemm.mlir, with f16 A and B. C = alpha * A * B + beta * C for row-major matrices with leading
  // dimensions lda, ldb and ldc (in elements); M and N multiples of 64, K a
  // multiple of 8, the leading dimensions even. C is not read when beta is 0.
  //
  // Each threadgroup is 2 x 2 SIMD-groups (128 threads) computing a 64 x 64
  // tile; each SIMD-group a 32 x 32 tile, 4 x 4 8x8 matrices accumulated in
  // registers along K with the operands read straight from device memory.
  // Threadgroups walk the output in groups of 8 tile rows so that concurrently
  // running ones share rows of A and columns of B in the caches, and the
  // SIMD-groups of a threadgroup meet at a barrier every 128 steps of K to keep
  // their working set together. Launch with (M / 64) * (N / 64) threadgroups of
  // 128 threads.
  gpu.func @agx_hgemm_tile64_cf16(%a0: memref<1xf16, 1>, %b0: memref<1xf16, 1>,
      %c0: memref<1xf16, 1>, %M: index, %N: index, %K: index, %lda: index,
      %ldb: index, %ldc: index, %bsa: index, %bsb: index, %bsc: index, %alpha: f32,
      %beta: f32) kernel
      attributes {known_block_size = array<i32: 128, 1, 1>} {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %i2 = arith.constant 2 : index
    %i8 = arith.constant 8 : index
    %i16 = arith.constant 16 : index
    %i24 = arith.constant 24 : index
    %i32 = arith.constant 32 : index
    %i64 = arith.constant 64 : index
    %i128 = arith.constant 128 : index
    %even = arith.constant -2 : index
    %a0a = memref.assume_alignment %a0, 8 : memref<1xf16, 1>
    %b0a = memref.assume_alignment %b0, 8 : memref<1xf16, 1>
    %c0a = memref.assume_alignment %c0, 8 : memref<1xf16, 1>
    %lda_e = arith.andi %lda, %even : index
    %ldb_e = arith.andi %ldb, %even : index
    %ldc_e = arith.andi %ldc, %even : index
    %z = gpu.block_id y
    %bsa_e = arith.andi %bsa, %even : index
    %oa = arith.muli %z, %bsa_e : index
    %bsb_e = arith.andi %bsb, %even : index
    %ob = arith.muli %z, %bsb_e : index
    %bsc_e = arith.andi %bsc, %even : index
    %oc = arith.muli %z, %bsc_e : index
    %A = memref.reinterpret_cast %a0a to offset: [%oa], sizes: [%M, %K], strides: [%lda_e, 1]
        : memref<1xf16, 1> to memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
    %B = memref.reinterpret_cast %b0a to offset: [%ob], sizes: [%K, %N], strides: [%ldb_e, 1]
        : memref<1xf16, 1> to memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
    %C = memref.reinterpret_cast %c0a to offset: [%oc], sizes: [%M, %N], strides: [%ldc_e, 1]
        : memref<1xf16, 1> to memref<?x?xf16, strided<[?, 1], offset: ?>, 1>

    // Grouped tile order: groups of 8 tile rows, column-major inside a group.
    %tiles_m = arith.divui %M, %i64 : index
    %tiles_n = arith.divui %N, %i64 : index
    %id = gpu.block_id x
    %per_group = arith.muli %tiles_n, %i8 : index
    %group = arith.divui %id, %per_group : index
    %first_m = arith.muli %group, %i8 : index
    %left_m = arith.subi %tiles_m, %first_m : index
    %group_m = arith.minui %left_m, %i8 : index
    %in_group = arith.remui %id, %per_group : index
    %rem_m = arith.remui %in_group, %group_m : index
    %tm = arith.addi %first_m, %rem_m : index
    %tn = arith.divui %in_group, %group_m : index

    // This SIMD-group's 32 x 32 tile: 4 x 4 matrices.
    %sg = gpu.subgroup_id : index
    %sr = arith.divui %sg, %i2 : index
    %sc = arith.remui %sg, %i2 : index
    %tm64 = arith.muli %tm, %i64 : index
    %tn64 = arith.muli %tn, %i64 : index
    %sr32 = arith.muli %sr, %i32 : index
    %sc32 = arith.muli %sc, %i32 : index
    %row = arith.addi %tm64, %sr32 : index
    %col = arith.addi %tn64, %sc32 : index
    %row1 = arith.addi %row, %i8 : index
    %col1 = arith.addi %col, %i8 : index
    %row2 = arith.addi %row, %i16 : index
    %col2 = arith.addi %col, %i16 : index
    %row3 = arith.addi %row, %i24 : index
    %col3 = arith.addi %col, %i24 : index
    %fz = arith.constant 0.0 : f32
    %zero = air.simdgroup_matrix_fill %fz : vector<64xf32>
    %acc:16 = scf.for %kk = %i0 to %K step %i128
        iter_args(%o00 = %zero, %o01 = %zero, %o02 = %zero, %o03 = %zero, %o10 = %zero, %o11 = %zero, %o12 = %zero, %o13 = %zero, %o20 = %zero, %o21 = %zero, %o22 = %zero, %o23 = %zero, %o30 = %zero, %o31 = %zero, %o32 = %zero, %o33 = %zero) -> (vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>) {
      gpu.barrier memfence []
      %kk_end0 = arith.addi %kk, %i128 : index
      %kk_end = arith.minui %kk_end0, %K : index
      %in:16 = scf.for %k = %kk to %kk_end step %i8
          iter_args(%a00 = %o00, %a01 = %o01, %a02 = %o02, %a03 = %o03, %a10 = %o10, %a11 = %o11, %a12 = %o12, %a13 = %o13, %a20 = %o20, %a21 = %o21, %a22 = %o22, %a23 = %o23, %a30 = %o30, %a31 = %o31, %a32 = %o32, %a33 = %o33) -> (vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>) {
        %ma0 = air.simdgroup_matrix_load %A[%row, %k] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %ma1 = air.simdgroup_matrix_load %A[%row1, %k] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %ma2 = air.simdgroup_matrix_load %A[%row2, %k] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %ma3 = air.simdgroup_matrix_load %A[%row3, %k] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb0 = air.simdgroup_matrix_load %B[%k, %col] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb1 = air.simdgroup_matrix_load %B[%k, %col1] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb2 = air.simdgroup_matrix_load %B[%k, %col2] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb3 = air.simdgroup_matrix_load %B[%k, %col3] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %n00 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb0, %a00 : vector<64xf16>, vector<64xf32>
        %n01 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb1, %a01 : vector<64xf16>, vector<64xf32>
        %n02 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb2, %a02 : vector<64xf16>, vector<64xf32>
        %n03 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb3, %a03 : vector<64xf16>, vector<64xf32>
        %n10 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb0, %a10 : vector<64xf16>, vector<64xf32>
        %n11 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb1, %a11 : vector<64xf16>, vector<64xf32>
        %n12 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb2, %a12 : vector<64xf16>, vector<64xf32>
        %n13 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb3, %a13 : vector<64xf16>, vector<64xf32>
        %n20 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb0, %a20 : vector<64xf16>, vector<64xf32>
        %n21 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb1, %a21 : vector<64xf16>, vector<64xf32>
        %n22 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb2, %a22 : vector<64xf16>, vector<64xf32>
        %n23 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb3, %a23 : vector<64xf16>, vector<64xf32>
        %n30 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb0, %a30 : vector<64xf16>, vector<64xf32>
        %n31 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb1, %a31 : vector<64xf16>, vector<64xf32>
        %n32 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb2, %a32 : vector<64xf16>, vector<64xf32>
        %n33 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb3, %a33 : vector<64xf16>, vector<64xf32>
        scf.yield %n00, %n01, %n02, %n03, %n10, %n11, %n12, %n13, %n20, %n21, %n22, %n23, %n30, %n31, %n32, %n33 : vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>
      }
      scf.yield %in#0, %in#1, %in#2, %in#3, %in#4, %in#5, %in#6, %in#7, %in#8, %in#9, %in#10, %in#11, %in#12, %in#13, %in#14, %in#15 : vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>
    }

    // C = alpha * acc + beta * C, C not read when beta is 0 (one branch for the tile).
    %no_c = arith.cmpf oeq, %beta, %fz : f32
    %malpha = air.simdgroup_matrix_fill %alpha : vector<64xf32>
    %mbeta = air.simdgroup_matrix_fill %beta : vector<64xf32>
    scf.if %no_c {
      %s_00 = arith.mulf %acc#0, %malpha : vector<64xf32>
      %h_00 = arith.truncf %s_00 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_00, %C[%row, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_01 = arith.mulf %acc#1, %malpha : vector<64xf32>
      %h_01 = arith.truncf %s_01 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_01, %C[%row, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_02 = arith.mulf %acc#2, %malpha : vector<64xf32>
      %h_02 = arith.truncf %s_02 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_02, %C[%row, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_03 = arith.mulf %acc#3, %malpha : vector<64xf32>
      %h_03 = arith.truncf %s_03 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_03, %C[%row, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_10 = arith.mulf %acc#4, %malpha : vector<64xf32>
      %h_10 = arith.truncf %s_10 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_10, %C[%row1, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_11 = arith.mulf %acc#5, %malpha : vector<64xf32>
      %h_11 = arith.truncf %s_11 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_11, %C[%row1, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_12 = arith.mulf %acc#6, %malpha : vector<64xf32>
      %h_12 = arith.truncf %s_12 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_12, %C[%row1, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_13 = arith.mulf %acc#7, %malpha : vector<64xf32>
      %h_13 = arith.truncf %s_13 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_13, %C[%row1, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_20 = arith.mulf %acc#8, %malpha : vector<64xf32>
      %h_20 = arith.truncf %s_20 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_20, %C[%row2, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_21 = arith.mulf %acc#9, %malpha : vector<64xf32>
      %h_21 = arith.truncf %s_21 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_21, %C[%row2, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_22 = arith.mulf %acc#10, %malpha : vector<64xf32>
      %h_22 = arith.truncf %s_22 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_22, %C[%row2, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_23 = arith.mulf %acc#11, %malpha : vector<64xf32>
      %h_23 = arith.truncf %s_23 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_23, %C[%row2, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_30 = arith.mulf %acc#12, %malpha : vector<64xf32>
      %h_30 = arith.truncf %s_30 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_30, %C[%row3, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_31 = arith.mulf %acc#13, %malpha : vector<64xf32>
      %h_31 = arith.truncf %s_31 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_31, %C[%row3, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_32 = arith.mulf %acc#14, %malpha : vector<64xf32>
      %h_32 = arith.truncf %s_32 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_32, %C[%row3, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_33 = arith.mulf %acc#15, %malpha : vector<64xf32>
      %h_33 = arith.truncf %s_33 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_33, %C[%row3, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
    } else {
      %s_00 = arith.mulf %acc#0, %malpha : vector<64xf32>
      %oh_00 = air.simdgroup_matrix_load %C[%row, %col] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_00 = arith.extf %oh_00 : vector<64xf16> to vector<64xf32>
      %r_00 = math.fma %old_00, %mbeta, %s_00 : vector<64xf32>
      %h_00 = arith.truncf %r_00 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_00, %C[%row, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_01 = arith.mulf %acc#1, %malpha : vector<64xf32>
      %oh_01 = air.simdgroup_matrix_load %C[%row, %col1] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_01 = arith.extf %oh_01 : vector<64xf16> to vector<64xf32>
      %r_01 = math.fma %old_01, %mbeta, %s_01 : vector<64xf32>
      %h_01 = arith.truncf %r_01 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_01, %C[%row, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_02 = arith.mulf %acc#2, %malpha : vector<64xf32>
      %oh_02 = air.simdgroup_matrix_load %C[%row, %col2] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_02 = arith.extf %oh_02 : vector<64xf16> to vector<64xf32>
      %r_02 = math.fma %old_02, %mbeta, %s_02 : vector<64xf32>
      %h_02 = arith.truncf %r_02 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_02, %C[%row, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_03 = arith.mulf %acc#3, %malpha : vector<64xf32>
      %oh_03 = air.simdgroup_matrix_load %C[%row, %col3] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_03 = arith.extf %oh_03 : vector<64xf16> to vector<64xf32>
      %r_03 = math.fma %old_03, %mbeta, %s_03 : vector<64xf32>
      %h_03 = arith.truncf %r_03 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_03, %C[%row, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_10 = arith.mulf %acc#4, %malpha : vector<64xf32>
      %oh_10 = air.simdgroup_matrix_load %C[%row1, %col] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_10 = arith.extf %oh_10 : vector<64xf16> to vector<64xf32>
      %r_10 = math.fma %old_10, %mbeta, %s_10 : vector<64xf32>
      %h_10 = arith.truncf %r_10 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_10, %C[%row1, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_11 = arith.mulf %acc#5, %malpha : vector<64xf32>
      %oh_11 = air.simdgroup_matrix_load %C[%row1, %col1] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_11 = arith.extf %oh_11 : vector<64xf16> to vector<64xf32>
      %r_11 = math.fma %old_11, %mbeta, %s_11 : vector<64xf32>
      %h_11 = arith.truncf %r_11 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_11, %C[%row1, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_12 = arith.mulf %acc#6, %malpha : vector<64xf32>
      %oh_12 = air.simdgroup_matrix_load %C[%row1, %col2] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_12 = arith.extf %oh_12 : vector<64xf16> to vector<64xf32>
      %r_12 = math.fma %old_12, %mbeta, %s_12 : vector<64xf32>
      %h_12 = arith.truncf %r_12 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_12, %C[%row1, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_13 = arith.mulf %acc#7, %malpha : vector<64xf32>
      %oh_13 = air.simdgroup_matrix_load %C[%row1, %col3] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_13 = arith.extf %oh_13 : vector<64xf16> to vector<64xf32>
      %r_13 = math.fma %old_13, %mbeta, %s_13 : vector<64xf32>
      %h_13 = arith.truncf %r_13 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_13, %C[%row1, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_20 = arith.mulf %acc#8, %malpha : vector<64xf32>
      %oh_20 = air.simdgroup_matrix_load %C[%row2, %col] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_20 = arith.extf %oh_20 : vector<64xf16> to vector<64xf32>
      %r_20 = math.fma %old_20, %mbeta, %s_20 : vector<64xf32>
      %h_20 = arith.truncf %r_20 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_20, %C[%row2, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_21 = arith.mulf %acc#9, %malpha : vector<64xf32>
      %oh_21 = air.simdgroup_matrix_load %C[%row2, %col1] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_21 = arith.extf %oh_21 : vector<64xf16> to vector<64xf32>
      %r_21 = math.fma %old_21, %mbeta, %s_21 : vector<64xf32>
      %h_21 = arith.truncf %r_21 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_21, %C[%row2, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_22 = arith.mulf %acc#10, %malpha : vector<64xf32>
      %oh_22 = air.simdgroup_matrix_load %C[%row2, %col2] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_22 = arith.extf %oh_22 : vector<64xf16> to vector<64xf32>
      %r_22 = math.fma %old_22, %mbeta, %s_22 : vector<64xf32>
      %h_22 = arith.truncf %r_22 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_22, %C[%row2, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_23 = arith.mulf %acc#11, %malpha : vector<64xf32>
      %oh_23 = air.simdgroup_matrix_load %C[%row2, %col3] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_23 = arith.extf %oh_23 : vector<64xf16> to vector<64xf32>
      %r_23 = math.fma %old_23, %mbeta, %s_23 : vector<64xf32>
      %h_23 = arith.truncf %r_23 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_23, %C[%row2, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_30 = arith.mulf %acc#12, %malpha : vector<64xf32>
      %oh_30 = air.simdgroup_matrix_load %C[%row3, %col] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_30 = arith.extf %oh_30 : vector<64xf16> to vector<64xf32>
      %r_30 = math.fma %old_30, %mbeta, %s_30 : vector<64xf32>
      %h_30 = arith.truncf %r_30 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_30, %C[%row3, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_31 = arith.mulf %acc#13, %malpha : vector<64xf32>
      %oh_31 = air.simdgroup_matrix_load %C[%row3, %col1] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_31 = arith.extf %oh_31 : vector<64xf16> to vector<64xf32>
      %r_31 = math.fma %old_31, %mbeta, %s_31 : vector<64xf32>
      %h_31 = arith.truncf %r_31 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_31, %C[%row3, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_32 = arith.mulf %acc#14, %malpha : vector<64xf32>
      %oh_32 = air.simdgroup_matrix_load %C[%row3, %col2] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_32 = arith.extf %oh_32 : vector<64xf16> to vector<64xf32>
      %r_32 = math.fma %old_32, %mbeta, %s_32 : vector<64xf32>
      %h_32 = arith.truncf %r_32 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_32, %C[%row3, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_33 = arith.mulf %acc#15, %malpha : vector<64xf32>
      %oh_33 = air.simdgroup_matrix_load %C[%row3, %col3] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_33 = arith.extf %oh_33 : vector<64xf16> to vector<64xf32>
      %r_33 = math.fma %old_33, %mbeta, %s_33 : vector<64xf32>
      %h_33 = arith.truncf %r_33 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_33, %C[%row3, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
    }
    gpu.return
  }

  // As agx_sgemm_tile64 in sgemm.mlir, with f16 A and B. C = alpha * A * B + beta * C for row-major matrices with leading
  // dimensions lda, ldb and ldc (in elements); M and N multiples of 64, K a
  // multiple of 8, the leading dimensions even. C is not read when beta is 0.
  //
  // Each threadgroup is 2 x 2 SIMD-groups (128 threads) computing a 64 x 64
  // tile; each SIMD-group a 32 x 32 tile, 4 x 4 8x8 matrices accumulated in
  // registers along K with the operands read straight from device memory.
  // Threadgroups walk the output in groups of 8 tile rows so that concurrently
  // running ones share rows of A and columns of B in the caches, and the
  // SIMD-groups of a threadgroup meet at a barrier every 128 steps of K to keep
  // their working set together. Launch with (M / 64) * (N / 64) threadgroups of
  // 128 threads.
  gpu.func @agx_hgemm_tile64_cf32(%a0: memref<1xf16, 1>, %b0: memref<1xf16, 1>,
      %c0: memref<1xf32, 1>, %M: index, %N: index, %K: index, %lda: index,
      %ldb: index, %ldc: index, %bsa: index, %bsb: index, %bsc: index, %alpha: f32,
      %beta: f32) kernel
      attributes {known_block_size = array<i32: 128, 1, 1>} {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %i2 = arith.constant 2 : index
    %i8 = arith.constant 8 : index
    %i16 = arith.constant 16 : index
    %i24 = arith.constant 24 : index
    %i32 = arith.constant 32 : index
    %i64 = arith.constant 64 : index
    %i128 = arith.constant 128 : index
    %even = arith.constant -2 : index
    %a0a = memref.assume_alignment %a0, 8 : memref<1xf16, 1>
    %b0a = memref.assume_alignment %b0, 8 : memref<1xf16, 1>
    %c0a = memref.assume_alignment %c0, 8 : memref<1xf32, 1>
    %lda_e = arith.andi %lda, %even : index
    %ldb_e = arith.andi %ldb, %even : index
    %ldc_e = arith.andi %ldc, %even : index
    %z = gpu.block_id y
    %bsa_e = arith.andi %bsa, %even : index
    %oa = arith.muli %z, %bsa_e : index
    %bsb_e = arith.andi %bsb, %even : index
    %ob = arith.muli %z, %bsb_e : index
    %bsc_e = arith.andi %bsc, %even : index
    %oc = arith.muli %z, %bsc_e : index
    %A = memref.reinterpret_cast %a0a to offset: [%oa], sizes: [%M, %K], strides: [%lda_e, 1]
        : memref<1xf16, 1> to memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
    %B = memref.reinterpret_cast %b0a to offset: [%ob], sizes: [%K, %N], strides: [%ldb_e, 1]
        : memref<1xf16, 1> to memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
    %C = memref.reinterpret_cast %c0a to offset: [%oc], sizes: [%M, %N], strides: [%ldc_e, 1]
        : memref<1xf32, 1> to memref<?x?xf32, strided<[?, 1], offset: ?>, 1>

    // Grouped tile order: groups of 8 tile rows, column-major inside a group.
    %tiles_m = arith.divui %M, %i64 : index
    %tiles_n = arith.divui %N, %i64 : index
    %id = gpu.block_id x
    %per_group = arith.muli %tiles_n, %i8 : index
    %group = arith.divui %id, %per_group : index
    %first_m = arith.muli %group, %i8 : index
    %left_m = arith.subi %tiles_m, %first_m : index
    %group_m = arith.minui %left_m, %i8 : index
    %in_group = arith.remui %id, %per_group : index
    %rem_m = arith.remui %in_group, %group_m : index
    %tm = arith.addi %first_m, %rem_m : index
    %tn = arith.divui %in_group, %group_m : index

    // This SIMD-group's 32 x 32 tile: 4 x 4 matrices.
    %sg = gpu.subgroup_id : index
    %sr = arith.divui %sg, %i2 : index
    %sc = arith.remui %sg, %i2 : index
    %tm64 = arith.muli %tm, %i64 : index
    %tn64 = arith.muli %tn, %i64 : index
    %sr32 = arith.muli %sr, %i32 : index
    %sc32 = arith.muli %sc, %i32 : index
    %row = arith.addi %tm64, %sr32 : index
    %col = arith.addi %tn64, %sc32 : index
    %row1 = arith.addi %row, %i8 : index
    %col1 = arith.addi %col, %i8 : index
    %row2 = arith.addi %row, %i16 : index
    %col2 = arith.addi %col, %i16 : index
    %row3 = arith.addi %row, %i24 : index
    %col3 = arith.addi %col, %i24 : index
    %fz = arith.constant 0.0 : f32
    %zero = air.simdgroup_matrix_fill %fz : vector<64xf32>
    %acc:16 = scf.for %kk = %i0 to %K step %i128
        iter_args(%o00 = %zero, %o01 = %zero, %o02 = %zero, %o03 = %zero, %o10 = %zero, %o11 = %zero, %o12 = %zero, %o13 = %zero, %o20 = %zero, %o21 = %zero, %o22 = %zero, %o23 = %zero, %o30 = %zero, %o31 = %zero, %o32 = %zero, %o33 = %zero) -> (vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>) {
      gpu.barrier memfence []
      %kk_end0 = arith.addi %kk, %i128 : index
      %kk_end = arith.minui %kk_end0, %K : index
      %in:16 = scf.for %k = %kk to %kk_end step %i8
          iter_args(%a00 = %o00, %a01 = %o01, %a02 = %o02, %a03 = %o03, %a10 = %o10, %a11 = %o11, %a12 = %o12, %a13 = %o13, %a20 = %o20, %a21 = %o21, %a22 = %o22, %a23 = %o23, %a30 = %o30, %a31 = %o31, %a32 = %o32, %a33 = %o33) -> (vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>) {
        %ma0 = air.simdgroup_matrix_load %A[%row, %k] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %ma1 = air.simdgroup_matrix_load %A[%row1, %k] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %ma2 = air.simdgroup_matrix_load %A[%row2, %k] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %ma3 = air.simdgroup_matrix_load %A[%row3, %k] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb0 = air.simdgroup_matrix_load %B[%k, %col] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb1 = air.simdgroup_matrix_load %B[%k, %col1] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb2 = air.simdgroup_matrix_load %B[%k, %col2] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb3 = air.simdgroup_matrix_load %B[%k, %col3] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %n00 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb0, %a00 : vector<64xf16>, vector<64xf32>
        %n01 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb1, %a01 : vector<64xf16>, vector<64xf32>
        %n02 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb2, %a02 : vector<64xf16>, vector<64xf32>
        %n03 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb3, %a03 : vector<64xf16>, vector<64xf32>
        %n10 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb0, %a10 : vector<64xf16>, vector<64xf32>
        %n11 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb1, %a11 : vector<64xf16>, vector<64xf32>
        %n12 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb2, %a12 : vector<64xf16>, vector<64xf32>
        %n13 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb3, %a13 : vector<64xf16>, vector<64xf32>
        %n20 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb0, %a20 : vector<64xf16>, vector<64xf32>
        %n21 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb1, %a21 : vector<64xf16>, vector<64xf32>
        %n22 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb2, %a22 : vector<64xf16>, vector<64xf32>
        %n23 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb3, %a23 : vector<64xf16>, vector<64xf32>
        %n30 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb0, %a30 : vector<64xf16>, vector<64xf32>
        %n31 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb1, %a31 : vector<64xf16>, vector<64xf32>
        %n32 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb2, %a32 : vector<64xf16>, vector<64xf32>
        %n33 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb3, %a33 : vector<64xf16>, vector<64xf32>
        scf.yield %n00, %n01, %n02, %n03, %n10, %n11, %n12, %n13, %n20, %n21, %n22, %n23, %n30, %n31, %n32, %n33 : vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>
      }
      scf.yield %in#0, %in#1, %in#2, %in#3, %in#4, %in#5, %in#6, %in#7, %in#8, %in#9, %in#10, %in#11, %in#12, %in#13, %in#14, %in#15 : vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>
    }

    // C = alpha * acc + beta * C, C not read when beta is 0 (one branch for the tile).
    %no_c = arith.cmpf oeq, %beta, %fz : f32
    %malpha = air.simdgroup_matrix_fill %alpha : vector<64xf32>
    %mbeta = air.simdgroup_matrix_fill %beta : vector<64xf32>
    scf.if %no_c {
      %s_00 = arith.mulf %acc#0, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_00, %C[%row, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_01 = arith.mulf %acc#1, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_01, %C[%row, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_02 = arith.mulf %acc#2, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_02, %C[%row, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_03 = arith.mulf %acc#3, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_03, %C[%row, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_10 = arith.mulf %acc#4, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_10, %C[%row1, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_11 = arith.mulf %acc#5, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_11, %C[%row1, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_12 = arith.mulf %acc#6, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_12, %C[%row1, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_13 = arith.mulf %acc#7, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_13, %C[%row1, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_20 = arith.mulf %acc#8, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_20, %C[%row2, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_21 = arith.mulf %acc#9, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_21, %C[%row2, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_22 = arith.mulf %acc#10, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_22, %C[%row2, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_23 = arith.mulf %acc#11, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_23, %C[%row2, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_30 = arith.mulf %acc#12, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_30, %C[%row3, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_31 = arith.mulf %acc#13, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_31, %C[%row3, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_32 = arith.mulf %acc#14, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_32, %C[%row3, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_33 = arith.mulf %acc#15, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_33, %C[%row3, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
    } else {
      %s_00 = arith.mulf %acc#0, %malpha : vector<64xf32>
      %old_00 = air.simdgroup_matrix_load %C[%row, %col] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_00 = math.fma %old_00, %mbeta, %s_00 : vector<64xf32>
      air.simdgroup_matrix_store %r_00, %C[%row, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_01 = arith.mulf %acc#1, %malpha : vector<64xf32>
      %old_01 = air.simdgroup_matrix_load %C[%row, %col1] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_01 = math.fma %old_01, %mbeta, %s_01 : vector<64xf32>
      air.simdgroup_matrix_store %r_01, %C[%row, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_02 = arith.mulf %acc#2, %malpha : vector<64xf32>
      %old_02 = air.simdgroup_matrix_load %C[%row, %col2] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_02 = math.fma %old_02, %mbeta, %s_02 : vector<64xf32>
      air.simdgroup_matrix_store %r_02, %C[%row, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_03 = arith.mulf %acc#3, %malpha : vector<64xf32>
      %old_03 = air.simdgroup_matrix_load %C[%row, %col3] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_03 = math.fma %old_03, %mbeta, %s_03 : vector<64xf32>
      air.simdgroup_matrix_store %r_03, %C[%row, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_10 = arith.mulf %acc#4, %malpha : vector<64xf32>
      %old_10 = air.simdgroup_matrix_load %C[%row1, %col] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_10 = math.fma %old_10, %mbeta, %s_10 : vector<64xf32>
      air.simdgroup_matrix_store %r_10, %C[%row1, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_11 = arith.mulf %acc#5, %malpha : vector<64xf32>
      %old_11 = air.simdgroup_matrix_load %C[%row1, %col1] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_11 = math.fma %old_11, %mbeta, %s_11 : vector<64xf32>
      air.simdgroup_matrix_store %r_11, %C[%row1, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_12 = arith.mulf %acc#6, %malpha : vector<64xf32>
      %old_12 = air.simdgroup_matrix_load %C[%row1, %col2] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_12 = math.fma %old_12, %mbeta, %s_12 : vector<64xf32>
      air.simdgroup_matrix_store %r_12, %C[%row1, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_13 = arith.mulf %acc#7, %malpha : vector<64xf32>
      %old_13 = air.simdgroup_matrix_load %C[%row1, %col3] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_13 = math.fma %old_13, %mbeta, %s_13 : vector<64xf32>
      air.simdgroup_matrix_store %r_13, %C[%row1, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_20 = arith.mulf %acc#8, %malpha : vector<64xf32>
      %old_20 = air.simdgroup_matrix_load %C[%row2, %col] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_20 = math.fma %old_20, %mbeta, %s_20 : vector<64xf32>
      air.simdgroup_matrix_store %r_20, %C[%row2, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_21 = arith.mulf %acc#9, %malpha : vector<64xf32>
      %old_21 = air.simdgroup_matrix_load %C[%row2, %col1] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_21 = math.fma %old_21, %mbeta, %s_21 : vector<64xf32>
      air.simdgroup_matrix_store %r_21, %C[%row2, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_22 = arith.mulf %acc#10, %malpha : vector<64xf32>
      %old_22 = air.simdgroup_matrix_load %C[%row2, %col2] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_22 = math.fma %old_22, %mbeta, %s_22 : vector<64xf32>
      air.simdgroup_matrix_store %r_22, %C[%row2, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_23 = arith.mulf %acc#11, %malpha : vector<64xf32>
      %old_23 = air.simdgroup_matrix_load %C[%row2, %col3] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_23 = math.fma %old_23, %mbeta, %s_23 : vector<64xf32>
      air.simdgroup_matrix_store %r_23, %C[%row2, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_30 = arith.mulf %acc#12, %malpha : vector<64xf32>
      %old_30 = air.simdgroup_matrix_load %C[%row3, %col] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_30 = math.fma %old_30, %mbeta, %s_30 : vector<64xf32>
      air.simdgroup_matrix_store %r_30, %C[%row3, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_31 = arith.mulf %acc#13, %malpha : vector<64xf32>
      %old_31 = air.simdgroup_matrix_load %C[%row3, %col1] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_31 = math.fma %old_31, %mbeta, %s_31 : vector<64xf32>
      air.simdgroup_matrix_store %r_31, %C[%row3, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_32 = arith.mulf %acc#14, %malpha : vector<64xf32>
      %old_32 = air.simdgroup_matrix_load %C[%row3, %col2] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_32 = math.fma %old_32, %mbeta, %s_32 : vector<64xf32>
      air.simdgroup_matrix_store %r_32, %C[%row3, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_33 = arith.mulf %acc#15, %malpha : vector<64xf32>
      %old_33 = air.simdgroup_matrix_load %C[%row3, %col3] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_33 = math.fma %old_33, %mbeta, %s_33 : vector<64xf32>
      air.simdgroup_matrix_store %r_33, %C[%row3, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
    }
    gpu.return
  }

  // As agx_sgemm_generic, with f16 A and B. C = alpha * A * B + beta * C for M x K, K x N and M x N matrices with
  // arbitrary element strides (row stride, column stride) for each, which
  // covers transposed operands and column-major storage. One thread per
  // element of C: launch an N x M grid (x along columns) in any threadgroups.
  // C is not read when beta is 0.
  gpu.func @agx_hgemm_generic_cf16(%a0: memref<1xf16, 1>, %b0: memref<1xf16, 1>,
      %c0: memref<1xf16, 1>, %M: index, %N: index, %K: index, %sar: index, %sac: index,
      %sbr: index, %sbc: index, %scr: index, %scc: index, %sa: index, %sb: index, %sc: index,
      %alpha: f32, %beta: f32) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %i3 = arith.constant 3 : index
    %i4 = arith.constant 4 : index
    %i32 = arith.constant 32 : index
    %i96 = arith.constant 96 : index
    %i127 = arith.constant 127 : index
    %i128 = arith.constant 128 : index
    %big = arith.constant 4611686018427387904 : index
    %z = gpu.block_id z
    %oa = arith.muli %z, %sa : index
    %ob = arith.muli %z, %sb : index
    %oc = arith.muli %z, %sc : index
    %A = memref.reinterpret_cast %a0 to offset: [%oa], sizes: [%big], strides: [1]
        : memref<1xf16, 1> to memref<?xf16, strided<[1], offset: ?>, 1>
    %B = memref.reinterpret_cast %b0 to offset: [%ob], sizes: [%big], strides: [1]
        : memref<1xf16, 1> to memref<?xf16, strided<[1], offset: ?>, 1>
    %C = memref.reinterpret_cast %c0 to offset: [%oc], sizes: [%big], strides: [1]
        : memref<1xf16, 1> to memref<?xf16, strided<[1], offset: ?>, 1>
    %f0 = arith.constant 0.0 : f32
    %j = gpu.global_id x
    %i = gpu.global_id y
    %in_m = arith.cmpi ult, %i, %M : index
    %in_n = arith.cmpi ult, %j, %N : index
    %live = arith.andi %in_m, %in_n : i1
    scf.if %live {
      %ia = arith.muli %i, %sar : index
      %jb = arith.muli %j, %sbc : index
      // four steps at a time (four loads of each operand in flight), then one
      %Kx = arith.maxui %K, %i3 : index
      %ub4 = arith.subi %Kx, %i3 : index
      %s4:4 = scf.for %k = %i0 to %ub4 step %i4
          iter_args(%c0_ = %f0, %c1_ = %f0, %c2_ = %f0, %c3_ = %f0) -> (f32, f32, f32, f32) {
        %k1 = arith.addi %k, %i1 : index
        %k2 = arith.addi %k1, %i1 : index
        %k3 = arith.addi %k2, %i1 : index
        %ka_d0 = arith.muli %k, %sac : index
        %kb_d0 = arith.muli %k, %sbr : index
        %pa_d0 = arith.addi %ia, %ka_d0 : index
        %pb_d0 = arith.addi %kb_d0, %jb : index
        %xn_d0 = memref.load %A[%pa_d0] : memref<?xf16, strided<[1], offset: ?>, 1>
        %yn_d0 = memref.load %B[%pb_d0] : memref<?xf16, strided<[1], offset: ?>, 1>
        %x_d0 = arith.extf %xn_d0 : f16 to f32
        %y_d0 = arith.extf %yn_d0 : f16 to f32
        %d0 = math.fma %x_d0, %y_d0, %c0_ : f32
        %ka_d1 = arith.muli %k1, %sac : index
        %kb_d1 = arith.muli %k1, %sbr : index
        %pa_d1 = arith.addi %ia, %ka_d1 : index
        %pb_d1 = arith.addi %kb_d1, %jb : index
        %xn_d1 = memref.load %A[%pa_d1] : memref<?xf16, strided<[1], offset: ?>, 1>
        %yn_d1 = memref.load %B[%pb_d1] : memref<?xf16, strided<[1], offset: ?>, 1>
        %x_d1 = arith.extf %xn_d1 : f16 to f32
        %y_d1 = arith.extf %yn_d1 : f16 to f32
        %d1 = math.fma %x_d1, %y_d1, %c1_ : f32
        %ka_d2 = arith.muli %k2, %sac : index
        %kb_d2 = arith.muli %k2, %sbr : index
        %pa_d2 = arith.addi %ia, %ka_d2 : index
        %pb_d2 = arith.addi %kb_d2, %jb : index
        %xn_d2 = memref.load %A[%pa_d2] : memref<?xf16, strided<[1], offset: ?>, 1>
        %yn_d2 = memref.load %B[%pb_d2] : memref<?xf16, strided<[1], offset: ?>, 1>
        %x_d2 = arith.extf %xn_d2 : f16 to f32
        %y_d2 = arith.extf %yn_d2 : f16 to f32
        %d2 = math.fma %x_d2, %y_d2, %c2_ : f32
        %ka_d3 = arith.muli %k3, %sac : index
        %kb_d3 = arith.muli %k3, %sbr : index
        %pa_d3 = arith.addi %ia, %ka_d3 : index
        %pb_d3 = arith.addi %kb_d3, %jb : index
        %xn_d3 = memref.load %A[%pa_d3] : memref<?xf16, strided<[1], offset: ?>, 1>
        %yn_d3 = memref.load %B[%pb_d3] : memref<?xf16, strided<[1], offset: ?>, 1>
        %x_d3 = arith.extf %xn_d3 : f16 to f32
        %y_d3 = arith.extf %yn_d3 : f16 to f32
        %d3 = math.fma %x_d3, %y_d3, %c3_ : f32
        scf.yield %d0, %d1, %d2, %d3 : f32, f32, f32, f32
      }
      // the rest from the first k the loop above did not reach
      %after = arith.cmpi ugt, %ub4, %i0 : index
      %span = arith.subi %ub4, %i0 : index
      %span1 = arith.addi %span, %i3 : index
      %trips = arith.divui %span1, %i4 : index
      %done = arith.muli %trips, %i4 : index
      %krest0 = arith.addi %i0, %done : index
      %krest = arith.select %after, %krest0, %i0 : index
      %s01 = arith.addf %s4#0, %s4#1 : f32
      %s23 = arith.addf %s4#2, %s4#3 : f32
      %sum0 = arith.addf %s01, %s23 : f32
      %part = scf.for %k = %krest to %K step %i1 iter_args(%a = %sum0) -> f32 {
        %ka_a1 = arith.muli %k, %sac : index
        %kb_a1 = arith.muli %k, %sbr : index
        %pa_a1 = arith.addi %ia, %ka_a1 : index
        %pb_a1 = arith.addi %kb_a1, %jb : index
        %xn_a1 = memref.load %A[%pa_a1] : memref<?xf16, strided<[1], offset: ?>, 1>
        %yn_a1 = memref.load %B[%pb_a1] : memref<?xf16, strided<[1], offset: ?>, 1>
        %x_a1 = arith.extf %xn_a1 : f16 to f32
        %y_a1 = arith.extf %yn_a1 : f16 to f32
        %a1 = math.fma %x_a1, %y_a1, %a : f32
        scf.yield %a1 : f32
      }
      %ic = arith.muli %i, %scr : index
      %jc = arith.muli %j, %scc : index
      %pc = arith.addi %ic, %jc : index
      %scaled = arith.mulf %part, %alpha : f32
      %no_c = arith.cmpf oeq, %beta, %f0 : f32
      %out = scf.if %no_c -> f32 {
        scf.yield %scaled : f32
      } else {
        %oldn = memref.load %C[%pc] : memref<?xf16, strided<[1], offset: ?>, 1>
        %old = arith.extf %oldn : f16 to f32
        %r = math.fma %old, %beta, %scaled : f32
        scf.yield %r : f32
      }
      %outn = arith.truncf %out : f32 to f16
      memref.store %outn, %C[%pc] : memref<?xf16, strided<[1], offset: ?>, 1>
    }
    gpu.return
  }
  // As agx_sgemm_generic, with f16 A and B. C = alpha * A * B + beta * C for M x K, K x N and M x N matrices with
  // arbitrary element strides (row stride, column stride) for each, which
  // covers transposed operands and column-major storage. One thread per
  // element of C: launch an N x M grid (x along columns) in any threadgroups.
  // C is not read when beta is 0.
  gpu.func @agx_hgemm_generic_cf32(%a0: memref<1xf16, 1>, %b0: memref<1xf16, 1>,
      %c0: memref<1xf32, 1>, %M: index, %N: index, %K: index, %sar: index, %sac: index,
      %sbr: index, %sbc: index, %scr: index, %scc: index, %sa: index, %sb: index, %sc: index,
      %alpha: f32, %beta: f32) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %i3 = arith.constant 3 : index
    %i4 = arith.constant 4 : index
    %i32 = arith.constant 32 : index
    %i96 = arith.constant 96 : index
    %i127 = arith.constant 127 : index
    %i128 = arith.constant 128 : index
    %big = arith.constant 4611686018427387904 : index
    %z = gpu.block_id z
    %oa = arith.muli %z, %sa : index
    %ob = arith.muli %z, %sb : index
    %oc = arith.muli %z, %sc : index
    %A = memref.reinterpret_cast %a0 to offset: [%oa], sizes: [%big], strides: [1]
        : memref<1xf16, 1> to memref<?xf16, strided<[1], offset: ?>, 1>
    %B = memref.reinterpret_cast %b0 to offset: [%ob], sizes: [%big], strides: [1]
        : memref<1xf16, 1> to memref<?xf16, strided<[1], offset: ?>, 1>
    %C = memref.reinterpret_cast %c0 to offset: [%oc], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1], offset: ?>, 1>
    %f0 = arith.constant 0.0 : f32
    %j = gpu.global_id x
    %i = gpu.global_id y
    %in_m = arith.cmpi ult, %i, %M : index
    %in_n = arith.cmpi ult, %j, %N : index
    %live = arith.andi %in_m, %in_n : i1
    scf.if %live {
      %ia = arith.muli %i, %sar : index
      %jb = arith.muli %j, %sbc : index
      // four steps at a time (four loads of each operand in flight), then one
      %Kx = arith.maxui %K, %i3 : index
      %ub4 = arith.subi %Kx, %i3 : index
      %s4:4 = scf.for %k = %i0 to %ub4 step %i4
          iter_args(%c0_ = %f0, %c1_ = %f0, %c2_ = %f0, %c3_ = %f0) -> (f32, f32, f32, f32) {
        %k1 = arith.addi %k, %i1 : index
        %k2 = arith.addi %k1, %i1 : index
        %k3 = arith.addi %k2, %i1 : index
        %ka_d0 = arith.muli %k, %sac : index
        %kb_d0 = arith.muli %k, %sbr : index
        %pa_d0 = arith.addi %ia, %ka_d0 : index
        %pb_d0 = arith.addi %kb_d0, %jb : index
        %xn_d0 = memref.load %A[%pa_d0] : memref<?xf16, strided<[1], offset: ?>, 1>
        %yn_d0 = memref.load %B[%pb_d0] : memref<?xf16, strided<[1], offset: ?>, 1>
        %x_d0 = arith.extf %xn_d0 : f16 to f32
        %y_d0 = arith.extf %yn_d0 : f16 to f32
        %d0 = math.fma %x_d0, %y_d0, %c0_ : f32
        %ka_d1 = arith.muli %k1, %sac : index
        %kb_d1 = arith.muli %k1, %sbr : index
        %pa_d1 = arith.addi %ia, %ka_d1 : index
        %pb_d1 = arith.addi %kb_d1, %jb : index
        %xn_d1 = memref.load %A[%pa_d1] : memref<?xf16, strided<[1], offset: ?>, 1>
        %yn_d1 = memref.load %B[%pb_d1] : memref<?xf16, strided<[1], offset: ?>, 1>
        %x_d1 = arith.extf %xn_d1 : f16 to f32
        %y_d1 = arith.extf %yn_d1 : f16 to f32
        %d1 = math.fma %x_d1, %y_d1, %c1_ : f32
        %ka_d2 = arith.muli %k2, %sac : index
        %kb_d2 = arith.muli %k2, %sbr : index
        %pa_d2 = arith.addi %ia, %ka_d2 : index
        %pb_d2 = arith.addi %kb_d2, %jb : index
        %xn_d2 = memref.load %A[%pa_d2] : memref<?xf16, strided<[1], offset: ?>, 1>
        %yn_d2 = memref.load %B[%pb_d2] : memref<?xf16, strided<[1], offset: ?>, 1>
        %x_d2 = arith.extf %xn_d2 : f16 to f32
        %y_d2 = arith.extf %yn_d2 : f16 to f32
        %d2 = math.fma %x_d2, %y_d2, %c2_ : f32
        %ka_d3 = arith.muli %k3, %sac : index
        %kb_d3 = arith.muli %k3, %sbr : index
        %pa_d3 = arith.addi %ia, %ka_d3 : index
        %pb_d3 = arith.addi %kb_d3, %jb : index
        %xn_d3 = memref.load %A[%pa_d3] : memref<?xf16, strided<[1], offset: ?>, 1>
        %yn_d3 = memref.load %B[%pb_d3] : memref<?xf16, strided<[1], offset: ?>, 1>
        %x_d3 = arith.extf %xn_d3 : f16 to f32
        %y_d3 = arith.extf %yn_d3 : f16 to f32
        %d3 = math.fma %x_d3, %y_d3, %c3_ : f32
        scf.yield %d0, %d1, %d2, %d3 : f32, f32, f32, f32
      }
      // the rest from the first k the loop above did not reach
      %after = arith.cmpi ugt, %ub4, %i0 : index
      %span = arith.subi %ub4, %i0 : index
      %span1 = arith.addi %span, %i3 : index
      %trips = arith.divui %span1, %i4 : index
      %done = arith.muli %trips, %i4 : index
      %krest0 = arith.addi %i0, %done : index
      %krest = arith.select %after, %krest0, %i0 : index
      %s01 = arith.addf %s4#0, %s4#1 : f32
      %s23 = arith.addf %s4#2, %s4#3 : f32
      %sum0 = arith.addf %s01, %s23 : f32
      %part = scf.for %k = %krest to %K step %i1 iter_args(%a = %sum0) -> f32 {
        %ka_a1 = arith.muli %k, %sac : index
        %kb_a1 = arith.muli %k, %sbr : index
        %pa_a1 = arith.addi %ia, %ka_a1 : index
        %pb_a1 = arith.addi %kb_a1, %jb : index
        %xn_a1 = memref.load %A[%pa_a1] : memref<?xf16, strided<[1], offset: ?>, 1>
        %yn_a1 = memref.load %B[%pb_a1] : memref<?xf16, strided<[1], offset: ?>, 1>
        %x_a1 = arith.extf %xn_a1 : f16 to f32
        %y_a1 = arith.extf %yn_a1 : f16 to f32
        %a1 = math.fma %x_a1, %y_a1, %a : f32
        scf.yield %a1 : f32
      }
      %ic = arith.muli %i, %scr : index
      %jc = arith.muli %j, %scc : index
      %pc = arith.addi %ic, %jc : index
      %scaled = arith.mulf %part, %alpha : f32
      %no_c = arith.cmpf oeq, %beta, %f0 : f32
      %out = scf.if %no_c -> f32 {
        scf.yield %scaled : f32
      } else {
        %old = memref.load %C[%pc] : memref<?xf32, strided<[1], offset: ?>, 1>
        %r = math.fma %old, %beta, %scaled : f32
        scf.yield %r : f32
      }
      memref.store %out, %C[%pc] : memref<?xf32, strided<[1], offset: ?>, 1>
    }
    gpu.return
  }

  // agx_hgemm_tile64_cf16 with A[r, k] at a[k * lda + r] (transposed).
  gpu.func @agx_hgemm_tile64_cf16_ta(%a0: memref<1xf16, 1>, %b0: memref<1xf16, 1>,
      %c0: memref<1xf16, 1>, %M: index, %N: index, %K: index, %lda: index,
      %ldb: index, %ldc: index, %bsa: index, %bsb: index, %bsc: index, %alpha: f32,
      %beta: f32) kernel
      attributes {known_block_size = array<i32: 128, 1, 1>} {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %i2 = arith.constant 2 : index
    %i8 = arith.constant 8 : index
    %i16 = arith.constant 16 : index
    %i24 = arith.constant 24 : index
    %i32 = arith.constant 32 : index
    %i64 = arith.constant 64 : index
    %i128 = arith.constant 128 : index
    %even = arith.constant -2 : index
    %a0a = memref.assume_alignment %a0, 8 : memref<1xf16, 1>
    %b0a = memref.assume_alignment %b0, 8 : memref<1xf16, 1>
    %c0a = memref.assume_alignment %c0, 8 : memref<1xf16, 1>
    %lda_e = arith.andi %lda, %even : index
    %ldb_e = arith.andi %ldb, %even : index
    %ldc_e = arith.andi %ldc, %even : index
    %z = gpu.block_id y
    %bsa_e = arith.andi %bsa, %even : index
    %oa = arith.muli %z, %bsa_e : index
    %bsb_e = arith.andi %bsb, %even : index
    %ob = arith.muli %z, %bsb_e : index
    %bsc_e = arith.andi %bsc, %even : index
    %oc = arith.muli %z, %bsc_e : index
    %A = memref.reinterpret_cast %a0a to offset: [%oa], sizes: [%K, %M], strides: [%lda_e, 1]
        : memref<1xf16, 1> to memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
    %B = memref.reinterpret_cast %b0a to offset: [%ob], sizes: [%K, %N], strides: [%ldb_e, 1]
        : memref<1xf16, 1> to memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
    %C = memref.reinterpret_cast %c0a to offset: [%oc], sizes: [%M, %N], strides: [%ldc_e, 1]
        : memref<1xf16, 1> to memref<?x?xf16, strided<[?, 1], offset: ?>, 1>

    // Grouped tile order: groups of 8 tile rows, column-major inside a group.
    %tiles_m = arith.divui %M, %i64 : index
    %tiles_n = arith.divui %N, %i64 : index
    %id = gpu.block_id x
    %per_group = arith.muli %tiles_n, %i8 : index
    %group = arith.divui %id, %per_group : index
    %first_m = arith.muli %group, %i8 : index
    %left_m = arith.subi %tiles_m, %first_m : index
    %group_m = arith.minui %left_m, %i8 : index
    %in_group = arith.remui %id, %per_group : index
    %rem_m = arith.remui %in_group, %group_m : index
    %tm = arith.addi %first_m, %rem_m : index
    %tn = arith.divui %in_group, %group_m : index

    // This SIMD-group's 32 x 32 tile: 4 x 4 matrices.
    %sg = gpu.subgroup_id : index
    %sr = arith.divui %sg, %i2 : index
    %sc = arith.remui %sg, %i2 : index
    %tm64 = arith.muli %tm, %i64 : index
    %tn64 = arith.muli %tn, %i64 : index
    %sr32 = arith.muli %sr, %i32 : index
    %sc32 = arith.muli %sc, %i32 : index
    %row = arith.addi %tm64, %sr32 : index
    %col = arith.addi %tn64, %sc32 : index
    %row1 = arith.addi %row, %i8 : index
    %col1 = arith.addi %col, %i8 : index
    %row2 = arith.addi %row, %i16 : index
    %col2 = arith.addi %col, %i16 : index
    %row3 = arith.addi %row, %i24 : index
    %col3 = arith.addi %col, %i24 : index
    %fz = arith.constant 0.0 : f32
    %zero = air.simdgroup_matrix_fill %fz : vector<64xf32>
    %acc:16 = scf.for %kk = %i0 to %K step %i128
        iter_args(%o00 = %zero, %o01 = %zero, %o02 = %zero, %o03 = %zero, %o10 = %zero, %o11 = %zero, %o12 = %zero, %o13 = %zero, %o20 = %zero, %o21 = %zero, %o22 = %zero, %o23 = %zero, %o30 = %zero, %o31 = %zero, %o32 = %zero, %o33 = %zero) -> (vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>) {
      gpu.barrier memfence []
      %kk_end0 = arith.addi %kk, %i128 : index
      %kk_end = arith.minui %kk_end0, %K : index
      %in:16 = scf.for %k = %kk to %kk_end step %i8
          iter_args(%a00 = %o00, %a01 = %o01, %a02 = %o02, %a03 = %o03, %a10 = %o10, %a11 = %o11, %a12 = %o12, %a13 = %o13, %a20 = %o20, %a21 = %o21, %a22 = %o22, %a23 = %o23, %a30 = %o30, %a31 = %o31, %a32 = %o32, %a33 = %o33) -> (vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>) {
        %ma0 = air.simdgroup_matrix_load %A[%k, %row] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %ma1 = air.simdgroup_matrix_load %A[%k, %row1] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %ma2 = air.simdgroup_matrix_load %A[%k, %row2] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %ma3 = air.simdgroup_matrix_load %A[%k, %row3] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb0 = air.simdgroup_matrix_load %B[%k, %col] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb1 = air.simdgroup_matrix_load %B[%k, %col1] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb2 = air.simdgroup_matrix_load %B[%k, %col2] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb3 = air.simdgroup_matrix_load %B[%k, %col3] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %n00 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb0, %a00 : vector<64xf16>, vector<64xf32>
        %n01 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb1, %a01 : vector<64xf16>, vector<64xf32>
        %n02 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb2, %a02 : vector<64xf16>, vector<64xf32>
        %n03 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb3, %a03 : vector<64xf16>, vector<64xf32>
        %n10 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb0, %a10 : vector<64xf16>, vector<64xf32>
        %n11 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb1, %a11 : vector<64xf16>, vector<64xf32>
        %n12 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb2, %a12 : vector<64xf16>, vector<64xf32>
        %n13 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb3, %a13 : vector<64xf16>, vector<64xf32>
        %n20 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb0, %a20 : vector<64xf16>, vector<64xf32>
        %n21 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb1, %a21 : vector<64xf16>, vector<64xf32>
        %n22 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb2, %a22 : vector<64xf16>, vector<64xf32>
        %n23 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb3, %a23 : vector<64xf16>, vector<64xf32>
        %n30 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb0, %a30 : vector<64xf16>, vector<64xf32>
        %n31 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb1, %a31 : vector<64xf16>, vector<64xf32>
        %n32 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb2, %a32 : vector<64xf16>, vector<64xf32>
        %n33 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb3, %a33 : vector<64xf16>, vector<64xf32>
        scf.yield %n00, %n01, %n02, %n03, %n10, %n11, %n12, %n13, %n20, %n21, %n22, %n23, %n30, %n31, %n32, %n33 : vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>
      }
      scf.yield %in#0, %in#1, %in#2, %in#3, %in#4, %in#5, %in#6, %in#7, %in#8, %in#9, %in#10, %in#11, %in#12, %in#13, %in#14, %in#15 : vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>
    }

    // C = alpha * acc + beta * C, C not read when beta is 0 (one branch for the tile).
    %no_c = arith.cmpf oeq, %beta, %fz : f32
    %malpha = air.simdgroup_matrix_fill %alpha : vector<64xf32>
    %mbeta = air.simdgroup_matrix_fill %beta : vector<64xf32>
    scf.if %no_c {
      %s_00 = arith.mulf %acc#0, %malpha : vector<64xf32>
      %h_00 = arith.truncf %s_00 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_00, %C[%row, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_01 = arith.mulf %acc#1, %malpha : vector<64xf32>
      %h_01 = arith.truncf %s_01 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_01, %C[%row, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_02 = arith.mulf %acc#2, %malpha : vector<64xf32>
      %h_02 = arith.truncf %s_02 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_02, %C[%row, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_03 = arith.mulf %acc#3, %malpha : vector<64xf32>
      %h_03 = arith.truncf %s_03 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_03, %C[%row, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_10 = arith.mulf %acc#4, %malpha : vector<64xf32>
      %h_10 = arith.truncf %s_10 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_10, %C[%row1, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_11 = arith.mulf %acc#5, %malpha : vector<64xf32>
      %h_11 = arith.truncf %s_11 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_11, %C[%row1, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_12 = arith.mulf %acc#6, %malpha : vector<64xf32>
      %h_12 = arith.truncf %s_12 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_12, %C[%row1, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_13 = arith.mulf %acc#7, %malpha : vector<64xf32>
      %h_13 = arith.truncf %s_13 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_13, %C[%row1, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_20 = arith.mulf %acc#8, %malpha : vector<64xf32>
      %h_20 = arith.truncf %s_20 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_20, %C[%row2, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_21 = arith.mulf %acc#9, %malpha : vector<64xf32>
      %h_21 = arith.truncf %s_21 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_21, %C[%row2, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_22 = arith.mulf %acc#10, %malpha : vector<64xf32>
      %h_22 = arith.truncf %s_22 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_22, %C[%row2, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_23 = arith.mulf %acc#11, %malpha : vector<64xf32>
      %h_23 = arith.truncf %s_23 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_23, %C[%row2, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_30 = arith.mulf %acc#12, %malpha : vector<64xf32>
      %h_30 = arith.truncf %s_30 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_30, %C[%row3, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_31 = arith.mulf %acc#13, %malpha : vector<64xf32>
      %h_31 = arith.truncf %s_31 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_31, %C[%row3, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_32 = arith.mulf %acc#14, %malpha : vector<64xf32>
      %h_32 = arith.truncf %s_32 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_32, %C[%row3, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_33 = arith.mulf %acc#15, %malpha : vector<64xf32>
      %h_33 = arith.truncf %s_33 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_33, %C[%row3, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
    } else {
      %s_00 = arith.mulf %acc#0, %malpha : vector<64xf32>
      %oh_00 = air.simdgroup_matrix_load %C[%row, %col] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_00 = arith.extf %oh_00 : vector<64xf16> to vector<64xf32>
      %r_00 = math.fma %old_00, %mbeta, %s_00 : vector<64xf32>
      %h_00 = arith.truncf %r_00 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_00, %C[%row, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_01 = arith.mulf %acc#1, %malpha : vector<64xf32>
      %oh_01 = air.simdgroup_matrix_load %C[%row, %col1] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_01 = arith.extf %oh_01 : vector<64xf16> to vector<64xf32>
      %r_01 = math.fma %old_01, %mbeta, %s_01 : vector<64xf32>
      %h_01 = arith.truncf %r_01 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_01, %C[%row, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_02 = arith.mulf %acc#2, %malpha : vector<64xf32>
      %oh_02 = air.simdgroup_matrix_load %C[%row, %col2] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_02 = arith.extf %oh_02 : vector<64xf16> to vector<64xf32>
      %r_02 = math.fma %old_02, %mbeta, %s_02 : vector<64xf32>
      %h_02 = arith.truncf %r_02 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_02, %C[%row, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_03 = arith.mulf %acc#3, %malpha : vector<64xf32>
      %oh_03 = air.simdgroup_matrix_load %C[%row, %col3] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_03 = arith.extf %oh_03 : vector<64xf16> to vector<64xf32>
      %r_03 = math.fma %old_03, %mbeta, %s_03 : vector<64xf32>
      %h_03 = arith.truncf %r_03 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_03, %C[%row, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_10 = arith.mulf %acc#4, %malpha : vector<64xf32>
      %oh_10 = air.simdgroup_matrix_load %C[%row1, %col] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_10 = arith.extf %oh_10 : vector<64xf16> to vector<64xf32>
      %r_10 = math.fma %old_10, %mbeta, %s_10 : vector<64xf32>
      %h_10 = arith.truncf %r_10 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_10, %C[%row1, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_11 = arith.mulf %acc#5, %malpha : vector<64xf32>
      %oh_11 = air.simdgroup_matrix_load %C[%row1, %col1] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_11 = arith.extf %oh_11 : vector<64xf16> to vector<64xf32>
      %r_11 = math.fma %old_11, %mbeta, %s_11 : vector<64xf32>
      %h_11 = arith.truncf %r_11 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_11, %C[%row1, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_12 = arith.mulf %acc#6, %malpha : vector<64xf32>
      %oh_12 = air.simdgroup_matrix_load %C[%row1, %col2] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_12 = arith.extf %oh_12 : vector<64xf16> to vector<64xf32>
      %r_12 = math.fma %old_12, %mbeta, %s_12 : vector<64xf32>
      %h_12 = arith.truncf %r_12 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_12, %C[%row1, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_13 = arith.mulf %acc#7, %malpha : vector<64xf32>
      %oh_13 = air.simdgroup_matrix_load %C[%row1, %col3] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_13 = arith.extf %oh_13 : vector<64xf16> to vector<64xf32>
      %r_13 = math.fma %old_13, %mbeta, %s_13 : vector<64xf32>
      %h_13 = arith.truncf %r_13 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_13, %C[%row1, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_20 = arith.mulf %acc#8, %malpha : vector<64xf32>
      %oh_20 = air.simdgroup_matrix_load %C[%row2, %col] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_20 = arith.extf %oh_20 : vector<64xf16> to vector<64xf32>
      %r_20 = math.fma %old_20, %mbeta, %s_20 : vector<64xf32>
      %h_20 = arith.truncf %r_20 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_20, %C[%row2, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_21 = arith.mulf %acc#9, %malpha : vector<64xf32>
      %oh_21 = air.simdgroup_matrix_load %C[%row2, %col1] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_21 = arith.extf %oh_21 : vector<64xf16> to vector<64xf32>
      %r_21 = math.fma %old_21, %mbeta, %s_21 : vector<64xf32>
      %h_21 = arith.truncf %r_21 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_21, %C[%row2, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_22 = arith.mulf %acc#10, %malpha : vector<64xf32>
      %oh_22 = air.simdgroup_matrix_load %C[%row2, %col2] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_22 = arith.extf %oh_22 : vector<64xf16> to vector<64xf32>
      %r_22 = math.fma %old_22, %mbeta, %s_22 : vector<64xf32>
      %h_22 = arith.truncf %r_22 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_22, %C[%row2, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_23 = arith.mulf %acc#11, %malpha : vector<64xf32>
      %oh_23 = air.simdgroup_matrix_load %C[%row2, %col3] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_23 = arith.extf %oh_23 : vector<64xf16> to vector<64xf32>
      %r_23 = math.fma %old_23, %mbeta, %s_23 : vector<64xf32>
      %h_23 = arith.truncf %r_23 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_23, %C[%row2, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_30 = arith.mulf %acc#12, %malpha : vector<64xf32>
      %oh_30 = air.simdgroup_matrix_load %C[%row3, %col] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_30 = arith.extf %oh_30 : vector<64xf16> to vector<64xf32>
      %r_30 = math.fma %old_30, %mbeta, %s_30 : vector<64xf32>
      %h_30 = arith.truncf %r_30 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_30, %C[%row3, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_31 = arith.mulf %acc#13, %malpha : vector<64xf32>
      %oh_31 = air.simdgroup_matrix_load %C[%row3, %col1] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_31 = arith.extf %oh_31 : vector<64xf16> to vector<64xf32>
      %r_31 = math.fma %old_31, %mbeta, %s_31 : vector<64xf32>
      %h_31 = arith.truncf %r_31 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_31, %C[%row3, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_32 = arith.mulf %acc#14, %malpha : vector<64xf32>
      %oh_32 = air.simdgroup_matrix_load %C[%row3, %col2] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_32 = arith.extf %oh_32 : vector<64xf16> to vector<64xf32>
      %r_32 = math.fma %old_32, %mbeta, %s_32 : vector<64xf32>
      %h_32 = arith.truncf %r_32 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_32, %C[%row3, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_33 = arith.mulf %acc#15, %malpha : vector<64xf32>
      %oh_33 = air.simdgroup_matrix_load %C[%row3, %col3] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_33 = arith.extf %oh_33 : vector<64xf16> to vector<64xf32>
      %r_33 = math.fma %old_33, %mbeta, %s_33 : vector<64xf32>
      %h_33 = arith.truncf %r_33 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_33, %C[%row3, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
    }
    gpu.return
  }

  // agx_hgemm_tile64_cf16 with B[k, c] at b[c * ldb + k] (transposed).
  gpu.func @agx_hgemm_tile64_cf16_tb(%a0: memref<1xf16, 1>, %b0: memref<1xf16, 1>,
      %c0: memref<1xf16, 1>, %M: index, %N: index, %K: index, %lda: index,
      %ldb: index, %ldc: index, %bsa: index, %bsb: index, %bsc: index, %alpha: f32,
      %beta: f32) kernel
      attributes {known_block_size = array<i32: 128, 1, 1>} {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %i2 = arith.constant 2 : index
    %i8 = arith.constant 8 : index
    %i16 = arith.constant 16 : index
    %i24 = arith.constant 24 : index
    %i32 = arith.constant 32 : index
    %i64 = arith.constant 64 : index
    %i128 = arith.constant 128 : index
    %even = arith.constant -2 : index
    %a0a = memref.assume_alignment %a0, 8 : memref<1xf16, 1>
    %b0a = memref.assume_alignment %b0, 8 : memref<1xf16, 1>
    %c0a = memref.assume_alignment %c0, 8 : memref<1xf16, 1>
    %lda_e = arith.andi %lda, %even : index
    %ldb_e = arith.andi %ldb, %even : index
    %ldc_e = arith.andi %ldc, %even : index
    %z = gpu.block_id y
    %bsa_e = arith.andi %bsa, %even : index
    %oa = arith.muli %z, %bsa_e : index
    %bsb_e = arith.andi %bsb, %even : index
    %ob = arith.muli %z, %bsb_e : index
    %bsc_e = arith.andi %bsc, %even : index
    %oc = arith.muli %z, %bsc_e : index
    %A = memref.reinterpret_cast %a0a to offset: [%oa], sizes: [%M, %K], strides: [%lda_e, 1]
        : memref<1xf16, 1> to memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
    %B = memref.reinterpret_cast %b0a to offset: [%ob], sizes: [%N, %K], strides: [%ldb_e, 1]
        : memref<1xf16, 1> to memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
    %C = memref.reinterpret_cast %c0a to offset: [%oc], sizes: [%M, %N], strides: [%ldc_e, 1]
        : memref<1xf16, 1> to memref<?x?xf16, strided<[?, 1], offset: ?>, 1>

    // Grouped tile order: groups of 8 tile rows, column-major inside a group.
    %tiles_m = arith.divui %M, %i64 : index
    %tiles_n = arith.divui %N, %i64 : index
    %id = gpu.block_id x
    %per_group = arith.muli %tiles_n, %i8 : index
    %group = arith.divui %id, %per_group : index
    %first_m = arith.muli %group, %i8 : index
    %left_m = arith.subi %tiles_m, %first_m : index
    %group_m = arith.minui %left_m, %i8 : index
    %in_group = arith.remui %id, %per_group : index
    %rem_m = arith.remui %in_group, %group_m : index
    %tm = arith.addi %first_m, %rem_m : index
    %tn = arith.divui %in_group, %group_m : index

    // This SIMD-group's 32 x 32 tile: 4 x 4 matrices.
    %sg = gpu.subgroup_id : index
    %sr = arith.divui %sg, %i2 : index
    %sc = arith.remui %sg, %i2 : index
    %tm64 = arith.muli %tm, %i64 : index
    %tn64 = arith.muli %tn, %i64 : index
    %sr32 = arith.muli %sr, %i32 : index
    %sc32 = arith.muli %sc, %i32 : index
    %row = arith.addi %tm64, %sr32 : index
    %col = arith.addi %tn64, %sc32 : index
    %row1 = arith.addi %row, %i8 : index
    %col1 = arith.addi %col, %i8 : index
    %row2 = arith.addi %row, %i16 : index
    %col2 = arith.addi %col, %i16 : index
    %row3 = arith.addi %row, %i24 : index
    %col3 = arith.addi %col, %i24 : index
    %fz = arith.constant 0.0 : f32
    %zero = air.simdgroup_matrix_fill %fz : vector<64xf32>
    %acc:16 = scf.for %kk = %i0 to %K step %i128
        iter_args(%o00 = %zero, %o01 = %zero, %o02 = %zero, %o03 = %zero, %o10 = %zero, %o11 = %zero, %o12 = %zero, %o13 = %zero, %o20 = %zero, %o21 = %zero, %o22 = %zero, %o23 = %zero, %o30 = %zero, %o31 = %zero, %o32 = %zero, %o33 = %zero) -> (vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>) {
      gpu.barrier memfence []
      %kk_end0 = arith.addi %kk, %i128 : index
      %kk_end = arith.minui %kk_end0, %K : index
      %in:16 = scf.for %k = %kk to %kk_end step %i8
          iter_args(%a00 = %o00, %a01 = %o01, %a02 = %o02, %a03 = %o03, %a10 = %o10, %a11 = %o11, %a12 = %o12, %a13 = %o13, %a20 = %o20, %a21 = %o21, %a22 = %o22, %a23 = %o23, %a30 = %o30, %a31 = %o31, %a32 = %o32, %a33 = %o33) -> (vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>) {
        %ma0 = air.simdgroup_matrix_load %A[%row, %k] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %ma1 = air.simdgroup_matrix_load %A[%row1, %k] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %ma2 = air.simdgroup_matrix_load %A[%row2, %k] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %ma3 = air.simdgroup_matrix_load %A[%row3, %k] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb0 = air.simdgroup_matrix_load %B[%col, %k] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb1 = air.simdgroup_matrix_load %B[%col1, %k] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb2 = air.simdgroup_matrix_load %B[%col2, %k] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb3 = air.simdgroup_matrix_load %B[%col3, %k] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %n00 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb0, %a00 : vector<64xf16>, vector<64xf32>
        %n01 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb1, %a01 : vector<64xf16>, vector<64xf32>
        %n02 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb2, %a02 : vector<64xf16>, vector<64xf32>
        %n03 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb3, %a03 : vector<64xf16>, vector<64xf32>
        %n10 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb0, %a10 : vector<64xf16>, vector<64xf32>
        %n11 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb1, %a11 : vector<64xf16>, vector<64xf32>
        %n12 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb2, %a12 : vector<64xf16>, vector<64xf32>
        %n13 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb3, %a13 : vector<64xf16>, vector<64xf32>
        %n20 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb0, %a20 : vector<64xf16>, vector<64xf32>
        %n21 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb1, %a21 : vector<64xf16>, vector<64xf32>
        %n22 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb2, %a22 : vector<64xf16>, vector<64xf32>
        %n23 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb3, %a23 : vector<64xf16>, vector<64xf32>
        %n30 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb0, %a30 : vector<64xf16>, vector<64xf32>
        %n31 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb1, %a31 : vector<64xf16>, vector<64xf32>
        %n32 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb2, %a32 : vector<64xf16>, vector<64xf32>
        %n33 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb3, %a33 : vector<64xf16>, vector<64xf32>
        scf.yield %n00, %n01, %n02, %n03, %n10, %n11, %n12, %n13, %n20, %n21, %n22, %n23, %n30, %n31, %n32, %n33 : vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>
      }
      scf.yield %in#0, %in#1, %in#2, %in#3, %in#4, %in#5, %in#6, %in#7, %in#8, %in#9, %in#10, %in#11, %in#12, %in#13, %in#14, %in#15 : vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>
    }

    // C = alpha * acc + beta * C, C not read when beta is 0 (one branch for the tile).
    %no_c = arith.cmpf oeq, %beta, %fz : f32
    %malpha = air.simdgroup_matrix_fill %alpha : vector<64xf32>
    %mbeta = air.simdgroup_matrix_fill %beta : vector<64xf32>
    scf.if %no_c {
      %s_00 = arith.mulf %acc#0, %malpha : vector<64xf32>
      %h_00 = arith.truncf %s_00 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_00, %C[%row, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_01 = arith.mulf %acc#1, %malpha : vector<64xf32>
      %h_01 = arith.truncf %s_01 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_01, %C[%row, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_02 = arith.mulf %acc#2, %malpha : vector<64xf32>
      %h_02 = arith.truncf %s_02 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_02, %C[%row, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_03 = arith.mulf %acc#3, %malpha : vector<64xf32>
      %h_03 = arith.truncf %s_03 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_03, %C[%row, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_10 = arith.mulf %acc#4, %malpha : vector<64xf32>
      %h_10 = arith.truncf %s_10 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_10, %C[%row1, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_11 = arith.mulf %acc#5, %malpha : vector<64xf32>
      %h_11 = arith.truncf %s_11 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_11, %C[%row1, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_12 = arith.mulf %acc#6, %malpha : vector<64xf32>
      %h_12 = arith.truncf %s_12 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_12, %C[%row1, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_13 = arith.mulf %acc#7, %malpha : vector<64xf32>
      %h_13 = arith.truncf %s_13 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_13, %C[%row1, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_20 = arith.mulf %acc#8, %malpha : vector<64xf32>
      %h_20 = arith.truncf %s_20 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_20, %C[%row2, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_21 = arith.mulf %acc#9, %malpha : vector<64xf32>
      %h_21 = arith.truncf %s_21 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_21, %C[%row2, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_22 = arith.mulf %acc#10, %malpha : vector<64xf32>
      %h_22 = arith.truncf %s_22 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_22, %C[%row2, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_23 = arith.mulf %acc#11, %malpha : vector<64xf32>
      %h_23 = arith.truncf %s_23 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_23, %C[%row2, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_30 = arith.mulf %acc#12, %malpha : vector<64xf32>
      %h_30 = arith.truncf %s_30 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_30, %C[%row3, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_31 = arith.mulf %acc#13, %malpha : vector<64xf32>
      %h_31 = arith.truncf %s_31 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_31, %C[%row3, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_32 = arith.mulf %acc#14, %malpha : vector<64xf32>
      %h_32 = arith.truncf %s_32 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_32, %C[%row3, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_33 = arith.mulf %acc#15, %malpha : vector<64xf32>
      %h_33 = arith.truncf %s_33 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_33, %C[%row3, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
    } else {
      %s_00 = arith.mulf %acc#0, %malpha : vector<64xf32>
      %oh_00 = air.simdgroup_matrix_load %C[%row, %col] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_00 = arith.extf %oh_00 : vector<64xf16> to vector<64xf32>
      %r_00 = math.fma %old_00, %mbeta, %s_00 : vector<64xf32>
      %h_00 = arith.truncf %r_00 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_00, %C[%row, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_01 = arith.mulf %acc#1, %malpha : vector<64xf32>
      %oh_01 = air.simdgroup_matrix_load %C[%row, %col1] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_01 = arith.extf %oh_01 : vector<64xf16> to vector<64xf32>
      %r_01 = math.fma %old_01, %mbeta, %s_01 : vector<64xf32>
      %h_01 = arith.truncf %r_01 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_01, %C[%row, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_02 = arith.mulf %acc#2, %malpha : vector<64xf32>
      %oh_02 = air.simdgroup_matrix_load %C[%row, %col2] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_02 = arith.extf %oh_02 : vector<64xf16> to vector<64xf32>
      %r_02 = math.fma %old_02, %mbeta, %s_02 : vector<64xf32>
      %h_02 = arith.truncf %r_02 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_02, %C[%row, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_03 = arith.mulf %acc#3, %malpha : vector<64xf32>
      %oh_03 = air.simdgroup_matrix_load %C[%row, %col3] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_03 = arith.extf %oh_03 : vector<64xf16> to vector<64xf32>
      %r_03 = math.fma %old_03, %mbeta, %s_03 : vector<64xf32>
      %h_03 = arith.truncf %r_03 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_03, %C[%row, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_10 = arith.mulf %acc#4, %malpha : vector<64xf32>
      %oh_10 = air.simdgroup_matrix_load %C[%row1, %col] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_10 = arith.extf %oh_10 : vector<64xf16> to vector<64xf32>
      %r_10 = math.fma %old_10, %mbeta, %s_10 : vector<64xf32>
      %h_10 = arith.truncf %r_10 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_10, %C[%row1, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_11 = arith.mulf %acc#5, %malpha : vector<64xf32>
      %oh_11 = air.simdgroup_matrix_load %C[%row1, %col1] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_11 = arith.extf %oh_11 : vector<64xf16> to vector<64xf32>
      %r_11 = math.fma %old_11, %mbeta, %s_11 : vector<64xf32>
      %h_11 = arith.truncf %r_11 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_11, %C[%row1, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_12 = arith.mulf %acc#6, %malpha : vector<64xf32>
      %oh_12 = air.simdgroup_matrix_load %C[%row1, %col2] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_12 = arith.extf %oh_12 : vector<64xf16> to vector<64xf32>
      %r_12 = math.fma %old_12, %mbeta, %s_12 : vector<64xf32>
      %h_12 = arith.truncf %r_12 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_12, %C[%row1, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_13 = arith.mulf %acc#7, %malpha : vector<64xf32>
      %oh_13 = air.simdgroup_matrix_load %C[%row1, %col3] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_13 = arith.extf %oh_13 : vector<64xf16> to vector<64xf32>
      %r_13 = math.fma %old_13, %mbeta, %s_13 : vector<64xf32>
      %h_13 = arith.truncf %r_13 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_13, %C[%row1, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_20 = arith.mulf %acc#8, %malpha : vector<64xf32>
      %oh_20 = air.simdgroup_matrix_load %C[%row2, %col] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_20 = arith.extf %oh_20 : vector<64xf16> to vector<64xf32>
      %r_20 = math.fma %old_20, %mbeta, %s_20 : vector<64xf32>
      %h_20 = arith.truncf %r_20 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_20, %C[%row2, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_21 = arith.mulf %acc#9, %malpha : vector<64xf32>
      %oh_21 = air.simdgroup_matrix_load %C[%row2, %col1] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_21 = arith.extf %oh_21 : vector<64xf16> to vector<64xf32>
      %r_21 = math.fma %old_21, %mbeta, %s_21 : vector<64xf32>
      %h_21 = arith.truncf %r_21 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_21, %C[%row2, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_22 = arith.mulf %acc#10, %malpha : vector<64xf32>
      %oh_22 = air.simdgroup_matrix_load %C[%row2, %col2] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_22 = arith.extf %oh_22 : vector<64xf16> to vector<64xf32>
      %r_22 = math.fma %old_22, %mbeta, %s_22 : vector<64xf32>
      %h_22 = arith.truncf %r_22 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_22, %C[%row2, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_23 = arith.mulf %acc#11, %malpha : vector<64xf32>
      %oh_23 = air.simdgroup_matrix_load %C[%row2, %col3] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_23 = arith.extf %oh_23 : vector<64xf16> to vector<64xf32>
      %r_23 = math.fma %old_23, %mbeta, %s_23 : vector<64xf32>
      %h_23 = arith.truncf %r_23 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_23, %C[%row2, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_30 = arith.mulf %acc#12, %malpha : vector<64xf32>
      %oh_30 = air.simdgroup_matrix_load %C[%row3, %col] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_30 = arith.extf %oh_30 : vector<64xf16> to vector<64xf32>
      %r_30 = math.fma %old_30, %mbeta, %s_30 : vector<64xf32>
      %h_30 = arith.truncf %r_30 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_30, %C[%row3, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_31 = arith.mulf %acc#13, %malpha : vector<64xf32>
      %oh_31 = air.simdgroup_matrix_load %C[%row3, %col1] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_31 = arith.extf %oh_31 : vector<64xf16> to vector<64xf32>
      %r_31 = math.fma %old_31, %mbeta, %s_31 : vector<64xf32>
      %h_31 = arith.truncf %r_31 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_31, %C[%row3, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_32 = arith.mulf %acc#14, %malpha : vector<64xf32>
      %oh_32 = air.simdgroup_matrix_load %C[%row3, %col2] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_32 = arith.extf %oh_32 : vector<64xf16> to vector<64xf32>
      %r_32 = math.fma %old_32, %mbeta, %s_32 : vector<64xf32>
      %h_32 = arith.truncf %r_32 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_32, %C[%row3, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_33 = arith.mulf %acc#15, %malpha : vector<64xf32>
      %oh_33 = air.simdgroup_matrix_load %C[%row3, %col3] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_33 = arith.extf %oh_33 : vector<64xf16> to vector<64xf32>
      %r_33 = math.fma %old_33, %mbeta, %s_33 : vector<64xf32>
      %h_33 = arith.truncf %r_33 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_33, %C[%row3, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
    }
    gpu.return
  }

  // agx_hgemm_tile64_cf16 with A[r, k] at a[k * lda + r] and B[k, c] at b[c * ldb + k] (transposed).
  gpu.func @agx_hgemm_tile64_cf16_tatb(%a0: memref<1xf16, 1>, %b0: memref<1xf16, 1>,
      %c0: memref<1xf16, 1>, %M: index, %N: index, %K: index, %lda: index,
      %ldb: index, %ldc: index, %bsa: index, %bsb: index, %bsc: index, %alpha: f32,
      %beta: f32) kernel
      attributes {known_block_size = array<i32: 128, 1, 1>} {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %i2 = arith.constant 2 : index
    %i8 = arith.constant 8 : index
    %i16 = arith.constant 16 : index
    %i24 = arith.constant 24 : index
    %i32 = arith.constant 32 : index
    %i64 = arith.constant 64 : index
    %i128 = arith.constant 128 : index
    %even = arith.constant -2 : index
    %a0a = memref.assume_alignment %a0, 8 : memref<1xf16, 1>
    %b0a = memref.assume_alignment %b0, 8 : memref<1xf16, 1>
    %c0a = memref.assume_alignment %c0, 8 : memref<1xf16, 1>
    %lda_e = arith.andi %lda, %even : index
    %ldb_e = arith.andi %ldb, %even : index
    %ldc_e = arith.andi %ldc, %even : index
    %z = gpu.block_id y
    %bsa_e = arith.andi %bsa, %even : index
    %oa = arith.muli %z, %bsa_e : index
    %bsb_e = arith.andi %bsb, %even : index
    %ob = arith.muli %z, %bsb_e : index
    %bsc_e = arith.andi %bsc, %even : index
    %oc = arith.muli %z, %bsc_e : index
    %A = memref.reinterpret_cast %a0a to offset: [%oa], sizes: [%K, %M], strides: [%lda_e, 1]
        : memref<1xf16, 1> to memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
    %B = memref.reinterpret_cast %b0a to offset: [%ob], sizes: [%N, %K], strides: [%ldb_e, 1]
        : memref<1xf16, 1> to memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
    %C = memref.reinterpret_cast %c0a to offset: [%oc], sizes: [%M, %N], strides: [%ldc_e, 1]
        : memref<1xf16, 1> to memref<?x?xf16, strided<[?, 1], offset: ?>, 1>

    // Grouped tile order: groups of 8 tile rows, column-major inside a group.
    %tiles_m = arith.divui %M, %i64 : index
    %tiles_n = arith.divui %N, %i64 : index
    %id = gpu.block_id x
    %per_group = arith.muli %tiles_n, %i8 : index
    %group = arith.divui %id, %per_group : index
    %first_m = arith.muli %group, %i8 : index
    %left_m = arith.subi %tiles_m, %first_m : index
    %group_m = arith.minui %left_m, %i8 : index
    %in_group = arith.remui %id, %per_group : index
    %rem_m = arith.remui %in_group, %group_m : index
    %tm = arith.addi %first_m, %rem_m : index
    %tn = arith.divui %in_group, %group_m : index

    // This SIMD-group's 32 x 32 tile: 4 x 4 matrices.
    %sg = gpu.subgroup_id : index
    %sr = arith.divui %sg, %i2 : index
    %sc = arith.remui %sg, %i2 : index
    %tm64 = arith.muli %tm, %i64 : index
    %tn64 = arith.muli %tn, %i64 : index
    %sr32 = arith.muli %sr, %i32 : index
    %sc32 = arith.muli %sc, %i32 : index
    %row = arith.addi %tm64, %sr32 : index
    %col = arith.addi %tn64, %sc32 : index
    %row1 = arith.addi %row, %i8 : index
    %col1 = arith.addi %col, %i8 : index
    %row2 = arith.addi %row, %i16 : index
    %col2 = arith.addi %col, %i16 : index
    %row3 = arith.addi %row, %i24 : index
    %col3 = arith.addi %col, %i24 : index
    %fz = arith.constant 0.0 : f32
    %zero = air.simdgroup_matrix_fill %fz : vector<64xf32>
    %acc:16 = scf.for %kk = %i0 to %K step %i128
        iter_args(%o00 = %zero, %o01 = %zero, %o02 = %zero, %o03 = %zero, %o10 = %zero, %o11 = %zero, %o12 = %zero, %o13 = %zero, %o20 = %zero, %o21 = %zero, %o22 = %zero, %o23 = %zero, %o30 = %zero, %o31 = %zero, %o32 = %zero, %o33 = %zero) -> (vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>) {
      gpu.barrier memfence []
      %kk_end0 = arith.addi %kk, %i128 : index
      %kk_end = arith.minui %kk_end0, %K : index
      %in:16 = scf.for %k = %kk to %kk_end step %i8
          iter_args(%a00 = %o00, %a01 = %o01, %a02 = %o02, %a03 = %o03, %a10 = %o10, %a11 = %o11, %a12 = %o12, %a13 = %o13, %a20 = %o20, %a21 = %o21, %a22 = %o22, %a23 = %o23, %a30 = %o30, %a31 = %o31, %a32 = %o32, %a33 = %o33) -> (vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>) {
        %ma0 = air.simdgroup_matrix_load %A[%k, %row] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %ma1 = air.simdgroup_matrix_load %A[%k, %row1] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %ma2 = air.simdgroup_matrix_load %A[%k, %row2] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %ma3 = air.simdgroup_matrix_load %A[%k, %row3] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb0 = air.simdgroup_matrix_load %B[%col, %k] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb1 = air.simdgroup_matrix_load %B[%col1, %k] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb2 = air.simdgroup_matrix_load %B[%col2, %k] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb3 = air.simdgroup_matrix_load %B[%col3, %k] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %n00 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb0, %a00 : vector<64xf16>, vector<64xf32>
        %n01 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb1, %a01 : vector<64xf16>, vector<64xf32>
        %n02 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb2, %a02 : vector<64xf16>, vector<64xf32>
        %n03 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb3, %a03 : vector<64xf16>, vector<64xf32>
        %n10 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb0, %a10 : vector<64xf16>, vector<64xf32>
        %n11 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb1, %a11 : vector<64xf16>, vector<64xf32>
        %n12 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb2, %a12 : vector<64xf16>, vector<64xf32>
        %n13 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb3, %a13 : vector<64xf16>, vector<64xf32>
        %n20 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb0, %a20 : vector<64xf16>, vector<64xf32>
        %n21 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb1, %a21 : vector<64xf16>, vector<64xf32>
        %n22 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb2, %a22 : vector<64xf16>, vector<64xf32>
        %n23 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb3, %a23 : vector<64xf16>, vector<64xf32>
        %n30 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb0, %a30 : vector<64xf16>, vector<64xf32>
        %n31 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb1, %a31 : vector<64xf16>, vector<64xf32>
        %n32 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb2, %a32 : vector<64xf16>, vector<64xf32>
        %n33 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb3, %a33 : vector<64xf16>, vector<64xf32>
        scf.yield %n00, %n01, %n02, %n03, %n10, %n11, %n12, %n13, %n20, %n21, %n22, %n23, %n30, %n31, %n32, %n33 : vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>
      }
      scf.yield %in#0, %in#1, %in#2, %in#3, %in#4, %in#5, %in#6, %in#7, %in#8, %in#9, %in#10, %in#11, %in#12, %in#13, %in#14, %in#15 : vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>
    }

    // C = alpha * acc + beta * C, C not read when beta is 0 (one branch for the tile).
    %no_c = arith.cmpf oeq, %beta, %fz : f32
    %malpha = air.simdgroup_matrix_fill %alpha : vector<64xf32>
    %mbeta = air.simdgroup_matrix_fill %beta : vector<64xf32>
    scf.if %no_c {
      %s_00 = arith.mulf %acc#0, %malpha : vector<64xf32>
      %h_00 = arith.truncf %s_00 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_00, %C[%row, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_01 = arith.mulf %acc#1, %malpha : vector<64xf32>
      %h_01 = arith.truncf %s_01 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_01, %C[%row, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_02 = arith.mulf %acc#2, %malpha : vector<64xf32>
      %h_02 = arith.truncf %s_02 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_02, %C[%row, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_03 = arith.mulf %acc#3, %malpha : vector<64xf32>
      %h_03 = arith.truncf %s_03 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_03, %C[%row, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_10 = arith.mulf %acc#4, %malpha : vector<64xf32>
      %h_10 = arith.truncf %s_10 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_10, %C[%row1, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_11 = arith.mulf %acc#5, %malpha : vector<64xf32>
      %h_11 = arith.truncf %s_11 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_11, %C[%row1, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_12 = arith.mulf %acc#6, %malpha : vector<64xf32>
      %h_12 = arith.truncf %s_12 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_12, %C[%row1, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_13 = arith.mulf %acc#7, %malpha : vector<64xf32>
      %h_13 = arith.truncf %s_13 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_13, %C[%row1, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_20 = arith.mulf %acc#8, %malpha : vector<64xf32>
      %h_20 = arith.truncf %s_20 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_20, %C[%row2, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_21 = arith.mulf %acc#9, %malpha : vector<64xf32>
      %h_21 = arith.truncf %s_21 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_21, %C[%row2, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_22 = arith.mulf %acc#10, %malpha : vector<64xf32>
      %h_22 = arith.truncf %s_22 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_22, %C[%row2, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_23 = arith.mulf %acc#11, %malpha : vector<64xf32>
      %h_23 = arith.truncf %s_23 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_23, %C[%row2, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_30 = arith.mulf %acc#12, %malpha : vector<64xf32>
      %h_30 = arith.truncf %s_30 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_30, %C[%row3, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_31 = arith.mulf %acc#13, %malpha : vector<64xf32>
      %h_31 = arith.truncf %s_31 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_31, %C[%row3, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_32 = arith.mulf %acc#14, %malpha : vector<64xf32>
      %h_32 = arith.truncf %s_32 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_32, %C[%row3, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_33 = arith.mulf %acc#15, %malpha : vector<64xf32>
      %h_33 = arith.truncf %s_33 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_33, %C[%row3, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
    } else {
      %s_00 = arith.mulf %acc#0, %malpha : vector<64xf32>
      %oh_00 = air.simdgroup_matrix_load %C[%row, %col] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_00 = arith.extf %oh_00 : vector<64xf16> to vector<64xf32>
      %r_00 = math.fma %old_00, %mbeta, %s_00 : vector<64xf32>
      %h_00 = arith.truncf %r_00 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_00, %C[%row, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_01 = arith.mulf %acc#1, %malpha : vector<64xf32>
      %oh_01 = air.simdgroup_matrix_load %C[%row, %col1] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_01 = arith.extf %oh_01 : vector<64xf16> to vector<64xf32>
      %r_01 = math.fma %old_01, %mbeta, %s_01 : vector<64xf32>
      %h_01 = arith.truncf %r_01 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_01, %C[%row, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_02 = arith.mulf %acc#2, %malpha : vector<64xf32>
      %oh_02 = air.simdgroup_matrix_load %C[%row, %col2] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_02 = arith.extf %oh_02 : vector<64xf16> to vector<64xf32>
      %r_02 = math.fma %old_02, %mbeta, %s_02 : vector<64xf32>
      %h_02 = arith.truncf %r_02 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_02, %C[%row, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_03 = arith.mulf %acc#3, %malpha : vector<64xf32>
      %oh_03 = air.simdgroup_matrix_load %C[%row, %col3] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_03 = arith.extf %oh_03 : vector<64xf16> to vector<64xf32>
      %r_03 = math.fma %old_03, %mbeta, %s_03 : vector<64xf32>
      %h_03 = arith.truncf %r_03 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_03, %C[%row, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_10 = arith.mulf %acc#4, %malpha : vector<64xf32>
      %oh_10 = air.simdgroup_matrix_load %C[%row1, %col] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_10 = arith.extf %oh_10 : vector<64xf16> to vector<64xf32>
      %r_10 = math.fma %old_10, %mbeta, %s_10 : vector<64xf32>
      %h_10 = arith.truncf %r_10 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_10, %C[%row1, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_11 = arith.mulf %acc#5, %malpha : vector<64xf32>
      %oh_11 = air.simdgroup_matrix_load %C[%row1, %col1] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_11 = arith.extf %oh_11 : vector<64xf16> to vector<64xf32>
      %r_11 = math.fma %old_11, %mbeta, %s_11 : vector<64xf32>
      %h_11 = arith.truncf %r_11 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_11, %C[%row1, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_12 = arith.mulf %acc#6, %malpha : vector<64xf32>
      %oh_12 = air.simdgroup_matrix_load %C[%row1, %col2] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_12 = arith.extf %oh_12 : vector<64xf16> to vector<64xf32>
      %r_12 = math.fma %old_12, %mbeta, %s_12 : vector<64xf32>
      %h_12 = arith.truncf %r_12 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_12, %C[%row1, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_13 = arith.mulf %acc#7, %malpha : vector<64xf32>
      %oh_13 = air.simdgroup_matrix_load %C[%row1, %col3] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_13 = arith.extf %oh_13 : vector<64xf16> to vector<64xf32>
      %r_13 = math.fma %old_13, %mbeta, %s_13 : vector<64xf32>
      %h_13 = arith.truncf %r_13 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_13, %C[%row1, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_20 = arith.mulf %acc#8, %malpha : vector<64xf32>
      %oh_20 = air.simdgroup_matrix_load %C[%row2, %col] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_20 = arith.extf %oh_20 : vector<64xf16> to vector<64xf32>
      %r_20 = math.fma %old_20, %mbeta, %s_20 : vector<64xf32>
      %h_20 = arith.truncf %r_20 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_20, %C[%row2, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_21 = arith.mulf %acc#9, %malpha : vector<64xf32>
      %oh_21 = air.simdgroup_matrix_load %C[%row2, %col1] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_21 = arith.extf %oh_21 : vector<64xf16> to vector<64xf32>
      %r_21 = math.fma %old_21, %mbeta, %s_21 : vector<64xf32>
      %h_21 = arith.truncf %r_21 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_21, %C[%row2, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_22 = arith.mulf %acc#10, %malpha : vector<64xf32>
      %oh_22 = air.simdgroup_matrix_load %C[%row2, %col2] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_22 = arith.extf %oh_22 : vector<64xf16> to vector<64xf32>
      %r_22 = math.fma %old_22, %mbeta, %s_22 : vector<64xf32>
      %h_22 = arith.truncf %r_22 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_22, %C[%row2, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_23 = arith.mulf %acc#11, %malpha : vector<64xf32>
      %oh_23 = air.simdgroup_matrix_load %C[%row2, %col3] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_23 = arith.extf %oh_23 : vector<64xf16> to vector<64xf32>
      %r_23 = math.fma %old_23, %mbeta, %s_23 : vector<64xf32>
      %h_23 = arith.truncf %r_23 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_23, %C[%row2, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_30 = arith.mulf %acc#12, %malpha : vector<64xf32>
      %oh_30 = air.simdgroup_matrix_load %C[%row3, %col] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_30 = arith.extf %oh_30 : vector<64xf16> to vector<64xf32>
      %r_30 = math.fma %old_30, %mbeta, %s_30 : vector<64xf32>
      %h_30 = arith.truncf %r_30 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_30, %C[%row3, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_31 = arith.mulf %acc#13, %malpha : vector<64xf32>
      %oh_31 = air.simdgroup_matrix_load %C[%row3, %col1] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_31 = arith.extf %oh_31 : vector<64xf16> to vector<64xf32>
      %r_31 = math.fma %old_31, %mbeta, %s_31 : vector<64xf32>
      %h_31 = arith.truncf %r_31 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_31, %C[%row3, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_32 = arith.mulf %acc#14, %malpha : vector<64xf32>
      %oh_32 = air.simdgroup_matrix_load %C[%row3, %col2] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_32 = arith.extf %oh_32 : vector<64xf16> to vector<64xf32>
      %r_32 = math.fma %old_32, %mbeta, %s_32 : vector<64xf32>
      %h_32 = arith.truncf %r_32 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_32, %C[%row3, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
      %s_33 = arith.mulf %acc#15, %malpha : vector<64xf32>
      %oh_33 = air.simdgroup_matrix_load %C[%row3, %col3] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
      %old_33 = arith.extf %oh_33 : vector<64xf16> to vector<64xf32>
      %r_33 = math.fma %old_33, %mbeta, %s_33 : vector<64xf32>
      %h_33 = arith.truncf %r_33 : vector<64xf32> to vector<64xf16>
      air.simdgroup_matrix_store %h_33, %C[%row3, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
    }
    gpu.return
  }

  // agx_hgemm_tile64_cf32 with A[r, k] at a[k * lda + r] (transposed).
  gpu.func @agx_hgemm_tile64_cf32_ta(%a0: memref<1xf16, 1>, %b0: memref<1xf16, 1>,
      %c0: memref<1xf32, 1>, %M: index, %N: index, %K: index, %lda: index,
      %ldb: index, %ldc: index, %bsa: index, %bsb: index, %bsc: index, %alpha: f32,
      %beta: f32) kernel
      attributes {known_block_size = array<i32: 128, 1, 1>} {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %i2 = arith.constant 2 : index
    %i8 = arith.constant 8 : index
    %i16 = arith.constant 16 : index
    %i24 = arith.constant 24 : index
    %i32 = arith.constant 32 : index
    %i64 = arith.constant 64 : index
    %i128 = arith.constant 128 : index
    %even = arith.constant -2 : index
    %a0a = memref.assume_alignment %a0, 8 : memref<1xf16, 1>
    %b0a = memref.assume_alignment %b0, 8 : memref<1xf16, 1>
    %c0a = memref.assume_alignment %c0, 8 : memref<1xf32, 1>
    %lda_e = arith.andi %lda, %even : index
    %ldb_e = arith.andi %ldb, %even : index
    %ldc_e = arith.andi %ldc, %even : index
    %z = gpu.block_id y
    %bsa_e = arith.andi %bsa, %even : index
    %oa = arith.muli %z, %bsa_e : index
    %bsb_e = arith.andi %bsb, %even : index
    %ob = arith.muli %z, %bsb_e : index
    %bsc_e = arith.andi %bsc, %even : index
    %oc = arith.muli %z, %bsc_e : index
    %A = memref.reinterpret_cast %a0a to offset: [%oa], sizes: [%K, %M], strides: [%lda_e, 1]
        : memref<1xf16, 1> to memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
    %B = memref.reinterpret_cast %b0a to offset: [%ob], sizes: [%K, %N], strides: [%ldb_e, 1]
        : memref<1xf16, 1> to memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
    %C = memref.reinterpret_cast %c0a to offset: [%oc], sizes: [%M, %N], strides: [%ldc_e, 1]
        : memref<1xf32, 1> to memref<?x?xf32, strided<[?, 1], offset: ?>, 1>

    // Grouped tile order: groups of 8 tile rows, column-major inside a group.
    %tiles_m = arith.divui %M, %i64 : index
    %tiles_n = arith.divui %N, %i64 : index
    %id = gpu.block_id x
    %per_group = arith.muli %tiles_n, %i8 : index
    %group = arith.divui %id, %per_group : index
    %first_m = arith.muli %group, %i8 : index
    %left_m = arith.subi %tiles_m, %first_m : index
    %group_m = arith.minui %left_m, %i8 : index
    %in_group = arith.remui %id, %per_group : index
    %rem_m = arith.remui %in_group, %group_m : index
    %tm = arith.addi %first_m, %rem_m : index
    %tn = arith.divui %in_group, %group_m : index

    // This SIMD-group's 32 x 32 tile: 4 x 4 matrices.
    %sg = gpu.subgroup_id : index
    %sr = arith.divui %sg, %i2 : index
    %sc = arith.remui %sg, %i2 : index
    %tm64 = arith.muli %tm, %i64 : index
    %tn64 = arith.muli %tn, %i64 : index
    %sr32 = arith.muli %sr, %i32 : index
    %sc32 = arith.muli %sc, %i32 : index
    %row = arith.addi %tm64, %sr32 : index
    %col = arith.addi %tn64, %sc32 : index
    %row1 = arith.addi %row, %i8 : index
    %col1 = arith.addi %col, %i8 : index
    %row2 = arith.addi %row, %i16 : index
    %col2 = arith.addi %col, %i16 : index
    %row3 = arith.addi %row, %i24 : index
    %col3 = arith.addi %col, %i24 : index
    %fz = arith.constant 0.0 : f32
    %zero = air.simdgroup_matrix_fill %fz : vector<64xf32>
    %acc:16 = scf.for %kk = %i0 to %K step %i128
        iter_args(%o00 = %zero, %o01 = %zero, %o02 = %zero, %o03 = %zero, %o10 = %zero, %o11 = %zero, %o12 = %zero, %o13 = %zero, %o20 = %zero, %o21 = %zero, %o22 = %zero, %o23 = %zero, %o30 = %zero, %o31 = %zero, %o32 = %zero, %o33 = %zero) -> (vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>) {
      gpu.barrier memfence []
      %kk_end0 = arith.addi %kk, %i128 : index
      %kk_end = arith.minui %kk_end0, %K : index
      %in:16 = scf.for %k = %kk to %kk_end step %i8
          iter_args(%a00 = %o00, %a01 = %o01, %a02 = %o02, %a03 = %o03, %a10 = %o10, %a11 = %o11, %a12 = %o12, %a13 = %o13, %a20 = %o20, %a21 = %o21, %a22 = %o22, %a23 = %o23, %a30 = %o30, %a31 = %o31, %a32 = %o32, %a33 = %o33) -> (vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>) {
        %ma0 = air.simdgroup_matrix_load %A[%k, %row] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %ma1 = air.simdgroup_matrix_load %A[%k, %row1] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %ma2 = air.simdgroup_matrix_load %A[%k, %row2] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %ma3 = air.simdgroup_matrix_load %A[%k, %row3] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb0 = air.simdgroup_matrix_load %B[%k, %col] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb1 = air.simdgroup_matrix_load %B[%k, %col1] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb2 = air.simdgroup_matrix_load %B[%k, %col2] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb3 = air.simdgroup_matrix_load %B[%k, %col3] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %n00 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb0, %a00 : vector<64xf16>, vector<64xf32>
        %n01 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb1, %a01 : vector<64xf16>, vector<64xf32>
        %n02 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb2, %a02 : vector<64xf16>, vector<64xf32>
        %n03 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb3, %a03 : vector<64xf16>, vector<64xf32>
        %n10 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb0, %a10 : vector<64xf16>, vector<64xf32>
        %n11 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb1, %a11 : vector<64xf16>, vector<64xf32>
        %n12 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb2, %a12 : vector<64xf16>, vector<64xf32>
        %n13 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb3, %a13 : vector<64xf16>, vector<64xf32>
        %n20 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb0, %a20 : vector<64xf16>, vector<64xf32>
        %n21 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb1, %a21 : vector<64xf16>, vector<64xf32>
        %n22 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb2, %a22 : vector<64xf16>, vector<64xf32>
        %n23 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb3, %a23 : vector<64xf16>, vector<64xf32>
        %n30 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb0, %a30 : vector<64xf16>, vector<64xf32>
        %n31 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb1, %a31 : vector<64xf16>, vector<64xf32>
        %n32 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb2, %a32 : vector<64xf16>, vector<64xf32>
        %n33 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb3, %a33 : vector<64xf16>, vector<64xf32>
        scf.yield %n00, %n01, %n02, %n03, %n10, %n11, %n12, %n13, %n20, %n21, %n22, %n23, %n30, %n31, %n32, %n33 : vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>
      }
      scf.yield %in#0, %in#1, %in#2, %in#3, %in#4, %in#5, %in#6, %in#7, %in#8, %in#9, %in#10, %in#11, %in#12, %in#13, %in#14, %in#15 : vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>
    }

    // C = alpha * acc + beta * C, C not read when beta is 0 (one branch for the tile).
    %no_c = arith.cmpf oeq, %beta, %fz : f32
    %malpha = air.simdgroup_matrix_fill %alpha : vector<64xf32>
    %mbeta = air.simdgroup_matrix_fill %beta : vector<64xf32>
    scf.if %no_c {
      %s_00 = arith.mulf %acc#0, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_00, %C[%row, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_01 = arith.mulf %acc#1, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_01, %C[%row, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_02 = arith.mulf %acc#2, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_02, %C[%row, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_03 = arith.mulf %acc#3, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_03, %C[%row, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_10 = arith.mulf %acc#4, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_10, %C[%row1, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_11 = arith.mulf %acc#5, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_11, %C[%row1, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_12 = arith.mulf %acc#6, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_12, %C[%row1, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_13 = arith.mulf %acc#7, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_13, %C[%row1, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_20 = arith.mulf %acc#8, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_20, %C[%row2, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_21 = arith.mulf %acc#9, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_21, %C[%row2, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_22 = arith.mulf %acc#10, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_22, %C[%row2, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_23 = arith.mulf %acc#11, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_23, %C[%row2, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_30 = arith.mulf %acc#12, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_30, %C[%row3, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_31 = arith.mulf %acc#13, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_31, %C[%row3, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_32 = arith.mulf %acc#14, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_32, %C[%row3, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_33 = arith.mulf %acc#15, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_33, %C[%row3, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
    } else {
      %s_00 = arith.mulf %acc#0, %malpha : vector<64xf32>
      %old_00 = air.simdgroup_matrix_load %C[%row, %col] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_00 = math.fma %old_00, %mbeta, %s_00 : vector<64xf32>
      air.simdgroup_matrix_store %r_00, %C[%row, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_01 = arith.mulf %acc#1, %malpha : vector<64xf32>
      %old_01 = air.simdgroup_matrix_load %C[%row, %col1] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_01 = math.fma %old_01, %mbeta, %s_01 : vector<64xf32>
      air.simdgroup_matrix_store %r_01, %C[%row, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_02 = arith.mulf %acc#2, %malpha : vector<64xf32>
      %old_02 = air.simdgroup_matrix_load %C[%row, %col2] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_02 = math.fma %old_02, %mbeta, %s_02 : vector<64xf32>
      air.simdgroup_matrix_store %r_02, %C[%row, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_03 = arith.mulf %acc#3, %malpha : vector<64xf32>
      %old_03 = air.simdgroup_matrix_load %C[%row, %col3] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_03 = math.fma %old_03, %mbeta, %s_03 : vector<64xf32>
      air.simdgroup_matrix_store %r_03, %C[%row, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_10 = arith.mulf %acc#4, %malpha : vector<64xf32>
      %old_10 = air.simdgroup_matrix_load %C[%row1, %col] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_10 = math.fma %old_10, %mbeta, %s_10 : vector<64xf32>
      air.simdgroup_matrix_store %r_10, %C[%row1, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_11 = arith.mulf %acc#5, %malpha : vector<64xf32>
      %old_11 = air.simdgroup_matrix_load %C[%row1, %col1] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_11 = math.fma %old_11, %mbeta, %s_11 : vector<64xf32>
      air.simdgroup_matrix_store %r_11, %C[%row1, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_12 = arith.mulf %acc#6, %malpha : vector<64xf32>
      %old_12 = air.simdgroup_matrix_load %C[%row1, %col2] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_12 = math.fma %old_12, %mbeta, %s_12 : vector<64xf32>
      air.simdgroup_matrix_store %r_12, %C[%row1, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_13 = arith.mulf %acc#7, %malpha : vector<64xf32>
      %old_13 = air.simdgroup_matrix_load %C[%row1, %col3] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_13 = math.fma %old_13, %mbeta, %s_13 : vector<64xf32>
      air.simdgroup_matrix_store %r_13, %C[%row1, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_20 = arith.mulf %acc#8, %malpha : vector<64xf32>
      %old_20 = air.simdgroup_matrix_load %C[%row2, %col] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_20 = math.fma %old_20, %mbeta, %s_20 : vector<64xf32>
      air.simdgroup_matrix_store %r_20, %C[%row2, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_21 = arith.mulf %acc#9, %malpha : vector<64xf32>
      %old_21 = air.simdgroup_matrix_load %C[%row2, %col1] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_21 = math.fma %old_21, %mbeta, %s_21 : vector<64xf32>
      air.simdgroup_matrix_store %r_21, %C[%row2, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_22 = arith.mulf %acc#10, %malpha : vector<64xf32>
      %old_22 = air.simdgroup_matrix_load %C[%row2, %col2] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_22 = math.fma %old_22, %mbeta, %s_22 : vector<64xf32>
      air.simdgroup_matrix_store %r_22, %C[%row2, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_23 = arith.mulf %acc#11, %malpha : vector<64xf32>
      %old_23 = air.simdgroup_matrix_load %C[%row2, %col3] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_23 = math.fma %old_23, %mbeta, %s_23 : vector<64xf32>
      air.simdgroup_matrix_store %r_23, %C[%row2, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_30 = arith.mulf %acc#12, %malpha : vector<64xf32>
      %old_30 = air.simdgroup_matrix_load %C[%row3, %col] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_30 = math.fma %old_30, %mbeta, %s_30 : vector<64xf32>
      air.simdgroup_matrix_store %r_30, %C[%row3, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_31 = arith.mulf %acc#13, %malpha : vector<64xf32>
      %old_31 = air.simdgroup_matrix_load %C[%row3, %col1] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_31 = math.fma %old_31, %mbeta, %s_31 : vector<64xf32>
      air.simdgroup_matrix_store %r_31, %C[%row3, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_32 = arith.mulf %acc#14, %malpha : vector<64xf32>
      %old_32 = air.simdgroup_matrix_load %C[%row3, %col2] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_32 = math.fma %old_32, %mbeta, %s_32 : vector<64xf32>
      air.simdgroup_matrix_store %r_32, %C[%row3, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_33 = arith.mulf %acc#15, %malpha : vector<64xf32>
      %old_33 = air.simdgroup_matrix_load %C[%row3, %col3] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_33 = math.fma %old_33, %mbeta, %s_33 : vector<64xf32>
      air.simdgroup_matrix_store %r_33, %C[%row3, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
    }
    gpu.return
  }

  // agx_hgemm_tile64_cf32 with B[k, c] at b[c * ldb + k] (transposed).
  gpu.func @agx_hgemm_tile64_cf32_tb(%a0: memref<1xf16, 1>, %b0: memref<1xf16, 1>,
      %c0: memref<1xf32, 1>, %M: index, %N: index, %K: index, %lda: index,
      %ldb: index, %ldc: index, %bsa: index, %bsb: index, %bsc: index, %alpha: f32,
      %beta: f32) kernel
      attributes {known_block_size = array<i32: 128, 1, 1>} {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %i2 = arith.constant 2 : index
    %i8 = arith.constant 8 : index
    %i16 = arith.constant 16 : index
    %i24 = arith.constant 24 : index
    %i32 = arith.constant 32 : index
    %i64 = arith.constant 64 : index
    %i128 = arith.constant 128 : index
    %even = arith.constant -2 : index
    %a0a = memref.assume_alignment %a0, 8 : memref<1xf16, 1>
    %b0a = memref.assume_alignment %b0, 8 : memref<1xf16, 1>
    %c0a = memref.assume_alignment %c0, 8 : memref<1xf32, 1>
    %lda_e = arith.andi %lda, %even : index
    %ldb_e = arith.andi %ldb, %even : index
    %ldc_e = arith.andi %ldc, %even : index
    %z = gpu.block_id y
    %bsa_e = arith.andi %bsa, %even : index
    %oa = arith.muli %z, %bsa_e : index
    %bsb_e = arith.andi %bsb, %even : index
    %ob = arith.muli %z, %bsb_e : index
    %bsc_e = arith.andi %bsc, %even : index
    %oc = arith.muli %z, %bsc_e : index
    %A = memref.reinterpret_cast %a0a to offset: [%oa], sizes: [%M, %K], strides: [%lda_e, 1]
        : memref<1xf16, 1> to memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
    %B = memref.reinterpret_cast %b0a to offset: [%ob], sizes: [%N, %K], strides: [%ldb_e, 1]
        : memref<1xf16, 1> to memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
    %C = memref.reinterpret_cast %c0a to offset: [%oc], sizes: [%M, %N], strides: [%ldc_e, 1]
        : memref<1xf32, 1> to memref<?x?xf32, strided<[?, 1], offset: ?>, 1>

    // Grouped tile order: groups of 8 tile rows, column-major inside a group.
    %tiles_m = arith.divui %M, %i64 : index
    %tiles_n = arith.divui %N, %i64 : index
    %id = gpu.block_id x
    %per_group = arith.muli %tiles_n, %i8 : index
    %group = arith.divui %id, %per_group : index
    %first_m = arith.muli %group, %i8 : index
    %left_m = arith.subi %tiles_m, %first_m : index
    %group_m = arith.minui %left_m, %i8 : index
    %in_group = arith.remui %id, %per_group : index
    %rem_m = arith.remui %in_group, %group_m : index
    %tm = arith.addi %first_m, %rem_m : index
    %tn = arith.divui %in_group, %group_m : index

    // This SIMD-group's 32 x 32 tile: 4 x 4 matrices.
    %sg = gpu.subgroup_id : index
    %sr = arith.divui %sg, %i2 : index
    %sc = arith.remui %sg, %i2 : index
    %tm64 = arith.muli %tm, %i64 : index
    %tn64 = arith.muli %tn, %i64 : index
    %sr32 = arith.muli %sr, %i32 : index
    %sc32 = arith.muli %sc, %i32 : index
    %row = arith.addi %tm64, %sr32 : index
    %col = arith.addi %tn64, %sc32 : index
    %row1 = arith.addi %row, %i8 : index
    %col1 = arith.addi %col, %i8 : index
    %row2 = arith.addi %row, %i16 : index
    %col2 = arith.addi %col, %i16 : index
    %row3 = arith.addi %row, %i24 : index
    %col3 = arith.addi %col, %i24 : index
    %fz = arith.constant 0.0 : f32
    %zero = air.simdgroup_matrix_fill %fz : vector<64xf32>
    %acc:16 = scf.for %kk = %i0 to %K step %i128
        iter_args(%o00 = %zero, %o01 = %zero, %o02 = %zero, %o03 = %zero, %o10 = %zero, %o11 = %zero, %o12 = %zero, %o13 = %zero, %o20 = %zero, %o21 = %zero, %o22 = %zero, %o23 = %zero, %o30 = %zero, %o31 = %zero, %o32 = %zero, %o33 = %zero) -> (vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>) {
      gpu.barrier memfence []
      %kk_end0 = arith.addi %kk, %i128 : index
      %kk_end = arith.minui %kk_end0, %K : index
      %in:16 = scf.for %k = %kk to %kk_end step %i8
          iter_args(%a00 = %o00, %a01 = %o01, %a02 = %o02, %a03 = %o03, %a10 = %o10, %a11 = %o11, %a12 = %o12, %a13 = %o13, %a20 = %o20, %a21 = %o21, %a22 = %o22, %a23 = %o23, %a30 = %o30, %a31 = %o31, %a32 = %o32, %a33 = %o33) -> (vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>) {
        %ma0 = air.simdgroup_matrix_load %A[%row, %k] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %ma1 = air.simdgroup_matrix_load %A[%row1, %k] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %ma2 = air.simdgroup_matrix_load %A[%row2, %k] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %ma3 = air.simdgroup_matrix_load %A[%row3, %k] : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb0 = air.simdgroup_matrix_load %B[%col, %k] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb1 = air.simdgroup_matrix_load %B[%col1, %k] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb2 = air.simdgroup_matrix_load %B[%col2, %k] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb3 = air.simdgroup_matrix_load %B[%col3, %k] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %n00 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb0, %a00 : vector<64xf16>, vector<64xf32>
        %n01 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb1, %a01 : vector<64xf16>, vector<64xf32>
        %n02 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb2, %a02 : vector<64xf16>, vector<64xf32>
        %n03 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb3, %a03 : vector<64xf16>, vector<64xf32>
        %n10 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb0, %a10 : vector<64xf16>, vector<64xf32>
        %n11 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb1, %a11 : vector<64xf16>, vector<64xf32>
        %n12 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb2, %a12 : vector<64xf16>, vector<64xf32>
        %n13 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb3, %a13 : vector<64xf16>, vector<64xf32>
        %n20 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb0, %a20 : vector<64xf16>, vector<64xf32>
        %n21 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb1, %a21 : vector<64xf16>, vector<64xf32>
        %n22 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb2, %a22 : vector<64xf16>, vector<64xf32>
        %n23 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb3, %a23 : vector<64xf16>, vector<64xf32>
        %n30 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb0, %a30 : vector<64xf16>, vector<64xf32>
        %n31 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb1, %a31 : vector<64xf16>, vector<64xf32>
        %n32 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb2, %a32 : vector<64xf16>, vector<64xf32>
        %n33 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb3, %a33 : vector<64xf16>, vector<64xf32>
        scf.yield %n00, %n01, %n02, %n03, %n10, %n11, %n12, %n13, %n20, %n21, %n22, %n23, %n30, %n31, %n32, %n33 : vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>
      }
      scf.yield %in#0, %in#1, %in#2, %in#3, %in#4, %in#5, %in#6, %in#7, %in#8, %in#9, %in#10, %in#11, %in#12, %in#13, %in#14, %in#15 : vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>
    }

    // C = alpha * acc + beta * C, C not read when beta is 0 (one branch for the tile).
    %no_c = arith.cmpf oeq, %beta, %fz : f32
    %malpha = air.simdgroup_matrix_fill %alpha : vector<64xf32>
    %mbeta = air.simdgroup_matrix_fill %beta : vector<64xf32>
    scf.if %no_c {
      %s_00 = arith.mulf %acc#0, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_00, %C[%row, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_01 = arith.mulf %acc#1, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_01, %C[%row, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_02 = arith.mulf %acc#2, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_02, %C[%row, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_03 = arith.mulf %acc#3, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_03, %C[%row, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_10 = arith.mulf %acc#4, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_10, %C[%row1, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_11 = arith.mulf %acc#5, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_11, %C[%row1, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_12 = arith.mulf %acc#6, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_12, %C[%row1, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_13 = arith.mulf %acc#7, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_13, %C[%row1, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_20 = arith.mulf %acc#8, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_20, %C[%row2, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_21 = arith.mulf %acc#9, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_21, %C[%row2, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_22 = arith.mulf %acc#10, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_22, %C[%row2, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_23 = arith.mulf %acc#11, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_23, %C[%row2, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_30 = arith.mulf %acc#12, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_30, %C[%row3, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_31 = arith.mulf %acc#13, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_31, %C[%row3, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_32 = arith.mulf %acc#14, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_32, %C[%row3, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_33 = arith.mulf %acc#15, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_33, %C[%row3, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
    } else {
      %s_00 = arith.mulf %acc#0, %malpha : vector<64xf32>
      %old_00 = air.simdgroup_matrix_load %C[%row, %col] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_00 = math.fma %old_00, %mbeta, %s_00 : vector<64xf32>
      air.simdgroup_matrix_store %r_00, %C[%row, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_01 = arith.mulf %acc#1, %malpha : vector<64xf32>
      %old_01 = air.simdgroup_matrix_load %C[%row, %col1] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_01 = math.fma %old_01, %mbeta, %s_01 : vector<64xf32>
      air.simdgroup_matrix_store %r_01, %C[%row, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_02 = arith.mulf %acc#2, %malpha : vector<64xf32>
      %old_02 = air.simdgroup_matrix_load %C[%row, %col2] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_02 = math.fma %old_02, %mbeta, %s_02 : vector<64xf32>
      air.simdgroup_matrix_store %r_02, %C[%row, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_03 = arith.mulf %acc#3, %malpha : vector<64xf32>
      %old_03 = air.simdgroup_matrix_load %C[%row, %col3] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_03 = math.fma %old_03, %mbeta, %s_03 : vector<64xf32>
      air.simdgroup_matrix_store %r_03, %C[%row, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_10 = arith.mulf %acc#4, %malpha : vector<64xf32>
      %old_10 = air.simdgroup_matrix_load %C[%row1, %col] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_10 = math.fma %old_10, %mbeta, %s_10 : vector<64xf32>
      air.simdgroup_matrix_store %r_10, %C[%row1, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_11 = arith.mulf %acc#5, %malpha : vector<64xf32>
      %old_11 = air.simdgroup_matrix_load %C[%row1, %col1] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_11 = math.fma %old_11, %mbeta, %s_11 : vector<64xf32>
      air.simdgroup_matrix_store %r_11, %C[%row1, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_12 = arith.mulf %acc#6, %malpha : vector<64xf32>
      %old_12 = air.simdgroup_matrix_load %C[%row1, %col2] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_12 = math.fma %old_12, %mbeta, %s_12 : vector<64xf32>
      air.simdgroup_matrix_store %r_12, %C[%row1, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_13 = arith.mulf %acc#7, %malpha : vector<64xf32>
      %old_13 = air.simdgroup_matrix_load %C[%row1, %col3] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_13 = math.fma %old_13, %mbeta, %s_13 : vector<64xf32>
      air.simdgroup_matrix_store %r_13, %C[%row1, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_20 = arith.mulf %acc#8, %malpha : vector<64xf32>
      %old_20 = air.simdgroup_matrix_load %C[%row2, %col] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_20 = math.fma %old_20, %mbeta, %s_20 : vector<64xf32>
      air.simdgroup_matrix_store %r_20, %C[%row2, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_21 = arith.mulf %acc#9, %malpha : vector<64xf32>
      %old_21 = air.simdgroup_matrix_load %C[%row2, %col1] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_21 = math.fma %old_21, %mbeta, %s_21 : vector<64xf32>
      air.simdgroup_matrix_store %r_21, %C[%row2, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_22 = arith.mulf %acc#10, %malpha : vector<64xf32>
      %old_22 = air.simdgroup_matrix_load %C[%row2, %col2] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_22 = math.fma %old_22, %mbeta, %s_22 : vector<64xf32>
      air.simdgroup_matrix_store %r_22, %C[%row2, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_23 = arith.mulf %acc#11, %malpha : vector<64xf32>
      %old_23 = air.simdgroup_matrix_load %C[%row2, %col3] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_23 = math.fma %old_23, %mbeta, %s_23 : vector<64xf32>
      air.simdgroup_matrix_store %r_23, %C[%row2, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_30 = arith.mulf %acc#12, %malpha : vector<64xf32>
      %old_30 = air.simdgroup_matrix_load %C[%row3, %col] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_30 = math.fma %old_30, %mbeta, %s_30 : vector<64xf32>
      air.simdgroup_matrix_store %r_30, %C[%row3, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_31 = arith.mulf %acc#13, %malpha : vector<64xf32>
      %old_31 = air.simdgroup_matrix_load %C[%row3, %col1] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_31 = math.fma %old_31, %mbeta, %s_31 : vector<64xf32>
      air.simdgroup_matrix_store %r_31, %C[%row3, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_32 = arith.mulf %acc#14, %malpha : vector<64xf32>
      %old_32 = air.simdgroup_matrix_load %C[%row3, %col2] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_32 = math.fma %old_32, %mbeta, %s_32 : vector<64xf32>
      air.simdgroup_matrix_store %r_32, %C[%row3, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_33 = arith.mulf %acc#15, %malpha : vector<64xf32>
      %old_33 = air.simdgroup_matrix_load %C[%row3, %col3] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_33 = math.fma %old_33, %mbeta, %s_33 : vector<64xf32>
      air.simdgroup_matrix_store %r_33, %C[%row3, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
    }
    gpu.return
  }

  // agx_hgemm_tile64_cf32 with A[r, k] at a[k * lda + r] and B[k, c] at b[c * ldb + k] (transposed).
  gpu.func @agx_hgemm_tile64_cf32_tatb(%a0: memref<1xf16, 1>, %b0: memref<1xf16, 1>,
      %c0: memref<1xf32, 1>, %M: index, %N: index, %K: index, %lda: index,
      %ldb: index, %ldc: index, %bsa: index, %bsb: index, %bsc: index, %alpha: f32,
      %beta: f32) kernel
      attributes {known_block_size = array<i32: 128, 1, 1>} {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %i2 = arith.constant 2 : index
    %i8 = arith.constant 8 : index
    %i16 = arith.constant 16 : index
    %i24 = arith.constant 24 : index
    %i32 = arith.constant 32 : index
    %i64 = arith.constant 64 : index
    %i128 = arith.constant 128 : index
    %even = arith.constant -2 : index
    %a0a = memref.assume_alignment %a0, 8 : memref<1xf16, 1>
    %b0a = memref.assume_alignment %b0, 8 : memref<1xf16, 1>
    %c0a = memref.assume_alignment %c0, 8 : memref<1xf32, 1>
    %lda_e = arith.andi %lda, %even : index
    %ldb_e = arith.andi %ldb, %even : index
    %ldc_e = arith.andi %ldc, %even : index
    %z = gpu.block_id y
    %bsa_e = arith.andi %bsa, %even : index
    %oa = arith.muli %z, %bsa_e : index
    %bsb_e = arith.andi %bsb, %even : index
    %ob = arith.muli %z, %bsb_e : index
    %bsc_e = arith.andi %bsc, %even : index
    %oc = arith.muli %z, %bsc_e : index
    %A = memref.reinterpret_cast %a0a to offset: [%oa], sizes: [%K, %M], strides: [%lda_e, 1]
        : memref<1xf16, 1> to memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
    %B = memref.reinterpret_cast %b0a to offset: [%ob], sizes: [%N, %K], strides: [%ldb_e, 1]
        : memref<1xf16, 1> to memref<?x?xf16, strided<[?, 1], offset: ?>, 1>
    %C = memref.reinterpret_cast %c0a to offset: [%oc], sizes: [%M, %N], strides: [%ldc_e, 1]
        : memref<1xf32, 1> to memref<?x?xf32, strided<[?, 1], offset: ?>, 1>

    // Grouped tile order: groups of 8 tile rows, column-major inside a group.
    %tiles_m = arith.divui %M, %i64 : index
    %tiles_n = arith.divui %N, %i64 : index
    %id = gpu.block_id x
    %per_group = arith.muli %tiles_n, %i8 : index
    %group = arith.divui %id, %per_group : index
    %first_m = arith.muli %group, %i8 : index
    %left_m = arith.subi %tiles_m, %first_m : index
    %group_m = arith.minui %left_m, %i8 : index
    %in_group = arith.remui %id, %per_group : index
    %rem_m = arith.remui %in_group, %group_m : index
    %tm = arith.addi %first_m, %rem_m : index
    %tn = arith.divui %in_group, %group_m : index

    // This SIMD-group's 32 x 32 tile: 4 x 4 matrices.
    %sg = gpu.subgroup_id : index
    %sr = arith.divui %sg, %i2 : index
    %sc = arith.remui %sg, %i2 : index
    %tm64 = arith.muli %tm, %i64 : index
    %tn64 = arith.muli %tn, %i64 : index
    %sr32 = arith.muli %sr, %i32 : index
    %sc32 = arith.muli %sc, %i32 : index
    %row = arith.addi %tm64, %sr32 : index
    %col = arith.addi %tn64, %sc32 : index
    %row1 = arith.addi %row, %i8 : index
    %col1 = arith.addi %col, %i8 : index
    %row2 = arith.addi %row, %i16 : index
    %col2 = arith.addi %col, %i16 : index
    %row3 = arith.addi %row, %i24 : index
    %col3 = arith.addi %col, %i24 : index
    %fz = arith.constant 0.0 : f32
    %zero = air.simdgroup_matrix_fill %fz : vector<64xf32>
    %acc:16 = scf.for %kk = %i0 to %K step %i128
        iter_args(%o00 = %zero, %o01 = %zero, %o02 = %zero, %o03 = %zero, %o10 = %zero, %o11 = %zero, %o12 = %zero, %o13 = %zero, %o20 = %zero, %o21 = %zero, %o22 = %zero, %o23 = %zero, %o30 = %zero, %o31 = %zero, %o32 = %zero, %o33 = %zero) -> (vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>) {
      gpu.barrier memfence []
      %kk_end0 = arith.addi %kk, %i128 : index
      %kk_end = arith.minui %kk_end0, %K : index
      %in:16 = scf.for %k = %kk to %kk_end step %i8
          iter_args(%a00 = %o00, %a01 = %o01, %a02 = %o02, %a03 = %o03, %a10 = %o10, %a11 = %o11, %a12 = %o12, %a13 = %o13, %a20 = %o20, %a21 = %o21, %a22 = %o22, %a23 = %o23, %a30 = %o30, %a31 = %o31, %a32 = %o32, %a33 = %o33) -> (vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>) {
        %ma0 = air.simdgroup_matrix_load %A[%k, %row] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %ma1 = air.simdgroup_matrix_load %A[%k, %row1] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %ma2 = air.simdgroup_matrix_load %A[%k, %row2] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %ma3 = air.simdgroup_matrix_load %A[%k, %row3] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb0 = air.simdgroup_matrix_load %B[%col, %k] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb1 = air.simdgroup_matrix_load %B[%col1, %k] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb2 = air.simdgroup_matrix_load %B[%col2, %k] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %mb3 = air.simdgroup_matrix_load %B[%col3, %k] transpose : memref<?x?xf16, strided<[?, 1], offset: ?>, 1> -> vector<64xf16>
        %n00 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb0, %a00 : vector<64xf16>, vector<64xf32>
        %n01 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb1, %a01 : vector<64xf16>, vector<64xf32>
        %n02 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb2, %a02 : vector<64xf16>, vector<64xf32>
        %n03 = air.simdgroup_matrix_multiply_accumulate %ma0, %mb3, %a03 : vector<64xf16>, vector<64xf32>
        %n10 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb0, %a10 : vector<64xf16>, vector<64xf32>
        %n11 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb1, %a11 : vector<64xf16>, vector<64xf32>
        %n12 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb2, %a12 : vector<64xf16>, vector<64xf32>
        %n13 = air.simdgroup_matrix_multiply_accumulate %ma1, %mb3, %a13 : vector<64xf16>, vector<64xf32>
        %n20 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb0, %a20 : vector<64xf16>, vector<64xf32>
        %n21 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb1, %a21 : vector<64xf16>, vector<64xf32>
        %n22 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb2, %a22 : vector<64xf16>, vector<64xf32>
        %n23 = air.simdgroup_matrix_multiply_accumulate %ma2, %mb3, %a23 : vector<64xf16>, vector<64xf32>
        %n30 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb0, %a30 : vector<64xf16>, vector<64xf32>
        %n31 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb1, %a31 : vector<64xf16>, vector<64xf32>
        %n32 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb2, %a32 : vector<64xf16>, vector<64xf32>
        %n33 = air.simdgroup_matrix_multiply_accumulate %ma3, %mb3, %a33 : vector<64xf16>, vector<64xf32>
        scf.yield %n00, %n01, %n02, %n03, %n10, %n11, %n12, %n13, %n20, %n21, %n22, %n23, %n30, %n31, %n32, %n33 : vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>
      }
      scf.yield %in#0, %in#1, %in#2, %in#3, %in#4, %in#5, %in#6, %in#7, %in#8, %in#9, %in#10, %in#11, %in#12, %in#13, %in#14, %in#15 : vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>, vector<64xf32>
    }

    // C = alpha * acc + beta * C, C not read when beta is 0 (one branch for the tile).
    %no_c = arith.cmpf oeq, %beta, %fz : f32
    %malpha = air.simdgroup_matrix_fill %alpha : vector<64xf32>
    %mbeta = air.simdgroup_matrix_fill %beta : vector<64xf32>
    scf.if %no_c {
      %s_00 = arith.mulf %acc#0, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_00, %C[%row, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_01 = arith.mulf %acc#1, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_01, %C[%row, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_02 = arith.mulf %acc#2, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_02, %C[%row, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_03 = arith.mulf %acc#3, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_03, %C[%row, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_10 = arith.mulf %acc#4, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_10, %C[%row1, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_11 = arith.mulf %acc#5, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_11, %C[%row1, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_12 = arith.mulf %acc#6, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_12, %C[%row1, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_13 = arith.mulf %acc#7, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_13, %C[%row1, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_20 = arith.mulf %acc#8, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_20, %C[%row2, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_21 = arith.mulf %acc#9, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_21, %C[%row2, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_22 = arith.mulf %acc#10, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_22, %C[%row2, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_23 = arith.mulf %acc#11, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_23, %C[%row2, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_30 = arith.mulf %acc#12, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_30, %C[%row3, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_31 = arith.mulf %acc#13, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_31, %C[%row3, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_32 = arith.mulf %acc#14, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_32, %C[%row3, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_33 = arith.mulf %acc#15, %malpha : vector<64xf32>
      air.simdgroup_matrix_store %s_33, %C[%row3, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
    } else {
      %s_00 = arith.mulf %acc#0, %malpha : vector<64xf32>
      %old_00 = air.simdgroup_matrix_load %C[%row, %col] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_00 = math.fma %old_00, %mbeta, %s_00 : vector<64xf32>
      air.simdgroup_matrix_store %r_00, %C[%row, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_01 = arith.mulf %acc#1, %malpha : vector<64xf32>
      %old_01 = air.simdgroup_matrix_load %C[%row, %col1] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_01 = math.fma %old_01, %mbeta, %s_01 : vector<64xf32>
      air.simdgroup_matrix_store %r_01, %C[%row, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_02 = arith.mulf %acc#2, %malpha : vector<64xf32>
      %old_02 = air.simdgroup_matrix_load %C[%row, %col2] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_02 = math.fma %old_02, %mbeta, %s_02 : vector<64xf32>
      air.simdgroup_matrix_store %r_02, %C[%row, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_03 = arith.mulf %acc#3, %malpha : vector<64xf32>
      %old_03 = air.simdgroup_matrix_load %C[%row, %col3] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_03 = math.fma %old_03, %mbeta, %s_03 : vector<64xf32>
      air.simdgroup_matrix_store %r_03, %C[%row, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_10 = arith.mulf %acc#4, %malpha : vector<64xf32>
      %old_10 = air.simdgroup_matrix_load %C[%row1, %col] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_10 = math.fma %old_10, %mbeta, %s_10 : vector<64xf32>
      air.simdgroup_matrix_store %r_10, %C[%row1, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_11 = arith.mulf %acc#5, %malpha : vector<64xf32>
      %old_11 = air.simdgroup_matrix_load %C[%row1, %col1] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_11 = math.fma %old_11, %mbeta, %s_11 : vector<64xf32>
      air.simdgroup_matrix_store %r_11, %C[%row1, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_12 = arith.mulf %acc#6, %malpha : vector<64xf32>
      %old_12 = air.simdgroup_matrix_load %C[%row1, %col2] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_12 = math.fma %old_12, %mbeta, %s_12 : vector<64xf32>
      air.simdgroup_matrix_store %r_12, %C[%row1, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_13 = arith.mulf %acc#7, %malpha : vector<64xf32>
      %old_13 = air.simdgroup_matrix_load %C[%row1, %col3] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_13 = math.fma %old_13, %mbeta, %s_13 : vector<64xf32>
      air.simdgroup_matrix_store %r_13, %C[%row1, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_20 = arith.mulf %acc#8, %malpha : vector<64xf32>
      %old_20 = air.simdgroup_matrix_load %C[%row2, %col] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_20 = math.fma %old_20, %mbeta, %s_20 : vector<64xf32>
      air.simdgroup_matrix_store %r_20, %C[%row2, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_21 = arith.mulf %acc#9, %malpha : vector<64xf32>
      %old_21 = air.simdgroup_matrix_load %C[%row2, %col1] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_21 = math.fma %old_21, %mbeta, %s_21 : vector<64xf32>
      air.simdgroup_matrix_store %r_21, %C[%row2, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_22 = arith.mulf %acc#10, %malpha : vector<64xf32>
      %old_22 = air.simdgroup_matrix_load %C[%row2, %col2] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_22 = math.fma %old_22, %mbeta, %s_22 : vector<64xf32>
      air.simdgroup_matrix_store %r_22, %C[%row2, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_23 = arith.mulf %acc#11, %malpha : vector<64xf32>
      %old_23 = air.simdgroup_matrix_load %C[%row2, %col3] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_23 = math.fma %old_23, %mbeta, %s_23 : vector<64xf32>
      air.simdgroup_matrix_store %r_23, %C[%row2, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_30 = arith.mulf %acc#12, %malpha : vector<64xf32>
      %old_30 = air.simdgroup_matrix_load %C[%row3, %col] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_30 = math.fma %old_30, %mbeta, %s_30 : vector<64xf32>
      air.simdgroup_matrix_store %r_30, %C[%row3, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_31 = arith.mulf %acc#13, %malpha : vector<64xf32>
      %old_31 = air.simdgroup_matrix_load %C[%row3, %col1] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_31 = math.fma %old_31, %mbeta, %s_31 : vector<64xf32>
      air.simdgroup_matrix_store %r_31, %C[%row3, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_32 = arith.mulf %acc#14, %malpha : vector<64xf32>
      %old_32 = air.simdgroup_matrix_load %C[%row3, %col2] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_32 = math.fma %old_32, %mbeta, %s_32 : vector<64xf32>
      air.simdgroup_matrix_store %r_32, %C[%row3, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
      %s_33 = arith.mulf %acc#15, %malpha : vector<64xf32>
      %old_33 = air.simdgroup_matrix_load %C[%row3, %col3] : memref<?x?xf32, strided<[?, 1], offset: ?>, 1> -> vector<64xf32>
      %r_33 = math.fma %old_33, %mbeta, %s_33 : vector<64xf32>
      air.simdgroup_matrix_store %r_33, %C[%row3, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1], offset: ?>, 1>
    }
    gpu.return
  }

  // As agx_hgemm_generic_cf16, with a SIMD-group per element of C whose lanes split K
  // (gen_small_gemm.py): for C with a side of at most 8, A or B contiguous along K.
  gpu.func @agx_hgemm_dot_cf16(%a0: memref<1xf16, 1>, %b0: memref<1xf16, 1>,
      %c0: memref<1xf16, 1>, %M: index, %N: index, %K: index, %sar: index, %sac: index,
      %sbr: index, %sbc: index, %scr: index, %scc: index, %sa: index, %sb: index, %sc: index,
      %alpha: f32, %beta: f32) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %i3 = arith.constant 3 : index
    %i4 = arith.constant 4 : index
    %i32 = arith.constant 32 : index
    %i96 = arith.constant 96 : index
    %i127 = arith.constant 127 : index
    %i128 = arith.constant 128 : index
    %big = arith.constant 4611686018427387904 : index
    %z = gpu.block_id y
    %oa = arith.muli %z, %sa : index
    %ob = arith.muli %z, %sb : index
    %oc = arith.muli %z, %sc : index
    %A = memref.reinterpret_cast %a0 to offset: [%oa], sizes: [%big], strides: [1]
        : memref<1xf16, 1> to memref<?xf16, strided<[1], offset: ?>, 1>
    %B = memref.reinterpret_cast %b0 to offset: [%ob], sizes: [%big], strides: [1]
        : memref<1xf16, 1> to memref<?xf16, strided<[1], offset: ?>, 1>
    %C = memref.reinterpret_cast %c0 to offset: [%oc], sizes: [%big], strides: [1]
        : memref<1xf16, 1> to memref<?xf16, strided<[1], offset: ?>, 1>
    %f0 = arith.constant 0.0 : f32
    %g = gpu.global_id x
    %s = arith.divui %g, %i32 : index
    %lane = gpu.lane_id
    %MN = arith.muli %M, %N : index
    // the same for the whole SIMD-group, which reduces together
    %live = arith.cmpi ult, %s, %MN : index
    scf.if %live {
      %i = arith.divui %s, %N : index
      %j = arith.remui %s, %N : index
      %ia = arith.muli %i, %sar : index
      %jb = arith.muli %j, %sbc : index
      // four steps at a time (four loads of each operand in flight), then one
      %Kx = arith.maxui %K, %i96 : index
      %ub4 = arith.subi %Kx, %i96 : index
      %s4:4 = scf.for %k = %lane to %ub4 step %i128
          iter_args(%c0_ = %f0, %c1_ = %f0, %c2_ = %f0, %c3_ = %f0) -> (f32, f32, f32, f32) {
        %k1 = arith.addi %k, %i32 : index
        %k2 = arith.addi %k1, %i32 : index
        %k3 = arith.addi %k2, %i32 : index
        %ka_d0 = arith.muli %k, %sac : index
        %kb_d0 = arith.muli %k, %sbr : index
        %pa_d0 = arith.addi %ia, %ka_d0 : index
        %pb_d0 = arith.addi %kb_d0, %jb : index
        %xn_d0 = memref.load %A[%pa_d0] : memref<?xf16, strided<[1], offset: ?>, 1>
        %yn_d0 = memref.load %B[%pb_d0] : memref<?xf16, strided<[1], offset: ?>, 1>
        %x_d0 = arith.extf %xn_d0 : f16 to f32
        %y_d0 = arith.extf %yn_d0 : f16 to f32
        %d0 = math.fma %x_d0, %y_d0, %c0_ : f32
        %ka_d1 = arith.muli %k1, %sac : index
        %kb_d1 = arith.muli %k1, %sbr : index
        %pa_d1 = arith.addi %ia, %ka_d1 : index
        %pb_d1 = arith.addi %kb_d1, %jb : index
        %xn_d1 = memref.load %A[%pa_d1] : memref<?xf16, strided<[1], offset: ?>, 1>
        %yn_d1 = memref.load %B[%pb_d1] : memref<?xf16, strided<[1], offset: ?>, 1>
        %x_d1 = arith.extf %xn_d1 : f16 to f32
        %y_d1 = arith.extf %yn_d1 : f16 to f32
        %d1 = math.fma %x_d1, %y_d1, %c1_ : f32
        %ka_d2 = arith.muli %k2, %sac : index
        %kb_d2 = arith.muli %k2, %sbr : index
        %pa_d2 = arith.addi %ia, %ka_d2 : index
        %pb_d2 = arith.addi %kb_d2, %jb : index
        %xn_d2 = memref.load %A[%pa_d2] : memref<?xf16, strided<[1], offset: ?>, 1>
        %yn_d2 = memref.load %B[%pb_d2] : memref<?xf16, strided<[1], offset: ?>, 1>
        %x_d2 = arith.extf %xn_d2 : f16 to f32
        %y_d2 = arith.extf %yn_d2 : f16 to f32
        %d2 = math.fma %x_d2, %y_d2, %c2_ : f32
        %ka_d3 = arith.muli %k3, %sac : index
        %kb_d3 = arith.muli %k3, %sbr : index
        %pa_d3 = arith.addi %ia, %ka_d3 : index
        %pb_d3 = arith.addi %kb_d3, %jb : index
        %xn_d3 = memref.load %A[%pa_d3] : memref<?xf16, strided<[1], offset: ?>, 1>
        %yn_d3 = memref.load %B[%pb_d3] : memref<?xf16, strided<[1], offset: ?>, 1>
        %x_d3 = arith.extf %xn_d3 : f16 to f32
        %y_d3 = arith.extf %yn_d3 : f16 to f32
        %d3 = math.fma %x_d3, %y_d3, %c3_ : f32
        scf.yield %d0, %d1, %d2, %d3 : f32, f32, f32, f32
      }
      // the rest from the first k the loop above did not reach
      %after = arith.cmpi ugt, %ub4, %lane : index
      %span = arith.subi %ub4, %lane : index
      %span1 = arith.addi %span, %i127 : index
      %trips = arith.divui %span1, %i128 : index
      %done = arith.muli %trips, %i128 : index
      %krest0 = arith.addi %lane, %done : index
      %krest = arith.select %after, %krest0, %lane : index
      %s01 = arith.addf %s4#0, %s4#1 : f32
      %s23 = arith.addf %s4#2, %s4#3 : f32
      %sum0 = arith.addf %s01, %s23 : f32
      %part = scf.for %k = %krest to %K step %i32 iter_args(%a = %sum0) -> f32 {
        %ka_a1 = arith.muli %k, %sac : index
        %kb_a1 = arith.muli %k, %sbr : index
        %pa_a1 = arith.addi %ia, %ka_a1 : index
        %pb_a1 = arith.addi %kb_a1, %jb : index
        %xn_a1 = memref.load %A[%pa_a1] : memref<?xf16, strided<[1], offset: ?>, 1>
        %yn_a1 = memref.load %B[%pb_a1] : memref<?xf16, strided<[1], offset: ?>, 1>
        %x_a1 = arith.extf %xn_a1 : f16 to f32
        %y_a1 = arith.extf %yn_a1 : f16 to f32
        %a1 = math.fma %x_a1, %y_a1, %a : f32
        scf.yield %a1 : f32
      }
      %sum = gpu.subgroup_reduce add %part : (f32) -> f32
      %first = arith.cmpi eq, %lane, %i0 : index
      scf.if %first {
        %ic = arith.muli %i, %scr : index
        %jc = arith.muli %j, %scc : index
        %pc = arith.addi %ic, %jc : index
        %scaled = arith.mulf %sum, %alpha : f32
        %no_c = arith.cmpf oeq, %beta, %f0 : f32
        %out = scf.if %no_c -> f32 {
          scf.yield %scaled : f32
        } else {
          %oldn = memref.load %C[%pc] : memref<?xf16, strided<[1], offset: ?>, 1>
          %old = arith.extf %oldn : f16 to f32
          %r = math.fma %old, %beta, %scaled : f32
          scf.yield %r : f32
        }
        %outn = arith.truncf %out : f32 to f16
        memref.store %outn, %C[%pc] : memref<?xf16, strided<[1], offset: ?>, 1>
      }
    }
    gpu.return
  }

  // As agx_hgemm_generic_cf32, with a SIMD-group per element of C whose lanes split K
  // (gen_small_gemm.py): for C with a side of at most 8, A or B contiguous along K.
  gpu.func @agx_hgemm_dot_cf32(%a0: memref<1xf16, 1>, %b0: memref<1xf16, 1>,
      %c0: memref<1xf32, 1>, %M: index, %N: index, %K: index, %sar: index, %sac: index,
      %sbr: index, %sbc: index, %scr: index, %scc: index, %sa: index, %sb: index, %sc: index,
      %alpha: f32, %beta: f32) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %i3 = arith.constant 3 : index
    %i4 = arith.constant 4 : index
    %i32 = arith.constant 32 : index
    %i96 = arith.constant 96 : index
    %i127 = arith.constant 127 : index
    %i128 = arith.constant 128 : index
    %big = arith.constant 4611686018427387904 : index
    %z = gpu.block_id y
    %oa = arith.muli %z, %sa : index
    %ob = arith.muli %z, %sb : index
    %oc = arith.muli %z, %sc : index
    %A = memref.reinterpret_cast %a0 to offset: [%oa], sizes: [%big], strides: [1]
        : memref<1xf16, 1> to memref<?xf16, strided<[1], offset: ?>, 1>
    %B = memref.reinterpret_cast %b0 to offset: [%ob], sizes: [%big], strides: [1]
        : memref<1xf16, 1> to memref<?xf16, strided<[1], offset: ?>, 1>
    %C = memref.reinterpret_cast %c0 to offset: [%oc], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1], offset: ?>, 1>
    %f0 = arith.constant 0.0 : f32
    %g = gpu.global_id x
    %s = arith.divui %g, %i32 : index
    %lane = gpu.lane_id
    %MN = arith.muli %M, %N : index
    // the same for the whole SIMD-group, which reduces together
    %live = arith.cmpi ult, %s, %MN : index
    scf.if %live {
      %i = arith.divui %s, %N : index
      %j = arith.remui %s, %N : index
      %ia = arith.muli %i, %sar : index
      %jb = arith.muli %j, %sbc : index
      // four steps at a time (four loads of each operand in flight), then one
      %Kx = arith.maxui %K, %i96 : index
      %ub4 = arith.subi %Kx, %i96 : index
      %s4:4 = scf.for %k = %lane to %ub4 step %i128
          iter_args(%c0_ = %f0, %c1_ = %f0, %c2_ = %f0, %c3_ = %f0) -> (f32, f32, f32, f32) {
        %k1 = arith.addi %k, %i32 : index
        %k2 = arith.addi %k1, %i32 : index
        %k3 = arith.addi %k2, %i32 : index
        %ka_d0 = arith.muli %k, %sac : index
        %kb_d0 = arith.muli %k, %sbr : index
        %pa_d0 = arith.addi %ia, %ka_d0 : index
        %pb_d0 = arith.addi %kb_d0, %jb : index
        %xn_d0 = memref.load %A[%pa_d0] : memref<?xf16, strided<[1], offset: ?>, 1>
        %yn_d0 = memref.load %B[%pb_d0] : memref<?xf16, strided<[1], offset: ?>, 1>
        %x_d0 = arith.extf %xn_d0 : f16 to f32
        %y_d0 = arith.extf %yn_d0 : f16 to f32
        %d0 = math.fma %x_d0, %y_d0, %c0_ : f32
        %ka_d1 = arith.muli %k1, %sac : index
        %kb_d1 = arith.muli %k1, %sbr : index
        %pa_d1 = arith.addi %ia, %ka_d1 : index
        %pb_d1 = arith.addi %kb_d1, %jb : index
        %xn_d1 = memref.load %A[%pa_d1] : memref<?xf16, strided<[1], offset: ?>, 1>
        %yn_d1 = memref.load %B[%pb_d1] : memref<?xf16, strided<[1], offset: ?>, 1>
        %x_d1 = arith.extf %xn_d1 : f16 to f32
        %y_d1 = arith.extf %yn_d1 : f16 to f32
        %d1 = math.fma %x_d1, %y_d1, %c1_ : f32
        %ka_d2 = arith.muli %k2, %sac : index
        %kb_d2 = arith.muli %k2, %sbr : index
        %pa_d2 = arith.addi %ia, %ka_d2 : index
        %pb_d2 = arith.addi %kb_d2, %jb : index
        %xn_d2 = memref.load %A[%pa_d2] : memref<?xf16, strided<[1], offset: ?>, 1>
        %yn_d2 = memref.load %B[%pb_d2] : memref<?xf16, strided<[1], offset: ?>, 1>
        %x_d2 = arith.extf %xn_d2 : f16 to f32
        %y_d2 = arith.extf %yn_d2 : f16 to f32
        %d2 = math.fma %x_d2, %y_d2, %c2_ : f32
        %ka_d3 = arith.muli %k3, %sac : index
        %kb_d3 = arith.muli %k3, %sbr : index
        %pa_d3 = arith.addi %ia, %ka_d3 : index
        %pb_d3 = arith.addi %kb_d3, %jb : index
        %xn_d3 = memref.load %A[%pa_d3] : memref<?xf16, strided<[1], offset: ?>, 1>
        %yn_d3 = memref.load %B[%pb_d3] : memref<?xf16, strided<[1], offset: ?>, 1>
        %x_d3 = arith.extf %xn_d3 : f16 to f32
        %y_d3 = arith.extf %yn_d3 : f16 to f32
        %d3 = math.fma %x_d3, %y_d3, %c3_ : f32
        scf.yield %d0, %d1, %d2, %d3 : f32, f32, f32, f32
      }
      // the rest from the first k the loop above did not reach
      %after = arith.cmpi ugt, %ub4, %lane : index
      %span = arith.subi %ub4, %lane : index
      %span1 = arith.addi %span, %i127 : index
      %trips = arith.divui %span1, %i128 : index
      %done = arith.muli %trips, %i128 : index
      %krest0 = arith.addi %lane, %done : index
      %krest = arith.select %after, %krest0, %lane : index
      %s01 = arith.addf %s4#0, %s4#1 : f32
      %s23 = arith.addf %s4#2, %s4#3 : f32
      %sum0 = arith.addf %s01, %s23 : f32
      %part = scf.for %k = %krest to %K step %i32 iter_args(%a = %sum0) -> f32 {
        %ka_a1 = arith.muli %k, %sac : index
        %kb_a1 = arith.muli %k, %sbr : index
        %pa_a1 = arith.addi %ia, %ka_a1 : index
        %pb_a1 = arith.addi %kb_a1, %jb : index
        %xn_a1 = memref.load %A[%pa_a1] : memref<?xf16, strided<[1], offset: ?>, 1>
        %yn_a1 = memref.load %B[%pb_a1] : memref<?xf16, strided<[1], offset: ?>, 1>
        %x_a1 = arith.extf %xn_a1 : f16 to f32
        %y_a1 = arith.extf %yn_a1 : f16 to f32
        %a1 = math.fma %x_a1, %y_a1, %a : f32
        scf.yield %a1 : f32
      }
      %sum = gpu.subgroup_reduce add %part : (f32) -> f32
      %first = arith.cmpi eq, %lane, %i0 : index
      scf.if %first {
        %ic = arith.muli %i, %scr : index
        %jc = arith.muli %j, %scc : index
        %pc = arith.addi %ic, %jc : index
        %scaled = arith.mulf %sum, %alpha : f32
        %no_c = arith.cmpf oeq, %beta, %f0 : f32
        %out = scf.if %no_c -> f32 {
          scf.yield %scaled : f32
        } else {
          %old = memref.load %C[%pc] : memref<?xf32, strided<[1], offset: ?>, 1>
          %r = math.fma %old, %beta, %scaled : f32
          scf.yield %r : f32
        }
        memref.store %out, %C[%pc] : memref<?xf32, strided<[1], offset: ?>, 1>
      }
    }
    gpu.return
  }
}
