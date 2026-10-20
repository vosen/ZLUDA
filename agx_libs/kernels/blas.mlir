// Level 1 and 2 BLAS kernels of zluda_blas_agx in f32, for every AGX
// generation like sgemm.mlir. Vectors follow BLAS conventions: the pointer is
// the lowest address and element i of an n-vector with increment inc is at
// i * inc, or at (n - 1 - i) * -inc when inc is negative. Matrices are
// column-major. Sizes and increments are i64.

gpu.module @agx_blas {
  // y = alpha x + y (op 0), x = alpha x (op 1, y unused), y = x (op 2),
  // swap x and y (op 3). One thread per element.
  gpu.func @agx_blas_vec(%x0: memref<1xf32, 1>, %y0: memref<1xf32, 1>,
                         %n_: i64, %incx_: i64, %incy_: i64, %op_: i64,
                         %alpha: f32) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %big = arith.constant 4611686018427387904 : index
    %n = arith.index_cast %n_ : i64 to index
    %op = arith.index_cast %op_ : i64 to index
    %x = memref.reinterpret_cast %x0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %y = memref.reinterpret_cast %y0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %i = gpu.global_id x
    %live = arith.cmpi ult, %i, %n : index
    scf.if %live {
      %ix = func.call @agx_blas_pos(%i, %n, %incx_) : (index, index, i64) -> index
      %vx = memref.load %x[%ix] : memref<?xf32, strided<[1]>, 1>
      %is_scal = arith.cmpi eq, %op, %i1 : index
      scf.if %is_scal {
        %s = arith.mulf %vx, %alpha : f32
        memref.store %s, %x[%ix] : memref<?xf32, strided<[1]>, 1>
      } else {
        %iy = func.call @agx_blas_pos(%i, %n, %incy_) : (index, index, i64) -> index
        %is_axpy = arith.cmpi eq, %op, %i0 : index
        %vy = memref.load %y[%iy] : memref<?xf32, strided<[1]>, 1>
        %ax = math.fma %vx, %alpha, %vy : f32
        %out = arith.select %is_axpy, %ax, %vx : f32
        memref.store %out, %y[%iy] : memref<?xf32, strided<[1]>, 1>
        %i3 = arith.constant 3 : index
        %is_swap = arith.cmpi eq, %op, %i3 : index
        scf.if %is_swap {
          memref.store %vy, %x[%ix] : memref<?xf32, strided<[1]>, 1>
        }
      }
    }
    gpu.return
  }

  func.func private @agx_blas_pos(%i: index, %n: index, %inc_: i64) -> index {
    %z = arith.constant 0 : i64
    %i1 = arith.constant 1 : index
    %neg = arith.cmpi slt, %inc_, %z : i64
    %ninc_ = arith.subi %z, %inc_ : i64
    %inc = arith.index_cast %inc_ : i64 to index
    %ninc = arith.index_cast %ninc_ : i64 to index
    %last = arith.subi %n, %i1 : index
    %r = arith.subi %last, %i : index
    %pn = arith.muli %r, %ninc : index
    %pp = arith.muli %i, %inc : index
    %p = arith.select %neg, %pn, %pp : index
    return %p : index
  }

  // Partial reductions: thread t < threads folds the elements t, t + threads,
  // ... into part[2 t] and part[2 t + 1]:
  //   op 0, dot:  (sum x y, 0)
  //   op 1, asum: (sum |x|, 0)
  //   op 2, nrm2: (scale, ssq) with sum x^2 = scale^2 ssq (LAPACK's slassq)
  //   op 3, amax / op 4, amin: (|x| of the first extreme, its index as i32 bits)
  gpu.func @agx_blas_reduce(%x0: memref<1xf32, 1>, %y0: memref<1xf32, 1>,
                            %part0: memref<1xf32, 1>, %n_: i64, %incx_: i64,
                            %incy_: i64, %op_: i64, %threads_: i64) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %i2 = arith.constant 2 : index
    %i3 = arith.constant 3 : index
    %f0 = arith.constant 0.0 : f32
    %f1 = arith.constant 1.0 : f32
    %big = arith.constant 4611686018427387904 : index
    %n = arith.index_cast %n_ : i64 to index
    %op = arith.index_cast %op_ : i64 to index
    %threads = arith.index_cast %threads_ : i64 to index
    %x = memref.reinterpret_cast %x0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %y = memref.reinterpret_cast %y0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %part = memref.reinterpret_cast %part0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %t = gpu.global_id x
    %live = arith.cmpi ult, %t, %threads : index
    scf.if %live {
      %is_dot = arith.cmpi eq, %op, %i0 : index
      %is_nrm = arith.cmpi eq, %op, %i2 : index
      %is_ext = arith.cmpi uge, %op, %i3 : index
      %is_max = arith.cmpi eq, %op, %i3 : index
      %none = arith.constant -1.0 : f32
      %a0 = arith.select %is_ext, %none, %f0 : f32
      %b0 = arith.select %is_nrm, %f1, %f0 : f32
      %res:2 = scf.for %k = %t to %n step %threads iter_args(%a = %a0, %b = %b0) -> (f32, f32) {
        %ix = func.call @agx_blas_pos(%k, %n, %incx_) : (index, index, i64) -> index
        %vx = memref.load %x[%ix] : memref<?xf32, strided<[1]>, 1>
        %ax = math.absf %vx : f32
        %r:2 = scf.if %is_dot -> (f32, f32) {
          %iy = func.call @agx_blas_pos(%k, %n, %incy_) : (index, index, i64) -> index
          %vy = memref.load %y[%iy] : memref<?xf32, strided<[1]>, 1>
          %s = math.fma %vx, %vy, %a : f32
          scf.yield %s, %b : f32, f32
        } else {
          %r2:2 = scf.if %is_nrm -> (f32, f32) {
            %r3:2 = func.call @agx_blas_ssq(%a, %b, %ax) : (f32, f32, f32) -> (f32, f32)
            scf.yield %r3#0, %r3#1 : f32, f32
          } else {
            %r4:2 = scf.if %is_ext -> (f32, f32) {
              // strictly better, so the first (smallest) index wins
              %gt = arith.cmpf ogt, %ax, %a : f32
              %lt = arith.cmpf olt, %ax, %a : f32
              %first = arith.cmpf olt, %a, %f0 : f32
              %better0 = arith.select %is_max, %gt, %lt : i1
              %better = arith.ori %better0, %first : i1
              %k64 = arith.index_cast %k : index to i32
              %kf = arith.bitcast %k64 : i32 to f32
              %na = arith.select %better, %ax, %a : f32
              %nb = arith.select %better, %kf, %b : f32
              scf.yield %na, %nb : f32, f32
            } else {
              %s = arith.addf %a, %ax : f32
              scf.yield %s, %b : f32, f32
            }
            scf.yield %r4#0, %r4#1 : f32, f32
          }
          scf.yield %r2#0, %r2#1 : f32, f32
        }
        scf.yield %r#0, %r#1 : f32, f32
      }
      %p0 = arith.muli %t, %i2 : index
      %p1 = arith.addi %p0, %i1 : index
      memref.store %res#0, %part[%p0] : memref<?xf32, strided<[1]>, 1>
      memref.store %res#1, %part[%p1] : memref<?xf32, strided<[1]>, 1>
    }
    gpu.return
  }

  // Adds |v| to (scale, ssq) with sum x^2 = scale^2 ssq.
  func.func private @agx_blas_ssq(%scale: f32, %ssq: f32, %v: f32) -> (f32, f32) {
    %f0 = arith.constant 0.0 : f32
    %f1 = arith.constant 1.0 : f32
    %nz = arith.cmpf ogt, %v, %f0 : f32
    %r:2 = scf.if %nz -> (f32, f32) {
      %bigger = arith.cmpf ogt, %v, %scale : f32
      %r2:2 = scf.if %bigger -> (f32, f32) {
        %q = arith.divf %scale, %v : f32
        %q2 = arith.mulf %q, %q : f32
        %s = math.fma %ssq, %q2, %f1 : f32
        scf.yield %v, %s : f32, f32
      } else {
        %q = arith.divf %v, %scale : f32
        %s = math.fma %q, %q, %ssq : f32
        scf.yield %scale, %s : f32, f32
      }
      scf.yield %r2#0, %r2#1 : f32, f32
    } else {
      scf.yield %scale, %ssq : f32, f32
    }
    return %r#0, %r#1 : f32, f32
  }

  // Combines the threads partials of agx_blas_reduce in order into result[0]:
  // f32 bits for dot, asum and nrm2, the 1-based i32 index (0 when n is 0)
  // for amax and amin. One thread.
  gpu.func @agx_blas_reduce_final(%part0: memref<1xf32, 1>, %res0: memref<1xi32, 1>,
                                  %threads_: i64, %op_: i64) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %i2 = arith.constant 2 : index
    %i3 = arith.constant 3 : index
    %f0 = arith.constant 0.0 : f32
    %f1 = arith.constant 1.0 : f32
    %big = arith.constant 4611686018427387904 : index
    %op = arith.index_cast %op_ : i64 to index
    %threads = arith.index_cast %threads_ : i64 to index
    %part = memref.reinterpret_cast %part0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %res = memref.reinterpret_cast %res0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xi32, 1> to memref<?xi32, strided<[1]>, 1>
    %t = gpu.global_id x
    %first_thread = arith.cmpi eq, %t, %i0 : index
    scf.if %first_thread {
      %is_nrm = arith.cmpi eq, %op, %i2 : index
      %is_ext = arith.cmpi uge, %op, %i3 : index
      %is_max = arith.cmpi eq, %op, %i3 : index
      %none = arith.constant -1.0 : f32
      %a0 = arith.select %is_ext, %none, %f0 : f32
      %b0 = arith.select %is_nrm, %f1, %f0 : f32
      %r:2 = scf.for %k = %i0 to %threads step %i1 iter_args(%a = %a0, %b = %b0) -> (f32, f32) {
        %p0 = arith.muli %k, %i2 : index
        %p1 = arith.addi %p0, %i1 : index
        %pa = memref.load %part[%p0] : memref<?xf32, strided<[1]>, 1>
        %pb = memref.load %part[%p1] : memref<?xf32, strided<[1]>, 1>
        %o:2 = scf.if %is_nrm -> (f32, f32) {
          // (scale, ssq) + (pa, pb): fold pa^2 pb as |pa| sqrt(pb)
          %zero = arith.cmpf oeq, %pa, %f0 : f32
          %o2:2 = scf.if %zero -> (f32, f32) {
            scf.yield %a, %b : f32, f32
          } else {
            %bigger = arith.cmpf ogt, %pa, %a : f32
            %o3:2 = scf.if %bigger -> (f32, f32) {
              %q = arith.divf %a, %pa : f32
              %q2 = arith.mulf %q, %q : f32
              %s = math.fma %b, %q2, %pb : f32
              scf.yield %pa, %s : f32, f32
            } else {
              %q = arith.divf %pa, %a : f32
              %q2 = arith.mulf %q, %q : f32
              %s = math.fma %pb, %q2, %b : f32
              scf.yield %a, %s : f32, f32
            }
            scf.yield %o3#0, %o3#1 : f32, f32
          }
          scf.yield %o2#0, %o2#1 : f32, f32
        } else {
          %o4:2 = scf.if %is_ext -> (f32, f32) {
            %empty = arith.cmpf olt, %pa, %f0 : f32
            %first = arith.cmpf olt, %a, %f0 : f32
            %gt = arith.cmpf ogt, %pa, %a : f32
            %lt = arith.cmpf olt, %pa, %a : f32
            %eq = arith.cmpf oeq, %pa, %a : f32
            %ia = arith.bitcast %b : f32 to i32
            %ib = arith.bitcast %pb : f32 to i32
            %earlier = arith.cmpi slt, %ib, %ia : i32
            %tie = arith.andi %eq, %earlier : i1
            %strict = arith.select %is_max, %gt, %lt : i1
            %better0 = arith.ori %strict, %tie : i1
            %better1 = arith.ori %better0, %first : i1
            %true = arith.constant true
            %have = arith.xori %empty, %true : i1
            %better = arith.andi %better1, %have : i1
            %na = arith.select %better, %pa, %a : f32
            %nb = arith.select %better, %pb, %b : f32
            scf.yield %na, %nb : f32, f32
          } else {
            %s = arith.addf %a, %pa : f32
            scf.yield %s, %b : f32, f32
          }
          scf.yield %o4#0, %o4#1 : f32, f32
        }
        scf.yield %o#0, %o#1 : f32, f32
      }
      %out = scf.if %is_ext -> i32 {
        %found = arith.cmpf oge, %r#0, %f0 : f32
        %idx = arith.bitcast %r#1 : f32 to i32
        %c1 = arith.constant 1 : i32
        %c0 = arith.constant 0 : i32
        %one_based = arith.addi %idx, %c1 : i32
        %v = arith.select %found, %one_based, %c0 : i32
        scf.yield %v : i32
      } else {
        %root = math.sqrt %r#1 : f32
        %norm = arith.mulf %r#0, %root : f32
        %v = arith.select %is_nrm, %norm, %r#0 : f32
        %vi = arith.bitcast %v : f32 to i32
        scf.yield %vi : i32
      }
      memref.store %out, %res[%i0] : memref<?xi32, strided<[1]>, 1>
    }
    gpu.return
  }

  // y = alpha op(A) x + beta y for an m x n column-major A (op transposes
  // when trans is 1); y is not read when beta is 0. One thread per element
  // of y.
  gpu.func @agx_sgemv(%a0: memref<1xf32, 1>, %x0: memref<1xf32, 1>,
                      %y0: memref<1xf32, 1>, %m_: i64, %n_: i64, %lda_: i64,
                      %incx_: i64, %incy_: i64, %trans_: i64, %alpha: f32,
                      %beta: f32) kernel {
    %i0 = arith.constant 0 : index
    %i1 = arith.constant 1 : index
    %f0 = arith.constant 0.0 : f32
    %big = arith.constant 4611686018427387904 : index
    %m = arith.index_cast %m_ : i64 to index
    %n = arith.index_cast %n_ : i64 to index
    %lda = arith.index_cast %lda_ : i64 to index
    %z = arith.constant 0 : i64
    %trans = arith.cmpi ne, %trans_, %z : i64
    %A = memref.reinterpret_cast %a0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %x = memref.reinterpret_cast %x0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %y = memref.reinterpret_cast %y0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    // y has ylen elements, the sum runs over xlen; A[i, j] at i + j lda
    %ylen = arith.select %trans, %n, %m : index
    %xlen = arith.select %trans, %m, %n : index
    %rs = arith.select %trans, %lda, %i1 : index
    %cs = arith.select %trans, %i1, %lda : index
    %i = gpu.global_id x
    %live = arith.cmpi ult, %i, %ylen : index
    scf.if %live {
      %row = arith.muli %i, %rs : index
      %s = scf.for %k = %i0 to %xlen step %i1 iter_args(%acc = %f0) -> f32 {
        %kc = arith.muli %k, %cs : index
        %p = arith.addi %row, %kc : index
        %va = memref.load %A[%p] : memref<?xf32, strided<[1]>, 1>
        %ix = func.call @agx_blas_pos(%k, %xlen, %incx_) : (index, index, i64) -> index
        %vx = memref.load %x[%ix] : memref<?xf32, strided<[1]>, 1>
        %acc2 = math.fma %va, %vx, %acc : f32
        scf.yield %acc2 : f32
      }
      %iy = func.call @agx_blas_pos(%i, %ylen, %incy_) : (index, index, i64) -> index
      %as = arith.mulf %s, %alpha : f32
      %zero = arith.cmpf oeq, %beta, %f0 : f32
      %v = scf.if %zero -> f32 {
        scf.yield %as : f32
      } else {
        %old = memref.load %y[%iy] : memref<?xf32, strided<[1]>, 1>
        %r = math.fma %old, %beta, %as : f32
        scf.yield %r : f32
      }
      memref.store %v, %y[%iy] : memref<?xf32, strided<[1]>, 1>
    }
    gpu.return
  }

  // A += alpha x y^T for an m x n column-major A. 2-D grid, x along rows.
  gpu.func @agx_sger(%a0: memref<1xf32, 1>, %x0: memref<1xf32, 1>,
                     %y0: memref<1xf32, 1>, %m_: i64, %n_: i64, %lda_: i64,
                     %incx_: i64, %incy_: i64, %alpha: f32) kernel {
    %big = arith.constant 4611686018427387904 : index
    %m = arith.index_cast %m_ : i64 to index
    %n = arith.index_cast %n_ : i64 to index
    %lda = arith.index_cast %lda_ : i64 to index
    %A = memref.reinterpret_cast %a0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %x = memref.reinterpret_cast %x0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %y = memref.reinterpret_cast %y0 to offset: [0], sizes: [%big], strides: [1]
        : memref<1xf32, 1> to memref<?xf32, strided<[1]>, 1>
    %i = gpu.global_id x
    %j = gpu.global_id y
    %in_m = arith.cmpi ult, %i, %m : index
    %in_n = arith.cmpi ult, %j, %n : index
    %in = arith.andi %in_m, %in_n : i1
    scf.if %in {
      %ix = func.call @agx_blas_pos(%i, %m, %incx_) : (index, index, i64) -> index
      %iy = func.call @agx_blas_pos(%j, %n, %incy_) : (index, index, i64) -> index
      %vx = memref.load %x[%ix] : memref<?xf32, strided<[1]>, 1>
      %vy = memref.load %y[%iy] : memref<?xf32, strided<[1]>, 1>
      %jl = arith.muli %j, %lda : index
      %p = arith.addi %jl, %i : index
      %old = memref.load %A[%p] : memref<?xf32, strided<[1]>, 1>
      %ax = arith.mulf %vx, %alpha : f32
      %v = math.fma %ax, %vy, %old : f32
      memref.store %v, %A[%p] : memref<?xf32, strided<[1]>, 1>
    }
    gpu.return
  }
}
