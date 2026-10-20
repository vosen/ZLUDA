// Single precision GEMM kernels of zluda_blas_agx, for every AGX generation:
// standard MLIR dialects and agx operations only. The chip comes from the
// #agxir.target the build attaches (agx-build-fatbin.py), which decides how
// agx.mma_tile and agx.block_barrier run.

gpu.module @agx_blas {
  // C = alpha * A * B + beta * C for row-major matrices with leading
  // dimensions lda, ldb and ldc (in elements); M and N multiples of 64, K a
  // multiple of 8, the leading dimensions even. C is not read when beta is 0.
  //
  // Each threadgroup is 2 x 2 SIMD-groups (128 threads) computing a 64 x 64
  // tile; each SIMD-group a 32 x 32 tile, accumulated in registers along K
  // with the operands read straight from device memory. Threadgroups walk the
  // output in groups of 8 tile rows so that concurrently running ones share
  // rows of A and columns of B in the caches, and the SIMD-groups of a
  // threadgroup meet at a barrier every 128 steps of K to keep their working
  // set together. Launch with (M / 64) * (N / 64) threadgroups of 128 threads.
  gpu.func @agx_sgemm_tile64(%a0: memref<1xf32, 1>, %b0: memref<1xf32, 1>,
                             %c0: memref<1xf32, 1>, %M: index, %N: index,
                             %K: index, %lda: index, %ldb: index,
                             %ldc: index, %alpha: f32, %beta: f32) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %i2 = arith.constant 2 : index
    %i4 = arith.constant 4 : index
    %i8 = arith.constant 8 : index
    %i16 = arith.constant 16 : index
    %i32 = arith.constant 32 : index
    %i64 = arith.constant 64 : index
    %i128 = arith.constant 128 : index
    %A = memref.reinterpret_cast %a0 to offset: [0], sizes: [%M, %K], strides: [%lda, 1]
        : memref<1xf32, 1> to memref<?x?xf32, strided<[?, 1]>, 1>
    %B = memref.reinterpret_cast %b0 to offset: [0], sizes: [%K, %N], strides: [%ldb, 1]
        : memref<1xf32, 1> to memref<?x?xf32, strided<[?, 1]>, 1>
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

    // This SIMD-group's 32 x 32 tile.
    %sg = gpu.subgroup_id : index
    %sr = arith.divui %sg, %i2 : index
    %sc = arith.remui %sg, %i2 : index
    %tm64 = arith.muli %tm, %i64 : index
    %tn64 = arith.muli %tn, %i64 : index
    %sr32 = arith.muli %sr, %i32 : index
    %sc32 = arith.muli %sc, %i32 : index
    %row = arith.addi %tm64, %sr32 : index
    %col = arith.addi %tn64, %sc32 : index

    %zero = arith.constant dense<0.0> : vector<32xf32>
    %acc = scf.for %kk = %i0 to %K step %i128 iter_args(%acc_o = %zero) -> vector<32xf32> {
      agx.block_barrier
      %kk_end0 = arith.addi %kk, %i128 : index
      %kk_end = arith.minui %kk_end0, %K : index
      %acc_i = scf.for %k = %kk to %kk_end step %i8 iter_args(%acc_k = %acc_o) -> vector<32xf32> {
        %next = agx.mma_tile %A[%row, %k] * %B[%k, %col] + %acc_k tile(32, 32, 8)
            : memref<?x?xf32, strided<[?, 1]>, 1>, memref<?x?xf32, strided<[?, 1]>, 1>, vector<32xf32>
        scf.yield %next : vector<32xf32>
      }
      scf.yield %acc_i : vector<32xf32>
    }

    // The 16 fragments of the tile, row-major, two elements per thread.
    %f0 = arith.constant 0.0 : f32
    %no_c = arith.cmpf oeq, %beta, %f0 : f32
    %valpha = vector.broadcast %alpha : f32 to vector<2xf32>
    %vbeta = vector.broadcast %beta : f32 to vector<2xf32>
    scf.for %f = %i0 to %i16 step %i1 {
      %f_r = arith.divui %f, %i4 : index
      %f_c = arith.remui %f, %i4 : index
      %r_o = arith.muli %f_r, %i8 : index
      %c_o = arith.muli %f_c, %i8 : index
      %r = arith.addi %row, %r_o : index
      %c = arith.addi %col, %c_o : index
      %e0 = arith.muli %f, %i2 : index
      %e1 = arith.addi %e0, %i1 : index
      %x0 = vector.extract %acc[%e0] : f32 from vector<32xf32>
      %x1 = vector.extract %acc[%e1] : f32 from vector<32xf32>
      %frag = vector.from_elements %x0, %x1 : vector<2xf32>
      %scaled = arith.mulf %frag, %valpha : vector<2xf32>
      %out = scf.if %no_c -> vector<2xf32> {
        scf.yield %scaled : vector<2xf32>
      } else {
        %old = agx.matrix_load %C[%r, %c] : memref<?x?xf32, strided<[?, 1]>, 1> -> vector<2xf32>
        %sum = math.fma %old, %vbeta, %scaled : vector<2xf32>
        scf.yield %sum : vector<2xf32>
      }
      agx.matrix_store %out, %C[%r, %c] : vector<2xf32> -> memref<?x?xf32, strided<[?, 1]>, 1>
    }
    gpu.return
  }

  // C = alpha * A * B + beta * C for M x K, K x N and M x N matrices with
  // arbitrary element strides (row stride, column stride) for each, which
  // covers transposed operands and column-major storage. One thread per
  // element of C: launch an N x M grid (x along columns) in any threadgroups.
  // C is not read when beta is 0.
  gpu.func @agx_sgemm_generic(%a0: memref<1xf32, 1>, %b0: memref<1xf32, 1>,
                              %c0: memref<1xf32, 1>, %M: index, %N: index,
                              %K: index, %sar: index, %sac: index,
                              %sbr: index, %sbc: index, %scr: index,
                              %scc: index, %alpha: f32, %beta: f32) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %big = arith.constant 4611686018427387904 : index
    %A = memref.reinterpret_cast %a0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %B = memref.reinterpret_cast %b0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
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
        %x = memref.load %A[%pa] : memref<?xf32, strided<[1]>, 1>
        %y = memref.load %B[%pb] : memref<?xf32, strided<[1]>, 1>
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

  // agx_sgemm_generic in double precision (cublasDgemm; f64 is software arithmetic on AGX).
  // C = alpha * A * B + beta * C for M x K, K x N and M x N matrices with
  // arbitrary element strides (row stride, column stride) for each, which
  // covers transposed operands and column-major storage. One thread per
  // element of C: launch an N x M grid (x along columns) in any threadgroups.
  // C is not read when beta is 0.
  gpu.func @agx_dgemm_generic(%a0: memref<1xf64, 1>, %b0: memref<1xf64, 1>,
                              %c0: memref<1xf64, 1>, %M: index, %N: index,
                              %K: index, %sar: index, %sac: index,
                              %sbr: index, %sbc: index, %scr: index,
                              %scc: index, %alpha: f64, %beta: f64) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %big = arith.constant 4611686018427387904 : index
    %A = memref.reinterpret_cast %a0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf64, 1> to memref<?xf64, strided<[1]>, 1>
    %B = memref.reinterpret_cast %b0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf64, 1> to memref<?xf64, strided<[1]>, 1>
    %C = memref.reinterpret_cast %c0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf64, 1> to memref<?xf64, strided<[1]>, 1>
    %j = gpu.global_id x
    %i = gpu.global_id y
    %in_m = arith.cmpi ult, %i, %M : index
    %in_n = arith.cmpi ult, %j, %N : index
    %in = arith.andi %in_m, %in_n : i1
    scf.if %in {
      %f0 = arith.constant 0.0 : f64
      %ia = arith.muli %i, %sar : index
      %jb = arith.muli %j, %sbc : index
      %sum = scf.for %k = %i0 to %K step %i1 iter_args(%s = %f0) -> f64 {
        %ka = arith.muli %k, %sac : index
        %kb = arith.muli %k, %sbr : index
        %pa = arith.addi %ia, %ka : index
        %pb = arith.addi %kb, %jb : index
        %x = memref.load %A[%pa] : memref<?xf64, strided<[1]>, 1>
        %y = memref.load %B[%pb] : memref<?xf64, strided<[1]>, 1>
        %s1 = math.fma %x, %y, %s : f64
        scf.yield %s1 : f64
      }
      %ic = arith.muli %i, %scr : index
      %jc = arith.muli %j, %scc : index
      %pc = arith.addi %ic, %jc : index
      %scaled = arith.mulf %sum, %alpha : f64
      %no_c = arith.cmpf oeq, %beta, %f0 : f64
      %out = scf.if %no_c -> f64 {
        scf.yield %scaled : f64
      } else {
        %old = memref.load %C[%pc] : memref<?xf64, strided<[1]>, 1>
        %r = math.fma %old, %beta, %scaled : f64
        scf.yield %r : f64
      }
      memref.store %out, %C[%pc] : memref<?xf64, strided<[1]>, 1>
    }
    gpu.return
  }
}
