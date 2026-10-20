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
    %base = arith.index_cast %base_ : i64 to index
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
      %lo_b = arith.index_cast %lo_r : i32 to index
      %lo = arith.subi %lo_b, %base : index
      %i_1 = arith.addi %i, %i1 : index
      %hi_r = memref.load %rowptr[%i_1] : memref<?xi32, strided<[1]>, 1>
      %hi_b = arith.index_cast %hi_r : i32 to index
      %hi = arith.subi %hi_b, %base : index
      %acc = scf.for %k = %lo to %hi step %i1 iter_args(%a = %f0) -> f32 {
        %c_r = memref.load %col[%k] : memref<?xi32, strided<[1]>, 1>
        %c_b = arith.index_cast %c_r : i32 to index
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
      %c_b = arith.index_cast %c_r : i32 to index
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
  gpu.func @agx_csr_spmv_t_i32(%rowptr0: memref<1xi32, 1>, %col0: memref<1xi32, 1>,
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
      %lo_b = arith.index_cast %lo_r : i32 to index
      %lo = arith.subi %lo_b, %base : index
      %i_1 = arith.addi %i, %i1 : index
      %hi_r = memref.load %rowptr[%i_1] : memref<?xi32, strided<[1]>, 1>
      %hi_b = arith.index_cast %hi_r : i32 to index
      %hi = arith.subi %hi_b, %base : index
      %xi = memref.load %x[%i] : memref<?xf32, strided<[1]>, 1>
      %axi = arith.mulf %xi, %alpha : f32
      scf.for %k = %lo to %hi step %i1 {
        %c_r = memref.load %col[%k] : memref<?xi32, strided<[1]>, 1>
        %c_b = arith.index_cast %c_r : i32 to index
        %c = arith.subi %c_b, %base : index
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
}
