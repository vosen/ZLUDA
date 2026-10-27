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
      %ldb: index, %ldc: index, %alpha: f32, %beta: f32) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %i2 = arith.constant 2 : index
    %i8 = arith.constant 8 : index
    %i16 = arith.constant 16 : index
    %i24 = arith.constant 24 : index
    %i32 = arith.constant 32 : index
    %i64 = arith.constant 64 : index
    %i128 = arith.constant 128 : index
    %A = memref.reinterpret_cast %a0 to offset: [0], sizes: [%M, %K], strides: [%lda, 1]
        : memref<1xf16, 1> to memref<?x?xf16, strided<[?, 1]>, 1>
    %B = memref.reinterpret_cast %b0 to offset: [0], sizes: [%K, %N], strides: [%ldb, 1]
        : memref<1xf16, 1> to memref<?x?xf16, strided<[?, 1]>, 1>
    %C = memref.reinterpret_cast %c0 to offset: [0], sizes: [%M, %N], strides: [%ldc, 1]
        : memref<1xf16, 1> to memref<?x?xf16, strided<[?, 1]>, 1>

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
        %ma0 = air.simdgroup_matrix_load %A[%row, %k] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
        %ma1 = air.simdgroup_matrix_load %A[%row1, %k] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
        %ma2 = air.simdgroup_matrix_load %A[%row2, %k] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
        %ma3 = air.simdgroup_matrix_load %A[%row3, %k] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
        %mb0 = air.simdgroup_matrix_load %B[%k, %col] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
        %mb1 = air.simdgroup_matrix_load %B[%k, %col1] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
        %mb2 = air.simdgroup_matrix_load %B[%k, %col2] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
        %mb3 = air.simdgroup_matrix_load %B[%k, %col3] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
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

    // C = alpha * acc + beta * C, C not read when beta is 0.
    %no_c = arith.cmpf oeq, %beta, %fz : f32
    %malpha = air.simdgroup_matrix_fill %alpha : vector<64xf32>
    %mbeta = air.simdgroup_matrix_fill %beta : vector<64xf32>
    %s_00 = arith.mulf %acc#0, %malpha : vector<64xf32>
    %out_00 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_00 : vector<64xf32>
    } else {
      %oh_00 = air.simdgroup_matrix_load %C[%row, %col] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
      %old_00 = arith.extf %oh_00 : vector<64xf16> to vector<64xf32>
      %r_00 = math.fma %old_00, %mbeta, %s_00 : vector<64xf32>
      scf.yield %r_00 : vector<64xf32>
    }
    %oh16_00 = arith.truncf %out_00 : vector<64xf32> to vector<64xf16>
    air.simdgroup_matrix_store %oh16_00, %C[%row, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1]>, 1>
    %s_01 = arith.mulf %acc#1, %malpha : vector<64xf32>
    %out_01 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_01 : vector<64xf32>
    } else {
      %oh_01 = air.simdgroup_matrix_load %C[%row, %col1] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
      %old_01 = arith.extf %oh_01 : vector<64xf16> to vector<64xf32>
      %r_01 = math.fma %old_01, %mbeta, %s_01 : vector<64xf32>
      scf.yield %r_01 : vector<64xf32>
    }
    %oh16_01 = arith.truncf %out_01 : vector<64xf32> to vector<64xf16>
    air.simdgroup_matrix_store %oh16_01, %C[%row, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1]>, 1>
    %s_02 = arith.mulf %acc#2, %malpha : vector<64xf32>
    %out_02 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_02 : vector<64xf32>
    } else {
      %oh_02 = air.simdgroup_matrix_load %C[%row, %col2] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
      %old_02 = arith.extf %oh_02 : vector<64xf16> to vector<64xf32>
      %r_02 = math.fma %old_02, %mbeta, %s_02 : vector<64xf32>
      scf.yield %r_02 : vector<64xf32>
    }
    %oh16_02 = arith.truncf %out_02 : vector<64xf32> to vector<64xf16>
    air.simdgroup_matrix_store %oh16_02, %C[%row, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1]>, 1>
    %s_03 = arith.mulf %acc#3, %malpha : vector<64xf32>
    %out_03 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_03 : vector<64xf32>
    } else {
      %oh_03 = air.simdgroup_matrix_load %C[%row, %col3] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
      %old_03 = arith.extf %oh_03 : vector<64xf16> to vector<64xf32>
      %r_03 = math.fma %old_03, %mbeta, %s_03 : vector<64xf32>
      scf.yield %r_03 : vector<64xf32>
    }
    %oh16_03 = arith.truncf %out_03 : vector<64xf32> to vector<64xf16>
    air.simdgroup_matrix_store %oh16_03, %C[%row, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1]>, 1>
    %s_10 = arith.mulf %acc#4, %malpha : vector<64xf32>
    %out_10 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_10 : vector<64xf32>
    } else {
      %oh_10 = air.simdgroup_matrix_load %C[%row1, %col] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
      %old_10 = arith.extf %oh_10 : vector<64xf16> to vector<64xf32>
      %r_10 = math.fma %old_10, %mbeta, %s_10 : vector<64xf32>
      scf.yield %r_10 : vector<64xf32>
    }
    %oh16_10 = arith.truncf %out_10 : vector<64xf32> to vector<64xf16>
    air.simdgroup_matrix_store %oh16_10, %C[%row1, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1]>, 1>
    %s_11 = arith.mulf %acc#5, %malpha : vector<64xf32>
    %out_11 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_11 : vector<64xf32>
    } else {
      %oh_11 = air.simdgroup_matrix_load %C[%row1, %col1] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
      %old_11 = arith.extf %oh_11 : vector<64xf16> to vector<64xf32>
      %r_11 = math.fma %old_11, %mbeta, %s_11 : vector<64xf32>
      scf.yield %r_11 : vector<64xf32>
    }
    %oh16_11 = arith.truncf %out_11 : vector<64xf32> to vector<64xf16>
    air.simdgroup_matrix_store %oh16_11, %C[%row1, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1]>, 1>
    %s_12 = arith.mulf %acc#6, %malpha : vector<64xf32>
    %out_12 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_12 : vector<64xf32>
    } else {
      %oh_12 = air.simdgroup_matrix_load %C[%row1, %col2] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
      %old_12 = arith.extf %oh_12 : vector<64xf16> to vector<64xf32>
      %r_12 = math.fma %old_12, %mbeta, %s_12 : vector<64xf32>
      scf.yield %r_12 : vector<64xf32>
    }
    %oh16_12 = arith.truncf %out_12 : vector<64xf32> to vector<64xf16>
    air.simdgroup_matrix_store %oh16_12, %C[%row1, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1]>, 1>
    %s_13 = arith.mulf %acc#7, %malpha : vector<64xf32>
    %out_13 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_13 : vector<64xf32>
    } else {
      %oh_13 = air.simdgroup_matrix_load %C[%row1, %col3] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
      %old_13 = arith.extf %oh_13 : vector<64xf16> to vector<64xf32>
      %r_13 = math.fma %old_13, %mbeta, %s_13 : vector<64xf32>
      scf.yield %r_13 : vector<64xf32>
    }
    %oh16_13 = arith.truncf %out_13 : vector<64xf32> to vector<64xf16>
    air.simdgroup_matrix_store %oh16_13, %C[%row1, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1]>, 1>
    %s_20 = arith.mulf %acc#8, %malpha : vector<64xf32>
    %out_20 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_20 : vector<64xf32>
    } else {
      %oh_20 = air.simdgroup_matrix_load %C[%row2, %col] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
      %old_20 = arith.extf %oh_20 : vector<64xf16> to vector<64xf32>
      %r_20 = math.fma %old_20, %mbeta, %s_20 : vector<64xf32>
      scf.yield %r_20 : vector<64xf32>
    }
    %oh16_20 = arith.truncf %out_20 : vector<64xf32> to vector<64xf16>
    air.simdgroup_matrix_store %oh16_20, %C[%row2, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1]>, 1>
    %s_21 = arith.mulf %acc#9, %malpha : vector<64xf32>
    %out_21 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_21 : vector<64xf32>
    } else {
      %oh_21 = air.simdgroup_matrix_load %C[%row2, %col1] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
      %old_21 = arith.extf %oh_21 : vector<64xf16> to vector<64xf32>
      %r_21 = math.fma %old_21, %mbeta, %s_21 : vector<64xf32>
      scf.yield %r_21 : vector<64xf32>
    }
    %oh16_21 = arith.truncf %out_21 : vector<64xf32> to vector<64xf16>
    air.simdgroup_matrix_store %oh16_21, %C[%row2, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1]>, 1>
    %s_22 = arith.mulf %acc#10, %malpha : vector<64xf32>
    %out_22 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_22 : vector<64xf32>
    } else {
      %oh_22 = air.simdgroup_matrix_load %C[%row2, %col2] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
      %old_22 = arith.extf %oh_22 : vector<64xf16> to vector<64xf32>
      %r_22 = math.fma %old_22, %mbeta, %s_22 : vector<64xf32>
      scf.yield %r_22 : vector<64xf32>
    }
    %oh16_22 = arith.truncf %out_22 : vector<64xf32> to vector<64xf16>
    air.simdgroup_matrix_store %oh16_22, %C[%row2, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1]>, 1>
    %s_23 = arith.mulf %acc#11, %malpha : vector<64xf32>
    %out_23 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_23 : vector<64xf32>
    } else {
      %oh_23 = air.simdgroup_matrix_load %C[%row2, %col3] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
      %old_23 = arith.extf %oh_23 : vector<64xf16> to vector<64xf32>
      %r_23 = math.fma %old_23, %mbeta, %s_23 : vector<64xf32>
      scf.yield %r_23 : vector<64xf32>
    }
    %oh16_23 = arith.truncf %out_23 : vector<64xf32> to vector<64xf16>
    air.simdgroup_matrix_store %oh16_23, %C[%row2, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1]>, 1>
    %s_30 = arith.mulf %acc#12, %malpha : vector<64xf32>
    %out_30 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_30 : vector<64xf32>
    } else {
      %oh_30 = air.simdgroup_matrix_load %C[%row3, %col] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
      %old_30 = arith.extf %oh_30 : vector<64xf16> to vector<64xf32>
      %r_30 = math.fma %old_30, %mbeta, %s_30 : vector<64xf32>
      scf.yield %r_30 : vector<64xf32>
    }
    %oh16_30 = arith.truncf %out_30 : vector<64xf32> to vector<64xf16>
    air.simdgroup_matrix_store %oh16_30, %C[%row3, %col] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1]>, 1>
    %s_31 = arith.mulf %acc#13, %malpha : vector<64xf32>
    %out_31 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_31 : vector<64xf32>
    } else {
      %oh_31 = air.simdgroup_matrix_load %C[%row3, %col1] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
      %old_31 = arith.extf %oh_31 : vector<64xf16> to vector<64xf32>
      %r_31 = math.fma %old_31, %mbeta, %s_31 : vector<64xf32>
      scf.yield %r_31 : vector<64xf32>
    }
    %oh16_31 = arith.truncf %out_31 : vector<64xf32> to vector<64xf16>
    air.simdgroup_matrix_store %oh16_31, %C[%row3, %col1] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1]>, 1>
    %s_32 = arith.mulf %acc#14, %malpha : vector<64xf32>
    %out_32 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_32 : vector<64xf32>
    } else {
      %oh_32 = air.simdgroup_matrix_load %C[%row3, %col2] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
      %old_32 = arith.extf %oh_32 : vector<64xf16> to vector<64xf32>
      %r_32 = math.fma %old_32, %mbeta, %s_32 : vector<64xf32>
      scf.yield %r_32 : vector<64xf32>
    }
    %oh16_32 = arith.truncf %out_32 : vector<64xf32> to vector<64xf16>
    air.simdgroup_matrix_store %oh16_32, %C[%row3, %col2] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1]>, 1>
    %s_33 = arith.mulf %acc#15, %malpha : vector<64xf32>
    %out_33 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_33 : vector<64xf32>
    } else {
      %oh_33 = air.simdgroup_matrix_load %C[%row3, %col3] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
      %old_33 = arith.extf %oh_33 : vector<64xf16> to vector<64xf32>
      %r_33 = math.fma %old_33, %mbeta, %s_33 : vector<64xf32>
      scf.yield %r_33 : vector<64xf32>
    }
    %oh16_33 = arith.truncf %out_33 : vector<64xf32> to vector<64xf16>
    air.simdgroup_matrix_store %oh16_33, %C[%row3, %col3] : vector<64xf16> -> memref<?x?xf16, strided<[?, 1]>, 1>
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
      %ldb: index, %ldc: index, %alpha: f32, %beta: f32) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %i2 = arith.constant 2 : index
    %i8 = arith.constant 8 : index
    %i16 = arith.constant 16 : index
    %i24 = arith.constant 24 : index
    %i32 = arith.constant 32 : index
    %i64 = arith.constant 64 : index
    %i128 = arith.constant 128 : index
    %A = memref.reinterpret_cast %a0 to offset: [0], sizes: [%M, %K], strides: [%lda, 1]
        : memref<1xf16, 1> to memref<?x?xf16, strided<[?, 1]>, 1>
    %B = memref.reinterpret_cast %b0 to offset: [0], sizes: [%K, %N], strides: [%ldb, 1]
        : memref<1xf16, 1> to memref<?x?xf16, strided<[?, 1]>, 1>
    %C = memref.reinterpret_cast %c0 to offset: [0], sizes: [%M, %N], strides: [%ldc, 1]
        : memref<1xf32, 1> to memref<?x?xf32, strided<[?, 1]>, 1>

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
        %ma0 = air.simdgroup_matrix_load %A[%row, %k] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
        %ma1 = air.simdgroup_matrix_load %A[%row1, %k] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
        %ma2 = air.simdgroup_matrix_load %A[%row2, %k] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
        %ma3 = air.simdgroup_matrix_load %A[%row3, %k] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
        %mb0 = air.simdgroup_matrix_load %B[%k, %col] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
        %mb1 = air.simdgroup_matrix_load %B[%k, %col1] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
        %mb2 = air.simdgroup_matrix_load %B[%k, %col2] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
        %mb3 = air.simdgroup_matrix_load %B[%k, %col3] : memref<?x?xf16, strided<[?, 1]>, 1> -> vector<64xf16>
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

    // C = alpha * acc + beta * C, C not read when beta is 0.
    %no_c = arith.cmpf oeq, %beta, %fz : f32
    %malpha = air.simdgroup_matrix_fill %alpha : vector<64xf32>
    %mbeta = air.simdgroup_matrix_fill %beta : vector<64xf32>
    %s_00 = arith.mulf %acc#0, %malpha : vector<64xf32>
    %out_00 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_00 : vector<64xf32>
    } else {
      %old_00 = air.simdgroup_matrix_load %C[%row, %col] : memref<?x?xf32, strided<[?, 1]>, 1> -> vector<64xf32>
      %r_00 = math.fma %old_00, %mbeta, %s_00 : vector<64xf32>
      scf.yield %r_00 : vector<64xf32>
    }
    air.simdgroup_matrix_store %out_00, %C[%row, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1]>, 1>
    %s_01 = arith.mulf %acc#1, %malpha : vector<64xf32>
    %out_01 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_01 : vector<64xf32>
    } else {
      %old_01 = air.simdgroup_matrix_load %C[%row, %col1] : memref<?x?xf32, strided<[?, 1]>, 1> -> vector<64xf32>
      %r_01 = math.fma %old_01, %mbeta, %s_01 : vector<64xf32>
      scf.yield %r_01 : vector<64xf32>
    }
    air.simdgroup_matrix_store %out_01, %C[%row, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1]>, 1>
    %s_02 = arith.mulf %acc#2, %malpha : vector<64xf32>
    %out_02 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_02 : vector<64xf32>
    } else {
      %old_02 = air.simdgroup_matrix_load %C[%row, %col2] : memref<?x?xf32, strided<[?, 1]>, 1> -> vector<64xf32>
      %r_02 = math.fma %old_02, %mbeta, %s_02 : vector<64xf32>
      scf.yield %r_02 : vector<64xf32>
    }
    air.simdgroup_matrix_store %out_02, %C[%row, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1]>, 1>
    %s_03 = arith.mulf %acc#3, %malpha : vector<64xf32>
    %out_03 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_03 : vector<64xf32>
    } else {
      %old_03 = air.simdgroup_matrix_load %C[%row, %col3] : memref<?x?xf32, strided<[?, 1]>, 1> -> vector<64xf32>
      %r_03 = math.fma %old_03, %mbeta, %s_03 : vector<64xf32>
      scf.yield %r_03 : vector<64xf32>
    }
    air.simdgroup_matrix_store %out_03, %C[%row, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1]>, 1>
    %s_10 = arith.mulf %acc#4, %malpha : vector<64xf32>
    %out_10 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_10 : vector<64xf32>
    } else {
      %old_10 = air.simdgroup_matrix_load %C[%row1, %col] : memref<?x?xf32, strided<[?, 1]>, 1> -> vector<64xf32>
      %r_10 = math.fma %old_10, %mbeta, %s_10 : vector<64xf32>
      scf.yield %r_10 : vector<64xf32>
    }
    air.simdgroup_matrix_store %out_10, %C[%row1, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1]>, 1>
    %s_11 = arith.mulf %acc#5, %malpha : vector<64xf32>
    %out_11 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_11 : vector<64xf32>
    } else {
      %old_11 = air.simdgroup_matrix_load %C[%row1, %col1] : memref<?x?xf32, strided<[?, 1]>, 1> -> vector<64xf32>
      %r_11 = math.fma %old_11, %mbeta, %s_11 : vector<64xf32>
      scf.yield %r_11 : vector<64xf32>
    }
    air.simdgroup_matrix_store %out_11, %C[%row1, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1]>, 1>
    %s_12 = arith.mulf %acc#6, %malpha : vector<64xf32>
    %out_12 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_12 : vector<64xf32>
    } else {
      %old_12 = air.simdgroup_matrix_load %C[%row1, %col2] : memref<?x?xf32, strided<[?, 1]>, 1> -> vector<64xf32>
      %r_12 = math.fma %old_12, %mbeta, %s_12 : vector<64xf32>
      scf.yield %r_12 : vector<64xf32>
    }
    air.simdgroup_matrix_store %out_12, %C[%row1, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1]>, 1>
    %s_13 = arith.mulf %acc#7, %malpha : vector<64xf32>
    %out_13 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_13 : vector<64xf32>
    } else {
      %old_13 = air.simdgroup_matrix_load %C[%row1, %col3] : memref<?x?xf32, strided<[?, 1]>, 1> -> vector<64xf32>
      %r_13 = math.fma %old_13, %mbeta, %s_13 : vector<64xf32>
      scf.yield %r_13 : vector<64xf32>
    }
    air.simdgroup_matrix_store %out_13, %C[%row1, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1]>, 1>
    %s_20 = arith.mulf %acc#8, %malpha : vector<64xf32>
    %out_20 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_20 : vector<64xf32>
    } else {
      %old_20 = air.simdgroup_matrix_load %C[%row2, %col] : memref<?x?xf32, strided<[?, 1]>, 1> -> vector<64xf32>
      %r_20 = math.fma %old_20, %mbeta, %s_20 : vector<64xf32>
      scf.yield %r_20 : vector<64xf32>
    }
    air.simdgroup_matrix_store %out_20, %C[%row2, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1]>, 1>
    %s_21 = arith.mulf %acc#9, %malpha : vector<64xf32>
    %out_21 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_21 : vector<64xf32>
    } else {
      %old_21 = air.simdgroup_matrix_load %C[%row2, %col1] : memref<?x?xf32, strided<[?, 1]>, 1> -> vector<64xf32>
      %r_21 = math.fma %old_21, %mbeta, %s_21 : vector<64xf32>
      scf.yield %r_21 : vector<64xf32>
    }
    air.simdgroup_matrix_store %out_21, %C[%row2, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1]>, 1>
    %s_22 = arith.mulf %acc#10, %malpha : vector<64xf32>
    %out_22 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_22 : vector<64xf32>
    } else {
      %old_22 = air.simdgroup_matrix_load %C[%row2, %col2] : memref<?x?xf32, strided<[?, 1]>, 1> -> vector<64xf32>
      %r_22 = math.fma %old_22, %mbeta, %s_22 : vector<64xf32>
      scf.yield %r_22 : vector<64xf32>
    }
    air.simdgroup_matrix_store %out_22, %C[%row2, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1]>, 1>
    %s_23 = arith.mulf %acc#11, %malpha : vector<64xf32>
    %out_23 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_23 : vector<64xf32>
    } else {
      %old_23 = air.simdgroup_matrix_load %C[%row2, %col3] : memref<?x?xf32, strided<[?, 1]>, 1> -> vector<64xf32>
      %r_23 = math.fma %old_23, %mbeta, %s_23 : vector<64xf32>
      scf.yield %r_23 : vector<64xf32>
    }
    air.simdgroup_matrix_store %out_23, %C[%row2, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1]>, 1>
    %s_30 = arith.mulf %acc#12, %malpha : vector<64xf32>
    %out_30 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_30 : vector<64xf32>
    } else {
      %old_30 = air.simdgroup_matrix_load %C[%row3, %col] : memref<?x?xf32, strided<[?, 1]>, 1> -> vector<64xf32>
      %r_30 = math.fma %old_30, %mbeta, %s_30 : vector<64xf32>
      scf.yield %r_30 : vector<64xf32>
    }
    air.simdgroup_matrix_store %out_30, %C[%row3, %col] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1]>, 1>
    %s_31 = arith.mulf %acc#13, %malpha : vector<64xf32>
    %out_31 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_31 : vector<64xf32>
    } else {
      %old_31 = air.simdgroup_matrix_load %C[%row3, %col1] : memref<?x?xf32, strided<[?, 1]>, 1> -> vector<64xf32>
      %r_31 = math.fma %old_31, %mbeta, %s_31 : vector<64xf32>
      scf.yield %r_31 : vector<64xf32>
    }
    air.simdgroup_matrix_store %out_31, %C[%row3, %col1] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1]>, 1>
    %s_32 = arith.mulf %acc#14, %malpha : vector<64xf32>
    %out_32 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_32 : vector<64xf32>
    } else {
      %old_32 = air.simdgroup_matrix_load %C[%row3, %col2] : memref<?x?xf32, strided<[?, 1]>, 1> -> vector<64xf32>
      %r_32 = math.fma %old_32, %mbeta, %s_32 : vector<64xf32>
      scf.yield %r_32 : vector<64xf32>
    }
    air.simdgroup_matrix_store %out_32, %C[%row3, %col2] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1]>, 1>
    %s_33 = arith.mulf %acc#15, %malpha : vector<64xf32>
    %out_33 = scf.if %no_c -> vector<64xf32> {
      scf.yield %s_33 : vector<64xf32>
    } else {
      %old_33 = air.simdgroup_matrix_load %C[%row3, %col3] : memref<?x?xf32, strided<[?, 1]>, 1> -> vector<64xf32>
      %r_33 = math.fma %old_33, %mbeta, %s_33 : vector<64xf32>
      scf.yield %r_33 : vector<64xf32>
    }
    air.simdgroup_matrix_store %out_33, %C[%row3, %col3] : vector<64xf32> -> memref<?x?xf32, strided<[?, 1]>, 1>
    gpu.return
  }

  // As agx_sgemm_generic, with f16 A and B. C = alpha * A * B + beta * C for M x K, K x N and M x N matrices with
  // arbitrary element strides (row stride, column stride) for each, which
  // covers transposed operands and column-major storage. One thread per
  // element of C: launch an N x M grid (x along columns) in any threadgroups.
  // C is not read when beta is 0.
  gpu.func @agx_hgemm_generic_cf16(%a0: memref<1xf16, 1>, %b0: memref<1xf16, 1>,
                              %c0: memref<1xf16, 1>, %M: index, %N: index,
                              %K: index, %sar: index, %sac: index,
                              %sbr: index, %sbc: index, %scr: index,
                              %scc: index, %alpha: f32, %beta: f32) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %big = arith.constant 4611686018427387904 : index
    %A = memref.reinterpret_cast %a0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf16, 1> to memref<?xf16, strided<[1]>, 1>
    %B = memref.reinterpret_cast %b0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf16, 1> to memref<?xf16, strided<[1]>, 1>
    %C = memref.reinterpret_cast %c0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf16, 1> to memref<?xf16, strided<[1]>, 1>
    %j = gpu.global_id x
    %i = gpu.global_id y
    %in_m = arith.cmpi ult, %i, %M : index
    %in_n = arith.cmpi ult, %j, %N : index
    %in = arith.andi %in_m, %in_n : i1
    scf.if %in {
      %f0 = arith.constant 0.0 : f32
      %ia = arith.muli %i, %sar : index
      %jb = arith.muli %j, %sbc : index
      %sum = scf.for %k = %i0 to %K step %i1 iter_args(%s = %f0) -> f32 {
        %ka = arith.muli %k, %sac : index
        %kb = arith.muli %k, %sbr : index
        %pa = arith.addi %ia, %ka : index
        %pb = arith.addi %kb, %jb : index
        %x16 = memref.load %A[%pa] : memref<?xf16, strided<[1]>, 1>
        %y16 = memref.load %B[%pb] : memref<?xf16, strided<[1]>, 1>
        %x = arith.extf %x16 : f16 to f32
        %y = arith.extf %y16 : f16 to f32
        %s1 = math.fma %x, %y, %s : f32
        scf.yield %s1 : f32
      }
      %ic = arith.muli %i, %scr : index
      %jc = arith.muli %j, %scc : index
      %pc = arith.addi %ic, %jc : index
      %scaled = arith.mulf %sum, %alpha : f32
      %no_c = arith.cmpf oeq, %beta, %f0 : f32
      %out = scf.if %no_c -> f32 {
        scf.yield %scaled : f32
      } else {
        %old16 = memref.load %C[%pc] : memref<?xf16, strided<[1]>, 1>
        %old = arith.extf %old16 : f16 to f32
        %r = math.fma %old, %beta, %scaled : f32
        scf.yield %r : f32
      }
      %out16 = arith.truncf %out : f32 to f16
      memref.store %out16, %C[%pc] : memref<?xf16, strided<[1]>, 1>
    }
    gpu.return
  }
  // As agx_sgemm_generic, with f16 A and B. C = alpha * A * B + beta * C for M x K, K x N and M x N matrices with
  // arbitrary element strides (row stride, column stride) for each, which
  // covers transposed operands and column-major storage. One thread per
  // element of C: launch an N x M grid (x along columns) in any threadgroups.
  // C is not read when beta is 0.
  gpu.func @agx_hgemm_generic_cf32(%a0: memref<1xf16, 1>, %b0: memref<1xf16, 1>,
                              %c0: memref<1xf32, 1>, %M: index, %N: index,
                              %K: index, %sar: index, %sac: index,
                              %sbr: index, %sbc: index, %scr: index,
                              %scc: index, %alpha: f32, %beta: f32) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %big = arith.constant 4611686018427387904 : index
    %A = memref.reinterpret_cast %a0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf16, 1> to memref<?xf16, strided<[1]>, 1>
    %B = memref.reinterpret_cast %b0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf16, 1> to memref<?xf16, strided<[1]>, 1>
    %C = memref.reinterpret_cast %c0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %j = gpu.global_id x
    %i = gpu.global_id y
    %in_m = arith.cmpi ult, %i, %M : index
    %in_n = arith.cmpi ult, %j, %N : index
    %in = arith.andi %in_m, %in_n : i1
    scf.if %in {
      %f0 = arith.constant 0.0 : f32
      %ia = arith.muli %i, %sar : index
      %jb = arith.muli %j, %sbc : index
      %sum = scf.for %k = %i0 to %K step %i1 iter_args(%s = %f0) -> f32 {
        %ka = arith.muli %k, %sac : index
        %kb = arith.muli %k, %sbr : index
        %pa = arith.addi %ia, %ka : index
        %pb = arith.addi %kb, %jb : index
        %x16 = memref.load %A[%pa] : memref<?xf16, strided<[1]>, 1>
        %y16 = memref.load %B[%pb] : memref<?xf16, strided<[1]>, 1>
        %x = arith.extf %x16 : f16 to f32
        %y = arith.extf %y16 : f16 to f32
        %s1 = math.fma %x, %y, %s : f32
        scf.yield %s1 : f32
      }
      %ic = arith.muli %i, %scr : index
      %jc = arith.muli %j, %scc : index
      %pc = arith.addi %ic, %jc : index
      %scaled = arith.mulf %sum, %alpha : f32
      %no_c = arith.cmpf oeq, %beta, %f0 : f32
      %out = scf.if %no_c -> f32 {
        scf.yield %scaled : f32
      } else {
        %old = memref.load %C[%pc] : memref<?xf32, strided<[1]>, 1>
        %r = math.fma %old, %beta, %scaled : f32
        scf.yield %r : f32
      }
      memref.store %out, %C[%pc] : memref<?xf32, strided<[1]>, 1>
    }
    gpu.return
  }
}
