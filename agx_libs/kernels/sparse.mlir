// Sparse kernels of zluda_sparse_agx (cuSPARSE generic API), f32 values with
// 32- or 64-bit indices, for every AGX generation like sgemm.mlir.

gpu.module @agx_sparse {
  // y[i * stride] *= beta for i < n (0 when beta is 0, whatever y held).
  gpu.func @agx_sparse_scale(%y0: memref<1xf32, 1>, %n_: i64, %stride_: i64,
                             %beta: f32) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %big = arith.constant 4611686018427387904 : index
    %n = arith.index_cast %n_ : i64 to index
    %stride = arith.index_cast %stride_ : i64 to index
    %y = memref.reinterpret_cast %y0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %i = gpu.global_id x
    %live = arith.cmpi ult, %i, %n : index
    scf.if %live {
      %o = arith.muli %i, %stride : index
      %zero = arith.cmpf oeq, %beta, %f0 : f32
      %v = scf.if %zero -> f32 {
        scf.yield %f0 : f32
      } else {
        %old = memref.load %y[%o] : memref<?xf32, strided<[1]>, 1>
        %s = arith.mulf %old, %beta : f32
        scf.yield %s : f32
      }
      memref.store %v, %y[%o] : memref<?xf32, strided<[1]>, 1>
    }
    gpu.return
  }
  // y[i] = alpha * sum over k in row i of val[k] x[col[k] - base] + beta * y[i]
  // for a rows-row CSR matrix with i32 offsets and columns (indices from
  // base). y is not read when beta is 0. One thread per row.
  gpu.func @agx_csr_spmv_i32(%rowptr0: memref<1xi32, 1>, %col0: memref<1xi32, 1>,
                             %val0: memref<1xf32, 1>, %x0: memref<1xf32, 1>,
                             %y0: memref<1xf32, 1>, %rows_: i64, %base_: i64,
                             %alpha: f32, %beta: f32) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %big = arith.constant 4611686018427387904 : index
    %rows = arith.index_cast %rows_ : i64 to index
    // Column indices are int32 and at least the base: the difference fits.
    %base32 = arith.trunci %base_ : i64 to i32
    %rowptr = memref.reinterpret_cast %rowptr0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xi32, 1> to memref<?xi32, strided<[1]>, 1>
    %col = memref.reinterpret_cast %col0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xi32, 1> to memref<?xi32, strided<[1]>, 1>
    %val = memref.reinterpret_cast %val0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %x = memref.reinterpret_cast %x0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %y = memref.reinterpret_cast %y0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %i = gpu.global_id x
    %live = arith.cmpi ult, %i, %rows : index
    scf.if %live {
      %lo_r = memref.load %rowptr[%i] : memref<?xi32, strided<[1]>, 1>
      %i_1 = arith.addi %i, %i1 : index
      %hi_r = memref.load %rowptr[%i_1] : memref<?xi32, strided<[1]>, 1>
      // Row pointers are int32: so is the loop over the row.
      %one32 = arith.constant 1 : i32
      %lo32 = arith.subi %lo_r, %base32 : i32
      %hi32 = arith.subi %hi_r, %base32 : i32
      %acc = scf.for %k32 = %lo32 to %hi32 step %one32 iter_args(%a = %f0) -> f32 : i32 {
        %k = arith.index_cast %k32 : i32 to index
        %c_r = memref.load %col[%k] : memref<?xi32, strided<[1]>, 1>
        %c_s = arith.subi %c_r, %base32 : i32
        %c = arith.index_cast %c_s : i32 to index
        %v = memref.load %val[%k] : memref<?xf32, strided<[1]>, 1>
        %xv = memref.load %x[%c] : memref<?xf32, strided<[1]>, 1>
        %a1 = math.fma %v, %xv, %a : f32
        scf.yield %a1 : f32
      }
      %scaled = arith.mulf %acc, %alpha : f32
      %no_old = arith.cmpf oeq, %beta, %f0 : f32
      %res = scf.if %no_old -> f32 {
        scf.yield %scaled : f32
      } else {
        %old = memref.load %y[%i] : memref<?xf32, strided<[1]>, 1>
        %ep = math.fma %old, %beta, %scaled : f32
        scf.yield %ep : f32
      }
      memref.store %res, %y[%i] : memref<?xf32, strided<[1]>, 1>
    }
    gpu.return
  }
  // y[dst[k]] += alpha * val[k] x[src[k]] with f32 atomic adds, for COO
  // entries k < nnz (row and column indices, from base): src is the column
  // and dst the row, or the other way round for the transposed product
  // (trans 1). Run agx_sparse_scale (y *= beta) first. One thread per entry.
  gpu.func @agx_coo_spmv_i32(%row0: memref<1xi32, 1>, %col0: memref<1xi32, 1>,
                             %val0: memref<1xf32, 1>, %x0: memref<1xf32, 1>,
                             %y0: memref<1xf32, 1>, %nnz_: i64, %base_: i64,
                             %trans_: i64, %alpha: f32) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %big = arith.constant 4611686018427387904 : index
    %nnz = arith.index_cast %nnz_ : i64 to index
    %base = arith.index_cast %base_ : i64 to index
    // Column indices are int32 and at least the base: the difference fits.
    %base32 = arith.trunci %base_ : i64 to i32
    %trans = arith.index_cast %trans_ : i64 to index
    %row = memref.reinterpret_cast %row0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xi32, 1> to memref<?xi32, strided<[1]>, 1>
    %col = memref.reinterpret_cast %col0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xi32, 1> to memref<?xi32, strided<[1]>, 1>
    %val = memref.reinterpret_cast %val0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %x = memref.reinterpret_cast %x0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %y = memref.reinterpret_cast %y0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %k = gpu.global_id x
    %live = arith.cmpi ult, %k, %nnz : index
    scf.if %live {
      %r_r = memref.load %row[%k] : memref<?xi32, strided<[1]>, 1>
      %r_b = arith.index_cast %r_r : i32 to index
      %r = arith.subi %r_b, %base : index
      %c_r = memref.load %col[%k] : memref<?xi32, strided<[1]>, 1>
      %c_s = arith.subi %c_r, %base32 : i32
      %c = arith.index_cast %c_s : i32 to index
      %is_t = arith.cmpi ne, %trans, %i0 : index
      %src = arith.select %is_t, %r, %c : index
      %dst = arith.select %is_t, %c, %r : index
      %v = memref.load %val[%k] : memref<?xf32, strided<[1]>, 1>
      %xv = memref.load %x[%src] : memref<?xf32, strided<[1]>, 1>
      %p = arith.mulf %v, %xv : f32
      %pa = arith.mulf %p, %alpha : f32
      %old = memref.atomic_rmw addf %pa, %y[%dst] : (f32, memref<?xf32, strided<[1]>, 1>) -> f32
    }
    gpu.return
  }

  // The transposed CSR product: y[col[k]] += alpha * val[k] x[i] for the
  // entries k of row i, with f32 atomic adds. Run agx_sparse_scale first. One
  // thread per row.
  gpu.func @agx_csr_spmv_t_i32(%rowptr0: memref<1xi32, 1>, %col0: memref<1xi32, 1>,
                               %val0: memref<1xf32, 1>, %x0: memref<1xf32, 1>,
                               %y0: memref<1xf32, 1>, %rows_: i64, %base_: i64,
                               %alpha: f32) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %big = arith.constant 4611686018427387904 : index
    %rows = arith.index_cast %rows_ : i64 to index
    // Column indices are int32 and at least the base: the difference fits.
    %base32 = arith.trunci %base_ : i64 to i32
    %rowptr = memref.reinterpret_cast %rowptr0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xi32, 1> to memref<?xi32, strided<[1]>, 1>
    %col = memref.reinterpret_cast %col0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xi32, 1> to memref<?xi32, strided<[1]>, 1>
    %val = memref.reinterpret_cast %val0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %x = memref.reinterpret_cast %x0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %y = memref.reinterpret_cast %y0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %i = gpu.global_id x
    %live = arith.cmpi ult, %i, %rows : index
    scf.if %live {
      %lo_r = memref.load %rowptr[%i] : memref<?xi32, strided<[1]>, 1>
      %i_1 = arith.addi %i, %i1 : index
      %hi_r = memref.load %rowptr[%i_1] : memref<?xi32, strided<[1]>, 1>
      %xi = memref.load %x[%i] : memref<?xf32, strided<[1]>, 1>
      %axi = arith.mulf %xi, %alpha : f32
      // Row pointers are int32: so is the loop over the row.
      %one32 = arith.constant 1 : i32
      %lo32 = arith.subi %lo_r, %base32 : i32
      %hi32 = arith.subi %hi_r, %base32 : i32
      scf.for %k32 = %lo32 to %hi32 step %one32 : i32 {
        %k = arith.index_cast %k32 : i32 to index
        %c_r = memref.load %col[%k] : memref<?xi32, strided<[1]>, 1>
        %c_s = arith.subi %c_r, %base32 : i32
        %c = arith.index_cast %c_s : i32 to index
        %v = memref.load %val[%k] : memref<?xf32, strided<[1]>, 1>
        %p = arith.mulf %v, %axi : f32
        %old = memref.atomic_rmw addf %p, %y[%c] : (f32, memref<?xf32, strided<[1]>, 1>) -> f32
      }
    }
    gpu.return
  }
  // C[i, j] = alpha * sum over k in row i of val[k] B[col[k] - base, j]
  // + beta * C[i, j] for a CSR matrix (i32 indices) and dense B (K x n) and C
  // (rows x n) with element strides (row, column) each. C is not read when
  // beta is 0. One thread per element of C: launch a rows x n grid.
  gpu.func @agx_csr_spmm_i32(%rowptr0: memref<1xi32, 1>, %col0: memref<1xi32, 1>,
                             %val0: memref<1xf32, 1>, %b0: memref<1xf32, 1>,
                             %c0: memref<1xf32, 1>, %rows_: i64, %n_: i64,
                             %base_: i64, %brs_: i64, %bcs_: i64,
                             %crs_: i64, %ccs_: i64, %alpha: f32,
                             %beta: f32) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %big = arith.constant 4611686018427387904 : index
    %rows = arith.index_cast %rows_ : i64 to index
    %n = arith.index_cast %n_ : i64 to index
    %base = arith.index_cast %base_ : i64 to index
    %brs = arith.index_cast %brs_ : i64 to index
    %bcs = arith.index_cast %bcs_ : i64 to index
    %crs = arith.index_cast %crs_ : i64 to index
    %ccs = arith.index_cast %ccs_ : i64 to index
    %rowptr = memref.reinterpret_cast %rowptr0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xi32, 1> to memref<?xi32, strided<[1]>, 1>
    %col = memref.reinterpret_cast %col0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xi32, 1> to memref<?xi32, strided<[1]>, 1>
    %val = memref.reinterpret_cast %val0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %b = memref.reinterpret_cast %b0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %c = memref.reinterpret_cast %c0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %i = gpu.global_id x
    %j = gpu.global_id y
    %in_i = arith.cmpi ult, %i, %rows : index
    %in_j = arith.cmpi ult, %j, %n : index
    %live = arith.andi %in_i, %in_j : i1
    scf.if %live {
      %lo_r = memref.load %rowptr[%i] : memref<?xi32, strided<[1]>, 1>
      %lo_b = arith.index_cast %lo_r : i32 to index
      %lo = arith.subi %lo_b, %base : index
      %i_1 = arith.addi %i, %i1 : index
      %hi_r = memref.load %rowptr[%i_1] : memref<?xi32, strided<[1]>, 1>
      %hi_b = arith.index_cast %hi_r : i32 to index
      %hi = arith.subi %hi_b, %base : index
      %bj = arith.muli %j, %bcs : index
      %acc = scf.for %k = %lo to %hi step %i1 iter_args(%a = %f0) -> f32 {
        %cc_r = memref.load %col[%k] : memref<?xi32, strided<[1]>, 1>
        %cc_b = arith.index_cast %cc_r : i32 to index
        %cc = arith.subi %cc_b, %base : index
        %v = memref.load %val[%k] : memref<?xf32, strided<[1]>, 1>
        %br = arith.muli %cc, %brs : index
        %bo = arith.addi %br, %bj : index
        %bv = memref.load %b[%bo] : memref<?xf32, strided<[1]>, 1>
        %a1 = math.fma %v, %bv, %a : f32
        scf.yield %a1 : f32
      }
      %ci = arith.muli %i, %crs : index
      %cj = arith.muli %j, %ccs : index
      %co = arith.addi %ci, %cj : index
      %scaled = arith.mulf %acc, %alpha : f32
      %no_old = arith.cmpf oeq, %beta, %f0 : f32
      %res = scf.if %no_old -> f32 {
        scf.yield %scaled : f32
      } else {
        %old = memref.load %c[%co] : memref<?xf32, strided<[1]>, 1>
        %ep = math.fma %old, %beta, %scaled : f32
        scf.yield %ep : f32
      }
      memref.store %res, %c[%co] : memref<?xf32, strided<[1]>, 1>
    }
    gpu.return
  }
  // y[i] = alpha * sum over k in row i of val[k] x[col[k] - base] + beta * y[i]
  // for a rows-row CSR matrix with i64 offsets and columns (indices from
  // base). y is not read when beta is 0. One thread per row.
  gpu.func @agx_csr_spmv_i64(%rowptr0: memref<1xi64, 1>, %col0: memref<1xi64, 1>,
                             %val0: memref<1xf32, 1>, %x0: memref<1xf32, 1>,
                             %y0: memref<1xf32, 1>, %rows_: i64, %base_: i64,
                             %alpha: f32, %beta: f32) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %big = arith.constant 4611686018427387904 : index
    %rows = arith.index_cast %rows_ : i64 to index
    %base = arith.index_cast %base_ : i64 to index
    %rowptr = memref.reinterpret_cast %rowptr0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xi64, 1> to memref<?xi64, strided<[1]>, 1>
    %col = memref.reinterpret_cast %col0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xi64, 1> to memref<?xi64, strided<[1]>, 1>
    %val = memref.reinterpret_cast %val0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %x = memref.reinterpret_cast %x0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %y = memref.reinterpret_cast %y0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %i = gpu.global_id x
    %live = arith.cmpi ult, %i, %rows : index
    scf.if %live {
      %lo_r = memref.load %rowptr[%i] : memref<?xi64, strided<[1]>, 1>
      %lo_b = arith.index_cast %lo_r : i64 to index
      %lo = arith.subi %lo_b, %base : index
      %i_1 = arith.addi %i, %i1 : index
      %hi_r = memref.load %rowptr[%i_1] : memref<?xi64, strided<[1]>, 1>
      %hi_b = arith.index_cast %hi_r : i64 to index
      %hi = arith.subi %hi_b, %base : index
      %acc = scf.for %k = %lo to %hi step %i1 iter_args(%a = %f0) -> f32 {
        %c_r = memref.load %col[%k] : memref<?xi64, strided<[1]>, 1>
        %c_b = arith.index_cast %c_r : i64 to index
        %c = arith.subi %c_b, %base : index
        %v = memref.load %val[%k] : memref<?xf32, strided<[1]>, 1>
        %xv = memref.load %x[%c] : memref<?xf32, strided<[1]>, 1>
        %a1 = math.fma %v, %xv, %a : f32
        scf.yield %a1 : f32
      }
      %scaled = arith.mulf %acc, %alpha : f32
      %no_old = arith.cmpf oeq, %beta, %f0 : f32
      %res = scf.if %no_old -> f32 {
        scf.yield %scaled : f32
      } else {
        %old = memref.load %y[%i] : memref<?xf32, strided<[1]>, 1>
        %ep = math.fma %old, %beta, %scaled : f32
        scf.yield %ep : f32
      }
      memref.store %res, %y[%i] : memref<?xf32, strided<[1]>, 1>
    }
    gpu.return
  }
  // y[dst[k]] += alpha * val[k] x[src[k]] with f32 atomic adds, for COO
  // entries k < nnz (row and column indices, from base): src is the column
  // and dst the row, or the other way round for the transposed product
  // (trans 1). Run agx_sparse_scale (y *= beta) first. One thread per entry.
  gpu.func @agx_coo_spmv_i64(%row0: memref<1xi64, 1>, %col0: memref<1xi64, 1>,
                             %val0: memref<1xf32, 1>, %x0: memref<1xf32, 1>,
                             %y0: memref<1xf32, 1>, %nnz_: i64, %base_: i64,
                             %trans_: i64, %alpha: f32) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %big = arith.constant 4611686018427387904 : index
    %nnz = arith.index_cast %nnz_ : i64 to index
    %base = arith.index_cast %base_ : i64 to index
    %trans = arith.index_cast %trans_ : i64 to index
    %row = memref.reinterpret_cast %row0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xi64, 1> to memref<?xi64, strided<[1]>, 1>
    %col = memref.reinterpret_cast %col0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xi64, 1> to memref<?xi64, strided<[1]>, 1>
    %val = memref.reinterpret_cast %val0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %x = memref.reinterpret_cast %x0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %y = memref.reinterpret_cast %y0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %k = gpu.global_id x
    %live = arith.cmpi ult, %k, %nnz : index
    scf.if %live {
      %r_r = memref.load %row[%k] : memref<?xi64, strided<[1]>, 1>
      %r_b = arith.index_cast %r_r : i64 to index
      %r = arith.subi %r_b, %base : index
      %c_r = memref.load %col[%k] : memref<?xi64, strided<[1]>, 1>
      %c_b = arith.index_cast %c_r : i64 to index
      %c = arith.subi %c_b, %base : index
      %is_t = arith.cmpi ne, %trans, %i0 : index
      %src = arith.select %is_t, %r, %c : index
      %dst = arith.select %is_t, %c, %r : index
      %v = memref.load %val[%k] : memref<?xf32, strided<[1]>, 1>
      %xv = memref.load %x[%src] : memref<?xf32, strided<[1]>, 1>
      %p = arith.mulf %v, %xv : f32
      %pa = arith.mulf %p, %alpha : f32
      %old = memref.atomic_rmw addf %pa, %y[%dst] : (f32, memref<?xf32, strided<[1]>, 1>) -> f32
    }
    gpu.return
  }

  // The transposed CSR product: y[col[k]] += alpha * val[k] x[i] for the
  // entries k of row i, with f32 atomic adds. Run agx_sparse_scale first. One
  // thread per row.
  gpu.func @agx_csr_spmv_t_i64(%rowptr0: memref<1xi64, 1>, %col0: memref<1xi64, 1>,
                               %val0: memref<1xf32, 1>, %x0: memref<1xf32, 1>,
                               %y0: memref<1xf32, 1>, %rows_: i64, %base_: i64,
                               %alpha: f32) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %big = arith.constant 4611686018427387904 : index
    %rows = arith.index_cast %rows_ : i64 to index
    %base = arith.index_cast %base_ : i64 to index
    %rowptr = memref.reinterpret_cast %rowptr0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xi64, 1> to memref<?xi64, strided<[1]>, 1>
    %col = memref.reinterpret_cast %col0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xi64, 1> to memref<?xi64, strided<[1]>, 1>
    %val = memref.reinterpret_cast %val0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %x = memref.reinterpret_cast %x0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %y = memref.reinterpret_cast %y0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %i = gpu.global_id x
    %live = arith.cmpi ult, %i, %rows : index
    scf.if %live {
      %lo_r = memref.load %rowptr[%i] : memref<?xi64, strided<[1]>, 1>
      %lo_b = arith.index_cast %lo_r : i64 to index
      %lo = arith.subi %lo_b, %base : index
      %i_1 = arith.addi %i, %i1 : index
      %hi_r = memref.load %rowptr[%i_1] : memref<?xi64, strided<[1]>, 1>
      %hi_b = arith.index_cast %hi_r : i64 to index
      %hi = arith.subi %hi_b, %base : index
      %xi = memref.load %x[%i] : memref<?xf32, strided<[1]>, 1>
      %axi = arith.mulf %xi, %alpha : f32
      scf.for %k = %lo to %hi step %i1 {
        %c_r = memref.load %col[%k] : memref<?xi64, strided<[1]>, 1>
        %c_b = arith.index_cast %c_r : i64 to index
        %c = arith.subi %c_b, %base : index
        %v = memref.load %val[%k] : memref<?xf32, strided<[1]>, 1>
        %p = arith.mulf %v, %axi : f32
        %old = memref.atomic_rmw addf %p, %y[%c] : (f32, memref<?xf32, strided<[1]>, 1>) -> f32
      }
    }
    gpu.return
  }
  // C[i, j] = alpha * sum over k in row i of val[k] B[col[k] - base, j]
  // + beta * C[i, j] for a CSR matrix (i64 indices) and dense B (K x n) and C
  // (rows x n) with element strides (row, column) each. C is not read when
  // beta is 0. One thread per element of C: launch a rows x n grid.
  gpu.func @agx_csr_spmm_i64(%rowptr0: memref<1xi64, 1>, %col0: memref<1xi64, 1>,
                             %val0: memref<1xf32, 1>, %b0: memref<1xf32, 1>,
                             %c0: memref<1xf32, 1>, %rows_: i64, %n_: i64,
                             %base_: i64, %brs_: i64, %bcs_: i64,
                             %crs_: i64, %ccs_: i64, %alpha: f32,
                             %beta: f32) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %big = arith.constant 4611686018427387904 : index
    %rows = arith.index_cast %rows_ : i64 to index
    %n = arith.index_cast %n_ : i64 to index
    %base = arith.index_cast %base_ : i64 to index
    %brs = arith.index_cast %brs_ : i64 to index
    %bcs = arith.index_cast %bcs_ : i64 to index
    %crs = arith.index_cast %crs_ : i64 to index
    %ccs = arith.index_cast %ccs_ : i64 to index
    %rowptr = memref.reinterpret_cast %rowptr0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xi64, 1> to memref<?xi64, strided<[1]>, 1>
    %col = memref.reinterpret_cast %col0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xi64, 1> to memref<?xi64, strided<[1]>, 1>
    %val = memref.reinterpret_cast %val0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %b = memref.reinterpret_cast %b0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %c = memref.reinterpret_cast %c0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %i = gpu.global_id x
    %j = gpu.global_id y
    %in_i = arith.cmpi ult, %i, %rows : index
    %in_j = arith.cmpi ult, %j, %n : index
    %live = arith.andi %in_i, %in_j : i1
    scf.if %live {
      %lo_r = memref.load %rowptr[%i] : memref<?xi64, strided<[1]>, 1>
      %lo_b = arith.index_cast %lo_r : i64 to index
      %lo = arith.subi %lo_b, %base : index
      %i_1 = arith.addi %i, %i1 : index
      %hi_r = memref.load %rowptr[%i_1] : memref<?xi64, strided<[1]>, 1>
      %hi_b = arith.index_cast %hi_r : i64 to index
      %hi = arith.subi %hi_b, %base : index
      %bj = arith.muli %j, %bcs : index
      %acc = scf.for %k = %lo to %hi step %i1 iter_args(%a = %f0) -> f32 {
        %cc_r = memref.load %col[%k] : memref<?xi64, strided<[1]>, 1>
        %cc_b = arith.index_cast %cc_r : i64 to index
        %cc = arith.subi %cc_b, %base : index
        %v = memref.load %val[%k] : memref<?xf32, strided<[1]>, 1>
        %br = arith.muli %cc, %brs : index
        %bo = arith.addi %br, %bj : index
        %bv = memref.load %b[%bo] : memref<?xf32, strided<[1]>, 1>
        %a1 = math.fma %v, %bv, %a : f32
        scf.yield %a1 : f32
      }
      %ci = arith.muli %i, %crs : index
      %cj = arith.muli %j, %ccs : index
      %co = arith.addi %ci, %cj : index
      %scaled = arith.mulf %acc, %alpha : f32
      %no_old = arith.cmpf oeq, %beta, %f0 : f32
      %res = scf.if %no_old -> f32 {
        scf.yield %scaled : f32
      } else {
        %old = memref.load %c[%co] : memref<?xf32, strided<[1]>, 1>
        %ep = math.fma %old, %beta, %scaled : f32
        scf.yield %ep : f32
      }
      memref.store %res, %c[%co] : memref<?xf32, strided<[1]>, 1>
    }
    gpu.return
  }

  // ---- helpers of the kernels below (and of sparse_ix.mlir)

  // m[k] of a buffer of any length.
  func.func private @agx_sp_ld(%m0: memref<1xf32, 1>, %k: index) -> f32 {
    %big = arith.constant 4611686018427387904 : index
    %m = memref.reinterpret_cast %m0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %v = memref.load %m[%k] : memref<?xf32, strided<[1]>, 1>
    return %v : f32
  }
  func.func private @agx_sp_st(%m0: memref<1xf32, 1>, %k: index, %v: f32) {
    %big = arith.constant 4611686018427387904 : index
    %m = memref.reinterpret_cast %m0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    memref.store %v, %m[%k] : memref<?xf32, strided<[1]>, 1>
    return
  }
  // m[k] - base and m[k] = v + base of an i32 index buffer.
  func.func private @agx_sp_ldi(%m0: memref<1xi32, 1>, %k: index, %base: index) -> index {
    %big = arith.constant 4611686018427387904 : index
    %m = memref.reinterpret_cast %m0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xi32, 1> to memref<?xi32, strided<[1]>, 1>
    %r = memref.load %m[%k] : memref<?xi32, strided<[1]>, 1>
    %b = arith.index_cast %r : i32 to index
    %v = arith.subi %b, %base : index
    return %v : index
  }
  func.func private @agx_sp_sti(%m0: memref<1xi32, 1>, %k: index, %v: index, %base: index) {
    %big = arith.constant 4611686018427387904 : index
    %m = memref.reinterpret_cast %m0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xi32, 1> to memref<?xi32, strided<[1]>, 1>
    %b = arith.addi %v, %base : index
    %r = arith.index_cast %b : index to i32
    memref.store %r, %m[%k] : memref<?xi32, strided<[1]>, 1>
    return
  }
  // The old m[k], adding 1 to it atomically.
  func.func private @agx_sp_inc(%m0: memref<1xi32, 1>, %k: index) -> index {
    %big = arith.constant 4611686018427387904 : index
    %one = arith.constant 1 : i32
    %m = memref.reinterpret_cast %m0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xi32, 1> to memref<?xi32, strided<[1]>, 1>
    %old = memref.atomic_rmw addi %one, %m[%k] : (i32, memref<?xi32, strided<[1]>, 1>) -> i32
    %v = arith.index_cast %old : i32 to index
    return %v : index
  }

  // out[0] = part[0] + ... + part[threads - 1], in order. One thread.
  gpu.func @agx_sparse_sum(%part0: memref<1xf32, 1>, %out0: memref<1xf32, 1>, %threads_: i64) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %threads = arith.index_cast %threads_ : i64 to index
    %i = gpu.global_id x
    %first = arith.cmpi eq, %i, %i0 : index
    scf.if %first {
      %s = scf.for %t = %i0 to %threads step %i1 iter_args(%a = %f0) -> f32 {
        %v = func.call @agx_sp_ld(%part0, %t) : (memref<1xf32, 1>, index) -> f32
        %a1 = arith.addf %a, %v : f32
        scf.yield %a1 : f32
      }
      func.call @agx_sp_st(%out0, %i0, %s) : (memref<1xf32, 1>, index, f32) -> ()
    }
    gpu.return
  }

  // D[i rs + j cs] = 0 for i < rows, j < cols: launch a rows x cols grid.
  gpu.func @agx_dense_zero(%d0: memref<1xf32, 1>, %rows_: i64, %cols_: i64, %rs_: i64, %cs_: i64) kernel {
    %f0 = arith.constant 0.0 : f32
    %rows = arith.index_cast %rows_ : i64 to index
    %cols = arith.index_cast %cols_ : i64 to index
    %rs = arith.index_cast %rs_ : i64 to index
    %cs = arith.index_cast %cs_ : i64 to index
    %i = gpu.global_id x
    %j = gpu.global_id y
    %in_i = arith.cmpi ult, %i, %rows : index
    %in_j = arith.cmpi ult, %j, %cols : index
    %live = arith.andi %in_i, %in_j : i1
    scf.if %live {
      %oi = arith.muli %i, %rs : index
      %oj = arith.muli %j, %cs : index
      %o = arith.addi %oi, %oj : index
      func.call @agx_sp_st(%d0, %o, %f0) : (memref<1xf32, 1>, index, f32) -> ()
    }
    gpu.return
  }

  // dst[p words + w] = src[perm[p] words + w] for p < n and w < words: moves
  // values of any size in words of 4 bytes (8 in the f64 copy). One thread
  // per p.
  gpu.func @agx_sparse_permute(%src0: memref<1xf32, 1>, %dst0: memref<1xf32, 1>,
                               %perm0: memref<1xi32, 1>, %n_: i64, %words_: i64) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %n = arith.index_cast %n_ : i64 to index
    %words = arith.index_cast %words_ : i64 to index
    %p = gpu.global_id x
    %live = arith.cmpi ult, %p, %n : index
    scf.if %live {
      %k = func.call @agx_sp_ldi(%perm0, %p, %i0) : (memref<1xi32, 1>, index, index) -> index
      %so = arith.muli %k, %words : index
      %do = arith.muli %p, %words : index
      scf.for %w = %i0 to %words step %i1 {
        %s = arith.addi %so, %w : index
        %d = arith.addi %do, %w : index
        %v = func.call @agx_sp_ld(%src0, %s) : (memref<1xf32, 1>, index) -> f32
        func.call @agx_sp_st(%dst0, %d, %v) : (memref<1xf32, 1>, index, f32) -> ()
      }
    }
    gpu.return
  }

  // CSR to CSC with 32-bit indices (cusparseCsr2cscEx2), in four steps over
  // a zeroed count array cnt of cols entries:
  // 1. cnt[col[k] - base] += 1 for the entries k of row i (one thread per row)
  gpu.func @agx_csr2csc_count(%rowptr0: memref<1xi32, 1>, %col0: memref<1xi32, 1>,
                              %cnt0: memref<1xi32, 1>, %rows_: i64, %base_: i64) kernel {
    %i1 = arith.constant 1 : index
    %rows = arith.index_cast %rows_ : i64 to index
    %base = arith.index_cast %base_ : i64 to index
    %i = gpu.global_id x
    %live = arith.cmpi ult, %i, %rows : index
    scf.if %live {
      %lo = func.call @agx_sp_ldi(%rowptr0, %i, %base) : (memref<1xi32, 1>, index, index) -> index
      %i_1 = arith.addi %i, %i1 : index
      %hi = func.call @agx_sp_ldi(%rowptr0, %i_1, %base) : (memref<1xi32, 1>, index, index) -> index
      scf.for %k = %lo to %hi step %i1 {
        %c = func.call @agx_sp_ldi(%col0, %k, %base) : (memref<1xi32, 1>, index, index) -> index
        %old = func.call @agx_sp_inc(%cnt0, %c) : (memref<1xi32, 1>, index) -> index
      }
    }
    gpu.return
  }
  // 2. the column offsets ptr (from base) and in cnt the first free position
  //    of each column (one thread)
  gpu.func @agx_csr2csc_scan(%cnt0: memref<1xi32, 1>, %ptr0: memref<1xi32, 1>,
                             %cols_: i64, %base_: i64) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %cols = arith.index_cast %cols_ : i64 to index
    %base = arith.index_cast %base_ : i64 to index
    %i = gpu.global_id x
    %first = arith.cmpi eq, %i, %i0 : index
    scf.if %first {
      %run = scf.for %j = %i0 to %cols step %i1 iter_args(%r = %i0) -> index {
        func.call @agx_sp_sti(%ptr0, %j, %r, %base) : (memref<1xi32, 1>, index, index, index) -> ()
        %v = func.call @agx_sp_ldi(%cnt0, %j, %i0) : (memref<1xi32, 1>, index, index) -> index
        func.call @agx_sp_sti(%cnt0, %j, %r, %i0) : (memref<1xi32, 1>, index, index, index) -> ()
        %r1 = arith.addi %r, %v : index
        scf.yield %r1 : index
      }
      func.call @agx_sp_sti(%ptr0, %cols, %run, %base) : (memref<1xi32, 1>, index, index, index) -> ()
    }
    gpu.return
  }
  // 3. for the entries k of row i, at the next free position p of their
  //    column: rowind[p] = i + base, perm[p] = k (one thread per row)
  gpu.func @agx_csr2csc_fill(%rowptr0: memref<1xi32, 1>, %col0: memref<1xi32, 1>,
                             %next0: memref<1xi32, 1>, %rowind0: memref<1xi32, 1>,
                             %perm0: memref<1xi32, 1>, %rows_: i64, %base_: i64) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %rows = arith.index_cast %rows_ : i64 to index
    %base = arith.index_cast %base_ : i64 to index
    %i = gpu.global_id x
    %live = arith.cmpi ult, %i, %rows : index
    scf.if %live {
      %lo = func.call @agx_sp_ldi(%rowptr0, %i, %base) : (memref<1xi32, 1>, index, index) -> index
      %i_1 = arith.addi %i, %i1 : index
      %hi = func.call @agx_sp_ldi(%rowptr0, %i_1, %base) : (memref<1xi32, 1>, index, index) -> index
      scf.for %k = %lo to %hi step %i1 {
        %c = func.call @agx_sp_ldi(%col0, %k, %base) : (memref<1xi32, 1>, index, index) -> index
        %p = func.call @agx_sp_inc(%next0, %c) : (memref<1xi32, 1>, index) -> index
        func.call @agx_sp_sti(%rowind0, %p, %i, %base) : (memref<1xi32, 1>, index, index, index) -> ()
        func.call @agx_sp_sti(%perm0, %p, %k, %i0) : (memref<1xi32, 1>, index, index, index) -> ()
      }
    }
    gpu.return
  }
  // 4. sorts the entries of each column by perm (so by row), moving rowind
  //    along (one thread per column)
  gpu.func @agx_csr2csc_sort(%ptr0: memref<1xi32, 1>, %rowind0: memref<1xi32, 1>,
                             %perm0: memref<1xi32, 1>, %cols_: i64, %base_: i64) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %false = arith.constant false
    %cols = arith.index_cast %cols_ : i64 to index
    %base = arith.index_cast %base_ : i64 to index
    %j = gpu.global_id x
    %live = arith.cmpi ult, %j, %cols : index
    scf.if %live {
      %lo = func.call @agx_sp_ldi(%ptr0, %j, %base) : (memref<1xi32, 1>, index, index) -> index
      %j_1 = arith.addi %j, %i1 : index
      %hi = func.call @agx_sp_ldi(%ptr0, %j_1, %base) : (memref<1xi32, 1>, index, index) -> index
      %lo1 = arith.addi %lo, %i1 : index
      scf.for %p = %lo1 to %hi step %i1 {
        %key = func.call @agx_sp_ldi(%perm0, %p, %i0) : (memref<1xi32, 1>, index, index) -> index
        %row = func.call @agx_sp_ldi(%rowind0, %p, %i0) : (memref<1xi32, 1>, index, index) -> index
        %q = scf.while (%q0 = %p) : (index) -> index {
          %more = arith.cmpi ugt, %q0, %lo : index
          %go = scf.if %more -> i1 {
            %qm = arith.subi %q0, %i1 : index
            %pv = func.call @agx_sp_ldi(%perm0, %qm, %i0) : (memref<1xi32, 1>, index, index) -> index
            %gt = arith.cmpi ugt, %pv, %key : index
            scf.yield %gt : i1
          } else {
            scf.yield %false : i1
          }
          scf.condition(%go) %q0 : index
        } do {
        ^bb0(%q1: index):
          %qm = arith.subi %q1, %i1 : index
          %pv = func.call @agx_sp_ldi(%perm0, %qm, %i0) : (memref<1xi32, 1>, index, index) -> index
          %rv = func.call @agx_sp_ldi(%rowind0, %qm, %i0) : (memref<1xi32, 1>, index, index) -> index
          func.call @agx_sp_sti(%perm0, %q1, %pv, %i0) : (memref<1xi32, 1>, index, index, index) -> ()
          func.call @agx_sp_sti(%rowind0, %q1, %rv, %i0) : (memref<1xi32, 1>, index, index, index) -> ()
          scf.yield %qm : index
        }
        func.call @agx_sp_sti(%perm0, %q, %key, %i0) : (memref<1xi32, 1>, index, index, index) -> ()
        func.call @agx_sp_sti(%rowind0, %q, %row, %i0) : (memref<1xi32, 1>, index, index, index) -> ()
      }
    }
    gpu.return
  }

  // sparse_ix.mlir for 32- and 64-bit indices
}
